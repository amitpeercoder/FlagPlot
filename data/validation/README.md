# Measures for validation

One file per battle: what step 2 measures in a run, and what the record gives for it. `docs/step2-spec.md` says how they are used. Nothing in the simulation reads these files; `sim-cli validate` will, once it is built.

## Format

A file opens with three keys:

| Key | Holds |
| --- | --- |
| `battle` | The engagement's name in `data/oob` |
| `scenario` | The scenario file the measures are taken from |
| `facts` | The fact file the `rows` are ids in |

Then one `[[measure]]` table for each measure:

| Key | Holds |
| --- | --- |
| `id` | A stable name. Never reused |
| `counts` | What is counted in a run, in the rules' own words (`docs/glossary.md`) |
| `record_low`, `record_high` | What the record gives, as whole numbers. Equal when the sources agree; a range when they do not |
| `judged` | `true` if the measure counts toward agreement. `false` if it is only reported |
| `rows` | The ids of the fact rows the record rests on |
| `reason` | Which rows give each end of the range, and for a measure not judged, why not |

## Rules

- **No recorded value without its rows.** If no row gives a fact, there is no measure for it; the gap is said in a `reason` nearby or in `data/oob/gaps.csv`.
- **A range spans a dispute and does not end it.** No row's standing is changed here. A row that is `unconfirmed` or `disputed` may give one end of a range only if the `reason` says so.
- **The record is cut to the scenario's scope.** A battle's totals in the histories often cover more days and more forces than the scenario holds.
- **A measure is not judged where the rules cannot produce what the record counts,** such as aircraft that ditched for want of fuel. It is still reported, so the gap stays in view.
- **A fact that is yes or no is a measure of 0 or 1.** It is judged by the same test as any other.
- **Whole numbers only,** as in every data file.

## Who settles them

The ranges are settled by the architect of the session that owns the step 2 spec, and each file says so with the date. Rows that feed derivation and scenarios are settled by the build session's architect.

## Files

| File | Battle | State |
| --- | --- | --- |
| `eastern_solomons.toml` | Eastern Solomons, 24 August 1942 | First pass, 12 measures, 7 judged |
| `savo_island.toml` | Savo Island, 8 to 9 August 1942 | First pass, 11 measures, all judged |

Still to write: Rabaul against Guadalcanal, and the claims and passages checks.
