//! Scenario 94: a trial. One course is played in a number of worlds built
//! from a side's picture, for a number of days, with both sides on
//! doctrine; the live game is not touched.
//!
//! Proves decision 0175, step 5 of step 3's stage 4, against a hand
//! calculation. The data is placeholder data: see
//! `data/scenarios/94_a_trial.toml`, which the build writes to the inputs
//! below. It runs on the frozen step 1 figures.
//!
//! # Hand calculation
//!
//! Written by the architect before the code. Everything is asserted
//! exactly. If the code disagrees, the code or the rule is wrong: do not
//! change a number here to make a test pass. A mismatch goes back to the
//! architect.
//!
//! ## Inputs
//!
//! Scenario 86 (see its hand calculation), with three changes:
//!
//! - its three commands of day 1 are taken out of the scenario: they are
//!   the course to be tried, and the live game is given none of them;
//! - it names a file of the trial's figures in which an enemy base is
//!   assumed to hold 0 t of supply and 0 t of fuel, and the three sizes
//!   are as decision 0175 has them;
//! - it names a file of sides that gives every class to both sides.
//!
//! (Added on review.) The build writes those two files, each with its
//! frozen copy: the trial's figures also hold `seed = 1942`, and the
//! file of sides has a row for every class and type scenario 86's
//! reference files give, the troop transport's among them.
//!
//! So the live game is: base 1 at A, (0,0), Allied, 1,000 t of supply;
//! base 2 at B, (4,0), Japanese, with nothing; base 4 at C, (0,6),
//! Allied, with nothing; task force 1, Allied, two troop transports at
//! sea at (2,0), unit 1 (600 fighting men) and 30 t of supply in ship 1
//! and unit 2 (400 working men) in ship 2. The Allied side holds no
//! contact and no estimate and believes nothing. No commands. In the
//! live game nothing is ever ordered.
//!
//! **The course:** for the Allied side, intent 1, control island 1;
//! leave to prepare given; leave to launch given.
//!
//! **The trial:** of the Allied side, from the evening before day 1,
//! seed 1942, trial 1.
//!
//! ## 1. Every world is scenario 86's game
//!
//! The enemy's half of every picture is base 2 with 0 t of supply and
//! 0 t of fuel and nothing else, as scenario 86 has it. The Japanese
//! side is given no intent. No die is rolled in scenario 86. So each
//! world of the trial plays scenario 86's days as its hand calculation
//! gives them.
//!
//! A trial of 4 worlds and 8 days. In each of worlds 1 to 4:
//!
//! - **The orders the Allied staff gives** are scenario 86's eight, on
//!   scenario 86's days: on day 1 the landing at B with unit 1, the
//!   route to B and on to base 1, and the unload order; on day 2 unit 1
//!   set to attack; on day 3 unit 1 set to defend; on day 4 a load of
//!   supply to a weight of 85 t; on day 5 the route to B and back to
//!   base 1, and the unload order. None on days 6 to 8.
//! - **Base 2 is the Allied side's** from the end of day 2, and the take
//!   of B is met on the morning of day 3.
//! - **At the end of day 8:** base 1 holds 915 t, base 2 holds 70 t and
//!   base 5, the base the landing formed, 10 t. Each ship has 944 t of
//!   fuel. Unit 1 has 600 men and unit 2 has 400, all ashore at B.
//! - **The Japanese staff gives no order** on any day.
//!
//! The four worlds hold the same state at the end of every day: ships,
//! task forces, bases, land units and both sides' command states, field
//! for field. (Amended on review: this said they have the same hash. A
//! world's hash holds its trial's number and its own, so the hashes of
//! two worlds are never equal.)
//!
//! ## 2. The live game is not touched
//!
//! The live game's hash is taken before the trial of case 1 and after
//! it: they are equal. The live game is then resolved for 8 days with
//! both staffs: no order is given on any day, task force 1 is at (2,0)
//! at the end, base 2 is Japanese, and its eight daily hashes are those
//! of the same game resolved with no trial ever asked.
//!
//! (Added on review.) And on every evening: the live game is played its
//! 8 days with both staffs, and on each evening from the one before day
//! 1 to the evening of day 7 a trial of 2 worlds for 2 days, with the
//! course of case 1, is asked of the Allied side's view and played out.
//! The live game's eight daily hashes are those of the game never
//! asked.
//!
//! ## 3. The sizes
//!
//! Small is 8 worlds for 7 days, 56 days of play; standard is 16 for 14,
//! 224; full is 64 for 21, 1,344. A trial asked at "small" plays worlds
//! 1 to 8 for days 1 to 7 and no more: each world's day at its end is
//! day 7.
//!
//! ## 4. A world depends on its own number alone
//!
//! With a contact added to the live game, 3 ships taken for a cruiser at
//! (3,2), one pulse old, so that worlds differ in where the force is,
//! and with an empty course:
//! world 3 of a trial of 4 worlds and world 3 of a trial of 8 have the
//! same hash at the end of each of 3 days, and their pictures are equal.
//! In the pictures of worlds 1 to 8 the Japanese force is not at the
//! same hex in all of them. (Amended on review: this compared the
//! worlds' hashes, which differ by their numbers alone.)
//!
//! ## 5. Stopped, and begun again
//!
//! The trial of case 1 is worked to the end of day 2 of world 2 and
//! stopped. Nothing of it is kept: the live game's hash is what it was.
//! The trial is begun again and played out: each world's hash at the end
//! of each day is that of case 1.
//!
//! ## 6. An empty course
//!
//! The trial of case 1 with no command in its course: in each world
//! nothing is ordered by either staff on any of the 8 days, task force 1
//! is at (2,0) at the end, and base 2 is Japanese.
//!
//! # Properties
//!
//! - **The same twice.** The same view, beliefs, figures, seed, trial,
//!   size and course give worlds with the same hash on every day.
//! - **The course goes to the side tried and to no other.** In case 1
//!   the Japanese side's command state holds no intent on any day.
//! - **No seat, no trial inside.** `sim-trial`'s source test passes: it
//!   hands in commands to and resolves only a world it built, and names
//!   no umpire, no whole picture and no view of a world it was not
//!   given.
//! - **A world that cannot be played is reported, and the others are
//!   played.** (Amended on review, which found no means named.) The
//!   trial of case 1 played from its pictures, with world 2's picture
//!   altered by the test so that its world is refused when built (its
//!   Allied counter of forces set below the side's first number): world
//!   2's record has its number, day 0 and the engine's words; worlds 1,
//!   3 and 4 are as in case 1. A day that fails in play is tested inside
//!   the crate, with a day's step made to fail by the test: that world's
//!   record has the day and the days before it, and the other worlds
//!   are played.
//! - **The enemy's intents are handed in once.** (Added on review.) With
//!   the Allied side believing that the Japanese side holds base 2: in
//!   each world of case 1, at the end of day 1 the Japanese side's
//!   command state has exactly one intent, to hold base 2, and no
//!   command of that side was refused.
//! - **Two courses, the same worlds.** (Added on review.) With case 4's
//!   contact: the pictures of worlds 1 to 4 are equal, world for world,
//!   whether the course is case 1's or empty.
//! - **The seed is the trial's.** (Added on review.) The live game
//!   loaded on another seed of its own gives the Allied side the same
//!   pictures and, played out, worlds with the same hashes.
//! - **A command on its day.** (Added on review.) The course of case 1
//!   with each command given day 3: in each world nothing is ordered on
//!   days 1 and 2, and the landing is ordered on day 3.
//! - **A refused command is reported.** (Added on review.) The course of
//!   case 1 with a fourth command, the approval of a plan that does not
//!   exist: each world's record has that command as refused, with the
//!   engine's words, and the world plays as in case 1.
//! - **No live game changes** by the build: no baseline, golden file or
//!   pinned hash moves.

// The tests are written by the build, to the statements above.

mod common;

use std::path::PathBuf;
use std::sync::OnceLock;

use sim_core::command::{CommandKind, IntentKind, Leave, PlanId, PlanState, StaffOrder};
use sim_core::events::EventKind;
use sim_core::fixed::Fixed;
use sim_core::hex::Hex;
use sim_core::ids::{BaseId, IntentId, LandUnitId, ShipId, TaskForceId};
use sim_core::orders::{Cargo, LandUnitOrder, OrderKind, Posture, TaskForceOrder};
use sim_core::refdata::{TrialFigures, TrialSize};
use sim_core::state::{Side, TruthState};
use sim_core::world_picture::PictureCounters;
use sim_data::Scenario;
use sim_staff::{as_staffs, staffs};
use sim_trial::{
    begin, begin_from_pictures, pictures, Asked, Believed, CourseCommand, Played, PlayedWorld,
    Playing, Stepped, Trial,
};

const ALLIED: Side = Side::Allied;
const JAPANESE: Side = Side::Japanese;
const A: Hex = Hex { q: 0, r: 0 };
const B: Hex = Hex { q: 4, r: 0 };
const AT_SEA: Hex = Hex { q: 2, r: 0 };

fn path() -> PathBuf {
    common::data_dir().join("scenarios/94_a_trial.toml")
}

fn text() -> String {
    std::fs::read_to_string(path()).expect("the scenario file reads")
}

fn n(whole: i64) -> Fixed {
    Fixed::from_int(whole)
}

/// Scenario 94 with `extra` added at the end of its file, on the frozen
/// step 1 figures.
fn scenario_with(extra: &str) -> Scenario {
    common::from_text_with(&format!("{}\n{extra}", text()), &path(), &common::frozen())
        .expect("scenario 94 loads")
}

fn scenario() -> Scenario {
    scenario_with("")
}

/// The commands of the course: intent 1, control island 1; leave to
/// prepare; leave to launch.
fn commands() -> Vec<CommandKind> {
    vec![
        CommandKind::SetIntent {
            parent: None,
            intent: IntentId::new(1),
            kind: IntentKind::ControlIsland {
                island: 1,
                by_day: None,
            },
        },
        CommandKind::SetLeave {
            leave: Leave::Prepare,
            given: true,
        },
        CommandKind::SetLeave {
            leave: Leave::Launch,
            given: true,
        },
    ]
}

/// The course of case 1: its three commands on day 1 of the world.
fn course() -> Vec<CourseCommand> {
    on_day(1)
}

/// The course of case 1 with each command on day `day` of the world.
fn on_day(day: u32) -> Vec<CourseCommand> {
    commands()
        .into_iter()
        .map(|command| CourseCommand { day, command })
        .collect()
}

fn asked(worlds: u32, days: u32, course: Vec<CourseCommand>) -> Asked {
    Asked {
        trial: 1,
        worlds,
        days,
        course,
    }
}

/// The world a played world's play left.
fn world_of(played: &PlayedWorld) -> &sim_core::World {
    played.world.as_ref().expect("the world was built")
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
        .expect("scenario 94 names its trial's figures")
}

/// A trial of the Allied side begun on the live game of `scenario` as it
/// stands.
fn begun(scenario: &Scenario, asked: Asked) -> Playing {
    begin(
        &scenario.world.side_view(ALLIED),
        &believed(scenario),
        &figures(scenario),
        scenario.world.ref_data(),
        scenario.world.map(),
        asked,
    )
    .expect("the trial begins")
}

/// What a day of a world left, read by the umpire of the test after the
/// step that played it.
struct Snapshot {
    world: u32,
    day: u32,
    hash: u64,
    truth: TruthState,
    japanese_intents: usize,
    allied_state: sim_core::command::CommandState,
    japanese_state: sim_core::command::CommandState,
}

/// A trial stepped to its end, with a snapshot of each day played.
struct Stepping {
    played: Played,
    days: Vec<Snapshot>,
}

fn step_out(mut playing: Playing) -> Stepping {
    let mut days = Vec::new();
    while let Some(stepped) = playing.step() {
        let Stepped::Day { world, day, hash } = stepped else {
            continue;
        };
        let current = playing
            .in_play()
            .filter(|w| w.number == world)
            .or_else(|| playing.played().last())
            .expect("the world just played");
        assert_eq!(current.number, world);
        days.push(Snapshot {
            world,
            day,
            hash,
            truth: world_of(current).umpire().clone(),
            japanese_intents: world_of(current).command_state(JAPANESE).intents.len(),
            allied_state: world_of(current).command_state(ALLIED).clone(),
            japanese_state: world_of(current).command_state(JAPANESE).clone(),
        });
    }
    Stepping {
        played: playing.play_out(),
        days,
    }
}

fn hashes(played: &Played) -> Vec<(u32, Vec<(u32, u64)>)> {
    played
        .worlds
        .iter()
        .map(|w| (w.number, w.days.iter().map(|d| (d.day, d.hash)).collect()))
        .collect()
}

/// Case 1: 4 worlds, 8 days, the course.
fn case_1() -> &'static Stepping {
    static GAME: OnceLock<Stepping> = OnceLock::new();
    GAME.get_or_init(|| step_out(begun(&scenario(), asked(4, 8, course()))))
}

/// An order of the staff, plainly, as scenario 86's test reads them.
#[derive(Clone, PartialEq, Eq, Debug)]
enum Given {
    Landing(u32, Hex, Vec<u32>),
    LoadTo(u32, Fixed),
    Attack(u32),
    Defend(u32),
    Route(u32, Vec<Hex>, Vec<Hex>),
    Unload(u32),
    Other(String),
}

fn given(o: &StaffOrder) -> Given {
    match &o.kind {
        OrderKind::TaskForce { target, order } => {
            let tf = target.raw();
            match order {
                TaskForceOrder::PrepareLanding { hex, roster } => {
                    Given::Landing(tf, *hex, roster.iter().map(|u| u.raw()).collect())
                }
                TaskForceOrder::LoadTo {
                    cargo: Cargo::Supply,
                    weight_t,
                    ..
                } => Given::LoadTo(tf, *weight_t),
                TaskForceOrder::SetRoute {
                    waypoints,
                    act_at,
                    stand_at,
                } if stand_at.is_empty() => {
                    Given::Route(tf, waypoints.clone(), act_at.iter().copied().collect())
                }
                TaskForceOrder::Unload => Given::Unload(tf),
                other => Given::Other(format!("task force {tf}: {other:?}")),
            }
        }
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

/// The orders `side`'s staff gave on each day of a world, in the order
/// given, sorted by their text so that the comparison is of the set.
fn orders_by_day(world: &PlayedWorld, side: Side) -> Vec<(u32, Vec<String>)> {
    world
        .days
        .iter()
        .map(|d| {
            let mut orders: Vec<String> = d
                .staff
                .get(&side)
                .map(|s| s.orders.iter().map(|o| format!("{:?}", given(o))).collect())
                .unwrap_or_default();
            orders.sort();
            (d.day, orders)
        })
        .collect()
}

/// Scenario 86's eight orders, on scenario 86's days.
fn scenario_86_orders() -> Vec<(u32, Vec<String>)> {
    let day = |d: u32, mut v: Vec<Given>| {
        let mut s: Vec<String> = v.drain(..).map(|g| format!("{g:?}")).collect();
        s.sort();
        (d, s)
    };
    vec![
        day(
            1,
            vec![
                Given::Landing(1, B, vec![1]),
                Given::Route(1, vec![B, A], vec![B]),
                Given::Unload(1),
            ],
        ),
        day(2, vec![Given::Attack(1)]),
        day(3, vec![Given::Defend(1)]),
        day(4, vec![Given::LoadTo(1, n(85))]),
        day(
            5,
            vec![Given::Route(1, vec![B, A], vec![B]), Given::Unload(1)],
        ),
        day(6, vec![]),
        day(7, vec![]),
        day(8, vec![]),
    ]
}

fn base_side(truth: &TruthState, base: u32) -> Side {
    truth.bases[&BaseId::new(base)].side
}

/// The day a plan of the Allied side became `to`, from a world's events.
fn plan_day(world: &sim_core::World, plan: PlanId, to: PlanState) -> Option<u32> {
    world.events().iter().find_map(|e| match &e.kind {
        EventKind::PlanStateChanged {
            side,
            plan: p,
            to: t,
            ..
        } if *side == ALLIED && *p == plan && *t == to => Some(e.day),
        _ => None,
    })
}

// ---------------------------------------------------------------- The inputs

#[test]
fn the_inputs_are_scenario_86_with_its_commands_taken_out() {
    let t = text();
    assert!(!t.contains("[[command]]"));
    assert!(!t.contains("[[order]]"));
    assert!(!t.contains("[[contact]]"));
    assert!(!t.contains("[[believed_"));
    assert!(t.contains("days = 8"));
    let s = scenario();
    let f = figures(&s);
    assert!(f.placeholder);
    assert_eq!(f.enemy_base_supply_t, n(0));
    assert_eq!(f.enemy_base_fuel_t, n(0));
    assert_eq!(f.seed, 1942);
    // Scenario 86's own file, but for its commands, its name, its header
    // and the two files named.
    let eighty_six = common::load(&common::data_dir().join("scenarios/86_fed_with_the_lift.toml"))
        .expect("scenario 86 loads");
    assert_eq!(
        s.world.umpire(),
        eighty_six.world.umpire(),
        "the live game is scenario 86's"
    );
    assert_eq!(s.world.day(), 0);
    assert!(s.world.command_state(ALLIED).intents.is_empty());
}

// ---------------------------------------------------------------- 1. Every world is scenario 86's game

#[test]
fn case_1_every_world_gives_scenario_86s_orders_on_its_days() {
    let g = case_1();
    assert!(g.played.failed().is_empty());
    assert_eq!(
        g.played.worlds.iter().map(|w| w.number).collect::<Vec<_>>(),
        vec![1, 2, 3, 4]
    );
    for w in &g.played.worlds {
        assert_eq!(w.days.len(), 8, "world {}", w.number);
        assert_eq!(
            orders_by_day(w, ALLIED),
            scenario_86_orders(),
            "world {}",
            w.number
        );
        // The Japanese staff gives no order on any day.
        for (day, orders) in orders_by_day(w, JAPANESE) {
            assert!(
                orders.is_empty(),
                "world {} day {day}: {orders:?}",
                w.number
            );
        }
    }
}

#[test]
fn case_1_the_orders_are_those_scenario_86_gives_when_played_itself() {
    // Scenario 86 played as its own file, with both staffs: the same orders
    // on the same days, so the trial's worlds and the scenario's game agree.
    let mut eighty_six =
        common::load(&common::data_dir().join("scenarios/86_fed_with_the_lift.toml"))
            .expect("scenario 86 loads");
    let held = staffs();
    let mut by_day = Vec::new();
    while let Some(summary) = eighty_six
        .next_day_staffed(as_staffs(&held))
        .expect("a day resolves")
    {
        let mut orders: Vec<String> = eighty_six
            .world
            .staff_handed_in()
            .get(&ALLIED)
            .map(|s| s.orders.iter().map(|o| format!("{:?}", given(o))).collect())
            .unwrap_or_default();
        orders.sort();
        by_day.push((summary.day, orders));
    }
    assert_eq!(by_day, scenario_86_orders());
    let end = eighty_six.world.umpire();
    for w in &case_1().played.worlds {
        assert_eq!(orders_by_day(w, ALLIED), by_day, "world {}", w.number);
        let truth = world_of(w).umpire();
        for b in [1, 2, 5] {
            let id = BaseId::new(b);
            assert_eq!(
                truth.bases[&id].supply_t, end.bases[&id].supply_t,
                "world {} base {b}",
                w.number
            );
        }
    }
}

#[test]
fn case_1_base_2_is_allied_from_the_end_of_day_2_and_the_take_is_met_on_day_3() {
    let g = case_1();
    for snap in &g.days {
        let want = if snap.day >= 2 { ALLIED } else { JAPANESE };
        assert_eq!(
            base_side(&snap.truth, 2),
            want,
            "world {} day {}",
            snap.world,
            snap.day
        );
    }
    for w in &g.played.worlds {
        let state = world_of(w).command_state(ALLIED);
        // The take is the staff's intent 2, under intent 1 (scenario 86).
        let take = &state.intents[&IntentId::new(2)];
        assert_eq!(
            take.kind,
            IntentKind::TakePlace {
                hex: B,
                by_day: None
            }
        );
        assert_eq!(
            plan_day(world_of(w), take.plan, PlanState::Complete),
            Some(3),
            "world {}",
            w.number
        );
        // Base 2 changed side on day 2.
        let changed = world_of(w).events().iter().find_map(|e| match &e.kind {
            EventKind::BaseChangedSide { base, .. } if base.raw() == 2 => Some(e.day),
            _ => None,
        });
        assert_eq!(changed, Some(2), "world {}", w.number);
    }
}

#[test]
fn case_1_at_the_end_of_day_8() {
    let g = case_1();
    for w in &g.played.worlds {
        let truth = world_of(w).umpire();
        assert_eq!(world_of(w).day(), 8);
        let supply = |b: u32| truth.bases[&BaseId::new(b)].supply_t;
        assert_eq!(supply(1), n(915), "world {}", w.number);
        assert_eq!(supply(2), n(70), "world {}", w.number);
        assert_eq!(supply(5), n(10), "world {}", w.number);
        // Base 5 is the base the landing formed.
        assert_eq!(truth.bases[&BaseId::new(5)].hex, B);
        assert_eq!(truth.bases[&BaseId::new(5)].side, ALLIED);
        for s in [1, 2] {
            assert_eq!(
                truth.ships[&ShipId::new(s)].fuel_t,
                n(944),
                "world {}",
                w.number
            );
        }
        for (u, men) in [(1, 600), (2, 400)] {
            let unit = &truth.land_units[&LandUnitId::new(u)];
            assert_eq!(unit.men_ashore, men, "world {} unit {u}", w.number);
            assert_eq!(unit.hex, Some(B), "world {} unit {u}", w.number);
        }
    }
}

#[test]
fn case_1_the_four_worlds_hold_the_same_state_at_the_end_of_every_day() {
    let g = case_1();
    for day in 1..=8 {
        let of_day: Vec<&Snapshot> = g.days.iter().filter(|snap| snap.day == day).collect();
        assert_eq!(of_day.len(), 4, "day {day}");
        let first = of_day[0];
        for snap in &of_day[1..] {
            let (a, b) = (&first.truth, &snap.truth);
            let at = format!("world {} day {day}", snap.world);
            assert_eq!(a.ships, b.ships, "{at}: ships");
            assert_eq!(a.task_forces, b.task_forces, "{at}: task forces");
            assert_eq!(a.bases, b.bases, "{at}: bases");
            assert_eq!(a.land_units, b.land_units, "{at}: land units");
            assert_eq!(first.allied_state, snap.allied_state, "{at}: Allied");
            assert_eq!(first.japanese_state, snap.japanese_state, "{at}: Japanese");
        }
    }
    // The steps report the hashes the worlds keep. The hashes of two worlds
    // differ, as a world's hash holds its own number.
    let stepped: Vec<(u32, u32, u64)> = g.days.iter().map(|s| (s.world, s.day, s.hash)).collect();
    let kept: Vec<(u32, u32, u64)> = g
        .played
        .worlds
        .iter()
        .flat_map(|w| w.days.iter().map(move |d| (w.number, d.day, d.hash)))
        .collect();
    assert_eq!(stepped, kept);
}

/// What does hold of the four worlds, and of scenario 86's game: at the end
/// of every day, each world's truth, its day's events and both sides'
/// command states are those of scenario 86 played as its own file with
/// both staffs, so the four worlds differ in nothing but their numbers.
/// They are compared as the umpire's text shows them, in which the number
/// of an event or an order is not shown (decision 0086): a built world's
/// log begins with one event of its own where scenario 86's begins with
/// its setup, so the same event has another number in each.
#[test]
fn case_1_every_world_is_scenario_86s_game_day_for_day() {
    let mut eighty_six =
        common::load(&common::data_dir().join("scenarios/86_fed_with_the_lift.toml"))
            .expect("scenario 86 loads");
    let held = staffs();
    let mut truth = Vec::new();
    while eighty_six
        .next_day_staffed(as_staffs(&held))
        .expect("a day resolves")
        .is_some()
    {
        truth.push(format!("{:?}", eighty_six.world.umpire()));
    }
    let events_of = |world: &sim_core::World, day: u32| -> Vec<String> {
        world
            .events()
            .iter()
            .filter(|e| e.day == day)
            .map(|e| format!("{:?} {:?}", e.pulse, e.kind))
            .collect()
    };
    let g = case_1();
    assert_eq!(g.days.len(), 32);
    for snap in &g.days {
        assert!(
            format!("{:?}", snap.truth) == truth[snap.day as usize - 1],
            "world {} day {}: the truth parts from scenario 86's",
            snap.world,
            snap.day
        );
    }
    for w in &g.played.worlds {
        for day in 1..=8 {
            assert_eq!(
                events_of(world_of(w), day),
                events_of(&eighty_six.world, day),
                "world {} day {day}",
                w.number
            );
        }
        for side in [ALLIED, JAPANESE] {
            assert_eq!(
                format!("{:?}", world_of(w).command_state(side)),
                format!("{:?}", eighty_six.world.command_state(side)),
                "world {} {side:?}",
                w.number
            );
        }
    }
}

// ---------------------------------------------------------------- 2. The live game is not touched

fn live_days(mut scenario: Scenario) -> (Vec<u64>, Vec<usize>, Scenario) {
    let held = staffs();
    let mut orders = Vec::new();
    while scenario
        .next_day_staffed(as_staffs(&held))
        .expect("a day resolves")
        .is_some()
    {
        let given: usize = scenario
            .world
            .staff_handed_in()
            .values()
            .map(|d| d.orders.len())
            .sum();
        orders.push(given);
    }
    (scenario.world.day_hashes().to_vec(), orders, scenario)
}

#[test]
fn case_2_the_live_game_is_not_touched() {
    let live = scenario();
    let before = live.world.state_hash();
    let played = begun(&live, asked(4, 8, course())).play_out();
    assert_eq!(played.worlds.len(), 4);
    assert_eq!(live.world.state_hash(), before);
    let (with_trial, orders, after) = live_days(live);
    assert_eq!(with_trial.len(), 8);
    assert_eq!(orders, vec![0; 8], "no order is given on any day");
    let tf = &after.world.umpire().task_forces[&TaskForceId::new(1)];
    assert_eq!(tf.from_hex, AT_SEA);
    assert_eq!(tf.toward_hex, None);
    assert_eq!(base_side(after.world.umpire(), 2), JAPANESE);
    let (never, ..) = live_days(scenario());
    assert_eq!(with_trial, never);
}

#[test]
fn case_2_a_trial_on_every_evening_changes_nothing() {
    let mut live = scenario();
    let held = staffs();
    while live.world.day() < 8 {
        let played = begun(&live, asked(2, 2, course())).play_out();
        assert_eq!(
            played.worlds.len(),
            2,
            "evening of day {}",
            live.world.day()
        );
        assert!(played.failed().is_empty());
        live.next_day_staffed(as_staffs(&held))
            .expect("a day resolves")
            .expect("a day to resolve");
    }
    let (never, ..) = live_days(scenario());
    assert_eq!(live.world.day_hashes(), never.as_slice());
}

// ---------------------------------------------------------------- 3. The sizes

#[test]
fn case_3_the_sizes() {
    let f = figures(&scenario());
    assert_eq!(f.small, TrialSize { worlds: 8, days: 7 });
    assert_eq!(
        f.standard,
        TrialSize {
            worlds: 16,
            days: 14
        }
    );
    assert_eq!(
        f.full,
        TrialSize {
            worlds: 64,
            days: 21
        }
    );
    assert_eq!(f.small.days_of_play(), 56);
    assert_eq!(f.standard.days_of_play(), 224);
    assert_eq!(f.full.days_of_play(), 1_344);
    // The live trial's figures and their frozen copy give the same sizes.
    for root in [common::data_dir().join("reference"), common::frozen_root()] {
        let live = sim_data::load_trial_figures(&root.join("trial.toml")).expect("trial.toml");
        assert_eq!(live.sizes(), f.sizes(), "{}", root.display());
    }
    let small = f.small;
    let played = begun(&scenario(), asked(small.worlds, small.days, course())).play_out();
    assert!(played.failed().is_empty());
    assert_eq!(
        played.worlds.iter().map(|w| w.number).collect::<Vec<_>>(),
        (1..=8).collect::<Vec<_>>()
    );
    for w in &played.worlds {
        assert_eq!(
            w.days.iter().map(|d| d.day).collect::<Vec<_>>(),
            (1..=7).collect::<Vec<_>>()
        );
        assert_eq!(world_of(w).day(), 7, "world {}", w.number);
    }
}

// ---------------------------------------------------------------- 4. A world depends on its own number alone

const CONTACT: &str = "[[contact]]\nside = \"allied\"\nq = 3\nr = 2\nship_count = 3\nkind = \
                       \"cruiser\"\nage_pulses = 1\n\n";

#[test]
fn case_4_world_3_of_4_is_world_3_of_8() {
    let s = scenario_with(CONTACT);
    let of_4 = begun(&s, asked(4, 3, vec![])).play_out();
    let of_8 = begun(&s, asked(8, 3, vec![])).play_out();
    assert!(of_4.failed().is_empty() && of_8.failed().is_empty());
    let third = |p: &Played| -> Vec<(u32, u64)> {
        p.worlds[2].days.iter().map(|d| (d.day, d.hash)).collect()
    };
    assert_eq!(of_4.worlds[2].number, 3);
    assert_eq!(of_8.worlds[2].number, 3);
    assert_eq!(third(&of_4).len(), 3);
    assert_eq!(third(&of_4), third(&of_8));
    let view = s.world.side_view(ALLIED);
    let make = |worlds: u32| {
        pictures(
            &view,
            &believed(&s),
            &figures(&s),
            Trial {
                seed: figures(&s).seed,
                trial: 1,
                worlds,
            },
        )
        .expect("the pictures are made")
    };
    assert_eq!(make(4)[2], make(8)[2]);
    // In the pictures of worlds 1 to 8 the Japanese force is not at the
    // same hex in all of them.
    let pictures_of_8 = make(8);
    let hexes: std::collections::BTreeSet<Hex> = pictures_of_8
        .iter()
        .map(|p| {
            p.picture
                .truth
                .task_forces
                .values()
                .find(|tf| tf.side == JAPANESE)
                .expect("the contact's force is made")
                .from_hex
        })
        .collect();
    assert!(hexes.len() > 1, "{hexes:?}");
}

// ---------------------------------------------------------------- 5. Stopped, and begun again

#[test]
fn case_5_stopped_and_begun_again() {
    let live = scenario();
    let before = live.world.state_hash();
    let mut playing = begun(&live, asked(4, 8, course()));
    loop {
        let stepped = playing.step().expect("a step");
        if stepped
            == (Stepped::Day {
                world: 2,
                day: 2,
                hash: case_1().played.worlds[1].days[1].hash,
            })
        {
            break;
        }
        assert!(
            matches!(stepped, Stepped::Day { world, .. } if world <= 2),
            "{stepped:?}"
        );
    }
    assert_eq!(playing.played().len(), 1);
    assert_eq!(
        playing.in_play().map(|w| (w.number, w.days.len())),
        Some((2, 2))
    );
    drop(playing);
    assert_eq!(live.world.state_hash(), before);
    let again = begun(&live, asked(4, 8, course())).play_out();
    assert_eq!(hashes(&again), hashes(&case_1().played));
}

// ---------------------------------------------------------------- 6. An empty course

#[test]
fn case_6_an_empty_course_orders_nothing() {
    let played = begun(&scenario(), asked(4, 8, vec![])).play_out();
    assert!(played.failed().is_empty());
    assert_eq!(played.worlds.len(), 4);
    for w in &played.worlds {
        assert_eq!(w.days.len(), 8);
        for side in [ALLIED, JAPANESE] {
            for (day, orders) in orders_by_day(w, side) {
                assert!(orders.is_empty(), "world {} {side:?} day {day}", w.number);
            }
        }
        let truth = world_of(w).umpire();
        let tf = &truth.task_forces[&TaskForceId::new(1)];
        assert_eq!(tf.from_hex, AT_SEA, "world {}", w.number);
        assert_eq!(tf.toward_hex, None, "world {}", w.number);
        assert_eq!(base_side(truth, 2), JAPANESE, "world {}", w.number);
    }
}

// ---------------------------------------------------------------- Properties

#[test]
fn property_the_same_twice() {
    let again = begun(&scenario(), asked(4, 8, course())).play_out();
    assert_eq!(hashes(&again), hashes(&case_1().played));
    for (a, b) in again.worlds.iter().zip(&case_1().played.worlds) {
        assert_eq!(a.days, b.days, "world {}", a.number);
        assert_eq!(a.assumptions, b.assumptions, "world {}", a.number);
    }
}

#[test]
fn property_the_course_goes_to_the_side_tried_and_to_no_other() {
    let g = case_1();
    assert_eq!(g.days.len(), 32);
    for snap in &g.days {
        assert_eq!(
            snap.japanese_intents, 0,
            "world {} day {}",
            snap.world, snap.day
        );
    }
    for w in &g.played.worlds {
        assert!(world_of(w).command_state(JAPANESE).intents.is_empty());
        let one = &world_of(w).command_state(ALLIED).intents[&IntentId::new(1)];
        assert_eq!(
            one.kind,
            IntentKind::ControlIsland {
                island: 1,
                by_day: None
            }
        );
    }
}

#[test]
fn property_a_world_that_cannot_be_played_is_reported_and_the_others_are_played() {
    let live = scenario();
    let mut made = pictures(
        &live.world.side_view(ALLIED),
        &believed(&live),
        &figures(&live),
        Trial {
            seed: figures(&live).seed,
            trial: 1,
            worlds: 4,
        },
    )
    .expect("the pictures are made");
    // The means of the test: world 2's Allied counter of forces set below
    // the side's first number, so that its world is refused when built.
    let first = PictureCounters::first(ALLIED);
    made[1]
        .picture
        .counters
        .entry(ALLIED)
        .or_insert(first)
        .next_task_force = first.next_task_force - 1;
    let played = begin_from_pictures(
        made,
        live.world.ref_data(),
        live.world.map(),
        asked(4, 8, course()),
    )
    .expect("the trial begins")
    .play_out();
    let failed = played.failed();
    assert_eq!(failed.len(), 1);
    assert_eq!((failed[0].world, failed[0].day), (2, 0));
    assert!(
        failed[0].words.starts_with("invalid setup: "),
        "{}",
        failed[0].words
    );
    assert_eq!(played.worlds.len(), 4);
    let two = &played.worlds[1];
    assert_eq!(two.number, 2);
    assert!(two.days.is_empty() && two.world.is_none());
    for (w, want) in played.worlds.iter().zip(&case_1().played.worlds) {
        if w.number != 2 {
            assert!(w.failed.is_none());
            assert_eq!(w.days, want.days, "world {}", w.number);
        }
    }
}

const HOLD_BASE_2: &str = "[[believed_intent]]\nside = \"allied\"\nintent_kind = \"hold\"\nbase \
                           = 2\nplaceholder = true\nnote = \"Scenario 94, a property.\"\n\n";

#[test]
fn property_the_enemys_intents_are_handed_in_once() {
    let s = scenario_with(HOLD_BASE_2);
    let played = begun(&s, asked(4, 1, course())).play_out();
    assert_eq!(played.worlds.len(), 4);
    for w in &played.worlds {
        let world = world_of(w);
        let kinds: Vec<IntentKind> = world
            .command_state(JAPANESE)
            .intents
            .values()
            .map(|i| i.kind)
            .collect();
        assert_eq!(
            kinds,
            vec![IntentKind::Hold {
                base: BaseId::new(2)
            }],
            "world {}",
            w.number
        );
        let refused = world.events().iter().any(
            |e| matches!(&e.kind, EventKind::CommandRefused { side, .. } if *side == JAPANESE),
        );
        assert!(!refused, "world {}", w.number);
    }
}

#[test]
fn property_two_courses_the_same_worlds() {
    let s = scenario_with(CONTACT);
    let with = begun(&s, asked(4, 3, course()));
    let without = begun(&s, asked(4, 3, vec![]));
    assert_eq!(with.to_play().len(), 4);
    assert_eq!(with.to_play(), without.to_play());
}

#[test]
fn property_the_seed_is_the_trials() {
    let live = scenario();
    let other = live.with_seed(77).expect("another seed");
    assert_ne!(other.world.seed(), live.world.seed());
    let a = begun(&live, asked(4, 8, course()));
    let b = begun(&other, asked(4, 8, course()));
    assert_eq!(a.to_play(), b.to_play());
    assert_eq!(hashes(&b.play_out()), hashes(&case_1().played));
}

#[test]
fn property_a_command_on_its_day() {
    let played = begun(&scenario(), asked(4, 8, on_day(3))).play_out();
    assert!(played.failed().is_empty());
    for w in &played.worlds {
        for side in [ALLIED, JAPANESE] {
            for (day, orders) in orders_by_day(w, side).into_iter().take(2) {
                assert!(orders.is_empty(), "world {} {side:?} day {day}", w.number);
            }
        }
        let day_3: Vec<Given> = w.days[2].staff[&ALLIED].orders.iter().map(given).collect();
        assert!(
            day_3.contains(&Given::Landing(1, B, vec![1])),
            "world {}: {day_3:?}",
            w.number
        );
        assert!(w.refused.is_empty());
    }
}

#[test]
fn property_a_refused_command_is_reported() {
    let mut with_fourth = course();
    with_fourth.push(CourseCommand::first_day(CommandKind::Approve {
        plan: PlanId::new(99),
    }));
    let played = begun(&scenario(), asked(4, 8, with_fourth)).play_out();
    assert!(played.failed().is_empty());
    for (w, want) in played.worlds.iter().zip(&case_1().played.worlds) {
        assert_eq!(w.refused.len(), 1, "world {}", w.number);
        let refused = &w.refused[0];
        assert_eq!(refused.day, 1);
        assert_eq!(
            refused.command,
            CommandKind::Approve {
                plan: PlanId::new(99)
            }
        );
        // The engine's words, as its refusal event gives them.
        let words = world_of(w)
            .events()
            .iter()
            .find_map(|e| match &e.kind {
                EventKind::CommandRefused { side, reason, .. } if *side == ALLIED => {
                    Some(reason.to_string())
                }
                _ => None,
            })
            .expect("the refusal is in the world's log");
        assert!(!refused.words.is_empty());
        assert_eq!(refused.words, words);
        // The world plays as in case 1.
        assert_eq!(orders_by_day(w, ALLIED), orders_by_day(want, ALLIED));
        assert_eq!(
            format!("{:?}", world_of(w).umpire()),
            format!("{:?}", world_of(want).umpire()),
            "world {}",
            w.number
        );
    }
}

/// Amended points 4 and 9: the record says which leaves the course gave
/// that the side had not, and whether the count of worlds was even.
#[test]
fn property_the_record_says_what_the_course_gave_and_whether_worlds_were_even() {
    let played = begun(&scenario(), asked(4, 1, course())).play_out();
    assert!(!played.uneven);
    assert_eq!(played.leaves_given, vec![Leave::Launch]);
    let three = begun(&scenario(), asked(3, 1, vec![])).play_out();
    assert!(three.uneven);
    assert_eq!(three.worlds.len(), 3);
    assert!(three.leaves_given.is_empty());
}
