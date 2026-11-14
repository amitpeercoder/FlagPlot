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
| `measure` | `measure.schema.json` | `measure`: one of five answers, told apart by `question` (`air_distance`, `sea_route`, `passage`, `yard`, `air_reach`) |
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

The fight ashore (decision 0090, see "The fight ashore" below) adds only fields
and values, so `version` stays 3.

Land units (decision 0066) add only fields, so `version` stays 3: the
briefing's `land_units` part and `men_fed_t` of a base's supply change, the
`land_units` list of `list_forces`, the `land_unit` entity, `troops` of a ship
and of a ship line, and the `unit` and `"none"` cargo of a load order (see
"Land units" below). Each is left out when it has nothing to say, so an answer
for a scenario without a land unit is byte for byte what it was.

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
| `yard` | `task_force`: `"task_force:N"` | what a `send_to_yard` order given now would do (decision 0069): whether the engine would take it and why not, the port and passage, each ship's days of repair, the days away, the day it leaves, `return_day` and `returns_before_end`; for a force away, its port and `return_day` |
| `air_reach` | `air_group`: `"air_group:N"`; `to`: a list of places | the type's radius, the transfer range a `rebase` order can cover, and for each place the air distance and whether it is within the radius |

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
`already_there`, `no_ship_afloat`, `out_of_fuel`) and `outcome` is then `does_not_sail`.
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
  at sea or a raid. They can be wrong. Fields added to each (the version was then
  1; it is now 3): `claimed_hits` (hits believed scored on ships), `taken_for` (the kind of
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

- The briefing has an `objectives` part after the heading (a field added; the version
  was then 1, it is now 3): `mission` (text, or `null` without objectives),
  `objectives` (one line for each, in the scenario's order) and `result`
  (`null` until the game is over). A line gives the `kind` (`ships_sunk`,
  `ships_crippled`, `ships_out_of_action`, `aircraft_destroyed`, `cargo_landed`,
  `base_open`, and, added with the ground fight (decision 0090), `base_held` and
  `men_lost`: see "The fight ashore" below), the
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
  For `ships_out_of_action` (decision 0088) the line also gives `horizon_days`, and
  the result's score line gives `horizon_days` and `ships_out`, one entry for each
  ship counted (both left out for every other kind).
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

Fields added (the version was then 1, it is now 3) and words changed after three models played
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

## Land units (decision 0066, as built)

A side knows its own land units exactly and nothing of the enemy's; no answer
has a field for an enemy land unit, and a base the enemy formed by a landing is
not among `enemy_bases` and is `not_known` to `get_entity` and refused by a base
strike as a base that does not exist.

- **Handles.** A side's land units are `land_unit:1`, `land_unit:2`... in the
  order of the engine's numbers, destroyed ones counted, so a number never
  changes; a unit made in play (a unit divided, decision 14) takes the next.
  Bases keep the engine's numbers, a base formed by a landing included.
- **The load order** takes an optional `unit` (`"land_unit:N"`) and `cargo` may
  be `"supply"`, `"fuel"` or `"none"`: `{"kind": "load", "unit": "land_unit:2",
  "cargo": "none"}`. With a unit and no `cargo` the cargo is supply. `"none"`
  without a unit is refused (`malformed order`). Without a unit the order is as
  it always was, and a load of supply is still written `{"kind": "load"}`. In a
  saved game `unit` holds the engine's number; `issue_orders` takes the side's.
  A unit that is not the side's own and one that does not exist are refused in
  the same words, and the refusal names no number.
- **`list_forces`** has `land_units` (left out when the side has none): a
  `land_unit_line` for each unit, in `area` if one is given (a unit with men
  ashore is where they are, one wholly aboard where its first ship is).
- **`get_entity("land_unit:N")`** gives `entity_kind` `land_unit` with that
  `summary`: `men_ashore` and `place`, `men_aboard` with `aboard` (each ship
  and its count), `destroyed`, and `fed_last_night` (`fed`, `partly_fed`,
  `unfed` from the side's own `MenFed` event of the last night pulse in the
  unit's hex; `not_ashore`; `not_known` when the side holds no such event).
  A ship's detail and a ship line have `troops` when she carries men.
- **The briefing's `land_units`** part follows `bases` and is left out when the
  side has no unit: the units, then yesterday's men landed and embarked, bases
  formed, units divided, loads finished, how the men ashore were fed, men lost
  (`unfed` or `lost_with_ship`) and units destroyed, then `supply_by_hex`: for
  each hex with men ashore, the base that feeds them (the side's lowest-numbered
  there, or null), the aircraft at it, its stock, the daily charge (aircraft
  times `daily_supply_t_per_aircraft` plus men times `daily_supply_t_per_man`)
  and the days it lasts at that charge. A base's `stocks_yesterday.supply` has
  `men_fed_t` when anything was paid for men. `rules_in_play.supply` is true
  where the ground figures are loaded as well as where the air supply figures
  are, since bases then spend supply on men.

## The fight ashore and a base that changes side (decision 0090, as built)

Fields and values added only, so `version` stays 3. The one place a field that
was always present can now be `null` is `location` of an air group line, which
is `null` for a group lost with a base that changed side: no group had that
state before, so no answer of an earlier game changes, and the group's line
also has `lost_with_base: true` (left out otherwise) and `place: null`.

- **The posture order.** A land unit is ordered like any other unit:
  `{"unit": "land_unit:2", "order": {"kind": "set_posture", "posture":
  "attack"}}` (`defend` or `attack`). The unit is the side's own handle; one
  that is not the side's own, one that does not exist and one written wrongly
  all get the one refusal `not known`. A destroyed unit and a unit with no man
  ashore get the engine's reasons (`the land unit is destroyed`, `the land unit
  has no men ashore`) with a hint. A posture order is a standing order.
  `order.schema.json` has a `land_unit_order`.
- **A land unit line** (`list_forces`, `get_entity`, the briefing's `units`)
  has `posture`, `fatigue` (0 to 100) and `fed_share` (0 to 1), and
  `posture_order`, the side's number of its standing posture order, left out
  when there is none.
- **The briefing's `land_units` part** has, each left out when empty,
  `fights_yesterday` (the hex only, never the attacker), `fire_yesterday` (the
  side's own unit, its firing men and its shots, never the chance or the hits),
  `fatigue_yesterday`, `back_to_defend_yesterday` and
  `fed_share_restored_yesterday`. Men lost in the fight are lines of
  `lost_yesterday` with `cause` `fight`, the hex and what is left ashore.
- **The briefing's `base_changes` part** follows `land_units` and is left out
  when nothing changed side and nothing was lost with a base: `taken` (the
  stocks kept and the stocks lost), `lost` (the base and the hex, never its
  stocks), `air_groups_lost`, `ships_lost`, `home_ports_lost` and
  `strike_missions_cleared`. A side is told only its own half: the winner is
  not told of the groups and the ship the other side lost with the base.
- **Objectives.** `base_held` (the base is the side's own at the end; known
  progress is exact, `count` 1 when the base is the side's own now) and
  `men_lost` (points for each hundred enemy men lost, by any cause; not
  observable before the end). They appear in the briefing and in the result.
- **Bases.** The side of a base changes during a game, so the lists of bases
  and the places that name them are read from the present state on every call;
  nothing is kept from the start of the game.
- **Supply by hex counts every base of the side in the hex.** The briefing's
  `land_units.supply_by_hex` line used to count the stock and the aircraft of the
  side's lowest-numbered base in the hex alone. It now counts the stock and the
  aircraft of every base of the side in the hex, as feeding does (decision 0090,
  24a): `supply_t`, `aircraft`, `daily_charge_t` and `days_of_supply` are for the
  bases together. `base` is still the lowest-numbered one. A new field,
  `other_bases` (handles), lists the side's other bases in the hex in base order and
  is left out when there are none, so a hex with one base reads as it did. Fields
  and values added only, so `version` stays 3.

## Shells, bombs and what a side knows of the enemy ashore (decision 0092)

Fields and values added only, so `version` stays 3. Each is left out when it has
nothing to say, so the answer for a game with no bombardment, no estimate and no
man hit by a bomb is byte for byte what it was.

- **The briefing's `enemy_ashore` part** follows `contacts`: `estimates`, one
  for each hex where the side holds a ground estimate of the enemy's men ashore
  (`place`, `men`, `day` reported, `age_days`, `event`), and
  `bases_found_yesterday` (an enemy base a ground report put on the side's map).
  An estimate is the count of the last report, wrong by up to half either way;
  it is never aged or corrected, and an opening estimate is dated day 0. The true
  count and the error are never given. `list_contacts` has the same estimates as
  `ground_estimates`, in the area if one is given.
- **The briefing's `action`** has `bombardments` (the side's own force, the base,
  shells and weighed shots, never a hit), `shell_fire` (each own ship's shells and
  the rounds it has left) and `bombardments_against` (the side's own base: shells,
  weighed shots, hits and where they landed, never the force or the ships). A raid
  on a base has `hits_on_men` when a hit landed on men.
- **Claims** have `from_bombardment` (true for the claim of a shore bombardment:
  `claimed_on_ground` aircraft, `from_air` false).
- **Men lost** in `land_units.lost_yesterday` have the cause `fire` (a bomb or a
  shell on the base).

## Marching overland (decision 0094)

Fields and values added only, so `version` stays 3. Each is left out when it has
nothing to say, so the answer for a game with no march (and for any map with no
land links) is byte for byte what it was.

- **The march order.** `{"unit": "land_unit:1", "order": {"kind": "march",
  "hex": [1, 0]}}`: the hex is a neighbour of the unit's hex joined to it by a
  land link, or the unit's own hex, which stops a march. It stands until the unit
  arrives or another march order replaces it. A unit that is not the side's own,
  one that does not exist and one written wrongly get the one refusal `not
  known`; a destroyed unit, a unit with no man ashore and a hex no land link
  joins to the unit's get the engine's reasons with a hint (the last names
  `march_to`). `order.schema.json` has the kind in its `land_unit_order`.
- **A land unit line** (`list_forces`, `get_entity`, the briefing's `units`) has
  `march` (`to`, `days_marched`, `days_counted` (decision 0098), `of_days`, and `order`, the side's number of the
  march order) while a march order stands, and `march_to`, the places the unit can
  be ordered to march to (the neighbours the map joins to its hex by land, in hex
  order; left out for a unit with no man ashore and on a map with no land links).
- **The briefing's `land_units` part** has, each left out when empty,
  `marches_yesterday` (a day marched: `from`, `to`, `days`, `counted`, `of_days`),
  `march_arrivals_yesterday` (`men`, the `supply_t` carried and the `base` it went
  into, null when none was carried), `bases_formed_by_march_yesterday` (the side's
  own base) and `marches_stopped_yesterday` (`days` marched and `reason`:
  `ordered`, `no_men_ashore` or `no_land_link`). `fatigue_yesterday` changes have
  the values `marched` and `marched_and_attacked`.
- **A base's `stocks_yesterday.supply`** has `taken_by_marches_t` (taken out by a
  unit that marched out of the base's hex) and `arrived_with_marches_t` (put in by
  one that marched into it), each present only when it is above 0, so the line
  still adds up. What a march takes out is a one-off, so it is not part of
  `spent_yesterday_t` and does not enter the days of supply.
- **What the enemy is told.** Nothing of a side's march is in the other side's
  answers: no unit, no day marched, no arrival and no supply carried. It learns
  that men have come by a ground report (an estimate of men in the hex) and that a
  base exists when a report finds it (`bases_found_yesterday`).

## A force that acts and steams on (decision 0096)

Fields and values added only, so `version` stays 3. Each is left out when it has
nothing to say, so the answer for a game with no marked route and no halt (and
every answer for a scenario that does not use the rule) is byte for byte what it
was.

- **The route order's `act_at`.** `{"kind": "set_route", "waypoints": [[1, 0],
  [3, 0], [1, 0]], "act_at": [[3, 0]]}`: hexes, each one of the waypoints; every
  waypoint at a marked hex is marked, the last never counts as marked. Written
  only when it is not empty, so a route with no mark is written as it always was.
  A mark that is not a waypoint is refused with the engine's words and a hint
  (`order.schema.json`, `task_force_order`).
- **The status `halted`** (`force_line.status`).
- **A force line** (`list_forces`, `get_entity`'s `summary`, the briefing's
  `task_forces`) has `act_at` (the marks of the route not yet passed, as places in
  hex order) and `halt` (`place`, `since` as a clock time, `waits_for`: `bombard`,
  `cargo` or both, and `bombarded` once it has fired).
- **The briefing's `halts` part** has `halted` (`task_force`, `place`, `at`,
  `waits_for`, `event`) and `steamed_on` (`task_force`, `place`, `waited_for`,
  `reasons`: `bombarded`, `cannot_bombard_any_longer`, `cargo_work_done`, and
  `event`). `explain` of either event works like that of any own event.
- **What the enemy is told.** Nothing: no halt, no mark, no steaming on. It sees
  the force only as it sees any force, by contact.

## Preparing a landing and the experience of a land unit (decisions 0095 and 0097)

Fields and values added only, so `version` stays 3. A reader must ignore fields it
does not know. Where an object is left out when it has nothing to say, a game with
no preparation looks as it did.

- **The prepare_landing order.** `{"unit": "task_force:1", "order": {"kind":
  "prepare_landing", "hex": [4, 0], "roster": ["land_unit:1"]}}`: the roster
  names the side's own land units by the side's numbers, and may be empty. A
  standing order until replaced or until the landing is done; it may be given at
  sea. A roster unit that is not the side's own or does not exist gets the refusal
  `the land unit is not one of your units` with a hint, a destroyed one `the land
  unit is destroyed`, and a hex that is not a coast hex of the map `(q,r) is not a
  coast hex on the map`, with a hint. The same order again changes nothing.
- **The set_activity order.** `{"unit": "land_unit:1", "order": {"kind":
  "set_activity", "activity": "train"}}`, `ready` or `train`; a standing order. It
  is accepted for a unit aboard ship, and refused as `not known` for a unit that is
  not the side's own.
- **A force line** (`list_forces`, `get_entity`, the briefing's `task_forces`) has
  `preparation` while the force holds one: `hex`, `roster` (handles), `loading` (0
  to 100), `landing_rate` (the share of the port's rate it would unload at in `hex`,
  the engine's own `landing_rate_factor`, 1 being the port's rate) and `order`; and
  `landing_at`, the hex where it has begun landing men and its unloading is not
  complete.
- **A land unit line** always has `activity` (`ready` or `train`), `experience` (0
  to 4, 2 the standard), `training` (`days` counted, `of_days` a step takes, `up_to`
  the experience training stops at) and `fighting` (`days`, `of_days`); and
  `activity_order`, `rehearsal` (`hex`, `rehearsal`) and `roster` (`task_force`,
  `hex`, and `landing_fatigue`, the fatigue it would gain when its first men land at
  that hex, the engine's own function) when they apply. The text says the experience
  and the activity only when they are not the standard.
- **The briefing's `land_units` part** has, each left out when empty,
  `loading_yesterday` (`change` gained or lost, with `loading_before` and `reason`
  `new_landing_place`, `unit_added` or `spent` for a loss), `rehearsal_yesterday`
  (`gained` with `task_force`, or `lost` with `reason` `new_landing_place`, `landed` or
  `other_landing_place`), `roster_left_yesterday` (`reason` `taken_by_another_force`,
  `dropped_by_order` or `landed`), `preparation_shared_yesterday` (a force that split),
  `experience_days_yesterday` (`source` `training` or `fighting`, the `days` counted)
  and `experience_raised_yesterday` (`by` `training` or `fighting`). `fatigue_yesterday`
  changes have the value `landed`.
- **What the enemy is told.** Nothing of any of it: no loading, rehearsal, roster,
  experience, day counted or order. The only trace of it in the other side's answers
  is what its own reports show (the men ashore, and the fight).

## What a land unit is good at (decision 0098)

Fields added only, so `version` stays 3.

- **A land unit line** (`list_forces`, `get_entity`, the briefing's `units`) always
  has `skills`: `assault`, `holding` and `weight`, and `movement` (`clear`, `jungle`,
  `mountain`, `atoll`), each a whole number 0 to 4, 2 the standard, fixed in play. The
  march (`march.days_counted`) and the briefing's `marches_yesterday` (`counted`) say
  the days counted toward `of_days`, a decimal: a day marched counts as the unit's
  movement factor for the hexes it leaves and enters. The text says the skills on every
  unit line and the days counted beside the days marched.
- **The supply estimate** of the briefing's `supply_by_hex` (`daily_charge_t`) counts
  each unit's men by its weight, the engine's own `daily_charge_t`.
- **What the enemy is told.** Nothing: skills appear only on the side's own unit
  lines, and there is no line for an enemy land unit.

## Units that work, and what they build (decision 0099)

Fields added only, so `version` stays 3.

- **The order** `set_activity` takes `activity` `work` and an optional `job`: `airfield`,
  `port`, `aircraft` or `defences`. With no job the default applies: the airfield of the
  unit's base if it has none, else the port. A unit shown `working` takes only `work`.
- **A land unit line** has `working` (only when true), `job` (only for a unit set to work: the
  job named, else the default one) with `job_is_default` (only when true), and `skills.works`
  (always, 0 to 4, 2 the standard). `activity` can be `work`.
- **A base line** (`list_bases`, `get_entity`, the briefing's `bases`) has `works` only when
  the base is not as a base always was: `airfield` (false until built), `airfield_work` (done)
  with `airfield_work_needed` (only while it has no airfield), `defences` (0 to 100), and
  `port_work_yesterday` and `aircraft_work_yesterday` (what today's cargo and tonight's repairs
  read).
- **The briefing's `land_units`** has `work_yesterday` (the side's `UnitWorked` events: unit,
  base, job, work, total) and `airfields_built_yesterday`, each left out when none.
- **What the enemy is told.** Nothing: no line, event or field of an enemy base or unit shows
  an airfield, work or defences. An enemy base keeps its public line.
