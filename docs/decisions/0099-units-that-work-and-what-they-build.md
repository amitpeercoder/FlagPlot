# 0099: Units that work, and what they build

Status: a plan by the architect, written on Nov 14, 2026, after the designer said that not every man ashore does the same job: construction battalions built and kept up the bases. The designer agreed decisions 1 and 6 on Nov 14, 2026, and added that fighting units may be put to work, less well, and that defences are something to build (decisions 3a and 4). On decision 4 he chose that each unit has its own job. Of it, the one activity a day (3b to 3d) and the order `set_activity` with the activities ready and train are built with decision 0097 (Nov 14, 2026, scenario 28); the rest is not built. Every figure is a placeholder. No real unit has a value yet.

Summary: Units that work, and what they build: a unit fights or works; a working unit makes work each day at its side's base, at a job of its own set by order (the airfield, the port, the aircraft or the defences); a fighting unit can be set to work at half the rate; a base formed in play has no airfield until one is built, and no aircraft uses a base without one. Placeholder figures

Standing: Confirmed by the designer, Nov 14, 2026. The one activity a day and the activities ready and train were built with 0097 (scenario 28). Built Nov 14, 2026 (scenario 30)

## The gap

Every man ashore is a rifleman. A base's airfield, its rate of unloading and its aircraft repairs are given by the scenario and never change. So nobody builds anything, there is no reason to land engineers, and a beach taken yesterday flies aircraft as well as a base a year old.

Decision 0093, 6, left construction workers out of the scenario of the August landings because, counted as soldiers, they would have fought for the airfield they in fact left. That was the right answer to the wrong choice. They belong in the game as what they were.

The plan for ground combat deferred "building and repairing airfields" until base orders existed (`docs/design/ground-combat.md`, "What is left out"). This is that rule, in its smallest form.

## Decisions

### Units that work

1. **A land unit fights or works.** A scenario marks a unit as a working unit. Its men do not fire in a fight, and they are not counted when a side is tested for being beaten: labourers do not hold a base. In everything else they are men: they are carried, landed, fed, hit in proportion with the rest, and lost. A unit not so marked is as it is today. *Agreed by the designer, Nov 14, 2026: a construction battalion is a unit.*
2. **A working unit has a works skill, from 0 to 4,** 2 being the standard for a unit raised to build. It is the fifth skill beside those of decision 0098, and like them it is fixed.
3. **It makes work each day.** In the ground combat phase, after the fights, each working unit with men ashore in a hex that holds a base of its side makes work: its men ashore, times its fed share, times `work_per_man_day`, times its works skill over 2. It makes none on a day its hex is contested, or a day it marched.

3a. **A fighting unit can be put to work, and does it less well,** by the order of decision 4. A fighting unit set to work makes work as a working unit does, times `fighting_unit_work_share`, with a works skill of 2 unless the scenario gives it one. A day it works is not a day of training or of rehearsal. If its hex is contested it fights as any unit does and makes no work that day. *Added by the designer, Nov 14, 2026: other units may spare men to build defences, and those men are not training or fighting.*

### One activity a day

3b. **A unit does one thing in a day, and this rule says which.** Rehearsal (decision 0095), training (decision 0097) and work all read it, so that no day is counted twice. It is worked out once a day, in the ground combat phase, after the marches and the fights, when it is known whether the unit marched, whether its hex was contested and how it was fed that night. Every gain of the day is given then:

   | In this order | The unit's day is |
   | --- | --- |
   | It marched, or its hex is contested | None of the three. It marches or fights |
   | Its activity is work | Work, at its job |
   | Its activity is train | Training, if the conditions of decision 0097, 3, hold; otherwise nothing |
   | It stands ready and is on the roster of a force preparing a landing, in that force's hex | Rehearsal for that landing |
   | It stands ready otherwise | Nothing |

   So rehearsal is what a roster unit does when it is not told to train or to work, and the staff schedules a raw unit to train first and rehearse after by changing its activity. Its place on the roster does not change, so the force's loading is not cut for it.
3c. **A unit is on one roster at a time.** A prepare order that names a unit already on another force's roster takes it from that roster. Taking a unit off a roster costs the force nothing.
3d. **A day's gain is given once.** Each of the three is added once for a unit for a day, whatever orders were given that day and however often the day is worked out again from a saved game. A unit at its ceiling of training, or at 100 of rehearsal, still spends the day.

### What work does

4. **Each unit that works has its own job, set by order.** `set_activity` names a land unit and what it does: a working unit works at a job; a fighting unit stands ready, trains, or works at a job. The order stands until replaced. A unit's work goes to its job at its side's lowest-numbered base in its hex. The jobs are four. *The designer's choice, Nov 14, 2026. The architect had proposed one job for a base at a time.* The staff of step 3 sets the jobs, and its report says what each base is getting.

   | Job | What the day's work does |
   | --- | --- |
   | Airfield | Adds to the work done on the base's airfield, until it is built (decision 5) |
   | Port | The next day, ships at the base work cargo at their rate times 1 plus the work over `work_to_double_unloading`, and no more than twice their rate |
   | Aircraft | The next night, the base repairs one more aircraft for each `work_per_aircraft_repair` of work, cut to a whole aircraft |
   | Defences | Adds to the base's defences, a number from 0 to 100: one for each `work_per_defence_point` of work. While a side defends in a hex, its units' chance to hit is multiplied by 1 plus `defences_strength` times the defences of its lowest-numbered base there over 100 |

   A working unit with no order builds the airfield if its base has none, and else works the port. The work of several units on one job at a base is added.
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

None rests on a source. With them a standard battalion of 1,000 builds an airfield in 20 days, doubles a beach's unloading, adds 5 aircraft to a base's nightly repairs, or digs full defences in 20 days. A regiment of 3,000 riflemen set to dig does the last in about 13 days and trains for none of them. The record has what stage 4 of ground combat (decision 0093) needs to check the first: the field at Lunga was begun in July, was nearly done on 7 August, and flew its first American aircraft on 20 August.

## What it changes in the August scenario

Decision 0093, 6, stands as to who fights. The construction units come back as working units that do not fight: the Japanese ones at Lunga, with the airfield they had nearly built as work done on a base that has none yet, and the American engineers and pioneers who finished it. The record's dispute over the number of Japanese labourers matters again, as a range of work done and of men who must be fed.

## Why

- **It gives a reason to land men who do not fight,** and a cost: they eat, they take berths, and every one of them is not a rifleman.
- **An airfield becomes something won by work,** which is what the campaign was about.
- **Where each unit's work goes is a choice:** the field, the beach, the aircraft or the defences. So is putting riflemen to dig: they are not training.
- **Each job acts on something a base already has.** Only the airfield itself is new.

## What it leaves out

- **Damage to an airfield or a port,** and so their repair. Bombs and shells still destroy aircraft, supply and men and leave the works whole (decision 0092). When damage comes, mending it is a fourth job.
- **Building a port's berths, guns, or a base for ship repair.**
- **Materials.** Work needs men and food and nothing else.
- **Working men pressed into a fight.**
- **Defences worn down** by shells, bombs or the fight.
- **More than one airfield, and the size of one.**

## What it costs

- **New state:** a unit's mark, works skill, activity and job; a base's airfield, the work done on it and its defences; the work made that day, where the next day reads it. Each goes into the hash and is named beside it; the hash version is raised and the golden files remade (decision 0057).
- **A base's airfield is read by rebasing, by landing elsewhere and by the loader,** which did not ask before.
- **A new order, new events** with their rows in `side_events.rs`, scenario keys, and the agent interface.
- **Research,** before real units and real work can be set.
- **The rulebook:** chapter 7, chapter 2 (bases, working cargo), chapter 5 (repair, rebasing, landing elsewhere), chapter 1 and the glossary, in the same pull request as the code. It is built after decisions 0094 to 0098.

## Rulings by the architect, with the hand calculation

Made on Nov 14, 2026, while writing the hand calculation of scenario 30 (`tests/scenario_30_working_units.rs`). Each fills a place the decisions above left open.

1. **Only fighting men make a hex contested or hold a base.** Decision 1 says working men are not counted when a side is tested for being beaten. The same holds wherever the rules of the fight count a side's men ashore: a hex is contested when both sides have men of fighting units ashore and a unit is on attack, and a base whose side has only working men in the hex falls without a fight, as a base with no men does (decision 0090, 16). A working unit is always on defend, and an attack order to it is refused. Its men are still hit in proportion with the rest when the fighting men beside them are fired on.
2. **Work on an airfield.** The day's work is added whole on the day the airfield is built, and nothing is added after. A unit with no order then works the port. A scenario may state a base with no airfield and with work already done on one.
3. **Defences are not cut to a whole number.** 450 of work is 2.25 points.
4. **Work on aircraft makes a base a place of repair.** A night's repairs at a base are its own figure and the repairs the work of the day before gives, also where its own figure is 0.

## Rule card

| | |
| --- | --- |
| Reads | A land unit's mark, activity, roster, men ashore, hex, fed share and works skill; whether its hex was contested and whether it marched; its job; the bases of its side in the hex, their airfield and work done; the four figures |
| Writes | A land unit's activity and job; a base's work done, airfield, defences and the day's work; the events. The cargo rule, aircraft repair, the fight, rebasing and landing elsewhere read them |

## Questions for the designer

1. **Decision 1:** a unit either fights or works. Agreed on Nov 14, 2026, with decision 3a added: a fighting unit can be set to work, at half the rate, and is then not training.
2. **Decision 4:** each unit has its own job, set by order: the airfield, the port, the aircraft or the defences. The designer's choice, Nov 14, 2026.
3. **Decision 6:** a base formed in play has no airfield, and no aircraft can use it, until one is built. Agreed on Nov 14, 2026.

## As built

Built on Nov 14, 2026 (scenario 30, state version 18; the hand calculation is in `tests/scenario_30_working_units.rs`, on the placeholder map `data/map/placeholder_works.toml`). Every number of the hand calculation matched the run. The code of the rule is `crates/sim-core/src/works.rs`, called from the day's activity in `ground_combat.rs`. Choices the record did not make:

- **State.** A land unit's `working` mark and a fifth skill `works` in its skills; the activity `Work`, with the job named or none for the default job, so a unit's job is part of its activity. A base's `airfield`, `airfield_work` (the work done on it), `defences`, and `day_work` (the day, and the work on the port and on the aircraft made that day): the next day reads it only if its day is the day before, so no reset is needed and none is recorded. Hash version 18: the base's four fields after `on_map`, the job after the activity's code, `works` after `atoll` and `working` after the skills.
- **A working unit's activity is always work.** The loader sets it to work at the default job; an order to stand ready or train it is refused (`WorkingUnitWorks`, "a working unit works: it does not stand ready or train"), and one to work at a job is accepted. An attack order to it is refused (`WorkingUnitDefends`), and a scenario may not start it on attack. An invariant checks both.
- **The default job** is worked out each day at the unit's base: the airfield if it has none, else the port. A `work` order with no job means the default job, for a fighting unit too.
- **Working men record no fire event:** a working unit is passed over when the units fire, so it has no `GroundFire` and no day of fighting counted. It is in the loss, in proportion, by the rule of decision 0066, 20.
- **"Fighting men"** are the men of units without the mark. A fighting unit set to work is a fighting unit for a hex contested, a side beaten and a base held. Both sides' counts in the test of a side beaten, and the event `SideBeatenAshore`, count fighting men only. The ground report and the estimate count every man ashore, working men too.
- **Work is made after the fights and any base changing side,** in unit order, at the side's lowest-numbered base in the unit's hex at that moment. A day's work of 0 records nothing. The event is `UnitWorked` (the unit, the base, the job, the work and the base's total for the job after it: work done on the airfield, the day's work on the port or the aircraft, or the defences), citing the activity order and the night's feeding in the hex. Its breakdown names the men ashore, the fed share, `work_per_man_day` and the works skill, with `fighting_unit_work_share` for a fighting unit and `work_per_defence_point` for defences.
- **An airfield is built once every unit has worked:** each base worked on that day whose work done has reached the figure gets its airfield, with `AirfieldBuilt` (the base, the builder's side, the hex, the work done) citing the day's work on it. A unit ordered to the airfield of a base that has one makes work that adds nothing, and the event says so by its total.
- **The port factor** multiplies every ship's rate at the base the next day, berth or lighter, for loads and unloads and for men as for cargo, before the landing factor of decision 0095. It reads the side's lowest-numbered base in the hex as the cargo rule finds it; a base formed in the pulse has none. The cargo breakdowns gain `port_work` and `port_work_factor`, only where the day before's work was above 0; the cargo events cite that work.
- **The extra repairs** are added to a base's own figure for the night (also where it is 0) and the repair events cite the work. The repair rule still needs the wear figures.
- **Defences** multiply the defender's chance with its other factors before the chance is cut to 0 to 1. The `GroundFire` breakdown gains `side_chance`, `defences` and `defences_factor` only where the factor is not 1; a side's own cut keeps `defences` and `defences_factor`. A work order on defences at a base at 100 makes work that adds nothing.
- **No aircraft uses a base without an airfield:** a rebase to one is refused (`NoAirfield`, "the base has no airfield", tried after "already at the base" and before the transfer range), aircraft that cannot get home do not land at one, and the loader refuses a group placed at one. An invariant checks that no group is at one.
- **A part of a working unit made by division** is a working unit at the default job, with its parent's skills and experience.
- **Scenario keys.** On `[[base]]`: `airfield` (default true), `airfield_work` (only with `airfield = false`) and `defences` (0 to 100). On `[[land_unit]]`: `working = true` and `works` in `skills`. In orders: `set_activity` with `activity = "work"` and an optional `job` (airfield, port, aircraft, defences).
- **What a side sees.** `UnitWorked` is seen whole by the unit's side and `AirfieldBuilt` by the builder's side; the enemy sees neither. An enemy base on the map carries no word on its airfield, work or defences (`PublicBase` has no field for them). The agent shows the side's own working mark, activity and job (with whether it is the default) and the works skill, and for its own bases a `works` part (airfield, work done and the work needed, defences, yesterday's work on the port and the aircraft) only where a base has no airfield, any work done, defences or yesterday's work; the briefing lists yesterday's work and airfields built. The MCP instructions gained a paragraph, "WORK". The schemas stay at version 3: fields only.
- **The figures** are the seven of the record, in `data/reference/ground.toml` and with the same values in the frozen copy, required in both.
- **Every existing scenario gives what it gave before.** The event logs of every run baseline, with hashes taken out, are as they were; the only other change is in the description of the state at the end, where the bases formed in play in scenarios 21, 25, 27 and 29 are now said to have "no airfield". No scenario or recorded game has aircraft that use a base formed in play. The golden briefings of scenario 21 gain one line for the formed base's works. The recorded games, the batch fixture of scenario 6 and the pinned hashes of the tests of scenarios 10 and 17a changed in their hashes only.
