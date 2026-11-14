# Flag Plot — Step 3 Spec: Staff Planner

Status: confirmed by the designer on Nov 14, 2026, and the source of truth for step 3. The architect wrote it after the designer read a first draft and an outside review of it and set the target below; the designer then answered its four questions ("Decided by the designer", at the end). Nothing is built. Each stage still gets its plan and its decision record before any code.

`docs/pillars.md` gives step 3 one line: the layer that turns intent into orders, serving player delegation and the enemy AI, so it is built once. This says what that means for the slice, what it must prove, and in what order it is built.

## What step 3 is for

Today a game is played by giving every unit its orders. A scenario scripts one side, and a player or a model orders the other, unit by unit, each day. That is the thing the pillars set out to remove.

Step 3 puts a staff between the commander and the units. The commander says what he wants and what he will not risk. The staff works out the orders, says why, says what the plan needs and what it may cost, carries it out day by day, and stops the clock when the plan and the world part company.

It has no screen. Step 4 puts a minimal interface over it. Step 3 is judged by tests, by batches and by models playing through the tools.

## The target

**One statement, given on 7 August 1942, plays the first 39 days of Guadalcanal on both sides with nobody ordering a unit.**

1. **The campaign.** The full slice, 7 August to 14 September 1942 (decision 0093), is played to its end, staff against staff, over a thousand seeds. Each side starts from area intents and the commander's limits. No scenario and no person gives a task or a unit order.
2. **Command.** The commander can step in anywhere, hold anything and give it back. The staff neither works round a hold nor takes up a stale job on release.
3. **Reasons.** Every order, and every decision not to act, explains itself from what the side knew.
4. **The estimate.** Before an operation is committed the commander is told what it needs, when it could go and what it may cost, from a trial that never touches the live enemy.
5. **The proof.** A model that commands by intent alone does at least as well as the same model ordering every unit, in a fraction of the decisions. This is the first pillar as a claim that can fail.

**Stretch.** The 180-day soak runs staff against staff with every invariant holding. After step 3, once the battles of October and November 1942 are researched, the goal is the real campaign carried past 14 September.

**Out of reach for this step, and not claimed:**

- Intent in plain language. Intent is a structured object.
- An estimate checked against history. A trial can be checked only against the engine it runs.
- The six months of the campaign from the record. The battles of October and November are not researched.
- A staff that plays well in any absolute sense. It is measured against no orders, against the script and against a model ordering units, and no further.

## Scope

In step 3:

- **Intent and limits** as objects: what a commander asks of an area, a force and a base, and what he will not trade for it.
- **The staff:** deterministic code that reads one side's view, intents and limits and gives the day's orders, each with its reason.
- **The plan** as a lasting object with a baseline, amendments and a daily report of plan against actual.
- **Hold, release and override.**
- **Running time:** a run of several days that stops for a reason.
- **The trial:** the staff's war game of a draft plan.
- **The same staff for the enemy.**
- The tools a model plays through (`sim-agent`) gain all of these.

Not in step 3:

- Any interface beyond the tools and the briefing (step 4).
- Weather, light and the moon (after the kill gate).
- Reading the enemy's intentions from signals. The enemy's plans come to exist in this step; nothing reads them yet.
- The economy, production and allocation between areas (step 5). Allocation inside the area, among the ships, aircraft, fuel and supply the slice has, is in.
- Closing the historical misses of step 2. They are accepted limits of the model (`docs/validation/step2-close.md`) and none is a condition of this step.

## What it builds on

Each line is a capability the step needs, where it comes from and how it stands today. A stage is not briefed until the lines it needs say built.

| Capability | From | Standing on Nov 14, 2026 |
| --- | --- | --- |
| A side's view, and events filtered for a side | Step 1 (decisions 0047 to 0051) | Built |
| Measuring by the engine's own functions | Decision 0049 | Built |
| Objectives counted from state, on one scale | Decisions 0054 and 0088 | Built. The scale's points are placeholders |
| Ship repair and ships out of action | Decisions 0069 and 0088 | Built |
| Land units, carried and fed | Decision 0066 | Built |
| The fight ashore, a base that changes side, `base_held` | Decision 0090 | Built |
| Rebasing an air group | Decision 0091 | Built |
| Shore bombardment, hits on men, the estimate of the enemy ashore | Decision 0092 | Being built |
| Marching overland | Decision 0094 | Hand calculation written |
| The full slice of 39 days with the landings | Decision 0093 | Planned |
| The rules' choices inside a day as functions of belief | Decision 0055, "Left for later" | Not built: stage 0 |
| A rules version on a saved game | This spec | Not built: stage 0 |
| The objectives scale set from the record | `docs/open-work.md` | Open. Not a condition of stages 0 to 2 |

## The seat and the levels

The slice's seat is the area commander, South Pacific, and on the other side his opposite at Rabaul.

| Level | The commander says | The staff does |
| --- | --- | --- |
| Area | Take and hold Guadalcanal; keep it at 30 days of supply; deny the enemy his reinforcement. Risk no carrier for it | Forms the tasks, commits forces to them by date, schedules convoys and cover, says what it cannot do |
| Task | This force covers the landing until the 12th | Routes, timing, readiness, rules of engagement, missions and arcs |
| Unit | This squadron flies search and not strike | Nothing: it is the order as it exists today |

Area intent is in the first stage that has a staff, in a narrow form, and is widened stage by stage. A campaign in which a scenario hands each force its task does not meet the target.

## The command contract

These are the rules between the commander and the staff. They are settled here, before any stage, because every stage leans on them. Each gets its acceptance cases in the stage's decision record.

### Intent and limits

An **intent** is a statement by a commander about one thing he commands. It has a kind, a subject, a measure where it has one and a date where it has one.

A commander also sets **limits**. They are few:

| Limit | What it says | Hard or tradable |
| --- | --- | --- |
| Forces | Which forces, groups and bases the staff may use for an intent | Hard |
| Area | Where they may go | Hard |
| Priority | The order of the commander's intents | Hard: a higher intent is never short so that a lower one is met |
| Protected | A commitment the staff may not break: this force stays on this task | Hard |
| Reserve | Fuel or supply at a base, or rounds in a force, that the staff may not plan to go below | Hard for planning. An event can still take a stock below it |
| Risk | One of three: preserve the force, balanced, or press | Tradable against a date, within its level |

The staff may trade a date against risk and against a lower intent. It may not cross a hard limit to meet a date. When it cannot meet an intent inside its limits it says so, says which limit binds, and goes on with the rest.

### Competing demands

- **A commitment** is a force given to a task from one day to another. A force has one commitment at a time; it may have another after it.
- **Where two intents need the same force,** the higher priority has it. At equal priority the earlier date has it, then the lower number. This is fixed, so the same inputs give the same allocation.
- **The staff reports what was displaced:** the shortfall at the lower intent, in its own measure, and the commitment that caused it.
- **A changed priority is a new allocation** from the next day, and the report says what moved.

### The plan's life

A plan is a draft, proposed, approved, complete, failed or cancelled.

- **A draft** is the staff's. It can be trialled. It gives no orders.
- **Approved,** it has a **baseline:** its objectives, dates, forces and phases as approved. The baseline never changes.
- **An amendment** is a numbered change to an approved plan, with its reason. The report of plan against actual is always against the baseline, with the amendments listed.
- **What the staff may amend alone:** routes, the day of a run inside the slack the plan gives, one like force for another inside the forces allowed, the order of tasks inside a phase.
- **What needs the commander:** an objective, an objective's date, a force outside those allowed, anything that crosses a limit, and cancelling. The staff proposes it and raises an interrupt. With no commander present the staff takes the proposal that keeps the higher priority and records that it did.
- **A phase** has a condition to begin and a condition to end, each read from the side's view. A task is done, failed or expired by a condition of the same kind.
- **An order stands until the reason for it changes.** The staff does not give a replacement order on a day when it would give the same order again, and a changed order names what changed. A convoy is not turned round twice by two small changes.
- **Cancelled with ships at sea,** each force is given a task to come home, and keeps it.

### Hold and release

- **What can be held:** one setting of a unit (its search arc), a unit (a force, an air group, a land unit), or a base. A held force holds its ships and the groups and men aboard them.
- **A hold forbids staff orders to what is held, and staff orders to anything else that would change it:** sending the carrier of a held group to the yard, detaching a held force's ships, loading a held unit's men, rebasing into a held base's place in a plan.
- **A hold does not stop the game's rules.** A held base still pays its daily supply and repairs its aircraft.
- **When the commander takes a unit, its orders stand** until he changes them.
- **What is held is not available.** The staff plans without it and says what it would have done with it.
- **On release the unit is newly available.** The staff plans from where it is and what is wanted now. It goes back to its old task only if the approved plan still has that task and it is still the right force for it.

### Running time

- **A run** is a number of days, with a limit, that resolves day after day and stops early for a reason. The day's orders are listed before a run; listing is not waiting.
- **An interrupt** is a condition the commander sets. It is tested before the first day of a run and between days.
- **It fires on a change, not on a state.** A base below its threshold stops the clock on the day it crosses, and not again until it has been above. Acknowledging an interrupt does not re-arm it; crossing back does.
- **Several reasons on one day stop the clock once,** listed together.
- **A preview changes nothing.** If the commander changes anything after a preview, what he was shown is void and is worked out again before the day runs.
- **With no commander,** the staff still works out what it cannot do and records it. Nothing waits.

### Measures a plan is held to

A plan's measures are not the score. The score is worked out at the end by the umpire. A plan's measures are read each day from the side's own view:

- **Exact, from the side's own state:** stocks, days of supply, a base held, a force on station, a force's fuel and rounds.
- **Believed, and said to be believed:** enemy ships sunk, enemy men ashore, an enemy base closed. The staff never reads the umpire's tally to decide that an objective is met.

"Days of supply" is defined once, in stage 1, with hand-worked cases for a unit arriving, a surge of sorties, no consumption at all, and a base taken and lost. It is stock over planned use, and a staff may not meet it by standing its aircraft down.

### Reasons

Every staff decision has a record, whether it gave an order or withheld one:

| Part | What it holds |
| --- | --- |
| Serves | The intent or task, and the plan and amendment |
| Evidence | What the side held that mattered, and the day of each piece |
| Binding | The limit, the stock, the distance or the commitment that shaped the decision |
| Worked | The calculation, or the trial and its assumptions |
| Would change it | What, if different, would give a different decision |

It names the side's own ships freely. It names the enemy only as contacts, claims and estimates. A refused intent, a strike held back, a plan that cannot be met and a convoy displaced each have one.

### What is saved

| Kind | What | Kept how |
| --- | --- | --- |
| Authoritative | Intents, limits, holds, approved plans with their amendments, commitments, interrupts set and acknowledged | State: hashed, and saved as the commander's orders are saved, so a replay rebuilds it |
| Derived | The staff's proposals and the day's staff orders | Worked out again from the state. The staff orders given are in the log as orders, with the staff as origin |
| Evidence | An estimate the commander was shown | Kept with the session, outside the state. A trial changes no state |

A saved game also names the version of the rules, the doctrine and the figures it was played under. A reason still resolves after its intent is replaced or its task is done.

## The staff's decisions

The staff is ordinary code, deterministic, with no random draw outside a trial. Every decision is a function of the side's view, the intents and limits in force, and the figures.

It is built as **doctrine and trial**. Doctrine is a small set of written ways of doing each kind of task. It is complete by itself: a staff with no trial still plays a side, and that is the staff of stages 1 to 3 and the staff inside a trial. The trial costs a few candidate courses for an operation, when a commander or the staff asks. It is not called every day.

What it must be able to decide in the slice, and the orders each needs:

| Task | What the staff decides | Orders it needs |
| --- | --- | --- |
| Keep a base supplied | Which ships, the load, the route, the days, the escort, from stocks, use and passage times | Exist |
| Cover a place or a force | Where to stand, readiness, search arcs, which groups strike and which patrol, when to refuel | Exist |
| Strike what comes within reach | Target priority, reach, when a contact is worth a strike | Exist. A strike of a stated size: decided in stage 2 |
| Intercept a force expected at a place | Where and when to be | Exist |
| Bombard a base | The force, arrival by night, the withdrawal | Decision 0092 |
| Land troops, take and hold a place | The shipping, the escort, the date, the march, the posture, the follow-up supply | Decisions 0066, 0090, 0094 |
| Pull out of a place | What is flown out and taken off, and when | Decisions 0091 and 0066 |
| Rest, repair and replace | Which groups stand down, which ships go to a repair base or the yard | Exist |
| Use shipping from a pool | Which ships form a convoy | Forming and merging forces by order: required, decided in stage 1 |

A force that retires when it has done what it came for is a staff decision and needs no new order.

## The trial

A trial is the staff's war game of a draft plan (`docs/design/planning.md`). It is stage 4, and its own decision record settles the contracts below before it is built. They are stated here so that nothing earlier closes them off.

- **It plays a private world, built from what the side may know.** A contact is a place, a count and a kind. It has no fuel, damage, aircraft or orders. A constructor builds a whole world from the side's view, public data, what the scenario says the side believed, and stated assumptions for the rest. It is separate from the decision code. The live world is never copied.
- **The constructor is tested on its hard cases:** no contact; an old contact; two reports of one force; a claim of a sinking that is too high; a contact of the wrong kind; an enemy carrier never seen. Each gives a valid world.
- **No peek.** The same visible inputs and assumptions give the same trial whatever the live enemy holds. Its draws come from a stream of its own, keyed on permitted inputs, and never from the live game's.
- **No trial inside a trial.** Both sides inside a trial play by doctrine alone.
- **A fixed budget of work,** in games and days, not a time on a clock. The course chosen never depends on how fast the machine was. The budget is set from a measurement of the whole workload, staff and all.
- **Its assumptions are in the estimate.** What was assumed about the unseen is shown with the figures.
- **It is checked three ways, apart:** that its sampling is right, in small cases worked by hand; that over independent runs its spreads hold the outcome as often as it says and are narrow enough to be of use, by a rule with numbers written before the run; and that controlled cases of a wrong picture move the estimate as expected. Runs that never reach an objective are reported, not dropped.

How the unseen is filled in is the hardest problem of the step. The first draft's recommendation stands as the starting point: what the side holds, plus forces it knew of and has lost, placed by how far they could have gone.

## The enemy

The same staff plays the Japanese side from Japanese intents and limits, which a scenario states. It reads the Japanese view. It is not told the Allied plan and reads nothing of the truth.

The scripted scenarios of steps 1 and 2 stay as they are. They are the validation fixtures.

## What a side is told

Everything the staff knows it knows from its side's view. The tests that hold it to that cover more than orders: feasibility, requirements, the order of courses, interrupts, refusals, the numbers a side is shown and replayed reasons are each the same in two worlds that differ only in what the side cannot see. A trial's worlds are shown as assumed worlds, numbered apart, so that nothing in a trial can be taken for a report or change a belief.

## The build, in stages

Each stage is a plan for the designer's approval, then a decision record with its acceptance cases and numbers, before any code.

| Stage | What is built | What it shows |
| --- | --- | --- |
| 0 | The rules' choices inside a day as functions of belief. The rules version on a saved game. A measurement of a day's cost with a staff-sized workload | The ground the staff stands on |
| 1 | The command contract, whole: intent and limits, commitments, the plan's life, hold and release, runs and interrupts, reasons, saving. A staff for one area intent: keep a base supplied from a pool of shipping | The loop end to end: propose, explain, approve, run a quiet week, a shortage that stops the clock, a hold and its release, a save in the middle |
| 2 | Doctrine for the fighting tasks, on both sides. The enemy played by the staff | A battle of the slice, and then a week of it, with no unit order from anyone |
| 3 | Taking and holding a place: landing, marching, bombardment, follow-up supply, pulling out. Area intent in full | The 39 days, staff against staff, from area intents alone |
| 4 | The trial: the constructor, bounded trials, requirements, a window, two or three courses, the estimate's checks | The estimate a commander decides on |
| 5 | The proof: a model commanding by intent against the same model ordering units. The interface step 4 needs: propose, explain, commit, run until stopped, revise | The first pillar, measured |

The kill gate after step 4 asks whether commanding by intent is fun and whether the staff is trusted. Stages 1 to 3 are what that needs first.

## How it is tested

The hand calculation still comes first wherever a stage has arithmetic: a convoy schedule, an allocation between two bases, days of supply. Choice is tested in other ways, each written into the stage's record before any code, each with a number to pass.

- **Pure decisions.** Each staff decision can be called alone, on a view, intents and limits chosen by the architect, with the orders that must come out.
- **Properties.** The staff never orders what it does not own or what is held, directly or through another unit. It never plans below a reserve or across a hard limit. Every staff order and every withheld one has a reason that resolves. The same inputs give the same orders. A staff with no intent gives no order.
- **The contract's cases.** Two bases and one convoy. A hold, a changed intent, a week, a release. A sunk tanker, a missed date, an objective that cannot be met, an operation cancelled with ships at sea. A save in the middle of an amended operation with a held unit and an acknowledged interrupt, resumed, matching play that was not interrupted.
- **Running time.** A quiet week in one call. A condition already true at the start. A shortage that lasts. Two reasons on one day. An intent changed after a preview.
- **No truth,** as above, for every stage.
- **Against no orders.** With the same start, the same seeds and the same opponent, the staff does better than a side given no orders, by a margin and on measures fixed in the stage's record: the objectives scale, and beside it supply delivered, days below reserve, bases held, own losses and ships out of fuel. A thousand quiet draws do not pass.
- **Against the script.** The staff's orders for the tuned battles are set beside the scripted ones and the difference is reported. It is not a test to pass: the record's orders are not the best play under these rules.
- **Situations it was not written for.** Starts and upsets that no doctrine was written around.
- **The campaign.** The 39 days, staff against staff, a thousand seeds, every invariant holding, no order refused that the staff should have known would be, and both sides doing something: the measures above, each inside bounds set in stage 3's record.
- **Legibility.** A cheap model, given a decision's record, says what the staff is trying to do and what would change its mind, at a pass mark written before the run. It does not score predictions that depend on where a hidden enemy is.
- **The proof.** The same model plays the same side of the campaign both ways over the same seeds: by intent only, and by unit orders. By intent it scores no worse on the objectives scale and gives fewer than a set share of the decisions. The numbers of games, the margin and the share are fixed in stage 5's record before a game is played.

Whether it is fun is judged by people at step 4, as the pillars say.

## Done means

- The campaign of the target runs as the campaign test says.
- The contract's cases all pass, and the save case among them.
- Every staff decision answers "why", acted on or not.
- The leak tests pass for orders, reasons, feasibility, interrupts, numbers shown and trials.
- A trial of the campaign's main operation returns within its budget, is the same twice, changes nothing, and passes its three checks.
- The staff beats no orders by the margin set, on each battle of the slice and on the campaign.
- The proof is run, and its result is reported whichever way it falls. If the model by intent does worse, the step is not done, and what it lacked is the next work.
- The rulebook has a chapter on the staff: every decision it makes, as the code has it.

## Decided by the designer

On Nov 14, 2026.

1. **The limits are the six of this spec:** forces, area, priority, protected commitments, reserves, and a risk level of three steps. More are added when play shows a need.
2. **The staff changes alone what this spec lists:** routes, the day of a run inside its slack, a like force for another, and the order of tasks in a phase. Everything else on an approved plan comes back to the commander.
3. **The proof's bar is no worse, in far fewer decisions.** The claim is that delegating costs nothing.
4. **The stretch is the 180-day soak first.** The real campaign carried past 14 September is the goal after it, once its battles are researched.
