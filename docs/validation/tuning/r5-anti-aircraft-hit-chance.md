# Air round 5: the guns' chance to hit. Looked at, not moved

**How this record was made.** It was written after the other air rounds, from numbers already in hand. No grid was run. Air round 3 left this figure to "the next round" (`r3-fighter-hit-chance.md`), and that round was never held. The gap was found while the air figures were being listed (`air-figures-closed.md`), and this record closes it. One batch of 1,000 runs on the live figures was made to confirm the verdicts below. It matches the round 4 report: the measures listed here are unchanged, and one other measure's 95th percentile differs by one.

Eastern Solomons, Nov 13, 2026. The figure is one of the air combat figures. They are tuned on the Eastern Solomons and checked on Rabaul against Guadalcanal. **Ruled by the coordinating architect: looked at, not moved.**

## The figure

| File | Figure | Now |
| --- | --- | --- |
| `data/reference/air_combat.toml` | `anti_aircraft_hit_chance` | 0.05 (placeholder) |

It is the chance that one anti-aircraft shot of a ship or a base shoots down an aircraft of a raid, times the protection factors of the raid's type (chapter 5, `docs/rules/05-air-operations.md`).

It is read by:
- ships' guns at the Eastern Solomons, where Task Force 16's guns fire at the Japanese strike;
- Henderson Field's guns at Rabaul against Guadalcanal (`anti_aircraft = 8` in scenario 6).

`anti_aircraft_before_attack_share` (0, decision 0079) sets when in the attack the guns fire. That is not this figure.

## The measures it bears on, and their verdicts now

These are from `docs/validation/eastern-solomons-r4.md`; the batch on today's live figures confirms them.

| Measure | Record | Median | Verdict |
| --- | --- | --- | --- |
| `bomb_hits_on_enterprise` | 3 | 3 | agrees |
| `japanese_aircraft_attacking_enterprise_force` | 37 | 37 | agrees |
| `japanese_carrier_aircraft_lost_to_enemy_action` | 29 to 32 | 26 | agrees |
| `japanese_carrier_aircraft_lost_all_causes` | 35 to 54 | 29 | agrees |
| `japanese_aircraft_claimed_per_hundred_shot_down` | 171 to 261 | 440 | agrees, at the edge (its divisor is the named miss of American losses, `r3-us-losses-named-misses.md`) |

**The guns' claims.** At `anti_aircraft_false_claim_chance` 0.05 they come to about 24 a run. That is the low end of the rows: 24 by Enterprise's captain for Task Force 16 (255), and up to 41 by the ships' own highest figures (`r4-claims.md`).

**At Rabaul**, Henderson Field's guns fire at the raids. `japanese_bombers_lost_to_enemy_action` misses high (29 against 12), and that miss is named on the fighters' side: no height-and-warning rule (`r3-fighter-hit-chance.md`).

## Who shoots down what

From round 4's split by firer (`r4-claims.md`, "By firer, at the Eastern Solomons"), in the runs where each firer fired:
- Task Force 16's guns make 12.3 real kills;
- the carrier fighters, VF-5 and VF-6, make 16.0.

So the guns make about 43% of the kills over the Enterprise force.

**The one row that speaks to the share** is `engagement_outcomes` 3953. It says that about half the planes attacking were shot down by gun fire, with very few by fighters. That is the consensus of the gunnery officers, from CINCPAC's gunnery bulletin on the Coral Sea. The row is:
- low;
- unconfirmed;
- an estimate of enemy losses, and so a claim;
- from another battle;
- silent on whether the fire came before or after the bombs were released.

The rules' 43% is consistent with it. The row cannot size a chance to hit: it gives no count of shots or of kills.

**The rounds-per-gun rows** are 3947 to 3952. They give Yorktown's and a destroyer division's average expenditure per gun on the last day at the Coral Sea: 5-inch 50 and 65, 1.1-inch 180 and 500, 20 mm 330, .50 calibre 880. They size no chance either. The bulletin gives no count of aircraft shot down to set them against, as each row's note says. They also count rounds by calibre, where the rules count shots of a ship's or a base's anti-aircraft rating.

## Conclusion

Looked at, not moved. `anti_aircraft_hit_chance` stays 0.05, **a placeholder that no row sizes.**
- The measures it bears on agree, and `bomb_hits_on_enterprise` sits at its record.
- The guns' share of the kills over the Enterprise force, about 43%, is consistent with the one weak row, about half (3953).
- No row gives shots against kills, so a grid would have nothing to aim at but the measures, which already agree. That would be a fit with no source, and tuning rule 4 forbids it.

It is not a real round with a grid, for that reason. It would become one if a row turned up that gives a force's gun kills, or its rounds and its kills together, for a battle the slice holds.
