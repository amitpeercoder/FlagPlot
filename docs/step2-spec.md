# Flag Plot — Step 2 Spec: Validation Harness

Status: draft by the architect, Nov 12, 2026. Not approved. The questions at the end are the designer's to answer before any of it is built.

`docs/pillars.md` gives step 2 one line: thousands of automated runs compared with historical outcomes, and the model tuned there, not by feel. This says what that means for the slice as it stands at the end of step 1.

## Scope

In step 2:

- A harness that runs a scenario many times and compares what it measures with the record.
- The record for each battle of step 1, as measures with a source and a width.
- Tuning the figures in `data/reference` and the three constants in `data/derivation/rules.toml`, all of which are placeholders today.
- Changes to rules where no figure can produce the record. Decision 0024 already names three for Savo Island.

Not in step 2: new subsystems (ground combat, submarines, weather), the staff planner, any change to what a player sees, and battles outside the slice.

## What is compared with what

Three battles, the ones step 1 built with real ships and aircraft. The record is in `data/oob/engagement_outcomes.csv`; `docs/research/oob/step2-record.md` reads it out and says how firm each part is.

| Battle | Scenario | What it tests |
| --- | --- | --- |
| Savo Island, 8 to 9 August 1942 | `04_savo_island` | Night surface combat, surprise, break-off |
| Eastern Solomons, 24 to 25 August 1942 | `05_eastern_solomons` | Search, the strike decision, air combat, damage to ships |
| Rabaul against Guadalcanal, 21 August to 17 September 1942 | `06_rabaul_guadalcanal` | Range, fatigue, pilots, wear, repair, base supply |

Two smaller checks ride with them: ship passages against recorded times, and claims against losses (decision 0056).

## Measures

A measure is one number taken from a run, with the recorded value beside it. Each is a row in a file, `data/validation/<battle>.toml`, naming the fact rows it rests on. No recorded value is typed in without its rows.

| Battle | Measures |
| --- | --- |
| Savo Island | Ships sunk by side and by type; ships crippled by side; which Allied group was hit; rounds until the Japanese break off |
| Eastern Solomons | Ships sunk and crippled by side; bomb hits on each carrier; aircraft lost by side; whether each side found and struck the other |
| Rabaul against Guadalcanal | Raid days; aircraft lost by side and by cause; aircraft ready at Henderson Field over time; supply used; days the field could not fly |
| Passages | Days at sea for the recorded runs the slice holds |
| Claims | Claimed over actual, for aircraft and for ships, by side |

The record is uneven and the measures must say so. From the research note:

- **Firm:** ships sunk at Savo Island (four Allied heavy cruisers, no Japanese ship) and at the Eastern Solomons.
- **Disputed:** Japanese aircraft lost at the Eastern Solomons. Sources give about 40, 62 and 75.
- **Thin:** aircraft destroyed on the ground, days Henderson Field could not fly, and any Japanese supply tonnage. Almost nothing was found.

A measure on a disputed figure takes a range for the record, not a point. A measure on a thin figure is reported and not judged.

## What counts as agreement

One night happened once. A model that gives the recorded outcome every time is as wrong as one that never gives it.

1. **A measure agrees if the record lies inside the middle 90 percent of the runs.** That is, between the 5th and 95th percentile over 1,000 seeds.
2. **And if the record is not a freak in the model.** The recorded value, or the range for it, must hold at least 1 run in 20.
3. **Shape checks are stated per battle, as yes or no.** For example: at Savo Island the Allied side loses more ships than the Japanese in at least 8 runs of 10.
4. **A battle passes when every judged measure agrees.** A miss is reported with the measure, the record, and the runs' spread.

These widths are a proposal. They are question 2 below.

## The harness

- **`sim-cli batch` records more for each run:** aircraft lost by side and cause, hits by ship, sorties flown, supply used, and the round a force broke off. Today it records ships sunk and the result only.
- **A new command, `sim-cli validate`,** reads a batch file and a measures file and writes a report: each measure, the record, the spread, and agree or not. It refuses a batch whose `inputs` do not match the data files on disk (decision 0057).
- **Every batch is labelled** with the revision that made it.
- **Reports are kept** under `docs/validation/`, one per battle per tuning round, so the history of the tuning can be read.

The harness is an umpire's tool, like the viewer. It reads truth. Nothing that plays the game uses it.

## Tuning

1. **One set of figures serves every battle.** No figure is set per scenario. If Savo Island and the Eastern Solomons want different values of one figure, the rule is wrong, not the figure.
2. **One mechanism at a time, in the order the rules run:** finding and surprise, then the chance of a hit, then what a hit does, then when a force breaks off.
3. **Every round is recorded:** what was changed, why, and the report before and after.
4. **A figure is moved only toward something a source supports.** `docs/research/oob` has notes on gunnery, torpedoes, anti-aircraft fire and search. A figure with no support stays marked as a placeholder, however well it fits.
5. **A rule change is a decision record with a rule card,** as now, and the designer confirms any that changes how the game plays.

Tuned on one battle and checked on the same battle proves little. So:

- **Air figures are tuned on the Eastern Solomons and checked on Rabaul against Guadalcanal,** which they were not tuned on.
- **Surface figures have only Savo Island.** They can be fitted, not checked. A second night action is needed for that; see question 3.

## Tests while figures move

Step 1's scenario tests assert exact numbers from a hand calculation. Those numbers come from the figures step 2 is about to change. Each tuning round would break every one.

Proposed: the hand-calculation tests run on a frozen copy of the figures as they stand at the end of step 1 (`data/reference/step1/`). They go on proving that the rules compute what they should. The live figures are judged by the bands above. A rule change still means a new hand calculation, written by the architect before the code.

## Before step 2 starts

- **Done:** the state hash at version 2, and batch files that record their inputs (decision 0057).
- **Not done: ships charged straight-line miles.** A force steaming between two axes of the grid covers up to 15.5 percent less ground than its speed says (decision 0040). Passages cannot be compared with the record until that is settled. It changes how a force moves, so it needs its own decision and hand calculation. It is question 1.

## Done means

- All three battles pass, or each miss is written down with its cause and accepted by the designer.
- No figure in `data/reference` is a placeholder without a note saying what it rests on.
- The frozen-figure tests, the invariants, the determinism scenario and the soak still pass.
- The legibility test is run again on the tuned Eastern Solomons.

## Questions for the designer

1. **Ship distances.** The simplest rule I can see: find the path as now, split it at the hexes where it has to turn to clear land, and charge each straight stretch its straight-line miles, spread evenly over its steps. Do you want that, or a different treatment?
2. **Widths.** Is "the record inside the middle 90 percent, and not rarer than 1 run in 20" the right test? Narrower bands demand more of the model than one night can support.
3. **A second night action.** Cape Esperance (11 to 12 October 1942) would let the surface figures be checked on a battle they were not fitted to. It is inside the slice but later than anything built so far, some facts for it are already in `data/oob`, and its ships would need deriving. Add it to step 2, or accept fitted surface figures?
4. **Frozen figures for the hand-calculation tests.** Agreed, or should exact numbers follow the live figures and be recalculated each round?
5. **Scoring.** Enterprise at the Eastern Solomons was flying again within the hour and then away for 53 days. Decision 0054's test for "crippled" does not count her. Should step 2 change the test, or leave scoring alone?
