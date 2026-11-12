# savo-island: validation of batch `baseline`

Battle: savo-island
Scenario: data/scenarios/04_savo_island.toml
Batch: label `baseline`, 1000 runs, seeds 1 to 1000, file hash 5461dcd92a978f54
Inputs: state hash layout flagplot-state-v6, scenario file hash 153cc0008adfdf4a
- data/scenarios/../map/south_pacific.csv ba6cb3aea5cc45f5
- data/scenarios/../reference/ship_classes_savo_island.csv 922ea7c389107084
- data/scenarios/../reference/port_sizes.csv da12e2b751976274
- data/scenarios/../reference/aircraft_types.csv 92830cbacc0a4328
- data/scenarios/../reference/search.toml 9d9cd2961a30d99a
- data/scenarios/../reference/surface_combat.toml 14b86fdba9f64bfc
Measures: data/validation/savo_island.toml (hash b7bc786ed7fd90cd), 17 measures, 16 judged

Judged measures: 8 agree, 2 agree at the edge, 6 miss, of 16.

**The battle does not pass:** 6 of 16 judged measures miss.

| Measure | Record | 5th to 95th | Median | Runs holding the record | Left out | Verdict |
| --- | --- | --- | --- | --- | --- | --- |
| allied_cruisers_sunk | 4 | 0 to 2 | 0 | 0 of 1000 | 0 | **MISS**: the record is outside the middle 90 percent; fewer than 1 run in 20 holds the record |
| southern_group_cruisers_sunk | 1 | 0 to 2 | 0 | 332 of 1000 | 0 | agrees |
| northern_group_cruisers_sunk | 3 | 0 to 1 | 0 | 0 of 1000 | 0 | **MISS**: the record is outside the middle 90 percent; fewer than 1 run in 20 holds the record |
| allied_destroyers_sunk | 0 | 1 to 4 | 3 | 29 of 1000 | 0 | **MISS**: the record is outside the middle 90 percent; fewer than 1 run in 20 holds the record |
| japanese_ships_sunk | 0 | 0 to 4 | 1 | 350 of 1000 | 0 | agrees |
| japanese_ships_with_propulsion_out | 0 | 0 to 1 | 0 | 911 of 1000 | 0 | agrees |
| gun_hits_on_allied_ships | 153 to 219 | 27 to 170 | 107 | 149 of 1000 | 0 | agrees, at the edge |
| gun_hits_on_japanese_ships | 10 to 17 | 3 to 221 | 96 | 51 of 1000 | 0 | agrees, at the edge |
| torpedo_hits_on_allied_ships | 4 to 9 | 1 to 7 | 3 | 404 of 1000 | 0 | agrees |
| japanese_gun_hits_per_thousand_shells | 81 to 117 | 28 to 64 | 48 | 0 of 1000 | 0 | **MISS**: the record is outside the middle 90 percent; fewer than 1 run in 20 holds the record |
| allied_gun_hits_per_thousand_shells | 21 to 36 | 12 to 57 | 39 | 321 of 980 | 20 | agrees |
| japanese_gun_shells_fired_in_hundreds | 18 | 7 to 30 | 22 | 23 of 1000 | 0 | **MISS**: fewer than 1 run in 20 holds the record |
| allied_gun_shells_fired_in_hundreds | 4 | 2 to 44 | 25 | 32 of 1000 | 0 | **MISS**: fewer than 1 run in 20 holds the record |
| minutes_of_action | 20 to 30 | 15 to 45 | 25 | 490 of 1000 | 0 | agrees |
| surface_fights_by_day | 0 | 0 to 4 | 1 | 438 of 1000 | 0 | agrees |
| japanese_ships_claimed_sunk_per_hundred_sunk | 350 | 100 to 300 | 100 | 5 of 973 | 27 | reported |
| allied_ships_claimed_sunk | 0 | 0 to 6 | 2 | 311 of 1000 | 0 | agrees |

## Misses

### allied_cruisers_sunk

- Counts: Allied cruisers sunk at the end of the run, of the five
- Record: 4 (rows 761, 762, 763, 764, 765, 768, 769)
- Verdict: **MISS**: the record is outside the middle 90 percent; fewer than 1 run in 20 holds the record
- Spread over 1000 runs kept (0 left out): least 0, 5th 0, 25th 0, median 0, 75th 1, 95th 2, greatest 3
- Each value and its runs: 0: 555, 1: 335, 2: 98, 3: 12

### northern_group_cruisers_sunk

- Counts: Cruisers of the northern group sunk at the end of the run, of the three
- Record: 3 (rows 762, 763, 764)
- Verdict: **MISS**: the record is outside the middle 90 percent; fewer than 1 run in 20 holds the record
- Spread over 1000 runs kept (0 left out): least 0, 5th 0, 25th 0, median 0, 75th 0, 95th 1, greatest 2
- Each value and its runs: 0: 894, 1: 101, 2: 5

### allied_destroyers_sunk

- Counts: Allied destroyers sunk at the end of the run, of the four
- Record: 0 (rows 770, 773, 774, 775)
- Verdict: **MISS**: the record is outside the middle 90 percent; fewer than 1 run in 20 holds the record
- Spread over 1000 runs kept (0 left out): least 0, 5th 1, 25th 2, median 3, 75th 3, 95th 4, greatest 4
- Each value and its runs: 0: 29, 1: 152, 2: 306, 3: 270, 4: 243

### japanese_gun_hits_per_thousand_shells

- Counts: Gun hits scored by the Japanese force for each 1,000 gun shots it fires, all calibres, rounded down
- Record: 81 to 117 (rows 7, 19, 37, 50, 58, 69, 83, 60, 71, 82, 9)
- Verdict: **MISS**: the record is outside the middle 90 percent; fewer than 1 run in 20 holds the record
- Spread over 1000 runs kept (0 left out): least 16, 5th 28, 25th 40, median 48, 75th 55, 95th 64, greatest 76

### japanese_gun_shells_fired_in_hundreds

- Counts: Gun shots the Japanese force fires in the run, all calibres, in hundreds, rounded down
- Record: 18 (rows 7)
- Verdict: **MISS**: fewer than 1 run in 20 holds the record
- Spread over 1000 runs kept (0 left out): least 3, 5th 7, 25th 14, median 22, 75th 24, 95th 30, greatest 37

### allied_gun_shells_fired_in_hundreds

- Counts: Gun shots the two Allied groups fire in the run, all calibres, in hundreds, rounded down
- Record: 4 (rows 10)
- Verdict: **MISS**: fewer than 1 run in 20 holds the record
- Spread over 1000 runs kept (0 left out): least 0, 5th 2, 25th 15, median 25, 75th 33, 95th 44, greatest 67

## At the edge

### gun_hits_on_allied_ships

- Counts: Gun hits on the nine Allied ships, all calibres
- Record: 153 to 219 (rows 19, 20, 37, 50, 58, 59, 60, 69, 70, 71, 82, 83, 49, 93, 96)
- Verdict: agrees, at the edge
- Spread over 1000 runs kept (0 left out): least 6, 5th 27, 25th 59, median 107, 75th 141, 95th 170, greatest 214

### gun_hits_on_japanese_ships

- Counts: Gun hits on the eight Japanese ships, all calibres
- Record: 10 to 17 (rows 11, 114, 115, 125, 140, 141, 151, 156, 132, 147, 157)
- Verdict: agrees, at the edge
- Spread over 1000 runs kept (0 left out): least 0, 5th 3, 25th 49, median 96, 75th 160, 95th 221, greatest 380

## Result on the objectives scale (reported, not judged)

Seen from the Allied side, runs of each result:

- decisive victory: 21 of 1000
- major victory: 164 of 1000
- minor victory: 162 of 1000
- stalemate: 213 of 1000
- minor defeat: 220 of 1000
- major defeat: 210 of 1000
- decisive defeat: 10 of 1000

Allied margin over 1000 runs: 5th -12, median -1, 95th 14

## Notes

- Percentiles are by nearest rank: the value at place ceil(p x n / 100) of the n kept runs, sorted. The record is inside the middle 90 percent if any part of it lies from the 5th percentile to the 95th, ends included. The verdict is "agrees, at the edge" where a measure agrees and no part of its record lies from the 25th percentile to the 75th, ends included; it still agrees. A run holds the record if its value is from the low end to the high end, ends included.
- Hits are counted as rolled: a salvo is rolled at the ships afloat when its round starts, so a hit on a ship another salvo sinks in the same round is counted (decision 0062).
- Escorts are matched to a raid's first strike group: an escort entry is added once, when its day, pulse and first group are those of an attack the measure counts (decision 0062).
