# 0178: A seat's trial: the tool, who runs it, and what the commander is shown

Status: a plan by the architect, written on Nov 19, 2026, inside the plan for step 3, stage 4 (decision 0161, step 8 of its ten, section G, rules 12 and 21). Step 8 is written in two records: this one, the tool `trial` and the estimate a commander is shown; and the next, what-ifs, the window from a seat, and the trial the staff's leave is asked with. Read by a reviewer who had not seen it written, and amended on that reading ("Amended on review", at the end) before it is built.

Summary: A trial can be played and an estimate read, by a program. This record gives them to a commander. A seat with a staff gains the tool `trial`: it names an intent of its own or a draft it has previewed, and a size, and is answered with the estimate in words a commander can read, or with why it cannot be tried. The session that holds the game cannot build worlds and must not learn to, so it is handed something that can, as it is handed its staffs. The estimate is kept with the session as a thing the commander was shown. The game is not changed by asking

Standing: A plan, written by the architect on Nov 19, 2026, inside decision 0161, and amended on review. To be built on the build of decision 0177

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

