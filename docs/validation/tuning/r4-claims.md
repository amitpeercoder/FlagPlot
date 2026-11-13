# Round 4: claims

The air battles, and the claim figure of surface fights, Nov 13, 2026. This is the last mechanism in the order of tuning, after the fight, the guns and the attack, because a claim is the real count plus false ones and a ratio of claims rests on the real count. **Written before any run of the round.** The moves are brought to the coordinating architect with their evidence before any becomes live, one figure at a time.

## The figures

| File | Figure | Now | What it is |
| --- | --- | --- | --- |
| `air_combat.toml` | `fighter_false_claim_chance` | 0.1 | The chance each whole fighter shot that missed is claimed as an aircraft shot down (patrol and escorts, both sides) |
| `air_combat.toml` | `anti_aircraft_false_claim_chance` | 0.05 | The same, for each whole gun shot that missed (ships and bases, both sides) |
| `air_combat.toml` | `bomb_false_claim_chance` | 0.1 | The chance each bomb or torpedo that missed is claimed as a hit on a ship, or, at a base, as an aircraft destroyed on the ground |
| `surface_combat.toml` | `claimed_sunk_fire` | 30 | The fire damage at which a ship still afloat is claimed sunk, in a surface fight or an air raid |

All four are placeholders (decision 0056, decision 0014). Each is one figure for both sides.

**Kinds:**
- The first three are air figures, tuned on the Eastern Solomons and checked on Rabaul against Guadalcanal.
- `claimed_sunk_fire` is one of the five figures shared between surface fights and bomb hits (tuning rule 9). If it moves, Savo Island's report is run again with the round. Cape Esperance is not run or looked at.

## The measures, what each divides by, and whether it can be tuned to

| Measure | Battle | Record | Judged | Divides by | Can the round tune to it? |
| --- | --- | --- | --- | --- | --- |
| `us_aircraft_claimed_per_hundred_shot_down` | Eastern Solomons | 237 to 362 | yes | Japanese aircraft really shot down, floatplanes included | **Yes.** Japanese losses to enemy action agree (median 26 against 29 to 32, record in the middle half), so the divisor is sound |
| `japanese_aircraft_claimed_per_hundred_shot_down` | Eastern Solomons | 171 to 261 | yes | American aircraft really lost to enemy action | **No.** The divisor is the named loss miss: about 4.6 a run against 13 to 14 (`r3-us-losses-named-misses.md`). The ratio is over a count the rules cannot reach. It is reported each step and does not stop the round |
| `us_hits_claimed_per_hundred_scored` | Eastern Solomons | 50 to 140 | no | Bomb and torpedo hits on Japanese ships | No: not judged (the hits are disputed twofold) |
| `us_aircraft_claimed_per_hundred_shot_down` | Rabaul | 250 to 382 | yes, a check | Japanese aircraft really shot down | **A check only, and partly distorted.** The divisor carries the bombers-lost named miss (26 against 12). The fighters agree (12 against 6 to 10) |
| `japanese_aircraft_claimed_on_ground_per_hundred_destroyed` | Rabaul | 2,066 to 4,600 | no | Aircraft really destroyed on the ground | No: not judged, and the divisor is a miss (12 to 13 against 2 to 3) |
| `allied_ships_claimed_sunk` | Savo Island | 0 | yes | (a count) | Read only if `claimed_sunk_fire` moves |
| `japanese_ships_claimed_sunk_per_hundred_sunk` | Savo Island | 350 | no | Allied ships sunk | No: not judged; the rules cannot pass 225 |

## The rows that size each figure

"Rows" are `engagement_outcomes`.

**`fighter_false_claim_chance`.** American fighters at the Eastern Solomons:
- 43 claimed by the two carrier squadrons (257; 238, 239: VF-5 17 and VF-6 26).
- 16 (310) to 21 (309) by the Marines over Guadalcanal.

The real kills by fighters alone are not in any row: the first wave's 22 to 25 lost are not split between fighters and guns (291). So this chance and the guns' are sized together, on the measure, with each one's claims compared by firer in the runs. The rows say both sides overclaimed about three to one (`us-side-air-losses.md`; the aviation annex's 2.9 to 1, 26 August to 14 September).

**`anti_aircraft_false_claim_chance`.** Ships' guns at the Eastern Solomons:
- 24 by Enterprise's captain for Task Force 16 (255).
- Up to 41 by the ships' own highest figures (215, 244, 252, 254, 710, 711, 713).

At Rabaul, as a check: about six claimed by Henderson Field's guns over the period (1786, 1333 and 1804, 1835, 1851).

**`bomb_false_claim_chance`.** No judged measure reads it. The rows pull two ways:
- Hits on Ryujo were claimed at about one for each scored (259, 261, 263 against 258, 260, 668, 1177).
- Aircraft on the ground were claimed at 20 or more for each destroyed (985 to 1106 against 1266, 1330).

It is not moved this round. It is written up as one figure carrying two kinds of claim that the record prices differently (the air plan, B19).

**`claimed_sunk_fire`.** Savo Island's Allied claim of no ship sunk agrees (814 runs of 1,000 hold it, after round 2). No row gives the fire at which a crew believed an enemy ship lost. It is not moved unless the air round shows the raid claims need it, and no air measure judges a claim of ships sunk.

## The order

1. `fighter_false_claim_chance`, then `anti_aircraft_false_claim_chance`, each on 1,000 runs of figure copies.
   - Each value is read on the American claims at the Eastern Solomons, by firer: the carrier fighters, the Marines and the ships' guns, against 43, 16 to 21 and 24 to 41.
   - Each is checked on the Rabaul claims and the Henderson guns' claims.
   - The Japanese claims at the Eastern Solomons are reported at each value, not tuned to.
2. `bomb_false_claim_chance` and `claimed_sunk_fire`: no move proposed, for the reasons above.

## What stops the round

The round stops for a figure, and leaves it as it is with the reason written here, if any of these holds:

1. **No value fits all three firers.** No value brings the American claims per hundred at the Eastern Solomons into the middle half of the runs without taking one firer's claims beyond its rows: the fighters beyond 43 plus the Marines' 21, or the guns beyond 41.
2. **A judged measure turns to a miss.** A value that does so turns a judged measure of the tuning battle from agreeing into a miss. The Japanese claims ratio is excluded, since its divisor is the named miss; it is reported.
3. **A chance above 1.** A value above 1 would be needed. A claim of more than one aircraft for a missed shot is not what the figure means.

A miss in the check at Rabaul is reported with its cause and does not move a figure (tuning, "Rabaul is checked, not tuned").
