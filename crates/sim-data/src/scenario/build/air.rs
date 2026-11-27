use crate::error::DataError;
use sim_core::fixed::Fixed;
use sim_core::ids::{AirGroupId, AircraftTypeId, BaseId, ShipId};
use sim_core::orders::{AirMission, Reach};
use sim_core::refdata::{RefData, ShipKind};
use sim_core::state::{AirGroup, AirGroupLocation, Arrival, Replacement, Side, TruthState};
use sim_core::world::MADE_IDS_FLOOR;
use std::collections::BTreeMap;
use std::path::Path;

use super::super::available_day;
use super::super::file::ScenarioFile;

pub(super) fn build_air_groups(
    file: &ScenarioFile,
    path: &Path,
    truth: &mut TruthState,
    ref_data: &RefData,
) -> Result<BTreeMap<ShipId, u32>, DataError> {
    let fail = |message: String| DataError::in_file(path, message);
    // Air groups come after ships, since a group may be aboard one.
    let mut aboard: BTreeMap<ShipId, u32> = BTreeMap::new();
    for spec in &file.air_group {
        let id = AirGroupId::new(spec.id);
        if spec.id >= MADE_IDS_FLOOR {
            return Err(fail(format!(
                    "{id} is numbered {MADE_IDS_FLOOR} or above: those numbers are kept for groups made in play (decision 0086)"
                )));
        }
        let side: Side = spec.side.into();
        let location = match (spec.base, spec.ship) {
            (Some(raw), None) => {
                let base_id = BaseId::new(raw);
                let base = truth
                    .bases
                    .get(&base_id)
                    .ok_or_else(|| fail(format!("{id} is at unknown base {raw}")))?;
                // A group with no aircraft and no pilots may wait at a base that
                // is not its side's or has no airfield (decision 0101).
                let empty = spec.ready == 0
                    && spec.damaged.unwrap_or(0) == 0
                    && spec.pilots.unwrap_or(0) == 0;
                if base.side != side && !empty {
                    return Err(fail(format!(
                        "{id} is {} but its base {base_id} is {}",
                        side.name(),
                        base.side.name()
                    )));
                }
                if !base.airfield && !empty {
                    return Err(fail(format!(
                        "{id} is at {base_id}, which has no airfield (decision 6 of 0099)"
                    )));
                }
                AirGroupLocation::Base(base_id)
            }
            (None, Some(raw)) => {
                let ship_id = ShipId::new(raw);
                let ship = truth
                    .ships
                    .get(&ship_id)
                    .ok_or_else(|| fail(format!("{id} is aboard unknown ship {raw}")))?;
                if ship.side != side {
                    return Err(fail(format!(
                        "{id} is {} but its ship {ship_id} is {}",
                        side.name(),
                        ship.side.name()
                    )));
                }
                let capacity = ref_data.ship_classes[&ship.class].aircraft_capacity;
                let total = aboard.entry(ship_id).or_insert(0);
                *total = total
                    .saturating_add(spec.ready)
                    .saturating_add(spec.damaged.unwrap_or(0));
                if *total > capacity {
                    return Err(fail(format!(
                        "{id} puts {} aircraft aboard {ship_id}, whose class carries {capacity}",
                        *total
                    )));
                }
                AirGroupLocation::Ship(ship_id)
            }
            _ => return Err(fail(format!("{id} needs exactly one of base and ship"))),
        };
        let aircraft_type = AircraftTypeId::new(spec.aircraft_type);
        if !ref_data.aircraft_types.contains_key(&aircraft_type) {
            return Err(fail(format!(
                "{id} has unknown aircraft type {}",
                spec.aircraft_type
            )));
        }
        let group = AirGroup {
            id,
            name: spec.name.clone(),
            side,
            location,
            aircraft_type,
            ready: spec.ready,
            damaged: spec.damaged.unwrap_or(0),
            mission: AirMission::Rest,
            search: None,
            mission_order: None,
            arc_order: None,
            patrol_share: Fixed::ONE,
            share_order: None,
            fatigue: match &spec.fatigue {
                Some(quantity) => {
                    let fatigue = quantity
                        .to_fixed()
                        .map_err(|e| fail(format!("{id} fatigue {e}")))?;
                    if fatigue < Fixed::ZERO || fatigue > Fixed::from_int(100) {
                        return Err(fail(format!(
                            "{id} has a fatigue of {fatigue}, not between 0 and 100"
                        )));
                    }
                    fatigue
                }
                None => Fixed::ZERO,
            },
            rest_above: None,
            rest_order: None,
            stood_down: false,
            pilots: spec.pilots,
            strength: spec
                .strength
                .unwrap_or_else(|| spec.ready.saturating_add(spec.damaged.unwrap_or(0))),
            reach: Reach::Normal,
            reach_order: None,
            target_priority: ShipKind::ALL.to_vec(),
            target_order: None,
            last_base_strike_day: None,
            rebased_day: None,
            strike_contact_age: match spec.strike_contact_age_pulses {
                Some(pulses) => {
                    // Decision 0114, 3: from 0 up to one less than the
                    // age at which a contact is dropped.
                    let max = ref_data.search.contact_drop_age_pulses.saturating_sub(1);
                    if pulses < 0 || pulses > i64::from(max) {
                        return Err(fail(format!(
                                "{id} has a strike_contact_age_pulses of {pulses}, not between 0 and {max}"
                            )));
                    }
                    Some(u32::try_from(pulses).expect("the age was checked"))
                }
                None => None,
            },
            strike_age_order: None,
            strike_held_share: match &spec.strike_held_share {
                Some(quantity) => {
                    let share = quantity
                        .to_fixed()
                        .map_err(|e| fail(format!("{id} strike_held_share {e}")))?;
                    if share < Fixed::ZERO || share > Fixed::ONE {
                        return Err(fail(format!(
                            "{id} has a strike_held_share of {share}, not between 0 and 1"
                        )));
                    }
                    share
                }
                None => Fixed::ZERO,
            },
            strike_share_order: None,
            strike_sizing: false,
            strike_sizing_order: None,
            search_once: false,
            search_once_order: None,
            available_from: available_day(spec.available_from, id).map_err(&fail)?,
            strike_first_waters: std::collections::BTreeSet::new(),
            waters_order: None,
            // Nothing claimed at the start of the game (decision 0180, 6).
            claims: sim_core::state::ClaimTally::default(),
        };
        if truth.air_groups.insert(id, group).is_some() {
            return Err(fail(format!("{id} appears twice")));
        }
    }
    Ok(aboard)
}

pub(super) fn build_replacements(
    file: &ScenarioFile,
    path: &Path,
    truth: &mut TruthState,
    ref_data: &RefData,
) -> Result<(), DataError> {
    let fail = |message: String| DataError::in_file(path, message);
    // Replacements come after bases and air groups' types are known.
    for (index, spec) in file.replacement.iter().enumerate() {
        let side: Side = spec.side.into();
        let base_id = BaseId::new(spec.base);
        let base = truth.bases.get(&base_id).ok_or_else(|| {
            fail(format!(
                "replacement {} is at unknown base {}",
                index + 1,
                spec.base
            ))
        })?;
        if base.side != side {
            return Err(fail(format!(
                "replacement {} is {} but its base {base_id} is {}",
                index + 1,
                side.name(),
                base.side.name()
            )));
        }
        let aircraft_type = AircraftTypeId::new(spec.aircraft_type);
        if !ref_data.aircraft_types.contains_key(&aircraft_type) {
            return Err(fail(format!(
                "replacement {} has unknown aircraft type {}",
                index + 1,
                spec.aircraft_type
            )));
        }
        truth.replacements.push(Replacement {
            side,
            base: base_id,
            aircraft_type,
            aircraft_per_day: spec.aircraft_per_day,
            pilots_per_day: spec.pilots_per_day,
        });
    }
    Ok(())
}

pub(super) fn build_arrivals(
    file: &ScenarioFile,
    path: &Path,
    truth: &mut TruthState,
    ref_data: &RefData,
    aboard: &BTreeMap<ShipId, u32>,
) -> Result<(), DataError> {
    let fail = |message: String| DataError::in_file(path, message);
    // Arrivals come after the air groups are known (decision 0041).
    for (index, spec) in file.arrival.iter().enumerate() {
        let number = index + 1;
        let group_id = AirGroupId::new(spec.air_group);
        let group = truth.air_groups.get(&group_id).ok_or_else(|| {
            fail(format!(
                "arrival {number} is for unknown air group {}",
                spec.air_group
            ))
        })?;
        if spec.day == 0 || spec.day > file.days {
            return Err(fail(format!(
                "arrival {number} is on day {}, outside days 1 to {}",
                spec.day, file.days
            )));
        }
        let pilots = spec.pilots.unwrap_or(0);
        if spec.aircraft == 0 && pilots == 0 {
            return Err(fail(format!("arrival {number} brings nothing")));
        }
        if pilots > 0 && group.pilots.is_none() {
            return Err(fail(format!(
                "arrival {number} brings pilots to {group_id}, whose pilots are not tracked"
            )));
        }
        // An arrival to a group aboard a ship must not overfill the ship:
        // the aircraft the ship's groups start with plus every arrival to
        // them by this day. Losses in play are ignored, so the check is
        // conservative (it mirrors `ShipOverAircraftCapacity`).
        if let AirGroupLocation::Ship(ship_id) = group.location {
            let mut total = aboard.get(&ship_id).copied().unwrap_or(0);
            for other in &file.arrival {
                let other_on_ship = truth
                    .air_groups
                    .get(&AirGroupId::new(other.air_group))
                    .is_some_and(|g| g.location == AirGroupLocation::Ship(ship_id));
                if other_on_ship && other.day <= spec.day {
                    total = total.saturating_add(other.aircraft);
                }
            }
            let capacity = ref_data.ship_classes[&truth.ships[&ship_id].class].aircraft_capacity;
            if total > capacity {
                return Err(fail(format!(
                        "arrival {number} would put {total} aircraft aboard {ship_id}, whose class carries {capacity}"
                    )));
            }
        }
        truth.arrivals.push(Arrival {
            day: spec.day,
            air_group: group_id,
            aircraft: spec.aircraft,
            pilots,
            waiting: false,
        });
    }
    Ok(())
}
