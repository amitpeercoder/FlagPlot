# 0233: Ceremony by risk, and where things are written

Status: ruled by the architect on Nov 27, 2026, on Amit's delegation of that day. He was shown the architect's review of the repository's upkeep (five refactorings, 1 to 5, and eight changes to how the work is done, A to H), approved all of it, and said: "Let the architect rule where and when such needed. Have Sonnet agents do the development. Have the architect as a final reviewer. Do all that's needed (all items)." This record rules A, B and C and notes D. Amended on an independent review the same day (`as-built/`). The delegation covers those items and the programme in `docs/open-work.md`; it does not cover what the game is.

Summary: A change is one of three classes, and the roles and reviews it needs follow from its class: housekeeping, told by the paths it touches; a bounded change, which changes no rule; or a rule. From this record on a decision record holds the why, what was weighed, the rule card and the rulings; a rule is stated once, in the record while it is a plan and in the rulebook once built; the arithmetic is in the test's doc comment; what was built and the rulings on a pull request go in a file beside the record. The day hashes leave the 252 run baselines for one file of pins, so a baseline changes only when a game does. No rule of the game changes.

Standing: Ruled by the architect on Nov 27, 2026, on Amit's delegation; amended on an independent review the same day; in force for records and pull requests from this one on; C to be built

## Why

Every change paid for the costliest kind. A record restated its rule, the rulebook stated it again and the test a third time, and grew by an "As built" and a set of rulings for each pull request: the three copies drifted and a record ran to hundreds of lines. A plan for a wording mend had the same reviews as a new rule. A state layout raised by one field rewrote 252 files, so a reader could not see whether a game had moved.

## What was weighed

- **Rewrite the old records to the new shape.** Not done: 232 records are cited by section name from code, tests and reports, and a rewrite is a risk for no gain. The shape binds records from 0233 on.
- **Two classes, not three.** Not done: the saving is in the middle, a change to what the game does that adds no state, draw or event.
- **Let the manager classify everything.** Not done: the manager does not rule. It may say a change is housekeeping by the paths it touches, which anyone can check; above that the class is the architect's.
- **Tell housekeeping by the golden files passing.** Written so at first and withdrawn on review: the rules under "Always" in `CLAUDE.md` are the ones no test catches, so a hash collection, a float on a path no scenario runs or a method on `SideView` would pass.
- **Let a bounded change alter a rule.** Not done: Amit sees the plan of a rule before it is built, and this record takes nothing from that.
- **Drop the day hashes from the baselines and pin only the last day.** Not done: the soaks have no events baseline, and their days' hashes are what is compared.

## A: what is written where

1. **A record holds:** the header; why; what was weighed and not chosen; the rule card (the state read and written, the events and who sees them, the versions moved); the rulings a build cannot make for itself; what it leaves out; and the questions for the designer, if any. It names its test file and its rulebook section. It does not hold a hand calculation.
2. **A rule is stated once.** While it is a plan it is in the record, under "The rule as planned". The pull request that builds it moves that text to its section of `docs/rules/` and leaves in the record one line for each run of rules, naming the section they went to. Built in steps, each step moves its own rules. The rulebook stays what the code does.
   **A rule keeps its number for life.** In the rulebook each moved rule carries its tag, `[NNNN, rule N]`, as the rulebook's sections carry their decisions today. So "decision NNNN, rule N" in code, in a test's doc comment or in a report goes on pointing at one rule, and nothing is cited again when the text moves. A rule changed later keeps its tag and gains the later record's; a number is never used twice.
3. **The statements and the hand calculation are in the test file's doc comment,** as now, written by the architect before code is delegated. They cite a rule by its record and number, or by its rulebook section once built, and do not restate it.
4. **What was built and what was ruled on it go beside the record,** in `docs/decisions/as-built/NNNN-the-records-name.md`. It opens `# NNNN, as built: Title` and is added to at the end, never rewritten, in sections headed by kind, date and pull request: "Built: step 2 (Nov 28, 2026, pull request 750)", "The architect's rulings on pull request 750 (Nov 28, 2026)", "Amended on review (date)". A build's section says what was built, the versions and games that moved, and the readings it asks the architect to rule on. The record itself is kept as the plan in force: an amendment changes its text, and the file beside it says what changed and why.
5. **Records before this one keep their form.** A new section for one of them (a later step built, a later ruling) goes in its file under `as-built/`, with one line in the record's Status pointing there.
6. **Status and Standing stay short:** who ruled or confirmed and when, and what is built. The index is made from them as before.

## B: three classes of change

| Class | What it is | Needs |
| --- | --- | --- |
| 1, housekeeping | A change whose every changed path is on the list in item 7 | The manager decides and briefs; a developer builds; the manager reads and merges. No record, no call to the architect |
| 2, a bounded change | A change that is not class 1 and changes no rule: no rule's substance, no state, draw, event kind, order or tool kind, nothing in who knows what, no scenario of record or for play, no figure, and no game of a closed proof or a validation measure moved. It is a mend that makes the code or the rulebook do what a record already says; a change to what a tool, the export or the table gives, with the rules as they are; a loader or tool change with a stated equality to check; a refactoring of a crate's `src` with every hash as it is; or work on tests, CI, scripts or documents that the list in item 7 leaves out | The architect says it is class 2 and what he must be shown. Where it changes what the game or a tool does, a short record by him, or a section in the `as-built/` file of the record it mends, with the statements and any expected number first; where it changes nothing the game does, no record. No independent review of a plan and no confirmation by Amit. A developer builds; the manager reads; the architect reads the pull request before it is merged |
| 3, a rule | A new rule, a changed one or a removed one; new state, a draw, an event kind, an order or tool kind; anything in truth and belief or in what is hashed beyond a pure move; a crate boundary or a dependency; a new scenario of record or for play, or a change to one; a figure tuned; a change that moves a closed proof's games or a validation measure; data settled between sources; anything that changes what the game is | As before this record: a plan in a record by the architect, an independent review and the amendment, Amit's word as item 13 has it, the hand calculation before code, the rulebook in the same pull request, and the architect's reading of the pull request before it is merged |

7. **Class 1 is told by paths, not by judgement.** A change is class 1 only if every path it adds, changes or removes is one of these:
   - a Markdown file under `docs/` that is not in `docs/decisions/`, `docs/rules/`, `docs/engineering/` or `docs/validation/`, and is not a spec (`docs/step*-spec.md`), `docs/pillars.md`, `docs/glossary.md` or `docs/open-work.md`;
   - `README.md` at the top, and `docs/decisions/README.md` where the only change is the table `scripts/decisions-index.sh` writes;
   - a file under `scripts/` other than `regenerate.sh`, `compare-scenarios.sh` and `decisions-index.sh`;
   - `table/index.html`, for wording inside the table's design rules, and `table/README.md`.

   Everything else goes to the architect, and by name: any line that is not a comment under a crate's `src`; any `Cargo.toml` or `Cargo.lock`; anything under `tests/` or a crate's `tests/`, golden files, pins and recorded games among them; anything under `data/`; anything under `.github/`; `CLAUDE.md`; and the documents the first line of the list leaves out. A change that is only comments under a crate's `src` is class 1 if `cargo fmt` and the build pass.
8. **Above class 1 the architect classifies.** The manager calls with the change and the files, not with a class. The architect's answer names the class, what he must be shown, and the record if there is one; all three are written in the pull request's description.
9. **In doubt, the higher class.** In doubt whether a path is on the list, it is not; in doubt whether something is a ruling, it is.
10. **A class can rise and never falls during a build.** A developer or the manager who finds a change reaching past its class (a path off the list, a golden file that moves, an event wanted, a statement that does not hold) stops and goes back to the architect.
11. **A baseline, pin or recorded game never moves in class 1,** since none is on the list. Pins are remade only inside the pull request that raised the state layout or a version, which is class 2 or 3 by what it changes; the manager may do the remaking there. Until C is built a baseline cannot show whether a game moved or only its hashes, so until then every changed baseline is shown to the architect with `scripts/compare-scenarios.sh`'s result.
12. **What no class relaxes:** nobody but the architect writes or changes a statement or an expected number; no test is removed, ignored or loosened and no check dropped but on the architect's word; a change to a rule changes the rulebook in the same pull request; determinism, traceability, truth and belief and the data rules; CI green before a merge; every merge reported to Amit, with its class.
13. **Amit gives up nothing he saw before a build.** What the game is, a spec, any record marked "needs review", and the plan for each scenario or rule are all class 3, and reach him as they did. Class 2 changes no rule and no scenario, so what he now learns only from the report of what was merged is: mends to what a record already says, changes to what a tool or the table gives with the rules as they are, refactorings and plumbing. If he wants any of those first, he says so and it is class 3.
14. **The class of what is in flight.** The move to one test binary a crate with the change to CI (D) touches `Cargo.toml` files, `tests/` and `.github/`: it is class 2 with no record of its own, and the architect reads its pull request before it is merged. He must be shown that the names of the tests run are the same before and after (`cargo test --workspace -- --list`, sorted and compared), that none is ignored or removed, that `needs-checks.sh` skips the checks for the same changes as before, and that a pull request's pushed commit still gets the three checks. The split of rules chapter 8 touches `docs/rules/`: class 2 with no record, and the architect is shown that no sentence of a rule was changed, only moved.

## C: the day hashes of the run baselines

Today each of the 252 files under `crates/sim-cli/tests/run_baseline` holds its run's day hashes, so a raise of the state layout rewrites all of them. From this build:

15. **A baseline holds no hash.** In every line of the form `day N  hash H  K events` the sixteen hex digits are written as sixteen hyphens. Everything else in the file is `sim-cli run`'s output byte for byte, and `run` itself prints as before.
16. **The hashes are pinned in `tests/golden/pinned_hashes.txt`,** the file that already holds the hashes a scenario test pins, one `name hexhash` line each. It gains a second kind of line, one a scenario: `run_<stem>` and then the hash of each day in order, sixteen hex digits each, separated by one space. The stem is the scenario's file name without `.toml`: the plain run and the run with `--events` give the same hashes and share the one line, so no name holds `.events`. Names that begin `run_` are kept for these lines; the `run_` lines stand after the others, in the order of their names. The file's heading comment is rewritten to say both kinds and who writes each.
    - **Two writers, each keeping the other's lines.** `scripts/regenerate.sh` writes the `run_` lines in its step 1, from the runs it makes the baselines from; `common::pinned_hash` writes a test's line in step 4, as now. Each replaces or adds only lines of its own kind and leaves every other line, comments included, byte for byte.
    - **A scenario that is gone** loses its `run_` line in the step that removes its baselines, and the test fails on a `run_` line with no scenario file, as on a scenario with no line.
    - **`crates/sim-cli/tests/export.rs` is another package** and cannot use `tests/common`. It reads the file by its path from the top of the repository (the `ROOT` it already has) with a reader of its own for `run_` lines; `tests/common` gains `pinned_day_hashes(stem)` for the scenario tests. The line's form is stated once, in the file's heading.
17. **`run_output_matches_the_golden_files` makes two comparisons** and says which failed: the output with its hashes hyphened against the baseline (a game moved), and the day hashes against the pinned line (the state or its layout moved). A scenario with no pinned line fails.
18. **A test that reads a day's hash from a baseline reads the pinned line instead,** through one helper in `tests/common`. No expected value changes: the same hashes are read from another place.
19. **`scripts/regenerate.sh` writes the baselines and the `run_` lines** in its step 1 and reports them apart: a baseline changed means a game moved; only pins changed means it did not. `scripts/compare-scenarios.sh` goes on cutting the hash, for a revision from before this build.
20. **The pull request that makes the move changes nothing else,** and shows it: each baseline differs from its old self only in the hyphens, and each pinned line holds exactly the hashes taken out.
21. **The state version is raised at most once in a pull request,** whatever fields it adds, and the pins are remade once, in its last commit before it is merged. Steps of one decision built on one branch share one raise. `main` never holds two layouts under one version.
22. **Saved games, recorded games, the agent's goldens and the batch fixture stay as they are.** A saved game records its own hashes by design.

This amends decision 0057's item 2 only in where a baseline's hashes are kept. Its item 3 stands: the version is raised whenever what is hashed, or its order, changes. The build of C is class 2 and this record is its record.

## D: noted

One test binary for each crate, and no second run of the checks on `main` for a tree a pull request already passed, are changes to CI and to the test packages. They change no rule and need no record; by the paths they touch they are class 2, and item 14 says what the architect is shown. "Merged only after CI reports success on the pushed commit" stands.

## Rule card

No rule of the game is added, changed or removed. No state, draw, event, order or version moves. C changes what two kinds of generated file hold and one test's comparison.

## What it leaves out

- The plans for the other items (a scenario built on another, the export carrying the engine's words) are records of their own.
- The records before 0233 are not reshaped, and nothing in them is moved.
- The rulebook's chapters are not rewritten to take rules out of old records.
