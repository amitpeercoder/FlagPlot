# savo-island: validation of batch `baseline`

Battle: savo-island
Scenario: data/scenarios/04_savo_island.toml
Batch: label `baseline`, 1000 runs, seeds 1 to 1000, file hash 5a50bf4692994237
Inputs: state hash layout flagplot-state-v6, scenario file hash 153cc0008adfdf4a
- data/scenarios/../map/south_pacific.csv ba6cb3aea5cc45f5
- data/scenarios/../reference/ship_classes_savo_island.csv 922ea7c389107084
- data/scenarios/../reference/port_sizes.csv da12e2b751976274
- data/scenarios/../reference/aircraft_types.csv 92830cbacc0a4328
- data/scenarios/../reference/search.toml 9d9cd2961a30d99a
- data/scenarios/../reference/surface_combat.toml 014bdbf0888f4925
Measures: data/validation/savo_island.toml (hash b28cd60bec957ec3), 20 measures, 19 judged

Judged measures: 10 agree, 1 agree at the edge, 8 miss, of 19.

**The battle does not pass:** 8 of 19 judged measures miss.

| Measure | Record | 5th to 95th | Median | Runs holding the record | Left out | Verdict |
| --- | --- | --- | --- | --- | --- | --- |
| allied_cruisers_sunk | 4 | 0 to 2 | 0 | 2 of 1000 | 0 | **MISS**: the record is outside the middle 90 percent; fewer than 1 run in 20 holds the record |
| southern_group_cruisers_sunk | 1 | 0 to 2 | 0 | 342 of 1000 | 0 | agrees |
| northern_group_cruisers_sunk | 3 | 0 to 1 | 0 | 0 of 1000 | 0 | **MISS**: the record is outside the middle 90 percent; fewer than 1 run in 20 holds the record |
| allied_destroyers_sunk | 0 | 1 to 4 | 3 | 15 of 1000 | 0 | **MISS**: the record is outside the middle 90 percent; fewer than 1 run in 20 holds the record |
| japanese_ships_sunk | 0 | 0 to 3 | 1 | 479 of 1000 | 0 | agrees |
| japanese_ships_with_propulsion_out | 0 | 0 to 1 | 0 | 923 of 1000 | 0 | agrees |
| gun_hits_on_allied_ships | 153 to 219 | 37 to 175 | 121 | 176 of 1000 | 0 | agrees, at the edge |
| gun_hits_on_japanese_ships | 10 to 17 | 2 to 159 | 53 | 32 of 1000 | 0 | **MISS**: fewer than 1 run in 20 holds the record |
| torpedo_hits_on_allied_ships | 4 to 9 | 1 to 7 | 3 | 411 of 1000 | 0 | agrees |
| japanese_gun_hits_per_thousand_shells | 81 to 117 | 33 to 64 | 50 | 0 of 1000 | 0 | **MISS**: the record is outside the middle 90 percent; fewer than 1 run in 20 holds the record |
| allied_gun_hits_per_thousand_shells | 21 to 36 | 11 to 57 | 38 | 326 of 980 | 20 | agrees |
| japanese_gun_shells_fired_in_hundreds | 18 | 10 to 32 | 24 | 26 of 1000 | 0 | **MISS**: fewer than 1 run in 20 holds the record |
| allied_gun_shells_fired_in_hundreds | 4 | 1 to 32 | 13 | 6 of 1000 | 0 | **MISS**: fewer than 1 run in 20 holds the record |
| southern_group_gun_shells_fired_in_hundreds | 0 | 0 to 13 | 4 | 373 of 1000 | 0 | agrees |
| northern_group_gun_shells_fired_in_hundreds | 3 to 4 | 1 to 24 | 8 | 79 of 1000 | 0 | agrees |
| tenryu_and_yubari_gun_shells_fired_in_hundreds | 1 to 2 | 4 to 15 | 9 | 0 of 1000 | 0 | **MISS**: the record is outside the middle 90 percent; fewer than 1 run in 20 holds the record |
| minutes_of_action | 20 to 30 | 15 to 50 | 25 | 458 of 1000 | 0 | agrees |
| surface_fights_by_day | 0 | 0 to 4 | 0 | 518 of 1000 | 0 | agrees |
| japanese_ships_claimed_sunk_per_hundred_sunk | 350 | 100 to 250 | 120 | 2 of 988 | 12 | reported |
| allied_ships_claimed_sunk | 0 | 0 to 4 | 1 | 455 of 1000 | 0 | agrees |

## Misses

### allied_cruisers_sunk

- Counts: Allied cruisers sunk at the end of the run, of the five
- Record: 4 (rows 761, 762, 763, 764, 765, 768, 769)
- Verdict: **MISS**: the record is outside the middle 90 percent; fewer than 1 run in 20 holds the record
- Spread over 1000 runs kept (0 left out): least 0, 5th 0, 25th 0, median 0, 75th 1, 95th 2, greatest 4
- Each value and its runs: 0: 524, 1: 349, 2: 115, 3: 10, 4: 2

### northern_group_cruisers_sunk

- Counts: Cruisers of the northern group sunk at the end of the run, of the three
- Record: 3 (rows 762, 763, 764)
- Verdict: **MISS**: the record is outside the middle 90 percent; fewer than 1 run in 20 holds the record
- Spread over 1000 runs kept (0 left out): least 0, 5th 0, 25th 0, median 0, 75th 0, 95th 1, greatest 2
- Each value and its runs: 0: 873, 1: 119, 2: 8

### allied_destroyers_sunk

- Counts: Allied destroyers sunk at the end of the run, of the four
- Record: 0 (rows 770, 773, 774, 775)
- Verdict: **MISS**: the record is outside the middle 90 percent; fewer than 1 run in 20 holds the record
- Spread over 1000 runs kept (0 left out): least 0, 5th 1, 25th 2, median 3, 75th 4, 95th 4, greatest 4
- Each value and its runs: 0: 15, 1: 99, 2: 269, 3: 302, 4: 315

### gun_hits_on_japanese_ships

- Counts: Gun hits on the eight Japanese ships, all calibres
- Record: 10 to 17 (rows 11, 114, 115, 125, 140, 141, 151, 156, 132, 147, 157)
- Verdict: **MISS**: fewer than 1 run in 20 holds the record
- Spread over 1000 runs kept (0 left out): least 0, 5th 2, 25th 22, median 53, 75th 91, 95th 159, greatest 267

### japanese_gun_hits_per_thousand_shells

- Counts: Gun hits scored by the Japanese force for each 1,000 gun shots it fires, all calibres, rounded down
- Record: 81 to 117 (rows 7, 19, 37, 50, 58, 69, 83, 60, 71, 82, 9)
- Verdict: **MISS**: the record is outside the middle 90 percent; fewer than 1 run in 20 holds the record
- Spread over 1000 runs kept (0 left out): least 22, 5th 33, 25th 43, median 50, 75th 56, 95th 64, greatest 76

### japanese_gun_shells_fired_in_hundreds

- Counts: Gun shots the Japanese force fires in the run, all calibres, in hundreds, rounded down
- Record: 18 (rows 7)
- Verdict: **MISS**: fewer than 1 run in 20 holds the record
- Spread over 1000 runs kept (0 left out): least 5, 5th 10, 25th 19, median 24, 75th 25, 95th 32, greatest 44

### allied_gun_shells_fired_in_hundreds

- Counts: Gun shots the two Allied groups fire in the run, all calibres, in hundreds, rounded down
- Record: 4 (rows 10)
- Verdict: **MISS**: fewer than 1 run in 20 holds the record
- Spread over 1000 runs kept (0 left out): least 0, 5th 1, 25th 7, median 13, 75th 20, 95th 32, greatest 52

### tenryu_and_yubari_gun_shells_fired_in_hundreds

- Counts: Gun shots Tenryu and Yubari fire in the run, all calibres, in hundreds, rounded down
- Record: 1 to 2 (rows 148, 149, 154)
- Verdict: **MISS**: the record is outside the middle 90 percent; fewer than 1 run in 20 holds the record
- Spread over 1000 runs kept (0 left out): least 3, 5th 4, 25th 9, median 9, 75th 12, 95th 15, greatest 21
- Each value and its runs: 3: 49, 4: 20, 5: 8, 6: 87, 7: 24, 8: 17, 9: 487, 10: 31, 11: 18, 12: 173, 13: 18, 14: 7, 15: 42, 16: 9, 17: 2, 18: 6, 20: 1, 21: 1

## At the edge

### gun_hits_on_allied_ships

- Counts: Gun hits on the nine Allied ships, all calibres
- Record: 153 to 219 (rows 19, 20, 37, 50, 58, 59, 60, 69, 70, 71, 82, 83, 49, 93, 96)
- Verdict: agrees, at the edge
- Spread over 1000 runs kept (0 left out): least 18, 5th 37, 25th 87, median 121, 75th 145, 95th 175, greatest 241

## Result on the objectives scale (reported, not judged)

Seen from the Allied side, runs of each result:

- decisive victory: 3 of 1000
- major victory: 78 of 1000
- minor victory: 135 of 1000
- stalemate: 221 of 1000
- minor defeat: 282 of 1000
- major defeat: 271 of 1000
- decisive defeat: 10 of 1000

Allied margin over 1000 runs: 5th -12, median -4, 95th 10

## Notes

- Percentiles are by nearest rank: the value at place ceil(p x n / 100) of the n kept runs, sorted. The record is inside the middle 90 percent if any part of it lies from the 5th percentile to the 95th, ends included. The verdict is "agrees, at the edge" where a measure agrees and no part of its record lies from the 25th percentile to the 75th, ends included; it still agrees. A run holds the record if its value is from the low end to the high end, ends included.
- Hits are counted as rolled: a salvo is rolled at the ships afloat when its round starts, so a hit on a ship another salvo sinks in the same round is counted (decision 0062).
- Escorts are matched to a raid's first strike group: an escort entry is added once, when its day, pulse and first group are those of an attack the measure counts (decision 0062).
