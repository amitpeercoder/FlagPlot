//! Interrupts, the firings of a day and the command report (decision 0107,
//! items 29, 30 and 46, rulings 31 to 37).
//!
//! Everything here is worked from a side's view: the command state for the
//! interrupts set, and the side's own events for the firings, the staff's
//! notices and the reasons they rest on. The report is worked from events
//! alone and is stored nowhere, so it reads the same after a save and a
//! resume (ruling 37).

use std::collections::BTreeMap;

use serde_json::{json, Value};
use sim_core::command::{
    IntentKind, InterruptCondition, InterruptRef, InterruptStatus, NoticeKind, PlanId,
    StandingKind, StandingSubject,
};
use sim_core::events::EventKind;
use sim_core::fixed::Fixed;
use sim_core::orders::Issuer;
use sim_core::side_events::{SideCause, SideEvent, SideEventKind};
use sim_core::view::SideView;

use crate::staffing::{answer, dec, Names};

/// One interrupt of the side that fired on a morning, as its event gives it.
struct Firing {
    /// The side's number of the event.
    event: u32,
    day: u32,
    interrupt: InterruptRef,
    condition: Option<InterruptCondition>,
    figure: Option<Fixed>,
    /// The side's numbers of the events the firing rests on.
    causes: Vec<u32>,
}

fn firings_on(view: &SideView<'_>, events: &[SideEvent], day: u32) -> Vec<Firing> {
    let side = view.side();
    events
        .iter()
        .filter(|e| e.day == day)
        .filter_map(|e| {
            let SideEventKind::Whole(EventKind::InterruptFired {
                side: owner,
                interrupt,
                condition,
                figure,
            }) = &e.kind
            else {
                return None;
            };
            (*owner == side).then(|| Firing {
                event: e.number,
                day: e.day,
                interrupt: *interrupt,
                condition: *condition,
                figure: *figure,
                causes: e
                    .causes
                    .iter()
                    .filter_map(|c| match c {
                        SideCause::Event(k) => Some(*k),
                        SideCause::Order(_) => None,
                    })
                    .collect(),
            })
        })
        .collect()
}

/// Whether any interrupt of the side fired on `day`: what stops a run.
pub(crate) fn fired_on_day(view: &SideView<'_>, day: u32) -> bool {
    !firings_on(view, &view.events(), day).is_empty()
}

fn event_at(events: &[SideEvent], number: u32) -> Option<&SideEvent> {
    number.checked_sub(1).and_then(|i| events.get(i as usize))
}

/// The reason a staff notice rests on: the side's number of the
/// `StaffReason` event it cites.
fn reason_of_notice(events: &[SideEvent], notice: &SideEvent) -> Option<u32> {
    notice.causes.iter().find_map(|cause| {
        let SideCause::Event(k) = cause else {
            return None;
        };
        let cited = event_at(events, *k)?;
        matches!(
            cited.kind,
            SideEventKind::Whole(EventKind::StaffReason { .. })
        )
        .then_some(*k)
    })
}

/// The `why` numbers a firing rests on: the reasons of the notices it cites.
fn why_numbers(events: &[SideEvent], firing: &Firing) -> Vec<u32> {
    let mut numbers: Vec<u32> = firing
        .causes
        .iter()
        .filter_map(|k| event_at(events, *k))
        .filter(|cited| {
            matches!(
                cited.kind,
                SideEventKind::Whole(EventKind::StaffNotice { .. })
            )
        })
        .filter_map(|notice| reason_of_notice(events, notice))
        .collect();
    numbers.sort_unstable();
    numbers.dedup();
    numbers
}

fn status_text(status: &InterruptStatus) -> String {
    match status.last_fired {
        None => "never fired".to_string(),
        Some(day) if status.acknowledged => format!("last fired day {day}, acknowledged"),
        Some(day) => format!("last fired day {day}, not acknowledged"),
    }
}

fn status_of(view: &SideView<'_>, interrupt: &InterruptRef) -> Option<InterruptStatus> {
    let state = view.command_state();
    match interrupt {
        InterruptRef::Set(id) => state.interrupts.get(id).map(|set| set.status),
        InterruptRef::Standing { kind, subject } => state.standing.get(&(*kind, *subject)).copied(),
    }
}

/// The subject of a standing interrupt in the side's numbers, as the keys
/// `plan` or `ship`, and in words.
fn subject_json(n: &Names<'_>, subject: StandingSubject) -> (&'static str, u32, String) {
    match subject {
        StandingSubject::Plan(plan) => ("plan", plan.raw(), format!("plan:{}", plan.raw())),
        StandingSubject::Ship(ship) => ("ship", n.ship(ship), format!("ship:{}", n.ship(ship))),
    }
}

fn standing_words(kind: StandingKind, subject_words: &str) -> String {
    match kind {
        StandingKind::CannotMeet => format!("{subject_words} cannot meet its intent"),
        StandingKind::WaitsForLeave => format!("{subject_words} waits for the commander's leave"),
        StandingKind::ShipLost => format!("{subject_words} was lost"),
        StandingKind::CannotBringEnough => {
            format!("{subject_words} cannot bring the men it wants")
        }
    }
}

fn condition_words(n: &Names<'_>, condition: &InterruptCondition) -> String {
    let InterruptCondition::SupplyBelow { base, days } = condition;
    let handle = n.base(*base);
    format!(
        "{} below {days} days",
        handle["handle"].as_str().unwrap_or("")
    )
}

/// A firing as the answers write it, in the side's numbers.
fn firing_json(n: &Names<'_>, view: &SideView<'_>, events: &[SideEvent], firing: &Firing) -> Value {
    let acknowledged = status_of(view, &firing.interrupt).is_some_and(|s| s.acknowledged);
    let mut out = json!({
        "day": firing.day,
        "event": firing.event,
        "acknowledged": acknowledged,
        "why": why_numbers(events, firing),
    });
    match (&firing.interrupt, &firing.condition) {
        (InterruptRef::Set(id), condition) => {
            out["interrupt"] = json!(id.raw());
            let mut words = format!("interrupt:{} fired", id.raw());
            if let Some(InterruptCondition::SupplyBelow { base, days }) = condition {
                out["base"] = n.base(*base);
                out["below_days"] = json!(days);
                let handle = n.base(*base);
                words.push_str(&format!(
                    ", {} at {} days of supply, below {days}",
                    handle["handle"].as_str().unwrap_or(""),
                    firing
                        .figure
                        .map_or_else(|| "no figure of".to_string(), |f| f.to_string()),
                ));
            }
            out["figure"] = firing.figure.map_or(Value::Null, dec);
            out["words"] = json!(words);
        }
        (InterruptRef::Standing { kind, subject }, _) => {
            let (key, number, subject_words) = subject_json(n, *subject);
            out["standing"] = json!(kind.name());
            out[key] = json!(number);
            out["words"] = json!(format!(
                "standing {} fired: {}",
                kind.name(),
                standing_words(*kind, &subject_words)
            ));
        }
    }
    out
}

/// The firings of the side on `day`, each with what it is, its day, whether
/// it is acknowledged and, for a standing one on notices, the `why` numbers
/// of the reasons its notices rest on.
pub(crate) fn firings_json(view: &SideView<'_>, day: u32) -> Vec<Value> {
    let n = Names::new(view);
    let events = view.events();
    firings_on(view, &events, day)
        .iter()
        .map(|firing| firing_json(&n, view, &events, firing))
        .collect()
}

/// One entry of the interrupts part: a set interrupt or a standing record,
/// in the side's numbers, with whether it waits to be acknowledged.
fn needs_ack(status: &InterruptStatus) -> bool {
    status.last_fired.is_some() && !status.acknowledged
}

/// The interrupts part of `command_state` and of the report: every numbered
/// interrupt, every standing record that has fired, and the standing kinds
/// switched off. Those fired and not acknowledged come first (ruling 37).
pub(crate) fn interrupts_json(n: &Names<'_>, view: &SideView<'_>) -> Value {
    let state = view.command_state();
    let mut set: Vec<(bool, u32, Value, String)> = Vec::new();
    for (id, interrupt) in &state.interrupts {
        let status = interrupt.status;
        let InterruptCondition::SupplyBelow { base, days } = interrupt.condition;
        let text = format!(
            "interrupt:{} ({}), {}",
            id.raw(),
            condition_words(n, &interrupt.condition),
            status_text(&status)
        );
        set.push((
            !needs_ack(&status),
            id.raw(),
            json!({
                "interrupt": id.raw(),
                "base": n.base(base),
                "below_days": days,
                "set_day": interrupt.set_day,
                "armed": status.armed,
                "last_fired": status.last_fired,
                "acknowledged": status.acknowledged,
                "needs_acknowledgement": needs_ack(&status),
                "text": text,
            }),
            text,
        ));
    }
    set.sort_by_key(|(quiet, number, _, _)| (*quiet, *number));

    let mut standing: Vec<(bool, u8, u32, Value, String)> = Vec::new();
    for ((kind, subject), status) in &state.standing {
        if status.last_fired.is_none() {
            continue;
        }
        let (key, number, subject_words) = subject_json(n, *subject);
        let text = format!(
            "standing {} for {subject_words}, {}",
            kind.name(),
            status_text(status)
        );
        let mut entry = json!({
            "standing": kind.name(),
            "armed": status.armed,
            "last_fired": status.last_fired,
            "acknowledged": status.acknowledged,
            "needs_acknowledgement": needs_ack(status),
            "text": text,
        });
        entry[key] = json!(number);
        standing.push((!needs_ack(status), kind.code(), number, entry, text));
    }
    standing.sort_by_key(|(quiet, code, number, _, _)| (*quiet, *code, *number));

    let off: Vec<&'static str> = state.standing_off.iter().map(|kind| kind.name()).collect();
    let mut texts: Vec<String> = Vec::new();
    // Those fired and not acknowledged lead the words too.
    let mut all: Vec<(bool, String)> = set
        .iter()
        .map(|(quiet, _, _, text)| (*quiet, text.clone()))
        .chain(
            standing
                .iter()
                .map(|(quiet, _, _, _, text)| (*quiet, text.clone())),
        )
        .collect();
    all.sort_by_key(|(quiet, _)| *quiet);
    texts.extend(all.into_iter().map(|(_, text)| text));
    texts.extend(
        off.iter()
            .map(|kind| format!("standing {kind} switched off")),
    );
    json!({
        "set": set.into_iter().map(|(_, _, value, _)| value).collect::<Vec<_>>(),
        "standing": standing.into_iter().map(|(_, _, _, value, _)| value).collect::<Vec<_>>(),
        "switched_off": off,
        "text": texts,
    })
}

/// The words of a matter, as the notice's event reads.
fn matter_words(
    kind: NoticeKind,
    plan: PlanId,
    intent: Option<(u32, IntentKind)>,
    run: Option<u32>,
) -> String {
    match (kind, run) {
        // The notice that a "take a place" is met says the intent's words
        // (the ruling on decision 0142's reading 4).
        (NoticeKind::Met, _) if matches!(intent, Some((_, IntentKind::TakePlace { .. }))) => {
            format!("intent:{} is met", intent.map_or(0, |(id, _)| id))
        }
        (NoticeKind::CannotMeet, _) => match intent.map(|(id, _)| id) {
            Some(intent) => format!(
                "plan:{} (intent:{intent}) cannot meet its intent",
                plan.raw()
            ),
            None => format!("plan:{} cannot meet its intent", plan.raw()),
        },
        (NoticeKind::CannotBringEnough, _) => match intent.map(|(id, _)| id) {
            Some(intent) => format!(
                "plan:{} (intent:{intent}) cannot bring the men it wants",
                plan.raw()
            ),
            None => format!("plan:{} cannot bring the men it wants", plan.raw()),
        },
        (NoticeKind::WaitsForLeave(leave), _) => {
            format!("plan:{} waits for leave to {}", plan.raw(), leave.name())
        }
        (NoticeKind::RunEnded, Some(run)) => {
            format!("run {run} of plan:{} has ended", plan.raw())
        }
        (NoticeKind::RunEnded, None) => format!("a run of plan:{} has ended", plan.raw()),
        // Decision 0141: the notices of the two tasks. The plan names the
        // task's plan as for the others.
        (NoticeKind::Ready, _) => format!("plan:{} is ready", plan.raw()),
        (NoticeKind::Late, _) => format!("plan:{} cannot be ready by its date", plan.raw()),
        (NoticeKind::Met, _) => format!("plan:{} has every want met", plan.raw()),
    }
}

struct Matter {
    kind: NoticeKind,
    plan: PlanId,
    run: Option<u32>,
    /// The days it was handed in, each with the `why` number of the reason
    /// that day's notice cites.
    history: Vec<(u32, Option<u32>)>,
}

/// The matters of the side, worked from its events: a plan, a run where
/// there is one and a notice kind, with the days it was handed in.
fn matters(events: &[SideEvent], side: sim_core::state::Side) -> Vec<Matter> {
    let mut found: BTreeMap<(u32, Option<u32>, u8), Matter> = BTreeMap::new();
    for event in events {
        let SideEventKind::Whole(EventKind::StaffNotice {
            side: owner,
            kind,
            plan,
            run,
            ..
        }) = &event.kind
        else {
            continue;
        };
        if *owner != side {
            continue;
        }
        let matter = found
            .entry((plan.raw(), *run, kind.code()))
            .or_insert_with(|| Matter {
                kind: *kind,
                plan: *plan,
                run: *run,
                history: Vec::new(),
            });
        if matter.history.last().map(|(day, _)| *day) != Some(event.day) {
            matter
                .history
                .push((event.day, reason_of_notice(events, event)));
        }
    }
    found.into_values().collect()
}

fn count_text(count: usize, singular: &str, plural: &str) -> String {
    if count == 1 {
        format!("1 {singular}")
    } else {
        format!("{count} {plural}")
    }
}

/// The refusals of a game so far, counted by cause (decision 0138, 4).
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub(crate) struct RefusalCounts {
    /// Staff orders refused because a unit was not as the area command held
    /// it (decision 0136, 14).
    pub staff_orders_not_as_held: u32,
    /// Staff orders refused for any other cause.
    pub staff_orders_other: u32,
    /// Commands refused.
    pub commands: u32,
}

/// One refusal, as the count reads it.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) enum Refused {
    /// A staff order refused; `not_as_held` if the reason is
    /// [`sim_core::orders::RejectReason::is_not_as_held`].
    StaffOrder { not_as_held: bool },
    /// A command refused.
    Command,
}

/// The refusal an event is, if it is one: a staff order refused (an order
/// whose origin issuer, by `issuer_of`, is `Staff`) or a command refused. An
/// order `issuer_of` does not know is not counted.
fn refused_of(
    kind: &EventKind,
    issuer_of: impl Fn(sim_core::ids::OrderId) -> Option<Issuer>,
) -> Option<Refused> {
    match kind {
        EventKind::OrderRejected { order, reason, .. }
            if issuer_of(*order) == Some(Issuer::Staff) =>
        {
            Some(Refused::StaffOrder {
                not_as_held: reason.is_not_as_held(),
            })
        }
        EventKind::CommandRefused { .. } => Some(Refused::Command),
        _ => None,
    }
}

/// Counts refusals by cause.
pub(crate) fn tally(refusals: impl IntoIterator<Item = Refused>) -> RefusalCounts {
    let mut counts = RefusalCounts::default();
    for refused in refusals {
        match refused {
            Refused::StaffOrder { not_as_held: true } => counts.staff_orders_not_as_held += 1,
            Refused::StaffOrder { not_as_held: false } => counts.staff_orders_other += 1,
            Refused::Command => counts.commands += 1,
        }
    }
    counts
}

/// The refusals of the side, from its own events.
fn refusals_of(view: &SideView<'_>, events: &[SideEvent]) -> RefusalCounts {
    let issuers: BTreeMap<sim_core::ids::OrderId, Issuer> =
        view.orders().map(|o| (o.id, o.origin.issuer)).collect();
    tally(events.iter().filter_map(|e| match &e.kind {
        SideEventKind::Whole(kind) => refused_of(kind, |id| issuers.get(&id).copied()),
        _ => None,
    }))
}

/// `report`: what is open and what closed in the side's command of its
/// staff, and the interrupts, worked from the side's events and stored
/// nowhere (ruling 37).
pub(crate) fn report(view: &SideView<'_>) -> Value {
    let n = Names::new(view);
    let events = view.events();
    // The day the clock is in: the last day resolved, or the open day at
    // 0600 or 1200 (decision 0156), whose dusk has been and gone.
    let today = view.day_in_hand().map_or(view.day(), |in_hand| in_hand.day);
    let state = view.command_state();
    let mut found = matters(&events, view.side());
    // Open first, then closed; each by first day, then plan, run, kind code.
    let is_open = |m: &Matter| m.history.last().map(|(day, _)| *day) == Some(today);
    found.sort_by_key(|m| {
        (
            !is_open(m),
            m.history.first().map_or(0, |(day, _)| *day),
            m.plan.raw(),
            m.run,
            m.kind.code(),
        )
    });
    let mut open_words: Vec<String> = Vec::new();
    let mut closed = 0;
    let matters_json: Vec<Value> = found
        .iter()
        .map(|m| {
            let open = is_open(m);
            let opened = m.history.first().map_or(0, |(day, _)| *day);
            let last = m.history.last().map_or(0, |(day, _)| *day);
            let intent = state.plans.get(&m.plan).map(|p| p.intent);
            let intent_kind = intent.and_then(|id| state.intents.get(&id)).map(|i| i.kind);
            let intent = intent.map(|id| id.raw());
            let words = matter_words(m.kind, m.plan, intent.zip(intent_kind), m.run);
            if open {
                open_words.push(format!("{words} (since day {opened})"));
            } else {
                closed += 1;
            }
            json!({
                "plan": m.plan.raw(),
                "intent": intent,
                "run": m.run,
                "kind": m.kind.name(),
                "words": words,
                "open": open,
                "opened": opened,
                "closed": if open { Value::Null } else { json!(last + 1) },
                "history": m.history
                    .iter()
                    .map(|(day, why)| json!({"day": day, "why": why}))
                    .collect::<Vec<_>>(),
            })
        })
        .collect();
    let interrupts = interrupts_json(&n, view);
    let interrupt_texts: Vec<String> = interrupts["text"]
        .as_array()
        .into_iter()
        .flatten()
        .filter_map(|t| t.as_str().map(str::to_string))
        .collect();
    let mut summary = format!("Day {today}. ");
    if open_words.is_empty() {
        summary.push_str("No open matter. ");
    } else {
        summary.push_str(&format!(
            "{}: {}. ",
            count_text(open_words.len(), "open matter", "open matters"),
            open_words.join("; ")
        ));
    }
    if closed == 0 {
        summary.push_str("No matter closed. ");
    } else {
        summary.push_str(&format!(
            "{} closed. ",
            count_text(closed, "matter", "matters")
        ));
    }
    if interrupt_texts.is_empty() {
        summary.push_str("No interrupt set.");
    } else {
        summary.push_str(&format!("Interrupts: {}.", interrupt_texts.join("; ")));
    }
    // The hold intents in force and the measure of each (decision 0117, 14).
    let holds: Vec<Value> = state
        .intents_in_force()
        .filter(|intent| matches!(intent.kind, IntentKind::Hold { .. }))
        .filter_map(|intent| {
            let measure = crate::hold_measure(view, intent.id)?;
            let mut entry = crate::staffing::hold_measure_json(&Some(measure));
            entry["intent"] = json!(intent.id.raw());
            entry["base"] = json!(intent.kind.base().map(|base| n.base(base)));
            Some(entry)
        })
        .collect();
    for hold in &holds {
        summary.push_str(&format!(
            " Hold intent:{}: {}.",
            hold["intent"],
            hold["text"].as_str().unwrap_or("")
        ));
    }
    // The control-an-island intents in force and the measure of each
    // (decision 0151, 11).
    let islands: Vec<Value> = state
        .intents_in_force()
        .filter(|intent| matches!(intent.kind, IntentKind::ControlIsland { .. }))
        .filter_map(|intent| {
            let measure = crate::island_measure(view, intent.id)?;
            let mut entry = crate::staffing::island_measure_json(&Some(measure));
            entry["intent"] = json!(intent.id.raw());
            entry["island"] = json!(intent.kind.island());
            Some(entry)
        })
        .collect();
    for island in &islands {
        summary.push_str(&format!(
            " Island intent:{}: {}.",
            island["intent"],
            island["text"].as_str().unwrap_or("")
        ));
    }
    // The neutralise intents in force and the measure of each (decision
    // 0120, 8).
    let neutralisations: Vec<Value> = state
        .intents_in_force()
        .filter(|intent| matches!(intent.kind, IntentKind::Neutralise { .. }))
        .filter_map(|intent| {
            let measure = crate::neutralise_measure(view, intent.id)?;
            let mut entry = crate::staffing::neutralise_measure_json(&Some(measure));
            entry["intent"] = json!(intent.id.raw());
            entry["target"] = json!(intent.kind.base().map(|base| n.base(base)));
            Some(entry)
        })
        .collect();
    for neutralised in &neutralisations {
        summary.push_str(&format!(
            " Neutralise intent:{}: {}.",
            neutralised["intent"],
            neutralised["text"].as_str().unwrap_or("")
        ));
    }
    // The deny intents in force and the measure of each (decision 0123, 4).
    let denials: Vec<Value> = state
        .intents_in_force()
        .filter(|intent| matches!(intent.kind, IntentKind::Deny { .. }))
        .filter_map(|intent| {
            let measure = crate::deny_measure(view, intent.id)?;
            let mut entry = crate::staffing::deny_measure_json(&Some(measure));
            entry["intent"] = json!(intent.id.raw());
            if let Some((hex, distance)) = intent.kind.waters() {
                entry["hex"] = json!([hex.q, hex.r]);
                entry["distance"] = json!(distance);
            }
            Some(entry)
        })
        .collect();
    for denied in &denials {
        summary.push_str(&format!(
            " Deny intent:{}: {}.",
            denied["intent"],
            denied["text"].as_str().unwrap_or("")
        ));
    }
    // Refusals by cause, over the whole game (decision 0138, 4): a sentence
    // only when there is one.
    let refusals = refusals_of(view, &events);
    if refusals != RefusalCounts::default() {
        let mut parts: Vec<String> = Vec::new();
        if refusals.staff_orders_not_as_held > 0 {
            parts.push(format!(
                "{} refused because the force was not as the area command held it",
                count_text(
                    refusals.staff_orders_not_as_held as usize,
                    "staff order",
                    "staff orders"
                )
            ));
        }
        if refusals.staff_orders_other > 0 {
            parts.push(format!(
                "{} refused for another cause",
                count_text(
                    refusals.staff_orders_other as usize,
                    "staff order",
                    "staff orders"
                )
            ));
        }
        if refusals.commands > 0 {
            parts.push(format!(
                "{} refused",
                count_text(refusals.commands as usize, "command", "commands")
            ));
        }
        summary.push_str(&format!(" Refusals: {}.", parts.join("; ")));
    }
    answer(
        "report",
        json!({
            "day": today,
            "refusals": {
                "staff_orders_not_as_held": refusals.staff_orders_not_as_held,
                "staff_orders_other": refusals.staff_orders_other,
                "commands": refusals.commands,
            },
            "matters": matters_json,
            "holds": holds,
            "islands": islands,
            "neutralisations": neutralisations,
            "denials": denials,
            "open_matters": open_words.len(),
            "closed_matters": closed,
            "interrupts": interrupts,
            "fired_last_day": firings_json(view, today),
            "summary": summary,
        }),
    )
}

/// The words that say why a run stopped, for its summary.
pub(crate) fn firing_words(firings: &[Value]) -> String {
    firings
        .iter()
        .filter_map(|f| f["words"].as_str())
        .collect::<Vec<_>>()
        .join("; ")
}

#[cfg(test)]
mod refusal_tests {
    use super::*;

    /// A log with two staff orders refused as not as held, one refused for
    /// another cause and two commands refused is counted as 2, 1 and 2; the
    /// staff's orders and the commands are all there is to count.
    #[test]
    fn refusals_are_counted_by_cause() {
        let log = [
            Refused::StaffOrder { not_as_held: true },
            Refused::StaffOrder { not_as_held: true },
            Refused::StaffOrder { not_as_held: false },
            Refused::Command,
            Refused::Command,
        ];
        assert_eq!(
            tally(log),
            RefusalCounts {
                staff_orders_not_as_held: 2,
                staff_orders_other: 1,
                commands: 2,
            }
        );
        assert_eq!(tally([]), RefusalCounts::default());
    }
}
