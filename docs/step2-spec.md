# Flag Plot — Step 2 Spec: Validation Harness

Status: approved by the designer on Nov 12, 2026. Written by the architect; the designer's answers to its five open questions are under "Decided by the designer". It is the source of truth for step 2.

`docs/pillars.md` gives step 2 one line: thousands of automated runs compared with historical outcomes, and the model tuned there, not by feel. This says what that means for the slice as it stands at the end of step 1.

## Scope

In step 2:

- A harness that runs a scenario many times and compares what it measures with the record.
- The record for each battle of step 1, as measures with a source and a width.
- Tuning the figures in `data/reference` and the three constants in `data/derivation/rules.toml`, all of which are placeholders today.
- Changes to rules where no figure can produce the record. Decision 0024 already names three for Savo Island.

- A second night action, Cape Esperance, built to check the surface figures on.

Not in step 2: new subsystems (ground combat, submarines, weather), the staff planner, any change to what a player sees, and battles outside the slice.

## What is compared with what

Three battles to tune on, the ones step 1 built with real ships and aircraft, and one to check on. The record is in `data/oob/engagement_outcomes.csv`; `docs/research/oob/step2-record.md` reads it out and says how firm each part is.

| Battle | Scenario | What it tests |
| --- | --- | --- |
| Savo Island, 8 to 9 August 1942 | `04_savo_island` | Night surface combat, surprise, break-off |
| Eastern Solomons, 24 to 25 August 1942 | `05_eastern_solomons` | Search, the strike decision, air combat, damage to ships |
| Rabaul against Guadalcanal, 21 August to 17 September 1942 | `06_rabaul_guadalcanal` | Range, fatigue, pilots, wear, repair, base supply |
| Cape Esperance, 11 to 12 October 1942 | To be built | The surface figures, on a night they were not tuned on |

Two smaller checks ride with them: ship passages against recorded times, and claims against losses (decision 0056).

## Where the model stands today

From `docs/step2-opening-list.md`, which the build session keeps, and the decisions it cites. These are the misses step 2 starts from:

- Savo Island scores as a minor Allied victory. The night was a decisive Japanese one (decisions 0024 and 0054).
- At the Eastern Solomons, over 1,000 runs on placeholder points, 54 runs in 100 are stalemates. The day is usually judged a limited American victory.
- The losses of the four weeks over Guadalcanal do not have the shape of the record (decision 0042).
- Claims overstate less than the record does: about 2.5 to 3 aircraft were claimed for each lost, and 14 ships for 4 at Savo Island.
- The Japanese search at the Eastern Solomons is a placeholder arc. The real search orders are now in the facts.
- A force in the game holds its ordered speed on a straight course. In the record a ship's speed made good is a half to three quarters of the speed a source states for it: ships zigzagged, waited, slowed for landfall and towed (`docs/research/oob/passages.md`). No rule covers this. The designer has put it aside for now (see "Decided by the designer", 8). So passages are reported and not judged (`data/validation/passages.toml`).

## Measures

A measure is one number taken from a run, with the recorded value beside it. Each is a row in a file, `data/validation/<battle>.toml`, naming the fact rows it rests on. No recorded value is typed in without its rows.

| Battle | Measures |
| --- | --- |
| Savo Island | Ships sunk by side and by type; ships crippled by side; which Allied group was hit; rounds until the Japanese break off |
| Eastern Solomons | Ships sunk and crippled by side; bomb hits on each carrier; aircraft lost by side; whether each side found and struck the other |
| Rabaul against Guadalcanal | Raid days; aircraft lost by side and by cause; aircraft ready at Henderson Field over time; supply used; days the field could not fly |
| All | The result on the objectives scale (decision 0054), beside history's result on that scale. Reported and not judged, until the scale counts a ship that is out of the campaign |
| Passages | Minutes for the recorded runs the slice holds, set against what the rules charge. Reported and not judged: research found no long run at one held speed |
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

The designer accepted these widths as a start. They are looked at again after the first reports.

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
- **Surface figures are tuned on Savo Island and checked on Cape Esperance,** where the other side had the radar and the surprise. A model that gets Savo Island right only because the Japanese always win at night fails there. Cape Esperance is built after Savo Island is tuned and is never tuned on. It needs its ships derived and a scenario, about the work of one of step 1's.

## Tests while figures move

Step 1's scenario tests assert exact numbers from a hand calculation. Those numbers come from the figures step 2 is about to change. Each tuning round would break every one.

Proposed:

1. **The hand-calculation tests run on a frozen copy of the figures** as they stand at the end of step 1 (`data/reference/step1/`), for the length of step 2. Whether to work the calculations again on the tuned figures, all of them or a few chosen to test the arithmetic at tuned sizes, is decided when step 2 closes.
2. **One mechanism in the loader points a scenario at them:** a reference root the caller may override. A scenario file names its reference files by path, so without this each test would need its own copy of the scenario. One test already copies a figures file and rewrites the path in the scenario's text; that is replaced by the loader's mechanism, not repeated.
3. **What such a test then proves is stated:** that the rules compute what the hand calculation says, on known figures. It proves nothing about the figures in use. That is `validate`'s job. Both run in CI.
4. **The run baselines follow the live figures.** The files under `crates/sim-cli/tests/run_baseline` record what each scenario does on the figures in use, so they are remade in the same change as each tuning round, and the change shows the difference.
5. **A rule change still means a new hand calculation,** written by the architect before the code.

## Rule work that comes before tuning

A figure cannot be tuned to cover for a rule that is missing. So the rules that decide which strikes fly and what they hit are built before the air figures are tuned. The harness, the measures files and a first report on today's figures are built meanwhile; that report is the baseline.

| Item | State | Must it land before tuning? |
| --- | --- | --- |
| The state hash at version 2, and batches that record their inputs | Done (decision 0057) | Done |
| Ships charged straight-line miles | Done (decision 0058). Savo Island and the Eastern Solomons are re-derived for it (decisions 0024 and 0031, amended); the hash is at version 3 | Yes, before passages are compared, and before Savo Island is tuned at all: its start is worked back from the hour the Japanese column enters Savo's hex, and that changes with the rule |
| Strike target priority as an order | On the opening list; not yet decided | Yes for the Eastern Solomons: what a strike goes for decides which ships are hit |
| Aircraft that cannot land where they took off: reach set by order, ditching for want of fuel, landing on another deck or at a base when a deck is lost, and the crew's recovery by where the aircraft comes down | Decided (decision 0059, plan approved by the designer on Nov 12, 2026); built in three stages after decision 0058 | Yes for the Eastern Solomons and the air campaign: it decides which strikes fly, and 7 of the 17 US carrier aircraft lost at the Eastern Solomons ditched. When its first two stages are on `main` the two measures of losses to all causes there are judged. A further stage gives a fighter shot down over its own base or force a draw for its pilot; until then the rules lose no pilot of a patrol |
| Experience: one number for an air group and one for a ship's crew | Added to the opening list at the designer's word, Nov 12, 2026; not yet decided. Morale is left out | Yes, before Savo Island is tuned, on the condition under "Experience" below |
| Ship repair | On the opening list; not yet decided | No. It matters to a campaign, not to one battle's losses |
| "Out of the campaign" on the objectives scale | On the opening list; not yet decided | No. Until it lands the result on the scale is reported and not judged |
| Two names for "crippled" | Not started | No, but soon: the glossary means flotation damage of 50 or more, and the objective `ships_crippled` means propulsion or flight deck out |

### Experience

At Savo Island both sides fire by the same rules, and decision 0024 names that as a cause of the wrong result. If training is not a rule, tuning bends the gun and torpedo figures to do its work, and Cape Esperance, where the other side had the better of it, then fails for a reason no figure can mend. So experience is built before the surface figures are tuned.

A number for each side that tuning may move would explain any miss. So:

1. **A unit's experience is derived from fact rows by a written rule,** as a ship's figures are (decision 0020): flying hours, time in commission, night exercises. It is never worked back from a battle's result.
2. **The values are fixed before tuning starts** and are not changed to improve a report.
3. **Only the strength of the effect is tuned:** one figure for each rule that reads experience, the same for both sides.
4. **If research finds too little to rest the values on, experience waits,** and the surface figures stay marked as fitted and not checked.

Values set from training records before tuning either bring Cape Esperance out right or they do not. That is the test.

It is a blunter test than was hoped. At Savo Island the record gives shells fired and hits for both sides, so how well each side shot can be measured directly: 81 to 117 hits for each thousand shells for the Japanese, 21 to 36 for the Allies. At Cape Esperance the shells fired are not on record, and research has looked twice. What is on record there is hits received by each side and ships sunk. Those depend on how long each side fired as well as how well. So the direct measure of experience exists only for the battle the figures are fitted on, and the check on the other battle is by its outcome. That makes conditions 1 and 2 carry more weight: if a unit's value could be moved, nothing would catch it.

`docs/step2-opening-list.md` stays the working list of these. Each becomes a decision record with a hand calculation, as in step 1.

## Done means

- All three battles pass, or each miss is written down with its cause and accepted by the designer.
- No figure in `data/reference` is a placeholder without a note saying what it rests on.
- The frozen-figure tests, the invariants, the determinism scenario and the soak still pass.
- The legibility test is run again on the tuned Eastern Solomons: a cheap model, given one side's briefing and the events, explains an outcome and predicts the next day.
- A cheap model given only the rulebook (`docs/rules/`), the glossary, the data files and two scenario files reproduces a set of hand calculations exactly. The build session ran this once on the step 1 rules; it is run again after the rule changes of step 2.

## Decided by the designer

On Nov 12, 2026, on the architect's recommendation in each case.

1. **Widths.** A measure agrees if the record lies inside the middle 90 percent of the runs and holds at least 1 run in 20. A start, looked at again after the first reports.
2. **A second night action.** Cape Esperance is added as a check on the surface figures, built after Savo Island is tuned and never tuned on.
3. **Frozen figures.** The hand-calculation tests run on the step 1 figures, frozen, for the length of step 2. Working them again on the tuned figures is decided when step 2 closes.
4. **Scoring.** The result on the objectives scale is reported and not judged until the scale counts a ship that is out of the campaign. Tuning is judged on direct measures: ships sunk, hits, aircraft lost.
5. **Order of the rule work.** Strike target priority as an order, and reach beyond the radius, are built before the air figures are tuned.
6. **Ditching.** Aircraft lost for want of fuel or a deck join the rule work. It became one rule with reach beyond the radius, a friendly field and aircraft aloft when their deck is lost: decision 0059.
7. **Experience.** It is in step 2 and is built before Savo Island is tuned, on the condition that its values come from facts and are fixed before tuning; only the strength of its effect is tuned. The designer kept it in step 2 after research found that Cape Esperance can check it only by outcome, not by hits for each shell fired.
8. **Speed made good.** Noted and not in step 2. None of the three battles turns on how fast a ship in transit travels. It is taken up when convoys and supply are compared with the record; the simplest answer may be a cruising speed for routine movement below a ship's stated speed, not a new mechanism.
