# Measures for validation

One file per battle: what step 2 measures in a run, and what the record gives for it. `docs/step2-spec.md` says how they are used. Nothing in the simulation reads these files; `sim-cli validate` does.

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
| `take` | How the number is taken from a run in the batch file, with the keys that go with it (see "What a measure takes"). Left out while the scenario is not built |
| `rows` | The ids of the fact rows the record rests on |
| `facts` | Only where the measure's rows are in another fact file than the one the file names at its head: that file |
| `reason` | Which rows give each end of the range, and for a measure not judged, why not |

## What a measure takes

`counts` says in words what is counted. `take` and its keys say the same thing so that `sim-cli validate` can do it, in the names the batch file uses. Ships, air groups, task forces and bases are given by their ids in the scenario. An air group formed in a run by a landing elsewhere counts with the group it came from, and with that group's source if it too was formed in the run. A ship's task force is the one it starts in.

| `take` | Keys | The number |
| --- | --- | --- |
| `ships_sunk` | `ships` | How many of the ships are sunk at the end |
| `ships_with_system_out` | `ships`, `system` | How many of the ships are afloat at the end with that system out: `propulsion`, `fire_control`, `main_battery`, `torpedo_mounts` or `flight_deck` |
| `ships_sunk_or_system_out` | `ships`, `system` | How many of the ships are sunk at the end, or afloat with that system out |
| `hits_on` | `ships`, `weapons` | Hits of those weapons on those ships, as rolled. A salvo is rolled at the ships afloat when its round starts, so a hit on a ship that another salvo sinks in the same round is counted. The record counts hits on a ship already sinking too |
| `hits_per_thousand_shots` | `side`, `weapon` | The side's hits with that weapon over the run, times 1,000, divided by its shots, rounded down. The shots are the exact figure the rules fired, fractions and all. A run in which the side fires no shot is left out |
| `shots_in_hundreds` | `side` or `ships`, not both; `weapon` | The side's shots with that weapon over the run, or those ships' shots in surface fights, summed and then divided by 100 and rounded down, once. Shells are counted in hundreds because a record of shells is one figure, and no run would hold a figure of thousands to the shell |
| `shots` | `side` or `ships`, not both; `weapon` | The shots with that weapon over the run, summed and rounded down to a whole shot |
| `air_losses` | `air_groups`, `causes` | Aircraft of those groups lost to those causes. The causes are the batch file's: `shot_down`, `destroyed_on_ground`, `damaged_destroyed_on_ground`, `lost_with_ship`, `worn_out`, `ditched_fuel`, `ditched_no_deck`. No aircraft is counted under two |
| `pilots_lost` | `air_groups` | Pilots of those groups lost |
| `ready` | `air_groups`, `day` | Aircraft of those groups ready at the end of that day, the first day being 1 |
| `aircraft_attacking` | `side` or `air_groups`; `target_task_forces` or `target_base`; `escorts` | Aircraft in every attack by that side or those groups on a ship of those task forces or on that base, summed over the run. With `escorts = true` the aircraft escorting an attacking group in the same day and pulse are added. The batch file names a raid's first strike group as the one escorted, so the escorts are matched to that group's attacks |
| `attack_days` | as `aircraft_attacking`, without `escorts` | The number of days with at least one such attack |
| `attack_of_at_least` | `side`, `target_task_forces`, `aircraft` | 1 if in some one day and pulse the aircraft attacking ships of those task forces come to that many or more, escorts not counted; else 0 |
| `claims` | `side`, `claim` | What the side claims of that kind (`sunk`, `hits`, `shot_down`, `on_ground`) |
| `claims_per_hundred` | `side`, `claim`, and what was really done: `ships` for `sunk`; `ships` and `weapons` for `hits`; `air_groups` and `causes` for `shot_down` and `on_ground` | The claims, times 100, divided by what was really done, rounded down. A run in which nothing was really done is left out |
| `minutes_of_action` | none | The rounds each surface fight ran (the batch file's `fights`), summed, times `round_minutes` from the surface combat figures the batch was run on |
| `fights_by_day` | none | The number of surface fights in the run that are not in a night pulse |
| `fights_sighted_first` | `by` | The number of surface fights in a night pulse in which `sighted_first` is one of the values listed in `by` ("allied", "japanese", "both", "neither") |
| `rounds_surprised` | `task_forces` | The rounds those task forces were surprised, summed over the run's surface fights in a night pulse |

**Lost to enemy action is aircraft shot down, and nothing else.** The rules have no write-off for damage taken in combat: an aircraft worn out after a sortie is worn out whoever it met. Where the record counts aircraft expended for combat damage or forced down by it, they stay in the record, and the aircraft shot down in a run stand for them. A ditching, for want of fuel or of a deck, is never enemy action; it is counted where a measure takes every cause.

**A claim does not say whose aircraft or which ship.** So what was really done takes every unit of the enemy that the claim could have been of.

## Claims

A claims measure is what a side believed it did for each hundred it really did, rounded down (decision 0056). It sits in its battle's file. A run in which the side really did none is left out of the measure. Each end of the range is a division of two recorded figures.

## Passages

`passages.toml` is laid out differently, because a passage is checked with one calculation and not with runs. Each `[[passage]]` gives `facts` (the file its rows are in), a start and an end, `speed_kn`, and the record in minutes as `record_low` and `record_high`. A start is `from_place` (a place in `data/oob/places.csv`) with `from_row`, the id of the row of `data/oob/place_positions.csv` its position is taken from; a place can have several rows, in different hexes. An end is `to_place` with `to_row` in the same way. Either may be a position instead: `from_lat` and `from_lon`, or `to_lat` and `to_lon`, decimal strings with south and west negative. No end is given as a bearing and a distance; the architect works the position out and the `reason` says so. `id`, `counts`, `judged`, `rows` and `reason` are as for a measure.

The minutes are worked out so: the sea route between the two hexes on `data/map/south_pacific.csv`, the miles the rules charge for it (decision 0058), less `to_short_by_nm` where the run ends that many miles before the place, steamed at `speed_kn`. `speed_changes` is a list of `[minute, speed]`, minutes counted from the start of the run: the speed held from that minute on. The answer is the whole minute in which the miles are reached, rounded up.

No passage is judged today. Research found no long run at one held speed, and a ship's speed made good in the record is a half to three quarters of its stated speed; the rules hold a force to its ordered speed on a straight course. The passages are reported to show the size of that.

## Rules

- **No recorded value without its rows.** If no row gives a fact, there is no measure for it; the gap is said in a `reason` nearby or in `data/oob/gaps.csv`.
- **A range spans a dispute and does not end it.** No row's standing is changed here. A row that is `unconfirmed` or `disputed` may give one end of a range only if the `reason` says so.
- **The record is cut to the scenario's scope.** A battle's totals in the histories often cover more days and more forces than the scenario holds.
- **A measure is not judged where the rules cannot produce what the record counts,** such as a strike lost coming home in the dark. It is still reported, so the gap stays in view. Where only a part of the record is out of reach and a row gives that part its own number, the part is taken out of the record, the `reason` says so, and the rest is judged.
- **A fact that is yes or no is a measure of 0 or 1.** It is judged by the same test as any other.
- **Whole numbers only,** as in every data file.

## Who settles them

The ranges, and the rows the measures read, are settled by the coordinating architect (the main session), who keeps these files and the step 2 spec since Nov 13, 2026; each file says who settled it, with the date. Files settled before then name the spec session's architect, whose session is archived.

## Files

| File | Battle | State |
| --- | --- | --- |
| `eastern_solomons.toml` | Eastern Solomons, 24 August 1942 | 16 measures, 14 judged |
| `savo_island.toml` | Savo Island, 8 to 9 August 1942 | 26 measures, 23 judged |
| `rabaul_guadalcanal.toml` | Rabaul against Guadalcanal, 21 August to 17 September 1942 | 15 measures, 10 judged |
| `cape_esperance.toml` | Cape Esperance, 11 to 12 October 1942 | 9 measures, 7 judged. The scenario is written (`data/scenarios/not-run/17_cape_esperance.toml`) and not run, and the file has no `take` yet; it says what scope it assumes. No measure of hits for each thousand shells is possible: the rounds fired are not on record |
| `passages.toml` | Ship passages | 6 passages, none judged |

Still to write: for Cape Esperance, who sighted whom first, when the scenario is scoped.
