# Flag Plot — Step 3 Spec: Staff Planner

Status: a first draft, written by the architect on Nov 13, 2026, for the designer's review. Nothing here is decided and nothing is built. The questions it leaves open are at the end, each with a recommendation. When the designer has answered them it becomes the source of truth for step 3.

`docs/pillars.md` gives step 3 one line: the layer that turns intent into orders, serving player delegation and the enemy AI, so it is built once. This says what that means for the slice as it stands at the close of step 2, and in what order it is built.

## What step 3 is for

Today a game is played by giving every unit its orders. A scenario scripts one side, and a player or a model orders the other, unit by unit, each day. That is the thing the pillars set out to remove.

Step 3 puts a staff between the commander and the units. The commander says what he wants. The staff works out the orders, says why it chose them, says what the plan needs and what it is likely to cost, carries it out day by day, and stops the clock when the plan and the world part company.

It has no screen. Step 4 puts a minimal interface over it. Step 3 is judged by tests, by batches and by models playing through the tools, as steps 1 and 2 were.

## Scope

In step 3:

- **Intent** as an object: what a commander asks of a task force, of an area, and of a base.
- **The staff:** deterministic code that reads one side's view and intent and gives the day's orders, each with its reason.
- **The plan** as a lasting object: objectives with a measure and a date, phases, and a daily report of plan against actual.
- **Hold, release and override:** the commander may take anything from the staff and give it back.
- **Interrupts:** what stops the clock between days.
- **The trial:** the staff's war game of a draft plan, played forward from belief with the real rules, giving what the plan needs, when it could go and what it is likely to cost.
- **The same staff for the enemy:** the Japanese side played by the planner, in place of a script.
- The tools a model plays through (`sim-agent`) gain intent, plans and the staff's reasons.

Not in step 3:

- Any interface beyond the tools and the briefing (step 4).
- Intent in plain language. Intent is a structured object. How a player enters it is a question for step 4.
- Weather, light and the moon (after the kill gate; `docs/design/planning.md`, answer 1).
- Reading the enemy's intentions from signals (layer 3 of `docs/design/intelligence.md`). The enemy's plans come to exist in this step, which is what that layer needs, but nothing reads them yet.
- The economy, production and theatre-wide allocation (step 5). In the slice, resources are ships, aircraft, fuel and supply stocks, which exist.
- The combat rules the close of step 2 named as missing (`docs/validation/step2-close.md`, "Handed to step 3"). They are taken up by what play in this step shows to matter, each as its own decision.

## What it builds on

- **A side's view.** `SideView` gives a side its own units exactly and the enemy as contacts and claims, and nothing else. Everything in this step reads a `SideView`. Nothing in it is handed the world.
- **Orders that exist:** routes, speed, readiness, rules of engagement, missions for air groups, search arcs, target priority, reach, load and unload, detach, send to the yard.
- **Measuring by the engine's own functions:** routes, passage times, fuel, reach, the strike decision (decision 0049).
- **Objectives counted from state,** with a result on one scale (decisions 0054 and 0088).
- **Every order carries the intent it serves and every event its causes.** This was put in at step 1 so that this step could use it.
- **A simulation that is deterministic and fast.** A thousand runs of a one-day battle take seconds.
- **Land units and basic ground combat** (decision 0066), built before this step so that a plan can say "take and hold a place".

## The seat and the levels

The slice's default seat is the area commander, South Pacific. Step 3 builds three levels and tests two of them hard.

| Level | The commander says | The staff does | In step 3 |
| --- | --- | --- | --- |
| Area | Take and hold Guadalcanal; keep it at 30 days of supply; deny the enemy his reinforcement | Forms the tasks, assigns forces to them, schedules convoys and air cover | Built, for the kinds of objective the slice has |
| Task | This force covers the landing until the 12th; that one runs supply to Lunga every four days | Routes, timing, readiness, rules of engagement, missions and search arcs for the groups involved | Built and tested hardest: it is what every higher level compiles to |
| Unit | This squadron flies search and not strike | Carries it out; it is the order as it exists today | Exists |

Theatre level (allocation between areas) is step 5.

## The objects

**Intent.** A statement by a commander about one thing he commands: a force, a base, an area. It has a kind, what it is about, a measure where it has one, and a date where it has one. It is given by an order, it lasts until it is changed, and it is state: it is saved with the game and is in the hash.

**Task.** What the staff makes of an intent at the level below: a named job with the forces assigned to it. "Cover the landing" is a task of a carrier force. A task is the intent the unit orders under it name as their cause.

**Plan.** A lasting object for an operation: its objectives, each with a measure and a date; its phases; the tasks in each; what each phase needs against what is on hand; and, once tried, what it is expected to cost. A plan is what the staff reports against each day.

**Staff order.** An order the staff gives. It is an order like any other, with one difference: its origin is the staff, and it carries a reason, which names the intent or task it serves and the constraint that shaped it. `explain` on a staff order gives the reason.

**Hold.** A mark on a force, a group or a base that the commander has taken it. The staff gives it no orders while it is held, and says in its report what it would have done.

**Interrupt.** A condition the commander sets that stops the clock between days: carriers sighted; a base below so many days of supply; a plan the staff can no longer meet; a force held by the commander with nothing to do.

## The day, with a staff

The game is still WEGO and a day still runs to its end once begun.

1. **Read.** The staff reads its side's view: units, contacts, claims, stocks, yesterday's events.
2. **Follow up.** It sets each plan against what has happened and marks what has drifted.
3. **Decide.** For every force, group and base that is not held, it works out the day's orders from the intents and plans in force.
4. **Show.** The orders are listed with their reasons before the day runs. The commander may change an intent, hold something, or override an order. An override holds until he releases it.
5. **Run.** The day resolves as it does now.
6. **Interrupt.** Between days the clock stops if a condition the commander set is met, and not otherwise.

With no commander present (a batch, or the enemy), steps 4 and 6 are empty and the staff plays the side alone.

## The staff's decisions

The staff is ordinary code, deterministic, with no random draw of its own outside a trial. Every decision is a function of the side's view, the intents in force and the figures, so that it can be tested alone and asked "what would you do".

It is built as **doctrine and trial**: a small set of written ways of doing each kind of task, which give candidate courses, and the trial, which says what each is likely to cost. The staff does not search the space of all orders. This is question 3 below.

What it must be able to decide in the slice, at the task level:

| Task | What the staff decides |
| --- | --- |
| Run supply or fuel to a base | Which ships, what load, the route, the days, the escort |
| Keep a base at so many days of supply | When each run must sail, from stocks, consumption and passage times |
| Cover a place or a force | Where to stand, readiness, search arcs, which groups strike and which patrol, when to withdraw to refuel |
| Strike enemy ships that come within reach | Target priority, reach, what is held back, when a contact is worth a strike |
| Bombard or raid a base | The force, the timing to arrive by night, the withdrawal |
| Intercept a force expected at a place | Where and when to be, from contacts and passage times |
| Land troops and hold a place | The shipping, the escort, the date, the follow-up supply (with decision 0066) |
| Rest, repair and replace | Which groups stand down, which ships go to a repair base or the yard and when (decision 0069) |

Some of these ask for orders the units cannot be given today. Each is a decision of its own, made when its task is built:

- a strike of a stated size, so that part of a group can be sent;
- forming and merging task forces by order (decision 0052, item 7);
- moving an air group from a carrier to a base and back (decision 0050);
- a force that retires when it has done what it came for.

And one piece of work comes first, named at the close of step 1 (decision 0055, "Left for later"): the choices the rules make inside a day (which contact a strike goes for, when a force breaks off, what a straggler does) are pulled into functions of belief and standing orders, so that the staff and the trial can ask what a commander will do without running the day.

## The trial

A trial is the staff's war game of a draft plan (`docs/design/planning.md`).

1. Start from the side's view.
2. Fill in what is not known: draw a number of enemy dispositions, each consistent with what the side holds.
3. Play each forward for the length of the plan with the real rules, the same staff playing the enemy from the enemy's view of that drawn world.
4. Report across the games as spreads, never as single numbers: losses on both sides, days to each objective, how often each objective was met, and the commonest ways it failed.

The rules a trial keeps:

- **No peek at the truth.** A trial is given a `SideView` and nothing else. Two worlds that differ only in what the side cannot see give the same trial. This is a test, in the manner of decision 0047's leak tests.
- **Deterministic.** Its draws are keyed like every other, on the day, the plan and the game of the trial. The same plan on the same day gives the same estimate, and a replay shows the estimate the commander was given.
- **It changes nothing.** It runs on copies. The game's state, hash and event log are untouched.
- **Traceable.** A figure in an estimate leads to the games it came from, and each game to its events.
- **Honest.** The trial adds no bias of its own. A side's hopefulness reaches the estimate through what it has been told: claims that overstate, and contacts that are wrong (`docs/design/planning.md`, answer 2).

How the unknown is filled in is the hardest design problem of the step and is not settled here (question 4).

## The enemy

The same staff plays the Japanese side. It is given Japanese intents, which a scenario states where it now scripts orders, and it reads the Japanese view. It is not told the Allied plan and reads nothing of the truth.

This replaces the scripted enemy for play. The scripted scenarios of steps 1 and 2 stay as they are: they are the validation fixtures, and a staff that chose differently would break the comparison with the record.

## What a side is told

Everything the staff knows it knows from its side's view, so nothing it says can tell a side more than the side already holds. Three places need care, and each gets a test:

- **A reason must not carry truth.** A staff order's reason is built from belief. It names contacts, never ships.
- **A trial's result must not carry truth.** Covered above.
- **The enemy staff's choices are the enemy's.** A side sees them only as their effects, through search and battle, as it sees a scripted enemy now.

## The build, in stages

Each stage is a plan for the designer's approval, then a decision record with what it is tested by, as in steps 1 and 2. The order is a recommendation (question 2).

| Stage | What is built | What it shows |
| --- | --- | --- |
| 0 | Before the staff: the rules' choices inside a day as functions (decision 0055); a rules version on a saved game; a trial timed by hand on the full slice with `batch` | That a trial can be fast enough, before the design leans on it |
| 1 | Intent, tasks, staff orders with reasons, hold and release. The staff for supply: keep a base at so many days | The loop end to end on the simplest task, against a scripted enemy |
| 2 | The staff for the fighting tasks: cover, strike what comes within reach, intercept, bombard, rest and repair | A side that fights a battle of the slice with no unit orders from anyone |
| 3 | The plan as an object, follow-up, and interrupts | Plan against actual, and a clock that stops for a reason |
| 4 | The enemy played by the staff, from Japanese intents | A campaign of the slice with no script on either side |
| 5 | The trial: requirements, a window from stocks and arrivals, the war game, and two or three courses of action | The estimate a commander decides on |
| 6 | Area intent compiled to tasks, and landing and holding a place | "Take and hold Guadalcanal" as one statement |

The kill gate after step 4 asks whether commanding by intent is fun and whether the staff is trusted. Stages 1 to 4 here are what that needs first. Stage 5 is what `docs/design/planning.md` says the gate judges in a small version.

## How it is tested

The hand calculation still comes first wherever a stage has arithmetic: a convoy schedule from stocks, consumption and passage times is worked by hand before it is coded. Much of the staff is choice and not arithmetic, and is tested in other ways. Each is written into the stage's decision record before any code.

- **Pure decisions.** Each staff decision is a function that can be called alone. Its tests give it a view and an intent and say what orders must come out, in small cases chosen by the architect.
- **Properties.** The staff never orders a unit it does not own or one that is held. Every staff order has a reason that names an intent in force. The same view and intent give the same orders. A staff with no intent gives no order.
- **No truth.** For each stage: two worlds that differ only in what the side cannot see give the same staff orders, the same reasons and the same trial.
- **Against the scripted battles.** Given the intents the record supports, the staff's orders for the three tuned battles are set beside the scripted orders. They need not match. Where they differ, the difference is explained or it is a finding.
- **Against a baseline.** Over many seeds the staff does better on the objectives scale than a side given no orders, and no worse than the side as scripted.
- **A whole campaign.** The full slice runs to its end, staff against staff, over many seeds, with every invariant holding and no order refused that the staff should have known would be.
- **The estimate against the outcome.** Where the side's picture is right, what happens falls inside the trial's middle nine-tenths about nine times in ten. Where the picture is wrong, the estimate is wrong in the direction the picture is wrong.
- **Legibility.** A cheap model, given a staff order and its reason, says what the staff is trying to do and what would change its mind. The rule for this test is written before it is run, and it does not score predictions that depend on where a hidden enemy is (the designer's decision at the close of step 2).
- **A model commanding by intent.** A model plays a side through the tools giving intent only and no unit orders, and its record is kept, as scenario 9's games were.

## What a scenario gains

- Intents for each side, where it now gives orders. A scenario may give both, for a side that is partly scripted.
- A length worth commanding. The staff shows itself over days and weeks. The full slice of 14 days exists; a campaign of the slice at 30 to 60 days, with objectives about holding and supplying and not only sinking, is written for this step. The released game is campaigns, and its smallest scenario, for teaching, is a few days (the designer, Nov 13, 2026).
- The one-day battles stay as validation fixtures and are not played by the staff except in the comparison above.

## Done means

Proposed; the designer settles it with the spec.

- A campaign of the slice of at least 30 days is played to its end, staff against staff, with no unit order from a person or a script, over a thousand seeds, with every invariant holding.
- Every staff order answers "why" with the intent it serves and the constraint that shaped it.
- The leak tests pass for the staff's orders, its reasons and its trials.
- A commander can hold any force, group or base and release it, and the staff works around what is held.
- A trial of a 30-day plan returns within a time the designer accepts (question 5), is deterministic, and changes nothing.
- The estimate-against-outcome test passes on the campaign.
- The staff beats the no-orders baseline on the objectives scale in each battle of the slice, and its differences from the scripted orders are each explained.
- The legibility test and a model commanding by intent are run and their records committed.
- The rulebook has a chapter on the staff: every decision it makes, as the code has it.

## Questions for the designer

Each with the architect's recommendation.

1. **Is this the right boundary for step 3?** In: intent, the staff, plans, hold and release, interrupts, the trial, the enemy as staff. Out: any screen, plain-language intent, weather, signals intelligence, the economy. Recommended: yes.
2. **Is the order of stages right: supply first, then fighting, then plans, then the enemy, then the trial, then area intent?** Recommended: yes. Supply is the simplest complete loop; the trial needs an enemy that answers, so the enemy comes before it. The alternative is the trial early, as the centre of the design, with a scripted enemy inside it until stage 4.
3. **How does the staff decide: written doctrine that gives candidate courses, with the trial to cost them?** Recommended: yes. Doctrine alone cannot weigh a course; search alone cannot explain itself. The pillars leave this open.
4. **How does a trial fill in what the side does not know?** Not settled. Three candidates: only what is held, taken as true (confident, and wrong when a carrier is missed); what is held, plus enemy forces the side last knew of and has lost, placed by how far they could have gone; or that plus forces the side has never seen, from a stated order of battle it believes the enemy has. Recommended: the second for stage 5, with the third looked at when it is seen how wrong the second is.
5. **How long may a trial take?** It sets how many games and how long a horizon the design can lean on. Recommended: measure first (stage 0), and aim at ten seconds or less for a trial the commander waits on.
6. **Is intent state, saved and hashed, with its own orders?** Recommended: yes. A replay must show the intent the staff acted on, and the staff's orders name it as their cause.
7. **Does a new campaign scenario of 30 to 60 days get written for this step, with objectives about holding and supplying?** Recommended: yes, early, in stage 1, since the supply staff needs it. It rests on land units being in.
8. **Which of the four missing orders are built in step 3?** A strike of a stated size, forming and merging forces, moving an air group to a base, a force that retires when done. Recommended: decide each when its task is built, not now; the first and second are likely to be needed in stage 2.
