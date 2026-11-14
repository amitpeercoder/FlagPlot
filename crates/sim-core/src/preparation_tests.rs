//! Tests of preparing a landing (decision 0095), the experience of a land
//! unit (decision 0097) and the activity of the day (decision 4 of 0099),
//! on hand-built worlds. The figures are the live ones: share 0.5, kept 0.5,
//! landing fatigue 40, strength 0.25, per day 5, 30 days of training, a
//! ceiling of 2 and 10 days of fighting.

use std::collections::{BTreeMap, BTreeSet};

use crate::events::{
    Event, EventKind, ExperienceSource, LandFatigueChange, LoadingLoss, RehearsalLoss, RosterLeft,
};
use crate::fixed::Fixed;
use crate::hex::Hex;
use crate::ids::{
    AirGroupId, BaseId, EventId, LandUnitId, OrderId, ShipClassId, ShipId, TaskForceId,
};
use crate::invariants::{self, InvariantViolation};
use crate::map::{HexData, HexKind, Map};
use crate::orders::{
    Activity, LandUnitOrder, Mission, OrderKind, OrderRequest, Origin, Posture, RejectReason,
    TaskForceOrder,
};
use crate::preparation::{joined, landing_fatigue_gain, landing_rate_factor, ship_joined};
use crate::refdata::{GroundFigures, PortSize, RefData, ShipClass, ShipKind};
use crate::state::{
    Base, Halt, HaltWork, LandUnit, Preparation, Ship, ShipLocation, Side, TaskForce,
    TaskForceStatus, Troops, TruthState, YardTrip,
};
use crate::time::{pulse_start, Pulse};
use crate::world::{SimConfig, World};

const A: Hex = Hex::new(0, 0);
const B: Hex = Hex::new(4, 0);
const C: Hex = Hex::new(3, 1);
/// Open sea, where no unit can be told to land.
const SEA: Hex = Hex::new(2, 0);
/// Where a fight is held.
const F: Hex = Hex::new(2, 2);
const G: Hex = Hex::new(6, 2);

fn fx(text: &str) -> Fixed {
    text.parse().expect("a decimal")
}

fn n(whole: i64) -> Fixed {
    Fixed::from_int(whole)
}

fn tf(id: u32) -> TaskForceId {
    TaskForceId::new(id)
}

fn sh(id: u32) -> ShipId {
    ShipId::new(id)
}

fn lu(id: u32) -> LandUnitId {
    LandUnitId::new(id)
}

fn live() -> GroundFigures {
    GroundFigures::FIRST
}

/// The frozen step 1 values of the switches: the rules are off.
fn frozen() -> GroundFigures {
    GroundFigures {
        unprepared_landing_share: Fixed::ONE,
        unprepared_landing_fatigue: Fixed::ZERO,
        experience_fire_strength: Fixed::ZERO,
        ..GroundFigures::FIRST
    }
}

fn roster(units: &[u32]) -> BTreeSet<LandUnitId> {
    units.iter().map(|unit| lu(*unit)).collect()
}

fn prepare(hex: Hex, units: &[u32]) -> TaskForceOrder {
    TaskForceOrder::PrepareLanding {
        hex,
        roster: roster(units),
    }
}

fn tf_request(target: u32, order: TaskForceOrder) -> OrderRequest {
    OrderRequest {
        kind: OrderKind::TaskForce {
            target: tf(target),
            order,
        },
        origin: Origin::PLAYER,
    }
}

fn unit_request(target: u32, order: LandUnitOrder) -> OrderRequest {
    OrderRequest {
        kind: OrderKind::LandUnit {
            target: lu(target),
            order,
        },
        origin: Origin::PLAYER,
    }
}

/// The world. Open sea, with coast hexes A (base 1, 1,000 t), B and C, F and
/// G; port size 0 everywhere, so every ship works by lighter at 20 t an
/// hour. Allied forces 1 to 5 at A, each of one troop transport (600 men,
/// 10 kn, 1,000 t of fuel) with its unit aboard: unit n in ship n, 600 men,
/// and unit 5 300. Ship 1 carries 60 t of supply. Unit 6, 300 men, is ashore
/// at A; unit 7 is destroyed; unit 9 is Japanese, 100 men ashore at G, with
/// Japanese base 9 there.
fn world_with(figures: GroundFigures, tweak: impl FnOnce(&mut TruthState)) -> World {
    let mut map = Map::new();
    for q in -1..=9 {
        for r in -1..=3 {
            map.insert(Hex::new(q, r), HexData::open_sea());
        }
    }
    for hex in [A, B, C, F, G] {
        map.insert(
            hex,
            HexData {
                kind: HexKind::Coast,
                ..HexData::open_sea()
            },
        );
    }
    let class = ShipClass {
        id: ShipClassId::new(1),
        name: "Troop transport".to_string(),
        kind: ShipKind::Transport,
        cruise_speed_kn: n(10),
        max_speed_kn: n(16),
        fuel_capacity_t: n(1000),
        endurance_nm: n(8000),
        cargo_capacity_t: n(3000),
        main_guns: 0,
        gun_rounds_per_min: Fixed::ZERO,
        gun_rounds_per_gun: 0,
        gun_damage: Fixed::ZERO,
        torpedo_tubes: 0,
        torpedo_tubes_one_side: 0,
        torpedo_damage: Fixed::ZERO,
        toughness: n(100),
        radar: false,
        aircraft_capacity: 0,
        anti_aircraft: 0,
        troop_capacity_men: 600,
    };
    let mut ref_data = RefData::default();
    ref_data.ship_classes.insert(class.id, class);
    ref_data.port_sizes.insert(
        0,
        PortSize {
            berths: 0,
            berth_rate_t_per_h: Fixed::ZERO,
            lighter_rate_t_per_h: n(20),
        },
    );
    ref_data.ground = Some(figures);
    ref_data.search = Some(crate::refdata::SearchFigures::from_search_toml());

    let mut truth = TruthState::default();
    truth.bases.insert(
        BaseId::new(1),
        Base {
            supply_t: n(1000),
            ..Base::new(BaseId::new(1), "Base 1".to_string(), Side::Allied, A)
        },
    );
    truth.bases.insert(
        BaseId::new(9),
        Base {
            supply_t: n(100),
            ..Base::new(BaseId::new(9), "Base 9".to_string(), Side::Japanese, G)
        },
    );
    for id in 1..=5u32 {
        let mut force = TaskForce::at(
            tf(id),
            format!("Force {id}"),
            Side::Allied,
            A,
            Mission::Transport,
        );
        force.ships.insert(sh(id));
        truth.task_forces.insert(tf(id), force);
        let men = if id == 5 { 300 } else { 600 };
        truth.ships.insert(
            sh(id),
            Ship {
                fuel_t: n(1000),
                supply_t: if id == 1 { n(60) } else { Fixed::ZERO },
                troops: Some(Troops { unit: lu(id), men }),
                ..Ship::new(
                    sh(id),
                    format!("Ship {id}"),
                    ShipClassId::new(1),
                    Side::Allied,
                    ShipLocation::TaskForce(tf(id)),
                )
            },
        );
        truth.land_units.insert(
            lu(id),
            LandUnit::new(lu(id), format!("Unit {id}"), Side::Allied, 0, None),
        );
    }
    truth.land_units.insert(
        lu(6),
        LandUnit::new(lu(6), "Unit 6".to_string(), Side::Allied, 300, Some(A)),
    );
    truth.land_units.insert(
        lu(7),
        LandUnit::new(lu(7), "Unit 7".to_string(), Side::Allied, 0, None),
    );
    truth.land_units.insert(
        lu(9),
        LandUnit::new(lu(9), "Unit 9".to_string(), Side::Japanese, 100, Some(G)),
    );
    tweak(&mut truth);
    World::new(7, ref_data, map, truth).expect("a sound setup")
}

fn fresh() -> World {
    world_with(live(), |_| {})
}

fn kinds(world: &World) -> Vec<EventKind> {
    world
        .events()
        .iter()
        .map(|event| event.kind.clone())
        .collect()
}

/// Gives an order at the start of day 1 and returns its number and the
/// events recorded after it: the acceptance and the follow-ups.
fn give(world: &mut World, request: OrderRequest) -> (OrderId, Vec<EventKind>) {
    let start = world.events().len();
    let (id, _, accepted) = world.take_in_order(1, None, Side::Allied, request, Vec::new());
    assert!(accepted, "the order is accepted");
    let after = world.events().from_index(start);
    assert!(matches!(after[0].kind, EventKind::OrderAccepted { .. }));
    (
        id,
        after[1..].iter().map(|event| event.kind.clone()).collect(),
    )
}

fn force(world: &World, id: u32) -> &TaskForce {
    &world.truth.task_forces[&tf(id)]
}

fn unit(world: &World, id: u32) -> &LandUnit {
    &world.truth.land_units[&lu(id)]
}

fn prep_of(world: &World, id: u32) -> Preparation {
    force(world, id)
        .preparation
        .clone()
        .expect("the force has a preparation")
}

fn set_prep(world: &mut World, id: u32, hex: Hex, units: &[u32], loading: i64) {
    let order = world.take_in_order(
        1,
        None,
        Side::Allied,
        tf_request(id, prepare(hex, units)),
        Vec::new(),
    );
    assert!(order.2);
    world
        .truth
        .task_forces
        .get_mut(&tf(id))
        .expect("force")
        .preparation
        .as_mut()
        .expect("prep")
        .loading = n(loading);
}

fn set_rehearsal(world: &mut World, id: u32, hex: Hex, rehearsal: i64) {
    let unit = world.truth.land_units.get_mut(&lu(id)).expect("unit");
    unit.rehearsal = n(rehearsal);
    unit.rehearsal_hex = (rehearsal > 0).then_some(hex);
}

fn find<T>(world: &World, pick: impl Fn(&Event) -> Option<T>) -> Vec<T> {
    world.events().iter().filter_map(pick).collect()
}

fn factor(event: &Event, name: &str) -> Option<Fixed> {
    event
        .breakdown
        .as_ref()?
        .factors
        .iter()
        .find(|factor| factor.name == name)
        .map(|factor| factor.value)
}

fn assert_sound(world: &World) {
    assert_eq!(
        invariants::check(&world.truth, &world.map, &world.ref_data),
        Vec::new()
    );
}

// ---- The order's refusals, in order ----

#[test]
fn a_prepare_order_is_refused_in_the_order_the_decision_lists() {
    let mut world = fresh();
    let check =
        |world: &World, side, target, order| world.check_order(side, &tf_request(target, order));
    // The force is not the side's.
    assert_eq!(
        check(&world, Side::Japanese, 1, prepare(B, &[1])),
        Err(RejectReason::NotYourUnit)
    );
    // A roster unit that is not the side's or does not exist comes before the
    // hex.
    assert_eq!(
        check(&world, Side::Allied, 1, prepare(SEA, &[1, 9])),
        Err(RejectReason::NotYourLandUnit)
    );
    assert_eq!(
        check(&world, Side::Allied, 1, prepare(B, &[77])),
        Err(RejectReason::NotYourLandUnit)
    );
    // The hex: open sea, or off the map.
    assert_eq!(
        check(&world, Side::Allied, 1, prepare(SEA, &[1])),
        Err(RejectReason::NotACoastHex(SEA))
    );
    let off = Hex::new(50, 50);
    assert_eq!(
        check(&world, Side::Allied, 1, prepare(off, &[])),
        Err(RejectReason::NotACoastHex(off))
    );
    // A destroyed unit comes after the hex.
    assert_eq!(
        check(&world, Side::Allied, 1, prepare(SEA, &[7])),
        Err(RejectReason::NotACoastHex(SEA))
    );
    assert_eq!(
        check(&world, Side::Allied, 1, prepare(B, &[1, 7])),
        Err(RejectReason::LandUnitDestroyed)
    );
    // A unit with men ashore and a unit with men aboard are both sound; an
    // empty roster is accepted.
    assert_eq!(check(&world, Side::Allied, 1, prepare(B, &[1, 6])), Ok(()));
    assert_eq!(check(&world, Side::Allied, 1, prepare(B, &[])), Ok(()));
    // At sea.
    let sea = world.truth.task_forces.get_mut(&tf(1)).expect("force");
    sea.status = TaskForceStatus::Underway;
    sea.toward_hex = Some(Hex::new(1, 0));
    sea.progress_nm = n(5);
    assert_eq!(check(&world, Side::Allied, 1, prepare(B, &[1])), Ok(()));
    // A force with no ship afloat has nothing to order, before anything
    // else.
    world
        .truth
        .task_forces
        .get_mut(&tf(3))
        .expect("force")
        .ships
        .clear();
    assert_eq!(
        check(&world, Side::Allied, 3, prepare(SEA, &[9])),
        Err(RejectReason::NoShipAfloat)
    );
    assert_eq!(
        RejectReason::NotACoastHex(B).to_string(),
        format!("{B} is not a coast hex on the map")
    );
}

#[test]
fn an_activity_order_is_for_a_unit_aboard_too_and_a_posture_still_needs_a_man_ashore() {
    let world = fresh();
    let check = |target, order| world.check_order(Side::Allied, &unit_request(target, order));
    assert_eq!(
        check(1, LandUnitOrder::SetActivity(Activity::Train)),
        Ok(())
    );
    assert_eq!(
        check(6, LandUnitOrder::SetActivity(Activity::Ready)),
        Ok(())
    );
    assert_eq!(
        check(7, LandUnitOrder::SetActivity(Activity::Train)),
        Err(RejectReason::LandUnitDestroyed)
    );
    assert_eq!(
        check(9, LandUnitOrder::SetActivity(Activity::Train)),
        Err(RejectReason::NotYourLandUnit)
    );
    assert_eq!(
        check(77, LandUnitOrder::SetActivity(Activity::Train)),
        Err(RejectReason::NotYourLandUnit)
    );
    // The checks of a posture and a march are as they were.
    assert_eq!(
        check(1, LandUnitOrder::SetPosture(Posture::Attack)),
        Err(RejectReason::NoMenAshore)
    );
    assert_eq!(
        check(1, LandUnitOrder::March(Hex::new(1, 0))),
        Err(RejectReason::NoMenAshore)
    );
    assert_eq!(
        check(7, LandUnitOrder::SetPosture(Posture::Attack)),
        Err(RejectReason::LandUnitDestroyed)
    );
    assert_eq!(check(6, LandUnitOrder::SetPosture(Posture::Attack)), Ok(()));
}

#[test]
fn an_activity_order_sets_the_activity_and_its_order_and_nothing_else() {
    let mut world = fresh();
    let before = unit(&world, 6).clone();
    let (id, follow_ups) = {
        let start = world.events().len();
        let (id, _, accepted) = world.take_in_order(
            1,
            None,
            Side::Allied,
            unit_request(6, LandUnitOrder::SetActivity(Activity::Train)),
            Vec::new(),
        );
        assert!(accepted);
        (id, world.events().len() - start)
    };
    assert_eq!(follow_ups, 1, "only the acceptance");
    let after = unit(&world, 6);
    assert_eq!(after.activity, Activity::Train);
    assert_eq!(after.activity_order, Some(id));
    assert_eq!(
        LandUnit {
            activity: Activity::Ready,
            activity_order: None,
            ..after.clone()
        },
        before
    );
}

// ---- The order's application ----

#[test]
fn a_first_order_stands_with_no_loading_and_the_same_order_again_changes_nothing() {
    let mut world = fresh();
    let (first, events) = give(&mut world, tf_request(1, prepare(B, &[1, 6])));
    assert_eq!(events, Vec::new());
    let prep = prep_of(&world, 1);
    assert_eq!(
        (prep.hex, prep.roster.clone(), prep.loading, prep.order),
        (B, roster(&[1, 6]), Fixed::ZERO, first)
    );
    world
        .truth
        .task_forces
        .get_mut(&tf(1))
        .expect("force")
        .preparation
        .as_mut()
        .expect("prep")
        .loading = n(25);
    let (_, events) = give(&mut world, tf_request(1, prepare(B, &[6, 1])));
    assert_eq!(events, Vec::new(), "the same order records nothing");
    let prep = prep_of(&world, 1);
    assert_eq!(prep.loading, n(25));
    assert_eq!(prep.order, first, "and keeps the old order's number");
}

#[test]
fn a_new_landing_place_loses_the_loading_and_the_rehearsal_of_every_unit_of_the_roster() {
    let mut world = fresh();
    give(&mut world, tf_request(4, prepare(B, &[4, 6])));
    set_prep(&mut world, 4, B, &[4, 6], 25);
    set_rehearsal(&mut world, 4, B, 25);
    set_rehearsal(&mut world, 6, B, 10);
    // A unit that has rehearsed another hex keeps that.
    let (order, events) = give(&mut world, tf_request(4, prepare(C, &[4])));
    assert_eq!(
        events,
        vec![
            EventKind::LoadingLost {
                task_force: tf(4),
                hex: B,
                loading_before: n(25),
                loading: Fixed::ZERO,
                reason: LoadingLoss::NewLandingPlace,
            },
            EventKind::RehearsalLost {
                unit: lu(4),
                hex: B,
                rehearsal_before: n(25),
                reason: RehearsalLoss::NewLandingPlace,
            },
            EventKind::RehearsalLost {
                unit: lu(6),
                hex: B,
                rehearsal_before: n(10),
                reason: RehearsalLoss::NewLandingPlace,
            },
            EventKind::LeftRoster {
                unit: lu(6),
                task_force: tf(4),
                reason: RosterLeft::DroppedByOrder,
            },
        ]
    );
    let prep = prep_of(&world, 4);
    assert_eq!(
        (prep.hex, prep.roster, prep.loading, prep.order),
        (C, roster(&[4]), Fixed::ZERO, order)
    );
    for id in [4, 6] {
        assert_eq!(unit(&world, id).rehearsal, Fixed::ZERO);
        assert_eq!(unit(&world, id).rehearsal_hex, None);
    }
    // Every follow-up cites the order.
    let cited = find(&world, |event| {
        matches!(event.kind, EventKind::LoadingLost { .. }).then(|| event.causes.clone())
    });
    assert_eq!(cited, vec![vec![crate::events::Cause::Order(order)]]);
}

#[test]
fn a_rehearsal_for_another_hex_is_not_lost_by_a_new_landing_place() {
    let mut world = fresh();
    give(&mut world, tf_request(4, prepare(B, &[4])));
    set_rehearsal(&mut world, 4, C, 40);
    let (_, events) = give(&mut world, tf_request(4, prepare(G, &[4])));
    assert_eq!(events, Vec::new(), "no loading, and a rehearsal for C kept");
    assert_eq!(unit(&world, 4).rehearsal, n(40));
    assert_eq!(unit(&world, 4).rehearsal_hex, Some(C));
}

#[test]
fn units_added_keep_the_share_of_the_loading_once_however_many_and_a_unit_dropped_costs_nothing() {
    let mut world = fresh();
    give(&mut world, tf_request(1, prepare(B, &[1, 6])));
    set_prep(&mut world, 1, B, &[1, 6], 25);
    set_rehearsal(&mut world, 6, B, 10);
    // Two units added: the loading is cut once.
    let (_, events) = give(&mut world, tf_request(1, prepare(B, &[1, 2, 3, 6])));
    assert_eq!(
        events,
        vec![EventKind::LoadingLost {
            task_force: tf(1),
            hex: B,
            loading_before: n(25),
            loading: fx("12.5"),
            reason: LoadingLoss::UnitAdded,
        }]
    );
    let last = world.events().iter().last().expect("an event");
    assert_eq!(factor(last, "loading_before"), Some(n(25)));
    assert_eq!(factor(last, "preparation_kept_on_change"), Some(fx("0.5")));
    assert_eq!(
        last.breakdown.as_ref().expect("breakdown").result,
        fx("12.5")
    );
    assert_eq!(prep_of(&world, 1).loading, fx("12.5"));
    // A unit left out costs nothing and keeps its rehearsal.
    let (_, events) = give(&mut world, tf_request(1, prepare(B, &[1, 2, 3])));
    assert_eq!(
        events,
        vec![EventKind::LeftRoster {
            unit: lu(6),
            task_force: tf(1),
            reason: RosterLeft::DroppedByOrder,
        }]
    );
    assert_eq!(prep_of(&world, 1).loading, fx("12.5"));
    assert_eq!(unit(&world, 6).rehearsal, n(10));
    // With no loading there is nothing to cut and nothing recorded.
    give(&mut world, tf_request(4, prepare(B, &[4])));
    let (_, events) = give(&mut world, tf_request(4, prepare(B, &[4, 5])));
    assert_eq!(events, Vec::new());
}

#[test]
fn a_unit_taken_from_another_roster_costs_that_force_nothing() {
    let mut world = fresh();
    give(&mut world, tf_request(4, prepare(B, &[4])));
    set_prep(&mut world, 4, B, &[4], 25);
    set_prep(&mut world, 5, B, &[5], 25);
    let (_, events) = give(&mut world, tf_request(5, prepare(B, &[4, 5])));
    assert_eq!(
        events,
        vec![
            EventKind::LeftRoster {
                unit: lu(4),
                task_force: tf(4),
                reason: RosterLeft::TakenByOrder,
            },
            EventKind::LoadingLost {
                task_force: tf(5),
                hex: B,
                loading_before: n(25),
                loading: fx("12.5"),
                reason: LoadingLoss::UnitAdded,
            },
        ]
    );
    let old = prep_of(&world, 4);
    assert_eq!(old.roster, BTreeSet::new());
    assert_eq!(
        old.loading,
        n(25),
        "the force it was taken from keeps its loading"
    );
    assert_eq!(prep_of(&world, 5).roster, roster(&[4, 5]));
    assert_sound(&world);
}

// ---- The leaving of a unit from its roster is in the invariants ----

#[test]
fn the_invariants_catch_an_unsound_preparation_or_experience() {
    let mut world = fresh();
    set_prep(&mut world, 1, B, &[1], 40);
    assert_sound(&world);
    let check = |world: &World| invariants::check(&world.truth, &world.map, &world.ref_data);

    let mut broken = world.clone();
    broken
        .truth
        .task_forces
        .get_mut(&tf(1))
        .expect("force")
        .preparation
        .as_mut()
        .expect("prep")
        .loading = n(101);
    assert_eq!(
        check(&broken),
        vec![InvariantViolation::PreparationUnsound(tf(1))]
    );

    let mut broken = world.clone();
    broken
        .truth
        .task_forces
        .get_mut(&tf(1))
        .expect("force")
        .preparation
        .as_mut()
        .expect("prep")
        .roster
        .insert(lu(9));
    assert_eq!(
        check(&broken),
        vec![InvariantViolation::PreparationUnsound(tf(1))]
    );

    let mut broken = world.clone();
    set_prep(&mut broken, 2, B, &[2], 0);
    broken
        .truth
        .task_forces
        .get_mut(&tf(2))
        .expect("force")
        .preparation
        .as_mut()
        .expect("prep")
        .roster
        .insert(lu(1));
    assert_eq!(
        check(&broken),
        vec![InvariantViolation::UnitOnTwoRosters(lu(1))]
    );

    let mut broken = world.clone();
    broken
        .truth
        .land_units
        .get_mut(&lu(6))
        .expect("unit")
        .rehearsal = n(5);
    assert_eq!(
        check(&broken),
        vec![InvariantViolation::LandUnitExperienceUnsound(lu(6))],
        "a rehearsal with no hex"
    );
    let mut broken = world.clone();
    broken
        .truth
        .land_units
        .get_mut(&lu(6))
        .expect("unit")
        .rehearsal_hex = Some(B);
    assert_eq!(
        check(&broken),
        vec![InvariantViolation::LandUnitExperienceUnsound(lu(6))],
        "a hex with no rehearsal"
    );
    let mut broken = world.clone();
    set_rehearsal(&mut broken, 6, B, 5);
    broken
        .truth
        .land_units
        .get_mut(&lu(6))
        .expect("unit")
        .rehearsal = n(101);
    assert_eq!(
        check(&broken),
        vec![InvariantViolation::LandUnitExperienceUnsound(lu(6))]
    );
    for (experience, training, fighting) in [(5, 0, 0), (2, 30, 0), (2, 0, 10)] {
        let mut broken = world.clone();
        let land_unit = broken.truth.land_units.get_mut(&lu(6)).expect("unit");
        land_unit.experience = experience;
        land_unit.training_days = training;
        land_unit.fight_days = fighting;
        assert_eq!(
            check(&broken),
            vec![InvariantViolation::LandUnitExperienceUnsound(lu(6))]
        );
    }
}

// ---- The helpers of the rate and the fatigue ----

#[test]
fn the_rate_factor_counts_loading_only_at_the_hex_of_the_order() {
    let figures = live();
    let prep = Preparation {
        hex: B,
        roster: BTreeSet::new(),
        loading: n(50),
        order: OrderId::new(1),
    };
    assert_eq!(
        landing_rate_factor(&figures, Some(&prep), B),
        (fx("0.75"), n(50))
    );
    assert_eq!(
        landing_rate_factor(&figures, Some(&prep), C),
        (fx("0.5"), Fixed::ZERO)
    );
    assert_eq!(
        landing_rate_factor(&figures, None, B),
        (fx("0.5"), Fixed::ZERO)
    );
    let full = Preparation {
        loading: n(100),
        ..prep.clone()
    };
    assert_eq!(landing_rate_factor(&figures, Some(&full), B).0, Fixed::ONE);
    // With the frozen share of 1 the factor is 1 whatever the loading.
    assert_eq!(landing_rate_factor(&frozen(), Some(&prep), B).0, Fixed::ONE);
    assert_eq!(landing_rate_factor(&frozen(), None, C).0, Fixed::ONE);
}

#[test]
fn the_landing_fatigue_counts_rehearsal_only_for_the_hex_it_is_for() {
    let figures = live();
    assert_eq!(
        landing_fatigue_gain(&figures, n(50), Some(B), B),
        (n(20), n(50))
    );
    assert_eq!(
        landing_fatigue_gain(&figures, n(50), Some(B), C),
        (n(40), Fixed::ZERO)
    );
    assert_eq!(
        landing_fatigue_gain(&figures, Fixed::ZERO, None, B),
        (n(40), Fixed::ZERO)
    );
    assert_eq!(
        landing_fatigue_gain(&figures, n(100), Some(B), B).0,
        Fixed::ZERO
    );
    assert_eq!(
        landing_fatigue_gain(&frozen(), Fixed::ZERO, None, B).0,
        Fixed::ZERO
    );
}

#[test]
fn the_view_gives_a_side_its_own_rate_fatigue_and_roster_and_never_the_enemys() {
    let mut world = fresh();
    set_prep(&mut world, 1, B, &[1], 50);
    set_rehearsal(&mut world, 1, B, 50);
    let allied = world.side_view(Side::Allied);
    assert_eq!(allied.landing_rate_factor(tf(1), B), Some(fx("0.75")));
    assert_eq!(allied.landing_rate_factor(tf(1), C), Some(fx("0.5")));
    assert_eq!(allied.landing_fatigue(lu(1), B), Some(n(20)));
    assert_eq!(allied.landing_fatigue(lu(1), C), Some(n(40)));
    assert_eq!(allied.roster_of(lu(1)), Some(tf(1)));
    assert_eq!(allied.roster_of(lu(6)), None);
    // None for what is not the side's own.
    assert_eq!(allied.landing_fatigue(lu(9), B), None);
    assert_eq!(allied.roster_of(lu(9)), None);
    let japanese = world.side_view(Side::Japanese);
    assert_eq!(japanese.landing_rate_factor(tf(1), B), None);
    assert_eq!(japanese.landing_fatigue(lu(1), B), None);
    assert_eq!(japanese.roster_of(lu(1)), None);
    assert_eq!(japanese.landing_fatigue(lu(9), B), Some(n(40)));
}

// ---- The day: loading, rehearsal and training ----

/// The day's step on day 3, with nothing marched, contested or fired.
fn step(world: &mut World) {
    let figures = world.ref_data.ground.expect("figures");
    world.day_activity(
        3,
        &BTreeMap::new(),
        &BTreeSet::new(),
        &BTreeMap::new(),
        &figures,
    );
}

fn loading_gained(world: &World) -> Vec<(TaskForceId, Fixed)> {
    find(world, |event| match event.kind {
        EventKind::LoadingGained {
            task_force,
            loading,
            ..
        } => Some((task_force, loading)),
        _ => None,
    })
}

#[test]
fn a_force_stopped_at_a_base_with_the_order_gains_loading_and_its_roster_rehearses() {
    let mut world = fresh();
    let (order, _) = give(&mut world, tf_request(1, prepare(B, &[1, 6])));
    // Unit 6 is ashore at A, in the force's hex; unit 1 is aboard its ship.
    step(&mut world);
    assert_eq!(prep_of(&world, 1).loading, n(5));
    assert_eq!(loading_gained(&world), vec![(tf(1), n(5))]);
    let event = world
        .events()
        .iter()
        .find(|event| matches!(event.kind, EventKind::LoadingGained { .. }))
        .expect("event");
    assert_eq!(event.causes, vec![crate::events::Cause::Order(order)]);
    assert_eq!(factor(event, "loading_before"), Some(Fixed::ZERO));
    assert_eq!(factor(event, "preparation_per_day"), Some(n(5)));
    assert_eq!(event.breakdown.as_ref().expect("breakdown").result, n(5));
    for id in [1, 6] {
        assert_eq!(unit(&world, id).rehearsal, n(5), "unit {id}");
        assert_eq!(unit(&world, id).rehearsal_hex, Some(B));
    }
    let gained = find(&world, |event| match &event.kind {
        EventKind::RehearsalGained {
            unit,
            task_force,
            hex,
            rehearsal,
        } => Some((*unit, *task_force, *hex, *rehearsal, event.causes.clone())),
        _ => None,
    });
    assert_eq!(
        gained,
        vec![
            (
                lu(1),
                tf(1),
                B,
                n(5),
                vec![crate::events::Cause::Order(order)]
            ),
            (
                lu(6),
                tf(1),
                B,
                n(5),
                vec![crate::events::Cause::Order(order)]
            ),
        ]
    );
    // Each gain is given once a day and stops at 100.
    for _ in 0..30 {
        step(&mut world);
    }
    assert_eq!(prep_of(&world, 1).loading, n(100));
    assert_eq!(unit(&world, 6).rehearsal, n(100));
    assert_eq!(loading_gained(&world).len(), 20, "none once it is 100");
    assert_sound(&world);
}

#[test]
fn a_force_that_does_not_qualify_gains_nothing() {
    // (what is changed in the world, why it does not qualify)
    type Change = fn(&mut World);
    let day_start = pulse_start(3, Pulse::Night);
    let cases: Vec<(&str, Change)> = vec![
        ("under way", |world| {
            let force = world.truth.task_forces.get_mut(&tf(1)).expect("force");
            force.status = TaskForceStatus::Underway;
            force.toward_hex = Some(Hex::new(1, 0));
        }),
        ("not stopped on a leg", |world| {
            let force = world.truth.task_forces.get_mut(&tf(1)).expect("force");
            force.toward_hex = Some(Hex::new(1, 0));
        }),
        ("out of fuel", |world| {
            world
                .truth
                .task_forces
                .get_mut(&tf(1))
                .expect("force")
                .status = TaskForceStatus::OutOfFuel;
        }),
        ("arrived today", |world| {
            let force = world.truth.task_forces.get_mut(&tf(1)).expect("force");
            force.status = TaskForceStatus::Arrived;
            force.arrived_at_hours = Some(pulse_start(3, Pulse::Night) + n(5));
        }),
        ("halted today", |world| {
            let force = world.truth.task_forces.get_mut(&tf(1)).expect("force");
            force.status = TaskForceStatus::Halted;
            force.halt = Some(Halt {
                work: HaltWork {
                    bombard: false,
                    cargo: true,
                },
                at_hours: pulse_start(3, Pulse::Night) + n(1),
                event: EventId::new(1),
                bombarded: None,
                cargo_completed: None,
            });
        }),
        ("no base of its side in its hex", |world| {
            world
                .truth
                .bases
                .get_mut(&BaseId::new(1))
                .expect("base")
                .side = Side::Japanese;
        }),
        ("an empty roster", |world| {
            world
                .truth
                .task_forces
                .get_mut(&tf(1))
                .expect("force")
                .preparation
                .as_mut()
                .expect("prep")
                .roster
                .clear();
        }),
        ("away at the yard", |world| {
            world
                .truth
                .task_forces
                .get_mut(&tf(1))
                .expect("force")
                .yard_trip = Some(YardTrip {
                base: BaseId::new(1),
                return_day: 9,
                departure: EventId::new(1),
            });
        }),
    ];
    let _ = day_start;
    for (why, change) in cases {
        let mut world = fresh();
        give(&mut world, tf_request(1, prepare(B, &[1, 6])));
        change(&mut world);
        step(&mut world);
        assert_eq!(prep_of(&world, 1).loading, Fixed::ZERO, "{why}");
        assert_eq!(loading_gained(&world), Vec::new(), "{why}");
        assert_eq!(unit(&world, 6).rehearsal, Fixed::ZERO, "{why}: rehearsal");
        assert_eq!(unit(&world, 1).rehearsal, Fixed::ZERO, "{why}: rehearsal");
    }
    // Arrived or halted before the day began, and idle, it qualifies.
    for change in [
        (|world: &mut World| {
            let force = world.truth.task_forces.get_mut(&tf(1)).expect("force");
            force.status = TaskForceStatus::Arrived;
            force.arrived_at_hours = Some(pulse_start(3, Pulse::Night) - n(1));
        }) as fn(&mut World),
        |world| {
            let force = world.truth.task_forces.get_mut(&tf(1)).expect("force");
            force.status = TaskForceStatus::Halted;
            force.halt = Some(Halt {
                work: HaltWork {
                    bombard: false,
                    cargo: true,
                },
                at_hours: pulse_start(3, Pulse::Night) - n(10),
                event: EventId::new(1),
                bombarded: None,
                cargo_completed: None,
            });
        },
        |_| {},
    ] {
        let mut world = fresh();
        give(&mut world, tf_request(1, prepare(B, &[1, 6])));
        change(&mut world);
        step(&mut world);
        assert_eq!(prep_of(&world, 1).loading, n(5));
    }
    assert!(day_start > Fixed::ZERO);
}

#[test]
fn a_force_with_an_empty_roster_keeps_the_loading_it_has_and_gains_none() {
    let mut world = fresh();
    set_prep(&mut world, 1, B, &[], 30);
    step(&mut world);
    assert_eq!(prep_of(&world, 1).loading, n(30));
    assert_eq!(loading_gained(&world), Vec::new());
}

#[test]
fn a_unit_rehearses_only_if_all_its_men_are_in_the_forces_hex_or_aboard_its_ships() {
    // Part of unit 6 aboard ship 1 and the rest ashore at A: it rehearses.
    let mut world = world_with(live(), |truth| {
        truth.ships.get_mut(&sh(1)).expect("ship").troops = Some(Troops {
            unit: lu(6),
            men: 100,
        });
        truth.land_units.get_mut(&lu(6)).expect("unit").men_ashore = 200;
    });
    give(&mut world, tf_request(1, prepare(B, &[6])));
    step(&mut world);
    assert_eq!(unit(&world, 6).rehearsal, n(5));

    // Men of the unit aboard a ship of another force: it does not.
    let mut world = world_with(live(), |truth| {
        truth.ships.get_mut(&sh(1)).expect("ship").troops = Some(Troops {
            unit: lu(6),
            men: 100,
        });
        truth.ships.get_mut(&sh(2)).expect("ship").troops = Some(Troops {
            unit: lu(6),
            men: 100,
        });
        truth.land_units.get_mut(&lu(6)).expect("unit").men_ashore = 100;
    });
    give(&mut world, tf_request(1, prepare(B, &[6])));
    step(&mut world);
    assert_eq!(unit(&world, 6).rehearsal, Fixed::ZERO);

    // Ashore elsewhere: it does not.
    let mut world = world_with(live(), |truth| {
        let land_unit = truth.land_units.get_mut(&lu(6)).expect("unit");
        land_unit.hex = Some(C);
    });
    give(&mut world, tf_request(1, prepare(B, &[6])));
    step(&mut world);
    assert_eq!(unit(&world, 6).rehearsal, Fixed::ZERO);
    assert_eq!(prep_of(&world, 1).loading, n(5), "the force still loads");

    // Aboard another force's ship only: it does not.
    let mut world = fresh();
    give(&mut world, tf_request(1, prepare(B, &[2])));
    step(&mut world);
    assert_eq!(unit(&world, 2).rehearsal, Fixed::ZERO);
}

#[test]
fn a_day_of_rehearsal_is_not_given_to_a_unit_that_trains_marched_or_was_in_a_contested_hex() {
    // Trains instead.
    let mut world = fresh();
    world
        .truth
        .land_units
        .get_mut(&lu(6))
        .expect("unit")
        .experience = 0;
    give(&mut world, tf_request(1, prepare(B, &[1, 6])));
    world.take_in_order(
        1,
        None,
        Side::Allied,
        unit_request(6, LandUnitOrder::SetActivity(Activity::Train)),
        Vec::new(),
    );
    step(&mut world);
    assert_eq!(unit(&world, 6).rehearsal, Fixed::ZERO);
    assert_eq!(unit(&world, 6).training_days, 1);
    assert_eq!(
        unit(&world, 1).rehearsal,
        n(5),
        "the unit left ready rehearses"
    );
    assert_eq!(
        prep_of(&world, 1).loading,
        n(5),
        "and its place on the roster costs no loading"
    );
    assert!(prep_of(&world, 1).roster.contains(&lu(6)));

    // Marched: none of the three.
    let figures = live();
    let mut world = fresh();
    give(&mut world, tf_request(1, prepare(B, &[1, 6])));
    let marched: BTreeMap<LandUnitId, EventId> = [(lu(6), EventId::new(1))].into_iter().collect();
    world.day_activity(3, &marched, &BTreeSet::new(), &BTreeMap::new(), &figures);
    assert_eq!(unit(&world, 6).rehearsal, Fixed::ZERO);
    assert_eq!(unit(&world, 1).rehearsal, n(5));

    // Its hex contested: a unit ashore there gains nothing; a unit aboard a
    // ship in the same hex is not in the fight.
    let mut world = fresh();
    give(&mut world, tf_request(1, prepare(B, &[1, 6])));
    let contested: BTreeSet<Hex> = [A].into_iter().collect();
    world.day_activity(3, &BTreeMap::new(), &contested, &BTreeMap::new(), &figures);
    assert_eq!(unit(&world, 6).rehearsal, Fixed::ZERO);
    assert_eq!(unit(&world, 1).rehearsal, n(5));

    // A unit set to train that cannot spend the day training does nothing
    // and does not rehearse.
    let mut world = fresh();
    give(&mut world, tf_request(1, prepare(B, &[6])));
    world
        .truth
        .land_units
        .get_mut(&lu(6))
        .expect("unit")
        .activity = Activity::Train;
    world
        .truth
        .land_units
        .get_mut(&lu(6))
        .expect("unit")
        .fed_share = fx("0.5");
    step(&mut world);
    assert_eq!(unit(&world, 6).rehearsal, Fixed::ZERO);
    assert_eq!(unit(&world, 6).training_days, 0);
}

#[test]
fn a_rehearsal_for_a_new_hex_replaces_an_old_one_and_stops_at_100() {
    let mut world = fresh();
    give(&mut world, tf_request(1, prepare(B, &[6])));
    set_rehearsal(&mut world, 6, C, 30);
    step(&mut world);
    assert_eq!(unit(&world, 6).rehearsal, n(5));
    assert_eq!(unit(&world, 6).rehearsal_hex, Some(B));
    let lost = find(&world, |event| match event.kind {
        EventKind::RehearsalLost {
            unit,
            hex,
            rehearsal_before,
            reason,
        } => Some((unit, hex, rehearsal_before, reason)),
        _ => None,
    });
    assert_eq!(
        lost,
        vec![(lu(6), C, n(30), RehearsalLoss::OtherLandingPlace)]
    );
    // At 100 it still spends the day, and records nothing.
    set_rehearsal(&mut world, 6, B, 100);
    let before = world.events().len();
    step(&mut world);
    assert_eq!(unit(&world, 6).rehearsal, n(100));
    assert!(world
        .events()
        .from_index(before)
        .iter()
        .all(|event| { !matches!(event.kind, EventKind::RehearsalGained { .. }) }));
}

// ---- Training ----

#[test]
fn a_unit_trains_to_the_ceiling_and_no_further() {
    let mut world = fresh();
    let (order, _) = {
        let (id, _, _) = world.take_in_order(
            1,
            None,
            Side::Allied,
            unit_request(6, LandUnitOrder::SetActivity(Activity::Train)),
            Vec::new(),
        );
        (id, ())
    };
    world
        .truth
        .land_units
        .get_mut(&lu(6))
        .expect("unit")
        .experience = 0;
    let figures = live();
    let mut table = Vec::new();
    for day in 1..=62 {
        world.fight_ashore(day);
        if [29, 30, 59, 60, 62].contains(&day) {
            let land_unit = unit(&world, 6);
            table.push((day, land_unit.training_days, land_unit.experience));
        }
    }
    assert_eq!(
        table,
        vec![(29, 29, 0), (30, 0, 1), (59, 29, 1), (60, 0, 2), (62, 0, 2)]
    );
    let days = find(&world, |event| match event.kind {
        EventKind::TrainingDay {
            unit: trained,
            days,
            experience,
        } => Some((trained, days, experience, event.id, event.causes.clone())),
        _ => None,
    });
    assert_eq!(
        days.len(),
        60,
        "days 61 and 62 count nothing and record nothing"
    );
    assert_eq!(days[0].1, 1);
    assert!(days[0].4.contains(&crate::events::Cause::Order(order)));
    assert_eq!((days[29].1, days[29].2), (30, 0), "the day that raises it");
    assert_eq!((days[30].1, days[30].2), (1, 1));
    let raised = find(&world, |event| match event.kind {
        EventKind::ExperienceRaised { experience, by, .. } => {
            Some((experience, by, event.causes.clone()))
        }
        _ => None,
    });
    assert_eq!(
        raised,
        vec![
            (
                1,
                ExperienceSource::Training,
                vec![crate::events::Cause::Event(days[29].3)]
            ),
            (
                2,
                ExperienceSource::Training,
                vec![crate::events::Cause::Event(days[59].3)]
            ),
        ]
    );
    let breakdown = world
        .events()
        .get(days[0].3)
        .and_then(|event| event.breakdown.clone())
        .expect("breakdown");
    assert_eq!(breakdown.result, n(1));
    assert!(breakdown
        .factors
        .iter()
        .any(|f| f.name == "days_before" && f.value == Fixed::ZERO));
    assert!(breakdown
        .factors
        .iter()
        .any(|f| f.name == "training_days_per_level" && f.value == n(30)));
    assert_eq!(figures.training_ceiling, 2);
    assert_sound(&world);
}

#[test]
fn a_unit_that_is_not_fed_or_not_at_a_base_or_not_all_ashore_does_not_train() {
    let train = |truth: &mut TruthState| {
        let land_unit = truth.land_units.get_mut(&lu(6)).expect("unit");
        land_unit.activity = Activity::Train;
        land_unit.experience = 0;
    };
    // Fed in full and at a base: trains.
    let mut world = world_with(live(), train);
    step(&mut world);
    assert_eq!(unit(&world, 6).training_days, 1);
    // Not fed in full.
    let mut world = world_with(live(), |truth| {
        train(truth);
        truth.land_units.get_mut(&lu(6)).expect("unit").fed_share = fx("0.5");
    });
    step(&mut world);
    assert_eq!(unit(&world, 6).training_days, 0);
    // No base of its side in its hex.
    let mut world = world_with(live(), |truth| {
        train(truth);
        truth.land_units.get_mut(&lu(6)).expect("unit").hex = Some(C);
    });
    step(&mut world);
    assert_eq!(unit(&world, 6).training_days, 0);
    // Some men aboard a ship.
    let mut world = world_with(live(), |truth| {
        train(truth);
        truth.ships.get_mut(&sh(1)).expect("ship").troops = Some(Troops {
            unit: lu(6),
            men: 100,
        });
    });
    step(&mut world);
    assert_eq!(unit(&world, 6).training_days, 0);
    // A unit left ready does not train.
    let mut world = fresh();
    step(&mut world);
    assert_eq!(unit(&world, 6).training_days, 0);
    // A unit that marched, or whose hex was contested, does not train.
    let figures = live();
    let mut world = world_with(live(), train);
    let marched: BTreeMap<LandUnitId, EventId> = [(lu(6), EventId::new(1))].into_iter().collect();
    world.day_activity(3, &marched, &BTreeSet::new(), &BTreeMap::new(), &figures);
    assert_eq!(unit(&world, 6).training_days, 0);
    let contested: BTreeSet<Hex> = [A].into_iter().collect();
    world.day_activity(3, &BTreeMap::new(), &contested, &BTreeMap::new(), &figures);
    assert_eq!(unit(&world, 6).training_days, 0);
}

#[test]
fn a_training_day_cites_that_nights_feeding_of_its_side_and_hex() {
    let mut world = world_with(live(), |truth| {
        let land_unit = truth.land_units.get_mut(&lu(6)).expect("unit");
        land_unit.activity = Activity::Train;
        land_unit.experience = 0;
    });
    let fed = world.events.record(
        3,
        Some(Pulse::Night),
        EventKind::MenFed {
            side: Side::Allied,
            hex: A,
            base: Some(BaseId::new(1)),
            men: 300,
            fed: 300,
            unfed: 0,
        },
        Vec::new(),
        None,
    );
    step(&mut world);
    let day = world
        .events()
        .iter()
        .find(|event| matches!(event.kind, EventKind::TrainingDay { .. }))
        .expect("a training day");
    assert!(day.causes.contains(&crate::events::Cause::Event(fed)));
}

// ---- Fighting: experience moves the chance, and days of fighting count ----

fn fight_world(figures: GroundFigures) -> World {
    world_with(figures, |truth| {
        // F: Allied unit 10 defends with 1,000 men, experience 2 and 9 days
        // of fighting counted; Japanese unit 11 attacks with 600, experience
        // 3. G: Allied unit 12, experience 0, attacks 500 men of Japanese
        // unit 13, experience 4, which defends. Unit 9 is theirs, too.
        let mut units = vec![
            (10, Side::Allied, 1000, F, Posture::Defend, 2),
            (11, Side::Japanese, 600, F, Posture::Attack, 3),
            (12, Side::Allied, 500, G, Posture::Attack, 0),
            (13, Side::Japanese, 500, G, Posture::Defend, 4),
        ];
        for (id, side, men, hex, posture, experience) in units.drain(..) {
            let mut land_unit = LandUnit::new(lu(id), format!("Unit {id}"), side, men, Some(hex));
            land_unit.posture = posture;
            land_unit.experience = experience;
            truth.land_units.insert(lu(id), land_unit);
        }
        truth.land_units.get_mut(&lu(10)).expect("unit").fight_days = 9;
        truth.land_units.remove(&lu(9));
        truth.bases.remove(&BaseId::new(9));
    })
}

fn fires(world: &World, id: u32) -> Vec<(Fixed, Fixed, u32, EventId)> {
    find(world, |event| match event.kind {
        EventKind::GroundFire {
            unit,
            chance,
            shots,
            firing_men,
            ..
        } if unit == lu(id) => Some((chance, shots, firing_men, event.id)),
        _ => None,
    })
}

#[test]
fn experience_moves_the_chance_a_shot_hits_and_the_breakdown_names_it() {
    let mut world = fight_world(live());
    world.fight_ashore(1);
    // F: experience 3 at 0.3 x 1.25, experience 2 at 0.6 x 1.
    let (chance, shots, firing, _) = fires(&world, 11)[0];
    assert_eq!((chance, shots, firing), (fx("0.375"), n(6), 600));
    let (chance, shots, firing, event) = fires(&world, 10)[0];
    assert_eq!((chance, shots, firing), (fx("0.6"), n(10), 1000));
    let event = world.events().get(event).expect("event");
    assert_eq!(factor(event, "side_chance"), Some(fx("0.6")));
    assert_eq!(factor(event, "experience"), Some(n(2)));
    assert_eq!(factor(event, "experience_factor"), Some(Fixed::ONE));
    assert_eq!(factor(event, "chance"), Some(fx("0.6")));
    let names: Vec<&str> = event
        .breakdown
        .as_ref()
        .expect("breakdown")
        .factors
        .iter()
        .map(|f| f.name)
        .collect();
    let at = |name| names.iter().position(|n| *n == name).expect("named");
    assert!(at("side_chance") < at("experience"));
    assert!(at("experience_factor") < at("chance"));
    // G: raw against the best.
    let (chance, shots, _, event) = fires(&world, 12)[0];
    assert_eq!((chance, shots), (fx("0.15"), n(5)));
    assert_eq!(
        factor(
            world.events().get(event).expect("event"),
            "experience_factor"
        ),
        Some(fx("0.5"))
    );
    let (chance, shots, _, event) = fires(&world, 13)[0];
    assert_eq!((chance, shots), (fx("0.9"), n(5)));
    assert_eq!(
        factor(
            world.events().get(event).expect("event"),
            "experience_factor"
        ),
        Some(fx("1.5"))
    );
}

#[test]
fn a_unit_one_day_short_of_a_step_gains_it_and_fires_stronger_the_next_day() {
    let mut world = fight_world(live());
    world.fight_ashore(1);
    // Unit 10 fired on its tenth day: experience 3 and the count starts
    // again. Unit 11 (on attack) has one day.
    assert_eq!(
        (unit(&world, 10).experience, unit(&world, 10).fight_days),
        (3, 0)
    );
    assert_eq!(
        (unit(&world, 11).experience, unit(&world, 11).fight_days),
        (3, 1)
    );
    let fire = fires(&world, 10)[0].3;
    let counted = find(&world, |event| match event.kind {
        EventKind::FightDayCounted {
            unit,
            days,
            experience,
        } if unit == lu(10) => Some((days, experience, event.id, event.causes.clone())),
        _ => None,
    });
    assert_eq!(counted.len(), 1);
    assert_eq!((counted[0].0, counted[0].1), (10, 2));
    assert_eq!(counted[0].3, vec![crate::events::Cause::Event(fire)]);
    let raised = find(&world, |event| match event.kind {
        EventKind::ExperienceRaised {
            unit,
            experience,
            by,
        } if unit == lu(10) => Some((experience, by, event.causes.clone())),
        _ => None,
    });
    assert_eq!(
        raised,
        vec![(
            3,
            ExperienceSource::Fighting,
            vec![crate::events::Cause::Event(counted[0].2)]
        )]
    );
    let breakdown = world
        .events()
        .get(counted[0].2)
        .and_then(|event| event.breakdown.clone())
        .expect("breakdown");
    assert_eq!(breakdown.result, n(10));
    // Day 2: 0.6 x 1.25.
    world.fight_ashore(2);
    assert_eq!(fires(&world, 10)[1].0, fx("0.75"));
    assert_sound(&world);
}

#[test]
fn a_unit_at_experience_4_counts_no_more_days_and_the_chance_is_cut_to_1() {
    let mut world = fight_world(live());
    world.fight_ashore(1);
    assert_eq!(unit(&world, 13).experience, 4);
    assert_eq!(unit(&world, 13).fight_days, 0);
    let counted = find(&world, |event| match event.kind {
        EventKind::FightDayCounted { unit, .. } if unit == lu(13) => Some(()),
        _ => None,
    });
    assert_eq!(counted, Vec::new());
    // The chance is cut to 0..=1.
    use crate::ground_combat::experience_chance;
    assert_eq!(
        experience_chance(fx("0.6"), fx("0.5"), 4),
        (Fixed::ONE, n(2))
    );
    assert_eq!(
        experience_chance(fx("0.6"), fx("0.5"), 0),
        (Fixed::ZERO, Fixed::ZERO)
    );
    assert_eq!(
        experience_chance(fx("0.3"), Fixed::ZERO, 4),
        (fx("0.3"), Fixed::ONE)
    );
}

#[test]
fn with_the_frozen_strength_the_chance_and_the_breakdown_are_as_before_and_fighting_is_counted() {
    let mut world = fight_world(frozen());
    world.fight_ashore(1);
    let (chance, _, _, event) = fires(&world, 11)[0];
    assert_eq!(chance, fx("0.3"));
    let event = world.events().get(event).expect("event");
    let names: Vec<&str> = event
        .breakdown
        .as_ref()
        .expect("breakdown")
        .factors
        .iter()
        .map(|f| f.name)
        .collect();
    assert_eq!(
        names,
        vec![
            "men_ashore",
            "fed_share",
            "fatigue_factor",
            "firing_men",
            "men_per_shot",
            "shots",
            "chance"
        ]
    );
    assert_eq!(fires(&world, 10)[0].0, fx("0.6"));
    // The days of fighting count whatever the strength.
    assert_eq!(unit(&world, 11).fight_days, 1);
}

#[test]
fn a_unit_that_does_not_fire_counts_no_day_of_fighting() {
    // A unit with too few men for one shot fires 0 shots.
    let mut world = world_with(live(), |truth| {
        for (id, side, men, posture) in [
            (10, Side::Allied, 50, Posture::Defend),
            (11, Side::Japanese, 600, Posture::Attack),
        ] {
            let mut land_unit = LandUnit::new(lu(id), format!("Unit {id}"), side, men, Some(F));
            land_unit.posture = posture;
            truth.land_units.insert(lu(id), land_unit);
        }
    });
    world.fight_ashore(1);
    // 50 men make half a shot: shots above 0, so it counts.
    assert_eq!(unit(&world, 10).fight_days, 1);
    let mut world = world_with(live(), |truth| {
        for (id, side, men, posture, fed) in [
            (10, Side::Allied, 1000, Posture::Defend, Fixed::ZERO),
            (11, Side::Japanese, 600, Posture::Attack, Fixed::ONE),
        ] {
            let mut land_unit = LandUnit::new(lu(id), format!("Unit {id}"), side, men, Some(F));
            land_unit.posture = posture;
            land_unit.fed_share = fed;
            truth.land_units.insert(lu(id), land_unit);
        }
    });
    world.fight_ashore(1);
    // Unfed: no firing men, no shots, no day.
    assert_eq!(unit(&world, 10).fight_days, 0);
    assert_eq!(unit(&world, 11).fight_days, 1);
}

// ---- The landing ----

fn landing_world(figures: GroundFigures) -> World {
    // Base 2 at B holds 100 t, as in the hand calculation.
    let mut world = world_with(figures, |truth| {
        truth.bases.insert(
            BaseId::new(2),
            Base {
                supply_t: n(100),
                ..Base::new(BaseId::new(2), "Base 2".to_string(), Side::Allied, B)
            },
        );
    });
    // Forces 1 and 3 prepare for B, 2 does not prepare at all; force 3 is
    // sent to C. Loading and rehearsal of 50 are set for B as the ten days of
    // the hand calculation would give.
    give(&mut world, tf_request(1, prepare(B, &[1])));
    give(&mut world, tf_request(3, prepare(B, &[3])));
    for id in [1, 3] {
        world
            .truth
            .task_forces
            .get_mut(&tf(id))
            .expect("force")
            .preparation
            .as_mut()
            .expect("prep")
            .loading = n(50);
        set_rehearsal(&mut world, id, B, 50);
    }
    world.submit_orders(
        Side::Allied,
        vec![
            tf_request(1, TaskForceOrder::route(vec![B])),
            tf_request(1, TaskForceOrder::Unload),
            tf_request(2, TaskForceOrder::route(vec![B])),
            tf_request(2, TaskForceOrder::Unload),
            tf_request(3, TaskForceOrder::route(vec![C])),
            tf_request(3, TaskForceOrder::Unload),
        ],
    );
    world
}

fn landed(world: &World, id: u32) -> Vec<(u32, Event)> {
    world
        .events()
        .iter()
        .filter_map(|event| match event.kind {
            EventKind::MenLanded { unit, men, .. } if unit == lu(id) => Some((men, event.clone())),
            _ => None,
        })
        .collect()
}

#[test]
fn a_prepared_force_lands_faster_and_its_units_land_less_tired() {
    let mut world = landing_world(live());
    world.resolve_day().expect("day 1");
    // Force 1: 20 x (0.5 + 0.5 x 50/100) = 15 t an hour for 2 hours.
    let first = landed(&world, 1);
    assert_eq!(first[0].0, 120);
    assert_eq!(factor(&first[0].1, "rate_t_per_h"), Some(n(15)));
    assert_eq!(factor(&first[0].1, "port_rate_t_per_h"), Some(n(20)));
    assert_eq!(
        factor(&first[0].1, "unprepared_landing_share"),
        Some(fx("0.5"))
    );
    assert_eq!(factor(&first[0].1, "loading"), Some(n(50)));
    assert_eq!(factor(&first[0].1, "loading_factor"), Some(fx("0.75")));
    assert_eq!(factor(&first[0].1, "capacity_by_time_t"), Some(n(30)));
    // Force 2 has no loading: 10 t an hour.
    let second = landed(&world, 2);
    assert_eq!(second[0].0, 80);
    assert_eq!(factor(&second[0].1, "rate_t_per_h"), Some(n(10)));
    assert_eq!(factor(&second[0].1, "loading"), Some(Fixed::ZERO));
    assert_eq!(factor(&second[0].1, "loading_factor"), Some(fx("0.5")));
    // Force 3 has 50 for B and lands at C: 10 t an hour.
    let third = landed(&world, 3);
    // (Its path is shorter, so it works more hours: the rate is what counts.)
    assert_eq!(factor(&third[0].1, "rate_t_per_h"), Some(n(10)));
    assert_eq!(factor(&third[0].1, "loading"), Some(Fixed::ZERO));
    // Day 1 afternoon: 90 t, 360 men, and 240 for force 2.
    assert_eq!(landed(&world, 1)[1].0, 360);
    assert_eq!(landed(&world, 2)[1].0, 240);
    // A force that has begun landing is marked as landing.
    for (id, hex) in [(1, B), (2, B), (3, C)] {
        assert_eq!(force(&world, id).landing_at, Some(hex), "force {id}");
    }
    // The fatigue on landing, then the day's rest of 5.
    assert_eq!(unit(&world, 1).fatigue, n(15));
    assert_eq!(unit(&world, 2).fatigue, n(35));
    assert_eq!(unit(&world, 3).fatigue, n(35));
    let fatigue: Vec<(LandUnitId, Fixed, Event)> = world
        .events()
        .iter()
        .filter_map(|event| match event.kind {
            EventKind::LandUnitFatigue {
                unit,
                change: LandFatigueChange::Landed,
                fatigue,
            } => Some((unit, fatigue, event.clone())),
            _ => None,
        })
        .collect();
    assert_eq!(
        fatigue.iter().map(|(u, f, _)| (*u, *f)).collect::<Vec<_>>(),
        vec![(lu(1), n(20)), (lu(2), n(40)), (lu(3), n(40))]
    );
    assert_eq!(factor(&fatigue[0].2, "fatigue_before"), Some(Fixed::ZERO));
    assert_eq!(
        factor(&fatigue[0].2, "unprepared_landing_fatigue"),
        Some(n(40))
    );
    assert_eq!(factor(&fatigue[0].2, "rehearsal"), Some(n(50)));
    assert_eq!(factor(&fatigue[2].2, "rehearsal"), Some(Fixed::ZERO));
    // The landing cited: the fatigue cites the unit's first MenLanded.
    let first_landing = world
        .events()
        .iter()
        .find(|event| matches!(event.kind, EventKind::MenLanded { unit, .. } if unit == lu(1)))
        .expect("landed");
    assert_eq!(
        fatigue[0].2.causes,
        vec![crate::events::Cause::Event(first_landing.id)]
    );
    // Rehearsal spent, and the unit left its roster.
    assert_eq!(unit(&world, 1).rehearsal, Fixed::ZERO);
    assert_eq!(unit(&world, 1).rehearsal_hex, None);
    assert_eq!(prep_of(&world, 1).roster, BTreeSet::new());
    let left = find(&world, |event| match event.kind {
        EventKind::LeftRoster {
            unit,
            task_force,
            reason,
        } => Some((unit, task_force, reason, event.causes.clone())),
        _ => None,
    });
    assert_eq!(left.len(), 2, "units 1 and 3; unit 2 was on no roster");
    assert_eq!(
        (left[0].0, left[0].1, left[0].2),
        (lu(1), tf(1), RosterLeft::Landed)
    );
    assert_eq!(
        left[0].3,
        vec![crate::events::Cause::Event(first_landing.id)]
    );
    assert_eq!(
        (left[1].0, left[1].1, left[1].2),
        (lu(3), tf(3), RosterLeft::Landed)
    );
    let lost = find(&world, |event| match event.kind {
        EventKind::RehearsalLost {
            unit,
            hex,
            rehearsal_before,
            reason,
        } => Some((unit, hex, rehearsal_before, reason)),
        _ => None,
    });
    assert_eq!(
        lost,
        vec![
            (lu(1), B, n(50), RehearsalLoss::Landed),
            (lu(3), B, n(50), RehearsalLoss::Landed),
        ]
    );
    assert_sound(&world);

    // Day 2: complete. Force 1: 120 men take 30 t and the 60 t of supply
    // follow at 15 t an hour: 90 t, 6 hours, at hour 30. Force 2: 280 men are
    // 70 t at 10 t an hour: hour 31. Force 3 likewise.
    world.resolve_day().expect("day 2");
    let complete = find(&world, |event| match event.kind {
        EventKind::UnloadingComplete {
            task_force,
            elapsed_hours,
            ..
        } => Some((task_force, elapsed_hours, event.id)),
        _ => None,
    });
    assert_eq!(
        complete
            .iter()
            .filter(|(f, _, _)| *f != tf(3))
            .map(|(f, h, _)| (*f, *h))
            .collect::<Vec<_>>(),
        vec![(tf(1), n(30)), (tf(2), n(31))]
    );
    assert_eq!(complete.len(), 3);
    // The supply was landed at the factored rate too.
    let supply = world
        .events()
        .iter()
        .find(|event| {
            matches!(event.kind, EventKind::CargoUnloaded { task_force, .. } if task_force == tf(1))
        })
        .expect("supply unloaded");
    assert_eq!(factor(supply, "rate_t_per_h"), Some(n(15)));
    assert_eq!(factor(supply, "port_rate_t_per_h"), Some(n(20)));
    assert_eq!(factor(supply, "loading_factor"), Some(fx("0.75")));
    // Force 1's loading is spent and its order done, citing the completion.
    assert!(force(&world, 1).preparation.is_none());
    assert_eq!(force(&world, 1).landing_at, None);
    let spent = find(&world, |event| match event.kind {
        EventKind::LoadingLost {
            task_force,
            hex,
            loading_before,
            loading,
            reason,
        } => Some((
            task_force,
            hex,
            loading_before,
            loading,
            reason,
            event.causes.clone(),
        )),
        _ => None,
    });
    assert_eq!(
        spent,
        vec![(
            tf(1),
            B,
            n(50),
            Fixed::ZERO,
            LoadingLoss::Spent,
            vec![crate::events::Cause::Event(complete[0].2)]
        )]
    );
    // Force 3 landed at C, not B: its loading and order stand and its
    // landing is done.
    let prep = prep_of(&world, 3);
    assert_eq!((prep.hex, prep.loading), (B, n(50)));
    assert_eq!(force(&world, 3).landing_at, None);
    // Units are rested by the day again: 10, 30, 30.
    assert_eq!(unit(&world, 1).fatigue, n(10));
    assert_eq!(unit(&world, 2).fatigue, n(30));
    assert_eq!(unit(&world, 3).fatigue, n(30));
    assert_eq!(unit(&world, 1).men_ashore, 600);
    assert_sound(&world);
}

#[test]
fn with_the_frozen_figures_a_landing_is_as_it_was() {
    let mut world = world_with(frozen(), |_| {});
    world.submit_orders(
        Side::Allied,
        vec![
            tf_request(1, TaskForceOrder::route(vec![B])),
            tf_request(1, TaskForceOrder::Unload),
        ],
    );
    world.resolve_day().expect("day 1");
    // 20 t an hour for 2 hours: 160 men, and the breakdown has no landing
    // factors.
    let first = landed(&world, 1);
    assert_eq!(first[0].0, 160);
    assert_eq!(factor(&first[0].1, "rate_t_per_h"), Some(n(20)));
    for name in [
        "port_rate_t_per_h",
        "unprepared_landing_share",
        "loading",
        "loading_factor",
    ] {
        assert_eq!(factor(&first[0].1, name), None, "{name}");
    }
    // No fatigue on landing, and none of the new events but the days of
    // fighting counted: there was no fight, so none at all.
    assert_eq!(unit(&world, 1).fatigue, Fixed::ZERO);
    let new = find(&world, |event| match event.kind {
        EventKind::LoadingGained { .. }
        | EventKind::LoadingLost { .. }
        | EventKind::RehearsalGained { .. }
        | EventKind::RehearsalLost { .. }
        | EventKind::LeftRoster { .. }
        | EventKind::PreparationShared { .. }
        | EventKind::TrainingDay { .. }
        | EventKind::FightDayCounted { .. }
        | EventKind::ExperienceRaised { .. } => Some(()),
        EventKind::LandUnitFatigue {
            change: LandFatigueChange::Landed,
            ..
        } => Some(()),
        _ => None,
    });
    assert_eq!(new, Vec::new());
    assert_eq!(force(&world, 1).landing_at, Some(B));
}

#[test]
fn a_force_with_no_men_aboard_and_no_landing_begun_unloads_as_it_always_did() {
    let mut world = world_with(live(), |truth| {
        truth.ships.get_mut(&sh(1)).expect("ship").troops = None;
        truth.ships.get_mut(&sh(1)).expect("ship").supply_t = n(300);
    });
    // Loading for the very hex it unloads at, which would give a rate of 15
    // if the force were landing men.
    set_prep(&mut world, 1, A, &[], 50);
    world.submit_orders(Side::Allied, vec![tf_request(1, TaskForceOrder::Unload)]);
    world.resolve_day().expect("day 1");
    let event = world
        .events()
        .iter()
        .find(|event| matches!(event.kind, EventKind::CargoUnloaded { .. }))
        .expect("supply unloaded");
    assert_eq!(
        factor(event, "rate_t_per_h"),
        Some(n(20)),
        "the port's rate"
    );
    assert_eq!(factor(event, "loading_factor"), None);
    assert_eq!(force(&world, 1).landing_at, None);
    // The order is not spent by an unloading that landed no man.
    assert!(force(&world, 1).preparation.is_some());
}

#[test]
fn a_unit_divided_keeps_its_experience_and_nothing_else_of_its_parent() {
    let mut world = world_with(live(), |truth| {
        let parent = truth.land_units.get_mut(&lu(6)).expect("unit");
        parent.experience = 3;
        parent.training_days = 7;
        parent.fight_days = 3;
        parent.activity = Activity::Train;
        parent.activity_order = Some(OrderId::new(40));
        parent.rehearsal = n(40);
        parent.rehearsal_hex = Some(B);
        // 300 of unit 6's men are ashore at A; 100 more are aboard ship 1.
        truth.ships.get_mut(&sh(1)).expect("ship").troops = Some(Troops {
            unit: lu(6),
            men: 100,
        });
    });
    set_prep(&mut world, 1, B, &[6], 0);
    world.submit_orders(
        Side::Allied,
        vec![
            tf_request(1, TaskForceOrder::route(vec![B])),
            tf_request(1, TaskForceOrder::Unload),
        ],
    );
    world.resolve_day().expect("day 1");
    // The new unit is the next number after the highest, 10 (unit 9 exists).
    let divided = unit(&world, 10);
    assert_eq!(divided.experience, 3);
    assert_eq!(
        LandUnit {
            id: lu(6),
            name: unit(&world, 6).name.clone(),
            experience: 2,
            ..divided.clone()
        }
        .activity,
        Activity::Ready
    );
    assert_eq!(divided.training_days, 0);
    assert_eq!(divided.activity_order, None);
    assert_eq!(divided.rehearsal_hex, None);
    // Its parent's rehearsal was not copied: it landed with 40 fatigue, the
    // parent keeps what it had.
    assert_eq!(unit(&world, 6).rehearsal, n(40));
    assert_eq!(unit(&world, 6).experience, 3);
    assert_eq!(unit(&world, 6).training_days, 7);
    assert_eq!(divided.fatigue, n(40) - n(5), "40 on landing, rested 5");
    assert_sound(&world);
}

// ---- A split ----

/// Force 1 of ships 1 and 6, and units: 1 aboard ship 1, 8 aboard ship 6,
/// 2 aboard ship 1 and ship 6 (so neither force takes it whole), 6 ashore.
fn split_fresh() -> World {
    let mut world = world_with(live(), |truth| {
        truth.ships.insert(
            sh(6),
            Ship {
                fuel_t: n(1000),
                troops: Some(Troops {
                    unit: lu(8),
                    men: 100,
                }),
                ..Ship::new(
                    sh(6),
                    "Ship 6".to_string(),
                    ShipClassId::new(1),
                    Side::Allied,
                    ShipLocation::TaskForce(tf(1)),
                )
            },
        );
        truth
            .task_forces
            .get_mut(&tf(1))
            .expect("force")
            .ships
            .insert(sh(6));
        truth.land_units.insert(
            lu(8),
            LandUnit::new(lu(8), "Unit 8".to_string(), Side::Allied, 0, None),
        );
        // Unit 2's men are in both ships of force 1.
        truth.ships.get_mut(&sh(2)).expect("ship").troops = None;
        truth.ships.get_mut(&sh(1)).expect("ship").troops = Some(Troops {
            unit: lu(1),
            men: 600,
        });
        truth.ships.get_mut(&sh(6)).expect("ship").troops = Some(Troops {
            unit: lu(8),
            men: 100,
        });
        truth.land_units.get_mut(&lu(2)).expect("unit").men_ashore = 100;
        truth.land_units.get_mut(&lu(2)).expect("unit").hex = Some(A);
    });
    set_prep(&mut world, 1, B, &[1, 2, 6, 8], 40);
    set_rehearsal(&mut world, 8, B, 30);
    world
        .truth
        .task_forces
        .get_mut(&tf(1))
        .expect("force")
        .landing_at = Some(B);
    world
}

#[test]
fn a_force_that_splits_gives_each_part_its_loading_and_the_roster_units_that_go_with_the_ships() {
    let mut world = split_fresh();
    // Detach ship 6, which holds all of unit 8's men aboard.
    let (_, events) = give(
        &mut world,
        tf_request(1, TaskForceOrder::DetachShips(vec![sh(6)])),
    );
    let new_force = match &events[0] {
        EventKind::ShipsDetached { new_force, .. } => *new_force,
        other => panic!("a detachment, not {other:?}"),
    };
    assert_eq!(
        events[1],
        EventKind::PreparationShared {
            parent: tf(1),
            new_force,
            hex: B,
            loading: n(40),
            units: vec![lu(8)],
        }
    );
    let split = world
        .truth
        .task_forces
        .get(&new_force)
        .expect("the new force");
    let part = split
        .preparation
        .clone()
        .expect("the new part's preparation");
    assert_eq!((part.hex, part.loading), (B, n(40)));
    assert_eq!(part.roster, roster(&[8]));
    assert_eq!(
        split.landing_at,
        Some(B),
        "it has begun landing where its parent had"
    );
    assert_eq!(force(&world, 1).landing_at, Some(B));
    // Every other roster unit stays; the leaving unit left the parent's
    // roster.
    let parent = prep_of(&world, 1);
    assert_eq!(parent.roster, roster(&[1, 2, 6]));
    assert_eq!(parent.loading, n(40));
    // Rehearsal is never copied: unit 8 keeps its own.
    assert_eq!(unit(&world, 8).rehearsal, n(30));
    assert_eq!(unit(&world, 1).rehearsal, Fixed::ZERO);
    // The shared preparation cites the detachment and the order.
    let detached = world
        .events()
        .iter()
        .find(|event| matches!(event.kind, EventKind::ShipsDetached { .. }))
        .expect("detached");
    let shared = world
        .events()
        .iter()
        .find(|event| matches!(event.kind, EventKind::PreparationShared { .. }))
        .expect("shared");
    assert!(shared
        .causes
        .contains(&crate::events::Cause::Event(detached.id)));
    assert_sound(&world);
}

#[test]
fn a_force_without_a_preparation_shares_none() {
    let mut world = world_with(live(), |truth| {
        truth.ships.insert(
            sh(6),
            Ship::new(
                sh(6),
                "Ship 6".to_string(),
                ShipClassId::new(1),
                Side::Allied,
                ShipLocation::TaskForce(tf(1)),
            ),
        );
        truth
            .task_forces
            .get_mut(&tf(1))
            .expect("force")
            .ships
            .insert(sh(6));
    });
    let (_, events) = give(
        &mut world,
        tf_request(1, TaskForceOrder::DetachShips(vec![sh(6)])),
    );
    assert_eq!(events.len(), 1);
    let new_force = match &events[0] {
        EventKind::ShipsDetached { new_force, .. } => *new_force,
        other => panic!("a detachment, not {other:?}"),
    };
    assert!(world.truth.task_forces[&new_force].preparation.is_none());
    assert_eq!(world.truth.task_forces[&new_force].landing_at, None);
}

// ---- A join, which nothing in the game does yet ----

#[test]
fn forces_that_join_take_the_lower_loading_and_both_rosters() {
    let into = Preparation {
        hex: B,
        roster: roster(&[1]),
        loading: n(60),
        order: OrderId::new(3),
    };
    let from = Preparation {
        hex: B,
        roster: roster(&[2, 3]),
        loading: n(40),
        order: OrderId::new(9),
    };
    let joined_prep = joined(Some(&into), Some(&from)).expect("a preparation");
    assert_eq!(joined_prep.loading, n(40));
    assert_eq!(joined_prep.roster, roster(&[1, 2, 3]));
    assert_eq!((joined_prep.hex, joined_prep.order), (B, OrderId::new(3)));
    // The other way: the lower is the receiving force's.
    let low = Preparation {
        loading: n(20),
        ..into.clone()
    };
    assert_eq!(
        joined(Some(&low), Some(&from)).expect("prep").loading,
        n(20)
    );
    // A part for another hex counts as no loading; one with none, the same.
    let elsewhere = Preparation {
        hex: C,
        ..from.clone()
    };
    let joined_prep = joined(Some(&into), Some(&elsewhere)).expect("prep");
    assert_eq!(joined_prep.loading, Fixed::ZERO);
    assert_eq!(joined_prep.roster, roster(&[1, 2, 3]));
    assert_eq!(
        joined(Some(&into), None).expect("prep").loading,
        Fixed::ZERO
    );
    // None if the receiving force has none.
    assert_eq!(joined(None, Some(&from)), None);
    assert_eq!(joined(None, None), None);
    // A ship that joins keeps the share of the loading.
    let shipped = ship_joined(&into, fx("0.5"));
    assert_eq!(shipped.loading, n(30));
    assert_eq!(shipped.roster, into.roster);
    assert_eq!((shipped.hex, shipped.order), (B, OrderId::new(3)));
}

// ---- The hash ----

#[test]
fn every_new_field_is_part_of_the_state_hash() {
    let mut base = fresh();
    set_prep(&mut base, 1, B, &[1], 10);
    let hash = base.state_hash();
    assert_eq!(base.state_hash(), hash);
    type Change = Box<dyn Fn(&mut World)>;
    let changes: Vec<(&str, Change)> = vec![
        (
            "preparation hex",
            Box::new(|w| {
                w.truth
                    .task_forces
                    .get_mut(&tf(1))
                    .unwrap()
                    .preparation
                    .as_mut()
                    .unwrap()
                    .hex = C;
            }),
        ),
        (
            "preparation roster",
            Box::new(|w| {
                w.truth
                    .task_forces
                    .get_mut(&tf(1))
                    .unwrap()
                    .preparation
                    .as_mut()
                    .unwrap()
                    .roster
                    .insert(lu(6));
            }),
        ),
        (
            "preparation loading",
            Box::new(|w| {
                w.truth
                    .task_forces
                    .get_mut(&tf(1))
                    .unwrap()
                    .preparation
                    .as_mut()
                    .unwrap()
                    .loading = n(11);
            }),
        ),
        (
            "preparation order",
            Box::new(|w| {
                w.truth
                    .task_forces
                    .get_mut(&tf(1))
                    .unwrap()
                    .preparation
                    .as_mut()
                    .unwrap()
                    .order = OrderId::new(99);
            }),
        ),
        (
            "no preparation",
            Box::new(|w| {
                w.truth.task_forces.get_mut(&tf(1)).unwrap().preparation = None;
            }),
        ),
        (
            "landing_at",
            Box::new(|w| {
                w.truth.task_forces.get_mut(&tf(1)).unwrap().landing_at = Some(B);
            }),
        ),
        (
            "rehearsal",
            Box::new(|w| {
                let u = w.truth.land_units.get_mut(&lu(6)).unwrap();
                u.rehearsal = n(5);
                u.rehearsal_hex = Some(B);
            }),
        ),
        (
            "rehearsal hex",
            Box::new(|w| {
                w.truth.land_units.get_mut(&lu(6)).unwrap().rehearsal_hex = Some(C);
            }),
        ),
        (
            "activity",
            Box::new(|w| {
                w.truth.land_units.get_mut(&lu(6)).unwrap().activity = Activity::Train;
            }),
        ),
        (
            "activity order",
            Box::new(|w| {
                w.truth.land_units.get_mut(&lu(6)).unwrap().activity_order = Some(OrderId::new(5));
            }),
        ),
        (
            "experience",
            Box::new(|w| {
                w.truth.land_units.get_mut(&lu(6)).unwrap().experience = 3;
            }),
        ),
        (
            "training days",
            Box::new(|w| {
                w.truth.land_units.get_mut(&lu(6)).unwrap().training_days = 4;
            }),
        ),
        (
            "fight days",
            Box::new(|w| {
                w.truth.land_units.get_mut(&lu(6)).unwrap().fight_days = 4;
            }),
        ),
    ];
    let mut seen = BTreeSet::new();
    for (name, change) in &changes {
        let mut world = base.clone();
        change(&mut world);
        let changed = world.state_hash();
        assert_ne!(changed, hash, "{name} is hashed");
        seen.insert(changed);
    }
    assert_eq!(seen.len(), changes.len(), "each change gives its own hash");
}

#[test]
fn recording_breakdowns_changes_neither_what_happens_nor_the_hash() {
    let run = |record: bool| {
        let mut world = landing_world(live());
        world.set_config(SimConfig {
            record_breakdowns: record,
        });
        world.resolve_day().expect("day 1");
        world.resolve_day().expect("day 2");
        world
    };
    let (with, without) = (run(true), run(false));
    assert_eq!(with.state_hash(), without.state_hash());
    assert_eq!(with.day_hashes(), without.day_hashes());
    assert_eq!(kinds(&with), kinds(&without));
    assert!(with.events().iter().any(|event| event.breakdown.is_some()));
    assert!(without
        .events()
        .iter()
        .all(|event| event.breakdown.is_none()));
    // And an experienced fight.
    let fight = |record: bool| {
        let mut world = fight_world(live());
        world.set_config(SimConfig {
            record_breakdowns: record,
        });
        world.fight_ashore(1);
        world.fight_ashore(2);
        world
    };
    let (with, without) = (fight(true), fight(false));
    assert_eq!(with.state_hash(), without.state_hash());
    assert_eq!(kinds(&with), kinds(&without));
}

#[test]
fn the_defaults_of_a_land_unit_are_standard_experience_and_nothing_counted() {
    let unit = LandUnit::new(lu(1), "Unit".to_string(), Side::Allied, 10, Some(A));
    assert_eq!(unit.experience, 2);
    assert_eq!(LandUnit::DEFAULT_EXPERIENCE, 2);
    assert_eq!(LandUnit::MAX_EXPERIENCE, 4);
    assert_eq!(
        (
            unit.rehearsal,
            unit.rehearsal_hex,
            unit.activity,
            unit.activity_order
        ),
        (Fixed::ZERO, None, Activity::Ready, None)
    );
    assert_eq!((unit.training_days, unit.fight_days), (0, 0));
    assert_eq!((Activity::Ready.code(), Activity::Train.code()), (0, 1));
    assert_eq!(
        (Activity::Ready.name(), Activity::Train.name()),
        ("ready", "train")
    );
    assert_eq!(
        [
            LoadingLoss::NewLandingPlace,
            LoadingLoss::UnitAdded,
            LoadingLoss::Spent
        ]
        .map(LoadingLoss::code),
        [0, 1, 2]
    );
    assert_eq!(
        [
            RehearsalLoss::NewLandingPlace,
            RehearsalLoss::Landed,
            RehearsalLoss::OtherLandingPlace
        ]
        .map(RehearsalLoss::code),
        [0, 1, 2]
    );
    assert_eq!(
        [
            RosterLeft::TakenByOrder,
            RosterLeft::DroppedByOrder,
            RosterLeft::Landed
        ]
        .map(RosterLeft::code),
        [0, 1, 2]
    );
    assert_eq!(
        [ExperienceSource::Training, ExperienceSource::Fighting].map(ExperienceSource::code),
        [0, 1]
    );
}

#[test]
fn the_events_have_text_and_name_their_entities() {
    use crate::ids::EntityRef;
    let samples = vec![
        EventKind::LoadingGained {
            task_force: tf(1),
            hex: B,
            loading: n(5),
        },
        EventKind::LoadingLost {
            task_force: tf(1),
            hex: B,
            loading_before: n(25),
            loading: fx("12.5"),
            reason: LoadingLoss::UnitAdded,
        },
        EventKind::LoadingLost {
            task_force: tf(1),
            hex: B,
            loading_before: n(25),
            loading: Fixed::ZERO,
            reason: LoadingLoss::Spent,
        },
        EventKind::RehearsalGained {
            unit: lu(1),
            task_force: tf(1),
            hex: B,
            rehearsal: n(5),
        },
        EventKind::RehearsalLost {
            unit: lu(1),
            hex: B,
            rehearsal_before: n(5),
            reason: RehearsalLoss::Landed,
        },
        EventKind::LeftRoster {
            unit: lu(1),
            task_force: tf(1),
            reason: RosterLeft::Landed,
        },
        EventKind::PreparationShared {
            parent: tf(1),
            new_force: tf(6),
            hex: B,
            loading: n(5),
            units: vec![lu(1), lu(2)],
        },
        EventKind::PreparationShared {
            parent: tf(1),
            new_force: tf(6),
            hex: B,
            loading: n(5),
            units: Vec::new(),
        },
        EventKind::TrainingDay {
            unit: lu(1),
            days: 1,
            experience: 0,
        },
        EventKind::FightDayCounted {
            unit: lu(1),
            days: 3,
            experience: 2,
        },
        EventKind::ExperienceRaised {
            unit: lu(1),
            experience: 3,
            by: ExperienceSource::Fighting,
        },
    ];
    for kind in samples {
        let text = kind.to_string();
        assert!(!text.is_empty());
        assert!(!kind.entities().is_empty(), "{text}");
    }
    let shared = EventKind::PreparationShared {
        parent: tf(1),
        new_force: tf(6),
        hex: B,
        loading: n(5),
        units: vec![lu(2)],
    };
    assert_eq!(
        shared.entities(),
        vec![
            EntityRef::TaskForce(tf(1)),
            EntityRef::TaskForce(tf(6)),
            EntityRef::LandUnit(lu(2))
        ]
    );
    assert_eq!(
        EventKind::LandUnitFatigue {
            unit: lu(1),
            change: LandFatigueChange::Landed,
            fatigue: n(20)
        }
        .to_string(),
        format!("{} landed: fatigue 20", lu(1))
    );
    let _ = AirGroupId::new(1);
}

// ---- What the enemy sees ----

#[test]
fn two_worlds_that_differ_only_in_a_prepare_order_give_the_enemy_the_same_events() {
    let run = |ordered: bool| {
        let mut world = fresh();
        let mut orders = vec![
            tf_request(1, TaskForceOrder::route(vec![B])),
            tf_request(1, TaskForceOrder::Unload),
        ];
        if ordered {
            orders.push(tf_request(1, prepare(B, &[1])));
            orders.push(tf_request(2, prepare(B, &[2, 6])));
            orders.push(unit_request(6, LandUnitOrder::SetActivity(Activity::Train)));
        }
        world.submit_orders(Side::Allied, orders);
        // The enemy's own act, so that it has events of its own.
        world.submit_orders(
            Side::Japanese,
            vec![unit_request(9, LandUnitOrder::SetPosture(Posture::Defend))],
        );
        for _ in 0..3 {
            world.resolve_day().expect("a day");
        }
        world
    };
    let plain = run(false);
    let prepared = run(true);
    // The Allied side sees the difference ...
    assert!(prepared
        .events()
        .iter()
        .any(|event| matches!(event.kind, EventKind::LoadingGained { .. })));
    // ... and the Japanese side sees none of it.
    assert_eq!(
        shown(&plain, Side::Japanese),
        shown(&prepared, Side::Japanese)
    );
    let own = prepared.side_view(Side::Allied).events();
    assert!(own.iter().any(|event| matches!(
        event.kind,
        crate::side_events::SideEventKind::Whole(EventKind::LoadingGained { .. })
    )));
}

#[test]
fn two_worlds_that_differ_only_in_the_enemys_experience_give_a_side_the_same_events_when_the_hits_are(
) {
    // A seed is found where the same hits fall whatever the Japanese unit's
    // experience, since the chance differs.
    let run = |seed: u64, experience: u8| {
        let mut world = world_with(live(), |truth| {
            for (id, side, men, posture, exp) in [
                (10, Side::Allied, 1000, Posture::Defend, 2),
                (11, Side::Japanese, 600, Posture::Attack, experience),
            ] {
                let mut land_unit = LandUnit::new(lu(id), format!("Unit {id}"), side, men, Some(F));
                land_unit.posture = posture;
                land_unit.experience = exp;
                truth.land_units.insert(lu(id), land_unit);
            }
        });
        world = rebuild_with_seed(world, seed);
        world.resolve_day().expect("a day");
        world
    };
    let hits = |world: &World, id: u32| -> u32 {
        fires(world, id)
            .iter()
            .map(
                |(_, _, _, event)| match world.events().get(*event).expect("event").kind {
                    EventKind::GroundFire { hits, .. } => hits,
                    _ => 0,
                },
            )
            .sum()
    };
    let mut compared = 0;
    for seed in 1..=60 {
        let (low, high) = (run(seed, 1), run(seed, 4));
        if hits(&low, 11) != hits(&high, 11) || hits(&low, 10) != hits(&high, 10) {
            continue;
        }
        compared += 1;
        // The Allied side is given the same events: the Japanese unit's
        // experience, factor and chance are nothing of its.
        assert_eq!(
            shown(&low, Side::Allied),
            shown(&high, Side::Allied),
            "seed {seed}"
        );
        // The Japanese side is shown its own unit's experience, not the
        // chance.
        let own = high.side_view(Side::Japanese).events();
        let fire = own
            .iter()
            .find(|event| {
                matches!(
                    event.kind,
                    crate::side_events::SideEventKind::GroundFire { unit, .. } if unit == lu(11)
                )
            })
            .expect("the unit's own fire");
        let names: Vec<&str> = fire
            .breakdown
            .as_ref()
            .expect("a breakdown")
            .factors
            .iter()
            .map(|f| f.name)
            .collect();
        assert!(names.contains(&"experience"));
        assert!(names.contains(&"experience_factor"));
        assert!(!names.contains(&"chance"));
        assert!(!names.contains(&"side_chance"));
    }
    assert!(compared >= 3, "seeds with the same hits: {compared}");
}

/// What a side is given, with the umpire's event numbers (which the side
/// cannot read and which differ with what the other side did) left out.
fn shown(world: &World, side: Side) -> Vec<String> {
    world
        .side_view(side)
        .events()
        .into_iter()
        .map(|event| {
            format!(
                "{} day {} {:?} {:?} order {:?} {:?} hidden {} {:?}",
                event.number,
                event.day,
                event.pulse,
                event.kind,
                event.order_number,
                event.causes,
                event.hidden_causes,
                event.breakdown
            )
        })
        .collect()
}

/// The same world under another seed.
fn rebuild_with_seed(world: World, seed: u64) -> World {
    World::new(
        seed,
        world.ref_data().clone(),
        world.map().clone(),
        world.umpire().clone(),
    )
    .expect("a sound setup")
}
