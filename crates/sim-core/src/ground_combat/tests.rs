use super::*;
use crate::choices::ground::{GroundAnswer, GroundQuestion, UnitDay};
use crate::choices::{Answer, Question};
use crate::events::EventKind;
use crate::fixed::Fixed;
use crate::ids::{AirGroupId, ShipId, TaskForceId};
use crate::orders::{Activity, AirMission};
use crate::state::{AirGroupLocation, ShipLocation};

use super::fight::{attack_failed, engaged_shots, firing_men};

fn fx(text: &str) -> Fixed {
    text.parse().expect("a decimal")
}

#[test]
fn firing_men_are_the_men_times_the_fed_share_times_the_rested_share_cut() {
    assert_eq!(firing_men(2000, Fixed::ONE, Fixed::ZERO).0, 2000);
    assert_eq!(firing_men(2000, Fixed::ONE, Fixed::from_int(20)).0, 1600);
    assert_eq!(firing_men(2000, Fixed::ONE, Fixed::from_int(60)).0, 800);
    assert_eq!(firing_men(285, Fixed::ZERO, Fixed::ZERO).0, 0);
    assert_eq!(firing_men(600, fx("0.5"), Fixed::from_int(20)).0, 240);
    // Cut to a whole man.
    assert_eq!(firing_men(7, fx("0.5"), Fixed::ZERO).0, 3);
    assert_eq!(
        firing_men(100, Fixed::ONE, Fixed::from_int(40)).1,
        fx("0.6")
    );
}

#[test]
fn shots_are_cut_in_proportion_only_above_the_engaged_men() {
    // 6000 firing men against a limit of 3 x 1000: 60 shots to 30.
    assert_eq!(
        engaged_shots(Fixed::from_int(60), 6000, Fixed::from_int(3000)),
        Fixed::from_int(30)
    );
    // At or below the limit nothing is cut.
    assert_eq!(
        engaged_shots(Fixed::from_int(40), 4000, Fixed::from_int(4800)),
        Fixed::from_int(40)
    );
    assert_eq!(
        engaged_shots(Fixed::from_int(30), 3000, Fixed::from_int(3000)),
        Fixed::from_int(30)
    );
    // A unit's share of a side's cut.
    assert_eq!(
        engaged_shots(fx("12.5"), 5000, Fixed::from_int(2500)),
        fx("6.25")
    );
}

#[test]
fn an_attack_fails_when_its_weight_is_below_the_others() {
    assert!(attack_failed(true, Fixed::from_int(3), Fixed::from_int(18)));
    assert!(!attack_failed(true, Fixed::from_int(12), fx("9.6")));
    // Equal weights: neither fails.
    assert!(!attack_failed(true, Fixed::from_int(9), Fixed::from_int(9)));
    // A side that did not attack never fails.
    assert!(!attack_failed(false, Fixed::ZERO, Fixed::from_int(9)));
    // A side with no weight that attacked fails if the other's is above 0.
    assert!(attack_failed(true, Fixed::ZERO, fx("0.1")));
    assert!(!attack_failed(true, Fixed::ZERO, Fixed::ZERO));
}

#[test]
fn a_spent_attacker_does_what_its_order_says_and_else_holds_where_it_is_fed() {
    for fed in [true, false] {
        assert_eq!(
            when_attack_spent(Some(WhenSpent::Hold), fed),
            WhenSpent::Hold
        );
        assert_eq!(
            when_attack_spent(Some(WhenSpent::FallBack), fed),
            WhenSpent::FallBack
        );
    }
    assert_eq!(when_attack_spent(None, true), WhenSpent::Hold);
    assert_eq!(when_attack_spent(None, false), WhenSpent::FallBack);
}

#[test]
fn a_side_is_beaten_with_no_man_or_below_the_share_of_the_others_men() {
    let share = fx("0.2");
    assert!(is_beaten(0, 0, share));
    assert!(is_beaten(285, 2000, share));
    assert!(is_beaten(399, 2000, share));
    assert!(!is_beaten(400, 2000, share));
    assert!(!is_beaten(520, 600, share));
}

// ---- A base that changes side, by hand-built worlds (decisions 20 to
// 23 of 0090). Each world is Japanese base 1 at AT with nobody to
// defend it, and 1,000 Allied men ashore there on attack: the base falls
// without a fight in the ground combat phase.

use crate::events::Event;
use crate::ids::{AircraftTypeId, OrderId, ShipClassId};
use crate::invariants;
use crate::map::{HexData, Map};
use crate::orders::{AirGroupOrder, Mission, OrderKind, OrderRequest, Origin, Reach, RejectReason};
use crate::refdata::{
    AircraftRole, AircraftType, AttackWeapon, PortSize, RefData, SearchFigures, ShipClass, ShipKind,
};
use crate::state::{
    AirGroup, Base, LandUnit, Replacement, Ship, TaskForce, Troops, TruthState, YardTrip,
};

/// The contested hex, with Japanese base 1; Japanese base 2 and Allied
/// base 3 are elsewhere.
const AT: Hex = Hex::new(0, 0);
const ELSEWHERE: Hex = Hex::new(4, 0);
const FAR: Hex = Hex::new(8, 0);

fn n(whole: i64) -> Fixed {
    Fixed::from_int(whole)
}

fn bid(id: u32) -> BaseId {
    BaseId::new(id)
}

fn gid(id: u32) -> AirGroupId {
    AirGroupId::new(id)
}

fn lid(id: u32) -> LandUnitId {
    LandUnitId::new(id)
}

fn sid(id: u32) -> ShipId {
    ShipId::new(id)
}

fn tid(id: u32) -> TaskForceId {
    TaskForceId::new(id)
}

fn escort_class() -> ShipClass {
    ShipClass {
        id: ShipClassId::new(1),
        name: "Test ship".to_string(),
        kind: ShipKind::Cruiser,
        cruise_speed_kn: n(16),
        max_speed_kn: n(32),
        fuel_capacity_t: n(1000),
        endurance_nm: n(8000),
        cargo_capacity_t: Fixed::ZERO,
        main_guns: 0,
        gun_rounds_per_min: Fixed::ZERO,
        gun_rounds_per_gun: 0,
        gun_damage: Fixed::ZERO,
        torpedo_tubes: 0,
        torpedo_tubes_one_side: 0,
        torpedo_damage: Fixed::ZERO,
        toughness: n(100),
        radar: false,
        aircraft_capacity: 20,
        anti_aircraft: 0,
        troop_capacity_men: 600,
        fuels_at_sea: false,
    }
}

fn fighter_type() -> AircraftType {
    AircraftType {
        id: AircraftTypeId::new(5),
        name: "Test fighter".to_string(),
        role: AircraftRole::Fighter,
        search_range_nm: Fixed::ZERO,
        sweep_width_nm: Fixed::ZERO,
        strike_range_nm: n(250),
        attack_weapon: AttackWeapon::None,
        attack_damage: Fixed::ZERO,
        pilot_armour: false,
        self_sealing_tanks: false,
    }
}

fn air_group(
    id: u32,
    side: Side,
    location: AirGroupLocation,
    ready: u32,
    damaged: u32,
    pilots: Option<u32>,
) -> AirGroup {
    AirGroup {
        ready,
        damaged,
        pilots,
        strength: ready + damaged + 2,
        ..AirGroup::new(
            gid(id),
            format!("Air group {id}"),
            side,
            location,
            AircraftTypeId::new(5),
        )
    }
}

/// The world: Japanese base 1 (60 t of supply, 40 t of fuel) at AT, with
/// no Japanese man there; Japanese base 2 at ELSEWHERE; Allied base 3 at
/// FAR. Allied unit 1, 1,000 men on attack at AT.
///
/// Japanese: air group 1 at base 1 (9 ready, 2 damaged, 10 pilots); ship
/// 1 in port at base 1, with air group 7 aboard (6 ready, 1 damaged, 5
/// pilots) and 150 men of unit 2 (all it has); task force 1 at sea with
/// ship 2 and base 1 as its home port; task force 2 at sea with ship 3
/// and base 2 as its home port.
///
/// Allied: air group 5 at base 3 on a strike on base 1; air group 6 at
/// base 3 on a strike on base 2.
fn world() -> World {
    let mut map = Map::new();
    for q in -1..=9 {
        for r in -1..=1 {
            map.insert(Hex::new(q, r), HexData::open_sea());
        }
    }
    let mut ref_data = RefData::for_tests();
    ref_data
        .ship_classes
        .insert(escort_class().id, escort_class());
    ref_data
        .aircraft_types
        .insert(fighter_type().id, fighter_type());
    ref_data.search = SearchFigures::from_search_toml();
    ref_data.port_sizes.insert(
        0,
        PortSize {
            berths: 0,
            berth_rate_t_per_h: Fixed::ZERO,
            lighter_rate_t_per_h: n(20),
            fuel_berth_rate_t_per_h: None,
            fuel_lighter_rate_t_per_h: None,
            ammunition_berth_rate_t_per_h: None,
            ammunition_lighter_rate_t_per_h: None,
        },
    );
    ref_data.ground = Some(GroundFigures::FIRST);

    let mut truth = TruthState::default();
    for (id, side, hex, supply, fuel) in [
        (1, Side::Japanese, AT, 60, 40),
        (2, Side::Japanese, ELSEWHERE, 10, 0),
        (3, Side::Allied, FAR, 100, 0),
    ] {
        truth.bases.insert(
            bid(id),
            Base {
                supply_t: n(supply),
                fuel_t: n(fuel),
                ..Base::new(bid(id), format!("Base {id}"), side, hex)
            },
        );
    }
    let mut unit = LandUnit::new(
        lid(1),
        "Allied unit".to_string(),
        Side::Allied,
        1000,
        Some(AT),
    );
    unit.posture = Posture::Attack;
    truth.land_units.insert(unit.id, unit);
    truth.land_units.insert(
        lid(2),
        LandUnit::new(lid(2), "Japanese unit".to_string(), Side::Japanese, 0, None),
    );
    for (id, home, hex, ship) in [(1, 1, Hex::new(2, 0), 2), (2, 2, Hex::new(5, 0), 3)] {
        let mut force = TaskForce::at(
            tid(id),
            format!("Force {id}"),
            Side::Japanese,
            hex,
            Mission::Escort,
        );
        force.home_port = Some(bid(home));
        force.ships.insert(sid(ship));
        truth.task_forces.insert(force.id, force);
        truth.ships.insert(
            sid(ship),
            Ship::new(
                sid(ship),
                format!("Ship {ship}"),
                ShipClassId::new(1),
                Side::Japanese,
                ShipLocation::TaskForce(tid(id)),
            ),
        );
    }
    truth.ships.insert(
        sid(1),
        Ship {
            troops: Some(Troops {
                unit: lid(2),
                men: 150,
            }),
            ..Ship::new(
                sid(1),
                "Ship in port".to_string(),
                ShipClassId::new(1),
                Side::Japanese,
                ShipLocation::Port(bid(1)),
            )
        },
    );
    for group in [
        air_group(
            1,
            Side::Japanese,
            AirGroupLocation::Base(bid(1)),
            9,
            2,
            Some(10),
        ),
        air_group(
            7,
            Side::Japanese,
            AirGroupLocation::Ship(sid(1)),
            6,
            1,
            Some(5),
        ),
        AirGroup {
            mission: AirMission::BaseStrike(bid(1)),
            mission_order: Some(OrderId::new(1)),
            ..air_group(5, Side::Allied, AirGroupLocation::Base(bid(3)), 6, 0, None)
        },
        AirGroup {
            mission: AirMission::BaseStrike(bid(2)),
            mission_order: Some(OrderId::new(2)),
            ..air_group(6, Side::Allied, AirGroupLocation::Base(bid(3)), 6, 0, None)
        },
    ] {
        truth.air_groups.insert(group.id, group);
    }
    let world = World::new(7, ref_data, map, truth).expect("valid setup");
    assert!(invariants::check(&world.truth, &world.map, &world.ref_data).is_empty());
    world
}

fn all_events(world: &World) -> Vec<Event> {
    world.events().iter().cloned().collect()
}

fn sound(world: &World) {
    let violations = invariants::check(&world.truth, &world.map, &world.ref_data);
    assert!(violations.is_empty(), "{violations:?}");
}

/// The world after the ground combat phase of day 1.
fn fallen() -> World {
    let mut world = world();
    world.fight_ashore(1);
    sound(&world);
    world
}

fn one(events: &[Event], pick: impl Fn(&EventKind) -> bool) -> &Event {
    let found: Vec<&Event> = events.iter().filter(|e| pick(&e.kind)).collect();
    assert_eq!(found.len(), 1, "{found:?}");
    found[0]
}

#[test]
fn a_base_with_no_man_falls_to_a_unit_on_attack_with_half_its_stocks() {
    let world = fallen();
    let events = all_events(&world);
    let beaten = one(&events, |k| matches!(k, EventKind::SideBeatenAshore { .. }));
    assert_eq!(
        beaten.kind,
        EventKind::SideBeatenAshore {
            side: Side::Japanese,
            hex: AT,
            men: 0,
            enemy_men: 1000,
            fought: false,
        }
    );
    let changed = one(&events, |k| matches!(k, EventKind::BaseChangedSide { .. }));
    assert_eq!(
        changed.kind,
        EventKind::BaseChangedSide {
            base: bid(1),
            hex: AT,
            from: Side::Japanese,
            to: Side::Allied,
            supply_t: n(30),
            fuel_t: n(20),
            supply_lost_t: n(30),
            fuel_lost_t: n(20),
        }
    );
    assert!(changed.causes.contains(&Cause::Event(beaten.id)));
    let base = &world.truth.bases[&bid(1)];
    assert_eq!(
        (base.side, base.supply_t, base.fuel_t, base.on_map),
        (Side::Allied, n(30), n(20), true)
    );
    // The base at another hex is not touched.
    assert_eq!(world.truth.bases[&bid(2)].side, Side::Japanese);
    assert_eq!(world.truth.bases[&bid(2)].supply_t, n(10));
}

#[test]
fn a_force_of_the_beaten_side_whose_home_port_it_was_has_none_and_another_keeps_its_own() {
    let world = fallen();
    let events = all_events(&world);
    let changed = one(&events, |k| matches!(k, EventKind::BaseChangedSide { .. }));
    let lost = one(&events, |k| matches!(k, EventKind::HomePortLost { .. }));
    assert_eq!(
        lost.kind,
        EventKind::HomePortLost {
            task_force: tid(1),
            base: bid(1),
        }
    );
    assert_eq!(lost.causes, vec![Cause::Event(changed.id)]);
    assert_eq!(world.truth.task_forces[&tid(1)].home_port, None);
    assert_eq!(world.truth.task_forces[&tid(2)].home_port, Some(bid(2)));
}

#[test]
fn a_winning_group_on_a_strike_at_the_base_rests_and_one_on_another_base_is_untouched() {
    let world = fallen();
    let events = all_events(&world);
    let changed = one(&events, |k| matches!(k, EventKind::BaseChangedSide { .. }));
    let cleared = one(&events, |k| {
        matches!(k, EventKind::StrikeMissionCleared { .. })
    });
    assert_eq!(
        cleared.kind,
        EventKind::StrikeMissionCleared {
            air_group: gid(5),
            base: bid(1),
        }
    );
    assert_eq!(cleared.causes, vec![Cause::Event(changed.id)]);
    let five = &world.truth.air_groups[&gid(5)];
    assert_eq!((five.mission, five.mission_order), (AirMission::Rest, None));
    let six = &world.truth.air_groups[&gid(6)];
    assert_eq!(
        (six.mission, six.mission_order),
        (AirMission::BaseStrike(bid(2)), Some(OrderId::new(2)))
    );
    assert!(!events.iter().any(|e| matches!(&e.kind,
        EventKind::StrikeMissionCleared { air_group, .. } if *air_group == gid(6))));
}

#[test]
fn an_air_group_at_the_base_is_lost_whole_with_its_pilots() {
    let world = fallen();
    let events = all_events(&world);
    let changed = one(&events, |k| matches!(k, EventKind::BaseChangedSide { .. }));
    let lost = one(&events, |k| {
        matches!(k, EventKind::AirGroupLostWithBase { .. })
    });
    assert_eq!(
        lost.kind,
        EventKind::AirGroupLostWithBase {
            air_group: gid(1),
            base: bid(1),
            aircraft: 11,
        }
    );
    assert_eq!(lost.causes, vec![Cause::Event(changed.id)]);
    let group = &world.truth.air_groups[&gid(1)];
    assert_eq!(group.location, AirGroupLocation::Lost);
    assert_eq!((group.ready, group.damaged, group.pilots), (0, 0, Some(0)));
    let pilots = events
        .iter()
        .find(
            |e| matches!(&e.kind, EventKind::PilotsLost { air_group, .. } if *air_group == gid(1)),
        )
        .expect("the pilots are lost");
    assert_eq!(
        pilots.kind,
        EventKind::PilotsLost {
            air_group: gid(1),
            lost: 10,
            left: 0,
        }
    );
    assert_eq!(pilots.causes, vec![Cause::Event(lost.id)]);
    // Allied groups elsewhere are not touched.
    assert_eq!(
        world.truth.air_groups[&gid(5)].location,
        AirGroupLocation::Base(bid(3))
    );
}

/// Decision 0101: a base that changes side loses only the groups of the
/// beaten side. An empty group of the side that takes it stays.
#[test]
fn a_group_of_the_side_that_takes_the_base_stays() {
    let mut world = world();
    let waiting = air_group(8, Side::Allied, AirGroupLocation::Base(bid(1)), 0, 0, None);
    world.truth.air_groups.insert(waiting.id, waiting);
    sound(&world);
    world.fight_ashore(1);
    sound(&world);
    let events = all_events(&world);
    one(&events, |k| matches!(k, EventKind::BaseChangedSide { .. }));
    // The beaten side's group is lost as before.
    assert_eq!(
        world.truth.air_groups[&gid(1)].location,
        AirGroupLocation::Lost
    );
    // The taker's group stays at the base, untouched, and no event names it.
    assert_eq!(
        world.truth.air_groups[&gid(8)].location,
        AirGroupLocation::Base(bid(1))
    );
    assert!(!events.iter().any(|e| matches!(&e.kind,
        EventKind::AirGroupLostWithBase { air_group, .. } if *air_group == gid(8))));
}

#[test]
fn a_group_with_no_pilots_counted_is_lost_with_no_pilot_event() {
    let mut world = world();
    world.truth.air_groups.get_mut(&gid(1)).unwrap().pilots = None;
    world.fight_ashore(1);
    let events = all_events(&world);
    assert_eq!(world.truth.air_groups[&gid(1)].pilots, None);
    assert!(!events.iter().any(|e| matches!(&e.kind,
        EventKind::PilotsLost { air_group, .. } if *air_group == gid(1))));
    assert_eq!(
        world.truth.air_groups[&gid(1)].location,
        AirGroupLocation::Lost
    );
    sound(&world);
}

#[test]
fn a_ship_in_port_is_sunk_with_the_group_and_the_men_aboard_her() {
    let world = fallen();
    let events = all_events(&world);
    let changed = one(&events, |k| matches!(k, EventKind::BaseChangedSide { .. }));
    let sunk = one(&events, |k| matches!(k, EventKind::ShipLostWithBase { .. }));
    assert_eq!(
        sunk.kind,
        EventKind::ShipLostWithBase {
            ship: sid(1),
            base: bid(1),
        }
    );
    assert_eq!(sunk.causes, vec![Cause::Event(changed.id)]);
    assert_eq!(world.truth.ships[&sid(1)].location, ShipLocation::Sunk);
    // The group aboard her is lost with her, as with a ship that sinks.
    let group = one(&events, |k| matches!(k, EventKind::AirGroupLost { .. }));
    assert_eq!(
        group.kind,
        EventKind::AirGroupLost {
            air_group: gid(7),
            ship: sid(1),
            aircraft: 7,
        }
    );
    assert_eq!(group.causes, vec![Cause::Event(sunk.id)]);
    let aboard = &world.truth.air_groups[&gid(7)];
    assert_eq!(
        (aboard.ready, aboard.damaged, aboard.pilots),
        (0, 0, Some(0))
    );
    // The men aboard her are lost with her, and a unit with no man left
    // is destroyed.
    let men = one(&events, |k| matches!(k, EventKind::MenLostWithShip { .. }));
    assert_eq!(
        men.kind,
        EventKind::MenLostWithShip {
            unit: lid(2),
            ship: sid(1),
            men: 150,
        }
    );
    assert_eq!(men.causes, vec![Cause::Event(sunk.id)]);
    assert_eq!(world.truth.ships[&sid(1)].troops, None);
    assert!(events
        .iter()
        .any(|e| e.kind == EventKind::LandUnitDestroyed { unit: lid(2) }));
    // A force at sea in the hex or near it is not touched.
    assert!(matches!(
        world.truth.ships[&sid(2)].location,
        ShipLocation::TaskForce(_)
    ));
}

#[test]
fn every_order_to_a_group_lost_with_its_base_is_refused() {
    let mut world = fallen();
    let requests: Vec<OrderRequest> = [
        AirGroupOrder::SetAirMission(AirMission::Search),
        AirGroupOrder::SetSearchArc {
            from_deg: 0,
            to_deg: 90,
            range_nm: n(100),
        },
        AirGroupOrder::SetPatrolShare { share: Fixed::ONE },
        AirGroupOrder::SetRestThreshold { fatigue: n(50) },
        AirGroupOrder::SetReach {
            reach: Reach::Extended,
        },
        AirGroupOrder::SetTargetPriority {
            kinds: vec![ShipKind::Carrier],
        },
        AirGroupOrder::Rebase { base: bid(2) },
    ]
    .into_iter()
    .map(|order| OrderRequest {
        kind: OrderKind::AirGroup {
            target: gid(1),
            order,
        },
        origin: Origin::PLAYER,
    })
    .collect();
    let count = requests.len();
    world.submit_orders(Side::Japanese, requests);
    world.resolve_day().expect("the day resolves");
    let events = all_events(&world);
    let refused: Vec<&RejectReason> = events
        .iter()
        .filter_map(|e| match &e.kind {
            EventKind::OrderRejected { reason, .. } => Some(reason),
            _ => None,
        })
        .collect();
    assert_eq!(refused, vec![&RejectReason::AirGroupLostWithBase; count]);
    assert!(!events
        .iter()
        .any(|e| matches!(e.kind, EventKind::OrderAccepted { .. })));
    // The group is still lost and flies nothing.
    assert_eq!(
        world.truth.air_groups[&gid(1)].location,
        AirGroupLocation::Lost
    );
    sound(&world);
}

#[test]
fn a_force_away_at_the_yard_from_the_port_returns_to_the_hex_and_finds_no_base() {
    let mut world = world();
    // Force 1 left from base 1 and is due back on day 2. Its ship is at
    // the yard.
    {
        let force = world.truth.task_forces.get_mut(&tid(1)).unwrap();
        force.ships.clear();
        force.from_hex = AT;
        force.yard_trip = Some(YardTrip {
            base: bid(1),
            return_day: 2,
            departure: EventId::new(0),
        });
        world.truth.ships.get_mut(&sid(2)).unwrap().location = ShipLocation::Yard(tid(1));
    }
    sound(&world);
    world.resolve_day().expect("day 1 resolves: the base falls");
    assert_eq!(world.truth.bases[&bid(1)].side, Side::Allied);
    // The force is away and has lost its home port, with an event.
    assert_eq!(world.truth.task_forces[&tid(1)].home_port, None);
    assert!(world.truth.task_forces[&tid(1)].yard_trip.is_some());
    let events = all_events(&world);
    let changed = one(&events, |k| matches!(k, EventKind::BaseChangedSide { .. }));
    let lost = one(&events, |k| matches!(k, EventKind::HomePortLost { .. }));
    assert_eq!(lost.causes, vec![Cause::Event(changed.id)]);
    sound(&world);
    world
        .resolve_day()
        .expect("day 2 resolves: the force returns");
    let events = all_events(&world);
    let back = one(&events, |k| {
        matches!(k, EventKind::ForceReturnedFromYard { .. })
    });
    assert_eq!(back.day, 2);
    assert!(matches!(&back.kind,
        EventKind::ForceReturnedFromYard { task_force, base, .. }
            if *task_force == tid(1) && *base == bid(1)));
    let force = &world.truth.task_forces[&tid(1)];
    assert_eq!(force.yard_trip, None);
    assert_eq!(force.position(), AT);
    assert_eq!(force.home_port, None);
    assert_eq!(
        world.truth.ships[&sid(2)].location,
        ShipLocation::TaskForce(tid(1))
    );
    // No Japanese base is in the hex: it finds none.
    assert!(!world
        .truth
        .bases
        .values()
        .any(|base| base.side == Side::Japanese && base.hex == AT));
    sound(&world);
}

#[test]
fn a_replacement_stream_of_the_beaten_side_delivers_nothing_after_the_base_falls() {
    let mut world = world();
    world.truth.replacements.push(Replacement {
        side: Side::Japanese,
        base: bid(1),
        aircraft_type: AircraftTypeId::new(5),
        aircraft_per_day: 5,
        pilots_per_day: 3,
    });
    world.resolve_day().expect("day 1");
    // An Allied group of the same type, short of its strength, is now at
    // base 1, which is the Allied side's.
    world.truth.air_groups.insert(
        gid(8),
        air_group(
            8,
            Side::Allied,
            AirGroupLocation::Base(bid(1)),
            4,
            0,
            Some(1),
        ),
    );
    let before = world.truth.air_groups[&gid(8)].clone();
    world.resolve_day().expect("day 2");
    world.resolve_day().expect("day 3");
    let events = all_events(&world);
    assert!(!events
        .iter()
        .any(|e| matches!(e.kind, EventKind::ReplacementsReceived { .. })));
    assert_eq!(world.truth.air_groups[&gid(8)], before);
    // The lost group has nothing either.
    let lost = &world.truth.air_groups[&gid(1)];
    assert_eq!(
        (lost.location, lost.ready, lost.pilots),
        (AirGroupLocation::Lost, 0, Some(0))
    );
    sound(&world);
}

#[test]
fn from_the_next_day_the_base_feeds_men_of_its_new_side() {
    let mut world = world();
    world.resolve_day().expect("day 1: the base falls");
    let events = all_events(&world);
    // Night of day 1: no base of the side yet; the 1,000 men are unfed
    // and lose 50.
    let night_1 = events
        .iter()
        .find_map(|e| match &e.kind {
            EventKind::MenFed {
                side: Side::Allied,
                base,
                men,
                fed,
                unfed,
                ..
            } if e.day == 1 => Some((*base, *men, *fed, *unfed)),
            _ => None,
        })
        .expect("the night of day 1");
    assert_eq!(night_1, (None, 1000, 0, 1000));
    assert_eq!(world.truth.land_units[&lid(1)].men_ashore, 950);
    assert_eq!(world.truth.land_units[&lid(1)].fed_share, Fixed::ZERO);
    world.resolve_day().expect("day 2");
    let events = all_events(&world);
    // Night of day 2: the 950 men are fed from base 1, which pays
    // 950 x 0.005 = 4.75 t of its 30.
    let fed = events
        .iter()
        .find_map(|e| match &e.kind {
            EventKind::MenFed {
                side: Side::Allied,
                base,
                men,
                fed,
                unfed,
                ..
            } if e.day == 2 => Some((*base, *men, *fed, *unfed)),
            _ => None,
        })
        .expect("the night of day 2");
    assert_eq!(fed, (Some(bid(1)), 950, 950, 0));
    assert_eq!(
        world.truth.bases[&bid(1)].supply_t,
        "25.25".parse().unwrap()
    );
    assert_eq!(world.truth.land_units[&lid(1)].fed_share, Fixed::ONE);
    // The Japanese unit never fed from it, and nothing of the Japanese
    // side is charged there.
    assert!(!events.iter().any(|e| matches!(&e.kind,
        EventKind::MenFed { side: Side::Japanese, hex, .. } if *hex == AT)));
    sound(&world);
}

// ---- The choices of the land units (decision 0100) ----

use crate::belief::{GroundReport, Report};
use crate::choices::{EndOfDayPhase, Moment};
use crate::orders::Job;

/// Decision 14 of 0090 by hand: only a unit on attack goes back, and
/// only above the limit of 60 (not at it).
#[test]
fn a_unit_goes_back_to_defend_only_on_attack_and_above_the_limit() {
    assert!(goes_back_to_defend(Posture::Attack, n(61), n(60)));
    assert!(!goes_back_to_defend(Posture::Attack, n(60), n(60)));
    assert!(!goes_back_to_defend(Posture::Defend, n(100), n(60)));
}

/// Decision 3b of 0099 by hand: no men is no day; a march or a contested
/// hex is the day; a working unit works whatever its activity; training
/// is training; a ready unit rehearses only if rostered and with the
/// force (`with_force` is not asked otherwise).
#[test]
fn a_units_day_is_the_first_of_no_men_march_fight_work_train_rehearse_ready() {
    let day = |ashore, aboard, working, activity, marched, contested, roster, with| {
        unit_day(
            ashore,
            aboard,
            working,
            activity,
            marched,
            contested,
            roster,
            |_| with,
        )
    };
    let ready = Activity::Ready;
    assert_eq!(
        day(0, 0, false, ready, true, true, None, true),
        UnitDay::NoMen
    );
    assert_eq!(
        day(10, 0, false, ready, true, true, None, true),
        UnitDay::Marched
    );
    assert_eq!(
        day(0, 10, false, ready, false, true, None, true),
        UnitDay::Contested
    );
    assert_eq!(
        day(10, 0, true, Activity::Train, false, false, None, true),
        UnitDay::Work(None)
    );
    assert_eq!(
        day(
            10,
            0,
            false,
            Activity::Work(Some(Job::Port)),
            false,
            false,
            None,
            true
        ),
        UnitDay::Work(Some(Job::Port))
    );
    assert_eq!(
        day(
            10,
            0,
            false,
            Activity::Train,
            false,
            false,
            Some(tid(1)),
            true
        ),
        UnitDay::Train
    );
    assert_eq!(
        day(10, 0, false, ready, false, false, Some(tid(1)), true),
        UnitDay::Rehearse(tid(1))
    );
    assert_eq!(
        day(10, 0, false, ready, false, false, Some(tid(1)), false),
        UnitDay::Ready
    );
    assert_eq!(
        day(10, 0, false, ready, false, false, None, true),
        UnitDay::Ready
    );
}

/// The test world with a tired Allied unit on attack (unit 1, fatigue
/// 100) and a second Allied unit (3) ashore at the Allied base with its
/// force, idle there, preparing a landing with it on the roster.
fn ground_world() -> World {
    let mut world = world();
    world
        .truth
        .land_units
        .get_mut(&lid(1))
        .expect("unit 1")
        .fatigue = n(100);
    world.truth.land_units.insert(
        lid(3),
        LandUnit::new(
            lid(3),
            "Allied unit 3".to_string(),
            Side::Allied,
            500,
            Some(FAR),
        ),
    );
    let mut force = TaskForce::at(
        tid(3),
        "Force 3".to_string(),
        Side::Allied,
        FAR,
        Mission::Escort,
    );
    force.preparation = Some(crate::state::Preparation {
        hex: FAR,
        roster: [lid(3)].into_iter().collect(),
        loading: Fixed::ZERO,
        order: OrderId::new(9),
    });
    world.truth.task_forces.insert(force.id, force);
    sound(&world);
    world
}

/// Asked from the Allied view before the phase, every question of the
/// view gives the answer the engine traced for the same unit: unit 1,
/// on attack at fatigue 100, goes back to defend (the engine's fatigue
/// is 95 after resting, still above 60); unit 3 rehearses with force 3;
/// unit 2 (Japanese, in port) is not the side's. The questions the
/// engine traced are answered by the view as the engine did.
#[test]
fn the_view_gives_the_answers_the_engine_traced_for_land_units() {
    let mut world = ground_world();
    let before = world.clone();
    world.trace_choices(true);
    world.pass_moment(Moment::EndOfDay {
        day: 1,
        phase: EndOfDayPhase::GroundCombat,
    });
    world.fight_ashore(1);
    let trace = world.take_choice_trace();
    let allied = before.side_view(Side::Allied);
    let mut back = 0;
    let mut rehearse = 0;
    for entry in &trace {
        // The one exception, an in-step change: the Japanese base falls
        // in this same step and its ship in port is sunk with the men
        // aboard, so unit 2 has no men by the time its day is chosen.
        if entry.side == Side::Japanese {
            continue;
        }
        let asked = before.side_view(entry.side).ask(&entry.question);
        assert_eq!(asked, Some(entry.answer.clone()), "{entry:?}");
        match &entry.answer {
            Answer::Ground(GroundAnswer::BackToDefend(true)) => back += 1,
            Answer::Ground(GroundAnswer::UnitDay(UnitDay::Rehearse(force))) => {
                assert_eq!(*force, tid(3));
                rehearse += 1;
            }
            _ => {}
        }
    }
    assert_eq!((back, rehearse), (1, 1), "{trace:?}");
    // The questions of the view, with the staff's default inputs, are
    // answered as the engine answered them here, where nothing marched
    // or fought.
    let ground_questions = allied
        .questions()
        .into_iter()
        .filter(|question| matches!(question, Question::Ground(_)));
    for question in ground_questions {
        let Some(Answer::Ground(answer)) = allied.ask(&question) else {
            panic!("a question the view lists has an answer: {question:?}");
        };
        // The engine asks what a spent attacker does only with enemy
        // fighting men in its hex, which there are not here: the view's
        // answer is the rule's on what the side holds (decision 0105).
        if let Question::Ground(GroundQuestion::AttackSpent { unit, hex }) = question {
            let land_unit = &before.truth.land_units[&unit];
            assert_eq!(
                answer,
                GroundAnswer::AttackSpent(when_attack_spent(
                    land_unit.when_spent,
                    fed_where_it_stands(&before.truth, land_unit.side, hex),
                ))
            );
            continue;
        }
        assert!(
            trace.iter().any(|entry| {
                entry.answer == Answer::Ground(answer.clone())
                    && matches!(
                        (&entry.question, &question),
                        (
                            Question::Ground(GroundQuestion::BackToDefend { unit: a, .. }),
                            Question::Ground(GroundQuestion::BackToDefend { unit: b, .. }),
                        ) | (
                            Question::Ground(GroundQuestion::UnitDay { unit: a, .. }),
                            Question::Ground(GroundQuestion::UnitDay { unit: b, .. }),
                        ) if a == b
                    )
            }),
            "{question:?} -> {answer:?}"
        );
    }
}

/// A march that stops, or a fight, changes `marched` and `contested`
/// inside the step: the question carries them, and the view answers with
/// them as given.
#[test]
fn a_marched_or_contested_unit_has_no_day_of_its_own_whatever_its_activity() {
    let world = ground_world();
    let view = world.side_view(Side::Allied);
    for (marched, contested, want) in [
        (true, false, UnitDay::Marched),
        (false, true, UnitDay::Contested),
        (false, false, UnitDay::Rehearse(tid(3))),
    ] {
        let answer = view.ask_ground(&GroundQuestion::UnitDay {
            unit: lid(3),
            marched,
            contested,
            rostered_with: Some(tid(3)),
        });
        assert_eq!(answer, Some(GroundAnswer::UnitDay(want)));
    }
    // Not the side's unit, or not the side's force: no answer.
    assert_eq!(
        view.ask_ground(&GroundQuestion::UnitDay {
            unit: lid(2),
            marched: false,
            contested: false,
            rostered_with: None
        }),
        None
    );
    assert_eq!(
        view.ask_ground(&GroundQuestion::UnitDay {
            unit: lid(3),
            marched: false,
            contested: false,
            rostered_with: Some(tid(1))
        }),
        None
    );
}

/// The default `contested` is true only when the side's own fighting
/// unit there is on attack and it holds an estimate of enemy men in the
/// hex.
#[test]
fn the_default_contested_needs_an_own_attack_and_an_estimate_of_enemy_men() {
    let mut world = ground_world();
    let contested = |world: &World| -> bool {
        world
            .side_view(Side::Allied)
            .ground_questions()
            .into_iter()
            .find_map(|question| match question {
                GroundQuestion::UnitDay {
                    unit, contested, ..
                } if unit == lid(1) => Some(contested),
                _ => None,
            })
            .expect("a question for unit 1")
    };
    assert!(!contested(&world));
    let figures = SearchFigures::from_search_toml();
    world
        .beliefs
        .get_mut(&Side::Allied)
        .expect("belief")
        .apply_report(
            &Report::Ground(GroundReport {
                hex: AT,
                men: 300,
                grade: crate::belief::GroundGrade::MenInContact,
                day: 0,
                received_event: EventId::new(1),
            }),
            &figures,
        );
    assert!(contested(&world));
    world
        .truth
        .land_units
        .get_mut(&lid(1))
        .expect("unit 1")
        .posture = Posture::Defend;
    assert!(!contested(&world));
}

/// Two worlds that differ only in what the Allied side cannot see (an
/// enemy unit's men ashore in its own hex, its force, its base's
/// stores) give the same questions and answers from the Allied view.
#[test]
fn the_enemys_men_ashore_change_no_answer_of_the_view_of_the_land_units() {
    let a = ground_world();
    let mut b = ground_world();
    let enemy = b.truth.land_units.get_mut(&lid(2)).expect("unit 2");
    enemy.men_ashore = 400;
    enemy.hex = Some(AT);
    enemy.posture = Posture::Attack;
    enemy.fatigue = n(70);
    b.truth.bases.get_mut(&bid(1)).expect("base 1").supply_t = n(1);
    b.truth
        .task_forces
        .get_mut(&tid(1))
        .expect("force 1")
        .from_hex = Hex::new(3, 1);
    let (va, vb) = (a.side_view(Side::Allied), b.side_view(Side::Allied));
    assert_eq!(va.questions(), vb.questions());
    for question in va.questions() {
        assert_eq!(va.ask(&question), vb.ask(&question), "{question:?}");
    }
}

// ---- Men falling back and a fight their own side starts (correction 1 of
// decision 0109), by hand-built worlds. Two coast hexes, FB_AT and FB_TO,
// joined by a land link; engagement ratio 3, the first figures otherwise.
// Japanese base 1 and Allied base 2 are at FB_AT, each with plenty of supply.

const FB_AT: Hex = Hex::new(0, 0);
const FB_TO: Hex = Hex::new(1, 0);

/// What a land unit is in a falling-back world: side, men, posture, whether
/// it is falling back, and its fed share.
struct FbUnit {
    id: u32,
    side: Side,
    men: u32,
    posture: Posture,
    falling_back: bool,
    fed_share: Fixed,
}

fn fb_unit(id: u32, side: Side, men: u32, posture: Posture) -> FbUnit {
    FbUnit {
        id,
        side,
        men,
        posture,
        falling_back: false,
        fed_share: Fixed::ONE,
    }
}

impl FbUnit {
    fn falling_back(self) -> FbUnit {
        FbUnit {
            falling_back: true,
            ..self
        }
    }

    fn unfed(self) -> FbUnit {
        FbUnit {
            fed_share: Fixed::ZERO,
            ..self
        }
    }
}

fn fb_world(units: Vec<FbUnit>) -> World {
    let mut map = Map::new();
    for q in -1..=2 {
        for r in -1..=1 {
            map.insert(Hex::new(q, r), HexData::open_sea());
        }
    }
    for hex in [FB_AT, FB_TO] {
        map.insert(
            hex,
            HexData {
                kind: crate::map::HexKind::Coast,
                ..HexData::open_sea()
            },
        );
    }
    map.add_land_link(FB_AT, FB_TO).expect("link");
    let mut ref_data = RefData::for_tests();
    ref_data.port_sizes.insert(
        0,
        PortSize {
            berths: 0,
            berth_rate_t_per_h: Fixed::ZERO,
            lighter_rate_t_per_h: n(20),
            fuel_berth_rate_t_per_h: None,
            fuel_lighter_rate_t_per_h: None,
            ammunition_berth_rate_t_per_h: None,
            ammunition_lighter_rate_t_per_h: None,
        },
    );
    ref_data.ground = Some(GroundFigures {
        engagement_ratio: n(3),
        ..GroundFigures::FIRST
    });
    ref_data.search = SearchFigures::from_search_toml();
    let mut truth = TruthState::default();
    for (id, side) in [(1, Side::Japanese), (2, Side::Allied)] {
        truth.bases.insert(
            bid(id),
            Base {
                supply_t: n(1000),
                ..Base::new(bid(id), format!("Base {id}"), side, FB_AT)
            },
        );
    }
    for unit in units {
        truth.land_units.insert(
            lid(unit.id),
            LandUnit {
                posture: unit.posture,
                fed_share: unit.fed_share,
                march: unit.falling_back.then_some(crate::state::March {
                    to: FB_TO,
                    days: 0,
                    counted: Fixed::ZERO,
                    cause: Cause::Event(crate::ids::EventId::new(1)),
                }),
                ..LandUnit::new(
                    lid(unit.id),
                    format!("Unit {}", unit.id),
                    unit.side,
                    unit.men,
                    Some(FB_AT),
                )
            },
        );
    }
    let world = World::new(7, ref_data, map, truth).expect("valid setup");
    assert!(invariants::check(&world.truth, &world.map, &world.ref_data).is_empty());
    world
}

/// `(firing men, shots, chance, hits)` of a unit's fire on day 1, if it
/// fired.
fn fb_fire(world: &World, id: u32) -> Option<(u32, Fixed, Fixed, u32)> {
    let found: Vec<_> = all_events(world)
        .into_iter()
        .filter_map(|e| match e.kind {
            EventKind::GroundFire {
                unit,
                firing_men,
                shots,
                chance,
                hits,
                ..
            } if unit == lid(id) => Some((firing_men, shots, chance, hits)),
            _ => None,
        })
        .collect();
    assert!(found.len() <= 1);
    found.first().copied()
}

/// The men a unit lost in the day's fight.
fn fb_lost(world: &World, id: u32) -> u32 {
    all_events(world)
        .into_iter()
        .filter_map(|e| match e.kind {
            EventKind::MenLostInFight { unit, lost, .. } if unit == lid(id) => Some(lost),
            _ => None,
        })
        .sum()
}

#[test]
fn a_fed_unit_falling_back_is_out_of_a_fight_its_own_side_starts() {
    // Japanese unit 3 attacks; Japanese unit 1, fed in full, is falling
    // back; Allied unit 2 defends with 6000 men.
    let mut world = fb_world(vec![
        fb_unit(1, Side::Japanese, 1000, Posture::Defend).falling_back(),
        fb_unit(2, Side::Allied, 6000, Posture::Defend),
        fb_unit(3, Side::Japanese, 1000, Posture::Attack),
    ]);
    world.fight_ashore(1);
    sound(&world);
    // Unit 1 fires nothing and loses nothing in the fight.
    assert_eq!(fb_fire(&world, 1), None);
    assert_eq!(fb_lost(&world, 1), 0);
    assert_eq!(world.truth.land_units[&lid(1)].men_ashore, 1000);
    // It is not counted for the limit: unit 2's 6000 firing men are limited
    // to 3 x 1000 (unit 3's men alone): 60 shots become 30.
    let (firing, shots, chance, hits_2) = fb_fire(&world, 2).expect("unit 2 fires");
    assert_eq!((firing, shots, chance), (6000, n(30), fx("0.6")));
    // Unit 3 fires its 1000 men: 10 shots at 0.3.
    let (firing, shots, chance, hits_3) = fb_fire(&world, 3).expect("unit 3 fires");
    assert_eq!((firing, shots, chance), (1000, n(10), fx("0.3")));
    // The whole loss falls on unit 3, and on unit 2 for unit 3's hits.
    assert_eq!(fb_lost(&world, 3), (20 * hits_2).min(1000));
    assert_eq!(fb_lost(&world, 2), 20 * hits_3);
}

#[test]
fn the_same_unit_is_fired_on_and_fires_when_the_other_side_is_on_attack() {
    // Control: Allied unit 2 attacks, Japanese unit 3 defends, Japanese
    // unit 1 is falling back and fed in full.
    let mut world = fb_world(vec![
        fb_unit(1, Side::Japanese, 1000, Posture::Defend).falling_back(),
        fb_unit(2, Side::Allied, 6000, Posture::Attack),
        fb_unit(3, Side::Japanese, 1000, Posture::Defend),
    ]);
    world.fight_ashore(1);
    sound(&world);
    // It is counted: the limit is 3 x 2000 = 6000, which does not cut unit
    // 2's 6000 firing men: 60 shots at 0.3.
    let (firing, shots, chance, hits_2) = fb_fire(&world, 2).expect("unit 2 fires");
    assert_eq!((firing, shots, chance), (6000, n(60), fx("0.3")));
    // It fires: 1000 men, 10 shots at 0.6, as unit 3 does.
    let (firing, shots, chance, _) = fb_fire(&world, 1).expect("unit 1 fires");
    assert_eq!((firing, shots, chance), (1000, n(10), fx("0.6")));
    let (firing, shots, chance, _) = fb_fire(&world, 3).expect("unit 3 fires");
    assert_eq!((firing, shots, chance), (1000, n(10), fx("0.6")));
    // It is fired on: the loss of 20 a hit is shared by units 1 and 3 in
    // proportion to their men, which are equal.
    assert!(hits_2 > 0);
    assert_eq!(fb_lost(&world, 1), 10 * hits_2);
    assert_eq!(fb_lost(&world, 3), 10 * hits_2);
}

/// What the Allied side is shown at the end of day 1, in full.
fn allied_view(world: &World) -> Vec<String> {
    let view = world.side_view(Side::Allied);
    view.events()
        .iter()
        .map(|e| {
            format!(
                "{} D{} {:?} {:?} {:?} {:?} {} {:?}",
                e.number,
                e.day,
                e.pulse,
                e.kind,
                e.order_number,
                e.causes,
                e.hidden_causes,
                e.breakdown
            )
        })
        .collect()
}

#[test]
fn the_enemys_view_is_the_same_whether_a_unit_is_falling_back_or_standing() {
    // Two worlds that differ only in whether Japanese unit 1, which fires
    // nothing (its fed share is 0), is falling back or standing. The Allied
    // unit 2 has 2000 firing men, below 3 x 1000, so its limit does not bind
    // with unit 1 counted or not: the same shots. Unit 3 fires the same.
    let build = |falling_back: bool| {
        let one = fb_unit(1, Side::Japanese, 1000, Posture::Defend).unfed();
        let one = if falling_back {
            one.falling_back()
        } else {
            one
        };
        let mut world = fb_world(vec![
            one,
            fb_unit(2, Side::Allied, 2000, Posture::Defend),
            fb_unit(3, Side::Japanese, 1000, Posture::Attack),
        ]);
        world.fight_ashore(1);
        world
    };
    let (falling, standing) = (build(true), build(false));
    // The hits of units 2 and 3 are the same in both worlds.
    assert_eq!(fb_fire(&falling, 1), None);
    assert_eq!(fb_fire(&standing, 1).map(|f| (f.0, f.1)), Some((0, n(0))));
    assert_eq!(fb_fire(&falling, 2), fb_fire(&standing, 2));
    assert_eq!(fb_fire(&falling, 3), fb_fire(&standing, 3));
    assert_eq!(fb_fire(&falling, 2).expect("fires").1, n(20));
    // The worlds do differ: the standing unit shares the loss of unit 2's
    // hits and the falling-back unit does not.
    assert_eq!(fb_lost(&falling, 1), 0);
    assert!(fb_lost(&standing, 1) > 0);
    let seen = allied_view(&falling);
    assert!(!seen.is_empty());
    assert_eq!(seen, allied_view(&standing));
}
