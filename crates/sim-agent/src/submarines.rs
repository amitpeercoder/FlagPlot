//! The briefing's part for submarines (decision 0218): the side's own boats,
//! its reports of enemy submarines, the waters it suspects they work in and
//! (decision 0223, stage 0) the submarine doctrine its scenario states for
//! it. Left out where the side has none of the four, so a game with no boat,
//! no waters and no doctrine is briefed as it was. A boat is known exactly to her own side
//! and an enemy boat is never shown: only a report of one, which may be out
//! of place, and a belief the side was told.

use serde::Serialize;
use sim_core::command::{SubmarineDoctrine, SubmarinePattern};
use sim_core::state::{Submarine, SubmarineCondition};
use sim_core::view::SideView;

use crate::common::{Ctx, Handle, Place};

/// What the side has of submarines (decision 0218).
#[derive(Clone, PartialEq, Eq, Debug, Serialize)]
pub struct SubmarinesPart {
    /// The side's own boats, in number order, those held apart until their
    /// day and those sunk or gone home among them. Left out when none.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub boats: Vec<SubmarineLine>,
    /// The side's reports of enemy submarines, in hex order. Left out when
    /// none.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub reports: Vec<SubmarineReportLine>,
    /// The waters where the side suspects enemy submarines work, in the
    /// order it was told. Left out when none.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub suspected_waters: Vec<SuspectedWatersLine>,
    /// The side's own submarine doctrine, as its scenario states it
    /// (decision 0223, stage 0). Left out where the scenario states neither
    /// a pattern nor an expectation. Never the enemy's.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub doctrine: Option<DoctrineLine>,
}

/// The side's own submarine doctrine (decision 0223, stage 0).
#[derive(Clone, PartialEq, Eq, Debug, Serialize)]
pub struct DoctrineLine {
    /// `off_bases` or `lines`. Left out where the scenario states none.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub submarine_pattern: Option<&'static str>,
    /// Whether the side expects enemy boats off its own bases.
    pub expects_boats_off_own_bases: bool,
    /// In words.
    pub words: String,
}

/// One of the side's own boats.
#[derive(Clone, PartialEq, Eq, Debug, Serialize)]
pub struct SubmarineLine {
    pub submarine: Handle,
    /// The name of her class.
    pub class: String,
    /// Where she is. Left out for a boat held apart, sunk or gone home.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub place: Option<Place>,
    /// `on station`, `on passage`, `going home`, `held apart`, `sunk` or
    /// `home`.
    pub state: &'static str,
    /// The station she keeps.
    pub station: Place,
    /// Where she is bound, on a passage to her station or home. Left out
    /// otherwise.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bound_for: Option<Place>,
    pub salvoes_left: u32,
    /// The last pulse she is held down, if she still is today.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub held_down_until: Option<HeldDownUntil>,
    pub damaged: bool,
    /// The day she comes onto the map, for a boat held apart. Left out
    /// otherwise.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comes_on_day: Option<u32>,
    /// Whose hands she is in (decision 0218, stage 3): the staff's under a
    /// patrol intent, the commander's by a hold or an order, or free. Left
    /// out for a boat held apart, sunk or gone home.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hands: Option<HandsLine>,
}

/// Whose hands a boat is in.
#[derive(Clone, PartialEq, Eq, Debug, Serialize)]
pub struct HandsLine {
    /// `staff`, `commander` or `free`.
    pub by: &'static str,
    /// The patrol intent whose boat she is, for `staff`. Left out otherwise.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intent: Option<u32>,
    /// In words.
    pub words: String,
}

/// Whose hands a boat that is on the map is in: the commander's where a
/// hold is on her (a hold command or an order to her), else the staff's
/// under the first patrol intent in force that may use her, else free.
fn hands_of(view: &SideView<'_>, boat: &Submarine) -> HandsLine {
    use sim_core::command::{HoldMaker, HoldTarget, IntentKind};
    let state = view.command_state();
    if let Some(hold) = state.holds.get(&HoldTarget::Submarine(boat.id)) {
        let words = match hold.by {
            HoldMaker::Command => format!("held by your hold command of day {}", hold.day),
            HoldMaker::Order(_) => format!("held by your order of day {}", hold.day),
        };
        return HandsLine {
            by: "commander",
            intent: None,
            words,
        };
    }
    let named_elsewhere = |except: sim_core::ids::IntentId| {
        state.intents_in_force().any(|other| {
            other.id != except
                && other
                    .limits
                    .forces
                    .as_ref()
                    .is_some_and(|forces| forces.submarines.contains(&boat.id))
        })
    };
    let staff = state
        .intents_in_force()
        .filter(|intent| matches!(intent.kind, IntentKind::Patrol { .. }))
        .find(|intent| match state.submarines_on_chain(intent.id) {
            Some(named) => named.contains(&boat.id),
            None => !named_elsewhere(intent.id),
        });
    match staff {
        Some(intent) => HandsLine {
            by: "staff",
            intent: Some(intent.id.raw()),
            words: format!("in the staff's hands under intent:{}", intent.id.raw()),
        },
        None => HandsLine {
            by: "free",
            intent: None,
            words: "in no one's hands: no hold on her and no patrol intent that may use her"
                .to_string(),
        },
    }
}

/// The pulse a boat is held down to.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize)]
pub struct HeldDownUntil {
    pub day: u32,
    /// `night`, `morning` or `afternoon`.
    pub pulse: &'static str,
}

/// A report of an enemy submarine: where, and how old.
#[derive(Clone, PartialEq, Eq, Debug, Serialize)]
pub struct SubmarineReportLine {
    pub place: Place,
    /// The day it was made.
    pub day: u32,
    /// The pulses since it was made.
    pub age_pulses: u32,
    /// The side's number of the event that gave it. Left out at a seat at a
    /// task force or a base, which is not shown the side's reports.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event: Option<u32>,
}

/// Waters where the side suspects enemy submarines work.
#[derive(Clone, PartialEq, Eq, Debug, Serialize)]
pub struct SuspectedWatersLine {
    /// The centre.
    pub place: Place,
    /// How many hexes round the centre.
    pub distance_hexes: u32,
    /// The belief, in words.
    pub words: String,
    /// The day the side was told.
    pub told_day: u32,
    /// The side's number of the event that told it. Left out at a seat at a
    /// task force or a base.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event: Option<u32>,
}

/// The side's estimate of the enemy's boats (decision 0223, stage D1): a
/// reading of its own reports, never the enemy's truth.
#[derive(Clone, PartialEq, Eq, Debug, Serialize)]
pub struct SubmarineEstimateLine {
    /// The areas the side's reports make, in the order they were begun.
    /// Left out when none.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub areas: Vec<EstimateAreaLine>,
    /// The side's own bases named as waters where boats may be expected, in
    /// base number order. Left out when none.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub by_habit: Vec<HabitWatersLine>,
}

/// An area of the estimate: waters told as suspected waters are.
#[derive(Clone, PartialEq, Eq, Debug, Serialize)]
pub struct EstimateAreaLine {
    /// The centre: the hex of the area's first report.
    pub place: Place,
    /// How many hexes round the centre.
    pub distance_hexes: u32,
    /// The hexes of the reports, in hex order.
    pub reports: Vec<Place>,
    /// How many boats, about: a whole number of 1 or more.
    pub count: u32,
    /// Always `reported`.
    pub basis: &'static str,
    /// The days of the reports, each once, in order.
    pub days: Vec<u32>,
    /// In words.
    pub words: String,
}

/// One of the side's own bases named by habit.
#[derive(Clone, PartialEq, Eq, Debug, Serialize)]
pub struct HabitWatersLine {
    pub base: Handle,
    pub place: Place,
    /// Always `by habit`.
    pub basis: &'static str,
    /// In words.
    pub words: String,
}

/// The estimate the asking seat's picture makes, its areas whose centre lies
/// in `area` and its bases whose hex lies in it. `None` where it holds
/// nothing.
pub(crate) fn estimate_line(
    ctx: &Ctx<'_>,
    view: &SideView<'_>,
    within: &dyn Fn(sim_core::hex::Hex) -> bool,
) -> Option<SubmarineEstimateLine> {
    let picture = crate::common::area_picture(view);
    let bases: Vec<(sim_core::ids::BaseId, sim_core::hex::Hex)> =
        view.bases().map(|b| (b.id, b.hex)).collect();
    let estimate = sim_core::submarine_estimate::submarine_estimate(
        &picture,
        &bases,
        view.submarine_doctrine(),
    );
    let areas: Vec<EstimateAreaLine> = estimate
        .areas
        .iter()
        .filter(|area| within(area.centre))
        .map(|area| EstimateAreaLine {
            place: ctx.place(area.centre),
            distance_hexes: area.distance_hexes,
            reports: area.reports.iter().map(|hex| ctx.place(*hex)).collect(),
            count: area.count,
            basis: "reported",
            days: area.days.clone(),
            words: format!("Enemy submarines reported here, about {}.", area.count),
        })
        .collect();
    let by_habit: Vec<HabitWatersLine> = estimate
        .by_habit
        .iter()
        .filter(|waters| within(waters.hex))
        .map(|waters| {
            let base = ctx.base(waters.base);
            let words = match &base.name {
                Some(name) => format!("Enemy submarines may be expected off {name}."),
                None => "Enemy submarines may be expected off this base.".to_string(),
            };
            HabitWatersLine {
                base,
                place: ctx.place(waters.hex),
                basis: "by habit",
                words,
            }
        })
        .collect();
    if areas.is_empty() && by_habit.is_empty() {
        return None;
    }
    Some(SubmarineEstimateLine { areas, by_habit })
}

/// The part, or `None` when the side has no boat, no report of a submarine,
/// no suspected waters and no doctrine. `today` is the day about to be played. The
/// context must have its events set.
pub(crate) fn submarines_part(
    ctx: &Ctx<'_>,
    view: &SideView<'_>,
    today: u32,
) -> Option<SubmarinesPart> {
    let picture = crate::common::area_picture(view);
    let boats: Vec<SubmarineLine> = ctx
        .submarines
        .values()
        .map(|boat| boat_line(ctx, view, boat, today))
        .collect();
    let reports: Vec<SubmarineReportLine> = picture
        .submarine_reports()
        .map(|report| SubmarineReportLine {
            place: ctx.place(report.hex),
            day: report.day,
            age_pulses: report.age_pulses,
            event: ctx.number_if_seen(report.received_event),
        })
        .collect();
    let suspected_waters: Vec<SuspectedWatersLine> = picture
        .suspected_waters()
        .iter()
        .map(|waters| SuspectedWatersLine {
            place: ctx.place(waters.hex),
            distance_hexes: waters.distance_hexes,
            words: waters.words.clone(),
            told_day: waters.day,
            event: ctx.number_if_seen(waters.received_event),
        })
        .collect();
    let doctrine = doctrine_line(view.submarine_doctrine());
    if boats.is_empty() && reports.is_empty() && suspected_waters.is_empty() && doctrine.is_none() {
        return None;
    }
    Some(SubmarinesPart {
        boats,
        reports,
        suspected_waters,
        doctrine,
    })
}

/// The side's own doctrine in the briefing: `Some` exactly when it is not
/// the default.
fn doctrine_line(doctrine: SubmarineDoctrine) -> Option<DoctrineLine> {
    if doctrine == SubmarineDoctrine::default() {
        return None;
    }
    let mut words = match doctrine.pattern {
        Some(SubmarinePattern::Lines) => {
            "Your staff places the boats of a patrol in lines across the approach of the enemy."
        }
        Some(SubmarinePattern::OffBases) => {
            "Your staff places the boats of a patrol off the enemy's bases."
        }
        None => "Your staff spreads the boats of a patrol evenly over the waters.",
    }
    .to_string();
    if doctrine.expects_boats_off_own_bases {
        words.push_str(" Boats of the enemy's may be expected off your own bases.");
    }
    Some(DoctrineLine {
        submarine_pattern: doctrine.pattern.map(SubmarinePattern::name),
        expects_boats_off_own_bases: doctrine.expects_boats_off_own_bases,
        words,
    })
}

/// One of the side's own boats, as the briefing, `list_forces` and
/// `get_entity` show her. `today` is the day about to be played.
pub(crate) fn boat_line(
    ctx: &Ctx<'_>,
    view: &SideView<'_>,
    boat: &Submarine,
    today: u32,
) -> SubmarineLine {
    // She comes onto the map first thing on her day, when the day is
    // opened: until then, on the day about to be played, she is held apart.
    let opened = view.day_in_hand().is_some();
    let held_apart = boat
        .enters_day
        .is_some_and(|enters| enters > today || (enters == today && !opened));
    let going_home = boat.passage.is_some_and(|passage| passage.home);
    let state = match boat.condition {
        SubmarineCondition::Sunk => "sunk",
        SubmarineCondition::Home => "home",
        _ if held_apart => "held apart",
        _ if going_home => "going home",
        _ if boat.passage.is_some_and(|passage| !passage.arrived) => "on passage",
        _ => "on station",
    };
    let on_map = !held_apart
        && !matches!(
            boat.condition,
            SubmarineCondition::Sunk | SubmarineCondition::Home
        );
    SubmarineLine {
        submarine: ctx.submarine(boat.id),
        class: view
            .submarine_class(boat.class)
            .map_or_else(|| boat.class.to_string(), |class| class.name.clone()),
        place: on_map.then(|| ctx.place(boat.hex)),
        state,
        station: ctx.place(boat.station),
        bound_for: boat
            .passage
            .filter(|passage| on_map && !passage.arrived)
            .map(|passage| ctx.place(passage.to)),
        salvoes_left: boat.salvoes,
        held_down_until: boat
            .held_down_until
            .filter(|(day, _)| on_map && *day >= today)
            .map(|(day, pulse)| HeldDownUntil {
                day,
                pulse: pulse.name(),
            }),
        damaged: boat.condition == SubmarineCondition::Damaged,
        comes_on_day: boat.enters_day.filter(|_| held_apart),
        hands: on_map.then(|| hands_of(view, boat)),
    }
}
