# Schemas of the agent interface

One JSON Schema file for each kind of answer the agent interface gives
(decision 0048). They are written by hand and checked, by a validator in the
tests of `crates/sim-agent`, against every answer the scenario tests produce.

Every answer is a JSON object with a top-level `"format": "flagplot-agent"`,
`"version": 11` and a `"kind"` that names it.

| Kind | File | Given by |
| --- | --- | --- |
| `briefing` | `briefing.schema.json` | `briefing` |
| `list_forces` | `list_forces.schema.json` | `list_forces` |
| `list_bases` | `list_bases.schema.json` | `list_bases` |
| `list_contacts` | `list_contacts.schema.json` | `list_contacts` |
| `entity` | `entity.schema.json` | `get_entity` |
| `explain` | `explain.schema.json` | `explain` |
| `measure` | `measure.schema.json` | `measure`: one of six answers, told apart by `question` (`air_distance`, `sea_route`, `passage`, `yard`, `air_reach`, `choices`) |
| `not_known` | `not_known.schema.json` | `get_entity`, `explain` and `measure`, for anything the side does not know of |
| `briefing_text` | `briefing_text.schema.json` | `get_briefing` with `{"text": true}` |
| `issue_orders` | `issue_orders.schema.json` | `issue_orders` (its arguments: `issue_orders_arguments.schema.json`) |
| `command_state` | `command_state.schema.json` | `command_state` (a seat whose side has a staff) |
| `plans` | `plans.schema.json` | `plans` |
| `command` | `command.schema.json` | `set_intent`, `withdraw_intent`, `set_limit`, `set_leave`, `hold`, `release`, `set_interrupt`, `acknowledge`, `approve` (of a plan or of a preview), `cancel_plan`: queued or refused; also `preview` of a command the tool refuses, and `approve` of a void preview |
| `preview` | `preview.schema.json` | `preview` |
| `propose` | `propose.schema.json` | `propose` |
| `plan_order` | `plan_order.schema.json` | `order` (a seat whose side has a staff): the plan of one of the commander's intents as the staff hands it in |
| `staff_orders` | `staff_orders.schema.json` | `staff_orders` |
| `why` | `why.schema.json` | `why` (or `not_known`) |
| `run` | `run.schema.json` | `run` (offered where exactly one seat is held by a player) |
| `report` | `report.schema.json` | `report` |
| `end_day` | `end_day.schema.json` | `end_day` |
| `events` | `events.schema.json` | `events`: the side's own events of a day, or of one part of it |
| `advance` | `advance.schema.json` | `advance` (offered where the scenario lets a seat stop between pulses and exactly one seat is held by a player) |
| `error` | `error.schema.json` | any call refused as a whole |
| `transcript_entry` | `transcript_entry.schema.json` | one line of a game's transcript |

`command_defs.schema.json` is not an answer: it holds what the staff tools'
answers share (a run, a plan, a projection, the five parts of a reason, a
staff's decisions).

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

`version` is 26. A field removed from an answer, or a field whose meaning
changes, raises the version by one. A field added does not change it, so a
reader must ignore fields it does not know. (The schemas here set
`additionalProperties: false` because they describe this version exactly; a
reader of a newer minor change should not.)

Version 2 (decision 0065) took the enemy's units out of the briefing: `forces`
of a fight became `force`, the side's own; `target` left a salvo fired and a
fighter's fire; `raid` left the guns' fire; `air_group` left a raid on a base;
and `by` of an attack on a ship became `"ship"` or `"aircraft"`. Nothing else
changed.

Version 7 (decision 0107, step 4) adds `set_interrupt`, `acknowledge`, `run` and
`report` to the tools of a seat whose side has a staff. `set_interrupt` and
`acknowledge` answer as the other commands do (the `command` kind, whose `tool`
takes both and which gains `interrupt`, the number chosen for a `set_interrupt`
with none given). `run` answers with the `run` kind: `staff_orders` (the
`staff_orders` answer for the first day, worked before it was resolved),
`days_resolved` (`first`, `last`, `count`), `stopped` (`interrupt`, `game_over` or
`days`), `fired` (the firings of the last day resolved), `game_over`, `day`,
`result` and `summary`. `report` answers with the `report` kind: `matters`,
`interrupts`, `fired_last_day` and `summary`. A firing has `day`, `event`,
`acknowledged`, `why` and `words`, and either `interrupt`, `base`, `below_days`
and `figure` (a set interrupt) or `standing` with `plan` or `ship` (a standing
one). `command_state` gains `interrupts` in the shape of the report's
`interrupts`. `staff_orders` and a preview's decisions gain `notices` when the
staff hands one in. The version was raised for the new tools and the two new
answer kinds; no field of an earlier answer changed.

Version 6 (decision 0107, step 3) adds `hold`, `release` and `preview` to the
tools of a seat whose side has a staff, and `preview` to `approve`. `hold` and
`release` answer as the other commands do (the `command` kind). `command_state`
gains `holds`: each hold with its `target` in the side's numbers (`{"ship": n}`,
`{"task_force": n}` or `{"air_group": n, "setting": "search_arc"}`), the `day`,
`made_by` (`command`, or `order` for an order of the commander's to a unit in an
intent's forces limit) and, for an order, its `order` number, and the words.
`preview` answers with the `preview` kind; an `approve` of a preview that is void
answers as a refused `command`, with `reason` saying why. The `command` kind's
`tool` takes `hold`, `release` and `preview`, and gains `preview`. The version was
raised for the new tools and the new answer kind; no field of an earlier answer
changed.

Version 4 (decision 0107, step 3 stage 1) adds the tools of a seat whose side
has a staff: `command_state`, `plans`, `set_intent`, `withdraw_intent`,
`set_limit`, `set_leave`, `approve`, `cancel_plan`, `propose`, `staff_orders`
and `why`, with an answer for each (above). A session made without staffs has
none of them and answers as it did. The version was raised for the new
answers and the new `why` value of `not_known`'s `query`; no field of an
earlier answer changed.

Version 5 (decision 0110, items 1 to 9 and 11) tells a side what it already
holds of its own affairs, and changes four things that were there. In the
briefing the warning `base_without_fuel_with_force` is now one for each force
(`task_force`, with its `least_fuelled` ship and a `home_passage`) and no
longer one for each base with a list of forces (`task_forces`); the day's
`planned_days_of_supply` of a base, and `days_of_supply` of a base line in
`list_bases`, `get_entity` and the briefing, are left out for a base in a hex
where the side holds more than one base, the figure being in `shared_hexes`,
once; an arrival in `arrivals.due` lists every arrival still due, those held
past their day included, and no longer only the next three days; and a loss
line of aircraft written off gives `written_off_by`. Fields added: in the
briefing `free_forces`, `shared_hexes`, `land_units.fatigue_rule`,
`land_units.no_activity_yesterday`, `land_units.landings_yesterday`, an
objective's `open_now`, a base's `formed_by`, an arrival's `at`,
`held_past_day`, `can_be_delivered` and `waits_for`; in a land unit line
`firing_men_now` and `after_another_attack_day`; in an air group line
`awaiting_arrival`; and in `list_bases` `shared_hexes` and `formed_by`.
`landed_yesterday` is kept in the data, a line for each ship, and the briefing's
text gives `landings_yesterday` in its place.

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
  A ship's detail and a ship line have `troops` when she carries men. A ship's detail has `troop_capacity_men`, the men her class can carry, when it can carry any. A ship's detail and a ship line have `fuel_capacity_t`, the tons of fuel her class's tanks hold, so that `fuel_t` reads as a level.
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
  `bases_found_yesterday` (an enemy base a ground report put on the side's map)
  and `found_empty_yesterday` (a hex where the side's own men found no enemy man
  and the estimate was cleared: `place`, `men` and `reported_day` of the estimate
  cleared, `event`; left out when none; decision 0110, item 10).
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
- **A force line** (the same three places) has, for a force stopped at a base of its
  side whose ships afloat lack fuel (decision 0112, 6): `fuel_lacked_t`, the tons they
  lack together; `fuelling_hours`, the hours it would take at that port (each ship's
  lack over the port's fuel berth rate where it has a berth, else its lighter rate; the
  longest ship's), left out when the port gives no rate (no limit: the force fills in the
  first pulse it lies there); and `fuelling_at`, the base. All three are left out when
  the force lacks nothing or is not in port. Fields added: `version` stays 7.
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

## An airfield damaged and mended (decision 0103)

Fields added only, so `version` stays 3.

- **A base line** (`list_bases`, `get_entity`, the briefing's `bases`) has `airfield_damage` only
  while the damage is above 0: `damage` (0 to 100), `share_that_flies` (100 less the damage: of
  every 100 ready aircraft of a group at the base that many may fly), `takes_aircraft` (false at
  or above `refused_at`, the ground figure `airfield_landing_refused_at`, given when the scenario
  has the figures). A rebase to a field that takes none is refused with `airfield_too_damaged`.
- **An air group line** has `kept_on_ground_by_field` (`aircraft`, `damage`) only for a group at
  a base with a damaged airfield when the field keeps at least one crewed aircraft on the ground.
- **The briefing's `bases`** has `airfield_yesterday`, left out when empty, with `damaged`
  (`hits`, `damage` after), `mended` (`land_unit`, `points`, `damage` after) and
  `arrivals_waiting` (`air_group`, `aircraft`, `damage`), from the side's own `AirfieldDamaged`,
  `AirfieldMended` and `ArrivalWaiting` events, each with its `event`.
- **What the enemy is told.** Nothing: an enemy base keeps its public line, and no event of an
  enemy field's damage, mending or held arrivals reaches a side.

## A beaten side falls back (decision 0104)

Fields added only, so `version` stays 3.

- **A land unit line's `march`** has `falling_back` (`from`, the hex of the defeat, and `event`)
  when the march is a falling back; such a march has no `order`, and the unit carries no supply.
- **The briefing's `land_units`** has `fell_back_yesterday` (`land_unit`, `from`, `to`, `men`,
  `fatigue` after), `surrendered_yesterday` (`land_unit`, `place`, `men`) and
  `prisoners_taken_yesterday` (`place`, `men`: enemy men who surrendered, counted as prisoners
  taken), each left out when none and each with its `event`.
- **What the enemy is told.** Of an enemy unit's falling back, nothing. Of its surrender, the hex
  and the men, never which unit.

## Who is engaged, and an attack that fails (decision 0105)

Fields added only, so `version` stays 3.

- **A land unit line** has `when_spent` (`hold` or `fall_back`) when its standing attack order
  carries the instruction, left out when none. **`march.falling_back`** has `reason`: `beaten`
  (decision 0104) or `attack_spent`, a unit that fell back because its attack was spent.
- **The briefing's `land_units`** has `attacks_failed_yesterday` (`place`, `event`: the side's own
  attacks that failed, never the weights of fire), and each `fell_back_yesterday` line has `reason`
  (`beaten` or `attack_spent`).
- **Orders.** `set_posture` takes an optional `when_spent` (`hold` or `fall_back`) with posture
  `attack`; with `defend` it is refused.
- **`measure choices`** has a choice `attack_spent` (asked `{hex}`, answer `{"does": "hold" |
  "fall_back"}`) for a unit on attack.
- **What the enemy is told.** Nothing of a failed attack or of a spent unit's falling back, and
  nothing of a unit's instruction.

## Cover and digging in (decision 0106)

Fields added only, so `version` stays 3. Everything here is of the side's own units and bases;
the enemy is told none of it.

- **A land unit line** has `fieldworks` (what the unit has dug where it stands, 0 to the ground
  figure `fieldworks_limit`) and `cover` (0 to 100: the higher of its fieldworks and the defences
  of its side's lowest-numbered base in its hex, and 0 while it marches or is on attack), each
  left out when 0.
- **A base's `works`** has `grade` of its defences: `improvised`, `prepared` or `fortified`,
  from the ground figures `prepared_from` and `fortified_from`.
- **The briefing's `land_units`** has `fieldworks_yesterday` (`land_unit`, `change`: `dug`,
  `worn` or `lost`, `fieldworks` after, `event`) and `defences_worn_yesterday` (`base`, `place`,
  `hits`, `defences` after, `event`), each left out when none. `fatigue_yesterday` has a change
  `strained`: the unit was under fire and did not rest.
- **What the enemy is told.** Nothing of a side's fieldworks, cover, grades, strain or wear, and
  nothing of what fire did to its men's state.

## Two strike settings of an air group (decision 0114)

Orders and fields added only, so `version` stays 7. Everything here is of the side's own air
groups; the enemy is told none of it.

- **Two order kinds** for an air group: `set_strike_contact_age` (`pulses`, a whole number: the
  oldest contact a naval strike group launches on, from 0 up to one less than the scenario's
  contact drop age) and `set_strike_held_share` (`share`, a decimal string from "0" to "1": the
  share of its aircraft able to fly that a group holds back from a naval or a base strike). A
  value out of range is refused at intake, with the reasons `strike_contact_age_out_of_range` and
  `strike_held_share_out_of_range`.
- **An air group line** has `strike_contact_age` (given only when the group has a setting) and
  `strike_held_share` (given only when above "0").
- **What the enemy is told.** Nothing of either setting or of the orders that set them; a raid's
  aircraft are seen as before.

## The intent tree, holding a place and cover from the air (decisions 0115 and 0117)

`version` is 8: a field changed in meaning (an intent's `limits.risk` is `null` where none is set
on it, read up the tree in `limits_read_up`), so the version rises. Everything here is of the
side's own command state; the enemy is told none of it.

- **Commands.** `set_intent` takes `intent_kind`: `"keep_supplied"` (left out when written, and
  read so where absent), `"cover"` or `"hold"`, the last two with `base` alone. `set_limit` of
  forces takes `air_groups` (written only when not empty) and `source` only where set; it may name
  any intent in force, one the staff formed too. `hold` and `release` with `air_group` and no
  `setting` hold the whole group. The staff's `form_intent` has `parent`, `intent_kind` and
  `base`. New refusals, by the engine's words: made by the staff, wider than its parent, cannot
  stand under, no source, kind cannot change.
- **`command_state`.** Each intent has `parent`, `maker` (`"commander"` or `"staff"`),
  `addressee`, `children` (in force), `depth`, and `limits_read_up`: what binds it up the tree,
  where every limit on the chain binds (the least bold `risk`, the hexes in every `area`,
  `air_groups_binding`, the groups named in every forces limit on the chain), the nearest
  `forces` and `priority`, each `_from` the intent it is read from, and words for what holds where
  nothing is set (`forces_if_unset`, `risk_if_unset`, `risk_set`). A hold intent has `measure`:
  `days_in_force`, `days_met`, `raids`, `raids_fought` and `days_of_supply` (a decimal string, or
  `null` where there is no figure). Supply intents' `floor_days` and interval fields are `null`
  for the other kinds. Intents are listed root by root, each followed by those beneath it.
- **`propose`.** `tree` (`null` for a supply intent): `sub_intents` the staff would form, each
  with its number, parent, words and reason; `groups`, the groups each intent would use with their
  orders; `arcs`, each search arc with `from_deg`, `to_deg`, `range_nm`, `whole_circle` and `how`
  (the reason's working, which names the threats); and `no_orders`.
- **`why`.** `serves`, the reason's serves part in words, and `chain`, the intents above the one
  served, nearest first, each "under intent N (the commander's): ...". A reason's JSON has
  `serves.chain`, a list of `{intent, maker, statement}`, left out when empty.
- **`report`.** `holds`: each hold intent in force with its `intent`, `base` and `measure`.

## Neutralising a base and raiding one (decision 0120)

`version` is 9: an intent's `base` is `null` where the kind names none of the side's own bases
(neutralise), which is a change in meaning, so the version rises. Everything here is of the
side's own command state; the enemy is told none of it.

- **Commands.** `set_intent` takes `intent_kind` `"neutralise"` with `target` (the number of an
  enemy base on the side's map) and no `base`, and `"raid"` with `base` (one of the side's own)
  and `target`. In a saved game and in a staff's hand-in the same keys are written; `form_intent`
  writes `target` for a raid and no `base` for a neutralise. `set_intent` for any other kind
  refuses `target`, and a neutralise refuses `base`, `floor_days` and `interval_days`. A target
  that is not an enemy base on the side's map is refused by the engine ("is not a base on the
  side's map that is not the side's"), in the same words for an enemy base the side may not know
  and for one that does not exist.
- **`command_state`, `plans`, `propose`.** Each intent has `kind` (`"neutralise"` or `"raid"`
  among the others), `base` (`null` for a neutralise) and `target` (the enemy base of a neutralise
  or a raid, `null` for the other kinds). The `tree` of `propose` is given for a neutralise and a
  raid as it is for a hold and a cover. A neutralise intent in force has a `measure` with
  `days_in_force`, `days_met` (the days of them on which none of the side's bases was raided from
  the air), `raids_flown` (the pulses in which a bomber group of the side's launched at the target;
  two groups launching together are one raid), `claimed_on_ground` (the aircraft the side claims to
  have destroyed on the ground at the target's hex from the air: a claim, never below the truth,
  never corrected), `raids_on_own_bases` (the air raids on the side's own bases) and `text`.
- **`report`.** `neutralisations`: each neutralise intent in force with its measure, its `intent`
  and its `target`.
- **A reason's `level`.** `serves` of a reason has `level` (left out where a reason has none, as
  in a game saved before): `"area"` or `{"base": n}`, the addressee of the intent served, the level
  that decided. `why` gives `level` in words ("the area" or "base:1", `null` where none) and
  `serves` ends "decided at the level of ..." before the chain. The replay file's `staff_reason`
  event has a `level` field (`"area"` or `"base:n"`), left out where none.
- **Saved games** are version 5; a game of version 4 or earlier is refused, not converted.

## Denying an area, striking ships, and a strike sized to its target (decisions 0123 and 0118)

`version` is 10 here (11 below): an intent's `base` is `null` for a deny too, which is a change in meaning, so the
version rises. Everything here is of the side's own command state, groups and choices; the enemy is
told none of it.

- **Commands.** `set_intent` takes `intent_kind` `"deny"` with `hex` (`[q, r]`) and `distance` (a
  whole number of hexes, 0 or more) and no `base`, `"strike_ships"` with `base`, and
  `"search_waters"` with `base`, which a commander may not state as a root: the engine refuses it
  ("not a root"). A hex off the map is refused ("is not on the map"). `form_intent` writes `hex`
  and `distance` for a deny. In a saved game and in a staff's hand-in the same keys are written.
  `set_intent` for any other kind refuses `hex` and `distance`.
- **`command_state`, `plans`, `propose`.** Each intent has `kind` (`"deny"`, `"strike_ships"` or
  `"search_waters"` among the others), `base` (`null` for a deny), and `hex` and `distance` (the
  waters of a deny, `null` for the other kinds). A deny intent in force has a `measure`:
  `days_in_force`, `contacts_held` (the distinct contacts the side held in the waters at the end
  of a day in force), `strikes_launched` (by its groups at a base given the task of striking ships
  under the intent, on the days the task stood), `claimed_sunk` and `claimed_hits` (what the
  side's people claim from the air in the waters: a claim, never below the truth, never
  corrected), `days_met` (the days that ended with no contact held in the waters) and `text`.
- **`report`.** `denials`: each deny intent in force with its measure, its `intent`, its `hex` and
  its `distance`.
- **A new air group order**, `set_strike_sizing` (`on`, true or false), a standing order; an air
  group line has `strike_sizing` (`true`, given only when it is on).
- **`measure` `choices`.** The strike answer gives `aircraft`, the aircraft the group would send,
  and, for a group that sizes its strikes, `need` (the aircraft its target needs, `null` where the
  kind has no figure and needs everything) and `sent_before` (the aircraft the side's groups before
  it launched at the contact in the pulse).
- **Saved games** are version 6; a game of version 5 or earlier is refused, not converted.

## Standing for the night and operating carriers (decisions 0124 and 0125)

`version` is 11: an intent's `base` is also `null` for the two new kinds, which is a change in
meaning, and a halt question's choice line gains `pulse`. Everything here is of the side's own
orders, command state and choices; the enemy is told none of it.

- **The route order's `stand_at`.** `{"kind": "set_route", "waypoints": [[1, 0], [3, 0], [1, 0]],
  "stand_at": [[3, 0]]}`: hexes, each one of the waypoints, marked as `act_at` marks them. A force
  that reaches a marked waypoint in the night pulse halts there until the morning pulse. Written
  only when it is not empty, so a route with no such mark is written as it always was. A mark that
  is not a waypoint is refused with the same words as an `act_at` mark.
- **A force line** (`list_forces`, `get_entity`'s `summary`, the briefing's `task_forces`) has
  `stand_at` (the marks of the route not yet passed, as places in hex order; left out when none).
  A halt's `waits_for`, and the briefing's `halted` `waits_for` and `steamed_on` `waited_for`, may
  include `night`; a `steamed_on` line's `reasons` may include `the_night_is_over`.
- **Commands.** `set_intent` takes `intent_kind` `"stand_at"` (stand one of the side's task forces
  at `hex` for the night) and `"operate_carriers"` (operate the carrier force from `hex`), each
  with `task_force` (the side's number of one of its own forces; the same refusal for an enemy force
  and for one that does not exist) and `hex` (`[q, r]`) and no `base`. `form_intent` and the saved
  game write the same keys.
- **`command_state`, `plans`, `propose`.** Each intent has `task_force` (the side's number; `null`
  for the other kinds), and `hex` is also the station of these two kinds. An intent's `addressee`
  is a task force's handle in words for them.
- **A reason's `level`** may be `{"task_force": n}` (`n` the side's number in the interface, the
  umpire's in a saved game); `why` gives it in words as `task_force:n`. The replay file's
  `staff_reason` event writes it as `"task_force:n"`.
- **`measure` `choices`.** The `halt` and `steam_on` lines have `pulse` (the pulse's name) in
  `asked`.
- **Saved games** are version 7; a game of version 6 or earlier is refused, not converted.

## A search flown once a day (decision 0131)

`version` is 14: a new air group order and a new field of a group's line, each of the side's own orders and
units; the enemy is told none of it.

- **A new air group order**, `set_search_once` (`on`, true or false), a standing order: a group on the
  search mission flies its search in the morning pulse only and rests in the afternoon. An air group line
  has `search_once` (`true`, given only when it is on).
- **Saved games** are version 10 (decision 0134: the file's keys are unchanged); a game of version 9 or earlier is refused, not converted.

## What the area command knows of its forces (decision 0136)

`version` is 15: the area seat reads its own units as the area command holds them, so a force or a base the
command last had reported is given as of that report. With command levels off, or no report figures loaded,
nothing is held and every answer is as it was.

- **`as_of`**, in a force's line and a base's line (`list_forces`, `list_bases`, `get_entity` and the
  briefing), such as "the morning of day 1" or "the start": where the area command holds the force or the
  base as last reported and that is not the pulse just ended. Left out otherwise. The briefing's text says
  "(as last reported, of the morning of day 1)" after the place.
- **`radio_silence`** in a force's line, `true`, given only when it is on.
- **A new task force order**, `set_radio_silence` (`on`, true or false): a silent force sends no state
  report and none of its own reports up or sideways until it fights, is attacked or launches a strike.
- **Saved games** are as the engine writes them (decision 0136); the file's keys are unchanged.

## A seat at any command, and what a command holds (decision 0138)

`version` is 16. A seat at the area command is as it was; every new field below is left out where it does not apply.

- **A seat at a task force or a base** (command levels on only): it is given its command's picture, its own
  units exactly (numbered from 1 in the seat), every base's public facts, and the side's events that name its
  own units only. It has the nine tools of play; the command tools are refused, and an order to any unit not
  its command's is refused "not one of your command's units". The briefing's `objectives` has
  `not_given_to_this_seat` there, and `result` is null at the end.
- **The easier setting at the area seat:** the side's own picture and its forces exactly (no `as_of`).
- **`command_state`**, with command levels on: `picture` (the area command's contacts with its own numbers,
  each with `own`) and `forces_held` (each force and base as the area command holds it, with `as_of` and
  `current`).
- **`report`**: `refusals`, the staff orders refused because a force was not as the area command held it
  (decision 0136, 14), the other staff orders refused, and the commands refused.

## Units made ready and a base built up (decision 0141)

`version` is 18. Everything below is of the side's own command state and is left out or `null` where it does not
apply; a game that gives none of it is answered as before.

- **Two intent kinds.** `set_intent` takes `intent_kind` `"ready_landing"` (ready a landing by one of the side's
  task forces at a coast hex), with `task_force` (the side's number), `hex` (`[q, r]`) and an optional `by_day`
  (a whole number from 1: the morning it is to be ready by), and no `base`; and `"build_up"` (build up one of the
  side's bases), with `base`, `wants` (a list, in order, of `"airfield"`, `"mend"`, `"defences"`, `"port"` and
  `"aircraft"`, each at most once) and `defences` (the points, 1 to 100, required where `"defences"` is among the
  wants and refused otherwise). For example `{"intent_kind": "ready_landing", "task_force": 1, "hex": [6, 0],
  "by_day": 6}` and `{"intent_kind": "build_up", "base": 1, "wants": ["airfield", "defences"], "defences": 10}`.
  `form_intent` and the saved game write the same keys.
- **A forces limit names land units.** `set_limit` of forces takes `land_units` (the side's land unit numbers;
  written only when not empty), and `command_state` gives `land_units` in a forces limit and `land_units_binding`
  in `limits_read_up` (the land units named in every forces limit on the chain; `null` where none is set).
- **A land unit can be held.** `hold` and `release` take `land_unit` (the side's number) as the one target; a
  `holds` entry's `target` is `{"land_unit": n}`. It takes no `setting`.
- **`command_state`, `plans`, `propose`.** An intent's kind has `by_day`, `wants` and `defences`, each `null` for
  the other kinds, and `kind` may be `ready_landing` or `build_up`; for a ready_landing, `task_force` and `hex` are
  as for the stand and carrier kinds.
- **Notices.** The notice kinds of `report` and `staff_orders` include `ready`, `late` and `met`.

## Taking a place (decision 0142)

`version` is 20. Everything below is of the side's own command state.

- **A third kind of end.** `set_intent` takes `intent_kind` `"take_place"` (take a hex of land), addressed to the
  area, with exactly one of `hex` (`[q, r]`) and `base` (the number of a base on the side's map that is not the
  side's: the tool writes that base's hex, and refuses with a message naming the base when it is not one), and an
  optional `by_day` (a whole number from 1). It takes no other key. For example `{"intent_kind": "take_place",
  "hex": [6, 0], "by_day": 12}` and `{"intent_kind": "take_place", "base": 7}`. `form_intent` and the saved game
  write `hex` and `by_day` (left out when there is none), never `base`. A hex that is not land is refused by the
  engine: "{hex} is not a hex of land on the map".
- **`command_state`.** An intent's `kind` may be `take_place`; its `hex` is the place and `by_day` its date. A new
  key `phase` on every intent is `"prepare"` (its plan is preparing), `"move"` (released, and no man of the side
  is ashore at the place) or `"land"` (released, and one is) for a take_place, and `null` for every other kind and
  for a plan in any other state. `propose` and `plans` give the kind the same way (no `phase`).
- **The staff's limits.** The staff may hand in `set_limit` of forces on an intent it formed (the land units
  and ships of a ready_landing it forms under a take_place); `staff_orders` and `propose` write it as for any
  other limit command.

## Ashore, and holding a place with men (decisions 0143 and 0144)

`version` is 20. Everything below is of the side's own command state.

- **A hold may name a base that is not the side's.** `set_intent` with `intent_kind` `"hold"` takes a `base` that is
  one of the side's own or an enemy base on the side's map; the engine accepts it, and the staff does nothing under
  it until the base is the side's. The keys are as before.
- **A staff's `form_intent` may be `keep_supplied`.** Under a hold it is written with `intent_kind` `"keep_supplied"`
  (always written for a formed intent), `base` (the hold's), `floor_days` (a whole number, 0 allowed) and, only where
  set, `interval_days`; the formed intent has no `interval_days` of its own. Its forces limit, where it has none,
  is the hold's, read up the tree. `staff_orders`, `propose`, the saved game and a hand-in write the same keys.
- **The `met` notice of a take_place.** In the replay export a `staff_notice` event may carry an optional `intent`
  (the number of the intent; only for the `met` notice of a take_place plan), and its words are "intent:N is met".
  An `intent` is left out of the other notices.

## Bombarding a base (decision 0146)

`version` is 22. Everything below is of the side's own command state.

- **A fourth kind of task of a force.** `set_intent` takes `intent_kind` `"bombard"`, addressed to the task force,
  with `task_force` (the side's own force's number) and `target` (a base on the side's map that is not the side's).
  It takes no other key. For example `{"intent_kind": "bombard", "task_force": 3, "target": 12}`. `form_intent` (the
  staff's) and the saved game write the same two keys.
- **`command_state`, `propose` and `plans`.** An intent's `kind` may be `bombard`; its `task_force` and `target` are
  the force and the base, and its `base`, `hex`, `distance`, `by_day` and `phase` are `null`.

## The force assembled by the staff (decision 0149)

`version` is 23. Everything below is of the side's own command state.

- **A take_place's forces limit may leave out what the staff is to choose.** `set_limit` of forces on a take_place
  may name any of `task_forces`, `land_units` and `source`, or none, and a take_place may have no forces limit at
  all: the engine no longer refuses one with no source ("no source" is still the refusal for a keep_supplied).
  What the commander names stands; the staff chooses the rest and keeps it as the forces limit of the ready_landing
  it forms (`set_limit` in `staff_orders` and `propose`), naming only what the commander did not.
- **`command_state`.** What binds an intent on a take_place's chain (`limits_read_up`, `land_units_binding` and the
  nearest `forces`) is read as it binds: for the take_place, the commander's task forces, land units and source
  where he named them and the staff's choice where he did not; for an intent under it, what its own limit does not
  name of the three is the take_place's, so read.

## Controlling an island (decision 0151)

`version` is 24. Everything below is of the side's own command state.

- **A fifth kind of end.** `set_intent` takes `intent_kind` `"control_island"`, addressed to the area, with `island`
  (the number of an island the scenario names, required) and an optional `by_day` (a whole number from 1: each
  take_place the staff forms under it is formed with that date). It takes no other key. For example
  `{"intent_kind": "control_island", "island": 1, "by_day": 30}`. `form_intent` and the saved game write `island`
  and `by_day` (left out when there is none). An island the scenario does not name is refused by the engine:
  "island {number} is not an island of the scenario".
- **`command_state`.** An intent's `kind` may be `control_island`. A new key `island` on every intent is the
  island's number for a control_island and `null` for every other kind (`hex`, `base`, `target`, `distance`,
  `task_force`, `phase`, `floor_days`, `interval_days`, `wants` and `defences` are `null` for it; `by_day` is its
  date). A control_island intent has a `measure`: `days_in_force`, `days_controlled` (the days on which every
  named place was the side's at the end of the day, with its supply above its reserves and its days of supply at
  least the floor of the supply intent under the place's hold, 5 where there is none), `places` (in the island's
  order, each `hex`, `base` (null for a bare coast hex), `ours` and `days_of_supply`, a decimal string or `null`)
  and `text`, in the words "controlled on N of M days in force". `propose` and `plans` give the kind and `island`
  the same way.
- **`report`.** A new key `islands` lists each control_island intent in force with the same measure and its
  `intent` and `island` numbers; the summary has a sentence for each.

## Orders inside a day: `events` and `advance` (decision 0156)

`version` is 25. Nothing changes in a game whose scenario leaves `orders_inside_a_day` off, beside the new tool `events`.

- **`events`** (every seat). Arguments `day` (a whole number from 1; by default the day the clock is in: the open day
  at 0600 or 1200, else the last day resolved) and `pulse` (`night`, `morning`, `afternoon`, or `end` for the events
  of the day that carry no pulse: those of its dusk and of its end). The answer has `day`, `pulse` (the part asked
  for, or `null`), `begun` (false for a day not yet begun, whose list is empty), `count`, `events` and `summary`. Each
  event has `number` (the number `explain` takes), `day`, `pulse`, `event_kind`, `text` and `entities`, exactly as
  `explain` gives the same event as its node 0. It reads the side's own events and nothing else.
- **`advance`** (offered where the scenario has `orders_inside_a_day = true` and exactly one seat is a player's).
  Arguments `pulses` (a whole number, at least 1), `stop_on` (a list of `contact`, `attack`, `loss`, `strike`,
  `landing`; none by default) and `reason`. It resolves up to `pulses` pulses one at a time: from dusk it first does
  what `end_day` does; before a morning or afternoon pulse it hands in the orders given at 0600 or 1200; after the
  afternoon it closes the day at once. The clock so stops only at dusk, at 0600 or at 1200. The answer has
  `pulses_resolved` (each `day` and `pulse`), `stop_on`, `stopped` (`pulses`, `told`, `interrupt` or `game_over`),
  `told` (the things of `stop_on` the side was told in the last pulse, each with the numbers of its events; for the
  afternoon, with the events of the day's end), `clock` (`day`, the day whose pulse is next; `at`, `dusk`, `0600`
  or `1200`; `next_pulse`, `null` once the game is over; `days_resolved`; `words`), `fired` (the firings of the last
  day closed in the call, as `run` gives them), `game_over`, `result` and `summary`. The event kinds behind each
  thing: `contact`, `report_received` that made a new contact (not a refresh); `attack`, `air_attack_against`,
  `base_attacked_against`, `shore_bombardment_against`, `salvo_against`, `engagement_begun`; `loss`, `ship_sunk`, `ship_lost_with_base`, and
  `system_hit` of the side's ship whose propulsion or flight deck goes out; `strike`, `strike_launched` and
  `base_strike_launched`; `landing`, `men_landed` and `ground_fight` (a fight ashore, which a side is told of only
  where it has men; it comes at the end of the day).
- **`issue_orders` at 0600 and 1200.** Each order is checked at once and only the seven kinds may be given (a task
  force's route, speed, readiness and rules of engagement, an air group's mission and search arc, a land unit's
  posture); any other is refused with the engine's words "this kind of order is given at dusk: ..." and a hint. The
  answer adds `for_pulse` (`morning` or `afternoon`) and each queued result's `for` ("the morning pulse of day 1");
  `for_day` is the open day and `queued` counts the orders queued inside the day. `clear` empties those.
- **`get_briefing` at 0600 and 1200** is refused: the briefing is made at dusk; `events` gives the day so far.
- **`inside_day`.** At 0600 or 1200, `command_state`, `plans`, `staff_orders`, `propose`, `preview` and a queued
  command (`set_intent` and the other command tools) carry a note `inside_day`: they are worked on the state as it
  stands, as if the open day were closed now, before the rest of the day, its fight ashore and its end are resolved;
  a command is queued for the next dusk, taken in at the start of the day after, and checked again then. Their
  `for_day` is that day. A passage or yard `measure` carries a note `inside_day` saying it is worked as if the force
  left at 1800, the start of the day's night, which has passed. The other read tools answer on the side's view as it
  stands, with no note; `report`'s `day` is the open day.

## A take that cannot bring enough (decision 0159)

The version is unchanged, 25. The notice kinds of `report`, `staff_orders` and a preview's decisions include
`cannot_bring_enough`: a "take a place" at balanced that can bring the men it wants on no day in reach. It is handed in
once for its plan, and its reason gives the men wanted, the men to hand and any unit coming with its day.
`set_interrupt` and `acknowledge` take `standing` `"cannot_bring_enough"`, with `plan`: a fourth standing interrupt,
on by default and armed apart for each plan.

## A plan told to wait (decision 0159, rule C)

The version is unchanged, 25: fields and values are added and none changes its meaning. `set_limit` takes three
more `limit` values, each only on a `take_place` or `control_island` of the commander's: `not_before` with
`from_day`; `wait_for` with one of `task_force` and `land_unit`, in the side's numbers (a number naming none of the
side's units is refused in the words a forces limit naming it gets); and `after` with `other_intent` (one that would close a ring of waits is refused as one naming its own intent is). Each with its
other key left out clears the limit. In `command_state` and `plans`, each intent's `limits` gains `not_before` (a
day), `wait_for` (`{"task_force": n}` or `{"land_unit": n}`, `command_defs.schema.json#/$defs/awaited`) and `after`
(an intent's number), each null where not set on the intent; `limits_read_up` gains `not_before` and
`not_before_from` (the latest day on the chain and the intent it is set on, both null where none is set), and
`wait_for` and `after`, lists nearest first, each entry with `from`, the intent it is set on (`after`'s entries name
the intent waited for as `intent`). A take held by one of them gives a reason that names what it waits for, and one
that can never be met as it stands hands in `cannot_meet` once.

## How much must be known (decision 0159, rule D, items 12 and 13)

The version is unchanged, 25: fields and values are added and none changes its meaning. `set_limit` takes the
`limit` value `knowledge`, only on a `take_place` or `control_island` of the commander's, with `knowledge` (`none`,
`count` or `recent_count`) and, with `recent_count` alone, `within_days` (3 where left out; 0 is refused by the game as
a recent count of no days, below 0 by the tool); `knowledge` left out clears the limit. Each intent's `limits` gains
`knowledge` (`command_defs.schema.json#/$defs/knowledge`: `setting` and `within_days`, null for none and count), null
where not set on the intent; `limits_read_up` gains `knowledge` and `knowledge_from`, the nearest on the chain and the
intent it is set on, both null where none is set and the risk decides. Every plan in `plans` and `command_state` gains
`need_to_know` (`command_defs.schema.json#/$defs/need_to_know`): for a take whose plan is preparing this morning,
what must be known before its landing goes, worked from the side's view as the staff works it and stored nowhere:
`intent`, `place`, `asked` (`setting`, `within_days` and `from`, the intent the limit is set on, null where the risk
sets it), `held` (the side's estimate: `men`, `came_from`, `day`, `age_days` and `could_be`, the `lowest` and
`highest` it could be in truth from its grade's error range, `highest` null where the range's low end is 0; null for a
count that was told or where the figures give no range; `held` is null where nothing is held), `looking` (the air
group in the side's numbers and `since_day`, or null), `moved_to_look` (decision 0159, item 14: the carrier force in
the side's numbers that the staff moves or has moved to look at the place while the need is not met, and its `hex`, or
null), `met` and `text`, the need in one line. It is null for every other plan, a released one too.

## The plan as an order: the tool `order` (decision 0159, rule E, items 15 to 17)

`version` is 26. The tool `order` (a seat whose side has a staff; argument `intent`, the number of one of the commander's
intents in force) gives the answer `plan_order` (`plan_order.schema.json`); it changes nothing and is worked on the
state as the coming day will find it, with the commands queued taken in. An intent not in force, the staff's own or
not the side's is refused in the words `propose` uses. The answer has `for_day`, `as_at`, `intent`, `mission` (the
intent in words), `plan` (`number`, `state`), `phases` (in order: `name`, one of `prepare`, `move`, `land`, `wait`,
`hold`; `stage`, `done`, `under_way` or `to_come`; `ends`, one of `released`, `first_man_ashore`,
`place_is_the_sides`, `base_is_the_sides` and `never`; `place`, a place, for the first two; `base`, a handle, for
`base_is_the_sides`; `text`, a line; a take has prepare, move and land, a hold wait and hold or hold alone, the other
kinds none), `waits_for` (each with `kind`, one of `men`, `leave`, `day`, `force`, `intent` and `knowledge`; `met`;
`expected_day`, null where the staff can work none; the key of its kind, `wanted` and `to_hand`, `leave`, `day`,
`force` (as `awaited`), `intent` or `knowledge` (as `knowledge`), the other keys null; and `text`), `men` (`wanted`
and `to_hand`, or null), `expected_ashore` (a day, or null where the staff cannot say or the men are ashore),
`need_to_know` (as in `plans`, or null), `tasks` (the staff's intents under it: `intent`, `kind`, `statement`, `phase`)
and `text`, the brief as an array of lines. Nothing else is in it: no reasons.
