//! The measure of a neutralise intent (decision 0120, 8): over the days the
//! intent has been in force, the raids the side's bombers flew on the target,
//! the aircraft the side's people claim to have destroyed on the ground
//! there, the air raids on the side's own bases and the days on which none of
//! them was raided from the air.
//!
//! Worked from a [`SideView`] and nothing else: the side's own events (the
//! launches of its own groups, the claims it received) and its own bases. A
//! claim is what the side's people believe, never below the truth and never
//! corrected; the measure says "claimed" and no more. Nothing is stored: the
//! same view gives the same measure after a save and a resume.

use std::collections::BTreeSet;

use sim_core::command::IntentKind;
use sim_core::events::EventKind;
use sim_core::ids::IntentId;
use sim_core::side_events::SideEventKind;
use sim_core::view::SideView;

/// What a neutralise intent has done so far.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct NeutraliseMeasure {
    /// The days the intent has been in force: from the day it was set to the
    /// last day resolved, or to the day before it was withdrawn, inclusive.
    pub days_in_force: u32,
    /// The raids flown on the target in those days: the pulses in which one
    /// or more of the side's groups launched a strike at it. Two groups
    /// launching together are one raid.
    pub raids_flown: u32,
    /// The aircraft the side claims to have destroyed on the ground at the
    /// target's hex, from the air, in those days.
    pub claimed_on_ground: u32,
    /// The air raids on the side's own bases in those days, summed over its
    /// bases (each pulse in which a base was bombed).
    pub raids_on_own_bases: u32,
    /// The days of those on which none of the side's bases was raided from
    /// the air.
    pub days_met: u32,
}

/// The measure of neutralise intent `intent`, or `None` where it is no
/// intent of the side or not a neutralise intent.
pub fn neutralise_measure(view: &SideView<'_>, intent: IntentId) -> Option<NeutraliseMeasure> {
    let found = view.command_state().intents.get(&intent)?;
    let IntentKind::Neutralise { target } = found.kind else {
        return None;
    };
    let last = match found.withdrawn_day {
        Some(day) => day.saturating_sub(1).min(view.day()),
        None => view.day(),
    };
    let first = found.set_day;
    let in_force = |day: u32| day >= first && day <= last;
    let days_in_force = if first <= last { last - first + 1 } else { 0 };

    // The target's hex: as the enemy's public fact, or, if the side has
    // taken it since, as its own base.
    let hex = view
        .enemy_bases()
        .find(|base| base.id == target)
        .map(|base| base.hex)
        .or_else(|| view.bases().find(|base| base.id == target).map(|b| b.hex));

    let mut raids: BTreeSet<(u32, Option<sim_core::time::Pulse>)> = BTreeSet::new();
    let mut claimed_on_ground: u32 = 0;
    for event in view.events() {
        if !in_force(event.day) {
            continue;
        }
        let SideEventKind::Whole(kind) = &event.kind else {
            continue;
        };
        match kind {
            EventKind::BaseStrikeLaunched { base, .. } if *base == target => {
                raids.insert((event.day, event.pulse));
            }
            EventKind::ClaimReceived {
                hex: claimed_at,
                claimed_on_ground: on_ground,
                from_air: true,
                ..
            } if Some(*claimed_at) == hex => {
                claimed_on_ground = claimed_on_ground.saturating_add(*on_ground);
            }
            _ => {}
        }
    }

    let mut raided_days: BTreeSet<u32> = BTreeSet::new();
    let mut raids_on_own_bases: u32 = 0;
    for base in view.bases() {
        for raid in view.air_raids_on(base.id) {
            if in_force(raid.day) {
                raids_on_own_bases = raids_on_own_bases.saturating_add(1);
                raided_days.insert(raid.day);
            }
        }
    }
    let days_met = if first <= last {
        let raided = u32::try_from(raided_days.len()).unwrap_or(u32::MAX);
        days_in_force.saturating_sub(raided)
    } else {
        0
    };

    Some(NeutraliseMeasure {
        days_in_force,
        raids_flown: u32::try_from(raids.len()).unwrap_or(u32::MAX),
        claimed_on_ground,
        raids_on_own_bases,
        days_met,
    })
}
