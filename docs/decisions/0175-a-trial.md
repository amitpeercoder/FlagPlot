# 0175: A trial: one course played in a number of worlds

Status: settled by the architect on Nov 19, 2026, inside the plan for step 3, stage 4 (decision 0161, step 5 of its ten, section D, rules 10 to 12), which the designer confirmed. The hand calculation is written. Amended on review on Nov 19, 2026. Built on Nov 19, 2026 (As built, below). Its three sizes are placeholders until the designer accepts the times measured by the build.

Summary: A trial takes a course, builds the worlds a side's picture allows, and plays each forward for a number of days with both sides on doctrine. It is given the course as the commander's commands that make it, each on its day, so that an order, a limit, a leave or an approval can be tried. Its size is a number of worlds and of days, fixed and not timed. It can be stopped at any point and leaves nothing. It reads the side's view and what the side believed and changes nothing in the live game: the live game's hash is the same after it

Standing: Settled by the architect, Nov 19, 2026, inside decision 0161, and amended on review the same day. Built on Nov 19, 2026. The sizes wait for the designer

## Why

Decisions 0170, 0171, 0173 and 0174 give a side a picture of a world, and worlds built from it with an enemy made by rule. Nothing yet plays them. Decision 0161, rules 10 to 12, says what a trial is: one course, a number of worlds, a number of days, both sides on doctrine, a size that is work and not time, nothing changed.

What is so today, read from the code:

- **A world is resolved for a day with both staffs** by one call, and the staff of each side is the doctrine's own. A day of the 39 days with both staffs takes of the order of 20 to 30 ms on the named machine (decision 0153's proof; decision 0100's measure).
- **A side's commands are handed in before a day** and taken in at its first step: an intent stated, a leave given, a plan approved, a limit set. A draft a seat previews is made of the same commands (decision 0107).
- **`sim-trial` makes pictures and builds their worlds,** and the building of a world already hands in the enemy's believed intents and his leaves (decision 0174, 19, as built). It does not yet resolve one: decision 0174 left that for this step.

## The rules

1. **A course is the commands that make it.** A trial is given, for the side tried, a list of commander's commands: those that state the draft and give it its way (an intent, a limit, a leave, an approval). They are handed in for that side on the first morning of every world. The side's command state is in its own half of the picture as it is (decision 0171), so its other plans go on beside the course and draw on the same men, ships and stocks. An empty course tries things as they stand.
2. **A trial has a size:** a number of worlds and a horizon in days. Three are named:

   | Size | Worlds | Days |
   | --- | --- | --- |
   | small | 8 | 7 |
   | standard | 16 | 14 |
   | full | 64 | 21 |

   Each count of worlds is a power of two, so that the forces of unknown standing are tried evenly (decision 0174, 14). The figures are placeholders of the trial's figures file. They are set by what is asked, never by a clock: the same question at the same size is the same work on any machine.
3. **It plays each world in the order of its number,** from 1: builds the world of its picture, which hands in the enemy's believed intents once (decision 0174, 19); hands in the course; and resolves the days of the horizon with both sides' staffs on doctrine. No seat plays in a world of a trial, and no trial is asked for inside one.
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

## Amended on review, Nov 19, 2026

The plan was read by a reviewer who had not seen it written, against decision 0161, the code and scenario 86's data, before its build was far along. What follows is the architect's, and is part of the rules above. The hand calculation is amended with it.

1. **A course's commands each have a day of the world,** 1 where none is given, and are handed in on the morning of that day (rule 1). The window of decision 0161, F, will hand in the same course on a later day.
2. **A course is commands, not orders to units.** A commander's direct order to a force is not in a course in this record. It is left for the step that gives a seat the tool.
3. **What a side has handed in for the morning and the engine has not yet taken in is not in its half** (decision 0170, reading 9). Whoever asks for a trial puts it at the head of the course. A trial asked of a view does not know of it.
4. **Leave is the side's, not a plan's.** A course that gives leave to launch gives it to every plan of the side, as the command does in the live game. Where a course gives a leave the side has not given, the trial's record says so, for the estimate to name.
5. **A command of the course that a world refuses** is in that world's record with the engine's words, and the world plays on without it.
6. **The course is no part of a picture or of a draw's key.** The same view, beliefs, figures, seed and trial's number give the same pictures and the same draws for any course, so that two courses are tried against the same worlds, world for world.
7. **The seed is the trial's, never the live game's.** It is a figure of the trial's figures file, `seed`. The trial's number is given by whoever asks; the same number is the same worlds. Two live games that differ only in their own seed and in nothing a side can see give that side the same trial.
8. **The horizon is played whole.** A world does not know a scenario's last day, and a trial of 21 days from day 20 of the 39 plays to day 41.
9. **A size is a name for two numbers,** read from the trial's figures. A trial may also be asked for by its numbers. A count of worlds that is not a power of two is played, and the forces of unknown standing are then not tried evenly: the record of the trial says so.
10. **What a trial gives back** (rule 7), for each world: its number; its picture's assumptions; for each day played, the world's hash at its end and what each side's staff handed in that day; any command of the course refused; and the world as it stands at the end. It is for the estimate and for the umpire, and no seat is given it. How the estimate reads a world of a trial is the next step's to settle.
11. **A world that cannot be played** (rule 8): one whose picture is refused when its world is built, or one of whose days the engine fails. Its record has its number, the day (0 where the picture was refused), the engine's words, and the days it did play. The other worlds are played.
12. **A world is named in `sim-trial` only as one it built.** Its source test is to hold that: no call of the crate takes a world from outside.
13. **The measure** (rule 9) is on the first arrangement of stage 3's proof, `data/scenarios/stage-3-proof/07a_slice.toml`, in which each side has its two orders to control an island. From the evening before day 1 the course is the Allied side's own commands of day 1 in that file. From the evening of day 20 the live game is first played 20 days with both staffs and its script, and the course is empty: its orders stand. Peak memory is given with the times where the tool can read it: a trial keeps each world at its end, with its own copy of the map and the reference data.
14. **Nothing changes, on every evening** (decision 0161, the arithmetic of its cases): the hand calculation now asks a trial on each evening of the live game, not on the first alone.
15. **Left for later, noted now:** the staff cannot ask for a trial itself, since `sim-trial` depends on `sim-staff`; where decision 0161, 20, has the staff attach a trial to a plan, the session that holds the game will run it.

## As built

Built on Nov 19, 2026, on the branch `a-trial-build`, from the head of `a-trial`, to the record and hand calculation as amended on review (`a-trial-amended`, merged in), with main merged in. Every case and property of the amended hand calculation holds (`tests/scenario_94_a_trial.rs`, 22 tests, and one test inside the crate, `crates/sim-trial/src/play_tests.rs`, for a day that fails). No number or statement of the hand calculation was changed. The build had begun on the record before review; what it had made to the first text and the review changed (a means of the test in a feature of `sim-core`, a power of two required by the loader, the live game's seed, a course of commands with no day) is gone.

**The times** (rule 9, amended point 13), on the named machine, `--machine "Apple M3 Pro (11 cores, 36 GB, macOS 27.2)"`, release build, Nov 19, 2026, `sim-cli time --only-trial`, on `data/scenarios/stage-3-proof/07a_slice.toml`: from the evening before day 1 with the Allied side's own four commands of day 1 as the course; from the evening of day 20, the live game played 20 days with both staffs and its own commands, with an empty course; the trial's seed 1942, trial 1, both staffs; timed from the making of the pictures to the dropping of the worlds. The machine was not quiet: other sessions' desktop programs ran beside it (load averages 6.2 to 11.7 on 11 cores; no other heavy job of this build). A trial runs on one core. Three runs; the middle value, then the spread:

| Size | Worlds x days | Days of play | From the evening before day 1 | From the evening of day 20 | Peak memory |
| --- | --- | --- | --- | --- | --- |
| small | 8 x 7 | 56 | 0.83 s (0.83 to 0.85) | 0.51 s (0.51 to 0.51) | 35 MB |
| standard | 16 x 14 | 224 | 2.93 s (2.92 to 2.93) | 1.83 s (1.83 to 1.85) | 75 MB |
| full | 64 x 21 | 1,344 | 16.0 s (15.9 to 16.0) | 10.5 s (10.5 to 10.5) | 318 MB |

About 13 to 15 ms a day of play from the first evening, when every plan of both sides is being made and run, and 8 ms from day 20. Every world of every trial played every day: none failed. Peak memory is the whole process's (`/usr/bin/time -l`, maximum resident set) for a run of one size from both evenings (`--trial-size`), the live games included: `sim-cli time` cannot read it itself without a dependency it does not have. The sizes are unchanged: the designer decides on the times.

**What was built.**

- **`sim-trial`** gains the playing of a trial (`src/play.rs`). Its public surface: `CourseCommand { day, command }` with `first_day(command)`; `Asked { trial, worlds, days, course }`; `begin(view, believed, figures, ref_data, map, asked) -> Result<Playing, Refused>`, which makes the pictures on the figures' seed and plays nothing; `play(...)`, the same played out; `begin_from_pictures(pictures, ref_data, map, asked)`, which plays pictures handed in; `Playing`, with `step() -> Option<Stepped>` (a world's day), `to_play()` (the pictures not yet begun), `in_play()`, `played()` and `play_out() -> Played`; `Stepped::{Day { world, day, hash }, Failed(Failed)}`; `Played { worlds, uneven, leaves_given }` with `failed()`; `PlayedWorld { number, assumptions, days, refused, failed, world: Option<World> }`; `PlayedDay { day, hash, staff }`; `RefusedCommand { day, command, words }`; `Failed { world, day, words }`; and `TrialSize`, re-exported. Both staffs are `sim_staff::staffs()`, made once for a trial. `Trial`, `pictures` and `build_world` are decision 0174's, unchanged.
- **`sim-core`:** `refdata::TrialSize { worlds, days }` with `days_of_play()`; `TrialFigures` gains `seed`, `small`, `standard` and `full`, and `sizes()`; `problem()` refuses a size of no world or no day. Nothing else in `sim-core` changed: no state, hash, event or draw.
- **`sim-data`:** `load_trial_figures` reads `seed` and the tables `[small]`, `[standard]` and `[full]`, each `worlds` and `days` as whole numbers; every key is required.
- **`sim-cli time`** gains workload h and `--trial-slice`, `--trial-figures`, `--trial-late-day`, `--trial-size` and `--only-trial`; `sim-cli` depends on `sim-trial` (`docs/engineering/layout.md` says so).
- **Data:** `seed = 1942` and the three sizes in `data/reference/trial.toml` and its frozen copy, placeholders; `data/reference/trial_placeholder_scenario_94.toml` (0 t of supply, 0 t of fuel, `seed = 1942`, the three sizes) and `data/reference/class_sides_placeholder_scenario_94.csv` (every class and type scenario 86's reference files give, classes 1 to 6, 8, 10 and 11 and types 1 to 4, to both sides), each with a frozen copy; `data/scenarios/94_a_trial.toml`, scenario 86 with its commands taken out and the two files named.
- **Tests:** `tests/scenario_94_a_trial.rs`; `crates/sim-trial/src/play_tests.rs`; scenario 93's test gains the seed and the sizes in the one place it writes the trial's figures by hand; `sim-data`'s unit test of the figures file the seed, the sizes and their refusals.
- **Generated:** `crates/sim-cli/tests/run_baseline/94_a_trial.txt` and `.events.txt`, new. `sh scripts/regenerate.sh` changed no other file: no run baseline, agent golden, recorded game, pinned hash or batch fixture moved. Nothing records the hash of a trial's figures file: scenario 93's baselines did not move with `trial.toml`.

**Readings the record did not make.**

1. *Case 1, the worlds and scenario 86.* Beside the statements, a test compares each world's truth, each day's events and both command states, day by day, with scenario 86 played as its own file with both staffs, as the umpire's text shows them (the text hides the number of an event or an order, decision 0086; a built world's log begins with one event of its own where scenario 86's begins with its setup, so the same event has another number). The trial's worlds do not part from scenario 86's game on any day: the made base 2 (no stock, no gun, an airfield with nothing at it, the map's port) plays as scenario 86's base 2, order for order and ton for ton. The four worlds' states are compared field for field, as the amended statement says.
2. *"The take of B is met on the morning of day 3"* is asserted as its plan's completion on day 3, which the staff marks at its step at the start of the day, as scenario 86's test has it.
3. *A course's day* is the day of the world, from 1, the day after the picture's evening; its commands of a day are handed in, in the order of the course, just before that day is resolved. A command for day 0 or a day past the horizon refuses the trial (`begin` and `begin_from_pictures`), since it would never be handed in.
4. *A command refused* is found in the day's own events: a `CommandRefused` of the side tried whose issuer is the commander, which only the course hands in; its words are the refusal's own (`CommandRefusal`'s text). The source test, which banned a world's events with the first text's lint, lets the crate read the events of a world it built for this.
5. *Amended point 4, the leaves:* `Played::leaves_given` names each leave the course gives (`set_leave`, given) that the side's own half had not given, in the course's order, once each. In scenario 94 the side holds leave to prepare and not to launch, so the course's leave to launch is named.
6. *Amended point 9:* `Played::uneven` is true where the count of worlds is not a power of two. The loader no longer asks the sizes to be powers of two; the three in the files are.
7. *Amended point 11:* a world whose picture is refused is kept in `Played::worlds` in its place, with no world (`world: None`), no day, and `Failed { day: 0, words }`; `Stepped::Failed` reports it at the step that begins it. A failed day keeps the days before it and the world as the last of them left it (the engine leaves a world unchanged when a day fails its invariants).
8. *The means of the tests of a world that cannot be played.* For a picture refused: scenario 94's test plays the pictures of case 1 through `begin_from_pictures`, world 2's with its Allied counter of task forces set one below the side's first number; the engine refuses it ("invalid setup: ..."). For a failed day: a test inside the crate (`play_tests.rs`) sets a field that exists only in the crate's test build (`#[cfg(test)] fail_on`), which makes world 2's second day fail in place of resolving it; nothing of it is in a normal build or in any other crate's build. That file is compiled only for the crate's own tests and reads a scenario through `sim-data`, a dev-dependency; the source test does not read files whose names end `_tests.rs`.
9. *What the source test now allows* (`crates/sim-trial/tests/source.rs`): `World` as a word of its own only on a line that imports it, calls `World::from_picture(`, returns `Result<World, SimError>`, or is the field `pub world: Option<World>,`; so no call of the crate takes a world, and every world it holds it built (amended point 12). Banned outright, beyond decision 0174's list: `submit_orders`, `submit_staff`, `open_day`, `resolve_next_pulse`, `close_day`, `resolve_day_watched`, `resolve_day(`, `as_if_day_closed`, `day_hashes`, `orders()` and `command_state(`. So what it does to a world is `submit_commands` and `resolve_day_staffed`, and it reads the world's `day()`, its `events()` for the day just resolved, `staff_handed_in()` and the hash of a day's summary.
10. *The same twice, stopped, two courses, the seed:* `play_out` is the remaining steps; `Playing::to_play` gives the pictures before a step, and the properties compare them, so that two courses, or two live seeds, are shown to give the same worlds before a day is played.
11. *What a world keeps* is the world itself, whole, to the end of its play, with its own copy of the map and the reference data, as amended point 13 says; the measure's peak memory shows the cost. Whether the estimate needs less is step 6's.
12. *Rule 6* holds by construction: a trial borrows the view only while its pictures are made and holds clones of the reference data and the map, never the live world. Case 2 asks a trial on every evening of scenario 94 and the live game's daily hashes are those of a game never asked.
13. *The measure.* The course from the first evening is all four of the Allied side's own commands of day 1 in `07a_slice.toml` (its two intents and its two leaves), as amended point 13 says. That file names no trial's figures, so `data/reference/trial.toml` is read, its seed 1942. The command checks the live game's hash after each trial.
14. *Built worlds record breakdowns* (decision 0170's constructor keeps the default). Turning them off in a trial would make it cheaper; not done, as no record asks it.
15. *Amended point 3* (what a side handed in and the engine has not taken in) needs nothing of the crate: a trial is asked of a view on an evening, and the caller puts such commands at the head of the course. *Amended point 15* is noted and nothing is built for it.

**Documents.** Chapter 9 of the rulebook gains "Playing a trial" (rules 1 to 10: the course and its days, the seed and number, the sizes, the playing and the horizon, the number alone, stopping, nothing changed, what it gives back, a world that cannot be played, the measure), and its opening, "What is built" and the source's rules say so; the glossary gains "course (of a trial)", "horizon (of a trial)" and "size (of a trial)" and amends "trial" and "trial's figures"; `docs/commands.md` the workload h and its options, and the seed and sizes in the `trial` key; `docs/engineering/layout.md` the crate's row and boundary and `sim-cli`'s use of the crate; the frozen figures' README the new copies, the seed and the sizes.
