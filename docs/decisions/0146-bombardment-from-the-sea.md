# 0146: Bombardment from the sea: step 5 of stage 3, second part

Status: settled by the architect on Nov 17, 2026, inside the plan for step 3, stage 3 (decision 0139, step 5 of its nine). It writes the task "bombard a base", given to a task force, and the doctrine that serves it, over the rule of shore bombardment (decision 0092), the halt at a marked waypoint (decision 0096) and the doctrine of a force that stands by night (decisions 0124 and 0127), which it follows closely. It adds no rule to the engine. It is built after decision 0145. Built on Nov 17, 2026, with decision 0145: see "As built".

Summary: Step 5 of stage 3, second part: the task "bombard a base", given to a task force on the bombardment mission, by which the staff sends the force to the enemy base in the night, at the speed that brings it there in the dark, with the fuel and the rounds its risk asks for, and home at the morning; formed under "neutralise a base" and under "take a place", where it fires on the night of the day the landing is due

Standing: Settled by the architect, Nov 17, 2026, inside decision 0139. Built; its readings ruled (The architect's rulings)

## Why

The engine has had shore bombardment since decision 0092: a force on the bombardment mission that ends the night in the hex of an enemy base fires at it. Only a scripted order sends one. The record of the campaign has the airfield shelled from the sea night after night. This record gives the staff the task.

## The task

1. **Bombard a base** names a task force of the side and a base on the side's map that is not the side's: the target. It has no date. It is a task, given to the task force (decision 0124, item 6), and a commander may state it as a root.
2. **It is formed by the staff** under an end whose plan is released, for each task force on the bombardment mission with a ship afloat that the nearest forces limit on the end's chain names, in task force order, where no bombard task for that force is in force anywhere on the side:
   - under **neutralise a base,** at that base;
   - under **take a place,** at the enemy's lowest-numbered base on the side's map in the place's hex. With none there, none is formed.
3. **A force on the bombardment mission is given no stand task,** as now: the mission says which task a force takes.

## The force's morning

Each morning, for each bombard task in force whose plan is released and whose force is not held, in priority order, the staff decides at the force's level from the side's view. It is the stand doctrine's morning (decision 0124, items 9 to 15) with these differences.

4. **The target.** Where the target is no longer the enemy's, or is not on the side's map, the force is given no route; it is set to normal readiness where it is not; and the reason says so.
5. **Whether it sorties tonight.**
   - **As a root, or under neutralise:** at press and at balanced, every night. At preserve the force, every night on which the side holds no contact of ships within 15 hexes of the target that reports more ships than the force has afloat.
   - **Under take a place:** on the morning of the day the landing force is due at the place (decision 0142, item 18), and on no other. Before the landing force is routed that day is not known, and the force waits.
6. **The speed and the route.** As a stand: the force must lie stopped at its home port; it is routed to the target's hex and home, the target's hex marked to act at; cruising speed, then full, the first that brings it there within the night's 12 hours. Where neither does, there is no sortie and the notice that the intent cannot be met is handed in.
7. **Fuel and ammunition come first,** by the stand doctrine's rule 4 and decision 0127, at the task's risk. A force short of either does not sortie; its reason gives the day from which it may be used, or none and the notice.
8. **Readiness and rules of engagement** as a stand: battle readiness on a morning it sorties and normal on a morning it does not; the rules of engagement set once from the risk.
9. **The order of the orders** is the stand's: home port, rules of engagement, readiness, speed, route.
10. **It fires and comes home by the rules as they are.** The force halts in the target's hex, fires in the night's bombardment phase, and steams on at the start of the morning pulse.
11. **Reasons** give the target and its distance, the hours to it, the fuel aboard against the fuel needed, each ship's rounds against the level, and, at preserve the force, the contact that stopped it.

## What it leaves for later

- **What a bombardment did,** read by the staff: a side is told its shells and never its hits (decision 0092). A task that stops when the base is judged out of action waits on what a side can know of that.
- **A bombardment timed with a raid from the air or with a surface fight,** and a force that covers the bombarding force.
- **A force that rearms at another base** when its own holds no ammunition: moving a force's home is the commander's (decision 0127).
- **Bombardment on the nights before a landing,** which wants a choice of how many: stage 4.

## Rule card

- **Reads:** the side's own task forces and ships (place, fuel, rounds, mission, readiness, home port), its map of the enemy's bases, its contacts of ships, the forecast of a passage, the ammunition and fuel figures of decision 0113 and 0127.
- **Writes:** one kind of intent with its task force and target, in the command state; nothing else.
- **Reuses:** the stand doctrine whole but for its threat and its mark; `form_intent`; the home port, rules of engagement, readiness, speed and route orders; the engine's bombardment and halt.

## What it costs

- **State:** one kind of intent, in the hash and named. The state layout, saved game, agent interface and doctrine versions rise. The rules label does not.
- **The rulebook's chapter 8** gains "Bombarding a base", and "A task force as an addressee" gains the third task.

## The hand calculation

`tests/scenario_71_bombardment_from_the_sea.rs`.

## As built

Built on Nov 17, 2026, on the branch `stage3-step5`, with decision 0145. Nothing is tuned and no figure changed. Every number of the hand calculation that an event, the state or a reason carries is asserted exactly in `tests/scenario_71_bombardment_from_the_sea.rs` (16 tests), and no test is ignored: no mismatch was found. The halts are at elapsed hours 10, 34, 58 and 82; the force steams on at the start of the morning pulses, hours 12, 36, 60 and 84, and is home at hours 22, 46, 70 and 94; base 1 ends with 4,680 t of fuel.

### What was built

- **The kind (`crates/sim-core/src/command.rs`, `command/intake.rs`, `hash.rs`).** `IntentKind::Bombard { force, target }`, code 14, name `bombard`, addressed to the force; its `force()` and `target()` are its two fields, so intake checks the force is the side's and the target a base of the other side on the map. It may stand under "neutralise a base" of the same target and under "take a place", where intake checks the target lies in the place's hex. Hashed after its code, the force then the target, and named in `name_every_field`; state layout `flagplot-state-v42`.
- **The doctrine (`crates/sim-staff/src/bombard.rs`, new):** `Day::bombards`, run last in the morning (after "take a place" launches, `air.rs`), forms the tasks under each released neutralise and take-a-place end (`form_bombard_tasks`) and serves each task (`serve_bombard`), with the landing force's due day under take a place (`landing_due`). The stand doctrine's helpers (`stand.rs`) are used as they are.
- **Plumbing:** the scenario file (`crates/sim-data`), the saved game (version 17, `crates/sim-cli/src/save.rs`), the replay export, the agent's commands, tools, schemas and interface (version 21), the viewer's words, addressee and shape (`viewer/index.html`, `viewer/check.js`), `docs/commands.md`.
- **Scenario 71** (`data/scenarios/71_bombardment_from_the_sea.toml`) with its own surface combat figures (`data/reference/surface_combat_placeholder_scenario_71.toml`, the frozen surface combat figures with the three bombardment figures of the hand calculation, frozen beside the others in `data/reference/step1/` with its README's rule), so that run by itself it plays the game its test proves; its live run and its run on the frozen figures give the same orders, halts and shells.
- **The rulebook:** chapter 8, "Bombarding a base" and the third task under "A task force as an addressee", the day's order and the versions; chapter 1, the state hash and the saved game; chapter 4, Shore bombardment, one line; the glossary. Doctrine `supply-26`.

### Readings taken

1. **The mission** is not checked by intake: a commander's own task given to a force on another mission is served as written, and the engine's halt then finds no bombardment to wait for. The staff forms the task only for a force on the bombardment mission.
2. **The target is read first** after the force's own checks (gone, before its day, held, no ship afloat): where it is no longer the enemy's on the side's map, the force gets the readiness order and no other, and no notice.
3. **The notice of "neutralise a base"** with no bomber that reaches the target is handed in as before, a bombard task formed under it or not.
4. **The due day under take a place** is, on the morning the landing force is routed (its route order in the same morning's hand-in), that day plus the days of the passage at the cruise speed of its slowest class less one (decision 0142, 18), even where the lift goes at full speed by decision 0145; while the landing force is under way to the place, the day the engine's forecast brings it there; once it has come there is none, and the force does not sortie again.
5. **A plan ended this morning:** a bombard task whose plan was released at the staff's hour and is marked complete by the staff that morning (the place taken) is still served, so that the force is set back to normal readiness (the hand calculation's day 6).
6. **At preserve the force** a contact stops the sortie when it reports more ships than the force has afloat (strictly more), within 15 hexes of the target, read from the force's own picture.
7. **The forming runs in the bombard step,** not in the neutralise doctrine's own pass, so its `form_intent` follows the morning's other commands; a force before its day is left out, as the stand's forming does.
8. **The reason of a sortie** gives each ship's rounds against the level of the risk where it is not short ("ship:1 has 800 rounds against the level of 400"), beside the stand's words for the target, the hours and the fuel.

### Earlier tests touched

No expected number changed. The saved game's version pins (16 to 17: `crates/sim-cli/src/save.rs`, `crates/sim-cli/tests/saved_games.rs`), the agent interface's (20 to 21: `crates/sim-agent/tests`, the schemas), and the export hashes of `crates/sim-cli/tests/export_pictures.rs` remade at the state layout v42, which changes an export's state version and day hashes and nothing else. The two tests that compare every run baseline with a run that has no staff (`tests/scenario_06d1_pilots_and_replacements.rs`, `tests/scenario_06d2_base_supply.rs`) leave out scenarios 70 and 71, as they leave out every scenario played by the staff. The run baselines, pinned hashes, batch fixture and recorded games are remade by `scripts/regenerate.sh`; what changed in them beyond versions and hashes is decision 0145's (its As built).

### Files to read

- `crates/sim-staff/src/bombard.rs`; `crates/sim-staff/src/night.rs`, `doctrine.rs` (`night_load`, `night_dispatch`) and `take.rs` (`move_step`, `lift_night`) for decision 0145.

## The architect's rulings

Made on Nov 17, 2026, on reading the build. Readings 1 to 8 are accepted as built. On reading 6, a contact that stops a sortie at preserve only when it reports strictly more ships than the force has afloat is a placeholder, as the rounds kept back are: the record gave no figure for either (`docs/research/oob/stage-3-leads-checked.md`, section 3).
