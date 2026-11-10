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
```

All three checks must pass before a commit. CI runs the same three.

## Layout

| Path | Contents |
| --- | --- |
| `crates/sim-core` | State, rules, resolution, event log. No dependencies. No file or network access |
| `crates/sim-data` | Loads reference data, maps and scenarios from files into `sim-core` types |
| `crates/sim-cli` | Command-line runner |
| `crates/sim-agent` | Agent interface (server mode, briefings, MCP). Placeholder |
| `data/reference` | Ship classes as CSV |
| `data/map` | Maps as TOML |
| `data/scenarios` | One TOML file per scenario: starting state, seed, scripted orders |
| `tests` | One test file per scenario, each with its hand calculation written out |

Crate boundaries:

- `sim-core` depends on nothing, not even other workspace crates. Keep it that way.
- Only `sim-data` reads simulation data files.
- `sim-cli` and the tests are the umpire. They may read truth through `World::umpire()`.
- Anything that plays the game (the staff planner, an agent, the UI) gets a `SideView` and nothing else.

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

## Truth and belief

- Truth state is the real world. Belief state is what one side thinks.
- A side's own units are known exactly. Enemy units reach a side only as contacts.
- The only way to change a belief state is `BeliefState::apply_report`. Keep it that way: it is what makes "belief is written only from reports" true.
- Never add a method to `SideView` that returns enemy truth.

## Data rules

- **Do not invent historical figures.** Anything made up is marked `placeholder` in its file and uses a name starting with "Placeholder".
- **Nothing from the War in the Pacific: Admiral's Edition database.** Real data comes from open historical sources, with the source recorded.
- Entity IDs in data files are stable. Never renumber or reuse one.

## Tests

- Each scenario in the spec gets one file in `tests/` and one TOML file in `data/scenarios/`.
- A scenario test writes out its hand calculation in the file's doc comment, then asserts those numbers exactly.
- Build scenarios in the spec's order. Each adds one subsystem, so a failure points at the newest code.
- Do not start the next scenario until the current one passes and has been reviewed.

## Status

Scenario 1 (transit) is done. Next is scenario 2 (supply run). See the spec's "First test scenarios" table.
