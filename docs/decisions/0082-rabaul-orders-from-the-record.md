# 0082: Rabaul against Guadalcanal: the raid days and the dive bombers' searches from the record

Status: decided by the architect on the designer's delegation, Nov 13, 2026. This is scenario data, not a rule: no rule changed and nothing under `crates/sim-core` changed. It was written after a scratch run had been seen (the air tuning plan of Nov 13, 2026, which ran the scenario with the record's raid days), and it rests on the units' forms and the fact rows named below, not on that run. It is made before any air figure is tuned on Rabaul or checked there (`docs/step2-spec.md`, "Tuning", 8). It replaces items 16 and 17 of decision 0042 for the live scenario. Decision 0042 keeps its text, with a dated note pointing here.

## The gap

Decision 0042 set three orders that no row sets:

- **The raid days came from fatigue.** The staff rested the Japanese groups above a fatigue of 45. With the placeholder figures, the bombers flew on days 1, 2 and 3 and then every third day. Decision 0042 said so: fatigue stood in for weather and for the command's rhythm. The units' own forms give the days the raids reached Henderson Field: 25 and 26 August, 29 and 30 August, 2 and 5 September, and 9 to 13 September. No bomber raid reached the field before 25 August.
- **The dive bombers were held for ships.** The scenario holds no ships, so they never flew, never wore and sat on the field where the bombs found them. The rows have them searching and striking ships.
- **A standing patrol, an Allied rest threshold of 40 and Rabaul's replacements of 2 and 2 a day.** No row sets any of them (item 3 below).

By "Tuning", 8, a recorded schedule outranks a stand-in. Where no row says what an order was, the scenario says so and keeps the default.

## What the record says

**The raids.** All rows are `data/oob/engagement_outcomes.csv`. The forms are tabled in `docs/research/oob/japanese-air-records.md`, section 1.

| Date | Day | Bombers sent (forms) | Fighters sent (forms) | Rows | US interceptors, by row |
| --- | --- | --- | --- | --- | --- |
| 25 Aug | 5 | 23 | 12 | 1115, 1044, 982 | none stated |
| 26 Aug | 6 | 16 | 9 | 1047, 987 | 12 (533, low) |
| 29 Aug | 9 | 18 | 12 of Shokaku's, none of Tainan's | 1059, 1005 | 10 F4F (544, low) |
| 30 Aug | 10 | 18 (20 with two of the 4th's, 1118) | 13 of Shokaku's; Tainan's 8 turned back | 1065, 1008 | 15 with P-400s (555, 556) |
| 2 Sep | 13 | 18 | 9, and 5 of Shokaku's | 1075, 1011 | none stated |
| 5 Sep | 16 | 18 | 18 | 1080, 1015 | 18 (592, low) |
| 9 Sep | 20 | 18 | 18 | 1085, 1019 | none stated |
| 10 Sep | 21 | 22 | 15 | 1089, 1023 | 11 (608); 5 (609, low) |
| 11 Sep | 22 | 24 | 18 | 1094, 1028 | 12 (618, low) |
| 12 Sep | 23 | 20 | 21, with the 2nd Air Group's | 1100, 1033 | 32 (628) |
| 13 Sep | 24 | 16 | 21 | 1108, 1037 | 20 (640); 28 against the morning's Zeros (1353) |

Raids that turned back, by the forms: 23, 24, 27, 28 and 31 August, and 1, 4 and 16 September. Row 954 has "14 attack missions" launched from 21 August to 12 September and four aborted for weather. That leaves ten that reached the field by the 12th, the forms' count, with the raid of the 13th the eleventh. The forms and row 954 do not agree on how many turned back; that changes no order here.

**The dive bombers.** Their searching is from rows:
- events 1189 (low): eight dive bombers left before dawn on 25 August "to search for enemy carriers" and made contact "about 125 miles to the north";
- outcomes 314, 418 and 419: the same sortie, 8 or 12 aircraft;
- aircrew_fates 115 (low): a Marine SBD shot down "returning from a search mission" on 13 September;
- aircrew_fates 117 (low): a VS-3 SBD ditched "during their search patrol" the same day;
- outcomes 475 (low): a VMSB-231 scout SBD shot down on the 13th.

Their strikes on ships are in rows 1763, 1765, 1777, 1817 and 1867. No row gives a search arc, a distance, a number of aircraft or a schedule. The contacts the rows place lie north (1189; 1765, about 150 miles north of Savo; 1777, 70 miles north) and north-west (1817, Gizo). NACS gives the Marine dive and torpedo bombers 20 action sorties in August (note on rows 1274 to 1276) and 125 in September (1280). It gives the Navy's 11 and 122 (1290, 1295).

**Warning, the patrol and replacements.** What the rows say is under item 3 below.

## Decisions

1. **The raid days are the record's, as dated orders.**
   - The bombers (group 1) are on rest from day 1. They are ordered to base strike on days 5, 9, 13, 16 and 20, and to rest on days 7, 11, 14, 17 and 25. So they raid on days 5, 6, 9, 10, 13, 16 and 20 to 24, the eleven days of the table, and on no other.
   - A scenario's `[[order]]` with a `day` is the scenario's script for that day. It is taken in at the start of the day, after refuelling, and checked then (`docs/rules/01-the-day-orders-and-what-a-side-knows.md`, "When an order is checked"). This is its intended use, as in scenario 6c2 and the soak.
   - A base strike group strikes once a day in the first daylight pulse in which the base is in reach. So the order is the day's raid.
   - How many bombers and escorts fly each day is what the groups have ready, cut by their fatigue: the rules decide it. The forms' numbers sent are kept above as the check, day by day ("As built").
2. **No Japanese rest threshold.**
   - No row sets one, so the orders that set 45 for groups 1 to 3 are removed, which is the default.
   - Kept, a threshold would override the record: five raid days running (20 to 24) put the bombers' fatigue past 45, and the staff would stand them down on days the forms have them flying.
   - Fatigue still cuts the size of each raid, as it does every group.
3. **The fighters at Rabaul** (groups 2 and 3) keep their patrol share of 0 over Rabaul (0042, 15). They escort only on a raid day, since an escort goes only with a raid.
4. **The dive bombers search.**
   - All four SBD groups, 11 (VMSB-232), 12 (Flight 300), 14 (VMSB-231) and 17 (VS-3), are on the search mission from day 1. Each is named searching by a row, except Flight 300, which flew with VMSB-232 on 25 August (note on 418).
   - The arc is 300 to 030 degrees true, to 200 miles. **The arc and the distance are a reading, not a row, approved by the architect.**
     - The arc takes in the north and the Slot, where the rows' contacts lie. It is immaterial while the scenario holds no ships.
     - The distance is the reading that matters, because wear rises with it. 200 miles is the SBD-3's search distance as the type's rows record it flown elsewhere: aircraft_type_facts 280 (200, high) and 282 (180), events 144 and 146. Every contact the Henderson rows place lies inside it.
     - The SBD-3's search radius is 397 (decision 0075), so the order is accepted.
   - No rest threshold is given: no row sets one.
   - The existing wear rule now works on them, at twice 200 miles a sortie (chapter 5, "What counts as a sortie").
5. **The standing patrol, the Allied rest threshold of 40 and Rabaul's replacements of 2 and 2 a day are left as decision 0042 set them,** marked in the scenario as invented, with what would settle them.
   - The rows do not set them:
     - The rows have the fighters scrambled on a coastwatcher's alarm (545, 572, 1328, 1799) and reaching height late (544; 1334's note). A coastwatcher reported 21 planes at 1100 for the raid of 1130 on 25 August (note on 1759), which is a lead of about half an hour; no row states a lead time. The only lead time read, Feldt's "about two hours", is in `air-search.md` for 7 to 8 August, not a row.
     - No row says the fighters kept a standing patrol or rested at a set fatigue.
     - Rabaul's arrivals are totals with no day or unit: air_unit_facts 335 and 336 (60 fighters and 72 bombers, undated), 337 (85 aircraft, 15 to 28 September) and 334 (140, an American estimate).
     - The dated rows are the 6th Air Group's 13 Zeros on 31 August (32, low, disputed against 18 in 31), already the scenario's arrival on day 11, and 19 G4M1 of the Kisarazu Air Group on 22 August (50, 52, low; Kavieng on the 20th by 51).
   - **Proposal, not a change:** the replacements cannot yet be set as dated arrivals from rows. Only the Kisarazu row is a dated, single-type arrival for a unit the scenario holds and that is not already in it. It is low, disputed as to place and date, and falls on day 2, before any raid. Research on the 26th Air Flotilla's daily strength by unit would settle it.

### Where the rows disagree or are unclear

- **30 August.** The forms count 18 or 20 bombers (1118 adds two of the 4th Air Group). The Tainan Air Group's eight escorts turned back, and Shokaku's thirteen, a unit the scenario leaves out, fought. The scenario flies the day's raid with whatever escort the Tainan and 6th groups have ready, and none turns back.
- **How many raids turned back:** the forms have seven by 12 September, row 954 four. Neither is flown.

## What the rules cannot express

- **Weather.** A raid that set out and turned back is not flown at all. The raids that turned back flew, tired and wore the force; here they do not. Fatigue and wear before day 5 are therefore understated.
- **A raid or an escort that turns back** (30 August's Tainan escort; the 16 September raid).
- **Units outside the scenario's scope that flew those days:** Shokaku's fighters from Buka (29 and 30 August, 2 September), the 2nd Air Group (12 September), the reconnaissance escort of 13 September. The scenario's escorts are the Tainan's and the 6th's alone.
- **A search by sections.** A group searches whole in both daylight pulses with all it can fly. The record sent sections. The scenario's searches are cut by fatigue alone.
- **The dive bombers' strikes on ships** (1763, 1765, 1777, 1817, 1867) cannot be flown: scenario 6 holds no ships. A group flies one mission, so it cannot hold a strike and search too.
- **Warning and height** stay as decision 0042 lists them.

## Who follows

- **The full slice (07) and the soak (08)** copy scenario 6's campaign on days 1 to 28, as they did for decisions 0063 and 0073. Both take the raid days and drop the Japanese rest thresholds; the slice runs 14 days, so it has the orders to day 14.
- **The soak from day 29.** The record's schedule ends with scenario 6's window. From day 29 the soak goes back to the standing orders of 0042, 16 (base strike, rest above 45). This is marked as the soak's, not the record's: the soak exists to run the campaign long (decision 0044).
- **The dive bombers stay on naval strike in 07 and 08.** Both hold ships, and the rows have these dive bombers striking the convoy on 25 August (1763, 1765, 314, 418, 419), which is day 5 of the slice. A group can fly only one mission. Item 4 rests on "no ships in the scenario", which does not hold there.
- **The Allied patrol and threshold** are unchanged there, marked invented.

## Measures (ruled by the architect, who now owns `data/validation`)

- **M3.** On day 5 VMF-223 is the only F4F group with aircraft, so `vmf_223_ready_on_day_5` already counts all the F4Fs, and no bookkeeping enters it.
  - For day 10 no row gives all the F4Fs ready on 30 August: 893 and 568 count the first echelon, 556 (low) counts eight Grummans before VMF-224 landed, and 895 is strength at the month's end.
  - So no measure is added. The reasons of the two squadron measures say what they read.
- **M4.** The reasons of `sbds_ready_on_day_21` and `vmsb_232_ready_on_day_5` say three things:
  - before this decision the SBDs never flew;
  - they now rest on the 200-mile reading;
  - at 150 miles their medians move by one aircraft and none ("As built").
- **M5.** The `[base_hit]` figures (the attack on a field) cannot be tuned on the Eastern Solomons, which has one six-bomber raid on a field (about 0.4 hits on aircraft a run). They are tuned on Rabaul and checked on no battle. This is said in `data/validation/README.md`, and put to the spec session for `docs/step2-spec.md`, "Tuning".
- **M7.** The reason of `us_fighters_lost_to_enemy_aircraft` says that the record counts bombers' guns and the rules have no return fire.
- **`raid_days_that_reach_guadalcanal`** no longer tests the choice of days, which are ordered. It tests whether an ordered raid still reaches the field, which only the slow figures decide.
- **`bomber_sorties_that_reach_guadalcanal`** now tests the size of each raid on the record's days: the slow figures' check on the bombers.
- Both stay reported. The architect's judgment on judging them is in the pull request.
- No record, range or judged flag moved.

## Rule card

No rule. These are scenario inputs. The batch file's `air_losses` now omits a group whose values are all zero, as decision 0062 always said. The SBDs' searches made such a row (wear with nothing written off), and the batch test caught it.

## Frozen figures go with frozen scenarios

`data/scenarios/step2-frozen/06_rabaul_guadalcanal.toml` is scenario 6 as it stood on main before this decision. Every test that runs scenario 6 on the frozen figures reads it:
- `tests/scenario_06_rabaul_guadalcanal.rs`, the hand calculation;
- `scenario_09a_side_events.rs`;
- `scenario_10b_objectives.rs`;
- `scenario_11_claims.rs`;
- every test of `crates/sim-agent/tests`, through `support::scenario_path`.

No expected number changed. Tests on the live figures read the live file (`scenario_07_determinism.rs`, `crates/sim-cli/tests/batch.rs`, the run baselines). The frozen copies of 07 and 08 are older and untouched.

## As built

Built Nov 13, 2026, on branch `rabaul-orders`. 1,000 runs, seeds 1 to 1000, live figures, validated against `data/validation/rabaul_guadalcanal.toml` as changed. "Before" is main at cdae5bc; "after" is this branch. The report is `docs/validation/rabaul-guadalcanal-air-baseline.md`.

| Measure | Record | Before: 5th to 95th, median, runs holding, verdict | After |
| --- | --- | --- | --- |
| japanese_bombers_lost_to_enemy_action | 12 | 6 to 18, 12, 110, agrees | 11 to 22, 16, 69, agrees at the edge |
| japanese_fighters_lost_to_enemy_action | 6 to 10 | 30 to 50, 39, 0, miss | 16 to 30, 22, 0, miss |
| japanese_bombers_forced_down | 5 to 6 | 9 to 20, 14, 9, reported | 6 to 17, 12, 54, reported |
| us_fighters_lost_to_enemy_aircraft | 7 to 23 | 27 to 40, 34, 10, miss | 16 to 29, 23, 583, agrees |
| us_aircraft_destroyed_on_the_ground | 2 to 3 | 10 to 23, 15, 0, miss | 7 to 19, 12, 1, miss |
| raid_days_that_reach_guadalcanal | 11 | 11 to 11, 11, 1000, reported | 8 to 9, 9, 0, reported |
| bomber_sorties_that_reach_guadalcanal | 211 to 213 | 252 to 272, 263, 0, reported | 202 to 221, 212, 194, reported |
| us_aircraft_claimed_per_hundred_shot_down | 250 to 382 | 117 to 145, 129, 0, miss | 122 to 160, 138, 0, miss |
| japanese_aircraft_claimed_on_ground_per_hundred_destroyed | 2066 to 4600 | 192 to 377, 257, 0, reported | 188 to 420, 254, 0, reported |
| vmf_223_ready_on_day_5 | 11 | 4 to 13, 9, 93, agrees at the edge | 7 to 14, 10, 167, agrees |
| vmsb_232_ready_on_day_5 | 9 | 5 to 12, 9, 228, agrees | 8 to 12, 10, 164, agrees |
| vmf_223_ready_on_day_10 | 4 to 5 | 0 to 8, 3, 205, agrees | 0 to 6, 1, 128, agrees at the edge |
| f4fs_ready_on_day_21 | 11 | 1 to 9, 3, 12, miss | 2 to 12, 6, 51, agrees at the edge |
| sbds_ready_on_day_21 | 22 | 22 to 32, 28, 35, miss | 19 to 31, 25, 55, agrees at the edge |
| us_pilots_lost | 9 to 22 | 8 to 19, 13, 943, reported | 5 to 13, 9, 573, reported |

- **Judged 10.** Before: 3 agree, 1 at the edge, 6 miss. After: 3 agree, 4 at the edge, 3 miss.
- **At 150 miles in place of 200**, on a scratch copy of this branch:
  - `vmsb_232_ready_on_day_5` is 7 to 12, median 10, 163 runs;
  - `sbds_ready_on_day_21` is 19 to 31, median 26, 69 runs.
  - Every other measure is within a run or two.

**The fights by raid day, after** (means over 1,000 runs; the median run's figures within one). "Patrol" is the Allied fighters that fired, at escorts and bombers; "at bombers" is the part that reached the bombers.

| Day | Date | Bombers launched | Bombers attacking | Forms: bombers sent | Escorts | Forms: fighters sent | Patrol (at bombers) | Interceptor rows | Bombers lost | Zeros lost | F4Fs lost |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| 5 | 25 Aug | 41.0 | 40.6 | 23 | 24.0 | 12 | 12.9 (0) | none | 0.4 | 3.9 | 4.1 |
| 6 | 26 Aug | 27.6 | 27.2 | 16 | 12.6 | 9 | 7.4 (0) | 12 | 0.4 | 2.2 | 2.1 |
| 9 | 29 Aug | 31.5 | 31.1 | 18 | 14.1 | 0 of Tainan's (12 of Shokaku's) | 4.4 (0) | 10 | 0.4 | 1.4 | 2.1 |
| 10 | 30 Aug | 21.3 | 17.2 | 18 to 20 | 8.5 | 8, turned back (13 of Shokaku's) | 20.7 (12.2) | 15 | 4.1 | 2.6 | 1.5 |
| 13 | 2 Sep | 23.4 | 23.0 | 18 | 24.0 | 9 (and 5 of Shokaku's) | 14.4 (0) | none | 0.4 | 4.3 | 4.1 |
| 16 | 5 Sep | 24.0 | 23.5 | 18 | 23.1 | 18 | 8.1 (0) | 18 | 0.4 | 2.4 | 3.8 |
| 20 | 9 Sep | 28.0 | 27.6 | 18 | 28.6 | 18 | 0.1 (0) | none | 0.4 | 0.0 | 0.1 |
| 21 | 10 Sep | 17.9 | 17.6 | 22 | 16.5 | 15 | 5.6 (0) | 11 (5) | 0.4 | 1.7 | 2.6 |
| 22 | 11 Sep | 10.2 | 4.2 | 24 | 8.8 | 18 | 27.6 (18.8) | 12 | 6.0 | 2.7 | 1.5 |
| 23 | 12 Sep | 2.1 | 0.0 | 20 | 3.1 | 21 | 21.1 (18.0) | 32 | 2.1 | 0.9 | 0.5 |
| 24 | 13 Sep | 1.0 | 0.0 | 16 | 1.7 | 21 | 40.2 (38.5) | 20 | 1.0 | 0.5 | 0.3 |
| All | | 228.1 | 212.0 | 211 to 213 | 165.1 | | 162.5 (87.4) | | 16.1 | 22.5 | 22.7 |

**The bombers sent by raid day in the middle run**, beside the forms. This is the table the fatigue round will be judged on.

| Day | 5 | 6 | 9 | 10 | 13 | 16 | 20 | 21 | 22 | 23 | 24 |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Median run | 41 | 27 | 32 | 21 | 24 | 24 | 28 | 18 | 10 | 2 | 1 |
| Forms | 23 | 16 | 18 | 18 | 18 | 18 | 18 | 22 | 24 | 20 | 16 |

What it shows:

- **The fatigue figures cannot fly five raid days running.** This is what the placeholder figures do with the recorded schedule, and nothing was changed for it. A raid of 1,112 miles costs a group 25.24 fatigue, and a day without flying gives back 5 a pulse. By day 22 the bombers fly a quarter of what they have, and on days 23 and 24 one or two, which the patrol shoots down before they bomb. So `raid_days_that_reach_guadalcanal` is 8 or 9 of the 11 ordered.
- **The first raids are too big.** Day 5 is the group's full 41 against 23 sent, and day 9 is 32 against 18. The forms' size is that of a group that had flown and turned back on the days between.
- **The total comes out right for the wrong reasons.** Bomber sorties reaching the field come to 212 against 211 to 213, but as early raids too big and late raids too small.
- **Day 20.** Every Allied fighter group is stood down by the invented threshold of 40 on day 20. The raid meets no patrol.
- **Days 10 and 22 to 24.** The patrol outnumbers the escort and reaches the bombers. That is where 13 of the 16 bombers lost fall.

Before (decision 0042's days), for comparison: raids on days 1, 2, 3, 6, 9, 12, 15, 18, 21, 24 and 27 of 41, 27.6, 19.1, 23.3, 24, 24, 24, 23, 23, 23 and 22.7 bombers. The patrol reached the bombers with 24.6 shots a run, against 87.4 after.

**The dive bombers' sorties, the order-of-size check** set before any round and never moved after. These are search sorties in the median run, against NACS action sorties of Marine and Navy dive and torpedo bombers.

| Period | Scenario (median, 5th to 95th) | Record |
| --- | --- | --- |
| 21 to 31 August (days 1 to 11) | 135 (121 to 144) | 31 for the whole month: 20 Marine (note on 1274 to 1276) and 11 Navy (1290) |
| 1 to 17 September (days 12 to 28) | 89 (78 to 98) | 247 for the whole month: 125 Marine (1280) and 122 Navy (1295); about 140 pro rata for 17 days |

- In August the scenario flies about four times the record's sorties; in September about two thirds.
- NACS counts action sorties. A search may fall under its "other flights", so this is an order of size and no more.
- The SBDs now wear: 29 worn and 6 written off a run, median. Their losses on the ground fall from 10 to 5 a run.

**Remade because they follow live data**, in commits of their own:
- the run baselines of scenarios 6, 7 and 8;
- the values of scenario 6's captured version 1 batch file (`crates/sim-cli/tests/batch_before`): the scenario hash, and the three runs' final hashes and margins;
- the recorded full slice game.
  - **Its transcript moved, which is a finding.** The slice now has no raid on day 1, so seven briefings and one `explain` answer differ from day 1 on (`crates/sim-cli/tests/recorded_games/README.md`).
  - The two Eastern Solomons games are unchanged.

No hand-calculation test's expected number changed. Nothing in `data/reference` changed: no figure is tuned in this stage.
