# 0195: Forces that arrive

Status: a plan by the architect, written on Nov 20, 2026, on the designer's word (his queue of that day: "ships and land units that arrive by date"). Read by a reviewer who had not seen it written, and amended on that reading ("Amended on review", which replaces what is above it where they differ) before any of it is built.

Summary: Every ship and land unit of a scenario is on the map from the first day. One that should come later can only be marked "may be used from day N", and then lies where the scenario put it, to be seen and attacked, for the days before. This lets a task force, with everything aboard it, and a land unit or an air group at a base, come onto the map on a day the scenario names. Until that day it is not there.

Standing: Planned on Nov 20, 2026 and amended on review; not yet built.

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

## Amended on review

A reviewer read the plan against the order of a day, the staff's readers and the code. Found right: that holding a force apart from the state the day's rules walk keeps every one of those rules right by construction; that an unknown number is refused today without a leak; that numbers made in play for forces and groups cannot collide with a scenario's; that refusing `enters = 1` is right; that objectives walk only what is in the state.

### When it enters, and who must be told it is coming

12. **It enters first thing in the day, before the pulses:** before the day's commands are taken in, before the staff is asked and before orders are checked. So on its day the staff sees it and an order to it is taken. Rule 3's "in the night pulse" is replaced. Chapter 1's "Before the pulses" gains the step.
13. **A side's view gives the side what is held apart for it,** each with its day and place, and every reader that today walks "all the side's forces" to answer a question about the future takes that list too. Named for the build: the staff's test of what a take may count on and of what is coming (decision 0193); the view's own answers to "from which day may this be used", "may I use it" and "is it mine"; the trial's dates. Without this a staff would conclude that nothing more is coming while a division waits to enter.
14. **Commands may name it.** A forces limit, an intent given to a force (operate a carrier force, stand at, ready a landing) and a hold may name a unit held apart for the side. Today each would be refused as not the side's.
15. **Only its own side is told it is coming.** An order or a command naming the other side's held unit is answered as for any unit that is not one's own. "It enters on day N" is said to its own side alone.

### Where and what

16. **A land unit or an air group enters at a base the scenario names,** which need not be its side's on day 1 (rule 1 said "of its side"; rule 4 stands). It waits while the base is not its side's. An air group with aircraft waits too while the base takes no aircraft, as an arrival of aircraft does.
17. **A task force enters at a hex a force may be in:** sea or coast, or a base of its side whose hex is so. A route and a speed the scenario gives it are kept: it sets out on them when it enters.
18. **Numbers.** A unit made by division and a base formed in play take the next number above the highest of the state and of what is held apart together. That gives every existing scenario the numbers it has today.
19. **Other things a scenario may say of a held unit:**
    - an arrival of aircraft dated before its group enters is refused when the scenario is read;
    - a commander named for a held force, and its radio silence, enter with it;
    - with command levels on, the force's command is formed when it enters, under the command the scenario placed it in;
    - an escort whose charge is held apart, or a held escort whose charge is not, is refused unless both enter on one day;
    - a formation may name a held unit, which is of it from its day;
    - `enters` on a land unit placed in a hex with no base is refused.
20. **Pictures and the trial.** A picture of the world and each side's half carry what is held apart for that side, so that a trial's worlds bring the side's reinforcements in on their days. A world made from two halves refuses a number that is both in the state and held apart.
21. **The umpire's readers.** The batch record counts a group as formed in play only where it was neither in the state at the start nor held apart, and finds an entered ship's force. The replay file's opening state lists what is held apart, and the viewer shows a side's entered ships and places them from their entry.

### What it does not do

22. **Nothing leaves the map.** A force that has entered stays. A schedule of supply convoys would pile up at its base. Leaving the map is a rule for a later record; a force that goes to the yard (decision 0069) is the one way off today. Decision 0196's claim that this plan makes cargo in real ships possible is true only of their coming.

### The hash and the events

23. **What is held apart is hashed last, and only where there is something held,** as decision 0170, item 12, hashed a field: each entry's day, place and waiting, and its force, ships, groups and units through the walkers the state uses. No existing scenario's daily hash moves, and no golden file is remade for this rule.
24. **The three events cite no cause,** as an arrival of aircraft cites none. Who may see each is decided in the table of who sees what: its own side, whole.

### The worked example and the order of building

25. **Restated by the architect:** the force enters before the day's orders; the sighting is the morning's search at a chance of 1; two forces in one hex fight in the night's movement; a unit that has not entered is named in no event but its entry, its waiting and the refusal of an order to it; refusals are added for rule 19.
26. **Built second,** after decision 0194, which gives the view the game's days that this rule's checks read.
