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

## As built

Built on Nov 14, 2026 (scenario 34, state version 22, rules label 5; the hand calculation is in `tests/scenario_34_cover.rs`, on the placeholder map `data/map/placeholder_cover.toml`). The code is in `crates/sim-core/src/cover.rs` (cover, the cut, the pieces of a part, strain, the grades, digging and leaving fieldworks), `bombardment.rs` (`apply_hits_on_men`, which serves shells and bombs alike), `ground_combat.rs` (the fight's losses, `tire_or_rest`, the day's digging), `works.rs` (the defences job) and `resolve.rs` and `cargo.rs` (leaving fieldworks by an order or by embarking). Choices the record did not make:

- **The architect's correction of decision 2's wording,** made before the build: a call's loss (the hits times `men_lost_per_base_hit`, no more than the men there) is shared between the side's units in the hex in proportion, by the rule of decision 20 of 0066, exactly as before. Each unit's part P is then taken hit by hit, spread evenly: the piece of the k-th of h hits is P x k / h, cut, less P x (k - 1) / h, cut. Each piece is cut by the cover the unit has at that hit: times 1 less the cover over 100 times `cover_from_fire`, cut to a whole man. Sharing each hit's men apart would round differently whenever two units are hit and change the full slice on the frozen figures. With one unit every piece is `men_lost_per_base_hit`.
- **The order within one hit:** the hit meets the cover of the moment, its pieces are cut, and then the cover is worn. So the k-th hit meets the cover the call began with less k - 1 wears. The wear falls on the defences of the side's lowest-numbered base in the hex (whatever base the fire was aimed at) and on the fieldworks of each of its units with men ashore there, each not below 0.
- **The day's hits on men** are new state, `TruthState::hits_on_men`: for each hex and side, the hits and the events of the fire that made them (the bombardment or the air attack), in order. Each call adds to it, whether or not cover saved any man. The ground combat phase reads it when it settles each unit's fatigue and empties it at its end, so it is empty whenever the state is hashed. It is hashed after the land units.
- **Strain** is settled in `tire_or_rest`, with the cover the unit has then, before the day's digging and before a tired attacker goes back to defend. It is counted on the fatigue the unit had before the day: the strain is the hits times `strain_per_hit`, no more than `strain_per_day`, times 1 less the cover over 100 times `cover_from_strain`, and no more than takes the fatigue before the day to `strain_limit`. It is added beside whatever else the day gives (a march, an attack, a failed attack), to 100 at most. A unit under fire that neither marched nor fired on attack does not rest, and its change is `strained`. A unit that both fought and was fired on gains both. A unit whose fatigue does not change records no event, as before: a unit above the limit under fire simply stays where it was. The fatigue event cites the fire events beside its other causes, and its breakdown names the strain's factors only where the strain before the limit is above 0.
- **Digging** is the last step of the ground combat phase, after the day's activity (work, training, rehearsal) and the days of fighting, in unit order. A unit digs if it is a fighting unit with men ashore, on defend at that moment, with no march standing and not marched that day. A unit on attack that went back to defend that day is on defend then, and digs. Digging is not an activity: it does not stop a unit working, training or rehearsing. The event is `FieldworksChanged` (`dug`), with a breakdown of the fieldworks before, `fieldworks_per_day` and `fieldworks_limit`; a unit already at the limit records nothing.
- **Leaving fieldworks** (`FieldworksChanged`, `lost`, recorded only if the unit had any): at the intake of an attack order or of a march order (a follow-up of the order); when a beaten unit falls back or a spent attacker falls back (citing that event); and when any of a unit's men are taken aboard (citing the embarkation). A unit made by division takes its parent's fieldworks.
- **Cover** is `cover::unit_cover`, which the engine and a side's view both call (`SideView::cover`): 0 for a unit with a march standing or on attack, else the higher of its fieldworks and the defences of its side's lowest-numbered base in its hex. It reads only the unit and its side's bases. Working units dig nothing and are sheltered by a base's defences.
- **The fight:** the losses are shared in proportion as before, and then each unit's part is cut by its cover times `cover_in_fight`, cut to a whole man. The breakdown of `MenLostInFight` gains `men_before_cover`, `cover` and `cover_in_fight`, and that of `MenLostToFire` `men_before_cover`, `cover` (at the unit's first piece), `cover_from_fire` and `cover_wear_per_hit` (if more than one hit wore), only where cover saves men. A unit whose cut part is 0 records no loss.
- **The grades:** work on the defences is spent in stretches, each up to the next grade's start where the cost changes, at `work_per_defence_point` times the grade's factor: 1 below `prepared_from`, `prepared_work_factor` below `fortified_from`, and `fortified_work_factor` from there (not the two factors multiplied). With the frozen factors of 1 there is one stretch, and the work is the work over `work_per_defence_point`, to 100, as before. The `UnitWorked` breakdown names `prepared_work_factor` or `fortified_work_factor` only where work was spent at a factor above 1.
- **Events:** `FieldworksChanged` (unit, change `dug`, `worn` or `lost`, the fieldworks after) and `DefencesWorn` (base, side, hex, hits, the defences after, with a breakdown), recorded after the call's men lost, only where something changed; the change `Strained` of `LandUnitFatigue`. Each is given whole to the unit's or the base's side and to the enemy not at all.
- **Figures and keys.** The thirteen figures are required in `ground.toml`, live and frozen. The loader refuses shares outside 0 to 1, figures of fatigue and fieldworks outside 0 to 100, a `fieldworks_limit` above `prepared_from`, a `fortified_from` below `prepared_from` or above 100, and a work factor below 1. A scenario's `[[land_unit]]` takes `fieldworks`, from 0 to the limit, only for a fighting unit ashore on defend. An invariant checks that fieldworks are from 0 to the limit and 0 for a working unit, one on attack and one with a march.
- **What the frozen figures leave.** On the frozen figures no man is saved, nobody digs, every grade costs what a point cost and no hit wears. A unit under fire still does not rest; no frozen run has a unit with fatigue above 0 under fire, so none changes. Every scenario run on the frozen figures, the live files and the frozen copies alike, gives the event log it gave before with the hashes taken out.
