# rabaul-guadalcanal-air: validation of batch `r3`

Battle: rabaul-guadalcanal-air
Scenario: data/scenarios/06_rabaul_guadalcanal.toml
Batch: label `r3`, 1000 runs, seeds 1 to 1000, file hash c8277bd03b1b6a0c
Inputs: state hash layout flagplot-state-v7, scenario file hash 7e02fa82236e0f69
- data/scenarios/../map/south_pacific.csv ba6cb3aea5cc45f5
- data/scenarios/../reference/ship_classes.csv 366fefba209ee1bc
- data/scenarios/../reference/port_sizes.csv da12e2b751976274
- data/scenarios/../reference/aircraft_types_rabaul_guadalcanal.csv 8d2c4aa0f492f24f
- data/scenarios/../reference/search.toml 9d9cd2961a30d99a
- data/scenarios/../reference/surface_combat.toml 1a9a0ae06d7282b1
- data/scenarios/../reference/air_combat.toml fe71bc60e64ccbe2
- data/scenarios/../reference/air_upkeep.toml b2db73fbc8e30d0e
- data/scenarios/../reference/fatigue.toml 32b26b3bef8a1089
- data/scenarios/../reference/air_supply.toml 4269ae7b442c3edc
Measures: data/validation/rabaul_guadalcanal.toml (hash 268e015147c174ee), 15 measures, 10 judged

Judged measures: 4 agree, 2 agree at the edge, 4 miss, of 10.

**The battle does not pass:** 4 of 10 judged measures miss.

| Measure | Record | 5th to 95th | Median | Runs holding the record | Left out | Verdict |
| --- | --- | --- | --- | --- | --- | --- |
| japanese_bombers_lost_to_enemy_action | 12 | 19 to 34 | 26 | 1 of 1000 | 0 | **MISS**: the record is outside the middle 90 percent; fewer than 1 run in 20 holds the record |
| japanese_fighters_lost_to_enemy_action | 6 to 10 | 7 to 17 | 12 | 334 of 1000 | 0 | agrees |
| japanese_bombers_forced_down | 5 to 6 | 6 to 16 | 11 | 69 of 1000 | 0 | reported |
| us_fighters_lost_to_enemy_aircraft | 7 to 23 | 17 to 29 | 23 | 528 of 1000 | 0 | agrees |
| us_aircraft_destroyed_on_the_ground | 2 to 3 | 7 to 19 | 13 | 1 of 1000 | 0 | **MISS**: the record is outside the middle 90 percent; fewer than 1 run in 20 holds the record |
| raid_days_that_reach_guadalcanal | 11 | 9 to 10 | 9 | 2 of 1000 | 0 | reported |
| bomber_sorties_that_reach_guadalcanal | 211 to 213 | 200 to 224 | 214 | 148 of 1000 | 0 | reported |
| us_aircraft_claimed_per_hundred_shot_down | 250 to 382 | 121 to 160 | 138 | 0 of 1000 | 0 | **MISS**: the record is outside the middle 90 percent; fewer than 1 run in 20 holds the record |
| japanese_aircraft_claimed_on_ground_per_hundred_destroyed | 2066 to 4600 | 188 to 412 | 260 | 0 of 1000 | 0 | reported |
| vmf_223_ready_on_day_5 | 11 | 6 to 14 | 10 | 152 of 1000 | 0 | agrees |
| vmsb_232_ready_on_day_5 | 9 | 8 to 12 | 10 | 161 of 1000 | 0 | agrees |
| vmf_223_ready_on_day_10 | 4 to 5 | 0 to 6 | 1 | 131 of 1000 | 0 | agrees, at the edge |
| f4fs_ready_on_day_21 | 11 | 2 to 12 | 6 | 40 of 1000 | 0 | **MISS**: fewer than 1 run in 20 holds the record |
| sbds_ready_on_day_21 | 22 | 19 to 31 | 26 | 58 of 1000 | 0 | agrees, at the edge |
| us_pilots_lost | 9 to 22 | 5 to 14 | 9 | 589 of 1000 | 0 | reported |

## Misses

### japanese_bombers_lost_to_enemy_action

- Counts: Rabaul's bombers shot down by fighters or by anti-aircraft fire
- Record: 12 (rows 1048, 1060, 1020, 1024, 1095, 1101, 1034, 1109)
- Verdict: **MISS**: the record is outside the middle 90 percent; fewer than 1 run in 20 holds the record
- Spread over 1000 runs kept (0 left out): least 12, 5th 19, 25th 24, median 26, 75th 29, 95th 34, greatest 40

### us_aircraft_destroyed_on_the_ground

- Counts: Henderson Field's F4Fs and SBDs destroyed on the ground by bombing
- Record: 2 to 3 (rows 1266, 1272, 1283, 1299, 1330)
- Verdict: **MISS**: the record is outside the middle 90 percent; fewer than 1 run in 20 holds the record
- Spread over 1000 runs kept (0 left out): least 2, 5th 7, 25th 10, median 13, 75th 15, 95th 19, greatest 23

### us_aircraft_claimed_per_hundred_shot_down

- Counts: Aircraft the US side claims shot down, for each 100 Japanese aircraft really shot down
- Record: 250 to 382 (rows 534, 546, 557, 560, 1332, 1333, 594, 593, 603, 1345, 612, 1351, 936, 629, 641, 472, 1358, 951, 1048, 1202, 1060, 1149, 1154, 1216, 1020, 1024, 1095, 1140, 1101, 1034, 1133, 1109, 1231, 1137, 988, 1049, 1061, 1090, 1110, 1102, 1138)
- Verdict: **MISS**: the record is outside the middle 90 percent; fewer than 1 run in 20 holds the record
- Spread over 1000 runs kept (0 left out): least 110, 5th 121, 25th 130, median 138, 75th 146, 95th 160, greatest 182

### f4fs_ready_on_day_21

- Counts: Aircraft of VMF-223 and VMF-224 ready at the end of day 21
- Record: 11 (rows 1342, 606)
- Verdict: **MISS**: fewer than 1 run in 20 holds the record
- Spread over 1000 runs kept (0 left out): least 1, 5th 2, 25th 4, median 6, 75th 8, 95th 12, greatest 15
- Each value and its runs: 1: 8, 2: 92, 3: 102, 4: 102, 5: 119, 6: 109, 7: 115, 8: 113, 9: 88, 10: 59, 11: 40, 12: 29, 13: 12, 14: 10, 15: 2

## At the edge

### vmf_223_ready_on_day_10

- Counts: Aircraft of VMF-223 ready at the end of day 10
- Record: 4 to 5 (rows 893, 569, 568)
- Verdict: agrees, at the edge
- Spread over 1000 runs kept (0 left out): least 0, 5th 0, 25th 0, median 1, 75th 3, 95th 6, greatest 11
- Each value and its runs: 0: 402, 1: 184, 2: 127, 3: 100, 4: 78, 5: 53, 6: 23, 7: 18, 8: 10, 9: 3, 10: 1, 11: 1

### sbds_ready_on_day_21

- Counts: Aircraft of VMSB-232, Flight 300 and VMSB-231 ready at the end of day 21
- Record: 22 (rows 1343)
- Verdict: agrees, at the edge
- Spread over 1000 runs kept (0 left out): least 13, 5th 19, 25th 23, median 26, 75th 28, 95th 31, greatest 34

## Result on the objectives scale (reported, not judged)

Seen from the Allied side, runs of each result:

- decisive victory: 0 of 1000
- major victory: 1000 of 1000
- minor victory: 0 of 1000
- stalemate: 0 of 1000
- minor defeat: 0 of 1000
- major defeat: 0 of 1000
- decisive defeat: 0 of 1000

Allied margin over 1000 runs: 5th 9.65, median 10.4, 95th 11.2

## Notes

- Percentiles are by nearest rank: the value at place ceil(p x n / 100) of the n kept runs, sorted. The record is inside the middle 90 percent if any part of it lies from the 5th percentile to the 95th, ends included. The verdict is "agrees, at the edge" where a measure agrees and no part of its record lies from the 25th percentile to the 75th, ends included; it still agrees. A run holds the record if its value is from the low end to the high end, ends included.
- Hits are counted as rolled: a salvo is rolled at the ships afloat when its round starts, so a hit on a ship another salvo sinks in the same round is counted (decision 0062).
- Escorts are matched to a raid's first strike group: an escort entry is added once, when its day, pulse and first group are those of an attack the measure counts (decision 0062).
