# Stage 3's proof: the files and the run scripts, before the run

Made on Nov 18, 2026, from main at 4dec6155: decision 0151, "control an island", is on main; decision 0152, "forces with men aboard", is not. This page says what was made for the proof of decision 0153, how to run each step, and what was tried. It holds no result of the proof and no verdict. The loading check has not been made.

## The three files

`data/scenarios/stage-3-proof/` holds the three arrangements of decision 0153's table:

| File | Allied | Japanese | It serves |
| --- | --- | --- | --- |
| `07a_slice.toml` | intents | intents | Conditions 1 to 4, 6 and 7, and both halves of 5 |
| `07b_slice.toml` | nothing | intents | The Allied half of condition 5 |
| `07c_slice.toml` | intents | nothing | The Japanese half of condition 5 |

Each is the staff's setup, `data/scenarios/staff/07_full_slice.toml`, with four changes and nothing else. Each file's header lists them.

1. **A header and the name.**
2. **Placeholder night force A** (task force 6, Japanese, the placeholder cruisers 40 and 41) is restated by the architect's ruling in decision 0153 ("The loading check"). In the staff's setup it lies in the open Coral Sea at (-8,10) on the surface combat mission. In these files it lies stopped at Shortland (base 7), with Shortland as its home port, on the bombardment mission. Its day, 18, and its ships are as they were. It is still a placeholder. The force is the one the setup names "Placeholder night force A"; no other force has that name.
3. **The two islands** of decision 0153: island 1, Guadalcanal, of Lunga (base 1), and island 2, Tulagi, of Tulagi (base 4).
4. **The intents as stated**, on day 1, for each side the arrangement gives intents to: intent 1, control island 1, and intent 2, control island 2. There is no forces limit and no risk, and both leaves (to prepare and to launch) are given. A side given nothing has no command at all.

The files are made by `scripts/stage-3-proof/make_arrangements.py` and are not edited by hand. If the staff's setup changes, run the script again. `python3 scripts/stage-3-proof/make_arrangements.py --check` says whether the files kept are still the ones it makes, and exits 1 where one is not. No build step and no test runs it.

**Kept out of the loops.** No loop of the tests, CI or `scripts/` walks a subfolder of `data/scenarios`. Each reads only `data/scenarios/*.toml`: CI's export of every scenario, `scripts/regenerate.sh`, `scripts/compare-scenarios.sh`, and the tests `crates/sim-cli/tests/export.rs` and `saved_games.rs`. So the folder keeps these files out of them, as `staff/`, `second-proof/` and `stage-2b-proof/` keep theirs (decision 0072).

## How to run each step

`scripts/stage-3-proof/run.sh STEP [a|b|c ...] --out DIR [--seeds N] [--sample "1 2 3 500 1000"] [--jobs J] [--machine TEXT]`

It runs nothing unless it is given a step, and it writes only into DIR. It refuses a DIR inside the repository or inside the main checkout that holds the worktree. With no arrangement named, it does all three. sim-cli is `DIR/target/release/sim-cli` once `build` has run, or the path in `SIM_CLI`. Every sim-cli command runs from the top of the repository.

| Step | What it makes |
| --- | --- |
| `build` | sim-cli in release, built into `DIR/target` |
| `loading` | The loading check: arrangement (a), seed 1, with breakdowns on. `export` and `run` record breakdowns; only `batch` turns them off. It writes `DIR/loading/a.seed1.json` (the export), `a.seed1.run.txt` (`run --events`), `a.seed1.save.json` and `a.seed1.replay.txt` |
| `batch` | For each arrangement, seeds 1 to N, made twice: `DIR/X.json` and `DIR/X.again.json` |
| `exports` | For each arrangement, one export for each seed from 1 to N: `DIR/X/seedN.json` |
| `sample` | For each arrangement, the sample seeds exported twice: `DIR/X.sample/` and `DIR/X.sample.again/` |
| `time` | `sim-cli time --slice` on arrangement (a), into `DIR/time.txt`. It needs `--machine` |
| `read` | `conditions.py` (with the batch as its check), `same.py`, `margin.py` for (a) beside (b) and (a) beside (c), and `pick.py`, on whatever files are there. Their output goes into `DIR/read/` |
| `all` | All of the above in order. It passes over `time` without `--machine` |

**Which exports are needed.**

- Conditions 2 to 4 are read by `conditions.py` from one export for each seed of arrangement (a), so the proof needs all 1,000 exports of (a): `run.sh exports a`.
- `pick.py` reads seeds 1 to 10 of (a), which are among them.
- Arrangements (b) and (c) need no export for the conditions, because condition 5 reads their batch files. They need exports only for condition 1's sample.
- The sample is not fixed by decision 0153. The script's default is the five seeds the second proof of stage 2 used: 1, 2, 3, 500 and 1,000. That makes five exports, made twice, for each of the three arrangements: 30 exports.

## What was tried

All of this was done on main as it is, without decision 0152, on an Apple M3 Pro (11 cores, 36 GB, macOS).

- **Each file loads and runs.** `sim-cli run` of each arrangement on seed 1 runs to day 39 and exits cleanly. No invariant fails, no order is rejected, and no command is refused. On arrangement (a), seed 1, the saved game replays to the same hash every day.
- **What the staff does on arrangement (a), seed 1.** This is a smoke test, not the loading check. It is read from the staff's own reasons and notices, and nothing about the proof is concluded from it.
  - Both sides' commands are taken.
  - The Allied staff forms, under each island, a take, a hold, a denial and the neutralising of Lunga, Tulagi and Shortland. Under Guadalcanal's denial it forms carrier tasks for task forces 1 and 2, from (7,-5).
  - The Allied takes form nothing on any day. Their reason is "no base can be the source": no transport force is stopped at a base beside a fighting unit. The Allied transports start at sea with the men aboard, which waits for decision 0152. So no man lands and no base changes side.
  - The Allied neutralising finds no bomber in reach on any day.
  - The Japanese staff forms a hold, a denial and searches and strikes from Rabaul under each island, and the neutralising of Espiritu Santo, which finds no bomber in reach. On day 17 it forms carrier tasks for task forces 3 and 4, from (3,-8).
  - The Japanese holds form no supply. Their reason: "its forces limit names no source, so the staff forms no supply for it".
  - No staff forms a bombardment, so night force A stays at Shortland.
  - The plans of both sides give the notice "cannot meet its intent" on every day.
  - Strikes are flown from day 17 and three ships sink.
  - There is no fight ashore and no bombardment. Men ashore are lost for want of food.
- **The reading scripts.** `run.sh` was run on five seeds of each arrangement: `loading`, then `batch`, `exports`, `sample` and `read`. Every script ran on what it made. The two batches of each arrangement were the same byte for byte, and so were the two sets of sample exports (seeds 1 to 3).
- **`pick.py`.**
  - Row 6b (the forming of a carrier task under an island's denial) now matches the words now on main. It was tested on scenario 76 with the carrier force its test adds, and on seed 1 of (a), where it takes the four carrier tasks the two sides form.
  - Row 4b (a force with men aboard sent by a hold) is still untested, because it waits for decision 0152.
  - One more change: an approval, or a plan marked or cancelled, now matches no row, by the architect's reading 12 of decision 0153. Since decision 0151, the "keep supplied" under an island's hold owns a plan, and row 4a had been taking its approvals on scenario 76. Scenarios 66 to 74 pick exactly the reasons they picked before.
- **`time`.** It ran on arrangement (a) and took 11 seconds in all.
- **Not tried.**
  - The loading check, which is the architect's to read.
  - Any run of 1,000 seeds.
  - Anything with decision 0152, whose men aboard will change what the games hold (more landings, fights and reasons, so larger exports and longer runs).

## Time and disk

These were measured on the machine above, release build, one process, before decision 0152. Once decision 0152 is built the games will hold more, so the figures below are a floor: allow about twice as much.

| What | Measured | Scaled |
| --- | --- | --- |
| One staffed game of (a), `run` | 0.16 to 0.21 s | |
| Batch of 20 seeds | (a) 2.6 s, (b) 1.1 s, (c) 0.5 s | 1,000 seeds: (a) about 130 s, (b) about 57 s, (c) about 23 s. The three arrangements twice: about 7 minutes |
| Batch file | 20 seeds: (a) 0.84 MB, (b) 0.72 MB, (c) 0.33 MB | 1,000 seeds: about 42, 36 and 17 MB. Twice: about 190 MB |
| Export of (a) | 0.24 s and 11.6 to 12.7 MB a seed | 1,000 seeds: about 4 minutes one at a time (less with `--jobs`), about 12 GB |
| The sample | 30 exports | About 10 s and 0.3 GB |
| `conditions.py` | 0.4 s for 5 exports of (a) | About 80 s for 1,000 |
| `pick.py` | 0.5 s for 5 exports | Reads 10 at most |
| `time` | 11 s | |

**In all: about 15 minutes and 13 GB before decision 0152; allow 30 minutes and 25 GB after it.** The scripted scenario 7 makes a 7.1 MB export of seed 1 against the staffed (a)'s 12.7 MB, because a staffed game records its reasons. Nothing made goes in the repository.
