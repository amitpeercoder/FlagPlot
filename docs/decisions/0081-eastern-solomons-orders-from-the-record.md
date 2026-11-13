# 0081: The Eastern Solomons scenario's remaining orders, from the record

Status: Decided by the architect on the designer's delegation, Nov 13, 2026. Scenario data, two measures and one new way of taking a measure; no rule changed and nothing under `crates/sim-core` changed. It comes before any air figure is tuned (`docs/step2-spec.md`, "Tuning", 8: a scenario's orders are set from rows before the first round of its battle and then left alone). It follows decisions 0031, 0063 ("As built (the scenario orders)") and 0073, and replaces their orders where it says so.

## The gap

The air tuning plan of Nov 13, 2026 ran 1,000 runs of the Eastern Solomons and traced each miss to its cause. Three of the scenario's orders were not the record's, and two agreements rested on one of them:

- **The Japanese strike was doubled.** Each fleet carrier's dive bomber unit was one group on naval strike, so all 54 dive bombers went at once. The record has a first wave of 27 with 10 fighters that attacked, and a second wave of 27 with 9 that launched about an hour later and found nothing. The patrol and the guns, which fire before the bombs fall, cut the 54 about in half, and the survivors made `japanese_aircraft_attacking_enterprise_force` (35 against 37) and `bomb_hits_on_enterprise` (4 against 3) agree. With the first wave alone, on a scratch copy, the attackers fell to 6 in the middle run and the bomb hits to 0.
- **Enterprise's VB-6 and VT-3 were on naval strike,** where the record has them on the afternoon search, and their strike group of 17:08 found nothing.
- **Henderson's VMSB-232 struck Ryujo,** which no row has it do.

## The reading

**An air group in a scenario is a launch the record has, where the forms or rows give launches apart.** A unit whose aircraft the record sends out in two launches of their own, each of a size the rules can fly, is two air groups; a unit the rows do not separate stays whole, as its strength rows give it. The reading is for every battle, the check battles before they are run, as the break-off reading of decision 0024 is. Where a launch did something the rules cannot order (it launched and found nothing, or went at a force the scenario does not hold), the group gets the order whose effect over the day is the record's, and the scenario file says beside the order that it is a limit of the rules, not the record (decision 0073 did this for the attack units).

## The rows, and each order

All rows are `data/oob/engagement_outcomes.csv` unless named otherwise. "Events" is `engagement_events.csv`, "units" is `air_unit_facts.csv`.

| Group | What | Ready | Order | Rows |
| --- | --- | --- | --- | --- |
| 12 | Shokaku's dive bombers, first wave | 18 (was 27) | Naval strike, list carrier, battleship, cruiser, transport, as before | 1159 (her form: 18 dive bombers, 4 fighters), 290; units 228 (27) |
| 15 | Zuikaku's dive bombers, first wave | 9 (was 27) | As group 12 | 1164 (her form: 9 and 6), 290; units 232 |
| 21 | Shokaku's dive bombers, second wave | 9, new | Rest: a limit of the rules | 1163 (her form: 9 and 3, no enemy found, bombs jettisoned), 409 |
| 22 | Zuikaku's dive bombers, second wave | 18, new | Rest: a limit of the rules | 1169 (her form: 18 and 6, launched 14:00 Tokyo time, found no target), 409 |
| 11 | Shokaku's fighters, first wave escort and patrol | 23 (was 26) | Patrol, share 0.83 (was 0.8): 23 x 0.83 = 19.09, 19 patrol and 4 escort | 1159; units 227 (26) |
| 14 | Zuikaku's fighters, first wave escort and patrol | 19 (was 25) | Patrol, share 0.7 (was 0.8): 19 x 0.7 = 13.3, 13 patrol and 6 escort | 1164; units 231 (25) |
| 23 | Shokaku's escort of the second wave | 3, new | Rest | 1163 |
| 24 | Zuikaku's escort of the second wave | 6, new | Rest | 1169 |
| 3 | VS-5 | 16 | Search 290 to 090 to 250 miles (was 200) | Events 158 (her report: 23 planes, 290 to 090, 250), 157 (the Navy narrative: 270 to 090, 250); 222; units 216 |
| 4 | VT-3, the seven of the afternoon search | 7 (was 14) | Search 290 to 090 to 250 (was naval strike) | Units 253 (7 VTB on the search, the war diary's flight 395), 218 (14 operational); 369 |
| 25 | VT-3, the seven of the 17:08 attack group | 7, new | Rest: a limit of the rules | 226 (7 F4F, 11 SBD, 7 TBF), 378 (found nothing) |
| 2 | VB-6 | 17 | Rest (was naval strike): a limit of the rules | 226, 378; units 214 |
| 6, 7 | VB-3, VS-3 | 15, 15 | Naval strike, as before | 231 (30 SBD and 8 TBF launched), 333, 1238 (29 and 7 reached); units 209, 151, 210 |
| 8 | VT-8, the eight of the strike on Ryujo | 8 (was 13) | Naval strike, as before | 231, 335; units 177, 211 (13) |
| 26 | VT-8, the five of Saratoga's second group | 5, new | Rest: a limit of the rules | 233 (2 SBD of VB-3 and 5 TBF of VT-8 at about 17:08), 359 |
| 10 | VMSB-232 | 11 | Rest (was naval strike) | 317, 415 (the night of the 24th, at the bombarding destroyers); 314 to 316, 418 (the convoy on the 25th) |

The fleet carriers' attack units (13, 16), Ryujo's groups (18, 19), the floatplanes (17, 20), the American fighters (1, 5, 9) and VMF-223 keep the orders of decisions 0063 and 0073. New group numbers come after every number the scenario had (21 to 26; 121 to 126 in scenarios 7 and 8), so no number changed its meaning.

### Where the rows separate launches, and where they do not

- **The Japanese dive bombers:** two launches each, sized by the carriers' own forms (1159, 1163, 1164, 1169). Split. The plan read Shokaku 18 and Zuikaku 9 in the first wave; the forms say so, and 290 sums the wave to 27 and 10. The units' totals (27 each, units 228 and 232) are the two waves together.
- **The Japanese fighters:** the escorts of each wave are on the forms (4 and 6 with the first, 3 and 6 with the second). The second wave's escorts are a group each, at rest. The first wave's escorts are the share not on patrol, so the share is set to give the forms' 4 and 6 from what is left, as 0.35 was set for Ryujo from row 1171. The 0.8 it replaces was decision 0031's "about ten of fifty", a reading the forms now make exact. No row gives the Japanese patrol over the carriers; it is what is left.
- **Ryujo:** her raid launched in two parts, at 10:20 (6 and 6) and 10:48 (9) Tokyo time (1171's note, the 8th Sentai table), and attacked as one; the rules fly it as one raid, so the units stay whole. Her patrol is as decision 0063 has it.
- **Enterprise:** the afternoon search was 16 SBDs and 7 TBFs (units 252, 253); the 17:08 attack group 11 SBDs and 7 TBFs (226). VT-3's 14 operational (units 218) split 7 and 7, as the rows give it. The SBDs of VB-6 (17) and VS-5 (16) flew in both launches, squadrons mixed: no row says how many of each searched (VB-6 crews are named among the searchers, 368; the dawn search of 20 SBDs, 144, was more than VS-5's 16). So the SBDs are not split: VS-5 (16) searches, standing for the 16 SBDs of both squadrons that flew the search, and VB-6 (17) rests, standing for the attack group whose 11 SBDs were hers. The search is 16 + 7 = 23, the record's (157, 158, 222).
- **Enterprise's fighters:** the seven F4Fs of the 17:08 group did not go (226); the fighters are one launch, the patrol, and stay whole.
- **Saratoga:** VT-8's 13 flew as 8 in the strike on Ryujo (231) and 5 in the second group (233). Split. The SBDs: 30 in the strike, which VB-3's 15 operational and VS-3's 15 make (units 209, 210), and VB-3's two more in the second group (233). Those two stay in VB-3: no row makes them a launch of their own of a size the rules could fly (a group of 2 at rest would change nothing but the count aboard), and VB-3's strength row gives 15 operational of 17 on hand, so the two are outside its operational count already.
- **Henderson Field:** VMSB-232 flew neither launch the scenario holds; VMF-223 is one launch, the patrol.

### Enterprise's search: which search, and to where

A standing search flies every daylight pulse with one arc. The day had two: at dawn 20 SBDs, 290 to 070, to 200 miles (144, 146), which found nothing; and at 13:09 the 23 aircraft above, 290 to 090 (158; 157 has 270 to 090), to 250. The scenario takes the afternoon's, the search that met the enemy, with its arc from the ship's own report (decision 0073 took the same arc). 250 is within the SBD-3's search radius of 397 and the TBF-1's of 420 (decision 0075). In the morning pulse the order then reaches 50 miles beyond the dawn search; that is the price of one order a day, named here. Decision 0073's item 4 (VS-5 at 200, a blend of the two searches) is replaced.

## What the rules cannot express

Each is named beside its order in the scenario files.

- **A launch that finds nothing.** The second wave (21 to 24) launched and found nothing; Enterprise's attack group (2, 25) launched and found nothing. The rules launch a strike at a contact in reach and send it to the contact's hex; they cannot order a strike that is sent and misses. Rest has the record's effect over the day: no attack. The rows of these launches stay in the record as what the scenario does not fly.
- **What it costs, for the second wave.** In the record its 36 aircraft were away from their carriers from 16:00 (scenario time); here they are aboard all day. If Shokaku or Zuikaku is sunk, they go down with her (`lost_with_ship`), where in the record they would have landed elsewhere or ditched; a hit that puts her flight deck out leaves them aboard, where in the record they would have had nowhere to land. Their losses after dark (293, 5 dive bombers; 1170) are outside the rules and the scenario's 18:00 end, as before. In the 1,000 runs below no Japanese fleet carrier is sunk, so the cost is not paid there.
- **A search that attacks what it finds.** Two VB-6 searchers bombed a carrier and two a cruiser; two of VT-3's bombed Ryujo, and a third was shot down by her fighters (368, 369, 372, 375). A search group never attacks and never meets fighters in the rules.
- **Part of a group.** A group searches and strikes whole. The reading above is how the scenario says "part of a unit".
- **A launch at a force the scenario does not hold.** Saratoga's second group (26) went at Kondo's force and Chitose (233, 359); VMSB-232 at the bombarding destroyers and the convoy (317, 415, 314 to 316). Rest.
- **A mechanical abort.** One SBD and one TBF of the strike on Ryujo turned back (231, 333); the rules have no abort. The measure's record spans it (below).

## The measures

Ruled by the architect, who now owns `data/validation`; the file's rules kept (no recorded value without its rows; a range spans a dispute or, by the README's rule, a part of the record out of reach).

- **M1, `japanese_aircraft_attacking_enterprise_force`:** counts the wave that took part, as its record does (290, 1159, 1164): the aircraft of the strike and its escort that reach the target hex, before the patrol and the guns fire, not the survivors. Record 37, as before. No existing take did this (`aircraft_attacking` counts the bombers that attack after the fight), so a new take was built, `aircraft_taking_part`, with batch version 5 (decision 0062, "As built (version 5)"): the batch records every strike launch with its aircraft, hex, whether it found nothing and the task force its raid met. A raid whose every bomber the patrol shoots down before the guns fire names no force in the log and is not counted: 4 runs of 1,000 below.
- **M2, `us_aircraft_attacking_ryujo`:** the same take; record 36 to 38. 38 launched (231), 36 reached (333, 1238). The rows do not dispute it: the two that turned back are a part of the record out of reach of the rules, so the record runs from what reached to what launched. Row 232 (37: 31 SBD and 6 TBF), first read as a dispute, is the strike of 23 August and is not read.
- **The new groups** are in the `air_groups` lists of every measure whose record counts those aircraft: 21 to 24 in both Japanese loss measures and in the American claims measure (the second wave lost nothing to the enemy; its 5 lost after dark are already out of the all-causes record), 25 and 26 in both American loss measures and the Japanese claims measure (the record's carrier losses are of every group; Saratoga's second group lost 2, 367).
- **M6:** both claims-per-hundred measures say they are read only after the fight's rounds, since they divide by the kills.
- M7 (`us_fighters_lost_to_enemy_aircraft` at Rabaul counts the bombers' return fire) is a note for the Rabaul measures and changes nothing here.

## C5: the B5N2's radius for Ryujo's raid

Ryujo's six attack planes flew 200 to 202 miles with bombs to Henderson Field and back (events 331, positions 310) with no fuel loss on record (1172: 3 self-destroyed, 2 missing; the rest landed beside her escorts because she could not take them, 699). The derived strike radius is 175: a third of `aircraft_type_facts` row 79, 528 miles. The plan said row 79 reads "528 miles with a torpedo"; it does not. Row 79 (Wikipedia, low, unconfirmed) and row 412 (pwencycl, low) give 528 with no load stated ("normal range"); "with a torpedo" is in the reason of the exception in `data/derivation/rules.toml`, which says more than the row. The rule (decision 0029, item 6) is a sourced combat radius where exactly one can be read, else a third of the range with a normal load. In the runs, at 200 miles on extended reach (218), each B5N2 that comes back ditches at 2 x (200 - 175) / 175 = 0.285714: 1.72 of 6 a run, before and after this record.

1. **Keep 175.** The rule is applied as written: no row states a range by load, so the normal range is read. Mend only the exception's reason, which should say "load not stated". The ditching stays, about 1.7 a run, and `ditch_chance_per_radius_beyond` (round 2) is the only thing that can touch it, which would bend a figure all types share to fit one derived radius. Not recommended for that reason.
2. **A radius by load, where rows state the load.** No B5N2 row states a range or radius with bombs or with a torpedo, so the rule cannot be applied to it today. Commission the row: a Japanese or TAIC statement of the Type 97's radius or range with a bomb load. Until it comes, 1 holds.
3. **The distance flown as the sourced radius,** as the derivation already does for the SBD-3 (200, `aircraft_type_facts` row 280, read by exception: "the distance of the morning search and about that of Saratoga's strike") and the TBF-1 (190, row 310: "flown with a torpedo by Saratoga's strike"). For the B5N2 that is 202 with bombs (events 331), a new `combat_radius_nm` row citing the raid, read by an exception with that reason. Worked: radius 202, so the raid at 200 is within it and nothing is rolled; the 1.72 ditched a run go to 0 (Japanese losses to all causes fall by about that: 30 in the middle run against 35 to 54, still agreeing), and extended reach is no longer needed for the raid (the order may stay; it changes nothing within the radius). The attack units 13 and 16, also B5N2s, reach 202 instead of 175; their contacts at Enterprise are 223 miles away in the afternoon, so on this scenario's positions they still launch nothing, but the full slice and the soak must be checked.

The lead recommended 3. **Ruled by the architect on Nov 13, 2026: option 1, with research commissioned.** The B5N2's strike radius stays 175, a third of the 528 miles of rows 79 and 412. A distance flown is not taken as a radius: decision 0075 refused that for the search radius, and one raid flown once is a ceiling case, not a radius. The SBD-3's and TBF-1's exceptions that do take a distance flown (`aircraft_type_facts` 280 and 310) are an inconsistency of the derivation, to be looked at with the fuel round, not followed. The one change made here in `data/derivation`: the exception's reason now says what the rows say ("528 miles, the normal range, with no load stated; two weak sources that agree (rows 79 and 412)"); the Eastern Solomons aircraft file was derived again and only its header changed. Ryujo's raid stays on extended reach at 200 miles. Its 1.72 aircraft ditched a run, against none in the record (1172; events 331), is evidence for the round of the fuel figure, `ditch_chance_per_radius_beyond`, where it is judged. A radius with bombs is asked of outside research (option 2).

**Note, Nov 13, 2026: decision 0084 reverses one ruling of this paragraph.** The ditchings of Ryujo's raid are not evidence for `ditch_chance_per_radius_beyond` and are left out of any round that tunes it: they rest on a derived radius whose rows state no load, and fitting the shared figure to them would bend every type's ditching to one type's unsourced radius, the objection option 1 itself makes. The text above stands as it was ruled.

## What was seen before, and that the orders rest on the rows

The plan's scratch variants (first wave alone; groups 2, 4 and 10 at rest; both) showed what the orders do before any was written: with the first wave alone the bomb hits on Enterprise fall to 0 in the middle run, and with Enterprise's groups and VMSB-232 at rest Ryujo sinks about half as often. Neither was a reason for any order: each order above is the rows', by the reading, and would be the same had the scratch runs shown the opposite. The two findings in the rows that changed a ruling (row 232 is the 23rd's strike; VT-8 and VT-3 were two launches) were taken back to the architect before the orders were written.

## What 1,000 runs show

Seeds 1 to 1000, live figures, `validate` against `data/validation/eastern_solomons.toml` as changed. Before is main at cdae5bc (its scenario, the same binary, the new measures less the new group ids, which that scenario lacks). The report of the after batch is `docs/validation/eastern-solomons-baseline.md`.

| Measure | Record | Before: 5th to 95th, median, runs holding, verdict | After |
| --- | --- | --- | --- |
| japanese_carriers_sunk | 1 | 0 to 1, 0, 464, agrees | 0 to 1, 0, 228, agrees at the edge |
| us_carriers_sunk | 0 | 0 to 0, 0, 1000, agrees | the same |
| bomb_hits_on_enterprise | 3 | 0 to 9, 4, 63, agrees | 0 to 2, 0, 29, miss |
| bomb_hits_on_saratoga | 0 | 0 to 0, 0, 1000, agrees | the same |
| enterprise_flight_deck_out_at_end | 0 | 0 to 1, 0, 846, agrees | 0 to 0, 0, 996, agrees |
| japanese_carrier_aircraft_lost_to_enemy_action | 29 to 32 | 2 to 40, 28, 186, agrees | 2 to 35, 26, 234, agrees |
| japanese_carrier_aircraft_lost_all_causes | 35 to 54 | 4 to 62, 31, 213, agrees | 4 to 58, 30, 195, agrees |
| ryujo_aircraft_lost_with_the_ship | 4 | 0 to 0, 0, 0, reported | the same |
| us_aircraft_lost_to_enemy_action | 13 to 14 | 1 to 11, 5, 14, miss | 1 to 9, 4, 2, miss |
| us_aircraft_lost_all_causes | 20 to 21 | 1 to 12, 6, 0, miss | 1 to 9, 5, 0, miss |
| japanese_aircraft_attacking_enterprise_force | 37 | 0 to 65, 65, 0, miss | 0 to 37, 37, 607, agrees |
| us_aircraft_attacking_ryujo | 36 to 38 | 0 to 85, 0, 0, miss | 0 to 38, 0, 488, agrees |
| us_strike_on_the_japanese_fleet_carriers | 0 | 0 to 0, 0, 1000, agrees | the same |
| us_aircraft_claimed_per_hundred_shot_down | 237 to 362 | 100 to 200, 144, 9 of 997, miss | 100 to 200, 148, 9 of 997, miss |
| japanese_aircraft_claimed_per_hundred_shot_down | 171 to 261 | 100 to 300, 166, 371 of 982, agrees | 100 to 300, 150, 298 of 980, agrees |
| us_hits_claimed_per_hundred_scored | 50 to 140 | 115 to 191, 144, 193 of 469, reported | 100 to 200, 140, 259 of 488, reported |

Judged 14: before 9 agree and 5 miss on the measures as changed; after 10 agree (one at the edge) and 4 miss. (Corrected Nov 13, 2026, by decision 0084: this line had read "after 9 agree (one at the edge)", which with 4 misses makes 13; the table has 9 plain agreements, 1 at the edge and 4 misses.)

- **The finding: with the true first wave, the bombs on Enterprise miss.** 0.50 bomb hits a run against 3.52 before, 3 in the record. The Japanese raids met the American force in 611 runs before and after; their bombers reached the attack in 508 runs after, against 611. The patrol of 54 and the guns, all firing before any bomb falls, leave few of 27. The plan traced this to the order of the fight (the guns fire before the dive, where the record has them open fire as the dives began) and to whom the patrol fires at; decisions 0079 and 0080, built by another team, answer it. Nothing here was adjusted for it.
- **The two strike measures agree on what took part.** M1 holds 37 in 607 runs (611 met the force; 4 are the raids the patrol wiped out before the guns, which name no force). Taken the old way, by the bombers left after the fight, it would hold 37 in no run (5 in the middle run, 0 to 17). M2 holds 38 in the 488 runs in which Saratoga's strike reaches Ryujo, and 0 in the others: the strike is still all or nothing on one sighting and one hex.
- **Ryujo is sunk half as often** (228 against 464; crippled in 234 more), since Enterprise's groups and VMSB-232 no longer strike her. Japanese carriers sunk agrees at the edge.
- **Ryujo's attack planes still ditch** 1.72 a run (C5: evidence for the fuel figure's round; reversed by decision 0084, which leaves them out of that round).
- **The result:** a stalemate in 547 runs, an Allied victory in 450, a defeat in 3 (before 436, 468, 96).

## Who follows

Scenarios 7 and 8 copy scenario 5's day on day 4, with group numbers plus 100 (decisions 0063 and 0073): groups 104, 108, 111, 112, 114 and 115 take the new sizes, 121 to 126 are added at rest, 103 and 104 search to 250, 102 rests, and the shares of 111 and 114 are 0.83 and 0.7. VMSB-232 there is scenario 6's group 11, on naval strike from day 1 by that scenario's orders, not a copy of scenario 5's day, so it is left as it is (scenario 5 is one day; there is no day 2 to set). In both, the orders stand after day 4: groups 102, 121 to 126 rest and 103 and 104 search for the rest of the run. No order of scenario 8's later parts names the changed groups. The frozen copies in `data/scenarios/step2-frozen/` are not touched; every test on the frozen figures reads them, and none moved.

## Rule card

No rule. Scenario inputs, and a take of `validate` that reads the log.

## As built

Built Nov 13, 2026, on branch `eastern-solomons-orders`. Nothing under `crates/sim-core` changed.

- The orders, the sizes and their reasons are in `data/scenarios/05_eastern_solomons.toml`, `07_full_slice.toml` and `08_soak.toml`.
- `sim-cli batch` writes version 5 with each run's `launches`; `validate` reads version 5 only and has the take `aircraft_taking_part`; `air_losses` no longer has a row of zeros for a group that lost nothing (a fuel roll that ditched none made one). Decision 0062, "As built (version 5)", `data/validation/README.md` and `docs/commands.md` say so. Tests: a hand-made batch of three runs (37, 15 and 0 with escorts; 27, 9 and 0 without) in `crates/sim-cli/tests/validate.rs`, the refusal of a version 4 file, and the launches checked against the event log in `tests/batch.rs`.
- `data/validation/eastern_solomons.toml`: M1, M2, the group lists and the claims reasons as above.
- Remade because they follow live data, in commits of their own: the run baselines of scenarios 5, 7 and 8 (no other changed); the pinned day-1 hash of scenario 5 in `tests/scenario_10_objectives.rs` (2ca5a5dd9f150a5d to 08c55b5f26162af0); the batch test's count of scenario 5's air groups (20 to 26); the three recorded games. All three transcripts changed, with no answer to `issue_orders` different: the briefings list the new groups and the smaller VT-3 and VT-8, and the fighting after differs. In the Eastern Solomons game played by Haiku the result went from a minor defeat to a stalemate (the Japanese no longer cripple Enterprise); in the one played by Sonnet from a minor to a major victory; in the full slice game the day 4 strike flies with 7 and 8 torpedo planes, Enterprise's search to 250 reports Ryujo at (7,-9) as a carrier where it had reported (7,-10) as a cruiser, and the `measure` answers for that contact change with it. `crates/sim-cli/tests/recorded_games/README.md` says so. No hand-calculation test, golden of the frozen figures or seed-pinned test moved.
- **Merged with main after pull request 206** (the strike report reads the force after the damage). The run baselines of 5, 7 and 8, the pinned hash and the recorded games were remade on the merged code. Scenario 5's day-1 hash on seed 1942 is still 08c55b5f26162af0 (Ryujo is not sunk that day). In 7 and 8 and in two recorded games only the reports of the Ryujo detachment made after she sinks changed (they count the ships left afloat). The 1,000-run batches before and after were made again on the merged code: no measure's percentiles, median, runs holding or verdict moved, before or after; the report was remade for its header's hashes only.
- **C5, as ruled:** the B5N2 exception's reason corrected in `data/derivation/rules.toml`, `aircraft_types_eastern_solomons.csv` derived again (header only), and the recorded games' saves remade for the new file hash (their transcripts did not change). No run baseline changed.
