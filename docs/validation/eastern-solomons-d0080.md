# eastern-solomons: validation of batch `d0080`

Battle: eastern-solomons
Scenario: data/scenarios/05_eastern_solomons.toml
Batch: label `d0080`, 1000 runs, seeds 1 to 1000, file hash 270b7b853ec6d02b
Inputs: state hash layout flagplot-state-v6, scenario file hash ca56f9c14e417648
- data/scenarios/../map/south_pacific.csv ba6cb3aea5cc45f5
- data/scenarios/../reference/ship_classes_eastern_solomons.csv 95e5a244ff64eada
- data/scenarios/../reference/port_sizes.csv da12e2b751976274
- data/scenarios/../reference/aircraft_types_eastern_solomons.csv b7a5c0d9ce3928e0
- data/scenarios/../reference/search.toml 9d9cd2961a30d99a
- data/scenarios/../reference/surface_combat.toml 1a9a0ae06d7282b1
- data/scenarios/../reference/air_combat.toml fee72093cc06b6e8
Measures: data/validation/eastern_solomons.toml (hash 016ae4090a817434), 16 measures, 14 judged

Judged measures: 8 agree, 0 agree at the edge, 6 miss, of 14.

**The battle does not pass:** 6 of 14 judged measures miss.

| Measure | Record | 5th to 95th | Median | Runs holding the record | Left out | Verdict |
| --- | --- | --- | --- | --- | --- | --- |
| japanese_carriers_sunk | 1 | 0 to 1 | 0 | 466 of 1000 | 0 | agrees |
| us_carriers_sunk | 0 | 0 to 0 | 0 | 1000 of 1000 | 0 | agrees |
| bomb_hits_on_enterprise | 3 | 0 to 12 | 6 | 13 of 1000 | 0 | **MISS**: fewer than 1 run in 20 holds the record |
| bomb_hits_on_saratoga | 0 | 0 to 0 | 0 | 1000 of 1000 | 0 | agrees |
| enterprise_flight_deck_out_at_end | 0 | 0 to 1 | 0 | 776 of 1000 | 0 | agrees |
| japanese_carrier_aircraft_lost_to_enemy_action | 29 to 32 | 2 to 40 | 28 | 186 of 1000 | 0 | agrees |
| japanese_carrier_aircraft_lost_all_causes | 35 to 54 | 4 to 62 | 31 | 206 of 1000 | 0 | agrees |
| ryujo_aircraft_lost_with_the_ship | 4 | 0 to 0 | 0 | 0 of 1000 | 0 | reported |
| us_aircraft_lost_to_enemy_action | 13 to 14 | 1 to 11 | 5 | 14 of 1000 | 0 | **MISS**: the record is outside the middle 90 percent; fewer than 1 run in 20 holds the record |
| us_aircraft_lost_all_causes | 20 to 21 | 1 to 12 | 6 | 0 of 1000 | 0 | **MISS**: the record is outside the middle 90 percent; fewer than 1 run in 20 holds the record |
| japanese_aircraft_attacking_enterprise_force | 37 | 0 to 56 | 49 | 0 of 1000 | 0 | **MISS**: fewer than 1 run in 20 holds the record |
| us_aircraft_attacking_ryujo | 36 | 0 to 85 | 0 | 0 of 1000 | 0 | **MISS**: fewer than 1 run in 20 holds the record |
| us_strike_on_the_japanese_fleet_carriers | 0 | 0 to 0 | 0 | 1000 of 1000 | 0 | agrees |
| us_aircraft_claimed_per_hundred_shot_down | 237 to 362 | 100 to 200 | 144 | 9 of 997 | 3 | **MISS**: the record is outside the middle 90 percent; fewer than 1 run in 20 holds the record |
| japanese_aircraft_claimed_per_hundred_shot_down | 171 to 261 | 100 to 300 | 166 | 371 of 982 | 18 | agrees |
| us_hits_claimed_per_hundred_scored | 50 to 140 | 115 to 190 | 143 | 198 of 469 | 531 | reported |

## Misses

### bomb_hits_on_enterprise

- Counts: Bomb hits on USS Enterprise
- Record: 3 (rows 181, 182, 183, 184)
- Verdict: **MISS**: fewer than 1 run in 20 holds the record
- Spread over 1000 runs kept (0 left out): least 0, 5th 0, 25th 0, median 6, 75th 9, 95th 12, greatest 17
- Each value and its runs: 0: 389, 2: 4, 3: 13, 4: 24, 5: 44, 6: 73, 7: 97, 8: 97, 9: 83, 10: 63, 11: 48, 12: 35, 13: 20, 14: 6, 15: 2, 16: 1, 17: 1

### us_aircraft_lost_to_enemy_action

- Counts: US aircraft shot down by fighters or by anti-aircraft fire
- Record: 13 to 14 (rows 305, 311, 700)
- Verdict: **MISS**: the record is outside the middle 90 percent; fewer than 1 run in 20 holds the record
- Spread over 1000 runs kept (0 left out): least 0, 5th 1, 25th 3, median 5, 75th 7, 95th 11, greatest 17
- Each value and its runs: 0: 18, 1: 61, 2: 94, 3: 125, 4: 119, 5: 137, 6: 131, 7: 105, 8: 76, 9: 48, 10: 35, 11: 24, 12: 12, 13: 4, 14: 10, 17: 1

### us_aircraft_lost_all_causes

- Counts: US aircraft lost to any cause
- Record: 20 to 21 (rows 305, 306, 307, 700)
- Verdict: **MISS**: the record is outside the middle 90 percent; fewer than 1 run in 20 holds the record
- Spread over 1000 runs kept (0 left out): least 0, 5th 1, 25th 4, median 6, 75th 9, 95th 12, greatest 17
- Each value and its runs: 0: 14, 1: 50, 2: 73, 3: 105, 4: 94, 5: 104, 6: 100, 7: 91, 8: 75, 9: 72, 10: 70, 11: 67, 12: 36, 13: 20, 14: 17, 15: 8, 16: 3, 17: 1

### japanese_aircraft_attacking_enterprise_force

- Counts: Japanese aircraft that attack a ship of Task Force 16 or Task Force 11, with the fighters that escort them, summed over the run
- Record: 37 (rows 290, 1159, 1164, 409)
- Verdict: **MISS**: fewer than 1 run in 20 holds the record
- Spread over 1000 runs kept (0 left out): least 0, 5th 0, 25th 0, median 49, 75th 52, 95th 56, greatest 59
- Each value and its runs: 0: 389, 42: 1, 43: 1, 44: 7, 45: 12, 46: 15, 47: 23, 48: 38, 49: 56, 50: 64, 51: 78, 52: 72, 53: 68, 54: 72, 55: 47, 56: 29, 57: 18, 58: 8, 59: 2

### us_aircraft_attacking_ryujo

- Counts: US aircraft that attack a ship of the Ryujo detachment, with the fighters that escort them, summed over the run
- Record: 36 (rows 333, 334, 335)
- Verdict: **MISS**: fewer than 1 run in 20 holds the record
- Spread over 1000 runs kept (0 left out): least 0, 5th 0, 25th 0, median 0, 75th 84, 95th 85, greatest 85
- Each value and its runs: 0: 531, 80: 2, 81: 12, 82: 44, 83: 117, 84: 190, 85: 104

### us_aircraft_claimed_per_hundred_shot_down

- Counts: Aircraft the US side claims shot down, for each 100 Japanese aircraft really shot down, floatplanes included
- Record: 237 to 362 (rows 257, 255, 310, 215, 244, 252, 254, 710, 711, 713, 309, 291, 296, 1172, 271, 974)
- Verdict: **MISS**: the record is outside the middle 90 percent; fewer than 1 run in 20 holds the record
- Spread over 997 runs kept (3 left out): least 100, 5th 100, 25th 126, median 144, 75th 158, 95th 200, greatest 400

## Result on the objectives scale (reported, not judged)

Seen from the Allied side, runs of each result:

- decisive victory: 0 of 1000
- major victory: 350 of 1000
- minor victory: 118 of 1000
- stalemate: 394 of 1000
- minor defeat: 138 of 1000
- major defeat: 0 of 1000
- decisive defeat: 0 of 1000

Allied margin over 1000 runs: 5th -3.95, median 1.75, 95th 12.6

## Notes

- Percentiles are by nearest rank: the value at place ceil(p x n / 100) of the n kept runs, sorted. The record is inside the middle 90 percent if any part of it lies from the 5th percentile to the 95th, ends included. The verdict is "agrees, at the edge" where a measure agrees and no part of its record lies from the 25th percentile to the 75th, ends included; it still agrees. A run holds the record if its value is from the low end to the high end, ends included.
- Hits are counted as rolled: a salvo is rolled at the ships afloat when its round starts, so a hit on a ship another salvo sinks in the same round is counted (decision 0062).
- Escorts are matched to a raid's first strike group: an escort entry is added once, when its day, pulse and first group are those of an attack the measure counts (decision 0062).
