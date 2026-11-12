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

## Where the model stands today

From `docs/step2-opening-list.md`, which the build session keeps, and the decisions it cites. These are the misses step 2 starts from:

- Savo Island scores as a minor Allied victory. The night was a decisive Japanese one (decisions 0024 and 0054).
- At the Eastern Solomons, over 1,000 runs on placeholder points, 54 runs in 100 are stalemates. The day is usually judged a limited American victory.
- The losses of the four weeks over Guadalcanal do not have the shape of the record (decision 0042).
- Claims overstate less than the record does: about 2.5 to 3 aircraft were claimed for each lost, and 14 ships for 4 at Savo Island.
- The Japanese search at the Eastern Solomons is a placeholder arc. The real search orders are now in the facts.

## Measures

A measure is one number taken from a run, with the recorded value beside it. Each is a row in a file, `data/validation/<battle>.toml`, naming the fact rows it rests on. No recorded value is typed in without its rows.

| Battle | Measures |
| --- | --- |
| Savo Island | Ships sunk by side and by type; ships crippled by side; which Allied group was hit; rounds until the Japanese break off |
| Eastern Solomons | Ships sunk and crippled by side; bomb hits on each carrier; aircraft lost by side; whether each side found and struck the other |
| Rabaul against Guadalcanal | Raid days; aircraft lost by side and by cause; aircraft ready at Henderson Field over time; supply used; days the field could not fly |
| All three | The result on the objectives scale (decision 0054), beside history's result on that scale |
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

These widths are a proposal. They are question 1 below.

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
- **Surface figures have only Savo Island.** They can be fitted, not checked. A second night action is needed for that; see question 2.

## Tests while figures move

Step 1's scenario tests assert exact numbers from a hand calculation. Those numbers come from the figures step 2 is about to change. Each tuning round would break every one.

Proposed:

1. **The hand-calculation tests run on a frozen copy of the figures** as they stand at the end of step 1 (`data/reference/step1/`).
2. **One mechanism in the loader points a scenario at them:** a reference root the caller may override. A scenario file names its reference files by path, so without this each test would need its own copy of the scenario. One test already copies a figures file and rewrites the path in the scenario's text; that is replaced by the loader's mechanism, not repeated.
3. **What such a test then proves is stated:** that the rules compute what the hand calculation says, on known figures. It proves nothing about the figures in use. That is `validate`'s job. Both run in CI.
4. **The run baselines follow the live figures.** The files under `crates/sim-cli/tests/run_baseline` record what each scenario does on the figures in use, so they are remade in the same change as each tuning round, and the change shows the difference.
5. **A rule change still means a new hand calculation,** written by the architect before the code.

## Rule work that comes before tuning

A figure cannot be tuned to cover for a rule that is missing. These are decided or listed already; the spec only says where each stands against the tuning.

| Item | State | Must it land before tuning? |
| --- | --- | --- |
| The state hash at version 2, and batches that record their inputs | Done (decision 0057) | Done |
| Ships charged straight-line miles | Decided (decision 0058, confirmed by the designer), being built. The hash goes to version 3 with it | Yes, before passages are compared, and before Savo Island is tuned at all: its start is worked back from the hour the Japanese column enters Savo's hex, and that changes with the rule |
| Strike target priority as an order | On the opening list; not yet decided | Yes for the Eastern Solomons: what a strike goes for decides which ships are hit |
| Reach beyond the radius, and a range policy order | On the opening list; not yet decided | Yes for the Eastern Solomons and the air campaign: it decides which strikes fly at all |
| Ship repair | On the opening list; not yet decided | No. It matters to a campaign, not to one battle's losses |
| "Out of the campaign" on the objectives scale | On the opening list; not yet decided | Before the result on the scale is judged. See question 4 |

`docs/step2-opening-list.md` stays the working list of these. Each becomes a decision record with a hand calculation, as in step 1.

## Done means

- All three battles pass, or each miss is written down with its cause and accepted by the designer.
- No figure in `data/reference` is a placeholder without a note saying what it rests on.
- The frozen-figure tests, the invariants, the determinism scenario and the soak still pass.
- The legibility test is run again on the tuned Eastern Solomons.

## Questions for the designer

1. **Widths.** Is "the record inside the middle 90 percent, and not rarer than 1 run in 20" the right test? Narrower bands demand more of the model than one night can support.
2. **A second night action.** Cape Esperance (11 to 12 October 1942) would let the surface figures be checked on a battle they were not fitted to. It is inside the slice but later than anything built so far, some facts for it are already in `data/oob`, and its ships would need deriving. Add it to step 2, or accept fitted surface figures?
3. **Frozen figures for the hand-calculation tests.** Agreed as set out under "Tests while figures move", or should exact numbers follow the live figures and be recalculated each round?
4. **Scoring.** Enterprise at the Eastern Solomons was flying again within the hour and then away for 53 days. Decision 0054's test for "crippled" does not count her. The word also has two meanings today: the glossary's "crippled" is flotation damage of 50 or more, while the objective `ships_crippled` counts a ship not sunk, in port or at sea, with propulsion or flight deck out. The opening list proposes points for a ship that must leave for repair. Should that be settled before the result on the scale is used as a measure, or should the scale be left out of the measures until it is?
5. **Order of the rule work.** The table under "Rule work that comes before tuning" puts strike target priority and reach beyond the radius ahead of tuning the air figures. Agreed, or tune first on the rules as they are and see how far the figures get?
