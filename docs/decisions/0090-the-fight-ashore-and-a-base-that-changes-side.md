# 0090: The fight ashore, and a base that changes side

Status: a plan by the architect, written on Nov 13, 2026. It is stage 2 of the ground combat plan the designer approved (`docs/design/ground-combat.md`). The designer agreed decisions 9 and 16 on Nov 13, 2026, and on decision 20 asked for rebasing by order to be built first (decision 0091), so that a player can fly a group out; with that, decisions 20 and 21 stand. Built on Nov 14, 2026: see "As built" at the end. Every figure is a placeholder set by the architect. Code waits for stage 1 (decision 0066) and rebasing (decision 0091) to merge; the hand calculation is written before any code is delegated.

## The gap

After stage 1 men are carried, landed and fed, and they do nothing else. Units of both sides in one hex ignore each other. No base changes hands, so no convoy sails to take a place and none to hold one, and no objective can be to take or hold ground (decision 0054, known limit).

This record adds the posture of a unit, one fight a day in a hex, fatigue for a land unit, a side beaten in a hex, a base that changes side, and two kinds of objective. Shells and bombs on men, and what a side knows of the enemy ashore, are stage 3.

## Decisions

### Posture

1. **A land unit has a posture: defend or attack.** It starts on defend. A scenario may start a unit on attack.
2. **A posture order sets it.** `set_posture` names a land unit and a posture. It stands until replaced. It is refused at intake if the unit is not the side's own, if the unit is destroyed, or if the unit has no man ashore.
3. **An attack order needs no enemy.** A unit on attack in a hex with nothing to attack does nothing and is not tired by it.

### The day's fight

4. **A hex is contested on a day** when both sides have men ashore in it and at least one unit there is on attack. Each contested hex has one fight, in the ground combat phase, after the third pulse. Hexes are taken in hex order.
5. **A side attacks in the fight if any of its units in the hex is on attack.** A side that does not attack defends. All of a side's units in the hex fight as one body: the units on defend of a side that attacks fire with the rest.
6. **A unit's firing men** are its men ashore, times its fed share (decision 8), times 100 less its fatigue over 100, cut to a whole man. Men aboard ships do not fight and are not hit.
7. **Each unit fires.** Its shots are its firing men over `men_per_shot`, with the part of a shot rolled at that part of the chance, by the rule surface combat uses. Each shot hits at `hit_chance`, or at `defender_hit_chance` if its side defends. Each shot is rolled, on a stream keyed on the day, no pulse, the `GroundCombat` subsystem, a new draw `GroundFire`, and the firing unit. A breakdown records the firing men and the chance.
8. **A unit's fed share** is the men of its side fed in its hex at the last night's charge over the men charged. It is 1 for a unit none of whose men were ashore at that charge, and 1 until the first charge. It is state of the unit, set by the men's supply phase of stage 1. So a side that was half fed fires with half its men.
9. **Both sides fire on the strengths they had at the start,** so the order of firing does not matter. Each hit removes `men_per_hit` men of the other side in the hex, as many as are there, taken from its units in proportion by stage 1's rule (decision 0066, 20). *Agreed by the designer, Nov 13, 2026. A hit is a fixed number of men whoever fires and whoever is hit. The other choice was a loss that grows with the size of the body hit.*
10. **A unit with no men anywhere is destroyed,** as in stage 1.

### Fatigue

11. **A land unit has fatigue, from 0 to 100,** on the scale air groups and task forces use. It starts at 0 unless the scenario says.
12. **A unit on attack that fired in a fight gains `fatigue_per_attack_day`,** up to 100. Every other unit ashore loses `rest_per_day`, down to 0, in the ground combat phase, whether it fought or not. A unit with no man ashore keeps its fatigue.
13. **Fatigue cuts the men who fire** (decision 6), as it cuts the aircraft of a tired air group that fly. It cuts nothing else.
14. **A tired unit stops attacking.** After the day's gain, a unit on attack whose fatigue is above `attack_fatigue_limit` goes to defend, and its attack order is spent. It is recorded, caused by the fight. *One figure serves every unit; there is no order for the limit. An order can be added if play shows a need.*

### A side beaten

15. **After the day's fight a side is beaten in a hex** if the other side attacked there that day, and it has no man ashore in the hex or fewer than `beaten_share` times the other side's men ashore. Men are counted after the day's losses. If both sides attacked, each is tested, and both cannot be beaten at once while `beaten_share` is below 1.
16. **A base with no men falls without a fight.** In a hex that is not contested, where a side has a unit on attack and the other side has a base and no man ashore, the other side is beaten in the ground combat phase of that day. *Agreed by the designer, Nov 13, 2026: without this an undefended base could never be taken. With it, landing one man on attack takes an empty airfield.*
17. **A beaten side loses every base it holds in the hex.** Its men who are left stay ashore, unfed while it has no base there. They can be taken off by sea, and they can attack on a later day.

### A base that changes side

18. **The base keeps its number, name, hex, guns and repair figures. Its side changes,** and it is on both sides' maps from then on, whatever it was before (decision 0066, 12). One event records it, caused by the fight or, under decision 16, by the posture order. Both sides see it.
19. **Its stocks.** `captured_stock_share` of its supply and of its fuel stays, and the rest is lost. The event carries both.
20. **Air groups at it are lost,** with every aircraft they have, ready and damaged, and their pilots where pilots are counted. The ground combat phase is after the last pulse, so none is in the air. A new event records each, as a group lost with its ship is recorded, and counts in the `aircraft_destroyed` tally the same way. The group has no place from then on: it flies nothing, receives no arrival, and an order to it is refused as for a group lost with its ship. *The designer's answer, Nov 13, 2026: build rebasing first. A group can be flown out by a rebase order on any day before the base falls (decision 0091); one still there is lost whole.*
21. **A ship in port at it is lost too,** recorded as sunk and scored as sunk. This is a ship in no task force (decision 0025). A task force in the hex is not touched: it is at sea.
22. **What pointed at the base is put right, each with an event** seen by its own side:
    - a force of the beaten side whose home port it was has no home port;
    - an air group of the winning side whose mission was a strike on it has no mission.
23. **What needs nothing.** A replacement stream for the beaten side stops (it is the rule already, chapter 5). A standing cargo order of the beaten side works no cargo there, as at any hex with no base of its side. A force of the beaten side away at the yard from that port returns to its hex and finds no base. From the next day the base refuels, repairs and works cargo for its new side, and aircraft of the new side that cannot get home may land there.
24. **Events of a base are seen by the side that held it when they were recorded.** Today the holder is read from the present state, which would hand a base's whole past to its taker and renumber what both sides had been shown (decision 0086). The holder at the time is worked out from the log.

### Feeding where a side has more than one base

24a. **Men are fed from every base of their side in the hex, in base order.** Each base with stock pays what it can of what is still owed, and the men fed are counted from all that was paid. Stage 1 asked only the lowest-numbered base (decision 0066, 16, and what it left out). With bases changing side that would starve a side that took an empty base with a lower number than its own. A hex with one base of the side is fed as before. *Added by the architect on Nov 14, 2026, when the hand calculation was written.*

### Objectives

25. **`base_held`:** a named base and its points. A side scores the points if the base is its side's at the end. A side can always see whose a base on its map is, so its progress is known.
26. **`men_lost`:** points for each 100 enemy men lost, by any cause: unfed, with a ship, or in a fight. A part of 100 scores its part. A side cannot see it before the end, as with enemy ships sunk.
27. **`base_open` is as it was.** It counts a base's days open whoever holds it. A scenario that wants holding to matter uses `base_held`.

### What a side knows

28. **A side sees its own half of a fight:** the hex, its units' firing men and shots, and the men each of its units lost. It is not told its hits, the enemy's units, or the enemy's men. It learns that the enemy has men in the hex from the fight itself and nothing about how many. The estimate and the claim are stage 3.
29. **A side sees its own postures, fatigue and fed share exactly,** and its own units going back to defend. The enemy sees none of it.
30. **Both sides see a base change side,** with the stocks left in it seen by the new holder only. The beaten side sees its air groups and its ship lost; the winner is not told what was lost at the base.

## Placeholder figures

In `ground.toml`, and in the frozen step 1 figures with the same first values:

| Key | Value | What it is |
| --- | --- | --- |
| `men_per_shot` | `100` | Firing men to one shot |
| `hit_chance` | `"0.3"` | The chance a shot hits, for a side that attacks |
| `defender_hit_chance` | `"0.6"` | The same, for a side that defends |
| `men_per_hit` | `20` | Men a hit removes |
| `fatigue_per_attack_day` | `20` | Fatigue a unit gains on a day it attacks |
| `rest_per_day` | `5` | Fatigue a unit ashore loses on any other day |
| `attack_fatigue_limit` | `60` | Above this a unit on attack goes back to defend |
| `beaten_share` | `"0.2"` | A side attacked is beaten below this share of the attacker's men |
| `captured_stock_share` | `"0.5"` | The share of a base's supply and fuel its taker keeps |

None rests on a source. With them, 1,000 fresh, fed men who attack 1,000 who defend expect to lose 120 on the first day and to remove 60; a unit can attack on four days running before it stops. Stage 4 sets the figures against the attacks on the airfield that the record names.

## The test

Scenario 23, of made-up units and bases marked placeholder, in two parts: an attack that fails against a fed defender and stops by fatigue, and one that takes the base from an unfed defender. The hand calculation is in `tests/scenario_23_fight_ashore.rs`. It gives, for each day, each unit's firing men, shots and chance, the fatigue, and what follows from each count of hits; what the dice give is read from the run and checked against the breakdowns. Beside it, property tests:

- **A fight never creates men.** After every fight each side has no more men in the hex than before.
- **More attackers never do worse.** Over a batch, with more attacking men the defender is beaten no later on average.
- **Men are conserved** with the fight as a cause of loss.
- **The order of firing does not matter:** the result is the same with the sides taken in either order.
- **A base's side and the invariants agree** at the end of every day: no air group is at a base of the other side and no force has a home port of the other side.
- **A scenario with no posture order is unchanged** from stage 1.
- **The enemy's view of a fight** is the same in two worlds that differ only in the other side's fatigue, fed share and number of units, where its men and its losses to them are the same.

## Why

- **One body in a hex.** With no position inside a hex and every unit firing, the fight needs no rule for who faces whom.
- **The defender's advantage is one figure,** and hunger and tiredness each cut one number, the men who fire. So a side can read why an attack failed from three factors.
- **A base falls by a count, not a roll.** The fight is rolled; being beaten is read from the men left. A player can see how close a base is to falling.
- **Losing a base is a real loss.** Its aircraft, a ship in its port and half its stock go with it, so the sea and air war has a reason to feed and reinforce the men who hold it.

## What it leaves out

- **Flying out on the day a base falls.** The fight is after the last pulse and a rebase order is given at the start of a day, so the move must be ordered a day ahead.
- **Retreat.** A beaten side's men stay in the hex; there is no movement overland.
- **Guns ashore firing on ships, and artillery closing an airfield.**
- **Terrain, dug-in defence that grows, and kinds of unit.**
- **A base damaged by the fight for it.** It is taken whole but for its stocks.
- **Shore bombardment, bombs on men, the ground report and the estimate:** stage 3.

## What it costs

- **New state:** a unit's posture, its posture order, its fatigue and its fed share; a base's side becomes a thing that changes; an air group's place gains "lost". Each goes into the hash and is named beside it; the hash version is raised and the golden files remade (decision 0057).
- **One new draw,** `GroundFire`, added at the end. The `GroundCombat` subsystem code and the land unit's kind code exist.
- **Every rule that asks whose a base is was read for this record.** Those that need a change are decisions 20 to 24. The rest read the present side each time and are right as they are: cargo, refuelling, forward repair, the yard port, air supply, aircraft repair, the raid on a base and landing elsewhere. The loader's checks are against the starting side and stay.
- **Outside the engine a base's side is read once, at the start:** the replay's base list, the batch file's header and the viewer. Each must carry a base's side by day.
- **The agent interface:** the posture order, a unit's posture, fatigue and fed share in the briefing and queries, the fight in the briefing, a base lost or taken, with their schema versions.
- **The rulebook:** chapter 7 gains the fight and the base; chapters 1 (the ground combat phase, orders and refusals, what a side sees), 2 (bases on hexes), 5 (a group lost with its base) and 6 (the two kinds, and the known limit removed) change, with the glossary, in the same pull request as the code.

## Rule card

| | |
| --- | --- |
| Reads | A land unit's men ashore, hex, side, posture, fatigue and fed share; the bases in a hex, their side and stocks; the air groups at a base; a ship in port at it; a force's home port; an air group's mission; the nine figures |
| Writes | A land unit's men, posture, fatigue and fed share; a base's side, supply, fuel and whether it is on the map; an air group's aircraft, pilots and place; a ship sunk; a force's home port; an air group's mission; the events of the fight, of fatigue, and of a base changing side |

A base's side is now written by one rule, in one phase, after the last pulse and before the day's base count. Nothing else in a day runs after it but that count and the invariants.

## Questions for the designer

Answered on Nov 13, 2026.

1. **Decision 9:** a hit removes a fixed number of men. Agreed.
2. **Decision 16:** a base with no men ashore falls, without a fight, to a unit on attack in its hex. Agreed.
3. **Decisions 20 and 21:** build rebasing by order first (decision 0091). With it built, the groups still at a base that falls, and a ship in its port, are lost.

## As built

Built on Nov 14, 2026, to the sections above and the architect's hand calculation in `tests/scenario_23_fight_ashore.rs`. Nothing above this section was changed. Every choice the build made that the record did not is listed here. The rulebook's chapters 1, 2, 5, 6 and 7 state the rules as built.

- **The fed share** is held as a fixed-point number: the men fed over the men charged, cut to six decimals. A unit's firing men are its men ashore times its fed share times (100 less its fatigue) over 100, cut to a whole man.
- **Where the share is set.** The share of the units ashore in a hex is set by the side's `MenFed` event there. A unit with men aboard and none ashore at the charge has its share set back to 1, with a `FedShareRestored` event. A destroyed unit keeps its share.
- **Feeding from several bases (24a).** Each base with stock, in base order, is asked what is still owed and records its own `BaseSupplySpent`. Its breakdown has a factor `paid_by_earlier_bases_t` only where an earlier base paid. `MenFed` names the first base that paid, or the lowest-numbered base of the side in the hex if none did, so a hex with one base reads as in stage 1.
- **"Fired."** A unit fired if it is on attack in a fight and its shots are above 0. A unit on attack with no firing men rests.
- **A tired unit goes back to defend whether or not it fought.** A unit on attack with men ashore whose fatigue after the day's change is above the limit goes back to defend. It cites its fatigue event.
- **Fatigue events** are recorded only when the value changes.
- **Both sides beaten at once.** This can happen only when both attacked and neither has a man left. Then neither is beaten. *Awaiting the architect's ruling.*
- **The test of a side beaten is its own event,** `SideBeatenAshore`, recorded whenever a side is beaten, whether or not it holds a base there. No side is given it, since it weighs both sides' men. `BaseChangedSide` cites it and what it cites (the fight, or the posture orders of the units on attack), so both sides are told that the change has causes they cannot see.
- **The fight events.** The fight is one `GroundFight` event for each contested hex, and a side is given only its hex. Each unit's fire is a `GroundFire` event, one for every unit in the fight, including those with no firing men. Its side is given the firing men and shots and the breakdown's own factors (`men_ashore`, `fed_share`, `fatigue_factor`, `firing_men`, `men_per_shot`, `shots`), not the chance or the hits. Men lost are `MenLostInFight`, seen whole by their side, citing the enemy's fire.
- **A group lost with its base** has the new place `Lost` (hash code 3), no aircraft and no pilots, and keeps its mission. An order to it is refused with its own reason, "the air group was lost with its base", and not with the reason for a group lost with its ship.
- **A ship lost in port** has its own event, `ShipLostWithBase`, since `ShipSunk` names a task force. She is sunk and scored as sunk, and the air groups and men aboard her are lost as with a ship that sinks.
- **The fix-ups of decision 22** are the events `HomePortLost` and `StrikeMissionCleared`. A cleared group's mission is rest, with no mission order.
- **`base_open` known progress** is not observable for a base the side saw change side, since it was counted on days the side did not hold it. *Awaiting the architect's ruling.*
- **The refusals of a posture order** are tried in this order: not the side's own unit (`NotYourLandUnit`), destroyed (`LandUnitDestroyed`), no man ashore (`NoMenAshore`).
- **Scenario keys.** A land unit takes `posture` (`"defend"` or `"attack"`) and `fatigue`. An order takes `land_unit = n`, `kind = "set_posture"` and `posture = "attack"` or `"defend"`. Objectives take `base_held` with `base` and `points`, and `men_lost` with `points_per_100_men`. `men_per_hit` must be a whole number and `beaten_share` below 1.
- **The holder of a base at the time of each event** is worked out in the projection of the log from its `BaseChangedSide` events.
- **Two invariants are added:** a group lost with its base has no aircraft; a land unit's fatigue is from 0 to 100 and its fed share from 0 to 1.
- **The state hash is version 12.**
- **The agent interface.** A posture order is `{"unit": "land_unit:N", "order": {"kind": "set_posture", "posture": "attack"}}`, as every other unit order. Each own land unit line gives `posture`, `fatigue`, `fed_share` and the standing posture order's number. The briefing gives a new optional part, `base_changes`, for bases taken and lost, after the land units. A group lost with its base has no location and is marked `lost_with_base`. The schemas gain fields only, so their version stays 3.
- **The replay and the batch file.** The replay keeps each base's side at the start in its base list and adds `base_changes`, one entry for each `BaseChangedSide` (its day, pulse, base, from and to), left out when empty; the viewer follows the change events, so a side's page shows the holder at each step. The batch file adds `base_changes` to each run the same way. Neither changes version, and a run with no change reads as before.
- **`sim-cli run`** adds ", on attack" and ", fatigue N" to a land unit line only when the unit is on attack or tired, so a run with no posture order prints as in stage 1 but for its hashes.
