//! Scenario 21: a convoy lands two land units and a little supply on an empty
//! shore, a third unit wastes on a shore with no base, and part of it is
//! carried across and lands as a new unit.
//!
//! Proves decision 0066 (land units, carried by sea and fed from a base)
//! against a hand calculation: men worked as cargo at a fixed weight, troop
//! capacity, a landing that forms a base, a unit divided, the nightly charge
//! for men, the unfed, and men lost taken from units in proportion.
//!
//! The data is placeholder data: see `data/scenarios/21_landing.toml`. The
//! test runs on the frozen step 1 figures (`data/reference/step1`), to which
//! `ground.toml` and the placeholder transport classes are added with their
//! first values.
//!
//! # Hand calculation
//!
//! Written by the architect before the code. The tests below assert these
//! numbers exactly. If the code disagrees, the code or the rule is wrong:
//! do not change a number here to make a test pass. A mismatch goes back to
//! the architect.
//!
//! ## Inputs
//!
//! Figures, `ground.toml`:
//!
//! - `handling_t_per_man` 0.25, `daily_supply_t_per_man` 0.005,
//!   `unfed_loss_share_per_day` 0.05.
//!
//! Map, `placeholder_landing.toml`: the strip of scenario 2 (deep sea, q from
//! -1 to 10, r from -1 to 1), with three coast hexes: (0,0) with port size 5,
//! (8,0) with port size 0 and (10,0) with port size 0. A size 0 port has no
//! berth, so every ship at (8,0) and (10,0) works by lighter at 20 t an hour.
//!
//! Classes, `ship_classes_placeholder_transports.csv`:
//!
//! - Class 10, Placeholder troop transport: the placeholder cargo ship
//!   (cruise 10 kn, 1000 t of fuel for 10000 nm, so 0.1 t a mile; cargo
//!   capacity 3000 t) with a troop capacity of 600 men.
//! - Class 11, Placeholder fast transport: the placeholder destroyer (cruise
//!   16 kn, 500 t of fuel for 4000 nm, so 0.125 t a mile; no cargo capacity,
//!   no guns, no torpedoes) with a troop capacity of 200 men.
//!
//! The Allied side:
//!
//! - Base 1 at (0,0), supply 1000 t. It is the only base at the start.
//! - Task force 1 at base 1: ships 1 and 2, class 10, 1000 t of fuel each.
//!   Ship 1 starts with 14 t of supply as cargo.
//! - Task force 2 at (10,0), stopped: ship 3, class 11, 500 t of fuel.
//! - Land unit 1, 600 men, aboard task force 1. Land unit 2, 400 men, aboard
//!   task force 1. Units are put aboard in unit order, each over the force's
//!   ships in ship order, and a ship carries one unit: unit 1 fills ship 1
//!   (600), and unit 2 goes into ship 2 (400).
//! - Land unit 3, 500 men, ashore at (10,0), where the side has no base.
//!
//! There is no Japanese unit, ship or base. The game is 7 days.
//!
//! Orders:
//!
//! - Day 1, task force 1: route to (8,0), speed cruise, unload.
//! - Day 1, task force 2: load, cargo none, unit 3.
//! - Day 3, task force 2: route to (8,0), speed cruise, unload.
//!
//! Clock: hour 0 is 18:00 on day 1. The night pulse is 12 hours and each
//! daylight pulse 6. Day d begins at hour 24 x (d - 1). In the night pulse
//! the men's supply is charged before movement, and loading is the last
//! phase of every pulse.
//!
//! In all: 1500 men at the start and 14 t of supply aboard.
//!
//! ## Task force 1: the passage
//!
//! 8 hexes, 320 nm, at 10 kn.
//!
//! | Day | Pulse     | Hours | Distance | Total  |
//! | --- | --------- | ----- | -------- | ------ |
//! | 1   | Night     | 12    | 120 nm   | 120 nm |
//! | 1   | Morning   | 6     | 60 nm    | 180 nm |
//! | 1   | Afternoon | 6     | 60 nm    | 240 nm |
//! | 2   | Night     | 8     | 80 nm    | 320 nm |
//!
//! It arrives at (8,0) at hour 32, 02:00 on day 2. Each ship burns 320 x 0.1
//! = 32 t and has 968 t left.
//!
//! ## Task force 1: the landing
//!
//! Day 2, night: 4 hours are left of the pulse. Each ship's work is 4 x 20 =
//! 80 t, which is 80 / 0.25 = 320 men.
//!
//! - Ship 1 lands 320 men of unit 1. The first of them forms base 2 at (8,0):
//!   Allied, the next free base number, no stock. 280 men stay aboard. No
//!   work is left, so no supply moves.
//! - Ship 2 lands 320 men of unit 2. 80 stay aboard.
//!
//! Day 2, morning: 6 hours. Each ship's work is 120 t.
//!
//! - Ship 1 lands its 280 men, which take 70 t. 50 t of work are left: the
//!   14 t of supply are landed. Base 2 holds 14 t.
//! - Ship 2 lands its 80 men, which take 20 t.
//!
//! Unloading is complete. The slowest ship is ship 1: 70 + 14 = 84 t of work
//! at 20 t an hour is 4.2 hours, so the hour is 36 + 4.2 = 40.2, 10:12 on day
//! 2. Ashore at (8,0): unit 1 with 600 men and unit 2 with 400.
//!
//! ## Task force 2: taking men off
//!
//! Day 1, night. The men's supply is charged first: unit 3 loses 25 men (see
//! below) and has 475 ashore. Then loading: the load order with a unit is
//! accepted with no base at (10,0) and loads men only. The force is stopped
//! for all 12 hours: 240 t of work, which is 960 men. Ship 3's troop capacity
//! is 200, so 200 men of unit 3 go aboard and 275 stay ashore. Loading is
//! complete: 200 x 0.25 = 50 t at 20 t an hour is 2.5 hours, hour 2.5, 20:30
//! on day 1.
//!
//! Day 2: the force stays, full. Nothing moves and nothing is recorded.
//!
//! Day 3, night: 2 hexes, 80 nm, at 16 kn is 5 hours. It arrives at (8,0) at
//! hour 48 + 5 = 53, 23:00 on day 3. Ship 3 burns 80 x 0.125 = 10 t and has
//! 490 t left. 7 hours are left of the pulse: 140 t of work, 560 men.
//!
//! Unit 3 has men ashore in another hex, 249 at (10,0), so the 200 men aboard
//! become land unit 4, the next free number, before any lands. Unit 4 lands
//! its 200 men, which take 50 t. Base 2 is there already, so no base is
//! formed. Unloading is complete at hour 53 + 2.5 = 55.5, 01:30 on day 3.
//!
//! ## Unit 3 at (10,0): no base
//!
//! Every man ashore is unfed every night. The loss is the unfed men times
//! 0.05, cut to a whole man. Nothing is paid and no payment is recorded.
//!
//! | Night of | Ashore before | Times 0.05 | Lost | Ashore after      |
//! | -------- | ------------- | ---------- | ---- | ----------------- |
//! | Day 1    | 500           | 25         | 25   | 475, then 275 (200 embark) |
//! | Day 2    | 275           | 13.75      | 13   | 262               |
//! | Day 3    | 262           | 13.1       | 13   | 249               |
//! | Day 4    | 249           | 12.45      | 12   | 237               |
//! | Day 5    | 237           | 11.85      | 11   | 226               |
//! | Day 6    | 226           | 11.3       | 11   | 215               |
//! | Day 7    | 215           | 10.75      | 10   | 205               |
//!
//! Unit 3 loses 95 men unfed. The 200 men aboard ship 3 on the nights of days
//! 2 and 3 do not eat and are not lost.
//!
//! ## The men at (8,0): base 2
//!
//! On the night of day 2 no man is ashore there when the charge is made (the
//! convoy arrives later in that pulse), so nothing happens. Unit 4 lands
//! after the charge of day 3, so it first eats on the night of day 4.
//!
//! | Night of | Men  | Asked | Stock before | Paid | Stock after | Fed  | Unfed | Lost |
//! | -------- | ---- | ----- | ------------ | ---- | ----------- | ---- | ----- | ---- |
//! | Day 3    | 1000 | 5 t   | 14 t         | 5 t  | 9 t         | 1000 | 0     | 0    |
//! | Day 4    | 1200 | 6 t   | 9 t          | 6 t  | 3 t         | 1200 | 0     | 0    |
//! | Day 5    | 1200 | 6 t   | 3 t          | 3 t  | 0 t         | 600  | 600   | 30   |
//! | Day 6    | 1170 | none  | 0 t          | none | 0 t         | 0    | 1170  | 58   |
//! | Day 7    | 1112 | none  | 0 t          | none | 0 t         | 0    | 1112  | 55   |
//!
//! Day 5: 3 / 0.005 = 600 fed. 600 x 0.05 = 30. Day 6: the base has no stock,
//! so nothing is asked, paid or recorded as paid; 1170 x 0.05 = 58.5, cut to
//! 58. Day 7: 1112 x 0.05 = 55.6, cut to 55.
//!
//! Men lost are taken from the units in the hex in proportion: unit i gives
//! the loss times its men over all the men, in whole numbers, cut; the men
//! still to take are taken one from each unit in unit order.
//!
//! | Night of | Loss | Unit 1          | Unit 2           | Unit 4          | Left over | After: 1, 2, 4 |
//! | -------- | ---- | --------------- | ---------------- | --------------- | --------- | -------------- |
//! | Day 5    | 30   | 30x600/1200 = 15 | 30x400/1200 = 10 | 30x200/1200 = 5 | 0         | 585, 390, 195  |
//! | Day 6    | 58   | 58x585/1170 = 29 | 58x390/1170 = 19.33, 19 | 58x195/1170 = 9.67, 9 | 1, from unit 1 | 555, 371, 186 |
//! | Day 7    | 55   | 55x555/1112 = 27.45, 27 | 55x371/1112 = 18.35, 18 | 55x186/1112 = 9.2, 9 | 1, from unit 1 | 527, 353, 177 |
//!
//! ## The end, after day 7
//!
//! | Unit | Ashore | Where  | Aboard | Lost unfed | Began with            |
//! | ---- | ------ | ------ | ------ | ---------- | --------------------- |
//! | 1    | 527    | (8,0)  | 0      | 73         | 600                   |
//! | 2    | 353    | (8,0)  | 0      | 47         | 400                   |
//! | 3    | 205    | (10,0) | 0      | 95         | 500, of which 200 became unit 4 |
//! | 4    | 177    | (8,0)  | 0      | 23         | 200, from unit 3      |
//!
//! 527 + 353 + 205 + 177 = 1262 men ashore, and 73 + 47 + 95 + 23 = 238 lost:
//! 1500 in all. Base 1 holds 1000 t and base 2 holds 0 t; the 14 t landed
//! were all eaten. Ships 1 and 2 hold 968 t of fuel and ship 3 holds 490 t.
//! No ship has men or cargo aboard.
//!
//! Men ashore at the end of each day, units 1, 2, 3, 4:
//!
//! - Day 1: 0, 0, 275, none. Day 2: 600, 400, 262, none.
//! - Day 3: 600, 400, 249, 200. Day 4: 600, 400, 237, 200.
//! - Day 5: 585, 390, 226, 195. Day 6: 555, 371, 215, 186.
//! - Day 7: 527, 353, 205, 177.
//!
//! # Properties
//!
//! Beside the exact numbers, the tests check what must always hold (decision
//! 0066, "The test"): men are conserved; more supply never loses more men;
//! men aboard never exceed a ship's troop capacity and a ship never holds men
//! of two units; a scenario with no land unit is unchanged; the enemy's view
//! is the same in two worlds that differ only in a land unit, its landing and
//! the base it formed.

mod common;

use std::collections::BTreeMap;
use std::path::PathBuf;

use sim_core::events::{Cause, Event, EventKind, SupplyUse};
use sim_core::fixed::Fixed;
use sim_core::hex::Hex;
use sim_core::ids::{BaseId, EntityRef, LandUnitId, ShipId, TaskForceId};
use sim_core::orders::{Cargo, OrderKind, TaskForceOrder};
use sim_core::state::{Side, TruthState};
use sim_core::time::Pulse;
use sim_core::SimConfig;
use sim_data::Scenario;

// ------------------------------------------------------------------ Helpers

fn path() -> PathBuf {
    common::data_dir().join("scenarios/21_landing.toml")
}

fn text() -> String {
    std::fs::read_to_string(path()).expect("the scenario file reads")
}

/// The live scenario file, on the frozen step 1 figures.
fn load() -> Scenario {
    common::load(&path()).expect("scenario 21 loads")
}

fn run() -> Scenario {
    let mut scenario = load();
    scenario.run_to_end().expect("the scenario runs");
    scenario
}

fn run_text(source: &str) -> Scenario {
    let mut scenario = common::from_text(source, &path()).expect("the variant loads");
    scenario.run_to_end().expect("the variant runs");
    scenario
}

fn fx(text: &str) -> Fixed {
    text.parse().expect("a decimal")
}

fn n(whole: i64) -> Fixed {
    Fixed::from_int(whole)
}

fn unit(id: u32) -> LandUnitId {
    LandUnitId::new(id)
}

fn ship(id: u32) -> ShipId {
    ShipId::new(id)
}

fn force(id: u32) -> TaskForceId {
    TaskForceId::new(id)
}

fn base(id: u32) -> BaseId {
    BaseId::new(id)
}

const SHORE: Hex = Hex::new(8, 0);
const BARE_SHORE: Hex = Hex::new(10, 0);

fn events_of(scenario: &Scenario) -> Vec<Event> {
    scenario.world.events().iter().cloned().collect()
}

/// The one event that satisfies `pick`; panics if there is not exactly one.
fn only(events: &[Event], pick: impl Fn(&EventKind) -> bool) -> &Event {
    let found: Vec<&Event> = events.iter().filter(|event| pick(&event.kind)).collect();
    assert_eq!(found.len(), 1, "exactly one such event, found {found:?}");
    found[0]
}

/// The unload order of task force 1 and the load order of task force 2.
fn order_of(scenario: &Scenario, target: u32, wanted: TaskForceOrder) -> sim_core::ids::OrderId {
    scenario
        .world
        .orders()
        .find(|order| {
            matches!(&order.kind, OrderKind::TaskForce { target: t, order }
                if *t == force(target) && *order == wanted)
        })
        .expect("the order was received")
        .id
}

/// Whether the event is one of the kinds decision 0066 added.
fn is_land_event(kind: &EventKind) -> bool {
    matches!(
        kind,
        EventKind::MenEmbarked { .. }
            | EventKind::MenLanded { .. }
            | EventKind::BaseFormed { .. }
            | EventKind::UnitDivided { .. }
            | EventKind::MenLostWithShip { .. }
            | EventKind::MenFed { .. }
            | EventKind::MenLostUnfed { .. }
            | EventKind::LandUnitDestroyed { .. }
            | EventKind::UnitLoadingComplete { .. }
            | EventKind::BaseSupplySpent {
                reason: SupplyUse::Men,
                ..
            }
    )
}

/// The factors of a breakdown as (name, value) pairs.
fn factors(event: &Event) -> Vec<(&'static str, Fixed)> {
    event
        .breakdown
        .as_ref()
        .expect("a breakdown")
        .factors
        .iter()
        .map(|factor| (factor.name, factor.value))
        .collect()
}

// ------------------------------------------------------------ The passage

#[test]
fn passage_task_force_1_arrives_at_hour_32_with_968_t_of_fuel_each() {
    let scenario = run();
    let events = events_of(&scenario);
    let arrived = only(
        &events,
        |kind| matches!(kind, EventKind::TaskForceArrived { task_force, .. } if *task_force == force(1)),
    );
    assert_eq!(
        arrived.kind,
        EventKind::TaskForceArrived {
            task_force: force(1),
            hex: SHORE,
            elapsed_hours: n(32),
        }
    );
    // 02:00 on day 2, in the night pulse.
    assert_eq!((arrived.day, arrived.pulse), (2, Some(Pulse::Night)));

    let truth = scenario.world.umpire();
    assert_eq!(truth.ships[&ship(1)].fuel_t, n(968));
    assert_eq!(truth.ships[&ship(2)].fuel_t, n(968));
}

#[test]
fn passage_task_force_2_arrives_at_hour_53_with_490_t_of_fuel() {
    let scenario = run();
    let events = events_of(&scenario);
    let arrived = only(
        &events,
        |kind| matches!(kind, EventKind::TaskForceArrived { task_force, .. } if *task_force == force(2)),
    );
    assert_eq!(
        arrived.kind,
        EventKind::TaskForceArrived {
            task_force: force(2),
            hex: SHORE,
            elapsed_hours: n(53),
        }
    );
    // 23:00 on day 3, in the night pulse.
    assert_eq!((arrived.day, arrived.pulse), (3, Some(Pulse::Night)));
    assert_eq!(scenario.world.umpire().ships[&ship(3)].fuel_t, n(490));
}

// -------------------------------------------- Task force 1: the landing

/// The landing events of task force 1, in log order, with their day and pulse.
fn landing_of_force_1(events: &[Event]) -> Vec<(u32, Option<Pulse>, EventKind)> {
    events
        .iter()
        .filter(|event| {
            event.day == 2
                && matches!(
                    &event.kind,
                    EventKind::BaseFormed { .. }
                        | EventKind::MenLanded { .. }
                        | EventKind::CargoUnloaded { .. }
                        | EventKind::UnloadingComplete { .. }
                )
        })
        .map(|event| (event.day, event.pulse, event.kind.clone()))
        .collect()
}

#[test]
fn landing_task_force_1_lands_men_and_supply_as_the_hand_calculation_says() {
    let scenario = run();
    let landed = |unit_id, ship_id, men, aboard, ashore| EventKind::MenLanded {
        unit: unit(unit_id),
        ship: ship(ship_id),
        task_force: force(1),
        hex: SHORE,
        men,
        aboard,
        ashore,
    };
    let expected = vec![
        // Night of day 2: base 2 is formed, each ship lands 320 men.
        (
            2,
            Some(Pulse::Night),
            EventKind::BaseFormed {
                base: base(2),
                side: Side::Allied,
                hex: SHORE,
                task_force: force(1),
            },
        ),
        (2, Some(Pulse::Night), landed(1, 1, 320, 280, 320)),
        (2, Some(Pulse::Night), landed(2, 2, 320, 80, 320)),
        // Morning of day 2: ship 1 lands 280 men and the 14 t of supply.
        (2, Some(Pulse::Morning), landed(1, 1, 280, 0, 600)),
        (
            2,
            Some(Pulse::Morning),
            EventKind::CargoUnloaded {
                ship: ship(1),
                task_force: force(1),
                base: base(2),
                cargo: Cargo::Supply,
                unloaded_t: n(14),
                aboard_t: n(0),
                base_stock_t: n(14),
            },
        ),
        (2, Some(Pulse::Morning), landed(2, 2, 80, 0, 400)),
        // Unloading is complete at hour 36 + 4.2.
        (
            2,
            Some(Pulse::Morning),
            EventKind::UnloadingComplete {
                task_force: force(1),
                base: base(2),
                elapsed_hours: fx("40.2"),
            },
        ),
    ];
    assert_eq!(landing_of_force_1(&events_of(&scenario)), expected);
    // 40.2 is exact: 40 hours and 12 minutes.
    assert_eq!(fx("40.2"), Fixed::from_raw(40_200_000));
}

#[test]
fn landing_task_force_1_events_name_the_order_the_arrival_and_the_base_formed() {
    let scenario = run();
    let events = events_of(&scenario);
    let unload = order_of(&scenario, 1, TaskForceOrder::Unload);
    let arrival = only(
        &events,
        |kind| matches!(kind, EventKind::TaskForceArrived { task_force, .. } if *task_force == force(1)),
    )
    .id;
    let formed = only(&events, |kind| matches!(kind, EventKind::BaseFormed { .. }));
    // The base is formed by the unload order on the arrival.
    assert_eq!(
        formed.causes,
        vec![Cause::Order(unload), Cause::Event(arrival)]
    );

    let landings: Vec<&Event> = events
        .iter()
        .filter(|event| matches!(&event.kind, EventKind::MenLanded { task_force, .. } if *task_force == force(1)))
        .collect();
    assert_eq!(landings.len(), 4);
    // Night: the two landings that come after the base was formed name it.
    for event in &landings[..2] {
        assert_eq!(
            event.causes,
            vec![
                Cause::Order(unload),
                Cause::Event(arrival),
                Cause::Event(formed.id)
            ]
        );
    }
    // Morning: the order and the arrival.
    for event in &landings[2..] {
        assert_eq!(
            event.causes,
            vec![Cause::Order(unload), Cause::Event(arrival)]
        );
    }

    // Unloading is complete because of the three landings of the morning.
    let complete = only(
        &events,
        |kind| matches!(kind, EventKind::UnloadingComplete { task_force, .. } if *task_force == force(1)),
    );
    let cargo = only(&events, |kind| {
        matches!(kind, EventKind::CargoUnloaded { .. })
    });
    assert_eq!(
        complete.causes,
        vec![
            Cause::Event(landings[2].id),
            Cause::Event(cargo.id),
            Cause::Event(landings[3].id)
        ]
    );
}

// ------------------------------------- Task force 2: embark, divide, land

#[test]
fn task_force_2_takes_200_men_aboard_on_day_1_and_records_nothing_on_day_2() {
    let scenario = run();
    let events = events_of(&scenario);
    let load = order_of(
        &scenario,
        2,
        TaskForceOrder::LoadUnit {
            unit: unit(3),
            cargo: None,
        },
    );
    let embarked = only(&events, |kind| {
        matches!(kind, EventKind::MenEmbarked { .. })
    });
    assert_eq!((embarked.day, embarked.pulse), (1, Some(Pulse::Night)));
    assert_eq!(
        embarked.kind,
        EventKind::MenEmbarked {
            unit: unit(3),
            ship: ship(3),
            task_force: force(2),
            hex: BARE_SHORE,
            men: 200,
            aboard: 200,
            ashore: 275,
        }
    );
    assert_eq!(embarked.causes, vec![Cause::Order(load)]);

    let complete = only(&events, |kind| {
        matches!(kind, EventKind::UnitLoadingComplete { .. })
    });
    assert_eq!((complete.day, complete.pulse), (1, Some(Pulse::Night)));
    assert_eq!(
        complete.kind,
        EventKind::UnitLoadingComplete {
            task_force: force(2),
            unit: unit(3),
            base: None,
            cargo: None,
            elapsed_hours: fx("2.5"),
        }
    );
    assert_eq!(complete.causes, vec![Cause::Event(embarked.id)]);

    // Day 2: the force stays, full. Nothing about it is recorded; the
    // order of day 3 is the next thing that names it.
    let on_day_2: Vec<&Event> = events
        .iter()
        .filter(|event| {
            event.day == 2
                && event
                    .kind
                    .entities()
                    .contains(&EntityRef::TaskForce(force(2)))
        })
        .collect();
    assert!(on_day_2.is_empty(), "{on_day_2:?}");
    // And no man is moved by task force 2 on days 1 and 2 but the 200.
    let moved_by_2 = events
        .iter()
        .filter(|event| {
            matches!(&event.kind, EventKind::MenEmbarked { task_force, .. }
                | EventKind::MenLanded { task_force, .. } if *task_force == force(2))
        })
        .filter(|event| event.day <= 2)
        .count();
    assert_eq!(moved_by_2, 1);
}

#[test]
fn task_force_2_divides_unit_3_into_unit_4_on_day_3_and_lands_it() {
    let scenario = run();
    let events = events_of(&scenario);
    let load_free_unload = order_of(&scenario, 2, TaskForceOrder::Unload);
    let arrival = only(
        &events,
        |kind| matches!(kind, EventKind::TaskForceArrived { task_force, .. } if *task_force == force(2)),
    )
    .id;

    let divided = only(&events, |kind| {
        matches!(kind, EventKind::UnitDivided { .. })
    });
    assert_eq!((divided.day, divided.pulse), (3, Some(Pulse::Night)));
    assert_eq!(
        divided.kind,
        EventKind::UnitDivided {
            from: unit(3),
            unit: unit(4),
            task_force: force(2),
            men: 200,
        }
    );
    assert_eq!(
        divided.causes,
        vec![Cause::Order(load_free_unload), Cause::Event(arrival)]
    );

    // The unit is divided before any of it lands, and lands at base 2, which
    // is there already: no second base is formed.
    let landed = only(
        &events,
        |kind| matches!(kind, EventKind::MenLanded { unit: u, .. } if *u == unit(4)),
    );
    assert!(divided.id < landed.id);
    assert_eq!(
        landed.kind,
        EventKind::MenLanded {
            unit: unit(4),
            ship: ship(3),
            task_force: force(2),
            hex: SHORE,
            men: 200,
            aboard: 0,
            ashore: 200,
        }
    );
    assert_eq!(
        landed.causes,
        vec![
            Cause::Order(load_free_unload),
            Cause::Event(arrival),
            Cause::Event(divided.id)
        ]
    );
    assert_eq!(
        events
            .iter()
            .filter(|event| matches!(event.kind, EventKind::BaseFormed { .. }))
            .count(),
        1
    );

    let complete = only(
        &events,
        |kind| matches!(kind, EventKind::UnloadingComplete { task_force, .. } if *task_force == force(2)),
    );
    assert_eq!(
        complete.kind,
        EventKind::UnloadingComplete {
            task_force: force(2),
            base: base(2),
            elapsed_hours: fx("55.5"),
        }
    );
    assert_eq!(complete.causes, vec![Cause::Event(landed.id)]);

    // The new unit is named for the one it came from.
    let truth = scenario.world.umpire();
    assert_eq!(
        truth.land_units[&unit(4)].name,
        "Placeholder battalion 3, detachment 4"
    );
    assert_eq!(truth.land_units[&unit(3)].name, "Placeholder battalion 3");
}

// ------------------------------------------------- The feeding of the men

/// One night of feeding at the shore with the base: day, men, asked, paid,
/// stock after, fed, unfed, lost.
type FeedRow = (u32, u32, Option<(Fixed, Fixed, Fixed)>, u32, u32, u32);

fn feeding_at(events: &[Event], hex: Hex) -> Vec<FeedRow> {
    let mut rows = Vec::new();
    for event in events {
        let EventKind::MenFed {
            hex: fed_hex,
            men,
            fed,
            unfed,
            ..
        } = &event.kind
        else {
            continue;
        };
        if *fed_hex != hex {
            continue;
        }
        // The charge that the feeding names, if any.
        let charge = event.causes.iter().find_map(|cause| match cause {
            Cause::Event(id) => match &events.iter().find(|e| e.id == *id)?.kind {
                EventKind::BaseSupplySpent {
                    reason: SupplyUse::Men,
                    asked_t,
                    paid_t,
                    left_t,
                    ..
                } => Some((*asked_t, *paid_t, *left_t)),
                _ => None,
            },
            Cause::Order(_) => None,
        });
        // The men lost that night, from the events that name this feeding.
        let lost: u32 = events
            .iter()
            .filter(|e| e.causes.contains(&Cause::Event(event.id)))
            .map(|e| match &e.kind {
                EventKind::MenLostUnfed { lost, .. } => *lost,
                _ => 0,
            })
            .sum();
        rows.push((event.day, *men, charge, *fed, *unfed, lost));
    }
    rows
}

#[test]
fn feeding_at_base_2_follows_the_table_row_by_row() {
    let events = events_of(&run());
    let expected: Vec<FeedRow> = vec![
        // Night of day 3: 1000 men ask 5 t, are paid 5 t, stock after 9 t.
        (3, 1000, Some((n(5), n(5), n(9))), 1000, 0, 0),
        (4, 1200, Some((n(6), n(6), n(3))), 1200, 0, 0),
        // Day 5: 3 / 0.005 = 600 fed, 600 x 0.05 = 30 lost.
        (5, 1200, Some((n(6), n(3), n(0))), 600, 600, 30),
        // Days 6 and 7: the base has no stock: nothing asked or paid.
        (6, 1170, None, 0, 1170, 58),
        (7, 1112, None, 0, 1112, 55),
    ];
    assert_eq!(feeding_at(&events, SHORE), expected);
}

#[test]
fn feeding_charges_for_men_only_where_something_was_paid() {
    let events = events_of(&run());
    let spent: Vec<(u32, Option<Pulse>, EventKind)> = events
        .iter()
        .filter(|event| {
            matches!(
                event.kind,
                EventKind::BaseSupplySpent {
                    reason: SupplyUse::Men,
                    ..
                }
            )
        })
        .map(|event| (event.day, event.pulse, event.kind.clone()))
        .collect();
    let charge = |day, men, asked, paid, left| {
        (
            day,
            Some(Pulse::Night),
            EventKind::BaseSupplySpent {
                base: base(2),
                reason: SupplyUse::Men,
                aircraft: 0,
                men,
                asked_t: n(asked),
                paid_t: n(paid),
                left_t: n(left),
            },
        )
    };
    // Three charges, on the nights of days 3, 4 and 5, all of base 2. None on
    // days 6 and 7, none of base 1 and none at (10,0), where there is no base.
    assert_eq!(
        spent,
        vec![
            charge(3, 1000, 5, 5, 9),
            charge(4, 1200, 6, 6, 3),
            charge(5, 1200, 6, 3, 0),
        ]
    );
    // The causes of a charge: none, it is the start of the night's feeding.
    for event in events.iter().filter(|e| {
        matches!(
            e.kind,
            EventKind::BaseSupplySpent {
                reason: SupplyUse::Men,
                ..
            }
        )
    }) {
        assert!(event.causes.is_empty());
    }
}

#[test]
fn feeding_nothing_is_eaten_at_the_shore_before_the_night_of_day_3() {
    // On the night of day 2 no man is ashore at (8,0) when the charge is made.
    let events = events_of(&run());
    assert!(!events
        .iter()
        .any(|e| matches!(&e.kind, EventKind::MenFed { hex, .. } if *hex == SHORE && e.day < 3)));
}

#[test]
fn unit_3_at_the_bare_shore_is_unfed_every_night_and_loses_95_men() {
    let events = events_of(&run());
    // (day, men before, lost, ashore after). Day 1 is 500 less 25 = 475, of
    // which 200 then embark; later days start from what is left.
    let expected: Vec<(u32, u32, u32, u32)> = vec![
        (1, 500, 25, 475),
        (2, 275, 13, 262),
        (3, 262, 13, 249),
        (4, 249, 12, 237),
        (5, 237, 11, 226),
        (6, 226, 11, 215),
        (7, 215, 10, 205),
    ];
    let fed: Vec<(u32, u32, u32, u32)> = events
        .iter()
        .filter_map(|event| match &event.kind {
            EventKind::MenFed {
                hex, men, unfed, ..
            } if *hex == BARE_SHORE => Some((event.day, *men, *unfed)),
            _ => None,
        })
        .map(|(day, men, unfed)| {
            assert_eq!(men, unfed, "every man is unfed on day {day}");
            (day, men, 0, 0)
        })
        .collect();
    let lost: Vec<(u32, u32, u32)> = events
        .iter()
        .filter_map(|event| match &event.kind {
            EventKind::MenLostUnfed {
                unit: u,
                hex,
                lost,
                ashore,
            } if *hex == BARE_SHORE => {
                assert_eq!(*u, unit(3));
                Some((event.day, *lost, *ashore))
            }
            _ => None,
        })
        .collect();
    let combined: Vec<(u32, u32, u32, u32)> = fed
        .iter()
        .zip(&lost)
        .map(|((day, men, _, _), (_, lost, ashore))| (*day, *men, *lost, *ashore))
        .collect();
    assert_eq!(fed.len(), 7);
    assert_eq!(combined, expected);
    assert_eq!(lost.iter().map(|row| row.1).sum::<u32>(), 95);
    // Nothing is paid and none of it is recorded: no base, no charge.
    let rows = feeding_at(&events, BARE_SHORE);
    assert_eq!(rows.len(), 7);
    for (_, _, charge, fed, unfed, _) in rows {
        assert_eq!((charge, fed), (None, 0));
        assert!(unfed > 0);
    }
}

#[test]
fn men_aboard_ship_3_do_not_eat_and_are_not_lost() {
    // On the nights of days 2 and 3 the 200 men aboard are not in the count:
    // 275 ashore on day 2, 262 on day 3.
    let events = events_of(&run());
    let men_on = |day| {
        events
            .iter()
            .find_map(|e| match &e.kind {
                EventKind::MenFed { hex, men, .. } if *hex == BARE_SHORE && e.day == day => {
                    Some(*men)
                }
                _ => None,
            })
            .expect("fed that night")
    };
    assert_eq!((men_on(2), men_on(3)), (275, 262));
}

#[test]
fn proportional_losses_at_base_2_follow_the_table() {
    let events = events_of(&run());
    // (day, unit, lost, ashore after), in the order the events come.
    let rows: Vec<(u32, u32, u32, u32)> = events
        .iter()
        .filter_map(|event| match &event.kind {
            EventKind::MenLostUnfed {
                unit: u,
                hex,
                lost,
                ashore,
            } if *hex == SHORE => Some((event.day, u.raw_for_test(), *lost, *ashore)),
            _ => None,
        })
        .collect();
    let expected = vec![
        // Day 5: 30 x 600/1200, 30 x 400/1200, 30 x 200/1200, nothing over.
        (5, 1, 15, 585),
        (5, 2, 10, 390),
        (5, 4, 5, 195),
        // Day 6: 29, 19 and 9 and one over, from unit 1.
        (6, 1, 30, 555),
        (6, 2, 19, 371),
        (6, 4, 9, 186),
        // Day 7: 27, 18 and 9 and one over, from unit 1.
        (7, 1, 28, 527),
        (7, 2, 18, 353),
        (7, 4, 9, 177),
    ];
    assert_eq!(rows, expected);
}

/// `LandUnitId` shows no number; the order of units tells which is which.
trait RawForTest {
    fn raw_for_test(&self) -> u32;
}

impl RawForTest for LandUnitId {
    fn raw_for_test(&self) -> u32 {
        (1..=9)
            .find(|id| unit(*id) == *self)
            .expect("a unit below 10")
    }
}

#[test]
fn proportional_losses_record_the_cut_part_and_the_man_left_over() {
    let events = events_of(&run());
    // (day, unit) -> (loss in the hex, the part cut, the part taken).
    let wanted: Vec<(u32, u32, i64, i64, i64)> = vec![
        (5, 1, 30, 15, 15),
        (5, 2, 30, 10, 10),
        (5, 4, 30, 5, 5),
        (6, 1, 58, 29, 30),
        (6, 2, 58, 19, 19),
        (6, 4, 58, 9, 9),
        (7, 1, 55, 27, 28),
        (7, 2, 55, 18, 18),
        (7, 4, 55, 9, 9),
    ];
    let mut seen = Vec::new();
    for event in &events {
        let EventKind::MenLostUnfed { unit: u, hex, .. } = &event.kind else {
            continue;
        };
        if *hex != SHORE {
            continue;
        }
        let breakdown = event.breakdown.as_ref().expect("a breakdown");
        let named = factors(event);
        let get = |name: &str| {
            named
                .iter()
                .find(|(factor, _)| *factor == name)
                .expect("the factor")
                .1
        };
        seen.push((
            event.day,
            u.raw_for_test(),
            get("loss_in_hex").trunc(),
            get("in_proportion").trunc(),
            breakdown.result.trunc(),
        ));
    }
    assert_eq!(seen, wanted);
}

// ------------------------------------------------------------- The end

/// Men lost unfed by each unit over the whole game.
fn lost_unfed(events: &[Event]) -> BTreeMap<LandUnitId, u32> {
    let mut lost = BTreeMap::new();
    for event in events {
        if let EventKind::MenLostUnfed {
            unit, lost: men, ..
        } = &event.kind
        {
            *lost.entry(*unit).or_insert(0) += men;
        }
    }
    lost
}

#[test]
fn the_end_table_after_day_7() {
    let scenario = run();
    let truth = scenario.world.umpire();
    let events = events_of(&scenario);
    let lost = lost_unfed(&events);

    // (ashore, where, aboard, lost unfed)
    let expected = [
        (1, 527, Some(SHORE), 73),
        (2, 353, Some(SHORE), 47),
        (3, 205, Some(BARE_SHORE), 95),
        (4, 177, Some(SHORE), 23),
    ];
    for (id, ashore, hex, lost_men) in expected {
        let land_unit = &truth.land_units[&unit(id)];
        assert_eq!(land_unit.men_ashore, ashore, "unit {id} ashore");
        assert_eq!(land_unit.hex, hex, "unit {id} where");
        assert_eq!(truth.men_aboard(unit(id)), 0, "unit {id} aboard");
        assert_eq!(lost[&unit(id)], lost_men, "unit {id} lost unfed");
    }
    assert_eq!(truth.land_units.len(), 4);
    assert_eq!(
        truth.land_units.values().map(|u| u.men_ashore).sum::<u32>(),
        1262
    );
    assert_eq!(lost.values().sum::<u32>(), 238);
    assert_eq!(1262 + 238, 1500);
    // No unit was destroyed.
    assert!(!events
        .iter()
        .any(|e| matches!(e.kind, EventKind::LandUnitDestroyed { .. })));
}

#[test]
fn the_end_bases_ships_and_fuel() {
    let scenario = run();
    let truth = scenario.world.umpire();
    assert_eq!(truth.bases.len(), 2);
    assert_eq!(truth.bases[&base(1)].supply_t, n(1000));
    // The 14 t landed were all eaten.
    assert_eq!(truth.bases[&base(2)].supply_t, n(0));
    assert_eq!(truth.bases[&base(2)].hex, SHORE);
    assert_eq!(truth.bases[&base(2)].side, Side::Allied);
    // It was formed in play and only its own side knows of it.
    assert!(!truth.bases[&base(2)].on_map);
    assert!(truth.bases[&base(1)].on_map);

    assert_eq!(truth.ships[&ship(1)].fuel_t, n(968));
    assert_eq!(truth.ships[&ship(2)].fuel_t, n(968));
    assert_eq!(truth.ships[&ship(3)].fuel_t, n(490));
    for ship in truth.ships.values() {
        assert_eq!(ship.troops, None, "{} has men aboard", ship.name);
        assert_eq!(ship.supply_t, n(0), "{} has supply aboard", ship.name);
        assert_eq!(ship.cargo_fuel_t, n(0), "{} has fuel cargo", ship.name);
    }
}

#[test]
fn men_ashore_and_aboard_at_the_end_of_each_day() {
    let mut scenario = load();
    // Units 1, 2, 3, 4 ashore; None for a unit that does not exist yet.
    let ashore: [[Option<u32>; 4]; 7] = [
        [Some(0), Some(0), Some(275), None],
        [Some(600), Some(400), Some(262), None],
        [Some(600), Some(400), Some(249), Some(200)],
        [Some(600), Some(400), Some(237), Some(200)],
        [Some(585), Some(390), Some(226), Some(195)],
        [Some(555), Some(371), Some(215), Some(186)],
        [Some(527), Some(353), Some(205), Some(177)],
    ];
    // Aboard: the 1000 men sail on day 1; the 200 sit in ship 3 until day 3.
    let aboard: [[u32; 4]; 7] = [
        [600, 400, 200, 0],
        [0, 0, 200, 0],
        [0, 0, 0, 0],
        [0, 0, 0, 0],
        [0, 0, 0, 0],
        [0, 0, 0, 0],
        [0, 0, 0, 0],
    ];
    for day in 0..7 {
        scenario.next_day().expect("a day").expect("a day to run");
        let truth = scenario.world.umpire();
        let seen: Vec<Option<u32>> = (1..=4)
            .map(|id| truth.land_units.get(&unit(id)).map(|u| u.men_ashore))
            .collect();
        assert_eq!(
            seen,
            ashore[day].to_vec(),
            "ashore at the end of day {}",
            day + 1
        );
        let held: Vec<u32> = (1..=4).map(|id| truth.men_aboard(unit(id))).collect();
        assert_eq!(
            held,
            aboard[day].to_vec(),
            "aboard at the end of day {}",
            day + 1
        );
    }
}

// ----------------------------------------------------------- Breakdowns

#[test]
fn breakdown_the_men_a_ship_can_move_names_its_factors() {
    let events = events_of(&run());
    // Embarking: the work of 12 hours at 20 t an hour is 240 t, 960 men; the
    // berths (troop capacity) cut it to 200.
    let embarked = only(&events, |k| matches!(k, EventKind::MenEmbarked { .. }));
    assert_eq!(
        factors(embarked),
        vec![
            ("hours_worked", n(12)),
            ("on_berth", n(0)),
            ("rate_t_per_h", n(20)),
            ("capacity_by_time_t", n(240)),
            ("handling_t_per_man", fx("0.25")),
            ("men_by_time", n(960)),
            ("berths", n(200)),
        ]
    );
    assert_eq!(embarked.breakdown.as_ref().expect("bd").result, n(200));

    // Landing, night of day 2, ship 1: 4 hours, 80 t, 320 men of 600 aboard.
    let landed: Vec<&Event> = events
        .iter()
        .filter(|e| matches!(e.kind, EventKind::MenLanded { .. }))
        .collect();
    assert_eq!(
        factors(landed[0]),
        vec![
            ("hours_worked", n(4)),
            ("on_berth", n(0)),
            ("rate_t_per_h", n(20)),
            ("capacity_by_time_t", n(80)),
            ("handling_t_per_man", fx("0.25")),
            ("men_by_time", n(320)),
            ("aboard", n(600)),
        ]
    );
    assert_eq!(landed[0].breakdown.as_ref().expect("bd").result, n(320));
    // Morning, ship 1: 6 hours, 120 t, 480 men by time, 280 aboard.
    assert_eq!(
        factors(landed[2]),
        vec![
            ("hours_worked", n(6)),
            ("on_berth", n(0)),
            ("rate_t_per_h", n(20)),
            ("capacity_by_time_t", n(120)),
            ("handling_t_per_man", fx("0.25")),
            ("men_by_time", n(480)),
            ("aboard", n(280)),
        ]
    );
    assert_eq!(landed[2].breakdown.as_ref().expect("bd").result, n(280));

    // The supply landed after the men: 120 t of work, 70 t of men, 14 t of
    // supply.
    let cargo = only(&events, |k| matches!(k, EventKind::CargoUnloaded { .. }));
    assert_eq!(
        factors(cargo),
        vec![
            ("hours_worked", n(6)),
            ("on_berth", n(0)),
            ("rate_t_per_h", n(20)),
            ("capacity_by_time_t", n(120)),
            ("men_t", n(70)),
            ("supply_aboard_t", n(14)),
        ]
    );
}

#[test]
fn breakdown_the_charge_men_fed_and_the_loss_name_their_factors() {
    let events = events_of(&run());

    // The charge, night of day 5: 1200 men at 0.005 t asks 6 t.
    let charge = events
        .iter()
        .find(|e| {
            e.day == 5
                && matches!(
                    e.kind,
                    EventKind::BaseSupplySpent {
                        reason: SupplyUse::Men,
                        ..
                    }
                )
        })
        .expect("the charge");
    assert_eq!(
        factors(charge),
        vec![
            ("men_ashore", n(1200)),
            ("daily_supply_t_per_man", fx("0.005"))
        ]
    );
    assert_eq!(charge.breakdown.as_ref().expect("bd").result, n(6));

    // Men fed, night of day 5: 3 t paid over 0.005 is 600 fed.
    let fed = events
        .iter()
        .find(|e| e.day == 5 && matches!(&e.kind, EventKind::MenFed { hex, .. } if *hex == SHORE))
        .expect("the feeding");
    assert_eq!(
        factors(fed),
        vec![
            ("men_ashore", n(1200)),
            ("paid_t", n(3)),
            ("daily_supply_t_per_man", fx("0.005")),
        ]
    );
    assert_eq!(fed.breakdown.as_ref().expect("bd").result, n(600));

    // The feeding with nothing paid: the same factors, paid 0.
    let unpaid = events
        .iter()
        .find(|e| e.day == 6 && matches!(&e.kind, EventKind::MenFed { hex, .. } if *hex == SHORE))
        .expect("the feeding");
    assert_eq!(
        factors(unpaid),
        vec![
            ("men_ashore", n(1170)),
            ("paid_t", n(0)),
            ("daily_supply_t_per_man", fx("0.005")),
        ]
    );
    assert_eq!(unpaid.breakdown.as_ref().expect("bd").result, n(0));

    // The loss and unit 1's part of it, night of day 6.
    let part = events
        .iter()
        .find(|e| {
            e.day == 6
                && matches!(&e.kind, EventKind::MenLostUnfed { unit: u, hex, .. }
                    if *u == unit(1) && *hex == SHORE)
        })
        .expect("the part");
    assert_eq!(
        factors(part),
        vec![
            ("unfed_in_hex", n(1170)),
            ("unfed_loss_share_per_day", fx("0.05")),
            ("loss_in_hex", n(58)),
            ("unit_men_ashore", n(585)),
            ("men_ashore_in_hex", n(1170)),
            ("in_proportion", n(29)),
        ]
    );
    assert_eq!(part.breakdown.as_ref().expect("bd").result, n(30));
}

#[test]
fn breakdown_the_unloading_complete_events_name_the_hours() {
    let events = events_of(&run());
    let complete: Vec<&Event> = events
        .iter()
        .filter(|e| matches!(e.kind, EventKind::UnloadingComplete { .. }))
        .collect();
    assert_eq!(complete.len(), 2);
    assert_eq!(
        factors(complete[0]),
        vec![("work_start_h", n(36)), ("longest_ship_hours", fx("4.2"))]
    );
    assert_eq!(
        factors(complete[1]),
        vec![("work_start_h", n(53)), ("longest_ship_hours", fx("2.5"))]
    );
    let loading = only(&events, |k| {
        matches!(k, EventKind::UnitLoadingComplete { .. })
    });
    assert_eq!(
        factors(loading),
        vec![("work_start_h", n(0)), ("longest_ship_hours", fx("2.5"))]
    );
}

#[test]
fn breakdowns_off_gives_the_same_hash_every_day_and_the_same_events() {
    let mut with = load();
    let mut without = load();
    without.world.set_config(SimConfig {
        record_breakdowns: false,
    });
    with.run_to_end().expect("runs");
    without.run_to_end().expect("runs");
    assert_eq!(with.world.day_hashes().len(), 7);
    assert_eq!(with.world.day_hashes(), without.world.day_hashes());
    // Events and their causes are the same; only the breakdowns are gone.
    let strip = |scenario: &Scenario| -> Vec<(EventKind, Vec<Cause>)> {
        scenario
            .world
            .events()
            .iter()
            .map(|e| (e.kind.clone(), e.causes.clone()))
            .collect()
    };
    assert_eq!(strip(&with), strip(&without));
    assert!(without.world.events().iter().all(|e| e.breakdown.is_none()));
    assert!(with
        .world
        .events()
        .iter()
        .any(|e| matches!(e.kind, EventKind::MenFed { .. }) && e.breakdown.is_some()));
}

// ---------------------------------------------------------- Properties

/// The day-end truth and the log of a scenario, one entry for each day, and
/// the men each unit began with.
struct Days {
    began: BTreeMap<LandUnitId, u32>,
    ends: Vec<(TruthState, Vec<Event>)>,
}

fn days_of(source: &str) -> Days {
    let mut scenario = common::from_text(source, &path()).expect("the variant loads");
    let start = scenario.world.umpire();
    let began = start
        .land_units
        .values()
        .map(|u| (u.id, u.men_ashore + start.men_aboard(u.id)))
        .collect();
    let mut ends = Vec::new();
    while scenario.next_day().expect("a day").is_some() {
        ends.push((scenario.world.umpire().clone(), events_of(&scenario)));
    }
    Days { began, ends }
}

/// The unit a divided unit came from, followed back to the first.
fn root_of(events: &[Event], unit: LandUnitId) -> LandUnitId {
    let mut root = unit;
    while let Some(from) = events.iter().find_map(|e| match &e.kind {
        EventKind::UnitDivided { from, unit, .. } if *unit == root => Some(*from),
        _ => None,
    }) {
        root = from;
    }
    root
}

fn assert_men_are_conserved(days: &Days, what: &str) {
    for (index, (truth, events)) in days.ends.iter().enumerate() {
        for (root, began) in &days.began {
            let mut total = 0;
            for land_unit in truth.land_units.values() {
                if root_of(events, land_unit.id) != *root {
                    continue;
                }
                total += land_unit.men_ashore + truth.men_aboard(land_unit.id);
                for event in events {
                    match &event.kind {
                        EventKind::MenLostUnfed { unit, lost, .. } if *unit == land_unit.id => {
                            total += lost;
                        }
                        EventKind::MenLostWithShip { unit, men, .. } if *unit == land_unit.id => {
                            total += men;
                        }
                        _ => {}
                    }
                }
            }
            assert_eq!(
                total,
                *began,
                "{what}: the men of the unit that began as {root:?} at the end of day {}",
                index + 1
            );
        }
    }
}

#[test]
fn property_men_are_conserved_on_every_day() {
    let days = days_of(&text());
    assert_eq!(days.began.len(), 3);
    assert_eq!(days.began.values().sum::<u32>(), 1500);
    assert_men_are_conserved(&days, "scenario 21");
}

/// Men aboard never exceed a ship's troop capacity; a ship never holds men
/// of two units. Checked in the truth at the end of every day and in the
/// log: no ship is given men of a unit while she holds another's.
fn assert_berths_hold(scenario_ref: &Scenario, days: &Days, what: &str) {
    let capacity_of = |ship: &sim_core::state::Ship| {
        scenario_ref.world.ref_data().ship_classes[&ship.class].troop_capacity_men
    };
    for (index, (truth, events)) in days.ends.iter().enumerate() {
        for ship in truth.ships.values() {
            if let Some(troops) = ship.troops {
                assert!(troops.men >= 1, "{what}: day {}", index + 1);
                assert!(
                    troops.men <= capacity_of(ship),
                    "{what}: {} holds {} men of {} at the end of day {}",
                    ship.name,
                    troops.men,
                    capacity_of(ship),
                    index + 1
                );
                assert!(truth.land_units.contains_key(&troops.unit));
            }
        }
        // The men of a unit aboard are the sum over the ships holding it.
        for land_unit in truth.land_units.values() {
            let held: u32 = truth
                .ships
                .values()
                .filter_map(|ship| ship.troops)
                .filter(|troops| troops.unit == land_unit.id)
                .map(|troops| troops.men)
                .sum();
            assert_eq!(truth.men_aboard(land_unit.id), held);
        }
        // The log, up to this day.
        let mut holding: BTreeMap<ShipId, (LandUnitId, u32)> = BTreeMap::new();
        for event in events {
            match &event.kind {
                EventKind::MenEmbarked {
                    unit, ship, aboard, ..
                } => {
                    if let Some((other, _)) = holding.get(ship) {
                        assert_eq!(other, unit, "{what}: a ship holds two units");
                    }
                    let class = truth.ships[ship].class;
                    let capacity =
                        scenario_ref.world.ref_data().ship_classes[&class].troop_capacity_men;
                    assert!(*aboard <= capacity, "{what}: over capacity");
                    holding.insert(*ship, (*unit, *aboard));
                }
                // A divided unit's men aboard the force become the new unit.
                EventKind::UnitDivided { from, unit, .. } => {
                    for held in holding.values_mut() {
                        if held.0 == *from {
                            held.0 = *unit;
                        }
                    }
                }
                EventKind::MenLanded {
                    unit, ship, aboard, ..
                } => {
                    assert_eq!(
                        holding.get(ship).map(|h| h.0).or(Some(*unit)),
                        Some(*unit),
                        "{what}: landed men of a unit the ship did not hold"
                    );
                    if *aboard == 0 {
                        holding.remove(ship);
                    } else {
                        holding.insert(*ship, (*unit, *aboard));
                    }
                }
                _ => {}
            }
        }
    }
}

#[test]
fn property_men_aboard_never_exceed_troop_capacity_nor_mix_units() {
    let scenario = load();
    let days = days_of(&text());
    assert_berths_hold(&scenario, &days, "scenario 21");
    // Ship 1 held the 600 of unit 1 exactly, at its class's capacity.
    let truth_day_0 = scenario.world.umpire();
    assert_eq!(
        truth_day_0.ships[&ship(1)].troops.map(|t| (t.unit, t.men)),
        Some((unit(1), 600))
    );
    assert_eq!(
        truth_day_0.ships[&ship(2)].troops.map(|t| (t.unit, t.men)),
        Some((unit(2), 400))
    );
    assert_eq!(truth_day_0.ships[&ship(3)].troops, None);
}

/// The scenario with ship 1 starting with `supply` t of supply.
fn with_supply(supply: u32) -> String {
    common::replace_once(
        &text(),
        "fuel_t = 1000\nsupply_t = 14\ntask_force = 1",
        &format!("fuel_t = 1000\nsupply_t = {supply}\ntask_force = 1"),
    )
}

/// The men ashore of units 1 to 4 at the end of the game and the total.
fn end_men(source: &str) -> (Vec<u32>, u32) {
    let scenario = run_text(source);
    let truth = scenario.world.umpire();
    let per_unit: Vec<u32> = (1..=4)
        .map(|id| {
            truth
                .land_units
                .get(&unit(id))
                .map_or(0, |u| u.men_ashore + truth.men_aboard(unit(id)))
        })
        .collect();
    let total = per_unit.iter().sum();
    (per_unit, total)
}

#[test]
fn property_more_supply_landed_never_ends_with_fewer_men() {
    // The tested value of the scenario itself is 14 t.
    assert_eq!(end_men(&with_supply(14)), end_men(&text()));
    assert_eq!(end_men(&text()), (vec![527, 353, 205, 177], 1262));

    let supplies = [0u32, 7, 14, 20, 50, 100, 200, 400];
    let mut last: Option<(u32, Vec<u32>, u32)> = None;
    for supply in supplies {
        let source = with_supply(supply);
        let (per_unit, total) = end_men(&source);
        if let Some((before, units_before, total_before)) = &last {
            assert!(
                total >= *total_before,
                "{supply} t ends with {total} men, {before} t with {total_before}"
            );
            for (index, (now, then)) in per_unit.iter().zip(units_before).enumerate() {
                assert!(
                    now >= then,
                    "unit {}: {supply} t ends with {now} men, {before} t with {then}",
                    index + 1
                );
            }
        }
        // Men are conserved and the berths hold in every variant too.
        let days = days_of(&source);
        assert_men_are_conserved(&days, &format!("{supply} t"));
        assert_berths_hold(
            &common::from_text(&source, &path()).expect("loads"),
            &days,
            "variant",
        );
        last = Some((supply, per_unit, total));
    }
    // And the extremes are not the same: supply does save men.
    assert!(end_men(&with_supply(400)).1 > end_men(&with_supply(0)).1);
}

#[test]
fn property_a_scenario_with_no_land_unit_records_no_land_event() {
    // Scenario 2 on the frozen figures: no land unit, none of the new kinds.
    let mut two = common::load(&common::data_dir().join("scenarios/02_supply_run.toml"))
        .expect("scenario 2 loads");
    assert!(two.world.umpire().land_units.is_empty());
    assert!(two.world.ref_data().ground.is_none());
    two.run_to_end().expect("runs");
    assert!(two.world.umpire().land_units.is_empty());
    assert!(!two.world.events().iter().any(|e| is_land_event(&e.kind)));
    assert!(two
        .world
        .umpire()
        .ships
        .values()
        .all(|ship| ship.troops.is_none()));

    // Scenario 21 with its land units and the ground key taken out and the
    // load order made a plain unload.
    let mut source = text();
    let from = source.find("# Aboard task force 1").expect("the units");
    let to = source
        .find("# Day 1: the convoy steams")
        .expect("the orders");
    source.replace_range(from..to, "");
    let source = common::replace_once(&source, "ground = \"../reference/ground.toml\"\n", "");
    let source = common::replace_once(
        &source,
        "kind = \"load\"\nunit = 3\ncargo = \"none\"",
        "kind = \"unload\"",
    );
    let bare = run_text(&source);
    assert!(bare.world.umpire().land_units.is_empty());
    assert!(bare.world.ref_data().ground.is_none());
    let land: Vec<String> = bare
        .world
        .events()
        .iter()
        .filter(|e| is_land_event(&e.kind))
        .map(|e| e.kind.to_string())
        .collect();
    assert!(land.is_empty(), "{land:?}");
    // The base is not formed, since no man lands: base 1 is the only one.
    assert_eq!(bare.world.umpire().bases.len(), 1);
}

// --------------------------------------------------- The enemy's view

/// What the Japanese side sees, every day: its events as text, its bases, the
/// enemy bases on its map, its land units and its ships.
fn japanese_view(source: &str) -> Vec<String> {
    let mut scenario = common::from_text(source, &path()).expect("the variant loads");
    let mut seen = Vec::new();
    while scenario.next_day().expect("a day").is_some() {
        let view = scenario.world.side_view(Side::Japanese);
        let events: Vec<String> = view
            .events()
            .into_iter()
            .map(|e| {
                format!(
                    "{} D{} {:?} {:?} {:?} {:?}",
                    e.number, e.day, e.pulse, e.kind, e.order_number, e.causes
                )
            })
            .collect();
        let bases: Vec<String> = view.bases().map(|b| format!("{b:?}")).collect();
        let enemy_bases: Vec<String> = view.enemy_bases().map(|b| format!("{b:?}")).collect();
        let units: Vec<String> = view.land_units().map(|u| format!("{u:?}")).collect();
        let ships: Vec<String> = view.ships().map(|s| format!("{s:?}")).collect();
        let contacts = view.contacts().count();
        seen.push(format!(
            "day {}: events {events:?}\nbases {bases:?}\nenemy bases {enemy_bases:?}\nunits {units:?}\nships {ships:?}\ncontacts {contacts}",
            scenario.world.day()
        ));
    }
    seen
}

/// Scenario 21 with a Japanese base far from the landing.
fn with_japanese_base(source: &str) -> String {
    let mut out = source.to_string();
    out = common::replace_once(
        &out,
        "[[task_force]]\nid = 1\n",
        "[[base]]\nid = 3\nname = \"Placeholder Japanese base\"\nside = \"japanese\"\nq = 0\nr = 1\nsupply_t = 50\n\n[[task_force]]\nid = 1\n",
    );
    out
}

/// Scenario 21 without the Allied land units and without anything that moves
/// men: the land units are removed and the load order made a plain unload.
fn without_land_units(source: &str) -> String {
    let mut out = source.to_string();
    let from = out.find("# Aboard task force 1").expect("the units");
    let to = out.find("# Day 1: the convoy steams").expect("the orders");
    out.replace_range(from..to, "");
    common::replace_once(
        &out,
        "kind = \"load\"\nunit = 3\ncargo = \"none\"",
        "kind = \"unload\"",
    )
}

#[test]
fn property_the_enemy_sees_the_same_with_or_without_the_landing() {
    let with = with_japanese_base(&text());
    let without = without_land_units(&with);

    // The worlds really differ: base 2 is formed in one and not the other.
    let a = run_text(&with);
    let b = run_text(&without);
    assert_eq!(a.world.umpire().bases.len(), 3);
    assert_eq!(b.world.umpire().bases.len(), 2);
    assert!(!a.world.umpire().land_units.is_empty());
    assert!(b.world.umpire().land_units.is_empty());
    assert_ne!(a.world.day_hashes(), b.world.day_hashes());

    let seen_with = japanese_view(&with);
    let seen_without = japanese_view(&without);
    assert_eq!(seen_with.len(), 7);
    assert_eq!(seen_with, seen_without);
}

#[test]
fn property_the_japanese_view_holds_the_japanese_base_and_no_allied_unit() {
    let source = with_japanese_base(&text());
    let scenario = run_text(&source);
    let view = scenario.world.side_view(Side::Japanese);
    assert_eq!(view.bases().count(), 1);
    assert_eq!(view.land_units().count(), 0);
    assert_eq!(view.men_aboard(unit(1)), None);
    // It sees the Allied base of the scenario, on the map, and not base 2.
    let enemy: Vec<_> = view.enemy_bases().collect();
    assert_eq!(enemy.len(), 1);
    assert_eq!(enemy[0].id, base(1));
    assert_eq!(enemy[0].hex, Hex::new(0, 0));
    // None of the events it may see names a land unit or a man.
    for event in view.events() {
        let text = format!("{:?}", event.kind);
        assert!(!text.contains("Men"), "{text}");
        assert!(!text.contains("LandUnit"), "{text}");
        assert!(!text.contains("BaseFormed"), "{text}");
    }
    // The Allied side sees all of its own.
    let allied = scenario.world.side_view(Side::Allied);
    assert_eq!(allied.land_units().count(), 4);
    assert_eq!(allied.bases().count(), 2);
    assert_eq!(allied.enemy_bases().count(), 1);
}
