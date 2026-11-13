# Savo Island: the surface tuning closed, with its misses named

Nov 13, 2026. Savo Island's surface tuning ends after round 2 and decision 0083. The third and fourth mechanisms in the order of tuning, what a hit does and when a force breaks off (`docs/step2-spec.md`, "Tuning", 2), were tested for a supported move, and none was found. The rule work this brought up is built: tubes that bear on one side, with the other side fired later (decisions 0074 and 0083). No further figure moves at Savo Island. The surface figures go to Cape Esperance to be checked, once, on the coordinating architect's word.

The report this page reads is `docs/validation/savo-island-after-0083.md`: the figures of round 2, on the engine with decision 0083, and the measures with decision 0084's.

## What was done, in order

| Step | What | Recorded in |
| --- | --- | --- |
| Rule | A force not at battle readiness fires its guns at a share, 0.5: Condition II, half the armament manned | Decision 0068 |
| Derivation | Rates of fire as achieved, from rows | Decision 0070 |
| Rule | A force not ready fires its torpedoes at a share, 0.5 | Decision 0071 |
| Derivation | Only a surface-search set with a plan display gives the radar bonus | Decision 0077 (round 0) |
| Rule | Guns go at ships of their own kind; torpedoes at the most important kind | Decision 0078 |
| Orders | The northern group's break-off level is 1, from rows. The southern group and the Japanese force keep 0.5, marked invented. Yunagi is in a task force of her own | Decision 0024, notes of Nov 13, 2026 |
| Round 0 | Finding and surprise: nothing moved | `r0-finding-and-surprise.md` |
| Round 1 | `experience_gun_strength` 0.25 to 0.5, the cap. Kept | `r1-experience-gun-strength.md` |
| Round 2 | `surprised_target_factor` 1.5 to 2. Kept | `r2-surprised-target-factor.md` |
| Rule and derivation | A ship fires the tubes that bear on one side, then the other side in her next torpedo round | Decisions 0074 and 0083 |
| Measures | Six measures by ship and group; the southern group's shells counted in tens; `minutes_of_action` takes the sum of the two fights, 32 to 36 | Decisions 0084 and 0083 (the measures file's head) |

Both figures are fitted at Savo Island, inside a direction the sources support, with no size from a source (decision 0084, findings 2 and 6). Both are placeholders until Cape Esperance is run.

## Where it ends

Judged 29: 21 agree, 4 at the edge, 4 miss. Each figure in the tables below is the 5th to the 95th percentile, the median, and (in brackets) the runs holding the record.

| Measure | Record | Runs | Cause | Recorded in |
| --- | --- | --- | --- | --- |
| `allied_torpedoes_fired`, miss | 4 to 6 | 16 to 44, 26 (0) | Helm and Wilson fire 16 between them in nearly every run. On the rows neither fired: Wilson's order to attack was not received and her captain held fire without his group commander's instructions (`engagement_outcomes` 3217); Helm's target proved friendly and she had no other orders (3218, 3219). That is how many ships of a force fire at all, which the torpedo share does not carry. Every other Allied ship fires as the rows have her: Bagley four of the eight that bear on her side | Decision 0071, "What the share carries"; this page, "The torpedo share" |
| `tenryu_and_yubari_main_battery_shells_fired_in_hundreds`, miss | 1 | 2 to 4, 3 (35) | The pair fire in a mean of 6.6 rounds each against about 3. See "The watch on Tenryu and Yubari" | Decision 0084, finding 11; this page |
| `gun_hits_on_vincennes`, miss | 57 to 74 | 4 to 44, 27 (1) | The record counts every calibre, and the rules fire main batteries only. In the record the column was on the northern group before it had done with the southern; the rules fight the two groups one after the other | Decisions 0084 (3) and 0083 ("Fights that overlap") |
| `southern_group_main_battery_shells_fired_in_tens`, miss | 3 to 5 | 0 to 47, 9 (25) | The group fires nothing, or several times the record | Decision 0084, finding 9 |
| `gun_hits_on_allied_ships`, at the edge | 153 to 219 | 43 to 160, 109 (86) | Every calibre in the record against main batteries in the rules | The measure's reason; decision 0084 (3) |
| `gun_hits_on_quincy`, at the edge | 36 to 54 | 1 to 41, 24 (156) | As for Vincennes | Decision 0084 (3) |
| `torpedo_hits_on_the_northern_group`, at the edge | 3 to 6 | 0 to 3, 0 (93) | Decision 0083 sends the second sides to whatever a ship fights next. The southern fight goes to a second round in 682 runs, so the second sides mostly go there. In the record the fights overlapped | Decision 0083, "As built" and "Fights that overlap" |
| `japanese_main_battery_shells_fired_in_hundreds`, at the edge | 10 to 11 | 7 to 19, 14 (101) | The Japanese fire through every round of both fights | Decision 0083, "Fights that overlap" |

`chicago_sunk` agrees, at 0 to 1, median 1, held by 435. It agrees only because the record is a yes-or-no figure that a run can hold: Chicago is sunk in 565 runs of 1,000. That is a named difference resting on the fights that overlap (decision 0084, "The limits of the checks"; decision 0083). `minutes_of_action` agrees: 30 to 65, median 40, held by 224, and the record of 32 to 36 lies in the middle half.

## What a hit does, and breaking off: no supported move

Each figure was moved alone on a scratch copy of `data/reference`, over 1,000 seeds, on round 2's figures. Nothing in the repository moved.

**Who reads them.** Three figures are read by bombs too (decision 0033): `gun_hit_flotation`, `system_hit_chance_per_point` and `degraded_factor`. `torpedo_system_hit` and `claimed_sunk_fire` are read by air attacks. The three constants of the derivation are read by every derived list; bomb damage and air torpedo damage use them too (decision 0029). So each of these moves the Eastern Solomons as well as both night battles. `fire_control_out_factor`, `gun_system_hit`, `max_rounds` and `crippled_flotation` are read by surface fights only.

**What the sources give.** None of them gives a size.

- Decision 0020 set the three constants "so that real ships land near the placeholder cruiser".
- The record gives instances only: Canberra, Chicago, Vincennes, Quincy, Astoria.
- The damage-control note reads War Damage Report 29 as saying that Astoria's "structural damage alone was survivable; uncontrolled fires lost her". That gives a direction and no size, and no fact row carries it.

| Measure | Record | Round 2 | Flotation share 0.5 | Toughness at 35 tons a point | Torpedo constant 0.25 | Six rounds to four |
| --- | --- | --- | --- | --- | --- | --- |
| Allied cruisers sunk | 4 | 0 to 5, 3 (221) | 0 to 4, 2 (90) | 0 to 4, 2 (94) | 0 to 5, 3 (238) | 0 to 4, 2 (82) |
| Northern group cruisers sunk | 3 | 0 to 3, 2 (368) | 0 to 2, 0 (33), miss | 0 to 3, 1 (51) | 0 to 3, 1 (343) | 0 to 2, 0 (13), miss |
| Gun hits on Allied ships | 153 to 219 | 35 to 158, 116 (88) | 41 to 169, 121 (184) | 49 to 170, 121 (183) | 41 to 160, 117 (95) | 28 to 123, 81 (0), miss |

The direction the one note points takes the northern group's losses away from the record. So does the cap on rounds. Nothing sinks a ship after a fight in the rules, and the record's three sank 21 minutes to ten hours after the Japanese ceased fire (`engagement_outcomes` 66, 77, 88).

**How many hits sink a cruiser.** An eight-inch hit does 5.27 points to a New Orleans class cruiser, so 19 hits on flotation sink her, about 25 hits in all.

In the runs, a northern cruiser that sinks has taken a median of 32 to 42 gun hits as rolled. That is about the record's hits from the main batteries, if the main batteries scored about three fifths of them. That share is an assumption no row supplies (decision 0084, 2).

The cruisers are not sunk early by gunfire. What differed is where the torpedoes went, which decisions 0074 and 0083 took up.

**Passing on.** No rule is built (the architect's ruling of Nov 13, 2026). The rows give the southern fight about two rounds: first fire about 01:43 (`engagement_events` 59), the group dissolved by 01:50 (81), "no more than 10 minutes" (`engagement_outcomes` 17). That is as long as the model's. A rule for passing on would cost the northern sinkings and would not mend Chicago. One fight of three forces, in which fights overlap, is recorded as a plan for the designer and not built (decision 0083, "Fights that overlap").

## The two tuned figures, looked at once more

These runs are on the engine with decision 0083, over 1,000 seeds, on scratch copies.

| Figure | Value | Judged: agree, edge, miss (of 23, the measures before 0084) | Japanese hits for each thousand shells, record 81 to 117 |
| --- | --- | --- | --- |
| As now | `experience_gun_strength` 0.5, `surprised_target_factor` 2 | 19, 2, 2 | 51 to 91, 74 (274) |
| `experience_gun_strength` | 0.35 | 17, 3, 3 (gun hits on Allied ships miss) | 45 to 82, 66 (70), at the edge |
| `surprised_target_factor` | 1.5 | 15, 6, 2 | 49 to 86, 70 (146), at the edge |
| `surprised_target_factor` | 2.5 | 20, 1, 2 | 51 to 98, 77 (401) |

Both figures stand.

- **Experience** is at its cap, and a step down is worse.
- **The surprised factor:** in the 577 runs in which the southern group is surprised, Canberra takes a median of 15 gun hits at 2 and 19 at 2.5. Going to 2.5 would ask the factor to carry the secondary guns. The report would gain one agreement, and no source sizes the factor. Not moved.

## The torpedo share

`unready_torpedo_share` stays at 0.5. With decision 0074 it now has a row: Bagley fired four of the eight tubes that bear on her port side, her starboard primers not yet in (`engagement_outcomes` 46, 47, 3206, 3228).

| Share | Allied torpedoes fired, record 4 to 6 |
| --- | --- |
| 0.5 | 16 to 44, 26 (0), miss |
| 0.25 | 8 to 18, 13 (2), miss |
| 1 | 32 to 72, 52 (0), miss |

No value reaches the record, because Helm and Wilson fire in 999 runs of 1,000 where the rows have neither firing at all. That is not the share's to carry (decision 0071).

## The watch on Tenryu and Yubari

The watch reads: if the minutes of action come to the middle of the runs and the pair still fire over the record, tuning stops and the matter goes back to the derivation. That condition is now met in form. The minutes' record of 32 to 36 lies in the middle half of the runs (35 to 45), and the pair still fire 3 hundreds against 1.

The minutes came to the middle because the record was corrected to the sum of the two fights. The fights did not get shorter: the median is still 40.

The pair fire in a mean of 6.6 rounds each. The record's shells are about 3.5 and 2.8 rounds at the derived rate. Its times have Tenryu open fire on Quincy at 01:59 (`engagement_events` 1060) and cease at 02:06 (1071), and Yubari cease at 02:08 (1069). That is a short window, about the length the record's shells give at the derived rate. So the rows do not show the rate to be too high. What they show is the pair firing for part of the night, where in the rules they fire through every round of both fights. Decision 0084 records the gate as failed and the rates as not certified by it (finding 11).

Under the watch this goes back to the derivation, as the architect rules. It is not a tuning round.

## What no figure may be asked to carry

From the records' own words:

- **The gun share** is not to cover "rates of fire that are too high for both navies, or ... fights the record does not have". Below 0.5 it carries "captains called from sleep, ships taken for friends, orders to cease fire" ("Tuning", 7; decision 0068, 8).
- **The torpedo share** does not carry "how many tubes bear on one broadside", "how many ships of a force fire at all, and in which round", "the range" or "the chance to hit" (decision 0071).
- **The surprised factor** "carries one round of surprise under fire ... It does not carry rates of fire (decision 0070) or readiness (decision 0068)" (decision 0076, 4). The record's longer surprise "is out of the rules' reach, and no figure is to be asked to supply it" (the two measures of rounds surprised).
- **Experience:** "At the cap, any gap left between the sides is not experience's to carry" (r1). No ship's value is changed (`docs/step2-spec.md`, "Experience", 2).
- **Tenryu and Yubari:** "no figure is moved to mend this measure itself" (`baseline-reading.md`).
- **What decision 0078's rule of targets cannot give,** which tuning is not to chase: Chicago's one hit, Tenryu and Yubari's targets, and the even sharing of fire.

## What Cape Esperance tests

| Figure | Value | Standing |
| --- | --- | --- |
| `experience_gun_strength` | 0.5, the cap | Fitted on Savo Island, not yet checked |
| `surprised_target_factor` | 2 | Fitted on Savo Island, not yet checked; there the Japanese were the side surprised |
| `unready_gun_share`, `unready_torpedo_share` | 0.5 each | Not tuned; there they fall on the other navy |
| Radar for sighting | Decision 0077's sets | A rule of the derivation; there the other side had the radar |
| Torpedo sides | Decisions 0074 and 0083 | Built; the Japanese cruisers there fight one force |

What the check can and cannot show is in decision 0084, finding 1. The check is one run, on the figures and rules as they stand when the scenario moves out of `not-run/`.

## Watches that still bind

- **Tenryu and Yubari:** as above, with the architect.
- **Fights by day:** if `surface_fights_by_day` misses once the night's hits on Japanese ships are near the record, scuttling a ship left dead in the water is next (decision 0071, 5). It agrees: 803 runs hold none.
- **No order moves.** The break-off levels and Yunagi's task force are not moved again (decision 0024, the notes).
- **The step 1 figures stay frozen** for the hand calculations ("Tests while figures move", 1).

The scratch batches are not kept in the repository. Each was made with `sim-cli batch data/scenarios/04_savo_island.toml --runs 1000 --reference DIR`, where DIR is a copy of `data/reference` with the one figure changed.
