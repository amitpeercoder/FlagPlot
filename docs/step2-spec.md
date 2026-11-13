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

Three battles to tune on, the ones step 1 built with real ships and aircraft, and one to check on. Rabaul against Guadalcanal is both: the figures of air combat are checked on it and the figures that take weeks to show are tuned on it ("Tuning"). The record is in `data/oob/engagement_outcomes.csv`; `docs/research/oob/step2-record.md` reads it out and says how firm each part is.

| Battle | Scenario | What it tests |
| --- | --- | --- |
| Savo Island, 8 to 9 August 1942 | `04_savo_island` | Night surface combat, surprise, break-off |
| Eastern Solomons, 24 to 25 August 1942 | `05_eastern_solomons` | Search, the strike decision, air combat, damage to ships |
| Rabaul against Guadalcanal, 21 August to 17 September 1942 | `06_rabaul_guadalcanal` | Tuned on it: fatigue, pilots and replacements, wear, repair, base supply. Checked on it: the air combat figures tuned on the Eastern Solomons |
| Cape Esperance, 11 to 12 October 1942 | To be built | The surface figures, on a night they were not tuned on |

Two smaller checks ride with them: ship passages against recorded times, and claims against losses (decision 0056).

## Where the model stands today

From `docs/step2-opening-list.md`, which the build session keeps, and the decisions it cites. These are the misses step 2 starts from:

- Savo Island scores as a minor Allied victory. The night was a decisive Japanese one (decisions 0024 and 0054).
- At the Eastern Solomons, over 1,000 runs on placeholder points, 54 runs in 100 are stalemates. The day is usually judged a limited American victory.
- The losses of the four weeks over Guadalcanal do not have the shape of the record (decision 0042).
- Claims overstate less than the record does: about 2.5 to 3 aircraft were claimed for each lost, and 14 ships for 4 at Savo Island.
- The Japanese search at the Eastern Solomons is a placeholder arc. The real search orders are now in the facts.
- A ship in the game fires its main battery and nothing else. A cruiser's secondary guns have no counterpart in the rules: at Savo Island they fired about a third of the Japanese shells (654 of 1,867). So the measures of shells fired count the main battery only, and the measures of gun hits, which the record cannot split by calibre, will run low by whatever the secondary guns scored. No rule is planned for this in step 2; it is named so that tuning is not asked to make the main battery do the secondary guns' work.
- A force in the game holds its ordered speed on a straight course. In the record a ship's speed made good is below the speed a source states for it: a half to three quarters for convoys and carrier forces, eight or nine tenths for a destroyer squadron on passage. Ships zigzagged, waited, slowed for landfall and towed (`docs/research/oob/passages.md`). No rule covers this. The designer has put it aside for now (see "Decided by the designer", 8). So passages are reported and not judged (`data/validation/passages.toml`).

## Measures

A measure is one number taken from a run, with the recorded value beside it. Each is a row in a file, `data/validation/<battle>.toml`, naming the fact rows it rests on. No recorded value is typed in without its rows.

| Battle | Measures |
| --- | --- |
| Savo Island | Ships sunk by side and by type, and by Allied group; Japanese ships left with propulsion out; hits by side and by weapon; hits for each thousand shells; minutes of fighting, for when a force breaks off |
| Eastern Solomons | Carriers sunk by side; bomb hits on each carrier; whether Enterprise's deck is out; aircraft lost by side, to the enemy and to all causes; the aircraft in each side's strike |
| Rabaul against Guadalcanal | Raid days and bomber sorties; aircraft lost by side and by cause; aircraft ready at Henderson Field over time; pilots lost. Supply used and the days the field could not fly have no measure: no row gives a figure for either |
| All | The result on the objectives scale (decision 0054): the report gives the tally of outcomes and the Allied margin over the runs. Reported and not judged, until the scale counts a ship that is out of the campaign |
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
3. **Shape checks are stated per battle, as yes or no.** For example: at Savo Island the Allied side loses more ships than the Japanese in at least 8 runs of 10. None is stated yet, and the measures files have no form for one. `validate` judges only what the files state; a form is added with the first shape check.
4. **A battle passes when every judged measure agrees.** A miss is reported with the measure, the record, and the runs' spread.

The designer accepted these widths as a start. They are looked at again after the first reports.

The two tests, exactly, as the architect of the spec session settled them on Nov 12, 2026:

- **The runs kept.** A measure's value is taken from each run. A measure that divides (claims for each hundred, hits for each thousand shells) leaves out a run in which the divisor is nothing. The runs left are the runs kept, and both tests are worked on them.
- **A percentile is a value that a run gave,** never an average of two. The kept runs' values are sorted, and the percentile p is the value at place ceil(p × n / 100), counting from 1, of n. Of 1,000 runs the 5th is the 50th value and the 95th the 950th.
- **Inside the middle 90 percent** means from the 5th percentile to the 95th, both ends included. A record that is a range is inside if any part of it is: the range spans a dispute, the true figure is somewhere in it, and a model is not at fault for the width of a dispute.
- **A run holds the record** if its value is from `record_low` to `record_high`, both ends included. At least 1 run in 20 means the runs that hold it, times 20, are no fewer than the runs kept.
- **Too few runs kept.** If fewer than 1 run in 5 is kept, a judged measure misses, and the report says that is why. A spread over a handful of runs says nothing. If none is kept the measure has no value and misses for the same reason.
- **An agreement at the edge is marked.** A measure that agrees, with no part of its record from the 25th percentile to the 75th, is reported as "agrees, at the edge". It still agrees: one night happened once, and the record may be an unusual night. But the mark keeps a count of agreements from being read as health. The baseline showed why: at Savo Island gun hits on Japanese ships agreed with a record of 10 to 17 against a middle run of 127, because the record touched the bottom of a spread of 8 to 224. Whether an agreement at the edge counts against a battle when step 2 closes is the designer's to decide, after the first tuning rounds show how many are left.
- **A battle passes only on 1,000 runs or more.** `validate` reports on fewer, and says in plain words how many runs it had and that 1,000 are asked for; such a report gives each measure's verdict and no verdict on the battle.

## The harness

- **`sim-cli batch` records more for each run** (decision 0062): aircraft lost by group and cause, hits by ship, shots and hits by side, attacks and escorts, aircraft ready by day, claims, supply used, the round a force broke off and the rounds each fight ran.
- **A new command, `sim-cli validate`,** reads a batch file and a measures file and writes a report: each measure, the record, the spread, and agree or not. It refuses a batch whose `inputs` do not match the data files on disk (decision 0057), and one whose hash layout version is not the engine's.
- **Every batch is labelled with its tuning round:** `baseline` for the first, then a name for each round. The label is letters, digits, dot, dash and underscore. The hashes in `inputs` say which figures the round ran on; the label says which round it was. `batch` writes a file without a label; `validate` refuses it.
- **Reports are kept** under `docs/validation/`, one per battle per tuning round, as `<battle>-<label>.md`, so the history of the tuning can be read. `validate` does not write over a report of the same name unless what it would write is the same, byte for byte.
- **What a report holds.** Its head names the batch's label, runs and seeds, every hash in `inputs`, and the measures file with its hash. It carries no date: the same inputs give the same file, and git dates it. Then each measure in the file's order: its id, the record, the 5th to the 95th percentile, the median, how many of the runs kept hold the record, how many runs were left out, and the verdict: agrees, or agrees at the edge, or a miss naming the test that failed, or reported, or not run. The head counts the three apart. A miss also gives what the measure counts, its rows, and the spread as least, 5th, median, 95th and greatest, with a count of each value where there are 20 or fewer. A measure that is not judged shows the same numbers and no verdict. Last, the tally of outcomes and the Allied margin on the objectives scale, reported.
- **A measures file whose scenario is not built** is given to `validate` with no batch, and the report lists each measure and its record as not run.
- **A miss is not a failure of the command.** `validate` ends with success when it has written its report, whatever the verdicts. Only a refusal ends with an error.

The harness is an umpire's tool, like the viewer. It reads truth. Nothing that plays the game uses it.

## Tuning

1. **One set of figures serves every battle.** No figure is set per scenario. If Savo Island and the Eastern Solomons want different values of one figure, the rule is wrong, not the figure.
2. **One mechanism at a time, in the order the rules run:** finding and surprise, then the chance of a hit, then what a hit does, then when a force breaks off.
3. **Every round is recorded:** what was changed, why, and the report before and after.
4. **A figure is moved only toward something a source supports.** `docs/research/oob` has notes on gunnery, torpedoes, anti-aircraft fire and search. A figure with no support stays marked as a placeholder, however well it fits.
5. **A rule change is a decision record with a rule card,** as now, and the designer confirms any that changes how the game plays.
6. **What is derived is checked before what is tuned.** A ship's rate of fire, its guns and its speed are derived from fact rows, and the tuned figures multiply them. Before a mechanism's figures are tuned, the derived quantities it rests on are set against a measure that no tuned figure touches. For gunnery that measure is the shells Tenryu and Yubari fire at Savo Island: they were at battle readiness, so no rule of readiness or experience reads their count, and at the baseline they fired several times what they did. If a derived quantity is wrong the derivation is mended, by its written rule and from rows, not the figure.
7. **A figure that only one side reads never covers for something both sides share.** The share of guns a force not at battle readiness fires with (decision 0068) starts at one half, which a row states. It may go lower only for what belongs to it, the confusion of a force caught unready. It is not taken down to make up for rates of fire that are too high for both navies, or for fights the record does not have.

Tuned on one battle and checked on the same battle proves little. So:

- **The figures of air combat are tuned on the Eastern Solomons and checked on Rabaul against Guadalcanal,** which they were not tuned on: search, the fight between aircraft, anti-aircraft fire, the attack on a ship or a field, and claims. A tuning round does not move one of these to mend a miss at Rabaul. A miss there goes back to the Eastern Solomons, or to the rule.
- **The figures that take weeks to show are tuned on Rabaul against Guadalcanal,** because nothing else in the slice can show them: fatigue, wear and repair, pilots and replacements, base supply. The Eastern Solomons is one day. **These figures have no battle to be checked on, and step 2 says so of each:** tuned on one battle and not checked. A second long scenario would check them; none is planned in step 2.
- **Each tuning round's record names which kind it moved.** That is how a reader of a Rabaul report knows what it shows.
- **Surface figures are tuned on Savo Island and checked on Cape Esperance,** where the other side had the radar and the surprise. A model that gets Savo Island right only because the Japanese always win at night fails there. Cape Esperance is built after Savo Island is tuned and is never tuned on. It needs its ships derived and a scenario, about the work of one of step 1's.

## Tests while figures move

Step 1's scenario tests assert exact numbers from a hand calculation. Those numbers come from the figures step 2 is about to change. Each tuning round would break every one.

Proposed:

1. **The hand-calculation tests run on a frozen copy of the figures** as they stand at the end of step 1 (`data/reference/step1/`), for the length of step 2. Whether to work the calculations again on the tuned figures, all of them or a few chosen to test the arithmetic at tuned sizes, is decided when step 2 closes.
2. **One mechanism in the loader points a scenario at them:** a reference root the caller may override. A scenario file names its reference files by path, so without this each test would need its own copy of the scenario. One test already copies a figures file and rewrites the path in the scenario's text; that is replaced by the loader's mechanism, not repeated.
3. **What such a test then proves is stated:** that the rules compute what the hand calculation says, on known figures. It proves nothing about the figures in use. That is `validate`'s job. Both run in CI: for `validate` that means its own tests, on a small batch made by hand whose percentiles and verdicts are worked out in the test's doc comment. No batch of 1,000 runs is run in CI.
4. **The run baselines follow the live figures.** The files under `crates/sim-cli/tests/run_baseline` record what each scenario does on the figures in use, so they are remade in the same change as each tuning round, and the change shows the difference.
5. **A rule change still means a new hand calculation,** written by the architect before the code.
6. **A rule added during step 2 adds its figures to the frozen copy too, at the value that leaves step 1's results as they were,** where there is such a value, and otherwise at its first value. The frozen copy stands for what step 1 did, so a rule step 1 did not have is switched off there wherever it can be. The pilot of a patrol shot down over its own base is always recovered in the frozen copy, as in step 1, and draws for it in the live figures. The strength of experience is frozen at the value that makes experience do nothing. So no hand calculation of step 1 moves when a rule is added.
7. **The new rule is then proved in a scenario of its own,** on values that make it act. Switched off in the frozen copy, it is tested nowhere else by hand, so this is not optional. The scenario's test has three parts:
   - **The chance, hand-calculated at a working value** strictly between 0 and 1, and asserted from the event's breakdown. A chance of 0 or 1 shows that the rule switches on and off and exercises no draw.
   - **A property over many seeds** at that value: the count drawn is close to what the chance expects.
   - **A pinned outcome for one seed** at that value, labelled as pinned and not as calculated. No hand calculation can say which draw succeeds; a pinned number catches a stream that moved, which the property would miss. It is remade when the rule is changed on purpose, with the change, and never to make a test pass.
8. **One scenario has every rule of step 2 acting together,** built when the last of them is in and before any figure is tuned. With each new rule switched off in the frozen copy, no other hand calculation has two of them acting at once. It is a cut-down Eastern Solomons that can be read at one sitting: one carrier a side, a patrol, a strike beyond the radius, a deck that goes out with aircraft away, pilots counted, experience unequal. Its hand calculation covers everything that is determined: who is within which reach, each chance at its working value, each expected count, and the order the rules run in. It carries pinned outcomes for its seed. The rule cards of the step's decisions are read together on it, which makes it the complexity review of step 2.
9. **A scenario whose orders are set from the record keeps a frozen copy for its hand calculations,** as the figures do. Setting a battle's orders from fact rows is not a rule change, and it would move every hand calculation of that scenario. So the hand-calculation tests of such a scenario run on a copy of it as it stood, and the live scenario is covered by the run baselines and by `validate`. There is one frozen copy for a scenario and no more; it changes only with a rule change's new hand calculation; and the test says in its doc comment that it runs on the frozen copy. Whether the hand calculations are worked again on the live scenarios is decided with the frozen figures, when step 2 closes.

## Rule work that comes before tuning

A figure cannot be tuned to cover for a rule that is missing. So the rules that decide which strikes fly and what they hit are built before the air figures are tuned. The harness, the measures files and a first report on today's figures are built meanwhile; that report is the baseline.

| Item | State | Must it land before tuning? |
| --- | --- | --- |
| The state hash at version 2, and batches that record their inputs | Done (decision 0057) | Done |
| Ships charged straight-line miles | Done (decision 0058). Savo Island and the Eastern Solomons are re-derived for it (decisions 0024 and 0031, amended); the hash is at version 3 | Yes, before passages are compared, and before Savo Island is tuned at all: its start is worked back from the hour the Japanese column enters Savo's hex, and that changes with the rule |
| Strike target priority as an order | Built (decision 0063, stage 1). The three strike measures at the Eastern Solomons are judged. The scenario's orders are being set from the record | Yes for the Eastern Solomons: what a strike goes for decides which ships are hit |
| Aircraft that cannot land where they took off: reach set by order, ditching for want of fuel, landing on another deck or at a base when a deck is lost, and the crew's recovery by where the aircraft comes down | Built in all its stages (decision 0059) | Yes for the Eastern Solomons and the air campaign: it decides which strikes fly, and 7 of the 17 US carrier aircraft lost at the Eastern Solomons ditched. Its first two stages are on `main`, so the two measures of losses to all causes there are judged; the Japanese one leaves out the five dive bombers lost coming home in the dark, which the rule does not cover. A further stage gives a fighter shot down over its own base or force a draw for its pilot; until then the rules lose no pilot of a patrol |
| Experience: one number for an air group and one for a ship's crew | Built for ships (decision 0064, stage 1): the values are derived and fixed, and at night the gun, torpedo and sighting chances read them. For air groups (stage 2) it waits, by the fourth condition below: worked by hand from the rows, the Japanese carrier groups' value rests on one statement of hours for the whole service in 1941, every US Navy group sits at the middle only because no row exists, and the Marine squadrons' two questions are one fact counted twice. Research is commissioned; stage 2 is not built in step 2 unless the rows arrive. Morale is left out | Yes, before Savo Island is tuned, on the condition under "Experience" below |
| A force that is not at battle readiness when a fight begins fires with part of its guns | Built (decision 0068). Ready is battle readiness by order, or alert since before the pulse began; a force alerted at the start of the pulse of its fight is not ready in it. The share is of guns only and holds in any fight, by day too. Shots are recorded by the ship that fired, and Savo Island has measures of each Allied group's shells. The record showed a share and not a wait: the Allied cruisers fired a small part of a broadside for the whole fight | Yes, before Savo Island is tuned. At the baseline the Allied groups fire about 2,900 shells a run where the record has 471, because surprise stops a force firing for one round and then both sides fire alike. Experience at its cap does not close the gap. The rule must serve Cape Esperance, where the other side was surprised, on the same figures |
| Rates of fire as achieved, not as designed | Found by the build session after decision 0068 was built. A round is five minutes at a class's designed rate, and one round at half manning already exceeds what any ship of the southern group fired in the whole action. The few rows on achieved rates put them at half the designed rate or less for both navies, at low confidence; more are to be commissioned. It is a change to the derivation, by its written rule, and the build session's | Yes, before the surface figures are tuned ("Tuning", 6) |
| Surface fights by day with ships that fell behind | Seen in the baseline: in about half the runs of Savo Island, Japanese ships left behind by the stragglers rule are caught after dawn, where the record has no such fight. The designer agreed with decision 0068 that these are looked at after that rule and experience are in | Yes, before the surface figures are tuned: by day the Allied groups are ready and fire in full, so these fights carry much of the shells that are left |
| The Japanese search at the Eastern Solomons set from the record, and search ranges as flown | Written as decision 0073 and held back. With the searches set from the rows, the American carriers are sighted in fewer runs and the Japanese strike does not fly in the middle run. The cause under it is in the derivation: the derived search ranges are below the distances flown (175 miles against 250 for the carrier attack plane, 253 against 330 for the cruiser floatplane; the sighting itself was at about 260). The ranges are looked at first, by "Tuning", 6, so that the report is remade once | Yes, before the air figures are tuned: who is found decides which strikes fly |
| Ship repair | On the opening list; not yet decided | No. It matters to a campaign, not to one battle's losses |
| "Out of the campaign" on the objectives scale | On the opening list; not yet decided | No. Until it lands the result on the scale is reported and not judged |
| Two objectives that can be gamed, found by the rulebook review on Nov 12, 2026. `cargo_landed` counts every ton unloaded, so cargo unloaded, loaded back at the same base and unloaded again scores each time. `base_open` reads only the last moment of the game, so a base shut for four weeks scores as open with one ton and one ready aircraft at the end | Decided (decision 0067, plan approved by the designer on Nov 12, 2026): cargo landed is tons unloaded less tons loaded back at that base, and base open is scored by the share of days. Built by the rulebook review session (pull request 155): the engine records every base at the end of each day. The state hash of every scenario with a base moved, since the hash counts the log's events; its version did not | No. No measure rests on either, and the result on the scale is reported and not judged. The tallies of outcomes for `06_rabaul_guadalcanal` and `07_full_slice` in the baseline reports were made before it and are remade with the next round of reports |
| Two names for "crippled" | Not started | No, but soon: the glossary means flotation damage of 50 or more, and the objective `ships_crippled` means propulsion or flight deck out |

### Experience

At Savo Island both sides fire by the same rules, and decision 0024 names that as a cause of the wrong result. If training is not a rule, tuning bends the gun and torpedo figures to do its work, and Cape Esperance, where the other side had the better of it, then fails for a reason no figure can mend. So experience is built before the surface figures are tuned.

A number for each side that tuning may move would explain any miss. So:

1. **A unit's experience is derived from fact rows by a written rule,** as a ship's figures are (decision 0020): flying hours, time in commission, night exercises. It is never worked back from a battle's result.
2. **The values are fixed before tuning starts** and are not changed to improve a report.
3. **Only the strength of the effect is tuned:** one figure for each rule that reads experience, the same for both sides.
4. **If research finds too little to rest the values on, experience waits,** and the surface figures stay marked as fitted and not checked.

Values set from training records before tuning either bring Cape Esperance out right or they do not. That is the test.

It is a blunter test than was hoped. At Savo Island the record gives shells fired and hits for both sides, so how well each side shot can be measured directly: 81 to 117 hits for each thousand shells for the Japanese, 21 to 36 for the Allies. At Cape Esperance the shells fired are on record for a few US ships only and for no Japanese ship from a source that can be relied on, though research has looked twice; and the derivation of rates of fire (decision 0070) reads those few as cases, so they cannot also check it. What is on record there is hits received by each side and ships sunk. Those depend on how long each side fired as well as how well. So the direct measure of experience exists only for the battle the figures are fitted on, and the check on the other battle is by its outcome. That makes conditions 1 and 2 carry more weight: if a unit's value could be moved, nothing would catch it.

`docs/step2-opening-list.md` stays the working list of these. Each becomes a decision record with a hand calculation, as in step 1.

## Done means

- All three battles pass, or each miss is written down with its cause and accepted by the designer.
- No figure in `data/reference` is a placeholder without a note saying what it rests on.
- The frozen-figure tests, the invariants, the determinism scenario and the soak still pass.
- The scenario with every rule of step 2 acting together passes ("Tests while figures move", 8), and was built before tuning began.
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
9. **Agreements at the edge.** Decided on Nov 12, 2026, after the baseline reports: the report marks them and they do not count against a battle for now. Whether they do at the close of step 2 is decided after the first tuning rounds show how many are left.
