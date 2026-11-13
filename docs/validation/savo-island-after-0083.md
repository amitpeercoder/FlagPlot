# savo-island: validation of batch `after-0083`

Battle: savo-island
Scenario: data/scenarios/04_savo_island.toml
Batch: label `after-0083`, 1000 runs, seeds 1 to 1000, file hash cfe74b0f88888f24
Inputs: state hash layout flagplot-state-v7, scenario file hash 5df521f430c89637
- data/scenarios/../map/south_pacific.csv ba6cb3aea5cc45f5
- data/scenarios/../reference/ship_classes_savo_island.csv 6ad083fca06fad91
- data/scenarios/../reference/port_sizes.csv da12e2b751976274
- data/scenarios/../reference/aircraft_types.csv 92830cbacc0a4328
- data/scenarios/../reference/search.toml 9d9cd2961a30d99a
- data/scenarios/../reference/surface_combat.toml 1a9a0ae06d7282b1
Measures: data/validation/savo_island.toml (hash bb48a81f36e3f353), 32 measures, 29 judged

Judged measures: 21 agree, 4 agree at the edge, 4 miss, of 29.

**The battle does not pass:** 4 of 29 judged measures miss.

| Measure | Record | 5th to 95th | Median | Runs holding the record | Left out | Verdict |
| --- | --- | --- | --- | --- | --- | --- |
| allied_cruisers_sunk | 4 | 1 to 5 | 3 | 220 of 1000 | 0 | agrees |
| southern_group_cruisers_sunk | 1 | 0 to 2 | 1 | 381 of 1000 | 0 | agrees |
| chicago_sunk | 0 | 0 to 1 | 1 | 435 of 1000 | 0 | agrees |
| northern_group_cruisers_sunk | 3 | 0 to 3 | 2 | 424 of 1000 | 0 | agrees |
| allied_destroyers_sunk | 0 | 0 to 2 | 0 | 754 of 1000 | 0 | agrees |
| japanese_ships_sunk | 0 | 0 to 1 | 0 | 877 of 1000 | 0 | agrees |
| japanese_ships_with_propulsion_out | 0 | 0 to 1 | 0 | 877 of 1000 | 0 | agrees |
| japanese_ships_sunk_or_stopped | 0 | 0 to 1 | 0 | 767 of 1000 | 0 | agrees |
| night_fights_the_japanese_did_not_sight_first | 0 | 0 to 2 | 1 | 335 of 1000 | 0 | agrees |
| southern_group_rounds_surprised | 1 to 2 | 0 to 1 | 1 | 577 of 1000 | 0 | reported |
| northern_group_rounds_surprised | 2 to 3 | 0 to 1 | 1 | 0 of 1000 | 0 | reported |
| gun_hits_on_allied_ships | 153 to 219 | 43 to 160 | 109 | 86 of 1000 | 0 | agrees, at the edge |
| gun_hits_on_vincennes | 57 to 74 | 4 to 44 | 27 | 1 of 1000 | 0 | **MISS**: the record is outside the middle 90 percent; fewer than 1 run in 20 holds the record |
| gun_hits_on_quincy | 36 to 54 | 1 to 41 | 24 | 156 of 1000 | 0 | agrees, at the edge |
| gun_hits_on_astoria | 34 to 65 | 1 to 49 | 21 | 314 of 1000 | 0 | agrees |
| gun_hits_on_japanese_ships | 10 to 17 | 6 to 72 | 24 | 193 of 1000 | 0 | agrees |
| torpedo_hits_on_allied_ships | 4 to 9 | 1 to 8 | 4 | 560 of 1000 | 0 | agrees |
| torpedo_hits_on_the_southern_group | 1 to 3 | 1 to 6 | 3 | 571 of 1000 | 0 | agrees |
| torpedo_hits_on_the_northern_group | 3 to 6 | 0 to 3 | 0 | 93 of 1000 | 0 | agrees, at the edge |
| torpedo_hits_on_japanese_ships | 0 | 0 to 4 | 1 | 303 of 1000 | 0 | agrees |
| allied_torpedoes_fired | 4 to 6 | 16 to 44 | 26 | 0 of 1000 | 0 | **MISS**: the record is outside the middle 90 percent; fewer than 1 run in 20 holds the record |
| japanese_gun_hits_per_thousand_shells | 81 to 117 | 51 to 91 | 74 | 274 of 1000 | 0 | agrees |
| allied_gun_hits_per_thousand_shells | 21 to 36 | 21 to 65 | 44 | 256 of 1000 | 0 | agrees |
| japanese_main_battery_shells_fired_in_hundreds | 10 to 11 | 7 to 19 | 14 | 101 of 1000 | 0 | agrees, at the edge |
| allied_main_battery_shells_fired_in_hundreds | 3 | 2 to 13 | 5 | 116 of 1000 | 0 | agrees |
| southern_group_main_battery_shells_fired_in_tens | 3 to 5 | 0 to 47 | 9 | 25 of 1000 | 0 | **MISS**: fewer than 1 run in 20 holds the record |
| northern_group_main_battery_shells_fired_in_hundreds | 3 | 1 to 9 | 4 | 179 of 1000 | 0 | agrees |
| tenryu_and_yubari_main_battery_shells_fired_in_hundreds | 1 | 2 to 4 | 3 | 35 of 1000 | 0 | **MISS**: the record is outside the middle 90 percent; fewer than 1 run in 20 holds the record |
| minutes_of_action | 32 to 36 | 30 to 65 | 40 | 224 of 1000 | 0 | agrees |
| surface_fights_by_day | 0 | 0 to 4 | 0 | 803 of 1000 | 0 | agrees |
| japanese_ships_claimed_sunk_per_hundred_sunk | 350 | 100 to 200 | 100 | 2 of 957 | 43 | reported |
| allied_ships_claimed_sunk | 0 | 0 to 1 | 0 | 844 of 1000 | 0 | agrees |

## Misses

### gun_hits_on_vincennes

- Counts: Gun hits on USS Vincennes, all calibres
- Record: 57 to 74 (rows 58, 59, 60)
- Verdict: **MISS**: the record is outside the middle 90 percent; fewer than 1 run in 20 holds the record
- Spread over 1000 runs kept (0 left out): least 0, 5th 4, 25th 16, median 27, 75th 36, 95th 44, greatest 57

### allied_torpedoes_fired

- Counts: Torpedoes the two Allied groups fire in the run
- Record: 4 to 6 (rows 46, 47, 57, 3211, 30, 180, 3179, 3180)
- Verdict: **MISS**: the record is outside the middle 90 percent; fewer than 1 run in 20 holds the record
- Spread over 1000 runs kept (0 left out): least 8, 5th 16, 25th 16, median 26, 75th 36, 95th 44, greatest 56

### southern_group_main_battery_shells_fired_in_tens

- Counts: Main battery shots the four ships of the southern group fire in the run, in tens, rounded down
- Record: 3 to 5 (rows 27, 41, 48, 53, 54)
- Verdict: **MISS**: fewer than 1 run in 20 holds the record
- Spread over 1000 runs kept (0 left out): least 0, 5th 0, 25th 0, median 9, 75th 24, 95th 47, greatest 80

### tenryu_and_yubari_main_battery_shells_fired_in_hundreds

- Counts: Main battery shots Tenryu and Yubari fire in the run, in hundreds, rounded down
- Record: 1 (rows 3, 148, 154)
- Verdict: **MISS**: the record is outside the middle 90 percent; fewer than 1 run in 20 holds the record
- Spread over 1000 runs kept (0 left out): least 0, 5th 2, 25th 3, median 3, 75th 4, 95th 4, greatest 6
- Each value and its runs: 0: 2, 1: 35, 2: 126, 3: 531, 4: 256, 5: 49, 6: 1

## At the edge

### gun_hits_on_allied_ships

- Counts: Gun hits on the nine Allied ships, all calibres
- Record: 153 to 219 (rows 19, 20, 37, 50, 58, 59, 60, 69, 70, 71, 82, 83, 49, 93, 96)
- Verdict: agrees, at the edge
- Spread over 1000 runs kept (0 left out): least 7, 5th 43, 25th 84, median 109, 75th 130, 95th 160, greatest 216

### gun_hits_on_quincy

- Counts: Gun hits on USS Quincy, all calibres
- Record: 36 to 54 (rows 69, 70, 71)
- Verdict: agrees, at the edge
- Spread over 1000 runs kept (0 left out): least 0, 5th 1, 25th 12, median 24, 75th 31, 95th 41, greatest 55

### torpedo_hits_on_the_northern_group

- Counts: Torpedo hits on the five ships of the northern group
- Record: 3 to 6 (rows 61, 62, 63, 72, 73, 85)
- Verdict: agrees, at the edge
- Spread over 1000 runs kept (0 left out): least 0, 5th 0, 25th 0, median 0, 75th 1, 95th 3, greatest 8
- Each value and its runs: 0: 636, 1: 147, 2: 121, 3: 55, 4: 28, 5: 7, 6: 3, 7: 2, 8: 1

### japanese_main_battery_shells_fired_in_hundreds

- Counts: Main battery shots the Japanese force fires in the run, in hundreds, rounded down
- Record: 10 to 11 (rows 1, 2, 3)
- Verdict: agrees, at the edge
- Spread over 1000 runs kept (0 left out): least 2, 5th 7, 25th 12, median 14, 75th 17, 95th 19, greatest 26

## Result on the objectives scale (reported, not judged)

Seen from the Allied side, runs of each result:

- decisive victory: 0 of 1000
- major victory: 3 of 1000
- minor victory: 13 of 1000
- stalemate: 72 of 1000
- minor defeat: 138 of 1000
- major defeat: 367 of 1000
- decisive defeat: 407 of 1000

Allied margin over 1000 runs: 5th -21, median -13, 95th 0

## Notes

- Percentiles are by nearest rank: the value at place ceil(p x n / 100) of the n kept runs, sorted. The record is inside the middle 90 percent if any part of it lies from the 5th percentile to the 95th, ends included. The verdict is "agrees, at the edge" where a measure agrees and no part of its record lies from the 25th percentile to the 75th, ends included; it still agrees. A run holds the record if its value is from the low end to the high end, ends included.
- Hits are counted as rolled: a salvo is rolled at the ships afloat when its round starts, so a hit on a ship another salvo sinks in the same round is counted (decision 0062).
- Escorts are matched to a raid's first strike group: an escort entry is added once, when its day, pulse and first group are those of an attack the measure counts (decision 0062).
