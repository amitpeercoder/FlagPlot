# Schemas of the agent interface

One JSON Schema file for each kind of answer the agent interface gives
(decision 0048). They are written by hand and checked, by a validator in the
tests of `crates/sim-agent`, against every answer the scenario tests produce.

Every answer is a JSON object with a top-level `"format": "flagplot-agent"`,
`"version": 1` and a `"kind"` that names it.

| Kind | File | Given by |
| --- | --- | --- |
| `briefing` | `briefing.schema.json` | `briefing` |
| `list_forces` | `list_forces.schema.json` | `list_forces` |
| `list_bases` | `list_bases.schema.json` | `list_bases` |
| `list_contacts` | `list_contacts.schema.json` | `list_contacts` |
| `entity` | `entity.schema.json` | `get_entity` |
| `explain` | `explain.schema.json` | `explain` |
| `measure` | `measure.schema.json` | `measure`: one of four answers, told apart by `question` (`air_distance`, `sea_route`, `passage`, `air_reach`) |
| `not_known` | `not_known.schema.json` | `get_entity`, `explain` and `measure`, for anything the side does not know of |
| `briefing_text` | `briefing_text.schema.json` | `get_briefing` with `{"text": true}` |
| `issue_orders` | `issue_orders.schema.json` | `issue_orders` (its arguments: `issue_orders_arguments.schema.json`) |
| `end_day` | `end_day.schema.json` | `end_day` |
| `error` | `error.schema.json` | any call refused as a whole |
| `transcript_entry` | `transcript_entry.schema.json` | one line of a game's transcript |

`order.schema.json` is not an answer: it describes one order, as saved games
and `issue_orders` write it.

`defs.schema.json` holds what the files share: the decimal string, the unit
handle, the place and the one-line summaries of a task force, an air group, a
base and a contact. Files refer to it as `defs.schema.json#/$defs/name`.
`entity.schema.json`, `explain.schema.json` and `measure.schema.json` accept
either their own answer or `not_known`.

## How numbers are written

A count of things is a JSON integer. A quantity of the game (tons, fatigue,
miles that can be a decimal, shots) is a decimal string such as `"12"` or
`"0.25"`, exactly as the game holds it. There are no JSON floats anywhere.

## The version rule

`version` is 1. A field removed from an answer, or a field whose meaning
changes, makes the version 2. A field added does not change it, so a reader
must ignore fields it does not know. (The schemas here set
`additionalProperties: false` because they describe version 1 exactly; a
reader of a newer minor change should not.)

## What is not in an answer

An answer holds nothing of the enemy that the side may not know (decisions
0012 and 0047): an enemy unit appears only as a contact or, by the log's
number, in an event the side may see. Asked about anything it does not know of,
`get_entity` and `explain` give the same `not_known` document, which carries
nothing of the question.

## measure

`measure` takes one question as a JSON object whose `question` names it
(decision 0049):

| `question` | Other fields | Answers |
| --- | --- | --- |
| `air_distance` | `from`, `to`: places | the straight-line air distance, in whole miles (decision 0040) |
| `sea_route` | `from`, `to`: places | whether ships can get there, the hexes entered in order and the miles |
| `passage` | `task_force`: `"task_force:N"`; `to`: a place; `speed` (optional): `"cruise"`, `"full"` or knots above zero, `17` or `"17.5"` | a forecast of the force's passage: the route, the speed, the hours, the day, pulse and clock time of arrival, and each ship's fuel |
| `air_reach` | `air_group`: `"air_group:N"`; `to`: a list of places | the type's radius, and for each place the air distance and whether it is within the radius |

A place is `{"q": 2, "r": -4}`, or the handle of one of the side's own ships,
task forces, air groups or bases, or of a contact it holds (which stands for
the hex the side believes). Anything else gives `not_known`: an enemy unit, a
number that does not exist, a handle that is not one, a hex off the map, a
unit of the wrong kind for the question (a ship as the task force of a
passage), and a sunk ship or a group with no place. It is the same document in
every case and carries nothing of the question.

A passage is a forecast, and its `assumptions` say so in fixed words: the force
leaves at the start of the coming day, keeps the speed it makes now, meets
nobody, loses no ship and has the fuel aboard now. `order` is `refused`, with a
`refusal`, when the engine would refuse the route order (`no_route`,
`already_there`, `no_ship_afloat`) and `outcome` is then `does_not_sail`.
`route` lists every hex of the route; `entered_steps` of them are entered
before the force arrives or stops (they differ only when a ship runs dry),
and are the hexes the steaming events list. A hex the force is already in
(past the middle of a leg) is not entered again. A force with no ship that can
make way has `outcome` `cannot_move` and an empty `route`. In `air_reach`,
`within` is true when the distance is not more than the radius, the test the
strike decision makes; `radius_is` says what the radius is (`strike`, a
fighter's `escort`, or `none`, for a type that cannot strike, which reaches
nothing).

Numbers follow the rule above: whole miles and counts are integers; tons,
hours, speeds and the miles of a passage are decimal strings.

## Playing a side (decision 0050)

A `Session` holds a game; `Session::call(seat, tool, arguments)` is the one
entry point and `sim-cli serve` wraps it in JSON lines. The tools are
`get_briefing` (`{"text": true}` for the text), `list_forces`, `list_bases`,
`list_contacts` (each `{"area": {"q", "r", "radius_nm"}}` optional),
`get_entity` (`{"handle": ".."}`), `measure` (the question object itself),
`explain` (`{"event": N}`), `issue_orders` and `end_day`. A call refused as a
whole (a tool that is not one, arguments that are not arguments, an order after
the day is closed or the game is over) gives an `error` document and changes
nothing.

### Orders

`issue_orders` takes `{"orders": [order, ...], "clear": true?, "reason": "..."?}` (`reason` is the player's own words for why, kept in the transcript and never read by the game). An order is
`{"unit": "task_force:3", "order": {"kind": "set_route", "waypoints": [[2, -4]]}}`;
`order.schema.json` lists every kind. `clear` empties the seat's queue first.
For each order in the order given the answer says `queued` or `refused` with a
reason. The engine's own check, the one made when the day begins, is made
against the state now. A reason is the engine's own words, except that an order
for a unit the side does not own, a unit that does not exist and a unit not
written as `task_force:N` or `air_group:N` all get the one reason `not known`.
A malformed order is refused with a reason beginning `malformed order`; the
others in the call stand. The answer holds no part of the order it answers.

The verdict is a forecast: an order queued can be refused when the day begins
(for instance a load after a route order, which finds the force under way) and
the next briefing's `orders.refused` says so.

The answer also counts the call: `queued_in_call`, `refused_in_call` and a
`summary` sentence say how many orders of this call were queued and how many
refused, and that a refused order does not stop the others; `queued` is the
whole queue for the coming day, all calls together.

### Time

`end_day` says the seat has finished. When every seat held by a player has, the
day is resolved. The answer: `resolved`, `day` (days resolved) and `game_over`.
After `end_day` and before the resolution the seat can read and cannot order;
after the last day `end_day` changes nothing.

### The record

The session keeps the orders as handed in to the engine (a saved game made of
them replays with no player) and a transcript of every call a player's seat
made, numbered, with `after_day` (days resolved when it was made), the tool,
the arguments and the whole answer. It holds no time of day.

### Bases of the other side

Where every base is and which side holds it is the map, not intelligence
(decision 0034, 2), so a side is told those facts and nothing else of an enemy
base. `list_bases` has a second list, `enemy_bases`, with a line for each base
the side does not own (`base` handle with its name, `place`, `held_by`); the
area filter applies to it. `get_entity("base:N")` for such a base gives an
`entity` whose `entity_kind` is `enemy_base`: a `summary` with those three
fields and `known`, the fixed words "number, name, place and holder only".
`measure` takes such a base as a place and names it. A base number that does
not exist is still `not_known`, so the two cases are deliberately told apart.
No supply, fuel, guns, repairs or aircraft of an enemy base appear anywhere;
`SideView::enemy_bases` returns a `PublicBase`, a struct with no field for them.
A base strike is therefore judged by facts the side can list.

## Changes after the first models played (decision 0051, as built)

Two models played scenarios 5 and 7 alone and found gaps in what is told. None
changes a rule, a result or a hash.

- The briefing's heading has `last_day`, the last day of the game, and the text
  says `Day: 2 of 5` (`Day: 6 (the game is over: day 5 was the last)` after it).
  A view does not hold the length of the game, so the session passes it to
  `briefing_for_game`; `briefing` leaves it out (`null`).
- A force line has `movement` (`normal`, `slowed` or `cannot_move`) and
  `propulsion_damaged` (the ships afloat with propulsion `degraded` or `out`).
  The briefing's warnings have a sixth part, `task_force_propulsion`, naming the
  force, the effect and the ships. A ship with propulsion out has no speed and
  the force steams at its slowest ship's speed, so one stops the whole force
  whatever its status says.
- `pilots: null` on an air group means the scenario does not count its pilots:
  no limit applies. The text says "pilots not counted (no pilot limit)".
- `port_size` and `airfield_size` of a base are the map's, 0 where it has none.
  No rule reads the airfield size; the port size sets only how fast cargo is
  worked.
- The text calls an enemy unit in a raid, an attack or a gun's target "enemy".
  The number is the log's and nothing else is known of it (decision 0047, 8);
  `get_entity` says "not known" for it as for every enemy unit.
- An arrival "due day N" joins its group in the night pulse that begins day N;
  the text says so.
- `set_search_arc` and `set_air_mission` carry their rules in the field
  descriptions of `order.schema.json`, which the MCP server's input schema
  repeats.

## Objectives and the result (decision 0054, as built)

A scenario may give each side a mission and objectives; neither changes a rule
or a hash.

- The briefing has an `objectives` part after the heading (a field added, so
  the version stays 1): `mission` (text, or `null` without objectives),
  `objectives` (one line for each, in the scenario's order) and `result`
  (`null` until the game is over). A line gives the `kind` (`ships_sunk`,
  `ships_crippled`, `aircraft_destroyed`, `cargo_landed`, `base_open`), the
  `base` where there is one, `closed` for `base_open`, the points (`ship_points`
  for each kind of ship, or `points` for one aircraft, one ton or the base), and
  `known`: what the side knows of its progress. `known.basis` is `exact` (the
  side's own units and events), `claims` (enemy ships it is told are sunk, of no
  known kind, so `points` is `null`) or `not_observable` (no unit of its own and
  no event it may see shows it; `count` is 0 and says nothing). The text says
  `none` for a scenario without objectives. A side is never given the enemy's
  mission or objectives.
- At the end of the game the final briefing's `objectives.result` and the
  `end_day` answer that reports `game_over` carry the result for both sides:
  for each `allied` and `japanese`, the points by objective (`count`, the
  `ships` counted by kind, `points`), `total`, `margin` (its points less the
  other side's), `result` (`victory`, `defeat` or `stalemate`), `level`
  (`minor`, `major`, `decisive`, or `null`) and `outcome` in words; and the
  scenario's `history` note. The `end_day` answer always has a `result` field,
  `null` before the end and for a scenario without objectives.
