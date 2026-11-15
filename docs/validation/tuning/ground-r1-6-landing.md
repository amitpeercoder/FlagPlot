# Ground round 1, step 6: the landing. Two figures moved

**These figures have no check battle.** They are fitted on the full slice and nothing checks them. Each is recorded as fitted on the two fights of the slice and not checked, and stays a placeholder.

The full slice, 39 days, Nov 15, 2026. All trials are on step 1's figures, with steps 2 to 4 left as they were. Each is 1,000 runs, seeds 1 to 1,000, with `sim-cli validate`, labelled `r1-s6-<value>`.

## What was wrong

When the transports sail at the end of day 3, the Americans have 68 days of food ashore at Lunga (3,725 t for 10,880 men). The record has 17 to 37 (land_unit_facts 415, 414).

The record also has less than half the supply landed in 26 hours of unloading. Of the sixty days' supplies the division had embarked, less than half had been unloaded (465). The 26 hours are 773. And 10,000 troops were ashore by 2000 on the first day (688).

## The figures

| File | Figure | Before | The direction a source supports |
| --- | --- | --- | --- |
| `data/reference/ground.toml` | `unprepared_landing_share` | 0.5 | down: less than half the supply landed (465, 773) |
| `data/reference/ground.toml` | `handling_t_per_man` | 0.25 | down, so that men still land in a day: 10,000 ashore by 2000 on day 1 (688) |

**What they do.** A force with no loading unloads at `unprepared_landing_share` of its ships' rate while it lands men, supply as well as men (decision 0095). Every ton of the ships' work moves 1 / `handling_t_per_man` men. Cutting the share alone slows the men too. With the share at 0.15 and the handling figure as it was, only 1,960 men are ashore at Lunga on day 1 and 7,208 on day 3. The record has them ashore on the first day. So the two move together.

**The condition of the step** (the architect's, Nov 15, 2026): a value is kept only if no American is left unfed later in the slice because of it.

## What was run

Each value is the share, then the tons per man. "Men at Lunga" is the median of the Allied men ashore at (2,-4) at the end of day 1; every run has 10,880 there by the end of day 2, but where shown. "Unfed" is the Allied men lost unfed over the 39 days, median and greatest.

| Share, tons per man | Food at Lunga, days (17 to 37), reported | Men at Lunga, end of day 1 | Americans unfed later | Least Allied supply at Lunga after day 3 |
| --- | --- | --- | --- | --- |
| 0.5, 0.25 (before) | 68 | 6,079 | 0 | 2,696 t |
| 0.25, 0.1 | 49 to 50 | 7,480 | 0 | 1,693 t |
| 0.2, 0.25 | 17 | 2,578 (9,384 by day 3) | 3,512 | |
| 0.15, 0.25 | 19 | 1,960 (7,208 by day 3) | 3,039 | |
| 0.2, 0.1 | 37 | 6,079 | 0 | 1,028 t |
| 0.15, 0.1 | 25 | 4,639 | 1,664 | |
| 0.12, 0.1 | 18 | 3,775 | 2,964 | |
| 0.15, 0.05 | 35 | 8,840 | 0 | 888 t |
| 0.14, 0.05 | 32 | 8,296 | 81 (greatest 111) | |
| 0.13, 0.05 | 30 | 7,752 | 852 | |
| 0.12, 0.05 | 27 to 28 | 7,208 | 1,455 | |
| 0.10, 0.05 | 23 | 6,079 | 2,394 | |
| 0.15, 0.02 | 40 to 41 | 10,880 | 0 | 1,204 t |
| 0.14, 0.02 | 38 | 10,880 | 0 | 1,070 t |
| **0.13, 0.02 (kept)** | **35 to 36, median 36** | **10,880** | **0** | **936 t** |
| 0.12, 0.02 | 33 | 10,880 | 0 (greatest 17) | 801 t |
| 0.10, 0.02 | 28 | 10,880 | 1,401 | 533 t |

No value changes a fight: the Tenaru's and the ridge's losses are those of step 1 in every trial. The base-holding measures agree in every trial, and the airfield is built on day 11 in every trial.

## The values kept, and why

`unprepared_landing_share` **0.13** and `handling_t_per_man` **0.02**. The food at Lunga when the transports sail falls from 68 days to 36 (35 to 36 over the runs), inside the record of 17 to 37.

- **The men land on day 1.** All 10,880 men of the Lunga landing are ashore at the end of day 1 in every run, where before the round 6,079 were (the record has 10,000 by 2000 that day).
- **Less than half the supply lands.** The transports land 1,529 t at Lunga and 1,002 t at Tulagi in two days. They sail on day 3, by the scenario's order, with 1,741 t and 820 t still aboard, 53 and 45 percent of their cargo.
- **No American goes hungry.** No American is unfed on any day of the 39, in any run.

**Why not lower.** Every value tried below this one that gives fewer days of food leaves Americans unfed later in the slice, in some runs or in all, so the architect's condition rules it out. With the share at 0.12 and 0.02, 17 men are lost unfed in the worst run, and at 0.14 and 0.05, 81 in the median.

**What binds is Tulagi.** In the kept runs, the least Allied stock at Lunga after day 3 is 936 t, about 14 days of food, in the worst run. At Tulagi, (3,-5), it is 88 t (85 to 94) on day 39, under five days. A lower share lands less at both places, and Tulagi's men are the first to go short before the slice's later supply runs.

The record's middle, about 27 days, cannot be reached without starving someone. The scenario's later supply, the runs after 9 August, is set from rows and is not a figure of this round. The measure is reported, not judged.

## What it rests on

The direction of the share: rows 465 and 773, less than half landed. The tons per man: row 688, the men ashore on the first day, which is why the handling figure moves the whole way to 0.02. A man's handling, now 20 kg of a ship's work, rests on nothing finer. Both stay placeholders, fitted on the slice and not checked.

## What it changed elsewhere

- **The Allied score.** In the slice the objective for cargo landed at Lunga falls with the supply landed: in the scenario's seed, 4,390 t against 6,195. Over the round the Allied margin falls from 101.6 to 92.2 (median), with step 1's higher American losses in the fight. It is still a decisive victory in every run.
- **The placeholder scenarios that land men** (21, 27) land faster. Their fights and losses are as they were (`docs/validation/full-slice-39-days-ground-r1.md`, "Every live scenario whose result changed").
