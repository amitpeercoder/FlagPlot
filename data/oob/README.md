# Order-of-battle facts

Sourced historical facts for Flag Plot's order of battle. This is the facts layer only. Game ratings are derived from it later and are not here.

- Format and rules: `docs/research/oob/database-design.md` (decision 0015)
- Which sources may be used and how to cite: `docs/research/oob/source-policy.md`
- Where sources disagree: `docs/research/oob/conflicts.md`
- What was looked for and not found: `gaps.csv`
- Savo Island, as a scenario needs it: `docs/research/oob/savo-island.md`. The disputed facts its ships need, for one-pass approval: `savo-island-settlement.md`
- Eastern Solomons, the same: `docs/research/oob/eastern-solomons.md`. The disputed facts scenario 5 needs, for approval: `eastern-solomons-settlement.md`
- Cape Esperance (11 to 12 October 1942), the same, with the disputes first: `docs/research/oob/cape-esperance.md`
- Rabaul against Guadalcanal in the air, 21 August to 17 September 1942, for scenario 6: `docs/research/oob/rabaul-guadalcanal-air.md`
- Proposals of 2026-11-11 for the architect (cruiser floatplanes, the two aerial torpedoes, Enterprise's 20 mm guns): `docs/research/oob/proposals-2026-11-11.md`
- The record for step 2 (outcomes in scoring terms, claims against losses, ship repair, base sizes), read on 2026-11-12: `docs/research/oob/step2-record.md`. Base facts are in `place_facts.csv`, **not yet checked by `oob-check`**
- Japanese records, read on 2026-11-12 (the search orders of 24 August 1942 from a Japanese war diary, carrier losses, daily air losses over Guadalcanal): `docs/research/oob/japanese-records-2026-11-12.md`
- Japanese unit records read from the archive scans (daily air losses over Guadalcanal by unit; Eastern Solomons searches, strikes and positions): `docs/research/oob/japanese-air-records.md`
- Strike launch ranges of 1942 and aircraft lost for fuel or on the return: `docs/research/oob/strike-ranges.md`
- Aircraft lost without being shot down and what became of their crews, 1942 (ditchings by cause, crews recovered by where the aircraft came down, rescue arrangements): `docs/research/oob/ditching-and-rescue.md`
- Airmen lost and recovered over Guadalcanal by name, 21 August to 17 September 1942: `docs/research/oob/aircrew-fates.md`. The rows are in `aircrew_fates.csv`, **not yet checked by `oob-check`**
- Search distances ordered and flown in the carrier actions of 1942 and the Japanese search signals of 23 to 24 August: the last section of `docs/research/oob/air-search.md`. First actions and replacements of air groups: the last section of `docs/research/oob/experience.md`
- Torpedo tubes that bear on one broadside by class, and torpedoes fired by ship in the night actions of 1942 with the reasons tubes were not fired: `docs/research/oob/torpedoes-fired.md`
- Crutchley's instructions and Chicago's conduct at Savo Island, Scott's orders at Cape Esperance and the Japanese turn there: `docs/research/oob/orders-and-conduct.md`
- Fuel capacity from FTP 218 for four American classes, Laffey's gun count, the Fubuki class's torpedo in 1942, and alarm and first fire by ship at Savo Island: `docs/research/oob/fuel-fits-and-alarm.md`
- Rates of gun fire achieved in the night surface actions of August to November 1942 (shells, salvos, firing windows, what governed the rate), with three engagements added for the November actions and Tassafaronga: `docs/research/oob/achieved-fire-rates.md`
- What the record gives to rest an experience figure on (pilot hours, air groups' time in action, ships' time in commission and night practice, damage control): `docs/research/oob/experience.md`. Statements by navy are in `training_facts.csv`, checked by `oob-check` (columns, closed lists of entities and attributes, the form of each value and of the dates; the rule that a value has to earn preferred is not applied to it yet)
- Recorded ship passages, July to October 1942 (start, end, times and stated speed, with what makes each run usable or not): `docs/research/oob/passages.md`. The rows are in `passages.csv`, checked by `oob-check` (columns, located ends, forms of times, coordinates and speeds)
- Land forces on Guadalcanal and Tulagi, August to October 1942 (units and their men, each side's estimates of the other, men carried by sea, the attacks on the perimeter, rations), read on 2026-11-12: `docs/research/oob/land-forces.md`. The rows are in `land_units.csv`, `land_unit_facts.csv`, `lift_entities.csv` and `troop_lift_facts.csv`, which `oob-check` holds to the sourcing rule, and since decision 0093, 8 also to their entities and attributes (`lift_entities.csv`, `troop_lift_facts.csv`; the other two not yet), and in `engagement_outcomes.csv` ids 1912 to 2275
- Transports, cargo ships and destroyer-transports of 7 August 1942, the Japanese destroyers of the troop runs of August and September, and troop capacity (decision 0093, 8), read on 2026-11-14: `docs/research/oob/troop-lift.md`. **How the troop rows relate to ships:** the rows of `troop_lift_facts.csv` are about the entities of `lift_entities.csv`, which are not ships. The rows a ship's troop figure rests on are restated under the ship in `ship_facts.csv` with the same source, locator and quote and a note naming the old row: a stated capacity as `troop_capacity_men`, a load on a voyage as `troops_carried_men`. A run stays a run: a ship is tied to it by a `troop_run` row of hers, whose value is the run's key. `oob-check` now checks `lift_entities.csv` and `troop_lift_facts.csv` (entities, a closed list of attributes, whole numbers where a number is meant, and that every `troop_run` names a run); it does not check their dates or standing beyond the sourcing rule
- An outside model's research answers of 13 November 2026, checked against the sources they cite (what was verified, what the sources really say, and the rate of misattribution): `docs/research/oob/outside-research-nov-13.md`
- An outside research pass of 14 November 2026 (transports, land units, construction and shore work, marches and place positions, preparation of the landing, verdicts, submarine damage), checked against the sources it cites, with the new attributes of `land_unit_facts.csv` and `ship_facts.csv`, the rows' ids, and each Guadalcanal place's map hex: `docs/research/oob/outside-research-nov-14.md`
- A second outside research pass of 14 November 2026 (the Japanese west of the airfield and the Oka group, positions of places from the 1974 US gazetteer, the engineers and the airfield, the Kawaguchi Detachment by day, Japanese rations), checked against the sources it cites, with each new position's map hex: `docs/research/oob/outside-research-nov-14-slice.md`
- Levels of supply and convoy cycles, 1942 (the War Department's levels for the South Pacific and how they were defined, the operating level between shipments, what was on hand at Guadalcanal and how often ships came, Noumea's congestion, the Japanese drum runs and their plans in days of rations), read on 2026-11-14, with the range the rows give for a convoy's load above a base's floor: `docs/research/oob/supply-levels-and-convoy-cycles.md`
- The US side of the air campaign (losses and credits by month from the Navy's statistics, daily dispatches, claims against losses both ways): `docs/research/oob/us-side-air-losses.md`
- Notes of 10 November 2026 (research notes that propose nothing binding; most statements carry their source, and the pages that would not load are marked "unverified"):
  - Shipboard anti-aircraft fire in the South Pacific, August 1942 to February 1943 (US and Japanese guns, directors, what each carrier's fire did at Eastern Solomons): `docs/research/oob/anti-aircraft.md`
  - Damage control, August 1942 to February 1943 (what sank or saved the ships, from the Bureau of Ships damage reports and the combat narratives): `docs/research/oob/damage-control.md`
  - Surface gunnery fire control, 1942 to 1943 (night doctrine, optics, computers and director control): `docs/research/oob/gunnery-fire-control.md`
  - Radar in the Solomons, 1942 to 1943 (the US search and fire-control sets, and what is known of Japanese shipboard radar): `docs/research/oob/radar.md`
  - Sonar and anti-submarine detection, 1942 to 1943 (the sets, depth charges and ahead-thrown weapons): `docs/research/oob/sonar.md`
  - Torpedoes and their reliability, 1942 to 1943 (the US Mark 14 faults and the Japanese torpedoes): `docs/research/oob/torpedoes.md`
- Savo Island, four proposed choices for scenario 4c (the research behind decision 0019, kept for the reasoning; where the two differ the decision stands): `docs/research/oob/savo-island-choices.md`
- Air-group experience values, a proposal of 13 November 2026 (ruled that the values wait; no value is in any data file): `docs/research/oob/air-group-experience-proposal.md`
- The record on the objectives scale (a fact sheet of 13 November 2026: each ship, ground unit and base in the scenarios in the scale's terms; it proposes no points or levels): `docs/research/oob/history-on-the-scale.md`

The simulation does not read these files. `data/reference/` is what it loads.

## Rules in brief

- One row is one source's statement of one fact, with a date range, a locator, the date read, a confidence and a standing.
- A missing fact is a missing row. Nothing is guessed and nothing is a placeholder.
- A source that disagrees gets its own row. Rows are never overwritten to settle a disagreement.
- A value is `preferred` only with one row at medium or high confidence, or two agreeing sources of which one is not graded C. Otherwise it is `unconfirmed`, and the ratings layer does not read it by default.
- A dispute is settled by a person, with the reason in the note.
- `id` and `key` values are never changed or reused. Entity ids start at 1000.
- Quantities are whole numbers or decimal strings in the unit `attributes.csv` gives. The source's own figure is kept in `as_stated`.
- Nothing comes from any wargame.

## Status after several passes, counted on 2026-11-14

Counted on 2026-11-14 (it was first written on 2026-11-10; the counts are new, the prose below the tables is of that date and was not re-read). Rows were counted with a CSV-aware script (Python's `csv` module, so a field that holds a newline is one row), leaving out each file's leading `#` comment lines and its header line. A `wc -l` count is higher by those lines and by any newline inside a field.

Scope when first written: what test scenarios 4 to 6 need. The ships at Savo Island (8 to 9 August 1942) and Eastern Solomons (24 to 25 August 1942), their classes, the aircraft flown from the carriers, Henderson Field and Rabaul in August and September 1942, and the weapons and sensors they carried. The second pass covered Savo Island only. The third re-read, in a browser, two official sites that refuse automated fetches. Later passes added Cape Esperance, the air campaign, the land forces, passages, training and troop lift (see the index above).

| File | Rows | Holds |
| --- | --- | --- |
| `ships.csv`, `ship_facts.csv` | 170, 1,422 | 82 US, 85 Japanese and 3 Australian ships with build dates and fates |
| `ship_classes.csv`, `ship_class_facts.csv` | 63, 1,441 | Dimensions, speed, fuel, endurance, armour |
| `aircraft_types.csv`, `aircraft_type_facts.csv` | 23, 556 | Performance and weights |
| `weapons.csv`, `weapon_facts.csv` | 71, 481 | Guns, torpedoes, bombs, depth charges |
| `sensors.csv`, `sensor_facts.csv` | 29, 103 | Radar, sonar, and Japanese rangefinders, binoculars and searchlights |
| `air_units.csv`, `air_unit_facts.csv` | 59, 450 | Equipment, base and strength by date |
| `fits.csv` | 863 | Armament by class, radar by ship, aircraft guns and loads |
| `engagement_forces.csv` | 375 | Who was present at the battles, by formation |
| `engagement_events.csv` | 1,300 | The engagements: sightings, reports, readiness, strikes, by time |
| `engagement_positions.csv` | 548 | The engagements: stations, tracks, reported positions, sinkings |
| `engagement_outcomes.csv` | 4,055 | The engagements: hits, losses, ammunition, aircraft |
| `place_positions.csv` | 94 | Latitude and longitude of the slice's bases and landmarks, by feature. **Not yet checked by `oob-check`** |
| `land_units.csv`, `land_unit_facts.csv` | 100, 1,185 | Land units on Guadalcanal and Tulagi, strengths, landings, estimates, rations |
| `lift_entities.csv`, `troop_lift_facts.csv` | 82, 189 | What men were carried by sea: ships, groups and runs, capacities and loads |
| `passages.csv` | 186 | Recorded ship passages |
| `aircrew_fates.csv` | 118 | Named airmen lost and what became of them |
| `training_facts.csv` | 105 | Training and experience by navy and period |
| `place_facts.csv`, `places.csv` | 240, 67 | Facts about bases; names of places |
| `engagements.csv`, `attributes.csv` | 11, 140 | The named battles; the vocabulary of the facts layer |
| `sources.csv` | 266 | Every source cited |
| `gaps.csv` | 1,288 | What was not found |

Every row has a source and a locator (`sources.csv`, `gaps.csv` and the identity files, which hold no facts, excepted). About 30 values from the first pass and about 15 from the second were re-read against the cited pages. One error was found and corrected: a website's displacement figures had been converted as metric tonnes when they are long tons.

### How firm it is

The two tables count the 13,711 rows of the 18 files that carry a `confidence` column (the fact files, `fits.csv`, the four engagement files with `engagements.csv` left out, `aircrew_fates.csv`, `passages.csv` and `place_positions.csv`). Shares are rounded.

| Confidence | Rows | Share |
| --- | --- | --- |
| high | 3,079 | 22% |
| medium | 6,828 | 50% |
| low | 3,804 | 28% |

| Standing | Rows | Share |
| --- | --- | --- |
| preferred | 5,639 | 41% |
| unconfirmed | 5,920 | 43% |
| disputed | 1,779 | 13% |
| alternative | 373 | 3% |

Where the rows come from matters more than the count:

- **Wikipedia is still the largest single source: 1,424 rows, 10% of the 13,711.** All are `low` confidence; 238 of them stand `preferred`, which this pass did not check against the rule above.
- **The Naval War College analysis of Savo Island (1950) is second: 986 rows.** The second pass found a complete scan. It was read through machine-read text, which misreads some digits. Rows with a doubtful digit say so and are graded down. Its diagrams were not read.
- **Websites carry most of the class and weapon data:** combinedfleet.com (868), navypedia.org (531), navweaps.com (485), the Nishida class pages (319) and uboat.net (235). Nishida and Navypedia give no sources.
- **Two official sites load in a browser and not otherwise.** The US Navy's online ship histories and the Australian Navy's unit histories were read that way for the Savo Island ships (30 rows). They gave firm dates, casualties and Chicago's radar, and nothing on the radar of the six ships still missing. Most ship histories in the data still come from a mirror of the older printed entries. NavSource refuses the browser too.
- **No book was in hand.** Lundstrom, Frank, Loxton, Friedman, Campbell, and Lacroix and Wells would settle most of the disputes.

### Main gaps

- **Savo Island radar.** Nothing found for Wilson, Jarvis or Australia. Bagley, Patterson and Helm have only an inference that they carried some set. Vincennes, Quincy, Astoria and Chicago had sets the sources describe in use but do not all name. Canberra's is Type 271 on the Australian official history and awaits settlement.
- **Savo Island torpedoes.** Yubari's type is open. Kinugasa's, Tenryu's and Yunagi's are inferences from their tubes. Reloads are unknown for the US destroyers, Canberra and Yunagi.
- **Savo Island ammunition.** No rounds-per-gun figure for the 14 cm and 12 cm guns of Tenryu, Yubari and Yunagi.
- **Displacement.** Disputed for nearly every Savo Island class, because sources quote different conditions.
- **Japanese night optics.** That the big binoculars and searchlights existed is sourced. How many each ship carried and how far they could see is not, beyond the sighting ranges recorded for the night itself.
- **Japanese destroyers.** Most have no build dates. Their class rows lack the torpedo model.
- **US sonar.** No source found for the destroyer sets of 1942.
- **Japanese air strength at Rabaul.** Almost entirely from Wikipedia.
- **US carrier squadron strengths on 24 August.** Only a combined fighter total has a primary source.
- **B-17E and P-400 performance.**
- **Australian ships.** Dates, guns and Canberra's losses now have the Australian Navy's own history behind them. Radar and torpedo fits still rest on websites.
- **Ship-to-class assignment.** Only Yunagi's has a sourced row (`ship_class`). The rest follow the usual class names. Two sources disagree on the class name for the Bagley group of destroyers. The transports and patrol boats have no class.
- **Transports and auxiliaries** are named in `gaps.csv` and not researched. Submarines are left out.
- **Eastern Solomons.** No Japanese track from Japanese records, and no squadron strengths on the day beyond totals. Its sources keep two clocks about half an hour apart; times are entered as printed.
- **Not started:** `events.csv`. Losses are recorded as `sunk` and `fate` facts. Refits are recorded as dated fits.

### Known limits of the format, found while filling it

- **Strength by aircraft type.** A mixed unit has several `aircraft_on_hand` rows on one date, told apart only in `note`. The strength and basing attributes are therefore `many`, which switches off the conflict check for them. A `qualifier` column would fix this and would also replace the three fixed torpedo speed settings.
- **"None fitted" needs an item.** A fit row with count 0 has to name what is absent. `jp-radar-any` and `us-radar-type-unstated` are stand-in sensors for "no radar of any kind" and "a radar the source does not name". They are not real sets and have no facts.
- **Actors in engagement rows are free text** where they are not a ship: a formation, an aircraft, a commander. They are not checked.
- **A class row cannot say which ships it covers.** Where sisters differ (the 8-inch gun in the New Orleans class, the rebuilt and unrebuilt Takao class), the difference is carried by ship rows.
- **Fact row ids** are assigned at first write and never change. Files are sorted by entity, so new rows do not sit at the end.
- **The checks have a permanent home:** `crates/oob-check` (run as `cargo run -p oob-check -- data/oob`, and by `cargo test`). The throwaway script of 2026-11-10 is gone.
- **`place_positions.csv`, `place_facts.csv` and `aircrew_fates.csv` are not checked by `oob-check`.** Its source (`crates/oob-check/src`) names none of the three; the index above says the same of `place_positions.csv` and `aircrew_fates.csv`. The references and number forms of `place_positions.csv` were checked by hand once.
