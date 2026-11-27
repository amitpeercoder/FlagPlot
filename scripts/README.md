# Scripts

Commands for working on the repository, not parts of the game (`docs/engineering/layout.md`). `docs/commands.md` says what each one does and takes.

## Live

Read and run these for current work.

- `decisions-index.sh` (CI runs it with `--check`), `regenerate.sh`, `compare-scenarios.sh`.
- `web/` (`build.sh`, `site.py`): the browser build, run by CI.
- `stage-4-proof/`: stage 4's proof, not yet run. Its `run.sh` also calls `stage-3-proof/`, `0159-proof/hunger.py` and `unfed/unfed.py`, so those three stay working, and the "not maintained" note below does not apply to them, until stage 4's proof closes.
- The scenario generators, each run with `--check` to confirm the generated files are as the script makes them: `table-boats/`, `table-copies/`, `table-rear-bases/`. Since step C of decision 0234 `table-copies/make_copies.py` makes 3 of its 12 files and keeps 9, and `table-boats/make_files.py` makes 13 of its 16 and keeps 3: a kept file's source is built on another file, so it is not read, written or compared (`--check` prints `: kept, its source is built on another file`). Step D retires the three scripts. A scenario's files are changed by changing its generator and running it, never by hand.
- Retired by decision 0234, which has the Coral Sea's, the five days' and Midway's files built on their bases and states only what differs: the generators `coral-sea/` (three), `five-days/make_files.py`, `five-days/make_oiler_files.py` and `midway/make_files.py`. They are in git before the commit that converted the files.
- `time-table-dry-run.py`: times a dry run of a written plan on the table's 39 days, each size beside the time the designer accepted, on a throwaway copy that names the trial's figures (decision 0232, step 3, the designer's answer of Nov 27, 2026). It is run by hand, once, on the designer's machine; when the trial key is in the table's files it has nothing more to do and goes.

## Frozen

These served a closed proof, a count or a measurement owed by a decision, and have been run. They are kept as the record of how the numbers in that decision or report were made, and for rerunning the same count if the record is ever challenged. They are not maintained: they may no longer build against the current engine, the data files or the tool names, and they are not to be read for current work. Do not extend one for a new proof; write a new folder.

| Folder or file | What it served |
| --- | --- |
| `0159-proof/` | The proof of decision 0159 (`docs/validation/0159-proof.md`) |
| `0220-count/` | The count owed by decision 0220, as stated and as run |
| `orders-inside-proof/` | The proof of orders inside a day (decision 0156) |
| `stage-3-proof/` | The proof of stage 3, taking and holding a place (decision 0153, `docs/validation/stage-3-proof.md`) |
| `unfed/` | The measurement of the men unfed (decisions 0161 and 0231) |
| `five-days/read_first_run.py` | The reading of the first run of scenario 103 (decision 0184) |

The `make_arrangements.py` in `orders-inside-proof/` and `stage-3-proof/` make the files of those proofs and take `--check`; run with `--check` they confirm the proof's files are unchanged, and that is all they are for. Nothing in this folder is deleted or moved, because the decisions and reports cite these paths.
