# eastern-solomons: validation of batch `baseline`

Battle: eastern-solomons
Scenario: data/scenarios/05_eastern_solomons.toml
Batch: label `baseline`, 1000 runs, seeds 1 to 1000, file hash addb914731821e0d
Inputs: state hash layout flagplot-state-v4, scenario file hash b775cd9c57b69b43
- data/scenarios/../map/south_pacific.csv ba6cb3aea5cc45f5
- data/scenarios/../reference/ship_classes_eastern_solomons.csv 41f5cac8b29d0036
- data/scenarios/../reference/port_sizes.csv da12e2b751976274
- data/scenarios/../reference/aircraft_types_eastern_solomons.csv 487d803eb95dba01
- data/scenarios/../reference/search.toml 9d9cd2961a30d99a
- data/scenarios/../reference/surface_combat.toml 485cfd1b8ba0d4f2
- data/scenarios/../reference/air_combat.toml 7019eca7f6998d5c
Measures: data/validation/eastern_solomons.toml (hash 4c9dc1c54c4dcc3b), 15 measures, 11 judged

Judged measures: 7 agree, 1 agree at the edge, 3 miss, of 11.

**The battle does not pass:** 3 of 11 judged measures miss.

| Measure | Record | 5th to 95th | Median | Runs holding the record | Left out | Verdict |
| --- | --- | --- | --- | --- | --- | --- |
| japanese_carriers_sunk | 1 | 0 to 1 | 0 | 265 of 1000 | 0 | agrees |
| us_carriers_sunk | 0 | 0 to 0 | 0 | 1000 of 1000 | 0 | agrees |
| bomb_hits_on_enterprise | 3 | 0 to 9 | 2 | 52 of 1000 | 0 | agrees |
| bomb_hits_on_saratoga | 0 | 0 to 0 | 0 | 1000 of 1000 | 0 | agrees |
| enterprise_flight_deck_out_at_end | 0 | 0 to 1 | 0 | 869 of 1000 | 0 | agrees |
| japanese_carrier_aircraft_lost_to_enemy_action | 29 to 32 | 0 to 35 | 21 | 155 of 1000 | 0 | agrees |
| japanese_carrier_aircraft_lost_all_causes | 35 to 54 | 0 to 59 | 28 | 114 of 1000 | 0 | agrees, at the edge |
| us_aircraft_lost_to_enemy_action | 13 to 14 | 0 to 5 | 2 | 0 of 1000 | 0 | **MISS**: the record is outside the middle 90 percent; fewer than 1 run in 20 holds the record |
| us_aircraft_lost_all_causes | 20 to 21 | 0 to 7 | 2 | 0 of 1000 | 0 | **MISS**: the record is outside the middle 90 percent; fewer than 1 run in 20 holds the record |
| japanese_aircraft_attacking_enterprise_force | 37 | 0 to 45 | 31 | 47 of 1000 | 0 | reported |
| us_aircraft_attacking_ryujo | 36 | 0 to 46 | 0 | 0 of 1000 | 0 | reported |
| us_strike_on_the_japanese_fleet_carriers | 0 | 0 to 0 | 0 | 1000 of 1000 | 0 | reported |
| us_aircraft_claimed_per_hundred_shot_down | 237 to 362 | 130 to 188 | 153 | 0 of 520 | 480 | **MISS**: the record is outside the middle 90 percent; fewer than 1 run in 20 holds the record |
| japanese_aircraft_claimed_per_hundred_shot_down | 171 to 261 | 100 to 400 | 175 | 219 of 657 | 343 | agrees |
| us_hits_claimed_per_hundred_scored | 50 to 140 | 108 to 200 | 140 | 204 of 383 | 617 | reported |

## Misses

### us_aircraft_lost_to_enemy_action

- Counts: US aircraft shot down by fighters or by anti-aircraft fire
- Record: 13 to 14 (rows 305, 311, 700)
- Verdict: **MISS**: the record is outside the middle 90 percent; fewer than 1 run in 20 holds the record
- Spread over 1000 runs kept (0 left out): least 0, 5th 0, 25th 0, median 2, 75th 3, 95th 5, greatest 9
- Each value and its runs: 0: 343, 1: 152, 2: 161, 3: 139, 4: 106, 5: 50, 6: 22, 7: 18, 8: 8, 9: 1

### us_aircraft_lost_all_causes

- Counts: US aircraft lost to any cause
- Record: 20 to 21 (rows 305, 306, 307, 700)
- Verdict: **MISS**: the record is outside the middle 90 percent; fewer than 1 run in 20 holds the record
- Spread over 1000 runs kept (0 left out): least 0, 5th 0, 25th 0, median 2, 75th 4, 95th 7, greatest 10
- Each value and its runs: 0: 334, 1: 126, 2: 136, 3: 112, 4: 81, 5: 39, 6: 20, 7: 106, 8: 29, 9: 16, 10: 1

### us_aircraft_claimed_per_hundred_shot_down

- Counts: Aircraft the US side claims shot down, for each 100 Japanese aircraft really shot down, floatplanes included
- Record: 237 to 362 (rows 257, 255, 310, 215, 244, 252, 254, 710, 711, 713, 309, 291, 296, 1172, 271, 974)
- Verdict: **MISS**: the record is outside the middle 90 percent; fewer than 1 run in 20 holds the record
- Spread over 520 runs kept (480 left out): least 115, 5th 130, 25th 142, median 153, 75th 165, 95th 188, greatest 233

## At the edge

### japanese_carrier_aircraft_lost_all_causes

- Counts: Aircraft of the Shokaku, Zuikaku and Ryujo units lost to any cause by 18:00 on 24 August, those aboard a carrier that sinks and those that ditch included
- Record: 35 to 54 (rows 291, 1172, 289, 270, 293, 974)
- Verdict: agrees, at the edge
- Spread over 1000 runs kept (0 left out): least 0, 5th 0, 25th 0, median 28, 75th 32, 95th 59, greatest 70

## Result on the objectives scale (reported, not judged)

Seen from the Allied side, runs of each result:

- decisive victory: 0 of 1000
- major victory: 220 of 1000
- minor victory: 143 of 1000
- stalemate: 542 of 1000
- minor defeat: 95 of 1000
- major defeat: 0 of 1000
- decisive defeat: 0 of 1000

Allied margin over 1000 runs: 5th -3.95, median 1.3, 95th 12.6

## Notes

- Percentiles are by nearest rank: the value at place ceil(p x n / 100) of the n kept runs, sorted. The record is inside the middle 90 percent if any part of it lies from the 5th percentile to the 95th, ends included. The verdict is "agrees, at the edge" where a measure agrees and no part of its record lies from the 25th percentile to the 75th, ends included; it still agrees. A run holds the record if its value is from the low end to the high end, ends included.
- Hits are counted as rolled: a salvo is rolled at the ships afloat when its round starts, so a hit on a ship another salvo sinks in the same round is counted (decision 0062).
- Escorts are matched to a raid's first strike group: an escort entry is added once, when its day, pulse and first group are those of an attack the measure counts (decision 0062).
