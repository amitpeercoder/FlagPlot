# 0043: The determinism scenario, saved games and replay

Status: confirmed by the architect, on the designer's standing word of Nov 11, 2026. Decision 2 reads a phrase of the spec in one of two possible ways and says why.

Summary: The determinism scenario, saved games and replay: the full slice, what shuffled order means, `--save`, `replay` and `hash`

Standing: Confirmed

## The gap

The spec's scenario 7 is "Determinism: the full slice run twice, and once with shuffled processing order", proving identical daily hashes. It also lists `replay` and `hash` among the commands of the runner, and asks that a saved game, which is the starting state, the seed and the orders of each day, replays to every daily hash. None of this exists. No scenario yet runs ships, carriers, an air campaign and a convoy together.

## Decisions

### The full slice

1. **Scenario 7 is one scenario with everything in it,** on the real map, for fourteen days from 21 August 1942: the carrier forces of the Eastern Solomons scenario, the air groups, arrivals and replacements of the Rabaul against Guadalcanal scenario, a convoy that carries supply to Henderson Field, and a night surface action. It names every figures file. It is assembled from the scenarios that exist; it is not a reconstruction and nothing is compared with history. Its only claim is that it runs and repeats.

### What "shuffled processing order" means

2. **The order things are given in is shuffled, not the order the rules work in.** The scenario is loaded with its ships, task forces, air groups, bases, arrivals, replacements and each day's orders for different units listed in a different order, several times over, and must give the same daily hashes and the same events.
3. **The rules' own order is not shuffled, because it is part of the rules.** Losses come off the lowest-numbered group first; forces move in order of clock time; a base repairs the lowest-numbered group first. Shuffling those would change the game, not test it. What the rules promise is that nothing depends on the order of the data or of memory, and that is what is tested. Random numbers are already keyed on the day, pulse, subsystem and unit, and a test in `rng.rs` checks that they do not depend on the order of asking.
4. **The orders of one unit on one day keep their order.** A later order to the same unit replaces an earlier one, so their order is meaning, not accident.

### What the first shuffle found

4a. **Order numbers followed the order orders were handed in.** An order gets its number when it is taken in, and the number appears in the event that accepts it and in the causes of everything the order leads to. So two files with the same orders listed differently gave different events and different hashes for the same game. The Eastern Solomons scenario was the only one whose file did not already list its orders unit by unit.
4b. **Orders are now put in one order before they are numbered:** when a day's orders are taken in, each side's are sorted by the unit they are for, task forces before air groups and then by number, keeping the order of one unit's orders as given. The sides are taken in their fixed order as before. This is done where the orders are taken in, not in the file loader, so that orders from a player, a staff or a model get the same treatment as a scenario's.
4c. **Arrivals and replacements are kept in one order too,** whatever order the file lists them in.
4d. **It costs four scenarios their order numbers or the order of their replacement events:** Eastern Solomons and stage 6c, whose files give a unit's orders apart from each other, and stage 6d1 and Rabaul against Guadalcanal, whose files list replacements in another order than the fixed one. Their golden files are made again and their tests cite the new numbers and order. No result changes in any of them: no roll is keyed on an order's number, and no two replacement streams compete for the same group. The other nineteen scenarios' golden files do not change.
4e. **Replacements are delivered in order of side, base and aircraft type,** and arrivals in order of day and group.

### Running twice

5. **"Run twice" is three checks:** twice in one process, in two separate processes of the runner, and with and without breakdown recording. The daily hashes are equal in all, and the event logs are equal in the first two.

### Saved games

6. **A saved game is a file that names the scenario, the seed and the orders of each day,** with who issued each and for what intent. `run --save FILE` writes one. It holds no state: the starting state is the scenario's.
7. **`replay FILE` runs a saved game** and reports each day's hash. If the file carries the hashes of the run that saved it, replay compares and fails on the first day that differs.
8. **`hash SCENARIO` prints the daily hashes of a run** and nothing else, one a line, so two runs can be compared with any tool.
9. **A saved game names its scenario file and records that file's own hash.** Replay refuses a scenario file that has changed since, saying so.

### As built

- **A saved game stores a scenario's scripted orders,** in the order they will be taken in. Orders a player or a model gives during a game are not yet saved: nothing gives any. The agent interface will need the runner to record orders as they are submitted. *(Nov 14, 2026: a saved game holds the orders as they were handed in, by decision 0050, item 11)*
- **Paths in a saved game are as given on the command line,** so a game is replayed from the directory it was saved in.
- **The full slice produces every kind of event the engine can record,** 58 of them, and a test reads the list from the code so that a new kind without a place in the slice fails. Two things are in the slice only for that: six damaged fighters at the start, and one order sent to the wrong side's force.
- **The shuffle test fails with the sort of 4b turned off,** in all four scenarios it runs on. So it does detect what it was written for.

## Why

- **Why not shuffle inside the steps.** Almost every step ends by applying something in a fixed order, and the event log is in that order. A shuffle there would change event numbers and so the hashes, and prove only that the order is used. The risk worth testing is the other one: that a result leans on the order a file was written in or a map was filled in.
- **Why a saved game holds no state.** The scenario is the starting state, and the rule of the project is that the same state, orders and seed give the same hashes. A copy of the state in the save would be a second truth to keep in step.

## What it costs

- A saved game is tied to its scenario file. A game cannot yet be saved in the middle and resumed from there; it is replayed from the start.
- The full slice is a patchwork of three battles that did not happen in one fortnight in this way.

## Deferred

- Saving in the middle of a game.
- `query events` and `serve` in the runner. *(Nov 14, 2026: built: `sim-cli serve` by decision 0050 and `sim-cli events` by decision 0052, item 9)*

## Rule card

No rule of the game is added. The runner gains `--save`, `replay` and `hash`.
