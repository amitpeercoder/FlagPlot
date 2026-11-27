# 0233: Ceremony by risk, and where things are written

Status: ruled by the architect on Nov 27, 2026, on Amit's delegation of that day. He was shown the architect's review of the repository's upkeep (five refactorings, 1 to 5, and eight changes to how the work is done, A to H), approved all of it, and said: "Let the architect rule where and when such needed. Have Sonnet agents do the development. Have the architect as a final reviewer. Do all that's needed (all items)." This record rules A, B and C and notes D. The delegation covers those items and the programme in `docs/open-work.md`; it does not cover what the game is.

Summary: A change is one of three classes, and the roles and reviews it needs follow from its class: housekeeping, a bounded change, or a rule. From this record on a decision record holds the why, what was weighed, the rule card and the rulings; a rule is stated once, in the record while it is a plan and in the rulebook once built; the arithmetic is in the test's doc comment; what was built and the rulings on a pull request go in a file beside the record. The day hashes leave the 252 run baselines for one file of pins, so a baseline changes only when a game does. No rule of the game changes.

Standing: Ruled by the architect on Nov 27, 2026, on Amit's delegation; in force for records and pull requests from this one on; C to be built

## Why

Every change paid for the costliest kind. A record restated its rule, the rulebook stated it again and the test a third time, and grew by an "As built" and a set of rulings for each pull request: the three copies drifted and a record ran to hundreds of lines. A plan for a wording mend had the same reviews as a new rule. A state layout raised by one field rewrote 252 files, so a reader could not see whether a game had moved.

## What was weighed

- **Rewrite the old records to the new shape.** Not done: 232 records are cited by section name from code, tests and reports, and a rewrite is a risk for no gain. The shape binds records from 0233 on.
- **Two classes, not three.** Not done: the saving is in the middle, a change to what the game does that adds no state, draw or event.
- **Let the manager classify everything.** Not done: the manager does not rule. It may say a change is housekeeping by a test anyone can check; above that the class is the architect's.
- **Drop the day hashes from the baselines and pin only the last day.** Not done: the soaks have no events baseline, and their days' hashes are what is compared.

## A: what is written where

1. **A record holds:** the header; why; what was weighed and not chosen; the rule card (the state read and written, the events and who sees them, the versions moved); the rulings a build cannot make for itself; what it leaves out; and the questions for the designer, if any. It names its test file and its rulebook section. It does not hold a hand calculation.
2. **A rule is stated once.** While it is a plan it is in the record, under "The rule as planned". The pull request that builds it moves that text to its section of `docs/rules/` and leaves in the record one line naming the section. Built in steps, each step moves its own rules. The rulebook stays what the code does.
3. **The statements and the hand calculation are in the test file's doc comment,** as now, written by the architect before code is delegated. They cite a rule by its record and number, or by its rulebook section once built, and do not restate it.
4. **What was built and what was ruled on it go beside the record,** in `docs/decisions/as-built/NNNN-the-records-name.md`. It opens `# NNNN, as built: Title` and is added to at the end, never rewritten, in sections headed by kind, date and pull request: "Built: step 2 (Nov 28, 2026, pull request 750)", "The architect's rulings on pull request 750 (Nov 28, 2026)", "Amended on review (date)". A build's section says what was built, the versions and games that moved, and the readings it asks the architect to rule on. The record itself is kept as the plan in force: an amendment changes its text, and the file beside it says what changed and why.
5. **Records before this one keep their form.** A new section for one of them (a later step built, a later ruling) goes in its file under `as-built/`, with one line in the record's Status pointing there.
6. **Status and Standing stay short:** who ruled or confirmed and when, and what is built. The index is made from them as before.

## B: three classes of change

| Class | What it is | Needs |
| --- | --- | --- |
| 1, housekeeping | Nothing the game does or says changes. Documents, comments, CI, scripts, test plumbing, a pure move of code, pins remade for a layout raised elsewhere, the table's wording inside its design rules | The manager decides and briefs; a developer builds; the manager reads and merges. No record, no call to the architect |
| 2, a bounded change | What the game does or what a player or agent reads changes, with no new state, draw, event kind, order or tool kind, no change to who knows what, and no game of a closed proof or a validation measure moved. Also a mend that makes the code do what its record says, a loader or tool change with a stated equality to check, and a refactoring of `sim-core` that leaves every hash as it is | A short record by the architect, or a section in the `as-built/` file of the record it mends, with the statements and any expected number first. No independent review of the plan and no confirmation by Amit. A developer builds; the manager reads; the architect reads the pull request before it is merged |
| 3, a rule | A new rule or a removed one; new state, a draw, an event kind, an order or tool kind; anything in truth and belief or in what is hashed beyond a pure move; a crate boundary; a change that moves a closed proof's games or a validation measure; data settled between sources; anything that changes what the game is | As before this record: a plan in a record by the architect, an independent review and the amendment, Amit's confirmation where the record says "needs review", the hand calculation before code, the rulebook in the same pull request, and the architect's reading of the pull request before it is merged |

7. **The test for class 1** is one anyone can run: no file under a crate's `src` changes what a run prints, so every golden file, pin and recorded game passes untouched; no statement, expected number or hand calculation changes; no rule's text changes beyond wording. A change that meets it is class 1 on the manager's word.
8. **Above class 1 the architect classifies.** The manager calls with the change and the files, not with a class. The architect's answer names the class and the record, and both are written in the pull request's description.
9. **In doubt, the higher class.** In doubt whether a change is housekeeping, it is not; in doubt whether something is a ruling, it is.
10. **A class can rise and never falls during a build.** A developer or the manager who finds a change reaching past its class (a golden file moves, an event is wanted, a statement does not hold) stops and goes back to the architect.
11. **What no class relaxes:** nobody but the architect writes or changes a statement or an expected number; a change to a rule changes the rulebook in the same pull request; determinism, traceability, truth and belief and the data rules; CI green before a merge; every merge reported to Amit, with its class.
12. **Amit sees before it is built** what he saw before: what the game is, a spec, and any record marked "needs review". Classes 1 and 2 he sees in the report of what was merged.

## C: the day hashes of the run baselines

Today each of the 252 files under `crates/sim-cli/tests/run_baseline` holds its run's day hashes, so a raise of the state layout rewrites all of them. From this build:

13. **A baseline holds no hash.** In every line of the form `day N  hash H  K events` the sixteen hex digits are written as sixteen hyphens. Everything else in the file is `sim-cli run`'s output byte for byte, and `run` itself prints as before.
14. **The hashes are pinned in `tests/golden/pinned_hashes.txt`,** one line a scenario: `run_<file stem>` and then the hash of each day in order, in hex, separated by one space. The plain run and the run with `--events` give the same hashes and share the line.
15. **`run_output_matches_the_golden_files` makes two comparisons** and says which failed: the output with its hashes hyphened against the baseline (a game moved), and the day hashes against the pinned line (the state or its layout moved). A scenario with no pinned line fails.
16. **A test that reads a day's hash from a baseline reads the pinned line instead,** through one helper in `tests/common`. No expected value changes: the same hashes are read from another place.
17. **`scripts/regenerate.sh` writes both** in its step 1 and reports them apart: a baseline changed means a game moved; only pins changed means it did not. `scripts/compare-scenarios.sh` goes on cutting the hash, for a revision from before this build.
18. **The pull request that makes the move changes nothing else,** and shows it: each baseline differs from its old self only in the hyphens, and each pinned line holds exactly the hashes taken out.
19. **The state version is raised at most once in a pull request,** whatever fields it adds, and the pins are remade once, in its last commit before it is merged. Steps of one decision built on one branch share one raise. `main` never holds two layouts under one version.
20. **Saved games, recorded games, the agent's goldens and the batch fixture stay as they are.** A saved game records its own hashes by design.

This amends decision 0057's item 2 only in where a baseline's hashes are kept. Its item 3 stands: the version is raised whenever what is hashed, or its order, changes. The build of C is class 2 and this record is its record.

## D: noted

One test binary for each crate, and no second run of the checks on `main` for a tree a pull request already passed, are changes to CI and to the test packages. They are class 1, change no rule, and need no record. "Merged only after CI reports success on the pushed commit" stands.

## Rule card

No rule of the game is added, changed or removed. No state, draw, event, order or version moves. C changes what two kinds of generated file hold and one test's comparison.

## What it leaves out

- The plans for the other items (a scenario built on another, the export carrying the engine's words) are records of their own.
- The records before 0233 are not reshaped, and nothing in them is moved.
- The rulebook's chapters are not rewritten to take rules out of old records.
