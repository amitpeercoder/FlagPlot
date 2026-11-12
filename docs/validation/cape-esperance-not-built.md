# cape-esperance: measures not run

Battle: cape-esperance
Measures: data/validation/cape_esperance.toml (hash a81b899ede1acddd), 9 measures, 7 judged

**Not run: the scenario is not built.** There is no batch and no verdict.

| Measure | Record | 5th to 95th | Median | Runs holding the record | Left out | Verdict |
| --- | --- | --- | --- | --- | --- | --- |
| japanese_ships_sunk | 2 | - | - | - | - | not run: scenario not built |
| japanese_ships_with_propulsion_out | 0 | - | - | - | - | not run: scenario not built |
| us_cruisers_sunk | 0 | - | - | - | - | not run: scenario not built |
| us_destroyers_sunk | 1 | - | - | - | - | not run: scenario not built |
| gun_hits_on_japanese_cruisers | 130 to 134 | - | - | - | - | not run: scenario not built |
| gun_hits_on_us_ships | 15 to 29 | - | - | - | - | not run: scenario not built |
| torpedo_hits_on_japanese_ships | 0 to 1 | - | - | - | - | not run: scenario not built |
| japanese_ships_claimed_sunk_per_hundred_sunk | 200 to 300 | - | - | - | - | not run: scenario not built |
| us_ships_claimed_sunk_per_hundred_sunk | 400 to 750 | - | - | - | - | not run: scenario not built |

## Notes

- Percentiles are by nearest rank: the value at place ceil(p x n / 100) of the n kept runs, sorted. The record is inside the middle 90 percent if any part of it lies from the 5th percentile to the 95th, ends included. The verdict is "agrees, at the edge" where a measure agrees and no part of its record lies from the 25th percentile to the 75th, ends included; it still agrees. A run holds the record if its value is from the low end to the high end, ends included.
- Hits are counted as rolled: a salvo is rolled at the ships afloat when its round starts, so a hit on a ship another salvo sinks in the same round is counted (decision 0062).
- Escorts are matched to a raid's first strike group: an escort entry is added once, when its day, pulse and first group are those of an attack the measure counts (decision 0062).
