# rabaul-guadalcanal-air: validation of batch `d0080`

Battle: rabaul-guadalcanal-air
Scenario: data/scenarios/06_rabaul_guadalcanal.toml
Batch: label `d0080`, 1000 runs, seeds 1 to 1000, file hash 05c690bf54e0fea2
Inputs: state hash layout flagplot-state-v6, scenario file hash 3cb76a936397a686
- data/scenarios/../map/south_pacific.csv ba6cb3aea5cc45f5
- data/scenarios/../reference/ship_classes.csv 9d8ba7a2e5948b4b
- data/scenarios/../reference/port_sizes.csv da12e2b751976274
- data/scenarios/../reference/aircraft_types_rabaul_guadalcanal.csv 8d2c4aa0f492f24f
- data/scenarios/../reference/search.toml 9d9cd2961a30d99a
- data/scenarios/../reference/surface_combat.toml 1a9a0ae06d7282b1
- data/scenarios/../reference/air_combat.toml fee72093cc06b6e8
- data/scenarios/../reference/air_upkeep.toml b2db73fbc8e30d0e
- data/scenarios/../reference/fatigue.toml 32b26b3bef8a1089
- data/scenarios/../reference/air_supply.toml 4269ae7b442c3edc
Measures: data/validation/rabaul_guadalcanal.toml (hash 1a585d180e7e0e59), 15 measures, 10 judged

Judged measures: 2 agree, 1 agree at the edge, 7 miss, of 10.

**The battle does not pass:** 7 of 10 judged measures miss.

| Measure | Record | 5th to 95th | Median | Runs holding the record | Left out | Verdict |
| --- | --- | --- | --- | --- | --- | --- |
| japanese_bombers_lost_to_enemy_action | 12 | 22 to 40 | 31 | 0 of 1000 | 0 | **MISS**: the record is outside the middle 90 percent; fewer than 1 run in 20 holds the record |
| japanese_fighters_lost_to_enemy_action | 6 to 10 | 13 to 27 | 20 | 7 of 1000 | 0 | **MISS**: the record is outside the middle 90 percent; fewer than 1 run in 20 holds the record |
| japanese_bombers_forced_down | 5 to 6 | 7 to 18 | 13 | 31 of 1000 | 0 | reported |
| us_fighters_lost_to_enemy_aircraft | 7 to 23 | 28 to 41 | 35 | 6 of 1000 | 0 | **MISS**: the record is outside the middle 90 percent; fewer than 1 run in 20 holds the record |
| us_aircraft_destroyed_on_the_ground | 2 to 3 | 9 to 21 | 14 | 0 of 1000 | 0 | **MISS**: the record is outside the middle 90 percent; fewer than 1 run in 20 holds the record |
| raid_days_that_reach_guadalcanal | 11 | 11 to 11 | 11 | 1000 of 1000 | 0 | reported |
| bomber_sorties_that_reach_guadalcanal | 211 to 213 | 223 to 255 | 241 | 5 of 1000 | 0 | reported |
| us_aircraft_claimed_per_hundred_shot_down | 250 to 382 | 117 to 145 | 128 | 0 of 1000 | 0 | **MISS**: the record is outside the middle 90 percent; fewer than 1 run in 20 holds the record |
| japanese_aircraft_claimed_on_ground_per_hundred_destroyed | 2066 to 4600 | 191 to 375 | 256 | 0 of 1000 | 0 | reported |
| vmf_223_ready_on_day_5 | 11 | 4 to 12 | 8 | 83 of 1000 | 0 | agrees, at the edge |
| vmsb_232_ready_on_day_5 | 9 | 6 to 11 | 9 | 251 of 1000 | 0 | agrees |
| vmf_223_ready_on_day_10 | 4 to 5 | 0 to 7 | 2 | 181 of 1000 | 0 | agrees |
| f4fs_ready_on_day_21 | 11 | 1 to 8 | 3 | 7 of 1000 | 0 | **MISS**: the record is outside the middle 90 percent; fewer than 1 run in 20 holds the record |
| sbds_ready_on_day_21 | 22 | 22 to 32 | 28 | 29 of 1000 | 0 | **MISS**: fewer than 1 run in 20 holds the record |
| us_pilots_lost | 9 to 22 | 9 to 20 | 14 | 948 of 1000 | 0 | reported |

## Misses

### japanese_bombers_lost_to_enemy_action

- Counts: Rabaul's bombers shot down by fighters or by anti-aircraft fire
- Record: 12 (rows 1048, 1060, 1020, 1024, 1095, 1101, 1034, 1109)
- Verdict: **MISS**: the record is outside the middle 90 percent; fewer than 1 run in 20 holds the record
- Spread over 1000 runs kept (0 left out): least 18, 5th 22, 25th 27, median 31, 75th 34, 95th 40, greatest 50

### japanese_fighters_lost_to_enemy_action

- Counts: Fighters of the Tainan and 6th Air Groups shot down by fighters or by anti-aircraft fire
- Record: 6 to 10 (rows 1202, 1216, 1140, 1231)
- Verdict: **MISS**: the record is outside the middle 90 percent; fewer than 1 run in 20 holds the record
- Spread over 1000 runs kept (0 left out): least 7, 5th 13, 25th 17, median 20, 75th 23, 95th 27, greatest 34

### us_fighters_lost_to_enemy_aircraft

- Counts: Henderson Field's F4Fs shot down by fighters or by bombers' guns
- Record: 7 to 23 (rows 1263, 1269, 1285)
- Verdict: **MISS**: the record is outside the middle 90 percent; fewer than 1 run in 20 holds the record
- Spread over 1000 runs kept (0 left out): least 21, 5th 28, 25th 32, median 35, 75th 37, 95th 41, greatest 48

### us_aircraft_destroyed_on_the_ground

- Counts: Henderson Field's F4Fs and SBDs destroyed on the ground by bombing
- Record: 2 to 3 (rows 1266, 1272, 1283, 1299, 1330)
- Verdict: **MISS**: the record is outside the middle 90 percent; fewer than 1 run in 20 holds the record
- Spread over 1000 runs kept (0 left out): least 6, 5th 9, 25th 12, median 14, 75th 17, 95th 21, greatest 33

### us_aircraft_claimed_per_hundred_shot_down

- Counts: Aircraft the US side claims shot down, for each 100 Japanese aircraft really shot down
- Record: 250 to 382 (rows 534, 546, 557, 560, 1332, 1333, 594, 593, 603, 1345, 612, 1351, 936, 629, 641, 472, 1358, 951, 1048, 1202, 1060, 1149, 1154, 1216, 1020, 1024, 1095, 1140, 1101, 1034, 1133, 1109, 1231, 1137, 988, 1049, 1061, 1090, 1110, 1102, 1138)
- Verdict: **MISS**: the record is outside the middle 90 percent; fewer than 1 run in 20 holds the record
- Spread over 1000 runs kept (0 left out): least 108, 5th 117, 25th 123, median 128, 75th 135, 95th 145, greatest 165

### f4fs_ready_on_day_21

- Counts: Aircraft of VMF-223 and VMF-224 ready at the end of day 21
- Record: 11 (rows 1342, 606)
- Verdict: **MISS**: the record is outside the middle 90 percent; fewer than 1 run in 20 holds the record
- Spread over 1000 runs kept (0 left out): least 1, 5th 1, 25th 2, median 3, 75th 4, 95th 8, greatest 13
- Each value and its runs: 1: 118, 2: 327, 3: 209, 4: 102, 5: 77, 6: 56, 7: 46, 8: 24, 9: 20, 10: 10, 11: 7, 12: 2, 13: 2

### sbds_ready_on_day_21

- Counts: Aircraft of VMSB-232, Flight 300 and VMSB-231 ready at the end of day 21
- Record: 22 (rows 1343)
- Verdict: **MISS**: fewer than 1 run in 20 holds the record
- Spread over 1000 runs kept (0 left out): least 12, 5th 22, 25th 26, median 28, 75th 30, 95th 32, greatest 35

## At the edge

### vmf_223_ready_on_day_5

- Counts: Aircraft of VMF-223 ready at the end of day 5
- Record: 11 (rows 455)
- Verdict: agrees, at the edge
- Spread over 1000 runs kept (0 left out): least 0, 5th 4, 25th 6, median 8, 75th 10, 95th 12, greatest 15
- Each value and its runs: 0: 2, 1: 2, 2: 13, 3: 19, 4: 44, 5: 69, 6: 116, 7: 142, 8: 146, 9: 142, 10: 125, 11: 83, 12: 49, 13: 29, 14: 15, 15: 4

## Result on the objectives scale (reported, not judged)

Seen from the Allied side, runs of each result:

- decisive victory: 0 of 1000
- major victory: 1000 of 1000
- minor victory: 0 of 1000
- stalemate: 0 of 1000
- minor defeat: 0 of 1000
- major defeat: 0 of 1000
- decisive defeat: 0 of 1000

Allied margin over 1000 runs: 5th 10.1, median 10.9, 95th 11.9

## Notes

- Percentiles are by nearest rank: the value at place ceil(p x n / 100) of the n kept runs, sorted. The record is inside the middle 90 percent if any part of it lies from the 5th percentile to the 95th, ends included. The verdict is "agrees, at the edge" where a measure agrees and no part of its record lies from the 25th percentile to the 75th, ends included; it still agrees. A run holds the record if its value is from the low end to the high end, ends included.
- Hits are counted as rolled: a salvo is rolled at the ships afloat when its round starts, so a hit on a ship another salvo sinks in the same round is counted (decision 0062).
- Escorts are matched to a raid's first strike group: an escort entry is added once, when its day, pulse and first group are those of an attack the measure counts (decision 0062).
