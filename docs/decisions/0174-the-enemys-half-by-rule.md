# 0174: The enemy's half, by rule: the third part of step 4 of stage 4

Status: settled by the architect on Nov 19, 2026, inside the plan for step 3, stage 4 (decision 0161, step 4 of its ten, section C, rules 6 to 9), which the designer confirmed. The third of the three records of step 4. It adds the crate `sim-trial` and the rules by which it builds the enemy's half of a picture from what a side holds and believes. The hand calculation is written. To be built when decisions 0171 and 0173 are built.

Summary: A side's view gives its own half of a picture. The other half is what it does not know, and a trial must still have an enemy to play against. This record says how one is made, by rules that are written down and named in the estimate where they mattered: an enemy base as the side's map has it; the men the side estimates ashore; a force for each contact, of the kind and count reported; an old report placed where the force could have steamed since; two reports that may be one force, one in some worlds and two in others; and the forces the side believes exist and has not seen, by their standing. It reads the side's view and what the scenario says the side believed, and nothing else

Standing: Settled by the architect, Nov 19, 2026, inside decision 0161. The hand calculation is written; the build waits for decisions 0171 and 0173

## Why

A contact is a place, a count and a kind. It has no fuel, no damage, no aircraft and no orders. A world needs all of them. Decision 0100's experiment built an enemy by rules of thumb and recorded what each cost: nine carriers from "9 ships, kind carrier"; an enemy who sat still for want of orders; two reports of one force built as two forces, or dropped as one, by a rule that is nearly always true after a week; a carrier never seen, and so a world with no carrier, "valid, and it proves nothing".

Decision 0161 gave the shape (rule 7). This record gives the rules, each with its figure.

What the rules may read is fixed (rule 6): a side's view; the scenario's statement of what that side believed (decision 0173); and a trial's settings. Never the world, the umpire's view or the full log.

## The rules

### The crate

1. **`sim-trial`** depends on `sim-core` and `sim-staff` and on nothing else. In this record it makes pictures and builds worlds from them; the playing of a trial is the next step's. It has a test that reads its own source, as the staff and the agent have: it may name a world only to build one from a picture and to resolve it, and may not name the umpire, a whole picture, a rebuild on a live seed, or a view taken from a world it was not given.
2. **One call makes the pictures of a trial:** from a side's view, that side's believed forces and intents, the trial's figures, a seed, a trial's number and a count of worlds. It answers one picture for each world, numbered from 1, and with each the list of what was assumed in making it. A second call builds the world of a picture, from the reference data and the map it is handed, on the stream of its trial and its world (decision 0170, 12).
3. **Every draw is the trial's own,** keyed on the seed, the trial, the world, the picture's day, and the thing drawn for by its place in the side's own lists. The same view, beliefs, seed, trial and world give the same picture, byte for byte.

### The side's own half

4. **It is the side's own half** as its view gives it (decision 0171), unchanged.

### The enemy's bases and men

5. **A base for each base of the enemy's on the side's map,** with the number, name, hex and side the map gives it. A side knows nothing of its stocks, guns or works (decision 0160's belief of an enemy base's works is not built): it is given `enemy_base_supply_t` of supply and `enemy_base_fuel_t` of fuel, no guns, and an airfield with nothing at it. Both figures are placeholders of the trial's figures file.
6. **A land unit for each estimate of men ashore:** one fighting unit of the estimate's men, in its hex, on defend, of the standard experience, fed. An estimate's age and grade change nothing here.

### A contact becomes a force

7. **Its ships.** A contact of `n` ships taken for a kind is built as one ship of that kind and `n - 1` destroyers; where the kind is transport or auxiliary, as `n` ships of that kind. The count is the contact's, never fewer.
8. **Their classes.** A ship is of the lowest-numbered class of its kind that the file of sides gives to the enemy, alone or with the side (decision 0173). Where the enemy has no class of a kind, a ship of that kind is not built, and the assumptions say which ships were left out.
9. **Their state.** Each ship is undamaged, with full fuel, full rounds and torpedoes, no cargo, and the standard experience. A carrier has two air groups: fighters, a third of its aircraft capacity, rounded down, and bombers, the rest; each of the lowest-numbered type of its role that the file of sides gives to the enemy; all ready, at rest.
10. **The force.** One task force for a contact, at the place of rule 13, stopped, at cruise, at normal readiness, with no route, no home port and no order. Its mission is by the contact's kind: carrier for a carrier; surface combat for a battleship, cruiser or destroyer; transport for a transport or auxiliary.
11. **A claim raises a kind and never lowers a count.** Where the side holds a claim of its own, made from the air against a kind, within the search figures' merging distance of a contact's place, and that kind is more important than the contact's, the contact is built as that kind. A claim of ships sunk takes no ship from any contact: a claim is never below the truth and may be well above it.
12. **Two reports that may be one force.** Two contacts may be one where the older could have steamed to the newer: the hexes between them are no more than the merging distance and the steps of rule 13 for the pulses between their ages. Pairs are taken in the order of the side's own numbers, each contact in one pair at most. For each pair one draw in each world, at an even chance: one force, with the newer's place and age, the more important kind and the larger count; or two. The assumptions of a world say which. No number supports the chance; the estimate shows the worlds apart (decision 0161, 13).

### Where a force is

13. **A force is where it could have steamed.** A contact of age 0 is at its reported hex. Older, it is at a navigable hex drawn evenly from those within its reach by sea from the report: the hours of its age, counted back from the picture's evening by the pulses' own lengths (afternoon 6, morning 6, night 12), times the cruising speed of the slowest class built for it, in miles; and miles to steps at 40 a step, rounded down. The hexes are taken in their own order and one is drawn for each world.

### The forces a side believes in and has not seen

14. **By its standing** (decision 0173, 6). Believed present: built in every world. Believed absent: built in none. Unknown: built in half the worlds exactly and not in the other half, with no draw: the first unknown force of the side's list is in the even-numbered worlds, the second in the worlds whose number less one has its second binary digit set, and so on, so that over a count of worlds that is a power of two every combination of the unknown forces is tried equally often.
15. **A contact may stand for it.** A believed force is not built in a world where the side holds a contact of its kind (after rule 11) that it could be: any contact of the kind where the force's place is not known; else one within the reach of rule 13 from where it was last known, for the days since. The assumptions say that the contact was taken for it.
16. **Built as a contact is,** from its kind and its count (rules 7 to 10).
17. **Placed** by rule 13 from where it was last known, with the hours since that day's evening (a day of 0, before the first day, counts from the evening before day 1). Where its place is not known: at a base of the enemy's on the side's map, one drawn evenly for each world from those bases in the order of their numbers. Where the map has none, it is not built, and the assumptions say so.

### What the enemy holds and wants

18. **The enemy is given no belief.** His half holds no contact, no claim and no estimate. He finds what he finds in the days played. It is an assumption, and the list says so.
19. **What the enemy wants is what the side believes he wants** (decision 0161, 8). Each believed intent is handed in for the enemy on the first morning of a world, as a commander's command, with his leave to prepare and to launch given, at the risk a commander gets by default. With none stated he holds what he holds: he is given no intent, and his staff gives no order.
20. **Numbers.** The ships, forces, groups and men made by these rules are numbered from 900,001 in the order made, each kind by itself; the enemy's counters stand at their first numbers. A side whose own units reach those numbers is refused a trial.

### What was assumed

21. **Every picture carries its assumptions:** a list, in the order the rules were worked, of each thing made or left out and the rule and figure it was made by. It is what the estimate will name (decision 0161, 7 and 13). It is data beside the picture and no part of it.

### No peek

22. **Two live games that differ only in what the side cannot see and has not been told give the same pictures,** byte for byte, for the same seed, trial and worlds. The constructor is tested on the hard cases of the spec, each of which gives a world that is built and resolves: no contact; an old contact; two reports of one force; a claim of a sinking that is too high; a contact of the wrong kind; an enemy carrier never seen.

## The figures

A file of the trial's figures, `data/reference/trial.toml`, named by a scenario that is to be tried: `enemy_base_supply_t = 1000` and `enemy_base_fuel_t = 500`. Both are placeholders set by the architect, resting on no source: no row sizes what a side should assume of a base it has not seen. The merging distance is the search figures' own. The speed of a force is its class's.

## What it leaves for later

- **Playing a trial,** its sizes and its time: step 5. **The estimate:** step 6.
- **An enemy's land air groups.** A side knows of them only by the raids it has suffered (decision 0167 tells it the type where recognised). None is built at an enemy base here; a side that is raided from Rabaul every day and tries a landing with no enemy bomber in the picture will be told so by the assumptions, and it is the first thing to mend after the trial is proved.
- **A kind that is wrong.** A contact's kind is taken as reported but for rule 11. The search figures hold the chance that a kind is mistaken; a trial that drew the kind from them would show it. Left until the estimate can show such worlds apart.
- **An enemy's composition by his own practice,** in place of one ship and destroyers: a file of typical forces by side and kind, from the order of battle.
- **What the enemy knows of us.**

## Rule card

- **Reads:** a side's view; that side's believed forces and intents; whose a class is; the search figures' merging distance; the trial's figures; a seed, a trial's number and a world's.
- **Writes:** nothing in any game. It returns pictures and their assumptions, and builds worlds of its own.
- **Reuses:** the picture and its constructor (decision 0170); a side's own half and the joining of two (decision 0171); the believed forces and intents and the file of sides (decision 0173); the map's distances by sea; a commander's commands.
- **New:** the crate; its figures file; the draws of a trial, in its own domain.

## What it costs

- **A crate,** with its row in `docs/engineering/layout.md` and its source test.
- **No game changes:** no state, no draw of a live game, no hash.
- **The rulebook** gains a chapter for the trial, begun here with how a picture's enemy is made; the glossary gains its words.

## The hand calculation

It is in `tests/scenario_93_the_enemys_half.rs`, written before the code. Its cases are on the placeholder classes and a small map with no land, so that every reach can be counted: a carrier contact of 5 ships; a claim that raises a cruiser to a carrier; a claim of two sunk that takes nothing away; a contact a day old, within 9 steps; a force last known two days ago, within 19 steps, 768 nm; two reports that may be one; a believed force of each standing over 8 worlds; an enemy who is given a hold and one who is given nothing; and the pair of worlds that differ only in what the side cannot see.
