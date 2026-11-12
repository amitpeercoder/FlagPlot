//! Scenario 4a2: a force with two damaged ships is ordered away from home
//! and told to detach its stragglers.
//!
//! Stage 4a2 of scenario 4. It proves that propulsion damage slows a ship
//! and its force, and that a force handles its stragglers by its standing
//! instruction. There is no enemy and no randomness: the damage is set in
//! the scenario file, so every number here is exact. System hits in combat
//! are proved in `scenario_04a_night_action.rs`.
//!
//! The data is placeholder data: see `data/scenarios/04a2_stragglers.toml`.
//! The rule is `docs/decisions/0016-systems-and-stragglers.md`.
//!
//! # Hand calculation
//!
//! Written by the architect before the code. The tests below assert these
//! numbers exactly. If the code disagrees, the code or the rule is wrong: do
//! not change a number here to make a test pass.
//!
//! Inputs, from the data files:
//!
//! - Task force 1, Allied, at (6,0): cruisers 1, 2 and 3 and destroyers 4, 5
//!   and 6. Its home port is base 1 at (0,0), 6 hexes and 240 nm to the west.
//! - Cruiser 2's propulsion is degraded: half speed. Destroyer 6's propulsion
//!   is out: dead in the water. The other four are undamaged.
//! - Both classes cruise at 16 kn. A cruiser burns 0.25 t per nm and a
//!   destroyer 0.125 t per nm, and no less at lower speeds.
//! - Orders on day 1: steam east to (12,0) at cruise speed. Detach
//!   stragglers, keeping at least 0.5 of the ships that can steam at full
//!   speed.
//! - Escorts for a straggler sent home: 1 for a cruiser, none for a
//!   destroyer.
//!
//! ## The instruction is applied
//!
//! At the start of the first movement step, in the night pulse of day 1:
//!
//! - Stragglers: cruiser 2 (can steam) and destroyer 6 (cannot).
//! - Ships with working propulsion: 1, 3, 4 and 5. That is 4.
//! - Escorts needed: 1, for cruiser 2. Leaving 3 of 4 is 0.75, which is at
//!   least 0.5, so the force detaches and does not retire.
//! - The escort is the least important ship with working propulsion and
//!   guns. Destroyers come before cruisers, and destroyer 4 has the lowest
//!   ID.
//!
//! | Task force          | Ships   | What it does                             |
//! | ------------------- | ------- | ---------------------------------------- |
//! | 1, the parent       | 1, 3, 5 | Carries on east to (12,0) at 16 kn       |
//! | 2, sent home        | 2, 4    | Heads for (0,0) at 8 kn, set to avoid    |
//! | 3, left behind      | 6       | Stays at (6,0), set to avoid             |
//!
//! Task force 2 is made first and task force 3 second. Task force 2's route
//! comes from a set-route order issued by the local commander and taken in at once. Its
//! speed is that of its slowest ship: cruiser 2 at 16 x 0.5 = 8 kn. All three
//! forces move in that same movement step.
//!
//! ## Task force 1: east at 16 kn
//!
//! (6,0) to (12,0) is 6 hexes, 240 nm, 15 hours.
//!
//! | Pulse         | Hours | Distance | Total  | Hex left, miles on | Position |
//! | ------------- | ----- | -------- | ------ | ------------------ | -------- |
//! | Day 1 night   | 12    | 192 nm   | 192 nm | (10,0) plus 32 nm  | (11,0)   |
//! | Day 1 morning | 3     | 48 nm    | 240 nm | (12,0), arrived    | (12,0)   |
//!
//! It arrives 15 hours after the start, at 09:00 on day 1. Fuel: cruisers 1
//! and 3 burn 240 x 0.25 = 60 t each and end with 1940 t. Destroyer 5 burns
//! 240 x 0.125 = 30 t and ends with 470 t.
//!
//! ## Task force 2: home at 8 kn
//!
//! (6,0) to (0,0) is 6 hexes, 240 nm, 30 hours.
//!
//! | Pulse           | Hours | Distance | Total  | Hex left, miles on | Position |
//! | --------------- | ----- | -------- | ------ | ------------------ | -------- |
//! | Day 1 night     | 12    | 96 nm    | 96 nm  | (4,0) plus 16 nm   | (4,0)    |
//! | Day 1 morning   | 6     | 48 nm    | 144 nm | (3,0) plus 24 nm   | (2,0)    |
//! | Day 1 afternoon | 6     | 48 nm    | 192 nm | (2,0) plus 32 nm   | (1,0)    |
//! | Day 2 night     | 6     | 48 nm    | 240 nm | (0,0), arrived     | (0,0)    |
//!
//! It arrives 30 hours after the start, at 00:00 on day 2. Fuel by pulse:
//! cruiser 2 burns 24, 12, 12 and 12 t, 60 t in all, and ends with 1940 t.
//! Destroyer 4 burns 12, 6, 6 and 6 t, 30 t in all, and ends with 470 t. The
//! destroyer could steam at 16 kn and is held to 8 kn by the cruiser. It
//! burns its cruise rate per mile all the same.
//!
//! ## Task force 3: left behind
//!
//! Destroyer 6 stays at (6,0) for both days and burns nothing. It ends with
//! 500 t.
//!
//! ## The other instructions
//!
//! The same force and orders, with one thing changed each time:
//!
//! | Change                     | What happens                                                    |
//! | -------------------------- | --------------------------------------------------------------- |
//! | Share to keep 0.8          | 3 of 4 is 0.75, below 0.8, so the force retires. Destroyer 6 is left as task force 2. Task force 1, ships 1 to 5, heads home at 8 kn on a staff order and arrives at hour 30. Cruisers burn 60 t and destroyers 30 t. |
//! | Instruction "retire"       | The same as the row above, whatever the share to keep.          |
//! | Instruction "leave"        | Cruiser 2 and destroyer 6 are left together as task force 2 at (6,0) and burn nothing. Task force 1, ships 1, 3, 4 and 5, steams east at 16 kn and arrives at hour 15. |
//! | Instruction "keep"         | Nothing is detached. The force's slowest ship is dead in the water, so it does not move for two days and no ship burns fuel. It stays under way. |
//! | "Detach", no home port     | Task force 2, ships 2 and 4, is made but gets no order and stays at (6,0). Task force 3 is destroyer 6. Task force 1 steams east. |
//! | Destroyer 6 undamaged      | "Detach": ships with working propulsion are 1, 3, 4, 5 and 6. Cruiser 2 goes home with destroyer 4. No force is left behind. Task force 1, ships 1, 3, 5 and 6, steams east. |
//! | Cruisers 1 and 3 damaged too | "Detach" with cruisers 1, 2 and 3 all at half speed: 3 escorts are needed and only destroyers 4 and 5 have working propulsion. Giving both would leave the parent none, so the force retires: ships 1 to 5 head home at 8 kn and destroyer 6 is left. |
//!
//! # Properties
//!
//! - No ship is ever in two task forces, and every ship afloat is in one.
//! - A task force made for stragglers holds only stragglers and its escorts.
//! - A parent that detached has no straggler left.
//! - The instruction is applied once: on later movement steps nothing more
//!   is detached, because the new forces keep their stragglers.
//! - Every movement event of a force sent home cites the staff order, and
//!   that order's acceptance cites the event that detached the force.
//! - A replay reproduces both daily hashes.

mod common;

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::PathBuf;

use sim_core::events::{Cause, DetachReason, Event, EventKind};
use sim_core::fixed::Fixed;
use sim_core::hex::Hex;
use sim_core::ids::{EventId, OrderId, ShipId, TaskForceId};
use sim_core::orders::{Issuer, Mission, OrderKind, SpeedSetting, Stance, StragglerInstruction};
use sim_core::orders::{Order, TaskForceOrder};
use sim_core::state::{ShipLocation, Side, SystemState, TaskForceStatus};
use sim_core::time::Pulse;
use sim_core::World;
use sim_data::Scenario;

const TF1: TaskForceId = TaskForceId::new(1);
const TF2: TaskForceId = TaskForceId::new(2);
const TF3: TaskForceId = TaskForceId::new(3);

fn ship(raw: u32) -> ShipId {
    ShipId::new(raw)
}

fn ships(raw: &[u32]) -> Vec<ShipId> {
    raw.iter().map(|id| ShipId::new(*id)).collect()
}

fn set(raw: &[u32]) -> BTreeSet<ShipId> {
    raw.iter().map(|id| ShipId::new(*id)).collect()
}

fn n(value: i64) -> Fixed {
    Fixed::from_int(value)
}

fn hex(q: i32, r: i32) -> Hex {
    Hex::new(q, r)
}

// ---------------------------------------------------------------------
// Building the scenario and its variants.
// ---------------------------------------------------------------------

/// The scenario and the changes to its text that make each variant of the
/// "other instructions" table.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Case {
    Main,
    ShareToKeep08,
    Retire,
    Leave,
    Keep,
    NoHomePort,
    Destroyer6Undamaged,
    CruisersDamaged,
}

const ALL_CASES: [Case; 8] = [
    Case::Main,
    Case::ShareToKeep08,
    Case::Retire,
    Case::Leave,
    Case::Keep,
    Case::NoHomePort,
    Case::Destroyer6Undamaged,
    Case::CruisersDamaged,
];

const SHARE: (&str, &str) = ("keep_share = \"0.5\"", "keep_share = \"0.8\"");
const DETACH: &str = "stragglers = \"detach\"";
const HOME_PORT: (&str, &str) = ("home_port = 1\n", "");
const DESTROYER_6_PROPULSION: (&str, &str) = ("propulsion = \"out\"\n", "");
const CRUISER_1: (&str, &str) = (
    "name = \"Placeholder cruiser 1\"",
    "name = \"Placeholder cruiser 1\"\npropulsion = \"degraded\"",
);
const CRUISER_3: (&str, &str) = (
    "name = \"Placeholder cruiser 3\"",
    "name = \"Placeholder cruiser 3\"\npropulsion = \"degraded\"",
);

fn edits(case: Case) -> Vec<(&'static str, &'static str)> {
    match case {
        Case::Main => vec![],
        Case::ShareToKeep08 => vec![SHARE],
        Case::Retire => vec![(DETACH, "stragglers = \"retire\"")],
        Case::Leave => vec![(DETACH, "stragglers = \"leave\"")],
        Case::Keep => vec![(DETACH, "stragglers = \"keep\"")],
        Case::NoHomePort => vec![HOME_PORT],
        Case::Destroyer6Undamaged => vec![DESTROYER_6_PROPULSION],
        Case::CruisersDamaged => vec![CRUISER_1, CRUISER_3],
    }
}

/// Loads the scenario on the frozen figures with each exact, asserted-unique
/// text replacement made in its text. Nothing in the repository's data folder
/// is touched.
fn variant(edits: &[(&str, &str)]) -> Scenario {
    let path =
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../data/scenarios/04a2_stragglers.toml");
    let mut text = fs::read_to_string(&path).expect("read the scenario");
    for (original, changed) in edits {
        text = common::replace_once(&text, original, changed);
    }
    common::from_text(&text, &path).expect("the variant loads")
}

fn build(case: Case) -> Scenario {
    variant(&edits(case))
}

/// A finished run: the scenario after both days, and every event by day.
struct Run {
    scenario: Scenario,
    events: Vec<Event>,
    day_event_counts: Vec<usize>,
}

fn run(case: Case) -> Run {
    let mut scenario = build(case);
    assert!(scenario.placeholder);
    assert_eq!(scenario.days, 2);
    let mut day_event_counts = Vec::new();
    let mut events = Vec::new();
    while let Some(summary) = scenario.next_day().expect("the day resolves") {
        assert_eq!(summary.day as usize, day_event_counts.len() + 1);
        let day_events = scenario.world.events().from_index(summary.first_event);
        assert_eq!(day_events.len(), summary.event_count);
        day_event_counts.push(day_events.len());
        events.extend(day_events.iter().cloned());
    }
    assert_eq!(day_event_counts.len(), 2);
    assert_eq!(scenario.world.day_hashes().len(), 2);
    assert_eq!(events.len(), scenario.world.events().len());
    Run {
        scenario,
        events,
        day_event_counts,
    }
}

impl Run {
    fn world(&self) -> &World {
        &self.scenario.world
    }

    fn fuel(&self, id: u32) -> Fixed {
        self.world().umpire().ships[&ship(id)].fuel_t
    }

    /// The final ships of every task force, by ID.
    fn forces(&self) -> BTreeMap<TaskForceId, BTreeSet<ShipId>> {
        self.world()
            .umpire()
            .task_forces
            .iter()
            .map(|(id, force)| (*id, force.ships.clone()))
            .collect()
    }

    fn force(&self, id: TaskForceId) -> &sim_core::state::TaskForce {
        &self.world().umpire().task_forces[&id]
    }

    fn steams(&self, force: TaskForceId) -> Vec<Steam> {
        self.events
            .iter()
            .filter_map(|event| match &event.kind {
                EventKind::TaskForceSteamed {
                    task_force,
                    from,
                    to,
                    distance_nm,
                    speed_kn,
                    ..
                } if *task_force == force => Some(Steam {
                    id: event.id,
                    day: event.day,
                    pulse: event.pulse.expect("movement has a pulse"),
                    from: *from,
                    to: *to,
                    distance: *distance_nm,
                    speed: *speed_kn,
                    causes: event.causes.clone(),
                }),
                _ => None,
            })
            .collect()
    }

    fn all_steams(&self) -> Vec<&Event> {
        self.events
            .iter()
            .filter(|event| matches!(event.kind, EventKind::TaskForceSteamed { .. }))
            .collect()
    }

    /// Burns of a ship as (day, pulse, burned, remaining, causes).
    fn burns(&self, id: u32) -> Vec<(u32, Pulse, Fixed, Fixed, Vec<Cause>)> {
        self.events
            .iter()
            .filter_map(|event| match &event.kind {
                EventKind::FuelBurned {
                    ship: burned,
                    burned_t,
                    remaining_t,
                    ..
                } if *burned == ship(id) => Some((
                    event.day,
                    event.pulse.expect("burn has a pulse"),
                    *burned_t,
                    *remaining_t,
                    event.causes.clone(),
                )),
                _ => None,
            })
            .collect()
    }

    fn total_burned(&self, id: u32) -> Fixed {
        self.burns(id)
            .iter()
            .fold(Fixed::ZERO, |sum, burn| sum + burn.2)
    }

    /// Every ShipsDetached event as (event, parent, new force, ships,
    /// escorts, reason).
    fn detachments(&self) -> Vec<Detached> {
        self.events
            .iter()
            .filter_map(|event| match &event.kind {
                EventKind::ShipsDetached {
                    parent,
                    new_force,
                    ships,
                    escorts,
                    reason,
                } => Some(Detached {
                    event: event.clone(),
                    parent: *parent,
                    new_force: *new_force,
                    ships: ships.clone(),
                    escorts: escorts.clone(),
                    reason: *reason,
                }),
                _ => None,
            })
            .collect()
    }

    fn retirements(&self) -> Vec<(Event, TaskForceId)> {
        self.events
            .iter()
            .filter_map(|event| match &event.kind {
                EventKind::ForceRetired { task_force } => Some((event.clone(), *task_force)),
                _ => None,
            })
            .collect()
    }

    /// Arrival hours of a force, one per arrival event.
    fn arrivals(&self, force: TaskForceId) -> Vec<Fixed> {
        self.events
            .iter()
            .filter_map(|event| match &event.kind {
                EventKind::TaskForceArrived {
                    task_force,
                    elapsed_hours,
                    ..
                } if *task_force == force => Some(*elapsed_hours),
                _ => None,
            })
            .collect()
    }

    fn staff_orders(&self) -> Vec<&Order> {
        self.world()
            .orders()
            .filter(|order| order.origin.issuer == Issuer::LocalCommander)
            .collect()
    }

    fn accepted_event(&self, order: OrderId) -> Vec<&Event> {
        self.events
            .iter()
            .filter(|event| {
                matches!(&event.kind, EventKind::OrderAccepted { order: o, .. } if *o == order)
            })
            .collect()
    }

    fn is_straggler(&self, id: ShipId) -> bool {
        self.world().umpire().ships[&id].systems.propulsion != SystemState::Working
    }
}

struct Steam {
    id: EventId,
    day: u32,
    pulse: Pulse,
    from: Hex,
    to: Hex,
    distance: Fixed,
    speed: Fixed,
    causes: Vec<Cause>,
}

struct Detached {
    event: Event,
    parent: TaskForceId,
    new_force: TaskForceId,
    ships: Vec<ShipId>,
    escorts: Vec<ShipId>,
    reason: DetachReason,
}

// ---------------------------------------------------------------------
// The hand calculation's movement tables.
// ---------------------------------------------------------------------

/// One row of a hand-calculation movement table.
struct Row {
    day: u32,
    pulse: Pulse,
    hours: i64,
    distance: i64,
    total: i64,
    /// The hex left, and the miles on toward the next one.
    hex_left: Hex,
    miles_on: i64,
    position: Hex,
}

const NM_PER_HEX: i64 = 40;

fn row(
    (day, pulse): (u32, Pulse),
    hours: i64,
    distance: i64,
    total: i64,
    (hex_left, miles_on): (Hex, i64),
    position: Hex,
) -> Row {
    Row {
        day,
        pulse,
        hours,
        distance,
        total,
        hex_left,
        miles_on,
        position,
    }
}

/// Task force 1: east at 16 kn.
fn tf1_rows() -> Vec<Row> {
    vec![
        row(
            (1, Pulse::Night),
            12,
            192,
            192,
            (hex(10, 0), 32),
            hex(11, 0),
        ),
        row((1, Pulse::Morning), 3, 48, 240, (hex(12, 0), 0), hex(12, 0)),
    ]
}

/// Task force 2 (or task force 1 retiring): home at 8 kn.
fn home_rows() -> Vec<Row> {
    vec![
        row((1, Pulse::Night), 12, 96, 96, (hex(4, 0), 16), hex(4, 0)),
        row((1, Pulse::Morning), 6, 48, 144, (hex(3, 0), 24), hex(2, 0)),
        row(
            (1, Pulse::Afternoon),
            6,
            48,
            192,
            (hex(2, 0), 32),
            hex(1, 0),
        ),
        row((2, Pulse::Night), 6, 48, 240, (hex(0, 0), 0), hex(0, 0)),
    ]
}

/// Checks a force's movement events against a table, and rebuilds the
/// position at the end of each pulse from the miles covered so far. The
/// stored leg at the end of each pulse is the hex left and the miles on, so
/// the rebuild is the stored leg.
fn assert_movement(steams: &[Steam], rows: &[Row], start: Hex, step: i32, speed_kn: i64) {
    assert_eq!(steams.len(), rows.len());
    let mut total = Fixed::ZERO;
    let mut at = start;
    for (steam, row) in steams.iter().zip(rows) {
        assert_eq!((steam.day, steam.pulse), (row.day, row.pulse));
        assert_eq!(steam.speed, n(speed_kn));
        assert_eq!(steam.distance, n(row.distance));
        // Hours are distance over speed.
        assert_eq!(steam.distance / steam.speed, n(row.hours));
        assert_eq!(steam.from, at);
        assert_eq!(steam.to, row.position);
        total += steam.distance;
        assert_eq!(total, n(row.total));
        // The hand calculation's own columns agree with each other: whole
        // hexes plus the miles on are the total, and the position is the
        // nearest hex centre (20 nm or more on rounds up).
        let hexes_done = (start.q - row.hex_left.q).abs();
        assert_eq!(hexes_done as i64 * NM_PER_HEX + row.miles_on, row.total);
        assert_eq!(row.hex_left.r, 0);
        let nearest = if row.miles_on >= NM_PER_HEX / 2 {
            row.hex_left.q + step
        } else {
            row.hex_left.q
        };
        assert_eq!(row.position, hex(nearest, 0));
        at = row.position;
    }
}

/// The stored leg after the last pulse: arrived at the hex, no progress.
fn assert_stopped_at(world: &World, id: TaskForceId, at: Hex) {
    let force = &world.umpire().task_forces[&id];
    assert_eq!(force.from_hex, at);
    assert_eq!(force.toward_hex, None);
    assert_eq!(force.progress_nm, Fixed::ZERO);
    assert_eq!(force.position(), at);
}

/// The fuel burned by a ship, by pulse, and what was left after each.
fn assert_burns(run: &Run, id: u32, rows: &[Row], per_nm_tenths: i64, start: i64) {
    let burns = run.burns(id);
    assert_eq!(burns.len(), rows.len());
    let mut left = n(start);
    for (burn, row) in burns.iter().zip(rows) {
        assert_eq!((burn.0, burn.1), (row.day, row.pulse));
        // Burn per mile in thousandths of a ton: 250 is 0.25 t per nm.
        let expected = Fixed::from_ratio(row.distance * per_nm_tenths, 1000);
        assert_eq!(burn.2, expected);
        left -= expected;
        assert_eq!(burn.3, left);
    }
}

// ---------------------------------------------------------------------
// The main case.
// ---------------------------------------------------------------------

#[test]
fn the_starting_state_is_the_inputs() {
    let scenario = build(Case::Main);
    let world = &scenario.world;
    let truth = world.umpire();
    assert_eq!(truth.task_forces.len(), 1);
    let force = &truth.task_forces[&TF1];
    assert_eq!(force.ships, set(&[1, 2, 3, 4, 5, 6]));
    assert_eq!(force.position(), hex(6, 0));
    assert_eq!(force.mission, Mission::SurfaceCombat);
    assert_eq!(force.home_port.map(|b| b.raw()), Some(1));
    assert_eq!(truth.bases[&force.home_port.unwrap()].hex, hex(0, 0));
    assert_eq!(
        truth.ships[&ship(2)].systems.propulsion,
        SystemState::Degraded
    );
    assert_eq!(truth.ships[&ship(6)].systems.propulsion, SystemState::Out);
    for id in [1, 3, 4, 5] {
        assert_eq!(
            truth.ships[&ship(id)].systems.propulsion,
            SystemState::Working
        );
    }
    for (id, fuel) in [
        (1, 2000),
        (2, 2000),
        (3, 2000),
        (4, 500),
        (5, 500),
        (6, 500),
    ] {
        assert_eq!(truth.ships[&ship(id)].fuel_t, n(fuel));
    }
    assert_eq!(world.orders().count(), 0);
}

#[test]
fn the_first_pass_makes_three_forces_in_order() {
    let run = run(Case::Main);
    let detached = run.detachments();
    assert_eq!(detached.len(), 2);

    // Task force 2 first: cruiser 2 sent home with destroyer 4 as escort.
    let first = &detached[0];
    assert_eq!(first.parent, TF1);
    assert_eq!(first.new_force, TF2);
    assert_eq!(first.ships, ships(&[2]));
    assert_eq!(first.escorts, ships(&[4]));
    assert_eq!(first.reason, DetachReason::SentHome);

    // Task force 3 second: destroyer 6 left behind, no escorts.
    let second = &detached[1];
    assert_eq!(second.parent, TF1);
    assert_eq!(second.new_force, TF3);
    assert_eq!(second.ships, ships(&[6]));
    assert!(second.escorts.is_empty());
    assert_eq!(second.reason, DetachReason::LeftBehind);

    // Both cite the instruction (order 3), in the night pulse of day 1,
    // before the force moves, and nothing retires.
    for event in [&first.event, &second.event] {
        assert_eq!((event.day, event.pulse), (1, Some(Pulse::Night)));
        assert_eq!(event.causes, vec![Cause::Order(OrderId::new(3))]);
    }
    assert!(run.retirements().is_empty());
    let first_steam = run.all_steams()[0].id;
    assert!(second.event.id < first_steam);
    // Staff order 4 is accepted between the two detachments.
    let accepted = run.accepted_event(OrderId::new(4));
    assert_eq!(accepted.len(), 1);
    assert!(first.event.id < accepted[0].id && accepted[0].id < second.event.id);

    // The forces as the pass leaves them.
    assert_eq!(
        run.forces(),
        BTreeMap::from([
            (TF1, set(&[1, 3, 5])),
            (TF2, set(&[2, 4])),
            (TF3, set(&[6]))
        ])
    );
    let parent = run.force(TF1);
    assert_eq!(parent.rules_of_engagement.stance, Stance::Engage);
    assert_eq!(
        parent.rules_of_engagement.stragglers,
        StragglerInstruction::Detach
    );
    assert_eq!(
        parent.rules_of_engagement.keep_share,
        Fixed::from_ratio(1, 2)
    );
    for (id, name) in [(TF2, "sent home"), (TF3, "left behind")] {
        let force = run.force(id);
        assert!(force.name.ends_with(name), "{}", force.name);
        assert_eq!(force.side, Side::Allied);
        assert_eq!(force.mission, Mission::SurfaceCombat);
        assert_eq!(force.speed, SpeedSetting::Cruise);
        assert_eq!(force.home_port, parent.home_port);
        // Set to avoid, and to keep their own stragglers.
        assert_eq!(force.rules_of_engagement.stance, Stance::Avoid);
        assert_eq!(
            force.rules_of_engagement.stragglers,
            StragglerInstruction::Keep
        );
    }
    // Task force 3 stays idle with no route.
    let left = run.force(TF3);
    assert_eq!(left.status, TaskForceStatus::Idle);
    assert!(left.route.waypoints.is_empty());
    assert_eq!(left.route_order, None);
    assert_stopped_at(run.world(), TF3, hex(6, 0));
}

#[test]
fn the_staff_order_sends_task_force_2_home() {
    let run = run(Case::Main);
    assert_eq!(run.world().orders().count(), 4);
    let staff = run.staff_orders();
    assert_eq!(staff.len(), 1);
    let order = staff[0];
    assert_eq!(order.id, OrderId::new(4));
    assert_eq!(order.side, Side::Allied);
    assert_eq!(order.day_issued, 1);
    assert_eq!(order.origin.issuer, Issuer::LocalCommander);
    assert_eq!(order.origin.intent, None);
    assert_eq!(
        order.kind,
        OrderKind::TaskForce {
            target: TF2,
            order: TaskForceOrder::SetRoute(vec![hex(0, 0)]),
        }
    );
    // Accepted in the night pulse of day 1, citing the order and the event
    // that sent the ships home.
    let accepted = run.accepted_event(order.id);
    assert_eq!(accepted.len(), 1);
    assert_eq!(
        (accepted[0].day, accepted[0].pulse),
        (1, Some(Pulse::Night))
    );
    let detach = run.detachments()[0].event.id;
    assert_eq!(
        accepted[0].causes,
        vec![Cause::Order(order.id), Cause::Event(detach)]
    );
    assert_eq!(run.force(TF2).route_order, Some(order.id));
    assert_eq!(run.force(TF2).route.waypoints, vec![hex(0, 0)]);
    // Every movement of task force 2 cites it.
    let steams = run.steams(TF2);
    assert_eq!(steams.len(), 4);
    for steam in &steams {
        assert!(steam.causes.contains(&Cause::Order(order.id)));
    }
}

#[test]
fn task_force_1_steams_east_at_16_knots() {
    let run = run(Case::Main);
    let steams = run.steams(TF1);
    assert_movement(&steams, &tf1_rows(), hex(6, 0), 1, 16);
    // It cites its own route and speed orders, not the staff order.
    for steam in &steams {
        assert_eq!(
            steam.causes,
            vec![Cause::Order(OrderId::new(1)), Cause::Order(OrderId::new(2))]
        );
    }
    // Arrived 15 hours after the start, 09:00 on day 1.
    assert_eq!(run.arrivals(TF1), vec![n(15)]);
    assert_eq!(run.force(TF1).status, TaskForceStatus::Arrived);
    assert_stopped_at(run.world(), TF1, hex(12, 0));
    // Fuel by pulse: cruisers 48 then 12 t, destroyer 5 24 then 6 t.
    for id in [1, 3] {
        assert_burns(&run, id, &tf1_rows(), 250, 2000);
        assert_eq!(run.total_burned(id), n(60));
        assert_eq!(run.fuel(id), n(1940));
    }
    assert_burns(&run, 5, &tf1_rows(), 125, 500);
    assert_eq!(run.total_burned(5), n(30));
    assert_eq!(run.fuel(5), n(470));
    let cruiser = run.burns(1);
    assert_eq!(cruiser[0].2, n(48));
    assert_eq!(cruiser[1].2, n(12));
    // Every burn cites the movement event of the same pulse.
    for id in [1, 3, 5] {
        for (burn, steam) in run.burns(id).iter().zip(&steams) {
            assert_eq!(burn.4, vec![Cause::Event(steam.id)]);
        }
    }
}

#[test]
fn task_force_2_steams_home_at_8_knots() {
    let run = run(Case::Main);
    let steams = run.steams(TF2);
    assert_movement(&steams, &home_rows(), hex(6, 0), -1, 8);
    for steam in &steams {
        assert_eq!(
            steam.causes,
            vec![Cause::Order(OrderId::new(4)), Cause::Order(OrderId::new(2))]
        );
    }
    // Arrived 30 hours after the start, 00:00 on day 2.
    assert_eq!(run.arrivals(TF2), vec![n(30)]);
    assert_eq!(run.force(TF2).status, TaskForceStatus::Arrived);
    assert_stopped_at(run.world(), TF2, hex(0, 0));
    // Cruiser 2 burns 24, 12, 12 and 12 t; destroyer 4 12, 6, 6 and 6 t.
    let cruiser = run.burns(2);
    let burned: Vec<Fixed> = cruiser.iter().map(|b| b.2).collect();
    assert_eq!(burned, vec![n(24), n(12), n(12), n(12)]);
    let left: Vec<Fixed> = cruiser.iter().map(|b| b.3).collect();
    assert_eq!(left, vec![n(1976), n(1964), n(1952), n(1940)]);
    let destroyer = run.burns(4);
    let burned: Vec<Fixed> = destroyer.iter().map(|b| b.2).collect();
    assert_eq!(burned, vec![n(12), n(6), n(6), n(6)]);
    let left: Vec<Fixed> = destroyer.iter().map(|b| b.3).collect();
    assert_eq!(left, vec![n(488), n(482), n(476), n(470)]);
    assert_burns(&run, 2, &home_rows(), 250, 2000);
    assert_burns(&run, 4, &home_rows(), 125, 500);
    assert_eq!(run.total_burned(2), n(60));
    assert_eq!(run.total_burned(4), n(30));
    assert_eq!(run.fuel(2), n(1940));
    assert_eq!(run.fuel(4), n(470));
    for id in [2, 4] {
        for (burn, steam) in run.burns(id).iter().zip(&steams) {
            assert_eq!(burn.4, vec![Cause::Event(steam.id)]);
        }
    }
}

#[test]
fn task_force_3_stays_and_burns_nothing() {
    let run = run(Case::Main);
    assert!(run.steams(TF3).is_empty());
    assert!(run.arrivals(TF3).is_empty());
    assert!(run.burns(6).is_empty());
    assert_eq!(run.fuel(6), n(500));
    assert_stopped_at(run.world(), TF3, hex(6, 0));
}

#[test]
fn the_main_case_totals_and_final_fuel() {
    let run = run(Case::Main);
    let total: Fixed = (1..=6).fold(Fixed::ZERO, |sum, id| sum + run.total_burned(id));
    // 150 t for task force 1, 90 t for task force 2, none for task force 3.
    assert_eq!(total, n(240));
    let force_1: Fixed = [1, 3, 5]
        .iter()
        .fold(Fixed::ZERO, |s, id| s + run.total_burned(*id));
    let force_2: Fixed = [2, 4]
        .iter()
        .fold(Fixed::ZERO, |s, id| s + run.total_burned(*id));
    assert_eq!((force_1, force_2), (n(150), n(90)));
    let fuel: Vec<Fixed> = (1..=6).map(|id| run.fuel(id)).collect();
    assert_eq!(
        fuel,
        vec![n(1940), n(1940), n(1940), n(470), n(470), n(500)]
    );
    // Events by day: orders, both detachments, the staff order, and all the
    // movement of day 1; only task force 2's last leg and arrival on day 2.
    assert_eq!(run.day_event_counts, vec![24, 4]);
}

// ---------------------------------------------------------------------
// The other instructions.
// ---------------------------------------------------------------------

/// Nothing steams, burns or arrives for these forces.
fn assert_idle_force(run: &Run, force: TaskForceId, fuels: &[(u32, i64)]) {
    assert!(run.steams(force).is_empty());
    assert!(run.arrivals(force).is_empty());
    for (id, fuel) in fuels {
        assert!(run.burns(*id).is_empty());
        assert_eq!(run.fuel(*id), n(*fuel));
    }
    assert_eq!(run.force(force).status, TaskForceStatus::Idle);
    assert!(run.force(force).route.waypoints.is_empty());
    assert_stopped_at(run.world(), force, hex(6, 0));
}

/// The whole force retires: ships 1 to 5 head home at 8 kn on a staff order
/// and arrive at hour 30, and destroyer 6 is left as task force 2.
fn assert_force_retires(run: &Run) {
    let detached = run.detachments();
    assert_eq!(detached.len(), 1);
    assert_eq!(detached[0].parent, TF1);
    assert_eq!(detached[0].new_force, TF2);
    assert_eq!(detached[0].ships, ships(&[6]));
    assert!(detached[0].escorts.is_empty());
    assert_eq!(detached[0].reason, DetachReason::LeftBehind);
    let retired = run.retirements();
    assert_eq!(retired.len(), 1);
    assert_eq!(retired[0].1, TF1);
    assert_eq!(retired[0].0.causes, vec![Cause::Order(OrderId::new(3))]);
    assert!(detached[0].event.id < retired[0].0.id);

    // The staff order is for task force 1 and cites the retirement.
    assert_eq!(run.world().orders().count(), 4);
    let staff = run.staff_orders();
    assert_eq!(staff.len(), 1);
    assert_eq!(staff[0].id, OrderId::new(4));
    assert_eq!(
        staff[0].kind,
        OrderKind::TaskForce {
            target: TF1,
            order: TaskForceOrder::SetRoute(vec![hex(0, 0)]),
        }
    );
    let accepted = run.accepted_event(staff[0].id);
    assert_eq!(accepted.len(), 1);
    assert_eq!(
        accepted[0].causes,
        vec![Cause::Order(staff[0].id), Cause::Event(retired[0].0.id)]
    );

    assert_eq!(
        run.forces(),
        BTreeMap::from([(TF1, set(&[1, 2, 3, 4, 5])), (TF2, set(&[6]))])
    );
    assert_eq!(run.force(TF1).rules_of_engagement.stance, Stance::Avoid);
    assert_eq!(
        run.force(TF1).rules_of_engagement.stragglers,
        StragglerInstruction::Keep
    );
    let steams = run.steams(TF1);
    assert_movement(&steams, &home_rows(), hex(6, 0), -1, 8);
    for steam in &steams {
        assert!(steam.causes.contains(&Cause::Order(staff[0].id)));
    }
    assert_eq!(run.arrivals(TF1), vec![n(30)]);
    assert_eq!(run.force(TF1).status, TaskForceStatus::Arrived);
    assert_stopped_at(run.world(), TF1, hex(0, 0));
    for id in [1, 2, 3] {
        assert_eq!(run.total_burned(id), n(60));
        assert_eq!(run.fuel(id), n(1940));
    }
    for id in [4, 5] {
        assert_eq!(run.total_burned(id), n(30));
        assert_eq!(run.fuel(id), n(470));
    }
    assert_idle_force(run, TF2, &[(6, 500)]);
}

#[test]
fn a_share_to_keep_of_0_8_retires_the_force() {
    // 3 of 4 is 0.75, below 0.8.
    assert_force_retires(&run(Case::ShareToKeep08));
}

#[test]
fn the_retire_instruction_retires_the_force() {
    // The same, whatever the share to keep (here 0.5, which would detach).
    assert_force_retires(&run(Case::Retire));
}

#[test]
fn the_leave_instruction_leaves_both_stragglers_together() {
    let run = run(Case::Leave);
    let detached = run.detachments();
    assert_eq!(detached.len(), 1);
    assert_eq!(detached[0].parent, TF1);
    assert_eq!(detached[0].new_force, TF2);
    assert_eq!(detached[0].ships, ships(&[2, 6]));
    assert!(detached[0].escorts.is_empty());
    assert_eq!(detached[0].reason, DetachReason::LeftBehind);
    assert!(run.retirements().is_empty());
    // No staff order: nothing is sent anywhere.
    assert!(run.staff_orders().is_empty());
    assert_eq!(run.world().orders().count(), 3);
    assert_eq!(
        run.forces(),
        BTreeMap::from([(TF1, set(&[1, 3, 4, 5])), (TF2, set(&[2, 6]))])
    );
    assert_idle_force(&run, TF2, &[(2, 2000), (6, 500)]);
    // Task force 1 steams east at 16 kn and arrives at hour 15.
    assert_movement(&run.steams(TF1), &tf1_rows(), hex(6, 0), 1, 16);
    assert_eq!(run.arrivals(TF1), vec![n(15)]);
    assert_stopped_at(run.world(), TF1, hex(12, 0));
    for id in [1, 3] {
        assert_eq!(run.total_burned(id), n(60));
        assert_eq!(run.fuel(id), n(1940));
    }
    for id in [4, 5] {
        assert_eq!(run.total_burned(id), n(30));
        assert_eq!(run.fuel(id), n(470));
    }
}

#[test]
fn the_keep_instruction_detaches_nothing_and_goes_nowhere() {
    let run = run(Case::Keep);
    assert!(run.detachments().is_empty());
    assert!(run.retirements().is_empty());
    assert!(run.staff_orders().is_empty());
    assert_eq!(
        run.forces(),
        BTreeMap::from([(TF1, set(&[1, 2, 3, 4, 5, 6]))])
    );
    // The slowest ship is dead in the water: no movement for two days, no
    // fuel burned, no arrival. The force stays under way.
    assert!(run.all_steams().is_empty());
    assert!(run
        .events
        .iter()
        .all(|event| !matches!(event.kind, EventKind::FuelBurned { .. })));
    assert!(run.arrivals(TF1).is_empty());
    for (id, fuel) in [
        (1, 2000),
        (2, 2000),
        (3, 2000),
        (4, 500),
        (5, 500),
        (6, 500),
    ] {
        assert_eq!(run.fuel(id), n(fuel));
    }
    assert_eq!(run.force(TF1).status, TaskForceStatus::Underway);
    assert_stopped_at(run.world(), TF1, hex(6, 0));
    // Only the three orders of intake were logged, all on day 1.
    assert_eq!(run.day_event_counts, vec![3, 0]);
}

#[test]
fn with_no_home_port_task_force_2_is_made_but_not_sent() {
    let run = run(Case::NoHomePort);
    let detached = run.detachments();
    assert_eq!(detached.len(), 2);
    assert_eq!(
        (
            detached[0].new_force,
            detached[0].ships.clone(),
            detached[0].escorts.clone()
        ),
        (TF2, ships(&[2]), ships(&[4]))
    );
    assert_eq!(detached[0].reason, DetachReason::SentHome);
    assert_eq!(
        (
            detached[1].new_force,
            detached[1].ships.clone(),
            detached[1].escorts.clone()
        ),
        (TF3, ships(&[6]), vec![])
    );
    assert_eq!(detached[1].reason, DetachReason::LeftBehind);
    // No staff order, and nothing waits on one.
    assert!(run.staff_orders().is_empty());
    assert_eq!(run.world().orders().count(), 3);
    assert!(run.retirements().is_empty());
    assert_eq!(run.force(TF2).home_port, None);
    assert_eq!(
        run.forces(),
        BTreeMap::from([
            (TF1, set(&[1, 3, 5])),
            (TF2, set(&[2, 4])),
            (TF3, set(&[6]))
        ])
    );
    // Task force 2 stays at (6,0): no route, no movement, no fuel.
    assert_idle_force(&run, TF2, &[(2, 2000), (4, 500)]);
    assert_eq!(run.force(TF2).route_order, None);
    assert_idle_force(&run, TF3, &[(6, 500)]);
    // Task force 1 steams east.
    assert_movement(&run.steams(TF1), &tf1_rows(), hex(6, 0), 1, 16);
    assert_eq!(run.arrivals(TF1), vec![n(15)]);
    assert_stopped_at(run.world(), TF1, hex(12, 0));
    assert_eq!(run.fuel(1), n(1940));
    assert_eq!(run.fuel(3), n(1940));
    assert_eq!(run.fuel(5), n(470));
}

#[test]
fn with_destroyer_6_undamaged_nothing_is_left_behind() {
    let run = run(Case::Destroyer6Undamaged);
    let detached = run.detachments();
    assert_eq!(detached.len(), 1);
    assert_eq!(detached[0].parent, TF1);
    assert_eq!(detached[0].new_force, TF2);
    assert_eq!(detached[0].ships, ships(&[2]));
    assert_eq!(detached[0].escorts, ships(&[4]));
    assert_eq!(detached[0].reason, DetachReason::SentHome);
    assert!(run.retirements().is_empty());
    // No force is left behind: only two forces exist.
    assert_eq!(
        run.forces(),
        BTreeMap::from([(TF1, set(&[1, 3, 5, 6])), (TF2, set(&[2, 4]))])
    );
    // Cruiser 2 goes home with destroyer 4 on a staff order, as before.
    let staff = run.staff_orders();
    assert_eq!(staff.len(), 1);
    assert_eq!(staff[0].id, OrderId::new(4));
    let steams = run.steams(TF2);
    assert_movement(&steams, &home_rows(), hex(6, 0), -1, 8);
    for steam in &steams {
        assert!(steam.causes.contains(&Cause::Order(staff[0].id)));
    }
    assert_eq!(run.arrivals(TF2), vec![n(30)]);
    assert_stopped_at(run.world(), TF2, hex(0, 0));
    assert_eq!(run.fuel(2), n(1940));
    assert_eq!(run.fuel(4), n(470));
    // Task force 1, ships 1, 3, 5 and 6, steams east at 16 kn.
    assert_movement(&run.steams(TF1), &tf1_rows(), hex(6, 0), 1, 16);
    assert_eq!(run.arrivals(TF1), vec![n(15)]);
    assert_stopped_at(run.world(), TF1, hex(12, 0));
    for id in [1, 3] {
        assert_eq!(run.fuel(id), n(1940));
    }
    for id in [5, 6] {
        assert_eq!(run.total_burned(id), n(30));
        assert_eq!(run.fuel(id), n(470));
    }
}

#[test]
fn with_all_three_cruisers_damaged_the_force_retires() {
    // Three escorts are needed and only destroyers 4 and 5 can give them,
    // which would leave the parent none.
    let run = run(Case::CruisersDamaged);
    for id in [1, 2, 3] {
        assert_eq!(
            run.world().umpire().ships[&ship(id)].systems.propulsion,
            SystemState::Degraded
        );
    }
    assert_force_retires(&run);
}

// ---------------------------------------------------------------------
// Properties.
// ---------------------------------------------------------------------

/// No ship is in two task forces, and every ship afloat is in one.
fn check_membership(run: &Run) {
    let truth = run.world().umpire();
    let mut seen: BTreeSet<ShipId> = BTreeSet::new();
    for (id, force) in &truth.task_forces {
        for ship_id in &force.ships {
            assert!(seen.insert(*ship_id), "{ship_id} is in two task forces");
            assert_eq!(truth.ships[ship_id].location, ShipLocation::TaskForce(*id));
        }
        assert!(!force.ships.is_empty(), "{id} is empty");
    }
    for (id, ship_data) in &truth.ships {
        // Nothing sinks in this scenario, so every ship is afloat in a force.
        assert_ne!(ship_data.location, ShipLocation::Sunk);
        assert!(seen.contains(id), "{id} is in no task force");
    }
    assert_eq!(seen.len(), truth.ships.len());
}

/// A task force made for stragglers holds only stragglers and its escorts.
fn check_straggler_forces(run: &Run) {
    for detached in run.detachments() {
        let force = &run.force(detached.new_force).ships;
        let expected: BTreeSet<ShipId> = detached
            .ships
            .iter()
            .chain(&detached.escorts)
            .copied()
            .collect();
        assert_eq!(force, &expected);
        for id in &detached.ships {
            assert!(run.is_straggler(*id), "{id} is not a straggler");
        }
        for id in &detached.escorts {
            assert!(!run.is_straggler(*id), "escort {id} is a straggler");
        }
        // Only a force that sends ships home has escorts.
        if detached.reason == DetachReason::LeftBehind {
            assert!(detached.escorts.is_empty());
        }
    }
}

/// A parent that detached has no straggler left. A parent that retired
/// keeps the stragglers that can steam, so it has none dead in the water.
fn check_parents(run: &Run) {
    let retired: BTreeSet<TaskForceId> = run.retirements().iter().map(|r| r.1).collect();
    let parents: BTreeSet<TaskForceId> = run.detachments().iter().map(|d| d.parent).collect();
    for parent in parents {
        for id in &run.force(parent).ships {
            let propulsion = run.world().umpire().ships[id].systems.propulsion;
            if retired.contains(&parent) {
                assert_ne!(propulsion, SystemState::Out, "{id} left in {parent}");
            } else {
                assert!(!run.is_straggler(*id), "{id} left in {parent}");
            }
        }
    }
}

/// The instruction is applied once: everything is in the first pass, before
/// any movement, and nothing is detached or retired later.
fn check_applied_once(run: &Run) {
    let passes: Vec<&Event> = run
        .events
        .iter()
        .filter(|event| {
            matches!(
                event.kind,
                EventKind::ShipsDetached { .. } | EventKind::ForceRetired { .. }
            )
        })
        .collect();
    let first_movement = run.all_steams().first().map(|event| event.id);
    for event in &passes {
        assert_eq!((event.day, event.pulse), (1, Some(Pulse::Night)));
        if let Some(first) = first_movement {
            assert!(event.id < first);
        }
    }
    // Each new force is made once, and each force retires at most once.
    let new_forces: Vec<TaskForceId> = run.detachments().iter().map(|d| d.new_force).collect();
    assert_eq!(
        new_forces.len(),
        new_forces.iter().collect::<BTreeSet<_>>().len()
    );
    assert!(run.retirements().len() <= 1);
    // Forces made by the pass are never parents themselves.
    for detached in run.detachments() {
        assert!(!new_forces.contains(&detached.parent));
    }
}

/// Every movement of a force sent home cites the staff order, and the
/// order's acceptance cites the event that detached or retired the force.
fn check_staff_orders(run: &Run) {
    let mut sent_home: BTreeMap<TaskForceId, EventId> = BTreeMap::new();
    for detached in run.detachments() {
        if detached.reason == DetachReason::SentHome
            && run.force(detached.new_force).home_port.is_some()
        {
            sent_home.insert(detached.new_force, detached.event.id);
        }
    }
    for (event, force) in run.retirements() {
        if run.force(force).home_port.is_some() {
            sent_home.insert(force, event.id);
        }
    }
    let staff = run.staff_orders();
    assert_eq!(staff.len(), sent_home.len());
    for order in staff {
        let OrderKind::TaskForce {
            target,
            order: ref kind,
        } = order.kind
        else {
            panic!("the staff order is for a task force");
        };
        assert_eq!(*kind, TaskForceOrder::SetRoute(vec![hex(0, 0)]));
        let cause = sent_home.get(&target).expect("a force sent home");
        let accepted = run.accepted_event(order.id);
        assert_eq!(accepted.len(), 1);
        assert_eq!(
            accepted[0].causes,
            vec![Cause::Order(order.id), Cause::Event(*cause)]
        );
        assert!(*cause < accepted[0].id);
        // The orders list has no rejection of it.
        assert!(run.events.iter().all(|event| !matches!(
            &event.kind,
            EventKind::OrderRejected { order: o, .. } if *o == order.id
        )));
        let steams = run.steams(target);
        assert!(!steams.is_empty());
        for steam in steams {
            assert!(steam.causes.contains(&Cause::Order(order.id)));
        }
    }
}

#[test]
fn no_ship_is_in_two_task_forces_and_every_ship_is_in_one() {
    for case in ALL_CASES {
        check_membership(&run(case));
    }
}

#[test]
fn a_straggler_force_holds_only_stragglers_and_their_escorts() {
    for case in ALL_CASES {
        check_straggler_forces(&run(case));
    }
}

#[test]
fn a_parent_that_detached_has_no_straggler_left() {
    for case in ALL_CASES {
        check_parents(&run(case));
    }
}

#[test]
fn the_instruction_is_applied_once() {
    for case in ALL_CASES {
        check_applied_once(&run(case));
    }
}

#[test]
fn movement_of_a_force_sent_home_cites_the_staff_order() {
    for case in ALL_CASES {
        check_staff_orders(&run(case));
    }
}

#[test]
fn a_replay_reproduces_both_daily_hashes() {
    for case in ALL_CASES {
        let first = run(case);
        let second = run(case);
        let hashes = first.world().day_hashes().to_vec();
        assert_eq!(hashes.len(), 2);
        assert_eq!(hashes, second.world().day_hashes());
        assert_eq!(first.events, second.events);
        // A copy with the file's own seed gives the same hashes.
        let mut copy = build(case).with_seed(1942).expect("a seed can be set");
        copy.run_to_end().expect("the run resolves");
        assert_eq!(hashes, copy.world.day_hashes());
    }
    // The cases are different histories.
    let main = run(Case::Main).world().day_hashes().to_vec();
    let retired = run(Case::Retire).world().day_hashes().to_vec();
    assert_ne!(main, retired);
}
