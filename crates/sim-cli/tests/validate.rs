//! Integration tests of `sim-cli validate` (`docs/step2-spec.md`, "What
//! counts as agreement" and "The harness").
//!
//! # The architect's hand calculation
//!
//! The fixture is a batch of 20 runs, seeds 1 to 20, made by hand. Its header
//! has ships 1 and 2 (Japanese, task force 1) and ships 3 and 4 (Allied, task
//! force 2); task forces 1 and 2; air groups 1 (Japanese), 2 (Allied) and 5
//! (Japanese); bases 1 and 2; two days. Everything not stated below is empty
//! or nothing. Its inputs are those of scenario 4a, so they match the disk.
//!
//! Per run:
//! - `sunk`: runs 1 to 10 none; runs 11 to 17 ship 1; run 18 ships 1 and 3;
//!   runs 19 and 20 ships 1, 2 and 3.
//! - Hits: ship 3 takes 2 gun hits in runs 11 to 20.
//! - Japanese gun fire: run 1 shots "0" hits 0; run 2 "938" and 35; run 3
//!   "12.5" and 1; run 4 "7.5" and 1; runs 5 to 20 "100" and 10.
//! - Japanese claims `shot_down`: run 1 gets 7, run 2 gets 5, the rest 0.
//!   Japanese claims `sunk`: 2 in run 18, 1 in run 19, 1 in run 20, else 0.
//! - Air group 2 `shot_down`: run 1 gets 3, run 2 none, runs 3 to 20 one.
//! - Fights: run 1 has two, of 2 and 3 rounds, both in the "night" pulse. Run
//!   2 has one in the "morning" pulse of 1 round. Run 3 has one "night" fight
//!   of 1 round and one "afternoon" fight of 1 round. No other run has a
//!   fight.
//! - Attacks in run 1, all day 1 afternoon by group 1: 12 aircraft on ship 3,
//!   5 on ship 4, 9 on ship 1. Escorts in run 1 with group 1: 6 in the
//!   afternoon and 4 in the morning.
//! - Ready on day 2 of group 2: 4 in every run. Run 5 has group 3, formed
//!   from group 2, with 2 ready on day 2. Run 6 has group 3 formed from 2 and
//!   group 4 formed from 3, each with 1.
//!
//! With 20 runs, the 5th percentile is rank ceil(5 x 20 / 100) = 1, the
//! median rank 10, the 95th rank 19; with 19 kept, ranks ceil(0.95) = 1,
//! ceil(9.5) = 10, ceil(18.05) = 19.
//!
//! - A (judged, ships_sunk [1,2], record 1): values ten 0, eight 1, two 2.
//!   5th 0, median 0, 95th 2. 1 is inside; held by 8 of 20, 8 x 20 = 160 >=
//!   20. AGREES.
//! - B (judged, same take, record 3 to 4): 3 to 4 does not overlap 0 to 2;
//!   held by 0. MISS, both tests.
//! - C (judged, hits_on ships [3,4] gun, record 1): values ten 0, ten 2. 5th
//!   0, median 0, 95th 2. 1 is inside; held by 0 of 20. MISS, the second test
//!   only.
//! - D (judged, hits_per_thousand_shots japanese gun, record 100): run 1
//!   fires "0", left out. Run 2: 35 x 1000 / 938 = 37.31, so 37. Run 3: 1000 /
//!   12.5 = 80. Run 4: 1000 / 7.5 = 133.33, so 133. Runs 5 to 20: 10 x 1000 /
//!   100 = 100. Kept 19, left out 1; 19 x 5 = 95 >= 20. Sorted 37, 80, sixteen
//!   100, 133: 5th 37, median 100, 95th 133. Held by 16 of 19, 320 >= 19.
//!   AGREES.
//! - E (not judged, claims_per_hundred japanese shot_down over air group 2's
//!   shot_down, record 200 to 300): run 1: 7 x 100 / 3 = 233.33, so 233. Run
//!   2: nothing really shot down, left out. Runs 3 to 20: 0 x 100 / 1 = 0.
//!   Kept 19, left out 1. 5th 0, median 0, 95th 233. Held by 1 of 19.
//!   REPORTED.
//! - F (judged, minutes_of_action, record 0): run 1: (2 + 3) x 5 = 25; run 2:
//!   1 x 5 = 5; run 3: (1 + 1) x 5 = 10; the other seventeen 0. Sorted:
//!   seventeen 0, then 5, 10, 25. The 5th is 0, the 25th 0, the median 0, the
//!   75th 0, the 95th (rank 19) is 10. Record 0 is inside both; held by 17 of
//!   20. Plain AGREES.
//! - N (judged, fights_by_day, record 0): run 2 counts 1 (the morning fight),
//!   run 3 counts 1 (the afternoon fight; the night one is not counted), the
//!   other eighteen 0, run 1's two fights being at night. 20 kept. Sorted:
//!   eighteen 0, two 1. The 5th percentile (rank 1) is 0, the 25th (rank 5)
//!   0, the median (rank 10) 0, the 75th (rank 15) 0, the 95th (rank 19) 1.
//!   Record 0 is inside both; held by 18 of 20. Plain AGREES.
//! - G (judged, ready air group 2 on day 2, record 4): 4 in eighteen runs;
//!   run 5: 4 + 2 = 6; run 6: 4 + 1 + 1 = 6, the second 1 from a group formed
//!   from a formed group. 5th 4, median 4, 95th 6. Held by 18 of 20. AGREES.
//! - H (not judged, aircraft_attacking side japanese on task force 2 with
//!   escorts, record 23): run 1: 12 + 5 = 17 attacking ships 3 and 4, plus the
//!   6 escorts of the same day, pulse and first group, 23; the morning's 4
//!   escorts match no attack and the 9 on ship 1 are on task force 1. The rest
//!   0. 5th 0, median 0, 95th 0. Held by 1 of 20. REPORTED.
//! - I (judged, claims_per_hundred japanese sunk over ships [3,4], record
//!   100): ship 3 is sunk only in runs 18, 19, 20: 2 x 100 / 1 = 200, 100,
//!   100. Kept 3, left out 17; 3 x 5 = 15 < 20. MISS, "too few runs kept".
//! - J and K on the same attacks: attack_days (side japanese, task force 2)
//!   is 1 in run 1 and 0 elsewhere; attack_of_at_least 10 aircraft (side
//!   japanese, task force 2) is 1 in run 1 (12 + 5 = 17 in one day and pulse,
//!   escorts not counted) and 0 elsewhere. Both not judged, record 1.
//! - The 25th and 75th percentiles (an agreement at the edge, step 2 spec):
//!   with 20 kept runs the 25th is rank ceil(25 x 20 / 100) = 5 and the 75th
//!   is rank 15; with 19 kept, ranks ceil(4.75) = 5 and ceil(14.25) = 15.
//!   A (ten 0, eight 1, two 2; record 1): rank 5 is 0, rank 15 is 1; 1 is
//!   within 0 to 1. Plain AGREES. D (37, 80, sixteen 100, 133; record 100):
//!   rank 5 is 100, rank 15 is 100. Plain AGREES. F (seventeen 0, then 5,
//!   10, 25; record 0): 0 to 0. Plain AGREES. G (eighteen 4, two 6; record 4): 4 to
//!   4. Plain AGREES.
//! - L (judged, ships_sunk [1,2], record 2): the 5th to 95th is 0 to 2 and 2
//!   of 20 hold it, 2 x 20 = 40 >= 20, so it agrees; the 25th to 75th is 0 to
//!   1 and 2 is outside it. AGREES, AT THE EDGE.
//! - M (judged, shots_in_hundreds japanese gun, record 1): the fixture's
//!   Japanese gun shots are "0" in run 1, "938" in run 2, "12.5" in run 3,
//!   "7.5" in run 4 and "100" in runs 5 to 20. In hundreds, rounded down: 0,
//!   9, 0, 0, and sixteen 1. No run is left out: 20 kept. Sorted: three 0,
//!   sixteen 1, one 9. The 5th percentile (rank 1) is 0, the 25th (rank 5) is
//!   1, the median (rank 10) is 1, the 75th (rank 15) is 1, the 95th (rank 19)
//!   is 1. Record 1 is inside both; 16 of 20 hold it. Plain AGREES.
//!   Unit cases for the calculation alone (`validate/take.rs`): "99.999999"
//!   gives 0; "100" gives 1; "199.5" gives 1; "2791" gives 27; "1867" gives
//!   18; "471" gives 4.
//! - Shots by ship (batch version 3). Ships 3 and 4 (Allied) fired gun shots
//!   "150" and "60.5" in run 1 and "99" and "0.999" in run 2, and none in the
//!   other eighteen runs. To stay consistent with them the Allied gun fire of
//!   the fixture is `shots` "210.5" in run 1 and "99.999" in run 2 (hits 0),
//!   and "0" in the others; no measure here reads the Allied side's fire.
//! - P (judged, shots_in_hundreds, ships [3, 4], weapon gun, record 0): the
//!   ships' shots are summed and floored once. Run 1: floor(210.5 / 100) = 2;
//!   run 2: floor(99.999 / 100) = 0; the other eighteen 0. No run is left
//!   out: 20 kept. Sorted: nineteen 0, one 2. The 5th, 25th, median, 75th and
//!   95th (rank 19) are all 0. Record 0 is inside both; held by 19 of 20.
//!   Plain AGREES.
//! - The fights of the fixture (batch version 4). Task force 1 is Japanese and
//!   task force 2 Allied. Run 1, first night fight, 2 rounds: `sighted_first`
//!   "japanese"; task force 1 first sighted in round 1, surprised 0 rounds;
//!   task force 2 first sighted in round 2, surprised 1. Run 1, second night
//!   fight, 3 rounds: "both"; each first sighted in round 1, surprised 0. Run
//!   2, the morning fight, 1 round: "allied"; task force 1 never sighted,
//!   surprised 1; task force 2 first sighted in round 1, surprised 0. Run 3,
//!   the night fight, 1 round: "allied"; task force 1 never sighted, surprised
//!   1; task force 2 first sighted in round 1, surprised 0. Run 3, the
//!   afternoon fight, 1 round: "japanese"; task force 1 first sighted in round
//!   1, surprised 0; task force 2 never sighted, surprised 1.
//! - S (judged, fights_sighted_first by ["allied", "both", "neither"], record
//!   0): run 1 counts 1 (its second night fight is "both"; its first is
//!   "japanese"). Run 2 counts 0: its fight is "allied" but in the morning.
//!   Run 3 counts 1 (the night fight is "allied"; the afternoon one is not
//!   counted). The other seventeen 0. 20 kept. Sorted: eighteen 0, two 1. The
//!   5th (rank 1) is 0, the 25th (rank 5) 0, the median 0, the 75th (rank 15)
//!   0, the 95th (rank 19) 1. Record 0 is inside both; held by 18 of 20. Plain
//!   AGREES.
//! - T (not judged, rounds_surprised task forces [2], record 1 to 2): run 1:
//!   1 + 0 = 1. Run 2: 0 (not surprised, and by day). Run 3: 0 by night; the 1
//!   of the afternoon fight is not counted. The rest 0. Sorted: nineteen 0,
//!   one 1. The 5th, median and 95th (rank 19) are 0. Held by 1 of 20.
//!   REPORTED.
//! - U (not judged, rounds_surprised task forces [1], record 0): run 1: 0. Run
//!   2: its 1 is by day, not counted: 0. Run 3: 1, by night. The rest 0.
//!   Sorted: nineteen 0, one 1. The 5th, median and 95th are 0. Held by 19 of
//!   20. REPORTED.
//! - The head line gains S among the judged and among the plain agreements:
//!   one more judged and one more agree than the line had before this change
//!   (11 and 7 became 12 and 8, if the line still read 11 and 7 when you
//!   start; if main changed it, add one to each of its two numbers and tell me
//!   what it read).
//! - So the fixture's head reads: judged 12 (A, B, C, D, F, G, I, L, M, N,
//!   P, S), agree 8 (A, D, F, G, M, N, P, S), at the edge 1 (L), miss 3 (B,
//!   C, I).
//! - The battle: 20 runs is fewer than 1,000, so no verdict on the battle;
//!   the report says so.
//!
//! The judging function alone, on 1,000 values, is tested in
//! `validate/judge.rs`: forty-nine 0, nine hundred and one 1, fifty 2. Rank 50
//! is 1 and rank 950 is 1, so the middle 90 percent is 1 to 1; the median,
//! rank 500, is 1. Record 1: agrees (901 hold it). Record 2: MISS on the first
//! test only. Record 0: MISS on both (49 x 20 = 980 < 1000). Record 0 to 1:
//! agrees (950 hold it), plain, as rank 250 and rank 750 are both 1. A second
//! set of 1,000: one hundred 0, eight hundred 1, one hundred 2. Rank 50 is 0
//! and rank 950 is 2; rank 250 and rank 750 are 1. Record 2: agrees (100 x 20
//! = 2000 >= 1000), at the edge (2 is outside 1 to 1). Record 1: plain agrees.
//! Record 0 to 1: plain agrees. Record 0: agrees at the edge. And the floor's
//! edge: with 20 runs, 4 kept is judged
//! and 3 kept is too few. A battle of 1,000 runs passes when every judged
//! measure agrees and does not when one misses; that is tested here on a
//! generated batch of 1,000 runs.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

use serde_json::{json, Value};

const ROOT: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../..");
const SCENARIO: &str = "data/scenarios/04a_night_action.toml";

fn binary(args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_sim-cli"))
        .args(args)
        .current_dir(ROOT)
        .output()
        .expect("the binary starts")
}

fn dir(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("flagplot-validate-{}-{name}", std::process::id()));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).expect("make the temp dir");
    dir
}

fn text(p: &Path) -> String {
    p.to_string_lossy().into_owned()
}

/// The `inputs` and scenario of a real batch of scenario 4a.
fn real_header(dir: &Path) -> Value {
    let out = dir.join("real.json");
    let result = binary(&[
        "batch",
        SCENARIO,
        "--runs",
        "1",
        "--label",
        "fixture",
        "--out",
        &text(&out),
    ]);
    assert!(
        result.status.success(),
        "{}",
        String::from_utf8_lossy(&result.stderr)
    );
    serde_json::from_slice(&fs::read(&out).expect("batch")).expect("json")
}

fn loss(group: u32, shot_down: u32) -> Value {
    json!({"air_group": group, "shot_down": shot_down, "destroyed_on_ground": 0,
        "damaged_destroyed_on_ground": 0, "lost_with_ship": 0, "worn_out": 0,
        "ditched_fuel": 0, "ditched_no_deck": 0, "pilots_lost": 0})
}

fn empty_run(seed: u64) -> Value {
    let none = json!({"shots": "0", "hits": 0});
    let fire = json!({"gun": none, "torpedo": none});
    json!({
        "seed": seed, "final_hash": "0", "sunk": [], "hits_on": [],
        "fire": {"japanese": fire, "allied": fire},
        "systems_out": [], "air_losses": [], "ready": [],
        "claims": {
            "japanese": {"sunk": 0, "hits": 0, "shot_down": 0, "on_ground": 0},
            "allied": {"sunk": 0, "hits": 0, "shot_down": 0, "on_ground": 0}},
        "attacks": [], "escorts": [], "launches": [], "broke_off": [], "fights": [],
        "supply": [], "formed_groups": [], "shots_by": []
    })
}

/// A surface fight of batch version 4 between task force 1 (Japanese) and
/// task force 2 (Allied): each force's first round of having sighted and its
/// rounds surprised.
fn fight(pulse: &str, rounds: u32, who: &str, forces: [(Option<u32>, u32); 2]) -> Value {
    let force = |id: u32, side: &str, (first, surprised): (Option<u32>, u32)| {
        json!({"task_force": id, "side": side, "first_sighted_round": first,
            "rounds_surprised": surprised})
    };
    json!({"day": 1, "pulse": pulse, "rounds": rounds, "sighted_first": who,
        "forces": [force(1, "japanese", forces[0]), force(2, "allied", forces[1])]})
}

fn attack(ship: u32, aircraft: u32) -> Value {
    json!({"day": 1, "pulse": "afternoon", "air_group": 1, "target_kind": "ship",
        "target": ship, "weapon": "bomb", "aircraft": aircraft})
}

/// The hand-made batch of the doc comment, with `runs` runs when asked (the
/// first 20 are the hand-made ones; more are empty).
fn fixture(dir: &Path, runs: u64) -> Value {
    let mut batch = real_header(dir);
    batch["scenario"]["days"] = json!(2);
    batch["label"] = json!("hand");
    batch["runs"] = json!(runs);
    batch["first_seed"] = json!(1);
    batch["ships"] = json!([
        {"id": 1, "task_force": 1}, {"id": 2, "task_force": 1},
        {"id": 3, "task_force": 2}, {"id": 4, "task_force": 2}]);
    batch["task_forces"] = json!([{"id": 1}, {"id": 2}]);
    batch["air_groups"] = json!([
        {"id": 1, "side": "japanese"}, {"id": 2, "side": "allied"},
        {"id": 5, "side": "japanese"}]);
    batch["bases"] = json!([{"id": 1}, {"id": 2}]);
    let mut per_run = Vec::new();
    for r in 1..=runs {
        let mut run = empty_run(r);
        let ready =
            |by: Vec<u32>, group: u32| json!({"air_group": group, "first_day": 1, "by_day": by});
        run["ready"] = json!([ready(vec![3, 4], 2)]);
        run["air_losses"] = json!([loss(
            2,
            if r == 2 {
                0
            } else if r == 1 {
                3
            } else {
                1
            }
        )]);
        if r == 2 {
            run["air_losses"] = json!([]);
        }
        match r {
            11..=17 => run["sunk"] = json!([1]),
            18 => run["sunk"] = json!([1, 3]),
            19 | 20 => run["sunk"] = json!([1, 2, 3]),
            _ => {}
        }
        if (11..=20).contains(&r) {
            run["hits_on"] = json!([{"ship": 3, "gun": 2, "torpedo": 0, "bomb": 0}]);
        }
        let (shots, hits) = match r {
            1 => ("0", 0),
            2 => ("938", 35),
            3 => ("12.5", 1),
            4 => ("7.5", 1),
            _ => ("100", 10),
        };
        run["fire"]["japanese"]["gun"] = json!({"shots": shots, "hits": hits});
        let by = |ship: u32, gun: &str| json!({"ship": ship, "gun": gun, "torpedo": "0"});
        match r {
            1 => {
                run["shots_by"] = json!([by(3, "150"), by(4, "60.5")]);
                run["fire"]["allied"]["gun"] = json!({"shots": "210.5", "hits": 0});
            }
            2 => {
                run["shots_by"] = json!([by(3, "99"), by(4, "0.999")]);
                run["fire"]["allied"]["gun"] = json!({"shots": "99.999", "hits": 0});
            }
            _ => {}
        }
        run["claims"]["japanese"]["shot_down"] = json!(match r {
            1 => 7,
            2 => 5,
            _ => 0,
        });
        run["claims"]["japanese"]["sunk"] = json!(match r {
            18 => 2,
            19 | 20 => 1,
            _ => 0,
        });
        if r == 1 {
            run["fights"] = json!([
                fight("night", 2, "japanese", [(Some(1), 0), (Some(2), 1)]),
                fight("night", 3, "both", [(Some(1), 0), (Some(1), 0)])
            ]);
            run["attacks"] = json!([attack(3, 12), attack(4, 5), attack(1, 9)]);
            run["escorts"] = json!([
                {"day": 1, "pulse": "afternoon", "air_group": 5, "with_air_group": 1, "aircraft": 6},
                {"day": 1, "pulse": "morning", "air_group": 5, "with_air_group": 1, "aircraft": 4}]);
        }
        if r == 2 {
            run["fights"] = json!([fight("morning", 1, "allied", [(None, 1), (Some(1), 0)])]);
        }
        if r == 3 {
            run["fights"] = json!([
                fight("night", 1, "allied", [(None, 1), (Some(1), 0)]),
                fight("afternoon", 1, "japanese", [(Some(1), 0), (None, 1)])
            ]);
        }
        if r == 5 {
            run["ready"] = json!([ready(vec![3, 4], 2), ready(vec![0, 2], 3)]);
            run["formed_groups"] =
                json!([{"id": 3, "name": "f", "side": "allied", "aircraft_type": "t", "from": 2}]);
        }
        if r == 6 {
            run["ready"] = json!([
                ready(vec![3, 4], 2),
                ready(vec![0, 1], 3),
                ready(vec![0, 1], 4)
            ]);
            run["formed_groups"] = json!([
                {"id": 3, "name": "f", "side": "allied", "aircraft_type": "t", "from": 2},
                {"id": 4, "name": "g", "side": "allied", "aircraft_type": "t", "from": 3}]);
        }
        per_run.push(run);
    }
    batch["per_run"] = json!(per_run);
    batch["results"] = Value::Null;
    batch
}

const MEASURES: &str = r#"
battle = "hand-made"
scenario = "data/scenarios/04a_night_action.toml"
facts = "data/oob/engagement_outcomes.csv"

[[measure]]
id = "a"
counts = "A"
record_low = 1
record_high = 1
judged = true
take = "ships_sunk"
ships = [1, 2]
rows = [1]
reason = "r"

[[measure]]
id = "b"
counts = "B"
record_low = 3
record_high = 4
judged = true
take = "ships_sunk"
ships = [1, 2]
rows = [1]
reason = "r"

[[measure]]
id = "c"
counts = "C"
record_low = 1
record_high = 1
judged = true
take = "hits_on"
ships = [3, 4]
weapons = ["gun"]
rows = [1]
reason = "r"

[[measure]]
id = "d"
counts = "D"
record_low = 100
record_high = 100
judged = true
take = "hits_per_thousand_shots"
side = "japanese"
weapon = "gun"
rows = [1]
reason = "r"

[[measure]]
id = "e"
counts = "E"
record_low = 200
record_high = 300
judged = false
take = "claims_per_hundred"
side = "japanese"
claim = "shot_down"
air_groups = [2]
causes = ["shot_down"]
rows = [1]
reason = "r"

[[measure]]
id = "f"
counts = "F"
record_low = 0
record_high = 0
judged = true
take = "minutes_of_action"
rows = [1]
reason = "r"

[[measure]]
id = "g"
counts = "G"
record_low = 4
record_high = 4
judged = true
take = "ready"
air_groups = [2]
day = 2
rows = [1]
reason = "r"

[[measure]]
id = "h"
counts = "H"
record_low = 23
record_high = 23
judged = false
take = "aircraft_attacking"
side = "japanese"
target_task_forces = [2]
escorts = true
rows = [1]
reason = "r"

[[measure]]
id = "i"
counts = "I"
record_low = 100
record_high = 100
judged = true
take = "claims_per_hundred"
side = "japanese"
claim = "sunk"
ships = [3, 4]
rows = [1]
reason = "r"

[[measure]]
id = "j"
counts = "J"
record_low = 1
record_high = 1
judged = false
take = "attack_days"
side = "japanese"
target_task_forces = [2]
rows = [1]
reason = "r"

[[measure]]
id = "k"
counts = "K"
record_low = 1
record_high = 1
judged = false
take = "attack_of_at_least"
side = "japanese"
target_task_forces = [2]
aircraft = 10
rows = [1]
reason = "r"

[[measure]]
id = "l"
counts = "L"
record_low = 2
record_high = 2
judged = true
take = "ships_sunk"
ships = [1, 2]
rows = [1]
reason = "r"

[[measure]]
id = "m"
counts = "M"
record_low = 1
record_high = 1
judged = true
take = "shots_in_hundreds"
side = "japanese"
weapon = "gun"
rows = [1]
reason = "r"

[[measure]]
id = "n"
counts = "N"
record_low = 0
record_high = 0
judged = true
take = "fights_by_day"
rows = [1]
reason = "r"

[[measure]]
id = "p"
counts = "P"
record_low = 0
record_high = 0
judged = true
take = "shots_in_hundreds"
ships = [3, 4]
weapon = "gun"
rows = [1]
reason = "r"

[[measure]]
id = "s"
counts = "S"
record_low = 0
record_high = 0
judged = true
take = "fights_sighted_first"
by = ["allied", "both", "neither"]
rows = [1]
reason = "r"

[[measure]]
id = "t"
counts = "T"
record_low = 1
record_high = 2
judged = false
take = "rounds_surprised"
task_forces = [2]
rows = [1]
reason = "r"

[[measure]]
id = "u"
counts = "U"
record_low = 0
record_high = 0
judged = false
take = "rounds_surprised"
task_forces = [1]
facts = "y"
rows = [1]
reason = "r"
"#;

struct Setup {
    dir: PathBuf,
    batch: PathBuf,
    measures: PathBuf,
}

fn setup(name: &str, batch: &Value, measures: &str) -> Setup {
    let dir = dir(name);
    let batch_path = dir.join("batch.json");
    fs::write(&batch_path, serde_json::to_vec(batch).expect("json")).expect("write");
    let measures_path = dir.join("measures.toml");
    fs::write(&measures_path, measures).expect("write");
    Setup {
        dir,
        batch: batch_path,
        measures: measures_path,
    }
}

fn validate(s: &Setup, out: &str) -> Output {
    binary(&[
        "validate",
        &text(&s.batch),
        &text(&s.measures),
        "--out",
        &text(&s.dir.join(out)),
    ])
}

fn stderr(o: &Output) -> String {
    String::from_utf8_lossy(&o.stderr).into_owned()
}

/// The cells of a measure's row in the report's table.
fn cells(report: &str, id: &str) -> Vec<String> {
    let line = report
        .lines()
        .find(|l| l.starts_with(&format!("| {id} |")))
        .unwrap_or_else(|| panic!("no row for {id}"));
    line.trim_matches('|')
        .split('|')
        .map(|c| c.trim().to_string())
        .collect()
}

fn expect(
    report: &str,
    id: &str,
    band: &str,
    median: &str,
    holding: &str,
    left: &str,
    verdict: &str,
) {
    let c = cells(report, id);
    assert_eq!(c[2], band, "{id} band");
    assert_eq!(c[3], median, "{id} median");
    assert_eq!(c[4], holding, "{id} holding");
    assert_eq!(c[5], left, "{id} left out");
    assert!(
        c[6].contains(verdict),
        "{id} verdict {:?} lacks {verdict:?}",
        c[6]
    );
    // A plain agreement is not one at the edge.
    if verdict == "agrees" {
        assert_eq!(c[6], "agrees", "{id} is a plain agreement");
    }
}

#[test]
fn the_hand_calculation_of_every_measure() {
    let s = setup("hand", &fixture(&dir("hand-h"), 20), MEASURES);
    let result = validate(&s, "report.md");
    assert!(result.status.success(), "{}", stderr(&result));
    let report = fs::read_to_string(s.dir.join("report.md")).expect("report");
    expect(&report, "a", "0 to 2", "0", "8 of 20", "0", "agrees");
    let b = cells(&report, "b");
    assert!(b[6].contains("outside the middle 90 percent; fewer than 1 run in 20"));
    expect(&report, "b", "0 to 2", "0", "0 of 20", "0", "MISS");
    let c = cells(&report, "c");
    assert!(c[6].contains("fewer than 1 run in 20") && !c[6].contains("outside"));
    expect(&report, "c", "0 to 2", "0", "0 of 20", "0", "MISS");
    expect(&report, "d", "37 to 133", "100", "16 of 19", "1", "agrees");
    expect(&report, "e", "0 to 233", "0", "1 of 19", "1", "reported");
    expect(&report, "f", "0 to 10", "0", "17 of 20", "0", "agrees");
    expect(&report, "n", "0 to 1", "0", "18 of 20", "0", "agrees");
    expect(&report, "g", "4 to 6", "4", "18 of 20", "0", "agrees");
    expect(&report, "h", "0 to 0", "0", "1 of 20", "0", "reported");
    expect(
        &report,
        "i",
        "100 to 200",
        "100",
        "2 of 3",
        "17",
        "too few runs kept",
    );
    expect(&report, "j", "0 to 0", "0", "1 of 20", "0", "reported");
    expect(&report, "k", "0 to 0", "0", "1 of 20", "0", "reported");
    expect(&report, "m", "0 to 1", "1", "16 of 20", "0", "agrees");
    expect(&report, "p", "0 to 0", "0", "19 of 20", "0", "agrees");
    expect(&report, "s", "0 to 1", "0", "18 of 20", "0", "agrees");
    expect(&report, "t", "0 to 0", "0", "1 of 20", "0", "reported");
    expect(&report, "u", "0 to 0", "0", "19 of 20", "0", "reported");
    expect(
        &report,
        "l",
        "0 to 2",
        "0",
        "2 of 20",
        "0",
        "agrees, at the edge",
    );
    // Twelve judged: a, d, f, g, m, n, p, s agree; l agrees at the edge; b, c,
    // i miss.
    assert!(report.contains("Judged measures: 8 agree, 1 agree at the edge, 3 miss, of 12."));
    assert!(report.contains("## At the edge\n\n### l\n"));
    assert!(report.contains("- Verdict: agrees, at the edge"));
    assert!(report.contains("least 0, 5th 0, 25th 0, median 0, 75th 1, 95th 2, greatest 2"));
    assert!(report.contains(
        "**No verdict on the battle.** This batch has 20 runs and the spec asks for 1000."
    ));
    // A miss carries its counts text, rows and spread.
    assert!(report.contains("### c\n"));
    assert!(report.contains("- Counts: C"));
    assert!(report.contains("least 0, 5th 0, 25th 0, median 0, 75th 2, 95th 2, greatest 2"));
    assert!(report.contains("- Each value and its runs: 0: 10, 2: 10"));
    // The two standing notes.
    assert!(report.contains("Hits are counted as rolled"));
    assert!(report.contains("Escorts are matched to a raid's first strike group"));
    assert!(!report.contains("not run"));
}

#[test]
fn the_same_inputs_give_the_same_bytes_and_a_report_is_not_written_over() {
    let s = setup("bytes", &fixture(&dir("bytes-h"), 20), MEASURES);
    assert!(validate(&s, "one.md").status.success());
    assert!(validate(&s, "two.md").status.success());
    let one = fs::read(s.dir.join("one.md")).expect("one");
    assert_eq!(one, fs::read(s.dir.join("two.md")).expect("two"));
    // Identical bytes: a no-op that succeeds.
    let again = validate(&s, "one.md");
    assert!(again.status.success());
    assert!(String::from_utf8_lossy(&again.stdout).starts_with("unchanged"));
    assert_eq!(one, fs::read(s.dir.join("one.md")).expect("one"));
    // Different bytes: refused, and the file is as it was.
    fs::write(s.dir.join("one.md"), "something else").expect("write");
    let refused = validate(&s, "one.md");
    assert!(!refused.status.success());
    assert!(stderr(&refused).contains("different contents"));
    assert_eq!(
        fs::read_to_string(s.dir.join("one.md")).expect("read"),
        "something else"
    );
}

#[test]
fn a_battle_of_a_thousand_runs_passes_when_every_judged_measure_agrees() {
    let dir = dir("thousand");
    let measures = r#"
battle = "hand-made"
scenario = "data/scenarios/04a_night_action.toml"
facts = "x"

[[measure]]
id = "a"
counts = "A"
record_low = 1
record_high = 1
judged = true
take = "ships_sunk"
ships = [1, 2]
rows = [1]
reason = "r"
"#;
    let mut batch = fixture(&dir, 1000);
    // Runs 1 to 1000: ships 1 and 2 sunk in 0, 1 or 2 as 49, 901, 50 runs.
    for (i, run) in batch["per_run"]
        .as_array_mut()
        .expect("runs")
        .iter_mut()
        .enumerate()
    {
        run["sunk"] = match i {
            0..=48 => json!([]),
            49..=949 => json!([1]),
            _ => json!([1, 2]),
        };
    }
    let s = setup("thousand-a", &batch, measures);
    assert!(validate(&s, "pass.md").status.success());
    let pass = fs::read_to_string(s.dir.join("pass.md")).expect("report");
    assert!(
        pass.contains("**The battle passes:** every judged measure agrees."),
        "{pass}"
    );
    expect(&pass, "a", "1 to 1", "1", "901 of 1000", "0", "agrees");
    // A record of 2 misses on the first test only; the battle does not pass.
    let missing = measures.replace(
        "record_low = 1\nrecord_high = 1",
        "record_low = 2\nrecord_high = 2",
    );
    fs::write(&s.measures, missing).expect("write");
    assert!(validate(&s, "fail.md").status.success());
    let fail = fs::read_to_string(s.dir.join("fail.md")).expect("report");
    assert!(fail.contains("**The battle does not pass:** 1 of 1 judged measures miss."));
    let c = cells(&fail, "a");
    assert!(c[6].contains("outside the middle 90 percent") && !c[6].contains("fewer than 1 run"));
}

#[test]
fn what_validate_refuses() {
    let h = dir("refuse-h");
    let good = fixture(&h, 20);
    let refuse = |name: &str, batch: &Value, measures: &str, want: &str| {
        let s = setup(name, batch, measures);
        let result = validate(&s, "r.md");
        assert!(!result.status.success(), "{name} should be refused");
        assert!(
            stderr(&result).contains(want),
            "{name}: {}",
            stderr(&result)
        );
        assert!(!s.dir.join("r.md").exists(), "{name} wrote a report");
    };
    let mut b = good.clone();
    b.as_object_mut().expect("object").remove("label");
    refuse("no-label", &b, MEASURES, "no label");
    let mut b = good.clone();
    b["label"] = json!("a/b");
    refuse("bad-label", &b, MEASURES, "not letters, digits");
    let mut b = good.clone();
    b["label"] = json!("");
    refuse("empty-label", &b, MEASURES, "not letters, digits");
    let mut b = good.clone();
    b["version"] = json!(1);
    refuse("version", &b, MEASURES, "version 1");
    // A version 3 batch, the shape before who sighted first, is refused for
    // its version and told to be made again.
    let mut b = good.clone();
    b["version"] = json!(3);
    for fight in b["per_run"][0]["fights"].as_array_mut().expect("fights") {
        let fight = fight.as_object_mut().expect("fight");
        fight.remove("sighted_first");
        fight.remove("forces");
    }
    refuse("version-3", &b, MEASURES, "version 3; validate reads");
    let mut b = good.clone();
    b["version"] = json!(3);
    refuse("version-3-again", &b, MEASURES, "make it again");
    let mut b = good.clone();
    b["version"] = json!(2);
    refuse("version-2", &b, MEASURES, "make it again");
    let mut b = good.clone();
    b["inputs"]["state_version"] = json!("flagplot-state-v0");
    refuse("state", &b, MEASURES, "state hash layout");
    let mut b = good.clone();
    b["inputs"]["scenario_hash"] = json!("0000000000000000");
    refuse("scenario-hash", &b, MEASURES, "scenario file");
    let mut b = good.clone();
    b["inputs"]["data"][1]["hash"] = json!("0000000000000000");
    refuse("data-hash", &b, MEASURES, "has changed since the batch");
    let mut b = good.clone();
    b["inputs"]["data"].as_array_mut().expect("data").pop();
    refuse("data-names", &b, MEASURES, "different data files");
    let other = MEASURES.replace("04a_night_action", "01_transit");
    refuse(
        "other-scenario",
        &good,
        &other,
        "is for data/scenarios/01_transit.toml",
    );
    let mut b = good.clone();
    b["runs"] = json!(21);
    refuse("runs", &b, MEASURES, "says 21 runs");
    let mut b = good.clone();
    b["per_run"][3]["seed"] = json!(99);
    refuse("seed", &b, MEASURES, "has seed 99");
    refuse(
        "unknown-ship",
        &good,
        &MEASURES.replace(
            "ships = [1, 2]\nrows = [1]\nreason = \"r\"\n\n[[measure]]\nid = \"b\"",
            "ships = [1, 77]\nrows = [1]\nreason = \"r\"\n\n[[measure]]\nid = \"b\"",
        ),
        "ship 77",
    );
    refuse(
        "unknown-weapon",
        &good,
        &MEASURES.replace(
            "take = \"shots_in_hundreds\"\nside = \"japanese\"\nweapon = \"gun\"",
            "take = \"shots_in_hundreds\"\nside = \"japanese\"\nweapon = \"cannon\"",
        ),
        "weapon \"cannon\"",
    );
    refuse(
        "unknown-side",
        &good,
        &MEASURES.replace(
            "take = \"shots_in_hundreds\"\nside = \"japanese\"",
            "take = \"shots_in_hundreds\"\nside = \"martian\"",
        ),
        "side \"martian\"",
    );
    refuse(
        "no-key",
        &good,
        &MEASURES.replace("day = 2\n", ""),
        "needs the key day",
    );
    refuse(
        "bad-take",
        &good,
        &MEASURES.replace("\"minutes_of_action\"", "\"nonsense\""),
        "not known",
    );
    refuse(
        "day-range",
        &good,
        &MEASURES.replace("day = 2", "day = 3"),
        "day 3",
    );
    refuse(
        "key-with-fights-by-day",
        &good,
        &MEASURES.replace(
            "take = \"fights_by_day\"\n",
            "take = \"fights_by_day\"\nside = \"japanese\"\n",
        ),
        "fights_by_day takes no keys",
    );
    // The takes of version 4.
    let sighted = |by: &str| {
        MEASURES.replace(
            "by = [\"allied\", \"both\", \"neither\"]",
            &format!("by = {by}"),
        )
    };
    refuse("by-empty", &good, &sighted("[]"), "measure s: by is empty");
    refuse(
        "by-enemy",
        &good,
        &sighted("[\"enemy\"]"),
        "measure s: by names \"enemy\"",
    );
    refuse(
        "by-twice",
        &good,
        &sighted("[\"both\", \"both\"]"),
        "measure s: by names \"both\" twice",
    );
    refuse(
        "surprised-no-forces",
        &good,
        &MEASURES.replace("\ntask_forces = [2]\n", "\n"),
        "needs the key task_forces",
    );
    refuse(
        "surprised-force-9",
        &good,
        &MEASURES.replace("\ntask_forces = [2]", "\ntask_forces = [9]"),
        "task force 9, which the batch does not have",
    );
    refuse(
        "surprised-empty",
        &good,
        &MEASURES.replace("\ntask_forces = [2]", "\ntask_forces = []"),
        "task_forces is empty",
    );
    refuse(
        "sighted-with-other-key",
        &good,
        &MEASURES.replace(
            "take = \"fights_sighted_first\"\n",
            "take = \"fights_sighted_first\"\nside = \"japanese\"\n",
        ),
        "does not take the key side",
    );
    refuse(
        "fights-by-day-with-by",
        &good,
        &MEASURES.replace(
            "take = \"fights_by_day\"\n",
            "take = \"fights_by_day\"\nby = [\"both\"]\n",
        ),
        "fights_by_day takes no keys",
    );
    refuse(
        "no-take",
        &good,
        &MEASURES.replace("take = \"minutes_of_action\"\n", ""),
        "needs the key take",
    );
    refuse(
        "backwards",
        &good,
        &MEASURES.replace(
            "record_low = 3\nrecord_high = 4",
            "record_low = 5\nrecord_high = 4",
        ),
        "record_low above",
    );
    refuse(
        "duplicate",
        &good,
        &MEASURES.replace("id = \"b\"", "id = \"a\""),
        "used twice",
    );
    refuse(
        "unknown-key",
        &good,
        &MEASURES.replace("counts = \"A\"", "counts = \"A\"\nbogus = 1"),
        "bogus",
    );
    // A batch with a key missing (a version 1 shape) is refused cleanly.
    let mut b = good.clone();
    b["per_run"][0]
        .as_object_mut()
        .expect("run")
        .remove("fights");
    refuse("old-shape", &b, MEASURES, "not a batch file of version 5");
    // `ships` and `side` together are an error, and so is neither.
    refuse(
        "ships-and-side",
        &good,
        &MEASURES.replace(
            "ships = [3, 4]\nweapon = \"gun\"",
            "ships = [3, 4]\nside = \"japanese\"\nweapon = \"gun\"",
        ),
        "ships or side, not both",
    );
    refuse(
        "neither-ships-nor-side",
        &good,
        &MEASURES.replace("ships = [3, 4]\nweapon = \"gun\"", "weapon = \"gun\""),
        "needs the key side",
    );
    refuse(
        "unknown-ship-in-shots",
        &good,
        &MEASURES.replace(
            "ships = [3, 4]\nweapon = \"gun\"",
            "ships = [3, 77]\nweapon = \"gun\"",
        ),
        "ship 77",
    );
}

#[test]
fn a_measures_file_with_no_scenario_is_reported_not_run() {
    let d = dir("notbuilt");
    let out = d.join("cape.md");
    let result = binary(&[
        "validate",
        "data/validation/cape_esperance.toml",
        "--out",
        &text(&out),
    ]);
    assert!(result.status.success(), "{}", stderr(&result));
    let report = fs::read_to_string(&out).expect("report");
    let file = fs::read_to_string(Path::new(ROOT).join("data/validation/cape_esperance.toml"))
        .expect("file");
    let measures = file.matches("[[measure]]").count();
    assert_eq!(
        report.matches("not run: scenario not built").count(),
        measures
    );
    assert!(report.contains("Not run: the scenario is not built"));
    // With a batch, a file with no scenario is an error.
    let s = setup("nb", &fixture(&d, 20), &file);
    let with_batch = validate(&s, "x.md");
    assert!(!with_batch.status.success());
    // A file with a scenario and no batch is an error.
    let no_batch = binary(&[
        "validate",
        "data/validation/savo_island.toml",
        "--out",
        &text(&d.join("y.md")),
    ]);
    assert!(!no_batch.status.success());
    assert!(stderr(&no_batch).contains("names a scenario"));
}

/// Real batches of 20 runs of scenarios 4, 5 and 6, judged by the measures
/// files on main: every measure id appears, and every `take` and key is known.
#[test]
fn real_batches_of_the_three_battles_are_judged() {
    let d = dir("real");
    for (scenario, measures, battle) in [
        (
            "data/scenarios/04_savo_island.toml",
            "data/validation/savo_island.toml",
            "savo-island",
        ),
        (
            "data/scenarios/05_eastern_solomons.toml",
            "data/validation/eastern_solomons.toml",
            "eastern-solomons",
        ),
        (
            "data/scenarios/06_rabaul_guadalcanal.toml",
            "data/validation/rabaul_guadalcanal.toml",
            "rabaul-guadalcanal-air",
        ),
    ] {
        let batch = d.join(format!("{battle}.json"));
        let made = binary(&[
            "batch",
            scenario,
            "--runs",
            "20",
            "--label",
            "test",
            "--out",
            &text(&batch),
        ]);
        assert!(made.status.success(), "{}", stderr(&made));
        let out = d.join(format!("{battle}.md"));
        let result = binary(&["validate", &text(&batch), measures, "--out", &text(&out)]);
        assert!(result.status.success(), "{battle}: {}", stderr(&result));
        let report = fs::read_to_string(&out).expect("report");
        let file = fs::read_to_string(Path::new(ROOT).join(measures)).expect("measures");
        if battle == "savo-island" {
            for id in [
                "japanese_main_battery_shells_fired_in_hundreds",
                "allied_main_battery_shells_fired_in_hundreds",
                "surface_fights_by_day",
            ] {
                assert!(report.contains(&format!("| {id} |")), "{id} missing");
            }
        }
        for line in file.lines().filter(|l| l.starts_with("id = ")) {
            let id = line.trim_start_matches("id = ").trim_matches('"');
            assert!(
                report.contains(&format!("| {id} |")),
                "{battle}: {id} missing"
            );
        }
        assert!(report.contains("**No verdict on the battle.**"));
    }
}

#[test]
fn nothing_that_plays_the_game_depends_on_validate() {
    let agent =
        fs::read_to_string(Path::new(ROOT).join("crates/sim-agent/Cargo.toml")).expect("manifest");
    assert!(
        !agent.contains("sim-cli"),
        "sim-agent must not depend on sim-cli, which holds validate"
    );
    assert!(!agent.contains("validate"));
    let core =
        fs::read_to_string(Path::new(ROOT).join("crates/sim-core/Cargo.toml")).expect("manifest");
    assert!(!core.contains("sim-cli"));
}

// ---- the kinds `shots` and `ships_sunk_or_system_out` ----

/// The architect's hand calculation for the two kinds. The batch is the
/// fixture's header with its 20 runs all made alike (so each measure's band
/// and median are its one value and 20 of 20 hold it):
///
/// - `sunk` is ships 1 and 3. `systems_out` is ship 2 with propulsion (afloat),
///   ship 3 with propulsion (sunk too), and ship 4 with fire control (afloat).
/// - Allied torpedo fire: `shots` "4.75". Japanese torpedo fire: "9.5".
///   `shots_by`: ship 3 torpedo "1.5", ship 4 torpedo "2.5", ship 1 torpedo
///   "0.75".
///
/// `shots` (whole shots, floored once):
/// - side allied, torpedo: floor(4.75) = 4. Side japanese, torpedo: 9.
/// - ships [3, 4], torpedo: 1.5 + 2.5 = 4.0, floored once: 4 (flooring each
///   would give 3). Ships [1, 3], torpedo: 0.75 + 1.5 = 2.25: 2.
/// - `side` with `ships`, or neither: refused.
///
/// `shots` with `unit` (decision 0084). Allied gun fire: `shots` "57.5";
/// `shots_by` gun: ship 3 "33.5", ship 4 "16.9", ship 1 "0".
/// - side allied, gun, unit tens: floor(57.5 / 10) = 5.
/// - ships [3, 4], gun, unit tens: 33.5 + 16.9 = 50.4, floored once after
///   the division: 5 (flooring each ship's tens would give 3 + 1 = 4).
/// - ships [3, 4], gun, unit shells (the default named): floor(50.4) = 50.
/// - ships [3], gun, unit tens: floor(3.35) = 3. Ships [1], gun, tens: 0.
/// - unit "hundreds", and unit on any take but `shots`: refused.
///
/// Every measure here has the record 0 to 99, so each is held in all 20 runs.
///
/// `ships_sunk_or_system_out`, system propulsion:
/// - ships [1]: ship 1 is sunk, no system out: 1.
/// - ships [2]: afloat with propulsion out: 1.
/// - ships [3]: sunk and with propulsion out too, counted once: 1.
/// - ships [4]: afloat with fire control out, not propulsion: 0.
/// - ships [1, 2, 3, 4]: 1 + 1 + 1 + 0 = 3.
#[test]
fn the_kinds_shots_and_ships_sunk_or_system_out_are_the_hand_calculated_ones() {
    let mut batch = fixture(&dir("two-kinds-h"), 20);
    for run in batch["per_run"].as_array_mut().expect("runs") {
        run["sunk"] = json!([1, 3]);
        run["systems_out"] = json!([
            {"ship": 2, "system": "propulsion"},
            {"ship": 3, "system": "propulsion"},
            {"ship": 4, "system": "fire_control"}]);
        run["fire"]["allied"]["torpedo"] = json!({"shots": "4.75", "hits": 0});
        run["fire"]["japanese"]["torpedo"] = json!({"shots": "9.5", "hits": 0});
        run["fire"]["allied"]["gun"] = json!({"shots": "57.5", "hits": 0});
        let by = |ship: u32, gun: &str, torpedo: &str| json!({"ship": ship, "gun": gun, "torpedo": torpedo});
        run["shots_by"] = json!([
            by(1, "0", "0.75"),
            by(3, "33.5", "1.5"),
            by(4, "16.9", "2.5")
        ]);
    }
    let measure = |id: &str, body: &str| {
        format!(
            "\n[[measure]]\nid = \"{id}\"\ncounts = \"X\"\nrecord_low = 0\nrecord_high = 99\n\
             judged = false\n{body}\nrows = [1]\nreason = \"r\"\n"
        )
    };
    let head = "battle = \"hand-made\"\nscenario = \"data/scenarios/04a_night_action.toml\"\nfacts = \"x\"\n";
    let cases = [
        (
            "s1",
            "take = \"shots\"\nside = \"allied\"\nweapon = \"torpedo\"",
            "4",
        ),
        (
            "s2",
            "take = \"shots\"\nside = \"japanese\"\nweapon = \"torpedo\"",
            "9",
        ),
        (
            "s3",
            "take = \"shots\"\nships = [3, 4]\nweapon = \"torpedo\"",
            "4",
        ),
        (
            "s4",
            "take = \"shots\"\nships = [1, 3]\nweapon = \"torpedo\"",
            "2",
        ),
        (
            "t1",
            "take = \"shots\"\nside = \"allied\"\nweapon = \"gun\"\nunit = \"tens\"",
            "5",
        ),
        (
            "t2",
            "take = \"shots\"\nships = [3, 4]\nweapon = \"gun\"\nunit = \"tens\"",
            "5",
        ),
        (
            "t3",
            "take = \"shots\"\nships = [3, 4]\nweapon = \"gun\"\nunit = \"shells\"",
            "50",
        ),
        (
            "t4",
            "take = \"shots\"\nships = [3]\nweapon = \"gun\"\nunit = \"tens\"",
            "3",
        ),
        (
            "t5",
            "take = \"shots\"\nships = [1]\nweapon = \"gun\"\nunit = \"tens\"",
            "0",
        ),
        (
            "u1",
            "take = \"ships_sunk_or_system_out\"\nships = [1]\nsystem = \"propulsion\"",
            "1",
        ),
        (
            "u2",
            "take = \"ships_sunk_or_system_out\"\nships = [2]\nsystem = \"propulsion\"",
            "1",
        ),
        (
            "u3",
            "take = \"ships_sunk_or_system_out\"\nships = [3]\nsystem = \"propulsion\"",
            "1",
        ),
        (
            "u4",
            "take = \"ships_sunk_or_system_out\"\nships = [4]\nsystem = \"propulsion\"",
            "0",
        ),
        (
            "u5",
            "take = \"ships_sunk_or_system_out\"\nships = [1, 2, 3, 4]\nsystem = \"propulsion\"",
            "3",
        ),
    ];
    let mut measures = head.to_string();
    for (id, body, _) in &cases {
        measures.push_str(&measure(id, body));
    }
    let s = setup("two-kinds", &batch, &measures);
    let result = validate(&s, "report.md");
    assert!(result.status.success(), "{}", stderr(&result));
    let report = fs::read_to_string(s.dir.join("report.md")).expect("report");
    for (id, _, value) in &cases {
        let c = cells(&report, id);
        assert_eq!(c[2], format!("{value} to {value}"), "{id} band");
        assert_eq!(c[3], *value, "{id} median");
        assert_eq!(c[4], "20 of 20", "{id} holding");
    }
    // Both `side` and `ships`, and neither, are refused.
    for (name, body) in [
        (
            "both",
            "take = \"shots\"\nside = \"allied\"\nships = [3]\nweapon = \"torpedo\"",
        ),
        ("neither", "take = \"shots\"\nweapon = \"torpedo\""),
        (
            "unit-hundreds",
            "take = \"shots\"\nside = \"allied\"\nweapon = \"gun\"\nunit = \"hundreds\"",
        ),
        (
            "unit-on-hundreds",
            "take = \"shots_in_hundreds\"\nside = \"allied\"\nweapon = \"gun\"\nunit = \"tens\"",
        ),
    ] {
        let s = setup(
            &format!("two-kinds-{name}"),
            &batch,
            &format!("{head}{}", measure("x", body)),
        );
        let result = validate(&s, "r.md");
        assert!(!result.status.success(), "{name} should be refused");
    }
}

/// `aircraft_taking_part` (batch version 5, decision 0081).
///
/// # The architect's hand calculation
///
/// A batch of 3 runs, seeds 1 to 3, made by hand on the header of scenario
/// 4a (so its inputs match the disk). Air groups 11, 12, 14 and 15 are
/// Japanese; task forces 1, 2, 3 and 4. A measure that names task forces 1
/// and 2 counts the aircraft launched, as `launches` records them, in ship
/// strikes that did not find nothing and whose raid met task force 1 or 2,
/// and with `escorts = true` adds the aircraft of each `escorts` entry of the
/// same day and pulse whose `with_air_group` is the first strike group of a
/// counted launch, each entry once. All launches are in day 1, afternoon.
///
/// - Run 1: group 12 launches 18, met task force 1; group 15 launches 9, met
///   task force 1 (two raids, from different ships). Escorts: group 11 sent 4
///   with group 12 and group 14 sent 6 with group 15. A base strike by group
///   12 of 7 aircraft is not counted (it is not a ship strike). With escorts
///   18 + 9 + 4 + 6 = 37; without 18 + 9 = 27.
/// - Run 2: group 12 launches 18 and finds nothing (not counted); group 15
///   launches 9, met task force 2. Escorts: group 11 sent 4 with group 12
///   (not counted: its raid found nothing), group 14 sent 6 with group 15.
///   With escorts 9 + 6 = 15; without 9.
/// - Run 3: group 12 launches 18, met task force 3, which is not in the list:
///   not counted. Escorts: group 11 sent 4 with group 12, not counted. 0 and
///   0.
///
/// Measure X (judged, side japanese, task forces [1, 2], escorts true,
/// record 37): values 37, 15, 0. With 3 runs the ranks are 5th: ceil(0.15) =
/// 1, 25th: ceil(0.75) = 1, median: ceil(1.5) = 2, 75th: ceil(2.25) = 3, 95th:
/// ceil(2.85) = 3. Sorted 0, 15, 37: 5th 0, median 15, 95th 37; 25th 0, 75th
/// 37. Record 37 is inside 0 to 37 and inside 0 to 37; held by 1 of 3, and 1
/// x 20 = 20 >= 3. Plain AGREES.
///
/// Measure Y (judged, the same without escorts, record 27): values 27, 9, 0.
/// Sorted 0, 9, 27: band 0 to 27, median 9, 25th 0, 75th 27. Record 27 is
/// held by 1 of 3. Plain AGREES.
///
/// Measure Z (judged, air_groups [15], task forces [1, 2], escorts true,
/// record 15): run 1: 9 + 6 = 15; run 2: 9 + 6 = 15; run 3: 0. Sorted 0, 15,
/// 15: band 0 to 15, median 15, 25th 0, 75th 15. Record 15 held by 2 of 3.
/// Plain AGREES.
///
/// The head reads: 3 agree, 0 at the edge, 0 miss, of 3 judged.
#[test]
fn aircraft_taking_part_counts_the_wave_that_met_the_named_forces() {
    let launch = |group: u32, aircraft: u32, nothing: bool, force: Option<u32>| {
        json!({"day": 1, "pulse": "afternoon", "air_group": group, "aircraft": aircraft,
            "hex": [5, 5], "target_kind": "ship", "found_nothing": nothing,
            "task_force": force, "base": null})
    };
    let escort = |group: u32, aircraft: u32, with: u32| {
        json!({"day": 1, "pulse": "afternoon", "air_group": group,
            "with_air_group": with, "aircraft": aircraft})
    };
    let mut batch = real_header(&dir("part-h"));
    batch["scenario"]["days"] = json!(2);
    batch["label"] = json!("hand");
    batch["runs"] = json!(3);
    batch["first_seed"] = json!(1);
    batch["ships"] = json!([]);
    batch["task_forces"] = json!([{"id": 1}, {"id": 2}, {"id": 3}, {"id": 4}]);
    batch["air_groups"] = json!([
        {"id": 11, "side": "japanese"}, {"id": 12, "side": "japanese"},
        {"id": 14, "side": "japanese"}, {"id": 15, "side": "japanese"}]);
    batch["bases"] = json!([{"id": 1}]);
    let mut runs = vec![empty_run(1), empty_run(2), empty_run(3)];
    runs[0]["launches"] = json!([
        launch(12, 18, false, Some(1)),
        launch(15, 9, false, Some(1)),
        {"day": 1, "pulse": "afternoon", "air_group": 12, "aircraft": 7, "hex": [8, 2],
            "target_kind": "base", "found_nothing": false, "task_force": null, "base": 1}]);
    runs[0]["escorts"] = json!([escort(11, 4, 12), escort(14, 6, 15)]);
    runs[1]["launches"] = json!([launch(12, 18, true, None), launch(15, 9, false, Some(2))]);
    runs[1]["escorts"] = json!([escort(11, 4, 12), escort(14, 6, 15)]);
    runs[2]["launches"] = json!([launch(12, 18, false, Some(3))]);
    runs[2]["escorts"] = json!([escort(11, 4, 12)]);
    batch["per_run"] = json!(runs);
    batch["results"] = Value::Null;

    let head = "battle = \"hand-made\"\nscenario = \"data/scenarios/04a_night_action.toml\"\nfacts = \"x\"\n";
    let measure = |id: &str, who: &str, escorts: bool, record: u32| {
        format!(
            "\n[[measure]]\nid = \"{id}\"\ncounts = \"{id}\"\nrecord_low = {record}\nrecord_high = {record}\n\
             judged = true\ntake = \"aircraft_taking_part\"\n{who}\ntarget_task_forces = [1, 2]\n\
             escorts = {escorts}\nrows = [1]\nreason = \"r\"\n"
        )
    };
    let measures = format!(
        "{head}{}{}{}",
        measure("x", "side = \"japanese\"", true, 37),
        measure("y", "side = \"japanese\"", false, 27),
        measure("z", "air_groups = [15]", true, 15)
    );
    let s = setup("part", &batch, &measures);
    let result = validate(&s, "report.md");
    assert!(result.status.success(), "{}", stderr(&result));
    let report = fs::read_to_string(s.dir.join("report.md")).expect("report");
    expect(&report, "x", "0 to 37", "15", "1 of 3", "0", "agrees");
    expect(&report, "y", "0 to 27", "9", "1 of 3", "0", "agrees");
    expect(&report, "z", "0 to 15", "15", "2 of 3", "0", "agrees");
    assert!(report.contains("Judged measures: 3 agree, 0 agree at the edge, 0 miss, of 3."));

    // The keys it takes: a target base, no side or groups, both, an unknown
    // task force and a missing escorts are refused.
    let refused = |name: &str, body: &str, want: &str| {
        let s = setup(&format!("part-{name}"), &batch, &format!("{head}{}", body));
        let result = validate(&s, "r.md");
        assert!(!result.status.success(), "{name} should be refused");
        assert!(
            stderr(&result).contains(want),
            "{name}: {}",
            stderr(&result)
        );
    };
    let good = measure("x", "side = \"japanese\"", true, 37);
    refused(
        "base",
        &good.replace("target_task_forces = [1, 2]", "target_base = 1"),
        "does not take the key target_base",
    );
    refused(
        "both",
        &good.replace(
            "side = \"japanese\"",
            "side = \"japanese\"\nair_groups = [15]",
        ),
        "side or air_groups, not both",
    );
    refused(
        "neither",
        &good.replace("side = \"japanese\"\n", ""),
        "side or air_groups",
    );
    refused(
        "force-9",
        &good.replace("[1, 2]", "[1, 9]"),
        "task force 9, which the batch does not have",
    );
    refused(
        "group-9",
        &good.replace("side = \"japanese\"", "air_groups = [9]"),
        "air group 9",
    );
    refused(
        "no-escorts",
        &good.replace("escorts = true\n", ""),
        "needs the key escorts",
    );
}

#[test]
fn a_version_4_batch_is_refused_and_told_to_be_made_again() {
    let mut b = fixture(&dir("v4-h"), 20);
    b["version"] = json!(4);
    for run in b["per_run"].as_array_mut().expect("runs") {
        run.as_object_mut().expect("run").remove("launches");
    }
    let s = setup("v4", &b, MEASURES);
    let result = validate(&s, "r.md");
    assert!(!result.status.success());
    let err = stderr(&result);
    assert!(err.contains("version 4; validate reads"), "{err}");
    assert!(err.contains("make it again"), "{err}");
    assert!(!s.dir.join("r.md").exists());
}
