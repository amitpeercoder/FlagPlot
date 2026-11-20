# Objectives and results

A scenario may give each side a mission and objectives. The umpire counts the objectives from the final truth and the event log, turns the counts into points, and reads a result from the margin. No rule of the simulation reads any of it. Points are fixed-point (chapter 1, Numbers and chance). [0054]

A scenario with no objectives plays as before and has no result. [0054]

## Mission and objectives

### The mission

A side's mission is one or two sentences of words. It is not empty. No rule reads it. A task force also has a mission, which is a different thing: only shore bombardment reads it (a force on the bombardment mission fires at an enemy base, chapter 4, Shore bombardment). [0054, 0092]

### The scenario file

An optional `[objectives]` table holds the three levels, an optional `history` note, an optional `horizon_days`, and one table for each side. Each side has a `mission` and any number of `objective` entries, including none. Both sides are required once the table is given. Point values and levels are in the scenario file and are not stated here. Levels, points and points per ton are whole numbers or decimal strings. [0054]

The loader refuses levels that are not above zero and rising, negative points, an unknown kind of objective or ship or field, an objective naming a base the scenario does not have, an empty mission, a missing side, a `horizon_days` that is not a whole number of at least 1 or is shorter than the scenario's days, and a scenario in which any side has `ships_out_of_action` and any side has `ships_crippled`: a scenario counts damaged ships one way. In a ship table a kind that is left out is worth nothing. The same kind may be given more than once, and each entry is scored on its own. Objectives are kept in the file's order. [0054]

### The nine kinds

Each objective is a count of something the engine already records, taken for the side that holds the objective (`base_held` is read from the final truth). A side's own losses are not a separate objective: a ship lost gives the enemy its points. [0054]

| Kind | What it counts | Points |
| --- | --- | --- |
| `ships_sunk` | Enemy ships whose location is sunk, by kind of ship | For each kind of ship |
| `ships_crippled` | Enemy ships not sunk (afloat, in port or at the yard) with propulsion out or the flight deck out, by kind | For each kind of ship |
| `ships_out_of_action` | Enemy ships not sunk, each by her days out, the share of the horizon she is away | For each kind of ship, for a ship away for the whole horizon. The points earned are these times her share |
| `aircraft_destroyed` | Enemy aircraft lost for good, by any cause | For one aircraft |
| `cargo_landed` | Tons of supply and fuel the side's own ships unloaded at a named base, less the tons they loaded there, never below 0 | For one ton |
| `base_open` | The days on which a named base ended the day with supply above zero and at least one ready aircraft at it; with `closed = true`, the days on which it did not | For the base being so on every day. The points earned are these times the days counted over the days resolved |
| `base_held` | Whether a named base is the side's own at the end of the game | The objective's points, if it is. Scenario keys `base` and `points` |
| `men_lost` | Enemy men lost, by any cause: unfed, lost with a ship, lost in a fight, lost to fire or surrendered | For each 100 men. A part of 100 scores its part. Scenario key `points_per_100_men` |
| `men_landed` | Men the side's own ships land in the hex of a named base and do not take off again, kept unit by unit | For each 100 men. A part of 100 scores its part. Scenario keys `base` and `points_per_100_men` |

[0054, 0090, 0186]

- **Enemy** means the other side from the ship's, or the air group's, side, whoever did the damage. A ship counts for the other side if it sank, whatever sank it. [0054]
- **A ship at the yard is afloat.** A ship away at the yard (chapter 4, Repair) is not sunk and is counted as any ship not sunk, from the damage she left with, which does not change until she returns: at the end of the game one away with propulsion or the flight deck out counts as crippled, and one back from the yard counts as whole. Repair adds no objective of its own; `ships_out_of_action` reads its days of repair (below). [0069, 0088]
- **The horizon** is the number of days a result is judged over: the scenario's `days`, or `horizon_days` from the `[objectives]` table if it states a longer one. A short scenario that stands inside a longer operation states it, or every damaged ship in it would count as away for the whole game. A horizon stated with no `ships_out_of_action` objective is allowed and unused. [0088]
- **Ships out of action.** For each enemy ship not sunk, her **days out** are her days of repair at the end of the game (chapter 4, Repair, without the passage), plus, for each time she left for a yard during the game, the days of repair her `ForceLeftForYard` event gave her. A ship away at the yard at the end has 0 days at the end: the damage she carries there is what she left with, which her departure already counts. A ship that went to the yard and came back still counts for the days she left with, so repairing early neither gives nor takes points. A ship with no days out is not listed. Her **share** is her days out, never more than the horizon, over the horizon: one fixed-point division, cut to six places. Her points are her kind's points times her share: one fixed-point multiplication, cut to six places. The objective's points are the sum over the ships, taken in ship number order, and its count is the number of ships listed. A ship sunk is counted as sunk and not here, whatever came before. Crippled is still counted as before for the scenarios that use it. [0088]
- **A ship sunk is not also crippled.** A ship that is degraded in propulsion or flight deck does not count as crippled. Only out counts. This is the glossary's crippled. Flotation damage of 50 or more, which counts toward break-off in a fight, is called heavily damaged and does not enter the objective. [0054]
- **Aircraft destroyed** counts, through the side of the air group that lost them: aircraft shot down (`AircraftLost`), aircraft lost with a sunk ship, ready and damaged (`AirGroupLost`), ready and damaged aircraft destroyed on the ground (`AircraftDestroyedOnGround`, `DamagedAircraftDestroyedOnGround`), aircraft written off by wear (`AircraftWorn`, the written off part only), damaged aircraft written off when their group is rebased (`AircraftWrittenOff`), aircraft that ran out of fuel on the way home (`AircraftDitched`, though no one claims them), and aircraft that came down in the sea for want of a deck (`AircraftDitchedNoDeck`). Aircraft lost with a base that changed side (`AirGroupLostWithBase`, ready and damaged together), as a group lost with its ship is. Aircraft that land elsewhere are not counted. It does not count the worn aircraft that are repaired later, pilots lost, or any event that adds aircraft. [0054, 0059, 0090, 0091]
- **Cargo landed** is the tons in the `CargoUnloaded` events of ships of the side at the named base, less the tons in their `CargoLoaded` events at that base, supply and fuel together. Cargo put ashore and taken off again was not landed. Tons a supply line gives a base (chapter 2, Stock from off the map) are no cargo and are not counted: no ship unloaded them. A side that loads more there than it unloads has landed nothing: the count is never below 0. Loading at another base takes nothing from this one. Cargo destroyed ashore after it was landed still counts, and so does fuel landed and then taken as bunker fuel, which records `ShipRefuelled` and cannot be unloaded again. [0054, 0067]
- **Men landed** is kept unit by unit. Each land unit has a number that starts at 0. The events are read in order. Men of the unit in a `MenLanded` event of a ship of the side, in the hex of the named base, raise the unit's number by those men. Men of the unit in a `MenEmbarked` event of a ship of the side, in any hex, lower it by those men, never below 0. The objective's count is the sum of the numbers. So men put ashore and taken off again were not landed, and men landed, marched to another coast hex, embarked and landed again are not counted twice: the second landing only brings the number back to what it was. Men who embark before their unit has landed anyone there, and men of another unit who march into the hex and embark, lower nothing. A unit divided aboard is a new unit from then on, with a number of its own that starts at 0. The hex is the base's whoever holds the base, since a base does not move: men landed in the hex of a base the enemy holds count, and so do men landed before or after the base changes side. Men landed and later lost still count as landed (the enemy is scored for their loss under `men_lost` if he has that objective). The points are the objective's points times the men over 100, in one fixed-point product, so a part of 100 scores its part. The loader refuses a base the scenario does not have and points below 0. [0186]
- **Base open** is counted by days. At the end of each day, after the afternoon pulse when the day's strikes have landed, every base records `BaseAtDayEnd` with its supply and the ready aircraft of the air groups located at it (chapter 1, After the pulses). The base was open that day if the supply is above zero and at least one aircraft is ready. Aircraft aboard ships do not count. The count is the days it was open, or with `closed = true` the days it was not, and the points earned are the objective's points times that count over the days resolved: a base open on 21 days of 28, for 10 points, earns 7.5. So the two sides' objectives on one base sum to its points. Before a day is resolved the count and the points are 0. Either side may hold the objective for any base. A base that changes side is counted every day whoever holds it, so a scenario that wants holding to matter uses `base_held`. [0054, 0067, 0090 item 27]
- **Base held** scores the objective's points to the side that holds the named base when the game ends, read from the base's side in the final state. A base that changed side during the game scores for the side that has it at the end, however often it changed. The two sides may each hold the objective for one base, and then the base is worth its points to its holder. [0090 item 25]
- **Men lost** is the men in the `MenLostWithShip`, `MenLostUnfed`, `MenLostInFight`, `MenLostToFire` and `UnitSurrendered` events of the enemy's land units (the men who surrendered are lost; decision 0104), counted for the side whose enemy lost them, by any cause. The points are the objective's points times the men over 100, in one fixed-point product, so a part of 100 scores its part. [0090 item 26, 0092 item 20, 0104 item 7]
- **Ships lost with a base** count as sunk ships for the enemy of their side, since their location is sunk, whatever sank them (chapter 7, A base that changes side). [0090 item 21]
- **Ryujo counts as a carrier.** Points go by the six kinds of ship the engine has (chapter 3, What a report holds). [0054]

Known limit: a kind of ship finer than the six waits for step 2. [0052, 0054]

A side sees: its own mission and objectives from day 1, and what the enemy is scored for (below). It never sees the enemy's mission. [0054, 0183]

### What a side sees of the enemy's objectives

What each side is scored for is a rule of the game, and both sides read it, as both read the levels. From day 1 and on every day, the briefing's objectives part gives, after the side's own objectives and the line of the levels, the enemy's objectives, one for one in the order the scenario gives them: each one's kind; the base it names, where it names one, by the map's number and name; for `base_open`, whether the enemy is scored for the base being open or shut; and its worth (its points, its points a ton, its points for each kind of ship, or its points for each 100 men); and for `ships_out_of_action` the horizon in days. The kinds are all nine: `ships_sunk`, `ships_crippled`, `ships_out_of_action`, `aircraft_destroyed`, `cargo_landed`, `base_open`, `base_held`, `men_lost` and `men_landed`. [0183, 0186]

It is written in a form of its own, with no field for a count, days counted, points earned, a basis or whether a base is open now, so that nothing of the enemy's progress or state can be shown in it. The text puts it under the line "The enemy is scored for:", each objective in words from the reader's side: "your ships sunk", "tons of supply and fuel the enemy unloads at" a base, a base "is the enemy's at the end of the game", "days" a base "is shut" (or open), "men the enemy lands from ships at" a base "and does not take off again". A base that is not on the reader's map (a base that is off a side's map, such as Taivu, base 5, for the Allied side in scenario 103) is named by its hex, "(3,-4)", in the enemy's objective lines, in the side's own objective lines and in the result lines, for supply landed and men landed alike. [0183, 0186]

Not shown: the enemy's mission, which is his orders and may say what he believes of the reader; his progress on any objective, known or true; whether a base is open now for him; and anything else of his state. The loader refuses an objective naming a base the scenario does not have, but a base the scenario has may be off a side's map (`on_map = false`), and an objective may name it. Decision 0183's amended rule 3 said this could not arise; it does (differences, row 45). That where an objective is shows to the other side is allowed: it is part of the victory conditions. [0183, 0186]

A scenario with no objectives shows none, as before. Where the side has objectives and the enemy none, the part is there and empty, and the text says so in one line. A seat at a command is given no objectives, its side's or the enemy's. No staff, no trial and no rule of the simulation reads the enemy's objectives: a commander reads them and decides. They change no state, no event, no draw and no hash. [0183]

## Points and the result

### A side's points

The points of an objective are its count times its points: for `ships_sunk` and `ships_crippled`, the sum over the kinds of ships of the number of ships times the points for that kind; for `ships_out_of_action`, the sum of each ship's points, so a result is a sum with fractions. A side's total is the sum of the points of its objectives. [0054]

### The margin

The margin is a side's total less the enemy's total. The Allied margin is the Japanese margin with its sign changed. [0054]

### The three levels

The scenario gives one set of three levels, `minor`, `major` and `decisive`, above zero and rising. There is one set, not one for each side, because the margin is one number seen from both sides. [0054]

### The seven outcomes

The size of the margin, ignoring its sign, is compared with the levels. [0054]

| Size of the margin | Margin above zero | Margin below zero |
| --- | --- | --- |
| At least `decisive` | Decisive victory | Decisive defeat |
| At least `major`, below `decisive` | Major victory | Major defeat |
| At least `minor`, below `major` | Minor victory | Minor defeat |
| Below `minor` (zero included) | Stalemate | Stalemate |

A margin exactly equal to a level is that level. A margin strictly between minus `minor` and `minor` is a stalemate. One side's victory is the other's defeat at the same level. [0054]

### Not state

The objectives are the scenario's fixed configuration, held beside the reference data. They are not state and are not in the state hash. No rule reads the objectives or the result. The result is a function of the final truth and the event log, so a replay gives the same result. The count of a base at the end of each day is recorded for every base whatever the objectives say, so it reads no objective either. [0054, 0067]

A side sees: nothing of the result before the game ends. [0054]

## What a side knows of its progress

During the game a side sees the progress of each of its objectives as far as it can know it. The count is made from its own bases and the events its view gives it (decision 0047), and from nothing else. It is labelled with its basis. A side can believe it is winning and be wrong. A known count is never above the true count for the basis `exact`. A side's own losses are not part of its progress. [0054, 0056]

| Basis | Meaning |
| --- | --- |
| `exact` | Counted from the side's own units and events, as good as the truth |
| `claims` | Counted from the claims the side received. Claims can be wrong |
| `not_observable` | No unit or event the side may see shows it. The count is 0 and says nothing |

[0054, 0056]

| Kind | What the side knows |
| --- | --- |
| `cargo_landed` | Basis `exact`. The tons in its own ships' `CargoUnloaded` events at the base less the tons in their `CargoLoaded` events there, both of which it sees whole, never below 0. Points are the tons times points per ton |
| `base_open` at a base of its own | Basis `exact`. The days so far, from its own base's `BaseAtDayEnd` events, which it sees whole: the days as wanted, the days counted, and the points those earn |
| `base_open` at an enemy base | `not_observable`. The count is 0 |
| `base_open` at a base the side saw change side | `not_observable`. The count is 0: it was counted on days the side did not hold the base, which it cannot see |
| `base_held` | Basis `exact`, always: a side can see whose a base on its map is. The count is 1 and the points are the objective's points if the base is the side's own now, else the count is 0 and the points 0 |
| `men_lost` | Always `not_observable`: the enemy's men are not something a side knows before the end |
| `men_landed` | Basis `exact`. The count kept unit by unit from its own ships' `MenLanded` and `MenEmbarked` events, both of which it sees whole (Men landed, above). Points are the men times the points for each 100 men over 100. The enemy is told the objective and never the count |
| `ships_sunk` | Basis `claims` if the side has a claim that sank a ship or names a kind of ship, else `not_observable`. The count is the sum of the ships its claims say sank. Points are given only for the claims that name a kind, each kind's count times that kind's points. A claim from a surface fight names no kind, so it gives a count and no points |
| `aircraft_destroyed` | Basis `claims` once the side has any claim from the air, even one claiming none. The count is the aircraft claimed shot down plus those claimed destroyed on the ground. Points are the count times the points for one. Otherwise `not_observable` |
| `ships_crippled` | Always `not_observable`: no claim says crippled |
| `ships_out_of_action` | Always `not_observable`: the enemy's damage is not something a side knows [0088] |

[0054, 0056, 0088, 0090, 0186]

Claims come from a surface fight (chapter 4, The report and the claim) and from a raid (chapter 5, The report and the claim). Chapter 3, Claims, says what a claim counts. A side learns of its own attacks only as claims. A claim is never below what really happened, which bounds the truth from above. [0056]

A side sees: all of the above for its own objectives only; of the enemy's it sees what he is scored for and nothing of his progress (What a side sees of the enemy's objectives, above). The briefing carries the mission, each objective with its points, and the known progress; for `ships_out_of_action` the points for each kind are for a ship away for the whole horizon, and the horizon in days is given with them. [0088] The text says none for a scenario without objectives. [0054]

## The result at the end

When the game is over, both sides are told the umpire's result, from the truth. A result built on belief would not be a result. It gives each side's points by objective, with the count and for the ship objectives the count by kind, each side's total, the margin and the outcome. For `ships_out_of_action` it gives the horizon and lists each ship counted with her kind, her days at the end, the days she left with, her days out, her share and her points, so that the sum can be checked by eye; a side's own ships are named by its own handles and the enemy's by their place in that list [0088]. With it comes the scenario's `history` note, where the scenario gives one. The result is in the final briefing and in the answer that reports the game over. The result of a game asked for before it ends is the score so far, and it is handed out only at the end. [0054]

The result is also printed by the command-line runner after the final state, and a batch adds the Allied margin and outcome of each run, with a tally of the seven outcomes, seen from the Allied side, to its summary file. [0054]

A side sees: the whole result, both sides' points, once the game is over. [0054]

## The history note

The `history` note is words, not a computed score. It says how the day went in history and is marked as the architect's reading. Computing history's points needs loss figures from the order-of-battle facts for every objective and is step 2's work. [0054]

### Each scenario's scale

The scale is each scenario's own. The simulation's figures are one set for every battle and stay so; what a side is scored for, what each thing is worth and the three levels are the scenario's. There are two kinds of scenario, and a scenario is of record unless its file says otherwise. [0187]

- **A scenario of record** checks the simulation against history or proves a rule. It keeps the scoring it has, and its scoring is never chosen or changed for balance. Its point values and levels are placeholders set by the architect, to be set from the record in step 2. Until then a victory level is a statement about the placeholder scale and not about history. [0054, 0187]
- **A scenario for play** is meant to be played against a person, an agent or the engine. It is marked `for_play = true` in its `[objectives]` table (left out or false, it is of record). It may have a scale of its own, under three guards: the reason for each figure is written before a run made with it is read, the contest is read on the sign of the margin inside a band and not by moving the levels, and games between agents are played before it is called balanced (`docs/engineering/roles-and-tests.md`). Its results are never laid beside the record of the battle or beside another scenario's, and it is never a proof's or a validation measure's scenario. The runner prints its result under "the scenario's own scale, for play" in place of the words about decision 0054. The key changes no game and no hash. [0187]

Known limit: the point values and levels of a scenario of record are placeholders. [0054]

## Not in this chapter

- How a battle's ships are sunk, crippled and claimed: chapter 4, and chapter 3, Claims.
- How a ship's days of repair are worked out and how a force leaves for the yard: chapter 4, Repair.
- How aircraft are lost, destroyed on the ground and written off: chapter 5.
- How cargo is unloaded and how a base's supply and aircraft change: chapter 2, Cargo, and chapter 5.
- What events a side may see, and how a claim reaches it: chapter 1, What a side sees of an event, and chapter 3, Belief.
- Saved games, replays and the state hash: chapter 1, The state hash and saved games.
