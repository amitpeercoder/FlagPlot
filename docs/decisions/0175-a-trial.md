# 0175: A trial: one course played in a number of worlds

Status: settled by the architect on Nov 19, 2026, inside the plan for step 3, stage 4 (decision 0161, step 5 of its ten, section D, rules 10 to 12), which the designer confirmed. The hand calculation is written. Built on Nov 19, 2026 (As built, below). Its three sizes are placeholders until the designer accepts the times measured by the build.

Summary: A trial takes a course, builds the worlds a side's picture allows, and plays each forward for a number of days with both sides on doctrine. It is given the course as the commands that make it, so that anything a commander could order can be tried. Its size is a number of worlds and of days, fixed and not timed. It can be stopped at any point and leaves nothing. It reads the side's view and what the side believed and changes nothing in the live game: the live game's hash is the same after it

Standing: Settled by the architect, Nov 19, 2026, inside decision 0161. Built on Nov 19, 2026. The sizes wait for the designer

## Why

Decisions 0170, 0171, 0173 and 0174 give a side a picture of a world, and worlds built from it with an enemy made by rule. Nothing yet plays them. Decision 0161, rules 10 to 12, says what a trial is: one course, a number of worlds, a number of days, both sides on doctrine, a size that is work and not time, nothing changed.

What is so today, read from the code:

- **A world is resolved for a day with both staffs** by one call, and the staff of each side is the doctrine's own. A day of the 39 days with both staffs takes of the order of 20 to 30 ms on the named machine (decision 0153's proof; decision 0100's measure).
- **A side's commands are handed in before a day** and taken in at its first step: an intent stated, a leave given, a plan approved, a limit set. A draft a seat previews is made of the same commands (decision 0107).
- **`sim-trial` makes pictures and builds their worlds** and may not yet resolve one: its source test forbids it.

## The rules

1. **A course is the commands that make it.** A trial is given, for the side tried, a list of commander's commands: those that state the draft and give it its way (an intent, a limit, a leave, an approval). They are handed in for that side on the first morning of every world. The side's command state is in its own half of the picture as it is (decision 0171), so its other plans go on beside the course and draw on the same men, ships and stocks. An empty course tries things as they stand.
2. **A trial has a size:** a number of worlds and a horizon in days. Three are named:

   | Size | Worlds | Days |
   | --- | --- | --- |
   | small | 8 | 7 |
   | standard | 16 | 14 |
   | full | 64 | 21 |

   Each count of worlds is a power of two, so that the forces of unknown standing are tried evenly (decision 0174, 14). The figures are placeholders of the trial's figures file. They are set by what is asked, never by a clock: the same question at the same size is the same work on any machine.
3. **It plays each world in the order of its number,** from 1: builds the world of its picture (decision 0174), hands in the enemy's believed intents and the course, and resolves the days of the horizon with both sides' staffs on doctrine. No seat plays in a world of a trial, and no trial is asked for inside one.
4. **A world's play depends on its own number alone.** World 3 of a trial of 8 is world 3 of the same trial of 64, day for day: no world reads another, and none reads how many there are.
5. **A trial can be stopped.** It is worked a world's day at a time, and whoever asked may stop asking. What was played is dropped with it: nothing of an unfinished trial is kept, and a trial begun again is the trial it would have been.
6. **A trial changes nothing.** It is given a side's view, that side's beliefs, the figures, the reference data and the map, and builds worlds of its own. Nothing of the live game is written, and its hash after a trial is its hash before.
7. **What it gives back** is the worlds as played, each with its number, its picture's assumptions and its days, for the estimate to be read from (the next step). A world of a trial is the trial's own invention: reading it is not reading the enemy.
8. **A day that fails in a world** stops that world and is reported with its number and day. The other worlds are played. A trial with a failed world says so wherever it is read.
9. **Its cost is measured, once,** on the named machine, with the staffs and all: a trial of the Allied side's "control Guadalcanal" from the evening before day 1 of the 39 days, and from the evening of day 20, at each size. `sim-cli time` gains the measure. The times are written here when built, and the designer accepts them or changes the sizes.

## What it leaves for later

- **The estimate** read from the worlds: step 6. **Dates and what-ifs:** steps 7 and 8, which give a trial other courses.
- **Worlds played side by side.** A trial plays its worlds one after another. They are independent (item 4), so they could be played at once; that is for when the times ask for it, and must not change a result.
- **A trial inside a day.** A picture is of an evening (decision 0170, 3).

## Rule card

- **Reads:** a side's view; its believed forces and intents; the trial's figures; the reference data and the map; a seed, a trial's number, a size; a course.
- **Writes:** nothing in any game. Worlds of its own, dropped with the trial.
- **Reuses:** the pictures and worlds of decision 0174; a day resolved with both staffs; a commander's commands.
- **New:** in `sim-trial`, the playing of a trial; three sizes in the trial's figures file; one measure in `sim-cli time`.

## What it costs

- **No live game changes:** no state, no draw of a live game, no hash.
- **`sim-trial`'s source test** lets it hand in commands to, and resolve, a world it built, and nothing more.
- **The rulebook's chapter on the trial** gains what a trial is and its sizes; `docs/commands.md` the measure.

## The hand calculation

It is in `tests/scenario_94_a_trial.rs`, written before the code. Its main case is scenario 86 tried from the evening before its first day, with the course its own three commands and an enemy base assumed empty: with no enemy and no die, every world of the trial is scenario 86's own game, order for order and ton for ton, and the take is met on day 3 in each. Beside it: the live game untouched; a trial stopped and begun again; world 3 of 4 and of 8; and an empty course, in which nothing is ordered.

## As built

Built on Nov 19, 2026, on the branch `a-trial-build`, from the head of `a-trial` with main merged in. Every case and property of the hand calculation holds but one statement of case 1, which cannot hold on the code as decision 0170 built it (reading 1): "the four worlds have the same hash at the end of every day". Its test is kept as written and ignored, and a test beside it shows what holds instead. No number or statement of the hand calculation was changed.

**The times** (rule 9), on the named machine, `--machine "Apple M3 Pro (11 cores, 36 GB, macOS 27.2)"`, release build, Nov 19, 2026, `sim-cli time --only-trial`: a trial of the Allied side's "control Guadalcanal" (the first Allied intent of `data/scenarios/stage-3-proof/07a_slice.toml` with its two leaves, reading 13), timed from the making of the pictures to the dropping of the worlds, with both staffs. The machine was not quiet: other sessions' desktop programs ran beside it (load averages 6.5 to 10.4 on 11 cores; no other heavy job of this build). A trial runs on one core. Three runs; the middle value, then the spread:

| Size | Worlds x days | Days of play | From the evening before day 1 | From the evening of day 20 |
| --- | --- | --- | --- | --- |
| small | 8 x 7 | 56 | 0.68 s (0.64 to 0.73) | 0.53 s (0.51 to 0.55) |
| standard | 16 x 14 | 224 | 2.09 s (1.92 to 2.13) | 1.93 s (1.93 to 1.94) |
| full | 64 x 21 | 1,344 | 10.6 s (10.0 to 10.8) | 11.5 s (10.5 to 12.4) |

About 8 to 9 ms a day of play, staffs and all. Every world of every trial played every day: no world failed. The whole measure, the full trials keeping their 64 worlds to the end, peaked at about 300 MB. The sizes are unchanged: the designer decides on the times.

**What was built.**

- **`sim-trial`** gains the playing of a trial (`src/play.rs`). Its public surface: `Asked { trial, days, course }`, what a trial is asked to play; `begin(view, believed, figures, ref_data, map, asked) -> Result<Playing, Refused>`, which makes the pictures of every world and plays nothing; `play(...)`, the same played out; `Playing`, with `step() -> Option<Stepped>` (a world's day), `in_play()`, `played()`, `not_built()` and `play_out() -> Played`; `Stepped::{Day { world, day, hash }, Failed(Failed), NotBuilt(Failed)}`; `Played { worlds, not_built }` with `failed()`; `PlayedWorld { number, assumptions, days, failed, world }`; `PlayedDay { day, hash, staff }`; `Failed { world, day, error }`; and `TrialSize`, re-exported. Both staffs are `sim_staff::staffs()`, made once for a trial.
- **`sim-core`:** `refdata::TrialSize { worlds, days }` with `days_of_play()`; `TrialFigures` gains `small`, `standard` and `full`, `sizes()`, and `problem()` refuses a size of no day or whose worlds are not a power of two. Under the feature `test-means` only, `World::break_an_invariant_for_a_test` (reading 9). No state, hash, event or draw changed.
- **`sim-data`:** `load_trial_figures` reads the tables `[small]`, `[standard]` and `[full]`, each `worlds` and `days` as whole numbers; every key is required (reading 5).
- **`sim-cli time`** gains workload h and `--trial-slice`, `--trial-figures`, `--trial-late-day` and `--only-trial` (reading 13); `sim-cli` depends on `sim-trial`.
- **Data:** the three sizes in `data/reference/trial.toml` and its frozen copy, placeholders; `data/reference/trial_placeholder_scenario_94.toml` (0 t of supply, 0 t of fuel, the three sizes) and `data/reference/class_sides_placeholder_scenario_94.csv` (every placeholder class and type, the two transport classes as well, to both sides), each with a frozen copy; `data/scenarios/94_a_trial.toml`, scenario 86 with its commands taken out and the two files named.
- **Tests:** `tests/scenario_94_a_trial.rs`, 15 tests to the hand calculation (one ignored, reading 1); scenario 93's test gains the three sizes in the one place it writes the trial's figures by hand; `sim-data`'s unit test of the figures file the sizes and their refusals.
- **Generated:** `crates/sim-cli/tests/run_baseline/94_a_trial.txt` and `.events.txt`, new. `sh scripts/regenerate.sh` changed no other file: no run baseline, agent golden, recorded game, pinned hash or batch fixture moved. Nothing records the hash of a trial's figures file: scenario 93's baselines did not move with `trial.toml`.

**Readings the record did not make.**

1. *The four worlds and their hashes (case 1).* A built world's hash covers its domain, the trial and the world (decision 0170, 12, and its "What it costs"), so worlds 1 to 4 have four hashes on every day though they play alike. The statement's test is kept and ignored (`case_1_the_four_worlds_have_the_same_hash_at_the_end_of_every_day`). What holds, asserted beside it: at the end of every day each world's truth, the day's events and both sides' command states are scenario 86's, played as its own file with both staffs, as the umpire's text shows them. They are compared as text because the number of an event or an order is not shown in it (decision 0086): a built world's log begins with one event of its own, where scenario 86's begins with its setup, so the same event has another number. The trial's worlds do not part from scenario 86's game on any day: the made base 2 (no stock, no gun, an airfield with nothing at it, the map's port) plays as scenario 86's base 2, order for order and ton for ton. For the architect: whether the statement should say "the same truth" or a hash without the domain.
2. *Case 4's "not all the same hash"* holds, but by the worlds' numbers alone (reading 1). The test also asserts what the case means: the contact's force is not in the same hex in all 8 pictures.
3. *"The take of B is met on the morning of day 3"* is asserted as its plan's completion on day 3, which the staff marks at its step at the start of the day, before the night pulse, as scenario 86's test has it.
4. *The course is handed in* after the world is built and after what the enemy is taken to want, for the first morning of every world, as one list in the order given; with an empty course nothing is handed in.
5. *The sizes* are in the trial's figures file, three tables, each required, as `enemy_base_supply_t` and `enemy_base_fuel_t` are: a file without them is refused. The loader also refuses a size of no day, and a count of worlds that is not a power of two (rule 2's reason), 0 among them. A trial may be asked for any count of worlds and any days of at least one; a horizon of no day is refused by `begin`.
6. *The horizon is the trial's, not the scenario's.* A built world has no last day; from the evening of day 20 of the 39 days, the full size plays days 21 to 41.
7. *What a day keeps:* its number, the hash at its end, and what each side's staff handed in (`World::staff_handed_in`). The world itself is kept whole to the end of its play, as the "worlds as played" of rule 7, for the estimate to read; whether the estimate needs less is step 6's.
8. *A world that cannot be built* from its picture is reported as failed on its first day, kept in `Played::not_built`, and named by `Played::failed`. None does in any test.
9. *The means of the test for a failed world.* A feature `test-means` in `sim-core` and `sim-trial`, enabled only by the scenario tests' manifest: `World::break_an_invariant_for_a_test` gives the lowest-numbered base a yard passage of 0 days, a figure read only when a ship goes to the yard, which none does in scenario 94, and `Playing::break_world_on(world, day)` calls it before that day of that world is resolved, so that the day fails its check at its end. No normal build compiles either (`cargo build -p sim-cli` does not enable the feature); `cargo test --workspace` and `cargo clippy --workspace --all-targets` do, through the tests' manifest.
10. *What the source test now allows* (`crates/sim-trial/tests/source.rs`): `World` as a word of its own only on a line that imports it, calls `World::from_picture(`, returns `Result<World, SimError>`, or is the field `pub world: World,`; so no function takes a world, and every world the crate holds it built. Banned outright, beyond decision 0174's list: `submit_orders`, `submit_staff`, `open_day`, `resolve_next_pulse`, `close_day`, `resolve_day_watched`, `resolve_day(`, `as_if_day_closed`, `day_hashes`, `events()`, `orders()` and `command_state(`. So what it does to a world is `submit_commands` and `resolve_day_staffed`, and it reads the world's `day()`, `staff_handed_in()` and the hash of a day's summary.
11. *A trial stepped and one played out are the same:* `play_out` is the remaining steps. A world's day is the unit of a step; the first step of a world builds it and hands in the commands.
12. *Rule 6* holds by construction: a trial borrows the view only while its pictures are made, and holds clones of the reference data and the map, never the live world.
13. *The measure.* The full slice (`07_full_slice.toml`) states no intent and has no islands, so the measure tries stage 3's proof's arrangement (a), `stage-3-proof/07a_slice.toml`, the 39 days with the intents as stated, whose first Allied intent is "control island 1", Guadalcanal: the course is that intent and the Allied side's two leaves of day 1, three commands. The live game from the evening of day 20 is that file played to it with its own commands and both staffs; there the Allied side already holds intent 1, and the course states it again, which is taken (its kind unchanged). The slice names no trial's figures, so `data/reference/trial.toml` is read. The trial is number 1 on the scenario's seed. The time runs from the making of the pictures to the dropping of the worlds. The command checks the live game's hash after each trial.
14. *Built worlds record breakdowns* (decision 0170's constructor keeps the default). Turning them off in a trial would make it cheaper; not done, as no record asks it.

**Documents.** Chapter 9 of the rulebook gains "Playing a trial" (rules 1 to 9: the course, the sizes, the playing, the number alone, stopping, nothing changed, what it gives back, a failed day, the measure) and its opening and "What is built" say so; the glossary gains "course (of a trial)" and "size (of a trial)" and amends "trial" and "trial's figures"; `docs/commands.md` the workload h and its options, and the sizes in the `trial` key; `docs/engineering/layout.md` the crate's row and boundary, the feature, and `sim-cli`'s use of the crate; the frozen figures' README the new copies and the sizes.
