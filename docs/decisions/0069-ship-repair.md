# 0069: Ship repair

Status: confirmed. The designer approved the plan on Nov 13, 2026, with the answers below. Nothing is built. The hand calculation is written before any code is delegated.

Nov 13, 2026, at the close of step 2: **this is next.** The designer decided ("Reported only; repair first"; `docs/step2-spec.md`, "Decided by the designer", 16) that ship repair and "out of the campaign" are built next and the objectives scale set from the record, before step 3 begins; land units and basic ground combat follow those.

Nov 13, 2026: not part of step 2's close, ruled by the coordinating architect on the designer's delegation. It adds an order and a briefing line, which the step 2 spec's scope leaves out, and it is in none of the lines of "Done means". It follows step 2, with land units and ground combat, before step 3; so does "out of the campaign" on the objectives scale, which reads it.

## The gap

No rule ever lowers a ship's damage. A system once out stays out, in port or at sea, for the rest of the game (decision 0052, item 11). In one battle that does not matter. Over a campaign it does: fleets only wear down, a damaged ship has nowhere to go, and a commander has no reason to pull one out of the line.

The record is in `docs/research/oob/step2-record.md`, section 3, from `ship_facts.csv` rows 613 to 769 and `place_facts.csv`. It shows two kinds of repair, by where it was done.

- **Forward, at a base with a repair ship.** Divers, patches and ship's force. South Dakota and Enterprise were back 16 days after the bombs of 26 October with no yard. No forward base in the slice could dock a cruiser or do turret work, and the Japanese had the same split between Rabaul, Truk and the home yards.
- **In a yard, far away.** Enterprise after 24 August was away 53 days, Saratoga 71, Shokaku 125, Boise 160, Chester about 330.

The spec does not ask for this before tuning (`docs/step2-spec.md`, "Rule work that comes before tuning"): it matters to a campaign, not to one battle's losses.

## Decisions

### Repair at a forward base

1. **A base may have a ship repair figure:** points of damage it mends each day, 0 unless the scenario gives one. It stands for a repair ship or tender there. With no such base in a scenario, nothing changes.
2. **Repair is the staff's, as refuelling in port is.** At the start of each day, each force stopped at a base of its own side is repaired from that base's figure, forces in number order and ships in number order within a force. Nobody orders it. It uses the test refuelling already uses for "stopped at a base" (decision 0045).
3. **A forward base mends light damage and no more.** A point of repair takes off a point of fire damage, or a point of flotation damage if the ship's flotation damage is below a figure, `forward_repair_max_flotation`. A system that is degraded is put back to working at a cost in points. Flotation damage at or above the figure, and a system that is out, need a yard.

### Repair in a yard

4. **The yards are off the map.** A scenario may mark a base of a side as a port for the yard, with the days of the passage there and back.
5. **A force stopped at such a base can be ordered to the yard.** The whole force goes. It leaves the map, and returns to the same base on a stated day with every ship fully repaired, refuelled and rearmed. To send only the damaged ships, detach them first, with the order that exists.
6. **The days away are worked out, not drawn,** by one function, for each ship: the days of the passage, plus a figure for each point of flotation damage, plus a figure for each system degraded and a larger one for each system out. The force is away for the days of its slowest ship to mend. There is no random draw.
7. **The player is told the day of return before he sends her.** `measure` answers it, and the order's acceptance states it. The briefing lists forces away and their day of return.
8. **A force that would return after the scenario ends does not return.** Its ships are afloat and out of the game.

### What a side knows

9. A side sees its own repairs, departures and returns whole. The enemy sees nothing of them: a force that leaves for a yard is a contact that ages and is dropped, as any force that steams out of sight.

## A small worked case

Placeholder figures, for the example only: a base that mends 4 points a day; `forward_repair_max_flotation` 30; a degraded system costs 10 points; a yard passage of 20 days there and back, 1 day for each point of flotation damage, 10 for a system degraded and 30 for a system out.

| Ship | Damage | At the forward base | If sent to the yard |
| --- | --- | --- | --- |
| A | Fire 12, flotation 20, fire control degraded | 42 points: ready on the 11th day | 20 + 20 + 10 = 50 days |
| B | Flotation 60, flight deck out | Fire only; the rest needs a yard | 20 + 60 + 30 = 110 days |

A is of the order of South Dakota's 16 days in October. B is near Shokaku's 125. The figures themselves are set later from the nineteen ships in the research.

## Why

- **Two places, two speeds.** That is the pattern of the record, and it gives the commander a real choice: keep a damaged ship fighting, patch her forward for a fortnight, or lose her for months and get her back whole.
- **The yard is a day on a calendar, not a place.** The yards were thousands of miles off the map. Steaming there would add map, fuel and escort rules for a passage nobody can attack.
- **Worked out, not drawn.** The staff planner of step 3, and the player, must be able to count on a ship's return. The record's spread is wide, but it follows the damage, which the function reads.
- **It reuses what exists:** the "stopped at a base" test of refuelling, a daily allowance for a base as for aircraft repair (decision 0035), the detach order, and the arrival of units on a day (decision 0041).
- **Off unless a scenario asks.** No base has a repair figure and none is a port for the yard until a scenario says so, so no scenario changes.

## What it leaves out

- **A ship that must be patched before she can make the passage.** New Orleans needed eleven days at Tulagi to be seaworthy. Any ship that can reach the port may be sent.
- **Towing, scuttling and damage control at sea** (decision 0016).
- **Repair that costs the base supply,** as aircraft repair does. It can be added by the same figure when supply for ships exists.
- **A ship sent alone from a force.** Forming and merging forces by order is not built (decision 0052, item 7), so a force goes and returns as it is.
- **Attacks on a repair ship,** and a repair ship that moves between bases. The figure belongs to the base.
- **Refits that improve a ship.** She returns as her class has her.

## What it does for "out of the campaign"

The opening list has a second item: points on the objectives scale for a ship that must leave for repair. It should not read whether the enemy sent her to a yard, since a side could then deny the points by keeping a wreck in the line. It can read the function of decision 6: a ship is out of the campaign if the yard would keep her longer than some number of days. That is its own decision record. This one only provides the function.

## What it costs

- A new field of state for a base (its repair figure and its yard passage) and for a task force (away, and its day of return). The hash version rises and the golden files' hashes are remade.
- One new order, two new kinds of event for the yard and one for a repair, each with its line in `side_events.rs`.
- A handful of placeholder figures in `surface_combat.toml`, to be set from the research.
- The rulebook's "No repair" section is rewritten, with chapter 1 (the order), chapter 2 (the daily step) and the glossary.

## Rule card

| | |
| --- | --- |
| Reads | A base's repair figure and yard passage; whether a force is stopped at a base of its side; each ship's fire damage, flotation damage and systems; the repair figures; the day and the scenario's last day |
| Writes | A ship's fire damage, flotation damage and systems; a force's away state and day of return; for a returning force, its ships' fuel and ammunition |

## When to build it

Not before the three battles are tuned. Ship repair changes no battle's result, and tuning is what the step 2 spec asks for first.

## Answered by the designer

Approved on Nov 13, 2026, each as recommended.

1. **The yard is a day of return, off the map.** No passage is steamed. Decisions 4 and 5.
2. **The days away are worked out, with no random draw.** Decision 6.
3. **Forward repair is the staff's, with no order.** Decision 2.
4. **A whole force goes to the yard and returns together,** until forces can be formed by order. Decision 5.
5. **It is built after the three battles are tuned.**

## As planned for the build

Nov 13, 2026. Written by the team lead for the coordinating architect's check, on `origin/main` at state version 8. Nothing here is built, and nothing stands until the architect has read the whole section and ruled on the choices at its end. The decided text above is not changed. Where this section picks something the decisions did not say, it is listed again under "Choices for the architect", with a recommendation. The hand calculation is in `tests/scenario_20a_ship_repair.rs`.

### Two named quantities: days of repair and days away

The yard function of decision 6 is one pure function in `sim-core` (proposed: `repair::days_of_repair(ship, figures) -> u32`), used by the order, by its acceptance, by `measure` and by decision 0088's count. Nothing else works out yard days.

- **Days of repair** of one ship, from her damage alone, with no passage:
  `ceil( yard_days_per_flotation_point x flotation_damage + yard_days_per_system_degraded x D + yard_days_per_system_out x O )`,
  where D and O are the counts of her systems degraded and out (of the five named systems; a system her class lacks is always working and counts for nothing). Each product is one fixed-point multiplication, cut to six places as every product is; the three are added; the sum is rounded up to a whole day (a part of a day is a day). Fire damage adds nothing: the decided text names no fire term, and the yard mends fire with the rest.
- **Days away** of a force: the base's `yard_passage_days` plus the largest days of repair among its ships afloat.
- **Day of return**: the day the order is taken in plus the days away. A force sent on day D with 34 days away is away on days D to D + 33 and is back at the start of day D + 34.

Decision 0088 reads the first quantity, so the departure event carries each ship's days of repair and the passage separately (below).

### State (state version 9)

| Where | Field | Type and meaning | Default |
| --- | --- | --- | --- |
| `Base` | `ship_repair_points_per_day` | `Fixed`, at least 0. The points of damage the base mends in a day, shared by every force of its side stopped there | 0 |
| `Base` | `yard_passage_days` | `Option<u32>`. `Some(n)` makes the base a port for the yard, with a passage there and back of n days; n is at least 1 | `None` |
| `Ship` | `system_repair_points` | `Fixed`, from 0 to below `forward_repair_system_points`. Points already put into the first degraded system, in system order, that a forward base has not finished | 0 |
| `TaskForce` | `yard_trip` | `Option<YardTrip>`: `YardTrip { base: BaseId, return_day: u32, departure: EventId }`. `Some` while the force is away | `None` |
| `ShipLocation` | new variant `Yard(TaskForceId)` | A ship away at the yard with the force named. Added at the end of the enum | |

**How a force is away** (a choice, below; this is the recommendation). When the force leaves, its ships are taken out of its `ships` set and each ship's location becomes `ShipLocation::Yard(force)`; the force itself stays in `task_forces`, at its base's hex, stopped, with no ships, and with `yard_trip` set. On return the ships whose location is `Yard(force)` go back into the set, in number order. The reason: every place that matches on a ship's location then fails to compile until the builder decides what a ship at the yard does there (movement, surface fights, search, strikes, landing, the air group's hex, invariants, export, the batch record, the agent's queries), and nearly every place that walks the forces already skips a force with no ships (refuelling, movement, the alarm, alert from belief, fatigue, the search's targets, the meeting test). The builder lists every remaining iteration over `task_forces` and says for each why a force away is skipped; the survey found these to check: `movement.rs` 479 and 583, `stragglers.rs` 51, `refuel.rs` 26, `cargo.rs` 39, `search.rs` 206, `airstrike.rs` 1935, `surface.rs` 353, 436, 512 and 589, `side_events.rs` 500, `view.rs` 128 and 377, `invariants.rs` 346.

**`hash_world`**, each new field in its place, and each named in `name_every_field` in the same change:

- `Base`: after `fuel_t` (the last field today), `ship_repair_points_per_day` as `i64` raw, then `yard_passage_days` as a tag byte (0 for `None`, 1 for `Some`) and, for `Some`, the `u32`.
- `Ship`: after `experience` (the last today), `system_repair_points` as `i64` raw. The location match gains `ShipLocation::Yard(id) => { h.u8(4); h.u32(id.raw()); }`: codes 1 to 3 are not renumbered.
- `TaskForce`: after `fatigue` (the last today), `yard_trip` as a tag byte and, for `Some`, `base`, `return_day`, `departure` (the event's raw number), in that order.
- `STATE_VERSION` becomes `b"flagplot-state-v9"`, with the history line: "version 9 adds a base's `ship_repair_points_per_day` and `yard_passage_days` after its `fuel_t`, a ship's `system_repair_points` after its `experience`, the ship location `Yard` (code 4), and a task force's `yard_trip` after its `fatigue` (decision 0069)". Every pinned copy of the version string moves with it (`crates/sim-cli/src/save.rs`, `crates/sim-cli/src/validate/mod.rs`, `crates/sim-cli/tests/batch.rs`, the `batch_before` file, `docs/engineering/determinism.md`), and the golden files are remade (decision 0057). No event log of an existing scenario changes.

No new field holds the day's remaining allowance of a base: the forward repair step uses it and drops it within the step, so it is not state.

### The order

`TaskForceOrder::SendToYard`, added at the end of the enum, with no arguments: the port is the base the force is stopped at. Spelt `send_to_yard` in a scenario's `[[order]]` and in the agent's `issue_orders` (`{"kind": "send_to_yard"}`).

Checked in this order; the first that fails is the reason:

| Test | Reason | New or reused |
| --- | --- | --- |
| The force is the issuing side's | `NotYourUnit` | reused |
| The force is not away, or it returns on or before the order's day | `AwayAtYard` | new, added at the end of `RejectReason`: "The task force is away at a yard until its day of return." |
| It has a ship afloat | `NoShipAfloat` | reused |
| It is stopped: not under way and with no `toward_hex` (the refuelling test, decision 0045; out of fuel at a hex centre counts as stopped) | `UnderWay` | reused; its doc widens from "A load order needs a task force that is stopped" to "The order needs a task force that is stopped" |
| A base of its side is in its hex | `NoOwnBaseHere` | reused |
| One of those bases is a port for the yard | `NotAYardPort` | new, at the end: "No base of the side in the task force's hex is a port for the yard." |

`AwayAtYard` is also the first test (after `NotYourUnit`) of every other task force order, so a force away takes no orders. "Returns on or before the order's day" lets a player give orders, the evening before, to a force that will be back at the start of the day the orders are taken in: `check_order` checks them for `truth.day + 1` and intake for the day itself, so the one condition serves both with no special case.

Where two bases of the side in the hex are ports for the yard, the lowest-numbered is the port (as refuelling takes from the lowest-numbered base).

**Applying it**, at intake, after `OrderAccepted`: work out each ship's days of repair and the force's days away from the state at that moment; take the ships out of the force (location `Yard`); set the force `Idle`, drop its route, `route_order`, `cargo_order`, `toward_hex`, `progress_nm`, `leg_nm`; set `alerted` false and `alerted_by` none; set `yard_trip`; record `ForceLeftForYard` as the order's follow-up event (the `ShipsDetached` precedent). Every other field of the force (mission, speed and its order, readiness and its order, rules of engagement, home port, fatigue) is kept as it is while away.

**Acceptance states the day.** The follow-up event carries the day of return. `issue_orders` answers a queued `send_to_yard` with `"day_of_return"` (or `"returns": false` if after the last day) worked by the same function on the state as it stands; since nothing that runs before intake on the next day changes a ship's damage (forward repair runs after intake, below), the two agree.

### Events

| Kind | Fields | Causes | `side_events.rs` |
| --- | --- | --- | --- |
| `ShipRepaired` | `ship`, `task_force`, `base`, `points` (spent on her this day), `fire_damage`, `flotation_damage` (after), `restored: Vec<ShipSystem>` (systems put back to working this day, in system order), `system_repair_points` (after) | The force's `arrival_event` if it has one, as `ShipRefuelled` cites it; none for a force that began the scenario there | Ships group: `whole_if(own_ship(ship))`. In the "false" list of `whole_may_have_hidden_causes` |
| `ForceLeftForYard` | `task_force`, `base`, `passage_days`, `ships: Vec<(ShipId, u32)>` (each ship afloat and her days of repair, in number order), `days_away`, `return_day` | The order | Task force group: `whole_if(own_force(task_force))`. "False" list |
| `ForceReturnedFromYard` | `task_force`, `base`, `ships: Vec<ShipId>` | The `ForceLeftForYard` event (held in `yard_trip.departure`) | `whole_if(own_force(task_force))`. "False" list |

The enemy is shown none of the three, by decision 9: `Seen::Nothing` for a side that does not own the unit. Each needs its arms in `EventKind::entities()`, `Display`, `seen()`, `whole_may_have_hidden_causes()`, `crates/sim-cli/src/export/events.rs` (`ship_repaired`, `force_left_for_yard`, `force_returned_from_yard`) and the viewer's check.

**Breakdowns** (recorded only when breakdowns are on; they change nothing):

- `ShipRepaired`: `allowance_before`, `fire_mended`, `flotation_mended`, `system_points_spent`, `allowance_after`, and `forward_repair_max_flotation` when her flotation was at or above it (to say why none was mended).
- `ForceLeftForYard`: `passage_days`; then for each ship in number order `flotation_damage`, `systems_degraded`, `systems_out`, `days_of_repair`; then `days_away`. The follow-up of `take_in_order` is recorded with no breakdown today: the builder adds one for this event only.

`Display` texts, proposed:
- `ship:3 repaired 6 points at base:1: fire 0, flotation 4, fire control working`
- `task_force:2 left base:2 for the yard: ship:3 30 days, ship:4 12 days, passage 4, back on day 36`
- `task_force:2 back at base:2 from the yard, ship:3 and ship:4 repaired, refuelled and rearmed`

### Where in the day

The start of each day becomes, in `World::resolve_day`, before the first pulse:

1. **Returns from the yard** (new, `return_from_yard(day)`): each force whose `yard_trip.return_day` is the day, in force number order.
2. **Refuelling in port** (decision 0045, unchanged).
3. **Orders taken in** (unchanged). Departures for the yard happen here, at each order's acceptance.
4. **Forward repair** (new, `repair_in_port(day)`).

It is not a pulse phase: like refuelling it runs once a day outside the pulses, and the night sequence is unchanged.

Why this order:
- Returns first, so a returning force is at its base for the day's refuelling (it takes nothing, being full) and for its orders, which `AwayAtYard` let through the evening before.
- Forward repair after the orders, unlike refuelling, for two reasons. The days a yard order states, by `measure` and by the queued answer, are then exactly the days the force leaves with: nothing between the evening and the order's acceptance changes damage. And repair takes the day in port: a force ordered to sail that morning (its route makes it under way at intake) is not patched that day, while refuelling, which takes hours, still fills it before it sails. A force that is given an order that is refused, or an order that does not move it, is still stopped and is repaired.

### Forward repair, exactly

For each base, the allowance of the day is its `ship_repair_points_per_day`. Forces are taken in number order (scenario forces, then forces made in play, by their numbers), skipping a force away, a force under way or with a `toward_hex`, and a force with no ship afloat. A force is served at its hex, from the bases of its side there: each ship takes from the lowest-numbered such base whose allowance left is above 0, as refuelling takes fuel; if there is none the force gets nothing. Ships within a force are taken in number order. One ship spends points in this order, each step taking `min(allowance left, what it needs)`:

1. **Fire damage**, down toward 0.
2. **Flotation damage**, down toward 0, only if her flotation damage is below `forward_repair_max_flotation` at the start of this step. Flotation at or above the figure is not touched.
3. **Degraded systems, in system order** (`ShipSystem::ALL`: propulsion, fire control, main battery, torpedo mounts, flight deck), only the first degraded one at a time. The points still needed for it are `forward_repair_system_points − system_repair_points`. If the allowance left covers them, the system becomes working, the allowance falls by them and `system_repair_points` returns to 0, and the next degraded system is taken; otherwise all the allowance left goes into `system_repair_points` and the ship is done. A system that is out is passed over and never mended forward.

Why system order: it is the order the hit bands already use (chapter 4, Where a hit lands), so a player who knows one knows the other; it puts propulsion first, which is what a commander patching a ship to steam would do first; a carrier has only propulsion and flight deck, so the order costs her nothing. Points banked in `system_repair_points` belong to the first degraded system in order at the time they are spent; if that system is then hit to out, the banked points go to the next degraded one.

A ship that takes no point records nothing. A ship that takes points records one `ShipRepaired`, after her points are spent and before the next ship is served. Forward repair never raises damage, never mends a system that is out, and leaves fuel, ammunition and torpedoes as they are.

### What a returning force gets

At the start of its day of return, before refuelling, for each ship whose location is `Yard(force)`, in number order:

| Field | Becomes |
| --- | --- |
| `flotation_damage`, `fire_damage` | 0 |
| Each of the five systems | working |
| `system_repair_points` | 0 |
| `fuel_t` | her class's fuel capacity |
| `gun_rounds` | `main_guns x gun_rounds_per_gun` of her class, the load the scenario loader gives a ship with no `gun_rounds` key |
| `torpedoes_ready` | true if her class has torpedo tubes, as at the scenario's start |
| `torpedoes_one_side_fired` (decision 0083) | false |
| `supply_t`, `cargo_fuel_t` (cargo aboard) | unchanged |
| `experience` | unchanged |
| `location` | `TaskForce(force)`, and she goes back into the force's `ships` |

And the force:

| Field | Becomes |
| --- | --- |
| `yard_trip` | `None` |
| `status` | `Idle`; `from_hex` is still the port's hex; `toward_hex` none, `progress_nm` and `leg_nm` 0, no route, no `route_order`, no `cargo_order` (all cleared when it left) |
| `arrival_event` | the `ForceReturnedFromYard` event, so the next refuelling and forward repair cite it; `arrived_at_hours` none |
| `fatigue` | 0 |
| `alerted`, `alerted_by` | false, none |
| `readiness` and `readiness_order`, `mission`, `speed` and `speed_order`, `rules_of_engagement` and `rules_order`, `home_port` | as they were when it left (standing orders) |

Then `ForceReturnedFromYard` is recorded. A force whose day of return is after the scenario's last day is never reached by step 1 of a day, so it does not return (decision 8) and needs no rule in the engine: its ships end the game afloat at the yard, with the damage they left with. The engine does not hold the last day; the agent's session does, and uses it for the briefing and `measure`.

### Carriers and their air groups

Proposed, the least: an air group aboard a ship at the yard goes and returns with her, aircraft, damaged aircraft and pilots as they were. While away it flies nothing, wears nothing, is not repaired, gets no replacements or arrivals, and tires and rests not at all. `air_group_hex` returns `None` for a ship at the yard; the builder checks that no site reads `None` as "lost" (today a group aboard a sunk ship is lost by its own event, `AirGroupLost`, and must stay so). An arrival or replacement due to a group while it is away is not delivered and not kept (as decision 0041, 6, for a group lost with its ship). A choice, below.

### What the enemy knows

Nothing is shown to the enemy. Its contact of the force is not refreshed, since nothing can sight a force with no ships, and it ages and is dropped by the existing rule (`contact_drop_age_pulses`, 5 in the frozen and live search figures), as for any force that steams out of sight. No new belief code: `BeliefState::apply_report` is not called by any part of this rule. The enemy's events for a game in which a force goes to the yard are those of the same game with the force left stopped in port, as long as nothing of the enemy's would have sighted it there (the hand calculation asserts this).

### The briefing and `measure`

- **Briefing, own side only:** a part "Away at the yard" after the task forces: for each force away, its name, the port, its ships and "back on day N" or "does not return before the game ends (day L is the last)". The force's line in the task forces part reads "away at the yard". Under each base, a line "Ships repaired yesterday" from that day's `ShipRepaired` events, one per ship: the points, and what she has left (fire, flotation, systems still degraded or out). A force that returned is said so under the base. The enemy's briefing gains nothing.
- **`measure`:** a fifth question, `{"question": "yard", "task_force": "task_force:2"}`. The answer: whether a `send_to_yard` order given now would be refused and why (by `check_order`), the port and its `passage_days`, for each ship afloat her flotation damage, systems degraded and out and her days of repair, the `days_away`, the day she would leave (tomorrow), the `return_day`, and `returns_before_end: true/false` with the last day. `measure` takes the last day from the session as `briefing_for_game` does; with none it omits `returns_before_end`. It also answers for a force already away: its port and day of return.

### Scenario file keys and the loader's refusals

| Entry | Key | Refused when |
| --- | --- | --- |
| `[[base]]` | `ship_repair_points_per_day` (whole number or decimal string) | negative; a TOML float |
| `[[base]]` | `yard_passage_days` (whole number) | 0, negative, or not a whole number |
| `[[ship]]` | `flotation_damage` (whole number or decimal string), new | negative, 100 or more (a ship sunk is not given this way), a float |
| `[[ship]]` | `fire_damage`, new | negative, above 100, a float |
| `[[order]]` | `kind = "send_to_yard"`, no other keys | any other key |

The two `[[ship]]` keys are new: today a scenario can start a ship with systems damaged but with no flotation or fire damage, and the rule's own scenario needs both. They are a choice, below. A ship's `system_repair_points` and a force's `yard_trip` cannot be given by a scenario: every game starts with no work in hand and no force away.

### Figures

Five figures, in `data/reference/surface_combat.toml` as the decided text says, required like every key there, with the same values in the frozen copy `data/reference/step1/surface_combat.toml` (no base of a step 1 scenario has a repair figure or a yard port, so any value leaves step 1 as it was; the README's rule then gives the first value):

| Figure | Placeholder | Loader refuses |
| --- | --- | --- |
| `forward_repair_max_flotation` | 10 | below 0 or above 100 |
| `forward_repair_system_points` | 10 | 0 or below |
| `yard_days_per_flotation_point` | 2 | below 0 |
| `yard_days_per_system_degraded` | 5 | below 0 |
| `yard_days_per_system_out` | 30 | below 0, or below `yard_days_per_system_degraded` (the decided text: "a larger one for each system out") |

Each is commented in the file as a placeholder with what it rests on (next section). A base's daily points and a port's passage are the scenario's, not reference figures, as a base's aircraft repairs and fuel are. A new file, `repair.toml`, was considered and is not recommended: it would add a scenario key, a `--reference` entry and a frozen copy for five numbers that sit with the damage they mend.

### Randomness and what must not move

There is no random draw. No `rng::Draw` value is added, and no existing stream is opened, read or keyed differently: the rule reads state and figures only. A run's events are the same for every seed. With no base given a repair figure or a yard passage, every scenario's event log is byte for byte as before; only the state hashes move, by the new fields and the version.

### Invariants

Added to `invariants::check`: a ship whose location is `Yard(f)` has `f` with `yard_trip` set, and is in no force's `ships`; a force with `yard_trip` set has no ships; `system_repair_points` is from 0 to below `forward_repair_system_points`, and is 0 on a ship with no degraded system; a base's `ship_repair_points_per_day` is not negative and its `yard_passage_days` is not 0.

### The rulebook and glossary, in the same pull request

`docs/rules/04-surface-combat-and-damage.md`, "No repair", becomes "Repair" (forward and yard, the two quantities, the figures); chapter 1 gets the order, the reasons, the day's start (returns, refuelling, orders, forward repair), the three events and who sees them; chapter 2 gets the base keys and the force away; chapter 5 a line on air groups aboard a ship at the yard; `docs/rules/differences.md` row 11 if it names the start of the day; the glossary gets "days of repair", "days away", "day of return", "forward repair", "port for the yard", "away at the yard". `docs/commands.md` names the new question of `measure`.

### Choices for the architect

Each is something the decided text did not settle, with the recommendation in one line.

1. **How a force is away.** Recommended: its ships move to a new location `Yard(force)` and out of the force's set, the force stays in place with no ships, so the compiler finds every site that reads a ship's location (the alternative, a flag on a force that keeps its ships, leaves the builder to find every site by hand).
2. **Forward repair after the day's orders, not before them as refuelling is.** Recommended: after, so the stated day of return is exact and a force that sails that morning is not patched.
3. **Partial work on a degraded system is kept** (`system_repair_points` on the ship). Recommended: keep it, as the decided worked case needs it (42 points at 4 a day, ready on the 11th day); without it a base below the system's cost could never mend one.
4. **Order of a ship's points:** fire, flotation, then degraded systems in system order. Recommended as written; the system order is the hit bands' order.
5. **The daily allowance is the base's, shared by every force there,** taken base by base in base order where two share a hex. Recommended, as refuelling shares a base's fuel.
6. **Fire damage adds no days of repair.** Recommended, as the decided formula has no fire term.
7. **Days of repair rounded up to a whole day; days away at least 1** (the loader refuses a passage of 0). Recommended.
8. **What a returning ship keeps:** cargo and experience kept; damage, systems, fuel, guns, torpedoes and the one-side flag restored. Recommended.
9. **What a returning force keeps:** standing orders (mission, speed, readiness, rules, home port) kept; route, cargo order, alert and fatigue cleared. Recommended.
10. **Air groups aboard a carrier at the yard** go and return as they were, flying nothing, with no repair, replacement or arrival while away (an arrival due then is lost). Recommended, the least; the alternative is that their damaged aircraft are repaired at the yard too, which is one more line on return.
11. **Orders to a force away are refused** (`AwayAtYard`) unless it is back by the order's day. Recommended.
12. **Two new `[[ship]]` keys,** `flotation_damage` and `fire_damage`, so a scenario can start a damaged ship. Recommended: the rule's scenario needs them and nothing else changes.
13. **A ship lying in port (`ShipLocation::Port`) at a repair base is not repaired:** the decided text names forces only. Recommended: leave it so; no scenario of the slice starts a damaged ship in port.
14. **The figures in `surface_combat.toml`,** not a new file. Recommended, as the decided text says.
15. **`measure` answers a new question, `yard`,** rather than a field on `passage`. Recommended: it is asked of a force in port, not of a place.
16. **`yard_days_per_system_out` may not be below `yard_days_per_system_degraded`.** Recommended, from the decided "a larger one".

## The yard function's figures from the record

Nov 13, 2026, for the architect's check. Rows `ship_facts.csv` 613 to 769 and `place_facts.csv`, read for this. Nothing from `references/` is used.

### What the rows give, ship by ship

Days are from the date of damage; "before the yard" is to the yard's start date in the rows (it holds the passage there and any forward stop); "in the yard" is the yard's start to its end or to the ship's sailing; "total" is the record's days away, which (the rows say so) ends when she leaves the yard or rejoins, so it holds the passage there but mostly not the passage back.

| Ship | Damage in the rows | Before the yard | In the yard | Total | Rows |
| --- | --- | --- | --- | --- | --- |
| Enterprise, 24 Aug | Three bombs; flight deck unable to land aircraft for about 65 minutes; steering out 38 minutes | 17 (Tongatabu for fuel only) | 36 (Pearl, 10 Sep to 16 Oct) | 53 | 613 to 617 |
| Enterprise, 26 Oct | Two bombs; a 30 by 60 ft section of the flight deck buckled; after elevator room flooded | forward only | none (12 days at Noumea, sailed unfinished) | 16 | 679 to 682 |
| South Dakota, 26 Oct | One bomb on No. 1 turret; collision on 30 Oct | forward only | none (Vestal, 5 days of work) | 16 | 647 to 649 |
| Chitose, 24 Aug | Near miss: port engine room flooded, 30 degree list, towed | Truk, then Yokosuka | short, to 14 Sep | 25 | 618 to 622, 725 to 728 |
| San Juan, 26 Oct | One bomb through the stern; the sources disagree on how bad | about 19 | 10 (Sydney) | 29 | 683 to 687 |
| South Dakota, 6 Sep | Grounding; bottom split for about 150 ft | 6 (Tongatabu, divers) | 30 (to Pearl and back to sea) | 36 | 643 to 646 |
| North Carolina, 15 Sep | One submarine torpedo; 5.6 degree list; held 26 kn | 15 | 48 (Pearl) | 63 | 634 to 637 |
| Saratoga, 31 Aug | One submarine torpedo; a fireroom flooded, power out 9 hours, towed | 21 (6 days of it at Tongatabu) | 50 (Pearl) | 71 | 628 to 633 |
| Zuiho, 26 Oct | One or two bombs aft; could not recover aircraft | 12 | 39 (Sasebo dock) | 89 (back at Truk) | 712 to 715 |
| Ralph Talbot, 9 Aug | Shells; steering lost; on two boilers | 33 | 61 (Mare Island) | 94 | 757 to 759 |
| Chikuma, 26 Oct | Three or four bombs (the rows disagree) | 12 | 112 (Kure, with refit) | 124 | 716 to 719 |
| Shokaku, 26 Oct | Four to six bombs; flight deck buckled and burst; kept 28 kn | 11 | 114 (Yokosuka) | 125, a lower bound | 708 to 711 |
| Aoba, 11 Oct | Up to forty shells; No. 3 turret destroyed, bridge wrecked, four boilers off line | 11 | 116 (Kure, with refit) | 127 | 720 to 724 |
| Jintsu, 25 Aug | One bomb forward; forward magazines flooded | 44 (35 with Akashi at Truk) | 93 (Kure) | 137 | 623 to 627, 729 to 733 |
| Yugiri, 28 Aug | One bomb near the bridge; "medium damage" | 39 (31 with Akashi) | 101 (Kure) | 140 | 734 to 737 |
| Farenholt, 11 Oct | Three shells; torpedo tubes out; stack and boiler damage | not dated | not dated | 143 (Wikipedia only) | 669 to 673 |
| Chicago, 9 Aug | One torpedo; bow damaged, about 10 kn; still fought | 65 (Noumea, Sydney) | to early Jan | about 150 | 750 to 756 |
| Boise, 11 Oct | Shells; the forward three turrets out by a magazine fire (one source: one turret wrecked) | 39 | 121 (Philadelphia) | 160 | 665 to 668 |
| Chester, 20 Oct | One submarine torpedo amidships; machinery and hull | 3 + 57 (Espiritu, Sydney) | Norfolk, not dated | about 330, an upper bound | 674 to 678 |

Forward repairs with no yard, in the same rows: Chokai, Aoba and Kinugasa after Savo Island, 8 days (761 to 769); Hagikaze 20 (747 to 749); Shirakumo 33 (738 to 741); Akizuki 6 (742 to 744); Tenryu 18 from the start of work (745, 746). Five more ships (Portland, Salt Lake City, Pensacola, New Orleans, Minneapolis) return by month only, after the slice.

### How many rows the function can read

**None, as the rows stand.** The function reads flotation points and the state of five systems. No source gives flotation points; they are the model's own scale. What the rows do give, and the mapping assumed to read them, each an **assumption**:

- **A1, flotation from the hits.** A ship's flotation damage is the hits the rows count x the model's damage per hit (`100 x weapon damage / class toughness`, chapter 4) x the share that lands on flotation: 0.75 for a bomb or shell (`gun_hit_flotation`, its expected value, not a draw), 1 for a torpedo. Weapon damage and toughness are the derived figures in `data/reference` (the D3A's bomb 41.666666, the SBD's 75.6; Yorktown class 792, Lexington 1600, North Carolina 1499.36, Shokaku 1027, Tone 448.6, Northampton 362, Fubuki 83.6, Atlanta 263.6).
- **A2, a submarine's torpedo** has no figure in the data. Taken as the range from the B5N's aircraft torpedo, 81.64, to the largest ship torpedo, 196.
- **A3, systems from the words.** "Dead in the water", "power out" is propulsion out; "flight deck buckled and burst", "could not recover aircraft" is flight deck out; a flight deck back within the hour, "steering failed", "held to 10 knots", "boilers off line" is degraded; "a turret destroyed" of several is main battery degraded; "bridge wrecked" is fire control out. A system put right within hours at sea is taken as degraded, since the model has no repair at sea.

On those assumptions eleven of the nineteen can be read in part: both Enterprise cases, Saratoga, North Carolina, Shokaku, Chikuma, Chicago, Yugiri, San Juan, Chester, and South Dakota in October (her class, South Dakota, has no derived figures; read with North Carolina's toughness it is a 2-point hit). Eight cannot: Aoba (forty shell hits would sink her in the model several times over), Boise (no hit count in the rows), Farenholt (no derived class), Ralph Talbot (no hit count), Zuiho, Jintsu (no derived class), Chitose (a near miss, which the model has no way to read), South Dakota's grounding (not a weapon).

### What the placeholder figures give

`yard_days_per_flotation_point` 2, `yard_days_per_system_degraded` 5, `yard_days_per_system_out` 30, and, for the cases, a passage of 20 days for a US port to Pearl Harbor, which matches the 15 to 21 days the rows show before the yard. Days of repair against the rows' days in the yard:

| Ship | Flotation by A1 | Systems by A3 | Days of repair | In the yard (rows) | Days away with 20 | Total (rows) |
| --- | --- | --- | --- | --- | --- | --- |
| Enterprise, 24 Aug | 3 x 5.260942 x 0.75 = 11.837119 | 2 degraded (flight deck, propulsion) | ceil(23.674238 + 10) = 34 | 36 | 54 | 53 |
| Saratoga | 5.1025 to 12.25 | propulsion out | ceil(10.205 + 30) = 41 to ceil(24.5 + 30) = 55 | 50 | 61 to 75 | 71 |
| North Carolina | 5.444989 to 13.072244 | none | 11 to 27 | 48 | 31 to 47 | 63 |
| Shokaku | 4 to 6 x 7.361246 x 0.75 = 22.083738 to 33.125607 | flight deck out | 75 to 97 | 114 | 95 to 117 | 125 |
| Chikuma | 3 to 4 x 16.852429 x 0.75 = 37.917965 to 50.557287 | none read | 76 to 102 | 112 | 96 to 122 | 124 |
| Chicago | 41.436464 to 54.143646 (a ship torpedo, 150 to 196) | propulsion degraded | 88 to 114 | not dated | 108 to 134 | about 150 |
| Yugiri | 67.822966 | none read | 136 | 101 | 156 | 140 |
| San Juan | 11.855082 | none read | 24 | 10 | 44 | 29 |
| Chester | 22.552486 to 54.143646 | propulsion degraded | 51 to 114 | not dated | 71 to 134 | about 330, an upper bound |

The forward cases, with `forward_repair_max_flotation` 10, `forward_repair_system_points` 10 and a base mending 2 points a day: Enterprise on 26 October has 2 x 5.260942 = 10.521884 points, 7.891413 on flotation and 2.630471 on fire, and her flight deck degraded. Flotation is below 10, so a forward base can mend all of it: 10.521884 + 10 = 20.521884 points, 11 days at 2 a day; the rows have 12 days in port and her sailing with the work unfinished, 16 days in all. South Dakota's 2-point hit is two days' work; the rows have five. Enterprise on 24 August, at 11.837119, is above 10 and needs the yard, as she went to one.

### What the rows size and what they cannot

- **`yard_days_per_system_out`, 30: borne on, by one case.** Saratoga, with a torpedo's flotation and propulsion out, spent 50 days in the yard; at 2 a point her flotation accounts for 10 to 25, leaving 25 to 40 for the system. Shokaku's flight deck out is consistent with it. It is still a placeholder with a note, not a fit: one clean case.
- **`yard_days_per_flotation_point`, 2: a placeholder, not sized.** The rows disagree with each other under A1. North Carolina's one torpedo gives less model flotation than Enterprise's three bombs, yet she spent longer in the yard; the model does not tell damage below the waterline from damage above it, and no single figure fits both. Destroyers (Yugiri) and light cruisers (San Juan) come out too long, because one bomb on a small toughness is many points. Japanese yards (Kure, Yokosuka) kept ships about twice as long as Pearl Harbor for like damage, with refits; one set of figures for both sides cannot follow that, and the port's passage, which is the scenario's, is the only place a difference could go.
- **`yard_days_per_system_degraded`, 5: a placeholder, not sized.** Only Enterprise in August reads it, and her flotation by A1 carries most of her days.
- **`forward_repair_max_flotation` 10, `forward_repair_system_points` 10, and a base's 2 points a day: placeholders matched to one pair of cases,** Enterprise in October (forward) and in August (yard). Vestal's "963 jobs on 58 ships in 60 days" (`place_facts.csv` row 165) cannot be turned into model points. A base's figure is the scenario's to set.

To size them properly the record would need the model's damage per ship from the batches set beside these ships' real damage, battle by battle: the first of the next piece of work, setting the scale from the record (decision 0088).
