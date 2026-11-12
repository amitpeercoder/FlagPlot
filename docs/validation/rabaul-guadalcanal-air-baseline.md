# rabaul-guadalcanal-air: validation of batch `baseline`

Battle: rabaul-guadalcanal-air
Scenario: data/scenarios/06_rabaul_guadalcanal.toml
Batch: label `baseline`, 1000 runs, seeds 1 to 1000, file hash 5cc38793a3749d77
Inputs: state hash layout flagplot-state-v6, scenario file hash 3cb76a936397a686
- data/scenarios/../map/south_pacific.csv ba6cb3aea5cc45f5
- data/scenarios/../reference/ship_classes.csv 9d8ba7a2e5948b4b
- data/scenarios/../reference/port_sizes.csv da12e2b751976274
- data/scenarios/../reference/aircraft_types_rabaul_guadalcanal.csv c2d44e8ffaa8fa3d
- data/scenarios/../reference/search.toml 9d9cd2961a30d99a
- data/scenarios/../reference/surface_combat.toml 14b86fdba9f64bfc
- data/scenarios/../reference/air_combat.toml 7019eca7f6998d5c
- data/scenarios/../reference/air_upkeep.toml b2db73fbc8e30d0e
- data/scenarios/../reference/fatigue.toml 32b26b3bef8a1089
- data/scenarios/../reference/air_supply.toml 4269ae7b442c3edc
Measures: data/validation/rabaul_guadalcanal.toml (hash 1a585d180e7e0e59), 15 measures, 10 judged

Judged measures: 3 agree, 1 agree at the edge, 6 miss, of 10.

**The battle does not pass:** 6 of 10 judged measures miss.

| Measure | Record | 5th to 95th | Median | Runs holding the record | Left out | Verdict |
| --- | --- | --- | --- | --- | --- | --- |
| japanese_bombers_lost_to_enemy_action | 12 | 6 to 18 | 12 | 110 of 1000 | 0 | agrees |
| japanese_fighters_lost_to_enemy_action | 6 to 10 | 30 to 50 | 39 | 0 of 1000 | 0 | **MISS**: the record is outside the middle 90 percent; fewer than 1 run in 20 holds the record |
| japanese_bombers_forced_down | 5 to 6 | 9 to 20 | 14 | 9 of 1000 | 0 | reported |
| us_fighters_lost_to_enemy_aircraft | 7 to 23 | 27 to 40 | 34 | 10 of 1000 | 0 | **MISS**: the record is outside the middle 90 percent; fewer than 1 run in 20 holds the record |
| us_aircraft_destroyed_on_the_ground | 2 to 3 | 10 to 23 | 15 | 0 of 1000 | 0 | **MISS**: the record is outside the middle 90 percent; fewer than 1 run in 20 holds the record |
| raid_days_that_reach_guadalcanal | 11 | 11 to 11 | 11 | 1000 of 1000 | 0 | reported |
| bomber_sorties_that_reach_guadalcanal | 211 to 213 | 252 to 272 | 263 | 0 of 1000 | 0 | reported |
| us_aircraft_claimed_per_hundred_shot_down | 250 to 382 | 117 to 145 | 129 | 0 of 1000 | 0 | **MISS**: the record is outside the middle 90 percent; fewer than 1 run in 20 holds the record |
| japanese_aircraft_claimed_on_ground_per_hundred_destroyed | 2066 to 4600 | 192 to 377 | 257 | 0 of 1000 | 0 | reported |
| vmf_223_ready_on_day_5 | 11 | 4 to 13 | 9 | 93 of 1000 | 0 | agrees, at the edge |
| vmsb_232_ready_on_day_5 | 9 | 5 to 12 | 9 | 228 of 1000 | 0 | agrees |
| vmf_223_ready_on_day_10 | 4 to 5 | 0 to 8 | 3 | 205 of 1000 | 0 | agrees |
| f4fs_ready_on_day_21 | 11 | 1 to 9 | 3 | 12 of 1000 | 0 | **MISS**: the record is outside the middle 90 percent; fewer than 1 run in 20 holds the record |
| sbds_ready_on_day_21 | 22 | 22 to 32 | 28 | 35 of 1000 | 0 | **MISS**: fewer than 1 run in 20 holds the record |
| us_pilots_lost | 9 to 22 | 8 to 19 | 13 | 943 of 1000 | 0 | reported |

## Misses

### japanese_fighters_lost_to_enemy_action

- Counts: Fighters of the Tainan and 6th Air Groups shot down by fighters or by anti-aircraft fire
- Record: 6 to 10 (rows 1202, 1216, 1140, 1231)
- Verdict: **MISS**: the record is outside the middle 90 percent; fewer than 1 run in 20 holds the record
- Spread over 1000 runs kept (0 left out): least 22, 5th 30, 25th 35, median 39, 75th 44, 95th 50, greatest 61

### us_fighters_lost_to_enemy_aircraft

- Counts: Henderson Field's F4Fs shot down by fighters or by bombers' guns
- Record: 7 to 23 (rows 1263, 1269, 1285)
- Verdict: **MISS**: the record is outside the middle 90 percent; fewer than 1 run in 20 holds the record
- Spread over 1000 runs kept (0 left out): least 19, 5th 27, 25th 31, median 34, 75th 37, 95th 40, greatest 47

### us_aircraft_destroyed_on_the_ground

- Counts: Henderson Field's F4Fs and SBDs destroyed on the ground by bombing
- Record: 2 to 3 (rows 1266, 1272, 1283, 1299, 1330)
- Verdict: **MISS**: the record is outside the middle 90 percent; fewer than 1 run in 20 holds the record
- Spread over 1000 runs kept (0 left out): least 4, 5th 10, 25th 13, median 15, 75th 18, 95th 23, greatest 35

### us_aircraft_claimed_per_hundred_shot_down

- Counts: Aircraft the US side claims shot down, for each 100 Japanese aircraft really shot down
- Record: 250 to 382 (rows 534, 546, 557, 560, 1332, 1333, 594, 593, 603, 1345, 612, 1351, 936, 629, 641, 472, 1358, 951, 1048, 1202, 1060, 1149, 1154, 1216, 1020, 1024, 1095, 1140, 1101, 1034, 1133, 1109, 1231, 1137, 988, 1049, 1061, 1090, 1110, 1102, 1138)
- Verdict: **MISS**: the record is outside the middle 90 percent; fewer than 1 run in 20 holds the record
- Spread over 1000 runs kept (0 left out): least 106, 5th 117, 25th 123, median 129, 75th 135, 95th 145, greatest 164

### f4fs_ready_on_day_21

- Counts: Aircraft of VMF-223 and VMF-224 ready at the end of day 21
- Record: 11 (rows 1342, 606)
- Verdict: **MISS**: the record is outside the middle 90 percent; fewer than 1 run in 20 holds the record
- Spread over 1000 runs kept (0 left out): least 1, 5th 1, 25th 2, median 3, 75th 5, 95th 9, greatest 15
- Each value and its runs: 1: 107, 2: 299, 3: 214, 4: 105, 5: 88, 6: 50, 7: 47, 8: 36, 9: 21, 10: 15, 11: 12, 12: 4, 13: 1, 15: 1

### sbds_ready_on_day_21

- Counts: Aircraft of VMSB-232, Flight 300 and VMSB-231 ready at the end of day 21
- Record: 22 (rows 1343)
- Verdict: **MISS**: fewer than 1 run in 20 holds the record
- Spread over 1000 runs kept (0 left out): least 15, 5th 22, 25th 25, median 28, 75th 30, 95th 32, greatest 35

## At the edge

### vmf_223_ready_on_day_5

- Counts: Aircraft of VMF-223 ready at the end of day 5
- Record: 11 (rows 455)
- Verdict: agrees, at the edge
- Spread over 1000 runs kept (0 left out): least 0, 5th 4, 25th 7, median 9, 75th 10, 95th 13, greatest 16
- Each value and its runs: 0: 1, 1: 2, 2: 7, 3: 12, 4: 30, 5: 68, 6: 80, 7: 117, 8: 148, 9: 189, 10: 130, 11: 93, 12: 56, 13: 42, 14: 15, 15: 8, 16: 2

## Result on the objectives scale (reported, not judged)

Seen from the Allied side, runs of each result:

- decisive victory: 0 of 1000
- major victory: 1000 of 1000
- minor victory: 0 of 1000
- stalemate: 0 of 1000
- minor defeat: 0 of 1000
- major defeat: 0 of 1000
- decisive defeat: 0 of 1000

Allied margin over 1000 runs: 5th 10, median 10.9, 95th 11.9

## Notes

- Percentiles are by nearest rank: the value at place ceil(p x n / 100) of the n kept runs, sorted. The record is inside the middle 90 percent if any part of it lies from the 5th percentile to the 95th, ends included. The verdict is "agrees, at the edge" where a measure agrees and no part of its record lies from the 25th percentile to the 75th, ends included; it still agrees. A run holds the record if its value is from the low end to the high end, ends included.
- Hits are counted as rolled: a salvo is rolled at the ships afloat when its round starts, so a hit on a ship another salvo sinks in the same round is counted (decision 0062).
- Escorts are matched to a raid's first strike group: an escort entry is added once, when its day, pulse and first group are those of an attack the measure counts (decision 0062).
