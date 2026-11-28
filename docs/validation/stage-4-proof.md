# Step 3, stage 4: the proof of the trial

Run on Nov 28, 2026, by the architect, to the thirteen conditions of decision 0161 as `docs/validation/stage-4-proof-setup.md` reads them, that page written before the run and amended three times before anything of it was judged. Release build, from a checkout of main at b9e488a (the merge of pull request 748, step 2 of decision 0232), with that commit's own scripts: state layout `flagplot-state-v60`, rules label 35, doctrine `supply-63`, agent interface 57, saved game 29. Nothing was tuned, and no rule, figure, reading, mark, case or file the proof reads was changed after the run. One page that holds no result was corrected with this report: the table of what moved, `stage-4-proof-what-moved.md`, whose row for the agent interface still read 56 and now reads 57, as the setup's second amendment has it.

**Result: not passed as stated.** Of the twelve judged conditions eight are met, three are missed and one could not be read here.

- **Missed, and expected to miss:** conditions 3 and 13 on the Japanese side, at 0.0692 and 0.1161 against a mark of 0.05, their cause named before the run.
- **Missed, and not expected:** condition 11, legibility. Two of six estimates pass, against a mark of six.
- **Not read:** condition 10, the budget. Its marks are tied to a named machine and this run was made on another.

This report is written before the designer has decided anything on it.

| Condition | What was read | Mark | Met |
| --- | --- | --- | --- |
| 1, the hand calculations | `cargo test --workspace --release`: 5,840 tests pass, none fails, 23 are ignored; the fourteen named scenarios and the six cases pass | pass | yes |
| 2, nothing changed where it is unused (the weaker reading of the setup's amendment 7) | `sh scripts/regenerate.sh` changes no file; the baselines' tests pass | no file | yes |
| 3, men landed are fed, the Allied side | median share lost unfed 0.0000 of those landed, on 1,000 seeds; the same on seeds 101 to 1,000. Beside it, each a median of its own: 16,975 men landed, 0 lost | 0.05 | yes |
| 3, men landed are fed, the Japanese side | median share **0.0692**, on 1,000 seeds; the same on seeds 101 to 1,000. Beside it, each a median of its own and not one game: 6,076 men landed, 415 lost | 0.05 | **no, as expected** |
| 4, no peek | scenarios 93 to 97, 100 and 135 pass; on the 39 days at three days both pairs are good and both pairs of answers are equal byte for byte | equal | yes |
| 5, the same twice, and nothing changed | six pairs of answers equal with the answer's own number set aside; 39 daily hashes the same with a trial asked every morning, both seats | equal | yes |
| 6, its sampling is right | scenario 95's 22 tests pass | pass | yes |
| 7, its spreads hold, S1 | men lost to the enemy 192 of 200 games inside, men lost unfed 199; K/64 = 0.7656, M/200 = 0.7250, apart by 0.0406 | 180; 0.19 | yes |
| 7, its spreads hold, S2 | men lost to the enemy 198 of 200 inside; K/64 = 0.9844, M/200 = 1.0000, apart by 0.0156 | 180; 0.19 | yes |
| 8, a wrong picture moves it | W1, W2 and W3 each wrong in the direction stated, each naming in its assumptions the thing that was wrong | all three | yes |
| 9, preparation matters | the course begun on day 1 has more fit men ashore in 16 of 16 worlds (1,000 against 930) | 15 | yes |
| 10, within its budget | not read: the run was not made on the named machine | the table, less its "Full" column | **not read** |
| 11, legible | **2 of 6** estimates pass; 26 of 30 answers agree | 6 | **no** |
| 12, the estimate beside the games | reported below | none | not judged |
| 13, men who begin ashore are fed, the Japanese side | median **0.1161** of 2,480 men lost unfed where a base of the side stood, on 1,000 seeds; the same on seeds 101 to 1,000 | 0.05 | **no, as expected** |
| 13, the Allied side | no man of the side begins ashore | 0.05 | does not arise |
| The same start gives the same game | each arrangement's two batches of 1,000 the same; five exports of each made twice, the same | identical | yes |

## What was run, and what was not as the setup has it

In a checkout of b9e488a, on a 4-core x86-64 Linux machine with 15 GB of memory, in this order: `run.sh check` (the three arrangements "as made"); `cargo test --workspace --release`; `sh scripts/regenerate.sh`; then `run.sh` for the build, the batches, the 1,000 exports of (a), the sample, the readers and the harness, with its judged seed, 1; then the six readers and the marker of condition 11. The outputs are kept outside the repository, less the exports.

Five things were not as the setup's list of commands has them. None changes a figure: every game is fixed by its file and its seed.

1. **The machine is not the one named.** Condition 10 names an Apple M3 Pro of 11 cores and 36 GB, and `run.sh time` asks macOS's `time -l` for a process's memory. Neither was to hand. Condition 10 is not read (below).
2. **`regenerate.sh` was started twice.** The first time the architect's own command put the build in a folder where the script does not look; it found no program, emptied eight baseline files and stopped. The files were restored from the commit and the script run again as it stands: it changes no file. Nothing of a judged seed was played by it.
3. **The steps were started twice.** The first start ran one after another and was stopped ten minutes in, during the first batch of (a) and before any file was written, to run the batches beside the other steps. No output of it was read; no game was made twice and none was set aside.
4. **The batches were made side by side.** A batch of 1,000 games takes about three quarters of an hour on this machine, and the script makes six in a row. (a)'s first was made by `run.sh batch`, whose loop was then stopped while that batch ran on, so the step ended with exit 143 and made no second batch; the other five were made by the same command the script gives `sim-cli`, three at a time. (a)'s first file is whole: it is the same byte for byte as its second, which was made apart.
5. **Exports and the harness's games were run three at a time, not four** (`--jobs 3`). The files are the same whatever the number.

`condition_4.txt` carries the path of the checkout in the line of a candidate that does not load, as the setup's second amendment said it would. No mark reads it.

## What had been seen before the run

As the setup lists, and repeated here:

- **Seeds 1 to 100 of (a)** were played and read for the men lost to hunger before and after decision 0231, and that rule was written from them. Conditions 3 and 13 are therefore given on seeds 101 to 1,000 as well: every median is the same to four places, and the verdicts are the same.
- **Seeds 29, 40 and 54** were read day by day for what binds the Japanese supply.
- **The times** of a trial were measured as each step was built and accepted by the designer.
- **The figures of R, of W1 and of W3's trial** were known from their tests, which work each by hand. Their part of conditions 8 and 9 is a check that the tests pass and the harness reads the same, not a discovery.
- **The estimates of condition 11** had been written once before, on Nov 27, 2026, when the harness was tried on seed 2,001 across the merge of step 2 (a trial draws on its own seed, 1942, and not the game's). They were compared by machine and read by no one. The key is written by the harness from each estimate's fields.

What nobody had played or read: S1's and S2's 200 games, W2's 200, W3's 20, and every game and trial of (a), (b) and (c) on a judged seed.

## The two misses expected: Japanese men ashore are not fed

| | Seeds 1 to 1,000 | Seeds 101 to 1,000 | Mark | Expected |
| --- | --- | --- | --- | --- |
| Condition 3, the Japanese side: men lost unfed over men landed, the median | 0.0692 | 0.0692 | 0.05 | about 0.07 |
| Condition 13, the Japanese side: men who begin ashore lost unfed where a base of the side stood, over those men, the median | 0.1161 | 0.1161 | 0.05 | about 0.12 |

**Condition 3.** The mark is read on the median over the seeds of each game's share, the men lost unfed over the men landed at a place not the side's: 0.0692. Two other medians are given beside it, each taken over the seeds by itself: 6,076 men landed and 415 lost unfed. They are not one game's figures, and 415 over 6,076 is 0.0683, not the median share. The share is at or under the mark on 103 of the 1,000 seeds, and lies between 0.0214 and 0.2920. Decision 0159's proof read 0.2537 on the same seeds by the same script: the fall to 0.0692 is the work of every row of `docs/validation/stage-4-proof-what-moved.md`, the doctrine's labels 40 to 63, and not of the mends of decisions 0162, 0163 and 0169 alone.

**Condition 13.** Five Japanese units begin ashore, 2,480 men: 2,040 at Lunga's hex on island 1 and 440 at Tulagi's on island 2. Every one is on the land of an island the side states, so the script's count is the reading's (`condition_13_units.txt`). The median game loses 0.1161 of them unfed in a hex where a base of the side stood; the share is above the mark on every seed, from 0.0996 to 0.4738. Reported beside it and judged by nothing: the same units lose a median of 0.1137 of their men unfed where no base of the side stood, and 0.2298 in all.

**The cause, as named before the run** and not read again here: the scenario gives the Japanese side six transports before day 23, in two forces usable from days 10 and 12; both are first used for the landings at Lunga; ships are lost off Lunga; and the garrison's 71 t at (2,-5) is gone on day 13, before a ship can be back. Three costs of the supply staff's own working add days: two mornings between a ship in port and its orders, a finished run's ships held two to three days, and a force that goes whole to one base. The designer was asked on Nov 24, 2026, and the proof was run with none of the three mended first. They are left for after (`docs/open-work.md`).

**The Allied side** meets condition 3 as expected, with a median of 0. The median hides a tail: on 491 of the 1,000 seeds the side loses some man unfed, and on 329 more than 0.05 of those landed, 0.2357 at the most. Decision 0159's proof read a median of 0.0346, all of it late in the game at the side's own bases. The mark is the median's and is met; the tail is reported.

## The miss not expected: legibility, 2 of 6

`crates/sim-cli/tests/recorded_games/legibility_stage_4/` holds the prompt, the six estimates as a seat is answered, the key, the answers and the marks. Six Haiku readers (`claude-haiku-5-5`), each given one estimate and the five questions and nothing else; a Sonnet marker (`claude-sonnet-5-5`) held to the key; the architect read the marks. Run once.

| Estimate | Needs | When | Worlds | Cost | Assumed | Passes |
| --- | --- | --- | --- | --- | --- | --- |
| 1. (a), the Allied side | agrees | **disagrees** | agrees | agrees | agrees | no |
| 2. (a), the Japanese side | agrees | agrees | agrees | agrees | agrees | yes |
| 3. S1 | **disagrees** | agrees | agrees | agrees | agrees | no |
| 4. W1 | **disagrees** | agrees | agrees | agrees | agrees | no |
| 5. A window | agrees | agrees | agrees | agrees | **disagrees** | no |
| 6. A trial beside another | agrees | agrees | agrees | agrees | agrees | yes |

Of 30 answers 26 agree. No reader got a number of worlds or a cost wrong. The four that fail:

- **Estimates 3 and 4, what it needs.** Each estimate says "a count of the enemy at (4,0) is asked for, and the one held is 200 men, told on day 0, which does not meet it". Both readers gave the men and the task and left the count out.
- **Estimate 1, when it could go.** The estimate says it was tried "from the morning of day 1" and that "no day ashore can yet be given". The reader gave the second and left out the first.
- **Estimate 5, what was assumed.** The reader called 200 men "believed present" whom the estimate gives as "believed, standing unknown". The men and the worlds are right; the standing is not.

**The cause is put to the estimate's reading, with two things said against the test itself.**

1. *The estimate's reading.* Three of the four are a thing left out of an answer that runs to between 8,000 and 37,000 characters, in which each thing assumed names every world by number. What a plan needs is one sentence among a dozen, and the count it asks for is the last clause of it. A reader with one or two sentences for each question takes the head of the sentence. An answer that put what is needed and not yet had on a line of its own would likely be read whole.
2. *The question "when could it go" does not fit a trial that is no window.* Such a trial says when it was tried from and, for a take, the day expected ashore. For an island it gives no day at all. The reader of estimate 1 answered what the estimate said about a day.
3. *The marker's words were not fixed before the run.* The setup fixed the reader's prompt and said the marker is "held to the key". The words given the marker are the architect's, written on the day of the run before any answer was seen and kept in `marks.md`; they ask that nothing the key requires be left out. The key is every field of the estimate, so a strict marker fails any omission. A setup that listed the points each answer must hold would have been a fairer test. That is a fault of the setup and is the architect's.

Two of the four marks are arguable. The fourth is the thinnest: the marker passed reader 6, who gave no standing for the same men. The mark on estimate 1 is arguable too: "it waits for nothing" is a true answer to "when could it go", and the marker passed readers 2, 3, 4 and 6, none of whom gave the last day. A fair range for the count is between two and four of six. The condition is missed anywhere in that range, and nothing was asked a second time.

What it did not reach: no estimate at the full size was read, and the two of the 39 days are of the evening before day 1.

## Condition 10: not read

The marks are times on a named machine, measured with no other job running, and a process's peak memory. This run was made on a machine of 4 cores against 11, with another processor and another system, and the script's way of reading memory does not work on it. A time taken here would say nothing of the mark, met or missed, and none is given as a reading.

**The "Full" column is struck** (the setup, "Amended before the run a third time", Nov 28, 2026). The designer's two sentences of that day, as the manager relayed them: "Unless you need it for the proof, I don't think we need to run full dry runs. Only small and standard, as the full ones are just too long in a gameplay." and "Keep full for the proof, strike full time limits". He struck the time limits of the full size and said no more: whether the full size is withdrawn from a player is not ruled and is his to say. What is owed is the six times of the small and standard sizes from `run.sh time` on the named machine in a checkout of b9e488a. The mark of 900 MB at a process's peak was not named by him: the architect reads it on the processes of the two sizes left, as a consequence and not as his word, and whether it is read at the full size too is a question for him. That step plays no judged game and reads nothing the rest of the proof made, so it can be run by itself.

One thing bears on it, seen in `docs/open-work.md` and not measured here: on main after b9e488a a trial of the Allied side on (a) is recorded as far slower where every side chooses its sources by the one rule. That is not the proof's commit, and condition 10 is read at b9e488a.

## The conditions that are met

**1. The hand calculations.** 5,840 tests pass and none fails. By name: scenarios 85 (18 tests), 86 (23), 90 (17), 91 (17), 92 (17), 93 (29), 94 (22), 95 (22), 96 (23), 97 (21), 100 (29), 118 (16), 135 (14) and 136 (9), and the cases R (4), W1 (3), W2 (4), W3 (4) and S1 with S2 (2). None has a test marked "statement written, not built". One test of those files is ignored: scenario 97's `refused_while_a_day_is_open`, "superseded by decision 0204: a trial is answered inside a day, at 0600 and 1200 (scenario 118)". The other 22 ignored tests are elsewhere, the harness's eight among them. `regenerate.sh` lists two test files with no entry in `tests/Cargo.toml`, so not run: `scenario_75_leaving_a_place.rs` and `scenarios_built_on_another.rs`; neither is among the files this condition names.

**2. Nothing changed where it is unused,** read as the setup's amendment 7 has it and not as "every game as before step 3", which cannot be played again. `regenerate.sh` at the proof's commit changes no run baseline, agent golden, recorded game, pinned hash, batch fixture or submarine table, and the baselines' tests pass. That asking a trial changes nothing is condition 5. What the five mends moved, from each record's "As built":

- *Decision 0162:* scenario 70's run baseline, in the words of its reasons and in its property "The night's limit"; every other run baseline, the recorded games, the pinned hashes and the batch fixture in their versions and hashes only.
- *Decision 0163:* scenario 77's run baseline in the words of the source's reason; the scripted full slice from day 1, by Rabaul's fuel; the recorded games in their doctrine version, and `full_slice_five_days_sonnet` in its hashes too. No transcript.
- *Decision 0169:* no run baseline but scenario 90's, which is new; the recorded games in their doctrine version alone.
- *Decision 0230:* the doctrine's label in the five recorded games' saves and the tests' pins that name it. No run baseline, agent golden, pinned hash or batch fixture.
- *Decision 0231:* by its plan, the labels wherever they are held, and by what the staff does the three arrangements of this proof and the staffed games of `five-days`, `coral-sea`, `table-hunger`, `table-boats` and `table-rear-bases`. Its "As built" says the remaking of the pins, the recorded games and the full slice's baselines was not finished in that build and does not list the files. They are as remade on main since, and the proof's commit regenerates to itself; the list the condition asks for cannot be given from that record.

**4. No peek.** On (a), seed 1, the evening of day 3, the small size.

- *The Allied seat:* Rabaul's supply raised by 1,000 t and one Japanese land unit's men changed by 100. The first candidate, land unit 25, was void: the copy does not load, its ships having berths for 400 of its 433 men. The second, land unit 24, gave the pair: the state hashes differ, the side's picture is equal field for field, and the two answers are equal byte for byte (11,843 bytes).
- *The Japanese seat:* Espiritu Santo's supply raised by 1,000 t and the fuel of ship 49 lowered by 100 t. The first candidate gave the pair: hashes differ, picture equal, answers equal byte for byte (12,525 bytes).

**5. The same twice.** For each seat, on the evenings before day 1, of day 3 and of day 20, the two answers are equal with the answer's own number set aside, on which the mark is read, and that number goes from 1 to 2, 3 to 4 and 5 to 6. As they stand the pairs are not equal byte for byte, by that number alone. The game played to day 39 with the trial asked on every morning has the daily hash of the game never asked on all 39 days, both seats.

**7. Its spreads hold,** at the full size, 64 worlds for 21 days, beside 200 games.

| | S1 | S2 |
| --- | --- | --- |
| Costs tested | 2 | 1 |
| Men lost to the enemy: the worlds' lowest, middle, highest | 20, 80, 160 | 40, 100, 260 |
| Games inside, of 200 | 192 | 198 |
| Men lost unfed: the worlds' lowest, middle, highest | 0, 0, 33 | 0, 0, 0: not tested |
| Games inside, of 200 | 199 | |
| The end met and kept: worlds of 64; games of 200 | 49 (0.7656); 145 (0.7250) | 63 (0.9844); 200 (1.0000) |
| The difference | 0.0406 | 0.0156 |
| Costs whose middle half is wider than the middle | 0 | 0 |

Ships sunk, ships out of action and aircraft are 0 in every world and every game of both cases: not tested. A right trial misses the mark of 180 about once in 70 for a cost, and falls outside the band of 0.19 about once in 100 for a case. S2 shows little of the count: the end is met in every game and in all but one world.

Beside it and judged by nothing, the games inside for men lost to the enemy at the smaller sizes, where a right trial is expected to cover 0.78 and 0.88 of games: S1, 125 of 200 at small and 173 at standard; S2, 183 and 189. The end met and kept: S1, 4 of 8 worlds against 100 of 200 games at small and 9 of 16 against 100 of 200 at standard; S2, none of 8 against 1 of 200 at small, and all 16 against all 200 at standard, where the case showed nothing of the count.

**8. A wrong picture moves it.**

- *W1, the garrison believed half the truth.* The estimate says day 4 and 100 men in every one of 16 worlds. The game, seed 1, takes the place on day 5 with 250 men lost: a day later and 150 men dearer, as worked by hand. The assumptions name "200 men at (4,0), from the side's estimate".
- *W2, a carrier force believed absent that is there.* The estimate's highest for ships sunk, ships out of action and aircraft is 0 over 8 worlds. The side has a ship sunk in 168 of 200 games, against a mark of at least 100, and a ship sunk or out of action in 199. The assumptions name "believed force 1 (Allied carrier force believed elsewhere); believed absent, ruled out in 8 worlds".
- *W3, an enemy intent not as stated.* The estimate says met and kept on day 4 in every one of 16 worlds, with 100 men lost at the highest. In each of the 20 games the side has lost 300 or 350 men by the end of day 8, more than the estimate's highest in all 20. The place is kept in every game. The assumptions say "the enemy given no intent".

**9. Preparation matters.** Case R, two trials of one number at the standard size. On the evening of day 11 the unit is ashore in both courses in all 16 worlds, with 1,000 fit men in the course handed in on day 1 and 930 in the course handed in on day 4, as worked by hand. Neither course rolls a die, so the 16 worlds are one world sixteen times: the condition shows the rule acting in a trial and nothing of its spread.

**6** is condition 1's scenario 95.

## Condition 12: the estimate beside the games, reported and not judged

On (a), from the evening before day 1, each side's first intent, "control an island" for island 1, at the full size, beside the 1,000 games to the end of day 21. Costs are the lowest, the middle and the highest.

| The Allied side | The estimate, 64 worlds | The games of (a) | The games of (c), the enemy given nothing |
| --- | --- | --- | --- |
| The end met and kept | 3 worlds, on days 18, 18 and 21 | 0 of 1,000 | 1,000, on day 1 |
| Men lost to the enemy | 500, 1,880, 2,640 | 89, 217, 367 | 0, 60, 120 |
| Men lost unfed | 0, 0, 0 | 0, 0, 0 | 0, 0, 0 |
| Ships sunk | 0, 0, 1 | 0, 0, 2 | 0, 0, 0 |
| Ships out of action | 0, 0, 7 | 0, 0, 2 | 0, 0, 0 |
| Aircraft | 15, 28, 43 | 28, 56, 157 | 13, 30, 48 |

| The Japanese side | The estimate, 64 worlds | The games of (a) | The games of (b), the enemy given nothing |
| --- | --- | --- | --- |
| The end met and kept | 64 worlds, on day 1 | 0 of 1,000 | 1,000, on day 1 |
| Men lost to the enemy | 0, 0, 0 | 440, 526, 713 | 0, 0, 0 |
| Men lost unfed | 114, 114, 114 | 606, 833, 1,079 | 114, 114, 114 |
| Ships sunk | 0, 0, 0 | 0, 1, 5 | 0, 0, 0 |
| Ships out of action | 0, 0, 0 | 0, 2, 7 | 0, 0, 0 |
| Aircraft | 3, 7, 14 | 9, 32, 175 | 9, 37, 87 |

Against a side given nothing, in (c), the Japanese side loses 440, 500, 590 men to the enemy and 825, 887, 926 unfed; in (b) the Allied side loses 0, 600, 4,200 men, 0, 2, 7 ships sunk and 0, 7, 19 out of action.

Neither estimate is near the games of (a), and each is wrong by its picture, in the way condition 8 shows on built cases.

- **The Japanese side's estimate is the game against an enemy who does nothing.** Its beliefs give the enemy no intent, so in every world he holds what he holds, and the estimate is the games of (b) in men and in ships, though not in aircraft. In the game the Allied side lands on day 1.
- **The Allied side's estimate fights a garrison two and a half times the true one at Lunga and four times at Tulagi.** Its opening estimates are 5,275 men at Lunga's hex and 1,850 at Tulagi's; 2,040 and 440 are there. Every game of (a) costs it fewer men than the estimate's lowest world.
- **Aircraft are understated on both sides.** A picture holds no land air group at an enemy base (decision 0174).

The beliefs of the 39 days are placeholders (decision 0173), and the opening estimates are those of decision 0160. So this says little of the trial and much of what it is given: on the one scenario of play the proof reads, a commander who asked for an estimate on the first evening would be told something far from what follows, on either side.

## What the proof does not show, and what the stage leaves

- **A window is "decide later".** A later date of a window is the order given later; nothing is prepared in the days between. The spec's "what if we wait" asks for those days to be played with preparation, and the stage does not do that. Condition 9 shows that preparation matters by two courses, at press and with a day named; at balanced an earlier start buys a date and not fitness (decision 0177, the reading of Nov 24, 2026).
- **What a picture lacks:** an enemy's land air groups; the kind of a contact, taken as reported; the beliefs of the 39 days, which are placeholders; and an air group or a supply run that comes due makes no date of a window.
- **What the stage leaves out by its plan:** looking back at a live game; a staff that chooses by trial; and step 10, the enemy's approaches.
- **Owed from the stage and not judged here:** a plan displaced by a take of another place (decision 0176); a staff that reckons a force forward from its own orders (decision 0136); the tests across a picture of four rules that read earlier days (decision 0170).
- **The proof is of b9e488a.** Main has moved on since, in `sim-core`, in the loader and in the reference files among others, and nothing of main after that commit was compared on the proof's files. What is said here is said of that commit.
- **After the proof:** the Japanese runs as the destroyers that made them (decision 0165, steps 5 and 6), with conditions 3, 12 and 13 read again on that file and reported beside these, and no second judgement.

## What it raises, for the designer

1. **Conditions 3 and 13 on the Japanese side: name the misses, as was foreseen.** The architect's recommendation. The figures are the ones expected, and the three costs of the supply staff's working are in `docs/open-work.md`, each wanting its own record and hand calculation. Conditions 3 and 13 are read again after any of them is built and reported beside these, with no second judgement.
2. **Condition 11: the miss is real and was not foreseen.** The choices: name it and go on; or mend the answer and test again as a new test, said to be one. The architect's recommendation is the second, in two parts, neither made here: the estimate's words put what a plan needs and has not got on a line of its own, and say in one plain sentence when it was tried from and when it expects to be ashore; and a second legibility test is set up with the points each answer must hold and the marker's words fixed before any estimate is written. The mend changes what a seat is told and raises the agent interface, so it is a decision record with its plan first.
3. **Condition 10: whether it is to be read at all, and where.** The small and standard times at b9e488a are owed on the named machine. Until they are read the proof has one condition open, and the stage cannot be called proved or closed on its marks. Two things his word of Nov 28, 2026 left open are his as well: whether the full size is withdrawn from a player, and whether the mark of 900 MB at a process's peak is read at the full size or only at the two sizes left.
4. **Condition 12: the estimates of the 39 days are far from the games because the beliefs are placeholders.** Not judged, and no fault of the trial. But the opening estimates and the beliefs of decision 0173 are what a player's first dry run rests on, and the Japanese side's believes in an enemy who will not come. Whether they are settled from the record before the trial is put before a player is the designer's.
5. **Not done:** nothing is run again to turn a miss into a pass, and no reader or marker was asked twice.
