//! Rebasing an air group by order (decision 0091).
//!
//! An order names a group and a base of its side. It is checked and applied
//! at intake, at the start of the day: the group's place becomes the new
//! base, its ready aircraft and all its pilots go with it, and its damaged
//! aircraft, which cannot fly, are written off at the base it left. Nothing
//! is rolled. The group flies nothing on the day it moves
//! ([`AirGroup::rebased_day`], read by `World::aircraft_able_to_fly`); its
//! mission, target list, rest threshold, stood-down state and fatigue are
//! left as they were.
//!
//! Reads: the air group's side, place, aircraft and type; the type's search
//! range and strike radius; the base's side and hex; the day. Writes: the air
//! group's place, its damaged aircraft and the day it moved; the events
//! `AircraftWrittenOff` and `AirGroupRebased`. Draws no random number.

use crate::events::{Breakdown, EventKind, Factor};
use crate::fixed::Fixed;
use crate::ids::BaseId;
use crate::orders::RejectReason;
use crate::state::{AirGroup, AirGroupLocation, ShipLocation, Side};
use crate::world::World;

impl World {
    /// The checks of a rebase order to `group` from `side` for `day`, after
    /// the checks every air group order has (the side's own, not lost),
    /// in the order of decision 0091, 2: away at the yard, aboard a ship,
    /// already moved that day, the base not the side's own, already at the
    /// base, beyond the transfer range.
    pub(crate) fn check_rebase(
        &self,
        group: &AirGroup,
        base: BaseId,
        side: Side,
        day: u32,
    ) -> Result<(), RejectReason> {
        let from = match group.location {
            AirGroupLocation::Ship(ship) => {
                let at_yard = self
                    .truth
                    .ships
                    .get(&ship)
                    .is_some_and(|ship| matches!(ship.location, ShipLocation::Yard(_)));
                return Err(if at_yard {
                    RejectReason::AirGroupAwayAtYard
                } else {
                    RejectReason::AirGroupAboardShip
                });
            }
            AirGroupLocation::Base(from) => from,
            // Refused at intake before this check (decision 0090, 20).
            AirGroupLocation::Lost => return Err(RejectReason::AirGroupLostWithBase),
        };
        if group.rebased_day == Some(day) {
            return Err(RejectReason::AlreadyMovedToday);
        }
        let to = match self.truth.bases.get(&base) {
            Some(to) if to.side == side => to,
            _ => return Err(RejectReason::NotYourBase),
        };
        if from == base {
            return Err(RejectReason::AlreadyAtBase);
        }
        let distance_nm = self.rebase_distance_nm(from, to.hex);
        let range_nm = self.ref_data.transfer_range_nm(group.aircraft_type);
        if distance_nm > range_nm {
            return Err(RejectReason::BeyondTransferRange {
                distance_nm,
                range_nm,
            });
        }
        Ok(())
    }

    /// The distance of a move from base `from` to the hex `to`: the air
    /// distance between the two hexes (decision 0040), the one function
    /// every rule that says how far an aircraft flies uses.
    fn rebase_distance_nm(&self, from: BaseId, to: crate::hex::Hex) -> Fixed {
        let from_hex = self.truth.bases[&from].hex;
        Fixed::from_int(i64::from(from_hex.air_distance_nm(to)))
    }

    /// Applies an accepted rebase order on `day` (decision 0091, 4 to 6):
    /// writes off the group's damaged aircraft at the base it leaves, moves
    /// it, and marks the day it moved. Returns the events to record after the
    /// acceptance, in order: `AircraftWrittenOff` if it had damaged aircraft,
    /// then `AirGroupRebased`. The caller records them, each citing the
    /// order.
    pub(crate) fn rebase_air_group(
        &mut self,
        group_id: crate::ids::AirGroupId,
        base: BaseId,
        day: u32,
    ) -> Vec<EventKind> {
        let to_hex = self.truth.bases[&base].hex;
        let group = &self.truth.air_groups[&group_id];
        let AirGroupLocation::Base(from) = group.location else {
            unreachable!("the order was checked: the group is at a base");
        };
        let distance_nm = self.rebase_distance_nm(from, to_hex);
        let transfer_range_nm = self.ref_data.transfer_range_nm(group.aircraft_type);
        let group = self
            .truth
            .air_groups
            .get_mut(&group_id)
            .expect("the order was checked against this air group");
        let mut events = Vec::new();
        if group.damaged > 0 {
            events.push(EventKind::AircraftWrittenOff {
                air_group: group_id,
                base: from,
                aircraft: group.damaged,
            });
            group.damaged = 0;
        }
        group.location = AirGroupLocation::Base(base);
        group.rebased_day = Some(day);
        events.push(EventKind::AirGroupRebased {
            air_group: group_id,
            from,
            to: base,
            distance_nm,
            transfer_range_nm,
            ready: group.ready,
            pilots: group.pilots,
        });
        events
    }

    /// The breakdown of an `AirGroupRebased` event when breakdowns are on:
    /// the type's search range and strike radius, the transfer range made
    /// from them, and the distance flown. `None` for any other event.
    pub(crate) fn rebase_breakdown(&self, event: &EventKind) -> Option<Breakdown> {
        let EventKind::AirGroupRebased {
            air_group,
            distance_nm,
            transfer_range_nm,
            ..
        } = event
        else {
            return None;
        };
        if !self.config.record_breakdowns {
            return None;
        }
        let aircraft_type = self.truth.air_groups[air_group].aircraft_type;
        let figures = &self.ref_data.aircraft_types[&aircraft_type];
        Some(Breakdown {
            factors: vec![
                Factor {
                    name: "search_range_nm",
                    value: figures.search_range_nm,
                },
                Factor {
                    name: "strike_range_nm",
                    value: figures.strike_range_nm,
                },
                Factor {
                    name: "transfer_range_nm",
                    value: *transfer_range_nm,
                },
                Factor {
                    name: "distance_nm",
                    value: *distance_nm,
                },
            ],
            result: *distance_nm,
        })
    }
}
