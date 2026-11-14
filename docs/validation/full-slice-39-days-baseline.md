# rabaul-guadalcanal-air: validation of batch `baseline`

Battle: rabaul-guadalcanal-air
Scenario: data/scenarios/07_full_slice.toml
Batch: label `baseline`, 1000 runs, seeds 1 to 1000, file hash 348d3476806e1ad6
Inputs: state hash layout flagplot-state-v20, scenario file hash d192af3a6fe3da7a
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
- data/scenarios/../reference/ground.toml dfebfaded048966f
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
| tenaru_japanese_men_lost | 800 to 900 | 916 to 916 | 916 | 12 of 1000 | 0 | **MISS**: the record is outside the middle 90 percent; fewer than 1 run in 20 holds the record |
| ridge_american_men_lost | 134 to 253 | 80 to 240 | 160 | 697 of 1000 | 0 | reported |
| ridge_japanese_men_lost | 600 to 1138 | 3325 to 3327 | 3326 | 0 of 1000 | 0 | reported |
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
- Spread over 1000 runs kept (0 left out): least 840, 5th 916, 25th 916, median 916, 75th 916, 95th 916, greatest 916
- Each value and its runs: 840: 1, 860: 3, 880: 1, 900: 7, 916: 988

## Result on the objectives scale (reported, not judged)

Seen from the Allied side, runs of each result:

- decisive victory: 1000 of 1000
- major victory: 0 of 1000
- minor victory: 0 of 1000
- stalemate: 0 of 1000
- minor defeat: 0 of 1000
- major defeat: 0 of 1000
- decisive defeat: 0 of 1000

Allied margin over 1000 runs: 5th 111.997083, median 129.857083, 95th 143.827083

## Notes

- Percentiles are by nearest rank: the value at place ceil(p x n / 100) of the n kept runs, sorted. The record is inside the middle 90 percent if any part of it lies from the 5th percentile to the 95th, ends included. The verdict is "agrees, at the edge" where a measure agrees and no part of its record lies from the 25th percentile to the 75th, ends included; it still agrees. A run holds the record if its value is from the low end to the high end, ends included.
- Hits are counted as rolled: a salvo is rolled at the ships afloat when its round starts, so a hit on a ship another salvo sinks in the same round is counted (decision 0062).
- Escorts are matched to a raid's first strike group: an escort entry is added once, when its day, pulse and first group are those of an attack the measure counts (decision 0062).

## What this report is

The baseline of the full slice lengthened to 39 days, 7 August to 14 September 1942 (decisions 0093 and 0101, with the architect's rulings at the end of 0101), before any tuning. Only its land fight is measured, by `data/validation/guadalcanal_landings.toml`; its sea and air parts are run and not compared with anything. No figure was tuned: every ground figure is the placeholder of `data/reference/ground.toml`. The tuning round is the architect's to plan. The batch is 1,000 runs, seeds 1 to 1,000, on the scenario and figures whose hashes are above; the day-by-day account below is of the scenario's own seed, 1942, whose numbers sit near the batch's medians unless it says otherwise. Day 1 is 7 August, and a game day runs from 18:00 the evening before its date.

## What happens ashore, in plain words

**7 August (day 1).** The Guadalcanal transports reach Lunga at 10:00 and the Tulagi group reaches Tulagi at 06:00. They begin landing men on attack at once, at half the port's rate because no force prepared its landing, and each unit is tired by 40 as it lands. At the end of the day about 5,700 Americans are ashore at Lunga against 240 naval guard troops and 1,800 construction men. The guard is beaten in the first fight, Lunga changes side with half its 1,090 t (545 t, less a night's food) and its 400 t of fuel, and the guard and both construction units fall back west toward (2,-5). On Tulagi about 5,000 Americans beat the 240 of the garrison, and Tulagi changes side. Its men, the garrison's survivors and the 200 construction men, have no land link to go by, and surrender: 440 Japanese men lost on Tulagi in every run. The Americans lose about 20 men at Lunga and 20 at Tulagi over the first three days (medians).

**8 to 9 August (days 2 and 3).** The Americans are still on attack on day 2 and fire on the Japanese who have not yet left the airfield's hex: the Japanese lose about 610 men at Lunga over days 1 to 3 (median), some of them unfed, since their side has no base there now. The American units go back to defend at a fatigue of 80, a day before the order of day 3 would have sent them. By the afternoon of day 2 every man and every ton of the 5,092 t aboard is ashore, so when the transports sail for Espiritu Santo on day 3 the Americans at Lunga hold 68 days of food, against the record's 17 to 37.

**10 to 20 August (days 4 to 14).** The garrison and the construction men reach (2,-5) on day 4 with 1,341 men and no supply, onto a stock of 72 t sized for the men who will land there later; from about day 14 the west is hungry, and it loses men unfed on most days to the end (about 1,760 Japanese men lost unfed in (2,-5) over the run, median). The 1st Pioneer Battalion works on the field from day 2 and the CUB One detachment lands on day 9; the airfield is built on day 11, 17 August, in every run, the first of the record's three days. The Oite's 200 men land in (2,-5) on the night of day 11, and the Ichiki echelon's 916 at Taivu, (3,-4), on the night of day 13. VMF-223 and VMSB-232 arrive on day 14, 20 August, the record's day, in every run.

**The Tenaru (day 16, 22 August).** Ichiki is ordered to march and attack on day 14, the first day it can take the order (ruling C), and reaches the airfield on day 16 with 916 men, of whom 412 fire, tired by the landing and the march. It meets the whole American force in the hex, about 10,000 men on defend, and every one of its men is lost that day, in every run but a few: 916 against the record's 800 to 900, with the 130 the record has surviving. The Americans lose 40 men (median; 20 on this seed) against the record's 109 or 110. The airfield is American at the end of day 16 in every run. The 2d Battalion, 5th Marines crosses from Tulagi on day 15 and the 2,000 t of 21 August are landed on days 15 and 16.

**21 August to 11 September (days 15 to 36).** The Eastern Solomons carriers fight on day 18 as in the fourteen-day slice. The Rabaul bombers raid Lunga on the record's days and kill about 90 Americans ashore over the period (median), the shells and bombs that fall on men. The Tassafaronga force lands on day 24 and the five parts of the Kawaguchi main body at Taivu on days 26, 28, 30, 31 and 32; the Raiders and the Parachutists cross on day 25 and the Seabees land on day 26. The east is fed: 248 t at Taivu keep its men whole. The Oka group lands in (2,-5) on day 32 and the Aoba part on day 37.

**The ridge (days 37 to 39, 12 to 14 September).** The five parts march on day 35 and reach the airfield on day 37 with 3,090 men. They carry five days of food, which forms a Japanese dump in the airfield's hex. They fight on days 37, 38 and 39. The tired parts go back to defend, but the fight goes on while any Japanese unit attacks, and a side that attacks is not beaten while the other defends, so nobody falls back. The Oka group reaches the field from the west on day 39 with 236 of its 333 men, the rest lost unfed, and attacks. By the end of day 39 every Japanese man at the field is lost: about 3,326 against the record's 600 to 1,138, and the Americans lose 160 (median) against the record's 134 to 253, which 70 percent of runs hold. The airfield is American at the end of day 39 in every run. The men in (2,-5), the Tassafaronga force, the Aoba part, the Oite's men and the remnants of the garrison, stay there, as decision 0101 orders.

## What looks wrong, and what causes it

Known before the run and recorded in decisions 0093 and 0101, and seen here:

- **A fight is the whole of both sides in a hex.** At the Tenaru and the ridge the Japanese meet 10,000 men on defend at `defender_hit_chance` 0.6, `men_per_shot` 100 and `men_per_hit` 20: over 100 hits a day, more than the whole attacking force. Every Japanese man who reaches the field is lost, at once at the Tenaru and over three days at the ridge. The rule (every man in the hex fights) and these three figures cause it.
- **An attacker that is not beaten cannot break off.** A side is beaten only where the other attacked, so the Japanese, attacking a side on defend, never fall back and are fought to the last man. No order in the scenario takes them off. This is the rule of decision 0104 as built.
- **The Japanese fire with half their men or fewer.** Ichiki fires 412 of 916: the landing fatigue of 40 (`unprepared_landing_fatigue`), three days of march at 10 a day and its fed share. The American losses at the Tenaru are a third of the record's because of it, with `hit_chance` 0.3.
- **The attack from the east is a day late.** It is on day 16 and not day 15 (ruling C), because the march takes `march_days_per_hex` 3 and Ichiki cannot be ordered to march on the night it lands.
- **The transports land everything in two days,** where the record's landed less than half: the placeholder port rates and `handling_t_per_man`. Days of food at Lunga when they sail are 68, against 17 to 37.
- **Lunga falls on day 1 and Tulagi on day 1.** The record has the field taken on 8 August and Tulagi secured by 9 August. The garrisons are a few hundred fighting men against thousands, and `beaten_share` 0.2 beats them in the first fight. The measures of the second and third days agree, because they ask only who holds the base by then.
- **The west starves.** The garrison of Lunga and the construction units fall back to (2,-5), 1,341 men, onto a stock sized for the 2,070 men to land there later (decision 0101, as amended, 5), and the west then loses men unfed at `unfed_loss_share_per_day` 0.05 for most of the period. The Oka group reaches the field with 236 of 333.
- **A march forms a dump inside the enemy's perimeter.** Ichiki and the Kawaguchi parts carry `march_supply_days` of food, and the supply they carry forms a Japanese base in the airfield's hex, which the Americans hold.
- **The construction units in the west work an airfield.** A working unit's default job is the airfield where its base has none, so the 11th and 13th Construction Units start building one at the stock west of the Lunga (base 6). None is finished in the 39 days.
- **The Americans go back to defend by fatigue on day 2,** before their order of day 3: 40 on landing and 20 a day on attack pass `attack_fatigue_limit` 60.
- **Nothing attacks the landings from the air or the sea.** No order sends the Rabaul bombers before 21 August, there are no escorts or covering cruisers, and there is no night action off Savo Island (rulings G and H; the gaps of decision 0101).
- **The landings' ships refuel from the captured fuel.** On day 2 the Guadalcanal transports take most of the 200 t of fuel kept at Lunga, the slice's placeholder for coverage of refuelling.
- **Sickness is not modelled,** nor the raid on Tasimboko, the patrols, or the men of the 1st Engineer Battalion (gaps).
- **The objectives scale is a placeholder,** and with the land objectives added every run is a decisive Allied victory.
