# 0073: The Japanese search at the Eastern Solomons, from the record

Status: settled by the architect. Scenario data, not a rule: no rule changed and nothing under `crates/*/src` changed. It replaces item 8 of decision 0031 for the Japanese side.

## The gap

The spec and the opening list both say it: the Japanese search at the Eastern Solomons was a placeholder arc. Decision 0031, item 8, had the floatplanes of Chikuma and Tone search 110 to 200 degrees to 253 miles, "a quarter-circle toward the south-east", because no source then gave the Japanese sectors. The Japanese records read since (`docs/research/oob/japanese-records-2026-11-12.md`) give the search orders and what was flown.

## What the record says

Japanese times are Tokyo time, two hours behind the scenario's clock (UTC+11): Ryujo's attack is 13:50 in Japanese quotes and 16:20 on the American clock, which is UTC+11.5. All rows are `data/oob/engagement_events.csv`.

| Who | When (Tokyo; scenario) | What | Rows |
| --- | --- | --- | --- |
| Nagumo's orders | 00:05 to 03:46 | Base line A is 150 degrees, launch 05:00; the second search group's left half shortened to 200 miles; the fourth search group cancelled; the 11th Sentai to search from 09:00, pattern 8 including line 1, left half, six aircraft, each line 30 miles shorter, base line 150 | 303, 304, 305, 435, 437 |
| Shokaku's attack unit | 04:15; 06:15 | 7 Type 97s, one to each line, 250 miles, no lateral leg. One sighted a submarine on the 150 degree line | 366, 367 |
| Zuikaku's attack unit | 04:35; 06:35 | 12 Type 97s, a sector "?0 to 110 degrees" to 230 miles (250 struck out) and 0 to 60 to 200 | 368 |
| The carriers, as Nagumo reported | 05:00; 07:00 | 7 attack planes searched 250 miles to the south and saw no enemy. The morning search saw nothing but two submarines | 369, 306, 440 |
| The vanguard's floatplanes | 09:00 to 09:40; 11:00 to 11:40 | Four searched south and south-east, to 330 miles at most: Chikuma's two at 09:00, Kumano's (two; Suzuya's catapult was out of order) at 09:20 to 09:40. Kumano's two came back at 15:00 having seen nothing | 370, 438, 439, 307, 308 |
| Chikuma's No. 2 | 12:05 to 12:10; 14:05 | "Large enemy force sighted", then pursued by fighters and lost. Nagumo put it at 153 degrees and about 260 miles from the carriers; the vanguard at 153 degrees 240 miles from Hiei | 441, 442, 443, 338, 339 |
| Contact and night planes | 13:18 and after; 15:18 | Hiei's and Chikuma's, one each, to keep contact; later Chikuma's No. 1 reports the fleet; Nagara's and Kirishima's for the night | 446, 342, 453, 452, 455 |
| Tone, with Ryujo | from 02:00 | Detached with Ryujo. No row has her or Ryujo fly a search on the 24th. A signal of 05:50 to the detachment is recorded without its text | 327, 147, 328 |
| A website | 11:00 local | "Chikuma and Tone of the Vanguard Force launch seven floatplanes" | 172 |

The strikes of the two fleet carriers were dive bombers and fighters only (344, 371, 374, 376 to 379). Their attack planes flew the dawn search and struck nothing.

## Decisions

1. **The vanguard's floatplanes (group 17, Chikuma's five E13A1) search 120 to 180 degrees to 253 miles.** The arc is the record's "south and south-east" (370) about Nagumo's base line of 150 (303, 437), sixty degrees for six lines ten degrees apart, the spacing of the vanguard's search order for the next day (394). It holds the bearing of the sighting, 153 (441). The range is the E13A1's derived search range, the most an order may name; the record has 330 at most and each line 30 shorter than the pattern's.
2. **The fleet carriers' attack units search, and no longer stand on naval strike.** Shokaku's (group 13) 120 to 180, seven lines about the base line of 150, "to the south" (366, 369). Zuikaku's (group 16) 0 to 110 (368). Both to 175 miles, the B5N2's derived search range; the record has 250, and 230 and 200.
3. **Tone's floatplanes (group 20) have no order.** No row has the Ryujo detachment search. Row 172, a website, has Tone launch searchers, but puts her in the vanguard with Chikuma, against her own division's war diary (327, high), which has her detached with Ryujo from 02:00; and the search was ordered to the vanguard's commander (437). The website's sentence is read as the vanguard's search, with Tone named in error. Whether the detachment flew a search of its own is a gap (row 328's text).
4. **The American search stays as it was.** It is from rows, not a placeholder: Enterprise's scouting squadron, 290 to 090 (the afternoon search, 158; 157 has 270 to 090) to 200 miles (the dawn search, 144 and 146, and the SBD-3's derived search range). It is a blend of the two searches the day had, which one standing order must be.

### Where the rows disagree or are unclear

- **How many attack planes searched.** Shokaku's report has 7 and Zuikaku's 12; Nagumo's report (369) has 7 "to the south". Nagumo's orders name more than one search group (303: the second group's left half; 304: the fourth cancelled). Settled: both searched; 369 reports the southern search only, which is Shokaku's. Zuikaku's sector, 0 to 110, is not to the south.
- **Zuikaku's first bearing is illegible** ("?0"). It is read as 60, so that the two sectors meet: 60 to 110 at 230 miles and 0 to 60 at 200. The second group's left half shortened to 200 miles (303) is then 0 to 60, the left half of 0 to 110. The bearings are read as true bearings, not as angles from the base line. Read either way at 175 miles nothing lies inside it on the day.
- **"Left half" in the vanguard's order (437).** If the left half of a pattern laid on 150 degrees is the lines east of it, the arc would be about 100 to 150. That is not taken: the vanguard's own report says south and south-east (370), and the sighting was at 153 (441). The sighting is an estimated position, three degrees from 150, so it is weak evidence either way; the words of 370 decide it.
- **Four flew, six were ordered** (370, 437). Group 17 has five, an establishment of June 1942 (decision 0031, item 7). The five stand for the vanguard's four to six; the scenario does not hold the other cruisers' and the battleships' floatplanes.

## What the rules cannot express

The search rules (`docs/rules/03-search-contacts-and-claims.md`, Air search) give an air group one standing arc, flown in every daylight pulse from the pulse's start, with every aircraft the group can fly. So:

- **A launch hour.** The dawn search (06:15 to 07:00) and the vanguard's search (11:00 to 11:40) both fly in the morning pulse, from the 06:00 positions; no enemy force is within 280 miles then, so the morning search finds nothing, as on the day. The vanguard's sighting at 14:05 falls in the afternoon pulse, which the group also flies, from the 12:00 positions. Nearest expression: the search mission for the whole day.
- **A search flown once.** The attack planes searched at dawn only; here they search in the afternoon too. At 175 miles they reach nothing in either pulse at the Eastern Solomons, so the cost is wear: each flight is a sortie of 350 miles for the wear rule (chapter 5).
- **Part of a group.** Shokaku sent 7 of her 18 attack planes and Zuikaku 12 of 18. A group searches whole ("Not built: a group sending only part of itself on search"). The remaining attack planes were held, and on search they cannot strike; on the day they struck nothing.
- **Range beyond the derived search range.** An order may not name more than the type's search range: 175 for the B5N2 against 250 flown, 253 for the E13A1 against 330. These are derived figures (decision 0029); whether the derivation should read the distances flown is a question for the derivation, not for this record, and nothing in `data/reference` is changed here.
- **Contact planes and night searches** (446, 452, 455) are not separate: a later sighting of the same force refreshes the contact in the afternoon search, and there is no night search.
- **A report not received.** The strike leader missed the floatplanes' reports (266). There is no lost report (decision 0031, "Not modelled").

## Who follows

The full slice (07) and the soak (08) copy scenario 5's orders on day 4, with group numbers plus 100, as they did for decision 0063. They follow: groups 117, 113 and 116 take the new arcs and 120 has no order. In both the standing search lasts from day 4 to the end, so the two attack units never strike in them. The sub-scenarios 05a to 05d have their own invented searches and are not copies.

## Rule card

No rule. These are scenario inputs.

## As built

Not finished: the orders are in `05_eastern_solomons.toml`, `07_full_slice.toml` and `08_soak.toml` on branch `eastern-solomons-search`, and the work stopped there, because hand-calculation tests would move and nobody but the architect changes an expected number:

- `tests/scenario_05_eastern_solomons.rs`: 14 of 23 tests fail. Its tables hold the step 1 orders (groups 13 and 16 on strike, 17 and 20 searching 110 to 200), so its search table, its morning and afternoon searches and everything downstream of the afternoon draws move.
- `tests/scenario_12d_flight_300.rs`: the pinned outcomes of seeds 28 and 37 in the full slice.
- `tests/scenario_10b_objectives.rs`: the full slice on its own seed (asserted `(80, 1, 0, 16)`, now `(22, 1, 0, 25)`).
- The hash tests of `scenario_06c`, `06d1`, `06d2` and `10`, the export golden files and the recorded games follow live scenario data and would be remade with the change.

No test, golden file, baseline or recorded game was changed.

What 1,000 runs of the Eastern Solomons show (seeds 1 to 1000, live figures; `validate` against `data/validation/eastern_solomons.toml`; no report committed):

| Measure | Record | Before: 5th to 95th, median, runs holding, verdict | After |
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

Judged 14: before 10 agree and 4 miss; after 9 agree and 5 miss. The Japanese dive bombers strike the American carriers in 495 runs of 1,000, against 551 before: Tone's floatplanes, 150 miles from the Americans, no longer search, and Chikuma's 120 to 180 arc is narrower than 110 to 200, so it sights more often. The torpedo groups struck in no run before and none after. The American strike on Ryujo is unchanged (469 runs). Nothing in `data/reference` was changed in answer to this.
