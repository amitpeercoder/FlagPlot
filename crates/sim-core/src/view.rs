//! What one side is allowed to see.
//!
//! A [`SideView`] is the only thing a player, the staff planner or an agent
//! is ever handed. It exposes the side's own units exactly and the enemy only
//! as contacts. It has no method that returns enemy truth, so code holding a
//! `SideView` cannot read it.
//!
//! The events a side may see come from [`SideView::events`], each cut down to
//! what the side may know (decision 0047, [`crate::side_events`]). The full log
//! stays with the umpire.
//!
//! The umpire's view is [`crate::World::umpire`]. It is for tests, the
//! command-line runner and hashing, and must never be passed to anything that
//! plays the game.

use std::collections::BTreeMap;

use crate::airstrike::StrikeChoice;
use crate::belief::{BattleClaim, BeliefState, Contact};
use crate::events::EventLog;
use crate::fixed::Fixed;
use crate::hex::Hex;
use crate::ids::{
    AirGroupId, AircraftTypeId, BaseId, ContactId, EventId, LandUnitId, OrderId, ShipClassId,
    ShipId, TaskForceId,
};
use crate::map::{HexData, Map};
use crate::movement::{self, PassageForecast};
use crate::objectives::{self, KnownProgress, Objective, SideObjectives};
use crate::orders::{AirMission, Order, SpeedSetting};
use crate::refdata::{
    AirSupplyFigures, AircraftType, GroundFigures, PortSize, RefData, SearchFigures, ShipClass,
};
use crate::side_events::{self, Owners, SideEvent};
use crate::state::{
    AirGroup, Arrival, Base, LandUnit, Replacement, Ship, Side, TaskForce, TruthState,
};

/// Which optional rules have their figures loaded: air wear and repair
/// (decision 0035), fatigue and rest (0036), and base supply use (0039).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct RulesInPlay {
    pub wear: bool,
    pub fatigue: bool,
    pub supply: bool,
}

/// What every side knows of a base that is not its own: where it is and which
/// side holds it, and its number and name. That is the map, not intelligence
/// (decision 0034, 2). The type has no field for supply, fuel, guns, repairs or
/// aircraft, so nothing more can be carried in it.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PublicBase {
    pub id: BaseId,
    pub name: String,
    pub hex: Hex,
    /// The side that holds it.
    pub side: Side,
}

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
    log: &'a EventLog,
    orders: &'a BTreeMap<OrderId, Order>,
    objectives: Option<&'a crate::objectives::Objectives>,
}

impl std::fmt::Debug for SideView<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SideView")
            .field("side", &self.side)
            .finish_non_exhaustive()
    }
}

impl<'a> SideView<'a> {
    // The world hands over each part of its fixed and changing state by
    // reference; one more argument (the objectives) is clearer than a bundle.
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn new(
        side: Side,
        truth: &'a TruthState,
        belief: &'a BeliefState,
        map: &'a Map,
        ref_data: &'a RefData,
        log: &'a EventLog,
        orders: &'a BTreeMap<OrderId, Order>,
        objectives: Option<&'a crate::objectives::Objectives>,
    ) -> SideView<'a> {
        SideView {
            side,
            truth,
            belief,
            map,
            ref_data,
            log,
            orders,
            objectives,
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

    /// The public facts of every base on the map the side does not own, in
    /// number order (see [`PublicBase`]). Nothing else of an enemy base is
    /// visible, and a base the enemy formed by a landing is not on the map
    /// (decision 0066, 12): it is not here at all.
    pub fn enemy_bases(&self) -> impl Iterator<Item = PublicBase> + '_ {
        let side = self.side;
        self.truth
            .bases
            .values()
            .filter(move |base| base.side != side && base.on_map)
            .map(|base| PublicBase {
                id: base.id,
                name: base.name.clone(),
                hex: base.hex,
                side: base.side,
            })
    }

    /// The side's own land units, known exactly: men ashore and where
    /// (decision 0066, 21). Their men aboard are in [`SideView::ships`], each
    /// ship's `troops`. The enemy's land units are not here, nor anywhere
    /// else in the view (decision 0066, 22). A destroyed unit is listed with
    /// no men.
    pub fn land_units(&self) -> impl Iterator<Item = &'a LandUnit> + '_ {
        let side = self.side;
        self.truth
            .land_units
            .values()
            .filter(move |unit| unit.side == side)
    }

    /// The men of one of the side's own land units aboard its ships. `None`
    /// for a unit that is not the side's.
    pub fn men_aboard(&self, unit: LandUnitId) -> Option<u32> {
        let own = self
            .truth
            .land_units
            .get(&unit)
            .is_some_and(|land_unit| land_unit.side == self.side);
        own.then(|| self.truth.men_aboard(unit))
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

    /// The side's own orders, accepted or not, in the order they were taken
    /// in. This includes the orders a rule gave for the side's units. An
    /// order's `id` is opaque; the side's own count of its orders is
    /// [`SideView::order_number`] (decisions 0055 and 0086).
    pub fn orders(&self) -> impl Iterator<Item = &'a Order> + '_ {
        let side = self.side;
        self.orders.values().filter(move |order| order.side == side)
    }

    /// The events the side may see, each cut down to what it may know, in
    /// log order (decision 0047). Each carries the side's own `number`, 1, 2,
    /// 3 and so on, so a gap in the numbers is not something the side can
    /// see (decision 0086); its `id` is opaque. This is the only way events leave for anything that plays the
    /// game; the full log stays with the umpire.
    pub fn events(&self) -> Vec<SideEvent> {
        let owners = Owners::from_state(
            self.truth,
            self.orders,
            self.ref_data
                .surface_combat
                .as_ref()
                .map_or(0, |figures| figures.round_minutes),
        );
        side_events::project(self.side, self.log.from_index(0), &owners)
    }

    /// The events of one day the side may see, in log order.
    pub fn events_on_day(&self, day: u32) -> Vec<SideEvent> {
        let mut events = self.events();
        events.retain(|event| event.day == day);
        events
    }

    /// One event the side may see, by its (opaque) id. `None` if there is no
    /// such event or the side may not see it. Its causes are the
    /// ones the side may see, and `hidden_causes` says if there are more.
    pub fn event(&self, id: EventId) -> Option<SideEvent> {
        self.events().into_iter().find(|event| event.id == id)
    }

    /// The side's number of an event the side may see: the `number` its
    /// [`SideEvent`] carries (decision 0086). `None` if there is no such
    /// event or the side may not see it.
    pub fn event_number(&self, id: EventId) -> Option<u32> {
        self.event(id).map(|event| event.number)
    }

    /// One event the side may see, by the side's own number of it (1 for the
    /// first [`SideView::events`] gives). `None` if there is no such event.
    pub fn event_by_number(&self, number: u32) -> Option<SideEvent> {
        self.events()
            .into_iter()
            .find(|event| event.number == number)
    }

    /// The side's number of one of its own orders: its place among the
    /// side's own orders, 1 for the first [`SideView::orders`] gives
    /// (decision 0086). `None` if the order is not the side's own.
    pub fn order_number(&self, id: OrderId) -> Option<u32> {
        let own = self.orders().position(|order| order.id == id)?;
        u32::try_from(own + 1).ok()
    }

    /// One of the side's own orders by the side's number of it. `None` if
    /// the side has no such order.
    pub fn order_by_number(&self, number: u32) -> Option<&'a Order> {
        let place = usize::try_from(number.checked_sub(1)?).ok()?;
        self.orders().nth(place)
    }

    /// The side's number of a contact: the count of contacts the side has
    /// made, which is already the side's own (each side numbers its own
    /// contacts from 1).
    pub fn contact_number(&self, id: ContactId) -> u32 {
        id.raw()
    }

    /// One contact the side holds, by the side's number of it. `None` if the
    /// side holds no such contact.
    pub fn contact_by_number(&self, number: u32) -> Option<&'a Contact> {
        self.belief
            .contacts()
            .find(|contact| contact.id.raw() == number)
    }

    fn own_objectives(&self) -> Option<&'a SideObjectives> {
        self.objectives.map(|o| o.side(self.side))
    }

    /// The side's own mission (decision 6 of 0054), or `None` if the
    /// scenario has no objectives. Never the enemy's.
    pub fn mission(&self) -> Option<&'a str> {
        self.own_objectives().map(|o| o.mission.as_str())
    }

    /// The three margins of the scenario that make a minor, a major and a
    /// decisive victory, or `None` if the scenario has no objectives. One set
    /// serves both sides (the margin is one number seen from both), so it
    /// says nothing of the enemy's mission or objectives; it is part of how
    /// the side's own objectives are scored.
    pub fn levels(&self) -> Option<crate::objectives::Levels> {
        self.objectives.map(|o| o.levels)
    }

    /// The days the scenario's result is judged over (decision 0088), or
    /// `None` if the scenario has no objectives. It is the scenario's
    /// configuration, common to both sides like [`SideView::levels`], and
    /// says nothing of the enemy's objectives or of any damage.
    pub fn horizon_days(&self) -> Option<u32> {
        self.objectives.map(|o| o.horizon_days)
    }

    /// The side's own objectives, in the order the scenario gives them
    /// (decision 6 of 0054). Never the enemy's.
    pub fn objectives(&self) -> &'a [Objective] {
        self.own_objectives()
            .map_or(&[], |o| o.objectives.as_slice())
    }

    /// The side's progress on each of its objectives, as far as it can know
    /// it (decision 7 of 0054): counted from its own units and from the
    /// events it may see, and from nothing else. In the order of
    /// [`SideView::objectives`]. What no unit or event of the side shows is
    /// marked not observable and counts 0. The umpire's result is not here.
    pub fn known_progress(&self) -> Vec<KnownProgress> {
        let Some(own) = self.own_objectives() else {
            return Vec::new();
        };
        let bases: Vec<BaseId> = self.bases().map(|base| base.id).collect();
        objectives::known_progress(own, &bases, &self.events())
    }

    /// The map is common knowledge.
    pub fn hex(&self, hex: Hex) -> Option<&'a HexData> {
        self.map.get(hex)
    }

    /// The fewest steps by sea from every hex that can reach `target` to it
    /// (decision 0048, warnings). The map is common knowledge, so this is
    /// not enemy truth; it counts steps, not miles (a step is charged by
    /// [`SideView::sea_charged_nm_in`], decision 0058). Empty if ships cannot
    /// enter `target`.
    pub fn sea_distances_to(&self, target: Hex) -> BTreeMap<Hex, u32> {
        self.map.sea_distances_to(target)
    }

    /// The miles a force is charged for steaming from `from` to `to` by the
    /// map's shortest sea path (decision 0058): [`Map::sea_charged_nm`]. The
    /// straight-line air distance where the way is clear, more where land
    /// forces a detour. `None` when ships cannot get from one to the other.
    pub fn sea_charged_nm(&self, from: Hex, to: Hex) -> Option<u32> {
        self.map.sea_charged_nm(from, to)
    }

    /// [`SideView::sea_charged_nm`] with the field of
    /// [`SideView::sea_distances_to`] of `to` already worked out.
    pub fn sea_charged_nm_in(&self, field: &BTreeMap<Hex, u32>, from: Hex, to: Hex) -> Option<u32> {
        self.map.sea_charged_nm_in(field, from, to)
    }

    /// The hexes a force steams through from `from` to `to`, in order and
    /// ending at `to` (decision 0017), or `None` when ships cannot get from
    /// one to the other. The map is common knowledge, so this is not enemy
    /// truth; what the steps are charged is [`SideView::sea_charged_nm`].
    pub fn sea_path(&self, from: Hex, to: Hex) -> Option<Vec<Hex>> {
        self.map.sea_path(from, to)
    }

    /// What the movement rules would do with one of the side's own task
    /// forces if it were given a route to `destination` now (decision 0049),
    /// at `speed` if given and otherwise at the speed it has, leaving at the
    /// start of the day after [`SideView::day`]. `None` if `task_force` is
    /// not one of the side's own. It reads only the side's own force and
    /// ships, the map and the reference data, and writes nothing.
    pub fn forecast_passage(
        &self,
        task_force: TaskForceId,
        destination: Hex,
        speed: Option<SpeedSetting>,
    ) -> Option<PassageForecast> {
        let force = self.truth.task_forces.get(&task_force)?;
        if force.side != self.side {
            return None;
        }
        let ships: BTreeMap<ShipId, Ship> = force
            .ships
            .iter()
            .filter_map(|id| self.truth.ships.get(id).map(|ship| (*id, ship.clone())))
            .collect();
        Some(movement::forecast_passage(
            self.map,
            self.ref_data,
            force,
            &ships,
            destination,
            speed,
            self.truth.day + 1,
        ))
    }

    /// What a yard order given to one of the side's own task forces now would
    /// do (decision 0069): the port, the passage, each ship's days of repair
    /// and the days away, worked by the function the order itself uses.
    /// `None` if `task_force` is not one of the side's own, or has no ship
    /// afloat in it (a force away has none) or no port for the yard here. It
    /// reads only the side's own force, ships and bases and checks nothing
    /// else; whether the order would be accepted is `World::check_order`'s.
    pub fn yard_plan(&self, task_force: TaskForceId) -> Option<crate::repair::YardPlan> {
        let force = self.truth.task_forces.get(&task_force)?;
        if force.side != self.side {
            return None;
        }
        crate::repair::yard_plan_in(self.truth, self.ref_data, task_force)
    }

    /// Reference data is common knowledge.
    pub fn ship_class(&self, id: ShipClassId) -> Option<&'a ShipClass> {
        self.ref_data.ship_classes.get(&id)
    }

    /// Aircraft types are reference data, so common knowledge.
    pub fn aircraft_type(&self, id: AircraftTypeId) -> Option<&'a AircraftType> {
        self.ref_data.aircraft_types.get(&id)
    }

    /// How far `group`'s strikes and escorts may go with the reach it has in
    /// force, in miles: [`RefData::reach_nm`], the one function every test of
    /// the distance of a strike or an escort uses (decision 2 of 0059). The
    /// figures are rules, so common knowledge.
    pub fn reach_nm(&self, group: &AirGroup) -> Fixed {
        self.ref_data.reach_nm(group.aircraft_type, group.reach)
    }

    /// The contact `group` would launch a strike at now (decision 6 of 0063):
    /// `contact_to_strike` (`airstrike.rs`) on this side's belief and the
    /// group's own state, orders and reach, which is what the strike decision
    /// calls. The group is named by its number and looked up among this
    /// side's own groups, so a caller cannot hand in a group of its own
    /// making. `None` for a number that is not one of this side's groups, a
    /// group that is not on the naval strike mission, has no position, or has
    /// no contact that qualifies. It reads belief and the side's own unit and
    /// nothing of the enemy.
    pub fn strike_choice(&self, group: AirGroupId) -> Option<StrikeChoice> {
        let group = self.air_groups().find(|own| own.id == group)?;
        if group.mission != AirMission::NavalStrike {
            return None;
        }
        let figures = self.ref_data.air_combat.as_ref()?;
        let origin = self.truth.air_group_hex(group)?;
        crate::airstrike::contact_to_strike(
            self.belief,
            group,
            origin,
            self.reach_nm(group),
            figures.strike_contact_max_age_pulses,
        )
    }

    /// The air combat figures' extended reach share and ditching chance are
    /// reference data, so common knowledge: the chance an aircraft runs out
    /// of fuel on the way home from a sortie of one-way distance
    /// `distance_nm`, for a type of strike radius `radius_nm`
    /// (`AirCombatFigures::ditch_chance`). `None` without the figures.
    pub fn ditch_chance(&self, distance_nm: Fixed, radius_nm: Fixed) -> Option<Fixed> {
        self.ref_data
            .air_combat
            .as_ref()
            .map(|figures| figures.ditch_chance(distance_nm, radius_nm))
    }

    /// The search figures are reference data, so common knowledge.
    pub fn search_figures(&self) -> Option<&'a SearchFigures> {
        self.ref_data.search.as_ref()
    }

    /// Which of the optional rules have their figures loaded for this game
    /// (decisions 0035, 0036 and 0039). The figures are rules, not enemy
    /// truth, so they are common knowledge, as the search figures are.
    /// The ground figures (decision 0066), if the scenario names them.
    pub fn ground_figures(&self) -> Option<&'a GroundFigures> {
        self.ref_data.ground.as_ref()
    }

    /// The base supply figures (decision 0039), if the scenario names them.
    pub fn air_supply_figures(&self) -> Option<&'a AirSupplyFigures> {
        self.ref_data.air_supply.as_ref()
    }

    pub fn rules_in_play(&self) -> RulesInPlay {
        RulesInPlay {
            wear: self.ref_data.air_upkeep.is_some(),
            fatigue: self.ref_data.fatigue.is_some(),
            supply: self.ref_data.air_supply.is_some(),
        }
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
    use crate::orders::{Mission, Readiness};
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
            ready: 4,
            strength: 4,
            ..AirGroup::new(
                AirGroupId::new(id),
                format!("Group {id}"),
                side,
                AirGroupLocation::Base(BaseId::new(base)),
                AircraftTypeId::new(1),
            )
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
                Base::new(
                    BaseId::new(id),
                    format!("Base {id}"),
                    side,
                    Hex::new(hex, 0),
                ),
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
                torpedo_tubes_one_side: 0,
                torpedo_damage: Fixed::ZERO,
                toughness: Fixed::from_int(400),
                radar: false,
                aircraft_capacity: 0,
                anti_aircraft: 0,
                troop_capacity_men: 0,
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
                        torpedoes_one_side_fired: false,
                        systems: ShipSystems::ALL_WORKING,
                        experience: Ship::DEFAULT_EXPERIENCE,
                        system_repair_points: Fixed::ZERO,
                        troops: None,
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
