# The scripts that read the proof of decision 0159

The proof of decision 0159 is written in the decision's last section, "The proof: how each condition is read, written before the run": the three arrangements of stage 3's proof, staff against staff, on seeds 1 to 1,000. These scripts make the files it is read from and read the conditions that need a reader. They were written and tried on five seeds before the proof is run, so that the reading is fixed first.

They are the umpire's. They read the truth: `sim-cli export` files, `sim-cli batch` files, the generated run files in git and the scenario file. Nothing that plays the game reads them or what they print (`docs/engineering/truth-and-belief.md`). They print counts, ratios and medians, never a verdict: the marks are read by the architect.

Python 3.9 or later, standard library only, and one Rust helper that is an ignored test. No build step and no test reads these scripts or what they make; they are run by hand. Stage 3's scripts (`scripts/stage-3-proof/`) are reused, not copied: its `run.sh` for the build, the batches, the exports and the sample, its `common.py` for reading an export, and its `conditions.py`, `margin.py` and `same.py`.

## The scripts

| Script | Condition | Reads |
| --- | --- | --- |
| `unused.py` | 2 | The generated run files of every scenario of `data/scenarios/*.toml` with no hold, take or island in force, at the parent of the merge of pull request 446 and at HEAD, by `git show`; with `--sim`, the events of the two soaks, which keep no events file |
| `hunger.py` | 3 | The exports of arrangement (a), one a seed |
| `short_lifts.py` | 4 | The exports of arrangement (a), and the scenario file they name |
| `tests/proof_0159_orders.rs` | 6 | An ignored test that plays scenarios 79 to 82 on the frozen figures through `sim_agent::Session` and writes the six plan orders as the seat's `order` tool answers them |
| stage 3's `conditions.py` | 8 | The exports of (a) and its batch: line 4b, the Japanese side attacks the airfield ashore |
| stage 3's `margin.py` | 8 | The batches of (a) beside (b), and (a) beside (c) |
| stage 3's `same.py` | the same start | Each arrangement's two batches and two sets of sample exports |
| `run.sh` | all | Makes every file the readers read, one step at a time, into a folder outside the repository, and runs the readers |

Condition 1 (the hand calculations), 5 (a plan that waits says what for) and 7 (a seat is told nothing more) are read from CI and from the tests of scenarios 79 to 84 and `crates/sim-agent/tests/leaks.rs`, and need no reader. Condition 6's key, its readers and their marks are the architect's; this folder only makes the six orders.

## How to run them

`run.sh STEP --out DIR [--seeds N] [--sample "S..."] [--jobs J]`, with the step `build`, `batch`, `exports`, `sample`, `orders`, `read` or `all`. Its header says what each makes. It refuses a DIR inside the repository or inside the main checkout that holds a worktree. `docs/validation/0159-proof-setup.md` has the times and the disk.

By hand, from the top of the repository, with `OUT` outside it:

```
python3 scripts/0159-proof/unused.py --sim $OUT/target/release/sim-cli --list
python3 scripts/0159-proof/hunger.py $OUT/a --list --jsonl $OUT/read/hunger.jsonl
python3 scripts/0159-proof/short_lifts.py $OUT/a --list --jsonl $OUT/read/short_lifts.jsonl
FLAGPILOT_0159_ORDERS_OUT=$OUT/orders CARGO_TARGET_DIR=$OUT/target \
  cargo test --release -p scenario-tests --test scenarios proof_0159_orders:: -- --ignored
python3 scripts/stage-3-proof/conditions.py $OUT/a --batch $OUT/a.json --list
python3 scripts/stage-3-proof/margin.py $OUT/a.json $OUT/b.json
python3 scripts/stage-3-proof/margin.py $OUT/a.json $OUT/c.json
python3 scripts/stage-3-proof/same.py --batch $OUT/a.json $OUT/a.again.json --exports $OUT/a.sample $OUT/a.sample.again
```

## From the decision's words to the files

| The decision says | The script reads |
| --- | --- |
| 2. A scenario with no intent of the kind hold, take a place or control an island in force on any day, whoever made it | Its `run --events` file at HEAD has no `command_taken` line that sets (`set intent:N to`) or forms (`form under intent:N an intent to`) an intent "hold base:", "take the place at" or "control island " (`sim-core`'s words for the three kinds). An intent comes into force only by a command taken; only the commander and the staff set intents. The soaks, with no events file, are run with `--sim` |
| 2. Its two generated run files, before step 1 and at the run's commit | `crates/sim-cli/tests/run_baseline/NAME.txt` and `NAME.events.txt`, by `git show` at the first parent of the merge of pull request 446 and at HEAD |
| 2. The lines that carry a hash | `^day +[0-9]+ +hash [0-9a-f]{16} +[0-9]+ events$`, the only lines of those files with a hash. A pair of such lines that differ in more than the hash (the count of events) is listed apart, not counted |
| 3. Men put ashore by a landing in a hex whose base was not the side's | `men_landed` of a unit of the side, in a hex with a base of the start (the export's `bases`) none of which the side holds just before the event (stage 3's `base_sides_timeline`). A landing in a hex with no base of the start is counted apart |
| 3. The men those units lose unfed from that pulse to the end of day 39 | `men_lost_unfed` of those units, by (day, pulse) from the unit's first counted landing |
| 3. The same counted only while the place was not the side's | The same losses where, just before the loss, no base of the start in the hex of the unit's latest counted landing is the side's |
| 4. A lift: a "ready a landing" whose ships sail for the place under a take | A staff order "route via ..., acting at" the take's hex, to a task force, whose `staff_reason` serves the take or its `ready_landing`. A route order to a force that has neither landed men at the place nor arrived anywhere since its last lift is a change of route, not a new lift |
| 4. The men of the units it carries | The `troops` of the force's ships at the last step at which it still lies where it lay that morning |
| 4. The men wanted on the morning it was released | The side's `ground_estimates` at the take's hex at the end of the day before, times 2. The morning is the plan's release for the first lift and the sailing for a later one |
| 4. Units the take's forces limit lets it use, not in a lift, usable at the source within 7 days | Land units of the side (those the take's or the island's forces limit names, all with none named) with men ashore in the source's hex or aboard a force lying there, not aboard the lift or another lift of that morning, whose `available_from` (the latest of the unit's, its ship's and its force's, from the scenario file) is no later than the morning plus 7 |
| 6. The six orders as a seat's tool gives them | `Session::call(Allied, "order", {"intent": N})` after the days before the morning are ended with the seat's `end_day`, each answer written untouched; checked against scenario 84's reading of the same morning |
