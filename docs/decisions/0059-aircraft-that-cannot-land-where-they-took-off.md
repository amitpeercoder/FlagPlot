# 0059: Aircraft that cannot land where they took off

Status: confirmed. The designer approved the plan on Nov 12, 2026. The figures are placeholders set by the architect, to be set from the record in step 2. It is built in stages (1, 1b, 2 and 3); this record is written before the first and gains an "As built" section with each.

## The gap

Four things the rules get wrong are one thing.

- **A strike's radius is a wall.** A group flies at full strength at 190 miles and not at all at 200. In a recorded game two torpedo groups stayed on deck 10 miles short of a carrier. In the record strikes were launched at 150 to 210 miles with every type together, and Saratoga's torpedo bombers flew against Ryujo, sighted at 198 miles (`docs/research/oob/strike-ranges.md`).
- **Nothing ditches.** At the Eastern Solomons 7 of the 17 American carrier aircraft lost came down in the sea out of fuel. The model can reach the recorded totals only by shooting too many down.
- **Aircraft aboard and aircraft in the air are lost alike when a carrier sinks.** Ryujo's aircraft in the air came down beside her destroyers and four went down with her; the model loses 29 with the ship, and every pilot.
- **A carrier whose flight deck is out takes her aircraft back all the same.** Enterprise's dive bombers went ashore to Henderson Field. The scenarios script that as an arrival.

The record of what became of such aircraft and their crews is in `docs/research/oob/ditching-and-rescue.md`.

## Decisions

### Reach (stage 1)

1. **An air group has a reach, set by order: normal or extended.** Normal is the default and is the rule as it was: a target is in reach if its air distance is within the type's strike radius. It is the "range limit" order of the spec, deferred by decision 0050.
2. **Extended reach is the radius and a share more:** the radius times (1 + `air_combat.toml: extended_reach_share`), cut to a whole mile. A naval strike group on extended reach strikes a contact within that, a base strike group a base within that, and a fighter group on extended reach sends its escorts with a raid within that of its own radius (decision 0030).
3. **An aircraft that flew beyond its radius may run out of fuel on the way home.** After air combat, before wear, each strike or escort sortie whose one-way distance d was beyond the type's radius R rolls each aircraft that came back, on its own, at `ditch_chance_per_radius_beyond` times (d - R) over R, 1 at most. An aircraft that fails is ditched: it is gone from the group for good. A sortie within the radius rolls nothing, so nothing changes for a group on normal reach.
4. **Each type is measured against its own radius.** Fighters, with the shorter radius, are further beyond theirs on the same strike and run the greater risk. No rule singles them out.
5. **A ditched aircraft is not then rolled for wear.** It counts as flown for fatigue and supply, and as destroyed for the enemy's objectives (decision 0054), though no one claims it.

### The crew (stage 1)

6. **Whether the crew is saved depends on where the aircraft comes down.** Where pilots are counted, one draw for each ditched aircraft:

   | Where | Chance the crew is recovered |
   | --- | --- |
   | Out of fuel on the way home | `crew_recovered_short_of_home` |
   | No deck, beside ships of its own side (stage 2) | `crew_recovered_beside_own_force` |
   | No deck, with no ship of its own side in the hex (stage 2) | `crew_recovered_open_sea` |

   A crew not recovered is a pilot lost. An aircraft shot down over the enemy loses its pilot as now (decision 0038).

### The pilot of a patrol shot down over its own hex (stage 1b)

6a. **A fighter shot down on patrol over its own hex no longer always keeps its pilot.** Decision 0038 saved every such pilot. The record of the four weeks over Guadalcanal has 9 to 22 American pilots lost, and the rules gave none. Where pilots are counted, each aircraft a patrol loses over its own hex draws for its pilot: over a base of its side, recovered with chance `crew_recovered_over_own_base`; over ships of its side at sea, with `crew_recovered_beside_own_force`, the figure of decision 6. The designer agreed this addition on Nov 12, 2026.
6b. **It is its own stage,** after stage 1, because it changes results that stage 1 does not: every scenario that counts pilots and loses patrol aircraft (the pilots scenario, the four weeks over Guadalcanal, the full slice, the soak) loses pilots it kept before, and its hand calculation of pilots is redone.

### No deck (stage 2)

7. **Aircraft in the air when their own deck cannot take them look for another place, in this order:** another carrier of their side in the same hex, afloat, with her flight deck not out and room under her aircraft capacity; a base of their side within the type's strike radius of the hex; the sea. A deck cannot take them when the ship has sunk or her flight deck is out when they return.
8. **Aircraft that land elsewhere join the group of their type and side at that place,** by the mechanism that delivers arrivals (decision 0041), with their pilots. Where there is no such group, one is formed there with the next free number. The group they came from keeps what it has aboard.
9. **Aircraft that go into the sea for want of a deck are ditched** as in decision 3, with the crew's chance by decision 6.
10. **Aircraft aboard a ship that sinks are lost with her, with their pilots, as now.** Most of those men were really saved. A saved pilot with no aircraft has nowhere to go in the rules, since replacements arrive only at bases; it waits for a roster of pilots and is recorded here as a known error.

### The scripted arrival goes (stage 3)

11. **Flight 300 is produced by the rule.** The arrival that scripts Enterprise's dive bombers to Henderson Field is taken out of scenario 6 and the full slice where the rule now delivers them, and the test is that they get there.

## Placeholder figures

`extended_reach_share = "0.25"`, `ditch_chance_per_radius_beyond = "2"`, `crew_recovered_short_of_home = "0.7"`, `crew_recovered_beside_own_force = "0.95"`, `crew_recovered_open_sea = "0.5"`, `crew_recovered_over_own_base = "0.6"`. The record behind the last three is 19 crews of 19 recovered beside their own force and 6 to 9 of 13 to 16 in open sea; the first two have examples and no rates. For a pilot over his own base: of 23 American pilots with a stated fate 12 were recovered, a sample that favours those who lived to be written of, and a broader figure not yet read at its source has two in three surviving.

## Why

- **One rule for four gaps.** An aircraft goes out and must come down somewhere. Reach, ditching, a lost deck and a closed deck are the same question asked at different distances.
- **The price is paid on the way home,** as it was, and it is paid in the step that already rolls an aircraft after its sortie.
- **A choice, not a cliff.** The commander decides whether a target is worth aircraft that may not get back.
- **The order is off unless given,** so no scenario changes until someone reaches.

## What it leaves out

- A landing planned at a field beyond the target, to strike further. Decision 7 gives the unplanned landing only.
- Daylight: a strike that comes home in the dark. It needs a clock inside the pulse.
- The carrier closing the range to shorten the flight home (step 3, with the staff).
- Aircraft pushed over the side to clear a full deck: those with no room go on to a field or the sea.
- Pilots aboard a ship that sinks (decision 10).

## What it costs

- A group's aircraft can come to be based somewhere else, and a group can be formed in the middle of a game. Both are new. A new group's number depends on how many groups exist, as a split-off task force's does (the leak decision 0047 records).
- One more field of state for an air group, so the hash version rises and every golden file's hashes are remade.
- When stage 2 lands, the Eastern Solomons and the full slice change, and their hand calculations with them.
- Stage 1b changes the pilots lost in every scenario that counts pilots and loses patrol aircraft, and their hand calculations.

## Rule card

| | |
| --- | --- |
| Reads | An air group's reach, its type's radius and the distance of its sortie; whether its ship is afloat and her flight deck's state; the carriers and bases of its side at and near its hex, and their room; whether a patrol's hex holds a base or ships of its side; the six figures |
| Writes | The group's ready aircraft and pilots; the aircraft and pilots of the group they join, or a new group; a ditching event |

## As built (stage 1)

Built Nov 12, 2026: decisions 1 to 5, and decision 6's first row (the crew of an aircraft out of fuel on the way home). Stages 1b, 2 and 3 are not built. What the code does, and the small choices the decision left open.

- **The order.** `AirGroupOrder::SetReach { reach }`, with `Reach` (`Normal` code 1, `Extended` code 2, in `orders.rs`). In scenario files, saved games and `issue_orders` it is `set_reach` with `reach` "normal" or "extended". A standing order of its own kind: it replaces the reach and nothing else. It is refused for another side's group (not your unit), for a group lost with its ship, and, if extended, where the scenario names no air combat figures (the existing reason "no air combat figures"). Normal is always accepted, so a group can go back. It is accepted for any group, including one that does not strike.
- **State and hash.** `AirGroup.reach` (default normal) and `AirGroup.reach_order`. Hashed last in the group, after `arc_order`; named in `hash::name_every_field`; `hash::STATE_VERSION` is `flagplot-state-v4`. Every daily hash in every golden file changed and nothing else did.
- **The one function.** `AirCombatFigures::reach_nm(radius, reach)`, read through `RefData::reach_nm(type, reach)` and `SideView::reach_nm(group)`. Callers: the naval strike decision and the base strike decision (`decide_strikes`, `decide_base_strike`), the fighters' escorts (`split_fighter_groups`, then `send_escorts`), `measure` air_reach, and the briefing's "out of reach" warning. Extended reach is the radius times (1 + `extended_reach_share`) cut to a whole mile (250 gives 312).
- **Causes.** A group on extended reach cites its reach order as a cause of its `StrikeLaunched`, `BaseStrikeLaunched` and `EscortsSent`. A group on normal reach cites nothing new, so no log of a game with no reach order changed.
- **Running out of fuel.** `World::roll_ditching`, first in the wear phase, before wear. A sortie of kind strike, base strike or escort rolls if it has an aircraft that came back, its group has a position, and its one-way distance (half the sortie's noted distance) is beyond the type's radius. The share beyond the radius, (d - R) over R, is cut to millionths; the chance is the figure times the share, cut, then 1 at most (d 280, R 250: 0.12, then 0.24). Each aircraft that came back is rolled by `damage::roll_shots`. A ditched aircraft leaves `ready`, leaves the sortie's `aircraft` (so wear does not roll it) and keeps `sent` (so fatigue and supply count it flown). A group aboard a ship that sank in the pulse has no position and rolls nothing, as in wear.
- **Sorties are noted when `air_combat.toml` is loaded,** as well as when the wear, fatigue or supply figures are, so the ditching has them to read even where no other upkeep rule is in play. Nothing else reads them without its figures.
- **The stream.** `rng::Draw::Ditch` (code 2, added at the end). Subsystem upkeep, entity the air group, no second entity, the pulse and day of the pulse: one stream for each group and pulse, taken across the group's sorties in the order the wear step uses. For each sortie: the ditching draws (one for each aircraft that came back), then the crew draws (one for each ditched aircraft). It is not the stream wear uses.
- **The crew.** Where the group's pilots are counted, each ditched aircraft draws at `crew_recovered_short_of_home`; each crew not recovered is a pilot lost, never more than the group has. `PilotsLost` is recorded right after the ditching event, caused by it, only when any is lost. A group whose pilots are not counted draws no crew, and the event says so (`crews_recovered` is none).
- **The event.** `EventKind::AircraftDitched { air_group, sortie, distance_nm, radius_nm, aircraft, chance, expected, ditched, crews_recovered, pilots_lost, ready }`, recorded for each sortie that rolled, whether or not any ditched, caused by the events that say the group flew (the launch, or `EscortsSent`). `distance_nm` is one way, unlike `AircraftWorn`'s, which is out and back. Its text reads "air_group:2 strike sortie of 280 nm, 30 beyond its radius of 250: 18 aircraft x 0.24 = 4.32 expected out of fuel, 5 ditched, 3 crews recovered, 2 pilots lost, 13 ready left". Its breakdown lists the aircraft, distance, radius, share beyond the radius, the figure per radius, the chance and the crew figure, and records nothing that changes the result. The group's own side sees it whole; the enemy sees nothing (`side_events.rs`). It counts as destroyed for the enemy's `aircraft_destroyed` and no claim counts it.
- **Figures.** `air_combat.toml` gains `extended_reach_share` ("0.25"), `ditch_chance_per_radius_beyond` ("2") and `crew_recovered_short_of_home` ("0.7"), as decimal strings. Like every key of that file they are required: a file without one is refused when the scenario loads, naming the key. Bounds: the share and the crew chance from 0 to 1; the chance per radius from 0 to 100 (it may be more than 1). The three stage 2 figures are not added.
- **What a player sees.** The order is in the MCP description and in `order.schema.json`. The briefing's air group line gains `reach_nm` only for a group on extended reach; the "out of reach" warning is made against the reach in force and carries `reach_nm` for such a group; the aircraft-lost line gains `ditched` where it is not 0. `measure` air_reach gives `reach` and `reach_nm` and, for each place, `within` (the reach in force), `within_radius` and `ditch_chance`. The replay export has `reach: "extended"` for such a group and the event `aircraft_ditched`; the viewer needed nothing and its check passes on every replay.
- **Scenario 12a** (`data/scenarios/12a_long_reach.toml`) is scenario 5b with the enemy force at (11,0), 18 pilots for the bombing squadron and the order for extended reach. (11,0) is on `placeholder_search.toml` (deep sea from q -2 to 24, r -3 to 3), so no map changed.
- **A mismatch with the hand calculation.** The calculation says that on normal reach no strike is launched in any pulse, whatever is sighted. A sighting report names a neighbouring hex of the true one with chance 0.05 each, and (10,0) is 240 nm from (4,0), within the radius. A report that lands there launches a strike on normal reach, which finds nothing (6 of the first 300 seeds). The test asserts what holds, that no strike is launched at the target hex or beyond 250 nm and nothing ditches, and says so beside the assertion. The same neighbours matter on extended reach: a strike sent to (11,-1) or (10,1), 262 nm, is a sortie of its own with a lower chance, so the table's figures are asserted for the runs whose first strike is at (11,0).
- **Tests of other scenarios that this touched.** The test that every golden file's scenario has no pilots (`scenario_06d1`) excludes 12a, whose squadron has pilots. The test that the full slice records every kind of event (`scenario_07`) lists `AircraftDitched` as a kind the full slice cannot record, since it gives no reach order; scenario 12a records it. The pinned day 1 hash of Eastern Solomons (`scenario_10`) and the hash version in the batch inputs (`sim-cli` `batch` test) are the new ones. No expected number of a hand calculation changed.
- **Not rebuilt.** The recorded games are not regenerated, and none replays now: the hash version and `air_combat.toml` (three new keys) both changed, so `replay` refuses each save at the data file's hash; and the `measure` answers in two of the three transcripts now carry the reach in force, so those two fail to repeat their transcripts. `every_recorded_game_replays_to_its_transcript_and_save` fails until they are played again. Row 35 of `docs/rules/differences.md` (the column `strike_range_nm` holds a radius) is not renamed.

## As built (stage 1b)

Built Nov 12, 2026: decisions 6a and 6b. Stage 2 and 3 are not built.

- **The rule.** Where a group's pilots are counted, each aircraft a patrol loses over its own hex draws for its pilot, in `lose_aircraft`. A group based at a base draws at `air_combat.toml: crew_recovered_over_own_base`; a group aboard a ship, at sea or in port, at `crew_recovered_beside_own_force`. A pilot not recovered is lost, never more than the group has, as `PilotsLost` records it right after the `AircraftLost`, caused by it. Bombers, escorts, aircraft written off, ditched or destroyed on the ground are as before. A group whose pilots are not counted draws nothing. The only loss that is a patrol's is the escorts' fire at it, so that is the only place the draw is made.
- **The stream.** `rng::Draw::PatrolPilot` (code 3, added at the end), in the air combat subsystem, a pair key: entity the patrol group, second entity the group of escorts whose firing roll took the loss. One stream for each firing roll's losses to a patrol group; one draw for each aircraft lost, in the order the losses come off the groups. Two rolls can take losses from one patrol group in a pulse (two raids), and each has its own stream.
- **The event change.** None to a kind or to any text. Where a draw was made, the `AircraftLost` carries a breakdown (only when breakdowns are recorded): the aircraft lost, the chance, and the number of crews recovered as its result, so a reader sees the recoveries even when no pilot was lost. Where no pilot is counted or the loss is not a patrol's, `AircraftLost` is as it was. `PilotsLost` is the existing event.
- **Figures.** `crew_recovered_over_own_base = "0.6"` and `crew_recovered_beside_own_force = "0.95"` in the live `air_combat.toml`, required like its other keys, from 0 to 1. In `data/reference/step1/air_combat.toml` both are "1": every patrol pilot recovered, which is step 1's rule (decision 0038), so no hand calculation of step 1 moves. This is a ruling on the frozen directory: a new figure is frozen at the value that leaves step 1's results as they were where there is one, and otherwise at its first value (its README says so).
- **State and hash.** None added; the hash stays at version 4.
- **Scenario 12b** (`data/scenarios/12b_patrol_pilots.toml`) is scenario 6a with 12 pilots for each of its two fighter groups. On its own seed the fights are 6a's (escorts' hits on the patrol 3, 2, 0; the patrol's hits on the escorts 1, 1, 1), and with the pilot events left out its log is 6a's, at every setting of the figures.
- **The run baselines follow the live figures and changed** where pilots are counted and a patrol loses aircraft: scenario 6 (10 `PilotsLost` events added and nothing else in its events; the final state lines show fewer pilots), 6d1 and 7 (pilot events added, then what fewer pilots do: patrols and raids fly fewer aircraft, fire fewer shots, spend less supply, and the draws after them differ), and the soak's summary. Every other baseline is unchanged. The recorded games were regenerated.

## As built (stage 2)

Built Nov 12, 2026: decisions 7 to 10. Stage 3 (the scripted arrival that goes) was built later the same day; see "As built (stage 3)". The settlements the architect gave for the build, and the choices made beyond them.

- **Which aircraft are in the air.** The pulse already notes what each group flew (`World::sorties`, decision 0035): a group's sorties hold the aircraft that flew and were not shot down, for strike, escort, patrol and search alike. Those are "in the air" until the wear phase takes the sorties. A sinking in the air combat step (`sink_ship`) reads them: the group keeps that many ready aircraft, and one pilot with each, and loses the rest with the ship. A sinking in the movement phase finds no sorties, since the wear phase has taken them, so it loses everything as before. This needed no new state.
- **The deck lost.** `World::deck_losses`, working data of the pulse (not hashed): the sinking (where she sank, the `ShipSunk` event) and a `SystemHit` that puts a flight deck out (the hit event), noted in the air combat step and cleared at the next one. The landing step also asks the state: a deck is lost if her ship has sunk or her flight deck is out when it runs. A degraded deck takes her aircraft.
- **The step.** `World::come_home` in `landing.rs`, between the fuel roll and wear. The fuel roll of stage 1 now also rolls the aircraft in the air of a group whose ship sank (it skipped a group with no position), since an aircraft out of fuel reaches no deck. Where they go, the draws and the new groups are as the rulebook's "Coming home to no deck" says.
- **Choices beyond the settlements.**
  - A type with no strike radius, as the placeholder scout of scenario 12c is, is measured to a base by its search range (the architect's ruling). A scout type that has a strike radius, as the SBD-3 does, is measured by that.
  - A new group is named "<old name> landed at <place>", for example "Placeholder dive bombers landed at ship:6". Its id is one above the highest air group number in the game, so it is stateless and the hash stays at version 4.
  - Landings and the sea are worked for one group at a time in group number order, and the state is updated between groups, so a carrier's room counts the groups that landed before.
  - A sortie goes with its aircraft to the group that has them (split if only part of it lands), so wear, fatigue and supply are worked for the receiving group; the source group keeps `sent` for the aircraft that ditched. A carrier pays no supply, a base does.
  - A group with a lost deck whose ship cannot be placed in a hex (it never happens: a ship that sinks has its hex noted) is left as it is.
  - The aircraft that land at a group that exists raise its ready aircraft and strength; its pilots rise too where they are counted, and the pilots that come are dropped where they are not.
  - `AirGroupLost` and `PilotsLost` after a sinking now count only what was aboard: an `AirGroupLost` can name 0 aircraft, and `PilotsLost` gives the pilots left (it gave 0).
- **Events.** `AircraftLandedElsewhere { from, to, at, aircraft, pilots, formed }` and `AircraftDitchedNoDeck { air_group, hex, aircraft, beside_own_force, chance, crews_recovered, pilots_lost, ready }`. The sibling event keeps stage 1's `AircraftDitched` and its text as they were. Both are caused by the event that closed the deck and the events that say the group flew; the ditching has a breakdown (aircraft, chance, whether a ship was in the hex). Both are seen whole by the group's side and by nobody else. The ditching counts for the enemy's `aircraft_destroyed`, a landing does not.
- **Figures.** `crew_recovered_open_sea = "0.5"` in the live `air_combat.toml`; "1" in `data/reference/step1/air_combat.toml`, because this rule cannot be switched off by a figure and step 1 lost no pilot this way (its frozen `crew_recovered_beside_own_force` of "1" stands for the same rule).
- **Draws.** `rng::Draw::NoDeck` (code 4), upkeep subsystem, keyed on the source group: one stream for each group and pulse, one draw for each aircraft ditched, after the group's landings.
- **Agent side.** The briefing gains `aircraft_landed_elsewhere` (left out when none) and `no_deck` in the aircraft lost line (left out when 0); a formed group appears as a new air group line; the MCP description says what happens. The export has `aircraft_landed_elsewhere` and `aircraft_ditched_no_deck`.
- **Scenario 12c** (`data/scenarios/12c_no_deck.toml`) is built to the hand calculation on a wider copy of the open map of scenario 5b, `placeholder_search_wide.toml` (rows from r -6 to 6): the original has no (9,5), and widening it would add neighbours to the report-error draws of five scenarios. The Japanese arc is 110 to 200, which is 90 degrees as the calculation's chance assumes. Both sides strike again in the afternoon pulse, so the tests read the morning.
- **The tests of the existing scenarios** (5, 5c, 5d, 8, 10, 10b and the others that read them) were taught the rule and not loosened. Their models work each pulse's `AirGroupLost` as the aircraft aboard (ready less in the air, plus damaged), where each group's aircraft in the air go (`tests/common/no_deck.rs`, written from this record and not from the engine), and assert every `AircraftLandedElsewhere` and `AircraftDitchedNoDeck` whole; their conservation sums count the no-deck ditchings and the aircraft that landed from and to a group, a formed group starting at 0; and they gain the property that nothing of the kind happens unless the group's ship sank or lost her deck in that pulse, never at a base and never without an air combat step. In the seed of scenario 5 Ryujo sinks with 6 of group 18 in the air (ditched) and Enterprise's deck goes out with 24, 16 and 16 in the air of groups 1, 2 and 3: 21 land on Saratoga, 3 ditch, and 32 land at Henderson Field. In scenario 5 the Allied `aircraft_destroyed` stays 60 (group 18's 6 that ditched replace 6 that were lost with Ryujo) and the Japanese goes from 5 to 8 (3 of group 1 ditched). In the full slice the Japanese aircraft lost stay 124: 78 shot down, 23 lost aboard Ryujo, 6 ditched, 17 written off.

## As built (stage 3)

Built Nov 12, 2026: decision 11. No rule changed and nothing under `crates/sim-core/src` changed; the stage is scenario data, tests and documents.

- **What went.** The arrival that scripted Enterprise's eleven dive bombers to Henderson Field is out of the full slice (`07_full_slice.toml`): air group 12 "Flight 300", its day-1 order and its day-4 arrival are gone, and the id 12 is retired and never reused (a comment says so where the group stood). In the soak (`08_soak.toml`) only the day-4 arrival is out: group 12 stays, with its day-1 order and its day-75 arrival of 8 aircraft. The recorded full slice game never named group 12 in a request, so its requests stand.
- **What stays.** The arrival stays in scenario 6 (`06_rabaul_guadalcanal.toml`), because that scenario has no carriers and the rule cannot deliver the aircraft there. That is what "where the rule now delivers them" in decision 11 means: it is taken out where the rule delivers and kept where it cannot. The product manager agreed this reading on Nov 12, 2026.
- **What the battle gives, seeds 1 to 60 and 1942** (the full slice, frozen figures and live figures agree landing for landing; only event numbers shift). In 13 of 61 runs aircraft land at Henderson Field, every time because Enterprise's flight deck went out (never Saratoga's, never a sinking), forming air group 121 and in three runs joining it with a second group's. The aircraft landed per run, sorted: 5, 7, 8, 8, 10, 10, 10, 11, 11, 13, 17, 26, 27 (163 in all, median 10). In 5 of the 13 it is on day 4 (24 August, the afternoon) and in 8 on the morning of day 5. In seeds 28 and 37 exactly 11 land on 24 August, the history. In three more runs (7, 8 and 24) Enterprise's deck goes out and everything fits on Saratoga. On the scenario's own seed, 1942, no Allied deck is lost, nothing lands at Henderson Field, and the default run fights the four weeks without those aircraft.
- **The numbers that moved.** The full slice's base 1 stock at the end is 4342.72 t and no longer 4300.955 t: group 12's 11 aircraft cost 41.765 t in the old run, 11 nights at 2.2 t on the aircraft at the base for the day (24.2), 15.565 t on its strike of the afternoon of day 5 and 2 t on its two repairs of the night of day 6. Every other count in the run is as it was. All the daily hashes of the full slice and the soak changed from day 1, because the arrivals still to come are hashed (each of the two has one arrival fewer in its list from day 1). The run baselines of 7 and 8 and the recorded full slice game's transcript and save were remade.
- **Scenario 12d** (`tests/scenario_12d_flight_300.rs`) is the hand calculation of seed 28 on 24 August: after one attack's bombs take Enterprise's deck from working to degraded and to out, 21 fighters of group 101 land on Saratoga, joining group 105 (her room was 90 less 69 = 21), and the 11 scouts of group 103 find no room and land at Henderson Field, 183 miles from (7,-5), forming group 121 "VS-5 landed at base:1", with 11 ready, at rest. It also tests seed 1942 and properties over seeds 1 to 60 and 1942. The model of `tests/common/no_deck.rs` could not be run on every event of the full slice without being loosened (it needs the aircraft in the air, the aircraft aboard every carrier and each ship's hex at that moment, which the full slice does not give), so 12d uses its property that a homecoming follows a lost deck in the same pulse and its rule for the receiving group.
- **Two corrections to the architect's first text of the hand calculation,** made by the architect before the merge: group 101 flies 23, not 24 (24 ready times (100 - 3) / 100 for its 3 fatigue, cut to 23) and loses 2, not 3, to the escorts of group 111, which still leaves 21; and the scouts of group 103 are SBD-3, which have a strike radius of 200 nm as well as a search range of 200 nm, so the base is found by the strike radius. The answer is the same in both.
