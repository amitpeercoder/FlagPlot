# 0097: The quality of a land unit, and training

Status: a plan by the architect, written on Nov 14, 2026, after the designer asked that step 3 go beyond rehearsal for one landing to troop quality that lasts. The designer agreed decisions 3 and 5 on Nov 14, 2026. Built on Nov 14, 2026, scenario 28 ("As built", at the end); the hand calculation is in `tests/scenario_28_experience_and_training.rs`. Every figure is a placeholder. No real unit has a value yet: that waits on research, as the experience of air groups does (decision 0064).

Summary: The quality of a land unit, and training: a unit has experience from 0 to 4 on the scale of ships' crews; it moves the chance its shots hit; training raises it slowly to a ceiling and fighting raises it further; real units have no values until researched. Placeholder figures

Standing: Confirmed by the designer, Nov 14, 2026. Built Nov 14, 2026 (scenario 28, with the activities ready and train of decision 0099)

## The gap

A land unit is a count of men. A battalion that has fought for a month and one that came off the transport yesterday fire alike. Decision 0095 gives a unit rehearsal for one landing, which the landing spends. Nothing a unit has lasts.

## Decisions

1. **A land unit has experience, a whole number from 0 to 4,** 2 being its service's standard. It is the scale ships' crews have (decision 0064), and it means the same: 0 is raw, 4 is the best there is. A scenario gives it; 2 if it does not.
2. **Experience moves the chance a unit's shot hits.** The chance of decision 0090, 7, is multiplied by 1 plus `experience_fire_strength` times the unit's experience less 2. At a strength of 0 it does nothing. One figure serves both sides.
3. **Training raises it, slowly, and only so far.** A unit ordered to train gains a day of training on each day that all its men are ashore at a base of its side, fed in full, not marching and not in a fight. The day is counted in the ground combat phase, with the day's other gains (decision 0099, 3b). After `training_days_per_level` such days its experience rises by one and the count starts again. A scenario may start a unit with days of training or of fighting already counted. Training cannot take a unit above `training_ceiling`. *Agreed by the designer, Nov 14, 2026. The ceiling is the standard, 2. Training makes raw troops into sound ones and no more; what is above that comes from fighting. The other choice is no ceiling.*
4. **A unit trains or rehearses or works, one of them in a day.** Which it is follows the rule of one activity a day (decision 0099, 3b). The order to train is the `set_activity` order of decision 0099, 4.
5. **Fighting raises it too.** A unit that fired in a fight on `fight_days_per_level` days gains one, up to 4. *Agreed by the designer, Nov 14, 2026. Without this, experience above the standard could only be given by a scenario. With it, a unit that survives a campaign is worth more than its replacement, which is a reason to preserve it.*
6. **Men who join a unit do not dilute it,** because nothing adds men to a unit in play. A unit made by division takes the experience of the unit it left.
7. **A side sees its own units' experience and days of training exactly. The enemy sees neither.** An estimate of the enemy ashore is of men and says nothing of their quality.

## Placeholder figures

In `ground.toml`. In the frozen step 1 figures the strength is `0`, so the hand calculations of the earlier stages stand.

| Key | Value | What it is |
| --- | --- | --- |
| `experience_fire_strength` | `"0.25"` | How much a step of experience moves the chance a shot hits |
| `training_days_per_level` | `30` | Days of training for one step |
| `training_ceiling` | `2` | The experience training cannot pass |
| `fight_days_per_level` | `10` | Days of firing in a fight for one step |

None rests on a source. At 0.25 a raw unit's shots hit half as often as a standard unit's, and a unit of experience 4 half as often again.

## Real units

No real unit gets a value from this record. A unit's experience is a fact about it on a date, with rows and a written rule, as a ship's is (decision 0064). Until that research is done every real land unit is 2, and the full slice plays with experience doing nothing between them. The tuning rule holds here as for ships: a unit's value is never changed to improve a report; only the strength is tuned.

## Why

- **The scale and the formula exist.** A player who has learned what experience 3 means for a cruiser knows what it means for a regiment.
- **Time spent training is time not spent rehearsing, fighting or holding ground.** That is the choice.
- **Quality lasts.** A commander who spends a seasoned unit loses something he cannot order back.

## What it leaves out

- **Experience lost with losses.** A unit cut to a tenth of its men keeps its number.
- **Kinds of training,** and training for one kind of fight.
- **The quality of the men who replace losses.**
- **Experience for air groups,** which is decision 0064's second stage and still waits on research.

## What it costs

- **New state:** a unit's experience, its training order and its days of training and of fighting. They go into the hash and are named beside it; the hash version is raised and the golden files remade (decision 0057).
- **A new order, new events** with their rows in `side_events.rs`, the scenario's key, and the agent interface.
- **Research,** before any real unit differs from another.
- **The rulebook:** chapter 7, chapter 1 for the order and the events, and the glossary's entry for experience, in the same pull request as the code.

## Rule card

| | |
| --- | --- |
| Reads | A land unit's men, place, experience, orders and days counted; whether it was fed, marched, rehearsed or fought that day; the bases of its side in its hex; the four figures |
| Writes | A land unit's experience and days counted; the events. The fight reads the experience for its chance |

## Questions for the designer

Answered on Nov 14, 2026.

1. **Decision 3:** training raises a unit to the standard and no further. Agreed.
2. **Decision 5:** fighting also raises experience, up to the top. Agreed.

## As built

Built on Nov 14, 2026 (scenario 28, state version 16; the hand calculation is in `tests/scenario_28_experience_and_training.rs`), with the part of decision 0099 that its decisions 3 and 4 name: the one activity a day (3b to 3d) and the order `set_activity` with the activities ready and train. The activity work and everything else of 0099 is not built. Choices the record did not make:

- **State.** A land unit's `activity` (ready or train; work to come), `activity_order`, `experience` (0 to 4, 2 by default), `training_days` and `fight_days`. The hash holds them.
- **The order `set_activity`** (a land unit, `ready` or `train`) is refused for the first reason that applies: not your land unit; land unit destroyed. A unit that has men aboard only may be set to train, and gains nothing until all its men are ashore. The order is a standing one and is applied when taken in.
- **The day's activity** is settled once every hex is resolved, at the end of the ground combat phase: first each force's loading (decision 0095), then each unit in unit order (a unit that marched or whose hex was contested: nothing; train: a day of training if all its men are ashore at a base of its side and its fed share is 1; ready, on the roster of a force that qualified: rehearsal; otherwise nothing), then the days of fighting.
- **"Fed in full"** is a fed share of 1, as the night's charge set it. A unit that landed after the charge keeps the share it had.
- **A unit fired** if its shots in a fight were above 0, on attack or on defend. Each such day adds one to its count, up to the step at `fight_days_per_level`; at experience 4 nothing is counted. At or above the training ceiling a day of training counts nothing and is not recorded.
- **The events** are `TrainingDay` and `FightDayCounted`, which record the count after the day and the experience before any rise, and `ExperienceRaised` (by training or by fighting). Each is seen whole by its own side and not at all by the enemy.
- **The chance** is the side's chance times 1 plus the strength times the experience less 2, kept within 0 and 1. The loader and `World::new` keep the strength from 0 to 0.5, so the factor is never below 0. The `GroundFire` breakdown gains `side_chance`, `experience` and `experience_factor` only where the strength is above 0. A side's own cut of `GroundFire` keeps `experience` and `experience_factor` and still has no chance or hits.
- **Scenario keys.** On `[[land_unit]]`: `experience` (0 to 4) and `training_days` and `fight_days` (0 or more, below their figures). In scenario files the order kinds `prepare_landing` (`task_force`, `hex`, `roster`) and `set_activity` (`land_unit`, `activity`).
- **A unit made by division** takes its parent's experience and nothing else: no rehearsal, no days counted, the activity ready and no roster.
- **Days of fighting are counted with the frozen figures too,** so the frozen runs of scenarios with a fight ashore gain `FightDayCounted` events, and a unit may reach 3. At the frozen strength of 0 nothing that happens changes. The baselines of scenarios 23, 24 and 25 changed only by those events, since no unit reaches 10 days.
- **Every real unit** in the repository's scenarios starts at experience 2 with nothing counted. Real units' values wait on research.
- **The agent interface** writes `set_activity {unit: "land_unit:N", activity}` and shows a land unit's `activity`, `experience`, `training {days, of_days, up_to}` and `fighting {days, of_days}`. The schema version stays 3. The MCP instructions gained a paragraph, "Preparing a landing and training".
- **The figures** are the four of the record, in `data/reference/ground.toml`; the frozen copy has the strength 0 and the same other three.
