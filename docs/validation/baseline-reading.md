# The baseline, read

This is the architect's reading of the first reports, made Nov 12, 2026 on the figures in use that day, before any figure is tuned. The reports are the fact; this page says what they show and what each miss is likely to rest on. It is not a measure and it judges nothing.

| Battle | Report | Judged | Agree | Agree at the edge | Miss |
| --- | --- | --- | --- | --- | --- |
| Savo Island | `savo-island-baseline.md` | 16 | 8 | 2 | 6 |
| Eastern Solomons | `eastern-solomons-baseline.md` | 14 | 10 | 0 | 4 |
| Rabaul against Guadalcanal | `rabaul-guadalcanal-air-baseline.md` | 10 | 3 | 1 | 6 |
| Cape Esperance | `cape-esperance-not-built.md` | 7 | not run | not run | not run |

An agreement at the edge is one whose record has no part in the middle half of the runs (the 25th to the 75th percentile). It still agrees by the rule; whether it counts against a battle at the close of step 2 is the designer's to decide after the first tuning rounds. The reports were remade once, the same day and on the same batches, when that mark was added to `validate`.

The Savo Island report was remade again that day with two more judged measures, the shells each side fires, and both miss: the Japanese fire 8 to 34 hundreds against a record of 18, and the Allies 4 to 50 against 4. That remake ran on the engine after decision 0063 (state layout version 5); the other two reports are still the first ones and will be remade when the Eastern Solomons gets its new orders.

The six ship passages are in `passages-baseline.md`. None is judged. Every one but the first is quicker in the rules than in the record, by 5 to 22 percent of the recorded time: a ship in the rules holds its stated speed along the route, and the record's ships did not. That is speed made good, which is not in step 2.

Each batch is 1,000 runs, seeds 1 to 1000, label `baseline`. The batch files are not kept in the repository: a report names the hash of every data file it ran on, and the same command on the same files gives the same report.

```
cargo run --release -p sim-cli -- batch data/scenarios/04_savo_island.toml --runs 1000 --label baseline --out BATCH.json
cargo run --release -p sim-cli -- validate BATCH.json data/validation/savo_island.toml
```

## Savo Island

- **The Japanese do not hit hard enough.** Their guns score 24 to 51 hits for each thousand shells where the record has 81 to 117, and no run reaches the record.
- **So the Allied cruisers live.** The record has four sunk. No cruiser is sunk in 681 runs of 1,000 and never more than three; the northern group, which lost all three, loses none in 948.
- **The Allied guns hit at about the right rate and fire far too much.** Their hits for each thousand shells are 16 to 53 against a record of 21 to 36. But the record has 471 Allied shells fired and the runs average about 2,800, where the Japanese fire about 2,170 against a record of 1,867. So Allied gun hits on Japanese ships are 127 in the middle run against a record of 10 to 17. A rate that agrees hid a count that is six times too high; two measures of shells fired were added on Nov 12, 2026 so that it cannot hide again. In the record most of the Allied cruisers never got their guns into effective action, and nothing in the rules keeps a surprised force from firing its full broadside from the second round.
- **An agreement that is weaker than it looks:** gun hits on Japanese ships agree because the record of 10 to 17 touches the bottom of a spread of 8 to 224. The median run has 127. The Allies score many more hits than they did; a likely reason is that the fights last while the Japanese fail to win them quickly, which the report does not itself show.
- **The result:** an Allied victory of some kind in 441 runs and a defeat in 308. The night was a decisive Japanese victory; one run in 1,000 is.

### The Savo Island report as remade with experience, Nov 12, 2026

The report in the file now is on the engine with experience for ships (decision 0064, stage 1), the ships' values derived from the fact rows, and the three strengths at their placeholder of 0.25. The bullets above describe the report before it.

- **The Japanese hit harder, and not hard enough:** 28 to 64 hits for each thousand shells, 48 in the middle run, against a record of 81 to 117. It was 39. At the cap of 0.5 the arithmetic gave about 61, so experience was never going to close this alone.
- **The Allied cruisers still live:** none is sunk in the middle run, and the record's four is held by no run.
- **Allied destroyers sunk is a new miss:** 3 in the middle run against a record of none, where it was 2 and agreed at the edge. The Japanese hit harder, and the Allied destroyers still fire far too much and are fired at in return.
- **The shells barely move** (Allied 25 hundreds in the middle run against 4; Japanese 22 against 18). Experience is how well a crew fights at night, not whether its guns are manned: that is decision 0068, which is built next.
- **The result has turned toward the record:** an Allied defeat of some kind in 440 runs and a victory in 347, where it was 308 and 441. The night was a decisive Japanese victory; ten runs in 1,000 are.

The three strengths are placeholders. They are tuned after decision 0068 is in, and not before.

## Eastern Solomons

- **The American aircraft do not die.** The record has 13 to 14 lost to the enemy and 20 to 21 in all; the middle run loses 2, and no run reaches the record.
- **American claims overstate too little:** about 1.5 claimed for each aircraft shot down, where the record is 2.4 to 3.6.
- **The American strike often does not fly.** Aircraft attacking Ryujo's force are none in the middle run (the record is 36), and Ryujo is sunk in 265 runs of 1,000. The measure of Japanese carriers sunk agrees only because the record, one, is what those 265 runs give.
- **Ryujo's aircraft, checked over the 1,000 runs** as the spec session asked. In the 265 runs in which she sinks, 23 of her aircraft go down aboard her in 256 and 6 come down in the sea; in the other 9 all 29 are in the air and come down in the sea. None is ever shot down. The record has about 4 lost with the ship and 5 from her strike. The cause is the scenario, not a figure: her fighters are ordered to patrol with seven in ten held on deck, and her attack unit waits for a ship to strike, so most of her aircraft are aboard when she is hit. In the record her strike was over Guadalcanal. Tuning is not to be asked to close this; the order that sends a strike at a named target (strike target priority, still to be planned) is what bears on it.
- *Nov 12, 2026:* the Ryujo bullet above describes the scenario before the orders of decision 0063 ("As built (the scenario orders)"): her detachment is now in (7,-9) at noon, her attack unit raids Henderson Field, and nothing of hers goes down aboard her. The report is remade by the second session.
- **The result:** a stalemate in 542 runs, an Allied victory in 363, a defeat in 95.

### The Eastern Solomons report as remade, Nov 12, 2026

The report was remade after the scenario got the orders of decision 0063 and three strike measures became judged. The bullets above describe the first report; these describe the one in the file now.

- **Ryujo's raid on Henderson Field flies in every run,** six attack planes and fifteen fighters, and none of her aircraft goes down aboard her (the record has 4; the measure is reported, not judged). She is sunk in 464 runs of 1,000.
- **The American strike on her is all or nothing.** It flies in about 470 runs, and then with about 82 aircraft where the record has 36; in the other runs no aircraft attacks her. So the middle run has none, the top of the spread is 84, and no run holds 36: a miss. Every American strike group in reach flies at the one contact, where on the day Enterprise's second strike was held and then found nothing.
- **The Japanese strike on Enterprise now agrees** (33 aircraft in the middle run against 37; three bomb hits against three).
- **The American aircraft still do not die:** 5 lost to the enemy in the middle run against 13 to 14, and 6 in all against 20 to 21.
- **American claims still overstate too little:** about 1.4 for each aircraft shot down against 2.4 to 3.6.
- **Two of Ryujo's six attack planes run out of fuel on the way home in the middle run,** which the record does not show: her raid flies 200 miles on a derived radius of 175. Noted for tuning, at the radius or at the placeholder fuel figure.

One thing to hold in mind when reading any of these reports: an agreement on a yes-or-no measure, or on a count that is mostly 0 or 1, says only that the record is not surprising in the model, never that the model expects it. Japanese carriers sunk agrees with the middle run sinking none; fights by day at Savo Island agrees with a fight by day in two runs of three. The edge mark cannot show that.

## Rabaul against Guadalcanal

- **Fighters die far too fast on both sides.** Japanese fighters lost to the enemy: 30 to 50 against a record of 6 to 10. American fighters lost to enemy aircraft: 27 to 40 against 7 to 23.
- **Too many American aircraft are destroyed on the ground:** 10 to 23 against 2 to 3.
- **So Henderson's fighters run down:** 1 to 9 ready on day 21 against a record of 11. The dive bombers end too strong (22 to 32 against 22, and only 35 runs hold 22): they are not being used up.
- **The bombers are right:** 12 lost against a record of 12, and the early days at Henderson agree.
- **American claims overstate too little here too:** about 1.3 for each shot down against 2.5 to 3.8.
- **Too many bombers reach the island** (252 to 272 against 211 to 213, reported and not judged): the model has no weather and no raid that turns back.
- **The result is the same in every run:** a major Allied victory, margin 10 to 11.9. The points do not yet tell one run of this scenario from another.

## What this says about the order of work

1. The surface gap at Savo Island is one-sided and large. It waits for the experience rule.
2. The air gaps pull in opposite directions in the two air battles: too few American losses in the carrier battle, too many fighter losses over Guadalcanal. One set of figures serves both, so the cause is more likely in which fights happen and who is in them (escorts, patrols, which strikes fly) than in a single chance. The strike target priority rule comes before the air figures are touched, as the spec already says.
3. Claims overstate too little in every battle that measures them. The three claim chances are placeholders and are the simplest thing here to set from the record.
