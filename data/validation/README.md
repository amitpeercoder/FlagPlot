# Measures for validation

One file per battle: what step 2 measures in a run, and what the record gives for it. `docs/step2-spec.md` says how they are used. Nothing in the simulation reads these files; `sim-cli validate` will, once it is built.

## Format

A file opens with three keys:

| Key | Holds |
| --- | --- |
| `battle` | The engagement's name in `data/oob` |
| `scenario` | The scenario file the measures are taken from. Left out while the scenario is not built |
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

## Claims

A claims measure is what a side believed it did for each hundred it really did, rounded down (decision 0056). It sits in its battle's file. A run in which the side really did none is left out of the measure. Each end of the range is a division of two recorded figures.

## Passages

`passages.toml` is laid out differently, because a passage is checked with one calculation and not with runs. Each `[[passage]]` gives `facts` (the file its rows are in), `from_place` (a place in `data/oob/places.csv`), the end as `to_place` or as `to_bearing_deg` and `to_distance_nm` from the start, `speed_kn`, and the record in minutes as `record_low` and `record_high`. `to_short_by_nm` ends the run that many miles before the place. `speed_changes` is a list of `[minute, speed]`: the speed held from that minute on. `id`, `counts`, `judged`, `rows` and `reason` are as for a measure.

No passage is judged today. Research found no long run at one held speed, and a ship's speed made good in the record is a half to three quarters of its stated speed; the rules hold a force to its ordered speed on a straight course. The passages are reported to show the size of that.

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
| `eastern_solomons.toml` | Eastern Solomons, 24 August 1942 | 15 measures, 9 judged |
| `savo_island.toml` | Savo Island, 8 to 9 August 1942 | 13 measures, 12 judged |
| `rabaul_guadalcanal.toml` | Rabaul against Guadalcanal, 21 August to 17 September 1942 | 15 measures, 10 judged |
| `cape_esperance.toml` | Cape Esperance, 11 to 12 October 1942 | 9 measures, 7 judged. The scenario is not built; the file says what scope it assumes |
| `passages.toml` | Ship passages | 4 passages, none judged |

Still to write: for Cape Esperance, who sighted whom first and hits for each thousand shells, when the scenario is scoped and if the rounds fired are found.
