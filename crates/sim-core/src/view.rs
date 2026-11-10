//! What one side is allowed to see.
//!
//! A [`SideView`] is the only thing a player, the staff planner or an agent
//! is ever handed. It exposes the side's own units exactly and the enemy only
//! as contacts. It has no method that returns enemy truth, so code holding a
//! `SideView` cannot read it.
//!
//! The umpire's view is [`crate::World::umpire`]. It is for tests, the
//! command-line runner and hashing, and must never be passed to anything that
//! plays the game.

use crate::belief::BeliefState;
use crate::belief::Contact;
use crate::hex::Hex;
use crate::ids::{AircraftTypeId, ShipClassId};
use crate::map::{HexData, Map};
use crate::refdata::{AircraftType, PortSize, RefData, SearchFigures, ShipClass};
use crate::state::{AirGroup, Base, Ship, Side, TaskForce, TruthState};

/// One side's view of the game.
#[derive(Clone, Copy, Debug)]
pub struct SideView<'a> {
    side: Side,
    truth: &'a TruthState,
    belief: &'a BeliefState,
    map: &'a Map,
    ref_data: &'a RefData,
}

impl<'a> SideView<'a> {
    pub(crate) fn new(
        side: Side,
        truth: &'a TruthState,
        belief: &'a BeliefState,
        map: &'a Map,
        ref_data: &'a RefData,
    ) -> SideView<'a> {
        SideView {
            side,
            truth,
            belief,
            map,
            ref_data,
        }
    }

    pub fn side(&self) -> Side {
        self.side
    }

    /// The last day resolved.
    pub fn day(&self) -> u32 {
        self.truth.day
    }

    /// The side's own ships, known exactly. Supply aboard is visible only for
    /// the own side, because the filter hides every enemy ship.
    pub fn ships(&self) -> impl Iterator<Item = &'a Ship> + '_ {
        let side = self.side;
        self.truth
            .ships
            .values()
            .filter(move |ship| ship.side == side)
    }

    /// The side's own task forces, known exactly.
    pub fn task_forces(&self) -> impl Iterator<Item = &'a TaskForce> + '_ {
        let side = self.side;
        self.truth
            .task_forces
            .values()
            .filter(move |task_force| task_force.side == side)
    }

    /// The side's own bases. Supply stock is visible only for the own side,
    /// because the filter hides every enemy base.
    pub fn bases(&self) -> impl Iterator<Item = &'a Base> + '_ {
        let side = self.side;
        self.truth
            .bases
            .values()
            .filter(move |base| base.side == side)
    }

    /// The side's own air groups, known exactly.
    pub fn air_groups(&self) -> impl Iterator<Item = &'a AirGroup> + '_ {
        let side = self.side;
        self.truth
            .air_groups
            .values()
            .filter(move |group| group.side == side)
    }

    /// What the side believes about the enemy.
    pub fn contacts(&self) -> impl Iterator<Item = &'a Contact> + '_ {
        self.belief.contacts()
    }

    /// The map is common knowledge.
    pub fn hex(&self, hex: Hex) -> Option<&'a HexData> {
        self.map.get(hex)
    }

    /// Reference data is common knowledge.
    pub fn ship_class(&self, id: ShipClassId) -> Option<&'a ShipClass> {
        self.ref_data.ship_classes.get(&id)
    }

    /// Aircraft types are reference data, so common knowledge.
    pub fn aircraft_type(&self, id: AircraftTypeId) -> Option<&'a AircraftType> {
        self.ref_data.aircraft_types.get(&id)
    }

    /// The search figures are reference data, so common knowledge.
    pub fn search_figures(&self) -> Option<&'a SearchFigures> {
        self.ref_data.search.as_ref()
    }

    /// Port handling figures are reference data, so common knowledge.
    pub fn port_size(&self, size: u8) -> Option<&'a PortSize> {
        self.ref_data.port_sizes.get(&size)
    }
}

#[cfg(test)]
mod tests {
    use crate::fixed::Fixed;
    use crate::hex::Hex;
    use crate::ids::{AirGroupId, AircraftTypeId, BaseId};
    use crate::map::{HexData, Map};
    use crate::orders::AirMission;
    use crate::refdata::{AircraftRole, AircraftType, PortSize, RefData, SearchFigures};
    use crate::state::{AirGroup, Base, Side, TruthState};
    use crate::world::World;

    fn group(id: u32, side: Side, base: u32) -> AirGroup {
        AirGroup {
            id: AirGroupId::new(id),
            name: format!("Group {id}"),
            side,
            base: BaseId::new(base),
            aircraft_type: AircraftTypeId::new(1),
            ready: 4,
            mission: AirMission::Rest,
            search: None,
            mission_order: None,
            arc_order: None,
        }
    }

    #[test]
    fn a_side_sees_only_its_own_air_groups() {
        let mut map = Map::new();
        map.insert(Hex::new(0, 0), HexData::open_sea());
        map.insert(Hex::new(5, 0), HexData::open_sea());
        let mut ref_data = RefData::default();
        ref_data.port_sizes.insert(
            0,
            PortSize {
                berths: 0,
                berth_rate_t_per_h: Fixed::ZERO,
                lighter_rate_t_per_h: Fixed::from_int(20),
            },
        );
        ref_data.aircraft_types.insert(
            AircraftTypeId::new(1),
            AircraftType {
                id: AircraftTypeId::new(1),
                name: "Test".to_string(),
                role: AircraftRole::Patrol,
                search_range_nm: Fixed::from_int(600),
                sweep_width_nm: Fixed::from_int(40),
            },
        );
        ref_data.search = Some(SearchFigures::from_search_toml());
        let mut truth = TruthState::default();
        for (id, side, hex) in [(1, Side::Allied, 0), (2, Side::Japanese, 5)] {
            truth.bases.insert(
                BaseId::new(id),
                Base {
                    id: BaseId::new(id),
                    name: format!("Base {id}"),
                    side,
                    hex: Hex::new(hex, 0),
                    supply_t: Fixed::ZERO,
                },
            );
            truth
                .air_groups
                .insert(AirGroupId::new(id), group(id, side, id));
        }
        let world = World::new(1, ref_data, map, truth).expect("valid setup");

        let allied: Vec<AirGroupId> = world
            .side_view(Side::Allied)
            .air_groups()
            .map(|g| g.id)
            .collect();
        let japanese: Vec<AirGroupId> = world
            .side_view(Side::Japanese)
            .air_groups()
            .map(|g| g.id)
            .collect();
        assert_eq!(allied, vec![AirGroupId::new(1)]);
        assert_eq!(japanese, vec![AirGroupId::new(2)]);

        let view = world.side_view(Side::Allied);
        assert!(view.aircraft_type(AircraftTypeId::new(1)).is_some());
        assert!(view.search_figures().is_some());
    }
}
