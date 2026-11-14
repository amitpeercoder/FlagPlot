# 0095: Preparing a landing

Status: a plan by the architect, written on Nov 14, 2026, after the designer asked that planning ahead should raise an operation's chance of success and that changing a plan should cost some of what was prepared. The designer answered its two questions on Nov 14, 2026: preparation also helps the fight ashore (decision 4a), and what a change costs depends on the change (decision 6). Nothing is built. Every figure is a placeholder set by the architect. The hand calculation is written before any code is delegated.

## The gap

Time already buys a side real things: stocks built up by convoy, ships back from repair, rested air groups, and more reports of the enemy. It does not buy preparation. A landing thrown together the night before goes ashore as fast as one loaded and rehearsed for a month, and a plan changed at the last hour costs nothing but the ships that must turn round.

The record's first landing is the case. It was ordered and mounted in a few weeks, its rehearsal went badly, its ships were not loaded for an assault, and what failed on 7 to 9 August was the unloading: the transports left with much of the supply still aboard (`docs/research/oob/land-forces.md`, sections 4 and 8).

## Decisions

1. **A force prepares a landing by order.** `prepare_landing` names a task force and a coast hex, the place it is to land at. It stands until replaced or until the landing is done. It is refused at intake if the force is not the side's own or the hex is not a coast hex on the map.
2. **Preparation is a number from 0 to 100, held by the force.** It starts at 0 unless a scenario says.
3. **It grows by the day in port.** At the start of each day, a force with the order that is stopped at a base of its own side gains `preparation_per_day`, up to 100. At sea it keeps what it has and gains nothing.
4. **Preparation speeds the landing.** When a force that carries men of a land unit unloads, each ship's work in a pulse, by berth or by lighter, is multiplied by `unprepared_landing_share` plus the rest of 1 times the preparation over 100. A force with no preparation, or one that unloads at another hex than the one it prepared for, works at `unprepared_landing_share` of the rate. It holds for every force that lands men, with or without the order, so that not giving the order is no way round it. A force that carries no men unloads as it always has.
4a. **It also helps the fight ashore.** When a unit's first men land in a hex, the unit gains `unprepared_landing_fatigue` times 100 less the force's preparation over 100, up to 100. Fatigue cuts the men who fire (decision 0090, 6) and wears off by the day, so men put ashore in a hurry fight worse in their first days and men landed after a full preparation lose nothing. No new state is needed. *The designer's choice, Nov 14, 2026. The architect had proposed that preparation speed the landing and do nothing else, since nothing in the rows sizes the effect on the fight. The figure is a placeholder with nothing behind it and the record says so.*
5. **It is spent by the landing.** When the force's unloading at the hex is complete, its preparation is 0 and the order is done.
6. **What a change costs depends on the change.** A new `prepare_landing` order for another hex loses all of it: the force starts again from 0. A ship joining the force, or loading men of a land unit that the force did not carry when it began to prepare, keeps `preparation_kept_on_change` of it. *The designer's choice, Nov 14, 2026.*
7. **A side sees its own forces' preparation exactly,** each day's gain and each loss with its reason. The enemy sees nothing of it.

## Placeholder figures

In `ground.toml`, and in the frozen step 1 figures with the same first values:

| Key | Value | What it is |
| --- | --- | --- |
| `preparation_per_day` | `5` | Preparation a force gains for each day in port with the order |
| `unprepared_landing_share` | `"0.5"` | The share of a ship's unloading rate a force with no preparation works at |
| `preparation_kept_on_change` | `"0.5"` | The share of its preparation a force keeps when a ship joins it or a new unit is loaded |
| `unprepared_landing_fatigue` | `40` | Fatigue a unit gains on landing from a force with no preparation |

In the frozen step 1 figures the rule is switched off by its first values, as experience and readiness are there: `unprepared_landing_share` is `"1"` and `unprepared_landing_fatigue` is `0`, so the hand calculations of the earlier stages stand. The live values are those above.

None rests on a source. With them a landing is fully prepared after 20 days in port, and one mounted in 10 days gets its men and supply ashore at three quarters of the rate. A coarse measure for stage 4 can rest on the supply the transports had landed when they left on 9 August, which has rows.

## A small worked case

A force lies at its base with the order from day 1. A ship of the force unloads by lighter at 20 t an hour.

| Days in port | Preparation | Rate at the landing | In a 12-hour pulse |
| --- | --- | --- | --- |
| 0 | 0 | 20 x 0.5 = 10 t an hour | 120 t, or 480 men |
| 10 | 50 | 20 x (0.5 + 0.5 x 50/100) = 15 t an hour | 180 t, or 720 men |
| 20 | 100 | 20 x 1 = 20 t an hour | 240 t, or 960 men |

After 20 days a ship joins the force. It keeps 100 x 0.5 = 50. Four more days in port bring it to 70, and it lands at 20 x (0.5 + 0.5 x 70/100) = 17 t an hour. A unit it lands gains 40 x (100 - 70)/100 = 12 fatigue, and fires on its first day with 88 men in a hundred.

Had the landing place been changed, the force would have gone to 0, and four days would bring it to 20: 12 t an hour, and 32 fatigue for a unit it lands.

## Why

- **It makes lead time a decision.** A commander who waits lands faster and loses fewer days under the enemy's aircraft. One who goes early gets ashore before the enemy is reinforced. The enemy uses the same days.
- **It makes a late change cost something** beyond the ships that turn round.
- **One number, two effects, no new mechanism.** It scales the cargo rule's rate and adds to a unit's fatigue. Both exist.

## What it leaves out

- **Training of troops.** A land unit is a count of men. Nothing in the rows would set a figure for it.
- **Preparation of anything but a landing:** a bombardment, a strike or a convoy.
- **Rehearsal as an act.** Days in port stand for all of it.
- **Loading for an assault as a choice** of what goes aboard and in what order.

## What it costs

- **New state:** a force's preparation and the hex it prepares for. They go into the hash and are named beside it; the hash version is raised and the golden files remade (decision 0057).
- **A new order with its refusals, new events** with their rows in `side_events.rs`, and the agent interface: the order, a force's preparation in the briefing and queries, and the rate it will land at.
- **The rulebook:** chapter 2 (working cargo) and chapter 7 (landing), chapter 1 for the order and the events, with the glossary, in the same pull request as the code.

## Rule card

| | |
| --- | --- |
| Reads | A force's side, position, status, ships, the land units aboard and its preparation; the bases of the side at its hex; the cargo order and the hex; the three figures |
| Writes | A force's preparation and the hex it prepares for; a land unit's fatigue on landing; the events. The cargo rule reads the preparation for its rate |

## Questions for the designer

Answered on Nov 14, 2026.

1. **Decision 4:** preparation speeds the landing and also helps the fight ashore (decision 4a).
2. **Decision 6:** a new landing place loses all of it; a ship joining or a new unit loaded keeps half.
