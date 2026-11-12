# 0054: Objectives and results

Status: confirmed. The designer approved the plan on Nov 12, 2026. The point values and the levels are placeholders set by the architect, to be set from the record in step 2.

## The gap

A scenario ends when its last day is resolved and says nothing more. No side has an aim and no game has a result. Two models that played a side each said so: neither could tell what it was meant to achieve. The staff planner of step 3 will need aims it can measure, and step 2 needs one scale on which a run and the record can be compared.

## Decisions

1. **A scenario may give each side a mission and objectives.** The mission is one or two sentences. An objective is a count the umpire reads from the truth at the end of the game, with the points one of it is worth. A scenario without objectives plays as before and has no result.
2. **Five kinds of objective**, each a count of something the engine already records:

   | Kind | Counts, for the side that holds the objective | Given as |
   | --- | --- | --- |
   | `ships_sunk` | enemy ships sunk | points for each kind of ship |
   | `ships_crippled` | enemy ships afloat at the end with propulsion out or flight deck out | points for each kind of ship |
   | `aircraft_destroyed` | enemy aircraft lost for good, by any cause: shot down, destroyed on the ground, lost with their ship, written off | points for one aircraft |
   | `cargo_landed` | tons of supply and fuel the side unloaded at a named base | points for one ton |
   | `base_open` | 1 if, at the end, a named base has supply above zero and at least one aircraft ready; with `closed = true` the opposite, for the side that wants it shut | points |

   A ship sunk is not also crippled. A side's own losses are not a separate objective: a carrier lost gives the enemy its points, and one ledger is enough.
3. **A side's points** are the sum over its objectives of count times points, in fixed point, cut to six decimal places.
4. **The result is read from the margin**, a side's points less the enemy's, against three levels the scenario gives (`minor`, `major`, `decisive`, rising). A margin of at least `decisive` is a decisive victory, at least `major` a major victory, at least `minor` a minor victory. A margin between minus `minor` and `minor`, neither included, is a stalemate. One side's victory is the other's defeat at the same level.
5. **No rule reads the objectives or the result.** They are not state and are not in the hash. The result is a function of the final truth and the event log, so a replay gives the same result.
6. **A side sees its own mission and objectives from day 1,** in the briefing, and never the enemy's.
7. **During the game a side sees its progress as far as it can know it:** each of its objectives counted from its own units and from the events it may see (decision 0047). It is labelled as what the side knows. A side can believe it is winning and be wrong.
8. **At the end both sides are told the umpire's result:** each side's points by objective, the margin and the level, from the truth. The game is over, and a result built on belief would not be a result. With it comes the scenario's note of how the day went in history, where the scenario gives one.
9. **The historical note is words, not a computed score, for now.** Computing history's points needs loss figures from the order-of-battle facts for every objective; that is step 2's work. Until then a scenario may carry `history = "..."`, marked as the architect's reading.
10. **Ryujo counts as a carrier.** Points go by the six kinds of ship the engine has. A finer scale (fleet carrier, light carrier, heavy and light cruiser) waits for step 2.

## The placeholder scale

Used by every scenario until step 2 says otherwise.

| Ship | Sunk | Crippled |
| --- | --- | --- |
| Carrier | 10 | 5 |
| Battleship | 8 | 4 |
| Cruiser | 4 | 2 |
| Destroyer | 1 | 0.5 |
| Transport | 2 | 1 |
| Auxiliary | 1 | 0.5 |

One aircraft destroyed is worth 0.05. Levels for a one-day battle: minor 3, major 8, decisive 16.

## Why

- **Counts the engine already has** mean no new state and nothing new to keep true.
- **One margin on one scale** lets a run, a batch of a thousand runs and the record be laid side by side.
- **Progress from what a side may see** keeps the rule of decision 0047: a tool tells a side nothing its view does not hold.

## What it costs

- The points are invented. Until step 2 a "minor victory" is a statement about the placeholder scale and not about history.
- With ground combat deferred (decision 0052) no base changes hands, so no objective can be to take or hold ground.
- Savo Island has no transports in it (decision 0024), so the Allied aim there, protecting them, cannot be scored.

## Rule card

| | |
| --- | --- |
| Reads | Ships (sunk, systems, kind, side); the event log (aircraft lost, cargo unloaded); bases (supply, aircraft ready); the scenario's objectives |
| Writes | Nothing. The result is computed when asked |
