# 0237: A base formed by a march is a command

Status: ruled by the architect on Nov 27, 2026, on Amit's word that the defect found in `docs/validation/old-switches.md` be fixed. Class 3 (ruling 1). Amended the same day on an independent reading of pull request 769, which found the cause complete and the rule right and asked for three things named and one said (rulings 3, 4 and 8, and the note under rule 2). It is built only after Amit's word; it reaches Amit as a plan because it moves one file of a closed proof.

Summary: With command levels on, a base a march forms (a dump) was never made a command, so the first rule that read its picture stopped the game ("base:8 is not a command of the Allied side", scenario 29 with command levels turned on). It becomes a command when it is formed, as a base formed by a landing does, with a copy of the picture of the command the marching unit left. With command levels off nothing changes. One file of stage 2b's closed proof, the week, forms a dump and moves; the proof stands as run.

Standing: Ruled by the architect on Nov 27, 2026, on Amit's word; class 3; amended on an independent reading the same day; not built

## Why

Decision 0134, item 4, says each base of a side is a command, and its reading 4 says a base formed in play is read as a task force formed in play is. The build made the hook for a landing (`cargo.rs` calls `World::base_formed`) and none for a march (`march.rs`, where "Dump at (q,r)" is put into the state, decision 0094, items 9 and 10). So with command levels on a dump is a base of its side in truth and no command of it. `command_at_hex`, `command_of_land_unit` and `command_of_base` all name it as the command, and:

- `World::picture` stops the game for a command that is not one. The fight ashore's report that a hex is empty (`estimate.rs`, `report_empty`) reads the picture of the side's base in the hex, and that is the stop found.
- `World::apply_report` quietly gives the area command a report whose holder is not a command. So a ground count in a dump's hex is held first by the area command where the rulebook says the base holds it.
- The dump makes no state report (decision 0136) and is in nobody's reach for a report passed sideways.

**What was measured** (Nov 27, 2026, main at b5d31a7a, `sim-cli run` of each of the 263 scenario files as written, with `command_levels = true` put at the head of a copy of each of the 254 that does not state it):

- One stops: `29_skills.toml`. The stop is at the end of day 3, the day unit 4 arrives at (2,3) and base 8 is formed, not on day 1 as the list said (`run` prints nothing until the game ends).
- Four more form a dump that is then no command, and do not stop: `07_full_slice.toml`, `25_march.toml`, `37_men_falling_back.toml`, `45_soak_ashore.toml`.
- Of the eight files that state command levels on, one forms a dump as written: `stage-2b-proof/07a_week.toml`, base 10 for the Japanese side at (2,-4) on day 16. It does not stop. The list's "no scenario on file has command levels on with a base that is not there at the start" is wrong by this file. The other seven form none.

Only orders written in the files were played. A seat's own orders to march were not tried: any march that carries supply into a hex with no base of the side forms a dump.

## What was weighed

- **Have `picture_of` give the area command's picture for a command that is not one.** Not chosen. It hides this defect and every later one of its kind; the stop is what found it. Decision 0134 says a rule that asks for the picture of a command that is not one is a bug in the rules.
- **Have `command_at_hex` and its two fellows pass over a base that is no command.** Not chosen: a dump would then be a base of the side that is under no command, against decision 0134, item 4.
- **Copy the area command's picture,** as a base that changes side does. Not chosen: decision 0134, item 5, gives a command formed in play the picture of the command that formed it, and a dump is formed by men of a known command.
- **Class 2, as a mend to what decision 0134 already says.** It is that in substance, and would be class 2 if no file with command levels on formed a dump. One does (ruling 1).

## The rule as planned

1. **A base formed by a march** (chapter 7, "The supply is put down") is, with command levels on, a command of its side under the area command from the moment it is formed.
2. **Its picture starts as a copy** of the picture of the command the marching unit belonged to in the hex it left: the lowest-numbered base of its side there, else the area command. Where that command is no longer one, the area command's. It has heard what that picture has heard, every contact in it is one it was told of, and it has no commander: as every command formed by copy (decisions 0135, 13 and 0137, 4 and 5). A march cannot reach "else the area command" today: a unit carries only what it takes from its side's bases in the hex it leaves, and one that carries nothing forms no base, so a dump's unit always left a base of its side. The words are kept for a base lost on the day and for any later way of carrying; they are tested only by calling the hook.
3. **Where reports travel** the area command holds the dump's state from the moment it is formed, as it stands then, before the supply carried is put into it (decision 0136, 5; a base formed by a landing is held the same way, before its rations are landed). From then on it reports its state as every base command does.
4. **With command levels off** no command exists and nothing changes.

Nothing else is new. The rulebook's tables already give a ground count and "none found" to "the base of the side in that hex", and a land unit to the base of its side in its hex; with rule 1 the code does so for a dump.

The test file is `tests/command_levels_ashore.rs`, with one statement in `crates/sim-core/src/pictures_tests.rs`. The rulebook section is chapter 1, "Commands and their pictures", the paragraph "A force formed".

## Rule card

- **Reads:** the bases in truth; the hex the marching unit left; the commands of the side and their pictures.
- **Writes:** the commands of the side (`World::chain`: one command added, with its picture and what it has heard); where reports travel, what the area command holds of its commands (`World::held`). Truth is not written.
- **Events:** none new, and no event's fields change. `MarchBaseFormed` is as it was.
- **Draws:** none new. Where reports travel a dump takes the draws every base command takes (its state report, the passing of its reports), each keyed on the command as now.
- **Versions:** the state layout is not raised (no field is added; the commands are hashed already). The rules label is raised by one (ruling 3).
- **Who knows what:** nothing a side is given changes with command levels off. With them on, a side's own dump has a picture, shown as every command's is.

## Rulings

1. **Class 3.** The change adds no state, draw or event kind, and with command levels off, which is every game for play and 255 of the 263 files, nothing moves. But `stage-2b-proof/07a_week.toml` has command levels on, has reports travel and forms a dump on day 16; from that day the dump reports its state and is told reports, so the file's events move. That is a game of a closed proof moved, which decision 0233, item B, makes class 3.
2. **Stage 2b's proof stands as run** and is not run again. The week was its condition 6, reported and not judged (decision 0138, item 13). `docs/validation/stage-2b-proof.md` is not changed.
3. **The rules label is raised by one,** from 35 to 36: a saved game with command levels on and a march that forms a dump would not replay. With it change, and only so:
   - the comment on `RULES_VERSION` in `world.rs`, which ends at 34: it gains the clause it lacks for 35 ("35 from decision 0232 (a sub-intent the commander writes, a command the old rules refused)") and the clause for 36 ("36 from decision 0237 (with command levels on a base formed by a march is a command, which changes a game with command levels on in which a march forms one)");
   - `assert_eq!(sim_core::world::RULES_VERSION, 35)` in `tests/scenario_122_hungry_men_are_slow_to_mend.rs` and `tests/scenario_123_a_fight_costs_supply.rs` (`property_no_new_state` in each), to 36, and the comment above each, which gains "and decision 0237 to 36, for a base formed by a march as a command" before "none touches this rule". These two values follow the constant and are not hand calculations; commit e536d81c is the precedent. No other expected value in either file changes;
   - rules chapter 1, the paragraph on what a saved game records, which stops at 32: it gains one sentence before "A game saved under an earlier layout or version is refused, not converted.": "Decision 0237 makes the rules label 36 (with command levels on a base formed by a march is a command); the state's layout is unchanged, and no game with command levels off plays differently." and `0237` at the end of its list of decisions. Its missing sentences for 33 to 35 are not this record's and are left;
   - the recorded games kept at the current label, remade by `scripts/regenerate.sh`: each differs in its label and in nothing else, since none has command levels on. `stage_2b_seat_sonnet`, kept at label 18, is left as it is.
4. **What may move, and nothing else:** the row of `stage-2b-proof/07a_week.toml` in `BEFORE` in `tests/scenario_130/part_f.rs`, remade as that file's doc comment says. The two texts (`FLAGPILOT_130_TEXTS`, before and after) are shown to the architect: no line differs before the `MarchBaseFormed` of base 10, and after it the differences are the dump's reports and what follows from them. No run baseline, no line of `tests/golden/pinned_hashes.txt` and no other row of `BEFORE` moves. These tests play the moved game or one like it with command levels on, and each passes with no word of it changed:
   - `every_report_has_a_road_the_state_agrees_with` in `crates/sim-cli/src/events.rs` (the second proof's week with command levels and reports on, seed 1);
   - `a_seat_at_each_command_of_a_long_game…` in `crates/sim-agent/tests/seat.rs` (seats at bases 1 to 10; 10 is the dump's number);
   - the week's on-is-off on seeds 1 to 3 in `tests/scenario_61_commands_and_their_pictures.rs`;
   - the "certain is step N" tests on the week in `tests/scenario_62_reports_between_commands.rs` and `tests/scenario_63_a_superiors_knowledge.rs`.

   If any of these fails, or anything else moves, the build stops and comes back: a failing one is a finding for the architect, not a value to remake.
5. **The call is made where the base is put into the state,** before `MarchBaseFormed` is recorded, as `cargo.rs` does for a landing. The command left is worked out from the hex left before anything else of the arrival changes.
6. **Corrections to `docs/validation/old-switches.md`** are those under "What was measured". The list is left as Amit read it; this record is the correction.
7. **Decision 0236, ruling 18, is met** when this is merged: command levels may then be turned on in a further scenario, by a record of its own.
8. **Rule 3 has a statement of its own,** in `crates/sim-core/src/held_tests.rs` (written out in the test file's doc comment): with report figures, the area command holds the dump as it stood when it was formed, with no supply, though supply is put into it after.
9. **The sweep compares events, not only stops.** Each of the 254 files that does not state `command_levels` is played with the line added, and each of the 9 that states it as written, before the mend and after. A file whose events differ is named, with its first differing line. Expected to differ: `stage-2b-proof/07a_week.toml` alone among the nine as written (`stage-2b-proof/04_savo_island.toml` has command levels on and no other check, and must not); among the 254 with the line added, only files in which a march forms a base may differ, and those without report figures must not.
10. **The as-built note records** the old and the new digest of the moved row, and that condition 6 of `docs/validation/stage-2b-proof.md` reproduces only at b5d31a7a or earlier.

## What it leaves out

- An invariant that every base of a side is a command of it while command levels are on. Worth having; it is a check added to `invariants.rs` and a record of its own if wanted.
- Retiring the switch (number 1 of the list), which waits on the interface's design.
- A seat's marches in the eight files with command levels on: not tried here, and mended by the same rule.

## Questions for the designer

None. The plan is shown because one file of a closed proof moves.
