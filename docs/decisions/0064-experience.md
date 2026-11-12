# 0064: Experience

Status: confirmed. The designer approved the plan on Nov 12, 2026, with the recommended answer to each of its four questions. Nothing is built yet, and no unit has a value. The hand calculation is written before any code is delegated, and this record gains an "As built" section with each stage.

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

Fixed by the architect on Nov 12, 2026, before any batch with the rule on exists. A value starts at 2; each question a row answers moves it by 1, once, however many rows answer it; it is clamped to 0 to 4. Rows that are not preferred never move a value and are listed in the output as not read. The thresholds are in `data/derivation/rules.toml`, measured to the first day of the engagement (`engagements.csv`, `date_from`: 1942-08-08 for Savo Island, 1942-10-11 for Cape Esperance).

1. **`night_practice_recent`, up 1.** A preferred row of medium or high confidence, naming the ship, says she fired or exercised at night, and the row or its note dates that inside the three months that end on that day. "Night training" is exercising at night. An undated "recently" answers nothing. A statement about a force or a squadron answers for a ship only if a row names her as part of it at that time.
2. **`no_night_practice`, down 1.** Such a row says she held no night practice over a period that reaches into the twelve months that end on that day, and no preferred row says she held one in them. A statement about "practically none of the ships" of a force names no ship and answers nothing.
3. **`new_ship`, down 1.** Her `commissioned` date is later than that day less twelve months (strictly). Where every commissioning row is disputed, the question is answered if every row, whatever its standing, gives the same answer; otherwise it is a problem to settle.
4. **`navy_night_first`, up 1.** The rules file names, for a nation, the `training_facts.csv` rows that say its fleet training put night action first, and the kinds of ship they cover. For Japan that is row 32 (cruisers and destroyers). A nation with no such row is not answered "no": nothing moves.

### The settlements, and why

- **The ship-crew rows carry no `valid_from` or `valid_to`,** so the list carries each answer for questions 1 and 2 (`[[experience.answer]]`: ship, question, row, answer, `from`, `to` as the row's text or note gives them, a reason). The tool checks the row exists, is of that ship, is a `night_practice_note` row, is preferred and of medium or high confidence, and that the period meets the question's window (a partial date counts if some day it could stand for is inside). When the research session dates the rows the answers can go.
- **A row for and a row against on one ship and question stop the derivation** unless the list settles it by naming the row it reads, with a reason (`[[experience.settle]]`). A settlement with nothing to settle is a problem.
- **A statement about a squadron or a force answers for no ship the rows do not name.** No row names the destroyers of Destroyer Squadron 4, so the night training of "Chicago and Destroyer Squadron 4" (rows 1019 to 1021) answers for none of them. Row 71 ("practically none of the ships") names none.
- **Astoria** is 2: row 1018 says she held "recently" some night practices, undated, so question 1 is not answered, and question 2 is not answered because a row says she held some.
- **Row 1022 does not contradict night training.** It says no ship had fired at night within eight months; it is about firing.
- **Decision 5 requires a second source for Japan before any value is fixed.** It is `training_facts.csv` row 87: Vice Admiral Ozawa to the Strategic Bombing Survey, 1945, "we stressed training on that type of battle from before the war". It is Japanese testimony, independent of the 1950 analysis behind row 32. The architect settled what counts as a second source: a row of the navy's entity, from a different `source` than the first row, read at the page, whose standing is not `disputed`; it need not be `preferred` (row 87 is `unconfirmed`, being one witness). The tool checks exactly that (row 87 is of `ijn-surface-fleet`, the navy's whole surface fleet, so the rules file names that entity for it). With it the Japanese values are **fixed** at 3. Its limits: it is about the navy as a whole, names no ship or date, and the question put to him assumed the emphasis. The tool keeps the other mechanism: with `second_source` empty for a nation, its ships are marked `provisional` in the `fixed` column and the file's header says so; `--check` passes on such a file, and a later stage that writes values into scenarios must refuse one. Synthetic tests cover a nation with no second source.

### The values

From the rows, with the questions above. Generated: `data/reference/ship_experience_savo_island.ships.csv` and `data/reference/ship_experience_cape_esperance.ships.csv`; the real-data tests carry the architect's hand calculation.

| Savo Island | Value | Why |
| --- | --- | --- |
| Chokai, Aoba, Kako, Kinugasa, Furutaka, Tenryu, Yubari, Yunagi | 3, fixed | Question 4 by row 32, second source row 87 |
| Canberra, Australia, Chicago | 3 | Question 1: night training at Brisbane, May to July 1942 (rows 1019, 1020, 1021). Decision 0064's illustration gave 2, a first reading that said so |
| Vincennes, Quincy | 1 | Question 2: no night practice in at least fifteen months in the Atlantic Fleet, which they left in March 1942 (rows 1016, 1017; 949, 950) |
| Astoria | 2 | Nothing answered |
| Bagley, Patterson, Helm, Wilson, Blue, Ralph Talbot, Jarvis | 2 | Nothing answered |

| Cape Esperance | Value | Why |
| --- | --- | --- |
| Salt Lake City, San Francisco, Boise | 3 | Question 1: the night firing practice of 7 October 1942 (rows 1033, 1036, 1037) |
| Helena | 2 | Nothing answered |
| Farenholt, Duncan, Laffey, Buchanan, McCalla | 1 | Question 3: commissioned March to May 1942 |
| Aoba, Furutaka, Kinugasa, Fubuki, Hatsuyuki | 3, fixed | Question 4 by row 32, second source row 87 |

The values are those of the architect's hand calculation, which the real-data tests carry.

### What the questions leave unused

Rows about night training that no question reads: the Japanese lookout, "better trained" and "assumed" rows (inferred, unconfirmed); damage control for both sides; "never maneuvered together" and its dispute; the planned weekly night quarters of 1941; the star-shell rows; rows 89 to 91 (Omori on November 1943: "lack of training in night operations", substitute ships that "never had an opportunity to train together"), which are about 1943 and limit nothing for August and October 1942; row 88 (a destroyer squadron that was at neither battle) and row 92 (night action as a basis of ship design, not of training); the destroyers' "diverse origins"; the rows that Task Force SUGAR "engaged only in target practice" and the Wikipedia rows on 8 October (not preferred, or about a force); the post-battle judgements. Question 3 does not read `last_refit_completed`, captains or time in theatre.

### Gaps found

- No row names the ships of Destroyer Squadron 4, so its night training cannot be matched to a destroyer.
- No ship-crew row is dated (`valid_from`, `valid_to`): the periods are carried in the lists, and the research session is asked to date the rows.
- Five rows of `training_facts.csv` would fail the "earned preferred" rule (lone medium rows marked `unconfirmed`: 3, 30, 31, 33, 79). `oob-check` does not apply that rule to the file in this stage.
