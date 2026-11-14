# Plan: land units and basic ground combat

Status: a plan by the architect, written on Nov 12, 2026, and approved by the designer the same day; the four questions it first left open are answered below. Stages 1, 2 and 3 are built (decisions 0066, 0090 and 0092); stage 4 is not. Each stage gets its decision record, with a rule card, before its code is delegated. Every figure named is a placeholder. Historical statements below are general and unsourced; any figure the game uses must come from sourced facts, as everywhere.

## Why now

Decision 0052 deferred ground combat, land units and shore bombardment to the scenario that first needs them. The designer has since decided that basic ground combat comes before the staff planner of step 3, so that a plan can say "take and hold a place" from the start (`docs/design/planning.md`, answer 1).

What step 1 cannot do without it (decision 0052, "What it costs"):

- No base changes hands. The fight for Henderson Field cannot be played.
- A transport's cargo is supply and fuel only. A reinforcement run carries nothing.
- Savo Island and the October bombardments run their sea and air parts and not their reason for being.
- No objective can be to take or hold ground (decision 0054, known limit).

## The test for every part

The pillars cut detailed ground combat: "land fighting is modelled at the level naval and air decisions need. This is not a ground-war game." So a part earns its place only if a decision at sea or in the air turns on it. Five do:

1. **A base can be taken.** That is why a convoy sails and why it is attacked.
2. **Men are carried by ship.** A reinforcement run that is sunk or turned back changes the fight ashore.
3. **Men ashore eat.** A side that cannot land supply loses its army without a battle.
4. **Ships and aircraft can hurt what is ashore.** A bombardment or a raid changes whether an airfield flies and whether an attack succeeds.
5. **It takes time.** A fight ashore lasts weeks, and the sea and air war is fought over those weeks.

Anything that serves none of these is left out, and the list of what is left out is below.

## The idea in one line

A land unit is a count of men in a hex that eats from its side's base there, is carried as cargo, and fights once a day by the hit mechanism the game already has; a side that is beaten in a hex loses its bases in it.

## What it reuses

Rule complexity asks for one mechanism over several. This plan adds one new kind of thing, the land unit, and one new phase. The rest is existing mechanism given one more case.

| Need | Reused from | What is added |
| --- | --- | --- |
| Carrying men | Cargo orders, berths and lighters, working cargo (decisions 0011, 0046) | Men as a third cargo, and a load order that names a land unit |
| Feeding men | Base supply and `BaseSupplySpent` (decision 0039) | A daily charge for each man, as there is for each aircraft |
| A landing place | A base. A hex may hold bases of both sides already | A base formed when a side lands in a hex where it has none, as an air group can be formed in play (decision 0059) |
| The fight | Shots, a hit chance, each shot rolled, with a breakdown (decisions 0014, 0033) | What a shot and a hit mean for men |
| Stopping an attack | Fatigue as one number and the stand-down threshold (decisions 0036, 0037) | Fatigue for a land unit |
| Shells and bombs on men | Where a hit on a base lands (decision 0034) | A third place for a hit to land: men |
| Shore bombardment | The attack on a base (decision 0034), fired by a force's guns | The empty phase of the night pulse filled |
| Knowing the enemy ashore | `BeliefState::apply_report` | A ground report and an estimate of men in a hex |
| Aircraft at a base that falls | `AirGroupLost` and aircraft arrivals (decisions 0025, 0041) | A base that changes side |
| Random draws | The `GroundCombat` subsystem code, which exists and is unused *(Nov 14, 2026: now used by `crates/sim-core/src/ground_combat.rs`)* | New `Draw` values, added at the end |

## The rules proposed

### The land unit

- A land unit has a side, a name, a count of men, fatigue, a posture and a place: ashore in a hex, or aboard a task force.
- A unit is whatever the record names as moving and fighting together: a regiment, a detachment, an echelon of one. Its size is its count of men, so no rule needs a table of unit sizes. Units are not split or merged by any rule; a formation that landed in two echelons is two units.
- A unit's men are a count, as an air group's pilots are (decision 0038). There are no weapons, no vehicles and no named officers in the rules.
- All units of a side ashore in one hex fight as one body. There is no position inside a hex.

### Carried by sea

- A ship class has a troop capacity, in men, beside its cargo capacity. For a real class it is derived from fact rows by a written rule, like every other figure of a real ship.
- A load order may name a land unit at the force's hex. The unit's men go aboard through the cargo mechanism, at a rate in men an hour by berth or by lighter, and are spread over the ships in ship order up to each ship's troop capacity.
- An unload order lands men first, then supply, then fuel.
- A unit is ashore when all its men are ashore, and aboard when all are aboard. While it is part one and part the other it does not fight, and the men ashore still eat.
- A ship that sinks takes the men aboard her with her. Most such men were saved in the record; it is a known error of the same kind as pilots aboard a sinking carrier (decision 0059, item 10).
- A force may unload in a hex where its side holds no base. Doing so forms a base of its side there, with port size as the hex has it, no stock and no guns. This is how a landing begins, and how a beach that destroyers run supply to is represented.

### Supply ashore

- Each night, after the daily supply for aircraft, each base pays a daily charge for each man of its side ashore in its hex, as much as its stock covers, recorded as `BaseSupplySpent` with its own reason.
- Men for whom nothing was paid are unfed. A share of the unfed men is lost each day, and a unit's fighting value is cut by the share of its men who went unfed. This one rule is the reason to land supply and the reason to sink it.
- A unit in a hex where its side has no base is unfed.

### The fight

- A unit's posture is set by order: defend, the default, or attack. An attack order stands until replaced, like every order.
- A hex is contested on a day when land units of both sides are ashore in it and at least one is on attack. Each contested hex has one fight, in the ground combat phase, after the third pulse.
- Each side's men, cut for fatigue and for the unfed, give a number of shots at so many men to a shot. A shot hits at a chance that is one figure, raised for a defender. Each shot is rolled. A hit removes a fixed number of men, taken from the other side's units in proportion to their size. Both sides fire on the strengths they had at the start, so the order of firing does not matter.
- A side on defend in a contested hex fires as a defender. If both sides attack, neither does.
- Attacking raises a unit's fatigue. Resting lowers it. A unit whose fatigue is over a threshold goes back to defend by itself, as a tired air group stands down, and its attack order is spent.
- **A side is beaten in a hex** when, after the day's fight, its men ashore are none, or are fewer than a set share of the attacker's. Its bases in that hex then change side.
- A unit with no men is destroyed.

### A base that changes side

- The base keeps its number and its hex. Its side changes.
- A share of its supply and fuel is captured and the rest is lost.
- Air groups at it are lost with their aircraft, by the event that records a group lost with its ship.
- The beaten side's men in the hex stay ashore with no base to feed them, unless they hold another base there. They can be taken off by sea.
- An arrival of aircraft set for a base (decision 0041) happens only if the base is then its side's. Rebasing by order is built (decision 0091), so a side may fly a group out of a base before it falls.

### Shells and bombs

- **Shore bombardment.** A force with the bombardment mission, in a hex with an enemy base in the night pulse, fires on it. Each ship's guns give shots by the rule surface combat uses for shots; each hits at one bombardment figure; each hit lands by the table that says where a bomb on a base lands. The mission exists and does nothing today. *(Nov 14, 2026: built by decision 0092, `crates/sim-core/src/bombardment.rs`.)*
- **Men as a place for a hit to land.** The table that sends a hit on a base to aircraft, to supply or to nothing gains a share that lands on men, when the base's side has men ashore in the hex. Each such hit removes a fixed number of men. This serves bombs and shells alike.
- **The ground support mission stays deferred.** A base strike already reaches men through the table. A mission of its own waits until something needs it.

### What a side knows

- A side knows its own units exactly.
- It knows the enemy ashore only as an estimate for a hex: a count of men and the day it was last reported. The estimate is written only by a ground report through `apply_report`.
- A fight gives each side a ground report of the other: the true count with an error drawn for it, keyed like every draw. Landing in a hex where the enemy has men, or having men landed on, gives one too.
- A scenario may give a side an opening estimate. For a historical scenario it is what that side believed, from a source. An estimate that is badly wrong is the point: a side that believes 2,000 men hold the airfield attacks with too few. This is how a side's hopefulness reaches the staff's estimates without a rule for it (`docs/design/planning.md`, answer 2).
- A side sees its own losses whole and the enemy's as a claim. A base changing side is seen by both.
- Each new event is given its row in `side_events.rs`, decided and not defaulted, and the leak tests of decision 0047 are extended to it.

### Objectives

Two kinds are added to the five of decision 0054, each a count read from state:

- `base_held`: 1 if a named base is the side's at the end.
- `men_lost`: enemy men lost, by any cause, with points for each.

## What is left out, on purpose

Each of these is real and was considered. None is needed for a naval or air decision in the first build, and each can be added later as its own rule.

| Left out | What it costs | When it might come |
| --- | --- | --- |
| Movement overland between hexes | The Kokoda Track cannot be played; men move only by sea | When a scenario in New Guinea needs it (answer 1 below) *(Nov 14, 2026: marching between linked hexes is built, decision 0094, `crates/sim-core/src/march.rs`.)* |
| Fire from the shore on ships landing men | An opposed landing costs no ships | With the first scenario of an assault on a defended beach |
| Artillery ashore closing an airfield | Guns on the island cannot stop Henderson Field flying | With damage to a base's airfield (decision 0052, item 12) |
| Building and repairing airfields | A base's airfield is given by the scenario | With base damage and base orders |
| Sickness apart from hunger | Malaria takes nobody who is fed | Only if the record shows the tuning cannot do without it |
| Terrain | Every land hex is jungle (decision 0052, item 14), so a terrain figure would be one constant | When the map carries real terrain |
| Kinds of unit, weapons, armour, engineers | A man is a man | Not planned. The pillars cut it |
| Leaders and their qualities | People are names no rule reads (`docs/design/storytelling.md`, answer 1) | Not planned |
| Men saved from a sinking ship | Too many are lost with a transport | With a roster of survivors, as for pilots |
| Dug-in defence that grows with time | The defender's advantage is one figure | Only if the record needs it |

## Rule card, in outline

Each stage's decision record carries its own. Taken together:

| | |
| --- | --- |
| Reads | A land unit's men, fatigue, posture and place; a ship's troop capacity and the men aboard; a base's side, supply and fuel; the hex's port; a force's mission and guns; the ground figures |
| Writes | A land unit's men, fatigue, posture and place; men aboard a ship; a base's side, supply and fuel; a base formed; an air group lost; a side's estimate of men in a hex; the ground events |

The one place where interactions can hide is a base's side. Many rules read it: cargo, refuelling, air groups, raids, objectives. Until now it never changed. Stage 2 below therefore includes a review of every rule that asks whose a base is.

## What it changes in the engine

- **State:** land units, men aboard ships, a side's ground estimates, and a base's side becoming something that changes. Each new field goes into `hash::hash_world` and `hash::name_every_field`; the hash version is raised and the golden files remade (decision 0057). This is done once for each stage that adds state, not once for each field.
- **Stable codes:** a kind code for a land unit and the new `Draw` values, each added at the end. The `GroundCombat` subsystem code is already there.
- **Figures:** a new `ground_combat.toml`, and its first values added to the frozen step 1 figures as the tests require. A scenario that does not name the file loads no ground rule, so every existing scenario runs as it does now.
- **Data:** a land unit entry in the scenario file; troop capacity on a ship class; an opening estimate.
- **The agent interface:** the briefing gains a part for land units and the estimate of the enemy ashore, with a schema change; the queries gain a land unit; orders gain posture and the load order that names a unit.
- **The viewer:** land units and a base's side by day.
- **The rulebook:** a new chapter 7, ground combat, and changes to chapters 1 (the order of a day, orders, what a side sees), 2 (cargo), 5 (the attack on a base) and 6 (objectives), with the glossary, in the pull request of each stage.

## Stages

One stage is one branch and one pull request. Each has its scenario, its hand calculation written by the architect before code is delegated, its property tests and its legibility test. Stages 1 to 3 use made-up units, marked placeholder and named "Placeholder".

| Stage | What is built | Its scenario shows | Property tests |
| --- | --- | --- | --- |
| 0. Facts (done: `docs/research/oob/land-forces.md`) | Research into `data/oob`: the land units of August to October 1942 on Guadalcanal and Tulagi, with men and dates landed; what each side believed of the other's strength; troop capacity of the transports and destroyers used. No code | Nothing: it is data | The order-of-battle check |
| 1. Men as cargo, and eating (decision 0066) | The land unit, troop capacity, loading and landing men, a base formed by landing, the daily charge, the unfed | A convoy lands a unit and supply on an empty shore; the unit eats; a second unit with no supply wastes away | Men are conserved between ship and shore; more supply never loses more men |
| 2. The fight and the base | Posture, the daily fight, fatigue, a side beaten, a base changing side, the two objectives, the review of every rule that reads a base's side | An attack that fails against a fed defender, and one that takes the base from an unfed one | More attackers never lose the attacker more ground; a fight never creates men |
| 3. Shells, bombs and belief | Shore bombardment, hits on men, the ground report and the estimate | A bombardment by night that destroys aircraft and supply at a base; an attack ordered on a wrong estimate | A side's view is the same in two worlds that differ only in enemy men it has no report of |
| 4. The record | The landings of August 1942 and the first attacks on the airfield, from stage 0's facts, as a scenario of its own; then land units added to the full slice and the soak | The airfield taken on the second day, and held against the attacks the record names | The invariants of the soak hold with land units in play |

After stage 2, the rule cards of stages 1 and 2 are read beside the existing ones for whose-base-is-it, in the manner of the review after scenario 5.

## Where it sits beside step 2

Step 2 is under way and its spec says ground combat is not in it. Three things keep the two apart. *(Nov 14, 2026: step 2 closed on Nov 13, 2026, and stages 1 to 3 are built: decisions 0066, 0090 and 0092.)*

- **Files.** Step 2 has three rules still to land in `sim-core` (strike target priority, the rest of ditching, experience), each raising the hash version. Stages 1 to 3 touch the same files. Two developers never edit the same files at once, so the code of stage 1 starts when those three have merged.
- **Stage 0 can start now.** It is research and touches no file step 2 is working in.
- **Figures.** The ground figures are placeholders like all the others. They get a few coarse measures and no more (answer 2 below).

## Risks

- **A base that changes side is the widest change since belief.** A rule that assumed a base's side is fixed will be wrong without failing a test. The review in stage 2 is the guard, and the leak tests are the second.
- **The sources for land strengths are thinner and more disputed than for ships,** above all for the Japanese. Stage 0 may return ranges and not figures, and a measure would then span a dispute.
- **Scope.** Every row of the left-out table will look reasonable to add. The test stays: does a decision at sea or in the air turn on it.
- **A fight that is decided in a day.** If the figures let an attack take a base at once, there is no campaign. The scenario of stage 4 is where that shows, and it is a matter of figures, not of rules.

## Answers from the product managers

Agreed by the designer on Nov 12, 2026, on the architect's suggestion.

1. **Men move only by sea in the first build.** Movement overland between hexes is a later rule of its own. The slice as far as the kill gate is the fight for Guadalcanal and Tulagi; the land fight in New Guinea waits. Note, Nov 14, 2026: marching overland between linked hexes is now built by decision 0094.
2. **The ground figures get a few coarse measures and no more.** Two or three, such as whether each attack on the airfield that the record names failed, and how fast an unfed force wasted. They are enough to make the campaign plausible. The measures belong to `docs/step2-spec.md` and `data/validation/`, which the second session owns: it adds them when stage 4 gives them something to measure, from the facts of stage 0.
3. **Men aboard a ship that sinks are all lost,** as a known error, until there is a way to hold survivors who have no unit to go back to. It is put right together with pilots aboard a sinking carrier (decision 0059, item 10).
4. **The plan is approved, and stage 0 starts now.** The code of stage 1 waits for step 2's three rules to land, as above.
