# Flag Plot — Design Pillars

Nov 9, 2026 · Amit Peer

> **Proposed, awaiting the designer's review** (drafted by the architect on Nov 26, 2026, on his instruction "Amend now"; `docs/design/levels.md` has the reasoning). Every change proposed to this document is in a block marked like this one, beside the text it would change, which is left as it was. Nothing in a marked block is in force until he confirms it; he may strike any of them. The five pillars are not changed. The three questions the first draft left open he answered the same day; the blocks are reworked to his answers, which are recorded at the end, and stay proposed until he has read them.

## Concept

An operational wargame of the Pacific War at the scale of War in the Pacific: Admiral's Edition (WitP:AE), where the player commands by intent and a staff executes. Title: Flag Plot, after the compartment on a flagship where the admiral and staff tracked the battle and issued orders.

WitP:AE models every ship, air group and pilot across the whole theatre, day by day, and makes the player hand-enter orders for all of it. This game keeps that depth in the simulation and removes it from the player's hands. The player works at whatever level of command they choose. Everything they are not holding is run by a staff that shows its reasoning.

> **Proposed, awaiting the designer's review** (drafted by the architect on Nov 26, 2026, on his instruction "Amend now"; `docs/design/levels.md` has the reasoning). Add after the paragraph above: **The game is about planning.** The player is taken through levels. Each is a war small enough to hold in the head, with a mission and a date given from above. He writes a plan, may try it in a dry run and amend it, and then fights it. The plan is measured once, in the war itself: the mission is met by its date or it is not, and luck is part of it. He may choose to set his plan beside the one his staff would have made alone, before the game by a dry run and after it over many games. Failing a level does not keep him from the next; not trying does, and he may fight a level again on a new situation with a changed plan. Each level teaches one more decision of planning and practises the ones before; the scenarios grow in forces and in length, and in the later ones he may command only part of a side. The ladder is the game; play outside it, with no mark to beat, may be unlocked later. The theatre is the top of the ladder, not the first thing he is given.

It is a passion project built for an audience of one. There is no commercial target and no deadline. It is the primary side project, ahead of Haulout. The two tests of success are that the UX is wonderful and that the game is engaging to play.

## Pillars

Every feature is tested against these five. A feature that serves none of them is cut.

1. **Command at any altitude.** Zoom level is command level. The player can run the whole theatre by intent, or drop down and personally plan one carrier strike. Anything can be grabbed, and released back to staff when done.
2. **Staff you can read.** Every staff-generated order shows its reasoning and is visible before it executes. The player corrects by changing intent or by overriding. An override holds until the player releases it.
3. **Time runs until something matters.** The simulation resolves in days, and a day, once begun, runs to its end, but the player does not click "end turn" 1,600 times. A quiet week passes in seconds. A major carrier battle takes an evening. The player sets what is worth an interruption.
4. **The map is the interface.** Supply flow, air ranges, threat and readiness are answered on the map, not in modal tables. Every number can be opened to show what produced it.
5. **Uncertainty and consequence.** Sighting reports are wrong, intelligence is partial, and results come back as signals and after-action accounts. Engagement comes from real decisions on incomplete information.

Pillar 1 is the expensive one. Staff has to be competent at every level, and the UI has to work at every level. Pillars 3 and 5 are the bet on engagement, and only a playable prototype can confirm it.

## Player and staff

The player sets intent and owns the decisions they choose to own. The staff owns everything else and is accountable for explaining itself.

| Level | Example of intent | What staff does with it |
| --- | --- | --- |
| Theatre | Hold Australia's supply line, go on the offensive in the Solomons by August | Allocates forces and shipping between areas |
| Area | Take and hold Guadalcanal, keep it supplied at 30 days | Forms task forces, schedules convoys, assigns air cover |
| Task force | Intercept the reinforcement run tonight, avoid a daylight air threat | Plots the route, timing and formation |
| Unit | This squadron flies long-range search, not strike | Executes, reports readiness and losses |

Rules of control:

- **Take:** the player can take direct control of any force, base or pipeline at any level.
- **Hold:** a held item is never touched by staff, and is visibly marked as held.
- **Release:** releasing returns it to staff under the current intent.
- **Explain:** any staff order answers "why" with the intent it serves and the constraint that shaped it.
- **Interrupt:** the player defines what stops the clock between days, such as a sighting of carriers, a base falling below supply, or a plan the staff cannot meet.

## Since these pillars were written

Three directions of the designer, of Nov 15 and 16, 2026, each recorded where the detail is. They sharpen pillars 1, 2 and 5 and change none of the five.

- **An intent is an end, and intents form a tree** (decision 0116). The player states a state of the world he wants; the staff forms the tasks under it and the orders under them. The tree of an end with its order in time is the plan. The table above reads as levels of one tree.
- **A staff at every level, each on its own picture** (`docs/design/command-levels.md`, decision 0119). Each commander interprets and splits the intent he is given. Levels differ in what they know and in what they report, of the enemy and of their own forces. Commanders differ in initiative. It is built as its own stage of step 3.
- **A seat at any level.** A player may later take a role, see what that level sees, and carry out the intents of the levels above. "Anything can be grabbed" then means sitting where that commander sits.

> **Proposed, awaiting the designer's review** (drafted by the architect on Nov 26, 2026, on his instruction "Amend now"; `docs/design/levels.md` has the reasoning). A fourth direction, of Nov 26 and 27, 2026, which sharpens pillars 1, 2 and 5 and changes none of the five:
>
> - **The player writes the plan, and the war measures it** (decision 0232; `docs/design/levels.md`). A commander may write part of the tree himself and the staff fills the rest; a node is his or the staff's and says so. A plan is measured once, in the game he fights. The staff's own plan for the mission is there to set his beside if he chooses, and gates nothing. Pillar 1 is learned one altitude at a time. Pillar 2 gains a second use: the staff's plan and its reasons are what he reads to learn. Pillar 3 stands: the game he fights runs until something matters. Pillar 5 is sharpened: the consequence of a plan is one war, fought once on what he knew. Reworked on Nov 27, 2026 to his change of that day.

## Cuts

These are left out on purpose. Each one protects a pillar or keeps the scope survivable.

- **Mandatory micromanagement.** No task exists that only the player can do. If staff cannot do it, it is not in the game yet.

  > **Proposed, awaiting the designer's review** (drafted by the architect on Nov 26, 2026, on his instruction "Amend now"; `docs/design/levels.md` has the reasoning). Add to this cut: *Inside a level one thing is the player's to do: he writes the part of the plan that the level teaches, and a level is not tried until he has. The staff does everything he does not write, and could have written that part too; it is asked of him because the level exists to teach it. Nothing below the altitude he is playing is ever his to do by hand, and what he writes need not be better than the staff's: failing a level does not close the next.* This is an exception to the cut's first sentence and is worded as one. The designer ruled that trying a level is "signing a plan of your own", knowing it makes writing mandatory inside a level, and that "the ladder is the game, but we may decide later that free play is unlocked"; the wording is the architect's and is his to change. Reworked on Nov 27, 2026 to his change of that day.

- **Modal spreadsheet screens.** Lists exist as a secondary view, never as the only route to a decision.
- **Japanese industrial management as a spreadsheet game.** Production is set by policy and priorities, with staff handling the conversion detail.
- **Detailed ground combat.** Land fighting is modelled at the level naval and air decisions need. This is not a ground-war game.
- **Art and audio polish.** Vector map, NATO-style counters, clean typography. The budget goes into information design.
- **Multiplayer infrastructure.** Single-player against the AI first. Turn files stay PBEM-compatible, but no lobby, matchmaking or server.
- **Anything from the WitP:AE database.** The order of battle is built from open historical sources.

## First slice

The first playable version is the South Pacific from August 1942 to February 1943, at full mechanical depth, with the architecture sized for the whole map.

| Aspect | Scope of the slice |
| --- | --- |
| Period | Guadalcanal landings to the Japanese evacuation, about 180 daily turns |
| Map | Rabaul to Nouméa: the Solomons, eastern New Guinea, New Hebrides, New Caledonia |
| Default seat | Area commander, South Pacific |
| Playable levels | One up (theatre allocation, simplified) and one down (task force) |
| Sides | Allied player against a Japanese AI first, then the reverse |
| Systems | Naval movement and combat, air operations, supply and shipping, search and fog of war, basic ground combat |

> **Proposed, awaiting the designer's review** (drafted by the architect on Nov 26, 2026, on his instruction "Amend now"; `docs/design/levels.md` has the reasoning). **The first playable version is the first levels** (his answer of Nov 26, 2026): the first rungs of the ladder, small scenarios, the first of them invented and marked placeholder, then levels cut from the battles already on file. The slice in the table above becomes the top rung, reached later, and the long order-of-battle work for it waits. The table is left as it was; read it as the top rung's scope.

This campaign is chosen because it stresses every system at once in a small space: carrier battles, night surface actions, an air war of attrition, and a supply fight both sides nearly lost. If the command model works here, it scales. If it fails here, a bigger map will not save it.

## Build sequence

Five steps, with a kill gate after step 4. No full database and no real UI before the gate is passed.

1. **Headless simulation.** Deterministic naval, air and logistics engine for the slice. No UI. Driven by tests and batch runs.
2. **Validation harness.** Thousands of automated runs compared against historical outcomes: loss ratios, sortie rates, supply consumption. The model is tuned here, not by feel.
3. **Staff planner.** The layer that turns intent into orders. It serves player delegation and the enemy AI, so it is built once.
4. **Command-loop prototype.** Minimal UI over steps 1 to 3: set intent, read staff plans, run time, get interrupted, react.
5. **Full build.** Real UI, complete order of battle, more theatres, the Japanese side.

> **Proposed, awaiting the designer's review** (drafted by the architect on Nov 26, 2026, on his instruction "Amend now"; `docs/design/levels.md` has the reasoning). Steps 1 to 3 are unchanged. Step 4, the command-loop prototype, is the first levels: a mission held from above, a plan written and signed, a dry run, the one game that measures it, the debrief, and the history of a level. Step 5 is the ladder built up to the slice, before more theatres. **The kill gate is judged on levels** (his answer of Nov 26, 2026): its questions are answered by playing the first levels. Proposed wording of the four, for his review; the originals below are left as they were:
>
> - Is writing a plan and seeing it fought once worth doing again, or is it filling a form?
> - Do you trust the staff to carry out your plan as you wrote it, and its dry run to tell you something, or do you stop reading both?
> - Does the game you fight with your plan produce tension when the clock stops, or just a stream of pop-ups?
> - Do you read the account of the game and ask for the debrief, or move on without them?
>
> A "no" on any of the first three means reworking the level before a ladder is built; a "no" on the fourth means reworking how the game is told and what the debrief shows. Reworked on Nov 27, 2026 to his change of that day.

**Kill gate, after step 4.** Four questions, answered by playing the prototype for a few weeks:

- Is commanding by intent fun, or does it feel like watching a screensaver?
- Do you trust the staff, or do you keep overriding it?
- Does the interrupt-driven clock produce tension, or just a stream of pop-ups?
- Do you read the diary of the day, or skip it? (`docs/design/storytelling.md`)

A "no" on any of the first three means reworking the command model before step 5, and a "no" on the fourth means reworking how the day is told. Planning an operation is judged at the gate in a small version (`docs/design/planning.md`). The order-of-battle research for step 5 is the long grind, and it should not start on an unproven design.

## Architecture decisions

These are proposals, each chosen to keep the simulation testable and the UI replaceable.

| Decision | Choice | Reason |
| --- | --- | --- |
| Simulation core | Rust, as a pure library with no UI dependency | Deterministic, and fast enough for thousands of validation runs |
| UI | TypeScript on web technology, wrapped with Tauri for desktop | Fastest UI iteration, and a path to a tablet client |
| Turn structure | WEGO: both sides plan, then the day resolves | Suits carrier warfare and makes the AI and PBEM use the same turn files |
| Determinism | Same inputs and seed always give the same result | Required for testing, replays and bug reports |
| Data | Order of battle and unit data as versioned text files in the repo | Reviewable in diffs, and open to outside contributors later |
| Staff planner | One planner for player delegation and enemy AI | Halves the AI work and keeps both sides playing by the same rules |
| Explanations | Every order and every number records what produced it | Pillars 2 and 4 depend on it, and it cannot be retrofitted |

The last row is the one to get right from the first commit. Traceability added later means rewriting the simulation.

**AI models as first-class players.** A model plays through the same interface as a human: one side's belief state in, orders out, no privileged access. The game is exposed as an MCP server from step 1. Models are used to playtest, to find exploits and broken rules, to act as an opponent before the staff planner exists, and later to test the intent layer. The in-game staff planner stays deterministic code, so the game runs offline and replays exactly. The kill gate is still judged by a human, because a model cannot tell you whether the game is fun.

## Open questions

- [ ] Haulout: paused, or kept moving at low priority?
- [ ] Title: decided, Flag Plot. Check Steam for a name collision.
- [ ] Stack: decided, Rust core and TypeScript UI.
- [ ] Map model: decided, 40 nm hexes in the simulation, hidden in the UI.
- [ ] Staff planner approach: deterministic code is decided. Still open: hand-written planning logic, search, or a mix?
- [ ] Sources for the order of battle, and how each entry records where it came from.
- [ ] How the player expresses intent: map gestures, structured forms, or plain language?
- [ ] The plan loop, to be designed in step 3 and judged at the kill gate. A plan is a lasting object, not a one-off instruction, and it runs in four stages:
  1. Plan: objectives, each with a measure and a date, such as "Guadalcanal held at 30 days of supply by 1 October".
  2. Feasibility: the staff costs the plan before the player commits, such as "this needs 14 transports and you have 9".
  3. Execute: the staff compiles the plan into orders.
  4. Follow up: plan against actual, with an interrupt when they drift apart.

  What it asks of step 1: every order carries the intent it serves, and every objective can be measured from state. War-wide strategy (production, the whole Pacific) stays in step 5.
- [ ] Open source from the start, or decide after the kill gate?

## Proposed amendment: the designer's answers

The first draft of the proposed blocks left three questions open, on which his intent could be read two ways. He answered them on Nov 26, 2026, and the blocks above are reworked to the answers. They stay proposed until he has read them.

1. **Is there play outside the ladder?** "I think the ladder is the game, but we may decide later that free play is unlocked."
2. **What is the first playable version?** "The first levels": the first rungs, small scenarios; the slice becomes the top rung, reached later, and the long order-of-battle work for it waits.
3. **What is the kill gate judged on?** "Judged on levels": answered by playing the first levels. The wording of the gate's questions, proposed above, is his to review.

On Nov 27, 2026 he changed how a level is measured (once, in the war; scoring against the staff is the player's choice; failure does not block, not trying does; trying is "signing a plan of your own"; a replay is on a new situation and the last game counts). It replaces "must beat par to pass" and the two-part score of Nov 26, on which the first draft of several blocks rested; those blocks are reworked and say so. `docs/design/levels.md` records each answer and what replaced it.
