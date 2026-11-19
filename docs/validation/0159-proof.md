# The proof of decision 0159: a plan that knows what it waits for

Run on Nov 19, 2026, by the architect, to the conditions of decision 0159 and to the readings written into that decision before the run ("The proof: how each condition is read"). Release build, live figures, seeds 1 to 1,000, from the branch's commit 450dfba1, which is main at 708e8fc3 (the merge of pull request 451, step 6) with the reading scripts and nothing else: state layout `flagplot-state-v47`, rules label 19, doctrine `supply-39`, agent interface 26, saved game 22. Nothing was tuned, and no rule, figure, reading, mark, intent or file was changed after the run.

**Result: not passed as stated.** One condition fails: the Japanese side's men landed at a place it has not taken still starve, one in four. Every other judged condition meets its mark.

| Condition | Value | Mark | Holds |
| --- | --- | --- | --- |
| 1, the hand calculations | the tests of scenarios 79 to 84 pass on main at 708e8fc3 (CI success) | pass | yes |
| 2, nothing changed where it is unused | 98 scenarios with no hold and no take, 194 run files: 0 lines differ that are not hash lines | none | yes |
| 3, men landed are fed, the Allied side | median lost to hunger 0.0346 of those landed (587 of 16,975), on 1,000 seeds | 0.05 | yes |
| 3, men landed are fed, the Japanese side | median **0.2537** (1,532 of 6,076), on 1,000 seeds | 0.05 | **no** |
| 4, no take goes short when waiting would have brought enough | 0 of 12,000 lifts | none | yes |
| 5, a plan that waits says what for | the five built cases pass in scenario 84's rows and in the day-by-day tests of scenarios 80, 81 and 82 (CI success) | pass | yes |
| 6, legible | 6 of 6 orders | 6 | yes |
| 7, a seat is told nothing more | `crates/sim-agent/tests/leaks.rs` and scenario 84's "nothing more is told" pass (CI success) | pass | yes |
| The same start gives the same game | each arrangement's two batches byte for byte the same; five exports of each made twice, the same | identical | yes |

## What was run

The three arrangements of `data/scenarios/stage-3-proof/`, each still the file its script makes (`make_arrangements.py --check`): (a) both sides from intents, (b) the Allied side given nothing, (c) the Japanese side given nothing. With `scripts/0159-proof/run.sh`, in order: the build; each arrangement's batch of 1,000 seeds twice; the 1,000 exports of (a); the sample of seeds 1, 2, 3, 500 and 1,000 of each, exported twice; the six plan orders; the readers. The exports step was started twice: the first time the architect's own command passed its options wrongly and the script stopped at once, before any export; no game was made twice and none was set aside. The outputs are kept outside the repository.

## The condition that fails: Japanese men ashore are not fed

The median Japanese game lands 6,076 men at Lunga's hex after the Allied side has taken the airfield, and loses 1,532 of them unfed by day 39, every one while the place was not the side's. Stage 3 saw 1,543 of 6,076. Step 1 of this decision, "fed ashore", was built to mend that, and in this game it does not.

The rule itself acts. On seed 1 the Japanese staff says so in its own reasons:

- Day 11: the hold of Lunga "is not the side's: the staff waits for it to be the side's, and until then forms nothing and orders nothing under intent:3 but the supply of the side's men in its hex, who are fed through base:10 (Landing at (2,-4))."
- Day 16: it forms the supply, "keep base:10 supplied from base:2 ... with a floor of 5 days".
- Day 17: the first run is worked at 110.3 t and "cut to 25 t: the night's limit: no ship carries more than it can land in the night's 1.25 hours". It goes by night because an enemy base, Tulagi, lies one hex away.
- Day 18: "It cannot be met: no ship of the pool is free until day 23."

So two things stand between the rule and the men, neither of them the rule of step 1:

1. **The supply is formed late.** The first men are ashore on day 10; the supply for them is formed on day 16.
2. **A run by night lands 25 t,** and the men ashore eat 0.005 t a man a day: 30 t a day for the 6,076 the lifts bring. The pool of ships gives a run every few days. The supply the staff may send cannot feed the men the same staff has landed.

The Allied side's 0.0346 is hunger of a different kind: its men are landed on day 1 at places it takes that day, and what they lose unfed they lose late in the game at bases of their own once Lunga's stock is spent (none while the place was not theirs). It is inside the mark.

## Condition 4 holds, and what it does not reach

No lift sailed short while units the take could use, in reach within the 7 days of the horizon, would have made up the number: 0 of 2,000 Allied lifts and 0 of 10,000 Japanese lifts.

- The Allied lifts, two a game on day 1, hold no count of the enemy and so want no number.
- Every one of the 10,000 Japanese lifts is short, and none has units in reach that make up the number: a lift carries 150 to 1,200 men (median 504) against 9,508 to 33,802 wanted (median 22,050), on a count by men in contact of 4,754 to 16,901 enemy men. On seed 1, the lift of day 23 carries 900 men against 19,806 wanted, and the four units that could be used within 7 days bring 2,720 more.

The condition asks only about the case where waiting would have brought enough, and that case does not arise in the 39 days: the Japanese side never has the men. What the run shows beside it is that the staff still lifts each few hundred men to a place where it counts itself outnumbered many times over, and they are the men of condition 3.

## Legibility: passes, 6 of 6

`crates/sim-cli/tests/recorded_games/legibility_0159/` holds the key, the six orders as given, the answers and the marks. The six orders were named in the decision before the run and the key was written from scenario 84's tables before any reader ran and before the architect read an order as the tool gives it. Six Haiku readers, each given one order and the four questions, answered; a Sonnet model marked them strictly against the key; the architect read the marks. All 24 answers agree.

What it did not reach: every order is of a built case on placeholder forces, one side's, with one task under it; no order of the 39 days, with its many tasks, was read.

## Reported and not judged (condition 8)

From arrangement (a), by stage 3's reading scripts, beside stage 3's figures:

| | This run | Stage 3 |
| --- | --- | --- |
| The Japanese side attacks the airfield ashore | 0 of 1,000 seeds | 0 of 1,000 |
| The Japanese side bombards it | 1,000 | 1,000 |
| The Allied side takes the airfield and flies from it | 1,000 | 1,000 |
| The Allied side's median margin from its intents; given nothing | 64.73; -59.95 | 64.65; -59.95 |
| The Japanese side's median margin from its intents; given nothing | -64.73; -51.51 | -64.65; -51.51 |
| A staff order rejected, a command refused | 0 | 0 |

The 39 days plays as it played at the end of stage 3. The six steps change what a plan can wait for and what it says; they have not changed how this game goes, because neither side's commander in these arrangements sets a limit of rules C or D, the Allied side holds no count on day 1, and the Japanese side has no day in reach that brings enough.

## What it raises, for the designer

1. **Name the miss and go on to the trial.** The rule of step 1 is built and acts; what defeats it is the size of a night's run against the men landed, and a staff that lands men where it cannot feed or use them. The architect's recommendation, with the two mends below taken into stage 4's plan, where the trial gives the staff the means to weigh a lift before it sails.
2. **The mends, each a decision record with its hand calculation first, neither made here:** the supply of men ashore is formed with the lift that lands them and sized to them, not days after; and a take does not lift men to a place where its own count says they cannot be enough and its own supply cannot feed them: it waits, or lands them elsewhere (the staff cannot choose a beach today).
3. **Not done:** nothing is run again to turn the miss into a pass.
