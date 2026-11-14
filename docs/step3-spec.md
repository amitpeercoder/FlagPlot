# Flag Plot — Step 3 Spec: Staff Planner

Status: confirmed by the designer on Nov 14, 2026, and the source of truth for step 3. The architect wrote it after the designer read a first draft and an outside review of it and set the target below; the designer then answered its four questions ("Decided by the designer", at the end). Nothing is built. Each stage still gets its plan and its decision record before any code.

Amended by the architect on Nov 14, 2026, with the designer's agreement, after a second outside review. The designer's four decisions are unchanged. Amended again the same day, after the designer set out the play the step must support ("The play it must support", below): the estimate now offers an operation at more than one date, and a rule for preparing a landing is named as a condition of stage 3 (decision 0095).

Amended a third time the same day, after a third outside review of that play: a plan can be preparing before it is released; what is made ready has named owners; a what-if carries the side's other plans; more reports come from collection and not from waiting; an amendment is shown with its cost before it is applied; and one built operation is added to what done means. Preparing a landing was revised (decision 0095) and the quality of a land unit added (decision 0097).

Amended a fourth time the same day, after a final outside review, which found the design ready to build and asked for four things: one activity a day for a unit across rehearsal, training and work (decision 0099, 3b); the built operation to exercise training, skills and working units; leave to prepare and leave to launch as two permissions; and a look back kept apart from a forecast. The designer has said there will be no further review of this spec. **He declared it ready on Nov 14, 2026.** From here it changes only by a decision record that says what it changes and why.

What changed in the first amendment: a sixth point of the target (the choices), orders that can honour a reserve, what an unattended staff may do, a separate setup for staff play, what the risk levels change, how the proof is kept fair, and a list of requirements for the stage briefs.

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
4. **The estimate.** Before an operation is committed the commander is told what it needs, when it could go and what it may cost, from a trial that never touches the live enemy. It is offered at two or three dates, each with what waiting buys and what the enemy is believed to gain by it.
5. **The proof.** A model that commands by intent alone does at least as well as the same model ordering every unit, in a fraction of the decisions. This is the first pillar as a claim that can fail.

6. **The choices.** In a small set of constructed situations the staff lays out two real courses, what each gives up, what is not known and by when the commander must choose; and a change of priority or risk changes what it recommends, for a reason it can state. Running alone shows the staff is competent. This shows the commander still has something to decide.

**Stretch.** The 180-day soak runs staff against staff with every invariant holding. After step 3, once the battles of October and November 1942 are researched, the goal is the real campaign carried past 14 September.

**Out of reach for this step, and not claimed:**

- Intent in plain language. Intent is a structured object.
- An estimate checked against history. A trial can be checked only against the engine it runs.
- The six months of the campaign from the record. The battles of October and November are not researched.
- A staff that plays well in any absolute sense. It is measured against no orders, against the script and against a model ordering units, and no further.

## The play it must support

Set out by the designer on Nov 14, 2026. Each line is checked against this spec.

| The designer asked for | Where it is met |
| --- | --- |
| The player plans a large invasion and the control of an island | Stage 3: taking and holding a place. What "control" of an island means day by day is defined there: the named places held, and a garrison supplied |
| The longer ahead he plans, the better his chances: troops and resources ready, and more known of the enemy, as claims | What time buys is real things: stocks, repaired ships, rested groups, forces come available, more reports. The estimate shows it by offering the operation at more than one date. The ships are loaded for the landing and the units rehearse it, by the day, as real activity (decision 0095). Training raises a unit's lasting experience (decision 0097); real units have no values for it until they are researched. Reports come from collection that is tasked, not from the passing of days |
| He plans, and the staff answers "what if". Other plans are in priority, since they draw on the same troops, shipping and stocks | The trial and its courses; priority, commitments and the report of what was displaced |
| The plan is carried out | An approved plan, its baseline and the daily report against it |
| It can be amended at any time, at a cost to the readiness built up | At the start of any day, and a run can be stopped there. Each thing made ready has an owner, and a change costs what that owner loses ("What is made ready"). The cost is shown before the change is applied |
| He feels like a general or an admiral, with their trade-offs and their strain | The choices, the command report and the interrupts give the decisions. Whether it feels so is judged by a person at step 4. This step has one seat, the area commander, who may take any force himself by holding it |

## Scope

In step 3:

- **Intent and limits** as objects: what a commander asks of an area, a force and a base, and what he will not trade for it.
- **The staff:** deterministic code that reads one side's view, intents and limits and gives the day's orders, each with its reason.
- **The plan** as a lasting object with a baseline, amendments and a daily report of plan against actual.
- **The command report:** a short account of what changed against the plan, what needs a decision, what was won or lost for good, and what is still open (`docs/design/storytelling.md` gives threads and plan-against-actual to this step).
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
- A choice of seats. The seat is the area commander.
- Values of experience for real land units, which wait on research.
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
| Shore bombardment, hits on men, the estimate of the enemy ashore | Decision 0092 | Built |
| Marching overland | Decision 0094 | Built |
| The full slice of 39 days with the landings | Decision 0093 | Planned |
| The rules' choices inside a day as functions of belief | Decision 0100 (stage 0) | Built |
| A rules version on a saved game | Decision 0100 (stage 0) | Built |
| A force that acts at a place and steams on by itself | Decision 0096 | Built |
| A load of a stated weight, and refuelling that leaves a reserve | Stage 1 | Not built |
| A day from which a force may be used | Stage 2 | Not built |
| Preparing a landing: loading and rehearsal | Decision 0095 | Built |
| The experience of a land unit, and training | Decision 0097 | Built |
| What a land unit is good at: its skills | Decision 0098 | Built |
| Units that work, and an airfield and defences that are built | Decision 0099 | Built |
| A report of enemy men ashore without a landing | Not planned | Missing. Today only a side with men in the hex gets one (decision 0092). Stage 3 says whether the invasion needs it |
| The objectives scale set from the record | `docs/open-work.md` | Open. Not a condition of stages 0 to 2. It is frozen before stage 5's games are played |

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

The staff may trade a date against risk and against a lower intent. It may not cross a hard limit to meet a date, and nothing but the commander changing a limit changes it: not a higher priority, not an empty seat. When it cannot meet an intent inside its limits it says so, says which limit binds, and goes on with the rest.

**A limit the orders cannot honour is not a limit.** Today a load order fills a ship from the stock and names no weight, and a force in port refuels before the day's orders are taken. Stage 1 gives the staff the orders it needs to leave a reserve where it lies: a load of a stated weight, and refuelling that stops at a base's reserve. A draw that the staff's own plan causes is never an event that excuses going below a reserve. Its acceptance case: two tasks load from one base without both counting the same stock, a convoy with room to spare leaves the reserve behind, and a force refuelling there that morning is in the sum.

### What the risk levels change

Risk is judged from belief. It is not a promise against what the side has not seen. Forbidding the use of a carrier is the forces limit, and is hard; risk is about how a force the staff may use is exposed.

| The staff's choice | Preserve the force | Balanced | Press |
| --- | --- | --- | --- |
| Committing scarce ships to a task | Only where the task cannot be done without them | Where they add to it | Whenever they are free |
| The evidence it strikes on | A fresh contact of a kind worth the strike | A contact within the age the doctrine sets | An older contact, or a place the enemy is expected |
| How far it reaches | Inside the radius, with a margin | To the radius | Beyond it, at the cost the rules give |
| Fuel and rounds kept to get home | A margin above the passage home | The passage home | The passage home, cut fine |
| An operation that has lost ships | Stops and comes home | Goes on if the objective can still be met | Goes on while any objective can be met |

The numbers behind each cell are fixed in stage 2's record. Its acceptance cases: on the same information the three levels give different orders in cases chosen to show each row; a new report changes the order with the level unchanged; and a choice that went badly still has a reason that stands.

### Competing demands

- **A commitment** is a force given to a task from one day to another. A force has one commitment at a time; it may have another after it.
- **Where two intents need the same force,** the higher priority has it, among the forces not held to another task by a protected commitment. At equal priority the earlier date has it, then the lower number. This is fixed, so the same inputs give the same allocation.
- **The staff reports what was displaced:** the shortfall at the lower intent, in its own measure, and the commitment that caused it.
- **A changed priority is a new allocation** from the next day, and the report says what moved.
- **A commitment counts what follows it:** the passage to the next task, loading and refuelling. A force is free when it can be where the next task starts, not when the last one ends. Stock at a base is committed by weight and by day as forces are. The tie-break is applied among the tasks a force can in fact do.

### The plan's life

A plan is a draft, preparing, released, complete, failed or cancelled. "Approved" below means preparing or released.

- **A draft** is the staff's. It can be trialled. It gives no orders, commits nothing and earns nothing. Five drafts prepare nothing.
- **Preparing,** the commander has said: make ready to do this by about then, and bring me the choice. The staff commits the forces, shipping and stock the preparation needs, and gives real tasks: assemble, load, rehearse, train, build up stock, search. It says what it is doing now, what is still provisional, and what will need his word. The launch is not yet ordered.
- **Released,** the commander has ordered it to go. From a plan that is preparing he may release it, put it back, change it or cancel it, and what was made ready is kept or lost by its owner's rule.
- **Approved,** it has a **baseline:** its objectives, dates, forces and phases as approved. The baseline never changes.
- **An amendment** is a numbered change to an approved plan, with its reason. It takes effect at the start of a day, and a commander can stop a run at the next day's start to make one; nothing is ordered inside a day. Before it is applied the staff shows what it costs: what must turn round, unload or wait, what is lost of what was made ready and by whom, the new earliest date, what other plans give up, and the estimate as it now stands. What is shown is tied to the plan's revision and the state it was worked from, and is void if either changes. What is applied is what was shown. A change turned down has no effect. The report of plan against actual is always against the baseline, with the amendments listed.
- **What the staff may amend alone:** routes, the day of a run inside the slack the plan gives, one like force for another inside the forces allowed, the order of tasks inside a phase.
- **What needs the commander:** an objective, an objective's date, and cancelling. The staff proposes it and raises an interrupt. A proposal never crosses a hard limit. Where only a wider limit would meet the intent, the staff says which limit and by how much, and that is all it may do.
- **While a proposal waits,** the forces in it go on with the approved plan as far as it can still be carried out inside the limits. A force that cannot go on takes the plan's fallback, which every plan states: hold where it is, or come home.
- **With no commander present,** on either side, the staff chooses among what it is allowed: it slips the date, then cuts the objective to what can be met, then stands the task down, taking the first that is feasible, and records which. It never widens a limit. The same rule holds for a player who runs time and does not answer.
- **Leave to prepare and leave to launch are two permissions.** The commander says, each apart, whether a draft inside his limits may be made ready without his word, and whether a plan that is ready may be released without it. The default for a player is that preparing is allowed and launching waits for him. With no commander, as for the enemy, both are allowed, and this is what lets a campaign begin from a statement. A player who runs time and does not answer a request to launch has not given leave: the plan stays preparing, and the rules for a waiting proposal hold.
- **A phase** has a condition to begin and a condition to end, each read from the side's view. A task is done, failed or expired by a condition of the same kind.
- **An order stands until the reason for it changes.** The staff does not give a replacement order on a day when it would give the same order again, and a changed order names what changed. A convoy is not turned round twice by two small changes.
- **Cancelled with ships at sea,** each force is given a task to come home, and keeps it.

### What is made ready

Readiness is not one number. It is a few facts, each with an owner, and a change costs what its owner loses.

| What | Belongs to | Earned by | Kept through | Lost or spent by |
| --- | --- | --- | --- | --- |
| Loading | A task force, for a landing place | Days in port with the order (decision 0095) | A change of date; a unit changing to this force keeps its own | A new place: all. A ship or a unit added: half. The landing spends it |
| Rehearsal | A land unit, for a landing place | Days with its force in port (decision 0095) | A change of ships, of force or of date | A new place: all. The landing spends it |
| Experience | A land unit | Training and fighting (decision 0097) | Everything | Nothing |
| Stock in place | A base | Convoys | A change of plan, where it lies | Use, raids, and the days it takes to move it elsewhere |
| Forces on hand | Each force | Repair, rest, arrival | A change of plan | A commitment to another plan |
| The picture of the enemy | The side's belief | Search and reports that were tasked and got through | A change of plan, for the places it covers | Age. A report grows stale; waiting alone earns none |

An amendment that changes nothing a row depends on costs nothing: a new name, a new order of reports, a later date for a force already ready. The staff may not make a substitution inside its own reach if it costs loading or rehearsal: that comes to the commander.

### Hold and release

- **What can be held:** one setting of a unit (its search arc), a unit (a force, an air group, a land unit), or a base. A held force holds its ships and the groups and men aboard them. A held setting holds that setting and what it depends on to mean anything (a held search arc holds the group to the search mission), and leaves the rest of the unit to the staff.
- **A hold forbids staff orders to what is held, and staff orders to anything else that would change it:** sending the carrier of a held group to the yard, detaching a held force's ships, loading a held unit's men, rebasing into a held base's place in a plan.
- **A hold does not stop the game's rules.** A held base still pays its daily supply and repairs its aircraft.
- **When the commander takes a unit, its orders stand** until he changes them.
- **What is held is not available.** The staff plans without it and says what it would have done with it.
- **On release the unit is newly available.** The staff plans from where it is and what is wanted now. It goes back to its old task only if the approved plan still has that task and it is still the right force for it.

### Running time

- **A run** is a number of days, with a limit, that resolves day after day and stops early for a reason. The day's orders are listed before a run; listing is not waiting.
- **An interrupt** is a condition the commander sets. It is tested before the first day of a run and between days. A condition that is already true when it is set, or at the start of a run, fires once and then waits to be crossed again.
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
| Land troops, take and hold a place | Which units, by what each is good at (decision 0098); the shipping, the escort, the date, the march, the posture, the follow-up supply | Decisions 0066, 0090, 0094 |
| Make units ready | What each unit does each day: rehearse, train, work or stand ready; when a raw unit trains and when it starts to rehearse | Decisions 0095, 0097, 0099 |
| Build up a base | Which units work at it and at what: the airfield, the port, the aircraft or the defences; when each job will be of use | Decision 0099 |
| Pull out of a place | What is flown out and taken off, and when | Decisions 0091 and 0066 |
| Rest, repair and replace | Which groups stand down, which ships go to a repair base or the yard | Exist |
| Use shipping from a pool | Which ships form a convoy | Forming and merging forces by order: required, decided in stage 1 |

A force that retires when it has done what it came for cannot be left to the staff's next day. Orders are taken once a day, so a force that arrives by night to bombard or to unload would lie there through the daylight that follows. It needs a standing instruction that the force carries out by itself inside the day: act at this place, then steam on. That is a rule of its own, decided before stage 2, and the daily cycle of command stays as it is.

## The trial

A trial is the staff's war game of a draft plan (`docs/design/planning.md`). It is stage 4 of step 3, and its own decision record settles the contracts below before it is built. They are stated here so that nothing earlier closes them off.

- **It plays a private world, built from what the side may know.** A contact is a place, a count and a kind. It has no fuel, damage, aircraft or orders. A constructor builds a whole world from the side's view, public data, what the scenario says the side believed, and stated assumptions for the rest. It is separate from the decision code. The live world is never copied.
- **The constructor is tested on its hard cases:** no contact; an old contact; two reports of one force; a claim of a sinking that is too high; a contact of the wrong kind; an enemy carrier never seen. Each gives a valid world.
- **No peek.** The same visible inputs and assumptions give the same trial whatever the live enemy holds. Its draws come from a stream of its own, keyed on permitted inputs, and never from the live game's.
- **No trial inside a trial.** Both sides inside a trial play by doctrine alone.
- **A fixed budget of work,** in games and days, not a time on a clock. The course chosen never depends on how fast the machine was. The budget is set from a measurement of the whole workload, staff and all.
- **Its assumptions are in the estimate.** What was assumed about the unseen is shown with the figures.
- **It carries the side's other plans.** A what-if is worked from the side's commitments, what has been made ready and its other approved plans, in a private copy. It reports the operation's spread and what each plan it displaces loses. It can vary the target, the troops, the shipping, a reserve and a priority as well as the date. Trying one changes no commitment, no readiness and no stock; approving one applies that one. Where a limit binds, the staff offers the comparison itself: keep the convoy where it is, or borrow it and take the shortage.
- **Looking back is kept apart from looking forward.** There are three things. A forecast from now, which is what a commander decides on. A controlled comparison, used to prove that preparation matters: the same declared start, the same landing date, and only the day preparation began changed. And a look back at a live game, "what if we had started sooner", which is offered only as a stated sensitivity: it is worked from what the side held on the earlier day, it never treats a later report as known then, it earns no readiness and gives back no time, and it says so. Two worlds that differ only in hidden history give the same look back.
- **A report it foresees is not a report.** A trial may say that waiting could show more. It never shows a drawn enemy as known, and reading an estimate changes no belief.
- **It answers "what if we wait".** For an operation the staff trials the same course at two or three starting dates. The days between are played, with preparation, use of supply, reports, other commitments and the enemy's moves. Each shows what the later date buys (stock, forces come available, repairs done, loading and rehearsal, training, the reports that tasked search may bring) and what the enemy is believed to do with the same days. The dates are chosen from when those things change, not at even steps.
- **It is checked three ways, apart:** that its sampling is right, in small cases worked by hand; that over independent runs its spreads hold the outcome as often as it says and are narrow enough to be of use, by a rule with numbers written before the run; and that controlled cases of a wrong picture move the estimate as expected. Runs that never reach an objective are reported, not dropped.

How the unseen is filled in is the hardest problem of the step. The first draft's recommendation stands as the starting point: what the side holds, plus forces it knew of and has lost, placed by how far they could have gone.

## The enemy

The same staff plays the Japanese side from Japanese intents and limits, which a scenario states. It reads the Japanese view. It is not told the Allied plan and reads nothing of the truth.

The scripted scenarios of steps 1 and 2 stay as they are. They are the validation fixtures.

## Two setups of the campaign

The 39-day slice is built first with scripted orders, each force sent out on the day the record gives (decision 0093). Every force is in it from day 1, because nothing brings a force in later. Taking the scripts away would hand a staff forces weeks before they were there to use.

- **The scripted setup** is kept as it is, for the measures against the record.
- **The staff setup** has the same forces, bases and map and no scripted order. Each force has a day from which its side may use it, from a source where one gives it, and declared as made up where none does. It is a dated entry in the forces limit.
- **A side knows its own forces' days and not the enemy's.** A staff cannot plan on an enemy arrival it has not been told of.
- **After its day a force is the staff's to use as it sees fit.** No date of any operation is fixed by the setup.

## What a side is told

Everything the staff knows it knows from its side's view. The tests that hold it to that cover more than orders: feasibility, requirements, the order of courses, interrupts, refusals, the numbers a side is shown and replayed reasons are each the same in two worlds that differ only in what the side cannot see. A trial's worlds are shown as assumed worlds, numbered apart, so that nothing in a trial can be taken for a report or change a belief.

## The build, in stages

Each stage is a plan for the designer's approval, then a decision record with its acceptance cases and numbers, before any code.

| Stage | What is built | What it shows |
| --- | --- | --- |
| 0 | The rules' choices inside a day as functions of belief. The rules version on a saved game. A measurement of a day's cost with a staff-sized workload | The ground the staff stands on |
| 1 | The command contract, whole: intent and limits, commitments, the plan's life, hold and release, runs and interrupts, reasons, saving. A staff for one area intent: keep a base supplied from a pool of shipping | The loop end to end: propose, explain, approve, run a quiet week, a shortage that stops the clock, a hold and its release, a save in the middle |
| 2 | Doctrine for the fighting tasks, on both sides. The enemy played by the staff | A battle of the slice, and then a week of it, with no unit order from anyone |
| 3 | Taking and holding a place: preparing, landing, marching, bombardment, follow-up supply, pulling out. Area intent in full | The 39 days, staff against staff, from area intents alone |
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
- **The choices.** A few situations built for the purpose, through the tools: commit scarce cover now or keep it; sail on what is known or wait while supply runs down; go on after losing an escort or change the objective. For each the staff gives the feasible courses, what each gives up, what is unknown and the day by which to choose. Where one course is plainly best it says so. A change of priority or of risk changes the recommendation and the reason. Accepting a recommendation takes one answer.
- **The operation.** One built invasion, on placeholder forces, through the intent tools alone:
  1. Ask for an island by a date, with what control means stated.
  2. Authorise its preparation, with leave to launch withheld. The staff assigns troops, shipping, cover, build-up and search. Among the troops are a raw unit below the standard, two units that differ in a skill that matters, and a working unit; the base the operation needs lacks something that work must build. The staff chooses which unit does what and says why: it trains the raw unit and then rehearses it, picks the fitter unit for the assault, and sets the working unit to the job the operation waits on. No one gives a `set_activity` order by hand.
  3. A second plan needs some of the same shipping and stock. The staff shows what each order of priority costs the other.
  4. Compare the same date prepared early and prepared late; then going sooner against waiting.
  5. A quiet interval is run. The staff comes back with the plan ready and asks leave to launch; nothing has sailed. Release it and run it through embarking, the passage, the landing, the fight and the supply that follows, reported against the plan.
  6. Before the landing, compare leaving it be, a new date, new troops or ships, and a new beach; apply one. A report or a loss then forces a choice between going on, waiting and asking for less.
  7. Go on past the capture, amend it once more, and save and resume twice along the way with nothing changed by it.
  The run must show a real gain from training, a choice between units by skill, and work that makes something possible that was not; when one of those units is taken by the other plan, the staff says what the second choice costs. The same rules act inside the trials. Every made-up value is marked a placeholder.
  Earlier preparation that is used well must show in what is ready and, in this built case, in the operation's prospects. It is not required that every added day helps in a live campaign: the enemy has the same days.
- **The proof.** The same model plays the same side of the campaign both ways: by intent only, and by unit orders. By intent it scores no worse on the objectives scale, by a margin fixed beforehand, and gives fewer than a set share of the decisions. Fixed in stage 5's record before a game is played: the model and its version, separate sessions, the prompts, what each is shown, the tools each has, the opponent, the seeds, the budget, and what is done with a game not finished. A decision is one intent, limit, hold, override or approval, or one unit order, however the tools batch them. Commands given, decisions that took judgement and calls made are reported apart. The games of the test are not the situations the doctrine was improved on, and the operational measures are reported beside the score. A failure is put down to its cause: what the staff could not do, what intent could not say, what a reason did not explain, or how the model used the tools.
- **Speed.** On a named machine, with the workloads named in stage 0: a day's staff decisions for both sides, a preview, a quiet week, a trial at each quality offered, each late in the campaign with its history behind it, inside times the designer accepts. The time is measured outside the game's logic and never changes a result. Memory and storage for reasons and trial records are measured with it.

Whether it is fun is judged by people at step 4, as the pillars say.

## Done means

- The campaign of the target runs as the campaign test says.
- The contract's cases all pass, and the save case among them.
- The operation above runs from end to end with no unit order, and its comparisons come out as stated.
- The choices are shown: each built situation gives courses that differ, and the recommendation moves with priority and risk.
- The command report gives a quiet week in a few lines and a heavy loss with what it means for the operation, each item leading to its reasons.
- Every staff decision answers "why", acted on or not.
- The leak tests pass for orders, reasons, feasibility, interrupts, numbers shown and trials.
- A trial of the campaign's main operation returns within its budget, is the same twice, changes nothing, and passes its three checks.
- The staff beats no orders by the margin set, on each battle of the slice and on the campaign.
- The proof is run, and its result is reported whichever way it falls. If the model by intent does worse, the step is not done, and what it lacked is the next work.
- The rulebook has a chapter on the staff: every decision it makes, as the code has it.

## For the stage briefs

Requirements each stage's record takes up, from the second review. They are not open questions.

- **Stage 0.** The measurement names its machine and workloads, as "Speed" above. A small experiment builds a trial's private world from a few of the slice's side views, to find what the constructor lacks before stage 4 depends on it.
- **Stage 1 is built in four steps, each with something to run:** one feasible convoy plan; two supply intents that compete; a hold and its release; then a shortage that stops a run, and a save resumed exactly. The commitment cases include the passage to the next task, loading, refuelling and a second operation on the same base.
- **Stage 1, holds:** a held setting, a held unit and a held parent force are tested apart. Guarding a hold must not freeze the whole chain of command above it.
- **Stage 1, the command report:** open matters keep one identity from day to day, so a delayed convoy is one concern for a week and not seven.
- **Stage 2.** The numbers of the risk table. The day from which a force may be used.
- **Stage 4, the unseen.** A world with no unseen carrier is valid and proves nothing. The constructor's tests state what the worlds must hold across a trial: where the scenario says a side believes a force exists and has not seen it, some worlds have it. "Not seen" and "believed absent" are different inputs. Where nothing supports a probability the estimate says so and shows the cases apart. What the enemy is assumed to want and to believe inside a trial is built from the side's permitted inputs and shown as an assumption.
- **Stage 1, permissions.** A player delegates preparing, runs a quiet interval and is brought a plan ready to launch that has not sailed. With leave to launch given, the same operation runs through unattended, inside the same limits.
- **Stage 3, activities.** One raw unit is given to an invasion: the staff trains it, then rehearses it; or puts it to work, at the cost stated. No day earns two things. Naming it on a second roster earns nothing more. A save and a resume keep the same counts.
- **Stage 3, collection.** A plan that is preparing names what it most needs to know, what the side has that could find it out, what that costs and what cannot be known. Search is tasked for it. Whether an invasion needs a report of the enemy's men before it lands, which nothing gives today, is settled in this stage's record.
- **Stage 4, dates.** One of the built situations of "The choices" is the date of a landing: go now, less prepared and against fewer men, or later.
- **Stage 4, quality.** A trial is offered at stated sizes, each a fixed amount of work with what is lost in precision at the smaller. A trial can be abandoned with no effect on the game.
- **Stage 5.** The objectives scale is frozen before its games. A better score never hides a supply line lost or a carrier spent for nothing: the operational measures are judged with it.

## Decided by the designer

On Nov 14, 2026.

1. **The limits are the six of this spec:** forces, area, priority, protected commitments, reserves, and a risk level of three steps. More are added when play shows a need.
2. **The staff changes alone what this spec lists:** routes, the day of a run inside its slack, a like force for another, and the order of tasks in a phase. Everything else on an approved plan comes back to the commander.
3. **The proof's bar is no worse, in far fewer decisions.** The claim is that delegating costs nothing.
4. **The stretch is the 180-day soak first.** The real campaign carried past 14 September is the goal after it, once its battles are researched.
