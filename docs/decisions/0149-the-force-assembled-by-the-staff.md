# 0149: The force assembled by the staff: step 7 of stage 3

Status: settled by the architect on Nov 17, 2026, inside the plan for step 3, stage 3 (decision 0139, step 7 of its nine, added by the designer on Nov 17, 2026). It lets "take a place" be stated with no forces named, or with some: the staff chooses the rest. It adds no rule to the engine. It is built after decision 0148. Built on Nov 17, 2026: see "As built".

Summary: Step 7 of stage 3: "take a place" may be stated with no forces named; the staff then works from its estimate of the enemy to the men wanted, the base they can sail from, the units by what each is good at, the transport force, and the bombardment and carrier forces it can give, keeps its choice as the forces limit of the task it forms, and says in numbers what it could not find

Standing: Settled by the architect, Nov 17, 2026, inside decision 0139. Built; its readings ruled (The architect's rulings)

## Why

Decision 0142 has the commander name the transport force, the units and the base they sail from. That is the staff's work. The designer set out the order a staff works in (decision 0139, "Amended by the designer"): the enemy, the men needed, the lift, the risk on the way, whether it can be kept up, and then the landing made ready. Steps 3 to 6 built each of those over forces the commander named. This step lets him name none.

## What the commander may leave out

1. **A "take a place" needs no forces limit.** Item 2 of decision 0142, that a forces limit with no source is refused, is withdrawn for this end.
2. **Each of three things is the commander's if he names it and the staff's if he does not:** the source; the land units; the task forces. What he names stands, and the staff chooses the rest inside it.
3. **What the staff chooses is kept** as the forces limit of the task "ready a landing" that it forms: the task forces, the land units and the source. Wherever decisions 0142 to 0148 read the forces limit of the "take a place" for one of the three, they read the commander's where he named it and this one where he did not. The commander may change it, as he may any limit.
4. **It is chosen once,** on the first morning the plan is preparing, and again on the first count of the enemy (decision 0147, item 16).

## What the staff may take

5. **A unit, a force or a base the staff may take** is the side's, one the side may use that day (decision 0140), not held by the commander, and not named in the forces limit of another intent in force that is higher in priority, nor taken by a task of one.

## The order it works in

6. **The enemy.** The side's estimate of the men at the place, with its grade and its day; and the men wanted, by the risk (decision 0142, item 10). With no estimate, as many as can be carried.
7. **The source.** A base can be the source if it is the side's, the staff may take it, a transport force it may take lies stopped there (decision 0142, item 6) and a fighting unit it may take has all its men ashore in its hex. For each such base, in base order, the staff works the lift it could make: the force of item 9 and the units of item 8 there. The source is the base whose lift gives the men wanted with the shortest passage to the place, by the engine's forecast for that force; where none gives them, the base whose lift gives the most men; then the lower number.
8. **The units,** from those at the source that it may take, in the order and by the ships of decision 0142, items 8 and 9, until the men wanted.
9. **The transport force,** of those stopped at the source that it may take: the lowest-numbered whose berths hold the men wanted; where none does, the one with the most berths, then the lowest numbered. One force lands an operation, as before.
10. **The risk on the way.** Every force on the bombardment mission and every force on the carrier mission that the staff may take, that lies stopped at a base of the side or, for a carrier force, anywhere, is taken, and is given its task by decisions 0146 and 0148. The escort is the run's (decision 0126), from the forces on the escort mission that the staff may take.
11. **Whether it can be kept up** is decision 0144's check, with the source chosen.

## What it could not find

12. **The reason of the choice gives each step in that order with its numbers,** and what is short: the men wanted against the men the lift gives; the berths wanted against the berths of the force; that it has no force to bombard, to cover or to escort.
13. **With no base that can be the source** the staff forms nothing, hands in the notice that it cannot meet the intent, once, and gives the reason each morning, with what is missing: a transport force stopped at a base of the side, or a fighting unit beside it.
14. **With fewer men than wanted** it does as decision 0142, item 10: at preserve the force it forms nothing and hands in the notice; at balanced and at press it goes with what it has and says so.
15. **A lack of a bombardment, carrier or escort force stops nothing** beyond what the rules of those forces already say. The reason names it.

## What it leaves for later

- **Forces brought together:** units and ships moved to one base, forces joined, and an operation that sails from two bases. The staff takes what already lies together.
- **Looking ahead to an operation not yet stated,** and keeping forces back for it: stage 4.
- **A source chosen for what it holds,** its supply, its airfield and its port, beyond the check of decision 0144.
- **Weather.**

## Rule card

- **Reads:** the side's own bases, task forces, ships and land units, with their places, missions, days and holds; every intent's forces limit and priority; the side's estimate; the forecast of a passage.
- **Writes:** nothing of its own: `form_intent` and `set_limit` on the task it formed.
- **Reuses:** every rule of decisions 0142 to 0148.

## What it costs

- **State:** none. The doctrine and agent interface versions rise. A "take a place" with a forces limit that names all three plays as before.
- **The rulebook's chapter 8,** "Taking a place", gains "With no forces named".

## The hand calculation

`tests/scenario_74_the_force_assembled.rs`.

## As built

Built on Nov 17, 2026, on the branch `stage3-step7`. Nothing is tuned and no figure changed. Scenario 74 (`data/scenarios/74_the_force_assembled.toml`, on scenario 27's map, its own ground figures `data/reference/ground_placeholder_scenario_74.toml` with `preparation_per_day` 100, frozen beside the others, and scenario 71's surface combat figures) plays the main case run by itself. `tests/scenario_74_the_force_assembled.rs` has 21 tests: 19 pass and two are ignored on a mismatch with the hand calculation (below). The main game gives the commands, orders, notices and hours of the table: six staff commands, eleven orders (six to the landing, five to the battleship), none refused, the limit on intent 2 of task forces 1 and 3, unit 1 and base 1 as the source, the halt at hour 82 and 120 shells, the transports at B at hour 88 and home at 124, 160 and 440 men landed, 10 t and 35 t, base 2 the side's at the end of day 4, the estimate cleared. Every property passes as written.

### Mismatches with the hand calculation

1. **The passage from base 3.** The hand calculation has 24 hours; the engine's forecast for task force 2 from C (0,6) to B (4,0) is 21.2 hours. C to B is 6 hexes but off the grid's axes, so the route is charged its straight line, 212 nm, at 10 kn, as the map's own note says (`data/map/placeholder_preparing.toml`). The choice is unchanged: base 1, 16 hours, is the nearer. `the_passage_from_base_3_is_24_hours` is ignored with that reason.
2. **The base formed by the landing.** The table of day 4 has it as base 3, "Landing at (4,0)"; base 3 is Placeholder base C in the inputs, and the base the first man forms takes the next free number, 4. The name, side, hex and day are asserted in `the_landing_and_the_hours`; `the_landing_forms_base_3` is ignored with that reason.

### What was built

- **The engine, intake and reading (`crates/sim-core/src/command.rs`, `command/intake.rs`).** Item 1: a take-a-place forces limit with no source is taken in; "No source" stays for keeping a base supplied. Item 3: `CommandState::forces_as_bound` reads a forces limit as it binds. Of a "take a place", each of the task forces (with the ships), the land units and the source is the commander's where he named it and, where he did not, that of the staff-made "ready a landing" under it (`assembled_forces`); of an intent under a "take a place", what its own limit does not name of the three is the end's, so read. `forces_on_chain` and `nearest_forces` return limits so read (`Cow`), so every reader of the chain, the check of a staff's order among them, reads it. `forces_narrow_under_take`: under a "take a place" each limit above binds only what it names of the three, so the staff's `set_limit` on the task it formed may name what the commander left out and nothing outside what he named. No state is added.
- **The staff (`crates/sim-staff/src/assemble.rs`, new).** `Day::assemble`, items 5 to 13, called from `prepare_place` (`take.rs`) where the commander's limit does not name all three; with all three named the end is decision 0142's as it was. `rechoice_limit` for item 4. `take.rs`, `look.rs` and `ready.rs` read the source, the task forces and the units as they bind, where they read the end's or the task's own limit before. `Choice` gains `men`.
- **Versions.** Doctrine `supply-30`; agent interface 23 (its schemas' constants, `crates/sim-agent/schemas/README.md`). The state layout (`flagplot-state-v43`), the saved game (18) and the rules label (19) do not change.
- **The rulebook:** chapter 8, Taking a place (The end, Prepare, With no forces named), The forces limit on a staff's orders, What is saved, Not built; the glossary (Take a place, Force assembled by the staff, Limit); `docs/commands.md`.

### Readings taken

1. **"Names all three"** is a forces limit with a task force or a ship, a land unit and a source. Then nothing new happens: the staff's limit names the units chosen alone, as decision 0142 has it, and the end plays as before.
2. **The staff's limit names only what the commander did not:** the transport force and the bombardment, carrier and escort forces taken, where he named no task force; the units chosen, where he named none, or where it chose other units than those he named (so with unit 2 named and chosen, none); the source, where he named none.
3. **"Read as it binds"** is one rule in the engine, `forces_as_bound`, so the staff and the check of its orders read the same limit. A forces limit set on an intent under a "take a place" fills what it does not name of the three from the end's, read as it binds; the air groups are read as set, for every intent.
4. **The narrowing check** under a "take a place": each limit above binds a land unit, a task force or ship, or a source only where it names one of that kind; the air groups are checked as before.
5. **"Higher in priority"** (item 5) is served before the end in the morning's order (the rank, the unranked last, then the number), in another tree. What such an intent names is its forces limit read as it binds; "taken by a task of one" is a task force an intent of it is addressed to; a base so named is its source.
6. **"The staff may take" a base** where the side may use it that day and no higher intent has it as its source; the commander holds no base.
7. **The bases weighed** are the side's own, in base order, or the commander's source alone where he named one. A unit counts at a base when it is a fighting unit with men ashore in the base's hex and none aboard; a transport force when it is on the transport mission, has a ship afloat that carries men and lies stopped there.
8. **The passage** of item 7 is the engine's forecast for the transport force to the place at cruising speed (`SideView::forecast_passage`), in hours; a force with no forecast, or one refused, gives no lift and is named.
9. **Ties and no count.** With the men wanted given by several bases, the shortest passage, then the lower number. With none giving them, the most men, then the lower number. With no estimate, the transport force with the most berths, and the base whose lift gives the most men, then the shortest passage, then the lower number.
10. **The risk on the way:** a force on the bombardment mission with a ship afloat stopped at any base of the side; one on the carrier mission with a ship afloat wherever it lies; one on the escort mission with a ship afloat stopped at the source, the run's escort rule asking that of a free escort. Each is put in the limit, and the rules of decisions 0126, 0146 and 0148 do the rest.
11. **No base can be the source** (item 13): the notice `cannot_meet` once, the reason each morning, with each base and what it lacks, and "no transport force the staff may take lies stopped at a base of the side" where none does anywhere.
12. **Chosen again** (item 4): on the first count, the source and the force of the task stand (the task is the force's), and the units are chosen again from those the end's limit binds and, where the commander named no unit, every fighting unit the staff may take ashore at the source.
13. **A second lift** (decision 0143) takes the units of the end's limit as it binds that were not chosen: under a limit the staff assembled, none, since the staff's limit names only those it chose.
14. **The words.** The reason of the choice carries, in order: the estimate and the men wanted; each base with its transport force, berths, men and hours, each base that could not with what it lacks, the source and why; what the staff may not take ("land_unit:1 (Placeholder First) serves intent:2, an intent higher in priority"); the units and berths of decision 0142; "short by N men" where the lift falls short; the forces to bombard, to cover and to escort, or "no force on the ... mission".
15. **The rules label stays 19,** as the record's costs say: no state changes. A game saved before in which a commander set a take-a-place forces limit with no source, refused then, would be taken in now; no recorded game holds one.

### Earlier tests, baselines and games that change

No expected number changed.

- **`crates/sim-core/src/command/tests.rs`, `taking_a_place_is_taken_in_and_its_landing_formed_while_preparing`:** it asserts the refusal "No source" of a take-a-place forces limit with no source, which item 1 withdraws. It is ignored with that reason, for the architect to restate; `taking_a_place_needs_no_source_and_the_staff_names_what_is_left_out` covers the intake of this record.
- **Fixtures:** the agent interface pins (22 to 23); the two tests that compare every baseline with a run with no staff leave out scenario 74.
- **Run baselines:** none changed; scenario 74's two are new. The agent goldens, pinned hashes and batch fixture are unchanged. **Recorded games:** each `save.json` differs only in its doctrine version and each transcript only in the answers' `version` (22 to 23) (`crates/sim-cli/tests/recorded_games/README.md`).

### Files to read

- `crates/sim-staff/src/assemble.rs` (`assemble` at line 227, `rechoice_limit` at 575), called from `crates/sim-staff/src/take.rs:344` in `prepare_place`.
- `crates/sim-core/src/command.rs` (`forces_on_chain` 1283, `assembled_forces` 1304, `forces_as_bound` 1320) and `crates/sim-core/src/command/intake.rs` (`forces_narrow_under_take` 235; the no-source check in `SetLimit`).
- `tests/scenario_74_the_force_assembled.rs`.

## The architect's rulings

Made on Nov 17, 2026, on reading the build.

1. **Readings 1 to 15 are accepted as built.**
2. **Both mismatches were the architect's errors and are restated;** the code stands. The passage from C to B is 21.2 hours, the route being charged its straight line off the grid's axes. The base the landing forms is base 4, base 3 being base C.
3. **The test of decision 0142 that asserted "No source"** is restated: a forces limit of "take a place" with no source is taken in.
4. **Known and left, on reading 13:** where the staff assembled the force, decision 0143's second lift finds no unit, since the staff's limit names only the units it chose. A side whose first lift falls short sends no more from the source of itself. Decision 0152's lifts as they stand are not touched by this, and the 39 days have every man aboard a force. A second lift chosen by the staff from what is still ashore is left for later.
5. **A fault found on reading the build, and mended.** The check that a staff's limit stays inside the limits above it read those limits as they bind. As it binds, the end's limit carries the staff's own earlier choice, so the staff could not name another source or add a unit when it chose again on a count, or added a later lift's units: its command would have been refused. The check now reads the limits above as they were set. `taking_a_place_needs_no_source_and_the_staff_names_what_is_left_out` has the staff name a second source after its first; it failed before the change.

