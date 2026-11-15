# rabaul-guadalcanal-air: validation of batch `before-tuning`

Battle: rabaul-guadalcanal-air
Scenario: data/scenarios/07_full_slice.toml
Batch: label `before-tuning`, 1000 runs, seeds 1 to 1000, file hash 52ea2e3769f29d3e
Inputs: state hash layout flagplot-state-v23, scenario file hash 5ed635ac5538b189
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
- data/scenarios/../reference/ground.toml e5d2184223d62b2a
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
| ridge_american_men_lost | 134 to 253 | 49 to 182 | 114 | 281 of 1000 | 0 | reported |
| ridge_japanese_men_lost | 600 to 1138 | 1060 to 1380 | 1220 | 194 of 1000 | 0 | reported |
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

Allied margin over 1000 runs: 5th 83.717083, median 101.637083, 95th 115.267083

## Notes

- Percentiles are by nearest rank: the value at place ceil(p x n / 100) of the n kept runs, sorted. The record is inside the middle 90 percent if any part of it lies from the 5th percentile to the 95th, ends included. The verdict is "agrees, at the edge" where a measure agrees and no part of its record lies from the 25th percentile to the 75th, ends included; it still agrees. A run holds the record if its value is from the low end to the high end, ends included.
- Hits are counted as rolled: a salvo is rolled at the ships afloat when its round starts, so a hit on a ship another salvo sinks in the same round is counted (decision 0062).
- Escorts are matched to a raid's first strike group: an escort entry is added once, when its day, pulse and first group are those of an attack the measure counts (decision 0062).

## Beside the four earlier reports

Written by hand after the report above, from the batch `before-tuning` and the event logs of the same 1,000 seeds. Three things changed since the stock-fix report, all settled before the round by decision 0109 (`docs/decisions/0109-the-first-tuning-round-of-the-ground-figures.md`, "Settled before the round"); nothing was tuned. (1) A unit falling back is out of a fight its own side starts: it does not fire, is not counted for the limit on engaged men and takes no share of the losses, on a day when the other side is not on attack in the hex. (2) Each Japanese run lands its own rations as cargo (14 days for the Kawaguchi runs, the Aoba part and the Oka group; 7 days for the Ichiki echelon, the Yokosuka force and the force of 29 August), and the stocks stated ashore are 0 in (3,-4) and 71 t in (2,-5). (3) `fighting_unit_work_share` is 0.1, where it was 0.5. State version flagplot-state-v23, rules label 7 (decision 0111, merged alongside, raised the state version and changes nothing in the slice: every scenario's run is the same apart from its hash lines). This batch is 1,000 runs, seeds 1 to 1,000; the event logs read below were made by the same build before that merge, whose runs are the same. The earlier reports are the baseline (`full-slice-39-days-baseline.md`, state v20), the report after decision 0105 (`-after-0105.md`, v21), the report after decision 0106 (`-after-0106.md`, v22) and the report after the stock fix (`-after-stock-fix.md`, v22).

| Measure | Record | Baseline: 5th to 95th, median | After 0105 | After 0106 | After the stock fix | Before tuning | Verdict: baseline, 0105, 0106, fix, before tuning |
| --- | --- | --- | --- | --- | --- | --- | --- |
| lunga_american_end_of_day_2 | 1 | 1 to 1, 1 | 1 to 1, 1 | 1 to 1, 1 | 1 to 1, 1 | 1 to 1, 1 | agrees, agrees, agrees, agrees, agrees |
| tulagi_american_end_of_day_3 | 1 | 1 to 1, 1 | 1 to 1, 1 | 1 to 1, 1 | 1 to 1, 1 | 1 to 1, 1 | agrees, agrees, agrees, agrees, agrees |
| lunga_american_after_the_tenaru | 1 | 1 to 1, 1 | 1 to 1, 1 | 1 to 1, 1 | 1 to 1, 1 | 1 to 1, 1 | agrees, agrees, agrees, agrees, agrees |
| lunga_american_end_of_day_39 | 1 | 1 to 1, 1 | 1 to 1, 1 | 1 to 1, 1 | 1 to 1, 1 | 1 to 1, 1 | agrees, agrees, agrees, agrees, agrees |
| tenaru_american_men_lost | 109 to 110 | 10 to 90, 40 | 10 to 90, 40 | 5 to 59, 23 | 5 to 59, 23 | 5 to 59, 23 | miss, miss, miss, miss, miss |
| tenaru_japanese_men_lost | 800 to 900 | 916 to 916, 916 | 240 to 420, 340 | 240 to 420, 340 | 240 to 420, 340 | 240 to 420, 340 | miss, miss, miss, miss, miss |
| ridge_american_men_lost | 134 to 253 | 80 to 240, 160 | 60 to 200, 120 | 49 to 169, 101 | 49 to 169, 101 | 49 to 182, 114 | reported |
| ridge_japanese_men_lost | 600 to 1138 | 3325 to 3327, 3326 | 1720 to 2080, 1900 | 1720 to 2080, 1900 | 1720 to 2080, 1900 | 1060 to 1380, 1220 | reported |
| tulagi_japanese_men_lost | 440 to 780 | 440 to 440, 440 | 440 to 440, 440 | 440 to 440, 440 | 440 to 440, 440 | 440 to 440, 440 | reported |
| airfield_built_day | 11 to 13 | 11 to 11, 11 | 11 to 11, 11 | 11 to 11, 11 | 11 to 11, 11 | 11 to 11, 11 | reported |
| squadrons_of_20_august_delivered_day | 14 | 14 to 14, 14 | 14 to 14, 14 | 14 to 14, 14 | 14 to 14, 14 | 14 to 14, 14 | reported |
| days_of_food_at_lunga_when_the_transports_sail | 17 to 37 | 68 to 68, 68 | 68 to 68, 68 | 68 to 68, 68 | 68 to 68, 68 | 68 to 68, 68 | reported |

Both Tenaru measures are where they were: the Tenaru is not touched by any of the three changes. The ridge's Japanese losses fall from 1,900 to 1,220 (median), as decision 0109, 1 intended: the Kawaguchi parts that fell back on day 37 no longer lose men to the Oka group's fight on day 39. The record, 600 to 1,138, is now inside the middle 90 percent (194 of 1,000 runs hold it), and the measure is reported, not judged. The ridge's American losses rise from 101 to 114: the Oka group now arrives fed and fires (188 men, not none), so the Marines lose a few men on day 39 in some runs (0 to 31). The airfield stays at day 11 in every run. The food at Lunga is the Allied stock and does not move.

The men each side lost over the 39 days, by cause, from the batch (5th, median and 95th over 1,000 runs):

| | After the stock fix | Before tuning |
| --- | --- | --- |
| American men lost to bombs and shells | 20, 43, 67 | 20, 43, 67 |
| American men lost in the fight ashore | 106, 178, 261 | 118, 189, 264 |
| Japanese men lost in the fight | 2160, 2360, 2540 | 1480, 1680, 1860 |
| Japanese men lost unfed | 2391, 2467, 2537 | 2026, 2096, 2162 |
| Japanese men surrendered | 360, 400, 440 | 360, 400, 440 |
| Allied margin | 94.04, 112.24, 126.14 | 83.72, 101.64, 115.27 |

All 1,000 runs are a decisive Allied victory, as before. The Japanese median men lost in the fight fall by 680 and those lost unfed by 371 (their own rations): the Allied margin falls by about 10.6, as the Allied score for the Japanese men lost is smaller.

### A typical run, in plain words

Medians over the 1,000 runs, with the middle 90 percent where it is not a single value. Every date, landing and ration below is the same in every run unless a spread is given. A day's feeding is read from its night pulse. All Japanese attacks are at (2,-4); the Tenaru is day 16, the ridge day 37, and the Oka group's day 39.

**The Tenaru.** The Ichiki echelon (land unit 17) lands at (3,-4) on the night of day 13, 916 men, and with them 32.06 t of rations, seven days (five ships; the stock there is 32.1 t that night). It is fed in full (916 of 916). It marches on days 14, 15 and 16 toward (2,-4), carrying 18.32 t of supply, and reaches the hex on day 16 and attacks the same day, with 412 of its 916 men firing, none unfed (its fed share is 1). The Marines (units 1, 2, 3 and 7) fire 27.5 shots at 0.6 (a weight of 16.49) against its 4.12 shots at 0.3 (1.24). The attack fails the same day. The echelon loses 340 men in the fight (240 to 420; 360 in the scenario's own seed, 1942) and the Americans 14 (0 to 49; 14 in seed 1942). The echelon, tired out at fatigue 100, goes back to defend and falls back toward (2,-5) the same day with 576 men (496 to 676). It is fed from the 18.3 t it carried (576 of 576 on days 17 to 19) and reaches (2,-5) on day 19. No fight is fought at (2,-4) on days 17, 18 or 19.

**The ridge.** The Kawaguchi main body lands at (3,-4) in five runs, each with its own 14 days of rations: part 1 on day 26 (1,200 men, 84 t), part 2 on day 28 (150 men, 10.5 t; lost at sea in 7 runs), part 3 on day 30 (1,000, 70 t), part 4 on day 31 (370, 25.9 t) and part 5 on day 32 (370, 25.9 t): 3,090 men and 216.3 t. The stock in (3,-4) is 165 t on day 32 and 104 t on day 36, and the 3,090 are fed in full, every night, to day 37. The five parts march on day 35 and reach (2,-4) on day 37, carrying 77.25 t, and all attack on day 37 (the four parts that arrive in the 7 runs where part 2 was lost, the same). They fire with 2,147 men (960, 112, 650, 222 and 203), each fed in full. The Marines fire 92.7 shots at 0.6 (weight 55.62), cut from 124 shots by the limit; the Japanese 21.47 shots at 0.3 (6.44). The attack fails. The Japanese lose 1,100 men that day (960 to 1,260; 1,040 in seed 1942), the Americans 101 (49 to 169; 153 in seed 1942). All five parts go back to defend the same day, at fatigue 80 to 100, and fall back toward (2,-5) with 1,970 men between them (1,810 to 2,130): 768, 96, 644, 237 and 236 (medians). The march is three days, so at the end of the game (day 39) they are still in (2,-4), fed from the 87 t the parts carried (1,970 of 1,970 fed on days 38 and 39). On day 38 there is no fight.

**The Oka group's attack, day 39.** The Oka group (land unit 25) lands at (2,-5) on day 32, 333 men with 23.31 t (14 days), after the earlier stock is gone. It marches from day 37 and reaches (2,-4) on day 39 with 289 men (289 to 290) and 7.25 t, fed in full (its fed share is 1), and attacks with 188 firing men. The Marines, with the fallers-back not counted, are limited to three times the Oka group's men ashore (867 men, 8.67 shots, a weight of 5.20): the Oka group loses 100 men (60 to 160), the Americans 0 (0 to 31). **It no longer costs the Kawaguchi men anything.** The Kawaguchi parts and the Ichiki echelon lose no man to fight or fire at all on day 39, in any of the 1,000 runs (they lost 709, median, to this fight in the stock-fix runs). The Oka group goes back to defend the same day with 189 men (129 to 230).

**The west, (2,-5), and (3,-4).** The Japanese stock in (3,-4) is 0 stated; the landings bring it, 32 t on day 13 and 84 t on day 26, and the men there are fed in full on every night they are there (share 1.00 on days 14 to 16 and 27 to 37). In (2,-5) the stated 71 t on day 4 feeds the 1,679 men there (1,626 to 1,732) until day 13, when the stock reaches 0 (0.4 t at the end of day 13; fed share 0.80 to 1 that night); on the night of day 14 5 percent are fed (0 to 31) and from day 15 none. Each later landing there brings its own days: the force of 29 August on day 24 (900 men, 31.5 t) feeds the hex (2,427 men) in full on the nights of days 25 and 26, 60 percent on day 27 (52 to 67) and none from day 28; the Oka group on day 32 (23.3 t) feeds 2,175 men in full on days 33 and 34, 14 percent on day 35 (10 to 19) and none on days 36 and 37; the Aoba part on day 37 (637 men, 44.6 t) feeds 2,517 in full on days 38 and 39, and the stock at the end of day 39 is 12.2 t (11.8 to 12.5). The Ichiki survivors reach the hex on day 19, to no stock and no food, and the hex's men are not fed on days 20 to 24 (1,970 men on day 20, 5th to 95th 1,883 to 2,059).

### For the tuning round

Each as the median and the middle 90 percent (5th to 95th) of the 1,000 runs; where only one number is given it is the same in every run. Read from the same seeds' event logs (`export --seed N`), the breakdowns of the `ground_fire`, `land_unit_fatigue`, `ground_attack_failed` and `men_lost_in_fight` events.

**(a) The firing men each attacker has on its first day of attack, and why.** The firing men are its men ashore, times its fed share, times the fatigue factor (1 less its fatigue as a fraction of 100: at fatigue 55, 0.45), rounded down.

| Unit | First day of attack | Men ashore | Fed share | Fatigue before the fire | of which landing | of which march | of which rest | other | Fatigue factor | Firing men |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Ichiki echelon (17) | 16 | 916 | 1 | 55 | +40 | +20 (two days) | -5 | 0 | 0.45 | 412 |
| Kawaguchi part 1 (19) | 37 | 1,200 | 1 | 20 | +40 | +20 | -40 | 0 | 0.80 | 960 |
| Kawaguchi part 2 (20) | 37 | 150 | 1 | 25 | +40 | +20 | -35 | 0 | 0.75 | 112 |
| Kawaguchi part 3 (21) | 37 | 1,000 | 1 | 35 | +40 | +20 | -25 | 0 | 0.65 | 650 |
| Kawaguchi part 4 (22) | 37 | 370 | 1 | 40 | +40 | +20 | -20 | 0 | 0.60 | 222 |
| Kawaguchi part 5 (23) | 37 | 370 | 1 | 45 | +40 | +20 | -15 | 0 | 0.55 | 203 |
| Oka group (25) | 39 | 289 (289 to 290) | 1 | 35 | +40 | +20 | -25 | 0 | 0.65 | 187 (187 to 188) |

The values are the same in every run (the 5th to 95th is the median) except the Oka group's men, as shown, and part 2 (20), which fires in 993 runs (in 7 it is lost at sea and does not land). The fatigue is the unit's value at the moment of the fire. The landing fatigue is `unprepared_landing_fatigue` (40, every unit lands at night without rehearsal), the march fatigue is `fatigue_per_march_day` (10 for each of the two days of march before the day of the attack; the march of the attack day itself is added after the fire and is not in this figure), and the rest is the unit's rest of 5 a day at its landing hex between its landing and its march (1 to 8 days). "Other" (fatigue from a defeat, from being under fire) is 0. Part 2's fed share is 1 as for the others, and no attacker is short of food on its first day of attack in any run (the lowest fed share over all attackers and runs is 1).

**(b) Each side's weight of fire, and the limit.** The weight is the sum of shots times chance of a side's units' `ground_fire` events, and is the `weight` and `enemy_weight` of the `ground_attack_failed` event (they agree). The limit cuts the defender's shots only; the attacker's side is not cut in any fight. The limit is the `engagement_ratio` (3) times the attackers' *men ashore* (not their firing men): 3 x 916 = 2,748, 3 x 3,090 = 9,270, 3 x 289 = 867.

| Day, fight | Attacker's firing men | Attacker's shots | Attacker's weight | Defender's firing men (`side_firing_men`) | Defender's shots before the limit | `engaged_men` | Defender's shots after the limit | Defender's weight | Limit cuts |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| 16, Tenaru | 412 | 4.12 | 1.236 | 9,385 (9,356 to 9,405) | 93.85 (93.56 to 94.05) | 2,748 | 27.48 | 16.49 | 71 percent of the shots |
| 37, ridge | 2,147 | 21.47 | 6.441 | 12,437 (11,971 to 12,491) | 124.4 (119.7 to 124.9) | 9,270 | 92.70 | 55.62 | 26 percent of the shots |
| 39, the Oka group | 187 (187 to 188) | 1.87 | 0.561 (0.561 to 0.564) | 12,355 (12,284 to 12,425) | 123.6 (122.8 to 124.3) | 867 (867 to 870) | 8.67 (8.67 to 8.70) | 5.20 (5.20 to 5.22) | 93 percent of the shots |

The limit cuts in every run on all three days. On day 37, in the 7 runs where part 2 is lost, the Japanese weight is 5.751 and the Marines' 49.32 (engaged men 8,820). The weights' ratio of defender to attacker is 13.3 at the Tenaru, 8.6 at the ridge and 9.3 on day 39. Decision 0109, 1, says that before correction 1 the Marines fired on day 39 as if about 2,200 men were attacking; the figures of that run are not in this batch.

**(c) Men lost by each side on each day of each fight, at (2,-4)** (fight losses only, `men_lost_in_fight`; the 5th to 95th is of all 1,000 runs, 0 where there was no fight):

| Day | Japanese | American |
| --- | --- | --- |
| 16 (Tenaru) | 340 (240 to 420) | 14 (0 to 49) |
| 17 | 0 | 0 |
| 18 | 0 | 0 |
| 19 | 0 | 0 |
| Tenaru, days 16 to 19 | 340 (240 to 420) | 14 (0 to 49) |
| 37 (ridge) | 1,100 (960 to 1,260) | 101 (49 to 169) |
| 38 | 0 | 0 |
| 39 (the Oka group) | 100 (60 to 160), all of them the Oka group's | 0 (0 to 31) |
| Ridge, days 37 to 39 | 1,220 (1,060 to 1,380) | 114 (49 to 182) |

These two totals are the measures' figures. The only other days with a fight loss at (2,-4) are days 1 and 2, the American attack on the garrison: the Japanese lose 40 (0 to 80) on each, the Americans 20 (0 to 60) on day 1 and none on day 2.

**(d) The day each attack stops, with the men left.** In every run each attack stops on the day it begins: the unit is "too tired to attack" (its fatigue after the fight, which adds `fatigue_per_march_day` 10, `fatigue_per_attack_day` 20 and `failed_attack_fatigue` 30 to its fatigue at the fire, is above `attack_fatigue_limit` 60), goes back to defend, and the same day falls back toward (2,-5) with the men it has left. It falls back in the same step (a `spent_unit_fell_back` event). No unit attacks on a second day.

| Unit | Stops on | Fatigue then | Men left | Falls back to |
| --- | --- | --- | --- | --- |
| Ichiki echelon (17) | day 16 | 100 | 576 (496 to 676) | (2,-5) |
| Kawaguchi part 1 (19) | day 37 | 80 | 768 (702 to 827) | (2,-5) |
| part 2 (20), 993 runs | day 37 | 85 | 96 (88 to 103) | (2,-5) |
| part 3 (21) | day 37 | 95 | 644 (586 to 689) | (2,-5) |
| part 4 (22) | day 37 | 100 | 237 (217 to 255) | (2,-5) |
| part 5 (23) | day 37 | 100 | 236 (217 to 256) | (2,-5) |
| the five parts together | day 37 | | 1,970 (1,810 to 2,130) | (2,-5) |
| Oka group (25) | day 39 | 95 | 189 (129 to 230) | (2,-5) |

The attack as a whole fails (`ground_attack_failed`) on days 16, 37 and 39, one event each, in every run.

**(e) The fed share of every Japanese unit on the day it attacks:** 1 for each of 17, 19, 20, 21, 22, 23 and 25, in every run; the lowest over all runs is 1 (fed share by unit, from the `fed_share` factor of its `ground_fire` event). In the stock-fix runs the Oka group's fed share was 0.

**(f) Days of food at Lunga when the transports sail** (the Allied stock at its bases in (2,-4) over its men's daily charge at the end of day 3, the measure): **68 days** (68 to 68). The stock is 3,725.1 t (3,724.9 to 3,725.5) and the men fed 10,880 (10,840 to 10,900), at 0.005 t a man a day. The record is 17 to 37.

### What the figures say, and the surprises

- The Tenaru attack lasts one day and one night's fight in every run; the Ichiki echelon's 412 firing men are 45 percent of its 916 (the fatigue of 55, 40 of it from landing); the defenders' fire is cut to 29 percent of what 9,385 men could fire.
- The limit counts the attackers' men ashore, not their firing men, so tired attackers do not lower the cap: it is 2,748 against the 412 who fire at the Tenaru. This is how the code reads, by the figures above.
- Every Japanese attacker is fed in full on its first day of attack, in every run, and none is short of food; the cost of the Kawaguchi men falling back is gone from day 39 (0 lost, against 709).
- No attack happens on a second day, and no Japanese attack happens on any day but 16, 37 and 39. Part 2 of the Kawaguchi main body is lost at sea in 7 of 1,000 runs.
- On the scenario's seed, 1942, the Japanese lose 4,043 men (1,560 in the fight, 2,063 unfed, 420 surrendered) and the Americans 281 (247 in the fight, 34 to bombs and shells); the Allied margin is 95.78 (a decisive victory).

### How these figures were made

`sim-cli batch data/scenarios/07_full_slice.toml --runs 1000 --first-seed 1 --label before-tuning` (the release build), then `sim-cli validate` on that file; the rest from `sim-cli export data/scenarios/07_full_slice.toml --seed N --out FILE` for N = 1 to 1,000 (the event list, with breakdowns), read by a script and each file removed after it was read. The seed-N export is the batch's seed-N run: the men lost in the fight and unfed, by side, of all 1,000 seeds agree with the batch's `men_lost`. Scripts and the raw per-seed extracts are in the session's scratch folder, not in the repository. Figures not readable from the logs: none were estimated.
