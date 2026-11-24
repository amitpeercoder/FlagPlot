# Stage 4's proof: the conditions as read, the files, and what is to be made, before the run

Written by the architect on Nov 24, 2026, for step 9 of decision 0161, the proof of the trial. It fixes how each of the thirteen conditions is read, on which files and seeds, and with what number, before anything of the proof is played. It names the two misses that are expected and their cause. It says what a developer makes for the run and what the report must say. It holds no result and no verdict.

It is read on main once decisions 0230 (supply at press) and 0231 (men ashore beside a place are fed) are merged, and the proof is run from that commit, which the report names with its labels. Nothing a condition reads changes between this page and the report. Decision 0165's ships of the record come after (the designer, Nov 24, 2026).

The rules of a proof hold: nothing is tuned to pass it; nothing is changed after a judged run without saying so; a case fixed here is not replaced after its figures are seen; and the proof is run once.

## What has been seen already, and said so

Three things were read on the proof's own arrangement before this page, each to find a cause and none to move a mark. The report repeats them.

- **Seeds 1 to 100 of arrangement (a)** were played and read for the men lost to hunger, before and after decision 0231 (decision 0161, "The men unfed, as measured"; decision 0231, "As measured after the build").
- **Seeds 29, 40 and 54** were read day by day for what binds the Japanese supply (decision 0161, "The reading of three seeds").
- **The times** of a trial, a trial beside another and a window were measured on the 39 days as each was built (decisions 0175, 0177, 0178 and 0181) and accepted by the designer.

The cases of conditions 7, 8 and 9 below have not been played by anyone. One game of case W1 was played by the architect to check its hand calculation, and is so marked.

## The two misses expected

On 100 of the 1,000 seeds, after decision 0231: condition 3 stands at about 0.07 on the Japanese side against a mark of 0.05, and condition 13 at about 0.12 against 0.05. Both are expected to miss. The Allied side is expected to meet both.

Their cause, named before the run: the scenario gives the Japanese side six transports before day 23, in two forces usable from days 10 and 12; both are first used for the landings at Lunga; ships are lost off Lunga; and the garrison's 71 t at (2,-5) is gone on day 13, before a ship can be back. Three costs of the supply staff's own working add days: two mornings between a ship in port and its orders, a finished run's ships held two to three days, and a force that goes whole to one base (`docs/open-work.md`, 1c). The designer was asked and answered on Nov 24, 2026: the proof is run with the two misses expected and none of the three mended first.

If either condition is met on the Japanese side after all, the report says so and says that it was expected to miss. If either misses on the Allied side, that is a finding of the proof and not one of these.

## The files

| Name | File | What it is |
| --- | --- | --- |
| (a) | `data/scenarios/stage-3-proof/07a_slice.toml` | The 39 days, both sides from "control an island" for Guadalcanal and Tulagi, both leaves, no risk set (balanced). As on main at the proof's commit |
| (b), (c) | `07b_slice.toml`, `07c_slice.toml` | One side given nothing. Read for condition 12's "beside what the games did" only |
| S1, S2 | to be made, `data/scenarios/stage-4-proof/` | Two built cases with dice where the picture is right (condition 7) |
| W1, W2, W3 | to be made, the same folder | Three built cases of a wrong picture (condition 8) |
| R | to be made, the same folder | The case of preparation (condition 9; decision 0161, "Condition 9, the case") |

`python3 scripts/stage-3-proof/make_arrangements.py --check` must report (a), (b) and (c) "as made" at the proof's commit. The folder `stage-4-proof` is read by no loop (decision 0072), and no file in it is a scenario for play.

**A trial on (a).** A trial needs the trial's figures and a file of sides. (a) names the file of sides and holds the placeholder beliefs of decision 0173. The developer reports whether (a) names the trial's figures (`trial`); if it does not, the harness hands `data/reference/trial.toml` to `sim-trial` itself, as `sim-cli time` does, and (a) is not changed. No condition is read on a copy of (a) that differs from (a) by anything a rule reads.

**The figures.** (a), (b) and (c) name the live ground figures, `ground.toml`, in which an unfed man is at risk from the first night. The table's figures, in which hunger takes time, are not the proof's. Conditions 3 and 13 are read on (a) as it is.

## The sizes

Small, 8 worlds for 7 days; standard, 16 for 14; full, 64 for 21 (decision 0175, accepted by the designer).

## The conditions

Each is given as decision 0161 has it, then how it is read. "Judged" has a mark that can fail. A count is of seeds or games; nothing is a share unless it says so.

### 1. The hand calculations pass. Judged

At the proof's commit, in release, `cargo test --workspace` passes, and with it by name the tests of scenarios 85, 86, 90 to 97, 100, 118, 135 and 136. None of them has a test marked "statement written, not built" or set aside. The report lists any test of those files that is ignored, with its reason.

### 2. Nothing changed where it is unused. Judged

At the proof's commit `sh scripts/regenerate.sh` changes no file, and the baselines' tests pass. That every game with no trial asked plays as it did is what each step's baselines held when it merged; it is not played again here. The report names, from the records' "As built", every file decisions 0162, 0163, 0169, 0230 and 0231 moved.

### 3. The mends hold where 0159 missed. Judged. Expected to miss on the Japanese side

On (a), seeds 1 to 1,000, 39 days, by `scripts/0159-proof/hunger.py` unchanged: for each side, over the seeds on which it landed men at a place not its own, the median of the men lost unfed over the men landed. The mark: no more than 0.05, each side by itself.

### 4. No peek. Judged

- **The built cases.** The tests of that kind in scenarios 93 to 97, 100 and 135 pass (condition 1).
- **The 39 days at three days.** Two pairs, each of (a) and a copy of it that differs only in things the asking side cannot see on the evening of day 3. For the Allied seat: Rabaul's supply raised by 1,000 t, and the men of one Japanese land unit that is aboard no ship and in contact with no one changed by 100. For the Japanese seat: Espiritu Santo's supply raised by 1,000 t, and the fuel of one Allied task force that no Japanese contact holds lowered by 100 t. Seed 1. The harness first shows that the two games differ (their state hashes on the evening of day 3 are not equal) and that the asking side's own half and its beliefs are equal, field for field. It then asks the same trial in each, of the side's own first intent as stated, at the small size, and compares the two answers byte for byte. The mark: equal, both pairs. If a pair's views are not equal the pair was badly made: it is reported as void, made again with another hidden thing, and that is said.

### 5. The same twice, and nothing changed. Judged

On (a), seed 1: the trial of the Allied side's first intent at the small size, asked on the evenings before day 1, of day 3 and of day 20, each asked twice: each pair of answers is equal byte for byte. And the game played to day 39 with that trial asked on every morning has, on every day, the daily hash of the game never asked. The same for the Japanese seat.

### 6. Its sampling is right. Judged

Scenario 95's tests pass (condition 1): four worlds, two with the believed men and two without, each worked by hand.

### 7. Its spreads hold. Judged

**The cases.** Both are scenario 68's map, bases, ships and course, played for 21 days, with scenario 76's ground figures, in which a shot hits by chance. In both the picture is right: the side's opening estimate is the garrison's true count, nothing is believed beyond it, the enemy's base holds the 200 t the trial's figures assume, and the enemy has no order.

- **S1:** unit 1 alone, 600 men, against a garrison of 200, estimated at 200.
- **S2:** units 1 and 2, 600 men each, against a garrison of 300, estimated at 300.

**The run.** For each case: one trial of the course at the full size, 64 worlds for 21 days, asked on the evening before day 1; and 200 games of the scenario with the course handed in on day 1, seeds 1 to 200, each played 21 days. From each game are read, as the estimate reads a world (decision 0176, "As built", 5 and 6): the day the end was met, or that it was not; whether it was lost again; and the five costs of the side tried (men lost to the enemy, men lost unfed, ships sunk, ships out of action, aircraft).

**The arithmetic of the mark.** If a trial's worlds and the games are draws of one thing, a game's cost falls between the lowest and the highest of n worlds with a chance of (n - 1) / (n + 1) where the cost can take many values, and more where it takes few. For 64 worlds that is 63 / 65 = 0.969230. For 16 it is 15 / 17 = 0.882352, and for 8 it is 7 / 9 = 0.777777: at the standard and small sizes a right trial would miss "nine in ten". So the condition is judged at the full size, and the smaller sizes are reported beside it with these figures and judged by nothing.

**The marks,** for each case by itself, at the full size:

- **Each cost:** in at least 180 of the 200 games the cost is no less than the estimate's lowest and no more than its highest.
- **How often it came off:** with K the worlds of 64 in which the end was met and kept, and M the games of 200 in which it was, K / 64 and M / 200 differ by no more than 0.19. The band is 2.6 times the largest the difference's spread can be, 0.5 x the root of (1/64 + 1/200) = 0.071807, so a right trial falls outside it about once in a hundred.
- **Too wide to be of use, reported and not judged:** for each cost whose middle value is above 0, whether the middle half of the worlds' values (from the worlds' own figures, the 17th to the 48th of 64 in order) is wider than the middle value. The count of such costs is reported.

If in a case the end is met in every world and every game, or in none, the report says that the case showed nothing of the count. The case is not replaced.

### 8. A wrong picture moves it as expected. Judged

Three cases. In each the mark has two parts: the estimate is wrong in the direction stated, and its own list of what was assumed names the thing that was wrong.

- **W1, the garrison believed half the truth.** Scenario 95's inputs with the garrison truly 400, the opening estimate 200 and nothing believed beyond it; scenario 68's figures, with no die. *By hand:* in every world the garrison is 200, the place is taken on day 4 and the side loses 100 men (scenario 95's worlds 1 and 3). In the game, on day 4 the 600 fire 6 hits and the 400 fire 4: the garrison loses 300 and has 100, the side loses 200 and has 400, and 100 is not fewer than 0.2 x 400 = 80, so the garrison is not beaten. On day 5 the side's fit men are 400 x 50 / 100 = 200, at least the 100 now reported: it attacks, fires 2 hits and the garrison's last 100 are gone; the garrison fires 1 hit and the side loses 50. The place is taken on day 5 with 250 men lost. (The architect played this one game on Nov 24, 2026, to check the working; it gave these figures.) *The mark:* the estimate says day 4 and 100 men in every world; the game is a day later and 150 men dearer; the assumptions name the men at the place as 200 from the estimate of day 0.
- **W2, a carrier force believed absent that is there.** A built case from scenario 93's inputs: a convoy of the side's bound for a base of its own, inside the reach of an enemy carrier force that the scenario's `[[believed_force]]` gives the standing believed absent, and that is in truth there with an order to strike ships. *By reasoning:* no world holds an enemy ship or aircraft, so in every world the side's ships sunk, ships out of action and aircraft lost are 0. *The mark:* the estimate's highest for each of those three costs is 0; in at least 20 of 200 games, seeds 1 to 200, the side has a ship sunk or out of action; the assumptions name the carrier force as believed absent and not made.
- **W3, an enemy intent not as stated.** W1's inputs with the garrison truly 200, as estimated, and a second enemy unit of 600 men with a transport force at a base of his own, which the scenario's `[[believed_force]]` gives as believed present there and for which no `[[believed_intent]]` is given; in truth the enemy states "take the place" back, with both leaves. Scenario 68's figures, with no die. *By reasoning:* in every world the enemy holds what he holds, his second unit never sails, the place is taken on day 4 and kept, and the side loses 100 men. In the game his unit lands and a fight follows. *The mark:* the estimate says met and kept in every world with 100 men lost at the highest; in the game the side loses more than 100 men, or the place is lost again; the assumptions say the enemy was given no intent. The developer works the game's days by hand in the case's test before it is played, and the architect reads that working before the run.

### 9. Preparation matters. Judged

Case R, as decision 0161 has it ("Condition 9, the case"): the same order at press with "not before day 9", handed in on day 1 in one course and on day 4 in the other; two trials of one number at the standard size, 16 worlds for 14 days; placeholder figures `preparation_per_day` 10 and `unprepared_landing_fatigue` 40; one unit of 1,000 men. By hand the earlier lands with rehearsal 80 and fatigue 8, 920 fit men, and the later with rehearsal 50 and fatigue 20, 800 fit men. The architect writes the case's full statement in its test's doc comment before it is built, checking the day counts against scenario 27. *The mark:* on the evening of the day the men are ashore, the unit's fit men by the side's own view are more in the course begun on day 1 in at least 15 of the 16 worlds; a world in which either course did not land that day counts as not more.

### 10. Within its budget. Judged

`sim-cli time`, release, on the named machine (Apple M3 Pro, 11 cores, 36 GB) with no other job of the proof running, each size in its own process, the middle of three runs: a trial, a trial beside another, and a window of the dates it chooses, of the Allied side's first intent on (a), asked on the evening before day 1 and on the evening of day 20. The marks are the highest time measured for each in the records whose times the designer accepted (decisions 0175, 0177, 0178 and 0181: 1.2 s, 3.9 s and 22.7 s; 2.0 s, 6.4 s and 36.1 s; 3.38 s, 12.8 s and 67 s; 804 MB), with a tenth over for the machine, rounded up:

| | Small | Standard | Full |
| --- | --- | --- | --- |
| A trial | 1.4 s | 4.3 s | 25 s |
| A trial beside another | 2.2 s | 7.1 s | 40 s |
| A window | 3.8 s | 14.1 s | 74 s |

and no process above 900 MB at its peak. The window at small is the three given dates of decision 0177's measure where it chooses one. A time above its mark at either evening misses the condition for that row; the report gives every time.

### 11. Legible. Judged

Six estimates, as a seat is answered, each given alone to a reader that has seen nothing else: the Allied side's and the Japanese side's first intents on (a) from the evening before day 1 at the standard size; S1's and W1's; one window (scenario 96's) and one trial beside another (scenario 100's). Five questions of each: what the operation needs, when it could go, in how many of the worlds tried it came off, what it may cost, and what was assumed. The key is written from the estimate's own fields by the harness before any reader runs. Readers and marker as in decision 0159's proof: a Haiku reader for each estimate, a Sonnet marker held to the key, the architect reading the marks. An estimate passes when all five answers agree with the key. *The mark:* six of six. The models' names and versions, the prompt and the answers are kept under `crates/sim-cli/tests/recorded_games/legibility_stage_4/`.

### 12. Reported, not judged

On (a), from the evening before day 1, for each side: the estimate of its first intent at the full size, beside what the 1,000 games did by the end of day 21: in how many the end was met and kept, the day, and the five costs as lowest, middle and highest. Beside them, from (b) and (c), the same for the side with orders against a side with none. The report says what the estimate's picture lacked that the games had: no land air group at an enemy base (decision 0174), and the placeholder beliefs of decision 0173.

### 13. Men who begin ashore are fed. Judged. Expected to miss on the Japanese side

On (a), seeds 1 to 1,000, by `scripts/unfed/` as it stands on main: for each side, the men of land units ashore at the start on the land of an island that side states "control an island" of on day 1; of those, the men lost unfed in a hex where a base of their side stood just before the loss, over the units' men at the start; the median over the seeds. The mark: no more than 0.05, each side by itself. The men of the same units lost unfed where no base of their side stood are reported beside it under their own name and judged by nothing (the designer, Nov 24, 2026).

## What moved since decision 0159's proof

That proof was run on Nov 19, 2026 at state layout v47, doctrine `supply-39`, rules label 19 and interface 26. This one is run at the labels main has once decisions 0230 and 0231 are in: state layout v60, rules label 34, doctrine `supply-62`, interface 55 or the next. The developer makes the table for the report: every change to (a), (b) and (c) and to the staff's setup since that day, by commit and decision; and every decision that raised the doctrine's label from 39, with a line on what it changed in a staff's orders. Among them, as the index gives them: decisions 0162, 0163 and 0169 (the mends of decision 0159's miss), 0191 (a lift turned back), 0193 (men ashore with no more coming), 0195 and 0196 (forces that arrive, and stock from off the map), 0214 (a carrier force that pursues), 0217, 0220, 0222 and 0228 (limits and the source of supply), 0230 and 0231. Condition 3's figure is set beside decision 0159's 0.2537 with that table, since the difference is the work of all of them and not of the two mends alone.

## What is to be made

By a developer, each read by the architect before the run. Nothing here is played before the judged run but to show that it runs, on a seed that is not among the judged ones (seed 2,001).

1. **The six case files** S1, S2, W1, W2, W3 and R, with a test file each that holds the case's statement as this page has it. R's and W3's statements are written by the architect first.
2. **`scripts/stage-4-proof/run.sh`,** in the manner of decision 0159's: build, the 1,000 games of (a) twice, the exports, the readers of conditions 3 and 13, and the steps below; it writes only outside the repository.
3. **A harness for conditions 4, 5, 7, 8, 9 and 12,** as ignored tests of `sim-cli` that are run by hand (as `tests/proof_0159_orders.rs` is): each writes what it read to a file and asserts nothing, so that the marks are read by the architect.
4. **The legibility files** of condition 11.
5. **The table of what moved.**

The same start is checked to give the same game, as in the earlier proofs: (a)'s batch is made twice and compared.

## What the report must say

`docs/validation/stage-4-proof.md`, in the manner of the earlier reports: the commit and its labels, a table of the thirteen conditions with what was read, the mark and whether it was met, and then each in turn. Whatever the conditions give, it also says:

- **The two misses expected,** their figures on 1,000 seeds, the cause as named above, and that the three costs of the supply staff's working were left for after.
- **What was read beforehand** on the proof's arrangement, as listed above.
- **A window is "decide later".** A later date of a window is the order given later; nothing is prepared in the days between. The spec's "what if we wait" asks for those days to be played with preparation, and the stage does not do that. Condition 9 shows that preparation matters by two courses, at press and with a day named; at balanced an earlier start buys a date and not fitness (decision 0177, the reading of Nov 24, 2026).
- **What a picture lacks:** an enemy's land air groups; the kind of a contact taken as reported; the beliefs of the 39 days, which are placeholders; and that an air group or a supply run that comes due makes no date of a window.
- **What the stage leaves out by its plan:** looking back at a live game; a staff that chooses by trial; and step 10, the enemy's approaches.
- **What is owed from the stage** and not judged here: a plan displaced by a take of another place (decision 0176); a staff that reckons a force forward from its own orders (decision 0136); the tests across a picture of four rules that read earlier days (decision 0170).
- **After the proof:** the Japanese runs as the destroyers that made them (decision 0165, steps 5 and 6), with conditions 3, 12 and 13 read again on that file and reported beside these, and no second judgement.

A failure is put down to its cause: the constructor, the trial's play, the estimate's reading, the doctrine the trial plays, or the case as it was made.
