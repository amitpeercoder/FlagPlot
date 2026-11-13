# Rulebook-only test, sitting 3: the model's answer

Scenarios 19a and 19b. Model `haiku`, Nov 13, 2026. Word for word as handed back, but for the scratch folder's long path, written [scratch]. The prompt is in `rulebook-sittings.md`, the marking in `../closing-tests.md`.

---

# Hand calculation: scenarios 19a and 19b, the morning pulse of day 1

## 0. Conventions and common facts

**Rounding.** Every product and quotient is cut toward zero at six decimals (rules/01, "Fixed-point numbers"). Where a probability is a product of several chances, I give the exact value and cut it to six decimals. A chain of cut products could differ in the last digit.

**Chances per shot** (reference-19a and reference-19b `air_combat.toml`, identical in both):
- Fighter shot: `fighter_hit_chance` = 0.3. Each fighter type has `pilot_armour` 0 and `self_sealing_tanks` 0 (`aircraft_types.csv`, types 3 and 4), so the protection factor is 1 and the chance stays 0.3 (rules/05, "Protection").
- Gun shot: `anti_aircraft_hit_chance` = 0.05.
- Bomb: `bomb_hit_chance` = 0.2.
- `anti_aircraft_before_attack_share` (b) = 1 and `patrol_drawn_off_share` (s) = 1 as the files stand. `patrol_before_attack_share` = 1 (frozen figure).

**Ships** (`ship_classes.csv`, identical in both):
- Class 4, carrier: toughness 600, aircraft_capacity 36, anti_aircraft 0.
- Class 5, carrier with guns: toughness 600, aircraft_capacity 48, anti_aircraft 20.

**Types** (`aircraft_types.csv`, identical in both):
- Type 3, dive bomber: role bomber, strike_range_nm 250, bomb, attack_damage 60.
- Type 4, fighter: strike_range_nm 250, no attack.

**Geography.** Allied task force 1 (TF1, ship 1, class 5) is at hex (4,0). Japanese task force 2 (TF2, ship 2, class 4) is at hex (9,0). Air distance (rules/05, "Air distance"): dq = 4 − 9 = −5, dr = 0, so 40 × sqrt(25) = 200 nm. Reach on normal = strike radius = 250 (rules/05, "Radius and reach"), so 200 ≤ 250 and the target is in reach.

**Contact.** The Japanese side holds one contact at hex (4,0), 1 ship, kind carrier. The scenario gives no age (see section 8). Strike qualification needs age ≤ `strike_contact_max_age_pulses` = 1 (rules/05, "Naval strike: the contact"). Any age of 0 or 1 qualifies.

**Figures not loaded.** The scenarios name only map, ship_classes, port_sizes, aircraft_types, search, surface_combat and air_combat. They do not name air_upkeep, air_supply or fatigue. So there is no fatigue, no wear, no supply cost, no repair, no arrival and no replacement (rules/01, "Where figures come from and when a rule is loaded"). Fatigue cuts nothing, and the sortie notes do not change the outcome.

**Pilots.** Neither scenario gives its groups pilots, so pilots are not tracked and the count of flyable aircraft is the ready count (rules/05, "The order of the limits", step 2).

**Ship systems.** Both carriers are assumed to have a working flight deck, so the deck factor is 1 (rules/05, step 3).

**Movement and fights.** No task force has a route, so none moves (rules/04, "Avoiding for the pulse": "a force with no route stays"). The forces are on different hexes, so no fight is held (rules/04, "Meeting"). No surface combat occurs in the morning pulse. No search group exists in either scenario.

**Running out of fuel.** A sortie's one-way distance is 200 nm, which is not above the radius of 250, so nothing rolls (rules/05, "Running out of fuel", step 1). No aircraft ditches.

**Pulse order for day 1.** The night pulse runs repair, arrivals, replacements, supply and stand-down. None of these applies here (section 0, "Figures not loaded"). The morning pulse then runs alert from belief (TF2 is 5 hexes from the contact, beyond `alert_contact_distance_hexes` = 3, so nothing happens), then search (none), strike decisions, air combat, the wear phase (nothing rolls), and movement (nothing).

**The split of fighters** (rules/05, "The split"). With F the flyable count, patrol = cut(F × patrol share) and escorts = F − patrol.

**The raid fire formulas** (rules/05, "The patrol against the escorts and the bombers"). With P the raid's share of the patrol, E the escorts and B the bombers at the start of the step:
- D = cut(min(P, E) × s)
- R = P − D
- X = min(cut(R × (E − D) / (E − D + B)), E − D)
- Patrol shots at escorts = D + X. Patrol shots at bombers = R − X.

All these shots are before the attack when the before-attack share is 1.

---

## 1. Scenario 19a: which groups launch, and the escorts

**Groups as the file gives them:**
- Group 1, Allied, fighters (type 4), ship 1, ready 6, mission combat air patrol, patrol share default 1.
- Group 2, Japanese, dive bombers (type 3), ship 2, ready 12, mission naval strike.
- Group 3, Japanese, fighters (type 4), ship 2, ready 4, mission combat air patrol, patrol share set to 0 by order.

**Strike decision for group 2** (rules/05, "When and who"): mission naval strike ✓; ready 12 ✓; has a position (ship 2) ✓; not dark (morning pulse) ✓; type 3 attacks with a bomb ✓; flyable count = 12 × 1 (deck working) = 12 ✓.

**Contact choice** (rules/05, "Naval strike: the contact"): the contact is within age limit, within reach (200 ≤ 250), and its kind (carrier) is on the default list, so it qualifies. Group 2 launches **12 dive bombers** at hex (4,0). It records `StrikeLaunched`.

**Group 3 (Japanese fighters, CAP, share 0).** Flyable F = 4 ready × deck 1 = 4. Patrol = cut(4 × 0) = 0. Escorts = 4 − 0 = **4**. The escorts go with the one raid from ship 2 (only one raid launches from that place). The raid's hex is 200 nm away, within the fighter type's reach of 250, so they go and `EscortsSent` is recorded (rules/05, "Which raid the escorts join, and the reach").

**Why the escorts are group 3's four fighters.** The patrol share of 0 keeps none over the group's own hex, so all four are escorts. Group 3's order gave the share.

**Group 1 (Allied, CAP).** Flyable F = 6 × deck 1 = 6. Patrol = cut(6 × 1) = **6**. Escorts = 0. Group 1 does not strike.

**Raid.** One raid, from ship 2 at (4,0). Bombers B = 12. Escorts E = 4. Patrol over TF1's hex P = 6. There is one raid, so the raid's share is the whole patrol: P = 6 (rules/05, "The patrol divided among raids"). Raid aircraft: 12 + 4 = 16.

---

## 2. Scenario 19a: the air combat over TF1

**(a) Escorts fire at the patrol** (rules/05, "Escorts against the patrol"). Shots = escort count = 4. Chance = 0.3 × 1 (the target is group 1, type 4) = 0.3. Expected hits = 4 × 0.3 = **1.200000**.

**(b) The patrol fires at the escorts and bombers.** D = cut(min(6, 4) × 1) = 4. R = 6 − 4 = 2. E − D = 0, so X = cut(2 × 0 / (0 + 12)) = 0.
- At escorts: shots = D + X = 4. Chance = 0.3 × 1 (first escort group, type 4) = 0.3. Expected hits = **1.200000**.
- At bombers: shots = R − X = 2. Chance = 0.3 × 1 (first strike group, type 3) = 0.3. Expected hits = **0.600000**. See (c).

**(c) The patrol at the bombers.** Shots = 2. All before the attack, since the before-attack share is 1 (k = cut(2 × 1) = 2, after = 0). Chance 0.3. Expected hits **0.600000**.

**(d) The anti-aircraft guns in all.** TF1 has one ship afloat, ship 1, with anti_aircraft 20. Shots = **20**. Chance = 0.05 × 1 (first strike group, type 3) = 0.05. Expected hits = 20 × 0.05 = **1.000000**. The guns fire only at bombers left after the patrol's hits (rules/05, "Anti-aircraft fire").

---

## 3. Scenario 19a: the guns' split, and the bombers, for b = 1, 0.5, 0.33, 0

**Common quantities.**
- Bombers after the patrol's before-attack hits: 12 − 0.6 = **11.4** expected.
- Guns: k = cut(20 × b) are before the attack, the rest 20 − k are after.
- Guns before the attack remove 0.05 × k expected bombers.
- Expected attack shots = 11.4 − 0.05k.
- Expected bomb hits on the carrier = 0.2 × expected attack shots.
- Expected bombers lost to guns: before = 0.05k, after = 0.05(20 − k). Total = 1.0.

The cap that a hit cannot exceed the bombers left only binds at a probability of about 1e-8 for b = 1, so it is ignored.

| b | k before | 20−k after | Expected attack shots | Expected bomb hits on carrier | Lost to guns before | Lost to guns after | Lost to guns, total |
|---|---|---|---|---|---|---|---|
| 1 | 20 | 0 | 10.400000 | 2.080000 | 1.000000 | 0.000000 | 1.000000 |
| 0.5 | 10 | 10 | 10.900000 | 2.180000 | 0.500000 | 0.500000 | 1.000000 |
| 0.33 | 6 (cut of 6.6) | 14 | 11.100000 | 2.220000 | 0.300000 | 0.700000 | 1.000000 |
| 0 | 0 | 20 | 11.400000 | 2.280000 | 0.000000 | 1.000000 | 1.000000 |

The attacks that run after the guns' after-attack fire still attack in full. Losses after the attack do not reduce what attacked (rules/05, step 4a: "whatever the attack did stands").

---

## 4. Scenario 19a: damage from one bomb hit on the carrier

Damage points = 100 × attack_damage / toughness = 100 × 60 / 600 = 6000 / 600 = **10.000000 points** (rules/04, "Where a hit lands"; rules/05, "What a hit does").

The points go to one track. A bomb hit draws its track as a gun does: flotation if the draw is below `gun_hit_flotation` = 0.75, otherwise fire (rules/05, "What a hit does", the bomb's track). So 10 points go to flotation with probability 0.75, or to fire with probability 0.25.

The system draw is made for every hit. The system chance = min(1, 10 × `system_hit_chance_per_point` 0.06) = **0.600000**. Bands of `bomb_system_hit` in the code's system order (propulsion, fire control, main battery, torpedo mounts, flight deck): propulsion 0.12, fire control 0.06, main battery 0.12, torpedo mounts 0, flight deck 0.30, each band being 0.6 × share. A system hit on the flight deck would degrade it. That does not move the Allied patrol, because chapter 5 says a deck takes aircraft unless it is out (section 8).

---

## 5. Scenario 19a with group 2 at 1 dive bomber, group 3 at 0, group 1 at 0

- Group 1 has no ready aircraft, so it is not split and flies no patrol. No patrol fire.
- Group 3 has no ready aircraft, so it has no escorts. No escort fire.
- Group 2 launches its 1 bomber at the same contact. Group 2's raid is one bomber, with 0 escorts and 0 patrol.
- Only the guns fire, at the single bomber. A single hit removes that bomber (rules/05, "Aircraft lost": "no more than they have"). So the bomber survives k shots only with no hit at all.
- P(bomber survives k shots) = 0.95^k. Each shot is one independent draw at 0.05 (rules/01, "How a chance is rolled"), and the probability of no hit is (1 − 0.05) per shot.
- The bomber is lost on the first hit, so the total chance of loss is 1 − 0.95^20 whatever the split.

| b | k before | P(bomber attacks) = 0.95^k | Expected bomb hits = 0.2 × P | Lost before attack = 1 − 0.95^k | Attacks and lost after = P × (1 − 0.95^(20−k)) | Lost in all = 1 − 0.95^20 |
|---|---|---|---|---|---|---|
| 1 | 20 | 0.358485 (exact 0.358485922) | 0.071697 | 0.641514 | 0.000000 | 0.641514 |
| 0.5 | 10 | 0.598736 (exact 0.598736939) | 0.119747 | 0.401263 | 0.240251 | 0.641514 |
| 0 | 0 | 1.000000 | 0.200000 | 0.000000 | 0.641514 | 0.641514 |

Notes on the rounding: 0.95^10 = 0.598736939..., 0.95^20 = 0.358485922...; each is cut to six decimals. The loss after attack at b = 0.5 is 0.598736939 × 0.401263061 = 0.240251 (cut). A bomber that attacks and is then lost still delivers its bomb (rules/05, step 4a).

---

## 6. Scenario 19b as the file stands (patrol 4, escorts 6, bombers 10)

Group 1 (Allied, 4 ready) gives P = 4. Group 3 (Japanese, 6 ready, share 0) gives E = 6. Group 2 gives B = 10. One raid, so the share is the whole patrol.

Escorts fire at the patrol: 6 shots, chance 0.3, expected **1.800000**. This is the same for every s. Note that the patrol has 4 aircraft, so hits beyond 4 are not removed. The expected aircraft actually lost is E[min(H, 4)] with H ~ Binomial(6, 0.3) = 1.788336 (the cap removes only 0.011664).

| s | D = cut(min(4,6) × s) | R = 4 − D | E − D | X = cut(R(E−D)/(E−D+B)) | Patrol shots at escorts (D + X) | Expected hits on escorts | Patrol shots at bombers (R − X) | Expected hits on bombers |
|---|---|---|---|---|---|---|---|---|
| 1 | 4 | 0 | 2 | 0 (0 × 2 / 12) | 4 | 1.200000 | 0 | 0.000000 |
| 0.5 | 2 | 2 | 4 | cut(8/14 = 0.571428) = 0 | 2 | 0.600000 | 2 | 0.600000 |
| 0 | 0 | 4 | 6 | cut(24/16 = 1.5) = 1 | 1 | 0.300000 | 3 | 0.900000 |

Check for s = 1: E − D = 2 (6 − 4) is not 0 here, so X = cut(0 × 2 / 12) = 0 and the escorts get 4 shots and the bombers 0.

Check for s = 0.5: X = 8/14 = 0.5714, cut to 0. Escorts get D + X = 2 shots, bombers R − X = 2 shots.

Check for s = 0: X = 24/16 = 1.5, cut to 1, and 1 ≤ E − D = 6, so the cap does not bind. Escorts get 1 shot, bombers 3 shots.

Escorts' shots at the patrol are 6 for every s (chance 0.3, expected 1.800000).

---

## 7. Scenario 19b with group 1 at 10, group 2 at 12, group 3 at 4

Here P = 10, E = 4, B = 12. Escorts' fire at the patrol: 4 shots, expected **1.200000**.

| s | D = cut(min(10,4) × s) | R = 10 − D | E − D | X | Escort shots (D + X) | Expected escort hits | Bomber shots (R − X) | Expected bomber hits |
|---|---|---|---|---|---|---|---|---|
| 1 | 4 | 6 | 0 | 0 | 4 | 1.200000 | 6 | 1.800000 |
| 0.5 | 2 | 8 | 2 | cut(16/14 = 1.142857) = 1 | 3 | 0.900000 | 7 | 2.100000 |
| 0 | 0 | 10 | 4 | cut(40/16 = 2.5) = 2 | 2 | 0.600000 | 8 | 2.400000 |

In each row X ≤ E − D, so the cap does not bind. Every pool holds more aircraft than the expected hits, so no hit is wasted.

---

## 8. Scenario 19b with group 1 at 20, group 2 at 3, group 3 at 2

Here P = 20, E = 2, B = 3. Escorts' fire at the patrol: 2 shots, expected **0.600000**.

**s = 1.** D = cut(min(20,2) × 1) = 2. R = 18. E − D = 0, so X = 0.
- Escorts: 2 shots, expected 0.600000.
- Bombers: 18 shots, expected rolled hits 5.400000.

**s = 0.** D = 0. R = 20. E − D = 2. X = cut(20 × 2 / 5) = cut(8) = 8, capped at E − D = 2, so X = 2.
- Escorts: 2 shots, expected 0.600000.
- Bombers: 18 shots, expected rolled hits 5.400000.

Here the two values of s give the same split, since the escort cap binds.

**The cap on bombers.** The pool holds only 3 bombers, so the aircraft actually removed are fewer than the 5.4 rolled hits. With H ~ Binomial(18, 0.3): P(0) = 0.001628, P(1) = 0.012562, P(2) = 0.045762. Then E[min(H, 3)] = 0.012562 + 2 × 0.045762 + 3 × (1 − 0.059952) = **2.924229** bombers removed in expectation. The rolled hits (5.4) remain the recorded expected value, as rules/05 requires ("the hits of a roll are never capped by the draws").

---

## 9. Task force 2 with two carriers against TF1 (patrol 10)

Raid A (carrier A): B = 8, E = 2. Raid B (carrier B): B = 6, E = 4. Each carrier's escorts go with its own carrier's raid (rules/05, "Which raid the escorts join": "All the escorts at a place go with one raid from that place").

**Each raid's share.** Each raid's aircraft = bombers + escorts: A = 10, B = 10. Total = 20.
- Share A = cut(10 × 10 / 20) = **5**. Share B = cut(10 × 10 / 20) = **5**. The shares add to the patrol of 10, so nothing is left to distribute.
- The shares do not depend on s.

The patrol's fire at each raid for each s:

| s | Raid | D | R | E − D | X | Escort shots | Expected escort hits | Bomber shots | Expected bomber hits |
|---|---|---|---|---|---|---|---|---|---|
| 1 | A (P 5, E 2, B 8) | 2 | 3 | 0 | 0 | 2 | 0.600000 | 3 | 0.900000 |
| 1 | B (P 5, E 4, B 6) | 4 | 1 | 0 | 0 | 4 | 1.200000 | 1 | 0.300000 |
| 0.5 | A | cut(2 × 0.5) = 1 | 4 | 1 | cut(4/9) = 0 | 1 | 0.300000 | 4 | 1.200000 |
| 0.5 | B | cut(4 × 0.5) = 2 | 3 | 2 | cut(6/8) = 0 | 2 | 0.600000 | 3 | 0.900000 |
| 0 | A | 0 | 5 | 2 | cut(10/10) = 1 | 1 | 0.300000 | 4 | 1.200000 |
| 0 | B | 0 | 5 | 4 | cut(20/10) = 2 | 2 | 0.600000 | 3 | 0.900000 |

The escorts of each raid also fire at the patrol: raid A, 2 shots (expected 0.600000); raid B, 4 shots (expected 1.200000). Each raid's patrol shots come from its own stream. Nothing here is capped, since no pool is smaller than the expected hits.

---

## 10. Where I had to assume or infer

1. **Age of the starting contact (19a and 19b).** The scenario's `[[contact]]` has no `age`. Rules/03, "Starting contacts", says each entry "names ... an age in pulses", and gives no default. I assumed age 0. Any age of 0 or 1 gives the same strike, since `strike_contact_max_age_pulses` = 1 (rules/05, "Naval strike: the contact").
2. **Ship systems at the start.** Rules/04, "Named systems", says systems are "each working, degraded or out", and does not say which at the start. I assumed both flight decks are working.
3. **Figures not named.** Rules/01 says a rule whose data file the scenario does not name "does nothing". I took fatigue, supply, upkeep and wear as not loaded.
4. **Pilots.** Rules/05 says "a group whose scenario gives it no pilots does not track them". I took the group's count as its ready aircraft.
5. **No movement.** Rules/04, "Avoiding for the pulse": "a force with no route stays". The `mission = "carrier"` line is not read by any rule (rules/01, "Set mission").
6. **Survival chance over several shots.** Rules/01, "How a chance is rolled", gives the draw for one roll and rules/05, "Aircraft lost", says each hit removes one aircraft. The book does not state how a chance over several shots is combined. I used the probability of no hit, 0.95^k, which follows from independent per-shot draws.
7. **The cut in X.** Rules/05 gives X = R × (E − D)/(E − D + B), "cut to a whole number". I read this as one quotient. The inputs are whole, so the product is exact and the single cut is the only one.
8. **Expected hits and expected losses under the aircraft cap.** Rules/05 says the hits of a roll are not capped by the draws and the cap "happens after" from a pool. Where the cap binds (19b, s-values of 19b Q8, and a one-bomber raid in section 5), I report the rolled expected hits and, where relevant, the expected aircraft removed.
9. **Two raids from one task force.** Rules/05, "Raids": "a place is a ship or a base", so each carrier is its own raid, and the two raids of 9 are separate raids sharing one patrol. I took the share formula as applied across both raids together.
10. **The rounding of the single-bomber survival.** Exact 0.95^20 = 0.358485922 is cut to 0.358485 in the table. Rounded it would be 0.358486. The loss complement 1 − 0.358485922 = 0.641514078 is cut to 0.641514.
11. **Deck hits and "a degraded deck takes them".** Rules/05, "Coming home to no deck": "A degraded deck takes them", which I read as the aircraft still land on a degraded deck. It is unclear and is not needed for the answers, since a single bomb hit on ship 1 can only degrade the deck and not put it out.
12. **Glossary and other chapters.** I did not open `glossary.md`, `differences.md`, `rules/02` or `rules/06`. Nothing in the questions needed them.

---

## Files read

Inside `[scratch]/ct_rulebook/sitting3/`:
- `rules/README.md`
- `rules/01-the-day-orders-and-what-a-side-knows.md`
- `rules/03-search-contacts-and-claims.md`
- `rules/04-surface-combat-and-damage.md`
- `rules/05-air-operations.md`
- `scenarios/19a_guns_and_the_attack.toml`
- `scenarios/19b_whom_the_patrol_fires_at.toml`
- `reference-19a/air_combat.toml`
- `reference-19a/aircraft_types.csv`
- `reference-19a/ship_classes.csv`
- `reference-19a/surface_combat.toml`
- `reference-19b/air_combat.toml`
- `reference-19b/aircraft_types.csv`
- `reference-19b/ship_classes.csv`
- `reference-19b/surface_combat.toml`

A directory listing of the folder was also taken (a `find`, no file opened). No other file was opened and no program was run.
