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
        fuels_at_sea: false,
    };
    let mut ref_data = RefData::default();
    ref_data.ship_classes.insert(class.id, class);
    ref_data.port_sizes.insert(
        0,
        PortSize {
            berths: 0,
            berth_rate_t_per_h: Fixed::ZERO,
            lighter_rate_t_per_h: t(20),
            fuel_berth_rate_t_per_h: None,
            fuel_lighter_rate_t_per_h: None,
            ammunition_berth_rate_t_per_h: None,
            ammunition_lighter_rate_t_per_h: None,
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
        air_groups: BTreeSet::new(),
        land_units: BTreeSet::new(),
        source: Some(base(1)),
        submarines: Default::default(),
        none_of: Default::default(),
        in_place: false,
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
            statement: String::new(),
            chain: Vec::new(),
            level: None,
        },
        evidence: vec![Evidence {
            what: "a stock".to_string(),
            day: 1,
        }],
        binding: vec!["a limit".to_string()],
        worked: worked.to_string(),
        would_change: vec!["more stock".to_string()],
        answer_of: None,
        cautions: Vec::new(),
        limits: Vec::new(),
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
    assert_eq!(limits.risk, Some(Risk::Press));
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
    enemy_source.source = Some(base(9));
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
    // No risk was set: the limit is unset, and read as balanced (decision
    // 0115, 4; decision 0117, 7).
    assert_eq!(baseline.limits.risk, None);
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

/// Decision 0142: "take a place" is taken in at a hex of land only, its
/// forces limit needs no source since decision 0149, the staff forms "ready a landing" under it
/// while its plan is preparing, and the staff sets a forces limit only, and
/// only on an intent it formed.
#[test]
fn taking_a_place_is_taken_in_and_its_landing_formed_while_preparing() {
    const PLACE: Hex = Hex::new(5, 0);
    let mut world = world();
    world.map.insert(
        PLACE,
        HexData {
            kind: crate::map::HexKind::Coast,
            ..HexData::open_sea()
        },
    );
    let take = |hex: Hex| CommandKind::SetIntent {
        intent: intent(1),
        kind: IntentKind::TakePlace { hex, by_day: None },
    };
    refused(
        &mut world,
        Issuer::Player,
        take(Hex::new(4, 0)),
        CommandRefusal::NotLand(Hex::new(4, 0)),
    );
    refused(
        &mut world,
        Issuer::Player,
        take(Hex::new(40, 0)),
        CommandRefusal::NotLand(Hex::new(40, 0)),
    );
    player(&mut world, take(PLACE)).unwrap();
    // A forces limit with no source is taken in (decision 0149, item 1,
    // which withdraws decision 0142, item 2, for this end).
    let mut no_source = forces(&[], &[1]);
    no_source.source = None;
    player(&mut world, set_forces(1, no_source)).unwrap();
    player(&mut world, set_forces(1, forces(&[], &[1]))).unwrap();
    // The staff sets no limit on the commander's intent.
    refused(
        &mut world,
        Issuer::Staff,
        set_forces(1, forces(&[], &[1])),
        CommandRefusal::NotYours,
    );
    let landing = |hex: Hex| CommandKind::FormIntent {
        parent: intent(1),
        kind: IntentKind::ReadyLanding {
            force: tf(1),
            hex,
            by_day: None,
        },
    };
    // A draft is not enough; preparing is.
    refused(
        &mut world,
        Issuer::Staff,
        landing(PLACE),
        CommandRefusal::WrongPlanState(PlanState::Draft),
    );
    player(&mut world, CommandKind::Approve { plan: plan(1) }).unwrap();
    refused(
        &mut world,
        Issuer::Staff,
        landing(HOME),
        CommandRefusal::CannotStandUnder(intent(1)),
    );
    staff(&mut world, landing(PLACE)).unwrap();
    let formed = &state(&world, A).intents[&intent(2)];
    assert_eq!(formed.maker, Maker::Staff);
    assert_eq!(formed.parent, Some(intent(1)));
    // The staff sets the forces limit of the intent it formed, and no other
    // kind of limit.
    let mut narrow = forces(&[], &[]);
    narrow.source = None;
    staff(&mut world, set_forces(2, narrow.clone())).unwrap();
    assert_eq!(
        state(&world, A).intents[&intent(2)].limits.forces,
        Some(narrow)
    );
    refused(
        &mut world,
        Issuer::Staff,
        CommandKind::SetLimit(Limit::Risk {
            intent: intent(2),
            risk: Risk::Press,
        }),
        CommandRefusal::NotYours,
    );
}

/// Decision 0159, 14: the staff forms "operate the carrier force from a hex"
/// under "take a place" while its plan is preparing, as it forms "ready a
/// landing" (a force moved to look); not while it is a draft.
#[test]
fn a_carrier_task_under_a_take_is_formed_while_preparing() {
    const PLACE: Hex = Hex::new(5, 0);
    let mut world = world();
    world.map.insert(
        PLACE,
        HexData {
            kind: crate::map::HexKind::Coast,
            ..HexData::open_sea()
        },
    );
    player(
        &mut world,
        CommandKind::SetIntent {
            intent: intent(1),
            kind: IntentKind::TakePlace {
                hex: PLACE,
                by_day: None,
            },
        },
    )
    .unwrap();
    player(&mut world, set_forces(1, forces(&[], &[1]))).unwrap();
    let cover = CommandKind::FormIntent {
        parent: intent(1),
        kind: IntentKind::OperateCarriers {
            force: tf(1),
            hex: Hex::new(3, 0),
        },
    };
    refused(
        &mut world,
        Issuer::Staff,
        cover.clone(),
        CommandRefusal::WrongPlanState(PlanState::Draft),
    );
    player(&mut world, CommandKind::Approve { plan: plan(1) }).unwrap();
    staff(&mut world, cover).unwrap();
    let formed = &state(&world, A).intents[&intent(2)];
    assert_eq!(formed.maker, Maker::Staff);
    assert_eq!(formed.parent, Some(intent(1)));
}

/// Decision 0149, items 1 to 3: a "take a place" needs no forces limit and
/// none with a source; the staff's limit on the "ready a landing" it forms
/// may name what the commander left out, and nothing outside what he named;
/// and the end's limit is then read as it binds: the commander's where he
/// named a thing, the staff's where he did not.
#[test]
fn taking_a_place_needs_no_source_and_the_staff_names_what_is_left_out() {
    const PLACE: Hex = Hex::new(5, 0);
    let mut world = world();
    world.map.insert(
        PLACE,
        HexData {
            kind: crate::map::HexKind::Coast,
            ..HexData::open_sea()
        },
    );
    player(
        &mut world,
        CommandKind::SetIntent {
            intent: intent(1),
            kind: IntentKind::TakePlace {
                hex: PLACE,
                by_day: None,
            },
        },
    )
    .unwrap();
    // A forces limit with no source is taken in (item 1).
    let mut no_source = forces(&[], &[1]);
    no_source.source = None;
    player(&mut world, set_forces(1, no_source.clone())).unwrap();
    player(&mut world, CommandKind::Approve { plan: plan(1) }).unwrap();
    staff(
        &mut world,
        CommandKind::FormIntent {
            parent: intent(1),
            kind: IntentKind::ReadyLanding {
                force: tf(1),
                hex: PLACE,
                by_day: None,
            },
        },
    )
    .unwrap();
    // The staff may not name a force the commander did not name.
    let mut wider = forces(&[], &[2]);
    wider.source = Some(base(1));
    refused(
        &mut world,
        Issuer::Staff,
        set_forces(2, wider),
        CommandRefusal::WiderThanParent(intent(1)),
    );
    // It may name the source he left out.
    let mut chosen = forces(&[], &[]);
    chosen.source = Some(base(1));
    staff(&mut world, set_forces(2, chosen)).unwrap();
    // The end's limit as it binds: his task force, the staff's source.
    let end = &state(&world, A).intents[&intent(1)];
    let bound = state(&world, A)
        .forces_as_bound(end)
        .expect("a limit")
        .into_owned();
    assert_eq!(bound.task_forces, no_source.task_forces);
    assert_eq!(bound.source, Some(base(1)));
    // And the task's limit reads the task force from the end.
    let task = &state(&world, A).intents[&intent(2)];
    let bound = state(&world, A)
        .forces_as_bound(task)
        .expect("a limit")
        .into_owned();
    assert_eq!(bound.task_forces, no_source.task_forces);
    assert_eq!(bound.source, Some(base(1)));
    // The staff's own earlier choice does not bind its next (the
    // architect's ruling 5 of decision 0149): having named base 1 as the
    // source it may name base 2, as it does when it chooses again.
    let mut again = forces(&[], &[]);
    again.source = Some(base(2));
    staff(&mut world, set_forces(2, again)).unwrap();
    let end = &state(&world, A).intents[&intent(1)];
    let bound = state(&world, A)
        .forces_as_bound(end)
        .expect("a limit")
        .into_owned();
    assert_eq!(bound.source, Some(base(2)));
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
                parent: None,
                maker: Maker::Commander,
                addressee: Addressee::Area,
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
            s.intents.get_mut(&intent(1)).unwrap().limits.risk = Some(Risk::Press);
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
        notices: Vec::new(),
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
            ..
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
fn the_staff_is_asked_after_the_commanders_commands_on_the_fuel_of_the_day_before() {
    let mut world = staffed_world();
    // The commander's reserve is taken in at step 1: base 1 may give down to
    // 450 t of fuel. Nothing is refuelled before the staff is asked
    // (decision 0112): it sees the fuel as it stood at the end of the day
    // before, 500 t at base 1 and the two ships 200 t short.
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
        assert_eq!(home.fuel_t, t(500), "nothing is refuelled before the staff");
        let ships: Vec<Fixed> = view
            .ships()
            .filter(|s| s.id == ship(1) || s.id == ship(2))
            .map(|s| s.fuel_t)
            .collect();
        assert_eq!(ships, vec![t(1900), t(1900)]);
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
            each_t: None,
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
                air_groups: BTreeSet::new(),
                land_units: BTreeSet::new(),
                source: Some(base(enemy_base)),
                submarines: Default::default(),
                none_of: Default::default(),
                in_place: false,
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
        notices: Vec::new(),
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
                each_t: None,
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

// ---------------------------------------------------------------------------
// Holds, the override and the end of a run (decision 0107, step 3).
// ---------------------------------------------------------------------------

fn ag(n: u32) -> crate::ids::AirGroupId {
    crate::ids::AirGroupId::new(n)
}

fn arc_of(n: u32) -> HoldTarget {
    HoldTarget::Setting {
        group: ag(n),
        setting: HeldSetting::SearchArc,
    }
}

fn hold(target: HoldTarget) -> CommandKind {
    CommandKind::Hold { target }
}

fn release(target: HoldTarget) -> CommandKind {
    CommandKind::Release { target }
}

/// The world with Allied air group 1 at base 1 (and Japanese air group 2).
fn world_with_groups() -> World {
    use crate::ids::AircraftTypeId;
    use crate::state::{AirGroup, AirGroupLocation};
    let mut world = staffed_world();
    for (n, side) in [(1, A), (2, J)] {
        world.truth.air_groups.insert(
            ag(n),
            AirGroup::new(
                ag(n),
                format!("Group {n}"),
                side,
                AirGroupLocation::Base(base(if side == A { 1 } else { 9 })),
                AircraftTypeId::new(1),
            ),
        );
    }
    world
}

#[test]
fn a_hold_is_taken_in_with_its_day_and_maker_and_a_release_removes_it_whoever_made_it() {
    let mut world = world_with_groups();
    for target in [
        HoldTarget::Ship(ship(5)),
        HoldTarget::TaskForce(tf(1)),
        arc_of(1),
    ] {
        player(&mut world, hold(target)).unwrap();
        assert_eq!(
            state(&world, A).hold_of(target),
            Some(&Hold {
                day: 1,
                by: HoldMaker::Command
            })
        );
        // No follow-up event: the command's record is the record.
        let events = log(&world);
        assert!(matches!(
            events.last().unwrap().kind,
            EventKind::CommandTaken { .. }
        ));
        refused(
            &mut world,
            Issuer::Player,
            hold(target),
            CommandRefusal::AlreadyHeld(target),
        );
    }
    player(&mut world, release(HoldTarget::Ship(ship(5)))).unwrap();
    assert!(state(&world, A)
        .hold_of(HoldTarget::Ship(ship(5)))
        .is_none());
    refused(
        &mut world,
        Issuer::Player,
        release(HoldTarget::Ship(ship(5))),
        CommandRefusal::NotHeld(HoldTarget::Ship(ship(5))),
    );
    // A hold an order made is released the same way.
    world.command.get_mut(&A).unwrap().holds.insert(
        HoldTarget::Ship(ship(6)),
        Hold {
            day: 1,
            by: HoldMaker::Order(OrderId::new(1)),
        },
    );
    player(&mut world, release(HoldTarget::Ship(ship(6)))).unwrap();
    assert_eq!(state(&world, A).holds.len(), 2);
}

#[test]
fn a_hold_or_release_is_refused_for_what_is_not_the_sides_own_or_the_wrong_issuer() {
    let mut world = world_with_groups();
    let p = Issuer::Player;
    for (target, reason) in [
        (
            HoldTarget::Ship(ship(20)),
            CommandRefusal::NotYourShip(ship(20)),
        ),
        (
            HoldTarget::Ship(ship(99)),
            CommandRefusal::NotYourShip(ship(99)),
        ),
        (
            HoldTarget::TaskForce(tf(20)),
            CommandRefusal::NotYourForce(tf(20)),
        ),
        (
            HoldTarget::TaskForce(tf(99)),
            CommandRefusal::NotYourForce(tf(99)),
        ),
        (arc_of(2), CommandRefusal::NotYourGroup(ag(2))),
        (arc_of(99), CommandRefusal::NotYourGroup(ag(99))),
    ] {
        refused(&mut world, p, hold(target), reason);
    }
    refused(
        &mut world,
        Issuer::Staff,
        hold(HoldTarget::Ship(ship(5))),
        CommandRefusal::NotYours,
    );
    refused(
        &mut world,
        Issuer::Staff,
        release(HoldTarget::Ship(ship(5))),
        CommandRefusal::NotYours,
    );
}

#[test]
fn a_run_ends_once_by_the_staff_on_an_approved_plan_and_nothing_else_changes() {
    let mut world = world_with_plan(true);
    staff(
        &mut world,
        CommandKind::AddRun {
            plan: plan(1),
            run: run_spec(&[5, 6], [2, 3, 6, 8]),
        },
    )
    .unwrap();
    staff(&mut world, commit(1, Some(1), &[5], 1, 8)).unwrap();
    let end = CommandKind::EndRun {
        plan: plan(1),
        run: 1,
    };
    assert_eq!(state(&world, A).plans[&plan(1)].runs[&1].ended, None);
    refused(
        &mut world,
        Issuer::Player,
        end.clone(),
        CommandRefusal::NotYours,
    );
    let commitments = state(&world, A).commitments.clone();
    staff(&mut world, end.clone()).unwrap();
    assert!(matches!(
        log(&world).last().unwrap().kind,
        EventKind::CommandTaken { .. }
    ));
    let run = &state(&world, A).plans[&plan(1)].runs[&1];
    assert_eq!(run.ended, Some(1));
    assert_eq!(run.ships, ships(&[5, 6]));
    assert_eq!(state(&world, A).commitments, commitments);
    refused(&mut world, Issuer::Staff, end, CommandRefusal::RunEnded(1));
    refused(
        &mut world,
        Issuer::Staff,
        CommandKind::EndRun {
            plan: plan(1),
            run: 9,
        },
        CommandRefusal::NoSuchRun(9),
    );
    refused(
        &mut world,
        Issuer::Staff,
        CommandKind::EndRun {
            plan: plan(9),
            run: 1,
        },
        CommandRefusal::NoSuchPlan(plan(9)),
    );
    // A draft plan is not approved.
    player(
        &mut world,
        CommandKind::SetIntent {
            intent: intent(2),
            kind: keep_supplied(1),
        },
    )
    .unwrap();
    refused(
        &mut world,
        Issuer::Staff,
        CommandKind::EndRun {
            plan: plan(2),
            run: 1,
        },
        CommandRefusal::WrongPlanState(PlanState::Draft),
    );
}

#[test]
fn a_run_or_a_commitment_naming_a_held_ship_or_one_in_a_held_force_is_refused() {
    let add = |ship_list: &[u32]| CommandKind::AddRun {
        plan: plan(1),
        run: run_spec(ship_list, [1, 2, 3, 4]),
    };
    let commit_ships = |ship_list: &[u32]| commit(1, None, ship_list, 1, 3);
    let s = Issuer::Staff;
    // A ship held.
    let mut world = world_with_plan(true);
    player(&mut world, hold(HoldTarget::Ship(ship(5)))).unwrap();
    let held = CommandRefusal::Held(HoldTarget::Ship(ship(5)));
    refused(&mut world, s, add(&[6, 5]), held);
    refused(&mut world, s, commit_ships(&[5]), held);
    staff(&mut world, add(&[6])).unwrap();
    // A ship of a held force.
    let mut world = world_with_plan(true);
    player(&mut world, hold(HoldTarget::TaskForce(tf(1)))).unwrap();
    let held = CommandRefusal::Held(HoldTarget::TaskForce(tf(1)));
    refused(&mut world, s, add(&[1]), held);
    refused(&mut world, s, commit_ships(&[2]), held);
    // A ship in a force with another held ship.
    let mut world = world_with_plan(true);
    player(&mut world, hold(HoldTarget::Ship(ship(2)))).unwrap();
    let held = CommandRefusal::Held(HoldTarget::Ship(ship(2)));
    refused(&mut world, s, add(&[1]), held);
    refused(&mut world, s, commit_ships(&[1]), held);
    // Freeing and timing are never refused for a hold.
    let mut world = world_with_plan(true);
    staff(&mut world, add(&[5])).unwrap();
    staff(&mut world, commit(1, Some(1), &[5], 1, 3)).unwrap();
    player(&mut world, hold(HoldTarget::Ship(ship(5)))).unwrap();
    staff(
        &mut world,
        CommandKind::TimeRun {
            plan: plan(1),
            run: 1,
            days: RunDays {
                load: 1,
                sail: 2,
                arrive: 3,
                free: 5,
            },
        },
    )
    .unwrap();
    staff(
        &mut world,
        CommandKind::FreeShips {
            plan: plan(1),
            ships: ships(&[5]),
        },
    )
    .unwrap();
    // Released, the ship is free for the staff again.
    player(&mut world, release(HoldTarget::Ship(ship(5)))).unwrap();
    staff(&mut world, commit_ships(&[5])).unwrap();
}

/// Hands a staff order in as intake does and returns why it was rejected,
/// `None` if it was accepted.
fn staff_order_answer(world: &mut World, kind: OrderKind) -> Option<RejectReason> {
    let before = world.events.len();
    world.take_in_staff_order(1, A, staff_request(kind), &reason(1, "a staff order"));
    world
        .events
        .from_index(before)
        .iter()
        .find_map(|event| match &event.kind {
            EventKind::OrderRejected { reason, .. } => Some(reason.clone()),
            _ => None,
        })
}

fn staff_request(kind: OrderKind) -> OrderRequest {
    OrderRequest {
        kind,
        origin: Origin {
            issuer: Issuer::Staff,
            intent: Some(intent(1)),
        },
    }
}

fn form(ship_list: &[u32]) -> OrderKind {
    OrderKind::Base {
        target: base(1),
        order: BaseOrder::FormTaskForce {
            ships: ship_list.iter().map(|n| ship(*n)).collect(),
        },
    }
}

fn arc_order(group: u32) -> OrderKind {
    OrderKind::AirGroup {
        target: ag(group),
        order: crate::orders::AirGroupOrder::SetSearchArc {
            from_deg: 0,
            to_deg: 90,
            range_nm: t(200),
        },
    }
}

#[test]
fn a_staffs_order_to_what_is_held_is_rejected_held_and_the_same_order_unheld_passes_the_guard() {
    let not_held = |answer: Option<RejectReason>| !matches!(answer, Some(RejectReason::Held(_)));
    // A held force, and a force with a held ship.
    let mut world = world_with_groups();
    assert!(not_held(staff_order_answer(&mut world, speed(1))));
    player(&mut world, hold(HoldTarget::TaskForce(tf(1)))).unwrap();
    assert_eq!(
        staff_order_answer(&mut world, speed(1)),
        Some(RejectReason::Held(HoldTarget::TaskForce(tf(1))))
    );
    let mut world = world_with_groups();
    player(&mut world, hold(HoldTarget::Ship(ship(2)))).unwrap();
    assert_eq!(
        staff_order_answer(&mut world, speed(1)),
        Some(RejectReason::Held(HoldTarget::Ship(ship(2))))
    );
    // A force with nothing held in it is not guarded.
    assert!(not_held(staff_order_answer(&mut world, speed(2))));
    // A forming order naming a held ship, the first found in list order.
    player(&mut world, hold(HoldTarget::Ship(ship(6)))).unwrap();
    assert!(not_held(staff_order_answer(&mut world, form(&[5]))));
    assert_eq!(
        staff_order_answer(&mut world, form(&[5, 6])),
        Some(RejectReason::Held(HoldTarget::Ship(ship(6))))
    );
    // A search arc, and the mission with it, where the arc is held.
    let mission = OrderKind::AirGroup {
        target: ag(1),
        order: crate::orders::AirGroupOrder::SetAirMission(crate::orders::AirMission::Rest),
    };
    assert!(not_held(staff_order_answer(&mut world, arc_order(1))));
    player(&mut world, hold(arc_of(1))).unwrap();
    let held = Some(RejectReason::Held(arc_of(1)));
    assert_eq!(staff_order_answer(&mut world, arc_order(1)), held);
    assert_eq!(staff_order_answer(&mut world, mission), held);
    // The guard comes before the forces-limit check, and is on a staff's
    // orders only: the commander's own order to a held force is not refused.
    let mut world = world_with_groups();
    player(&mut world, hold(HoldTarget::TaskForce(tf(1)))).unwrap();
    let (_, _, accepted) = world.take_in_order(
        1,
        None,
        A,
        OrderRequest {
            kind: speed(1),
            origin: Origin::PLAYER,
        },
        Vec::new(),
    );
    assert!(accepted);
}

/// The Allied commander's order, with no intent.
fn commander_order(kind: OrderKind) -> OrderRequest {
    OrderRequest {
        kind,
        origin: Origin::PLAYER,
    }
}

fn events_of(world: &World, pred: impl Fn(&EventKind) -> bool) -> Vec<Event> {
    log(world).into_iter().filter(|e| pred(&e.kind)).collect()
}

fn held_by_order(kind: &EventKind) -> bool {
    matches!(kind, EventKind::HeldByOrder { .. })
}

#[test]
fn an_order_to_a_force_in_a_forces_limit_is_taken_in_before_the_staff_is_asked_and_holds_it() {
    let mut world = staffed_world();
    world.submit_orders(A, vec![commander_order(speed(1))]);
    // The world as the staff will see it.
    let at_step = world.at_staff_step();
    let id = *at_step.orders.keys().next().expect("the order is numbered");
    assert!(at_step.pending[&A].is_empty());
    assert_eq!(
        state(&at_step, A).holds,
        BTreeMap::from([(
            HoldTarget::TaskForce(tf(1)),
            Hold {
                day: 1,
                by: HoldMaker::Order(id)
            }
        )])
    );
    let accepted = events_of(&at_step, |k| matches!(k, EventKind::OrderAccepted { .. }));
    assert_eq!(accepted.len(), 1);
    let held = events_of(&at_step, held_by_order);
    assert_eq!(held.len(), 1);
    assert_eq!(
        held[0].kind,
        EventKind::HeldByOrder {
            side: A,
            target: HoldTarget::TaskForce(tf(1)),
            order: id
        }
    );
    assert_eq!(
        held[0].causes,
        vec![Cause::Order(id), Cause::Event(accepted[0].id)]
    );
    // The original world is untouched.
    assert!(state(&world, A).holds.is_empty());
    assert_eq!(world.pending[&A].len(), 1);
    // A staff asked in the day sees the hold, and its own order to the force
    // is refused Held.
    let saw = Cell::new(false);
    let staffs = Computed(|view: SideView<'_>| {
        saw.set(
            view.command_state()
                .hold_of(HoldTarget::TaskForce(tf(1)))
                .is_some(),
        );
        StaffDecisions {
            orders: vec![staff_order(speed(1), 1, "to the held force")],
            ..StaffDecisions::default()
        }
    });
    world
        .resolve_day_staffed(Staffs::none().with(A, &staffs))
        .unwrap();
    assert!(saw.get(), "the hold is in force when the staff is asked");
    let rejected = events_of(&world, |k| {
        matches!(
            k,
            EventKind::OrderRejected {
                reason: RejectReason::Held(HoldTarget::TaskForce(_)),
                ..
            }
        )
    });
    assert_eq!(rejected.len(), 1);
    // The commander's order was taken in once, before the staff's.
    let accepted = events_of(&world, |k| matches!(k, EventKind::OrderAccepted { .. }));
    assert_eq!(accepted.len(), 1);
    assert!(accepted[0].id < rejected[0].id);
    // The hold stands until released.
    assert!(state(&world, A)
        .hold_of(HoldTarget::TaskForce(tf(1)))
        .is_some());
}

#[test]
fn an_order_to_a_unit_in_no_forces_limit_holds_nothing_and_is_taken_in_at_step_five() {
    let mut world = staffed_world();
    // Force 2 is in no forces limit; the order stays pending, in the order
    // handed in, behind or ahead of one that is an override.
    world.submit_orders(
        A,
        vec![commander_order(speed(2)), commander_order(speed(1))],
    );
    let at_step = world.at_staff_step();
    assert_eq!(
        at_step.pending[&A],
        vec![commander_order(speed(2))],
        "only the order that is not an override stays pending"
    );
    assert_eq!(state(&at_step, A).holds.len(), 1);
    world.resolve_day().unwrap();
    assert_eq!(
        events_of(&world, |k| matches!(k, EventKind::OrderAccepted { .. })).len(),
        2
    );
    assert_eq!(events_of(&world, held_by_order).len(), 1);
    assert!(state(&world, A)
        .hold_of(HoldTarget::TaskForce(tf(2)))
        .is_none());
}

#[test]
fn an_order_to_an_already_held_force_records_no_second_hold() {
    let mut world = staffed_world();
    player(&mut world, hold(HoldTarget::TaskForce(tf(1)))).unwrap();
    world.submit_orders(A, vec![commander_order(speed(1))]);
    let at_step = world.at_staff_step();
    assert!(events_of(&at_step, held_by_order).is_empty());
    assert_eq!(
        state(&at_step, A).hold_of(HoldTarget::TaskForce(tf(1))),
        Some(&Hold {
            day: 1,
            by: HoldMaker::Command
        })
    );
    assert_eq!(
        events_of(&at_step, |k| matches!(k, EventKind::OrderAccepted { .. })).len(),
        1
    );
}

#[test]
fn a_forming_order_naming_a_pool_ship_holds_that_ship_and_a_rejected_override_holds_nothing() {
    let mut world = staffed_world();
    // Ship 6 is in the pool, ship 3 is not; a forming order at base 1 names
    // pool ships 6 and 5, in that order.
    world.submit_orders(A, vec![commander_order(form(&[6, 5]))]);
    let at_step = world.at_staff_step();
    let held: Vec<EventKind> = events_of(&at_step, held_by_order)
        .into_iter()
        .map(|e| e.kind)
        .collect();
    let id = *at_step.orders.keys().next().unwrap();
    assert_eq!(
        held,
        vec![
            EventKind::HeldByOrder {
                side: A,
                target: HoldTarget::Ship(ship(5)),
                order: id
            },
            EventKind::HeldByOrder {
                side: A,
                target: HoldTarget::Ship(ship(6)),
                order: id
            }
        ],
        "in ship order"
    );
    assert!(at_step.pending[&A].is_empty());
    // An order the intake rejects holds nothing: a route to where the force
    // already is.
    let mut world = staffed_world();
    world.submit_orders(
        A,
        vec![commander_order(OrderKind::TaskForce {
            target: tf(1),
            order: TaskForceOrder::SetRoute {
                waypoints: vec![HOME],
                act_at: BTreeSet::new(),
                stand_at: BTreeSet::new(),
            },
        })],
    );
    let at_step = world.at_staff_step();
    assert!(state(&at_step, A).holds.is_empty());
    assert!(events_of(&at_step, held_by_order).is_empty());
    assert_eq!(
        events_of(&at_step, |k| matches!(k, EventKind::OrderRejected { .. })).len(),
        1
    );
    assert!(at_step.pending[&A].is_empty());
}

#[test]
fn a_side_with_no_intent_in_force_is_not_touched_by_the_override() {
    let mut world = world();
    world.submit_orders(A, vec![commander_order(speed(1))]);
    let pending = world.pending.clone();
    let at_step = world.at_staff_step();
    assert_eq!(at_step.pending, pending);
    assert!(at_step.command.values().all(CommandState::is_empty));
    assert!(events_of(&at_step, held_by_order).is_empty());
    // The other side's orders are not touched either, with an intent for the
    // Allied side only.
    let mut world = staffed_world();
    world.submit_orders(J, vec![commander_order(speed(20))]);
    let at_step = world.at_staff_step();
    assert_eq!(at_step.pending[&J].len(), 1);
    assert!(state(&at_step, J).holds.is_empty());
}

#[test]
fn a_hold_and_a_runs_end_change_the_hash() {
    let base_world = world();
    let hash = base_world.state_hash();
    let changed = |edit: &dyn Fn(&mut CommandState)| {
        let mut world = base_world.clone();
        edit(world.command.get_mut(&A).unwrap());
        world.state_hash()
    };
    let put = |target: HoldTarget, day: u32, by: HoldMaker| {
        move |s: &mut CommandState| {
            s.holds.insert(target, Hold { day, by });
        }
    };
    let hashes = [
        changed(&put(HoldTarget::Ship(ship(1)), 1, HoldMaker::Command)),
        changed(&put(HoldTarget::TaskForce(tf(1)), 1, HoldMaker::Command)),
        changed(&put(arc_of(1), 1, HoldMaker::Command)),
        changed(&put(HoldTarget::Ship(ship(1)), 2, HoldMaker::Command)),
        changed(&put(
            HoldTarget::Ship(ship(1)),
            1,
            HoldMaker::Order(OrderId::new(4)),
        )),
        changed(&put(
            HoldTarget::Ship(ship(1)),
            1,
            HoldMaker::Order(OrderId::new(5)),
        )),
    ];
    let distinct: BTreeSet<u64> = hashes.iter().copied().chain([hash]).collect();
    assert_eq!(distinct.len(), hashes.len() + 1);
    // A run's ended day.
    let mut world = world_with_plan(true);
    staff(
        &mut world,
        CommandKind::AddRun {
            plan: plan(1),
            run: run_spec(&[5], [1, 2, 3, 4]),
        },
    )
    .unwrap();
    let open = world.state_hash();
    staff(
        &mut world,
        CommandKind::EndRun {
            plan: plan(1),
            run: 1,
        },
    )
    .unwrap();
    assert_ne!(world.state_hash(), open);
}

// ---------------------------------------------------------------------------
// Interrupts and the staff's notices (decision 0107, step 4, rulings 31 to
// 35). Base 2 (Allied, at OUT) holds 100 t. Its daily use is the upkeep of the
// aircraft of air group 1 at it, 1 t each, so a use of 10 t a day (10
// aircraft) is 10 days of supply at 100 t. A test sets the stock before the
// day it resolves: the morning reads it as it is then.
// ---------------------------------------------------------------------------

fn interrupt_id(n: u32) -> InterruptId {
    InterruptId::new(n)
}

fn set_below(n: u32, base_n: u32, days: u32) -> CommandKind {
    CommandKind::SetInterrupt(InterruptSetting::Set {
        interrupt: interrupt_id(n),
        condition: InterruptCondition::SupplyBelow {
            base: base(base_n),
            days,
        },
    })
}

fn switch(kind: StandingKind, on: bool) -> CommandKind {
    CommandKind::SetInterrupt(InterruptSetting::Standing { kind, on })
}

fn acknowledge_set(n: u32) -> CommandKind {
    CommandKind::Acknowledge {
        interrupt: InterruptRef::Set(interrupt_id(n)),
    }
}

fn acknowledge_standing(kind: StandingKind, subject: StandingSubject) -> CommandKind {
    CommandKind::Acknowledge {
        interrupt: InterruptRef::Standing { kind, subject },
    }
}

/// Puts `aircraft` aircraft of air group 1 at base 2 (an airfield), at 1 t a
/// day each, so that the base's planned use is `aircraft` t a day.
fn put_use_on_base_2(world: &mut World, aircraft: u32) {
    world.ref_data.air_supply = Some(crate::refdata::AirSupplyFigures {
        sortie_supply_t: Fixed::ZERO,
        sortie_supply_t_per_100_nm: Fixed::ZERO,
        daily_supply_t_per_aircraft: Fixed::ONE,
        repair_supply_t: Fixed::ZERO,
    });
    world
        .truth
        .bases
        .get_mut(&base(2))
        .expect("base 2")
        .airfield = true;
    let group = crate::ids::AirGroupId::new(1);
    let mut air_group = crate::state::AirGroup::new(
        group,
        "Group 1".to_string(),
        A,
        crate::state::AirGroupLocation::Base(base(2)),
        crate::ids::AircraftTypeId::new(1),
    );
    air_group.ready = aircraft;
    air_group.strength = aircraft;
    world.truth.air_groups.insert(group, air_group);
}

fn set_stock(world: &mut World, base_n: u32, tons: i64) {
    world
        .truth
        .bases
        .get_mut(&base(base_n))
        .expect("the base exists")
        .supply_t = t(tons);
}

/// A world with a use of 10 t a day on base 2 and interrupt 1 set below
/// `days` days there, taken in.
fn world_with_interrupt(days: u32) -> World {
    let mut world = world();
    put_use_on_base_2(&mut world, 10);
    player(&mut world, set_below(1, 2, days)).unwrap();
    world
}

/// Resolves one day with the base 2 stock at `tons` when the morning reads it.
fn day_with_stock(world: &mut World, tons: i64) {
    set_stock(world, 2, tons);
    world.resolve_day().unwrap();
}

/// The firings of `side` on `day`, in the order they were recorded.
fn firings(world: &World, side: Side, day: u32) -> Vec<InterruptRef> {
    world
        .events
        .iter()
        .filter(|event| event.day == day)
        .filter_map(|event| match &event.kind {
            EventKind::InterruptFired {
                side: owner,
                interrupt,
                ..
            } if *owner == side => Some(*interrupt),
            _ => None,
        })
        .collect()
}

fn all_firings(world: &World, side: Side) -> usize {
    world
        .events
        .iter()
        .filter(|event| matches!(&event.kind, EventKind::InterruptFired { side: owner, .. } if *owner == side))
        .count()
}

fn status_of(world: &World, n: u32) -> InterruptStatus {
    state(world, A).interrupts[&interrupt_id(n)].status
}

fn set_ref(n: u32) -> InterruptRef {
    InterruptRef::Set(interrupt_id(n))
}

#[test]
fn a_threshold_already_true_when_set_fires_that_morning_once_and_not_again_while_true() {
    let mut world = world_with_interrupt(10);
    assert_eq!(status_of(&world, 1), InterruptStatus::default());
    day_with_stock(&mut world, 50); // 5 days, below 10
    assert_eq!(firings(&world, A, 1), vec![set_ref(1)]);
    let after = status_of(&world, 1);
    assert_eq!(
        after,
        InterruptStatus {
            armed: false,
            last_fired: Some(1),
            acknowledged: false
        }
    );
    // True on the next two mornings too: no firing.
    day_with_stock(&mut world, 40);
    day_with_stock(&mut world, 30);
    assert_eq!(all_firings(&world, A), 1);
    assert_eq!(status_of(&world, 1), after);
    // The enemy has none.
    assert_eq!(all_firings(&world, J), 0);
}

#[test]
fn an_interrupt_is_armed_again_on_a_morning_it_is_false_and_fires_on_the_next_crossing() {
    let mut world = world_with_interrupt(10);
    day_with_stock(&mut world, 50);
    assert_eq!(all_firings(&world, A), 1);
    day_with_stock(&mut world, 200); // 20 days: not below
    assert!(status_of(&world, 1).armed);
    assert_eq!(status_of(&world, 1).last_fired, Some(1));
    assert_eq!(all_firings(&world, A), 1);
    day_with_stock(&mut world, 60); // 6 days: crosses again
    assert_eq!(firings(&world, A, 3), vec![set_ref(1)]);
    assert_eq!(status_of(&world, 1).last_fired, Some(3));
    assert!(!status_of(&world, 1).armed);
}

#[test]
fn a_reading_equal_to_the_threshold_is_not_below_it() {
    let mut world = world_with_interrupt(10);
    day_with_stock(&mut world, 100); // exactly 10 days
    assert_eq!(all_firings(&world, A), 0);
    assert!(status_of(&world, 1).armed);
    day_with_stock(&mut world, 99); // 9.9 days
    assert_eq!(all_firings(&world, A), 1);
    assert!(super::interrupt::figure_is_below(Some(t(9)), 10));
    assert!(!super::interrupt::figure_is_below(Some(t(10)), 10));
    assert!(!super::interrupt::figure_is_below(None, 10));
}

#[test]
fn a_hex_with_no_figure_never_fires_and_arms_the_interrupt_again() {
    // No planned use at base 2: it has no figure.
    let mut world = world();
    player(&mut world, set_below(1, 2, 10)).unwrap();
    day_with_stock(&mut world, 5);
    assert_eq!(all_firings(&world, A), 0);
    assert!(status_of(&world, 1).armed);
    // Fired, then the figure goes: armed again, with no firing.
    let mut world = world_with_interrupt(10);
    day_with_stock(&mut world, 50);
    assert!(!status_of(&world, 1).armed);
    world.truth.air_groups.clear();
    day_with_stock(&mut world, 50);
    assert!(status_of(&world, 1).armed);
    assert_eq!(all_firings(&world, A), 1);
}

#[test]
fn acknowledging_arms_nothing_and_a_new_firing_is_not_acknowledged() {
    let mut world = world_with_interrupt(10);
    // Never fired.
    refused(
        &mut world,
        Issuer::Player,
        acknowledge_set(1),
        CommandRefusal::NotFired(set_ref(1)),
    );
    refused(
        &mut world,
        Issuer::Player,
        acknowledge_set(7),
        CommandRefusal::NoSuchInterrupt(interrupt_id(7)),
    );
    day_with_stock(&mut world, 50);
    player(&mut world, acknowledge_set(1)).unwrap();
    let status = status_of(&world, 1);
    assert!(status.acknowledged);
    assert!(!status.armed, "acknowledging arms nothing");
    // Still true: no new firing, the status unchanged.
    day_with_stock(&mut world, 50);
    assert_eq!(all_firings(&world, A), 1);
    assert_eq!(status_of(&world, 1), status);
    // Acknowledged already.
    refused(
        &mut world,
        Issuer::Player,
        acknowledge_set(1),
        CommandRefusal::AlreadyAcknowledged(set_ref(1)),
    );
    // A new firing is not acknowledged.
    day_with_stock(&mut world, 500);
    day_with_stock(&mut world, 50);
    assert_eq!(all_firings(&world, A), 2);
    assert!(!status_of(&world, 1).acknowledged);
    player(&mut world, acknowledge_set(1)).unwrap();
}

#[test]
fn setting_an_interrupt_again_is_a_new_record_armed_and_never_fired() {
    let mut world = world_with_interrupt(10);
    day_with_stock(&mut world, 50);
    assert!(!status_of(&world, 1).armed);
    // Set again (a day later: the condition still holds, so it fires again
    // that morning).
    player(&mut world, set_below(1, 2, 20)).unwrap();
    let record = state(&world, A).interrupts[&interrupt_id(1)];
    assert_eq!(record.status, InterruptStatus::default());
    assert_eq!(record.set_day, 2);
    assert_eq!(
        record.condition,
        InterruptCondition::SupplyBelow {
            base: base(2),
            days: 20
        }
    );
    day_with_stock(&mut world, 50);
    assert_eq!(firings(&world, A, 2), vec![set_ref(1)]);
}

#[test]
fn the_commands_that_set_and_acknowledge_interrupts_are_refused_for_these_reasons() {
    let mut world = world();
    refused(
        &mut world,
        Issuer::Player,
        set_below(0, 2, 10),
        CommandRefusal::InterruptNumberNotUsable(interrupt_id(0)),
    );
    // A base of the enemy and a base that does not exist are refused alike.
    refused(
        &mut world,
        Issuer::Player,
        set_below(1, 9, 10),
        CommandRefusal::NotYourBase(base(9)),
    );
    refused(
        &mut world,
        Issuer::Player,
        set_below(1, 99, 10),
        CommandRefusal::NotYourBase(base(99)),
    );
    refused(
        &mut world,
        Issuer::Player,
        set_below(1, 2, 0),
        CommandRefusal::NoDays,
    );
    // The staff may not give either.
    refused(
        &mut world,
        Issuer::Staff,
        set_below(1, 2, 10),
        CommandRefusal::NotYours,
    );
    refused(
        &mut world,
        Issuer::Staff,
        switch(StandingKind::ShipLost, false),
        CommandRefusal::NotYours,
    );
    refused(
        &mut world,
        Issuer::Staff,
        acknowledge_set(1),
        CommandRefusal::NotYours,
    );
    // A standing subject the side does not own, or that does not exist.
    refused(
        &mut world,
        Issuer::Player,
        acknowledge_standing(StandingKind::ShipLost, StandingSubject::Ship(ship(20))),
        CommandRefusal::NotYourShip(ship(20)),
    );
    refused(
        &mut world,
        Issuer::Player,
        acknowledge_standing(StandingKind::ShipLost, StandingSubject::Ship(ship(99))),
        CommandRefusal::NotYourShip(ship(99)),
    );
    refused(
        &mut world,
        Issuer::Player,
        acknowledge_standing(StandingKind::CannotMeet, StandingSubject::Plan(plan(4))),
        CommandRefusal::NoSuchPlan(plan(4)),
    );
    // Switching a kind to the state it is in is accepted and changes nothing.
    let before = world.command.clone();
    player(&mut world, switch(StandingKind::CannotMeet, true)).unwrap();
    assert_eq!(world.command, before);
    player(&mut world, switch(StandingKind::CannotMeet, false)).unwrap();
    player(&mut world, switch(StandingKind::CannotMeet, false)).unwrap();
    assert_eq!(
        state(&world, A).standing_off,
        BTreeSet::from([StandingKind::CannotMeet])
    );
    // Their text.
    assert_eq!(
        set_below(1, 2, 35).to_string(),
        "set interrupt:1: base:2 below 35 days of supply"
    );
    assert_eq!(
        switch(StandingKind::CannotMeet, false).to_string(),
        "switch off the standing interrupt cannot_meet"
    );
    assert_eq!(acknowledge_set(1).to_string(), "acknowledge interrupt:1");
    assert_eq!(
        acknowledge_standing(StandingKind::WaitsForLeave, StandingSubject::Plan(plan(1)))
            .to_string(),
        "acknowledge the standing interrupt waits_for_leave for plan:1"
    );
}

/// A hand-in of the staff with one reason of no order for each notice, each
/// notice resting on its own.
fn hand_in_of(notices: &[(NoticeKind, u32)]) -> StaffDecisions {
    StaffDecisions {
        no_orders: notices
            .iter()
            .map(|(_, plan_n)| reason(1, &format!("a decision on plan:{plan_n}")))
            .collect(),
        notices: notices
            .iter()
            .enumerate()
            .map(|(i, (kind, plan_n))| StaffNotice {
                kind: *kind,
                plan: plan(*plan_n),
                run: None,
                rests_on: NoticeBasis::NoOrder(i),
                pursuit: None,
                follow: None,
            })
            .collect(),
        ..StaffDecisions::default()
    }
}

fn cannot_meet() -> NoticeKind {
    NoticeKind::CannotMeet
}

fn standing_cannot_meet(plan_n: u32) -> InterruptRef {
    InterruptRef::Standing {
        kind: StandingKind::CannotMeet,
        subject: StandingSubject::Plan(plan(plan_n)),
    }
}

fn day_with_notices(world: &mut World, notices: &[(NoticeKind, u32)]) {
    world.submit_staff(A, hand_in_of(notices));
    world.resolve_day().unwrap();
}

#[test]
fn a_standing_cannot_meet_fires_the_first_morning_of_a_run_of_days_and_again_only_after_a_gap() {
    let mut world = world();
    day_with_notices(&mut world, &[(cannot_meet(), 1)]);
    assert_eq!(firings(&world, A, 1), vec![standing_cannot_meet(1)]);
    // Three more mornings with the notice: no more firings.
    day_with_notices(&mut world, &[(cannot_meet(), 1)]);
    day_with_notices(&mut world, &[(cannot_meet(), 1)]);
    assert_eq!(all_firings(&world, A), 1);
    let record =
        state(&world, A).standing[&(StandingKind::CannotMeet, StandingSubject::Plan(plan(1)))];
    assert!(!record.armed);
    assert_eq!(record.last_fired, Some(1));
    // A morning without the notice arms it; the next with it fires.
    day_with_notices(&mut world, &[]);
    let record =
        state(&world, A).standing[&(StandingKind::CannotMeet, StandingSubject::Plan(plan(1)))];
    assert!(record.armed);
    day_with_notices(&mut world, &[(cannot_meet(), 1)]);
    assert_eq!(firings(&world, A, 5), vec![standing_cannot_meet(1)]);
    assert_eq!(all_firings(&world, A), 2);
    // The enemy has none.
    assert_eq!(all_firings(&world, J), 0);
}

#[test]
fn a_standing_interrupt_is_armed_apart_for_each_plan() {
    let mut world = world();
    day_with_notices(&mut world, &[(cannot_meet(), 1)]);
    // Plan 2 joins: it fires; plan 1, still true, does not.
    day_with_notices(&mut world, &[(cannot_meet(), 1), (cannot_meet(), 2)]);
    assert_eq!(firings(&world, A, 2), vec![standing_cannot_meet(2)]);
    // Plan 1 drops out for a morning, plan 2 stays: only plan 1 is armed.
    day_with_notices(&mut world, &[(cannot_meet(), 2)]);
    day_with_notices(&mut world, &[(cannot_meet(), 1), (cannot_meet(), 2)]);
    assert_eq!(firings(&world, A, 4), vec![standing_cannot_meet(1)]);
}

#[test]
fn a_waits_for_leave_notice_of_either_leave_fires_the_standing_waits_for_leave() {
    let mut world = world();
    day_with_notices(&mut world, &[(NoticeKind::WaitsForLeave(Leave::Launch), 1)]);
    let waits = InterruptRef::Standing {
        kind: StandingKind::WaitsForLeave,
        subject: StandingSubject::Plan(plan(1)),
    };
    assert_eq!(firings(&world, A, 1), vec![waits]);
    // The other leave, the same plan, the same run of days: no new firing.
    day_with_notices(
        &mut world,
        &[(NoticeKind::WaitsForLeave(Leave::Prepare), 1)],
    );
    assert_eq!(all_firings(&world, A), 1);
    // A run ended is no standing kind.
    day_with_notices(&mut world, &[(NoticeKind::RunEnded, 1)]);
    assert_eq!(all_firings(&world, A), 1);
    // Both kinds on one morning fire in kind order, cannot_meet first.
    let mut world = world_for_two_kinds();
    assert_eq!(
        firings(&world, A, 1),
        vec![
            standing_cannot_meet(2),
            InterruptRef::Standing {
                kind: StandingKind::WaitsForLeave,
                subject: StandingSubject::Plan(plan(1)),
            }
        ]
    );
    world.resolve_day().unwrap();
}

fn world_for_two_kinds() -> World {
    let mut world = world();
    day_with_notices(
        &mut world,
        &[
            (NoticeKind::WaitsForLeave(Leave::Prepare), 1),
            (cannot_meet(), 2),
        ],
    );
    world
}

#[test]
fn switching_a_standing_kind_off_stops_it_and_on_again_lets_it_fire() {
    let mut world = world();
    player(&mut world, switch(StandingKind::CannotMeet, false)).unwrap();
    day_with_notices(&mut world, &[(cannot_meet(), 1)]);
    assert_eq!(all_firings(&world, A), 0);
    assert!(
        state(&world, A).standing.is_empty(),
        "a kind off is not tested"
    );
    // On again: the notice still comes, and it fires.
    player(&mut world, switch(StandingKind::CannotMeet, true)).unwrap();
    day_with_notices(&mut world, &[(cannot_meet(), 1)]);
    assert_eq!(firings(&world, A, 2), vec![standing_cannot_meet(1)]);
    // Off again: the record is left as it is, and a gap does not arm it.
    player(&mut world, switch(StandingKind::CannotMeet, false)).unwrap();
    day_with_notices(&mut world, &[]);
    let record =
        state(&world, A).standing[&(StandingKind::CannotMeet, StandingSubject::Plan(plan(1)))];
    assert!(!record.armed);
    assert_eq!(record.last_fired, Some(2));
}

#[test]
fn a_notice_cites_its_reasons_event_and_a_firing_cites_the_notice() {
    let mut world = world();
    // One notice resting on a staff command (refused here), one on a
    // decision to give no order, one resting on nothing.
    let decisions = StaffDecisions {
        commands: vec![StaffCommand {
            kind: CommandKind::Approve { plan: plan(9) },
            reason: reason(1, "the command's reason"),
        }],
        no_orders: vec![reason(1, "the no-order reason")],
        notices: vec![
            StaffNotice {
                kind: cannot_meet(),
                plan: plan(1),
                run: None,
                rests_on: NoticeBasis::Command(0),
                pursuit: None,
                follow: None,
            },
            StaffNotice {
                kind: NoticeKind::RunEnded,
                plan: plan(1),
                run: Some(3),
                rests_on: NoticeBasis::NoOrder(0),
                pursuit: None,
                follow: None,
            },
            StaffNotice {
                kind: NoticeKind::WaitsForLeave(Leave::Launch),
                plan: plan(2),
                run: None,
                rests_on: NoticeBasis::NoOrder(5),
                pursuit: None,
                follow: None,
            },
        ],
        ..StaffDecisions::default()
    };
    world.submit_staff(A, decisions);
    world.resolve_day().unwrap();
    let log = log(&world);
    let reason_event = |worked: &str| {
        log.iter()
            .find(|e| matches!(&e.kind, EventKind::StaffReason { reason, .. } if reason.worked == worked))
            .expect("the reason is recorded")
            .id
    };
    let notice = |kind: NoticeKind| {
        log.iter()
            .find(|e| matches!(&e.kind, EventKind::StaffNotice { kind: k, .. } if *k == kind))
            .expect("the notice is recorded")
    };
    assert_eq!(
        notice(cannot_meet()).causes,
        vec![Cause::Event(reason_event("the command's reason"))]
    );
    assert_eq!(
        notice(NoticeKind::RunEnded).causes,
        vec![Cause::Event(reason_event("the no-order reason"))]
    );
    assert!(notice(NoticeKind::WaitsForLeave(Leave::Launch))
        .causes
        .is_empty());
    // The notices follow the reasons, in the order handed in.
    let order: Vec<usize> = log
        .iter()
        .enumerate()
        .filter(|(_, e)| {
            matches!(
                e.kind,
                EventKind::StaffNotice { .. } | EventKind::StaffReason { .. }
            )
        })
        .map(|(i, _)| i)
        .collect();
    let kinds: Vec<bool> = order
        .iter()
        .map(|i| matches!(log[*i].kind, EventKind::StaffNotice { .. }))
        .collect();
    assert_eq!(kinds, vec![false, false, true, true, true]);
    // The firing of the standing cannot_meet cites the notice of the morning.
    let fired = log
        .iter()
        .find(|e| matches!(e.kind, EventKind::InterruptFired { .. }))
        .expect("a firing");
    assert_eq!(fired.causes, vec![Cause::Event(notice(cannot_meet()).id)]);
    assert!(
        fired.id > notice(cannot_meet()).id,
        "the firing follows the notice"
    );
    // Their text.
    assert_eq!(
        notice(cannot_meet()).kind.to_string(),
        "Allied staff's notice: plan:1 cannot meet its intent"
    );
    assert_eq!(
        notice(NoticeKind::WaitsForLeave(Leave::Launch))
            .kind
            .to_string(),
        "Allied staff's notice: plan:2 waits for leave to launch"
    );
    assert_eq!(
        notice(NoticeKind::RunEnded).kind.to_string(),
        "Allied staff's notice: run 3 of plan:1 has ended"
    );
    assert_eq!(
        fired.kind.to_string(),
        "Allied standing interrupt fired: plan:1 cannot meet its intent"
    );
}

#[test]
fn a_set_interrupts_firing_cites_the_command_that_set_it_and_carries_a_breakdown() {
    let mut world = world();
    put_use_on_base_2(&mut world, 10);
    // An earlier setting of the same number, then the one that stands.
    player(&mut world, set_below(1, 2, 5)).unwrap();
    player(&mut world, set_below(2, 2, 9)).unwrap();
    player(&mut world, set_below(1, 2, 10)).unwrap();
    day_with_stock(&mut world, 90);
    let log = log(&world);
    let taken = |n: u32| {
        log.iter()
            .filter(|e| {
                matches!(&e.kind, EventKind::CommandTaken { command, .. }
                    if matches!(command, CommandKind::SetInterrupt(InterruptSetting::Set { interrupt, .. }) if *interrupt == interrupt_id(n)))
            })
            .map(|e| e.id)
            .next_back()
            .expect("the command was taken")
    };
    let fired: Vec<&Event> = log
        .iter()
        .filter(|e| matches!(e.kind, EventKind::InterruptFired { .. }))
        .collect();
    // 90 t over 10 t a day is 9 days: interrupt 1 (below 10) fires; 2
    // (below 9) does not.
    assert_eq!(fired.len(), 1);
    assert_eq!(fired[0].causes, vec![Cause::Event(taken(1))]);
    assert_eq!(
        fired[0].kind.to_string(),
        "Allied interrupt:1 fired: base:2 at 9 days of supply, below 10"
    );
    let breakdown = fired[0].breakdown.as_ref().expect("breakdowns are on");
    let factors: Vec<(&str, Fixed)> = breakdown
        .factors
        .iter()
        .map(|f| (f.name, f.value))
        .collect();
    assert_eq!(
        factors,
        vec![
            ("stock_t", t(90)),
            ("planned_use_t", t(10)),
            ("days_of_supply", t(9)),
            ("threshold_days", t(10)),
        ]
    );
    assert_eq!(breakdown.result, t(9));
}

#[test]
fn recording_the_breakdown_of_a_firing_changes_neither_what_happens_nor_the_hash() {
    let play = |breakdowns: bool| {
        let mut world = world_with_interrupt(10);
        world.set_config(crate::world::SimConfig {
            record_breakdowns: breakdowns,
        });
        day_with_stock(&mut world, 50);
        day_with_stock(&mut world, 500);
        day_with_stock(&mut world, 50);
        world
    };
    let (with, without) = (play(true), play(false));
    assert_eq!(with.state_hash(), without.state_hash());
    assert_eq!(all_firings(&with, A), 2);
    assert_eq!(with.day_hashes, without.day_hashes);
    let has_breakdown = |w: &World| {
        w.events
            .iter()
            .any(|e| matches!(e.kind, EventKind::InterruptFired { .. }) && e.breakdown.is_some())
    };
    assert!(has_breakdown(&with));
    assert!(!has_breakdown(&without));
}

#[test]
fn the_side_sees_its_notices_and_firings_whole_and_the_enemy_sees_none() {
    use crate::side_events::SideEventKind;
    let mut world = world();
    put_use_on_base_2(&mut world, 10);
    player(&mut world, set_below(1, 2, 10)).unwrap();
    set_stock(&mut world, 2, 50);
    world.submit_staff(A, hand_in_of(&[(cannot_meet(), 1)]));
    world.resolve_day().unwrap();
    let count = |side: Side, pick: fn(&EventKind) -> bool| {
        world
            .side_view(side)
            .events()
            .iter()
            .filter(|e| matches!(&e.kind, SideEventKind::Whole(kind) if pick(kind)))
            .count()
    };
    let notice = |k: &EventKind| matches!(k, EventKind::StaffNotice { .. });
    let fired = |k: &EventKind| matches!(k, EventKind::InterruptFired { .. });
    assert_eq!(count(A, notice), 1);
    assert_eq!(count(A, fired), 2); // the set one and the standing one
    assert_eq!(count(J, notice), 0);
    assert_eq!(count(J, fired), 0);
    // Nothing of them is in what the enemy sees at all.
    assert!(texts(&world.side_view(J).events())
        .iter()
        .all(|text| !text.contains("interrupt") && !text.contains("notice")));
    // The side's causes still resolve for it: the firing's causes are its own.
    for event in world.side_view(A).events() {
        if matches!(
            &event.kind,
            SideEventKind::Whole(EventKind::InterruptFired { .. })
        ) {
            assert!(!event.causes.is_empty());
        }
    }
}

#[test]
fn the_hash_changes_when_an_interrupts_status_changes() {
    let mut world = world_with_interrupt(10);
    let set = world.state_hash();
    day_with_stock(&mut world, 50);
    // (The day's own changes move the hash too; compare states of one day.)
    let fired = world.state_hash();
    assert_ne!(set, fired);
    let acknowledged = {
        let mut copy = world.clone();
        player(&mut copy, acknowledge_set(1)).unwrap();
        copy.state_hash()
    };
    assert_ne!(fired, acknowledged, "acknowledged is hashed");
    let rearmed = {
        let mut copy = world.clone();
        copy.command
            .get_mut(&A)
            .unwrap()
            .interrupts
            .get_mut(&interrupt_id(1))
            .unwrap()
            .status
            .armed = true;
        copy.state_hash()
    };
    assert_ne!(fired, rearmed, "armed is hashed");
    let off = {
        let mut copy = world.clone();
        player(&mut copy, switch(StandingKind::ShipLost, false)).unwrap();
        copy.state_hash()
    };
    assert_ne!(fired, off, "a kind switched off is hashed");
    let standing = {
        let mut copy = world.clone();
        copy.command.get_mut(&A).unwrap().standing.insert(
            (StandingKind::CannotMeet, StandingSubject::Plan(plan(1))),
            InterruptStatus::default(),
        );
        copy.state_hash()
    };
    assert_ne!(fired, standing, "a standing record is hashed");
    // And a state with none is not empty.
    assert!(!state(&world, A).is_empty());
}

/// A world where ship 3 (of the side's force 2) is sunk, recorded on `day` as
/// the engine records a sinking.
fn sink_ship_3_on(world: &mut World, day: u32) -> crate::ids::EventId {
    {
        let record = world.truth.ships.get_mut(&ship(3)).expect("ship 3");
        record.location = ShipLocation::Sunk;
        record.flotation_damage = t(100);
    }
    world
        .truth
        .task_forces
        .get_mut(&tf(2))
        .expect("force 2")
        .ships
        .remove(&ship(3));
    world.events.record(
        day,
        None,
        EventKind::ShipSunk {
            round: 1,
            ship: ship(3),
            task_force: tf(2),
        },
        Vec::new(),
        None,
    )
}

#[test]
fn a_ship_lost_fires_once_the_morning_after_and_only_for_a_side_with_an_intent_in_force() {
    // An intent in force: the ship sunk on day 1 fires on the morning of day 2.
    let mut world = world();
    player(&mut world, set_intent(1)).unwrap();
    world.resolve_day().unwrap();
    let sinking = sink_ship_3_on(&mut world, 1);
    world.resolve_day().unwrap();
    let lost = InterruptRef::Standing {
        kind: StandingKind::ShipLost,
        subject: StandingSubject::Ship(ship(3)),
    };
    assert_eq!(firings(&world, A, 2), vec![lost]);
    let fired = log(&world)
        .into_iter()
        .find(|e| matches!(e.kind, EventKind::InterruptFired { .. }))
        .unwrap();
    assert_eq!(fired.causes, vec![Cause::Event(sinking)]);
    assert_eq!(
        fired.kind.to_string(),
        "Allied standing interrupt fired: ship:3 was lost"
    );
    // Once: the next mornings do not fire it again.
    world.resolve_day().unwrap();
    world.resolve_day().unwrap();
    assert_eq!(all_firings(&world, A), 1);
    // A side with no intent in force gets no event, though its ship is lost.
    let mut world = self::world();
    world.resolve_day().unwrap();
    sink_ship_3_on(&mut world, 1);
    world.resolve_day().unwrap();
    assert_eq!(all_firings(&world, A), 0);
    assert!(state(&world, A).standing.is_empty());
    // An intent withdrawn is no intent in force.
    let mut world = self::world();
    player(&mut world, set_intent(1)).unwrap();
    world.resolve_day().unwrap();
    player(
        &mut world,
        CommandKind::WithdrawIntent { intent: intent(1) },
    )
    .unwrap();
    sink_ship_3_on(&mut world, 1);
    world.resolve_day().unwrap();
    assert_eq!(all_firings(&world, A), 0);
    // The enemy's ship lost is not the side's.
    let mut world = self::world();
    player(&mut world, set_intent(1)).unwrap();
    world.resolve_day().unwrap();
    {
        let record = world.truth.ships.get_mut(&ship(20)).expect("ship 20");
        record.location = ShipLocation::Sunk;
        record.flotation_damage = t(100);
    }
    world
        .truth
        .task_forces
        .get_mut(&tf(20))
        .unwrap()
        .ships
        .remove(&ship(20));
    world.events.record(
        1,
        None,
        EventKind::ShipSunk {
            round: 1,
            ship: ship(20),
            task_force: tf(20),
        },
        Vec::new(),
        None,
    );
    world.resolve_day().unwrap();
    assert_eq!(all_firings(&world, A), 0);
}

#[test]
fn a_hand_in_given_back_and_a_live_staffs_are_merged_with_the_notices_still_on_their_reasons() {
    let mut world = world();
    // Given back first: one no-order reason and one notice on it. The live
    // staff then hands in the same shape: its notice rests on its own
    // reason, which is now the second.
    world.submit_staff(A, hand_in_of(&[(cannot_meet(), 1)]));
    let live = Fixed_(StaffDecisions {
        no_orders: vec![reason(1, "the live staff's reason")],
        commands: vec![StaffCommand {
            kind: CommandKind::Approve { plan: plan(9) },
            reason: reason(1, "the live staff's command"),
        }],
        notices: vec![
            StaffNotice {
                kind: cannot_meet(),
                plan: plan(2),
                run: None,
                rests_on: NoticeBasis::NoOrder(0),
                pursuit: None,
                follow: None,
            },
            StaffNotice {
                kind: NoticeKind::RunEnded,
                plan: plan(2),
                run: Some(1),
                rests_on: NoticeBasis::Command(0),
                pursuit: None,
                follow: None,
            },
        ],
        ..StaffDecisions::default()
    });
    world
        .resolve_day_staffed(Staffs::none().with(A, &live))
        .unwrap();
    let handed = &world.staff_handed_in()[&A];
    assert_eq!(handed.no_orders.len(), 2);
    assert_eq!(handed.commands.len(), 1);
    assert_eq!(
        handed
            .notices
            .iter()
            .map(|n| n.rests_on)
            .collect::<Vec<_>>(),
        vec![
            NoticeBasis::NoOrder(0),
            NoticeBasis::NoOrder(1),
            NoticeBasis::Command(0)
        ]
    );
    let log = log(&world);
    let reason_event = |worked: &str| {
        log.iter()
            .find(|e| matches!(&e.kind, EventKind::StaffReason { reason, .. } if reason.worked == worked))
            .expect("the reason is recorded")
            .id
    };
    let notice_of = |plan_n: u32| {
        log.iter()
            .find(
                |e| matches!(&e.kind, EventKind::StaffNotice { plan: p, .. } if *p == plan(plan_n)),
            )
            .expect("the notice is recorded")
            .causes
            .clone()
    };
    assert_eq!(
        notice_of(1),
        vec![Cause::Event(reason_event("a decision on plan:1"))]
    );
    assert_eq!(
        notice_of(2),
        vec![Cause::Event(reason_event("the live staff's reason"))]
    );
    // The same shape handed in by one `submit_staff` after another.
    let mut again = self::world();
    again.submit_staff(A, hand_in_of(&[(cannot_meet(), 1)]));
    again.submit_staff(A, hand_in_of(&[(cannot_meet(), 2)]));
    again.resolve_day().unwrap();
    let handed = &again.staff_handed_in()[&A];
    assert_eq!(
        handed
            .notices
            .iter()
            .map(|n| n.rests_on)
            .collect::<Vec<_>>(),
        vec![NoticeBasis::NoOrder(0), NoticeBasis::NoOrder(1)]
    );
}

#[test]
fn a_day_with_no_notice_and_no_interrupt_records_neither() {
    let mut world = world();
    player(&mut world, set_intent(1)).unwrap();
    world.resolve_day().unwrap();
    world.resolve_day().unwrap();
    assert!(!world.events.iter().any(|e| matches!(
        e.kind,
        EventKind::StaffNotice { .. } | EventKind::InterruptFired { .. }
    )));
}

#[test]
fn a_set_interrupt_and_a_standing_one_fire_on_one_morning_in_the_order_set_then_standing() {
    let mut world = world();
    put_use_on_base_2(&mut world, 10);
    player(&mut world, set_below(2, 2, 10)).unwrap();
    player(&mut world, set_below(1, 2, 10)).unwrap();
    set_stock(&mut world, 2, 50);
    world.submit_staff(A, hand_in_of(&[(cannot_meet(), 1)]));
    world.resolve_day().unwrap();
    assert_eq!(
        firings(&world, A, 1),
        vec![set_ref(1), set_ref(2), standing_cannot_meet(1)]
    );
    // All of them before the day's orders: the first event of the day that
    // is an order's answer comes after the last firing (there is none here),
    // and the firings come after the commands' records.
    let log = log(&world);
    let taken = log
        .iter()
        .rposition(|e| matches!(e.kind, EventKind::CommandTaken { .. }))
        .unwrap();
    let first_fired = log
        .iter()
        .position(|e| matches!(e.kind, EventKind::InterruptFired { .. }))
        .unwrap();
    assert!(taken < first_fired);
}

// ---------------------------------------------------------------------------
// Controlling an island (decision 0151).
// ---------------------------------------------------------------------------

/// A world with island 1 on its map: base 1's place and a coast hex at
/// (5,0), and `control_island` of it taken in as intent 1, its plan
/// released.
fn world_with_island() -> World {
    let mut world = world();
    world.map.insert(
        Hex::new(5, 0),
        HexData {
            kind: crate::map::HexKind::Coast,
            ..HexData::open_sea()
        },
    );
    world.map.add_island(crate::map::Island {
        number: 1,
        name: "Placeholder island".to_string(),
        places: vec![
            crate::map::IslandPlace {
                base: Some(base(1)),
                hex: HOME,
            },
            crate::map::IslandPlace {
                base: None,
                hex: Hex::new(5, 0),
            },
        ],
    });
    player(&mut world, island_intent(1, 1)).unwrap();
    player(&mut world, CommandKind::Approve { plan: plan(1) }).unwrap();
    player(&mut world, CommandKind::Approve { plan: plan(1) }).unwrap();
    assert_eq!(state(&world, A).plans[&plan(1)].state, PlanState::Released);
    world
}

fn island_intent(n: u32, island: u32) -> CommandKind {
    CommandKind::SetIntent {
        intent: intent(n),
        kind: IntentKind::ControlIsland {
            island,
            by_day: None,
        },
    }
}

fn form_under(parent: u32, kind: IntentKind) -> CommandKind {
    CommandKind::FormIntent {
        parent: intent(parent),
        kind,
    }
}

fn island_hold(n: u32) -> IntentKind {
    IntentKind::Hold { base: base(n) }
}

fn island_take(hex: Hex) -> IntentKind {
    IntentKind::TakePlace { hex, by_day: None }
}

fn island_invariants(world: &World) {
    for side in [A, J] {
        let found = crate::invariants::check_command(
            side,
            world.command_state(side),
            &world.truth,
            world.truth.day,
        );
        assert!(found.is_empty(), "{found:?}");
    }
}

/// Item 3: an island of the scenario is taken in and one the map does not
/// have is refused, with nothing changed.
#[test]
fn controlling_an_island_is_taken_in_only_for_an_island_the_map_has() {
    let mut world = world();
    world.map.add_island(crate::map::Island {
        number: 1,
        name: "Placeholder island".to_string(),
        places: vec![crate::map::IslandPlace {
            base: Some(base(1)),
            hex: HOME,
        }],
    });
    refused(
        &mut world,
        Issuer::Player,
        island_intent(1, 2),
        CommandRefusal::NoSuchIsland(2),
    );
    assert_eq!(
        CommandRefusal::NoSuchIsland(2).to_string(),
        "island 2 is not an island of the scenario"
    );
    player(&mut world, island_intent(1, 1)).unwrap();
    let found = &state(&world, A).intents[&intent(1)];
    assert_eq!(
        found.kind,
        IntentKind::ControlIsland {
            island: 1,
            by_day: None
        }
    );
    assert_eq!(found.parent, None);
    assert_eq!(found.addressee, Addressee::Area);
    assert_eq!(state(&world, A).plans[&plan(1)].state, PlanState::Draft);
    island_invariants(&world);
}

/// Item 8b: the staff withdraws an intent it formed, and what lies beneath
/// it, each with an event of its own; it may not withdraw the commander's.
#[test]
fn the_staff_withdraws_an_intent_it_formed_and_what_lies_beneath_it() {
    let mut world = world_with_island();
    staff(&mut world, form_under(1, island_hold(1))).unwrap();
    staff(
        &mut world,
        form_under(
            2,
            IntentKind::KeepSupplied {
                base: base(1),
                floor_days: 10,
                interval_days: Some(15),
            },
        ),
    )
    .unwrap();
    assert!(state(&world, A).intents[&intent(3)].in_force());
    let before = world.events.len();
    staff(
        &mut world,
        CommandKind::WithdrawIntent { intent: intent(2) },
    )
    .unwrap();
    let state_now = state(&world, A);
    assert_eq!(state_now.intents[&intent(2)].withdrawn_day, Some(1));
    assert_eq!(state_now.intents[&intent(3)].withdrawn_day, Some(1));
    // The island and its plan stand.
    assert!(state_now.intents[&intent(1)].in_force());
    assert_eq!(state_now.plans[&plan(1)].state, PlanState::Released);
    let withdrawn: Vec<_> = world
        .events
        .from_index(before)
        .iter()
        .filter_map(|event| match &event.kind {
            EventKind::IntentWithdrawn {
                intent: below,
                parent,
                ..
            } => Some((*below, *parent)),
            _ => None,
        })
        .collect();
    assert_eq!(withdrawn, vec![(intent(3), intent(2))]);
    island_invariants(&world);
}

#[test]
fn the_staff_may_not_withdraw_an_intent_the_commander_made() {
    let mut world = world_with_island();
    refused(
        &mut world,
        Issuer::Staff,
        CommandKind::WithdrawIntent { intent: intent(1) },
        CommandRefusal::NotYours,
    );
    // The commander still may not withdraw one the staff made.
    staff(&mut world, form_under(1, island_hold(1))).unwrap();
    refused(
        &mut world,
        Issuer::Player,
        CommandKind::WithdrawIntent { intent: intent(2) },
        CommandRefusal::MadeByStaff(intent(2)),
    );
}

/// Item 9: a "take a place" formed under an island drafts a plan of its own;
/// a hold formed under it shares the island's.
#[test]
fn a_take_under_an_island_has_a_plan_of_its_own_and_a_hold_shares_the_islands() {
    let mut world = world_with_island();
    let before = world.events.len();
    staff(&mut world, form_under(1, island_take(Hex::new(5, 0)))).unwrap();
    let events: Vec<_> = world.events.from_index(before).to_vec();
    assert!(matches!(events[0].kind, EventKind::CommandTaken { .. }));
    assert!(matches!(events[1].kind, EventKind::IntentFormed { .. }));
    assert_eq!(
        events[2].kind,
        EventKind::PlanDrafted {
            side: A,
            plan: plan(2),
            intent: intent(2)
        }
    );
    let take = &state(&world, A).intents[&intent(2)];
    assert_eq!(take.parent, Some(intent(1)));
    assert_eq!(take.maker, Maker::Staff);
    assert_eq!(take.plan, plan(2));
    assert_eq!(state(&world, A).plans[&plan(2)].intent, intent(2));
    assert_eq!(state(&world, A).plans[&plan(2)].state, PlanState::Draft);
    // A hold formed under the island is served by the island's plan.
    staff(&mut world, form_under(1, island_hold(1))).unwrap();
    let held = &state(&world, A).intents[&intent(3)];
    assert_eq!(held.plan, plan(1));
    assert_eq!(state(&world, A).next_plan, 3);
    island_invariants(&world);
}

/// Items 3 and 12: withdrawing the island ends the plan of the take beneath
/// it as a root's ends, complete if released, else cancelled.
#[test]
fn withdrawing_an_island_ends_the_plan_of_the_take_beneath_it() {
    // A take whose plan is preparing is cancelled.
    let mut world = world_with_island();
    staff(&mut world, form_under(1, island_take(Hex::new(5, 0)))).unwrap();
    player(&mut world, CommandKind::Approve { plan: plan(2) }).unwrap();
    assert_eq!(state(&world, A).plans[&plan(2)].state, PlanState::Preparing);
    let before = world.events.len();
    player(
        &mut world,
        CommandKind::WithdrawIntent { intent: intent(1) },
    )
    .unwrap();
    assert_eq!(state(&world, A).intents[&intent(2)].withdrawn_day, Some(1));
    assert_eq!(state(&world, A).plans[&plan(2)].state, PlanState::Cancelled);
    assert_eq!(state(&world, A).plans[&plan(1)].state, PlanState::Complete);
    let changed: Vec<_> = world
        .events
        .from_index(before)
        .iter()
        .filter_map(|event| match &event.kind {
            EventKind::PlanStateChanged { plan, to, .. } => Some((*plan, *to)),
            _ => None,
        })
        .collect();
    assert_eq!(
        changed,
        vec![
            (plan(2), PlanState::Cancelled),
            (plan(1), PlanState::Complete)
        ]
    );
    island_invariants(&world);
    // A take whose plan is released is complete.
    let mut world = world_with_island();
    staff(&mut world, form_under(1, island_take(Hex::new(5, 0)))).unwrap();
    player(&mut world, CommandKind::Approve { plan: plan(2) }).unwrap();
    player(&mut world, CommandKind::Approve { plan: plan(2) }).unwrap();
    player(
        &mut world,
        CommandKind::WithdrawIntent { intent: intent(1) },
    )
    .unwrap();
    assert_eq!(state(&world, A).plans[&plan(2)].state, PlanState::Complete);
    island_invariants(&world);
}

/// A world for decision 0159, A: the Allied commander holds `held`, with its
/// plan released. Allied base 3 and Japanese base 10 are added at FAR, the
/// hex of Japanese base 9.
fn world_holding(held: u32) -> World {
    let mut world = world();
    for (id, side) in [(3, A), (10, J)] {
        world.truth.bases.insert(
            base(id),
            Base::new(base(id), format!("Base {id}"), side, FAR),
        );
    }
    player(
        &mut world,
        CommandKind::SetIntent {
            intent: intent(1),
            kind: IntentKind::Hold { base: base(held) },
        },
    )
    .unwrap();
    player(&mut world, CommandKind::Approve { plan: plan(1) }).unwrap();
    player(&mut world, CommandKind::Approve { plan: plan(1) }).unwrap();
    world
}

/// Decision 0159, A, 1: "keep supplied" stands under a hold of a base that is
/// not the side's for a base of the side in the held base's hex.
#[test]
fn keep_supplied_stands_under_a_hold_for_a_base_of_the_side_in_its_hex() {
    let mut world = world_holding(9);
    staff(&mut world, form_under(1, keep_supplied(3))).unwrap();
    let formed = &state(&world, A).intents[&intent(2)];
    assert_eq!(formed.kind, keep_supplied(3));
    assert_eq!(formed.parent, Some(intent(1)));
    assert_eq!(formed.maker, Maker::Staff);
}

/// Decision 0159, A, 1: not for a base of the side in another hex, nor for
/// a base of the other side in the held base's hex.
#[test]
fn keep_supplied_under_a_hold_is_refused_in_another_hex_or_for_the_enemy() {
    let mut world = world_holding(9);
    refused(
        &mut world,
        Issuer::Staff,
        form_under(1, keep_supplied(1)),
        CommandRefusal::CannotStandUnder(intent(1)),
    );
    refused(
        &mut world,
        Issuer::Staff,
        form_under(1, keep_supplied(10)),
        CommandRefusal::CannotStandUnder(intent(1)),
    );
}

/// Decision 0144, 7, as before: "keep supplied" of the held base itself.
#[test]
fn keep_supplied_of_the_held_base_itself_stands_under_its_hold_as_before() {
    let mut world = world_holding(2);
    staff(&mut world, form_under(1, keep_supplied(2))).unwrap();
    assert_eq!(state(&world, A).intents[&intent(2)].kind, keep_supplied(2));
    // Another base of the side in another hex is refused, as before.
    refused(
        &mut world,
        Issuer::Staff,
        form_under(1, keep_supplied(1)),
        CommandRefusal::CannotStandUnder(intent(1)),
    );
}

#[test]
fn cannot_bring_enough_has_its_names_and_codes() {
    assert_eq!(NoticeKind::CannotBringEnough.code(), 8);
    assert_eq!(NoticeKind::CannotBringEnough.name(), "cannot_bring_enough");
    assert_eq!(
        NoticeKind::from_name("cannot_bring_enough"),
        Some(NoticeKind::CannotBringEnough)
    );
    assert_eq!(StandingKind::CannotBringEnough.code(), 4);
    assert_eq!(
        StandingKind::CannotBringEnough.name(),
        "cannot_bring_enough"
    );
    assert_eq!(
        StandingKind::from_name("cannot_bring_enough"),
        Some(StandingKind::CannotBringEnough)
    );
}

#[test]
fn lift_turned_back_has_its_names_and_codes() {
    assert_eq!(NoticeKind::LiftTurnedBack.code(), 9);
    assert_eq!(NoticeKind::LiftTurnedBack.name(), "lift_turned_back");
    assert_eq!(
        NoticeKind::from_name("lift_turned_back"),
        Some(NoticeKind::LiftTurnedBack)
    );
    assert_eq!(StandingKind::LiftTurnedBack.code(), 5);
    assert_eq!(StandingKind::LiftTurnedBack.name(), "lift_turned_back");
    assert_eq!(
        StandingKind::from_name("lift_turned_back"),
        Some(StandingKind::LiftTurnedBack)
    );
}

// ---------------------------------------------------------------------------
// Decision 0159, rule C: no ring of waits (the architect's ruling on step 3).
// ---------------------------------------------------------------------------

/// A world with three takes of the commander's, intents 1, 2 and 3, each of
/// a land hex of its own.
fn three_takes() -> World {
    let mut world = world();
    for (n, q) in [(1, 5), (2, 6), (3, 7)] {
        let hex = Hex::new(q, 0);
        world.map.insert(
            hex,
            HexData {
                kind: crate::map::HexKind::Coast,
                ..HexData::open_sea()
            },
        );
        player(
            &mut world,
            CommandKind::SetIntent {
                intent: intent(n),
                kind: IntentKind::TakePlace { hex, by_day: None },
            },
        )
        .unwrap();
    }
    world
}

fn after(on: u32, other: u32) -> CommandKind {
    CommandKind::SetLimit(Limit::After {
        intent: intent(on),
        other: Some(intent(other)),
    })
}

/// Two takes that would wait for each other: the second "after" is refused
/// as one naming its own intent is, and nothing changes.
#[test]
fn an_after_that_closes_a_ring_of_two_is_refused() {
    let mut world = three_takes();
    player(&mut world, after(1, 2)).unwrap();
    refused(
        &mut world,
        Issuer::Player,
        after(2, 1),
        CommandRefusal::NotAnEndToWaitFor(intent(1)),
    );
    refused(
        &mut world,
        Issuer::Player,
        after(1, 1),
        CommandRefusal::NotAnEndToWaitFor(intent(1)),
    );
}

/// A ring through a third take is refused too.
#[test]
fn an_after_that_closes_a_ring_of_three_is_refused() {
    let mut world = three_takes();
    player(&mut world, after(1, 2)).unwrap();
    player(&mut world, after(2, 3)).unwrap();
    refused(
        &mut world,
        Issuer::Player,
        after(3, 1),
        CommandRefusal::NotAnEndToWaitFor(intent(1)),
    );
}

/// A chain that is not a ring is taken in: 1 after 2, 2 after 3, and 1
/// after 3 as well.
#[test]
fn a_chain_of_afters_that_is_not_a_ring_is_taken_in() {
    let mut world = three_takes();
    player(&mut world, after(1, 2)).unwrap();
    player(&mut world, after(2, 3)).unwrap();
    player(&mut world, after(1, 3)).unwrap();
    assert_eq!(
        state(&world, A).intents[&intent(1)].limits.after,
        Some(intent(3))
    );
    assert_eq!(
        state(&world, A).intents[&intent(2)].limits.after,
        Some(intent(3))
    );
}

// ---- the targets limit (decision 0190) ----

fn targets(n: u32, kinds: &[ShipKind], waters_first: bool) -> CommandKind {
    CommandKind::SetLimit(Limit::Targets {
        intent: intent(n),
        targets: Some(TargetsLimit {
            kinds: kinds.to_vec(),
            waters_first,
        }),
    })
}

fn deny_intent(n: u32) -> CommandKind {
    CommandKind::SetIntent {
        intent: intent(n),
        kind: IntentKind::Deny {
            hex: Hex::new(4, 0),
            distance: 1,
        },
    }
}

#[test]
fn a_targets_limit_is_taken_read_up_the_tree_and_cleared() {
    let mut world = world();
    player(&mut world, deny_intent(1)).unwrap();
    assert_eq!(state(&world, A).targets_read_up(intent(1)), None);
    let kinds = [ShipKind::Transport, ShipKind::Carrier];
    player(&mut world, targets(1, &kinds, true)).unwrap();
    let (found, on) = state(&world, A).targets_read_up(intent(1)).unwrap();
    assert_eq!(
        (found.kinds.as_slice(), found.waters_first, on),
        (&kinds[..], true, intent(1))
    );
    // A child formed by the staff reads its parent's; its own replaces it,
    // and is not refused as wider.
    player(&mut world, CommandKind::Approve { plan: plan(1) }).unwrap();
    player(&mut world, CommandKind::Approve { plan: plan(1) }).unwrap();
    staff(
        &mut world,
        CommandKind::FormIntent {
            parent: intent(1),
            kind: IntentKind::StrikeShips { base: base(1) },
        },
    )
    .unwrap();
    assert_eq!(
        state(&world, A).targets_read_up(intent(2)).unwrap().1,
        intent(1)
    );
    player(&mut world, targets(2, &ShipKind::ALL, true)).unwrap();
    let (found, on) = state(&world, A).targets_read_up(intent(2)).unwrap();
    assert_eq!(
        (found.kinds.clone(), on),
        (ShipKind::ALL.to_vec(), intent(2))
    );
    assert_eq!(
        state(&world, A).targets_read_up(intent(1)).unwrap().1,
        intent(1)
    );
    // Cleared with no kinds.
    player(
        &mut world,
        CommandKind::SetLimit(Limit::Targets {
            intent: intent(1),
            targets: None,
        }),
    )
    .unwrap();
    assert_eq!(state(&world, A).intents[&intent(1)].limits.targets, None);
    assert!(state(&world, A).is_or_under_deny(intent(2)));
}

#[test]
fn a_targets_limit_is_refused_for_a_kind_twice_none_or_waters_first_off_a_deny() {
    let mut world = world();
    player(&mut world, set_intent(1)).unwrap();
    player(&mut world, deny_intent(2)).unwrap();
    refused(
        &mut world,
        Issuer::Player,
        targets(2, &[ShipKind::Carrier, ShipKind::Carrier], false),
        CommandRefusal::BadTargetKinds(intent(2)),
    );
    refused(
        &mut world,
        Issuer::Player,
        targets(2, &[], false),
        CommandRefusal::BadTargetKinds(intent(2)),
    );
    refused(
        &mut world,
        Issuer::Player,
        targets(1, &[ShipKind::Transport], true),
        CommandRefusal::NoWatersFirstOn(intent(1)),
    );
    refused(
        &mut world,
        Issuer::Player,
        targets(9, &[ShipKind::Transport], false),
        CommandRefusal::NoSuchIntent(intent(9)),
    );
    // Without the switch any intent takes it; on an approved plan it amends.
    player(&mut world, CommandKind::Approve { plan: plan(1) }).unwrap();
    player(&mut world, targets(1, &[ShipKind::Transport], false)).unwrap();
    assert!(matches!(
        &state(&world, A).plans[&plan(1)].amendments[0].change,
        AmendmentChange::Limit(Limit::Targets { .. })
    ));
}

#[test]
fn the_denies_naming_a_force_are_those_whose_nearest_forces_limit_names_it() {
    let mut world = world();
    player(&mut world, deny_intent(1)).unwrap();
    player(&mut world, deny_intent(2)).unwrap();
    let mut named = forces(&[], &[1]);
    named.source = None;
    player(&mut world, set_forces(2, named)).unwrap();
    let found: Vec<IntentId> = state(&world, A)
        .denies_naming(tf(1))
        .iter()
        .map(|i| i.id)
        .collect();
    assert_eq!(found, vec![intent(2)]);
    assert!(state(&world, A).denies_naming(tf(2)).is_empty());
}

// ---------------------------------------------------------------------------
// Decision 0214: a force asks leave to go after what it has found.
// ---------------------------------------------------------------------------

fn set_pursuit(n: u32, leave: Option<PursueLeave>) -> CommandKind {
    CommandKind::SetPursuit {
        intent: intent(n),
        leave,
    }
}

fn answer(force: u32, contact: u32, allow: bool) -> CommandKind {
    CommandKind::AnswerPursuit {
        force: tf(force),
        contact,
        allow,
    }
}

/// The event of the last `CommandTaken` of an `answer_pursuit` in the log.
fn last_answer_event(world: &World) -> crate::ids::EventId {
    world
        .events
        .iter()
        .filter(|e| {
            matches!(
                &e.kind,
                EventKind::CommandTaken {
                    command: CommandKind::AnswerPursuit { .. },
                    ..
                }
            )
        })
        .map(|e| e.id)
        .last()
        .expect("an answer was taken")
}

fn pursuit_find(contact: u32) -> PursuitFind {
    PursuitFind {
        contact,
        kind: ShipKind::Carrier,
        age: 2,
        reported_at: Hex::new(6, 0),
        to: Hex::new(5, 0),
        hours: Some(t(9)),
    }
}

/// A hand-in of one pursuit notice of `kind` for force `force`, resting on
/// its own no-order reason.
fn pursuit_hand_in(kind: NoticeKind, force: u32, contact: u32) -> StaffDecisions {
    StaffDecisions {
        no_orders: vec![reason(1, "a pursuit")],
        notices: vec![StaffNotice {
            kind,
            plan: plan(1),
            run: None,
            rests_on: NoticeBasis::NoOrder(0),
            pursuit: Some(PursuitNotice {
                force: tf(force),
                intent: intent(1),
                find: (kind != NoticeKind::PursuitEnded).then(|| pursuit_find(contact)),
            }),
            follow: None,
        }],
        ..StaffDecisions::default()
    }
}

fn standing_asks(force: u32) -> InterruptRef {
    InterruptRef::Standing {
        kind: StandingKind::AsksLeaveToPursue,
        subject: StandingSubject::TaskForce(tf(force)),
    }
}

#[test]
fn the_pursuit_codes_and_names_round_trip() {
    for (leave, code, name) in [
        (PursueLeave::Ask, 1, "ask"),
        (PursueLeave::Go, 2, "go"),
        (PursueLeave::Stay, 3, "stay"),
    ] {
        assert_eq!(leave.code(), code);
        assert_eq!(leave.name(), name);
        assert_eq!(PursueLeave::from_name(name), Some(leave));
    }
    assert_eq!(PursueLeave::from_name("maybe"), None);
    assert_eq!(PursuitBy::Taken.code(), 2);
    assert_eq!(
        PursuitBy::Answer(world().events.record(
            1,
            None,
            EventKind::CommandRefused {
                side: A,
                issuer: Issuer::Player,
                command: set_intent(1),
                reason: CommandRefusal::NotYours,
            },
            Vec::new(),
            None,
        ))
        .code(),
        1
    );
    for (kind, code, name) in [
        (NoticeKind::AsksLeaveToPursue, 10, "asks_leave_to_pursue"),
        (NoticeKind::Pursues, 11, "pursues"),
        (NoticeKind::PursuitEnded, 12, "pursuit_ended"),
    ] {
        assert_eq!(kind.code(), code);
        assert_eq!(kind.name(), name);
        assert_eq!(NoticeKind::from_name(name), Some(kind));
    }
    assert_eq!(StandingKind::AsksLeaveToPursue.code(), 6);
    assert_eq!(
        StandingKind::AsksLeaveToPursue.name(),
        "asks_leave_to_pursue"
    );
    assert_eq!(
        StandingKind::from_name("asks_leave_to_pursue"),
        Some(StandingKind::AsksLeaveToPursue)
    );
    assert!(StandingKind::ALL.contains(&StandingKind::AsksLeaveToPursue));
    assert_eq!(StandingSubject::TaskForce(tf(4)).code(), 3);
    assert_eq!(StandingSubject::TaskForce(tf(4)).raw(), 4);
    assert_eq!(set_pursuit(1, None).name(), "set_pursuit");
    assert_eq!(answer(1, 1, true).name(), "answer_pursuit");
}

#[test]
fn set_pursuit_sets_and_clears_a_leave_and_is_not_an_amendment() {
    // Plan 1 is approved, so a change to its intent would amend it.
    let mut world = world_with_plan(true);
    player(&mut world, set_pursuit(1, Some(PursueLeave::Stay))).unwrap();
    assert_eq!(
        state(&world, A).pursue_leaves[&intent(1)],
        PursueLeave::Stay
    );
    assert!(
        state(&world, A).plans[&plan(1)].amendments.is_empty(),
        "the setting is not an amendment of the plan"
    );
    player(&mut world, set_pursuit(1, Some(PursueLeave::Go))).unwrap();
    assert_eq!(state(&world, A).pursue_leaves[&intent(1)], PursueLeave::Go);
    player(&mut world, set_pursuit(1, None)).unwrap();
    assert!(state(&world, A).pursue_leaves.is_empty());
}

#[test]
fn set_pursuit_is_the_commanders_and_names_an_intent_in_force() {
    let mut world = world();
    player(&mut world, set_intent(1)).unwrap();
    refused(
        &mut world,
        Issuer::Staff,
        set_pursuit(1, Some(PursueLeave::Go)),
        CommandRefusal::NotYours,
    );
    refused(
        &mut world,
        Issuer::Player,
        set_pursuit(9, Some(PursueLeave::Go)),
        CommandRefusal::NoSuchIntent(intent(9)),
    );
    // A withdrawn intent is not in force.
    player(
        &mut world,
        CommandKind::WithdrawIntent { intent: intent(1) },
    )
    .unwrap();
    refused(
        &mut world,
        Issuer::Player,
        set_pursuit(1, Some(PursueLeave::Go)),
        CommandRefusal::NoSuchIntent(intent(1)),
    );
}

#[test]
fn answer_pursuit_is_the_commanders_and_names_a_force_of_the_side() {
    let mut world = world();
    refused(
        &mut world,
        Issuer::Staff,
        answer(2, 1, true),
        CommandRefusal::NotYours,
    );
    // The enemy's force and one that does not exist are refused alike.
    refused(
        &mut world,
        Issuer::Player,
        answer(20, 1, true),
        CommandRefusal::NotYourForce(tf(20)),
    );
    refused(
        &mut world,
        Issuer::Player,
        answer(99, 1, false),
        CommandRefusal::NotYourForce(tf(99)),
    );
    player(&mut world, answer(2, 1, true)).unwrap();
}

#[test]
fn pursue_read_up_takes_the_nearest_setting_and_has_two_defaults() {
    const PLACE: Hex = Hex::new(5, 0);
    let mut world = world();
    world.map.insert(
        PLACE,
        HexData {
            kind: crate::map::HexKind::Coast,
            ..HexData::open_sea()
        },
    );
    player(
        &mut world,
        CommandKind::SetIntent {
            intent: intent(1),
            kind: IntentKind::TakePlace {
                hex: PLACE,
                by_day: None,
            },
        },
    )
    .unwrap();
    player(&mut world, set_forces(1, forces(&[], &[1]))).unwrap();
    player(&mut world, CommandKind::Approve { plan: plan(1) }).unwrap();
    staff(
        &mut world,
        form_under(
            1,
            IntentKind::OperateCarriers {
                force: tf(1),
                hex: Hex::new(3, 0),
            },
        ),
    )
    .unwrap();
    // A player's default is ask (the leave to launch is withheld).
    assert_eq!(
        state(&world, A).pursue_read_up(intent(2)),
        (PursueLeave::Ask, None)
    );
    // Set on the root, the child reads it up the tree.
    player(&mut world, set_pursuit(1, Some(PursueLeave::Stay))).unwrap();
    assert_eq!(
        state(&world, A).pursue_read_up(intent(2)),
        (PursueLeave::Stay, Some(intent(1)))
    );
    // Set on the child, the nearest wins.
    staff_free_set(&mut world, 2, PursueLeave::Go);
    assert_eq!(
        state(&world, A).pursue_read_up(intent(2)),
        (PursueLeave::Go, Some(intent(2)))
    );
    assert_eq!(
        state(&world, A).pursue_read_up(intent(1)),
        (PursueLeave::Stay, Some(intent(1)))
    );
    // A side given the leave to launch (as one with no commander is) has go.
    let mut open = state(&world, A).clone();
    open.pursue_leaves.clear();
    open.leaves.launch = true;
    assert_eq!(open.pursue_read_up(intent(2)), (PursueLeave::Go, None));
}

/// Sets the pursue leave on an intent directly, for a child the commander
/// may not set by command.
fn staff_free_set(world: &mut World, intent_n: u32, leave: PursueLeave) {
    world
        .command
        .get_mut(&A)
        .unwrap()
        .pursue_leaves
        .insert(intent(intent_n), leave);
}

#[test]
fn a_yes_sets_a_pursuit_by_answer_naming_its_event_and_clears_a_refusal() {
    let mut world = world();
    player(&mut world, answer(2, 7, false)).unwrap();
    assert!(state(&world, A).pursuit_refusals.contains(&(tf(2), 7)));
    player(&mut world, answer(2, 7, true)).unwrap();
    let taken = last_answer_event(&world);
    let pursuit = state(&world, A).pursuits[&tf(2)];
    assert_eq!(pursuit.by, PursuitBy::Answer(taken));
    assert_eq!(pursuit.contact, 7);
    assert_eq!(pursuit.since_day, 1);
    assert!(
        state(&world, A).pursuit_refusals.is_empty(),
        "a yes clears the refusal of that contact"
    );
    assert_eq!(
        state(&world, A).leaves_by_answer().collect::<Vec<_>>(),
        vec![tf(2)]
    );
    // A refusal of another contact stays.
    player(&mut world, answer(2, 8, false)).unwrap();
    assert!(state(&world, A).pursuit_refusals.contains(&(tf(2), 8)));
}

#[test]
fn a_no_records_a_refusal_and_ends_a_pursuit_under_way() {
    let mut world = world();
    player(&mut world, answer(2, 7, true)).unwrap();
    assert!(state(&world, A).pursuits.contains_key(&tf(2)));
    player(&mut world, answer(2, 7, false)).unwrap();
    assert!(state(&world, A).pursuits.is_empty());
    assert!(state(&world, A).pursuit_refusals.contains(&(tf(2), 7)));
    assert_eq!(state(&world, A).leaves_by_answer().count(), 0);
}

#[test]
fn a_pursues_notice_inserts_a_taken_pursuit_and_a_pursuit_ended_notice_removes_it() {
    let mut world = world();
    world.submit_staff(A, pursuit_hand_in(NoticeKind::Pursues, 2, 4));
    world.resolve_day().unwrap();
    let pursuit = state(&world, A).pursuits[&tf(2)];
    assert_eq!(pursuit.by, PursuitBy::Taken);
    assert_eq!(pursuit.contact, 4);
    assert_eq!(pursuit.since_day, 1);
    assert_eq!(state(&world, A).leaves_by_answer().count(), 0);
    // The notice's pursuit is on its event.
    assert!(world.events.iter().any(|e| matches!(
        &e.kind,
        EventKind::StaffNotice { kind: NoticeKind::Pursues, pursuit: Some(p), .. }
            if p.force == tf(2) && p.find.is_some_and(|f| f.contact == 4)
    )));
    // A second "pursues" for a force that has an entry changes nothing.
    world.submit_staff(A, pursuit_hand_in(NoticeKind::Pursues, 2, 5));
    world.resolve_day().unwrap();
    assert_eq!(state(&world, A).pursuits[&tf(2)].contact, 4);
    world.submit_staff(A, pursuit_hand_in(NoticeKind::PursuitEnded, 2, 0));
    world.resolve_day().unwrap();
    assert!(state(&world, A).pursuits.is_empty());
    // And none of it fires the interrupt.
    assert_eq!(all_firings(&world, A), 0);
}

#[test]
fn an_asking_notice_fires_the_standing_interrupt_for_the_force_and_changes_nothing_else() {
    let mut world = world();
    world.submit_staff(A, pursuit_hand_in(NoticeKind::AsksLeaveToPursue, 2, 3));
    world.resolve_day().unwrap();
    assert_eq!(firings(&world, A, 1), vec![standing_asks(2)]);
    assert!(state(&world, A).pursuits.is_empty());
    // It cites the notice's event.
    let log = log(&world);
    let notice = log
        .iter()
        .find(|e| {
            matches!(
                e.kind,
                EventKind::StaffNotice {
                    kind: NoticeKind::AsksLeaveToPursue,
                    ..
                }
            )
        })
        .expect("a notice")
        .id;
    let fired = log
        .iter()
        .find(|e| matches!(e.kind, EventKind::InterruptFired { .. }))
        .expect("a firing");
    assert_eq!(fired.causes, vec![Cause::Event(notice)]);
    // Not again while it holds.
    world.submit_staff(A, pursuit_hand_in(NoticeKind::AsksLeaveToPursue, 2, 3));
    world.resolve_day().unwrap();
    assert_eq!(all_firings(&world, A), 1);
    // The enemy has none.
    assert_eq!(all_firings(&world, J), 0);
}

#[test]
fn an_asking_notice_does_not_fire_when_the_kind_is_switched_off() {
    let mut world = world();
    player(&mut world, switch(StandingKind::AsksLeaveToPursue, false)).unwrap();
    world.submit_staff(A, pursuit_hand_in(NoticeKind::AsksLeaveToPursue, 2, 3));
    world.resolve_day().unwrap();
    assert_eq!(all_firings(&world, A), 0);
    assert!(state(&world, A).standing.is_empty());
}

#[test]
fn a_stray_notice_fires_nothing_for_the_asking_interrupt() {
    // Another kind of notice carrying a pursuit, or an asking with none,
    // does not fire it.
    let mut world = world();
    world.submit_staff(A, pursuit_hand_in(NoticeKind::Pursues, 2, 3));
    let mut bare = pursuit_hand_in(NoticeKind::AsksLeaveToPursue, 2, 3);
    bare.notices[0].pursuit = None;
    world.submit_staff(A, bare);
    world.resolve_day().unwrap();
    assert_eq!(all_firings(&world, A), 0);
}

#[test]
fn the_new_command_state_fields_each_change_the_hash() {
    let base_world = world();
    let hash = base_world.state_hash();
    let changed = |edit: &dyn Fn(&mut CommandState)| {
        let mut world = base_world.clone();
        edit(world.command.get_mut(&A).unwrap());
        world.state_hash()
    };
    let leave_ask = changed(&|s| {
        s.pursue_leaves.insert(intent(1), PursueLeave::Ask);
    });
    let leave_go = changed(&|s| {
        s.pursue_leaves.insert(intent(1), PursueLeave::Go);
    });
    let taken = changed(&|s| {
        s.pursuits.insert(
            tf(2),
            Pursuit {
                by: PursuitBy::Taken,
                contact: 3,
                since_day: 1,
            },
        );
    });
    let other_contact = changed(&|s| {
        s.pursuits.insert(
            tf(2),
            Pursuit {
                by: PursuitBy::Taken,
                contact: 4,
                since_day: 1,
            },
        );
    });
    let other_day = changed(&|s| {
        s.pursuits.insert(
            tf(2),
            Pursuit {
                by: PursuitBy::Taken,
                contact: 3,
                since_day: 2,
            },
        );
    });
    let refusal = changed(&|s| {
        s.pursuit_refusals.insert((tf(2), 3));
    });
    let other_refusal = changed(&|s| {
        s.pursuit_refusals.insert((tf(2), 4));
    });
    let all = [
        hash,
        leave_ask,
        leave_go,
        taken,
        other_contact,
        other_day,
        refusal,
        other_refusal,
    ];
    for (i, a) in all.iter().enumerate() {
        for b in &all[i + 1..] {
            assert_ne!(a, b, "two states of the new fields hash alike");
        }
    }
    // A pursuit by answer hashes through its event.
    let mut by_answer = base_world.clone();
    player(&mut by_answer, answer(2, 3, true)).unwrap();
    assert_ne!(by_answer.state_hash(), hash);
    assert!(!by_answer.command[&A].is_empty());
}

#[test]
fn a_reasons_cites_become_causes_of_its_order_and_its_reason() {
    let mut world = world();
    player(&mut world, answer(2, 3, true)).unwrap();
    let answered = last_answer_event(&world);
    let mut cited = reason(1, "a pursuit by leave");
    cited.answer_of = Some(tf(2));
    let decisions = StaffDecisions {
        orders: vec![StaffOrder {
            kind: OrderKind::TaskForce {
                target: tf(2),
                order: TaskForceOrder::SetSpeed(SpeedSetting::Cruise),
            },
            intent: intent(1),
            reason: cited,
        }],
        ..StaffDecisions::default()
    };
    player(&mut world, set_intent(1)).unwrap();
    player(&mut world, set_forces(1, forces(&[], &[2]))).unwrap();
    world.submit_staff(A, decisions);
    world.resolve_day().unwrap();
    let log = log(&world);
    let reason_event = log
        .iter()
        .find(|e| matches!(&e.kind, EventKind::StaffReason { reason, .. } if reason.worked == "a pursuit by leave"))
        .expect("the reason is recorded");
    assert!(reason_event.causes.contains(&Cause::Event(answered)));
    let order_event = log
        .iter()
        .find(|e| {
            matches!(
                e.kind,
                EventKind::OrderAccepted { .. } | EventKind::OrderRejected { .. }
            )
        })
        .expect("the order is answered");
    assert!(order_event.causes.contains(&Cause::Event(answered)));
}

// ---------------------------------------------------------------------------
// Decision 0218, stage 3: the patrol order, a hold on a boat and a forces
// limit naming boats, and the override of a direct order to a boat.
// ---------------------------------------------------------------------------

fn sub(n: u32) -> crate::ids::SubmarineId {
    crate::ids::SubmarineId::new(n)
}

/// A boat of `side` at (5,0), sound, with base 1 or base 9 as her home.
fn boat_of_side(n: u32, side: Side) -> crate::state::Submarine {
    crate::state::Submarine {
        id: sub(n),
        name: format!("Boat {n}"),
        side,
        class: crate::ids::SubmarineClassId::new(1),
        hex: Hex::new(5, 0),
        station: Hex::new(5, 0),
        passage: None,
        salvoes: 2,
        held_down_until: None,
        condition: crate::state::SubmarineCondition::Sound,
        home: if side == A { base(1) } else { base(9) },
        leave_day: None,
        enters_day: None,
        available_from: None,
        station_order: None,
    }
}

const PATROL_HEX: Hex = Hex::new(5, 0);

fn patrol_kind() -> IntentKind {
    IntentKind::Patrol {
        hex: PATROL_HEX,
        distance: 3,
    }
}

fn set_patrol(n: u32) -> CommandKind {
    CommandKind::SetIntent {
        intent: intent(n),
        kind: patrol_kind(),
    }
}

fn boat_limit(boats: &[u32]) -> ForcesLimit {
    ForcesLimit {
        submarines: boats.iter().map(|b| sub(*b)).collect(),
        source: None,
        ..forces(&[], &[])
    }
}

fn station_order(boat: u32) -> OrderKind {
    OrderKind::Submarine {
        target: sub(boat),
        order: crate::orders::SubmarineOrder::Station(Hex::new(6, 0)),
    }
}

/// The Allied boats 1 and 2 and the Japanese boat 3, in `world()`.
fn world_with_boats() -> World {
    let mut world = world();
    for (n, side) in [(1, A), (2, A), (3, J)] {
        world.truth.submarines.insert(sub(n), boat_of_side(n, side));
    }
    world
}

/// A world whose Allied side states a patrol as intent 1 and has approved
/// its plan twice, so it is released.
fn world_patrolling() -> World {
    let mut world = world_with_boats();
    player(&mut world, set_patrol(1)).unwrap();
    player(&mut world, CommandKind::Approve { plan: plan(1) }).unwrap();
    player(&mut world, CommandKind::Approve { plan: plan(1) }).unwrap();
    world
}

#[test]
fn a_patrol_is_a_root_the_commander_states_and_it_is_refused_off_the_map() {
    let mut world = world_with_boats();
    player(&mut world, set_patrol(1)).unwrap();
    let found = &state(&world, A).intents[&intent(1)];
    assert_eq!(found.kind, patrol_kind());
    assert_eq!(found.maker, Maker::Commander);
    assert_eq!(found.addressee, Addressee::Area);
    assert_eq!(found.parent, None);
    assert_eq!(patrol_kind().code(), 16);
    assert_eq!(patrol_kind().name(), "patrol");
    // Its figures may change and not its kind.
    player(
        &mut world,
        CommandKind::SetIntent {
            intent: intent(1),
            kind: IntentKind::Patrol {
                hex: Hex::new(6, 0),
                distance: 1,
            },
        },
    )
    .unwrap();
    refused(
        &mut world,
        Issuer::Player,
        CommandKind::SetIntent {
            intent: intent(1),
            kind: keep_supplied(1),
        },
        CommandRefusal::KindCannotChange(intent(1)),
    );
    // A hex off the map.
    let mut world = world_with_boats();
    let off = Hex::new(99, 99);
    refused(
        &mut world,
        Issuer::Player,
        CommandKind::SetIntent {
            intent: intent(1),
            kind: IntentKind::Patrol {
                hex: off,
                distance: 2,
            },
        },
        CommandRefusal::NotOnMap(off),
    );
}

#[test]
fn a_patrol_is_worded_with_its_hexes_and_the_staff_cannot_form_it() {
    assert_eq!(
        crate::command::patrol_words(Hex::new(5, 0), 3),
        "patrol the waters within 3 hexes of (5,0) with submarines"
    );
    assert_eq!(
        crate::command::patrol_words(Hex::new(5, 0), 1),
        "patrol the waters within 1 hex of (5,0) with submarines"
    );
    assert!(
        set_patrol(1).to_string().contains("with submarines"),
        "{}",
        set_patrol(1)
    );
    // The staff cannot form one under an intent in force with a released
    // plan: it stands under nothing.
    let mut world = staffed_world();
    refused(
        &mut world,
        Issuer::Staff,
        CommandKind::FormIntent {
            parent: intent(1),
            kind: patrol_kind(),
        },
        CommandRefusal::CannotStandUnder(intent(1)),
    );
    // Nothing stands under a patrol.
    let mut world = world_patrolling();
    refused(
        &mut world,
        Issuer::Staff,
        CommandKind::FormIntent {
            parent: intent(1),
            kind: keep_supplied(1),
        },
        CommandRefusal::CannotStandUnder(intent(1)),
    );
    refused(
        &mut world,
        Issuer::Staff,
        CommandKind::FormIntent {
            parent: intent(1),
            kind: patrol_kind(),
        },
        CommandRefusal::CannotStandUnder(intent(1)),
    );
}

#[test]
fn a_hold_may_hold_a_boat_and_a_release_frees_her() {
    let mut world = world_with_boats();
    let held = HoldTarget::Submarine(sub(1));
    assert_eq!(held.code(), 6);
    assert_eq!(held.to_string(), "submarine:1");
    player(&mut world, hold(held)).unwrap();
    assert_eq!(
        state(&world, A).holds[&held],
        Hold {
            day: 1,
            by: HoldMaker::Command
        }
    );
    refused(
        &mut world,
        Issuer::Player,
        hold(held),
        CommandRefusal::AlreadyHeld(held),
    );
    player(&mut world, release(held)).unwrap();
    assert!(state(&world, A).holds.is_empty());
    refused(
        &mut world,
        Issuer::Player,
        release(held),
        CommandRefusal::NotHeld(held),
    );
    // An enemy boat, one that does not exist, and a sunk one: one reason.
    let mut world = world_with_boats();
    world.truth.submarines.get_mut(&sub(2)).unwrap().condition =
        crate::state::SubmarineCondition::Sunk;
    for boat in [3, 99, 2] {
        refused(
            &mut world,
            Issuer::Player,
            hold(HoldTarget::Submarine(sub(boat))),
            CommandRefusal::NotYourSubmarine(sub(boat)),
        );
    }
}

#[test]
fn a_forces_limit_may_name_boats_and_refuses_one_not_the_sides() {
    let mut world = world_with_boats();
    player(&mut world, set_patrol(1)).unwrap();
    player(&mut world, set_forces(1, boat_limit(&[1]))).unwrap();
    let set = state(&world, A).intents[&intent(1)]
        .limits
        .forces
        .clone()
        .unwrap();
    assert_eq!(set.submarines, [sub(1)].into_iter().collect());
    // Not the side's (the enemy's, or none), or sunk: refused in boat order,
    // after the other kinds.
    world.truth.submarines.get_mut(&sub(2)).unwrap().condition =
        crate::state::SubmarineCondition::Sunk;
    for boat in [3, 99, 2] {
        refused(
            &mut world,
            Issuer::Player,
            set_forces(1, boat_limit(&[1, boat])),
            CommandRefusal::NotYourSubmarine(sub(boat)),
        );
    }
    // A land unit refusal comes before a boat's.
    let mut both = boat_limit(&[99]);
    both.land_units = [LandUnitId::new(77)].into_iter().collect();
    refused(
        &mut world,
        Issuer::Player,
        set_forces(1, both),
        CommandRefusal::NotYourLandUnit(LandUnitId::new(77)),
    );
}

#[test]
fn a_boat_in_a_forces_limit_is_inside_the_parents_limit_or_the_parent_names_none() {
    // The narrowing check is on an intent with a parent: a "take a place"
    // is the one root under which the staff may form tasks with forces, so
    // the check is read on the function itself.
    let mut world = world_patrolling();
    let parent = intent(1);
    // No limit naming a boat on the parent's chain: any boat of the side.
    assert!(world.command[&A].submarines_on_chain(parent).is_none());
    let any = boat_limit(&[1, 2]);
    assert!(world.forces_narrow(A, parent, parent, &any));
    // A limit naming boat 1 only: boat 2 is wider.
    player(&mut world, set_forces(1, boat_limit(&[1]))).unwrap();
    assert!(world.forces_narrow(A, parent, parent, &boat_limit(&[1])));
    assert!(!world.forces_narrow(A, parent, parent, &boat_limit(&[1, 2])));
    assert!(!world.forces_narrow(A, parent, parent, &boat_limit(&[2])));
}

#[test]
fn a_staffs_station_order_to_a_boat_is_inside_a_patrol_that_may_use_her() {
    // A patrol with no limit naming a boat may use every boat of the side.
    let mut world = world_patrolling();
    assert_eq!(staff_order_answer(&mut world, station_order(1)), None);
    assert_eq!(staff_order_answer(&mut world, station_order(2)), None);
    // Not the enemy's boat: it is not the side's.
    assert_eq!(
        staff_order_answer(&mut world, station_order(3)),
        Some(RejectReason::OutsideForcesLimit)
    );
    // A limit naming boat 1 binds: boat 2 is outside it.
    player(&mut world, set_forces(1, boat_limit(&[1]))).unwrap();
    assert_eq!(staff_order_answer(&mut world, station_order(1)), None);
    assert_eq!(
        staff_order_answer(&mut world, station_order(2)),
        Some(RejectReason::OutsideForcesLimit)
    );
    // A limit naming only ships or groups does not bind the boats.
    let mut world = world_patrolling();
    player(&mut world, set_forces(1, forces(&[5], &[1]))).unwrap();
    assert_eq!(staff_order_answer(&mut world, station_order(2)), None);
    // A held boat: the staff's order to her is refused as held.
    let mut world = world_patrolling();
    player(&mut world, hold(HoldTarget::Submarine(sub(1)))).unwrap();
    assert_eq!(
        staff_order_answer(&mut world, station_order(1)),
        Some(RejectReason::Held(HoldTarget::Submarine(sub(1))))
    );
    assert_eq!(staff_order_answer(&mut world, station_order(2)), None);
    // Released, she is free for the staff again.
    player(&mut world, release(HoldTarget::Submarine(sub(1)))).unwrap();
    assert_eq!(staff_order_answer(&mut world, station_order(1)), None);
}

#[test]
fn a_staffs_order_to_a_boat_under_any_other_intent_is_outside_the_forces_limit() {
    // Intent 1 is a "keep supplied", with a forces limit that names a boat
    // all the same: only a patrol may use a boat.
    let mut world = staffed_world();
    world.truth.submarines.insert(sub(1), boat_of_side(1, A));
    let mut limit = forces(&[5, 6], &[1]);
    limit.submarines = [sub(1)].into_iter().collect();
    player(&mut world, set_forces(1, limit)).unwrap();
    assert_eq!(
        staff_order_answer(&mut world, station_order(1)),
        Some(RejectReason::OutsideForcesLimit)
    );
}

#[test]
fn a_direct_order_to_a_boat_a_patrol_may_use_is_taken_in_at_once_and_holds_her() {
    let mut world = world_patrolling();
    world.submit_orders(A, vec![commander_order(station_order(1))]);
    let at_step = world.at_staff_step();
    let id = *at_step.orders.keys().next().expect("the order is numbered");
    assert!(at_step.pending[&A].is_empty());
    assert_eq!(
        state(&at_step, A).holds,
        BTreeMap::from([(
            HoldTarget::Submarine(sub(1)),
            Hold {
                day: 1,
                by: HoldMaker::Order(id)
            }
        )])
    );
    let accepted = events_of(&at_step, |k| matches!(k, EventKind::OrderAccepted { .. }));
    assert_eq!(accepted.len(), 1);
    let held = events_of(&at_step, held_by_order);
    assert_eq!(held.len(), 1);
    assert_eq!(
        held[0].kind,
        EventKind::HeldByOrder {
            side: A,
            target: HoldTarget::Submarine(sub(1)),
            order: id
        }
    );
    assert_eq!(
        held[0].causes,
        vec![Cause::Order(id), Cause::Event(accepted[0].id)]
    );
    // The original world is untouched.
    assert!(state(&world, A).holds.is_empty());
    // A boat already held gets no second hold.
    let mut world = world_patrolling();
    player(&mut world, hold(HoldTarget::Submarine(sub(1)))).unwrap();
    world.submit_orders(A, vec![commander_order(station_order(1))]);
    let at_step = world.at_staff_step();
    assert!(events_of(&at_step, held_by_order).is_empty());
    assert_eq!(
        state(&at_step, A).holds[&HoldTarget::Submarine(sub(1))].by,
        HoldMaker::Command
    );
}

#[test]
fn a_direct_order_to_a_boat_no_intent_names_or_may_use_holds_nothing() {
    // A side with no intent in force is untouched.
    let mut world = world_with_boats();
    world.submit_orders(A, vec![commander_order(station_order(1))]);
    let pending = world.pending.clone();
    let at_step = world.at_staff_step();
    assert_eq!(at_step.pending, pending);
    assert!(events_of(&at_step, held_by_order).is_empty());
    // A patrol whose limit names boat 1 does not hold boat 2: the order to
    // her stays pending for step 5.
    let mut world = world_patrolling();
    player(&mut world, set_forces(1, boat_limit(&[1]))).unwrap();
    world.submit_orders(A, vec![commander_order(station_order(2))]);
    let at_step = world.at_staff_step();
    assert!(state(&at_step, A).holds.is_empty());
    assert_eq!(at_step.pending[&A].len(), 1);
    // A "keep supplied" that names the boat in its forces limit holds her.
    let mut world = staffed_world();
    world.truth.submarines.insert(sub(1), boat_of_side(1, A));
    let mut limit = forces(&[5, 6], &[1]);
    limit.submarines = [sub(1)].into_iter().collect();
    player(&mut world, set_forces(1, limit)).unwrap();
    world.submit_orders(A, vec![commander_order(station_order(1))]);
    let at_step = world.at_staff_step();
    assert!(state(&at_step, A)
        .holds
        .contains_key(&HoldTarget::Submarine(sub(1))));
}

#[test]
fn a_forces_limit_with_no_boat_hashes_as_it_did_and_one_with_a_boat_does_not() {
    let base_world = world_with_boats();
    let plain = |boats: &[u32]| {
        let mut world = base_world.clone();
        player(&mut world, set_patrol(1)).unwrap();
        player(&mut world, set_forces(1, boat_limit(boats))).unwrap();
        world.state_hash()
    };
    // The bytes written for a limit with no boat are the ones the layout
    // wrote before: a hasher fed the limit's fields and nothing more.
    assert_eq!(plain(&[]), plain(&[]));
    assert_ne!(plain(&[]), plain(&[1]));
    assert_ne!(plain(&[1]), plain(&[2]));
    assert_ne!(plain(&[1]), plain(&[1, 2]));
    // A patrol hashes by its hex and its distance.
    let hash_of = |kind: IntentKind| {
        let mut world = base_world.clone();
        player(
            &mut world,
            CommandKind::SetIntent {
                intent: intent(1),
                kind,
            },
        )
        .unwrap();
        world.state_hash()
    };
    assert_ne!(
        hash_of(patrol_kind()),
        hash_of(IntentKind::Patrol {
            hex: PATROL_HEX,
            distance: 4
        })
    );
    assert_ne!(
        hash_of(patrol_kind()),
        hash_of(IntentKind::Patrol {
            hex: Hex::new(6, 0),
            distance: 3
        })
    );
}

#[test]
fn a_hold_on_a_boat_changes_the_hash() {
    let base_world = world_with_boats();
    let hash = base_world.state_hash();
    let mut world = base_world.clone();
    player(&mut world, hold(HoldTarget::Submarine(sub(1)))).unwrap();
    let held = world.state_hash();
    assert_ne!(held, hash);
    let mut other = base_world;
    player(&mut other, hold(HoldTarget::Submarine(sub(2)))).unwrap();
    assert_ne!(other.state_hash(), held);
}

// ---- Decision 0228, the architect's rulings on the build (statements 21 to 25)

/// A world with a hold of base 1 (intent 1) whose plan is released and a
/// "keep supplied" for base 1 the staff formed under it (intent 2); the side's
/// staff chooses its sources where `keyed`.
fn world_with_a_hold_and_its_supply(keyed: bool) -> World {
    let mut world = world();
    if keyed {
        world.command.get_mut(&A).unwrap().staff_chooses_sources = true;
    }
    player(
        &mut world,
        CommandKind::SetIntent {
            intent: intent(1),
            kind: IntentKind::Hold { base: base(1) },
        },
    )
    .unwrap();
    player(&mut world, CommandKind::Approve { plan: plan(1) }).unwrap();
    player(&mut world, CommandKind::Approve { plan: plan(1) }).unwrap();
    staff(
        &mut world,
        CommandKind::FormIntent {
            parent: intent(1),
            kind: keep_supplied(1),
        },
    )
    .unwrap();
    world
}

/// A limit naming source `source` and the task forces `force_list`.
fn sourced(source: u32, force_list: &[u32]) -> ForcesLimit {
    ForcesLimit {
        source: Some(base(source)),
        ..forces(&[], force_list)
    }
}

/// Statement 21: without the key, a limit naming a source and a task force on
/// a "keep supplied" under a hold with no limit is refused as wider.
#[test]
fn statement_21_without_the_key_a_supply_under_a_hold_names_nothing_the_hold_does_not() {
    let mut world = world_with_a_hold_and_its_supply(false);
    refused(
        &mut world,
        Issuer::Staff,
        set_forces(2, sourced(2, &[1])),
        CommandRefusal::WiderThanParent(intent(1)),
    );
}

/// Statement 22: with the key, the same limit is taken.
#[test]
fn statement_22_with_the_key_the_same_limit_is_taken() {
    let mut world = world_with_a_hold_and_its_supply(true);
    staff(&mut world, set_forces(2, sourced(2, &[1]))).unwrap();
    assert_eq!(
        state(&world, A).intents[&intent(2)].limits.forces,
        Some(sourced(2, &[1]))
    );
}

/// Statement 23: with the key, under a hold whose limit names source base 2, a
/// limit naming base 1 is refused and one naming base 2 and a task force is
/// taken.
#[test]
fn statement_23_a_source_the_commander_names_still_binds() {
    let mut world = world_with_a_hold_and_its_supply(true);
    player(&mut world, set_forces(1, sourced(2, &[]))).unwrap();
    refused(
        &mut world,
        Issuer::Staff,
        set_forces(2, sourced(1, &[])),
        CommandRefusal::WiderThanParent(intent(1)),
    );
    staff(&mut world, set_forces(2, sourced(2, &[1]))).unwrap();
}

/// Statement 24: with the key, a limit on a cover formed under the same hold
/// is checked as today.
#[test]
fn statement_24_a_cover_under_the_hold_is_checked_as_today() {
    let mut world = world_with_a_hold_and_its_supply(true);
    staff(
        &mut world,
        CommandKind::FormIntent {
            parent: intent(1),
            kind: IntentKind::Cover { base: base(1) },
        },
    )
    .unwrap();
    // Intent 3 is the cover: a task force the hold's limits do not name is
    // wider than its parent.
    refused(
        &mut world,
        Issuer::Staff,
        set_forces(3, sourced(1, &[1])),
        CommandRefusal::WiderThanParent(intent(1)),
    );
}

/// Statement 25: with the key, a run of a hold's plan is checked against the
/// limit of the supply under the hold; without the key, with no limit on the
/// hold, it is refused as having no forces limit.
#[test]
fn statement_25_a_run_of_a_holds_plan_draws_on_the_supply_under_it() {
    let add = |ship_list: &[u32]| CommandKind::AddRun {
        plan: plan(1),
        run: run_spec(ship_list, [2, 3, 6, 8]),
    };
    let mut world = world_with_a_hold_and_its_supply(true);
    staff(&mut world, set_forces(2, sourced(1, &[1]))).unwrap();
    staff(&mut world, add(&[1, 2])).unwrap();
    refused(
        &mut world,
        Issuer::Staff,
        add(&[3]),
        CommandRefusal::NotInPool(ship(3)),
    );
    let mut world = world_with_a_hold_and_its_supply(false);
    refused(
        &mut world,
        Issuer::Staff,
        add(&[1, 2]),
        CommandRefusal::NoForcesLimit,
    );
}

// ---------------------------------------------------------------------------
// Decision 0220: an order against an enemy force.
// ---------------------------------------------------------------------------

/// Gives the Allied picture a contact of `kind` at `hex`, `age` pulses old,
/// by a sighting; returns its number.
fn hold_contact(world: &mut World, hex: Hex, kind: ShipKind, age: u32) -> u32 {
    use crate::belief::{ApplyOutcome, Report, SightingReport};
    let figures = crate::refdata::SearchFigures::from_search_toml();
    let outcome = world.beliefs.get_mut(&A).unwrap().apply_report(
        &Report::Sighting(SightingReport {
            estimated_hex: hex,
            estimated_ship_count: 2,
            estimated_kind: kind,
            received_event: crate::ids::EventId::new(1),
            age_pulses: age,
        }),
        &figures,
    );
    match outcome {
        ApplyOutcome::Contact(found) => found.contact.raw(),
        other => panic!("a sighting made no contact: {other:?}"),
    }
}

fn destroy(n: u32, contact: u32) -> CommandKind {
    CommandKind::SetIntent {
        intent: intent(n),
        kind: IntentKind::DestroyForce {
            contact,
            distance: 2,
            search_only: false,
        },
    }
}

#[allow(clippy::too_many_arguments)]
fn follow_notice(
    kind: NoticeKind,
    contact: u32,
    taken_from: Option<u32>,
    other_sort: bool,
    hex: Hex,
    day: u32,
    pulse: Pulse,
    carrier: bool,
) -> StaffDecisions {
    StaffDecisions {
        no_orders: vec![reason(1, "a force followed")],
        notices: vec![StaffNotice {
            kind,
            plan: plan(1),
            run: None,
            rests_on: NoticeBasis::NoOrder(0),
            pursuit: None,
            follow: Some(FollowNotice {
                intent: intent(1),
                contact,
                taken_from,
                other_sort,
                hex,
                day,
                pulse,
                ship_count: 2,
                kind: if carrier {
                    ShipKind::Carrier
                } else {
                    ShipKind::Cruiser
                },
                carrier,
            }),
        }],
        ..StaffDecisions::default()
    }
}

/// Takes in a hand-in of the staff at once, as the day's step 5 does.
fn hand_in(world: &mut World, decisions: StaffDecisions) {
    let day = world.truth.day + 1;
    world.intake_staff_commands(day, &BTreeMap::from([(A, decisions)]));
}

#[test]
fn the_order_to_destroy_a_force_has_its_code_and_name_and_is_the_areas_root() {
    let kind = IntentKind::DestroyForce {
        contact: 3,
        distance: 2,
        search_only: true,
    };
    assert_eq!(kind.code(), 17);
    assert_eq!(kind.name(), "destroy_force");
    assert_eq!(kind.followed(), Some((3, 2, true)));
    assert_eq!(keep_supplied(1).followed(), None);
    assert_eq!(kind.addressee(), Addressee::Area);
    assert!(kind.may_be_root());
    assert_eq!(kind.waters(), None);
    assert_eq!(kind.base(), None);
    assert_eq!(kind.own_base(), None);
    // The strike, search and carrier tasks stand under it; a stand task does
    // not (decision 0220, 2).
    let under = |task: IntentKind| task.may_stand_under(kind);
    assert!(under(IntentKind::StrikeShips { base: base(1) }));
    assert!(under(IntentKind::SearchWaters { base: base(1) }));
    assert!(under(IntentKind::OperateCarriers {
        force: tf(1),
        hex: HOME
    }));
    assert!(!under(IntentKind::StandAt {
        force: tf(1),
        hex: HOME
    }));
    assert_eq!(NoticeKind::Follows.code(), 13);
    assert_eq!(NoticeKind::ContactLost.code(), 14);
    assert_eq!(NoticeKind::ForceNotFound.code(), 15);
    for kind in [
        NoticeKind::Follows,
        NoticeKind::ContactLost,
        NoticeKind::ForceNotFound,
    ] {
        assert_eq!(NoticeKind::from_name(kind.name()), Some(kind));
    }
    assert_eq!(NoticeKind::Follows.name(), "follows");
    assert_eq!(NoticeKind::ContactLost.name(), "contact_lost");
    assert_eq!(NoticeKind::ForceNotFound.name(), "force_not_found");
}

#[test]
fn the_pulse_of_a_report_is_read_from_its_age() {
    // At dusk of day 3 the next pulse is the night of day 3.
    assert_eq!(
        report_pulse(3, Pulse::Night, 4),
        (1, Pulse::Morning),
        "age 4 at dusk of day 3"
    );
    assert_eq!(report_pulse(3, Pulse::Night, 1), (2, Pulse::Morning));
    assert_eq!(report_pulse(3, Pulse::Night, 0), (2, Pulse::Afternoon));
    // A contact the side starts with, at age 0 at dusk of day 1.
    assert_eq!(report_pulse(1, Pulse::Night, 0), (0, Pulse::Afternoon));
    // Inside day 1 after its night: the next pulse is the morning of day 1.
    assert_eq!(report_pulse(1, Pulse::Morning, 1), (0, Pulse::Afternoon));
    assert_eq!(report_pulse(1, Pulse::Morning, 0), (1, Pulse::Night));
    // Before the beginning it reads as the night of day 0.
    assert_eq!(report_pulse(0, Pulse::Night, 9), (0, Pulse::Night));
}

#[test]
fn the_distance_between_two_reports_is_counted_in_whole_hexes() {
    // At 20 knots a hex of 40 nautical miles is 2 hours.
    let hexes = |a: (u32, Pulse), b: (u32, Pulse)| report_distance_hexes(a.0, a.1, b.0, b.1, 20);
    // The morning of day 1 to the morning of day 2: 30 hours.
    assert_eq!(
        hexes((1, Pulse::Morning), (2, Pulse::Morning)),
        15,
        "30 hours"
    );
    // The afternoon of day 0 to the morning of day 1: 24 hours.
    assert_eq!(hexes((0, Pulse::Afternoon), (1, Pulse::Morning)), 12);
    // The afternoon of day 0 to the morning of day 2: 48 hours.
    assert_eq!(hexes((0, Pulse::Afternoon), (2, Pulse::Morning)), 24);
    // The afternoon of day 1 to the afternoon of day 2: 30 hours.
    assert_eq!(hexes((1, Pulse::Afternoon), (2, Pulse::Afternoon)), 15);
    // Cut to whole hexes: 7 hours at 20 knots is 3.5 hexes.
    assert_eq!(
        report_distance_hexes(1, Pulse::Morning, 1, Pulse::Afternoon, 20),
        6
    );
    assert_eq!(
        report_distance_hexes(1, Pulse::Morning, 1, Pulse::Afternoon, 7),
        2
    );
}

#[test]
fn an_order_to_destroy_a_force_keeps_the_report_it_follows_at_the_giving() {
    let mut world = world();
    let cruisers = hold_contact(&mut world, Hex::new(8, 0), ShipKind::Cruiser, 0);
    let carrier = hold_contact(&mut world, Hex::new(14, 1), ShipKind::Carrier, 3);
    assert_eq!((cruisers, carrier), (1, 2));
    player(&mut world, destroy(1, cruisers)).unwrap();
    // Taken in on day 1, before its night, a contact of age 0 was reported
    // in the afternoon of day 0.
    assert_eq!(
        state(&world, A).followed[&intent(1)],
        Followed {
            contact: 1,
            hex: Hex::new(8, 0),
            day: 0,
            pulse: Pulse::Afternoon,
            carrier: false,
            lost: false,
        }
    );
    // A carrier 3 pulses old: 4 pulses before the night of day 1, the night
    // of day 0.
    player(&mut world, destroy(2, carrier)).unwrap();
    assert_eq!(
        state(&world, A).followed[&intent(2)],
        Followed {
            contact: 2,
            hex: Hex::new(14, 1),
            day: 0,
            pulse: Pulse::Night,
            carrier: true,
            lost: false,
        }
    );
    // Setting the same order again with another distance keeps what it kept.
    let kept = state(&world, A).followed[&intent(1)];
    player(
        &mut world,
        CommandKind::SetIntent {
            intent: intent(1),
            kind: IntentKind::DestroyForce {
                contact: 1,
                distance: 4,
                search_only: false,
            },
        },
    )
    .unwrap();
    assert_eq!(state(&world, A).followed[&intent(1)], kept);
    // The commander's picture is the side's own belief here, and the contact
    // named is read from it: another contact, another report.
    player(
        &mut world,
        CommandKind::SetIntent {
            intent: intent(1),
            kind: IntentKind::DestroyForce {
                contact: 2,
                distance: 4,
                search_only: false,
            },
        },
    )
    .unwrap();
    assert_eq!(state(&world, A).followed[&intent(1)].contact, 2);
    assert!(state(&world, A).followed[&intent(1)].carrier);
}

#[test]
fn an_order_to_destroy_a_force_names_a_contact_held() {
    let mut world = world();
    hold_contact(&mut world, Hex::new(8, 0), ShipKind::Cruiser, 0);
    // A number not held, and none, are refused alike as not held.
    refused(
        &mut world,
        Issuer::Player,
        destroy(1, 7),
        CommandRefusal::ContactNotHeld(7),
    );
    refused(
        &mut world,
        Issuer::Player,
        destroy(1, 0),
        CommandRefusal::ContactNotHeld(0),
    );
    assert_eq!(
        CommandRefusal::ContactNotHeld(7).to_string(),
        "contact 7 is not held: the commander's picture holds no contact of that number"
    );
    assert_eq!(
        CommandRefusal::ContactNotHeld(0).to_string(),
        "no contact is named"
    );
    // The enemy's picture does not hold the Allied contact.
    assert_eq!(
        give(&mut world, J, Issuer::Player, destroy(1, 1)),
        Err(CommandRefusal::ContactNotHeld(1))
    );
    assert!(state(&world, A).followed.is_empty());
    assert!(state(&world, A).intents.is_empty());
    // A contact held and old may be named.
    let old = hold_contact(&mut world, Hex::new(2, 0), ShipKind::Destroyer, 4);
    player(&mut world, destroy(1, old)).unwrap();
    assert_eq!(state(&world, A).followed[&intent(1)].contact, old);
    assert_eq!(state(&world, A).followed[&intent(1)].day, 0);
    assert_eq!(state(&world, A).followed[&intent(1)].pulse, Pulse::Night);
}

#[test]
fn a_destroy_force_order_with_no_leave_set_reads_go_and_search_only_reads_stay() {
    let mut world = world();
    let found = hold_contact(&mut world, Hex::new(8, 0), ShipKind::Cruiser, 0);
    player(&mut world, destroy(1, found)).unwrap();
    player(
        &mut world,
        CommandKind::SetIntent {
            intent: intent(2),
            kind: IntentKind::DestroyForce {
                contact: found,
                distance: 2,
                search_only: true,
            },
        },
    )
    .unwrap();
    // The leave to launch is withheld, so any other order would ask.
    assert_eq!(
        state(&world, A).pursue_read_up(intent(1)),
        (PursueLeave::Go, Some(intent(1)))
    );
    assert_eq!(
        state(&world, A).pursue_read_up(intent(2)),
        (PursueLeave::Stay, Some(intent(2)))
    );
    // One set on the order wins.
    player(&mut world, set_pursuit(1, Some(PursueLeave::Ask))).unwrap();
    assert_eq!(
        state(&world, A).pursue_read_up(intent(1)),
        (PursueLeave::Ask, Some(intent(1)))
    );
}

#[test]
fn a_follows_notice_keeps_the_report_and_a_taking_changes_the_sort_and_clears_lost() {
    let mut world = world();
    let cruisers = hold_contact(&mut world, Hex::new(8, 0), ShipKind::Cruiser, 0);
    player(&mut world, destroy(1, cruisers)).unwrap();
    world
        .command
        .get_mut(&A)
        .unwrap()
        .followed
        .get_mut(&intent(1))
        .unwrap()
        .lost = true;
    // A refresh of the same contact: the report moves, the sort and lost do
    // not.
    hand_in(
        &mut world,
        follow_notice(
            NoticeKind::Follows,
            1,
            None,
            false,
            Hex::new(14, 0),
            1,
            Pulse::Morning,
            true,
        ),
    );
    let kept = state(&world, A).followed[&intent(1)];
    assert_eq!(
        kept,
        Followed {
            contact: 1,
            hex: Hex::new(14, 0),
            day: 1,
            pulse: Pulse::Morning,
            carrier: false,
            lost: true,
        }
    );
    // A taking of another contact: the number, the report and the sort
    // change and the order is no longer lost.
    hand_in(
        &mut world,
        follow_notice(
            NoticeKind::Follows,
            3,
            Some(1),
            true,
            Hex::new(20, 0),
            2,
            Pulse::Afternoon,
            true,
        ),
    );
    assert_eq!(
        state(&world, A).followed[&intent(1)],
        Followed {
            contact: 3,
            hex: Hex::new(20, 0),
            day: 2,
            pulse: Pulse::Afternoon,
            carrier: true,
            lost: false,
        }
    );
    // The notice is on the event, whole.
    assert!(world.events.iter().any(|e| matches!(
        &e.kind,
        EventKind::StaffNotice { kind: NoticeKind::Follows, follow: Some(f), .. }
            if f.taken_from == Some(1) && f.other_sort
    )));
}

#[test]
fn a_contact_lost_notice_sets_lost_and_a_force_not_found_notice_withdraws_the_tree() {
    let mut world = world();
    let cruisers = hold_contact(&mut world, Hex::new(8, 0), ShipKind::Cruiser, 0);
    player(&mut world, destroy(1, cruisers)).unwrap();
    player(&mut world, CommandKind::Approve { plan: plan(1) }).unwrap();
    player(&mut world, CommandKind::Approve { plan: plan(1) }).unwrap();
    // The staff forms a task under it.
    staff(
        &mut world,
        form_under(1, IntentKind::SearchWaters { base: base(1) }),
    )
    .unwrap();
    assert!(!state(&world, A).followed[&intent(1)].lost);
    hand_in(
        &mut world,
        follow_notice(
            NoticeKind::ContactLost,
            1,
            None,
            false,
            Hex::new(8, 0),
            0,
            Pulse::Afternoon,
            false,
        ),
    );
    assert!(state(&world, A).followed[&intent(1)].lost);
    assert!(state(&world, A).intents[&intent(1)].in_force());
    // Not found: the order and what is beneath it are withdrawn, each by an
    // event caused by the notice's, and nothing of it is kept.
    hand_in(
        &mut world,
        follow_notice(
            NoticeKind::ForceNotFound,
            1,
            None,
            false,
            Hex::new(8, 0),
            0,
            Pulse::Afternoon,
            false,
        ),
    );
    assert!(state(&world, A).followed.is_empty());
    assert!(!state(&world, A).intents[&intent(1)].in_force());
    assert!(!state(&world, A).intents[&intent(2)].in_force());
    assert_eq!(state(&world, A).plans[&plan(1)].state, PlanState::Complete);
    let log = log(&world);
    let notice = log
        .iter()
        .rev()
        .find(|e| {
            matches!(
                e.kind,
                EventKind::StaffNotice {
                    kind: NoticeKind::ForceNotFound,
                    ..
                }
            )
        })
        .expect("the notice")
        .id;
    // The order itself ends with its plan, as a commander's withdrawal ends
    // it; what is beneath it has an event of its own.
    let withdrawn = log
        .iter()
        .find(|e| {
            matches!(&e.kind, EventKind::IntentWithdrawn { intent, .. } if *intent == self::intent(2))
        })
        .expect("a withdrawal");
    assert!(
        withdrawn.causes.contains(&Cause::Event(notice)),
        "the task beneath is withdrawn on the notice's cause"
    );
    let ended = log.iter().find(|e| {
        matches!(e.kind, EventKind::PlanStateChanged { .. })
            && e.causes.contains(&Cause::Event(notice))
    });
    assert!(ended.is_some(), "the plan's end cites the notice");
    // A second notice for an order gone changes nothing and does not fail.
    let events = world.events.len();
    hand_in(
        &mut world,
        follow_notice(
            NoticeKind::ForceNotFound,
            1,
            None,
            false,
            Hex::new(8, 0),
            0,
            Pulse::Afternoon,
            false,
        ),
    );
    assert_eq!(world.events.len(), events + 2, "its reason and itself");
}

#[test]
fn a_commanders_withdrawal_removes_what_the_order_kept_and_the_staff_may_not_withdraw_it() {
    let mut world = world();
    let cruisers = hold_contact(&mut world, Hex::new(8, 0), ShipKind::Cruiser, 0);
    player(&mut world, destroy(1, cruisers)).unwrap();
    refused(
        &mut world,
        Issuer::Staff,
        CommandKind::WithdrawIntent { intent: intent(1) },
        CommandRefusal::NotYours,
    );
    assert!(state(&world, A).followed.contains_key(&intent(1)));
    player(
        &mut world,
        CommandKind::WithdrawIntent { intent: intent(1) },
    )
    .unwrap();
    assert!(state(&world, A).followed.is_empty());
    // An order the staff may not form: it is a root only.
    refused(
        &mut world,
        Issuer::Staff,
        form_under(
            1,
            IntentKind::DestroyForce {
                contact: cruisers,
                distance: 1,
                search_only: false,
            },
        ),
        CommandRefusal::NoSuchIntent(intent(1)),
    );
}

#[test]
fn what_the_order_keeps_is_in_the_hash_and_the_command_state_is_not_empty_with_it() {
    let mut world = world();
    let cruisers = hold_contact(&mut world, Hex::new(8, 0), ShipKind::Cruiser, 0);
    player(&mut world, destroy(1, cruisers)).unwrap();
    // The contacts made are no longer waiting to be aged.
    world
        .beliefs
        .get_mut(&A)
        .unwrap()
        .age_contacts(&crate::refdata::SearchFigures::from_search_toml());
    assert!(!state(&world, A).is_empty());
    let mut other = world.clone();
    assert_eq!(world.state_hash(), other.state_hash());
    // Two worlds that differ only in the pulse of the report kept.
    other
        .command
        .get_mut(&A)
        .unwrap()
        .followed
        .get_mut(&intent(1))
        .unwrap()
        .pulse = Pulse::Morning;
    assert_ne!(world.state_hash(), other.state_hash());
    // And in each other kept field.
    for change in 0..5 {
        let mut other = world.clone();
        let kept = other
            .command
            .get_mut(&A)
            .unwrap()
            .followed
            .get_mut(&intent(1))
            .unwrap();
        match change {
            0 => kept.contact += 1,
            1 => kept.hex = Hex::new(9, 0),
            2 => kept.day += 1,
            3 => kept.carrier = true,
            _ => kept.lost = true,
        }
        assert_ne!(world.state_hash(), other.state_hash(), "field {change}");
    }
    // The intent's own fields are hashed.
    let mut other = world.clone();
    other
        .command
        .get_mut(&A)
        .unwrap()
        .intents
        .get_mut(&intent(1))
        .unwrap()
        .kind = IntentKind::DestroyForce {
        contact: cruisers,
        distance: 2,
        search_only: true,
    };
    assert_ne!(world.state_hash(), other.state_hash());
    // The invariant holds, and fails for an entry with no order in force.
    assert!(
        crate::invariants::check_command(A, state(&world, A), &world.truth, world.truth.day)
            .is_empty()
    );
    other = world.clone();
    other.command.get_mut(&A).unwrap().followed.insert(
        intent(9),
        *world.command_state(A).followed.get(&intent(1)).unwrap(),
    );
    assert_eq!(
        crate::invariants::check_command(A, state(&other, A), &other.truth, other.truth.day),
        vec![crate::invariants::InvariantViolation::FollowedUnsound(
            A,
            intent(9)
        )]
    );
}
