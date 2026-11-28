# Stage 4's proof: the conditions as read, the files, and what is to be made, before the run

Written by the architect on Nov 24, 2026, for step 9 of decision 0161, the proof of the trial. It fixes how each of the thirteen conditions is read, on which files and seeds, and with what number, before anything of the proof is played. It names the two misses that are expected and their cause. It says what a developer makes for the run and what the report must say. It holds no result and no verdict.

It is read on main once decisions 0230 (supply at press) and 0231 (men ashore beside a place are fed) are merged, and the proof is run from that commit, which the report names with its labels. Nothing a condition reads changes between this page and the report. Decision 0165's ships of the record come after (the designer, Nov 24, 2026).

Amended on Nov 27, 2026, before the run: step 1 of decision 0232 merged first, and the labels the proof is run at are those of main with it in ("Amended before the run, Nov 27, 2026", at the foot of this page).

The rules of a proof hold: nothing is tuned to pass it; nothing is changed after a judged run without saying so; a case fixed here is not replaced after its figures are seen; and the proof is run once.

## What has been seen already, and said so

Three things were read on the proof's own arrangement before this page, each to find a cause and none to move a mark. The report repeats them.

- **Seeds 1 to 100 of arrangement (a)** were played and read for the men lost to hunger, before and after decision 0231 (decision 0161, "The men unfed, as measured"; decision 0231, "As measured after the build").
- **Seeds 29, 40 and 54** were read day by day for what binds the Japanese supply (decision 0161, "The reading of three seeds").
- **The times** of a trial, a trial beside another and a window were measured on the 39 days as each was built (decisions 0175, 0177, 0178 and 0181) and accepted by the designer.

The cases of conditions 7, 8 and 9 below have not been played by anyone. One game of case W1 was played by the architect to check its hand calculation, and is so marked.

## The two misses expected

On 100 of the 1,000 seeds, after decision 0231: condition 3 stands at about 0.07 on the Japanese side against a mark of 0.05, and condition 13 at about 0.12 against 0.05. Both are expected to miss. The Allied side is expected to meet condition 3; condition 13 does not arise for it (amended on review, 5).

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

Case R, as decision 0161 has it ("Condition 9, the case"): the same order at press with "not before day 9", handed in on day 1 in one course and on day 4 in the other; two trials of one number at the standard size, 16 worlds for 14 days; placeholder figures `preparation_per_day` 10 and `unprepared_landing_fatigue` 40; one unit of 1,000 men. By hand the earlier lands on day 11 with rehearsal 100 and no fatigue, 1,000 fit men, and the later with rehearsal 70 and a fatigue of 7 at the day's end, 930 fit men (corrected on review, 10). The architect writes the case's full statement in its test's doc comment before it is built, checking the day counts against scenario 27. *The mark:* on the evening of the day the men are ashore, the unit's fit men by the side's own view are more in the course begun on day 1 in at least 15 of the 16 worlds; a world in which either course did not land that day counts as not more.

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

That proof was run on Nov 19, 2026 at state layout v47, doctrine `supply-39`, rules label 19 and interface 26. This one is run at the labels main has once decisions 0230 and 0231 are in: state layout v60, rules label 34, doctrine `supply-62`, interface 55 or the next. (Amended on Nov 27, 2026, before the run: state layout v60, rules label 35, doctrine `supply-63`, agent interface 56, saved game 29; "Amended before the run, Nov 27, 2026" below has the evidence. Amended a second time the same day: agent interface 57, and the proof is run from b9e488a; "Amended before the run a second time" below.) The developer makes the table for the report: every change to (a), (b) and (c) and to the staff's setup since that day, by commit and decision; and every decision that raised the doctrine's label from 39, with a line on what it changed in a staff's orders. Among them, as the index gives them: decisions 0162, 0163 and 0169 (the mends of decision 0159's miss), 0191 (a lift turned back), 0193 (men ashore with no more coming), 0195 and 0196 (forces that arrive, and stock from off the map), 0214 (a carrier force that pursues), 0217, 0220, 0222 and 0228 (limits and the source of supply), 0230 and 0231, and, as amended on Nov 27, 2026, 0232's step 1 (a take the commander wrote, which no file of the proof holds). Condition 3's figure is set beside decision 0159's 0.2537 with that table, since the difference is the work of all of them and not of the two mends alone.

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

## What the developer builds, item by item

Added on Nov 24, 2026, while this page was under review; it changes no reading above. Two statements are now written and their files made by the architect; the rest is the developer's.

**Already written, not to be changed by the build:**

- **Case R:** `data/scenarios/stage-4-proof/case_r_preparation.toml`, its ground figures `data/reference/ground_placeholder_stage_4_proof_r.toml` with the frozen copy, and `tests/proof_stage_4_case_r_preparation.rs`, four empty tests. Its figures are those of the test's doc comment (rehearsal 100 against 70; 1,000 fit men against 930, on day 11), which correct the ones in condition 9 above (decision 0161, "Condition 9's case, corrected"); that line is corrected with the review.
- **Case W3:** `data/scenarios/stage-4-proof/case_w3_an_intent_not_as_stated.toml` and `tests/proof_stage_4_case_w3_an_intent_not_as_stated.rs`, four empty tests. Its trial is at the standard size and its game is read on seeds 1 to 20: the side's men lost by the end of day 8 are at least 300 against the estimate's 100. The line in condition 8 above that leaves the game's working to the developer is met by that file.

**To build:**

1. **Fill the eight tests of R and W3** to their statements and take the marks off. A mismatch goes to the architect.
2. **Case W1:** a scenario file in the same folder (scenario 95's inputs with the garrison at 400, the estimate 200 and no believed men) and a test file whose doc comment is the statement of condition 8, W1, word for word, with tests of the worlds (day 4, 100 men), of the game (day 5, 250 men) and of the assumption named.
3. **Cases S1 and S2:** two scenario files (scenario 68's map, bases, ships and course; 21 days; scenario 76's ground figures; the garrison and the estimate as condition 7 gives them; the enemy's base at the 200 t the trial's figures assume; no enemy order) and one test file that asserts only that each loads and that the opening estimate equals the garrison. They are not played but on seed 2,001, to show that they run.
4. **Case W2:** a scenario file made from scenario 93's inputs as condition 8 describes, and a test file with the statement of W2. Before it is built the developer reports to the architect which force, convoy, base and order it proposes, and the architect writes the statement's particulars.
5. **`scripts/stage-4-proof/run.sh`** and its `README.md`, with the steps: build; the batch of (a) twice; the exports of (a); the readers of conditions 3 (`scripts/0159-proof/hunger.py`) and 13 (`scripts/unfed/`); `sim-cli time` for condition 10; and each harness below. It writes only into a folder outside the repository.
6. **The harness,** ignored tests run by hand, each writing what it read to a file and asserting nothing: no peek on the 39 days (condition 4, with the four hidden changes it names, and the check that the views are equal); the same twice and the hashes (condition 5); the spreads of S1 and S2 at the three sizes with the 200 games each (condition 7); the three wrong pictures (condition 8); the two courses of R (condition 9); and the estimates of day 1 beside the 1,000 games (condition 12).
7. **The legibility files** of condition 11: the six estimates as a seat is answered, the key made from their fields, the prompt, and the folder `crates/sim-cli/tests/recorded_games/legibility_stage_4/`. No reader is run before the judged run.
8. **The table of what moved** since Nov 19, 2026, as "What moved" above asks.
9. **A report to the architect, before anything is run,** of two facts: whether (a) names the trial's figures, and if not how the harness hands them in; and that `make_arrangements.py --check` reports the three arrangements as made at the proof's commit.

## Amended on review, Nov 24, 2026

The page as first written (commit f3f861ae) was read by a reviewer who had not seen it written. The reviewer did the arithmetic of condition 7 again and had the same figures, and found no mark moved to fit the two misses. Eight things were to be closed before the run, so that nothing is chosen after a result is seen. What follows is the architect's and replaces the text above where they differ. Nothing of the proof has been played.

1. **Condition 7: what a miss means, and which costs are tested.** The figure 63 / 65 is the coverage a right trial has on average. One trial's own coverage varies: with 64 worlds it is spread as a Beta(63, 2), and by the reviewer's working a right trial has fewer than 180 of 200 games inside for a cost of many values about 15 times in 1,000. A cost that takes few values does better. So the mark of 180 stands, a miss of it is a miss, and the report gives beside each cost the games inside of 200 and says that a right trial misses that mark about once in 70 for a cost, and falls outside the band of 0.19 about once in 100 for a case. No second trial and no second set of games is made after a miss. **A cost is tested** only where it is not the same number in every world and every game. In S1 and S2 no ship or aircraft can be lost, so ships sunk, ships out of action and aircraft are 0 throughout: they are reported as not tested and counted neither as met nor as missed. The costs tested are the men lost to the enemy and, where any is, the men lost unfed. The report says how many costs were tested in each case; if a case has none, it showed nothing and says so.
2. **The trial's figures.** S1, S2, W1, W2, W3 and R name `data/reference/trial_placeholder_scenario_95.toml` (an enemy base is assumed to hold 200 t of supply and no fuel; seed 1942) and the file of sides `data/reference/class_sides_placeholder_scenario_95.csv`, each through its frozen copy. (a) names no trial's figures, and every trial on (a), in conditions 4, 5, 10, 11 and 12, is given `data/reference/trial.toml`, as `sim-cli time` gives it (`crates/sim-cli/src/timing.rs`). The developer is no longer asked to report whether (a) names them.
3. **W1's mark is the direction, and its figures are the working.** The condition is met where the game takes the place later than the estimate says, or not at all, and loses more men than the estimate's highest, and the list of what was assumed names the men at the place as 200 from the estimate of day 0. A game that took the place on day 6 with 300 men lost would meet the condition. It would also fail the case's own test, which asserts day 5 and 250 men, and that failure would be reported as a fault in the working or in the rules and go to the architect; it would not change the condition's reading. The same holds for W3: the mark is that the game costs the side more men than the estimate's highest; "at least 300 by the end of day 8" is the working. **The files, seeds and games:** W1 is `data/scenarios/stage-4-proof/case_w1_the_garrison_believed_half.toml`, to be made; its trial is at the standard size and its game is one, seed 1, since no die is rolled in it. W3's trial is at the standard size and its game is read on seeds 1 to 20. W2's is as written: the full size, and seeds 1 to 200.
4. **Condition 7's M** is the games in which the end was met and kept to the end of day 21, the same thing K counts in the worlds.
5. **Condition 13 where a side has no man ashore at the start.** In (a) every Allied land unit begins aboard a ship. The condition then has nothing to count for the Allied side: it is reported as "no man of the side begins ashore" and is neither met nor missed. The line above that expects the Allied side to meet both conditions is corrected: it is expected to meet condition 3, and condition 13 does not arise for it. **Which units are counted:** the reading is the units ashore at the start on the land of an island their side states "control an island" of on day 1 (decision 0231, rule 1). `scripts/unfed/unfed.py` counts every unit of the side ashore at the start. The Japanese side of (a) states both islands. Before the run the developer lists the Japanese units ashore at the start with their hexes and says whether each is on the land of island 1 or island 2; if every one is, the script's count is the reading's and the report says so; if one is not, the reader is given the island's land to count by, before the run, and the change is named. Either way the mark is untouched.
6. **Seeds 1 to 100.** They were played and read before decision 0231 was written, and that rule was written from what they showed. They stay in the judged 1,000, because condition 3 is decision 0159's reading unchanged and is set beside its 0.2537 on the same seeds. But a rule is flattered by the games it was written from, so conditions 3 and 13 are also given on seeds 101 to 1,000 alone, which nobody has read, beside the 1,000. The verdict is the 1,000's; if the 900 would give the other verdict, the report says so in the table and not in a note.
7. **Condition 2 is read more weakly than decision 0161 wrote it, and says so.** "Every game plays as before step 3" cannot be played again: the doctrine has moved through more than twenty labels since for other reasons, and no game of today is the game of Nov 19. What can be shown is shown: that the trial's means touch nothing when unused, by each step's baselines when it merged and by the baselines at the proof's commit; and that asking a trial changes nothing, by condition 5. The report gives condition 2 in those words and does not call it "every game as before".
8. **The tests of condition 1, by name:** `scenario_85_a_nights_load_spread`, `scenario_86_fed_with_the_lift`, `scenario_90_a_take_that_does_not_go_to_starve`, `scenario_91_a_world_from_a_picture`, `scenario_92_a_sides_own_half`, `scenario_93_the_enemys_half`, `scenario_94_a_trial`, `scenario_95_the_estimate`, `scenario_96_the_window`, `scenario_97_a_seats_trial`, `scenario_100_what_ifs_from_a_seat`, `scenario_118_a_dry_run_inside_a_day`, `scenario_135_men_beside_a_place`, and `scenario_136_supply_at_press` once decision 0230 is merged. Scenarios 91 and 92 have tests and no scenario file of their own; 136's file is in a folder.
9. **The scripts.** `scripts/stage-4-proof/run.sh` does not exist and is made for the proof. `scripts/unfed/run.sh` plays seeds 1 to 100; the proof's script makes its own exports of seeds 1 to 1,000 and hands them to the readers, and does not call it.
10. **Condition 9's figures** are corrected as decision 0161 has it ("Condition 9's case, corrected"): rehearsal 100 against 70, fatigue 0 against 7 at the end of day 11, 1,000 fit men against 930. The case, the size, what is read and the mark of 15 worlds in 16 are unchanged.
11. **What the architect played, and on what.** To check three workings, five games in all, each a copy of a case's file with the course written in, kept outside the cases' folder or removed after, on the scenario's own seed, 1942: W1's game (at commit 9fd3861c, before this page was committed), each of R's two courses, and W3's game. Neither R nor W1 rolls a die. Seed 1942 is not among any judged seeds. No trial of any case has been played, no game of S1, S2 or W2, and no game of (a), (b) or (c) by the architect. The copies are removed; nothing temporary is in the folder.

## The architect's rulings on the harness as built, Nov 24, 2026

The harness, the cases and the scripts were built by a developer (commits 86c98651 to ee673654) and reported. Five things came back for a ruling. Nothing of the judged run has been played.

1. **Case W2 is a case, and is now stated.** `data/scenarios/stage-4-proof/case_w2_a_carrier_believed_absent.toml` and `tests/proof_stage_4_case_w2_a_carrier_believed_absent.rs`, four empty tests, written by the architect. It is not made from scenario 93, as this page first said, but from scenario 5b, the unopposed strike, whose chances are already worked by hand: the same forces and orders for 7 days, read from the Japanese seat, which believes the Allied carrier force absent. This replaces W2 in condition 8 and in "Amended on review", 3:
   - **The trial:** of the Japanese side, an empty course, the small size, 8 worlds for 7 days, which is the game's length; the trial's figures and the file of sides are scenario 95's and scenario 93's.
   - **By reasoning:** no enemy is in any world, so the side's ships sunk, ships out of action and aircraft lost are 0 in every world.
   - **The games:** seeds 1 to 200, 7 days each, played once, by the harness, for the judged run.
   - **The mark:** the estimate's highest for ships sunk is 0; the side has a ship sunk in at least 100 of the 200 games; and the list of what was assumed names the carrier force as believed absent and in no world. By scenario 5b's working carried roughly over a week the carrier is sunk in most games; 100 is set well under that and can still fail.
   - The developer fills the four tests, and the harness's part for W2 reads this file at the small size. If a picture with no enemy at all is refused, that comes to the architect before the run.
2. **Which cases may be played before the judged run.** "Amended on review", 11, said no trial of any case had been played. The tests of R, W1 and W3 now play those cases' trials, and W1's and W3's games on their own seeds, and that is right: each of those statements is worked by hand to a figure, and a test that asserts it is the check of the working, as for any scenario. What stays unplayed until the judged run is everything whose mark is a count over games with dice: S1's and S2's trials and their 200 games, W2's 200 games, W3's games of seeds 1 to 20, and every game and trial of (a), (b) and (c). The tests of S1, S2 and W2 play none of those. The report says which cases' figures were known from their tests before the run (R, W1, and W3's trial) and that their part of conditions 8 and 9 is therefore a check that the tests pass and the harness reads the same, not a discovery.
3. **Condition 5, "the same answer".** A seat's answer carries the number of the estimate among those the seat has been shown, and the second asking is number 2. That number is the session's and not the trial's. The reading: the two answers are equal byte for byte with that one field set aside, and the field itself rises by one within the pair, whatever number the pair starts at (the session counts every estimate it has shown, so the pair of the evening of day 3 is 3 and 4; corrected on Nov 24, 2026, before the run: this first said "from 1 to 2"). The harness's file gives both the plain comparison and the one with the field set aside; the mark is read on the second, and the report says so.
4. **Condition 4, what makes a pair.** A pair is good where two things hold, and the harness checks both: the two games differ in truth on the evening of day 3, and the asking side's own half and its beliefs are equal field for field. Whether the hidden land unit is aboard a ship was the architect's way of choosing a candidate likely to pass and is not checked for itself: a unit whose change the side could see would fail the second check. A candidate that does not load or fails a check is void and the next is tried, as "Amended on review" allows; the report names each candidate tried, which was void and why, and which gave the pair.
5. **`Losses::of` made public in `sim-trial`** is accepted. It reads a side's view and gives that side's own losses; it takes no world and gives nothing of the enemy. The harness is the umpire's and reads a game by it so that a game and a world are counted by one function. The crate's source test must still pass, and `docs/engineering/layout.md` needs no change.

**The table of what moved** gains its row for doctrine `supply-62` when decision 0230 is merged.

**The commands of the run,** confirmed, from main at the commit that has decision 0230, each with `--out` a folder outside the repository:

```
scripts/stage-4-proof/run.sh check
scripts/stage-4-proof/run.sh build
scripts/stage-4-proof/run.sh batch
scripts/stage-4-proof/run.sh exports --jobs 4
scripts/stage-4-proof/run.sh sample
scripts/stage-4-proof/run.sh read
scripts/stage-4-proof/run.sh harness --jobs 4
scripts/stage-4-proof/run.sh time --machine "Apple M3 Pro, 11 cores, 36 GB"
```

`time` is run alone on the machine, after the rest. The harness is run with its judged seed, 1, which is its default under `run.sh`. Before them, by hand: `cargo test --workspace` in release (condition 1) and `sh scripts/regenerate.sh` (condition 2). After them: the readers and the marker of condition 11, on the files the harness wrote. W2's tests are filled and the harness reads W2 before any of this is run; that is the one piece of building left.

## Amended before the run, Nov 27, 2026: step 1 of decision 0232 merged first

Written by the architect on Nov 27, 2026. Nothing of the judged run has been played.

**What happened.** This page pinned the labels of main with decisions 0230 and 0231 in: rules label 34 and doctrine `supply-62`. Decision 0232 said the proof was to be run first and its step 1 merged after, and what follows if the order is turned: the architect amends this page before the run, and not after, on the comparison of the proof's own files across the merge and not on the expectation. On the designer's word of Nov 27, 2026, step 1 (pull request 739) was merged before the run, as main at 03d9a25. The commit before it on main is e01ec4d.

**The labels the proof is run at.** State layout `flagplot-state-v60`, unchanged; rules label 35; doctrine `supply-63`; agent interface 56; saved game 29. They are read from the code at 03d9a25 (`world::RULES_VERSION`, `sim_staff::DOCTRINE_VERSION`, `sim_agent::answer::VERSION`, `sim-cli`'s `save::VERSION`, `hash::STATE_VERSION`). Wherever this page says rules label 34, doctrine `supply-62` or interface "55 or the next", these stand in their place. The commit the proof is run from is main at 03d9a25 or a later commit of main that carries these five and no other; the report names it.

**The evidence: the comparison, with the hash lines kept.** Every scenario file of the proof was run with `sim-cli run FILE --events` by the release build of e01ec4d, in its own tree and on its own files, and by the release build of 03d9a25, built again for this on Nov 27, 2026, and the two outputs of each run were compared byte for byte, the day's hash lines in them and not cut:

- (a), (b) and (c), `data/scenarios/stage-3-proof/07a_slice.toml`, `07b_slice.toml` and `07c_slice.toml`, 39 days each, on the files' own figures: with no seed given, with `--seed 1942` (the files' own), and with `--seed` 2001 to 2100. 102 runs of each, 306 in all.
- The six cases, `data/scenarios/stage-4-proof/case_s1_one_unit_against_200.toml`, `case_s2_two_units_against_300.toml`, `case_w1_the_garrison_believed_half.toml`, `case_w2_a_carrier_believed_absent.toml`, `case_w3_an_intent_not_as_stated.toml` and `case_r_preparation.toml`, on the figures each names: with no seed given, with `--seed 1942`, with `--seed` 2001 to 2020, and once more with `--reference data/reference/step1`. 23 runs of each, 138 in all.

444 runs, 2,243,539 lines, 14,027 of them a day's hash: **no line differs, and no run failed on either build.** `make_arrangements.py --check` reports (a), (b) and (c) "as made" at 03d9a25. No file under `data/scenarios/stage-3-proof`, `data/scenarios/stage-4-proof`, `data/scenarios/staff` or `data/reference`, and no script of the proof, differs between the two commits; `sim-trial` differs by the new field of `set_intent`, given as none, and by nothing else.

**What the comparison is, and is not.**

- No judged seed was played for it. The seeds are 1942, which is each file's own and judged by nothing, and 2001 upward, which are no judged seeds of (a), (b), (c) (1 to 1,000) or of the cases (1 to 200, and W3's 1 to 20). The outputs were compared by machine and read by no one for what the games did; nothing of them is kept.
- These files are in no loop that walks `data/scenarios/*.toml` (decision 0072), so the manager's comparison of every file in `data/scenarios`, live and frozen, across the same merge (254 of 254 runs the same with the hash lines kept, 240bee5 against e01ec4d) did not cover them, and this one does. The two together are the whole of what is on file.
- It is the games of the files, played as `sim-cli run` plays them. A trial and a seat's answer were not played on the build before: its harness is not built there, and no room was made for it. What they rest on is read from the difference of the two commits and is so named: `sim-trial` as above; the trial's answer changed by its version number alone (`schemas/trial.schema.json`); and conditions 4 and 5 compare two answers of one build. The estimates of condition 11 will carry interface 56. If the tests of R, W1 or W3 no longer hold a trial's figure they held before step 1, that is condition 1 failing and is reported as that.

**The table of what moved** has its row for `supply-63` and its labels as above.

**The commands of the run** are those of "The architect's rulings on the harness as built", from main at the commit named above.

**Step 2 of decision 0232 is not in this, and is held until the run.** Step 2 raises the agent interface to 57 and leaves the rules label and the doctrine alone. It is not a change of a number only: with it "the trial's answer names the staff's plan" (decision 0232, "Versions, the hash and the files"), and the trial's answer is what conditions 5 and 11 read, and what condition 11's readers are shown. So the interface is pinned here at 56 and not at "56 or the next": if step 2 merged before the run this page would be untrue, and the six estimates of the legibility test would not be the ones this page set up. The proof is run before step 2 merges. Step 2 may be built on its branch meanwhile. If it must merge first all the same, that is a second amendment by the architect before the run, on its own comparison and on a reading of what the trial's answer then shows a reader, and not on this one.

## Amended before the run a second time, Nov 27, 2026: step 2 of decision 0232 merged first

Written by the architect on Nov 27, 2026. Nothing of the judged run has been played.

**What happened.** The amendment above pinned the agent interface at 56 and at no later one, and said that if step 2 merged first a second amendment would be made on its own comparison and on a reading of what the trial's answer then shows a reader. On the designer's word of Nov 27, 2026, step 2 (pull request 748) was merged before the run, as main at b9e488a. The commit before it on main is a9d1b95.

**The labels the proof is run at.** State layout `flagplot-state-v60`, rules label 35, doctrine `supply-63` and saved game 29, as above; agent interface 57. Wherever this page says agent interface 56, 57 stands in its place. The paragraph above that holds step 2 until the run stands as it was written and is replaced by this section.

**The commit the proof is run from is b9e488a,** in a checkout of that commit, with its own `scripts/stage-4-proof/run.sh`; the report names it. It replaces "main at 03d9a25 or a later commit" above. Main has moved past it, to d903b77 when this was written, and the proof is not run from there: what follows was compared at b9e488a and at no later commit ("Main after b9e488a", below).

**The games: by identity, not by comparison.** Between a9d1b95 and b9e488a no file differs under `crates/sim-core` but one test (`command/tests.rs`, a case added to the tree's invariant), and none under `crates/sim-staff`, `crates/sim-data`, `crates/sim-trial`, `data/` or `scripts/`, nor `tests/proof_stage_4_harness.rs`. In `sim-cli` the files that differ are `seat_trial.rs`, `staff_tools.rs`, `mcp.rs` and `timing.rs`, none of them on the path of `sim-cli run`. The 444 runs of the first amendment would be the same code on the same files and are not made again.

**The trial's answer: read, and compared.**

- *Read from the difference.* Every new branch of `trial` asks for a parent on the intent tried, a sub-intent the commander made under it, or `under` on a preview. The harness tries intents with no parent and nothing written under them, by number, and previews nothing with `under`; no command of a file of the proof names a parent. For such an intent the commands handed to each trial, the default of `compare`, every field of the answer and every word are as before: the answer's `beside` gains `order` and `with_your_plan` only where the second trial is the staff's plan, which none here is. So the six estimates of condition 11 and the answers of conditions 4 and 5 show a reader what they showed, and differ by the version number alone.
- *Compared.* In a checkout of each commit, a9d1b95 and b9e488a: `scripts/stage-4-proof/run.sh build --out DIR`, then `scripts/stage-4-proof/run.sh harness --seed 2001 --out DIR`, run by a developer, the bytes compared again by the manager. Each wrote the same 14 files under `harness/`: `condition_4`, `5`, `7`, `8`, `9`, `12` and `13_units` (`.txt`), and `legibility/estimate1.json` to `estimate6.json` with `key.md`. Seven are the same byte for byte. Each of the six estimates differs in one line, `"version": 56` against `"version": 57`. `condition_4.txt` differs in one line, ruled on below. With those two set aside, all 14 are the same byte for byte.
- *The line of `condition_4.txt`: accepted, and nothing of the build.* It is the loader's words for a candidate pair that does not load, which the harness names as void (the rulings on the harness, 4), and the words begin with the path of the scenario file, which holds the name of the folder the commit was checked out in: `.../cmp-a/tests/../data/scenarios/stage-3-proof/07a_slice.toml` against `.../cmp-b/...`. The rest of the line is the same. So the judged run's `condition_4.txt` carries the path of the checkout it is run in; the report says so, and no mark reads it.

**What the comparison played, and what it did not.**

- With `--seed 2001` every game the harness plays takes its seed from 2001 upward (`first_seed` and `seeds` in `tests/proof_stage_4_harness.rs`, from `FLAGPILOT_STAGE4_SEED`): seeds 2001 to 3000, none of the judged 1 to 1,000. Conditions 9 and 13 play no game.
- **A trial does not take the harness's seed.** Every trial draws on the seed of its trial's figures, 1942 (`data/reference/trial.toml` and the cases' files), and not on the game's. So the comparison played the trials of S1, S2 and the other cases as the judged run will, and wrote estimates 3 to 6 as the judged run will write them; and estimates 1 and 2 are asked on the evening before day 1 of (a), before the game's seed has drawn anything, so they are expected to be the judged run's as well. That is so of every earlier trying of the harness on seed 2001 too, and the rulings on the harness, 2, did not say it. What stays unplayed is every mark that is a count over games with dice, and every trial asked after a day of a judged game: no game of a judged seed was played. The outputs were compared by machine and read only at the lines that differ; no figure of a trial was read from them, and none is to be before the run. They are kept outside the repository for the manager's check and are deleted before the run. The key of condition 11 is written by the harness from each estimate's fields and by no one who has read them, so the legibility test is not touched by this; the report says that the estimates had been written once before the run, unread.

**Main after b9e488a.** Three pull requests are on main after it, to d903b77: 749 (decision 0236, documents), 750 (the loader's `scenario.rs` split into `crates/sim-data/src/scenario/`, decision 0234, step A) and 747 (one test program for each package; the harness is a module of the program `scenarios`, and `run.sh` asks for it as `--test scenarios` with the filter `proof_stage_4_harness::NAME`). Between b9e488a and d903b77 no file differs under `crates/sim-core`, `data/`, or the `src` of `sim-staff`, `sim-trial`, `sim-agent` and `sim-cli`. The loader's source and the way the harness is built and called do differ, and neither was compared on the proof's files, which no loop over `data/scenarios/*.toml` walks. So the proof is run from b9e488a. To run it from a later commit is a third amendment by the architect, before the run, on two comparisons against b9e488a: the 444 runs of the first amendment, and the harness with `--seed 2001`, all 14 files, the path's line aside.

**The commands of the run** are those of "The architect's rulings on the harness as built", in the checkout of b9e488a, each with `--out` a folder outside the repository. The legibility files are copied from there to `crates/sim-cli/tests/recorded_games/legibility_stage_4/` on main, and the report is written on main.

## Amended before the run a third time, Nov 28, 2026: condition 10 loses its "Full" column

Written by the architect on Nov 28, 2026. When this was written, of the run only `run.sh check` and condition 1's tests had been started, in the checkout of b9e488a; no game of a judged seed, no trial and no time had been played or measured.

**What happened.** The designer said on Nov 28, 2026, as the manager relayed it: "Unless you need it for the proof, I don't think we need to run full dry runs. Only small and standard, as the full ones are just too long in a gameplay." The architect was asked, before anything at the full size was run, whether the proof needs it. The answer: condition 7 does, since its mark of 180 games in 200 rests on (n - 1) / (n + 1) at 64 worlds and would not hold at 16 ("The arithmetic of the mark", above); to read it at the standard size the mark would fall to about 130 of 200 and the band of the count would widen from 0.19 to 0.34, and a trial whose spread is plainly too narrow would pass both. The designer's answer, as the manager relayed it, Nov 28, 2026: "Keep full for the proof, strike full time limits". Those two sentences are all he said. (Mended on review after the run, Nov 28, 2026: this section first paraphrased him as saying the full size is to be offered to no player, which goes further than his words; whether the full size is withdrawn from a player is not ruled and is left open for him.)

**What is changed.** In condition 10 the column "Full" of the table of marks is struck: 25 s for a trial, 40 s for a trial beside another and 74 s for a window are marks no longer, on his word. The columns "Small" and "Standard" stand as written. He struck time limits and did not name the mark of no process above 900 MB at its peak: reading it on the processes of the two sizes left is the architect's consequence and not his word, and whether it is to be read at the full size too is a question for him. If times at the full size are measured they are reported and judged by nothing.

**What is not changed.** Condition 7 is judged at the full size, 64 worlds for 21 days, as written, with the small and standard sizes reported beside it. Condition 12 gives its estimates at the full size, as written. The trial's figures are not touched: `data/reference/trial.toml` and the cases' files name small, standard and full at b9e488a, and the proof reads them as they are. What sizes the table offers a player is the table's matter and no condition reads it.

**Why this is no mark moved to fit a result.** No time of the proof has been measured, on this machine or the named one. The mark struck was the hardest of the row to meet only in the sense that it was the longest; it is struck on the designer's word, given for the length of a full dry run in a game, and not because of any reading.
