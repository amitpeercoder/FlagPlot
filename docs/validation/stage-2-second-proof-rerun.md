# Step 3, stage 2: the second proof, run again

Run on Nov 16, 2026, by the team lead of the rerun, to the test plan of decision 0128 as it stands with the architect's rulings made before the first run ("The intents as stated" and "Restated before the legibility test was run" in that record). The first run's report, `docs/validation/stage-2-second-proof.md`, is kept as it was. Since that run, decision 0130 (a carrier force comes up at full speed and closes the range) and decision 0131 (a search flown once a day, a bomber that takes a turn at the search) were built. Release build, live figures, seeds 1 to 1,000, doctrine `supply-16`, state layout `flagplot-state-v34`, from main at the merge of pull request 380. Nothing was tuned, and no pass mark, intent, measure or variant file was changed.

**Result: the proof does not pass as stated.**
- **The Eastern Solomons fails** on two conditions.
  - **The strike condition, for the Allied side.** Both sides from intents, the Allied side launches a strike at ships on 558 seeds, against the 800 asked. The Japanese side now passes it, on 865.
  - **The comparison against no orders, for the Japanese side.** From its intents its median margin is -5.40, against 0.00 given nothing. The Allied side passes, 0.25 against 0.20.
- **The week passes every condition.** The Japanese search, which failed on 317 seeds in the first run, now holds on all 1,000.
- **Everything else holds:**
  - every invariant;
  - no staff order and no command refused, on any seed;
  - the same start gives the same game;
  - legibility, 24 of 24.

Each failure is reported below with its seeds and the reasons the staff gave. Nothing was mended.

## The two runs side by side

| Part | Condition | First run (`supply-14`) | Rerun (`supply-16`) | Holds now |
| --- | --- | --- | --- | --- |
| Eastern Solomons | Every invariant, every seed | 1,000 | 1,000 | yes |
| | No staff order or command refused | 0 refused | 0 refused | yes |
| | A search and a patrol, every seed (Allied, Japanese) | 1,000, 1,000 | 1,000, 1,000 | yes |
| | A strike at ships on 800 or more seeds: Allied | 0 | **558** | **no** |
| | the same: Japanese | 152 | 865 | yes |
| | The script's own strikes, (f), for reference (Allied, Japanese) | 741, 814 | 741, 814 | (not judged) |
| | Same start, same game | identical | identical | yes |
| | Against no orders, Allied: median margin (b) against (c) | 1.05 against 0.20 | 0.25 against 0.20 | yes |
| | Against no orders, Japanese: median margin (d) against (e) | 0.00 against 0.00 | **-5.40 against 0.00** | **no** |
| The week | Every invariant, every seed | 1,000 | 1,000 | yes |
| | No staff order or command refused; no commander's command of day 15 refused | 0; none | 0; none | yes |
| | A search and a patrol on 5 of 7 days, every seed: Allied | 1,000 | 1,000 | yes |
| | the same: Japanese | 683 | 1,000 | yes |
| | A strike or raid on 800 or more seeds (Allied, Japanese) | 997, 1,000 | 1,000, 1,000 | yes |
| | Supply landed at its base on 800 or more seeds (Allied, Japanese) | 1,000, 1,000 | 1,000, 1,000 | yes |
| | Same start, same game | identical | identical | yes |
| | Against no orders, Allied (3 no worse, 1 better) | passes | passes | yes |
| | Against no orders, Japanese (3 no worse, 1 better) | passes | passes | yes |
| Legibility | 20 of 24 pass both, no doctrine all fail | 24 of 24 | 24 of 24 | yes |

The script's own figures in (f) are the same as the first report's, 741 Allied and 814 Japanese seeds with a strike: decisions 0130 and 0131 change only what the staff does, and the scenario has no staff in (f).

## The files and how they were measured

- **Variant files:** `data/scenarios/second-proof/`, as committed for the first run. Every file loads on this main, and neither scenario 5 nor scenario 7 has changed since the files were made, so none was changed.
- **The six arrangements** are the first report's: (a) both from intents, (b) Allied intents against the Japanese script, (c) Allied given nothing against the Japanese script, (d) and (e) the same for the Japanese side, (f) the script.
- **How each arrangement was run:** as the first run did, by the same scripts:
  - `sim-cli batch` twice over seeds 1 to 1,000;
  - `sim-cli export` of every seed, read by `measure.py`, and summed by `summarise.py`.
- **What was changed in the scripts.** They were copied to a new folder outside the repository, beside the first run's, and the originals were left as they were. The one change is the path of the worktree they run in (`W` in `measure.py` and `build.py`, `W` and the output folder in `run_all.sh`). No line that reads an event, a state or a batch file was changed: the export's format and the events the script reads are unchanged by state v34, and the new order `set_search_once` is not one the script reads. A short script of a dozen lines (`pulse.py`) was added to count the first strikes in the same pulse as the script's, which the first run counted by hand from the same lines.

**The same start gives the same game.** For all twelve arrangements:
- the two batch files of 1,000 seeds are byte for byte the same;
- the exports of seeds 1, 2, 3, 500 and 1,000, made twice, are byte for byte the same.

## First: the Eastern Solomons, 24 August 1942

### The conditions, arrangement (a), both sides from intents

| Condition | Allied | Japanese | Holds |
| --- | --- | --- | --- |
| Every invariant holds, on every seed | 1,000 of 1,000 runs complete | | yes |
| No staff order or command refused, on every seed | 0 refused | 0 refused | yes |
| A search and a patrol, on every seed | 1,000 | 1,000 | yes |
| At least one strike at ships, on 800 or more seeds | 558 | 865 | **no** (Allied) |
| The same start and seed give the same game | identical | | yes |

The 442 seeds on which the Allied side launched no strike:

> 1, 3, 4, 5, 6, 9, 10, 11, 12, 13, 14, 17, 19, 21, 22, 23, 26, 27, 32, 35, 36, 40, 43, 44, 45, 46, 55, 56, 57, 60, 62, 64, 65, 66, 69, 73, 78, 86, 87, 88, 92, 94, 98, 99, 104, 107, 108, 109, 110, 113, 115, 122, 123, 125, 126, 128, 132, 133, 137, 139, 145, 149, 150, 152, 154, 161, 162, 166, 169, 170, 174, 176, 177, 179, 180, 181, 183, 184, 186, 187, 188, 190, 194, 197, 199, 202, 204, 207, 209, 214, 217, 220, 232, 236, 237, 238, 239, 240, 246, 250, 256, 257, 258, 260, 265, 266, 267, 268, 269, 271, 273, 279, 281, 284, 285, 288, 289, 290, 291, 295, 296, 298, 299, 300, 302, 312, 318, 320, 321, 322, 327, 329, 333, 334, 335, 339, 341, 342, 345, 348, 350, 352, 354, 358, 359, 363, 364, 366, 371, 373, 374, 375, 376, 385, 388, 389, 391, 394, 395, 400, 401, 402, 405, 411, 412, 413, 417, 418, 420, 429, 430, 431, 432, 433, 434, 440, 441, 445, 446, 448, 449, 450, 457, 459, 463, 466, 470, 471, 473, 475, 477, 478, 480, 484, 485, 486, 489, 492, 493, 496, 497, 498, 500, 501, 503, 505, 506, 507, 509, 510, 516, 517, 519, 520, 523, 525, 526, 529, 531, 532, 533, 534, 535, 536, 538, 539, 541, 544, 547, 550, 553, 555, 557, 558, 559, 561, 563, 564, 566, 569, 570, 573, 575, 583, 585, 588, 590, 592, 593, 594, 599, 605, 608, 609, 611, 612, 614, 615, 616, 617, 618, 621, 622, 625, 629, 633, 634, 635, 637, 638, 639, 641, 646, 648, 649, 650, 656, 658, 662, 663, 664, 666, 667, 668, 674, 677, 680, 682, 684, 685, 687, 691, 692, 695, 699, 702, 704, 706, 707, 709, 710, 711, 713, 714, 715, 717, 718, 721, 722, 727, 728, 730, 731, 732, 733, 735, 740, 741, 742, 745, 748, 749, 751, 752, 753, 758, 761, 769, 773, 775, 776, 777, 778, 781, 790, 799, 800, 801, 803, 805, 811, 812, 814, 815, 816, 817, 821, 824, 825, 826, 828, 829, 830, 831, 834, 835, 837, 838, 841, 843, 844, 845, 851, 856, 857, 858, 859, 860, 862, 863, 864, 865, 866, 870, 872, 873, 874, 875, 879, 881, 883, 884, 888, 892, 894, 898, 901, 903, 905, 907, 908, 909, 910, 911, 913, 914, 918, 921, 922, 927, 928, 929, 930, 932, 934, 938, 942, 943, 945, 946, 947, 949, 950, 952, 953, 954, 955, 956, 959, 960, 965, 968, 969, 971, 972, 973, 974, 975, 976, 978, 979, 983, 984, 985, 986, 987, 988, 989, 990, 991, 993, 1000

**Why, from the staff's reasons and the log (seed 1 read in full):**

1. **The Japanese carriers now come up overnight.** By decision 0130, rule 1, a force that would not reach its hex within 12 hours at cruise goes at full speed. On seed 1 task force 3 is at (8,-11) and task force 4 at (7,-9) by the morning, and their floatplanes find both American carriers that morning. Every staff first strike of either side on every seed now goes in the morning; the script's go in the afternoon.
2. **The American carriers search once, in the morning, and find nothing.** By decision 0131 a group searches once a day, in the morning, at the balanced risk ("at balanced risk a search is flown once a day, in the morning", in the reason for each search order). On seed 1 the carriers' searches (groups 2 and 6) miss task force 3 at 223 nm (chances 0.24 and 0.21) and task force 4 at 160 nm (0.34 and 0.30). They fly no search in the afternoon.
3. **The one contact the side holds is beyond reach, and the carriers do not close on it in the day.** Henderson Field's search (group 10, the cover's) reports task force 3 at (8,-11) in the morning, 223 nm from task forces 16 and 11 at (7,-5). The reach is the least strike radius aboard, the Avenger's 190 nm. The carrier doctrine weighs closing once, on the staff's morning, before any pulse; that morning the side held no contact, and the reasons of both forces say "does not close: the side holds no contact of ships 3 pulses old or less". The game is one day long, so the closing is never weighed again. The carriers stay in (7,-5) with their bombers on the strike mission and nothing in reach.
4. On the 558 seeds on which the Allied side strikes, its first strike is in the morning on every one. That a morning search finding a force within 190 nm is what makes the difference was read from seed 1 alone and is not confirmed on the other seeds.

**Against the script, reported and not judged:**

- The script strikes on 741 seeds (Allied) and 814 (Japanese).
- First strike in the same pulse as the script's:

| Arrangement | Side | Seeds where both struck | Same pulse |
| --- | --- | --- | --- |
| (a) | Allied | 409 | 0 |
| (a) | Japanese | 723 | 0 |
| (b) | Allied | 0 (the staff struck on no seed) | |
| (b) | Japanese (script) | 814 | 814 |
| (d) | Japanese | 723 | 0 |
| (d) | Allied (script) | 666 | 128 |

- Every staff first strike went in the morning, and every script's in the afternoon. In (d) the scripted Allied side strikes in the morning on 721 seeds, when the Japanese carriers come up overnight and its morning search finds them.

### Against no orders, judged

| Side | Median margin from intents | Median margin from nothing | Passes |
| --- | --- | --- | --- |
| Allied: (b) against (c) | 0.25 | 0.20 | yes |
| Japanese: (d) against (e) | -5.40 | 0.00 | **no** |

Reported beside them (medians):

| Arrangement | Allied aircraft lost | Allied ships sunk | Allied out of action | Japanese aircraft lost | Japanese ships sunk | Japanese out of action |
| --- | --- | --- | --- | --- | --- | --- |
| (a) | 8 | 0 | 1 | 34 | 0 | 0 |
| (b) | 3 | 0 | 1 | 19 | 0 | 0 |
| (c) | 0 | 0 | 1 | 11 | 0 | 0 |
| (d) | 8 | 0 | 1 | 50 | 0 | 0 |
| (e) | 0 | 0 | 0 | 0 | 0 | 0 |
| (f), the script | 5 | 0 | 1 | 29 | 0 | 0 |

In (d) the median of Japanese ships sunk or out of action together is 1, though each median alone is 0.

**Why the Allied side's margin fell, and it still passes.** In (b) the Allied staff launches no strike on any seed. On seed 1, read in full, the scripted Japanese carriers are 280 to 302 nm off in the morning, when the American carriers fly their one search, and come into reach in the afternoon, when the carriers no longer search (seed 1: "air_group:2 searched for task_force:3 at 302 nm ... not sighted"; Henderson's search reports task force 3 at (8,-13), beyond reach). The Japanese strike the American carriers in the afternoon as the script has them. The difference from (c) is small: Japanese aircraft lost 19 against 11, Allied 3 against 0. Which objective carries the 0.05 was not read.

**Why the Japanese side fails it.**
- In (e) the Japanese carriers do not move at all, nobody finds anybody on any seed, and the margin is 0.
- In (d) the Japanese carriers come up overnight at full speed (decision 0130), and are found and struck in the morning by the scripted American carriers on 721 seeds. The Japanese side strikes on 865 seeds but loses a median of 50 aircraft, against 8 American, and the Allied side wins on 665 of the 1,000 seeds.
- So the Japanese side from its intents does worse than given nothing, -5.40 against 0.00. This comes from the speed of the approach and the exchange of strikes that follows; nothing on the Japanese side refused or failed.

## Second: the week of the slice, days 15 to 21

### The conditions, arrangement (a)

| Condition | Allied | Japanese | Holds |
| --- | --- | --- | --- |
| Every invariant holds, on every seed | 1,000 of 1,000 runs complete | | yes |
| No staff order or command refused, on every seed | 0 refused; no commander's command of day 15 refused on any seed | | yes |
| A search and a patrol on at least 5 of the 7 days, on every seed | 1,000 | 1,000 | yes |
| A strike or raid in the week, on 800 or more seeds | 1,000 | 1,000 | yes |
| Supply landed at its base in the week, on 800 or more seeds | 1,000 | 1,000 | yes |
| The same start and seed give the same game | identical | | yes |

- Both sides fly a search and a patrol on all 7 days on every seed.
- **Why the Japanese search now holds.** Its searchers are still the two floatplane groups, 117 and 120. By decision 0131 they search once a day, in the morning, and rest in the afternoon. On seed 3, which failed in the first run, group 117's reason on day 17 reads "has fatigue 0 not above the 45 of the balanced risk: it searches 090 to 260 out to 398 nm ... at balanced risk a search is flown once a day, in the morning", where the first run's read "fatigue 75.68 above the 45 of the balanced risk: it rests". It searches each morning of days 15 to 21.
- The scripted orders rejected in (b), (c) and (f) are the day-19 order of the wrong side's force kept in scenario 7 for coverage (decision 0043), one on each seed, counted apart and not judged, as in the first run.

### Against no orders, judged

The medians over the week, days 15 to 21.

**Allied: (b), intents against the Japanese script, beside (c), given nothing. It passes.**

| Measure | From intents | From nothing | No worse | Better | First run |
| --- | --- | --- | --- | --- | --- |
| Henderson Field open to its holder | 7 | 7 | yes | no | 7 against 7 |
| Supply landed at Henderson Field | 3,220.8 t | 2,000.0 t | yes | yes | 3,220.8 t against 2,000.0 t |
| Aircraft lost | 36 | 1 | **no** | no | 24 against 1 |
| Ships sunk or out of action | 4 | 4 | yes | no | 3 against 3 |
| Objectives margin (reported) | 61.38 | 47.66 | | | 56.05 against 47.59 |

**Japanese: (d), intents against the Allied script, beside (e), given nothing. It passes.**

| Measure | From intents | From nothing | No worse | Better | First run |
| --- | --- | --- | --- | --- | --- |
| Henderson Field open to its holder | -7 | -7 | yes | no | -7 against -7 |
| Supply landed at Taivu and west of the Lunga | 119.08 t | 0 t | yes | yes | 119.08 t against 0 t |
| Aircraft lost | 31 | 0 | **no** | no | 29 against 0 |
| Ships sunk or out of action | 0 | 0 | yes | no | 0 against 0 |
| Objectives margin (reported) | -52.41 | -51.32 | | | -52.40 against -51.36 |

**Against the script, reported and not judged:** medians, days 15 to 21.

| Arrangement | Allied margin | Allied aircraft lost | Allied sunk or out of action | Supply at Henderson | Japanese aircraft lost | Japanese sunk or out of action | Japanese supply landed | Airfield open (Allied) |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| (a) both from intents | 53.66 | 20 | 0 | 3,220.8 t | 25 | 1 | 119.08 t | 7 |
| (f) the script | 63.38 | 26 | 4 | 2,960.0 t | 71 | 3 | 0 t | 7 |

The script's own week moved since the first run ((f): Allied margin 61.03, aircraft lost 23 and 61): days 1 to 14 and the scripted week are played under the live fatigue figures, to which decision 0131 added the search figure.

**Limits named beside the result** are the first report's and are unchanged: no carrier station chosen by the staff; no home moved, so no stand task in the week; no carrier raid; wave groups played as they are; the men left aboard under the ruling on reading 6.

## Legibility

The set was picked by the same rule (`pick.py`, unchanged) from the rerun's exports: the week's arrangement (a), seed 1, and scenarios 53 and 54 as they stand. The set, the key, the answers and the marks are in `crates/sim-cli/tests/recorded_games/legibility_second_proof_rerun/`, beside the first run's.
- **The set.** 21 of the 24 reasons are word for word the first run's. Reason 4 (the cover of Lunga, day 16) and reasons 14 and 16 (a carrier force's day with no order) differ: the once-a-day search and the words on closing were added. Several reasons' event numbers moved, with the same text.
- **The key** is the first run's, with a note on reasons 4, 14 and 16 written from the rerun's reasons before any reader ran. The rows stand.
- **The readers:** one Haiku agent per reason, given the reason's file alone and the two questions. Three readers made one stray call of the session's own tools before answering (reasons 6, 10 and 24); none read another file.
- **The marking:** by a Sonnet agent against the key.

| Doctrine | From | Pass both | First run |
| --- | --- | --- | --- |
| Cover | the week (a), seed 1 | 6 of 6 | 6 of 6 |
| Raid | the week (a), seed 1 | 6 of 6 | 6 of 6 |
| Carrier | the week (a), seed 1 | 4 of 4 | 4 of 4 |
| Strike | scenario 53, placeholder | 4 of 4 | 4 of 4 |
| Stand | scenario 54, placeholder | 4 of 4 | 4 of 4 |
| All | | 24 of 24 | 24 of 24 |

**It passes.** The marker noted factual slips that changed no mark: reasons 4, 6 and 10 say orders were given where the reason gives none; reason 5 opens with "keep air group 11 resting" before saying it was set to search. Reason 4 is still about 52 KB long, almost all of it the list of hexes of the waters to deny.

## A model's game

Not played again. The plan asks for one game, "once, and its account is kept"; it is not judged, and nothing in the plan asks for it again on a rerun. The recorded game `crates/sim-cli/tests/recorded_games/second_proof_week_sonnet/` was regenerated under decision 0131 with `regame.py` (its orders and commands replayed, the staff's decisions and reasons remade); the model was not asked again, and its account stands as the first run's.

## Speed

`sim-cli time --machine "Apple M3 Pro, 36 GB, macOS (the architect's laptop)" --slice data/scenarios/second-proof/07a_week.toml --early-day 15 --late-day 18`, release build, both staffs at work from day 15. The workloads are decision 0100's.

| Workload | Median | First run |
| --- | --- | --- |
| One day of the slice | 14.0 ms (day 16), 9.56 ms (day 19) | 13.2 ms, 8.79 ms |
| The staff's decisions for one side, longest day | Allied 4.08 ms (day 15); Japanese 7.94 ms (day 17) | 4.08 ms; 7.42 ms |
| Six days in one call from day 15 | 71.1 ms (about 11.9 ms a day) | 68.1 ms |
| A day with every question asked at every moment | 29.5 ms early, 24.7 ms late | 27.6 ms, 23.8 ms |

**The staffs' share of a day** is as before: up to about half of a day's time on the morning the intents are stated and on the day of the carrier battle (4 to 8 ms of 10 to 14 ms), well under a millisecond on a quiet day.

## What it puts to the designer

Each failure raises a choice of one of three: a doctrine to change, a limit of the stage to lift, or a miss to name.

1. **A carrier force whose one search finds nothing has no strike that day.**
   - The search once a day, in the morning, at the balanced risk (decision 0131) leaves the American carriers with no afternoon search, and the closing is weighed only on the staff's morning, before any contact is held (decision 0130).
   - In a one-day battle neither is weighed again, so the Allied side strikes on 558 seeds and, against the script, on none.
2. **Coming up at full speed costs the Japanese side the exchange.** From intents its carriers are found in the morning and lose more than they take, and do worse than staying put (-5.40 against 0.00). Whether the comparison against no orders is a fair test of a side on a one-day battle in which doing nothing scores 0 is the designer's to say; the condition is reported as failed as stated.
3. **What the two decisions mended:** the Japanese strike (152 to 865 seeds) and the Japanese search in the week (683 to 1,000 seeds).
