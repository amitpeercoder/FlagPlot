# rabaul-guadalcanal-air: validation of batch `after-stock-fix`

Battle: rabaul-guadalcanal-air
Scenario: data/scenarios/07_full_slice.toml
Batch: label `after-stock-fix`, 1000 runs, seeds 1 to 1000, file hash 83432e2139c2a716
Inputs: state hash layout flagplot-state-v22, scenario file hash b3d47f3433ed4a0b
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

Allied margin over 1000 runs: 5th 94.037083, median 112.237083, 95th 126.137083

## Notes

- Percentiles are by nearest rank: the value at place ceil(p x n / 100) of the n kept runs, sorted. The record is inside the middle 90 percent if any part of it lies from the 5th percentile to the 95th, ends included. The verdict is "agrees, at the edge" where a measure agrees and no part of its record lies from the 25th percentile to the 75th, ends included; it still agrees. A run holds the record if its value is from the low end to the high end, ends included.
- Hits are counted as rolled: a salvo is rolled at the ships afloat when its round starts, so a hit on a ship another salvo sinks in the same round is counted (decision 0062).
- Escorts are matched to a raid's first strike group: an escort entry is added once, when its day, pulse and first group are those of an attack the measure counts (decision 0062).

## Beside the three earlier reports

Written by hand after the report above, for amendments 8 and 9 of decision 0101 (`docs/decisions/0101-the-data-of-the-august-landings.md`): the placeholder Japanese stock in (2,-5) is 143 t where it was 72 t, seven days for the 2,040 men of the garrison of Lunga and its two construction units who fall back there as well as for the 2,070 who land there; and two scripted orders of day 1, with no row behind them, set those two construction units to work at the port, so that they do not build an airfield in the west by their default job. Nothing else in the scenario changed (a comment on the bases' defences changes no result), no figure was tuned and no rule changed. The earlier reports are the baseline (`full-slice-39-days-baseline.md`, state v20), the report after decision 0105 (`full-slice-39-days-after-0105.md`, v21) and the report after decision 0106 (`full-slice-39-days-after-0106.md`, v22). This batch is 1,000 runs, seeds 1 to 1,000, state v22.

| Measure | Record | Baseline: 5th to 95th, median | After 0105 | After 0106 | After the stock fix | Verdict: baseline, 0105, 0106, fix |
| --- | --- | --- | --- | --- | --- | --- |
| lunga_american_end_of_day_2 | 1 | 1 to 1, 1 | 1 to 1, 1 | 1 to 1, 1 | 1 to 1, 1 | agrees, agrees, agrees, agrees |
| tulagi_american_end_of_day_3 | 1 | 1 to 1, 1 | 1 to 1, 1 | 1 to 1, 1 | 1 to 1, 1 | agrees, agrees, agrees, agrees |
| lunga_american_after_the_tenaru | 1 | 1 to 1, 1 | 1 to 1, 1 | 1 to 1, 1 | 1 to 1, 1 | agrees, agrees, agrees, agrees |
| lunga_american_end_of_day_39 | 1 | 1 to 1, 1 | 1 to 1, 1 | 1 to 1, 1 | 1 to 1, 1 | agrees, agrees, agrees, agrees |
| tenaru_american_men_lost | 109 to 110 | 10 to 90, 40 | 10 to 90, 40 | 5 to 59, 23 | 5 to 59, 23 | miss, miss, miss, miss |
| tenaru_japanese_men_lost | 800 to 900 | 916 to 916, 916 | 240 to 420, 340 | 240 to 420, 340 | 240 to 420, 340 | miss, miss, miss, miss |
| ridge_american_men_lost | 134 to 253 | 80 to 240, 160 | 60 to 200, 120 | 49 to 169, 101 | 49 to 169, 101 | reported |
| ridge_japanese_men_lost | 600 to 1138 | 3325 to 3327, 3326 | 1720 to 2080, 1900 | 1720 to 2080, 1900 | 1720 to 2080, 1900 | reported |
| tulagi_japanese_men_lost | 440 to 780 | 440 to 440, 440 | 440 to 440, 440 | 440 to 440, 440 | 440 to 440, 440 | reported |
| airfield_built_day | 11 to 13 | 11 to 11, 11 | 11 to 11, 11 | 11 to 11, 11 | 11 to 11, 11 | reported |
| squadrons_of_20_august_delivered_day | 14 | 14 to 14, 14 | 14 to 14, 14 | 14 to 14, 14 | 14 to 14, 14 | reported |
| days_of_food_at_lunga_when_the_transports_sail | 17 to 37 | 68 to 68, 68 | 68 to 68, 68 | 68 to 68, 68 | 68 to 68, 68 | reported |

No measure moved: every spread, median and count of runs holding the record is what it was after decision 0106. The fix acts only in the west, which no measure counts, and through the Japanese losses on the objectives scale.

The men each side lost over the 39 days, by cause, from the batch (5th, median and 95th over 1000 runs):

| | After 0106 | After the stock fix |
| --- | --- | --- |
| American men lost to bombs and shells | 20, 43, 67 | 20, 43, 67 |
| American men lost in the fight ashore | 106, 178, 261 | 106, 178, 261 |
| Japanese men lost in the fight | 2160, 2360, 2540 | 2160, 2360, 2540 |
| Japanese men lost unfed | 2618, 2692, 2761 | 2391, 2467, 2537 |
| Japanese men surrendered | 360, 400, 440 | 360, 400, 440 |
| Allied margin | 96.33, 114.52, 128.38 | 94.04, 112.24, 126.14 |

### The west, in plain words

The figures below were read from the event logs of the same 1,000 seeds, run one by one with `sim-cli run --seed N --events`, on this scenario and, for "before", on the scenario as it was at the merge of pull request 297. A day's feeding is read from its night pulse.

- **Who gathers there, and when.** The garrison of Lunga and the 11th and 13th Construction Units leave the airfield's hex on day 1 and reach (2,-5) on day 4, 10 August, in every run, with 1,679 of their 2,040 men (median; 1,626 to 1,732), the rest lost in the fight and unfed on days 1 to 3. The Yokosuka 5th Special Naval Landing Force's 200 join them on day 11, Ichiki's survivors on day 19 (2,460 men in the hex on day 20, median), the Tassafaronga force's 900 on day 24 (the most in the hex, 2,881 on day 25, median; it was 2,393 before), the Oka group's 333 on day 32 and the Aoba part's 637 on day 37. The Oka group leaves on day 37 for the field. At the end, on day 39, 2,260 men are in the hex (median), where 2,022 were.
- **When the stock runs out.** It runs short on day 20, 26 August (5th to 95th, day 20 to 21), and from day 21 (21 to 22) no man in the hex is fed, to the end. Before the fix it ran short on day 13, 19 August, and fed nobody from day 14, in every run. The stock feeds the garrison and the Yokosuka force, about 1,880 men, for the two weeks before Ichiki's survivors come; with them, on day 19, the hex holds about 2,460, and the stock is gone within two days.
- **Men lost unfed.** In (2,-5) over the run, 2,184 men (median; 2,114 to 2,253), where 2,410 were lost before (2,343 to 2,477): 226 fewer, about one in eleven. The Japanese lose 2,467 unfed in all hexes (2,391 to 2,537), where they lost 2,692. Most of the west's men still starve: the stock buys them a week, and the men of the attacks who fall back there, and those who land after day 20, come to an empty dump, as amendment 8 says.
- **The Oka group** reaches the field on day 39 with 236 of its 333 men (median, as before): it lands on day 32, after the stock has run out.
- **No airfield in the west.** Built with amendment 8 alone, the two construction units, fed a week longer, finished an airfield at the placeholder stock by their default job (day 17 to 19 in every run, with the Ryujo's fighters landing on it in 407); amendment 9 sets them to work at the port by a scripted order, and in this batch base 6 gets no airfield and no Japanese air group forms there in any run.

### For the tuning round

From the same event logs, each as the median and the middle 90 percent (5th to 95th) of the 1,000 runs. None changed with the fix, except where it says so.

- **(a) The fight the Oka group starts at the ridge on day 39.** In every run the Japanese side attacks at (2,-4) on day 39, the Oka group arriving from the west with 236 men (median), none of them firing. The fight's losses are taken from every Japanese unit in the hex. The Kawaguchi parts that went back to defend and fell back on day 37 and are still marching out lose **709** men (575 to 838; 710 before the fix, the same spread); the Oka group itself loses 84 (70 to 98); all Japanese losses that day are 800 (640 to 940).
- **(b) American men lost to bombs and shells over the 39 days: 43** (20 to 67). They are all bombs: no ship shells Lunga in the slice. They come from **9** hits on men (4 to 14), on 6 days (3 to 8), every one from the raids on base 1.
- **(c) When each attack breaks off.**
  - The Tenaru: the Ichiki echelon goes back to defend, its attack spent, on **day 16**, 22 August, the day it attacks, in every run, with **576** men left (496 to 676) of its 916.
  - The ridge: the Kawaguchi parts go back to defend on **day 37**, 12 September, the day they attack, in every run, with **1,970** men left among them (1,810 to 2,130) of the 3,090 landed. In 7 runs one part (150 men) never arrived, lost at sea, and four parts break off.

### What did not move, and what the scenario's seed shows

Everything east of the Matanikau is as it was: who holds Lunga and Tulagi, the airfield's day, the squadrons, the food at Lunga, the Tenaru and the ridge on both sides, and the American losses to bombs. On the scenario's seed, 1942, the garrison and the construction units reach the west on day 4 with 1,662 men and are fed until the night of day 20, when 2,070 of the 2,418 men there are fed and 348 are not; from day 21 none is. The west loses 2,152 men unfed over the run, and base 6 has no airfield. The Allied margin is 106.25 where it was 108.51, the Japanese having lost 5,090 men where they lost 5,316.
