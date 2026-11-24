//! Stage 4's proof, case W1: the garrison believed half the truth (decision
//! 0161, condition 8; `docs/validation/stage-4-proof-setup.md`, condition
//! 8). A landing is tried against a place whose garrison the side puts at
//! 200 and which is truly 400. The estimate is wrong in the direction
//! stated, and its list of what was assumed names the thing that was wrong.
//!
//! The data is placeholder data: see
//! `data/scenarios/stage-4-proof/case_w1_the_garrison_believed_half.toml`.
//! It runs on the frozen step 1 figures with scenario 68's ground figures,
//! in which a whole shot always hits (no die is rolled), and scenario 95's
//! files of the trial's figures and of sides. It is scenario 95's inputs
//! with the garrison at 400 and no believed men.
//!
//! # The statement
//!
//! As the setup page gives it, condition 8, case W1, word for word:
//!
//! > **W1, the garrison believed half the truth.** Scenario 95's inputs
//! > with the garrison truly 400, the opening estimate 200 and nothing
//! > believed beyond it; scenario 68's figures, with no die. *By hand:* in
//! > every world the garrison is 200, the place is taken on day 4 and the
//! > side loses 100 men (scenario 95's worlds 1 and 3). In the game, on day
//! > 4 the 600 fire 6 hits and the 400 fire 4: the garrison loses 300 and
//! > has 100, the side loses 200 and has 400, and 100 is not fewer than 0.2
//! > x 400 = 80, so the garrison is not beaten. On day 5 the side's fit men
//! > are 400 x 50 / 100 = 200, at least the 100 now reported: it attacks,
//! > fires 2 hits and the garrison's last 100 are gone; the garrison fires
//! > 1 hit and the side loses 50. The place is taken on day 5 with 250 men
//! > lost. (The architect played this one game on Nov 24, 2026, to check
//! > the working; it gave these figures.) *The mark:* the estimate says day
//! > 4 and 100 men in every world; the game is a day later and 150 men
//! > dearer; the assumptions name the men at the place as 200 from the
//! > estimate of day 0.
//!
//! The trial is at the standard size, 16 worlds for 14 days; the game is
//! one, seed 1, since no die is rolled in it (the setup's amendment 3). The
//! course is scenario 95's four commands of day 1.
//!
//! The case's file was made from scenario 95's by the developer on Nov 24,
//! 2026 (the garrison at 400, no believed men). If the code disagrees with
//! a number above, the code or the rule is wrong; a mismatch goes back to
//! the architect.
#![allow(clippy::doc_lazy_continuation, clippy::doc_markdown)]

mod common;

use sim_core::ids::{BaseId, IntentId, LandUnitId};
use sim_core::state::{Side, TruthState};
use sim_data::Scenario;
use sim_trial::{estimate, AssumedOf, Estimate, Spread};

use common::stage_4::{
    case_text, course_of, hex, load_case, play_game, step_trial, Game, Stepped4,
};

const NAME: &str = "case_w1_the_garrison_believed_half";
const ALLIED: Side = Side::Allied;
const JAPANESE: Side = Side::Japanese;

/// The course: scenario 95's four commands of day 1, as scenario text.
const COURSE: &str = r#"
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
land_units = [1]
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
"#;

/// The case with the Allied course handed in on day 1, on `seed`.
fn scenario_on(seed: u32) -> Scenario {
    let text = case_text(NAME).replace("seed = 1942", &format!("seed = {seed}"));
    load_case(NAME, &format!("{text}\n{COURSE}"))
}

fn trial() -> &'static Stepped4 {
    static TRIAL: std::sync::OnceLock<Stepped4> = std::sync::OnceLock::new();
    TRIAL.get_or_init(|| {
        let scenario = scenario_on(1942);
        let course = course_of(&scenario, ALLIED);
        assert_eq!(course.len(), 4);
        assert!(course.iter().all(|c| c.day == 1));
        step_trial(&scenario, ALLIED, 1, 16, 14, course)
    })
}

fn the_estimate() -> &'static Estimate {
    static EST: std::sync::OnceLock<Estimate> = std::sync::OnceLock::new();
    EST.get_or_init(|| estimate(&trial().played, IntentId::new(1), None))
}

fn unit(truth: &TruthState, id: u32) -> u32 {
    truth
        .land_units
        .get(&LandUnitId::new(id))
        .map_or(0, |u| u.men_ashore)
}

fn base_side(truth: &TruthState, base: u32) -> Side {
    truth.bases[&BaseId::new(base)].side
}

fn game_on(seed: u32) -> Game {
    play_game(scenario_on(seed), ALLIED)
}

fn spread(v: u32) -> Spread {
    Spread {
        lowest: v,
        middle: v,
        highest: v,
    }
}

/// The worlds: in every world the garrison is 200, the place is taken on
/// day 4 and the side loses 100 men.
#[test]
fn the_worlds_take_the_place_on_day_4_for_100_men() {
    let trial = trial();
    assert_eq!(trial.played.worlds.len(), 16);
    for w in &trial.played.worlds {
        assert!(
            w.failed.is_none() && w.refused.is_empty(),
            "world {}",
            w.number
        );
        let one = &trial.snapshot(w.number, 1).truth;
        assert_eq!(
            unit(one, 900_001),
            200,
            "world {}: the garrison is 200",
            w.number
        );
        assert!(!one.land_units.contains_key(&LandUnitId::new(900_002)));
        let four = &trial.snapshot(w.number, 4);
        assert_eq!(base_side(&four.truth, 2), ALLIED, "world {}", w.number);
        assert_eq!(unit(&four.truth, 1), 500, "world {}", w.number);
        assert_eq!(four.lost.men_to_enemy, 100, "world {}", w.number);
        assert_eq!(base_side(&trial.snapshot(w.number, 3).truth, 2), JAPANESE);
    }
    let e = the_estimate();
    let all: Vec<u32> = (1..=16).collect();
    assert_eq!(e.went.met_kept, all);
    assert_eq!(e.went.met_days, Some(spread(4)));
    assert!(e.went.lost_again.is_empty() && e.went.not_met.is_empty());
    assert_eq!(e.side_costs.expect("costs").men_to_enemy, spread(100));
}

/// The game, seed 1: on day 4 the garrison has 100 and the side 400 and
/// the garrison is not beaten; on day 5 the place is taken with 250 men
/// lost. A day later and 150 men dearer than the estimate.
#[test]
fn the_game_takes_it_on_day_5_for_250_men() {
    let game = game_on(1);
    let four = game.day(4);
    assert_eq!(unit(&four.truth, 10), 100);
    assert_eq!(unit(&four.truth, 1), 400);
    assert_eq!(four.lost.men_to_enemy, 200);
    assert_eq!(
        base_side(&four.truth, 2),
        JAPANESE,
        "the garrison is not beaten"
    );
    let five = game.day(5);
    assert_eq!(unit(&five.truth, 10), 0);
    assert_eq!(unit(&five.truth, 1), 350);
    assert_eq!(five.lost.men_to_enemy, 250);
    assert_eq!(base_side(&five.truth, 2), ALLIED);
    for day in 5..=14 {
        assert_eq!(base_side(&game.day(day).truth, 2), ALLIED, "day {day}");
        assert_eq!(game.day(day).lost.men_to_enemy, 250, "day {day}");
    }
    // The direction: later, and dearer, than the estimate's highest.
    let e = the_estimate();
    let est = e.went.met_days.expect("a day").highest;
    assert_eq!((est, 5), (4, 5));
    assert!(est < 5);
    assert!(five.lost.men_to_enemy > e.side_costs.expect("costs").men_to_enemy.highest);
    assert_eq!(
        five.lost.men_to_enemy - e.side_costs.unwrap().men_to_enemy.highest,
        150
    );
}

/// What was assumed names the men at the place as 200 from the estimate of
/// day 0, in every world, and nothing believed beyond it.
#[test]
fn the_assumptions_name_the_men_at_the_place() {
    let e = the_estimate();
    assert!(
        e.assumed.iter().any(|a| a.of
            == AssumedOf::MenAshore {
                hex: hex(4, 0),
                men: 200
            }
            && a.made_in.len() == 16
            && a.not_in.is_empty()),
        "{:?}",
        e.assumed
    );
    assert!(
        e.assumed
            .iter()
            .all(|a| !matches!(a.of, AssumedOf::BelievedMen { .. })),
        "nothing is believed beyond the estimate: {:?}",
        e.assumed
    );
    let words = e.to_string();
    assert!(words.contains("200"), "{words}");
}
