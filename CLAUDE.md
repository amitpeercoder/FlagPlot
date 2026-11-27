# Flag Plot

An operational wargame of the Pacific War where the player commands by intent and a staff executes. This repo holds step 1, a headless, deterministic simulation core, and step 2, the validation harness, both complete, and step 3, the staff planner, which is being built.

This file is short on purpose. It holds the rules that break something silently if missed, each in one line, and says where everything else is. The pages it points to hold the full rules and the reasons. Read the page for the work in hand before starting it.

If a spec or a rule is ambiguous or looks wrong, stop and ask. Do not guess. A design choice the spec did not make is recorded in `docs/decisions/` as the next numbered file; check `origin/main` and the open pull requests for the next free number first.

## Where to look

| Before you | Read |
| --- | --- |
| Do anything | `docs/pillars.md`: what the game is and the five pillars every feature is tested against |
| Ask what the game does today | `docs/rules/`: every rule as the code has it, in the order a day resolves. `docs/glossary.md`: one meaning for each word |
| Ask why a rule is as it is | `docs/decisions/` (`README.md` is the index). `docs/rules/differences.md`: where a record and the code differ, with a ruling on each |
| Work on step 3, the staff planner | `docs/step3-spec.md`, the source of truth for step 3: the target, the command contract, the stages and what done means |
| Read step 2, or work on what follows it | `docs/step2-spec.md`, the source of truth for step 2. `docs/validation/step2-close.md`: where it ended and what the designer decided. `docs/open-work.md`: what is open and what comes next. `data/validation/`: the measures, and its `README.md` for their rules (no recorded value without the rows it rests on; a range spans a dispute and changes no row's standing). `docs/validation/`: the reports, and `docs/validation/tuning/`: each tuning round's record |
| Ask what step 1 was for, or left out | `docs/step1-spec.md`, and decision 0052 |
| Change or add a rule | `docs/engineering/rule-complexity.md`, `docs/engineering/traceability.md`, and the rule's chapter in `docs/rules/` |
| Add or change state, a random draw, or anything in `sim-core` | `docs/engineering/determinism.md` |
| Write anything a player, an agent or the UI will read | `docs/engineering/truth-and-belief.md`, and decisions 0047 to 0051 |
| Add a crate, a dependency or a file that reads data | `docs/engineering/layout.md` |
| Touch `data/oob`, `data/reference` or `data/derivation`, or use `references/` | `docs/engineering/data-rules.md`, then `data/oob/README.md` |
| Start a working session, or wonder which model does a job or what reviews a change needs | `docs/engineering/running-the-project.md`: the manager, the architect and the developers, the three classes of change (decision 0233), what goes to the architect, and the working practices |
| Build a scenario or its test, brief a developer, or open a pull request | `docs/engineering/roles-and-tests.md`. A test on the frozen figures: `data/scenarios/step2-frozen/README.md`. A scenario written and not to be run yet is kept out of the loops that walk `data/scenarios/*.toml`, as decision 0072 says |
| Run a command and want to know what it does | `docs/commands.md` |
| Think about what is not planned yet | `docs/design/` (intelligence, materials and economy, storytelling and commands, planning an operation, command levels and what each knows and reports, the plan for ground combat: stages 1 to 3 built, and marching, decision 0094, and levels: the game as a ladder of planning decisions, each a scenario with a mission from above, judged over many seeds against the staff's own plan, `levels.md`; a note, nothing built). `docs/rules-comparison.md`: the gaps against another game's manual, triaged by decision 0102 |

## Always

Each of these fails with no error and no failing test if it is missed. The page named in brackets has the rest.

**Determinism** (`docs/engineering/determinism.md`)

- No floating point in `sim-core`: every quantity is a `fixed::Fixed`. Data files give quantities as whole numbers or decimal strings; the loader rejects TOML floats.
- `BTreeMap` and `BTreeSet` only, never the hash collections. No wall clock, thread timing or locale.
- All randomness goes through `World::rng`, keyed on seed, day, pulse, subsystem and entity. A new rule's draws carry their own `rng::Draw` value, added at the end. Never share a stream between entities or carry one across pulses.
- Never renumber a stable code (`Subsystem::code`, `Pulse::code`, `EntityRef::kind_code`, `Side::code`, `Draw`). Add new values at the end.
- A new field of the state goes into `hash::hash_world` in its place and is named in `hash::name_every_field` in the same change; the hash version is raised and the golden files remade (decision 0057).
- Do not change the pinned numbers in `rng.rs` without a decision record. No `unsafe`. Overflow checks stay on.

**Traceability** (`docs/engineering/traceability.md`)

- Every change to truth state records an event, and every event names its causes: the orders and earlier events that led to it.
- Every computed quantity or chance records a breakdown of its factors when breakdowns are on. Recording one never changes what happens or the hash.

**Truth and belief** (`docs/engineering/truth-and-belief.md`)

- A side knows its own units exactly and the enemy only as contacts and claims. The only way to change a belief is `BeliefState::apply_report`.
- Never add a method to `SideView` that returns enemy truth. Anything that plays the game gets a `SideView` and nothing else; `sim-agent` library code never takes the world, the umpire's view or the full log.
- The replay viewer, replay files, batch files, `sim-cli events` and the validation harness are the umpire's: they show the truth. Nothing that plays the game reads them.
- A side gets events only through `SideView::events()`. A new kind of event does not compile until `side_events.rs` says who may see it: decide it, do not default it.

**Crates** (`docs/engineering/layout.md`)

- `sim-core` depends on nothing, not even other workspace crates, and reads no file. Only `sim-data` reads simulation data files, from the disk or from a pack that `sim-data` itself made. The simulation never reads `data/oob`.

**Rules and the rulebook** (`docs/engineering/rule-complexity.md`)

- A pull request that adds a rule, changes one or removes one changes `docs/rules/` and the glossary in the same pull request. One that changes a rule with no change to the rulebook is not merged. A change to a figure in a data file needs none.
- A new rule reuses an existing mechanism where it can. A decision record that adds a rule carries a rule card: the state it reads and the state it writes.
- A rule is stated once: in its record while it is a plan, in `docs/rules/` once built. From record 0233 on, what was built and the rulings on a pull request go in `docs/decisions/as-built/`, not in the record.

**Data** (`docs/engineering/data-rules.md`)

- Do not invent a historical figure. Anything made up is marked `placeholder` and named "Placeholder". No real ship, aircraft or weapon gets an invented rating: its figures are derived from fact rows by a written rule (`oob-derive`), or, where the record has none, estimated from like ships by a written rule that marks the figure as estimated (decision 0207). An estimate is never evidence.
- Nothing from the War in the Pacific: Admiral's Edition database, and nothing from `references/` is ever committed. Its manuals teach mechanics and lessons, not numbers.
- One fact row is one source's statement, citing what was read. A source that disagrees gets its own row; rows are never overwritten. A missing fact is a gap, not a guess; a gap stays open under an estimate.
- Entity IDs in data files are stable. Never renumber or reuse one. Files `oob-derive` generates are never edited by hand.
- Class ids are pinned in each derivation list's `[class_ids]` table, each id with its ships; a new class takes a new id there.

**Tests and numbers** (`docs/engineering/roles-and-tests.md`)

- The hand calculation is written by the architect before any code is delegated, in the test file's doc comment and nowhere else. Nobody else changes an expected number, and never to make a test pass: a mismatch goes back to the architect.
- Tests passing does not make a change housekeeping, and no test is removed, ignored or loosened but on the architect's word. A baseline, pin or recorded game moves only in a pull request the architect reads (decision 0233).
- When figures are tuned (step 2): one set of figures serves every battle and none is set for one scenario, a figure moves only toward something a source supports, a unit's experience value is never changed to improve a report (only the strength of its effect is tuned), and the hand-calculation tests run on the frozen step 1 figures (`data/reference/step1`). `docs/step2-spec.md` has the rules of tuning.
- No test and no build step reads a Markdown file: CI skips its checks when only Markdown changed, so such a test would not run when its input does. If one must, exempt its input in `.github/scripts/needs-checks.sh` first.
- Frozen figures go with frozen scenarios: a test on the frozen figures reads the frozen copy of a scenario in `data/scenarios/step2-frozen/`, never the live file (that folder's `README.md`).
- A subagent's summary is a lead, not a fact. Check any claim a design choice will rest on against the source.

**Git**

- Nobody pushes to `main`. One stage is one branch and one pull request. A pull request is merged only after CI reports success on the pushed commit, and what was merged is reported to Amit.
- Two developers never edit the same files at the same time. More than one session works in this repository: work in a worktree, not the main checkout.

## Before a commit

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
cargo run -p sim-cli -- events data/scenarios/05_eastern_solomons.toml --unit ship:33
cargo run -p sim-cli -- events data/scenarios/05_eastern_solomons.toml --why 133
cargo run --release -p map-tool -- render
cargo run -p oob-check -- data/oob
cargo run -p oob-derive -- data/derivation/savo_island.toml --check
cargo run -p oob-derive -- data/derivation/rabaul_guadalcanal.toml --check
cargo run -p oob-derive -- data/derivation/coral_sea.toml --check
cargo run -p oob-derive -- data/derivation/midway.toml --check
```

The first three must pass before a commit; CI runs them, then runs scenario 1, exports every scenario and runs the viewer's check, unless only Markdown files changed, when it skips them all. So success on a documents-only pull request means nothing was checked. `cargo test` includes the order-of-battle check, so a bad row in `data/oob` fails the build. `docs/commands.md` says what each command does and the options it takes.

## Who decides what

- **Amit, the product manager:** what the game is, the specs, every decision marked "needs review", and the plan for each scenario or rule before it is built.
- **The manager (the main session, Opus, since Nov 23, 2026):** runs the work: what comes next, briefing Sonnet developers and reading what they build, every pull request and its merge, and the report to Amit. It does not write a plan, a statement or an expected number, and does not rule: those go to the architect.
- **A change has a class (decision 0233):** 1, housekeeping, is the manager's alone, and is a change whose every path is on the decision's list (most Markdown under `docs/`, most of `scripts/`, the table's wording). Never on it: a crate's `src` beyond comments, any `Cargo.toml`, `tests/`, `data/`, `.github/`, this file, `docs/decisions/`, `docs/rules/`, `docs/engineering/`, `docs/validation/`, the specs. 2, a bounded change, changes no rule: the architect says what he must be shown and reads the pull request. 3, a rule, a scenario or a figure, has the plan, its independent review, Amit's word and the architect's reading. Above class 1 the architect classifies. In doubt, the higher class.
- **The architect (the most capable model, called by the manager for one job at a time):** settling data, plans, specs, hand calculations, decision records, crate boundaries, rulings, and the last reading of a pull request of class 2 or 3. Use the cheapest model that can do the job well.
- **Data is settled by the architect, not by Amit,** with the reason written on the row, in the rules file or in a decision record. The architect keeps `docs/step2-spec.md` and `data/validation/` and settles what the measures read; the spec session that owned them is archived.
- **Questions go up, not sideways.** A developer who finds a brief or a spec unclear stops and asks. Only the product managers settle a spec question.

`docs/engineering/roles-and-tests.md` has the full table and the rules of delegation. `docs/engineering/running-the-project.md` has who sits where.

## Status

As of Nov 27, 2026, with decision 0233 the latest. Kept to these lines on purpose: `docs/open-work.md` says what is open and next, the index in `docs/decisions/README.md` gives each record's standing, and git has the history. When a stage lands, change the line it makes untrue; do not add one.

- **Steps 1 and 2 are complete.** Step 1 to its spec as amended by decision 0052, which lists what it left out. Step 2 closed on Nov 13, 2026 (`docs/validation/step2-close.md`): of 62 judged measures over four battles 43 agree, 8 agree at the edge and 11 are named misses.
- **The rules as they stand are in `docs/rules/`,** and `docs/rules/differences.md` lists where a record and the code differ, with a ruling on each. Built since step 2, beyond the staff: ground combat's stages 1 to 3 with marching, landings, cover, hunger and a fight's cost in supply; ship repair; refuelling in port and, on placeholder ships, at sea; forces and stock that arrive from off the map; submarines against warships; the 39 days of the full slice.
- **Step 3, the staff, is built through stage 3 and most of stage 4.** Stages 0 and 1 are built (decisions 0100, 0107 and 0111); stages 2, 2b and 3 are closed by the designer, each with its named misses (decisions 0128, 0138, 0153 and 0159's proof). Stage 4, the trial (decision 0161), is built through step 8 with its mends (0162, 0163, 0169, 0230, 0231); its proof, step 9, is set up (`docs/validation/stage-4-proof-setup.md`) and not run. A plan the commander writes (decision 0232) has step 1 built.
- **The table** (`table/`, decision 0154 on) plays a real game through one seat's tools, and builds for a browser without a server (decision 0229, stages 1 to 4 merged). Three scenarios for play are built: the Eastern Solomons over five days, the Coral Sea and Midway (decisions 0184, 0188 and 0198).
- **Next:** stage 4's proof, run once by the architect; then steps 2 and 3 of decision 0232, the enemy's approaches (decision 0161, step 10) and the levels of `docs/design/levels.md`. Beside them the clean-up of decision 0233 is in progress (`docs/open-work.md`, "The clean-up in progress").
- **Most figures are still placeholders,** each with a note on what it rests on (`docs/validation/tuning/`); every point value is one, and the objectives scale is set from the record before step 3's stage 5. Experience for air groups waits on research (decision 0064).
- **The order-of-battle facts** (`data/oob`) are unsettled until a derivation reads them; `data/oob/README.md` says what they cover, how firm they are and what is missing.
