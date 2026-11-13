# Step 2: the close

Nov 13, 2026. Assembled from the records by the architect's team; it adds no finding of its own. Each number here is copied from the report or round record named beside it, and that file is the source.

## Where step 2 ends, and the one decision asked of you

Step 2 built the harness, measured the slice's battles against the record, tuned what the sources allowed, and checked the surface figures once on a battle they were not tuned on. No battle passes outright. Across the four battles 62 measures are judged: 43 agree, 8 agree only at the edge, and 11 miss. Every miss and every edge agreement is listed below with its cause and with what in step 3 would answer it. **The one decision asked of you: accept the named misses, or say which one is not acceptable.** The same goes for the one closing test that did not pass its own rule, the legibility test, which missed by one prediction (below). With it come three smaller decisions the spec left for the close, each with the architect's recommendation.

## "Done means", line by line

| Line of the spec | Met? | Shown by |
| --- | --- | --- |
| All three battles pass, or each miss is written down with its cause and accepted by the designer | **Not met until you decide.** No battle passes. Every miss is written down with its cause; your acceptance is the decision asked above. <!-- GAP: the cause of Rabaul's edge agreement on Japanese fighters lost, from `final-reports`. --> | This page; the four reports in the table below |
| No figure in `data/reference` is a placeholder without a note saying what it rests on | **Met.** A note above every figure in its file (pull requests 226 and 227), and two summary pages list them. Outside `data/reference` and so outside the line: scenario 6's own figures, placeholders with notes in the scenario file and on the air page; and the objectives scale's points in the scenario files, whose comment still says "to be set from the record in step 2" (left so by the architect's ruling, `docs/step2-opening-list.md`) | `docs/validation/tuning/air-figures-closed.md`, `docs/validation/tuning/surface-and-search-figures-closed.md` |
| The frozen-figure tests, the invariants, the determinism scenario and the soak still pass | **Met.** The frozen-figure tests, the invariants and the determinism scenario run in `cargo test` on every pull request. The hundred-seed soak, which CI does not run, was run by hand on Nov 13, 2026: 100 seeds of 180 days, 1,978,115 events, every check holding; the determinism scenario 9 of 9 and the invariants' tests 24 of 24 beside it | `docs/validation/closing-tests.md`, test 1 |
| The scenario with every rule of step 2 acting together passes, and was built before tuning began | **Met.** Built Nov 13, 2026, before any figure was tuned; it runs in `cargo test` | `tests/scenario_16_all_together.rs`; `docs/step2-opening-list.md`, "Done" |
| The legibility test is run again on the tuned Eastern Solomons | **Not met by its own rule.** The explanation, claims against truth, and truth and belief pass; two of four predictions pass where three are needed. Both misses are about where the enemy would be, which the opening briefing cannot show. Not run again, and the rule not changed after the result. For you to accept or not (below) | `docs/validation/closing-tests.md`, test 2 |
| A cheap model given only the rulebook, the glossary, the data files and two scenarios reproduces a set of hand calculations exactly | **Met after a rerun, not at the first sitting.** First run: every number of the step 2 rules exact, but two derived chances cut where the hand calculations round; in the step 1 control one number wrong, on a term the rulebook did not define ("free round", "both fire from the start"), and two chances cut. The architect then ruled that such chances are written cut (R6), and the rulebook and glossary were fixed. The control sitting, run again on the fixed book, got every number exact. One new finding (R12) is open | `docs/validation/closing-tests.md`, test 3 |

## The four battles

Each report is 1,000 runs, seeds 1 to 1000. "At the edge" is an agreement whose record has no part in the middle half of the runs.

| Battle | Judged | Agree | At the edge | Miss | Report | What it is |
| --- | --- | --- | --- | --- | --- | --- |
| Savo Island | 29 | 21 | 4 | 4 | `savo-island-after-0083.md` | The closing report after decision 0083. Tuned on |
| Eastern Solomons | 15 | 11 | 2 | 2 | `eastern-solomons-r4.md` | The last remade report, air round 4 (claims). Rounds 3 and 5 and the Rabaul rounds made no new one for this battle. Tuned on |
| Rabaul against Guadalcanal | 10 | 7 | 1 | 2 | `rabaul-guadalcanal-air-rabaul-r1.md` | The last remade report, Rabaul round 1. Rounds 2 and 3 moved nothing and made none (`rabaul-r3-wear.md` says so). Air figures checked on it; slow figures tuned on it |
| Cape Esperance | 8 | 4 | 1 | 3 | `cape-esperance-check.md` | Checked once, never tuned on |
| **All four** | **62** | **43** | **8** | **11** | | |

The three tuning reports were made on state version 7; the Cape Esperance check on version 8. The engine moved to version 8 with decision 0086 after the three were made, and they were not remade.

## Every named miss and every agreement at the edge

Figures are the 5th to the 95th percentile, then the median, then (in brackets) the runs of 1,000 that hold the record. Misses that share one cause are grouped.

### Savo Island (`savo-island-after-0083.md`; causes from `tuning/savo-island-closed.md`)

**The two fights overlapped in the record, and the rules fight them one after the other.** In the record the Japanese column was on the northern group before it had done with the southern one.

| Measure | Verdict | Record | Runs | Cause in one line | Recorded in |
| --- | --- | --- | --- | --- | --- |
| `gun_hits_on_vincennes` | miss | 57 to 74 | 4 to 44, 27 (1) | Overlap, and the record counts every calibre while the rules fire main batteries only | Decisions 0083 ("Fights that overlap"), 0084 (3) |
| `gun_hits_on_quincy` | edge | 36 to 54 | 1 to 41, 24 (156) | As for Vincennes | The same |
| `torpedo_hits_on_the_northern_group` | edge | 3 to 6 | 0 to 3, 0 (93) | The second side of the tubes goes to whatever a ship fights next, mostly the southern fight; in the record the second sides went north | Decision 0083, "As built", "Fights that overlap" |
| `japanese_main_battery_shells_fired_in_hundreds` | edge | 10 to 11 | 7 to 19, 14 (101) | The Japanese fire through every round of both fights | Decision 0083 |
| `tenryu_and_yubari_main_battery_shells_fired_in_hundreds` | miss | 1 | 2 to 4, 3 (35) | The pair fire about 6.6 rounds each against about 3: they fired for part of the night, and the rules have them fire in every round. The rate-of-fire check failed and certified no rate | Decision 0084 (11); `savo-island-closed.md`, "The watch" |

Also resting on it, though it agrees: `chicago_sunk` agrees only as a yes-or-no figure; Chicago sinks in 565 runs of 1,000.

**Step 3:** one fight for three forces, so that fights can overlap. Decision 0083 records it as a plan for you at the close, not built.

**The secondary guns.** A cruiser's secondary battery has no counterpart in the rules; at Savo Island it fired about a third of the Japanese shells (654 of 1,867).

| Measure | Verdict | Record | Runs | Cause | Recorded in |
| --- | --- | --- | --- | --- | --- |
| `gun_hits_on_allied_ships` | edge | 153 to 219 | 43 to 160, 109 (86) | Every calibre in the record against main batteries in the rules | The measure's reason; decision 0084 (3) |

Vincennes's and Quincy's hits above rest on it too. **Step 3:** secondary guns.

**Ships that take part where the record has them holding fire.**

| Measure | Verdict | Record | Runs | Cause | Recorded in |
| --- | --- | --- | --- | --- | --- |
| `allied_torpedoes_fired` | miss | 4 to 6 | 16 to 44, 26 (0) | Helm and Wilson fire 16 between them in nearly every run; neither fired (Wilson's order not received, Helm's target friendly; `engagement_outcomes` 3217 to 3219). It is participation, not the torpedo share | `savo-island-closed.md`; decision 0071 |
| `southern_group_main_battery_shells_fired_in_tens` | miss | 3 to 5 | 0 to 47, 9 (25) | The group fires nothing, or several times the record: one round at half manning is already more than any ship of the group fired all night | Decision 0084 (9); `baseline-reading.md` |

**Step 3:** which ships of a force fire at all, and for how long. Cape Esperance shows the same pattern (below).

### Eastern Solomons (`eastern-solomons-r4.md`)

**Bombers' return fire, and three other missing air rules.** Ruled named misses on Nov 13, 2026.

| Measure | Verdict | Record | Runs | Cause | Recorded in |
| --- | --- | --- | --- | --- | --- |
| `us_aircraft_lost_to_enemy_action` | miss | 13 to 14 | 1 to 9, 4 (2) | No return fire from bombers; a search never meets fighters; no aircraft written off for combat damage | `tuning/r3-us-losses-named-misses.md` |
| `us_aircraft_lost_all_causes` | miss | 20 to 21 | 1 to 11, 5 (0) | The same, and a carrier's deck closed for a time (65 minutes on the day), with its aircraft aloft running out of fuel | The same |
| `japanese_aircraft_claimed_per_hundred_shot_down` | edge | 171 to 261 | 250 to 1,100, 440 (67 of 980) | It divides by the American losses above, which are too few. The Japanese claims themselves fit their rows | `tuning/r4-claims.md` |

**Step 3:** bombers' return fire; a search that meets enemy fighters; an aircraft written off for combat damage; a deck closed for a time.

**A yes-or-no measure.**

| Measure | Verdict | Record | Runs | Cause | Recorded in |
| --- | --- | --- | --- | --- | --- |
| `japanese_carriers_sunk` | edge | 1 | 0 to 1, 0 (249) | Ryujo is sunk in about a quarter of the runs; the strike on her flies whole or not at all. A yes-or-no agreement says only that the record is not surprising | `baseline-reading.md` (Eastern Solomons, Nov 13); decision 0084, "The limits of the checks" |

### Rabaul against Guadalcanal (`rabaul-guadalcanal-air-rabaul-r1.md`)

**No height-and-warning rule at Henderson Field.**

| Measure | Verdict | Record | Runs | Cause | Recorded in |
| --- | --- | --- | --- | --- | --- |
| `japanese_bombers_lost_to_enemy_action` | miss | 12 | 21 to 37, 29 (0) | The patrol fought high-level bombers after the coastwatchers' warning; no rule has height or warning, and no row counts the fighters that made contact (the air plan's C3) | `tuning/r3-fighter-hit-chance.md` |
| `japanese_fighters_lost_to_enemy_action` | edge | 6 to 10 | 8 to 19, 13 (187) | **No record names a cause.** It went from agreeing (median 12, `rabaul-guadalcanal-air-r4.md`) to the edge (13) in Rabaul round 1, which cut the aircraft destroyed on the ground so that more American fighters fly; that round's record does not say this measure moved for it | None |

**Step 3:** height and warning for a field's patrol.

**Aircraft not worn down as the record's were.**

| Measure | Verdict | Record | Runs | Cause | Recorded in |
| --- | --- | --- | --- | --- | --- |
| `sbds_ready_on_day_21` | miss | 22 | 27 to 33, 30 (0) | Aircraft under repair for days (5 of 31 on 25 August, row 455, against about 1.3 in the rules); strikes at ships that scenario 6 cannot fly; losses on take-off, landing and ferrying that do not go with distance | `tuning/rabaul-r3-wear.md`, handed on from `rabaul-r1-base-hit-aircraft.md` |

**Step 3:** small airfields that wear aircraft faster, and wear that does not rise in a straight line with distance.

**Named, and not judged.** These are not in the count above.

| What | Record | Runs | Cause | Recorded in |
| --- | --- | --- | --- | --- |
| Raid sizes by day | 23 on day 5; 20 and 16 on days 23 and 24 | 40 on day 5; almost nothing on days 23 and 24 | The units chose how many to send, about 18 a raid whatever their rest; in the rules a group sends all it can fly. Total sorties agree (212 against 211 to 213) only because the late raids collapse | `tuning/rabaul-r2-fatigue.md` |
| F4F write-offs | about 6.1 | 8.2 a run | Wear's shape, looked at and not moved | `rabaul-r3-wear.md` |
| Japanese bombers forced down (reported) | 5 to 6 | 10 | A straight line in distance cannot fit both a 400-mile search and a 1,112-mile raid | `rabaul-r3-wear.md` |
| Japanese claims on the ground (reported) | 2,066 to 4,600 for each hundred | 900 | The record's ratio is over two or three aircraft | `rabaul-r1-base-hit-aircraft.md` |
| Rabaul escorts' claims (no measure) | about 100 | about 159 a run | One shared claim chance for both sides | `tuning/r4-claims.md` |
| Marine fighters' claims at the Eastern Solomons (no measure) | 16 to 21 | out of reach | Thirteen fighters fire one shot each in the rules | `tuning/r4-claims.md` |

**Step 3:** a strike of a given size, so a staff can send part of a group.

### Cape Esperance (`cape-esperance-check.md`; causes from its section of `baseline-reading.md`, marked there as conjecture for step 3)

| Measure | Verdict | Record | Runs | Cause | What in step 3 would answer it |
| --- | --- | --- | --- | --- | --- |
| `us_destroyers_sunk` | miss | 1 | 0 to 0, 0 (4) | Duncan left the column to attack between the lines and was hit by both sides; the rules have no fire on a friend and no ship acting apart | Fire on a friend; a ship acting apart from its force |
| `japanese_ships_claimed_sunk_per_hundred_sunk` | miss | 200 to 300 | 100 to 300, 100 (2 of 44; 956 left out) | Too few runs sink a US ship to keep: the same cause as Duncan | The same |
| `gun_hits_on_aoba` | miss | 40 | 13 to 55, 29 (18) | A weak point record (decision 0084, 14); a short night fight (a median of 3 rounds against about 34 minutes of fire); the rule of targets spreading fire evenly; no radar fire control | Radar fire control; the length of a night fight |
| `gun_hits_on_kinugasa` | edge | 0 to 4 | 4 to 49, 15 (60) | The rule of targets (decision 0078) gives her to Salt Lake City while the three cruisers are afloat; in the record she took no fire | A finer rule of targets |

Also seen in the batch and not measured: Task Force 64 fights stragglers by day in 771 runs of 1,000, which the record does not have; and the five destroyers fire every tube in nearly every run, where two attacked. The second is the Helm and Wilson pattern of Savo Island.

## What the agreements show, and what they do not

From decision 0084, and the Cape Esperance check read under it:

1. Each measure is judged alone. No report shows one run holding several agreements together.
2. A figure fitted inside a direction the sources support is still a fit, not a size from a source.
3. Orders, levels and radii were in places chosen with the outcome known, and the check is narrower for it.
4. A yes-or-no agreement says only that the record is not surprising: Chicago "agrees" while she sinks in 565 runs of 1,000.
5. The rate-of-fire check failed and certified no rate.
6. **Cape Esperance, plainly:** the outcome transferred. The side with the radar and the surprise wins the night: two Japanese ships sunk in the middle run, and no US cruiser sunk in 960 runs.
7. The hits did not transfer. The Japanese guns score a median of 6 hits; Duncan's loss and Aoba's hits miss.
8. Part of the outcome agreement rests on fights by day the record does not have. In the 229 runs with no fight by day the median is 1 Japanese ship sunk, not the record's 2.
9. The check shows none of the three figures fitted at Savo Island (`experience_gun_strength`, `surprised_target_factor`, the unready shares) right, and none wrong.

## The figures by status

From the two "figures closed" pages, which sort the figures alike (the figure notes in the files are the authority).

**Surface, search, air combat and derivation figures** (`surface-and-search-figures-closed.md`): 101 figures.

| Sized from rows | Fitted, not checked | Looked at, not moved | Placeholder | No battle reads it |
| --- | --- | --- | --- | --- |
| 5 | 4 | 19 | 61 | 12 |

Three hand-made files are also read by no battle: `port_sizes.csv`, `ship_classes.csv`, `aircraft_types.csv`.

Sized from rows:
- `unready_gun_share` 0.5: Condition II, half the armament manned (decision 0068).
- `unready_torpedo_share` 0.5: Bagley fired four of eight tubes (decisions 0071, 0074).
- `fighter_false_claim_chance` 0.7: 43 claimed by VF-5 and VF-6 (air round 4).
- `[base_hit] aircraft` 0.06: about 2.5 aircraft destroyed on the ground for about 212 bomber sorties (Rabaul round 1).
- `[search_radius] divisor_of_scouting_range` 3: the Bureau of Aeronautics' method (decision 0075).

Fitted, not checked:
- `surprised_target_factor` 2 (Savo Island round 2).
- `experience_gun_strength` 0.5, the cap (Savo Island round 1).
- `anti_aircraft_before_attack_share` 0 (decision 0079).
- `patrol_drawn_off_share` 0 (decision 0080).

**The slow air files** (`air-figures-closed.md`): `air_upkeep.toml` 3 placeholders and 1 read by no battle; `fatigue.toml` 3 placeholders and 2 read by no battle; `air_supply.toml` 4 placeholders. They are tuned on Rabaul and have no battle to check them; rounds 2 and 3 looked at fatigue and wear and moved nothing. Scenario 6's own 7 figures (repairs a day, supply, replacements, the rest threshold, the dive bombers' search) are placeholders or readings.

## The closing tests

Run on Nov 13, 2026 on `main` after the batch of small fixes (pull request 228). The record, with every prompt and answer word for word, is `docs/validation/closing-tests.md` and its folder `closing-tests/` (pull request 230).

| Test | Result |
| --- | --- |
| The hundred-seed soak | **Pass.** 100 seeds of 180 days, 1,978,115 events; the invariants checked at the end of each of the 18,000 days |
| Legibility, on the tuned Eastern Solomons | **Fails its rule by one item.** A cheap model (Claude Haiku) predicted the day from the opening briefing, the day was run, and a fresh one explained it. The rule asks the explanation, claims and truth-and-belief items to pass and three of four predictions. The three pass, and two predictions. It said the American search would find the Japanese carriers in the morning (it was the afternoon), and would not say whether a Japanese strike would come (one came) |
| The rulebook-only hand calculations | **First run not a pass under its rule; the control rerun exact.** First run: every number of a step 2 rule exact, except two derived chances of scenario 19a that differ in the sixth decimal (cut where the hand calculation rounds). In the step 1 control (scenario 04a) one number wrong, "both fire from the start", a term the rulebook did not define, and two more chances cut. After the architect's ruling that such chances are written cut, and the rulebook's fixes, the control sitting was run again: every number exact |

**The legibility test, for you to accept or not.** *The architect's reading:* both misses are about where the enemy would be. The opening briefing holds no contact and no enemy force, only that the enemy is "expected to come down from the north". So the misses are the fog the game is meant to have, not a misreading of a rule. The explanation, the part the spec names in its own words ("explains an outcome"), passed. The test was not run again.

**What followed from the tests** (the architect's rulings, recorded with the tests). Of the legibility test's twelve findings, four are fixed in the glossary and the rulebook, among them one meaning of "crippled": a ship afloat with propulsion or flight deck out; flotation damage of 50 or more is now "heavily damaged". Five change the briefing's wording and follow in a second pull request. Two go to step 3 (listed below), and one needs nothing. Of the rulebook test's eleven findings, seven are fixed in the rulebook and glossary, one is a rule of the hand calculations, and three need nothing. The rerun found one more, R12 (how a starting contact ages), which is open.

## The three decisions the spec left for the close

### (a) Do agreements at the edge count against a battle?

You decided on Nov 12, 2026 that they do not count for now, and that the close would decide ("Decided by the designer", 9). Eight are left: four at Savo Island, two at the Eastern Solomons, one at Rabaul, one at Cape Esperance. Each is in the lists above with what it rests on: overlapping fights, secondary guns, a divisor that is itself a named miss, a yes-or-no measure, and the rule of targets. One has no recorded cause (Rabaul's Japanese fighters lost).

**The architect recommends:** they do not count against a battle, since each is listed with its cause. The cause of the Rabaul fighters' edge should be named before you accept.

### (b) Are the hand calculations worked again on the tuned figures?

The hand-calculation tests run on the step 1 figures, frozen for step 2 ("Decided by the designer", 3), and on frozen copies of scenarios 4 to 8 (`data/scenarios/step2-frozen/`). The live figures are covered in other ways:
- the run baselines, remade with every round, pin what each scenario does on them;
- the determinism scenario;
- the soak.

**The architect recommends:** no. The frozen-figure tests stay as the proof of the rules' arithmetic. The live figures are covered by the pinned outcomes, the determinism scenario and the soak. The same answer is recommended for the frozen scenarios, which the spec ties to this decision ("Tests while figures move", 9).

### (c) The objectives scale

The result on the scale is reported and not judged until the scale counts a ship that is out of the campaign ("Decided by the designer", 4). Every point and level of decision 0054 is still a placeholder. At Rabaul the result is the same in every run (a major Allied victory in 1,000 of 1,000), so the scale does not yet tell one run from another.

**The architect recommends:** the scale stays reported and not judged. Ship repair (decision 0069, which you confirmed) and "out of the campaign" follow step 2, with land units and ground combat, before step 3. This was already ruled on your delegation on Nov 13, 2026.

## Decided on your delegation, Nov 13, 2026

You gave the coordinating architect your decisions on Nov 13 ("Go ahead and decide all matters that need decisions"). Decisions 0070 to 0087 were made under it, or settled by the architect. **One reverses an answer of yours: 0071.** Several others replace or amend items of decisions you had confirmed; they are marked.

| Decision | What it decides | Your earlier answers |
| --- | --- | --- |
| 0070 | Rates of fire as achieved, not as designed, from named cases (settled by the architect, to be reviewed) | None changed |
| 0071 | A force not ready fires part of its torpedoes, as it does its guns | **Reverses your answer 4 to decision 0068** ("torpedoes left as they are") |
| 0072 | The Cape Esperance scenario (settled by the architect, to be reviewed) | None changed |
| 0073 | The Japanese search at the Eastern Solomons from the record | Replaces item 8 of 0031 for the Japanese side |
| 0074 | Torpedo tubes counted by the side they bear on | None changed |
| 0075 | Each aircraft type has a search radius of its own | Changes 0029 item 6, where one radius served both |
| 0076 | How long surprise lasts: no rule | None changed |
| 0077 | Only a surface-search radar with a plan display counts for sighting at night | Changes the derivation rule of 0019 |
| 0078 | Which enemy ship a ship fires at: guns like at like, torpedoes at the most important | Amends the pairing of 0014 |
| 0079 | Anti-aircraft fire split before and after the attack | Amends 0027 items 7 and 8 |
| 0080 | Whom the patrol fires at: escorts draw off only a share | Amends 0027 item 6 |
| 0081 | The Eastern Solomons' remaining orders from the record | Replaces orders of 0031, 0063 and 0073 where it says so |
| 0082 | Rabaul's raid days and the dive bombers' search from the record | Replaces items 16 and 17 of 0042 for the live scenario |
| 0083 | A ship fires the other side's tubes in a later round | None changed |
| 0084 | What the agreements show and do not, after an outside review | None of yours; reverses one ruling of the architect's in 0081 |
| 0085 | What a side is told of a fight, and other leaks | Amends 0047 (items 5, 7, 9) and 0061 |
| 0086 | The numbers a side is given | None changed |
| 0087 | The patrol's fire split before and after the attack | Amends 0079 |

Also for you to look at: decision 0066 (land units) has three choices marked "needs review" (its items 7, 12 and 14), and decision 0083 records one fight for three forces as a plan for you.

## Handed to step 3

Gathered from `docs/step2-opening-list.md` and the round records, with duplicates removed.

**Before step 3, already ruled:**
- Ship repair (decision 0069) and "out of the campaign" on the objectives scale.
- Land units and basic ground combat (decision 0066).
- The staff closing the range on a contact it cannot reach; choices inside a day made into functions the planner can ask; objectives that name a flow; timing a trial of a plan by hand.

**Surface fights:**
- One fight for three forces, so fights can overlap (a plan for you, decision 0083).
- Which ships of a force fire at all, and for how long (Helm and Wilson; Cape Esperance's destroyers).
- A raiding force that fights and passes on.
- Secondary guns.
- Torpedo reloads.
- Fire on a friend, a ship acting apart from its force, identification under fire, and a force stalked before the first shot.
- Radar fire control, and land behind a target masking a set.
- One kind for every cruiser (Tenryu and Yubari).
- Forces left in one hex fighting again by day (Cape Esperance's stragglers; `docs/rules/differences.md`, row 33).

**Air:**
- Bombers' return fire; a search that meets enemy fighters; an aircraft written off for combat damage; a deck closed for a time.
- Height and warning for a field's patrol.
- A strike of a given size.
- Small airfields that wear aircraft faster; wear that does not rise in a straight line with distance.
- A search flown in one pulse only.
- Experience for air groups, waiting on research (decision 0064, stage 2).
- A friendly field beyond the target; daylight as a limit on a launch.

**What a side is shown (from the legibility test):**
- An enemy strike's hidden causes: say that an enemy strike always rests on a contact the enemy held.
- A task force's mission is shown in the briefing and no rule reads it (also `docs/rules/differences.md`, row 12).

**Research asked for:** a B5N2 radius with a bomb load (decision 0081, C5); Rabaul's replacements by date (decision 0082); bounds on Aoba's and Furutaka's hits (decision 0084, 14).

**For the architect to settle from research:** the points and levels of decision 0054; base sizes for the real map; a surface claim with no ship burning (decision 0085, 9).

**Not step 2, kept:** speed made good; coastwatchers and reconnaissance as standing sources.

## Where the records disagreed, and how it was settled

Found while this page was assembled, and corrected in the same pull request on the coordinating architect's rulings of Nov 13, 2026:

1. **The air combat figures by status.** The two figures pages sorted `air_combat.toml` differently. The figure notes' sorting is the one; `air-figures-closed.md` now follows it (the counts above).
2. **The `crew_recovered_*` figures.** Three of the four are read by no battle of the slice; only `crew_recovered_over_own_base` is read, at Rabaul. Both pages and the spec say so.
3. **Cape Esperance** was still "not yet run" on `surface-and-search-figures-closed.md`; it now says checked once, did not pass, not tuned on.
4. **The Ryujo "left to attack" record** is 38. The decisions index and `baseline-reading.md` said 36; both now say 38, with a note that it was 36 for some hours on Nov 13, 2026.
5. **Decision 0074's status line** said "not built"; it now says built with decision 0083.
6. **The spec's quote of `savo-island-r2.md`** (19, 1, 3 of 23) now says that was the file when written, and that it was remade with decision 0084's measures and reads 21, 2, 6 of 29.
7. **The state version of the closing reports.** Savo Island, the Eastern Solomons and Rabaul were reported on state version 7, before decision 0086 moved the engine to version 8, and no record compared the two. One batch of each on the final engine (label `close`) is being run and compared measure by measure with the closing reports. <!-- GAP: when `final-reports` lands, the four-battle table takes the `close` reports for the three tuned battles, the totals are recounted, and any verdict that changed is said here. -->
