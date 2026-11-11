//! `sim-cli batch`: many runs of one scenario, summarised as losses per side.
//!
//! Runs `execute` once per seed, in seed order, one after another, and writes
//! one JSON file. Every quantity in it is a whole number, a string or a bool.
//! The structs are serialised in declaration order, so the same inputs give
//! the same bytes. The file is written only after every run has succeeded.

use std::collections::BTreeSet;
use std::fs;

use serde::Serialize;
use sim_core::state::{ShipLocation, Side};

const DEFAULT_RUNS: u64 = 1000;
const DEFAULT_FIRST_SEED: u64 = 1;

/// The sides in the order the file lists them.
const SIDES: [Side; 2] = [Side::Japanese, Side::Allied];

#[derive(Serialize)]
struct Batch {
    format: &'static str,
    version: u32,
    scenario: ScenarioOut,
    runs: u64,
    first_seed: u64,
    ships: Vec<ShipOut>,
    sides: SidesOut,
    per_ship: Vec<PerShip>,
    per_run: Vec<PerRun>,
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
}

fn parse_count(flag: &str, value: Option<&str>, usage: &str) -> Result<u64, String> {
    let value = value.ok_or_else(|| format!("{flag} needs a number\n\n{usage}"))?;
    value
        .parse::<u64>()
        .map_err(|_| format!("{flag} \"{value}\" is not a whole number"))
}

/// `sim-cli batch <scenario.toml> --out <file.json> [--runs N] [--first-seed S]`.
/// `args` are the words after `batch`.
pub fn command(args: &[String], usage: &str) -> Result<(), String> {
    let mut scenario_path: Option<&str> = None;
    let mut out: Option<&str> = None;
    let mut runs = DEFAULT_RUNS;
    let mut first_seed = DEFAULT_FIRST_SEED;

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

    let batch = build(scenario_path, runs, first_seed)?;

    print_summary(&batch);
    let mut text = serde_json::to_string(&batch).map_err(|e| e.to_string())?;
    text.push('\n');
    fs::write(out, text).map_err(|e| format!("{out}: {e}"))
}

fn build(scenario_path: &str, runs: u64, first_seed: u64) -> Result<Batch, String> {
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

    for index in 0..runs {
        let seed = first_seed + index;
        let executed = crate::execute(scenario_path, Some(seed), |_| {})?;
        let world = &executed.scenario.world;
        let truth = world.umpire();

        if index == 0 {
            let ref_data = world.ref_data();
            for ship in executed.initial.ships.values() {
                let class = &ref_data.ship_classes[&ship.class];
                ships.push(ShipOut {
                    id: ship.id.raw(),
                    name: ship.name.clone(),
                    side: ship.side.name().to_lowercase(),
                    class: class.name.clone(),
                    kind: class.kind.name(),
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
        per_run.push(PerRun {
            seed,
            final_hash,
            sunk: sunk_ids.into_iter().collect(),
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

    Ok(Batch {
        format: "flag-plot-batch",
        version: 1,
        scenario: scenario_out.ok_or_else(|| "no runs were made".to_string())?,
        runs,
        first_seed,
        ships,
        sides: SidesOut {
            japanese: side_out(0),
            allied: side_out(1),
        },
        per_ship,
        per_run,
    })
}

fn print_summary(batch: &Batch) {
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
}
