# The marks

Marked on Nov 28, 2026 by a second model (Sonnet, `claude-sonnet-5-5`, no tools), given `key.md`, the six readers' answers and the instruction below, and nothing else. Run once. The architect's reading of the marks follows them.

## The marker's instruction

The setup fixed the reader's prompt before the run and said the marker is "held to the key"; it did not fix the marker's words. These are the architect's, written on the day of the run before any answer was seen, after the manner of decision 0159's proof ("told to mark strictly"):

```
You are marking a reading test, strictly. Below is a key and, after it, six readers' answers, one reader for each estimate. Each reader answered five questions about one estimate: 1 what the operation needs, 2 when it could go, 3 in how many of the worlds tried it came off, 4 what it may cost, 5 what was assumed. Hold every answer to the key and to nothing else. An answer agrees only if it says what the key says for that question of that estimate, with no figure wrong and nothing the key requires left out; an answer that adds something the key contradicts disagrees. For each estimate give a line for each of the five questions: 'agrees' or 'disagrees', with a short reason quoting the figure. Then say for each estimate whether all five agree, and end with the count of estimates that pass out of six.

THE KEY:
<the key>

THE ANSWERS:
<the six readers' answers>
```

## The marker's marks, as given

## Estimate 1
1. **Needs:** agrees. All nine tasks are covered, including the carrier forces operated from (7,-5) and the three bases neutralised.
2. **When it could go:** disagrees. The key gives first day 1 and last day 14. The answer says only that no landing day is given and never says the order runs from day 1 to day 14.
3. **Worlds:** agrees. None of 16.
4. **Cost:** agrees. Men lost 500 / 1,680 / 2,120, ships sunk 0 to 1, ships out of action 0 to 7, aircraft 15 / 22 / 26.
5. **Assumed:** agrees. The four bases at 1,000 t and 500 t, 5,275 and 1,850 men, the 6-ship cruiser force, the 8-ship carrier force in 8 of 16 worlds, no belief, and the hold intents are all present.

Not all five agree. **Fails.**

## Estimate 2
1. **Needs:** agrees. All six tasks are present.
2. **When it could go:** agrees. From day 1, met on day 1.
3. **Worlds:** agrees. All 16.
4. **Cost:** agrees. 79 unfed against 26 without the order, aircraft 0 / 2 / 5.
5. **Assumed:** agrees.

All five agree. **Passes.**

## Estimate 3
1. **Needs:** disagrees. The answer leaves out that a count of the enemy at (4,0) is asked for and that the 200 held, told on day 0, does not meet it.
2. **When it could go:** agrees. Day 4 expected ashore, met on days 5 to 7 with 6 as the middle.
3. **Worlds:** agrees. 9 of 16.
4. **Cost:** agrees. 20 / 60 / 100, everything else 0.
5. **Assumed:** agrees.

Not all five agree. **Fails.**

## Estimate 4
1. **Needs:** disagrees. It also leaves out the count that is asked for and not met. It says "needs 400 men at (4,0)", which misplaces the men wanted.
2. **When it could go:** agrees. Met on day 4 in every world.
3. **Worlds:** agrees. 16 of 16.
4. **Cost:** agrees. 100 in every world.
5. **Assumed:** agrees.

Not all five agree. **Fails.**

## Estimate 5
1. **Needs:** agrees. The landing by the transport force, and a count asked for with none held.
2. **When it could go:** agrees. Dates day 1 and day 4, with ashore days 4 and 8.
3. **Worlds:** agrees. 4 of 8 from day 1, 8 of 8 from day 4.
4. **Cost:** agrees. 100 / 100 / 200.
5. **Assumed:** disagrees. It calls the extra 200 men "believed present". The key says "believed, standing unknown" for that group, and only the first 200 are "believed present".

Not all five agree. **Fails.**

## Estimate 6
1. **Needs:** agrees. The landing, expected ashore day 4, and a count asked for and not held.
2. **When it could go:** agrees. From day 1 over five days, met on day 4.
3. **Worlds:** agrees. 4 of 8, with worlds 1, 3, 5 and 7.
4. **Cost:** agrees. 100 to 200, middle 100.
5. **Assumed:** agrees. The second 200 men in worlds 2, 4, 6 and 8 contradicts nothing in the key.

All five agree. **Passes.**

## Result
**2 of 6 estimates pass** (estimates 2 and 6).

## The architect's reading

The mark is six of six. **Two of six pass: the condition is missed.** Nothing was asked again.

| Estimate | Needs | When | Worlds | Cost | Assumed | Passes |
| --- | --- | --- | --- | --- | --- | --- |
| 1. (a), Allied | agrees | disagrees | agrees | agrees | agrees | no |
| 2. (a), Japanese | agrees | agrees | agrees | agrees | agrees | yes |
| 3. S1 | disagrees | agrees | agrees | agrees | agrees | no |
| 4. W1 | disagrees | agrees | agrees | agrees | agrees | no |
| 5. A window | agrees | agrees | agrees | agrees | disagrees | no |
| 6. A trial beside another | agrees | agrees | agrees | agrees | agrees | yes |

Of the 30 answers 26 agree. The four that do not, each read against the estimate and the key:

- **Estimate 1, when it could go.** The estimate says it was tried "as it would go from the morning of day 1" for 14 days, and that "no day ashore can yet be given". The reader gave the second and left out the first. The mark stands. The question fits this estimate badly: a trial that is no window says when it was tried from, not when the operation could go, and for an island it gives no day ashore.
- **Estimates 3 and 4, what it needs.** Each estimate says in its words "a count of the enemy at (4,0) is asked for, and the one held is 200 men, told on day 0, which does not meet it". Both readers gave the men and the task and left the count out. The marks stand. Reader 4's "needs 400 men at (4,0)" is loose and not wrong: 400 are the men wanted.
- **Estimate 5, what was assumed.** The estimate gives the second 200 men as "believed, standing unknown"; the reader wrote "believed present in only 4 worlds". The number of men and of worlds is right and the standing is not. It is the thinnest of the four, and the marker passed reader 6, who gave no standing for the same men. The mark is left as given: a strict marker was asked for, and "present" is a standing the estimate did not give.

So three estimates fail by a thing left out and one by a word. Two of the four marks are arguable: estimate 5's, as said above, and estimate 1's, since "it waits for nothing" is a true answer to "when could it go" and the marker passed readers 2, 3, 4 and 6, none of whom gave the last day. A fair range for the count is between two and four of six, and the condition is missed anywhere in that range.

The files `estimate1.json` to `estimate6.json` and `key.md` are as `run.sh harness` wrote them at b9e488a (the test `legibility_estimates`), copied here before any reader ran. They carry agent interface 57.
