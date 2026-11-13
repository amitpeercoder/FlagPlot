# cape-esperance: validation of batch `check`

Battle: cape-esperance
Scenario: data/scenarios/17_cape_esperance.toml
Batch: label `check`, 1000 runs, seeds 1 to 1000, file hash 100525c6c713b215
Inputs: state hash layout flagplot-state-v8, scenario file hash 6984fa7a6979aa28
- data/scenarios/../map/south_pacific.csv ba6cb3aea5cc45f5
- data/scenarios/../reference/ship_classes_cape_esperance.csv c79b1bc1e17fcc16
- data/scenarios/../reference/port_sizes.csv da12e2b751976274
- data/scenarios/../reference/aircraft_types.csv 92830cbacc0a4328
- data/scenarios/../reference/search.toml 9d9cd2961a30d99a
- data/scenarios/../reference/surface_combat.toml 1a9a0ae06d7282b1
Measures: data/validation/cape_esperance.toml (hash dcda49f98a4431be), 11 measures, 8 judged

Judged measures: 4 agree, 1 agree at the edge, 3 miss, of 8.

**The battle does not pass:** 3 of 8 judged measures miss.

| Measure | Record | 5th to 95th | Median | Runs holding the record | Left out | Verdict |
| --- | --- | --- | --- | --- | --- | --- |
| japanese_ships_sunk | 2 | 0 to 3 | 2 | 407 of 1000 | 0 | agrees |
| japanese_ships_with_propulsion_out | 0 | 0 to 1 | 0 | 908 of 1000 | 0 | agrees |
| us_cruisers_sunk | 0 | 0 to 0 | 0 | 960 of 1000 | 0 | agrees |
| us_destroyers_sunk | 1 | 0 to 0 | 0 | 4 of 1000 | 0 | **MISS**: the record is outside the middle 90 percent; fewer than 1 run in 20 holds the record |
| gun_hits_on_aoba | 40 | 13 to 55 | 29 | 18 of 1000 | 0 | **MISS**: fewer than 1 run in 20 holds the record |
| gun_hits_on_furutaka | 90 | 10 to 57 | 27 | 0 of 1000 | 0 | reported |
| gun_hits_on_kinugasa | 0 to 4 | 4 to 49 | 15 | 60 of 1000 | 0 | agrees, at the edge |
| gun_hits_on_us_ships | 15 to 29 | 1 to 35 | 6 | 179 of 1000 | 0 | reported |
| torpedo_hits_on_japanese_ships | 0 to 1 | 0 to 6 | 3 | 269 of 1000 | 0 | agrees |
| japanese_ships_claimed_sunk_per_hundred_sunk | 200 to 300 | 100 to 300 | 100 | 2 of 44 | 956 | **MISS**: too few runs kept |
| us_ships_claimed_sunk_per_hundred_sunk | 400 to 750 | 100 to 250 | 100 | 8 of 923 | 77 | reported |

## Misses

### us_destroyers_sunk

- Counts: US destroyers sunk at the end of the run, of the five
- Record: 1 (rows 1532, 1533, 1534, 1530)
- Verdict: **MISS**: the record is outside the middle 90 percent; fewer than 1 run in 20 holds the record
- Spread over 1000 runs kept (0 left out): least 0, 5th 0, 25th 0, median 0, 75th 0, 95th 0, greatest 2
- Each value and its runs: 0: 995, 1: 4, 2: 1

### gun_hits_on_aoba

- Counts: Gun hits on Aoba, all calibres
- Record: 40 (rows 1598, 4034, 1599)
- Verdict: **MISS**: fewer than 1 run in 20 holds the record
- Spread over 1000 runs kept (0 left out): least 4, 5th 13, 25th 22, median 29, 75th 39, 95th 55, greatest 80

### japanese_ships_claimed_sunk_per_hundred_sunk

- Counts: Ships the Japanese side claims sunk, for each 100 US ships really sunk
- Record: 200 to 300 (rows 1524, 1522, 1532)
- Verdict: **MISS**: too few runs kept
- Spread over 44 runs kept (956 left out): least 100, 5th 100, 25th 100, median 100, 75th 100, 95th 300, greatest 500
- Each value and its runs: 100: 39, 150: 1, 250: 1, 300: 1, 400: 1, 500: 1

## At the edge

### gun_hits_on_kinugasa

- Counts: Gun hits on Kinugasa, all calibres
- Record: 0 to 4 (rows 1605, 4036, 1686, 1687, 1688)
- Verdict: agrees, at the edge
- Spread over 1000 runs kept (0 left out): least 0, 5th 4, 25th 9, median 15, 75th 29, 95th 49, greatest 80

## Result on the objectives scale (reported, not judged)

Seen from the Allied side, runs of each result:

- decisive victory: 0 of 1000
- major victory: 449 of 1000
- minor victory: 393 of 1000
- stalemate: 146 of 1000
- minor defeat: 11 of 1000
- major defeat: 1 of 1000
- decisive defeat: 0 of 1000

Allied margin over 1000 runs: 5th 0, median 6, 95th 12

## Notes

- Percentiles are by nearest rank: the value at place ceil(p x n / 100) of the n kept runs, sorted. The record is inside the middle 90 percent if any part of it lies from the 5th percentile to the 95th, ends included. The verdict is "agrees, at the edge" where a measure agrees and no part of its record lies from the 25th percentile to the 75th, ends included; it still agrees. A run holds the record if its value is from the low end to the high end, ends included.
- Hits are counted as rolled: a salvo is rolled at the ships afloat when its round starts, so a hit on a ship another salvo sinks in the same round is counted (decision 0062).
- Escorts are matched to a raid's first strike group: an escort entry is added once, when its day, pulse and first group are those of an attack the measure counts (decision 0062).
