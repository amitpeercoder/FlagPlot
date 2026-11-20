# 0181: What-ifs and the window from a seat: a draft beside the plan, an order taken away, and the same order at later dates

Status: a plan by the architect, written on Nov 20, 2026, inside the plan for step 3, stage 4 (decision 0161, step 8 of its ten, section G, rule 18, and section F from a seat). It is the second record of step 8; the first is decision 0178. Read by a reviewer who had not seen it written, and amended on that reading ("Amended on review", which replaces the rules above it where they differ) before any of it is built.

Summary: A commander can ask what an order would cost. This record lets him ask the three questions that follow. What if I change one thing: the draft is tried beside the plan as it stands, in the same worlds, and the two are set side by side. What does this order cost me: the side is tried with the order and without it. What if I give it later: the same order is tried at two or three dates. Each is the tool `trial` with one more argument. Nothing new is played: the trial, the estimate and the window are those already built

Standing: A plan, written by the architect on Nov 20, 2026, inside decision 0161, and amended on review. Built on Nov 20, 2026. Its readings wait for the architect

## Why

Decision 0161: "A what-if varies one thing: the date, the troops, the shipping, a reserve, the risk or a priority, as a draft beside the plan. Trying one changes no commitment; approving one applies that one, by the commands a seat already has" (rule 18). Decision 0177 built the window and gave it to no seat. Decision 0178's rulings left two things here: what an order in force costs the other plans ("without this order"), and that the second trial of an order already in force compares nothing and is not to be played.

What is so today, read from the code:

- **A draft can already be tried.** `trial` takes `preview`, the number of a preview the seat was shown (decision 0178, rule 7). A preview is of one command: an intent set, or a limit. The limits a seat can set are the things rule 18 names and more: the forces (troops and shipping), a base's reserve, the risk, the priority, "not before" a day, "wait for" a force or a unit, "after" another intent, and what must be known. Approving a preview queues that one command.
- **A draft tried is not set beside the plan.** The second trial is played only where the side has another plan, and its answer gives what the other plans lost. What the draft changed for the intent itself is not said.
- **The second trial is given no leave.** It is played with an empty course (decision 0178, reading 3), while the first is given the leaves the side lacks. Where the side has not given leave to launch, the first trial's takes go and the second's wait for leave: the two differ by the leave as well as by the thing tried.
- **For an intent in force the second trial removes nothing,** so both trials are the same (decision 0178, reading 3 and ruling 3).
- **Two estimates of the same worlds can be compared world for world** by one function (decision 0177, amended 10): met and kept in both, in the first alone, in the later alone, in neither, and the worlds that could not be played counted apart.
- **A window takes standing commands, a course and dates** (decision 0177, amended 2 to 4). The course is shifted to each date; the standing commands are not. The runner of a seat's trial already asks a window of one date (decision 0178, reading 2). Dates may be given, one to three, ascending.
- **"Not before" a day holds a landing that is readied at once** (decision 0159, rule C; scenario 81): the staff readies it, does not release its plan before the day, and goes on from that morning as a take does. That is "prepare now and go later", which a window's later date is not (decision 0177, amended 1).

## The rules

### Beside what

1. **`trial` gains `beside`:** what the thing tried is set beside. It is played as a second trial in the same worlds: the same view, beliefs, figures, seed, trial's number and size. Its values:
   - `nothing`: no second trial;
   - `as_it_stands`: the side as it would go on the coming morning without the draft;
   - `without_it`: the side without the intent tried.
2. **Where none is given:**
   - for a `preview` of a limit, or of a change to an intent that is in force or queued: `as_it_stands`;
   - for a `preview` that sets a new intent: `as_it_stands` where the side has another plan to weigh, as today, else `nothing`;
   - for an `intent` that is queued and not yet in force: `without_it` where the side has another plan to weigh, as today, else `nothing`;
   - for an `intent` in force: `nothing`. The answer says the order is in force, and that asking it beside `without_it` shows what it costs.
3. **What may be asked:** `as_it_stands` with a `preview` only; `without_it` with an `intent` only; `nothing` with either. The others are refused with words that say which goes with which.
4. **The two trials differ in the one thing.** Each is given the same standing commands but for that thing, and the same leaves: a leave the course gives the first is given the second too (this mends decision 0178's reading 3). So:
   - `as_it_stands`: the standing commands as they are, and the leaves; the first has the draft besides.
   - `without_it`, the intent queued: the second has the standing commands less those that set the intent or a limit on it, and the leaves.
   - `without_it`, the intent in force: the second has the standing commands, the leaves, and the intent's withdrawal, handed in on the coming morning. The answer says the withdrawal is the trial's and not the commander's.

### What the answer gains

5. **Beside it,** where a second trial was played, the answer gives for the intent tried:
   - how it went in the second trial, and what it cost each side there, as the first is given: counts of worlds, and the lowest, the middle and the highest. For `without_it` the intent is not there to have gone: only the costs are given.
   - for `as_it_stands`, world for world (decision 0177, amended 10): the worlds in which the end was met and kept in both, with the draft alone, as it stands alone, and in neither, with the worlds that could not be played counted apart.
6. **What its other plans lost** is given as today (decision 0176), from the same second trial.
7. **No advice.** The answer says nothing of which is to be chosen; it gives no chance, no per cent and no points (decision 0176, 15; decision 0177, 10).
8. **A what-if is applied by the command it is.** A preview tried is not spent: `approve` queues it, as before. Nothing in this record changes a commitment.

### The window from a seat

9. **`trial` gains `dates`:** `chosen`, or a list of one or two days of the game after the coming morning, ascending. The answer is then a window (decision 0177): the first date is the coming morning; the later dates are those the window chooses from what the course may use (decision 0177, amended 3), or those given.
10. **What is handed in at each date** is the thing tried:
    - for a `preview`: its command;
    - for an `intent` that is queued: the queued commands that set it or a limit on it, in the order queued;
    - with the leaves the side lacks, as rule 9 of decision 0178 has them. They are the course's and are handed in on the date with it.
    Everything else the seat has queued, and the scenario's commands of the day, are standing commands, handed in on the coming morning at every date.
11. **An intent in force has no later date.** A window of one is refused: there is nothing left to hand in later. The words say that a landing in force is held to a day by a "not before", which can be previewed and tried.
12. **A window has no second trial.** `beside` with `dates` is refused, but for `nothing`.
13. **The window's answer** is of the kind `window`. It gives the dates, each with what came due on it; and for each date the fields a trial's answer has, with the day the course was taken in, the men its take chose, the enemy on the eve of the date and, for each later date, world for world against the first. It names things as decision 0178, rule 16, names them: what came due and what was chosen are the side's own units by its own numbers; the enemy on the eve is given as men and cover at a place, marked as the trial's own reckoning, and never as a unit.
14. **Its words are a commander's** (decision 0178, rule 17), each date apart, and each later date saying what decision 0177, amended 1, has it say: that nothing of the order was done before the date, and that a landing decided on that day is ashore later than one prepared from the first morning and held.

### Asked before it is played

15. **`trial` gains `play`,** true where none is given. With `play` false nothing is played: the answer says what would be tried. It gives the standing commands, the course, what it would be set beside, the dates with what is due on each, and the worlds and days of each trial it would play. It is not kept among the estimates shown. A seat, or the table, can so tell a commander what a dry run will be, and how large, before he waits for it (decision 0177, rule 11).

### Kept

16. **A window and a trial with a second trial are each one estimate** in the list `trials` gives, numbered as before (decision 0178, rule 19).

## What it leaves for later

- **The trial the staff's leave is asked with, and the comparison the staff offers where a limit binds** (decision 0161, rules 19 and 20): the next record. Both are trials the staff asks for, not the commander.
- **The lift and the weight of supply a plan needs** (decision 0176, amended 5): the next record, with the trial a leave is asked with, where the commander is shown what is to sail.
- **A later date prepared from the first morning** is a what-if of "not before", by rule 1. A window that prepares early by itself needs a take that can add a unit to a landing already readied, which the staff cannot do (decision 0177, amended 1).
- **Air groups that come due make no date** (decision 0177, rulings, 2): still owed.
- **A what-if of a direct order.** A seat's orders to units are not tried (decision 0178).
- **The table's "Dry run":** after the next record.

## Rule card

- **Reads:** what decision 0178 reads: a side's whole view on an evening, the session's queue and previews, and through the runner what a trial, an estimate and a window read.
- **Writes:** nothing in the game. The session's own list of estimates shown.
- **Reuses:** the trial, the estimate, the window with its standing commands and given dates, world for world, the preview, the seat's words.
- **New:** in `sim-agent`, three arguments of `trial`, the kind of answer `window`, and what the answer gains; what a session asks a runner gains the second trial's own commands and course. In `sim-cli`, the runner fills them. Nothing in `sim-core`, and nothing new in `sim-trial` beyond what the runner needs made public.
- **Draws:** none new.

## What it costs

- **The agent interface** gains arguments and a kind of answer: its version rises, in every schema and recorded transcript.
- **No game changes:** no state, no draw, no hash.
- **An answer changes** where the second trial was played and the side lacked a leave: the second trial is now given it (rule 4). A trial of an intent in force is no longer played twice (rule 2).
- **Time.** A what-if is two trials, about 1.6 times one (decision 0178, the measure). A window of three dates is as decision 0177 measured it: at the full size over a minute on the 39 days. `play` false costs nothing.
- **Documents:** the agent's schemas and their README; `docs/commands.md`; the rulebook's chapter on the trial; the glossary ("what-if", and "beside").

## The hand calculation

It is in `tests/scenario_100_what_ifs_from_a_seat.rs`, written before the code: scenario 96's second case, the worlds that differ, played from a seat at a size of 4 worlds for 5 days.

- **The window.** The commander queues the take, its forces limit and his leave, and asks `trial` with `dates` chosen. He is answered with scenario 96's window: at the coming morning the place is taken in 2 of 4 worlds, and on day 4, with the second unit, in 4 of 4.
- **A what-if of the date.** He previews "not before day 3" on the take and tries it. The landing is readied at once and goes a day later, against men who have dug a day more. Beside the plan as it stands: taken in the same 2 worlds, on day 5 and not day 4, at the same cost to his side; the trial's enemy loses 240 men, not 254, in the worlds where the place is not taken.
- **Without it.** Compared with the side without the order, the side loses no man and the place is not taken.

## Amended on review

An independent reading of the plan and its hand calculation, against the code, confirmed every statement of "What is so today" and every number of the hand calculation, and found the following. Each was checked before it was taken. This section replaces the rules above where they differ, and the hand calculation is amended with it.

1. **One switch, not three values** (replaces rules 1 to 3). `as_it_stands` with a preview and `without_it` with an intent were one thing under two names, and a preview that sets a new intent had no intent in the second trial for "as it stands" to read. `trial` gains `compare`, true or false: whether a second trial is played of the side **without the thing tried**. What that is:
   - a preview of a limit, or of a change to an intent that is in force or queued: the plan as it stands, without the draft. The intent is in both trials, and the answer gives it in both, and world for world;
   - a preview that sets a new intent, or an intent that is queued: the side without that intent. It is not in the second trial to have gone: the answer gives the costs of each side there and no count of worlds for it;
   - an intent in force: the side with the intent withdrawn on the coming morning.
2. **Where `compare` is not given** it is true for a preview of a limit or of a change to an intent that stands; true for a new intent, previewed or queued, where the side has another plan to weigh, as today; false for an intent in force; and false wherever `dates` is given.
3. **A withdrawal is a withdrawal** (rule 4). Withdrawing an intent in force cancels its plan and the staff's intents under it. It does not undo what was ordered: a landing being readied, ships under way and men ashore stay as they are. The answer says "with the order withdrawn on the coming morning", that what was already ordered stands, and that the withdrawal is the trial's.
4. **A window hands in the whole order at each date** (replaces rule 10). The course of a window is every standing command that sets the intent tried or a limit on it, in order, and after them the draft where a preview is tried, and the leaves the side lacks. So a previewed limit is tried as part of the order given later, not as a limit given later to an order given now; and the dates are chosen from the order's own forces limit, which is in the course. The standing commands are the rest.
5. **No later date for an order in force** (replaces rule 11). `dates` is refused where the intent tried is in force, whether it is named by `intent` or by a preview of a limit on it or of a change to it. The words say that an order in force cannot be given later, and that a landing in force is held to a day by a "not before", which can be previewed and compared.
6. **The scenario's commands are standing commands too.** Wherever a rule moves or drops "the commands that set the intent or a limit on it", it means among all the standing commands: the scenario's commands of the day for that side and what the seat has queued, as the second trial's filter reads them today.
7. **A standing command left behind that names the intent.** Where the commands that set the intent tried are moved to a later date or dropped, and another standing command names that intent (an "after" on another intent), the trial is refused, naming the command: it would be refused at intake in the worlds played, and the two trials would differ by more than the thing tried. An intent in force that is withdrawn is not such a case: the other order then waits, as it would in the game.
8. **`dates` is forgiving and bounded** (rule 9). A list may begin with the coming morning, which is the first date in any case: it is dropped, and one or two later days remain. A given date later than the last day of the first date's play, or than the game's last day, is refused; chosen dates are bound so already.
9. **`describe`, not `play`** (rule 15). The table's button is "Dry run", which is the trial itself; an argument `play` set to false would be taken for it. `trial` gains `describe`, false where none is given: with it true nothing is played and the answer says what would be tried.
10. **What it costs, said for each call.** A preview of a limit is compared by default, so it is two trials: at the full size about 36 s on the 39 days against 23 s for one. `compare` false asks for one.
11. **This changes how decision 0178 split step 8.** That record says step 8 is written in two records, and puts the lift and the weight of supply with the window. Step 8 is written in three: decision 0178; this one; and the next, the trials the staff asks for, with the lift and the weight of supply. They go with the trial a leave is asked with because that is where a commander is shown what is to sail; a window's later date already says what it gains in men.
12. **Reworded:** rule 11's "a window of one" meant a window of an intent in force (item 5).

## As built

Built on Nov 20, 2026, on the branch `what-ifs-build`, to the record as amended on review. No number or statement of the hand calculation in `tests/scenario_100_what_ifs_from_a_seat.rs` was changed. Every statement of A, B and C and every property holds as written, at the first run of the figures; no mismatch was found. The figures the hand calculation asks to be reported are below.

**What was built.**

- **`sim-agent`** (`src/trial.rs`, `src/session.rs`, depending on `sim-core` alone). `TrialAsked` now carries `dates: DatesAsked` (`Coming`, `Chosen` or `Given(days)`, the coming morning first) and `second: Option<SecondAsked { of, standing, course }>`, with `BesideOf::{AsItStands, WithoutIt, Withdrawn}`, in place of `dates: Vec<u32>` and `as_they_stand`. `TrialRunner` gains `describe`, which plays nothing and answers `Described` (the size, the dates with what came due on each, those not tried). `TrialStep::Done` carries `Answered::{Trial, Window}`. `TrialAnswer` gains `beside: Option<BesideShown>` (what the second trial is of, how the intent went there where it is there, its costs, and `MatchedShown`, world for world, for the plan as it stands). The window's answer is `WindowAnswer` with `DateShown` (what came due as `DueShown` of `DueThing::{Force, BackFromYard, LandUnit}`, the take's units as `ChosenShown` of `OwnUnit`, the enemy on the eve as `EnemyEveShown`, a place with men and cover, and the date's own `TrialAnswer`). Every unit, force or base these hold is an `OwnForce`, `OwnUnit` or `MapBase`, which only `LiveIds` makes: no field can hold a unit the trial made for the enemy (decision 0178, rule 4). The tool `trial` takes `compare`, `dates` and `describe`; the answers `window` and `trial_described` are new; `trials` names each estimate's `kind`. The agent's source test `tests/leaks.rs` is unchanged and passes.
- **`sim-cli`** (`src/seat_trial.rs`): the runner plays the first trial as a window of the dates asked (the coming morning, chosen, or given and bound by the first date's play), and the second, where asked, as a window of the coming morning with its own standing commands and course; reads the estimate with the second trial's play for the other plans, the second's own estimate for `beside`, and `sim_trial::world_for_world` for the plan as it stands; and fills a window's answer from `sim_trial::window`. `describe` reads `sim_trial::dates` or `given_dates` and builds no world. The MCP definition of `trial` gives the three arguments. `time`'s workload j now plays its second trial with the same leaves as the first: the order compared with the side without it.
- **`sim-trial` and `sim-core`:** not changed. Its source test is not loosened.
- **Data:** `data/scenarios/100_what_ifs_from_a_seat.toml` (scenario 96's file with its second case's believed men, its own file of sides and trial's figures); `data/reference/trial_placeholder_scenario_100.toml` (scenario 96's figures with the sizes 4 x 5, 8 x 5 and 16 x 5) and `class_sides_placeholder_scenario_100.csv` (scenario 96's), each with a frozen copy and a line in `data/reference/step1/README.md`. All placeholders.
- **Tests:** `tests/scenario_100_what_ifs_from_a_seat.rs`, 29 tests, one or more to each statement and property, the answers checked against their schemas. B's trial with the draft is also played in its worlds through `sim_trial::play_window`, to check the staff's orders day by day and the fights of day 5. Scenario 97's test is unchanged and passes. Two tests that walk every scenario and set aside scenario 96 by name, for its digging ground figures (`scenario_34_cover.rs`) and for its unit with a day (`scenario_65_a_day_from_which_a_force_may_be_used.rs`), set aside scenario 100 too, which has both by the same inputs.
- **Generated:** the agent interface's version is 31, in every schema and in the five recorded transcripts `regame.py` remakes, which differ in that alone; the schemas `window.schema.json` and `trial_described.schema.json` are new, and `trial.schema.json` and `trials.schema.json` gain the fields above; `run_baseline/100_what_ifs_from_a_seat.txt` and `.events.txt` are new. No `save.json`, golden, pinned hash or batch fixture changed.

**Readings the record did not make.**

1. *A change to an intent that stands* is a preview of a `set_intent` whose number is in force in the game or set among the standing commands; a `set_intent` of any other number is a new intent. A preview of a limit is "of a limit" whatever the limit, a reserve included.
2. *In force* is read on the game as it stands at dusk: an intent taken in on an earlier morning. An intent only queued, set among the standing commands, is queued, even where the coming world holds it.
3. *The second trial's course.* For the plan as it stands and for the side without the order it is the leaves the first is given, on day 1; for an order in force it is the withdrawal and then the leaves, on day 1, after the standing commands.
4. *What names the intent* (amended 7) is a limit "after" whose other intent is the one tried. No other command is read as naming it. The check is made only where the commands that set the intent move or go: a window of an intent not in force, and the second trial of a queued intent. A new intent previewed has no command among the standing ones to name it.
5. *The bounds of `dates`.* An empty list, or one that holds the coming morning alone, is refused ("gives 0 days after the coming morning"), as are a day before the coming morning and a value that is neither `chosen` nor a list of whole numbers. The game's last day is checked by the session; the last day of the first date's play by the runner, which knows the size. Chosen dates are bound by the first date's play alone, as the record says.
6. *The described answer* is of the kind `trial_described`, which the record does not name. It is refused for what the session and the runner check before any world is built (the arguments, the dates, the size); a refusal `begin_window` would make only once the pictures are made (decision 0174, 20) is not made by `describe`.
7. *A window's date* gives a trial's fields with the first day of the trial as `first_day`, since each date's worlds are played from it, and the day the order was handed in as `taken_in`.
8. *An order in force asked with no `compare`* is answered with the line "Your order is in force: what it costs your other plans is in things as they stand, and no second trial was played", and that `compare` true tries it beside the side with it withdrawn, in place of the line that the side has no other plan; it says so whether or not the side has another plan.
9. *The words of a withdrawal* say "what was already ordered stands, a landing being readied, ships under way and men ashore as they are", and that the withdrawal is the trial's and the intent is not withdrawn in the game.
10. *A window's words* end with the line that no second trial is played beside a window and that its answer names no other plan; its dates' fields have no `other_plans`.
11. *The words of a leave given by the trial* gain "It is given in both trials" where a second trial was played.
12. *`trials`* names each estimate's `kind` (`trial` or `window`) and, in its words, marks one "compared" or "at later dates".
13. *The MCP input schema* gives `dates` no type, so that a value of the wrong kind reaches the session's refusal, as `size` does (decision 0178, reading 15); `compare` and `describe` are booleans there.
14. *Scenario 97's answers* are as they were in every field and word they had; each gains the new fields of `asked` (`compare`, `beside`, `in_force`, `dates`) and `beside: null`, which the schema requires. Its test passes unchanged.
15. *A reserve previewed and tried at later dates:* a reserve names no intent, so a standing reserve is not moved; the draft itself goes to each date with the order.
16. *What came due and the men chosen* name only what the live view has; a thing it lacks would be left out (none is, in any case tried).

**Reported, for the architect to state.**

- *B, the plan order's day expected ashore with the draft:* day 5 (world 1 at the end of day 1), waiting for "day 3, which you set it not to go before"; as it stands, day 4.
- *An order in force, compared* (the three commands taken in on day 1, asked on the evening of day 1, days 2 to 6): in the first trial the place is taken in all four worlds, worlds 1 and 3 on day 4 and worlds 2 and 4 on day 6 (the earliest day 4, the middle 4, the latest 6); the side's men lost to the enemy 100, 100, 300 over the four, and 250, 250, 300 over worlds 2 and 4; the trial's enemy's 200, 200, 400, and 400 in each of worlds 2 and 4. Beside it, with the order withdrawn on the morning of day 2: nothing sails, and every cost of either side is 0, 0, 0 in every world.
- *A draft limit at later dates* (preview 1 with `dates` chosen): the course is commands 1 and 2 and then the draft; the dates are day 1 and day 4. Its first date has B's figures with the draft. The second date: met and kept in 4 of 4 worlds on day 8; the side's men lost 100, 100, 200 and the trial's enemy's 200, 200, 400; the day expected ashore day 8; the take chose 1,200 men, units 1 and 2. These are A's second date: on day 4 a "not before day 3" is already past.
- *A draft at later dates* (the `set_intent` previewed alone, on a fresh evening, `dates` chosen): two dates, day 1 and day 4. At day 1, met and kept in 2 of 4 (worlds 1 and 3) on day 4, the side's men lost 100, 100, 200, the trial's enemy's 200, 200, 254, the take chose unit 1, 600 men. At day 4, met and kept in 4 of 4 on day 8, 100, 100, 200 and 200, 200, 400, units 1 and 2, 1,200 men. At both, leave to launch is given by the course and named so.

**The measure** (amended 10), `sim-cli time --only-seat-trial --trial-size SIZE` (workload j), release build, each size in its own process under `/usr/bin/time -l`, one run each, on an Apple M3 Pro with a load average of about 4.5 to 7 from other work, so the times are noisy. The slice is `stage-3-proof/07a_slice.toml` with its own trial's figures, as decision 0178 measured it; workload j's second trial is now the order compared with the side without it, given the same leaves (here none: the side holds or is given both among its commands). The live game's hash was the same after each.

| Size | One trial | Compared, two trials | Peak memory of the process (resident) | The record's estimate |
| --- | --- | --- | --- | --- |
| small, 8 worlds x 7 days | 0.90 s | 1.50 s | 41 MB | |
| standard, 16 x 14 | 3.07 s | 4.86 s | 97 MB | |
| full, 64 x 21 | 19.6 s | 27.2 s | 469 MB | 23 s for one; about 36 s compared |

Compared is about 1.4 to 1.7 times one trial. A window from a seat is decision 0177's window (workload i), which this build does not change, and was not measured again.

**Test counts.** TESTCOUNTS

**Not verified by this build.** That a game file kept by `play` replays a what-if or a window at the cost logged (no recorded game has a trial); the times on an unloaded machine; an agent playing the three new arguments through `mcp` end to end; the table, which is not changed.
