# 0175: A trial: one course played in a number of worlds

Status: settled by the architect on Nov 19, 2026, inside the plan for step 3, stage 4 (decision 0161, step 5 of its ten, section D, rules 10 to 12), which the designer confirmed. The hand calculation is written. To be built. Its three sizes are placeholders until they are measured by the build and the designer accepts the times.

Summary: A trial takes a course, builds the worlds a side's picture allows, and plays each forward for a number of days with both sides on doctrine. It is given the course as the commander's commands that make it, each on its day, so that an order, a limit, a leave or an approval can be tried. Its size is a number of worlds and of days, fixed and not timed. It can be stopped at any point and leaves nothing. It reads the side's view and what the side believed and changes nothing in the live game: the live game's hash is the same after it

Standing: Settled by the architect, Nov 19, 2026, inside decision 0161. The hand calculation is written; the build is next. The sizes wait for the designer

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

