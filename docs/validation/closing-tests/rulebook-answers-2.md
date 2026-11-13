# Rulebook-only test, sitting 2: the model's answer

Scenarios 17a (case 1) and 18. Model `haiku`, Nov 13, 2026. Word for word as handed back, but for the scratch folder's long path, written [scratch]. The prompt is in `rulebook-sittings.md`, the marking in `../closing-tests.md`.

---

# Scenario 17a: which enemy ship a ship fires at

## Setup and rules used

- **Scenario and figures.** `scenarios/17a_target_choice.toml` names `reference-17a/` (`surface_combat.toml`, `ship_classes.csv`). Force X (Japanese, TF1) is ships 1 and 2 (class 1 cruisers) and 3 and 4 (class 2 destroyers). Force Y (Allied, TF2) is ship 5 (class 1) and ships 6, 7 and 8 (class 2). Both forces have readiness `battle`.
- **Class figures** (`ship_classes.csv`, columns read by name):
  - Class 1 cruiser: `main_guns` 8, `gun_rounds_per_min` 3, `gun_rounds_per_gun` 100, `gun_damage` 20, `torpedo_tubes` 8, `torpedo_tubes_one_side` 8, `torpedo_damage` 200, `toughness` 400, `radar` 0.
  - Class 2 destroyer: `main_guns` 4, `gun_rounds_per_min` 6, `gun_rounds_per_gun` 150, `gun_damage` 5, `torpedo_tubes` 8, `torpedo_tubes_one_side` 8, `torpedo_damage` 200, `toughness` 100, `radar` 0.
  - Class 3 cargo ship (question 4): `main_guns` 0, `torpedo_tubes` 0, `gun_damage` 0, `torpedo_damage` 0, `toughness` 300, `radar` 0.
- **Figures** (`reference-17a/surface_combat.toml`): `gun_hit_chance_day` 0.05, `torpedo_hit_chance_day` 0.02, `sighting_chance_day` 1, `radar_sighting_bonus` 0.2, `not_alert_sighting_factor` 0.5, `round_minutes` 5, `max_rounds` 6, `surprised_target_factor` 1.5, `lit_target_factor` 2, `system_hit_chance_per_point` 0.06, `unready_gun_share` 1, `unready_torpedo_share` 1, experience strengths 0.
- **Darkness.** The morning pulse is not the night pulse, so the fight is light (chapter 1, Darkness: dark "exactly in the night pulse").
- **Alert and ready.** Battle readiness is always alert (chapter 3, What alert is; glossary "Alert"), and battle readiness is ready (chapter 4, Ready). So `alert_factor` is 1 and the fire shares do not bite.
- **Round 1 sighting** (chapter 4, Sighting): `chance = min(1, (base + radar_bonus) x alert_factor x experience_factor) x fatigue_factor`. Base 1 (day), radar bonus 0 (no ship has radar), alert factor 1, experience factor 1 (light), fatigue factor 1 (the scenario does not name `fatigue.toml`; chapter 4 says "else 1"). So `chance = 1.000000` for each force. A chance of 1 always succeeds (chapter 1, How a chance is rolled), so both forces sight in round 1. Nobody is surprised (chapter 4, Surprise), and all eight ships fire in round 1 (chapter 4, Who fires).

## Question 1: target of each ship in round 1

Chapter 4, Who fires:
- **Guns.** A ship fires at enemy ships of its own kind. If none, at the nearest more important kind. Among the targets it takes index `p mod n`, where `p` is the firing ship's place among ships of its own kind in its force and `n` is the number of targets, in ship ID order.
- **Torpedoes.** A ship fires at the enemy ships of the most important kind afloat, taking index `i mod n`, where `i` is the firing ship's place in its force (all ships count, armed or not).
- **Place convention.** The rulebook does not say whether place counts from 0 or 1 ("p modulo their number", no base given). I take 0-based as the primary reading. The 1-based result is in the last column.

Kinds present: Y has cruiser 5 and destroyers 6, 7, 8. X has cruisers 1, 2 and destroyers 3, 4. The most important enemy kind is cruiser for both sides.

| Ship | Guns: own-kind enemies | Place, index, target (0-based) | Torpedoes: enemy kind | Place, index, target (0-based) | 1-based alternative: guns / torpedoes |
|---|---|---|---|---|---|
| 1 (X, cruiser) | Cruisers of Y = [5] | p=0, 0 mod 1 = 0, gives 5 | Cruisers of Y = [5] | i=0, gives 5 | 5 / 5 |
| 2 (X, cruiser) | [5] | p=1, 1 mod 1 = 0, gives 5 | [5] | i=1, gives 5 | 5 / 5 |
| 3 (X, destroyer) | Destroyers of Y = [6, 7, 8] | p=0, index 0, gives 6 | [5] | i=2, gives 5 | 7 / 5 |
| 4 (X, destroyer) | [6, 7, 8] | p=1, index 1, gives 7 | [5] | i=3, gives 5 | 8 / 5 |
| 5 (Y, cruiser) | Cruisers of X = [1, 2] | p=0, index 0, gives 1 | Cruisers of X = [1, 2] | i=0, index 0, gives 1 | 2 / 2 |
| 6 (Y, destroyer) | Destroyers of X = [3, 4] | p=0, index 0, gives 3 | [1, 2] | i=1, index 1, gives 2 | 4 / 1 |
| 7 (Y, destroyer) | [3, 4] | p=1, index 1, gives 4 | [1, 2] | i=2, index 0, gives 1 | 3 / 2 |
| 8 (Y, destroyer) | [3, 4] | p=2, index 0, gives 3 | [1, 2] | i=3, index 1, gives 2 | 4 / 1 |

**Reasons.**
- Ships 1 and 2 have one enemy cruiser (5), so both fire guns and torpedoes at 5.
- Ships 3 and 4 fire guns at Y's destroyers (own kind) and torpedoes at cruiser 5 (the most important kind in Y).
- Ships 5 to 8 fire guns at X's own kinds (cruisers for 5, destroyers for 6 to 8) and torpedoes at X's cruisers, since cruisers are the most important kind in X.
- Every armed ship fires its gun salvo first and its torpedo salvo second (chapter 4, Who fires). Destroyers have eight tubes, so they fire torpedoes too.

## Question 2: one gun salvo and one torpedo salvo, round 1

Chapter 4, Shots and The hit chance.

**Cruiser gun salvo.**
- `full = gun_rounds_per_min x (main_guns x round_minutes) = 3 x (8 x 5) = 120`.
- Main battery factor 1, unready share 1. Ammunition is `8 x 100 = 800`, not limiting.
- Shots = `min(120, 800) = 120`.
- `chance = 0.05 x 1 (surprised) x 1 (lit) x 1 (fire control) x 1 (experience) = 0.050000`.
- Expected hits = 120 x 0.050000 = **6.000000**.

**Destroyer gun salvo.**
- `full = 6 x (4 x 5) = 120`. Ammunition `4 x 150 = 600`, not limiting. Shots = **120**.
- Chance = **0.050000**. Expected hits = **6.000000**.

**Torpedo salvo** (cruiser or destroyer, eight tubes; B equals T so one salvo of eight).
- Shots = `8 x 1 x 1 = 8`.
- `chance = 0.02 x 1 x 1 x 1 x 1 = 0.020000` (fire-control factor is 1 for a torpedo).
- Expected hits = 8 x 0.020000 = **0.160000**.

## Question 3: points per hit

Chapter 4, Where a hit lands: `points = 100 x weapon damage / toughness`, one product and one truncating division.

| Weapon | On class 1 (toughness 400) | On class 2 (toughness 100) | On class 3 (toughness 300) |
|---|---|---|---|
| Cruiser gun (`gun_damage` 20): 2000 | 2000 / 400 = **5.000000** | 2000 / 100 = **20.000000** | 2000 / 300 = 6.666666... cut to **6.666666** |
| Destroyer gun (`gun_damage` 5): 500 | 500 / 400 = **1.250000** | 500 / 100 = **5.000000** | 500 / 300 = 1.666666... cut to **1.666666** |
| Torpedo (`torpedo_damage` 200): 20000 | 20000 / 400 = **50.000000** | 20000 / 100 = **200.000000** | 20000 / 300 = 66.666666... cut to **66.666666** |

Cutting is toward zero (chapter 1, Fixed-point numbers).

## Question 4: changed force (X = ships 1, 2, 3; Y = ships 4, 5, 6)

**Assumption.** The question lists ships 1 to 6 only, so I take ships 7 and 8 as absent. Readiness is as in 17a, so both forces sight in round 1 with chance 1 and all live armed ships fire.

**Kinds.** X: ship 1 is a transport (class 3, no guns, no tubes); ships 2 and 3 are cruisers. Y: ships 4, 5, 6 are all cruisers.

**Ship 1 (transport)** fires nothing. It has no guns and no tubes, so its salvos have no shots and are not fired (chapter 4: "an unarmed ship fires nothing"; "A salvo with no shots is not fired"). Its target is not used.

**Ships 2 and 3 (X cruisers).** Own-kind place among X cruisers: ship 2 p=0, ship 3 p=1. Force place for torpedoes: ship 2 i=1, ship 3 i=2 (ship 1 is place 0).
- Guns, Y cruisers [4, 5, 6]: ship 2 index 0 gives **ship 4**; ship 3 index 1 gives **ship 5**.
- Torpedoes, most important enemy kind is cruiser, Y = [4, 5, 6]: ship 2 index 1 mod 3 gives **ship 5**; ship 3 index 2 mod 3 gives **ship 6**.

**Ships 4, 5, 6 (Y cruisers)** fire at X's cruisers [2, 3] (both guns and torpedoes, since cruiser is the most important kind present in X).
- Guns: ship 4 (p=0) gives **ship 2**; ship 5 (p=1) gives **ship 3**; ship 6 (p=2) index 2 mod 2 = 0 gives **ship 2**.
- Torpedoes: ship 4 (i=0) gives **ship 2**; ship 5 (i=1) gives **ship 3**; ship 6 (i=2) index 0 gives **ship 2**.

1-based alternative: X2 guns to 5, X3 guns to 6, X2 torpedoes to 6, X3 torpedoes to 4; Y4 guns to 3, Y5 guns to 2, Y6 guns to 3; Y4 torpedoes to 3, Y5 torpedoes to 2, Y6 torpedoes to 3.

---

# Scenario 18: the night fight, force A (TF1, class 8) against force B (TF2, class 1)

## Setup and figures

- **Ships.** Force A is ships 1, 2, 3, each class 8: `torpedo_tubes` 8, `torpedo_tubes_one_side` 4 (T = 8, B = 4, T - B = 4), `torpedo_damage` 200, `toughness` 400, eight guns. Force B is ships 4, 5, 6, each class 1 (T = B = 8).
- **Figures** (`reference-18/surface_combat.toml`): `sighting_chance_night` 0.5, `radar_sighting_bonus` 0.2, `not_alert_sighting_factor` 0.5, `torpedo_hit_chance_night` 0.04, `surprised_target_factor` 1.5, `lit_target_factor` 2, `degraded_factor` 0.5, `system_hit_chance_per_point` 0.06, `unready_gun_share` 0.5, `unready_torpedo_share` 0.5, experience strengths 0, `round_minutes` 5.
- **Readiness.** TF1 is `battle`: always alert and ready. TF2 has no readiness line, so it is `normal` ("It is normal by default", chapter 3, Readiness by order). It has no contacts at the start, so it is not alert, and so not ready.
- **Darkness.** The night pulse is dark (chapter 1).
- **Fatigue.** Not named by the scenario, so the factor is 1.
- **Sighting, round 1.** Force A: `min(1, (0.5 + 0) x 1 x 1) = 0.500000`. Force B (not alert): `min(1, 0.5 x 0.5) = 0.250000`.
- **Torpedo hit chance, night, unsurprised, not lit:** `0.04 x 1 x 1 x 1 x 1 = 0.040000`.
- **Tube sides.** The first salvo fires B = 4 tubes; the second fires T - B = 4 tubes in the next round with torpedo shots to fire (chapter 4, Torpedo shots). Each tube fires once.
- **Targets.** A1 to B1 (ship 4), A2 to B2 (ship 5), A3 to B3 (ship 6), each round, by the place rule (index = place mod 3, B cruisers).

## Question 5: target not surprised

Conditions: A sighted in round 1 (0.500000) and B sighted in round 1 (0.250000), so B is not surprised. Joint chance 0.125.

**Round 1.**
- Tubes firing: 4 (first side).
- Shots: 4 x 1 x 1 = **4**.
- Chance: **0.040000** (surprised factor 1; B has no fire damage at the start of round 1, so lit 1; fire control 1; experience 1).
- Expected hits: 4 x 0.040000 = **0.160000 per ship** (0.480000 for three ships).

**Round 2.**
- B was fired on in round 1, so it has sighted (no roll) and is not surprised.
- Tubes firing: 4 (second side, T - B).
- Shots: **4**.
- Chance: **0.040000**, or **0.080000** if B has fire damage at the start of round 2 (lit factor 2). A's guns fire before its torpedoes in round 1, and a gun hit lands on fire with chance 0.25. For one A ship's 120 gun shots at 0.03, the chance B has at least one fire hit is about 1 - (1 - 0.0075)^120, roughly 0.595. This is my approximation (shots treated as independent; only A's guns considered).
- Expected hits: 4 x 0.04 = **0.160000 per ship** if not lit; 0.320000 if lit.

**After round 2:** nothing is left. Both sides are spent and `torpedoes_ready` is cleared (chapter 4, Torpedo shots: "After any other salvo ... both are cleared"). There are no reloads.

## Question 6: target force surprised in round 1

- Condition: A sighted and B did not (0.5 x 0.75 = 0.375 given A sighted). B is surprised, since A has torpedo shots.
- Tubes: 4. Shots: **4**.
- Chance: 0.04 x 1.5 x 1 x 1 x 1 = **0.060000**.
- Expected hits: 4 x 0.060000 = **0.240000 per ship**.
- Round 2: B was fired on in round 1, so it is not surprised; same as question 5 (0.040000 chance, 0.160000 expected).

## Question 7: force A not ready when the fight began

- A is at normal readiness and not alert, so it is not ready. `unready_torpedo_share` 0.5 applies.
- **Caveat on sighting.** A's round 1 sighting chance falls to 0.5 x 0.5 = 0.250000, because a not-alert force reads `not_alert_sighting_factor` (chapter 3, What alert is). Chapter 4 says sighting is "unchanged" by readiness, which conflicts. I follow the sighting factor. The figures below assume A sighted.
- **Round 1.** Tubes 4. Shots = 4 x 1 x 0.5 = **2.000000**. Chance **0.040000** (as question 5). Expected = **0.080000**.
- **Round 2.** Tubes 4 (second side). Shots = 4 x 1 x 0.5 = **2.000000**. Chance **0.040000** (lit caveat as question 5). Expected = **0.080000**.
- Nothing left afterwards.

## Question 8: torpedo mounts degraded before round 1 (A ready)

- `torpedo_mount_factor` = `degraded_factor` 0.5 (chapter 4, Named systems).
- **Round 1.** Tubes 4. Shots = 4 x 0.5 x 1 = **2.000000**. Chance 0.040000. Expected = **0.080000**.
- **Round 2.** Tubes 4. Shots = **2.000000**. Chance 0.040000 (lit caveat). Expected = **0.080000**.
- Nothing left afterwards.

## Question 9: a torpedo hit from her on a class 1 cruiser

- **Points.** `100 x 200 / 400 = 50.000000`.
- **System chance.** `min(1, 50.000000 x 0.060000) = min(1, 3.000000) = 1.000000`. A system is always hit (chapter 4, Each hit also makes one system draw, always).
- **Which system.** Torpedo table: propulsion 0.8, fire control 0.1, main battery 0.1, torpedo mounts absent (band width 0). Bands: propulsion [0, 0.8), fire control [0.8, 0.9), main battery [0.9, 1.0).
- **Result.** Chance a system steps down one level: **1.000000**. Propulsion 0.8, fire control 0.1, main battery 0.1, torpedo mounts 0. The hit also adds 50.000000 to the flotation track, since torpedo hits always land on flotation.

---

## Where I had to assume or infer

- **Place convention** (questions 1 and 4). "it takes the ship in place (p modulo their number), in ship ID order, where p is the firing ship's place among the ships of its own kind in its force". I took 0-based and gave 1-based results.
- **Ships 7 and 8 in question 4.** "force X changed to ships 1 ... 2 and 3 ... and force Y to ships 4, 5 and 6". I assumed 7 and 8 are absent.
- **Transport's target** (question 4). "if none is afloat, at those of the nearest more important kind" does not define "nearest". It does not matter, since the transport fires nothing.
- **Movement and meeting** (17a). I did not read chapter 2, so I took the fight as given by the question.
- **Frozen unready figures disagree with the rulebook.** Chapter 4 says the frozen step 1 file has unready shares at 1 ("0.5 in the live file, 1 in the frozen step 1 file"). But `reference-18/surface_combat.toml` (headed as a frozen copy) has both at 0.5, and `reference-17a/surface_combat.toml` has both at 1. I used the file values, so 0.5 for scenario 18. This should go to the architect.
- **Sighting under not-ready** (question 7). Chapter 4: "The chance to hit, sighting, surprise, the alarm and breaking off are unchanged." Chapter 3: `alert_factor` is `not_alert_sighting_factor` if not alert. A not-ready force is not alert, so these conflict. I used the sighting factor.
- **"Target not surprised"** (question 5). I read it as B having sighted in round 1, with A also sighted (otherwise A fires nothing). I gave joint chances (A 0.5, B 0.25, both 0.125).
- **Round 2 lit factor** (question 5). The chance is 0.040000 unless B has fire damage at the start of round 2. The 0.595 figure is my approximation, using A's guns only.
- **Damage timing.** I took chapter 4's "Everything in a round is worked out from the state at its start, and damage is applied at its end" to mean round 1 torpedo chances do not see round 1 gun hits.
- **TF2 readiness in scenario 18.** "It is normal by default" (chapter 3). I took TF2 as not alert and not ready at the start, since it has no contacts.
- **Fatigue.** Not in play, since neither scenario names `fatigue.toml` (chapter 1, Where figures come from; chapter 4: "else 1").
- **`search.toml`.** Not read. Alert from belief needs a contact, and neither scenario has one, so it does not affect these answers.
- **Experience.** Strength is 0 in both frozen files, so the experience factor is 1 throughout.

---

## Files read

All under `[scratch]/ct_rulebook/sitting2/`:

1. `rules/README.md`
2. `rules/04-surface-combat-and-damage.md`
3. `rules/03-search-contacts-and-claims.md`
4. `rules/01-the-day-orders-and-what-a-side-knows.md`
5. `glossary.md`
6. `scenarios/17a_target_choice.toml`
7. `scenarios/18_torpedo_sides.toml`
8. `reference-17a/surface_combat.toml`
9. `reference-17a/ship_classes.csv`
10. `reference-18/surface_combat.toml`
11. `reference-18/ship_classes.csv`

Also a directory listing (`ls -R`) of the sitting2 folder only, to find file names. Two Read attempts failed (a directory, and a guessed file name that does not exist). No other file was opened.
