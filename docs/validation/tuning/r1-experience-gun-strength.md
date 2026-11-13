# Round 1: the strength of experience for guns, 0.25 to 0.5

Savo Island, Nov 13, 2026. The second mechanism in the order of tuning, the chance of a hit (`docs/step2-spec.md`, "Tuning", 2). Ruled by the coordinating architect before the move. Whether it is kept or reverted is the coordinating architect's word, on this record.

## What moved

| File | Figure | From | To |
| --- | --- | --- | --- |
| `data/reference/surface_combat.toml` | `experience_gun_strength` | 0.25 | 0.5 |

Nothing else. No experience value of any ship was touched; only the strength of its effect. The frozen step 1 figures keep 0, the value at which experience does nothing.

**Its kind:** a surface figure, tuned on Savo Island and not checked on any battle until Cape Esperance is run.

## Why this figure

On the report tuning started from, the Japanese hit with 58 of each thousand shells against a record of 81 to 117, and the Allied ships with 40 against 21 to 36. The Japanese rate is too low and the Allied rate is already at the top of its record: what is wrong is the gap between the sides, about 1.45 to 1 in the model against about 3 to 1 in the record. A figure both sides share in equal measure cannot mend that. The base chance at 0.04 or the factor for a lit target at 3 or 4, tried on scratch copies, raised the Allied rate to 49 to 58 and gun hits on Japanese ships to 30 to 35 against 10 to 17.

Experience acts on one side more than the other: every Japanese ship has the value 3, and Vincennes and Quincy have 1 and Astoria and Wilson 2. It is the figure the spec set this measure to test.

**The support.** The sources give its sign and no size. The 1950 analysis has "practically none of the ships" of the amphibious force having "engaged in any night battle exercise since the beginning of the war", and the Japanese stressing night training; the research note warns that statements "written after the battle by the side that lost it ... should not feed a figure" (`docs/research/oob/experience.md`). So it stays marked a placeholder however well it fits.

**Why this size.** 0.5 is the cap decision 0064 set. The half step to 0.35 was tried and bought a third as much. At the cap, any gap left between the sides is not experience's to carry.

## Every measure, before and after

1,000 runs each, seeds 1 to 1,000. Each cell is the 5th to the 95th percentile, the median, and in brackets the runs holding the record. Before is `savo-island-baseline.md` as it stood; after is `savo-island-r1.md`.

| Measure | Record | Before | After | Verdict |
| --- | --- | --- | --- | --- |
| allied_cruisers_sunk | 4 | 0 to 4, 2 (113) | 0 to 5, 2 (204) | at the edge, now agrees |
| southern_group_cruisers_sunk | 1 | 0 to 2, 1 (531) | 0 to 2, 1 (467) | agrees |
| northern_group_cruisers_sunk | 3 | 0 to 3, 1 (110) | 0 to 3, 1 (302) | at the edge, now agrees |
| allied_destroyers_sunk | 0 | 0 to 0, 0 (984) | 0 to 1, 0 (924) | agrees |
| japanese_ships_sunk | 0 | 0 to 1, 0 (823) | 0 to 1, 0 (843) | agrees |
| japanese_ships_with_propulsion_out | 0 | 0 to 1, 0 (879) | 0 to 1, 0 (871) | agrees |
| japanese_ships_sunk_or_stopped | 0 | 0 to 2, 0 (722) | 0 to 1, 0 (728) | agrees |
| night_fights_the_japanese_did_not_sight_first | 0 | 0 to 2, 1 (335) | 0 to 2, 1 (335) | agrees |
| southern_group_rounds_surprised | 1 to 2 | 0 to 1, 1 (577) | 0 to 1, 1 (577) | reported |
| northern_group_rounds_surprised | 2 to 3 | 0 to 1, 1 (0) | 0 to 1, 1 (0) | reported |
| gun_hits_on_allied_ships | 153 to 219 | 30 to 129, 87 (8) | 35 to 151, 108 (42) | miss |
| gun_hits_on_japanese_ships | 10 to 17 | 9 to 68, 25 (232) | 9 to 69, 25 (231) | agrees |
| torpedo_hits_on_allied_ships | 4 to 9 | 1 to 6, 3 (403) | 1 to 6, 3 (410) | agrees |
| torpedo_hits_on_japanese_ships | 0 | 0 to 4, 1 (288) | 0 to 4, 1 (289) | agrees |
| allied_torpedoes_fired | 4 to 6 | 16 to 36, 36 (0) | 16 to 36, 36 (0) | miss |
| japanese_gun_hits_per_thousand_shells | 81 to 117 | 38 to 72, 58 (3) | 48 to 86, 71 (157) | a miss, now agrees at the edge |
| allied_gun_hits_per_thousand_shells | 21 to 36 | 21 to 60, 40 (332) | 23 to 62, 42 (302) | agrees |
| japanese_main_battery_shells_fired_in_hundreds | 10 to 11 | 6 to 19, 14 (114) | 7 to 19, 14 (101) | agrees |
| allied_main_battery_shells_fired_in_hundreds | 3 | 3 to 13, 6 (120) | 3 to 13, 5 (154) | agrees, at the edge |
| southern_group_main_battery_shells_fired_in_hundreds | 0 | 0 to 4, 0 (554) | 0 to 4, 0 (575) | agrees |
| northern_group_main_battery_shells_fired_in_hundreds | 3 | 2 to 10, 5 (174) | 2 to 9, 4 (217) | at the edge, now agrees |
| tenryu_and_yubari_main_battery_shells_fired_in_hundreds | 1 | 2 to 5, 3 (27) | 2 to 5, 3 (27) | miss |
| minutes_of_action | 20 to 30 | 35 to 65, 40 (19) | 35 to 65, 40 (17) | miss |
| surface_fights_by_day | 0 | 0 to 4, 0 (777) | 0 to 4, 0 (788) | agrees |
| japanese_ships_claimed_sunk_per_hundred_sunk | 350 | 100 to 300, 100 (1) | 100 to 300, 100 (1) | reported |
| allied_ships_claimed_sunk | 0 | 0 to 2, 0 (790) | 0 to 1, 0 (814) | agrees |

The head of the report: judged 23; agree 14, then 17; at the edge 4, then 2; miss 5, then 4.

## Hits by ship

Gun hits, the median run, against the record.

| Ship | Record | Before | After |
| --- | --- | --- | --- |
| Canberra | 24 | 9 | 11 |
| Chicago | 1 | 6 | 8 |
| Vincennes | 57 to 74 | 28 | 31 |
| Quincy | 36 to 54 | 22 | 27 |
| Astoria | 34 to 65 | 15 | 21 |
| Patterson, Bagley, Helm, Wilson | 1 in all | 0 each | 0 each |

## What it was aimed at, and what else moved

- **Aimed at:** the Japanese hit rate. It went from a median of 58 to 71, and from 3 runs holding its record to 157. It now agrees at the edge: the record is still above the middle half of the runs.
- **With it:** gun hits on Allied ships (87 to 108, still a miss), and the runs with four Allied cruisers sunk (113 to 204) and with three of the northern group's sunk (110 to 302), both of which now agree plainly.
- **Not aimed at and moved:** Allied destroyers sunk, from none in 984 runs to none in 924 (the Allied destroyers' own value is 3 or 2, and the Japanese cruisers that sink them shoot better). The Allied hit rate, 40 to 42. Both still agree.
- **Not moved:** gun hits on Japanese ships, every torpedo measure, the shells both sides fire, who sights first.

## The named misses

- **`minutes_of_action`: not moved** (35 to 65, median 40, against 20 to 30). The cause is named: the Japanese ended each fight by passing on, and no rule has it.
- **Tenryu and Yubari** (the watch of "Tuning", 6): not moved. 2 to 5 hundreds, median 3, 27 runs holding 1. The pair fire a mean of 6.7 and 6.8 rounds each, where the record's shells are about 3.5 and 2.8 rounds at the derived rate. The minutes of action do not have their record in the middle half, so the cause is still the length of the fight and the watch stands.
- **`allied_torpedoes_fired`: not moved.** No torpedo figure was touched.

## The other battles

No run of any other scenario changed but two: scenarios 14a and 16, the two placeholder scenarios whose ships are given unequal experience, whose run baselines are remade with scenario 4's. Every other run baseline is byte for byte as before, the Eastern Solomons, Rabaul, the full slice and the soak among them, so their reports are not remade. That is the proof that the figure is read only where a night surface fight has ships of unequal experience. The three recorded games have one data-file hash remade each; their transcripts are byte for byte as before.

## Not moved in this round

The break-off levels of scenario 4 and Yunagi's task force (decision 0024, the notes of Nov 13, 2026); `surprised_target_factor` (1.5), `lit_target_factor` (2) and `gun_hit_chance_night` (0.03); the torpedo and sighting strengths of experience (0.25); the shares of a force not ready (0.5).

## What is left for the chance to hit

The Japanese rate is at 71 against 81 to 117 with experience at its cap. The next figure that acts on one side more than the other is `surprised_target_factor`, which reaches 17 in 100 of the Japanese gun salvos and carries one round of surprise (decision 0076). Numbers on this report come before any move.
