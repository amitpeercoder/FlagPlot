# savo-island: validation of batch `baseline`

Battle: savo-island
Scenario: data/scenarios/04_savo_island.toml
Batch: label `baseline`, 1000 runs, seeds 1 to 1000, file hash 7f1c6169106f4907
Inputs: state hash layout flagplot-state-v6, scenario file hash 5df521f430c89637
- data/scenarios/../map/south_pacific.csv ba6cb3aea5cc45f5
- data/scenarios/../reference/ship_classes_savo_island.csv dd1b9196736f2f7b
- data/scenarios/../reference/port_sizes.csv da12e2b751976274
- data/scenarios/../reference/aircraft_types.csv 92830cbacc0a4328
- data/scenarios/../reference/search.toml 9d9cd2961a30d99a
- data/scenarios/../reference/surface_combat.toml d51cd2bd52324e8e
Measures: data/validation/savo_island.toml (hash e6d489dd6cd897d1), 26 measures, 23 judged

Judged measures: 14 agree, 4 agree at the edge, 5 miss, of 23.

**The battle does not pass:** 5 of 23 judged measures miss.

| Measure | Record | 5th to 95th | Median | Runs holding the record | Left out | Verdict |
| --- | --- | --- | --- | --- | --- | --- |
| allied_cruisers_sunk | 4 | 0 to 4 | 2 | 113 of 1000 | 0 | agrees, at the edge |
| southern_group_cruisers_sunk | 1 | 0 to 2 | 1 | 531 of 1000 | 0 | agrees |
| northern_group_cruisers_sunk | 3 | 0 to 3 | 1 | 110 of 1000 | 0 | agrees, at the edge |
| allied_destroyers_sunk | 0 | 0 to 0 | 0 | 984 of 1000 | 0 | agrees |
| japanese_ships_sunk | 0 | 0 to 1 | 0 | 823 of 1000 | 0 | agrees |
| japanese_ships_with_propulsion_out | 0 | 0 to 1 | 0 | 879 of 1000 | 0 | agrees |
| japanese_ships_sunk_or_stopped | 0 | 0 to 2 | 0 | 722 of 1000 | 0 | agrees |
| night_fights_the_japanese_did_not_sight_first | 0 | 0 to 2 | 1 | 335 of 1000 | 0 | agrees |
| southern_group_rounds_surprised | 1 to 2 | 0 to 1 | 1 | 577 of 1000 | 0 | reported |
| northern_group_rounds_surprised | 2 to 3 | 0 to 1 | 1 | 0 of 1000 | 0 | reported |
| gun_hits_on_allied_ships | 153 to 219 | 30 to 129 | 87 | 8 of 1000 | 0 | **MISS**: the record is outside the middle 90 percent; fewer than 1 run in 20 holds the record |
| gun_hits_on_japanese_ships | 10 to 17 | 9 to 68 | 25 | 232 of 1000 | 0 | agrees |
| torpedo_hits_on_allied_ships | 4 to 9 | 1 to 6 | 3 | 403 of 1000 | 0 | agrees |
| torpedo_hits_on_japanese_ships | 0 | 0 to 4 | 1 | 288 of 1000 | 0 | agrees |
| allied_torpedoes_fired | 4 to 6 | 16 to 36 | 36 | 0 of 1000 | 0 | **MISS**: the record is outside the middle 90 percent; fewer than 1 run in 20 holds the record |
| japanese_gun_hits_per_thousand_shells | 81 to 117 | 38 to 72 | 58 | 3 of 1000 | 0 | **MISS**: the record is outside the middle 90 percent; fewer than 1 run in 20 holds the record |
| allied_gun_hits_per_thousand_shells | 21 to 36 | 21 to 60 | 40 | 332 of 1000 | 0 | agrees |
| japanese_main_battery_shells_fired_in_hundreds | 10 to 11 | 6 to 19 | 14 | 114 of 1000 | 0 | agrees |
| allied_main_battery_shells_fired_in_hundreds | 3 | 3 to 13 | 6 | 120 of 1000 | 0 | agrees, at the edge |
| southern_group_main_battery_shells_fired_in_hundreds | 0 | 0 to 4 | 0 | 554 of 1000 | 0 | agrees |
| northern_group_main_battery_shells_fired_in_hundreds | 3 | 2 to 10 | 5 | 174 of 1000 | 0 | agrees, at the edge |
| tenryu_and_yubari_main_battery_shells_fired_in_hundreds | 1 | 2 to 5 | 3 | 27 of 1000 | 0 | **MISS**: the record is outside the middle 90 percent; fewer than 1 run in 20 holds the record |
| minutes_of_action | 20 to 30 | 35 to 65 | 40 | 19 of 1000 | 0 | **MISS**: the record is outside the middle 90 percent; fewer than 1 run in 20 holds the record |
| surface_fights_by_day | 0 | 0 to 4 | 0 | 777 of 1000 | 0 | agrees |
| japanese_ships_claimed_sunk_per_hundred_sunk | 350 | 100 to 300 | 100 | 1 of 897 | 103 | reported |
| allied_ships_claimed_sunk | 0 | 0 to 2 | 0 | 790 of 1000 | 0 | agrees |

## Misses

### gun_hits_on_allied_ships

- Counts: Gun hits on the nine Allied ships, all calibres
- Record: 153 to 219 (rows 19, 20, 37, 50, 58, 59, 60, 69, 70, 71, 82, 83, 49, 93, 96)
- Verdict: **MISS**: the record is outside the middle 90 percent; fewer than 1 run in 20 holds the record
- Spread over 1000 runs kept (0 left out): least 8, 5th 30, 25th 64, median 87, 75th 108, 95th 129, greatest 172

### allied_torpedoes_fired

- Counts: Torpedoes the two Allied groups fire in the run
- Record: 4 to 6 (rows 46, 47, 57, 3211, 30, 180, 3179, 3180)
- Verdict: **MISS**: the record is outside the middle 90 percent; fewer than 1 run in 20 holds the record
- Spread over 1000 runs kept (0 left out): least 16, 5th 16, 25th 16, median 36, 75th 36, 95th 36, greatest 56
- Each value and its runs: 16: 345, 20: 1, 32: 101, 34: 19, 36: 488, 48: 31, 52: 3, 56: 12

### japanese_gun_hits_per_thousand_shells

- Counts: Gun hits scored by the Japanese force for each 1,000 gun shots it fires, all calibres, rounded down
- Record: 81 to 117 (rows 7, 19, 37, 50, 58, 69, 83, 60, 71, 82, 9)
- Verdict: **MISS**: the record is outside the middle 90 percent; fewer than 1 run in 20 holds the record
- Spread over 1000 runs kept (0 left out): least 17, 5th 38, 25th 50, median 58, 75th 65, 95th 72, greatest 85

### tenryu_and_yubari_main_battery_shells_fired_in_hundreds

- Counts: Main battery shots Tenryu and Yubari fire in the run, in hundreds, rounded down
- Record: 1 (rows 3, 148, 154)
- Verdict: **MISS**: the record is outside the middle 90 percent; fewer than 1 run in 20 holds the record
- Spread over 1000 runs kept (0 left out): least 0, 5th 2, 25th 3, median 3, 75th 4, 95th 5, greatest 6
- Each value and its runs: 0: 5, 1: 27, 2: 97, 3: 596, 4: 184, 5: 90, 6: 1

### minutes_of_action

- Counts: Minutes of surface fighting in the run: the rounds each fight ran, summed, times the minutes a round stands for
- Record: 20 to 30 (rows 14, 15, 16)
- Verdict: **MISS**: the record is outside the middle 90 percent; fewer than 1 run in 20 holds the record
- Spread over 1000 runs kept (0 left out): least 15, 5th 35, 25th 35, median 40, 75th 50, 95th 65, greatest 95
- Each value and its runs: 15: 3, 20: 4, 25: 5, 30: 10, 35: 327, 40: 241, 45: 129, 50: 69, 55: 69, 60: 68, 65: 33, 70: 25, 75: 8, 80: 4, 85: 2, 90: 2, 95: 1

## At the edge

### allied_cruisers_sunk

- Counts: Allied cruisers sunk at the end of the run, of the five
- Record: 4 (rows 761, 762, 763, 764, 765, 768, 769)
- Verdict: agrees, at the edge
- Spread over 1000 runs kept (0 left out): least 0, 5th 0, 25th 1, median 2, 75th 3, 95th 4, greatest 5
- Each value and its runs: 0: 103, 1: 318, 2: 264, 3: 170, 4: 113, 5: 32

### northern_group_cruisers_sunk

- Counts: Cruisers of the northern group sunk at the end of the run, of the three
- Record: 3 (rows 762, 763, 764)
- Verdict: agrees, at the edge
- Spread over 1000 runs kept (0 left out): least 0, 5th 0, 25th 0, median 1, 75th 2, 95th 3, greatest 3
- Each value and its runs: 0: 490, 1: 219, 2: 181, 3: 110

### allied_main_battery_shells_fired_in_hundreds

- Counts: Main battery shots the two Allied groups fire in the run, in hundreds, rounded down
- Record: 3 (rows 12, 53, 54, 94, 97)
- Verdict: agrees, at the edge
- Spread over 1000 runs kept (0 left out): least 1, 5th 3, 25th 4, median 6, 75th 8, 95th 13, greatest 22

### northern_group_main_battery_shells_fired_in_hundreds

- Counts: Main battery shots the five ships of the northern group fire in the run, in hundreds, rounded down
- Record: 3 (rows 64, 74, 86, 94, 97)
- Verdict: agrees, at the edge
- Spread over 1000 runs kept (0 left out): least 0, 5th 2, 25th 4, median 5, 75th 6, 95th 10, greatest 17
- Each value and its runs: 0: 1, 1: 20, 2: 34, 3: 174, 4: 227, 5: 204, 6: 147, 7: 79, 8: 42, 9: 21, 10: 22, 11: 15, 12: 11, 13: 1, 14: 1, 17: 1

## Result on the objectives scale (reported, not judged)

Seen from the Allied side, runs of each result:

- decisive victory: 0 of 1000
- major victory: 2 of 1000
- minor victory: 20 of 1000
- stalemate: 162 of 1000
- minor defeat: 242 of 1000
- major defeat: 426 of 1000
- decisive defeat: 148 of 1000

Allied margin over 1000 runs: 5th -18, median -8, 95th 0

## Notes

- Percentiles are by nearest rank: the value at place ceil(p x n / 100) of the n kept runs, sorted. The record is inside the middle 90 percent if any part of it lies from the 5th percentile to the 95th, ends included. The verdict is "agrees, at the edge" where a measure agrees and no part of its record lies from the 25th percentile to the 75th, ends included; it still agrees. A run holds the record if its value is from the low end to the high end, ends included.
- Hits are counted as rolled: a salvo is rolled at the ships afloat when its round starts, so a hit on a ship another salvo sinks in the same round is counted (decision 0062).
- Escorts are matched to a raid's first strike group: an escort entry is added once, when its day, pulse and first group are those of an attack the measure counts (decision 0062).
