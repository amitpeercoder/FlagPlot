# The surface, search and derivation figures: where each stands

Nov 13, 2026, at the close of step 2. This page lists every figure of `surface_combat.toml`, `search.toml`, the model parameters of `data/derivation/rules.toml` and the other hand-made files of `data/reference`, with its value and what it rests on. The air figures are on `air-figures-closed.md`; where this page reads one of them differently, it says so below. It is a summary: the round records and decisions it names are the source, and each figure carries the same note as a comment above it in its file. A figure that moves changes its row here.

**What each status means:**
- **Sized from rows.** Its value is what fact rows state, or a source's method, with the rows named.
- **Fitted, not checked.** A round moved it to fit a battle, inside a direction the sources support, with no size from a source. Decision 0084 calls these fits, not sizes; they are placeholders until a check battle is run.
- **Looked at, not moved.** A round tried it, or named it and gave the reason it stays, and it did not move. No row sizes it.
- **Placeholder.** No row sizes it, and no round looked at it.
- **No battle reads it.** No battle of the slice (Savo Island, the Eastern Solomons, Rabaul against Guadalcanal) can move it.

**Which battle reads which file.**
- Savo Island (scenario 4) loads `search.toml` and `surface_combat.toml`. Its ships are the derived classes of `ship_classes_savo_island.csv`.
- The Eastern Solomons (scenario 5) loads those and `air_combat.toml`. Its ships and aircraft are derived.
- Rabaul against Guadalcanal (scenario 6) loads those and the three slow air files. It has no ships and no task forces, so no surface figure and no figure of contacts acts there.

The surface figures are tuned on Savo Island and checked on Cape Esperance, which is not yet run (`docs/step2-spec.md`, "Tuning"; decision 0072).

## `surface_combat.toml`

| Figure | Value | Status | Record |
| --- | --- | --- | --- |
| `round_minutes` | 5 | Placeholder (decision 0014). The rates of fire of decision 0070 count cases of at least one round | The file |
| `max_rounds` | 6 | Looked at, not moved: four rounds took the northern group's sinkings away from the record | `savo-island-closed.md` |
| `sighting_chance_night` | 0.5 | Looked at, not moved: no source gives a size; the rows give a direction | `r0-finding-and-surprise.md` |
| `sighting_chance_day` | 1 | Placeholder. At Savo Island read only in runs with a fight by day, which the measure asks to have none | The file |
| `radar_sighting_bonus` | 0.2 | Looked at, not moved: the radar was mended in the derivation instead (decision 0077). No ship at Savo Island has a set that counts, so it never acts there | `r0-finding-and-surprise.md` |
| `not_alert_sighting_factor` | 0.5 | Looked at, not moved: 0.4 to 0.2 tried; no row gives a size (`engagement_events` 23, 24 give the direction) | `r0-finding-and-surprise.md` |
| `gun_hit_chance_night` | 0.03 | Looked at, not moved: 0.04 tried; both sides share it, and it took the Allied rate past its record | `r1-experience-gun-strength.md`, `r2-surprised-target-factor.md` |
| `gun_hit_chance_day` | 0.05 | Placeholder. Fights by day only | The file |
| `torpedo_hit_chance_night` | 0.04 | Placeholder. No round moved a torpedo figure | `r1-experience-gun-strength.md` |
| `torpedo_hit_chance_day` | 0.02 | Placeholder. Fights by day only | The file |
| `surprised_target_factor` | 2 | **Fitted on Savo Island, not checked.** Round 2, 1.5 to 2; 2.5 looked at and not moved. Decision 0084, finding 2: "a figure fitted at Savo Island inside a direction the sources support, a placeholder, and not sized from a source" | `r2-surprised-target-factor.md`, `savo-island-closed.md` |
| `lit_target_factor` | 2 | Looked at, not moved: 3 and 4 tried; both sides share it | `r1-experience-gun-strength.md`, `r2-surprised-target-factor.md` |
| `gun_hit_flotation` | 0.75 | Looked at, not moved: 0.5 took the northern group's sinkings away from the record. Read by bombs too | `savo-island-closed.md` |
| `system_hit_chance_per_point` | 0.06 | Placeholder (decision 0033: chosen so the placeholder cruiser's hit keeps its 0.3). The round on what a hit does found no source for a size. Read by bombs and aircraft torpedoes too | `savo-island-closed.md` |
| `degraded_factor` | 0.5 | Placeholder (decision 0016). Also sets a degraded ship's speed and a degraded deck's launches | The file |
| `fire_control_out_factor` | 0.25 | Placeholder (decision 0016). Surface fights only | The file |
| `alert_contact_distance_hexes` | 3 | Looked at, not moved: no source for a size; at 7 or more the Allied side's starting contact would alert both groups | `r0-finding-and-surprise.md` |
| `alarm_chance` | 0.5 | Placeholder (decision 0021) | The file |
| `crippled_flotation` | 50 | Placeholder (decision 0014). It decides only breaking off, where nothing moved | `savo-island-closed.md` |
| `claimed_sunk_fire` | 30 | Looked at, not moved: no row gives the fire at which a crew believed a ship lost; Savo Island's Allied claim agrees. Read by a raid's claim too | `r4-claims.md` |
| `experience_gun_strength` | 0.5 | **Fitted on Savo Island, not checked.** Round 1, 0.25 to 0.5, the cap; 0.35 looked at and worse. Decision 0084, finding 6: "an effect size fitted at Savo Island, at the cap decision 0064 allows, with a sign from the sources and no size" | `r1-experience-gun-strength.md`, `savo-island-closed.md` |
| `experience_torpedo_strength` | 0.25 | Placeholder: the middle of decision 0064's range | The file |
| `experience_sighting_strength` | 0.25 | Looked at, not moved: no source for a size | `r0-finding-and-surprise.md` |
| `unready_gun_share` | 0.5 | **Sized from a row:** Condition II, half the armament manned (`engagement_events` 23). Not tuned. The record's shares are lower, 3 to 21 in 100, for causes the rule does not model (decision 0068, 8) | Decision 0068; `savo-island-closed.md` |
| `unready_torpedo_share` | 0.5 | **Sized from a row,** and looked at, not moved: Bagley fired four of the eight tubes that bear on her port side (`engagement_outcomes` 46, 47, 3206, 3228). 0.25 and 1 miss as 0.5 does, because Helm and Wilson fire where the rows have neither firing | `savo-island-closed.md`; decisions 0071, 0074 |
| `[gun_system_hit]` | propulsion 0.2, fire control 0.3, main battery 0.35, torpedo mounts 0.15 | Placeholders (decision 0016). Surface fights only | The file |
| `[torpedo_system_hit]` | propulsion 0.8, fire control 0.1, main battery 0.1 | Placeholders (decision 0016). The damage-control note gives a direction only (Saratoga's engines). Read by aircraft torpedoes too | The file |
| `[straggler_escorts]` cruiser, destroyer | 1, 0 | Placeholders (decision 0016). Read at Savo Island, where the Japanese column detaches stragglers | The file |
| `[straggler_escorts]` carrier, battleship, transport, auxiliary | 2, 2, 1, 0 | **No battle reads them:** no other force in the slice detaches stragglers, and the default is to keep them | The file |

Five of these are read beyond the surface fight (`docs/step2-spec.md`, "Tuning", 9): `gun_hit_flotation`, `system_hit_chance_per_point`, `degraded_factor`, `[torpedo_system_hit]` and `claimed_sunk_fire`. A round that moves one runs all three battles again.

## `search.toml`

No tuning round looked at these figures. Finding and searching were set otherwise: the Japanese search at the Eastern Solomons from the record (decision 0073), each type's search radius (decision 0075), the radar a class has (decision 0077), and round 0 on the sighting figures of `surface_combat.toml`.

| Figure | Value | Status | Record |
| --- | --- | --- | --- |
| `contact_drop_age_pulses` | 5 | Placeholder: decision 0032 kept the age the earlier rule of confidence gave | Decision 0032 |
| `merge_distance_hexes` | 2 | Placeholder (decision 0012) | The file |
| `fresh_contact_max_age_pulses` | 2 | Placeholder (decisions 0012, 0021). Through alertness it reaches the surface fight | The file |
| `position_error_chance` | 0.3 | Placeholder. The air search note names position error as a thing to model and gives no size | The file |
| `count_over_chance`, `count_under_chance` | 0.2, 0.2 | Placeholders | The file |
| `[kind_confusion]` carrier, battleship, cruiser, destroyer (14 entries) | 0.7 for the true kind; the rest as in the file | Placeholders. Carrier and battleship are read at the Eastern Solomons; cruiser and destroyer there and at Savo Island. The first Hudson report at Savo Island gave three cruisers, three destroyers and two "seaplane tenders or gunboats" for seven cruisers and a destroyer, which the table cannot give | The file |
| `[kind_confusion]` transport, auxiliary (5 entries) | 0.7 for the true kind; the rest as in the file | **No battle reads them:** the slice has no transport and no auxiliary | The file |

## `data/derivation/rules.toml`: the model parameters

Every other figure of a real ship or aircraft is read from fact rows; the derived files name the rows (`*.sources.csv`). These are the parameters the rows are multiplied or filtered by. Comments on them do not change a generated file (`oob-derive --check` passes on every list).

| Figure | Value | Status | Record |
| --- | --- | --- | --- |
| `gun_shell_kg_per_damage` | 6 | Placeholder: set so real ships land near the placeholder cruiser (decision 0020). Reaches both class files, and bomb damage | `savo-island-closed.md` (no source gives a size) |
| `torpedo_damage_per_warhead_kg` | 0.4 | Looked at, not moved: 0.25 tried. Reaches aircraft torpedo damage too | `savo-island-closed.md` |
| `displacement_lt_per_toughness` | 25 | Looked at, not moved: 35 tried. The rows support displacement as the measure, not the divisor | `savo-island-closed.md` |
| `anti_aircraft_heavy_weight`, `anti_aircraft_light_weight` | 1, 0.25 | Placeholders (decision 0029, 1). Reach the ships of the Eastern Solomons. Air round 5 looked at the guns' chance to hit, not these | Decision 0029 |
| `strike_radius_fraction_of_range` | 0.333333 | Placeholder (decision 0029, 6). Gives the strike radius of the D3A1, B5N2 and E13A1 at the Eastern Solomons and the G4M1 at Rabaul. The Bureau of Aeronautics' third is of the scouting range, which is decision 0075's rule for search, not this | Decision 0029; `aircraft_types_*.sources.csv` |
| `search_sweep_width_nm` | 20 | Placeholder (decision 0029, 10: "No source gives it"). Every air search at the Eastern Solomons and Rabaul reads it | Decision 0029 |
| `[fire_rate] min_span_minutes` | 5 | Placeholder: five minutes because a round is five minutes (decision 0070). The salvo median rests on four cases, the destroyer and continuous kinds on one each | Decision 0070 |
| `[experience]` `recent_months`, `lookback_months`, `new_ship_months` | 3, 12, 12 | Placeholders: fixed with the questions before any batch (decision 0064). They read the Savo Island and Cape Esperance ships | Decision 0064 |
| `[search_radius] divisor_of_scouting_range` | 3 | **Sized from a source:** the method of the Bureau of Aeronautics' sheets, a third of the range in the scouting condition (decision 0075; the note on `aircraft_type_facts` 555) | Decision 0075 |

`constants_are_placeholders = true` stays: `oob-derive` refuses the file without it.

## The other hand-made files of `data/reference`

| File | Status |
| --- | --- |
| `port_sizes.csv` | **No battle reads it.** Every scenario loads it, but no force at Savo Island or the Eastern Solomons has an order to load or unload, and scenario 6 has no ships. Placeholders. `place_facts.csv` has port rows (Noumea 96, 116; Henderson Field 58; Milne Bay 85) that no derivation reads yet |
| `ship_classes.csv` | **No battle reads it.** Scenario 6 loads it and has no ships; the other two battles read derived classes |
| `aircraft_types.csv` | **No battle reads it.** Savo Island loads it and has no aircraft; the other two battles read derived types |

The derived files (`ship_classes_*`, `aircraft_types_*`, `ship_experience_*`) are generated by `oob-derive` and carry their own headers: each figure from rows, or rows times a parameter above. The frozen copy, `data/reference/step1`, is not touched.

## The figures no battle of the slice can move

Checked against the scenario files and the code on Nov 13, 2026.

- **The three the spec names** ("Tuning"): `air_upkeep.carrier_repairs_per_day` (scenario 5 does not load `air_upkeep.toml`; scenario 6 has no carriers), and `fatigue.force_fatigue_per_pulse_alert` and `force_rest_per_pulse` (scenarios 4 and 5 do not load `fatigue.toml`; scenario 6 has no task forces). All three hold.
- **Three of the four `crew_recovered_*` figures**, which the spec puts with Rabaul's. Every draw of them needs pilots counted, and the Eastern Solomons counts none. At Rabaul:
  - `crew_recovered_beside_own_force` needs a ship of the group's side (a patrol aboard a ship, or a landing in the sea beside one), and scenario 6 has no ships;
  - `crew_recovered_open_sea` needs a group whose deck is lost, and a group at a base is never affected (`landing.rs`);
  - `crew_recovered_short_of_home` needs a sortie beyond its type's radius, and scenario 6 orders no extended reach, so no strike launches beyond it. No run baseline of scenario 6 ditches.
  Only `crew_recovered_over_own_base` is read, by a patrol's pilots over Henderson Field.
- `[straggler_escorts]` carrier, battleship, transport and auxiliary.
- `[kind_confusion]` transport and auxiliary.
- `port_sizes.csv`, `ship_classes.csv` and `aircraft_types.csv`.

Two more are close to it. `radar_sighting_bonus` never acts at Savo Island, where no ship has a set that counts; at the Eastern Solomons only the Atlanta class has one, and the measures there have no surface fight. The day chances of the surface fight are read at Savo Island only in the runs with a fight by day, which its measure asks to have none.

## Where this page reads the air figures differently

`air-figures-closed.md` lists every figure of `air_combat.toml`. The notes in that file follow it, with two differences, both from the records:

- **`anti_aircraft_before_attack_share` and `patrol_drawn_off_share`** are noted as fitted, not checked, in decision 0084's words (finding 5): endpoint placeholders "chosen with both air battles' results known". Its list of limits names them with the fitted figures.
- **`crew_recovered_short_of_home`, `crew_recovered_beside_own_force` and `crew_recovered_open_sea`** are noted as read by no battle of the slice (above), not as placeholders with no check battle.

The comment on `crew_recovered_open_sea` now gives the note's own figures, 6 to 9 crews recovered and 4 to 7 lost (`docs/research/oob/ditching-and-rescue.md`, section 2). "6 to 9 of 13 to 16" (decision 0059) gives a total the note does not state.

## Counts

| File | Sized from rows | Fitted, not checked | Looked at, not moved | Placeholder | No battle reads it | All |
| --- | --- | --- | --- | --- | --- | --- |
| `surface_combat.toml` | 2 | 2 | 10 | 20 | 4 | 38 |
| `search.toml` | 0 | 0 | 0 | 20 | 5 | 25 |
| `air_combat.toml` | 2 | 2 | 7 | 12 | 3 | 26 |
| `rules.toml` parameters | 1 | 0 | 2 | 9 | 0 | 12 |
| Other hand-made files | 0 | 0 | 0 | 0 | 3 files | 3 files |

A table entry counts as one figure; `placeholder = true` and `constants_are_placeholders` are flags, not figures.

## Outside `data/reference`

The objectives scale (decision 0054: ships' points, `aircraft_destroyed`, `base_open`, `cargo_landed`, the levels 3, 8 and 16) is in the scenario files, not here. Every value on it is a placeholder, and the comment beside it in scenarios 4, 5 and 6 still says "to be set from the record in step 2". It is reported and not judged (`docs/step2-spec.md`, "Rule work that comes before tuning", the line on "out of the campaign").
