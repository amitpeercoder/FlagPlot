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

## As built

Added by the developer. Nothing above this heading was changed.

### The scenario file

An optional `[objectives]` table. A scenario without it plays as before and has no result.

```toml
[objectives]
minor = 3                 # the three levels: above zero and rising; one set for the
major = 8                 # scenario (the margin is one number seen from both sides)
decisive = 16
history = "..."           # optional note of how the day went (decision 9)

[objectives.allied]       # both sides are required
mission = "..."           # not empty

[[objectives.allied.objective]]
kind = "ships_sunk"       # or "ships_crippled"
points = { carrier = 10, battleship = 8, cruiser = 4, destroyer = 1, transport = 2, auxiliary = 1 }

[[objectives.allied.objective]]
kind = "aircraft_destroyed"
points = "0.05"           # for one aircraft

[[objectives.allied.objective]]
kind = "cargo_landed"
base = 1
points_per_t = "0.1"

[[objectives.allied.objective]]
kind = "base_open"
base = 2
closed = true             # optional, false by default
points = 4

[objectives.japanese]
mission = "..."
# objectives as for the Allied side, in any number and order
```

Levels, points and `points_per_t` are whole numbers or decimal strings; a TOML float is rejected, as everywhere. In the ship tables a kind that is left out is worth nothing. The loader refuses: levels that are not above zero and rising; negative points; an unknown kind of objective, of ship, or an unknown field; an objective naming a base the scenario does not have (checked by `World::set_objectives`); an empty mission; a missing side. An objective list may be empty (the side then scores nothing). The same kind may be given more than once (for example `cargo_landed` at two bases). Objectives are listed in the file's order, and the briefing, the known progress and the result keep that order.

### Where it lives

`sim-core/src/objectives.rs` holds the types, `tallies` (what each side achieved, from the final truth and the log), `score` (points, margin and level from the tallies, pure, on plain counts) and `known_progress`. `World` holds the objectives beside the reference data: not state and not in the hash, which no rule reads. `World::result()` gives the umpire's result; `SideView::mission()`, `SideView::objectives()` and `SideView::known_progress()` give a side its own.

### What each count reads

- `ships_sunk`: enemy ships with location sunk, by ship kind.
- `ships_crippled`: enemy ships not sunk (afloat or in port) with propulsion out or flight deck out, by kind. Degraded does not count.
- `aircraft_destroyed`: the enemy's aircraft, from `AircraftLost` (shot down), `AirGroupLost` (lost with a sunk ship, ready and damaged), `AircraftDestroyedOnGround` (ready), `DamagedAircraftDestroyedOnGround` (damaged) and the `written_off` of `AircraftWorn`. Not counted: the `worn` aircraft of `AircraftWorn` (damaged, repaired later), `PilotsLost` (pilots; the aircraft are counted by their own events) and every event that adds aircraft. Each event is attributed through its air group's side, so the enemy of that side scores it.
- `cargo_landed`: `CargoUnloaded` events of the side's own ships at the named base, supply and fuel tons together.
- `base_open`: the named base's supply above zero and at least one ready aircraft of an air group based there, read from the final truth. Either side may hold the objective for any base; `closed = true` reverses it.

### What a side knows (decision 7)

Counted from the side's own units and `SideView::events()`, each objective with a basis:

- `exact`: `cargo_landed` (its own ships' unloading is a whole event it sees) and `base_open` for a base of its own (its own base's supply and aircraft). As good as the truth.
- `claims`: `ships_sunk` counts the enemy ships of the `ClaimReceived` events the side has. A claim names no kind of ship, so the known points are not given. No rule makes a claim today.
- `not_observable`: `ships_crippled`, `aircraft_destroyed`, and `base_open` for an enemy base. No unit of the side's and no event it may see (decision 0047 cuts the hits out of its own firing) shows them. The count is 0 and says nothing; the briefing says so in words.

A known count is never above the true count. A side's own losses are not part of its progress.

### The agent interface and the runner

The briefing has an `objectives` part after the heading: the mission, each objective with its points, each with its known progress, and `result`, which is null until the game is over. The text says `none` for a scenario without objectives. At the end of the game the session adds the result for both sides (points by objective, margin, level and the history note) to the final briefing and to the `end_day` answer that reports `game_over`; the `end_day` answer always has a `result` field, null without objectives or before the end. `sim-cli run` prints the result after the final state; `sim-cli batch` adds to each run the Allied margin and result and a tally of the seven results (seen from the Allied side) to the summary file, for a scenario with objectives.

### Choices the decision did not state

- One set of levels per scenario, not one per side: the margin is one number.
- Both sides' mission is required once `[objectives]` is given; a list of objectives may be empty.
- The result of a game that is asked for before it ends is the score so far; the session and the runner only hand it out at the end.
- Scenario 5 carries the objectives of the placeholder scale and nothing else changes in it.
