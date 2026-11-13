# savo-island: validation of batch `r1`

Battle: savo-island
Scenario: data/scenarios/04_savo_island.toml
Batch: label `r1`, 1000 runs, seeds 1 to 1000, file hash 2c8f6814a9be0dc6
Inputs: state hash layout flagplot-state-v6, scenario file hash 5df521f430c89637
- data/scenarios/../map/south_pacific.csv ba6cb3aea5cc45f5
- data/scenarios/../reference/ship_classes_savo_island.csv dd1b9196736f2f7b
- data/scenarios/../reference/port_sizes.csv da12e2b751976274
- data/scenarios/../reference/aircraft_types.csv 92830cbacc0a4328
- data/scenarios/../reference/search.toml 9d9cd2961a30d99a
- data/scenarios/../reference/surface_combat.toml a020bdae075e8349
Measures: data/validation/savo_island.toml (hash e6d489dd6cd897d1), 26 measures, 23 judged

Judged measures: 17 agree, 2 agree at the edge, 4 miss, of 23.

**The battle does not pass:** 4 of 23 judged measures miss.

| Measure | Record | 5th to 95th | Median | Runs holding the record | Left out | Verdict |
| --- | --- | --- | --- | --- | --- | --- |
| allied_cruisers_sunk | 4 | 0 to 5 | 2 | 204 of 1000 | 0 | agrees |
| southern_group_cruisers_sunk | 1 | 0 to 2 | 1 | 467 of 1000 | 0 | agrees |
| northern_group_cruisers_sunk | 3 | 0 to 3 | 1 | 302 of 1000 | 0 | agrees |
| allied_destroyers_sunk | 0 | 0 to 1 | 0 | 924 of 1000 | 0 | agrees |
| japanese_ships_sunk | 0 | 0 to 1 | 0 | 843 of 1000 | 0 | agrees |
| japanese_ships_with_propulsion_out | 0 | 0 to 1 | 0 | 871 of 1000 | 0 | agrees |
| japanese_ships_sunk_or_stopped | 0 | 0 to 1 | 0 | 728 of 1000 | 0 | agrees |
| night_fights_the_japanese_did_not_sight_first | 0 | 0 to 2 | 1 | 335 of 1000 | 0 | agrees |
| southern_group_rounds_surprised | 1 to 2 | 0 to 1 | 1 | 577 of 1000 | 0 | reported |
| northern_group_rounds_surprised | 2 to 3 | 0 to 1 | 1 | 0 of 1000 | 0 | reported |
| gun_hits_on_allied_ships | 153 to 219 | 35 to 151 | 108 | 42 of 1000 | 0 | **MISS**: the record is outside the middle 90 percent; fewer than 1 run in 20 holds the record |
| gun_hits_on_japanese_ships | 10 to 17 | 9 to 69 | 25 | 231 of 1000 | 0 | agrees |
| torpedo_hits_on_allied_ships | 4 to 9 | 1 to 6 | 3 | 410 of 1000 | 0 | agrees |
| torpedo_hits_on_japanese_ships | 0 | 0 to 4 | 1 | 289 of 1000 | 0 | agrees |
| allied_torpedoes_fired | 4 to 6 | 16 to 36 | 36 | 0 of 1000 | 0 | **MISS**: the record is outside the middle 90 percent; fewer than 1 run in 20 holds the record |
| japanese_gun_hits_per_thousand_shells | 81 to 117 | 48 to 86 | 71 | 157 of 1000 | 0 | agrees, at the edge |
| allied_gun_hits_per_thousand_shells | 21 to 36 | 23 to 62 | 42 | 302 of 1000 | 0 | agrees |
| japanese_main_battery_shells_fired_in_hundreds | 10 to 11 | 7 to 19 | 14 | 101 of 1000 | 0 | agrees |
| allied_main_battery_shells_fired_in_hundreds | 3 | 3 to 13 | 5 | 154 of 1000 | 0 | agrees, at the edge |
| southern_group_main_battery_shells_fired_in_hundreds | 0 | 0 to 4 | 0 | 575 of 1000 | 0 | agrees |
| northern_group_main_battery_shells_fired_in_hundreds | 3 | 2 to 9 | 4 | 217 of 1000 | 0 | agrees |
| tenryu_and_yubari_main_battery_shells_fired_in_hundreds | 1 | 2 to 5 | 3 | 27 of 1000 | 0 | **MISS**: the record is outside the middle 90 percent; fewer than 1 run in 20 holds the record |
| minutes_of_action | 20 to 30 | 35 to 65 | 40 | 17 of 1000 | 0 | **MISS**: the record is outside the middle 90 percent; fewer than 1 run in 20 holds the record |
| surface_fights_by_day | 0 | 0 to 4 | 0 | 788 of 1000 | 0 | agrees |
| japanese_ships_claimed_sunk_per_hundred_sunk | 350 | 100 to 300 | 100 | 1 of 931 | 69 | reported |
| allied_ships_claimed_sunk | 0 | 0 to 1 | 0 | 814 of 1000 | 0 | agrees |

## Misses

### gun_hits_on_allied_ships

- Counts: Gun hits on the nine Allied ships, all calibres
- Record: 153 to 219 (rows 19, 20, 37, 50, 58, 59, 60, 69, 70, 71, 82, 83, 49, 93, 96)
- Verdict: **MISS**: the record is outside the middle 90 percent; fewer than 1 run in 20 holds the record
- Spread over 1000 runs kept (0 left out): least 8, 5th 35, 25th 77, median 108, 75th 130, 95th 151, greatest 190

### allied_torpedoes_fired

- Counts: Torpedoes the two Allied groups fire in the run
- Record: 4 to 6 (rows 46, 47, 57, 3211, 30, 180, 3179, 3180)
- Verdict: **MISS**: the record is outside the middle 90 percent; fewer than 1 run in 20 holds the record
- Spread over 1000 runs kept (0 left out): least 16, 5th 16, 25th 16, median 36, 75th 36, 95th 36, greatest 56
- Each value and its runs: 16: 357, 20: 1, 32: 97, 34: 21, 36: 479, 48: 31, 52: 5, 56: 9

### tenryu_and_yubari_main_battery_shells_fired_in_hundreds

- Counts: Main battery shots Tenryu and Yubari fire in the run, in hundreds, rounded down
- Record: 1 (rows 3, 148, 154)
- Verdict: **MISS**: the record is outside the middle 90 percent; fewer than 1 run in 20 holds the record
- Spread over 1000 runs kept (0 left out): least 0, 5th 2, 25th 3, median 3, 75th 4, 95th 5, greatest 5
- Each value and its runs: 0: 5, 1: 27, 2: 99, 3: 616, 4: 185, 5: 68

### minutes_of_action

- Counts: Minutes of surface fighting in the run: the rounds each fight ran, summed, times the minutes a round stands for
- Record: 20 to 30 (rows 14, 15, 16)
- Verdict: **MISS**: the record is outside the middle 90 percent; fewer than 1 run in 20 holds the record
- Spread over 1000 runs kept (0 left out): least 15, 5th 35, 25th 35, median 40, 75th 50, 95th 65, greatest 95
- Each value and its runs: 15: 3, 20: 4, 25: 3, 30: 10, 35: 330, 40: 254, 45: 144, 50: 53, 55: 66, 60: 70, 65: 26, 70: 18, 75: 10, 80: 6, 85: 2, 95: 1

## At the edge

### japanese_gun_hits_per_thousand_shells

- Counts: Gun hits scored by the Japanese force for each 1,000 gun shots it fires, all calibres, rounded down
- Record: 81 to 117 (rows 7, 19, 37, 50, 58, 69, 83, 60, 71, 82, 9)
- Verdict: agrees, at the edge
- Spread over 1000 runs kept (0 left out): least 24, 5th 48, 25th 62, median 71, 75th 78, 95th 86, greatest 96

### allied_main_battery_shells_fired_in_hundreds

- Counts: Main battery shots the two Allied groups fire in the run, in hundreds, rounded down
- Record: 3 (rows 12, 53, 54, 94, 97)
- Verdict: agrees, at the edge
- Spread over 1000 runs kept (0 left out): least 1, 5th 3, 25th 4, median 5, 75th 8, 95th 13, greatest 22

## Result on the objectives scale (reported, not judged)

Seen from the Allied side, runs of each result:

- decisive victory: 0 of 1000
- major victory: 1 of 1000
- minor victory: 17 of 1000
- stalemate: 110 of 1000
- minor defeat: 192 of 1000
- major defeat: 368 of 1000
- decisive defeat: 312 of 1000

Allied margin over 1000 runs: 5th -20, median -10, 95th 0

## Notes

- Percentiles are by nearest rank: the value at place ceil(p x n / 100) of the n kept runs, sorted. The record is inside the middle 90 percent if any part of it lies from the 5th percentile to the 95th, ends included. The verdict is "agrees, at the edge" where a measure agrees and no part of its record lies from the 25th percentile to the 75th, ends included; it still agrees. A run holds the record if its value is from the low end to the high end, ends included.
- Hits are counted as rolled: a salvo is rolled at the ships afloat when its round starts, so a hit on a ship another salvo sinks in the same round is counted (decision 0062).
- Escorts are matched to a raid's first strike group: an escort entry is added once, when its day, pulse and first group are those of an attack the measure counts (decision 0062).
