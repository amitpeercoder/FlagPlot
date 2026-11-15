# rabaul-guadalcanal-air: validation of batch `ground-r1`

Battle: rabaul-guadalcanal-air
Scenario: data/scenarios/07_full_slice.toml
Batch: label `ground-r1`, 1000 runs, seeds 1 to 1000, file hash d87884667e45db48
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
| tenaru_american_men_lost | 109 to 110 | 14 to 86 | 46 | 1 of 1000 | 0 | **MISS**: the record is outside the middle 90 percent; fewer than 1 run in 20 holds the record |
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
- Spread over 1000 runs kept (0 left out): least 0, 5th 14, 25th 28, median 46, 75th 63, 95th 86, greatest 120

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

Allied margin over 1000 runs: 5th 73.965333, median 92.145333, 95th 105.615333

## Notes

- Percentiles are by nearest rank: the value at place ceil(p x n / 100) of the n kept runs, sorted. The record is inside the middle 90 percent if any part of it lies from the 5th percentile to the 95th, ends included. The verdict is "agrees, at the edge" where a measure agrees and no part of its record lies from the 25th percentile to the 75th, ends included; it still agrees. A run holds the record if its value is from the low end to the high end, ends included.
- Hits are counted as rolled: a salvo is rolled at the ships afloat when its round starts, so a hit on a ship another salvo sinks in the same round is counted (decision 0062).
- Escorts are matched to a raid's first strike group: an escort entry is added once, when its day, pulse and first group are those of an attack the measure counts (decision 0062).

## The first tuning round of the ground figures, beside the record

Written by hand after the report above, from the batch `ground-r1` and the event logs of the same 1,000 seeds. It closes round 1 of decision 0109 (`docs/decisions/0109-the-first-tuning-round-of-the-ground-figures.md`, "The round"). Each step's own record is under `docs/validation/tuning/`, as `ground-r1-1-who-can-fire.md` to `ground-r1-6-landing.md`. The report before the round is `docs/validation/full-slice-39-days-before-tuning.md`. State version flagplot-state-v23, rules label 7. Nothing in the code, the scenario's orders, its units or its stocks was changed; only four figures of `data/reference/ground.toml` moved.

**Every figure moved is fitted on the two fights of the slice and not checked.** No other battle reads the ground figures, so nothing checks them, and each stays a placeholder.

### Every measure, before and after the round

| Measure | Record | Before the round: 5th to 95th, median | After round 1 | Verdict: before, after |
| --- | --- | --- | --- | --- |
| lunga_american_end_of_day_2 | 1 | 1 to 1, 1 | 1 to 1, 1 | agrees, agrees |
| tulagi_american_end_of_day_3 | 1 | 1 to 1, 1 | 1 to 1, 1 | agrees, agrees |
| lunga_american_after_the_tenaru | 1 | 1 to 1, 1 | 1 to 1, 1 | agrees, agrees |
| lunga_american_end_of_day_39 | 1 | 1 to 1, 1 | 1 to 1, 1 | agrees, agrees |
| tenaru_american_men_lost | 109 to 110 | 5 to 59, 23 | 14 to 86, 46 | miss, miss |
| tenaru_japanese_men_lost | 800 to 900 | 240 to 420, 340 | 240 to 420, 340 | miss, miss |
| ridge_american_men_lost | 134 to 253 | 49 to 182, 114 | 82 to 218, 148 (638 runs hold the record) | reported |
| ridge_japanese_men_lost | 600 to 1138 | 1060 to 1380, 1220 | 1060 to 1380, 1220 (199 runs) | reported |
| tulagi_japanese_men_lost | 440 to 780 | 440 to 440, 440 | 440 to 440, 440 | reported |
| airfield_built_day | 11 to 13 | 11 to 11, 11 | 11 to 11, 11 | reported |
| squadrons_of_20_august_delivered_day | 14 | 14 to 14, 14 | 14 to 14, 14 | reported |
| days_of_food_at_lunga_when_the_transports_sail | 17 to 37 | 68 to 68, 68 | 35 to 36, 36 | reported |

Of 6 judged measures, 4 agree and 2 miss, as before: the Tenaru's two. The ridge's American losses move into the middle half of their record. The food at Lunga moves inside its record. The rest are where they were.

### The steps

| Step | Figure | Values tried | Kept | The step's measure, before and after |
| --- | --- | --- | --- | --- |
| 1. Who can fire | `unprepared_landing_fatigue`, `fatigue_per_march_day` | landing 40, 30, 20, 10, 0; march 10, 5 | **10 and 5** (from 40 and 10) | Ichiki firing men 412 to 778; Tenaru American losses 23 to 46 (still a miss) |
| 2. The limit | `engagement_ratio` | 1, 1.5, 2, 2.5, 3, 4, 6, 8, 12 | **3, stopped**: the Tenaru wants about 8, the ridge about 1.5 to 2.5 | Tenaru Japanese 340 |
| 3. What a hit does | `men_per_hit` (then the chances) | 20, 30, 40, 50 | **20, stopped**: the Tenaru wants about 50, the ridge 20 or less | Tenaru 46 and 340 |
| 4. When an attack stops | `attack_fatigue_limit`, `failed_attack_fatigue` | limit 60, 65; failed 30, 25 | **60 and 30, left** | every attack stops on its first day |
| 5. Cover | none: a report | cover 10, 20, 30, 50, 100 by the rule's arithmetic | | at cover 30, 0.517 of a raid's men saved against 0.27 stated |
| 6. The landing | `unprepared_landing_share`, `handling_t_per_man` | 17 pairs, from 0.5 and 0.25 down to 0.10 and 0.02 | **0.13 and 0.02** (from 0.5 and 0.25) | food at Lunga 68 days to 36, no American unfed |

**Where the two fights wanted different values.** Steps 2 and 3, and nowhere else. Step 1 moved both fights the same way, and step 6 touches neither.

Both stops have one cause. While the limit on engaged men binds, which it does at both fights for any ratio up to 4, the defender's fire is the ratio times every man the attacker has ashore in the hex. So the Japanese loss at the ridge is 3,090 / 916 = 3.37 times that at the Tenaru, whatever the figures. The American loss is about 2,781 / 778 = 3.6 times, by the attackers' firing men. The record has 0.7 to 1.4 and 1.2 to 2.3. The architect confirmed both stops on Nov 15, 2026.

### The men each side lost over the 39 days, by cause

From the batch, 5th, median and 95th over 1,000 runs:

| | Before the round | After round 1 |
| --- | --- | --- |
| American men lost to bombs and shells | 20, 43, 67 | 20, 43, 67 |
| American men lost in the fight ashore | 118, 189, 264 | 162, 242, 329 |
| American men lost unfed | 0, 0, 0 | 0, 0, 0 (no American is unfed on any day of any run) |
| Japanese men lost in the fight | 1480, 1680, 1860 | 1480, 1680, 1860 |
| Japanese men lost unfed | 2026, 2096, 2162 | 2026, 2096, 2168 |
| Japanese men surrendered | 360, 400, 440 | 360, 400, 440 |
| Allied margin | 83.72, 101.64, 115.27 | 73.97, 92.15, 105.62 |

All 1,000 runs are a decisive Allied victory, as before. The margin falls for two reasons. The Americans lose more men in the fight, because the Japanese fire with more men. And less cargo is landed at Lunga, which an Allied objective counts: in seed 1942, 4,390 t against 6,195.

### A typical run after the round, in plain words

Medians over the 1,000 runs, with the 5th to 95th where it is not a single value. Read from the event logs of `export --seed N`, N = 1 to 1,000; the men lost by side and cause of every seed agree with the batch.

**The landing, 7 to 9 August.** The two transport forces arrive on day 1. All 16,975 men are ashore that day: 10,880 at Lunga (10,840 to 10,900, after the day's fight) and 6,055 at Tulagi. Before the round only 6,079 were ashore at Lunga on day 1, and the rest on day 2.

The supply comes more slowly. The ships land 218.8 t at Lunga on day 1 and 1,310.4 t on day 2, and 252.9 t and 748.8 t at Tulagi. They sail on the night of day 3, by the scenario's order, with 1,740.8 t and 820.3 t still aboard (53 and 45 percent of their cargo), and unload it at Espiritu Santo on day 5.

At the end of day 3 the Americans at Lunga hold 1,960.3 t for 10,880 men: 36 days. Before the round they held 3,725 t, 68 days. No American goes unfed on any day of the 39. The least Allied stock at Lunga after day 3 is 1,473 t (1,269 to 1,475), about 27 days, on day 14 or day 39. At Tulagi it is 88 t (85 to 94) on day 39.

**The Tenaru, day 16.** The Ichiki echelon (land unit 17) lands at (3,-4) on the night of day 13, 916 men with 32.06 t of rations, at fatigue 10. It rests once to 5, marches on days 14, 15 and 16, and reaches the airfield's hex on day 16 at fatigue 15. It attacks that day with 778 of its 916 men firing (412 before the round).

The Marines fire 27.48 shots at 0.6 (a weight of 16.49), cut by the limit from 112.4; the echelon fires 7.78 shots at 0.3 (2.33). The attack fails, as before. The echelon loses 340 men (240 to 420) and the Americans 32 (0 to 85; with 9 lost to bombs on the same days, the measure's 46).

The echelon goes back to defend at fatigue 70, falls back toward (2,-5) the same day with 576 men (496 to 676), and reaches it on day 19. There is no fight on days 17 to 19.

**The ridge, day 37.** The five Kawaguchi parts land at (3,-4) on days 26 to 32, 3,090 men with 216.3 t, and are fed in full to day 37. Each has rested its landing fatigue of 10 away. They march on day 35 and attack on day 37 at fatigue 10, with 2,781 men firing (2,147 before).

The Marines fire 92.70 shots at 0.6 (55.62), cut from 124.2 by the limit; the Japanese 27.81 at 0.3 (8.34). The attack fails. The Japanese lose 1,100 (960 to 1,260) and the Americans 135 (66 to 205; 101 before).

All five parts go back to defend at fatigue 65 and fall back with 1,970 men (1,810 to 2,130). There is no fight on day 38.

**The Oka group, day 39.** The Oka group lands at (2,-5) on day 32, 333 men, and marches from day 37. It attacks on day 39 with 289 men ashore and 260 firing (187 before), against the Marines limited to three times its men ashore. It loses 100 (60 to 160) and the Americans 13 (0 to 31). No Kawaguchi or Ichiki man is lost on day 39, in any run. The ridge's measure, days 37 to 39: Japanese 1,220, Americans 148.

**Unchanged.** The west and Taivu are as before the round, every figure checked against the logs:

- The Japanese stocks and who is fed in (2,-5) and (3,-4).
- Every Japanese fed share on its day of attack, 1.
- The airfield, built on day 11.
- The squadrons, delivered on day 14.

### Step 5: what cutting each share to a whole man does

From `ground-r1-5-cover.md`. The share of a raid's men that cover saves, over the 5,642 raids on Lunga in the 1,000 runs, recomputed by the rule with the cover set at each level (and checked against every one of the 18,159 events of the runs):

| Cover | The share the figure states | The share the rule gives |
| --- | --- | --- |
| 10 | 0.090 | 0.383 |
| 20 | 0.180 | 0.383 |
| 30 | 0.270 | 0.549 (the runs, at cover 30 less wear: 0.517) |
| 50 | 0.450 | 0.583 |
| 100 | 0.900 | 1.000 |

The step moves no figure. The architect rules on the rule.

### Every live scenario whose result changed

`sh scripts/compare-scenarios.sh`, this tree against the merge of pull request 318: 118 runs compared, 111 the same with the day's hashes cut, 7 differ. **Every run on the frozen figures is the same.** The seven are all live runs, and each change is what the four figures do: men land faster, landing and marching tire less, and less supply is landed while men are landed.

- **07, the full slice** (seed 1942). All 16,975 men land on day 1 (11,209 before). The Americans' attack of day 2 is no longer stopped by fatigue (fatigue 50, where three units were at 80).
  - Lunga's stock at the end of day 3 is 1,741.5 t (3,569.9 before), and the transports take their remaining cargo back to Espiritu Santo.
  - The Japanese fire with more men in all three attacks: 778, 2,781 and 260. They still fail, and each still stops on its first day.
  - Losses: Japanese 4,033 (4,043 before), Americans 348 (281 before; the 67 more are all on day 37). The Allied margin is 85.99 (95.78 before), still a decisive victory.
- **21, a landing.** The two units are ashore by noon of day 2, where the last men landed in the afternoon. The base's supply lands sooner. Landing fatigue is 10, not 40. The men lost (all unfed, 238), the men left and the bases are the same.
- **25, a march.** A march day tires 5, not 10. The Allied regiment fires with 900 and 611 men in its two attacks (800 and 470 before). The second still fails, and it still goes back to defend tired. Men, supply and bases are the same.
- **27, preparing a landing.** On day 11 the three forces land all their men that day, where two finished on the night of day 12. Base B's supply is 160 t at the end of day 11 (100 before) and 154 t at the end (156). The men and the unfed are the same, and every unit ends at fatigue 0.
- **29, the skills.** Only fatigue: each march day tires 5, not 10. No landing, fight, loss or supply changes.
- **32, falling back.** Only fatigue: a unit falling back gains 5 a day, not 10. The fights, men, surrender and bases are the same.
- **37, men falling back.** Fatigue falls with the march figure. The Japanese attack of day 3 fires with 270 men (240 before) and still fails. Losses, men and bases are the same. The scenario's hand calculation runs on the frozen figures and is unchanged.

## Closing: what the round did and could not do

### Each figure moved

| Figure | Before | After | Its standing |
| --- | --- | --- | --- |
| `unprepared_landing_fatigue` | 40 | 10 | Placeholder. Fitted on the two fights of the slice in round 1 and not checked. Direction from land_unit_facts 1049, 994, 1020; size from no row ("in strength") |
| `fatigue_per_march_day` | 10 | 5 | Placeholder. Fitted on the two fights of the slice in round 1 and not checked. Direction from 994, 1020 |
| `unprepared_landing_share` | 0.5 | 0.13 | Placeholder. Fitted on the slice in round 1 and not checked. Direction from 465 and 773; size bounded by no American going unfed |
| `handling_t_per_man` | 0.25 | 0.02 | Placeholder. Fitted on the slice in round 1 and not checked. Moved so the men land on day 1 (688) |

No other figure moved. `engagement_ratio`, `men_per_hit`, `hit_chance`, `defender_hit_chance`, `attack_fatigue_limit`, `failed_attack_fatigue`, `cover_from_fire` and `cover_in_fight` are as they were, placeholders resting on no source. The frozen figures (`data/reference/step1`) are untouched, and every hand calculation stands.

### What the round could not reach

**1. The Tenaru's losses, both sides (judged, still missing).** One figure (`men_per_hit` at about 50) would bring both into the middle half, but it would make the ridge three times too bloody. A ratio of about 8 would bring the Japanese losses in, against the rows and with the same cost at the ridge. The fights cannot be fitted together while the limit counts all the men ashore. It would take the two things below, which the record has and the game does not.

**2. A defender's counter-attack that destroys a spent attacker.** At the Tenaru the Japanese were not only thrown back. What the rows have:

- **The night.** The first rush came at 0310 on 21 August across the sand bar at the mouth of the Ilu, about 200 men, then a second company (land_unit_facts 857, 1022, 1024; engagement_outcomes 1923, 1925, 1926). A company-size party went into the sea and came ashore against the left of the 2d Battalion, 1st Marines (eo 1913).
- **The defence.** The 2d Battalion, 1st Marines held the line (land_unit_facts 525; eo 1914). The 1st Battalion, 1st Marines was ordered out of division reserve, Company G counter-attacked, the 3d Battalion, 11th Marines fired its howitzers, and a platoon of light tanks crossed the bar at 1500 (eo 1935).
- **The end.** The Japanese record has the enemy counter-attack the next afternoon killing most of the Advance Unit, its commander and practically all its officers (eo 1921). The action ended at about 1700 (eo 1919; land_unit_facts 857). Almost 800 were lost and only 130 survived (eo 1920, 1930); the few survivors fled east toward Taivu (eo 1918; land_unit_facts 270).
- **The gaps.** No row in `data/oob` says where the 1st Battalion went: that it crossed upstream and came in behind the Japanese is not in the tables. Nor do they give its time or the men of any American unit engaged. "Encircled" is in the researchers' notes to eo 1914, not in a quoted source. These are gaps.

**The slice's orders have none of it.** Every Allied land unit is put on defend on day 3 (units 1, 2 and 3) or day 4 (units 5 to 8), and none has a later order to attack or to march (`data/scenarios/07_full_slice.toml`, the orders of days 3 and 4). The Americans stand on the defence from the day the base is theirs.

**What stands in the way:**

- **The orders.** A counter-attack is an order, and the orders are data set from rows before the first round and not changed to improve a report (`docs/step2-spec.md`, "Tuning", rule 8). Rows do name the 1st Battalion's order out of reserve and Company G's counter-attack (eo 1935). Taking them into the scenario is a data change for the architect, made by the reading of rule 8 and applied to every battle alike.
- **One fight a day.** The fight is worked once a day. The night's assault and the afternoon's counter-attack would be one fight in which both sides attack. The Marines would then fire as attackers, at 0.3 and with no cover, and the spent echelon would fall back in the same step.
- **No position inside a hex.** No unit can be behind another, so a column cannot be cut off. A beaten side falls back to the first linked hex with a base of its side, or with no enemy fighting man. It surrenders only where there is none, so a unit enveloped in its own hex still walks out.
- **Where the echelon falls back to.** The echelon now falls back west, to (2,-5), where the record's survivors went east toward Taivu. The rule of a beaten side reads the base of its side in a linked hex.

**3. An attacker who commits part of his force and holds the rest.** What the rows say of 12 to 14 September:

- **The probe.** On the night of 12 September only one battalion had reached its jump-off point (land_unit_facts 859), and it probed the raider and parachute line lightly (1017, 1001). The rest of X-day was put off a day because the main force had not arrived (987).
- **The main attack.** On the night of 13 to 14 September "at least two battalions" (1004) attacked, "a two-battalion attack against the center and right" (1018; engagement_outcomes 2014, 2015, 2019). It came at 1830 and 2130 (1012, 1013) and ended by dawn (1004).
- **How many attacked.** Of about 2,000 who attacked the ridge about 600 were killed (eo 2013, 2023; land_unit_facts 1009, 311). Hoffman has 2,500 for the main assault, with one of four battalions detailed to attack along the Tenaru (859). The scenario's main body is 3,090 (310: the 35th Brigade headquarters and the 124th Infantry).
- **East of the perimeter.** A second unit, of about two reinforced companies to a battalion, attacked the right flank of the 3d Battalion, 1st Marines on the Ilu about midnight (1006, 1014, 1019; eo 2016, 2017, 2020, 2021). It was the Ichiki rear echelon by Miller and Hough, or the 1st Battalion, 124th Infantry by Zimmerman. About 200 were killed there, not counted in the 600 on the ridge (eo 2028, 2029).
- **The Kuma battalion.** It was 658 men of the Ichiki second echelon on 1 September (979). It sat on the Tenaru line, and its antitank company fired no shot in the night (980, 984, 986, 988, 989). It began to retreat at 1400 on 14 September.
- **The west.** The Oka group, about 1,000 at the start, a third of it at Malbobo, about 650 in the attack (916, 917, 925), struck on the afternoon of 14 September (1007, 949). Wikipedia has 0400 (926).
- **The Aoba part.** 637 men landed at Kamimbo on 11 September (troop_lift_facts 168; land_unit_facts 942). It was too late for the attack and did not attack.
- **The gaps.** No row gives the men of a named battalion at the ridge, or ties the battalions named in a diary (Kokusho, Watanabe, Tamura) to a regiment or a number.

**In the slice,** all five Kawaguchi parts (units 19 to 23, 3,090 men) are ordered to march on day 35 and attack on day 37, and they attack together, all 2,781 firing men. The parts are a landing schedule, not battalions: the scenario says they are not a statement of which unit came on which night. The Oka group (unit 25, 333 men) attacks on day 39. The Aoba part (24), the force of 29 August (18) and the Yokosuka force (16) have no order.

**What stands in the way:**

- **The rules: there is no position inside a hex.** The whole Lunga perimeter is the one hex (2,-4). The ridge, the Ilu flank and the Tenaru line are one fight, and all of each side's units in the hex fight as one body.
  - A unit held back in the hex is in the fight: the units on defend of a side that attacks fire with the rest, as attackers, and share the losses.
  - The limit counts every fighting man the other side has ashore in the hex, not the part on attack.
  - So a reserve can be held only in another hex.
- **The data.** The scenario's units are not the battalions that attacked. No row gives their men, so they cannot be built from rows as they stand.
- **The orders.** Which part attacked on which night is an order, set from rows before the round and not changed after it (rule 8).
- **What would mend it.** A rule that engages only part of each side, for example a limit that counts only the attacker's units on attack, or a frontage. Then the units of the main body by battalion, from rows. Both are rule and data work for the designer and the architect, outside a tuning round.

**4. Two nights at the ridge.** Step 4 found that a limit of 65, or a failed day's fatigue of 25, gives one day at the Tenaru and two at the ridge. It does so only on a margin of five points of fatigue from the landing figure, and its second day is a spent force at a third of its fire, which takes the ridge's Japanese losses out of their record. In the record the second night was the main attack, after a probe on the first night: the thing named in 3.

**5. Cover that grades.** Cutting each piece of a raid's loss to a whole man makes cover save about twice the share its figure states. Cover 10 and 20 save the same, 30 and 50 nearly the same, and full cover saves every man. That is the rule's arithmetic, for the architect to rule on (step 5).

**6. The food at Lunga at the middle of its record.** 36 days is inside 17 to 37 but near its top. Every value tried that gives fewer days leaves Americans unfed later in the slice, first at Tulagi. Reaching the middle would take the slice's later supply runs, which are data set from rows, not a figure.

**Not in this round** (decision 0109, "What the round will not mend"):

- **No enemy against the landing.** The landings meet no enemy at sea or in the air.
- **No shelling of Lunga.** No ship shells Lunga.
- **All real units alike.** Every real unit's experience and skills are 2.
- **What a side is told.** What a side is told of its own attack is not in this round.

### How these figures were made

The round batch:

- `sim-cli batch data/scenarios/07_full_slice.toml --runs 1000 --first-seed 1 --label ground-r1` (the release build), then `sim-cli validate` on that file.
- The batch's runs are the same, run by run (every final hash), as the last trial of step 6 (label `r1-s6-s13h02`), which was made on a scratch copy of the same figures with `--reference`.

Each step's trials: 1,000 runs on a scratch copy of the reference files with the step's values, and `validate` with the same `--reference`.

The firing men, weights, limits, days of attack, fed shares, landing and supply read from `export --seed N` for N = 1 to 1,000, by scripts in the session's scratch folder, not in the repository. The men lost by side and cause of every seed agree with the batch.

Step 5's table is from the same logs, with the rule's arithmetic reproduced in exact fractions and checked against every event. Figures not readable from the logs: none were estimated.

## After the cover ruling

Written by hand on Nov 15, 2026, after the architect's ruling on decision 0106 ("cut once"): a unit's pieces of one raid or shelling are worked by the cover each hit meets, added, and cut to a whole man once. The figures are those of round 1; none moved. Batch `ground-r1-cut`, 1,000 runs, seeds 1 to 1,000 (file hash ccdac417be26bd79, state version flagplot-state-v24, rules label 8), then `sim-cli validate` with `data/validation/guadalcanal_landings.toml`. The batch before is the same command on the build of main before the ruling (label `ground-r1`, the runs of this report).

**The measures that moved.** Only two. `tenaru_american_men_lost` goes from 14 to 86 (median 46) to 14 to 90 (median 49), and still misses, with 1 run of 1,000 holding the record of 109 to 110: the Americans lose a few more men to the bombs that fall on Lunga on those days. The Allied margin goes from 73.97, 92.15, 105.62 to 73.94, 92.04, 105.52 (5th, median, 95th), because the Allied men lost count against it. All 1,000 runs are still a decisive Allied victory. Every other measure is as it was, to its percentiles, and so are the American men lost in the fight (162, 242, 330 against 162, 242, 329) and every Japanese loss.

**American men lost to bombs and shells over the 39 days** (5th, median, 95th, from the batch): 20, 43, 67 before, and 22, 50, 85 after. In all, 42,409 men before and 51,640 after over the 1,000 runs, a fifth more.

**What cover now saves.** The table of step 5 (`docs/validation/tuning/ground-r1-5-cover.md`), redone by the new rule: the calls, units and hits of the 1,000 runs (`export --seed N`), with the cover set at each call's first hit and worn 0.2 a hit after. The arithmetic was reproduced in exact fractions and checked against the runs' events: of 5,642 calls, 5,616 were rebuilt and matched every `men_lost_to_fire` event in them (each unit's part and the men lost); 26 could not be rebuilt from the state between phases (a unit's men changed within the day) and are left out.

| Cover | Share the figure states | Share the rule gives now | Before the ruling | Men lost, of 87,610 |
| --- | --- | --- | --- | --- |
| 10 | 0.090 | 0.269 | 0.383 | 64,040 |
| 20 | 0.180 | 0.321 | 0.383 | 59,495 |
| 30 | 0.270 | 0.444 | 0.549 | 48,699 |
| 50 | 0.450 | 0.538 | 0.583 | 40,476 |
| 100 | 0.900 | 0.984 | 1.000 | 1,417 |

By the size of a unit's part (share saved):

| Part P | Parts | Men before | Cover 10 | 20 | 30 | 50 | 100 |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1 to 4 | 18,395 | 47,255 | 0.389 | 0.389 | 0.546 | 0.561 | 1.000 |
| 5 to 9 | 3,093 | 23,234 | 0.133 | 0.265 | 0.339 | 0.529 | 1.000 |
| 10 to 19 | 1,403 | 16,966 | 0.122 | 0.208 | 0.306 | 0.486 | 0.917 |
| 20 and over | 7 | 155 | 0.090 | 0.200 | 0.290 | 0.465 | 0.910 |

In the runs as they are (the cover each unit met, about 30 less wear), the Allied units at Lunga lost 51,510 of 87,610 men before cover: 0.412 saved, where the figure states 0.259. Before the ruling it was 0.517.

**What it says.** Cover now grades: 10, 20, 30 and 50 save different shares, and a part of 10 men or more saves close to what the figure states. It still saves more than the figure at Lunga, because most of a raid's parts there are of 1 to 4 men (18,395 of 22,898), and a part is still cut to a whole man once for each unit: a part of 1 man behind any cover above 0 loses nothing (0.91 of a man at cover 10, cut), and a part of 4 behind cover 30 loses 2 (2.92, cut). The cut that remains is one per unit and call, not one per hit. Nothing was tuned.
