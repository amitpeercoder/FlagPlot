# 0064: Experience

Status: proposed, needs review. This is the plan for the designer's approval. Nothing is built, and no unit has a value yet. The hand calculation is written once the plan is approved, before any code is delegated.

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

## Decisions proposed

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

5. **The last question is the weak leg.** It is the only thing in the facts that lifts the Japanese above an ordinary Allied cruiser, and it is a statement about a navy, not about a ship. The research warns that several statements about the Japanese were written after the battle by the side that lost it. This one rests on Japanese officials and records, and was read at the source. The architect will commission a second source on Japanese night training before the values are fixed. If the designer will not admit a statement about a navy, the rows cannot tell the two sides apart at Savo Island, and by the spec's fourth condition experience waits.

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

## Questions for the designer

1. **May a statement about a navy's training feed a ship's value?** Decision 5. Recommended: yes, with a second source commissioned first. Without it experience waits and the surface figures stay marked as fitted and not checked.
2. **Night only?** Decision 8. Recommended.
3. **Air groups: derive the values and show them before any rule reads them?** Decision 9. Recommended.
4. **Leave out experience that rises with time and action, for step 2?** Recommended.
