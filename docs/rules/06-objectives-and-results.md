# Objectives and results

A scenario may give each side a mission and objectives. The umpire counts the objectives from the final truth and the event log, turns the counts into points, and reads a result from the margin. No rule of the simulation reads any of it. Points are fixed-point (chapter 1, Numbers and chance). [0054]

A scenario with no objectives plays as before and has no result. [0054]

## Mission and objectives

### The mission

A side's mission is one or two sentences of words. It is not empty. No rule reads it. A task force also has a mission, which is a different thing and which no rule reads either (glossary). [0054]

### The scenario file

An optional `[objectives]` table holds the three levels, an optional `history` note, and one table for each side. Each side has a `mission` and any number of `objective` entries, including none. Both sides are required once the table is given. Point values and levels are in the scenario file and are not stated here. Levels, points and points per ton are whole numbers or decimal strings. [0054]

The loader refuses levels that are not above zero and rising, negative points, an unknown kind of objective or ship or field, an objective naming a base the scenario does not have, an empty mission and a missing side. In a ship table a kind that is left out is worth nothing. The same kind may be given more than once, and each entry is scored on its own. Objectives are kept in the file's order. [0054]

### The five kinds

Each objective is a count of something the engine already records, taken for the side that holds the objective. A side's own losses are not a separate objective: a ship lost gives the enemy its points. [0054]

| Kind | What it counts | Points |
| --- | --- | --- |
| `ships_sunk` | Enemy ships whose location is sunk, by kind of ship | For each kind of ship |
| `ships_crippled` | Enemy ships not sunk (afloat or in port) with propulsion out or the flight deck out, by kind | For each kind of ship |
| `aircraft_destroyed` | Enemy aircraft lost for good, by any cause | For one aircraft |
| `cargo_landed` | Tons of supply and fuel unloaded by the side's own ships at a named base | For one ton |
| `base_open` | 1 if a named base has supply above zero and at least one ready aircraft at it, else 0; with `closed = true`, the opposite | For the base |

[0054]

- **Enemy** means the other side from the ship's, or the air group's, side, whoever did the damage. A ship counts for the other side if it sank, whatever sank it. [0054]
- **A ship sunk is not also crippled.** A ship that is degraded in propulsion or flight deck does not count as crippled. Only out counts. This is a different meaning from the glossary's crippled (flotation damage of 50 or more). [0054]
- **Aircraft destroyed** counts, through the side of the air group that lost them: aircraft shot down (`AircraftLost`), aircraft lost with a sunk ship, ready and damaged (`AirGroupLost`), ready and damaged aircraft destroyed on the ground (`AircraftDestroyedOnGround`, `DamagedAircraftDestroyedOnGround`), and aircraft written off by wear (`AircraftWorn`, the written off part only). It does not count the worn aircraft that are repaired later, pilots lost, or any event that adds aircraft. [0054]
- **Cargo landed** counts `CargoUnloaded` events of ships of the side, supply and fuel tons together, at the named base. [0054]
- **Base open** reads the final truth: the base's supply above zero, and at least one ready aircraft in the air groups located at that base. Aircraft aboard ships do not count. Either side may hold the objective for any base. [0054]
- **Ryujo counts as a carrier.** Points go by the six kinds of ship the engine has: carrier, battleship, cruiser, destroyer, transport, auxiliary. [0054]

Known limit: with ground combat deferred, no base changes hands, so no objective can be to take or hold ground. A kind of ship finer than the six waits for step 2. [0052, 0054]

A side sees: its own mission and objectives from day 1. It never sees the enemy's. [0054]

## Points and the result

### A side's points

The points of an objective are its count times its points: for the two ship kinds, the sum over the kinds of ships of the number of ships times the points for that kind. A side's total is the sum of the points of its objectives. [0054]

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

The objectives are the scenario's fixed configuration, held beside the reference data. They are not state and are not in the state hash. No rule reads the objectives or the result. The result is a function of the final truth and the event log, so a replay gives the same result. [0054]

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
| `cargo_landed` | Basis `exact`. The tons in its own ships' `CargoUnloaded` events at the base, which it sees whole. Points are the tons times points per ton |
| `base_open` at a base of its own | Basis `exact`. Its own base's supply and ready aircraft. Points are the count times the points |
| `base_open` at an enemy base | `not_observable`. The count is 0 |
| `ships_sunk` | Basis `claims` if the side has a claim that sank a ship or names a kind of ship, else `not_observable`. The count is the sum of the ships its claims say sank. Points are given only for the claims that name a kind, each kind's count times that kind's points. A claim from a surface fight names no kind, so it gives a count and no points |
| `aircraft_destroyed` | Basis `claims` once the side has any claim from the air, even one claiming none. The count is the aircraft claimed shot down plus those claimed destroyed on the ground. Points are the count times the points for one. Otherwise `not_observable` |
| `ships_crippled` | Always `not_observable`: no claim says crippled |

[0054, 0056]

Claims come from a surface fight (chapter 4, The report and the claim) and from a raid (chapter 5, The report and the claim). Chapter 3, Claims, says what a claim counts. A side learns of its own attacks only as claims. A claim is never below what really happened, which bounds the truth from above. [0056]

A side sees: all of the above for its own objectives only. The briefing carries the mission, each objective with its points, and the known progress. The text says none for a scenario without objectives. [0054]

## The result at the end

When the game is over, both sides are told the umpire's result, from the truth. A result built on belief would not be a result. It gives each side's points by objective, with the count and for the ship objectives the count by kind, each side's total, the margin and the outcome. With it comes the scenario's `history` note, where the scenario gives one. The result is in the final briefing and in the answer that reports the game over. The result of a game asked for before it ends is the score so far, and it is handed out only at the end. [0054]

The result is also printed by the command-line runner after the final state, and a batch adds the Allied margin and outcome of each run, with a tally of the seven outcomes, seen from the Allied side, to its summary file. [0054]

A side sees: the whole result, both sides' points, once the game is over. [0054]

## The history note

The `history` note is words, not a computed score. It says how the day went in history and is marked as the architect's reading. Computing history's points needs loss figures from the order-of-battle facts for every objective and is step 2's work. [0054]

Known limit: the point values and levels are placeholders set by the architect, to be set from the record in step 2. Until then a victory level is a statement about the placeholder scale and not about history. [0054]

## Not in this chapter

- How a battle's ships are sunk, crippled and claimed: chapter 4, and chapter 3, Claims.
- How aircraft are lost, destroyed on the ground and written off: chapter 5.
- How cargo is unloaded and how a base's supply and aircraft change: chapter 2, Cargo, and chapter 5.
- What events a side may see, and how a claim reaches it: chapter 1, What a side sees of an event, and chapter 3, Belief.
- Saved games, replays and the state hash: chapter 1, The state hash and saved games.
