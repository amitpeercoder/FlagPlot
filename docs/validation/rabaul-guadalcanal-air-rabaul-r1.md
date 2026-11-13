# rabaul-guadalcanal-air: validation of batch `rabaul-r1`

Battle: rabaul-guadalcanal-air
Scenario: data/scenarios/06_rabaul_guadalcanal.toml
Batch: label `rabaul-r1`, 1000 runs, seeds 1 to 1000, file hash feb636d1095a5e25
Inputs: state hash layout flagplot-state-v7, scenario file hash 7e02fa82236e0f69
- data/scenarios/../map/south_pacific.csv ba6cb3aea5cc45f5
- data/scenarios/../reference/ship_classes.csv 366fefba209ee1bc
- data/scenarios/../reference/port_sizes.csv da12e2b751976274
- data/scenarios/../reference/aircraft_types_rabaul_guadalcanal.csv 8d2c4aa0f492f24f
- data/scenarios/../reference/search.toml 9d9cd2961a30d99a
- data/scenarios/../reference/surface_combat.toml 1a9a0ae06d7282b1
- data/scenarios/../reference/air_combat.toml c0cd9c5313d38b5b
- data/scenarios/../reference/air_upkeep.toml b2db73fbc8e30d0e
- data/scenarios/../reference/fatigue.toml 32b26b3bef8a1089
- data/scenarios/../reference/air_supply.toml 4269ae7b442c3edc
Measures: data/validation/rabaul_guadalcanal.toml (hash 268e015147c174ee), 15 measures, 10 judged

Judged measures: 7 agree, 1 agree at the edge, 2 miss, of 10.

**The battle does not pass:** 2 of 10 judged measures miss.

| Measure | Record | 5th to 95th | Median | Runs holding the record | Left out | Verdict |
| --- | --- | --- | --- | --- | --- | --- |
| japanese_bombers_lost_to_enemy_action | 12 | 21 to 37 | 29 | 0 of 1000 | 0 | **MISS**: the record is outside the middle 90 percent; fewer than 1 run in 20 holds the record |
| japanese_fighters_lost_to_enemy_action | 6 to 10 | 8 to 19 | 13 | 187 of 1000 | 0 | agrees, at the edge |
| japanese_bombers_forced_down | 5 to 6 | 5 to 16 | 10 | 75 of 1000 | 0 | reported |
| us_fighters_lost_to_enemy_aircraft | 7 to 23 | 19 to 31 | 25 | 380 of 1000 | 0 | agrees |
| us_aircraft_destroyed_on_the_ground | 2 to 3 | 0 to 5 | 2 | 476 of 1000 | 0 | agrees |
| raid_days_that_reach_guadalcanal | 11 | 9 to 10 | 9 | 0 of 1000 | 0 | reported |
| bomber_sorties_that_reach_guadalcanal | 211 to 213 | 196 to 222 | 212 | 166 of 1000 | 0 | reported |
| us_aircraft_claimed_per_hundred_shot_down | 250 to 382 | 259 to 389 | 312 | 911 of 1000 | 0 | agrees |
| japanese_aircraft_claimed_on_ground_per_hundred_destroyed | 2066 to 4600 | 433 to 2600 | 900 | 119 of 911 | 89 | reported |
| vmf_223_ready_on_day_5 | 11 | 9 to 16 | 12 | 130 of 1000 | 0 | agrees |
| vmsb_232_ready_on_day_5 | 9 | 8 to 12 | 10 | 160 of 1000 | 0 | agrees |
| vmf_223_ready_on_day_10 | 4 to 5 | 1 to 10 | 5 | 241 of 1000 | 0 | agrees |
| f4fs_ready_on_day_21 | 11 | 3 to 15 | 9 | 89 of 1000 | 0 | agrees |
| sbds_ready_on_day_21 | 22 | 27 to 33 | 30 | 0 of 1000 | 0 | **MISS**: the record is outside the middle 90 percent; fewer than 1 run in 20 holds the record |
| us_pilots_lost | 9 to 22 | 5 to 15 | 10 | 665 of 1000 | 0 | reported |

## Misses

### japanese_bombers_lost_to_enemy_action

- Counts: Rabaul's bombers shot down by fighters or by anti-aircraft fire
- Record: 12 (rows 1048, 1060, 1020, 1024, 1095, 1101, 1034, 1109)
- Verdict: **MISS**: the record is outside the middle 90 percent; fewer than 1 run in 20 holds the record
- Spread over 1000 runs kept (0 left out): least 14, 5th 21, 25th 26, median 29, 75th 32, 95th 37, greatest 46

### sbds_ready_on_day_21

- Counts: Aircraft of VMSB-232, Flight 300 and VMSB-231 ready at the end of day 21
- Record: 22 (rows 1343)
- Verdict: **MISS**: the record is outside the middle 90 percent; fewer than 1 run in 20 holds the record
- Spread over 1000 runs kept (0 left out): least 23, 5th 27, 25th 29, median 30, 75th 32, 95th 33, greatest 35
- Each value and its runs: 23: 2, 24: 4, 25: 11, 26: 29, 27: 57, 28: 93, 29: 147, 30: 185, 31: 196, 32: 152, 33: 91, 34: 31, 35: 2

## At the edge

### japanese_fighters_lost_to_enemy_action

- Counts: Fighters of the Tainan and 6th Air Groups shot down by fighters or by anti-aircraft fire
- Record: 6 to 10 (rows 1202, 1216, 1140, 1231)
- Verdict: agrees, at the edge
- Spread over 1000 runs kept (0 left out): least 5, 5th 8, 25th 11, median 13, 75th 16, 95th 19, greatest 26

## Result on the objectives scale (reported, not judged)

Seen from the Allied side, runs of each result:

- decisive victory: 0 of 1000
- major victory: 1000 of 1000
- minor victory: 0 of 1000
- stalemate: 0 of 1000
- minor defeat: 0 of 1000
- major defeat: 0 of 1000
- decisive defeat: 0 of 1000

Allied margin over 1000 runs: 5th 10.25, median 11, 95th 11.75

## Notes

- Percentiles are by nearest rank: the value at place ceil(p x n / 100) of the n kept runs, sorted. The record is inside the middle 90 percent if any part of it lies from the 5th percentile to the 95th, ends included. The verdict is "agrees, at the edge" where a measure agrees and no part of its record lies from the 25th percentile to the 75th, ends included; it still agrees. A run holds the record if its value is from the low end to the high end, ends included.
- Hits are counted as rolled: a salvo is rolled at the ships afloat when its round starts, so a hit on a ship another salvo sinks in the same round is counted (decision 0062).
- Escorts are matched to a raid's first strike group: an escort entry is added once, when its day, pulse and first group are those of an attack the measure counts (decision 0062).
