# savo-island: validation of batch `baseline`

Battle: savo-island
Scenario: data/scenarios/04_savo_island.toml
Batch: label `baseline`, 1000 runs, seeds 1 to 1000, file hash f771a08ac56ee603
Inputs: state hash layout flagplot-state-v5, scenario file hash a828cd46c317534b
- data/scenarios/../map/south_pacific.csv ba6cb3aea5cc45f5
- data/scenarios/../reference/ship_classes_savo_island.csv 922ea7c389107084
- data/scenarios/../reference/port_sizes.csv da12e2b751976274
- data/scenarios/../reference/aircraft_types.csv 92830cbacc0a4328
- data/scenarios/../reference/search.toml 9d9cd2961a30d99a
- data/scenarios/../reference/surface_combat.toml 485cfd1b8ba0d4f2
Measures: data/validation/savo_island.toml (hash b7bc786ed7fd90cd), 17 measures, 16 judged

Judged measures: 7 agree, 4 agree at the edge, 5 miss, of 16.

**The battle does not pass:** 5 of 16 judged measures miss.

| Measure | Record | 5th to 95th | Median | Runs holding the record | Left out | Verdict |
| --- | --- | --- | --- | --- | --- | --- |
| allied_cruisers_sunk | 4 | 0 to 1 | 0 | 0 of 1000 | 0 | **MISS**: the record is outside the middle 90 percent; fewer than 1 run in 20 holds the record |
| southern_group_cruisers_sunk | 1 | 0 to 1 | 0 | 256 of 1000 | 0 | agrees |
| northern_group_cruisers_sunk | 3 | 0 to 1 | 0 | 0 of 1000 | 0 | **MISS**: the record is outside the middle 90 percent; fewer than 1 run in 20 holds the record |
| allied_destroyers_sunk | 0 | 0 to 4 | 2 | 50 of 1000 | 0 | agrees, at the edge |
| japanese_ships_sunk | 0 | 0 to 4 | 2 | 275 of 1000 | 0 | agrees |
| japanese_ships_with_propulsion_out | 0 | 0 to 1 | 0 | 940 of 1000 | 0 | agrees |
| gun_hits_on_allied_ships | 153 to 219 | 23 to 153 | 91 | 54 of 1000 | 0 | agrees, at the edge |
| gun_hits_on_japanese_ships | 10 to 17 | 8 to 224 | 127 | 66 of 1000 | 0 | agrees, at the edge |
| torpedo_hits_on_allied_ships | 4 to 9 | 0 to 6 | 3 | 297 of 1000 | 0 | agrees |
| japanese_gun_hits_per_thousand_shells | 81 to 117 | 24 to 51 | 39 | 0 of 1000 | 0 | **MISS**: the record is outside the middle 90 percent; fewer than 1 run in 20 holds the record |
| allied_gun_hits_per_thousand_shells | 21 to 36 | 16 to 53 | 40 | 300 of 994 | 6 | agrees |
| japanese_gun_shells_fired_in_hundreds | 18 | 8 to 34 | 23 | 19 of 1000 | 0 | **MISS**: fewer than 1 run in 20 holds the record |
| allied_gun_shells_fired_in_hundreds | 4 | 4 to 50 | 29 | 21 of 1000 | 0 | **MISS**: fewer than 1 run in 20 holds the record |
| minutes_of_action | 20 to 30 | 15 to 50 | 30 | 495 of 1000 | 0 | agrees |
| surface_fights_by_day | 0 | 0 to 4 | 1 | 358 of 1000 | 0 | agrees |
| japanese_ships_claimed_sunk_per_hundred_sunk | 350 | 100 to 300 | 100 | 1 of 957 | 43 | reported |
| allied_ships_claimed_sunk | 0 | 0 to 6 | 2 | 240 of 1000 | 0 | agrees, at the edge |

## Misses

### allied_cruisers_sunk

- Counts: Allied cruisers sunk at the end of the run, of the five
- Record: 4 (rows 761, 762, 763, 764, 765, 768, 769)
- Verdict: **MISS**: the record is outside the middle 90 percent; fewer than 1 run in 20 holds the record
- Spread over 1000 runs kept (0 left out): least 0, 5th 0, 25th 0, median 0, 75th 1, 95th 1, greatest 3
- Each value and its runs: 0: 681, 1: 277, 2: 40, 3: 2

### northern_group_cruisers_sunk

- Counts: Cruisers of the northern group sunk at the end of the run, of the three
- Record: 3 (rows 762, 763, 764)
- Verdict: **MISS**: the record is outside the middle 90 percent; fewer than 1 run in 20 holds the record
- Spread over 1000 runs kept (0 left out): least 0, 5th 0, 25th 0, median 0, 75th 0, 95th 1, greatest 2
- Each value and its runs: 0: 948, 1: 51, 2: 1

### japanese_gun_hits_per_thousand_shells

- Counts: Gun hits scored by the Japanese force for each 1,000 gun shots it fires, all calibres, rounded down
- Record: 81 to 117 (rows 7, 19, 37, 50, 58, 69, 83, 60, 71, 82, 9)
- Verdict: **MISS**: the record is outside the middle 90 percent; fewer than 1 run in 20 holds the record
- Spread over 1000 runs kept (0 left out): least 13, 5th 24, 25th 32, median 39, 75th 44, 95th 51, greatest 62

### japanese_gun_shells_fired_in_hundreds

- Counts: Gun shots the Japanese force fires in the run, all calibres, in hundreds, rounded down
- Record: 18 (rows 7)
- Verdict: **MISS**: fewer than 1 run in 20 holds the record
- Spread over 1000 runs kept (0 left out): least 5, 5th 8, 25th 15, median 23, 75th 25, 95th 34, greatest 46

### allied_gun_shells_fired_in_hundreds

- Counts: Gun shots the two Allied groups fire in the run, all calibres, in hundreds, rounded down
- Record: 4 (rows 10)
- Verdict: **MISS**: fewer than 1 run in 20 holds the record
- Spread over 1000 runs kept (0 left out): least 0, 5th 4, 25th 18, median 29, 75th 37, 95th 50, greatest 65

## At the edge

### allied_destroyers_sunk

- Counts: Allied destroyers sunk at the end of the run, of the four
- Record: 0 (rows 770, 773, 774, 775)
- Verdict: agrees, at the edge
- Spread over 1000 runs kept (0 left out): least 0, 5th 0, 25th 1, median 2, 75th 3, 95th 4, greatest 4
- Each value and its runs: 0: 50, 1: 211, 2: 345, 3: 202, 4: 192

### gun_hits_on_allied_ships

- Counts: Gun hits on the nine Allied ships, all calibres
- Record: 153 to 219 (rows 19, 20, 37, 50, 58, 59, 60, 69, 70, 71, 82, 83, 49, 93, 96)
- Verdict: agrees, at the edge
- Spread over 1000 runs kept (0 left out): least 8, 5th 23, 25th 51, median 91, 75th 122, 95th 153, greatest 210

### gun_hits_on_japanese_ships

- Counts: Gun hits on the eight Japanese ships, all calibres
- Record: 10 to 17 (rows 11, 114, 115, 125, 140, 141, 151, 156, 132, 147, 157)
- Verdict: agrees, at the edge
- Spread over 1000 runs kept (0 left out): least 0, 5th 8, 25th 64, median 127, 75th 168, 95th 224, greatest 308

### allied_ships_claimed_sunk

- Counts: Japanese ships the Allied side claims sunk
- Record: 0 (rows 749, 751)
- Verdict: agrees, at the edge
- Spread over 1000 runs kept (0 left out): least 0, 5th 0, 25th 1, median 2, 75th 4, 95th 6, greatest 12
- Each value and its runs: 0: 240, 1: 142, 2: 185, 3: 146, 4: 135, 5: 68, 6: 40, 7: 27, 8: 7, 9: 5, 10: 3, 11: 1, 12: 1

## Result on the objectives scale (reported, not judged)

Seen from the Allied side, runs of each result:

- decisive victory: 31 of 1000
- major victory: 201 of 1000
- minor victory: 209 of 1000
- stalemate: 251 of 1000
- minor defeat: 188 of 1000
- major defeat: 119 of 1000
- decisive defeat: 1 of 1000

Allied margin over 1000 runs: 5th -10, median 2, 95th 14.5

## Notes

- Percentiles are by nearest rank: the value at place ceil(p x n / 100) of the n kept runs, sorted. The record is inside the middle 90 percent if any part of it lies from the 5th percentile to the 95th, ends included. The verdict is "agrees, at the edge" where a measure agrees and no part of its record lies from the 25th percentile to the 75th, ends included; it still agrees. A run holds the record if its value is from the low end to the high end, ends included.
- Hits are counted as rolled: a salvo is rolled at the ships afloat when its round starts, so a hit on a ship another salvo sinks in the same round is counted (decision 0062).
- Escorts are matched to a raid's first strike group: an escort entry is added once, when its day, pulse and first group are those of an attack the measure counts (decision 0062).
