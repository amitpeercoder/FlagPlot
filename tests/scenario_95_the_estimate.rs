//! Scenario 95: the estimate. A landing of 600 men is tried against a
//! place whose garrison the side puts at 200 and believes may be 600. In
//! four worlds it is taken in the two with 200 and not in the two with
//! 600, and the estimate says so in counts of worlds, with what each
//! cost.
//!
//! Proves decision 0176, step 6 of step 3's stage 4, against a hand
//! calculation. The data is placeholder data: see
//! `data/scenarios/95_the_estimate.toml`, which the build writes to the
//! inputs below. It runs on the frozen step 1 figures with scenario 68's
//! ground figures, in which every shot is a whole shot and hits and a
//! report gives the true count, so no die changes anything.
//!
//! # Hand calculation
//!
//! Written by the architect before the code. Everything is asserted
//! exactly. If the code disagrees, the code or the rule is wrong: do not
//! change a number here to make a test pass. A mismatch goes back to the
//! architect.
//!
//! ## Figures
//!
//! Scenario 68's (see its hand calculation). A unit fires one shot for
//! each 100 firing men; every shot hits and takes 50 men, shared between
//! the units hit in proportion to their men. A unit on attack that fired
//! gains 50 of fatigue; any other loses 25. A side is beaten where the
//! other attacked and it has no man left, or fewer than 0.2 of the
//! other's. A man eats 0.005 t a day. A size 0 port lands 20 t an hour
//! for each ship and a man weighs 0.25 t: 80 men an hour. At balanced
//! the staff wants 2 men for each man of its estimate, and a unit can
//! attack at a fatigue of 40 or less, or if it is on attack already; its
//! fit men are its men times 100 less its fatigue over 100.
//!
//! ## Inputs
//!
//! Scenario 68's map, bases, ships and figures, with these changes:
//!
//! - **One unit.** The Allied side has unit 1 alone, 600 fighting men at
//!   A. Units 2 and 3 are not in the scenario.
//! - **The opening estimate** of the Allied side is 200 enemy men at B.
//! - **Believed men:** the Allied side believes 400 more may be at B,
//!   standing unknown.
//! - **No commands in the scenario.** The course is below.
//! - **The trial's figures:** an enemy base is assumed to hold 200 t of
//!   supply and 0 t of fuel; the seed is 1942. **A file of sides** that
//!   gives every class to both sides.
//! - The Japanese garrison of the live game is as scenario 68 has it,
//!   1,200 men. The trial is not told.
//!
//! **The course:** scenario 68's four commands of day 1, with the forces
//! limit naming unit 1 alone: for the Allied side on day 1, intent 1,
//! take the place at (4,0); its forces limit, task force 1, unit 1 and
//! base 1 as the source; leave to prepare given; leave to launch given.
//! (Amended on review: the inputs first said both that scenario 68's
//! commands were the course and gave another.)
//!
//! **The trial:** of the Allied side, from the evening before day 1,
//! trial 1, 4 worlds for 8 days.
//!
//! ## The worlds
//!
//! In every world the enemy has base 2 at B with 200 t of supply and
//! land unit 900,001, 200 fighting men on defend at B, from the
//! estimate. The 400 believed men are the first thing of unknown
//! standing: they are in worlds 2 and 4, as land unit 900,002, and not
//! in worlds 1 and 3. So the garrison is 200 in worlds 1 and 3 and 600
//! in worlds 2 and 4.
//!
//! ## The choice, and the days to the landing
//!
//! The estimate is 200: the men wanted are 400. Unit 1 takes ship 1:
//! 600 men, enough. The supply is 600 x 0.005 x 15 = 45 t.
//!
//! | Day | The Allied staff | What happens |
//! | --- | --- | --- |
//! | 1 | Approves plan 1 to preparing. Forms intent 2, ready a landing by task force 1 at B. Orders `prepare_landing` at B, roster 1 | Loading 100; unit 1's rehearsal 100 |
//! | 2 | Notice: intent 2 is ready. Approves plan 1 to released. Orders a load of supply to a weight of 45 t | Ship 1 loads 45 t |
//! | 3 | Unit 1 set to attack. Load unit 1 | Ship 1 takes 600 men |
//! | 4 | A route to B and back to A, acting at B, and an unload order | Sails at hour 72 and arrives at hour 88. Ship 1 lands 160 men in the morning's last 2 hours and 440 in the afternoon, and then 10 t. Base 3, "Landing at (4,0)", is formed. The fight |
//!
//! These days are the same in all four worlds: nothing of the enemy is
//! read before the fight but the estimate.
//!
//! ## The fight, day 4
//!
//! | | Worlds 1 and 3 | Worlds 2 and 4 |
//! | --- | --- | --- |
//! | Allied men ashore, on attack | 600 | 600 |
//! | Japanese men ashore, on defend | 200 | 200 and 400 |
//! | Allied shots, all hits | 6 | 6 |
//! | Japanese shots, all hits | 2 | 2 and 4 |
//! | Japanese men lost | 6 x 50 = 300, more than the 200: all 200 | 300: 100 and 200 |
//! | Allied men lost | 2 x 50 = 100 | 6 x 50 = 300 |
//! | Left | 500 against none | 300 against 100 and 200 |
//!
//! - **Worlds 1 and 3.** The Japanese side has no man at B and is
//!   beaten: base 2 is Allied at the end of day 4. The end is met on
//!   day 4. On the morning of day 5 the plan is marked complete, and
//!   the staff orders nothing else: unit 1 stays as it is, and with no
//!   enemy in the hex there is no fight. No more men are lost on days 5
//!   to 8. Six staff orders in all. (Amended on review: this said unit 1
//!   is set to defend. A take that is met orders nothing more, as
//!   scenario 68's own case shows.)
//! - **Worlds 2 and 4.** The Japanese side has 300 men against
//!   0.2 x 300 = 60: it is not beaten. Unit 1 has 300 men at a fatigue
//!   of 50. That evening's report gives 300 enemy men. On the morning of
//!   day 5 the men wanted are 600 and unit 1's fit men are 150: the
//!   staff sets it to defend and waits. Its fit men are 225 on the
//!   morning of day 6 and 300 from day 7, against 600 wanted: it waits
//!   on every morning to day 8, and there is no unit to bring. No one
//!   attacks, so there is no fight after day 4 and no more men are
//!   lost. Base 2 is Japanese at the end of day 8: the end is not met.
//!   Seven staff orders in all, the seventh unit 1 set to defend on day
//!   5. The weights of fire on day 4 are equal, 6 against 6: the attack
//!   is not below the defence, so it does not fail.
//!
//! Every man of both sides is fed on every night. The Japanese are fed
//! from base 2's 200 t. The Allied men ashore are fed from base 3 in
//! worlds 2 and 4; in worlds 1 and 3 from base 2, the lower numbered,
//! from the night of day 5: it changed side with (200 - 4 x 1) / 2 =
//! 98 t. (Amended on review: this said base 3 in every world.)
//!
//! ## The estimate of intent 1
//!
//! - **How it went.** 4 worlds played. Met and kept in 2, worlds 1 and
//!   3: the earliest day 4, the middle day 4, the latest day 4. Met and
//!   lost again in none. Not met by day 8 in 2, worlds 2 and 4. None
//!   could not be played.
//! - **What it cost the side,** lowest, middle, highest. Men lost to the
//!   enemy: the worlds give 100, 300, 100, 300; in order 100, 100, 300,
//!   300; the middle is the value at place 2: 100, 100, 300. Men lost
//!   unfed: 0, 0, 0. Ships sunk: 0, 0, 0. Ships out of action: 0, 0, 0.
//!   Aircraft: 0, 0, 0.
//! - **What the trial's enemy lost.** Men lost to the enemy: the worlds
//!   give 200, 300, 200, 300: 200, 200, 300. The rest: 0.
//! - **What it needs and has,** from world 1's plan order at the end of
//!   day 1: men wanted 400, to hand 600; it waits for nothing; the day
//!   expected ashore is day 4. The plan order is the same in all four
//!   worlds.
//! - **What was assumed,** each with its count of worlds: base 2 with
//!   200 t of supply and 0 t of fuel, in 4; 200 men at B from the
//!   side's estimate, in 4; 400 men believed at B, standing unknown, in
//!   2 and not in 2, one line; the enemy given no belief, in 4; the
//!   enemy given no intent, in 4; and leave to launch, given by the
//!   course and not by the side. (Amended on review: the leave was left
//!   out.)
//! - **Where the worlds part,** by the 400 men believed at B, the only
//!   parting. With them, worlds 2 and 4: 2 worlds; met and kept in 0,
//!   lost again in 0, not met in 2; the side's men lost to the enemy
//!   300, 300, 300; the enemy's 300, 300, 300; every other cost of
//!   either side 0, 0, 0. Without them, worlds 1 and 3: 2 worlds; met
//!   and kept in 2, on day 4, 4 and 4; the side's men lost to the enemy
//!   100, 100, 100; the enemy's 200, 200, 200; every other cost 0, 0, 0.
//! - **Each world's own figures:** world 1, met on day 4, not lost,
//!   the side's men lost to the enemy 100, the enemy's 200; world 2,
//!   not met, 300 and 300; world 3 as world 1; world 4 as world 2.
//! - **Its other plans:** the side has none, and no trial of things as
//!   they stand is given.
//!
//! Its words name the counts of worlds ("2 of 4 worlds") and hold no
//! per cent sign and neither the word "chance" nor "points". The
//! figures are asserted on the estimate's data, not on its words.
//!
//! # Properties
//!
//! - **Things as they stand.** A second trial of the same four worlds
//!   with an empty course: nothing is ordered, no man is lost, and base
//!   2 is Japanese in all four. Given to the estimate of intent 1 to
//!   compare with, it changes nothing above: the side has no other plan.
//! - **A plan displaced.** With a second Allied unit of 600 men at A and
//!   a second intent already stated in the live game on day 1 and its
//!   leaves given, to take the place at base 2 with that unit and task
//!   force 1; and the course a third intent that takes task force 1 for
//!   unit 1 at once: reported by the build with its figures for the
//!   architect to state, and not asserted here. (The numbers of two
//!   takes that share a force are the staff's priority's, decision 0111,
//!   and are worked when the build shows what it gives.)
//!
//!   Stated by the architect on Nov 19, 2026, from the build's run: the
//!   live game played to the evening of day 1 with intent 2 standing,
//!   and the course intent 10, take the place at (4,0) with task force 1
//!   and unit 1; a trial of 4 worlds for 8 days and a second of the same
//!   worlds with an empty course. Without the course unit 2 lands on
//!   day 4; with it unit 1 lands on day 4 and unit 2 stays at A. Intent
//!   2 is met and kept in worlds 1 and 3 with a middle day of 4, with
//!   the course and without: it is not named as displaced. Both intents
//!   end at the same place, so the course changes which unit lands and
//!   costs the other plan nothing an estimate reads. A displacement by
//!   two takes of different places is left for the what-ifs of the next
//!   step.
//! - **Believed present, and absent.** With the 400 men's standing
//!   present: the garrison is 600 in all four worlds, the end is met in
//!   none, and the estimate shows no parting. With it absent: 200 in all
//!   four, met in 4 on day 4, no parting.
//! - **Met and lost again.** Not reached by this scenario: the Japanese
//!   side is given no intent and never attacks. It is tested inside the
//!   crate on a world's record made by the test: a place that is the
//!   side's at the end of days 3 to 5 and not from day 6 to the end is
//!   met on day 3 and lost again, and not met and kept. A place that is
//!   the side's on days 3 to 5, not on day 6 and again from day 7 is met
//!   on day 3 and kept.
//! - **A place with no enemy base is met by a landing.** (Added on
//!   review.) Inside the crate, on a record made by the test: a hex in
//!   which the enemy has no base and the side first has one at the end
//!   of day 2 is met on day 2.
//! - **A plan displaced, by counting.** (Added on review.) Inside the
//!   crate, on records made by the test: another intent met and kept in
//!   4 worlds of 4 without the course and in 2 with it is named as
//!   displaced, 4 and 2; one met in 4 and 4 with middle days 5 and 7 is
//!   named as displaced by its days; one met in 4 and 4 on the same
//!   days is not named.
//! - **Believed men are set aside.** (Added on review.) With the Allied
//!   side's estimate of B of a day after the start, a picture taken on
//!   an evening after a report: the 400 are made in no world, and the
//!   assumptions say the side has a count since. The case is made from
//!   the Allied side's view of world 2 of the main trial at the end of
//!   its play, which holds a count of 300 men of day 8: asked of that
//!   view, the 400 are made in no world. (Corrected by the architect on
//!   Nov 19, 2026, after the build: this said to play the live game to
//!   day 5 with the course. There the landing meets the live garrison
//!   of 1,200, unit 1 is destroyed on day 4, and with no man of the
//!   side at B no report is made.)
//! - **No world played.** (Added on review.) With every picture made
//!   unsound by the test: the estimate says that none of 4 worlds could
//!   be played, and gives no day and no cost.
//! - **An intent with no end.** The estimate of an intent to keep a base
//!   supplied gives the costs and says that it has no end an estimate
//!   can read.
//! - **A world that could not be played** is counted apart: with world
//!   2's picture made unsound by the test, the estimate has 3 worlds
//!   played and 1 that could not be, and its counts are of the 3.
//! - **Believed men are loaded as a believed force is,** with a name,
//!   and refused for the same faults: no man; an unknown standing; a
//!   base that is the believing side's own, or not on its map; a hex
//!   off the map, or that is sea; both a base and a hex, or neither; an
//!   unknown key.
//! - **The same twice,** and the live game untouched: its hash before
//!   and after the trials and the estimates is the same.
//! - **`sim-trial` reads a world only where it makes an estimate.** Its
//!   source test passes with the one allowance.
//! - **No live game changes** by the build: no baseline, golden file or
//!   pinned hash moves.

// The tests are written by the build, to the statements above.

mod common;

use std::path::PathBuf;
use std::sync::OnceLock;

use sim_core::believed::Standing;
use sim_core::command::{CommandKind, IntentKind, Leave, MenCount, StaffOrder};
use sim_core::events::EventKind;
use sim_core::fixed::Fixed;
use sim_core::hex::Hex;
use sim_core::ids::{BaseId, IntentId, LandUnitId};
use sim_core::orders::{Cargo, LandUnitOrder, OrderKind, Posture, TaskForceOrder};
use sim_core::refdata::TrialFigures;
use sim_core::state::{Side, TruthState};
use sim_core::world_picture::PictureCounters;
use sim_data::Scenario;
use sim_staff::{as_staffs, staffs};
use sim_trial::{
    begin, begin_from_pictures, estimate, pictures, Asked, Assumed, AssumedOf, Assumption,
    Believed, Costs, CourseCommand, End, Estimate, Kept, Losses, OtherPlan, PartSide, PartedBy,
    Parting, Played, PlayedWorld, SetAside, Spread, Stepped, Trial, Went,
};

const ALLIED: Side = Side::Allied;
const JAPANESE: Side = Side::Japanese;
const A: Hex = Hex { q: 0, r: 0 };
const B: Hex = Hex { q: 4, r: 0 };
const BELIEVED: &str = "Reinforcement believed at B";

/// The course: scenario 68's four commands of day 1, the forces limit
/// naming unit 1 alone, as scenario text, read by the loader.
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

fn path() -> PathBuf {
    common::data_dir().join("scenarios/95_the_estimate.toml")
}

fn text() -> String {
    std::fs::read_to_string(path()).expect("the scenario file reads")
}

fn n(whole: i64) -> Fixed {
    Fixed::from_int(whole)
}

fn load(text: &str) -> Scenario {
    common::from_text(text, &path()).expect("the scenario loads")
}

fn scenario() -> Scenario {
    load(&text())
}

/// The loader's words for a text that is refused.
fn refused(text: &str) -> String {
    common::from_text(text, &path())
        .expect_err("the scenario is refused")
        .to_string()
}

/// The commands of `extra`, scenario text of commands added to scenario 95,
/// as the loader reads them: the Allied side's of day 1, as a course.
fn course_of(extra: &str) -> Vec<CourseCommand> {
    let with = load(&format!("{}\n{extra}", text()));
    with.scripted_commands()
        .get(&1)
        .map(|day| {
            day.iter()
                .filter(|(side, _)| *side == ALLIED)
                .map(|(_, command)| CourseCommand::first_day(command.clone()))
                .collect()
        })
        .unwrap_or_default()
}

fn course() -> Vec<CourseCommand> {
    course_of(COURSE)
}

fn believed(scenario: &Scenario) -> Believed {
    Believed {
        forces: scenario.believed_forces(ALLIED).cloned().collect(),
        intents: scenario.believed_intents(ALLIED).cloned().collect(),
        men: scenario.believed_men(ALLIED).cloned().collect(),
    }
}

fn figures(scenario: &Scenario) -> TrialFigures {
    scenario
        .trial_figures()
        .cloned()
        .expect("scenario 95 names its trial's figures")
}

fn asked(course: Vec<CourseCommand>) -> Asked {
    Asked {
        trial: 1,
        worlds: 4,
        days: 8,
        course,
    }
}

/// The trial of the Allied side on the live game of `scenario` as it stands:
/// trial 1, 4 worlds for 8 days.
fn trial(scenario: &Scenario, course: Vec<CourseCommand>) -> Played {
    begin(
        &scenario.world.side_view(ALLIED),
        &believed(scenario),
        &figures(scenario),
        scenario.world.ref_data(),
        scenario.world.map(),
        asked(course),
    )
    .expect("the trial begins")
    .play_out()
}

/// What a day of a world left, read by the umpire of the test.
struct Snapshot {
    world: u32,
    day: u32,
    truth: TruthState,
    /// The Allied side's estimate of the men at B.
    estimate_at_b: Option<u32>,
}

/// The main case: the trial stepped to its end, with each day's truth.
struct Main {
    played: Played,
    days: Vec<Snapshot>,
}

fn main_case() -> &'static Main {
    static MAIN: OnceLock<Main> = OnceLock::new();
    MAIN.get_or_init(|| {
        let live = scenario();
        let mut playing = begin(
            &live.world.side_view(ALLIED),
            &believed(&live),
            &figures(&live),
            live.world.ref_data(),
            live.world.map(),
            asked(course()),
        )
        .expect("the trial begins");
        let mut days = Vec::new();
        while let Some(stepped) = playing.step() {
            let Stepped::Day { world, day, .. } = stepped else {
                panic!("a world could not be played: {stepped:?}");
            };
            let current = playing
                .in_play()
                .filter(|w| w.number == world)
                .or_else(|| playing.played().last())
                .expect("the world just played");
            let built = current.world.as_ref().expect("built");
            days.push(Snapshot {
                world,
                day,
                truth: built.umpire().clone(),
                estimate_at_b: built.side_view(ALLIED).ground_estimate(B).map(|e| e.men),
            });
        }
        Main {
            played: playing.play_out(),
            days,
        }
    })
}

fn main_estimate() -> &'static Estimate {
    static EST: OnceLock<Estimate> = OnceLock::new();
    EST.get_or_init(|| estimate(&main_case().played, IntentId::new(1), None))
}

fn snapshot(world: u32, day: u32) -> &'static TruthState {
    &main_case()
        .days
        .iter()
        .find(|s| s.world == world && s.day == day)
        .expect("a day played")
        .truth
}

fn world_of(played: &PlayedWorld) -> &sim_core::World {
    played.world.as_ref().expect("the world was built")
}

fn spread(lowest: u32, middle: u32, highest: u32) -> Spread {
    Spread {
        lowest,
        middle,
        highest,
    }
}

fn zero() -> Spread {
    spread(0, 0, 0)
}

/// The five costs, the men lost to the enemy as given and every other 0.
fn costs(men: Spread) -> Costs {
    Costs {
        men_to_enemy: men,
        men_unfed: zero(),
        ships_sunk: zero(),
        ships_out_of_action: zero(),
        aircraft: zero(),
    }
}

fn men_lost(men: u32) -> Losses {
    Losses {
        men_to_enemy: men,
        ..Losses::default()
    }
}

/// An order of the staff, plainly.
#[derive(Clone, PartialEq, Eq, Debug)]
enum Given {
    Landing(Hex, Vec<u32>),
    LoadTo(Fixed),
    Attack(u32),
    Defend(u32),
    LoadUnit(u32),
    Route(Vec<Hex>, Vec<Hex>),
    Unload,
    Other(String),
}

fn given(o: &StaffOrder) -> Given {
    match &o.kind {
        OrderKind::TaskForce { order, .. } => match order {
            TaskForceOrder::PrepareLanding { hex, roster } => {
                Given::Landing(*hex, roster.iter().map(|u| u.raw()).collect())
            }
            TaskForceOrder::LoadTo {
                cargo: Cargo::Supply,
                weight_t,
                ..
            } => Given::LoadTo(*weight_t),
            TaskForceOrder::LoadUnit { unit, cargo: None } => Given::LoadUnit(unit.raw()),
            TaskForceOrder::SetRoute {
                waypoints,
                act_at,
                stand_at,
            } if stand_at.is_empty() => {
                Given::Route(waypoints.clone(), act_at.iter().copied().collect())
            }
            TaskForceOrder::Unload => Given::Unload,
            other => Given::Other(format!("{other:?}")),
        },
        OrderKind::LandUnit {
            target,
            order:
                LandUnitOrder::SetPosture {
                    posture,
                    when_spent: None,
                },
        } => match posture {
            Posture::Attack => Given::Attack(target.raw()),
            Posture::Defend => Given::Defend(target.raw()),
        },
        other => Given::Other(format!("{other:?}")),
    }
}

/// The Allied staff's orders of each day of a world.
fn orders(world: &PlayedWorld) -> Vec<(u32, Vec<Given>)> {
    world
        .days
        .iter()
        .map(|d| {
            (
                d.day,
                d.staff
                    .get(&ALLIED)
                    .map(|s| s.orders.iter().map(given).collect())
                    .unwrap_or_default(),
            )
        })
        .collect()
}

fn base_side(truth: &TruthState, base: u32) -> Side {
    truth.bases[&BaseId::new(base)].side
}

/// The men ashore of the land unit `raw`, 0 where it is gone.
fn men_of(truth: &TruthState, raw: u32) -> u32 {
    truth
        .land_units
        .get(&LandUnitId::new(raw))
        .map_or(0, |u| u.men_ashore)
}

// ---------------------------------------------------------------- The inputs

#[test]
fn the_inputs_are_scenario_68_with_its_changes() {
    let t = text();
    assert!(!t.contains("[[command]]"));
    assert!(!t.contains("[[order]]"));
    assert!(t.contains("placeholder = true"));
    let s = scenario();
    let truth = s.world.umpire();
    let allied: Vec<(u32, u32, Option<Hex>)> = truth
        .land_units
        .values()
        .filter(|u| u.side == ALLIED)
        .map(|u| (u.id.raw(), u.men_ashore, u.hex))
        .collect();
    assert_eq!(allied, vec![(1, 600, Some(A))]);
    assert_eq!(men_of(truth, 10), 1200, "the live garrison is 1,200");
    assert_eq!(base_side(truth, 2), JAPANESE);
    assert_eq!(truth.bases[&BaseId::new(2)].supply_t, n(200));
    let view = s.world.side_view(ALLIED);
    let estimates: Vec<(Hex, u32, u32)> = view
        .ground_estimates()
        .map(|e| (e.hex, e.men, e.day))
        .collect();
    assert_eq!(estimates, vec![(B, 200, 0)]);
    let men: Vec<_> = s.believed_men(ALLIED).collect();
    assert_eq!(men.len(), 1);
    assert_eq!(men[0].name, BELIEVED);
    assert_eq!(men[0].hex, B);
    assert_eq!(men[0].base, Some(BaseId::new(2)));
    assert_eq!(men[0].men, 400);
    assert_eq!(men[0].standing, Standing::Unknown);
    assert!(men[0].placeholder);
    assert_eq!(s.believed_men(JAPANESE).count(), 0);
    let f = figures(&s);
    assert!(f.placeholder);
    assert_eq!(f.enemy_base_supply_t, n(200));
    assert_eq!(f.enemy_base_fuel_t, n(0));
    assert_eq!(f.seed, 1942);
    // A file of sides that gives every class and type to both sides.
    for class in view.ship_classes() {
        let whose = view
            .ship_class_side(class.id)
            .expect("a side for each class");
        assert!(whose.used_by(ALLIED) && whose.used_by(JAPANESE));
    }
    for aircraft in view.aircraft_types() {
        let whose = view
            .aircraft_type_side(aircraft.id)
            .expect("a side for each type");
        assert!(whose.used_by(ALLIED) && whose.used_by(JAPANESE));
    }
    // The course: scenario 68's four commands, unit 1 alone.
    let c = course();
    assert_eq!(c.len(), 4);
    assert!(c.iter().all(|c| c.day == 1));
    assert_eq!(
        c[0].command,
        CommandKind::SetIntent {
            intent: IntentId::new(1),
            kind: IntentKind::TakePlace {
                hex: B,
                by_day: None
            }
        }
    );
    let CommandKind::SetLimit(sim_core::command::Limit::Forces { intent, limit }) = &c[1].command
    else {
        panic!("the second command is the forces limit");
    };
    assert_eq!(*intent, IntentId::new(1));
    assert_eq!(
        limit
            .task_forces
            .iter()
            .map(|t| t.raw())
            .collect::<Vec<_>>(),
        vec![1]
    );
    assert_eq!(
        limit.land_units.iter().map(|u| u.raw()).collect::<Vec<_>>(),
        vec![1]
    );
    assert_eq!(limit.source, Some(BaseId::new(1)));
    assert_eq!(
        c[2].command,
        CommandKind::SetLeave {
            leave: Leave::Prepare,
            given: true
        }
    );
    assert_eq!(
        c[3].command,
        CommandKind::SetLeave {
            leave: Leave::Launch,
            given: true
        }
    );
}

// ---------------------------------------------------------------- The worlds

#[test]
fn the_worlds_hold_200_men_and_the_believed_400_in_worlds_2_and_4() {
    let main = main_case();
    assert_eq!(main.played.worlds.len(), 4);
    for w in &main.played.worlds {
        assert!(w.failed.is_none(), "world {}", w.number);
        assert!(w.refused.is_empty(), "world {}", w.number);
        assert_eq!(w.days.len(), 8);
        let made_400 = w.assumptions.iter().any(|a| {
            matches!(a, Assumption::BelievedMenMade { unit, men: 400, hex, standing: Standing::Unknown, .. }
                if unit.raw() == 900_002 && *hex == B)
        });
        let not_400 = w.assumptions.iter().any(|a| {
            matches!(a, Assumption::BelievedMenNotInWorld { place: 1, men: 400, hex, .. } if *hex == B)
        });
        let with = w.number % 2 == 0;
        assert_eq!(made_400, with, "world {}", w.number);
        assert_eq!(not_400, !with, "world {}", w.number);
        assert!(w.assumptions.iter().any(|a| matches!(a,
            Assumption::MenAshore { unit, hex, men: 200 } if unit.raw() == 900_001 && *hex == B)));
        assert!(w.assumptions.iter().any(|a| matches!(a,
            Assumption::EnemyBase { base, hex, supply_t, fuel_t }
                if base.raw() == 2 && *hex == B && *supply_t == n(200) && *fuel_t == n(0))));
    }
    // The garrison in truth at the end of the first day: 200, and 400 more
    // in worlds 2 and 4, each on defend at B.
    for world in 1..=4 {
        let truth = snapshot(world, 1);
        let japanese: Vec<(u32, u32, Option<Hex>, Posture)> = truth
            .land_units
            .values()
            .filter(|u| u.side == JAPANESE)
            .map(|u| (u.id.raw(), u.men_ashore, u.hex, u.posture))
            .collect();
        let mut want = vec![(900_001, 200, Some(B), Posture::Defend)];
        if world % 2 == 0 {
            want.push((900_002, 400, Some(B), Posture::Defend));
        }
        assert_eq!(japanese, want, "world {world}");
        assert_eq!(base_side(truth, 2), JAPANESE);
    }
}

// ---------------------------------------------------------------- The days to the landing

fn days_to_the_landing() -> Vec<(u32, Vec<Given>)> {
    vec![
        (1, vec![Given::Landing(B, vec![1])]),
        (2, vec![Given::LoadTo(n(45))]),
        (3, vec![Given::Attack(1), Given::LoadUnit(1)]),
        (4, vec![Given::Route(vec![B, A], vec![B]), Given::Unload]),
    ]
}

#[test]
fn the_staff_works_the_same_four_days_in_every_world() {
    for w in &main_case().played.worlds {
        let mut got = orders(w);
        for (_, day) in &mut got {
            day.sort_by_key(|g| format!("{g:?}"));
        }
        let mut want = days_to_the_landing();
        for (_, day) in &mut want {
            day.sort_by_key(|g| format!("{g:?}"));
        }
        assert_eq!(&got[..4], &want[..], "world {}", w.number);
        // Day 1: plan 1 approved to preparing, intent 2 formed; day 2:
        // approved to released.
        let commands = |day: usize| -> Vec<String> {
            w.days[day]
                .staff
                .get(&ALLIED)
                .map(|s| s.commands.iter().map(|c| format!("{:?}", c.kind)).collect())
                .unwrap_or_default()
        };
        let one = commands(0);
        assert!(
            one.contains(&"Approve { plan: PlanId(1) }".to_string()),
            "{one:?}"
        );
        assert!(one.iter().any(|c| c.starts_with(
            "FormIntent { parent: IntentId(1), kind: ReadyLanding { force: TaskForceId(1), hex: Hex { q: 4, r: 0 }"
        )), "{one:?}");
        assert_eq!(commands(1), vec!["Approve { plan: PlanId(1) }".to_string()]);
        // Base 3, "Landing at (4,0)", is formed on day 4.
        let built = world_of(w);
        let formed: Vec<(u32, u32, Hex)> = built
            .events()
            .iter()
            .filter_map(|e| match &e.kind {
                EventKind::BaseFormed {
                    base, side, hex, ..
                } if *side == ALLIED => Some((e.day, base.raw(), *hex)),
                _ => None,
            })
            .collect();
        assert_eq!(formed, vec![(4, 3, B)], "world {}", w.number);
        assert_eq!(
            built.umpire().bases[&BaseId::new(3)].name,
            "Landing at (4,0)"
        );
        // Ship 1 loads 45 t and takes 600 men; 160 men land in the morning
        // and 440 in the afternoon.
        let landed: Vec<(sim_core::time::Pulse, u32)> = built
            .events()
            .iter()
            .filter_map(|e| match &e.kind {
                EventKind::MenLanded { unit, men, .. } if unit.raw() == 1 => {
                    Some((e.pulse.expect("a pulse"), *men))
                }
                _ => None,
            })
            .collect();
        assert_eq!(
            landed,
            vec![
                (sim_core::time::Pulse::Morning, 160),
                (sim_core::time::Pulse::Afternoon, 440)
            ],
            "world {}",
            w.number
        );
    }
}

// ---------------------------------------------------------------- The fight

#[test]
fn the_fight_of_day_4_takes_the_place_in_worlds_1_and_3() {
    for world in [1, 3] {
        let truth = snapshot(world, 4);
        assert_eq!(men_of(truth, 1), 500, "world {world}");
        assert_eq!(men_of(truth, 900_001), 0, "world {world}");
        assert_eq!(base_side(truth, 2), ALLIED, "world {world}");
        // It changed side with (200 - 4 x 1) / 2 = 98 t.
        assert_eq!(
            truth.bases[&BaseId::new(2)].supply_t,
            n(98),
            "world {world}"
        );
        for day in 5..=8 {
            let truth = snapshot(world, day);
            assert_eq!(men_of(truth, 1), 500, "world {world} day {day}");
            assert_eq!(base_side(truth, 2), ALLIED);
        }
        // The plan is marked complete on the morning of day 5, and nothing
        // else is ordered: six staff orders in all.
        let w = &main_case().played.worlds[world as usize - 1];
        let all: Vec<Given> = orders(w).into_iter().flat_map(|(_, o)| o).collect();
        assert_eq!(all.len(), 6, "world {world}: {all:?}");
        let five: Vec<String> = w.days[4]
            .staff
            .get(&ALLIED)
            .map(|s| s.commands.iter().map(|c| format!("{:?}", c.kind)).collect())
            .unwrap_or_default();
        assert_eq!(
            five,
            vec!["MarkPlan { plan: PlanId(1), outcome: Complete }".to_string()]
        );
        assert!(w.days[4..]
            .iter()
            .all(|d| d.staff.get(&ALLIED).is_none_or(|s| s.orders.is_empty())));
    }
}

#[test]
fn the_fight_of_day_4_does_not_take_the_place_in_worlds_2_and_4() {
    for world in [2, 4] {
        let truth = snapshot(world, 4);
        assert_eq!(men_of(truth, 1), 300, "world {world}");
        assert_eq!(men_of(truth, 900_001), 100, "world {world}");
        assert_eq!(men_of(truth, 900_002), 200, "world {world}");
        assert_eq!(base_side(truth, 2), JAPANESE);
        // Unit 1's fatigue: 50 after the attack, then 25 and 0: fit men 150,
        // 225 and 300 on the mornings of days 5, 6 and 7.
        let fatigue = |day: u32| snapshot(world, day).land_units[&LandUnitId::new(1)].fatigue;
        assert_eq!(fatigue(4), n(50));
        assert_eq!(fatigue(5), n(25));
        assert_eq!(fatigue(6), n(0));
        // That evening's report gives 300 enemy men.
        let report = main_case()
            .days
            .iter()
            .find(|s| s.world == world && s.day == 4)
            .and_then(|s| s.estimate_at_b);
        assert_eq!(report, Some(300));
        for day in 5..=8 {
            let truth = snapshot(world, day);
            assert_eq!(men_of(truth, 1), 300, "world {world} day {day}");
            assert_eq!(men_of(truth, 900_001), 100);
            assert_eq!(men_of(truth, 900_002), 200);
            assert_eq!(base_side(truth, 2), JAPANESE);
            assert_eq!(
                truth.land_units[&LandUnitId::new(1)].posture,
                Posture::Defend
            );
        }
        // Seven staff orders, the seventh unit 1 set to defend on day 5.
        let w = &main_case().played.worlds[world as usize - 1];
        let by_day = orders(w);
        let all: Vec<Given> = by_day.iter().flat_map(|(_, o)| o.clone()).collect();
        assert_eq!(all.len(), 7, "world {world}: {all:?}");
        assert_eq!(by_day[4], (5, vec![Given::Defend(1)]));
        // The attack did not fail: the weights of fire were equal.
        assert!(!world_of(w)
            .events()
            .iter()
            .any(|e| matches!(e.kind, EventKind::GroundAttackFailed { .. })));
    }
}

#[test]
fn every_man_is_fed_on_every_night() {
    for w in &main_case().played.worlds {
        let fed: Vec<(u32, Side, Option<u32>, u32)> = world_of(w)
            .events()
            .iter()
            .filter_map(|e| match &e.kind {
                EventKind::MenFed {
                    side,
                    hex,
                    base,
                    unfed,
                    ..
                } if *hex == B => {
                    assert_eq!(*unfed, 0, "world {} day {}", w.number, e.day);
                    Some((e.day, *side, base.map(|b| b.raw()), *unfed))
                }
                _ => None,
            })
            .collect();
        assert!(!world_of(w)
            .events()
            .iter()
            .any(|e| matches!(e.kind, EventKind::MenLostUnfed { .. })));
        // The Japanese at B from base 2 while it is theirs.
        assert!(fed
            .iter()
            .filter(|(_, side, _, _)| *side == JAPANESE)
            .all(|(_, _, base, _)| *base == Some(2)));
        let allied: Vec<(u32, Option<u32>)> = fed
            .iter()
            .filter(|(_, side, _, _)| *side == ALLIED)
            .map(|(day, _, base, _)| (*day, *base))
            .collect();
        let want: Vec<(u32, Option<u32>)> = if w.number % 2 == 1 {
            // From base 2, the lower numbered, from the night of day 5.
            (5..=8).map(|day| (day, Some(2))).collect()
        } else {
            (5..=8).map(|day| (day, Some(3))).collect()
        };
        assert_eq!(allied, want, "world {}", w.number);
    }
}

// ---------------------------------------------------------------- The estimate

fn assumed_main() -> Vec<Assumed> {
    vec![
        Assumed {
            of: AssumedOf::EnemyBase {
                base: BaseId::new(2),
                hex: B,
                supply_t: n(200),
                fuel_t: n(0),
            },
            made_in: vec![1, 2, 3, 4],
            not_in: vec![],
            notes: vec![],
        },
        Assumed {
            of: AssumedOf::MenAshore { hex: B, men: 200 },
            made_in: vec![1, 2, 3, 4],
            not_in: vec![],
            notes: vec![],
        },
        Assumed {
            of: AssumedOf::BelievedMen {
                place: 1,
                name: BELIEVED.to_string(),
                hex: B,
                men: 400,
                standing: Standing::Unknown,
            },
            made_in: vec![2, 4],
            not_in: vec![1, 3],
            notes: vec![],
        },
        Assumed {
            of: AssumedOf::EnemyGivenNoBelief,
            made_in: vec![1, 2, 3, 4],
            not_in: vec![],
            notes: vec![],
        },
        Assumed {
            of: AssumedOf::EnemyGivenNoIntent,
            made_in: vec![1, 2, 3, 4],
            not_in: vec![],
            notes: vec![],
        },
    ]
}

#[test]
fn how_it_went() {
    let e = main_estimate();
    assert_eq!(e.side, Some(ALLIED));
    assert_eq!(e.intent, IntentId::new(1));
    assert_eq!(
        e.kind,
        Some(IntentKind::TakePlace {
            hex: B,
            by_day: None
        })
    );
    assert_eq!(e.end, Some(End { places: vec![B] }));
    assert_eq!(e.asked, 4);
    assert_eq!(e.last_day, Some(8));
    assert_eq!(
        e.went,
        Went {
            worlds: vec![1, 2, 3, 4],
            met_kept: vec![1, 3],
            met_days: Some(spread(4, 4, 4)),
            lost_again: vec![],
            not_met: vec![2, 4],
        }
    );
    assert!(e.could_not.is_empty());
    assert!(!e.uneven);
}

#[test]
fn what_it_cost_each_side() {
    let e = main_estimate();
    assert_eq!(e.side_costs, Some(costs(spread(100, 100, 300))));
    assert_eq!(e.enemy_costs, Some(costs(spread(200, 200, 300))));
}

#[test]
fn what_it_needs_and_has() {
    let e = main_estimate();
    assert_eq!(e.plan_order_world, Some(1));
    let order = e.plan_order.as_ref().expect("a plan order");
    assert_eq!(order.intent, IntentId::new(1));
    assert_eq!(
        order.men,
        Some(MenCount {
            wanted: 400,
            to_hand: 600
        })
    );
    assert!(
        order.waits_for.iter().all(|w| w.met),
        "it waits for nothing"
    );
    assert_eq!(order.expected_ashore, Some(4));
    assert!(e.plan_order_differs_in.is_empty(), "the same in all four");
}

#[test]
fn what_was_assumed() {
    let e = main_estimate();
    assert_eq!(e.assumed, assumed_main());
    assert_eq!(e.leaves_given, vec![Leave::Launch]);
}

#[test]
fn where_the_worlds_part() {
    let e = main_estimate();
    assert_eq!(
        e.partings,
        vec![Parting {
            by: PartedBy::BelievedMen {
                place: 1,
                name: BELIEVED.to_string(),
            },
            with: PartSide {
                went: Went {
                    worlds: vec![2, 4],
                    met_kept: vec![],
                    met_days: None,
                    lost_again: vec![],
                    not_met: vec![2, 4],
                },
                side: Some(costs(spread(300, 300, 300))),
                enemy: Some(costs(spread(300, 300, 300))),
            },
            without: PartSide {
                went: Went {
                    worlds: vec![1, 3],
                    met_kept: vec![1, 3],
                    met_days: Some(spread(4, 4, 4)),
                    lost_again: vec![],
                    not_met: vec![],
                },
                side: Some(costs(spread(100, 100, 100))),
                enemy: Some(costs(spread(200, 200, 200))),
            },
        }]
    );
}

#[test]
fn each_worlds_own_figures() {
    let e = main_estimate();
    let got: Vec<(u32, Option<u32>, bool, Losses, Losses)> = e
        .worlds
        .iter()
        .map(|w| (w.number, w.met_day, w.lost_again, w.side, w.enemy))
        .collect();
    assert_eq!(
        got,
        vec![
            (1, Some(4), false, men_lost(100), men_lost(200)),
            (2, None, false, men_lost(300), men_lost(300)),
            (3, Some(4), false, men_lost(100), men_lost(200)),
            (4, None, false, men_lost(300), men_lost(300)),
        ]
    );
    for (w, played) in e.worlds.iter().zip(&main_case().played.worlds) {
        assert_eq!(w.assumptions, played.assumptions);
    }
    assert_eq!(e.other_plans, None, "no trial of things as they stand");
}

#[test]
fn its_words_name_the_counts_and_no_chance() {
    let words = main_estimate().to_string();
    assert!(words.contains("2 of 4 worlds"), "{words}");
    let lower = words.to_lowercase();
    assert!(!words.contains('%'), "{words}");
    assert!(!lower.contains("chance"), "{words}");
    assert!(!lower.contains("points"), "{words}");
}

// ---------------------------------------------------------------- Properties

#[test]
fn things_as_they_stand_change_nothing_above() {
    let live = scenario();
    let stand = trial(&live, Vec::new());
    assert_eq!(stand.worlds.len(), 4);
    for w in &stand.worlds {
        assert!(w.failed.is_none());
        for d in &w.days {
            for side in [ALLIED, JAPANESE] {
                assert!(
                    d.staff.get(&side).is_none_or(|s| s.orders.is_empty()),
                    "nothing is ordered"
                );
            }
        }
        let truth = world_of(w).umpire();
        assert_eq!(base_side(truth, 2), JAPANESE);
        assert!(!world_of(w).events().iter().any(|e| matches!(
            e.kind,
            EventKind::MenLostInFight { .. }
                | EventKind::MenLostUnfed { .. }
                | EventKind::MenLostToFire { .. }
        )));
    }
    let with = estimate(&main_case().played, IntentId::new(1), Some(&stand));
    assert_eq!(
        with.other_plans,
        Some(Vec::new()),
        "the side has no other plan"
    );
    let mut same = with.clone();
    same.other_plans = None;
    assert_eq!(&same, main_estimate());
}

fn with_standing(standing: &str) -> Estimate {
    let live = load(&common::replace_once(
        &text(),
        "standing = \"unknown\"",
        &format!("standing = \"{standing}\""),
    ));
    estimate(&trial(&live, course()), IntentId::new(1), None)
}

#[test]
fn believed_present_and_absent() {
    let present = with_standing("present");
    assert_eq!(present.went.met_kept, Vec::<u32>::new());
    assert_eq!(present.went.not_met, vec![1, 2, 3, 4]);
    assert!(present.partings.is_empty());
    let garrison: Vec<bool> = present
        .worlds
        .iter()
        .map(|w| {
            w.assumptions.iter().any(|a| {
                matches!(
                    a,
                    Assumption::BelievedMenMade {
                        men: 400,
                        standing: Standing::Present,
                        ..
                    }
                )
            })
        })
        .collect();
    assert_eq!(garrison, vec![true; 4], "600 in all four");

    let absent = with_standing("absent");
    assert_eq!(absent.went.met_kept, vec![1, 2, 3, 4]);
    assert_eq!(absent.went.met_days, Some(spread(4, 4, 4)));
    assert!(absent.partings.is_empty());
    assert!(absent
        .worlds
        .iter()
        .all(|w| w.assumptions.iter().any(|a| matches!(
            a,
            Assumption::BelievedMenRuledOut {
                place: 1,
                men: 400,
                ..
            }
        )) && !w
            .assumptions
            .iter()
            .any(|a| matches!(a, Assumption::BelievedMenMade { .. }))));
}

/// The build's means, which the hand calculation did not give: its own
/// (the live game played to the evening of day 5 with the course) gives no
/// count, for unit 1's attack on 1,200 men fails on day 4 and its men are
/// lost, so no man of the side is at B to report. The view taken is world 2
/// of the main case at the end of its play, where the daily report from day 4 gives
/// 300 men at B: a side's view, as any other, with a count since the start.
#[test]
fn believed_men_are_set_aside_when_the_side_has_a_count_since() {
    let live = scenario();
    let two = &main_case().played.worlds[1];
    assert_eq!(two.number, 2);
    let view = world_of(two).side_view(ALLIED);
    let count = view.ground_estimate(B).expect("a count of B");
    assert_eq!(
        (count.men, count.day),
        (300, 8),
        "a count of a day after the start: the daily report"
    );
    let made = pictures(
        &view,
        &believed(&live),
        &figures(&live),
        Trial {
            seed: 1942,
            trial: 1,
            worlds: 4,
        },
    )
    .expect("the pictures are made");
    assert_eq!(made.len(), 4);
    for picture in &made {
        assert!(
            !picture
                .picture
                .truth
                .land_units
                .values()
                .any(|u| u.men_ashore == 400 && u.side == JAPANESE),
            "the 400 are made in no world"
        );
        assert!(picture
            .assumptions
            .contains(&Assumption::BelievedMenSetAside {
                place: 1,
                name: BELIEVED.to_string(),
                hex: B,
                men: 400,
                standing: Standing::Unknown,
                why: SetAside::CountSince { day: count.day },
            }));
        assert!(!picture.assumptions.iter().any(|a| matches!(
            a,
            Assumption::BelievedMenMade { .. } | Assumption::BelievedMenNotInWorld { .. }
        )));
    }
}

/// The pictures of the main case with those of `unsound` made unsound: each
/// one's Allied counter of task forces set below the side's first number.
fn with_unsound(unsound: &[usize]) -> Played {
    let live = scenario();
    let mut made = pictures(
        &live.world.side_view(ALLIED),
        &believed(&live),
        &figures(&live),
        Trial {
            seed: 1942,
            trial: 1,
            worlds: 4,
        },
    )
    .expect("the pictures are made");
    let first = PictureCounters::first(ALLIED);
    for &i in unsound {
        made[i]
            .picture
            .counters
            .entry(ALLIED)
            .or_insert(first)
            .next_task_force = first.next_task_force - 1;
    }
    begin_from_pictures(
        made,
        live.world.ref_data(),
        live.world.map(),
        asked(course()),
    )
    .expect("the trial begins")
    .play_out()
}

#[test]
fn no_world_played() {
    let e = estimate(&with_unsound(&[0, 1, 2, 3]), IntentId::new(1), None);
    assert_eq!(e.asked, 4);
    assert_eq!(e.could_not.len(), 4);
    assert!(e.went.worlds.is_empty());
    assert_eq!(e.went.met_days, None);
    assert_eq!(e.last_day, None);
    assert_eq!(e.side_costs, None);
    assert_eq!(e.enemy_costs, None);
    assert!(e.worlds.is_empty());
    let words = e.to_string();
    assert!(
        words.contains("None of 4 worlds could be played"),
        "{words}"
    );
}

#[test]
fn a_world_that_could_not_be_played_is_counted_apart() {
    let e = estimate(&with_unsound(&[1]), IntentId::new(1), None);
    assert_eq!(
        e.could_not
            .iter()
            .map(|f| (f.world, f.day))
            .collect::<Vec<_>>(),
        vec![(2, 0)]
    );
    assert_eq!(
        e.went,
        Went {
            worlds: vec![1, 3, 4],
            met_kept: vec![1, 3],
            met_days: Some(spread(4, 4, 4)),
            lost_again: vec![],
            not_met: vec![4],
        }
    );
    // 100, 100, 300 in order: the middle at place 2.
    assert_eq!(e.side_costs, Some(costs(spread(100, 100, 300))));
    assert_eq!(e.enemy_costs, Some(costs(spread(200, 200, 300))));
    let words = e.to_string();
    assert!(words.contains("3 of 4 worlds played"), "{words}");
}

#[test]
fn an_intent_with_no_end() {
    let keep = r#"
[[command]]
day = 1
side = "allied"
kind = "set_intent"
intent = 1
intent_kind = "keep_supplied"
base = 1
floor_days = 3
"#;
    let live = scenario();
    let e = estimate(&trial(&live, course_of(keep)), IntentId::new(1), None);
    assert!(matches!(e.kind, Some(IntentKind::KeepSupplied { .. })));
    assert_eq!(e.end, None);
    assert_eq!(e.went.worlds, vec![1, 2, 3, 4]);
    assert!(e.went.met_kept.is_empty() && e.went.not_met.is_empty());
    assert!(e.side_costs.is_some() && e.enemy_costs.is_some());
    let words = e.to_string();
    assert!(words.contains("no end an estimate can read"), "{words}");
}

// ---------------------------------------------------------------- The loader

fn with_men(body: &str) -> String {
    format!(
        "{}\n[[believed_men]]\nside = \"allied\"\nname = \"More\"\n{body}\n",
        text()
    )
}

#[test]
fn believed_men_are_loaded_and_refused_as_a_believed_force_is() {
    let s = load(&with_men("q = 4\nr = 0\nmen = 50\nstanding = \"present\""));
    let men: Vec<_> = s.believed_men(ALLIED).collect();
    assert_eq!(men.len(), 2);
    assert_eq!(
        (men[1].name.as_str(), men[1].hex, men[1].base, men[1].men),
        ("More", B, None, 50)
    );
    assert_eq!(men[1].standing, Standing::Present);
    assert!(!men[1].placeholder);

    let cases = [
        (
            "q = 4\nr = 0\nmen = 0\nstanding = \"present\"",
            "have no man",
        ),
        (
            "q = 4\nr = 0\nmen = 5\nstanding = \"maybe\"",
            "unknown standing",
        ),
        ("at_base = 1\nmen = 5\nstanding = \"present\"", "side's own"),
        (
            "at_base = 99\nmen = 5\nstanding = \"present\"",
            "not a base of the scenario",
        ),
        (
            "q = 99\nr = 0\nmen = 5\nstanding = \"present\"",
            "off the map",
        ),
        (
            "q = 2\nr = 0\nmen = 5\nstanding = \"present\"",
            "which is sea",
        ),
        (
            "at_base = 2\nq = 4\nr = 0\nmen = 5\nstanding = \"present\"",
            "both",
        ),
        ("men = 5\nstanding = \"present\"", "no place"),
        ("q = 4\nmen = 5\nstanding = \"present\"", "a hex takes both"),
        (
            "q = 4\nr = 0\nmen = 5\nstanding = \"present\"\nday = 2",
            "day",
        ),
        (
            "q = 4\nr = 0\nmen = 5\nstanding = \"present\"\nplaceholder = true",
            "note",
        ),
    ];
    for (body, words) in cases {
        let message = refused(&with_men(body));
        assert!(message.contains(words), "{body}: {message}");
    }
    // A base of the enemy's not on the side's map.
    let off_map = format!(
        "{}\n[[base]]\nid = 7\nname = \"Placeholder unseen\"\nside = \"japanese\"\nq = 0\nr = 6\non_map = false\n",
        with_men("at_base = 7\nmen = 5\nstanding = \"present\"")
    );
    let message = refused(&off_map);
    assert!(
        message.contains("not on the Allied side's map"),
        "{message}"
    );
}

// ---------------------------------------------------------------- The same twice

#[test]
fn the_same_twice_and_the_live_game_untouched() {
    let live = scenario();
    let before = live.world.state_hash();
    let first = trial(&live, course());
    let second = trial(&live, course());
    let a = estimate(&first, IntentId::new(1), Some(&second));
    let b = estimate(&second, IntentId::new(1), Some(&first));
    assert_eq!(a, b);
    assert_eq!(&estimate(&first, IntentId::new(1), None), main_estimate());
    assert_eq!(live.world.state_hash(), before);
}

// ---------------------------------------------------------------- A plan displaced

/// A plan displaced, as the architect stated it on Nov 19, 2026, from the
/// build's run. A second Allied unit of 600 men at A; intent 2 stated in the
/// live game on day 1, to take the place at base 2 with that unit and task
/// force 1, its leaves given, the live game played to the evening of day 1;
/// the course, intent 10, takes the same place with task force 1 and unit 1.
/// Run with `--nocapture` to read what it gives.
#[test]
fn a_plan_displaced() {
    let second = r#"
[[land_unit]]
id = 2
name = "Placeholder Second"
side = "allied"
men = 600
q = 0
r = 0

[[command]]
day = 1
side = "allied"
kind = "set_intent"
intent = 2
intent_kind = "take_place"
hex = [4, 0]

[[command]]
day = 1
side = "allied"
kind = "set_limit"
limit = "forces"
intent = 2
task_forces = [1]
land_units = [2]
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
    // The course: intent 10, to take the place at (4,0) with unit 1 and
    // task force 1 (the side's leaves are given already). Intent 3 is the
    // staff's own by the evening of day 1 (its "ready a landing" under
    // intent 2), so the course takes a number the side has not used.
    let course: Vec<CourseCommand> = course_of(&COURSE.replace("intent = 1", "intent = 10"))
        .into_iter()
        .take(2)
        .collect();
    let mut live = load(&format!("{}\n{second}", text()));
    // The live game takes in its own commands of day 1: played to the
    // evening of day 1 with both staffs, so that intent 2 is stated.
    let held = staffs();
    live.next_day_staffed(as_staffs(&held))
        .expect("a day resolves")
        .expect("a day");
    let with = trial(&live, course);
    let stand = trial(&live, Vec::new());
    let e = estimate(&with, IntentId::new(10), Some(&stand));
    for (name, played) in [("with the course", &with), ("things as they stand", &stand)] {
        for w in &played.worlds {
            let units: Vec<(u32, u32, Option<Hex>)> = world_of(w)
                .umpire()
                .land_units
                .values()
                .map(|u| (u.id.raw(), u.men_ashore, u.hex))
                .collect();
            let fights: Vec<String> = world_of(w)
                .events()
                .iter()
                .filter(|e| {
                    matches!(
                        e.kind,
                        EventKind::GroundFire { .. }
                            | EventKind::MenLanded { .. }
                            | EventKind::MenLostInFight { .. }
                    )
                })
                .map(|e| format!("day {} {:?}", e.day, e.kind))
                .collect();
            println!(
                "{name}, world {}: units at the end {units:?}; {fights:?}",
                w.number
            );
            for d in &w.days {
                let s = d.staff.get(&ALLIED).cloned().unwrap_or_default();
                println!(
                    "  day {}: orders {:?}; commands {:?}; bases own {:?} enemy {:?}",
                    d.day,
                    s.orders.iter().map(given).collect::<Vec<_>>(),
                    s.commands
                        .iter()
                        .map(|c| format!("{:?}", c.kind))
                        .collect::<Vec<_>>(),
                    d.bases.own,
                    d.bases.enemy
                );
            }
        }
    }
    println!("{e:#?}");
    println!("{e}");
    assert_eq!(e.worlds.len(), 4, "every world played");
    // Stated by the architect on Nov 19, 2026, from the build's run: without
    // the course unit 2 lands on day 4; with it unit 1 lands on day 4 and
    // unit 2 stays at A.
    let landed = |w: &PlayedWorld| -> Vec<(u32, u32)> {
        let mut days: Vec<(u32, u32)> = world_of(w)
            .events()
            .iter()
            .filter_map(|e| match &e.kind {
                EventKind::MenLanded { unit, .. } => Some((e.day, unit.raw())),
                _ => None,
            })
            .collect();
        days.dedup();
        days
    };
    for w in &stand.worlds {
        assert_eq!(
            landed(w),
            vec![(4, 2)],
            "without the course, world {}",
            w.number
        );
    }
    for w in &with.worlds {
        assert_eq!(
            landed(w),
            vec![(4, 1)],
            "with the course, world {}",
            w.number
        );
        let two = &world_of(w).umpire().land_units[&LandUnitId::new(2)];
        assert_eq!(
            (two.hex, two.men_ashore),
            (Some(A), 600),
            "unit 2 stays at A, world {}",
            w.number
        );
    }
    // Intent 2 is met and kept in worlds 1 and 3 with a middle day of 4,
    // with the course and without: it is not named as displaced.
    let kept = Kept {
        worlds: vec![1, 3],
        middle_day: Some(4),
    };
    assert_eq!(
        e.other_plans,
        Some(vec![OtherPlan {
            intent: IntentId::new(2),
            kind: IntentKind::TakePlace {
                hex: B,
                by_day: None
            },
            with_course: kept.clone(),
            without: kept,
            displaced: None,
        }])
    );
}
