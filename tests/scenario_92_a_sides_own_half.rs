//! Scenario 92: a side's own half of a picture. A side's view gives, on an
//! evening, everything of its own that a picture of a world needs, with no
//! number of the live game in it; two halves make a picture.
//!
//! Proves decision 0171, the first part of step 4 of step 3's stage 4,
//! against a hand calculation. It has no scenario file of its own: it reads
//! live games of scenarios 1, 68 and 86 and of the full slice. It runs on
//! the frozen step 1 figures, but for the full slice, which runs as its own
//! tests run it.
//!
//! # Hand calculation
//!
//! Written by the architect before the code. Everything is asserted
//! exactly. If the code disagrees, the code or the rule is wrong: do not
//! change a number here to make a test pass. A mismatch goes back to the
//! architect.
//!
//! ## 1. By hand: the Allied half of scenario 1 on the evening of day 1
//!
//! Scenario 1 is played for one day (see its hand calculation: the force
//! steams 384 nm on day 1 and is 10 nm short of (8,3), in its hex). The
//! Allied side's view gives this half:
//!
//! - **The day:** 1. **The side:** Allied.
//! - **Bases:** base 1, Noumea, at (0,0) and base 2, Espiritu Santo, at
//!   (8,3), as the game holds them.
//! - **Task force 1:** at (8,3), under way, with its route of one
//!   waypoint, (8,3), and the speed cruise. Its route cites the order at
//!   place 1 and its speed the order at place 2. It cites no event: it
//!   has not arrived, is not alerted, halted or on a yard trip.
//! - **Ships:** ship 1 with 2,000 - 96 = 1,904 t of fuel and ship 2 with
//!   500 - 48 = 452 t, both in task force 1.
//! - **No air group, land unit, formation, replacement or arrival.**
//! - **The orders in force,** two: at place 1 the route for task force 1
//!   to (8,3), and at place 2 the speed cruise for task force 1; each
//!   Allied, issued on day 1, by the commander.
//! - **The belief:** no contact, no claim, no estimate.
//! - **The command state:** empty, the next plan 1.
//! - **The counters:** 1,000,001 for a force and 1,000,001 for an air
//!   group.
//!
//! The Japanese side's half of the same evening holds the day, 1, the
//! side, nothing else, an empty command state, and the counters 2,000,001
//! and 2,000,001.
//!
//! The two joined are a picture of day 1 with orders 1 and 2 as above. A
//! world built from it on the game's own seed with no domain (the
//! umpire's rebuild of decision 0170) and resolved for days 2 and 3 ends
//! each with ship 1 at 1,901.5 t and ship 2 at 450.75 t, the force
//! arrived at hour 24.625, as scenario 1 does.
//!
//! ## 2. Two halves are the whole
//!
//! - **Scenario 86,** played with both staffs to the evening of day 3.
//!   The Allied half and the Japanese half, joined, are equal to the
//!   umpire's whole picture of that evening, field for field, with no
//!   objectives. (The Japanese side has given no order, so the orders
//!   are in the same places.) A world built from the joined halves on
//!   the game's seed with no domain and played on to day 8 with both
//!   staffs has, at the end of each of days 4 to 8, the live game's
//!   ships, task forces, bases and land units, and ends with base 1 at
//!   915 t, base 2 at 70 t and base 5 at 10 t.
//! - **The full slice,** `07_full_slice.toml`, played as its own test
//!   plays it to the evening of day 3. The two halves joined hold the
//!   units, the belief, the command states and the counters of the
//!   umpire's whole picture, field for field; their orders are the same
//!   orders, each unit citing the same order, with the Allied side's
//!   first and then the Japanese side's where the whole picture has them
//!   in the order they were given. A world built from the joined halves
//!   on the game's seed with no domain, given the scenario's orders of
//!   days 4 and 5 as the scenario gives them, has at the end of each of
//!   those days the live game's ships, task forces, bases and air groups,
//!   field for field but for the numbers of the orders and events they
//!   cite. (Corrected by the architect on Nov 19, 2026: this sentence
//!   said that day 4 is the day of the carrier battle. In the full slice
//!   of 39 days the carriers meet on days 18 and 19, and days 4 and 5
//!   have no fight at sea or in the air.)
//!
//!   So that the fights are covered: on every evening from day 1 to day
//!   38 the two halves joined hold what the whole picture holds, as
//!   above, and a world built from them on the game's seed with no
//!   domain, given the scenario's orders of every later day, has at the
//!   end of each day to day 39 the live game's ships, task forces, bases,
//!   air groups and land units, and the events of each day are the live
//!   game's by day, pulse and kind. The carrier battle of days 18 and 19,
//!   the raids on bases, the ships sunk and the fights ashore are all
//!   among them.
//!
//!   This case is also the test decision 0170 owes of a world with days
//!   behind it (its rulings, 7). If the built world and the live game
//!   part, the first difference is traced to the rule that read the
//!   history, and goes to the architect: the statement is not changed.
//!
//! ## 3. No peek
//!
//! Scenario 68's pair of worlds, as its property "what the side cannot
//! see changes nothing" makes them: the same game, and in one of them
//! Japanese base 2 holds 300 t of supply. Each is played to the evening
//! of day 5. The Allied side's two halves are equal, field for field.
//! The Japanese side's two halves differ, in base 2's supply and nowhere
//! else.
//!
//! In the full slice on the evening of day 3, the count of orders in the
//! Allied half is the count of orders its own units and command state
//! refer to, and no order in it is at a place above that count, whatever
//! the Japanese side has ordered.
//!
//! ## 4. What it refuses
//!
//! | Asked | Refused because |
//! | --- | --- |
//! | A half while a day is open (after the night pulse of day 2 of scenario 1) | a picture is of an evening |
//! | A half at a view cut to a seat (scenario 63 or another with command levels, at a task force's seat) | only a whole side has a half |
//! | Two Allied halves joined | two halves of one side |
//! | An Allied half of day 1 and a Japanese half of day 2 joined | halves of different days |
//! | Two halves that each hold a ship 1 | two units of one number |
//!
//! # Properties
//!
//! - **A half taken twice is the same half.**
//! - **Nothing of the live game is in it.** Every order a unit of the
//!   half cites is at a place from 1 to the count of its orders, and
//!   every event a unit cites is the first.
//! - **A half made by hand joins as a copied one does.** A Japanese half
//!   of day 1 made in the test, of one base and one force of one ship
//!   with a route citing the order at its place 1, joined to scenario
//!   1's Allied half of day 1, gives a picture whose Japanese force
//!   cites order 3, and a world is built from it.
//! - **Nothing that plays can take or join a half** outside what its
//!   own crate is allowed: the test that reads the agent's source
//!   passes, with the new words in it.
//! - **No live game changes:** no baseline, golden file or pinned hash
//!   moves.

mod common;

use std::collections::{BTreeMap, BTreeSet};

use sim_core::command::{CommandState, Hold, HoldMaker};
use sim_core::events::{Cause, Event};
use sim_core::fixed::Fixed;
use sim_core::hex::Hex;
use sim_core::ids::{BaseId, EventId, OrderId, ShipClassId, ShipId, TaskForceId};
use sim_core::orders::{OrderKind, Origin, SpeedSetting, TaskForceOrder};
use sim_core::pictures::CommandRef;
use sim_core::rng::{Domain, Stream};
use sim_core::state::{
    AirGroup, CargoOrder, Charge, Halt, LandUnit, March, Preparation, Route, Side, TaskForce,
    TaskForceStatus, TruthState, YardTrip,
};
use sim_core::world_picture::{
    PictureBelief, PictureCounters, PictureHalf, PictureOrder, WorldPicture,
};
use sim_core::{SimError, Staffs, World};
use sim_data::Scenario;
use sim_staff::{as_staffs, staffs};

const TASK_FORCE: TaskForceId = TaskForceId::new(1);
const CRUISER: ShipId = ShipId::new(1);
const DESTROYER: ShipId = ShipId::new(2);

fn scenario(name: &str) -> Scenario {
    common::load(&common::data_dir().join("scenarios").join(name)).expect("the scenario loads")
}

/// The full slice as its own test loads it: on the live figures. No number
/// here depends on a figure; every claim on it is of equality.
fn full_slice() -> Scenario {
    common::load_live(&common::data_dir().join("scenarios/07_full_slice.toml"))
        .expect("the full slice loads")
}

fn tons(value: i64) -> Fixed {
    Fixed::from_int(value)
}

fn hundredths(value: i64) -> Fixed {
    Fixed::from_ratio(value, 100)
}

/// A value in words. An order's and an event's number show as `_` outside
/// the engine, so two values that differ only in the numbers of the orders
/// and events they cite read the same.
fn words<T: std::fmt::Debug>(value: &T) -> String {
    format!("{value:?}")
}

fn half(world: &World, side: Side) -> PictureHalf {
    world
        .side_view(side)
        .own_half()
        .expect("a whole side's half on an evening")
}

fn joined(world: &World) -> WorldPicture {
    WorldPicture::from_halves(half(world, Side::Allied), half(world, Side::Japanese))
        .expect("one half of each side, of one day")
}

/// The umpire's rebuild of a picture on the live game's seed with no
/// domain (decision 0170, 13).
fn rebuild(live: &World, picture: WorldPicture) -> World {
    World::rebuild_from_picture(
        picture,
        live.ref_data().clone(),
        live.map().clone(),
        live.seed(),
        common::umpire_key(),
    )
    .expect("the joined picture is sound")
}

fn refused(result: Result<impl std::fmt::Debug, SimError>, because: &str) {
    match result {
        Err(SimError::InvalidSetup(words)) => {
            assert!(words.contains(because), "{words:?} should say {because:?}")
        }
        other => panic!("expected a refusal saying {because:?}, got {other:?}"),
    }
}

/// A number a unit or a command state holds: of an order or of an event.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Cited {
    Order(OrderId),
    Event(EventId),
}

/// Every number of an order or an event a state and its command states
/// hold, with words for where it is held, in a fixed order. Each struct
/// that holds one is named with all its fields, as the engine's own walk
/// names them, so that a field added to one does not compile here until it
/// is read.
fn cited(truth: &TruthState, commands: &BTreeMap<Side, CommandState>) -> Vec<(String, Cited)> {
    let mut out = Vec::new();
    let TruthState {
        day: _,
        ships: _,
        task_forces,
        bases: _,
        air_groups,
        replacements: _,
        arrivals: _,
        land_units,
        hits_on_men,
        formations: _,
        entering: _,
        supply_lines: _,
        men_replacement: _,
        // A boat holds an order, walked by the engine's own walk; this
        // test's scenarios have none (decision 0218).
        submarines: _,
        // Waters waiting to be told hold no order (decision 0218, 47).
        waters_waiting: _,
    } = truth;
    for force in task_forces.values() {
        cited_by_force(force, &mut out);
    }
    for group in air_groups.values() {
        cited_by_group(group, &mut out);
    }
    for unit in land_units.values() {
        cited_by_land_unit(unit, &mut out);
    }
    for ((hex, side), hits) in hits_on_men {
        for cause in &hits.causes {
            out.push((format!("{side:?} hits at {hex}"), Cited::Event(*cause)));
        }
    }
    for (side, command) in commands {
        let CommandState {
            intents: _,
            leaves: _,
            plans: _,
            commitments: _,
            holds,
            next_plan: _,
            interrupts: _,
            standing: _,
            standing_off: _,
            pursue_leaves: _,
            pursuits,
            pursuit_refusals: _,
        } = command;
        // A leave to pursue by answer names the event of the answer
        // (decision 0214).
        for (force, pursuit) in pursuits {
            if let sim_core::command::PursuitBy::Answer(event) = pursuit.by {
                out.push((
                    format!("{side:?} leave to pursue of {force}"),
                    Cited::Event(event),
                ));
            }
        }
        for (target, Hold { day: _, by }) in holds {
            if let HoldMaker::Order(order) = by {
                out.push((format!("{side:?} hold on {target}"), Cited::Order(*order)));
            }
        }
    }
    out
}

fn cited_by_force(force: &TaskForce, out: &mut Vec<(String, Cited)>) {
    let TaskForce {
        id,
        name: _,
        side: _,
        ships: _,
        mission: _,
        speed: _,
        route: _,
        from_hex: _,
        toward_hex: _,
        progress_nm: _,
        leg_nm: _,
        status: _,
        route_order,
        speed_order,
        cargo_order,
        arrived_at_hours: _,
        arrival_event,
        readiness: _,
        readiness_order,
        alerted: _,
        alerted_by,
        rules_of_engagement: _,
        rules_order,
        home_port: _,
        fatigue: _,
        yard_trip,
        halt,
        preparation,
        landing_at: _,
        charge,
        available_from: _,
        fuel_from,
    } = force;
    let mut order = |what: &str, held: Option<OrderId>| {
        if let Some(held) = held {
            out.push((format!("{id} {what}"), Cited::Order(held)));
        }
    };
    order("route", *route_order);
    order("speed", *speed_order);
    order("readiness", *readiness_order);
    order("rules", *rules_order);
    order("fuel-from", fuel_from.map(|fuel_from| fuel_from.order));
    if let Some(CargoOrder {
        action: _,
        order: held,
        unit: _,
        weight_t: _,
        heeds_reserve: _,
        each_t: _,
    }) = cargo_order
    {
        order("cargo", Some(*held));
    }
    if let Some(Preparation {
        hex: _,
        roster: _,
        loading: _,
        order: held,
    }) = preparation
    {
        order("preparation", Some(*held));
    }
    if let Some(Charge {
        force: _,
        order: held,
    }) = charge
    {
        order("escort", Some(*held));
    }
    let mut event = |what: &str, held: Option<EventId>| {
        if let Some(held) = held {
            out.push((format!("{id} {what}"), Cited::Event(held)));
        }
    };
    event("arrival", *arrival_event);
    event("alert", *alerted_by);
    if let Some(YardTrip {
        base: _,
        return_day: _,
        departure,
    }) = yard_trip
    {
        event("yard", Some(*departure));
    }
    if let Some(Halt {
        work: _,
        at_hours: _,
        event: halted,
        bombarded,
        cargo_completed,
    }) = halt
    {
        event("halt", Some(*halted));
        event("bombarded", *bombarded);
        event("cargo completed", *cargo_completed);
    }
}

fn cited_by_group(group: &AirGroup, out: &mut Vec<(String, Cited)>) {
    let AirGroup {
        id,
        name: _,
        side: _,
        location: _,
        aircraft_type: _,
        ready: _,
        damaged: _,
        mission: _,
        search: _,
        mission_order,
        arc_order,
        patrol_share: _,
        share_order,
        fatigue: _,
        rest_above: _,
        rest_order,
        stood_down: _,
        pilots: _,
        strength: _,
        reach: _,
        reach_order,
        target_priority: _,
        target_order,
        last_base_strike_day: _,
        rebased_day: _,
        strike_contact_age: _,
        strike_age_order,
        strike_held_share: _,
        strike_share_order,
        strike_sizing: _,
        strike_sizing_order,
        search_once: _,
        search_once_order,
        available_from: _,
        // A tally cites no order and no event (decision 0180).
        claims: _,
        strike_first_waters: _,
        waters_order,
    } = group;
    for (what, held) in [
        ("mission", mission_order),
        ("arc", arc_order),
        ("share", share_order),
        ("rest", rest_order),
        ("reach", reach_order),
        ("target", target_order),
        ("strike age", strike_age_order),
        ("strike share", strike_share_order),
        ("strike sizing", strike_sizing_order),
        ("search once", search_once_order),
        ("waters", waters_order),
    ] {
        if let Some(held) = held {
            out.push((format!("{id} {what}"), Cited::Order(*held)));
        }
    }
}

fn cited_by_land_unit(unit: &LandUnit, out: &mut Vec<(String, Cited)>) {
    let LandUnit {
        id,
        name: _,
        side: _,
        men_ashore: _,
        hex: _,
        posture: _,
        posture_order,
        fatigue: _,
        fed_share: _,
        march,
        rehearsal: _,
        rehearsal_hex: _,
        activity: _,
        activity_order,
        experience: _,
        training_days: _,
        fight_days: _,
        skills: _,
        working: _,
        when_spent: _,
        fieldworks: _,
        available_from: _,
        formation: _,
        hunger_nights: _,
    } = unit;
    for (what, held) in [("posture", posture_order), ("activity", activity_order)] {
        if let Some(held) = held {
            out.push((format!("{id} {what}"), Cited::Order(*held)));
        }
    }
    if let Some(March {
        to: _,
        days: _,
        counted: _,
        cause,
    }) = march
    {
        let held = match cause {
            Cause::Order(order) => Cited::Order(*order),
            Cause::Event(event) => Cited::Event(*event),
        };
        out.push((format!("{id} march"), held));
    }
}

/// What a half's units and command state cite.
fn cited_by_half(half: &PictureHalf) -> Vec<(String, Cited)> {
    let truth = TruthState {
        day: half.day,
        ships: half.ships.clone(),
        task_forces: half.task_forces.clone(),
        bases: half.bases.clone(),
        air_groups: half.air_groups.clone(),
        replacements: half.replacements.clone(),
        arrivals: half.arrivals.clone(),
        land_units: half.land_units.clone(),
        hits_on_men: BTreeMap::new(),
        formations: half.formations.clone(),
        entering: half.entering.clone(),
        supply_lines: half.supply_lines.clone(),
        men_replacement: half.men_replacement.clone(),
        submarines: half.submarines.clone(),
        waters_waiting: Vec::new(),
    };
    cited(&truth, &BTreeMap::from([(half.side, half.command.clone())]))
}

/// The order a picture's list holds at `order`'s place.
fn order_in(orders: &[PictureOrder], order: OrderId) -> &PictureOrder {
    let place = order.umpire_raw(common::umpire_key());
    &orders[usize::try_from(place).expect("a place") - 1]
}

fn first_event() -> EventId {
    EventId::for_umpire(1, common::umpire_key())
}

/// Nothing of the live game is in a half: every order it cites is at a
/// place from 1 to the count of its orders, every event it cites is the
/// first, and every order in its list is cited.
fn nothing_of_the_live_game(half: &PictureHalf) {
    let count = u32::try_from(half.orders.len()).expect("a count");
    let mut places = BTreeSet::new();
    for (place, held) in cited_by_half(half) {
        match held {
            Cited::Order(order) => {
                let at = order.umpire_raw(common::umpire_key());
                assert!(
                    (1..=count).contains(&at),
                    "{place} cites order {at} of {count}"
                );
                places.insert(at);
            }
            Cited::Event(event) => assert_eq!(event, first_event(), "{place}"),
        }
    }
    assert_eq!(places, (1..=count).collect(), "every order listed is cited");
}

// ---------------------------------------------------------------------------
// 1. By hand: the Allied half of scenario 1 on the evening of day 1.
// ---------------------------------------------------------------------------

fn route_order() -> OrderKind {
    OrderKind::TaskForce {
        target: TASK_FORCE,
        order: TaskForceOrder::route(vec![Hex::new(8, 3)]),
    }
}

fn speed_order() -> OrderKind {
    OrderKind::TaskForce {
        target: TASK_FORCE,
        order: TaskForceOrder::SetSpeed(SpeedSetting::Cruise),
    }
}

fn scenario_one_on_day_one() -> Scenario {
    let mut live = scenario("01_transit.toml");
    live.next_day().expect("day 1").expect("day 1 exists");
    live
}

#[test]
fn case_one_the_allied_half_by_hand() {
    let live = scenario_one_on_day_one();
    let half = half(&live.world, Side::Allied);

    assert_eq!(half.day, 1);
    assert_eq!(half.side, Side::Allied);

    // The bases, as the game holds them.
    let truth = live.world.umpire();
    assert_eq!(
        half.bases.keys().copied().collect::<Vec<_>>(),
        vec![BaseId::new(1), BaseId::new(2)]
    );
    assert_eq!(half.bases[&BaseId::new(1)].name, "Noumea");
    assert_eq!(half.bases[&BaseId::new(1)].hex, Hex::new(0, 0));
    assert_eq!(half.bases[&BaseId::new(2)].name, "Espiritu Santo");
    assert_eq!(half.bases[&BaseId::new(2)].hex, Hex::new(8, 3));
    assert_eq!(half.bases, truth.bases);

    // Task force 1, under way in (8,3), its route and speed citing the
    // orders at places 1 and 2, and no event.
    assert_eq!(
        half.task_forces.keys().copied().collect::<Vec<_>>(),
        vec![TASK_FORCE]
    );
    let force = &half.task_forces[&TASK_FORCE];
    assert_eq!(force.position(), Hex::new(8, 3));
    assert_eq!(force.status, TaskForceStatus::Underway);
    assert_eq!(
        force.route,
        Route {
            waypoints: vec![Hex::new(8, 3)],
            next_index: force.route.next_index,
            act_at: BTreeSet::new(),
            stand_at: BTreeSet::new(),
        }
    );
    assert_eq!(force.route.waypoints, vec![Hex::new(8, 3)]);
    assert_eq!(force.speed, SpeedSetting::Cruise);
    assert_eq!(force.route_order, Some(WorldPicture::order_at(1)));
    assert_eq!(force.speed_order, Some(WorldPicture::order_at(2)));
    assert_eq!(force.arrival_event, None);
    assert_eq!(force.alerted_by, None);
    assert_eq!(force.halt, None);
    assert_eq!(force.yard_trip, None);
    assert_eq!(
        cited_by_half(&half)
            .into_iter()
            .map(|(_, held)| held)
            .collect::<Vec<_>>(),
        vec![
            Cited::Order(WorldPicture::order_at(1)),
            Cited::Order(WorldPicture::order_at(2))
        ]
    );

    // The ships: 2,000 - 96 and 500 - 48 t, both in task force 1.
    assert_eq!(
        half.ships.keys().copied().collect::<Vec<_>>(),
        vec![CRUISER, DESTROYER]
    );
    assert_eq!(half.ships[&CRUISER].fuel_t, tons(1904));
    assert_eq!(half.ships[&DESTROYER].fuel_t, tons(452));
    assert_eq!(force.ships, BTreeSet::from([CRUISER, DESTROYER]));

    // Nothing else of its own.
    assert!(half.air_groups.is_empty());
    assert!(half.land_units.is_empty());
    assert!(half.formations.is_empty());
    assert!(half.replacements.is_empty());
    assert!(half.arrivals.is_empty());

    // The orders in force, two.
    assert_eq!(
        half.orders,
        vec![
            PictureOrder {
                side: Side::Allied,
                day_issued: 1,
                kind: route_order(),
                origin: Origin::PLAYER,
            },
            PictureOrder {
                side: Side::Allied,
                day_issued: 1,
                kind: speed_order(),
                origin: Origin::PLAYER,
            },
        ]
    );

    // No contact, claim or estimate; the command state empty, its next
    // plan 1; the counters.
    assert_eq!(half.belief, PictureBelief::default());
    assert_eq!(half.command, CommandState::new());
    assert_eq!(half.command.next_plan, 1);
    assert_eq!(
        half.counters,
        PictureCounters {
            next_task_force: 1_000_001,
            next_air_group: 1_000_001,
        }
    );
}

#[test]
fn case_one_the_japanese_half_holds_nothing_but_its_day_and_counters() {
    let live = scenario_one_on_day_one();
    let half = half(&live.world, Side::Japanese);
    assert_eq!(
        half,
        PictureHalf {
            day: 1,
            side: Side::Japanese,
            ships: BTreeMap::new(),
            task_forces: BTreeMap::new(),
            bases: BTreeMap::new(),
            air_groups: BTreeMap::new(),
            land_units: BTreeMap::new(),
            formations: BTreeMap::new(),
            entering: BTreeMap::new(),
            replacements: Vec::new(),
            supply_lines: Vec::new(),
            men_replacement: Default::default(),
            submarines: BTreeMap::new(),
            arrivals: Vec::new(),
            orders: Vec::new(),
            command: CommandState::new(),
            belief: PictureBelief::default(),
            counters: PictureCounters {
                next_task_force: 2_000_001,
                next_air_group: 2_000_001,
            },
            // Decision 0194, 11: the game's days, scenario 1's 3, as the
            // view gives them.
            game_days: Some(3),
        }
    );
}

#[test]
fn case_one_the_halves_joined_play_on_as_scenario_one_does() {
    let live = scenario_one_on_day_one();
    let picture = joined(&live.world);
    assert_eq!(picture.day, 1);
    assert_eq!(
        picture
            .orders
            .iter()
            .map(|order| order.kind.clone())
            .collect::<Vec<_>>(),
        vec![route_order(), speed_order()]
    );
    assert_eq!(picture.objectives, None);
    let mut built = rebuild(&live.world, picture);
    assert_eq!(built.domain(), None);
    for day in 2..=3 {
        let summary = built.resolve_day().expect("a day");
        assert_eq!(summary.day, day);
        let truth = built.umpire();
        assert_eq!(
            truth.ships[&CRUISER].fuel_t,
            hundredths(190_150),
            "day {day}"
        );
        assert_eq!(
            truth.ships[&DESTROYER].fuel_t,
            hundredths(45_075),
            "day {day}"
        );
        let force = &truth.task_forces[&TASK_FORCE];
        assert_eq!(force.status, TaskForceStatus::Arrived, "day {day}");
        assert_eq!(
            force.arrived_at_hours,
            Some(Fixed::from_ratio(24_625, 1_000)),
            "day {day}"
        );
    }
}

// ---------------------------------------------------------------------------
// 2. Two halves are the whole.
// ---------------------------------------------------------------------------

/// Scenario 86, played by each side's staff to the evening of day 3.
#[test]
fn scenario_eighty_six_two_halves_are_the_whole() {
    let key = common::umpire_key();
    let mut live = scenario("86_fed_with_the_lift.toml");
    let held = staffs();
    for _ in 1..=3 {
        live.next_day_watched(as_staffs(&held), &mut |_, _| {})
            .expect("a day")
            .expect("a day exists");
    }
    let picture = joined(&live.world);
    let mut whole = live.world.whole_picture(key).expect("an evening");
    whole.objectives = None;
    assert_eq!(picture, whole, "the halves joined are the whole picture");

    let mut built = rebuild(&live.world, picture);
    let held_built = staffs();
    for day in 4..=8 {
        live.next_day_watched(as_staffs(&held), &mut |_, _| {})
            .expect("a day")
            .expect("a day exists");
        let staffs_built: Staffs<'_> = as_staffs(&held_built);
        let summary = built.resolve_day_staffed(staffs_built).expect("a day");
        assert_eq!(summary.day, day);
        let (a, b) = (live.world.umpire(), built.umpire());
        assert_eq!(words(&a.ships), words(&b.ships), "the ships, day {day}");
        assert_eq!(
            words(&a.task_forces),
            words(&b.task_forces),
            "the task forces, day {day}"
        );
        assert_eq!(words(&a.bases), words(&b.bases), "the bases, day {day}");
        assert_eq!(
            words(&a.land_units),
            words(&b.land_units),
            "the land units, day {day}"
        );
    }
    let supply = |base: u32| built.umpire().bases[&BaseId::new(base)].supply_t;
    assert_eq!(supply(1), tons(915));
    assert_eq!(supply(2), tons(70));
    assert_eq!(supply(5), tons(10));
}

/// The full slice played as its own test plays it, on its scripted orders,
/// to the evening of `evening`; and the scenario as loaded, for its script.
fn full_slice_on_evening(evening: u32) -> (Scenario, Scenario) {
    let mut live = full_slice();
    let script = full_slice();
    for _ in 1..=evening {
        live.next_day().expect("a day").expect("a day exists");
    }
    (live, script)
}

fn full_slice_on_day_three() -> (Scenario, Scenario) {
    full_slice_on_evening(3)
}

/// Each place a picture's state and command states cite, with the order
/// or event it cites there: an order by its content, so that two pictures
/// that number their orders differently can be set side by side.
fn cited_in_words(picture: &WorldPicture) -> Vec<(String, String)> {
    cited(&picture.truth, &picture.command)
        .into_iter()
        .map(|(place, held)| {
            let what = match held {
                Cited::Order(order) => words(order_in(&picture.orders, order)),
                Cited::Event(event) => {
                    format!("event {}", event.umpire_raw(common::umpire_key()))
                }
            };
            (place, what)
        })
        .collect()
}

#[test]
fn the_full_slice_two_halves_are_the_whole_and_play_on_as_it_does() {
    full_slice_round_trip(3, 5);
}

/// The full slice's halves on the evening of `evening` are the whole
/// picture, and a world built from them, given the scenario's orders, has
/// the live game's units and events on every day to `through`.
fn full_slice_round_trip(evening: u32, through: u32) {
    let key = common::umpire_key();
    let (mut live, script) = full_slice_on_evening(evening);
    let allied = half(&live.world, Side::Allied);
    let japanese = half(&live.world, Side::Japanese);
    nothing_of_the_live_game(&allied);
    nothing_of_the_live_game(&japanese);
    let picture = WorldPicture::from_halves(allied.clone(), japanese.clone()).expect("joined");
    let whole = live.world.whole_picture(key).expect("an evening");

    // The units, the belief, the command states and the counters of the
    // whole picture, field for field.
    assert_eq!(picture.day, whole.day);
    let (a, b) = (&picture.truth, &whole.truth);
    assert_eq!(words(&a.ships), words(&b.ships), "the ships");
    assert_eq!(words(&a.task_forces), words(&b.task_forces), "the forces");
    assert_eq!(words(&a.bases), words(&b.bases), "the bases");
    assert_eq!(words(&a.air_groups), words(&b.air_groups), "the air groups");
    assert_eq!(words(&a.land_units), words(&b.land_units), "the land units");
    assert_eq!(a.formations, b.formations, "the formations");
    assert_eq!(a.hits_on_men, b.hits_on_men);
    assert_eq!(a.replacements, b.replacements, "the replacements");
    assert_eq!(a.arrivals, b.arrivals, "the arrivals");
    assert_eq!(picture.beliefs, whole.beliefs, "the belief");
    assert_eq!(
        words(&picture.command),
        words(&whole.command),
        "the command states"
    );
    assert_eq!(picture.counters, whole.counters, "the counters");

    // The same orders, each unit citing the same order; the Allied side's
    // first, then the Japanese side's.
    assert_eq!(cited_in_words(&picture), cited_in_words(&whole));
    assert_eq!(picture.orders.len(), whole.orders.len());
    let sides: Vec<Side> = picture.orders.iter().map(|order| order.side).collect();
    let allied_count = allied.orders.len();
    assert!(allied_count > 0 && !japanese.orders.is_empty());
    assert!(sides[..allied_count].iter().all(|s| *s == Side::Allied));
    assert!(sides[allied_count..].iter().all(|s| *s == Side::Japanese));
    let as_set =
        |orders: &[PictureOrder]| -> BTreeSet<String> { orders.iter().map(words).collect() };
    assert_eq!(as_set(&picture.orders), as_set(&whole.orders));

    // A world built from the joined halves, given the scenario's orders of
    // the days after, has the live game's units at the end of each, and
    // everything that happens in it, every fight hit for hit, is the live
    // game's.
    let mut built = rebuild(&live.world, picture);
    for day in evening + 1..=through {
        let first_live = live.world.events().len();
        live.next_day().expect("a day").expect("a day exists");
        let first_built = built.events().len();
        play_scripted_day(&mut built, &script, day);
        assert_eq!(built.day(), day);
        let (a, b) = (live.world.umpire(), built.umpire());
        assert_eq!(words(&a.ships), words(&b.ships), "the ships, day {day}");
        assert_eq!(
            words(&a.task_forces),
            words(&b.task_forces),
            "the task forces, day {day}"
        );
        assert_eq!(words(&a.bases), words(&b.bases), "the bases, day {day}");
        assert_eq!(
            words(&a.air_groups),
            words(&b.air_groups),
            "the air groups, day {day}"
        );
        assert_eq!(
            words(&a.land_units),
            words(&b.land_units),
            "the land units, day {day}"
        );
        let happened = |log: &[&Event]| -> Vec<String> {
            log.iter()
                .map(|event| words(&(event.day, event.pulse, &event.kind)))
                .collect()
        };
        let live_log: Vec<&Event> = live.world.events().iter().skip(first_live).collect();
        let built_log: Vec<&Event> = built.events().iter().skip(first_built).collect();
        assert_eq!(
            happened(&live_log),
            happened(&built_log),
            "what happened on day {day}"
        );
    }
}

/// Gives a built world the scenario's commands and orders of `day` as the
/// scenario gives them, and resolves the day, as `Scenario::next_day` does.
fn play_scripted_day(world: &mut World, script: &Scenario, day: u32) {
    for (side, command) in script.scripted_commands().get(&day).into_iter().flatten() {
        world.submit_commands(*side, vec![command.clone()]);
    }
    for (side, request) in script.scripted_orders().get(&day).into_iter().flatten() {
        world.submit_orders(*side, vec![request.clone()]);
    }
    let inside = script
        .scripted_orders_inside_day()
        .get(&day)
        .cloned()
        .unwrap_or_default();
    if inside.is_empty() {
        world.resolve_day().expect("a day");
        return;
    }
    world
        .open_day(Staffs::none(), &mut |_, _| {})
        .expect("the day opens");
    for pulse in sim_core::time::Pulse::ALL {
        for (at, side, request) in &inside {
            if *at == pulse {
                world
                    .submit_orders_inside_day(*side, vec![request.clone()])
                    .expect("an order inside the day");
            }
        }
        world.resolve_next_pulse(&mut |_, _| {}).expect("a pulse");
    }
    world.close_day(&mut |_, _| {}).expect("the day closes");
}

// ---------------------------------------------------------------------------
// 3. No peek.
// ---------------------------------------------------------------------------

fn scenario_68_to_day_five(supply_t: u32) -> World {
    let path = common::data_dir().join("scenarios/68_ashore.toml");
    let text = std::fs::read_to_string(&path).expect("the scenario file reads");
    let text = common::replace_once(
        &text,
        "name = \"Placeholder airfield B\"\nside = \"japanese\"\nq = 4\nr = 0\nsupply_t = 200\n",
        &format!(
            "name = \"Placeholder airfield B\"\nside = \"japanese\"\nq = 4\nr = 0\nsupply_t = \
             {supply_t}\n"
        ),
    );
    let mut game = common::from_text(&text, &path).expect("the scenario loads");
    let held = staffs();
    for _ in 1..=5 {
        game.next_day_watched(as_staffs(&held), &mut |_, _| {})
            .expect("a day")
            .expect("a day exists");
    }
    game.world
}

#[test]
fn what_the_allied_side_cannot_see_gives_it_equal_halves() {
    let base = scenario_68_to_day_five(200);
    let more = scenario_68_to_day_five(300);
    assert_eq!(half(&base, Side::Allied), half(&more, Side::Allied));

    let japanese = half(&base, Side::Japanese);
    let mut other = half(&more, Side::Japanese);
    let b = BaseId::new(2);
    assert_ne!(japanese.bases[&b].supply_t, other.bases[&b].supply_t);
    other.bases.get_mut(&b).expect("base 2").supply_t = japanese.bases[&b].supply_t;
    assert_eq!(
        japanese, other,
        "the Japanese halves differ in base 2's supply alone"
    );
}

#[test]
fn the_full_slice_allied_half_holds_only_the_orders_it_refers_to() {
    let (live, _) = full_slice_on_day_three();
    let allied = half(&live.world, Side::Allied);
    let referred: BTreeSet<u32> = cited_by_half(&allied)
        .into_iter()
        .filter_map(|(_, held)| match held {
            Cited::Order(order) => Some(order.umpire_raw(common::umpire_key())),
            Cited::Event(_) => None,
        })
        .collect();
    assert_eq!(allied.orders.len(), referred.len());
    let count = u32::try_from(referred.len()).expect("a count");
    assert!(referred.iter().all(|place| (1..=count).contains(place)));
    // The Japanese side has given orders, and none of them is counted.
    assert!(live
        .world
        .orders()
        .any(|order| order.side == Side::Japanese));
    assert!(allied.orders.iter().all(|order| order.side == Side::Allied));
}

// ---------------------------------------------------------------------------
// 4. What it refuses.
// ---------------------------------------------------------------------------

#[test]
fn no_half_while_a_day_is_open() {
    let mut live = scenario_one_on_day_one();
    live.world
        .open_day(Staffs::none(), &mut |_, _| {})
        .expect("day 2 opens");
    live.world
        .resolve_next_pulse(&mut |_, _| {})
        .expect("the night of day 2");
    refused(
        live.world.side_view(Side::Allied).own_half(),
        "a picture is of an evening",
    );
}

#[test]
fn no_half_at_a_view_cut_to_a_seat() {
    let mut live = scenario("63_a_superiors_knowledge.toml");
    live.next_day().expect("day 1").expect("day 1 exists");
    let view = live.world.side_view(Side::Allied);
    let seat = view
        .at_seat(CommandRef::TaskForce(TaskForceId::new(1)))
        .expect("a seat at task force 1");
    assert_eq!(seat.next_task_force_id(), TaskForceId::new(0));
    assert_eq!(seat.next_air_group_id(), sim_core::ids::AirGroupId::new(0));
    refused(seat.own_half(), "only a whole side has a half");
    // The view of the whole side gives one.
    assert!(view.own_half().is_ok());
}

/// Beyond the hand calculation: a view whose units are as the area command
/// holds them, while reports travel, is not the side's own state either.
#[test]
fn no_half_from_a_view_of_units_as_a_command_holds_them() {
    let mut live = scenario("63_a_superiors_knowledge.toml");
    live.next_day().expect("day 1").expect("day 1 exists");
    let view = live.world.side_view(Side::Allied);
    let known = view.known_to(CommandRef::Area);
    let held = known.view();
    assert!(held
        .held_as_of(CommandRef::TaskForce(TaskForceId::new(1)))
        .is_some());
    refused(held.own_half(), "only a whole side has a half");
}

#[test]
fn two_halves_of_one_side_are_refused() {
    let live = scenario_one_on_day_one();
    refused(
        WorldPicture::from_halves(
            half(&live.world, Side::Allied),
            half(&live.world, Side::Allied),
        ),
        "two halves of one side",
    );
}

#[test]
fn halves_of_different_days_are_refused() {
    let mut live = scenario_one_on_day_one();
    let allied = half(&live.world, Side::Allied);
    live.next_day().expect("day 2").expect("day 2 exists");
    let japanese = half(&live.world, Side::Japanese);
    assert_eq!((allied.day, japanese.day), (1, 2));
    refused(
        WorldPicture::from_halves(allied, japanese),
        "halves of different days",
    );
}

#[test]
fn two_units_of_one_number_are_refused() {
    let live = scenario_one_on_day_one();
    let allied = half(&live.world, Side::Allied);
    let mut japanese = half(&live.world, Side::Japanese);
    let mut ship = allied.ships[&CRUISER].clone();
    ship.side = Side::Japanese;
    japanese.ships.insert(CRUISER, ship);
    refused(
        WorldPicture::from_halves(allied, japanese),
        "two units of one number",
    );
}

// ---------------------------------------------------------------------------
// Properties.
// ---------------------------------------------------------------------------

#[test]
fn a_half_taken_twice_is_the_same_half() {
    let (live, _) = full_slice_on_day_three();
    for side in Side::ALL {
        assert_eq!(half(&live.world, side), half(&live.world, side), "{side:?}");
    }
}

#[test]
fn nothing_of_the_live_game_is_in_a_half() {
    let live = scenario_one_on_day_one();
    for side in Side::ALL {
        nothing_of_the_live_game(&half(&live.world, side));
    }
}

/// A Japanese half of day 1 made in the test: one base and one force of
/// one ship, its route citing the order at its place 1.
fn japanese_by_hand(allied: &PictureHalf) -> PictureHalf {
    let mut base = allied.bases[&BaseId::new(1)].clone();
    base.id = BaseId::new(3);
    base.name = "Placeholder Japanese base".to_string();
    base.side = Side::Japanese;
    base.hex = Hex::new(4, 0);
    let mut force = allied.task_forces[&TASK_FORCE].clone();
    force.id = TaskForceId::new(2);
    force.name = "Placeholder Japanese force".to_string();
    force.side = Side::Japanese;
    force.ships = BTreeSet::from([ShipId::new(3)]);
    force.route_order = Some(WorldPicture::order_at(1));
    force.speed_order = None;
    let mut ship = allied.ships[&DESTROYER].clone();
    ship.id = ShipId::new(3);
    ship.name = "Placeholder Japanese destroyer".to_string();
    ship.side = Side::Japanese;
    ship.location = sim_core::state::ShipLocation::TaskForce(TaskForceId::new(2));
    assert_eq!(ship.class, ShipClassId::new(2));
    PictureHalf {
        day: 1,
        side: Side::Japanese,
        ships: BTreeMap::from([(ship.id, ship)]),
        task_forces: BTreeMap::from([(force.id, force)]),
        bases: BTreeMap::from([(base.id, base)]),
        air_groups: BTreeMap::new(),
        land_units: BTreeMap::new(),
        formations: BTreeMap::new(),
        entering: BTreeMap::new(),
        replacements: Vec::new(),
        supply_lines: Vec::new(),
        men_replacement: Default::default(),
        submarines: BTreeMap::new(),
        arrivals: Vec::new(),
        orders: vec![PictureOrder {
            side: Side::Japanese,
            day_issued: 1,
            kind: OrderKind::TaskForce {
                target: TaskForceId::new(2),
                order: TaskForceOrder::route(vec![Hex::new(8, 3)]),
            },
            origin: Origin::PLAYER,
        }],
        command: CommandState::new(),
        belief: PictureBelief::default(),
        counters: PictureCounters::first(Side::Japanese),
        // Decision 0194, 11: the game's days are the same in both halves.
        game_days: allied.game_days,
    }
}

#[test]
fn a_half_made_by_hand_joins_as_a_copied_one_does() {
    let live = scenario_one_on_day_one();
    let allied = half(&live.world, Side::Allied);
    let japanese = japanese_by_hand(&allied);
    // Given in either order, the Allied orders come first.
    let picture = WorldPicture::from_halves(japanese, allied).expect("joined");
    let force = &picture.truth.task_forces[&TaskForceId::new(2)];
    assert_eq!(force.route_order, Some(WorldPicture::order_at(3)));
    assert_eq!(picture.orders.len(), 3);
    assert_eq!(picture.orders[2].side, Side::Japanese);
    let allied_force = &picture.truth.task_forces[&TASK_FORCE];
    assert_eq!(allied_force.route_order, Some(WorldPicture::order_at(1)));
    assert_eq!(allied_force.speed_order, Some(WorldPicture::order_at(2)));
    let built = World::from_picture(
        picture,
        live.world.ref_data().clone(),
        live.world.map().clone(),
        Stream {
            seed: 1942,
            domain: Domain { trial: 1, world: 1 },
        },
    )
    .expect("a world is built from it");
    assert_eq!(built.day(), 1);
}

/// Beyond the hand calculation, which names day 4 as the day of the carrier
/// battle: in the full slice the carrier forces meet on days 18 and 19, and
/// days 4 and 5 hold no fight at sea or in the air. So the round trip is
/// also made from every evening of the slice, played to its end on day 39,
/// through the carrier battle, the strikes on Lunga and the ground fights
/// of days 37 and 39. Everything that happens in each built world is the
/// live game's.
#[test]
fn the_full_slice_from_every_evening_plays_on_to_its_end_as_it_does() {
    for evening in 1..=38 {
        full_slice_round_trip(evening, 39);
    }
}
