# Flag Plot — Step 1 Spec: Simulation Core

Nov 9, 2026 · Amit Peer

## Scope

Step 1 delivers a headless, deterministic simulation of naval, air and logistics operations in the South Pacific, August 1942 to February 1943, run from the command line.

In scope:

- Hex map of the slice area with bases, ports and airfields.
- Ships, task forces, air groups, pilots, bases and simplified land units.
- One day of resolution: movement, search, air strikes, surface combat, ground combat, supply.
- Unit-level orders as the only input.
- A full event log with causes, and a state hash per day.
- A command-line runner for single runs, batch runs and replays.

Out of scope:

- Any UI.
- The staff planner and the AI. Both sides are driven by scripted orders in tests.
- A complete order of battle. Step 1 needs only the units its test scenarios use.
- Submarines, mines, production and reinforcement schedules. These arrive in later steps.
- Tuning against history. Step 1 makes the model run and be measurable. Step 2 makes it accurate.

## Map

The simulation uses a hex grid at 40 nautical miles per hex, in axial coordinates. The UI will never show the hexes.

| Property | Value |
| --- | --- |
| Scale | 40 nm per hex, the same as WitP:AE, so published ranges and speeds convert directly |
| Area | Rabaul to Nouméa, roughly 1,500 hexes (approximate, to be fixed when the map is drawn) |
| Coordinates | Axial (q, r), with a stored latitude and longitude per hex for rendering |
| Hex type | Deep sea, shallow sea, coast, land |
| Land terrain | Clear, jungle, mountain, atoll |
| Sites | Port size 0 to 10, airfield size 0 to 10, anchorage flag |

Movement is hex to hex inside the simulation. A task force stores its hex plus progress toward the next one, so speed differences of a knot or two still matter over a day. It counts as being in the hex whose centre is nearest, so it changes hex at the midpoint. A re-routed task force finishes the step it is on. See decision 0005.

## Data model

Static reference data is separate from mutable state. Reference data is loaded once from files. State is the only thing a day of resolution changes.

Reference data (read-only):

| Entity | Holds |
| --- | --- |
| ShipClass | Type, displacement, speed, endurance, fuel capacity, guns, torpedoes, anti-aircraft, armour, aircraft capacity, cargo and troop capacity |
| AircraftType | Role, speed, range, payload, guns, durability, manoeuvre, carrier-capable flag, service rating |
| Weapon | Range, penetration, effect, rate of fire, accuracy |
| Hex | Coordinates, type, terrain, site sizes |

State (mutable):

| Entity | Holds |
| --- | --- |
| Ship | Class, side, system damage, flotation damage, fire, fuel, ammunition, crew experience, location (task force or port) |
| TaskForce | Ships, mission, route, speed setting, rules of engagement, hex and progress, home port |
| AirGroup | Aircraft type, counts (ready, damaged, in reserve), pilots, base (airfield or carrier), mission, fatigue, morale |
| Pilot | Skills by mission type, experience, fatigue, status |
| Base | Hex, side, port and airfield size and damage, supply, fuel, aviation support, garrison |
| LandUnit | Type, strength, supply, fatigue, posture, hex |
| Order | Target entity, type, parameters, origin, day issued |
| Event | Day, pulse, type, entities involved, outcome, causes |

Every entity has a stable ID that never changes or gets reused. Orders, events and saved games refer to entities only by ID.

## Truth and belief

The simulation keeps three separate worlds: the truth, and what each side believes. Nothing a side sees is read from the truth directly.

- **Truth state:** the real position and condition of everything.
- **Belief state, one per side:** contacts with an estimated position, composition, course, age and confidence.
- **Reports:** generated from truth through a detection and error model, then written into belief.

Rules:

- A side's own units are known exactly. Enemy units exist for that side only as contacts.
- A contact can be wrong about type and number. Two cruisers reported as a carrier group is a valid report.
- Contacts age and lose confidence each pulse without a fresh sighting.
- Combat results are also reports. Claimed hits and sinkings go into belief. Actual damage goes into truth.

This split is in step 1 because it cannot be added later. The staff planner in step 3 and the UI in step 4 must read only belief state, and the type system should make reading truth from a side's view impossible.

## Daily resolution order

A day resolves in three pulses, then an end-of-day pass. Both sides' orders are fixed before the day starts.

A game day starts at 18:00 in the scenario's reference zone, which is UTC+11 for the slice. The simulation keeps one clock, anchored to UTC, and pulses are simultaneous across the map. A game day takes the reference-zone date of its daylight pulses. A pulse is a time bucket: rules that depend on light ask whether a hex is dark during a pulse, and do not read it off the pulse name. See decision 0001.

1. **Orders intake.** Validate every order against what the issuing side can see: its own units, the map and ownership. Reject invalid ones with a logged reason. An order that would fail because of something the side does not know is accepted and fails in execution. See decision 0008.
2. **Night pulse (12 hours).**
   1. Task force movement.
   2. Night search, limited to radar and night-capable aircraft.
   3. Surface engagements where opposing forces meet.
   4. Shore bombardment.
   5. Loading and unloading at ports and beaches.
3. **Morning pulse (6 hours).**
   1. Air search by arc and range.
   2. Strike decisions by local commanders, based on belief state and rules of engagement.
   3. Air combat: escort against combat air patrol, then anti-aircraft fire, then the attack.
   4. Task force movement.
   5. Surface engagements.
   6. Loading and unloading.
4. **Afternoon pulse (6 hours).** Same sequence as the morning pulse.
5. **Ground combat.** One resolution per contested hex.
6. **Logistics.** Supply and fuel consumption, base repair, ship repair in port, refuelling.
7. **Air group upkeep.** Maintenance, fatigue recovery, replacements, pilot experience.
8. **Intelligence update.** Age contacts, merge duplicates, generate the day's reports.
9. **Close the day.** Check invariants, write the event log, compute the state hash.

Strike decisions in the pulses are automatic and made by the simulated local commander. That is deliberate. A carrier admiral who sights the enemy at 09:00 does not wait for tomorrow's orders.

## Orders

Unit-level orders are the only way to influence the simulation. Intent, delegation and the staff live above this layer and compile down to these.

| Target | Orders |
| --- | --- |
| TaskForce | Form, disband, add or detach ships, set mission, set route, set speed, set rules of engagement, set home port |
| AirGroup | Set mission, set search arcs, set combat air patrol share, set target priority, set range limit, rebase |
| Base | Set repair priority, set supply and fuel reserve levels |
| LandUnit | Set posture, move, embark, disembark |

Task force missions in step 1: transport, surface combat, carrier, bombardment, escort.

Air missions in step 1: search, combat air patrol, naval strike, airfield attack, ground support, escort, rest.

Every order carries an origin field with two parts: who issued it (player or staff) and the ID of the intent it serves. Who holds the seat (human, agent or script) is recorded once per side in the saved game, not on each order. Step 1 only stores and logs this field. Steps 3 and 4 depend on it to answer "why did this happen".

Orders persist until replaced. A day with no new orders is valid and continues the standing ones.

## Agent interface

AI models are first-class players. An agent plays through the same interface a human will: it reads one side's belief state, issues orders, and advances the day. It gets no access to truth state.

What step 1 adds for this:

- **Schemas:** orders and observations are JSON with published schemas, versioned with the code.
- **Server mode:** `sim-cli serve` holds a game open and accepts calls, so a model can play turn by turn.
- **MCP server:** a thin wrapper that exposes the game as tools, so any MCP-capable model can play without custom glue.
- **Briefings:** a daily situation report per side in structured text: what changed, new contacts, losses, supply warnings, orders that failed.
- **Drill-down:** queries by entity, area or event, so an agent pulls detail on demand instead of loading the whole state.
- **Geometry helpers:** range, path, travel time and air-coverage calculations as tools. Models reason poorly over raw hex coordinates.
- **Why:** the event-log queries are exposed, so an agent can ask why something happened.

| Tool | Does |
| --- | --- |
| `get_briefing` | Returns the daily situation report for the agent's side |
| `list_forces`, `list_contacts`, `list_bases` | Returns summaries, filterable by area |
| `get_entity` | Returns full detail on one own unit or one contact |
| `measure` | Returns distance, path, travel time or air range between points |
| `issue_orders` | Validates and queues orders, returns accepted and rejected with reasons |
| `end_day` | Resolves the day once both sides have ended |
| `explain` | Returns the causes of an event from the log |

Rules:

- Agent decisions enter the game only as orders, which are recorded. A game played by a model replays deterministically from its saved orders.
- Every agent game saves a transcript of what the agent read and what it ordered, for review.
- Agent play is for dozens of games, not thousands. Batch validation stays on scripted orders, because a model takes minutes per turn.

Added test scenario 9: a model plays the Allied side of scenario 5 through the MCP server from start to finish with no human help, and the saved game replays to the same hashes.

Added repo path: `crates/sim-agent` for the server mode, briefings and MCP wrapper.

## Determinism and traceability

The same starting state, orders and seed must produce the same state hash on any machine. Every outcome must be able to say what caused it.

Determinism rules:

- **Random numbers:** counter-based generator keyed on seed, day, pulse, subsystem and entity ID. Results do not depend on the order entities are processed in.
- **Arithmetic:** integers and fixed-point for all state. No floating point in anything that feeds the hash.
- **Collections:** ordered maps and sorted iteration only. No hash-map iteration order in the rules.
- **Time and environment:** no wall clock, no thread timing, no locale.
- **Replay:** a saved game is the starting state, the seed and the list of orders per day. Replaying it must reproduce every daily hash.

Traceability rules:

- **Events:** every state change is written as an event with references to the events and orders that caused it.
- **Breakdowns:** every computed chance or quantity records its factors, such as base value, skill, fatigue, weather and range.
- **Queries:** the event log can answer "what happened to this ship today" and "why was this strike launched" without re-running the day.

The cost is a larger event log and somewhat slower runs. Batch validation can switch breakdown recording off. Event recording stays on always.

## First test scenarios

Eight scenarios, built in this order. Each one adds a subsystem, so a failure points at the newest code.

| # | Scenario | Proves |
| --- | --- | --- |
| 1 | Transit: one task force, Nouméa to Espiritu Santo | Movement, fuel burn, arrival time match hand calculation |
| 2 | Supply run: a convoy unloads at Guadalcanal | Loading, port capacity limits, base supply levels |
| 3 | Search: flying boats search an arc with a task force inside it | Detection rates over 1,000 runs, report errors, contact ageing |
| 4 | Night surface action: Savo Island, 8 to 9 August 1942 | Surface combat, surprise, night detection |
| 5 | Carrier duel: Eastern Solomons, 24 to 25 August 1942 | Search, strike decisions, air combat, ship damage |
| 6 | Air attrition: Rabaul against Guadalcanal, four weeks of raids | Range effects, fatigue, pilot losses, maintenance |
| 7 | Determinism: the full slice run twice, and once with shuffled processing order | Identical daily hashes |
| 8 | Soak: 180 days with scripted orders on both sides | No crashes, all invariants hold |

Scenarios 4 and 5 are the first historical checks. In step 1 they only need to run and produce plausible spreads. The acceptance bands for losses and damage are set from sources in step 2.

Two checks guard against rules that are correct but cannot be followed:

- **Legibility test, after each scenario from 3 on.** A cheap model is given only one side's briefing and the event log, and is asked to explain an outcome and predict the next day. Failure means the rules or the events are simplified before the next scenario.
- **Complexity review, after scenario 5.** It is the first scenario where search, strike decisions, air combat and ship damage meet. Every rule's inputs and outputs are read together before scenario 6 starts.

Base supply consumption arrives by scenario 6 at the latest. Objectives such as "30 days of supply" cannot be measured without it.

Invariants checked at the end of every day in every scenario:

- No negative fuel, supply, ammunition or aircraft counts.
- Every ship is in exactly one task force or one port.
- Every air group is at exactly one base or on one carrier.
- No belief-state data was written from anything except a report.

## Repo layout and definition of done

One Rust workspace with three crates and a data directory.

| Path | Contents |
| --- | --- |
| `crates/sim-core` | State, rules, resolution, event log. No file or network access |
| `crates/sim-data` | Loaders and schema validation for reference data and scenarios |
| `crates/sim-cli` | Command-line runner: run, batch, replay, hash, query events |
| `data/reference` | Ship classes, aircraft types and weapons as CSV |
| `data/map` | Hex data for the slice |
| `data/scenarios` | One TOML file per scenario: starting state, scripted orders, seed |
| `tests` | The eight scenarios as automated tests |

Step 1 is done when:

- [ ] All eight scenarios run from the command line and pass in CI.
- [ ] A 180-day run completes in under 60 seconds on one core (target, to be confirmed once it runs).
- [ ] Replay of any saved run reproduces every daily hash.
- [ ] The event log answers "what happened to this entity" and "why" for any event in a run.
- [ ] A batch of 1,000 runs produces a summary file of losses per side that step 2 can read.

Suggested first commit: the workspace, the entity IDs, the counter-based random generator with its tests, and scenario 1.
