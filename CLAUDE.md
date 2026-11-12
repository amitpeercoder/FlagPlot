# Flag Plot

An operational wargame of the Pacific War where the player commands by intent and a staff executes. This repo currently holds step 1: a headless, deterministic simulation core.

Read these before changing anything:

- `docs/pillars.md`: what the game is and the five pillars every feature is tested against.
- `docs/step1-spec.md`: the spec for the simulation core. It is the source of truth.
- `docs/decisions/`: choices the spec left open, one short file each.
- `docs/glossary.md`: one meaning for each word the rules use.
- `docs/complexity-review-after-scenario-5.md`: what was cut, merged and left alone after scenario 5, and why.

If the spec is ambiguous or looks wrong, stop and ask. Do not guess. If you make a design choice the spec did not make, record it in `docs/decisions/` as the next numbered file.

## Commands

```
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo run -p sim-cli -- run data/scenarios/01_transit.toml --events
cargo run -p sim-cli -- export data/scenarios/04a_night_action.toml --out viewer/replays/04a_night_action.json
cargo run --release -p sim-cli -- batch data/scenarios/04_savo_island.toml --out /tmp/savo.json
cargo run -p sim-cli -- run data/scenarios/07_full_slice.toml --save /tmp/slice.save.json
cargo run -p sim-cli -- replay /tmp/slice.save.json
cargo run -p sim-cli -- hash data/scenarios/07_full_slice.toml
cargo run --release -p map-tool -- render
cargo run -p oob-check -- data/oob
cargo run -p oob-derive -- data/derivation/savo_island.toml --check
cargo run -p oob-derive -- data/derivation/rabaul_guadalcanal.toml --check
```

`serve SCENARIO --seat allied=player --seat japanese=script` holds a game open and takes one JSON request a line on standard input (decision 0050). `mcp SCENARIO --seat allied` serves the same game and tools as a Model Context Protocol server on standard input and output, and `play --game FILE ... TOOL [ARGUMENTS]` makes one call to it and exits, keeping the game as a file of the calls made (decision 0051); `play` alone prints how. `run --save FILE` writes a saved game (the scenario's name, the seed, each day's orders and the daily hashes); `replay FILE` runs it again and fails on the first day whose hash differs, and refuses a scenario or data file that has changed; `hash` prints the daily hashes only (decision 0043). `export` runs a scenario exactly as `run` does and writes it as one replay file for the viewer (decision 0018); it also takes `--seed N` and `--labels data/map/south_pacific_anchors.csv`. `batch` runs a scenario over many seeds (`--runs N`, default 1000, `--first-seed S`, default 1) and writes one summary file of losses per side, per ship and per run, for step 2 to read. Open `viewer/index.html` in a browser and pick the file. `node viewer/check.js viewer/replays/*.json` checks replays against the page's own rebuild.

The map tool also has `locate --lat --lon`, `path --from q,r --to q,r`, `sides --hex q,r` (a hex's closed sides and neighbours), `water --lat --lon` (why a position's hex has the water it has) and `generate`, which rebuilds `data/map/south_pacific.csv` from the Natural Earth files under `references/` (the exact command is in the file's header). `generate --anchors data/map/south_pacific_anchors.csv` applies the listed anchorages (decision 0017, decision 6); without it hexes keep their largest body of water. `render` draws `#` land, `+` coast, `%` coast with a closed side toward water and `.` sea, each row labelled with its r and the q of its first hex.

`oob-derive` derives ship classes for a list of real ships from `data/oob` (decision 0020). A ship list may name working assumptions (unknown radar means none, decision 0019; main-gun ammunition not limiting, decision 0023; surface weapons not modelled, decision 0029), may ask for the air figures and may list aircraft types to derive, and the rules file names every exception. With any problem it writes nothing and lists every problem; `--check` only reports; `--facts DIR` reads another facts directory.

All three checks must pass before a commit. CI runs the same three. `cargo test` includes the order-of-battle check, so a bad row in `data/oob` fails the build; `oob-check` run by hand lists the problems.

## Layout

| Path | Contents |
| --- | --- |
| `crates/sim-core` | State, rules, resolution, event log. No dependencies. No file or network access |
| `crates/sim-data` | Loads reference data, maps and scenarios from files into `sim-core` types |
| `crates/sim-cli` | Command-line runner |
| `crates/sim-agent` | Agent interface: the daily briefing, the list and detail queries, `explain`, `measure`, and the session a seat plays through (`issue_orders`, `end_day`), as JSON with schemas in `schemas/` (decisions 0048 to 0050). Every tool takes a `SideView`; only `session.rs` holds the world. The MCP server that wraps it is in `sim-cli` (decision 0051) |
| `crates/map-tool` | Generates the hex map of the slice from open coastline data, and renders it, locates points and finds sea paths. May use floats and its own dependencies; nothing depends on it |
| `crates/oob-check` | Checker for the order-of-battle facts in `data/oob`. Depends on nothing else in the workspace |
| `crates/oob-derive` | Derives ship class figures for a list of real ships from `data/oob` and `data/derivation` (decision 0020). Depends only on `oob-check`; `sim-data` and `sim-core` only in its tests |
| `data/reference` | Ship classes, port sizes and aircraft types as CSV; search, surface combat and air combat figures as TOML. Files `oob-derive` generates (a ship class file `<name>.csv` with `<name>.sources.csv` and `<name>.ships.csv`, and an aircraft type file with its own `.sources.csv`) say so in their header and are never edited by hand |
| `data/derivation` | The derivation rules and constants (`rules.toml`) and one ship list per scenario, for `oob-derive` |
| `data/map` | The generated map of the slice as CSV (`south_pacific.csv`, decision 0017) and the anchorages it was generated with (`south_pacific_anchors.csv`), and small hand-drawn placeholder maps as TOML |
| `data/scenarios` | One TOML file per scenario: starting state, seed, scripted orders |
| `data/oob` | Order-of-battle facts: sourced historical statements as CSV. Research, not simulation data. Start with its `README.md` |
| `docs/research/oob` | The facts format, the source policy, where sources disagree, and a note per battle and per topic |
| `tests` | One test file per scenario, each with its hand calculation written out |
| `viewer` | The read-only replay viewer (decision 0018): one HTML page with no build step, its check script, and `replays/` (git-ignored) for exported runs |

Crate boundaries:

- `sim-core` depends on nothing, not even other workspace crates. Keep it that way.
- Only `sim-data` reads simulation data files. The simulation never reads `data/oob`; only `oob-check` and `oob-derive` do.
- `sim-cli` and the tests are the umpire. They may read truth through `World::umpire()`.
- Anything that plays the game (the staff planner, an agent, the UI) gets a `SideView` and nothing else.
- `sim-agent` depends on `sim-core` only. Its library code takes a `SideView` and never the world, the umpire's view or the full event log; a test reads its source to check.
- The replay viewer (`viewer/`) is an umpire's tool like `sim-cli`: it shows the truth. Nothing that plays the game may use it or the replay files.

## Determinism rules

The same starting state, orders and seed must give the same state hash on any machine. Breaking any of these rules breaks replays.

- **No floating point in `sim-core`.** Use `fixed::Fixed` for every quantity. `clippy::float_arithmetic` is denied in the crate.
- **No floats on the way in.** Data files give quantities as whole numbers or decimal strings such as `"0.25"`. The loader rejects TOML floats.
- **Ordered collections only.** `BTreeMap` and `BTreeSet`, never `HashMap` or `HashSet`. Clippy rejects the hash types across the workspace.
- **No wall clock, thread timing or locale.** Clippy rejects `SystemTime::now` and `Instant::now`.
- **All randomness through `World::rng`.** Each draw is keyed on seed, day, pulse, subsystem and entity, so results cannot depend on processing order. Never share one stream between entities, and never carry a stream from one pulse to the next.
- **Never renumber stable codes.** `Subsystem::code`, `Pulse::code`, `EntityRef::kind_code`, `Side::code` and the like feed the random number keys and the state hash. Add new values at the end.
- **The random number generator's output is pinned** by a test in `rng.rs`. If that test fails, saved games no longer replay. Do not update the pinned numbers without a decision record.
- **New state goes into the hash.** When a field is added to the state, add it to `hash::hash_world` in the same change.
- **Overflow panics in every profile.** `overflow-checks` is on for release builds. Do not turn it off.
- **No `unsafe`.** It is forbidden workspace-wide.

## Traceability rules

Every outcome must be able to say what caused it. This cannot be added later.

- **Every state change records an event.** If a rule changes truth state and writes no event, that is a bug.
- **Every event names its causes:** the orders and earlier events that led to it. Follow the pattern in `resolve.rs`: movement cites the standing orders, and fuel burn and arrival cite the movement event.
- **Every computed quantity or chance records a breakdown** of its factors when `SimConfig::record_breakdowns` is on. Breakdown recording must never change what happens or the state hash.
- **Every order carries its origin:** who issued it and the intent it serves.

## Rule complexity

The game must stay something a person and a model can both reason about. A rule set that is correct but cannot be followed is a failure.

- **One mechanism over several.** A new rule reuses or replaces an existing mechanism where it can. Two limits on the same quantity need a reason, written in the decision record.
- **Every decision record that adds a rule carries a rule card:** the state the rule reads and the state it writes. A rule that reads what many other rules write is where interactions hide.
- **Every scenario test has property tests beside its hand calculation.** They state what must always hold: more of a helpful thing never makes the outcome worse, and quantities that should be conserved are conserved.
- **The legibility test, from scenario 3 on.** A cheap model is given only one side's briefing and the event log, and is asked to explain an outcome and predict the next day. If it cannot, the rules or the events need simplifying before the next scenario starts.
- **Complexity review after scenario 5,** the first scenario where several subsystems meet. The rule cards are read together and anything nobody can explain is cut or merged.

## Truth and belief

- Truth state is the real world. Belief state is what one side thinks.
- A side's own units are known exactly. Enemy units reach a side only as contacts.
- The only way to change a belief state is `BeliefState::apply_report`. Keep it that way: it is what makes "belief is written only from reports" true.
- Never add a method to `SideView` that returns enemy truth.
- The event log is truth. A side gets its events only through `SideView::events()`, which gives each event whole, cut down or not at all (decision 0047). The rule is one exhaustive `match` in `side_events.rs`: a new kind of event does not compile until someone decides who may see it.

## Data rules

- **Do not invent historical figures.** Anything made up is marked `placeholder` in its file and uses a name starting with "Placeholder".
- **Nothing from the War in the Pacific: Admiral's Edition database.** Real data comes from open historical sources, with the source recorded.
- Entity IDs in data files are stable. Never renumber or reuse one.

### Order-of-battle facts

`data/oob` holds what sources say about real ships, aircraft, weapons and battles. `data/reference` holds the figures the simulation uses. Decision 0015 and `docs/research/oob/database-design.md` give the full rules. The ones that matter most:

- **No real ship, aircraft or weapon gets an invented rating.** A game figure for a real entity is derived from fact rows by a written rule, and can name the rule and the rows. The derivation now exists: `oob-derive`, with its rules in `data/derivation/rules.toml` and decision 0020. Until it can derive a ship, figures in `data/reference` stay placeholders.
- **One row is one source's statement.** It carries its source, where in the source, the date read, a confidence and a standing. A missing fact is a missing row, listed in `gaps.csv`. Nothing is guessed.
- **Cite what was read.** A figure a website quotes from a book cites the website. `docs/research/oob/source-policy.md` says which sources may be used.
- **A source that disagrees gets its own row.** Rows are never overwritten to settle a disagreement. The architect settles it and writes the reason on the row (see "Who settles data" below).
- **A value has to earn `preferred`.** A lone weak row is `unconfirmed`, and a rule may read it only if the rule says so.
- **Engagement rows are history to test against.** A scenario may take starting positions, readiness and contacts already held from them. Its result is compared with the recorded outcome and is never scripted to match it.
- **Run `oob-check` after any change to `data/oob`.**

### Who settles data

- **The architect settles data, not the designer.** Which of two disputed rows to prefer, which weak row to read by exception, what to assume where the sources are silent: the architect decides these to the best of its knowledge and does not ask Amit to approve them. This replaces the earlier practice of a settlement sheet for his approval (decisions 0015, 0019, 0023).
- **Every such choice is still written down:** the reason on the row, the exception in the rules file, the assumption in the scenario's ship list, and a decision record where a choice shapes a scenario. A choice is marked with who made it, so "settled by the architect" is never confused with "stated by a source".
- **Ask for more research when it is worth it.** If a figure matters to a result and rests on a weak or single source, the architect sends researchers (cheaper agents, or the order-of-battle session) to look for a better one before settling, and says in the record what was looked for and not found.
- **This is about data only.** What the game is, the spec, and rules that change how the game plays are still the product managers' to decide.
- **Nothing here loosens the source rules.** No invented figures, no War in the Pacific data, every row cites what was read, and a missing fact stays a gap.

## Reference material

`references/` is local only and ignored by git. It holds the designer's own copy of War in the Pacific: Admiral's Edition: the manuals as PDF and text, a map image and the installers.

- **Mechanics and lessons, not numbers.** Use the manuals to learn how a subsystem was modelled and where it made the player do clerical work. Do not copy their tables, formulas' constants or data. Our figures come from open historical sources, or are marked `placeholder`.
- **Own words, with section numbers.** Notes go in `docs/research/`, written in our own words and citing manual sections. No copied passages.
- **Read just in time.** Before planning a scenario, read the sections `docs/research/witpae-manual-index.md` lists for it. Report how WitP:AE models it, what the player had to do by hand, and what Flag Plot should do differently.
- **The map image is for looking at only.** Do not trace hexes or take terrain from it. The real map is generated from open coastline data.
- **Never commit anything from `references/`.** Do not run or unpack the installers.
- **`references/natural-earth/`** holds public-domain coastlines (Natural Earth 1:10m land and minor islands, 5.1.1), used only by the map tool, never committed.

## Roles and delegation

| Role | Who | Owns |
| --- | --- | --- |
| Product manager | Amit, with the main session | What the game is, the spec, every decision marked "needs review", approval of each scenario plan. Not data: see "Who settles data" |
| Architect | The main session (the most capable model) | Settling data, the plan for each scenario, crate boundaries, the determinism and traceability rules, the hand calculation, decision records, final review before a pull request |
| Team lead | An Opus subagent | Turning an approved plan into tasks, briefing the developers, reviewing their work against the plan and this file, reporting back what was built, deferred and unclear |
| Developers | Sonnet subagents | Implementation tasks, tests, bulk reading and summarising of reference material, code search |

Rules:

- **Use the cheapest model that can do the job well.** Haiku is enough for plain lookups.
- **The hand calculation is written by the architect before any code is delegated.** Developers make the code match it. Nobody changes an expected number to make a test pass; a mismatch goes back to the architect.
- **The architect opens and merges pull requests without waiting for Amit,** who delegated both on Nov 11, 2026. A pull request is merged only after its CI check is green, and what was merged is reported to him. Nobody pushes to `main` directly.
- **Nothing reaches a pull request unreviewed.** The team lead reviews the developers. The architect reviews the team lead, reads every change to `sim-core`, and runs the three checks.
- **Questions go up, not sideways.** If a brief or the spec is unclear, a developer or the team lead stops and asks. Only the product managers settle a spec question.
- **A subagent's summary is a lead, not a fact.** Check any claim a design choice will rest on against the source before using it.
- **Two developers never edit the same files at the same time.**
- **Small jobs skip the chain.** A change of a few lines is done by whoever is holding it.

## Tests

- Each scenario in the spec gets one file in `tests/` and one TOML file in `data/scenarios/`.
- A scenario test writes out its hand calculation in the file's doc comment, then asserts those numbers exactly.
- Build scenarios in the spec's order. Each adds one subsystem, so a failure points at the newest code.
- Do not start the next scenario until the current one passes and has been reviewed.

## Status

Scenarios 1 (transit), 2 (supply run) and 3 (search), the movement change from decision 0005, and stages 4a, 4a2 and 4b of scenario 4 (the surface combat rule, decision 0014; damage to named systems and stragglers, decision 0016; the real map and pathfinding, decision 0017) are done. Stage 4c1's tool is built (`oob-derive`, decision 0020), and stage 4c2a is done (readiness by order, alertness from belief and events, the alarm roll, speed orders in knots and starting contacts, decision 0021), and so is stage 4c2b (forces move in order of clock time and fight where they meet, inside the movement step; a force that breaks off avoids action for the rest of the pulse, decision 0022). The 20 Savo Island ships are derived (`data/reference/ship_classes_savo_island.csv`, decision 0023, from the settlement sheet `docs/research/oob/savo-island-settlement.md`). The Savo Island scenario itself (stage 4c2, decision 0024) runs with the real ships and closes scenario 4. Its losses are not yet a plausible spread, for reasons decision 0024 lists; the combat figures are placeholders until step 2. Scenario 5, the carrier duel, is built in stages 5a to 5e. Stage 5a is done: air groups aboard ships, aircraft capacity, and search flown from a carrier (decision 0025). Stage 5b is done: the strike decision on belief, an unopposed strike resolved in the same pulse through the damage rules of surface combat, and the flight deck as a fifth ship system (decision 0026). Stage 5c is done: fighters on patrol and escort, anti-aircraft fire, and aircraft losses (decision 0027). Stage 5d is done: two carrier forces search for and strike each other in the same pulse, each on its own belief; it added no rule. A strike that finds nothing drops the contact it was sent at (decision 0028). Stage 5e's figures are derived (`data/reference/ship_classes_eastern_solomons.csv` and `aircraft_types_eastern_solomons.csv`, decision 0029): anti-aircraft shots, aircraft capacity, aircraft types with their radius, load and protection. Escorts go with a raid only within their type's radius, and otherwise stay on patrol (decision 0030). The battle itself (stage 5e, decision 0031) runs with the real ships and aircraft and closes scenario 5: unlike Savo Island its spread has the shape of the day. The complexity review is written (`docs/complexity-review-after-scenario-5.md`); the cuts it led to are made (decision 0032), and the three rule changes the product managers approved (decision 0033) are made: system hits scaled by damage, each shot rolled, and the patrol divided among raids. Scenario 6, air attrition, is built in stages 6a to 6e. Stage 6a is done: the base strike mission, a base's anti-aircraft fire, and hits on aircraft on the ground and on supply (decision 0034). Stage 6b is done: damaged aircraft, a wear roll after each sortie that rises with its distance, a share of the worn written off, and repairs by the day at a base (decision 0035). Stage 6c is done: fatigue for an air group, gained by flying and lost by rest, which cuts the aircraft that fly, and a rest threshold the staff applies each morning (decision 0036). Stage 6c2 is done: a task force that is alert tires each pulse and one that is not rests, and fatigue cuts its chance to sight (decision 0037), which is the cost of battle readiness the complexity review asked for. Stage 6d1 is done: pilots, lost with an aircraft shot down away from home and saved at home, a limit on what flies, and replacements of aircraft and pilots by the day at a base (decision 0038). Stage 6d2 is done: base supply spent for each sortie, each aircraft a day and each repair, and a base out of supply flying nothing (decision 0039), which is the base supply consumption the spec wanted by scenario 6. Air distance is now the straight line between hex centres in whole miles (`Hex::air_distance_nm`, decision 0040): every rule, event and tool that measures how far an aircraft flies uses that one function. Ships still move hex by hex, which is deferred to a decision before step 2. Stage 6e closes scenario 6: aircraft that arrive on a day (decision 0041), four aircraft types derived for Rabaul and Henderson Field (`data/reference/aircraft_types_rabaul_guadalcanal.csv`), and the four weeks themselves with real air groups and every air subsystem running together (decision 0042). Its rhythm of raids has the shape of the record; its losses do not yet, for reasons decision 0042 lists. Scenario 7, determinism, is done (decision 0043): a full-slice scenario that records every kind of event, run twice, in two processes and with its tables in shuffled order, with identical daily hashes; orders are sorted by unit before they are numbered, so their order in a file or a submission does not matter; and the runner has `--save`, `replay` and `hash`. Scenario 8, the 180-day soak, is done (decision 0044): the full slice carried on for 180 days with every subsystem in use in each 30 days, no panic, every invariant holding, and no growth worse than its length. It found that a force turning back recorded no event, which is fixed, and orders to a force with no ship afloat or an air group lost with its ship are now rejected. It also showed that ships could not refuel. Refuelling in port is now built (decision 0045, scenario 1b): a base has a fuel stock, and a force stopped at its own side's base at the start of a day fills up from it, lowest-numbered ship first. Fuel is a cargo too (decision 0046, scenario 2b): a load order names supply or fuel, a ship carries both up to its cargo capacity, and fuel landed joins the base's stock. A ship cannot burn its cargo, and there is no refuelling at sea. The agent interface is built in stages 9a to 9e. Stage 9a is done: what a side may know of what happened (decision 0047), the events a side may see, each whole, cut or absent, proved by games that differ only in what a side cannot know. Stage 9b is done: the daily briefing for a side, the list and detail queries and `explain`, built on a side's view only, as JSON with schemas (decision 0048). A player is shown a side's own event numbers, never the log's. Stage 9c is done: `measure` (decision 0049) answers air distance, sea route, a force's passage with arrival time and fuel, and an air group's reach, by running the engine's own movement, fuel, path and range code, so a forecast and what then happens cannot disagree. Stage 9d is done: playing a side (decision 0050). `sim-cli serve` holds a game open over JSON lines; a seat held by a player gives orders with `issue_orders`, which asks the engine's own check, and ends the day with `end_day`; the saved game holds the orders as handed in and replays with no player; a transcript keeps every call and its answer. A player issuing a scenario's own orders through the tools gives the scripted game event for event. Stage 9e's server is built (decision 0051): `sim-cli mcp` gives the same tools over the Model Context Protocol, in both the current form of the protocol and the older one, and `sim-cli play` makes one call at a time against a game kept as a file, so anything that can run a command can play. Games a model played are kept under `crates/sim-cli/tests/recorded_games/` and a test replays each to the same answers. See the spec's "First test scenarios" table.

The order-of-battle facts cover what scenarios 4 to 6 need: the ships at Savo Island and Eastern Solomons, their classes, weapons and sensors, the aircraft of August and September 1942, and what happened at both battles. `data/oob/README.md` says how firm they are and what is missing.
