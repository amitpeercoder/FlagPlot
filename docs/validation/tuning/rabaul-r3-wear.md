# Rabaul round 3: wear

**These figures have no check battle.** They are tuned on Rabaul against Guadalcanal and no other battle can read them: the Eastern Solomons does not load the air upkeep figures. An agreement here is a fit, not a check (`docs/step2-spec.md`, "Tuning": the figures that take weeks to show).

Rabaul against Guadalcanal, Nov 13, 2026. This is the third of the slow figures, in the order of the plan the architect approved. **Written before any run of the round;** the sections from "What was run" on are added after it. The proposed move goes to the coordinating architect with its evidence before it becomes live.

## The figures

| File | Figure | Now | Read by |
| --- | --- | --- | --- |
| `data/reference/air_upkeep.toml` | `wear_base_chance` | 0.05 | Every sortie: the chance an aircraft that came back is worn, before distance |
| | `wear_chance_per_100_nm` | 0.02 | Every sortie, times its distance over 100 |
| | `write_off_share` | 0.2 | Every worn aircraft: the share lost for good; the rest are damaged |
| | `carrier_repairs_per_day` | 2 | Carriers only. Scenario 6 has none, so no tuned battle reads it |
| `data/scenarios/06_rabaul_guadalcanal.toml` | `aircraft_repairs_per_day` | Henderson 3, Rabaul 4 | Repair at the night pulse (chapter 5, "Repair"). Scenario figures, invented (decision 0042) |

All are placeholders (decision 0035). The rule is chapter 5, "Wear": each aircraft that came back is worn at `wear_base_chance` plus `wear_chance_per_100_nm` times the sortie's distance over 100; each worn aircraft is written off at `write_off_share`, and the rest are damaged until repaired.

**The distance of each kind of sortie in scenario 6** (chapter 5, "What counts as a sortie"):

| Sortie | Distance | Chance worn | Written off a sortie |
| --- | --- | --- | --- |
| Henderson's fighter patrol, every daylight pulse | 0 nm | 0.05 | 0.010 |
| The dive bombers' search, 200 miles out (decision 0082) | 400 nm | 0.13 | 0.026 |
| Rabaul's bombers and escorts on a raid | 1,112 nm | 0.2724 | 0.054 |

So the three wear figures are read by different sorties in different measure. The base chance is nearly all that the patrols read. The distance term is what the searches and the raids read. The write-off share is read by all of them alike.

## What the rules give now

1,000 runs on the live figures, seeds 1 to 1,000, counted from the `AircraftWorn`, `AircraftRepaired` and ground loss events of `run --events`. Means a run.

| Groups | Sortie | Aircraft that flew | Worn | Written off | Written off a sortie |
| --- | --- | --- | --- | --- | --- |
| SBDs: 11, 12, 14 by day 21 | Search, 400 nm | 167 | 21.5 | 4.3 | 2.6% |
| SBDs: 11, 12, 14, 17, whole run | Search, 400 nm | 224 | 28.8 | 5.7 | 2.5% |
| F4Fs: 10, 13, 15, 16, whole run | Patrol, 0 nm | 813 | 40.8 | 8.2 | 1.0% |
| G4Ms: 1, whole run | Raid, 1,112 nm | 193 | 52.4 | 10.5 | 5.4% |
| Zeros: 2, 3, whole run | Escort, 1,112 nm | 156 | 42.4 | 8.6 | 5.5% |

**The dive bombers on day 21.** Groups 11, 12 and 14 have had 35 aircraft delivered (12, 11 and 12). By the end of day 21 they have lost 4.3 written off and 0.3 on the ground, and hold 0.3 damaged. That leaves 30 ready, the median of `sbds_ready_on_day_21`, against a record of 22.

Henderson repairs 3 aircraft a night. Its groups wear about 2 a day before day 22, so the damaged are nearly all back the next morning: groups 10 and 11 hold 0.7 and 0.6 damaged at the end of day 5, and the SBDs 0.3 together at the end of day 21.

## The rows that give a direction and a size

All rows are `data/oob/engagement_outcomes.csv`, the Naval Aviation Combat Statistics (NACS, 1946), Table 17, for Navy and Marine aircraft ashore in the Solomons. NACS gives a month's losses on action sorties by cause, and losses on other flights (patrol, ferry, training, take-off and landing) with no cause. Each was read for this round. The window is 21 August to 17 September. August is the Marines' 21 to 31 August, and September is pro rata, 17 of 30 days. **The pro rata share is an assumption:** NACS gives no date within a month.

**Dive and torpedo bombers.** The SBDs are most of these, but not all of them.

| Month | Operational, action sorties | Other flights | Action sorties | Rows |
| --- | --- | --- | --- | --- |
| August, Marine | 0 | 1 | 20 | 1276, 1277 |
| August, Navy | 0 | 0 | 11 | 1292, 1293 |
| September, Marine | 4 | 4 | 125 | 1281, 1282 |
| September, Navy | 0 | 4 | 122 | 1297, 1298 |

For the window that is 1 + 12 × 17/30 = **about 7.8 written off.** The brief listed 1295 for the Navy: that row is 1 lost to anti-aircraft fire, a combat loss, which wear does not carry. The Navy's operational row for September is 1297, 0.

**Fighters.**

| Month | Operational, action sorties | Other flights | Action sorties | Rows |
| --- | --- | --- | --- | --- |
| August, Marine | 0 | 1 | 57 | 1264, 1265 |
| September, Marine | 0 | 6 | 177 | 1270, 1271 |
| September, Navy | 1 | 2 | 82 | 1286, 1287 |

For the window that is 1 + 9 × 17/30 = **about 6.1 written off.** The Navy fighters came ashore on 11 September, so their share of the window may be smaller.

**Rabaul's bombers forced down and wrecked.** One each on 26 August for two units (988, 1049), on 29 August (1061), 10 September (1090) and 13 September (1110). A sixth on 12 September rests on a low row (1102). **5 to 6 over about 212 bomber sorties.** This is the measure `japanese_bombers_forced_down`, reported and not judged, because the forms count a forced landing and not an aircraft that came home and never flew again. It is a floor for what the rules write off.

**What the rows say, set beside the rules.**

| Who | Rows, the window | Rules now | Rules against the rows |
| --- | --- | --- | --- |
| US dive and torpedo bombers | about 7.8 | 5.7 | About three quarters |
| US fighters | about 6.1 | 8.2 | About a third more |
| Japanese bombers | at least 5 to 6 | 10.5 | About twice the floor |

The rules are already in the record's size for both American types. The SBDs come out under it and the fighters over it.

**Which single figure carries the SBDs' share.**
- `wear_base_chance` is nearly all that the fighters' patrols read. Raising it to lift the SBDs' write-offs would raise the fighters' too, already above their rows.
- `write_off_share` is read alike by every sortie. Raising it lifts the fighters' write-offs by the same share, away from their rows.
- **`wear_chance_per_100_nm`** is read by the searches and the raids, and not by the patrols. It is the one figure that can bring the SBDs' write-offs to their rows without moving the fighters'.

At about 224 search sorties a run, the rows' 7.8 is about 3.5% a sortie. At a write-off share of 0.2, that is a wear chance of about 0.174 on a 400 nm search, which is `wear_chance_per_100_nm` = (0.174 − 0.05) / 4 = **about 0.03.** This is the size the rows point to.

## What it cannot carry

The rows' 7.8 written off is less than a quarter of the gap on day 21. At 0.03 the SBDs lose about 1.5 more by day 21, so the expected median is about 28 to 29 against a record of 22. The rest of the gap is two things the wear figures do not hold:

1. **Aircraft out of service, not lost.** On 25 August the commanding general had 5 of 31 under repair: 3 of 19 fighters and 2 of 12 dive bombers (455). The rules hold about 1.3 damaged in those groups that evening. On 10 September 22 SBDs were flyable (1343), and not every one of the other 13 can have been lost. So the record held a pool of aircraft under repair for days, which the rules do not. Henderson's repairs are a scenario figure, 3 a night, that no row sizes. Major repair went to Australia (`docs/research/oob/outside-research-nov-13.md`, "Air upkeep"), which is qualitative.
2. **Combat losses on strikes at ships.** The dive bombers lost 1 to anti-aircraft fire in August (1274), 2 to fighters (1280) and 1 to anti-aircraft fire (1295) in September, and 1 on the ground on 2 September (1330). Scenario 6 holds no ships, so its SBDs fly no strikes and lose none of these (decision 0082).

## What is read beside it

- **Judged, at Rabaul:** `sbds_ready_on_day_21` (22, a miss now), `f4fs_ready_on_day_21` (11), `vmf_223_ready_on_day_5` (11), `vmsb_232_ready_on_day_5` (9), `vmf_223_ready_on_day_10` (4 to 5), `us_fighters_lost_to_enemy_aircraft` (7 to 23), `japanese_bombers_lost_to_enemy_action` (12, a miss now), `japanese_fighters_lost_to_enemy_action` (6 to 10, at the edge now), `us_aircraft_destroyed_on_the_ground` (2 to 3) and the claims ratio (250 to 382).
- **Reported:** `japanese_bombers_forced_down` (5 to 6), `bomber_sorties_that_reach_guadalcanal` (211 to 213), raid days, US pilots lost.
- **The SBDs ready on day 5 as well as day 21**, and the write-offs by group from the events, so that the round shows the write-offs against the rows and not only the ready counts.
- The distance term also wears Rabaul's bombers and escorts. Fewer of them reach Henderson Field, which moves the losses on both sides. Those are reported, not chased.
- **The Eastern Solomons** does not load `air_upkeep.toml` (scenario 5 names no air upkeep file), so none of its verdicts can move. This is shown with one batch on a figure copy.
- **Savo Island** is not rerun: no surface figure is touched.

## Named and not tuned to

- **Rows 3976 and 3977:** 80% of the aircraft available the day after an all-out attack, and 50% on the third. This is one officer's recollection, the 25th Air Flotilla's senior staff officer interrogated after the war. It speaks of 1942 in general and ties it to supply. It is named, not tuned to.
- **The fuel figure.** It is not tuned here. Under decision 0084 the ditchings of Ryujo's raid are not evidence for it.
- **The SBDs ready on day 21** beyond what the write-off rows carry: the pool under repair, and the combat losses on strikes at ships, above.
- **The fighters over their rows** (8.2 against about 6.1) through `wear_base_chance`. One figure is moved at a time. It is named for a later round, if there is one.
- **Henderson's and Rabaul's repairs** (3 and 4 a night) and `carrier_repairs_per_day` (2). No row sizes them, and no tuned battle reads the carrier figure.

## What stops the round

The round moves `wear_chance_per_100_nm` and nothing else. It stops, and the figure stays at 0.02 with the reason written here, if any of these holds:

1. **A move turns an agreeing judged measure into a miss.** Then it goes to the architect before anything moves.
2. **A move helps only by standing in for another cause.** The size is set by the write-off rows, about 7.8 SBDs in the window, and not by `sbds_ready_on_day_21`. A value past the rows' size, taken to close the ready count, would stand in for the pool under repair and for the combat losses above. It is shown in the grid and not proposed.
3. **No row sizes the move.** If the grid shows that no value brings the SBDs' write-offs to their rows (for example, because damaged aircraft fly fewer searches), the figure stays and the gap is a named miss.
4. **The fit needs an absurd value,** such as a chance above 1.

The SBDs ready on day 21 is expected to stay a miss after the round. That is a finding, and does not stop the round.

## What was run

1,000 runs of Rabaul against Guadalcanal at each value of `wear_chance_per_100_nm`, seeds 1 to 1,000, on scratch copies of the figures. Medians, with the verdict. Written off is the mean a run, counted from the events.

| `wear_chance_per_100_nm` | 0.02 (now) | 0.025 | 0.03 | 0.035 | 0.04 | 0.06 | 0.1 |
| --- | --- | --- | --- | --- | --- | --- | --- |
| SBDs ready on day 21, 22 | 30: miss | 30: miss | 29: miss | 29: miss | 28: miss | 27: miss | 23: agrees |
| SBDs ready on day 5 (VMSB-232 and Flight 300) | 18 | 17 | 16 | 15 | 15 | 12 | 7 |
| VMSB-232 ready on day 5, 9 | 10: agrees | 10: agrees | 10: agrees | 9: agrees | 9: agrees | 8: agrees | 5: miss |
| F4Fs ready on day 21, 11 | 9: agrees | 10: agrees | 12: agrees | 14: agrees | 15: at the edge | 18: miss | 22: miss |
| VMF-223 ready on day 10, 4 to 5 | 5: agrees | 6: agrees | 7: agrees | 7: agrees | 8: at the edge | 9: at the edge | 11: miss |
| VMF-223 ready on day 5, 11 | 12: agrees | 12 | 12 | 12 | 12 | 12 | 12 |
| US fighters lost to enemy aircraft, 7 to 23 | 25: agrees | 22: agrees | 20: agrees | 18: agrees | 17: agrees | 12: agrees | 9: agrees |
| Japanese fighters lost to enemy action, 6 to 10 | 13: at the edge | 12: agrees | 11: agrees | 10: agrees | 10: agrees | 8: agrees | 6: agrees |
| Japanese bombers lost to enemy action, 12 | 29: miss | 30: miss | 32: miss | 32: miss | 33: miss | 33: miss | 32: miss |
| Japanese bombers forced down, 5 to 6 (reported) | 10 | 12 | 13 | 14 | 15 | 18 | 20 |
| Bomber sorties reaching the island, 211 to 213 (reported) | 212 | 205 | 196 | 186 | 177 | 145 | 117 |
| Destroyed on the ground, 2 to 3 | 2: agrees | 2 | 2 | 2 | 2 | 2 | 2 |
| Claims per hundred shot down, 250 to 382 | 312: agrees | 317 | 320 | 325 | agrees | agrees | agrees |
| Written off, SBDs (rows about 7.8) | 5.7 | 6.4 | 7.1 | 7.7 | 8.2 | 10.0 | 13.1 |
| Written off, SBDs by day 21 (groups 11, 12, 14) | 4.3 | 4.8 | 5.3 | 5.7 | 6.1 | 7.5 | 9.8 |
| Written off, F4Fs (rows about 6.1) | 8.2 | 8.4 | 8.7 | 8.9 | 9.0 | 9.5 | 9.8 |
| Written off, G4Ms (forms 5 to 6, a floor) | 10.4 | 12.0 | 13.3 | 14.4 | 15.3 | 17.9 | 19.7 |
| Written off, Zeros | 8.6 | 9.2 | 9.3 | 9.4 | 9.5 | 9.6 | 9.5 |

**The Eastern Solomons.** 1,000 runs of scenario 5 on the copy at 0.1 give the same final hash in every run as on the live figures. None of its measures moves at any value of the grid.

**What the grid shows.**
- The SBDs' write-offs reach their rows, about 7.8, at about 0.035. There SBDs ready on day 21 is 29, still a miss. So the rows carry about one aircraft of the eight, as the record expected.
- Every judged measure that moves for the better does so through the Japanese side: Rabaul's bombers and escorts wear more, fewer reach Henderson Field, and fewer American fighters are shot down. The Japanese bombers' write-offs go from 10 to 14 against their forms' 5 to 6, away from that source, and the bomber sorties reaching the island go from 212 to 186 against the forms' 211 to 213.
- From 0.04 two judged measures go from agreeing to at the edge, and at 0.06 one is a miss. Only at 0.1 does the SBD count agree, with three other judged measures missing and the SBDs' write-offs at 13, beyond their rows: a stand-in for other causes.
