# Flag Plot

An operational wargame of the Pacific War where the player commands by intent and a staff executes. This repo currently holds step 1: a headless, deterministic simulation core.

Read these before changing anything:

- `docs/pillars.md`: what the game is and the five pillars every feature is tested against.
- `docs/step1-spec.md`: the spec for the simulation core. It is the source of truth.
- `docs/decisions/`: choices the spec left open, one short file each.

If the spec is ambiguous or looks wrong, stop and ask. Do not guess. If you make a design choice the spec did not make, record it in `docs/decisions/` as the next numbered file.

## Commands

```
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo run -p sim-cli -- run data/scenarios/01_transit.toml --events
cargo run -p sim-cli -- export data/scenarios/04a_night_action.toml --out viewer/replays/04a_night_action.json
cargo run --release -p map-tool -- render
cargo run -p oob-check -- data/oob
```

`export` runs a scenario exactly as `run` does and writes it as one replay file for the viewer (decision 0018); it also takes `--seed N` and `--labels data/map/south_pacific_anchors.csv`. Open `viewer/index.html` in a browser and pick the file. `node viewer/check.js viewer/replays/*.json` checks replays against the page's own rebuild.

The map tool also has `locate --lat --lon`, `path --from q,r --to q,r`, `sides --hex q,r` (a hex's closed sides and neighbours), `water --lat --lon` (why a position's hex has the water it has) and `generate`, which rebuilds `data/map/south_pacific.csv` from the Natural Earth files under `references/` (the exact command is in the file's header). `generate --anchors data/map/south_pacific_anchors.csv` applies the listed anchorages (decision 0017, decision 6); without it hexes keep their largest body of water. `render` draws `#` land, `+` coast, `%` coast with a closed side toward water and `.` sea, each row labelled with its r and the q of its first hex.

All three checks must pass before a commit. CI runs the same three. `cargo test` includes the order-of-battle check, so a bad row in `data/oob` fails the build; `oob-check` run by hand lists the problems.

## Layout

| Path | Contents |
| --- | --- |
| `crates/sim-core` | State, rules, resolution, event log. No dependencies. No file or network access |
| `crates/sim-data` | Loads reference data, maps and scenarios from files into `sim-core` types |
| `crates/sim-cli` | Command-line runner |
| `crates/sim-agent` | Agent interface (server mode, briefings, MCP). Placeholder |
| `crates/map-tool` | Generates the hex map of the slice from open coastline data, and renders it, locates points and finds sea paths. May use floats and its own dependencies; nothing depends on it |
| `crates/oob-check` | Checker for the order-of-battle facts in `data/oob`. Depends on nothing else in the workspace |
| `data/reference` | Ship classes, port sizes and aircraft types as CSV; search and surface combat figures as TOML |
| `data/map` | The generated map of the slice as CSV (`south_pacific.csv`, decision 0017) and the anchorages it was generated with (`south_pacific_anchors.csv`), and small hand-drawn placeholder maps as TOML |
| `data/scenarios` | One TOML file per scenario: starting state, seed, scripted orders |
| `data/oob` | Order-of-battle facts: sourced historical statements as CSV. Research, not simulation data. Start with its `README.md` |
| `docs/research/oob` | The facts format, the source policy, where sources disagree, and a note per battle and per topic |
| `tests` | One test file per scenario, each with its hand calculation written out |
| `viewer` | The read-only replay viewer (decision 0018): one HTML page with no build step, its check script, and `replays/` (git-ignored) for exported runs |

Crate boundaries:

- `sim-core` depends on nothing, not even other workspace crates. Keep it that way.
- Only `sim-data` reads simulation data files. The simulation never reads `data/oob`; only `oob-check` does.
- `sim-cli` and the tests are the umpire. They may read truth through `World::umpire()`.
- Anything that plays the game (the staff planner, an agent, the UI) gets a `SideView` and nothing else.
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

## Data rules

- **Do not invent historical figures.** Anything made up is marked `placeholder` in its file and uses a name starting with "Placeholder".
- **Nothing from the War in the Pacific: Admiral's Edition database.** Real data comes from open historical sources, with the source recorded.
- Entity IDs in data files are stable. Never renumber or reuse one.

### Order-of-battle facts

`data/oob` holds what sources say about real ships, aircraft, weapons and battles. `data/reference` holds the figures the simulation uses. Decision 0015 and `docs/research/oob/database-design.md` give the full rules. The ones that matter most:

- **No real ship, aircraft or weapon gets an invented rating.** A game figure for a real entity is derived from fact rows by a written rule, and can name the rule and the rows. Until that derivation exists, figures in `data/reference` stay placeholders.
- **One row is one source's statement.** It carries its source, where in the source, the date read, a confidence and a standing. A missing fact is a missing row, listed in `gaps.csv`. Nothing is guessed.
- **Cite what was read.** A figure a website quotes from a book cites the website. `docs/research/oob/source-policy.md` says which sources may be used.
- **A source that disagrees gets its own row.** Rows are never overwritten to settle a disagreement. A person settles it and writes the reason on the row.
- **A value has to earn `preferred`.** A lone weak row is `unconfirmed`, and a rule may read it only if the rule says so.
- **Engagement rows are history to test against.** A scenario may take starting positions and alert states from them. Its result is compared with the recorded outcome and is never scripted to match it.
- **Run `oob-check` after any change to `data/oob`.**

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
| Product manager | Amit, with the main session | What the game is, the spec, every decision marked "needs review", approval of each scenario plan |
| Architect | The main session (the most capable model) | The plan for each scenario, crate boundaries, the determinism and traceability rules, the hand calculation, decision records, final review before a pull request |
| Team lead | An Opus subagent | Turning an approved plan into tasks, briefing the developers, reviewing their work against the plan and this file, reporting back what was built, deferred and unclear |
| Developers | Sonnet subagents | Implementation tasks, tests, bulk reading and summarising of reference material, code search |

Rules:

- **Use the cheapest model that can do the job well.** Haiku is enough for plain lookups.
- **The hand calculation is written by the architect before any code is delegated.** Developers make the code match it. Nobody changes an expected number to make a test pass; a mismatch goes back to the architect.
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

Scenarios 1 (transit), 2 (supply run) and 3 (search), the movement change from decision 0005, and stages 4a, 4a2 and 4b of scenario 4 (the surface combat rule, decision 0014; damage to named systems and stragglers, decision 0016; the real map and pathfinding, decision 0017) are done. Next is stage 4c, Savo Island. See the spec's "First test scenarios" table.

The order-of-battle facts cover what scenarios 4 to 6 need: the ships at Savo Island and Eastern Solomons, their classes, weapons and sensors, the aircraft of August and September 1942, and what happened at both battles. `data/oob/README.md` says how firm they are and what is missing.
