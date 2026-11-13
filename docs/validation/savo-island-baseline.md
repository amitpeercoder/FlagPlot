# savo-island: validation of batch `baseline`

Battle: savo-island
Scenario: data/scenarios/04_savo_island.toml
Batch: label `baseline`, 1000 runs, seeds 1 to 1000, file hash d03c342aa3c52517
Inputs: state hash layout flagplot-state-v6, scenario file hash 44289f3b31746674
- data/scenarios/../map/south_pacific.csv ba6cb3aea5cc45f5
- data/scenarios/../reference/ship_classes_savo_island.csv 41ffdbbdc3ec42b9
- data/scenarios/../reference/port_sizes.csv da12e2b751976274
- data/scenarios/../reference/aircraft_types.csv 92830cbacc0a4328
- data/scenarios/../reference/search.toml 9d9cd2961a30d99a
- data/scenarios/../reference/surface_combat.toml d51cd2bd52324e8e
Measures: data/validation/savo_island.toml (hash e6d489dd6cd897d1), 26 measures, 23 judged

Judged measures: 13 agree, 3 agree at the edge, 7 miss, of 23.

**The battle does not pass:** 7 of 23 judged measures miss.

| Measure | Record | 5th to 95th | Median | Runs holding the record | Left out | Verdict |
| --- | --- | --- | --- | --- | --- | --- |
| allied_cruisers_sunk | 4 | 0 to 2 | 1 | 3 of 1000 | 0 | **MISS**: the record is outside the middle 90 percent; fewer than 1 run in 20 holds the record |
| southern_group_cruisers_sunk | 1 | 0 to 1 | 0 | 257 of 1000 | 0 | agrees |
| northern_group_cruisers_sunk | 3 | 0 to 2 | 0 | 1 of 1000 | 0 | **MISS**: the record is outside the middle 90 percent; fewer than 1 run in 20 holds the record |
| allied_destroyers_sunk | 0 | 1 to 4 | 3 | 24 of 1000 | 0 | **MISS**: the record is outside the middle 90 percent; fewer than 1 run in 20 holds the record |
| japanese_ships_sunk | 0 | 0 to 1 | 0 | 868 of 1000 | 0 | agrees |
| japanese_ships_with_propulsion_out | 0 | 0 to 1 | 0 | 847 of 1000 | 0 | agrees |
| japanese_ships_sunk_or_stopped | 0 | 0 to 1 | 0 | 734 of 1000 | 0 | agrees |
| night_fights_the_japanese_did_not_sight_first | 0 | 0 to 2 | 1 | 204 of 1000 | 0 | agrees, at the edge |
| southern_group_rounds_surprised | 1 to 2 | 0 to 1 | 0 | 445 of 1000 | 0 | reported |
| northern_group_rounds_surprised | 2 to 3 | 0 to 1 | 0 | 0 of 1000 | 0 | reported |
| gun_hits_on_allied_ships | 153 to 219 | 32 to 137 | 95 | 9 of 1000 | 0 | **MISS**: the record is outside the middle 90 percent; fewer than 1 run in 20 holds the record |
| gun_hits_on_japanese_ships | 10 to 17 | 4 to 52 | 17 | 249 of 1000 | 0 | agrees |
| torpedo_hits_on_allied_ships | 4 to 9 | 1 to 7 | 3 | 428 of 1000 | 0 | agrees |
| torpedo_hits_on_japanese_ships | 0 | 0 to 4 | 1 | 292 of 1000 | 0 | agrees |
| allied_torpedoes_fired | 4 to 6 | 12 to 36 | 32 | 11 of 1000 | 0 | **MISS**: the record is outside the middle 90 percent; fewer than 1 run in 20 holds the record |
| japanese_gun_hits_per_thousand_shells | 81 to 117 | 35 to 68 | 54 | 0 of 1000 | 0 | **MISS**: the record is outside the middle 90 percent; fewer than 1 run in 20 holds the record |
| allied_gun_hits_per_thousand_shells | 21 to 36 | 13 to 52 | 32 | 487 of 1000 | 0 | agrees |
| japanese_main_battery_shells_fired_in_hundreds | 10 to 11 | 8 to 21 | 17 | 52 of 1000 | 0 | agrees, at the edge |
| allied_main_battery_shells_fired_in_hundreds | 3 | 2 to 11 | 5 | 129 of 1000 | 0 | agrees |
| southern_group_main_battery_shells_fired_in_hundreds | 0 | 0 to 3 | 0 | 606 of 1000 | 0 | agrees |
| northern_group_main_battery_shells_fired_in_hundreds | 3 | 1 to 8 | 4 | 174 of 1000 | 0 | agrees |
| tenryu_and_yubari_main_battery_shells_fired_in_hundreds | 1 | 2 to 4 | 3 | 22 of 1000 | 0 | **MISS**: the record is outside the middle 90 percent; fewer than 1 run in 20 holds the record |
| minutes_of_action | 20 to 30 | 30 to 60 | 40 | 96 of 1000 | 0 | agrees, at the edge |
| surface_fights_by_day | 0 | 0 to 4 | 0 | 759 of 1000 | 0 | agrees |
| japanese_ships_claimed_sunk_per_hundred_sunk | 350 | 100 to 266 | 120 | 3 of 981 | 19 | reported |
| allied_ships_claimed_sunk | 0 | 0 to 1 | 0 | 838 of 1000 | 0 | agrees |

## Misses

### allied_cruisers_sunk

- Counts: Allied cruisers sunk at the end of the run, of the five
- Record: 4 (rows 761, 762, 763, 764, 765, 768, 769)
- Verdict: **MISS**: the record is outside the middle 90 percent; fewer than 1 run in 20 holds the record
- Spread over 1000 runs kept (0 left out): least 0, 5th 0, 25th 0, median 1, 75th 1, 95th 2, greatest 5
- Each value and its runs: 0: 471, 1: 366, 2: 128, 3: 31, 4: 3, 5: 1

### northern_group_cruisers_sunk

- Counts: Cruisers of the northern group sunk at the end of the run, of the three
- Record: 3 (rows 762, 763, 764)
- Verdict: **MISS**: the record is outside the middle 90 percent; fewer than 1 run in 20 holds the record
- Spread over 1000 runs kept (0 left out): least 0, 5th 0, 25th 0, median 0, 75th 1, 95th 2, greatest 3
- Each value and its runs: 0: 624, 1: 318, 2: 57, 3: 1

### allied_destroyers_sunk

- Counts: Allied destroyers sunk at the end of the run, of the four
- Record: 0 (rows 770, 773, 774, 775)
- Verdict: **MISS**: the record is outside the middle 90 percent; fewer than 1 run in 20 holds the record
- Spread over 1000 runs kept (0 left out): least 0, 5th 1, 25th 2, median 3, 75th 4, 95th 4, greatest 4
- Each value and its runs: 0: 24, 1: 118, 2: 236, 3: 347, 4: 275

### gun_hits_on_allied_ships

- Counts: Gun hits on the nine Allied ships, all calibres
- Record: 153 to 219 (rows 19, 20, 37, 50, 58, 59, 60, 69, 70, 71, 82, 83, 49, 93, 96)
- Verdict: **MISS**: the record is outside the middle 90 percent; fewer than 1 run in 20 holds the record
- Spread over 1000 runs kept (0 left out): least 8, 5th 32, 25th 68, median 95, 75th 115, 95th 137, greatest 161

### allied_torpedoes_fired

- Counts: Torpedoes the two Allied groups fire in the run
- Record: 4 to 6 (rows 46, 47, 57, 3211, 30, 180, 3179, 3180)
- Verdict: **MISS**: the record is outside the middle 90 percent; fewer than 1 run in 20 holds the record
- Spread over 1000 runs kept (0 left out): least 4, 5th 12, 25th 16, median 32, 75th 36, 95th 36, greatest 56
- Each value and its runs: 4: 11, 8: 33, 12: 55, 16: 275, 18: 1, 20: 13, 22: 1, 24: 19, 26: 3, 28: 27, 32: 74, 34: 1, 36: 479, 40: 4, 48: 2, 56: 2

### japanese_gun_hits_per_thousand_shells

- Counts: Gun hits scored by the Japanese force for each 1,000 gun shots it fires, all calibres, rounded down
- Record: 81 to 117 (rows 7, 19, 37, 50, 58, 69, 83, 60, 71, 82, 9)
- Verdict: **MISS**: the record is outside the middle 90 percent; fewer than 1 run in 20 holds the record
- Spread over 1000 runs kept (0 left out): least 17, 5th 35, 25th 48, median 54, 75th 61, 95th 68, greatest 80

### tenryu_and_yubari_main_battery_shells_fired_in_hundreds

- Counts: Main battery shots Tenryu and Yubari fire in the run, in hundreds, rounded down
- Record: 1 (rows 3, 148, 154)
- Verdict: **MISS**: the record is outside the middle 90 percent; fewer than 1 run in 20 holds the record
- Spread over 1000 runs kept (0 left out): least 0, 5th 2, 25th 3, median 3, 75th 3, 95th 4, greatest 5
- Each value and its runs: 0: 6, 1: 22, 2: 122, 3: 744, 4: 96, 5: 10

## At the edge

### night_fights_the_japanese_did_not_sight_first

- Counts: Surface fights by night in which the Allied force sighted first, or both sighted in the same round, or neither sighted
- Record: 0 (rows 53, 54, 62, 64, 72, 88)
- Verdict: agrees, at the edge
- Spread over 1000 runs kept (0 left out): least 0, 5th 0, 25th 1, median 1, 75th 2, 95th 2, greatest 2
- Each value and its runs: 0: 204, 1: 467, 2: 329

### japanese_main_battery_shells_fired_in_hundreds

- Counts: Main battery shots the Japanese force fires in the run, in hundreds, rounded down
- Record: 10 to 11 (rows 1, 2, 3)
- Verdict: agrees, at the edge
- Spread over 1000 runs kept (0 left out): least 2, 5th 8, 25th 14, median 17, 75th 19, 95th 21, greatest 27

### minutes_of_action

- Counts: Minutes of surface fighting in the run: the rounds each fight ran, summed, times the minutes a round stands for
- Record: 20 to 30 (rows 14, 15, 16)
- Verdict: agrees, at the edge
- Spread over 1000 runs kept (0 left out): least 10, 5th 30, 25th 35, median 40, 75th 45, 95th 60, greatest 105
- Each value and its runs: 10: 1, 15: 8, 20: 2, 25: 31, 30: 63, 35: 372, 40: 197, 45: 85, 50: 43, 55: 77, 60: 73, 65: 25, 70: 14, 75: 2, 80: 3, 85: 2, 90: 1, 105: 1

## Result on the objectives scale (reported, not judged)

Seen from the Allied side, runs of each result:

- decisive victory: 0 of 1000
- major victory: 1 of 1000
- minor victory: 28 of 1000
- stalemate: 185 of 1000
- minor defeat: 383 of 1000
- major defeat: 372 of 1000
- decisive defeat: 31 of 1000

Allied margin over 1000 runs: 5th -14, median -6, 95th 1.5

## Notes

- Percentiles are by nearest rank: the value at place ceil(p x n / 100) of the n kept runs, sorted. The record is inside the middle 90 percent if any part of it lies from the 5th percentile to the 95th, ends included. The verdict is "agrees, at the edge" where a measure agrees and no part of its record lies from the 25th percentile to the 75th, ends included; it still agrees. A run holds the record if its value is from the low end to the high end, ends included.
- Hits are counted as rolled: a salvo is rolled at the ships afloat when its round starts, so a hit on a ship another salvo sinks in the same round is counted (decision 0062).
- Escorts are matched to a raid's first strike group: an escort entry is added once, when its day, pulse and first group are those of an attack the measure counts (decision 0062).
