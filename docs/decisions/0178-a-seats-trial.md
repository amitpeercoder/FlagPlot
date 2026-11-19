# 0178: A seat's trial: the tool, who runs it, and what the commander is shown

Status: a plan by the architect, written on Nov 19, 2026, inside the plan for step 3, stage 4 (decision 0161, step 8 of its ten, section G, rules 12 and 21). Step 8 is written in two records: this one, the tool `trial` and the estimate a commander is shown; and the next, what-ifs, the window from a seat, and the trial the staff's leave is asked with. To be read by a reviewer who has not seen it written, and amended, before it is built.

Summary: A trial can be played and an estimate read, by a program. This record gives them to a commander. A seat with a staff gains the tool `trial`: it names an intent of its own or a draft it has previewed, and a size, and is answered with the estimate in words a commander can read, or with why it cannot be tried. The session that holds the game cannot build worlds and must not learn to, so it is handed something that can, as it is handed its staffs. The estimate is kept with the session as a thing the commander was shown. The game is not changed by asking

Standing: A plan, written by the architect on Nov 19, 2026, inside decision 0161. Under review

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

1. **`sim-core` defines what a session asks and is given,** as it defines the staff's trait: what is asked (the trial's number, a size by its name or its two numbers, the course, the standing commands, and the intent whose estimate is wanted); the estimate, as plain data; why a trial is refused, in words; and a trait with one call, which takes a side's view and what is asked and answers with the estimate or the refusal.
2. **The estimate's data moves to `sim-core`** from `sim-trial`, unchanged in what it holds: it is plain data, as a plan order is. `sim-trial` fills it. Its programmer's words stay in `sim-trial`.
3. **`sim-trial` gives the thing that runs trials:** a value that holds, for each side, what the scenario says that side believed, with the trial's figures, the reference data and the map, and implements the trait by playing a trial (decision 0175) and reading its estimate (decision 0176).
4. **`sim-cli` hands it to the session,** as it hands the staffs, where the scenario names a file of the trial's figures. A scenario that names none has no trial: its sessions are as they were, and the tool says so.

### The tool

5. **`trial`** is a tool of a seat that has a staff and plays a whole side: where the tools of command are. Its arguments:
   - `intent`, the number of an intent of the commander's own, in force or queued for the morning; or `preview`, the number of a preview the seat was shown, to try that draft. One of the two.
   - `size`: `small`, `standard` or `full`; `small` where none is given.
6. **The course.** The standing commands are what would be taken in on the coming morning without the trial: the scenario's commands of that day, and what the seat has queued. The course is then:
   - for an intent: leave to prepare and leave to launch, each where the side has not given it, so that the intent is tried as approved (decision 0161, 10);
   - for a preview: its command, and the same leaves.
7. **The trial's number is 1,** always. The same question on the same evening is the same worlds and the same estimate (decision 0161, condition 5), and two questions on one evening are tried in the same worlds.
8. **It is refused,** with words that say why and nothing played: at a seat of a command; with no staff; where the game has no trial (rule 4); while a day is open; with both `intent` and `preview` or neither; an intent that is not the side's own, is the staff's, or is withdrawn; a preview that is not the seat's or is void; a size the figures do not name; and whatever the runner refuses (a side whose own units reach the numbers kept for the enemy's, decision 0174, 20).
9. **It changes nothing.** No command is queued and no preview is spent. The live game's hash is what it was, and a preview tried may still be approved.

### What the commander is shown

10. **The answer** is of the kind `trial`: the estimate's number among those the seat has been shown; what was asked; the estimate's fields, with every unit by the side's own number and name, every place as a seat's tools give a place, and every kind and task in the words the seat's other tools use; and its words.
11. **The words are a commander's.** They hold no name of the engine's: no kind, task, unit or place as the program prints it. They say, in this order: what was tried and in how many worlds; how it went; what it cost the side; what the trial's enemy lost, as the trial's own reckoning; what it needs and has and when it could go; what was assumed; where the worlds part. They give counts of worlds and no chance, no per cent and no points (decision 0176, 15).
12. **The enemy's units in a trial are never named to the seat.** The trial's enemy is an invention; the estimate gives his losses as counts and his assumed forces by what the side's own belief they were made from, and no number of a unit made for him.
13. **The estimate is kept with the session,** numbered from 1 among its side's, with the day and the moment it was asked and what was asked. It is no state, no event and no hash, and it is not in a saved game. The tool `estimates` lists what the seat has been shown, and gives one again by its number, unchanged.

## What it leaves for later

- **What-ifs, the window from a seat, and the trial the staff's leave is asked with:** the next record.
- **The table's "Dry run":** the page, after both records, with the busy state a call of seconds needs.
- **Stopping a trial from a seat.** A call is answered when it is done. The full size is 16 s; whether a seat needs a way to stop one is for the table to show.
- **A commander's direct orders in a course** (decision 0175, amended, 2). A seat's queued orders to units are not tried: the trial says so where the seat has any queued.
- **A trial inside a day.**

## Rule card

- **Reads:** a side's whole view on an evening; the session's queue and previews; through the runner, what decisions 0175 and 0176 read.
- **Writes:** nothing in the game. The session's own list of estimates shown.
- **Reuses:** the way a session is handed its staffs; the trial and the estimate; the seat's words for a task, a unit and a place; the preview.
- **New:** in `sim-core`, what a trial is asked and the trait; in `sim-trial`, the runner; in `sim-agent`, the tools `trial` and `estimates`, their answers and words.

## What it costs

- **The agent interface** gains two tools and two kinds of answer; its version rises, as it has for every tool added, and the version in every schema and recorded transcript with it.
- **No game changes:** no state, no draw, no hash. A scenario that names no trial figures plays, and its sessions answer, as before but for the two tools' names in the list.
- **The 39 days** are given a file of the trial's figures only where a scenario is to be tried from a seat; the arrangements of the proofs are not changed by this record.
- **Documents:** the agent's schemas and their README; `docs/commands.md`; the rulebook's chapter on the trial; `docs/engineering/layout.md` for the trait and who hands it in; the glossary.

## The hand calculation

It is in `tests/scenario_97_a_seats_trial.rs`, written before the code: scenario 95 played from a seat. The Allied commander queues the take, its forces limit and his leave to launch, and asks `trial` at a size of 4 worlds for 8 days. He is answered with scenario 95's estimate: met and kept in 2 of 4 worlds on day 4, his men lost 100, 100 and 300. The game is as it was, and he may still change his mind.
