//! Why a group did not launch (decision 0138, item 2): the review of one
//! naval strike group's decision in one pulse, from the picture of its
//! command at the moment of the decision, set beside what the log says it
//! launched. The review is `SideView::strike_review`; this prints it. Like
//! all of `sim-cli events` it is the umpire's.

use sim_core::events::EventKind;
use sim_core::ids::AirGroupId;
use sim_core::time::Pulse;
use sim_core::view::strike_review::{ContactFate, StrikeReview, StrikeStop};
use sim_core::World;

use super::{report_answers, report_road, Watched};

/// What was asked.
pub struct Asked {
    pub group: AirGroupId,
    pub day: u32,
    pub pulse: Pulse,
    pub report: Option<u32>,
}

/// What the log says a group launched in a pulse: the number of the event and
/// what it names.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct Launched {
    event: u32,
    contact: u32,
    hex: sim_core::hex::Hex,
    aircraft: u32,
    distance_nm: sim_core::fixed::Fixed,
}

fn launched_in_log(world: &World, asked: &Asked) -> Vec<Launched> {
    let key = world.umpire_key();
    world
        .events()
        .iter()
        .filter(|event| event.day == asked.day && event.pulse == Some(asked.pulse))
        .filter_map(|event| match &event.kind {
            EventKind::StrikeLaunched {
                air_group,
                contact,
                hex,
                aircraft,
                distance_nm,
            } if *air_group == asked.group => Some(Launched {
                event: event.id.umpire_raw(&key),
                contact: contact.umpire_raw(&key),
                hex: *hex,
                aircraft: *aircraft,
                distance_nm: *distance_nm,
            }),
            _ => None,
        })
        .collect()
}

fn kind_name(kind: sim_core::refdata::ShipKind) -> String {
    format!("{kind:?}").to_lowercase()
}

fn mission_name(mission: sim_core::orders::AirMission) -> String {
    match mission {
        sim_core::orders::AirMission::Rest => "rest".to_string(),
        sim_core::orders::AirMission::Search => "search".to_string(),
        sim_core::orders::AirMission::NavalStrike => "naval strike".to_string(),
        sim_core::orders::AirMission::CombatAirPatrol => "combat air patrol".to_string(),
        sim_core::orders::AirMission::BaseStrike(base) => format!("base strike on {base}"),
        sim_core::orders::AirMission::Reconnaissance(hex) => format!("reconnaissance of {hex}"),
    }
}

fn stop_words(stop: StrikeStop) -> &'static str {
    match stop {
        StrikeStop::NotOnStrikeMission => "the group is not on the naval strike mission",
        StrikeStop::OnBaseStrike => {
            "the group is on the base strike mission, which reads no contact"
        }
        StrikeStop::NoReadyAircraft => "the group has no aircraft ready",
        StrikeStop::NoPosition => "the group has no position",
        StrikeStop::Dark => "the pulse is dark where the group is",
        StrikeStop::NoWeapon => "the group's aircraft carry no weapon",
        StrikeStop::NothingAbleToFly => "the group has no aircraft able to fly",
        StrikeStop::AllHeldBack => "the share the group holds back leaves it no aircraft to send",
    }
}

fn fate_words(fate: ContactFate) -> String {
    match fate {
        ContactFate::Chosen { aircraft } => format!("chosen: {aircraft} aircraft sent at it"),
        ContactFate::TooOld {
            age_pulses,
            limit_pulses,
        } => format!(
            "too old: {age_pulses} pulse{} against the limit of {limit_pulses}",
            plural(age_pulses)
        ),
        ContactFate::OutOfReach {
            distance_nm,
            reach_nm,
        } => format!("out of reach: {distance_nm} nm away, the reach is {reach_nm} nm"),
        ContactFate::KindNotOnList => "a kind that is not on the group's target list".to_string(),
        ContactFate::CameFirst { chosen } => format!(
            "qualified, but contact {chosen} came first (the kind first on the list, then the nearest, then the lowest number)"
        ),
        ContactFate::Served { need, sent_before } => match need {
            Some(need) => format!(
                "qualified, but served: it needs {need} aircraft and the groups before this one sent {sent_before} at it"
            ),
            None => format!("qualified, but served ({sent_before} sent at it)"),
        },
        ContactFate::GroupStopped(stop) => {
            format!("qualified, but nothing can launch: {}", stop_words(stop))
        }
    }
}

fn plural(n: u32) -> &'static str {
    if n == 1 {
        ""
    } else {
        "s"
    }
}

/// Whether the review's launch and the log's agree: both none, or the same
/// contact (by its number in the command's picture), hex and aircraft.
fn agree(world: &World, review: &StrikeReview, log: &[Launched]) -> bool {
    let key = world.umpire_key();
    match (review.launches(), log) {
        (None, []) => true,
        (Some(strike), [launched]) => {
            strike.choice.contact.umpire_raw(&key) == launched.contact
                && strike.choice.hex == launched.hex
                && strike.aircraft == launched.aircraft
        }
        _ => false,
    }
}

pub fn answer(
    world: &World,
    watch: &Watched,
    asked: Asked,
    header: &str,
    json: bool,
) -> Result<(), String> {
    let truth = world.umpire();
    let Some(group) = truth.air_groups.get(&asked.group) else {
        return Err(format!("no {} in this run", asked.group));
    };
    let Some((side, review)) = &watch.review else {
        if asked.pulse == Pulse::Night {
            return Err(format!(
                "no strike is decided in a night pulse: ask for the morning or the afternoon of day {}",
                asked.day
            ));
        }
        return Err(format!(
            "the run has no strike decision for {} in the {} of day {} (the day is past the game's last, or there are no air combat figures)",
            asked.group,
            asked.pulse.name(),
            asked.day
        ));
    };
    let log = launched_in_log(world, &asked);
    let agrees = agree(world, review, &log);
    let report = asked.report.map(|n| {
        let held = review
            .contacts
            .iter()
            .find(|contact| contact.report_event.umpire_raw(&world.umpire_key()) == n);
        let road = report_answers(world, n, Some(review.command), watch);
        (n, held.map(|c| c.number), road)
    });
    let strike_text = |strike: &sim_core::NavalStrike| {
        format!(
            "{} aircraft at contact {} ({}) at {}, {} nm",
            strike.aircraft,
            strike.choice.contact.umpire_raw(&world.umpire_key()),
            kind_name(strike.choice.kind),
            strike.choice.hex,
            strike.choice.distance_nm
        )
    };

    if json {
        let key = world.umpire_key();
        let value = serde_json::json!({
            "group": asked.group.to_string(),
            "side": side.name(),
            "day": asked.day,
            "pulse": asked.pulse.name(),
            "command": review.command.to_string(),
            "command_levels": review.command_levels,
            "mission": mission_name(review.mission),
            "hex": review.hex.map(|hex| hex.to_string()),
            "reach": format!("{:?}", review.reach).to_lowercase(),
            "reach_nm": review.reach_nm.to_string(),
            "age_limit_pulses": review.age_limit_pulses,
            "target_list": review.target_list.iter().map(|k| kind_name(*k)).collect::<Vec<_>>(),
            "aircraft_able_to_fly": review.able_to_fly,
            "held_share": review.held_share.to_string(),
            "aircraft_to_send": review.can_send,
            "dark": review.dark,
            "sizing": review.sizing,
            "stopped_by": review.stop.map(stop_words),
            "review_launches": review.launches().as_ref().map(strike_text),
            "log_launches": log.iter().map(|l| format!(
                "#{}: {} aircraft at contact {} at {}", l.event, l.aircraft, l.contact, l.hex
            )).collect::<Vec<_>>(),
            "agree": agrees,
            "contacts": review.contacts.iter().map(|c| serde_json::json!({
                "number": c.number,
                "hex": c.hex.to_string(),
                "kind": kind_name(c.kind),
                "ships": c.ships,
                "age_pulses": c.age_pulses,
                "report_event": c.report_event.umpire_raw(&key),
                "own": c.own,
                "fate": fate_words(c.fate),
                "chosen": matches!(c.fate, ContactFate::Chosen { .. }),
            })).collect::<Vec<_>>(),
            "report": report.as_ref().map(|(n, held, road)| serde_json::json!({
                "event": n,
                "held_as_contact": held,
                "answer": match road {
                    Ok(answers) => report_road::answers_json(*n, answers),
                    Err(message) => serde_json::json!({ "error": message }),
                },
            })),
        });
        println!("{value}");
        return Ok(());
    }

    println!("{header}");
    println!(
        "{} \"{}\" ({}), belongs to {}{}",
        asked.group,
        group.name,
        side.name(),
        review.command,
        if review.command_levels {
            ""
        } else {
            " (command levels are off: the side's own picture)"
        }
    );
    println!(
        "mission {}; {}; reach {} = {} nm; contact age limit {} pulse{}; targets, first first: {}",
        mission_name(review.mission),
        review
            .hex
            .map_or("no position".to_string(), |hex| format!("at {hex}")),
        format!("{:?}", review.reach).to_lowercase(),
        review.reach_nm,
        review.age_limit_pulses,
        plural(review.age_limit_pulses),
        review
            .target_list
            .iter()
            .map(|k| kind_name(*k))
            .collect::<Vec<_>>()
            .join(", ")
    );
    println!(
        "{} aircraft able to fly, the share held back is {}, {} to send; strikes {}sized; the pulse is {}dark where the group is",
        review.able_to_fly,
        review.held_share,
        review.can_send,
        if review.sizing { "" } else { "not " },
        if review.dark { "" } else { "not " },
    );
    match review.stop {
        Some(stop) => println!("the group cannot launch: {}", stop_words(stop)),
        None => println!("no gate of the group itself stops it"),
    }
    match review.launches() {
        Some(strike) => println!("the rules gave: launch {}", strike_text(&strike)),
        None => println!("the rules gave: launch nothing"),
    }
    match log.as_slice() {
        [] => println!("the log has no launch of {} in this pulse", asked.group),
        launched => {
            for l in launched {
                println!(
                    "the log has: #{} launched {} aircraft at contact {} at {}, {} nm",
                    l.event, l.aircraft, l.contact, l.hex, l.distance_nm
                );
            }
        }
    }
    if !agrees {
        println!(
            "warning: the review and the log disagree about the launch (a bug in the review: tell the architect)"
        );
    }
    if let (Some(answer), Some(chosen)) = (&review.answer, review.chosen_contact()) {
        if answer.choice.contact.umpire_raw(&world.umpire_key()) != chosen.number {
            println!(
                "warning: the contact of the strike question (contact {}) is not the contact the review's walk took (contact {}): a bug",
                answer.choice.contact.umpire_raw(&world.umpire_key()),
                chosen.number
            );
        }
    }
    println!(
        "{} contact{} in the picture of {}:",
        review.contacts.len(),
        plural(u32::try_from(review.contacts.len()).unwrap_or(u32::MAX)),
        review.command
    );
    let key = world.umpire_key();
    for c in &review.contacts {
        let source = if !review.command_levels || !review.reports_travel {
            String::new()
        } else if c.own {
            ", its own".to_string()
        } else {
            ", told".to_string()
        };
        println!(
            "  contact {} at {}: a {}, {} ship{}, {} pulse{} old, rests on report #{}{}: {}",
            c.number,
            c.hex,
            kind_name(c.kind),
            c.ships,
            plural(c.ships),
            c.age_pulses,
            plural(c.age_pulses),
            c.report_event.umpire_raw(&key),
            source,
            fate_words(c.fate)
        );
    }
    if let Some((n, held, road)) = &report {
        match held {
            Some(number) => println!(
                "report #{n}: held by {} as contact {number} at that moment",
                review.command
            ),
            None => println!(
                "report #{n}: not held by {} as a contact at that moment",
                review.command
            ),
        }
        match road {
            Ok(answers) => {
                for line in report_road::answers_text(answers) {
                    println!("{line}");
                }
            }
            Err(message) => println!("{message}"),
        }
    }
    Ok(())
}
