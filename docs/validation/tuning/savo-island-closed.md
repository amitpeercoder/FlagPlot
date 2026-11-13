# Savo Island: the surface tuning closed after round 2

Nov 13, 2026. Savo Island's surface tuning ends with round 2. The third and fourth mechanisms in the order of tuning, what a hit does and when a force breaks off (`docs/step2-spec.md`, "Tuning", 2), were tested for a supported move and have none. The surface figures go to Cape Esperance to be checked, once, on the coordinating architect's word.

## What was done, in order

Rule and derivation work first, then two figures.

| Step | What | Where it is recorded |
| --- | --- | --- |
| Rule | A force not at battle readiness fires its guns at a share, 0.5: Condition II, half the armament manned | Decision 0068 |
| Derivation | Rates of fire as achieved, from rows, not as designed | Decision 0070 |
| Rule | A force not ready fires its torpedoes at a share, 0.5, a first value with no row of its own | Decision 0071 |
| Derivation | Only a surface-search set with a plan display gives the radar bonus | Decision 0077 (round 0) |
| Rule | Guns go at ships of their own kind, torpedoes at the most important kind | Decision 0078 |
| Orders | The northern group's break-off level set at 1 from rows; the southern group and the Japanese force keep 0.5, marked invented | Decision 0024, note of Nov 13, 2026 |
| Orders | Yunagi in a task force of her own, from rows: she turned away from the cruiser action | Decision 0024, note of Nov 13, 2026 |
| Round 0 | Finding and surprise: nothing moved | `r0-finding-and-surprise.md` |
| Round 1 | `experience_gun_strength` 0.25 to 0.5, the cap of decision 0064. Kept | `r1-experience-gun-strength.md` |
| Round 2 | `surprised_target_factor` 1.5 to 2, its size from Canberra's hits. Kept; "The chance to hit: closed" | `r2-surprised-target-factor.md` |

Both moves act on one side more than the other. The gap between the sides' hit rates went from about 1.45 to 1 to about 1.75 to 1, against about 3 to 1 in the record (r2).

## Where it ends

The report after round 2 (`savo-island-r2.md`): **judged 23; agree 19, at the edge 1, miss 3.**

| Measure | Record | 5th to 95th, median (runs holding) | Cause | Recorded in |
| --- | --- | --- | --- | --- |
| `minutes_of_action`, miss | 20 to 30 | 35 to 65, 40 (25) | The Japanese ended each fight by passing on, and no rule has a force retire when it has done what it came for | Decision 0024, note on break-off levels; r1, r2 |
| `tenryu_and_yubari_main_battery_shells_fired_in_hundreds`, miss | 1 | 2 to 5, 3 (37) | The pair fire about 6.5 rounds each against about 3; the derived rate stands on rows. Read as the length of the fight, under the watch below | `baseline-reading.md`; r1, r2 |
| `allied_torpedoes_fired`, miss | 4 to 6 | 16 to 48, 34 (0) | Decision 0071's share at its first value, and tubes that bear on one side (decision 0074, held). See "The torpedo share" below for which ships fire | Decisions 0071, 0074, 0078; r2 |
| `gun_hits_on_allied_ships`, at the edge | 153 to 219 | 35 to 158, 116 (88) | The record counts every calibre; the rules fire main batteries only, about 1,400 shells a run, and at the record's own rate that is 113 to 164 hits. The rest is the secondary guns | The measure's reason; r2, "The chance to hit: closed" |

By ship, the median run: Canberra 14 gun hits against 24, Chicago 9 against 1, Vincennes 33 against 57 to 74, Quincy 28 against 36 to 54, Astoria 23 against 34 to 65 (r2). Chicago's are the target rule's limit: she left the fight, and no target rule or chance gives that (decision 0078).

## What a hit does, and breaking off: no supported move

Each figure was moved alone on a scratch copy of `data/reference`, 1,000 seeds, against the round 2 batch. Nothing in the repository moved.

**Who reads them.** `gun_hit_flotation`, `system_hit_chance_per_point` and `degraded_factor` are read when a bomb hits too (decision 0033), and `torpedo_system_hit` and `claimed_sunk_fire` by air attacks; a move of any reruns all three battles ("Tuning", 9). The three constants of the derivation (`data/derivation/rules.toml`) are read by every derived list: a bomb's damage is its weight over the gun constant, an aerial torpedo's is its warhead times the torpedo constant (decision 0029), and toughness divides every hit, so each moves the Eastern Solomons as well as both night battles. `fire_control_out_factor`, `gun_system_hit`, `max_rounds` and `crippled_flotation` are read by surface fights only.

**What the sources give.** No row and no note gives a size for any of them. Decision 0020 set the three constants "so that real ships land near the placeholder cruiser". The record gives instances: Canberra 24 shells and none to two torpedoes, scuttled; Chicago one shell and one torpedo, afloat; Vincennes 57 to 74 shells and one to three torpedoes; Quincy 36 to 54 and two or three; Astoria 34 to 65 and none, capsized at noon. The damage-control note reads War Damage Report 29 as saying Astoria's "structural damage alone was survivable; uncontrolled fires lost her", which gives a direction (fewer gun hits on flotation, or tougher ships) and no size, and has no fact row.

**What the direction does.** Each cell: 5th to 95th, median (runs holding the record).

| Measure | Record | Round 2 | Flotation share 0.5 | Toughness at 35 tons a point | Torpedo constant 0.25 | Six rounds to four |
| --- | --- | --- | --- | --- | --- | --- |
| Allied cruisers sunk | 4 | 0 to 5, 3 (221) | 0 to 4, 2 (90) | 0 to 4, 2 (94) | 0 to 5, 3 (238) | 0 to 4, 2 (82) |
| Northern group cruisers sunk | 3 | 0 to 3, 2 (368) | 0 to 2, 0 (33), miss | 0 to 3, 1 (51) | 0 to 3, 1 (343) | 0 to 2, 0 (13), miss |
| Gun hits on Allied ships | 153 to 219 | 35 to 158, 116 (88) | 41 to 169, 121 (184) | 49 to 170, 121 (183) | 41 to 160, 117 (95) | 28 to 123, 81 (0), miss |
| Minutes of action | 20 to 30 | 35 to 65, 40 (25) | 35 to 70, 40 (20) | 35 to 70, 40 (11) | 35 to 70, 40 (18) | 25 to 55, 30 (642) |
| Tenryu and Yubari's shells | 1 | 2 to 5, 3 (37) | 2 to 5, 3 (27) | 2 to 5, 3 (19) | 2 to 5, 3 (28) | 1 to 3, 2 (96) |
| The report's head: agree, edge, miss | | 19, 1, 3 | 13, 6, 4 | 13, 7, 3 | 17, 3, 3 | 18, 2, 3 |

Allied cruisers sunk in all, over 1,000 runs: 2,869 at round 2; 1,865, 1,790, 2,582 and 1,915 in the four columns. Allied destroyers sunk: 178; 5, 5, 158 and 13. Japanese ships sunk: 187; 139, 112, 128 and 171. Also tried, with smaller effects and none toward the record that a source asks for: the system chance at 0.03 a point, the degraded factor at 0.75, a ship crippled at 30 points of flotation.

So the direction the one note points takes the northern group's losses away from the record, and the cap on rounds that brings the minutes right does the same: the northern cruisers are not sunk in four rounds. Neither cap nor share is the record's cause, which is the Japanese passing on. The break-off levels are orders, set from rows before round 1, and are not moved ("Tuning", 8).

**How many hits sink a cruiser.** An 8-inch hit on a New Orleans class cruiser does 5.27 points, so 19 hits on flotation sink her: about 25 hits, three in four landing on flotation. In the runs, a northern cruiser that sinks has taken, at the median, 36 gun hits (Vincennes), 32 (Quincy) and 42 (Astoria), counted as rolled, so with the hits of the sinking round and the light cruisers' 1.6-point hits in; none of them a torpedo. Read with the round 2 record's assumption that the main batteries scored about three fifths of the hits, the record's ships took about 34 to 44 (Vincennes), 22 to 32 (Quincy) and 20 to 39 (Astoria) from the main batteries, and Vincennes and Quincy one to three torpedoes besides. **The cruisers are not sunk early by gunfire; they sink at about the record's main battery hits.** What differs is where the torpedoes go.

**Where the torpedoes go.** Each Japanese ship fires every tube once, in its first fight, which is the southern group's. In 1,000 runs Vincennes, Quincy and Astoria take no torpedo hit; Canberra and Chicago take a median of 2 each. The record has one to three on the southern group and three to six on the northern (the measure's rows). The total agrees (4 to 9, median 4, 524 runs), so no measure shows the split. It costs Chicago: she is sunk in 560 runs, 491 of them with a torpedo hit or more, and both southern cruisers in 471, against one in the record. A torpedo of 490 kg does 54 points to her, so two sink her; she survived one. Decision 0074 names the cause, "Firing the other side's tubes later in the fight", and leaves it out. Its derivation alone, tried on a scratch class file, takes the southern group's torpedo hits down (both sunk in 290 runs, Chicago in 411) and still puts none into the northern fight, and turns torpedo hits on Allied ships to the edge (0 to 5, 2, 230 runs). Only its second-side rule, which is the designer's, would move torpedoes to the northern group, and it was not tried.

## The torpedo share

Of the five Allied ships with tubes, one fired: Bagley, four from her port mount after her starboard primers were not in (`engagement_outcomes` 46, 47, 3206, 3228). Patterson's order to fire was not heard (57, 3175, 3226). Canberra's triggers did not respond, or she fired two (180, 30). Wilson's order to attack was not received; her captain had his battery ready and judged it unwise to attack in so confused an action without the group commander's instructions (3217). Helm's target turned out to be friendly, and with no other orders she stayed by her group (3218, 3219). In the runs Helm and Wilson fire in 997 of 1,000; at the share of 0.5 they alone fire 16. Their not firing is how many ships fire and who is taken for a friend, which decision 0071 says the share does not carry.

| Torpedo share (with 0074's tubes where marked) | Allied torpedoes fired: 5th to 95th, median (runs holding) |
| --- | --- |
| 0.5, round 2 | 16 to 48, 34 (0), miss |
| 0.25 | 8 to 18, 16 (0), miss |
| 0.15 | 4 to 10, 9 (453) |
| 0.125 | 4 to 9, 8 (457) |
| 0.1, the least the loader takes | 3 to 7, 6 (78) |
| 0.5, with 0074's tubes | 8 to 18, 18 (0), miss |
| 0.25, with 0074's tubes | 4 to 9, 9 (359) |

The share alone would have to go to 0.15 or below. The one stated count, Bagley's four of sixteen, is 0.25 of her tubes and half of those that bear on one side. **The share is left at 0.5, and the measure stays a miss with its cause named;** whether 0074 with its second side is built is a plan for the designer, on the evidence of where the Japanese torpedoes land rather than of this measure.

## What no figure may be asked to carry

From the records' own words:

- The gun share: not "rates of fire that are too high for both navies, or ... fights the record does not have"; below 0.5 it carries "captains called from sleep, ships taken for friends, orders to cease fire" ("Tuning", 7; decision 0068, 8).
- The torpedo share: not "how many tubes bear on one broadside", "how many ships of a force fire at all, and in which round", "the range", "the chance to hit" (decision 0071).
- The surprised factor "carries one round of surprise under fire ... It does not carry rates of fire (decision 0070) or readiness (decision 0068)" (decision 0076, 4). The record's longer surprise "is out of the rules' reach, and no figure is to be asked to supply it" (the two measures of rounds surprised).
- Experience: "At the cap, any gap left between the sides is not experience's to carry" (r1). No ship's value is changed (`docs/step2-spec.md`, "Experience", 2).
- Tenryu and Yubari: "no figure is moved to mend this measure itself" (`baseline-reading.md`).
- Torpedo figures are "not tuned to cover" the tubes that bear ("Rule work that comes before tuning", the 0074 row).
- Chicago's one hit, Tenryu and Yubari's targets, the even sharing of fire: "What it cannot give, and tuning must not chase" (decision 0078).

## What Cape Esperance tests

| Figure | Value | Standing |
| --- | --- | --- |
| `experience_gun_strength` | 0.5, at the cap of decision 0064 | Fitted on Savo Island, not yet checked. The sources give its sign and no size |
| `surprised_target_factor` | 2 | Fitted on Savo Island, not yet checked. Its size rests on one ship, Canberra; at Cape Esperance the Japanese were the surprised side |
| `unready_gun_share`, `unready_torpedo_share` | 0.5 each | Not tuned. Set from Condition II (the guns) and mirrored (the torpedoes); at Cape Esperance they fall on the other navy |
| Radar for sighting | Decision 0077's sets | A derivation rule, not tuned; at Cape Esperance the other side had the radar |

Every other surface figure is where step 1 left it, a placeholder with no source for its size.

## Watches that still bind

- **Tenryu and Yubari.** If the minutes of action come to the middle of the runs and the pair still fire over the record, tuning stops and it goes back to the derivation. On the scratch run with four rounds the minutes came to the middle (642 runs hold) and the pair fired 1 to 3, median 2, against 1 (96 runs hold). That is a probe, not a move, and the watch stands.
- **Fights by day.** If `surface_fights_by_day` misses once the night's hits on Japanese ships are near the record, scuttling a ship left dead in the water is next (decision 0071, 5). It agrees: 776 runs hold none.
- **Decision 0074** is built only with a rule for the other side's tubes, the designer's plan, and only if the torpedo measures call for it.
- **No order moves.** The break-off levels and Yunagi's task force are not moved again (decision 0024, the notes).
- **The step 1 figures stay frozen** for the hand calculations ("Tests while figures move", 1).

The scratch batches are not kept in the repository. Each was made with `sim-cli batch data/scenarios/04_savo_island.toml --runs 1000 --reference DIR` on a copy of `data/reference` with the one figure changed, or for the constants and 0074's tubes, the class file's columns changed by the same factor or by 0074's table.
