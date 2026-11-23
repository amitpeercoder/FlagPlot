//! The paper of an order to destroy a force, and the words of the three
//! notices of following (decision 0220, rules 5 to 11 and the page's check).
//!
//! Everything here is worked from a side's view: the order and what it keeps
//! of the force it follows (the command state), the commander's picture of
//! the contact, and the side's own events (the staff's notices). Nothing of
//! the enemy is read and nothing is stored: the same view gives the same
//! paper after a save and a resume.
//!
//! The words are the staff's, in the first person plural, and name the force
//! by its report (how many ships, taken for what, where and when last
//! reported). They carry no contact number as a handle: the JSON carries the
//! number for a table that needs it, the words never do. They say "order",
//! never "intent", and explain no rule.

use serde_json::{json, Value};
use sim_core::command::{report_pulse, FollowNotice, Intent, NoticeKind};
use sim_core::events::EventKind;
use sim_core::hex::Hex;
use sim_core::refdata::ShipKind;
use sim_core::side_events::SideEventKind;
use sim_core::time::Pulse;
use sim_core::view::SideView;

use crate::common::Ctx;

/// The things a call of time can be asked to stop on that are notices of
/// following (decision 0220, R11), in the order the interface lists them.
pub const NOTICE_STOPS: [&str; 3] = ["follows_other_sort", "contact_lost", "force_not_found"];

/// Which of [`NOTICE_STOPS`] a notice is, if any. A taking of the same sort,
/// and a refresh of the contact followed, stop nothing.
pub fn notice_stop(kind: NoticeKind, follow: Option<&FollowNotice>) -> Option<&'static str> {
    match (kind, follow) {
        (NoticeKind::Follows, Some(notice)) if notice.taken_from.is_some() && notice.other_sort => {
            Some("follows_other_sort")
        }
        (NoticeKind::ContactLost, _) => Some("contact_lost"),
        (NoticeKind::ForceNotFound, _) => Some("force_not_found"),
        _ => None,
    }
}

/// The plural of a kind of ship's name.
fn plural(kind: ShipKind) -> String {
    match kind.name() {
        "auxiliary" => "auxiliaries".to_string(),
        name => format!("{name}s"),
    }
}

/// "2 cruisers", "1 destroyer".
fn ships_words(count: u32, kind: ShipKind) -> String {
    if count == 1 {
        format!("1 {}", kind.name())
    } else {
        format!("{count} {}", plural(kind))
    }
}

/// "the morning of day 2".
fn pulse_words(day: u32, pulse: Pulse) -> String {
    format!("the {} of day {day}", pulse.name())
}

/// What a force is taken for, in words.
fn taken_for_words(kind: ShipKind) -> String {
    if kind == ShipKind::Carrier {
        "a carrier force".to_string()
    } else {
        format!("{} (not a carrier force)", plural(kind))
    }
}

/// A force by its report: how many, taken for what, where and when last
/// reported.
fn report_words(count: u32, kind: ShipKind, hex: Hex, day: u32, pulse: Pulse) -> String {
    format!(
        "the force of {} last reported at {hex} in {}",
        ships_words(count, kind),
        pulse_words(day, pulse)
    )
}

/// The words of a notice of following, in the staff's voice (decision 0220,
/// 8 to 10 and R3).
pub fn follow_words(kind: NoticeKind, notice: &FollowNotice) -> String {
    let report = report_words(
        notice.ship_count,
        notice.kind,
        notice.hex,
        notice.day,
        notice.pulse,
    );
    match kind {
        NoticeKind::Follows => match (notice.taken_from, notice.other_sort) {
            (Some(_), true) if notice.carrier => format!(
                "We now take the force for a carrier force: we follow {report}, where before we followed one that was not."
            ),
            (Some(_), true) => format!(
                "We no longer take the force for a carrier force: we follow {report}, where before we followed a carrier force."
            ),
            (Some(_), false) => format!("We follow another report of the force: {report}."),
            (None, _) => format!("We follow {report}."),
        },
        NoticeKind::ContactLost => format!(
            "We have lost the force: it was last reported at {} in {}, and we are searching where it was.",
            notice.hex,
            pulse_words(notice.day, notice.pulse)
        ),
        NoticeKind::ForceNotFound => format!(
            "We have not found the force last reported at {} in {}: the order is ended.",
            notice.hex,
            pulse_words(notice.day, notice.pulse)
        ),
        _ => report,
    }
}

/// The force a destroy order follows, in words, from the commander's picture
/// of the contact as it stands: for the words of a command given. Never the
/// contact's number.
pub(crate) fn contact_words(view: &SideView<'_>, contact: u32) -> String {
    let picture = crate::common::area_picture(view);
    match picture.contact_by_number(contact) {
        Some(found) => {
            let (day, pulse) = report_pulse(view.day() + 1, Pulse::Night, found.age_pulses);
            format!(
                "the force of {} last reported at {} in {}",
                ships_words(found.estimated_ship_count, found.estimated_kind),
                found.estimated_hex,
                pulse_words(day, pulse)
            )
        }
        None => "a force no longer held in the picture".to_string(),
    }
}

/// An order to destroy a force in words: the force by its report, never by
/// its number, and the distance round it (decision 0220).
pub(crate) fn order_words(
    view: &SideView<'_>,
    contact: u32,
    distance: u32,
    search_only: bool,
) -> String {
    format!(
        "{} {}, within {distance} {}",
        if search_only { "shadow" } else { "destroy" },
        contact_words(view, contact),
        if distance == 1 { "hex" } else { "hexes" }
    )
}

/// What the order in force keeps and has done, as `command_state` shows it
/// (decision 0220, "The page's check", 4 to 8): what it follows now by the
/// report, each taking in order, whether it is lost, its distance and whether
/// it is search only, and the ring. `None` for an intent that is no order to
/// destroy a force, for one withdrawn or ended, and where the order keeps
/// nothing.
pub(crate) fn following_json(view: &SideView<'_>, intent: &Intent) -> Option<Value> {
    let (_, distance, search_only) = intent.kind.followed()?;
    if !intent.in_force() {
        return None;
    }
    let state = view.command_state();
    let kept = state.followed.get(&intent.id)?;
    let ctx = Ctx::new(*view);
    let today = view.day() + 1;
    let picture = crate::common::area_picture(view);
    // A lost order keeps the last report's place and is read from it.
    let held = picture
        .contact_by_number(kept.contact)
        .filter(|_| !kept.lost);
    let events = view.events();
    // The side's own notices of this order, in the order handed in.
    let notices: Vec<(u32, NoticeKind, FollowNotice)> = events
        .iter()
        .filter_map(|event| match &event.kind {
            SideEventKind::Whole(EventKind::StaffNotice {
                side,
                kind,
                follow: Some(notice),
                ..
            }) if *side == view.side() && notice.intent == intent.id => {
                Some((event.day, *kind, *notice))
            }
            _ => None,
        })
        .collect();
    let latest = notices
        .iter()
        .rev()
        .find(|(_, kind, _)| *kind == NoticeKind::Follows)
        .map(|(_, _, notice)| *notice);
    // Where the contact is held, its report is read from the picture; where
    // it is not, what was kept and what the last notice said (R3).
    let (hex, report_day, report_pulse_, age, ships, taken_for, held_now) = match held {
        Some(found) => {
            let (day, pulse) = report_pulse(today, Pulse::Night, found.age_pulses);
            (
                found.estimated_hex,
                day,
                pulse,
                found.age_pulses,
                Some(found.estimated_ship_count),
                Some(found.estimated_kind),
                true,
            )
        }
        None => {
            let next = 3 * u64::from(today);
            let at = 3 * u64::from(kept.day)
                + match kept.pulse {
                    Pulse::Night => 0,
                    Pulse::Morning => 1,
                    Pulse::Afternoon => 2,
                };
            let age = u32::try_from(next.saturating_sub(at).saturating_sub(1)).unwrap_or(u32::MAX);
            (
                kept.hex,
                kept.day,
                kept.pulse,
                age,
                latest.map(|notice| notice.ship_count),
                latest.map(|notice| notice.kind),
                false,
            )
        }
    };
    let takings: Vec<Value> = notices
        .iter()
        .filter(|(_, kind, notice)| *kind == NoticeKind::Follows && notice.taken_from.is_some())
        .map(|(day, _, notice)| {
            let mut taking = json!({
                "day": day,
                "taken_from": notice.taken_from,
                "contact": notice.contact,
                "other_sort": notice.other_sort,
                "place": ctx.place(notice.hex),
                "ships": notice.ship_count,
                "taken_for": notice.kind.name(),
                "carrier": notice.carrier,
            });
            if notice.other_sort {
                taking["was_carrier"] = json!(!notice.carrier);
            }
            taking
        })
        .collect();
    let mut text: Vec<String> = Vec::new();
    match (ships, taken_for) {
        (Some(count), Some(kind)) => text.push(format!(
            "We follow {}.",
            report_words(count, kind, hex, report_day, report_pulse_)
        )),
        _ => text.push(format!(
            "We follow the force last reported at {hex} in {}.",
            pulse_words(report_day, report_pulse_)
        )),
    }
    if let Some(kind) = taken_for {
        text.push(format!("We take it for {}.", taken_for_words(kind)));
    }
    for (day, _, notice) in notices
        .iter()
        .filter(|(_, kind, notice)| *kind == NoticeKind::Follows && notice.taken_from.is_some())
    {
        text.push(format!(
            "On day {day}: {}",
            follow_words(NoticeKind::Follows, notice)
        ));
    }
    if kept.lost {
        text.push(format!(
            "We have lost the force: last reported at {hex} on day {report_day}; we are searching there."
        ));
    }
    if search_only {
        text.push("We search for the force and strike nothing.".to_string());
    }
    Some(json!({
        "contact": kept.contact,
        "held": held_now,
        "place": ctx.place(hex),
        "hex": [hex.q, hex.r],
        "ships": ships,
        "taken_for": taken_for.map(ShipKind::name),
        "carrier": kept.carrier,
        "age_pulses": age,
        "reported": {"day": report_day, "pulse": report_pulse_.name()},
        "takings": takings,
        "lost": kept.lost,
        "last_place": ctx.place(kept.hex),
        "last_day": kept.day,
        "distance": distance,
        "search_only": search_only,
        "ring": {
            "place": ctx.place(hex),
            "hex": [hex.q, hex.r],
            "distance": distance,
            "dashed": kept.lost,
        },
        "text": text,
    }))
}
