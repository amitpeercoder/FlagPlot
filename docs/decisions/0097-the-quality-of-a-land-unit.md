# 0097: The quality of a land unit, and training

Status: a plan by the architect, written on Nov 14, 2026, after the designer asked that step 3 go beyond rehearsal for one landing to troop quality that lasts. The designer agreed decisions 3 and 5 on Nov 14, 2026. Nothing is built. Every figure is a placeholder. No real unit has a value yet: that waits on research, as the experience of air groups does (decision 0064).

## The gap

A land unit is a count of men. A battalion that has fought for a month and one that came off the transport yesterday fire alike. Decision 0095 gives a unit rehearsal for one landing, which the landing spends. Nothing a unit has lasts.

## Decisions

1. **A land unit has experience, a whole number from 0 to 4,** 2 being its service's standard. It is the scale ships' crews have (decision 0064), and it means the same: 0 is raw, 4 is the best there is. A scenario gives it; 2 if it does not.
2. **Experience moves the chance a unit's shot hits.** The chance of decision 0090, 7, is multiplied by 1 plus `experience_fire_strength` times the unit's experience less 2. At a strength of 0 it does nothing. One figure serves both sides.
3. **Training raises it, slowly, and only so far.** A unit ordered to train gains a day of training on each day that all its men are ashore at a base of its side, fed in full, not marching and not in a fight. After `training_days_per_level` such days its experience rises by one and the count starts again. Training cannot take a unit above `training_ceiling`. *Agreed by the designer, Nov 14, 2026. The ceiling is the standard, 2. Training makes raw troops into sound ones and no more; what is above that comes from fighting. The other choice is no ceiling.*
4. **A unit trains or rehearses, not both.** A day that counts for rehearsal under decision 0095 does not count for training.
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
