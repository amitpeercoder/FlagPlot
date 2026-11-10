# Order-of-battle facts

Sourced historical facts for Flag Plot's order of battle. This is the facts layer only. Game ratings are derived from it later and are not here.

- Format and rules: `docs/research/oob/database-design.md` (decision 0015)
- Which sources may be used and how to cite: `docs/research/oob/source-policy.md`
- Where sources disagree: `docs/research/oob/conflicts.md`
- What was looked for and not found: `gaps.csv`
- Savo Island, as a scenario needs it: `docs/research/oob/savo-island.md`

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

## Status after three passes, read on 2026-11-10

Scope: what test scenarios 4 to 6 need. The ships at Savo Island (8 to 9 August 1942) and Eastern Solomons (24 to 25 August 1942), their classes, the aircraft flown from the carriers, Henderson Field and Rabaul in August and September 1942, and the weapons and sensors they carried. The second pass covered Savo Island only. The third re-read, in a browser, two official sites that refuse automated fetches.

| File | Rows | Holds |
| --- | --- | --- |
| `ships.csv`, `ship_facts.csv` | 122, 612 | 53 US, 66 Japanese and 3 Australian ships with build dates and fates |
| `ship_classes.csv`, `ship_class_facts.csv` | 44, 668 | Dimensions, speed, fuel, endurance, armour |
| `aircraft_types.csv`, `aircraft_type_facts.csv` | 20, 240 | Performance and weights |
| `weapons.csv`, `weapon_facts.csv` | 60, 430 | Guns, torpedoes, bombs, depth charges |
| `sensors.csv`, `sensor_facts.csv` | 26, 103 | Radar, sonar, and Japanese rangefinders, binoculars and searchlights |
| `air_units.csv`, `air_unit_facts.csv` | 45, 201 | Equipment, base and strength by date |
| `fits.csv` | 393 | Armament by class, radar by ship, aircraft guns and loads |
| `engagement_forces.csv` | 278 | Who was present at the two battles, by formation |
| `engagement_events.csv` | 120 | Savo Island: sightings, warnings, readiness, open fire, by time |
| `engagement_positions.csv` | 261 | Savo Island: patrol stations, Mikawa's track, sinking positions |
| `engagement_outcomes.csv` | 180 | Savo Island: hits, losses, rounds and torpedoes fired |
| `sources.csv` | 40 | Every source cited |
| `gaps.csv` | 339 | What was not found |

Every row has a source and a locator. About 30 values from the first pass and about 15 from the second were re-read against the cited pages. One error was found and corrected: a website's displacement figures had been converted as metric tonnes when they are long tons.

### How firm it is

| Confidence | Rows | Share |
| --- | --- | --- |
| high | 691 | 20% |
| medium | 1,665 | 48% |
| low | 1,130 | 32% |

| Standing | Rows | Share |
| --- | --- | --- |
| preferred | 2,095 | 60% |
| unconfirmed | 707 | 20% |
| disputed | 647 | 19% |
| alternative | 37 | 1% |

Where the rows come from matters more than the count:

- **Wikipedia is still the largest single source: 795 rows, 24%.** All are `low` confidence and none is `preferred` on its own.
- **The Naval War College analysis of Savo Island (1950) is second: 465 rows.** The second pass found a complete scan. It was read through machine-read text, which misreads some digits. Rows with a doubtful digit say so and are graded down. Its diagrams were not read.
- **Websites carry most of the class and weapon data:** combinedfleet.com (401), navweaps.com (383), the Nishida class pages (285), uboat.net (224) and navypedia.org (76). Nishida and Navypedia give no sources.
- **Two official sites load in a browser and not otherwise.** The US Navy's online ship histories and the Australian Navy's unit histories were read that way for the Savo Island ships (30 rows). They gave firm dates, casualties and Chicago's radar, and nothing on the radar of the six ships still missing. Most ship histories in the data still come from a mirror of the older printed entries. NavSource refuses the browser too.
- **No book was in hand.** Lundstrom, Frank, Loxton, Friedman, Campbell, and Lacroix and Wells would settle most of the disputes.

### Main gaps

- **Savo Island radar.** Nothing found for Wilson, Jarvis or Australia. Bagley, Patterson and Helm have only an inference that they carried some set. Vincennes, Quincy, Astoria and Chicago had sets the sources describe in use but do not all name. Canberra's is settled as Type 271.
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
- **Eastern Solomons** has no events, positions or outcomes yet.
- **Not started:** `events.csv`. Losses are recorded as `sunk` and `fate` facts. Refits are recorded as dated fits.

### Known limits of the format, found while filling it

- **Strength by aircraft type.** A mixed unit has several `aircraft_on_hand` rows on one date, told apart only in `note`. The strength and basing attributes are therefore `many`, which switches off the conflict check for them. A `qualifier` column would fix this and would also replace the three fixed torpedo speed settings.
- **"None fitted" needs an item.** A fit row with count 0 has to name what is absent. `jp-radar-any` and `us-radar-type-unstated` are stand-in sensors for "no radar of any kind" and "a radar the source does not name". They are not real sets and have no facts.
- **Actors in engagement rows are free text** where they are not a ship: a formation, an aircraft, a commander. They are not checked.
- **A class row cannot say which ships it covers.** Where sisters differ (the 8-inch gun in the New Orleans class, the rebuilt and unrebuilt Takao class), the difference is carried by ship rows.
- **Fact row ids** are assigned at first write and never change. Files are sorted by entity, so new rows do not sit at the end.
- **The checks were run with a throwaway script** that is not in the repo. They need a permanent home in `sim-data` before the next data pass.
