# 0099: Units that work, and what they build

Status: a plan by the architect, written on Nov 14, 2026, after the designer said that not every man ashore does the same job: construction battalions built and kept up the bases. The designer agreed decisions 1 and 6 on Nov 14, 2026, and added that fighting units may be put to work, less well, and that defences are something to build (decisions 3a and 4). Decision 4's one job at a time still needs review. Nothing is built. Every figure is a placeholder. No real unit has a value yet.

## The gap

Every man ashore is a rifleman. A base's airfield, its rate of unloading and its aircraft repairs are given by the scenario and never change. So nobody builds anything, there is no reason to land engineers, and a beach taken yesterday flies aircraft as well as a base a year old.

Decision 0093, 6, left construction workers out of the scenario of the August landings because, counted as soldiers, they would have fought for the airfield they in fact left. That was the right answer to the wrong choice. They belong in the game as what they were.

The plan for ground combat deferred "building and repairing airfields" until base orders existed (`docs/design/ground-combat.md`, "What is left out"). This is that rule, in its smallest form.

## Decisions

### Units that work

1. **A land unit fights or works.** A scenario marks a unit as a working unit. Its men do not fire in a fight, and they are not counted when a side is tested for being beaten: labourers do not hold a base. In everything else they are men: they are carried, landed, fed, hit in proportion with the rest, and lost. A unit not so marked is as it is today. *Agreed by the designer, Nov 14, 2026: a construction battalion is a unit.*
2. **A working unit has a works skill, from 0 to 4,** 2 being the standard for a unit raised to build. It is the fifth skill beside those of decision 0098, and like them it is fixed.
3. **It makes work each day.** In the ground combat phase, after the fights, each working unit with men ashore in a hex that holds a base of its side makes work: its men ashore, times its fed share, times `work_per_man_day`, times its works skill over 2. It makes none on a day its hex is contested, or a day it marched.

3a. **A fighting unit can be put to work, and does it less well.** A `set_activity` order sets a land unit to stand ready, to train (decision 0097) or to work. A fighting unit set to work makes work as a working unit does, times `fighting_unit_work_share`, with a works skill of 2 unless the scenario gives it one. A day it works is not a day of training or of rehearsal. If its hex is contested it fights as any unit does and makes no work that day. *Added by the designer, Nov 14, 2026: other units may spare men to build defences, and those men are not training or fighting.*

### What work does

4. **A base has one job in hand, set by order.** `set_works` names a base and a job. It stands until replaced. All the work made in the hex that day goes to it. The jobs are four. *Needs review: one job at a time makes it a choice. The other way is that work is split by a rule, with nothing to decide.*

   | Job | What the day's work does |
   | --- | --- |
   | Airfield | Adds to the work done on the base's airfield, until it is built (decision 5) |
   | Port | The next day, ships at the base work cargo at their rate times 1 plus the work over `work_to_double_unloading`, and no more than twice their rate |
   | Aircraft | The next night, the base repairs one more aircraft for each `work_per_aircraft_repair` of work, cut to a whole aircraft |
   | Defences | Adds to the base's defences, a number from 0 to 100: one for each `work_per_defence_point` of work. While a side defends in a hex, its units' chance to hit is multiplied by 1 plus `defences_strength` times the defences of its lowest-numbered base there over 100 |

   A base with no order builds its airfield if it has none, and else works its port.
4a. **Defences belong to the base and stay with it.** They do not wear away, and a side that takes the base takes them. A scenario may give a base defences to start with. This is the dug-in defence the plan for ground combat left out until something asked for it.
5. **An airfield is built, not given.** A base has an airfield or it does not. Every base a scenario states has one unless the scenario says otherwise, so no existing scenario changes. A base formed by a landing or a march has none (decisions 0066 and 0094). It gets one when the work done on it reaches `airfield_work`. Work done is kept by the base: it is not lost if the builders leave, and a side that takes the base takes the work with it.
6. **Without an airfield no aircraft uses a base.** An air group cannot be rebased to it (decision 0091), aircraft that cannot get home do not land there, and a scenario may not put a group on it. *Agreed by the designer, Nov 14, 2026. It means a side that lands on a bare shore has no air cover of its own until its engineers have had their days.*

### What a side knows

7. **A side sees its own works whole:** each unit's work, each base's job, the work done on an airfield and the day it is built. The enemy sees none of it. An enemy base on a side's map is shown with no word on whether it has an airfield; that is learned as anything about the enemy ashore is, which today means it is not.

## Placeholder figures

In `ground.toml`, and in the frozen step 1 figures with the same first values (no frozen scenario has a working unit, so they do nothing there):

| Key | Value | What it is |
| --- | --- | --- |
| `work_per_man_day` | `1` | Work a man of a standard working unit makes in a day |
| `airfield_work` | `20000` | Work to build an airfield |
| `work_to_double_unloading` | `1000` | Work in a day that doubles the next day's unloading at a base |
| `work_per_aircraft_repair` | `200` | Work in a day for one more aircraft repaired the next night |
| `work_per_defence_point` | `200` | Work for one point of a base's defences |
| `defences_strength` | `"0.5"` | How much full defences raise a defender's chance to hit |
| `fighting_unit_work_share` | `"0.5"` | The share of a working unit's work that a fighting unit set to work makes |

None rests on a source. With them a standard battalion of 1,000 builds an airfield in 20 days, doubles a beach's unloading, adds 5 aircraft to a base's nightly repairs, or digs full defences in 20 days. A regiment of 3,000 riflemen set to dig does the last in about 13 days and trains for none of them. The record has what stage 4 needs to check the first: the field at Lunga was begun in July, was nearly done on 7 August, and flew its first American aircraft on 20 August.

## What it changes in the August scenario

Decision 0093, 6, stands as to who fights. The construction units come back as working units that do not fight: the Japanese ones at Lunga, with the airfield they had nearly built as work done on a base that has none yet, and the American engineers and pioneers who finished it. The record's dispute over the number of Japanese labourers matters again, as a range of work done and of men who must be fed.

## Why

- **It gives a reason to land men who do not fight,** and a cost: they eat, they take berths, and every one of them is not a rifleman.
- **An airfield becomes something won by work,** which is what the campaign was about.
- **One job at a time is a commander's choice:** the field, the beach, the aircraft or the defences. So is putting riflemen to dig: they are not training.
- **Each job acts on something a base already has.** Only the airfield itself is new.

## What it leaves out

- **Damage to an airfield or a port,** and so their repair. Bombs and shells still destroy aircraft, supply and men and leave the works whole (decision 0092). When damage comes, mending it is a fourth job.
- **Building a port's berths, guns, or a base for ship repair.**
- **Materials.** Work needs men and food and nothing else.
- **Working men pressed into a fight.**
- **Defences worn down** by shells, bombs or the fight.
- **More than one airfield, and the size of one.**

## What it costs

- **New state:** a unit's mark, works skill and activity; a base's airfield, the work done on it, its defences and its job; the work made that day, where the next day reads it. Each goes into the hash and is named beside it; the hash version is raised and the golden files remade (decision 0057).
- **A base's airfield is read by rebasing, by landing elsewhere and by the loader,** which did not ask before.
- **A new order, new events** with their rows in `side_events.rs`, scenario keys, and the agent interface.
- **Research,** before real units and real work can be set.
- **The rulebook:** chapter 7, chapter 2 (bases, working cargo), chapter 5 (repair, rebasing, landing elsewhere), chapter 1 and the glossary, in the same pull request as the code. It is built after decisions 0094 to 0098.

## Rule card

| | |
| --- | --- |
| Reads | A land unit's mark, activity, men ashore, hex, fed share and works skill; whether its hex was contested and whether it marched; the bases of its side in the hex, their job, airfield and work done; the four figures |
| Writes | A land unit's activity; a base's work done, airfield, defences and the day's work; the events. The cargo rule, aircraft repair, the fight, rebasing and landing elsewhere read them |

## Questions for the designer

1. **Decision 1:** a unit either fights or works. Agreed on Nov 14, 2026, with decision 3a added: a fighting unit can be set to work, at half the rate, and is then not training.
2. **Decision 4:** a base does one job at a time, set by order: the airfield, the port, the aircraft or the defences. Open.
3. **Decision 6:** a base formed in play has no airfield, and no aircraft can use it, until one is built. Agreed on Nov 14, 2026.
