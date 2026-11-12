# eastern-solomons: validation of batch `baseline`

Battle: eastern-solomons
Scenario: data/scenarios/05_eastern_solomons.toml
Batch: label `baseline`, 1000 runs, seeds 1 to 1000, file hash 7a9cceea4cce4e82
Inputs: state hash layout flagplot-state-v6, scenario file hash f6e1a405a8039d60
- data/scenarios/../map/south_pacific.csv ba6cb3aea5cc45f5
- data/scenarios/../reference/ship_classes_eastern_solomons.csv 41f5cac8b29d0036
- data/scenarios/../reference/port_sizes.csv da12e2b751976274
- data/scenarios/../reference/aircraft_types_eastern_solomons.csv 487d803eb95dba01
- data/scenarios/../reference/search.toml 9d9cd2961a30d99a
- data/scenarios/../reference/surface_combat.toml 14b86fdba9f64bfc
- data/scenarios/../reference/air_combat.toml 7019eca7f6998d5c
Measures: data/validation/eastern_solomons.toml (hash 016ae4090a817434), 16 measures, 14 judged

Judged measures: 10 agree, 0 agree at the edge, 4 miss, of 14.

**The battle does not pass:** 4 of 14 judged measures miss.

| Measure | Record | 5th to 95th | Median | Runs holding the record | Left out | Verdict |
| --- | --- | --- | --- | --- | --- | --- |
| japanese_carriers_sunk | 1 | 0 to 1 | 0 | 464 of 1000 | 0 | agrees |
| us_carriers_sunk | 0 | 0 to 0 | 0 | 1000 of 1000 | 0 | agrees |
| bomb_hits_on_enterprise | 3 | 0 to 9 | 3 | 55 of 1000 | 0 | agrees |
| bomb_hits_on_saratoga | 0 | 0 to 0 | 0 | 1000 of 1000 | 0 | agrees |
| enterprise_flight_deck_out_at_end | 0 | 0 to 1 | 0 | 867 of 1000 | 0 | agrees |
| japanese_carrier_aircraft_lost_to_enemy_action | 29 to 32 | 2 to 40 | 26 | 165 of 1000 | 0 | agrees |
| japanese_carrier_aircraft_lost_all_causes | 35 to 54 | 4 to 62 | 29 | 195 of 1000 | 0 | agrees |
| ryujo_aircraft_lost_with_the_ship | 4 | 0 to 0 | 0 | 0 of 1000 | 0 | reported |
| us_aircraft_lost_to_enemy_action | 13 to 14 | 1 to 10 | 5 | 6 of 1000 | 0 | **MISS**: the record is outside the middle 90 percent; fewer than 1 run in 20 holds the record |
| us_aircraft_lost_all_causes | 20 to 21 | 1 to 12 | 6 | 0 of 1000 | 0 | **MISS**: the record is outside the middle 90 percent; fewer than 1 run in 20 holds the record |
| japanese_aircraft_attacking_enterprise_force | 37 | 0 to 46 | 33 | 50 of 1000 | 0 | agrees |
| us_aircraft_attacking_ryujo | 36 | 0 to 84 | 0 | 0 of 1000 | 0 | **MISS**: fewer than 1 run in 20 holds the record |
| us_strike_on_the_japanese_fleet_carriers | 0 | 0 to 0 | 0 | 1000 of 1000 | 0 | agrees |
| us_aircraft_claimed_per_hundred_shot_down | 237 to 362 | 100 to 200 | 142 | 15 of 998 | 2 | **MISS**: the record is outside the middle 90 percent; fewer than 1 run in 20 holds the record |
| japanese_aircraft_claimed_per_hundred_shot_down | 171 to 261 | 100 to 320 | 166 | 358 of 983 | 17 | agrees |
| us_hits_claimed_per_hundred_scored | 50 to 140 | 115 to 191 | 144 | 193 of 469 | 531 | reported |

## Misses

### us_aircraft_lost_to_enemy_action

- Counts: US aircraft shot down by fighters or by anti-aircraft fire
- Record: 13 to 14 (rows 305, 311, 700)
- Verdict: **MISS**: the record is outside the middle 90 percent; fewer than 1 run in 20 holds the record
- Spread over 1000 runs kept (0 left out): least 0, 5th 1, 25th 3, median 5, 75th 7, 95th 10, greatest 16
- Each value and its runs: 0: 17, 1: 61, 2: 107, 3: 135, 4: 119, 5: 135, 6: 133, 7: 95, 8: 67, 9: 42, 10: 40, 11: 27, 12: 15, 13: 4, 14: 2, 16: 1

### us_aircraft_lost_all_causes

- Counts: US aircraft lost to any cause
- Record: 20 to 21 (rows 305, 306, 307, 700)
- Verdict: **MISS**: the record is outside the middle 90 percent; fewer than 1 run in 20 holds the record
- Spread over 1000 runs kept (0 left out): least 0, 5th 1, 25th 3, median 6, 75th 8, 95th 12, greatest 16
- Each value and its runs: 0: 11, 1: 44, 2: 88, 3: 123, 4: 112, 5: 112, 6: 112, 7: 97, 8: 77, 9: 56, 10: 67, 11: 44, 12: 34, 13: 11, 14: 8, 15: 2, 16: 2

### us_aircraft_attacking_ryujo

- Counts: US aircraft that attack a ship of the Ryujo detachment, with the fighters that escort them, summed over the run
- Record: 36 (rows 333, 334, 335)
- Verdict: **MISS**: fewer than 1 run in 20 holds the record
- Spread over 1000 runs kept (0 left out): least 0, 5th 0, 25th 0, median 0, 75th 82, 95th 84, greatest 85
- Each value and its runs: 0: 531, 76: 2, 77: 8, 78: 17, 79: 39, 80: 52, 81: 97, 82: 117, 83: 83, 84: 49, 85: 5

### us_aircraft_claimed_per_hundred_shot_down

- Counts: Aircraft the US side claims shot down, for each 100 Japanese aircraft really shot down, floatplanes included
- Record: 237 to 362 (rows 257, 255, 310, 215, 244, 252, 254, 710, 711, 713, 309, 291, 296, 1172, 271, 974)
- Verdict: **MISS**: the record is outside the middle 90 percent; fewer than 1 run in 20 holds the record
- Spread over 998 runs kept (2 left out): least 100, 5th 100, 25th 120, median 142, 75th 158, 95th 200, greatest 400

## Result on the objectives scale (reported, not judged)

Seen from the Allied side, runs of each result:

- decisive victory: 0 of 1000
- major victory: 399 of 1000
- minor victory: 70 of 1000
- stalemate: 441 of 1000
- minor defeat: 90 of 1000
- major defeat: 0 of 1000
- decisive defeat: 0 of 1000

Allied margin over 1000 runs: 5th -3.75, median 1.8, 95th 12.7

## Notes

- Percentiles are by nearest rank: the value at place ceil(p x n / 100) of the n kept runs, sorted. The record is inside the middle 90 percent if any part of it lies from the 5th percentile to the 95th, ends included. The verdict is "agrees, at the edge" where a measure agrees and no part of its record lies from the 25th percentile to the 75th, ends included; it still agrees. A run holds the record if its value is from the low end to the high end, ends included.
- Hits are counted as rolled: a salvo is rolled at the ships afloat when its round starts, so a hit on a ship another salvo sinks in the same round is counted (decision 0062).
- Escorts are matched to a raid's first strike group: an escort entry is added once, when its day, pulse and first group are those of an attack the measure counts (decision 0062).
