# Round 3: the fighters' chance to hit

Eastern Solomons, Nov 13, 2026. The air figures' round on the fight between aircraft, after decisions 0079, 0080 and 0087 are in. Ruled by the coordinating architect: one figure, `fighter_hit_chance`, tuned on the Eastern Solomons and checked on Rabaul against Guadalcanal. No height-and-warning rule in step 2 (the air plan's C3). **Written before any run of the round.**

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
