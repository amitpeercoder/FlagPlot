# eastern-solomons: validation of batch `d0079`

Battle: eastern-solomons
Scenario: data/scenarios/05_eastern_solomons.toml
Batch: label `d0079`, 1000 runs, seeds 1 to 1000, file hash b6c0f7920f302563
Inputs: state hash layout flagplot-state-v7, scenario file hash a9443b066a393e0b
- data/scenarios/../map/south_pacific.csv ba6cb3aea5cc45f5
- data/scenarios/../reference/ship_classes_eastern_solomons.csv 2716ae28c805274f
- data/scenarios/../reference/port_sizes.csv da12e2b751976274
- data/scenarios/../reference/aircraft_types_eastern_solomons.csv 8a7741632e7e7ec6
- data/scenarios/../reference/search.toml 9d9cd2961a30d99a
- data/scenarios/../reference/surface_combat.toml 1a9a0ae06d7282b1
- data/scenarios/../reference/air_combat.toml 439815ff5b44e117
Measures: data/validation/eastern_solomons.toml (hash 86d416372040b1ea), 16 measures, 14 judged

Judged measures: 10 agree, 1 agree at the edge, 3 miss, of 14.

**The battle does not pass:** 3 of 14 judged measures miss.

| Measure | Record | 5th to 95th | Median | Runs holding the record | Left out | Verdict |
| --- | --- | --- | --- | --- | --- | --- |
| japanese_carriers_sunk | 1 | 0 to 1 | 0 | 239 of 1000 | 0 | agrees, at the edge |
| us_carriers_sunk | 0 | 0 to 0 | 0 | 1000 of 1000 | 0 | agrees |
| bomb_hits_on_enterprise | 3 | 0 to 5 | 1 | 123 of 1000 | 0 | agrees |
| bomb_hits_on_saratoga | 0 | 0 to 0 | 0 | 1000 of 1000 | 0 | agrees |
| enterprise_flight_deck_out_at_end | 0 | 0 to 0 | 0 | 955 of 1000 | 0 | agrees |
| japanese_carrier_aircraft_lost_to_enemy_action | 29 to 32 | 2 to 35 | 26 | 234 of 1000 | 0 | agrees |
| japanese_carrier_aircraft_lost_all_causes | 35 to 54 | 4 to 58 | 29 | 201 of 1000 | 0 | agrees |
| ryujo_aircraft_lost_with_the_ship | 4 | 0 to 0 | 0 | 0 of 1000 | 0 | reported |
| us_aircraft_lost_to_enemy_action | 13 to 14 | 1 to 9 | 4 | 2 of 1000 | 0 | **MISS**: the record is outside the middle 90 percent; fewer than 1 run in 20 holds the record |
| us_aircraft_lost_all_causes | 20 to 21 | 1 to 10 | 5 | 0 of 1000 | 0 | **MISS**: the record is outside the middle 90 percent; fewer than 1 run in 20 holds the record |
| japanese_aircraft_attacking_enterprise_force | 37 | 0 to 37 | 37 | 607 of 1000 | 0 | agrees |
| us_aircraft_attacking_ryujo | 36 to 38 | 0 to 38 | 0 | 488 of 1000 | 0 | agrees |
| us_strike_on_the_japanese_fleet_carriers | 0 | 0 to 0 | 0 | 1000 of 1000 | 0 | agrees |
| us_aircraft_claimed_per_hundred_shot_down | 237 to 362 | 100 to 200 | 148 | 9 of 997 | 3 | **MISS**: the record is outside the middle 90 percent; fewer than 1 run in 20 holds the record |
| japanese_aircraft_claimed_per_hundred_shot_down | 171 to 261 | 100 to 300 | 150 | 298 of 980 | 20 | agrees |
| us_hits_claimed_per_hundred_scored | 50 to 140 | 108 to 220 | 140 | 254 of 488 | 512 | reported |

## Misses

### us_aircraft_lost_to_enemy_action

- Counts: US aircraft shot down by fighters or by anti-aircraft fire
- Record: 13 to 14 (rows 305, 311, 700)
- Verdict: **MISS**: the record is outside the middle 90 percent; fewer than 1 run in 20 holds the record
- Spread over 1000 runs kept (0 left out): least 0, 5th 1, 25th 3, median 4, 75th 6, 95th 9, greatest 14
- Each value and its runs: 0: 20, 1: 63, 2: 123, 3: 160, 4: 155, 5: 147, 6: 125, 7: 92, 8: 48, 9: 36, 10: 18, 11: 9, 12: 2, 14: 2

### us_aircraft_lost_all_causes

- Counts: US aircraft lost to any cause
- Record: 20 to 21 (rows 305, 306, 307, 700)
- Verdict: **MISS**: the record is outside the middle 90 percent; fewer than 1 run in 20 holds the record
- Spread over 1000 runs kept (0 left out): least 0, 5th 1, 25th 3, median 5, 75th 7, 95th 10, greatest 15
- Each value and its runs: 0: 13, 1: 49, 2: 103, 3: 140, 4: 160, 5: 129, 6: 114, 7: 108, 8: 60, 9: 53, 10: 31, 11: 18, 12: 13, 13: 5, 14: 3, 15: 1

### us_aircraft_claimed_per_hundred_shot_down

- Counts: Aircraft the US side claims shot down, for each 100 Japanese aircraft really shot down, floatplanes included
- Record: 237 to 362 (rows 257, 255, 310, 215, 244, 252, 254, 710, 711, 713, 309, 291, 296, 1172, 271, 974)
- Verdict: **MISS**: the record is outside the middle 90 percent; fewer than 1 run in 20 holds the record
- Spread over 997 runs kept (3 left out): least 100, 5th 100, 25th 130, median 148, 75th 161, 95th 200, greatest 400

## At the edge

### japanese_carriers_sunk

- Counts: Japanese carriers sunk at the end of the run
- Record: 1 (rows 666, 667, 1175, 273, 274)
- Verdict: agrees, at the edge
- Spread over 1000 runs kept (0 left out): least 0, 5th 0, 25th 0, median 0, 75th 0, 95th 1, greatest 1
- Each value and its runs: 0: 761, 1: 239

## Result on the objectives scale (reported, not judged)

Seen from the Allied side, runs of each result:

- decisive victory: 0 of 1000
- major victory: 225 of 1000
- minor victory: 219 of 1000
- stalemate: 534 of 1000
- minor defeat: 22 of 1000
- major defeat: 0 of 1000
- decisive defeat: 0 of 1000

Allied margin over 1000 runs: 5th 0, median 1.6, 95th 12.45

## Notes

- Percentiles are by nearest rank: the value at place ceil(p x n / 100) of the n kept runs, sorted. The record is inside the middle 90 percent if any part of it lies from the 5th percentile to the 95th, ends included. The verdict is "agrees, at the edge" where a measure agrees and no part of its record lies from the 25th percentile to the 75th, ends included; it still agrees. A run holds the record if its value is from the low end to the high end, ends included.
- Hits are counted as rolled: a salvo is rolled at the ships afloat when its round starts, so a hit on a ship another salvo sinks in the same round is counted (decision 0062).
- Escorts are matched to a raid's first strike group: an escort entry is added once, when its day, pulse and first group are those of an attack the measure counts (decision 0062).
