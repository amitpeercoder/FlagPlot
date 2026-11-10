//! Scenario 4b: a task force finds its own way round land, and the real map
//! of the slice.
//!
//! Stage 4b of scenario 4. It proves that a route order needs no waypoints
//! round an obstacle, that the path taken is a shortest one and always the
//! same one, that the projection puts a point in the right hex, and that the
//! generated map of the slice is sound.
//!
//! The rule is `docs/decisions/0017-map-and-pathfinding.md`.
//!
//! # Hand calculation
//!
//! Written by the architect before the code. The tests below assert these
//! numbers exactly. If the code disagrees, the code or the rule is wrong: do
//! not change a number here to make a test pass.
//!
//! ## Part 1: round the island
//!
//! Data: `data/scenarios/04b_round_the_island.toml`, placeholder.
//!
//! - Open sea from q = -1 to 9 and r = -3 to 3. A wall of land at (4,-1),
//!   (4,0) and (4,1).
//! - Task force 1, a cruiser and a destroyer, starts at (0,0) and is ordered
//!   to (8,0) at cruise speed, 16 kn. The route has that one waypoint.
//! - The six directions, in their fixed order, are (+1,0), (+1,-1), (0,-1),
//!   (-1,0), (-1,+1), (0,+1).
//!
//! The straight line is 8 hexes and runs through (4,0), which is land. Each
//! step changes q by one at most, so any path crosses q = 4, and it can do
//! so only at r = -3, -2, 2 or 3. Through (4,-2) the shortest way is 4 steps
//! there and 6 on: 10. Through (4,2) it is 6 steps there and 4 on: 10.
//! Through (4,-3) or (4,3) it is longer. So the shortest path is 10 steps,
//! 400 nm, and there is a tie between north and south.
//!
//! The rule breaks the tie: from each hex, take the first direction, in the
//! fixed order, that leads to a hex one step nearer the target.
//!
//! | From   | Steps left | First direction that helps | Why the earlier ones do not       |
//! | ------ | ---------- | -------------------------- | --------------------------------- |
//! | (0,0)  | 10         | (+1,0) to (1,0)            |                                   |
//! | (1,0)  | 9          | (+1,0) to (2,0)            |                                   |
//! | (2,0)  | 8          | (+1,0) to (3,0)            |                                   |
//! | (3,0)  | 7          | (0,+1) to (3,1)            | (4,0) and (4,-1) are land; (3,-1) is 7 from the target; (2,0) is 8; (2,1) is 7 |
//! | (3,1)  | 6          | (0,+1) to (3,2)            | (4,1) and (4,0) are land; (3,0) is 7; (2,1) is 7; (2,2) is 6 |
//! | (3,2)  | 5          | (+1,0) to (4,2)            |                                   |
//! | (4,2)  | 4          | (+1,0) to (5,2)            |                                   |
//! | (5,2)  | 3          | (+1,0) to (6,2)            |                                   |
//! | (6,2)  | 2          | (+1,-1) to (7,1)           | (7,2) is 2 from the target        |
//! | (7,1)  | 1          | (+1,-1) to (8,0)           | (8,1) is 1 from the target        |
//!
//! So the task force goes south of the island: (1,0), (2,0), (3,0), (3,1),
//! (3,2), (4,2), (5,2), (6,2), (7,1), (8,0).
//!
//! It steams 192 nm in the night pulse and 96 nm in each day pulse. It
//! changes hex 20 nm past each centre (decision 0005).
//!
//! | Pulse           | Total  | Hex left, miles on | Position | Hexes entered                          |
//! | --------------- | ------ | ------------------ | -------- | -------------------------------------- |
//! | Day 1 night     | 192 nm | (3,1) plus 32 nm   | (3,2)    | (1,0), (2,0), (3,0), (3,1), (3,2)      |
//! | Day 1 morning   | 288 nm | (5,2) plus 8 nm    | (5,2)    | (4,2), (5,2)                           |
//! | Day 1 afternoon | 384 nm | (7,1) plus 24 nm   | (8,0)    | (6,2), (7,1), (8,0)                    |
//! | Day 2 night     | 400 nm | (8,0), arrived     | (8,0)    | none                                   |
//!
//! It arrives 25 hours after the start, at 19:00 on day 2. Fuel: the cruiser
//! burns 400 x 0.25 = 100 t (48, 24, 24 and 4 by pulse) and ends with 1900 t.
//! The destroyer burns 400 x 0.125 = 50 t (24, 12, 12 and 2) and ends with
//! 450 t.
//!
//! The same scenario with one thing changed:
//!
//! | Change                                   | What happens                                    |
//! | ---------------------------------------- | ----------------------------------------------- |
//! | The wall runs the whole height, r = -3 to 3 | The order is rejected: (8,0) cannot be reached. The task force stays idle at (0,0). |
//! | The waypoint is (4,0), which is land     | The order is rejected: (4,0) cannot be reached. |
//! | The waypoint is (20,0), off the map      | The order is rejected: the waypoint is off the map. |
//! | No wall                                  | 8 steps along r = 0: (1,0) to (8,0). 320 nm, arriving at hour 20. This is the path the rule gave before pathfinding. |
//! | Waypoints (3,-2) then (8,0)              | North of the island: (1,0), (2,-1), (3,-2), then (4,-2), (5,-2), (6,-2), (7,-2), (8,-2), (8,-1), (8,0). 3 steps and then 7: 400 nm, arriving at hour 25. |
//!
//! For the last row, the first leg: (3,-2) is 3 steps from (0,0). (+1,0)
//! leads to (1,0), which is 2 away, so it is taken. From (1,0), (+1,0) leads
//! to (2,0), still 2 away; (+1,-1) leads to (2,-1), 1 away. From (2,-1),
//! (+1,0) leads to (3,-1), 1 away; (+1,-1) leads to (3,-2).
//!
//! The second leg: (8,0) is 7 steps from (3,-2) (5 east and 2 south-east
//! cannot share steps, because no direction is (+1,+1)). (+1,0) helps five
//! times, to (4,-2), (5,-2), (6,-2), (7,-2) and (8,-2), which is 2 away.
//! From (8,-2) the first five directions lead to hexes 2 or 3 away, and
//! (0,+1) leads to (8,-1), 1 away. From (8,-1), again only (0,+1) helps, to
//! (8,0).
//!
//! ## Part 2: the projection
//!
//! Hex (0,0) is centred at 12 S, 160 E. East, in nm, is (longitude - 160) x
//! 58.68888. North, in nm, is (latitude + 12) x 60. The centre of hex (q, r)
//! lies 40 x (q + r/2) nm east and 34.641016 x r nm south. A point is in the
//! hex whose centre is nearest.
//!
//! | Point        | East       | North    | Nearest centre                         | Hex      |
//! | ------------ | ---------- | -------- | -------------------------------------- | -------- |
//! | 12 S, 160 E  | 0          | 0        | (0,0), at 0 east, 0 north              | (0,0)    |
//! | 10 S, 162 E  | 117.378 nm | 120 nm   | (5,-4), at 120 east, 138.564 north: 18.7 nm away | (5,-4)   |
//! | 15 S, 157 E  | -176.067 nm| -180 nm  | (-7,5), at -180 east, -173.205 north: 7.9 nm away | (-7,5)   |
//! | 4 S, 152 E   | -469.511 nm| 480 nm   | (-5,-14), at -480 east, 484.974 north: 11.6 nm away | (-5,-14) |
//!
//! For 10 S, 162 E the other candidates are (4,-3), at 100 east and 103.923
//! north, 23.7 nm away, and (5,-3), at 140 east, 27.8 nm away.
//!
//! The centre of hex (5,-4) is at latitude -12 + 138.564 / 60 = -9.6906 and
//! longitude 160 + 120 / 58.68888 = 162.0447.
//!
//! ## Part 3: the real map
//!
//! Written by the architect after the map was generated and inspected, and
//! before the tests for it. Data: `data/map/south_pacific.csv`, generated
//! from Natural Earth coastlines with the anchorages in
//! `data/map/south_pacific_anchors.csv`, and
//! `data/scenarios/04b_santo_to_lunga.toml`.
//!
//! ### Where the anchorages fall
//!
//! From the sourced positions in the anchors file, by the projection of
//! Part 2. Distances are from the hex centre.
//!
//! | Place                         | Latitude, longitude    | East       | North      | Hex      | From centre |
//! | ----------------------------- | ---------------------- | ---------- | ---------- | -------- | ----------- |
//! | Rabaul, Simpson Harbour       | -4.22224, 152.16544    | -459.80 nm | 466.67 nm  | (-5,-13) | 16.3 nm     |
//! | Tulagi Harbour                | -9.09975, 160.15356    | 9.01 nm    | 174.02 nm  | (3,-5)   | 11.0 nm     |
//! | Lunga Point                   | -9.40538, 160.02861    | 1.68 nm    | 155.68 nm  | (2,-4)   | 17.2 nm     |
//! | Espiritu Santo, Segond Channel| -15.55075, 167.15375   | 419.85 nm  | -213.05 nm | (7,6)    | 20.5 nm     |
//! | Noumea, Grande Rade           | -22.25675, 166.42019   | 376.79 nm  | -615.41 nm | (0,18)   | 18.7 nm     |
//!
//! Segond Channel is a close call: it is 19.85 nm east of the centre of
//! (7,6) and 20.15 nm west of the centre of (8,6), so it falls in (7,6) by
//! 0.15 nm. Savo Island's peak, at -9.1303, 159.81619, is in (2,-5), 9.3 nm
//! from the centre.
//!
//! Each of these hexes is coast, so ships can enter it.
//!
//! ### What is open and what is closed
//!
//! Read from the generated map with the map tool, then checked against what
//! the geography should be. The test asserts each.
//!
//! - Lunga Point's hex, (2,-4), stands for Ironbottom Sound, because the
//!   anchorage is on that side. It is open toward Tulagi's hex (3,-5) and
//!   Savo's hex (2,-5), and closed on its other four sides.
//! - From Lunga Point to the open sea just south of Guadalcanal, (2,-3), is
//!   1 hex in a straight line and 3 by sea, round the east end.
//! - New Britain cannot be crossed: from (-9,-12) on its north coast to
//!   (-10,-9) on its south coast is 3 hexes in a straight line and 7 by sea.
//! - New Guinea cannot be crossed: from Rabaul's hex to (-17,-4), on the
//!   south coast near Port Moresby, is 12 hexes in a straight line and 17 by
//!   sea, round the eastern tip.
//! - The Slot is open: from (-1,-9), in Bougainville Strait, to Savo's hex
//!   is 7 hexes by sea, the same as the straight line.
//! - Every hex that is not land can be reached from (0,0): 1,520 hexes.
//!   There are 1,600 hexes in all: 80 land, 210 coast and 1,310 deep sea.
//! - Every closed side is closed from both hexes.
//!
//! ### Distances between anchorages
//!
//! Hex steps overstate true distance. A step is 40 nm in one of six
//! directions, so a course between two of them zigzags: due north or south
//! costs about 15 percent more than the straight line. Land adds more. The
//! test asserts each sea path is no shorter than the straight line between
//! the two sourced positions, and no more than 30 percent longer.
//!
//! | From, to                        | Straight line, from the positions | By sea             | Over  |
//! | ------------------------------- | --------------------------------- | ------------------ | ----- |
//! | Rabaul to Lunga Point           | 461.5 east, 311.0 south: 556.5 nm | 16 hexes, 640 nm   | 15 %  |
//! | Espiritu Santo to Lunga Point   | 418.2 west, 368.7 north: 557.5 nm | 16 hexes, 640 nm   | 15 %  |
//! | Noumea to Espiritu Santo        | 43.1 east, 402.4 north: 404.7 nm  | 13 hexes, 520 nm   | 28 %  |
//!
//! ### The transit
//!
//! Task force 1, a cruiser and a destroyer, starts in (7,6) and is ordered
//! to (2,-4) at cruise speed, 16 kn, with that one waypoint.
//!
//! The path is 16 steps. It cannot be worked by hand as Part 1 was, because
//! it depends on 1,600 hexes of generated map. It is taken from the map
//! tool, and the test checks it by the properties below: every step is to a
//! neighbour across an open side and is one step nearer by sea.
//!
//! (7,5), (7,4), (7,3), (7,2), (7,1), (7,0), (7,-1), (7,-2), (7,-3), (7,-4),
//! (6,-4), (5,-4), (4,-4), (4,-5), (3,-5), (2,-4).
//!
//! It runs north up q = 7, west along r = -4, then north-west into
//! Indispensable Strait, and enters Lunga Point's hex from Tulagi's, which
//! is the only way in from the east. The straight line is 15 hexes.
//!
//! | Pulse           | Total  | Hex left, miles on | Position |
//! | --------------- | ------ | ------------------ | -------- |
//! | Day 1 night     | 192 nm | (7,2) plus 32 nm   | (7,1)    |
//! | Day 1 morning   | 288 nm | (7,-1) plus 8 nm   | (7,-1)   |
//! | Day 1 afternoon | 384 nm | (7,-3) plus 24 nm  | (7,-4)   |
//! | Day 2 night     | 576 nm | (4,-5) plus 16 nm  | (4,-5)   |
//! | Day 2 morning   | 640 nm | (2,-4), arrived    | (2,-4)   |
//!
//! 640 nm at 16 kn is 40 hours: it arrives at 10:00 on day 2. The cruiser
//! burns 640 x 0.25 = 160 t and ends with 1840 t. The destroyer burns 640 x
//! 0.125 = 80 t and ends with 420 t.
//!
//! # Properties
//!
//! - Every step of a path is to a neighbouring hex that ships can enter, and
//!   is one step nearer the target.
//! - A path is never shorter than the straight-line hex count, and equals it
//!   on open water.
//! - The same order on the same map always gives the same path.
//! - Scenarios 1 to 4a2 keep every number: on their maps the shortest path
//!   is the path they already took.

use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};

use sim_core::events::{Event, EventKind};
use sim_core::fixed::Fixed;
use sim_core::hex::Hex;
use sim_core::ids::{ShipId, TaskForceId};
use sim_core::map::Map;
use sim_core::orders::RejectReason;
use sim_core::state::TaskForceStatus;
use sim_core::time::{ClockTime, Pulse};
use sim_core::World;
use sim_data::Scenario;
use std::collections::BTreeMap;

const TASK_FORCE: TaskForceId = TaskForceId::new(1);
const CRUISER: ShipId = ShipId::new(1);
const DESTROYER: ShipId = ShipId::new(2);

fn hex(q: i32, r: i32) -> Hex {
    Hex::new(q, r)
}

fn nm(value: i64) -> Fixed {
    Fixed::from_int(value)
}

// ---------------------------------------------------------------------
// Building the scenario and its variants.
// ---------------------------------------------------------------------

const MAIN_FILE: &str = "scenarios/04b_round_the_island.toml";
const MAP_FILE: &str = "map/placeholder_island.toml";
const ROUTE: &str = "waypoints = [[8, 0]]";

fn data_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../data")
}

fn load() -> Scenario {
    Scenario::load(&data_dir().join(MAIN_FILE)).expect("scenario 4b loads")
}

/// The wall of land across the whole height, as extra hex blocks.
fn land_block(q: i32, r: i32) -> String {
    format!("\n[[hex]]\nq = {q}\nr = {r}\nkind = \"land\"\nterrain = \"jungle\"\n")
}

/// A change to the data: appended to the map file, and exact replacements
/// in the scenario file.
#[derive(Clone, Default)]
struct Change {
    map_append: String,
    /// Cut the three wall hexes out of the map.
    remove_wall: bool,
    route: Option<&'static str>,
}

static VARIANT_COUNT: AtomicUsize = AtomicUsize::new(0);

/// Copies the scenario and its data into a folder of its own under the build
/// directory, changes the copies, and loads them. Nothing in the repository's
/// data folder is touched.
fn variant(change: &Change) -> Scenario {
    let data = data_dir();
    let id = VARIANT_COUNT.fetch_add(1, Ordering::SeqCst);
    let root = Path::new(env!("CARGO_TARGET_TMPDIR")).join(format!("pathfinding_{id}"));
    for folder in ["scenarios", "map", "reference"] {
        fs::create_dir_all(root.join(folder)).expect("make the variant folder");
    }
    for file in [
        "reference/ship_classes.csv",
        "reference/port_sizes.csv",
        "reference/aircraft_types.csv",
        "reference/search.toml",
        "reference/surface_combat.toml",
    ] {
        fs::copy(data.join(file), root.join(file)).expect("copy a data file");
    }

    let mut map_text = fs::read_to_string(data.join(MAP_FILE)).expect("read the map");
    if change.remove_wall {
        for r in -1..=1 {
            let block = land_block(4, r);
            let block = block.trim_start_matches('\n');
            assert_eq!(map_text.matches(block).count(), 1, "wall hex (4,{r})");
            map_text = map_text.replace(block, "");
        }
    }
    map_text.push_str(&change.map_append);
    fs::write(root.join(MAP_FILE), map_text).expect("write the variant map");

    let mut text = fs::read_to_string(data.join(MAIN_FILE)).expect("read the scenario");
    if let Some(route) = change.route {
        assert_eq!(text.matches(ROUTE).count(), 1, "the route appears once");
        text = text.replace(ROUTE, route);
    }
    fs::write(root.join(MAIN_FILE), text).expect("write the variant scenario");
    Scenario::load(&root.join(MAIN_FILE)).expect("the variant loads")
}

fn walled() -> Change {
    Change {
        map_append: [-3, -2, 2, 3].map(|r| land_block(4, r)).concat(),
        ..Change::default()
    }
}

fn route(waypoints: &'static str) -> Change {
    Change {
        route: Some(waypoints),
        ..Change::default()
    }
}

fn no_wall() -> Change {
    Change {
        remove_wall: true,
        ..Change::default()
    }
}

// ---------------------------------------------------------------------
// Reading what happened.
// ---------------------------------------------------------------------

/// A finished run: every event, by day.
struct Run {
    scenario: Scenario,
    days: Vec<Vec<Event>>,
}

impl Run {
    fn events(&self) -> impl Iterator<Item = &Event> {
        self.days.iter().flatten()
    }
}

fn run(mut scenario: Scenario) -> Run {
    let mut days = Vec::new();
    while let Some(summary) = scenario.next_day().expect("the day resolves") {
        let events = scenario.world.events().from_index(summary.first_event);
        assert_eq!(events.len(), summary.event_count);
        days.push(events.to_vec());
    }
    Run { scenario, days }
}

/// Movement events as (day, pulse, distance, from, to, hexes entered).
type Steam = (usize, Pulse, Fixed, Hex, Hex, Vec<Hex>);

fn steams(run: &Run) -> Vec<Steam> {
    let mut out = Vec::new();
    for (index, events) in run.days.iter().enumerate() {
        for event in events {
            if let EventKind::TaskForceSteamed {
                distance_nm,
                from,
                to,
                hexes_entered,
                ..
            } = &event.kind
            {
                out.push((
                    index + 1,
                    event.pulse.expect("movement has a pulse"),
                    *distance_nm,
                    *from,
                    *to,
                    hexes_entered.clone(),
                ));
            }
        }
    }
    out
}

/// Every hex entered, in order, across all pulses.
fn path(run: &Run) -> Vec<Hex> {
    steams(run).into_iter().flat_map(|steam| steam.5).collect()
}

fn total_nm(run: &Run) -> Fixed {
    steams(run)
        .into_iter()
        .fold(Fixed::ZERO, |sum, steam| sum + steam.2)
}

/// Fuel burned by a ship as (day, pulse, tons).
fn burns(run: &Run, ship: ShipId) -> Vec<(usize, Pulse, Fixed)> {
    let mut out = Vec::new();
    for (index, events) in run.days.iter().enumerate() {
        for event in events {
            if let EventKind::FuelBurned {
                ship: burned,
                burned_t,
                ..
            } = &event.kind
            {
                if *burned == ship {
                    out.push((index + 1, event.pulse.expect("burn has a pulse"), *burned_t));
                }
            }
        }
    }
    out
}

fn arrivals(run: &Run) -> Vec<(Hex, Fixed)> {
    run.events()
        .filter_map(|event| match &event.kind {
            EventKind::TaskForceArrived {
                hex, elapsed_hours, ..
            } => Some((*hex, *elapsed_hours)),
            _ => None,
        })
        .collect()
}

fn rejections(run: &Run) -> Vec<RejectReason> {
    run.events()
        .filter_map(|event| match &event.kind {
            EventKind::OrderRejected { reason, .. } => Some(reason.clone()),
            _ => None,
        })
        .collect()
}

fn assert_leg(world: &World, from: Hex, toward: Option<Hex>, progress: i64, position: Hex) {
    let task_force = &world.umpire().task_forces[&TASK_FORCE];
    assert_eq!(task_force.from_hex, from);
    assert_eq!(task_force.toward_hex, toward);
    assert_eq!(task_force.progress_nm, nm(progress));
    assert_eq!(task_force.position(), position);
}

// ---------------------------------------------------------------------
// Part 1: round the island.
// ---------------------------------------------------------------------

#[test]
fn the_task_force_goes_round_the_island_as_calculated() {
    let mut scenario = load();
    assert!(scenario.placeholder);
    assert_eq!(scenario.days, 2);
    {
        let world = &scenario.world;
        assert_leg(world, hex(0, 0), None, 0, hex(0, 0));
        assert_eq!(world.umpire().ships[&CRUISER].fuel_t, nm(2000));
        assert_eq!(world.umpire().ships[&DESTROYER].fuel_t, nm(500));
    }

    // Day 1. Only the end-of-day state can be read from the world (there is
    // no per-pulse stepping), so the per-pulse figures below come from the
    // events, and the stored leg is checked at the end of each day.
    let mut days = Vec::new();
    let day1 = scenario.next_day().unwrap().unwrap();
    days.push(
        scenario
            .world
            .events()
            .from_index(day1.first_event)
            .to_vec(),
    );
    // After the afternoon pulse: left (7,1) with 24 nm on, in (8,0).
    assert_leg(&scenario.world, hex(7, 1), Some(hex(8, 0)), 24, hex(8, 0));
    assert_eq!(
        scenario.world.umpire().task_forces[&TASK_FORCE].status,
        TaskForceStatus::Underway
    );
    assert_eq!(scenario.world.umpire().ships[&CRUISER].fuel_t, nm(1900 + 4));
    assert_eq!(
        scenario.world.umpire().ships[&DESTROYER].fuel_t,
        nm(450 + 2)
    );

    let day2 = scenario.next_day().unwrap().unwrap();
    days.push(
        scenario
            .world
            .events()
            .from_index(day2.first_event)
            .to_vec(),
    );
    // After the night pulse of day 2: arrived at (8,0), no leg.
    assert_leg(&scenario.world, hex(8, 0), None, 0, hex(8, 0));
    assert_eq!(
        scenario.world.umpire().task_forces[&TASK_FORCE].status,
        TaskForceStatus::Arrived
    );
    assert_eq!(scenario.world.umpire().ships[&CRUISER].fuel_t, nm(1900));
    assert_eq!(scenario.world.umpire().ships[&DESTROYER].fuel_t, nm(450));
    assert!(scenario.next_day().unwrap().is_none());

    let run = Run { scenario, days };

    // Nothing was rejected.
    assert!(rejections(&run).is_empty());

    // The path, hex by hex.
    assert_eq!(
        path(&run),
        vec![
            hex(1, 0),
            hex(2, 0),
            hex(3, 0),
            hex(3, 1),
            hex(3, 2),
            hex(4, 2),
            hex(5, 2),
            hex(6, 2),
            hex(7, 1),
            hex(8, 0),
        ]
    );

    // Pulse by pulse: miles, position after, hexes entered.
    assert_eq!(
        steams(&run),
        vec![
            (
                1,
                Pulse::Night,
                nm(192),
                hex(0, 0),
                hex(3, 2),
                vec![hex(1, 0), hex(2, 0), hex(3, 0), hex(3, 1), hex(3, 2)]
            ),
            (
                1,
                Pulse::Morning,
                nm(96),
                hex(3, 2),
                hex(5, 2),
                vec![hex(4, 2), hex(5, 2)]
            ),
            (
                1,
                Pulse::Afternoon,
                nm(96),
                hex(5, 2),
                hex(8, 0),
                vec![hex(6, 2), hex(7, 1), hex(8, 0)]
            ),
            (2, Pulse::Night, nm(16), hex(8, 0), hex(8, 0), vec![]),
        ]
    );
    assert_eq!(total_nm(&run), nm(400));

    // Fuel by pulse.
    assert_eq!(
        burns(&run, CRUISER),
        vec![
            (1, Pulse::Night, nm(48)),
            (1, Pulse::Morning, nm(24)),
            (1, Pulse::Afternoon, nm(24)),
            (2, Pulse::Night, nm(4)),
        ]
    );
    assert_eq!(
        burns(&run, DESTROYER),
        vec![
            (1, Pulse::Night, nm(24)),
            (1, Pulse::Morning, nm(12)),
            (1, Pulse::Afternoon, nm(12)),
            (2, Pulse::Night, nm(2)),
        ]
    );

    // Arrival 25 hours after the start: 19:00 on day 2.
    let arrived = arrivals(&run);
    assert_eq!(arrived, vec![(hex(8, 0), nm(25))]);
    assert_eq!(
        ClockTime::from_elapsed(arrived[0].1).to_string(),
        "D2 19:00"
    );
    let arrival_event = run
        .events()
        .find(|event| matches!(event.kind, EventKind::TaskForceArrived { .. }))
        .expect("an arrival");
    assert_eq!(arrival_event.pulse, Some(Pulse::Night));
}

// ---------------------------------------------------------------------
// The variants table.
// ---------------------------------------------------------------------

#[test]
fn a_wall_the_whole_height_makes_the_order_unreachable() {
    let run = run(variant(&walled()));
    assert_eq!(
        rejections(&run),
        vec![RejectReason::WaypointUnreachable(hex(8, 0))]
    );
    assert!(steams(&run).is_empty());
    assert!(arrivals(&run).is_empty());
    let truth = run.scenario.world.umpire();
    assert_eq!(truth.task_forces[&TASK_FORCE].status, TaskForceStatus::Idle);
    assert_leg(&run.scenario.world, hex(0, 0), None, 0, hex(0, 0));
    assert_eq!(truth.ships[&CRUISER].fuel_t, nm(2000));
}

#[test]
fn a_land_waypoint_is_unreachable() {
    let run = run(variant(&route("waypoints = [[4, 0]]")));
    assert_eq!(
        rejections(&run),
        vec![RejectReason::WaypointUnreachable(hex(4, 0))]
    );
    assert!(steams(&run).is_empty());
    assert_eq!(
        run.scenario.world.umpire().task_forces[&TASK_FORCE].status,
        TaskForceStatus::Idle
    );
    assert_leg(&run.scenario.world, hex(0, 0), None, 0, hex(0, 0));
}

#[test]
fn an_off_map_waypoint_is_refused() {
    let run = run(variant(&route("waypoints = [[20, 0]]")));
    assert_eq!(
        rejections(&run),
        vec![RejectReason::WaypointOffMap(hex(20, 0))]
    );
    assert!(steams(&run).is_empty());
    assert_eq!(
        run.scenario.world.umpire().task_forces[&TASK_FORCE].status,
        TaskForceStatus::Idle
    );
    assert_leg(&run.scenario.world, hex(0, 0), None, 0, hex(0, 0));
}

#[test]
fn with_no_wall_the_path_is_the_straight_line() {
    let run = run(variant(&no_wall()));
    assert!(rejections(&run).is_empty());
    let expected: Vec<Hex> = (1..=8).map(|q| hex(q, 0)).collect();
    assert_eq!(path(&run), expected);
    assert_eq!(path(&run), hex(0, 0).steps_to(hex(8, 0)));
    assert_eq!(total_nm(&run), nm(320));
    assert_eq!(arrivals(&run), vec![(hex(8, 0), nm(20))]);
    assert_leg(&run.scenario.world, hex(8, 0), None, 0, hex(8, 0));
}

#[test]
fn waypoints_to_the_north_take_the_north_route() {
    let run = run(variant(&route("waypoints = [[3, -2], [8, 0]]")));
    assert!(rejections(&run).is_empty());
    assert_eq!(
        path(&run),
        vec![
            hex(1, 0),
            hex(2, -1),
            hex(3, -2),
            hex(4, -2),
            hex(5, -2),
            hex(6, -2),
            hex(7, -2),
            hex(8, -2),
            hex(8, -1),
            hex(8, 0),
        ]
    );
    assert_eq!(total_nm(&run), nm(400));
    assert_eq!(arrivals(&run), vec![(hex(8, 0), nm(25))]);
    assert_leg(&run.scenario.world, hex(8, 0), None, 0, hex(8, 0));
}

// ---------------------------------------------------------------------
// Properties.
// ---------------------------------------------------------------------

/// The maps to check the properties on: the island, the island with the
/// wall across the whole height, and the island with no wall.
fn property_maps() -> Vec<(&'static str, Map)> {
    vec![
        ("island", load().world.map().clone()),
        ("walled", variant(&walled()).world.map().clone()),
        ("no wall", variant(&no_wall()).world.map().clone()),
    ]
}

fn navigable_hexes(map: &Map) -> Vec<Hex> {
    map.iter()
        .filter(|(hex, _)| map.is_navigable(**hex))
        .map(|(hex, _)| *hex)
        .collect()
}

/// Walks the path from `from` to `target` by `next_step`, checking the
/// first property at each step. Returns the hexes entered.
fn walk(map: &Map, field: &BTreeMap<Hex, u32>, from: Hex, target: Hex) -> Vec<Hex> {
    let mut entered = Vec::new();
    let mut here = from;
    while here != target {
        let remaining = field[&here];
        let next = map.next_step_in(field, here).expect("a step exists");
        assert!(here.neighbors().contains(&next), "{here} to {next}");
        assert!(map.is_navigable(next), "{next} is not navigable");
        assert_eq!(field.get(&next), Some(&(remaining - 1)));
        entered.push(next);
        here = next;
    }
    assert_eq!(map.next_step_in(field, target), None);
    entered
}

#[test]
fn every_step_is_to_a_neighbouring_navigable_hex_one_step_nearer() {
    for (name, map) in property_maps() {
        let hexes = navigable_hexes(&map);
        for &target in &hexes {
            let distances = map.sea_distances_to(target);
            for &from in &hexes {
                match distances.get(&from) {
                    Some(&steps) => {
                        let entered = walk(&map, &distances, from, target);
                        assert_eq!(entered.len() as u32, steps, "{name}: {from} to {target}");
                    }
                    None => assert_eq!(map.next_step_in(&distances, from), None, "{name}"),
                }
            }
            // The public method agrees with the field, from one start.
            let probe = hexes[0];
            assert_eq!(
                map.next_step(probe, target),
                map.next_step_in(&distances, probe),
                "{name}: {probe} to {target}"
            );
            assert_eq!(map.next_step(target, target), None);
        }
    }
}

#[test]
fn the_steps_actually_taken_are_each_one_nearer() {
    let run = run(load());
    let target = hex(8, 0);
    let map = run.scenario.world.map();
    let mut here = hex(0, 0);
    let mut remaining = map.sea_distance(here, target).expect("reachable");
    for next in path(&run) {
        assert!(here.neighbors().contains(&next));
        assert!(map.is_navigable(next));
        remaining -= 1;
        assert_eq!(map.sea_distance(next, target), Some(remaining));
        here = next;
    }
    assert_eq!(here, target);
    assert_eq!(remaining, 0);
}

#[test]
fn a_path_is_never_shorter_than_the_straight_line_and_equal_on_open_water() {
    for (name, map) in property_maps() {
        let hexes = navigable_hexes(&map);
        for &target in &hexes {
            let distances = map.sea_distances_to(target);
            for (&from, &steps) in &distances {
                let straight = from.distance(target);
                assert!(steps >= straight, "{name}: {from} to {target}");
                // Open water: the straight line crosses only navigable hexes.
                if from.steps_to(target).iter().all(|h| map.is_navigable(*h)) {
                    assert_eq!(steps, straight, "{name}: {from} to {target}");
                }
            }
        }
    }
    // With no wall the whole map is open water: always equal.
    let (_, open) = property_maps().pop().unwrap();
    let hexes = navigable_hexes(&open);
    for &target in &hexes {
        let distances = open.sea_distances_to(target);
        assert_eq!(distances.len(), hexes.len());
        for &from in &hexes {
            assert_eq!(distances.get(&from), Some(&from.distance(target)));
        }
        assert_eq!(
            open.sea_distance(hexes[0], target),
            Some(hexes[0].distance(target))
        );
    }
    // The main case: 10 steps against a straight line of 8.
    let island = load();
    assert_eq!(
        island.world.map().sea_distance(hex(0, 0), hex(8, 0)),
        Some(10)
    );
    assert_eq!(hex(0, 0).distance(hex(8, 0)), 8);
}

#[test]
fn the_same_order_on_the_same_map_always_gives_the_same_path() {
    for change in [
        Change::default(),
        walled(),
        no_wall(),
        route("waypoints = [[3, -2], [8, 0]]"),
    ] {
        let first = run(variant(&change));
        let second = run(variant(&change));
        assert_eq!(first.days, second.days);
        assert_eq!(path(&first), path(&second));
        assert_eq!(
            first.scenario.world.state_hash(),
            second.scenario.world.state_hash()
        );
        assert_eq!(
            first.scenario.world.day_hashes(),
            second.scenario.world.day_hashes()
        );
    }
}

/// The scenarios before this one, with the waypoints of their route orders
/// (each file has one route order, checked below against the file).
const EARLIER_SCENARIOS: [(&str, &str, (i32, i32)); 5] = [
    ("01_transit.toml", "waypoints = [[8, 3]]", (8, 3)),
    ("02_supply_run.toml", "waypoints = [[8, 0]]", (8, 0)),
    ("03_search.toml", "waypoints = [[4, 0]]", (4, 0)),
    ("04a_night_action.toml", "waypoints = [[10, 0]]", (10, 0)),
    ("04a2_stragglers.toml", "waypoints = [[12, 0]]", (12, 0)),
];

/// On the maps of scenarios 1 to 4a2, from every navigable hex whose
/// straight line to a route waypoint crosses only navigable hexes, the
/// shortest-path step is the straight-line step, so their paths and numbers
/// are unchanged. This covers every hex, not only each force's own path.
#[test]
fn earlier_scenarios_keep_their_paths() {
    let mut checked = 0;
    for (file, route_text, (q, r)) in EARLIER_SCENARIOS {
        let path = data_dir().join("scenarios").join(file);
        let text = fs::read_to_string(&path).expect("read the scenario");
        assert_eq!(text.matches("waypoints").count(), 1, "{file}");
        assert_eq!(text.matches(route_text).count(), 1, "{file}");
        let scenario = Scenario::load(&path).expect("the scenario loads");
        let map = scenario.world.map();
        let waypoint = hex(q, r);
        let field = map.sea_distances_to(waypoint);
        assert!(map.is_navigable(waypoint), "{file}");
        // Every force starts on its greedy path, which must be clear.
        for task_force in scenario.world.umpire().task_forces.values() {
            let start = task_force.position();
            if start == waypoint {
                continue;
            }
            assert!(
                start
                    .steps_to(waypoint)
                    .iter()
                    .all(|h| map.is_navigable(*h)),
                "{file}: the greedy path of {start} is not clear"
            );
            // The public method, from each force's own start.
            assert_eq!(
                map.next_step(start, waypoint),
                Some(start.step_toward(waypoint)),
                "{file}: from {start}"
            );
        }
        for from in navigable_hexes(map) {
            if from == waypoint || !from.steps_to(waypoint).iter().all(|h| map.is_navigable(*h)) {
                continue;
            }
            assert_eq!(
                map.next_step_in(&field, from),
                Some(from.step_toward(waypoint)),
                "{file}: from {from} to {waypoint}"
            );
            checked += 1;
        }
    }
    assert!(checked > 100);
}

// ---------------------------------------------------------------------
// Part 2: the projection. Tested in the map tool crate, crates/map-tool.
// ---------------------------------------------------------------------

// ---------------------------------------------------------------------
// Part 3: the real map.
// ---------------------------------------------------------------------

const REAL_FILE: &str = "scenarios/04b_santo_to_lunga.toml";

fn load_real() -> Scenario {
    Scenario::load(&data_dir().join(REAL_FILE)).expect("the real-map scenario loads")
}

/// The anchors file as (place, latitude, longitude), in file order.
fn anchor_positions() -> Vec<(String, f64, f64)> {
    let text = fs::read_to_string(data_dir().join("map/south_pacific_anchors.csv"))
        .expect("read the anchors file");
    let mut reader = csv::ReaderBuilder::new()
        .has_headers(false)
        .comment(Some(b'#'))
        .from_reader(text.as_bytes());
    let mut records = reader.records();
    let header = records.next().expect("a header").expect("it parses");
    assert_eq!(&header[0], "place");
    records
        .map(|record| {
            let record = record.expect("a row");
            (
                record[0].to_string(),
                record[2].parse().expect("latitude"),
                record[3].parse().expect("longitude"),
            )
        })
        .collect()
}

/// A table row: place, east, north, hex, distance from the centre.
type Placed = (&'static str, f64, f64, (i32, i32), f64);
/// From, to, straight-line hexes, hexes by sea.
type SeaPair = ((i32, i32), (i32, i32), u32, u32);

fn within(value: f64, expected: f64, tolerance: f64) -> bool {
    (value - expected).abs() <= tolerance
}

/// "Where the anchorages fall". The positions are placed with the map
/// tool's projection, `map_tool::projection` (a dev-dependency of this
/// package), which works in `f64`; that is the only floating point in these
/// Part 3 tests besides the distance check below.
#[test]
fn the_anchorages_fall_in_the_hexes_the_table_gives() {
    use map_tool::projection::{centre_nm, hex_of, to_nm};
    let scenario = load_real();
    let map = scenario.world.map();
    let anchors = anchor_positions();
    assert_eq!(anchors.len(), 7);
    // (place, east, north, hex, distance from the centre) as in the table.
    let table: [Placed; 5] = [
        ("rabaul", -459.80, 466.67, (-5, -13), 16.3),
        ("tulagi", 9.01, 174.02, (3, -5), 11.0),
        ("lunga-point", 1.68, 155.68, (2, -4), 17.2),
        ("espiritu-santo", 419.85, -213.05, (7, 6), 20.5),
        ("noumea", 376.79, -615.41, (0, 18), 18.7),
    ];
    for (place, east, north, hex_qr, from_centre) in table {
        let (_, lat, lon) = anchors
            .iter()
            .find(|a| a.0 == place)
            .unwrap_or_else(|| panic!("{place} is in the anchors file"));
        let (e, n) = to_nm(*lat, *lon);
        assert!(within(e, east, 0.006), "{place} east {e}");
        assert!(within(n, north, 0.006), "{place} north {n}");
        assert_eq!(hex_of(*lat, *lon), hex_qr, "{place}");
        let (ce, cn) = centre_nm(hex_qr.0, hex_qr.1);
        let distance = ((e - ce) * (e - ce) + (n - cn) * (n - cn)).sqrt();
        assert!(within(distance, from_centre, 0.1), "{place} {distance}");
        assert_eq!(
            map.get(hex(hex_qr.0, hex_qr.1)).map(|d| d.kind),
            Some(sim_core::map::HexKind::Coast),
            "{place}'s hex is coast"
        );
    }

    // Segond Channel is a close call: 19.85 nm east of the centre of (7,6),
    // 20.15 nm west of the centre of (8,6).
    let (e, _) = to_nm(-15.55075, 167.15375);
    assert!(within(e - centre_nm(7, 6).0, 19.85, 0.01));
    assert!(within(centre_nm(8, 6).0 - e, 20.15, 0.01));

    // Savo Island's peak is in (2,-5), 9.3 nm from the centre.
    let (e, n) = to_nm(-9.1303, 159.81619);
    assert_eq!(hex_of(-9.1303, 159.81619), (2, -5));
    let (ce, cn) = centre_nm(2, -5);
    let distance = ((e - ce) * (e - ce) + (n - cn) * (n - cn)).sqrt();
    assert!(within(distance, 9.3, 0.1), "{distance}");
    assert_eq!(
        map.get(hex(2, -5)).map(|d| d.kind),
        Some(sim_core::map::HexKind::Coast)
    );
}

/// "What is open and what is closed".
#[test]
fn the_real_map_is_open_and_closed_where_the_geography_says() {
    use sim_core::map::HexKind;
    let scenario = load_real();
    let map = scenario.world.map();

    // Lunga Point's hex: open to Tulagi's (direction 1) and Savo's
    // (direction 2), closed on its other four sides.
    let lunga = hex(2, -4);
    let opened: Vec<usize> = (0..6).filter(|d| map.can_step(lunga, *d)).collect();
    assert_eq!(opened, vec![1, 2]);
    assert_eq!(lunga.neighbors()[1], hex(3, -5));
    assert_eq!(lunga.neighbors()[2], hex(2, -5));
    let closed: Vec<usize> = (0..6)
        .filter(|d| map.get(lunga).expect("on the map").closed_sides[*d])
        .collect();
    assert_eq!(closed, vec![0, 3, 4, 5]);

    // Straight-line hexes and sea hexes.
    let pairs: [SeaPair; 4] = [
        ((2, -4), (2, -3), 1, 3),
        ((-9, -12), (-10, -9), 3, 7),
        ((-5, -13), (-17, -4), 12, 17),
        ((-1, -9), (2, -5), 7, 7),
    ];
    for (from, to, straight, sea) in pairs {
        let (from, to) = (hex(from.0, from.1), hex(to.0, to.1));
        assert_eq!(from.distance(to), straight, "{from} to {to}");
        assert_eq!(map.sea_distance(from, to), Some(sea), "{from} to {to}");
    }

    // Counts by type, and every hex that is not land reachable from (0,0).
    assert_eq!(map.len(), 1600);
    let count = |kind: HexKind| map.iter().filter(|(_, d)| d.kind == kind).count();
    assert_eq!(count(HexKind::Land), 80);
    assert_eq!(count(HexKind::Coast), 210);
    assert_eq!(count(HexKind::DeepSea), 1310);
    let field = map.sea_distances_to(hex(0, 0));
    assert_eq!(field.len(), 1520);
    for hex_ in navigable_hexes(map) {
        assert!(field.contains_key(&hex_), "{hex_} cannot be reached");
    }

    // Every closed side is closed from both hexes.
    for (here, data) in map.iter() {
        for (d, neighbor) in here.neighbors().into_iter().enumerate() {
            if let Some(other) = map.get(neighbor) {
                assert_eq!(
                    data.closed_sides[d],
                    other.closed_sides[(d + 3) % 6],
                    "{here} direction {d}"
                );
            }
        }
    }
}

/// "Distances between anchorages": the sea distance in hexes, and that 40 nm
/// a hex is at least the straight line between the sourced positions and at
/// most 1.3 times it. Squares are compared, so no square root is needed; the
/// positions are in `f64` from the map tool's projection.
#[test]
fn sea_distances_between_anchorages_are_near_the_straight_line() {
    use map_tool::projection::to_nm;
    let scenario = load_real();
    let map = scenario.world.map();
    let anchors = anchor_positions();
    let position = |place: &str| {
        let a = anchors.iter().find(|a| a.0 == place).expect("a place");
        to_nm(a.1, a.2)
    };
    // (from, to, hexes, east, north, straight line) from the table.
    let table = [
        (
            "rabaul",
            (-5, -13),
            "lunga-point",
            (2, -4),
            16,
            461.5,
            311.0,
            556.5,
        ),
        (
            "espiritu-santo",
            (7, 6),
            "lunga-point",
            (2, -4),
            16,
            418.2,
            368.7,
            557.5,
        ),
        (
            "noumea",
            (0, 18),
            "espiritu-santo",
            (7, 6),
            13,
            43.1,
            402.4,
            404.7,
        ),
    ];
    for (from, from_hex, to, to_hex, hexes, east, north, straight) in table {
        assert_eq!(
            map.sea_distance(hex(from_hex.0, from_hex.1), hex(to_hex.0, to_hex.1)),
            Some(hexes),
            "{from} to {to}"
        );
        let ((fe, fn_), (te, tn)) = (position(from), position(to));
        let (de, dn) = ((te - fe).abs(), (tn - fn_).abs());
        assert!(within(de, east, 0.1), "{from} to {to}: east {de}");
        assert!(within(dn, north, 0.1), "{from} to {to}: north {dn}");
        let line_squared = de * de + dn * dn;
        assert!(within(line_squared.sqrt(), straight, 0.1), "{from} to {to}");
        let by_sea = f64::from(hexes) * 40.0;
        assert!(by_sea * by_sea >= line_squared, "{from} to {to}: shorter");
        assert!(
            by_sea * by_sea <= 1.69 * line_squared,
            "{from} to {to}: more than 30 percent over"
        );
    }
}

/// "The transit".
#[test]
fn the_task_force_steams_from_santo_to_lunga_as_in_part_3() {
    let mut scenario = load_real();
    assert!(scenario.placeholder);
    assert_eq!(scenario.days, 2);
    assert_leg(&scenario.world, hex(7, 6), None, 0, hex(7, 6));

    let mut days = Vec::new();
    let day1 = scenario.next_day().unwrap().unwrap();
    days.push(
        scenario
            .world
            .events()
            .from_index(day1.first_event)
            .to_vec(),
    );
    // End of day 1: left (7,-3) with 24 nm on, in (7,-4).
    assert_leg(
        &scenario.world,
        hex(7, -3),
        Some(hex(7, -4)),
        24,
        hex(7, -4),
    );
    let day2 = scenario.next_day().unwrap().unwrap();
    days.push(
        scenario
            .world
            .events()
            .from_index(day2.first_event)
            .to_vec(),
    );
    // End of day 2: arrived in (2,-4), no leg.
    assert_leg(&scenario.world, hex(2, -4), None, 0, hex(2, -4));
    assert_eq!(
        scenario.world.umpire().task_forces[&TASK_FORCE].status,
        TaskForceStatus::Arrived
    );
    assert_eq!(scenario.world.umpire().ships[&CRUISER].fuel_t, nm(1840));
    assert_eq!(scenario.world.umpire().ships[&DESTROYER].fuel_t, nm(420));
    assert!(scenario.next_day().unwrap().is_none());
    let run = Run { scenario, days };
    assert!(rejections(&run).is_empty());

    // The 16 hexes, in order.
    let expected = vec![
        hex(7, 5),
        hex(7, 4),
        hex(7, 3),
        hex(7, 2),
        hex(7, 1),
        hex(7, 0),
        hex(7, -1),
        hex(7, -2),
        hex(7, -3),
        hex(7, -4),
        hex(6, -4),
        hex(5, -4),
        hex(4, -4),
        hex(4, -5),
        hex(3, -5),
        hex(2, -4),
    ];
    let entered = path(&run);
    assert_eq!(entered, expected);

    // Every step is across an open side to a neighbour one nearer by sea.
    let map = run.scenario.world.map();
    let field = map.sea_distances_to(hex(2, -4));
    let mut here = hex(7, 6);
    assert_eq!(field[&here], 16);
    assert_eq!(here.distance(hex(2, -4)), 15);
    for next in &entered {
        let direction = here
            .neighbors()
            .iter()
            .position(|n| n == next)
            .unwrap_or_else(|| panic!("{here} to {next} is not a neighbour"));
        assert!(map.can_step(here, direction), "{here} to {next} is closed");
        assert_eq!(field[next] + 1, field[&here], "{here} to {next}");
        here = *next;
    }

    // Per pulse: the total so far, the hex left and miles on, the position.
    let steams = steams(&run);
    let pulses: Vec<(usize, Pulse)> = steams.iter().map(|s| (s.0, s.1)).collect();
    assert_eq!(
        pulses,
        vec![
            (1, Pulse::Night),
            (1, Pulse::Morning),
            (1, Pulse::Afternoon),
            (2, Pulse::Night),
            (2, Pulse::Morning),
        ]
    );
    // (total nm, hex left, miles on, position)
    let table = [
        (192, hex(7, 2), 32, hex(7, 1)),
        (288, hex(7, -1), 8, hex(7, -1)),
        (384, hex(7, -3), 24, hex(7, -4)),
        (576, hex(4, -5), 16, hex(4, -5)),
        (640, hex(2, -4), 0, hex(2, -4)),
    ];
    let mut total = Fixed::ZERO;
    let mut with_start = vec![hex(7, 6)];
    with_start.extend(expected.iter().copied());
    for (steam, (cumulative, left, on, position)) in steams.iter().zip(table) {
        total += steam.2;
        assert_eq!(total, nm(cumulative), "{:?}", steam.1);
        // The hex left is the one the whole 40 nm steps have reached.
        let whole = (cumulative / 40) as usize;
        assert_eq!(with_start[whole], left, "{:?}", steam.1);
        assert_eq!(cumulative % 40, on, "{:?}", steam.1);
        assert_eq!(steam.4, position, "{:?}", steam.1);
    }
    assert_eq!(total_nm(&run), nm(640));

    // Arrival 40 hours in: the end of the morning pulse of day 2.
    let arrived = arrivals(&run);
    assert_eq!(arrived, vec![(hex(2, -4), nm(40))]);
    assert_eq!(ClockTime::from_elapsed(nm(40)).to_string(), "D2 10:00");
    let arrival_event = run
        .events()
        .find(|event| matches!(event.kind, EventKind::TaskForceArrived { .. }))
        .expect("an arrival");
    assert_eq!(arrival_event.pulse, Some(Pulse::Morning));

    // Fuel: 640 x 0.25 = 160 t and 640 x 0.125 = 80 t.
    let burned = |ship| {
        burns(&run, ship)
            .into_iter()
            .fold(Fixed::ZERO, |sum, b| sum + b.2)
    };
    assert_eq!(burned(CRUISER), nm(160));
    assert_eq!(burned(DESTROYER), nm(80));
}
