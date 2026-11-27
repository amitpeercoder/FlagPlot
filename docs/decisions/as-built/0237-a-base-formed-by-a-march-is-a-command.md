# 0237, as built: A base formed by a march is a command

## Built (Nov 27, 2026, pull request pending)

Class 3. The rule is built as the record rules it, in `sim-core` only: `World::base_formed_by_march(base, left)` (`pictures.rs`) makes the base a command with a copy of the picture of the command at the hex left (`command_at_hex`) by `base_formed`, and `march.rs` calls it where "Dump at (q,r)" is put into the state, before `MarchBaseFormed` is recorded and before the supply carried is put into the base (ruling 5). The rules label is 36 (ruling 3). No state, field, draw, event or `Draw` value is added; `hash.rs` and `picture_of` are not touched.

### Statements

- `tests/command_levels_ashore.rs`: the three tests of the statements, written under the doc comment with no word of it changed. Before the mend: scenario 29 with command levels on stops at the end of day 3 with "base:8 is not a command of the Allied side" (statement 1 and the on-is-off property fail); scenarios 25 and 37 fail statement 4 at the end of day 3 (`left: {Area, Base(1)}`, `right: {Area, Base(1), Base(3)}` for scenario 25). After the mend all three pass.
- `crates/sim-core/src/pictures_tests.rs`: `a_base_formed_by_a_march_is_a_command_with_a_copy_of_the_picture_of_the_command_left` (the three cases) and the command-levels-off case. `held_tests.rs`: `a_base_formed_by_a_march_is_held_as_it_stood_before_its_supply_was_put_into_it`. They do not compile before the mend (the hook does not exist); they pass after.

### The moved row

Only the row of `stage-2b-proof/07a_week.toml` in `BEFORE` (`tests/scenario_130/part_f.rs`) moved. Its digest was `0x485071c7e65da537` and is `0xe774a1c621dab10b` (FNV-1a over the text, `FLAGPILOT_130_TEXTS`, before and after; the old digest was reproduced from the "before" text at main).

- The game's events are the same up to and including the `MarchBaseFormed` of base 10 (day 16, event 2705 of 4357); the first event that differs is the 2731st, day 16, a `ReportSent` from base 10 (the dump reports its state, up and sideways) where the area command's report was. The staff decisions part of the text differs first at day 18 (line 10887), the first day a staff reads what the dump's reports changed; the decisions of days 1 to 17 are identical.
- After the first difference the game is a different game: events 4357 before, 4345 after. By kind (before, after): StateReportLate 585, 590; ReportSent 87, 83; OrderAccepted 161, 163; StaffReason 313, 315; AirGroupFatigue 257, 249; AircraftWorn 201, 198; AircraftRepaired 47, 45; ContactAged 23, 27; ContactDropped 5, 6; StrikeLaunched 8, 5; StrikeFoundNothing 3, 0; ShipDamaged 2, 1; SystemHit 2, 1; CannotMeet 0, 1; ContactNotThere 1, 0.
- No other row of `BEFORE`, no run baseline and no line of `tests/golden/pinned_hashes.txt` moved. `condition 6` of `docs/validation/stage-2b-proof.md` (the week) reproduces only at b5d31a7a or earlier (ruling 10); the report stands as run.

### The five tests of ruling 4, and ruling 11

On the first full run four passed with no word changed (the seat test, the week's on-is-off on seeds 1 to 3, and the "certain is step N" tests of scenarios 62 and 63) and one failed: `every_report_has_a_road_the_state_agrees_with` (`crates/sim-cli/src/events.rs`), on the second proof's week with command levels and reports on, seed 1, report #2235 (base 2's, day 15). Base 10, the dump, holds it from the end of day 16 because it is formed with a copy of base 2's picture, and the road reader named it "its own" at day 15. The build was right; the reader had no road for a report held by copy. Ruling 11 mends it in `sim-cli` only: the watcher notes the first moment each command is seen; `Road::Own` gains `formed: Option<Moment>`, set for a command not there at the game's first moment that first held the report at the moment it was first seen and did not send it (checked before any passing); `agrees` accepts it only when the state first showed the report held at exactly that moment; its line reads "{command} first held report #N at {moment}: a copy of the picture it was formed with." and its JSON road is "copy". One test is added beside it: the row of base 10 for report #2235 reads "base:10 first held report #2235 at the end of day 16, before its Logistics step: a copy of the picture it was formed with." After the mend all five tests of ruling 4 pass with no word of them changed, and the whole workspace test run passes.

### The sweep (ruling 9)

All 263 scenario files played with `sim-cli run --events` on a build at main and on a build with the mend, from a scratch copy of `data/` with `command_levels = true` as the first line of the 254 files that do not state it; the 9 that state it as written. None stopped after the mend; one stopped before (`29_skills.toml`, the stop found). Compared with the daily hash lines left out (the hash covers the commands, so it differs from the day a dump forms):

- Events differ in two files: `29_skills.toml` (it stopped before; the first differing line is the end of its run) and `stage-2b-proof/07a_week.toml` (first differing line `#2731 D16`, a `ReportSent` of base 10). `stage-2b-proof/04_savo_island.toml` is the same.
- Events are the same, daily hashes differ from the day a dump forms: `07_full_slice.toml`, `25_march.toml`, `37_men_falling_back.toml`, `45_soak_ashore.toml` and `second-proof/07a` to `07f_week.toml` and `07m_model_week.toml`. None of them has report figures, so a dump's command changes no event. (Ruling 9 as amended expects these: the hash lines differ from the day a dump forms because the hash covers the commands and a dump adds one.)

### Files changed outside `crates/sim-core/src`, `tests/` and `docs/`

`crates/sim-cli/tests/recorded_games/*/save.json` (five; `rules_version` 35 to 36 and nothing else) and its `README.md`; and, for ruling 11, `crates/sim-cli/src/events.rs` and `crates/sim-cli/src/events/report_road.rs`.
