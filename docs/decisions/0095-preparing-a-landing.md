# 0095: Preparing a landing

Status: a plan by the architect, written on Nov 14, 2026, after the designer asked that planning ahead should raise an operation's chance of success and that changing a plan should cost some of what was prepared. The designer answered its two questions on Nov 14, 2026: preparation also helps the fight ashore, and what a change costs depends on the change. It was revised the same day (see "Decisions"). Nothing is built. Every figure is a placeholder set by the architect. The hand calculation is written before any code is delegated.

## The gap

Time already buys a side real things: stocks built up by convoy, ships back from repair, rested air groups, and more reports of the enemy. It does not buy preparation. A landing thrown together the night before goes ashore as fast as one loaded and rehearsed for a month, and a plan changed at the last hour costs nothing but the ships that must turn round.

The record's first landing is the case. It was ordered and mounted in a few weeks, its rehearsal went badly, its ships were not loaded for an assault, and what failed on 7 to 9 August was the unloading: the transports left with much of the supply still aboard (`docs/research/oob/land-forces.md`, sections 4 and 8).

## Decisions

Revised on Nov 14, 2026, after a third outside review and the designer's answer that troops should rehearse as a real activity. What is prepared now has two owners: the force, which is loaded for the landing, and each land unit, which rehearses it. The first version held one number on the force.

### The order

1. **A force prepares a landing by order.** `prepare_landing` names a task force, a coast hex, and the land units that are to land from it: its roster. It stands until replaced or until the landing is done. It is refused at intake if the force or a unit is not the side's own, if the hex is not a coast hex on the map, or if a unit is destroyed. Giving the same order again changes nothing.

### What is earned, and by whom

2. **The force has loading, from 0 to 100.** It is the ships: what is stowed where, and the boats and their crews. It starts at 0 unless a scenario says.
3. **Each land unit has rehearsal, from 0 to 100, for one hex.** It is the men: knowing the plan, the boats and the beach. It starts at 0 unless a scenario says.
4. **Both grow by the day, by doing it.** After the day's orders are taken, a force with the order that is stopped at a base of its own side gains `preparation_per_day` of loading, up to 100. Each unit of its roster that is in the force's hex that day, ashore or aboard its ships, gains the same of rehearsal for the hex of the order. A unit elsewhere gains nothing. Neither grows at sea. Resting, or a plan that exists only on paper, earns nothing.

### What it buys

5. **Loading speeds the landing.** From the first man a force lands in a hex until its unloading there is complete, each ship's work in a pulse is multiplied by `unprepared_landing_share` plus the rest of 1 times its loading over 100. That holds for everything it unloads in that time, supply as well as men, and after its last man is ashore. Loading counts only at the hex of the order: at any other hex the force works as one with no loading. It holds for every force that lands men, with or without the order. A force that carries no men unloads as it always has.
6. **Rehearsal spares the men on landing.** When a unit's first men land in a hex, the unit gains `unprepared_landing_fatigue` times 100 less its rehearsal over 100, up to 100. Its rehearsal counts only if it is for that hex: at any other hex it counts as none. Fatigue cuts the men who fire and wears off by the day (decision 0090), so men put ashore in a hurry fight worse in their first days.
7. **Both are spent by the landing.** A unit's rehearsal is 0 once its first men have landed. A force's loading is 0, and its order done, when its unloading at the hex is complete.

### What a change costs

8. **A new landing place loses everything.** A new order for another hex sets the force's loading to 0 and the rehearsal of every unit of its roster to 0, since each is for a place.
9. **A change of ships costs loading and not rehearsal.** A ship joining the force keeps `preparation_kept_on_change` of its loading. Its units keep their rehearsal.
10. **A change of units costs loading, and the new unit starts where it stands.** A unit added to the roster keeps `preparation_kept_on_change` of the force's loading. The unit brings whatever rehearsal it has for that hex, which is none unless it rehearsed it with another force.
11. **A unit that changes force keeps its rehearsal** for the same hex.
12. **A force that splits gives each part its loading. Forces that join take the lower of the two.** Rehearsal is the unit's and is never copied.
13. **A change of date costs nothing here.** It costs what the days themselves cost.

### What a side knows

14. **A side sees its own forces' loading and its own units' rehearsal exactly,** each day's gain and each loss with its reason. The enemy sees nothing of them.

## Placeholder figures

In `ground.toml`, and in the frozen step 1 figures with the same first values:

| Key | Value | What it is |
| --- | --- | --- |
| `preparation_per_day` | `5` | Preparation a force gains for each day in port with the order |
| `unprepared_landing_share` | `"0.5"` | The share of a ship's unloading rate a force with no preparation works at |
| `preparation_kept_on_change` | `"0.5"` | The share of its preparation a force keeps when a ship joins it or a new unit is loaded |
| `unprepared_landing_fatigue` | `40` | Fatigue a unit gains on landing from a force with no preparation |

In the frozen step 1 figures the rule is switched off by its first values, as experience and readiness are there: `unprepared_landing_share` is `"1"` and `unprepared_landing_fatigue` is `0`, so the hand calculations of the earlier stages stand. The live values are those above.

None rests on a source. With them a landing is fully prepared after 20 days in port, and one mounted in 10 days gets its men and supply ashore at three quarters of the rate and its men ashore with a fatigue of 20. A coarse measure for stage 4 can rest on the supply the transports had landed when they left on 9 August, which has rows.

## A small worked case

A force lies at its base with the order from day 1, with its one unit in its hex. A ship of the force unloads by lighter at 20 t an hour.

| Days in port | Loading and rehearsal | Rate at the landing | Fatigue on landing |
| --- | --- | --- | --- |
| 0 | 0 | 20 x 0.5 = 10 t an hour | 40 |
| 10 | 50 | 20 x (0.5 + 0.5 x 50/100) = 15 t an hour | 40 x 50/100 = 20 |
| 20 | 100 | 20 x 1 = 20 t an hour | 0 |

After 20 days a ship joins the force. Its loading is 100 x 0.5 = 50; the unit's rehearsal is still 100. Four more days bring the loading to 70. It lands at 20 x (0.5 + 0.5 x 70/100) = 17 t an hour, and the unit lands with no fatigue.

Had the landing place been changed after 20 days, loading and rehearsal would both be 0, and four days would bring each to 20: 12 t an hour, and a fatigue of 32.

A force with loading 100 for one hex that lands at another works at 10 t an hour, and its unit, rehearsed for the first hex, lands with a fatigue of 40.

## Why

- **It makes lead time a decision.** A commander who waits lands faster and loses fewer days under the enemy's aircraft. One who goes early gets ashore before the enemy is reinforced. The enemy uses the same days.
- **It makes a late change cost something** beyond the ships that turn round.
- **Two numbers, each with its owner, and no new mechanism.** Loading scales the cargo rule's rate. Rehearsal takes from a unit's fatigue on landing. Each follows what earned it: the ships or the men.
- **Having written a plan earns nothing.** Only days spent by real units in a real place do.

## What it leaves out

- **Lasting troop quality.** Rehearsal is for one landing and is spent by it. What a unit keeps is decision 0097.
- **Preparation of anything but a landing:** a bombardment, a strike or a convoy.
- **Rehearsal as an act.** Days in port stand for all of it.
- **Loading for an assault as a choice** of what goes aboard and in what order.

## What it costs

- **New state:** a force's loading, the hex and the roster of its order; a unit's rehearsal and the hex it is for. They go into the hash and are named beside it; the hash version is raised and the golden files remade (decision 0057).
- **A new order with its refusals, new events** with their rows in `side_events.rs`, and the agent interface: the order, a force's preparation in the briefing and queries, and the rate it will land at.
- **The rulebook:** chapter 2 (working cargo) and chapter 7 (landing), chapter 1 for the order and the events, with the glossary, in the same pull request as the code.

## Rule card

| | |
| --- | --- |
| Reads | A force's side, position, status, ships, order, roster and loading; a land unit's place and rehearsal; the bases of the side at the force's hex; the four figures |
| Writes | A force's loading, hex and roster; a land unit's rehearsal and its hex, and its fatigue on landing; the events. The cargo rule reads the loading for its rate |

## Questions for the designer

Answered on Nov 14, 2026.

1. **Decision 4:** preparation speeds the landing and also helps the fight ashore (decision 4a).
2. **Decision 6:** a new landing place loses all of it; a ship joining or a new unit loaded keeps half.
