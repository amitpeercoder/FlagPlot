# The baseline, read

This is the architect's reading of the first reports, made Nov 12, 2026 on the figures in use that day, before any figure is tuned. The reports are the fact; this page says what they show and what each miss is likely to rest on. It is not a measure and it judges nothing.

| Battle | Report | Judged | Agree | Agree at the edge | Miss |
| --- | --- | --- | --- | --- | --- |
| Savo Island | `savo-island-baseline.md` | 19 | 10 | 1 | 8 |
| Eastern Solomons | `eastern-solomons-baseline.md` | 14 | 10 | 0 | 4 |
| Rabaul against Guadalcanal | `rabaul-guadalcanal-air-baseline.md` | 10 | 3 | 1 | 6 |
| Cape Esperance | `cape-esperance-check.md`, the check (below) | 8 | 4 | 1 | 3 |

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

### The Savo Island report as remade with the readiness rule, Nov 13, 2026

The report in the file now is on the engine with decision 0068 as well: a force not at battle readiness fires its guns at a share, the placeholder 0.5. Both rules that the spec put before tuning at this battle are in.

- **The Allied groups fire half what they did:** 13 hundreds of shells in the middle run, where it was 25, against a record of 4. By ship, the southern group fires about 440 and the northern about 870 in the middle run. It is still three times the record; the share would have to go well below the half that the record states, which the designer has allowed tuning to do.
- **Gun hits on Japanese ships fall with them,** from 96 to 53 in the middle run against a record of 10 to 17. The measure agreed at the edge before and misses now: fewer than one run in twenty holds the record. Nothing got worse; the spread narrowed off a record it had only been brushing.
- **Fights by day are rarer:** 518 runs of 1,000 have none, where 438 had.
- **Neither rule has moved the two things at the heart of the night.** The Japanese guns hit 50 times for each thousand shells against 81 to 117, and no Allied cruiser is sunk in the middle run where four were. These are for tuning: the strength of experience, the share, and the chance to hit itself.
- **The result has turned further toward the record:** an Allied defeat of some kind in 563 runs and a victory in 216. Ten runs in 1,000 are the decisive Japanese victory the night was.

### Shells by group, Nov 13, 2026

Three more judged measures count the shells of the southern group, the northern group, and Tenryu and Yubari, from the shots each ship fired. The report was remade for them on the same engine and figures.

- **Tenryu and Yubari miss, and this is the one to read first:** 4 to 15 hundreds, 9 in the middle run, against a record of 1 to 2. Both are at battle readiness, so neither the readiness rule nor experience touches what they fire. What is wrong is under them: a round is five minutes at a class's designed rate of fire, and no ship fired like that. The spec now puts the rates of fire before any tuning of the surface figures, as a matter for the derivation.
- **The two Allied groups agree, and neither agreement is strong.** The southern group's record is under a hundred shells; 373 runs of 1,000 hold it, most of them runs in which the group never fired at all, and the middle run fires 4 hundreds. (Since decision 0084 that measure counts in tens, record 3 to 5, so a run that never fires no longer holds it: on the report of round 2 it misses, 0 to 47 tens with a median of 9, held by 10 runs.) The northern group's record is 3 to 4 hundreds; 79 runs hold it and the middle run fires 8. One round at half manning is 67 shells for a cruiser and 150 for a destroyer, which is already more than any ship of the southern group fired all night.
- **The order of work that follows,** as the spec has it: the rates of fire in the derivation, from rows on rates achieved; then the fights by day; then the share, which stays at one half until then and goes lower only for the confusion decision 0068 names.

### The reports tuning starts from, Nov 13, 2026

All three were remade once, on batch version 4, after everything that had to come before tuning was on main: the rates of fire as achieved (decision 0070), the share of torpedoes a force not ready fires (0071), the Japanese search at the Eastern Solomons from the record and a search radius of its own (0073, 0075), the northern group's break-off level set from the record (0024, the note of Nov 13, 2026), the record of who sighted first (0062, version 4) with its measures, and which radar sets count for sighting ships at night (0077).

| Battle | Judged | Agree | At the edge | Miss |
| --- | --- | --- | --- | --- |
| Savo Island | 23 | 14 | 2 | 7 |
| Eastern Solomons | 14 | 10 | 0 | 4 |
| Rabaul against Guadalcanal | 10 | 3 | 1 | 6 |

Savo Island, where the surface figures are tuned first:

| What | Record | 5th to 95th and median | Verdict |
| --- | --- | --- | --- |
| Night fights the Japanese did not sight first | 0 | 0 to 2, 1 | agrees (335 runs hold it) |
| Japanese main battery shells, in hundreds | 10 to 11 | 9 to 21, 17 | agrees, at the edge |
| Allied main battery shells, in hundreds | 3 | 1 to 10, 4 | agrees |
| Tenryu and Yubari's shells, in hundreds | 1 | 2 to 4, 3 | miss |
| Gun hits on Allied ships | 153 to 219 | 35 to 139, 100 | miss |
| Japanese gun hits for each thousand shells | 81 to 117 | 37 to 71, 57 | miss |
| Allied cruisers sunk | 4 | 0 to 2, 1 | miss |
| Northern group cruisers sunk | 3 | 0 to 2, 0 | miss |
| Allied destroyers sunk | 0 | 1 to 4, 3 | miss |
| Allied torpedoes fired | 4 to 6 | 8 to 36, 20 | miss |
| Minutes of action | 20 to 30 | 30 to 60, 35 | agrees, at the edge |

What it shows:

- **The shells are now about right and the hits are not.** With the achieved rates both sides fire near what they fired. The Japanese hit at about 57 for each thousand shells against 81 to 117, so they land half the hits the record has and no more than two cruisers sink. That is the chance to hit, the second mechanism, and nothing else is left for it to hide behind.
- **Tenryu and Yubari** (the check of what is derived before what is tuned, step 2 spec, "Tuning", 6). Their rate of fire is derived from rows and did not change between the batches in which this measure moved: it agreed at the edge after decision 0070 and misses again now that the northern group stays and its fight runs longer. What is left is the rounds they fire, about five against three, which is the length of the fight and not the rate. The coordinating architect rules the check explained and not a hold, with a watch: every round's record gives this measure and the rounds the pair fire; if the minutes of action come to the middle of the runs and the pair still fire over the record, tuning stops and it goes back to the derivation; and no figure is moved to mend this measure itself. **Said plainly (decision 0084):** this was the check of the derived rates, it failed, and tuning went on. It is a miss, not a passed check, and the rates of fire are not independently certified by it: what the pair fire is the rate times how long they fire, and how long they fire moves with every figure that changes which ships survive.
- **Finding and surprise.** With no Allied ship given the radar bonus (decision 0077, a mend in the derivation and not a tuning round), the Japanese sight first in 57 night fights of 100, where it was 44, and one run in three has every night fight go their way. The measure now agrees plainly by the rule, with a median of one fight a run still going the other way: mostly both sides sighting in the same round of five minutes. A round aims at the middle half of the runs ("Tuning", 10), so this is still the first mechanism's task. The two measures of rounds surprised are reported only: the rules end a force's surprise in the round after it is first fired on (decision 0076).
- **Allied torpedoes fired** is judged for the first time and misses by about four times, with the share of decision 0071 at its first value. Half of what is left is tubes that do not bear on one side (decision 0074, held).
- **Minutes of action and Japanese shells** are at the edge for a cause that is named and left: the Japanese ended each fight by passing on, and the rules have no force that retires when it has done what it came for (decision 0024, the note).

No figure of finding and surprise is moved: `docs/validation/tuning/r0-finding-and-surprise.md` says what was looked at and why.

The Eastern Solomons and Rabaul reports keep the misses they had: American aircraft lost, the all-or-nothing strike on Ryujo and claims at the Eastern Solomons; fighter losses, aircraft destroyed on the ground, claims and the aircraft ready after three weeks at Rabaul. Nothing in this remake was aimed at them.

### Savo Island with the rule for whom a ship fires at, Nov 13, 2026

Remade once more after decision 0078 (guns like at like, torpedoes at the most important kind afloat) and with Yunagi in a task force of her own (decision 0024, the note). The other two reports did not change: no run of theirs has a fight between forces of mixed kinds. Judged 23, agree 14, at the edge 4, miss 5. This is the report the tuning rounds start from.

| What | Record | Before 0078 | Now | Verdict now |
| --- | --- | --- | --- | --- |
| Allied cruisers sunk | 4 | 0 to 2, 1 | 0 to 4, 2 | agrees, at the edge (113 runs hold it) |
| Northern group cruisers sunk | 3 | 0 to 2, 0 | 0 to 3, 1 | agrees, at the edge |
| Southern group cruisers sunk | 1 | 0 to 1, 0 | 0 to 2, 1 | agrees |
| Allied destroyers sunk | 0 | 1 to 4, 3 | 0 to 0, 0 | agrees |
| Japanese ships sunk | 0 | 0 to 1, 0 | 0 to 1, 0 | agrees |
| Gun hits on Allied ships | 153 to 219 | 35 to 139, 100 | 30 to 129, 87 | miss |
| Gun hits on Japanese ships | 10 to 17 | 2 to 45, 13 | 9 to 68, 25 | agrees |
| Japanese gun hits for each thousand shells | 81 to 117 | 37 to 71, 57 | 38 to 72, 58 | miss |
| Allied gun hits for each thousand shells | 21 to 36 | 10 to 50, 29 | 21 to 60, 40 | agrees |
| Japanese main battery shells, in hundreds | 10 to 11 | 9 to 21, 17 | 6 to 19, 14 | agrees |
| Allied main battery shells, in hundreds | 3 | 1 to 10, 4 | 3 to 13, 6 | agrees, at the edge |
| Tenryu and Yubari's shells, in hundreds | 1 | 2 to 4, 3 | 2 to 5, 3 | miss |
| Allied torpedoes fired | 4 to 6 | 8 to 36, 20 | 16 to 36, 36 | miss |
| Minutes of action | 20 to 30 | 30 to 60, 35 | 35 to 65, 40 | miss |

What it shows:

- **The hits now fall where the record's did.** No Allied destroyer is sunk in 984 runs of 1,000, and four cruisers sunk is held by one run in nine where it was held by four in a thousand. The two measures that missed in opposite directions for one cause have both come in.
- **Gun hits on Allied ships fall** (median 87 from 100), though the cruisers take more: the destroyers no longer take any, and with Yunagi away there is one ship fewer firing. The hit rate itself is unmoved at 58 for each thousand against 81 to 117. That is the next round's, and it is now the whole of what stands between the model and the cruisers' losses.
- **The Allied ships live longer and fire longer,** so gun hits on Japanese ships rise to a median of 25 against 10 to 17 (still agreeing, by its spread) and Allied shells go to the edge.
- **Minutes of action now misses** (35 to 65 against 20 to 30). The cause is named and not tuned: the Japanese ended each fight by passing on, and no rule has a force retire when it has done what it came for (decision 0024, the note on break-off levels). With the northern group never leaving and its cruisers hard to sink, its fight runs the full six rounds in many runs.
- **Allied torpedoes fired** misses further (36 against 4 to 6): every Allied destroyer now lives to fire. What is left is readiness (decision 0071's share, at its first value) and tubes that do not bear (decision 0074, held).
- **Tenryu and Yubari** still miss, for the length of the fight and not the rate; the watch on them stands.

### Savo Island after round 2, Nov 13, 2026

Two figures moved, each in a round of its own: the strength of experience for guns to its cap of 0.5 (round 1) and the factor against a surprised target to 2 (round 2). Both act on one side more than the other. The report is `savo-island-r2.md`: judged 23, agree 19, at the edge 1, miss 3. The chance to hit was closed there.

| What | Record | Tuning started | After round 2 |
| --- | --- | --- | --- |
| Japanese gun hits for each thousand shells | 81 to 117 | 38 to 72, 58: miss | 49 to 92, 75: agrees |
| Gun hits on Allied ships | 153 to 219 | 30 to 129, 87: miss | 35 to 158, 116: at the edge |
| Allied cruisers sunk | 4 | 0 to 4, 2: at the edge | 0 to 5, 3: agrees |
| Northern group cruisers sunk | 3 | 0 to 3, 1: at the edge | 0 to 3, 2: agrees |
| Minutes of action | 20 to 30 | 35 to 65, 40: miss | 35 to 65, 40: miss |
| Tenryu and Yubari's shells, in hundreds | 1 | 2 to 5, 3: miss | 2 to 5, 3: miss |
| Allied torpedoes fired | 4 to 6 | 16 to 36, 36: miss | 16 to 48, 34: miss |

- **The chance to hit is done.** The Japanese rate agrees; gun hits on Allied ships are short by what the secondary guns scored, which the rules do not fire.
- **The three misses are named and left:** the Japanese passing on, which no rule has; the pair's rounds, which follow from it, under the watch; and Allied torpedoes, of which Helm and Wilson fire 16 in nearly every run, two ships the rows have not firing for want of orders and for a target that was friendly.
- **What a hit does and breaking off had no supported move.** A cruiser sinks at about the main battery hits the record's took. What is wrong is where the Japanese torpedoes go: all of them into the first fight, none into the northern group's, so Chicago sinks in about half the runs.
- **The tuned figures are fitted on Savo Island and not yet checked.** Cape Esperance checks them.

### The round 2 report with the measures of decision 0084, Nov 13, 2026

`savo-island-r2.md` was remade on the same figures and seeds with six measures added and one counted in tens. Judged 29: 21 agree, 2 at the edge, 6 miss. The totals that agreed hid this:

- **The northern group takes no torpedo hit in any run** (record 3 to 6), while the southern group takes a median of 4 (record 1 to 3, held by 418 runs). The total on the nine ships agrees.
- **Chicago is sunk in 560 runs of 1,000**; the record has her steam away. The measure agrees by the rule (440 runs hold it), with the median run sinking her.
- **Each northern cruiser takes too few gun hits:** Vincennes misses (median 33 against 57 to 74, one run holds it), Quincy is at the edge (28 against 36 to 54), Astoria agrees (23 against 34 to 65, held by 357).
- **The southern group fires too much or nothing:** in tens, median 9 against 3 to 5, held by 10 runs. Counted in hundreds it had agreed, because a run that never fired held the record.

Each measure is judged alone. That several agree does not show one run holding them together.

### Savo Island after decision 0083: the closing record, Nov 13, 2026

`savo-island-after-0083.md` is round 2's figures on the engine with decisions 0074 and 0083, and the measures of 0084, with `minutes_of_action` counting the sum of the two fights (32 to 36). Judged 29: 21 agree, 4 at the edge, 4 miss. The surface tuning is closed on it, with no further round (`tuning/savo-island-closed.md`). It is the closing record on its engine and is not remade for later engine changes.

- **Torpedoes now reach the northern group**: 0 to 3, median 0, in 364 runs one or more, at the edge against 3 to 6. Before 0083 it was none in any run. The southern group's agree: median 3, held by 571.
- **Chicago is still sunk in 565 runs.** The record's southern fight lasted about as long as the model's. The second sides went north in the record because the fights overlapped, which the rules do not do (decision 0083, "Fights that overlap"; a plan for the designer).
- **Minutes of action agree,** with the record in the middle half. The fights are no shorter (median 40); the record now counts what a run counts.
- **The four misses are named:**
  - Allied torpedoes fired: Helm and Wilson fire where the rows have neither firing. That is participation, not the share.
  - Tenryu and Yubari's shells.
  - Vincennes's gun hits.
  - The southern group's shells in tens.
- **The watch on Tenryu and Yubari was met in form and ruled.** The minutes came to the middle and the pair still fire over the record. The architect did not reopen the derivation: the record's shells fill the rows' short firing window (`engagement_events` 1060, 1069, 1071) at the derived rate. The pair fire through every round of both fights, where they fired for part of the night. Decision 0084's finding 11 stands: the gate failed and certified no rate.
- **The two tuned figures and the torpedo share stand,** looked at once more on this engine. They are fitted on Savo Island and not yet checked.

## Cape Esperance: the check, Nov 13, 2026

The one check of the surface figures (`docs/step2-spec.md`, "Tuning"; decision 0084, finding 1). One batch, label `check`, 1,000 runs, seeds 1 to 1000, on the figures and rules on `main` at 3e90304, with the scenario moved out of `not-run/` and the measures' takes added in commit 897ba4f before any batch of it existed. The report, `cape-esperance-check.md`, is kept as `validate` wrote it, and is not remade: it is the check on the engine of that commit. Nothing was tuned on it. Decision 0072, "As built (the check)", has the commands and the hashes.

**Judged 8: 4 agree, 1 at the edge, 3 miss. The battle does not pass.**

| Measure | Record (rows) | 5th to 95th, median | Runs holding | Verdict |
| --- | --- | --- | --- | --- |
| `japanese_ships_sunk` | 2 (1506, 1507, 1544, 1546, 1661, 1662) | 0 to 3, 2 | 407 | agrees |
| `japanese_ships_with_propulsion_out` | 0 (1508, 1547, 1514) | 0 to 1, 0 | 908 | agrees |
| `us_cruisers_sunk` | 0 (1526, 1529, 1536, 1538) | 0 to 0, 0 | 960 | agrees |
| `torpedo_hits_on_japanese_ships` | 0 to 1 (1603, 1604) | 0 to 6, 3 | 269 | agrees |
| `gun_hits_on_kinugasa` | 0 to 4 (1605, 4036, 1686, 1687, 1688) | 4 to 49, 15 | 60 | agrees, at the edge |
| `us_destroyers_sunk` | 1 (1532, 1533, 1534, 1530) | 0 to 0, 0 | 4 | **miss**: outside the middle 90 percent, and held by fewer than 1 run in 20 |
| `gun_hits_on_aoba` | 40 (1598, 4034, 1599) | 13 to 55, 29 | 18 | **miss**: held by fewer than 1 run in 20 |
| `japanese_ships_claimed_sunk_per_hundred_sunk` | 200 to 300 (1524, 1522, 1532) | 100 to 300, 100, over the 44 runs kept | 2 of 44 | **miss**: too few runs kept (956 sink no US ship) |

Reported, not judged: gun hits on Furutaka 10 to 57, median 27, against a floor of 90 that no run reaches; gun hits on the US ships 1 to 35, median 6, held by 179, against 15 to 29 (a ceiling, some of it their own side's); ships the US side claims sunk for each hundred sunk 100 to 250, median 100, held by 8 of 923, against 400 to 750. On the objectives scale the Allies win in 842 runs of 1,000 (449 major, 393 minor), 146 are a stalemate and 12 an Allied defeat.

What it shows:

- **The side with the radar and the surprise wins the night.** That is what the check was built to ask (the spec: "a model that gets Savo Island right only because the Japanese always win at night fails there"). It does not fail that way: two Japanese ships sunk in the middle run, and no US cruiser sunk in 960 runs. Task Force 64 sights first in 630 night fights of 1,000, both sides in the same round in 260, the Japanese first in 110; the Japanese force is surprised for one round in 630 runs.
- **The Japanese hit almost nothing.** Their guns fire a median of 171 shots and score 6 hits; a US destroyer is sunk in 5 runs of 1,000, where Duncan was lost. In the record Duncan was hit by both sides (the measure of hits on US ships says so; rows 1596, 1597), she had left the column to attack between the lines, and the Japanese claimed two or three ships sunk where the rules give them none to claim in 956 runs. The claims miss follows from the destroyers' miss: the measure keeps only runs in which a US ship sinks.
- **Aoba's 40 is missed by a weak record and a short fight.** The spread holds 40 (13 to 55), and a point record of a count near 30 is held by few runs of any spread; 0084 (finding 14) called the record weak before the run. Beside it, Kinugasa takes a median of 15 where her own reports say she took none, and Furutaka 27 where the rows put her over 90. The Allied guns spread their fire over the three cruisers like at like (decision 0078): San Francisco and Helena at Aoba, Boise at Furutaka, Salt Lake City at Kinugasa.
- **The night fight is short.** A median of 3 rounds of five minutes (1 to 6), the Japanese force breaking off in the night in 974 runs, where the record's fire ran from 23:46 to about 00:20. The three Japanese cruisers take a median of 81 gun hits all told (46 to 118); 12 runs reach 130.
- **Fights by day the record does not have.** In 771 runs Task Force 64 fights again in the morning, and in 352 in the afternoon too, against Japanese ships detached as stragglers that are still in its hex: it is ordered to stay in (2,-5), its turn away at 00:27 not ordered (decision 0072, 5). No measure of Cape Esperance counts fights by day. In the 229 runs with no fight by day the Japanese lose a median of 1 ship (0: 70, 1: 95, 2: 57, 3: 7); in the 771 with one, mostly 2 (1: 192, 2: 350, 3: 189, 4: 32, 5: 1). The two sets of runs are not alike, since a fight by day needs a ship already left behind, and the batch file does not say in which pulse a ship sank. So it is not shown how much of the agreement of `japanese_ships_sunk` rests on fights by day, only that some may.
- **Allied torpedoes:** the five destroyers fire all 50 tubes in nearly every run (40 to 50); in the record Buchanan and Duncan attacked and Farenholt's mount jammed, with no count for any ship (decision 0072, "Gaps"). No measure counts them. It is the pattern of Helm and Wilson at Savo Island: every ship of a force that has sighted fires.

**The three things fitted at Savo Island.** Within decision 0084's limits, which bind this paragraph: an agreement here cannot tell how well a side shot from how long it fired, its readiness and the damage it took; the US 6-inch rate of fire rests on Helena's firing in this battle, and Salt Lake City's is a case of the salvo rate; the meeting, who took part and the break-off levels were set with the outcome known; and each measure is judged alone. With that said: `experience_gun_strength` at 0.5 favours the Japanese here (all five at 3 against Task Force 64's mean of 2.11), and it did not turn the night their way; the Japanese guns, at about 47 hits for each thousand shots over the batch (the Allied at about 54), are not a measure (none is possible here), and their 6 hits in the middle run sit under the unready share and a round of surprise, so the check cannot say whether 0.5 is too much or too little. `surprised_target_factor` at 2 falls here on the Allied fire at a Japanese force surprised in round 1 in 630 runs; the hits it multiplies agree on ships sunk and miss on how they are spread between the cruisers, which is the rule of targets and not the factor, so the check gives no clean reading of it. The unready shares of 0.5 fall on the Japanese, as the scenario has them not ready (decision 0072, 9); their cruisers' main batteries fire a median of 85 shells (17 to 229) where the rows have 107 to 123 (`engagement_outcomes` 2276 to 2282), which is not a measure and is not against the share. **None of the three is shown wrong by the check, and none is shown right:** the outcomes it can read agree where the side that lost was the one the figures favour, and miss where the rules have no counterpart (fire on a friend, a ship between the lines) or a weak record.

**Each miss and the edge, and what it most likely rests on (conjecture, for step 3; nothing is tuned on it):**

| Measure | Most likely resting on |
| --- | --- |
| `us_destroyers_sunk` | Fire on a friend and ships acting apart, which the rules do not have (decision 0072, "What the rules cannot show"): Duncan was lost to fire from both sides after leaving the column. Behind them, the Japanese fire under the unready share and a round of surprise |
| `gun_hits_on_aoba` | The weak point record (decision 0084, 14); the length of the night fight, with the Japanese level of 0.5, invented; the rule of targets spreading the fire evenly; radar fire control, which the rules do not have and which the architect ruled is written as a cause if the check misses for it (0072, "Built", 1). The run's hits on the three cruisers, 81 in the middle run against the record's figures, which are historians' and not counts, bear on it |
| `japanese_ships_claimed_sunk_per_hundred_sunk` | The same as `us_destroyers_sunk`: too few runs sink a US ship to keep |
| `gun_hits_on_kinugasa`, at the edge | The rule of targets (decision 0078): Salt Lake City's place takes her while the three cruisers are afloat; in the record she took no fire (1686, 1687) |

Limits that stay: no measure of who sighted whom first was written for this battle (decision 0072 says which events it could read); there is no measure of hits for each thousand shells (the measures file's closing note); and the fights by day above are not measured. **The fights by day are a limit on the agreement of `japanese_ships_sunk`:** Task Force 64 fights stragglers by day in 771 runs of 1,000, and in the 229 runs without one the median is 1 Japanese ship sunk, not the record's 2.

Observations for step 3, from the batch and not measured: the fights by day (771 runs); the short night fight (a median of 3 rounds of five minutes, against about 34 minutes of fire in the record); and the destroyers' torpedoes (every tube fired in nearly every run, where two ships attacked).

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

### The Eastern Solomons report as remade with the orders from the record, Nov 13, 2026

Remade after decision 0081 set the scenario's remaining orders from the rows: each launch the record has apart is an air group of its own (the Japanese carriers' second wave, Enterprise's attack group, Saratoga's second group, all at rest), Enterprise's search is the afternoon's 16 SBDs and 7 TBFs to 250 miles, VMSB-232 rests, and the two strike measures count the aircraft that took part, not those left after the fight. Same figures, 1,000 runs, seeds 1 to 1000; before is main at cdae5bc validated against the measures as changed.

- **The Japanese strike was doubled, and two agreements rested on it.** The first wave is now the record's 27 dive bombers and 10 fighters. Counted as it took part, it holds the record of 37 in 607 runs (it met the American force in 611; in 4 the patrol shot down every bomber before the guns, and the log then names no force, so the measure counts none). Counted the old way, by the bombers left after the fight, it would have held 37 in no run (5 in the middle run, 0 to 17).
- **Bomb hits on Enterprise now miss:** 0 in the middle run, 0 to 2, against 3 (29 runs hold it; 0.50 a run, against 3.52 before). Of the 27 dive bombers, the patrol of 54 and the guns, all firing before the bombs fall, leave few. That is the finding that the rule changes of decisions 0079 and 0080 answer; nothing here was adjusted for it.
- **The strike on Ryujo agrees:** 38 aircraft in 488 runs (the record was then 36 to 38; since decision 0084 it is 38, as launched), none in the others. Ryujo is sunk in 228 runs, against 464, now that Enterprise's groups and VMSB-232 no longer strike her, so Japanese carriers sunk agrees at the edge.
- **The American losses are as far off as before** (4 and 5 in the middle run against 13 to 14 and 20 to 21), and the claims measures are unchanged in verdict. They are read after the fight's rounds.
- **Two of Ryujo's six attack planes still run out of fuel** coming home (1.72 a run): her raid flies 200 miles on a derived radius of 175. Decision 0081 keeps the radius (C5). It also called these ditchings evidence for the round of the fuel figure; decision 0084 reverses that: they rest on a radius whose rows state no load, so they are left out of any round that tunes `ditch_chance_per_radius_beyond`.
- **What took part and what attacked (decision 0084).** Two measures beside the strike measures keep the old count, the aircraft left to attack after the patrol and the guns. Saratoga's strike on Ryujo, left to attack, has the record 36, the aircraft that reached and released; it agrees (on the report as remade after decision 0079: 0 to 38, median 0, held by 130 runs; the strike reaches her in about half the runs, and a run in which the patrol and the guns take none attacks with all 38 it launched, which the rules cannot make 36). The Japanese bombers left to attack Enterprise's force are reported, not judged, since no row counts the bombers that reached release: 0 to 18 with a median of 11 after decision 0079 (0 to 9, median 1, before it, when the guns fired before the attack), against the target's estimate of 20 or more, held by 16 runs.
- **The result:** a stalemate in 547 runs, an Allied victory in 450, a defeat in 3 (before: 436, 468 and 96).

## Rabaul against Guadalcanal

- **Fighters die far too fast on both sides.** Japanese fighters lost to the enemy: 30 to 50 against a record of 6 to 10. American fighters lost to enemy aircraft: 27 to 40 against 7 to 23.
- **Too many American aircraft are destroyed on the ground:** 10 to 23 against 2 to 3.
- **So Henderson's fighters run down:** 1 to 9 ready on day 21 against a record of 11. The dive bombers end too strong (22 to 32 against 22, and only 35 runs hold 22): they are not being used up.
- **The bombers are right:** 12 lost against a record of 12, and the early days at Henderson agree.
- **American claims overstate too little here too:** about 1.3 for each shot down against 2.5 to 3.8.
- **Too many bombers reach the island** (252 to 272 against 211 to 213, reported and not judged): the model has no weather and no raid that turns back.
- **The result is the same in every run:** a major Allied victory, margin 10 to 11.9. The points do not yet tell one run of this scenario from another.

### The Rabaul report as remade with the orders from the record, Nov 13, 2026

Decision 0082 changed the scenario's orders before any air figure is tuned on Rabaul or checked there:
- the bombers raid on the eleven days of the units' forms, and the Japanese rest thresholds are gone;
- Henderson's dive bombers search 300 to 030 to 200 miles, a reading.

The report, `rabaul-guadalcanal-air-baseline.md`, is remade from that scenario: judged 10, 3 agree, 4 at the edge, 3 miss (before: 3, 1, 6).

- **American fighters lost now agree** (16 to 29, median 23, against 7 to 23). Japanese fighters lost still miss (16 to 30 against 6 to 10), at a little over half what they were.
- **F4Fs on day 21 and SBDs on day 21 agree, at the edge.** The SBDs now wear on their searches; until this decision they never flew. Their measures rest on the 200-mile reading; at 150 miles the medians move by one aircraft or none.
- **Japanese bombers lost go from agreeing to agreeing at the edge** (median 16 against 12). The patrol now meets raids it outnumbers (days 10 and 22 to 24) and reaches the bombers with 87 shots a run, against 25.
- **Aircraft destroyed on the ground still miss** (7 to 19 against 2 to 3). The dive bombers lose half as many on the ground as before.
- **The fatigue figures cannot fly the record's five raid days running** (days 20 to 24). The bombers launch 28, 18, 10, 2 and 1 in the median run, against the forms' 18, 22, 24, 20 and 16. So 8 or 9 of the 11 ordered raids reach the field. The early raids are too big: 41 on the first against 23 sent. Bomber sorties that reach the field total 212 against 211 to 213, right by a wrong path. Decision 0082 tables the bombers by raid day beside the forms; the fatigue round will be judged on that table.
- **Claims** are unchanged in kind (about 1.4 for each shot down).

## What this says about the order of work

1. The surface gap at Savo Island is one-sided and large. It waits for the experience rule.
2. The air gaps pull in opposite directions in the two air battles: too few American losses in the carrier battle, too many fighter losses over Guadalcanal. One set of figures serves both, so the cause is more likely in which fights happen and who is in them (escorts, patrols, which strikes fly) than in a single chance. The strike target priority rule comes before the air figures are touched, as the spec already says.
3. Claims overstate too little in every battle that measures them. The three claim chances are placeholders and are the simplest thing here to set from the record.

## At the close of step 2, Nov 13, 2026

This page reads the reports as each was made. Where step 2 ends is `docs/validation/step2-close.md`: the four battles' last reports in one table, every named miss and every agreement at the edge with its cause, and the decisions asked of the designer.
