# 0195: Forces that arrive

Status: a plan by the architect, written on Nov 20, 2026, on the designer's word (his queue of that day: "ships and land units that arrive by date"). To be read by a reviewer who has not seen it written, and amended on that reading, before any of it is built.

Summary: Every ship and land unit of a scenario is on the map from the first day. One that should come later can only be marked "may be used from day N", and then lies where the scenario put it, to be seen and attacked, for the days before. This lets a task force, with everything aboard it, and a land unit or an air group at a base, come onto the map on a day the scenario names. Until that day it is not there.

Standing: Planned on Nov 20, 2026; waits for its review.

## Why

A campaign longer than the 39 days is its reinforcements (`docs/design/larger-scenarios.md`). And in the 39 days as they stand, the Japanese carrier forces sit in their hexes from day 1 to day 17 marked "may be used from day 17": on the map, visible to a search, open to attack, for sixteen days before they came.

What is so today:

- **Aircraft arrive by date** (`[[arrival]]`, `[[replacement]]`): numbers added to a group that exists, at night.
- **A unit before its day** (`available_from`, decision 0140): its side may not order it before day N. "A unit before its day is still there ... It can be seen and attacked and it defends itself. Nothing brings a force onto the map later."
- **No ship is ever made in play.** Task forces and air groups made in play take numbers from a series of their own; land units and bases made in play take the next number above the highest.

## The rules

1. **A new key, `enters = N`,** a whole number from 2 to the game's last day, on a `[[task_force]]`, or on a `[[land_unit]]` or an `[[air_group]]` that the scenario places at a base of its side. A task force that enters brings its ships, the air groups aboard them and the land units and cargo aboard them: all of it enters with the force, and none of those carries the key itself.
2. **Before its day it is not on the map.** It is in no hex. It is not searched for, sighted, struck, engaged, fed, tired, worn or fuelled, and it burns nothing. No rule of a day acts on it. It is held apart, whole, as the scenario wrote it.
3. **It enters at the start of its day,** in the night pulse, before anything of that pulse is resolved: the force in the hex the scenario gives it, stopped, at the settings the scenario gives it; a land unit or an air group at its base. From then on it is a unit like any other.
4. **Where a force may enter:** a sea or coast hex, or a base of its side. The scenario is refused where the hex is land with no base of the side, or off the map. Where the base a land unit or an air group enters at is not its side's on the day, it waits and is tried again each night, as an arrival of aircraft waits (decision 0101), and its side is told. A task force enters whoever is in the hex.
5. **Its numbers are the scenario's.** They are taken from the first day: nothing made in play is given one of them.
6. **An event,** `ForceEntered` (and `UnitEntered` for a land unit or an air group at a base), with the day, the hex or base and what came; and `EntryWaiting` where it waits. Each is its own side's alone, whole. The enemy learns of it as he learns of anything: by sighting it.
7. **What its side knows before the day.** Everything, exactly: it is the side's own. A side's view lists what is to enter, with the day, the place and what it is. The briefing has it under what is due, as aircraft are.
8. **Orders and intents before the day.** An order to a unit that has not entered is refused, saying it enters on day N. An intent's forces limit may name it; the staff reads it as a unit that may be used from its day (decision 0140), and so counts it as coming (decision 0193) and leaves it out of every pool until then.
9. **With `available_from`.** A unit may have both: it enters on its day and may be used from the other, the later of the two binding its orders. `available_from` alone is unchanged.
10. **Objectives.** A ship that never entered was never at risk and is in no count. A game that ends before a force's day ends without it.
11. **A scenario with no `enters` plays as before.**

## Rule card

- **Reads:** the units held apart and their days; the day; the holder of a base.
- **Writes:** the truth state: on its day a force, its ships, its groups and its units are put on the map. The list of what is held apart.
- **Reuses:** an arrival that waits for its base; the day from which a unit may be used, for the staff; the briefing's lines of what is due.
- **New:** one key; a list in the state of what is held apart; one step at the start of the night pulse; three events. No draw.

## What it costs

- **State and hash:** the list held apart is state, walked by the hash and named field by field; the hash version rises and the golden files are remade.
- **Every reader of "all the side's forces"** must know that a force held apart is not among them: the engine's loops by construction, since it is not in the state they walk; the side's view, the briefing, the agent's lists, the table and the replay viewer by a list of their own.
- **The 39 days is not changed by this plan.** Its carrier forces could enter on day 17 in place of sitting from day 1; that changes a scenario of record and its proofs' files, and is put to the designer.
- **Rulebook:** chapters 1 and 2 (the day's order; what a side knows), chapter 7, chapter 8 where it speaks of a unit before its day, the glossary; the scenario file's keys; the agent interface and its version.

## Questions for the designer

1. **The 39 days.** Change its late forces to enter on their days (it then no longer plays as its proofs did, and its staff file and the stage 3 proof's files are remade and their runs made again), or leave the 39 days as it is and use the rule from the next scenario on. The architect proposes to leave it.
