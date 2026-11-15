//! Days of supply: a side's stock in a hex over its planned use there for a
//! day (decision 0107, 31 to 36, with rulings 1 to 3 of its hand
//! calculations). A function of the side's view: it reads the side's own
//! bases, men, forces and air groups in the hex, its own forces bound there
//! with men aboard, and the figures. Nothing of the enemy. Writes nothing and
//! draws no random number.
//!
//! Each part of the planned use is worked by the engine's own rule, never by
//! a second formula: a unit's charge by [`crate::ground::daily_charge_t`],
//! the day a force is due by the passage forecast the measuring tool gives
//! ([`SideView::forecast_passage`]), the aircraft able to fly by
//! [`crate::airstrike::able_to_fly`], a sortie's cost by
//! [`crate::refdata::AirSupplyFigures::sortie_cost`], and a base's repairs
//! by the repair step's own count (its figure and the repairs its work on the
//! aircraft the day before gives).
//!
//! It is right when read where a staff is asked: at the start of a day,
//! after forces back from the yard are home and before the day's orders are
//! taken in. Read between days it gives the same, since nothing between the end of
//! a day and that step changes a stock or a use.

use crate::fixed::Fixed;
use crate::hex::Hex;

use std::collections::BTreeMap;

use crate::airstrike::{able_to_fly, fighter_split};
use crate::ids::LandUnitId;
use crate::movement::{HaltGround, PassageEnd};
use crate::orders::{AirMission, CargoAction, Job};
use crate::state::{AirGroupLocation, Base, TaskForce, TaskForceStatus};

use super::SideView;

/// The parts of a side's planned use of supply in a hex for one day, each in
/// tons (decision 0107, 32).
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct PlannedUse {
    /// The men's supply for the side's men ashore in the hex, each unit at
    /// its weight.
    pub men_ashore_t: Fixed,
    /// The men's supply for men aboard the side's own forces that are
    /// ordered to land in the hex and are due that day, or were due earlier
    /// and have not landed (ruling 1).
    pub men_due_t: Fixed,
    /// The daily upkeep of the side's aircraft at its bases there, ready
    /// and damaged.
    pub aircraft_upkeep_t: Fixed,
    /// The supply of the repairs its bases there would make that night: for
    /// each, the smaller of its repairs a day and its damaged aircraft, at
    /// the supply of a repair.
    pub repairs_t: Fixed,
    /// The sorties of its groups there on search or patrol, at each of the
    /// day's two daylight pulses, with the aircraft able to fly (ruling 2).
    /// A strike mission counts nothing (decision 33).
    pub sorties_t: Fixed,
}

impl PlannedUse {
    pub fn total_t(&self) -> Fixed {
        self.men_ashore_t
            + self.men_due_t
            + self.aircraft_upkeep_t
            + self.repairs_t
            + self.sorties_t
    }
}

/// A side's days of supply in a hex where it holds a base.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct DaysOfSupply {
    pub hex: Hex,
    /// The supply of all the side's bases in the hex.
    pub stock_t: Fixed,
    pub planned_use: PlannedUse,
    /// The stock over the planned use, cut at six decimals. `None` where
    /// the planned use is 0: no planned use, no figure (decision 36).
    pub days: Option<Fixed>,
}

impl SideView<'_> {
    /// The side's days of supply in `hex`, with its parts. `None` if the
    /// side holds no base there (decision 36).
    ///
    /// Read where a staff is asked: at the start of day [`SideView::day`] + 1,
    /// before its night pulse and before the day's orders are taken in, so
    /// the stock is as the day before left it. It reads only the side's own
    /// bases, land units, task forces, ships and air groups, the map and the
    /// figures; the day's two daylight pulses and the coming night are the
    /// ones it plans for.
    pub fn days_of_supply(&self, hex: Hex) -> Option<DaysOfSupply> {
        let bases: Vec<&Base> = self.bases().filter(|base| base.hex == hex).collect();
        if bases.is_empty() {
            return None;
        }
        let stock_t = bases
            .iter()
            .fold(Fixed::ZERO, |sum, base| sum + base.supply_t);
        let mut planned_use = PlannedUse {
            men_ashore_t: self.men_ashore_use(hex),
            men_due_t: self.men_due_use(hex),
            ..PlannedUse::default()
        };
        for base in &bases {
            self.add_air_use(base, &mut planned_use);
        }
        let total = planned_use.total_t();
        let days = (!total.is_zero()).then(|| stock_t / total);
        Some(DaysOfSupply {
            hex,
            stock_t,
            planned_use,
            days,
        })
    }

    /// The night's feeding of the side's men ashore in `hex`: each unit's
    /// charge by the engine's own function.
    fn men_ashore_use(&self, hex: Hex) -> Fixed {
        self.land_units()
            .filter(|unit| unit.hex == Some(hex) && unit.men_ashore > 0)
            .filter_map(|unit| self.daily_charge_t(unit.id))
            .fold(Fixed::ZERO, |sum, charge| sum + charge)
    }

    /// The charge of the men aboard the side's own forces that are ordered
    /// to land in `hex` and are due by the day being planned (ruling 1): a
    /// force with an unload order that is stopped in the hex with men still
    /// aboard, or whose route ends there and whose passage, forecast as the
    /// measuring tool forecasts it, ends on or before the day. A force
    /// counts from the day it is due until its men are ashore. It is the
    /// sum of the charges of [`SideView::men_due_by_day`] due by today.
    fn men_due_use(&self, hex: Hex) -> Fixed {
        let today = self.truth.day + 1;
        self.men_due_by_day(hex)
            .into_iter()
            .filter(|(day, _)| *day <= today)
            .fold(Fixed::ZERO, |sum, (_, charge)| sum + charge)
    }

    /// For each of the side's own forces that has an unload order, is not
    /// at the yard and carries men for `hex`, the day its men are due there
    /// and the daily charge of those men, in force order (decision 0107, "As
    /// built"). A force stopped in the hex with its unload able to work and
    /// men aboard is due today (the day being planned, [`SideView::day`] +
    /// 1); one whose route ends in the hex is due on the day its passage,
    /// forecast as the measuring tool forecasts it, arrives. A charge is the
    /// sum over the land units aboard the force, each at the engine's own
    /// function of its men aboard. A day may be later than today: such a
    /// force is on its way and not yet counted in the planned use. The
    /// planned use's men due is the sum of the charges due today or earlier.
    pub fn men_due_by_day(&self, hex: Hex) -> Vec<(u32, Fixed)> {
        let Some(figures) = self.ref_data.ground.as_ref() else {
            return Vec::new();
        };
        let ground = HaltGround {
            ref_data: self.ref_data,
            map: self.map,
            ships: &self.truth.ships,
            bases: &self.truth.bases,
            land_units: &self.truth.land_units,
        };
        let mut due = Vec::new();
        for force in self.task_forces() {
            let unloads = force
                .cargo_order
                .is_some_and(|cargo| cargo.action == CargoAction::Unload);
            if !unloads || force.yard_trip.is_some() {
                continue;
            }
            let Some(day) = self.due_day(&ground, force, hex) else {
                continue;
            };
            let mut aboard: BTreeMap<LandUnitId, u32> = BTreeMap::new();
            for ship in &force.ships {
                if let Some(troops) = self.truth.ships.get(ship).and_then(|ship| ship.troops) {
                    *aboard.entry(troops.unit).or_insert(0) += troops.men;
                }
            }
            let charges: Vec<Fixed> = aboard
                .into_iter()
                .filter_map(|(unit, men)| {
                    let unit = self.truth.land_units.get(&unit)?;
                    Some(crate::ground::daily_charge_t(figures, &unit.skills, men))
                })
                .collect();
            if charges.is_empty() {
                continue;
            }
            due.push((day, charges.into_iter().fold(Fixed::ZERO, |sum, c| sum + c)));
        }
        due
    }

    /// The day `force`, with an unload order, is due in `hex`: today if it
    /// is stopped there with its unload able to work and men aboard, the
    /// day of its forecast arrival if its route ends there, else `None`.
    fn due_day(&self, ground: &HaltGround, force: &TaskForce, hex: Hex) -> Option<u32> {
        let today = self.truth.day + 1;
        let stopped = matches!(
            force.status,
            TaskForceStatus::Idle | TaskForceStatus::Arrived | TaskForceStatus::Halted
        ) && force.toward_hex.is_none();
        if stopped && force.from_hex == hex {
            // Stopped in the hex: due if the unload can work there and men
            // are still aboard (the cargo rule's own test).
            let cargo = force.cargo_order?;
            let able = ground.cargo_place(force, cargo, hex).is_some()
                && force.ships.iter().any(|ship| {
                    self.truth
                        .ships
                        .get(ship)
                        .is_some_and(|s| s.troops.is_some())
                });
            return able.then_some(today);
        }
        if force.route.waypoints.last() != Some(&hex)
            || !matches!(
                force.status,
                TaskForceStatus::Underway | TaskForceStatus::Halted
            )
        {
            return None;
        }
        match self.forecast_passage(force.id, hex, None)?.end {
            PassageEnd::Arrives { day, .. } => Some(day),
            _ => None,
        }
    }

    /// Adds to `used` what one of the side's bases pays for aircraft at it:
    /// their upkeep, the night's repairs and the sorties of its groups on
    /// search and patrol (ruling 2), by the rules of the engine's air
    /// supply and repair steps. The figures of a rule not loaded charge
    /// nothing, as the engine does.
    fn add_air_use(&self, base: &Base, used: &mut PlannedUse) {
        let Some(supply) = self.ref_data.air_supply else {
            return;
        };
        let today = self.truth.day + 1;
        let groups: Vec<&crate::state::AirGroup> = self
            .air_groups()
            .filter(|group| group.location == AirGroupLocation::Base(base.id))
            .collect();

        let aircraft: u32 = groups.iter().map(|group| group.ready + group.damaged).sum();
        used.aircraft_upkeep_t +=
            supply.daily_supply_t_per_aircraft * Fixed::from_int(i64::from(aircraft));

        // The repairs the base makes that night: its figure and the repairs
        // its work on the aircraft the day before gives, over the damaged
        // aircraft at it (`World::repair_aircraft`).
        if self.ref_data.air_upkeep.is_some() {
            let extra = match self.ref_data.ground.as_ref() {
                Some(ground) if today > 1 => {
                    crate::works::extra_repairs(base.work_of_day(today - 1, Job::Aircraft), ground)
                }
                _ => 0,
            };
            let damaged: u32 = groups.iter().map(|group| group.damaged).sum();
            let repairs = (base.aircraft_repairs_per_day + extra).min(damaged);
            used.repairs_t += supply.repair_supply_t * Fixed::from_int(i64::from(repairs));
        }

        // The sorties of the day's two daylight pulses.
        let grounded = base.supply_t.is_zero();
        let fatigue_loaded = self.ref_data.fatigue.is_some();
        for group in groups {
            let able = |group: &crate::state::AirGroup| {
                able_to_fly(
                    group,
                    grounded,
                    base.airfield_damage,
                    None,
                    self.ref_data.surface_combat.as_ref(),
                    fatigue_loaded,
                    today,
                )
            };
            let (aircraft, distance_nm) = match group.mission {
                AirMission::Search if self.ref_data.search.is_some() => match group.search {
                    Some(arc) if group.ready > 0 => {
                        (able(group), arc.range_nm * Fixed::from_int(2))
                    }
                    _ => continue,
                },
                AirMission::CombatAirPatrol
                    if self.ref_data.air_combat.is_some() && group.ready > 0 =>
                {
                    (
                        fighter_split(able(group), group.patrol_share).patrol,
                        Fixed::ZERO,
                    )
                }
                _ => continue,
            };
            used.sorties_t += supply.sortie_cost(distance_nm)
                * Fixed::from_int(i64::from(aircraft))
                * Fixed::from_int(2);
        }
    }
}
