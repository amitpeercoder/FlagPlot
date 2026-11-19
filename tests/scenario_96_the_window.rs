//! Scenario 96: the window. The same landing is tried at two dates in the
//! same worlds: now, with the one unit that is to hand, and on the day a
//! second unit may first be used. The estimates are set side by side,
//! with what the later date buys and what the enemy did with the days.
//!
//! Proves decision 0177, step 7 of step 3's stage 4, against a hand
//! calculation. The data is placeholder data: see
//! `data/scenarios/96_the_window.toml`, which the build writes to the
//! inputs below. It runs on the frozen step 1 figures with scenario 68's
//! ground figures and three more of its own, so that men dig in: every
//! shot is a whole shot and hits, and no die changes anything.
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
//! Scenario 68's (see its hand calculation, and scenario 95's): one
//! shot for each 100 firing men, every shot hits and takes 50 men; a
//! side is beaten where the other attacked and it has no man left, or
//! fewer than 0.2 of the other's. B has no land link, so beaten men
//! cannot fall back: they surrender. With three ground figures of this
//! scenario's own: `fieldworks_per_day` 10, `fieldworks_limit` 100 and
//! `cover_in_fight` 0.5. A fighting unit ashore on the defence digs 10
//! of fieldworks at the end of each day, after that day's fight, to 100
//! at most. In a fight the loss of a side is first held to the men it
//! has and shared between its units; each unit's part is then cut by
//! its cover over 100 times 0.5, and cut to a whole man: at a cover of
//! 30 a part of 200 is 200 x (1 - 0.15) = 170; at 70 it is
//! 200 x (1 - 0.35) = 130. Cover does not change the shots of either
//! side, and an attacker has none. (Amended on review: this first cut
//! the loss by cover before holding it to the men there, which is not
//! the engine's order, and left out the 0.2 of the beaten.)
//!
//! With no count of the enemy the staff takes every unit its ships can
//! carry, and ashore it attacks. At balanced it asks for a count before
//! it goes and, with nothing that can look, goes without one. There is
//! one task force: no unit is sent after the first lift while the
//! horizons here last.
//!
//! ## Inputs
//!
//! Scenario 68's map, bases, ships and task force, with these changes:
//!
//! - **Two units.** Unit 1, 600 fighting men at A, usable from day 1.
//!   Unit 2, 600 fighting men at A, usable from day 4. No unit 3.
//! - **No opening estimate.** The Allied side holds no count of B.
//! - **Believed men:** the Allied side believes 200 men are at B,
//!   standing present.
//! - **No commands in the scenario.**
//! - **The trial's figures:** an enemy base is assumed to hold 200 t of
//!   supply and 0 t of fuel; the seed is 1942. A file of sides that
//!   gives every class to both sides.
//!
//! **The course:** for the Allied side: intent 1, take the place at
//! (4,0); its forces limit, task force 1, units 1 and 2 and base 1 as
//! the source; leave to prepare given; leave to launch given. Each
//! command is of day 1 of the course.
//!
//! **The window:** of the Allied side, from the evening before day 1,
//! trial 1, 4 worlds, a size of 8 days.
//!
//! ## The dates
//!
//! The first date is day 1. Of what the intent's forces limit names,
//! task force 1 and unit 1 may be used from day 1 and unit 2 from day 4.
//! No aircraft are due, no force is in a yard and no supply run is due.
//! So there is one later date, day 4. The window has two dates: day 1,
//! played for 8 days, to day 8; and day 4, played for 8 + 3 = 11 days,
//! to day 11.
//!
//! ## The worlds
//!
//! In every world the enemy has base 2 at B with 200 t of supply and
//! land unit 900,001, 200 fighting men on defend at B, from the
//! believed men. Nothing is of unknown standing, so the four worlds are
//! alike, and the Japanese side is given no intent and orders nothing.
//!
//! The 200 men dig: their fieldworks are 10 at the end of day 1, 20
//! after day 2, 30 after day 3, and so on by 10 a day.
//!
//! ## The first date: day 1
//!
//! The staff holds no count: it takes every unit its ships can carry
//! that it may use. On the morning of day 1 that is unit 1. The days
//! are scenario 95's: on day 1 it forms the landing and orders
//! `prepare_landing` with roster 1; on day 2 the landing is ready, the
//! plan is released and 45 t of supply are loaded; on day 3 unit 1 is
//! set to attack and loaded; on day 4 the force sails at hour 72,
//! arrives at hour 88 and lands the 600 men, and the fight is that
//! evening.
//!
//! | The fight of day 4 | Allied | Japanese |
//! | --- | --- | --- |
//! | Men ashore | 600, on attack | 200, on defend, fieldworks 30 |
//! | Shots, all hits | 6 | 2 |
//! | Men lost in the fight | 2 x 50 = 100 | 6 x 50 = 300, held to the 200 there, cut by cover to 170 |
//! | Left | 500 | 30 |
//!
//! The Japanese side has 30 men against 0.2 x 500 = 100: it is beaten.
//! Its 30 men surrender. Base 2 is Allied at the end of day 4: the end
//! is met on day 4, in each of the four worlds. The plan is marked
//! complete on the morning of day 5, and no man is lost after day 4.
//!
//! ## The second date: day 4
//!
//! Days 1 to 3 are played as things stand: nothing is ordered. On the
//! morning of day 4 the course is handed in, and the staff may use
//! units 1 and 2: it takes both, one to a ship. The days are scenario
//! 80's from its day 4: on day 4 it forms the landing and orders
//! `prepare_landing` with roster 1 and 2; on day 5 the landing is
//! ready, the plan is released and 1,200 x 0.005 x 15 = 90 t of supply
//! are loaded; on day 6 unit 1 is set to attack and loaded; on day 7
//! unit 2; on day 8 the force sails at hour 168, arrives at hour 184
//! and lands 600 men from each ship, and the fight is that evening.
//!
//! | The fight of day 8 | Allied | Japanese |
//! | --- | --- | --- |
//! | Men ashore | 600 and 600, on attack | 200, on defend, fieldworks 70 |
//! | Shots, all hits | 6 and 6 | 2 |
//! | Men lost in the fight | 2 x 50 = 100: 50 each | 12 x 50 = 600, held to the 200 there, cut by cover to 130 |
//! | Left | 550 and 550 | 70 |
//!
//! The Japanese side has 70 men against 0.2 x 1,100 = 220: it is
//! beaten, and its 70 men surrender. Base 2 is Allied at the end of day
//! 8: the end is met on day 8, in each of the four worlds.
//!
//! ## The window of intent 1
//!
//! | | The first date, day 1 | The second date, day 4 |
//! | --- | --- | --- |
//! | Played to | day 8 | day 11 |
//! | Its course taken in on | day 1 | day 4 |
//! | The men its take chose | 600, unit 1 | 1,200, units 1 and 2 |
//! | The plan order: men wanted and to hand | none: no count is held | none |
//! | The plan order: day expected ashore | day 4 | day 8 |
//! | The enemy at B on the eve: men, lowest, middle, highest | 200, 200, 200 (the evening before day 1) | 200, 200, 200 (the evening of day 3) |
//! | The enemy at B on the eve: cover | 0, 0, 0 | 30, 30, 30 |
//! | Met and kept | 4 of 4 worlds, on day 4, 4 and 4 | 4 of 4, on day 8, 8 and 8 |
//! | The side's men lost to the enemy | 100, 100, 100 | 100, 100, 100 |
//! | The trial's enemy's men lost to the enemy | 200, 200, 200 (170 in the fight and 30 by surrender) | 200, 200, 200 (130 and 70) |
//! | Every other cost of either side | 0 | 0 |
//!
//! - **What the second date buys:** unit 2, a land unit, usable from
//!   day 4; its take chose 1,200 men against 600.
//! - **What the second date's words say** besides: that nothing of the
//!   course was done before day 4.
//! - **World for world,** the second date against the first: met and
//!   kept at both in 4 worlds; at the first alone in none; at the later
//!   alone in none; at neither in none.
//!
//! ## Worlds that differ: where waiting wins
//!
//! (Added on review, so that the comparison of the dates world for
//! world has something to compare.) The main case with one change: the
//! Allied side also believes 200 more men may be at B, standing
//! unknown; and a size of 5 days. In worlds 2 and 4 the garrison is two
//! units of 200, land units 900,001 and 900,002; in worlds 1 and 3 it
//! is the 200. The first date is played to day 5 and the second to day
//! 8.
//!
//! - **Worlds 1 and 3** are the main case: met on day 4 at the first
//!   date and on day 8 at the second; the side loses 100 men at each.
//! - **Worlds 2 and 4, the first date.** The fight of day 4: 600
//!   against 200 and 200 at fieldworks 30. The 6 hits take 300, 150
//!   from each unit, each cut by cover to 150 x 0.85 = 127.5, 127 men:
//!   73 are left in each, 146. The 4 Japanese shots take 200: unit 1
//!   has 400. 146 is not fewer than 0.2 x 400 = 80: not beaten. The
//!   report gives 146, so 292 men are wanted; unit 1's fit men on the
//!   morning of day 5 are 400 x 50 / 100 = 200: it is set to defend,
//!   and there is no fight on day 5. Not met by day 5. The side has
//!   lost 200 men and the enemy 254.
//! - **Worlds 2 and 4, the second date.** The fight of day 8: 1,200
//!   against 200 and 200 at fieldworks 70. The 12 hits take 600, held
//!   to the 400 there, 200 from each unit, each cut to 200 x 0.65 =
//!   130: 70 are left in each, 140. The 4 Japanese shots take 200, 100
//!   from each Allied unit: 1,000 are left. 140 is fewer than
//!   0.2 x 1,000 = 200: beaten, and the 140 surrender. Met on day 8.
//!   The side has lost 200 men and the enemy 400.
//!
//! | | The first date, day 1 | The second date, day 4 |
//! | --- | --- | --- |
//! | Met and kept | 2 of 4 worlds, worlds 1 and 3, on day 4 | 4 of 4, on day 8 |
//! | Not met | 2, worlds 2 and 4 | none |
//! | The side's men lost to the enemy | 100, 100, 200 | 100, 100, 200 |
//! | The trial's enemy's men lost to the enemy | 200, 200, 254 | 200, 200, 400 |
//! | The enemy at B on the eve: men | 200, 200, 400 | 200, 200, 400 |
//! | The enemy at B on the eve: the least cover | 0, 0, 0 | 30, 30, 30 |
//!
//! - **World for world,** the second date against the first: met and
//!   kept at both in 2 worlds, 1 and 3; at the first alone in none; at
//!   the later alone in 2, worlds 2 and 4; at neither in none.
//! - **Each date's estimate parts its worlds** by the 200 men of
//!   unknown standing, as decision 0176 has it.
//!
//! # Properties
//!
//! - **The days before a date are played once, and nothing is changed
//!   by it.** (Added on review.) The second date's estimate in the main
//!   case is, field for field, the estimate of a plain trial of 4
//!   worlds for 11 days whose course has each command on day 4; and each
//!   of its worlds has that trial's hash at the end of every day.
//! - **World for world, by counting.** (Added on review.) Inside the
//!   crate, on records made by the test: five worlds, one met and kept
//!   at both dates, one at the first alone, one at the later alone, one
//!   at neither, and one that could not be played at the later date,
//!   give 1, 1, 1, 1 and one counted apart. A world met and lost again
//!   at a date counts there as not met and kept.
//! - **The edge of the horizon.** (Added on review.) With unit 2 usable
//!   from day 8 and a size of 8 days: day 8 is a date. From day 9: it is
//!   not.
//! - **Two things on one day are one date.** (Added on review.) With a
//!   third unit also usable from day 4: one later date, day 4, naming
//!   units 2 and 3.
//! - **A force back from a yard is a date.** (Added on review.) With a
//!   second transport force of the side's in a yard and due back on day
//!   3, and no forces limit on the course: day 3 is a date and names
//!   it.
//! - **Standing commands are not shifted.** (Added on review.) With a
//!   command given as standing, a hold put on unit 2 for the morning of
//!   day 1: at the second date it is taken in on day 1, and the course
//!   on day 4.
//! - **Dates may be given.** (Added on review.) The main case asked for
//!   with the dates 1 and 3: the second date is day 3; unit 2 may not be
//!   used that morning, so its take chooses unit 1 alone, 600 men.
//! - **The same worlds.** The pictures of the two dates' trials are
//!   equal, world for world.
//! - **The first date is the estimate.** The first date's estimate is
//!   the estimate decision 0176 gives of the same course in a trial of 4
//!   worlds for 8 days, field for field.
//! - **Nothing of the side's is due to change.** With unit 2 usable from
//!   day 1: the window has one date, day 1, and says that nothing is due
//!   to change. The staff takes both units on day 1.
//! - **Three dates at most.** With a third unit of 600 men usable from
//!   day 6 and a fourth from day 7, all in the forces limit: the dates
//!   are days 1, 4 and 6. Day 7 is not tried.
//! - **Outside the limit does not count.** With unit 2 left out of the
//!   course's forces limit: the window has one date.
//! - **Beyond the horizon does not count.** With unit 2 usable from day
//!   10 and a size of 8 days: one date.
//! - **A date is told before it is played.** The dates of the main case
//!   are had from the view and the course alone, with no world built.
//! - **The same twice,** and the live game untouched: its hash before
//!   and after the window is the same.
//! - **No live game changes** by the build: no baseline, golden file or
//!   pinned hash moves.

// The tests are written by the build, to the statements above.

mod common;

use std::path::PathBuf;
use std::sync::OnceLock;

use sim_core::believed::Standing;
use sim_core::command::{CommandKind, HoldTarget, IntentKind, Leave, Limit, StaffOrder};
use sim_core::events::EventKind;
use sim_core::fixed::Fixed;
use sim_core::hex::Hex;
use sim_core::ids::{BaseId, EntityRef, IntentId, LandUnitId, TaskForceId};
use sim_core::orders::{Cargo, LandUnitOrder, OrderKind, Posture, TaskForceOrder};
use sim_core::refdata::TrialFigures;
use sim_core::state::{Side, TruthState};
use sim_data::Scenario;
use sim_staff::{as_staffs, staffs};
use sim_trial::{
    begin, begin_window, dates, estimate, play, play_window, window, Asked, Assumption, Believed,
    Costs, CourseCommand, CoverSpread, Date, Due, DueKind, Estimate, Played, PlayedWindow,
    PlayedWorld, Spread, Stepped, Window, WindowAsked, WorldForWorld,
};

const ALLIED: Side = Side::Allied;
const JAPANESE: Side = Side::Japanese;
const A: Hex = Hex { q: 0, r: 0 };
const B: Hex = Hex { q: 4, r: 0 };

/// The course: intent 1, take the place at (4,0); its forces limit, task
/// force 1, units 1 and 2 and base 1 as the source; both leaves given; as
/// scenario text, read by the loader.
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
land_units = [1, 2]
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
    common::data_dir().join("scenarios/96_the_window.toml")
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

/// The scenario with `original` replaced by `changed`, once.
fn changed(original: &str, replaced: &str) -> Scenario {
    load(&common::replace_once(&text(), original, replaced))
}

/// The Allied side's commands of day 1 in `extra`, scenario text added to
/// `base`, as the loader reads them, each on day `day` of the course.
fn commands_of(base: &str, extra: &str) -> Vec<CommandKind> {
    let with = load(&format!("{base}\n{extra}"));
    with.scripted_commands()
        .get(&1)
        .map(|day| {
            day.iter()
                .filter(|(side, _)| *side == ALLIED)
                .map(|(_, command)| command.clone())
                .collect()
        })
        .unwrap_or_default()
}

fn on_day(commands: &[CommandKind], day: u32) -> Vec<CourseCommand> {
    commands
        .iter()
        .cloned()
        .map(|command| CourseCommand { day, command })
        .collect()
}

fn course_text(text: &str) -> Vec<CourseCommand> {
    on_day(&commands_of(&self::text(), text), 1)
}

fn course() -> Vec<CourseCommand> {
    course_text(COURSE)
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
        .expect("scenario 96 names its trial's figures")
}

fn asked(days: u32, course: Vec<CourseCommand>, dates: Option<Vec<u32>>) -> WindowAsked {
    WindowAsked {
        trial: 1,
        worlds: 4,
        days,
        intent: IntentId::new(1),
        standing: Vec::new(),
        course,
        dates,
    }
}

fn played_window(scenario: &Scenario, asked: WindowAsked) -> PlayedWindow {
    play_window(
        &scenario.world.side_view(ALLIED),
        &believed(scenario),
        &figures(scenario),
        scenario.world.ref_data(),
        scenario.world.map(),
        asked,
    )
    .expect("the window is played")
}

/// A plain trial of the Allied side on `scenario`: trial 1, 4 worlds.
fn trial(scenario: &Scenario, days: u32, course: Vec<CourseCommand>) -> Played {
    play(
        &scenario.world.side_view(ALLIED),
        &believed(scenario),
        &figures(scenario),
        scenario.world.ref_data(),
        scenario.world.map(),
        Asked {
            trial: 1,
            worlds: 4,
            days,
            course,
        },
    )
    .expect("the trial is played")
}

struct Main {
    played: PlayedWindow,
    window: Window,
}

fn main_case() -> &'static Main {
    static MAIN: OnceLock<Main> = OnceLock::new();
    MAIN.get_or_init(|| {
        let played = played_window(&scenario(), asked(8, course(), None));
        let window = window(&played);
        Main { played, window }
    })
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

fn costs(men: Spread) -> Costs {
    Costs {
        men_to_enemy: men,
        men_unfed: zero(),
        ships_sunk: zero(),
        ships_out_of_action: zero(),
        aircraft: zero(),
    }
}

fn cover(lowest: i64, middle: i64, highest: i64) -> CoverSpread {
    CoverSpread {
        lowest: n(lowest),
        middle: n(middle),
        highest: n(highest),
    }
}

fn world_of(played: &PlayedWorld) -> &sim_core::World {
    played.world.as_ref().expect("the world was built")
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

/// The Allied staff's orders of each day of a world, each day's sorted.
fn orders(world: &PlayedWorld) -> Vec<(u32, Vec<Given>)> {
    world
        .days
        .iter()
        .map(|d| {
            let mut day: Vec<Given> = d
                .staff
                .get(&ALLIED)
                .map(|s| s.orders.iter().map(given).collect())
                .unwrap_or_default();
            day.sort_by_key(|g| format!("{g:?}"));
            (d.day, day)
        })
        .collect()
}

fn sorted(mut days: Vec<(u32, Vec<Given>)>) -> Vec<(u32, Vec<Given>)> {
    for (_, day) in &mut days {
        day.sort_by_key(|g| format!("{g:?}"));
    }
    days
}

/// The men each unit lost in a fight, and the men that surrendered, by day:
/// (day, unit, lost) and (day, unit, men).
/// (day, unit, men).
type ByUnit = Vec<(u32, u32, u32)>;

fn fight_losses(world: &PlayedWorld) -> (ByUnit, ByUnit) {
    let mut lost = Vec::new();
    let mut surrendered = Vec::new();
    for e in world_of(world).events().iter() {
        match &e.kind {
            EventKind::MenLostInFight {
                unit, lost: men, ..
            } => {
                lost.push((e.day, unit.raw(), *men));
            }
            EventKind::UnitSurrendered { unit, men, .. } => {
                surrendered.push((e.day, unit.raw(), *men));
            }
            _ => {}
        }
    }
    (lost, surrendered)
}

/// The first day at whose end the side holds B and the enemy no base there.
fn met_on(world: &PlayedWorld) -> Option<u32> {
    world
        .days
        .iter()
        .find(|d| d.bases.own.contains(&B) && !d.bases.enemy.contains(&B))
        .map(|d| d.day)
}

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
    assert!(!t.contains("[[estimate]]"));
    assert!(t.contains("placeholder = true"));
    let s = scenario();
    let truth = s.world.umpire();
    let allied: Vec<(u32, u32, Option<Hex>, Option<u32>)> = truth
        .land_units
        .values()
        .filter(|u| u.side == ALLIED)
        .map(|u| (u.id.raw(), u.men_ashore, u.hex, u.available_from))
        .collect();
    assert_eq!(
        allied,
        vec![(1, 600, Some(A), None), (2, 600, Some(A), Some(4))]
    );
    let view = s.world.side_view(ALLIED);
    assert_eq!(view.ground_estimates().count(), 0, "no opening estimate");
    let men: Vec<_> = s.believed_men(ALLIED).collect();
    assert_eq!(men.len(), 1);
    assert_eq!(
        (men[0].hex, men[0].base, men[0].men, men[0].standing),
        (B, Some(BaseId::new(2)), 200, Standing::Present)
    );
    assert!(men[0].placeholder);
    let ground = s.world.ref_data().ground.as_ref().expect("ground figures");
    assert_eq!(ground.fieldworks_per_day, n(10));
    assert_eq!(ground.fieldworks_limit, n(100));
    assert_eq!(ground.cover_in_fight, Fixed::from_ratio(1, 2));
    assert_eq!(ground.men_per_shot, n(100));
    assert_eq!(ground.men_per_hit, n(50));
    let f = figures(&s);
    assert!(f.placeholder);
    assert_eq!(
        (f.enemy_base_supply_t, f.enemy_base_fuel_t, f.seed),
        (n(200), n(0), 1942)
    );
    for class in view.ship_classes() {
        let whose = view.ship_class_side(class.id).expect("a side");
        assert!(whose.used_by(ALLIED) && whose.used_by(JAPANESE));
    }
    for aircraft in view.aircraft_types() {
        let whose = view.aircraft_type_side(aircraft.id).expect("a side");
        assert!(whose.used_by(ALLIED) && whose.used_by(JAPANESE));
    }
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
    let CommandKind::SetLimit(Limit::Forces { intent, limit }) = &c[1].command else {
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
        vec![1, 2]
    );
    assert_eq!(limit.source, Some(BaseId::new(1)));
    assert_eq!(
        (c[2].command.clone(), c[3].command.clone()),
        (
            CommandKind::SetLeave {
                leave: Leave::Prepare,
                given: true
            },
            CommandKind::SetLeave {
                leave: Leave::Launch,
                given: true
            }
        )
    );
}

// ---------------------------------------------------------------- The dates

fn due_unit(raw: u32, name: &str, day: u32) -> Due {
    Due {
        unit: EntityRef::LandUnit(LandUnitId::new(raw)),
        name: name.to_string(),
        kind: DueKind::LandUnit,
        day,
    }
}

#[test]
fn the_dates_are_day_1_and_day_4() {
    let s = scenario();
    let d = dates(&s.world.side_view(ALLIED), &course(), IntentId::new(1), 8);
    assert_eq!(d.first_day, 1);
    assert_eq!(d.last_day, 8);
    assert!(!d.given);
    assert_eq!(
        d.dates,
        vec![
            Date {
                day: 1,
                due: vec![]
            },
            Date {
                day: 4,
                due: vec![due_unit(2, "Placeholder Second", 4)]
            },
        ]
    );
    assert!(d.not_tried.is_empty());
    // Day 1 played for 8 days, to day 8; day 4 for 11, to day 11.
    let main = main_case();
    let ends: Vec<(u32, u32, Vec<usize>)> = main
        .played
        .played
        .iter()
        .map(|date| {
            (
                date.date.day,
                date.shift,
                date.played.worlds.iter().map(|w| w.days.len()).collect(),
            )
        })
        .collect();
    assert_eq!(ends, vec![(1, 0, vec![8; 4]), (4, 3, vec![11; 4])]);
}

// ---------------------------------------------------------------- The worlds

#[test]
fn every_world_holds_the_believed_200_who_dig_10_a_day() {
    let main = main_case();
    for date in &main.played.played {
        for w in &date.played.worlds {
            assert!(w.failed.is_none(), "world {}", w.number);
            assert!(w.refused.is_empty(), "world {}: {:?}", w.number, w.refused);
            assert!(w.assumptions.iter().any(|a| matches!(a,
                Assumption::BelievedMenMade { unit, men: 200, hex, standing: Standing::Present, .. }
                    if unit.raw() == 900_001 && *hex == B)));
            assert!(w
                .assumptions
                .iter()
                .any(|a| matches!(a, Assumption::EnemyGivenNoIntent)));
            assert!(!w.assumptions.iter().any(|a| matches!(
                a,
                Assumption::MenAshore { .. } | Assumption::BelievedMenNotInWorld { .. }
            )));
            // The Japanese side orders nothing.
            assert!(w
                .days
                .iter()
                .all(|d| d.staff.get(&JAPANESE).is_none_or(|s| s.orders.is_empty())));
        }
    }
    // The 200 dig: things as they stand, stepped, the umpire reading each
    // day's end.
    let s = scenario();
    let mut playing = begin(
        &s.world.side_view(ALLIED),
        &believed(&s),
        &figures(&s),
        s.world.ref_data(),
        s.world.map(),
        Asked {
            trial: 1,
            worlds: 1,
            days: 8,
            course: Vec::new(),
        },
    )
    .expect("the trial begins");
    let mut dug = Vec::new();
    while let Some(stepped) = playing.step() {
        let Stepped::Day { day, .. } = stepped else {
            panic!("{stepped:?}");
        };
        let current = playing
            .in_play()
            .or_else(|| playing.played().last())
            .expect("the world just played");
        let truth = world_of(current).umpire();
        let unit = &truth.land_units[&LandUnitId::new(900_001)];
        assert_eq!((unit.men_ashore, unit.hex), (200, Some(B)));
        assert_eq!(unit.posture, Posture::Defend);
        dug.push((day, unit.fieldworks));
    }
    assert_eq!(
        dug,
        (1..=8)
            .map(|day| (day, n(10 * i64::from(day))))
            .collect::<Vec<_>>()
    );
}

// ---------------------------------------------------------------- The first date

#[test]
fn the_first_date_lands_unit_1_on_day_4_and_takes_the_place() {
    let first = &main_case().played.played[0];
    for w in &first.played.worlds {
        let got = orders(w);
        assert_eq!(
            &got[..4],
            &sorted(vec![
                (1, vec![Given::Landing(B, vec![1])]),
                (2, vec![Given::LoadTo(n(45))]),
                (3, vec![Given::Attack(1), Given::LoadUnit(1)]),
                (4, vec![Given::Route(vec![B, A], vec![B]), Given::Unload]),
            ])[..],
            "world {}",
            w.number
        );
        // The fight of day 4: 100 Allied men lost; 300 Japanese, held to the
        // 200 there, cut by cover 30 to 170; the 30 left surrender.
        let (lost, surrendered) = fight_losses(w);
        assert_eq!(
            lost,
            vec![(4, 1, 100), (4, 900_001, 170)],
            "world {}",
            w.number
        );
        assert_eq!(surrendered, vec![(4, 900_001, 30)], "world {}", w.number);
        assert_eq!(met_on(w), Some(4));
        let truth = world_of(w).umpire();
        assert_eq!(men_of(truth, 1), 500);
        assert_eq!(men_of(truth, 2), 600, "unit 2 stays at A");
        assert_eq!(truth.bases[&BaseId::new(2)].side, ALLIED);
        // The plan is marked complete on the morning of day 5.
        let five: Vec<String> = w.days[4]
            .staff
            .get(&ALLIED)
            .map(|s| s.commands.iter().map(|c| format!("{:?}", c.kind)).collect())
            .unwrap_or_default();
        assert_eq!(
            five,
            vec!["MarkPlan { plan: PlanId(1), outcome: Complete }".to_string()]
        );
    }
}

// ---------------------------------------------------------------- The second date

#[test]
fn the_second_date_lands_units_1_and_2_on_day_8_and_takes_the_place() {
    let second = &main_case().played.played[1];
    for w in &second.played.worlds {
        let got = orders(w);
        assert_eq!(
            &got[..8],
            &sorted(vec![
                (1, vec![]),
                (2, vec![]),
                (3, vec![]),
                (4, vec![Given::Landing(B, vec![1, 2])]),
                (5, vec![Given::LoadTo(n(90))]),
                (6, vec![Given::Attack(1), Given::LoadUnit(1)]),
                (7, vec![Given::Attack(2), Given::LoadUnit(2)]),
                (8, vec![Given::Route(vec![B, A], vec![B]), Given::Unload]),
            ])[..],
            "world {}",
            w.number
        );
        // Nothing of the side's commander before day 4.
        let taken: Vec<u32> = world_of(w)
            .events()
            .iter()
            .filter_map(|e| match &e.kind {
                EventKind::CommandTaken {
                    side,
                    issuer: sim_core::orders::Issuer::Player,
                    ..
                } if *side == ALLIED => Some(e.day),
                _ => None,
            })
            .collect();
        assert_eq!(taken, vec![4; 4], "world {}", w.number);
        // The fight of day 8: 50 from each Allied unit; 600 Japanese, held
        // to the 200 there, cut by cover 70 to 130; the 70 left surrender.
        let (lost, surrendered) = fight_losses(w);
        assert_eq!(
            lost,
            vec![(8, 1, 50), (8, 2, 50), (8, 900_001, 130)],
            "world {}",
            w.number
        );
        assert_eq!(surrendered, vec![(8, 900_001, 70)], "world {}", w.number);
        assert_eq!(met_on(w), Some(8));
        let truth = world_of(w).umpire();
        assert_eq!((men_of(truth, 1), men_of(truth, 2)), (550, 550));
    }
}

// ---------------------------------------------------------------- The window

#[test]
fn the_window_of_intent_1() {
    let w = &main_case().window;
    assert_eq!(w.side, Some(ALLIED));
    assert_eq!(w.intent, IntentId::new(1));
    assert_eq!(w.dates.len(), 2);
    let (first, second) = (&w.dates[0], &w.dates[1]);
    // Played to, and taken in on.
    assert_eq!(
        (first.day, first.estimate.last_day, first.taken_in),
        (1, Some(8), Some(1))
    );
    assert_eq!(
        (second.day, second.estimate.last_day, second.taken_in),
        (4, Some(11), Some(4))
    );
    assert_eq!(second.after_first, 3);
    // The men its take chose.
    let chose = |d: &sim_trial::WindowDate| {
        d.chosen.as_ref().map(|c| {
            (
                c.men,
                c.units.iter().map(|(u, _)| u.raw()).collect::<Vec<_>>(),
                c.differs_in.clone(),
            )
        })
    };
    assert_eq!(chose(first), Some((600, vec![1], vec![])));
    assert_eq!(chose(second), Some((1200, vec![1, 2], vec![])));
    // The plan order: no men wanted or to hand, no count being held; the day
    // expected ashore.
    for (date, ashore) in [(first, 4), (second, 8)] {
        let order = date.estimate.plan_order.as_ref().expect("a plan order");
        assert_eq!(order.men, None);
        assert_eq!(date.expected_ashore, Some(ashore));
        assert!(date.estimate.plan_order_differs_in.is_empty());
    }
    // The enemy at B on the eve.
    for (date, eve, least) in [(first, 0, 0), (second, 3, 30)] {
        assert_eq!(date.enemy.len(), 1);
        let e = &date.enemy[0];
        assert_eq!((e.hex, e.eve), (B, eve));
        assert_eq!(e.men, Some(spread(200, 200, 200)));
        assert_eq!(e.least_cover, Some(cover(least, least, least)));
    }
    // Met and kept, and the costs.
    for (date, day) in [(first, 4), (second, 8)] {
        let e = &date.estimate;
        assert_eq!(e.went.met_kept, vec![1, 2, 3, 4]);
        assert_eq!(e.went.met_days, Some(spread(day, day, day)));
        assert!(e.went.lost_again.is_empty() && e.went.not_met.is_empty());
        assert!(e.could_not.is_empty());
        assert_eq!(e.side_costs, Some(costs(spread(100, 100, 100))));
        assert_eq!(e.enemy_costs, Some(costs(spread(200, 200, 200))));
        assert!(e.partings.is_empty());
    }
    // What the second date buys: unit 2, a land unit, usable from day 4.
    assert!(first.due.is_empty());
    assert_eq!(second.due, vec![due_unit(2, "Placeholder Second", 4)]);
    // World for world, the second date against the first.
    assert_eq!(
        w.against_first,
        vec![(
            4,
            WorldForWorld {
                both: vec![1, 2, 3, 4],
                ..WorldForWorld::default()
            }
        )]
    );
}

#[test]
fn its_words() {
    let words = main_case().window.to_string();
    println!("{words}");
    assert!(
        words.contains("Nothing of the course was done before day 4"),
        "{words}"
    );
    assert!(
        words.contains("Placeholder Second (land_unit:2), a land unit, may first be used on day 4")
    );
    assert!(words.contains("4 of 4 worlds"), "{words}");
    let lower = words.to_lowercase();
    assert!(!words.contains('%'));
    for banned in [
        "chance",
        "points",
        "better",
        "should",
        "recommend",
        "advise",
    ] {
        assert!(!lower.contains(banned), "{banned}: {words}");
    }
}

// ---------------------------------------------------------------- Worlds that differ

const MORE: &str = r#"
[[believed_men]]
side = "allied"
name = "More believed at B"
at_base = 2
men = 200
standing = "unknown"
"#;

fn differ() -> &'static Window {
    static DIFFER: OnceLock<Window> = OnceLock::new();
    DIFFER.get_or_init(|| {
        let s = load(&format!("{}\n{MORE}", text()));
        window(&played_window(&s, asked(5, course(), None)))
    })
}

#[test]
fn worlds_that_differ_where_waiting_wins() {
    let w = differ();
    assert_eq!(
        w.dates.iter().map(|d| d.day).collect::<Vec<_>>(),
        vec![1, 4]
    );
    let (first, second) = (&w.dates[0].estimate, &w.dates[1].estimate);
    assert_eq!(first.last_day, Some(5));
    assert_eq!(second.last_day, Some(8));
    let figures = |e: &Estimate| -> Vec<(u32, Option<u32>, u32, u32)> {
        e.worlds
            .iter()
            .map(|w| {
                (
                    w.number,
                    w.met_day,
                    w.side.men_to_enemy,
                    w.enemy.men_to_enemy,
                )
            })
            .collect()
    };
    assert_eq!(
        figures(first),
        vec![
            (1, Some(4), 100, 200),
            (2, None, 200, 254),
            (3, Some(4), 100, 200),
            (4, None, 200, 254)
        ]
    );
    assert_eq!(
        figures(second),
        vec![
            (1, Some(8), 100, 200),
            (2, Some(8), 200, 400),
            (3, Some(8), 100, 200),
            (4, Some(8), 200, 400)
        ]
    );
    assert_eq!(first.went.met_kept, vec![1, 3]);
    assert_eq!(first.went.not_met, vec![2, 4]);
    assert_eq!(second.went.met_kept, vec![1, 2, 3, 4]);
    assert_eq!(first.side_costs, Some(costs(spread(100, 100, 200))));
    assert_eq!(second.side_costs, Some(costs(spread(100, 100, 200))));
    assert_eq!(first.enemy_costs, Some(costs(spread(200, 200, 254))));
    assert_eq!(second.enemy_costs, Some(costs(spread(200, 200, 400))));
    for date in &w.dates {
        assert_eq!(date.enemy[0].men, Some(spread(200, 200, 400)));
    }
    assert_eq!(w.dates[0].enemy[0].least_cover, Some(cover(0, 0, 0)));
    assert_eq!(w.dates[1].enemy[0].least_cover, Some(cover(30, 30, 30)));
    assert_eq!(
        w.against_first,
        vec![(
            4,
            WorldForWorld {
                both: vec![1, 3],
                later_alone: vec![2, 4],
                ..WorldForWorld::default()
            }
        )]
    );
    // Each date's estimate parts its worlds by the 200 of unknown standing.
    assert_eq!(first.partings.len(), 1);
    assert!(matches!(
        &first.partings[0].by,
        sim_trial::PartedBy::BelievedMen { place: 2, .. }
    ));
    assert_eq!(first.partings[0].with.went.worlds, vec![2, 4]);
    assert_eq!(second.partings.len(), 1);
    assert_eq!(second.partings[0].with.went.worlds, vec![2, 4]);
}

#[test]
fn worlds_that_differ_the_fights() {
    let s = load(&format!("{}\n{MORE}", text()));
    let played = played_window(&s, asked(5, course(), None));
    for (k, date) in played.played.iter().enumerate() {
        for w in &date.played.worlds {
            let (lost, surrendered) = fight_losses(w);
            let want = match (k, w.number % 2) {
                (0, 1) => (vec![(4, 1, 100), (4, 900_001, 170)], vec![(4, 900_001, 30)]),
                (0, _) => (
                    vec![(4, 1, 200), (4, 900_001, 127), (4, 900_002, 127)],
                    vec![],
                ),
                (_, 1) => (
                    vec![(8, 1, 50), (8, 2, 50), (8, 900_001, 130)],
                    vec![(8, 900_001, 70)],
                ),
                _ => (
                    vec![
                        (8, 1, 100),
                        (8, 2, 100),
                        (8, 900_001, 130),
                        (8, 900_002, 130),
                    ],
                    vec![(8, 900_001, 70), (8, 900_002, 70)],
                ),
            };
            let mut got = (lost, surrendered);
            got.0.sort_unstable();
            got.1.sort_unstable();
            assert_eq!(got, want, "date {} world {}", date.date.day, w.number);
        }
    }
}

// ---------------------------------------------------------------- Properties

#[test]
fn the_days_before_a_date_are_played_once_and_nothing_is_changed_by_it() {
    let s = scenario();
    let plain = trial(&s, 11, on_day(&commands_of(&text(), COURSE), 4));
    let second = &main_case().played.played[1].played;
    assert_eq!(
        &estimate(&plain, IntentId::new(1), None),
        &main_case().window.dates[1].estimate,
        "field for field"
    );
    assert_eq!(plain.worlds.len(), second.worlds.len());
    for (a, b) in plain.worlds.iter().zip(&second.worlds) {
        let hashes = |w: &PlayedWorld| w.days.iter().map(|d| (d.day, d.hash)).collect::<Vec<_>>();
        assert_eq!(hashes(a), hashes(b), "world {}", a.number);
        assert_eq!(hashes(a).len(), 11);
        assert_eq!(a.taken_in, b.taken_in);
        assert_eq!(a.plan_orders, b.plan_orders);
        assert_eq!(world_of(a).state_hash(), world_of(b).state_hash());
    }
    assert_eq!(plain.leaves_given, second.leaves_given);
}

#[test]
fn the_edge_of_the_horizon() {
    for (day, is_date) in [(8, true), (9, false)] {
        let s = changed("available_from = 4", &format!("available_from = {day}"));
        let d = dates(&s.world.side_view(ALLIED), &course(), IntentId::new(1), 8);
        let days: Vec<u32> = d.dates.iter().map(|d| d.day).collect();
        if is_date {
            assert_eq!(days, vec![1, 8]);
        } else {
            assert_eq!(days, vec![1]);
        }
    }
}

const THIRD: &str = r#"
[[land_unit]]
id = 3
name = "Placeholder Third"
side = "allied"
men = 600
q = 0
r = 0
available_from = DAY
"#;

fn with_units(extra: &[(u32, &str, u32)]) -> String {
    let mut t = text();
    for (id, name, day) in extra {
        t.push_str(
            &THIRD
                .replace("id = 3", &format!("id = {id}"))
                .replace("Placeholder Third", name)
                .replace("DAY", &day.to_string()),
        );
    }
    t
}

#[test]
fn two_things_on_one_day_are_one_date() {
    let t = with_units(&[(3, "Placeholder Third", 4)]);
    let s = load(&t);
    let course = on_day(
        &commands_of(
            &t,
            &COURSE.replace("land_units = [1, 2]", "land_units = [1, 2, 3]"),
        ),
        1,
    );
    let d = dates(&s.world.side_view(ALLIED), &course, IntentId::new(1), 8);
    assert_eq!(
        d.dates,
        vec![
            Date {
                day: 1,
                due: vec![]
            },
            Date {
                day: 4,
                due: vec![
                    due_unit(2, "Placeholder Second", 4),
                    due_unit(3, "Placeholder Third", 4)
                ]
            },
        ]
    );
}

#[test]
fn a_force_back_from_a_yard_is_a_date() {
    let yard = r#"
[[task_force]]
id = 2
name = "Placeholder transport force 2"
side = "allied"
at_base = 1
mission = "transport"

[[ship]]
id = 3
name = "Placeholder troop transport 3"
class = 10
side = "allied"
fuel_t = 1000
task_force = 2

[[order]]
day = 1
side = "allied"
task_force = 2
kind = "send_to_yard"
"#;
    let t = format!(
        "{}\n{yard}",
        common::replace_once(
            &text(),
            "supply_t = 1000\nfuel_t = 0\n",
            "supply_t = 1000\nfuel_t = 0\nyard_passage_days = 2\n"
        )
    );
    let mut live = load(&t);
    let held = staffs();
    live.next_day_staffed(as_staffs(&held))
        .expect("a day resolves")
        .expect("a day");
    let force = &live.world.umpire().task_forces[&TaskForceId::new(2)];
    assert_eq!(force.yard_trip.map(|trip| trip.return_day), Some(3));
    // No forces limit on the course.
    let course: Vec<CourseCommand> = course()
        .into_iter()
        .filter(|c| !matches!(c.command, CommandKind::SetLimit(_)))
        .collect();
    let d = dates(&live.world.side_view(ALLIED), &course, IntentId::new(1), 8);
    assert_eq!(d.first_day, 2);
    let three = d
        .dates
        .iter()
        .find(|d| d.day == 3)
        .expect("day 3 is a date");
    assert_eq!(
        three.due,
        vec![Due {
            unit: EntityRef::TaskForce(TaskForceId::new(2)),
            name: "Placeholder transport force 2".to_string(),
            kind: DueKind::BackFromYard,
            day: 3,
        }]
    );
}

#[test]
fn standing_commands_are_not_shifted() {
    let s = scenario();
    let hold = CommandKind::Hold {
        target: HoldTarget::LandUnit(LandUnitId::new(2)),
    };
    let mut a = asked(8, course(), None);
    a.standing = vec![hold.clone()];
    let played = played_window(&s, a);
    let second = &played.played[1];
    assert_eq!(second.date.day, 4);
    for w in &second.played.worlds {
        let taken: Vec<(u32, CommandKind)> = world_of(w)
            .events()
            .iter()
            .filter_map(|e| match &e.kind {
                EventKind::CommandTaken {
                    side,
                    issuer: sim_core::orders::Issuer::Player,
                    command,
                } if *side == ALLIED => Some((e.day, command.clone())),
                _ => None,
            })
            .collect();
        assert_eq!(taken[0], (1, hold.clone()), "world {}", w.number);
        assert!(taken[1..].iter().all(|(day, _)| *day == 4), "{taken:?}");
        assert_eq!(taken.len(), 5);
    }
}

#[test]
fn dates_may_be_given() {
    let s = scenario();
    let played = played_window(&s, asked(8, course(), Some(vec![1, 3])));
    let w = window(&played);
    assert!(w.given);
    assert_eq!(
        w.dates.iter().map(|d| d.day).collect::<Vec<_>>(),
        vec![1, 3]
    );
    let second = &w.dates[1];
    assert!(second.due.is_empty());
    let chosen = second.chosen.as_ref().expect("a landing formed");
    assert_eq!(
        (
            chosen.men,
            chosen
                .units
                .iter()
                .map(|(u, _)| u.raw())
                .collect::<Vec<_>>()
        ),
        (600, vec![1])
    );
}

#[test]
fn the_same_worlds() {
    let s = scenario();
    let begun = |course: Vec<CourseCommand>, days: u32| {
        begin(
            &s.world.side_view(ALLIED),
            &believed(&s),
            &figures(&s),
            s.world.ref_data(),
            s.world.map(),
            Asked {
                trial: 1,
                worlds: 4,
                days,
                course,
            },
        )
        .expect("the trial begins")
    };
    let first = begun(course(), 8);
    let second = begun(on_day(&commands_of(&text(), COURSE), 4), 11);
    assert_eq!(first.to_play(), second.to_play());
    // And the window's two dates are of the same worlds.
    let main = main_case();
    for (a, b) in main.played.played[0]
        .played
        .worlds
        .iter()
        .zip(&main.played.played[1].played.worlds)
    {
        assert_eq!(a.number, b.number);
        assert_eq!(a.assumptions, b.assumptions);
    }
}

#[test]
fn the_first_date_is_the_estimate() {
    let s = scenario();
    let plain = trial(&s, 8, course());
    assert_eq!(
        &estimate(&plain, IntentId::new(1), None),
        &main_case().window.dates[0].estimate
    );
}

#[test]
fn nothing_of_the_sides_is_due_to_change() {
    let s = changed("available_from = 4\n", "");
    let played = played_window(&s, asked(8, course(), None));
    assert_eq!(played.dates.dates.len(), 1);
    let w = window(&played);
    assert_eq!(w.dates.len(), 1);
    assert!(w.against_first.is_empty());
    let words = w.to_string();
    assert!(
        words.contains("Nothing of the side's that the course may use is due to change by day 8"),
        "{words}"
    );
    let chosen = w.dates[0].chosen.as_ref().expect("a landing formed");
    assert_eq!(
        (
            chosen.men,
            chosen
                .units
                .iter()
                .map(|(u, _)| u.raw())
                .collect::<Vec<_>>()
        ),
        (1200, vec![1, 2])
    );
}

#[test]
fn three_dates_at_most() {
    let t = with_units(&[(3, "Placeholder Third", 6), (4, "Placeholder Fourth", 7)]);
    let s = load(&t);
    let course = on_day(
        &commands_of(
            &t,
            &COURSE.replace("land_units = [1, 2]", "land_units = [1, 2, 3, 4]"),
        ),
        1,
    );
    let d = dates(&s.world.side_view(ALLIED), &course, IntentId::new(1), 8);
    assert_eq!(
        d.dates.iter().map(|d| d.day).collect::<Vec<_>>(),
        vec![1, 4, 6]
    );
    assert_eq!(
        d.not_tried.iter().map(|d| d.day).collect::<Vec<_>>(),
        vec![7]
    );
}

#[test]
fn outside_the_limit_does_not_count() {
    let s = scenario();
    let course = course_text(&COURSE.replace("land_units = [1, 2]", "land_units = [1]"));
    let d = dates(&s.world.side_view(ALLIED), &course, IntentId::new(1), 8);
    assert_eq!(d.dates.iter().map(|d| d.day).collect::<Vec<_>>(), vec![1]);
}

#[test]
fn beyond_the_horizon_does_not_count() {
    let s = changed("available_from = 4", "available_from = 10");
    let d = dates(&s.world.side_view(ALLIED), &course(), IntentId::new(1), 8);
    assert_eq!(d.dates.iter().map(|d| d.day).collect::<Vec<_>>(), vec![1]);
}

#[test]
fn a_date_is_told_before_it_is_played() {
    let s = scenario();
    let view = s.world.side_view(ALLIED);
    let told = dates(&view, &course(), IntentId::new(1), 8);
    let playing = begin_window(
        &view,
        &believed(&s),
        &figures(&s),
        s.world.ref_data(),
        s.world.map(),
        asked(8, course(), None),
    )
    .expect("the window begins");
    assert_eq!(playing.dates(), &told);
    assert_eq!(&told, &main_case().played.dates);
}

#[test]
fn the_same_twice_and_the_live_game_untouched() {
    let s = scenario();
    let before = s.world.state_hash();
    let a = window(&played_window(&s, asked(8, course(), None)));
    let b = window(&played_window(&s, asked(8, course(), None)));
    assert_eq!(a, b);
    assert_eq!(&a, &main_case().window);
    assert_eq!(s.world.state_hash(), before);
}
