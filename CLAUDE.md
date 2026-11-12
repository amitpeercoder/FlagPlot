# Flag Plot

An operational wargame of the Pacific War where the player commands by intent and a staff executes. This repo holds step 1, a headless, deterministic simulation core, which is complete, and step 2, the validation harness, which is under way.

This file is short on purpose. It holds the rules that break something silently if missed, each in one line, and says where everything else is. The pages it points to hold the full rules and the reasons. Read the page for the work in hand before starting it.

If a spec or a rule is ambiguous or looks wrong, stop and ask. Do not guess. A design choice the spec did not make is recorded in `docs/decisions/` as the next numbered file; check `origin/main` and the open pull requests for the next free number first.

## Where to look

| Before you | Read |
| --- | --- |
| Do anything | `docs/pillars.md`: what the game is and the five pillars every feature is tested against |
| Ask what the game does today | `docs/rules/`: every rule as the code has it, in the order a day resolves. `docs/glossary.md`: one meaning for each word |
| Ask why a rule is as it is | `docs/decisions/` (`README.md` is the index). `docs/rules/differences.md`: where a record and the code differ, with a ruling on each |
| Work on step 2 | `docs/step2-spec.md`, the source of truth for step 2. `docs/step2-opening-list.md`: what is to be decided and built first. `data/validation/`: the measures, and its `README.md` for their rules (no recorded value without the rows it rests on; a range spans a dispute and changes no row's standing) |
| Ask what step 1 was for, or left out | `docs/step1-spec.md`, and decision 0052 |
| Change or add a rule | `docs/engineering/rule-complexity.md`, `docs/engineering/traceability.md`, and the rule's chapter in `docs/rules/` |
| Add or change state, a random draw, or anything in `sim-core` | `docs/engineering/determinism.md` |
| Write anything a player, an agent or the UI will read | `docs/engineering/truth-and-belief.md`, and decisions 0047 to 0051 |
| Add a crate, a dependency or a file that reads data | `docs/engineering/layout.md` |
| Touch `data/oob`, `data/reference` or `data/derivation`, or use `references/` | `docs/engineering/data-rules.md`, then `data/oob/README.md` |
| Build a scenario or its test, brief a developer, or open a pull request | `docs/engineering/roles-and-tests.md` |
| Run a command and want to know what it does | `docs/commands.md` |
| Think about what is not planned yet | `docs/design/` (intelligence, materials and economy) |

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

- `sim-core` depends on nothing, not even other workspace crates, and reads no file. Only `sim-data` reads simulation data files. The simulation never reads `data/oob`.

**Rules and the rulebook** (`docs/engineering/rule-complexity.md`)

- A pull request that adds a rule, changes one or removes one changes `docs/rules/` and the glossary in the same pull request. One that changes a rule with no change to the rulebook is not merged. A change to a figure in a data file needs none.
- A new rule reuses an existing mechanism where it can. A decision record that adds a rule carries a rule card: the state it reads and the state it writes.

**Data** (`docs/engineering/data-rules.md`)

- Do not invent a historical figure. Anything made up is marked `placeholder` and named "Placeholder". No real ship, aircraft or weapon gets an invented rating: its figures are derived from fact rows by a written rule (`oob-derive`).
- Nothing from the War in the Pacific: Admiral's Edition database, and nothing from `references/` is ever committed. Its manuals teach mechanics and lessons, not numbers.
- One fact row is one source's statement, citing what was read. A source that disagrees gets its own row; rows are never overwritten. A missing fact is a gap, not a guess.
- Entity IDs in data files are stable. Never renumber or reuse one. Files `oob-derive` generates are never edited by hand.

**Tests and numbers** (`docs/engineering/roles-and-tests.md`)

- The hand calculation is written by the architect before any code is delegated, in the test file's doc comment. Nobody else changes an expected number, and never to make a test pass: a mismatch goes back to the architect.
- When figures are tuned (step 2): one set of figures serves every battle and none is set for one scenario, a figure moves only toward something a source supports, a unit's experience value is never changed to improve a report (only the strength of its effect is tuned), and the hand-calculation tests run on the frozen step 1 figures (`data/reference/step1`). `docs/step2-spec.md` has the rules of tuning.
- No test and no build step reads a Markdown file: CI skips its checks when only Markdown changed, so such a test would not run when its input does. If one must, exempt its input in `.github/scripts/needs-checks.sh` first.
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
```

The first three must pass before a commit; CI runs them, then runs scenario 1, exports every scenario and runs the viewer's check, unless only Markdown files changed, when it skips them all. So success on a documents-only pull request means nothing was checked. `cargo test` includes the order-of-battle check, so a bad row in `data/oob` fails the build. `docs/commands.md` says what each command does and the options it takes.

## Who decides what

- **Amit, the product manager:** what the game is, the specs, every decision marked "needs review", and the plan for each scenario or rule before it is built.
- **The architect (the main session):** settling data, plans, hand calculations, decision records, crate boundaries, and the final review and merge of each pull request. Developers are Sonnet subagents; use the cheapest model that can do the job well.
- **Data is settled by the architect, not by Amit,** with the reason written on the row, in the rules file or in a decision record. A second session owns `docs/step2-spec.md` and `data/validation/` and settles the rows its measures read.
- **Questions go up, not sideways.** A developer who finds a brief or a spec unclear stops and asks. Only the product managers settle a spec question.

`docs/engineering/roles-and-tests.md` has the full table and the rules of delegation.

## Status

Kept short on purpose. The history of how each stage was built is in the decision records (`docs/decisions/README.md` is the index) and in git; do not grow this section back into a narrative.

- **Step 1 is complete** against the spec as amended by decision 0052, which lists what it leaves out (ground combat, land units, shore bombardment, ship repair, base damage, crew and pilot quality, among others). All nine scenarios pass; scenario 9's recorded games are under `crates/sim-cli/tests/recorded_games/`.
- **The rules as they stand are in `docs/rules/`.** `docs/rules/differences.md` lists where a decision record and the code differ, with a ruling on each; nine small code fixes it calls for are not made yet.
- **Step 2 is under way** to `docs/step2-spec.md`. Another session owns the spec and the measures files (`data/validation/`); the build is planned from `docs/step2-opening-list.md`. Ships are charged straight-line miles (decision 0058) and the scenarios are re-derived for it. Before any figure is tuned: the step 1 figures frozen for the hand-calculation tests, and three rules (strike target priority as an order, reach beyond the radius with ditching, experience), each with its own decision record.
- **Every combat, wear, fatigue, supply and claim figure, and every point value, is a placeholder** until step 2 sets it from the record. The loss spreads of Savo Island and of the four weeks over Guadalcanal are known not to match history yet (decisions 0024, 0042).
- **The order-of-battle facts** (`data/oob`) cover Savo Island, the Eastern Solomons, the air campaign of August and September 1942 and strike launch ranges; research on Cape Esperance, ditching and crew recovery, and training is arriving in its own pull requests. Rows are unsettled until a derivation reads them (a measure takes a range across a dispute and settles nothing); `data/oob/README.md` says how firm they are and what is missing.

When a stage lands, change the line it makes untrue. Do not add a line for it.
