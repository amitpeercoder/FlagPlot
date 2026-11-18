# Layout and crate boundaries

Where things are in the repository and which crate may depend on which. Moved out of `CLAUDE.md` on Nov 12, 2026, word for word, to keep that file short. `CLAUDE.md` states each rule that fails silently in one line and points here for the rest and the reasons.

| Path | Contents |
| --- | --- |
| `crates/sim-core` | State, rules, resolution, event log. No dependencies. No file or network access |
| `crates/sim-data` | Loads reference data, maps and scenarios from files into `sim-core` types |
| `crates/sim-cli` | Command-line runner. Every runner takes its staffs from `sim_staff::staffs()` (decision 0107) |
| `crates/sim-agent` | Agent interface: the daily briefing, the list and detail queries, `explain`, `measure`, and the session a seat plays through (`issue_orders`, `end_day`), as JSON with schemas in `schemas/` (decisions 0048 to 0050). Every tool takes a `SideView`; only `session.rs` holds the world. The MCP server that wraps it is in `sim-cli` (decision 0051) |
| `crates/sim-staff` | The staff and its doctrine (decision 0107, step 3, stage 1): `SupplyStaff`, the doctrine for keeping a base supplied, its projection of days of supply and its proposals; `DOCTRINE_VERSION`; `staffs()`, the staff of each side every runner passes. Depends on `sim-core` only. Forbids floating point, as `sim-core` does, because its decisions reach the hash. Every function in it takes a `SideView` (with plain numbers) and nothing else; `tests/source.rs` reads its source to check, and bans the world and `at_staff_step` by name |
| `crates/map-tool` | Generates the hex map of the slice from open coastline data, and renders it, locates points and finds sea paths. May use floats and its own dependencies. Nothing but the `tests` crate depends on it, as a dev-dependency (scenario 4b's test places the anchors with its projection) |
| `crates/oob-check` | Checker for the order-of-battle facts in `data/oob`. Depends on nothing else in the workspace |
| `crates/oob-derive` | Derives ship class figures for a list of real ships from `data/oob` and `data/derivation` (decision 0020). Depends only on `oob-check`; `sim-data` and `sim-core` only in its tests |
| `data/reference` | Ship classes, port sizes and aircraft types as CSV; search, surface combat and air combat figures as TOML. Files `oob-derive` generates (a ship class file `<name>.csv` with `<name>.sources.csv` and `<name>.ships.csv`, and an aircraft type file with its own `.sources.csv`) say so in their header and are never edited by hand |
| `data/derivation` | The derivation rules and constants (`rules.toml`) and one ship list per scenario, for `oob-derive` |
| `data/map` | The generated map of the slice as CSV (`south_pacific.csv`, decision 0017) and the anchorages it was generated with (`south_pacific_anchors.csv`), and small hand-drawn placeholder maps as TOML |
| `data/scenarios` | One TOML file per scenario: starting state, seed, scripted orders |
| `data/oob` | Order-of-battle facts: sourced historical statements as CSV. Research, not simulation data. Start with its `README.md` |
| `docs/research/oob` | The facts format, the source policy, where sources disagree, and a note per battle and per topic |
| `tests` | One test file per scenario, each with its hand calculation written out |
| `scripts` | Commands for working on the repository, not parts of the game: shell scripts run from the top of the repository, each described in `docs/commands.md` |
| `viewer` | The read-only replay viewer (decision 0018): one HTML page with no build step, its check script, and `replays/` (git-ignored) for exported runs |
| `table` | A prototype of the player's interface (decision 0154): a bridge that serves one seat's tools to a browser on this machine, and one HTML page with no build step. It plays the game: it is given a seat's tools and the map file and nothing else |

Crate boundaries:

- `sim-core` depends on nothing, not even other workspace crates. Keep it that way.
- Only `sim-data` reads simulation data files. The simulation never reads `data/oob`; only `oob-check` and `oob-derive` do.
- `sim-cli` and the tests are the umpire. They may read truth through `World::umpire()`.
- Anything that plays the game (the staff planner, an agent, the UI) gets a `SideView` and nothing else.
- The staff is passed in. `sim-core` defines the `Staff` trait (given a side's view, it hands in orders, staff commands and reasons) and the function that resolves a day takes each side's staff or none (`Staffs`), so `sim-core` still depends on nothing. Every loop that resolves days passes the staffs it is given: the session, `run`, `batch`, `export`, `hash`, `events`, `time` and `replay --check-staff`. `sim-agent`'s session is handed its staffs by its caller and still depends on `sim-core` only. `sim_staff::staffs()` is the one place a runner gets its staffs, and `sim_staff::DOCTRINE_VERSION` the doctrine version (decision 0107). `sim-agent` may have `sim-staff` as a dev-dependency, for its tests only. Its session alone takes `World::at_staff_step()`, the copy of the world at the step a staff is asked, and hands the staff and the tools only a `SideView` of it.
- `sim-agent` depends on `sim-core` only. Its library code takes a `SideView` and never the world, the umpire's view or the full event log; a test reads its source to check.
- The replay viewer (`viewer/`) is an umpire's tool like `sim-cli`: it shows the truth. Nothing that plays the game may use it or the replay files.
- The table (`table/`) plays the game. It reads the answers of one seat's tools and the map file, and never a replay, a save, a batch file, the event log or a scenario file (decision 0154).
- `sim-cli validate` (`crates/sim-cli/src/validate/`) reads `data/validation` and a batch file, and writes reports under `docs/validation/`. It is the umpire's tool and shows the truth: nothing that plays the game may depend on it or read its reports. A test reads `sim-agent`'s manifest to check.
- `sim-cli validate --passages` (`crates/sim-cli/src/validate/passages.rs`) reads `data/oob/place_positions.csv` for the positions of the places a passage names, and nothing else under `data/oob`. It is the umpire's tool and not the simulation: the simulation still never reads `data/oob`. It reads the file with a small reader of its own (the `csv` crate, four columns), not `oob-check`'s, so `sim-cli` stays free of that crate.
