# Design note: planning an operation

Status: a note for discussion, written by the architect on Nov 12, 2026 at the designer's request. Nothing here is built. It says how a player could plan a large operation with the staff, be told what it needs and what it is likely to cost, and be faced with a commander's choices. It says what exists to build on, what is missing, and what is open for the designer. Historical statements below are general and unsourced; any figure the game uses must come from sourced facts, as everywhere.

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
| Ground combat and land units | Left out of step 1 (decision 0052) | An invasion, and any estimate of troops lost |
| Weather, light and the moon | Left out of step 1 (decision 0052) | A best date on anything but stocks and arrivals |
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
2. **Step 3:** the plan as an object, requirements and windows from stocks and measuring, the trial, courses of action, follow up. Naval and air operations only.
3. **When ground combat exists:** invasions and troop losses.
4. **When weather exists:** windows that read it.
5. **Step 5:** plans across theatres, and resources that come from the economy.

## Open questions for the designer

1. **Ground combat and weather:** do they enter the slice before the planner is built, so that step 3 plans invasions from the start, or after, with step 3 planning naval and air operations first? The pillars list basic ground combat among the slice's systems; decision 0052 deferred it.
2. **Should the staff's estimates be imperfect beyond the gap in belief,** for example a staff that leans hopeful? The architect's view is no. Belief already makes the estimates wrong, and a staff that misleads works against the trust the kill gate asks about.
3. **Is planning judged at the kill gate?** If so the step 4 prototype needs at least requirements, one window and a trial for one operation.
