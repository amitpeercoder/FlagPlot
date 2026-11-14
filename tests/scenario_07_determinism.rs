//! Scenario 7: determinism (decision 0043).
//!
//! `data/scenarios/07_full_slice.toml` runs everything the earlier scenarios
//! built, together, for 14 days on the real map: the carrier forces of the
//! Eastern Solomons scenario on day 4, the air campaign of the Rabaul against
//! Guadalcanal scenario, a convoy from Espiritu Santo to Henderson Field and
//! three night surface actions. It is assembled from files that exist and is
//! not compared with history.
//!
//! There is no hand calculation of outcomes. The claim is equality, and
//! nothing else:
//!
//! 1. Run twice in one process, with seeds 1942 (the scenario's own) and 1 to
//!    20, the 14 daily hashes and every field of every event are equal.
//! 2. Run with and without breakdown recording, the daily hashes are equal
//!    and the events are equal apart from their breakdowns, which are
//!    present in one and absent in the other.
//! 3. Load with the order of the file's tables permuted, the daily hashes
//!    and every event are equal to the unpermuted run's. The permutation is
//!    applied to the scenario text before the loader sees it, so it reaches
//!    the loader and not a map after ids have been put in it. It permutes
//!    the `[[base]]`, `[[task_force]]`, `[[ship]]`, `[[air_group]]`,
//!    `[[arrival]]` and `[[replacement]]` tables among their own places, and
//!    the `[[order]]` tables among theirs, except that the orders to one unit
//!    on one day keep their order (decision 4 of 0043, a later order
//!    replaces an earlier one). 22 permutations from a small generator
//!    written in this file plus the reversal of every kind, for the full
//!    slice; fewer for Savo Island, Eastern Solomons and Rabaul against
//!    Guadalcanal.
//! 4. The full slice's log holds at least one event of every kind the
//!    engine can emit. Its kinds are read from `EventKind` in
//!    `crates/sim-core/src/events.rs`.
//!
//! The same claim across two separate processes of the runner is in
//! `crates/sim-cli/tests/saved_games.rs`, where the binary can be run.
//!
//! Orders are not shuffled by the rules' own order: losses come off the
//! lowest group first, forces move in clock order, and so on. That order is
//! part of the rules (decision 3 of 0043).

mod common;

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};

use sim_core::events::Event;
use sim_core::SimConfig;
use sim_data::Scenario;

const ROOT: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/..");

fn scenario_path(name: &str) -> PathBuf {
    Path::new(ROOT).join("data/scenarios").join(name)
}

/// The determinism scenario runs on the live figures: that a run repeats, that
/// its order of tables and its breakdowns change nothing, and that the
/// invariants hold are claims about the figures in use, and step 2 tunes them.
/// Only the test of which kinds of event the log holds, which depends on what
/// the figures make happen, runs on the frozen figures of `data/reference/step1`.
fn load(name: &str, seed: Option<u64>) -> Scenario {
    let scenario = common::load_live(&scenario_path(name)).expect("the scenario loads");
    match seed {
        Some(seed) => scenario.with_seed(seed).expect("a seed can be set"),
        None => scenario,
    }
}

/// What a run leaves: the hash of every day and the whole event log.
#[derive(PartialEq, Debug)]
struct Outcome {
    hashes: Vec<u64>,
    events: Vec<Event>,
}

fn run(mut scenario: Scenario) -> Outcome {
    let summaries = scenario.run_to_end().expect("the run resolves");
    Outcome {
        hashes: summaries.iter().map(|s| s.state_hash).collect(),
        events: scenario.world.events().iter().cloned().collect(),
    }
}

fn assert_same(name: &str, a: &Outcome, b: &Outcome) {
    assert_eq!(a.hashes, b.hashes, "{name}: the daily hashes differ");
    assert_eq!(
        a.events.len(),
        b.events.len(),
        "{name}: the logs have different lengths"
    );
    for (x, y) in a.events.iter().zip(&b.events) {
        assert_eq!(x, y, "{name}: events differ");
    }
}

const FULL: &str = "07_full_slice.toml";
/// The frozen copy of the full slice, for the run on the frozen figures.
const FULL_FROZEN: &str = "step2-frozen/07_full_slice.toml";

#[test]
fn the_full_slice_runs_fourteen_days_with_the_invariants_holding() {
    // `run_to_end` checks the invariants at the end of every day and fails
    // the run if one breaks.
    let outcome = run(load(FULL, None));
    assert_eq!(outcome.hashes.len(), 14);
    let distinct: BTreeSet<u64> = outcome.hashes.iter().copied().collect();
    assert_eq!(distinct.len(), 14, "every day changes the state");
}

// ---------------------------------------------------------------------
// 1. Twice in one process.
// ---------------------------------------------------------------------

#[test]
fn run_twice_in_one_process_gives_the_same_hashes_and_events() {
    let mut seeds = vec![1942u64];
    seeds.extend(1..=20);
    for seed in seeds {
        let a = run(load(FULL, Some(seed)));
        let b = run(load(FULL, Some(seed)));
        assert_eq!(a.hashes.len(), 14);
        assert_same(&format!("seed {seed}"), &a, &b);
    }
    // The seed matters: two seeds give different runs.
    let a = run(load(FULL, Some(1)));
    let b = run(load(FULL, Some(2)));
    assert_ne!(a.hashes, b.hashes);
}

// ---------------------------------------------------------------------
// 2. With and without breakdowns.
// ---------------------------------------------------------------------

#[test]
fn breakdown_recording_changes_no_hash_and_no_event_but_the_breakdowns() {
    let mut seeds = vec![1942u64];
    seeds.extend(1..=20);
    for seed in seeds {
        let mut with = load(FULL, Some(seed));
        with.world.set_config(SimConfig {
            record_breakdowns: true,
        });
        let mut without = load(FULL, Some(seed));
        without.world.set_config(SimConfig {
            record_breakdowns: false,
        });
        let with = run(with);
        let without = run(without);
        assert_eq!(with.hashes, without.hashes, "seed {seed}");
        assert_eq!(with.events.len(), without.events.len(), "seed {seed}");
        let mut some_breakdown = false;
        for (a, b) in with.events.iter().zip(&without.events) {
            assert!(b.breakdown.is_none(), "seed {seed}: {}", b.kind);
            some_breakdown |= a.breakdown.is_some();
            let mut stripped = a.clone();
            stripped.breakdown = None;
            assert_eq!(&stripped, b, "seed {seed}");
        }
        assert!(some_breakdown, "seed {seed}: the run records breakdowns");
    }
}

// ---------------------------------------------------------------------
// 3. The order the file gives its tables in.
// ---------------------------------------------------------------------

/// A small deterministic generator (xorshift64*). No clock, no dependency.
struct Gen(u64);

impl Gen {
    fn next(&mut self) -> u64 {
        let mut x = self.0;
        x ^= x >> 12;
        x ^= x << 25;
        x ^= x >> 27;
        self.0 = x;
        x.wrapping_mul(0x2545_F491_4F6C_DD1D)
    }

    fn below(&mut self, n: usize) -> usize {
        usize::try_from(self.next() % u64::try_from(n).expect("small")).expect("small")
    }

    /// Fisher-Yates.
    fn shuffle<T>(&mut self, items: &mut [T]) {
        for i in (1..items.len()).rev() {
            let j = self.below(i + 1);
            items.swap(i, j);
        }
    }
}

/// A scenario file taken apart: the lines before the first table, then each
/// `[[kind]]` table with the comment and blank lines after it.
struct Parts {
    head: String,
    tables: Vec<(String, String)>,
}

fn take_apart(text: &str) -> Parts {
    let mut head = String::new();
    let mut tables: Vec<(String, String)> = Vec::new();
    for line in text.split_inclusive('\n') {
        if let Some(kind) = line
            .trim_end()
            .strip_prefix("[[")
            .and_then(|rest| rest.strip_suffix("]]"))
        {
            tables.push((kind.to_string(), line.to_string()));
        } else if let Some(last) = tables.last_mut() {
            last.1.push_str(line);
        } else {
            head.push_str(line);
        }
    }
    Parts { head, tables }
}

fn put_together(parts: &Parts) -> String {
    let mut text = parts.head.clone();
    for (_, table) in &parts.tables {
        text.push_str(table);
    }
    text
}

/// The unit and day an `[[order]]` table is for.
fn order_key(table: &str) -> (String, String) {
    let value = |key: &str| {
        table
            .lines()
            .find_map(|line| line.strip_prefix(key).and_then(|r| r.strip_prefix(" = ")))
            .map(|v| v.trim().to_string())
    };
    let day = value("day").expect("an order has a day");
    let unit = value("task_force")
        .map(|n| format!("task_force {n}"))
        .or_else(|| value("air_group").map(|n| format!("air_group {n}")))
        .expect("an order is for a unit");
    (day, unit)
}

const SHUFFLED: [&str; 6] = [
    "base",
    "task_force",
    "ship",
    "air_group",
    "arrival",
    "replacement",
];

/// The tables in another order. `how` is a shuffle of the places a kind's
/// tables sit in; the same kind keeps its places, so the file's layout of
/// kinds is unchanged. Orders are shuffled over their places, then the orders
/// to one unit on one day are put back in their given order.
fn permuted(parts: &Parts, how: &mut dyn FnMut(&mut Vec<usize>)) -> Parts {
    let mut tables = parts.tables.clone();
    let mut kinds: Vec<&str> = SHUFFLED.to_vec();
    kinds.push("order");
    for kind in kinds {
        let places: Vec<usize> = (0..tables.len())
            .filter(|&i| parts.tables[i].0 == kind)
            .collect();
        let mut order: Vec<usize> = (0..places.len()).collect();
        how(&mut order);
        if kind == "order" {
            // `order[p]` is which original order sits at place p. Put the
            // orders of one unit on one day back in their original order.
            let mut by_key: BTreeMap<(String, String), Vec<usize>> = BTreeMap::new();
            for (p, original) in order.iter().enumerate() {
                by_key
                    .entry(order_key(&parts.tables[places[*original]].1))
                    .or_default()
                    .push(p);
            }
            for (key, mut at) in by_key {
                let mut originals: Vec<usize> = order
                    .iter()
                    .copied()
                    .filter(|o| order_key(&parts.tables[places[*o]].1) == key)
                    .collect();
                originals.sort_unstable();
                at.sort_unstable();
                for (p, o) in at.into_iter().zip(originals) {
                    order[p] = o;
                }
            }
        }
        for (p, original) in order.iter().enumerate() {
            tables[places[p]] = parts.tables[places[*original]].clone();
        }
    }
    Parts {
        head: parts.head.clone(),
        tables,
    }
}

fn load_text(name: &str, text: &str) -> Scenario {
    Scenario::from_text(text, &scenario_path(name)).expect("the permuted scenario loads")
}

/// Runs `name` as the file is, then with its tables permuted `count` times by
/// a generator seeded with `seed` and once reversed.
fn shuffled_runs_equal_the_file_run(name: &str, count: usize, seed: u64) {
    let text = fs::read_to_string(scenario_path(name)).expect("the file reads");
    let parts = take_apart(&text);
    assert_eq!(put_together(&parts), text, "taking apart loses nothing");
    let base = run(load_text(name, &text));

    let mut moved = 0;
    let mut generator = Gen(seed);
    for n in 0..count {
        let shuffled = permuted(&parts, &mut |order| generator.shuffle(order));
        let shuffled_text = put_together(&shuffled);
        moved += usize::from(shuffled_text != text);
        let outcome = run(load_text(name, &shuffled_text));
        assert_same(&format!("{name}, permutation {n}"), &base, &outcome);
    }
    assert!(moved > 0, "{name}: the permutations changed the text");

    let reversed = permuted(&parts, &mut |order| order.reverse());
    let reversed_text = put_together(&reversed);
    assert_ne!(reversed_text, text);
    let outcome = run(load_text(name, &reversed_text));
    assert_same(&format!("{name}, reversed"), &base, &outcome);
}

#[test]
fn the_full_slice_does_not_depend_on_the_order_of_its_tables() {
    shuffled_runs_equal_the_file_run(FULL, 22, 0x1942_0821);
}

#[test]
fn savo_island_does_not_depend_on_the_order_of_its_tables() {
    shuffled_runs_equal_the_file_run("04_savo_island.toml", 6, 0x0809_1942);
}

#[test]
fn eastern_solomons_does_not_depend_on_the_order_of_its_tables() {
    shuffled_runs_equal_the_file_run("05_eastern_solomons.toml", 6, 0x2408_1942);
}

#[test]
fn rabaul_against_guadalcanal_does_not_depend_on_the_order_of_its_tables() {
    shuffled_runs_equal_the_file_run("06_rabaul_guadalcanal.toml", 6, 0x2108_1942);
}

#[test]
fn the_permutation_keeps_one_units_orders_in_order() {
    // Two orders to one unit on one day, between others: a reversal of every
    // kind must leave them in their given order.
    let text = fs::read_to_string(scenario_path(FULL)).expect("the file reads");
    let parts = take_apart(&text);
    let reversed = permuted(&parts, &mut |order| order.reverse());
    let orders = |p: &Parts| -> Vec<(String, String)> {
        p.tables
            .iter()
            .filter(|(kind, _)| kind == "order")
            .map(|(_, table)| order_key(table))
            .collect()
    };
    let units = |p: &Parts, key: &(String, String)| -> Vec<String> {
        p.tables
            .iter()
            .filter(|(kind, table)| kind == "order" && &order_key(table) == key)
            .map(|(_, table)| table.clone())
            .collect()
    };
    let mut seen_two = false;
    for key in orders(&parts) {
        let given = units(&parts, &key);
        seen_two |= given.len() > 1;
        assert_eq!(given, units(&reversed, &key), "{key:?}");
    }
    assert!(seen_two, "the scenario has a unit with two orders in a day");
    assert_ne!(orders(&parts), orders(&reversed));
}

// ---------------------------------------------------------------------
// 4. Every kind of event.
// ---------------------------------------------------------------------

/// The variant names of `EventKind`, read from the source.
fn event_kinds() -> BTreeSet<String> {
    let source = fs::read_to_string(Path::new(ROOT).join("crates/sim-core/src/events.rs"))
        .expect("the events file reads");
    let start = source
        .find("pub enum EventKind {")
        .expect("the enum is there");
    let body = &source[start..];
    let end = body.find("\n}\n").expect("the enum ends");
    body[..end]
        .lines()
        .filter_map(|line| {
            let rest = line.strip_prefix("    ")?;
            let first = rest.chars().next()?;
            if !first.is_ascii_uppercase() {
                return None;
            }
            Some(
                rest.chars()
                    .take_while(|c| c.is_ascii_alphanumeric())
                    .collect::<String>(),
            )
        })
        .collect()
}

#[test]
fn the_full_slice_log_has_every_kind_of_event_the_engine_can_emit() {
    let all = event_kinds();
    assert!(all.len() >= 50, "read {} kinds", all.len());
    // On the frozen figures: which kinds of event the log holds depends on
    // what the figures make happen. It runs on the frozen copy of the scenario,
    // `data/scenarios/step2-frozen/07_full_slice.toml`, as decision 0073 and
    // the folder's README say: frozen figures go with frozen scenarios.
    let outcome = run(common::load(&scenario_path(FULL_FROZEN)).expect("the scenario loads"));
    let seen: BTreeSet<String> = outcome
        .events
        .iter()
        .map(|e| {
            format!("{:?}", e.kind)
                .chars()
                .take_while(|c| c.is_ascii_alphanumeric())
                .collect()
        })
        .collect();
    // A kind the full slice cannot record: a ditching needs an air group on
    // extended reach and the full slice gives no reach order (decision
    // 0059, stage 1). Scenario 12a records it, and its own test reads it.
    //
    // `AircraftLandedElsewhere` is the other: the full slice on its seed has
    // no deck lost beside another deck with room or within reach of a base (its
    // one deck lost, Ryujo's, has nothing near), so its aircraft in the air
    // come down in the sea. Stage 3 of 0059 took Flight 300's scripted
    // arrival out of the slice, and on its own seed no Allied deck is lost
    // (Enterprise's deck goes out in about a quarter of other seeds), so
    // `AircraftLandedElsewhere` stays out of this log. Scenarios 12c and 12d
    // record it. `AircraftDitchedNoDeck` is in the log and needs no entry.
    //
    // `ShipRepaired`, `ForceLeftForYard` and `ForceReturnedFromYard` are the
    // three of decision 0069: no base of the full slice mends a ship or is a
    // port for the yard. Scenario 20a records them, and its own test reads
    // them.
    //
    // The nine kinds of land units (decision 0066), `MenEmbarked`,
    // `MenLanded`, `BaseFormed`, `UnitDivided`, `MenLostWithShip`, `MenFed`,
    // `MenLostUnfed`, `LandUnitDestroyed` and `UnitLoadingComplete`: the full
    // slice has no land unit. Scenario 21 records them, and its own test
    // reads them.
    //
    // `AircraftWrittenOff` and `AirGroupRebased` are the two of decision
    // 0091: the full slice gives no rebase order. Scenario 22 records them,
    // and its own test reads them.
    //
    // The twelve kinds of decision 0090, `FedShareRestored`, `GroundFight`,
    // `GroundFire`, `MenLostInFight`, `LandUnitFatigue`,
    // `LandUnitBackToDefend`, `SideBeatenAshore`, `BaseChangedSide`,
    // `AirGroupLostWithBase`, `ShipLostWithBase`, `HomePortLost` and
    // `StrikeMissionCleared`: the full slice has no land unit. Scenario 23
    // records them, and its own test reads them.
    //
    // The five kinds of decision 0092, `BombardmentFire`, `ShoreBombardment`,
    // `MenLostToFire`, `GroundReportReceived` and `BaseFound`: the full slice
    // has no bombardment mission and no land unit. Scenario 24 records them,
    // and its own test reads them.
    //
    // The four kinds of decision 0094, `MarchDay`, `MarchBaseFormed`,
    // `MarchArrived` and `MarchStopped`: the full slice gives no march order.
    // Scenario 25 records them, and its own test reads them.
    const NOT_IN_THE_FULL_SLICE: [&str; 37] = [
        "AircraftDitched",
        "AircraftLandedElsewhere",
        "ShipRepaired",
        "ForceLeftForYard",
        "ForceReturnedFromYard",
        "MenEmbarked",
        "MenLanded",
        "BaseFormed",
        "UnitDivided",
        "MenLostWithShip",
        "MenFed",
        "MenLostUnfed",
        "LandUnitDestroyed",
        "UnitLoadingComplete",
        "AircraftWrittenOff",
        "AirGroupRebased",
        "FedShareRestored",
        "GroundFight",
        "GroundFire",
        "MenLostInFight",
        "LandUnitFatigue",
        "LandUnitBackToDefend",
        "SideBeatenAshore",
        "BaseChangedSide",
        "AirGroupLostWithBase",
        "ShipLostWithBase",
        "HomePortLost",
        "StrikeMissionCleared",
        "BombardmentFire",
        "ShoreBombardment",
        "MenLostToFire",
        "GroundReportReceived",
        "BaseFound",
        "MarchDay",
        "MarchBaseFormed",
        "MarchArrived",
        "MarchStopped",
    ];
    let missing: Vec<&String> = all
        .difference(&seen)
        .filter(|kind| !NOT_IN_THE_FULL_SLICE.contains(&kind.as_str()))
        .collect();
    assert!(missing.is_empty(), "kinds not in the log: {missing:?}");
    let unknown: Vec<&String> = seen.difference(&all).collect();
    assert!(
        unknown.is_empty(),
        "kinds not read from the source: {unknown:?}"
    );
}
