# 0084: What the agreements show, and what they do not

Status: Decided by the architect on the designer's delegation, Nov 13, 2026 ("Go ahead and decide all matters that need decisions"). Measures, one option of a take of `validate`, and the wording of tuning records and decisions; no rule changed, no figure moved and nothing under `crates/sim-core` changed.

## Why

An outside review of Nov 13, 2026 read the step 2 tuning adversarially and made fifteen findings. A review is a lead, not a source: every claim acted on here was checked against the repository's file and line or the fact row, and where a finding is wrong this record says so. The rulings are the architect's; this record carries them out and says what the agreements so far can and cannot be taken to show.

Three things run through most of the findings:

- **A measure is judged alone.** The rule of agreement tests each measure's spread against its record, one at a time. Agreements on several measures do not show that any one run holds them together, and a total can agree while its parts are wrong. Where that was so, the parts now have measures of their own (findings 3 and 4).
- **A figure fitted inside a supported direction is still fitted.** Where the sources give a sign and no size, the size chosen is a fit to the battle it was tuned on, and is a placeholder however well it fits (findings 2, 5 and 6).
- **A choice made with the outcome known is a limit of the check.** Orders, levels and radii set from rows can still have been chosen among readings with the result in view; where so, it is said, and the check is narrower (findings 1, 7, 10, 12 and 13).

## The findings and what was done

### 1. What the Cape Esperance check can and cannot show. Accepted

Checked: decision 0070's cases include Salt Lake City (296 rounds in 29 minutes, outcomes 1704 and 1638) and Helena (580 rounds, outcome 2980) at Cape Esperance, and Helena's is the only case of continuous fire, kind (c) (0070, "Which rows are sources", which already says so). Decision 0072 works the Japanese speed back from the hour fire opened, reads Task Force 64's break-off level from its conduct, sets the Japanese level at 0.5 (invented), and leaves out the force that passed through without fighting.

**What the check can show:** whether figures tuned on Savo Island give the right losses and hits received when the other navy had the radar and the surprise, on inputs fixed before the run.

**What it cannot show:** the rate of fire of the US 6-inch cruisers, which rests on Helena's own firing in that battle; Salt Lake City's rate, which is one of the four salvo cases; when the forces met, who took part and when each side broke off, which the scenario set with the outcome known; and how well each side shot for the shells it fired, since no measure of hits for each thousand shells is possible there (below). An agreement there cannot tell accuracy from how long a side fired, its readiness and the damage it took.

**Committed in advance,** in `docs/step2-spec.md` ("Tuning"): the check is one run of the batch, the first 1,000 runs on the figures and rules as they stand when the scenario moves out of `not-run/`. Its report is kept as it comes out. Any rule or figure changed after that report is read is not checked by it, and the change's record says so.

**No hits-for-each-thousand-shells measure was added,** because the rows do not allow one for either side (`cape_esperance.toml`, its closing note, rewritten here). US main battery shells are on record for Salt Lake City (1636, 1704), Helena (2980, 3001), Boise only as "more than 1,000" of two calibres from the press (1715), and McCalla (1641); none for San Francisco and four of the five destroyers, and the hits on the Japanese ships are the whole force's. The Japanese cruisers' main battery rounds are on record (2276 to 2282; 1648 to 1653; 1700), the destroyers' are not (2283), and the hits on US ships include an uncounted share of their own side's fire. Nothing of the scenario was run, batched, validated or read for this.

### 2. Round 2's surprised-target factor. Accepted

Checked: row 19 (the 1943 narrative) says at least 24 five-inch shells; row 20 (the 1950 analysis) says 8-inch and 4.7-inch; row 21 (the Australian war memorial's encyclopedia) gives the count and no calibre. The round 2 record had put rows 19 and 20 together under the mixed calibres. Corrected there.

The three fifths that sized the factor is the whole Japanese force's expenditure over the whole action, applied to Canberra as an assumption no row supplies; the target research (decision 0078) has the heavy cruisers' main batteries, their secondary guns and the light cruisers' guns at different targets. So `surprised_target_factor` = 2 is a figure fitted at Savo Island inside a direction the sources support, a placeholder, and not sized from a source. The Canberra count cannot size it for torpedoes, which it also multiplies. `docs/validation/tuning/r2-surprised-target-factor.md` now says so.

**A correction to a fact row, for the research owner.** Row 19's locator names the page "End of the Canberra" (USN-CN-Savo-8), which gives no count of her hits; the passage it quotes is on "Attack on our Southern Group" (USN-CN-Savo-4). Both pages were read again on Nov 13, 2026. The row is not overwritten; the correction is listed in `data/oob/gaps.csv` for the next research pass.

### 3. Totals that agree with the wrong ships hit. Accepted

Checked on the round 2 batch (seeds 1 to 1,000, the figures on `main`): the northern group takes no torpedo hit in any run, against 3 to 6 in the record, while the total on the nine ships agrees; Chicago is sunk in 560 runs of 1,000. The review's other figures for this finding come from decision 0083, which is not on `main` and is not relied on here.

New measures in `data/validation/savo_island.toml`, all judged, with their rows:

| Measure | Record | Rows | First result (round 2 batch) |
| --- | --- | --- | --- |
| `torpedo_hits_on_the_southern_group` | 1 to 3 | 25, 23, 24, 163, 38, 39 | agrees: 1 to 8, median 4, held by 418 |
| `torpedo_hits_on_the_northern_group` | 3 to 6 | 61, 62, 63, 72, 73, 85 | **miss**: 0 in every run |
| `chicago_sunk` | 0 | 768, 769 | agrees: 0 to 1, median 1, held by 440 |
| `gun_hits_on_vincennes` | 57 to 74 | 58, 59, 60 | **miss**: 4 to 45, median 33, held by 1 |
| `gun_hits_on_quincy` | 36 to 54 | 69, 70, 71 | at the edge: 2 to 42, median 28, held by 190 |
| `gun_hits_on_astoria` | 34 to 65 | 83, 82 | agrees: 0 to 52, median 23, held by 357 |

**Marginal agreements do not show one run holding them together.** The judge stays as it is: each measure alone. A joint condition was not asked for and is not built.

The review is right that the closure of the chance to hit in round 2 multiplied the record's hit rate by the model's 1,400 main battery shells, not the record's 1,090 to 1,190; that sum shows what the rules could give at the model's expenditure, not that the measure was met as far as the record allows. The overfiring of the main batteries and the missing secondary guns are not separated by it.

### 4. The Eastern Solomons strike measures. Partly accepted

The wave sizes from the carriers' forms stand: they are independent of any run. What the old measures checked, the aircraft left to attack after the patrol and the guns, is kept beside them under new ids with the old take, `aircraft_attacking`, escorts not counted:

| Measure | Record | Judged | Rows | Result (baseline batch, remade after decision 0079) |
| --- | --- | --- | --- | --- |
| `japanese_bombers_left_to_attack_enterprise_force` | 20 to 27 | no | 390, 389, 290, 1159, 1164 | reported: 0 to 18, median 11, held by 16 (before 0079: 0 to 9, median 1, by none) |
| `us_aircraft_left_to_attack_ryujo` | 36 | yes | 333, 1238, 334, 335, 234, 235, 351 | agrees: 0 to 38, median 0, held by 130 |

The Japanese measure is not judged: no row counts the bombers that reached release. The rows that try (390, Enterprise's report, "20 to 40, probably about 30"; 389, the 1943 narrative, about 30 left after about 6 destroyed before release) are the target's estimates of a wave they put at 36 dive bombers where the forms launched 27; the record's high end is the 27 launched. The American measure is judged: the narrative counts the bombs released by each squadron and the torpedo planes at Ryujo and her cruiser, and none was lost.

**Each strike measure on Ryujo takes the count it counts** (ruled by the architect, Nov 13, 2026). `us_aircraft_attacking_ryujo`, the aircraft that took part as launched, now has the record 38 (row 231), and still agrees (held by the 488 runs in which the strike reaches her). `us_aircraft_left_to_attack_ryujo` has the 36 that reached and released (333, 1238, 334, 335). The earlier range of 36 to 38 was wrong by the measures file's own rule: a part of the record out of the rules' reach is taken out of the record, which leaves a point, and a range spans only a dispute, which the rows do not have. The two that turned back for mechanical trouble are outside the rules, which launch them, so a run's count left to attack can fall short of 38 only by what the patrol and the guns take; a run in which they take none attacks with 38 and does not hold 36. That limit is named in the measure's reason. Decision 0081 has a dated note.

The review's further point, that the second wave and Enterprise's attack group are put at rest because their recorded outcome was no attack, is a limit of decision 0081 that it states itself ("What the rules cannot express"): those groups cannot fail a measure of attacks by attacking.

### 5. The two endpoint shares of decisions 0079 and 0080. Accepted as wording

Decisions 0079 (open in pull request 210) and 0080 (being built) are not edited here. This record states: `anti_aircraft_before_attack_share = 0` and `patrol_drawn_off_share = 0` are endpoint placeholders, chosen with both air battles' results known. No row sizes either. They are moved in the air rounds only toward rows. Rabaul against Guadalcanal is a tuning battle for them, not a check: its losses were in view when they were chosen.

### 6. Round 1's experience strength. Accepted

`experience_gun_strength` = 0.5 is an effect size fitted at Savo Island, at the cap decision 0064 allows, with a sign from the sources and no size. Round 0 refused to move `not_alert_sighting_factor` because no row gives a size, though rows give its sign too. That reason applies to round 1 as much. The one difference is that round 0's measure already agreed and round 1's did not; since a round aims at the middle even after agreement ("Tuning", 10), that is not a difference of principle, and the record of round 1 now says the two rounds applied the rule of support differently. Neither round is undone.

### 7. The radar flag for sets with no plan display. Recorded, not changed

Decision 0077 gives a class the radar figure only for a surface-search set with a plan position indicator, and nothing for any other set. That zero is an interpretation, not a finding of the rows. Checked: `fits.csv` 291 has Chicago's radar contact with Bagley at 7,000 yards at 02:26, the set's type not stated; 661 has Farenholt's contact at 13,000 yards at Cape Esperance, the set not named; 355 and 390 have Canberra's Type 271 working that night but masked by land, with no telephone to the bridge. So two sets of unstated type made surface contacts at night. (The review calls them older sets; the rows do not name them.) A graded benefit for a set with no plan display, and for one masked by land or not reported to the bridge, is an open question for the research of step 3.

### 8. Rate of fire, readiness and gun damage. Recorded

The separation of the three is not established. The achieved rates of decision 0070 divide the shells by every fitted gun of the battery and by the span from first to last fire, pauses included; the rules then multiply that rate by the share of a force not ready (0068) and by main battery damage. So some of the suppression the achieved rates hold, from guns out of action, lost targets and orders to cease fire, may be counted twice. How much is not known.

### 9. Measures a run holds by doing nothing. Accepted

`southern_group_main_battery_shells_fired_in_hundreds` had the record 0 hundreds for 33 to 50 shells, which a run that never fired held as well. It is retired. `southern_group_main_battery_shells_fired_in_tens` counts the same shots in tens, record 3 to 5 (rows 27, 41, 48, 53, 54): first result a **miss**, 0 to 47 tens with a median of 9, held by 10 runs. The group fires either nothing or several times the record.

For it the take `shots` has a new key, `unit`: `"shells"` (the default) or `"tens"`, the sum divided by ten and rounded down once. No other take accepts the key. Tests: a hand-made batch in `crates/sim-cli/tests/validate.rs` (side and ships in tens, floored once after the sum; the default named; refusals of an unknown unit and of `unit` on another take) and a unit test in `take.rs`. `data/validation/README.md` and `docs/commands.md` say so.

The other measures of the three live files (Savo Island, the Eastern Solomons, Rabaul against Guadalcanal) were read for the same fault, a record of something that happened that a run doing nothing holds. No other has a positive record floored to nothing. These are held by a run in which the event never had the chance to happen, and are listed rather than fixed, since no record from rows would mend them:

- `night_fights_the_japanese_did_not_sight_first` (Savo Island, record 0): a run with no night fight holds it. `southern_group_rounds_surprised` (reported) shows such runs, at 0 against 1 to 2.
- `bomb_hits_on_saratoga` (the Eastern Solomons, record 0): the scenario puts both carrier forces in one hex and Enterprise is chosen first, so Saratoga's zero says nothing of how a target is chosen.
- `us_strike_on_the_japanese_fleet_carriers` (record 0): the groups whose launches found nothing are at rest by decision 0081 and cannot attack.

Rabaul against Guadalcanal has no measure with a record of 0.

### 10 and 12. Search orders, and Rabaul's total. Recorded as limits

**Search orders.** The Eastern Solomons search orders (decisions 0073, 0075 and 0081) were set from rows, but among readings chosen with the encounter known: the Japanese arc of 120 to 180 degrees holds the known bearing where another reading of the order gives about 100 to 150; the attack units keep an order that leaves them on deck; Enterprise flies the afternoon's search, the one that met the enemy. The distances and the removal of the launch no row has are independent of the runs; the exact arcs and which real sorties are left out are not.

**Rabaul's total.** Where the Rabaul total of bomber sorties agrees after its orders were set from the record (decision 0082, pull request 208), it agrees through daily errors that cancel, too many early and too few late. A total does not check fatigue, wear or how long the raids kept up, and the launches that turned back for weather are not flown.

No change for either.

### 11. The rate-of-fire gate. Accepted

The check of what is derived before what is tuned ("Tuning", 6) was Tenryu and Yubari's shells. It failed, and tuning went on, under the architect's ruling that the cause was the length of the fight. It is a miss, not a passed check, and the rates of fire are not independently certified by it: the pair's shells are their rate times how long they fire, and how long they fire moves with every figure that changes which ships survive. `docs/validation/baseline-reading.md` now says so.

### 13. The ditchings of Ryujo's raid. Accepted

Ruled: the ditchings of Ryujo's raid (B5N2s flown 200 miles on a derived radius of 175, from rows 79 and 412 of `aircraft_type_facts`, neither of which states a load; 412's block is headed B5N1) are not evidence for `ditch_chance_per_radius_beyond` and are left out of any round that tunes it. Fitting the shared figure to them would bend every type's ditching to one type's unsourced radius. Decision 0081 had called them evidence for that round; a dated note there points here, and its index entry and `docs/validation/baseline-reading.md` are corrected. A line is in `docs/step2-opening-list.md`.

### 14. Aoba's hits at Cape Esperance. Accepted in part: the record kept, called weak

Checked: row 1598 says "up to forty" and row 1599 "over 40". They do not overlap: one is at most 40, the other more than 40. A range that holds both runs from whatever bounds the low end of "up to" to whatever bounds the high end of "over", and no row bounds either. Row 1695 (Japanese Wikipedia, a lead only) says many of her hits were duds; 1510 and 1694 name places hit and no count. So the review is right that 40 is not a statement of any row, and the rows do not give the ends of a range. Furutaka's 90 is in the same form ("about 90", 1600; "over 90", 1601; "more than 90 above the waterline", 1602).

Ruled by the architect, Nov 13, 2026: the record of `gun_hits_on_japanese_cruisers` stays 130 to 134, judged. Its reason now says outright that 40 is where "up to forty" and "over 40" meet, that neither bounds its far end, and that Furutaka's 90 is the same, so the record is weak. Outside research is asked for bounded sources. If a row arrives before the check is run, the record changes; it does not change after the run. It is a limit of the check (below).

### 15. Decision 0081's count. Accepted

The headline read "after 9 agree (one at the edge) and 4 miss" of 14 judged; its table has 9 plain agreements, 1 at the edge and 4 misses. Corrected in 0081 to 10 agree (one at the edge), with a dated note.

## The limits of the checks, in one place

- **Cape Esperance** checks the surface figures on inputs partly set with the outcome known, with the US 6-inch rate of fire derived from that battle, and with no measure of how well either side shot for its shells (finding 1). Its record of hits on the Japanese cruisers is weak: Aoba's 40 and Furutaka's 90 are where an "up to" or "about" meets an "over", and no row bounds the far ends (finding 14).
- **A yes-or-no agreement says only that the record is not surprising.** The plainest case: `chicago_sunk` agrees by the rule while Chicago sinks in 560 runs of 1,000 and the median run sinks her. Japanese carriers sunk at the Eastern Solomons, and fights by day at Savo Island, are the same kind (`docs/validation/baseline-reading.md`).
- **Each measure is judged alone** (finding 3). No report shows one run holding several agreements together.
- **The fitted figures** (`surprised_target_factor`, `experience_gun_strength`, the two zero shares of 0079 and 0080) are fits inside a supported direction, not sizes from a source (findings 2, 5 and 6).
- **The rate-of-fire gate failed** and certified nothing (finding 11).
- **Measures held by a run in which the event had no chance to happen** (finding 9, the list).
- **Orders and readings chosen with the outcome known**: the search orders, Rabaul's raid days and searches, the radar flag (findings 7, 10 and 12).

## The reports

The round 2 batch of Savo Island and the baseline batch of the Eastern Solomons were made again on `main` with their labels, `r2` and `baseline`; validated against the measures as they were, they gave the committed reports' figures for every measure. Validated against the measures as changed, their reports replace `savo-island-r2.md` (judged 29: 21 agree, 2 at the edge, 6 miss) and `eastern-solomons-baseline.md` (judged 15: 10 agree, 1 at the edge, 4 miss). The Eastern Solomons report was remade again on `main` after decision 0079 landed, which moves its results: judged 15, 11 agree, 1 at the edge, 3 miss, bomb hits on Enterprise now agreeing. With 0079's live share at 0 every gun fires after the attack, so the measures of aircraft left to attack now count what the patrol leaves: the Japanese bombers left go from a median of 1 to 11. The figures of round 2's tuning record are those of the round as it was run, on the measures of the time.

## Rule card

No rule. Measures, and the `unit` key of the take `shots` in `sim-cli validate`, which reads the batch file and nothing that plays the game.
