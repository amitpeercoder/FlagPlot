# Design note: planning an operation

Status: a note for discussion, written by the architect on Nov 12, 2026 at the designer's request. Nothing here is built. It says how a player could plan a large operation with the staff, be told what it needs and what it is likely to cost, and be faced with a commander's choices. The three questions it first left open were answered the same day; the answers are below. It says what exists to build on and what is missing. Historical statements below are general and unsourced; any figure the game uses must come from sourced facts, as everywhere.

## The problem

War in the Pacific: Admiral's Edition has no way to plan. A player who wants to invade an island works out alone what it needs, when it could go and what else must be given up, and learns the cost only by doing it.

The designer wants planning to be part of the game. The player should be able to lay out an invasion and be told by the staff:

- what it entails: forces, shipping, fuel, supply;
- when it could go and when it would best go: weather, light, what is in stock and what arrives when;
- what it takes from other operations;
- what it is expected to cost in troops, ships, tonnage and time.

And then the player decides, as a commander does, between courses that each cost something.

## What exists to build on

- **The plan loop is already in the pillars** as an open question for step 3: plan, feasibility, execute, follow up. A plan is a lasting object with objectives, each with a measure and a date. This note is that loop with the feasibility stage worked out.
- **The simulation is deterministic and fast.** `sim-cli batch` runs a scenario over many seeds today.
- **A side's belief is separate from the truth,** and is what anything that plays the game is given (decision 0007).
- **Measuring is by the engine's own functions:** routes, passage time, fuel and reach (decision 0049).
- **Supply and fuel are tons** that are stocked, carried and spent, and aircraft and pilots arrive at rates (decisions 0038, 0039, 0045, 0046).
- **Objectives are counts read from state** (decision 0054).
- **Every order carries the intent it serves,** and every event its causes.

## The idea in one line

The staff does not estimate with planning formulas of its own. It plays the plan forward many times with the real rules, from what the side believes, and reports what happened across those games.

## The trial

A trial is the staff's war game of a draft plan.

1. Start from the side's belief: its own units exactly, the enemy as contacts, claims and, later, estimates.
2. Fill in what is not known. Draw a number of enemy dispositions, each consistent with what the side holds.
3. Play each forward for the length of the plan, with the real rules and a planner playing the enemy.
4. Report across the games: ships lost, aircraft and pilots lost, troops lost, tonnage sunk, days to each objective, and how often each objective was met.

What follows from doing it this way:

- **One set of rules.** A planning formula kept beside the simulation drifts from it. A trial cannot.
- **The estimate is wrong in the way a staff's is.** It rests on belief. If the side has missed a carrier division, the estimate is hopeful and the player finds out at sea. Search and intelligence therefore buy better plans, which is the reason to pay for them.
- **Every figure can be opened.** The player can look at any one game of the trial and see why it went badly. This is pillar 2 applied to an estimate.
- **The enemy answers.** The trial needs an enemy that reacts to the operation, so the planner plays both sides inside it. This supports the pillars' choice of one planner for both sides.

## The plan as an object

A plan has phases. An invasion might have four: build up, wear down the enemy's airfields, escort and land, sustain. Each phase states:

| Part | What it says | Example |
| --- | --- | --- |
| Requirements | Forces, shipping, fuel and supply, against what is on hand, with each shortfall named | Needs 14 transports; 9 are free; 5 more on 12 October if the Nouméa run is cut |
| Window | The earliest date it can go and the best date, and what sets each | Stocks at Espiritu Santo reach 30 days on the 9th; no moon from the 10th to the 14th |
| What it takes | What those forces and that fuel were committed to, and what slips | The Port Moresby convoy goes a week late |
| Risk | The trial's spread of losses and time, and the commonest ways it failed | In 3 games in 10 the transports were caught unloading by air from Rabaul |

A plan is saved with the game, is what its orders name as their intent, and is what the staff reports against afterwards.

## Courses of action

The staff brings two or three courses, not one answer, each with its table of cost.

- Go on 7 October with nine transports and thin air cover.
- Wait until the 20th for the second carrier, while the enemy lands another regiment.
- Go on the 7th for a nearer, smaller objective and build an airfield first.

The player chooses, changes a course and asks for it to be tried again, or drops the operation. This is where the commander's dilemma is: nothing is free, and the staff says what each choice costs before it is made.

## After the decision

- **Execute.** The staff turns the plan into orders. The player can take any part under direct control and release it again.
- **Follow up.** Each day the staff sets the plan against what has happened. When they drift apart beyond what the player set, the clock stops: the tanker was lost, the bombardment group will sail at 60 in 100 of its fuel, do we go?
- **In the diary.** A plan gives the chronicle its measure of what mattered (`docs/design/storytelling.md`). "Three days before the landing" is a story because there is a plan.

## What is missing

| Missing | Where it stands | What it holds up |
| --- | --- | --- |
| The staff planner | Step 3; its approach is an open question in the pillars | Everything here. The trial is its central job |
| Ground combat and land units | Left out of step 1 (decision 0052); to be built before step 3 (answer 1 below) | An invasion, and any estimate of troops lost |
| Weather, light and the moon | Left out of step 1 (decision 0052); after the kill gate (answer 1 below) | A best date on anything but stocks and arrivals |
| Estimates of enemy intent | Layer 3 of `docs/design/intelligence.md`, which needs the planner | A trial that knows an enemy convoy is expected |
| Figures set from the record | Step 2, under way | Any trial worth believing. On placeholder figures a spread of losses means nothing |
| Materials and oil | Step 5 (`docs/design/economy.md`) | In the slice, resources are shipping, fuel and supply stocks, which exist |

## Rules it must keep

- **No peek at the truth.** A trial starts from `SideView` and never from the world. The enemy dispositions it draws are made from belief. A trial that was handed the truth would tell the player where the enemy is by how the estimate came out. The leak tests of decision 0047 should cover it: two worlds that differ only in what the side cannot see must give the same trial.
- **Deterministic.** A trial's draws are keyed like every other, so the same plan on the same day gives the same estimate, and a replay shows the estimate the player was given.
- **A trial changes nothing.** It runs on copies. No state, hash or event log of the game is touched.
- **Traceable.** A figure in an estimate leads to the games it came from, and each game to its events.
- **No clerical work.** The player states the objective and the limits. The staff finds the forces, the shipping and the dates.

## Risks

- **Time to run.** A few hundred games of a 30-day plan must come back in seconds, or planning is waiting. Measure it early, on the full slice, before the design leans on it. Fewer games, a shorter horizon or a coarser trial are the ways out.
- **Filling in the unknown.** How the staff draws enemy dispositions from belief is a design problem of its own. Drawn too narrowly, every estimate is confident and wrong; too widely, every estimate says "anything could happen".
- **False precision.** "Expect to lose 2.3 destroyers" is worse than "one to four, most often two". Estimates should be shown as spreads.
- **A staff the player cannot trust.** If the estimates are often far out, the player stops reading them, and the kill gate's second question fails.
- **Planning that plays the game for the player.** If the staff's best course is always right, there is no decision. The courses must differ in what they risk, not only in how good they are.

## Order of arrival

1. **Before step 3:** time a trial by hand: the full slice, 30 days, a few hundred seeds, on one machine. It needs no new code beyond what `sim-cli batch` does.
2. **Before step 3:** land units and basic ground combat (answer 1 below).
3. **Step 3:** the plan as an object, requirements and windows from stocks and measuring, the trial, courses of action, follow up, for naval, air and landing operations.
4. **Step 4:** the small version the kill gate judges (answer 3 below).
5. **After the kill gate:** weather, light and the moon, and windows that read them.
6. **Step 5:** plans across theatres, and resources that come from the economy.

## Answers from the product managers

Agreed by the designer on Nov 12, 2026, on the architect's suggestion. These are directions for the design, not built rules; each becomes a decision record when its part is planned.

1. **Basic ground combat comes before the planner, and weather after the kill gate.** Land units and basic ground combat are built between step 2 and step 3, so that the planner's objectives can say "take and hold a place" from the start; adding land to a planner designed without it is the dearer road. This takes up what decision 0052 deferred, and the plan is `docs/design/ground-combat.md`, and each stage gets its decision record before it is built. Weather, light and the moon follow the kill gate.
2. **The staff estimates honestly, and a side's hopefulness comes from what it is told.** The trial adds no bias of its own. Where a side's picture was historically too hopeful, that reaches the estimate through its inputs: claims that overstate (decision 0056) and the reliability of each source, which are figures by side and period set from the record (`docs/design/intelligence.md`). A side that believes it sank four carriers plans against an enemy it thinks is weak. A second bias in the estimate would count the same thing twice, would need a figure no source gives, and could not be explained to the player, where a false claim behind an estimate can be opened and seen.

   Left open for later: planning assumptions. A staff that assumes the enemy will do what the plan needs is a failing the record may support for a side and a period. In the trial it is how the enemy's response is drawn. It is not built for the slice, and it is considered only if research finds something to rest it on.
3. **Planning is judged at the kill gate, in a small version.** The step 4 prototype carries, for one operation: requirements, one window, a trial and two courses of action.
