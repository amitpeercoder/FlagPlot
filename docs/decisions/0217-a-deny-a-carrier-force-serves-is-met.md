# 0217: A deny that a carrier force serves is met, and a plan's cautions are said apart

Status: a ruling by the architect, written on Nov 22, 2026, on two things the designer found at the table. Of a plan that waited for his leave he said: "This is a wall of text. I don't suppose any human will read it." Of an order the staff said it could not meet: "It says that it cannot be met because there are no airbases with bombers nearby, but what about the aircraft carriers that are nearby?" It mends a fault and adds nothing a side can do.

Summary: Where no base can put a bomber or a searcher over the waters of a deny, the staff says it cannot meet the order, even while carrier forces it has itself stationed there serve it. It now says so only where no carrier force serves the order either. And the two things a commander needs before he approves a plan, that a station stands in danger and that a force's fuel could not be measured, are handed to him as items of their own beside the staff's working, where today they are sentences inside it.

Standing: Ruled on Nov 22, 2026; amended by the architect on the build's finding the same day; built on Nov 22, 2026 (see "As built"), for the architect to read.

## Why

- **The fault** (`sim-staff`, `deny.rs`; decision 0202, rule 28): the staff gives the reason "No base ... The staff forms no task and cannot meet" and the notice that it cannot meet the order whenever no base serves a deny. Rule 28 added to that reason the words that no carrier force can serve it, where none can. Where one does serve, the notice and the words "cannot meet" are still given. In the designer's game of the 39 days the order to deny the waters round Guadalcanal had two carrier forces on station under it and was counted among the orders that cannot be met.
- **The wall of text:** the reason of a station chosen for a carrier force is one paragraph of the staff's working. The table now folds it and lists what the plan would do, and cannot list what he should know, since those are words inside the paragraph.

## The rules

1. **A deny is not met by a base alone.** Where no base the order may use can strike or search its waters, and a carrier task under the order is in force or is formed at this dusk, the staff gives no notice that it cannot meet the order. Its reason for giving the bases no task says first which carrier forces serve the order, then what it says today of the bases and of what is coming, and does not say "cannot meet".
2. **Where no carrier task serves it** the staff says it cannot meet the order, in today's words, with the notice, as now.
3. **Nothing else of the deny changes:** what it forms, the orders it gives and the days it gives them.
4. **A reason may carry cautions.** A caution is a short statement of something the commander should weigh before he approves, with the unit or place it is of. The staff gives two, where today it says them inside its working:
   - **the station stands in danger:** the station chosen lies inside the circle of an enemy base or landing, which it names;
   - **the fuel was not measured:** the force has no home port the staff knows.
   The working keeps its sentences as they are.
5. **A side is shown a reason's cautions wherever it is shown the reason,** and the tool that says what a plan would form gives each task's cautions as a list. The table shows them under the list of what a plan would do, each a line, with no number but a name or a place.

## What is recorded and who knows

A reason is recorded with the staff's orders and commands, as now, and gains its cautions. A side sees its own. The state and its hash do not change. The doctrine's version and the agent's interface rise; recorded games are remade, and the records of closed proofs are not altered.

## Rule card

Reads: the deny's sub-tasks in force and those formed this dusk; the station's circles and the force's home port, as the station rule reads them. Writes: nothing new; the notice not given. Draws: none.

## Games on file

No game moves in what any unit does. A game in which a deny had no base and a carrier force records one notice fewer each day, and an interrupt for an order that cannot be met no longer fires there.

## Statements for the tests

Written by the architect; a test that disagrees comes back to him.

1. A deny with no base in reach of its waters and one free carrier force: the carrier task is formed as today; on no day is the notice that the order cannot be met handed in; the standing interrupt for it does not fire; the reason for the bases names the carrier force first and has no "cannot meet".
2. The same with no carrier force on the side: the notice and today's words, unchanged, each day.
3. The same with the one carrier force held by the commander: the notice, and the words that no carrier force can serve it, as today.
4. A deny with a base in reach: every order, reason and notice as before this ruling.
5. A station inside an enemy circle: the reason of the task that stations the force has the caution that the station stands in danger, naming the enemy place, and the tool's list for that task holds it. A station outside every circle has none.
6. A force with no home port: the caution that its fuel was not measured. A force with one: none.
7. Every staffed scenario built before this gives the orders it gave.

## Amended by the architect on the build's finding, Nov 22, 2026

The build found the "Why" wrong in where the fault lies. It is not in a commander's deny: decision 0202 already mended that (its carrier pass marks the order served, and scenario 116's statement 8 holds that no notice is handed in). It is in the deny the staff forms under an order to control an island (decision 0151, item 6), whose carrier task `carriers_for_ends` does not read, because it reads the commander's orders alone: in scenario 76's carrier variant that deny handed in the notice on every day of 14 with its carrier task in force under it. The designer's order was "Control Guadalcanal". Rule 1 holds for every deny, whoever made it. Statements 1 to 3 already held before the build and are kept as guards. A statement is added, the architect's:

8. In scenario 76's carrier variant the deny the staff forms under the order to control the island has a carrier task under it, and on no day of the 14 is the notice that it cannot be met handed in for that deny; its reason for the bases names the carrier force first and has no "cannot meet". Every order of every day is what it was.

The island's neutralise, which says it cannot be met while the carrier's bombers raid its target, is not in this ruling: it is row 59 of `docs/rules/differences.md`, open.

## As built, Nov 22, 2026

- **Rule 1** (`sim-staff/src/deny.rs`, `denials`): the doctrine keeps apart whether a base served, whether a force its forces limit names was given a task, and whether a carrier task serves: a task "operate a carrier force" in force under the deny (`carrier_tasks_of`, which reads the children in force and so sees one kept, or formed that morning by the carrier pass, the island or the forming rule), or the carrier pass's mark. With no base and a carrier task, `carriers_serve_deny` hands in one reason for giving the bases no task and no notice: it begins "intent:6, to operate the carrier force task_force:2 from (0,0), serves intent:5.", then the bases as "cannot meet" words them (the shared `base_lines`), and has no "cannot meet" and no words that no carrier force can serve it. This reason is new also for a commander's deny the carrier pass serves, which before gave no reason for its bases at all. A stand task alone gives nothing, as before; nothing serving, `cannot_deny` as before.
- **Rules 4 and 5:** `Reason` gains `cautions` at the end, a list of `Caution` (`kind`, `of`, an optional `EntityRef`, `hex`, an optional hex, and `words`); `CautionKind` is `StationInDanger` (code 1) and `FuelNotMeasured` (code 2). Only the reason of a carrier station the staff chose carries them (`station.rs`, `choose_station` and `form_station`): one for each circle that holds the station, of the enemy base whose circle it is or, for a contact's circle, of no unit and the contact's hex; and one where the force has no home port. The agent writes `cautions` in a reason only where there are any, so no saved game or golden moves for it, and reads it back from a save; `propose` gives each sub-intent's `cautions` (an empty list where none); `sim-cli events` writes their words. The state and its hash are unchanged: the hash counts the log's events and does not read a reason.
- **The table** shows the cautions under the list of what a waiting plan would do, in the warning ink, by name: "Task Force 16 would stand within reach of Lunga, Rabaul, Tulagi and Shortland", "Task Force 16's fuel was not measured: it has no home port"; a contact's circle reads "an enemy carrier reported near" the place.
- **"Enemy base or landing"** in rule 4 is read as the circles the station rule casts: an enemy base on the side's map, and a fresh contact reported as a carrier.
- **Tests:** `tests/scenario_116_carriers_serve_an_end.rs`, `decision_0217_statement_1` to `_6` (statement 1 also counts the standing interrupt's firings: none, and one in statements 2 and 3); `tests/scenario_76_controlling_an_island.rs`, `decision_0217_statement_8`. For statement 8 every order and command of the 14 days was dumped before and after the change and the two were the same; only the deny's notice and its reason changed. Statement 7: the order-bearing events of `sim-cli run --events` on the games whose pins moved (scenario 55, the five days' and the second proof's Eastern Solomons, the 39 days' first week, the second proof's week) were compared with a build of the commit before: the same, line for line. Their pins moved by the reason's new field and the reason added or the notice removed.
- **Versions:** doctrine `supply-54`; the agent interface 45, once for the batch with decisions 0215 and 0219.
