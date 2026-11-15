//! Tests of command intake (decision 0107, step 1 of stage 1): each command
//! of the commander and of the staff, each reason for a refusal, the order of
//! the day, and what a side may not learn. The staffs here are scripted: they
//! return fixed decisions or compute them from the view, with no doctrine.

use std::cell::Cell;
use std::collections::{BTreeMap, BTreeSet};

use super::*;
use crate::events::{Cause, Event, EventKind};
use crate::hex::Hex;
use crate::ids::{BaseId, IntentId, OrderId, ShipClassId, ShipId, TaskForceId};
use crate::map::{HexData, Map};
use crate::orders::{
    BaseOrder, Cargo, Mission, OrderKind, OrderRequest, Origin, RejectReason, SpeedSetting,
    TaskForceOrder,
};
use crate::refdata::{PortSize, RefData, ShipClass, ShipKind};
use crate::state::{Base, Ship, ShipLocation, TaskForce, TruthState};
use crate::world::World;

// ---------------------------------------------------------------------------
// The world.
// ---------------------------------------------------------------------------

const A: Side = Side::Allied;
const J: Side = Side::Japanese;
const HOME: Hex = Hex::new(0, 0);
const OUT: Hex = Hex::new(3, 0);
const FAR: Hex = Hex::new(10, 0);

fn tf(n: u32) -> TaskForceId {
    TaskForceId::new(n)
}
fn ship(n: u32) -> ShipId {
    ShipId::new(n)
}
fn base(n: u32) -> BaseId {
    BaseId::new(n)
}
fn intent(n: u32) -> IntentId {
    IntentId::new(n)
}
fn plan(n: u32) -> PlanId {
    PlanId::new(n)
}
fn t(n: i64) -> Fixed {
    Fixed::from_int(n)
}
fn ships(list: &[u32]) -> BTreeSet<ShipId> {
    list.iter().map(|n| ship(*n)).collect()
}

/// The Allied side: base 1 at HOME (1,000 t of supply, 500 t of fuel), base 2
/// at OUT (100 t of supply); force 1 (ships 1 and 2, 1,900 t of fuel each) at
/// HOME, force 2 (ship 3) at OUT; ships 5 and 6 lie in port at base 1 in no
/// force. The Japanese side: base 9 and force 20 (ship 20) at FAR, ship 21 in
/// port at base 9. One cargo class, 3,000 t of cargo and 2,000 t of fuel.
fn world() -> World {
    let mut map = Map::new();
    for q in -1..=16 {
        for r in -1..=1 {
            map.insert(Hex::new(q, r), HexData::open_sea());
        }
    }
    let class = ShipClass {
        id: ShipClassId::new(1),
        name: "Cargo ship".to_string(),
        kind: ShipKind::Transport,
        cruise_speed_kn: t(12),
        max_speed_kn: t(16),
        fuel_capacity_t: t(2000),
        endurance_nm: t(8000),
        cargo_capacity_t: t(3000),
        main_guns: 0,
        gun_rounds_per_min: Fixed::ZERO,
        gun_rounds_per_gun: 0,
        gun_damage: Fixed::ZERO,
        torpedo_tubes: 0,
        torpedo_tubes_one_side: 0,
        torpedo_damage: Fixed::ZERO,
        toughness: t(100),
        radar: false,
        aircraft_capacity: 0,
        anti_aircraft: 0,
        troop_capacity_men: 0,
    };
    let mut ref_data = RefData::default();
    ref_data.ship_classes.insert(class.id, class);
    ref_data.port_sizes.insert(
        0,
        PortSize {
            berths: 0,
            berth_rate_t_per_h: Fixed::ZERO,
            lighter_rate_t_per_h: t(20),
        },
    );
    let mut truth = TruthState::default();
    let add_ship = |truth: &mut TruthState, n: u32, side: Side, location: ShipLocation| {
        truth.ships.insert(
            ship(n),
            Ship {
                fuel_t: t(1900),
                ..Ship::new(
                    ship(n),
                    format!("Ship {n}"),
                    ShipClassId::new(1),
                    side,
                    location,
                )
            },
        );
    };
    for (force, side, hex, members) in [
        (1, A, HOME, vec![1, 2]),
        (2, A, OUT, vec![3]),
        (20, J, FAR, vec![20]),
    ] {
        let mut task_force = TaskForce::at(
            tf(force),
            format!("Force {force}"),
            side,
            hex,
            Mission::Transport,
        );
        for member in members {
            task_force.ships.insert(ship(member));
            add_ship(&mut truth, member, side, ShipLocation::TaskForce(tf(force)));
        }
        truth.task_forces.insert(tf(force), task_force);
    }
    add_ship(&mut truth, 5, A, ShipLocation::Port(base(1)));
    add_ship(&mut truth, 6, A, ShipLocation::Port(base(1)));
    add_ship(&mut truth, 21, J, ShipLocation::Port(base(9)));
    for (id, side, hex, supply, fuel) in [
        (1, A, HOME, 1000, 500),
        (2, A, OUT, 100, 0),
        (9, J, FAR, 100, 100),
    ] {
        truth.bases.insert(
            base(id),
            Base {
                supply_t: t(supply),
                fuel_t: t(fuel),
                ..Base::new(base(id), format!("Base {id}"), side, hex)
            },
        );
    }
    World::new(7, ref_data, map, truth).expect("valid setup")
}

/// Hands one command in at once, as `issuer`, for the coming day.
fn give(
    world: &mut World,
    side: Side,
    issuer: Issuer,
    kind: CommandKind,
) -> Result<(), CommandRefusal> {
    let before = world.events.len();
    let day = world.truth.day + 1;
    world.take_in_command(day, side, Command { kind, issuer });
    let first = world.events.from_index(before)[0].clone();
    assert!(first.causes.is_empty(), "the root record has no causes");
    match first.kind {
        EventKind::CommandTaken { .. } => Ok(()),
        EventKind::CommandRefused { reason, .. } => Err(reason),
        other => panic!("the first event is the command's record, not {other:?}"),
    }
}

fn player(world: &mut World, kind: CommandKind) -> Result<(), CommandRefusal> {
    give(world, A, Issuer::Player, kind)
}

fn staff(world: &mut World, kind: CommandKind) -> Result<(), CommandRefusal> {
    give(world, A, Issuer::Staff, kind)
}

/// Asserts that `kind` is refused with `reason` and that nothing changed but
/// the one event that records the refusal.
fn refused(world: &mut World, issuer: Issuer, kind: CommandKind, reason: CommandRefusal) {
    let command_before = world.command.clone();
    let truth_before = format!("{:?}", world.truth);
    let events_before = world.events.len();
    assert_eq!(give(world, A, issuer, kind.clone()), Err(reason), "{kind}");
    assert_eq!(
        world.command, command_before,
        "{kind} changed the command state"
    );
    assert_eq!(
        format!("{:?}", world.truth),
        truth_before,
        "{kind} changed the truth"
    );
    assert_eq!(
        world.events.len(),
        events_before + 1,
        "{kind} recorded more than its refusal"
    );
}

fn keep_supplied(base_id: u32) -> IntentKind {
    IntentKind::KeepSupplied {
        base: base(base_id),
        floor_days: 10,
        interval_days: Some(15),
    }
}

fn set_intent(n: u32) -> CommandKind {
    CommandKind::SetIntent {
        intent: intent(n),
        kind: keep_supplied(1),
    }
}

fn forces(ship_list: &[u32], force_list: &[u32]) -> ForcesLimit {
    ForcesLimit {
        ships: ships(ship_list),
        task_forces: force_list.iter().map(|n| tf(*n)).collect(),
        source: base(1),
    }
}

fn set_forces(n: u32, limit: ForcesLimit) -> CommandKind {
    CommandKind::SetLimit(Limit::Forces {
        intent: intent(n),
        limit,
    })
}

fn state(world: &World, side: Side) -> &CommandState {
    world.command_state(side)
}

/// A world where intent 1 (ships 5 and 6 and force 1, source base 1) has
/// plan 1 released.
fn world_with_plan(released: bool) -> World {
    let mut world = world();
    player(&mut world, set_intent(1)).unwrap();
    player(&mut world, set_forces(1, forces(&[5, 6], &[1]))).unwrap();
    player(&mut world, CommandKind::Approve { plan: plan(1) }).unwrap();
    if released {
        player(&mut world, CommandKind::Approve { plan: plan(1) }).unwrap();
    }
    world
}

fn run_spec(ship_list: &[u32], days: [u32; 4]) -> RunSpec {
    RunSpec {
        ships: ships(ship_list),
        load_t: t(300),
        days: RunDays {
            load: days[0],
            sail: days[1],
            arrive: days[2],
            free: days[3],
        },
    }
}

fn log(world: &World) -> Vec<Event> {
    world.events.iter().cloned().collect()
}

fn texts(events: &[crate::side_events::SideEvent]) -> Vec<String> {
    events.iter().map(|event| event.to_string()).collect()
}

// ---------------------------------------------------------------------------
// The scripted staffs.
// ---------------------------------------------------------------------------

/// A staff that gives fixed decisions.
struct Fixed_(StaffDecisions);

impl Staff for Fixed_ {
    fn decide(&self, _view: SideView<'_>) -> StaffDecisions {
        self.0.clone()
    }
}

/// A staff that computes its decisions from the view.
struct Computed<F>(F);

impl<F: Fn(SideView<'_>) -> StaffDecisions> Staff for Computed<F> {
    fn decide(&self, view: SideView<'_>) -> StaffDecisions {
        (self.0)(view)
    }
}

fn reason(intent_n: u32, worked: &str) -> Reason {
    Reason {
        serves: Serves {
            intent: Some(intent(intent_n)),
            plan: Some(plan(1)),
            amendment: None,
        },
        evidence: vec![Evidence {
            what: "a stock".to_string(),
            day: 1,
        }],
        binding: vec!["a limit".to_string()],
        worked: worked.to_string(),
        would_change: vec!["more stock".to_string()],
    }
}

fn staff_order(kind: OrderKind, intent_n: u32, worked: &str) -> StaffOrder {
    StaffOrder {
        kind,
        intent: intent(intent_n),
        reason: reason(intent_n, worked),
    }
}

fn speed(target: u32) -> OrderKind {
    OrderKind::TaskForce {
        target: tf(target),
        order: TaskForceOrder::SetSpeed(SpeedSetting::Cruise),
    }
}

// ---------------------------------------------------------------------------
// The commander's commands.
// ---------------------------------------------------------------------------

#[test]
fn a_new_intent_is_taken_in_and_its_plan_drafted_citing_the_command() {
    let mut world = world();
    player(&mut world, set_intent(1)).unwrap();
    let command = state(&world, A);
    let found = &command.intents[&intent(1)];
    assert_eq!(found.kind, keep_supplied(1));
    assert_eq!(found.set_day, 1);
    assert_eq!(found.plan, plan(1));
    assert_eq!(found.withdrawn_day, None);
    assert_eq!(found.limits, IntentLimits::default());
    let drafted = &command.plans[&plan(1)];
    assert_eq!(drafted.state, PlanState::Draft);
    assert_eq!(drafted.baseline, None);
    assert_eq!(command.next_plan, 2);
    let events = log(&world);
    assert_eq!(events.len(), 2);
    assert!(matches!(events[0].kind, EventKind::CommandTaken { .. }));
    assert_eq!(
        events[1].kind,
        EventKind::PlanDrafted {
            side: A,
            plan: plan(1),
            intent: intent(1)
        }
    );
    assert_eq!(events[1].causes, vec![Cause::Event(events[0].id)]);
    assert_eq!(events[0].pulse, None);
    // The enemy's state is untouched.
    assert!(state(&world, J).is_empty());
}

#[test]
fn an_intent_is_refused_for_a_bad_number_base_or_days_and_nothing_changes() {
    let mut world = world();
    let bad = |n: u32, base_id: u32, floor: u32, interval: Option<u32>| CommandKind::SetIntent {
        intent: intent(n),
        kind: IntentKind::KeepSupplied {
            base: base(base_id),
            floor_days: floor,
            interval_days: interval,
        },
    };
    refused(
        &mut world,
        Issuer::Player,
        bad(0, 1, 10, None),
        CommandRefusal::IntentNumberNotUsable(intent(0)),
    );
    // A base of the other side and a base that does not exist are alike.
    refused(
        &mut world,
        Issuer::Player,
        bad(1, 9, 10, None),
        CommandRefusal::NotYourBase(base(9)),
    );
    refused(
        &mut world,
        Issuer::Player,
        bad(1, 77, 10, None),
        CommandRefusal::NotYourBase(base(77)),
    );
    refused(
        &mut world,
        Issuer::Player,
        bad(1, 1, 0, None),
        CommandRefusal::NoDays,
    );
    refused(
        &mut world,
        Issuer::Player,
        bad(1, 1, 10, Some(0)),
        CommandRefusal::NoDays,
    );
    // A withdrawn number is never used again.
    player(&mut world, set_intent(1)).unwrap();
    player(
        &mut world,
        CommandKind::WithdrawIntent { intent: intent(1) },
    )
    .unwrap();
    refused(
        &mut world,
        Issuer::Player,
        set_intent(1),
        CommandRefusal::IntentNumberNotUsable(intent(1)),
    );
    // An interval of None is allowed: the staff proposes it.
    player(&mut world, bad(2, 1, 10, None)).unwrap();
}

#[test]
fn setting_an_intent_again_amends_an_approved_plan_and_not_a_draft() {
    let mut world = world();
    player(&mut world, set_intent(1)).unwrap();
    let again = |floor: u32| CommandKind::SetIntent {
        intent: intent(1),
        kind: IntentKind::KeepSupplied {
            base: base(1),
            floor_days: floor,
            interval_days: None,
        },
    };
    // A draft: the kind is replaced and nothing is amended.
    let before = world.events.len();
    player(&mut world, again(20)).unwrap();
    assert_eq!(world.events.len(), before + 1);
    assert!(state(&world, A).plans[&plan(1)].amendments.is_empty());
    assert!(matches!(
        state(&world, A).intents[&intent(1)].kind,
        IntentKind::KeepSupplied { floor_days: 20, .. }
    ));
    // Preparing: the baseline is the intent as it then stood.
    player(&mut world, CommandKind::Approve { plan: plan(1) }).unwrap();
    let baseline = state(&world, A).plans[&plan(1)].baseline.clone().unwrap();
    assert_eq!(baseline.day, 1);
    assert!(matches!(
        baseline.kind,
        IntentKind::KeepSupplied { floor_days: 20, .. }
    ));
    // Approved: a change is a numbered amendment, and the baseline stays.
    player(&mut world, again(30)).unwrap();
    player(&mut world, again(40)).unwrap();
    let amendments = &state(&world, A).plans[&plan(1)].amendments;
    assert_eq!(amendments.len(), 2);
    assert_eq!((amendments[0].number, amendments[1].number), (1, 2));
    assert!(matches!(
        amendments[1].change,
        AmendmentChange::Intent(IntentKind::KeepSupplied { floor_days: 40, .. })
    ));
    assert_eq!(state(&world, A).plans[&plan(1)].baseline, Some(baseline));
    let events = log(&world);
    let amended: Vec<_> = events
        .iter()
        .filter(|event| matches!(event.kind, EventKind::PlanAmended { .. }))
        .collect();
    assert_eq!(amended.len(), 2);
    assert_eq!(
        amended[1].kind,
        EventKind::PlanAmended {
            side: A,
            plan: plan(1),
            number: 2
        }
    );
    assert_eq!(amended[1].causes.len(), 1);
}

#[test]
fn an_intent_whose_plan_ended_is_given_a_new_plan_when_set_again() {
    let mut world = world_with_plan(false);
    staff(
        &mut world,
        CommandKind::MarkPlan {
            plan: plan(1),
            outcome: PlanOutcome::Failed,
        },
    )
    .unwrap();
    assert_eq!(state(&world, A).plans[&plan(1)].state, PlanState::Failed);
    let before = world.events.len();
    player(&mut world, set_intent(1)).unwrap();
    let command = state(&world, A);
    assert_eq!(command.intents[&intent(1)].plan, plan(2));
    assert_eq!(command.plans[&plan(2)].state, PlanState::Draft);
    assert_eq!(command.plans[&plan(1)].state, PlanState::Failed);
    assert_eq!(command.next_plan, 3);
    let events = log(&world);
    assert_eq!(
        events[before + 1].kind,
        EventKind::PlanDrafted {
            side: A,
            plan: plan(2),
            intent: intent(1)
        }
    );
}

#[test]
fn withdrawing_an_intent_ends_its_plan_complete_if_released_else_cancelled() {
    // A draft is cancelled.
    let mut world = world();
    player(&mut world, set_intent(1)).unwrap();
    player(
        &mut world,
        CommandKind::WithdrawIntent { intent: intent(1) },
    )
    .unwrap();
    assert_eq!(state(&world, A).intents[&intent(1)].withdrawn_day, Some(1));
    assert_eq!(state(&world, A).plans[&plan(1)].state, PlanState::Cancelled);
    let events = log(&world);
    assert_eq!(
        events.last().unwrap().kind,
        EventKind::PlanStateChanged {
            side: A,
            plan: plan(1),
            from: PlanState::Draft,
            to: PlanState::Cancelled
        }
    );
    refused(
        &mut world,
        Issuer::Player,
        CommandKind::WithdrawIntent { intent: intent(1) },
        CommandRefusal::NoSuchIntent(intent(1)),
    );
    refused(
        &mut world,
        Issuer::Player,
        CommandKind::WithdrawIntent { intent: intent(4) },
        CommandRefusal::NoSuchIntent(intent(4)),
    );
    // Preparing is cancelled; released is complete.
    let mut world = world_with_plan(false);
    player(
        &mut world,
        CommandKind::WithdrawIntent { intent: intent(1) },
    )
    .unwrap();
    assert_eq!(state(&world, A).plans[&plan(1)].state, PlanState::Cancelled);
    let mut world = world_with_plan(true);
    player(
        &mut world,
        CommandKind::WithdrawIntent { intent: intent(1) },
    )
    .unwrap();
    assert_eq!(state(&world, A).plans[&plan(1)].state, PlanState::Complete);
    // A plan that already ended is left as it is.
    let mut world = world_with_plan(false);
    player(&mut world, CommandKind::CancelPlan { plan: plan(1) }).unwrap();
    let before = world.events.len();
    player(
        &mut world,
        CommandKind::WithdrawIntent { intent: intent(1) },
    )
    .unwrap();
    assert_eq!(world.events.len(), before + 1);
}

#[test]
fn every_kind_of_limit_is_taken_in_and_written() {
    let mut world = world();
    player(&mut world, set_intent(1)).unwrap();
    let area: BTreeSet<Hex> = [HOME, OUT].into_iter().collect();
    for limit in [
        Limit::Forces {
            intent: intent(1),
            limit: forces(&[1, 5], &[1]),
        },
        Limit::Area {
            intent: intent(1),
            hexes: area.clone(),
        },
        Limit::Priority {
            intent: intent(1),
            rank: Some(2),
        },
        Limit::Protected {
            intent: intent(1),
            ships: ships(&[1]),
        },
        Limit::Risk {
            intent: intent(1),
            risk: Risk::Press,
        },
    ] {
        player(&mut world, CommandKind::SetLimit(limit)).unwrap();
    }
    let limits = &state(&world, A).intents[&intent(1)].limits;
    assert_eq!(limits.forces, Some(forces(&[1, 5], &[1])));
    assert_eq!(limits.area, Some(area));
    assert_eq!(limits.priority, Some(2));
    assert_eq!(limits.protected, ships(&[1]));
    assert_eq!(limits.risk, Risk::Press);
    // A rank can be taken away again.
    player(
        &mut world,
        CommandKind::SetLimit(Limit::Priority {
            intent: intent(1),
            rank: None,
        }),
    )
    .unwrap();
    assert_eq!(state(&world, A).intents[&intent(1)].limits.priority, None);
    // The draft has no amendment for any of it.
    assert!(state(&world, A).plans[&plan(1)].amendments.is_empty());
    // A reserve is the base's: it writes the base and names no intent.
    player(
        &mut world,
        CommandKind::SetLimit(Limit::Reserve {
            base: base(1),
            supply_t: t(400),
            fuel_t: t(300),
        }),
    )
    .unwrap();
    assert_eq!(world.truth.bases[&base(1)].supply_reserve_t, t(400));
    assert_eq!(world.truth.bases[&base(1)].fuel_reserve_t, t(300));
    // A reserve of 0 is allowed: it takes the reserve away.
    player(
        &mut world,
        CommandKind::SetLimit(Limit::Reserve {
            base: base(1),
            supply_t: Fixed::ZERO,
            fuel_t: Fixed::ZERO,
        }),
    )
    .unwrap();
    assert_eq!(world.truth.bases[&base(1)].supply_reserve_t, Fixed::ZERO);
}

#[test]
fn a_limit_is_refused_for_what_is_not_the_sides_own_or_not_sound() {
    let mut world = world();
    player(&mut world, set_intent(1)).unwrap();
    player(&mut world, set_intent(2)).unwrap();
    player(
        &mut world,
        CommandKind::WithdrawIntent { intent: intent(2) },
    )
    .unwrap();
    // A sunk ship, an enemy's, and one that does not exist.
    world.truth.ships.get_mut(&ship(2)).unwrap().location = ShipLocation::Sunk;
    let limit = |l: Limit| CommandKind::SetLimit(l);
    let p = Issuer::Player;
    refused(
        &mut world,
        p,
        set_forces(1, forces(&[2], &[])),
        CommandRefusal::NotYourShip(ship(2)),
    );
    refused(
        &mut world,
        p,
        set_forces(1, forces(&[20], &[])),
        CommandRefusal::NotYourShip(ship(20)),
    );
    refused(
        &mut world,
        p,
        set_forces(1, forces(&[99], &[])),
        CommandRefusal::NotYourShip(ship(99)),
    );
    refused(
        &mut world,
        p,
        set_forces(1, forces(&[], &[20])),
        CommandRefusal::NotYourForce(tf(20)),
    );
    refused(
        &mut world,
        p,
        set_forces(1, forces(&[], &[99])),
        CommandRefusal::NotYourForce(tf(99)),
    );
    let mut enemy_source = forces(&[1], &[]);
    enemy_source.source = base(9);
    refused(
        &mut world,
        p,
        set_forces(1, enemy_source),
        CommandRefusal::NotYourBase(base(9)),
    );
    refused(
        &mut world,
        p,
        limit(Limit::Area {
            intent: intent(1),
            hexes: BTreeSet::new(),
        }),
        CommandRefusal::BadArea,
    );
    refused(
        &mut world,
        p,
        limit(Limit::Area {
            intent: intent(1),
            hexes: [HOME, Hex::new(40, 40)].into_iter().collect(),
        }),
        CommandRefusal::BadArea,
    );
    refused(
        &mut world,
        p,
        limit(Limit::Protected {
            intent: intent(1),
            ships: ships(&[20]),
        }),
        CommandRefusal::NotYourShip(ship(20)),
    );
    refused(
        &mut world,
        p,
        limit(Limit::Reserve {
            base: base(9),
            supply_t: t(1),
            fuel_t: t(1),
        }),
        CommandRefusal::NotYourBase(base(9)),
    );
    refused(
        &mut world,
        p,
        limit(Limit::Reserve {
            base: base(1),
            supply_t: t(1),
            fuel_t: t(-1),
        }),
        CommandRefusal::BadWeight(t(-1)),
    );
    // An intent limit needs the intent in force: withdrawn, or never set.
    for missing in [2, 3] {
        refused(
            &mut world,
            p,
            limit(Limit::Risk {
                intent: intent(missing),
                risk: Risk::Preserve,
            }),
            CommandRefusal::NoSuchIntent(intent(missing)),
        );
    }
}

#[test]
fn a_change_of_limit_amends_an_approved_plan_and_not_a_draft() {
    let mut world = world();
    player(&mut world, set_intent(1)).unwrap();
    player(&mut world, set_forces(1, forces(&[5], &[]))).unwrap();
    assert!(state(&world, A).plans[&plan(1)].amendments.is_empty());
    player(&mut world, CommandKind::Approve { plan: plan(1) }).unwrap();
    let risk = CommandKind::SetLimit(Limit::Risk {
        intent: intent(1),
        risk: Risk::Preserve,
    });
    player(&mut world, risk).unwrap();
    player(&mut world, set_forces(1, forces(&[5, 6], &[]))).unwrap();
    let command = state(&world, A);
    let amendments = &command.plans[&plan(1)].amendments;
    assert_eq!(amendments.len(), 2);
    assert_eq!(amendments[1].number, 2);
    assert!(matches!(
        &amendments[1].change,
        AmendmentChange::Limit(Limit::Forces { .. })
    ));
    // The baseline is what the plan was approved under: ship 5 alone, balanced.
    let baseline = command.plans[&plan(1)].baseline.as_ref().unwrap();
    assert_eq!(baseline.limits.forces, Some(forces(&[5], &[])));
    assert_eq!(baseline.limits.risk, Risk::Balanced);
    assert_eq!(
        command.intents[&intent(1)].limits.forces,
        Some(forces(&[5, 6], &[]))
    );
    // A reserve names no intent and amends no plan.
    let before = world.events.len();
    player(
        &mut world,
        CommandKind::SetLimit(Limit::Reserve {
            base: base(1),
            supply_t: t(1),
            fuel_t: t(1),
        }),
    )
    .unwrap();
    assert_eq!(world.events.len(), before + 1);
}

#[test]
fn leaves_are_set_and_bind_the_staffs_approval() {
    let mut world = world();
    assert_eq!(
        state(&world, A).leaves,
        Leaves {
            prepare: true,
            launch: false
        }
    );
    player(&mut world, set_intent(1)).unwrap();
    player(&mut world, set_forces(1, forces(&[5], &[]))).unwrap();
    // Without the leave to prepare, the staff may not.
    player(
        &mut world,
        CommandKind::SetLeave {
            leave: Leave::Prepare,
            given: false,
        },
    )
    .unwrap();
    assert!(!state(&world, A).leaves.prepare);
    refused(
        &mut world,
        Issuer::Staff,
        CommandKind::Approve { plan: plan(1) },
        CommandRefusal::NoLeave(Leave::Prepare),
    );
    player(
        &mut world,
        CommandKind::SetLeave {
            leave: Leave::Prepare,
            given: true,
        },
    )
    .unwrap();
    staff(&mut world, CommandKind::Approve { plan: plan(1) }).unwrap();
    assert_eq!(state(&world, A).plans[&plan(1)].state, PlanState::Preparing);
    assert!(state(&world, A).plans[&plan(1)].baseline.is_some());
    // The launch leave is withheld by default.
    refused(
        &mut world,
        Issuer::Staff,
        CommandKind::Approve { plan: plan(1) },
        CommandRefusal::NoLeave(Leave::Launch),
    );
    player(
        &mut world,
        CommandKind::SetLeave {
            leave: Leave::Launch,
            given: true,
        },
    )
    .unwrap();
    staff(&mut world, CommandKind::Approve { plan: plan(1) }).unwrap();
    assert_eq!(state(&world, A).plans[&plan(1)].state, PlanState::Released);
    let events = log(&world);
    assert_eq!(
        events.last().unwrap().kind,
        EventKind::PlanStateChanged {
            side: A,
            plan: plan(1),
            from: PlanState::Preparing,
            to: PlanState::Released
        }
    );
    // A released plan is approved no more; an unknown one is not found.
    refused(
        &mut world,
        Issuer::Player,
        CommandKind::Approve { plan: plan(1) },
        CommandRefusal::WrongPlanState(PlanState::Released),
    );
    refused(
        &mut world,
        Issuer::Staff,
        CommandKind::Approve { plan: plan(1) },
        CommandRefusal::WrongPlanState(PlanState::Released),
    );
    refused(
        &mut world,
        Issuer::Player,
        CommandKind::Approve { plan: plan(9) },
        CommandRefusal::NoSuchPlan(plan(9)),
    );
}

#[test]
fn the_commander_prepares_and_releases_without_leave_and_cancels() {
    let mut world = world();
    player(&mut world, set_intent(1)).unwrap();
    player(
        &mut world,
        CommandKind::SetLeave {
            leave: Leave::Prepare,
            given: false,
        },
    )
    .unwrap();
    player(&mut world, CommandKind::Approve { plan: plan(1) }).unwrap();
    player(&mut world, CommandKind::Approve { plan: plan(1) }).unwrap();
    assert_eq!(state(&world, A).plans[&plan(1)].state, PlanState::Released);
    player(&mut world, CommandKind::CancelPlan { plan: plan(1) }).unwrap();
    assert_eq!(state(&world, A).plans[&plan(1)].state, PlanState::Cancelled);
    refused(
        &mut world,
        Issuer::Player,
        CommandKind::CancelPlan { plan: plan(1) },
        CommandRefusal::WrongPlanState(PlanState::Cancelled),
    );
    refused(
        &mut world,
        Issuer::Player,
        CommandKind::CancelPlan { plan: plan(5) },
        CommandRefusal::NoSuchPlan(plan(5)),
    );
    let events = log(&world);
    assert!(events.iter().any(|event| event.kind
        == EventKind::PlanStateChanged {
            side: A,
            plan: plan(1),
            from: PlanState::Released,
            to: PlanState::Cancelled
        }));
}

#[test]
fn the_staff_is_refused_the_commanders_commands_and_the_commander_the_staffs() {
    let mut world = world_with_plan(true);
    let staff_kinds = [
        CommandKind::CommitShips {
            plan: plan(1),
            run: None,
            ships: ships(&[5]),
            from_day: 1,
            to_day: 2,
        },
        CommandKind::FreeShips {
            plan: plan(1),
            ships: ships(&[5]),
        },
        CommandKind::AddRun {
            plan: plan(1),
            run: run_spec(&[5], [1, 1, 2, 3]),
        },
        CommandKind::TimeRun {
            plan: plan(1),
            run: 1,
            days: RunDays {
                load: 1,
                sail: 1,
                arrive: 2,
                free: 3,
            },
        },
        CommandKind::MarkPlan {
            plan: plan(1),
            outcome: PlanOutcome::Complete,
        },
    ];
    for kind in staff_kinds {
        refused(&mut world, Issuer::Player, kind, CommandRefusal::NotYours);
    }
    let player_kinds = [
        set_intent(2),
        CommandKind::WithdrawIntent { intent: intent(1) },
        CommandKind::SetLimit(Limit::Risk {
            intent: intent(1),
            risk: Risk::Press,
        }),
        CommandKind::SetLeave {
            leave: Leave::Launch,
            given: true,
        },
        CommandKind::CancelPlan { plan: plan(1) },
    ];
    for kind in player_kinds {
        refused(&mut world, Issuer::Staff, kind, CommandRefusal::NotYours);
    }
    // The local commander gives none.
    refused(
        &mut world,
        Issuer::LocalCommander,
        CommandKind::Approve { plan: plan(1) },
        CommandRefusal::NotYours,
    );
}

// ---------------------------------------------------------------------------
// The staff's commands.
// ---------------------------------------------------------------------------

fn commit(plan_n: u32, run: Option<u32>, list: &[u32], from_day: u32, to_day: u32) -> CommandKind {
    CommandKind::CommitShips {
        plan: plan(plan_n),
        run,
        ships: ships(list),
        from_day,
        to_day,
    }
}

#[test]
fn ships_are_committed_in_order_of_their_first_day_and_freed_by_plan() {
    // Ships 5 and 6 are listed; ships 1 and 2 are in force 1, which is listed.
    let mut world = world_with_plan(false);
    staff(&mut world, commit(1, None, &[5, 1], 5, 8)).unwrap();
    staff(&mut world, commit(1, None, &[5], 1, 3)).unwrap();
    let list = &state(&world, A).commitments[&ship(5)];
    assert_eq!(
        list.iter()
            .map(|c| (c.from_day, c.to_day))
            .collect::<Vec<_>>(),
        vec![(1, 3), (5, 8)]
    );
    assert_eq!(
        state(&world, A).commitment_on(ship(5), 6).unwrap().from_day,
        5
    );
    assert!(state(&world, A).commitment_on(ship(5), 4).is_none());
    assert_eq!(state(&world, A).commitments[&ship(1)].len(), 1);
    // A run it serves must exist.
    refused(
        &mut world,
        Issuer::Staff,
        commit(1, Some(3), &[6], 1, 2),
        CommandRefusal::NoSuchRun(3),
    );
    // Freeing removes every commitment of the ship to the plan, and the
    // entry when it is empty.
    staff(
        &mut world,
        CommandKind::FreeShips {
            plan: plan(1),
            ships: ships(&[5]),
        },
    )
    .unwrap();
    assert!(!state(&world, A).commitments.contains_key(&ship(5)));
    assert!(state(&world, A).commitments.contains_key(&ship(1)));
    refused(
        &mut world,
        Issuer::Staff,
        CommandKind::FreeShips {
            plan: plan(1),
            ships: ships(&[5]),
        },
        CommandRefusal::NotCommitted(ship(5)),
    );
    refused(
        &mut world,
        Issuer::Staff,
        CommandKind::FreeShips {
            plan: plan(1),
            ships: ships(&[6]),
        },
        CommandRefusal::NotCommitted(ship(6)),
    );
    refused(
        &mut world,
        Issuer::Staff,
        CommandKind::FreeShips {
            plan: plan(8),
            ships: ships(&[1]),
        },
        CommandRefusal::NoSuchPlan(plan(8)),
    );
}

#[test]
fn a_commitment_is_refused_for_a_plan_a_pool_a_ship_or_days_and_overlap_across_plans() {
    let mut world = world_with_plan(false);
    let s = Issuer::Staff;
    // Ship 3 is the side's own but in force 2, which is not listed.
    refused(
        &mut world,
        s,
        commit(1, None, &[3], 1, 2),
        CommandRefusal::NotInPool(ship(3)),
    );
    refused(
        &mut world,
        s,
        commit(1, None, &[20], 1, 2),
        CommandRefusal::NotYourShip(ship(20)),
    );
    refused(
        &mut world,
        s,
        commit(1, None, &[99], 1, 2),
        CommandRefusal::NotYourShip(ship(99)),
    );
    world.truth.ships.get_mut(&ship(6)).unwrap().location = ShipLocation::Sunk;
    refused(
        &mut world,
        s,
        commit(1, None, &[6], 1, 2),
        CommandRefusal::NotYourShip(ship(6)),
    );
    // The first day may not be before the day the command is taken in, nor
    // after the last.
    world.truth.day = 4;
    refused(
        &mut world,
        s,
        commit(1, None, &[5], 4, 6),
        CommandRefusal::BadDays,
    );
    refused(
        &mut world,
        s,
        commit(1, None, &[5], 7, 6),
        CommandRefusal::BadDays,
    );
    world.truth.day = 0;
    refused(
        &mut world,
        s,
        commit(2, None, &[5], 1, 2),
        CommandRefusal::NoSuchPlan(plan(2)),
    );
    // A plan that is a draft takes no commitment.
    player(&mut world, set_intent(2)).unwrap();
    refused(
        &mut world,
        s,
        commit(2, None, &[5], 1, 2),
        CommandRefusal::WrongPlanState(PlanState::Draft),
    );
    // An approved plan whose intent has no forces limit.
    player(&mut world, CommandKind::Approve { plan: plan(2) }).unwrap();
    refused(
        &mut world,
        s,
        commit(2, None, &[5], 1, 2),
        CommandRefusal::NoForcesLimit,
    );
    player(&mut world, set_forces(2, forces(&[5], &[]))).unwrap();
    // Overlap with a commitment to another plan: a day shared is enough.
    staff(&mut world, commit(1, None, &[5], 3, 6)).unwrap();
    refused(
        &mut world,
        s,
        commit(2, None, &[5], 6, 9),
        CommandRefusal::AlreadyCommitted(ship(5)),
    );
    refused(
        &mut world,
        s,
        commit(2, None, &[5], 1, 3),
        CommandRefusal::AlreadyCommitted(ship(5)),
    );
    refused(
        &mut world,
        s,
        commit(2, None, &[5], 4, 5),
        CommandRefusal::AlreadyCommitted(ship(5)),
    );
    refused(
        &mut world,
        s,
        commit(1, None, &[5], 6, 7),
        CommandRefusal::AlreadyCommitted(ship(5)),
    );
    // Days that do not touch are allowed, before and after.
    staff(&mut world, commit(2, None, &[5], 1, 2)).unwrap();
    staff(&mut world, commit(2, None, &[5], 7, 9)).unwrap();
    let list = &state(&world, A).commitments[&ship(5)];
    assert_eq!(
        list.iter()
            .map(|c| (c.plan, c.from_day))
            .collect::<Vec<_>>(),
        vec![(plan(2), 1), (plan(1), 3), (plan(2), 7)]
    );
    // The ship not listed, left in the pool by a limit that lists its force
    // only while it is in it: a ship in a listed force is in the pool.
    player(&mut world, set_forces(2, forces(&[], &[1]))).unwrap();
    staff(&mut world, commit(2, None, &[2], 1, 2)).unwrap();
}

#[test]
fn runs_are_added_numbered_and_timed_and_refused_when_unsound() {
    let mut world = world_with_plan(false);
    let s = Issuer::Staff;
    let add = |spec: RunSpec| CommandKind::AddRun {
        plan: plan(1),
        run: spec,
    };
    staff(&mut world, add(run_spec(&[5, 6], [2, 3, 6, 8]))).unwrap();
    staff(&mut world, add(run_spec(&[1], [1, 1, 1, 1]))).unwrap();
    let runs = &state(&world, A).plans[&plan(1)].runs;
    assert_eq!(runs.keys().copied().collect::<Vec<_>>(), vec![1, 2]);
    assert_eq!(runs[&1].source, base(1));
    assert_eq!(runs[&1].load_t, t(300));
    assert_eq!(runs[&1].ships, ships(&[5, 6]));
    assert_eq!(runs[&2].number, 2);
    // Refusals.
    refused(
        &mut world,
        s,
        add(run_spec(&[], [1, 2, 3, 4])),
        CommandRefusal::NoShips,
    );
    refused(
        &mut world,
        s,
        add(run_spec(&[3], [1, 2, 3, 4])),
        CommandRefusal::NotInPool(ship(3)),
    );
    refused(
        &mut world,
        s,
        add(run_spec(&[20], [1, 2, 3, 4])),
        CommandRefusal::NotYourShip(ship(20)),
    );
    let mut heavy = run_spec(&[5], [1, 2, 3, 4]);
    heavy.load_t = Fixed::ZERO;
    refused(
        &mut world,
        s,
        add(heavy),
        CommandRefusal::BadWeight(Fixed::ZERO),
    );
    refused(
        &mut world,
        s,
        add(run_spec(&[5], [2, 1, 3, 4])),
        CommandRefusal::BadDays,
    );
    refused(
        &mut world,
        s,
        add(run_spec(&[5], [1, 3, 2, 4])),
        CommandRefusal::BadDays,
    );
    refused(
        &mut world,
        s,
        add(run_spec(&[5], [1, 2, 4, 3])),
        CommandRefusal::BadDays,
    );
    world.truth.day = 2;
    refused(
        &mut world,
        s,
        add(run_spec(&[5], [2, 3, 4, 5])),
        CommandRefusal::BadDays,
    );
    world.truth.day = 0;
    refused(
        &mut world,
        s,
        CommandKind::AddRun {
            plan: plan(3),
            run: run_spec(&[5], [1, 2, 3, 4]),
        },
        CommandRefusal::NoSuchPlan(plan(3)),
    );
    // Timing again.
    let time = |run: u32, days: [u32; 4]| CommandKind::TimeRun {
        plan: plan(1),
        run,
        days: RunDays {
            load: days[0],
            sail: days[1],
            arrive: days[2],
            free: days[3],
        },
    };
    staff(&mut world, time(1, [3, 4, 7, 9])).unwrap();
    assert_eq!(
        state(&world, A).plans[&plan(1)].runs[&1].days,
        RunDays {
            load: 3,
            sail: 4,
            arrive: 7,
            free: 9
        }
    );
    refused(
        &mut world,
        s,
        time(5, [3, 4, 7, 9]),
        CommandRefusal::NoSuchRun(5),
    );
    refused(
        &mut world,
        s,
        time(1, [5, 4, 7, 9]),
        CommandRefusal::BadDays,
    );
    world.truth.day = 10;
    refused(
        &mut world,
        s,
        time(1, [3, 4, 7, 9]),
        CommandRefusal::BadDays,
    );
    world.truth.day = 0;
    // A plan that is not approved takes no run.
    player(&mut world, set_intent(2)).unwrap();
    refused(
        &mut world,
        s,
        CommandKind::AddRun {
            plan: plan(2),
            run: run_spec(&[5], [1, 2, 3, 4]),
        },
        CommandRefusal::WrongPlanState(PlanState::Draft),
    );
    // An intent with no forces limit has no pool.
    player(&mut world, CommandKind::Approve { plan: plan(2) }).unwrap();
    refused(
        &mut world,
        s,
        CommandKind::AddRun {
            plan: plan(2),
            run: run_spec(&[5], [1, 2, 3, 4]),
        },
        CommandRefusal::NoForcesLimit,
    );
}

#[test]
fn a_plan_is_marked_complete_or_failed_only_when_approved() {
    let mut world = world_with_plan(true);
    staff(
        &mut world,
        CommandKind::MarkPlan {
            plan: plan(1),
            outcome: PlanOutcome::Complete,
        },
    )
    .unwrap();
    assert_eq!(state(&world, A).plans[&plan(1)].state, PlanState::Complete);
    let events = log(&world);
    assert_eq!(
        events.last().unwrap().kind,
        EventKind::PlanStateChanged {
            side: A,
            plan: plan(1),
            from: PlanState::Released,
            to: PlanState::Complete
        }
    );
    refused(
        &mut world,
        Issuer::Staff,
        CommandKind::MarkPlan {
            plan: plan(1),
            outcome: PlanOutcome::Failed,
        },
        CommandRefusal::WrongPlanState(PlanState::Complete),
    );
    player(&mut world, set_intent(2)).unwrap();
    refused(
        &mut world,
        Issuer::Staff,
        CommandKind::MarkPlan {
            plan: plan(2),
            outcome: PlanOutcome::Failed,
        },
        CommandRefusal::WrongPlanState(PlanState::Draft),
    );
    refused(
        &mut world,
        Issuer::Staff,
        CommandKind::MarkPlan {
            plan: plan(7),
            outcome: PlanOutcome::Failed,
        },
        CommandRefusal::NoSuchPlan(plan(7)),
    );
}

#[test]
fn a_command_changes_the_hash_through_every_field_of_the_command_state() {
    let base_world = world();
    let hash = base_world.state_hash();
    let changed = |edit: &dyn Fn(&mut CommandState)| {
        let mut world = base_world.clone();
        edit(world.command.get_mut(&A).unwrap());
        world.state_hash()
    };
    let with_intent = |state: &mut CommandState| {
        state.intents.insert(
            intent(1),
            Intent {
                id: intent(1),
                kind: keep_supplied(1),
                limits: IntentLimits::default(),
                set_day: 1,
                plan: plan(1),
                withdrawn_day: None,
            },
        );
    };
    let hashes = [
        changed(&|s| s.leaves.launch = true),
        changed(&|s| s.next_plan = 2),
        changed(&with_intent),
        changed(&|s| {
            with_intent(s);
            s.intents.get_mut(&intent(1)).unwrap().withdrawn_day = Some(3);
        }),
        changed(&|s| {
            with_intent(s);
            s.intents.get_mut(&intent(1)).unwrap().limits.risk = Risk::Press;
        }),
        changed(&|s| {
            s.commitments.insert(
                ship(1),
                vec![Commitment {
                    plan: plan(1),
                    run: None,
                    from_day: 1,
                    to_day: 2,
                }],
            );
        }),
        changed(&|s| {
            s.plans.insert(
                plan(1),
                Plan {
                    id: plan(1),
                    intent: intent(1),
                    state: PlanState::Draft,
                    baseline: None,
                    amendments: vec![],
                    runs: BTreeMap::new(),
                },
            );
        }),
    ];
    let distinct: BTreeSet<u64> = hashes.iter().copied().chain([hash]).collect();
    assert_eq!(distinct.len(), hashes.len() + 1);
    // Allied then Japanese: the same change on the other side hashes otherwise.
    let mut other = base_world.clone();
    other.command.get_mut(&J).unwrap().leaves.launch = true;
    assert_ne!(other.state_hash(), hashes[0]);
}

// ---------------------------------------------------------------------------
// The staff in the day.
// ---------------------------------------------------------------------------

/// A world with intent 1 limited to ships 5 and 6 and force 1 (source base
/// 1), its plan released, taken in before the first day.
fn staffed_world() -> World {
    world_with_plan(true)
}

fn staff_command(kind: CommandKind, worked: &str) -> StaffCommand {
    StaffCommand {
        kind,
        reason: reason(1, worked),
    }
}

fn decisions_for_the_day() -> StaffDecisions {
    StaffDecisions {
        orders: vec![
            staff_order(speed(1), 1, "inside: force 1 is listed"),
            staff_order(speed(2), 1, "outside: force 2 is not listed"),
        ],
        commands: vec![staff_command(commit(1, None, &[5], 1, 3), "commit ship 5")],
        no_orders: vec![reason(1, "nothing to do for ship 6")],
    }
}

#[test]
fn a_staffs_order_outside_its_forces_limit_is_refused_and_one_inside_accepted_each_with_its_reason()
{
    let mut world = staffed_world();
    let staffs = Fixed_(decisions_for_the_day());
    world
        .resolve_day_staffed(Staffs::none().with(A, &staffs))
        .unwrap();
    let events = log(&world);
    let position = |pred: &dyn Fn(&Event) -> bool| events.iter().position(pred).unwrap();
    // The commander's commands, the staff's commands, then the orders.
    let accepted = position(&|e| matches!(e.kind, EventKind::OrderAccepted { .. }));
    let rejected = position(&|e| {
        matches!(
            e.kind,
            EventKind::OrderRejected {
                reason: RejectReason::OutsideForcesLimit,
                ..
            }
        )
    });
    let commit_taken = position(&|e| {
        matches!(
            &e.kind,
            EventKind::CommandTaken {
                issuer: Issuer::Staff,
                command: CommandKind::CommitShips { .. },
                ..
            }
        )
    });
    assert!(commit_taken < accepted && accepted < rejected);
    // Each order is followed by its reason, citing the order and its answer.
    for (answer, worked) in [(accepted, "inside"), (rejected, "outside")] {
        let next = &events[answer + 1];
        let (EventKind::OrderAccepted { order, .. } | EventKind::OrderRejected { order, .. }) =
            events[answer].kind
        else {
            unreachable!()
        };
        let EventKind::StaffReason {
            side,
            order: reason_order,
            reason,
        } = &next.kind
        else {
            panic!(
                "the answer is followed by the staff's reason, not {:?}",
                next.kind
            )
        };
        assert_eq!((*side, *reason_order), (A, Some(order)));
        assert!(reason.worked.starts_with(worked));
        assert_eq!(
            next.causes,
            vec![Cause::Order(order), Cause::Event(events[answer].id)]
        );
        assert_eq!(next.pulse, None);
    }
    // The order refused changed nothing; the one accepted is the staff's.
    let EventKind::OrderRejected { order, .. } = events[rejected].kind else {
        unreachable!()
    };
    assert_eq!(world.order(order).unwrap().origin.issuer, Issuer::Staff);
    assert_eq!(world.order(order).unwrap().origin.intent, Some(intent(1)));
    // The decision to give no order is a reason with no order and no causes.
    let none = events
        .iter()
        .find(|e| {
            matches!(&e.kind, EventKind::StaffReason { order: None, .. }) && e.causes.is_empty()
        })
        .unwrap();
    let EventKind::StaffReason { reason, .. } = &none.kind else {
        unreachable!()
    };
    assert_eq!(reason.worked, "nothing to do for ship 6");
    assert_eq!(reason.serves.intent, Some(intent(1)));
    // The commitment the staff made stands.
    assert!(state(&world, A).commitment_on(ship(5), 2).is_some());
    // What was handed in is kept for the runner.
    assert_eq!(world.staff_handed_in()[&A], decisions_for_the_day());
    assert!(!world.staff_handed_in().contains_key(&J));
}

#[test]
fn a_staff_commands_reason_follows_the_command_and_cites_it_whether_taken_or_refused() {
    let mut world = staffed_world();
    let decisions = StaffDecisions {
        commands: vec![
            // Taken.
            staff_command(commit(1, None, &[5], 1, 3), "reason of the taken command"),
            // Refused: the staff may not set an intent.
            staff_command(set_intent(3), "reason of the refused command"),
        ],
        ..StaffDecisions::default()
    };
    let staffs = Fixed_(decisions);
    world
        .resolve_day_staffed(Staffs::none().with(A, &staffs))
        .unwrap();
    let events = log(&world);
    for (worked, taken) in [
        ("reason of the taken command", true),
        ("reason of the refused command", false),
    ] {
        let at = events
            .iter()
            .position(|e| {
                matches!(&e.kind, EventKind::StaffReason { reason, .. } if reason.worked == worked)
            })
            .unwrap();
        let EventKind::StaffReason { side, order, .. } = &events[at].kind else {
            unreachable!()
        };
        assert_eq!((*side, *order), (A, None));
        let [Cause::Event(cited)] = events[at].causes[..] else {
            panic!("a staff command's reason cites exactly its command's event")
        };
        let cited_at = events.iter().position(|e| e.id == cited).unwrap();
        assert!(cited_at < at, "the reason is recorded after the command");
        match (&events[cited_at].kind, taken) {
            (
                EventKind::CommandTaken {
                    issuer: Issuer::Staff,
                    ..
                },
                true,
            )
            | (
                EventKind::CommandRefused {
                    issuer: Issuer::Staff,
                    ..
                },
                false,
            ) => {}
            (other, _) => panic!("cited event is not the expected command record: {other:?}"),
        }
    }
}

#[test]
fn a_staffs_order_is_outside_when_its_intent_is_gone_or_has_no_limit_or_is_not_a_force_or_forming_there(
) {
    let mut world = staffed_world();
    player(&mut world, set_intent(2)).unwrap();
    let form = |target: u32, list: &[u32]| OrderKind::Base {
        target: base(target),
        order: BaseOrder::FormTaskForce {
            ships: list.iter().map(|n| ship(*n)).collect(),
        },
    };
    let decisions = StaffDecisions {
        orders: vec![
            // Inside: the source base forms a force of the pool's ships.
            staff_order(form(1, &[5, 6]), 1, "inside forming"),
            // Outside: another base; a ship not in the pool; an intent with
            // no forces limit; an intent that was never set.
            staff_order(form(2, &[5]), 1, "outside base"),
            staff_order(form(1, &[3]), 1, "outside ship"),
            staff_order(speed(1), 2, "outside no limit"),
            staff_order(speed(1), 9, "outside no intent"),
            // A force whose ships are not all in the pool.
            staff_order(speed(2), 1, "outside force"),
        ],
        ..StaffDecisions::default()
    };
    let staffs = Fixed_(decisions);
    world
        .resolve_day_staffed(Staffs::none().with(A, &staffs))
        .unwrap();
    // Each reason cites its order's answer: find it in the log.
    let events = log(&world);
    let verdicts: Vec<(String, bool)> = events
        .iter()
        .filter_map(|event| {
            let EventKind::StaffReason {
                order: Some(_),
                reason,
                ..
            } = &event.kind
            else {
                return None;
            };
            let Cause::Event(answer) = event.causes[1] else {
                panic!("the reason cites the answer")
            };
            let answer = events.iter().find(|e| e.id == answer).unwrap();
            let not_outside = match &answer.kind {
                EventKind::OrderAccepted { .. } => true,
                EventKind::OrderRejected { reason: why, .. } => {
                    *why != RejectReason::OutsideForcesLimit
                }
                other => panic!("{other:?}"),
            };
            Some((reason.worked.clone(), not_outside))
        })
        .collect();
    // `true` marks an answer other than "outside the forces limit".
    assert_eq!(verdicts.len(), 6, "{verdicts:?}");
    for (worked, not_outside) in verdicts {
        assert_eq!(not_outside, worked.starts_with("inside"), "{worked}");
    }
}

#[test]
fn the_staff_is_asked_after_the_commanders_commands_and_the_refuelling() {
    let mut world = staffed_world();
    // The commander's reserve is taken in at step 1: base 1 may give down to
    // 450 t of fuel, so of the 200 t the two ships lack it gives 50 t, to
    // ship 1, before the staff is asked.
    world.submit_commands(
        A,
        vec![CommandKind::SetLimit(Limit::Reserve {
            base: base(1),
            supply_t: t(400),
            fuel_t: t(450),
        })],
    );
    let asked = Cell::new(0);
    let staffs = Computed(|view: SideView<'_>| {
        asked.set(asked.get() + 1);
        let home = view.bases().find(|b| b.id == base(1)).unwrap();
        assert_eq!(
            home.fuel_reserve_t,
            t(450),
            "the reserve set by command is in force"
        );
        assert_eq!(home.supply_reserve_t, t(400));
        assert_eq!(home.fuel_t, t(450), "the stock is what refuelling left");
        let ships: Vec<Fixed> = view
            .ships()
            .filter(|s| s.id == ship(1) || s.id == ship(2))
            .map(|s| s.fuel_t)
            .collect();
        assert_eq!(ships, vec![t(1950), t(1900)]);
        // The commander's commands are in the command state, and the day
        // is the day being resolved.
        assert_eq!(view.day(), 0);
        assert!(view.command_state().plans[&plan(1)].state == PlanState::Released);
        StaffDecisions::default()
    });
    world
        .resolve_day_staffed(Staffs::none().with(A, &staffs))
        .unwrap();
    assert_eq!(asked.get(), 1, "asked once a day");
    // A side with no staff is not asked and not in the hand-in. A staff that
    // gave nothing is, as an empty hand-in.
    assert_eq!(world.staff_handed_in()[&A], StaffDecisions::default());
}

#[test]
fn a_same_day_forming_order_then_a_load_order_to_the_new_force_are_both_accepted() {
    let mut world = world();
    player(&mut world, set_intent(1)).unwrap();
    player(&mut world, set_forces(1, forces(&[5, 6], &[]))).unwrap();
    player(&mut world, CommandKind::Approve { plan: plan(1) }).unwrap();
    let new_force = tf(world.made_ids.next_task_force(A));
    let load = OrderKind::TaskForce {
        target: new_force,
        order: TaskForceOrder::LoadTo {
            cargo: Cargo::Supply,
            weight_t: t(100),
        },
    };
    let form = OrderKind::Base {
        target: base(1),
        order: BaseOrder::FormTaskForce {
            ships: vec![ship(5), ship(6)],
        },
    };
    // The load is handed in first: forming orders are taken in first.
    let staffs = Fixed_(StaffDecisions {
        orders: vec![staff_order(load, 1, "load"), staff_order(form, 1, "form")],
        ..StaffDecisions::default()
    });
    world
        .resolve_day_staffed(Staffs::none().with(A, &staffs))
        .unwrap();
    let answers: Vec<bool> = log(&world)
        .iter()
        .filter_map(|e| match e.kind {
            EventKind::OrderAccepted { .. } => Some(true),
            EventKind::OrderRejected { .. } => Some(false),
            _ => None,
        })
        .collect();
    assert_eq!(answers, vec![true, true]);
    let force = &world.truth.task_forces[&new_force];
    assert_eq!(force.ships, ships(&[5, 6]));
    assert!(force.cargo_order.is_some());
    assert_eq!(world.made_ids.next_task_force(A), new_force.raw() + 1);
}

#[test]
fn the_world_at_the_staff_step_has_the_commands_in_and_the_original_is_untouched() {
    let mut world = world();
    world.submit_commands(A, vec![set_intent(1)]);
    let before = world.state_hash();
    let events_before = world.events.len();
    let copy = world.at_staff_step();
    assert!(copy
        .side_view(A)
        .command_state()
        .intents
        .contains_key(&intent(1)));
    assert!(!world
        .side_view(A)
        .command_state()
        .intents
        .contains_key(&intent(1)));
    // The copy is at the day before the coming one, with its orders and
    // commands still pending, and the original is as it was.
    assert_eq!(copy.truth.day, world.truth.day);
    assert_eq!(world.state_hash(), before);
    assert_eq!(world.events.len(), events_before);
    assert!(copy.events.len() > events_before);
    // Resolving the original day gives the same command state as the copy.
    world.resolve_day().unwrap();
    assert_eq!(
        format!("{:?}", world.side_view(A).command_state()),
        format!("{:?}", copy.side_view(A).command_state())
    );
}

#[test]
fn the_next_task_force_id_is_the_number_a_forming_order_then_makes() {
    let mut world = staffed_world();
    let view = world.side_view(A);
    let new_force = view.next_task_force_id();
    assert_eq!(new_force.raw(), world.made_ids.next_task_force(A));
    // The other side's series is its own.
    assert_ne!(world.side_view(J).next_task_force_id(), new_force);
    let form = OrderKind::Base {
        target: base(1),
        order: BaseOrder::FormTaskForce {
            ships: vec![ship(5), ship(6)],
        },
    };
    let staffs = Fixed_(StaffDecisions {
        orders: vec![staff_order(form, 1, "form")],
        ..StaffDecisions::default()
    });
    world
        .resolve_day_staffed(Staffs::none().with(A, &staffs))
        .unwrap();
    assert_eq!(world.truth.task_forces[&new_force].ships, ships(&[5, 6]));
    assert_eq!(
        world.side_view(A).next_task_force_id().raw(),
        new_force.raw() + 1
    );
}

#[test]
fn submitting_the_staffs_decisions_gives_the_same_day_as_a_staff_that_hands_them_in() {
    let decisions = decisions_for_the_day();
    let mut asked = staffed_world();
    let staff = Fixed_(decisions.clone());
    asked
        .resolve_day_staffed(Staffs::none().with(A, &staff))
        .unwrap();
    let mut given = staffed_world();
    given.submit_staff(A, decisions.clone());
    given.resolve_day().unwrap();
    assert_eq!(asked.state_hash(), given.state_hash());
    assert_eq!(log(&asked), log(&given));
    assert_eq!(asked.staff_handed_in(), given.staff_handed_in());
    assert_eq!(given.staff_handed_in()[&A], decisions);
    // A staff asked as well as given back adds to what was given back.
    let mut both = staffed_world();
    both.submit_staff(
        A,
        StaffDecisions {
            no_orders: vec![reason(1, "given back")],
            ..StaffDecisions::default()
        },
    );
    both.resolve_day_staffed(Staffs::none().with(A, &staff))
        .unwrap();
    let handed = &both.staff_handed_in()[&A];
    assert_eq!(handed.no_orders.len(), 2);
    assert_eq!(handed.no_orders[0].worked, "given back");
    // The hand-in is only for the day: the next day with no staff has none.
    both.resolve_day().unwrap();
    assert!(both.staff_handed_in().is_empty());
}

#[test]
fn with_no_command_and_no_staff_a_day_is_the_day_it_was() {
    let mut plain = world();
    let mut staffed = world();
    for _ in 0..3 {
        let a = plain.resolve_day().unwrap();
        let b = staffed.resolve_day_staffed(Staffs::none()).unwrap();
        assert_eq!(a, b);
    }
    assert_eq!(plain.state_hash(), staffed.state_hash());
    assert_eq!(log(&plain), log(&staffed));
    assert!(log(&plain).iter().all(|e| !matches!(
        e.kind,
        EventKind::CommandTaken { .. }
            | EventKind::CommandRefused { .. }
            | EventKind::PlanDrafted { .. }
            | EventKind::PlanStateChanged { .. }
            | EventKind::PlanAmended { .. }
            | EventKind::StaffReason { .. }
    )));
    assert!(plain.staff_handed_in().is_empty());
    assert!(plain.command.values().all(CommandState::is_empty));
}

#[test]
fn commands_handed_in_for_a_day_are_taken_in_side_by_side_in_the_order_handed_in() {
    let mut world = world();
    world.submit_commands(
        J,
        vec![CommandKind::SetLeave {
            leave: Leave::Launch,
            given: true,
        }],
    );
    world.submit_commands(
        A,
        vec![set_intent(1), CommandKind::Approve { plan: plan(1) }],
    );
    world.resolve_day().unwrap();
    let kinds: Vec<(Side, &'static str)> = log(&world)
        .iter()
        .filter_map(|e| match &e.kind {
            EventKind::CommandTaken { side, command, .. } => Some((*side, command.name())),
            _ => None,
        })
        .collect();
    assert_eq!(
        kinds,
        vec![(A, "set_intent"), (A, "approve"), (J, "set_leave")]
    );
    assert_eq!(state(&world, A).plans[&plan(1)].state, PlanState::Preparing);
    assert!(world.pending_commands.is_empty());
}

// ---------------------------------------------------------------------------
// What a side may not learn.
// ---------------------------------------------------------------------------

/// Everything a side's view gives that could change with what the other side
/// did, as text: its events, its units, bases and contacts, its command
/// state.
fn what_side_sees(world: &World, side: Side) -> Vec<String> {
    let view = world.side_view(side);
    let mut seen = texts(&view.events());
    seen.push(format!("{:?}", view.command_state()));
    seen.extend(view.ships().map(|s| format!("{s:?}")));
    seen.extend(view.task_forces().map(|f| format!("{f:?}")));
    seen.extend(view.bases().map(|b| format!("{b:?}")));
    seen.extend(
        view.bases()
            .map(|b| format!("{:?}", view.days_of_supply(b.hex))),
    );
    seen.extend(view.enemy_bases().map(|b| format!("{b:?}")));
    seen.extend(view.contacts().map(|c| format!("{c:?}")));
    seen.push(format!("{:?}", view.claims()));
    seen
}

/// Plays the same days in two worlds that differ only in what `other` does:
/// nothing in one; in the other, commands (taken in and refused), and a staff
/// that hands in commands, orders, and reasons.
fn two_worlds_differing_in_the_other_side(other: Side) -> (World, World) {
    let enemy_base = if other == J { 9 } else { 1 };
    let enemy_force = if other == J { 20 } else { 1 };
    let enemy_ship = if other == J { 20 } else { 1 };
    let quiet = world();
    let mut busy = world();
    let give_other = |world: &mut World, kind: CommandKind| {
        world.submit_commands(other, vec![kind]);
    };
    give_other(
        &mut busy,
        CommandKind::SetIntent {
            intent: intent(1),
            kind: IntentKind::KeepSupplied {
                base: base(enemy_base),
                floor_days: 10,
                interval_days: None,
            },
        },
    );
    give_other(
        &mut busy,
        CommandKind::SetLimit(Limit::Forces {
            intent: intent(1),
            limit: ForcesLimit {
                ships: [ship(enemy_ship)].into_iter().collect(),
                task_forces: [tf(enemy_force)].into_iter().collect(),
                source: base(enemy_base),
            },
        }),
    );
    give_other(
        &mut busy,
        CommandKind::SetLimit(Limit::Reserve {
            base: base(enemy_base),
            supply_t: t(10),
            fuel_t: t(10),
        }),
    );
    give_other(&mut busy, CommandKind::Approve { plan: plan(1) });
    give_other(&mut busy, CommandKind::CancelPlan { plan: plan(5) });
    (quiet, busy)
}

fn leak_test(other: Side, watcher: Side) {
    let (mut quiet, mut busy) = two_worlds_differing_in_the_other_side(other);
    let enemy_force = if other == J { 20 } else { 1 };
    let enemy_ship = if other == J { 20 } else { 1 };
    let enemy_base = if other == J { 9 } else { 1 };
    let decisions = StaffDecisions {
        orders: vec![
            StaffOrder {
                kind: speed(enemy_force),
                intent: intent(1),
                reason: reason(1, "inside"),
            },
            StaffOrder {
                kind: speed(2),
                intent: intent(1),
                reason: reason(1, "outside"),
            },
        ],
        commands: [
            CommandKind::CommitShips {
                plan: plan(1),
                run: None,
                ships: ships(&[enemy_ship]),
                from_day: 1,
                to_day: 4,
            },
            CommandKind::AddRun {
                plan: plan(1),
                run: RunSpec {
                    ships: ships(&[enemy_ship]),
                    load_t: t(10),
                    days: RunDays {
                        load: 1,
                        sail: 1,
                        arrive: 2,
                        free: 3,
                    },
                },
            },
            // Refused: the staff may not set an intent.
            set_intent(3),
        ]
        .into_iter()
        .map(|kind| staff_command(kind, "a command"))
        .collect(),
        no_orders: vec![reason(1, "no order for the idle ship")],
    };
    let busy_staff = Fixed_(decisions);
    for day in 0..3 {
        quiet.resolve_day().unwrap();
        busy.resolve_day_staffed(Staffs::none().with(other, &busy_staff))
            .unwrap();
        assert_eq!(
            what_side_sees(&quiet, watcher),
            what_side_sees(&busy, watcher),
            "day {day}: {watcher:?} learned of {other:?}'s commands or staff"
        );
    }
    // The worlds did differ, in the other side's own record.
    assert_ne!(quiet.state_hash(), busy.state_hash());
    assert!(state(&quiet, other).is_empty());
    assert!(!state(&busy, other).is_empty());
    let own = texts(&busy.side_view(other).events());
    assert!(own.iter().any(|text| text.contains("command taken")));
    assert!(own.iter().any(|text| text.contains("staff")));
    // The watcher's own record has no word of it, whatever the other did.
    let watched = texts(&busy.side_view(watcher).events());
    assert!(watched.iter().all(|text| !text.contains("command")));
    assert!(watched.iter().all(|text| !text.contains("staff")));
    let _ = enemy_base;
}

#[test]
fn the_japanese_commands_and_staff_leave_no_trace_in_the_allied_view() {
    leak_test(J, A);
}

#[test]
fn the_allied_commands_and_staff_leave_no_trace_in_the_japanese_view() {
    leak_test(A, J);
}

#[test]
fn a_refusal_names_no_enemy_unit_or_base_that_exists_any_differently_from_one_that_does_not() {
    // An enemy base, ship and force and one that do not exist are refused
    // alike, with the same reason apart from the number asked about.
    let mut world = world();
    player(&mut world, set_intent(1)).unwrap();
    let ask = |world: &mut World, kind: CommandKind| {
        let before = world.events.len();
        let _ = player(world, kind);
        match &world.events.from_index(before)[0].kind {
            EventKind::CommandRefused { reason, .. } => reason.to_string(),
            other => panic!("{other:?}"),
        }
    };
    let enemy_ship = ask(&mut world, set_forces(1, forces(&[20], &[])));
    let no_ship = ask(&mut world, set_forces(1, forces(&[98], &[])));
    assert_eq!(enemy_ship.replace("20", "N"), no_ship.replace("98", "N"));
    let enemy_force = ask(&mut world, set_forces(1, forces(&[], &[20])));
    let no_force = ask(&mut world, set_forces(1, forces(&[], &[98])));
    assert_eq!(enemy_force.replace("20", "N"), no_force.replace("98", "N"));
}

// ---------------------------------------------------------------------------
// The check an order meets beside the orders queued with it.
// ---------------------------------------------------------------------------

#[test]
fn an_order_to_a_force_a_queued_forming_order_will_make_is_accepted_by_the_queued_check_only() {
    let mut world = world();
    let new_force = tf(world.made_ids.next_task_force(A));
    let load = OrderRequest {
        kind: OrderKind::TaskForce {
            target: new_force,
            order: TaskForceOrder::LoadTo {
                cargo: Cargo::Supply,
                weight_t: t(100),
            },
        },
        origin: Origin::PLAYER,
    };
    let form = OrderRequest {
        kind: OrderKind::Base {
            target: base(1),
            order: BaseOrder::FormTaskForce {
                ships: vec![ship(5), ship(6)],
            },
        },
        origin: Origin::PLAYER,
    };
    // Nothing queued: the force does not exist, for either check.
    assert_eq!(world.check_order(A, &load), Err(RejectReason::NotYourUnit));
    assert_eq!(
        world.check_order_queued(A, &load, &[]),
        Err(RejectReason::NotYourUnit)
    );
    // The forming order queued beside it, handed in with it.
    assert_eq!(world.check_order(A, &load), Err(RejectReason::NotYourUnit));
    assert_eq!(
        world.check_order_queued(A, &load, std::slice::from_ref(&form)),
        Ok(())
    );
    // The forming order already submitted for the day.
    world.submit_orders(A, vec![form.clone()]);
    assert_eq!(world.check_order(A, &load), Err(RejectReason::NotYourUnit));
    assert_eq!(world.check_order_queued(A, &load, &[]), Ok(()));
    // It changed nothing: no force was made and no order numbered.
    assert!(!world.truth.task_forces.contains_key(&new_force));
    assert_eq!(world.orders().count(), 0);
    // The forming order itself is checked as before.
    assert_eq!(
        world.check_order_queued(A, &form, &[]),
        world.check_order(A, &form)
    );
    // The other side's queued forming orders do not make the force.
    let mut other = self::world();
    other.submit_orders(J, vec![form]);
    assert_eq!(
        other.check_order_queued(A, &load, &[]),
        Err(RejectReason::NotYourUnit)
    );
}

#[test]
fn an_order_number_of_a_staffs_order_is_the_sides_own_whatever_the_other_side_orders() {
    // The staff's order and its reason cite the side's own number of the
    // order; an extra order of the other side shifts the umpire's number and
    // not the side's.
    let mut quiet = staffed_world();
    let mut busy = staffed_world();
    busy.submit_orders(
        J,
        vec![OrderRequest {
            kind: speed(20),
            origin: Origin::PLAYER,
        }],
    );
    let staffs = Fixed_(decisions_for_the_day());
    for world in [&mut quiet, &mut busy] {
        world
            .resolve_day_staffed(Staffs::none().with(A, &staffs))
            .unwrap();
    }
    assert_eq!(
        texts(&quiet.side_view(A).events()),
        texts(&busy.side_view(A).events())
    );
    let first_order: OrderId = *quiet.orders.keys().next().unwrap();
    assert_eq!(quiet.side_view(A).order_number(first_order), Some(1));
    let _ = (FAR, OUT);
}
