//! Integration tests of `sim-cli batch`. The summary is checked against what
//! `sim-cli run --seed s` prints for the same seeds.

use std::fs;
use std::path::PathBuf;
use std::process::Command;

use serde_json::Value;

const ROOT: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../..");
const FIGHT: &str = "data/scenarios/04a_night_action.toml";
const QUIET: &str = "data/scenarios/01_transit.toml";

fn binary(args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_sim-cli"))
        .args(args)
        .current_dir(ROOT)
        .output()
        .expect("the binary starts")
}

fn temp_file(label: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("flagplot-batch-test-{}", std::process::id()));
    fs::create_dir_all(&dir).expect("make the temp dir");
    dir.join(format!("{label}.json"))
}

/// Runs a batch and returns the file's bytes.
fn batch(label: &str, scenario: &str, extra: &[&str]) -> Vec<u8> {
    let out = temp_file(label);
    let out_text = out.to_string_lossy().into_owned();
    let mut args = vec!["batch", scenario, "--out", out_text.as_str()];
    args.extend_from_slice(extra);
    let result = binary(&args);
    assert!(
        result.status.success(),
        "batch of {scenario} failed: {}",
        String::from_utf8_lossy(&result.stderr)
    );
    let bytes = fs::read(&out).expect("the batch file exists");
    let _ = fs::remove_file(&out);
    bytes
}

fn parse(bytes: &[u8]) -> Value {
    serde_json::from_slice(bytes).expect("the batch is valid JSON")
}

fn ids_of(side: &str, summary: &Value) -> Vec<u64> {
    summary["ships"]
        .as_array()
        .expect("ships")
        .iter()
        .filter(|ship| ship["side"] == side)
        .map(|ship| ship["id"].as_u64().expect("id"))
        .collect()
}

#[test]
fn the_summary_adds_up_and_matches_single_runs() {
    let summary = parse(&batch("fight", FIGHT, &["--runs", "20"]));
    assert_eq!(summary["format"], "flag-plot-batch");
    assert_eq!(summary["version"], 2);
    assert_eq!(summary["runs"], 20);
    assert_eq!(summary["first_seed"], 1);
    assert_eq!(summary["scenario"]["file"], FIGHT);

    let per_ship = summary["per_ship"].as_array().expect("per_ship");
    let per_run = summary["per_run"].as_array().expect("per_run");
    assert_eq!(per_run.len(), 20);

    for side in ["japanese", "allied"] {
        let block = &summary["sides"][side];
        let ids = ids_of(side, &summary);
        assert_eq!(block["ships"], ids.len());
        let histogram = block["sunk_histogram"].as_array().expect("histogram");
        assert_eq!(histogram.len(), ids.len() + 1);
        let runs: u64 = histogram.iter().map(|n| n.as_u64().expect("count")).sum();
        assert_eq!(runs, 20, "{side}: histogram sums to runs");

        let total = block["sunk_total"].as_u64().expect("sunk_total");
        let from_ships: u64 = per_ship
            .iter()
            .filter(|ship| ids.contains(&ship["id"].as_u64().expect("id")))
            .map(|ship| ship["sunk_runs"].as_u64().expect("sunk_runs"))
            .sum();
        let from_runs: u64 = per_run
            .iter()
            .map(|run| {
                run["sunk"]
                    .as_array()
                    .expect("sunk")
                    .iter()
                    .filter(|id| ids.contains(&id.as_u64().expect("id")))
                    .count() as u64
            })
            .sum();
        assert_eq!(total, from_ships, "{side}: per ship");
        assert_eq!(total, from_runs, "{side}: per run");
    }
    assert!(per_ship.iter().all(|ship| ship["crippled_runs"].is_u64()));

    for index in [0usize, 9, 19] {
        let run = &per_run[index];
        let seed = run["seed"].as_u64().expect("seed");
        assert_eq!(seed, 1 + index as u64);
        let output = binary(&["run", FIGHT, "--seed", &seed.to_string()]);
        assert!(output.status.success());
        let text = String::from_utf8(output.stdout).expect("UTF-8");
        let last_hash = text
            .lines()
            .rfind(|line| line.starts_with("day "))
            .and_then(|line| line.split_whitespace().nth(3))
            .expect("a day line with a hash");
        assert_eq!(run["final_hash"], last_hash, "seed {seed}: hash");

        let sunk: Vec<u64> = text
            .lines()
            .filter(|line| line.contains(": SUNK;"))
            .map(|line| line.split_whitespace().next().expect("an id"))
            .map(|id| id.trim_start_matches(|c: char| !c.is_ascii_digit()))
            .map(|id| id.parse().expect("a ship id"))
            .collect();
        let mut sunk = sunk;
        sunk.sort_unstable();
        let listed: Vec<u64> = run["sunk"]
            .as_array()
            .expect("sunk")
            .iter()
            .map(|id| id.as_u64().expect("id"))
            .collect();
        assert_eq!(listed, sunk, "seed {seed}: sunk ships");
    }
}

#[test]
fn two_identical_invocations_give_identical_bytes() {
    let first = batch("a", FIGHT, &["--runs", "5", "--first-seed", "40"]);
    let second = batch("b", FIGHT, &["--runs", "5", "--first-seed", "40"]);
    assert!(first == second);
    assert!(first.ends_with(b"\n"));
    assert_eq!(parse(&first)["per_run"][0]["seed"], 40);
}

#[test]
fn a_scenario_without_fighting_loses_nothing() {
    let summary = parse(&batch("quiet", QUIET, &["--runs", "3"]));
    for side in ["japanese", "allied"] {
        let block = &summary["sides"][side];
        assert_eq!(block["sunk_total"], 0);
        let histogram = block["sunk_histogram"].as_array().expect("histogram");
        assert_eq!(histogram[0], 3);
        assert!(histogram[1..].iter().all(|n| n == 0));
    }
    for ship in summary["per_ship"].as_array().expect("per_ship") {
        assert_eq!(ship["sunk_runs"], 0);
        assert_eq!(ship["crippled_runs"], 0);
    }
}

#[test]
fn a_scenario_with_objectives_adds_a_margin_and_a_result_to_each_run_and_a_tally() {
    let scenario = "data/scenarios/05_eastern_solomons.toml";
    let summary = parse(&batch(
        "objectives",
        scenario,
        &["--runs", "4", "--first-seed", "1940"],
    ));
    let runs = summary["per_run"].as_array().expect("per_run");
    assert_eq!(runs.len(), 4);
    let tally = summary["results"].as_array().expect("a tally of results");
    let names: Vec<&str> = tally
        .iter()
        .map(|row| row["outcome"].as_str().expect("an outcome"))
        .collect();
    assert_eq!(
        names,
        [
            "decisive victory",
            "major victory",
            "minor victory",
            "stalemate",
            "minor defeat",
            "major defeat",
            "decisive defeat"
        ]
    );
    let total: u64 = tally
        .iter()
        .map(|row| row["runs"].as_u64().expect("runs"))
        .sum();
    assert_eq!(total, 4);
    // Each run's own result is counted in the tally, and agrees with what
    // `run --seed` prints for the seed.
    for run in runs {
        let outcome = run["allied_outcome"].as_str().expect("an outcome");
        let row = tally
            .iter()
            .find(|row| row["outcome"] == outcome)
            .expect("a row");
        assert!(row["runs"].as_u64().unwrap() >= 1);
        let seed = run["seed"].as_u64().expect("seed").to_string();
        let printed = binary(&["run", scenario, "--seed", &seed]);
        let text = String::from_utf8_lossy(&printed.stdout).into_owned();
        let margin = run["allied_margin"].as_str().expect("a margin");
        assert!(
            text.contains(&format!("margin {margin}, {outcome}")),
            "seed {seed}: {margin} {outcome} not in\n{text}"
        );
    }
}

#[test]
fn a_scenario_without_objectives_has_no_result_in_its_batch() {
    let summary = parse(&batch("no_objectives", QUIET, &["--runs", "2"]));
    assert!(summary.get("results").is_none());
    assert!(summary["per_run"][0].get("allied_margin").is_none());
    assert!(summary["per_run"][0].get("allied_outcome").is_none());
}

#[test]
fn bad_invocations_fail_and_write_nothing() {
    let out = temp_file("never");
    let _ = fs::remove_file(&out);
    let out_text = out.to_string_lossy().into_owned();
    let cases: Vec<Vec<&str>> = vec![
        vec!["batch", QUIET],
        vec!["batch", QUIET, "--out", &out_text, "--runs", "0"],
        vec!["batch", QUIET, "--out", &out_text, "--runs", "many"],
        vec!["batch", QUIET, "--out", &out_text, "--runs"],
        vec!["batch", QUIET, "--out", &out_text, "--first-seed", "x"],
        vec!["batch", QUIET, "--out", &out_text, "--bogus"],
        vec!["batch", "--out", &out_text],
        vec!["batch", "data/scenarios/missing.toml", "--out", &out_text],
    ];
    for args in cases {
        let result = binary(&args);
        assert!(!result.status.success(), "{args:?} should fail");
        assert!(!result.stderr.is_empty(), "{args:?}: no message");
        assert!(!out.exists(), "{args:?}: wrote a file");
    }
}

#[test]
fn a_batch_says_what_it_was_run_on_and_carries_the_callers_label() {
    // Decision 0057: the hash layout's version, the hash of the scenario
    // file and of every data file it names, and the label as given.
    let plain = parse(&batch("inputs-plain", FIGHT, &["--runs", "2"]));
    let inputs = &plain["inputs"];
    assert_eq!(inputs["state_version"], "flagplot-state-v6");
    let scenario = fs::read(PathBuf::from(ROOT).join(FIGHT)).expect("the scenario reads");
    let mut hasher = sim_core::hash::StateHasher::new();
    hasher.bytes(&scenario);
    assert_eq!(
        inputs["scenario_hash"],
        format!("{:016x}", hasher.finish()).as_str()
    );
    let data = inputs["data"].as_array().expect("data files");
    assert!(!data.is_empty());
    for file in data {
        assert!(file["file"].as_str().is_some_and(|f| !f.is_empty()));
        assert_eq!(file["hash"].as_str().expect("a hash").len(), 16);
    }
    assert!(plain.get("label").is_none());

    let labelled = parse(&batch(
        "inputs-labelled",
        FIGHT,
        &["--runs", "2", "--label", "abc123"],
    ));
    assert_eq!(labelled["label"], "abc123");
    // The label changes nothing else.
    assert_eq!(labelled["inputs"], plain["inputs"]);
    assert_eq!(labelled["per_run"], plain["per_run"]);
}

// ------------------------------------------------- what a run counts (0062)

use std::collections::BTreeMap;

use sim_core::events::{EventKind, Weapon};
use sim_core::fixed::Fixed;
use sim_core::state::{ShipLocation, Side};

/// A scenario run by the day, as `batch` runs it: the world after each
/// day's end and the aircraft ready in each group then.
struct Played {
    scenario: sim_data::Scenario,
    ready_by_day: Vec<BTreeMap<u32, u32>>,
    hashes: Vec<u64>,
}

fn play(file: &str, seed: u64) -> Played {
    let path = PathBuf::from(ROOT).join(file);
    let mut scenario = sim_data::Scenario::load_with(&path, &sim_data::LoadOptions::default())
        .expect("the scenario loads")
        .with_seed(seed)
        .expect("a seed");
    scenario.world.set_config(sim_core::world::SimConfig {
        record_breakdowns: false,
    });
    let mut ready_by_day = Vec::new();
    let mut hashes = Vec::new();
    while let Some(summary) = scenario.next_day().expect("a day") {
        hashes.push(summary.state_hash);
        ready_by_day.push(
            scenario
                .world
                .umpire()
                .air_groups
                .values()
                .map(|group| (group.id.raw(), group.ready))
                .collect(),
        );
    }
    Played {
        scenario,
        ready_by_day,
        hashes,
    }
}

fn num(value: &Value) -> u64 {
    value.as_u64().expect("a whole number")
}

fn find<'a>(rows: &'a Value, key: &str, id: u64) -> Option<&'a Value> {
    rows.as_array()
        .expect("a list")
        .iter()
        .find(|row| row[key] == id)
}

const CAUSES: [&str; 7] = [
    "shot_down",
    "destroyed_on_ground",
    "damaged_destroyed_on_ground",
    "lost_with_ship",
    "worn_out",
    "ditched_fuel",
    "ditched_no_deck",
];

/// The ships with hits, from the log, as (gun, torpedo, bomb) by ship.
fn hits_by_ship(played: &Played) -> BTreeMap<u32, [u64; 3]> {
    let mut out: BTreeMap<u32, [u64; 3]> = BTreeMap::new();
    for event in played.scenario.world.events().iter() {
        let (ship, weapon, hits) = match &event.kind {
            EventKind::Salvo {
                target,
                weapon,
                hits,
                ..
            } => (target.raw(), *weapon, *hits),
            EventKind::AirAttack {
                ship, weapon, hits, ..
            } => (ship.raw(), *weapon, *hits),
            _ => continue,
        };
        if hits == 0 {
            continue;
        }
        let slot = match weapon {
            Weapon::Gun => 0,
            Weapon::Torpedo => 1,
            Weapon::Bomb => 2,
        };
        out.entry(ship).or_default()[slot] += u64::from(hits);
    }
    out
}

/// A system's name in lower snake case, as the file spells it.
fn snake(system: sim_core::state::ShipSystem) -> String {
    system.name().replace(' ', "_")
}

/// Checks every counted quantity of one run's record against the log.
fn check_run(file: &str, seed: u64, record: &Value, played: &Played) {
    let world = &played.scenario.world;
    let truth = world.umpire();
    let events: Vec<_> = world.events().iter().collect();
    let what = format!("{file} seed {seed}");

    // Hits by ship and weapon.
    let hits = hits_by_ship(played);
    let listed = record["hits_on"].as_array().expect("hits_on");
    assert_eq!(listed.len(), hits.len(), "{what}: ships hit");
    for (ship, [gun, torpedo, bomb]) in &hits {
        let row = find(&record["hits_on"], "ship", u64::from(*ship)).expect("a row");
        assert_eq!(
            [num(&row["gun"]), num(&row["torpedo"]), num(&row["bomb"])],
            [*gun, *torpedo, *bomb],
            "{what}: ship {ship}"
        );
    }

    // Fire by the firer's side: exact shots and hits.
    for (name, side) in [("japanese", Side::Japanese), ("allied", Side::Allied)] {
        for (weapon_name, weapon) in [("gun", Weapon::Gun), ("torpedo", Weapon::Torpedo)] {
            let mut shots = Fixed::ZERO;
            let mut hit_total = 0u64;
            for event in &events {
                if let EventKind::Salvo {
                    firer,
                    weapon: w,
                    shots: s,
                    hits,
                    ..
                } = &event.kind
                {
                    if *w == weapon && truth.ships[firer].side == side {
                        shots += *s;
                        hit_total += u64::from(*hits);
                    }
                }
            }
            let cell = &record["fire"][name][weapon_name];
            assert_eq!(cell["shots"], shots.to_string().as_str(), "{what} {name}");
            assert_eq!(num(&cell["hits"]), hit_total, "{what} {name} {weapon_name}");
        }
    }

    // Claims by claimant.
    for (name, side) in [("japanese", Side::Japanese), ("allied", Side::Allied)] {
        let mut sums = [0u64; 4];
        for event in &events {
            if let EventKind::ClaimReceived {
                side: s,
                claimed_sunk,
                claimed_hits,
                claimed_shot_down,
                claimed_on_ground,
                ..
            } = &event.kind
            {
                if *s == side {
                    sums[0] += u64::from(*claimed_sunk);
                    sums[1] += u64::from(*claimed_hits);
                    sums[2] += u64::from(*claimed_shot_down);
                    sums[3] += u64::from(*claimed_on_ground);
                }
            }
        }
        let claims = &record["claims"][name];
        let got = [
            num(&claims["sunk"]),
            num(&claims["hits"]),
            num(&claims["shot_down"]),
            num(&claims["on_ground"]),
        ];
        assert_eq!(got, sums, "{what}: {name} claims");
    }

    // Aircraft lost, by group and cause, and pilots.
    for group in truth.air_groups.keys().map(|id| id.raw()) {
        let mut sums = [0u64; 8];
        for event in &events {
            match &event.kind {
                EventKind::AircraftLost {
                    air_group,
                    aircraft,
                    ..
                } if air_group.raw() == group => sums[0] += u64::from(*aircraft),
                EventKind::AircraftDestroyedOnGround {
                    air_group,
                    aircraft,
                    ..
                } if air_group.raw() == group => sums[1] += u64::from(*aircraft),
                EventKind::DamagedAircraftDestroyedOnGround {
                    air_group,
                    aircraft,
                    ..
                } if air_group.raw() == group => sums[2] += u64::from(*aircraft),
                EventKind::AirGroupLost {
                    air_group,
                    aircraft,
                    ..
                } if air_group.raw() == group => sums[3] += u64::from(*aircraft),
                EventKind::AircraftWorn {
                    air_group,
                    written_off,
                    ..
                } if air_group.raw() == group => sums[4] += u64::from(*written_off),
                EventKind::AircraftDitched {
                    air_group, ditched, ..
                } if air_group.raw() == group => sums[5] += u64::from(*ditched),
                EventKind::AircraftDitchedNoDeck {
                    air_group,
                    aircraft,
                    ..
                } if air_group.raw() == group => sums[6] += u64::from(*aircraft),
                EventKind::PilotsLost {
                    air_group, lost, ..
                } if air_group.raw() == group => sums[7] += u64::from(*lost),
                _ => {}
            }
        }
        let row = find(&record["air_losses"], "air_group", u64::from(group));
        if sums == [0; 8] {
            assert!(row.is_none(), "{what}: group {group} lost nothing");
            continue;
        }
        let row = row.unwrap_or_else(|| panic!("{what}: group {group} has a row"));
        let mut got = CAUSES.map(|key| num(&row[key])).to_vec();
        got.push(num(&row["pilots_lost"]));
        assert_eq!(got, sums.to_vec(), "{what}: group {group}");
    }

    // The aircraft lost for good, by side, are the objectives' count.
    let tallies = sim_core::objectives::tallies(truth, world.events(), world.ref_data());
    for side in [Side::Allied, Side::Japanese] {
        // A side's tally holds the enemy's losses.
        let lost: u64 = record["air_losses"]
            .as_array()
            .expect("air_losses")
            .iter()
            .filter(|row| {
                let id = sim_core::ids::AirGroupId::new(num(&row["air_group"]) as u32);
                truth.air_groups[&id].side != side
            })
            .map(|row| CAUSES.iter().map(|key| num(&row[*key])).sum::<u64>())
            .sum();
        assert_eq!(lost, u64::from(tallies[&side].aircraft), "{what}: {side:?}");
    }

    // Ready at the end of each day; the last day is the final state.
    let days = played.ready_by_day.len();
    for row in record["ready"].as_array().expect("ready") {
        let id = num(&row["air_group"]) as u32;
        let by_day = row["by_day"].as_array().expect("by_day");
        assert_eq!(row["first_day"], 1, "{what}: by_day starts at day 1");
        assert_eq!(by_day.len(), days, "{what}: one number a day");
        for (index, ready) in by_day.iter().enumerate() {
            let expected = played.ready_by_day[index].get(&id).copied().unwrap_or(0);
            assert_eq!(
                num(ready),
                u64::from(expected),
                "{what}: group {id} day {}",
                index + 1
            );
        }
    }
    assert_eq!(
        record["ready"].as_array().expect("ready").len(),
        truth.air_groups.len(),
        "{what}: a row for every group"
    );

    // Systems out, for ships not sunk.
    let mut out = 0;
    for ship in truth.ships.values() {
        if ship.location == ShipLocation::Sunk {
            continue;
        }
        for system in sim_core::state::ShipSystem::ALL {
            if ship.systems.get(system) == sim_core::state::SystemState::Out {
                out += 1;
                let listed = record["systems_out"]
                    .as_array()
                    .expect("systems_out")
                    .iter()
                    .any(|row| row["ship"] == ship.id.raw() && row["system"] == snake(system));
                assert!(listed, "{what}: ship {} {}", ship.id.raw(), system.name());
            }
        }
    }
    assert_eq!(record["systems_out"].as_array().expect("list").len(), out);

    // Attacks, escorts and break-offs, in log order.
    let attacks: Vec<(u32, u32, u32)> = events
        .iter()
        .filter_map(|event| match &event.kind {
            EventKind::AirAttack {
                air_group, ship, ..
            } => Some((event.day, air_group.raw(), ship.raw())),
            EventKind::BaseAttacked {
                air_group, base, ..
            } => Some((event.day, air_group.raw(), base.raw())),
            _ => None,
        })
        .collect();
    let recorded: Vec<(u32, u32, u32)> = record["attacks"]
        .as_array()
        .expect("attacks")
        .iter()
        .map(|row| {
            (
                num(&row["day"]) as u32,
                num(&row["air_group"]) as u32,
                num(&row["target"]) as u32,
            )
        })
        .collect();
    assert_eq!(recorded, attacks, "{what}: attacks");
    let escorts = events
        .iter()
        .filter(|event| matches!(event.kind, EventKind::EscortsSent { .. }))
        .count();
    assert_eq!(
        record["escorts"].as_array().expect("escorts").len(),
        escorts
    );
    let rounds: Vec<u32> = events
        .iter()
        .filter_map(|event| match &event.kind {
            EventKind::ForceBrokeOff { round, .. } => Some(*round),
            _ => None,
        })
        .collect();
    let recorded: Vec<u32> = record["broke_off"]
        .as_array()
        .expect("broke_off")
        .iter()
        .map(|row| num(&row["round"]) as u32)
        .collect();
    assert_eq!(recorded, rounds, "{what}: break-offs");

    // One fight, one entry, in log order: the round it ended in.
    let ended: Vec<(u32, u32)> = events
        .iter()
        .filter_map(|event| match &event.kind {
            EventKind::EngagementEnded { round, .. } => Some((event.day, *round)),
            _ => None,
        })
        .collect();
    let fights: Vec<(u32, u32)> = record["fights"]
        .as_array()
        .expect("fights")
        .iter()
        .map(|row| (num(&row["day"]) as u32, num(&row["rounds"]) as u32))
        .collect();
    assert_eq!(fights, ended, "{what}: fights");
    assert!(record["fights"]
        .as_array()
        .expect("fights")
        .iter()
        .all(|row| ["night", "morning", "afternoon"]
            .contains(&row["pulse"].as_str().expect("a pulse"))));
}

#[test]
fn what_a_run_counts_matches_its_event_log_counted_again() {
    for (file, runs) in [
        ("data/scenarios/04_savo_island.toml", 3u64),
        ("data/scenarios/04a_night_action.toml", 3),
        ("data/scenarios/05_eastern_solomons.toml", 3),
        ("data/scenarios/06_rabaul_guadalcanal.toml", 2),
        ("data/scenarios/12c_no_deck.toml", 4),
    ] {
        let summary = parse(&batch("counts", file, &["--runs", &runs.to_string()]));
        for index in 0..runs {
            let seed = 1 + index;
            let record = &summary["per_run"][index as usize];
            assert_eq!(record["seed"], seed);
            check_run(file, seed, record, &play(file, seed));
        }
    }
}

#[test]
fn the_header_names_what_the_counts_name() {
    let summary = parse(&batch(
        "header",
        "data/scenarios/05_eastern_solomons.toml",
        &["--runs", "1"],
    ));
    let forces = summary["task_forces"].as_array().expect("task_forces");
    assert_eq!(forces.len(), 4);
    assert_eq!(forces[0]["name"], "Task Force 16");
    assert_eq!(forces[0]["side"], "allied");
    let groups = summary["air_groups"].as_array().expect("air_groups");
    assert_eq!(groups.len(), 20);
    assert_eq!(groups[10]["name"], "Shokaku fighter unit");
    assert_eq!(groups[10]["side"], "japanese");
    assert!(groups[10]["aircraft_type"]
        .as_str()
        .is_some_and(|t| !t.is_empty()));
    assert!(!summary["bases"].as_array().expect("bases").is_empty());
    // Every ship starts in a task force of the table.
    for ship in summary["ships"].as_array().expect("ships") {
        let force = ship["task_force"].as_u64().expect("a task force");
        assert!(find(&summary["task_forces"], "id", force).is_some());
    }
    // The Savo groups, as the measures name them.
    let savo = parse(&batch(
        "header-savo",
        "data/scenarios/04_savo_island.toml",
        &["--runs", "1"],
    ));
    let names: Vec<&str> = savo["task_forces"]
        .as_array()
        .expect("task_forces")
        .iter()
        .map(|force| force["name"].as_str().expect("a name"))
        .collect();
    assert!(names.contains(&"Southern group") && names.contains(&"Northern group"));
}

#[test]
fn a_scenario_with_nothing_to_count_still_writes_every_key_empty() {
    let summary = parse(&batch("empty", QUIET, &["--runs", "2"]));
    for key in ["task_forces", "air_groups", "bases"] {
        assert!(summary[key].is_array(), "{key}");
    }
    let run = &summary["per_run"][0];
    for key in [
        "hits_on",
        "systems_out",
        "air_losses",
        "ready",
        "attacks",
        "escorts",
        "broke_off",
        "fights",
        "supply",
        "formed_groups",
    ] {
        assert_eq!(run[key].as_array().expect(key).len(), 0, "{key}");
    }
    assert_eq!(run["fire"]["japanese"]["gun"]["shots"], "0");
    assert_eq!(run["claims"]["allied"]["sunk"], 0);
}

#[test]
fn shots_are_exact_decimal_strings_and_can_be_fractional() {
    // A damaged battery fires a part of a shot. The sum is written exactly,
    // not cut to a whole number.
    let file = "data/scenarios/04_savo_island.toml";
    let summary = parse(&batch("fractions", file, &["--runs", "12"]));
    let mut fractional = false;
    for (index, record) in summary["per_run"]
        .as_array()
        .expect("runs")
        .iter()
        .enumerate()
    {
        let played = play(file, 1 + index as u64);
        for (name, side) in [("japanese", Side::Japanese), ("allied", Side::Allied)] {
            let mut shots = Fixed::ZERO;
            for event in played.scenario.world.events().iter() {
                if let EventKind::Salvo {
                    firer,
                    weapon: Weapon::Gun,
                    shots: s,
                    ..
                } = &event.kind
                {
                    if played.scenario.world.umpire().ships[firer].side == side {
                        shots += *s;
                    }
                }
            }
            let text = record["fire"][name]["gun"]["shots"]
                .as_str()
                .expect("a string");
            assert_eq!(text, shots.to_string());
            fractional |= text.contains('.');
        }
    }
    assert!(fractional, "no Savo run in 12 had fractional gun shots");
}

#[test]
fn ditching_and_a_lost_deck_are_counted_by_cause_and_formed_groups_name_their_source() {
    let long = parse(&batch(
        "long-reach",
        "data/scenarios/12a_long_reach.toml",
        &["--runs", "12"],
    ));
    let fuel: u64 = long["per_run"]
        .as_array()
        .expect("runs")
        .iter()
        .flat_map(|run| run["air_losses"].as_array().expect("losses").iter())
        .map(|row| num(&row["ditched_fuel"]))
        .sum();
    assert!(fuel > 0, "12a ditches for want of fuel");

    let deck = parse(&batch(
        "no-deck",
        "data/scenarios/12c_no_deck.toml",
        &["--runs", "30"],
    ));
    let runs = deck["per_run"].as_array().expect("runs");
    let no_deck: u64 = runs
        .iter()
        .flat_map(|run| run["air_losses"].as_array().expect("losses").iter())
        .map(|row| num(&row["ditched_no_deck"]))
        .sum();
    assert!(no_deck > 0, "12c ditches with no deck");
    let formed: Vec<&Value> = runs
        .iter()
        .flat_map(|run| run["formed_groups"].as_array().expect("formed").iter())
        .collect();
    assert!(
        !formed.is_empty(),
        "12c forms a group by a landing elsewhere"
    );
    for group in formed {
        let from = num(&group["from"]);
        assert!(
            find(&deck["air_groups"], "id", from).is_some(),
            "the source group is one of the scenario's"
        );
        assert!(group["name"]
            .as_str()
            .expect("a name")
            .contains("landed at"));
    }
}

#[test]
fn the_day_by_day_loop_makes_the_same_run_as_running_to_the_end() {
    for file in [
        "data/scenarios/05_eastern_solomons.toml",
        "data/scenarios/06_rabaul_guadalcanal.toml",
    ] {
        for seed in [1u64, 7] {
            let path = PathBuf::from(ROOT).join(file);
            let mut whole = sim_data::Scenario::load_with(&path, &sim_data::LoadOptions::default())
                .expect("loads")
                .with_seed(seed)
                .expect("seed");
            whole.world.set_config(sim_core::world::SimConfig {
                record_breakdowns: false,
            });
            let summaries = whole.run_to_end().expect("runs");
            let by_day = play(file, seed);
            let hashes: Vec<u64> = summaries.iter().map(|s| s.state_hash).collect();
            assert_eq!(by_day.hashes, hashes, "{file} seed {seed}: daily hashes");
            let a: Vec<_> = whole.world.events().iter().collect();
            let b: Vec<_> = by_day.scenario.world.events().iter().collect();
            assert!(a == b, "{file} seed {seed}: the event logs differ");

            // And the batch's final hash is what `run --seed` prints.
            let summary = parse(&batch(
                "loop",
                file,
                &["--runs", "1", "--first-seed", &seed.to_string()],
            ));
            let printed = binary(&["run", file, "--seed", &seed.to_string()]);
            let text = String::from_utf8(printed.stdout).expect("UTF-8");
            let last = text
                .lines()
                .rfind(|line| line.starts_with("day "))
                .and_then(|line| line.split_whitespace().nth(3))
                .expect("a day line");
            assert_eq!(summary["per_run"][0]["final_hash"], last);
        }
    }
}

#[test]
fn the_keys_a_version_1_reader_used_keep_their_values() {
    // Files made by `batch` before decision 0062, three seeds each.
    // Everything they held is held, unchanged, except the version. There was
    // a captured file of scenario 5 too, made by the code of that day; it
    // could not be made again (the code is gone) and the scenario's orders
    // changed on Nov 12, 2026 (decision 0063, "As built (the scenario
    // orders)"), so its runs and its inputs no longer match any run today.
    // It is dropped, with its file. Scenarios 4a and 6 keep the test's
    // purpose: what a version 1 reader used is still where it was.
    for file in ["04a_night_action", "06_rabaul_guadalcanal"] {
        let before_path = PathBuf::from(ROOT)
            .join("crates/sim-cli/tests/batch_before")
            .join(format!("{file}.json"));
        let before: Value =
            serde_json::from_slice(&fs::read(before_path).expect("the old file")).expect("JSON");
        let after = parse(&batch(
            "old-reader",
            &format!("data/scenarios/{file}.toml"),
            &["--runs", "3"],
        ));
        assert_eq!(before["version"], 1);
        assert_eq!(after["version"], 2);
        for (key, value) in before.as_object().expect("an object") {
            if key == "version" {
                continue;
            }
            if key == "ships" || key == "per_run" {
                // Each entry gained keys; what it had is unchanged.
                let old = value.as_array().expect("a list");
                let new = after[key].as_array().expect("a list");
                assert_eq!(old.len(), new.len(), "{file}: {key}");
                for (old, new) in old.iter().zip(new) {
                    for (field, v) in old.as_object().expect("an object") {
                        assert_eq!(&new[field], v, "{file}: {key} {field}");
                    }
                }
            } else {
                assert_eq!(&after[key], value, "{file}: {key}");
            }
        }
    }
}

#[test]
fn the_seven_aircraft_causes_are_each_loss_once_and_sum_to_the_objectives_count() {
    // No aircraft is in two of the seven: a loss takes it out of `ready` or
    // `damaged` as it is counted. So per side their sum is the objectives'
    // `aircraft_destroyed` tally, which is built from the same events.
    for (file, runs) in [
        ("data/scenarios/05_eastern_solomons.toml", 6u64),
        ("data/scenarios/06_rabaul_guadalcanal.toml", 3),
        ("data/scenarios/12c_no_deck.toml", 8),
    ] {
        let summary = parse(&batch("causes", file, &["--runs", &runs.to_string()]));
        for index in 0..runs {
            let played = play(file, 1 + index);
            let world = &played.scenario.world;
            let tallies =
                sim_core::objectives::tallies(world.umpire(), world.events(), world.ref_data());
            for side in [Side::Allied, Side::Japanese] {
                let lost: u64 = summary["per_run"][index as usize]["air_losses"]
                    .as_array()
                    .expect("air_losses")
                    .iter()
                    .filter(|row| {
                        let id = sim_core::ids::AirGroupId::new(num(&row["air_group"]) as u32);
                        world.umpire().air_groups[&id].side != side
                    })
                    .map(|row| CAUSES.iter().map(|key| num(&row[*key])).sum::<u64>())
                    .sum();
                assert_eq!(lost, u64::from(tallies[&side].aircraft), "{file} {index}");
            }
        }
    }
}
