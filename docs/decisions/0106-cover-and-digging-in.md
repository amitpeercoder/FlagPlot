# 0106: Cover, men who dig in, and a place under bombardment

Status: a plan by the architect, written on Nov 14, 2026, after the designer said that well-prepared defenders were massively bombed and did not suffer heavily for it, and revised the same day on his answers. He agreed that troops dig shallow fieldworks for themselves (decision 5). He ruled that protection depends on how deep the preparation is, with a year's work a different thing from a week's (decisions 8 and 9), and that a long bombardment must tell even where the works hold: on the men's nerves and sleep, and slowly on the works (decisions 10 to 12). Confirmed as revised. Nothing is built. Every figure is a placeholder.

## The gap

A base's defences, built by work (decision 0099, 4), do one thing: they raise the chance to hit of a side that defends there. They do not shelter anyone. A man behind full defences is lost to a bomb, a shell or a rifle as easily as a man in the open (decisions 0092 and 0090). So bombardment before an assault is always worth its shells, and a prepared position cannot be what it was: a place that takes a great weight of fire and is still held when the firing stops.

Three more things are missing. Every defence is one kind, and full defences cost a labour battalion twenty days: troops cannot scrape holes where they stand, and nothing separates a week of digging from a year of tunnelling. Nothing can be dug where a side has no base. And fire that kills nobody does nothing at all.

## Decisions

### Cover

1. **Cover is a number from 0 to 100 for a unit in a hex.** It is the higher of two things: the defences of its side's lowest-numbered base in the hex (decision 0099), and the unit's own fieldworks (decision 5). A unit that is marching, or on attack, has no cover: it is out of its holes.
2. **Cover saves men from bombs and shells.** The men a hit on men removes (decision 0092) are cut for each unit by its cover: the men it would lose, times 1 less its cover over 100 times `cover_from_fire`, cut to a whole man. (Nov 14, 2026, as built: the call's loss is shared between units first, as before, and each unit's part is then taken hit by hit.)
3. **Cover saves men in the fight.** The men a side's hits remove from a unit that defends (decision 0090) are cut in the same way by `cover_in_fight`. The defences' effect on the defender's own chance to hit stays as it is.
4. **Cover does not shelter aircraft, supply or the airfield.** They are hit as now.

### Fieldworks

5. **Men dig in where they stand.** A fighting unit on defend that did not march that day gains `fieldworks_per_day` of fieldworks, up to `fieldworks_limit`. It needs no base, no order and no labourers, and it does not stop a unit training or rehearsing. It digs on a day it is fought or fired on as on any other. Working units do not dig for themselves; they are sheltered by a base's defences if it has them. *Agreed by the designer, Nov 14, 2026.*
6. **Fieldworks are the unit's and are lost when it leaves them.** They go to 0 when the unit marches, falls back, is taken aboard, or is set to attack. A unit made by division starts with those of the unit it left.

### How deep the preparation is

7. **The scale has three grades,** so that a player can read a number:

   | Cover | Grade | Who makes it |
   | --- | --- | --- |
   | Below `prepared_from` | Improvised: holes and scrapes | The troops, as fieldworks, in days; or labour |
   | From `prepared_from` to below `fortified_from` | Prepared: trenches, wire, dug-in guns | Labour at a base, in weeks |
   | From `fortified_from` to 100 | Fortified: tunnels, concrete, caves | Labour at a base, in months |

8. **Each grade costs more work than the one before.** A point of a base's defences costs `work_per_defence_point` (decision 0099) in the improvised grade, that times `prepared_work_factor` in the prepared grade, and that times `fortified_work_factor` in the fortified grade. A day's work that crosses from one grade to the next is spent point by point at each grade's cost. Fieldworks never pass `fieldworks_limit`, which is the top of the improvised grade.
9. **Protection rises with the number,** by decisions 2 and 3, so it depends on the cover and the preparation: little behind a scrape, most behind a year's work. *The designer's ruling, Nov 14, 2026.*

### A place under bombardment

*The designer's ruling, Nov 14, 2026: a place bombarded for a month must not be untouched. Even where the cover holds, the men lose their nerve and their sleep.*

10. **Men under fire do not rest, and tire.** On a day when at least one bomb or shell landed on men of a unit's side in its hex, the unit does not rest that day. Instead it gains `strain_per_hit` of fatigue for each such hit, no more than `strain_per_day` in a day, cut by its cover: times 1 less its cover over 100 times `cover_from_strain`. Strain alone raises no unit's fatigue above `strain_limit`: worn men still fight. A hit counts whether or not cover saved the men it would have taken.
11. **Fatigue does what it does already.** It cuts the men who fire (decision 0090, 6). So a garrison shelled for weeks meets the assault with part of its fire, and a commander who bombards is buying that as well as casualties.
12. **Fire wears cover down, slowly.** Each hit that lands on men lowers the defences of that side's lowest-numbered base in the hex by `cover_wear_per_hit`, and the fieldworks of each of its units there by the same, not below 0. Troops dig their fieldworks back by decision 5. A base's defences are mended only by work on the defences, at the cost of the grade they are in. So a defender under long fire must keep men at the defences or lose them over weeks.
13. **Assault does not wear cover down.** Only bombs and shells do. Breaching by engineers is left out.

### What a side knows

14. **A side sees its own units' cover, strain and the wear on its defences whole.** The enemy sees none of it: a base's defences and a unit's fieldworks are not in what it is told, and a raid's claim says nothing of them or of what the fire did to the men's state. A side learns that an enemy is well dug in, or worn down, by what its own attacks then cost and achieve.

## Placeholder figures

In `ground.toml`:

| Key | Value | Frozen step 1 value | What it is |
| --- | --- | --- | --- |
| `cover_from_fire` | `"0.9"` | `0` | The share of losses to bombs and shells that full cover saves |
| `cover_in_fight` | `"0.5"` | `0` | The share of losses in the fight that full cover saves a defender |
| `fieldworks_per_day` | `10` | `0` | Fieldworks a unit gains in a day on the defence |
| `fieldworks_limit` | `30` | `0` | The most fieldworks a unit can dig for itself |
| `prepared_from` | `30` | `30` | The cover at which the prepared grade begins |
| `fortified_from` | `60` | `60` | The cover at which the fortified grade begins |
| `prepared_work_factor` | `3` | `1` | How many times dearer a point is in the prepared grade |
| `fortified_work_factor` | `10` | `1` | How many times dearer a point is in the fortified grade |
| `strain_per_hit` | `1` | `0` | Fatigue a unit gains for each hit on its side's men in its hex |
| `strain_per_day` | `10` | `0` | The most fatigue strain adds in a day |
| `strain_limit` | `60` | `60` | The fatigue above which strain adds nothing |
| `cover_from_strain` | `"0.5"` | `0` | The share of strain that full cover saves |
| `cover_wear_per_hit` | `"0.2"` | `0` | Cover a hit on men wears away |

None rests on a source. The frozen values make the rule do nothing, so every earlier hand calculation stands: the work factors are 1, so a point of defences costs what it did in decision 0099's calculation.

With the live values: a unit that has stood three days has fieldworks of 30 and loses 27 in 100 fewer men to shelling. Full defences cost 30 x 200 + 30 x 600 + 40 x 2,000 = 104,000 of work: 104 days for a labour battalion of 1,000, where they cost 20 days before. Behind them a hit on men takes one man in ten, a day of heavy shelling tires a unit by 5 and not 10, and a month of daily raids with five hits on men each wears the defences from 100 to 70 unless they are mended.

## What it changes in the August scenario

The Marines at Lunga dig in from the day the field is theirs and have fieldworks of 30 within three days. The Japanese shellings and raids of the period cost them fewer men and some rest. The Japanese garrisons of 7 August have what the scenario gives their bases; no row sizes that, so it stays 0 until research gives one, and the scenario says so.

## Why

- **Bombardment becomes a judgement with a return that grows with time.** A day of it against a prepared position buys little; weeks of it buy tired defenders and worn works, at the price of the shells, the ships' time and the warning given.
- **Time on the defence is worth something by itself,** and labour buys what troops cannot dig.
- **A year's preparation is a different thing from a week's,** and a player can read which he faces in his own positions.
- **It reuses what exists:** the hit on men, the losses of the fight, fatigue, and work on the defences.

## What it leaves out

- **Breaching by assault or by engineers, and shells or bombs heavy enough to wreck a grade at once.**
- **Cover for aircraft, supply, guns and the airfield.**
- **Terrain as cover:** jungle, caves and ridges that cost no labour.
- **Morale apart from fatigue:** men who break, as against men who are tired.
- **The enemy learning of defences** by reconnaissance.

## What it costs

- **New state:** a unit's fieldworks, and the day's hits on men of each side in each hex, in the hash and named beside it; the hash version and the rules label are raised and the generated files remade.
- **Factors in the breakdowns** of men lost to fire, men lost in the fight, the day's fatigue and the work on defences, named only where the rule acts; events for fieldworks, strain and wear, with their rows in `side_events.rs`.
- **A scenario key** for a unit's fieldworks at the start.
- **Decision 0099's defences job** gains the grades. Its hand calculation stands on the frozen factors of 1.
- **The live baselines of every scenario with men ashore under fire or in a fight** change.
- **The rulebook:** chapter 7 ("Losses", "Fatigue", "Men lost to fire", "Units that work"), chapters 4 and 5, and the glossary, in the same pull request. It is built after decision 0105, which changes the same fight.

## Rule card

| | |
| --- | --- |
| Reads | A unit's posture, march, fatigue and fieldworks; the defences of its side's base in the hex; the men a hit or a fight would remove; the day's hits on men of the side in the hex; a unit's work on the defences; the figures |
| Writes | A unit's fieldworks and fatigue; the men a unit loses to fire and in the fight; a base's defences; the day's hits on men; the events |

## Questions for the designer

Answered on Nov 14, 2026.

1. **Decisions 2 and 3:** cover saves men from bombs and shells and in the fight. The designer answered that it depends on the cover and the preparation, with a year's work in mind. Decisions 7 to 9 are the answer: three grades, each dearer than the last, and protection that rises with the number.
2. **Decision 5:** troops dig shallow fieldworks for themselves. Agreed.
3. **Decision 13 of the first draft, that nothing wears cover down:** rejected. The designer ruled that long bombardment must tell, on the men above all. Decisions 10 to 12 are the answer.
