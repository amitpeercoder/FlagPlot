# Step 3, stage 2: the second proof, as run

Run on Nov 16, 2026, by the team lead of step 7, to the test plan of decision 0128 as it stands with the architect's rulings made before any judged run ("The intents as stated" and "Restated before the legibility test was run" in that record). Release build, live figures, seeds 1 to 1,000, doctrine `supply-14`, state layout `flagplot-state-v33`. Nothing was tuned, and no pass mark, intent or measure was changed after a run.

**Result: the proof does not pass as stated.**
- **The Eastern Solomons fails** on the strike condition for both sides. The Japanese side from its intents does no better than given nothing on the objectives margin.
- **The week passes every condition but one:** the Japanese side searches on fewer than 5 of the 7 days on 317 seeds.
- **Everything else holds:**
  - every invariant;
  - no staff order and no command refused, on any seed;
  - the same start gives the same game;
  - legibility, 24 of 24;
  - the week's comparison against no orders, for both sides;
  - the Eastern Solomons' comparison for the Allied side.

Each failure is reported below with its seeds and the reasons the staff gave. Nothing was mended.

## The files and how they were measured

- **Variant files:** `data/scenarios/second-proof/`: `05a` to `05e` and the scenario itself for (f); `07a` to `07f` for the week.
- **The model's game:** `07m_model_week.toml`.
- **The six arrangements:**

| | Allied | Japanese |
| --- | --- | --- |
| (a) | intents | intents |
| (b) | intents | scripted |
| (c) | nothing | scripted |
| (d) | scripted | intents |
| (e) | scripted | nothing |
| (f) | scripted | scripted |

- **How each arrangement was run:**
  - `sim-cli batch` twice over seeds 1 to 1,000, for the margins and the check that the same start gives the same game.
  - `sim-cli export` of every seed, read by the measuring script on the rules decision 0128 writes down ("How the conditions are read").
- **Where the scripts are:** the measuring scripts (`measure.py`, `summarise.py`) and the generators of the files (`gen.py`, `build.py`) were kept outside the repository, in the session's scratch folder. The variant files they made are committed.

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
| At least one strike at ships, on 800 or more seeds | 0 | 152 | **no** |
| The same start and seed give the same game | identical | | yes |

The 152 Japanese seeds with a strike:

> 2, 3, 4, 6, 7, 25, 27, 39, 40, 50, 51, 54, 56, 67, 68, 78, 84, 92, 95, 98, 102, 104, 108, 119, 126, 130, 134, 136, 141, 150, 153, 154, 167, 170, 172, 194, 217, 220, 222, 227, 236, 240, 243, 246, 257, 259, 262, 263, 265, 267, 271, 274, 284, 291, 306, 311, 318, 323, 338, 341, 351, 355, 362, 373, 382, 404, 407, 420, 432, 433, 434, 455, 460, 466, 484, 489, 491, 492, 496, 499, 502, 505, 508, 512, 513, 516, 526, 547, 554, 556, 559, 570, 575, 578, 580, 581, 588, 594, 607, 608, 609, 612, 620, 626, 644, 646, 662, 690, 694, 695, 697, 716, 721, 725, 729, 733, 742, 746, 751, 754, 757, 759, 766, 769, 789, 792, 806, 814, 815, 818, 821, 827, 842, 847, 861, 863, 869, 871, 878, 881, 887, 894, 910, 928, 933, 951, 968, 970, 981, 990, 992, 998

The Allied side launched none on any seed.

**Why, from the staff's reasons and the log (seed 1 read in full):**

1. **The Japanese carriers steam at cruising speed.** The carrier doctrine routes a force to its hex at cruise (decision 0125, 7). Task force 3 makes 15 knots and task force 4 makes 14, where the script ordered 17 and 20.
   - On seed 1, task force 3 reaches (8,-11) in the afternoon.
   - Task force 4 is still under way at (7,-10) at the end of the day.
   - The script's forces are in their noon hexes by midday.
   - So the Japanese bombers are in reach of the American carriers late, or not at all. When they are, they strike on the same pulse as the script (below).
2. **The American side holds the Japanese only at long range.**
   - On seed 1 Henderson Field's search (the cover's) reported the striking force at (8,-14), about 360 nm from task forces 16 and 11 at (7,-5), in the morning.
   - The carriers' searches found nothing in the afternoon, and that contact was beyond every American bomber's reach.
   - The carrier doctrine does not close the range (decision 0125, "What it leaves out"), so the carriers stayed in (7,-5) with their bombers on the strike mission and nothing in reach.
3. **One task a force.** After the ruling of #375, each carrier force has one task, the commander's, at the hex its scripted route ends at. The deny intent forms none and says why. This was checked on the loading check: each force had exactly one operate task and no order was given twice.

**Against the script, reported and not judged:**

- The script strikes on 741 seeds (Allied) and 814 (Japanese).
- First strike in the same pulse as the script's:

| Arrangement | Side | Seeds where both struck | Same pulse |
| --- | --- | --- | --- |
| (a) | Japanese | 129 | 129 |
| (b) | Allied | 447 | 447 |
| (b) | Japanese (script) | 814 | 814 |
| (d) | Japanese | 129 | 129 |

- In every case the staff's first strike went in the afternoon, as the script's did.

### Against no orders, judged

| Side | Median margin from intents | Median margin from nothing | Passes |
| --- | --- | --- | --- |
| Allied: (b) against (c) | 1.05 | 0.20 | yes |
| Japanese: (d) against (e) | 0.00 | 0.00 | **no** |

Reported beside them (medians):

| Arrangement | Allied aircraft lost | Allied ships sunk | Allied out of action | Japanese aircraft lost | Japanese ships sunk | Japanese out of action |
| --- | --- | --- | --- | --- | --- | --- |
| (a) | 0 | 0 | 0 | 0 | 0 | 0 |
| (b) | 5 | 0 | 1 | 26 | 0 | 0 |
| (c) | 0 | 0 | 1 | 11 | 0 | 0 |
| (d) | 0 | 0 | 0 | 0 | 0 | 0 |
| (e) | 0 | 0 | 0 | 0 | 0 | 0 |
| (f), the script | 5 | 0 | 1 | 29 | 0 | 0 |

**Why the Japanese side fails it.**
- In (d) the scripted Allied carriers stay in (7,-5), and the Japanese carriers come down at cruise. On 848 seeds no strike is flown by either side, so the margin is 0.
- In (e) the Japanese carriers do not move at all, and the margin is 0.
- The median of 0 against 0 is not higher. This comes from the cruise-speed routing and the staff not closing the range, as above.

## Second: the week of the slice, days 15 to 21

### The conditions, arrangement (a)

| Condition | Allied | Japanese | Holds |
| --- | --- | --- | --- |
| Every invariant holds, on every seed | 1,000 of 1,000 runs complete | | yes |
| No staff order or command refused, on every seed | 0 refused; no commander's command of day 15 refused on any seed | | yes |
| A search and a patrol on at least 5 of the 7 days, on every seed | 1,000 | 683 | **no** |
| A strike or raid in the week, on 800 or more seeds | 997 | 1,000 | yes |
| Supply landed at its base in the week, on 800 or more seeds | 1,000 | 1,000 | yes |
| The same start and seed give the same game | identical | | yes |

- **Allied:** a search and a patrol on 7 days on 479 seeds, 6 on 361 and 5 on 160.
- **Japanese:**
  - A patrol on all 7 days on every seed.
  - A search on 6 days on 192 seeds, 5 on 491 and 4 on 317.
  - The 317 failing seeds miss their searches on days 17 (292 seeds), 21 (313), 19 (194), 20 (127) and 18 (25).

**Why.**
- The Japanese side has no searching group at Rabaul. Its searchers are the two floatplane groups of the carrier forces: Chikuma's (group 117, 5 aircraft) and Tone's (group 120, 5 aircraft).
- The carrier doctrine flies them at the type's range, 398 nm, over arcs of up to 180 degrees. Their fatigue then passes the 45 of the balanced risk, and they rest.
- On seed 3 the staff's reasons of day 17 read "air_group:117, a group that searches, has fatigue 75.68 above the 45 of the balanced risk: it rests", and the same for group 120. The carriers' own bombers are not set to search, because each force has a group that can only search (decision 0125, 4).
- The 317 seeds:

> 3, 12, 15, 16, 19, 22, 26, 28, 33, 38, 40, 43, 47, 48, 51, 52, 54, 55, 57, 61, 63, 65, 67, 70, 73, 77, 84, 88, 91, 92, 95, 97, 98, 99, 104, 105, 109, 110, 114, 116, 124, 129, 134, 144, 148, 150, 155, 162, 166, 167, 172, 176, 189, 193, 195, 200, 202, 204, 205, 209, 215, 219, 220, 231, 233, 235, 236, 239, 240, 241, 242, 243, 245, 250, 262, 265, 271, 272, 273, 278, 279, 281, 283, 286, 287, 289, 290, 294, 297, 300, 304, 308, 309, 310, 311, 313, 317, 323, 325, 329, 331, 333, 336, 340, 342, 347, 351, 353, 354, 362, 363, 364, 376, 377, 378, 380, 381, 382, 384, 385, 386, 388, 390, 397, 398, 400, 402, 406, 408, 414, 416, 418, 420, 428, 438, 446, 453, 459, 464, 467, 469, 475, 479, 486, 488, 489, 490, 491, 493, 494, 500, 501, 503, 504, 511, 512, 514, 525, 528, 536, 538, 540, 546, 549, 556, 560, 562, 564, 566, 568, 572, 575, 576, 579, 580, 584, 588, 592, 595, 597, 599, 600, 602, 605, 612, 613, 614, 620, 621, 622, 623, 625, 627, 628, 629, 638, 640, 642, 643, 650, 652, 656, 658, 660, 661, 665, 666, 677, 681, 683, 684, 685, 687, 688, 690, 691, 694, 695, 696, 700, 702, 703, 704, 706, 707, 708, 713, 715, 716, 718, 720, 727, 728, 734, 736, 737, 744, 746, 752, 753, 758, 765, 767, 768, 771, 774, 779, 782, 784, 785, 788, 796, 803, 807, 808, 809, 810, 819, 820, 823, 826, 829, 830, 844, 848, 849, 851, 854, 855, 858, 863, 865, 867, 873, 874, 875, 878, 881, 885, 886, 887, 888, 891, 893, 896, 912, 913, 919, 921, 922, 924, 926, 927, 931, 934, 938, 943, 945, 951, 953, 956, 958, 962, 967, 968, 969, 974, 978, 980, 982, 989, 992, 993, 994, 995, 997, 998

### Against no orders, judged

The medians over the week, days 15 to 21.

**Allied: (b), intents against the Japanese script, beside (c), given nothing. It passes.**

| Measure | From intents | From nothing | No worse | Better |
| --- | --- | --- | --- | --- |
| Henderson Field open to its holder (days the Allied side held it open, less days the Japanese did) | 7 | 7 | yes | no |
| Supply landed at Henderson Field | 3,220.8 t | 2,000.0 t | yes | yes |
| Aircraft lost | 24 | 1 | **no** | no |
| Ships sunk or out of action | 3 | 3 | yes | no |
| Objectives margin (reported) | 56.05 | 47.59 | | |

Three are no worse and one is better, so it passes.

**Japanese: (d), intents against the Allied script, beside (e), given nothing. It passes.**

| Measure | From intents | From nothing | No worse | Better |
| --- | --- | --- | --- | --- |
| Henderson Field open to its holder (days the Japanese held it open, less days the Allied side did) | -7 | -7 | yes | no |
| Supply landed at Taivu and west of the Lunga | 119.08 t | 0 t | yes | yes |
| Aircraft lost | 29 | 0 | **no** | no |
| Ships sunk or out of action | 0 | 0 | yes | no |
| Objectives margin (reported) | -52.40 | -51.36 | | |

Three are no worse and one is better, so it passes.

**Against the script, reported and not judged:** medians, days 15 to 21.

| Arrangement | Allied margin | Allied aircraft lost | Allied sunk or out of action | Supply at Henderson | Japanese aircraft lost | Japanese sunk or out of action | Japanese supply landed | Airfield open (Allied) |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| (a) both from intents | 53.48 | 15 | 0 | 3,220.8 t | 24 | 0 | 119.08 t | 7 |
| (f) the script | 61.03 | 23 | 3 | 2,960.0 t | 61 | 3 | 0 t | 7 |

**How the week's supply was carried.** Under the ruling on reading 7, the Japanese runs carry supply from Rabaul in task force 31, and land it at Taivu. West of the Lunga the staff hands in that it cannot meet the intent on most mornings: the pool's six ships are committed to Taivu's runs.

**Limits named beside the result (decision 0128, "What the staff cannot do", and the rulings):**
- **No carrier station:** the staff does not choose where a carrier force stands.
- **No home moved:** it does not move a surface force's home. No surface force is within a night's steaming, so no stand task is formed in the week.
- **No carrier raid:** a carrier does not raid a base.
- **Wave groups:** groups split by wave are played as the groups they are.
- **Men left aboard (ruling on reading 6):** they stay aboard all week. That is the Seabees of ship 99 and the Kawaguchi and other units in task forces 32 to 39.

## Legibility

The set, the rule, the key, the answers and the marks are in `crates/sim-cli/tests/recorded_games/legibility_second_proof/`, beside stage 1's. The set, rule and key were written in decision 0128 before any reader ran.
- **The readers:** one Haiku agent per reason, given the reason's file alone and stage 1's two questions.
- **The marking:** by a Sonnet agent against the key.

| Doctrine | From | Pass both |
| --- | --- | --- |
| Cover | the week (a), seed 1 | 6 of 6 |
| Raid | the week (a), seed 1 | 6 of 6 |
| Carrier | the week (a), seed 1 | 4 of 4 |
| Strike | scenario 53, placeholder | 4 of 4 |
| Stand | scenario 54, placeholder | 4 of 4 |
| All | | 24 of 24 |

**It passes:** at least 20 of 24, and no doctrine's reasons all fail.
- **Where the strike and stand reasons came from:** placeholder scenarios. The week forms no stand task and no strike task from a base (the architect's ruling).
- **Factual slips:** the marker noted a few in the answers, such as a wrong day or "were set" for a day with no order. None changed a mark.
- **A finding for the designer:** reason 4, the cover of Lunga on day 16, is 52 KB long. Almost all of it is the list of every hex of the waters to deny, named as threats in its search arc's binding.

## A model's game

Kept as `crates/sim-cli/tests/recorded_games/second_proof_week_sonnet/` (game, transcript, save and the model's `account.md`).

**How it was set up.**
- **The Allied seat's days 1 to 14** were pre-played with the script's own Allied orders through `issue_orders` and `end_day`, on seed 1 of `07m_model_week.toml`. All 24 orders were queued, and the day hashes 1 to 14 equal the scripted run's.
- **From day 15** a Sonnet model played by intents alone. It was given the tools' descriptions and one instruction, and gave no order to a unit.

**What it stated:**
- hold Lunga and hold Tulagi;
- keep Lunga supplied at 30 days, a run every 10, with task forces 9, 11 and 3 from Espiritu Santo;
- deny the waters within 4 hexes of (1,-7);
- operate task force 1 from (4,-3) and task force 2 from (5,-3);
- the leave to launch, and an interrupt below 12 days at Lunga.

**How it went.**
- One `set_limit` named the wrong intent and was refused. It set the limit again on the right one.
- It resolved days 18 to 21 in one `run`.
- **Result:** an Allied decisive victory, 64.4 points to 7.8. That came from 4,712 t landed at Lunga, 1,866 enemy men lost, both bases held, two destroyers sunk and 27 aircraft destroyed.
- No Japanese carrier ever appeared to it as a contact.
- Not judged.

## Speed

`sim-cli time --machine "Apple M3 Pro, 36 GB, macOS (the architect's laptop)" --slice data/scenarios/second-proof/07a_week.toml --early-day 15 --late-day 18`, release build, both staffs at work from day 15. The workloads are decision 0100's.

| Workload | Median |
| --- | --- |
| One day of the slice | 13.2 ms (day 16), 8.79 ms (day 19) |
| The staff's decisions for one side, longest day | Allied 4.08 ms (day 15); Japanese 7.42 ms (day 17) |
| Six days in one call from day 15 | 68.1 ms (about 11.4 ms a day) |
| Three days in one call from day 18 | 27.7 ms (about 9.2 ms a day) |
| A day with every question asked at every moment | 27.6 ms early, 23.8 ms late |

**The staffs' share of a day.** On the heaviest days, the morning the intents are stated and the day of the carrier battle, the staffs take up to about half of a day's time (4 to 7 ms of 9 to 13 ms). On a quiet day they take well under a millisecond.

## What it puts to the designer

Each failure raises a choice of one of three: a doctrine to change, a limit of the stage to lift, or a miss to name.

1. **A carrier force from an end does not reach its strike in time.**
   - The routing at cruise (decision 0125, 7) brings the Japanese carriers in late.
   - The American carriers do not close the range on a contact beyond reach (decision 0125, "What it leaves out").
   - Neither side reaches 800 seeds with a strike.
2. **Two floatplane groups cannot search every day at the type's range.**
   - The searchers rest above the fatigue level, and no other group of the force searches while one that can only search exists.
   - The Japanese side has no searcher at Rabaul, so it misses days.
