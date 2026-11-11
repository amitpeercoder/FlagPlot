//! Scenario 2b: a convoy loads fuel at Espiritu Santo and lands it at
//! Guadalcanal, and refuels there from what it has landed.
//!
//! It proves fuel as a cargo (decision 0046) together with refuelling in
//! port (decision 0045). The data is `data/scenarios/02b_fuel_run.toml`.
//! Everything in it is a placeholder. There is no chance in it: every seed
//! gives the same events.
//!
//! # Hand calculation
//!
//! ## The setting
//!
//! It is scenario 2 (`tests/scenario_02_supply_run.rs`) with fuel for
//! cargo, over four days.
//!
//! - Task force 1, Allied: cargo ships 1 and 2 and destroyer 3, stopped at
//!   Espiritu Santo.
//! - A cargo ship carries 3,000 t of cargo, cruises at 10 kn and burns 0.1 t
//!   a mile; its own fuel is 1,000 t, full. The destroyer carries no cargo
//!   and burns 0.125 t a mile, at 10 kn too; its own fuel is 500 t, full.
//! - Espiritu Santo, base 1 at (0,0): port size 5, four berths at 150 t an
//!   hour. It has 10,000 t of supply and 6,500 t of fuel.
//! - Guadalcanal, base 2 at (8,0): port size 1, one berth at 50 t an hour,
//!   and lighters at 20 t an hour. It has 500 t of supply and no fuel.
//! - The route is 8 hexes, 320 nm: 32 hours at 10 kn.
//! - Orders: on day 1, load fuel. On day 2, steam to (8,0) at cruising
//!   speed and unload.
//!
//! Fuel as cargo is not the fuel a ship steams on. Cargo moves at the end
//! of each pulse for the hours the force was stopped at the port in it.
//! Refuelling is at the start of a day, before the day's orders.
//!
//! ## Day 1: loading at Espiritu Santo
//!
//! At the start every ship's own fuel is full: nothing is refuelled.
//!
//! Two ships load and the port has four berths, so both work at 150 t an
//! hour, drawing on the fuel stock.
//!
//! | Pulse | Hours worked | Each ship loads | Each has aboard | Espiritu Santo's fuel |
//! | ----- | ------------ | --------------- | --------------- | --------------------- |
//! | Night | 12 | 1,800 t | 1,800 t | 2,900 t |
//! | Morning | 6 | 900 t | 2,700 t | 1,100 t |
//! | Afternoon | 2 | 300 t | 3,000 t | 500 t |
//!
//! Loading is complete 20 hours after the start, at 14:00 on day 1.
//! Espiritu Santo's supply is 10,000 t throughout: no supply is loaded.
//!
//! ## Days 2 and 3: the passage
//!
//! At the start of day 2 the force is stopped at Espiritu Santo and has
//! burned nothing: nothing is refuelled, and the base keeps its 500 t.
//!
//! It steams 240 nm on day 2 and is under way at the start of day 3, so it
//! does not refuel then. It arrives at Guadalcanal 8 hours into the night
//! pulse of day 3, at 02:00, after 320 nm. Each cargo ship has burned 32 t
//! and has 968 t of its own fuel; the destroyer has burned 40 t and has
//! 460 t.
//!
//! ## Day 3: unloading at Guadalcanal
//!
//! One berth. Ship 1 has it, at 50 t an hour, and ship 2 works by lighter
//! at 20 t an hour. The force was stopped for the last 4 hours of the night
//! pulse.
//!
//! | Pulse | Hours worked | Ship 1 lands | Ship 2 lands | Ship 1 aboard | Ship 2 aboard | Guadalcanal's fuel |
//! | ----- | ------------ | ------------ | ------------ | ------------- | ------------- | ------------------ |
//! | Night | 4 | 200 t | 80 t | 2,800 t | 2,920 t | 280 t |
//! | Morning | 6 | 300 t | 120 t | 2,500 t | 2,800 t | 700 t |
//! | Afternoon | 6 | 300 t | 120 t | 2,200 t | 2,680 t | 1,120 t |
//!
//! ## Day 4: the convoy refuels from what it landed
//!
//! At the start of day 4 the force is stopped at Guadalcanal, which has
//! 1,120 t of fuel.
//!
//! | Ship | Lacks | Takes | Own fuel | Guadalcanal has left |
//! | ---- | ----- | ----- | -------- | -------------------- |
//! | 1 | 32 t | 32 t | 1,000 t | 1,088 t |
//! | 2 | 32 t | 32 t | 1,000 t | 1,056 t |
//! | 3, the destroyer | 40 t | 40 t | 500 t | 1,016 t |
//!
//! Then unloading goes on:
//!
//! | Pulse | Hours worked | Ship 1 lands | Ship 2 lands | Ship 1 aboard | Ship 2 aboard | Guadalcanal's fuel |
//! | ----- | ------------ | ------------ | ------------ | ------------- | ------------- | ------------------ |
//! | Night | 12 | 600 t | 240 t | 1,600 t | 2,440 t | 1,856 t |
//! | Morning | 6 | 300 t | 120 t | 1,300 t | 2,320 t | 2,276 t |
//! | Afternoon | 6 | 300 t | 120 t | 1,000 t | 2,200 t | 2,696 t |
//!
//! ## At the end
//!
//! | | Fuel |
//! | - | ---- |
//! | Espiritu Santo's stock | 500 t |
//! | Guadalcanal's stock | 2,696 t |
//! | Cargo aboard ship 1 and ship 2 | 1,000 t and 2,200 t |
//! | Own fuel of ships 1, 2 and 3 | 1,000 t, 1,000 t and 500 t |
//!
//! All the fuel there was at the start, 6,500 t in Espiritu Santo's stock
//! and 2,500 t in the ships' own tanks, is 9,000 t. The ships burned 32 +
//! 32 + 40 = 104 t. What is left adds up to 500 + 2,696 + 3,200 + 2,500 =
//! 8,896 t.
//!
//! Supply does not move: 10,000 t at Espiritu Santo and 500 t at
//! Guadalcanal throughout, and none aboard.
//!
//! ## Variants
//!
//! | Variant | What changes |
//! | ------- | ------------ |
//! | The load order names no cargo | It loads supply: the run is scenario 2's first four days, event for event, apart from the refuelling of day 4, which finds no fuel at Guadalcanal and does nothing |
//! | Espiritu Santo has no fuel | The load order is accepted and loads nothing. The convoy arrives empty and lands nothing |
//! | Espiritu Santo has 5,000 t of fuel | The stock runs short in the morning of day 1. In the night each ship loads 1,800 t, leaving 1,400 t. In the morning ship 1 loads its 900 t and ship 2 the 500 t left. They sail with 2,700 t and 2,300 t |
//! | Each cargo ship starts with 1,000 t of supply aboard | A load of fuel fills what is left: 2,000 t each, complete after 12 + 1.333333 hours. At Guadalcanal supply is landed first: in the 4 hours of the night of day 3 ship 1 lands 200 t of supply and ship 2 80 t, and no fuel reaches the stock until a ship's supply is gone |
//! | Guadalcanal is Japanese | The unload order does nothing there and nothing is refuelled |
//!
//! ## What cannot happen
//!
//! - No ship carries more supply and fuel cargo together than its cargo
//!   capacity, or less than none of either.
//! - No ship burns its cargo: a ship's own fuel changes only by steaming
//!   and by refuelling.
//! - No fuel is made or lost: it is conserved as "At the end" has it.
//! - No cargo moves while the force is under way.
//!
//! # Properties
//!
//! - Fuel and supply are each conserved in every variant.
//! - More fuel at Espiritu Santo never lands less at Guadalcanal.
//! - Every seed gives the same events.
//! - Recording breakdowns changes neither the events nor the hashes.
//! - A scenario that loads no fuel has the hashes it had before this stage:
//!   no existing golden file changes.

// The test code is written by a developer from the hand calculation above.

use std::fs;
use std::path::{Path, PathBuf};

use sim_core::events::{Cause, Event, EventKind};
use sim_core::fixed::Fixed;
use sim_core::hex::Hex;
use sim_core::ids::{BaseId, EntityRef, ShipId, TaskForceId};
use sim_core::orders::{Cargo, OrderKind, TaskForceOrder};
use sim_core::state::{Side, TaskForceStatus};
use sim_core::time::{ClockTime, Pulse};
use sim_core::{SimConfig, World};
use sim_data::Scenario;

const TASK_FORCE: TaskForceId = TaskForceId::new(1);
const CARGO_1: ShipId = ShipId::new(1);
const CARGO_2: ShipId = ShipId::new(2);
const DESTROYER: ShipId = ShipId::new(3);
const SHIPS: [ShipId; 3] = [CARGO_1, CARGO_2, DESTROYER];
const SANTO: BaseId = BaseId::new(1);
const GUADALCANAL: BaseId = BaseId::new(2);

fn tons(value: i64) -> Fixed {
    Fixed::from_int(value)
}

fn data_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../data")
}

fn load() -> Scenario {
    Scenario::load(&data_dir().join("scenarios/02b_fuel_run.toml")).expect("scenario 2b loads")
}

/// Resolves the next day and returns its number and its events.
fn run_day(scenario: &mut Scenario) -> (u32, Vec<Event>) {
    let summary = scenario
        .next_day()
        .expect("the day resolves")
        .expect("the day exists");
    let events = scenario.world.events().from_index(summary.first_event);
    assert_eq!(events.len(), summary.event_count);
    (summary.day, events.to_vec())
}

/// Every day of the scenario: the events of the day and the world at its end.
struct Run {
    days: Vec<(u32, Vec<Event>)>,
    scenario: Scenario,
}

impl Run {
    fn events(&self) -> Vec<Event> {
        self.days
            .iter()
            .flat_map(|(_, events)| events.iter().cloned())
            .collect()
    }

    fn day(&self, day: u32) -> &[Event] {
        &self
            .days
            .iter()
            .find(|(number, _)| *number == day)
            .expect("the day was run")
            .1
    }
}

/// Runs a scenario to its end, calling `check` with the world after each day.
fn run_with(mut scenario: Scenario, mut check: impl FnMut(&World, u32)) -> Run {
    let mut days = Vec::new();
    while let Some(summary) = scenario.next_day().expect("the day resolves") {
        let events = scenario
            .world
            .events()
            .from_index(summary.first_event)
            .to_vec();
        check(&scenario.world, summary.day);
        days.push((summary.day, events));
    }
    Run { days, scenario }
}

fn run_to_end(scenario: Scenario) -> Run {
    run_with(scenario, |_, _| {})
}

fn own_fuel(world: &World, ship: ShipId) -> Fixed {
    world.umpire().ships[&ship].fuel_t
}

fn fuel_cargo(world: &World, ship: ShipId) -> Fixed {
    world.umpire().ships[&ship].cargo_fuel_t
}

fn supply_aboard(world: &World, ship: ShipId) -> Fixed {
    world.umpire().ships[&ship].supply_t
}

fn fuel_stock(world: &World, base: BaseId) -> Fixed {
    world.umpire().bases[&base].fuel_t
}

fn supply_stock(world: &World, base: BaseId) -> Fixed {
    world.umpire().bases[&base].supply_t
}

/// One cargo event as (pulse, ship, cargo, tons moved, aboard after, stock
/// after), loads and unloads alike.
type Move = (Pulse, ShipId, Cargo, Fixed, Fixed, Fixed);

fn moves(events: &[Event]) -> Vec<Move> {
    events
        .iter()
        .filter_map(|event| match &event.kind {
            EventKind::CargoLoaded {
                ship,
                cargo,
                loaded_t,
                aboard_t,
                base_stock_t,
                ..
            } => Some((
                event.pulse.expect("cargo has a pulse"),
                *ship,
                *cargo,
                *loaded_t,
                *aboard_t,
                *base_stock_t,
            )),
            EventKind::CargoUnloaded {
                ship,
                cargo,
                unloaded_t,
                aboard_t,
                base_stock_t,
                ..
            } => Some((
                event.pulse.expect("cargo has a pulse"),
                *ship,
                *cargo,
                *unloaded_t,
                *aboard_t,
                *base_stock_t,
            )),
            _ => None,
        })
        .collect()
}

fn fuel_moves(events: &[Event]) -> Vec<Move> {
    moves(events)
        .into_iter()
        .filter(|(_, _, cargo, ..)| *cargo == Cargo::Fuel)
        .collect()
}

/// The refuellings as (ship, tons taken, own fuel after, base fuel left).
fn refuellings(events: &[Event]) -> Vec<(ShipId, Fixed, Fixed, Fixed)> {
    events
        .iter()
        .filter_map(|event| match &event.kind {
            EventKind::ShipRefuelled {
                ship,
                taken_t,
                fuel_t,
                base_fuel_t,
                ..
            } => Some((*ship, *taken_t, *fuel_t, *base_fuel_t)),
            _ => None,
        })
        .collect()
}

fn burned(events: &[Event], ship: ShipId) -> Fixed {
    events
        .iter()
        .filter_map(|event| match &event.kind {
            EventKind::FuelBurned {
                ship: burned_ship,
                burned_t,
                ..
            } if *burned_ship == ship => Some(*burned_t),
            _ => None,
        })
        .fold(Fixed::ZERO, |sum, t| sum + t)
}

fn completions(events: &[Event]) -> Vec<(Option<Pulse>, Cargo, Fixed)> {
    events
        .iter()
        .filter_map(|event| match &event.kind {
            EventKind::LoadingComplete {
                cargo,
                elapsed_hours,
                ..
            } => Some((event.pulse, *cargo, *elapsed_hours)),
            _ => None,
        })
        .collect()
}

fn is_cargo_event(event: &Event) -> bool {
    matches!(
        event.kind,
        EventKind::CargoLoaded { .. }
            | EventKind::CargoUnloaded { .. }
            | EventKind::LoadingComplete { .. }
            | EventKind::UnloadingComplete { .. }
    )
}

/// A fuel cargo event as the tables write it: (pulse, ship, tons moved,
/// tons aboard after, the base's stock after).
fn fuel_row(pulse: Pulse, ship: ShipId, moved: i64, aboard: i64, stock: i64) -> Move {
    (
        pulse,
        ship,
        Cargo::Fuel,
        tons(moved),
        tons(aboard),
        tons(stock),
    )
}

/// Every ship's fuel as (own fuel, fuel cargo), for comparing worlds.
fn fuel_of_ships(world: &World) -> Vec<(Fixed, Fixed)> {
    SHIPS
        .iter()
        .map(|ship| (own_fuel(world, *ship), fuel_cargo(world, *ship)))
        .collect()
}

/// All the fuel in the world: the stocks, the cargo and the ships' own tanks.
fn total_fuel(world: &World) -> Fixed {
    let truth = world.umpire();
    let stocks = truth
        .bases
        .values()
        .fold(Fixed::ZERO, |sum, base| sum + base.fuel_t);
    let ships = truth.ships.values().fold(Fixed::ZERO, |sum, ship| {
        sum + ship.fuel_t + ship.cargo_fuel_t
    });
    stocks + ships
}

fn total_supply(world: &World) -> Fixed {
    let truth = world.umpire();
    let stocks = truth
        .bases
        .values()
        .fold(Fixed::ZERO, |sum, base| sum + base.supply_t);
    let ships = truth
        .ships
        .values()
        .fold(Fixed::ZERO, |sum, ship| sum + ship.supply_t);
    stocks + ships
}

#[test]
fn the_fuel_run_matches_the_hand_calculation() {
    let mut scenario = load();
    assert!(scenario.placeholder);
    assert_eq!(scenario.days, 4);
    assert_eq!(scenario.world.umpire().ships.len(), 3);

    // The setting.
    {
        let world = &scenario.world;
        assert_eq!(fuel_stock(world, SANTO), tons(6500));
        assert_eq!(fuel_stock(world, GUADALCANAL), tons(0));
        assert_eq!(supply_stock(world, SANTO), tons(10_000));
        assert_eq!(supply_stock(world, GUADALCANAL), tons(500));
        for ship in SHIPS {
            assert_eq!(fuel_cargo(world, ship), Fixed::ZERO);
            assert_eq!(supply_aboard(world, ship), Fixed::ZERO);
        }
        assert_eq!(own_fuel(world, CARGO_1), tons(1000));
        assert_eq!(own_fuel(world, CARGO_2), tons(1000));
        assert_eq!(own_fuel(world, DESTROYER), tons(500));
        assert_eq!(total_fuel(world), tons(9000));
        let classes = &world.ref_data().ship_classes;
        let class_of = |ship: ShipId| &classes[&world.umpire().ships[&ship].class];
        assert_eq!(class_of(CARGO_1).cargo_capacity_t, tons(3000));
        assert_eq!(class_of(DESTROYER).cargo_capacity_t, Fixed::ZERO);
    }
    let mut all_events: Vec<Event> = Vec::new();

    // Day 1: loading at Espiritu Santo. Nothing is refuelled: every ship's
    // own fuel is full.
    let (day, events) = run_day(&mut scenario);
    assert_eq!(day, 1);
    assert!(refuellings(&events).is_empty());
    assert_eq!(
        fuel_moves(&events),
        vec![
            fuel_row(Pulse::Night, CARGO_1, 1800, 1800, 4700),
            fuel_row(Pulse::Night, CARGO_2, 1800, 1800, 2900),
            fuel_row(Pulse::Morning, CARGO_1, 900, 2700, 2000),
            fuel_row(Pulse::Morning, CARGO_2, 900, 2700, 1100),
            fuel_row(Pulse::Afternoon, CARGO_1, 300, 3000, 800),
            fuel_row(Pulse::Afternoon, CARGO_2, 300, 3000, 500),
        ]
    );
    // Nothing but fuel is loaded or landed, and the destroyer loads nothing.
    assert_eq!(moves(&events), fuel_moves(&events));
    assert!(events.iter().all(|event| !matches!(
        event.kind,
        EventKind::CargoLoaded { ship, .. } if ship == DESTROYER
    )));
    let loading = completions(&events);
    assert_eq!(
        loading,
        vec![(Some(Pulse::Afternoon), Cargo::Fuel, tons(20))]
    );
    assert_eq!(ClockTime::from_elapsed(tons(20)).to_string(), "D1 14:00");
    assert!(events
        .iter()
        .all(|event| !matches!(event.kind, EventKind::TaskForceSteamed { .. })));
    {
        let world = &scenario.world;
        assert_eq!(fuel_stock(world, SANTO), tons(500));
        assert_eq!(supply_stock(world, SANTO), tons(10_000));
        assert_eq!(fuel_cargo(world, CARGO_1), tons(3000));
        assert_eq!(fuel_cargo(world, CARGO_2), tons(3000));
        assert_eq!(fuel_of_ships(world)[0].0, tons(1000));
        assert_eq!(own_fuel(world, DESTROYER), tons(500));
    }
    all_events.extend(events);

    // Day 2: the passage begins. The force is stopped at Espiritu Santo and
    // has burned nothing, so nothing is refuelled and the base keeps 500 t.
    let (day, events) = run_day(&mut scenario);
    assert_eq!(day, 2);
    assert!(refuellings(&events).is_empty());
    assert!(events.iter().all(|event| !is_cargo_event(event)));
    let distance = events
        .iter()
        .filter_map(|event| match &event.kind {
            EventKind::TaskForceSteamed { distance_nm, .. } => Some(*distance_nm),
            _ => None,
        })
        .fold(Fixed::ZERO, |sum, nm| sum + nm);
    assert_eq!(distance, tons(240));
    assert_eq!(fuel_stock(&scenario.world, SANTO), tons(500));
    assert_eq!(
        scenario.world.umpire().task_forces[&TASK_FORCE].status,
        TaskForceStatus::Underway
    );
    all_events.extend(events);

    // Day 3: under way at the start, so no refuelling. It arrives 8 hours
    // into the night pulse, at 02:00 and 56 hours in, after 320 nm.
    let (day, events) = run_day(&mut scenario);
    assert_eq!(day, 3);
    assert!(refuellings(&events).is_empty());
    {
        let world = &scenario.world;
        let force = &world.umpire().task_forces[&TASK_FORCE];
        assert_eq!(force.position(), Hex::new(8, 0));
        assert_eq!(force.arrived_at_hours, Some(tons(56)));
        assert_eq!(ClockTime::from_elapsed(tons(56)).to_string(), "D3 02:00");
        // Own fuel: each cargo ship burned 32 t, the destroyer 40 t.
        let mut so_far = all_events.clone();
        so_far.extend(events.iter().cloned());
        assert_eq!(burned(&so_far, CARGO_1), tons(32));
        assert_eq!(burned(&so_far, CARGO_2), tons(32));
        assert_eq!(burned(&so_far, DESTROYER), tons(40));
        assert_eq!(own_fuel(world, CARGO_1), tons(968));
        assert_eq!(own_fuel(world, CARGO_2), tons(968));
        assert_eq!(own_fuel(world, DESTROYER), tons(460));
        // The cargo is exactly what it was: nothing burned it.
        assert_eq!(world.umpire().bases[&SANTO].fuel_t, tons(500));
    }
    // Unloading: ship 1 on the one berth at 50 t an hour, ship 2 by lighter
    // at 20 t an hour, for 4, 6 and 6 hours.
    assert_eq!(
        moves(&events),
        vec![
            fuel_row(Pulse::Night, CARGO_1, 200, 2800, 200),
            fuel_row(Pulse::Night, CARGO_2, 80, 2920, 280),
            fuel_row(Pulse::Morning, CARGO_1, 300, 2500, 580),
            fuel_row(Pulse::Morning, CARGO_2, 120, 2800, 700),
            fuel_row(Pulse::Afternoon, CARGO_1, 300, 2200, 1000),
            fuel_row(Pulse::Afternoon, CARGO_2, 120, 2680, 1120),
        ]
    );
    {
        let world = &scenario.world;
        assert_eq!(fuel_stock(world, GUADALCANAL), tons(1120));
        assert_eq!(fuel_cargo(world, CARGO_1), tons(2200));
        assert_eq!(fuel_cargo(world, CARGO_2), tons(2680));
    }
    all_events.extend(events);

    // Day 4: the convoy refuels from what it landed, at the start of the
    // day, in ship order; then unloading goes on.
    let (day, events) = run_day(&mut scenario);
    assert_eq!(day, 4);
    assert_eq!(
        refuellings(&events),
        vec![
            (CARGO_1, tons(32), tons(1000), tons(1088)),
            (CARGO_2, tons(32), tons(1000), tons(1056)),
            (DESTROYER, tons(40), tons(500), tons(1016)),
        ]
    );
    // Refuelling comes before the day's cargo handling.
    let first_cargo = events
        .iter()
        .position(is_cargo_event)
        .expect("unloading goes on");
    let last_refuel = events
        .iter()
        .rposition(|event| matches!(event.kind, EventKind::ShipRefuelled { .. }))
        .expect("refuelled");
    assert!(last_refuel < first_cargo);
    assert_eq!(
        moves(&events),
        vec![
            fuel_row(Pulse::Night, CARGO_1, 600, 1600, 1616),
            fuel_row(Pulse::Night, CARGO_2, 240, 2440, 1856),
            fuel_row(Pulse::Morning, CARGO_1, 300, 1300, 2156),
            fuel_row(Pulse::Morning, CARGO_2, 120, 2320, 2276),
            fuel_row(Pulse::Afternoon, CARGO_1, 300, 1000, 2576),
            fuel_row(Pulse::Afternoon, CARGO_2, 120, 2200, 2696),
        ]
    );
    all_events.extend(events);

    // The end.
    {
        let world = &scenario.world;
        assert_eq!(fuel_stock(world, SANTO), tons(500));
        assert_eq!(fuel_stock(world, GUADALCANAL), tons(2696));
        assert_eq!(fuel_cargo(world, CARGO_1), tons(1000));
        assert_eq!(fuel_cargo(world, CARGO_2), tons(2200));
        assert_eq!(fuel_cargo(world, DESTROYER), Fixed::ZERO);
        assert_eq!(own_fuel(world, CARGO_1), tons(1000));
        assert_eq!(own_fuel(world, CARGO_2), tons(1000));
        assert_eq!(own_fuel(world, DESTROYER), tons(500));
        // Fuel: 9,000 t at the start; the ships burned 32 + 32 + 40 = 104 t;
        // 500 + 2,696 + 3,200 + 2,500 = 8,896 t is left.
        let burns: Fixed = SHIPS
            .iter()
            .fold(Fixed::ZERO, |sum, ship| sum + burned(&all_events, *ship));
        assert_eq!(burns, tons(104));
        assert_eq!(
            tons(500) + tons(2696) + tons(1000) + tons(2200) + tons(2500),
            tons(8896)
        );
        assert_eq!(total_fuel(world), tons(8896));
        assert_eq!(total_fuel(world) + burns, tons(9000));
        // Supply does not move.
        assert_eq!(supply_stock(world, SANTO), tons(10_000));
        assert_eq!(supply_stock(world, GUADALCANAL), tons(500));
        for ship in SHIPS {
            assert_eq!(supply_aboard(world, ship), Fixed::ZERO);
        }
    }
    // Unloading is not finished: fuel is still aboard.
    assert!(all_events
        .iter()
        .all(|event| !matches!(event.kind, EventKind::UnloadingComplete { .. })));
    assert!(scenario.next_day().expect("no error").is_none());
}

#[test]
fn fuel_and_supply_are_conserved_and_cargo_stays_within_limits_every_day() {
    let initial_fuel = tons(9000);
    let mut so_far: Vec<Event> = Vec::new();
    let mut seen = 0;
    let mut scenario = load();
    while let Some(summary) = scenario.next_day().expect("the day resolves") {
        so_far.extend(
            scenario
                .world
                .events()
                .from_index(summary.first_event)
                .to_vec(),
        );
        seen += 1;
        let world = &scenario.world;
        let burns = SHIPS
            .iter()
            .fold(Fixed::ZERO, |sum, ship| sum + burned(&so_far, *ship));
        assert_eq!(total_fuel(world) + burns, initial_fuel, "day {seen}");
        assert_eq!(total_supply(world), tons(10_500), "day {seen}");
        assert_limits(world, seen);
    }
    assert_eq!(seen, 4);
}

fn assert_limits(world: &World, day: u32) {
    let truth = world.umpire();
    for ship in truth.ships.values() {
        let capacity = world.ref_data().ship_classes[&ship.class].cargo_capacity_t;
        assert!(
            ship.supply_t >= Fixed::ZERO && ship.cargo_fuel_t >= Fixed::ZERO,
            "day {day}: {} holds a negative cargo",
            ship.id
        );
        assert!(
            ship.supply_t + ship.cargo_fuel_t <= capacity,
            "day {day}: {} holds {} t of supply and {} t of fuel, capacity {capacity} t",
            ship.id,
            ship.supply_t,
            ship.cargo_fuel_t
        );
        assert!(ship.fuel_t >= Fixed::ZERO);
    }
    for base in truth.bases.values() {
        assert!(base.supply_t >= Fixed::ZERO && base.fuel_t >= Fixed::ZERO);
    }
}

/// A ship's own fuel changes only by steaming and by refuelling: its end
/// state is what it started with, less what it burned, plus what it took. No
/// ship burns its cargo.
fn assert_own_fuel_follows_burns_and_refuelling(run: &Run, initial: &[Fixed]) {
    let events = run.events();
    for (index, ship) in SHIPS.iter().enumerate() {
        let taken = refuellings(&events)
            .iter()
            .filter(|(refuelled, ..)| refuelled == ship)
            .fold(Fixed::ZERO, |sum, (_, taken, ..)| sum + *taken);
        assert_eq!(
            own_fuel(&run.scenario.world, *ship),
            initial[index] - burned(&events, *ship) + taken,
            "{ship}"
        );
    }
}

#[test]
fn no_ship_burns_its_cargo() {
    let run = run_to_end(load());
    assert_own_fuel_follows_burns_and_refuelling(&run, &[tons(1000), tons(1000), tons(500)]);
}

#[test]
fn no_cargo_moves_while_the_force_is_under_way() {
    let run = run_to_end(load());
    // Day 2 is all passage.
    assert!(run.day(2).iter().all(|event| !is_cargo_event(event)));
    // On day 3 every cargo event comes after the arrival and names it.
    let arrival = run
        .day(3)
        .iter()
        .find(|event| matches!(event.kind, EventKind::TaskForceArrived { .. }))
        .expect("it arrives on day 3");
    for event in run.day(3).iter().filter(|event| is_cargo_event(event)) {
        assert!(event.id > arrival.id);
        assert!(event.causes.contains(&Cause::Event(arrival.id)));
    }
    // Nothing moves on day 1 or later that is not at one of the two bases:
    // each cargo event names the base of the hex the force is in.
    for event in run.events().iter().filter(|event| is_cargo_event(event)) {
        let base = match &event.kind {
            EventKind::CargoLoaded { base, .. }
            | EventKind::CargoUnloaded { base, .. }
            | EventKind::LoadingComplete { base, .. }
            | EventKind::UnloadingComplete { base, .. } => *base,
            _ => unreachable!(),
        };
        assert_eq!(base, if event.day == 1 { SANTO } else { GUADALCANAL });
    }
}

#[test]
fn the_log_explains_a_fuel_unload_and_the_refuelling_from_it() {
    let run = run_to_end(load());
    let world = &run.scenario.world;
    let log = world.events();
    let unload = log
        .involving(EntityRef::Ship(CARGO_1))
        .find(|event| matches!(event.kind, EventKind::CargoUnloaded { .. }))
        .expect("ship 1 unloaded");
    assert_eq!((unload.day, unload.pulse), (3, Some(Pulse::Night)));
    assert_eq!(
        unload.kind.to_string(),
        "ship:1 unloaded 200 t of fuel cargo at base:2, 2800 t of fuel cargo aboard, 200 t of fuel in stock"
    );
    let arrival = log
        .involving(EntityRef::TaskForce(TASK_FORCE))
        .find(|event| matches!(event.kind, EventKind::TaskForceArrived { .. }))
        .expect("arrived");
    let order_id = unload
        .causes
        .iter()
        .find_map(|cause| match cause {
            Cause::Order(id) => Some(*id),
            Cause::Event(_) => None,
        })
        .expect("the unload order is a cause");
    assert_eq!(unload.causes.len(), 2);
    assert!(unload.causes.contains(&Cause::Event(arrival.id)));
    assert_eq!(
        world.order(order_id).expect("on record").kind,
        OrderKind::TaskForce {
            target: TASK_FORCE,
            order: TaskForceOrder::Unload
        }
    );
    // The load order names its cargo.
    let load = log
        .involving(EntityRef::Ship(CARGO_1))
        .find(|event| matches!(event.kind, EventKind::CargoLoaded { .. }))
        .expect("ship 1 loaded");
    let load_order = load
        .causes
        .iter()
        .find_map(|cause| match cause {
            Cause::Order(id) => Some(*id),
            Cause::Event(_) => None,
        })
        .expect("the load order is a cause");
    assert_eq!(
        world.order(load_order).expect("on record").kind,
        OrderKind::TaskForce {
            target: TASK_FORCE,
            order: TaskForceOrder::LoadFuel
        }
    );
    // The refuelling of day 4 comes from the arrival, and its trace reaches
    // the movement and the route.
    let refuel = log
        .involving(EntityRef::Ship(CARGO_1))
        .find(|event| matches!(event.kind, EventKind::ShipRefuelled { .. }))
        .expect("ship 1 refuelled");
    assert_eq!(refuel.day, 4);
    assert!(refuel.causes.contains(&Cause::Event(arrival.id)));
    // The unload breakdown: 4 hours on the berth at 50 t an hour; the fuel
    // is all the ship had to land, so no supply came first.
    let breakdown = unload.breakdown.as_ref().expect("breakdowns are on");
    let factor = |name: &str| {
        breakdown
            .factors
            .iter()
            .find(|factor| factor.name == name)
            .map(|factor| factor.value)
    };
    assert_eq!(factor("hours_worked"), Some(tons(4)));
    assert_eq!(factor("on_berth"), Some(Fixed::ONE));
    assert_eq!(factor("rate_t_per_h"), Some(tons(50)));
    assert_eq!(factor("supply_unloaded_t"), Some(Fixed::ZERO));
    assert_eq!(breakdown.result, tons(200));
}

#[test]
fn each_side_sees_its_own_ships_fuel_cargo_and_nothing_of_the_enemys() {
    let run = run_to_end(load());
    let world = &run.scenario.world;
    let allied = world.side_view(Side::Allied);
    let cargo = |id: ShipId| {
        allied
            .ships()
            .find(|ship| ship.id == id)
            .expect("own ship")
            .cargo_fuel_t
    };
    assert_eq!(cargo(CARGO_1), tons(1000));
    assert_eq!(cargo(CARGO_2), tons(2200));
    assert_eq!(cargo(DESTROYER), Fixed::ZERO);
    let japanese = world.side_view(Side::Japanese);
    assert_eq!(japanese.ships().count(), 0);
    assert_eq!(japanese.bases().count(), 0);
}

#[test]
fn every_seed_gives_the_same_events() {
    let own = run_to_end(load()).events();
    assert_eq!(own.len(), 43);
    for seed in [1, 2, 7, 99, 31_337] {
        let run = run_to_end(load().with_seed(seed).expect("a seed can be set"));
        assert_eq!(run.events(), own, "seed {seed}");
    }
}

#[test]
fn recording_breakdowns_changes_neither_the_events_nor_the_hashes() {
    let run = |record_breakdowns: bool| {
        let mut scenario = load();
        scenario.world.set_config(SimConfig { record_breakdowns });
        let hashes: Vec<u64> = scenario
            .run_to_end()
            .expect("it runs")
            .iter()
            .map(|summary| summary.state_hash)
            .collect();
        (
            hashes,
            scenario.world.events().iter().cloned().collect::<Vec<_>>(),
        )
    };
    let (with_hashes, with_events) = run(true);
    let (without_hashes, without_events) = run(false);
    assert_eq!(with_hashes.len(), 4);
    assert_eq!(with_hashes, without_hashes);
    assert!(with_events.iter().any(|event| event.breakdown.is_some()));
    assert!(without_events.iter().all(|event| event.breakdown.is_none()));
    let stripped: Vec<Event> = with_events
        .into_iter()
        .map(|mut event| {
            event.breakdown = None;
            event
        })
        .collect();
    assert_eq!(stripped, without_events);
}

/// Loads a copy of scenario 2b whose text has been edited, in a fresh folder
/// under the build directory with the data it names. Each edit is
/// (old text, new text, the number of times the old text must appear).
/// Nothing in the repository's data folder is touched.
fn variant(tag: &str, edits: &[(&str, &str, usize)]) -> Scenario {
    let data = data_dir();
    let root = Path::new(env!("CARGO_TARGET_TMPDIR")).join(format!("fuel_run_{tag}"));
    for folder in ["scenarios", "map", "reference"] {
        fs::create_dir_all(root.join(folder)).expect("make the variant folder");
    }
    for file in [
        "map/placeholder_supply_run.toml",
        "reference/ship_classes.csv",
        "reference/port_sizes.csv",
        "reference/aircraft_types.csv",
        "reference/search.toml",
        "reference/surface_combat.toml",
    ] {
        fs::copy(data.join(file), root.join(file)).expect("copy a data file");
    }
    let mut text =
        fs::read_to_string(data.join("scenarios/02b_fuel_run.toml")).expect("read the scenario");
    for (old, new, count) in edits {
        assert_eq!(text.matches(old).count(), *count, "edit of {old:?}");
        text = text.replace(old, new);
    }
    let path = root.join("scenarios/02b_fuel_run.toml");
    fs::write(&path, text).expect("write the variant");
    Scenario::load(&path).expect("the variant loads")
}

const SANTO_FUEL: &str = "supply_t = 10000\nfuel_t = 6500\n";
const SANTO_NO_FUEL: &str = "supply_t = 10000\n";
const LOAD_FUEL: &str = "kind = \"load\"\ncargo = \"fuel\"\n";
const LOAD_SUPPLY: &str = "kind = \"load\"\n";
const SHIP_FUEL: &str = "fuel_t = 1000\ntask_force = 1\n";
const SHIP_SUPPLY_ABOARD: &str = "fuel_t = 1000\nsupply_t = 1000\ntask_force = 1\n";
const GUADALCANAL_ALLIED: &str = "name = \"Guadalcanal\"\nside = \"allied\"";
const GUADALCANAL_JAPANESE: &str = "name = \"Guadalcanal\"\nside = \"japanese\"";

fn with_santo_fuel(tons_of_fuel: i64) -> Scenario {
    let new = format!("supply_t = 10000\nfuel_t = {tons_of_fuel}\n");
    variant(&format!("santo_{tons_of_fuel}"), &[(SANTO_FUEL, &new, 1)])
}

/// Checks the limits and the conservation of fuel and supply in a variant,
/// every day, against what it started with.
fn run_checked(scenario: Scenario) -> Run {
    let fuel = total_fuel(&scenario.world);
    let supply = total_supply(&scenario.world);
    let mut burned_so_far = Fixed::ZERO;
    let mut checked: Vec<Fixed> = Vec::new();
    let run = run_with(scenario, |world, day| {
        assert_limits(world, day);
        assert_eq!(total_supply(world), supply, "supply, day {day}");
        checked.push(total_fuel(world));
    });
    for (seen, (day, events)) in run.days.iter().enumerate() {
        for ship in SHIPS {
            burned_so_far += burned(events, ship);
        }
        assert_eq!(checked[seen] + burned_so_far, fuel, "fuel, day {day}");
    }
    run
}

fn landed_fuel(run: &Run) -> Fixed {
    run.events()
        .iter()
        .filter_map(|event| match &event.kind {
            EventKind::CargoUnloaded {
                cargo: Cargo::Fuel,
                unloaded_t,
                ..
            } => Some(*unloaded_t),
            _ => None,
        })
        .fold(Fixed::ZERO, |sum, t| sum + t)
}

#[test]
fn a_load_order_that_names_no_cargo_loads_supply() {
    // Without the cargo the order is scenario 2's load. With no fuel at
    // Espiritu Santo either, the world is scenario 2's for four days: the
    // same events and the same hashes. The refuelling of day 4 finds no fuel
    // at Guadalcanal and does nothing.
    let run = run_checked(variant(
        "no_cargo_named",
        &[(LOAD_FUEL, LOAD_SUPPLY, 1), (SANTO_FUEL, SANTO_NO_FUEL, 1)],
    ));
    let mut supply_run =
        Scenario::load(&data_dir().join("scenarios/02_supply_run.toml")).expect("scenario 2 loads");
    let mut supply_events: Vec<Event> = Vec::new();
    let mut supply_hashes: Vec<u64> = Vec::new();
    for _ in 0..4 {
        let (_, events) = run_day(&mut supply_run);
        supply_events.extend(events);
        supply_hashes.push(*supply_run.world.day_hashes().last().expect("a hash"));
    }
    assert_eq!(run.events(), supply_events);
    assert_eq!(run.scenario.world.day_hashes(), supply_hashes.as_slice());
    assert!(refuellings(&run.events()).is_empty());
    assert!(moves(&run.events())
        .iter()
        .all(|(_, _, cargo, ..)| *cargo == Cargo::Supply));
    // And with fuel at Espiritu Santo, naming no cargo still loads supply and
    // leaves the fuel stock alone.
    let run = run_checked(variant(
        "no_cargo_named_fuel_there",
        &[(LOAD_FUEL, LOAD_SUPPLY, 1)],
    ));
    assert!(moves(&run.events())
        .iter()
        .all(|(_, _, cargo, ..)| *cargo == Cargo::Supply));
    assert_eq!(fuel_stock(&run.scenario.world, SANTO), tons(6500));
    assert_eq!(supply_stock(&run.scenario.world, SANTO), tons(4000));
}

#[test]
fn a_base_with_no_fuel_loads_nothing_and_the_convoy_lands_nothing() {
    let run = run_checked(variant("no_fuel", &[(SANTO_FUEL, SANTO_NO_FUEL, 1)]));
    let events = run.events();
    assert!(matches!(events[0].kind, EventKind::OrderAccepted { .. }));
    assert!(events.iter().all(|event| !is_cargo_event(event)));
    assert!(refuellings(&events).is_empty());
    let world = &run.scenario.world;
    for ship in SHIPS {
        assert_eq!(fuel_cargo(world, ship), Fixed::ZERO);
    }
    assert_eq!(fuel_stock(world, GUADALCANAL), Fixed::ZERO);
    // It still arrives.
    assert!(events
        .iter()
        .any(|event| matches!(event.kind, EventKind::TaskForceArrived { .. })));
}

#[test]
fn a_stock_that_runs_short_in_the_morning_is_shared_in_ship_order() {
    let run = run_checked(with_santo_fuel(5000));
    assert_eq!(
        fuel_moves(run.day(1)),
        vec![
            fuel_row(Pulse::Night, CARGO_1, 1800, 1800, 3200),
            fuel_row(Pulse::Night, CARGO_2, 1800, 1800, 1400),
            fuel_row(Pulse::Morning, CARGO_1, 900, 2700, 500),
            fuel_row(Pulse::Morning, CARGO_2, 500, 2300, 0),
        ]
    );
    // The stock is gone: loading is complete in the morning.
    let loading = completions(run.day(1));
    assert_eq!(loading.len(), 1);
    assert_eq!(loading[0].0, Some(Pulse::Morning));
    assert_eq!(loading[0].1, Cargo::Fuel);
    // They sail with 2,700 t and 2,300 t, and land at the same rates: what
    // reaches Guadalcanal in four days is what the full run landed from each
    // ship, so the stock there is the same, and 800 t is left in ship 2 less.
    let world = &run.scenario.world;
    assert_eq!(fuel_cargo(world, CARGO_1), tons(700));
    assert_eq!(fuel_cargo(world, CARGO_2), tons(1500));
    assert_eq!(fuel_stock(world, SANTO), Fixed::ZERO);
    assert_eq!(fuel_stock(world, GUADALCANAL), tons(2696));
}

#[test]
fn a_ship_with_supply_aboard_loads_fuel_into_what_is_left_and_lands_supply_first() {
    let run = run_checked(variant(
        "supply_aboard",
        &[(SHIP_FUEL, SHIP_SUPPLY_ABOARD, 2)],
    ));
    // Each ship loads 2,000 t: 1,800 t in the night, 200 t in the morning,
    // which takes 200 / 150 hours: complete 12 + 1.333333 hours in.
    assert_eq!(
        fuel_moves(run.day(1)),
        vec![
            fuel_row(Pulse::Night, CARGO_1, 1800, 1800, 4700),
            fuel_row(Pulse::Night, CARGO_2, 1800, 1800, 2900),
            fuel_row(Pulse::Morning, CARGO_1, 200, 2000, 2700),
            fuel_row(Pulse::Morning, CARGO_2, 200, 2000, 2500),
        ]
    );
    assert_eq!(
        completions(run.day(1)),
        vec![(
            Some(Pulse::Morning),
            Cargo::Fuel,
            Fixed::from_ratio(13_333_333, 1_000_000)
        )]
    );
    // At Guadalcanal supply goes first: in the 4 hours of the night of day
    // 3, ship 1 lands 200 t of supply and ship 2 80 t, and no fuel reaches
    // the stock until a ship's supply is gone.
    let night: Vec<Move> = moves(run.day(3))
        .into_iter()
        .filter(|(pulse, ..)| *pulse == Pulse::Night)
        .collect();
    assert_eq!(
        night,
        vec![
            (
                Pulse::Night,
                CARGO_1,
                Cargo::Supply,
                tons(200),
                tons(800),
                tons(700)
            ),
            (
                Pulse::Night,
                CARGO_2,
                Cargo::Supply,
                tons(80),
                tons(920),
                tons(780)
            ),
        ]
    );
    assert!(fuel_moves(run.day(3)).is_empty());
    // Ship 1 has 200 t of supply left on day 4. In the night's 12 hours it
    // lands that and then 400 t of fuel; ship 2, with 240 t, lands supply
    // only. So the first fuel to reach the stock is ship 1's 400 t.
    let first_fuel = fuel_moves(run.day(4));
    assert_eq!(
        first_fuel[0],
        fuel_row(Pulse::Night, CARGO_1, 400, 1600, 400)
    );
    // No fuel was in the stock at the start of day 4: nothing is refuelled.
    assert!(refuellings(run.day(4)).is_empty());
}

#[test]
fn an_enemy_base_takes_nothing_and_refuels_nobody() {
    let run = run_checked(variant(
        "japanese_guadalcanal",
        &[(GUADALCANAL_ALLIED, GUADALCANAL_JAPANESE, 1)],
    ));
    let events = run.events();
    assert!(events.iter().all(|event| !matches!(
        event.kind,
        EventKind::CargoUnloaded { .. } | EventKind::UnloadingComplete { .. }
    )));
    assert!(refuellings(&events).is_empty());
    let world = &run.scenario.world;
    assert_eq!(fuel_stock(world, GUADALCANAL), Fixed::ZERO);
    assert_eq!(fuel_cargo(world, CARGO_1), tons(3000));
    assert_eq!(fuel_cargo(world, CARGO_2), tons(3000));
}

#[test]
fn more_fuel_at_espiritu_santo_never_lands_less_at_guadalcanal() {
    let mut landed: Vec<(i64, Fixed)> = Vec::new();
    for stock in [0, 1000, 2500, 3600, 5000, 6500, 9000] {
        let run = run_checked(with_santo_fuel(stock));
        landed.push((stock, landed_fuel(&run)));
    }
    for pair in landed.windows(2) {
        assert!(
            pair[1].1 >= pair[0].1,
            "{} t landed with {} t, but {} t with {} t",
            pair[1].1,
            pair[1].0,
            pair[0].1,
            pair[0].0
        );
    }
    assert_eq!(landed[0], (0, Fixed::ZERO));
    // The full run lands what the unloading tables say: 200 + 80 + 300 + 120
    // + 300 + 120 on day 3 and 600 + 240 + 300 + 120 + 300 + 120 on day 4.
    assert_eq!(landed[5], (6500, tons(2800)));
}

#[test]
fn a_scenario_that_loads_no_fuel_has_the_hashes_it_had() {
    // Scenario 2 and its four-day twin with fuel left out entirely have the
    // same state on every day, so the same hashes: fuel cargo adds nothing to
    // the hash while it is zero. (The golden files of every other scenario
    // are the standing proof for the rest.)
    let mut scenario =
        Scenario::load(&data_dir().join("scenarios/02_supply_run.toml")).expect("scenario 2 loads");
    let before: Vec<u64> = scenario
        .run_to_end()
        .expect("it runs")
        .iter()
        .take(4)
        .map(|summary| summary.state_hash)
        .collect();
    let twin = variant(
        "no_fuel_twin",
        &[(SANTO_FUEL, SANTO_NO_FUEL, 1), (LOAD_FUEL, LOAD_SUPPLY, 1)],
    );
    let run = run_to_end(twin);
    assert_eq!(run.scenario.world.day_hashes(), before.as_slice());
}
