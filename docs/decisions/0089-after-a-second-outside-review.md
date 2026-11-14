# 0089: After a second outside review of the tuning

Status: decided by the architect on the designer's delegation, Nov 13, 2026 ("Go ahead and decide all matters that need decisions"). Written by the architect. It changes no rule, no figure, no measure's record and no verdict. It corrects what four records say about how well a figure is supported, and so the count of figures by status.

Summary: After a second outside review of the tuning: four records overstated a figure's support. `patrol_drawn_off_share` has no row behind either end (under 0080's cap both ends send 10 of 53 at the escorts); `patrol_before_attack_share`'s 0.22 to 0.61 is an interpretation, not a bound; air round 5's two shares are not comparable; `[base_hit] aircraft` and `fighter_false_claim_chance` are fitted, not sized; Aoba's 40 is one statement bounded by two others. The count becomes 3 sized and 6 fitted. No rule, figure, record or verdict changed

Standing: Decided by the architect

## What this is

An outside review of Nov 13, 2026 read the closed state of step 2 as an adversary, as one did before decision 0084. It is a lead and not a source: each point below was checked against the repository before it was acted on. Most of its twelve points repeat limits that decision 0084 and the closing page already state. Five say something new, and all five are right.

## Rulings

1. **`patrol_drawn_off_share` is not supported by rows as an end.** Decision 0087, item 11, compares the proportional split (10 escorts of 37 aircraft, 0.27 of the patrol) with one for one (10 of 53, 0.19) and concludes that the rows support 0. The comparison is wrong. Decision 0080 caps the shots at the escorts at the number of escorts. With the day's 53 fighters, 10 escorts and 27 bombers, a share of 0 gives 53 x 10 / 37 = 14, capped to 10; a share of 1 gives 10. Both ends send 10 of 53 at the escorts. The study's rows (12 of 38 fighters met the escort first) therefore do not tell the two ends apart in this case; they show only that an escort can occupy more than one defender, which the rule cannot do. The figure's status goes back to what decision 0084 gave it: fitted, not checked, with no row behind either end. That was the architect's error, in the ruling on air round 3.
2. **The range of `patrol_before_attack_share` is an interpretation, not a bound from a source.** The rows count the moment each pilot first engaged (4 before the push-over, 7 at it or in the dive, 7 in the retirement). The rule splits shots. A pilot who first engaged before the push-over may have fired after the release, and the study covers fighters attached to one carrier. So 0.22 to 0.61 is the architect's reading of pilot counts as shares of shots. The figure stays 0.4 and stays a placeholder; the range stays as the limit a later round may move within, named as an interpretation. The rows do support the direction: much of the patrol's fire fell during and after the attack.
3. **The anti-aircraft note compared two different things.** Air round 5 set the guns' share of kills in the runs (about 43 percent) beside a row that says about half the aircraft attacking were shot down by gunfire (`engagement_outcomes` 3953). The row's share is of attackers, not of kills. If few attackers fell to fighters, guns could kill half the attackers and nearly all of those killed. The two numbers cannot be called consistent. `anti_aircraft_hit_chance` stays 0.05: looked at, not moved, and no row bears on it.
4. **Two figures listed as "sized from rows" are fits.**
   - `[base_hit] aircraft` 0.06 (Rabaul round 1) was worked from about 2.5 aircraft destroyed on the ground over about 212 bomber sorties, through the placeholder bomb-hit chance of 0.2 and one bomb for each bomber. No row gives bombs that hit the field. The figure is the product of an assumed exposure, an assumed hit chance and the share that falls on aircraft; the rows size the product, not this figure.
   - `fighter_false_claim_chance` 0.7 (air round 4) was worked from 43 aircraft claimed, which is a row, against 16 real kills and 38 missed shots, which are the model's. No source divides the day's kills between fighters and guns. The figure is fitted to the model's own kills. It also carries what the rules lack: a fighter that never engaged claims nothing in the record and may claim a miss in the rules.

   Both become "fitted, not checked". Rabaul's agreement on United States claims at 0.7 is a check of that fit on a second battle, and is said so; it does not make the figure sized.
5. **Aoba's record at Cape Esperance is one statement of 40, bounded from both sides by two others.** The measure's reason says three statements "meet at 40". Only one of them says 40 (row 4034). One says "up to forty" (1598) and one "over 40" (1599), which disagree with each other. The reason is reworded to say that. The record stays 40 and stays marked weak; the check was made on it and nothing about the check changes.

## The count of figures by status

Of the 101 surface, search, air combat and derivation figures:

| | Before | Now |
| --- | --- | --- |
| Sized from rows | 5 | 3 |
| Fitted, not checked | 4 | 6 |
| Looked at, not moved | 19 | 19 |
| Placeholder | 61 | 61 |
| Read by no battle | 12 | 12 |

Sized from rows: `unready_gun_share` (Condition II), `unready_torpedo_share` (Bagley's four of eight; one ship's case, as its note says), and the divisor of the search radius (the Bureau of Aeronautics' method).

Fitted, not checked: `surprised_target_factor`, `experience_gun_strength`, `anti_aircraft_before_attack_share`, `patrol_drawn_off_share`, `[base_hit] aircraft`, `fighter_false_claim_chance`.

## What stands as it was

The review's other points are limits already recorded, and nothing changes for them: Cape Esperance is a check of transfer under inputs set with the outcome known (decision 0084, and the closing page); the two Savo Island effect sizes are fits (0084); the rate-of-fire gate failed and certified no rate (0084, finding 11); the radar flag's zero is an interpretation (0084, finding 7); orders chosen with the encounter known, and groups at rest that cannot fail a no-attack measure (0084, findings 10 and 9); each measure is judged alone, so totals can agree with the wrong victims (0084, finding 3); Rabaul's total of sorties agrees only because the late raids collapse (the fatigue round); and Rabaul is a tuning battle, not a check, for the two shares of decisions 0079 and 0080.

One point the review makes that the closing page should carry in its own words: Rabaul round 1 changed how many fighters survive on the ground, and so the fights that the check of the fighter and claim chances reads. Rabaul checks one figure held still at a time; it is not an independent test of the air model as a whole.

## What changes, and where

Documents and comments only:

- Dated notes in decision 0087 (items 11 and 12), air round 3's record, air round 4's, air round 5's and Rabaul round 1's, each pointing here. Their text is not deleted.
- The notes above the four figures in `data/reference/air_combat.toml` and the base-hit table, and the reason of `gun_hits_on_aoba` in `data/validation/cape_esperance.toml` (the reason only: no record, row or flag).
- The two "figures closed" pages, `docs/validation/step2-close.md` (the count, the two lists, and the sentence on Rabaul as a check) and the status line of `CLAUDE.md` that gives the count.
- Comments in data files change their hashes, so what pins those hashes is remade in the same pull request.

The designer's acceptance of the named misses on Nov 13, 2026 rested on the page as it then read: 5 figures sized and 4 fitted. He is told of the correction. No miss, no edge and no verdict among the 62 judged measures changes.
