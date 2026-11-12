# Roles, delegation and tests

Who does what, how work is handed out and reviewed, and how scenario tests are written. Moved out of `CLAUDE.md` on Nov 12, 2026, word for word, to keep that file short. `CLAUDE.md` states each rule that fails silently in one line and points here for the rest and the reasons.

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
