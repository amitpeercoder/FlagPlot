//! `sim-cli batch`: many runs of one scenario, summarised as losses per side
//! and, for each run, everything the step 2 measures count (decision 0062).
//!
//! Runs `execute` once per seed, in seed order, one after another, and writes
//! one JSON file. Every quantity in it is a whole number, a string or a bool.
//! The structs are serialised in declaration order, so the same inputs give
//! the same bytes. The file is written only after every run has succeeded.

use std::collections::{BTreeMap, BTreeSet};
use std::fs;

use serde::Serialize;
use sim_core::ids::{AirGroupId, ShipId, TaskForceId};
use sim_core::objectives::{Level, Outcome};
use sim_core::state::{ShipLocation, Side};

use crate::batch_record::{self, RunCounts};

const DEFAULT_RUNS: u64 = 1000;
const DEFAULT_FIRST_SEED: u64 = 1;

/// The sides in the order the file lists them.
const SIDES: [Side; 2] = [Side::Japanese, Side::Allied];

#[derive(Serialize)]
struct Batch {
    format: &'static str,
    version: u32,
    scenario: ScenarioOut,
    /// What the runs were made on (decision 0057).
    inputs: crate::save::Inputs,
    /// The caller's own name for the engine that ran, such as a revision,
    /// from `--label`. The engine does not know its own revision.
    #[serde(skip_serializing_if = "Option::is_none")]
    label: Option<String>,
    runs: u64,
    first_seed: u64,
    ships: Vec<ShipOut>,
    sides: SidesOut,
    per_ship: Vec<PerShip>,
    per_run: Vec<PerRun>,
    /// The runs of each result, seen from the Allied side, when the scenario
    /// has objectives (decision 0054): from a decisive Allied victory to a
    /// decisive Allied defeat, every level listed.
    #[serde(skip_serializing_if = "Option::is_none")]
    results: Option<Vec<ResultRow>>,
    /// The tables the per-run counts name ids from (decision 0062): the
    /// scenario's task forces, air groups and bases as they start.
    task_forces: Vec<batch_record::TaskForceOut>,
    air_groups: Vec<batch_record::AirGroupOut>,
    bases: Vec<batch_record::BaseOut>,
}

#[derive(Serialize)]
struct ResultRow {
    /// `decisive victory`, `major victory`, `minor victory`, `stalemate`,
    /// `minor defeat`, `major defeat` or `decisive defeat`, of the Allied side.
    outcome: String,
    runs: u64,
}

/// The seven outcomes in the order the file lists them.
fn outcomes() -> [Outcome; 7] {
    [
        Outcome::Victory(Level::Decisive),
        Outcome::Victory(Level::Major),
        Outcome::Victory(Level::Minor),
        Outcome::Stalemate,
        Outcome::Defeat(Level::Minor),
        Outcome::Defeat(Level::Major),
        Outcome::Defeat(Level::Decisive),
    ]
}

#[derive(Serialize)]
struct ScenarioOut {
    name: String,
    /// The path as given on the command line.
    file: String,
    placeholder: bool,
    days: u32,
}

#[derive(Serialize)]
struct ShipOut {
    id: u32,
    name: String,
    side: String,
    class: String,
    kind: &'static str,
    /// The task force the ship starts in; none for a ship in port.
    task_force: Option<u32>,
}

#[derive(Serialize)]
struct SidesOut {
    japanese: SideOut,
    allied: SideOut,
}

#[derive(Serialize)]
struct SideOut {
    ships: u64,
    sunk_total: u64,
    /// `sunk_histogram[k]` is the number of runs in which exactly `k` of the
    /// side's ships were sunk at the end.
    sunk_histogram: Vec<u64>,
}

#[derive(Serialize)]
struct PerShip {
    id: u32,
    sunk_runs: u64,
    /// Afloat at the end with flotation damage at or over the scenario's
    /// crippled level. Left out when the scenario has no surface combat
    /// figures.
    #[serde(skip_serializing_if = "Option::is_none")]
    crippled_runs: Option<u64>,
}

#[derive(Serialize)]
struct PerRun {
    seed: u64,
    final_hash: String,
    sunk: Vec<u32>,
    /// The Allied margin and result of the run, when the scenario has
    /// objectives. The Japanese margin is its negative and the result its
    /// mirror.
    #[serde(skip_serializing_if = "Option::is_none")]
    allied_margin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    allied_outcome: Option<String>,
    /// What the run counted (decision 0062).
    #[serde(flatten)]
    counts: RunCounts,
}

fn parse_count(flag: &str, value: Option<&str>, usage: &str) -> Result<u64, String> {
    let value = value.ok_or_else(|| format!("{flag} needs a number\n\n{usage}"))?;
    value
        .parse::<u64>()
        .map_err(|_| format!("{flag} \"{value}\" is not a whole number"))
}

/// `sim-cli batch <scenario.toml> --out <file.json> [--runs N] [--first-seed S]`.
/// `args` are the words after `batch`.
pub fn command(
    args: &[String],
    usage: &str,
    options: &sim_data::LoadOptions,
) -> Result<(), String> {
    let mut scenario_path: Option<&str> = None;
    let mut out: Option<&str> = None;
    let mut runs = DEFAULT_RUNS;
    let mut first_seed = DEFAULT_FIRST_SEED;
    let mut label: Option<String> = None;

    let mut words = args.iter().map(String::as_str);
    while let Some(word) = words.next() {
        match word {
            "--out" => {
                out = Some(
                    words
                        .next()
                        .ok_or_else(|| format!("--out needs a file\n\n{usage}"))?,
                );
            }
            "--runs" => {
                runs = parse_count("--runs", words.next(), usage)?;
                if runs == 0 {
                    return Err("--runs must be at least 1".to_string());
                }
            }
            "--first-seed" => first_seed = parse_count("--first-seed", words.next(), usage)?,
            "--label" => {
                label = Some(
                    words
                        .next()
                        .ok_or_else(|| format!("--label needs some text\n\n{usage}"))?
                        .to_string(),
                );
            }
            flag if flag.starts_with('-') => {
                return Err(format!("unknown option \"{flag}\"\n\n{usage}"))
            }
            path if scenario_path.is_none() => scenario_path = Some(path),
            extra => return Err(format!("unexpected argument \"{extra}\"\n\n{usage}")),
        }
    }
    let scenario_path = scenario_path.ok_or_else(|| format!("no scenario given\n\n{usage}"))?;
    let out = out.ok_or_else(|| format!("batch needs --out <file.json>\n\n{usage}"))?;
    // The last seed must exist.
    first_seed
        .checked_add(runs - 1)
        .ok_or_else(|| "--first-seed plus --runs is too large".to_string())?;

    let batch = build(scenario_path, runs, first_seed, label, options)?;

    print_summary(&batch);
    let mut text = serde_json::to_string(&batch).map_err(|e| e.to_string())?;
    text.push('\n');
    fs::write(out, text).map_err(|e| format!("{out}: {e}"))
}

fn build(
    scenario_path: &str,
    runs: u64,
    first_seed: u64,
    label: Option<String>,
    options: &sim_data::LoadOptions,
) -> Result<Batch, String> {
    let inputs = {
        let scenario = sim_data::Scenario::load_with(std::path::Path::new(scenario_path), options)
            .map_err(|e| e.to_string())?;
        crate::save::inputs(scenario_path, &scenario)?
    };
    let mut ships: Vec<ShipOut> = Vec::new();
    let mut scenario_out: Option<ScenarioOut> = None;
    // Per ship, in id order: runs sunk, runs crippled.
    let mut sunk_runs: Vec<u64> = Vec::new();
    let mut crippled_runs: Vec<u64> = Vec::new();
    let mut has_combat = false;
    // histogram per side, indexed like SIDES.
    let mut histograms: Vec<Vec<u64>> = Vec::new();
    let mut side_ships: Vec<u64> = Vec::new();
    let mut per_run: Vec<PerRun> = Vec::new();
    let mut outcome_runs = [0u64; 7];
    let mut has_objectives = false;
    let mut header: Option<batch_record::Header> = None;

    for index in 0..runs {
        let seed = first_seed + index;
        // A batch reads losses, not reasons, so it runs with breakdown
        // recording off (the spec's "can be switched off in batch runs").
        // Recording never changes what happens or a hash.
        let mut ready_by_day: Vec<BTreeMap<AirGroupId, u32>> = Vec::new();
        let executed = crate::execute_observing(
            scenario_path,
            Some(seed),
            options,
            |scenario| {
                scenario.world.set_config(sim_core::world::SimConfig {
                    record_breakdowns: false,
                });
            },
            |scenario| ready_by_day.push(batch_record::ready_now(scenario.world.umpire())),
        )?;
        let world = &executed.scenario.world;
        let truth = world.umpire();

        if index == 0 {
            let ref_data = world.ref_data();
            header = Some(batch_record::header(&executed.initial, world));
            let force_of: BTreeMap<ShipId, TaskForceId> =
                batch_record::initial_force_of(&executed.initial);
            for ship in executed.initial.ships.values() {
                let class = &ref_data.ship_classes[&ship.class];
                ships.push(ShipOut {
                    id: ship.id.raw(),
                    name: ship.name.clone(),
                    side: ship.side.name().to_lowercase(),
                    class: class.name.clone(),
                    kind: class.kind.name(),
                    task_force: force_of.get(&ship.id).map(|force| force.raw()),
                });
            }
            sunk_runs = vec![0; ships.len()];
            crippled_runs = vec![0; ships.len()];
            has_combat = ref_data.surface_combat.is_some();
            for side in SIDES {
                let count = executed
                    .initial
                    .ships
                    .values()
                    .filter(|ship| ship.side == side)
                    .count();
                side_ships.push(count as u64);
                histograms.push(vec![0; count + 1]);
            }
            scenario_out = Some(ScenarioOut {
                name: executed.scenario.name.clone(),
                file: scenario_path.to_string(),
                placeholder: executed.scenario.placeholder,
                days: executed.scenario.days,
            });
        }

        let crippled_level = world
            .ref_data()
            .surface_combat
            .as_ref()
            .map(|figures| figures.crippled_flotation);
        let mut sunk_ids: BTreeSet<u32> = BTreeSet::new();
        let mut sunk_by_side = [0usize; 2];
        for (position, ship) in executed.initial.ships.values().enumerate() {
            let now = &truth.ships[&ship.id];
            if now.location == ShipLocation::Sunk {
                sunk_ids.insert(ship.id.raw());
                sunk_runs[position] += 1;
                if let Some(slot) = SIDES.iter().position(|side| *side == ship.side) {
                    sunk_by_side[slot] += 1;
                }
            } else if crippled_level.is_some_and(|level| now.flotation_damage >= level) {
                crippled_runs[position] += 1;
            }
        }
        for (slot, count) in sunk_by_side.iter().enumerate() {
            histograms[slot][*count] += 1;
        }
        let final_hash = executed
            .summaries
            .last()
            .map(|summary| format!("{:016x}", summary.state_hash))
            .ok_or_else(|| format!("{scenario_path}: the scenario has no days"))?;
        let result = world.result();
        if let Some(result) = &result {
            has_objectives = true;
            if let Some(slot) = outcomes().iter().position(|o| *o == result.allied.outcome) {
                outcome_runs[slot] += 1;
            }
        }
        per_run.push(PerRun {
            seed,
            final_hash,
            sunk: sunk_ids.into_iter().collect(),
            allied_margin: result.as_ref().map(|r| r.allied.margin.to_string()),
            allied_outcome: result.as_ref().map(|r| r.allied.outcome.name()),
            counts: batch_record::count(world, &executed.initial, &ready_by_day),
        });
    }

    let side_out = |slot: usize| {
        let histogram = histograms[slot].clone();
        let total = histogram
            .iter()
            .enumerate()
            .map(|(k, runs)| k as u64 * runs)
            .sum();
        SideOut {
            ships: side_ships[slot],
            sunk_total: total,
            sunk_histogram: histogram,
        }
    };
    let per_ship = ships
        .iter()
        .enumerate()
        .map(|(position, ship)| PerShip {
            id: ship.id,
            sunk_runs: sunk_runs[position],
            crippled_runs: has_combat.then_some(crippled_runs[position]),
        })
        .collect();

    let header = header.ok_or_else(|| "no runs were made".to_string())?;
    Ok(Batch {
        format: "flag-plot-batch",
        version: 4,
        scenario: scenario_out.ok_or_else(|| "no runs were made".to_string())?,
        inputs,
        label,
        runs,
        first_seed,
        ships,
        sides: SidesOut {
            japanese: side_out(0),
            allied: side_out(1),
        },
        per_ship,
        per_run,
        results: has_objectives.then(|| {
            outcomes()
                .iter()
                .zip(outcome_runs)
                .map(|(outcome, runs)| ResultRow {
                    outcome: outcome.name(),
                    runs,
                })
                .collect()
        }),
        task_forces: header.task_forces,
        air_groups: header.air_groups,
        bases: header.bases,
    })
}

fn print_summary(batch: &Batch) {
    // The summary below is unchanged for a scenario without objectives.
    println!("{}", batch.scenario.name);
    if batch.scenario.placeholder {
        println!("PLACEHOLDER DATA: some or all of the units, figures and geography are invented, not historical.");
    }
    println!(
        "{} runs, seeds {} to {}",
        batch.runs,
        batch.first_seed,
        batch.first_seed + batch.runs - 1
    );
    for (name, side) in [
        ("japanese", &batch.sides.japanese),
        ("allied", &batch.sides.allied),
    ] {
        println!();
        let histogram: Vec<String> = side.sunk_histogram.iter().map(u64::to_string).collect();
        println!(
            "{name}: {} ships, sunk histogram [{}]",
            side.ships,
            histogram.join(", ")
        );
        for (ship, count) in batch.ships.iter().zip(&batch.per_ship) {
            if ship.side == name {
                println!(
                    "  {}: sunk in {} of {}",
                    ship.name, count.sunk_runs, batch.runs
                );
            }
        }
    }
    if let Some(results) = &batch.results {
        println!();
        println!("results, seen from the Allied side:");
        for row in results {
            println!("  {}: {} of {}", row.outcome, row.runs, batch.runs);
        }
    }
}
