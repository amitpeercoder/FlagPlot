# The scripts that read stage 4's proof

The proof of stage 4 is planned in decision 0161 and its setup page, `docs/validation/stage-4-proof-setup.md`: thirteen conditions read on the 39 days of stage 3's three arrangements, on built cases, and on a seat's own trials. `run.sh` makes the files it is read from and runs the readers; the harness is a set of ignored tests. They were written and tried on seed 2,001 (which is no judged seed) before the proof is run, so that the reading is fixed first. The proof is run once, on main, at the commit the report names.

They are the umpire's. They read the truth: exports, batch files, the worlds a trial plays and the truth of the games. Nothing that plays the game reads them or what they write (`docs/engineering/truth-and-belief.md`). They write counts, ratios and medians, never a verdict: the marks are read by the architect.

Python 3.9 or later, standard library only, and one Rust file of ignored tests. No build step and no test reads these scripts or what they make; they are run by hand. Earlier proofs' scripts are reused, not copied: stage 3's `run.sh` (build, batches, exports, sample) and `same.py`, decision 0159's `hunger.py` (condition 3), and `scripts/unfed/unfed.py` (condition 13).

## From a condition to a file

| Condition | Made and read by |
| --- | --- |
| 1. The hand calculations pass | `cargo test --workspace` in release at the proof's commit, and by name the tests the setup lists (amendment 8). By hand |
| 2. Nothing changed where it is unused | `sh scripts/regenerate.sh`, the baselines' tests. By hand |
| 3. The mends hold | `run.sh exports`, `run.sh read`: `scripts/0159-proof/hunger.py` on seeds 1 to 1,000 and on 101 to 1,000 alone (`DIR/read/hunger.txt`, `hunger.101.txt`) |
| 4. No peek | `run.sh harness`: `condition_4_no_peek`, `DIR/harness/condition_4.txt` |
| 5. The same twice | `condition_5_the_same_twice`, `condition_5.txt` |
| 6. Its sampling is right | Scenario 95's tests (condition 1) |
| 7. Its spreads hold | `condition_7_spreads`, `condition_7.txt`: cases S1 and S2 at three sizes beside 200 games |
| 8. A wrong picture moves it | `condition_8_wrong_pictures`, `condition_8.txt`: W1, W2 (where its file exists) and W3 |
| 9. Preparation matters | `condition_9_preparation`, `condition_9.txt`: case R |
| 10. Within its budget | `run.sh time`, `DIR/time/`: each size in its own process, three runs of each of a trial, a trial beside another and a window, with the peak memory |
| 11. Legible | `legibility_estimates`: the six estimates as a seat is answered and the key from their fields (`DIR/harness/legibility/`); the prompt is `crates/sim-cli/tests/recorded_games/legibility_stage_4/prompt.md`. No reader is run by these scripts |
| 12. Reported, not judged | `condition_12_beside_the_games`, `condition_12.txt` |
| 13. Men who begin ashore are fed | `run.sh read`: `scripts/unfed/unfed.py --all` on the two sets (`DIR/read/unfed.txt`, `unfed.101.txt`); and `condition_13_japanese_ashore_at_the_start` for the list of units against the islands' land (`condition_13_units.txt`) |

## How to run them

```
OUT=/a/folder/outside/the/repository
scripts/stage-4-proof/run.sh check   --out $OUT
scripts/stage-4-proof/run.sh build   --out $OUT
scripts/stage-4-proof/run.sh batch   --out $OUT
scripts/stage-4-proof/run.sh exports --out $OUT --jobs 4
scripts/stage-4-proof/run.sh sample  --out $OUT
scripts/stage-4-proof/run.sh read    --out $OUT
scripts/stage-4-proof/run.sh harness --out $OUT --jobs 4
scripts/stage-4-proof/run.sh time    --out $OUT --machine "Apple M3 Pro, 11 cores, 36 GB"   # alone on the machine
```

`run.sh all --out DIR` runs every step but `time`. It refuses a DIR inside the repository or the main checkout that holds a worktree. The harness's seed is `--seed` (default 1, the judged one); `--seed 2001` shows that it runs on a seed that is not judged.

The harness is `tests/proof_stage_4_harness.rs`: eight ignored tests, each writing one file and asserting nothing. Without `FLAGPILOT_STAGE4_SEED` and `FLAGPILOT_STAGE4_GAMES` it plays seed 2,001 and four games.

## Choices of the harness the architect should know

- **A trial on (a), (b) or (c)** is given `data/reference/trial.toml` (the setup, amendment 2); a case's trial is given its own named file.
- **The cases run on the frozen step 1 figures,** as their hand-calculation tests do. (a), (b) and (c) run on the live ones.
- **The seat's answer** (conditions 4, 5 and 11) is the session's `trial` tool, answered as a seat is. The two answers of condition 5 differ by the answer's own number among the estimates the seat was shown (the second is numbered 2); the file gives equality both as it stands and with that one field set aside.
- **Condition 4's hidden changes** are text changes to (a) on the three-day game: a base's supply raised by 1,000 t, and a hidden thing tried in turn (a Japanese land unit's men by 100; an Allied ship's fuel by minus 100 t) until the side's picture, by the seat's `get_briefing`, `list_forces`, `list_contacts`, `list_bases` and `command_state`, is equal field for field and the state hashes differ. A copy that does not load, or whose picture is not equal, is written as void and the next is tried.
- **A game's reading** is the estimate's own: the end met at the end of a day by `sim_trial::met` on the side's bases, the losses by `sim_trial::Losses::of` on the side's view of the game, read at the horizon of the size tried.
