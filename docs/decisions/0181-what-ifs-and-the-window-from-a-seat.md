# 0181: What-ifs and the window from a seat: a draft beside the plan, an order taken away, and the same order at later dates

Status: a plan by the architect, written on Nov 20, 2026, inside the plan for step 3, stage 4 (decision 0161, step 8 of its ten, section G, rule 18, and section F from a seat). It is the second record of step 8; the first is decision 0178. It is to be read by a reviewer who has not seen it written, and amended, before any of it is built.

Summary: A commander can ask what an order would cost. This record lets him ask the three questions that follow. What if I change one thing: the draft is tried beside the plan as it stands, in the same worlds, and the two are set side by side. What does this order cost me: the side is tried with the order and without it. What if I give it later: the same order is tried at two or three dates. Each is the tool `trial` with one more argument. Nothing new is played: the trial, the estimate and the window are those already built

Standing: A plan, written by the architect on Nov 20, 2026, inside decision 0161. Not yet reviewed and not built

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
- **Without it.** Beside `without_it` the side loses no man and the place is not taken.
