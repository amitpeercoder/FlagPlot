# Round 2: the factor against a surprised target, 1.5 to 2. The chance to hit is then closed

Savo Island, Nov 13, 2026. The second mechanism in the order of tuning, the chance of a hit. Ruled by the coordinating architect before the move, at 2 and not at the 2.5 the build session proposed. Whether it is kept or reverted is the coordinating architect's word, on this record.

## What moved

| File | Figure | From | To |
| --- | --- | --- | --- |
| `data/reference/surface_combat.toml` | `surprised_target_factor` | 1.5 | 2 |

Nothing else. **Its kind:** a surface figure, tuned on Savo Island and not checked until Cape Esperance is run.

## The row its size rests on

Canberra is the record's one clean case of fire at a surprised ship. She "was hit 24 times in less than two minutes" (`engagement_outcomes` 21, medium, preferred), "not less than twenty-four 8-inch and 4.7-inch hits" (rows 19 and 20, high, preferred), made no reply with her main battery, and the Japanese passed on. In the model that is this factor's case exactly: in the 577 runs of 1,000 in which the southern group is surprised, its fight lasts a median of one round.

| Factor | Canberra's gun hits in those 577 runs: 25th, median, 75th |
| --- | --- |
| 1.5 | 9, 11, 15 |
| 2 | 12, 14, 17 |
| 2.5 | 15, 18, 21 |
| 3 | 19, 22, 25 |

Where the group is not surprised she takes a median of 10 whatever the factor.

**The arithmetic of the size.** The 24 hits are of every calibre and a ship in the rules fires her main battery only. The measures set the two side by side on the assumption that the secondary guns shot as the main did, and the Japanese main batteries fired 1,090 to 1,190 of the 1,867 shells, about three fifths. Three fifths of 24 is about 14 main battery hits, and the surprised Canberra's median is 14 at a factor of 2. At 2.5 it is 18, three quarters of 24, which would ask the factor to carry part of what the secondary guns scored. No row gives the main batteries a larger share of the fire at Canberra in particular.

**What that support is worth.** It is one ship's, in the battle the figure is tuned on. It will be tested at Cape Esperance on the other navy, where the Japanese were the surprised side and Aoba and Furutaka took about 40 and about 90 hits (`engagement_outcomes` 1598 to 1602). Until then the figure stays a placeholder as to its size.

**It carries one round.** The rules end a force's surprise in the round after it is first fired on, and the record has about one round of surprise under fire (decision 0076). The factor reaches 17 in 100 of the Japanese gun salvos at Allied ships, all of them in that round.

## It is also a torpedo figure

The factor multiplies the chance of a torpedo at a surprised target as it does a shell's. Torpedo hits on Allied ships go from a median of 3 to 4, toward the record of 4 to 9 (524 runs hold it, from 410). No other torpedo figure was touched.

**What decision 0074 would have changed** (it is written and held): with the tubes that bear on one side, the five Japanese heavy cruisers would fire 4 torpedoes each in the surprised round and not 8, so about half of that gain would not be there. The round is not held for it.

## Every measure, before and after

1,000 runs each, seeds 1 to 1,000. Each cell is the 5th to the 95th percentile, the median, and in brackets the runs holding the record. Before is `savo-island-r1.md`; after is `savo-island-r2.md`.

| Measure | Record | Before | After | Verdict |
| --- | --- | --- | --- | --- |
| allied_cruisers_sunk | 4 | 0 to 5, 2 (204) | 0 to 5, 3 (221) | agrees |
| southern_group_cruisers_sunk | 1 | 0 to 2, 1 (467) | 0 to 2, 1 (399) | agrees |
| northern_group_cruisers_sunk | 3 | 0 to 3, 1 (302) | 0 to 3, 2 (368) | agrees |
| allied_destroyers_sunk | 0 | 0 to 1, 0 (924) | 0 to 1, 0 (862) | agrees |
| japanese_ships_sunk | 0 | 0 to 1, 0 (843) | 0 to 1, 0 (840) | agrees |
| japanese_ships_with_propulsion_out | 0 | 0 to 1, 0 (871) | 0 to 1, 0 (862) | agrees |
| japanese_ships_sunk_or_stopped | 0 | 0 to 1, 0 (728) | 0 to 2, 0 (717) | agrees |
| night_fights_the_japanese_did_not_sight_first | 0 | 0 to 2, 1 (335) | 0 to 2, 1 (335) | agrees |
| southern_group_rounds_surprised | 1 to 2 | 0 to 1, 1 (577) | 0 to 1, 1 (577) | reported |
| northern_group_rounds_surprised | 2 to 3 | 0 to 1, 1 (0) | 0 to 1, 1 (0) | reported |
| gun_hits_on_allied_ships | 153 to 219 | 35 to 151, 108 (42) | 35 to 158, 116 (88) | a miss, now agrees at the edge |
| gun_hits_on_japanese_ships | 10 to 17 | 9 to 69, 25 (231) | 8 to 69, 24 (227) | agrees |
| torpedo_hits_on_allied_ships | 4 to 9 | 1 to 6, 3 (410) | 1 to 8, 4 (524) | agrees |
| torpedo_hits_on_japanese_ships | 0 | 0 to 4, 1 (289) | 0 to 4, 1 (303) | agrees |
| allied_torpedoes_fired | 4 to 6 | 16 to 36, 36 (0) | 16 to 48, 34 (0) | miss |
| japanese_gun_hits_per_thousand_shells | 81 to 117 | 48 to 86, 71 (157) | 49 to 92, 75 (331) | at the edge, now agrees |
| allied_gun_hits_per_thousand_shells | 21 to 36 | 23 to 62, 42 (302) | 22 to 65, 43 (271) | agrees |
| japanese_main_battery_shells_fired_in_hundreds | 10 to 11 | 7 to 19, 14 (101) | 6 to 18, 14 (96) | agrees |
| allied_main_battery_shells_fired_in_hundreds | 3 | 3 to 13, 5 (154) | 2 to 13, 5 (187) | at the edge, now agrees |
| southern_group_main_battery_shells_fired_in_hundreds | 0 | 0 to 4, 0 (575) | 0 to 4, 0 (593) | agrees |
| northern_group_main_battery_shells_fired_in_hundreds | 3 | 2 to 9, 4 (217) | 2 to 9, 4 (243) | agrees |
| tenryu_and_yubari_main_battery_shells_fired_in_hundreds | 1 | 2 to 5, 3 (27) | 2 to 5, 3 (37) | miss |
| minutes_of_action | 20 to 30 | 35 to 65, 40 (17) | 35 to 65, 40 (25) | miss |
| surface_fights_by_day | 0 | 0 to 4, 0 (788) | 0 to 4, 0 (776) | agrees |
| japanese_ships_claimed_sunk_per_hundred_sunk | 350 | 100 to 300, 100 (1) | 100 to 300, 100 (2) | reported |
| allied_ships_claimed_sunk | 0 | 0 to 1, 0 (814) | 0 to 2, 0 (814) | agrees |

The head of the report: judged 23; agree 17, then 19; at the edge 2, then 1; miss 4, then 3.

## Hits by ship

Gun hits, the median run, against the record.

| Ship | Record | Before | After |
| --- | --- | --- | --- |
| Canberra | 24 | 11 | 14 |
| Chicago | 1 | 8 | 9 |
| Vincennes | 57 to 74 | 31 | 33 |
| Quincy | 36 to 54 | 27 | 28 |
| Astoria | 34 to 65 | 21 | 23 |
| Patterson, Bagley, Helm, Wilson | 1 in all | 0 each | 0 each |

## What it was aimed at, and what else moved

- **Aimed at:** the Japanese hit rate, from a median of 71 to 75, with 331 runs holding its record from 157. It now agrees plainly: the record's bottom is inside the middle half of the runs.
- **With it:** gun hits on Allied ships, 108 to 116, a miss to an agreement at the edge; Canberra's hits, as above.
- **Not aimed at and moved:** Allied destroyers sunk, none in 862 runs from 924 (with the cruisers sinking sooner the Japanese cruisers' guns come to the destroyers sooner); southern group cruisers sunk, one held in 399 runs from 467 (both are sunk more often); torpedo hits on Allied ships, toward the record. All still agree.
- **Not moved:** gun hits on Japanese ships, the Allied hit rate (42 to 43), shells, who sights first.

## The named misses

- **`minutes_of_action`: not moved** (35 to 65, median 40, against 20 to 30). The Japanese ended each fight by passing on, and no rule has it.
- **Tenryu and Yubari:** not moved. 2 to 5 hundreds, median 3, 37 runs holding 1. The pair fire a mean of 6.5 and 6.6 rounds each, against about 3.5 and 2.8 in the record at the derived rate. The minutes of action do not have their record in the middle half, so the cause is still the length of the fight and the watch stands.
- **`allied_torpedoes_fired`:** not aimed at (16 to 48, median 34, against 4 to 6).

## The other scenarios

The figure is read wherever a force is surprised in a night fight, so more runs changed than in round 1. Run baselines remade: scenario 4; the placeholder night scenarios 4a, 4c (head on, in and out, three forces), 6c2, 14a, 15a and 15b; and the full slice (7) and the soak (8), whose night fights are between placeholder forces. **The Eastern Solomons (5) and Rabaul against Guadalcanal (6) did not change by a byte,** so their reports are not remade. The three recorded games have one data-file hash remade each; their transcripts are byte for byte as before. The pinned outcomes of scenarios 14a, 15a and 15b on the live figures are remade and say so. The frozen step 1 figures keep 1.5.

## Not moved in this round

The break-off levels of scenario 4 and Yunagi's task force; `experience_gun_strength` (0.5, round 1), the torpedo and sighting strengths (0.25); `lit_target_factor` (2); `gun_hit_chance_night` (0.03); the shares of a force not ready (0.5).

# The chance to hit: closed

With rounds 1 and 2 the second mechanism is done, with two moves. The Japanese hit rate agrees with its record and so does the Allied rate; the gap between the sides went from about 1.45 to 1 to about 1.75 to 1, against about 3 to 1 in the record, and the figures that could open it further without a source are left alone. `lit_target_factor` and `gun_hit_chance_night` are not moved: both sides share them in equal measure, and tried on scratch copies they took the Allied rate, already at the top of its record, to 49 to 58 and gun hits on Japanese ships to 30 to 35 against 10 to 17.

What stays short, and why it is not the chance to hit:

- **`gun_hits_on_allied_ships`** (116 against 153 to 219, agreeing at the edge). The record is hits of every calibre from 1,867 shells. The rules fire main batteries only, about 1,400 shells a run; at the record's own rate of 81 to 117 for each thousand that is 113 to 164 hits. So a median of 116 is the measure met as far as the rules can meet it. The rest is the secondary guns, which the rules do not have, as the measure's own reason says.
- **Vincennes, Quincy and Astoria** (33, 28 and 23 hits against 57 to 74, 36 to 54 and 34 to 65). The same, and with it the length of the fight and what a hit does: they are sunk, or the fight is over, before they have been hit as often as the record has them hit.
- **Chicago** (9 against 1). She left the fight; no target rule or chance gives that (decision 0078).

Those go to the third and fourth mechanisms, what a hit does and when a force breaks off, or are written up as gaps.
