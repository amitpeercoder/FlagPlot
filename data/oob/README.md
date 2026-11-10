# Order-of-battle facts

Sourced historical facts for Flag Plot's order of battle. This is the facts layer only. Game ratings are derived from it later and are not here.

- Format and rules: `docs/research/oob/database-design.md`
- Which sources may be used and how to cite: `docs/research/oob/source-policy.md`
- Where sources disagree: `docs/research/oob/conflicts.md`
- What was looked for and not found: `gaps.csv`

The simulation does not read these files. `data/reference/` is what it loads.

## Rules in brief

- One row is one source's statement of one fact, with a date range, a locator, the date read, a confidence and a standing.
- A missing fact is a missing row. Nothing is guessed and nothing is a placeholder.
- A source that disagrees gets its own row. Rows are never overwritten to settle a disagreement.
- `id` and `key` values are never changed or reused. Entity ids start at 1000.
- Quantities are whole numbers or decimal strings in the unit `attributes.csv` gives. The source's own figure is kept in `as_stated`.
- Nothing comes from any wargame.

## Status: first data set, read on 2026-11-10

Scope: what test scenarios 4 to 6 need. The ships at Savo Island (8 to 9 August 1942) and Eastern Solomons (24 to 25 August 1942), their classes, the aircraft flown from the carriers, Henderson Field and Rabaul in August and September 1942, and the weapons and sensors they carried.

| File | Rows | Holds |
| --- | --- | --- |
| `ships.csv`, `ship_facts.csv` | 122, 594 | 53 US, 66 Japanese and 3 Australian ships with build dates and fates |
| `ship_classes.csv`, `ship_class_facts.csv` | 44, 569 | Dimensions, speed, fuel, endurance, armour |
| `aircraft_types.csv`, `aircraft_type_facts.csv` | 20, 240 | Performance and weights |
| `weapons.csv`, `weapon_facts.csv` | 56, 384 | Guns, torpedoes, bombs, depth charges |
| `sensors.csv`, `sensor_facts.csv` | 15, 87 | US, British and Japanese radar; Japanese sonar and hydrophone |
| `air_units.csv`, `air_unit_facts.csv` | 45, 201 | Equipment, base and strength by date |
| `fits.csv` | 279 | Armament by class, radar by ship, aircraft guns and loads |
| `engagement_forces.csv` | 278 | Who was present at the two battles, by formation |
| `sources.csv` | 29 | Every source cited |
| `gaps.csv` | 187 | What was not found |

Every fact row has a source and a locator. A sample of about 30 values across all batches was re-read against the cited pages; none was wrong.

### How firm it is

| Confidence | Rows | Share |
| --- | --- | --- |
| high | 403 | 15% |
| medium | 1,275 | 48% |
| low | 954 | 36% |

500 rows (19%) are marked `disputed` because sources disagree.

Where the rows come from matters more than the count:

- **Wikipedia is the largest single source: 769 rows, 29%.** All are `low` confidence. They are leads to be replaced by the books Wikipedia cites.
- **Four websites carry most of the rest:** combinedfleet.com (362), navweaps.com (357), the Nishida class pages (274) and uboat.net (224). The Nishida pages give no sources and carry most Japanese class data.
- **Primary and official documents are about 440 rows:** the wartime Navy narratives, the Naval War College analysis of Savo, the older Navy ship histories, the 1943 radar manual, the wartime fuel-consumption tables, the Technical Mission to Japan reports and the two service histories of Guadalcanal.
- **The Navy's own history site would not load on the day.** Its ship histories were read from a mirror of the older printed entries. The weekly aircraft location reports, which would give US squadron strengths, were not read at all.
- **No book was in hand.** Lundstrom, Frank, Friedman, Campbell, and Lacroix and Wells would settle most of the disputes below.

### Main gaps

- **Radar by ship.** Seven Allied ships at Savo Island have a sourced radar set. Vincennes, Quincy, Australia, Hobart and six destroyers have none. Of the Japanese ships at Savo, only Tenryu has a sourced "no radar".
- **Japanese destroyers.** Most have no build dates. Their class rows lack the torpedo model.
- **Japanese night optics, rangefinders, directors and searchlights.** Nothing usable was found.
- **US sonar.** No source found for the destroyer sets of 1942.
- **Japanese air strength at Rabaul.** Almost entirely from Wikipedia.
- **US carrier squadron strengths on 24 August.** Only a combined fighter total has a primary source.
- **B-17E and P-400 performance.**
- **Australian ships.** The Navy's own site would not load. Their rows come from one website and Wikipedia.
- **Ship-to-class assignment has no source column.** It follows the usual class names. Two sources disagree on the class name for the Bagley group of destroyers. Yunagi, the transports and the patrol boats have no class.
- **Transports and auxiliaries** are named in `gaps.csv` and not researched. Submarines are left out.
- **Not in this pass:** `events.csv`. Losses are recorded as `sunk` and `fate` facts. Refits are recorded as dated fits.

### Known limits of the format, found while filling it

- **Strength by aircraft type.** A mixed unit has several `aircraft_on_hand` rows on one date, told apart only in `note`. The strength and basing attributes are therefore `many`, which switches off the conflict check for them. A `qualifier` column would fix this and would also replace the three fixed torpedo speed settings.
- **Fact row ids** were assigned at first write. From now on a new row takes the next free number and existing rows keep theirs.
- **The checks were run with a throwaway script** that is not in the repo. They need a permanent home in `sim-data` before the next data pass.
