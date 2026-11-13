# Round 3: the fighters' chance to hit. Nothing moved

Eastern Solomons, Nov 13, 2026. The air figures' round on the fight between aircraft, after decisions 0079, 0080 and 0087 are in. Ruled by the coordinating architect: one figure, `fighter_hit_chance`, tuned on the Eastern Solomons and checked on Rabaul against Guadalcanal. No height-and-warning rule in step 2 (the air plan's C3). **Written before any run of the round;** the sections from "What was run" on were added after it. **Ruled by the coordinating architect, Nov 13, 2026: no move.**

## The figure

| File | Figure | Now |
| --- | --- | --- |
| `data/reference/air_combat.toml` | `fighter_hit_chance` | 0.3 (placeholder, decision 0027) |

Nothing else moves in this round:
- The protection factors (`pilot_armour_factor`, `self_sealing_tanks_factor`, 0.75 each) have no source and stay.
- The guns' chance is the next round's.
- The claim chances come after both, because a claim rests on the real kills and on the fighters that never engaged (decision 0087, 10).

**Its kind:** an air combat figure, tuned on the Eastern Solomons and checked on Rabaul against Guadalcanal. Rabaul's result never moves it.

## The rows that give the direction

The first wave at Enterprise was 27 dive bombers and 10 fighters (`engagement_outcomes` 290, 1159, 1164).

- **It lost most of itself.**
  - Shokaku's form: 11 (1160). Zuikaku's: 8 dive bombers (1165) and 3 fighters (1166).
  - The tabular record: 18 dive bombers and 6 fighters, 24 in all (291).
  - Kojinsha: Shokaku 10 dive bombers and 3 fighters (287, 288).
- **The measure.** `japanese_carrier_aircraft_lost_to_enemy_action` has 29 to 32. It adds Ryujo's 5 to 7 over Guadalcanal (1172, 271).
- **No row splits these losses between the fighters and the guns.**
  - The guns opened only as the dives began (events 222, 1242 to 1245).
  - The fighters met the wave from 33 miles out (events 218, 219).
  - 26 of the 38 fighters in Simpler's study engaged the bombers or the escort (3979 to 3982).
- **The American side.** 13 to 14 aircraft were lost to the enemy (305, 311, 700). Of these, only the patrol's losses to the escort (about 5 to 6: 219, 220, 240) and the Marines' over Guadalcanal (3 to 4: 388, 700) are inside the rules (the air plan, A2).

So the direction is up if the model's Japanese losses to the enemy are below the record, and down if they are above. On the baseline the round starts from (`eastern-solomons-baseline.md`, remade with decision 0087) they are 2 to 35, median 26, against 29 to 32. **The direction the rows give is up.** No row gives a chance per fighter, so no row gives a size. The size is the one that brings the record into the middle half of the runs.

## The measures that judge it

The Eastern Solomons:
- Japanese carrier aircraft lost to enemy action (29 to 32), and to all causes (35 to 54).
- US aircraft lost to enemy action (13 to 14; its reachable part is about 7 to 9, the rest being a named gap).
- Bomb hits on Enterprise (3).
- The Japanese bombers left to attack (20 to 27, reported).
- The Japanese claims per hundred, which agree today and must not be broken by more real kills.

## What the check on Rabaul must show

Rabaul against Guadalcanal is not tuned on. After the move it is run, and the round reports:
- Japanese fighters lost (6 to 10);
- American fighters lost to enemy aircraft (7 to 23);
- Japanese bombers lost (12);
- F4Fs ready on day 21 (11).

Japanese bombers lost is expected to miss high whatever the move. That is the C3 named miss: the patrol at Henderson Field fought high-level bombers after coastwatcher warning, and no row sizes how many of its fighters made contact. Raising the chance makes it worse, and that is recorded and not mended.

## What stops the round

The round stops, and the figure stays at 0.3 with the reason written here, if any of these holds:
1. No value in the direction brings the Japanese losses to enemy action into the middle half (25th to 75th percentile) of the runs. At most the round takes it into the middle 90 percent, and says it is left at the edge.
2. The value that does so turns bomb hits on Enterprise, or the Japanese claims per hundred, from agreeing into a miss.

What the round does **not** stop for is a miss at Rabaul. By the spec ("Tuning": "A tuning round does not move one of these to mend a miss at Rabaul. A miss there goes back to the Eastern Solomons, or to the rule"), a check that fails is reported with its cause and goes to the architect. American fighters lost at Rabaul is at the record's ceiling of 23 in the middle run before the round, so any move up is expected to take it over, and that is stated, not chased.

The move is brought to the architect with its evidence before it is committed as the live figure.

## What was run

1,000 runs of each battle on scratch copies of the figures (`--reference`), at five values. Only `fighter_hit_chance` differed between them.

The Eastern Solomons, the battle tuned on:

| Chance | Japanese lost to enemy action, record 29 to 32: 5th to 95th / 25th, median, 75th / runs holding | Japanese lost to all causes, runs holding | Bomb hits on Enterprise, median (runs holding 3) | Japanese claims per hundred | US lost to enemy action, median |
| --- | --- | --- | --- | --- | --- |
| 0.3 | 2 to 35 / 4, 26, 31 / 234 | 189 | 3 (105) | agrees | 4 |
| 0.35 | 3 to 37 / 5, 29, 33 / 224 | 201 | 2 (104) | agrees | 5 |
| 0.4 | 3 to 38 / 6, 31, 35 / 146 | 248 | 2 (109) | at the edge | 6 |
| 0.45 | 4 to 40 / 7, 34, 37 / 57 | 305 | 2 (117) | at the edge | 6 |
| 0.5 | 4 to 41 / 7, 36, 38 / 11: miss | 339 | 2 (123) | at the edge | 7 |

The 25th percentile is low at every value because in about a quarter of the runs no Japanese strike reaches the American carriers. The American claims per hundred miss at every value and fall as the chance rises (148 to 132); that is the claims round's.

Rabaul against Guadalcanal, the check:

| Chance | Japanese fighters lost, 6 to 10 | American fighters lost, 7 to 23 | Japanese bombers lost, 12 | F4Fs ready on day 21, 11 (runs holding) | VMF-223 ready on day 10, 4 to 5 |
| --- | --- | --- | --- | --- | --- |
| 0.3 | 12, agrees | 23, agrees | 26, miss | 6 (40), miss | at the edge |
| 0.35 | 13, at the edge | 27, at the edge | 28, miss | 4 (18), miss | at the edge |
| 0.4 | 15, at the edge | 29, at the edge | 30, miss | 3 (3), miss | miss |
| 0.45 | 16, at the edge | 31, miss | 32, miss | 2 (0), miss | miss |
| 0.5 | 17, miss | 32, miss | 34, miss | 2 (0), miss | miss |

## The ruling

No move: `fighter_hit_chance` stays 0.3.

- **No size.** The rows give a direction and no size.
- **The aim is already met.** The record already lies in the middle half of the runs at 0.3, and the runs holding it are highest there.
- **0.35 was declined.** It would put the median on the record, and it was declined because the tuning battle does not need it and nothing sizes it.
- **Still a placeholder.** The figure remains a placeholder that no source sizes.

**The named miss.** Rabaul's Japanese bombers lost (26 against 12) and F4Fs ready on day 21 (a median of 6 against 11) are the named miss of the air plan's C3. There is no height-and-warning rule in step 2, and no row counts the fighters in contact at Henderson Field. They are not chased.

## The reports

- `docs/validation/eastern-solomons-r3.md`: judged 15, agree 11, at the edge 1, miss 3.
- `docs/validation/rabaul-guadalcanal-air-r3.md`: judged 10, agree 4, at the edge 2, miss 4.

Both are on the figures in use, which this round did not change: the same as the reports of decision 0087, labelled for the round.
