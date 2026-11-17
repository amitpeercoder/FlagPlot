//! When a report reached a command, and by which road (decision 0138, item
//! 1). Whether a command held a report and when it first did is read from the
//! state (the watcher of `events.rs` notes the first moment each command's
//! heard set holds the report's event); the road it came by is read from the
//! log: the report's `ReportSent` events, their senders, directions and
//! arrivals. The two are set beside each other, and a disagreement is said.
//! Like all of `sim-cli events` it is the umpire's and reads the truth.

use std::collections::BTreeMap;

use sim_core::choices::Moment;
use sim_core::events::{Arrival, Cause, Direction, Event, EventKind};
use sim_core::ids::UmpireKey;
use sim_core::pictures::CommandRef;
use sim_core::state::Side;
use sim_core::time::Pulse;
use sim_core::World;

/// A day and a pulse; `None` is the end of the day, which belongs to no
/// pulse.
pub type When = (u32, Option<Pulse>);

/// One `ReportSent` event of a report.
#[derive(Clone, Debug)]
pub struct Sent {
    pub event: u32,
    pub side: Side,
    pub when: When,
    pub sender: CommandRef,
    pub listeners: Vec<sim_core::events::ReportListener>,
    /// The `ReportSent` events among its causes: for a passing down, the
    /// passing up that brought the report to the area command.
    pub brought_by: Vec<u32>,
}

/// Every `ReportSent` of the report with the event number `report`, in the
/// order of the log.
pub fn sents_of(world: &World, key: &UmpireKey, report: u32) -> Vec<Sent> {
    let is_sent: BTreeMap<u32, ()> = world
        .events()
        .iter()
        .filter(|e| matches!(e.kind, EventKind::ReportSent { .. }))
        .map(|e| (e.id.umpire_raw(key), ()))
        .collect();
    world
        .events()
        .iter()
        .filter_map(|event| {
            let EventKind::ReportSent {
                side,
                sender,
                report: sent_report,
                listeners,
            } = &event.kind
            else {
                return None;
            };
            if sent_report.umpire_raw(key) != report {
                return None;
            }
            let brought_by = event
                .causes
                .iter()
                .filter_map(|cause| match cause {
                    Cause::Event(id) => Some(id.umpire_raw(key)),
                    Cause::Order(_) => None,
                })
                .filter(|n| *n != report && is_sent.contains_key(n))
                .collect();
            Some(Sent {
                event: event.id.umpire_raw(key),
                side: *side,
                when: (event.day, event.pulse),
                sender: *sender,
                listeners: listeners.clone(),
                brought_by,
            })
        })
        .collect()
}

/// How a command came to hold a report.
#[derive(Clone, Debug)]
pub enum Road {
    /// It made the report (or holds it with no passing to it that arrived).
    Own { when: When },
    /// A passing brought it.
    Told {
        event: u32,
        sender: CommandRef,
        direction: Direction,
        arrival: Arrival,
        /// The day and pulse it reached the command.
        when: When,
        /// For a passing down: the passing up behind it, if the area command
        /// had the report from one.
        up_behind: Option<(u32, CommandRef, Arrival)>,
    },
    /// A passing to it was a pulse late and, when it fell due, the game had
    /// ended or the command had ceased to be one (a listener that is no
    /// longer a command of the side is dropped): it never held the report.
    Overtaken {
        event: u32,
        sender: CommandRef,
        direction: Direction,
        due: When,
        game_ended: bool,
    },
    /// No passing to it arrived and it did not make the report. What was
    /// sent to it: the passing, its sender, direction and arrival.
    Never {
        sent_to_it: Vec<(u32, CommandRef, Direction, Arrival)>,
    },
}

/// The pulse after `(day, pulse)`; the end of a day counts as its afternoon.
fn next_pulse(day: u32, pulse: Option<Pulse>) -> When {
    match pulse {
        Some(Pulse::Night) => (day, Some(Pulse::Morning)),
        Some(Pulse::Morning) => (day, Some(Pulse::Afternoon)),
        Some(Pulse::Afternoon) | None => (day + 1, Some(Pulse::Night)),
    }
}

/// When a passing sent at `sent` arrives, and the order it is heard in: a
/// late one at the start of its pulse, before anything of it, so before an
/// on-time one of the same pulse.
fn arrival_key(sent: When, arrival: Arrival, event: u32) -> ((u32, u8, u8, u32), When) {
    let slot = |when: When| match when.1 {
        Some(Pulse::Night) => 0,
        Some(Pulse::Morning) => 1,
        Some(Pulse::Afternoon) => 2,
        None => 3,
    };
    match arrival {
        Arrival::OnTime => ((sent.0, slot(sent), 1, event), sent),
        Arrival::Late => {
            let at = next_pulse(sent.0, sent.1);
            ((at.0, slot(at), 0, event), at)
        }
        Arrival::Never => ((u32::MAX, 0, 0, event), sent),
    }
}

/// The road by which `command` came to hold the report whose event is at
/// `made`, given its `ReportSent` events and whether the state says it held
/// the report.
pub fn road(command: CommandRef, made: When, sents: &[Sent], held: bool) -> Road {
    // It sent the report up or sideways: the passing of its own report.
    if sents.iter().any(|sent| {
        sent.sender == command
            && sent
                .listeners
                .iter()
                .any(|l| matches!(l.direction, Direction::Up | Direction::Sideways))
    }) {
        return Road::Own { when: made };
    }
    // The area command sending it down with no passing up behind it made it
    // itself.
    if command == CommandRef::Area
        && sents
            .iter()
            .any(|sent| sent.sender == CommandRef::Area && sent.brought_by.is_empty())
    {
        return Road::Own { when: made };
    }
    // The first passing to it that arrived.
    let mut arrived: Vec<_> = sents
        .iter()
        .flat_map(|sent| {
            sent.listeners
                .iter()
                .filter(move |l| l.command == command && l.arrival != Arrival::Never)
                .map(move |l| {
                    let (key, at) = arrival_key(sent.when, l.arrival, sent.event);
                    (key, at, sent, *l)
                })
        })
        .collect();
    arrived.sort_by_key(|(key, ..)| *key);
    if let Some((_, at, sent, listener)) = arrived.first() {
        let up_behind = match listener.direction {
            Direction::Down => sent.brought_by.first().and_then(|up| {
                let up_sent = sents.iter().find(|s| s.event == *up)?;
                let arrival = up_sent
                    .listeners
                    .iter()
                    .find(|l| l.command == CommandRef::Area)
                    .map(|l| l.arrival)?;
                Some((up_sent.event, up_sent.sender, arrival))
            }),
            _ => None,
        };
        return Road::Told {
            event: sent.event,
            sender: sent.sender,
            direction: listener.direction,
            arrival: listener.arrival,
            when: *at,
            up_behind,
        };
    }
    if held {
        return Road::Own { when: made };
    }
    Road::Never {
        sent_to_it: sents
            .iter()
            .flat_map(|sent| {
                sent.listeners
                    .iter()
                    .filter(move |l| l.command == command)
                    .map(move |l| (sent.event, sent.sender, l.direction, l.arrival))
            })
            .collect(),
    }
}

/// "in the afternoon of day 1", "at the end of day 3".
pub fn when_words(when: When) -> String {
    match when.1 {
        Some(pulse) => format!("in the {} of day {}", pulse.name(), when.0),
        None => format!("at the end of day {}", when.0),
    }
}

fn arrival_words(arrival: Arrival) -> &'static str {
    match arrival {
        Arrival::OnTime => "on time",
        Arrival::Late => "a pulse late",
        Arrival::Never => "lost",
    }
}

/// The road in words, without its subject: "sent sideways by task_force:1
/// (event #33), a pulse late".
pub fn road_words(road: &Road) -> String {
    match road {
        Road::Own { .. } => "its own (it made the report)".to_string(),
        Road::Overtaken { .. } => "never held in the game".to_string(),
        Road::Told {
            event,
            sender,
            direction,
            arrival,
            up_behind,
            ..
        } => {
            let mut text = format!(
                "sent {} by {sender} (event #{event}), {}",
                direction.name(),
                arrival_words(*arrival)
            );
            if *direction == Direction::Down {
                match up_behind {
                    Some((up, from, up_arrival)) => text.push_str(&format!(
                        "; the area command had it from the passing up by {from} (event #{up}, {})",
                        arrival_words(*up_arrival)
                    )),
                    None => text.push_str("; the area command made the report itself"),
                }
            }
            text
        }
        Road::Never { .. } => "never".to_string(),
    }
}

/// The one plain line of an answer: when `command` first held report
/// `report` and by which road, or that it never did.
pub fn line(command: CommandRef, report: u32, road: &Road) -> String {
    match road {
        Road::Own { when } | Road::Told { when, .. } => format!(
            "{command} first held report #{report} {}: {}.",
            when_words(*when),
            road_words(road)
        ),
        Road::Overtaken {
            event,
            sender,
            direction,
            due,
            game_ended,
        } => format!(
            "{command} never held report #{report} (sent {} by {sender}, event #{event}, a pulse late: it fell due {} and {}).",
            direction.name(),
            when_words(*due),
            if *game_ended {
                "the game had ended"
            } else {
                "the command had ceased to be one"
            }
        ),
        Road::Never { sent_to_it } => {
            let why = if sent_to_it.is_empty() {
                format!("no passing of it was sent to {command}")
            } else {
                sent_to_it
                    .iter()
                    .map(|(event, sender, direction, arrival)| {
                        format!(
                            "sent {} by {sender}, event #{event}: {}",
                            direction.name(),
                            arrival_words(*arrival)
                        )
                    })
                    .collect::<Vec<_>>()
                    .join("; ")
            };
            format!("{command} never held report #{report} ({why}).")
        }
    }
}

/// The attributions of a moment: the days and pulses an event first seen at
/// that moment may have happened in. A late report is heard before the first
/// phase of its pulse, so first seen at a phase of that pulse; an on-time one
/// is heard in a phase and first seen at the next moment, which may be the
/// start of the next pulse or the first step of the end of the day.
fn attributions(moment: Moment) -> Vec<When> {
    let before = |day: u32, pulse: Pulse| -> Vec<When> {
        match pulse {
            Pulse::Night => vec![
                (day.saturating_sub(1), Some(Pulse::Afternoon)),
                (day.saturating_sub(1), None),
            ],
            Pulse::Morning => vec![(day, Some(Pulse::Night))],
            Pulse::Afternoon => vec![(day, Some(Pulse::Morning))],
        }
    };
    match moment {
        Moment::DayBegins { day } | Moment::Orders { day } => {
            vec![
                (day.saturating_sub(1), None),
                (day.saturating_sub(1), Some(Pulse::Afternoon)),
                (day, None),
            ]
        }
        Moment::PulseBegins { day, pulse } => before(day, pulse),
        Moment::Phase { day, pulse, .. } => vec![(day, Some(pulse))],
        Moment::EndOfDay { day, .. } => vec![(day, None), (day, Some(Pulse::Afternoon))],
    }
}

/// Whether the moment the state first showed the report held is one the road
/// says it can have been held in.
pub fn agrees(first_seen: Moment, road: &Road) -> bool {
    let when = match road {
        Road::Own { when } | Road::Told { when, .. } => *when,
        Road::Never { .. } | Road::Overtaken { .. } => return false,
    };
    attributions(first_seen).contains(&when)
}

/// A report's own day and pulse: the event it is known by.
pub fn made_when(event: &Event) -> When {
    (event.day, event.pulse)
}

/// One command's row of the answer: where the state first showed it holding
/// the report and the road the log gives.
pub struct Row {
    pub command: CommandRef,
    pub first_seen: Option<Moment>,
    pub road: Road,
}

/// The answer to "when did the commands of the side hold report N".
pub enum Answers {
    /// Command levels are off, or reports do not travel: no command differs.
    Everyone { side: Side, text: String },
    PerCommand {
        side: Side,
        report: u32,
        rows: Vec<Row>,
    },
}

/// A moment in words, for the note of a disagreement.
pub fn moment_words(moment: Moment) -> String {
    match moment {
        Moment::DayBegins { day } => format!("the beginning of day {day}"),
        Moment::Orders { day } => format!("the orders of day {day}"),
        Moment::PulseBegins { day, pulse } => {
            format!("the start of the {} of day {day}", pulse.name())
        }
        Moment::Phase { day, pulse, phase } => {
            format!(
                "the {} of day {day}, before its {phase:?} step",
                pulse.name()
            )
        }
        Moment::EndOfDay { day, phase } => {
            format!("the end of day {day}, before its {phase:?} step")
        }
    }
}

/// The note on a row, if the state and the log do not agree: a command the
/// state shows holding the report in a moment the road does not allow, or one
/// the road says held it and the state never showed so.
fn note_of(row: &Row) -> Option<String> {
    match (&row.road, row.first_seen) {
        (Road::Never { .. } | Road::Overtaken { .. }, None) => None,
        (Road::Never { .. } | Road::Overtaken { .. }, Some(_)) => Some(format!(
            "note: {} holds the report in the state but the log gives no road to it",
            row.command
        )),
        (_, None) => Some(format!(
            "note: the state never showed {} holding the report, though the log gives a road to it (it may have ceased to be a command first)",
            row.command
        )),
        (road, Some(moment)) => (!agrees(moment, road)).then(|| {
            format!(
                "note: the state first showed {} holding the report at {}, which does not agree with the road named (a gap in the reading of the road, or a command formed by copy: tell the architect)",
                row.command,
                moment_words(moment)
            )
        }),
    }
}

/// The lines of the answer, plain.
pub fn answers_text(answers: &Answers) -> Vec<String> {
    match answers {
        Answers::Everyone { side, text } => {
            vec![format!("the {} side: {text}.", side.name())]
        }
        Answers::PerCommand { report, rows, .. } => {
            let mut lines = Vec::new();
            for row in rows {
                lines.push(line(row.command, *report, &row.road));
                if let Some(note) = note_of(row) {
                    lines.push(note);
                }
            }
            lines
        }
    }
}

/// The answer as one JSON value.
pub fn answers_json(report: u32, answers: &Answers) -> serde_json::Value {
    use serde_json::json;
    let when = |when: When| json!({ "day": when.0, "pulse": when.1.map(|p| p.name()) });
    match answers {
        Answers::Everyone { side, text } => json!({
            "report": report,
            "side": side.name(),
            "everyone": text,
        }),
        Answers::PerCommand { side, rows, .. } => {
            let rows: Vec<_> = rows
                .iter()
                .map(|row| {
                    let mut value = match &row.road {
                        Road::Own { when: at } => json!({
                            "held": true,
                            "road": "own",
                            "first_held": when(*at),
                        }),
                        Road::Told {
                            event,
                            sender,
                            direction,
                            arrival,
                            when: at,
                            up_behind,
                        } => json!({
                            "held": true,
                            "road": direction.name(),
                            "sent_by": sender.to_string(),
                            "passing_event": event,
                            "arrival": arrival.name(),
                            "first_held": when(*at),
                            "up_passing_event": up_behind.map(|(up, ..)| up),
                        }),
                        Road::Overtaken {
                            event,
                            sender,
                            direction,
                            due,
                            game_ended,
                        } => json!({
                            "held": false,
                            "road": "overtaken",
                            "game_ended": game_ended,
                            "sent_by": sender.to_string(),
                            "passing_event": event,
                            "direction": direction.name(),
                            "due": when(*due),
                        }),
                        Road::Never { sent_to_it } => json!({
                            "held": false,
                            "road": "never",
                            "sent_to_it": sent_to_it
                                .iter()
                                .map(|(event, sender, direction, arrival)| json!({
                                    "event": event,
                                    "sender": sender.to_string(),
                                    "direction": direction.name(),
                                    "arrival": arrival.name(),
                                }))
                                .collect::<Vec<_>>(),
                        }),
                    };
                    value["command"] = json!(row.command.to_string());
                    value["text"] = json!(line(row.command, report, &row.road));
                    if let Some(note) = note_of(row) {
                        value["note"] = json!(note);
                    }
                    value
                })
                .collect();
            json!({ "report": report, "side": side.name(), "commands": rows })
        }
    }
}
