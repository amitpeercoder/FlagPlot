//! Not a test: the harness of stage 4's proof (decision 0161; the setup,
//! `docs/validation/stage-4-proof-setup.md`, "What is to be made", item 3).
//! Each function below is an ignored test that is run by hand, as
//! `scripts/stage-4-proof/run.sh` runs them. Each writes what it read to a
//! file in the folder named by `FLAGPILOT_STAGE4_OUT` and asserts nothing,
//! so that the marks are read by the architect. `cargo test` never runs
//! them.
//!
//! ```text
//! FLAGPILOT_STAGE4_OUT=DIR [FLAGPILOT_STAGE4_SEED=2001] [FLAGPILOT_STAGE4_GAMES=4] \
//!     cargo test --release -p scenario-tests --test proof_stage_4_harness \
//!     -- --ignored --nocapture condition_7_spreads
//! ```
//!
//! # What is judged, and what a default is
//!
//! The judged run is on seeds 1 to 200 (conditions 7 and 8), seed 1
//! (conditions 4 and 5) and seeds 1 to 1,000 (condition 12). Nothing here
//! plays those unless told to: the first seed of every game is
//! `FLAGPILOT_STAGE4_SEED`, which is **2001** when it is not set, and the
//! games of one reading are `FLAGPILOT_STAGE4_GAMES`, which is **4** when it
//! is not set. The proof's run sets `FLAGPILOT_STAGE4_SEED=1` and the
//! games it needs (`run.sh` does). A case's trial is not a game and has no
//! seed of the game's: its worlds are the trial's figures' (1942) and the
//! trial's number.
//!
//! `FLAGPILOT_STAGE4_JOBS` is how many games are played at once (default 4);
//! a game is its own, so the answers are the same whatever it is.
//!
//! # The tests
//!
//! | Test | Condition | Writes |
//! | --- | --- | --- |
//! | `condition_4_no_peek` | 4, the 39 days at three days | `condition_4.txt` |
//! | `condition_5_the_same_twice` | 5 | `condition_5.txt` |
//! | `condition_7_spreads` | 7, cases S1 and S2 at three sizes | `condition_7.txt` |
//! | `condition_8_wrong_pictures` | 8, cases W1, W2 and W3 | `condition_8.txt` |
//! | `condition_9_preparation` | 9, case R | `condition_9.txt` |
//! | `condition_12_beside_the_games` | 12 | `condition_12.txt` |
//! | `condition_13_japanese_ashore_at_the_start` | 13 (setup, amendment 5) | `condition_13_units.txt` |
//! | `legibility_estimates` | 11 | `legibility/` |
//!
//! Each uses `data/reference/trial.toml` for a trial on (a), (b) or (c), as
//! `sim-cli time` does, and the case's own named file for a case (the
//! setup's amendment 2). The cases run on the frozen step 1 figures, as
//! their hand-calculation tests do; (a), (b) and (c) run on the live ones.
//!
//! These are the umpire's: they read the truth. Nothing that plays the game
//! reads what they write.
#![allow(clippy::doc_lazy_continuation, clippy::doc_markdown)]

mod common;

use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write as _;
use std::path::{Path, PathBuf};

use serde_json::{json, Value};
use sim_agent::{Seat, Session};
use sim_core::command::{CommandKind, IntentKind};
use sim_core::hex::Hex;
use sim_core::ids::IntentId;
use sim_core::refdata::TrialFigures;
use sim_core::state::Side;
use sim_data::Scenario;
use sim_staff::{as_staffs, staffs};
use sim_trial::{estimate, Asked, BasesAt, End, Estimate, Losses, Spread};

use common::stage_4::{case_text, course_of, hex, load_case, step_trial, Snapshot, Stepped4};

const ALLIED: Side = Side::Allied;
const JAPANESE: Side = Side::Japanese;
const B: Hex = Hex { q: 4, r: 0 };

// ------------------------------------------------------------------ settings

fn env(name: &str) -> Option<String> {
    std::env::var(name).ok().filter(|v| !v.trim().is_empty())
}

/// The folder everything is written to.
fn out_dir() -> PathBuf {
    let dir = PathBuf::from(env("FLAGPILOT_STAGE4_OUT").expect(
        "FLAGPILOT_STAGE4_OUT names the folder the harness writes to (outside the repository)",
    ));
    std::fs::create_dir_all(&dir).expect("the output folder is made");
    dir
}

/// The first seed of a game: 2,001 unless set. The judged run sets 1.
fn first_seed() -> u64 {
    env("FLAGPILOT_STAGE4_SEED").map_or(2001, |v| v.parse().expect("a seed is a whole number"))
}

/// How many games a reading plays: 4 unless set.
fn games() -> u64 {
    env("FLAGPILOT_STAGE4_GAMES").map_or(4, |v| v.parse().expect("a count of games"))
}

fn jobs() -> usize {
    env("FLAGPILOT_STAGE4_JOBS").map_or(4, |v| v.parse().expect("a count of jobs"))
}

fn write(name: &str, text: &str) {
    let path = out_dir().join(name);
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).expect("a folder is made");
    }
    std::fs::write(&path, text).expect("the file is written");
    println!("wrote {}", path.display());
}

/// `f` of each seed, `jobs` at a time, in the order of the seeds.
fn par_map<T: Send>(seeds: &[u64], f: impl Fn(u64) -> T + Sync) -> Vec<T> {
    let jobs = jobs().max(1);
    let mut slots: Vec<Option<T>> = (0..seeds.len()).map(|_| None).collect();
    let next = std::sync::atomic::AtomicUsize::new(0);
    let done = std::sync::Mutex::new(&mut slots);
    std::thread::scope(|scope| {
        for _ in 0..jobs {
            scope.spawn(|| loop {
                let i = next.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                if i >= seeds.len() {
                    break;
                }
                let value = f(seeds[i]);
                done.lock().expect("a lock")[i] = Some(value);
            });
        }
    });
    slots
        .into_iter()
        .map(|v| v.expect("a game was played"))
        .collect()
}

fn seeds(count: u64) -> Vec<u64> {
    (first_seed()..first_seed() + count).collect()
}

// ------------------------------------------------------------- the arrangements

fn slice_path(name: &str) -> PathBuf {
    common::data_dir().join(format!("scenarios/stage-3-proof/{name}.toml"))
}

/// An arrangement of stage 3's proof, on the live figures, as sim-cli loads it.
fn arrangement(name: &str) -> Scenario {
    Scenario::load(&slice_path(name)).expect("the arrangement loads")
}

fn arrangement_from_text(text: &str) -> Result<Scenario, String> {
    Scenario::from_text_with(
        text,
        &slice_path("07a_slice"),
        &sim_data::LoadOptions::default(),
    )
    .map_err(|e| e.to_string())
}

/// The trial's figures a trial on an arrangement is given: the scenario's
/// own where it names them, else `data/reference/trial.toml` (the setup,
/// amendment 2).
fn slice_figures(scenario: &Scenario) -> TrialFigures {
    scenario.trial_figures().cloned().unwrap_or_else(|| {
        sim_data::load_trial_figures(&common::data_dir().join("reference/trial.toml"))
            .expect("the trial's figures load")
    })
}

/// The commands of `side` of day 1 of a scenario.
fn day_one(scenario: &Scenario, side: Side) -> Vec<CommandKind> {
    scenario
        .scripted_commands()
        .get(&1)
        .map(|day| {
            day.iter()
                .filter(|(s, _)| *s == side)
                .map(|(_, c)| c.clone())
                .collect()
        })
        .unwrap_or_default()
}

fn first_intent(commands: &[CommandKind]) -> Option<IntentId> {
    commands.iter().find_map(|c| match c {
        CommandKind::SetIntent { intent, .. } => Some(*intent),
        _ => None,
    })
}

/// The estimate of `side`'s first intent on `scenario` from the evening
/// before day 1, at a size.
fn estimate_of_first(
    scenario: &Scenario,
    side: Side,
    figures: &TrialFigures,
    size: (&str, sim_core::refdata::TrialSize),
) -> Option<(IntentId, Estimate)> {
    let commands = day_one(scenario, side);
    let intent = first_intent(&commands)?;
    let course = commands
        .into_iter()
        .map(sim_trial::CourseCommand::first_day)
        .collect();
    let played = sim_trial::play(
        &scenario.world.side_view(side),
        &common::stage_4::believed(scenario, side),
        figures,
        scenario.world.ref_data(),
        scenario.world.map(),
        Asked {
            trial: 1,
            worlds: size.1.worlds,
            days: size.1.days,
            course,
        },
    )
    .expect("the trial begins");
    Some((intent, estimate(&played, intent, None)))
}

// ------------------------------------------------------------ reading a game

/// What a game gave of one side, as the estimate reads a world: for each day
/// whether the end was met at the day's end, and the losses at the days
/// asked for.
struct Read {
    met: Vec<bool>,
    losses: BTreeMap<u32, Losses>,
}

impl Read {
    /// The first day the end was met by `horizon`, and whether it was kept to
    /// its end.
    fn met_by(&self, horizon: u32) -> (Option<u32>, bool) {
        let days = &self.met[..(horizon as usize).min(self.met.len())];
        let first = days.iter().position(|m| *m).map(|i| i as u32 + 1);
        let kept = days.last().copied().unwrap_or(false);
        (first, kept)
    }

    fn lost(&self, day: u32) -> Losses {
        self.losses[&day]
    }
}

/// A scenario played to its end with both staffs, read for each watched
/// side at each day.
fn play_read(mut scenario: Scenario, watches: &[(Side, Option<End>)], at: &[u32]) -> Vec<Read> {
    let held = staffs();
    let mut reads: Vec<Read> = watches
        .iter()
        .map(|_| Read {
            met: Vec::new(),
            losses: BTreeMap::new(),
        })
        .collect();
    loop {
        let day = scenario.world.day() + 1;
        if scenario
            .next_day_staffed(as_staffs(&held))
            .expect("a day resolves")
            .is_none()
        {
            break;
        }
        for ((side, end), read) in watches.iter().zip(&mut reads) {
            let view = scenario.world.side_view(*side);
            let met = end
                .as_ref()
                .is_some_and(|end| sim_trial::met(end, &BasesAt::of(&view)));
            read.met.push(met);
            if at.contains(&day) {
                read.losses.insert(day, Losses::of(&view));
            }
        }
    }
    reads
}

fn cost_values(losses: &Losses) -> [u32; 5] {
    [
        losses.men_to_enemy,
        losses.men_unfed,
        losses.ships_sunk,
        losses.ships_out_of_action,
        losses.aircraft,
    ]
}

const COSTS: [&str; 5] = [
    "men lost to the enemy",
    "men lost unfed",
    "ships sunk",
    "ships out of action",
    "aircraft",
];

fn middle(values: &[u32]) -> u32 {
    let mut v = values.to_vec();
    v.sort_unstable();
    v[v.len().div_ceil(2) - 1]
}

/// The estimate beside the games, as condition 7 reads it: for each cost the
/// worlds' lowest, middle and highest, the games inside, whether the cost
/// is tested, and whether its middle half is wider than its middle; and K
/// and M.
fn beside_games(e: &Estimate, games: &[&Read], horizon: u32) -> String {
    let mut out = String::new();
    let worlds = e.worlds.len();
    let n = games.len();
    let _ = writeln!(
        out,
        "  worlds played {worlds} of {} asked, games {n}, horizon day {horizon}",
        e.asked
    );
    let mut tested = 0;
    let mut met_all_costs = 0;
    let mut too_wide = 0;
    for (i, name) in COSTS.iter().enumerate() {
        let w: Vec<u32> = e.worlds.iter().map(|f| cost_values(&f.side)[i]).collect();
        let g: Vec<u32> = games
            .iter()
            .map(|r| cost_values(&r.lost(horizon))[i])
            .collect();
        let (lo, hi) = (*w.iter().min().unwrap(), *w.iter().max().unwrap());
        let mid = middle(&w);
        let inside = g.iter().filter(|v| **v >= lo && **v <= hi).count();
        let constant = w.iter().chain(&g).all(|v| *v == w[0]);
        let mut sorted = w.clone();
        sorted.sort_unstable();
        let (q1, q3) = (sorted[worlds / 4], sorted[(3 * worlds / 4).max(1) - 1]);
        let wide = mid > 0 && q3 - q1 > mid;
        if !constant {
            tested += 1;
            if inside * 10 >= n * 9 {
                met_all_costs += 1;
            }
        }
        if wide {
            too_wide += 1;
        }
        let _ = writeln!(
            out,
            "  {name}: worlds {lo}/{mid}/{hi}; games inside {inside} of {n}{}; middle half {q1} to {q3}{}",
            if constant {
                " (the same number in every world and every game: not tested)"
            } else {
                ""
            },
            if wide { " (wider than the middle)" } else { "" }
        );
    }
    let k = e
        .worlds
        .iter()
        .filter(|f| f.met_day.is_some() && !f.lost_again)
        .count();
    let m = games
        .iter()
        .filter(|r| {
            let (first, kept) = r.met_by(horizon);
            first.is_some() && kept
        })
        .count();
    let (kf, mf) = (k as f64 / worlds as f64, m as f64 / n as f64);
    let _ = writeln!(
        out,
        "  the end met and kept: worlds K = {k} of {worlds} ({kf:.4}); games M = {m} of {n} ({mf:.4}); difference {:.4}",
        (kf - mf).abs()
    );
    if (k == 0 && m == 0) || (k == worlds && m == n) {
        let _ = writeln!(out, "  the case showed nothing of the count");
    }
    let _ = writeln!(
        out,
        "  costs tested {tested}; tested with at least 90 in 100 games inside {met_all_costs}; costs whose middle half is wider than the middle {too_wide}"
    );
    out
}

fn spread_words(s: Spread) -> String {
    format!("{}/{}/{}", s.lowest, s.middle, s.highest)
}

// ----------------------------------------------------------------- the cases

/// Scenario 95's four commands of day 1, the forces limit naming `units`.
fn take_course(units: &[u32]) -> String {
    let units = units
        .iter()
        .map(u32::to_string)
        .collect::<Vec<_>>()
        .join(", ");
    format!(
        r#"
[[command]]
day = 1
side = "allied"
kind = "set_intent"
intent = 1
intent_kind = "take_place"
hex = [4, 0]

[[command]]
day = 1
side = "allied"
kind = "set_limit"
limit = "forces"
intent = 1
task_forces = [1]
land_units = [{units}]
source = 1

[[command]]
day = 1
side = "allied"
kind = "set_leave"
leave = "prepare"
given = true

[[command]]
day = 1
side = "allied"
kind = "set_leave"
leave = "launch"
given = true
"#
    )
}

/// A case of the proof with `course` appended to its file.
fn case_with(file: &str, course: &str) -> (String, Scenario) {
    let text = format!("{}\n{course}", case_text(file));
    let scenario = load_case(file, &text);
    (text, scenario)
}

/// The trial of a case at one size, from the evening before day 1.
fn trial_of_case(scenario: &Scenario, size: (&str, sim_core::refdata::TrialSize)) -> Estimate {
    let course = course_of(scenario, ALLIED);
    let played = sim_trial::play(
        &scenario.world.side_view(ALLIED),
        &common::stage_4::believed(scenario, ALLIED),
        &common::stage_4::figures(scenario),
        scenario.world.ref_data(),
        scenario.world.map(),
        Asked {
            trial: 1,
            worlds: size.1.worlds,
            days: size.1.days,
            course,
        },
    )
    .expect("the trial begins");
    estimate(&played, IntentId::new(1), None)
}

/// `count` games of a case from the text, one a seed, each read for the
/// Allied side at `at`, the end being `end`.
fn case_games(file: &str, text: &str, end: &Option<End>, seeds: &[u64], at: &[u32]) -> Vec<Read> {
    par_map(seeds, |seed| {
        let scenario = load_case(file, text).with_seed(seed).expect("a seed");
        play_read(scenario, &[(ALLIED, end.clone())], at)
            .pop()
            .expect("a read")
    })
}

// ------------------------------------------------------------- condition 7

/// Condition 7: the spreads hold. Cases S1 and S2, a trial of each at each
/// size beside the games.
#[test]
#[ignore = "run by hand by the proof's harness"]
fn condition_7_spreads() {
    let mut out = String::new();
    let _ = writeln!(
        out,
        "Condition 7. First game seed {}, {} games a case. The mark is read at the full size; the smaller sizes are reported beside it and judged by nothing.\n",
        first_seed(),
        games()
    );
    for (name, file, units) in [
        ("S1", "case_s1_one_unit_against_200", vec![1]),
        ("S2", "case_s2_two_units_against_300", vec![1, 2]),
    ] {
        let course = take_course(&units);
        let (text, scenario) = case_with(file, &course);
        let figures = common::stage_4::figures(&scenario);
        let sizes = figures.sizes();
        let full = trial_of_case(&scenario, sizes[2]);
        let end = full.end.clone();
        let horizons: Vec<u32> = sizes.iter().map(|s| s.1.days).collect();
        let reads = case_games(file, &text, &end, &seeds(games()), &horizons);
        let refs: Vec<&Read> = reads.iter().collect();
        let _ = writeln!(out, "== Case {name} ({file})");
        for (i, size) in sizes.iter().enumerate() {
            let e = if i == 2 {
                full.clone()
            } else {
                trial_of_case(&scenario, *size)
            };
            let _ = writeln!(
                out,
                "-- {} size: {} worlds for {} days{}",
                size.0,
                size.1.worlds,
                size.1.days,
                if i == 2 { " (the judged size)" } else { "" }
            );
            out.push_str(&beside_games(&e, &refs, size.1.days));
            let _ = writeln!(out, "  estimate in words:\n{}", indent(&e.to_string()));
            // The day the end was met, as a spread, worlds beside games.
            let world_days: Vec<u32> = e.worlds.iter().filter_map(|f| f.met_day).collect();
            let game_days: Vec<u32> = refs
                .iter()
                .filter_map(|r| r.met_by(size.1.days).0)
                .collect();
            let _ = writeln!(
                out,
                "  the day first met: worlds {:?}, games {:?}\n",
                Spread::of(&world_days).map(spread_words),
                Spread::of(&game_days).map(spread_words)
            );
        }
    }
    write("condition_7.txt", &out);
}

fn indent(text: &str) -> String {
    text.lines()
        .map(|l| format!("    {l}"))
        .collect::<Vec<_>>()
        .join("\n")
}

// ------------------------------------------------------------- condition 8

/// Condition 8: a wrong picture moves it as expected. Cases W1, W2 and W3.
#[test]
#[ignore = "run by hand by the proof's harness"]
fn condition_8_wrong_pictures() {
    let mut out = String::new();
    let _ = writeln!(
        out,
        "Condition 8. First game seed {}, {} games where a case has many.\n",
        first_seed(),
        games()
    );

    // W1: the standard size, and one game.
    {
        let file = "case_w1_the_garrison_believed_half";
        let (text, scenario) = case_with(file, &take_course(&[1]));
        let sizes = common::stage_4::figures(&scenario).sizes();
        let e = trial_of_case(&scenario, sizes[1]);
        let end = e.end.clone();
        let read = case_games(file, &text, &end, &[first_seed()], &[4, 5, 14]);
        let r = &read[0];
        let _ = writeln!(out, "== W1, the garrison believed half the truth ({file})");
        let _ = writeln!(
            out,
            "  the estimate (standard, {} worlds for {} days): went {:?}; costs {:?}",
            sizes[1].1.worlds, sizes[1].1.days, e.went, e.side_costs
        );
        let _ = writeln!(
            out,
            "  the game, seed {}: place first met by day 14 on {:?} (kept: {}); men lost by day 4 {}, day 5 {}, day 14 {}",
            first_seed(),
            r.met_by(14).0,
            r.met_by(14).1,
            r.lost(4).men_to_enemy,
            r.lost(5).men_to_enemy,
            r.lost(14).men_to_enemy
        );
        let named = e.assumed.iter().any(
            |a| matches!(a.of, sim_trial::AssumedOf::MenAshore { hex: h, men: 200 } if h == B),
        );
        let _ = writeln!(
            out,
            "  the assumptions name the men at the place as 200 from the estimate: {named}"
        );
        let _ = writeln!(out, "  estimate in words:\n{}\n", indent(&e.to_string()));
    }

    // W2: the Japanese side's trial of an empty course at the small size (8
    // worlds for 7 days), and games on seeds first to first + games - 1, 7
    // days each (scenario 5b read from the Japanese seat).
    {
        let file = "case_w2_a_carrier_believed_absent";
        let text = case_text(file);
        let scenario = load_case(file, &text);
        let sizes = common::stage_4::figures(&scenario).sizes();
        let size = sizes[0];
        let played = sim_trial::play(
            &scenario.world.side_view(JAPANESE),
            &common::stage_4::believed(&scenario, JAPANESE),
            &common::stage_4::figures(&scenario),
            scenario.world.ref_data(),
            scenario.world.map(),
            Asked {
                trial: 1,
                worlds: size.1.worlds,
                days: size.1.days,
                course: Vec::new(),
            },
        );
        let _ = writeln!(
            out,
            "== W2, a carrier force believed absent that is there ({file})"
        );
        match played {
            Err(refused) => {
                let _ = writeln!(
                    out,
                    "  the trial was REFUSED: {refused:?} (a picture with no enemy: to the architect before the run)\n"
                );
            }
            Ok(played) => {
                let e = estimate(&played, IntentId::new(1), None);
                let reads = par_map(&seeds(games()), |seed| {
                    let s = load_case(file, &text).with_seed(seed).expect("a seed");
                    play_read(s, &[(JAPANESE, None)], &[7])
                        .pop()
                        .expect("a read")
                });
                let costs = e.side_costs.expect("costs");
                let _ = writeln!(
                    out,
                    "  the estimate ({} worlds for {} days): ships sunk {}, ships out of action {}, aircraft {} (lowest/middle/highest)",
                    size.1.worlds,
                    size.1.days,
                    spread_words(costs.ships_sunk),
                    spread_words(costs.ships_out_of_action),
                    spread_words(costs.aircraft)
                );
                let sunk = reads.iter().filter(|r| r.lost(7).ships_sunk > 0).count();
                let hurt = reads
                    .iter()
                    .filter(|r| r.lost(7).ships_sunk + r.lost(7).ships_out_of_action > 0)
                    .count();
                let _ = writeln!(
                    out,
                    "  games read: {}; with a ship sunk: {sunk} (the mark is at least 100 of 200); with a ship sunk or out of action: {hurt}",
                    reads.len()
                );
                let named = e.assumed.iter().any(|a| {
                    matches!(a.of, sim_trial::AssumedOf::BelievedForce { .. })
                        && a.notes
                            .iter()
                            .any(|n| matches!(n, sim_trial::AssumedNote::RuledOut { .. }))
                        && a.made_in.is_empty()
                });
                let _ = writeln!(
                    out,
                    "  the assumptions name the carrier force as believed absent and in no world: {named}"
                );
                let _ = writeln!(out, "  estimate in words:\n{}\n", indent(&e.to_string()));
            }
        }
    }

    // W3: the standard size, and games on seeds first to first + 19.
    {
        let file = "case_w3_an_intent_not_as_stated";
        let (text, scenario) = case_with(file, &take_course(&[1]));
        let sizes = common::stage_4::figures(&scenario).sizes();
        let e = trial_of_case(&scenario, sizes[1]);
        let count = games().min(20);
        let reads = case_games(file, &text, &e.end, &seeds(count), &[8, 14]);
        let _ = writeln!(out, "== W3, an enemy intent not as stated ({file})");
        let _ = writeln!(
            out,
            "  the estimate (standard): went {:?}; side costs {:?}",
            e.went, e.side_costs
        );
        let highest = e.side_costs.map_or(0, |c| c.men_to_enemy.highest);
        let more = reads
            .iter()
            .filter(|r| r.lost(8).men_to_enemy > highest)
            .count();
        let _ = writeln!(
            out,
            "  games read: {}; men lost by the end of day 8, per game: {:?}; more than the estimate's highest ({highest}): {more}",
            reads.len(),
            reads.iter().map(|r| r.lost(8).men_to_enemy).collect::<Vec<_>>()
        );
        let _ = writeln!(
            out,
            "  the place first met and lost again by day 14 (reported, judged by nothing): {:?}",
            reads
                .iter()
                .map(|r| {
                    let (first, kept) = r.met_by(14);
                    (first, first.is_some() && !kept)
                })
                .collect::<Vec<_>>()
        );
        let named = e
            .assumed
            .iter()
            .any(|a| matches!(a.of, sim_trial::AssumedOf::EnemyGivenNoIntent));
        let _ = writeln!(
            out,
            "  the assumptions say the enemy was given no intent: {named}"
        );
        let _ = writeln!(out, "  estimate in words:\n{}\n", indent(&e.to_string()));
    }
    write("condition_8.txt", &out);
}

// ------------------------------------------------------------- condition 9

/// Condition 9: preparation matters. Case R, two trials of one number at the
/// standard size, one with the course handed in on day 1 and one on day 4.
#[test]
#[ignore = "run by hand by the proof's harness"]
fn condition_9_preparation() {
    let file = "case_r_preparation";
    let course_text =
        |day: u32| {
            let mut text = String::new();
            for (kind, extra) in [
            ("set_intent", "intent = 1\nintent_kind = \"take_place\"\nhex = [4, 0]\n"),
            (
                "set_limit",
                "limit = \"forces\"\nintent = 1\ntask_forces = [1]\nland_units = [1]\nsource = 1\n",
            ),
            ("set_limit", "limit = \"risk\"\nintent = 1\nrisk = \"press\"\n"),
            ("set_limit", "limit = \"not_before\"\nintent = 1\nfrom_day = 9\n"),
            ("set_leave", "leave = \"prepare\"\ngiven = true\n"),
            ("set_leave", "leave = \"launch\"\ngiven = true\n"),
        ] {
            let _ = write!(
                text,
                "\n[[command]]\nday = {day}\nside = \"allied\"\nkind = \"{kind}\"\n{extra}"
            );
        }
            text
        };
    let mut results: Vec<Stepped4> = Vec::new();
    for day in [1, 4] {
        let (_, scenario) = case_with(file, &course_text(day));
        let course = course_of(&scenario, ALLIED);
        results.push(step_trial(&scenario, ALLIED, 1, 16, 14, course));
    }
    let (early, late) = (&results[0], &results[1]);
    let mut out = String::from(
        "Condition 9. Case R, two trials of one number, 16 worlds for 14 days: the course handed in on day 1 (early) and on day 4 (late). On the evening of day 11, the unit's fit men by the side's own view; a world in which a course did not land the unit that day counts as not more.\n\n",
    );
    let mut more = 0;
    for world in 1..=16u32 {
        let reading = |s: &Snapshot| {
            let unit = &s.truth.land_units[&sim_core::ids::LandUnitId::new(1)];
            let landed = unit.men_ashore > 0 && unit.hex == Some(hex(4, 0));
            (landed, s.fit.get(&1).copied().unwrap_or(0))
        };
        let (le, fe) = reading(early.snapshot(world, 11));
        let (ll, fl) = reading(late.snapshot(world, 11));
        let counts = le && ll && fe > fl;
        if counts {
            more += 1;
        }
        let _ = writeln!(
            out,
            "world {world}: early landed {le}, fit {fe}; late landed {ll}, fit {fl}; more in the early course: {counts}"
        );
    }
    let _ = writeln!(
        out,
        "\nWorlds in which the early course has more: {more} of 16 (the mark is at least 15)."
    );
    write("condition_9.txt", &out);
}

// ------------------------------------------------------------ sessions on (a)

fn session_of(mut scenario: Scenario, player: Side, figures: &TrialFigures) -> Session {
    let runner = sim_cli::seat_trial::SeatTrials::with_figures(&scenario, figures.clone());
    let commands: BTreeMap<u32, Vec<(Side, CommandKind)>> = scenario
        .scripted_commands()
        .iter()
        .map(|(day, list)| {
            (
                *day,
                list.iter().filter(|(s, _)| *s != player).cloned().collect(),
            )
        })
        .collect();
    scenario.replace_commands(BTreeMap::new());
    let inside = scenario.orders_inside_a_day();
    let script_inside = scenario.scripted_orders_inside_day().clone();
    let name = scenario.name.clone();
    let seat = |side: Side| {
        if side == player {
            Seat::Player
        } else {
            Seat::Script(BTreeMap::new())
        }
    };
    Session::new(
        scenario.world,
        &name,
        scenario.days,
        seat(ALLIED),
        seat(JAPANESE),
    )
    .with_staffs(staffs())
    .with_commands(commands)
    .with_orders_inside_a_day(inside, script_inside)
    .with_trial(Box::new(runner))
}

fn tool(session: &mut Session, side: Side, name: &str, args: Value) -> Result<Value, String> {
    session.call(side, name, &args).map_err(|e| e.message)
}

/// The seat queues what the scenario's script gives its side on day 1: the
/// two intents "control an island" and both leaves, as stated.
fn queue_as_stated(session: &mut Session, side: Side, commands: &[CommandKind]) -> Vec<String> {
    let mut notes = Vec::new();
    for command in commands {
        let (name, args) = match command {
            CommandKind::SetIntent {
                kind: IntentKind::ControlIsland { island, .. },
                ..
            } => (
                "set_intent",
                json!({"intent_kind": "control_island", "island": island}),
            ),
            CommandKind::SetLeave { leave, given } => (
                "set_leave",
                json!({"leave": format!("{leave:?}").to_lowercase(), "given": given}),
            ),
            other => {
                notes.push(format!("not queued: {other:?}"));
                continue;
            }
        };
        match tool(session, side, name, args.clone()) {
            Ok(a) => notes.push(format!("{name} {args}: {}", a["status"])),
            Err(e) => notes.push(format!("{name} {args}: refused: {e}")),
        }
    }
    notes
}

fn end_days(session: &mut Session, side: Side, count: u32) {
    for _ in 0..count {
        tool(session, side, "end_day", json!({})).expect("a day ends");
    }
}

/// The side's whole picture as the seat's tools give it, for a field for
/// field comparison.
fn picture_of(session: &mut Session, side: Side) -> String {
    let mut out = String::new();
    for name in [
        "get_briefing",
        "list_forces",
        "list_contacts",
        "list_bases",
        "command_state",
    ] {
        let answer = tool(session, side, name, json!({}));
        let _ = writeln!(out, "== {name}\n{answer:#?}");
    }
    out
}

/// The answer of a trial of intent 1, as the seat is given it, and the same
/// with its number among the estimates the seat was shown set aside (the
/// seat's second answer is numbered 2, by design, and says so).
fn ask_trial_both(session: &mut Session, side: Side, size: &str) -> (String, String) {
    match tool(session, side, "trial", json!({"intent": 1, "size": size})) {
        Ok(a) => {
            let raw = a.to_string();
            let mut set_aside = a;
            set_aside["estimate"] = Value::Null;
            (raw, set_aside.to_string())
        }
        Err(e) => (format!("refused: {e}"), format!("refused: {e}")),
    }
}

fn ask_trial(session: &mut Session, side: Side, size: &str) -> String {
    ask_trial_both(session, side, size).0
}

// ------------------------------------------------------------- condition 5

/// Condition 5: the same twice, and nothing changed. On (a), the trial of the
/// first intent at the small size asked twice on the evenings before day 1,
/// of day 3 and of day 20; and the game played to day 39 with the trial asked
/// on every morning against the game never asked: the daily hashes.
#[test]
#[ignore = "run by hand by the proof's harness"]
fn condition_5_the_same_twice() {
    let mut out = format!("Condition 5. Arrangement (a), seed {}.\n\n", first_seed());
    for side in [ALLIED, JAPANESE] {
        let base = arrangement("07a_slice")
            .with_seed(first_seed())
            .expect("a seed");
        let figures = slice_figures(&base);
        let stated = day_one(&base, side);
        let _ = writeln!(out, "== The {side:?} seat");
        let mut asked = session_of(base.with_seed(first_seed()).unwrap(), side, &figures);
        for note in queue_as_stated(&mut asked, side, &stated) {
            let _ = writeln!(out, "  {note}");
        }
        let mut plain = session_of(base.with_seed(first_seed()).unwrap(), side, &figures);
        queue_as_stated(&mut plain, side, &stated);
        let mut day = 0u32;
        for evening in [0u32, 3, 20] {
            end_days(&mut asked, side, evening - day);
            end_days(&mut plain, side, evening - day);
            day = evening;
            let first = ask_trial_both(&mut asked, side, "small");
            let second = ask_trial_both(&mut asked, side, "small");
            let serial = |raw: &str| {
                serde_json::from_str::<Value>(raw)
                    .map_or(Value::Null, |v| v["estimate"].clone())
                    .to_string()
            };
            let _ = writeln!(
                out,
                "  evening of day {evening}: the answer's own number goes from {} to {}; the two answers equal byte for byte: {}; equal with that number set aside (the mark is read on this): {} ({} bytes)",
                serial(&first.0),
                serial(&second.0),
                first.0 == second.0,
                first.1 == second.1,
                first.0.len()
            );
        }
        // Played on to day 39 with a trial asked on every morning, against a
        // game never asked.
        let before = asked.state_hash();
        let _ = before;
        let mut hashes = Vec::new();
        let mut plain_hashes = Vec::new();
        let mut live = session_of(base.with_seed(first_seed()).unwrap(), side, &figures);
        queue_as_stated(&mut live, side, &stated);
        for _day in 1..=39 {
            ask_trial(&mut live, side, "small");
            end_days(&mut live, side, 1);
            hashes.push(live.state_hash());
        }
        let mut never = session_of(base.with_seed(first_seed()).unwrap(), side, &figures);
        queue_as_stated(&mut never, side, &stated);
        for _day in 1..=39 {
            end_days(&mut never, side, 1);
            plain_hashes.push(never.state_hash());
        }
        let differ: Vec<usize> = hashes
            .iter()
            .zip(&plain_hashes)
            .enumerate()
            .filter(|(_, (a, b))| a != b)
            .map(|(i, _)| i + 1)
            .collect();
        let _ = writeln!(
            out,
            "  daily hashes with a trial asked on every morning against never asked: {} days compared; days that differ: {:?}\n",
            hashes.len(),
            differ
        );
    }
    write("condition_5.txt", &out);
}

// ------------------------------------------------------------- condition 4

/// A block of a scenario's text and the key = value lines it holds.
fn blocks(text: &str) -> Vec<String> {
    let mut blocks: Vec<String> = Vec::new();
    for line in text.lines() {
        if line.starts_with("[[") || blocks.is_empty() {
            blocks.push(String::new());
        }
        let last = blocks.last_mut().expect("a block");
        last.push_str(line);
        last.push('\n');
    }
    blocks
}

fn field(block: &str, key: &str) -> Option<String> {
    block
        .lines()
        .find_map(|l| l.strip_prefix(&format!("{key} = ")).map(str::to_string))
}

fn set_field(block: &str, key: &str, value: i64) -> String {
    block
        .lines()
        .map(|l| {
            if l.starts_with(&format!("{key} = ")) {
                format!("{key} = {value}")
            } else {
                l.to_string()
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
        + "\n"
}

/// The text of (a) with `change` applied to each block `pick` selects, the
/// block's number in the file being `which`.
fn changed_text(text: &str, mut edit: impl FnMut(usize, &str) -> Option<String>) -> String {
    blocks(text)
        .iter()
        .enumerate()
        .map(|(i, b)| edit(i, b).unwrap_or_else(|| b.clone()))
        .collect::<String>()
}

/// Raises the supply of the base named `name` by `by` tons.
fn base_supply_raised(text: &str, name: &str, by: i64) -> String {
    changed_text(text, |_, b| {
        if b.starts_with("[[base]]") && field(b, "name").as_deref() == Some(&format!("\"{name}\""))
        {
            let now: i64 = field(b, "supply_t")?.parse().ok()?;
            Some(set_field(b, "supply_t", now + by))
        } else {
            None
        }
    })
}

/// The ids of the blocks of `kind` with a `side`.
fn ids_of(text: &str, kind: &str, side: &str) -> Vec<u32> {
    blocks(text)
        .iter()
        .filter(|b| b.starts_with(&format!("[[{kind}]]")))
        .filter(|b| field(b, "side").as_deref() == Some(&format!("\"{side}\"")))
        .filter_map(|b| field(b, "id")?.parse().ok())
        .collect()
}

fn block_changed(text: &str, kind: &str, id: u32, key: &str, by: i64) -> Option<String> {
    let mut found = false;
    let changed = changed_text(text, |_, b| {
        if b.starts_with(&format!("[[{kind}]]")) && field(b, "id")?.parse::<u32>().ok()? == id {
            let now: i64 = field(b, key)?.parse().ok()?;
            found = true;
            Some(set_field(b, key, (now + by).max(0)))
        } else {
            None
        }
    });
    found.then_some(changed)
}

/// Condition 4: no peek, on the 39 days at three days. Two pairs, each of
/// (a) and a copy that differs only in things the asking side cannot see on
/// the evening of day 3.
#[test]
#[ignore = "run by hand by the proof's harness"]
fn condition_4_no_peek() {
    let text = std::fs::read_to_string(slice_path("07a_slice")).expect("(a) reads");
    let base = arrangement("07a_slice");
    let figures = slice_figures(&base);
    let mut out = format!(
        "Condition 4. Arrangement (a), seed {}, the evening of day 3, the side's own first intent as stated, the small size.\n\n",
        first_seed()
    );
    // (the asking side, the base whose supply is raised by 1,000 t, the
    // hidden things to try in turn: (kind, id, key, by))
    let japanese_units: Vec<u32> = {
        let mut ids = ids_of(&text, "land_unit", "japanese");
        ids.reverse();
        ids
    };
    let allied_ships: Vec<u32> = {
        let mut ids = ids_of(&text, "ship", "allied");
        ids.reverse();
        ids
    };
    for (side, base_name, kind, ids, key, by) in [
        (ALLIED, "Rabaul", "land_unit", japanese_units, "men", 100i64),
        (
            JAPANESE,
            "Espiritu Santo",
            "ship",
            allied_ships,
            "fuel_t",
            -100i64,
        ),
    ] {
        let _ = writeln!(out, "== The {side:?} seat: {base_name}'s supply raised by 1,000 t, and one {kind}'s {key} changed by {by}");
        let raised = base_supply_raised(&text, base_name, 1000);
        let stated = day_one(&base, side);
        let mut used = false;
        for id in ids {
            let Some(copy) = block_changed(&raised, kind, id, key, by) else {
                continue;
            };
            let make = |text: &str| -> Result<Session, String> {
                let scenario = arrangement_from_text(text)?
                    .with_seed(first_seed())
                    .map_err(|e| e.to_string())?;
                let mut session = session_of(scenario, side, &figures);
                queue_as_stated(&mut session, side, &stated);
                end_days(&mut session, side, 3);
                Ok(session)
            };
            let (mut one, mut two) = match (make(&text), make(&copy)) {
                (Ok(one), Ok(two)) => (one, two),
                (_, Err(why)) | (Err(why), _) => {
                    let _ = writeln!(out, "  candidate {kind} {id}: the copy does not load ({why}): void, another hidden thing is tried");
                    continue;
                }
            };
            let (h1, h2) = (one.state_hash(), two.state_hash());
            let (p1, p2) = (picture_of(&mut one, side), picture_of(&mut two, side));
            let _ = writeln!(
                out,
                "  candidate {kind} {id}: state hashes on the evening of day 3 {} ; the side's picture (briefing, forces, contacts, bases, command state) equal field for field: {}",
                if h1 == h2 { "EQUAL (the games do not differ)" } else { "differ" },
                p1 == p2
            );
            if h1 == h2 || p1 != p2 {
                let _ = writeln!(out, "    void: another hidden thing is tried");
                continue;
            }
            let a1 = ask_trial(&mut one, side, "small");
            let a2 = ask_trial(&mut two, side, "small");
            let _ = writeln!(
                out,
                "    the two answers (each the first asked in its game) equal byte for byte: {} ({} bytes)",
                a1 == a2,
                a1.len()
            );
            used = true;
            break;
        }
        if !used {
            let _ = writeln!(out, "  no hidden thing gave a pair: the pair is void");
        }
        out.push('\n');
    }
    write("condition_4.txt", &out);
}

// ------------------------------------------------------------ condition 12

/// Condition 12: reported, not judged. On (a), from the evening before day
/// 1, for each side, the estimate of its first intent at the full size beside
/// what the games did by the end of day 21; and from (b) and (c) the same.
#[test]
#[ignore = "run by hand by the proof's harness"]
fn condition_12_beside_the_games() {
    let mut out = format!(
        "Condition 12. Reported, judged by nothing. First game seed {}, {} games an arrangement.\n\n",
        first_seed(),
        games()
    );
    let base = arrangement("07a_slice");
    let figures = slice_figures(&base);
    let full = figures.sizes()[2];
    let mut ends: BTreeMap<Side, Option<End>> = BTreeMap::new();
    let mut estimates: BTreeMap<Side, Estimate> = BTreeMap::new();
    for side in [ALLIED, JAPANESE] {
        if let Some((intent, e)) = estimate_of_first(&base, side, &figures, full) {
            let _ = writeln!(
                out,
                "== The estimate of the {side:?} side's first intent ({intent}) on (a), full size ({} worlds for {} days)\n{}\n",
                full.1.worlds,
                full.1.days,
                indent(&e.to_string())
            );
            ends.insert(side, e.end.clone());
            estimates.insert(side, e);
        }
    }
    let day = full.1.days;
    for name in ["07a_slice", "07b_slice", "07c_slice"] {
        let watches: Vec<(Side, Option<End>)> = [ALLIED, JAPANESE]
            .into_iter()
            .map(|s| (s, ends.get(&s).cloned().flatten()))
            .collect();
        let reads = par_map(&seeds(games()), |seed| {
            play_read(
                arrangement(name).with_seed(seed).expect("a seed"),
                &watches,
                &[day],
            )
        });
        let _ = writeln!(out, "== The games of {name}, to the end of day {day}");
        for (i, (side, _)) in watches.iter().enumerate() {
            let side_reads: Vec<&Read> = reads.iter().map(|r| &r[i]).collect();
            let met: Vec<u32> = side_reads
                .iter()
                .filter_map(|r| {
                    let (first, kept) = r.met_by(day);
                    first.filter(|_| kept)
                })
                .collect();
            let _ = writeln!(
                out,
                "  {side:?}: end met and kept in {} of {} games; the day first met {:?} (lowest/middle/highest)",
                met.len(),
                side_reads.len(),
                Spread::of(&met).map(spread_words)
            );
            for (c, cname) in COSTS.iter().enumerate() {
                let v: Vec<u32> = side_reads
                    .iter()
                    .map(|r| cost_values(&r.lost(day))[c])
                    .collect();
                let _ = writeln!(
                    out,
                    "    {cname}: {:?} (lowest/middle/highest)",
                    Spread::of(&v).map(spread_words)
                );
            }
            if name == "07a_slice" {
                if let Some(e) = estimates.get(side) {
                    let _ = writeln!(
                        out,
                        "    the estimate: went {:?}; side costs {:?}",
                        e.went, e.side_costs
                    );
                }
            }
        }
        out.push('\n');
    }
    write("condition_12.txt", &out);
}

// ------------------------------------------------------------ condition 13

/// Condition 13, the setup's amendment 5: the Japanese units ashore at the
/// start with their hexes, and whether each is on the land of island 1 or
/// island 2 (the island's land is the hexes of its named places and every hex
/// joined to one by land links, decision 0231, rule 1).
#[test]
#[ignore = "run by hand by the proof's harness"]
fn condition_13_japanese_ashore_at_the_start() {
    let scenario = arrangement("07a_slice");
    let truth = scenario.world.umpire();
    let view = scenario.world.side_view(JAPANESE);
    // The named places of each island, from the scenario's file.
    let text = std::fs::read_to_string(slice_path("07a_slice")).expect("(a) reads");
    let mut islands: BTreeMap<u32, Vec<Hex>> = BTreeMap::new();
    for b in blocks(&text).iter().filter(|b| b.starts_with("[[island]]")) {
        let number: u32 = field(b, "number").unwrap().parse().unwrap();
        let places = field(b, "places").unwrap();
        for part in places.split("base = ").skip(1) {
            let base: u32 = part
                .chars()
                .take_while(char::is_ascii_digit)
                .collect::<String>()
                .parse()
                .unwrap();
            let h = truth.bases[&sim_core::ids::BaseId::new(base)].hex;
            islands.entry(number).or_default().push(h);
        }
    }
    let land: BTreeMap<u32, BTreeSet<Hex>> = islands
        .iter()
        .map(|(n, places)| {
            let mut land = BTreeSet::new();
            let mut next = places.clone();
            while let Some(h) = next.pop() {
                if land.insert(h) {
                    next.extend(view.land_neighbours(h));
                }
            }
            (*n, land)
        })
        .collect();
    let mut out = String::from(
        "Condition 13, amendment 5: the Japanese units ashore at the start of (a), with their hexes.\n\n",
    );
    let mut all_on = true;
    for u in truth
        .land_units
        .values()
        .filter(|u| u.side == JAPANESE && u.men_ashore > 0 && u.hex.is_some())
    {
        let h = u.hex.unwrap();
        let on: Vec<u32> = land
            .iter()
            .filter(|(_, l)| l.contains(&h))
            .map(|(n, _)| *n)
            .collect();
        all_on &= !on.is_empty();
        let _ = writeln!(
            out,
            "unit {} ({}), {} men, hex ({},{}): on the land of island(s) {:?}",
            u.id.raw(),
            u.name,
            u.men_ashore,
            h.q,
            h.r,
            on
        );
    }
    let _ = writeln!(
        out,
        "\nThe islands' land: {}",
        land.iter()
            .map(|(n, l)| format!("island {n}: {} hexes", l.len()))
            .collect::<Vec<_>>()
            .join("; ")
    );
    let _ = writeln!(
        out,
        "Every unit is on the land of an island the side states: {all_on}. {}",
        if all_on {
            "The script's count (every unit of the side ashore at the start) is the reading's."
        } else {
            "The script's count is not the reading's: the reader is to be given the island's land."
        }
    );
    let _ = writeln!(
        out,
        "Allied units ashore at the start: {}",
        truth
            .land_units
            .values()
            .filter(|u| u.side == ALLIED && u.men_ashore > 0 && u.hex.is_some())
            .count()
    );
    write("condition_13_units.txt", &out);
}

// ------------------------------------------------------------ condition 11

/// The legibility files of condition 11: the six estimates as a seat is
/// answered, and the key made from their own fields, written before any
/// reader runs.
#[test]
#[ignore = "run by hand by the proof's harness"]
fn legibility_estimates() {
    let mut answers: Vec<(String, Value)> = Vec::new();

    // 1 and 2: the first intents of (a), the standard size.
    for side in [ALLIED, JAPANESE] {
        let base = arrangement("07a_slice").with_seed(first_seed()).unwrap();
        let figures = slice_figures(&base);
        let stated = day_one(&base, side);
        let mut s = session_of(base, side, &figures);
        queue_as_stated(&mut s, side, &stated);
        let a = tool(
            &mut s,
            side,
            "trial",
            json!({"intent": 1, "size": "standard"}),
        )
        .expect("the trial is answered");
        answers.push((format!("(a), the {side:?} side's first intent"), a));
    }

    // 3 and 4: S1's and W1's, the standard size.
    for (name, file, units) in [
        ("S1", "case_s1_one_unit_against_200", vec![1u32]),
        ("W1", "case_w1_the_garrison_believed_half", vec![1u32]),
    ] {
        let (_, scenario) = case_with(file, "");
        let figures = common::stage_4::figures(&scenario);
        let mut s = session_of(scenario, ALLIED, &figures);
        for (tool_name, args) in [
            (
                "set_intent",
                json!({"intent_kind": "take_place", "hex": [4, 0]}),
            ),
            (
                "set_limit",
                json!({"limit": "forces", "intent": 1, "task_forces": [1], "land_units": units, "source": 1}),
            ),
            ("set_leave", json!({"leave": "launch", "given": true})),
        ] {
            tool(&mut s, ALLIED, tool_name, args).expect("a command is queued");
        }
        let a = tool(
            &mut s,
            ALLIED,
            "trial",
            json!({"intent": 1, "size": "standard"}),
        )
        .expect("the trial is answered");
        answers.push((format!("{name}'s"), a));
    }

    // 5 and 6: a window (scenario 96's worlds that differ, as scenario
    // 100's seat asks it) and a trial beside another (scenario 100's).
    for (name, args) in [
        (
            "a window (scenario 96's, asked as scenario 100's seat asks it)",
            json!({"intent": 1, "dates": "chosen", "size": "standard"}),
        ),
        (
            "a trial beside another (scenario 100's)",
            json!({"intent": 1, "compare": true, "size": "standard"}),
        ),
    ] {
        let path = common::data_dir().join("scenarios/100_what_ifs_from_a_seat.toml");
        let scenario = common::load(&path).expect("scenario 100 loads");
        let figures = scenario.trial_figures().cloned().expect("its figures");
        let mut s = session_of(scenario, ALLIED, &figures);
        for (tool_name, queued) in [
            (
                "set_intent",
                json!({"intent_kind": "take_place", "hex": [4, 0]}),
            ),
            (
                "set_limit",
                json!({"limit": "forces", "intent": 1, "task_forces": [1], "land_units": [1, 2], "source": 1}),
            ),
            ("set_leave", json!({"leave": "launch", "given": true})),
        ] {
            tool(&mut s, ALLIED, tool_name, queued).expect("a command is queued");
        }
        let a = tool(&mut s, ALLIED, "trial", args).expect("the trial is answered");
        answers.push((name.to_string(), a));
    }

    // The files: each estimate as given, and the key from its own fields.
    let mut key = String::from(
        "# The key to the legibility test of stage 4's proof (condition 11)\n\nWritten by the harness from each estimate's own fields, before any reader ran. Five questions of each: what the operation needs, when it could go, in how many of the worlds tried it came off, what it may cost, and what was assumed. An estimate passes when all five answers agree with this key.\n\n",
    );
    for (i, (name, answer)) in answers.iter().enumerate() {
        let n = i + 1;
        write(
            &format!("legibility/estimate{n}.json"),
            &serde_json::to_string_pretty(answer).expect("json"),
        );
        let _ = writeln!(key, "## Estimate {n}: {name}\n");
        for (question, fields) in [
            ("What the operation needs", vec!["needs"]),
            ("When it could go", vec!["asked", "first_day", "last_day"]),
            (
                "In how many of the worlds tried it came off",
                vec!["how_it_went"],
            ),
            ("What it may cost", vec!["cost"]),
            (
                "What was assumed",
                vec!["assumed", "leaves_given_to_the_side"],
            ),
        ] {
            let _ = writeln!(key, "**{question}**\n");
            for f in fields {
                let _ = writeln!(
                    key,
                    "- `{f}`: `{}`",
                    answer
                        .get(f)
                        .map_or("(absent)".to_string(), Value::to_string)
                );
            }
            let _ = writeln!(key);
        }
        if let Some(text) = answer["text"].as_array() {
            let _ = writeln!(key, "The estimate's words, as the seat reads them:\n");
            for line in text {
                let _ = writeln!(key, "> {}", line.as_str().unwrap_or(""));
            }
            let _ = writeln!(key);
        }
    }
    write("legibility/key.md", &key);
}

#[allow(dead_code)]
fn unused(_: &Path) {}
