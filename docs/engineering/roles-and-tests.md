# Roles, delegation and tests

Who does what, how work is handed out and reviewed, and how scenario tests are written. Moved out of `CLAUDE.md` on Nov 12, 2026, word for word, to keep that file short. `CLAUDE.md` states each rule that fails silently in one line and points here for the rest and the reasons.

## Roles and delegation

| Role | Who | Owns |
| --- | --- | --- |
| Product manager | Amit | What the game is, the spec, every decision marked "needs review", approval of each scenario plan. Not data: see "Who settles data" |
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
- A chance worked by hand over several draws (at least one hit in n shots, a fight that never opens, a bomber that is never hit) is written cut to six decimal places, as the engine's fixed point cuts, never rounded: 0.95^20 = 0.358485922… is written 0.358485. Ruled by the coordinating architect on Nov 13, 2026, after the rulebook-only test (`docs/validation/closing-tests.md`, finding R6). Doc comments written before then that round such a chance are not edited (nobody changes an expected number, and none of them is asserted); that record lists them.
- Build scenarios in the spec's order. Each adds one subsystem, so a failure points at the newest code.
- Do not start the next scenario until the current one passes and has been reviewed.
- **The hand-calculation tests run on the frozen step 1 figures,** `data/reference/step1`, for the length of step 2 (`docs/step2-spec.md`, "Tests while figures move"). A scenario test loads its scenario through `tests/common` (`common::load`, `common::from_text`, and `common::altered_reference` for a variant with a figure changed), and the agent's and the runner's tests through `LoadOptions::frozen_step1`, so the directory is named once, as `sim_data::FROZEN_REFERENCE_DIR`. A test never rewrites a scenario's text to point at another figures file. What passes there proves the rules compute what the hand calculation says, on known figures; it says nothing about the figures in use. A rule that adds a figure adds it to the frozen file too, with its first value (`crates/sim-data/tests/frozen_figures.rs` fails if it does not). The run baselines (`crates/sim-cli/tests/run_baseline`), `batch` and the recorded games follow the live figures and are remade with each tuning round; so do the tests that compare a run with a baseline, and the determinism scenario's and the soak's checks that a run repeats, completes and holds its invariants, so that tuned figures are held to them.
