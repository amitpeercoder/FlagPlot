# 0168: A raid recognised by the experience of the ship attacked: an amendment to decision 0167

Status: a plan by the architect, written on Nov 19, 2026 at the designer's suggestion and confirmed by him the same day as recommended. The hand calculation is written. To be built after decision 0167's build is merged, and not before.

Summary: Decision 0167 gives every attacked ship the same chance of recognising the aircraft that raided it. Recognising aircraft was a trained skill, and the game already holds the experience of a ship's crew. The chance for a ship is now moved by its own experience, by the form the dark surface fight uses and a strength figure of its own; a crew of the standard experience recognises as before, and a base, which has no experience, keeps the plain chance

Standing: Confirmed by the designer, Nov 19, 2026, as recommended. The hand calculation is written; the build waits for decision 0167's

## Why

The designer asked whether the chance of recognising an aircraft should depend on the experience of the force attacked. It should. A ship long in the theatre had lookouts and gunners who knew a torpedo plane from a dive bomber as it came in; a crew new to it often did not, and reported "aircraft".

The game has what it needs. A ship has an experience, a whole number from 0 to 4 with 2 its service's standard, set when the scenario loads and written by no rule (decision 0064). Three chances of a dark surface fight read it by one form, `1 + strength x (experience - 2)`, each with a strength figure of its own, and a land unit's fire reads the unit's experience by the same form. Nothing new is built: one more chance reads the same field in the same way.

## The rule

1. **A ship's chance is moved by its own experience.** Where an air attack on a ship is resolved, the chance that its people recognise the aircraft is `min(1, raid_recognised_chance x experience_factor)`, in place of the plain figure of decision 0167, item 3. The one draw of that decision is made against it.
2. **The factor** is `1 + raid_recognised_experience_strength x (experience - 2)`, with the experience of the ship attacked. With the strength at 0 it is 1 and nothing is multiplied, so the chance is the plain figure to the last digit. A crew of experience 2 has a factor of 1 at any strength.
3. **The ship attacked, not its force.** Each air attack on a ship has its own event and its own draw (decision 0167, item 5), so each ship of a force is asked on its own experience. Two ships of one force under one raid may differ in what they made out, and the side is told both.
4. **A base keeps the plain chance.** A base has no experience. A raid on a base is recognised with `raid_recognised_chance` as decision 0167 has it. Taking the experience of the men ashore there is left for later (below).
5. **The strength** is one figure of the air combat figures file, `raid_recognised_experience_strength`, a placeholder marked as such: no source sizes it. It is refused below 0 or above 0.5, as the strengths of the surface fight are, so that no factor falls below 0.
6. **By day.** The rulebook says experience is read in a dark fight only. That stays true of the surface fight. This is the first chance read by day that a ship's experience moves, and the rulebook is restated to say so.
7. **Nothing else of decision 0167 changes.** The weapon is always told. A type not recognised is not named, and never named wrongly. The draw, its stream and what the true event records are as that decision has them; the attacker is told nothing new; no belief is written and no doctrine reads it.
8. **The breakdown** of the draw names the plain chance, the experience and the factor where the strength is above 0, and the plain chance alone where it is 0, as the surface fight names its factor only where it applies.
9. **Experience is not written.** No ship's experience is changed by this rule or to fit it. Where the effect is to be tuned, the strength is tuned (the standing rule of tuning).

## What it does, at the live figures

With `raid_recognised_chance` at 0.75 and the strength at 0.25:

| Experience | Factor | Chance |
| --- | --- | --- |
| 0 | 0.5 | 0.375 |
| 1 | 0.75 | 0.5625 |
| 2, the standard | 1 | 0.75 |
| 3 | 1.25 | 0.9375 |
| 4 | 1.5 | 1, cut from 1.125 |

A ship has an experience other than 2 only where a scenario gives it one, and a scenario gives one only where it is researched: today the ships of Savo Island and of Cape Esperance. Every other ship recognises as decision 0167 has it until its experience is researched.

## What it leaves for later

- **A base's men.** The experience of the land units ashore at a base could move the base's chance. It needs a ruling on whose experience counts where several units are ashore, and most bases of the 39 days hold men at the standard.
- **Experience that grows.** A ship's experience does not change with time or action (decision 0064). A crew raided daily does not learn here.
- **The fighters' pilots,** who saw the raid closer than anyone, are not asked (decision 0167, What it leaves for later), and an air group has no experience yet.

## Rule card

- **Reads:** the experience of the ship attacked; `raid_recognised_chance`; `raid_recognised_experience_strength`.
- **Writes:** nothing more than decision 0167 writes: the one field of the true event of an air attack on a ship.
- **Reuses:** the experience of a ship and the form of its factor (decision 0064); the draw of decision 0167. No new draw, event or field of the state.

## What it costs

- **One figure.** The air combat figures file gains `raid_recognised_experience_strength`: 0.25 in the live figures, a placeholder; 0 in the frozen step 1 figures, where the chance is the plain figure and decision 0167's hand calculation holds as written.
- **Versions.** No field of the state and no event changes shape. The rules label rises, since a game recorded at the live figures with a ship of experience other than 2 under a raid is told differently. Whatever else the build finds it must raise, it reports.
- **Earlier games.** No fight is resolved differently: the draw's stream is its own. A recorded game changes only where a ship whose experience is not 2 is attacked from the air on the live figures, and there only in whether the type is named. The build lists each.
- **The rulebook:** chapter 5's passage on what the attacked saw gains the factor; chapter 4's "Known limit" on experience is restated (item 6); the glossary's entry for experience names the new reader.
- **The table:** nothing. It says what the side's event says.

## Confirmed on Nov 19, 2026

The designer suggested the rule, the architect recommended its form, and the designer confirmed: "yes, please write that amendment as per your recommendations".

1. The chance of a ship is moved by the ship's own experience, by the form the game already uses.
2. The strength is 0.25, a placeholder.
3. A base keeps the plain chance in this build.
4. It is built after decision 0167's build is merged, as a change of its own with its own hand calculation.

## The hand calculation

It is in `tests/scenario_89_a_raid_recognised_by_experience.rs`, written before the code. It has no scenario file of its own: it plays the morning raid of scenario 19a with the attacked ship given each experience from 0 to 4 by the scenario's own key, and the first raid of scenario 6a, on a base, on the frozen step 1 figures with the two figures altered.
