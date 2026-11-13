# 0070: Rates of fire as achieved, not as designed

Status: settled by the architect, to be reviewed. Nov 13, 2026. Built; see "As built". A change to the derivation (`oob-derive` and `data/derivation/rules.toml`), by its written rule and from rows. No rule of the simulation changes, no figure of `surface_combat.toml` moves, and the frozen step 1 figures are untouched. The hand derivation below was written before any code was delegated.

## The gap

A combat round is five minutes. In it a ship fires guns x rate x 5 shells (decision 0014), at the rate the derivation gives its class: the sourced practical rate of its main gun, or a designed rate read by exception (decisions 0020 and 0023). No ship fired like that in a night action. At Savo Island, on the figures of Nov 13, 2026, Tenryu and Yubari fire a median 900 shells a run against a record of 176 (Tenryu 80 and Yubari 96 of 5.5-inch, `engagement_outcomes` row 3). They were at battle readiness, so neither the readiness rule (decision 0068) nor experience (decision 0064) touches their count: what is wrong is under them. The step 2 spec puts this before any surface figure is tuned ("Tuning", item 6; "Rule work that comes before tuning").

The research of pull request 167 (`docs/research/oob/achieved-fire-rates.md`) gives what ships fired and over how long, for the night actions of August to November 1942. Two things in it decide the form of the rule:

- **The achieved rate is a rate, not a share.** As a share of each gun's designed rate the cases run from 0.13 to 0.63. As rounds a gun a minute, over a stated span of firing, they cluster by the kind of battery: about 1 to 1.5 for heavy guns fired in salvos, 8-inch and 16-inch alike, and about 2.5 for destroyer guns. What governed the rate was fire control, spotting, target changes, flash and the radar, not loading (the research's count of 336 statements of what limited fire).
- **The Japanese cruisers at Savo Island fired 15 to 32 rounds a gun whatever the calibre,** and the 14 cm guns fewer a gun than the 8-inch. No share of a designed rate can give that, since the 14 cm's designed rate is two and a half times the 8-inch's.

A flat share was measured and set aside: at 0.44 of the designed rate Tenryu and Yubari still fire a median 6 hundreds, three times the record.

## Decisions

### The rule

1. **A class's rate of fire in action is the lesser of its sourced rate and the achieved rate for its kind of battery.** The sourced rate is what the derivation read before (decision 0020: the practical rate, or a designed rate named by exception). A class with no sourced rate takes its kind's achieved rate, and the missing rate is a gap in `gaps.csv`. A kind with no case keeps its sourced rate, and that is a gap too.
2. **The achieved rate of a kind is the median of its cases.** With an even number of cases it is the mean of the two in the middle. Every figure is an exact decimal of six places; a division truncates at the sixth place, as everywhere in the derivation.
3. **A case** is a named ship for which rows state the shells fired by one battery and a span of firing of five minutes or more, with the number of guns from a row. Its rate is shells / (guns x minutes). The span is from the first opening of fire to the last cease fire, pauses included, as the source states it: either a duration row of the ship or two event rows, an opening and a cease, whose times the tool subtracts. The guns are the main battery fit of the ship or its class on the date of the engagement, read by the derivation's usual rule.
4. **Not a case:**
   - a burst under five minutes: a round is five minutes, and a rate kept for one or two minutes is not what a ship keeps up through a round;
   - a ship whose span can be read two ways (Minneapolis at Tassafaronga, Honolulu there);
   - a ship for which no row states the guns, or the shells of the whole battery (Barton);
   - a total with no span behind it (Wilson's 212 at Savo Island; South Dakota, Sterett, Drayton, Atago);
   - a count of one kind of shell where the ship fired more than one (Takao's 36 armour-piercing between 0102 and 0114 on 15 November, row 2502, a compiler's statement: her sister Atago fired armour-piercing and common shell that night);
   - a secondary battery: the model fires main guns only.
5. **Each case is written in `rules.toml` as an exception** (decision 0020, item 3): the ship, its class, the rows it reads, the reason, and the date. The case rows are `unconfirmed` (the research settled nothing), and only an exception lets the derivation read them. The exceptions are kept in one block of the rules file.
6. **The kinds of battery:**
   - **(a) Main battery fired in salvos,** cruisers and larger. One rate for 8-inch and 16-inch alike: no battleship is in the slice, fewer kinds is better, and Washington's figure lies inside the cruisers' spread. Whether a battleship needs its own kind is open for when one arrives.
   - **(b) Destroyer main battery.**
   - **(c) Continuous rapid fire,** only for a class that a row names as firing so, named by exception with the row: the US 6-inch cruisers (row 2541, the Battle Experience bulletin on "our light cruisers which used rapid continuous fire" at Cape Esperance; Helena's own report, row 2974; Honolulu's, row 2906; Boise, row 2523).
   - **Salvo fire is the default for a cruiser's main battery.** Tenryu and Yubari fall under (a) because no row says otherwise. That is the default of the rule, not a finding about how they fired.
   - The rows of Admiral Scott's memorandum that told the US heavy cruisers to use continuous fire against small ships at short range (2291, 2514) do not move the heavy cruisers to (c). They are an order for one situation, and Salt Lake City's own count at Cape Esperance is 30 salvos (row 1704).
7. **Row 432 is not read for Tenryu.** Its note says it is the rate of the twin mount, Yubari's; Tenryu's four guns are in single mounts. Reading it for her was a misreading. Tenryu has no sourced rate, so she takes the rate of kind (a). The missing rate was already a gap: `gaps.csv` has "a numeric practical rate for Tenryu single mounts and Yubari", and no new row is needed. Yubari still reads row 432, by the exception of Nov 10.

### Which rows are sources

Savo Island and Cape Esperance rows are sources. A derivation reads every row that states the quantity, and leaving rows out is a choice of its own. What a ship fired in a stated span is a fact about the ship, not a battle's result worked backward. The check that spec "Tuning" item 6 names is Tenryu and Yubari, and there is no case for them, so the check stays independent.

**But it is circular elsewhere, and this says so plainly.** Chokai and Kako are in the Savo Island measure of all Japanese shells, and two of the four cases of kind (a) are theirs. **That measure is no longer an independent check of the heavy cruisers' rate.** Salt Lake City and Helena are Cape Esperance ships, and Cape Esperance is the battle the surface figures are checked on. The only case of kind (c) is Helena's there.

Each kind's median, as used and with the rows of those two battles left out:

| Kind | Median used | Without Savo Island and Cape Esperance |
| --- | --- | --- |
| (a) salvo | 1.128571 (four cases) | 1.190476 (Washington alone) |
| (b) destroyer | 2.444444 (Asagumo) | 2.444444 (the same case) |
| (c) continuous | 1.487179 (Helena at Cape Esperance) | no case |

For (a) the two differ by 0.06. That is well inside the span of cruiser rates, 0.8 to 1.2, over which Tenryu and Yubari's shells did not leave the record when measured (Nov 13, scratch batches). For (c), nothing is left once Cape Esperance is taken out. No ship of Savo Island or the Eastern Solomons has a class of kind (c), so it moves nothing in this change. When Cape Esperance is derived, its US 6-inch cruisers' rate will rest on Helena's own firing in that battle.

## The cases

| Kind | Ship, battle | Shells | Span | Guns | Rate | Rows |
| --- | --- | --- | --- | --- | --- | --- |
| (a) | Chokai, Savo Island | 302 (her own report) | 20 min, from opening gunfire to the end, fire held between targets | 10 | 302 / 200 = **1.51** | outcomes 3131, 3132; fits 71 |
| (a) | Kako, Savo Island | 192 armour-piercing (her own report; the 20 cm row of her expenditure) | 30 min ("fired over thirty minutes") | 6 | 192 / 180 = **1.066666** | outcomes 3138, 3140; fits 301 |
| (a) | Salt Lake City, Cape Esperance | 296 in 30 salvos | 29 min, 2347 to 0016 with pauses | 10 | 296 / 290 = **1.020689** | outcomes 1704, 1638; fits 669, 670 |
| (a) | Washington, 15 November | 75 of 16-inch | 7 min, 0100 to 0107, with a check of 0102.30 to 0104 inside it | 9 | 75 / 63 = **1.190476** | outcomes 2645; events 860, 863; fits 251 |
| (b) | Asagumo, 13 November | 88 (her own report) | 6 min ("about six minutes") | 6 | 88 / 36 = **2.444444** | outcomes 3146, 3147; fits 147 |
| (c) | Helena, Cape Esperance | 580 of 6-inch (her own report) | 26 min, 2346 to 0012 | 15 | 580 / 390 = **1.487179** | outcomes 2980; events 1031, 1034; fits 704 to 706 |

Medians: (a) the middle two are 1.066666 and 1.190476, so (1.066666 + 1.190476) / 2 = 2.257142 / 2 = **1.128571**. (b) **2.444444**. (c) **1.487179**.

Notes on the cases, settled by the architect:

- **Washington is not in `ships.csv`.** The engagement rows name her `washington`. Her class, the North Carolina class, is given in the case, and her nine guns come from that class's fit row 251. Every other ship's class, but Yunagi's, is also a convention of the ship list and not a sourced row (`data/oob/README.md`, "Main gaps").
- **Washington's 75 shells are a window, not her total for the night.** Her other window, 42 shells from 0016 to 0019, is three minutes and is not a case. The two are not summed, since that would be a span the source does not state.
- **Helena's 580 against 560.** The 560 is a veterans' pamphlet whose columns are matched to actions by their order only (row 3001, low). Her own report's 580 is read. With 560 her rate would be 1.435897.
- **Kako's 30 minutes is a lower bound of her span,** so her rate is an upper bound. Her disputed gun rows disagree on the type of gun, not the six; row 301 is preferred.

Left out, and why:

| Ship | Why |
| --- | --- |
| Minneapolis, Tassafaronga | 95 shells. The span reads two ways: to the torpedo hit at 2327 (7 min, 1.507936) or to the power failure at about 2333 (13 min, 0.811965) |
| Honolulu, Tassafaronga | 244 shells from about 2325. Her report checks fire at 2331, her war diary ceases at 2330 (events 1016, 1020) |
| Takao, 15 November | One kind of shell only, from a compiler (row 2502). Read as a case it would give 36 / 120 = 0.3, and the median of (a) would be 1.066666 |
| Helena, 13 November; Fletcher; Washington 0016 to 0019 and her bursts of 2 min 39 s and 2 min 45 s; Sterett's 13 salvos | Bursts under five minutes |
| Barton, 13 November | About 60 from two guns and about 10 a gun from the others; no row says how many others |
| Wilson at Savo Island; South Dakota; Sterett; Drayton; Atago | A total with no span the source states |
| Tenryu, Yubari, Yunagi | No case: Tenryu's one minute on Wilson, no count for Yunagi, no span for Yubari |

## The hand derivation

What the rule gives the worked classes. "A round" is the shells of one five-minute round at full manning: rate x guns x 5.

| Class | Ship | Guns | Sourced rate | Kind | Rate before | Rate after | A round before | A round after |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Tenryu class | Tenryu | 4 | none (row 432 is not read) | (a) | 6 | 1.128571 | 120 | 22.57142 |
| Yubari class | Yubari | 6 | 6 (row 432, by exception) | (a) | 6 | 1.128571 | 180 | 33.85713 |
| Takao class | Chokai | 10 | 2.4 (row 431, by exception) | (a) | 2.4 | 1.128571 | 120 | 56.42855 |
| Aoba class | Aoba, Kinugasa | 6 | 2.4 (row 431) | (a) | 2.4 | 1.128571 | 72 | 33.85713 |
| New Orleans class | Vincennes, Quincy, Astoria | 9 | 3 (row 358, designed, by exception) | (a) | 3 | 1.128571 | 135 | 50.785695 |
| County class | Canberra | 8 | 3.5 | (a) | 3.5 | 1.128571 | 140 | 45.14284 |
| Benham class | Wilson | 4 | 15 | (b) | 15 | 2.444444 | 300 | 48.88888 |
| Bagley class | Bagley, Patterson, Helm | 4 | 15 | (b) | 15 | 2.444444 | 300 | 48.88888 |
| Kamikaze class | Yunagi | 4 | 5.5 (row 44, designed, by exception) | (b) | 5.5 | 2.444444 | 110 | 48.88888 |

Every class at Savo Island now fires at its kind's achieved rate, since every sourced rate is above it. A force not ready fires half of that (decision 0068), as before.

At the Eastern Solomons the surface weapons are not modelled (decision 0029, 4); the classes' rates are written as 0 there, and nothing changes.

## Why

- **A rate, not a share,** because the rows cluster as a rate and scatter as a share, and because a share cannot put the 14 cm below the 8-inch, which the Savo Island record has.
- **The lesser of the two,** because the sourced rate is what the gun can do and the achieved rate is what was done with it. Neither was exceeded in any case read.
- **A median,** because the cases are few and one odd case should not carry a kind.
- **Five minutes,** because that is a round.
- **Exceptions, not a new standing,** because the rows are one ship's statement each and stay `unconfirmed`. The rules file names each one with its reason, as for any other weak row the derivation reads.

## What it leaves out

- **Secondary batteries.** The model fires main guns only. The Japanese shells measure at Savo Island counts every calibre, about 650 shells of 4.7-inch and 5-inch among them; the spec session has been told.
- **Fight length and how many rounds a ship fires.** These are rules of the simulation, untouched here. As rates fall, fewer hits land early and fights run longer; that is for the breaking-off mechanism, last in the order of tuning.
- **A battleship kind,** until a battleship is in a scenario.
- **The chance to hit.** With fewer shells, hits fall too. That is the next mechanism to tune, and it can be read cleanly once the shells are right.

## What it costs

- **State and hash:** none in `sim-core`. The live figures move, so the run baselines, the goldens and the recorded games that follow the live figures are remade (spec, "Tests while figures move", item 4).
- **The tool:** a reader for engagement outcome and event rows in `oob-derive`, the cases and the kinds in `rules.toml`, and the cases, kinds and medians in the generated files' headers and in the sources file beside the rate.
- **The frozen figures** (`data/reference/step1`) are a copy. The tool does not write them, and no hand calculation moves.

## Rule card

No simulation rule. The tool reads `data/oob` (now including `engagement_outcomes.csv` and `engagement_events.csv`, for the named cases only) and `data/derivation/rules.toml`, and writes `gun_rounds_per_min` in the derived ship class files of `data/reference`. The simulation reads `data/reference` as before.

## As built

Built on Nov 13, 2026.

**The tool.**
- `crates/oob-derive/src/fire_rate.rs` reads the `[fire_rate]` block of `rules.toml`. The block has the cases, the continuous classes, and the one class that does not read its sourced rate.
- It checks each case against `engagement_outcomes.csv`, `engagement_events.csv`, `engagements.csv` and the fits, and works the medians.
- A case's guns are read on its engagement's first date. A span from events is the difference of the two events' times.
- `rate_in_action` in `derive.rs` takes the lesser of the sourced and the achieved rate.
- The generated file's header lists the kinds, their medians and every case. The sources file gives the case rows beside the rate, with a note naming the two rates compared.
- With no `[fire_rate]` block the tool does exactly what it did before. Its earlier fixture tests pass unchanged.
- A problem in the block stops every list that reads the rules file, the Eastern Solomons and Rabaul included.
- The case rates and medians match the hand derivation above to the last place. `crates/oob-derive/tests/fire_rate.rs` asserts them on the real facts, with fixture tests in `tests/derive.rs`.

**What changed in the data.**
- `ship_classes_savo_island.csv` was re-derived. Only `gun_rounds_per_min` moved: 1.128571 for the nine cruiser rows and 2.444444 for the four destroyer rows.
- The Eastern Solomons and Rabaul derivations are byte-identical. `cape_esperance.toml` derives only experience, so it is unchanged.
- `data/reference/step1` is untouched. No hand-calculation test moved.
- The run baselines of scenario 4 (`crates/sim-cli/tests/run_baseline/04_savo_island.*`) were remade. No other scenario reads the Savo Island classes. No recorded game moved.

**Savo Island, 1,000 runs, before and after.** Both batches were run on the same engine, the before on the reference files of `origin/main` (`--reference`). They were validated against `data/validation/savo_island.toml` as pull request 169 left it: the shells measures count the main battery only. No report was committed; `docs/validation` belongs to the spec session.

| Measure | Record | Before: median, 5th to 95th, verdict | After: median, 5th to 95th, verdict |
| --- | --- | --- | --- |
| Allied cruisers sunk | 4 | 0, 0 to 2, miss | 0, 0 to 1, miss |
| Southern group cruisers sunk | 1 | 0, 0 to 2, agrees | 0, 0 to 1, agrees |
| Northern group cruisers sunk | 3 | 0, 0 to 1, miss | 0, 0 to 0, miss |
| Allied destroyers sunk | 0 | 3, 1 to 4, miss | 2, 1 to 4, miss |
| Japanese ships sunk | 0 | 1, 0 to 3, agrees | 0, 0 to 2, agrees |
| Japanese ships with propulsion out | 0 | 0, agrees | 0, agrees |
| Gun hits on Allied ships | 153 to 219 | 121, 37 to 175, at the edge | 72, 21 to 101, miss |
| Gun hits on Japanese ships | 10 to 17 | 53, 2 to 159, miss | 19, 2 to 55, agrees |
| Torpedo hits on Allied ships | 4 to 9 | 3, agrees | 3, agrees |
| Torpedo hits on Japanese ships | 0 | 2, at the edge | 3, at the edge |
| Japanese gun hits a thousand shells | 81 to 117 | 50, miss | 52, miss |
| Allied gun hits a thousand shells | 21 to 36 | 38, agrees | 33, agrees |
| Japanese main battery shells, hundreds | 10 to 11 | 24, 10 to 32, miss | 13, 5 to 18, agrees (207 runs) |
| Allied main battery shells, hundreds | 3 | 13, 1 to 32, miss | 5, 1 to 12, agrees (119 runs) |
| Southern group shells, hundreds | 0 | 4, 0 to 13, agrees | 0, 0 to 4, agrees (527 runs) |
| Northern group shells, hundreds | 3 | 8, 1 to 24, miss | 4, 1 to 9, agrees (150 runs) |
| **Tenryu and Yubari shells, hundreds** | **1** | **9, 4 to 15, miss (0 runs)** | **2, 1 to 4, agrees at the edge (80 runs)** |
| Minutes of action | 20 to 30 | 25, 15 to 50, agrees | 35, 20 to 65, agrees |
| Surface fights by day | 0 | 0, agrees | 0, agrees |
| Allied ships claimed sunk | 0 | 1, agrees | 0, agrees |
| Japanese claims for each hundred sunk (reported) | 350 | 120 | 120 |

Judged measures, before: 9 agree, 2 at the edge, 9 miss. After: 13 agree, 2 at the edge, 5 miss.

**The check of spec "Tuning" item 6: Tenryu and Yubari agree, at the edge.**
- **The runs, in hundreds:**

  | Hundreds | 0 | 1 (the record) | 2 | 3 | 4 | 5 |
  | --- | --- | --- | --- | --- | --- | --- |
  | Runs of 1,000 | 20 | 80 | 513 | 336 | 41 | 10 |

  The mean is 288 shells against a record of 176. Before, the middle run was 900 and no run held the record.
- **Why it stays about one and a half times the record,** read from the batch:
  - A round of full fire is now 56.4 shells for the two ships, and they fire about five rounds a run.
  - That is because fights last longer as fewer hits land: night rounds rise from 4.2 to 6.4 a run, and the minutes of action from a median of 25 to 35.
  - In the record they fired about three rounds' worth. They were at the tail of the column, and Tenryu was on Wilson for about a minute.
  - Five minutes is the smallest unit of fire. Every ship that sights fires for the whole round, so a ship that fired for one minute of a round is charged five.
- **Not the cause:** the median of kind (a). With the cruiser rate at 0.8 instead of 1.128571 (scratch batches) the middle run is still 2 hundreds.
- **Nothing was moved in response.** The rule, the kinds and the cases are as settled.

**Against the scratch variant C of the study.**
- Variant C used 1.05 for cruisers and 2.5 for destroyers, round numbers set from the rows before the study's batch. The built rule gives 1.128571 and 2.444444.
- The built batch is within a few per cent of C on every measure. Tenryu and Yubari average 288 against 278. The Japanese heavy cruisers average 832 against 800. The northern group averages 476 against 480, and the southern group 149 against 151.
- The differences are the rates and nothing else.

**What the batch shows for what comes next:**
- **Gun hits on Allied ships now miss low:** 72 against 153 to 219. Japanese hits for each thousand shells, 52 against 81 to 117, missed before and still do. With the shells right, that is the chance to hit, the next mechanism in the order of tuning.
- **The minutes of action** stay inside the record, but the middle run is now at its top. That is the breaking-off mechanism.
- **The Japanese main battery shells measure agrees,** but it is not an independent check of the heavy cruisers' rate: two of the four salvo cases are Chokai and Kako, whose shells it counts.
