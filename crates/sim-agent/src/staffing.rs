//! The tools of a seat whose side has a staff (decision 0107, "As built",
//! "The tools"): `command_state`, `plans`, `propose`, `staff_orders`, `why`
//! and the answers of the six commands a seat queues.
//!
//! Every function here takes a [`SideView`] and nothing else of the game: a
//! view of the world as the coming day will find it (the commands the seat
//! has queued taken in, the forces back from the yard and the refuelling
//! done), which `session.rs` makes and nothing else may. The staff is asked
//! on that view, as it will be asked when the day resolves.
//!
//! A unit is shown in the side's own numbers (decision 0086): the staff's
//! words and commands carry the engine's ids (`ship:1`, `task_force:1000001`)
//! and [`Names`] writes them in the side's numbers, a task force the staff
//! will form being the next number the side would give it. A base keeps its
//! number. A quantity is a decimal string, never a float.

use std::collections::BTreeMap;

use serde_json::{json, Map, Value};
use sim_core::command::{
    AmendmentChange, CommandKind, CommandRefusal, Committed, Displacement, HoldMaker, HoldTarget,
    Intent, IntentKind, IntentLimits, Limit, Plan, PlanState, Proposal, Run, RunDays, Staff,
    StaffDecisions, SupplyProjection,
};
use sim_core::events::EventKind;
use sim_core::fixed::Fixed;
use sim_core::ids::{AirGroupId, IntentId, LandUnitId, ShipId, TaskForceId};
use sim_core::orders::Issuer;
use sim_core::side_events::{SideCause, SideEvent, SideEventKind};
use sim_core::state::ShipLocation;
use sim_core::view::SideView;
use sim_core::world::MADE_IDS_FLOOR;

use crate::answer::{NotKnown, FORMAT, VERSION};
use crate::commands::{
    command_from_json, command_to_json, reason_to_json, staff_decisions_to_json,
};
use crate::common::{map_unit_tokens, Ctx};
use crate::orders::{order_to_json, unit_text};
use crate::record::CallError;

/// The days between shipments the staff uses where the commander set none.
/// It is the doctrine's own figure (`DEFAULT_INTERVAL_DAYS` of `sim-staff`);
/// a test in `sim-cli` holds the two together.
pub const STAFF_DEFAULT_INTERVAL_DAYS: u32 = 15;

/// What every answer about the staff says first: it does nothing for a side
/// until an intent with a forces limit is set.
const STAFF_NOTE: &str = "The staff does nothing for your side until you set an intent and give it a forces limit (set_intent, then set_limit forces). Commands you queue take effect at the start of the coming day.";

/// The ships and forces of the umpire's ids shown in the side's numbers,
/// from the side's view.
pub(crate) struct Names<'a> {
    ctx: Ctx<'a>,
    next_force: u32,
}

impl<'a> Names<'a> {
    pub(crate) fn new(view: &SideView<'a>) -> Names<'a> {
        Names {
            ctx: Ctx::new(*view),
            next_force: view.next_task_force_id().raw(),
        }
    }

    /// The side's number of a task force: one it has, or one its next
    /// forming order makes (the next number past its last, and so on).
    fn force_number(&self, raw: u32) -> Option<u32> {
        self.ctx.force_number(TaskForceId::new(raw)).or_else(|| {
            let made = raw.checked_sub(self.next_force)?;
            (made < MADE_IDS_FLOOR).then(|| self.ctx.force_count() + 1 + made)
        })
    }

    /// Text with each unit token written in the side's numbers. `ships` and
    /// `forces` are the numbers a refusal's stand-ins stand for.
    fn text_with(
        &self,
        text: &str,
        ships: &BTreeMap<u32, u32>,
        forces: &BTreeMap<u32, u32>,
        groups: &BTreeMap<u32, u32>,
    ) -> String {
        map_unit_tokens(text, |kind, raw| match kind {
            "ship" => ships
                .get(&raw)
                .copied()
                .or_else(|| self.ctx.ship_number(ShipId::new(raw))),
            "task_force" => forces.get(&raw).copied().or_else(|| self.force_number(raw)),
            "land_unit" => self.ctx.land_unit_number(LandUnitId::new(raw)),
            _ => groups
                .get(&raw)
                .copied()
                .or_else(|| self.ctx.group_number(AirGroupId::new(raw))),
        })
    }

    pub(crate) fn text(&self, text: &str) -> String {
        self.text_with(text, &BTreeMap::new(), &BTreeMap::new(), &BTreeMap::new())
    }

    /// A JSON value with every string's unit tokens, and every number under
    /// `ships` or `task_forces`, written in the side's numbers.
    pub(crate) fn value(&self, value: &Value) -> Value {
        match value {
            Value::String(text) => Value::String(self.text(text)),
            Value::Array(items) => Value::Array(items.iter().map(|v| self.value(v)).collect()),
            Value::Object(map) => Value::Object(
                map.iter()
                    .map(|(key, v)| {
                        let shown = match (key.as_str(), v) {
                            ("ships", Value::Array(ids)) => Value::Array(
                                ids.iter()
                                    .map(|id| {
                                        id.as_u64()
                                            .and_then(|n| u32::try_from(n).ok())
                                            .and_then(|raw| self.ctx.ship_number(ShipId::new(raw)))
                                            .map_or_else(|| id.clone(), |n| json!(n))
                                    })
                                    .collect(),
                            ),
                            ("task_forces", Value::Array(ids)) => Value::Array(
                                ids.iter()
                                    .map(|id| {
                                        id.as_u64()
                                            .and_then(|n| u32::try_from(n).ok())
                                            .and_then(|raw| self.force_number(raw))
                                            .map_or_else(|| id.clone(), |n| json!(n))
                                    })
                                    .collect(),
                            ),
                            _ => self.value(v),
                        };
                        (key.clone(), shown)
                    })
                    .collect(),
            ),
            other => other.clone(),
        }
    }

    fn ship(&self, id: ShipId) -> u32 {
        self.ctx.ship_number(id).unwrap_or_else(|| id.raw())
    }

    fn ships<'s>(&self, ids: impl IntoIterator<Item = &'s ShipId>) -> Vec<u32> {
        let mut numbers: Vec<u32> = ids.into_iter().map(|id| self.ship(*id)).collect();
        numbers.sort_unstable();
        numbers
    }

    fn base(&self, id: sim_core::ids::BaseId) -> Value {
        serde_json::to_value(self.ctx.base(id)).expect("a handle serialises")
    }
}

fn dec(value: Fixed) -> Value {
    Value::String(value.to_string())
}

fn days_json(days: &RunDays) -> Value {
    json!({
        "load": days.load,
        "sail": days.sail,
        "arrive": days.arrive,
        "free": days.free,
    })
}

fn issuer_word(issuer: Issuer) -> &'static str {
    match issuer {
        Issuer::Player => "commander",
        Issuer::Staff => "staff",
        Issuer::LocalCommander => "local_commander",
    }
}

fn answer(kind: &str, mut fields: Value) -> Value {
    let map = fields.as_object_mut().expect("an answer is an object");
    let mut out = Map::new();
    out.insert("format".into(), json!(FORMAT));
    out.insert("version".into(), json!(VERSION));
    out.insert("kind".into(), json!(kind));
    out.append(map);
    Value::Object(out)
}

// ---------------------------------------------------------------------------
// Intents, limits, plans.
// ---------------------------------------------------------------------------

fn force_numbers<'s>(n: &Names<'_>, forces: impl IntoIterator<Item = &'s TaskForceId>) -> Vec<u32> {
    let mut numbers: Vec<u32> = forces
        .into_iter()
        .map(|f| n.ctx.force_number(*f).unwrap_or_else(|| f.raw()))
        .collect();
    numbers.sort_unstable();
    numbers
}

fn limits_json(n: &Names<'_>, limits: &IntentLimits) -> Value {
    json!({
        "forces": limits.forces.as_ref().map(|forces| json!({
            "ships": n.ships(&forces.ships),
            "task_forces": force_numbers(n, &forces.task_forces),
            "source": n.base(forces.source),
        })),
        "area": limits.area.as_ref().map(|hexes| {
            hexes.iter().map(|h| json!([h.q, h.r])).collect::<Vec<_>>()
        }),
        "priority": limits.priority,
        "protected": n.ships(&limits.protected),
        "risk": limits.risk.name(),
    })
}

fn projection_json(projection: &SupplyProjection) -> Value {
    let readings: Vec<Value> = projection
        .readings
        .iter()
        .map(|r| {
            json!({
                "day": r.day,
                "stock_t": dec(r.stock_t),
                "use_t": dec(r.use_t),
                "days": r.days.map(dec),
            })
        })
        .collect();
    json!({
        "hex": {"q": projection.hex.q, "r": projection.hex.r},
        "floor_days": projection.floor_days,
        "today": readings.first().cloned(),
        "first_day_below_floor": projection.first_day_below,
        "readings": readings,
    })
}

/// The short form of a projection for `command_state`: today's reading and
/// the first day below the floor.
fn projection_summary(projection: &SupplyProjection) -> Value {
    let today = projection.readings.first();
    json!({
        "floor_days": projection.floor_days,
        "today": today.map(|r| json!({
            "day": r.day,
            "stock_t": dec(r.stock_t),
            "use_t": dec(r.use_t),
            "days": r.days.map(dec),
        })),
        "first_day_below_floor": projection.first_day_below,
    })
}

fn intent_json(n: &Names<'_>, staff: &dyn Staff, view: &SideView<'_>, intent: &Intent) -> Value {
    let IntentKind::KeepSupplied {
        base,
        floor_days,
        interval_days,
    } = intent.kind;
    let interval = match interval_days {
        Some(days) => days.to_string(),
        None => format!("not set: the staff uses {STAFF_DEFAULT_INTERVAL_DAYS}"),
    };
    let proposal = if intent.in_force() {
        staff.propose(*view, intent.id)
    } else {
        None
    };
    let displaced_by: Vec<Value> = proposal
        .as_ref()
        .and_then(|p| p.shortfall.as_ref())
        .map(|s| {
            s.displaced_by
                .iter()
                .map(|d| displacement_json(n, d))
                .collect()
        })
        .unwrap_or_default();
    let projection = proposal
        .and_then(|p| p.projection)
        .as_ref()
        .map(projection_summary);
    // Its place in the order the staff serves: ranked first, then by number.
    let served = intent.in_force().then(|| {
        let mut order: Vec<&Intent> = view.command_state().intents_in_force().collect();
        order.sort_by_key(|i| {
            (
                i.limits.priority.is_none(),
                i.limits.priority.unwrap_or(0),
                i.id,
            )
        });
        order
            .iter()
            .position(|i| i.id == intent.id)
            .map_or(0, |at| at + 1)
    });
    json!({
        "number": intent.id.raw(),
        "in_force": intent.in_force(),
        "kind": "keep_supplied",
        "base": n.base(base),
        "floor_days": floor_days,
        "interval_days": interval_days,
        "interval": interval,
        "set_day": intent.set_day,
        "withdrawn_day": intent.withdrawn_day,
        "plan": intent.plan.raw(),
        "limits": limits_json(n, &intent.limits),
        "projection": projection,
        "served": served,
        "displaced_by": displaced_by,
    })
}

fn amendment_text(n: &Names<'_>, plan: &Plan, change: &AmendmentChange) -> String {
    let command = match change {
        AmendmentChange::Intent(kind) => CommandKind::SetIntent {
            intent: plan.intent,
            kind: *kind,
        },
        AmendmentChange::Limit(limit) => CommandKind::SetLimit(limit.clone()),
    };
    n.text(&command.to_string())
}

/// Where a ship is, in words, and the supply aboard.
fn ship_state(n: &Names<'_>, id: ShipId) -> Value {
    let Some(ship) = n.ctx.ships.get(&id) else {
        return json!({"ship": n.ship(id), "at": "not known", "supply_aboard_t": "0"});
    };
    let at = match ship.location {
        ShipLocation::Port(base) => format!("in port at {base}"),
        ShipLocation::TaskForce(force) => {
            let number = n.ctx.force_number(force).unwrap_or_else(|| force.raw());
            match n.ctx.forces.get(&force) {
                Some(f) => format!(
                    "in task_force:{number} at ({},{})",
                    f.position().q,
                    f.position().r
                ),
                None => format!("in task_force:{number}"),
            }
        }
        ShipLocation::Yard(force) => {
            let number = n.ctx.force_number(force).unwrap_or_else(|| force.raw());
            format!("away at the yard with task_force:{number}")
        }
        ShipLocation::Sunk => "sunk".to_string(),
    };
    json!({
        "ship": n.ship(id),
        "at": at,
        "supply_aboard_t": dec(ship.supply_t),
    })
}

/// What a run is doing by the staff's days.
fn run_phase(run: &Run, today: u32) -> (&'static str, String) {
    let days = run.days;
    if today < days.load {
        ("waiting", format!("not begun: loads on day {}", days.load))
    } else if today < days.sail {
        (
            "loading",
            format!(
                "loading {} t of supply at base:{} for the sail on day {}",
                run.load_t,
                run.source.raw(),
                days.sail
            ),
        )
    } else if today <= days.arrive {
        (
            "under_way",
            format!(
                "sailed on day {}, due at the destination on day {}",
                days.sail, days.arrive
            ),
        )
    } else if today < days.free {
        (
            "returning",
            format!(
                "delivered or unloading, and coming home: its ships are free on day {}",
                days.free
            ),
        )
    } else {
        (
            "finished",
            format!("its ships were due free on day {}", days.free),
        )
    }
}

fn run_json(n: &Names<'_>, run: &Run, today: u32, doing: bool) -> Value {
    let mut out = json!({
        "number": run.number,
        "ships": n.ships(&run.ships),
        "load_t": dec(run.load_t),
        "source": n.base(run.source),
        "days": days_json(&run.days),
    });
    if doing {
        let (phase, text) = run_phase(run, today);
        out["phase"] = json!(phase);
        out["doing"] = json!(text);
        out["ship_states"] = Value::Array(run.ships.iter().map(|s| ship_state(n, *s)).collect());
    }
    out
}

fn plan_json(n: &Names<'_>, view: &SideView<'_>, plan: &Plan, doing: bool) -> Value {
    let today = view.day() + 1;
    let state = view.command_state();
    let intent_in_force = state
        .intents
        .get(&plan.intent)
        .is_some_and(|intent| intent.in_force());
    json!({
        "number": plan.id.raw(),
        "intent": plan.intent.raw(),
        "intent_in_force": intent_in_force,
        "state": plan.state.name(),
        "baseline": plan.baseline.as_ref().map(|b| {
            let IntentKind::KeepSupplied { base, floor_days, interval_days } = b.kind;
            json!({
                "day": b.day,
                "base": n.base(base),
                "floor_days": floor_days,
                "interval_days": interval_days,
                "limits": limits_json(n, &b.limits),
            })
        }),
        "baseline_day": plan.baseline.as_ref().map(|b| b.day),
        "amendments": plan
            .amendments
            .iter()
            .map(|a| json!({
                "number": a.number,
                "day": a.day,
                "change": amendment_text(n, plan, &a.change),
            }))
            .collect::<Vec<_>>(),
        "runs": plan
            .runs
            .values()
            .map(|run| run_json(n, run, today, doing))
            .collect::<Vec<_>>(),
    })
}

fn leaves_json(view: &SideView<'_>) -> Value {
    let leaves = view.command_state().leaves;
    json!({
        "prepare": leaves.prepare,
        "launch": leaves.launch,
        "meaning": "prepare: the staff may approve a draft plan that is inside your limits (given by default). launch: the staff may release a plan that is preparing (withheld by default: until you give it, you release a plan yourself with approve).",
    })
}

// ---------------------------------------------------------------------------
// Holds (decision 0107, step 3, rulings 17 to 23).
// ---------------------------------------------------------------------------

/// A hold's target as the interface writes it, in the side's numbers. With
/// the `request` it was read from, a number the side wrote that names none
/// of its units is shown as the side wrote it.
fn hold_target_json(n: &Names<'_>, target: HoldTarget, request: Option<&Request>) -> Value {
    let written = |map: fn(&Request) -> &BTreeMap<u32, u32>, raw: u32| {
        request.and_then(|r| map(r).get(&raw).copied())
    };
    match target {
        HoldTarget::Ship(ship) => json!({
            "ship": written(|r| &r.ship_stand_ins, ship.raw()).unwrap_or_else(|| n.ship(ship)),
        }),
        HoldTarget::TaskForce(force) => json!({
            "task_force": written(|r| &r.force_stand_ins, force.raw())
                .or_else(|| n.force_number(force.raw()))
                .unwrap_or_else(|| force.raw()),
        }),
        HoldTarget::Setting { group, setting } => json!({
            "air_group": written(|r| &r.group_stand_ins, group.raw())
                .or_else(|| n.ctx.group_number(group))
                .unwrap_or_else(|| group.raw()),
            "setting": setting.name(),
        }),
    }
}

/// A command as the interface writes it, in the side's numbers.
fn command_json(n: &Names<'_>, command: &CommandKind, request: Option<&Request>) -> Value {
    let target = match command {
        CommandKind::Hold { target } | CommandKind::Release { target } => Some(*target),
        _ => None,
    };
    match target {
        Some(target) => {
            let mut body = json!({"kind": command.name()});
            if let (Some(body), Value::Object(keys)) =
                (body.as_object_mut(), hold_target_json(n, target, request))
            {
                body.extend(keys);
            }
            body
        }
        None => n.value(&command_to_json(command)),
    }
}

/// A command in words, in the side's numbers.
fn command_text(n: &Names<'_>, command: &CommandKind, request: Option<&Request>) -> String {
    match request {
        Some(r) => n.text_with(
            &command.to_string(),
            &r.ship_stand_ins,
            &r.force_stand_ins,
            &r.group_stand_ins,
        ),
        None => n.text(&command.to_string()),
    }
}

/// A hold's target in words, in the side's numbers.
fn hold_target_words(n: &Names<'_>, target: HoldTarget) -> String {
    match target {
        HoldTarget::Ship(ship) => format!("ship {}", n.ship(ship)),
        HoldTarget::TaskForce(force) => format!(
            "task force {}",
            n.ctx.force_number(force).unwrap_or_else(|| force.raw())
        ),
        HoldTarget::Setting { group, setting } => format!(
            "the {} of air group {}",
            setting.name().replace('_', " "),
            n.ctx.group_number(group).unwrap_or_else(|| group.raw())
        ),
    }
}

/// What made a hold, in words: "your hold command of day 8" or "your order
/// 3 of day 2".
fn hold_maker_words(n: &Names<'_>, hold: &sim_core::command::Hold) -> String {
    match hold.by {
        HoldMaker::Command => format!("your hold command of day {}", hold.day),
        HoldMaker::Order(id) => match n.ctx.order_number_if_own(id) {
            Some(number) => format!("your order {number} of day {}", hold.day),
            None => format!("an order of yours of day {}", hold.day),
        },
    }
}

/// The holds of the side, each with its target, the day, what made it and
/// the words.
fn holds_json(n: &Names<'_>, view: &SideView<'_>) -> Vec<Value> {
    view.command_state()
        .holds
        .iter()
        .map(|(target, hold)| {
            let (made_by, order) = match hold.by {
                HoldMaker::Command => ("command", None),
                HoldMaker::Order(id) => ("order", n.ctx.order_number_if_own(id)),
            };
            json!({
                "target": hold_target_json(n, *target, None),
                "day": hold.day,
                "made_by": made_by,
                "order": order,
                "text": format!(
                    "{}, held by {}",
                    hold_target_words(n, *target),
                    hold_maker_words(n, hold)
                ),
            })
        })
        .collect()
}

// ---------------------------------------------------------------------------
// What the staff decided, by event.
// ---------------------------------------------------------------------------

/// The decision a `StaffReason` event records: what it is for and its
/// reason, in the side's numbers. `None` for an event that is no reason.
fn decision_of(
    n: &Names<'_>,
    view: &SideView<'_>,
    events: &[SideEvent],
    event: &SideEvent,
) -> Option<(Value, Value)> {
    let SideEventKind::Whole(EventKind::StaffReason { order, reason, .. }) = &event.kind else {
        return None;
    };
    let reason_json = n.value(&reason_to_json(reason));
    let cited = |number: u32| number.checked_sub(1).and_then(|i| events.get(i as usize));
    let for_json = if let Some(id) = order {
        let number = n.ctx.order_number(*id);
        let order = view.orders().find(|o| o.id == *id);
        let verdict = events.iter().find_map(|e| match &e.kind {
            SideEventKind::Whole(EventKind::OrderAccepted { order, .. }) if *order == *id => {
                Some((e.number, None))
            }
            SideEventKind::Whole(EventKind::OrderRejected { order, reason, .. })
                if *order == *id =>
            {
                Some((e.number, Some(crate::words::refusal(reason))))
            }
            _ => None,
        });
        let (accepted, refusal, verdict_event) = match &verdict {
            Some((number, None)) => (Some(true), None, Some(*number)),
            Some((number, Some(why))) => (Some(false), Some(n.text(why)), Some(*number)),
            None => (None, None, None),
        };
        json!({
            "type": "order",
            "order": number,
            "unit": order.map(|o| n.text(&unit_text(&o.kind))),
            "body": order.map(|o| n.value(&order_to_json(&o.kind))),
            "intent": order.and_then(|o| o.origin.intent).map(|i| i.raw()),
            "accepted": accepted,
            "refusal": refusal,
            "event": verdict_event,
        })
    } else if let Some((cause, kind)) = event.causes.iter().find_map(|c| match c {
        SideCause::Event(k) => cited(*k).map(|e| (*k, e)),
        SideCause::Order(_) => None,
    }) {
        match &kind.kind {
            SideEventKind::Whole(EventKind::CommandTaken {
                issuer, command, ..
            }) => json!({
                "type": "command",
                "command": command_json(n, command, None),
                "text": n.text(&command.to_string()),
                "issuer": issuer_word(*issuer),
                "taken": true,
                "refusal": Value::Null,
                "event": cause,
            }),
            SideEventKind::Whole(EventKind::CommandRefused {
                issuer,
                command,
                reason,
                ..
            }) => json!({
                "type": "command",
                "command": command_json(n, command, None),
                "text": n.text(&command.to_string()),
                "issuer": issuer_word(*issuer),
                "taken": false,
                "refusal": n.text(&reason.to_string()),
                "event": cause,
            }),
            _ => json!({"type": "no_order"}),
        }
    } else {
        json!({"type": "no_order"})
    };
    Some((for_json, reason_json))
}

/// The staff's decisions of one day, each with the number `why` takes: what
/// it was for in a line, and whether it was carried out (an order accepted,
/// a command taken; none for a decision to give no order).
fn decisions_of_day(n: &Names<'_>, view: &SideView<'_>, day: u32) -> Vec<Value> {
    let events = view.events();
    events
        .iter()
        .filter(|e| e.day == day)
        .filter_map(|e| {
            let (for_json, reason) = decision_of(n, view, &events, e)?;
            let text = |key: &str| for_json[key].as_str().unwrap_or("").to_string();
            let (kind, summary, carried_out) = match for_json["type"].as_str() {
                Some("order") => (
                    "order",
                    format!(
                        "order to {}: {}",
                        text("unit"),
                        for_json["body"]["kind"].as_str().unwrap_or("")
                    ),
                    for_json["accepted"].clone(),
                ),
                Some("command") => ("command", text("text"), for_json["taken"].clone()),
                _ => (
                    "no_order",
                    reason["worked"].as_str().unwrap_or("").to_string(),
                    Value::Null,
                ),
            };
            Some(json!({
                "number": e.number,
                "day": e.day,
                "for": kind,
                "summary": summary,
                "carried_out": carried_out,
            }))
        })
        .collect()
}

// ---------------------------------------------------------------------------
// The tools.
// ---------------------------------------------------------------------------

/// `command_state`: the side's command state as the coming day will find it.
/// `queued` are the commands the seat has queued, in order.
pub fn command_state(view: &SideView<'_>, staff: &dyn Staff, queued: &[CommandKind]) -> Value {
    let n = Names::new(view);
    let state = view.command_state();
    let today = view.day() + 1;
    let mut notes: Vec<String> = vec![STAFF_NOTE.to_string()];
    for intent in state.intents_in_force() {
        if intent.limits.forces.is_none() {
            notes.push(format!(
                "intent:{} has no forces limit: the staff gives no order for it until you set one (set_limit forces).",
                intent.id.raw()
            ));
        }
    }
    for plan in state.plans.values() {
        match plan.state {
            PlanState::Draft => notes.push(format!(
                "plan:{} is a draft: {}",
                plan.id.raw(),
                if state.leaves.prepare {
                    "the staff will prepare it if the intent has a forces limit, or you can approve it."
                } else {
                    "approve it to prepare it (you withheld the leave to prepare)."
                }
            )),
            PlanState::Preparing => notes.push(format!(
                "plan:{} is preparing: {}",
                plan.id.raw(),
                if state.leaves.launch {
                    "the staff will release it."
                } else {
                    "approve it to release it (the leave to launch is withheld)."
                }
            )),
            _ => {}
        }
    }
    answer(
        "command_state",
        json!({
            "for_day": today,
            "as_at": format!("the start of day {today}, with the commands you have queued taken in"),
            "notes": notes,
            "leaves": leaves_json(view),
            "intents": state
                .intents
                .values()
                .map(|intent| intent_json(&n, staff, view, intent))
                .collect::<Vec<_>>(),
            "reserves": view
                .bases()
                .map(|b| json!({
                    "base": n.base(b.id),
                    "supply_t": dec(b.supply_t),
                    "fuel_t": dec(b.fuel_t),
                    "supply_reserve_t": dec(b.supply_reserve_t),
                    "fuel_reserve_t": dec(b.fuel_reserve_t),
                }))
                .collect::<Vec<_>>(),
            "plans": state
                .plans
                .values()
                .map(|plan| plan_json(&n, view, plan, false))
                .collect::<Vec<_>>(),
            "commitments": state
                .commitments
                .iter()
                .flat_map(|(ship, list)| list.iter().map(move |c| (ship, c)))
                .map(|(ship, c)| json!({
                    "ship": n.ship(*ship),
                    "plan": c.plan.raw(),
                    "run": c.run,
                    "from_day": c.from_day,
                    "to_day": c.to_day,
                }))
                .collect::<Vec<_>>(),
            "holds": holds_json(&n, view),
            "queued": queued
                .iter()
                .enumerate()
                .map(|(i, command)| json!({
                    "index": i,
                    "command": command_json(&n, command, None),
                    "text": command_text(&n, command, None),
                }))
                .collect::<Vec<_>>(),
            "staff_decisions_yesterday": decisions_of_day(&n, view, view.day()),
        }),
    )
}

/// `plans`: the plans with their runs and what each run is doing.
pub fn plans(view: &SideView<'_>) -> Value {
    let n = Names::new(view);
    let today = view.day() + 1;
    answer(
        "plans",
        json!({
            "for_day": today,
            "as_at": format!("the start of day {today}, with the commands you have queued taken in"),
            "plans": view
                .command_state()
                .plans
                .values()
                .map(|plan| plan_json(&n, view, plan, true))
                .collect::<Vec<_>>(),
        }),
    )
}

/// `propose`: what the staff would do for one intent.
pub fn propose(view: &SideView<'_>, staff: &dyn Staff, intent: u32) -> Result<Value, CallError> {
    let n = Names::new(view);
    let today = view.day() + 1;
    let Some(proposal) = staff.propose(*view, IntentId::new(intent)) else {
        return Err(CallError::new(format!(
            "intent:{intent} is not an intent in force (command_state lists the intents)"
        )));
    };
    let in_force = view.command_state().intents_in_force().count();
    let displaces = if in_force <= 1 {
        "nothing: one intent".to_string()
    } else {
        displaces_text(&n, &proposal)
    };
    let displaced_by: Vec<Value> = proposal
        .shortfall
        .as_ref()
        .map(|s| {
            s.displaced_by
                .iter()
                .map(|d| displacement_json(&n, d))
                .collect()
        })
        .unwrap_or_default();
    let displaced_intents: Vec<Value> = proposal
        .displaces
        .iter()
        .map(|d| {
            json!({
                "intent": d.intent.raw(),
                "plan": d.plan.raw(),
                "shortfall_days": dec(d.shortfall.shortfall_days),
                "check_day": d.shortfall.check_day,
                "by": d.shortfall.displaced_by.iter().map(|b| displacement_json(&n, b)).collect::<Vec<_>>(),
            })
        })
        .collect();
    Ok(answer(
        "propose",
        json!({
            "for_day": today,
            "intent": intent,
            "plan": {"number": proposal.plan.raw(), "state": proposal.plan_state.name()},
            "projection": proposal.projection.as_ref().map(projection_json),
            "runs": proposal
                .runs
                .iter()
                .enumerate()
                .map(|(i, run)| json!({
                    "index": i + 1,
                    "ships": n.ships(&run.ships),
                    "load_t": dec(run.load_t),
                    "days": days_json(&run.days),
                    "reason": n.value(&reason_to_json(&run.reason)),
                }))
                .collect::<Vec<_>>(),
            "binds": proposal.shortfall.as_ref().map(|s| json!({
                "binds": n.text(&s.binds),
                "shortfall_days": dec(s.shortfall_days),
                "source_dry_day": s.source_dry_day,
                "check_day": s.check_day,
                "displaced_by": s.displaced_by.iter().map(|d| displacement_json(&n, d)).collect::<Vec<_>>(),
            })),
            "displaces": displaces,
            "displaced_by": displaced_by,
            "displaced_intents": displaced_intents,
            "would_hand_in": with_day(decisions_json(&n, &proposal.decisions), today),
            "reason": n.value(&reason_to_json(&proposal.reason)),
        }),
    ))
}

/// One commitment that displaces, in the side's numbers.
fn displacement_json(n: &Names<'_>, d: &Displacement) -> Value {
    let (ships, tons_t) = match &d.committed {
        Committed::Ships(ships) => (Some(n.ships(ships)), None),
        Committed::Tons(t) => (None, Some(dec(*t))),
    };
    json!({
        "intent": d.intent.raw(),
        "plan": d.plan.raw(),
        "run": d.run,
        "ships": ships,
        "tons_t": tons_t,
        "from_day": d.from_day,
        "to_day": d.to_day,
    })
}

/// A commitment in words, in the side's numbers.
fn displacement_text(n: &Names<'_>, d: &Displacement) -> String {
    let what = match &d.committed {
        Committed::Ships(ships) => {
            let numbers: Vec<String> = n.ships(ships).iter().map(u32::to_string).collect();
            format!(
                "{} {}",
                if numbers.len() == 1 { "ship" } else { "ships" },
                numbers.join(", ")
            )
        }
        Committed::Tons(t) => format!("{t} t at the source"),
    };
    format!(
        "run {} of plan {} (intent {}): {what}, days {} to {}",
        d.run,
        d.plan.raw(),
        d.intent.raw(),
        d.from_day,
        d.to_day
    )
}

/// What a proposal is displaced by and what it displaces, in a sentence.
fn displaces_text(n: &Names<'_>, proposal: &Proposal) -> String {
    let by = match proposal.shortfall.as_ref().map(|s| &s.displaced_by) {
        Some(list) if !list.is_empty() => format!(
            "displaced by {}",
            list.iter()
                .map(|d| displacement_text(n, d))
                .collect::<Vec<_>>()
                .join("; ")
        ),
        _ => "displaced by nothing".to_string(),
    };
    let displaces = if proposal.displaces.is_empty() {
        "displaces nothing".to_string()
    } else {
        proposal
            .displaces
            .iter()
            .map(|d| {
                let days = d.shortfall.shortfall_days;
                format!(
                    "displaces intent {}: {days} {} short at the reading of day {}",
                    d.intent.raw(),
                    if days == Fixed::ONE { "day" } else { "days" },
                    d.shortfall.check_day
                )
            })
            .collect::<Vec<_>>()
            .join("; ")
    };
    format!("{by}; {displaces}")
}

fn decisions_json(n: &Names<'_>, decisions: &StaffDecisions) -> Value {
    n.value(&staff_decisions_to_json(decisions))
}

/// `staff_orders`: what the staff would hand in for the coming day.
pub fn staff_orders(view: &SideView<'_>, staff: &dyn Staff) -> Value {
    let n = Names::new(view);
    let today = view.day() + 1;
    let decisions = staff.decide(*view);
    let summary = if decisions.is_empty() {
        "The staff would hand in nothing for the coming day: with no intent in force and a forces limit, it does nothing.".to_string()
    } else {
        format!(
            "The staff would hand in {} orders, {} commands and {} decisions to give no order on day {today}.",
            decisions.orders.len(),
            decisions.commands.len(),
            decisions.no_orders.len()
        )
    };
    let mut body = with_day(decisions_json(&n, &decisions), today);
    body["summary"] = json!(summary);
    answer("staff_orders", body)
}

/// The decisions' JSON with the day they are for, as `for_day`.
fn with_day(mut decisions: Value, day: u32) -> Value {
    decisions["for_day"] = json!(day);
    decisions
}

/// `why`: the staff's reason recorded in event number `number`, whole, with
/// what it is for. Anything that is not one of the side's own staff reasons
/// gets the one `not_known` answer.
pub fn why(view: &SideView<'_>, number: u32) -> Value {
    let events = view.events();
    let unknown = || serde_json::to_value(NotKnown::of("why")).expect("not_known serialises");
    let Some(event) = number.checked_sub(1).and_then(|i| events.get(i as usize)) else {
        return unknown();
    };
    let n = Names::new(view);
    let Some((for_json, reason)) = decision_of(&n, view, &events, event) else {
        return unknown();
    };
    answer(
        "why",
        json!({
            "number": number,
            "day": event.day,
            "pulse": event.pulse.map(|p| p.name()),
            "for": for_json,
            "reason": reason,
        }),
    )
}

// ---------------------------------------------------------------------------
// The commands a seat queues.
// ---------------------------------------------------------------------------

/// A command as the seat wrote it, read into the engine's own, with the
/// numbers written for ships and forces the side does not own kept so that a
/// refusal can show them.
#[derive(Clone)]
pub(crate) struct Request {
    pub kind: CommandKind,
    /// A stand-in id (above any unit's) to the ship number the side wrote.
    ship_stand_ins: BTreeMap<u32, u32>,
    force_stand_ins: BTreeMap<u32, u32>,
    group_stand_ins: BTreeMap<u32, u32>,
    /// The intent number was the side's next free one, chosen by the tool.
    pub number_chosen: bool,
}

/// The first id given to a number that names no unit of the side's.
const FIRST_STAND_IN: u32 = 4_000_000_000;

const COMMON_KEYS: [&str; 2] = ["clear", "reason"];

fn allowed_keys(tool: &str) -> &'static [&'static str] {
    match tool {
        "set_intent" => &["intent", "base", "floor_days", "interval_days"],
        "withdraw_intent" => &["intent"],
        "set_limit" => &[
            "limit",
            "intent",
            "ships",
            "task_forces",
            "source",
            "hexes",
            "rank",
            "base",
            "supply_t",
            "fuel_t",
            "risk",
        ],
        "set_leave" => &["leave", "given"],
        "approve" | "cancel_plan" => &["plan"],
        "hold" | "release" => &["ship", "task_force", "air_group", "setting"],
        _ => &[],
    }
}

/// Reads a seat's arguments of a command tool into a command. `next_intent`
/// is the side's next free intent number.
pub(crate) fn request(
    view: &SideView<'_>,
    tool: &str,
    args: &Map<String, Value>,
    next_intent: u32,
) -> Result<Request, CallError> {
    let allowed = allowed_keys(tool);
    for key in args.keys() {
        if !allowed.contains(&key.as_str()) && !COMMON_KEYS.contains(&key.as_str()) {
            return Err(CallError::naming(
                format!("\"{key}\" is not an argument of {tool}"),
                allowed,
            ));
        }
    }
    let ctx = Ctx::new(*view);
    let mut ship_stand_ins: BTreeMap<u32, u32> = BTreeMap::new();
    let mut force_stand_ins: BTreeMap<u32, u32> = BTreeMap::new();
    let mut group_stand_ins: BTreeMap<u32, u32> = BTreeMap::new();
    let mut made = 0;
    let mut body = Map::new();
    for (key, value) in args {
        if !COMMON_KEYS.contains(&key.as_str()) {
            body.insert(key.clone(), value.clone());
        }
    }
    body.insert("kind".into(), json!(tool));
    let mut number_chosen = false;
    if tool == "set_intent" && !body.contains_key("intent") {
        body.insert("intent".into(), json!(next_intent));
        number_chosen = true;
    }
    if tool == "set_limit" {
        let limit = body.get("limit").and_then(Value::as_str).unwrap_or("");
        if limit == "forces" {
            body.entry("ships").or_insert_with(|| json!([]));
            body.entry("task_forces").or_insert_with(|| json!([]));
        }
        // A weight written as a whole number is the same weight as its
        // decimal string.
        for key in ["supply_t", "fuel_t"] {
            if let Some(whole) = body.get(key).and_then(Value::as_i64) {
                body.insert(key.into(), json!(whole.to_string()));
            }
        }
        for (key, own_force) in [("ships", false), ("task_forces", true)] {
            let Some(Value::Array(numbers)) = body.get_mut(key) else {
                continue;
            };
            for item in numbers.iter_mut() {
                let Some(number) = item.as_u64().and_then(|n| u32::try_from(n).ok()) else {
                    continue;
                };
                let (raw, stand_ins) = if own_force {
                    (
                        ctx.own_force(number).map(|f| f.id.raw()),
                        &mut force_stand_ins,
                    )
                } else {
                    (
                        ctx.own_ship(number).map(|s| s.id.raw()),
                        &mut ship_stand_ins,
                    )
                };
                let id = match raw {
                    Some(raw) => raw,
                    None => match stand_ins.iter().find(|(_, written)| **written == number) {
                        Some((id, _)) => *id,
                        None => {
                            let id = FIRST_STAND_IN + made;
                            made += 1;
                            stand_ins.insert(id, number);
                            id
                        }
                    },
                };
                *item = json!(id);
            }
        }
    }
    if tool == "hold" || tool == "release" {
        if body.contains_key("setting") && !body.contains_key("air_group") {
            return Err(CallError::new(format!(
                "\"setting\" is an argument of {tool} only with \"air_group\" (the one setting that can be held is \"search_arc\")"
            )));
        }
        // The target is named by the side's number; a number that names none
        // of the side's units becomes one no unit has, so the engine refuses
        // an enemy unit and one that does not exist alike.
        for key in ["ship", "task_force", "air_group"] {
            let Some(number) = body
                .get(key)
                .and_then(Value::as_u64)
                .and_then(|n| u32::try_from(n).ok())
            else {
                continue;
            };
            let (raw, stand_ins) = match key {
                "ship" => (
                    ctx.own_ship(number).map(|s| s.id.raw()),
                    &mut ship_stand_ins,
                ),
                "task_force" => (
                    ctx.own_force(number).map(|f| f.id.raw()),
                    &mut force_stand_ins,
                ),
                _ => (
                    ctx.own_group(number).map(|g| g.id.raw()),
                    &mut group_stand_ins,
                ),
            };
            let id = match raw {
                Some(raw) => raw,
                None => match stand_ins.iter().find(|(_, written)| **written == number) {
                    Some((id, _)) => *id,
                    None => {
                        let id = FIRST_STAND_IN + made;
                        made += 1;
                        stand_ins.insert(id, number);
                        id
                    }
                },
            };
            body.insert(key.into(), json!(id));
        }
    }
    let kind = command_from_json(&Value::Object(body))
        .map_err(|e| CallError::new(format!("malformed command: {e}")))?;
    Ok(Request {
        kind,
        ship_stand_ins,
        force_stand_ins,
        group_stand_ins,
        number_chosen,
    })
}

/// The engine's verdict on the `index`th command of the side taken in on the
/// coming day, from the view of the world that took them in: `Ok` for
/// taken, `Err` for the refusal. `None` if there is no such command.
pub(crate) fn verdict(view: &SideView<'_>, index: usize) -> Option<Result<(), CommandRefusal>> {
    let side = view.side();
    let day = view.day() + 1;
    view.events_on_day(day)
        .into_iter()
        .filter_map(|e| match e.kind {
            SideEventKind::Whole(EventKind::CommandTaken { side: s, .. }) if s == side => {
                Some(Ok(()))
            }
            SideEventKind::Whole(EventKind::CommandRefused {
                side: s, reason, ..
            }) if s == side => Some(Err(reason)),
            _ => None,
        })
        .nth(index)
}

/// The answer to a command a seat queued: queued, or refused with the
/// engine's reason in words. `after` is the view of the world with the
/// command taken in (or refused).
pub(crate) fn command_answer(
    after: &SideView<'_>,
    request: &Request,
    tool: &str,
    verdict: &Result<(), CommandRefusal>,
    queued: usize,
) -> Value {
    let n = Names::new(after);
    let day = after.day() + 1;
    let command = command_json(&n, &request.kind, Some(request));
    let text = command_text(&n, &request.kind, Some(request));
    let mut out = json!({
        "tool": tool,
        "for_day": day,
        "command": command,
        "text": text,
        "queued": queued,
    });
    match verdict {
        Ok(()) => {
            out["status"] = json!("queued");
            let mut notes: Vec<String> = Vec::new();
            if let CommandKind::SetIntent { intent, .. } = &request.kind {
                if request.number_chosen {
                    out["intent"] = json!(intent.raw());
                    notes.push(format!(
                        "You gave no intent number: intent:{} is the side's next free number.",
                        intent.raw()
                    ));
                }
                let has_forces = after
                    .command_state()
                    .intents
                    .get(intent)
                    .is_some_and(|i| i.limits.forces.is_some());
                if !has_forces {
                    notes.push(format!(
                        "intent:{} has no forces limit yet: the staff gives no order for it until you set one (set_limit forces).",
                        intent.raw()
                    ));
                }
            }
            if matches!(request.kind, CommandKind::Approve { .. }) {
                notes.push(
                    "approve prepares a draft plan, or releases a plan that is preparing."
                        .to_string(),
                );
            }
            match &request.kind {
                CommandKind::Hold { target } => notes.push(format!(
                    "{} is held from the start of day {day}: the staff gives no order to it and plans without it. release lifts the hold.",
                    hold_target_words(&n, *target)
                )),
                CommandKind::Release { target } => notes.push(format!(
                    "{} is released from the start of day {day}: it is free to the staff again from that morning.",
                    hold_target_words(&n, *target)
                )),
                _ => {}
            }
            out["notes"] = json!(notes);
            out["summary"] = json!(format!(
                "Queued for the start of day {day}. {queued} command{} queued for day {day}.",
                if queued == 1 { " is" } else { "s are" }
            ));
        }
        Err(reason) => {
            out["status"] = json!("refused");
            out["reason"] = json!(n.text_with(
                &reason.to_string(),
                &request.ship_stand_ins,
                &request.force_stand_ins,
                &request.group_stand_ins
            ));
            out["notes"] = json!([]);
            out["summary"] = json!(format!(
                "Refused, and not queued. {queued} command{} queued for day {day}.",
                if queued == 1 { " is" } else { "s are" }
            ));
        }
    }
    answer("command", out)
}

/// The intent numbers ever set, the commands in `queued` counted: the side's
/// next free number is one past the highest.
pub(crate) fn next_free_intent(view: &SideView<'_>, queued: &[CommandKind]) -> u32 {
    let set = view
        .command_state()
        .intents
        .keys()
        .map(|id| id.raw())
        .max()
        .unwrap_or(0);
    let queued = queued
        .iter()
        .filter_map(|c| match c {
            CommandKind::SetIntent { intent, .. } => Some(intent.raw()),
            _ => None,
        })
        .max()
        .unwrap_or(0);
    set.max(queued) + 1
}

// ---------------------------------------------------------------------------
// Preview (decision 0107, step 3, rulings 25 to 28).
// ---------------------------------------------------------------------------

/// What a change that was previewed leaves of the plan's runs in hand
/// (ruling 28), written for a preview that makes no promise it cannot keep.
const MADE_READY_LOST: &str = "nothing: in a supply plan stock aboard a ship stays aboard and is carried by its next run, and stock landed stays where it lies";

/// The note a preview ends with.
const PREVIEW_NOTE: &str = "Nothing is changed by a preview. approve with {\"preview\": <n>} queues exactly this command. The preview is void once anything is queued or a day is resolved.";

/// The words for the commands in `commands`, in the side's numbers.
pub(crate) fn commands_text(view: &SideView<'_>, commands: &[CommandKind]) -> Vec<String> {
    let n = Names::new(view);
    commands
        .iter()
        .map(|command| command_text(&n, command, None))
        .collect()
}

/// The intent a previewed command is about: the intent set, or the intent
/// the limit is for. A reserve is for a base, not an intent: it is read as
/// about the lowest intent in force whose forces limit loads at that base.
fn previewed_intent(view: &SideView<'_>, kind: &CommandKind) -> Option<IntentId> {
    match kind {
        CommandKind::SetIntent { intent, .. } => Some(*intent),
        CommandKind::SetLimit(Limit::Reserve { base, .. }) => view
            .command_state()
            .intents_in_force()
            .find(|i| i.limits.forces.as_ref().is_some_and(|f| f.source == *base))
            .map(|i| i.id),
        CommandKind::SetLimit(limit) => limit.intent(),
        _ => None,
    }
}

/// The staff's decisions that serve `intent`; all of them where none.
fn decisions_for(decisions: &StaffDecisions, intent: Option<IntentId>) -> StaffDecisions {
    let Some(intent) = intent else {
        return decisions.clone();
    };
    StaffDecisions {
        orders: decisions
            .orders
            .iter()
            .filter(|o| o.intent == intent)
            .cloned()
            .collect(),
        commands: decisions
            .commands
            .iter()
            .filter(|c| c.reason.serves.intent == Some(intent))
            .cloned()
            .collect(),
        no_orders: decisions
            .no_orders
            .iter()
            .filter(|r| r.serves.intent == Some(intent))
            .cloned()
            .collect(),
        // A notice points into the whole hand-in by index; the filtered
        // decisions of a preview carry none.
        notices: Vec::new(),
    }
}

fn binds_json(n: &Names<'_>, proposal: &Proposal) -> Value {
    proposal.shortfall.as_ref().map_or(Value::Null, |s| {
        json!({
            "binds": n.text(&s.binds),
            "shortfall_days": dec(s.shortfall_days),
            "source_dry_day": s.source_dry_day,
            "check_day": s.check_day,
            "displaced_by": s.displaced_by.iter().map(|d| displacement_json(n, d)).collect::<Vec<_>>(),
        })
    })
}

/// What the staff would do for the intent on the coming day on one view: its
/// decisions (orders, commands, decisions to give no order), the runs of its
/// proposal, the next of them, the first day below the floor and what binds.
fn staff_side_json(
    n: &Names<'_>,
    staff: &dyn Staff,
    view: &SideView<'_>,
    intent: Option<IntentId>,
) -> (Value, StaffDecisions, Option<Proposal>) {
    let decisions = decisions_for(&staff.decide(*view), intent);
    let proposal = intent.and_then(|i| staff.propose(*view, i));
    let shown = decisions_json(n, &decisions);
    let runs: Vec<Value> = proposal
        .iter()
        .flat_map(|p| p.runs.iter())
        .map(|run| {
            json!({
                "ships": n.ships(&run.ships),
                "load_t": dec(run.load_t),
                "days": days_json(&run.days),
            })
        })
        .collect();
    let json = json!({
        "orders": shown["orders"],
        "commands": shown["commands"],
        "no_orders": shown["no_orders"],
        "next_run": runs.first().cloned(),
        "runs": runs,
        "first_day_below": proposal
            .as_ref()
            .and_then(|p| p.projection.as_ref())
            .and_then(|p| p.first_day_below),
        "binds": proposal.as_ref().map_or(Value::Null, |p| binds_json(n, p)),
    });
    (json, decisions, proposal)
}

/// The holds that keep a ship of the intent's pool from the staff (ruling
/// 18), in words.
fn held_words(n: &Names<'_>, view: &SideView<'_>, intent: Option<IntentId>) -> Vec<String> {
    let state = view.command_state();
    let Some(forces) = intent
        .and_then(|i| state.intents.get(&i))
        .and_then(|i| i.limits.forces.as_ref())
    else {
        return Vec::new();
    };
    let mut pool: std::collections::BTreeSet<ShipId> = forces.ships.clone();
    for force in &forces.task_forces {
        if let Some(force) = n.ctx.forces.get(force) {
            pool.extend(force.ships.iter().copied());
        }
    }
    let mut by_target: BTreeMap<HoldTarget, Vec<ShipId>> = BTreeMap::new();
    for ship in pool {
        let in_force = match n.ctx.ships.get(&ship).map(|s| s.location) {
            Some(ShipLocation::TaskForce(force)) => {
                n.ctx.forces.get(&force).map(|f| (force, f.ships.clone()))
            }
            _ => None,
        };
        let target = state.hold_on_ship(ship, in_force.as_ref().map(|(f, ships)| (*f, ships)));
        if let Some(target) = target {
            by_target.entry(target).or_default().push(ship);
        }
    }
    by_target
        .into_iter()
        .filter_map(|(target, ships)| {
            let hold = state.holds.get(&target)?;
            let maker = hold_maker_words(n, hold);
            let numbers = |own: Option<ShipId>| -> String {
                ships
                    .iter()
                    .filter(|s| Some(**s) != own)
                    .map(|s| n.ship(*s).to_string())
                    .collect::<Vec<_>>()
                    .join(", ")
            };
            let own = match target {
                HoldTarget::Ship(ship) => Some(ship),
                _ => None,
            };
            let others = numbers(own);
            let mut text = format!("{} is held by {maker}", hold_target_words(n, target));
            if !others.is_empty() {
                text.push_str(&format!(
                    ", so ship {others} of the pool {} not free either",
                    if others.contains(',') { "are" } else { "is" }
                ));
            }
            Some(text)
        })
        .collect()
}

/// For each run of the intent's plan that has not finished, what the change
/// does to it (ruling 28): a run that has sailed goes on to the base it was
/// sent to; a run still loading is sent to the new base; a change that is
/// not of the base leaves every run as ordered.
fn runs_in_hand(view: &SideView<'_>, intent: Option<IntentId>, kind: &CommandKind) -> Vec<String> {
    let state = view.command_state();
    let Some(intent) = intent.and_then(|i| state.intents.get(&i)) else {
        return Vec::new();
    };
    let Some(plan) = state.plans.get(&intent.plan) else {
        return Vec::new();
    };
    let IntentKind::KeepSupplied { base: old, .. } = intent.kind;
    let new_base = match kind {
        CommandKind::SetIntent {
            kind: IntentKind::KeepSupplied { base, .. },
            ..
        } if *base != old => Some(*base),
        _ => None,
    };
    let today = view.day() + 1;
    plan.runs
        .values()
        .filter(|run| run.ended.is_none())
        .filter_map(|run| {
            let (phase, _) = run_phase(run, today);
            let number = run.number;
            Some(match (phase, new_base) {
                ("finished", _) => return None,
                ("under_way", Some(_)) => format!(
                    "run {number} has sailed and goes on to base {}",
                    old.raw()
                ),
                ("returning", Some(_)) => format!(
                    "run {number} has delivered at base {} and is coming home; the change does not touch it",
                    old.raw()
                ),
                ("loading" | "waiting", Some(base)) => format!(
                    "run {number} is {} and is sent to base {}",
                    if phase == "loading" { "loading" } else { "not yet loading" },
                    base.raw()
                ),
                (phase, None) => format!(
                    "run {number} is {} and goes on as ordered",
                    match phase {
                        "waiting" => "not yet loading",
                        "loading" => "loading",
                        "under_way" => "under way",
                        _ => "coming home",
                    }
                ),
                _ => return None,
            })
        })
        .collect()
}

/// What other intents give up with the change: each other intent in force
/// the with-change proposal newly displaces, and each whose decisions for
/// the coming day differ between the two.
fn other_plans_give_up(
    staff: &dyn Staff,
    as_is: &SideView<'_>,
    with: &SideView<'_>,
    intent: Option<IntentId>,
    before: Option<&Proposal>,
    after: Option<&Proposal>,
) -> Vec<String> {
    let mut words = Vec::new();
    if let Some(after) = after {
        for d in &after.displaces {
            let already = before.is_some_and(|b| b.displaces.iter().any(|o| o.intent == d.intent));
            if already {
                continue;
            }
            let days = d.shortfall.shortfall_days;
            words.push(format!(
                "intent {} (plan {}) would be {days} {} short at the reading of day {}",
                d.intent.raw(),
                d.plan.raw(),
                if days == Fixed::ONE { "day" } else { "days" },
                d.shortfall.check_day
            ));
        }
    }
    let day = with.day() + 1;
    let (all_before, all_after) = (staff.decide(*as_is), staff.decide(*with));
    for other in with.command_state().intents_in_force() {
        if Some(other.id) == intent {
            continue;
        }
        let (a, b) = (
            decisions_for(&all_before, Some(other.id)),
            decisions_for(&all_after, Some(other.id)),
        );
        if a != b {
            if words
                .iter()
                .any(|w| w.starts_with(&format!("intent {} ", other.id.raw())))
            {
                continue;
            }
            words.push(format!(
                "intent {}: what the staff would do for it on day {day} differs with the change ({} orders, {} commands and {} decisions to give no order as things stand; {}, {} and {} with it)",
                other.id.raw(),
                a.orders.len(),
                a.commands.len(),
                a.no_orders.len(),
                b.orders.len(),
                b.commands.len(),
                b.no_orders.len()
            ));
        }
    }
    words
}

/// `preview`: what a changed intent or limit would do, worked on two copies
/// of the world as the coming day will find it: `as_is` with the side's
/// queued commands, and `with` the same and the command queued last. It is
/// stored nowhere in the game. `number` is the preview's number in the
/// session.
pub(crate) fn preview(
    number: u32,
    as_is: &SideView<'_>,
    with: &SideView<'_>,
    staff: &dyn Staff,
    request: &Request,
) -> Value {
    let n = Names::new(with);
    let intent = previewed_intent(with, &request.kind);
    let (now_json, _, now_proposal) = staff_side_json(&Names::new(as_is), staff, as_is, intent);
    let (with_json, _, with_proposal) = staff_side_json(&n, staff, with, intent);
    answer(
        "preview",
        json!({
            "preview": number,
            "day": with.day() + 1,
            "command": command_json(&n, &request.kind, Some(request)),
            "shown": command_text(&n, &request.kind, Some(request)),
            "intent": intent.map(|i| i.raw()),
            "as_things_stand": now_json,
            "with_change": with_json,
            "held": held_words(&n, with, intent),
            "runs_in_hand": runs_in_hand(as_is, intent, &request.kind),
            "made_ready_lost": MADE_READY_LOST,
            "other_plans_give_up": other_plans_give_up(
                staff,
                as_is,
                with,
                intent,
                now_proposal.as_ref(),
                with_proposal.as_ref(),
            ),
            "valid": true,
            "note": PREVIEW_NOTE,
        }),
    )
}

/// The refusal of a command that was not taken in as the tool read it: the
/// shape of a refused `command` answer, with words of the caller's own (a
/// void preview).
pub(crate) fn refused_answer(
    view: &SideView<'_>,
    request: &Request,
    tool: &str,
    reason: String,
    queued: usize,
) -> Value {
    let n = Names::new(view);
    let day = view.day() + 1;
    answer(
        "command",
        json!({
            "tool": tool,
            "for_day": day,
            "command": command_json(&n, &request.kind, Some(request)),
            "text": command_text(&n, &request.kind, Some(request)),
            "queued": queued,
            "status": "refused",
            "reason": reason,
            "notes": [],
            "summary": format!(
                "Refused, and not queued. {queued} command{} queued for day {day}.",
                if queued == 1 { " is" } else { "s are" }
            ),
        }),
    )
}
