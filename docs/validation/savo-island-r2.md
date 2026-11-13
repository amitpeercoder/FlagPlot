# savo-island: validation of batch `r2`

Battle: savo-island
Scenario: data/scenarios/04_savo_island.toml
Batch: label `r2`, 1000 runs, seeds 1 to 1000, file hash a2b31f37f411250b
Inputs: state hash layout flagplot-state-v6, scenario file hash 5df521f430c89637
- data/scenarios/../map/south_pacific.csv ba6cb3aea5cc45f5
- data/scenarios/../reference/ship_classes_savo_island.csv dd1b9196736f2f7b
- data/scenarios/../reference/port_sizes.csv da12e2b751976274
- data/scenarios/../reference/aircraft_types.csv 92830cbacc0a4328
- data/scenarios/../reference/search.toml 9d9cd2961a30d99a
- data/scenarios/../reference/surface_combat.toml 1a9a0ae06d7282b1
Measures: data/validation/savo_island.toml (hash e6d489dd6cd897d1), 26 measures, 23 judged

Judged measures: 19 agree, 1 agree at the edge, 3 miss, of 23.

**The battle does not pass:** 3 of 23 judged measures miss.

| Measure | Record | 5th to 95th | Median | Runs holding the record | Left out | Verdict |
| --- | --- | --- | --- | --- | --- | --- |
| allied_cruisers_sunk | 4 | 0 to 5 | 3 | 221 of 1000 | 0 | agrees |
| southern_group_cruisers_sunk | 1 | 0 to 2 | 1 | 399 of 1000 | 0 | agrees |
| northern_group_cruisers_sunk | 3 | 0 to 3 | 2 | 368 of 1000 | 0 | agrees |
| allied_destroyers_sunk | 0 | 0 to 1 | 0 | 862 of 1000 | 0 | agrees |
| japanese_ships_sunk | 0 | 0 to 1 | 0 | 840 of 1000 | 0 | agrees |
| japanese_ships_with_propulsion_out | 0 | 0 to 1 | 0 | 862 of 1000 | 0 | agrees |
| japanese_ships_sunk_or_stopped | 0 | 0 to 2 | 0 | 717 of 1000 | 0 | agrees |
| night_fights_the_japanese_did_not_sight_first | 0 | 0 to 2 | 1 | 335 of 1000 | 0 | agrees |
| southern_group_rounds_surprised | 1 to 2 | 0 to 1 | 1 | 577 of 1000 | 0 | reported |
| northern_group_rounds_surprised | 2 to 3 | 0 to 1 | 1 | 0 of 1000 | 0 | reported |
| gun_hits_on_allied_ships | 153 to 219 | 35 to 158 | 116 | 88 of 1000 | 0 | agrees, at the edge |
| gun_hits_on_japanese_ships | 10 to 17 | 8 to 69 | 24 | 227 of 1000 | 0 | agrees |
| torpedo_hits_on_allied_ships | 4 to 9 | 1 to 8 | 4 | 524 of 1000 | 0 | agrees |
| torpedo_hits_on_japanese_ships | 0 | 0 to 4 | 1 | 303 of 1000 | 0 | agrees |
| allied_torpedoes_fired | 4 to 6 | 16 to 48 | 34 | 0 of 1000 | 0 | **MISS**: the record is outside the middle 90 percent; fewer than 1 run in 20 holds the record |
| japanese_gun_hits_per_thousand_shells | 81 to 117 | 49 to 92 | 75 | 331 of 1000 | 0 | agrees |
| allied_gun_hits_per_thousand_shells | 21 to 36 | 22 to 65 | 43 | 271 of 1000 | 0 | agrees |
| japanese_main_battery_shells_fired_in_hundreds | 10 to 11 | 6 to 18 | 14 | 96 of 1000 | 0 | agrees |
| allied_main_battery_shells_fired_in_hundreds | 3 | 2 to 13 | 5 | 187 of 1000 | 0 | agrees |
| southern_group_main_battery_shells_fired_in_hundreds | 0 | 0 to 4 | 0 | 593 of 1000 | 0 | agrees |
| northern_group_main_battery_shells_fired_in_hundreds | 3 | 2 to 9 | 4 | 243 of 1000 | 0 | agrees |
| tenryu_and_yubari_main_battery_shells_fired_in_hundreds | 1 | 2 to 5 | 3 | 37 of 1000 | 0 | **MISS**: the record is outside the middle 90 percent; fewer than 1 run in 20 holds the record |
| minutes_of_action | 20 to 30 | 35 to 65 | 40 | 25 of 1000 | 0 | **MISS**: the record is outside the middle 90 percent; fewer than 1 run in 20 holds the record |
| surface_fights_by_day | 0 | 0 to 4 | 0 | 776 of 1000 | 0 | agrees |
| japanese_ships_claimed_sunk_per_hundred_sunk | 350 | 100 to 300 | 100 | 2 of 937 | 63 | reported |
| allied_ships_claimed_sunk | 0 | 0 to 2 | 0 | 814 of 1000 | 0 | agrees |

## Misses

### allied_torpedoes_fired

- Counts: Torpedoes the two Allied groups fire in the run
- Record: 4 to 6 (rows 46, 47, 57, 3211, 30, 180, 3179, 3180)
- Verdict: **MISS**: the record is outside the middle 90 percent; fewer than 1 run in 20 holds the record
- Spread over 1000 runs kept (0 left out): least 16, 5th 16, 25th 16, median 34, 75th 36, 95th 48, greatest 56
- Each value and its runs: 16: 422, 20: 3, 32: 69, 34: 9, 36: 437, 48: 54, 52: 2, 56: 4

### tenryu_and_yubari_main_battery_shells_fired_in_hundreds

- Counts: Main battery shots Tenryu and Yubari fire in the run, in hundreds, rounded down
- Record: 1 (rows 3, 148, 154)
- Verdict: **MISS**: the record is outside the middle 90 percent; fewer than 1 run in 20 holds the record
- Spread over 1000 runs kept (0 left out): least 0, 5th 2, 25th 3, median 3, 75th 3, 95th 5, greatest 6
- Each value and its runs: 0: 7, 1: 37, 2: 106, 3: 671, 4: 128, 5: 50, 6: 1

### minutes_of_action

- Counts: Minutes of surface fighting in the run: the rounds each fight ran, summed, times the minutes a round stands for
- Record: 20 to 30 (rows 14, 15, 16)
- Verdict: **MISS**: the record is outside the middle 90 percent; fewer than 1 run in 20 holds the record
- Spread over 1000 runs kept (0 left out): least 15, 5th 35, 25th 35, median 40, 75th 45, 95th 65, greatest 95
- Each value and its runs: 15: 4, 20: 7, 25: 6, 30: 12, 35: 384, 40: 218, 45: 122, 50: 51, 55: 70, 60: 64, 65: 26, 70: 20, 75: 9, 80: 4, 85: 2, 95: 1

## At the edge

### gun_hits_on_allied_ships

- Counts: Gun hits on the nine Allied ships, all calibres
- Record: 153 to 219 (rows 19, 20, 37, 50, 58, 59, 60, 69, 70, 71, 82, 83, 49, 93, 96)
- Verdict: agrees, at the edge
- Spread over 1000 runs kept (0 left out): least 6, 5th 35, 25th 80, median 116, 75th 139, 95th 158, greatest 179

## Result on the objectives scale (reported, not judged)

Seen from the Allied side, runs of each result:

- decisive victory: 0 of 1000
- major victory: 5 of 1000
- minor victory: 14 of 1000
- stalemate: 125 of 1000
- minor defeat: 152 of 1000
- major defeat: 282 of 1000
- decisive defeat: 422 of 1000

Allied margin over 1000 runs: 5th -21, median -12, 95th 0

## Notes

- Percentiles are by nearest rank: the value at place ceil(p x n / 100) of the n kept runs, sorted. The record is inside the middle 90 percent if any part of it lies from the 5th percentile to the 95th, ends included. The verdict is "agrees, at the edge" where a measure agrees and no part of its record lies from the 25th percentile to the 75th, ends included; it still agrees. A run holds the record if its value is from the low end to the high end, ends included.
- Hits are counted as rolled: a salvo is rolled at the ships afloat when its round starts, so a hit on a ship another salvo sinks in the same round is counted (decision 0062).
- Escorts are matched to a raid's first strike group: an escort entry is added once, when its day, pulse and first group are those of an attack the measure counts (decision 0062).
