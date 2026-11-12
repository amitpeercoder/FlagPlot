# 0064: Experience

Status: confirmed. The designer approved the plan on Nov 12, 2026, with the recommended answer to each of its four questions. Stage 1 (ships) is built; see "As built (stage 1)". Stage 2 (air groups) is not. The hand calculation is written before any code is delegated, and this record gains an "As built" section with each stage.

## The gap

At Savo Island both sides fire by the same rules, and only their ships' figures differ. In the baseline the Japanese guns score 24 to 51 hits for each thousand shells where the record has 81 to 117, and the Allied guns are about right (`docs/validation/baseline-reading.md`). No cruiser is sunk in 681 runs of 1,000; the night cost four. What is missing is that one navy had trained to fight at night and most of the ships it met had not. If that is not a rule, tuning bends the gun and torpedo figures to do its work, and Cape Esperance then fails for a reason no figure can mend.

The spec keeps experience in step 2 on four conditions (`docs/step2-spec.md`, "Experience"): a unit's value is derived from fact rows by a written rule; the values are fixed before tuning; only the strength of the effect is tuned, the same for both sides; and if the facts are too thin, experience waits. This plan is written to those conditions, and says plainly where the facts are thin.

## What the facts will bear

From `docs/research/oob/experience.md` and the rows it names.

| | What has a row | What does not |
| --- | --- | --- |
| Ships' crews | The date a ship was commissioned. For most Allied ships at both battles, whether she had fired or exercised at night, and when. One statement for the Japanese navy as a whole, from Japanese officials and records, read at the source: its fleet training put "great stress" on night attack | Any number for a crew. Any direct statement that the Japanese cruisers at Savo Island had fired at night that year: the 1950 analysis assumes it |
| Air groups | The date a unit first saw action, most at low confidence. Flying hours on joining for two services: 650 for the Japanese navy in December 1941, 275 for the Marine squadrons of mid 1942 | Hours for a US Navy pilot of 1942: not found anywhere. Any share of veterans in any unit. Any figure for a Japanese replacement of late 1942 |

So the ships can carry a value now, with one weak leg named below. The air groups are thinner, and the plan takes them second.

## Decisions

### The number

1. **A ship has an experience value and an air group has one:** a whole number from 0 to 4. 2 is a crew or a group trained to its service's standard, with nothing in the record for or against it. A placeholder unit is 2.
2. **A value is derived from fact rows by a written list of yes or no questions,** by `oob-derive`, as a ship's other figures are (decision 0020). Each question a row answers moves the value up or down by 1 from 2. A question no row answers moves nothing: a missing fact is a gap, not a guess. Words such as "well trained" or "very inexperienced" answer no question.
3. **The questions are fixed, and the values written into the scenario files, before any batch run with the rule on is looked at.** After that a value changes only when a fact row changes, and never to improve a report.

### The questions for a ship (stage 1)

4. Proposed, with the thresholds to be fixed in the derivation file:

   | Question | Moves | Who it touches, on a first reading of the rows |
   | --- | --- | --- |
   | Does a source say she fired or exercised at night in the three months before the battle? | Up 1 | San Francisco, Salt Lake City and Boise (7 October 1942). Astoria, by one source and against another |
   | Does a source say she held no night practice in the year before? | Down 1 | Vincennes and Quincy |
   | Was she in commission less than a year? | Down 1 | The new destroyers at Cape Esperance |
   | Did her navy's fleet training put night action first, by a statement about the navy read at its source? | Up 1 | Every Japanese ship |

   On that reading the Japanese cruisers are 3 at both battles; Vincennes and Quincy are 1; Chicago, Canberra and Australia are 2; Scott's three practised cruisers are 3, Helena 2 and his new destroyers 1. These are an illustration of the rule, not settled values.

5. **The last question is the weak leg.** It is the only thing in the facts that lifts the Japanese above an ordinary Allied cruiser, and it is a statement about a navy, not about a ship. The research warns that several statements about the Japanese were written after the battle by the side that lost it. This one rests on Japanese officials and records, and was read at the source. The architect will commission a second source on Japanese night training before the values are fixed. The designer admitted a statement about a navy on Nov 12, 2026, with the second source to be commissioned first. Without it the rows could not tell the two sides apart at Savo Island, and by the spec's fourth condition experience would wait.

### What reads it (stage 1)

6. **Three rules read a ship's experience, and only in a dark fight:** the chance a gun shot hits, the chance a torpedo hits, and the chance a force sights the enemy (the force's value is the mean of its ships afloat). Each chance is multiplied by

   `1 + strength x (experience - 2)`

   with one strength figure for each of the three rules, from 0 to 0.5, the same for both sides. Those three figures are what tuning may move.
7. **With every strength at 0 the rule does nothing.** The frozen step 1 figures hold 0, so no hand calculation of step 1 moves.
8. **Nothing reads it by day.** The questions are about night fighting, and step 2 has no daylight surface action to check a daylight effect against.

### Air groups (stage 2)

9. **An air group's value is derived the same way,** from the date it first saw action and its service's flying hours on joining, where a row gives them. The architect derives the values and shows them to the designer before any rule reads them.
10. **If they stand, three rules read it:** a fighter's chance to hit, a bomber's chance to hit a ship, and the chance of running out of fuel on the way home (decision 0059), each with its own strength figure.
11. **Replacements dilute it.** Replacement pilots arrive at their service's value for the period, and the group's value becomes the average of its pilots', weighted by number. This reuses the replacement step (decision 0038) and replaces its line "replacement pilots are as good as the pilots they replace". The group's value then carries a fraction.

### What a side knows

12. A side sees its own units' experience exactly, in the briefing and in the factors of its own fire. It sees nothing of the enemy's: no method of the side's view returns it, and an event the enemy may see is cut of it.

## A small worked case

Two cruisers fire at night with the placeholder `gun_hit_chance_night` of 0.03, neither surprised nor lit, and a strength of 0.4.

| Ship | Experience | Factor | Chance | Hits expected from 300 shells |
| --- | --- | --- | --- | --- |
| A | 3 | 1 + 0.4 x 1 = 1.4 | 0.042 | 12.6 |
| B | 1 | 1 + 0.4 x (-1) = 0.6 | 0.018 | 5.4 |

A ship at 2 fires at 0.03, as today. The strength of 0.4 is for the example only.

## The test

Cape Esperance is built after Savo Island is tuned and is never tuned on. On the first reading above, the cruisers of the two sides there are about level in experience, so the American result has to come from the rules already in the model: radar, and a Japanese force that was not alert. Values fixed from training records either bring that night out right or they do not.

## Why

- **A few steps, not a fine scale.** The facts support "trained for it", "not trained for it" and "nothing said". A value of 63 against 58 would claim a precision no row has.
- **Questions, not judgement.** Anyone can work a ship's value from the rows and get the same answer, and a new row moves it by a rule.
- **One number, one formula.** Every rule that reads experience reads it the same way, and a strength of 0 switches it off.
- **Ships first.** Savo Island is what cannot be tuned without it. The air groups' facts are thinner and their effect on the two air battles is less sure: fighters die too fast on both sides over Guadalcanal, which no difference in experience explains.

## What it leaves out

- **Experience that rises with time and action.** No source gives a rate, and a rate would have to be worked back from results, which the first condition forbids. Four weeks is the longest scenario. It waits for a campaign.
- **Fighting a fire.** One second-hand sentence stands for the Japanese against detailed American self-criticism; the two sides cannot be put on one scale.
- **Morale,** already left out by the designer.
- **Ships that had not worked together.** The rows on this are disputed for the Japanese at Savo Island and clear for Scott's destroyers. It may become a question later; it is not one now.
- **Experience by day,** until a daylight action is on record to check it.

## What it costs

- A new field of state for a ship (stage 1) and for an air group (stage 2). The hash version rises with each and the golden files' hashes are remade.
- Three new figures in `surface_combat.toml` in stage 1 and three in `air_combat.toml` in stage 2, all placeholders until tuned.
- `oob-derive` gains the questions, and `oob-check` must read `training_facts.csv`, which it does not yet.
- The scenarios of Savo Island, the Eastern Solomons, the four weeks over Guadalcanal and the full slice gain derived values. With the strengths at 0 nothing in them changes but the hashes.

## Rule card

| | |
| --- | --- |
| Reads | A ship's experience and whether the fight is dark; the three strength figures. In stage 2, an air group's experience, the replacements arriving and their service's value, and three more strength figures |
| Writes | Nothing in stage 1: a ship's experience is set when the scenario loads and does not change. In stage 2, an air group's experience when replacements join it, with an event that names them |

## Answered by the designer

On Nov 12, 2026, as recommended in each case.

1. **May a statement about a navy's training feed a ship's value?** Yes (decision 5), with a second source commissioned before the values are fixed.
2. **Night only?** Yes (decision 8).
3. **Air groups: derive the values and show them before any rule reads them?** Yes (decision 9).
4. **Leave out experience that rises with time and action, for step 2?** Yes.

## As built (the derivation)

The part of stage 1 that does not touch the simulation: `oob-check` reads `data/oob/training_facts.csv`, and `oob-derive` derives a ship's value from fact rows. Nothing reads the values yet; no scenario has one, and the hash is unchanged.

### The four questions, as fixed

Fixed by the architect on Nov 12, 2026, before any batch with the rule on exists. A value starts at 2; each question a row answers moves it by 1, once, however many rows answer it; it is clamped to 0 to 4. Rows that are not preferred never move a value and are printed, with the other rows no question read, as not read (not written to the generated file). The thresholds are in `data/derivation/rules.toml`, measured to the first day of the engagement (`engagements.csv`, `date_from`: 1942-08-08 for Savo Island, 1942-10-11 for Cape Esperance).

1. **`night_practice_recent`, up 1.** A preferred row of medium or high confidence, naming the ship, says she fired or exercised at night, and the row or its note dates that inside the three months that end on that day. "Night training" is exercising at night. An undated "recently" answers nothing. A statement about a force or a squadron answers for a ship only if a row names her as part of it at that time.
2. **`no_night_practice`, down 1.** Such a row says she held no night practice over a period that reaches into the twelve months that end on that day, and no preferred row says she held one in them. A statement about "practically none of the ships" of a force names no ship and answers nothing.
3. **`new_ship`, down 1.** Her `commissioned` date is later than that day less twelve months (strictly). Where every commissioning row is disputed, the question is answered if every row, whatever its standing, gives the same answer; otherwise it is a problem to settle.
4. **`navy_night_first`, up 1.** The rules file names, for a nation, the `training_facts.csv` rows that say its fleet training put night action first, and the kinds of ship they cover. For Japan that is row 32 (cruisers and destroyers). A nation with no such row is not answered "no": nothing moves.

### The settlements, and why

- **The list carries each answer for questions 1 and 2** (`[[experience.answer]]`: ship, question, row, answer, a reason, and optionally `named_by`, `from`, `to`). The tool checks the row exists, is of that ship, is a `night_practice_note` row, is preferred and of medium or high confidence, and that the period meets the question's window (a partial date counts if some day it could stand for is inside). The period is read from the row's `valid_from` and `valid_to` (see "Dates", below); the answer gives it only where the row has none.
- **A row for and a row against on one ship and question stop the derivation** unless the list settles it by naming the row it reads, with a reason (`[[experience.settle]]`). A settlement with nothing to settle is a problem.
- **A statement about a squadron or a force answers for a ship only if a row names her as part of it at that time** (the rule as fixed in pull request 137). Settled on Nov 12, 2026, once rows naming the destroyers existed: "at that time" means a row puts her with that force during the period the statement is dated to; a list of the squadron on one day does not by itself show that, as the rows' own note says. So the answer names the list and a row that puts her with the force (see "Force statements", below). Row 71 ("practically none of the ships") names none and answers nothing.
- **Astoria** is 2: row 1018 says she held "recently" some night practices, undated, so question 1 is not answered, and question 2 is not answered because a row says she held some.
- **Row 1022 does not contradict night training.** It says no ship had fired at night within eight months; it is about firing.
- **Decision 5 requires a second source for Japan before any value is fixed.** It is `training_facts.csv` row 87: Vice Admiral Ozawa to the Strategic Bombing Survey, 1945, "we stressed training on that type of battle from before the war". It is Japanese testimony, independent of the 1950 analysis behind row 32. The architect settled what counts as a second source: a row of the navy's entity, from a different `source` than the first row, read at the page, whose standing is not `disputed`; it need not be `preferred` (row 87 is `unconfirmed`, being one witness). The tool checks exactly that (row 87 is of `ijn-surface-fleet`, the navy's whole surface fleet, so the rules file names that entity for it). With it the Japanese values are **fixed** at 3. Its limits: it is about the navy as a whole, names no ship or date, and the question put to him assumed the emphasis. The tool keeps the other mechanism: with `second_source` empty for a nation, its ships are marked `provisional` in the `fixed` column and the file's header says so; `--check` passes on such a file, and a later stage that writes values into scenarios must refuse one. Synthetic tests cover a nation with no second source.

### The values

As they stand after the changes of Nov 12, 2026 (below). From the rows, with the questions above. Generated: `data/reference/ship_experience_savo_island.ships.csv` and `data/reference/ship_experience_cape_esperance.ships.csv`; the real-data tests carry the architect's hand calculation.

| Savo Island | Value | Why |
| --- | --- | --- |
| Chokai, Aoba, Kako, Kinugasa, Furutaka, Tenryu, Yubari, Yunagi | 3, fixed | Question 4 by row 32, second source row 87 |
| Canberra, Australia, Chicago | 3 | Question 1: night training at Brisbane, May to July 1942 (rows 1019, 1020, 1021). Decision 0064's illustration gave 2, a first reading that said so |
| Vincennes, Quincy | 1 | Question 2: no night practice in at least fifteen months in the Atlantic Fleet, which they left in March 1942 (rows 1016, 1017; 949, 950) |
| Astoria | 2 | Nothing answered |
| Helm | 3 | Question 1 by the force statement (rows 1019 to 1021), named in Destroyer Squadron 4 (row 1064) and joined Task Force 44 at Brisbane on 19 May 1942 (row 1072) |
| Bagley | 3 | The same: named (row 1068), at Brisbane from 30 May (row 1074) |
| Ralph Talbot | 3 | The same: named (row 1066), left Hawaii on 14 June (row 1078) and left Brisbane with the force on 14 July (row 1070): with the force at Brisbane inside the period, if briefly |
| Patterson | 3 | The same: named (row 1065); her rows on where she was are disputed (Gill, row 1069, left Brisbane with the force on 14 July; the ship dictionary, row 1075, on 22 June) and both put her at Brisbane inside the period, so the answer stands and names both |
| Wilson, Blue, Jarvis | 2 | Nothing answered. No row names Wilson in the squadron. Blue is named (row 1063) but no row puts her at Brisbane (row 1077: convoys, then Wellington). Jarvis is named (row 1067) and her two disputed rows disagree (Gill, row 1071, with the force; the dictionary, row 1076, at Sydney from 18 June) |

| Cape Esperance | Value | Why |
| --- | --- | --- |
| Salt Lake City, San Francisco, Boise | 3 | Question 1: the night firing practice of 7 October 1942 (rows 1033, 1036, 1037) |
| Helena | 2 | Nothing answered |
| Farenholt, Buchanan, Laffey | 2 | Question 1 by the Navy's narrative of the force (training_facts row 74: target practice on 8 October and brief antiaircraft practice that night, an exercise at night), each named as one of the three destroyers of the force as first organised, with it on 7 and 8 October (rows 1081, 1082, 1083); question 3 (commissioned March to May 1942). 2 - 1 + 1 |
| Duncan, McCalla | 1 | Question 3 only: rows 1084 and 1085 say each was not with the force for the practices of 8 October, so question 1 is not answered |
| Aoba, Furutaka, Kinugasa, Fubuki, Hatsuyuki | 3, fixed | Question 4 by row 32, second source row 87 |

The values are those of the architect's hand calculation, which the real-data tests carry.

### What changed on Nov 12, 2026, and why

New fact rows arrived: the destroyers of the two forces (rows 1060 to 1085), the Japanese cruiser divisions' war diaries (rows 1086 to 1122) and dates on the training rows. A ship's value changes only when a fact row changes, by the four questions as they are worded, and none was reworded.

- **Seven values changed, by new rows:** Helm, Bagley, Patterson and Ralph Talbot from 2 to 3 (Savo Island), and Farenholt, Buchanan and Laffey from 1 to 2 (Cape Esperance). Nothing else moved: the eight Japanese ships 3, Vincennes and Quincy 1, Astoria 2, Chicago, Canberra and Australia 3, Wilson, Blue and Jarvis 2, San Francisco, Boise and Salt Lake City 3, Helena 2, Duncan and McCalla 1, the five Japanese ships 3.
- **Bagley and Helm are 3 although Bagley fired no gun at Savo Island and Helm fired four shells.** The rule gives it: both were with the force at Brisbane while it trained at night. Experience is not readiness; decision 0068 is the rule for readiness.
- **The Japanese cruisers keep 3, and the war diaries change nothing.** No row says a Japanese cruiser fired or exercised at night inside either window. The drills and firings the 6th Cruiser Division's diary records in July and October 1942 (rows 1102 to 1105, 1109 to 1111) say "the diary does not say" whether by night. For Tenryu, the 18th Cruiser Division's order for the passage to Truk schedules a searchlight drill and a night formation-keeping test in June 1942 (row 1117), inside her three months, but it is an order, "not a record that they were done", and the record of what was done (row 1118) does not say night: an order is not a statement that she exercised, and she is not answered. The rows that say the diaries record no night exercise (rows 1098 to 1101, 1112 to 1116) are the researcher's finding of an absence, of low confidence and unconfirmed, in diaries with months missing; an absence is a gap and answers no question.
- **The designer's decision, Nov 12, 2026:** shown that the only rows about these ships themselves fail to confirm the navy-level statement, and that the 18th Division's own lessons after Savo Island ask for "still more earnest, intensive training" in opening fire promptly and in rate of fire (row 1121), the designer decided the Japanese ships keep the point for their navy's night training (training_facts rows 32 and 87). His words: "Keep it". Recorded plainly: **the weak leg of decision 5 is weaker for it.** The point still rests on a statement about a navy, and the one body of evidence about these ships' own divisions does not confirm it and, in the 18th Division's lessons, qualifies it.

### Force statements

A force or squadron statement may answer question 1 for a ship. An `[[experience.answer]]` may carry `named_by`, the row or rows that name the ship as part of the force at that time, beside `row`, the statement. The tool checks:

- the statement row is preferred, of medium or high confidence, and dated inside the three months;
- each `named_by` row is of that ship, of medium or high confidence, and dated so that it puts her with the force inside the statement's period (its own period meets the statement's; a row with a start and no end goes on);
- a `named_by` row is preferred, except that disputed rows are allowed if every disputed row of hers that bears on the period (that meets it) is named too and each is dated inside it. That is the same rule the tool already has for disputed commissioning rows, applied to where she was, and the answer names all of them;
- the attributes allowed are a short list in the rules file (`[experience.force]`), checked by the tool: the statement is a `night_practice_note` row of `ship_facts.csv` or a `night_training_note` row of `training_facts.csv`; `named_by` rows are her `time_in_theatre_note` or `night_practice_note` rows;
- `named_by` is for question 1 only.

The tool reads dates, standings and attributes, not what a row says. That a row puts her with the force, and that disputed rows agree, is the architect's reading, written in the answer's reason; the tool cannot tell that Jarvis's disputed rows disagree, which is why her answer is not given. Each destroyer's answer names the list (the squadron on 7 July) and a row that puts her with the force, together.

### Dates

The research session has dated the ship-crew rows, so the periods come from the rows. An answer's `from` and `to` are optional: where the cited row has `valid_from` and `valid_to` the tool reads them; if the answer also gives dates they must agree, else a problem. Where the row has none, the answer must give them, with its reason, as before: question 1 needs both, question 2 needs `to` (an open start is allowed). Only the two Atlantic rows (ship_facts 1016 and 1017) have none now.

### What a run prints instead of the rows not read

The generated file no longer carries a `not_read` column. A research pull request that adds a row for a listed ship therefore no longer makes it stale. `--check` and a run print, after the list of files, one line for each listed ship that has rows no question read (night practice, commissioning and, for the force rules, time-in-theatre rows), with their standing. The generated file keeps the `rows` column: the rows read, "no" answers included, and the rows that name a ship in a force.

### What the questions leave unused

Rows about night training that no question reads: the Japanese lookout, "better trained" and "assumed" rows (inferred, unconfirmed); damage control for both sides; "never maneuvered together" and its dispute; the planned weekly night quarters of 1941; the star-shell rows; rows 89 to 91 (Omori on November 1943: "lack of training in night operations", substitute ships that "never had an opportunity to train together"), which are about 1943 and limit nothing for August and October 1942; row 88 (a destroyer squadron that was at neither battle) and row 92 (night action as a basis of ship design, not of training); the destroyers' "diverse origins"; the rows that Task Force SUGAR "engaged only in target practice" and the Wikipedia rows on 8 October (not preferred, or about a force); the post-battle judgements. Question 3 does not read `last_refit_completed` or captains. The time-in-theatre rows are read only to name a ship in a force (question 1).

### Gaps found

- Wilson is named in no row of Destroyer Squadron 4. Blue is named but no row puts her at Brisbane. Jarvis's rows disagree. Their night training, if any, cannot be matched to them.
- Two ship-crew rows are undated (`valid_from`, `valid_to`): ship_facts 1016 and 1017 (the Atlantic months), whose answers give the period.
- Five rows of `training_facts.csv` would fail the "earned preferred" rule (lone medium rows marked `unconfirmed`: 3, 30, 31, 33, 79). `oob-check` does not apply that rule to the file in this stage.

## As built (stage 1)

Decisions 1, 6, 7, 8 and 12 in the simulation, for ships.

- **State.** `Ship.experience`, a `u8` from 0 to 4, default 2 (`Ship::DEFAULT_EXPERIENCE`). A scenario's `[[ship]]` may give `experience = N`; the loader refuses a value outside 0 to 4, a float or a string. No rule writes it. It is hashed last in the ship, named in `hash::name_every_field`; `hash::STATE_VERSION` is `flagplot-state-v6`. Every hash in every golden file changes and nothing else does.
- **Figures.** `experience_gun_strength`, `experience_torpedo_strength` and `experience_sighting_strength` in `surface_combat.toml`, each from 0 to 0.5 (the loader refuses outside). The frozen file has all three at "0", so no step 1 result moves; the live file has "0.25", a placeholder, the middle of the range.
- **The rule.** The factor is `experience_factor(strength, experience) = 1 + strength x (experience - 2)`. A gun or torpedo salvo in a dark fight takes the firer's own value, multiplied in last after the fire control factor. A force's chance to sight in a dark fight is `min(1, (base + radar_bonus) x alert_factor x experience_factor) x fatigue_factor`, with the mean of the ships in the force's list at the moment of the roll (sum over count in `Fixed`: ships of 3, 3 and 2 have 2.666666, a factor of 1.266666 at 0.4 and a sighting of 0.633333). By day nothing reads it. No new random draw.
- **Breakdown.** `experience_factor` is last among a salvo's chance factors and after `alert_factor` in a sighting, and is there only in a dark fight when the rule's strength is above 0. On the frozen figures no breakdown and no event changes.
- **What a side sees.** In `side_events.rs`, `experience_factor` is added to the own factors of `Salvo` (the firer's side). A `SightingRoll` is shown whole to its own force's side and already carried it. `SalvoAgainst` has no breakdown. The query answers `detail` for a ship and for a force's ships give `experience` when it is not 2, for the side's own ships only; no method of `SideView` returns an enemy ship's experience.
- **Values.** Scenario 4 gives `experience` to each ship whose value is not 2, from `data/reference/ship_experience_savo_island.ships.csv`; a test holds every ship to the file's value. No other scenario gives values.
- **Tests.** Scenario 14a (`data/scenarios/14a_experience.toml`, `tests/scenario_14a_experience.rs`): scenario 4a with experience 3 and 1, at a working strength of 0.4, with the architect's hand calculation. Unit tests in `sim-core` for the factor, the mean, the breakdown, the hash and the figures' limits, and in `sim-data` for the loader.
- **Limits.** Night and ships only; no change with time or action; the replay export does not show experience.
- **Rule card as built.** Reads: a ship's experience, whether the fight is dark, the three strengths. Writes: nothing.
