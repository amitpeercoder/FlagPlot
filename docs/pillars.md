# Flag Plot — Design Pillars

Nov 9, 2026 · Amit Peer

## Concept

An operational wargame of the Pacific War at the scale of War in the Pacific: Admiral's Edition (WitP:AE), where the player commands by intent and a staff executes. Title: Flag Plot, after the compartment on a flagship where the admiral and staff tracked the battle and issued orders.

WitP:AE models every ship, air group and pilot across the whole theatre, day by day, and makes the player hand-enter orders for all of it. This game keeps that depth in the simulation and removes it from the player's hands. The player works at whatever level of command they choose. Everything they are not holding is run by a staff that shows its reasoning.

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

## Cuts

These are left out on purpose. Each one protects a pillar or keeps the scope survivable.

- **Mandatory micromanagement.** No task exists that only the player can do. If staff cannot do it, it is not in the game yet.
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

This campaign is chosen because it stresses every system at once in a small space: carrier battles, night surface actions, an air war of attrition, and a supply fight both sides nearly lost. If the command model works here, it scales. If it fails here, a bigger map will not save it.

## Build sequence

Five steps, with a kill gate after step 4. No full database and no real UI before the gate is passed.

1. **Headless simulation.** Deterministic naval, air and logistics engine for the slice. No UI. Driven by tests and batch runs.
2. **Validation harness.** Thousands of automated runs compared against historical outcomes: loss ratios, sortie rates, supply consumption. The model is tuned here, not by feel.
3. **Staff planner.** The layer that turns intent into orders. It serves player delegation and the enemy AI, so it is built once.
4. **Command-loop prototype.** Minimal UI over steps 1 to 3: set intent, read staff plans, run time, get interrupted, react.
5. **Full build.** Real UI, complete order of battle, more theatres, the Japanese side.

**Kill gate, after step 4.** Three questions, answered by playing the prototype for a few weeks:

- Is commanding by intent fun, or does it feel like watching a screensaver?
- Do you trust the staff, or do you keep overriding it?
- Does the interrupt-driven clock produce tension, or just a stream of pop-ups?

A "no" on any of them means reworking the command model before step 5. The order-of-battle research for step 5 is the long grind, and it should not start on an unproven design.

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
