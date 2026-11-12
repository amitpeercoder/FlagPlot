# The baseline, read

This is the architect's reading of the first reports, made Nov 12, 2026 on the figures in use that day, before any figure is tuned. The reports are the fact; this page says what they show and what each miss is likely to rest on. It is not a measure and it judges nothing.

| Battle | Report | Judged | Agree | Miss |
| --- | --- | --- | --- | --- |
| Savo Island | `savo-island-baseline.md` | 13 | 10 | 3 |
| Eastern Solomons | `eastern-solomons-baseline.md` | 11 | 8 | 3 |
| Rabaul against Guadalcanal | `rabaul-guadalcanal-air-baseline.md` | 10 | 4 | 6 |
| Cape Esperance | `cape-esperance-not-built.md` | 7 | not run | not run |

Each batch is 1,000 runs, seeds 1 to 1000, label `baseline`. The batch files are not kept in the repository: a report names the hash of every data file it ran on, and the same command on the same files gives the same report.

```
cargo run --release -p sim-cli -- batch data/scenarios/04_savo_island.toml --runs 1000 --label baseline --out BATCH.json
cargo run --release -p sim-cli -- validate BATCH.json data/validation/savo_island.toml
```

## Savo Island

- **The Japanese do not hit hard enough.** Their guns score 24 to 51 hits for each thousand shells where the record has 81 to 117, and no run reaches the record.
- **So the Allied cruisers live.** The record has four sunk. No cruiser is sunk in 681 runs of 1,000 and never more than three; the northern group, which lost all three, loses none in 948.
- **The Allied guns are about right** (16 to 53 for each thousand against 21 to 36), so the gap is on one side. This is the gap the experience rule is to close, with the readiness rules already in the model.
- **An agreement that is weaker than it looks:** gun hits on Japanese ships agree because the record of 10 to 17 touches the bottom of a spread of 8 to 224. The median run has 127. The Allies score many more hits than they did; a likely reason is that the fights last while the Japanese fail to win them quickly, which the report does not itself show.
- **The result:** an Allied victory of some kind in 441 runs and a defeat in 308. The night was a decisive Japanese victory; one run in 1,000 is.

## Eastern Solomons

- **The American aircraft do not die.** The record has 13 to 14 lost to the enemy and 20 to 21 in all; the middle run loses 2, and no run reaches the record.
- **American claims overstate too little:** about 1.5 claimed for each aircraft shot down, where the record is 2.4 to 3.6.
- **The American strike often does not fly.** Aircraft attacking Ryujo's force are none in the middle run (the record is 36), and Ryujo is sunk in 265 runs of 1,000. The measure of Japanese carriers sunk agrees only because the record, one, is what those 265 runs give.
- **Ryujo's aircraft, checked over the 1,000 runs** as the spec session asked. In the 265 runs in which she sinks, 23 of her aircraft go down aboard her in 256 and 6 come down in the sea; in the other 9 all 29 are in the air and come down in the sea. None is ever shot down. The record has about 4 lost with the ship and 5 from her strike. The cause is the scenario, not a figure: her fighters are ordered to patrol with seven in ten held on deck, and her attack unit waits for a ship to strike, so most of her aircraft are aboard when she is hit. In the record her strike was over Guadalcanal. Tuning is not to be asked to close this; the order that sends a strike at a named target (strike target priority, still to be planned) is what bears on it.
- **The result:** a stalemate in 542 runs, an Allied victory in 363, a defeat in 95.

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
