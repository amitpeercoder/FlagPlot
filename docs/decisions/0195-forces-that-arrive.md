# 0195: Forces that arrive

Status: a plan by the architect, written on Nov 20, 2026, on the designer's word (his queue of that day: "ships and land units that arrive by date"). Read by a reviewer who had not seen it written, and amended on that reading ("Amended on review", which replaces what is above it where they differ) before any of it is built.

Summary: Every ship and land unit of a scenario is on the map from the first day. One that should come later can only be marked "may be used from day N", and then lies where the scenario put it, to be seen and attacked, for the days before. This lets a task force, with everything aboard it, and a land unit or an air group at a base, come onto the map on a day the scenario names. Until that day it is not there.

Standing: Planned on Nov 20, 2026 and amended on review; the designer's answer is taken; built on Nov 20, 2026 ("As built", below), its readings and four statements of scenario 111 with the architect.

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

## Confirmed on Nov 20, 2026

The designer: "Go with your recommendations." **The 39 days is left as it is.** Its late forces go on lying on the map from day 1, marked as usable from their days, and its proofs' files stand. The rule is used from the next scenario on.

## As built

Built on Nov 20, 2026, on the branch `forces-that-arrive`, on decision 0194's build. No figure changed. Every scenario under `data/scenarios` (160 files, the frozen copies apart) gives, with `sim-cli run --events`, byte for byte the output it gave before: no event and no daily hash moved. No existing scenario has the key, and the 39 days is not changed (the designer's answer).

### What was built

- **The state** (`crates/sim-core/src/entering.rs`). `TruthState::entering`, the last field of the truth: a map from the unit that enters (a task force, an air group or a land unit) to an `Entering`: its day, side, place (`EntryPlace::Hex` for a force, `Base` for a unit or group), whether it waits, and, whole as the scenario wrote them, the force, its ships, the groups aboard and the land units whose men are all aboard; and for a force its superior, commander and radio silence. `TruthState::hold_apart` is the loader's way in: it moves a unit and all aboard out of the state. Nothing held is in the maps the day's rules walk, so no rule acts on it.
- **The entry step** (`World::bring_in`), the first of the day, inside `take_steps_one_to_three`, so a day and `at_staff_step` (the staff's view and a seat's check of orders) both take it, before the commands. Entries come in key order: task forces by number, then air groups, then land units. A force always enters at its hex. A land unit enters at its base where the base is its side's; an air group too, and where it has aircraft only while the base takes them. Else it waits, tried again each day.
- **Events**, at the end of `EventKind`: `ForceEntered` (force, side, hex, the side's base there if any, ships, groups, land units), `UnitEntered` (unit, side, base, hex, men, aircraft) and `EntryWaiting` (unit, side, base, the day named, why: `NotTheSidesBase` or `TakesNoAircraft`). No cause, no pulse. Its own side sees each whole; the enemy nothing (`side_events.rs`).
- **Orders.** `RejectReason::NotYetEntered { unit, day, waiting }`, at the end: "{unit} is not on the map yet: it enters on day {day}" (waiting: "it was to enter on day {day} and waits for its base"). Tried first in `validate_order` and for a staff's order, on the order's unit, the ships a base forms a force from and the land unit a force is to load, for the side's own held units. The enemy's held unit is not in the state and is refused as any unit not one's own (`NotYourUnit`; for a land unit's order `NotYourLandUnit`, as for one that does not exist).
- **Commands.** A forces limit, an intent naming a force, a "wait for" limit and a hold may name a unit held apart for the side (`command/intake.rs`, the `own_*` readers; `invariants.rs`, the command state's check of holds, commitments and runs).
- **Numbers.** A land unit made by division takes `TruthState::next_land_unit_id`, above the state's and what is held apart together. Nothing held apart is a base, so a base formed in play is numbered as before.
- **Arrivals.** An arrival to a held group waits, with no event, and is delivered on the first night after the group enters.
- **Command levels.** At the start the superior, commander and radio silence named for a held force are kept in its entry; a held force is not a command at the start, so it cannot be named a superior or a contact's `seen_by`. On entry it becomes a command as one formed in play does, under its superior where that is still a command, else the area command, with its commander.
- **The hash.** Hashed last, after the domain, and only where something is held (`hash_entry`), every field named in `name_every_field`. `flagplot-state-v51` stands; no golden file was remade.
- **The view** (`view.rs`). `SideView::entering()` and `entry_of(unit)`, the side's own only. "Is it mine" is true of an own held unit; `available_from` gives the later of its entry day and its own days; `may_use` is false for any held unit; `arrivals()` lists those to own held groups. A held unit is in none of the view's lists of the map, and every question about the map (a march, a landing, a force's ammunition, a strike) answers nothing of it.
- **Pictures.** A picture's truth carries the entries; a side's half its own (`PictureHalf::entering`); a world made from a picture or from halves refuses a number both in the state and held apart.
- **The loader** (`crates/sim-data/src/scenario.rs`). The key `enters` on `[[task_force]]`, `[[land_unit]]` and `[[air_group]]` (and on `[[ship]]` only to refuse it). Refused, each with its reason: below 2; after the last day; on a ship; on a group aboard a ship; on a unit aboard a force; a force off the map or on a hex that is not sea or coast; a land unit in a hex with no base; an arrival dated before its group enters (its own day or its ship's force's); an escort order whose force and charge do not enter on one day.
- **The staff** (`crates/sim-staff`, doctrine `supply-48`). `counted_by_a_take` counts the side's held forces and land units; "more is coming" counts a held fighting unit entering at the source, or a held lift with fighting men aboard, from its day, and names it with its day; the carrier, stand, bombard, landing and take doctrines give a held force no order and hand in one reason, "{force} enters on day {day}", leaving the intent neither failed nor finished; a "wait for" naming a held unit waits.
- **The agent** (interface version 36). The briefing's `arrivals.to_enter`, `entered_yesterday` and `entry_waiting_yesterday`; `list_forces.to_enter`; `get_entity` of a held unit (`to_enter`); handles for held units; the refusal's words. Each left out when empty.
- **The umpire's readers.** The batch record finds an entered ship's force and counts a group as formed in play only where it was neither in the state nor held apart at the start; the batch's ship table holds the ships held apart, and one that never entered is in no count. The replay's opening state lists what is held apart (`entering`, left out when empty); the viewer draws a force from its `ForceEntered` and `viewer/check.js` checks it. `sim-cli run` prints a line for each unit held apart after the header.
- **The trial.** A trial's worlds are built from the side's half, which carries its entries, and bring them in on their day; its dates read `counted_by_a_take`. The enemy's made half holds nothing apart: what the side does not know is coming is not made up. A ready landing's men count those of a held unit (`sim-trial/src/play.rs`).
- **Scenario 111** (`data/scenarios/forces-that-arrive/`, with the placeholder map `data/map/placeholder_forces_that_arrive.toml`) and its tests: 29 pass and 3 are left ignored, below.
- **Versions.** The rules label stays 25: no existing game plays differently. The state's layout stays v51. The saved game's version stays 23: its format did not change. The doctrine version is `supply-48` and the agent interface 36; the recorded games were remade for these two labels alone.

### Readings taken

1. **Where a force may enter** is a hex of the map that is sea or coast (`is_navigable`); a base of its side on land is not one, since no force may lie on land.
2. **A waiting unit is tried at the first step of each day,** not in a night pulse (rule 12 put the entry there); a base that changes side in a day's fight ashore is the unit's from the next day's first step (`docs/rules/differences.md`, row 56).
3. **The day a held unit may be used,** as the staff and the view read it, is the later of its own days and the day it is next tried: its day, or for one that waits the day after the last resolved.
4. **`ForceEntered`'s base** is the lowest numbered base of the force's side in its hex. A land unit enters with its men as written, in its base's hex.
5. **An escort and its charge** are checked in the scripted escort orders, since only an order gives a charge; a force with no `enters` is there from day 1.
6. **A held air group at a base with no airfield** is refused when the scenario is read, by the check that refuses any group with aircraft there (decision 6 of 0099), though rule 16 would have it wait. A base whose field takes no aircraft for damage makes it wait.
7. **The agent's numbers** for a side's units count its held units from day 1, so no number changes when one enters.

### The statements of scenario 111 that do not hold as written

- **9a, a route given is kept.** A scenario gives a task force no route but by an `[[order]]`, and an order before its day is refused (statement 5). The test is ignored; a substitute shows a route ordered on day 3 is taken and sets out in day 3's night. Rule 17's "a route and a speed the scenario gives it are kept" has no key to rest on.
- **11a, on days 1 and 2.** The staff judges "more is coming" only on a morning with men ashore at the place, which no take has on days 1 and 2. The test is ignored; a substitute on scenario 109's setting shows that on the mornings ashore before the held unit's day the staff says more is coming and names it and its day.
- **14, a game of two days.** Statement 13 refuses `enters` after the game's last day, so a game of two days cannot hold K's `enters = 3`. The test is ignored; a substitute stops the six days after day 2 and finds K's ships in no count.
- **11b, a hold** was refused by the check of the command state's invariants until that check read the entries; it holds as built.

### Not done

Nothing leaves the map (rule 22). The 39 days is unchanged. The viewer does not show a unit still held at the end of a game.

## The architect's rulings on the build

Read on Nov 20, 2026, with the build's report and its "As built".

1. **The readings of "As built" stand,** and row 56 of `docs/rules/differences.md` is confirmed: a waiting entry is tried at the first step of each day, where its first entry is. Among the others: a force enters only at a sea or coast hex; an air group with aircraft at a base that has no airfield is refused when the scenario is read, as it is today, and waits only where the field is damaged; a side's held units are numbered for its seat from day 1, so that no handle changes when one enters.
2. **Three statements of the worked example were the architect's mistakes** and are corrected by him: a scenario gives a force a route only by an order, so "a route given is kept" becomes a route ordered on its day; the staff says what is coming only on a morning of the judgement ashore, so that statement is set in scenario 109's setting; and a game of two days cannot hold a force that enters on day 3, so the count is read from the game stopped after day 2. Each has the test the build wrote in its place.
3. **No existing game moved:** 160 scenario files give the same events and the same daily hashes before and after; no golden file was remade; the state's layout and the rules label are as they were. The full run passes on the final commit.
4. **A fault the build found and mended:** the check of holds, commitments and runs did not read what is held apart, so a hold naming a held force failed the game's own invariants.
5. **Left as it is, and known:** nothing leaves the map; the viewer and the end of `run` show nothing of a unit still held apart when a game ends; the staff's stand, bombardment and landing doctrines' words for a held force have no test of their own.
