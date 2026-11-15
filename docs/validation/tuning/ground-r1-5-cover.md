# Ground round 1, step 5: cover. A report, no figure moved

**This step moves no figure** (decision 0109's table). It reports what cutting each unit's small share to a whole man does to the share of a raid's men that cover saves, and the architect rules whether the rule is changed.

The full slice, 39 days, on the live figures after step 1 (which does not touch cover), Nov 15, 2026. The runs are the event logs of seeds 1 to 1,000 (`export --seed N`). The rule's arithmetic was reproduced exactly, in fractions, and checked against every event (below).

## The rule, and where it cuts

A call is one raid's attack on a base, with h hits on men (rulebook chapter 7, "Men lost to fire"). It works in four stages:

1. **The loss.** The call's loss is h x `men_lost_per_base_hit` (10).
2. **Shared out.** The loss is shared between the side's units in the hex in proportion, cut to whole men, with the remainder taken a man at a time.
3. **Spread over the hits.** Each unit's part P is spread over the hits: the k-th piece is floor(P x k / h) - floor(P x (k - 1) / h).
4. **Cut by cover.** Each piece is cut by cover: floor(piece x (1 - cover/100 x `cover_from_fire`)), with `cover_from_fire` at 0.9.

The figure states that cover C saves C/100 x 0.9 of the men. The cut in stage 4 rounds each piece down, and the pieces are small: at Lunga the six to nine Allied units share 10 men a hit among about 12,000 men. A piece of 1 man is cut to 0 by any cover above 0, and a piece under 10 is cut to 0 by full cover.

## The check

- **Every raid event matched.** Every one of the 18,159 `men_lost_to_fire` events in the 1,000 runs was matched by the recomputed shares, men in the hex and hits. All 15,755 events that record a cover were recomputed from `men_before_cover`, the hits, the cover and the wear (0.2 a hit), and every one equals the men the engine took.
- **What the runs held.** There were 5,642 calls, every one a raid on Lunga (base 1, hex (2,-4)), and no bombardment and no raid on a Japanese base.
- **Hits per call:**

  | Hits | 1 | 2 | 3 | 4 | 5 | 6 |
  | --- | --- | --- | --- | --- | --- | --- |
  | Calls | 3,428 | 1,509 | 528 | 132 | 41 | 4 |

- **The parts before cover.** Of 22,529 parts, 18,020 are of 1 to 4 men. 4,373 are of 1 man, 5,330 of 2 and 7,500 of 4.
- **The cover met** is 30 in almost every case: the fieldworks at their limit, less a little wear.

## What the runs give

| | Men before cover | Men lost | Share saved | Share the figure states |
| --- | --- | --- | --- | --- |
| Every Allied unit at Lunga, 1,000 runs | 87,870 | 42,422 | **0.517** | 0.261 (0.27 at cover 30, less wear; units with no loss taken at the cover of their neighbours, 30) |

The units that lost nothing record no event. Their men were rebuilt from the state, so their parts count. Their cover is not in the log, and only the stated share depends on taking it as 30.

## What the rule gives at each level of cover

The same 5,642 calls, units and hits as in the runs, with the cover set at each call's first hit and worn 0.2 a hit after:

| Cover | Share the figure states | Share the rule gives | Men lost, of 87,870 |
| --- | --- | --- | --- |
| 10 | 0.090 | 0.383 | 54,256 |
| 20 | 0.180 | 0.383 | 54,256 |
| 30 | 0.270 | 0.549 | 39,619 |
| 50 | 0.450 | 0.583 | 36,682 |
| 100 | 0.900 | 1.000 | 0 |

By the size of a unit's part (share saved):

| Part P | Parts | Men before | Cover 10 | 20 | 30 | 50 | 100 |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1 to 4 | 18,020 | 47,484 | 0.474 | 0.474 | 0.619 | 0.619 | 1.000 |
| 5 to 9 | 3,099 | 23,265 | 0.276 | 0.276 | 0.478 | 0.535 | 1.000 |
| 10 to 19 | 1,403 | 16,966 | 0.273 | 0.273 | 0.453 | 0.547 | 1.000 |
| 20 and over | 7 | 155 | 0.271 | 0.271 | 0.458 | 0.542 | 1.000 |

One call on the Allied units at Lunga as they stand on day 20 of seed 1942: units 1 (4,292 men), 2 (4,546), 3 (1,370), 4 (658), 7 (1,074) and 9 (123), 12,063 in all.

| Hits | Men before cover | Lost at cover 10 (share saved) | 20 | 30 | 50 | 100 |
| --- | --- | --- | --- | --- | --- | --- |
| 1 | 10 | 7 (0.300) | 7 (0.300) | 5 (0.500) | 5 (0.500) | 0 (1.000) |
| 2 | 20 | 12 (0.400) | 12 (0.400) | 8 (0.600) | 8 (0.600) | 0 (1.000) |
| 3 | 30 | 18 (0.400) | 18 (0.400) | 13 (0.567) | 12 (0.600) | 0 (1.000) |
| 5 | 50 | 28 (0.440) | 28 (0.440) | 21 (0.580) | 18 (0.640) | 0 (1.000) |

## What it says

- **Cover saves too much, and does not grade.** At the levels a unit can dig (10 to 30), cover saves two to four times the share the figure states. Cover 10 and 20 save the same, and so, near enough, do 30 and 50. Full cover saves every man, where the figure states nine in ten.
- **The cause is the cut, not the figure.** Even a part of 20 men or more saves 0.27 at cover 10 against 0.09 stated: each hit's piece of a part is small, and is cut again.
- **A smaller figure would not mend it.** The same floor applies to every piece, so the grading of the scale is lost below a piece of about 10 men.

**The same cut in the fight**, for comparison. A defender's part of the hex's loss is cut by floor(part x (1 - cover/100 x `cover_in_fight`)), with `cover_in_fight` at 0.5. Over the Allied units at (2,-4) on days 16 and 37, of the events that record a cover, 138,913 men before cover give 113,296 lost: 0.184 saved against 0.149 stated. The fight's parts are larger, 2 to 26 men, so the floor costs less. The units whose part was cut to 0 record no event and are not counted, so the true share saved is somewhat higher.

## For the architect

What the cut does is stated above; this step stops here and the ruling is the architect's. The figures `cover_from_fire` (0.9) and `cover_in_fight` (0.5) are unchanged. Two ways the rule could be changed, named and not chosen:

- **Cut once.** Cut each unit's whole part once, not each piece.
- **Draw the parts of a man.** Round a part of a man by a draw, as the part of a shot is rolled at the part of its chance.
