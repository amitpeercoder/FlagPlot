//! Not a test: a helper for the first run of scenario 103, the Eastern
//! Solomons over five days (decision 0184, "Points are not fitted to a run";
//! the report is `docs/validation/five-days-eastern-solomons-first-run.md`).
//! It plays file a, `data/scenarios/five-days/01a_eastern_solomons.toml`,
//! staff against staff on the live figures, on seeds FIRST to FIRST + RUNS -
//! 1 (by default 1 to 1000), exactly as `sim-cli batch` plays each run (the
//! scenario given the seed, each day resolved with `sim_staff::staffs()`),
//! and writes one JSON line a seed into the file named by the environment
//! variable `FLAGPILOT_FIVE_DAYS_OUT`: the final hash, the Allied margin and
//! outcome, and the counts behind the points, read from the game's own
//! result and events. The final hash and the margin are the batch's for the
//! same seed, which is how the report checks that the two are one run.
//! Since decision 0186 the Japanese side's objectives include men landed at
//! Taivu, so each line has its count and points under `men_landed` with the
//! others; beside them `men_landed_taivu` is every man the events show landed
//! there by the Japanese side and `japanese_men_embarked` every Japanese man
//! the events show embarked anywhere, so that the objective's count can be
//! checked against the landings (decision 0186, 22).
//!
//! It is ignored, so `cargo test` never runs it; it is run by hand:
//!
//! ```text
//! FLAGPILOT_FIVE_DAYS_OUT=FILE [FLAGPILOT_FIVE_DAYS_RUNS=N] \
//!     cargo test --release -p scenario-tests --test scenarios five_days_first_run:: \
//!     -- --ignored --nocapture
//! ```
//!
//! It is the umpire's: it reads the truth. Nothing that plays the game reads
//! it. It reads no Markdown file.

use std::fmt::Write as _;

use sim_core::events::EventKind;
use sim_core::hex::Hex;
use sim_core::refdata::ShipKind;
use sim_core::state::Side;
use sim_data::Scenario;
use sim_staff::{as_staffs, staffs};

use crate::common;

fn number(name: &str, default: u64) -> u64 {
    std::env::var(name)
        .ok()
        .map(|v| v.parse().expect("a whole number"))
        .unwrap_or(default)
}

#[test]
#[ignore = "a helper for the first run of scenario 103, run by hand"]
fn write_the_counts_of_each_seed() {
    let out =
        std::env::var("FLAGPILOT_FIVE_DAYS_OUT").expect("FLAGPILOT_FIVE_DAYS_OUT names a file");
    let first = number("FLAGPILOT_FIVE_DAYS_FIRST", 1);
    let runs = number("FLAGPILOT_FIVE_DAYS_RUNS", 1000);
    let path = common::data_dir().join("scenarios/five-days/01a_eastern_solomons.toml");
    let base = common::load_live(&path).expect("file a loads");
    let held = staffs();
    let taivu = Hex::new(3, -4);
    let mut text = String::new();
    for seed in first..first + runs {
        let mut scenario: Scenario = base.with_seed(seed).expect("the seed is taken");
        let mut hash = 0;
        while let Some(summary) = scenario
            .next_day_watched(as_staffs(&held), &mut |_, _| {})
            .expect("a day resolves")
        {
            hash = summary.state_hash;
        }
        let world = &scenario.world;
        let result = world.result().expect("a result");
        let mut men_landed_taivu = 0u32;
        let mut japanese_men_embarked = 0u32;
        for event in world.events().iter() {
            let japanese =
                |unit| world.umpire().land_units.get(&unit).map(|u| u.side) == Some(Side::Japanese);
            match event.kind {
                EventKind::MenLanded { hex, men, unit, .. } if hex == taivu && japanese(unit) => {
                    men_landed_taivu += men;
                }
                EventKind::MenEmbarked { men, unit, .. } if japanese(unit) => {
                    japanese_men_embarked += men;
                }
                _ => {}
            }
        }
        // What each side scored, by the objective's kind: the count, and for
        // the ship objectives the ships by kind and the carriers' days out.
        let mut sides = Vec::new();
        for side in Side::ALL {
            let r = result.side(side);
            let mut parts = Vec::new();
            for o in &r.objectives {
                let kinds: Vec<String> = o
                    .by_kind
                    .iter()
                    .map(|(k, n)| format!("\"{}\": {n}", k.name()))
                    .collect();
                let carriers_out: Vec<String> = o
                    .ships_out
                    .iter()
                    .filter(|s| s.kind == ShipKind::Carrier)
                    .map(|s| format!("[{}, {}]", s.ship.raw(), s.days_out))
                    .collect();
                parts.push(format!(
                    "\"{}\": {{\"count\": \"{}\", \"points\": \"{}\", \"by_kind\": {{{}}}, \"carriers_out\": [{}]}}",
                    o.kind,
                    o.count,
                    o.points,
                    kinds.join(", "),
                    carriers_out.join(", ")
                ));
            }
            sides.push(format!(
                "\"{}\": {{\"total\": \"{}\", {}}}",
                side.name().to_lowercase(),
                r.total,
                parts.join(", ")
            ));
        }
        writeln!(
            text,
            "{{\"seed\": {seed}, \"final_hash\": \"{hash:016x}\", \"allied_margin\": \"{}\", \"allied_outcome\": \"{}\", \"men_landed_taivu\": {men_landed_taivu}, \"japanese_men_embarked\": {japanese_men_embarked}, {}}}",
            result.allied.margin,
            result.allied.outcome.name(),
            sides.join(", ")
        )
        .expect("writes");
    }
    std::fs::write(&out, text).expect("the output file is written");
}
