//! What a seat has queued, and what withdrawing one entry takes with it
//! (decision 0221, "Taking back", rules 15 to 18): the answers of `queued`
//! and `take_back`, built from the seat's own view and the seat's queues.
//!
//! Nothing here holds the game. `session.rs` gives the queues and the views
//! (its own for the orders, the one of the coming day for the commands'
//! words) and does the judging of what is left after a take back, which
//! needs the world. Every unit is written in the side's numbers: an order's
//! body is the reverse of what `issue_orders` reads (decision 0086).

use std::collections::BTreeSet;

use serde_json::{json, Value};
use sim_core::command::{CommandKind, HoldTarget};
use sim_core::ids::IntentId;
use sim_core::orders::OrderRequest;
use sim_core::time::Pulse;
use sim_core::view::SideView;

use crate::answer::{FORMAT, VERSION};
use crate::common::{map_unit_tokens, Ctx};
use crate::orders::{order_to_json, order_words, unit_text};
use crate::staffing::{command_json, command_text, Names};

/// The three queues of a seat, in the order `queued` lists them.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub(crate) enum QueueName {
    /// Orders for the next pulse of the open day.
    Pulse,
    /// Orders for dusk, given at 1800 for the coming day.
    Dusk,
    /// Commands for the coming day's start.
    Commands,
}

impl QueueName {
    pub(crate) fn name(self) -> &'static str {
        match self {
            QueueName::Pulse => "pulse",
            QueueName::Dusk => "dusk",
            QueueName::Commands => "commands",
        }
    }

    pub(crate) fn parse(name: &str) -> Option<QueueName> {
        [QueueName::Pulse, QueueName::Dusk, QueueName::Commands]
            .into_iter()
            .find(|queue| queue.name() == name)
    }
}

/// The place of an entry: its queue and its number there, from 1.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub(crate) struct Slot {
    pub(crate) queue: QueueName,
    pub(crate) number: usize,
}

impl Slot {
    fn json(self) -> Value {
        json!({ "queue": self.queue.name(), "number": self.number })
    }

    fn words(self) -> String {
        format!("{} {}", self.queue.name(), self.number)
    }
}

/// A seat's queues, as the session holds them.
#[derive(Clone, Copy)]
pub(crate) struct Queues<'q> {
    pub(crate) pulse: &'q [OrderRequest],
    pub(crate) dusk: &'q [OrderRequest],
    pub(crate) commands: &'q [CommandKind],
}

impl Queues<'_> {
    fn orders(&self, queue: QueueName) -> &[OrderRequest] {
        match queue {
            QueueName::Pulse => self.pulse,
            _ => self.dusk,
        }
    }

    /// Every slot, in the order `queued` lists them.
    pub(crate) fn slots(&self) -> Vec<Slot> {
        let mut slots = Vec::new();
        for (queue, len) in [
            (QueueName::Pulse, self.pulse.len()),
            (QueueName::Dusk, self.dusk.len()),
            (QueueName::Commands, self.commands.len()),
        ] {
            slots.extend((1..=len).map(|number| Slot { queue, number }));
        }
        slots
    }

    /// Whether `slot` names an entry.
    pub(crate) fn has(&self, slot: Slot) -> bool {
        slot.number >= 1
            && slot.number
                <= match slot.queue {
                    QueueName::Pulse => self.pulse.len(),
                    QueueName::Dusk => self.dusk.len(),
                    QueueName::Commands => self.commands.len(),
                }
    }
}

/// Where the clock stands, for what the entries say of when they act.
#[derive(Clone, Copy)]
pub(crate) struct Moment {
    /// The day open and its next pulse, while the clock is stopped inside
    /// a day.
    pub(crate) open: Option<(u32, Pulse)>,
    /// The day the orders and commands for dusk are for.
    pub(crate) dusk_day: u32,
}

/// One entry as `queued` lists it, with its words.
pub(crate) struct Row {
    pub(crate) slot: Slot,
    pub(crate) json: Value,
    pub(crate) text: String,
}

/// The unit a hold keeps from the staff, as an order is for it, `kind:raw`.
/// A ship's hold is tied to no order.
fn hold_unit(target: HoldTarget) -> Option<String> {
    match target {
        HoldTarget::TaskForce(force) => Some(format!("task_force:{}", force.raw())),
        HoldTarget::AirGroup(group) | HoldTarget::Setting { group, .. } => {
            Some(format!("air_group:{}", group.raw()))
        }
        HoldTarget::LandUnit(unit) => Some(format!("land_unit:{}", unit.raw())),
        HoldTarget::Submarine(boat) => Some(format!("submarine:{}", boat.raw())),
        HoldTarget::Ship(_) => None,
    }
}

/// The intent a command names as its own, where it names one: the limit's,
/// the leave to pursue's, the withdrawal's.
fn intent_named(command: &CommandKind) -> Option<IntentId> {
    match command {
        CommandKind::SetLimit(limit) => limit.intent(),
        CommandKind::SetPursuit { intent, .. } | CommandKind::WithdrawIntent { intent } => {
            Some(*intent)
        }
        _ => None,
    }
}

/// The queued orders to the unit `unit` (`kind:raw`), each with its slot.
fn orders_to(queues: &Queues<'_>, unit: &str) -> Vec<Slot> {
    let mut found = Vec::new();
    for queue in [QueueName::Pulse, QueueName::Dusk] {
        for (i, order) in queues.orders(queue).iter().enumerate() {
            if unit_text(&order.kind) == unit {
                found.push(Slot {
                    queue,
                    number: i + 1,
                });
            }
        }
    }
    found
}

/// What goes with the entry at `slot` when it is taken back (rule 16):
///
/// - an order to a unit: where, once it is gone, no other queued order (in
///   either order queue) is to that unit, every queued hold on the unit;
/// - a `set_intent` that makes a new intent (its number is not yet in the
///   side's command state): the commands after it that name that intent as
///   their own.
///
/// Anything else takes nothing with it.
pub(crate) fn goes_with(view: &SideView<'_>, queues: &Queues<'_>, slot: Slot) -> Vec<Slot> {
    if !queues.has(slot) {
        return Vec::new();
    }
    match slot.queue {
        QueueName::Pulse | QueueName::Dusk => {
            let order = &queues.orders(slot.queue)[slot.number - 1];
            let unit = unit_text(&order.kind);
            if orders_to(queues, &unit).iter().any(|other| *other != slot) {
                return Vec::new();
            }
            queues
                .commands
                .iter()
                .enumerate()
                .filter(|(_, command)| {
                    matches!(command, CommandKind::Hold { target }
                        if hold_unit(*target).as_deref() == Some(unit.as_str()))
                })
                .map(|(i, _)| Slot {
                    queue: QueueName::Commands,
                    number: i + 1,
                })
                .collect()
        }
        QueueName::Commands => {
            let CommandKind::SetIntent { intent, .. } = &queues.commands[slot.number - 1] else {
                return Vec::new();
            };
            let made_here = !view.command_state().intents.contains_key(intent)
                && !queues.commands[..slot.number - 1].iter().any(
                    |before| matches!(before, CommandKind::SetIntent { intent: i, .. } if i == intent),
                );
            if !made_here {
                return Vec::new();
            }
            queues
                .commands
                .iter()
                .enumerate()
                .skip(slot.number)
                .filter(|(_, command)| intent_named(command) == Some(*intent))
                .map(|(i, _)| Slot {
                    queue: QueueName::Commands,
                    number: i + 1,
                })
                .collect()
        }
    }
}

/// What a hold rests on: the queued orders to its unit, which it goes with
/// when the last of them is taken back. Empty for any other entry.
fn rests_on(queues: &Queues<'_>, slot: Slot) -> Vec<Slot> {
    if slot.queue != QueueName::Commands || !queues.has(slot) {
        return Vec::new();
    }
    match &queues.commands[slot.number - 1] {
        CommandKind::Hold { target } => hold_unit(*target)
            .map(|unit| orders_to(queues, &unit))
            .unwrap_or_default(),
        _ => Vec::new(),
    }
}

/// The side's number of each unit a text names by the umpire's id.
fn numbered(names: &Names<'_>, ctx: &Ctx<'_>, text: &str) -> String {
    map_unit_tokens(text, |kind, raw| match kind {
        "ship" => ctx.ship_number(sim_core::ids::ShipId::new(raw)),
        "task_force" => names.force_number(raw),
        "land_unit" => ctx.land_unit_number(sim_core::ids::LandUnitId::new(raw)),
        _ => ctx.group_number(sim_core::ids::AirGroupId::new(raw)),
    })
}

/// An order's body as `issue_orders` takes it, in the side's numbers: the
/// reverse of what the session reads (the ships of a detach or a forming
/// order, a load's land unit, a landing's roster, the force a join goes
/// into, an escort's charge, a fuelling's giver).
fn body_in_numbers(names: &Names<'_>, ctx: &Ctx<'_>, body: &Value) -> Value {
    match body {
        Value::String(text) => Value::String(numbered(names, ctx, text)),
        Value::Array(items) => Value::Array(
            items
                .iter()
                .map(|v| body_in_numbers(names, ctx, v))
                .collect(),
        ),
        Value::Object(map) => Value::Object(
            map.iter()
                .map(|(key, value)| {
                    let shown = match (key.as_str(), value) {
                        ("ships", Value::Array(ids)) => Value::Array(
                            ids.iter()
                                .map(|id| {
                                    id.as_u64()
                                        .and_then(|n| u32::try_from(n).ok())
                                        .and_then(|raw| {
                                            ctx.ship_number(sim_core::ids::ShipId::new(raw))
                                        })
                                        .map_or_else(|| id.clone(), |n| json!(n))
                                })
                                .collect(),
                        ),
                        _ => body_in_numbers(names, ctx, value),
                    };
                    (key.clone(), shown)
                })
                .collect(),
        ),
        other => other.clone(),
    }
}

/// Where an entry acts: the pulse's start, or the dusk it is given at.
fn acts(queue: QueueName, moment: &Moment) -> (Value, String) {
    match (queue, moment.open) {
        (QueueName::Pulse, Some((day, pulse))) => (
            json!({ "at": "pulse", "day": day, "pulse": pulse.name() }),
            format!("at the start of the {} pulse of day {day}", pulse.name()),
        ),
        _ => (
            json!({ "at": "dusk", "day": moment.dusk_day }),
            format!("at 1800, for day {}", moment.dusk_day),
        ),
    }
}

/// Every entry of the queues, in the order `queued` lists them. `view` is
/// the seat's own view, which numbers the orders; `commands_view` the view of
/// the coming day's world with the commands taken in, which numbers the
/// commands as their answers do.
pub(crate) fn rows(
    view: &SideView<'_>,
    commands_view: &SideView<'_>,
    queues: &Queues<'_>,
    moment: &Moment,
) -> Vec<Row> {
    let ctx = Ctx::new(*view);
    let names = Names::new(view);
    let command_names = Names::new(commands_view);
    let mut rows = Vec::new();
    for slot in queues.slots() {
        let (acts, acts_words) = acts(slot.queue, moment);
        let with: Vec<Value> = goes_with(view, queues, slot)
            .into_iter()
            .map(Slot::json)
            .collect();
        let rests: Vec<Value> = rests_on(queues, slot).into_iter().map(Slot::json).collect();
        let (mut json, text) = match slot.queue {
            QueueName::Pulse | QueueName::Dusk => {
                let order = &queues.orders(slot.queue)[slot.number - 1];
                let unit = numbered(&names, &ctx, &unit_text(&order.kind));
                let text = numbered(
                    &names,
                    &ctx,
                    &format!("{}: {}", unit_text(&order.kind), order_words(&order.kind)),
                );
                (
                    json!({
                        "what": "order",
                        "unit": unit,
                        "order": body_in_numbers(&names, &ctx, &order_to_json(&order.kind)),
                    }),
                    text,
                )
            }
            QueueName::Commands => {
                let command = &queues.commands[slot.number - 1];
                let text = command_text(&command_names, command, None);
                (
                    json!({
                        "what": "command",
                        "tool": command.name(),
                        "command": command_json(&command_names, command, None),
                    }),
                    text,
                )
            }
        };
        let map = json.as_object_mut().expect("an entry is an object");
        let mut entry = serde_json::Map::new();
        entry.insert("place".into(), slot.json());
        entry.append(map);
        entry.insert("text".into(), json!(text));
        entry.insert("acts".into(), acts);
        entry.insert("acts_words".into(), json!(acts_words));
        entry.insert("goes_with".into(), Value::Array(with));
        entry.insert("rests_on".into(), Value::Array(rests));
        rows.push(Row {
            slot,
            json: Value::Object(entry),
            text,
        });
    }
    rows
}

/// The words of a count of entries: "3 entries queued: 2 orders and 1
/// command".
fn count_words(rows: &[Row]) -> String {
    let orders = rows
        .iter()
        .filter(|row| row.slot.queue != QueueName::Commands)
        .count();
    let commands = rows.len() - orders;
    let plural =
        |n: usize, one: &str, many: &str| format!("{n} {}", if n == 1 { one } else { many });
    if rows.is_empty() {
        return "Nothing is queued".to_string();
    }
    format!(
        "{} queued: {} and {}",
        plural(rows.len(), "entry", "entries"),
        plural(orders, "order", "orders"),
        plural(commands, "command", "commands"),
    )
}

/// The answer of `queued`.
pub(crate) fn queued_answer(rows: &[Row], clock: Value, as_of: &str) -> Value {
    json!({
        "format": FORMAT,
        "version": VERSION,
        "kind": "queued",
        "clock": clock,
        "as_of": as_of,
        "entries": rows.iter().map(|row| row.json.clone()).collect::<Vec<_>>(),
        "summary": format!(
            "{}. An entry is taken back by its place with take_back, given this answer's as_of; orders for the pulse act at the start of that pulse, orders and commands for dusk are given at 1800.",
            count_words(rows)
        ),
    })
}

/// The answer of `take_back`: the entries taken back (the one named first,
/// as `queued` listed them before), the new `as_of` and the entries left.
pub(crate) fn take_back_answer(taken: &[&Row], left: &[Row], as_of: &str) -> Value {
    let named = taken.first().map_or(String::new(), |row| row.text.clone());
    let with: Vec<String> = taken.iter().skip(1).map(|row| row.text.clone()).collect();
    let mut summary = format!("Taken back: {named}");
    if !with.is_empty() {
        summary.push_str(&format!("; with it: {}", with.join("; ")));
    }
    summary.push_str(&format!(
        ". {} {} remain{} queued.",
        left.len(),
        if left.len() == 1 { "entry" } else { "entries" },
        if left.len() == 1 { "s" } else { "" },
    ));
    json!({
        "format": FORMAT,
        "version": VERSION,
        "kind": "take_back",
        "taken_back": taken.iter().map(|row| row.json.clone()).collect::<Vec<_>>(),
        "as_of": as_of,
        "queued": left.iter().map(|row| row.json.clone()).collect::<Vec<_>>(),
        "summary": summary,
    })
}

/// The words that refuse a take back because an entry rests on what would go.
pub(crate) fn rests_on_it(row: &Row) -> String {
    format!(
        "{} (place {}) rests on it: take that back first",
        row.text,
        row.slot.words()
    )
}

/// The slots that go: the one named and what goes with it.
pub(crate) fn gone(view: &SideView<'_>, queues: &Queues<'_>, named: Slot) -> BTreeSet<Slot> {
    let mut gone: BTreeSet<Slot> = goes_with(view, queues, named).into_iter().collect();
    gone.insert(named);
    gone
}
