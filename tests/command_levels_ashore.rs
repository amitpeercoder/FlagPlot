//! Command levels ashore: a base formed by a march is a command.
//!
//! Proves decision 0237 (`docs/decisions/0237-a-base-formed-by-a-march-is-a-command.md`),
//! rules 1, 2 and 4. Rule 2's source of the copy is proved in
//! `crates/sim-core/src/pictures_tests.rs` (below, "In `sim-core`"); rule 3
//! is `World::base_formed`, proved for a landing already.
//!
//! No scenario of record is edited. Each game here is the text of a
//! scenario file with the line `command_levels = true` put in by the test
//! as its first line, loaded from text on the frozen
//! step 1 figures as written, as `tests/scenario_29_skills.rs` loads its
//! `from_frozen` and `tests/scenario_25_march.rs` its `from`. No file names
//! report figures, so reports do not travel: every report reaches every
//! picture of the side at once (decision 0134, 10).
//!
//! # Hand calculation
//!
//! Written by the architect before the code. Everything is asserted
//! exactly. If the code disagrees, the code or the rule is wrong: do not
//! change a statement here to make a test pass. A mismatch goes back to the
//! architect.
//!
//! ## Scenario 29 with command levels on
//!
//! The inputs are scenario 29's (`tests/scenario_29_skills.rs`, "Inputs"),
//! on the frozen figures as written: every skill's strength is 0, so a
//! day's march counts 1 for every unit, and `march_days_per_hex` is 3,
//! `march_supply_days` 5, `daily_supply_t_per_man` 0.005.
//!
//! Unit 4, 200 men, marches from M2 (1,3) to P (2,3). It arrives at the end
//! of day 3, carrying 5 x 0.005 x 200 = 5 t taken from base 3, the Allied
//! base at M2. The Allied side has no base at P and the highest base is 7,
//! so base 8, "Dump at (2,3)", is formed on day 3. Units 1, 2 and 3 march
//! from M1 to M2, where base 3 stands, and form no base. No Japanese man is
//! ever at P, so base 8 never changes side.
//!
//! **The commands.** At the start the Allied side has nine: its area
//! command; task forces 1, 2 and 3; bases 1, 2, 3, 4 and 6. The Japanese
//! side has three: its area command; bases 5 and 7. Every one but the two
//! area commands is under its side's area command. No force sails, so
//! task forces 1, 2 and 3 are commands to the end.
//!
//! | End of day | Base 8 |
//! | --- | --- |
//! | 1, 2 | No such base; no such command |
//! | 3 to 8 | A command of the Allied side, under the Allied area command; not a command of the Japanese side |
//!
//! **Statements.**
//!
//! 1. The game plays its 8 days. (Before the mend it stops at the end of
//!    day 3: "base:8 is not a command of the Allied side".)
//! 2. The commands at the start are the twelve above, each under its
//!    side's area command.
//! 3. Base 8 is as the table has it at the end of each of the 8 days.
//! 4. At the end of every day, each side's commands are exactly its area
//!    command, its task forces with a ship, and every base that is the
//!    side's in truth at that moment; and every base of a side in truth is
//!    a command of that side and of no other. (The fights at F and G are
//!    the dice's; this holds whoever holds those bases.)
//! 5. At the end of day 3 and of every later day, base 8's picture holds
//!    the ground estimates that base 3's picture holds, and that the
//!    Allied side's own belief holds: the same hexes, men, grades and days.
//! 6. No `GroundFoundEmpty` is recorded at P on any day: the Allied side
//!    holds no estimate for P. (This is the reading that stopped the game:
//!    with the mend it reads base 8's picture, finds no estimate there and
//!    makes no report.)
//! 7. Unit 4 is ashore at P with 200 men at the end of day 3, and
//!    `MarchBaseFormed` on day 3 names base 8, the Allied side, P and unit
//!    4, as with command levels off.
//!
//! # Properties
//!
//! - **On is off, scenario 29.** The same text without the line gives the
//!   same events, in the same order with the same numbers, the same truth
//!   and the same belief of each side at the end of every day. (The day
//!   hashes are not compared: the commands are hashed.) With it off no
//!   command exists on any day.
//! - **Scenarios 25 and 37 with command levels on** (`25_march.toml`, 5
//!   days; `37_men_falling_back.toml`, 6 days), the two other short
//!   scenarios of record in which a march forms a base. Each plays to its
//!   end; statement 4 holds at the end of every day; at least one
//!   `MarchBaseFormed` is recorded, and the base each names is, at the end
//!   of that day, a command of the side that holds it in truth then; and
//!   on is off, as above.
//!   (Before the mend statement 4 fails for the base formed.)
//!
//! # What is not in this file
//!
//! - **Every scenario of record is unchanged.** Nothing here states it:
//!   `run_output_matches_the_golden_files` and the pins of
//!   `tests/golden/pinned_hashes.txt` do, and the pull request shows that no
//!   baseline and no pinned line changed. The one game that moves is the
//!   row of `stage-2b-proof/07a_week.toml` in `tests/scenario_130/part_f.rs`
//!   (decision 0237, ruling 4).
//!
//! # In `sim-core`
//!
//! One test in `crates/sim-core/src/pictures_tests.rs`, beside
//! `a_base_formed_by_a_landing_is_a_command_with_a_copy_of_the_landing_forces_picture`,
//! in that file's world with command levels on (Allied base 1 at (0,0),
//! Allied task forces 1 and 2, Japanese base 2 at (5,0)). It calls the one
//! hook `march.rs` calls, with the base it made and the hex left: a march
//! is not played there.
//!
//! - Base 1's picture alone is told of 120 men at (3,3); the Allied area
//!   command's picture alone is told of 40 men at (4,4).
//! - **From a hex with a base of the side.** A base of the Allied side is
//!   put into the state at (1,1) and the hook is called with the hex left,
//!   (0,0). The new base is a command of the Allied side under the area
//!   command, and its picture holds what base 1's holds: the 120 men at
//!   (3,3) and nothing at (4,4).
//! - **From a hex with no base of the side.** A second base of the Allied
//!   side is put into the state at (2,2) and the hook is called with the
//!   hex left, (6,6), where no base stands. It is a command under the area
//!   command and its picture holds what the area command's holds: the 40
//!   men at (4,4) and nothing at (3,3).
//! - **From a hex with only the enemy's base.** A third is put in at (2,3)
//!   and the hook called with the hex left, (5,0), where only Japanese base
//!   2 stands. Its picture is the Allied area command's.
//! - **With command levels off** the hook makes no command.
//!
//! The third case cannot come about by a march today (decision 0237, the
//! note under rule 2): it is proved by the hook alone.
//!
//! And one test in `crates/sim-core/src/held_tests.rs`, beside
//! `a_command_formed_is_held_as_of_now_and_a_base_changing_side_moves_its_hold`,
//! in that file's world with report figures (`holding(ON_TIME)`), for rule
//! 3 and ruling 5:
//!
//! - A base of the Allied side is put into the state at (1,1) with no
//!   supply, and the hook is called with the hex left, (0,0), base 1's.
//!   The Allied area command holds the new base at once, as of the moment
//!   the hook ran (the start, in that world), with 0 t of supply.
//! - 5 t is then put into the base in truth, as the march does after the
//!   hook. What the area command holds of it is unchanged: 0 t.
//! - The Japanese side holds nothing of it.

use crate::common;

use std::collections::BTreeSet;
use std::path::PathBuf;

use sim_core::events::{Event, EventKind};
use sim_core::hex::Hex;
use sim_core::ids::{BaseId, LandUnitId};
use sim_core::pictures::CommandRef;
use sim_core::state::{Side, TruthState};
use sim_core::World;
use sim_data::Scenario;

const P: Hex = Hex::new(2, 3);
const SIDES: [Side; 2] = [Side::Allied, Side::Japanese];

fn path(file: &str) -> PathBuf {
    common::data_dir().join("scenarios").join(file)
}

fn text(file: &str) -> String {
    std::fs::read_to_string(path(file)).expect("the scenario file reads")
}

/// The text with command levels put in as its first line.
fn on(text: &str) -> String {
    format!("command_levels = true\n{text}")
}

fn load(file: &str, source: &str) -> Scenario {
    common::from_text(source, &path(file)).expect("the variant loads on the frozen figures")
}

/// What a side's commands were at one moment.
struct Commands {
    /// Each command and its superior.
    posts: Vec<(CommandRef, Option<CommandRef>)>,
    /// Each command's ground estimates, as its picture holds them.
    pictures: Vec<(CommandRef, String)>,
}

struct Snapshot {
    truth: TruthState,
    /// Per side: the commands, or `None` with command levels off.
    commands: [Option<Commands>; 2],
    /// Per side: the side's own belief, as text.
    belief: [String; 2],
    /// Per side: the side's own ground estimates, as text.
    own_estimates: [String; 2],
}

fn snapshot(world: &World) -> Snapshot {
    let commands = SIDES.map(|side| {
        world.commands(side).map(|posts| Commands {
            posts: posts.iter().map(|(c, p)| (*c, p.superior())).collect(),
            pictures: posts
                .iter()
                .map(|(c, p)| {
                    let held: Vec<_> = p.picture().ground_estimates().collect();
                    (*c, format!("{held:?}"))
                })
                .collect(),
        })
    });
    let belief = SIDES.map(|side| {
        let view = world.side_view(side);
        let contacts: Vec<_> = view.contacts().collect();
        let estimates: Vec<_> = view.ground_estimates().collect();
        let enemy_bases: Vec<BaseId> = view.enemy_bases().map(|b| b.id).collect();
        format!("{contacts:?} {estimates:?} {enemy_bases:?}")
    });
    let own_estimates = SIDES.map(|side| {
        let estimates: Vec<_> = world.side_view(side).ground_estimates().collect();
        format!("{estimates:?}")
    });
    Snapshot {
        truth: world.umpire().clone(),
        commands,
        belief,
        own_estimates,
    }
}

struct Run {
    /// The start (day 0) and the end of each day.
    days: Vec<Snapshot>,
    events: Vec<Event>,
}

impl Run {
    fn of(mut scenario: Scenario) -> Run {
        let mut days = vec![snapshot(&scenario.world)];
        while scenario.next_day().expect("a day resolves").is_some() {
            days.push(snapshot(&scenario.world));
        }
        let events = scenario.world.events().iter().cloned().collect();
        Run { days, events }
    }

    fn len(&self) -> u32 {
        u32::try_from(self.days.len() - 1).expect("a few days")
    }

    fn commands(&self, day: u32, side: Side) -> &Commands {
        self.days[day as usize].commands[side as usize]
            .as_ref()
            .expect("command levels are on")
    }

    fn is_command(&self, day: u32, side: Side, command: CommandRef) -> bool {
        self.commands(day, side)
            .posts
            .iter()
            .any(|(c, _)| *c == command)
    }

    fn picture(&self, day: u32, side: Side, command: CommandRef) -> &str {
        &self
            .commands(day, side)
            .pictures
            .iter()
            .find(|(c, _)| *c == command)
            .expect("a command")
            .1
    }

    fn formed(&self) -> Vec<&Event> {
        self.events
            .iter()
            .filter(|e| matches!(e.kind, EventKind::MarchBaseFormed { .. }))
            .collect()
    }
}

fn base(raw: u32) -> CommandRef {
    CommandRef::Base(BaseId::new(raw))
}

/// Statement 4, at the end of `day`: each side's commands are its area
/// command, its task forces with a ship and every base that is its own in
/// truth; and a base of a side is a command of that side and of no other.
fn commands_are_whole(run: &Run, day: u32) {
    let truth = &run.days[day as usize].truth;
    for side in SIDES {
        let mut expected: BTreeSet<CommandRef> = BTreeSet::new();
        expected.insert(CommandRef::Area);
        for force in truth.task_forces.values() {
            if force.side == side && !force.ships.is_empty() {
                expected.insert(CommandRef::TaskForce(force.id));
            }
        }
        for b in truth.bases.values() {
            if b.side == side {
                expected.insert(CommandRef::Base(b.id));
            }
        }
        let held: BTreeSet<CommandRef> = run
            .commands(day, side)
            .posts
            .iter()
            .map(|(c, _)| *c)
            .collect();
        assert_eq!(held, expected, "day {day}, {side:?}");
        for b in truth.bases.values() {
            let other = if side == Side::Allied {
                Side::Japanese
            } else {
                Side::Allied
            };
            if b.side == other {
                assert!(
                    !run.is_command(day, side, CommandRef::Base(b.id)),
                    "day {day}: base {} is not a command of {side:?}",
                    b.id.raw()
                );
            }
        }
    }
}

/// Everything a day shows that command levels must not change: the events
/// (in order, with their numbers), the truth and each side's belief at the
/// end of every day.
fn same_game(a: &Run, b: &Run) {
    assert_eq!(a.len(), b.len());
    assert_eq!(format!("{:?}", a.events), format!("{:?}", b.events));
    for day in 0..=a.len() as usize {
        assert!(a.days[day].truth == b.days[day].truth, "truth, day {day}");
        assert_eq!(a.days[day].belief, b.days[day].belief, "belief, day {day}");
    }
}

#[test]
fn scenario_29_with_command_levels_on_plays_its_eight_days() {
    let source = text("29_skills.toml");
    let run = Run::of(load("29_skills.toml", &on(&source)));

    // 1. The game plays its 8 days.
    assert_eq!(run.len(), 8);

    // 2. The twelve commands at the start, each under its side's area
    // command.
    let area = Some(CommandRef::Area);
    let tfs = |raw: u32| CommandRef::TaskForce(sim_core::ids::TaskForceId::new(raw));
    let allied: Vec<(CommandRef, Option<CommandRef>)> = vec![
        (CommandRef::Area, None),
        (tfs(1), area),
        (tfs(2), area),
        (tfs(3), area),
        (base(1), area),
        (base(2), area),
        (base(3), area),
        (base(4), area),
        (base(6), area),
    ];
    let japanese: Vec<(CommandRef, Option<CommandRef>)> =
        vec![(CommandRef::Area, None), (base(5), area), (base(7), area)];
    let mut got = run.commands(0, Side::Allied).posts.clone();
    got.sort();
    let mut want = allied;
    want.sort();
    assert_eq!(got, want);
    let mut got = run.commands(0, Side::Japanese).posts.clone();
    got.sort();
    let mut want = japanese;
    want.sort();
    assert_eq!(got, want);

    // 3. Base 8 as the table has it.
    for day in 1..=8 {
        let allied_has = run.is_command(day, Side::Allied, base(8));
        let japanese_has = run.is_command(day, Side::Japanese, base(8));
        assert!(!japanese_has, "day {day}");
        assert_eq!(allied_has, day >= 3, "day {day}");
        if allied_has {
            let superior = run
                .commands(day, Side::Allied)
                .posts
                .iter()
                .find(|(c, _)| *c == base(8))
                .expect("a command")
                .1;
            assert_eq!(superior, area, "day {day}");
        }
    }

    // 4. The commands are whole at the end of every day.
    for day in 0..=8 {
        commands_are_whole(&run, day);
    }

    // 5. Base 8's picture is base 3's and the side's own belief.
    for day in 3..=8 {
        let own = &run.days[day as usize].own_estimates[Side::Allied as usize];
        assert_eq!(
            run.picture(day, Side::Allied, base(8)),
            run.picture(day, Side::Allied, base(3)),
            "day {day}"
        );
        assert_eq!(run.picture(day, Side::Allied, base(8)), own, "day {day}");
    }

    // 6. No `GroundFoundEmpty` at P.
    assert!(!run
        .events
        .iter()
        .any(|e| matches!(&e.kind, EventKind::GroundFoundEmpty { hex, .. } if *hex == P)));

    // 7. Unit 4 is ashore at P with 200 men; `MarchBaseFormed` names base 8.
    let unit4 = LandUnitId::new(4);
    let at_3 = &run.days[3].truth.land_units[&unit4];
    assert_eq!(at_3.hex, Some(P));
    assert_eq!(at_3.men_ashore, 200);
    let formed = run.formed();
    assert_eq!(formed.len(), 1);
    assert_eq!(formed[0].day, 3);
    assert_eq!(
        formed[0].kind,
        EventKind::MarchBaseFormed {
            base: BaseId::new(8),
            side: Side::Allied,
            hex: P,
            unit: unit4,
        }
    );
}

#[test]
fn scenario_29_with_command_levels_on_is_scenario_29_with_them_off() {
    let source = text("29_skills.toml");
    let with = Run::of(load("29_skills.toml", &on(&source)));
    let without = Run::of(load("29_skills.toml", &source));
    same_game(&with, &without);
    for day in 0..=without.len() as usize {
        for side in SIDES {
            assert!(without.days[day].commands[side as usize].is_none());
        }
    }
}

#[test]
fn scenarios_25_and_37_with_command_levels_on_keep_every_base_a_command() {
    for (file, days) in [("25_march.toml", 5), ("37_men_falling_back.toml", 6)] {
        let source = text(file);
        let run = Run::of(load(file, &on(&source)));
        assert_eq!(run.len(), days, "{file}");
        for day in 0..=days {
            commands_are_whole(&run, day);
        }
        let formed = run.formed();
        assert!(!formed.is_empty(), "{file}: a march forms a base");
        for event in formed {
            let EventKind::MarchBaseFormed { base: id, .. } = &event.kind else {
                unreachable!()
            };
            let holder = run.days[event.day as usize].truth.bases[id].side;
            assert!(
                run.is_command(event.day, holder, CommandRef::Base(*id)),
                "{file}: base {} on day {}",
                id.raw(),
                event.day
            );
        }
        let without = Run::of(load(file, &source));
        same_game(&run, &without);
    }
}
