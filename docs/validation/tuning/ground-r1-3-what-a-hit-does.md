# Ground round 1, step 3: what a hit does. Stopped, nothing moved

**The two fights want different values of one figure, so the step is stopped and the figures stay as they are** (decision 0109, "Rules of the round"). Ruled by the architect, Nov 15, 2026: `men_per_hit` stays 20. `hit_chance` and `defender_hit_chance` were not moved.

The full slice, 39 days, Nov 15, 2026. All trials are on step 1's figures, with `engagement_ratio` at 3 (step 2). Each is 1,000 runs, seeds 1 to 1,000, with `sim-cli validate`, labelled `r1-s3-<value>`.

## What was wrong

Both sides lose too few at the Tenaru: after step 1, 46 Americans (median) against 109 to 110, and 340 Japanese against 800 to 900. The record's losses of both fights on both sides are the measures' rows (engagement_outcomes 1936 to 1946, 1742, 1743, 1920, 1927, 1928, 1746, 2023 to 2048).

## The figures

| File | Figure | Now | The direction a source supports |
| --- | --- | --- | --- |
| `data/reference/ground.toml` | `men_per_hit` | 20 | up, for the Tenaru, where both sides lose too few |
| `data/reference/ground.toml` | `hit_chance`, `defender_hit_chance` | 0.3, 0.6 | for the balance between the sides, after `men_per_hit` |

## What was run

Medians, with the 5th, 25th, 75th and 95th where they decide the verdict.

| `men_per_hit` | Tenaru, American (109 to 110), judged | Tenaru, Japanese (800 to 900), judged | Ridge, American (134 to 253), reported | Ridge, Japanese (600 to 1,138), reported |
| --- | --- | --- | --- | --- |
| **20 (now)** | 14, 28, **46**, 63, 86: miss | 240, 300, **340**, 360, 420: miss | 82, 117, **148**, 182, 218 | 1,060, 1,140, **1,220**, 1,280, 1,380 |
| 30 | 14, 38, **63**, 90, 128: agrees | 360, 450, **510**, 540, 630: miss | 126, 179, **228**, 278, 332 | 1,590, 1,710, **1,830**, 1,920, 2,070: out |
| 40 | 14, 46, **81**, 116, 172: middle half | 480, 600, **680**, 720, 840: agrees | 169, 238, **304**, 374, 443 | 2,120, 2,280, **2,440**, 2,560, 2,760: out |
| 50 | 14, 56, **99**, 142, 214: middle half | 600, 750, **850**, 900, 916: middle half | 213, 298, **383**, 468, 555: inside the 90 percent, 84 runs | 2,644, 2,850, **3,000**, 3,193, 3,344: out |

At 50 both Tenaru measures are in the middle half of the runs, on one figure, with nothing else moved. At 50 the ridge's Japanese losses are three times the record's high end, and its American losses one and a half times.

## Where the two fights pull apart

| | The Tenaru | The ridge |
| --- | --- | --- |
| The value its losses want | about 50 (both measures in the middle half) | 20 or less for the Japanese (1,220 at 20, the record 600 to 1,138); about 25 for the Americans |
| At 50 | 99 and 850: both in the middle half | 383 and 3,000: the Japanese far out |
| At 20 | 46 and 340: both miss | 148 and 1,220 |

**The chances to hit do not part them.** A side's loss is the other side's shots times its chance times `men_per_hit`. So `defender_hit_chance` scales the Japanese losses of both fights alike, and `hit_chance` the American losses of both fights alike. The ratio between the fights stays where it is:

- **Japanese losses.** The ridge's over the Tenaru's is 3,090 over 916 = 3.37, the attackers' men ashore that the limit counts (step 2).
- **American losses.** It is about 3.6, the attackers' firing men, 2,781 over 778.
- **The record.** The Japanese ratio is 0.7 to 1.4 and the American ratio 1.2 to 2.3.

No figure of this step changes either ratio, so no value of any of them fits both fights. The cause is the one named at step 2: the game fights all of each side in the hex, and the record has a part of each engaged at the ridge.

## The ruling

No move. `men_per_hit` stays 20, and `hit_chance` 0.3 and `defender_hit_chance` 0.6 stay as they are, placeholders resting on no source. The Tenaru's two judged measures stay misses. Sent to the architect, Nov 15, 2026; the architect confirmed the stop.

Said plainly: one figure, `men_per_hit` at 50, would make the Tenaru pass. It would do so by making the ridge three times too bloody, and rule 1 of tuning forbids a figure that fits one fight by breaking the other.
