//! Air strikes on ships, and the flight deck limit shared with air search
//! (decision `docs/decisions/0026-air-strikes.md`).
//!
//! A daylight pulse has two steps for a strike. In the `StrikeDecisions`
//! step ([`World::decide_strikes`]) each air group on the naval strike
//! mission decides for itself, on its side's belief and nothing else, and
//! launches or does not. In the `AirCombat` step ([`World::resolve_strikes`])
//! every launched strike flies to the hex it was sent to and attacks an
//! enemy task force that is really there, or finds nothing.
//!
//! Both sides' strikes are launched before either lands (decision 8): every
//! decision is made on the state at the start of the decision step, every
//! attack is worked out on the state at the start of the air combat step,
//! and only then is damage applied, in order of air group number. A group
//! aboard a ship that sinks in the pulse still attacks.
//!
//! The attack writes truth only through [`World::apply_hit_set`], the same
//! path a salvo's hits take, and belief only through
//! [`BeliefState::apply_report`](crate::belief::BeliefState::apply_report).

use crate::belief::{ApplyOutcome, Report, SightingReport};
use crate::damage::{damage_per_hit, raw_u64, system_for_draw, DamageSite, HitSet};
use crate::events::{Breakdown, Cause, DamageTrack, EventKind, Factor, Weapon};
use crate::fixed::Fixed;
use crate::hex::{Hex, HEX_NM};
use crate::ids::{AirGroupId, EntityRef, EventId, ShipId, TaskForceId};
use crate::orders::AirMission;
use crate::refdata::{AttackWeapon, ShipKind};
use crate::rng::Subsystem;
use crate::search::build_sighting_report;
use crate::state::{AirGroup, AirGroupLocation, ShipSystem, Side};
use crate::time::{self, Pulse};
use crate::world::World;

/// A strike launched in the decision step and not yet landed. It lives from
/// the decision step to the air combat step of one pulse, and is empty
/// between pulses, so it is not part of the state hash.
#[derive(Clone, Debug)]
pub(crate) struct StrikeInFlight {
    air_group: AirGroupId,
    side: Side,
    /// The hex the contact names: where the strike flies to.
    hex: Hex,
    /// The aircraft launched.
    aircraft: u32,
    /// The `StrikeLaunched` event.
    launched: EventId,
}

/// One strike's attack, worked out and not yet applied.
struct Attack {
    air_group: AirGroupId,
    side: Side,
    task_force: TaskForceId,
    ship: ShipId,
    weapon: Weapon,
    launched: EventId,
    shots: Fixed,
    chance: Fixed,
    expected: Fixed,
    hits: u32,
    damage_per_hit: Fixed,
    /// Hits on flotation and fire, in [`DamageTrack::ALL`] order.
    track_hits: [u32; 2],
    system_hits: Vec<ShipSystem>,
    breakdown: Option<Breakdown>,
    /// The sighting report of the force attacked (decision 12).
    report: SightingReport,
}

/// What the strike found at its hex.
enum Outcome {
    Nothing {
        air_group: AirGroupId,
        hex: Hex,
        launched: EventId,
    },
    Attack(Box<Attack>),
}

impl World {
    /// The aircraft of `group` that can be flown this pulse (decision 14 of
    /// 0026): all its ready aircraft from a base, or from a ship whose flight
    /// deck works; the ready aircraft times the degraded factor, rounded
    /// down, if the deck is degraded; none if it is out.
    pub(crate) fn aircraft_able_to_fly(&self, group: &AirGroup) -> u32 {
        let AirGroupLocation::Ship(ship) = group.location else {
            return group.ready;
        };
        let Some(ship) = self.truth.ships.get(&ship) else {
            return group.ready;
        };
        let state = ship.systems.get(ShipSystem::FlightDeck);
        let Some(figures) = &self.ref_data.surface_combat else {
            // Without the figures no system can be damaged (World::new).
            return group.ready;
        };
        let factor = figures.system_factor(ShipSystem::FlightDeck, state);
        let flown = (Fixed::from_int(i64::from(group.ready)) * factor).trunc();
        u32::try_from(flown).expect("a share of the aircraft is not negative")
    }

    /// The strike decision step of one daylight pulse (decisions 3 to 6 of
    /// 0026). Each group on the naval strike mission, in group order, that
    /// has aircraft, a position, light and a flight deck that lets it launch
    /// at least one aircraft, takes the contact its side holds that is no
    /// older than the figures allow and within the type's strike range: the
    /// most important reported kind first, then the nearest, then the lowest
    /// contact number. It launches every aircraft it can.
    ///
    /// The decision reads the group's own side's belief and the group's own
    /// state. It never reads the enemy.
    pub(crate) fn decide_strikes(&mut self, day: u32, pulse: Pulse) {
        debug_assert!(
            self.strikes_in_flight.is_empty(),
            "the last pulse's strikes landed"
        );
        let Some(figures) = self.ref_data.air_combat.clone() else {
            return;
        };
        let groups: Vec<AirGroup> = self
            .truth
            .air_groups
            .values()
            .filter(|group| group.mission == AirMission::NavalStrike && group.ready > 0)
            .cloned()
            .collect();
        for group in groups {
            let Some(origin) = self.truth.air_group_hex(&group) else {
                continue;
            };
            if time::is_dark(origin, pulse) {
                continue;
            }
            let aircraft_type = &self.ref_data.aircraft_types[&group.aircraft_type];
            if aircraft_type.attack_weapon == AttackWeapon::None {
                continue;
            }
            let strike_range_nm = aircraft_type.strike_range_nm;
            let aircraft = self.aircraft_able_to_fly(&group);
            if aircraft == 0 {
                continue;
            }

            let chosen = self.beliefs[&group.side]
                .contacts()
                .filter(|contact| contact.age_pulses <= figures.strike_contact_max_age_pulses)
                .filter_map(|contact| {
                    let distance_nm =
                        Fixed::from_int(i64::from(origin.distance(contact.estimated_hex)) * HEX_NM);
                    (distance_nm <= strike_range_nm).then_some((
                        contact.estimated_kind,
                        distance_nm,
                        contact.id,
                        contact.estimated_hex,
                        contact.last_report_event,
                    ))
                })
                .min_by_key(|(kind, distance_nm, id, _, _)| (*kind, *distance_nm, *id));
            let Some((_, distance_nm, contact, hex, report_event)) = chosen else {
                continue;
            };

            let causes: Vec<Cause> = group
                .mission_order
                .map(Cause::Order)
                .into_iter()
                .chain([Cause::Event(report_event)])
                .collect();
            let launched = self.events.record(
                day,
                Some(pulse),
                EventKind::StrikeLaunched {
                    air_group: group.id,
                    contact,
                    hex,
                    aircraft,
                    distance_nm,
                },
                causes,
                None,
            );
            self.strikes_in_flight.push(StrikeInFlight {
                air_group: group.id,
                side: group.side,
                hex,
                aircraft,
                launched,
            });
        }
    }

    /// The air combat step of one daylight pulse (decisions 7 to 12 and 16
    /// of 0026): every strike launched in this pulse reaches its hex.
    ///
    /// First every attack is worked out on the state as it stands, and the
    /// attack events are recorded. Then, in order of air group number,
    /// damage is applied and the sighting report is sent.
    pub(crate) fn resolve_strikes(&mut self, day: u32, pulse: Pulse) {
        let strikes = std::mem::take(&mut self.strikes_in_flight);
        if strikes.is_empty() {
            return;
        }

        // Work out every attack on the state at the start of the step.
        let outcomes: Vec<Outcome> = strikes
            .iter()
            .map(|strike| self.work_out_attack(day, pulse, strike))
            .collect();

        // Record what each strike found, in group order.
        let mut attack_events: Vec<Option<EventId>> = Vec::new();
        for outcome in &outcomes {
            match outcome {
                Outcome::Nothing {
                    air_group,
                    hex,
                    launched,
                } => {
                    self.events.record(
                        day,
                        Some(pulse),
                        EventKind::StrikeFoundNothing {
                            air_group: *air_group,
                            hex: *hex,
                        },
                        vec![Cause::Event(*launched)],
                        None,
                    );
                    attack_events.push(None);
                }
                Outcome::Attack(attack) => {
                    let event = self.events.record(
                        day,
                        Some(pulse),
                        EventKind::AirAttack {
                            air_group: attack.air_group,
                            task_force: attack.task_force,
                            ship: attack.ship,
                            weapon: attack.weapon,
                            shots: attack.shots,
                            chance: attack.chance,
                            expected: attack.expected,
                            hits: attack.hits,
                        },
                        vec![Cause::Event(attack.launched)],
                        attack.breakdown.clone(),
                    );
                    attack_events.push(Some(event));
                }
            }
        }

        // Damage lands, then the report goes out, in group order.
        let site = DamageSite {
            day,
            pulse,
            round: 0,
        };
        let search = self
            .ref_data
            .search
            .clone()
            .expect("World::new requires search figures with air groups");
        for (outcome, event) in outcomes.into_iter().zip(attack_events) {
            let Outcome::Attack(attack) = outcome else {
                continue;
            };
            let attack_event = event.expect("an attack was recorded");
            self.apply_hit_set(
                site,
                &HitSet {
                    cause: attack_event,
                    target: attack.ship,
                    target_force: attack.task_force,
                    damage_per_hit: attack.damage_per_hit,
                    track_hits: attack.track_hits,
                    system_hits: attack.system_hits.clone(),
                },
            );

            let mut report = attack.report.clone();
            report.received_event = self.events.next_id();
            let ApplyOutcome::Contact(outcome) = self
                .beliefs
                .get_mut(&attack.side)
                .expect("both sides have a belief state")
                .apply_report(&Report::Sighting(report.clone()), &search)
            else {
                unreachable!("a sighting report makes or refreshes a contact");
            };
            let received = self.events.record(
                day,
                Some(pulse),
                EventKind::ReportReceived {
                    side: attack.side,
                    contact: outcome.contact,
                    hex: report.estimated_hex,
                    ship_count: report.estimated_ship_count,
                    kind: report.estimated_kind,
                    confidence: report.confidence,
                    created: outcome.created,
                },
                vec![Cause::Event(attack_event)],
                None,
            );
            assert_eq!(
                received, report.received_event,
                "the report cites the event that records it"
            );
        }
    }

    /// Finds what one strike meets at its hex and rolls the attack. Reads
    /// truth, which the umpire may; writes nothing. Draws, from one stream
    /// keyed on the air group and the task force attacked: the fraction of
    /// the expected hits, then for each hit its track (a bomb only: a torpedo
    /// always hits flotation) and then its system, then the sighting report.
    fn work_out_attack(&self, day: u32, pulse: Pulse, strike: &StrikeInFlight) -> Outcome {
        // The most important ship of each enemy force really in the hex.
        let target = self
            .truth
            .task_forces
            .values()
            .filter(|force| force.side != strike.side && force.position() == strike.hex)
            .filter_map(|force| {
                let ship = force
                    .ships
                    .iter()
                    .map(|ship| (self.kind_of(*ship), *ship))
                    .min()?;
                Some((ship.0, force.id, ship.1))
            })
            .min();
        let Some((kind, task_force, ship)) = target else {
            return Outcome::Nothing {
                air_group: strike.air_group,
                hex: strike.hex,
                launched: strike.launched,
            };
        };

        let air = self
            .ref_data
            .air_combat
            .as_ref()
            .expect("a strike needs the air combat figures");
        let surface = self
            .ref_data
            .surface_combat
            .as_ref()
            .expect("World::new requires surface combat figures");
        let search = self
            .ref_data
            .search
            .as_ref()
            .expect("World::new requires search figures with air groups");
        let group = &self.truth.air_groups[&strike.air_group];
        let aircraft_type = &self.ref_data.aircraft_types[&group.aircraft_type];
        let toughness = self.ref_data.ship_classes[&self.truth.ships[&ship].class].toughness;
        let (weapon, chance, system_chance, system_table) = match aircraft_type.attack_weapon {
            AttackWeapon::Bomb => (
                Weapon::Bomb,
                air.bomb_hit_chance,
                air.bomb_system_hit_chance,
                &air.bomb_system_hit,
            ),
            AttackWeapon::Torpedo => (
                Weapon::Torpedo,
                air.torpedo_hit_chance,
                surface.torpedo_system_hit_chance,
                &surface.torpedo_system_hit,
            ),
            AttackWeapon::None => unreachable!("only groups that attack with something launch"),
        };

        let shots = Fixed::from_int(i64::from(strike.aircraft));
        let expected = shots * chance;
        let whole = expected.trunc();
        let fraction = expected - Fixed::from_int(whole);
        let mut stream = self.rng_pair(
            day,
            Some(pulse),
            Subsystem::AirCombat,
            EntityRef::AirGroup(strike.air_group),
            EntityRef::TaskForce(task_force),
        );
        let hits = u32::try_from(whole).expect("expected hits are not negative")
            + u32::from(stream.chance(fraction));

        let mut track_hits = [0u32; 2];
        let mut system_hits = Vec::new();
        for _ in 0..hits {
            let track = if weapon == Weapon::Bomb {
                let draw = stream.below(Fixed::SCALE.unsigned_abs());
                if draw < raw_u64(surface.gun_hit_flotation) {
                    DamageTrack::Flotation
                } else {
                    DamageTrack::Fire
                }
            } else {
                DamageTrack::Flotation
            };
            track_hits[track as usize] += 1;
            // One draw for the system, always, even if the chance is 0.
            let draw = stream.below(Fixed::SCALE.unsigned_abs());
            let draw = Fixed::from_raw(i64::try_from(draw).expect("a draw fits in i64"));
            system_hits.extend(system_for_draw(draw, system_chance, system_table));
        }

        let ship_count =
            u32::try_from(self.truth.task_forces[&task_force].ships.len()).expect("few ships");
        let report = build_sighting_report(
            &mut stream,
            &self.map,
            strike.hex,
            ship_count,
            kind,
            search,
            // Replaced with the event that records the report's arrival.
            EventId::new(0),
        );

        let breakdown = self.config.record_breakdowns.then(|| Breakdown {
            factors: vec![
                Factor {
                    name: "aircraft",
                    value: shots,
                },
                Factor {
                    name: "hit_chance",
                    value: chance,
                },
            ],
            result: expected,
        });
        Outcome::Attack(Box::new(Attack {
            air_group: strike.air_group,
            side: strike.side,
            task_force,
            ship,
            weapon,
            launched: strike.launched,
            shots,
            chance,
            expected,
            hits,
            damage_per_hit: damage_per_hit(aircraft_type.attack_damage, toughness),
            track_hits,
            system_hits,
            breakdown,
            report,
        }))
    }

    fn kind_of(&self, ship: ShipId) -> ShipKind {
        self.ref_data.ship_classes[&self.truth.ships[&ship].class].kind
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::events::Event;
    use crate::ids::{AircraftTypeId, OrderId, ShipClassId};
    use crate::map::{HexData, Map};
    use crate::orders::{AirGroupOrder, Mission, OrderKind, OrderRequest, Origin, RejectReason};
    use crate::refdata::{
        AirCombatFigures, AircraftRole, AircraftType, PortSize, RefData, SearchFigures, ShipClass,
        SurfaceCombatFigures,
    };
    use crate::start::StartingContact;
    use crate::state::{
        AirGroup, Ship, ShipLocation, ShipSystems, SystemState, TaskForce, TruthState,
    };

    // ---- a fixture ----
    //
    // Open sea from q -12 to 12 and r -6 to 6. Classes: 1 a cruiser (toughness
    // 400), 2 a carrier (toughness 600, carries 36 aircraft). Types: 1 a scout
    // (search 300 nm, sweep 20), 2 a dive bomber (bomb, 60 damage, strikes to
    // 250 nm), 3 a torpedo bomber (torpedo, 200 damage, 250 nm), 4 a bomber
    // that attacks with nothing.
    //
    // Allied task force 1 at (0,0): carrier ship 1, with air group 1, 18 dive
    // bombers on the naval strike mission, and air group 2, 8 scouts at rest.
    // Japanese task force 2 at (5,0): carrier ship 2, cruisers ship 3 and
    // ship 4, with air group 3, 18 dive bombers at rest.

    const FORCE_A: TaskForceId = TaskForceId::new(1);
    const FORCE_B: TaskForceId = TaskForceId::new(2);
    const BOMBERS: AirGroupId = AirGroupId::new(1);
    const SCOUTS: AirGroupId = AirGroupId::new(2);
    const ENEMY_BOMBERS: AirGroupId = AirGroupId::new(3);

    fn fixed(num: i64, den: i64) -> Fixed {
        Fixed::from_ratio(num, den)
    }

    fn class(id: u32, kind: ShipKind, toughness: i64, capacity: u32) -> ShipClass {
        ShipClass {
            id: ShipClassId::new(id),
            name: format!("Test class {id}"),
            kind,
            cruise_speed_kn: Fixed::from_int(16),
            max_speed_kn: Fixed::from_int(32),
            fuel_capacity_t: Fixed::from_int(2000),
            endurance_nm: Fixed::from_int(8000),
            cargo_capacity_t: Fixed::ZERO,
            main_guns: if capacity == 0 { 8 } else { 0 },
            gun_rounds_per_min: if capacity == 0 {
                Fixed::from_int(3)
            } else {
                Fixed::ZERO
            },
            gun_rounds_per_gun: if capacity == 0 { 100 } else { 0 },
            gun_damage: if capacity == 0 {
                Fixed::from_int(20)
            } else {
                Fixed::ZERO
            },
            torpedo_tubes: 0,
            torpedo_damage: Fixed::ZERO,
            toughness: Fixed::from_int(toughness),
            radar: false,
            aircraft_capacity: capacity,
        }
    }

    fn aircraft(id: u32, role: AircraftRole, weapon: AttackWeapon, damage: i64) -> AircraftType {
        let scout = role == AircraftRole::Patrol;
        AircraftType {
            id: AircraftTypeId::new(id),
            name: format!("Test type {id}"),
            role,
            search_range_nm: Fixed::from_int(if scout { 300 } else { 0 }),
            sweep_width_nm: Fixed::from_int(if scout { 20 } else { 0 }),
            strike_range_nm: Fixed::from_int(if weapon == AttackWeapon::None { 0 } else { 250 }),
            attack_weapon: weapon,
            attack_damage: Fixed::from_int(damage),
        }
    }

    fn group(id: AirGroupId, side: Side, ship: u32, type_id: u32, ready: u32) -> AirGroup {
        AirGroup {
            id,
            name: format!("Test group {}", id.raw()),
            side,
            location: AirGroupLocation::Ship(ShipId::new(ship)),
            aircraft_type: AircraftTypeId::new(type_id),
            ready,
            mission: AirMission::Rest,
            search: None,
            mission_order: None,
            arc_order: None,
        }
    }

    fn ship(id: u32, class: u32, side: Side, force: TaskForceId) -> Ship {
        Ship {
            id: ShipId::new(id),
            name: format!("Test ship {id}"),
            class: ShipClassId::new(class),
            side,
            fuel_t: Fixed::from_int(2000),
            supply_t: Fixed::ZERO,
            location: ShipLocation::TaskForce(force),
            flotation_damage: Fixed::ZERO,
            fire_damage: Fixed::ZERO,
            gun_rounds: Fixed::ZERO,
            torpedoes_ready: false,
            systems: ShipSystems::ALL_WORKING,
        }
    }

    fn held(hex: Hex, kind: ShipKind, age_pulses: u32) -> StartingContact {
        StartingContact {
            side: Side::Allied,
            hex,
            ship_count: 3,
            kind,
            age_pulses,
        }
    }

    /// The standard world with the Allied side holding `contacts`.
    fn world(seed: u64, contacts: Vec<StartingContact>) -> World {
        let mut map = Map::new();
        for q in -12..=12 {
            for r in -6..=6 {
                map.insert(Hex::new(q, r), HexData::open_sea());
            }
        }
        let mut ref_data = RefData::default();
        ref_data
            .ship_classes
            .insert(ShipClassId::new(1), class(1, ShipKind::Cruiser, 400, 0));
        ref_data
            .ship_classes
            .insert(ShipClassId::new(2), class(2, ShipKind::Carrier, 600, 36));
        ref_data.port_sizes.insert(
            0,
            PortSize {
                berths: 0,
                berth_rate_t_per_h: Fixed::ZERO,
                lighter_rate_t_per_h: Fixed::from_int(20),
            },
        );
        for aircraft_type in [
            aircraft(1, AircraftRole::Patrol, AttackWeapon::None, 0),
            aircraft(2, AircraftRole::Bomber, AttackWeapon::Bomb, 60),
            aircraft(3, AircraftRole::Bomber, AttackWeapon::Torpedo, 200),
            aircraft(4, AircraftRole::Bomber, AttackWeapon::None, 0),
        ] {
            ref_data
                .aircraft_types
                .insert(aircraft_type.id, aircraft_type);
        }
        ref_data.search = Some(SearchFigures::from_search_toml());
        ref_data.surface_combat = Some(SurfaceCombatFigures::from_surface_combat_toml());
        ref_data.air_combat = Some(AirCombatFigures::from_air_combat_toml());

        let mut truth = TruthState::default();
        for (id, side, hex, ships) in [
            (FORCE_A, Side::Allied, Hex::new(0, 0), vec![(1, 2)]),
            (
                FORCE_B,
                Side::Japanese,
                Hex::new(5, 0),
                vec![(2, 2), (3, 1), (4, 1)],
            ),
        ] {
            let mut force = TaskForce::at(
                id,
                format!("Test force {}", id.raw()),
                side,
                hex,
                Mission::Carrier,
            );
            for (ship_id, class_id) in ships {
                force.ships.insert(ShipId::new(ship_id));
                truth
                    .ships
                    .insert(ShipId::new(ship_id), ship(ship_id, class_id, side, id));
            }
            truth.task_forces.insert(id, force);
        }
        let mut bombers = group(BOMBERS, Side::Allied, 1, 2, 18);
        bombers.mission = AirMission::NavalStrike;
        for group in [
            bombers,
            group(SCOUTS, Side::Allied, 1, 1, 8),
            group(ENEMY_BOMBERS, Side::Japanese, 2, 2, 18),
        ] {
            truth.air_groups.insert(group.id, group);
        }
        let mut world = World::new(seed, ref_data, map, truth).expect("valid setup");
        world
            .hold_starting_contacts(contacts)
            .expect("the contacts are held");
        world
    }

    /// The Allied side holds a fresh cruiser contact at the enemy's true hex.
    fn world_with_a_good_contact(seed: u64) -> World {
        world(seed, vec![held(Hex::new(5, 0), ShipKind::Cruiser, 0)])
    }

    fn air_figures(world: &mut World) -> &mut AirCombatFigures {
        world.ref_data.air_combat.as_mut().expect("air figures")
    }

    fn surface_figures(world: &mut World) -> &mut SurfaceCombatFigures {
        world
            .ref_data
            .surface_combat
            .as_mut()
            .expect("surface figures")
    }

    fn group_mut(world: &mut World, id: AirGroupId) -> &mut AirGroup {
        world.truth.air_groups.get_mut(&id).expect("group")
    }

    fn ship_of(world: &World, id: u32) -> &Ship {
        &world.truth.ships[&ShipId::new(id)]
    }

    fn ship_mut(world: &mut World, id: u32) -> &mut Ship {
        world.truth.ships.get_mut(&ShipId::new(id)).expect("ship")
    }

    fn strike_pulse(world: &mut World, pulse: Pulse) {
        world.decide_strikes(1, pulse);
        world.resolve_strikes(1, pulse);
    }

    fn events_where(world: &World, keep: impl Fn(&EventKind) -> bool) -> Vec<Event> {
        world
            .events()
            .iter()
            .filter(|event| keep(&event.kind))
            .cloned()
            .collect()
    }

    fn launches(world: &World) -> Vec<Event> {
        events_where(world, |k| matches!(k, EventKind::StrikeLaunched { .. }))
    }

    fn attacks(world: &World) -> Vec<Event> {
        events_where(world, |k| matches!(k, EventKind::AirAttack { .. }))
    }

    fn found_nothing(world: &World) -> Vec<Event> {
        events_where(world, |k| matches!(k, EventKind::StrikeFoundNothing { .. }))
    }

    fn system_hits(world: &World) -> Vec<(u32, ShipSystem, SystemState, SystemState)> {
        events_where(world, |k| matches!(k, EventKind::SystemHit { .. }))
            .iter()
            .map(|event| match event.kind {
                EventKind::SystemHit {
                    ship,
                    system,
                    before,
                    after,
                    ..
                } => (ship.raw(), system, before, after),
                _ => unreachable!(),
            })
            .collect()
    }

    fn launch_of(event: &Event) -> (u32, u32, Hex, u32, Fixed) {
        match event.kind {
            EventKind::StrikeLaunched {
                air_group,
                contact,
                hex,
                aircraft,
                distance_nm,
            } => (air_group.raw(), contact.raw(), hex, aircraft, distance_nm),
            _ => panic!("not a launch"),
        }
    }

    fn attack_of(event: &Event) -> (u32, u32, u32, Weapon, Fixed, Fixed, Fixed, u32) {
        match event.kind {
            EventKind::AirAttack {
                air_group,
                task_force,
                ship,
                weapon,
                shots,
                chance,
                expected,
                hits,
            } => (
                air_group.raw(),
                task_force.raw(),
                ship.raw(),
                weapon,
                shots,
                chance,
                expected,
                hits,
            ),
            _ => panic!("not an attack"),
        }
    }

    // ---- the decision ----

    #[test]
    fn with_no_contact_a_group_launches_nothing_and_records_nothing() {
        let mut world = world(1, vec![]);
        let before = world.events().len();
        strike_pulse(&mut world, Pulse::Morning);
        assert_eq!(world.events().len(), before);
        assert!(world.strikes_in_flight.is_empty());
    }

    #[test]
    fn a_contact_launches_every_aircraft_the_group_can_fly_citing_the_order_and_the_report() {
        let mut world = world_with_a_good_contact(1);
        let order = OrderId::new(7);
        group_mut(&mut world, BOMBERS).mission_order = Some(order);
        let report = world
            .beliefs
            .get(&Side::Allied)
            .expect("belief")
            .contacts()
            .next()
            .expect("a contact")
            .last_report_event;
        world.decide_strikes(1, Pulse::Morning);
        let all = launches(&world);
        assert_eq!(all.len(), 1);
        assert_eq!(
            launch_of(&all[0]),
            (1, 1, Hex::new(5, 0), 18, Fixed::from_int(200))
        );
        assert_eq!(
            all[0].causes,
            vec![Cause::Order(order), Cause::Event(report)]
        );
        assert_eq!(all[0].pulse, Some(Pulse::Morning));
        assert_eq!(
            all[0].kind.to_string(),
            "air_group:1 launched 18 aircraft at contact:1 at (5,0), 200 nm"
        );
        // Nothing lands until the air combat step.
        assert!(attacks(&world).is_empty());
        assert_eq!(world.strikes_in_flight.len(), 1);
    }

    #[test]
    fn a_group_that_does_not_strike_decides_nothing() {
        type Change = fn(&mut World);
        let setups: [(&str, Change); 5] = [
            ("resting", |w| {
                group_mut(w, BOMBERS).mission = AirMission::Rest
            }),
            ("searching", |w| {
                group_mut(w, BOMBERS).mission = AirMission::Search;
            }),
            ("no aircraft", |w| group_mut(w, BOMBERS).ready = 0),
            ("no position", |w| {
                let ship = ship_mut(w, 1);
                ship.location = ShipLocation::Sunk;
                w.truth
                    .task_forces
                    .get_mut(&FORCE_A)
                    .expect("force")
                    .ships
                    .remove(&ShipId::new(1));
            }),
            ("attacks with nothing", |w| {
                group_mut(w, BOMBERS).aircraft_type = AircraftTypeId::new(4);
            }),
        ];
        for (label, change) in setups {
            let mut world = world_with_a_good_contact(1);
            change(&mut world);
            let before = world.events().len();
            strike_pulse(&mut world, Pulse::Morning);
            assert_eq!(world.events().len(), before, "{label}");
        }
    }

    #[test]
    fn there_is_no_strike_decision_in_the_dark() {
        let mut world = world_with_a_good_contact(1);
        let before = world.events().len();
        strike_pulse(&mut world, Pulse::Night);
        assert_eq!(world.events().len(), before);
    }

    #[test]
    fn a_contact_older_than_the_figure_is_not_struck_and_one_exactly_that_old_is() {
        // The figure is one pulse.
        for (age, launches_at) in [(0, true), (1, true), (2, false), (3, false)] {
            let mut world = world(1, vec![held(Hex::new(5, 0), ShipKind::Cruiser, age)]);
            world.decide_strikes(1, Pulse::Morning);
            assert_eq!(
                launches(&world).len(),
                usize::from(launches_at),
                "age {age}"
            );
        }
        let mut world = world(1, vec![held(Hex::new(5, 0), ShipKind::Cruiser, 2)]);
        air_figures(&mut world).strike_contact_max_age_pulses = 2;
        world.decide_strikes(1, Pulse::Morning);
        assert_eq!(launches(&world).len(), 1);
    }

    #[test]
    fn a_contact_beyond_the_strike_range_is_not_struck_and_one_at_the_range_is() {
        // Six hexes is 240 nm and seven is 280; the type reaches 250.
        for (hex, launches_at) in [(Hex::new(6, 0), true), (Hex::new(7, 0), false)] {
            let mut world = world(1, vec![held(hex, ShipKind::Cruiser, 0)]);
            world.decide_strikes(1, Pulse::Morning);
            assert_eq!(launches(&world).len(), usize::from(launches_at), "{hex}");
        }
        // The range is compared exactly: 240 nm reaches six hexes, 239 does not.
        for (range_nm, launches_at) in [(240, true), (239, false)] {
            let mut world = self::world(1, vec![held(Hex::new(6, 0), ShipKind::Cruiser, 0)]);
            world
                .ref_data
                .aircraft_types
                .get_mut(&AircraftTypeId::new(2))
                .expect("type")
                .strike_range_nm = Fixed::from_int(range_nm);
            world.decide_strikes(1, Pulse::Morning);
            assert_eq!(
                launches(&world).len(),
                usize::from(launches_at),
                "{range_nm}"
            );
        }
    }

    #[test]
    fn it_chooses_the_most_important_kind_then_the_nearest_then_the_lowest_contact() {
        // Contacts are made in the order given, so ids 1, 2, 3, 4. They are
        // more than two hexes apart, so none merges with another.
        let chosen = |contacts: Vec<StartingContact>| {
            let mut world = world(1, contacts);
            world.decide_strikes(1, Pulse::Morning);
            let all = launches(&world);
            assert_eq!(all.len(), 1);
            launch_of(&all[0])
        };
        // The carrier is the most important, though the farthest.
        let (_, contact, hex, _, _) = chosen(vec![
            held(Hex::new(3, 0), ShipKind::Cruiser, 0),
            held(Hex::new(-4, 0), ShipKind::Destroyer, 0),
            held(Hex::new(0, 6), ShipKind::Carrier, 0),
        ]);
        assert_eq!((contact, hex), (3, Hex::new(0, 6)));
        // Of equal kind the nearest: 4 hexes against 3 and 6.
        let (_, contact, hex, _, distance) = chosen(vec![
            held(Hex::new(4, 0), ShipKind::Cruiser, 0),
            held(Hex::new(-3, 0), ShipKind::Cruiser, 0),
            held(Hex::new(0, 6), ShipKind::Cruiser, 0),
        ]);
        assert_eq!((contact, hex), (2, Hex::new(-3, 0)));
        assert_eq!(distance, Fixed::from_int(120));
        // Of equal kind and distance the lowest contact number.
        let (_, contact, hex, _, _) = chosen(vec![
            held(Hex::new(4, 0), ShipKind::Cruiser, 0),
            held(Hex::new(-4, 0), ShipKind::Cruiser, 0),
        ]);
        assert_eq!((contact, hex), (1, Hex::new(4, 0)));
        // A contact out of range does not count however important it is.
        let (_, contact, _, _, _) = chosen(vec![
            held(Hex::new(9, 0), ShipKind::Carrier, 0),
            held(Hex::new(4, 0), ShipKind::Destroyer, 0),
        ]);
        assert_eq!(contact, 2);
    }

    #[test]
    fn the_strike_flies_to_the_hex_the_contact_names_not_to_where_the_enemy_is() {
        // The enemy is at (5,0). The contact is a hex out of place.
        let mut world = world(1, vec![held(Hex::new(4, 0), ShipKind::Carrier, 0)]);
        strike_pulse(&mut world, Pulse::Morning);
        assert_eq!(launch_of(&launches(&world)[0]).2, Hex::new(4, 0));
        assert!(attacks(&world).is_empty());
        let nothing = found_nothing(&world);
        assert_eq!(nothing.len(), 1);
        assert_eq!(
            nothing[0].kind.to_string(),
            "air_group:1 found nothing at (4,0)"
        );
        let launch = launches(&world)[0].id;
        assert_eq!(nothing[0].causes, vec![Cause::Event(launch)]);
        // Nothing is hit and the side hears nothing more.
        for id in 1..=4 {
            assert_eq!(ship_of(&world, id).flotation_damage, Fixed::ZERO);
            assert_eq!(ship_of(&world, id).fire_damage, Fixed::ZERO);
        }
        assert!(
            events_where(&world, |k| matches!(k, EventKind::ReportReceived { .. }))
                .iter()
                .all(|event| event.day == 0)
        );
        assert_eq!(group_mut(&mut world, BOMBERS).ready, 18);
    }

    #[test]
    fn a_strike_never_reads_where_the_enemy_really_is() {
        // Moving the enemy changes what the attack finds and nothing about
        // the decision.
        let launched = |enemy_at: Hex| {
            let mut world = world_with_a_good_contact(1);
            world
                .truth
                .task_forces
                .get_mut(&FORCE_B)
                .expect("force")
                .from_hex = enemy_at;
            world.decide_strikes(1, Pulse::Morning);
            launches(&world)
                .iter()
                .map(|e| e.kind.clone())
                .collect::<Vec<_>>()
        };
        assert_eq!(launched(Hex::new(5, 0)), launched(Hex::new(-8, 3)));
    }

    // ---- the attack ----

    /// Lets every bomb hit land on the flight deck: all hits are system
    /// hits, all on the flight deck. Makes the flight deck the only thing a
    /// strike can change besides the two damage tracks.
    fn every_hit_is_a_flight_deck_hit(world: &mut World) {
        let air = air_figures(world);
        air.bomb_system_hit_chance = Fixed::ONE;
        air.bomb_system_hit = [(ShipSystem::FlightDeck, Fixed::ONE)].into_iter().collect();
    }

    /// Adds a task force with the given ships, each `(id, class)`.
    fn add_force(world: &mut World, id: u32, side: Side, hex: Hex, ships: &[(u32, u32)]) {
        let id = TaskForceId::new(id);
        let mut force = TaskForce::at(
            id,
            format!("Added {}", id.raw()),
            side,
            hex,
            Mission::Carrier,
        );
        for (ship_id, class_id) in ships {
            force.ships.insert(ShipId::new(*ship_id));
            world
                .truth
                .ships
                .insert(ShipId::new(*ship_id), ship(*ship_id, *class_id, side, id));
        }
        world.truth.task_forces.insert(id, force);
    }

    fn set_class(world: &mut World, ship_id: u32, class_id: u32) {
        ship_mut(world, ship_id).class = ShipClassId::new(class_id);
    }

    #[test]
    fn a_strike_with_no_enemy_force_in_the_hex_finds_nothing_even_if_ours_is_there() {
        // An Allied force in the hex is not an enemy.
        let mut world = world(1, vec![held(Hex::new(0, 0), ShipKind::Cruiser, 0)]);
        strike_pulse(&mut world, Pulse::Morning);
        assert_eq!(found_nothing(&world).len(), 1);
        assert!(attacks(&world).is_empty());
    }

    #[test]
    fn it_attacks_the_force_with_the_most_important_ship_then_the_lowest_number() {
        let target_of = |build: fn(&mut World)| {
            let mut world = world_with_a_good_contact(1);
            build(&mut world);
            strike_pulse(&mut world, Pulse::Morning);
            let all = attacks(&world);
            assert_eq!(all.len(), 1);
            let (_, force, ship, ..) = attack_of(&all[0]);
            (force, ship)
        };
        // Force 2 holds a carrier (ship 2) and two cruisers. Force 3 is
        // cruisers only, so it is not taken though it is in the same hex.
        assert_eq!(
            target_of(|w| add_force(w, 3, Side::Japanese, Hex::new(5, 0), &[(5, 1)])),
            (2, 2)
        );
        // Force 2 has no carrier now and force 3 has: force 3.
        assert_eq!(
            target_of(|w| {
                set_class(w, 2, 1);
                add_force(w, 3, Side::Japanese, Hex::new(5, 0), &[(5, 1), (6, 2)]);
            }),
            (3, 6)
        );
        // Two forces each with a carrier: the lower number.
        assert_eq!(
            target_of(|w| add_force(w, 3, Side::Japanese, Hex::new(5, 0), &[(5, 2)])),
            (2, 2)
        );
        // A force of another hex is not there.
        assert_eq!(
            target_of(|w| add_force(w, 3, Side::Japanese, Hex::new(4, 0), &[(5, 2)])),
            (2, 2)
        );
        // Within the force the most important ship, the lowest number on a
        // tie: ship 2 is made a cruiser, so the carrier is ship 8, not the
        // first ship; with two carriers the lower one.
        assert_eq!(
            target_of(|w| {
                set_class(w, 2, 1);
                w.truth
                    .task_forces
                    .get_mut(&FORCE_B)
                    .expect("force")
                    .ships
                    .insert(ShipId::new(8));
                w.truth
                    .ships
                    .insert(ShipId::new(8), ship(8, 2, Side::Japanese, FORCE_B));
            }),
            (2, 8)
        );
        assert_eq!(
            target_of(|w| {
                w.truth
                    .task_forces
                    .get_mut(&FORCE_B)
                    .expect("force")
                    .ships
                    .insert(ShipId::new(8));
                w.truth
                    .ships
                    .insert(ShipId::new(8), ship(8, 2, Side::Japanese, FORCE_B));
            }),
            (2, 2)
        );
        // Another Allied force is never the target.
        assert_eq!(
            target_of(|w| add_force(w, 0, Side::Allied, Hex::new(5, 0), &[(9, 2)])),
            (2, 2)
        );
    }

    #[test]
    fn hits_are_the_whole_part_of_the_expectation_or_one_more() {
        // 18 shots at 0.2 expect 3.6 hits: 3 or 4, over many seeds both.
        let mut seen = std::collections::BTreeSet::new();
        for seed in 1..=60 {
            let mut world = world_with_a_good_contact(seed);
            strike_pulse(&mut world, Pulse::Morning);
            let (_, _, _, weapon, shots, chance, expected, hits) = attack_of(&attacks(&world)[0]);
            assert_eq!(weapon, Weapon::Bomb);
            assert_eq!(shots, Fixed::from_int(18));
            assert_eq!(chance, fixed(2, 10));
            assert_eq!(expected, fixed(36, 10));
            assert!(hits == 3 || hits == 4, "{hits}");
            seen.insert(hits);
        }
        assert_eq!(seen.len(), 2);
        // A whole expectation never gets the one more: 18 x 0.5 is 9.
        for seed in 1..=30 {
            let mut world = world_with_a_good_contact(seed);
            air_figures(&mut world).bomb_hit_chance = fixed(1, 2);
            strike_pulse(&mut world, Pulse::Morning);
            assert_eq!(attack_of(&attacks(&world)[0]).7, 9);
        }
        // Certain hits and no hits.
        for (chance, hits) in [(Fixed::ONE, 18), (Fixed::ZERO, 0)] {
            let mut world = world_with_a_good_contact(1);
            air_figures(&mut world).bomb_hit_chance = chance;
            strike_pulse(&mut world, Pulse::Morning);
            assert_eq!(attack_of(&attacks(&world)[0]).7, hits);
        }
    }

    #[test]
    fn the_attack_event_reads_as_the_decision_says_and_cites_the_launch_with_a_breakdown() {
        let mut world = world_with_a_good_contact(3);
        strike_pulse(&mut world, Pulse::Morning);
        let attack = attacks(&world)[0].clone();
        let (_, _, _, _, _, _, _, hits) = attack_of(&attack);
        assert_eq!(
            attack.kind.to_string(),
            format!(
                "air_group:1 bombs at ship:2: 18 shots x 0.2 = 3.6 expected, {hits} {}",
                if hits == 1 { "hit" } else { "hits" }
            )
        );
        assert_eq!(attack.causes, vec![Cause::Event(launches(&world)[0].id)]);
        let breakdown = attack.breakdown.expect("breakdowns are on");
        assert_eq!(breakdown.result, fixed(36, 10));
        let factors: Vec<(&str, Fixed)> = breakdown
            .factors
            .iter()
            .map(|factor| (factor.name, factor.value))
            .collect();
        assert_eq!(
            factors,
            vec![
                ("aircraft", Fixed::from_int(18)),
                ("hit_chance", fixed(2, 10))
            ]
        );
    }

    #[test]
    fn a_bomb_hit_does_what_a_gun_hit_of_the_same_damage_does() {
        // 100 x 60 / 600 = 10 points a hit, on flotation or fire, and the
        // events have the form of a salvo's damage without a round.
        for seed in 1..=40 {
            let mut world = world_with_a_good_contact(seed);
            strike_pulse(&mut world, Pulse::Morning);
            let attack = attacks(&world)[0].clone();
            let hits = attack_of(&attack).7;
            let damaged = events_where(&world, |k| matches!(k, EventKind::ShipDamaged { .. }));
            let mut total_hits = 0;
            for event in &damaged {
                let EventKind::ShipDamaged {
                    round,
                    ship,
                    track,
                    hits,
                    points,
                    value_after,
                } = event.kind
                else {
                    unreachable!()
                };
                assert_eq!((round, ship.raw()), (0, 2));
                assert_eq!(points, Fixed::from_int(10) * i64::from(hits));
                assert_eq!(
                    value_after,
                    match track {
                        DamageTrack::Flotation => ship_of(&world, 2).flotation_damage,
                        DamageTrack::Fire => ship_of(&world, 2).fire_damage,
                    }
                );
                assert_eq!(event.causes, vec![Cause::Event(attack.id)]);
                assert!(event.kind.to_string().starts_with("ship:2 took "));
                total_hits += hits;
            }
            assert_eq!(total_hits, hits);
            assert_eq!(
                ship_of(&world, 2).flotation_damage + ship_of(&world, 2).fire_damage,
                Fixed::from_int(10) * i64::from(hits)
            );
            // Nothing else is damaged.
            for id in [1, 3, 4] {
                assert_eq!(ship_of(&world, id).flotation_damage, Fixed::ZERO);
                assert_eq!(ship_of(&world, id).fire_damage, Fixed::ZERO);
            }
            // The aircraft all come home.
            assert_eq!(group_mut(&mut world, BOMBERS).ready, 18);
        }
        assert_eq!(
            damage_per_hit(Fixed::from_int(60), Fixed::from_int(600)),
            Fixed::from_int(10)
        );
    }

    #[test]
    fn a_torpedo_from_the_air_hits_flotation_and_uses_the_torpedo_system_table() {
        // Torpedo bombers, certain hits on the force's cruiser (ship 2 is a
        // cruiser now): 9 hits of 100 x 200 / 400 = 50 points would sink it,
        // so the chance is 0.5 of 18 = 9 hits only to count them: set the
        // damage low enough that it survives, 100 x 8 / 400 = 2 points.
        let mut world = world_with_a_good_contact(1);
        set_class(&mut world, 2, 1);
        group_mut(&mut world, BOMBERS).aircraft_type = AircraftTypeId::new(3);
        world
            .ref_data
            .aircraft_types
            .get_mut(&AircraftTypeId::new(3))
            .expect("type")
            .attack_damage = Fixed::from_int(8);
        air_figures(&mut world).torpedo_hit_chance = fixed(1, 2);
        {
            let surface = surface_figures(&mut world);
            surface.torpedo_system_hit_chance = Fixed::ONE;
            surface.torpedo_system_hit = [(ShipSystem::FireControl, Fixed::ONE)]
                .into_iter()
                .collect();
        }
        strike_pulse(&mut world, Pulse::Morning);
        let (_, _, ship, weapon, shots, chance, expected, hits) = attack_of(&attacks(&world)[0]);
        assert_eq!(
            (ship, weapon, shots, chance, expected, hits),
            (
                2,
                Weapon::Torpedo,
                Fixed::from_int(18),
                fixed(1, 2),
                Fixed::from_int(9),
                9
            )
        );
        assert!(attacks(&world)[0]
            .kind
            .to_string()
            .starts_with("air_group:1 torpedoes at ship:2: 18 shots x 0.5 = 9 expected, 9 hits"));
        // All on flotation: 9 x 2 points, no fire.
        assert_eq!(ship_of(&world, 2).flotation_damage, Fixed::from_int(18));
        assert_eq!(ship_of(&world, 2).fire_damage, Fixed::ZERO);
        // Nine system hits on fire control: working -> degraded -> out, and
        // then seven that change nothing and say so.
        let hits = system_hits(&world);
        assert_eq!(hits.len(), 9);
        assert_eq!(
            hits[0],
            (
                2,
                ShipSystem::FireControl,
                SystemState::Working,
                SystemState::Degraded
            )
        );
        assert_eq!(
            hits[1],
            (
                2,
                ShipSystem::FireControl,
                SystemState::Degraded,
                SystemState::Out
            )
        );
        for hit in &hits[2..] {
            assert_eq!(
                *hit,
                (
                    2,
                    ShipSystem::FireControl,
                    SystemState::Out,
                    SystemState::Out
                )
            );
        }
        let texts: Vec<String> = events_where(&world, |k| matches!(k, EventKind::SystemHit { .. }))
            .iter()
            .map(|e| e.kind.to_string())
            .collect();
        assert_eq!(texts[0], "ship:2 fire control hit, working -> degraded");
        assert_eq!(texts[8], "ship:2 fire control hit, no change, still out");
        assert_eq!(ship_of(&world, 2).systems.fire_control, SystemState::Out);
    }

    #[test]
    fn a_ship_that_reaches_100_flotation_sinks_with_its_air_groups_and_leaves_its_force() {
        // Torpedoes of 600 damage on the carrier (toughness 600) are 100
        // points a hit: one hit sinks it.
        let mut world = world_with_a_good_contact(1);
        group_mut(&mut world, BOMBERS).aircraft_type = AircraftTypeId::new(3);
        world
            .ref_data
            .aircraft_types
            .get_mut(&AircraftTypeId::new(3))
            .expect("type")
            .attack_damage = Fixed::from_int(600);
        air_figures(&mut world).torpedo_hit_chance = Fixed::ONE;
        strike_pulse(&mut world, Pulse::Morning);
        assert_eq!(ship_of(&world, 2).location, ShipLocation::Sunk);
        assert_eq!(ship_of(&world, 2).flotation_damage, Fixed::from_int(100));
        assert!(!world.truth.task_forces[&FORCE_B]
            .ships
            .contains(&ShipId::new(2)));
        let sunk = events_where(&world, |k| matches!(k, EventKind::ShipSunk { .. }));
        assert_eq!(sunk.len(), 1);
        assert_eq!(sunk[0].kind.to_string(), "ship:2 of task_force:2 sank");
        let lost = events_where(&world, |k| matches!(k, EventKind::AirGroupLost { .. }));
        assert_eq!(lost.len(), 1);
        assert_eq!(lost[0].causes, vec![Cause::Event(sunk[0].id)]);
        assert_eq!(group_mut(&mut world, ENEMY_BOMBERS).ready, 0);
        // The sinking is told once, though 18 hits fell.
        assert_eq!(sunk.len(), 1);
    }

    // ---- the flight deck ----

    fn deck(world: &mut World, ship: u32, state: SystemState) {
        ship_mut(world, ship).systems.flight_deck = state;
    }

    #[test]
    fn a_degraded_flight_deck_flies_half_rounded_down_and_one_that_is_out_flies_none() {
        let flown = |state: SystemState, ready: u32| {
            let mut world = world_with_a_good_contact(1);
            deck(&mut world, 1, state);
            group_mut(&mut world, BOMBERS).ready = ready;
            world.decide_strikes(1, Pulse::Morning);
            launches(&world).first().map(|launch| launch_of(launch).3)
        };
        assert_eq!(flown(SystemState::Working, 18), Some(18));
        assert_eq!(flown(SystemState::Degraded, 18), Some(9));
        assert_eq!(flown(SystemState::Degraded, 7), Some(3));
        assert_eq!(flown(SystemState::Degraded, 1), None);
        assert_eq!(flown(SystemState::Out, 18), None);
        // From a base the deck is not in it.
        let mut world = world_with_a_good_contact(1);
        deck(&mut world, 1, SystemState::Out);
        world.truth.bases.insert(
            crate::ids::BaseId::new(1),
            crate::state::Base {
                id: crate::ids::BaseId::new(1),
                name: "Test base".to_string(),
                side: Side::Allied,
                hex: Hex::new(0, 0),
                supply_t: Fixed::ZERO,
            },
        );
        group_mut(&mut world, BOMBERS).location =
            AirGroupLocation::Base(crate::ids::BaseId::new(1));
        world.decide_strikes(1, Pulse::Morning);
        assert_eq!(launch_of(&launches(&world)[0]).3, 18);
    }

    #[test]
    fn a_degraded_flight_deck_halves_the_search_and_one_that_is_out_stops_it() {
        let searched = |state: SystemState| {
            let mut world = world_with_a_good_contact(1);
            deck(&mut world, 1, state);
            let scouts = group_mut(&mut world, SCOUTS);
            scouts.mission = AirMission::Search;
            scouts.search = Some(crate::state::SearchArc {
                from_deg: 40,
                to_deg: 130,
                range_nm: Fixed::from_int(300),
            });
            world.search_from_the_air(1, Pulse::Morning);
            let flights = events_where(&world, |k| matches!(k, EventKind::SearchFlown { .. }));
            let attempts = events_where(&world, |k| matches!(k, EventKind::SearchAttempt { .. }));
            let flown: Vec<u32> = flights
                .iter()
                .map(|e| match e.kind {
                    EventKind::SearchFlown { aircraft, .. } => aircraft,
                    _ => unreachable!(),
                })
                .collect();
            let chances: Vec<Fixed> = attempts
                .iter()
                .map(|e| match e.kind {
                    EventKind::SearchAttempt { chance, .. } => chance,
                    _ => unreachable!(),
                })
                .collect();
            (flown, chances, attempts.len())
        };
        // 57.295779 x 8 x 20 / (90 x 200) = 0.509295, and with 4: 0.254647.
        assert_eq!(
            searched(SystemState::Working),
            (vec![8], vec![Fixed::from_raw(509_295)], 1)
        );
        assert_eq!(
            searched(SystemState::Degraded),
            (vec![4], vec![Fixed::from_raw(254_647)], 1)
        );
        assert_eq!(searched(SystemState::Out), (vec![], vec![], 0));
    }

    #[test]
    fn a_flight_deck_hit_on_a_ship_that_carries_no_aircraft_changes_nothing_and_says_so() {
        let mut world = world_with_a_good_contact(1);
        set_class(&mut world, 2, 1);
        every_hit_is_a_flight_deck_hit(&mut world);
        strike_pulse(&mut world, Pulse::Morning);
        let hits = system_hits(&world);
        assert!(!hits.is_empty());
        for (ship, system, before, after) in &hits {
            assert_eq!(
                (*ship, *system, *before, *after),
                (
                    2,
                    ShipSystem::FlightDeck,
                    SystemState::Working,
                    SystemState::Working
                )
            );
        }
        assert_eq!(ship_of(&world, 2).systems, ShipSystems::ALL_WORKING);
        let first = events_where(&world, |k| matches!(k, EventKind::SystemHit { .. }))[0]
            .kind
            .to_string();
        assert_eq!(first, "ship:2 flight deck hit, no change, still working");
    }

    #[test]
    fn flight_deck_hits_on_a_carrier_step_it_down_and_the_hash_sees_it() {
        let mut world = world_with_a_good_contact(1);
        every_hit_is_a_flight_deck_hit(&mut world);
        air_figures(&mut world).bomb_hit_chance = fixed(1, 10);
        let working_hash = world.state_hash();
        world.decide_strikes(1, Pulse::Morning);
        world.resolve_strikes(1, Pulse::Morning);
        world.age_contacts_at_end_of_pulse(1, Pulse::Morning);
        let hits = attack_of(&attacks(&world)[0]).7;
        // 18 x 0.1 = 1.8 expected: one or two hits.
        assert!(hits == 1 || hits == 2);
        let expected = if hits == 1 {
            SystemState::Degraded
        } else {
            SystemState::Out
        };
        assert_eq!(ship_of(&world, 2).systems.flight_deck, expected);
        assert_ne!(world.state_hash(), working_hash);
        // Each state hashes differently.
        let hash_of = |state: SystemState| {
            let mut world = world_with_a_good_contact(1);
            deck(&mut world, 2, state);
            world.state_hash()
        };
        let (w, d, o) = (
            hash_of(SystemState::Working),
            hash_of(SystemState::Degraded),
            hash_of(SystemState::Out),
        );
        assert!(w != d && d != o && w != o);
    }

    // ---- both sides strike at once ----

    /// Both sides hold a fresh contact on the other's force, both bomber
    /// groups strike, and every hit lands on a flight deck.
    fn two_sided(seed: u64) -> World {
        let mut world = world(
            seed,
            vec![
                held(Hex::new(5, 0), ShipKind::Carrier, 0),
                StartingContact {
                    side: Side::Japanese,
                    hex: Hex::new(0, 0),
                    ship_count: 1,
                    kind: ShipKind::Carrier,
                    age_pulses: 0,
                },
            ],
        );
        group_mut(&mut world, ENEMY_BOMBERS).mission = AirMission::NavalStrike;
        every_hit_is_a_flight_deck_hit(&mut world);
        world
    }

    #[test]
    fn two_groups_striking_each_others_carriers_in_one_pulse_each_attack_at_full_strength() {
        for seed in 1..=20 {
            let mut world = two_sided(seed);
            strike_pulse(&mut world, Pulse::Morning);
            let all = attacks(&world);
            assert_eq!(all.len(), 2, "seed {seed}");
            let first = attack_of(&all[0]);
            let second = attack_of(&all[1]);
            // Each flies all 18 and attacks the other's carrier, though its
            // own carrier's flight deck is hit in the same pulse.
            assert_eq!((first.0, first.1, first.2), (1, 2, 2));
            assert_eq!((second.0, second.1, second.2), (3, 1, 1));
            assert_eq!(first.4, Fixed::from_int(18));
            assert_eq!(second.4, Fixed::from_int(18));
            assert_eq!(first.6, fixed(36, 10));
            assert_eq!(second.6, fixed(36, 10));
            // Both attacks are recorded before any damage lands...
            let last_attack = all[1].id;
            let first_damage = events_where(&world, |k| matches!(k, EventKind::SystemHit { .. }))
                .first()
                .expect("system hits")
                .id;
            assert!(last_attack < first_damage);
            // ...and damage lands in group order: all of group 1's hits
            // (on ship 2) before group 2's (on ship 1).
            let ships: Vec<u32> = system_hits(&world).iter().map(|h| h.0).collect();
            let split = ships
                .iter()
                .position(|ship| *ship == 1)
                .expect("ship 1 is hit");
            assert!(ships[..split].iter().all(|ship| *ship == 2));
            assert!(ships[split..].iter().all(|ship| *ship == 1));
            assert_eq!(ships[..split].len(), first.7 as usize);
            assert_eq!(ships[split..].len(), second.7 as usize);
            // Three or four hits take either deck to out.
            assert_eq!(ship_of(&world, 1).systems.flight_deck, SystemState::Out);
            assert_eq!(ship_of(&world, 2).systems.flight_deck, SystemState::Out);
        }
    }

    #[test]
    fn the_flight_decks_hit_in_one_pulse_stop_the_next_pulses_strikes_on_both_sides() {
        let mut world = two_sided(1);
        strike_pulse(&mut world, Pulse::Morning);
        assert_eq!(launches(&world).len(), 2);
        // Fresh contacts, decks out: nothing launches in the afternoon.
        strike_pulse(&mut world, Pulse::Afternoon);
        assert_eq!(launches(&world).len(), 2);
        assert_eq!(attacks(&world).len(), 2);
    }

    #[test]
    fn a_strike_launched_by_a_group_whose_ship_sinks_in_the_pulse_still_attacks() {
        // Carriers of toughness 10: a bomb of 60 does 600 points, so one hit
        // sinks one. Both strikes are launched first, so both land.
        for seed in 1..=10 {
            let mut world = two_sided(seed);
            world
                .ref_data
                .ship_classes
                .get_mut(&ShipClassId::new(2))
                .expect("class")
                .toughness = Fixed::from_int(10);
            air_figures(&mut world).bomb_hit_chance = Fixed::ONE;
            strike_pulse(&mut world, Pulse::Morning);
            let all = attacks(&world);
            assert_eq!(all.len(), 2, "seed {seed}");
            assert_eq!(attack_of(&all[0]).7, 18);
            assert_eq!(attack_of(&all[1]).7, 18);
            for ship in [1, 2] {
                assert_eq!(ship_of(&world, ship).location, ShipLocation::Sunk, "{ship}");
            }
            let sunk = events_where(&world, |k| matches!(k, EventKind::ShipSunk { .. }));
            assert_eq!(sunk.len(), 2);
            // Each air group was lost with its ship, after the attacks.
            let lost = events_where(&world, |k| matches!(k, EventKind::AirGroupLost { .. }));
            assert_eq!(
                lost.len(),
                3,
                "bombers and scouts aboard ship 1, bombers aboard ship 2"
            );
            assert!(lost.iter().all(|event| event.id > all[1].id));
            assert_eq!(group_mut(&mut world, BOMBERS).ready, 0);
            assert_eq!(group_mut(&mut world, ENEMY_BOMBERS).ready, 0);
        }
    }

    // ---- the report ----

    #[test]
    fn a_strike_that_attacks_reports_the_force_after_the_damage_citing_the_attack() {
        let mut world = world_with_a_good_contact(2);
        strike_pulse(&mut world, Pulse::Morning);
        let attack = attacks(&world)[0].id;
        let reports = events_where(&world, |k| matches!(k, EventKind::ReportReceived { .. }));
        // The starting contact, then the strike's report.
        let report = reports.last().expect("a report");
        assert_eq!(report.causes, vec![Cause::Event(attack)]);
        assert_eq!(report.day, 1);
        assert!(report.id > attack);
        let EventKind::ReportReceived {
            side,
            contact,
            ship_count,
            created,
            ..
        } = report.kind
        else {
            unreachable!()
        };
        assert_eq!(side, Side::Allied);
        // It refreshes the contact the strike flew at: one belief, one report.
        assert_eq!(contact.raw(), 1);
        assert!(!created);
        assert!((2..=4).contains(&ship_count));
        // The belief changed through the report: the contact was refreshed.
        let belief = world.beliefs.get(&Side::Allied).expect("belief");
        assert_eq!(belief.contacts().count(), 1);
        assert_eq!(
            belief.contact(contact).expect("contact").last_report_event,
            report.id
        );
    }

    #[test]
    fn a_strike_that_finds_nothing_sends_no_report() {
        let mut world = world(1, vec![held(Hex::new(4, 0), ShipKind::Cruiser, 0)]);
        let before = events_where(&world, |k| matches!(k, EventKind::ReportReceived { .. })).len();
        strike_pulse(&mut world, Pulse::Morning);
        let after = events_where(&world, |k| matches!(k, EventKind::ReportReceived { .. })).len();
        assert_eq!(before, after);
    }

    // ---- randomness ----

    #[test]
    fn the_attack_stream_draws_the_fraction_then_track_and_system_per_hit_then_the_report() {
        for seed in 1..=25 {
            let mut world = world_with_a_good_contact(seed);
            every_hit_is_a_flight_deck_hit(&mut world);
            strike_pulse(&mut world, Pulse::Morning);
            let (_, _, _, _, _, _, _, hits) = attack_of(&attacks(&world)[0]);

            let mut stream = world.rng_pair(
                1,
                Some(Pulse::Morning),
                Subsystem::AirCombat,
                EntityRef::AirGroup(BOMBERS),
                EntityRef::TaskForce(FORCE_B),
            );
            let expected_hits = 3 + u32::from(stream.chance(fixed(6, 10)));
            assert_eq!(hits, expected_hits, "seed {seed}");
            let mut flotation = 0;
            let mut fire = 0;
            for _ in 0..expected_hits {
                if stream.below(1_000_000) < 750_000 {
                    flotation += 1;
                } else {
                    fire += 1;
                }
                // The system draw: every hit is on the flight deck here.
                assert!(stream.below(1_000_000) < 1_000_000);
            }
            let damaged = events_where(&world, |k| matches!(k, EventKind::ShipDamaged { .. }));
            let counted = |wanted: DamageTrack| -> u32 {
                damaged
                    .iter()
                    .map(|e| match e.kind {
                        EventKind::ShipDamaged { track, hits, .. } if track == wanted => hits,
                        _ => 0,
                    })
                    .sum()
            };
            assert_eq!(counted(DamageTrack::Flotation), flotation, "seed {seed}");
            assert_eq!(counted(DamageTrack::Fire), fire, "seed {seed}");

            // The report is the next three draws of the same stream.
            let report = build_sighting_report(
                &mut stream,
                &world.map,
                Hex::new(5, 0),
                3,
                ShipKind::Carrier,
                world.ref_data.search.as_ref().expect("search"),
                EventId::new(0),
            );
            let received = events_where(&world, |k| matches!(k, EventKind::ReportReceived { .. }));
            let EventKind::ReportReceived {
                hex,
                ship_count,
                kind,
                ..
            } = received.last().expect("a report").kind
            else {
                unreachable!()
            };
            assert_eq!(
                (hex, ship_count, kind),
                (
                    report.estimated_hex,
                    report.estimated_ship_count,
                    report.estimated_kind
                ),
                "seed {seed}"
            );
        }
    }

    #[test]
    fn recording_breakdowns_changes_neither_the_outcome_nor_the_hash() {
        let run = |record: bool, seed: u64| {
            let mut world = two_sided(seed);
            world.set_config(crate::world::SimConfig {
                record_breakdowns: record,
            });
            strike_pulse(&mut world, Pulse::Morning);
            world.age_contacts_at_end_of_pulse(1, Pulse::Morning);
            strike_pulse(&mut world, Pulse::Afternoon);
            world.age_contacts_at_end_of_pulse(1, Pulse::Afternoon);
            let breakdowns = attacks(&world)
                .iter()
                .filter(|e| e.breakdown.is_some())
                .count();
            (world.state_hash(), world.events().len(), breakdowns)
        };
        for seed in 1..=10 {
            let on = run(true, seed);
            let off = run(false, seed);
            assert_eq!((on.0, on.1), (off.0, off.1), "seed {seed}");
            assert_eq!((on.2, off.2), (2, 0));
        }
    }

    // ---- orders ----

    fn mission_order(group: AirGroupId, mission: AirMission) -> OrderRequest {
        OrderRequest {
            kind: OrderKind::AirGroup {
                target: group,
                order: AirGroupOrder::SetAirMission(mission),
            },
            origin: Origin::PLAYER,
        }
    }

    fn rejections(world: &World) -> Vec<RejectReason> {
        events_where(world, |k| matches!(k, EventKind::OrderRejected { .. }))
            .iter()
            .map(|e| match &e.kind {
                EventKind::OrderRejected { reason, .. } => reason.clone(),
                _ => unreachable!(),
            })
            .collect()
    }

    #[test]
    fn the_strike_mission_is_rejected_without_air_combat_figures_or_for_a_type_that_attacks_with_nothing(
    ) {
        // Accepted for a bomber with the figures.
        let mut world = self::world(1, vec![]);
        group_mut(&mut world, BOMBERS).mission = AirMission::Rest;
        world.submit_orders(
            Side::Allied,
            vec![mission_order(BOMBERS, AirMission::NavalStrike)],
        );
        world.resolve_day().expect("day resolves");
        assert!(rejections(&world).is_empty());
        assert_eq!(
            group_mut(&mut world, BOMBERS).mission,
            AirMission::NavalStrike
        );

        // Rejected with no figures, and the mission is unchanged.
        let mut world = self::world(1, vec![]);
        group_mut(&mut world, BOMBERS).mission = AirMission::Rest;
        world.ref_data.air_combat = None;
        world.submit_orders(
            Side::Allied,
            vec![mission_order(BOMBERS, AirMission::NavalStrike)],
        );
        world.resolve_day().expect("day resolves");
        assert_eq!(rejections(&world), vec![RejectReason::NoAirCombatFigures]);
        assert_eq!(group_mut(&mut world, BOMBERS).mission, AirMission::Rest);

        // Rejected for scouts, whose type attacks with nothing, and for a
        // bomber type that attacks with nothing. Other missions are fine.
        let mut world = self::world(1, vec![]);
        group_mut(&mut world, BOMBERS).aircraft_type = AircraftTypeId::new(4);
        group_mut(&mut world, BOMBERS).mission = AirMission::Rest;
        world.submit_orders(
            Side::Allied,
            vec![
                mission_order(SCOUTS, AirMission::NavalStrike),
                mission_order(BOMBERS, AirMission::NavalStrike),
                mission_order(SCOUTS, AirMission::Search),
            ],
        );
        world.resolve_day().expect("day resolves");
        assert_eq!(
            rejections(&world),
            vec![
                RejectReason::AircraftAttackWithNothing,
                RejectReason::AircraftAttackWithNothing
            ]
        );
        assert_eq!(group_mut(&mut world, SCOUTS).mission, AirMission::Search);
        assert_eq!(group_mut(&mut world, BOMBERS).mission, AirMission::Rest);
        let text = events_where(&world, |k| matches!(k, EventKind::OrderRejected { .. }))[0]
            .kind
            .to_string();
        assert!(text.contains("attacks with nothing"), "{text}");
    }

    #[test]
    fn a_full_day_launches_and_attacks_in_the_daylight_pulses_only() {
        let mut world = world_with_a_good_contact(2);
        world.resolve_day().expect("day resolves");
        for event in launches(&world).iter().chain(&attacks(&world)) {
            assert_ne!(event.pulse, Some(Pulse::Night));
            assert_eq!(event.day, 1);
        }
        // The contact made before day 1 was aged once by the night pulse, so
        // it is a pulse old in the morning: still struck.
        assert!(!launches(&world).is_empty());
        assert!(world.strikes_in_flight.is_empty());
    }
}
