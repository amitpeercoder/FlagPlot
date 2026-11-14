# rabaul-guadalcanal-air: validation of batch `after-0106`

Battle: rabaul-guadalcanal-air
Scenario: data/scenarios/07_full_slice.toml
Batch: label `after-0106`, 1000 runs, seeds 1 to 1000, file hash 9afea88c22ca89fd
Inputs: state hash layout flagplot-state-v22, scenario file hash f8fc8df6e1e2da5f
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
- data/scenarios/../reference/ground.toml 7bc7c7ffd6c4f80e
Measures: data/validation/guadalcanal_landings.toml (hash a326f21d27e3f8ce), 12 measures, 6 judged

Judged measures: 4 agree, 0 agree at the edge, 2 miss, of 6.

**The battle does not pass:** 2 of 6 judged measures miss.

| Measure | Record | 5th to 95th | Median | Runs holding the record | Left out | Verdict |
| --- | --- | --- | --- | --- | --- | --- |
| lunga_american_end_of_day_2 | 1 | 1 to 1 | 1 | 1000 of 1000 | 0 | agrees |
| tulagi_american_end_of_day_3 | 1 | 1 to 1 | 1 | 1000 of 1000 | 0 | agrees |
| lunga_american_after_the_tenaru | 1 | 1 to 1 | 1 | 1000 of 1000 | 0 | agrees |
| lunga_american_end_of_day_39 | 1 | 1 to 1 | 1 | 1000 of 1000 | 0 | agrees |
| tenaru_american_men_lost | 109 to 110 | 5 to 59 | 23 | 0 of 1000 | 0 | **MISS**: the record is outside the middle 90 percent; fewer than 1 run in 20 holds the record |
| tenaru_japanese_men_lost | 800 to 900 | 240 to 420 | 340 | 0 of 1000 | 0 | **MISS**: the record is outside the middle 90 percent; fewer than 1 run in 20 holds the record |
| ridge_american_men_lost | 134 to 253 | 49 to 169 | 101 | 271 of 1000 | 0 | reported |
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
- Spread over 1000 runs kept (0 left out): least 0, 5th 5, 25th 14, median 23, 75th 41, 95th 59, greatest 91

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

Allied margin over 1000 runs: 5th 96.327083, median 114.517083, 95th 128.377083

## Notes

- Percentiles are by nearest rank: the value at place ceil(p x n / 100) of the n kept runs, sorted. The record is inside the middle 90 percent if any part of it lies from the 5th percentile to the 95th, ends included. The verdict is "agrees, at the edge" where a measure agrees and no part of its record lies from the 25th percentile to the 75th, ends included; it still agrees. A run holds the record if its value is from the low end to the high end, ends included.
- Hits are counted as rolled: a salvo is rolled at the ships afloat when its round starts, so a hit on a ship another salvo sinks in the same round is counted (decision 0062).
- Escorts are matched to a raid's first strike group: an escort entry is added once, when its day, pulse and first group are those of an attack the measure counts (decision 0062).

## Beside the two earlier reports

Written by hand after the report above, for decision 0106 (`docs/decisions/0106-cover-and-digging-in.md`). The baseline is `docs/validation/full-slice-39-days-baseline.md` (1000 runs, state v20), and the report after decision 0105 is `docs/validation/full-slice-39-days-after-0105.md` (1000 runs, state v21). Nothing was tuned and nothing in the slice's scenario file changed. The live figures are the record's: `cover_from_fire` 0.9, `cover_in_fight` 0.5, `fieldworks_per_day` 10, `fieldworks_limit` 30, grades from 30 and 60 at factors 3 and 10, `strain_per_hit` 1, `strain_per_day` 10, `strain_limit` 60, `cover_from_strain` 0.5, `cover_wear_per_hit` 0.2. No base in the slice has defences, so every unit's cover is its own fieldworks.

| Measure | Record | Baseline: 5th to 95th, median | After 0105 | After 0106 | Verdict: baseline, 0105, 0106 |
| --- | --- | --- | --- | --- | --- |
| lunga_american_end_of_day_2 | 1 | 1 to 1, 1 | 1 to 1, 1 | 1 to 1, 1 | agrees, agrees, agrees |
| tulagi_american_end_of_day_3 | 1 | 1 to 1, 1 | 1 to 1, 1 | 1 to 1, 1 | agrees, agrees, agrees |
| lunga_american_after_the_tenaru | 1 | 1 to 1, 1 | 1 to 1, 1 | 1 to 1, 1 | agrees, agrees, agrees |
| lunga_american_end_of_day_39 | 1 | 1 to 1, 1 | 1 to 1, 1 | 1 to 1, 1 | agrees, agrees, agrees |
| tenaru_american_men_lost | 109 to 110 | 10 to 90, 40 | 10 to 90, 40 | 5 to 59, 23 | miss, miss, miss |
| tenaru_japanese_men_lost | 800 to 900 | 916 to 916, 916 | 240 to 420, 340 | 240 to 420, 340 | miss, miss, miss |
| ridge_american_men_lost | 134 to 253 | 80 to 240, 160 | 60 to 200, 120 | 49 to 169, 101 | reported |
| ridge_japanese_men_lost | 600 to 1138 | 3325 to 3327, 3326 | 1720 to 2080, 1900 | 1720 to 2080, 1900 | reported |
| tulagi_japanese_men_lost | 440 to 780 | 440 to 440, 440 | 440 to 440, 440 | 440 to 440, 440 | reported |
| airfield_built_day | 11 to 13 | 11 to 11, 11 | 11 to 11, 11 | 11 to 11, 11 | reported |
| squadrons_of_20_august_delivered_day | 14 | 14 to 14, 14 | 14 to 14, 14 | 14 to 14, 14 | reported |
| days_of_food_at_lunga_when_the_transports_sail | 17 to 37 | 68 to 68, 68 | 68 to 68, 68 | 68 to 68, 68 | reported |

The men each side lost over the 39 days, by cause, from the batch (5th, median and 95th over 1000 runs; the batch after 0105 was run again from that build for this table):

| | After 0105 | After 0106 |
| --- | --- | --- |
| American men lost to bombs and shells | 40, 90, 140 | 20, 43, 67 |
| American men lost in the fight ashore | 120, 200, 300 | 106, 178, 261 |
| Japanese men lost in the fight, unfed, surrendered | 2160, 2360, 2540; 2618, 2692, 2761; 360, 400, 440 | unchanged |
| Allied margin | 95.44, 113.86, 127.59 | 96.33, 114.52, 128.38 |

A typical run (the scenario's seed, 1942, the run baseline):

- **Digging in.** The Marines at Lunga take the field on day 1 and dig from day 2. Combat Groups A and B have fieldworks of 10 after day 2, 20 after day 3 and 30, the limit, after day 4. The Raiders and the Parachute Battalion on the Tulagi side reach 30 on day 6, after the fighting there. They leave their holes when they are taken aboard for Lunga on day 24 and dig again there. The Japanese units at (2,-5) dig in too, and so does the Ichiki echelon once it has gone back to defend. Every scripted attack order and march leaves the unit's fieldworks (10 such events in the run).
- **Shellings and raids.** The slice has no bombardment of Lunga by ships. Its raids put 7 bombs on men in four mornings (days 20, 24, 27 and 34: 3, 1, 2 and 1 hits). They took 70 men in the run after 0105 and take 34 now. Cover of 30 saves 27 in 100 of a hit's men. Each unit's share of a hit is a few men, though, and each share is cut to a whole man, so in this slice cover saves about half. Across the batch the median loss to bombs falls from 90 to 43. Fatigue barely moves. A raid of 1 to 3 hits keeps the Marines from resting that day and adds 0.85 to 2.6 of fatigue behind fieldworks of 30, and 1 to 3 to the engineers and the pioneers, who are working units and have no cover. Every unit is back to rest the next day. No raid wears more than 0.6 off a unit's fieldworks, and the next day's digging puts it back.
- **The Tenaru (day 16).** Unchanged on the Japanese side. The Ichiki echelon attacks with 916 men, its attack fails, and it loses 360 and falls back. The Marines lose 14 where they lost 20, their fieldworks of 30 cutting each unit's part of the loss by 15 in 100. Across the batch the American loss there falls from a median of 40 to 23, further below the record of 109 to 110.
- **The ridge (days 37 to 39).** Unchanged on the Japanese side: the Kawaguchi parts lose 1,040 on day 37 and 646 more on day 39. The Marines lose 154 where they lost 189. Across the batch the American loss falls from a median of 120 to 101, and 271 runs in 1000 hold the record of 134 to 253, where 437 did.
- **What did not move.** Who holds Lunga and Tulagi, the airfield's day, the squadrons' arrival, the food and every Japanese loss are as they were. The Japanese side defends at Lunga only on day 1 and 2, before it has dug, and after that it attacks, out of its holes. The rule cuts American losses only, so it widens the misses at the Tenaru and the ridge, which were already American losses below the record.
