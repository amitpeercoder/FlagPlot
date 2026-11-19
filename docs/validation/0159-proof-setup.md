# The proof of decision 0159: the files and the run scripts, before the run

Made on Nov 19, 2026, from main at 2ed57bfd (decision 0159's proof section written, all six steps on main): state layout `flagplot-state-v47`, doctrine `supply-39`, rules label 19, agent interface 26. This page says what was made for the proof, how to run each step and what was tried. It holds no result of the proof and no verdict. The proof is the architect's to run and judge.

## The three arrangements

The proof runs stage 3's three arrangements, `data/scenarios/stage-3-proof/07a_slice.toml`, `07b_slice.toml` and `07c_slice.toml`, made again by their script from the staff's setup as it now stands. `python3 scripts/stage-3-proof/make_arrangements.py --check` reports each of the three "as made": the staff's setup has not changed since they were made, so they were not made again and nothing in them changed.

## What was made

`scripts/0159-proof/` (its `README.md` has the table from the decision's words to the files):

- `run.sh`, in the manner of stage 3's: `build`, `batch`, `exports`, `sample` call stage 3's `run.sh` for the same steps; `orders` runs the helper below; `read` runs every reader; `all` runs them in order. It writes only into `--out DIR` and refuses a DIR inside the repository or the main checkout.
- `unused.py`, condition 2.
- `hunger.py`, condition 3.
- `short_lifts.py`, condition 4.
- `tests/proof_0159_orders.rs` (an entry in `tests/Cargo.toml`), condition 6: an ignored test that writes the six plan orders. `cargo test` passes over it.

Stage 3's `conditions.py`, `margin.py` and `same.py` serve condition 8 and the check that the same start gives the same game. They read today's exports and batch files as they are: every event kind and data key they read is still written (`search_flown` writes `base` for a group flying from a base, `ship` from a carrier), so nothing in them was mended.

## How to run each step

`scripts/0159-proof/run.sh STEP --out DIR [--seeds N] [--sample "1 2 3 500 1000"] [--jobs J]`, from any folder; every `sim-cli` command runs from the top of the repository.

| Step | What it makes |
| --- | --- |
| `build` | sim-cli in release, into `DIR/target` |
| `batch` | (a), (b) and (c), seeds 1 to N, each twice: `DIR/X.json`, `DIR/X.again.json` |
| `exports` | (a), one export a seed: `DIR/a/seedN.json` |
| `sample` | The sample seeds of each arrangement, exported twice: `DIR/X.sample/`, `DIR/X.sample.again/` |
| `orders` | The six plan orders: `DIR/orders/<n>_<case>.intent<N>.json`, and `DIR/orders/check.txt` |
| `read` | Every reader's output in `DIR/read/`: `unused.txt`, `hunger.txt` and `.jsonl`, `short_lifts.txt` and `.jsonl`, `a.conditions.txt` and `.jsonl`, `margin.a-b.txt`, `margin.a-c.txt`, `X.same.txt` |
| `all` | All of the above, in order |

For the run of 1,000 seeds, in order:

```
OUT=/a/folder/outside/the/repository
scripts/0159-proof/run.sh build   --out $OUT
scripts/0159-proof/run.sh batch   --out $OUT
scripts/0159-proof/run.sh exports --out $OUT --jobs 4
scripts/0159-proof/run.sh sample  --out $OUT
scripts/0159-proof/run.sh orders  --out $OUT
scripts/0159-proof/run.sh read    --out $OUT
```

## What was tried

On an Apple M3 Pro (11 cores, macOS), main at 2ed57bfd, seeds 1 to 5 only, into a folder outside the repository.

- **Every step ran**, `build`, `batch`, `exports`, `sample` (seeds 1 to 3, the default sample less 500 and 1,000), `orders` and `read`, and every reader ran to its end on what they made and wrote output of the shape described in its docstring. A DIR inside the worktree and one inside the main checkout were refused.
- **`unused.py`** reads 112 scenarios: 14 with a hold, a take or an island in force (48, 67, 68, 69, 72, 73, 74, 76, 77, 79 to 83) and 98 without, whose 194 run files it compares (96 with two files, the two soaks with one). Its reading of BEFORE's events files agrees with HEAD's on every scenario that has one.
- **`hunger.py` checked by hand** against `sim-cli run --events` of (a), seed 1. Japanese units 16 and 17, followed by eye: 200 and 916 men landed at (2,-4) while base 1 was Allied, and 83 and 356 lost unfed from day 17 to day 39, which are the script's counts for them. Allied unit 1: 4,306 landed on day 1 while base 1 was Japanese, 412 lost unfed on days 37 to 39, as the script has it. Allied unit 9 (landed on day 9, base 1 then Allied) is not counted, as the rule says. No loss falls in a landing's pulse before its landing, and no landing is in a hex without a base of the start, on the five seeds.
- **`short_lifts.py` checked by hand** against the same run. The estimate it reads for the Japanese first lift on morning 10 (11,450, so 22,900 wanted) is the one the staff's own reason states that morning; the Allied lifts of day 1 find no estimate held, as the staff's reason says ("the side holds no estimate"); the days it reads from the scenario file for units 19 to 25 (through their task forces' days, 25 to 36) are the days the staff's reasons give ("may be used from day 25", ...); the units it lists for the lifts of day 31 leave out the unit aboard the other lift of that morning.
- **The orders.** The helper writes seven files for the six cases: scenario 81's "after another place" has two of the commander's intents in force on morning 3, intent 1 (which waits after intent 3) and intent 3, and both answers are written. Each answer equals scenario 84's reading of the same morning (`check.txt`).
- **The same start.** Each arrangement's two batches of five seeds were the same byte for byte, and so were its two sets of sample exports.
- **Not tried:** any run of more than five seeds; the reading of anything for the proof.

## Time and disk

Measured on five seeds, release build, one process unless said; scaled to 1,000.

| What | Measured | 1,000 seeds |
| --- | --- | --- |
| One game, `run` | (a) 0.81 s, (b) 0.33 s, (c) 0.28 s | |
| Batch of 5 | (a) 4.6 s, (b) 1.7 s, (c) 1.4 s | (a) about 15 min, (b) 6 min, (c) 5 min; each twice: about 52 min |
| Batch file | 5 seeds: (a) 0.38 MB, (b) 0.19 MB, (c) 0.14 MB | about 76, 37 and 27 MB; twice: about 280 MB |
| Export of (a) | 1.3 s and 20.3 to 20.9 MB a seed | about 22 min one at a time (less with `--jobs`), about 21 GB |
| The sample | 3 seeds of each, twice: 15 s, 0.22 GB | 5 seeds of each, twice: about 25 s, about 0.4 GB |
| `hunger.py`, `short_lifts.py`, `conditions.py` | about 0.85 s each for 5 exports | about 3 min each |
| `unused.py` | 12 s, with the soaks run | the same |
| `orders` | 2 s once built | the same |
| The build into `DIR/target` | 33 s | 0.2 GB |

**In all: about 1 hour 30 minutes one process at a time (about 1 hour 10 minutes with `--jobs 4` on the exports), and about 22 GB.** Nothing made goes in the repository.

## What is not made here

- **Condition 8's "who holds the airfield on day 39"** is not read by stage 3's scripts (`conditions.py` counts the attack, line 4b; `margin.py` the margins). Stage 3's report gave it from the exports; the decision names stage 3's scripts and no other reader was written. The export's `base_changes` and `final.bases` hold it.
- **Condition 6's key, readers and marks**, which are the architect's.
