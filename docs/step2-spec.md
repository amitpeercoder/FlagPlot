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
| Cape Esperance, 11 to 12 October 1942 | `not-run/17_cape_esperance`, written and not run (decision 0072) | The surface figures, on a night they were not tuned on |

Two smaller checks ride with them: ship passages against recorded times, and claims against losses (decision 0056).

## Where the model stands today

From the reports in `docs/validation/`, each of 1,000 runs, which `docs/validation/baseline-reading.md` reads. These are the misses:

- **Savo Island** (`savo-island-r2.md`, after tuning round 2, kept): of 23 judged measures 19 agree, 1 at the edge, 3 miss. Allied torpedoes fired, 16 to 48 against 4 to 6. Tenryu and Yubari's shells, 2 to 5 hundreds against 1: the length of the fight, not the rate, and watched in every round (`baseline-reading.md`). Minutes of action, 35 to 65 against 20 to 30: the Japanese ended each fight by passing on, which no rule has (decision 0024, the note on break-off levels). Gun hits on Allied ships agree at the edge, 116 in the middle run against 153 to 219: the rest is the secondary guns (below). On the objectives scale an Allied defeat of some kind comes in 856 runs, and a decisive one, as the night was, in 422.
- **Eastern Solomons** (`eastern-solomons-baseline.md`, with the Japanese search set from the record, decisions 0073 and 0075): of 14 judged 10 agree and 4 miss. American aircraft lost to the enemy, 1 to 11 against 13 to 14, and to all causes, 1 to 12 against 20 to 21. American aircraft attacking Ryujo, none in the middle run and 84 at the 95th against 36: the strike flies whole or not at all. American claims, 144 for each hundred shot down against 237 to 362. On the scale 436 runs are stalemates, 468 Allied victories and 96 defeats; the day is usually judged a limited American victory.
- **Rabaul against Guadalcanal** (`rabaul-guadalcanal-air-baseline.md`): of 10 judged 3 agree, 1 at the edge, 6 miss. Fighters die far too fast on both sides (Japanese 30 to 50 against 6 to 10; American 27 to 40 against 7 to 23), American aircraft destroyed on the ground are 10 to 23 against 2 to 3, Henderson's fighters ready on day 21 are 1 to 9 against 11 and its dive bombers too many, and American claims are 129 for each hundred shot down against 250 to 382. Every run is a major Allied victory.
- Claims overstate less than the record does wherever the Americans claim, and the Japanese ships claimed at Savo Island too (reported: 100 for each hundred sunk in the middle run against 350); the Japanese claims of aircraft at the Eastern Solomons agree.
- A ship in the game fires its main battery and nothing else. A cruiser's secondary guns have no counterpart in the rules: at Savo Island they fired about a third of the Japanese shells (654 of 1,867). So the measures of shells fired count the main battery only, and the measures of gun hits, which the record cannot split by calibre, will run low by whatever the secondary guns scored. No rule is planned for this in step 2; it is named so that tuning is not asked to make the main battery do the secondary guns' work.
- A force in the game holds its ordered speed on a straight course. In the record a ship's speed made good is below the speed a source states for it: a half to three quarters for convoys and carrier forces, eight or nine tenths for a destroyer squadron on passage. Ships zigzagged, waited, slowed for landfall and towed (`docs/research/oob/passages.md`). No rule covers this. The designer has put it aside for now (see "Decided by the designer", 8). So passages are reported and not judged (`data/validation/passages.toml`).

## Measures

A measure is one number taken from a run, with the recorded value beside it. Each is a row in a file, `data/validation/<battle>.toml`, naming the fact rows it rests on. No recorded value is typed in without its rows.

| Battle | Measures |
| --- | --- |
| Savo Island | Ships sunk by side and by type, and by Allied group; Japanese ships sunk or left with propulsion out; who sighted first in the night fights, and the rounds each Allied group was surprised (reported); hits by side and by weapon; hits for each thousand shells; main battery shells by side, by Allied group, and Tenryu and Yubari's; Allied torpedoes fired; minutes of fighting, for when a force breaks off; fights by day |
| Eastern Solomons | Carriers sunk by side; bomb hits on each carrier; whether Enterprise's deck is out; aircraft lost by side, to the enemy and to all causes; the aircraft in each side's strike, and none at the Japanese fleet carriers; Ryujo's aircraft lost with her (reported) |
| Rabaul against Guadalcanal | Raid days and bomber sorties; aircraft lost by side and by cause; aircraft ready at Henderson Field over time; pilots lost. Supply used and the days the field could not fly have no measure: no row gives a figure for either |
| Cape Esperance | Japanese ships sunk and left with propulsion out; American cruisers and destroyers sunk; gun hits on Japanese cruisers, and on American ships (reported: some were from their own side); torpedo hits on Japanese ships. Not run before Savo Island is tuned (`docs/validation/cape-esperance-not-built.md`) |
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

- **`sim-cli batch` records more for each run** (decision 0062): aircraft lost by group and cause, hits by ship, shots and hits by side, attacks and escorts, aircraft ready by day, claims, supply used, the round a force broke off and the rounds each fight ran, the shots each ship fired, and who sighted first in each surface fight with the rounds each force was surprised. The batch file is at version 4, and `validate` reads version 4 only. The ways a measure takes its number from a batch (`take`) are listed in `data/validation/README.md`. Ship passages are worked by `validate --passages`, from the map and not from runs.
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
3. **Every round is recorded:** what was changed, why, and the report before and after, in `docs/validation/tuning/`.
4. **A figure is moved only toward something a source supports.** `docs/research/oob` has notes on gunnery, torpedoes, anti-aircraft fire and search. A figure with no support stays marked as a placeholder, however well it fits.
5. **A rule change is a decision record with a rule card,** as now, and the designer confirms any that changes how the game plays.
6. **What is derived is checked before what is tuned.** A ship's rate of fire, its guns and its speed are derived from fact rows, and the tuned figures multiply them. Before a mechanism's figures are tuned, the derived quantities it rests on are set against a measure that no tuned figure touches. For gunnery that measure is the shells Tenryu and Yubari fire at Savo Island: they were at battle readiness, so no rule of readiness or experience reads their count, and at the baseline they fired several times what they did. If a derived quantity is wrong the derivation is mended, by its written rule and from rows, not the figure. For the rates of fire that was decision 0070; for the radar a class has, decision 0077.
7. **A figure that only one side reads never covers for something both sides share.** The share of guns a force not at battle readiness fires with (decision 0068) starts at one half, which a row states. It may go lower only for what belongs to it, the confusion of a force caught unready. It is not taken down to make up for rates of fire that are too high for both navies, or for fights the record does not have. The same holds for the share of its torpedoes such a force fires (decision 0071): it starts at one half and carries unreadiness and nothing else. It does not carry the tubes that bear on one side only, which are left out and held under decision 0074.
8. **A scenario's orders are data, set from rows before the first round and then left alone.** What a force was ordered to do, the rules of engagement it fought under and the level of loss at which it breaks off are the scenario's, not figures, and each battle has its own. They are set from fact rows, with the row named beside the order, before the first tuning round of that battle is run. After that an order changes only when a row changes, and never to improve a report: an order moved after a report is seen is a figure set for one scenario under another name, which rule 1 forbids. Where no row says what a commander's order was, the scenario says so and keeps the default. The reading by which an order is taken from rows is itself written down, in the decision record of the scenario, and is applied to every battle alike, the check battles before they are ever run: a reading made only where it mends a miss is a fit.
9. **A figure that more than one battle reads is tuned on all of them at once.** Five figures of `surface_combat.toml` are read beyond the surface fight: the flotation a hit takes and the chance a hit puts a system out, when a bomb or an aerial torpedo hits (decision 0033); the `torpedo_system_hit` table, when an aerial torpedo hits; `claimed_sunk_fire`, in a raid's claim of ships sunk; and what a degraded system still does (`degraded_factor`), which also sets a ship's speed and a flight deck's launches. A round that moves one of them runs all three battles again, and its record shows all three reports before and after.
10. **A round aims at the middle, and the rule of agreement is only the floor.** A measure agrees when its record is inside the middle 90 percent and is held by one run in twenty. A round does not stop there. It aims to bring the record inside the middle half of the runs, where the report no longer marks the agreement as at the edge, and it says of each judged measure it leaves at the edge why it left it there. This matters most for a measure of a fact that is yes or no, or a count that is mostly nothing: the first batch with sightings recorded had the Japanese sight first in under half the night fights at Savo Island, and the measure for it agreed, at the edge, because one run in five happened to go the way the night did. Whether an agreement left at the edge counts against a battle at the close is still the decision put off under "Decided by the designer", 9.

Tuned on one battle and checked on the same battle proves little. So:

- **The figures of air combat are tuned on the Eastern Solomons and checked on Rabaul against Guadalcanal,** which they were not tuned on: search, the fight between aircraft, anti-aircraft fire, the attack on a ship or a field, and claims. A tuning round does not move one of these to mend a miss at Rabaul. A miss there goes back to the Eastern Solomons, or to the rule.
- **The figures that take weeks to show are tuned on Rabaul against Guadalcanal,** because nothing else in the slice can show them: fatigue, wear and repair, pilots and replacements, base supply. The Eastern Solomons is one day. **These figures have no battle to be checked on, and step 2 says so of each:** tuned on one battle and not checked. A second long scenario would check them; none is planned in step 2. With them go the four `crew_recovered_*` figures, since the Eastern Solomons counts no pilots, and `[base_hit]`, the attack on a field, since the Eastern Solomons has one raid of six bombers on one.
- **Some figures no tuned battle reads,** and they close as placeholders marked "no battle of the slice reads it": `air_upkeep.carrier_repairs_per_day` (scenario 5 does not load the air upkeep figures, and scenario 6 has no carriers), and `fatigue.force_fatigue_per_pulse_alert` and `force_rest_per_pulse` (scenarios 4 and 5 do not load the fatigue figures, and scenario 6 has no task forces).
- **Each tuning round's record names which kind it moved.** That is how a reader of a Rabaul report knows what it shows.
- **Surface figures are tuned on Savo Island and checked on Cape Esperance,** where the other side had the radar and the surprise. A model that gets Savo Island right only because the Japanese always win at night fails there. Cape Esperance is built after Savo Island is tuned and is never tuned on. Its ships are derived and its scenario is written (decision 0072), in `data/scenarios/not-run/`, where no loop of CI or the tests runs it; it moves to `data/scenarios/` when it may be run, after Savo Island is tuned. **The check is one run of the batch** (committed in advance, decision 0084): the first batch of 1,000 runs on the figures and rules as they stand when it moves is the check, and its report is kept as it comes out. Any rule or figure changed after that report is read is not checked by it, and the change's record says so. What the check can show is narrower than "a battle the figures were not tuned on": the rate of fire of the US 6-inch cruisers rests on Helena's own firing there, and Salt Lake City's is a case of the salvo rate (decision 0070); the scenario's speed, break-off levels and who takes part were set with the outcome known (decision 0072). Decision 0084 says what it can and cannot show.

## Tests while figures move

Step 1's scenario tests assert exact numbers from a hand calculation. Those numbers come from the figures step 2 is about to change. Each tuning round would break every one.

In force since the spec was approved:

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

   A rule with no draw has nothing to count or to pin, and is proved by its hand calculation alone.
8. **One scenario has every rule of step 2 acting together,** built when the last of them is in and before any figure is tuned. With each new rule switched off in the frozen copy, no other hand calculation has two of them acting at once. It is a cut-down Eastern Solomons that can be read at one sitting: one carrier a side, a patrol, a strike beyond the radius, a deck that goes out with aircraft away, pilots counted, experience unequal. Its hand calculation covers everything that is determined: who is within which reach, each chance at its working value, each expected count, and the order the rules run in. It carries pinned outcomes for its seed. The rule cards of the step's decisions are read together on it, which makes it the complexity review of step 2.
9. **Frozen figures go with frozen scenarios.** A scenario whose orders or data are set from the record keeps a frozen copy, as the figures do. Setting a battle's orders from fact rows is not a rule change, and it would move every hand calculation of that scenario; and a live order may name what the frozen figures refuse, so that it would drop out of a test without a word (decision 0075, "As built"). So every test that runs such a scenario on the frozen figures, hand calculation or not, runs on a copy of it as it stood, and the live scenario is covered by the run baselines and by `validate`. The copies are in `data/scenarios/step2-frozen/` (scenarios 4, 5, 7 and 8 today; decisions 0024 and 0073), whose `README.md` says who reads them. There is one frozen copy for a scenario and no more; it changes only with a rule change's new hand calculation; and the test says in its doc comment that it runs on the frozen copy. Whether the hand calculations are worked again on the live scenarios is decided with the frozen figures, when step 2 closes.

## Rule work that comes before tuning

A figure cannot be tuned to cover for a rule that is missing. So the rules that decide which strikes fly and what they hit are built before the air figures are tuned. The harness, the measures files and a first report on today's figures are built meanwhile; that report is the baseline.

| Item | State | Must it land before tuning? |
| --- | --- | --- |
| The state hash at version 2, and batches that record their inputs | Done (decision 0057) | Done |
| Ships charged straight-line miles | Done (decision 0058). Savo Island and the Eastern Solomons are re-derived for it (decisions 0024 and 0031, amended) | Yes, and it has landed |
| Strike target priority as an order | Built (decision 0063, stage 1), with the scenario orders of the Japanese carriers and Ryujo's raid ("As built (the scenario orders)"). The three strike measures at the Eastern Solomons are judged | Yes, and it has landed |
| Aircraft that cannot land where they took off: reach set by order, ditching for want of fuel, landing on another deck or at a base when a deck is lost, and the crew's recovery by where the aircraft comes down | Built in all its stages (decision 0059), the pilot of a patrol shot down over its own base or force included. The two measures of losses to all causes at the Eastern Solomons are judged; the Japanese one leaves out the five dive bombers lost coming home in the dark, which the rule does not cover | Yes, and it has landed |
| Experience: one number for an air group and one for a ship's crew | Built for ships (decision 0064, stage 1): the values are derived and fixed, and at night the gun, torpedo and sighting chances read them. For air groups (stage 2) it waits, by the fourth condition below: worked by hand from the rows, the Japanese carrier groups' value rests on one statement of hours for the whole service in 1941, every US Navy group sits at the middle only because no row exists, and the Marine squadrons' two questions are one fact counted twice. Research looked again and found almost nothing more: no flying hours for a US Navy pilot joining in 1942, only the lengths of courses; none for a Japanese pilot joining in 1942; nothing for two of the Marine squadrons. So stage 2 is not built in step 2, and the air figures are tuned with every air group alike in experience. That is a named gap, not a finding that experience did not matter in the air. Morale is left out | Yes for ships, and it has landed. Air groups wait on research |
| A force that is not at battle readiness when a fight begins fires with part of its guns | Built (decision 0068). Ready is battle readiness by order, or alert since before the pulse began. The share is of guns and holds in any fight, by day too; decision 0071 adds a share of the torpedoes | Yes, and it has landed |
| Rates of fire as achieved, not as designed | Built (decision 0070), a change to the derivation: a class fires at the lesser of its sourced rate and the median achieved rate of its kind of battery, from named cases. Tenryu and Yubari's shells fell from 9 hundreds in the middle run to 3 against a record of 1; what is left is the length of the fight, ruled explained and watched in every round (`docs/validation/baseline-reading.md`) | Yes, and it has landed |
| Surface fights by day with ships that fell behind | Looked at in decision 0071: the cause is the cripples made by too many Allied torpedo hits at night, and a force not ready now fires a share of its torpedoes. Fights by day agree (776 runs of 1,000 have none, `savo-island-r2.md`). If the measure misses once the night's hits on Japanese ships are near the record, a force scuttling a ship it leaves dead in the water is next (0071, option 3) | Yes, and it has landed |
| The Japanese search at the Eastern Solomons set from the record, and search ranges as flown | Built (decisions 0073 and 0075). The vanguard's floatplanes search 120 to 180 degrees to the 330 miles flown, the fleet carriers' attack units stay on naval strike, and Tone's floatplanes have no order. A type has a search radius of its own in the derivation, a ceiling below which an order names the distance flown | Yes, and it has landed |
| Which radar sets count for sighting ships at night | Built (decision 0077), a change to the derivation: only a surface-search set with a plan position indicator gives a class `radar`. At Savo Island no ship has one; at Cape Esperance Helena and Boise do | Yes, and it has landed |
| Which enemy ship a ship fires at | Built (decision 0078): guns like at like, torpedoes at the most important kind afloat. At Savo Island Allied cruisers sunk and Allied destroyers sunk, which missed in opposite directions for this one cause, both agree (`savo-island-r2.md`) | Yes, and it has landed |
| How long surprise lasts | Decided: no rule (decision 0076). In the record a force under fire replied within about one round, which the rules give | No rule to land |
| Savo Island's orders set from the record | Set (decision 0024, the notes of Nov 13, 2026): the northern group's break-off level is 1, the southern group and the Japanese force keep 0.5, marked invented, and Yunagi is in a task force of her own | Yes, and it has landed: no order moves after the first round ("Tuning", 8) |
| Radar fire control | Flagged by decision 0072 for Cape Esperance, where Helena and Boise laid their guns by radar; decision 0077 names it a gap. Ruled by the architect: no rule before the check is run (0072, "Built", 1) | No. If Cape Esperance misses for it when run, that is written as the cause and a plan follows |
| One scenario with every rule of step 2 acting together ("Tests while figures move", 8) | Built: `16_all_together` is on `main` | Yes, and it is |
| Torpedo tubes that bear on one side | Written as a change to the derivation (decision 0074) and held, not built. Alone it would halve the Japanese heavy cruisers' torpedoes, 40 to 20 against 32 to 34 fired, because three of them fired one side and then the other; made whole it needs a rule for firing the other side in a later round, which is new state and the designer's. It is built, with that rule, only if the first tuning rounds at Savo Island show the torpedo measures need it | No. This is the one place where a derived quantity known to be too generous is left as it is, and it is left because half the correction would be further from the record than none. "Tuning", 6, is not set aside for it: the torpedo figures are not tuned to cover it, and a tuning round that moves a torpedo figure says what 0074 would have changed |
| Ship repair | Decided (decision 0069, confirmed by the designer on Nov 13, 2026); not built. Not part of step 2's close: it adds an order and a briefing line, which "Scope" leaves out of step 2. It follows step 2, with land units and ground combat, before step 3 (ruled by the coordinating architect on the designer's delegation, Nov 13, 2026) | No. It matters to a campaign, not to one battle's losses |
| "Out of the campaign" on the objectives scale | Not decided. Not part of step 2's close: it follows step 2 with ship repair, before step 3 (the same ruling) | No. The result on the scale stays reported and not judged |
| Two objectives that can be gamed: cargo unloaded and loaded back scored each time, and a base scored open by the last moment only | Built (decision 0067): cargo landed is tons unloaded less tons loaded back at that base, and base open is scored by the share of days | No. No measure rests on either, and the result on the scale is reported and not judged |
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

**How each line is met,** as ruled by the coordinating architect on the designer's delegation, Nov 13, 2026:

- **The placeholders:** a note on every figure, written at the close from an inventory of the figures: its source, or the battle it was fitted on and not checked, or that no battle of the slice reads it.
- **The standing checks:** the hundred-seed soak, which CI does not run (it is ignored), is run by hand at the close.
- **The legibility test** is run as predict, then run, then explain: scenario 5 is one day, so "predicts the next day" cannot be done as written. The model predicts the day from the opening briefing, the day is run, and a fresh model explains an outcome from the final briefing and `explain`.
- **The rulebook-only test** is defined afresh and its record committed: the model, the scenarios, the prompt, the calculations and its answers. The step 1 run left none.

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
10. **Who decides, from Nov 13, 2026.** The designer named a coordinating session as overall architect and project manager for finishing step 2, and gave it the decisions that had been his: research, development, the plans that add or change a rule, and the reversal of an earlier answer of his where the evidence calls for it. He confirmed this to the spec session in his own words ("Yes, it coordinates"; "Yes, it decides"). A decision made this way is recorded with his words of delegation and the date as its authority, not as his approval of the rule itself, and he is told of it. The first is decision 0071, which reverses his answer to decision 0068 that torpedoes stay as they are. The spec and the measures stay with the spec session, and the rules of this document bind whoever decides.
11. **Who keeps the spec, from Nov 13, 2026.** The spec session is archived. The designer named the coordinating architect, the main session, overall architect and project manager ("You're the overall architect and project manager"; "You are empowered to make decisions, write specs"). It keeps this document and `data/validation/` and settles what the measures read. The work is done by Opus team leads with Sonnet developers under them; the architect plans, rules and reads their pull requests.
