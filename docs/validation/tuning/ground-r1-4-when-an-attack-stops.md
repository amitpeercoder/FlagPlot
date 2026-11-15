# Ground round 1, step 4: when an attack stops. Tried and left, nothing moved

**Ruled by the architect, Nov 15, 2026: no move.** A second day at the ridge that hangs on five points of fatigue, and that takes the ridge's Japanese losses out of their record, is not a fit.

The full slice, 39 days, Nov 15, 2026. The trials are on step 1's figures, with steps 2 and 3 left as they were. Each is 1,000 runs, seeds 1 to 1,000, with `sim-cli validate`, labelled `r1-s4-<value>`.

## What is wrong

The record has the Tenaru as one night. The first rush was at 0310 on 21 August and the action was over by about 1700 (land_unit_facts 1024, 857; engagement_outcomes 1919). It has the ridge as two nights: a light probe on the night of 12 September and the main attack on the night of 13 to 14 September, ended by dawn (land_unit_facts 1017, 1001, 1012, 1013, 1018, 1004).

In the game a day is one night pulse and two daylight pulses, and the fight ashore is worked once a day, so two nights are two days of attack. Before the step every attack stops on the day it begins, in every run. The Ichiki echelon goes back to defend at fatigue 70 and the Kawaguchi parts at 65. Each falls back the same day.

## The figures

| File | Figure | Now | What it does |
| --- | --- | --- | --- |
| `data/reference/ground.toml` | `attack_fatigue_limit` | 60 | above it a unit on attack goes back to defend |
| `data/reference/ground.toml` | `failed_attack_fatigue` | 30 | fatigue a unit gains on a day its side's attack failed |

The fatigue after a day of attack is the fatigue at the fire, plus a day's march if the unit marched in (5 after step 1), plus `fatigue_per_attack_day` (20), plus `failed_attack_fatigue` (30) when the attack failed. Every attack in the slice fails on its first day: the Marines' weight of fire is six to seven times the attackers'.

So with step 1's figures:

- **The Ichiki echelon** ends its day at 15 + 5 + 20 + 30 = 70.
- **The Kawaguchi parts and the Oka group** end at 10 + 5 + 20 + 30 = 65.

## What the figures can and cannot do

**What they can do.** A limit of 65, or a failed day's fatigue of 25, puts the line between the two numbers above. The Ichiki echelon stops on day 16, and the Kawaguchi parts attack again on day 37 and 38 and stop on day 38. On the second day they start at 65, with 35 percent of their men firing.

**What they cannot do.**

- **The split rests on five points of fatigue.** That margin comes from step 1's landing figure. The Ichiki echelon marched the day after it landed and still carried 10 of landing fatigue; the Kawaguchi men had rested it off. Nothing in that is why the record's fights lasted one night and two.
- **The record's reasons are not in the rules.** The Tenaru ended because the echelon was surrounded and destroyed where it stood. The ridge's first night was a probe by the only battalion at its jump-off point (land_unit_facts 859), and the main attack came the next night. Neither is fatigue.

## What was run

| Values | Tenaru: days of attack | Ridge: days of attack | Ridge, Japanese (600 to 1,138), reported | Ridge, American (134 to 253), reported | Tenaru, both judged |
| --- | --- | --- | --- | --- | --- |
| Limit 60, failed 30 (now) | 1 | 1 | 1,220 (1,060 to 1,380) | 148 (82 to 218) | 46 and 340: both miss |
| Limit 65 | 1 | 2 in every run (day 38: 700 Japanese, 580 to 840; 31 Americans, 0 to 66) | 1,920 (1,740 to 2,100): out | 179 (108 to 266) | unchanged |
| Failed 25 | 1 | 2 in every run (day 38: 700 Japanese; 31 Americans, 0 to 82) | 1,920 (1,740 to 2,100): out | 183 (108 to 267) | unchanged |

Both moves give the record's count of nights, and the same fight on the second day.

## The ruling

No move. `attack_fatigue_limit` stays 60 and `failed_attack_fatigue` 30, placeholders resting on no source.

- **The second day carries the wrong fight.** The record's second night at the ridge was the main attack. In the game it would be a spent force at a third of its fire, losing 700 more men. That takes the ridge's Japanese losses from inside their record's 90 percent to well out of it.
- **The fit is not robust.** It holds only while the landing fatigue keeps the Ichiki echelon five points above the Kawaguchi men.
- **What it would take.** What ends an attack in the record is an attacker committing part of his force first (the ridge) and a defender destroying a spent attacker (the Tenaru). Both are named in `docs/validation/full-slice-39-days-ground-r1.md`, "What the round could not reach".
