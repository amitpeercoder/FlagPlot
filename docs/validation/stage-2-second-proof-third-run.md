# Step 3, stage 2: the second proof, run a third time

Run on Nov 16, 2026, by the team lead of the third run, to the test plan of decision 0128 as it stands with the architect's rulings made before the first run ("The intents as stated" and "Restated before the legibility test was run" in that record). The first run's report, `docs/validation/stage-2-second-proof.md`, and the rerun's, `docs/validation/stage-2-second-proof-rerun.md`, are kept as they were.

**One change since the rerun, made after seeing its result.** Decision 0131, rule 6, was amended on Nov 16, 2026, on the designer's choice, after the rerun showed the American carriers searching in the morning only: a carrier force now searches in both daylight pulses at the balanced risk, and once a day only at preserve the force. A base's cover and its search of waters are unchanged. This is a change made after a result of this proof was seen, and this run measures it.

Release build, live figures, seeds 1 to 1,000, doctrine `supply-17`, state layout `flagplot-state-v34`, from main at the merge of pull request 382. Nothing was tuned, and no pass mark, intent, measure or variant file was changed.

**Result: the proof does not pass as stated.**
- **The Eastern Solomons fails** on the same two conditions as in the rerun.
  - **The strike condition, for the Allied side.** Both sides from intents, the Allied side launches a strike at ships on 767 seeds, against the 800 asked (558 in the rerun). The Japanese side passes it, on 979.
  - **The comparison against no orders, for the Japanese side.** From its intents its median margin is -5.50, against 0.00 given nothing. The Allied side passes, 0.95 against 0.20.
- **The week passes every condition.**
- **Everything else holds:**
  - every invariant;
  - no staff order and no command refused, on any seed;
  - the same start gives the same game;
  - legibility, 24 of 24, carried over: every picked reason is word for word one the rerun's readers read and marked.

Each failure is reported below with its seeds and the reasons the staff gave. Nothing was mended.

## The three runs side by side

| Part | Condition | First run (`supply-14`) | Rerun (`supply-16`) | Third run (`supply-17`) | Holds now |
| --- | --- | --- | --- | --- | --- |
| Eastern Solomons | Every invariant, every seed | 1,000 | 1,000 | 1,000 | yes |
| | No staff order or command refused | 0 refused | 0 refused | 0 refused | yes |
| | A search and a patrol, every seed (Allied, Japanese) | 1,000, 1,000 | 1,000, 1,000 | 1,000, 1,000 | yes |
| | A strike at ships on 800 or more seeds: Allied | 0 | 558 | **767** | **no** |
| | the same: Japanese | 152 | 865 | 979 | yes |
| | The script's own strikes, (f), for reference (Allied, Japanese) | 741, 814 | 741, 814 | 741, 814 | (not judged) |
| | Same start, same game | identical | identical | identical | yes |
| | Against no orders, Allied: median margin (b) against (c) | 1.05 against 0.20 | 0.25 against 0.20 | 0.95 against 0.20 | yes |
| | Against no orders, Japanese: median margin (d) against (e) | 0.00 against 0.00 | -5.40 against 0.00 | **-5.50 against 0.00** | **no** |
| The week | Every invariant, every seed | 1,000 | 1,000 | 1,000 | yes |
| | No staff order or command refused; no commander's command of day 15 refused | 0; none | 0; none | 0; none | yes |
| | A search and a patrol on 5 of 7 days, every seed: Allied | 1,000 | 1,000 | 1,000 | yes |
| | the same: Japanese | 683 | 1,000 | 1,000 | yes |
| | A strike or raid on 800 or more seeds (Allied, Japanese) | 997, 1,000 | 1,000, 1,000 | 1,000, 1,000 | yes |
| | Supply landed at its base on 800 or more seeds (Allied, Japanese) | 1,000, 1,000 | 1,000, 1,000 | 1,000, 1,000 | yes |
| | Same start, same game | identical | identical | identical | yes |
| | Against no orders, Allied (3 no worse, 1 better) | passes | passes | passes | yes |
| | Against no orders, Japanese (3 no worse, 1 better) | passes | passes | passes | yes |
| Legibility | 20 of 24 pass both, no doctrine all fail | 24 of 24 | 24 of 24 | 24 of 24 (carried over) | yes |

The script's own figures in (f) are the same in all three runs, 741 Allied and 814 Japanese seeds with a strike: the amendment changes only what the staff does, and (f) has no staff at work.

## The files and how they were measured

- **Variant files:** `data/scenarios/second-proof/`, as committed for the first run. Every file loads on this main, and neither scenario 5 nor scenario 7 has changed since the files were made, so none was changed.
- **The six arrangements** are the first report's: (a) both from intents, (b) Allied intents against the Japanese script, (c) Allied given nothing against the Japanese script, (d) and (e) the same for the Japanese side, (f) the script.
- **How each arrangement was run:** as the earlier runs did, by the same scripts:
  - `sim-cli batch` twice over seeds 1 to 1,000;
  - `sim-cli export` of every seed, read by `measure.py`, and summed by `summarise.py`; `pulse.py` for the first strike's pulse.
- **What was changed in the scripts.** They were copied to a new folder outside the repository, beside the rerun's, and the rerun's were left as they were. The one change is the path of the worktree they run in (`W` in `measure.py` and `build.py`, `W` and the output folder in `run_all.sh`). No line that reads an event, a state or a batch file was changed.
- **Added, for the Allied strike's cause:** `nostrike.py`, which exports each seed on which a side launched no strike and puts it in a class (below), and `validate.py`, which runs the same classing on seeds where the side did strike. On the first 100 seeds of (a) on which the Allied side struck, it puts all 100 in the class "an eligible contact within reach", as it should. The classing reads the launch rule from outside the code: the contacts the side holds of ships reported in the pulse or the one before (at the hex of the last report), the groups on the strike mission with ready aircraft aboard a carrier whose flight deck is not out, read from the state at the start of the pulse, and the distance from the group's force to the contact against the group type's strike radius.

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
| At least one strike at ships, on 800 or more seeds | 767 | 979 | **no** (Allied) |
| The same start and seed give the same game | identical | | yes |

What the amendment changed: the Allied first strikes in the morning are on 558 seeds, as in the rerun, and the afternoon search adds 209 seeds whose first strike is in the afternoon. The Japanese side strikes in the morning on 865 seeds and in the afternoon on 114 more.

The 233 seeds on which the Allied side launched no strike:

> 1, 3, 6, 9, 10, 11, 12, 13, 14, 26, 27, 35, 43, 55, 56, 60, 62, 69, 78, 87, 88, 92, 98, 99, 104, 107, 108, 109, 122, 123, 126, 133, 137, 139, 150, 152, 162, 166, 170, 174, 179, 181, 186, 187, 188, 190, 199, 202, 204, 207, 217, 236, 237, 240, 256, 257, 260, 266, 269, 271, 273, 281, 284, 288, 291, 296, 298, 299, 318, 322, 327, 329, 333, 334, 341, 352, 354, 358, 364, 371, 373, 375, 376, 385, 389, 395, 400, 402, 411, 417, 418, 420, 433, 445, 446, 449, 459, 473, 475, 477, 478, 485, 489, 493, 501, 505, 507, 509, 510, 520, 526, 529, 531, 532, 533, 534, 535, 539, 547, 555, 557, 558, 559, 564, 566, 570, 573, 583, 590, 599, 608, 609, 611, 612, 614, 615, 616, 622, 625, 629, 633, 637, 638, 641, 650, 656, 658, 662, 664, 666, 674, 682, 684, 704, 710, 711, 714, 715, 718, 722, 727, 728, 730, 731, 732, 733, 735, 745, 749, 751, 752, 758, 769, 790, 800, 801, 803, 805, 816, 826, 828, 829, 830, 835, 838, 841, 844, 856, 857, 858, 859, 862, 863, 866, 870, 872, 873, 874, 875, 879, 881, 883, 884, 888, 894, 898, 907, 921, 927, 928, 929, 934, 943, 947, 949, 952, 953, 955, 956, 959, 960, 968, 969, 972, 973, 974, 975, 978, 983, 986, 987, 989, 990

**What stopped the strike, counted over all 233 seeds from the exports** (by `nostrike.py`):

| What stopped it | Seeds |
| --- | --- |
| No contact held: the side held no contact of ships at any time on the day | 64 |
| Contact beyond reach: a contact held in a daylight pulse, but at more than every ready striking group's radius (200 nm for the Dauntless, 190 nm for the Avenger) from the carriers | 169 |
| of which: a report within reach was replaced, in the same pulse, by a later report of the same contact at a farther hex | 58 |
| Contact too old: within reach, but reported two pulses or more before | 0 |
| No group able to launch while a contact was held | 0 |
| Something else: a contact eligible and within reach, and still no strike | 0 |

Of the 169 seeds beyond reach, the nearest the contact came to a ready group was 223 nm on 134 (the carrier striking force at (8,-11), the hex the Japanese commander's intent sends it to), and 212 to 262 nm on the rest.

**Why, from the staff's reasons and the log (seeds 1, 3 and 12 read in full):**

1. **The carriers do not close.** On all three seeds both carrier forces' reasons on the staff's morning say "does not close: the side holds no contact of ships 3 pulses old or less", and the forces stay in (7,-5) all day. The closing is weighed only then (decision 0130), before any pulse, and the game is one day long. This is the same as in the rerun; the amendment does not touch it.
2. **The carriers now search in both pulses** ("at balanced risk a carrier force searches in both daylight pulses", in each searcher's reason). On seed 3 groups 2 and 6 search in the afternoon and miss both Japanese forces (chances 0.21 to 0.34). On seeds 1 and 12, Enterprise's flight deck is put out by the Japanese morning strike, and only Saratoga's group 6 searches in the afternoon (28 and 29 aircraft, chances 0.40 to 0.58), and misses.
3. **Seed 1, beyond reach.** Henderson Field's search (group 10, the cover's) reports the carrier striking force at (8,-11) in the morning, 223 nm from the carriers, beyond the Dauntless's 200 nm. Nothing else is found all day.
4. **Seed 3, a report in reach replaced.** In the morning group 2 sights the Ryujo detachment at (7,-9), 160 nm, a new contact 1. Then Henderson Field's search sights the carrier striking force and "refreshed contact:1" at (8,-11), 223 nm. The side holds one contact, at the farther hex, and the strike rule finds nothing in reach, in the morning or the afternoon. On 58 seeds a report within reach was so replaced; the side's two sightings of two forces became one contact.
5. **Seed 12, no contact.** Neither carrier's search nor Henderson Field's sights anything in the morning (chances 0.13 to 0.34); the afternoon search of group 6 misses both forces.

The counts in the table are over all 233 seeds. The reasons (points 1 and 2) were read from three seeds; that the staff's reasons say the same on the other seeds was not read.

**Against the script, reported and not judged:**

- The script strikes on 741 seeds (Allied) and 814 (Japanese), its first strike in the afternoon on every one.
- First strike in the same pulse as the script's:

| Arrangement | Side | Seeds where both struck | Same pulse |
| --- | --- | --- | --- |
| (a) | Allied | 560 | 151 |
| (a) | Japanese | 806 | 83 |
| (b) | Allied | 392 (the staff struck on 528, all in the afternoon) | 392 |
| (b) | Japanese (script) | 814 | 814 |
| (d) | Japanese | 806 | 83 |
| (d) | Allied (script) | 666 | 128 |

- In the rerun the staff's first strikes were all in the morning; now those in the afternoon match the script's pulse.

### Against no orders, judged

| Side | Median margin from intents | Median margin from nothing | Passes | Rerun |
| --- | --- | --- | --- | --- |
| Allied: (b) against (c) | 0.95 | 0.20 | yes | 0.25 against 0.20 |
| Japanese: (d) against (e) | -5.50 | 0.00 | **no** | -5.40 against 0.00 |

Reported beside them (medians):

| Arrangement | Allied aircraft lost | Allied ships sunk | Allied out of action | Japanese aircraft lost | Japanese ships sunk | Japanese out of action |
| --- | --- | --- | --- | --- | --- | --- |
| (a) | 10 | 0 | 1 | 47 | 0 | 0 |
| (b) | 4 | 0 | 1 | 25 | 0 | 0 |
| (c) | 0 | 0 | 1 | 11 | 0 | 0 |
| (d) | 9 | 0 | 1 | 56 | 0 | 0 |
| (e) | 0 | 0 | 0 | 0 | 0 | 0 |
| (f), the script | 5 | 0 | 1 | 29 | 0 | 0 |

In (a) and (d) the median of Japanese ships sunk or out of action together is 1, though each median alone is 0. The Ryujo is sunk on 415 seeds of (a), 452 of (d) and 249 of the script.

**Why the Allied side's margin rose, and it passes.** In (b) the staff now strikes on 528 seeds (0 in the rerun), every one in the afternoon, when the scripted Japanese carriers have come into reach and the afternoon search finds them.

**Why the Japanese side fails it (seeds 1, 2 and 3 of (d) read in full).**
- In (e) the Japanese carriers do not move, nobody finds anybody on any seed, and the margin is 0 on all 1,000.
- In (d) the Japanese carriers come up overnight at full speed (decision 0130) to the hexes the commander's intents name; the Ryujo detachment at (7,-9) is 160 nm from the American carriers, inside their reach. On seeds 1 and 3 the scripted American search finds it in the morning; on seed 2 in the afternoon. On all three the Ryujo (ship 33) is sunk. The Japanese strikes hit the American carriers too (Enterprise's flight deck out on seed 1), but lose more aircraft: seed 1, 59 Japanese to 18 American.
- Over the 1,000 seeds of (d) the Allied side wins on 675, the Japanese on 289, and 36 are even. So the Japanese side from its intents does worse than given nothing. The cause is the station the intents name and the exchange that follows; nothing on the Japanese side refused or failed. It was read from three seeds, and is counted over all 1,000 only in the Ryujo's sinking and the wins.

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

The scripted orders rejected in (b) and (c) are the day-19 order of the wrong side's force kept in scenario 7 for coverage (decision 0043), one on each seed, counted apart and not judged, as in the earlier runs.

### Against no orders, judged

The medians over the week, days 15 to 21.

**Allied: (b), intents against the Japanese script, beside (c), given nothing. It passes.**

| Measure | From intents | From nothing | No worse | Better | Rerun |
| --- | --- | --- | --- | --- | --- |
| Henderson Field open to its holder | 7 | 7 | yes | no | 7 against 7 |
| Supply landed at Henderson Field | 3,220.8 t | 2,000.0 t | yes | yes | 3,220.8 t against 2,000.0 t |
| Aircraft lost | 39 | 1 | **no** | no | 36 against 1 |
| Ships sunk or out of action | 4 | 4 | yes | no | 4 against 4 |
| Objectives margin (reported) | 62.36 | 47.66 | | | 61.38 against 47.66 |

**Japanese: (d), intents against the Allied script, beside (e), given nothing. It passes.**

| Measure | From intents | From nothing | No worse | Better | Rerun |
| --- | --- | --- | --- | --- | --- |
| Henderson Field open to its holder | -7 | -7 | yes | no | -7 against -7 |
| Supply landed at Taivu and west of the Lunga | 119.08 t | 0 t | yes | yes | 119.08 t against 0 t |
| Aircraft lost | 31 | 0 | **no** | no | 31 against 0 |
| Ships sunk or out of action | 0 | 0 | yes | no | 0 against 0 |
| Objectives margin (reported) | -52.43 | -51.32 | | | -52.41 against -51.32 |

**Against the script, reported and not judged:** medians, days 15 to 21.

| Arrangement | Allied margin | Allied aircraft lost | Allied sunk or out of action | Supply at Henderson | Japanese aircraft lost | Japanese sunk or out of action | Japanese supply landed | Airfield open (Allied) |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| (a) both from intents | 53.42 | 24 | 0 | 3,220.8 t | 26 | 1 | 119.08 t | 7 |
| (f) the script | 63.38 | 26 | 4 | 2,960.0 t | 71 | 3 | 0 t | 7 |

(f) is the rerun's to the figure.

**Limits named beside the result** are the first report's and are unchanged: no carrier station chosen by the staff; no home moved, so no stand task in the week; no carrier raid; wave groups played as they are; the men left aboard under the ruling on reading 6.

## Legibility

The set was picked by the same rule (`pick.py` of the rerun, unchanged) from this run's exports: the week's arrangement (a), seed 1, and scenarios 53 and 54 as they stand. **Every one of the 24 picked reasons is word for word, in all five parts, the reason of the same number that the rerun's readers read and its marker marked** (`crates/sim-cli/tests/recorded_games/legibility_second_proof_rerun/`). The carrier reasons picked are the force's readiness, its rules of engagement and its home port, whose words the amendment does not change. Four raid reasons have other event numbers, with the same text. So no reader was asked again, and the rerun's result is carried over: **24 of 24, every doctrine all of its own. It passes.**

## A model's game

Not played again, as the plan asks for one game.

## Speed

`sim-cli time --machine "Apple M3 Pro, 36 GB, macOS (the architect's laptop)" --slice data/scenarios/second-proof/07a_week.toml --early-day 15 --late-day 18`, release build, both staffs at work from day 15. The workloads are decision 0100's.

| Workload | Median | Rerun |
| --- | --- | --- |
| One day of the slice | 12.9 ms (day 16), 8.68 ms (day 19) | 14.0 ms, 9.56 ms |
| The staff's decisions for one side, longest day | Allied 3.57 ms (day 15); Japanese 7.26 ms (day 17) | 4.08 ms; 7.94 ms |
| Six days in one call from day 15 | 67.0 ms (about 11.2 ms a day) | 71.1 ms |
| A day with every question asked at every moment | 26.9 ms early, 24.0 ms late | 29.5 ms, 24.7 ms |

**The staffs' share of a day** is as before: up to about half of a day's time on the morning the intents are stated and on the day of the carrier battle, well under a millisecond on a quiet day.

## What it puts to the designer

Each failure raises a choice of one of three: a doctrine to change, a limit of the stage to lift, or a miss to name.

1. **The Allied strike, 767 of 800.** The afternoon search raised it from 558. What is left, counted over all 233 seeds: no contact found all day on 64; a contact held but beyond reach on 169, the carriers not closing because the closing is weighed only before the day's first pulse; and on 58 of those 169 a sighting within reach of one force was overwritten by a farther sighting of another as one contact.
2. **The Japanese comparison, -5.50 against 0.00.** Unchanged in kind from the rerun: the stated station brings the Ryujo detachment into American reach, and the exchange is lost. Whether the comparison against no orders is a fair test on a one-day battle in which doing nothing scores 0 is the designer's to say; it is reported as failed as stated.
