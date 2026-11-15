//! The measure of a hold intent (decision 0117, 14): how many of the days the
//! intent has been in force the base was the side's at the end of the day with
//! its airfield open, the air raids on the base, how many of them a patrol of
//! the side's fought, and the base's days of supply today.
//!
//! Worked from a [`SideView`] and nothing else: the side's own events (the
//! `BaseAtDayEnd` of a base it holds, the airfield's damage as its own events
//! give it, the raids its own events show) and its own base. Nothing is
//! stored: the same view gives the same measure after a save and a resume.
//!
//! What the view cannot give, as the side's events are projected for the side
//! that holds the base *now*: a base the side has lost shows none of its past
//! days of the base held (a side's events of a base are its own only while it
//! is its own), so the measure of a hold on a base that was lost counts the
//! days of any earlier holding as not met.

use std::collections::BTreeMap;

use sim_core::command::IntentKind;
use sim_core::events::EventKind;
use sim_core::fixed::Fixed;
use sim_core::ids::{BaseId, IntentId};
use sim_core::side_events::SideEventKind;
use sim_core::view::SideView;

/// What a hold intent has done so far, in days and raids.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct HoldMeasure {
    /// The days the intent has been in force: from the day it was set to the
    /// last day resolved, or to the day before it was withdrawn, inclusive.
    pub days_in_force: u32,
    /// The days of those on which the base was the side's at the end of the
    /// day and its airfield open.
    pub days_met: u32,
    /// The air raids on the base in the days in force: each daylight pulse in
    /// which it was bombed.
    pub raids: u32,
    /// The raids in which a patrol of the side's, from a group now based
    /// there, fired.
    pub raids_fought: u32,
    /// The base's days of supply today; `None` where there is no figure (the
    /// base is not the side's, or its planned use is nothing).
    pub days_of_supply: Option<Fixed>,
}

/// The damage of a base's airfield as the side's events give it, by day: the
/// damage after the last event of the day or of a day before it.
fn airfield_events(view: &SideView<'_>, base: BaseId) -> (BTreeMap<u32, Fixed>, Option<u32>) {
    let mut damage: BTreeMap<u32, Fixed> = BTreeMap::new();
    let mut built: Option<u32> = None;
    let mut first_before: Option<Fixed> = None;
    for event in view.events() {
        let SideEventKind::Whole(kind) = &event.kind else {
            continue;
        };
        match kind {
            EventKind::AirfieldDamaged {
                base: hit,
                damage: after,
                ..
            } if *hit == base => {
                damage.insert(event.day, *after);
            }
            EventKind::AirfieldMended {
                base: mended,
                damage: after,
                points,
                ..
            } if *mended == base => {
                if damage.is_empty() && first_before.is_none() {
                    // Before the first mending the field had what it mended
                    // away and what is left.
                    first_before = Some(*after + *points);
                }
                damage.insert(event.day, *after);
            }
            EventKind::AirfieldBuilt { base: built_at, .. } if *built_at == base => {
                built.get_or_insert(event.day);
            }
            _ => {}
        }
    }
    if let Some(before) = first_before {
        // Day 0 stands for the state before the first event.
        damage.insert(0, before);
    }
    (damage, built)
}

/// The measure of hold intent `intent`, or `None` where it is no intent of
/// the side or not a hold intent.
pub fn hold_measure(view: &SideView<'_>, intent: IntentId) -> Option<HoldMeasure> {
    let found = view.command_state().intents.get(&intent)?;
    let IntentKind::Hold { base } = found.kind else {
        return None;
    };
    let last = match found.withdrawn_day {
        Some(day) => day.saturating_sub(1).min(view.day()),
        None => view.day(),
    };
    let first = found.set_day;
    let own = view.bases().find(|b| b.id == base);
    let (damage, built) = airfield_events(view, base);
    let refused_at = view.ground_figures().map(|g| g.airfield_landing_refused_at);

    // The days on which the side's events say it held the base at day's end.
    let held: std::collections::BTreeSet<u32> = view
        .events()
        .iter()
        .filter_map(|event| match &event.kind {
            SideEventKind::Whole(EventKind::BaseAtDayEnd { base: at, .. }) if *at == base => {
                Some(event.day)
            }
            _ => None,
        })
        .collect();

    let mut days_met = 0;
    if first <= last {
        for day in first..=last {
            if !held.contains(&day) {
                continue;
            }
            // An airfield: the base has one now, and had been built by then
            // if its building is among the events.
            let has_field =
                own.is_some_and(|b| b.airfield) && built.is_none_or(|built_day| built_day <= day);
            // The damage as of the end of the day: the last event on or
            // before it; with none, the base's damage as it stands if no
            // event of the field's is known at all.
            let damage_then = damage
                .range(..=day)
                .next_back()
                .map(|(_, damage)| *damage)
                .unwrap_or_else(|| {
                    if damage.is_empty() {
                        own.map_or(Fixed::ZERO, |b| b.airfield_damage)
                    } else {
                        Fixed::ZERO
                    }
                });
            let open = has_field && !refused_at.is_some_and(|level| damage_then >= level);
            if open {
                days_met += 1;
            }
        }
    }

    let days_in_force = if first <= last { last - first + 1 } else { 0 };
    let raids_seen: Vec<_> = view
        .air_raids_on(base)
        .into_iter()
        .filter(|raid| raid.day >= first && raid.day <= last)
        .collect();
    Some(HoldMeasure {
        days_in_force,
        days_met,
        raids: u32::try_from(raids_seen.len()).unwrap_or(u32::MAX),
        raids_fought: u32::try_from(raids_seen.iter().filter(|raid| raid.fought).count())
            .unwrap_or(u32::MAX),
        days_of_supply: own
            .and_then(|b| view.days_of_supply(b.hex))
            .and_then(|figure| figure.days),
    })
}
