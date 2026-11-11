//! The straggler rule (decisions 10 to 14 of
//! `docs/decisions/0016-systems-and-stragglers.md`).
//!
//! A straggler is a ship whose propulsion is degraded or out. A task force
//! with a straggler and a standing instruction other than keep applies it at
//! the start of every movement step, for every force, and straight after each
//! fight, for the two forces that fought (decision 0016, decision 12, with
//! decision 5 of `docs/decisions/0022-meeting-and-breaking-off.md`). The rule
//! uses no randomness.
//!
//! - Leave: every straggler goes into one new force that stays where it is.
//! - Detach: stragglers that can steam (degraded) go home with escorts taken
//!   from the sound ships; stragglers dead in the water (out) are left. If
//!   the escorts would leave the parent less than its share to keep of the
//!   sound ships it had, or none, the force retires instead.
//! - Retire: ships dead in the water are left, and the rest of the force
//!   heads home together.
//!
//! Heading home is a set-route order issued by the staff and taken in at
//! once, so it is validated and applied like any route order.

use std::cmp::Reverse;
use std::collections::BTreeSet;

use crate::events::{Breakdown, Cause, DetachReason, EventKind, Factor};
use crate::fixed::Fixed;
use crate::ids::{EventId, ShipId, TaskForceId};
use crate::orders::{
    Issuer, OrderKind, OrderRequest, Origin, Stance, StragglerInstruction, TaskForceOrder,
};
use crate::refdata::SurfaceCombatFigures;
use crate::state::{RulesOfEngagement, SystemState, TaskForceStatus};
use crate::time::Pulse;
use crate::world::World;

/// A count of ships as a `Fixed`.
fn count(n: usize) -> Fixed {
    Fixed::from_int(i64::try_from(n).expect("few ships"))
}

impl World {
    /// Applies each task force's straggler instruction. Forces are taken in
    /// ID order as they stand when the pass begins; forces made during the
    /// pass are not processed in it.
    pub(crate) fn handle_stragglers(&mut self, day: u32, pulse: Pulse) {
        // Without surface combat figures no ship can be damaged
        // (`World::new`), so there are no stragglers.
        let Some(figures) = self.ref_data.surface_combat.clone() else {
            return;
        };
        let ids: Vec<TaskForceId> = self.truth.task_forces.keys().copied().collect();
        for id in ids {
            self.handle_stragglers_of(day, pulse, &figures, id);
        }
    }

    /// The same pass for the two forces of a fight, straight after it, the
    /// lower ID first (the caller passes them in that order). A force made
    /// by the pass is not processed in it.
    pub(crate) fn handle_stragglers_after_fight(
        &mut self,
        day: u32,
        pulse: Pulse,
        forces: [TaskForceId; 2],
    ) {
        let Some(figures) = self.ref_data.surface_combat.clone() else {
            return;
        };
        for id in forces {
            self.handle_stragglers_of(day, pulse, &figures, id);
        }
    }

    fn handle_stragglers_of(
        &mut self,
        day: u32,
        pulse: Pulse,
        figures: &SurfaceCombatFigures,
        id: TaskForceId,
    ) {
        let force = &self.truth.task_forces[&id];
        let instruction = force.rules_of_engagement.stragglers;
        if instruction == StragglerInstruction::Keep {
            return;
        }
        let keep_share = force.rules_of_engagement.keep_share;
        let causes: Vec<Cause> = force.rules_order.map(Cause::Order).into_iter().collect();

        // Sort the ships by propulsion. Ships in a force are afloat.
        let mut sound: Vec<ShipId> = Vec::new();
        let mut can_steam: BTreeSet<ShipId> = BTreeSet::new();
        let mut dead_in_water: BTreeSet<ShipId> = BTreeSet::new();
        for ship_id in &force.ships {
            match self.truth.ships[ship_id].systems.propulsion {
                SystemState::Working => sound.push(*ship_id),
                SystemState::Degraded => {
                    can_steam.insert(*ship_id);
                }
                SystemState::Out => {
                    dead_in_water.insert(*ship_id);
                }
            }
        }
        if can_steam.is_empty() && dead_in_water.is_empty() {
            return;
        }
        let sound_count = sound.len();

        if instruction == StragglerInstruction::Leave {
            let stragglers: BTreeSet<ShipId> = can_steam.union(&dead_in_water).copied().collect();
            let breakdown = self.straggler_breakdown(sound_count, 0, 0, keep_share, sound_count);
            self.detach_ships(
                day,
                pulse,
                id,
                &stragglers,
                &BTreeSet::new(),
                DetachReason::LeftBehind,
                &causes,
                breakdown,
            );
            return;
        }

        // Escorts: sum over the stragglers that can steam of the figure for
        // each one's kind. Taken from sound ships with guns, the least
        // important kind first, the lowest ID on a tie.
        let escorts_needed: usize = can_steam
            .iter()
            .map(|ship| {
                let kind = self.ref_data.ship_classes[&self.truth.ships[ship].class].kind;
                usize::try_from(figures.straggler_escorts[&kind]).expect("small figure")
            })
            .sum();
        let mut candidates: Vec<(Reverse<crate::refdata::ShipKind>, ShipId)> = sound
            .iter()
            .filter_map(|ship| {
                let class = &self.ref_data.ship_classes[&self.truth.ships[ship].class];
                (class.main_guns > 0).then_some((Reverse(class.kind), *ship))
            })
            .collect();
        candidates.sort();
        let escorts: BTreeSet<ShipId> = candidates
            .iter()
            .take(escorts_needed)
            .map(|(_, ship)| *ship)
            .collect();
        let kept = sound_count - escorts.len();

        let retires = instruction == StragglerInstruction::Retire
            || kept == 0
            || count(kept) < keep_share * count(sound_count);
        let breakdown =
            self.straggler_breakdown(sound_count, escorts_needed, escorts.len(), keep_share, kept);

        if !retires {
            if !can_steam.is_empty() {
                let (sent_home, event) = self.detach_ships(
                    day,
                    pulse,
                    id,
                    &can_steam,
                    &escorts,
                    DetachReason::SentHome,
                    &causes,
                    breakdown.clone(),
                );
                self.head_home(day, pulse, sent_home, event);
            }
            if !dead_in_water.is_empty() {
                self.detach_ships(
                    day,
                    pulse,
                    id,
                    &dead_in_water,
                    &BTreeSet::new(),
                    DetachReason::LeftBehind,
                    &causes,
                    breakdown,
                );
            }
            return;
        }

        // Retire: leave what is dead in the water, then the parent goes home
        // with everything else.
        if !dead_in_water.is_empty() {
            self.detach_ships(
                day,
                pulse,
                id,
                &dead_in_water,
                &BTreeSet::new(),
                DetachReason::LeftBehind,
                &causes,
                breakdown.clone(),
            );
        }
        // With every ship dead in the water there is nothing left to send
        // home. The empty parent stays as it is.
        if self.truth.task_forces[&id].ships.is_empty() {
            return;
        }
        let parent = self
            .truth
            .task_forces
            .get_mut(&id)
            .expect("the task force exists");
        parent.rules_of_engagement = settled_rules(parent.rules_of_engagement);
        let retired = self.events.record(
            day,
            Some(pulse),
            EventKind::ForceRetired { task_force: id },
            causes,
            breakdown,
        );
        let parent = self
            .truth
            .task_forces
            .get_mut(&id)
            .expect("the task force exists");
        parent.route = crate::state::Route::default();
        parent.route_order = None;
        parent.status = TaskForceStatus::Idle;
        self.head_home(day, pulse, id, retired);
    }

    /// The breakdown of one straggler decision, when recording is on. The
    /// result is the number of sound ships the parent keeps.
    fn straggler_breakdown(
        &self,
        sound_ships: usize,
        escorts_needed: usize,
        escorts_taken: usize,
        keep_share: Fixed,
        kept: usize,
    ) -> Option<Breakdown> {
        self.config.record_breakdowns.then(|| Breakdown {
            factors: vec![
                Factor {
                    name: "sound_ships",
                    value: count(sound_ships),
                },
                Factor {
                    name: "escorts_needed",
                    value: count(escorts_needed),
                },
                Factor {
                    name: "escorts_taken",
                    value: count(escorts_taken),
                },
                Factor {
                    name: "keep_share",
                    value: keep_share,
                },
            ],
            result: count(kept),
        })
    }

    /// Splits `ships` and `escorts` off `parent` into a new force, sets the
    /// new force to avoid combat and keep its own stragglers, and records
    /// `ShipsDetached`. Returns the new force and the event.
    #[allow(clippy::too_many_arguments)]
    fn detach_ships(
        &mut self,
        day: u32,
        pulse: Pulse,
        parent: TaskForceId,
        ships: &BTreeSet<ShipId>,
        escorts: &BTreeSet<ShipId>,
        reason: DetachReason,
        causes: &[Cause],
        breakdown: Option<Breakdown>,
    ) -> (TaskForceId, EventId) {
        let parent_name = self.truth.task_forces[&parent].name.clone();
        let name = match reason {
            DetachReason::SentHome => format!("{parent_name}, sent home"),
            DetachReason::LeftBehind => format!("{parent_name}, left behind"),
            DetachReason::Ordered => format!("{parent_name}, detached"),
        };
        let moved: BTreeSet<ShipId> = ships.union(escorts).copied().collect();
        let new_force = self.split_task_force(parent, &moved, name);
        let force = self
            .truth
            .task_forces
            .get_mut(&new_force)
            .expect("the new task force exists");
        force.rules_of_engagement = settled_rules(force.rules_of_engagement);
        let event = self.events.record(
            day,
            Some(pulse),
            EventKind::ShipsDetached {
                parent,
                new_force,
                ships: ships.iter().copied().collect(),
                escorts: escorts.iter().copied().collect(),
                reason,
            },
            causes.to_vec(),
            breakdown,
        );
        (new_force, event)
    }

    /// Sends a force for its home port with a staff set-route order that
    /// cites `detaching_event` (decision 14). A force with no home port, or
    /// already at a centre at the home port, gets no order. A rejected order
    /// leaves the force as it is: idle.
    fn head_home(&mut self, day: u32, pulse: Pulse, force: TaskForceId, detaching_event: EventId) {
        let task_force = &self.truth.task_forces[&force];
        let Some(base) = task_force
            .home_port
            .and_then(|port| self.truth.bases.get(&port))
        else {
            return;
        };
        if task_force.toward_hex.is_none() && task_force.from_hex == base.hex {
            return;
        }
        let request = OrderRequest {
            kind: OrderKind::TaskForce {
                target: force,
                order: TaskForceOrder::SetRoute(vec![base.hex]),
            },
            origin: Origin {
                issuer: Issuer::Staff,
                intent: None,
            },
        };
        let side = task_force.side;
        self.take_in_order(
            day,
            Some(pulse),
            side,
            request,
            vec![Cause::Event(detaching_event)],
        );
    }
}

/// The rules of a force made for stragglers, or of one that retires: avoid
/// combat, keep its own stragglers, no share to keep (decision 13). The
/// break-off level is the parent's.
fn settled_rules(parent: RulesOfEngagement) -> RulesOfEngagement {
    RulesOfEngagement {
        stance: Stance::Avoid,
        break_off: parent.break_off,
        stragglers: StragglerInstruction::Keep,
        keep_share: Fixed::ZERO,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::events::Event;
    use crate::hex::Hex;
    use crate::ids::{BaseId, OrderId, ShipClassId};
    use crate::invariants;
    use crate::map::{HexData, Map};
    use crate::orders::{Mission, Readiness};
    use crate::refdata::{PortSize, RefData, SearchFigures, ShipClass, ShipKind};
    use crate::resolve::PulsePhase;
    use crate::state::{Base, Ship, ShipLocation, ShipSystems, Side, TaskForce, TruthState};
    use crate::world::SimConfig;

    const TF: TaskForceId = TaskForceId::new(1);
    const HOME: BaseId = BaseId::new(1);

    // Classes, all steaming 16 kn at cruise. The straggler escort figures
    // are those of data/reference/surface_combat.toml: carrier 2, battleship
    // 2, cruiser 1, destroyer 0, transport 1, auxiliary 0.
    const CRUISER: u32 = 1;
    const DESTROYER: u32 = 2;
    /// A destroyer with no main guns.
    const UNARMED: u32 = 3;
    /// A transport with guns.
    const GUNNED_TRANSPORT: u32 = 4;
    const BATTLESHIP: u32 = 5;
    /// An auxiliary with guns.
    const GUNNED_AUXILIARY: u32 = 6;
    const CARRIER: u32 = 7;

    use SystemState::{Degraded, Out, Working};

    fn hx(q: i32, r: i32) -> Hex {
        Hex::new(q, r)
    }

    fn ids(raw: &[u32]) -> Vec<ShipId> {
        raw.iter().map(|n| ShipId::new(*n)).collect()
    }

    /// A task force 1 at (6,0), idle, of the listed ships (class and
    /// propulsion; ship IDs 1, 2, 3 and so on in order), with the given
    /// straggler instruction and share to keep. Base 1, the force's home
    /// port, is at (0,0), 6 hexes west. Its rules of engagement are engage,
    /// break off at 0.5.
    fn world(
        ships: &[(u32, SystemState)],
        instruction: StragglerInstruction,
        keep_share: Fixed,
    ) -> World {
        let mut map = Map::new();
        for q in -1..=12 {
            for r in -2..=2 {
                map.insert(hx(q, r), HexData::open_sea());
            }
        }
        let mut ref_data = RefData::default();
        // (id, kind, main guns)
        let classes = [
            (CRUISER, ShipKind::Cruiser, 8),
            (DESTROYER, ShipKind::Destroyer, 4),
            (UNARMED, ShipKind::Destroyer, 0),
            (GUNNED_TRANSPORT, ShipKind::Transport, 2),
            (BATTLESHIP, ShipKind::Battleship, 9),
            (GUNNED_AUXILIARY, ShipKind::Auxiliary, 1),
            (CARRIER, ShipKind::Carrier, 4),
        ];
        for (id, kind, guns) in classes {
            ref_data.ship_classes.insert(
                ShipClassId::new(id),
                ShipClass {
                    id: ShipClassId::new(id),
                    name: format!("Test class {id}"),
                    kind,
                    cruise_speed_kn: Fixed::from_int(16),
                    max_speed_kn: Fixed::from_int(32),
                    fuel_capacity_t: Fixed::from_int(2000),
                    endurance_nm: Fixed::from_int(8000),
                    cargo_capacity_t: Fixed::ZERO,
                    main_guns: guns,
                    gun_rounds_per_min: if guns > 0 {
                        Fixed::from_int(3)
                    } else {
                        Fixed::ZERO
                    },
                    gun_rounds_per_gun: if guns > 0 { 100 } else { 0 },
                    gun_damage: if guns > 0 {
                        Fixed::from_int(20)
                    } else {
                        Fixed::ZERO
                    },
                    torpedo_tubes: 0,
                    torpedo_damage: Fixed::ZERO,
                    toughness: Fixed::from_int(300),
                    radar: false,
                    aircraft_capacity: 0,
                },
            );
        }
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

        let mut truth = TruthState::default();
        let mut force = TaskForce::at(
            TF,
            "Test force".to_string(),
            Side::Allied,
            hx(6, 0),
            Mission::SurfaceCombat,
        );
        force.home_port = Some(HOME);
        force.rules_of_engagement = RulesOfEngagement {
            stance: Stance::Engage,
            break_off: Fixed::from_ratio(1, 2),
            stragglers: instruction,
            keep_share,
        };
        for (index, (class, propulsion)) in ships.iter().enumerate() {
            let id = ShipId::new(u32::try_from(index + 1).expect("few ships"));
            force.ships.insert(id);
            let mut systems = ShipSystems::ALL_WORKING;
            systems.propulsion = *propulsion;
            truth.ships.insert(
                id,
                Ship {
                    id,
                    name: format!("Test ship {}", id.raw()),
                    class: ShipClassId::new(*class),
                    side: Side::Allied,
                    fuel_t: Fixed::from_int(2000),
                    supply_t: Fixed::ZERO,
                    location: ShipLocation::TaskForce(TF),
                    flotation_damage: Fixed::ZERO,
                    fire_damage: Fixed::ZERO,
                    gun_rounds: Fixed::ZERO,
                    torpedoes_ready: false,
                    systems,
                },
            );
        }
        truth.task_forces.insert(TF, force);
        truth.bases.insert(
            HOME,
            Base {
                id: HOME,
                name: "Home".to_string(),
                side: Side::Allied,
                hex: hx(0, 0),
                supply_t: Fixed::ZERO,
            },
        );
        World::new(7, ref_data, map, truth).expect("valid setup")
    }

    /// The scenario's force: cruisers 1, 2 (degraded) and 3, destroyers 4, 5
    /// and 6 (out).
    fn scenario_ships() -> Vec<(u32, SystemState)> {
        vec![
            (CRUISER, Working),
            (CRUISER, Degraded),
            (CRUISER, Working),
            (DESTROYER, Working),
            (DESTROYER, Working),
            (DESTROYER, Out),
        ]
    }

    fn half() -> Fixed {
        Fixed::from_ratio(1, 2)
    }

    fn force(world: &World, raw: u32) -> &TaskForce {
        &world.truth.task_forces[&TaskForceId::new(raw)]
    }

    fn members(world: &World, raw: u32) -> Vec<ShipId> {
        force(world, raw).ships.iter().copied().collect()
    }

    fn assert_sound(world: &World) {
        let violations = invariants::check(world.umpire(), world.map(), world.ref_data());
        assert!(violations.is_empty(), "{violations:?}");
        // No ship is in two forces, and every ship is where it says.
        for (id, ship) in &world.truth.ships {
            let holders: Vec<_> = world
                .truth
                .task_forces
                .values()
                .filter(|force| force.ships.contains(id))
                .collect();
            assert_eq!(holders.len(), 1, "{id}");
            assert_eq!(ship.location, ShipLocation::TaskForce(holders[0].id));
        }
    }

    /// The events of a kind, with their IDs.
    fn detached(world: &World) -> Vec<(&Event, TaskForceId, TaskForceId, DetachReason)> {
        world
            .events()
            .iter()
            .filter_map(|event| match &event.kind {
                EventKind::ShipsDetached {
                    parent,
                    new_force,
                    reason,
                    ..
                } => Some((event, *parent, *new_force, *reason)),
                _ => None,
            })
            .collect()
    }

    fn retired(world: &World) -> Vec<&Event> {
        world
            .events()
            .iter()
            .filter(|event| matches!(event.kind, EventKind::ForceRetired { .. }))
            .collect()
    }

    fn accepted_orders(world: &World) -> Vec<&Event> {
        world
            .events()
            .iter()
            .filter(|event| matches!(event.kind, EventKind::OrderAccepted { .. }))
            .collect()
    }

    /// The settings a force made for stragglers, or one that retired, has:
    /// avoid, the parent's break-off level 0.5, keep stragglers, share 0.
    fn assert_settled(world: &World, raw: u32) {
        let rules = force(world, raw).rules_of_engagement;
        assert_eq!(rules.stance, Stance::Avoid);
        assert_eq!(rules.break_off, half());
        assert_eq!(rules.stragglers, StragglerInstruction::Keep);
        assert_eq!(rules.keep_share, Fixed::ZERO);
    }

    fn assert_goes_home(world: &World, raw: u32) {
        let force = force(world, raw);
        assert_eq!(force.status, TaskForceStatus::Underway);
        assert_eq!(force.route.waypoints, vec![hx(0, 0)]);
        assert_eq!(force.route.next_index, 0);
        assert!(force.route_order.is_some());
    }

    fn assert_stays(world: &World, raw: u32) {
        let force = force(world, raw);
        assert_eq!(force.status, TaskForceStatus::Idle);
        assert!(force.route.waypoints.is_empty());
        assert_eq!(force.route_order, None);
    }

    #[test]
    fn keep_does_nothing() {
        // Cruiser 2 degraded and destroyer 3 out, but the instruction is
        // keep: no event, no new force.
        let mut world = world(
            &[(CRUISER, Working), (CRUISER, Degraded), (DESTROYER, Out)],
            StragglerInstruction::Keep,
            half(),
        );
        let before = world.state_hash();
        world.handle_stragglers(1, Pulse::Night);
        assert!(world.events().is_empty());
        assert_eq!(world.truth.task_forces.len(), 1);
        assert_eq!(world.state_hash(), before);
        assert_sound(&world);
    }

    #[test]
    fn a_force_with_no_straggler_is_left_alone_whatever_its_instruction() {
        for instruction in [
            StragglerInstruction::Leave,
            StragglerInstruction::Detach,
            StragglerInstruction::Retire,
        ] {
            let mut world = world(&[(CRUISER, Working); 3], instruction, half());
            world.handle_stragglers(1, Pulse::Night);
            assert!(world.events().is_empty());
            assert_eq!(world.truth.task_forces.len(), 1);
        }
    }

    #[test]
    fn a_world_without_surface_combat_figures_does_nothing() {
        let mut world = world(&[(CRUISER, Working)], StragglerInstruction::Leave, half());
        world.ref_data.surface_combat = None;
        world.handle_stragglers(1, Pulse::Night);
        assert!(world.events().is_empty());
    }

    #[test]
    fn leave_puts_every_straggler_into_one_force_that_stays() {
        // Ships 1 and 3 work, 2 is degraded, 4 is out. Leave: ships 2 and 4
        // go to task force 2, which stays at (6,0). U = 2 sound ships, so
        // the parent keeps R = U = 2 and no escorts are needed or taken.
        let mut world = world(
            &[
                (CRUISER, Working),
                (CRUISER, Degraded),
                (DESTROYER, Working),
                (DESTROYER, Out),
            ],
            StragglerInstruction::Leave,
            half(),
        );
        world.truth.task_forces.get_mut(&TF).unwrap().rules_order = Some(OrderId::new(9));
        world.handle_stragglers(1, Pulse::Night);

        assert_eq!(members(&world, 1), ids(&[1, 3]));
        assert_eq!(members(&world, 2), ids(&[2, 4]));
        assert_eq!(force(&world, 2).name, "Test force, left behind");
        assert_eq!(force(&world, 2).position(), hx(6, 0));
        assert_stays(&world, 2);
        assert_settled(&world, 2);
        // The parent's rules are not touched.
        assert_eq!(
            force(&world, 1).rules_of_engagement.stragglers,
            StragglerInstruction::Leave
        );
        assert!(retired(&world).is_empty());
        assert!(accepted_orders(&world).is_empty());

        let events = detached(&world);
        assert_eq!(events.len(), 1);
        let (event, parent, new_force, reason) = events[0];
        assert_eq!(
            (parent, new_force, reason),
            (TF, TaskForceId::new(2), DetachReason::LeftBehind)
        );
        let EventKind::ShipsDetached { ships, escorts, .. } = &event.kind else {
            unreachable!()
        };
        assert_eq!(ships, &ids(&[2, 4]));
        assert!(escorts.is_empty());
        assert_eq!(event.pulse, Some(Pulse::Night));
        assert_eq!(event.causes, vec![Cause::Order(OrderId::new(9))]);
        let breakdown = event.breakdown.as_ref().expect("recorded");
        let values: Vec<(&str, Fixed)> = breakdown
            .factors
            .iter()
            .map(|f| (f.name, f.value))
            .collect();
        assert_eq!(
            values,
            vec![
                ("sound_ships", Fixed::from_int(2)),
                ("escorts_needed", Fixed::ZERO),
                ("escorts_taken", Fixed::ZERO),
                ("keep_share", half()),
            ]
        );
        assert_eq!(breakdown.result, Fixed::from_int(2));
        assert_sound(&world);
    }

    #[test]
    fn a_force_with_no_rules_order_cites_nothing() {
        let mut world = world(&scenario_ships(), StragglerInstruction::Leave, half());
        world.handle_stragglers(1, Pulse::Night);
        assert!(detached(&world)[0].0.causes.is_empty());
    }

    #[test]
    fn detach_sends_the_steaming_straggler_home_with_an_escort_and_leaves_the_rest() {
        // The scenario. Sound: 1, 3, 4, 5, so U = 4. Straggler 2 (cruiser)
        // can steam: E = 1. Straggler 6 is out. Candidates, least important
        // first: destroyers 4, 5, then cruisers 1, 3. Taken: 4. R = 3, and
        // 3 >= 0.5 x 4 = 2, so the force detaches and does not retire.
        let mut world = world(&scenario_ships(), StragglerInstruction::Detach, half());
        world.truth.task_forces.get_mut(&TF).unwrap().rules_order = Some(OrderId::new(9));
        world.handle_stragglers(1, Pulse::Night);

        assert_eq!(members(&world, 1), ids(&[1, 3, 5]));
        assert_eq!(members(&world, 2), ids(&[2, 4]));
        assert_eq!(members(&world, 3), ids(&[6]));
        assert_eq!(force(&world, 2).name, "Test force, sent home");
        assert_eq!(force(&world, 3).name, "Test force, left behind");
        assert_settled(&world, 2);
        assert_settled(&world, 3);
        assert_goes_home(&world, 2);
        assert_stays(&world, 3);
        assert_eq!(force(&world, 3).position(), hx(6, 0));
        // The parent is unchanged.
        assert_eq!(
            force(&world, 1).rules_of_engagement.stragglers,
            StragglerInstruction::Detach
        );
        assert_eq!(force(&world, 1).rules_of_engagement.stance, Stance::Engage);

        // Event order: sent home, its order, left behind.
        let log: Vec<&Event> = world.events().iter().collect();
        assert_eq!(log.len(), 3);
        let EventKind::ShipsDetached {
            parent,
            new_force,
            ships,
            escorts,
            reason,
        } = &log[0].kind
        else {
            panic!("{}", log[0]);
        };
        assert_eq!(
            (*parent, *new_force, *reason),
            (TF, TaskForceId::new(2), DetachReason::SentHome)
        );
        assert_eq!(ships, &ids(&[2]));
        assert_eq!(escorts, &ids(&[4]));
        assert_eq!(log[0].causes, vec![Cause::Order(OrderId::new(9))]);
        let breakdown = log[0].breakdown.as_ref().expect("recorded");
        let values: Vec<(&str, Fixed)> = breakdown
            .factors
            .iter()
            .map(|f| (f.name, f.value))
            .collect();
        assert_eq!(
            values,
            vec![
                ("sound_ships", Fixed::from_int(4)),
                ("escorts_needed", Fixed::from_int(1)),
                ("escorts_taken", Fixed::from_int(1)),
                ("keep_share", half()),
            ]
        );
        assert_eq!(breakdown.result, Fixed::from_int(3));

        // The staff order: issued by the staff, cites itself and the event.
        let EventKind::OrderAccepted { order, target } = log[1].kind else {
            panic!("{}", log[1]);
        };
        assert_eq!(
            target,
            crate::ids::EntityRef::TaskForce(TaskForceId::new(2))
        );
        assert_eq!(
            log[1].causes,
            vec![Cause::Order(order), Cause::Event(log[0].id)]
        );
        let stored = world.order(order).expect("stored");
        assert_eq!(stored.origin.issuer, Issuer::Staff);
        assert_eq!(force(&world, 2).route_order, Some(order));

        let EventKind::ShipsDetached {
            parent,
            new_force,
            ships,
            escorts,
            reason,
        } = &log[2].kind
        else {
            panic!("{}", log[2]);
        };
        assert_eq!(
            (*parent, *new_force, *reason),
            (TF, TaskForceId::new(3), DetachReason::LeftBehind)
        );
        assert_eq!(ships, &ids(&[6]));
        assert!(escorts.is_empty());
        assert!(retired(&world).is_empty());
        assert_sound(&world);
    }

    #[test]
    fn detach_with_no_straggler_that_can_steam_takes_no_escorts() {
        // Only ship 3 is a straggler and it is out: E = 0 (nothing steams),
        // nothing is taken, R = U = 2. Only a left-behind force is made.
        let mut world = world(
            &[(CRUISER, Working), (DESTROYER, Working), (DESTROYER, Out)],
            StragglerInstruction::Detach,
            half(),
        );
        world.handle_stragglers(1, Pulse::Night);
        assert_eq!(members(&world, 1), ids(&[1, 2]));
        assert_eq!(members(&world, 2), ids(&[3]));
        assert_eq!(world.truth.task_forces.len(), 2);
        assert_stays(&world, 2);
        let events = detached(&world);
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].3, DetachReason::LeftBehind);
        assert!(accepted_orders(&world).is_empty());
        assert_sound(&world);
    }

    #[test]
    fn retire_leaves_the_dead_and_the_rest_go_home_together() {
        // Instruction retire, whatever the share (0). Ship 6 is out and is
        // left as task force 2. The parent keeps 1 to 5, stragglers
        // included, is set to avoid, and heads for (0,0).
        let mut world = world(&scenario_ships(), StragglerInstruction::Retire, Fixed::ZERO);
        world.truth.task_forces.get_mut(&TF).unwrap().rules_order = Some(OrderId::new(9));
        world.truth.task_forces.get_mut(&TF).unwrap().route = crate::state::Route {
            waypoints: vec![hx(12, 0)],
            next_index: 0,
        };
        world.truth.task_forces.get_mut(&TF).unwrap().status = TaskForceStatus::Underway;
        world.truth.task_forces.get_mut(&TF).unwrap().route_order = Some(OrderId::new(5));
        world.handle_stragglers(1, Pulse::Night);

        assert_eq!(members(&world, 1), ids(&[1, 2, 3, 4, 5]));
        assert_eq!(members(&world, 2), ids(&[6]));
        assert_eq!(force(&world, 2).name, "Test force, left behind");
        assert_stays(&world, 2);
        assert_settled(&world, 2);
        assert_settled(&world, 1);
        // The parent's rules order is left as it was; its route is the
        // staff's.
        assert_eq!(force(&world, 1).rules_order, Some(OrderId::new(9)));
        assert_goes_home(&world, 1);
        assert_ne!(force(&world, 1).route_order, Some(OrderId::new(5)));

        let log: Vec<&Event> = world.events().iter().collect();
        assert_eq!(log.len(), 3);
        assert!(matches!(
            log[0].kind,
            EventKind::ShipsDetached {
                reason: DetachReason::LeftBehind,
                ..
            }
        ));
        assert!(matches!(
            log[1].kind,
            EventKind::ForceRetired { task_force } if task_force == TF
        ));
        assert_eq!(log[1].causes, vec![Cause::Order(OrderId::new(9))]);
        let EventKind::OrderAccepted { order, .. } = log[2].kind else {
            panic!("{}", log[2]);
        };
        assert_eq!(
            log[2].causes,
            vec![Cause::Order(order), Cause::Event(log[1].id)]
        );
        assert_eq!(force(&world, 1).route_order, Some(order));
        // U = 4 sound (1, 3, 4, 5), E = 1 (cruiser 2), taken 1 (destroyer
        // 4), so the numbers on the event say 3 would stay.
        let breakdown = log[1].breakdown.as_ref().expect("recorded");
        assert_eq!(breakdown.result, Fixed::from_int(3));
        assert_sound(&world);
    }

    #[test]
    fn detach_retires_when_the_share_to_keep_is_not_met() {
        // U = 4 sound ships (1, 3, 4, 5), E = 1, taken 1, R = 3. Keep share
        // 0.8: 0.8 x 4 = 3.2 and 3 < 3.2, so the force retires. Ship 6 is
        // left behind, nobody is sent home with escorts.
        let mut world = world(
            &scenario_ships(),
            StragglerInstruction::Detach,
            Fixed::from_ratio(4, 5),
        );
        world.handle_stragglers(1, Pulse::Night);
        assert_eq!(members(&world, 1), ids(&[1, 2, 3, 4, 5]));
        assert_eq!(members(&world, 2), ids(&[6]));
        assert_eq!(world.truth.task_forces.len(), 2);
        assert_goes_home(&world, 1);
        assert_settled(&world, 1);
        assert_eq!(retired(&world).len(), 1);
        assert_eq!(detached(&world).len(), 1);
        assert_sound(&world);

        // At exactly the share (0.75 x 4 = 3, R = 3) it detaches.
        let mut exact = self::world(
            &scenario_ships(),
            StragglerInstruction::Detach,
            Fixed::from_ratio(3, 4),
        );
        exact.handle_stragglers(1, Pulse::Night);
        assert!(retired(&exact).is_empty());
        assert_eq!(members(&exact, 1), ids(&[1, 3, 5]));
    }

    #[test]
    fn detach_retires_when_the_escorts_would_take_every_sound_ship() {
        // Cruisers 1, 2 and 3 degraded: E = 3. Sound: destroyers 4 and 5, so
        // U = 2 and only 2 can be taken. R = 2 - 2 = 0: the force retires
        // whatever the share (0). Ship 6 is out and is left.
        let mut world = world(
            &[
                (CRUISER, Degraded),
                (CRUISER, Degraded),
                (CRUISER, Degraded),
                (DESTROYER, Working),
                (DESTROYER, Working),
                (DESTROYER, Out),
            ],
            StragglerInstruction::Detach,
            Fixed::ZERO,
        );
        world.handle_stragglers(1, Pulse::Night);
        assert_eq!(members(&world, 1), ids(&[1, 2, 3, 4, 5]));
        assert_eq!(members(&world, 2), ids(&[6]));
        assert_goes_home(&world, 1);
        assert_eq!(retired(&world).len(), 1);
        let breakdown = retired(&world)[0].breakdown.clone().expect("recorded");
        let values: Vec<Fixed> = breakdown.factors.iter().map(|f| f.value).collect();
        assert_eq!(
            values,
            vec![
                Fixed::from_int(2),
                Fixed::from_int(3),
                Fixed::from_int(2),
                Fixed::ZERO
            ]
        );
        assert_eq!(breakdown.result, Fixed::ZERO);
        assert_sound(&world);
    }

    #[test]
    fn escorts_are_the_least_important_kind_first_then_the_lowest_id() {
        // Stragglers: battleship 1 (2 escorts) and carrier 2 (2): E = 4.
        // Sound with guns, least important first: auxiliary 6, transport 5,
        // destroyers 4 and 7 (ID order), then cruiser 3. Taken: 6, 5, 4, 7.
        // U = 5, R = 1; the share is 0, so the force detaches.
        let mut world = world(
            &[
                (BATTLESHIP, Degraded),
                (CARRIER, Degraded),
                (CRUISER, Working),
                (DESTROYER, Working),
                (GUNNED_TRANSPORT, Working),
                (GUNNED_AUXILIARY, Working),
                (DESTROYER, Working),
            ],
            StragglerInstruction::Detach,
            Fixed::ZERO,
        );
        world.handle_stragglers(1, Pulse::Night);
        assert_eq!(members(&world, 1), ids(&[3]));
        assert_eq!(members(&world, 2), ids(&[1, 2, 4, 5, 6, 7]));
        let EventKind::ShipsDetached { ships, escorts, .. } =
            &world.events().iter().next().unwrap().kind
        else {
            panic!()
        };
        assert_eq!(ships, &ids(&[1, 2]));
        assert_eq!(escorts, &ids(&[4, 5, 6, 7]));
        assert_sound(&world);

        // With E = 1 (one degraded cruiser) the escort is the least
        // important: transport 3 before destroyer 2 and cruiser 4.
        let mut one = self::world(
            &[
                (CRUISER, Degraded),
                (DESTROYER, Working),
                (GUNNED_TRANSPORT, Working),
                (CRUISER, Working),
            ],
            StragglerInstruction::Detach,
            Fixed::ZERO,
        );
        one.handle_stragglers(1, Pulse::Night);
        assert_eq!(members(&one, 2), ids(&[1, 3]));
        // And a tie on kind goes to the lowest ID.
        let mut tie = self::world(
            &[
                (CRUISER, Degraded),
                (DESTROYER, Working),
                (DESTROYER, Working),
            ],
            StragglerInstruction::Detach,
            Fixed::ZERO,
        );
        tie.handle_stragglers(1, Pulse::Night);
        assert_eq!(members(&tie, 2), ids(&[1, 2]));
    }

    #[test]
    fn a_sound_ship_with_no_main_guns_is_never_an_escort() {
        // Cruiser 1 degraded: E = 1. Sound: unarmed destroyer 2, armed
        // destroyer 3, cruiser 4. Ship 2 would be first by ID but has no
        // guns, so the escort is destroyer 3.
        let mut world = world(
            &[
                (CRUISER, Degraded),
                (UNARMED, Working),
                (DESTROYER, Working),
                (CRUISER, Working),
            ],
            StragglerInstruction::Detach,
            Fixed::ZERO,
        );
        world.handle_stragglers(1, Pulse::Night);
        assert_eq!(members(&world, 2), ids(&[1, 3]));
        assert_eq!(members(&world, 1), ids(&[2, 4]));
        assert_sound(&world);

        // With only an unarmed ship sound there is no escort: it is not
        // taken, so R = U = 1 and the share 0.5 is met. The straggler goes
        // home alone.
        let mut alone = self::world(
            &[(CRUISER, Degraded), (UNARMED, Working)],
            StragglerInstruction::Detach,
            half(),
        );
        alone.handle_stragglers(1, Pulse::Night);
        assert_eq!(members(&alone, 2), ids(&[1]));
        assert_eq!(members(&alone, 1), ids(&[2]));
        let EventKind::ShipsDetached { escorts, .. } = &alone.events().iter().next().unwrap().kind
        else {
            panic!()
        };
        assert!(escorts.is_empty());
    }

    /// Puts task force 1 mid-leg on a leg from (6,0) toward `toward` with
    /// `progress` nm covered.
    fn mid_leg(world: &mut World, toward: Hex, progress: i64) {
        let force = world.truth.task_forces.get_mut(&TF).unwrap();
        force.toward_hex = Some(toward);
        force.progress_nm = Fixed::from_int(progress);
    }

    #[test]
    fn a_force_sent_home_mid_leg_turns_back_when_that_is_shorter() {
        // The parent is 10 nm along the leg from (6,0) east to (7,0). The
        // sent-home force starts at the same spot. Waypoint (0,0), 6 hexes
        // from (6,0), 7 from (7,0). Going on: (40 - 10) + 40 x 7 = 310.
        // Turning back: 10 + 40 x 6 = 250. 250 < 310, so it turns back: it
        // is now on the leg from (7,0) toward (6,0), 30 nm along.
        let mut world = world(&scenario_ships(), StragglerInstruction::Detach, half());
        mid_leg(&mut world, hx(7, 0), 10);
        world.handle_stragglers(1, Pulse::Night);
        let sent = force(&world, 2);
        assert_eq!(sent.from_hex, hx(7, 0));
        assert_eq!(sent.toward_hex, Some(hx(6, 0)));
        assert_eq!(sent.progress_nm, Fixed::from_int(30));
        assert_goes_home(&world, 2);
        // The parent and the left-behind force keep the leg as it was.
        assert_eq!(force(&world, 1).from_hex, hx(6, 0));
        assert_eq!(force(&world, 1).toward_hex, Some(hx(7, 0)));
        assert_eq!(force(&world, 3).from_hex, hx(6, 0));
        assert_eq!(force(&world, 3).toward_hex, Some(hx(7, 0)));
        assert_eq!(force(&world, 3).progress_nm, Fixed::from_int(10));
        assert_eq!(force(&world, 3).status, TaskForceStatus::Idle);
        assert_sound(&world);
    }

    #[test]
    fn a_force_sent_home_mid_leg_goes_on_when_that_is_shorter() {
        // The parent is 10 nm along the leg from (6,0) west to (5,0).
        // Going on: 30 + 40 x 5 = 230. Turning back: 10 + 40 x 6 = 250.
        // 230 < 250, so it goes on and keeps the leg.
        let mut world = world(&scenario_ships(), StragglerInstruction::Detach, half());
        mid_leg(&mut world, hx(5, 0), 10);
        world.handle_stragglers(1, Pulse::Night);
        let sent = force(&world, 2);
        assert_eq!(sent.from_hex, hx(6, 0));
        assert_eq!(sent.toward_hex, Some(hx(5, 0)));
        assert_eq!(sent.progress_nm, Fixed::from_int(10));
        assert_goes_home(&world, 2);
        assert_sound(&world);
    }

    #[test]
    fn a_retiring_parent_mid_leg_turns_back_by_the_same_rule() {
        // Retire, 10 nm along the leg east from (6,0): turns back as above.
        let mut world = world(&scenario_ships(), StragglerInstruction::Retire, Fixed::ZERO);
        mid_leg(&mut world, hx(7, 0), 10);
        world.handle_stragglers(1, Pulse::Night);
        let parent = force(&world, 1);
        assert_eq!(parent.from_hex, hx(7, 0));
        assert_eq!(parent.toward_hex, Some(hx(6, 0)));
        assert_eq!(parent.progress_nm, Fixed::from_int(30));
        assert_goes_home(&world, 1);
        assert_sound(&world);
    }

    #[test]
    fn with_no_home_port_the_force_is_made_but_gets_no_order() {
        let mut world = world(&scenario_ships(), StragglerInstruction::Detach, half());
        world.truth.task_forces.get_mut(&TF).unwrap().home_port = None;
        world.handle_stragglers(1, Pulse::Night);
        assert_eq!(members(&world, 2), ids(&[2, 4]));
        assert_eq!(members(&world, 3), ids(&[6]));
        assert_stays(&world, 2);
        assert!(accepted_orders(&world).is_empty());
        assert_eq!(world.events().len(), 2);

        // A retiring force with no home port stops: route cleared, idle.
        let mut retiring = self::world(&scenario_ships(), StragglerInstruction::Retire, half());
        {
            let force = retiring.truth.task_forces.get_mut(&TF).unwrap();
            force.home_port = None;
            force.route = crate::state::Route {
                waypoints: vec![hx(12, 0)],
                next_index: 0,
            };
            force.status = TaskForceStatus::Underway;
            force.route_order = Some(OrderId::new(5));
        }
        retiring.handle_stragglers(1, Pulse::Night);
        assert_stays(&retiring, 1);
        assert_settled(&retiring, 1);
        assert_eq!(retired(&retiring).len(), 1);
        assert!(accepted_orders(&retiring).is_empty());
        assert_sound(&retiring);
    }

    #[test]
    fn a_force_already_at_its_home_port_gets_no_order() {
        // Task force 1 sits on the centre of (0,0), the home port's hex.
        let mut world = world(&scenario_ships(), StragglerInstruction::Detach, half());
        {
            let force = world.truth.task_forces.get_mut(&TF).unwrap();
            force.from_hex = hx(0, 0);
        }
        world.handle_stragglers(1, Pulse::Night);
        assert_eq!(members(&world, 2), ids(&[2, 4]));
        assert_stays(&world, 2);
        assert_eq!(force(&world, 2).position(), hx(0, 0));
        assert!(accepted_orders(&world).is_empty());

        let mut retiring = self::world(&scenario_ships(), StragglerInstruction::Retire, half());
        retiring.truth.task_forces.get_mut(&TF).unwrap().from_hex = hx(0, 0);
        retiring.handle_stragglers(1, Pulse::Night);
        assert_stays(&retiring, 1);
        assert!(accepted_orders(&retiring).is_empty());
        assert_sound(&retiring);
    }

    #[test]
    fn a_rejected_staff_order_leaves_the_force_idle() {
        // The home port's hex is made land, so the route is blocked.
        let mut world = world(&scenario_ships(), StragglerInstruction::Detach, half());
        world.map.insert(
            hx(0, 0),
            HexData {
                kind: crate::map::HexKind::Land,
                ..HexData::open_sea()
            },
        );
        world.handle_stragglers(1, Pulse::Night);
        assert_stays(&world, 2);
        assert!(world
            .events()
            .iter()
            .any(|event| matches!(event.kind, EventKind::OrderRejected { .. })));
    }

    #[test]
    fn the_instruction_is_applied_at_the_end_of_a_surface_engagement_step() {
        // Propulsion goes down with no movement step before it: the end of
        // the surface engagement step catches it, with no enemy near.
        let mut world = world(&scenario_ships(), StragglerInstruction::Detach, half());
        world.run_pulse_phase(1, Pulse::Night, PulsePhase::Movement);
        assert_eq!(members(&world, 1), ids(&[1, 3, 5]));
        assert_eq!(members(&world, 2), ids(&[2, 4]));
        assert_eq!(members(&world, 3), ids(&[6]));
        assert_sound(&world);
    }

    #[test]
    fn a_fight_that_cripples_a_ship_is_followed_by_the_instruction() {
        // Task force 1 (three cruisers, detach) meets an enemy force in one
        // hex at night. Look for a seed in which a hit stops an engine;
        // then the detachment is recorded after the fight ends, and the
        // force that was hit is not left with a straggler.
        let mut found = false;
        for seed in 1..=300 {
            let mut world = fight_world(seed);
            world.run_pulse_phase(1, Pulse::Night, PulsePhase::Movement);
            let events: Vec<&Event> = world.events().iter().collect();
            let Some(detach) = events
                .iter()
                .position(|event| matches!(event.kind, EventKind::ShipsDetached { .. }))
            else {
                continue;
            };
            let ended = events
                .iter()
                .rposition(|event| matches!(event.kind, EventKind::EngagementEnded { .. }))
                .expect("a fight ended");
            assert!(detach > ended, "seed {seed}: detached after the fight");
            // Nothing was cited from the fight: no rules order here.
            assert!(events[detach].causes.is_empty());
            for force in world.truth.task_forces.values() {
                if force.rules_of_engagement.stragglers == StragglerInstruction::Keep {
                    continue;
                }
                for ship in &force.ships {
                    assert_eq!(
                        world.truth.ships[ship].systems.propulsion, Working,
                        "seed {seed}: {ship} is a straggler in {}",
                        force.id
                    );
                }
            }
            assert_sound(&world);
            found = true;
            break;
        }
        assert!(found, "no seed in 1..=300 stopped an engine");
    }

    /// Allied task force 1 (four cruisers, detach, home port base 1 at
    /// (0,0)) and Japanese task force 2 (three cruisers) together at (5,0).
    fn fight_world(seed: u64) -> World {
        let mut world = world(
            &[(CRUISER, Working); 4],
            StragglerInstruction::Detach,
            Fixed::ZERO,
        );
        world.seed = seed;
        {
            let force = world.truth.task_forces.get_mut(&TF).unwrap();
            force.from_hex = hx(5, 0);
            force.readiness = Readiness::Normal;
        }
        let enemy = TaskForceId::new(2);
        world.task_force_ids.alloc();
        let mut force = TaskForce::at(
            enemy,
            "Enemy".to_string(),
            Side::Japanese,
            hx(5, 0),
            Mission::SurfaceCombat,
        );
        // The enemy was alert by default before decision 0021.
        force.readiness = Readiness::Battle;
        for raw in 11..=13 {
            let id = ShipId::new(raw);
            let mut ship = world.truth.ships[&ShipId::new(1)].clone();
            ship.id = id;
            ship.side = Side::Japanese;
            ship.location = ShipLocation::TaskForce(enemy);
            ship.gun_rounds = Fixed::from_int(800);
            world.truth.ships.insert(id, ship);
            force.ships.insert(id);
        }
        world.truth.task_forces.insert(enemy, force);
        for ship in world.truth.ships.values_mut() {
            if ship.side == Side::Allied {
                ship.gun_rounds = Fixed::from_int(800);
            }
        }
        world
    }

    #[test]
    fn the_instruction_is_not_applied_twice() {
        // After the first pass the new forces keep their stragglers and the
        // parent has none: a second pass, a second movement step and a
        // second day detach nothing more.
        let mut world = world(&scenario_ships(), StragglerInstruction::Detach, half());
        world.resolve_day().expect("day 1");
        let after_day_one = (world.events().len(), world.truth.task_forces.len());
        assert_eq!(detached(&world).len(), 2);
        world.handle_stragglers(2, Pulse::Night);
        world.resolve_day().expect("day 2");
        assert_eq!(detached(&world).len(), 2);
        assert_eq!(world.truth.task_forces.len(), after_day_one.1);
        assert!(retired(&world).is_empty());
        assert_sound(&world);
    }

    #[test]
    fn leave_with_every_ship_a_straggler_leaves_an_empty_parent_that_cannot_hurt_anything() {
        // Both ships are stragglers, so U = 0 and both go to task force 2.
        // The empty parent goes nowhere and the days resolve.
        let mut world = world(
            &[(CRUISER, Degraded), (DESTROYER, Out)],
            StragglerInstruction::Leave,
            half(),
        );
        {
            let force = world.truth.task_forces.get_mut(&TF).unwrap();
            // At the home port with a load order, so cargo handling meets
            // the empty force.
            force.from_hex = hx(0, 0);
            force.cargo_order = Some(crate::state::CargoOrder {
                action: crate::orders::CargoAction::Load,
                order: OrderId::new(5),
            });
        }
        world.handle_stragglers(1, Pulse::Night);
        assert!(force(&world, 1).ships.is_empty());
        assert_eq!(members(&world, 2), ids(&[1, 2]));
        world.resolve_day().expect("day 1 with an empty force");
        world.resolve_day().expect("day 2 with an empty force");
        // The empty force never moved and the stragglers stayed.
        assert_eq!(force(&world, 1).position(), hx(0, 0));
        assert_eq!(force(&world, 2).position(), hx(0, 0));
        assert_sound(&world);
    }

    #[test]
    fn detach_with_every_ship_able_to_steam_retires_with_nothing_sound() {
        // Ships 1 and 2 degraded: U = 0, so R = 0 and the force retires. No
        // ship is dead, so nothing is left; the parent heads home whole.
        let mut world = world(
            &[(CRUISER, Degraded), (CRUISER, Degraded)],
            StragglerInstruction::Detach,
            Fixed::ZERO,
        );
        world.handle_stragglers(1, Pulse::Night);
        assert_eq!(members(&world, 1), ids(&[1, 2]));
        assert_eq!(world.truth.task_forces.len(), 1);
        assert_goes_home(&world, 1);
        assert_eq!(retired(&world).len(), 1);
        assert!(detached(&world).is_empty());
        world.resolve_day().expect("day 1");
        assert_sound(&world);
    }

    #[test]
    fn detach_with_every_ship_dead_in_the_water_empties_the_parent_and_it_gets_no_order() {
        // Ships 1 and 2 out: U = 0, so R = 0 and the force would retire. Both
        // go to the left-behind force. The parent is then empty: it does not
        // retire and gets no order, because there is nothing to send home.
        let mut world = world(
            &[(CRUISER, Out), (DESTROYER, Out)],
            StragglerInstruction::Detach,
            half(),
        );
        world.handle_stragglers(1, Pulse::Night);
        assert!(force(&world, 1).ships.is_empty());
        assert_eq!(members(&world, 2), ids(&[1, 2]));
        assert!(retired(&world).is_empty());
        assert!(accepted_orders(&world).is_empty());
        assert_stays(&world, 2);
        world.resolve_day().expect("day 1");
        world.resolve_day().expect("day 2");
        assert_eq!(force(&world, 1).position(), hx(6, 0));
        assert_sound(&world);

        // With no home port the empty parent has no order at all.
        let mut no_port = self::world(
            &[(CRUISER, Out), (DESTROYER, Out)],
            StragglerInstruction::Detach,
            half(),
        );
        no_port.truth.task_forces.get_mut(&TF).unwrap().home_port = None;
        no_port.handle_stragglers(1, Pulse::Night);
        assert!(force(&no_port, 1).ships.is_empty());
        assert!(retired(&no_port).is_empty());
        assert_sound(&no_port);
    }

    #[test]
    fn breakdown_recording_does_not_change_what_happens_or_the_hash() {
        let run = |record_breakdowns: bool| {
            let mut world = world(&scenario_ships(), StragglerInstruction::Detach, half());
            world.set_config(SimConfig { record_breakdowns });
            world.resolve_day().expect("day 1");
            world
        };
        let with = run(true);
        let without = run(false);
        assert_eq!(with.state_hash(), without.state_hash());
        assert_eq!(with.day_hashes(), without.day_hashes());
        assert!(detached(&with)
            .iter()
            .all(|(event, ..)| event.breakdown.is_some()));
        assert!(detached(&without)
            .iter()
            .all(|(event, ..)| event.breakdown.is_none()));
        assert_eq!(with.events().len(), without.events().len());
    }

    #[test]
    fn the_scenario_case_steams_as_the_hand_calculation_says() {
        // Spot check of the scenario: the straggler pass runs at the top of
        // the night movement step, so all three forces move in that step.
        // Task force 1 (1, 3, 5) at 16 kn east, 192 nm in the night pulse;
        // task force 2 (2, 4) at 8 kn west, 96 nm.
        let mut world = world(&scenario_ships(), StragglerInstruction::Detach, half());
        {
            let force = world.truth.task_forces.get_mut(&TF).unwrap();
            force.route = crate::state::Route {
                waypoints: vec![hx(12, 0)],
                next_index: 0,
            };
            force.status = TaskForceStatus::Underway;
        }
        world.run_pulse_phase(1, Pulse::Night, PulsePhase::Movement);
        let steamed: Vec<(TaskForceId, Fixed, Fixed)> = world
            .events()
            .iter()
            .filter_map(|event| match &event.kind {
                EventKind::TaskForceSteamed {
                    task_force,
                    distance_nm,
                    speed_kn,
                    ..
                } => Some((*task_force, *distance_nm, *speed_kn)),
                _ => None,
            })
            .collect();
        assert_eq!(
            steamed,
            vec![
                (TF, Fixed::from_int(192), Fixed::from_int(16)),
                (TaskForceId::new(2), Fixed::from_int(96), Fixed::from_int(8)),
            ]
        );
        assert_sound(&world);
    }
}
