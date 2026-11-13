# 0073: The Japanese search at the Eastern Solomons, from the record

Status: settled by the architect, on the designer's delegation of Nov 13, 2026 ("Go ahead and decide all matters that need decisions"). The orders were revised on Nov 13, 2026: the carriers' attack units stay on naval strike, and Chikuma's floatplanes search to the recorded 330 miles once decision 0075 gives the type a search radius of its own. Scenario data, not a rule: no rule changed and nothing under `crates/*/src` changed. It replaces item 8 of decision 0031 for the Japanese side.

## The gap

The spec and the opening list both say it: the Japanese search at the Eastern Solomons was a placeholder arc. Decision 0031, item 8, had the floatplanes of Chikuma and Tone search 110 to 200 degrees to 253 miles, "a quarter-circle toward the south-east", because no source then gave the Japanese sectors. The Japanese records read since (`docs/research/oob/japanese-records-2026-11-12.md`, and the notes at the end of `docs/research/oob/air-search.md`) give the search orders and what was flown.

## What the record says

Japanese times are Tokyo time, two hours behind the scenario's clock (UTC+11): Ryujo's attack is 13:50 in Japanese quotes and 16:20 on the American clock, which is UTC+11.5. All rows are `data/oob/engagement_events.csv`. Rows 1198 to 1208 are from the research of Nov 13, 2026 (pull request 182).

| Who | When (Tokyo; scenario) | What | Rows |
| --- | --- | --- | --- |
| Nagumo's orders of the 23rd | 16:25 and after | The first search: the 11th Battleship Division's floatplanes, 11 aircraft, 300 miles, lines 7 degrees apart, base line A. The second: Zuikaku, 12 aircraft, base line A minus 90, 250 miles, 10 degrees apart. The third: Ryujo, 7 aircraft, 250 miles, only if the floatplanes cannot be used; signal 15 at 23:45 gives that task to Shokaku | 1201, 1202, 1203, 1204 |
| Nagumo's orders of the 24th | 00:05 to 03:46 | Base line A is 150 degrees, launch 05:00; the second group's left half shortened to 200 miles; the fourth search group cancelled; the 11th Battleship Division to search from 09:00, method 8 including line 1, left half, six aircraft, each line 30 miles shorter (another reading has the lines at 300, the page cut), base line 150 | 303, 304, 305, 435, 437, 1205 |
| Shokaku's attack unit | 04:15; 06:15 | 7 Type 97s, one to each line, 250 miles, no lateral leg. One sighted a submarine on the 150 degree line | 366, 367 |
| Zuikaku's attack unit | 04:35; 06:35 | 12 Type 97s, a sector "?0 to 110 degrees" to 230 miles (250 struck out) and 0 to 60 to 200 | 368 |
| The carriers, as Nagumo reported | 05:00; 07:00 | 7 attack planes searched 250 miles to the south and saw no enemy. The morning search saw nothing but two submarines | 369, 306, 440 |
| The vanguard's floatplanes | 09:00 to 09:40; 11:00 to 11:40 | Four searched south and south-east, to 330 miles at most: Chikuma's two at 09:00, Kumano's (two; Suzuya's catapult was out of order) at 09:20 to 09:40. Kumano's two came back at 15:00 having seen nothing | 370, 438, 439, 307, 308 |
| Chikuma's No. 2 | 12:05 to 12:10; 14:05 | "Large enemy force sighted", then pursued by fighters and lost. Nagumo put it at 153 degrees and about 260 miles from the carriers; the vanguard at 153 degrees 240 miles from Hiei | 441, 442, 443, 338, 339 |
| Contact and night planes | 13:18 to 13:50 and after; 15:18 | Hiei's and Chikuma's, one each, to keep contact; later Chikuma's No. 1 reports the fleet; Nagara's and Kirishima's for the night | 446, 342, 1206, 453, 452, 455 |
| Tone, with Ryujo | from 02:00 | A detachment under the 8th Cruiser Division commander by Mobile Force operations signal no. 3, to support the reinforcement and attack Guadalcanal; separated at 02:00. Her division's report has her lead the attack on the air base and rejoin the vanguard in the afternoon, and enters no aircraft launch by Tone. The detachment's signal no. 1 of 05:50 is recorded without its text | 1198, 1199, 327, 147, 1207, 1208 (328) |
| A website | 11:00 local | "Chikuma and Tone of the Vanguard Force launch seven floatplanes" | 172 |

The strikes of the two fleet carriers were dive bombers and fighters only (344, 371, 374, 376 to 379). Their attack planes flew the dawn search and struck nothing.

## Decisions

1. **The vanguard's floatplanes (group 17, Chikuma's five E13A1) search 120 to 180 degrees.** The range is the recorded 330 miles (370) once decision 0075 gives the type a search radius of its own; until then it is 253, the E13A1's derived radius, the most an order may name. The arc is a reading, not a row: the vanguard's own report says "south and south-east" (370), Nagumo's base line was 150 (303, 437), and six lines ten degrees apart (the spacing of the vanguard's order for the next day, 394) make sixty degrees about it. The arc holds the bearing of the sighting, 153 (441). Search method 8 and "left half" are defined on no page read (`air-search.md`, notes of Nov 13, 2026), so the order itself does not settle the arc.
2. **The fleet carriers' attack units (groups 13 and 16) stay on naval strike,** as decision 0063 has them. On the day they flew the dawn search, found nothing, and in the afternoon were held armed and neither searched nor struck. The day is the smallest command cycle, so a group gets the one order whose effect over the day is the record's: an order to search would add an afternoon search at 250 miles that did not happen and that sights the Americans; naval strike leaves them on deck, since nothing is in their strike radius. Rows 366, 368 and 369 are kept as the record of the dawn search, which the scenario does not fly. See "What the rules cannot express".
3. **Tone's floatplanes (group 20) have no order.** This rests on rows, not only on a gap: the detachment was set up to attack Guadalcanal (1198, 1199); her division's report has her do that and rejoin the vanguard, and enters no aircraft launch by her on the 24th (1207); the vanguard's contact aircraft were Hiei's and Chikuma's (1206, 446); the third dawn search, once Ryujo's, was given to Shokaku (1203, 1204). Row 172, a website that has Tone launch searchers with the vanguard, is contradicted by these. The text of the detachment's signal no. 1 (1208) is still not read.
4. **The American search stays as it was.** It is from rows, not a placeholder: Enterprise's scouting squadron, 290 to 090 (the afternoon search, 158; 157 has 270 to 090) to 200 miles (the dawn search, 144 and 146). It is a blend of the two searches the day had, which one standing order must be.

### Where the rows disagree or are unclear

- **How many attack planes searched.** Shokaku's report has 7 and Zuikaku's 12; Nagumo's report (369) has 7 "to the south". The orders name a second search group, Zuikaku's, on a base line 90 degrees left of A (1202), and a third, given to Shokaku (1204). Settled: both searched; 369 reports the southern search only. It no longer moves any order (decision 2).
- **Zuikaku's first bearing is illegible** ("?0"). With A at 150 her base line was 60 (1202), and her sectors read 60 to 110 and 0 to 60 (368). Kept as the record; no order reads it.
- **"Left half" in the vanguard's order (437, 1205).** If it means the lines east of the base line the arc would be about 100 to 150. That is not taken, for the words of 370; the sighting at 153 (441) is an estimated position three degrees from 150 and weak evidence either way.
- **Four flew, six were ordered** (370, 437). Group 17 has five, an establishment of June 1942 (decision 0031, item 7). The five stand for the vanguard's four to six; the scenario does not hold the other cruisers' and the battleships' floatplanes.

## What the rules cannot express

The search rules (`docs/rules/03-search-contacts-and-claims.md`, Air search) give an air group one standing arc, flown in every daylight pulse from the pulse's start, with every aircraft the group can fly. So:

- **A search flown in one pulse only. This is a known limit of the rules, and the orders of groups 13 and 16 (113 and 116 in the full slice and the soak) are not the record.** The attack units searched at dawn (366, 368, 369) and found nothing; the scenario cannot say so, because a search cannot be ordered for one pulse, and an order to search for the day adds an afternoon search the record does not have. They stand on naval strike instead, which over the day has the record's effect: they fly nothing. The scenario files say so beside those orders, so that a later reader does not take the order for the record (spec, "Tuning", 8).
- **A launch hour.** The vanguard's search (11:00 to 11:40) flies in the morning pulse, from the 06:00 positions, when no enemy force is within 280 miles; its sighting at 14:05 falls in the afternoon pulse, which the group also flies, from the 12:00 positions. Nearest expression: the search mission for the whole day.
- **Part of a group.** A group searches whole ("Not built: a group sending only part of itself on search").
- **Range beyond the derived search range.** Until decision 0075 is built an order may name no more than 253 miles for the E13A1 against the 330 flown. Decision 0075 gives every type a search radius of its own; the order then names the distance flown.
- **Contact planes and night searches** (446, 1206, 452, 455) are not separate: a later sighting of the same force refreshes the contact in the afternoon search, and there is no night search.
- **A report not received.** The strike leader missed the floatplanes' reports (266). There is no lost report (decision 0031, "Not modelled").

## Who follows

The full slice (07) and the soak (08) copy scenario 5's orders on day 4, with group numbers plus 100, as they did for decision 0063. They follow: group 117 takes the new arc, 120 has no order, and 113 and 116 stay on naval strike, with the same comment. The sub-scenarios 05a to 05d have their own invented searches and are not copies.

## Rule card

No rule. These are scenario inputs.

## The hand-calculation tests run on frozen copies of the scenarios

Settled by the architect on Nov 13, 2026, an engineering choice; the spec session took it into `docs/step2-spec.md` as "Tests while figures move", 9.

1. **The hand calculations of scenario 5 and of Flight 300 in the full slice run on frozen copies of the scenarios,** `data/scenarios/step2-frozen/05_eastern_solomons.toml` (read by `tests/scenario_05_eastern_solomons.rs`) and `data/scenarios/step2-frozen/07_full_slice.toml` (read by `tests/scenario_12d_flight_300.rs`): each scenario as it stood on main with decision 0063's orders, before this record. They are the counterpart of the frozen figures in `data/reference/step1`, and the tests load them on those figures. The only difference from those files is that their paths to the map and the reference files are one folder deeper. They are not scenarios anyone plays: `data/scenarios/*.toml` does not match them, so CI's export loop and the tests that walk the scenarios folder skip them.
2. **Limits.** One frozen copy for a scenario and no more. A copy changes only with a rule change's new hand calculation, written by the architect; nothing remakes it otherwise, and a change to the live scenario's data does not reach it. The doc comment of every test that reads one says it runs on the frozen copy. The folder's `README.md` says the same.
3. **What such a test proves:** that the rules compute what the hand calculation says, on known orders and known figures. It says nothing about the live scenario. The live orders are covered by the run baselines (`crates/sim-cli/tests/run_baseline`) and by `validate`.
4. **Why not the other ways.** Reworking the hand calculations now would be done twice, because the orders move again with decision 0075. Having the test rewrite the scenario's text to put the old orders back is what the spec forbids for figures, and it would hide in a test what the test runs on.

## As built

In progress on branch `eastern-solomons-search`. No rule changed and nothing under `crates/*/src` changed.

- The orders are in `05_eastern_solomons.toml`, `07_full_slice.toml` and `08_soak.toml`, with their reasons as comments. Chikuma's range is 253 until decision 0075 is built.
- `tests/scenario_05_eastern_solomons.rs` and `tests/scenario_12d_flight_300.rs` read the frozen copies. None of their expected numbers changed, and both pass.
- To be remade once, when the orders are final, in commits of their own: the run baselines of scenarios 5, 7 and 8, the pinned hash of scenario 5 in `tests/scenario_10_objectives.rs`, the recorded games, and the full slice's pinned outcome in `tests/scenario_10b_objectives.rs`.

What 1,000 runs of the Eastern Solomons show (seeds 1 to 1000, live figures; `validate` against `data/validation/eastern_solomons.toml`; no report committed). "Interim" is the branch as it stands, Chikuma's floatplanes at 253; the final measures are added when decision 0075 is built.

| Measure | Record | Before (main): 5th to 95th, median, runs holding, verdict | Interim |
| --- | --- | --- | --- |
| japanese_carriers_sunk | 1 | 0 to 1, 0, 464, agrees | the same |
| us_carriers_sunk | 0 | 0 to 0, 0, 1000, agrees | the same |
| bomb_hits_on_enterprise | 3 | 0 to 9, 3, 55, agrees | 0 to 9, 0, 51, agrees |
| bomb_hits_on_saratoga | 0 | 0 to 0, 0, 1000, agrees | the same |
| enterprise_flight_deck_out_at_end | 0 | 0 to 1, 0, 867, agrees | 0 to 1, 0, 876, agrees |
| japanese_carrier_aircraft_lost_to_enemy_action | 29 to 32 | 2 to 40, 26, 165, agrees | 2 to 39, 7, 147, agrees |
| japanese_carrier_aircraft_lost_all_causes | 35 to 54 | 4 to 62, 29, 195, agrees | 4 to 61, 29, 169, agrees |
| ryujo_aircraft_lost_with_the_ship | 4 | 0 to 0, 0, 0, reported | the same |
| us_aircraft_lost_to_enemy_action | 13 to 14 | 1 to 10, 5, 6, miss | 1 to 10, 5, 11, miss |
| us_aircraft_lost_all_causes | 20 to 21 | 1 to 12, 6, 0, miss | 1 to 12, 5, 0, miss |
| japanese_aircraft_attacking_enterprise_force | 37 | 0 to 46, 33, 50, agrees | 0 to 46, 0, 40, **miss** |
| us_aircraft_attacking_ryujo | 36 | 0 to 84, 0, 0, miss | the same |
| us_strike_on_the_japanese_fleet_carriers | 0 | 0 to 0, 0, 1000, agrees | the same |
| us_aircraft_claimed_per_hundred_shot_down | 237 to 362 | 100 to 200, 142, 15 of 998, miss | 100 to 200, 141, 12 of 997, miss |
| japanese_aircraft_claimed_per_hundred_shot_down | 171 to 261 | 100 to 320, 166, 358 of 983, agrees | 100 to 320, 166, 364 of 981, agrees |
| us_hits_claimed_per_hundred_scored | 50 to 140 | 115 to 191, 144, 193 of 469, reported | the same |

Judged 14: before 10 agree and 4 miss; interim 9 agree and 5 miss.

In the interim the Japanese strike on Enterprise's force falls from a middle run of 33 aircraft to none: the dive bombers strike the American carriers in 495 runs of 1,000, against 551 before. Two things do it, both from the record and neither from a figure. Tone's search is gone: her floatplanes searched from (7,-9), 150 miles from the Americans, with a chance of about 0.4 in the afternoon, and the record has no such search (decision 3). And the range flown is beyond the derived search range: the vanguard's floatplanes flew to 330 miles, and until decision 0075 an order may name only 253. Chikuma's arc is narrower than before (60 degrees, not 90), which raises its chance, but it is the only Japanese search that reaches the Americans. A scratch batch with the E13A1 at 330 (a copy of the figures outside the repository) gave 611 runs with a strike and a middle run of 35 aircraft attacking, 50 runs holding 37, 10 measures agreeing; the built result is to be reported against it.

The attack groups struck in no run before and none after. The American strike on Ryujo is unchanged (469 runs). Nothing in `data/reference` was changed in answer to this: no tuning in this stage.
