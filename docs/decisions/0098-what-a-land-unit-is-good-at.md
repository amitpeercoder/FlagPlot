# 0098: What a land unit is good at

Status: a plan by the architect, written on Nov 14, 2026, after the designer asked that units differ in what they are suited to, so that a staff or a player picks a unit for a mission: Marines for an assault from the sea, mountain troops for the mountains. The designer agreed decisions 2 and 6 on Nov 14, 2026. Built on Nov 14, 2026 (see "As built"). Every figure is a placeholder. No real unit has a value yet.

## The gap

A land unit is a count of men with a name. With decisions 0095 and 0097 it will also carry rehearsal for one landing and experience. Both come from what a unit has done. Nothing says what a unit is: two units with the same history are the same unit, whatever they were raised, trained and equipped for.

The plan for ground combat cut kinds of unit, weapons and vehicles, and that stands. This record does not add a table of unit types. It adds a few numbers to a unit.

## Decisions

1. **A land unit has skills.** A skill is a whole number from 0 to 4, 2 being the standard, on the scale experience has. A scenario gives a unit its skills; any it does not give is 2. A unit with every skill at 2 is the unit as it is today.
2. **There are four skills, and each acts on one thing the rules already do.** *Agreed by the designer, Nov 14, 2026. The list is closed and short on purpose. A fifth is added by a record of its own when a mission needs it.*

   | Skill | What it changes | Who has it |
   | --- | --- | --- |
   | Assault from the sea | The rehearsal a unit gains for each day (decision 0095, 4) | Troops raised and equipped to land from boats |
   | Movement, by terrain | The march a unit makes in a day, when the hex it leaves or the hex it enters has that terrain (decision 0094, 4) | Mountain troops in mountains; troops trained for the jungle in jungle |
   | Holding ground | The chance its shots hit when its side defends (decision 0090, 7) | Troops with guns and the means to dig in and stay |
   | Weight | The shots its men fire, and the supply each man costs a day (decisions 0090, 7 and 0066, 16) | A heavy formation with its artillery and its train, against light infantry |

3. **One formula.** A skill multiplies what it changes by 1 plus its strength times the skill less 2. The strengths are figures, one for each skill, the same for both sides. At a strength of 0 a skill does nothing.
4. **Movement is by terrain.** A unit has one movement skill for each kind of terrain the map has, each 2 unless given. The map has four: clear, jungle, mountain and atoll. A march counts the lower of the unit's skills for the terrain of the hex it leaves and of the hex it enters. So mountain troops are fast from one mountain hex to the next and ordinary on the plain, and nobody is fast through a hex he is not trained for. A day's march by a unit whose factor is above 1 counts as more than a day toward `march_days_per_hex`; below 1, as less.
5. **Weight cuts both ways.** A heavy unit fires more for its men and eats more for its men, by the same factor. It is the better unit where supply can reach it and the worse one where it cannot.
6. **Skills do not change in play.** They are what the unit is. Experience is what it has become, and is the only thing training and fighting raise (decision 0097). *Agreed by the designer, Nov 14, 2026. The other choice is that a unit could be retrained, over months, to raise a skill. It is left out to keep "what a unit is" apart from "what it has done", and because nothing in the rows would size it.*
7. **A unit made by division takes the skills of the unit it left.**
8. **A side sees its own units' skills exactly. The enemy sees none of them.** An estimate of the enemy ashore is of men.

## What it gives the staff

The staff of step 3 picks units by them. Its doctrine for a task prefers, among the units it may use, the one whose skills fit: the highest assault skill for the first wave, the highest holding skill and weight for the garrison that relieves it, the movement skill for the terrain of an overland approach. Its reason for the choice names the skill. Where the best-fitted unit is committed elsewhere, the report says what the second choice costs.

## Placeholder figures

In `ground.toml`. In the frozen step 1 figures every strength is `0`, so the hand calculations of the earlier stages stand.

| Key | Value | What it is |
| --- | --- | --- |
| `assault_skill_strength` | `"0.25"` | How much a step of the assault skill moves the rehearsal gained in a day |
| `movement_skill_strength` | `"0.25"` | How much a step of a movement skill moves a day's march |
| `holding_skill_strength` | `"0.15"` | How much a step of the holding skill moves a defender's chance to hit |
| `weight_strength` | `"0.25"` | How much a step of weight moves a unit's shots and its supply for each man |

None rests on a source. At 0.25 a unit of assault skill 4 rehearses a landing in 14 days where a standard unit takes 20 and a unit of skill 0 takes 40.

## Real units

No real unit gets a value from this record. A unit's skills are facts about it, with rows and a written rule, as a ship's figures are. Until that research is done every real land unit is 2 in every skill. The hand calculation is in `tests/scenario_29_skills.rs`. The tuning rule holds: a unit's value is never changed to improve a report; only the strengths are tuned.

The real map marks every land hex as jungle (decision 0052, item 14), so on it the only movement skill that can act is the one for jungle. The skill for mountains is tested on a placeholder map that has them.

## Why

- **Numbers, not types.** A staff can compare two units on a skill without a table of what each kind may do, and a new formation needs no new rule.
- **Each skill acts through a rule that exists.** Nothing here adds a mechanism.
- **It makes the choice of unit a decision.** The best assault troops are few, and the plan that uses them for the landing does not have them to hold the ground.

## What it leaves out

- **Weapons, vehicles, artillery and engineers as things.** Weight stands for all of it.
- **A skill at fighting in a terrain,** as against moving through it. Terrain does not enter the fight.
- **Skills for air groups and ships.**
- **Retraining.**

## What it costs

- **New state:** a unit's skills. They go into the hash and are named beside it; the hash version is raised and the golden files remade (decision 0057).
- **A scenario key for each skill,** breakdowns that name the skill wherever it acts, and the agent interface: a unit's skills in the briefing and queries.
- **Research,** before any real unit differs from another.
- **The rulebook:** chapter 7 and the glossary, in the same pull request as the code. It is built after decisions 0094, 0095 and 0097, since it acts through them.

## Rule card

| | |
| --- | --- |
| Reads | A land unit's skills; the terrain of the hexes of a march; the four strengths |
| Writes | Nothing of its own. The rules of rehearsal, marching, the fight and the men's supply read a unit's skills for their factors |

## Questions for the designer

Answered on Nov 14, 2026.

1. **Decision 2:** four skills to start with: assault from the sea, movement by terrain, holding ground, and weight. Agreed.
2. **Decision 6:** a unit's skills are fixed; only experience grows in play. Agreed.

## As built

Built on Nov 14, 2026 (scenario 29, state version 17; the hand calculation is in `tests/scenario_29_skills.rs`, on the placeholder map `data/map/placeholder_skills.toml`). Choices the record did not make:

- **A hex with no terrain counts as jungle.** The generated map marks every land hex jungle and gives coast hexes no terrain (decision 0052, 14), so on the real map only the jungle skill can act, as the record says.
- **Each strength is at least 0 and below 0.5,** so that a skill of 0 never makes a factor of 0: a march that never ends, a unit that eats nothing. The loader and `World::new` refuse a strength outside that, and a TOML float.
- **Breakdowns name a skill and its factor only where the factor is not 1** (the strength above 0 and the skill not 2), so a standard unit's breakdowns and every existing scenario's logs are as they were. The holding skill is named only while the unit's side defends, with `side_chance` beside it.
- **A march keeps `days` and gains `counted`.** `days` is the days marched, which fatigue counts; `counted` is the days counted toward `march_days_per_hex`, the sum of the day's movement factors, and the unit arrives when it reaches the figure. The `MarchDay` event carries `counted`, and its text adds "counted as" only where it differs from the days. A march order to the hex already marched to keeps both.
- **Weight acts on a march's carried supply too:** a marching unit carries `march_supply_days` of its own daily charge, weight included.
- **Feeding where weight acts.** The hex's charge is the sum, for each unit, of its men times the daily charge times its weight factor. The men fed are the tons paid times the men over that charge, cut: exactly the old count for standard units. The fed share is common to the side's units in the hex, as before, and the unfed loss is by men.
- **The chance** is the side's chance times the experience factor times the holding factor, cut to 0 to 1. Shots are not cut to whole numbers, as before.
- **The scenario key** is an inline table on a land unit: `skills = { assault, holding, weight, clear, jungle, mountain, atoll }`, each optional and 2 when not given, whole numbers from 0 to 4. An invariant refuses a skill above 4 in play.
- **The agent interface.** The side's own unit line carries `skills` (assault, holding, weight and movement by terrain), the march line `days_counted` and the march-day line `counted`. The briefing's daily supply by hex counts each unit's weight. `SideView::movement_factor` and `SideView::daily_charge_t` answer for the side's own units. The replay export carries skills, left out when all are 2, and `counted`. Schemas version 3 is unchanged: fields only.
- **Hash version 17:** a march's `counted` after its `days`, and a land unit's seven skills after its `fight_days`.

The figures are in `data/reference/ground.toml` (live) and `data/reference/step1/ground.toml` (frozen, every strength 0). The rulebook is chapter 7 (What a land unit is good at), with chapters 1 and 2 and the glossary.
