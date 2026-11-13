//! Tests of ship repair (decision 0069). The hand calculation of the whole
//! rule is in `tests/scenario_20a_ship_repair.rs`; these are the cases the
//! scenario cannot reach, and the order's checks.

use super::*;
use crate::hex::Hex;
use crate::ids::{OrderId, ShipClassId};
use crate::map::{HexData, Map};
use crate::orders::{
    Mission, OrderKind, OrderRequest, Origin, Readiness, RejectReason, SpeedSetting, TaskForceOrder,
};
use crate::refdata::{PortSize, RefData, SearchFigures, ShipClass, ShipKind};
use crate::state::{Base, CargoOrder, Ship, ShipSystems, Side, TaskForce, TruthState};
use crate::world::World;

const HERE: Hex = Hex::new(0, 0);

fn tf(id: u32) -> TaskForceId {
    TaskForceId::new(id)
}

fn sid(id: u32) -> ShipId {
    ShipId::new(id)
}

fn fx(n: i64) -> Fixed {
    Fixed::from_int(n)
}

/// Class 1: a cruiser with four systems and eight tubes, four of them
/// bearing on one side (a wing-tubes class); 2,000 t of fuel, 800 gun rounds.
fn class() -> ShipClass {
    ShipClass {
        id: ShipClassId::new(1),
        name: "Cruiser".to_string(),
        kind: ShipKind::Cruiser,
        cruise_speed_kn: fx(16),
        max_speed_kn: fx(20),
        fuel_capacity_t: fx(2000),
        endurance_nm: fx(8000),
        cargo_capacity_t: Fixed::ZERO,
        main_guns: 8,
        gun_rounds_per_min: Fixed::ZERO,
        gun_rounds_per_gun: 100,
        gun_damage: Fixed::ZERO,
        torpedo_tubes: 8,
        torpedo_tubes_one_side: 4,
        torpedo_damage: Fixed::ZERO,
        toughness: fx(100),
        radar: false,
        aircraft_capacity: 0,
        anti_aircraft: 0,
        troop_capacity_men: 0,
    }
}

/// An Allied ship of class 1 in force `force`, with some fuel, gun rounds
/// and her torpedoes.
fn ship(id: u32, force: u32) -> Ship {
    Ship {
        fuel_t: fx(1000),
        gun_rounds: fx(300),
        torpedoes_ready: true,
        ..Ship::new(
            sid(id),
            format!("Ship {id}"),
            ShipClassId::new(1),
            Side::Allied,
            ShipLocation::TaskForce(tf(force)),
        )
    }
}

/// A base of the Allied side: its number, hex, points a day and passage.
fn base(id: u32, hex: Hex, points: i64, passage: Option<u32>) -> Base {
    Base {
        ship_repair_points_per_day: fx(points),
        yard_passage_days: passage,
        ..Base::new(BaseId::new(id), format!("Base {id}"), Side::Allied, hex)
    }
}

/// One force for `world`: its number, side, hex and ships.
type Force = (u32, Side, Hex, Vec<Ship>);

/// A world of open sea from (-1,-1) to (12,1) with the bases and forces given.
fn world(bases: Vec<Base>, forces: Vec<Force>) -> World {
    let mut map = Map::new();
    for q in -1..=12 {
        for r in -1..=1 {
            map.insert(Hex::new(q, r), HexData::open_sea());
        }
    }
    let mut ref_data = RefData::default();
    ref_data.ship_classes.insert(class().id, class());
    ref_data.port_sizes.insert(
        0,
        PortSize {
            berths: 0,
            berth_rate_t_per_h: Fixed::ZERO,
            lighter_rate_t_per_h: fx(20),
        },
    );
    ref_data.surface_combat = Some(SurfaceCombatFigures::from_surface_combat_toml());
    ref_data.search = Some(SearchFigures::from_search_toml());
    let mut truth = TruthState::default();
    for base in bases {
        truth.bases.insert(base.id, base);
    }
    for (id, side, hex, ships) in forces {
        let mut force = TaskForce::at(tf(id), format!("Force {id}"), side, hex, Mission::Transport);
        for ship in ships {
            force.ships.insert(ship.id);
            truth.ships.insert(ship.id, ship);
        }
        truth.task_forces.insert(tf(id), force);
    }
    World::new(7, ref_data, map, truth).expect("valid setup")
}

fn request(force: u32, order: TaskForceOrder) -> OrderRequest {
    OrderRequest {
        kind: OrderKind::TaskForce {
            target: tf(force),
            order,
        },
        origin: Origin::PLAYER,
    }
}

/// One `ShipRepaired` event: day, ship, base, points, fire, flotation,
/// systems restored, bank.
type Repaired = (
    u32,
    u32,
    u32,
    String,
    String,
    String,
    Vec<ShipSystem>,
    String,
);

fn repaired(world: &World) -> Vec<Repaired> {
    world
        .events()
        .iter()
        .filter_map(|event| match &event.kind {
            EventKind::ShipRepaired {
                ship,
                base,
                points,
                fire_damage,
                flotation_damage,
                restored,
                system_repair_points,
                ..
            } => Some((
                event.day,
                ship.raw(),
                base.raw(),
                points.to_string(),
                fire_damage.to_string(),
                flotation_damage.to_string(),
                restored.clone(),
                system_repair_points.to_string(),
            )),
            _ => None,
        })
        .collect()
}

/// The expected row of [`repaired`], with the quantities as text.
struct Row<'a> {
    day: u32,
    ship: u32,
    base: u32,
    points: &'a str,
    fire: &'a str,
    flotation: &'a str,
    restored: &'a [ShipSystem],
    bank: &'a str,
}

fn row(r: Row) -> Repaired {
    (
        r.day,
        r.ship,
        r.base,
        r.points.to_string(),
        r.fire.to_string(),
        r.flotation.to_string(),
        r.restored.to_vec(),
        r.bank.to_string(),
    )
}

fn day(world: &mut World) {
    world.resolve_day().expect("day resolves");
}

fn ship_of(world: &World, id: u32) -> &Ship {
    &world.umpire().ships[&sid(id)]
}

fn set_ship(world: &mut World, id: u32, change: impl Fn(&mut Ship)) {
    change(world.truth.ships.get_mut(&sid(id)).expect("ship"));
}

// ---- days of repair ----

#[test]
fn days_of_repair_are_rounded_up_to_a_whole_day_and_fire_adds_none() {
    let figures = SurfaceCombatFigures::from_surface_combat_toml();
    let with = |flotation: Fixed, systems: &[(ShipSystem, SystemState)]| {
        let mut ship = ship(1, 1);
        ship.flotation_damage = flotation;
        for (system, state) in systems {
            ship.systems.set(*system, *state);
        }
        days_of_repair(&ship, &figures)
    };
    let degraded = SystemState::Degraded;
    // 2 x 12.5 + 5 x 1 = 30, exactly; 2 x 3.25 + 5 = 11.5, up to 12.
    assert_eq!(
        with(
            Fixed::from_ratio(25, 2),
            &[(ShipSystem::FireControl, degraded)]
        ),
        30
    );
    assert_eq!(
        with(
            Fixed::from_ratio(13, 4),
            &[(ShipSystem::TorpedoMounts, degraded)]
        ),
        12
    );
    // 30 for a system out; a system degraded and one out add.
    assert_eq!(
        with(Fixed::ZERO, &[(ShipSystem::Propulsion, SystemState::Out)]),
        30
    );
    assert_eq!(
        with(
            Fixed::ZERO,
            &[
                (ShipSystem::Propulsion, SystemState::Out),
                (ShipSystem::MainBattery, degraded)
            ]
        ),
        35
    );
    // Nothing damaged is no days; a millionth of a point is a day (2 x
    // 0.000001 is 0.000002, a part of a day).
    assert_eq!(with(Fixed::ZERO, &[]), 0);
    assert_eq!(with(Fixed::from_raw(1), &[]), 1);
    // Fire damage adds nothing.
    let mut burning = ship(1, 1);
    burning.fire_damage = fx(60);
    assert_eq!(days_of_repair(&burning, &figures), 0);
    // The days away: the passage plus the largest days of repair.
    assert_eq!(days_away(4, &[(sid(3), 30), (sid(4), 12)]), 34);
    assert_eq!(days_away(4, &[]), 4);
}

// ---- forward repair ----

#[test]
fn forward_repair_spends_fire_then_flotation_then_the_first_degraded_system_and_banks_the_rest() {
    let mut damaged = ship(1, 1);
    damaged.fire_damage = fx(4);
    damaged.flotation_damage = fx(6);
    damaged.systems.fire_control = SystemState::Degraded;
    damaged.systems.main_battery = SystemState::Out;
    let mut world = world(
        vec![base(1, HERE, 6, None)],
        vec![(1, Side::Allied, HERE, vec![damaged])],
    );
    for _ in 0..5 {
        day(&mut world);
    }
    let none = [];
    let fire_control = [ShipSystem::FireControl];
    let at = |day, points, flotation, restored, bank| {
        row(Row {
            day,
            ship: 1,
            base: 1,
            points,
            fire: "0",
            flotation,
            restored,
            bank,
        })
    };
    assert_eq!(
        repaired(&world),
        vec![
            at(1, "6", "4", &none, "0"),
            at(2, "6", "0", &none, "2"),
            at(3, "6", "0", &none, "8"),
            at(4, "2", "0", &fire_control, "0"),
        ]
    );
    // The main battery is out and is never mended forward; nothing is
    // recorded from day 5.
    assert_eq!(ship_of(&world, 1).systems.main_battery, SystemState::Out);
    assert_eq!(
        ship_of(&world, 1).systems.fire_control,
        SystemState::Working
    );
}

#[test]
fn a_flotation_at_or_above_the_figure_is_not_mended_forward_and_the_event_says_so() {
    let mut damaged = ship(1, 1);
    damaged.fire_damage = fx(1);
    damaged.flotation_damage = fx(10);
    damaged.systems.propulsion = SystemState::Degraded;
    let mut world = world(
        vec![base(1, HERE, 6, None)],
        vec![(1, Side::Allied, HERE, vec![damaged])],
    );
    day(&mut world);
    // Fire 1, then the flotation of 10 is not below 10; propulsion banks 5.
    assert_eq!(
        repaired(&world),
        vec![row(Row {
            day: 1,
            ship: 1,
            base: 1,
            points: "6",
            fire: "0",
            flotation: "10",
            restored: &[],
            bank: "5"
        })]
    );
    let event = world
        .events()
        .iter()
        .find(|event| matches!(event.kind, EventKind::ShipRepaired { .. }))
        .expect("repaired");
    let breakdown = event.breakdown.as_ref().expect("breakdown");
    let named: Vec<(&str, String)> = breakdown
        .factors
        .iter()
        .map(|factor| (factor.name, factor.value.to_string()))
        .collect();
    assert_eq!(
        named,
        vec![
            ("allowance_before", "6".to_string()),
            ("fire_mended", "1".to_string()),
            ("flotation_mended", "0".to_string()),
            ("system_points_spent", "5".to_string()),
            ("allowance_after", "0".to_string()),
            ("forward_repair_max_flotation", "10".to_string()),
        ]
    );
    assert_eq!(breakdown.result, fx(6));
}

#[test]
fn banked_points_go_to_the_next_degraded_system_when_the_first_is_hit_out() {
    let mut damaged = ship(1, 1);
    damaged.systems.propulsion = SystemState::Degraded;
    damaged.systems.torpedo_mounts = SystemState::Degraded;
    let mut world = world(
        vec![base(1, HERE, 5, None)],
        vec![(1, Side::Allied, HERE, vec![damaged])],
    );
    day(&mut world);
    assert_eq!(ship_of(&world, 1).system_repair_points, fx(5));
    // A hit takes the propulsion from degraded to out.
    set_ship(&mut world, 1, |ship| {
        ship.systems.propulsion = SystemState::Out
    });
    day(&mut world);
    // The bank now belongs to the torpedo mounts: 10 - 5 = 5 are needed.
    assert_eq!(
        repaired(&world)[1],
        row(Row {
            day: 2,
            ship: 1,
            base: 1,
            points: "5",
            fire: "0",
            flotation: "0",
            restored: &[ShipSystem::TorpedoMounts],
            bank: "0"
        })
    );
    assert_eq!(ship_of(&world, 1).systems.propulsion, SystemState::Out);
}

#[test]
fn with_no_other_degraded_system_the_bank_is_kept_until_forward_repair_reaches_step_three() {
    let mut damaged = ship(1, 1);
    damaged.systems.propulsion = SystemState::Degraded;
    let mut world = world(
        vec![base(1, HERE, 4, None)],
        vec![(1, Side::Allied, HERE, vec![damaged])],
    );
    day(&mut world);
    assert_eq!(ship_of(&world, 1).system_repair_points, fx(4));
    set_ship(&mut world, 1, |ship| {
        ship.systems.propulsion = SystemState::Out
    });
    // The hit code is not touched: the bank is still there, and sound.
    assert_eq!(ship_of(&world, 1).system_repair_points, fx(4));
    day(&mut world);
    // Step three finds no degraded system, clears the bank and spends
    // nothing: the change is recorded as a `ShipRepaired` of 0 points.
    assert_eq!(ship_of(&world, 1).system_repair_points, Fixed::ZERO);
    let rows = repaired(&world);
    assert_eq!(rows.len(), 2);
    assert_eq!(
        rows[1],
        row(Row {
            day: 2,
            ship: 1,
            base: 1,
            points: "0",
            fire: "0",
            flotation: "0",
            restored: &[],
            bank: "0",
        })
    );
    day(&mut world);
    // Nothing changes after that, and nothing more is recorded.
    assert_eq!(repaired(&world).len(), 2);
    assert_eq!(ship_of(&world, 1).systems.propulsion, SystemState::Out);
}

#[test]
fn two_bases_in_one_hex_serve_ship_by_ship_the_lower_number_first() {
    let burning = |id| {
        let mut ship = ship(id, 1);
        ship.fire_damage = fx(8);
        ship
    };
    let mut world = world(
        vec![base(1, HERE, 6, None), base(2, HERE, 6, None)],
        vec![(
            1,
            Side::Allied,
            HERE,
            vec![burning(1), burning(2), burning(3)],
        )],
    );
    day(&mut world);
    // Ship 1 takes the 6 of base 1; ship 2, base 1 being spent, the 6 of
    // base 2; ship 3 finds nothing left.
    let spent = |day, ship, base, points, fire| {
        row(Row {
            day,
            ship,
            base,
            points,
            fire,
            flotation: "0",
            restored: &[],
            bank: "0",
        })
    };
    assert_eq!(
        repaired(&world),
        vec![spent(1, 1, 1, "6", "2"), spent(1, 2, 2, "6", "2")]
    );
    assert_eq!(ship_of(&world, 3).fire_damage, fx(8));
    day(&mut world);
    // Day 2: ships 1 and 2 need 2 each, from base 1; ship 3 gets the 2 left
    // of base 1 (a ship takes from one base), and base 2 is not reached.
    assert_eq!(
        repaired(&world)[2..],
        [
            spent(2, 1, 1, "2", "0"),
            spent(2, 2, 1, "2", "0"),
            spent(2, 3, 1, "2", "6"),
        ]
    );
}

#[test]
fn forward_repair_skips_a_force_under_way_with_a_leg_begun_empty_or_of_another_side() {
    let burning = |id, force| {
        let mut ship = ship(id, force);
        ship.fire_damage = fx(3);
        ship
    };
    let mut enemy = burning(5, 5);
    enemy.side = Side::Japanese;
    let mut world = world(
        vec![base(1, HERE, 100, None)],
        vec![
            (1, Side::Allied, HERE, vec![burning(1, 1)]),
            (2, Side::Allied, HERE, vec![burning(2, 2)]),
            (3, Side::Allied, HERE, vec![burning(3, 3)]),
            (4, Side::Allied, HERE, vec![]),
            (5, Side::Japanese, HERE, vec![enemy]),
        ],
    );
    // Force 2 is under way, force 3 has a leg begun.
    world
        .truth
        .task_forces
        .get_mut(&tf(2))
        .expect("force")
        .status = TaskForceStatus::Underway;
    let force_3 = world.truth.task_forces.get_mut(&tf(3)).expect("force");
    force_3.toward_hex = Some(Hex::new(1, 0));
    force_3.progress_nm = fx(5);
    world.repair_in_port(1);
    let who: Vec<u32> = repaired(&world).iter().map(|row| row.1).collect();
    assert_eq!(who, vec![1]);
}

// ---- the order ----

#[test]
fn the_yard_order_is_refused_for_the_first_reason_that_applies() {
    let mut enemy_ship = ship(9, 9);
    enemy_ship.side = Side::Japanese;
    let port = Hex::new(5, 0);
    let mut world = world(
        vec![
            base(1, HERE, 0, None),
            base(2, port, 0, Some(3)),
            // A second port in the same hex: the lowest number is the port.
            base(4, port, 0, Some(9)),
        ],
        vec![
            (1, Side::Allied, HERE, vec![ship(1, 1)]),
            (2, Side::Allied, Hex::new(2, 0), vec![ship(2, 2)]),
            (3, Side::Allied, port, vec![ship(3, 3)]),
            (4, Side::Allied, port, vec![]),
            (5, Side::Allied, Hex::new(2, 0), vec![ship(5, 5)]),
            (6, Side::Allied, port, vec![ship(6, 6)]),
            (7, Side::Allied, Hex::new(2, 0), vec![ship(7, 7)]),
            (9, Side::Japanese, port, vec![enemy_ship]),
        ],
    );
    let check = |world: &World, force| {
        world.check_order(Side::Allied, &request(force, TaskForceOrder::SendToYard))
    };
    // Another side's force, and one that does not exist.
    assert_eq!(check(&world, 9), Err(RejectReason::NotYourUnit));
    assert_eq!(check(&world, 40), Err(RejectReason::NotYourUnit));
    // No ship afloat.
    assert_eq!(check(&world, 4), Err(RejectReason::NoShipAfloat));
    // Under way, or with a leg begun, wherever it is: the stopped test comes
    // before the base tests (force 5 is in a hex with no base).
    world
        .truth
        .task_forces
        .get_mut(&tf(5))
        .expect("force")
        .status = TaskForceStatus::Underway;
    assert_eq!(check(&world, 5), Err(RejectReason::UnderWay));
    {
        let force = world.truth.task_forces.get_mut(&tf(7)).expect("force");
        force.toward_hex = Some(Hex::new(3, 0));
        force.progress_nm = fx(5);
    }
    assert_eq!(check(&world, 7), Err(RejectReason::UnderWay));
    // No base of the side here, then a base that is not a port.
    assert_eq!(check(&world, 2), Err(RejectReason::NoOwnBaseHere));
    assert_eq!(check(&world, 1), Err(RejectReason::NotAYardPort));
    // A force out of fuel at a centre counts as stopped.
    world
        .truth
        .task_forces
        .get_mut(&tf(6))
        .expect("force")
        .status = TaskForceStatus::OutOfFuel;
    assert_eq!(check(&world, 6), Ok(()));
    assert_eq!(check(&world, 3), Ok(()));
    // The port is the lowest-numbered of the two in the hex.
    let plan = world.yard_plan(tf(3)).expect("a plan");
    assert_eq!((plan.base, plan.passage_days), (BaseId::new(2), 3));
    // Take the order in; a force away then takes no order at all.
    world.submit_orders(Side::Allied, vec![request(3, TaskForceOrder::SendToYard)]);
    day(&mut world);
    // AwayAtYard is the first test after NotYourUnit of every order, ahead of
    // "no ship afloat", which a force away would otherwise fail.
    for order in [
        TaskForceOrder::SendToYard,
        TaskForceOrder::SetSpeed(SpeedSetting::Full),
        TaskForceOrder::SetReadiness(Readiness::Battle),
        TaskForceOrder::SetRoute(vec![Hex::new(6, 0)]),
        TaskForceOrder::DetachShips(vec![sid(3)]),
    ] {
        assert_eq!(
            world.check_order(Side::Allied, &request(3, order.clone())),
            Err(RejectReason::AwayAtYard),
            "{order:?}"
        );
    }
    assert_eq!(
        world.check_order(Side::Japanese, &request(3, TaskForceOrder::SendToYard)),
        Err(RejectReason::NotYourUnit)
    );
}

// ---- the trip, there and back ----

#[test]
fn a_force_leaves_for_the_yard_is_away_and_returns_repaired_refuelled_and_rearmed() {
    // Ship 1 of the wing-tubes class has fired one side and has a bank;
    // ship 2 has spent her torpedoes.
    let mut first = ship(1, 1);
    first.flotation_damage = fx(2);
    first.fire_damage = fx(5);
    first.systems.propulsion = SystemState::Degraded;
    first.system_repair_points = fx(3);
    first.torpedoes_one_side_fired = true;
    first.experience = 4;
    let mut second = ship(2, 1);
    second.flotation_damage = fx(1);
    second.torpedoes_ready = false;
    second.gun_rounds = fx(10);
    // The base mends 6 points a day and is a port for the yard with a
    // passage of 1: ship 1 needs ceil(2 x 2 + 5) = 9 days, ship 2 ceil(2) = 2.
    let mut world = world(
        vec![base(1, HERE, 6, Some(1))],
        vec![(1, Side::Allied, HERE, vec![first, second])],
    );
    {
        let force = world.truth.task_forces.get_mut(&tf(1)).expect("force");
        force.fatigue = fx(40);
        force.readiness = Readiness::Battle;
        force.speed = SpeedSetting::Full;
        force.home_port = Some(BaseId::new(1));
        force.mission = Mission::SurfaceCombat;
        force.alerted = true;
        force.cargo_order = Some(CargoOrder {
            action: crate::orders::CargoAction::Unload,
            order: OrderId::new(77),
            unit: None,
        });
        force.rules_of_engagement.break_off = Fixed::from_ratio(1, 4);
    }
    world.submit_orders(Side::Allied, vec![request(1, TaskForceOrder::SendToYard)]);
    day(&mut world);

    // Day 1: accepted, and the follow-up says everything.
    let left = world
        .events()
        .iter()
        .find(|event| matches!(event.kind, EventKind::ForceLeftForYard { .. }))
        .expect("the force left")
        .clone();
    assert_eq!(
        left.kind,
        EventKind::ForceLeftForYard {
            task_force: tf(1),
            base: BaseId::new(1),
            passage_days: 1,
            ships: vec![(sid(1), 9), (sid(2), 2)],
            days_away: 10,
            return_day: 11,
        }
    );
    assert_eq!(
        left.kind.to_string(),
        "task_force:1 left base:1 for the yard: ship:1 9 days, ship:2 2 days, passage 1, back on day 11"
    );
    assert_eq!(left.causes, vec![Cause::Order(OrderId::new(1))]);
    let named: Vec<(&str, String)> = left
        .breakdown
        .as_ref()
        .expect("breakdown")
        .factors
        .iter()
        .map(|factor| (factor.name, factor.value.to_string()))
        .collect();
    assert_eq!(
        named,
        vec![
            ("passage_days", "1".to_string()),
            ("flotation_damage", "2".to_string()),
            ("systems_degraded", "1".to_string()),
            ("systems_out", "0".to_string()),
            ("days_of_repair", "9".to_string()),
            ("flotation_damage", "1".to_string()),
            ("systems_degraded", "0".to_string()),
            ("systems_out", "0".to_string()),
            ("days_of_repair", "2".to_string()),
            ("days_away", "10".to_string()),
        ]
    );
    {
        let force = &world.umpire().task_forces[&tf(1)];
        assert!(force.ships.is_empty());
        assert_eq!(
            force.yard_trip,
            Some(YardTrip {
                base: BaseId::new(1),
                return_day: 11,
                departure: left.id,
            })
        );
        assert_eq!(force.status, TaskForceStatus::Idle);
        assert!(force.route.waypoints.is_empty() && force.route_order.is_none());
        assert!(force.cargo_order.is_none());
        assert!(!force.alerted && force.alerted_by.is_none());
        assert_eq!((force.toward_hex, force.progress_nm), (None, Fixed::ZERO));
        // Standing orders and fatigue are kept while away.
        assert_eq!(force.fatigue, fx(40));
        assert_eq!(force.readiness, Readiness::Battle);
    }
    for id in [1, 2] {
        assert_eq!(ship_of(&world, id).location, ShipLocation::Yard(tf(1)));
    }
    // No repair is made at the base for a force away, and its ships keep the
    // damage they left with.
    assert!(repaired(&world).is_empty());
    assert_eq!(ship_of(&world, 1).fire_damage, fx(5));

    // An order to it is refused at intake while it is away.
    world.submit_orders(
        Side::Allied,
        vec![request(1, TaskForceOrder::SetSpeed(SpeedSetting::Cruise))],
    );
    day(&mut world);
    assert!(world.events().iter().any(|event| matches!(
        event.kind,
        EventKind::OrderRejected {
            reason: RejectReason::AwayAtYard,
            ..
        }
    )));
    // Days 3 to 9: still away. After day 9 resolves the next order day is
    // 10, before the return.
    for _ in 3..=9 {
        day(&mut world);
    }
    let set_speed = request(1, TaskForceOrder::SetSpeed(SpeedSetting::Cruise));
    assert_eq!(
        world.check_order(Side::Allied, &set_speed),
        Err(RejectReason::AwayAtYard)
    );
    day(&mut world);
    // Day 10 resolved: the order's day would be 11, the day of return, and
    // the order is checked as the force will then be, with its ships.
    assert_eq!(world.check_order(Side::Allied, &set_speed), Ok(()));
    assert_eq!(
        world.check_order(
            Side::Allied,
            &request(1, TaskForceOrder::DetachShips(vec![sid(2)]))
        ),
        Ok(())
    );
    assert_eq!(
        world.check_order(Side::Allied, &request(1, TaskForceOrder::SendToYard)),
        Ok(())
    );
    assert!(world.umpire().task_forces[&tf(1)].ships.is_empty());
    assert!(!world
        .events()
        .iter()
        .any(|e| matches!(e.kind, EventKind::ForceReturnedFromYard { .. })));

    // Day 11: back, first of the day's steps.
    day(&mut world);
    let back = world
        .events()
        .iter()
        .find(|event| matches!(event.kind, EventKind::ForceReturnedFromYard { .. }))
        .expect("the force returned")
        .clone();
    assert_eq!(back.day, 11);
    assert_eq!(
        back.kind,
        EventKind::ForceReturnedFromYard {
            task_force: tf(1),
            base: BaseId::new(1),
            ships: vec![sid(1), sid(2)],
        }
    );
    assert_eq!(back.causes, vec![Cause::Event(left.id)]);
    assert_eq!(
        back.kind.to_string(),
        "task_force:1 back at base:1 from the yard, ship:1 and ship:2 repaired, refuelled and rearmed"
    );
    let first_of_day = world
        .events()
        .iter()
        .find(|event| event.day == 11)
        .map(|event| event.id);
    assert_eq!(first_of_day, Some(back.id));
    for id in [1, 2] {
        let ship = ship_of(&world, id);
        assert_eq!(ship.location, ShipLocation::TaskForce(tf(1)));
        assert_eq!(
            (ship.flotation_damage, ship.fire_damage),
            (Fixed::ZERO, Fixed::ZERO)
        );
        assert_eq!(ship.systems, ShipSystems::ALL_WORKING);
        assert_eq!(ship.system_repair_points, Fixed::ZERO);
        assert_eq!(ship.fuel_t, fx(2000));
        assert_eq!(ship.gun_rounds, fx(800));
        assert!(ship.torpedoes_ready);
        assert!(!ship.torpedoes_one_side_fired);
    }
    // Experience is kept.
    assert_eq!(ship_of(&world, 1).experience, 4);
    let force = &world.umpire().task_forces[&tf(1)];
    assert_eq!(
        force.ships.iter().map(|s| s.raw()).collect::<Vec<_>>(),
        vec![1, 2]
    );
    assert_eq!(force.yard_trip, None);
    assert_eq!(force.status, TaskForceStatus::Idle);
    assert_eq!(force.fatigue, Fixed::ZERO);
    assert!(!force.alerted);
    assert_eq!(force.arrival_event, Some(back.id));
    assert_eq!(force.arrived_at_hours, None);
    // Standing orders as they were.
    assert_eq!(force.readiness, Readiness::Battle);
    assert_eq!(force.speed, SpeedSetting::Full);
    assert_eq!(force.mission, Mission::SurfaceCombat);
    assert_eq!(force.home_port, Some(BaseId::new(1)));
    assert_eq!(force.rules_of_engagement.break_off, Fixed::from_ratio(1, 4));
    // Nothing was mended forward: the ships came back whole.
    assert!(repaired(&world).is_empty());
}
