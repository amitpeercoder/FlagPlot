//! A side's events together (decision 0156, rule 9, the tool `events`), and
//! what a side was told in a pulse, for `advance` to stop on (rule 8).
//!
//! Both read the side's own events from its view and nothing else. `events`
//! gives, for each event, the facts `explain` gives of the same event as its
//! node 0: so it tells nothing `explain` does not tell one at a time. What
//! `advance` stops on is read from the same events, as `run` reads the
//! firings of a day: no state is added.

use serde_json::{json, Value};
use sim_core::events::EventKind;
use sim_core::side_events::{SideEvent, SideEventKind};
use sim_core::state::{ShipSystem, SystemState};
use sim_core::time::Pulse;
use sim_core::view::SideView;

use crate::answer::{FORMAT, VERSION};
use crate::explain::{ExplainAnswer, Explainer};

/// Which part of a day `events` is asked for: a pulse, or the events of the
/// day that carry no pulse (`end`).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum DayPart {
    Pulse(Pulse),
    /// The events of the day that carry no pulse: those of its dusk (the
    /// orders and commands taken in) and of its end (the fight ashore, the
    /// supply, the count of the bases).
    End,
}

impl DayPart {
    /// The part written `night`, `morning`, `afternoon` or `end`.
    pub fn parse(text: &str) -> Option<DayPart> {
        match text {
            "night" => Some(DayPart::Pulse(Pulse::Night)),
            "morning" => Some(DayPart::Pulse(Pulse::Morning)),
            "afternoon" => Some(DayPart::Pulse(Pulse::Afternoon)),
            "end" => Some(DayPart::End),
            _ => None,
        }
    }

    pub const fn name(self) -> &'static str {
        match self {
            DayPart::Pulse(pulse) => pulse.name(),
            DayPart::End => "end",
        }
    }

    fn holds(self, pulse: Option<Pulse>) -> bool {
        match self {
            DayPart::Pulse(p) => pulse == Some(p),
            DayPart::End => pulse.is_none(),
        }
    }
}

/// The answer to `events`: the side's own events of `day`, or of `part` of
/// it, in order, each as `explain` gives it. `begun` is false for a day the
/// game has not yet begun: the list is then empty and the words say so.
pub fn events_answer(
    view: &SideView<'_>,
    day: u32,
    part: Option<DayPart>,
    begun: bool,
    clock_words: &str,
) -> Value {
    let explainer = Explainer::new(view);
    let mut entries: Vec<Value> = Vec::new();
    if begun {
        for number in explainer.event_numbers() {
            let ExplainAnswer::Found(found) = explainer.explain(number) else {
                continue;
            };
            let node = &found.nodes[0];
            if node.day != Some(day) {
                continue;
            }
            let pulse = view_pulse(node.pulse);
            if part.is_some_and(|p| !p.holds(pulse)) {
                continue;
            }
            entries.push(json!({
                "number": node.id,
                "day": node.day,
                "pulse": node.pulse,
                "event_kind": node.event_kind,
                "text": node.text,
                "entities": node.entities,
            }));
        }
    }
    let of = match part {
        Some(DayPart::End) => format!("the events of day {day} that carry no pulse"),
        Some(DayPart::Pulse(pulse)) => format!("the {} pulse of day {day}", pulse.name()),
        None => format!("day {day}"),
    };
    let summary = if !begun {
        format!("Day {day} has not begun: there are no events of it yet. The clock stands at {clock_words}.")
    } else {
        let count = entries.len();
        format!(
            "{} of yours in {of}, in order; the clock stands at {clock_words}. explain takes any of their numbers.",
            if count == 1 {
                "1 event".to_string()
            } else {
                format!("{count} events")
            }
        )
    };
    json!({
        "format": FORMAT,
        "version": VERSION,
        "kind": "events",
        "day": day,
        "pulse": part.map(DayPart::name),
        "begun": begun,
        "count": entries.len(),
        "events": entries,
        "summary": summary,
    })
}

/// The pulse of an explained node back from its name.
fn view_pulse(name: Option<&str>) -> Option<Pulse> {
    match name {
        Some("night") => Some(Pulse::Night),
        Some("morning") => Some(Pulse::Morning),
        Some("afternoon") => Some(Pulse::Afternoon),
        _ => None,
    }
}

/// The things `advance` can stop on (decision 0156, rule 8), in the order
/// the interface lists them.
pub const STOP_ON: [&str; 5] = ["contact", "attack", "loss", "strike", "landing"];

/// Which of [`STOP_ON`] the side was told by one of its events, if any. The
/// event kinds behind each:
///
/// - `contact`: `report_received` of the side's own with `created` true (a
///   report that made a new contact, not one that refreshed a contact held).
/// - `attack`: `air_attack_against` (an air attack on one of its ships),
///   `base_attacked_against` (a raid on one of its bases),
///   `shore_bombardment_against` (one of its bases shelled from the sea),
///   `salvo_against` (one of its ships fired at in a surface fight) and
///   `engagement_begun` (a surface fight its force is in has begun).
/// - `loss`: `ship_sunk` or `ship_lost_with_base` of one of its ships, and
///   `system_hit` of one of its
///   ships whose propulsion or flight deck went out.
/// - `strike`: `strike_launched` and `base_strike_launched` of one of its
///   air groups.
/// - `landing`: `men_landed` of one of its land units, and `ground_fight`, a
///   fight ashore, which a side is told of only where it has men in the
///   fight.
pub fn told_as(event: &SideEvent) -> Option<&'static str> {
    match &event.kind {
        SideEventKind::Whole(EventKind::ReportReceived { created: true, .. }) => Some("contact"),
        SideEventKind::AirAttackAgainst { .. }
        | SideEventKind::BaseAttackedAgainst { .. }
        | SideEventKind::ShoreBombardmentAgainst { .. }
        | SideEventKind::SalvoAgainst { .. }
        | SideEventKind::EngagementBegun { .. } => Some("attack"),
        SideEventKind::Whole(EventKind::ShipSunk { .. } | EventKind::ShipLostWithBase { .. }) => {
            Some("loss")
        }
        SideEventKind::Whole(EventKind::SystemHit {
            system: ShipSystem::Propulsion | ShipSystem::FlightDeck,
            before,
            after: SystemState::Out,
            ..
        }) if *before != SystemState::Out => Some("loss"),
        SideEventKind::Whole(
            EventKind::StrikeLaunched { .. } | EventKind::BaseStrikeLaunched { .. },
        ) => Some("strike"),
        SideEventKind::Whole(EventKind::MenLanded { .. }) | SideEventKind::GroundFight { .. } => {
            Some("landing")
        }
        _ => None,
    }
}

/// What the side was told among its events numbered above `before` (the
/// count it held before), for each of `wanted`: the thing, with the numbers
/// of its events. Things with no event are left out.
pub fn told_since(view: &SideView<'_>, before: usize, wanted: &[&str]) -> Vec<Value> {
    let events = view.events();
    let mut out: Vec<Value> = Vec::new();
    for thing in STOP_ON {
        if !wanted.contains(&thing) {
            continue;
        }
        let numbers: Vec<u32> = events
            .iter()
            .enumerate()
            .skip(before)
            .filter(|(_, e)| told_as(e) == Some(thing))
            .map(|(i, _)| u32::try_from(i + 1).expect("few events"))
            .collect();
        if !numbers.is_empty() {
            out.push(json!({"thing": thing, "events": numbers}));
        }
    }
    out
}
