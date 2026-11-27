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
