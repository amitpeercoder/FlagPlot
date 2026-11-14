# rabaul-guadalcanal-air: validation of batch `after-0105`

Battle: rabaul-guadalcanal-air
Scenario: data/scenarios/07_full_slice.toml
Batch: label `after-0105`, 1000 runs, seeds 1 to 1000, file hash b88fb7b2c138a94d
Inputs: state hash layout flagplot-state-v21, scenario file hash f8fc8df6e1e2da5f
- data/scenarios/../map/south_pacific.csv ba6cb3aea5cc45f5
- data/scenarios/../map/south_pacific_land_links.csv a17610478627687e
- data/scenarios/../reference/ship_classes_eastern_solomons.csv 2716ae28c805274f
- data/scenarios/../reference/ship_classes.csv 0701cb1c6c75b124
- data/scenarios/../reference/ship_classes_placeholder_transports.csv 0808255af995ea1e
- data/scenarios/../reference/ship_classes_placeholder_fast_transport_cargo.csv 3ae71c9874889886
- data/scenarios/../reference/ship_classes_guadalcanal_landings.csv 729d1098f8c68ac4
- data/scenarios/../reference/port_sizes.csv 53811d71d5b18b81
- data/scenarios/../reference/aircraft_types_eastern_solomons.csv 8a7741632e7e7ec6
- data/scenarios/../reference/aircraft_types_rabaul_guadalcanal.csv 8d2c4aa0f492f24f
- data/scenarios/../reference/search.toml 8d5c49f5f0d4a40c
- data/scenarios/../reference/surface_combat.toml 851083b54e6c45a3
- data/scenarios/../reference/air_combat.toml 768ef8b371e81d68
- data/scenarios/../reference/air_upkeep.toml 24457754c1eeecc8
- data/scenarios/../reference/fatigue.toml 5aac70e8a41f6579
- data/scenarios/../reference/air_supply.toml 3915fcb7aab986b4
- data/scenarios/../reference/ground.toml 9c4316095d7247ab
Measures: data/validation/guadalcanal_landings.toml (hash a326f21d27e3f8ce), 12 measures, 6 judged

Judged measures: 4 agree, 0 agree at the edge, 2 miss, of 6.

**The battle does not pass:** 2 of 6 judged measures miss.

| Measure | Record | 5th to 95th | Median | Runs holding the record | Left out | Verdict |
| --- | --- | --- | --- | --- | --- | --- |
| lunga_american_end_of_day_2 | 1 | 1 to 1 | 1 | 1000 of 1000 | 0 | agrees |
| tulagi_american_end_of_day_3 | 1 | 1 to 1 | 1 | 1000 of 1000 | 0 | agrees |
| lunga_american_after_the_tenaru | 1 | 1 to 1 | 1 | 1000 of 1000 | 0 | agrees |
| lunga_american_end_of_day_39 | 1 | 1 to 1 | 1 | 1000 of 1000 | 0 | agrees |
| tenaru_american_men_lost | 109 to 110 | 10 to 90 | 40 | 4 of 1000 | 0 | **MISS**: the record is outside the middle 90 percent; fewer than 1 run in 20 holds the record |
| tenaru_japanese_men_lost | 800 to 900 | 240 to 420 | 340 | 0 of 1000 | 0 | **MISS**: the record is outside the middle 90 percent; fewer than 1 run in 20 holds the record |
| ridge_american_men_lost | 134 to 253 | 60 to 200 | 120 | 437 of 1000 | 0 | reported |
| ridge_japanese_men_lost | 600 to 1138 | 1720 to 2080 | 1900 | 0 of 1000 | 0 | reported |
| tulagi_japanese_men_lost | 440 to 780 | 440 to 440 | 440 | 1000 of 1000 | 0 | reported |
| airfield_built_day | 11 to 13 | 11 to 11 | 11 | 1000 of 1000 | 0 | reported |
| squadrons_of_20_august_delivered_day | 14 | 14 to 14 | 14 | 1000 of 1000 | 0 | reported |
| days_of_food_at_lunga_when_the_transports_sail | 17 to 37 | 68 to 68 | 68 | 0 of 1000 | 0 | reported |

## Misses

### tenaru_american_men_lost

- Counts: Allied men lost ashore at (2,-4) on days 16 to 19, the game's attack from the east and the three days a fight can run after it
- Record: 109 to 110 (rows 1936, 1938, 1939, 1742, 1937, 1943, 1944, 1945, 1946, 1743)
- Verdict: **MISS**: the record is outside the middle 90 percent; fewer than 1 run in 20 holds the record
- Spread over 1000 runs kept (0 left out): least 0, 5th 10, 25th 30, median 40, 75th 60, 95th 90, greatest 140
- Each value and its runs: 0: 38, 10: 78, 20: 125, 30: 141, 40: 178, 50: 148, 60: 113, 70: 80, 80: 44, 90: 34, 100: 15, 110: 4, 130: 1, 140: 1

### tenaru_japanese_men_lost

- Counts: Japanese men lost ashore at (2,-4) on days 16 to 19
- Record: 800 to 900 (rows 1920, 1927, 1928, 1746)
- Verdict: **MISS**: the record is outside the middle 90 percent; fewer than 1 run in 20 holds the record
- Spread over 1000 runs kept (0 left out): least 180, 5th 240, 25th 300, median 340, 75th 360, 95th 420, greatest 500
- Each value and its runs: 180: 3, 200: 9, 220: 8, 240: 37, 260: 55, 280: 103, 300: 117, 320: 141, 340: 167, 360: 142, 380: 101, 400: 55, 420: 34, 440: 17, 460: 8, 480: 2, 500: 1

## Result on the objectives scale (reported, not judged)

Seen from the Allied side, runs of each result:

- decisive victory: 1000 of 1000
- major victory: 0 of 1000
- minor victory: 0 of 1000
- stalemate: 0 of 1000
- minor defeat: 0 of 1000
- major defeat: 0 of 1000
- decisive defeat: 0 of 1000

Allied margin over 1000 runs: 5th 95.437083, median 113.857083, 95th 127.587083

## Notes

- Percentiles are by nearest rank: the value at place ceil(p x n / 100) of the n kept runs, sorted. The record is inside the middle 90 percent if any part of it lies from the 5th percentile to the 95th, ends included. The verdict is "agrees, at the edge" where a measure agrees and no part of its record lies from the 25th percentile to the 75th, ends included; it still agrees. A run holds the record if its value is from the low end to the high end, ends included.
- Hits are counted as rolled: a salvo is rolled at the ships afloat when its round starts, so a hit on a ship another salvo sinks in the same round is counted (decision 0062).
- Escorts are matched to a raid's first strike group: an escort entry is added once, when its day, pulse and first group are those of an attack the measure counts (decision 0062).
