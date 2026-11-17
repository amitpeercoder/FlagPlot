# 0140: A day from which a force may be used, formations, and the staff's setup of the 39 days: step 1 of stage 3

Status: settled by the architect on Nov 17, 2026, inside the plan for step 3, stage 3, that the designer confirmed the same day (decision 0139, step 1 of its eight, and items 7 to 11). It changes one sentence of `docs/step3-spec.md`, "Two setups of the campaign", as the spec says it may be changed. Nothing is built.

Summary: Step 1 of stage 3: a task force, a ship, an air group and a land unit may have a day from which its side may use it, before which no order to it is taken and the staff leaves it out; a side knows its own forces' days and not the enemy's; a land unit may belong to a formation, and formations have parents, as data; and the 39-day slice is set up a second time with no scripted order, each force with its day from the row the script's day rests on

Standing: Settled by the architect, Nov 17, 2026, inside decision 0139. Not built

## Why

The 39 days are scripted: each force is sent out on the day the record gives. Every force is in the scenario from day 1, because nothing brings a force in later. Take the scripts away and a staff has, on 7 August, ships and men that were not there to use for weeks. A staff that plans an operation must also know what it will have and when, which is the reinforcement schedule in its first form.

## The day

1. **A task force, a ship, an air group and a land unit may each have a day from which its side may use it,** given by the scenario. With none, it may be used from day 1, as now.
2. **Before its day no order to it is taken,** from the staff or from the commander. It is refused with the day. A ship takes the later of its own day and its task force's; an air group aboard a ship and a land unit aboard a ship take the later of their own and the ship's.
3. **Before its day the staff leaves it out:** it is in no pool, it is no force a task may use, and no doctrine gives it an order. A reason that would have used it says the day it may be used from.
4. **It is still there.** It lies where the scenario put it, at the settings the scenario gave it, and the rules act on it as on any unit: it can be seen, attacked, and it defends itself. The game has no way to bring a force onto the map later, and this step adds none.
5. **A side knows its own forces' days.** The side's view gives them, and the tools show them: "available from day 9". Nothing gives a side the enemy's days.
6. **After its day a unit is the side's to use as it sees fit.** No date of any operation is fixed by a day.

## What it changes in the spec

`docs/step3-spec.md`, "Two setups of the campaign", says of the day: "It is a dated entry in the forces limit." It becomes a property of the force, set by the scenario. A forces limit belongs to one intent, and stage 3 has the staff draw its own intents from "control an island"; a day that lived in a hand-written limit would be lost the moment the staff wrote the limit. The row "A day from which a force may be used" of "What it builds on" is this record.

## Formations

7. **A land unit may belong to a formation,** and a formation may have a parent formation: a battalion under a regiment under a division. A scenario states them. The loader refuses a parent that is not there and a formation that is its own ancestor.
8. **No rule reads a formation in this step.** The side's view gives the tree, so that step 3 of this stage can choose and move a formation whole. A formation is not a command (decision 0139, 10).
9. **In the staff's setup each formation and each parent comes from a row** of `data/oob` (`land_units.csv`, the parent rows of `land_unit_facts.csv`). Where no row gives a unit a parent it has none: a gap, not a guess.

## The staff's setup of the 39 days

10. **A second file,** made from `data/scenarios/07_full_slice.toml`: the same map, bases, forces, groups, units, arrivals and figures, with every scripted order taken out and no intent stated. It is kept out of the loops that walk `data/scenarios/*.toml`.
11. **Each unit's day is the day of its first scripted order** in the scripted setup, with the row that order's day rests on cited beside it. A unit aboard a force takes the force's day. A unit the script never orders has no day, and may be used from day 1.
12. **A day with no row behind it is declared made up,** and marked a placeholder: the placeholder convoys and night forces of the slice are such.
13. **The architect rules on the list before the step is merged.** The build makes the table of every unit, its day and its row, and the table of the formations and their rows; where the script's first order is not the day a force was there to use (the Rabaul air groups, which the script first orders on day 15 and which flew on 7 August), the table says so and the architect settles it, with the reason on the row.
14. **With no intent stated it plays as the scripted file with its orders taken out:** the same events. A day changes nothing until someone would give an order.

## Rule card

- **Reads:** a unit's day; the day of the game.
- **Writes:** nothing but the days and the formations, from the scenario.
- **Reuses:** the engine's intake and its refusals; every doctrine's test of what a task may use.

## What it costs

- **State:** a day on a task force, a ship, an air group and a land unit, the formations and each unit's formation, in the hash and named. The state layout and saved game versions rise. The rules label does not: no game without a day plays differently. The agent interface version rises: a seat's lists show a unit's day.
- **Every doctrine's "may use" test** reads the day. "As built" lists each.

## The hand calculation

`tests/scenario_65_a_day_from_which_a_force_may_be_used.rs`.
