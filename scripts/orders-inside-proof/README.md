# The proof of decision 0156: orders inside a day

What was run for step 5 of decision 0156, the proof of its eight conditions. `docs/validation/orders-inside-a-day-proof.md` has the figures. The outputs are kept outside the repository.

| File | What it is |
| --- | --- |
| `make_arrangements.py` | Makes the four scenario files of `data/scenarios/orders-inside-proof/` from scenario 5, scenario 36 and stage 2's second proof's arrangement (c); with `--check`, says whether the files kept are the ones it makes. Its docstring says what each file changes and which condition it serves |
| `run.sh` | The commands of each condition, one step at a time (`run.sh STEP --out DIR [--before WORKTREE]`) |
| `read_games.py` | Reads condition 7's games from the umpire's side once each is over: a strike at ships, the Allied margin and the orders given inside the day |
| `condition-7/` | The two briefs of condition 7, as written before the first game (`brief_common.txt` with `brief_days.txt` or `brief_pulse.txt`; `PLAY` stood for the wrapper's path), and the wrapper each player called |

The tests are in `crates/sim-agent/tests/orders_inside_proof.rs`: condition 2 (`#[ignore]`, `PROOF_SEEDS`), condition 5 (run with the rest), condition 6's six days (`#[ignore]`, `PROOF_OUT`) and condition 8's timing (`#[ignore]`, `PROOF_REPEATS`). Condition 6's material, key, answers and marks are in `crates/sim-cli/tests/recorded_games/legibility_orders_inside_a_day/`.

Python 3.9 or later, standard library only. No build step and no test runs these scripts.
