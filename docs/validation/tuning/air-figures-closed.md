# The air figures: where each stands

Nov 13, 2026, after Rabaul round 3 and air round 5. This page lists every air figure with its value and what it rests on, and every named miss of the air battles with where it is recorded. It is a summary: the round records it names are the source, and a figure that moves changes its row here.

**What each status means:**
- **Sized from rows.** A round set the value from fact rows, with the rows and the assumptions named.
- **Fitted, not checked.** Chosen with the battles' results known, with no size from a source (decision 0084).
- **Looked at, not moved.** A round tried it, or named it and gave the reason it stays, and it did not move. No row sizes it. Used for `air_combat.toml`, sorted as its figure notes are; in the slow files below such a figure is listed as a placeholder, with the round that looked at it.
- **Placeholder.** No row sizes it. It is not fitted to any measure, and its file says so.
- **No battle reads it.** No tuned battle of the slice can move it (`docs/step2-spec.md`, "Tuning").

**Which battle reads which file.**
- Savo Island (scenario 4) loads `search.toml` and `surface_combat.toml`.
- The Eastern Solomons (scenario 5) loads those and `air_combat.toml`.
- Rabaul against Guadalcanal (scenario 6) loads those and also `air_upkeep.toml`, `fatigue.toml` and `air_supply.toml`.

The figures of `air_combat.toml` are tuned on the Eastern Solomons and checked on Rabaul. The rest, and `[base_hit]` and `crew_recovered_over_own_base`, have no check battle: an agreement on them at Rabaul is a fit, not a check. The other three `crew_recovered_*` figures are read by no battle of the slice (corrected Nov 13, 2026, from the figure notes; `surface-and-search-figures-closed.md`).

## `air_combat.toml`

| Figure | Value | Status | Where |
| --- | --- | --- | --- |
| `fighter_false_claim_chance` | 0.7 | Sized from rows: 43 claimed by VF-5 and VF-6 (257, 238, 239) over the runs' own 16 real kills and 38 missed shots. The runs' kills are an assumption: no row gives the fighters' kills apart from the guns'. Checked on Rabaul's American claims, which agree | `r4-claims.md` |
| `[base_hit] aircraft` | 0.06 | Sized from rows: about 2.5 aircraft destroyed on the ground (1266, 1272, 1283, 1299, 1330) for about 212 bomber sorties. It assumes one bomb a bomber at `bomb_hit_chance`; no row gives the bombs that hit Henderson Field. No check battle | `rabaul-r1-base-hit-aircraft.md` |
| `fighter_hit_chance` | 0.3 | Looked at, not moved: no row sizes it | `r3-fighter-hit-chance.md` |
| `pilot_armour_factor` | 0.75 | Looked at, not moved: no source | `r3-fighter-hit-chance.md`, `r3-us-losses-named-misses.md` |
| `self_sealing_tanks_factor` | 0.75 | Looked at, not moved: no source | the same |
| `anti_aircraft_hit_chance` | 0.05 | Looked at, not moved, in a round written from numbers already in hand, with no grid. The guns' share of the kills over the Enterprise force, about 43%, is consistent with the one weak row (3953, about half, Coral Sea, low); no row sizes the chance | `r5-anti-aircraft-hit-chance.md` |
| `anti_aircraft_false_claim_chance` | 0.05 | Looked at, not moved: left at the low end of what the rows give for the Eastern Solomons' ships' guns (255; 215 and others) | `r4-claims.md` |
| `bomb_false_claim_chance` | 0.1 | Looked at, not moved: one figure carries two kinds of claim that the record prices differently: about 1 per hit on ships, 20 or more per aircraft on the ground. No judged measure reads it | `r4-claims.md` |
| `patrol_before_attack_share` | 0.4 | Placeholder, inside the range the rows give, 0.22 to 0.61 (3979 to 3981). It was set inside that range before any result was read, and may move only inside it | Decision 0087 |
| `patrol_drawn_off_share` | 0 | Fitted, not checked: an endpoint placeholder "chosen with both air battles' results known" (decision 0084, finding 5). Since then 0 is "an end the rows support, not sized" (round 3): 12 of the 30 of Simpler's study who engaged met the escort first, at or above the proportional split (decision 0087, 11) | Decisions 0080, 0084, 0087 |
| `anti_aircraft_before_attack_share` | 0 | Fitted, not checked: an endpoint placeholder "chosen with both air battles' results known" (decision 0084, finding 5). Events 222 and 223 support guns that fire during the dive; no source sizes it | Decisions 0079, 0084 |
| `bomb_hit_chance` | 0.2 | Looked at, not moved: named in Rabaul round 1, no value tried. Read by every raid on ships and on fields | `rabaul-r1-base-hit-aircraft.md` |
| `torpedo_hit_chance` | 0.15 | Placeholder. No round has looked at it | The file |
| `[bomb_system_hit]` | flight deck 0.5, propulsion 0.2, fire control 0.1, main battery 0.2 | Placeholders, four entries. No round has looked at them | The file |
| `strike_contact_max_age_pulses` | 0 | Placeholder (decision 0026) | The file |
| `extended_reach_share` | 0.25 | Placeholder (decision 0059) | The file |
| `ditch_chance_per_radius_beyond` | 2 | Placeholder. Under decision 0084 the ditchings of Ryujo's raid are not evidence for it. A fuel round waits on research into a radius with a bomb load | Decision 0084; `docs/open-work.md` |
| `crew_recovered_short_of_home` | 0.7 | No battle reads it: the Eastern Solomons counts no pilots, and scenario 6 orders no sortie beyond a radius | The file |
| `crew_recovered_over_own_base` | 0.6 | Placeholder, no check battle. The record has 12 of 23, a sample that favours those who lived | The file |
| `crew_recovered_beside_own_force` | 0.95 | No battle reads it: the Eastern Solomons counts no pilots, and scenario 6 has no ships. The record has 19 recovered and none stated lost | The file |
| `crew_recovered_open_sea` | 0.5 | No battle reads it: the Eastern Solomons counts no pilots, and in scenario 6 every group is at a base, which never lacks a deck. The record has 6 to 9 crews recovered and 4 to 7 lost (`docs/research/oob/ditching-and-rescue.md`, section 2) | The file |
| `base_supply_destroyed_per_hit_t` | 50 | Placeholder (decision 0034). No row, no measure | The file |
| `[base_hit] supply` | 0.3 | Placeholder. No row, no measure | `rabaul-r1-base-hit-aircraft.md` |

**Counts** (a table entry counts as one figure, so `[bomb_system_hit]` is four), the same sorting as `surface-and-search-figures-closed.md`: sized from rows 2, fitted and not checked 2, looked at and not moved 7, placeholders 12, read by no battle 3; 26 in all. *Corrected Nov 13, 2026, on the coordinating architect's ruling that the figure notes' sorting is the one: this page had called `anti_aircraft_false_claim_chance` and `patrol_before_attack_share` "placeholder, bounded" and counted `[bomb_system_hit]` as one.*

## `air_upkeep.toml`: wear and carrier repair

Rabaul round 3 looked at wear and moved nothing.

| Figure | Value | Status |
| --- | --- | --- |
| `wear_base_chance` | 0.05 | Placeholder. The rows give the American types' operational losses (NACS, 1262 to 1299), and the rules are already that size: the F4Fs about a third over their rows (8.2 a run against about 6.1), looked at and not moved |
| `wear_chance_per_100_nm` | 0.02 | Placeholder. The rows pull it opposite ways by side, about 0.035 for the SBDs and about 0.007 for Rabaul's bombers. That is the rule's shape, not its size. 0.035 was declined |
| `write_off_share` | 0.2 | Placeholder. Read alike by every sortie, so it cannot move toward one type's rows without moving another away from its own |
| `carrier_repairs_per_day` | 2 | No battle reads it: the Eastern Solomons does not load air upkeep, and scenario 6 has no carriers |

Record: `rabaul-r3-wear.md`.

## `fatigue.toml`

Rabaul round 2 looked at fatigue and moved nothing.

| Figure | Value | Status |
| --- | --- | --- |
| `air_fatigue_per_sortie` | 3 | Placeholder. No row sizes it |
| `air_fatigue_per_100_nm` | 2 | Placeholder. The forms' raid sizes cannot size it: the units sent about 18 a raid whatever their rest, and in the rules a group sends all it can fly |
| `air_rest_per_pulse` | 5 | Placeholder. No row sizes it |
| `force_fatigue_per_pulse_alert` | 5 | No battle reads it: scenarios 4 and 5 do not load fatigue, and scenario 6 has no task forces |
| `force_rest_per_pulse` | 3 | No battle reads it, for the same reason |

Rows 3976 and 3977 are named and not tuned to: 80% available the day after an all-out attack, and 50% on the third. They are one officer's recollection, which he ties to supply. Record: `rabaul-r2-fatigue.md`.

## `air_supply.toml`

No round has looked at these, and no row sizes them. The only rows on supply are American fuel: fighter-hours per transport load (3975) and the 4,000 gallons (3978, low). There is nothing for bombs, and nothing for Japanese supply beyond "lack of supply" (`docs/research/oob/outside-research-nov-13.md`, "What each slow figure now has"). No measure reads base supply.

| Figure | Value | Status |
| --- | --- | --- |
| `sortie_supply_t` | 0.5 | Placeholder |
| `sortie_supply_t_per_100_nm` | 0.25 | Placeholder |
| `daily_supply_t_per_aircraft` | 0.2 | Placeholder |
| `repair_supply_t` | 1 | Placeholder |

## Scenario 6's own figures

These are scenario figures, set in `data/scenarios/06_rabaul_guadalcanal.toml` and read by no other battle. They are not tuned (tuning rule 8).

| Figure | Value | Status |
| --- | --- | --- |
| Henderson Field `aircraft_repairs_per_day` | 3 | Placeholder. No row sizes it. On 25 August 5 of 31 aircraft were under repair (455), against about 1.3 in the rules. This is a cause of the SBDs' miss on day 21 |
| Rabaul `aircraft_repairs_per_day` | 4 | Placeholder. No row sizes it |
| Henderson Field `supply_t` | 4,000 | Placeholder (decision 0042) |
| Rabaul `supply_t` | 10,000 | Placeholder (decision 0042) |
| Rabaul's replacements | 2 bombers and 2 fighters a day, each with a pilot | Placeholder: invented, not from a row (decision 0082). The arrivals are undated totals (air_unit_facts 334 to 337) |
| The Allied rest threshold | 40, on groups 10, 13, 15 and 16 | Placeholder: invented, not from a row (decision 0082) |
| The dive bombers' search | 300 to 030 degrees, 200 miles | A reading, not a row: the SBD-3's search distance as its rows record it flown elsewhere (decision 0082) |

For repair, replacement and supply, `docs/research/oob/outside-research-nov-13.md` ("What each slow figure now has") records what was looked for: repair time, none, only that major repair went to Australia; pilots, nothing that sizes replacement; supply, US fuel only.

`search.toml` is read by the surface battles and the air battles alike. None of its figures is air-only, and **none of them had a tuning round.** Finding and searching were set otherwise:
- the Japanese search at the Eastern Solomons, from the record (decision 0073);
- each aircraft type's own search radius (decision 0075);
- finding and surprise in the surface fight, by round 0 (`r0-finding-and-surprise.md`), on the figures of `surface_combat.toml`.

The figures of reports and contacts in `search.toml` stay placeholders.

## Named misses of the air battles

| Battle | Miss | Cause named | Recorded in |
| --- | --- | --- | --- |
| Eastern Solomons | US aircraft lost to enemy action (13 to 14) and to all causes (20 to 21) | No return fire from bombers; a search never meets fighters; no write-off for combat damage; no deck closed for a time | `r3-us-losses-named-misses.md`; step 3 list |
| Eastern Solomons | The Japanese claims ratio, at the edge | Its divisor is the loss miss above | `r4-claims.md` |
| Eastern Solomons | The Marines' claims, 16 to 21 | Out of reach under the one-shot rule | `r4-claims.md` |
| Rabaul | Japanese bombers lost to enemy action (29 against 12) | No rule for height and warning, and no row counts the fighters in contact (the air plan's C3) | `r3-fighter-hit-chance.md` |
| Rabaul | SBDs ready on day 21 (30 against 22) | Aircraft under repair for days (455, against the repair placeholder); strikes at ships that scenario 6 cannot fly; losses on other flights that do not go with distance | `rabaul-r3-wear.md`, handed on from `rabaul-r1-base-hit-aircraft.md` |
| Rabaul | Raid sizes by day: 40 against 23 on day 5, almost nothing against 20 and 16 on days 23 and 24 | The units' sortie counts, a scenario order not changed after tuning began; no rule for a strike of a given size | `rabaul-r2-fatigue.md`; step 3 list |
| Rabaul | The F4Fs' write-offs over their rows (8.2 against about 6.1) | Looked at through `wear_base_chance` and not moved | `rabaul-r3-wear.md` |
| Rabaul, reported | Japanese bombers forced down (10 against the forms' 5 to 6) | The shape of wear: a straight line in distance cannot fit both a 400 nm search and a 1,112 nm raid | `rabaul-r3-wear.md`; step 3 list |
| Rabaul, reported | Japanese claims on the ground (900 for each hundred destroyed) | The record's ratio is over two or three aircraft | `rabaul-r1-base-hit-aircraft.md` |
| Rabaul, not judged | The escorts' claims (about 159 a run against about 100 on the forms) | The same shared chance as the Americans' | `r4-claims.md` |

The rules missing behind these are on `docs/open-work.md`, in the lines for step 3 named by air round 3 and by Rabaul rounds 2 and 3.
