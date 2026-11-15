# Legibility of the staff's reasons (decision 0107, ruling 39)

Run once, on Nov 15, 2026, by the team lead of step 4 of stage 1. It is not a test CI runs, and this folder holds no game (`regame.py` and the `mcp` tests skip a folder with no `game.jsonl`).

- `pick.py`: the rule that chose the 24 reasons, written before the pick (its docstring), run on the exports of scenarios 36, 40a, 41a and 42b on the frozen figures (`sim-cli export ... --reference data/reference/step1`).
- `reasons.json`: the 24 reasons with their five parts, and `prompt_text`, the only thing each model was given.
- `key.md`: the key, a draft by the team lead written before any model ran, for the architect to read (the ruling makes the key the architect's).
- `answers.md`: each Haiku answer verbatim, one agent per reason, given the reason alone and the two questions.
- `marks.md`: a second model's marks against the key.

Result as marked: 20 of 24 pass both questions; every order, no-order, displaced and hold reason passes; the four leave reasons fail question 1 only, because those reasons name no base. By the ruling's second part (no kind under half) the pass mark is not met. The architect read the key and every fail on Nov 15, 2026: the result stands as marked, not met. The fault it found is in the reasons, whose "serves" part gives an intent's number and not what the intent is (decision 0107, "As built (step 4)").

## The second run (Nov 15, 2026)

The designer decided on Nov 15, 2026 to mend the reasons and run the four leave reasons once more, keeping both results (decision 0107, "As built (step 4)", "The mend and the second run"). Every reason's serves part now gives the intent's statement in words, and a reason whose worked part speaks of a plan with no base names the plan with what it is for. The first run's files above are untouched.

- `second_run.py`: takes `pick.py`'s rule, run unchanged on the mended build's exports of the same four scenarios on the frozen figures, and keeps its reasons 21 to 24 (the same scenarios, days and events as the first run); it adds each reason's serves line exactly as the log words it and writes the text the model is given, in the first run's layout.
- `reasons_second_run.json`: the four, with their five parts and `prompt_text`.
- `key.md`, section "Added for the second run": the base and aim each of the four now states, written before any model ran. The key's requirement is unchanged.
- `answers_second_run.md`: one Haiku agent per reason, given the reason alone and the first run's prompt word for word (only the file's path differs).
- `marks_second_run.md`: a Sonnet agent's marks against the key, as in the first run.

| Kind | First run (pass both) | Second run (pass both) |
| --- | --- | --- |
| order | 6 of 6 | not run again |
| no order | 6 of 6 | not run again |
| displaced | 4 of 4 | not run again |
| hold | 4 of 4 | not run again |
| leave | 0 of 4 (question 1 failed: no base named) | 4 of 4 |
| all | 20 of 24, pass mark not met | 24 of 24 with the second run's four in place of the first's |

With the four second-run marks in place of the four first-run ones, the test comes to 24 of 24 passing both questions and no kind under half, which meets ruling 39's pass mark. That whole rests on two runs: the twenty other reasons were run once, on the reasons before the mend, and the four leave reasons on the mended ones. It was run once more and only once; the other twenty were not rerun.

Read by the architect on Nov 15, 2026: the four second-run answers and their marks stand. The test as a whole is 24 of 24 on two runs, twenty reasons before the mend and four after it; the first run's result, not met, stays on record above.
