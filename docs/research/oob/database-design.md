# Order-of-battle database: design proposal

Status: approved by Amit on 2026-11-10 as the working format and recorded as decision 0015. The standing rules and engagement records added in the Savo Island pass await his review.

Scope: the first slice only (South Pacific, August 1942 to February 1943). The design is sized so the full order of battle can use it in step 5 without a change of format.

## Summary

- Two layers. **Facts** are sourced historical statements. **Ratings** are game values derived from facts by written rules. Only facts are filled now.
- Facts live in `data/oob/` as CSV files. The CSV files are the only source of truth. A SQLite file may be generated from them for queries, and is never committed.
- Every fact is one row: one entity, one attribute, one value, one date range, one source. A second source that disagrees is a second row. Nothing is overwritten to settle a disagreement.
- Every entity has two identifiers: a whole-number `id` for the simulation and a readable `key` for people and diffs. Neither ever changes or is reused.
- A missing fact is a missing row. Nothing is guessed, and there are no placeholders in this layer.

## The two layers

| | Facts layer | Ratings layer |
| --- | --- | --- |
| Holds | What a source says was true: "Mark 15 torpedo, warhead 494 lb, source X page Y" | What the game uses: "torpedo effect 37" |
| Comes from | A cited source | Facts, through a documented rule |
| Changes when | A better source is found | A rule is tuned in step 2 |
| Filled now | Yes, for scenarios 4 to 6 | No. The combat and detection rules do not exist yet |
| Lives in | `data/oob/` | Generated into `data/reference/`, see "Feeding the simulation" |

The split matters for three reasons. Tuning never touches the historical record. A rating can always be opened to show the facts and the rule behind it, which pillar 4 requires. And a rule change re-derives every rating at once instead of needing a hand edit per ship.

A judgement such as "crew quality" or "radar was poorly used" is not a fact in this sense. It goes in the research notes, and later becomes a rating with its rule written down.

## Entities

Each entity type has an identity file: one row per thing, holding only what defines it. Everything else about it is a fact row.

| File | One row per | Identity fields beyond `id`, `key`, `name` |
| --- | --- | --- |
| `ship_classes.csv` | Ship class | `nation`, `type` |
| `ships.csv` | Individual ship | `nation`, `class` (key), `hull_number` (if the navy used them) |
| `aircraft_types.csv` | Aircraft model and variant, for example A6M2 Model 21 | `nation`, `designation`, `role` |
| `weapons.csv` | Gun, torpedo, bomb, depth charge, by mark or type | `nation`, `kind` |
| `sensors.csv` | Radar, sonar, hydrophone, optical director or rangefinder, by model | `nation`, `kind` |
| `air_units.csv` | Squadron, kōkūtai, or carrier air group component | `nation`, `service`, `parent` (key, optional) |
| `places.csv` | Airfield, anchorage or base named in a fact | `region` |
| `engagements.csv` | Named battle or operation used to group an order of battle | `date_from`, `date_to` |
| `sources.csv` | Source, see "Sources" | |

Notes:

- `places.csv` is a list of names for facts to point at. It carries no map data. It will be matched to map bases by `key` when the real map exists.
- A ship's class is identity, not a dated fact. A ship rebuilt into something else (a seaplane tender converted to a carrier) gets a dated fact `reclassified_as` and, if needed, a second class.
- Japanese names use Hepburn romanisation without macrons in `name`. The native form is a fact (`name_native`), as are other names and Allied reporting names (`alias`).

### Identifiers

| Field | Form | Used by | Rule |
| --- | --- | --- | --- |
| `id` | Whole number, unique within the file, next free number | The simulation, saved games, generated reference files | Never changed, never reused, even if the row is retired |
| `key` | Lower-case text, unique within the file, for example `ca-44-vincennes`, `chokai`, `a6m2-21`, `sg-radar` | Every other file in `data/oob/` | Never changed, even if the spelling is later thought wrong. Fix `name` instead |

Two identifiers because the two readers need different things. The simulation needs compact integers that match its existing entity IDs. A reviewer reading a diff needs `chokai,main_battery_calibre_mm,203`, not `17,41,203`.

A row entered by mistake is not deleted. It gets `retired` in a `status` column with a note, so its `id` and `key` stay taken.

## Facts

Each entity type has a facts file beside its identity file: `ship_class_facts.csv`, `ship_facts.csv`, and so on. One row is one source's statement of one value.

| Column | Meaning |
| --- | --- |
| `id` | Whole number, unique within the file, never reused. Lets a rating cite the exact rows it was derived from |
| `entity` | `key` of the entity |
| `attribute` | A name from `attributes.csv` |
| `value` | Whole number, decimal string, date, entity key, or short text, as the attribute defines |
| `as_stated` | The figure as the source prints it, when `value` was converted, for example `8 in` or `34,000 yd` |
| `valid_from` | First day the value held. Empty means from the entity's start |
| `valid_to` | First day it no longer held. Empty means open-ended |
| `source` | `key` in `sources.csv` |
| `locator` | Page, table, section, or full URL of the page read |
| `accessed` | Date we read it. Required for online sources |
| `confidence` | `high`, `medium` or `low`, see below |
| `basis` | `stated` if the source says it directly. `inferred` if we reasoned to it, with the reasoning in `note` |
| `standing` | `preferred`, `unconfirmed`, `alternative` or `disputed`, see "Conflicting sources" |
| `note` | Short. Why this row is `inferred`, `low` or not preferred |

### Attributes

`attributes.csv` is the vocabulary. A fact row may only use an attribute listed there.

| Column | Meaning |
| --- | --- |
| `key` | Attribute name, with its unit as a suffix: `max_speed_kn`, `warhead_kg`, `commissioned` |
| `applies_to` | Entity type |
| `value_type` | `integer`, `decimal`, `date`, `text`, or `ref:<entity type>` |
| `unit` | The one unit every value of this attribute is stored in |
| `cardinality` | `one` (a single value at any date) or `many` (for example `alias`) |
| `description` | What exactly is measured, including the condition: "at standard displacement", "at 45° elevation" |

Units are fixed per attribute, so two sources can be compared directly. The proposed set is the units the simulation and most sources already use:

| Quantity | Unit |
| --- | --- |
| Speed | knots |
| Ship endurance, aircraft range | nautical miles |
| Gun and torpedo range, altitude, ship dimensions | metres |
| Calibre, armour thickness | millimetres |
| Weapon and aircraft weights | kilograms |
| Ship displacement, fuel | long tons, with the condition (standard, full load) in the attribute name |
| Radar and sonar detection range | nautical miles, with the target and condition in the attribute name |

A converted value keeps the source's own figure in `as_stated`, so a conversion error can be found later.

The first attribute list is written with the first data set, driven by what scenarios 4 to 6 need, and grows as sources are read. An attribute is added when a source gives a fact worth keeping, not in advance.

### Dates

- Dates are `YYYY-MM-DD`. Where a source gives only a month or year, the date is written `YYYY-MM` or `YYYY`. The precision is then visible and nothing is invented.
- `valid_from` is inclusive and `valid_to` is exclusive, so consecutive ranges do not overlap.
- An event with a single date, such as `commissioned` or `sunk`, is a fact with `value_type` `date` and no range.
- "True on this date, limits unknown" is written with `valid_from` and `valid_to` both set to that one date and a note. This is common: a source confirms a radar set aboard at Savo without saying when it was fitted.

## Fits: what was mounted on what

Weapons and sensors on ships and aircraft are relations, not single values, so they have their own file, `fits.csv`. This is where most time-dated change lives.

| Column | Meaning |
| --- | --- |
| `id` | Whole number, as for facts |
| `carrier_type` | `ship_class`, `ship` or `aircraft_type` |
| `carrier` | `key` of the class, ship or aircraft type |
| `slot` | What the item is there for: `main_battery`, `secondary_battery`, `heavy_aa`, `light_aa`, `torpedo_tubes`, `torpedo_reloads`, `depth_charges`, `air_search_radar`, `surface_search_radar`, `fire_control_radar`, `main_director`, `sonar`, `aircraft`, `fixed_guns`, `bomb_load`, and so on |
| `item_type` | `weapon`, `sensor` or `aircraft_type` |
| `item` | `key` of the weapon, sensor or aircraft type |
| `count` | How many: barrels, tubes, sets, aircraft |
| `arrangement` | Optional short text: `3x3`, `2x4 centreline` |
| `valid_from`, `valid_to`, `source`, `locator`, `accessed`, `confidence`, `basis`, `standing`, `note` | As for facts |

Rules:

- **A class fit is the fit as built.** It applies to every ship of the class unless the ship has its own rows.
- **A ship's rows replace the class rows slot by slot.** If a ship has any row for `light_aa` covering a date, the class's `light_aa` rows do not apply to that ship on that date. Other slots still come from the class.
- **Removal is recorded.** A slot emptied by a refit gets a ship row with `count` 0, so "removed" and "not researched" stay different.
- **Aircraft loads are fits too.** An aircraft type's guns and its typical bomb or torpedo loads use the same file. Alternative loads share a slot and are told apart in `arrangement`.

Radar is the case that shaped this. A class row cannot say which ships had SG radar in August 1942. A ship row with a date can, and it can be marked `inferred` when the only evidence is a photograph or a later action report.

## Air units

An air unit's equipment, strength and base all changed week to week, so they are dated facts in `air_unit_facts.csv`:

| Attribute | Value |
| --- | --- |
| `equipped_with` | `key` of an aircraft type. `cardinality` is `many`, since mixed units were common |
| `based_at` | `key` of a place or a ship |
| `aircraft_on_hand`, `aircraft_operational` | Whole number on a date, with the aircraft type in `note` when the unit is mixed |

Strength figures are almost always "on this date", so most rows use the single-date form. The gaps between them stay gaps. The simulation's starting state for a scenario reads the row nearest the start date, and says which.

## Events

`events.csv` records dated happenings that explain why facts change: refits, damage, losses, transfers, renamings.

| Column | Meaning |
| --- | --- |
| `id`, `key` | As for entities |
| `entity_type`, `entity` | What it happened to |
| `kind` | `refit`, `damaged`, `sunk`, `lost`, `transferred`, `renamed`, `reclassified` |
| `date_from`, `date_to` | When. A refit has both |
| `place` | Optional `key` in `places.csv`, or empty |
| `summary` | One line, in our own words |
| Source columns | As for facts |

Fact and fit rows have an optional `event` column naming the event that started the range. A radar fit can then point at the refit that installed it. This mirrors the simulation's rule that every change names its cause.

## Orders of battle

`engagement_forces.csv` records who was present, which is what scenarios 4 and 5 need first.

| Column | Meaning |
| --- | --- |
| `id` | Whole number |
| `engagement` | `key` in `engagements.csv` |
| `side` | `allied` or `japanese` |
| `formation` | The task organisation as the source names it: `TF 62.6 Southern Group`, `8th Fleet striking force` |
| `member_type`, `member` | `ship` or `air_unit`, and its `key` |
| `role` | Optional: `flagship`, `picket`, `detached before action` |
| Source columns | As for facts |

A ship that was in the task organisation but not in the fight (for example, a cruiser that had left before the action) is listed with a `role` that says so, not left out.

## Engagement records

Added for scenario 4c (Savo Island). A scenario needs more than who was present: where each force was, what state it was in, who saw whom first, and what happened, so the result can be checked against history. Three files hold this. Each row is one source's statement, with the same source columns and the same standing rules as a fact row.

`engagement_events.csv`: what happened, in time order.

| Column | Meaning |
| --- | --- |
| `id` | Whole number |
| `engagement` | `key` in `engagements.csv` |
| `time` | `YYYY-MM-DDTHH:MM` in the engagement's reference zone, which `engagements.csv` names. Empty if the source gives no time |
| `kind` | `sighting`, `radar_contact`, `report_sent`, `report_received`, `order`, `readiness`, `course_change`, `illumination`, `open_fire`, `torpedo_launch`, `torpedo_hit`, `gun_hit`, `cease_fire`, `sunk`, `other` |
| `actor`, `target` | A ship `key`, a formation name, or a few words for an aircraft or submarine |
| `means` | `visual`, `radar`, `sound`, `aircraft`, `searchlight`, `starshell`, `flare`, `radio`, `tbs`, `blinker`, or empty |
| `range_m`, `bearing_deg` | Range and true bearing from actor to target, if the source gives them |
| `position` | Where, in the source's terms |
| `value` | A count the event carries, such as torpedoes in a salvo |
| `as_stated` | The source's own time, zone and figures |
| Source columns | As for facts |

`engagement_positions.csv`: patrol stations, anchorages, track points and sinking positions.

| Column | Meaning |
| --- | --- |
| `id`, `engagement` | As above |
| `subject` | A ship `key`, a formation name or a place |
| `kind` | `patrol_station`, `patrol_line`, `anchorage`, `track_point`, `sinking_position`, `reference_point` |
| `time` | As above, or empty for a standing station |
| `lat`, `lon` | Signed decimal degrees as strings, south and west negative, only when the source gives latitude and longitude |
| `ref_point`, `bearing_deg`, `distance_nm` | Otherwise: true bearing and distance from a named point |
| `course_deg`, `speed_kn` | If the source gives them |
| `as_stated` | The source's own words for the position |
| Source columns | As for facts |

`engagement_outcomes.csv`: totals for checking a result against history.

| Column | Meaning |
| --- | --- |
| `id`, `engagement` | As above |
| `ship` | A ship `key`, or a force name for a whole-force total |
| `measure` | `gun_hits_received`, `torpedo_hits_received`, `killed`, `wounded`, `main_rounds_fired`, `secondary_rounds_fired`, `torpedoes_fired`, `time_first_hit`, `time_sunk`, `fate`, and others as sources give them |
| `value`, `as_stated` | As for facts |
| Source columns | As for facts |

These rows are history to test against. They are never an input that makes the simulation reproduce the night: a scenario takes its starting positions and alert states from them, and its result is then compared with the outcomes.

## Sources

### sources.csv

| Column | Meaning |
| --- | --- |
| `id`, `key` | As for entities. Keys such as `danfs`, `morison-v5`, `bates-savo-1950` |
| `kind` | `primary` (action reports, war diaries, technical manuals of the time), `official_history`, `reference_book`, `scholarly`, `web_compilation`, `tertiary` |
| `author`, `title`, `publisher`, `year`, `edition` | Full citation |
| `url` | Where it can be read, if online |
| `rights` | `public_domain`, `copyrighted`, `cc_by_sa`, `unclear`. See `source-policy.md` |
| `reliability` | `A`, `B` or `C`, with the reason in `note`. See `source-policy.md` |
| `note` | Known weaknesses: "wartime estimate of enemy equipment", "written from memory in 1947" |

One source per edition. A page number is only meaningful for the edition named.

### Confidence

Confidence belongs to the fact row, not the source, because a good source can be weak on one point.

| Level | Meaning |
| --- | --- |
| `high` | A primary document of the owning side, or two independent reliable sources that agree |
| `medium` | One reliable secondary source, uncontested |
| `low` | One source of unknown reliability, a wartime estimate of the enemy, a value we inferred, or a value other sources contradict |

"Independent" means not copied from each other. Two websites that both cite the same book count once.

### Cite what was read

A row cites the thing we actually read. If a website quotes a figure from a book we have not seen, the row cites the website, and the note says "citing" the book. It does not cite the book's page. This keeps every citation checkable by the person who entered it.

### Conflicting sources

Sources disagree often, especially on Japanese equipment and on what a ship carried on a given night. The disagreement is data.

- Each source's statement is its own row with its own value. Two sources that agree are two rows with the same value.
- Rows for the same entity, attribute and overlapping dates form a group. `standing` says how the group is settled:

| `standing` | Meaning |
| --- | --- |
| `preferred` | The value the ratings layer uses. All `preferred` rows in a group must carry the same value |
| `unconfirmed` | Nothing contradicts the value, but it is too weakly supported to be `preferred`. See the rule below |
| `alternative` | A differing value we judged less likely. The `note` on it says why |
| `disputed` | We cannot choose. Every row in the group is `disputed`, and the ratings layer gets no value until someone settles it |

- **A value has to earn `preferred`.** It needs one row at `medium` or `high` confidence, or two rows from different sources that agree, at least one of them from a source not graded `C`. A value with less support is `unconfirmed`. A lone Wikipedia row is `unconfirmed`, and so are two enthusiast sites repeating each other.
- **The ratings layer does not read `unconfirmed` values by default.** A derivation rule may use one only if the rule says so, and the generated rating then carries a flag saying it rests on an unconfirmed fact. A better source turns the value into `preferred`; nothing else does.
- **A dispute is settled by a person, with the reason in the note.** The guide: a primary document of the owning side, or of a technical examination of the equipment, that states the figure directly outranks a secondary compilation, and a compilation outranks a tertiary summary. A higher confidence grade alone does not settle a dispute, because the better source can carry a misprint. Rows that agree with the chosen value become `preferred`; the others become `alternative`.
- Settling a dispute changes `standing` and `note` only. The losing row stays.
- `docs/research/oob/conflicts.md` lists every group that has an `alternative` or `disputed` row, with a sentence on each. It is the reading list for a reviewer.

### Gaps

`data/oob/gaps.csv` lists what was looked for and not found: entity, attribute or slot, period, sources checked, note. It separates "not found in the sources we have" from "not looked for yet", and it is the work list for the next research pass.

## File format

### The options

| | CSV files, one per table | TOML, one file per entity | SQLite file built from text | SQLite file as the source |
| --- | --- | --- | --- | --- |
| Reviewable in a diff | Yes: one changed fact is one changed line | Yes, but a fact with its source spans eight or more lines | Text is reviewed, the database is not | No. Binary |
| Per-fact source and date range | A column each | Verbose, deeply nested | Yes | Yes |
| Matches the repo today | `data/reference/` is CSV with `#` comments | Scenarios and maps are TOML | New | Breaks the "versioned text" rule |
| Merge conflicts | Small, line-level | Small | None, it is not committed | Unresolvable |
| Ad hoc queries ("every ship with SG radar on 1942-08-08") | Awkward by hand | Awkward | Easy | Easy |
| Scales to the full order of battle | Yes, split files by nation if they grow | Thousands of files | Yes | Yes |
| Float risk | None. Every field is text until parsed by our rules | TOML has a float type the loader must reject | SQLite will silently store `REAL` if a column is typed loosely | Same |

### Recommendation

**CSV files are the source of truth. SQLite is an optional generated view.**

- The data is tables of uniform rows, and the unit of review is one fact. CSV puts one fact on one line.
- It matches `data/reference/` and the loader conventions already decided in decision 0009: `#` comment lines, whole numbers or decimal strings, unknown columns are an error.
- TOML is right for a scenario, which is one nested document. It is the wrong shape for ten thousand uniform rows.
- A SQLite file is worth having for research and for checking, because cross-table questions are painful in CSV. It should be built by a tool from the CSV files, written to a git-ignored path, with every quantity column declared `TEXT` or `INTEGER` so no float is ever introduced. It is a convenience. Nothing reads it but people.
- The SQLite build is not needed for the first data set and is not part of this task.

### CSV conventions

- UTF-8, comma-separated, header row, fields quoted only when they contain a comma or quote.
- `#` at the start of a line is a comment. Each file opens with a comment saying what it holds.
- Quantities match `-?[0-9]+` or `-?[0-9]+\.[0-9]+`. No exponents, no thousands separators, no units in the field.
- An empty field means "not applicable" or "unknown" as the column defines. There is no `null`, `?` or `0` stand-in.
- Identity files are sorted by `id`. Fact and fit files are sorted by entity, then attribute or slot, then `valid_from`, then source, so everything about one ship sits together and diffs stay local.
- New `id` values are the next free number in the file. Two branches that take the same number collide at merge, and the check below catches it.

### Checks

The format is only as good as its checker. These rules should be enforced by a test before the data grows:

- Every `key` and `id` is unique in its file. No `id` or `key` in the committed history has disappeared or changed.
- Every reference (`entity`, `class`, `source`, `item`, `event`, `place`) resolves.
- Every attribute is in `attributes.csv`, applies to that entity type, and its value parses as the declared type.
- Every row has a source and locator. Online sources have `accessed`.
- For `cardinality` `one`: within a group, all `preferred` rows agree, and a group is all `disputed`, or has at least one `preferred`, or has at least one `unconfirmed` and no `preferred`.
- A `preferred` value has the support the rule above asks for, and an `unconfirmed` value does not.
- `inferred`, `alternative`, `unconfirmed` and `low` rows have a note.
- Files are in the stated sort order.

Proposed home: a loader and test in `sim-data`, or a small separate tool crate, since `sim-core` must not read files. That is Rust work outside this task. For the first data set I will run the same checks with a throwaway script and report the result, and not commit the script.

## Feeding the simulation

Not built now. This is the intended path, so the facts layer is shaped to allow it.

```
data/oob/*.csv  ──►  derivation tool  ──►  data/reference/*.csv  ──►  sim-data loader  ──►  sim-core
 (facts, cited)      (rules, as of a        (ratings, generated,
                      date)                  committed)
```

- **A tool reads the facts as of a date.** It resolves each entity's `preferred` values and fits on that date, applying the ship-over-class rule. A `disputed` or missing value is an error naming the gap, not a default.
- **It applies rating rules.** Each rule is written down in `docs/` with an ID, and turns named facts into one rating. The rules are designed with the combat and detection subsystems in scenarios 3 to 5, and tuned in step 2.
- **It writes `data/reference/` files**, in the format the loader already reads, with a header saying they are generated and from which commit of the facts.
- **It writes a provenance file beside each output:** for every rating, the rule ID and the fact row IDs used. This is what lets the UI open a number all the way down to a page in a book.
- **Generated files are committed,** and CI checks that regenerating them changes nothing. The simulation then never depends on the tool at run time, and a ratings change shows up as a reviewable diff.
- **The tool is fixed-point or integer only,** by the same rule as `sim-core`, so generated ratings are identical on every machine.

The simulation's `id` for a ship, class or aircraft type is the `id` in the identity file. The placeholder classes in `data/reference/ship_classes.csv` today use ids 1 and 2. The real data should start above a reserved range (proposed: real ids start at 1000) so placeholder and real entities can never be confused in a saved game.

## Layout

```
data/oob/
  README.md                 what this is, the rules in brief, pointer to this file
  sources.csv
  attributes.csv
  ship_classes.csv          ship_class_facts.csv
  ships.csv                 ship_facts.csv
  aircraft_types.csv        aircraft_type_facts.csv
  weapons.csv               weapon_facts.csv
  sensors.csv               sensor_facts.csv
  air_units.csv             air_unit_facts.csv
  places.csv
  fits.csv
  events.csv
  engagements.csv           engagement_forces.csv
  engagement_events.csv     engagement_positions.csv     engagement_outcomes.csv
  gaps.csv
docs/research/oob/
  database-design.md        this file
  source-policy.md
  conflicts.md              every disagreement between sources, one line each
  <topic>.md                research notes: radar, sonar, torpedoes, ...
```

## Questions for Amit

These are choices I would rather not make alone. My recommendation is first in each.

1. **Two identifiers per entity (`id` number and readable `key`).** Recommended. The alternative is numbers only, which is simpler but makes every diff unreadable without a lookup.
2. **Units.** Metric for weapons and dimensions, knots and nautical miles for movement, long tons for displacement, as in the table above, with the source's own figure kept in `as_stated`. The alternative is to store whatever unit each source uses, which is more faithful but makes conflicts hard to see.
3. **How wide is "ships present" for the first data set?** Recommended: every warship in the forces engaged or screening at Savo Island and Eastern Solomons, including ships that were detached or absent on the night, with a role saying so. Transports and auxiliaries are named in `gaps.csv` but not researched yet. Submarines are left out, as the spec puts them out of step 1.
4. **Where does the checker live?** Recommended: a `sim-data` test, written by the scenario team or after their current work lands, since I am not to touch `crates/`. Until then the checks are run by hand.
5. **Real ids start at 1000,** leaving the low range to placeholders. This touches a `CLAUDE.md` data rule ("entity IDs in data files are stable"), so it should be your call.
6. **Decision record.** Once you agree the format, I propose recording it as the next numbered file in `docs/decisions/`. The scenario 2 team may also be taking the next number, so I will check the number at that point, not now.

## Changes made while filling the first data set

- `aircraft_on_hand`, `aircraft_operational` and `based_at` are `many`, not `one`. A mixed unit has one strength row per aircraft type on a date, and a unit can have detachments at two bases. This switches off the conflict check for those attributes. A `qualifier` column on fact rows would restore it and is the first change to consider.
- `displacement_trial_lt` was added. Japanese sources give trial displacement where others give full load.
- `events.csv` was not started. Losses are `sunk` and `fate` facts and refits are dated fits. It is added when a scenario needs a change with a cause.
- Sources graded `C` are capped at `low` confidence whatever the row says.
- The link from a ship to its class has no source column. It should get one, or become a dated fact.

## Changes made in the Savo Island pass

- **A fourth standing, `unconfirmed`,** and the two rules under "Conflicting sources". Review found lone low-confidence rows marked `preferred` by default, and a primary figure left `disputed` beside weaker ones.
- **`ship_class` is now an attribute of a ship,** so a source's statement of a ship's class is a fact row like any other. The `class` column in `ships.csv` holds the preferred value.
- **Three engagement files,** described under "Engagement records". `engagements.csv` gains a `zone` column for the reference time zone.
- **A fit row with `count` 0 also records "never fitted",** not only "removed", when a source says a ship or class had none.
- **Night optics.** Sensor kinds `night_binocular` and `searchlight` and fit slots of the same names were added.
- Notes no longer point at a `proposed_attributes.csv` file. That was a working file and was never part of the format.

## What this design does not cover

- Pilots, land units, bases' port and airfield sizes, and supply figures. The spec's state model has them, and scenarios 2 and 6 will need some. They fit the same pattern (identity file plus facts file) and are added when needed.
- Reinforcement schedules and production. Out of step 1.
- The rating rules themselves.
- Anything outside the first slice.
