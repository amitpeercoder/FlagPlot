# Schemas of the agent interface

One JSON Schema file for each kind of answer the agent interface gives
(decision 0048). They are written by hand and checked, by a validator in the
tests of `crates/sim-agent`, against every answer the scenario tests produce.

Every answer is a JSON object with a top-level `"format": "flagplot-agent"`,
`"version": 3` and a `"kind"` that names it.

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

`version` is 3. A field removed from an answer, or a field whose meaning
changes, raises the version by one. A field added does not change it, so a
reader must ignore fields it does not know. (The schemas here set
`additionalProperties: false` because they describe this version exactly; a
reader of a newer minor change should not.)

Version 2 (decision 0065) took the enemy's units out of the briefing: `forces`
of a fight became `force`, the side's own; `target` left a salvo fired and a
fighter's fire; `raid` left the guns' fire; `air_group` left a raid on a base;
and `by` of an attack on a ship became `"ship"` or `"aircraft"`. Nothing else
changed.

Version 3 (decision 0067) changed what two counts mean. The `count` of
`cargo_landed` is the tons unloaded at the base less the tons loaded back
there, never below 0. The `count` of `base_open` is the days the base ended
as wanted, no longer 1 or 0, and a new field `days` beside it says how many
days were counted; the points are the objective's points times `count` over
`days`. `days` is null for every other objective.

## What is not in an answer

An answer holds nothing of the enemy that the side may not know (decisions
0012, 0047 and 0065): an enemy ship, task force or air group appears only as a
contact. No event the side may see names one. Asked about anything it does not know of,
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
- An enemy unit in a raid, an attack or a fight is not named, in the JSON or
  in the text (decision 0065, which replaced item 8 of decision 0047). A fight
  has `force`, the side's own; fire, a raid and an attack on a ship have no
  field for the enemy; an attack on a ship says `by`: `"ship"` or
  `"aircraft"`. An air attack flown has `target` null. An enemy base is still
  named.
- An arrival "due day N" joins its group in the night pulse that begins day N;
  the text says so.
- `set_search_arc` and `set_air_mission` carry their rules in the field
  descriptions of `order.schema.json`, which the MCP server's input schema
  repeats.

## Claims (decision 0056, as built)

- `losses.claims` lists the claims the side received yesterday, written in the
  text as "Claims yesterday": what its own people believe they did in a fight
  at sea or a raid. They can be wrong. Fields added to each (the version stays
  1): `claimed_hits` (hits believed scored on ships), `taken_for` (the kind of
  ship the attackers took the contact for when the strike was launched, or
  `null`), `claimed_shot_down` and `claimed_on_ground` (enemy aircraft), and
  `from_air` (true for a raid, false for a fight at sea, whose claim says ships
  sunk only, with the rest 0 and `null`). A claim from the air reads "Claim over
  (7,-5): 12 aircraft shot down" or "Claim at (9,0): 5 hits on a carrier, 0
  sunk".
- A claim is never below what really happened and is never corrected later; a
  side fires and so claims more the more shots it fires, because each shot that
  missed may be claimed.
- `explain` on the claim's `event` number shows the firing events of the side's
  own that it rests on. A claim of the enemy's is never given.
- The `known` progress of `ships_sunk` and `aircraft_destroyed` objectives is
  read from the claims (`basis` `claims`); see the next section.

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
  side's own units and events), `claims` (the enemy ships and aircraft its own
  claims say it sank, shot down or destroyed on the ground, decision 0056: they
  can be wrong, so `count` can be above the truth; `points` are for the kind of
  ship each claim names, `null` where none does) or `not_observable` (no unit of its own and
  no event it may see shows it; `count` is 0 and says nothing). For `base_open`
  at a base of the side's own, `count` is the days the base ended as wanted and
  `known.days` the days counted (decision 0067); `days` is `null` otherwise.
  The text says `none` for a scenario without objectives. A side is never given the enemy's
  mission or objectives.
- At the end of the game the final briefing's `objectives.result` and the
  `end_day` answer that reports `game_over` carry the result for both sides:
  for each `allied` and `japanese`, the points by objective (`count`, the
  `ships` counted by kind, `points`), `total`, `margin` (its points less the
  other side's), `result` (`victory`, `defeat` or `stalemate`), `level`
  (`minor`, `major`, `decisive`, or `null`) and `outcome` in words; and the
  scenario's `history` note. The `end_day` answer always has a `result` field,
  `null` before the end and for a scenario without objectives.

## The wording pass after the second round (decision 0051)

Fields added (the version stays 1) and words changed after three models played
scenarios 5 and 7. No rule, result or hash changes.

- `objectives.levels` (`minor`, `major`, `decisive`, decimal strings, or `null`
  without objectives): the margins that make a victory or a defeat of each
  level; a margin of less than `minor` either way is a stalemate. Shown from
  day 1, in the text as "Levels: ...". A stalemate's `level` stays `null` and
  its `outcome` says `stalemate`.
- `objectives.result.history` is what happened in the real battle, not in the
  game just played; the text labels it so.
- The briefing's text heading says "This is the last day: the game ends when
  you end it." on the last day.
- `warnings.strike_group_out_of_reach` (a seventh part): an own air group on
  `naval_strike` that holds contacts and has none within its aircraft type's
  strike radius, or on `base_strike` at a base beyond it. It names the group,
  the radius, the nearest contact (or the target base) and its air distance,
  the figures `measure` `air_reach` gives.
- A force line (briefing, `list_forces`, `get_entity`) has `cargo_supply_t` and
  `cargo_fuel_t`: the tons of cargo aboard, summed over the ships afloat, set
  when above zero or when the force's mission is `transport` and `null`
  otherwise.
- An air group line has `strike_radius_nm` (for a fighter, its escort radius)
  and `search_range_nm`, the aircraft type's figures in miles.
- A briefing base has `stocks_yesterday`: `null` when nothing moved in or out of
  its stocks, otherwise `supply` and `fuel` parts (each `null` if that stock
  did not move) with `start_t`, `end_t` and the totals by cause: `landed_t`,
  `loaded_onto_ships_t`, and for supply `sorties_and_upkeep_t`, `repairs_t` and
  `destroyed_by_raids_t`, for fuel `refuelling_ships_t`. They come from the
  side's own events, and `start_t` is `end_t` less the net of the causes.
- A refusal shown to a player (`issue_orders` results, the briefing's refused
  orders, and an order's rejection in `explain`) is the engine's own words, and
  for a few reasons ends with a short hint in brackets of what the order needs
  (a load refused for being under way or away from a base; a base strike at an
  own or unknown base; a route that cannot be reached; combat air patrol for a
  group that is not fighters). The engine's text in the events and saved games
  is unchanged.
- `measure` `air_reach` takes `to` as a list of places or as one place (a list
  of one); a `to` that is neither is an error naming the shape, not `not_known`.
  One place that is not known anywhere in the question, a hex off the map for
  instance, still makes the whole answer `not_known`.
- The words for `set_patrol_share` say what the share is: the share of the group
  kept over its own hex (1 keeps every fighter at home and sends no escort).
