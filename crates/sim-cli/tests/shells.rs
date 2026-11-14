//! Shells and bombs on a base, and what a side knows of the enemy ashore, in
//! the exports, the batch file, the run text and the MCP text (decision
//! 0092), on scenario 24 and, for what must not change, on scenario 6a.
//!
//! Scenario 24's night of day 1: the Japanese cruiser fires 720 shells and
//! the destroyer 600 (their rounds aboard), 1320 in all and 870 weighed shots;
//! on day 2 the cruiser's last 80 rounds. So the Japanese side fired 1400
//! shells in the run. The hits, the claim and the men lost to fire are the
//! rolls' own and are checked against each other: a hit that lands on men
//! removes 10, as many as are there.

use std::fs;
use std::path::PathBuf;
use std::process::Command;

use serde_json::Value;

const ROOT: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../..");
const SCENARIO: &str = "data/scenarios/24_shells_and_estimates.toml";

fn binary(args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_sim-cli"))
        .args(args)
        .current_dir(ROOT)
        .output()
        .expect("the binary starts")
}

fn temp_file(label: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("flagplot-shells-test-{}", std::process::id()));
    fs::create_dir_all(&dir).expect("make the temp dir");
    dir.join(format!("{label}.json"))
}

/// Runs a command that writes a JSON file with `--out` and returns it.
fn json_out(command: &str, scenario: &str, label: &str, extra: &[&str]) -> Value {
    let out = temp_file(label);
    let out_text = out.to_string_lossy().into_owned();
    let mut args = vec![command, scenario, "--out", out_text.as_str()];
    args.extend_from_slice(extra);
    let result = binary(&args);
    assert!(
        result.status.success(),
        "{command} of {scenario} failed: {}",
        String::from_utf8_lossy(&result.stderr)
    );
    let value = serde_json::from_slice(&fs::read(&out).expect("the file exists")).expect("JSON");
    let _ = fs::remove_file(&out);
    value
}

fn of_kind<'a>(events: &'a [Value], kind: &str) -> Vec<&'a Value> {
    events.iter().filter(|e| e["kind"] == kind).collect()
}

#[test]
fn the_export_writes_the_new_events_with_every_field() {
    let replay = json_out("export", SCENARIO, "export", &[]);
    let events = replay["events"].as_array().expect("events");

    let fire = of_kind(events, "bombardment_fire");
    assert_eq!(fire.len(), 3, "two ships on night 1, one on night 2");
    let d = &fire[0]["data"];
    assert_eq!(d["shells"], 720);
    assert_eq!(d["weighed_shots"], 720);
    assert_eq!(d["rounds_left"], 80);
    for key in [
        "ship",
        "base",
        "chance",
        "expected",
        "hits",
        "hits_on_aircraft",
        "hits_on_supply",
        "hits_on_men",
    ] {
        assert!(d.get(key).is_some(), "bombardment_fire lacks {key}: {d}");
    }

    let shore = of_kind(events, "shore_bombardment");
    assert_eq!(shore.len(), 2);
    let d = &shore[0]["data"];
    assert_eq!(d["shells"], 1320);
    assert_eq!(d["weighed_shots"], 870);
    let hits = d["hits"].as_u64().unwrap();
    let parts: u64 = ["hits_on_aircraft", "hits_on_supply", "hits_on_men"]
        .iter()
        .map(|k| d[*k].as_u64().unwrap())
        .sum();
    assert!(parts <= hits, "{d}");
    assert_eq!(d["hex"], serde_json::json!([0, 0]));
    assert!(d.get("task_force").is_some());

    // Men lost to fire: 10 for each hit that landed on men, in the log and
    // in the unit's men ashore.
    let lost = of_kind(events, "men_lost_to_fire");
    assert_eq!(lost.len(), 2);
    assert_eq!(
        lost[0]["data"]["lost"].as_u64().unwrap(),
        10 * d["hits_on_men"].as_u64().unwrap()
    );
    assert_eq!(
        lost[0]["data"]["ashore"].as_u64().unwrap(),
        1000 - lost[0]["data"]["lost"].as_u64().unwrap()
    );

    // The claim of a bombardment is marked and says aircraft on the ground.
    let claims: Vec<&Value> = of_kind(events, "claim_received")
        .into_iter()
        .filter(|e| e["data"].get("from_bombardment").is_some())
        .collect();
    assert_eq!(claims.len(), 2);
    assert_eq!(claims[0]["data"]["from_bombardment"], true);
    assert!(claims[0]["data"]["claimed_on_ground"].is_u64());
    assert!(claims[0]["text"]
        .as_str()
        .unwrap()
        .contains("claim from the sea at (0,0)"));

    // The ground reports and the base found: the opening estimate is day 0.
    let reports = of_kind(events, "ground_report_received");
    assert_eq!(reports[0]["day"], 0);
    assert_eq!(reports[0]["data"]["men"], 300);
    assert_eq!(reports[0]["data"]["side"], "japanese");
    assert_eq!(reports[0]["data"]["replaced"], false);
    assert!(reports.iter().skip(1).all(|e| e["day"] != 0));
    let found = of_kind(events, "base_found");
    assert_eq!(found.len(), 1);
    assert_eq!(found[0]["data"]["side"], "allied");
    assert_eq!(found[0]["data"]["base"], 3);

    // The final state has each side's estimates beside its contacts.
    let estimates = &replay["final"]["ground_estimates"];
    for side in ["allied", "japanese"] {
        let list = estimates[side].as_array().expect("a list");
        assert_eq!(list.len(), 1, "{estimates}");
        assert_eq!(list[0]["hex"], serde_json::json!([8, 0]));
        // The Japanese battalion lives through the fights since the limit on
        // engaged men (decision 0105), so the reports go on to day 4.
        assert_eq!(list[0]["day"], 4);
    }
}

#[test]
fn a_side_is_exported_only_what_it_may_see_of_a_bombardment() {
    let replay = json_out("export", SCENARIO, "sides", &[]);
    let allied = replay["side_views"]["allied"]["events"]
        .as_array()
        .expect("events");
    let japanese = replay["side_views"]["japanese"]["events"]
        .as_array()
        .expect("events");

    // The base's side: the shots and where they landed, never the force.
    let against = of_kind(allied, "shore_bombardment");
    assert_eq!(against.len(), 2);
    for key in [
        "base",
        "hex",
        "shells",
        "weighed_shots",
        "hits",
        "hits_on_aircraft",
        "hits_on_supply",
        "hits_on_men",
    ] {
        assert!(against[0]["data"].get(key).is_some(), "lacks {key}");
    }
    assert!(against[0]["data"].get("task_force").is_none());
    assert_eq!(against[0]["cut"], true);
    for e in allied {
        let text = e["text"].as_str().unwrap();
        assert!(
            !text.contains("task_force:") && !text.contains("ship:"),
            "the Allied side names the enemy's ships: {text}"
        );
    }
    // It sees its own men lost to fire whole, and a ground report of its own.
    assert_eq!(of_kind(allied, "men_lost_to_fire").len(), 2);
    assert_eq!(of_kind(allied, "base_found").len(), 1);

    // The firing side: its shells and its claim, never a hit; and it sees
    // nothing of the men the shells killed.
    let fired = of_kind(japanese, "shore_bombardment");
    assert_eq!(fired.len(), 2);
    let d = &fired[0]["data"];
    assert_eq!(d["shells"], 1320);
    for key in ["hits", "hits_on_aircraft", "hits_on_supply", "hits_on_men"] {
        assert!(d.get(key).is_none(), "the firing side is told {key}");
    }
    for e in of_kind(japanese, "bombardment_fire") {
        for key in ["chance", "expected", "hits", "hits_on_men"] {
            assert!(
                e["data"].get(key).is_none(),
                "the firing side is told {key}"
            );
        }
    }
    assert!(of_kind(japanese, "men_lost_to_fire").is_empty());
    assert_eq!(of_kind(japanese, "claim_received").len(), 2);
    // A side's ground report is its own: the Japanese side holds the opening
    // estimate and the four reports of the Allied men (one for each day of
    // fighting, decision 0105), never the Allied ones.
    let reports = of_kind(japanese, "ground_report_received");
    assert_eq!(reports.len(), 5);
    assert!(reports.iter().all(|e| e["data"]["side"] == "japanese"));
}

#[test]
fn a_raid_exports_hits_on_men_only_when_one_landed() {
    // Scenario 6a has no land unit: its exports carry no `hits_on_men`, so
    // they are what they were.
    let replay = json_out("export", "data/scenarios/06a_base_raid.toml", "raid", &[]);
    let events = replay["events"].as_array().expect("events");
    let raids = of_kind(events, "base_attacked");
    assert!(!raids.is_empty());
    assert!(raids.iter().all(|e| e["data"].get("hits_on_men").is_none()));
    assert!(replay["final"].get("ground_estimates").is_none());
}

#[test]
fn the_batch_counts_men_lost_by_cause_and_shells_fired_by_side() {
    let batch = json_out("batch", SCENARIO, "batch", &["--runs", "3"]);
    assert_eq!(batch["version"], 5);
    for run in batch["per_run"].as_array().expect("runs") {
        // The Japanese side fired 1400 shells and the Allied none.
        assert_eq!(run["bombardment_shells"]["japanese"], "1400");
        assert_eq!(run["bombardment_shells"]["allied"], "0");
        let lost = &run["men_lost"];
        // Men are lost to fire only by the side shelled, 10 a hit on men less
        // what the regiment's fieldworks save (decision 0106), never more
        // than its 1000; and in the fight ashore a part of the Japanese
        // battalion's 800 men (the division fires with no more men than the
        // limit on engaged men, decision 0105) and the Allied division's few,
        // 20 a hit; none are unfed or lost with a ship.
        assert_eq!(lost["japanese"]["fire"], 0);
        assert!(lost["allied"]["fire"].as_u64().unwrap() <= 1000);
        let japanese_fight = lost["japanese"]["fight"].as_u64().unwrap();
        assert!(japanese_fight > 0 && japanese_fight < 800 && japanese_fight % 20 == 0);
        assert!(lost["allied"]["fight"].as_u64().unwrap() <= 3000);
        for side in ["allied", "japanese"] {
            assert_eq!(lost[side]["unfed"], 0);
            assert_eq!(lost[side]["lost_with_ship"], 0);
        }
    }
}

#[test]
fn a_run_with_no_man_lost_and_no_shell_fired_has_neither_key() {
    let batch = json_out(
        "batch",
        "data/scenarios/01_transit.toml",
        "quiet",
        &["--runs", "2"],
    );
    for run in batch["per_run"].as_array().expect("runs") {
        assert!(run.get("men_lost").is_none());
        assert!(run.get("bombardment_shells").is_none());
    }
}

#[test]
fn run_and_events_print_the_new_kinds() {
    let out = binary(&["run", SCENARIO, "--events"]);
    assert!(out.status.success());
    let text = String::from_utf8_lossy(&out.stdout);
    for phrase in [
        "ship:1 shelled base:1: 720 shells, 720 weighed shots x 0.05",
        "task_force:1 bombarded base:1 at (0,0): 1320 shells, 870 weighed shots",
        "to fire on base:1 at (0,0)",
        "Japanese ground report at (8,0): 300 enemy men ashore",
        "Allied found enemy base:3 at (8,0)",
        "Japanese claim from the sea at (0,0)",
        "Japanese estimate at (8,0): ",
        "Allied estimate at (8,0): ",
    ] {
        assert!(text.contains(phrase), "run output lacks: {phrase}");
    }
    // The umpire's question names the force's bombardment.
    let out = binary(&["events", SCENARIO, "--unit", "task_force:1"]);
    assert!(out.status.success());
    assert!(String::from_utf8_lossy(&out.stdout).contains("bombarded base:1"));
}
