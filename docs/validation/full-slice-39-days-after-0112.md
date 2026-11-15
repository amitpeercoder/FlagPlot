# rabaul-guadalcanal-air: validation of batch `after-0112`

Battle: rabaul-guadalcanal-air
Scenario: data/scenarios/07_full_slice.toml
Batch: label `after-0112`, 1000 runs, seeds 1 to 1000, file hash f72bec91552f9409
Inputs: state hash layout flagplot-state-v26, scenario file hash 5ed635ac5538b189
- data/scenarios/../map/south_pacific.csv ba6cb3aea5cc45f5
- data/scenarios/../map/south_pacific_land_links.csv a17610478627687e
- data/scenarios/../reference/ship_classes_eastern_solomons.csv 2716ae28c805274f
- data/scenarios/../reference/ship_classes.csv 0701cb1c6c75b124
- data/scenarios/../reference/ship_classes_placeholder_transports.csv 0808255af995ea1e
- data/scenarios/../reference/ship_classes_placeholder_fast_transport_cargo.csv 3ae71c9874889886
- data/scenarios/../reference/ship_classes_guadalcanal_landings.csv 729d1098f8c68ac4
- data/scenarios/../reference/port_sizes.csv bf78df85d94172ae
- data/scenarios/../reference/aircraft_types_eastern_solomons.csv 8a7741632e7e7ec6
- data/scenarios/../reference/aircraft_types_rabaul_guadalcanal.csv 8d2c4aa0f492f24f
- data/scenarios/../reference/search.toml 8d5c49f5f0d4a40c
- data/scenarios/../reference/surface_combat.toml 851083b54e6c45a3
- data/scenarios/../reference/air_combat.toml 768ef8b371e81d68
- data/scenarios/../reference/air_upkeep.toml 24457754c1eeecc8
- data/scenarios/../reference/fatigue.toml 5aac70e8a41f6579
- data/scenarios/../reference/air_supply.toml 3915fcb7aab986b4
- data/scenarios/../reference/ground.toml 5ef7d142f1faa97f
Measures: data/validation/guadalcanal_landings.toml (hash a326f21d27e3f8ce), 12 measures, 6 judged

Judged measures: 4 agree, 0 agree at the edge, 2 miss, of 6.

**The battle does not pass:** 2 of 6 judged measures miss.

| Measure | Record | 5th to 95th | Median | Runs holding the record | Left out | Verdict |
| --- | --- | --- | --- | --- | --- | --- |
| lunga_american_end_of_day_2 | 1 | 1 to 1 | 1 | 1000 of 1000 | 0 | agrees |
| tulagi_american_end_of_day_3 | 1 | 1 to 1 | 1 | 1000 of 1000 | 0 | agrees |
| lunga_american_after_the_tenaru | 1 | 1 to 1 | 1 | 1000 of 1000 | 0 | agrees |
| lunga_american_end_of_day_39 | 1 | 1 to 1 | 1 | 1000 of 1000 | 0 | agrees |
| tenaru_american_men_lost | 109 to 110 | 14 to 90 | 49 | 1 of 1000 | 0 | **MISS**: the record is outside the middle 90 percent; fewer than 1 run in 20 holds the record |
| tenaru_japanese_men_lost | 800 to 900 | 240 to 420 | 340 | 0 of 1000 | 0 | **MISS**: the record is outside the middle 90 percent; fewer than 1 run in 20 holds the record |
| ridge_american_men_lost | 134 to 253 | 82 to 218 | 148 | 638 of 1000 | 0 | reported |
| ridge_japanese_men_lost | 600 to 1138 | 1060 to 1380 | 1220 | 199 of 1000 | 0 | reported |
| tulagi_japanese_men_lost | 440 to 780 | 440 to 440 | 440 | 1000 of 1000 | 0 | reported |
| airfield_built_day | 11 to 13 | 11 to 11 | 11 | 1000 of 1000 | 0 | reported |
| squadrons_of_20_august_delivered_day | 14 | 14 to 14 | 14 | 1000 of 1000 | 0 | reported |
| days_of_food_at_lunga_when_the_transports_sail | 17 to 37 | 35 to 36 | 36 | 1000 of 1000 | 0 | reported |

## Misses

### tenaru_american_men_lost

- Counts: Allied men lost ashore at (2,-4) on days 16 to 19, the game's attack from the east and the three days a fight can run after it
- Record: 109 to 110 (rows 1936, 1938, 1939, 1742, 1937, 1943, 1944, 1945, 1946, 1743)
- Verdict: **MISS**: the record is outside the middle 90 percent; fewer than 1 run in 20 holds the record
- Spread over 1000 runs kept (0 left out): least 0, 5th 14, 25th 32, median 49, 75th 66, 95th 90, greatest 129

### tenaru_japanese_men_lost

- Counts: Japanese men lost ashore at (2,-4) on days 16 to 19
- Record: 800 to 900 (rows 1920, 1927, 1928, 1746)
- Verdict: **MISS**: the record is outside the middle 90 percent; fewer than 1 run in 20 holds the record
- Spread over 1000 runs kept (0 left out): least 180, 5th 240, 25th 300, median 340, 75th 360, 95th 420, greatest 500
- Each value and its runs: 180: 2, 200: 9, 220: 11, 240: 37, 260: 63, 280: 95, 300: 127, 320: 135, 340: 165, 360: 136, 380: 95, 400: 66, 420: 31, 440: 17, 460: 9, 480: 1, 500: 1

## Result on the objectives scale (reported, not judged)

Seen from the Allied side, runs of each result:

- decisive victory: 1000 of 1000
- major victory: 0 of 1000
- minor victory: 0 of 1000
- stalemate: 0 of 1000
- minor defeat: 0 of 1000
- major defeat: 0 of 1000
- decisive defeat: 0 of 1000

Allied margin over 1000 runs: 5th 73.935333, median 92.035333, 95th 105.515333

## Notes

- Percentiles are by nearest rank: the value at place ceil(p x n / 100) of the n kept runs, sorted. The record is inside the middle 90 percent if any part of it lies from the 5th percentile to the 95th, ends included. The verdict is "agrees, at the edge" where a measure agrees and no part of its record lies from the 25th percentile to the 75th, ends included; it still agrees. A run holds the record if its value is from the low end to the high end, ends included.
- Hits are counted as rolled: a salvo is rolled at the ships afloat when its round starts, so a hit on a ship another salvo sinks in the same round is counted (decision 0062).
- Escorts are matched to a raid's first strike group: an escort entry is added once, when its day, pulse and first group are those of an attack the measure counts (decision 0062).

## Beside the run before decision 0112

Written by the team lead of decision 0112 on Nov 15, 2026. The batch `before-0112` was made by the build of `240b2cc6`, the commit this build starts from, of the same scenario file with the same 1000 seeds, and judged by that build against the same measures file. Only the port sizes file differs between the two runs' inputs. It gained the two fuelling rates of decision 0112, "The first figures": 200 t an hour at a berth and 130 t an hour by lighter.

| Measure | Before 0112: 5th to 95th, median | After 0112: 5th to 95th, median | Verdict before | Verdict after |
| --- | --- | --- | --- | --- |
| lunga_american_end_of_day_2 | 1 to 1, 1 | 1 to 1, 1 | agrees | agrees |
| tulagi_american_end_of_day_3 | 1 to 1, 1 | 1 to 1, 1 | agrees | agrees |
| lunga_american_after_the_tenaru | 1 to 1, 1 | 1 to 1, 1 | agrees | agrees |
| lunga_american_end_of_day_39 | 1 to 1, 1 | 1 to 1, 1 | agrees | agrees |
| tenaru_american_men_lost | 14 to 90, 49 | 14 to 90, 49 | miss | miss |
| tenaru_japanese_men_lost | 240 to 420, 340 | 240 to 420, 340 | miss | miss |

**Nothing the measures read moved.** The six judged measures, the six reported ones and the result on the objectives scale (a decisive victory in all 1000 runs; Allied margin 5th 73.935333, median 92.035333, 95th 105.515333) are the same in every run. The two batch files are identical in every count the batch records, run by run. They differ only in each run's final state hash, the label and the hash of the port sizes file. The state differs, but only in fuel, which no measure reads and which, in this slice, decides nothing.

**What moved, in the run of the scenario's own seed** (the state at the end of each day, compared by the build of each tree):

- **A day earlier.** Most forces that come into a base of their side short of fuel now fill in the pulse they arrive, where before they filled the next morning. The ships end their arrival day with more fuel and the base with less, and both are as before from the next day. This is the case of 440 ship-days, from day 5 to day 39.
- **One force sails short where it sailed full: task force 20, the transports at Guadalcanal (ships 52 to 64).**
  - It arrives at Lunga (2,-4) at hour 10 of day 1, and lands its men and stores by lighter through every hour of the pulses of days 1 and 2.
  - Under decision 0112 a ship does its cargo first and fuels in the hours its cargo leaves, so it takes no fuel at Lunga while it lands.
  - Before, the morning's refuelling of day 2 gave it Lunga's 200 t at no cost in hours: 16 t to each of twelve ships and 8 t to the thirteenth.
  - It now sails for Espiritu Santo on day 3 with 16 t less in each ship (8 t in ship 64): 984 t, not 1000.
  - It arrives at Espiritu Santo at 03:48 on day 5 and takes the base's 600 t in that pulse, after its unloading. Before, it took the same 600 t the morning after.
  - So it ends the game 200 t shorter across its ships than before (up to 73.8 t in one ship, as Espiritu Santo's 600 t are shared in ship order), and Lunga keeps its 200 t of fuel: it holds 200 t at the end of every day from day 2, where before it held none.
  - No ship of it runs dry and no order of it is refused. It sails no later and arrives no later.
- **No other force sails short,** and no force is held at a reserve that was not before.

**The four step 2 battles.** Savo Island (`04_savo_island`), the Eastern Solomons (`05_eastern_solomons`) and the air campaign (`06_rabaul_guadalcanal`) were each run over 1000 seeds by both builds. Each pair of batch files is identical, final state hashes included: in those scenarios no force lies at a base of its side short of fuel. No judged measure of theirs changes standing: Savo Island 21 agree, 4 at the edge, 4 miss of 29; the Eastern Solomons 11, 2, 2 of 15; the air campaign 7, 1, 2 of 10, as before. Cape Esperance's check was not run again.
