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

use crate::belief::{BattleClaim, BeliefState, Contact};
use crate::hex::Hex;
use crate::ids::{AircraftTypeId, ShipClassId};
use crate::map::{HexData, Map};
use crate::refdata::{AircraftType, PortSize, RefData, SearchFigures, ShipClass};
use crate::state::{AirGroup, Arrival, Base, Replacement, Ship, Side, TaskForce, TruthState};

/// One side's view of the game.
///
/// `Debug` is written by hand and prints the side only. A derived one would
/// print the whole truth state the view holds, enemy included, into whatever
/// log, briefing or prompt the view was formatted for.
#[derive(Clone, Copy)]
pub struct SideView<'a> {
    side: Side,
    truth: &'a TruthState,
    belief: &'a BeliefState,
    map: &'a Map,
    ref_data: &'a RefData,
}

impl std::fmt::Debug for SideView<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SideView")
            .field("side", &self.side)
            .finish_non_exhaustive()
    }
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

    /// The side's own bases. Supply and fuel stock are visible only for the own side,
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

    /// The side's own replacement streams (decision 6 of 0038), known
    /// exactly. The enemy's are not visible.
    pub fn replacements(&self) -> impl Iterator<Item = &'a Replacement> + '_ {
        let side = self.side;
        self.truth
            .replacements
            .iter()
            .filter(move |stream| stream.side == side)
    }

    /// The arrivals due to the side's own air groups (decision 0041), known
    /// exactly. The enemy's are not visible.
    pub fn arrivals(&self) -> impl Iterator<Item = &'a Arrival> + '_ {
        let side = self.side;
        self.truth.arrivals.iter().filter(move |arrival| {
            self.truth
                .air_groups
                .get(&arrival.air_group)
                .is_some_and(|group| group.side == side)
        })
    }

    /// What the side believes about the enemy.
    pub fn contacts(&self) -> impl Iterator<Item = &'a Contact> + '_ {
        self.belief.contacts()
    }

    /// What the side believes came of its fights: the enemy ships it claims
    /// sank, in the order the claims arrived. Real damage is truth and is not
    /// shown.
    pub fn claims(&self) -> &'a [BattleClaim] {
        self.belief.claims()
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
    use crate::ids::{AirGroupId, AircraftTypeId, BaseId, ShipClassId, ShipId, TaskForceId};
    use crate::map::{HexData, Map};
    use crate::orders::{AirMission, Mission, Readiness};
    use crate::refdata::{
        AircraftRole, AircraftType, PortSize, RefData, SearchFigures, ShipClass, ShipKind,
        SurfaceCombatFigures,
    };
    use crate::resolve::PulsePhase;
    use crate::state::{
        AirGroup, AirGroupLocation, Base, Ship, ShipLocation, ShipSystems, Side, TaskForce,
        TruthState,
    };
    use crate::time::Pulse;
    use crate::world::World;

    fn group(id: u32, side: Side, base: u32) -> AirGroup {
        AirGroup {
            id: AirGroupId::new(id),
            name: format!("Group {id}"),
            side,
            location: AirGroupLocation::Base(BaseId::new(base)),
            aircraft_type: AircraftTypeId::new(1),
            ready: 4,
            damaged: 0,
            mission: AirMission::Rest,
            search: None,
            mission_order: None,
            arc_order: None,
            patrol_share: Fixed::ONE,
            share_order: None,
            fatigue: Fixed::ZERO,
            rest_above: None,
            rest_order: None,
            stood_down: false,
            pilots: None,
            strength: 4,
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
                strike_range_nm: Fixed::ZERO,
                attack_weapon: crate::refdata::AttackWeapon::None,
                attack_damage: Fixed::ZERO,
                pilot_armour: false,
                self_sealing_tanks: false,
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
                    anti_aircraft: 0,
                    aircraft_repairs_per_day: 0,
                    fuel_t: Fixed::ZERO,
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

        // Formatting a view must not print the truth it holds.
        for printed in [format!("{view:?}"), format!("{view:#?}")] {
            assert!(printed.contains("Allied"), "{printed}");
            assert!(!printed.contains("Base 2"), "{printed}");
            assert!(!printed.contains("Group 2"), "{printed}");
            assert!(!printed.contains("Japanese"), "{printed}");
        }
    }
    /// A day fight in one hex: Allied task force 1 (ships 1 and 2) against
    /// Japanese task force 2 (ship 3). Every gun hit lands on flotation, there
    /// are no torpedoes, and ships 1 and 3 are one hit from sinking, so both
    /// forces lose a ship in round 1.
    fn fought_world() -> World {
        let hex = Hex::new(0, 0);
        let mut map = Map::new();
        map.insert(hex, HexData::open_sea());
        let mut ref_data = RefData::default();
        ref_data.port_sizes.insert(
            0,
            PortSize {
                berths: 0,
                berth_rate_t_per_h: Fixed::ZERO,
                lighter_rate_t_per_h: Fixed::from_int(20),
            },
        );
        ref_data.ship_classes.insert(
            ShipClassId::new(1),
            ShipClass {
                id: ShipClassId::new(1),
                name: "Test cruiser".to_string(),
                kind: ShipKind::Cruiser,
                cruise_speed_kn: Fixed::from_int(16),
                max_speed_kn: Fixed::from_int(32),
                fuel_capacity_t: Fixed::from_int(2000),
                endurance_nm: Fixed::from_int(8000),
                cargo_capacity_t: Fixed::ZERO,
                main_guns: 8,
                gun_rounds_per_min: Fixed::from_int(3),
                gun_rounds_per_gun: 100,
                gun_damage: Fixed::from_int(20),
                torpedo_tubes: 0,
                torpedo_damage: Fixed::ZERO,
                toughness: Fixed::from_int(400),
                radar: false,
                aircraft_capacity: 0,
                anti_aircraft: 0,
            },
        );
        ref_data.search = Some(SearchFigures::from_search_toml());
        let mut figures = SurfaceCombatFigures::from_surface_combat_toml();
        figures.gun_hit_flotation = Fixed::ONE;
        ref_data.surface_combat = Some(figures);

        let mut truth = TruthState::default();
        for (force, side, ships) in [(1, Side::Allied, vec![1, 2]), (2, Side::Japanese, vec![3])] {
            let force_id = TaskForceId::new(force);
            let mut task_force = TaskForce::at(
                force_id,
                format!("Force {force}"),
                side,
                hex,
                Mission::SurfaceCombat,
            );
            // Both forces were alert by default before decision 0021.
            task_force.readiness = Readiness::Battle;
            for ship in ships {
                let id = ShipId::new(ship);
                task_force.ships.insert(id);
                truth.ships.insert(
                    id,
                    Ship {
                        id,
                        name: format!("Ship {ship}"),
                        class: ShipClassId::new(1),
                        side,
                        fuel_t: Fixed::from_int(2000),
                        supply_t: Fixed::ZERO,
                        cargo_fuel_t: Fixed::ZERO,
                        location: ShipLocation::TaskForce(force_id),
                        // Ships 1 and 3 sink to the first flotation hit.
                        flotation_damage: if ship == 2 {
                            Fixed::ZERO
                        } else {
                            Fixed::from_int(95)
                        },
                        fire_damage: Fixed::ZERO,
                        gun_rounds: Fixed::from_int(800),
                        torpedoes_ready: false,
                        systems: ShipSystems::ALL_WORKING,
                    },
                );
            }
            truth.task_forces.insert(force_id, task_force);
        }
        let mut world = World::new(1, ref_data, map, truth).expect("valid setup");
        world.run_pulse_phase(1, Pulse::Morning, PulsePhase::Movement);
        world
    }

    #[test]
    fn a_side_sees_its_own_ships_damage_and_ammunition_its_sunk_ships_and_its_claims() {
        // Day, both sight in round 1. Allied ships afloat: 1, 2. Japanese: 3.
        // Matching: ship 1 -> 3, ship 2 -> 3 (3 mod 1 targets), ship 3 -> 1.
        // Each Allied gun salvo is 8 x 3 x 5 = 120 shots at day chance 0.05
        // = 6 expected hits, all on flotation at 100 x 20 / 400 = 5 points
        // each, so 30 points: ship 3 (95) sinks, and ship 1 (95) sinks from
        // ship 3's 30 points. Ship 2 is not fired at.
        let world = fought_world();
        let allied = world.side_view(Side::Allied);
        let japanese = world.side_view(Side::Japanese);

        // Own ships only, and a sunk own ship is still shown, as sunk.
        let ids: Vec<u32> = allied.ships().map(|ship| ship.id.raw()).collect();
        assert_eq!(ids, vec![1, 2]);
        let ids: Vec<u32> = japanese.ships().map(|ship| ship.id.raw()).collect();
        assert_eq!(ids, vec![3]);
        let ship1 = allied.ships().find(|s| s.id == ShipId::new(1)).unwrap();
        assert_eq!(ship1.location, ShipLocation::Sunk);
        assert_eq!(ship1.flotation_damage, Fixed::from_int(100));
        let sunk3 = japanese.ships().next().unwrap();
        assert_eq!(sunk3.location, ShipLocation::Sunk);

        // Own damage and ammunition are visible: ship 2 fired 120 rounds of
        // its 800 and was not hit; ship 1 fired in the round it sank.
        let ship2 = allied.ships().find(|s| s.id == ShipId::new(2)).unwrap();
        assert_eq!(ship2.gun_rounds, Fixed::from_int(680));
        assert_eq!(ship2.flotation_damage, Fixed::ZERO);
        assert_eq!(ship1.gun_rounds, Fixed::from_int(680));

        // Claims: each side fired and the other side lost one ship.
        for (view, other) in [(&allied, Side::Japanese), (&japanese, Side::Allied)] {
            let claims = view.claims();
            assert_eq!(claims.len(), 1, "{other:?} side lost one ship");
            assert_eq!(claims[0].claimed_sunk, 1);
            assert_eq!(claims[0].hex, Hex::new(0, 0));
            assert_eq!(claims[0].day, 1);
        }
    }

    #[test]
    fn a_side_with_no_fight_has_no_claims() {
        let mut map = Map::new();
        map.insert(Hex::new(0, 0), HexData::open_sea());
        let mut ref_data = RefData::default();
        ref_data.port_sizes.insert(
            0,
            PortSize {
                berths: 0,
                berth_rate_t_per_h: Fixed::ZERO,
                lighter_rate_t_per_h: Fixed::from_int(20),
            },
        );
        ref_data.search = Some(SearchFigures::from_search_toml());
        ref_data.surface_combat = Some(SurfaceCombatFigures::from_surface_combat_toml());
        let world = World::new(1, ref_data, map, TruthState::default()).expect("valid setup");
        assert!(world.side_view(Side::Allied).claims().is_empty());
    }
}
