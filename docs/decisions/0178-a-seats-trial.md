# 0178: A seat's trial: the tool, who runs it, and what the commander is shown

Status: a plan by the architect, written on Nov 19, 2026, inside the plan for step 3, stage 4 (decision 0161, step 8 of its ten, section G, rules 12 and 21). Step 8 is written in two records: this one, the tool `trial` and the estimate a commander is shown; and the next, what-ifs, the window from a seat, and the trial the staff's leave is asked with. Read by a reviewer who had not seen it written, and amended on that reading ("Amended on review") before it was built. Built on Nov 20, 2026 (As built, at the end), on decision 0177's build.

Summary: A trial can be played and an estimate read, by a program. This record gives them to a commander. A seat with a staff gains the tool `trial`: it names an intent of its own or a draft it has previewed, and a size, and is answered with the estimate in words a commander can read, or with why it cannot be tried. The session that holds the game cannot build worlds and must not learn to, so it is handed something that can, as it is handed its staffs. The estimate is kept with the session as a thing the commander was shown. The game is not changed by asking

Standing: A plan, written by the architect on Nov 19, 2026, inside decision 0161, and amended on review. Built on Nov 20, 2026, and its readings ruled on the same day. The second trial for an intent in force was settled by decision 0181

## Why

Decision 0161: "A seat's tool `trial` takes an intent of the commander's, a size and, where wanted, what to vary; it answers with the estimate, or with why it cannot be tried" (rule 21); and a trial's "result is kept with the session, outside the state, as an estimate the commander was shown" (rule 12).

What is so today, read from the code:

- **`sim-agent` may depend on `sim-core` alone,** and its library may not name a picture, a half or a world built from one: a test reads its source. `sim-trial` depends on `sim-core` and `sim-staff`. So the session cannot call `sim-trial`.
- **The session is already handed what it may not depend on:** its staffs, by its caller, through a trait that `sim-core` defines. `sim-cli` depends on every crate and builds the session; it has the scenario in hand when it does, with the side's believed forces, intents and men and the trial's figures.
- **An estimate is a type of `sim-trial`,** with words that print the engine's own names for kinds and tasks (decision 0176, rulings, 5). `sim-agent` has the words a seat is given for a task, a unit by the side's own number, and a place.
- **What a seat has handed in for the morning is in the session,** not the world: its queued commands, and the scenario's own commands of the day. A side's own half leaves them out (decision 0170).
- **A preview** (decision 0107) is the session's record of one draft command, with what the staff would do; it is numbered among its side's, and is void once anything is queued after it or a day is resolved.
- **A side's own half is of a whole side, on an evening** (decision 0171): it is refused at a command's seat, at a view of units as a command holds them, and while a day is open.
- **Every call of a seat is in the session's transcript,** with its answer. A saved game holds orders, commands and the staff's hand-ins, and no call. A game file kept by `play` replays every call.
- **A trial at the small size takes under a second** on the 39 days and at the full size 16 s (decision 0175, accepted by the designer). A call is answered when it is done; nothing else of the game answers meanwhile.

## The rules

### Who runs a trial

1. **`sim-agent` defines what a session asks and is given** (amended, 2): what is asked; the answer, as plain data made for a seat; why a trial is refused, in words; and a trait by which a trial is begun and stepped. Nothing in `sim-core` calls a trial, so nothing of this is `sim-core`'s.
2. **What is asked** is general enough for the next record (amended, 8): the trial's number, a size by its name or its two numbers, the standing commands, the course as commands with their days, the dates to try (one here: the coming morning), the intent whose estimate is wanted, and whether a second trial of things as they stand is wanted.
3. **A trial is begun and stepped.** To begin takes a side's whole view and what is asked, and answers with a trial under way or the refusal. A step plays some of it and says whether it is done; when done it gives the answer. A trial under way that is dropped leaves nothing. The tool of this record steps it to its end.
4. **The answer is made for a seat, and holds no more than a seat may be shown** (amended, 2): the figures of the estimate (how it went, the costs, the enemy's losses as counts, what it needs and has, where the worlds part, the other plans); the plan order's tasks, each by its kind, its place, its day and the side's own units it names; and what was assumed, each as a kind with a place, a number of men or tons, a standing and the worlds it was in. It has no field that could hold a unit made for the enemy, no number of an intent or a plan formed inside a trial's world, and no list of worlds' own figures beyond the counts.
5. **The estimate stays `sim-trial`'s,** with its programmer's words (decision 0176). `sim-cli` holds the thing that runs trials: a value with, for each side, what the scenario says that side believed, the trial's figures, the reference data and the map. It implements `sim-agent`'s trait by playing a trial (decision 0175), reading its estimate (decision 0176) and filling the answer from it.
6. **`sim-cli` hands it to the session,** as it hands the staffs, where the scenario names a file of the trial's figures. A scenario that names none has no trial: its sessions are as they were, with the same tools listed as before.

### The tool

7. **`trial`** is a tool of a seat that has a staff and plays a whole side, in a game that has a trial: it is listed where the tools of command are and only in such a game. Its arguments:
   - `intent`, the number of an intent of the commander's own, in force or queued for the morning; or `preview`, the number of a preview the seat was shown, to try that draft. One of the two. A preview of a limit is tried for the intent the limit is on.
   - `size`: `small`, `standard` or `full`; `small` where none is given.
8. **The standing commands** are what the side tried would have taken in on the coming morning without the trial: that side's commands of that day in the scenario, and what its seat has queued, in that order. Nothing of the other side's, scripted or queued, is among them (amended, 6). They are given apart from the course, as decision 0177 has it, and this record is built on that build.
9. **The course** is then:
   - for an intent: leave to prepare and leave to launch, each where the side neither holds it nor has it among the standing commands, so that the intent is tried as approved (decision 0161, 10);
   - for a preview: its command, and the same leaves.
   A leave is named in the answer as given by the course only where the course gave it: one the commander holds or has queued is his own (amended, 1). A leave the course gives is the side's, and frees every plan of the side that waits for it; the answer says "leave given to the side", not to the one plan (amended, 15).
10. **The side's view** is the whole side's, as the staff's tools take it, at any seat that plays a whole side, with command levels on or off: a trial knows the side's own units exactly (amended, 9). It is taken outside the dispatch of tools, where the agent's source test allows it.
11. **The second trial.** Where the side has another intent of the commander's with an end, in force or queued, the tool asks for the second trial, of things as they stand, and the answer gives what the other plans lost (decision 0176, amended, 11). Where it has none, no second trial is played and the answer says the side has no other plan to weigh. The second trial doubles the time: about 1.7 s at the small size and 32 s at the full, on the 39 days.
12. **The trial's number is 1,** always. The same question on the same evening is the same worlds and the same answer (decision 0161, condition 5), and two questions on one evening are tried in the same worlds.
13. **It is refused,** with words that say why and nothing played: where the game is over; while a day is open; with both `intent` and `preview` or neither; an intent that is not the side's own, is the staff's, is withdrawn, or has its withdrawal queued; a preview that is not the seat's or is void; a size the figures do not name; and whatever the runner refuses (a side whose own units reach the numbers kept for the enemy's, decision 0174, 20). At a seat of a command, at a seat with no staff and in a game with no trial the tool is not listed, and a call of it is answered as a call of any tool the seat does not have.
14. **It changes nothing.** No command is queued and no preview is spent. The live game's hash is what it was, and a preview tried may still be approved. It reads nothing of the live enemy: the same seat in the same game with another enemy behind it is given the same answer, byte for byte.

### What the commander is shown

15. **The answer** is of the kind `trial`: its number among those the seat has been shown; what was asked; the fields of rule 4; and its words.
16. **How each thing is named** (amended, 4):
   - a unit, a task force or a base of the side's that the live game has: by the side's own number and name, as the seat's other tools give it;
   - a task force or a base the staff forms inside a trial, which the live game does not have: "a force the staff would form", "a landing place the staff would make", with its ships or its place;
   - a task of the plan: by its kind in the words the tool `order` uses, its place and its day, and no number;
   - a place: as the seat's tools give a place;
   - a force assumed for the enemy: by its kind and its place, and by what belief of the side's it was made from, in words ("from your contact at ...", "from your estimate of ...", "believed, standing unknown"); never by a number.
17. **The words are a commander's.** They hold no name of the engine's: no kind, task, unit or place as the program prints it. They say, in this order: what was tried and in how many worlds; how it went; what it cost the side; what the trial's enemy lost, as the trial's own reckoning; what it needs and has and when it could go; what its other plans lost, or that there is none; what was assumed; where the worlds part. They give counts of worlds and no chance, no per cent and no points (decision 0176, 15). Where the seat has orders to units queued, they say those are not tried.
18. **The enemy's units in a trial are never named to the seat.** The trial's enemy is an invention; the answer gives his losses as counts and his assumed forces as rule 16 has it. This holds by what the answer can hold (rule 4), and is tested: every number of a unit in an answer is one the live view has.
19. **The answer is kept with the session,** numbered from 1 among its side's, with the day and the moment it was asked, the revision of the seat's queue then, and what was asked. It is no state, no event and no hash, and it is not in a saved game. The tool `trials` lists what the seat has been shown, marking each asked on an earlier evening or before the queue changed, and gives one again by its number, unchanged. What the staff attaches when it asks leave (the next record) is numbered in the same list.
20. **A game file kept by `play`** replays every call, and so plays every trial in it again. That is kept, as the check that a trial is the same asked twice; its cost is a few seconds a trial and is logged with the performance items of decision 0172.

## What it leaves for later

- **What-ifs, the window from a seat, and the trial the staff's leave is asked with:** the next record.
- **The lift and the weight of supply a plan needs,** which decision 0176 (amended, 5) left for the step that shows an estimate to a seat: the next record, with the window, where what a later date gains in lift is the point of showing them.
- **The table's "Dry run":** the page, after both records, with the busy state a call of seconds needs.
- **Stopping a trial from a seat.** The trait allows it (rule 3); the tool of this record does not offer it. The table's dry run will.
- **A commander's direct orders in a course** (decision 0175, amended, 2). A seat's queued orders to units are not tried: the trial says so where the seat has any queued.
- **A trial inside a day.**

## Rule card

- **Reads:** a side's whole view on an evening; the session's queue and previews; through the runner, what decisions 0175 and 0176 read.
- **Writes:** nothing in the game. The session's own list of estimates shown.
- **Reuses:** the way a session is handed its staffs; the trial and the estimate; the seat's words for a task, a unit and a place; the preview.
- **New:** in `sim-agent`, what a trial is asked, the answer, the trait, and the tools `trial` and `trials` with their words; in `sim-cli`, the runner over `sim-trial`. In `sim-trial`, the count of leaves the course gave reads the standing commands (rule 9). Nothing in `sim-core`.

## What it costs

- **The agent interface** gains two tools and two kinds of answer; its version rises, as it has for every tool added, and the version in every schema and recorded transcript with it.
- **No game changes:** no state, no draw, no hash. A scenario that names no trial figures plays, and its sessions answer, as before, with the same tools listed.
- **The 39 days** are given a file of the trial's figures only where a scenario is to be tried from a seat; the arrangements of the proofs are not changed by this record.
- **Documents:** the agent's schemas and their README; `docs/commands.md`; the rulebook's chapter on the trial; `docs/engineering/layout.md` for the trait and who hands it in; the glossary.
- **Time:** a trial with a second trial is twice a trial (rule 11).

## The hand calculation

It is in `tests/scenario_97_a_seats_trial.rs`, written before the code: scenario 95 played from a seat. The Allied commander queues the take, its forces limit and his leave to launch, and asks `trial` at a size of 4 worlds for 8 days. He is answered with scenario 95's estimate: met and kept in 2 of 4 worlds on day 4, his men lost 100, 100 and 300. The game is as it was, and he may still change his mind.

## Amended on review

An independent reading of the plan and its hand calculation, against the code, found the following; each was checked in the source before it was taken.

1. **The leave the commander queued would have been named as the course's.** The trial as built counts every leave in the course against the side's own half, and the queued commands went at the head of the course. Rules 8 and 9 now give the queue as standing commands (decision 0177) and count a leave as the course's only where the course gave it.
2. **The estimate could not move to `sim-core` as written.** Its words are implemented on the type and cannot stay behind in another crate; its assumptions move with it; and they hold the numbers of units made for the enemy, which rule 18 forbids a seat. The trait and a narrow answer are `sim-agent`'s instead, filled in `sim-cli` (rules 1, 4 and 5). `sim-core` is not touched.
3. **Two things handed to this step were dropped in silence.** The second trial is now asked where there is another plan to weigh (rule 11). The lift and the weight of supply go to the next record, said so under "What it leaves for later".
4. **Things that exist only in a trial's world had no name a seat could be given:** the staff's tasks and plans formed there, a force formed there, and the enemy's assumed forces. Rule 16.
5. **The hand calculation said no count of the enemy was held.** One is: the opening estimate, told on day 0, which does not meet the need. Corrected there.
6. **The standing commands were not said to be the tried side's alone,** and nothing tested that the tool cannot see the live enemy. Rules 8 and 14, and two tests in the hand calculation.
7. **A game file replays every trial.** Kept and its cost logged (rule 20).
8. **A trait with one call could not be stopped and did not fit the next record.** Rules 2 and 3.
9. **The seat at the area command, with command levels on,** is tried on the whole side's view. Rule 10.
10. **Whether the tools are listed in a game with no trial** was said both ways. They are not (rules 6, 7 and 13).
11. **The test of forbidden words was easy to pass,** and one of its marks is the seat's own form for a base. The hand calculation now has a test that holds by construction: every number in the answer is the live view's.
12. **Refusals left unsaid:** the game over, a preview of a limit, a withdrawal queued, and what a seat without the tool is answered. Rules 7 and 13.
13. **The test's session.** The hand calculation's session is built in the test, as `sim-cli` builds one, on the frozen figures; a test of `sim-cli` proves it hands a runner in exactly where the scenario names a trial's figures.
14. **The list's name** could be taken for the seat's ground estimates: the tool is `trials`. Each answer is kept with the queue's revision (rule 19).
15. **A leave the course gives is the whole side's.** The answer says so (rule 9).

## As built

Built on Nov 20, 2026, on the branch `seat-trial-build`, from the head of `the-window-build` (decision 0177's build, pull request 542) with main merged in, to the record and the hand calculation as amended on review. No number or statement of the hand calculation was changed. Every statement of the main case and every property holds as written, at the first run of the figures: scenario 97's answer is scenario 95's estimate of intent 1, field for field. Two properties are built and reported, not asserted, as the hand calculation asks (the draft tried) or as the record leaves open (a second trial); one is asserted by what the answer can hold (reading 21).

**What was built.**

- **`sim-trial`:** one change (rule 9). `setup_with_standing` counts the leaves a course gives with its first commands taken as standing commands: a `SetLeave` among them is the commander's own and is taken in as held (or withheld), and a later command of the course counts as given only where the side does not then hold the leave. A window passes its standing commands' count; a plain trial passes none and is as it was. Scenario 96's tests pass unchanged.
- **`sim-agent`** (`src/trial.rs`, depending on `sim-core` alone): `TrialAsked { trial, size, standing, course, dates, intent, as_they_stand }` with `SizeAsked::{Named, Numbers}` and `CourseStep { day, command }`; the traits `TrialRunner::begin(&SideView, &TrialAsked) -> Result<Box<dyn TrialUnderWay>, TrialRefused>` and `TrialUnderWay::step() -> TrialStep::{Going, Done(Box<TrialAnswer>)}`; the narrow answer `TrialAnswer { worlds, days, size, first_day, last_day, has_end, went, could_not, side_costs, enemy_costs, needs, other_plans, assumed, leaves_given, uneven, partings }` with `TrialWent`, `TrialSpread`, `TrialCosts`, `NeedsShown`, `WaitShown`, `CountShown`, `PlanTaskShown`, `TaskShown`, `ForceShown::{Live, Formed { ships }}`, `BaseShown::{Live, Formed { place }}`, `AssumedShown`, `AssumedThing`, `MadeFrom`, `AssumedNoteShown`, `PartingShown`, `PartShown` and `OtherPlanShown`. A unit, ship, force or base in it is an `OwnForce`, `OwnShip`, `OwnUnit` or `MapBase`, whose field is private and which only `LiveIds` makes, from a live view's own units and the bases on its map: no field of the answer can hold a unit made for the enemy, or an intent or a plan formed inside a trial. The session gains `with_trial(runner)`, `has_trial()`, `has_trial_tools(side)`, the tools `trial` and `trials` (`TRIAL_TOOLS`, listed after the tools of command and before `advance`), the list of estimates shown (no state, not in a saved game), and the words (rules 15 to 17). `staffing::intent_words` and `previewed_intent` are made crate-wide for it. The source test `tests/leaks.rs` is unchanged and passes: the side's whole view is taken in `play_trial` and `trial_tool`, outside `dispatch`.
- **`sim-cli`:** a library target (`src/lib.rs`) with `seat_trial::SeatTrials`, the runner: for each side the scenario's believed forces, intents and men, the trial's figures, the reference data and the map. It plays the trial as a window of one given date, the coming morning (decision 0177), its standing commands apart, and the second trial likewise with an empty course; reads the estimate (decision 0176); and fills the answer. `serve::open` hands it in where the scenario names a `trial` file (`SeatTrials::runner_of`); `mcp`, `play` and `serve` all open games there. The MCP tool definitions of `trial` and `trials`. `time` gains workload j (`--only-seat-trial`, `--seat-trial-runs`).
- **Data:** `data/scenarios/97_a_seats_trial.toml` (scenario 95's file but for its name, its file of sides and its trial's figures); `data/reference/trial_placeholder_scenario_97.toml` (scenario 95's figures with the sizes 4 x 8, 8 x 8 and 16 x 8) and `class_sides_placeholder_scenario_97.csv` (scenario 95's), each with a frozen copy and a line in `data/reference/step1/README.md`. All placeholders.
- **Tests:** `tests/scenario_97_a_seats_trial.rs`, 22 tests, one or more to each statement and property, the answers checked against their schemas; in `crates/sim-cli/tests/serve.rs`, `serve` offers `trials` for scenarios 97 and 95, which name a trial's figures, and answers it as no tool for scenarios 5 and 80, which do not. `tests/Cargo.toml` takes `sim-cli` as a dev-dependency.
- **Generated:** the agent interface's version is 29, in every schema and in the five recorded transcripts `regame.py` remakes, which differ in nothing else (no scenario of them names a trial's figures, so no tool was added to what they list); two new schemas, `trial.schema.json` and `trials.schema.json`; `run_baseline/97_a_seats_trial.txt` and `.events.txt`, new. No `save.json`, golden, pinned hash or batch fixture changed.

**Readings the record did not make.**

1. *Where the runner lives.* The record puts it in `sim-cli`; the hand calculation has the test build the session "as `sim-cli` builds one". So `sim-cli` gains a library target holding the runner alone, which its binary and the scenario test both use. The test repeats `serve::open`'s steps (staffs, the scenario's commands, its orders inside a day, the runner where the trial's figures are named); the `serve` test proves the binary hands the runner in exactly there.
2. *A seat's trial is a window of one date.* The record builds on decision 0177's standing commands; the runner asks `begin_window` with the dates given as the coming morning alone, so the standing commands are handed in apart and never shifted, and the trial is trial 1. With the course empty no `TakenIn` is recorded and the plan order is the first day's, as decision 0177's reading 5 has it.
3. *What things as they stand are* (rule 11), which the record does not say for a seat: the second trial's standing commands are, for `intent`, the standing commands less those that set the intent tried or a limit on it, and for `preview`, the standing commands as they are, each with an empty course. `TrialAsked` carries them (`as_they_stand`), not a flag, so that the session, which knows what was asked, decides. **For an intent already in force** nothing is removed: the intent stands in both trials, and what its other plans lose by it cannot be read (both counts are the same). The architect may rule that the second trial withdraw it, or that the second trial be skipped for an intent in force.
4. *Another intent of the commander's with an end* is read on the world as the coming morning finds it (the queue taken in): an intent of the commander's in force, not the one tried, of the kinds take a place, control an island or hold. A withdrawal queued takes an intent out.
5. *The leaves the course gives* are read against the live view's leaves (the side's whole view) and the standing commands: a leave the side holds, or one a standing `set_leave` gives, is not given. A standing command that withholds a leave the side holds is the commander's: the course does not give it back, and the trial plays without it.
6. *A preview of a limit* is tried for the intent the preview itself reads it as about (`previewed_intent`): a reserve for the lowest intent in force whose forces limit loads at its base; one about no intent is refused ("is of a limit on no intent"). A preview's limit on an intent of the staff's is not refused.
7. *A void preview* is read as `approve` reads one: void where a day was resolved after it, or the clock moved, or the seat's revision rose (something queued or cleared). The words say which of the first and the last.
8. *The words of the refusals:* "the game is over: there is no coming morning to try"; "a trial is asked at dusk, between days, and the clock stands at ...: ask it at dusk"; "a trial is of an intent or of a preview, not both"; "\"intent\" or \"preview\" is needed"; "intent N is not an intent of yours"; "intent N is the staff's own, formed under an intent of yours"; "intent N is withdrawn"; "intent N has its withdrawal queued"; "preview N is not a preview you were shown"; "preview N is void: ..."; "the trial's figures name no size \"vast\""; and the runner's own, "it cannot be tried: ..." with `sim-trial`'s words. An unknown argument is refused naming the three.
9. *A force or a base the staff forms inside a trial* is named by its ships, or its place, as the trial's world of the plan order has them at the end of its play; where it no longer stands there, it is named without them.
10. *What was assumed is named by what it was made from:* an enemy base "from your map"; men "from your estimate of the place"; a believed body of men or force "believed, standing ..." with its name from the scenario; a contact "from your contact at" its place, with its kind and ships, read from the side's own half (the runner may take it; `sim-agent` may not); a pair "from your contacts at ... and ..., taken for one force"; the enemy's belief and intents from nothing of the side's, with no "from". An intent the enemy is taken to hold is named in a commander's words with its place or base, never a force of his. A force believed in is named by its kind, ships and last known place.
11. *The estimate's number and the list.* Numbered among the side's, from 1. `trials` lists each with the days resolved and the day it was for, what was asked, and a standing: `current`, `queue_changed` (the seat's revision has risen) or `earlier_evening` (the clock has moved); given `estimate`, it returns the answer document as it was given.
12. *The tools are listed* after the tools of command and before `advance`; at a seat at a command, at a seat with no staff and in a game with no trial they are not, and a call is "\"trial\" is not a tool", as for any tool the seat lacks.
13. *What was asked, in words.* The standing commands and the course are each written as the seat gives a command, with a forces limit naming each force and unit by the side's own number ("limit intent 1's forces to task_force:1 \"Placeholder transport force 1\", land_unit:1 \"Placeholder First\", from base:1 \"Placeholder base A\""): this is where task force 1 and unit 1 are named, as the hand calculation says.
14. *Orders to units not tried* are the seat's orders queued for the coming day, counted in the answer (`orders_not_tried`) and named in its words.
15. *The size* is given by name; `SizeAsked::Numbers` is in the trait for the next record and no tool asks it. The MCP schema gives `size` as a string, so that a size the figures do not name reaches the session's refusal.
16. *The words of a parting* give the men lost to the enemy on each side, and each other cost only where it is above 0; the main costs give all five.
17. *An intent of the commander's own* is read on the world as the coming day finds it, so a queued intent may be tried; one the side does not have and one of the other side's are refused alike.
18. *Every number of a unit in the answer is the live view's* is tested against the live view's own counts of forces, units, ships and groups (the side's numbers run 1 to the count) and the bases on its map; no run of digits is 900001 or 900002.
19. *The other side's commands are not in it:* tested with both seats played, the Japanese seat's own `set_leave` queued before the Allied seat asks; the answer is the one-seat answer byte for byte.
20. *The area seat with command levels on* is tried on the whole side's view: tested, and its worlds go as the hand calculation's.
21. *"Worlds 1 to 4 of it have the figures they had at small."* The answer holds no world's own figures (rule 4). The test asserts it by what the answer gives: the worlds met and kept and not met fall on the same numbers, the believed men are in the even worlds at both sizes, and each side of the parting has the same spreads at both. That a world's play depends on its number alone is scenario 94's (decision 0175, 4).

**Reported, for the architect to state.**

- *A draft tried* (the `set_intent` of a take of (4,0) previewed alone, on a fresh evening, at `small`): tried and answered as preview 1, intent 1; nothing queued; the preview approved after. Its figures are the main case's exactly: met and kept in 2 of 4 worlds (1 and 3) on day 4, not met by day 8 in 2 (2 and 4); the side's men lost to the enemy 100, 100, 300 and the trial's enemy's 200, 200, 300; men wanted 400, to hand 600, waiting for nothing, ashore on day 4, a count asked and the told 200 held; with no forces limit the staff assembled task force 1 and unit 1 itself ("ready a landing at (4,0) by task_force:1"). Its course is the `set_intent` and leave to launch, which the answer names "given to the side by the trial, not by you". The main case's answer and this one differ in what was asked and in that line.
- *A second trial* (not in the hand calculation): with the main case's three commands and a hold of base 1 queued, the trial of intent 1 asks the second trial, and weighs intent 2, met and kept in 4 worlds with the take and 4 without, not displaced.

**The main case's words**, whole, as the seat is given them:

```
Tried: intent 1, take the place at (4,0), as it would go from the morning of day 1, with what you have handed in for that morning, in 4 worlds of 8 days each (the size small).
Taken in on that morning as you have it: set intent 1: take the place at (4,0); limit intent 1's forces to task_force:1 "Placeholder transport force 1", land_unit:1 "Placeholder First", from base:1 "Placeholder base A"; give leave to launch.
How it went, over 4 worlds played: met and kept in 2 of 4 worlds (worlds 1 and 3), the earliest day 4, the middle day 4 and the latest day 4; met and lost again in none of the 4 worlds; not met by day 8 in 2 of 4 worlds (worlds 2 and 4).
What it cost your side, the lowest, the middle and the highest over the worlds: men lost to the enemy 100, 100, 300; men lost unfed 0, 0, 0; ships sunk 0, 0, 0; ships out of action at the end 0, 0, 0; aircraft lost 0, 0, 0.
What the trial's enemy lost, by the trial's own reckoning of an enemy it made from your beliefs, not a claim or a report: men lost to the enemy 200, 200, 300; men lost unfed 0, 0, 0; ships sunk 0, 0, 0; ships out of action at the end 0, 0, 0; aircraft lost 0, 0, 0.
What it needs and has, as your staff worked it in world 1 at the end of day 1: men wanted 400, to hand 600; it waits for nothing; the day expected ashore is day 4; a count of the enemy at (4,0) is asked for, and the one held is 200 men, told on day 0, which does not meet it. Its tasks: ready a landing at (4,0) by task_force:1 "Placeholder transport force 1".
Your side has no other plan to weigh: no second trial was played.
What was assumed: the enemy's base:2 "Placeholder airfield B" at (4,0), holding 200 t of supply and 0 t of fuel, from your map, in 4 of 4 worlds (worlds 1, 2, 3 and 4); 200 men at (4,0), from your estimate of the place, in 4 of 4 worlds (worlds 1, 2, 3 and 4); 400 men at (4,0), believed, standing unknown ("Reinforcement believed at B"), in 2 of 4 worlds (worlds 2 and 4) and not in 2 of 4 worlds (worlds 1 and 3); the enemy given nothing he could know of you: no contact, no claim and no estimate, in 4 of 4 worlds (worlds 1, 2, 3 and 4); the enemy given no intent: he holds what he holds, in 4 of 4 worlds (worlds 1, 2, 3 and 4).
No leave was given by the trial: what it needed you had given, or have queued.
Where the worlds part, by 400 men at (4,0), believed, standing unknown ("Reinforcement believed at B"). With them, 2 worlds (worlds 2 and 4): met and kept in none, your side's men lost to the enemy 300, 300, 300, the trial's enemy's men lost to the enemy 300, 300, 300. Without them, 2 worlds (worlds 1 and 3): met and kept in 2 on day 4, your side's men lost to the enemy 100, 100, 100, the trial's enemy's men lost to the enemy 200, 200, 200.
```

**The measure** (rule 11), `sim-cli time --only-seat-trial --trial-size SIZE` (workload j), release build, each size in its own process under `/usr/bin/time -l`, one run each, on an Apple M3 Pro with a load average of about 8 to 9 from other work, so the times are noisy and likely high. The slice is `stage-3-proof/07a_slice.toml` with its own trial's figures (seed 1942); from the evening before day 1, the Allied side's four commands of day 1 as standing commands, the course empty (the side holds or is given both leaves among them), the intent tried intent 1, control Guadalcanal; with the second trial, the standing commands less intent 1's set and limits, which weighs one other plan, intent 2. Every world was played at every size, and the live game's hash was the same after each.

| Size | Without the second trial | With it | Peak memory of the process (resident) | The record's estimate (rule 11) |
| --- | --- | --- | --- | --- |
| small, 8 worlds x 7 days | 1.17 s | 1.99 s | 43 MB | under 1 s; about 1.7 s with |
| standard, 16 x 14 | 3.87 s | 6.35 s | 99 MB | about 2.9 s |
| full, 64 x 21 | 22.7 s | 36.1 s | 467 MB | 16 s; 32 s with |

A trial of a seat is a plain trial's work and a little more (the window's bookkeeping and the answer); the second trial plays an empty course, which is cheaper than the first, so "with" is about 1.6 to 1.7 times "without", not twice. The full size is about 40 per cent above decision 0175's measured 16 s, on a machine under load; it is for the designer with the window's 67 s (decision 0177, rulings, 8).

**Not verified by this build.** That a game file kept by `play` replays a trial at the cost logged (no recorded game has a trial); the times on an unloaded machine; the table, which is not changed.

## The architect's rulings

Read on Nov 20, 2026, with the build's core: the answer's types and the makers of the values that name a unit, the agent's source test (unchanged), and that nothing of `sim-core` is touched. The hand calculation in `tests/scenario_97_a_seats_trial.rs` is as the architect wrote it; no number was changed.

1. **The runner in a library of `sim-cli`** stands (reading 1): it is the one crate that may know both the session and a trial, and the scenario test uses the runner the binary uses.
2. **A window of one given date** for a seat's trial stands (reading 2): one way of playing, and the next record asks for more dates of the same thing.
3. **The second trial of an intent already in force is not to be played** (reading 3). With the intent standing in both trials nothing is compared, and the time is spent for a line that says nothing. The answer is to say that the order is already in force and its cost to the other plans is in things as they stand. What an order in force costs the others is a what-if, "without this order", and belongs to the next record. Owed, in `docs/open-work.md`: today the second trial is played there.
4. **A leave the commander has withheld by a queued command is not given back by the course** (reading 5). Stands: it is his word, and the answer says the trial was given no leave.
5. **A preview of a limit on a staff's intent** is tried (reading 6). Stands: the limit is the commander's command, whoever formed the intent.
6. **A force or a base formed inside a trial** is named by its ships or its place (reading 9), and the side's own units by the side's numbers in the words of the standing commands (reading 13). Stand.
7. **"Worlds 1 to 4 at the larger size are those of the smaller"** is tested through what the answer gives and not world by world, since rule 4 keeps a world's own figures out of the answer. Stands; decision 0175's own tests hold it world by world.
8. **The measured times,** on the 39 days under load: 1.2 s, 3.9 s and 22.7 s for the three sizes, and 2.0 s, 6.4 s and 36.1 s with the second trial, which is about 1.6 times and not twice, its course being empty. The full size is above decision 0175's 16 s, measured on a quieter machine. No size is changed. It goes to the designer with the window's 67 s.

