# 0086: The numbers a side is given

Status: decided by the architect on the designer's delegation of Nov 13, 2026. It closes the numbering leaks that decision 0085 (item 10) left open: the leak decision 0047 recorded in "What it costs", and findings 3 and 8 of the outside review of Nov 13, 2026 and the team's own finding A (pull request 213). It is built in two parts. Part 1 changes no run, state or hash. Part 2 raises the state version to 8.

## The gap

Every number the game uses is counted for both sides together, and a side saw some of them.

- **The log's event numbers and the order book's numbers.** Events are one log for both sides, and orders are one sequence. sim-agent already showed a player the side's own count of each (decisions 0048 and 0055). But a `SideView` handed a Rust caller the log's numbers: on every event, its causes, its own orders, its own units' standing-order fields and its contacts. An enemy order that was refused, and which the side never saw, moved every later number. The staff planner of step 3 will hold a `SideView`.
- **The scenario's unit numbers.** A scenario numbers both sides' ships, task forces and air groups in one series. In scenario 5 the Allied ships are 1 to 18 and the Japanese 19 to 36, so the Japanese player could read from its own `ship:19` that the enemy has up to 18 ships.
- **Units made in play.** A force detached in play, or a group formed by aircraft landing away from home, takes the next number after the highest of both sides. Its number counts the enemy's units. Random draws are keyed on a unit's number, so an enemy unit that does nothing changes the luck of a force made later.

## Decisions

### Part 1: what a side is shown (no change to a run)

1. **Event, order and contact numbers are opaque outside the simulation core.**
   - An `EventId`, `OrderId` or `ContactId` can be compared and kept, and holds its order within a side: a side knows the order of what it was told.
   - It cannot be read, printed or made outside sim-core. Its text and its debug print show its kind and no number, and it has no `std::hash::Hash`, since a hasher fed one would read the number.
   - The umpire's tools (the runner, `events`, `export`, `batch`, saved games, the validation harness and the tests) read and make these numbers with an `UmpireKey`. Only a `World` gives the key, and nothing that plays the game holds a world.
2. **The view counts each side's own.**
   - A side's events are numbered 1, 2, 3... in the order it is given them. Their causes are given by those numbers, and by the side's own count of its orders.
   - `SideView` answers both ways for events, orders and contacts: `event_number` and `event_by_number`, `order_number` and `order_by_number`, `contact_number` and `contact_by_number`.
   - A contact's number was already counted for each side, and stays as it was.
3. **A side's own units are shown numbered among its own.**
   - Its ships, task forces and air groups are numbered 1, 2, 3... for each kind, in the order of their numbers in the scenario. Every served answer and every handle a player writes uses these numbers.
   - The numbers count every unit the side has had, sunk ships and lost groups among them, so a unit's number never changes in a game.
   - Bases keep their numbers. They are on the map for both sides, and an order names an enemy base by its number.
   - A number that names no unit of the side gets the same answer as before for a unit that does not exist.
   - The engine still works in the scenario's numbers. sim-agent translates at its edge, so saved games are unchanged.
4. **The recorded games are translated, not replayed differently.**
   - The requests of the recorded games name units by the old numbers. A script, `translate_handles.py`, rewrites each handle by a table kept beside the game (`handles_0086.json`) and changes nothing else.
   - The proof that each is the same game: its `save.json`, which records the orders in the umpire's numbers, is byte for byte as it was, and its transcript differs only in the handles shown (`check_translation.py`).

### Part 2: units made in play (state version 8)

5. **A unit made in play takes a number from its own side's series.**
   - The Allied side's task forces and air groups made in play are numbered from 1,000,001, and the Japanese side's from 2,000,001, each kind with its own counter for each side, kept in the state.
   - The loader refuses a scenario task force or air group numbered 1,000,000 or above.
   - The state hash covers the new counters, and its version goes to 8.

## Why

- **Opaque, not renumbered, in the core.** The log must keep one numbering, so that two sides and the umpire can name one event. A number that cannot be read outside the crate gives a side nothing to count, and keeps the log as it is.
- **The key, not a rule of conduct.** What may read a number is said by the type. A planner that is handed a view and no world cannot ask for the key. sim-agent's source is also checked for the words that would get round it: `umpire`, `World::new`, `IdAllocator`, `Hasher`, `EventLog`.
- **Handles at the edge, scenario numbers in the core.** Renumbering every scenario file would touch about forty files and four hundred lines of tests. It would also change every draw of every Japanese unit, and so every result. Translating at the edge changes only what a side is shown.
- **Ranges for units made in play.** A count for each side alone would collide with the other side's numbers in one table. A range for each side, far above any scenario number, does not, and a unit's draws then depend on its own side's history only.

## What it costs

- **A Rust test or tool that needs a number must hold the key.** About 130 places in the tests changed their syntax. No expected number changed, other than the side's own counts in the unit tests of `side_events.rs`, which now count from the side's own first event.
- **A player's handles are not the umpire's.** `task_force:3` for the Allied player of the full slice is `task_force:5` in `sim-cli events`. The viewer and the umpire's tools show the umpire's numbers.
- **The accounts of the recorded games quote the old numbers.** Each has a note pointing to its table.

## Built

Part 1, Nov 13, 2026. No run, state or hash changed: the run baselines, goldens of the engine and every recorded game's `save.json` are unchanged. The sim-agent goldens and the transcripts changed only in handles. The umpire's output was compared byte for byte between a binary built from main (0c5c01f) and one built from this change, for scenarios 4, 5, 6 and 7: `export`, `run --events`, `events --unit` (a ship and a task force) and `events --why`. All twenty files are identical. In the scenario tests that moved to the key, no expected number changed: only how an id is read. The test for finding 8 is switched on. Finding 3's served form is tested in `crates/sim-agent/tests/numbering.rs`. Its core test, and the test of the force leak, wait for part 2.

Part 2: not yet built.

## Rule card

| | |
| --- | --- |
| Reads | Part 1: the side's own units, events, orders and contacts. Part 2: the side of the unit a detach or a landing makes |
| Writes | Part 1: nothing. Part 2: one counter for each side and kind of unit made in play |
| Events | None new |
| Randomness | None new. Part 2 changes the number, and so the draws, of a unit made in play |
