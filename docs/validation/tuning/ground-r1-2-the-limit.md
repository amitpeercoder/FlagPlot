# Ground round 1, step 2: the limit on engaged men. Stopped, nothing moved

**The two fights want different values of one figure, so the step is stopped and the figure stays as it is** (decision 0109, "Rules of the round": the rule is wrong, not the figure). Ruled by the architect, Nov 15, 2026: `engagement_ratio` stays 3.

The full slice, 39 days, Nov 15, 2026. All trials are on step 1's figures (`docs/validation/tuning/ground-r1-1-who-can-fire.md`). Each is 1,000 runs, seeds 1 to 1,000, with `sim-cli validate`, labelled `r1-s2-<value>`.

## What was wrong

A side fires with no more men than `engagement_ratio` times the other side's fighting men ashore in the hex (decision 0105). At 3, the Marines fire with 2,748 men at the Tenaru against 916 Japanese ashore, and with 9,270 at the ridge against 3,090.

The rows say fewer were engaged:

- **The Tenaru.** The Ichiki group was about 900 (engagement_outcomes 1924). It was met by the 2d Battalion, 1st Marines in line, the 1st Battalion from reserve, a platoon of light tanks and a battalion of howitzers (1935).
- **The ridge.** About 2,000 attacked of the 3,090 ashore (2013; land_unit_facts 311). They met the raider and parachute battalions with the artillery (2014, 2019; land_unit_facts 1018, 1004).

No row counts the American men engaged. What the rows support is the direction, down: about two battalions with guns against about one at the Tenaru, and about one to one at the ridge, of those who attacked.

## The figure

| File | Figure | Now | The direction a source supports |
| --- | --- | --- | --- |
| `data/reference/ground.toml` | `engagement_ratio` | 3 | down, at both fights (rows above). The loader refuses a value below 1 |

## What was run

Medians, with the 5th to 95th. Losses are at (2,-4), the Tenaru on days 16 to 19 and the ridge on days 37 to 39.

| `engagement_ratio` | Tenaru, Japanese (800 to 900), judged | Tenaru, American (109 to 110), judged | Ridge, Japanese (600 to 1,138), reported | Ridge, American (134 to 253), reported |
| --- | --- | --- | --- | --- |
| 1 | 100 (60 to 160): miss | 46 (14 to 86): miss | 400 (300 to 500): out | 148 (82 to 218) |
| 1.5 | 160 (100 to 220): miss | 46: miss | 600 (480 to 720): middle half | 148 |
| 2 | 220 (160 to 300): miss | 46: miss | 800 (680 to 960): middle half | 148 |
| 2.5 | 280 (200 to 360): miss | 46: miss | 1,020 (860 to 1,160): middle half | 148 |
| **3 (now)** | **340 (240 to 420): miss** | **46: miss** | **1,220 (1,060 to 1,380): inside the 90 percent** | **148** |
| 4 | 440 (340 to 520): miss | 46: miss | 1,620 (1,420 to 1,800): out | 148 |
| 6 | 660 (540 to 780): miss, 27 runs hold it | 46: miss | 1,700 (1,500 to 1,900): out | 148 |
| 8 | 880 (740 to 916): middle half, 478 runs | 46: miss | 1,755 (1,560 to 1,953): out | 148 |
| 12 | 916 in every run: miss (the whole echelon) | 46: miss | 1,775 (1,595 to 1,974): out | 148 |

The American losses do not move: the Japanese are the smaller side and are never cut. The base-holding measures agree at every value, and the American attack of days 1 and 2 still takes Lunga.

## Where the two fights pull apart

| | The Tenaru | The ridge |
| --- | --- | --- |
| The value its Japanese losses want | about 8 (880, in the middle half) | about 1.5 to 2.5 (600 to 1,020, in the middle half) |
| At the Tenaru's value, 8 | judged measure in the middle half | 1,755, out |
| At the ridge's value, 2 | 220, a miss further out than now | 800, in the middle half |
| The direction the rows support | down | down |

The step cannot be done by the rules. A move down is supported, and it brings the ridge in, but it takes the judged Tenaru measure further from its record. A move up brings the Tenaru in, but no source supports it, and it takes the ridge out.

**Why this is the rule and not the figure.** While the limit binds, which it does at both fights for any value up to 4, the defender's fire is the ratio times every man the attacker has ashore in the hex. The Japanese loss at the ridge over that at the Tenaru is therefore 3,090 over 916, which is 3.37, whatever the ratio. The record has the two about equal: 600 to 1,138 at the ridge against 800 to 900 at the Tenaru.

The limit counts the men ashore, not the part of them that attacked or the part of the defender that met them. At the ridge about 2,000 of 3,090 attacked, and the game cannot say so. The two things the record has and the game does not are named in `docs/validation/full-slice-39-days-ground-r1.md`, "What the round could not reach".

## The ruling

No move: `engagement_ratio` stays 3, a placeholder resting on no source. Sent to the architect with the table above, Nov 15, 2026; the architect confirmed the stop.
