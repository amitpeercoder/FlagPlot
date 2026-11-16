//! The measure of a deny intent (decision 0123, 4): over the days the intent
//! has been in force, the contacts the side held in the waters at the end of
//! each day, the strikes its groups launched from the bases given the task
//! of striking ships under it, the sinkings and hits its people claim in the
//! waters from the air, and the days on which the side held no contact in the
//! waters.
//!
//! Worked from a [`SideView`] and nothing else: the side's own events (the
//! reports that made, moved or dropped its contacts, the launches of its own
//! groups, the claims it received), its command state and its own air
//! groups. A claim is what the side's people believe, never below the truth
//! and never corrected; the measure says "claimed" and no more. Nothing is
//! stored: the same view gives the same measure after a save and a resume.
//!
//! What the view cannot give: the groups are the ones based at a base now, so
//! a group rebased since a launch counts for the base it is at.

use std::collections::{BTreeMap, BTreeSet};

use sim_core::command::IntentKind;
use sim_core::events::EventKind;
use sim_core::hex::Hex;
use sim_core::ids::{AirGroupId, BaseId, ContactId, IntentId};
use sim_core::side_events::SideEventKind;
use sim_core::state::AirGroupLocation;
use sim_core::view::SideView;

/// What a deny intent has done so far.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct DenyMeasure {
    /// The days the intent has been in force: from the day it was set to the
    /// last day resolved, or to the day before it was withdrawn, inclusive.
    pub days_in_force: u32,
    /// The distinct contacts the side held in the waters at the end of a day
    /// in force.
    pub contacts_held: u32,
    /// The strikes launched in those days by the side's groups, based now at
    /// a base that was given the task of striking ships under this intent,
    /// on the days that task stood. Not only at the waters: a base given the
    /// task strikes what its groups reach (decision 0123, 6).
    pub strikes_launched: u32,
    /// The ships the side claims to have sunk from the air in the waters in
    /// those days.
    pub claimed_sunk: u32,
    /// The hits the side claims to have scored from the air in the waters in
    /// those days.
    pub claimed_hits: u32,
    /// The days of those on whose end the side held no contact in the
    /// waters.
    pub days_met: u32,
}

/// The measure of deny intent `intent`, or `None` where it is no intent of
/// the side or not a deny intent.
pub fn deny_measure(view: &SideView<'_>, intent: IntentId) -> Option<DenyMeasure> {
    let state = view.command_state();
    let found = state.intents.get(&intent)?;
    let (centre, distance) = found.kind.waters()?;
    let last_of = |withdrawn: Option<u32>| match withdrawn {
        Some(day) => day.saturating_sub(1).min(view.day()),
        None => view.day(),
    };
    let last = last_of(found.withdrawn_day);
    let first = found.set_day;
    let in_force = |day: u32| day >= first && day <= last;
    let days_in_force = if first <= last { last - first + 1 } else { 0 };
    let in_waters = |hex: Hex| hex.distance(centre) <= distance;

    // The bases given the task of striking ships under this intent, and the
    // days each task stood.
    let mut tasks: BTreeMap<BaseId, Vec<(u32, u32)>> = BTreeMap::new();
    for child in state.intents.values() {
        if child.parent != Some(intent) {
            continue;
        }
        if let IntentKind::StrikeShips { base } = child.kind {
            let to = last_of(child.withdrawn_day).min(last);
            tasks
                .entry(base)
                .or_default()
                .push((child.set_day.max(first), to));
        }
    }
    // The side's groups now at one of those bases.
    let groups: BTreeMap<AirGroupId, BaseId> = view
        .air_groups()
        .filter_map(|group| match group.location {
            AirGroupLocation::Base(base) if tasks.contains_key(&base) => Some((group.id, base)),
            _ => None,
        })
        .collect();

    let side = view.side();
    let mut held: BTreeMap<ContactId, Hex> = BTreeMap::new();
    // The contacts held in the waters at the end of each day that had an
    // event, rebuilt in order; a day with none carries the day before.
    let mut ends: Vec<(u32, BTreeSet<ContactId>)> = Vec::new();
    let mut current_day: Option<u32> = None;
    let mut strikes_launched: u32 = 0;
    let mut claimed_sunk: u32 = 0;
    let mut claimed_hits: u32 = 0;
    let waters_of = |held: &BTreeMap<ContactId, Hex>| -> BTreeSet<ContactId> {
        held.iter()
            .filter(|(_, hex)| in_waters(**hex))
            .map(|(id, _)| *id)
            .collect()
    };
    for event in view.events() {
        if current_day.is_some_and(|day| day != event.day) {
            ends.push((current_day.unwrap_or(0), waters_of(&held)));
        }
        current_day = Some(event.day);
        let SideEventKind::Whole(kind) = &event.kind else {
            continue;
        };
        match kind {
            EventKind::ReportReceived {
                side: to,
                contact,
                hex,
                ..
            } if *to == side => {
                held.insert(*contact, *hex);
            }
            EventKind::ContactDropped {
                side: to, contact, ..
            }
            | EventKind::ContactNotThere {
                side: to, contact, ..
            } if *to == side => {
                held.remove(contact);
            }
            EventKind::StrikeLaunched { air_group, .. } if in_force(event.day) => {
                if let Some(base) = groups.get(air_group) {
                    let stood = tasks.get(base).is_some_and(|spans| {
                        spans
                            .iter()
                            .any(|(a, b)| event.day >= *a && event.day <= *b)
                    });
                    if stood {
                        strikes_launched = strikes_launched.saturating_add(1);
                    }
                }
            }
            EventKind::ClaimReceived {
                side: to,
                hex,
                claimed_sunk: sunk,
                claimed_hits: hits,
                from_air: true,
                ..
            } if *to == side && in_force(event.day) && in_waters(*hex) => {
                claimed_sunk = claimed_sunk.saturating_add(*sunk);
                claimed_hits = claimed_hits.saturating_add(*hits);
            }
            _ => {}
        }
    }
    if let Some(day) = current_day {
        ends.push((day, waters_of(&held)));
    }

    // What was held in the waters at the end of a day: the last snapshot of
    // that day or of a day before it.
    let held_at_end = |day: u32| -> BTreeSet<ContactId> {
        ends.iter()
            .rev()
            .find(|(at, _)| *at <= day)
            .map(|(_, set)| set.clone())
            .unwrap_or_default()
    };
    let mut seen: BTreeSet<ContactId> = BTreeSet::new();
    let mut days_met: u32 = 0;
    if first <= last {
        for day in first..=last {
            let at_end = held_at_end(day);
            if at_end.is_empty() {
                days_met += 1;
            }
            seen.extend(at_end);
        }
    }

    Some(DenyMeasure {
        days_in_force,
        contacts_held: u32::try_from(seen.len()).unwrap_or(u32::MAX),
        strikes_launched,
        claimed_sunk,
        claimed_hits,
        days_met,
    })
}
