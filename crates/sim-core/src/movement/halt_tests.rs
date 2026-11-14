//! Halts at marked waypoints and steaming on (decision 0096): unit tests
//! on hand-built worlds. The hand calculation of the rule is in
//! `tests/scenario_26_act_and_steam_on.rs`; these are the cases the scenario
//! does not reach.
//!
//! The world is a strip of open sea from (-1,-1) to (9,1) with Allied base 1
//! (2000 t of supply) at (0,0) and Japanese task force 1 on the bombardment
//! mission at (4,0): one cruiser (8 guns at 3 rounds a minute, gun damage 20,
//! 800 rounds, 16 kn at cruise) on the route (0,0), (4,0), marked to act at
//! (0,0). 160 nm at 16 kn is 10 hours, so it reaches (0,0) in the night
//! pulse of day 1 and halts there. A test runs the phases of a pulse by hand
//! so that it can change the world between two pulses.

use crate::events::{Cause, EventKind};
use crate::fixed::Fixed;
use crate::hex::Hex;
use crate::ids::{BaseId, ShipClassId, ShipId, TaskForceId};
use crate::map::{HexData, Map};
use crate::orders::{
    Mission, OrderKind, OrderRequest, Origin, RejectReason, SpeedSetting, TaskForceOrder,
};
use crate::refdata::{
    AirCombatFigures, PortSize, RefData, SearchFigures, ShipClass, ShipKind, SurfaceCombatFigures,
};
use crate::resolve::PulsePhase;
use crate::state::{
    Base, HaltWork, Route, Ship, ShipLocation, Side, TaskForce, TaskForceStatus, TruthState,
};
use crate::time::Pulse;
use crate::world::World;
use std::collections::BTreeSet;

const AT: Hex = Hex::new(0, 0);
const OUT: Hex = Hex::new(4, 0);
const BOMBARD: HaltWork = HaltWork {
    bombard: true,
    cargo: false,
};

fn n(whole: i64) -> Fixed {
    Fixed::from_int(whole)
}

fn force() -> TaskForceId {
    TaskForceId::new(1)
}

fn cruiser() -> ShipClass {
    ShipClass {
        id: ShipClassId::new(1),
        name: "Test cruiser".to_string(),
        kind: ShipKind::Cruiser,
        cruise_speed_kn: n(16),
        max_speed_kn: n(32),
        fuel_capacity_t: n(2000),
        endurance_nm: n(8000),
        cargo_capacity_t: Fixed::ZERO,
        main_guns: 8,
        gun_rounds_per_min: n(3),
        gun_rounds_per_gun: 100,
        gun_damage: n(20),
        torpedo_tubes: 0,
        torpedo_tubes_one_side: 0,
        torpedo_damage: Fixed::ZERO,
        toughness: n(100),
        radar: false,
        aircraft_capacity: 0,
        anti_aircraft: 0,
        troop_capacity_men: 0,
    }
}

/// The world described in the module notes. `marks` are the hexes the route
/// is marked at.
fn world_marked(marks: &[Hex]) -> World {
    let mut map = Map::new();
    for q in -1..=9 {
        for r in -1..=1 {
            map.insert(Hex::new(q, r), HexData::open_sea());
        }
    }
    let mut ref_data = RefData::default();
    ref_data.ship_classes.insert(cruiser().id, cruiser());
    ref_data.port_sizes.insert(
        0,
        PortSize {
            berths: 0,
            berth_rate_t_per_h: Fixed::ZERO,
            lighter_rate_t_per_h: n(20),
        },
    );
    ref_data.search = Some(SearchFigures::from_search_toml());
    ref_data.surface_combat = Some(SurfaceCombatFigures::from_surface_combat_toml());
    ref_data.air_combat = Some(AirCombatFigures::from_air_combat_toml());

    let mut truth = TruthState::default();
    truth.bases.insert(
        BaseId::new(1),
        Base {
            supply_t: n(2000),
            ..Base::new(BaseId::new(1), "Base 1".to_string(), Side::Allied, AT)
        },
    );
    let mut task_force = TaskForce::at(
        force(),
        "Force 1".to_string(),
        Side::Japanese,
        OUT,
        Mission::Bombardment,
    );
    task_force.ships.insert(ShipId::new(1));
    task_force.route = Route {
        waypoints: vec![AT, OUT],
        next_index: 0,
        act_at: marks.iter().copied().collect::<BTreeSet<Hex>>(),
    };
    task_force.status = TaskForceStatus::Underway;
    truth.task_forces.insert(force(), task_force);
    truth.ships.insert(
        ShipId::new(1),
        Ship {
            fuel_t: n(1000),
            gun_rounds: n(800),
            ..Ship::new(
                ShipId::new(1),
                "Ship 1".to_string(),
                ShipClassId::new(1),
                Side::Japanese,
                ShipLocation::TaskForce(force()),
            )
        },
    );
    World::new(7, ref_data, map, truth).expect("valid setup")
}

fn world() -> World {
    world_marked(&[AT])
}

fn assert_sound(world: &World) {
    assert_eq!(
        crate::invariants::check(&world.truth, &world.map, &world.ref_data),
        vec![],
        "the invariants hold"
    );
}

/// The movement phase of a pulse only. The invariants are checked after it.
fn movement(world: &mut World, day: u32, pulse: Pulse) {
    world.run_pulse_phase(day, pulse, PulsePhase::Movement);
    assert_sound(world);
}

/// The phases of a pulse that matter here, in their order: movement, the
/// night's shore bombardment, loading. The invariants are checked after it.
fn whole_pulse(world: &mut World, day: u32, pulse: Pulse) {
    world.run_pulse_phase(day, pulse, PulsePhase::Movement);
    if pulse == Pulse::Night {
        world.run_pulse_phase(day, pulse, PulsePhase::ShoreBombardment);
    }
    world.run_pulse_phase(day, pulse, PulsePhase::Loading);
    assert_sound(world);
}

fn status(world: &World) -> TaskForceStatus {
    world.truth.task_forces[&force()].status
}

fn position(world: &World) -> Hex {
    world.truth.task_forces[&force()].position()
}

fn count(world: &World, pick: impl Fn(&EventKind) -> bool) -> usize {
    world.events().iter().filter(|e| pick(&e.kind)).count()
}

fn halts(world: &World) -> usize {
    count(world, |k| matches!(k, EventKind::TaskForceHalted { .. }))
}

fn steamings_on(world: &World) -> usize {
    count(world, |k| matches!(k, EventKind::TaskForceSteamedOn { .. }))
}

fn steamings(world: &World) -> usize {
    count(world, |k| matches!(k, EventKind::TaskForceSteamed { .. }))
}

fn request(order: TaskForceOrder) -> OrderRequest {
    OrderRequest {
        kind: OrderKind::TaskForce {
            target: force(),
            order,
        },
        origin: Origin::PLAYER,
    }
}

#[test]
fn a_force_reaches_the_marked_waypoint_halts_and_bombards() {
    // The check the others build on: the world halts at the first pulse,
    // fires, and each pulse leaves the invariants whole.
    let mut world = world();
    whole_pulse(&mut world, 1, Pulse::Night);
    assert_eq!(status(&world), TaskForceStatus::Halted);
    assert_eq!(position(&world), AT);
    let halt = world.truth.task_forces[&force()].halt.expect("a halt");
    assert_eq!(halt.work, BOMBARD);
    assert_eq!(halt.at_hours, n(10));
    assert!(halt.bombarded.is_some(), "it fired in the phase");
    assert_eq!(halts(&world), 1);
    assert_eq!(world.truth.ships[&ShipId::new(1)].gun_rounds, n(80));
    // The halt cites the steaming that reached it.
    let halted = world
        .events()
        .iter()
        .find(|e| matches!(e.kind, EventKind::TaskForceHalted { .. }))
        .expect("the halt");
    assert_eq!(halted.causes.len(), 1, "no route order was given");
    assert!(matches!(halted.causes[0], Cause::Event(_)));
}

#[test]
fn a_halted_force_with_no_round_left_steams_on_at_the_start_of_the_next_movement() {
    let mut world = world();
    // Night: it halts. The shore bombardment phase is not run, so it has not
    // fired; then its ship has no round left.
    movement(&mut world, 1, Pulse::Night);
    assert_eq!(status(&world), TaskForceStatus::Halted);
    assert_eq!(position(&world), AT);
    world
        .truth
        .ships
        .get_mut(&ShipId::new(1))
        .expect("the ship")
        .gun_rounds = Fixed::ZERO;
    assert_eq!(steamings_on(&world), 0);
    // Morning: at the start of its movement it steams on, having fired
    // nothing, and moves in the pulse.
    movement(&mut world, 1, Pulse::Morning);
    assert_eq!(steamings_on(&world), 1);
    let on = world
        .events()
        .iter()
        .find_map(|e| match &e.kind {
            EventKind::TaskForceSteamedOn {
                hex, work, fired, ..
            } => Some((*hex, *work, *fired, e.pulse)),
            _ => None,
        })
        .expect("a steaming on");
    assert_eq!(on, (AT, BOMBARD, false, Some(Pulse::Morning)));
    assert_eq!(status(&world), TaskForceStatus::Underway);
    assert!(world.truth.task_forces[&force()].halt.is_none());
    assert_ne!(position(&world), AT);
    // The steaming on is followed by its movement, with the pulse's 6 hours:
    // 96 nm.
    let steamed = world
        .events()
        .iter()
        .filter_map(|e| match &e.kind {
            EventKind::TaskForceSteamed {
                from, distance_nm, ..
            } if e.pulse == Some(Pulse::Morning) => Some((*from, *distance_nm)),
            _ => None,
        })
        .collect::<Vec<_>>();
    assert_eq!(steamed, vec![(AT, n(96))]);
    // The halt is cited, and the bombardment is not: it never happened.
    let steamed_on = world
        .events()
        .iter()
        .find(|e| matches!(e.kind, EventKind::TaskForceSteamedOn { .. }))
        .expect("the event");
    let halted = world
        .events()
        .iter()
        .find(|e| matches!(e.kind, EventKind::TaskForceHalted { .. }))
        .expect("the halt");
    assert_eq!(steamed_on.causes, vec![Cause::Event(halted.id)]);
}

#[test]
fn a_halted_force_whose_work_is_not_done_stays_halted_and_records_no_steaming() {
    let mut world = world();
    // It halts and has not yet fired: the bombardment phase of the night is
    // not run.
    movement(&mut world, 1, Pulse::Night);
    assert_eq!(steamings(&world), 1, "the steaming that reached the halt");
    for pulse in [Pulse::Morning, Pulse::Afternoon] {
        movement(&mut world, 1, pulse);
        assert_eq!(status(&world), TaskForceStatus::Halted, "{pulse:?}");
        assert_eq!(position(&world), AT);
    }
    movement(&mut world, 2, Pulse::Night);
    assert_eq!(status(&world), TaskForceStatus::Halted);
    assert_eq!(steamings_on(&world), 0);
    assert_eq!(steamings(&world), 1, "no steaming after the halt");
    assert_eq!(halts(&world), 1);
    assert!(world.truth.task_forces[&force()]
        .halt
        .expect("the halt")
        .bombarded
        .is_none());
    // Then the night's phase runs: it fires, and the next movement sends it on.
    world.run_pulse_phase(2, Pulse::Night, PulsePhase::ShoreBombardment);
    assert_sound(&world);
    assert!(world.truth.task_forces[&force()]
        .halt
        .expect("the halt")
        .bombarded
        .is_some());
    movement(&mut world, 2, Pulse::Morning);
    assert_eq!(steamings_on(&world), 1);
    assert_eq!(status(&world), TaskForceStatus::Underway);
}

#[test]
fn a_force_halted_with_no_ship_left_stays_halted_and_records_nothing() {
    let mut world = world();
    movement(&mut world, 1, Pulse::Night);
    assert_eq!(status(&world), TaskForceStatus::Halted);
    let before = world.events().len();
    // Its only ship is sunk.
    let force_mut = world
        .truth
        .task_forces
        .get_mut(&force())
        .expect("the force");
    force_mut.ships.clear();
    world
        .truth
        .ships
        .get_mut(&ShipId::new(1))
        .expect("the ship")
        .location = ShipLocation::Sunk;
    for (day, pulse) in [
        (1, Pulse::Morning),
        (1, Pulse::Afternoon),
        (2, Pulse::Night),
    ] {
        movement(&mut world, day, pulse);
        assert_eq!(status(&world), TaskForceStatus::Halted);
        assert_eq!(position(&world), AT);
    }
    assert_eq!(steamings_on(&world), 0);
    assert_eq!(
        world.events().len(),
        before,
        "nothing is recorded for a force with no ship"
    );
    assert!(world.truth.task_forces[&force()].halt.is_some());
}

#[test]
fn the_last_waypoint_is_never_marked() {
    let route = Route {
        waypoints: vec![Hex::new(0, 0), Hex::new(1, 0), Hex::new(2, 0)],
        next_index: 0,
        act_at: [Hex::new(0, 0), Hex::new(2, 0)].into_iter().collect(),
    };
    assert!(route.marked(0));
    assert!(!route.marked(1));
    assert!(
        !route.marked(2),
        "the last waypoint is marked in act_at but not marked"
    );
    assert!(!route.marked(3), "past the end of the route");
    // A route of one waypoint has no waypoint but the last.
    let one = Route {
        waypoints: vec![Hex::new(0, 0)],
        next_index: 0,
        act_at: [Hex::new(0, 0)].into_iter().collect(),
    };
    assert!(!one.marked(0));
    assert!(!Route::default().marked(0));
}

#[test]
fn a_mark_that_is_not_a_waypoint_is_refused() {
    let mut world = world();
    let (_, _, accepted) = world.take_in_order(
        1,
        None,
        Side::Japanese,
        request(TaskForceOrder::SetRoute {
            waypoints: vec![AT, OUT],
            act_at: [Hex::new(2, 0)].into_iter().collect(),
        }),
        Vec::new(),
    );
    assert!(!accepted);
    let reasons: Vec<RejectReason> = world
        .events()
        .iter()
        .filter_map(|e| match &e.kind {
            EventKind::OrderRejected { reason, .. } => Some(reason.clone()),
            _ => None,
        })
        .collect();
    assert_eq!(reasons, vec![RejectReason::MarkNotOnRoute(Hex::new(2, 0))]);
    // The route was not changed: the world's own mark stands.
    assert_eq!(
        world.truth.task_forces[&force()].route.act_at,
        [AT].into_iter().collect::<BTreeSet<Hex>>()
    );
    // A mark on a waypoint is accepted and is stored with the route.
    let (_, _, accepted) = world.take_in_order(
        1,
        None,
        Side::Japanese,
        request(TaskForceOrder::SetRoute {
            waypoints: vec![AT, OUT],
            act_at: [OUT].into_iter().collect(),
        }),
        Vec::new(),
    );
    assert!(accepted);
    assert_eq!(
        world.truth.task_forces[&force()].route.act_at,
        [OUT].into_iter().collect::<BTreeSet<Hex>>()
    );
    // And one with no mark clears it.
    let (_, _, accepted) = world.take_in_order(
        1,
        None,
        Side::Japanese,
        request(TaskForceOrder::route(vec![AT, OUT])),
        Vec::new(),
    );
    assert!(accepted);
    assert!(world.truth.task_forces[&force()].route.act_at.is_empty());
}

#[test]
fn a_new_route_order_ends_a_halt() {
    let mut world = world();
    movement(&mut world, 1, Pulse::Night);
    assert_eq!(status(&world), TaskForceStatus::Halted);
    let (_, _, accepted) = world.take_in_order(
        2,
        None,
        Side::Japanese,
        request(TaskForceOrder::route(vec![OUT])),
        Vec::new(),
    );
    assert!(accepted);
    assert_eq!(status(&world), TaskForceStatus::Underway);
    assert!(world.truth.task_forces[&force()].halt.is_none());
    assert_sound(&world);
    movement(&mut world, 2, Pulse::Night);
    assert_eq!(steamings_on(&world), 0, "the order ended the halt");
    assert_eq!(halts(&world), 1);
    assert_eq!(position(&world), OUT);
    assert_eq!(status(&world), TaskForceStatus::Arrived);
}

#[test]
fn a_force_with_no_work_at_the_mark_does_not_halt() {
    // The same route, but the force is not on the bombardment mission: it
    // has no work at (0,0) and passes it.
    let mut world = world();
    world
        .truth
        .task_forces
        .get_mut(&force())
        .expect("the force")
        .mission = Mission::SurfaceCombat;
    // (4,0) to (0,0) and back is 320 nm, 20 hours at 16 kn: it passes (0,0)
    // in the night and is home in the afternoon, having halted nowhere.
    whole_pulse(&mut world, 1, Pulse::Night);
    assert_eq!(status(&world), TaskForceStatus::Underway);
    whole_pulse(&mut world, 1, Pulse::Morning);
    whole_pulse(&mut world, 1, Pulse::Afternoon);
    assert_eq!(halts(&world), 0);
    assert_eq!(steamings_on(&world), 0);
    assert_eq!(status(&world), TaskForceStatus::Arrived);
    assert_eq!(position(&world), OUT);
}

#[test]
fn a_halted_force_at_knots_that_arrive_by_day_waits_for_night() {
    // At 10 kn the cruiser takes 16 hours: it halts in the morning of day 1.
    let mut world = world();
    world
        .truth
        .task_forces
        .get_mut(&force())
        .expect("the force")
        .speed = SpeedSetting::Knots(n(10));
    whole_pulse(&mut world, 1, Pulse::Night);
    assert_eq!(status(&world), TaskForceStatus::Underway);
    whole_pulse(&mut world, 1, Pulse::Morning);
    assert_eq!(status(&world), TaskForceStatus::Halted);
    assert!(world.truth.task_forces[&force()]
        .halt
        .expect("the halt")
        .bombarded
        .is_none());
    whole_pulse(&mut world, 1, Pulse::Afternoon);
    assert_eq!(status(&world), TaskForceStatus::Halted);
    assert_eq!(steamings_on(&world), 0);
    whole_pulse(&mut world, 2, Pulse::Night);
    assert!(world.truth.task_forces[&force()]
        .halt
        .expect("the halt")
        .bombarded
        .is_some());
    whole_pulse(&mut world, 2, Pulse::Morning);
    assert_eq!(steamings_on(&world), 1);
    assert_eq!(status(&world), TaskForceStatus::Underway);
}

// ---- The choices of the passage as questions (decision 0100) ----
//
// Each question is answered by the engine's own function, and a side asks it
// from its view. The hand cases below reason the answer from the world of the
// module notes; the leak test changes only what the side cannot see; the
// agreement tests run the engine with the trace on and set its answer beside
// the view's.

use crate::choices::movement::{MovementAnswer, MovementQuestion, Rerouted};
use crate::choices::{Answer, Moment, Question};
use crate::ids::OrderId;
use crate::orders::CargoAction;
use crate::state::CargoOrder;

fn ask(world: &World, side: Side, question: MovementQuestion) -> Option<MovementAnswer> {
    match world.side_view(side).ask(&Question::Movement(question))? {
        Answer::Movement(answer) => Some(answer),
        other => panic!("not a movement answer: {other:?}"),
    }
}

/// The traced movement choices, as `(question, answer)` pairs.
fn traced(world: &mut World) -> Vec<(MovementQuestion, MovementAnswer)> {
    world
        .take_choice_trace()
        .into_iter()
        .filter_map(|choice| match (choice.question, choice.answer) {
            (Question::Movement(q), Answer::Movement(a)) => Some((q, a)),
            _ => None,
        })
        .collect()
}

#[test]
fn the_force_would_halt_at_the_marked_waypoint_where_it_has_work() {
    // Marked at (0,0), on the bombardment mission, with the Allied base 1 on
    // the map in that hex and a cruiser with 800 rounds: it halts at index 0.
    let world = world();
    assert_eq!(
        ask(
            &world,
            Side::Japanese,
            MovementQuestion::Halt { force: force() }
        ),
        Some(MovementAnswer::Halt(Some(0)))
    );
    // The last waypoint is never marked, so a mark there is no halt.
    let world = world_marked(&[OUT]);
    assert_eq!(
        ask(
            &world,
            Side::Japanese,
            MovementQuestion::Halt { force: force() }
        ),
        Some(MovementAnswer::Halt(None))
    );
    // Not marked at all: no halt.
    let world = world_marked(&[]);
    assert_eq!(
        ask(
            &world,
            Side::Japanese,
            MovementQuestion::Halt { force: force() }
        ),
        Some(MovementAnswer::Halt(None))
    );
}

#[test]
fn an_enemy_base_off_the_map_gives_no_bombardment_work() {
    // A base formed by a landing is not on the map (decision 0066, 12): the
    // bombardment rule does not see it, and neither does the view.
    let mut world = world();
    world
        .truth
        .bases
        .get_mut(&BaseId::new(1))
        .expect("base 1")
        .on_map = false;
    assert_eq!(
        ask(
            &world,
            Side::Japanese,
            MovementQuestion::Halt { force: force() }
        ),
        Some(MovementAnswer::Halt(None))
    );
}

#[test]
fn a_force_with_cargo_to_load_at_its_own_base_would_halt_there() {
    // No bombardment mission, but a load order, the Japanese base 2 at (0,0)
    // with 500 t and a ship with room for 100 t: cargo work, so it halts.
    let mut world = world();
    world
        .ref_data
        .ship_classes
        .get_mut(&ShipClassId::new(1))
        .expect("class")
        .cargo_capacity_t = n(100);
    world.truth.bases.insert(
        BaseId::new(2),
        Base {
            supply_t: n(500),
            ..Base::new(BaseId::new(2), "Base 2".to_string(), Side::Japanese, AT)
        },
    );
    let task_force = world.truth.task_forces.get_mut(&force()).expect("force");
    task_force.mission = Mission::SurfaceCombat;
    task_force.cargo_order = Some(CargoOrder {
        action: CargoAction::Load,
        order: OrderId::new(1),
        unit: None,
    });
    assert_eq!(
        ask(
            &world,
            Side::Japanese,
            MovementQuestion::Halt { force: force() }
        ),
        Some(MovementAnswer::Halt(Some(0)))
    );
    // With the base empty there is nothing to load: no work, no halt.
    world
        .truth
        .bases
        .get_mut(&BaseId::new(2))
        .expect("base 2")
        .supply_t = Fixed::ZERO;
    assert_eq!(
        ask(
            &world,
            Side::Japanese,
            MovementQuestion::Halt { force: force() }
        ),
        Some(MovementAnswer::Halt(None))
    );
}

#[test]
fn a_halted_force_steams_on_once_it_has_bombarded() {
    // Halted for a bombardment: it stays until it has fired, then goes on.
    let mut world = world();
    movement(&mut world, 1, Pulse::Night);
    assert_eq!(status(&world), TaskForceStatus::Halted);
    let steam = MovementQuestion::SteamOn { force: force() };
    assert_eq!(
        ask(&world, Side::Japanese, steam.clone()),
        Some(MovementAnswer::SteamOn(false)),
        "it has not fired yet"
    );
    world.run_pulse_phase(1, Pulse::Night, PulsePhase::ShoreBombardment);
    assert_eq!(
        ask(&world, Side::Japanese, steam.clone()),
        Some(MovementAnswer::SteamOn(true)),
        "it has fired"
    );
    // The question is for a halted force only, and only the side's own.
    assert_eq!(ask(&world, Side::Allied, steam), None);
    let world = self::world();
    assert_eq!(
        ask(
            &world,
            Side::Japanese,
            MovementQuestion::SteamOn { force: force() }
        ),
        None,
        "a force that is under way has not halted"
    );
}

#[test]
fn a_new_route_turns_a_force_back_when_that_is_the_shorter_trip() {
    // Between (4,0) and (3,0), 10 of 40 nm out from (4,0): to a waypoint at
    // (5,0) the trip back is 10 nm and one hex, the trip ahead 30 nm and two
    // hexes, so it turns back and steers from (4,0). With no waypoint it goes
    // on to (3,0).
    let mut world = world();
    let task_force = world.truth.task_forces.get_mut(&force()).expect("force");
    task_force.from_hex = OUT;
    task_force.toward_hex = Some(Hex::new(3, 0));
    task_force.leg_nm = n(40);
    task_force.progress_nm = n(10);
    assert_eq!(
        ask(
            &world,
            Side::Japanese,
            MovementQuestion::Reroute {
                force: force(),
                first_waypoint: Some(Hex::new(5, 0)),
            }
        ),
        Some(MovementAnswer::Reroute(Some(Rerouted {
            turns_back: true,
            steers_from: OUT,
        })))
    );
    assert_eq!(
        ask(
            &world,
            Side::Japanese,
            MovementQuestion::Reroute {
                force: force(),
                first_waypoint: None,
            }
        ),
        Some(MovementAnswer::Reroute(Some(Rerouted {
            turns_back: false,
            steers_from: Hex::new(3, 0),
        })))
    );
}

#[test]
fn the_view_gives_the_engines_answers_to_halt_steam_on_and_reroute() {
    // Halt: asked before the movement phase, the answer is the one the engine
    // acted on at its start. Steam on: the same at the next pulse's movement.
    let mut world = world();
    world.trace_choices(true);
    world.pass_moment(Moment::DayBegins { day: 1 });
    let halt_q = MovementQuestion::Halt { force: force() };
    let before = ask(&world, Side::Japanese, halt_q.clone()).expect("an answer");
    whole_pulse(&mut world, 1, Pulse::Night);
    let first = traced(&mut world);
    assert_eq!(first.first(), Some(&(halt_q.clone(), before)));
    let steam_q = MovementQuestion::SteamOn { force: force() };
    let before = ask(&world, Side::Japanese, steam_q.clone()).expect("an answer");
    movement(&mut world, 1, Pulse::Morning);
    let second = traced(&mut world);
    assert!(
        second.contains(&(steam_q, before)),
        "the engine's steam-on answer is the view's: {second:?}"
    );

    // Reroute: a route order taken in turns the force back, and the traced
    // answer is the view's answer to the same question.
    let mut world = self::world();
    let task_force = world.truth.task_forces.get_mut(&force()).expect("force");
    task_force.from_hex = OUT;
    task_force.toward_hex = Some(Hex::new(3, 0));
    task_force.leg_nm = n(40);
    task_force.progress_nm = n(10);
    world.trace_choices(true);
    world.pass_moment(Moment::Orders { day: 1 });
    let reroute_q = MovementQuestion::Reroute {
        force: force(),
        first_waypoint: Some(Hex::new(5, 0)),
    };
    let before = ask(&world, Side::Japanese, reroute_q.clone()).expect("an answer");
    world.take_in_order(
        1,
        None,
        Side::Japanese,
        request(TaskForceOrder::route(vec![Hex::new(5, 0)])),
        Vec::new(),
    );
    assert_eq!(traced(&mut world), vec![(reroute_q, before)]);
}

/// The questions of every kind a side could ask, with their answers.
fn all_answers(world: &World, side: Side) -> Vec<(Question, Option<Answer>)> {
    let view = world.side_view(side);
    view.questions()
        .into_iter()
        .map(|question| {
            let answer = view.ask(&question);
            (question, answer)
        })
        .collect()
}

#[test]
fn what_the_side_cannot_see_changes_no_answer() {
    // Two worlds that differ only in what the Japanese cannot see: the Allied
    // base's stores, an Allied ship and force, and an Allied base off the
    // map (which the Japanese do not see, and which the bombardment rule does
    // not read). Every question of the Japanese gets the same answer. The
    // cargo case is added to both so the cargo test reads an own base too.
    let build = |extra: bool| {
        let mut world = world();
        if extra {
            let base = world.truth.bases.get_mut(&BaseId::new(1)).expect("base 1");
            base.supply_t = n(7);
            base.fuel_t = n(3);
            world.truth.bases.insert(
                BaseId::new(9),
                Base {
                    on_map: false,
                    supply_t: n(900),
                    ..Base::new(BaseId::new(9), "Hidden".to_string(), Side::Allied, OUT)
                },
            );
            world.truth.ships.insert(
                ShipId::new(2),
                Ship::new(
                    ShipId::new(2),
                    "Ship 2".to_string(),
                    ShipClassId::new(1),
                    Side::Allied,
                    ShipLocation::TaskForce(TaskForceId::new(2)),
                ),
            );
            let mut allied = TaskForce::at(
                TaskForceId::new(2),
                "Force 2".to_string(),
                Side::Allied,
                OUT,
                Mission::SurfaceCombat,
            );
            allied.ships.insert(ShipId::new(2));
            world.truth.task_forces.insert(TaskForceId::new(2), allied);
        }
        world
    };
    let plain = build(false);
    let more = build(true);
    let answers = all_answers(&plain, Side::Japanese);
    assert!(
        answers
            .iter()
            .any(|(q, _)| matches!(q, Question::Movement(MovementQuestion::Halt { .. }))),
        "the side has movement questions"
    );
    assert_eq!(answers, all_answers(&more, Side::Japanese));
    // And after the halt, with a steam-on question in the list.
    let (mut plain, mut more) = (build(false), build(true));
    for world in [&mut plain, &mut more] {
        movement(world, 1, Pulse::Night);
    }
    let answers = all_answers(&plain, Side::Japanese);
    assert!(answers
        .iter()
        .any(|(q, _)| matches!(q, Question::Movement(MovementQuestion::SteamOn { .. }))));
    assert_eq!(answers, all_answers(&more, Side::Japanese));
}
