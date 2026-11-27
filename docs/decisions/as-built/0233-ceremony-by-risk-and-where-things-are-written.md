# 0233, as built: Ceremony by risk, and where things are written

## Amended on review (Nov 27, 2026, pull request 743)

An independent review of the record as first written said "must change". What changed, and why:

1. **Class 1 is told by paths** (item 7), where it was told by the golden files passing and by "no rule's text changes beyond wording". The rules under "Always" in `CLAUDE.md` are the ones no test catches, "wording" is a judgement, "documents" let the manager edit specs, records and closed reports, "test plumbing" and "CI, scripts" let a test be loosened or a check dropped, and `data/` was in no class. All of those now go to the architect by name.
2. **A pin never moves in class 1** (item 11). The first table let class 1 remake pins "for a layout raised elsewhere" while another line said a moved pin stops housekeeping. Until C is built every changed baseline is shown to the architect.
3. **Class 2 changes no rule and no scenario** (the table, item 13), where it could change what the game does. So Amit sees before a build all he saw before, and the record says what he now learns only afterwards.
4. **Scenarios, figures and data have a class** (the table): 3.
5. **What is in flight is classed** (item 14): the move to one test binary a crate, begun as housekeeping under the first test, is class 2 with no record, read by the architect; so is the split of rules chapter 8.
6. **A rule keeps its number for life** (A, item 2), carried as a tag in the rulebook, so no citation in code or in a doc comment is written again when a rule's text moves.
7. **C says how it shares `tests/golden/pinned_hashes.txt`** (item 16): the line of many hashes, the heading, the two writers, the stem, a scenario that is gone, and how `export.rs` reads it.

## Built: C (the day hashes out of the run baselines)

Items 15 to 20 and 22 of the record, in one pull request that changes nothing else.

**What was built**

- Each of the 252 files of `crates/sim-cli/tests/run_baseline` writes the hash of every `day N  hash H  K events` line as sixteen hyphens; everything else is `sim-cli run`'s output byte for byte, and `run` prints as before.
- `tests/golden/pinned_hashes.txt` has a new heading that states both kinds of line and who writes each, the four `name hexhash` lines as they were, and 127 `run_<stem>` lines (one a scenario, 1,289 day hashes) after them in the order of their names. The plain run and the run with `--events` share a line.
- `scripts/regenerate.sh`, step 1, writes the baselines and the `run_` lines from the same runs, checks that the run with `--events` gives the hashes of the plain run, writes the pinned file only when every run was made, and removes the line of a scenario that is gone with its baselines. Its closing report gives the baselines and the `run_` lines apart: a baseline changed means a game moved; only lines changed means the state or its layout moved and no game did. `scripts/compare-scenarios.sh` is unchanged and still cuts the hash.
- `common::pinned_hash` keeps the `run_` lines (a new line goes in before the first of them, and a name beginning `run_` is refused); `common::pinned_day_hashes(stem)` reads a `run_` line. The tests that read a day's hash from a baseline read it from there: scenario 6c (`a_scenario_without_the_fatigue_file_...`), 6d1 and 6d2 (`..._has_the_hashes_it_had_before_this_stage`), 10 (`objectives_change_no_hash_and_no_event`, twice), 102 (`baseline`, hashes only) and 128 (`baseline_hash`). No expected value changed: the same hashes are read from another place.
- `run_output_matches_the_golden_files` (in `crates/sim-cli/tests/export.rs`, which reads the pinned file by `ROOT` with a reader of its own) compares the output with its hashes hyphened against the baseline ("A GAME MOVED") and the day hashes against the pinned line ("THE STATE OR ITS LAYOUT MOVED"), fails a scenario with no line and a `run_` line with no scenario file.
- Not touched (item 22): saved games, recorded games, the agent's goldens, the batch fixture. `scripts/0159-proof/unused.py` matches a hash in a baseline and reads the old form at an old commit; it is a record of that proof and is left as it is.
- `docs/commands.md`, `docs/engineering/running-the-project.md`, `docs/engineering/roles-and-tests.md` and `docs/open-work.md` say where they described the baseline or the pins.

**Item 20, shown.** Throwaway script (not committed) comparing the old baselines and pins file, copied before the change, with the new:

```
baselines compared 252 extra set() hash lines 2218 bad 0
run_ lines 127 equal to old baselines 127 scenarios 127 hashes pinned 1289
name lines same: True 4
```

Each of the 252 baselines differs from its old self in the 2,218 hash digits only (every other line equal, the same number of lines), the plain and the `--events` baseline of a scenario held the same hashes, each of the 127 `run_` lines holds exactly the hashes taken out of its baselines, and the four `name hexhash` lines are unchanged.

**Checks**

- `cargo fmt --all -- --check`: clean. `cargo clippy --workspace --all-targets -- -D warnings`: clean.
- `cargo test --profile checked --workspace`: 5,849 passed, 0 failed, 23 ignored (the ignored tests are the ones that were).
- `sh scripts/regenerate.sh --check`: every group checked (1 run baselines, 2 agent goldens, 2 viewer measure fixture, 3 recorded games, 4 pinned hashes, 5 batch fixture, 6 submarine tables), exit 0, "every generated file is what the engine gives". With one pinned hex digit altered the test said "THE STATE OR ITS LAYOUT MOVED (the game did not)"; with one baseline hyphen replaced by a digit it said "A GAME MOVED" (both undone).
