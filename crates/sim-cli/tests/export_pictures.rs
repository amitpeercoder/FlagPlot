//! The export of command pictures and of the reports sent (replay version 6,
//! decision 0138, item 6), against the hand calculations of scenarios 62, 63
//! and 64 (`tests/scenario_6{2,3,4}_*.rs`), which say which command held
//! which contact after each pulse and which reports were sent to whom.
//!
//! The expected figures below are those tables, read off them: nothing here
//! is a number the code was made to give.
//!
//! The export of a scenario with command levels off is what it was before
//! version 6, but for the version number: its bytes, with the version put
//! back to 5, have the hash that the export made on the commit before this
//! one had (decision 0138, item 3).

use std::collections::BTreeMap;
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use std::sync::atomic::{AtomicUsize, Ordering};

use serde_json::Value;

const ROOT: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../..");

static COUNT: AtomicUsize = AtomicUsize::new(0);

fn export_bytes(label: &str, scenario: &str) -> Vec<u8> {
    // One file for each call: the tests run side by side.
    let label = format!("{label}-{}", COUNT.fetch_add(1, Ordering::SeqCst));
    let dir = std::env::temp_dir().join(format!("flagplot-export-pictures-{}", std::process::id()));
    fs::create_dir_all(&dir).expect("make the temp dir");
    let out: PathBuf = dir.join(format!("{label}.json"));
    let result = Command::new(env!("CARGO_BIN_EXE_sim-cli"))
        .args(["export", scenario, "--out"])
        .arg(&out)
        .current_dir(ROOT)
        .output()
        .expect("the binary starts");
    assert!(
        result.status.success(),
        "{scenario}: {}",
        String::from_utf8_lossy(&result.stderr)
    );
    let bytes = fs::read(&out).expect("the export was written");
    let _ = fs::remove_file(&out);
    bytes
}

fn export(scenario: &str) -> Value {
    let label = scenario.rsplit('/').next().expect("a file name");
    serde_json::from_slice(&export_bytes(label, scenario)).expect("the export is JSON")
}

/// A step's `(day, phase)` and the pictures after it.
type Step = ((u64, String), BTreeMap<String, Held>);

/// A listener as `(command, direction, arrival, day and pulse)`.
type Listener = (String, String, String, Option<(u64, String)>);

/// What a command holds: contact number, age and whether it is the
/// command's own, in contact order.
type Held = Vec<(u64, u64, bool)>;

/// The pictures of `side` after each step, by the steps applied in order:
/// `(day, phase)` to command to what it holds.
fn pictures_by_step(replay: &Value, side: &str) -> Vec<Step> {
    let mut state: BTreeMap<String, Held> = BTreeMap::new();
    let mut out = Vec::new();
    for step in replay["steps"].as_array().expect("steps") {
        if let Some(list) = step["changed"]["pictures"][side].as_array() {
            for picture in list {
                let held = picture["contacts"]
                    .as_array()
                    .expect("contacts")
                    .iter()
                    .map(|c| {
                        (
                            c["id"].as_u64().expect("id"),
                            c["age_pulses"].as_u64().expect("age"),
                            c["own"].as_bool().expect("own"),
                        )
                    })
                    .collect();
                state.insert(picture["command"].as_str().expect("command").into(), held);
            }
        }
        if let Some(list) = step["removed"]["pictures"][side].as_array() {
            for command in list {
                assert!(state.remove(command.as_str().expect("command")).is_some());
            }
        }
        out.push((
            (
                step["day"].as_u64().expect("day"),
                step["phase"].as_str().expect("phase").to_string(),
            ),
            state.clone(),
        ));
    }
    out
}

fn at<'a>(steps: &'a [Step], day: u64, phase: &str) -> &'a BTreeMap<String, Held> {
    &steps
        .iter()
        .find(|((d, p), _)| *d == day && p == phase)
        .unwrap_or_else(|| panic!("no step day {day} {phase}"))
        .1
}

/// Contact 1's age in each command's picture, `None` for no contact.
fn ages(held: &BTreeMap<String, Held>) -> BTreeMap<&str, Option<u64>> {
    held.iter()
        .map(|(command, held)| {
            assert!(held.len() <= 1, "{command}: more than contact 1");
            (command.as_str(), held.first().map(|(_, age, _)| *age))
        })
        .collect()
}

fn listeners(event: &Value) -> Vec<Listener> {
    event["data"]["listeners"]
        .as_array()
        .expect("listeners")
        .iter()
        .map(|l| {
            (
                l["command"].as_str().expect("command").to_string(),
                l["direction"].as_str().expect("direction").to_string(),
                l["arrival"].as_str().expect("arrival").to_string(),
                l["day"]
                    .as_u64()
                    .map(|day| (day, l["pulse"].as_str().expect("pulse").to_string())),
            )
        })
        .collect()
}

fn reports_sent(replay: &Value) -> Vec<&Value> {
    replay["events"]
        .as_array()
        .expect("events")
        .iter()
        .filter(|e| e["kind"] == "report_sent")
        .collect()
}

fn word(command: &str, direction: &str, arrival: &str, when: Option<(u64, &str)>) -> Listener {
    (
        command.into(),
        direction.into(),
        arrival.into(),
        when.map(|(day, pulse)| (day, pulse.to_string())),
    )
}

/// Scenario 62, "The pictures": contact 1 by its age, or none, in each
/// command's picture after each pulse.
#[test]
fn scenario_62_pictures_are_the_hand_calculations_table() {
    let replay = export("data/scenarios/62_reports_between_commands.toml");
    assert_eq!(replay["version"], 6);
    let steps = pictures_by_step(&replay, "allied");
    // (step, task force 1, the area command, task force 2, base 1)
    let table: [(u64, &str, [Option<u64>; 4]); 6] = [
        (0, "start", [Some(0), Some(0), Some(0), Some(0)]),
        (1, "night", [Some(1), Some(1), Some(1), Some(1)]),
        (1, "morning", [None, None, Some(2), Some(2)]),
        (1, "afternoon", [None, None, Some(3), Some(3)]),
        (2, "night", [None, None, Some(4), Some(4)]),
        (2, "morning", [None, None, None, None]),
    ];
    for (day, phase, [tf1, area, tf2, base1]) in table {
        let held = ages(at(&steps, day, phase));
        assert_eq!(held["task_force:1"], tf1, "day {day} {phase}: task force 1");
        assert_eq!(held["area"], area, "day {day} {phase}: the area command");
        assert_eq!(held["task_force:2"], tf2, "day {day} {phase}: task force 2");
        assert_eq!(held["base:1"], base1, "day {day} {phase}: base 1");
        assert_eq!(held.len(), 4, "the four Allied commands");
    }
    // Every contact there is was told: the scenario gave it to every command.
    assert!(steps[0].1.values().flatten().all(|(_, _, own)| !own));
    // The last applied state is what `final` gives.
    for side in ["allied", "japanese"] {
        let last = &pictures_by_step(&replay, side)
            .last()
            .expect("steps")
            .1
            .clone();
        let wanted: BTreeMap<String, usize> = replay["final"]["pictures"][side]
            .as_array()
            .expect("final pictures")
            .iter()
            .map(|p| {
                (
                    p["command"].as_str().expect("command").to_string(),
                    p["contacts"].as_array().expect("contacts").len(),
                )
            })
            .collect();
        let have: BTreeMap<String, usize> =
            last.iter().map(|(c, h)| (c.clone(), h.len())).collect();
        assert_eq!(have, wanted, "{side}: the steps end at final");
    }
}

/// Scenario 62, "The report": two "report sent" events in the morning of
/// day 1, the first with three listeners and the second with two.
#[test]
fn scenario_62_reports_sent_name_who_got_each_and_when() {
    let replay = export("data/scenarios/62_reports_between_commands.toml");
    let sent = reports_sent(&replay);
    assert_eq!(sent.len(), 2);
    let (first, second) = (sent[0], sent[1]);
    for event in [first, second] {
        assert_eq!(event["day"], 1);
        assert_eq!(event["pulse"], "morning");
        assert_eq!(event["data"]["side"], "allied");
    }
    assert_eq!(first["data"]["sender"], "task_force:1");
    assert_eq!(
        listeners(first),
        vec![
            word("area", "up", "on_time", Some((1, "morning"))),
            word("task_force:2", "sideways", "never", None),
            word("base:1", "sideways", "never", None),
        ]
    );
    assert_eq!(second["data"]["sender"], "area");
    assert_eq!(
        listeners(second),
        vec![
            word("task_force:2", "down", "never", None),
            word("base:1", "down", "never", None),
        ]
    );
    // Both carry the report's event: "nothing there" is the strike's own
    // event, which both name; the first is its cause.
    assert_eq!(first["data"]["report"], second["data"]["report"]);
    let report = first["data"]["report"].as_u64().expect("report number");
    let events = replay["events"].as_array().expect("events");
    assert!(events.iter().any(|e| e["id"] == report));
    assert_eq!(first["causes"][0]["event"], report);
}

/// Scenario 63: every chance certain. "Nothing there" is task force 1's:
/// up to the area command and sideways to base 1 on time, in the morning of
/// day 1, and the area command drops contact 1 with base 1 and her.
#[test]
fn scenario_63_pictures_and_reports() {
    let replay = export("data/scenarios/63_a_superiors_knowledge.toml");
    let steps = pictures_by_step(&replay, "allied");
    for (phase, day, age) in [("start", 0, Some(0)), ("night", 1, Some(1))] {
        let held = ages(at(&steps, day, phase));
        for command in ["area", "task_force:1", "base:1"] {
            assert_eq!(held[command], age, "{phase}: {command}");
        }
    }
    for (day, phase) in [(1, "morning"), (1, "afternoon"), (2, "night"), (3, "end")] {
        let held = ages(at(&steps, day, phase));
        assert!(held.values().all(Option::is_none), "day {day} {phase}");
    }
    let sent = reports_sent(&replay);
    assert_eq!(sent[0]["data"]["sender"], "task_force:1");
    assert_eq!(
        listeners(sent[0]),
        vec![
            word("area", "up", "on_time", Some((1, "morning"))),
            word("base:1", "sideways", "on_time", Some((1, "morning"))),
        ]
    );
    assert_eq!(sent[1]["data"]["sender"], "area");
    assert_eq!(
        listeners(sent[1]),
        vec![word("base:1", "down", "on_time", Some((1, "morning")))]
    );
    assert_eq!(sent.len(), 2);
}

/// Scenario 64, "The pictures": contact 1 is in task force 1's picture, as
/// its own, and in no other command's, at any time; it is dropped at the
/// end of the morning of day 2. No report is sent of a contact a scenario
/// says one command saw.
#[test]
fn scenario_64_the_contact_is_task_force_1s_own_alone() {
    let replay = export("data/scenarios/64_commanders_and_initiative.toml");
    let steps = pictures_by_step(&replay, "allied");
    for (day, phase, age) in [
        (0, "start", Some(0)),
        (1, "night", Some(1)),
        (1, "morning", Some(2)),
        (1, "afternoon", Some(3)),
        (2, "night", Some(4)),
        (2, "morning", None),
    ] {
        let held = ages(at(&steps, day, phase));
        assert_eq!(held["task_force:1"], age, "day {day} {phase}");
        assert_eq!(held["area"], None, "day {day} {phase}: the area command");
        assert_eq!(held["base:1"], None, "day {day} {phase}: base 1");
    }
    let held = at(&steps, 1, "night");
    assert!(held["task_force:1"].iter().all(|(_, _, own)| *own));
    // A contact a scenario says one command saw is sent to no one, and no
    // strike or search of this scenario makes a report: none is sent.
    assert!(reports_sent(&replay).is_empty());
}

/// A step carries a command's picture only where it differs from the step
/// before: a step in which no picture changed has no `pictures`, and a
/// picture that did not change is not repeated.
#[test]
fn a_step_carries_only_the_pictures_that_changed() {
    let replay = export("data/scenarios/62_reports_between_commands.toml");
    let steps = replay["steps"].as_array().expect("steps");
    let named = |step: &Value| -> Vec<String> {
        step["changed"]["pictures"]["allied"]
            .as_array()
            .map(|l| {
                l.iter()
                    .map(|p| p["command"].as_str().expect("command").to_string())
                    .collect()
            })
            .unwrap_or_default()
    };
    // The first step: every command of each side.
    assert_eq!(
        named(&steps[0]),
        ["area", "task_force:1", "task_force:2", "base:1"]
    );
    // Day 1's orders: nothing yet changed.
    assert!(steps[1]["changed"]["pictures"].is_null());
    // The morning of day 1: task force 1 and the area command dropped the
    // contact; the other two aged it.
    let morning = steps
        .iter()
        .find(|s| s["day"] == 1 && s["phase"] == "morning")
        .expect("the step");
    assert_eq!(
        named(morning),
        ["area", "task_force:1", "task_force:2", "base:1"]
    );
    // The end of day 1 is the afternoon's state: nothing changed.
    let end = steps
        .iter()
        .find(|s| s["day"] == 1 && s["phase"] == "end")
        .expect("the step");
    assert!(end["changed"]["pictures"].is_null());
    assert!(end["removed"]["pictures"].is_null());
}

/// Each scenario with command levels off exports what it did before version
/// 6, but for the version: no `pictures`, no `report_sent`, and, with the
/// version put back to 5, the bytes of the export made on the commit before
/// (their FNV-1a 64-bit hash). Remade at the state layout v44 (decision
/// 0151), which changes an export's state version and day hashes and gives
/// every intent's kind the key `island` (null but for "control an island"),
/// and nothing else of it: every earlier scenario gives the events it gave.
/// Remade again at the state layout v45 (decision 0159, step 2), which
/// changes an export's state version and day hashes and nothing else.
/// Remade again at the state layout v46 (decision 0159, step 3), which
/// changes an export's state version and day hashes and nothing else.
/// Remade again at the state layout v47 (decision 0159, step 4), which
/// changes an export's state version and day hashes and nothing else.
/// Remade again at the state layout v48 (decision 0162), which changes an
/// export's state version and day hashes and nothing else.
/// Remade again for decision 0167, in the four scenarios with an air attack
/// or a raid (5, 31, 50 and 58): each changes in the hash of
/// `air_combat.toml`, the words of those events (the true event's type where
/// recognised; the attacked side's weapon, and its type recognised or "type
/// not made out"), their `recognised`
/// field where a type was recognised and the attacked side's `weapon`, and
/// nothing else: with those taken back out each file hashes as pinned before.
/// Remade again for decision 0168, in the same four scenarios: each changes
/// in the hash of `air_combat.toml` alone, which gained
/// `raid_recognised_experience_strength`; with the old hash put back each
/// file hashes as pinned before.
/// Remade again for decisions 0179 and 0180, built together (the state
/// layout v49): every file changes in its state version and day hashes, and
/// those with a raid's claim (5, 31, 50 and 58) in the claim's words, its
/// `claimed_by` and `ground_types` and its breakdown's factors, and nothing
/// else: checked against the exports of the commit before, with those taken
/// out. Remade again when a claim's event came to name its units as its
/// entities (decision 0180, on the architect's word): scenarios 5, 31 and 50
/// change in the `entities` of their raids' claims and in nothing else.
/// Remade again at the state layout v50 (decision 0191), which changes an
/// export's state version and day hashes and nothing else: with the old
/// version and the day hashes of the run baselines before it put back, each
/// file hashes as pinned before.
/// Remade again at the state layout v51 (decision 0190), which changes an
/// export's state version and day hashes and nothing else: checked against
/// the exports of the commit before, with those taken out.
/// Remade for decision 0199, on the designer's word of Nov 21, 2026: an
/// export of a game on a generated map carries the map's frame (rule 30).
/// Of the ten, only scenario 5 is on such a map; its export gains
/// `"frame":{"centre_lat":-12,"centre_lon":160,"factor":0.978148,"west":145,"east":172},`
/// and nothing else: with that taken out it hashes as pinned before
/// (0x090e_e743_ff55_2f54), and the other nine hash as they did.
/// Remade again at the state layout v53 (decision 0205), which changes an
/// export's state version and day hashes and nothing else: checked against
/// the exports of the commit before, with those taken out, each of which
/// hashes as pinned before.
#[test]
fn a_scenario_with_command_levels_off_exports_as_it_did() {
    fn fnv(bytes: &[u8]) -> u64 {
        bytes.iter().fold(0xcbf2_9ce4_8422_2325, |h, b| {
            (h ^ u64::from(*b)).wrapping_mul(0x0100_0000_01b3)
        })
    }
    let before: [(&str, u64); 10] = [
        ("01_transit", 0x8bc1_8b6c_e4ae_b78b),
        ("04a_night_action", 0xeaf8_146b_9889_2fea),
        ("05_eastern_solomons", 0x0414_5eac_952d_1cd4),
        ("21_landing", 0x212b_8225_6624_4d84),
        ("23_fight_ashore", 0x7d7c_7119_8ea3_2f04),
        ("31_airfield_damage", 0xcb0e_caa9_efd5_1749),
        ("36_one_convoy", 0x2732_7152_1f3a_0c52),
        ("43_keep_two_bases_fed", 0xcd66_0120_8212_f71b),
        ("50_neutralise_a_base", 0x95d5_f42d_eb36_a6c2),
        ("58_a_carrier_force_closes", 0xac7d_ed67_d30a_379c),
    ];
    let prefix6 = br#"{"format":"flagplot-replay","version":6,"#;
    let prefix5 = br#"{"format":"flagplot-replay","version":5,"#;
    for (stem, hash) in before {
        let bytes = export_bytes(stem, &format!("data/scenarios/{stem}.toml"));
        assert!(bytes.starts_with(prefix6), "{stem}: version 6 first");
        let text = String::from_utf8_lossy(&bytes);
        assert!(!text.contains("\"pictures\""), "{stem}: has pictures");
        assert!(!text.contains("\"report_sent\""), "{stem}: has a report");
        let mut old = prefix5.to_vec();
        old.extend_from_slice(&bytes[prefix6.len()..]);
        assert_eq!(fnv(&old), hash, "{stem}: not the export it was");
    }
}
