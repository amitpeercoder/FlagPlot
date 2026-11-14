# 0095: Preparing a landing

Status: a plan by the architect, written on Nov 14, 2026, after the designer asked that planning ahead should raise an operation's chance of success and that changing a plan should cost some of what was prepared. Decisions 4 and 6 need review. Nothing is built. Every figure is a placeholder set by the architect. The hand calculation is written before any code is delegated.

## The gap

Time already buys a side real things: stocks built up by convoy, ships back from repair, rested air groups, and more reports of the enemy. It does not buy preparation. A landing thrown together the night before goes ashore as fast as one loaded and rehearsed for a month, and a plan changed at the last hour costs nothing but the ships that must turn round.

The record's first landing is the case. It was ordered and mounted in a few weeks, its rehearsal went badly, its ships were not loaded for an assault, and what failed on 7 to 9 August was the unloading: the transports left with much of the supply still aboard (`docs/research/oob/land-forces.md`, sections 4 and 8).

## Decisions

1. **A force prepares a landing by order.** `prepare_landing` names a task force and a coast hex, the place it is to land at. It stands until replaced or until the landing is done. It is refused at intake if the force is not the side's own or the hex is not a coast hex on the map.
2. **Preparation is a number from 0 to 100, held by the force.** It starts at 0 unless a scenario says.
3. **It grows by the day in port.** At the start of each day, a force with the order that is stopped at a base of its own side gains `preparation_per_day`, up to 100. At sea it keeps what it has and gains nothing.
4. **Preparation speeds the landing and nothing else.** When the force unloads at the hex it prepared for, each ship's work in a pulse, by berth or by lighter, is multiplied by `unprepared_landing_share` plus the rest of 1 times the preparation over 100. A force with no preparation, or one that unloads at another hex, works at `unprepared_landing_share` of the rate. A force with no `prepare_landing` order ever given works at the full rate, as now, so no existing scenario changes. *Needs review: it acts on how fast men and supply get ashore, because that is what the record shows going wrong and what our rows can check. The other choice is to let it also raise the men's fire in the first days ashore, which nothing in the rows would size.*
5. **It is spent by the landing.** When the force's unloading at the hex is complete, its preparation is 0 and the order is done.
6. **A change costs part of it.** A new `prepare_landing` order for another hex keeps `preparation_kept_on_change` of what the force had. So does a ship joining the force. Loading men of a land unit that the force did not carry when it began to prepare does the same. *Needs review: this is what makes a late change of plan cost something. The other choice is that any change sets it to 0.*
7. **A side sees its own forces' preparation exactly,** each day's gain and each loss with its reason. The enemy sees nothing of it.

## Placeholder figures

In `ground.toml`, and in the frozen step 1 figures with the same first values:

| Key | Value | What it is |
| --- | --- | --- |
| `preparation_per_day` | `5` | Preparation a force gains for each day in port with the order |
| `unprepared_landing_share` | `"0.5"` | The share of a ship's unloading rate a force with no preparation works at |
| `preparation_kept_on_change` | `"0.5"` | The share of its preparation a force keeps when its landing is changed |

None rests on a source. With them a landing is fully prepared after 20 days in port, and one mounted in 10 days gets its men and supply ashore at three quarters of the rate. A coarse measure for stage 4 can rest on the supply the transports had landed when they left on 9 August, which has rows.

## A small worked case

A force lies at its base with the order from day 1. A ship of the force unloads by lighter at 20 t an hour.

| Days in port | Preparation | Rate at the landing | In a 12-hour pulse |
| --- | --- | --- | --- |
| 0 | 0 | 20 x 0.5 = 10 t an hour | 120 t, or 480 men |
| 10 | 50 | 20 x (0.5 + 0.5 x 50/100) = 15 t an hour | 180 t, or 720 men |
| 20 | 100 | 20 x 1 = 20 t an hour | 240 t, or 960 men |

After 20 days the landing place is changed. The force keeps 100 x 0.5 = 50. Four more days in port bring it to 70, and it lands at 20 x (0.5 + 0.5 x 70/100) = 17 t an hour.

## Why

- **It makes lead time a decision.** A commander who waits lands faster and loses fewer days under the enemy's aircraft. One who goes early gets ashore before the enemy is reinforced. The enemy uses the same days.
- **It makes a late change cost something** beyond the ships that turn round.
- **One number, one effect.** It reuses the cargo rule's rate and adds no new kind of thing.

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
| Writes | A force's preparation and the hex it prepares for; the events. The cargo rule reads the preparation for its rate |

## Questions for the designer

1. **Decision 4:** preparation speeds the landing and does nothing else, for now. Agreed?
2. **Decision 6:** changing the landing place, adding a ship or loading a new unit keeps half the preparation. Agreed, or should a change lose all of it?
