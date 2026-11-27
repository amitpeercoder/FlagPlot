//! Stage 4's proof, case W3: an enemy intent not as stated (decision 0161,
//! condition 8). A landing is tried against a garrison the side has
//! counted rightly. The side knows of the enemy's second unit and its
//! transports at an anchorage of his own, and believes nothing of what he
//! means to do with them. In every world of the trial they stay where
//! they are. In the game he means to control the island, and when the
//! airfield is taken he lands the second unit on it.
//!
//! The data is placeholder data: see
//! `data/scenarios/stage-4-proof/case_w3_an_intent_not_as_stated.toml`.
//! It runs on the frozen step 1 figures with scenario 68's ground
//! figures, in which a whole shot always hits, and scenario 95's files of
//! the trial's figures and of sides.
//!
//! **Written before the case is built.** Every test below is empty and
//! marked `#[ignore = "statement written, not built"]`. It is a case of a
//! proof: the harness plays it once, for the judged run.
//!
//! # The working
//!
//! Written by the architect before the code. If the code disagrees, the
//! code or the rule is wrong: do not change a number here to make a test
//! pass. A mismatch goes back to the architect.
//!
//! It is a statement of direction, with the least the game must cost
//! worked by hand, and not a full hand calculation of the game: the
//! second fight has part shots, which hit by chance, and its later days
//! are not followed. One game was played by the architect on Nov 24,
//! 2026, on seed 1942, with the course written into a copy of the
//! scenario, to check the days and the least loss below; it agreed. No
//! trial of the case has been played.
//!
//! ## The rules it reads, and their figures
//!
//! Scenario 68's, as scenario 95's hand calculation gives them: a unit
//! fires one shot for each 100 firing men; a whole shot hits and takes 50
//! men; a part of a shot hits by chance; a side is beaten where the other
//! attacked and it has no man left, or fewer than 0.2 of the other's. A
//! size 0 port lands 80 men an hour for each ship. `preparation_per_day`
//! is 100, so a landing is ready the morning after it is ordered. And of
//! the staff: under "control an island" a take is formed for a named
//! place on the first morning the side holds no base there (decision
//! 0151); with no estimate short of his men it lifts the day after it is
//! ready (decisions 0142 and 0149).
//!
//! ## Inputs
//!
//! Scenario 95's, with the garrison at its estimate and the enemy's
//! second force added:
//!
//! | Base | Side | Hex | Supply |
//! | ---- | ---- | --- | ------ |
//! | 1 | Allied | A (0,0) | 1,000 t |
//! | 2 | Japanese | B (4,0) | 200 t |
//! | 3 | Japanese, on the Allied map | C (0,6) | 1,000 t |
//!
//! Unit 1, Allied, 600 men at A, with task force 1, two troop transports.
//! Unit 10, Japanese, 200 men at B. Unit 11, Japanese, 1,200 men at C,
//! with task force 2, two troop transports of 600 men each. C to B is
//! 212 nm, 21.2 hours at 10 kn. Island 1 is base 2.
//!
//! **What the Allied side holds:** an opening estimate of 200 men at B;
//! believed men, 1,200 at base 3, believed present; a believed force of
//! two transports at base 3, believed present; and no believed intent.
//!
//! **What the enemy means, in the game:** on day 1, control island 1,
//! with both leaves. It is in the scenario file. The trial is not told.
//!
//! **The course:** scenario 95's, the Allied side's four commands of day
//! 1: intent 1, take the place at (4,0); its forces limit, task force 1,
//! unit 1 and base 1; both leaves.
//!
//! **The trial:** of the Allied side, from the evening before day 1, 16
//! worlds for 14 days.
//!
//! ## In every world
//!
//! The enemy has base 2 with 200 men, base 3 with a unit of 1,200 men,
//! and a force of two transports at base 3. He is given no intent: he
//! holds what he holds, and nothing of his is ordered on any day. The
//! landing is scenario 95's worlds 1 and 3: the 600 land on day 4, fire
//! 6 hits and destroy the 200, who fire 2 hits; the side loses 100 men
//! and the place is its own at the end of day 4. Nothing lands at B
//! after. So the estimate says: met and kept in 16 of 16 worlds, on day
//! 4; lost again in none; the side's men lost to the enemy 100 at the
//! lowest, the middle and the highest; and, among what was assumed, that
//! the enemy was given no intent.
//!
//! ## In the game
//!
//! Days 1 to 4 are the worlds': the enemy's hold of base 2 leaves the 200
//! on the defence, and the side takes the place on day 4 with 100 men
//! lost and 500 left.
//!
//! | Day | The Japanese staff | What happens |
//! | --- | --- | --- |
//! | 5 | Base 2 is not his: forms a take of the place at B under the island. `prepare_landing` at B by task force 2, roster 11 | Loading and rehearsal 100 |
//! | 6 | Notice: the landing is ready. A load of supply | |
//! | 7 | Unit 11 set to attack. Load unit 11 | 600 men in each ship |
//! | 8 | A route to B and back to C, acting at B, and an unload order | Sails at hour 168 and arrives at hour 189.2, 3.2 hours into the afternoon. Each ship lands 2.8 x 80 = 224 men: 448 ashore. The fight |
//!
//! On day 8 the 448 fire 4 whole shots and a part of one: at least 4
//! hits, 200 men of unit 1. So by the end of day 8 the side has lost at
//! least 100 + 200 = 300 men, whatever the dice of the part shot and of
//! every later day.
//!
//! ## What the condition reads
//!
//! - **The direction.** The estimate's highest for the side's men lost
//!   to the enemy is 100, and it says the place is kept in every world.
//!   In the game the side's men lost by the end of day 8 are at least
//!   300. The estimate is too cheap, by at least 200 men, and it is so in
//!   every game: the mark is read on seeds 1 to 20, and holds in all 20.
//!   Whether the place is lost again by day 14 is reported and judged by
//!   nothing.
//! - **What was assumed.** The estimate's list names the men at base 3
//!   and the transports there as made in every world, and says that the
//!   enemy was given no intent.
#![allow(clippy::doc_lazy_continuation, clippy::doc_markdown)]

use crate::common;

use sim_core::command::NoticeKind;
use sim_core::events::EventKind;
use sim_core::fixed::Fixed;
use sim_core::ids::{BaseId, IntentId, LandUnitId};
use sim_core::state::{Side, TruthState};
use sim_data::Scenario;
use sim_trial::{estimate, AssumedOf, Estimate, Spread};

use common::stage_4::{
    case_text, commands_on, course_of, hex, load_case, notice_days, orders_on, play_game,
    step_trial, world_orders_on, Game, Given, Stepped4,
};

const NAME: &str = "case_w3_an_intent_not_as_stated";
const ALLIED: Side = Side::Allied;
const JAPANESE: Side = Side::Japanese;

fn n(whole: i64) -> Fixed {
    Fixed::from_int(whole)
}

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

/// In every world of the trial: nothing of the enemy's is ordered, the
/// place is taken on day 4 and kept, and the side loses 100 men.
#[test]
fn in_every_world_the_second_unit_stays() {
    let trial = trial();
    assert_eq!(trial.played.worlds.len(), 16);
    for w in &trial.played.worlds {
        assert!(
            w.failed.is_none() && w.refused.is_empty(),
            "world {}",
            w.number
        );
        for day in 1..=14 {
            // Nothing of the enemy's is ordered on any day.
            assert_eq!(
                world_orders_on(w, day, JAPANESE),
                Vec::<Given>::new(),
                "world {} day {day}",
                w.number
            );
            let truth = &trial.snapshot(w.number, day).truth;
            // The second unit and its transports stay where they are: the
            // enemy's men at C and his ships are as they were on day 1. (A
            // trial's world makes the believed men and ships under numbers
            // of its own.)
            let there = |truth: &TruthState| -> Vec<(u32, u32)> {
                truth
                    .land_units
                    .values()
                    .filter(|u| u.side == JAPANESE && u.hex == Some(hex(0, 6)))
                    .map(|u| (u.id.raw(), u.men_ashore))
                    .collect()
            };
            let ships = |truth: &TruthState| -> Vec<(u32, String)> {
                truth
                    .ships
                    .values()
                    .filter(|s| s.side == JAPANESE)
                    .map(|s| (s.id.raw(), format!("{:?}", s.location)))
                    .collect()
            };
            let first = &trial.snapshot(w.number, 1).truth;
            assert_eq!(
                there(first).iter().map(|(_, m)| *m).sum::<u32>(),
                1200,
                "world {}",
                w.number
            );
            assert!(!ships(first).is_empty(), "world {}", w.number);
            assert_eq!(there(truth), there(first), "world {} day {day}", w.number);
            assert_eq!(ships(truth), ships(first), "world {} day {day}", w.number);
        }
        // The landing is scenario 95's worlds 1 and 3: the place is the
        // side's at the end of day 4 and after, with 500 men left.
        assert_eq!(base_side(&trial.snapshot(w.number, 3).truth, 2), JAPANESE);
        for day in 4..=14 {
            let truth = &trial.snapshot(w.number, day).truth;
            assert_eq!(base_side(truth, 2), ALLIED, "world {} day {day}", w.number);
            assert_eq!(unit(truth, 1), 500, "world {} day {day}", w.number);
            assert_eq!(unit(truth, 10), 0, "world {} day {day}", w.number);
        }
        assert_eq!(trial.snapshot(w.number, 14).lost.men_to_enemy, 100);
    }
}

/// The estimate: met and kept in 16 of 16 worlds on day 4, lost again in
/// none, men lost 100 at the lowest, the middle and the highest.
#[test]
fn the_estimate_is_cheap() {
    let e = the_estimate();
    assert_eq!(e.asked, 16);
    assert_eq!(e.last_day, Some(14));
    let all: Vec<u32> = (1..=16).collect();
    assert_eq!(e.went.worlds, all);
    assert_eq!(e.went.met_kept, all);
    assert_eq!(
        e.went.met_days,
        Some(Spread {
            lowest: 4,
            middle: 4,
            highest: 4
        })
    );
    assert!(e.went.lost_again.is_empty());
    assert!(e.went.not_met.is_empty());
    assert!(e.could_not.is_empty());
    let hundred = Spread {
        lowest: 100,
        middle: 100,
        highest: 100,
    };
    assert_eq!(e.side_costs.expect("costs").men_to_enemy, hundred);
}

/// What was assumed names the men and the transports at base 3 as made in
/// every world, and says the enemy was given no intent.
#[test]
fn what_was_assumed_names_the_intent() {
    let e = the_estimate();
    let in_all = |of: &AssumedOf| {
        e.assumed
            .iter()
            .any(|a| &a.of == of && a.made_in.len() == 16 && a.not_in.is_empty())
    };
    // The men at base 3 and the transports there, made in every world.
    assert!(
        e.assumed.iter().any(|a| matches!(&a.of,
            AssumedOf::BelievedMen { hex: h, men: 1200, .. } if *h == hex(0, 6))
            && a.made_in.len() == 16
            && a.not_in.is_empty()),
        "{:?}",
        e.assumed
    );
    assert!(
        e.assumed
            .iter()
            .any(|a| matches!(&a.of, AssumedOf::BelievedForce { .. })
                && a.made_in.len() == 16
                && a.not_in.is_empty()),
        "{:?}",
        e.assumed
    );
    // The enemy was given no intent.
    assert!(in_all(&AssumedOf::EnemyGivenNoIntent), "{:?}", e.assumed);
}

/// The game, seeds 1 to 20: the take is formed on day 5, the landing is
/// ordered on days 5 to 8 as the table has it, 448 men are ashore on the
/// afternoon of day 8, and the side's men lost by the end of day 8 are at
/// least 300.
#[test]
fn in_the_game_he_comes_back() {
    for seed in 1..=20u32 {
        let game = game_on(seed);
        // Days 1 to 4 are the worlds': the place is the side's on day 4
        // with 100 men lost and 500 left.
        let four = game.day(4);
        assert_eq!(base_side(&four.truth, 2), ALLIED, "seed {seed}");
        assert_eq!(unit(&four.truth, 1), 500, "seed {seed}");
        assert_eq!(four.lost.men_to_enemy, 100, "seed {seed}");
        // Day 5: the take is formed and the landing prepared by task force 2.
        assert!(
            commands_on(&game, 5, JAPANESE)
                .iter()
                .any(|c| c.starts_with("FormIntent")),
            "seed {seed}: {:?}",
            commands_on(&game, 5, JAPANESE)
        );
        assert_eq!(
            orders_on(&game, 5, JAPANESE),
            vec![Given::Landing(hex(4, 0), vec![11])],
            "seed {seed}"
        );
        // Day 6: the notice that the landing is ready, and a load of supply.
        assert_eq!(
            notice_days(&game, JAPANESE, NoticeKind::Ready),
            vec![6],
            "seed {seed}"
        );
        let six = orders_on(&game, 6, JAPANESE);
        assert!(
            matches!(six.as_slice(), [Given::LoadTo(_)]),
            "seed {seed}: {six:?}"
        );
        // Day 7: unit 11 set to attack and loaded.
        let mut seven = orders_on(&game, 7, JAPANESE);
        seven.sort_by_key(|g| format!("{g:?}"));
        assert_eq!(
            seven,
            vec![Given::Attack(11), Given::LoadUnit(11)],
            "seed {seed}"
        );
        // Day 8: a route to B and back to C, acting at B, and an unload.
        let mut eight = orders_on(&game, 8, JAPANESE);
        eight.sort_by_key(|g| format!("{g:?}"));
        assert_eq!(
            eight,
            vec![
                Given::Route(vec![hex(4, 0), hex(0, 6)], vec![hex(4, 0)]),
                Given::Unload
            ],
            "seed {seed}"
        );
        // 448 men are ashore on day 8: 2.8 hours x 80 men an hour for each
        // of two ships.
        let landed: u32 = game
            .events
            .iter()
            .filter_map(|e| match &e.kind {
                EventKind::MenLanded { unit, men, .. } if unit.raw() == 11 && e.day == 8 => {
                    Some(*men)
                }
                _ => None,
            })
            .sum();
        assert_eq!(landed, 448, "seed {seed}");
        // The side's men lost by the end of day 8 are at least 300.
        assert!(
            game.day(8).lost.men_to_enemy >= 300,
            "seed {seed}: {}",
            game.day(8).lost.men_to_enemy
        );
        let _ = n(0);
    }
}
