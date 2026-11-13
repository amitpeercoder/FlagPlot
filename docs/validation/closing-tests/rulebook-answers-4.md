# Rulebook-only test, sitting 4: the model's answer

Scenarios 19c and 04a. Model `haiku`, Nov 13, 2026. Word for word as handed back, but for the scratch folder's long path, written [scratch]. The prompt is in `rulebook-sittings.md`, the marking in `../closing-tests.md`.

---

WORKED ANSWER: scenarios 19c and 04a, from the rulebook and reference files in sitting4 only.

CONVENTIONS
- Fixed-point, six decimals. Every product here is exact or a short decimal, so nothing needed cutting except where noted. Where a value is not exact I give it cut toward zero at six decimals and say so. A chance is one draw per whole shot, with an extra draw at chance x fraction for a part-shot (rules/04, Each shot is rolled). Expected hits = shots x chance.
- The reference-19c and reference-04a air_combat.toml and surface_combat.toml files are marked FROZEN copies of the live files, with patrol_before_attack_share = 1, anti_aircraft_before_attack_share = 1, patrol_drawn_off_share = 1, unready shares = 1 and experience strengths = 0. Those are the figures I used, as your p = b = 1 says they are.
- Scenario 19c names air_combat, search and surface_combat and the CSVs. It does not name fatigue.toml, air_upkeep.toml or air_supply.toml, so those rules do not run (rules/01, Where figures come from). Scenario 04a names no air_combat file, so there are no air rules at all in 04a.
- Where a rule could not be read off the rulebook I say so in the last section.

=====================================================
SCENARIO 19c: THE MORNING PULSE OF DAY 1
=====================================================

SET-UP, FROM THE FILES
- 19c file, [[task_force]] 1: allied, q 4, r 0, carrier force. [[ship]] 1: class 4, fuel 3000, TF1.
- ship_classes.csv row 4 (carrier): aircraft_capacity 36, anti_aircraft 0. So TF1 has no guns firing at a raid.
- [[task_force]] 2: japanese, q 9, r 0. [[ship]] 2: class 4, TF2.
- [[air_group]] 1: allied fighter squadron, aircraft_type 4 (fighter), ready 13, on ship 1. Order: combat_air_patrol. patrol_share not ordered, so the default 1 applies. No pilots figure, so pilots are not tracked.
- [[air_group]] 2: japanese bombing squadron, aircraft_type 3 (dive bomber), ready 12, on ship 2. Order: naval_strike.
- [[air_group]] 3: japanese fighter squadron, aircraft_type 4, ready 4, on ship 2. Order: combat_air_patrol. Order: patrol_share 0.
- [[contact]]: japanese, q 4, r 0, one carrier, age not given (see assumptions). The strike is possible at age 0 or 1 either way.
- aircraft_types.csv: type 3 (dive bomber): strike_range_nm 250, attack_weapon bomb, pilot_armour 0, self_sealing 0. Type 4 (fighter): strike_range 250, pilot_armour 0, self_sealing 0.
- air_combat.toml (frozen): bomb_hit_chance 0.2, fighter_hit_chance 0.3, anti_aircraft_hit_chance 0.05, pilot_armour_factor 0.75 (unused: no armour), self_sealing_tanks_factor 0.75 (unused).
- Map: deep sea, so air distance is the straight line. Hex (4,0) to (9,0): dq = 5, dr = 0. Air distance = 40 x sqrt(25) = 200 nm (chapter 5, Air distance).

STEP 1: THE STRIKE DECISION (chapter 5, The strike decision; morning pulse, day 1)
- Group 2: mission naval_strike; ready 12, so it has ready aircraft; it has a position (on ship 2); the morning is not dark; its type carries a bomb; able to fly = min(ready 12, pilots not tracked so ready only) = 12. Ship 2's deck is working (factor 1, no system damage). Fatigue is not loaded, so no cut. Able to fly = 12. It decides.
- Contact qualifies: age 0 <= strike_contact_max_age_pulses 1 (air_combat.toml). Its hex (4,0) is 200 nm from the group, inside reach 250. Its kind carrier is on the default list.
- The group launches 12 aircraft at (4,0). One raid: bombers 12, from ship 2 at hex (4,0).
- Group 3: combat_air_patrol. Able to fly 4. Split (chapter 5, The split): patrol = floor(4 x 0) = 0, escorts = 4. Raid hex (4,0) is 200 nm from group 3 and inside its reach 250, so the escorts go with the raid (EscortsSent).
- Group 1 (allied): able to fly 13. Patrol = floor(13 x 1) = 13 over (4,0). A raid's attack finds TF1 at (4,0) (chapter 5, A strike that finds nothing: not a finding of nothing, the target is there). So the air combat runs.

STEP 2: THE AIR COMBAT STEP, COUNTS (chapter 5, The patrol divided among raids; The patrol against the escorts and the bombers)
- Patrol P = 13 (one raid, so the share is the whole patrol). E = 4 escorts. B = 12 bombers. All counts are taken at the start of the step.
- D = min(P, E) x patrol_drawn_off_share = min(13,4) x 1 = 4.
- R = P - D = 9.
- X = R x (E - D) / (E - D + B) = 9 x 0 / (0 + 12) = 0. (Cut at 0, and never more than E - D = 0.)
- Shots at the escorts = D + X = 4. Shots at the bombers = R - X = 9. Check: 4 + 9 = 13 = P.
- Escorts fire at the patrol: 4 shots (one per escort). Chance = fighter_hit_chance 0.3 x protection of the first patrol group's type (type 4: no armour, no tanks) = 0.3.
- Guns: TF1's ship 1 is class 4, anti_aircraft 0, so there are no gun shots and no gun fire.

QUESTION 1 (file as it stands: p = 1, b = 1)
- Escorts at the patrol: 4 shots, chance 0.3, expected hits 4 x 0.3 = 1.2. Taken off the patrol, lowest group first.
- Patrol at the escorts: 4 shots (D + X = 4 + 0), chance 0.3 x 1 (type 4 has no protection) = 0.3, expected hits 1.2. Taken off the escorts, lowest group first.
- Patrol at the bombers: 9 shots (R - X = 9), chance 0.3 x 1 (type 3 has no protection) = 0.3, expected hits 9 x 0.3 = 2.7.
  At p = 1, the before-attack share of these shots is floor(9 x 1) = 9, so all 9 are before the attack and none after.
- The escorts' 1.2 and the patrol's 1.2 do not change the bomber figures. The escort hits reduce the escorts and the patrol, but the patrol's own shot count was fixed at the start of the step at P = 13.

QUESTION 2 (group 2 = 12 bombers, group 3 = 4 escorts, group 1 = 13 patrol; p = 1, 0.4, 0; b irrelevant, no guns)
- Patrol shots at the bombers: 9 at every p (the total is fixed). Only the before/after split changes.
- Before-attack count k = floor(9 x p), after = 9 - k.
  p = 1: k = floor(9) = 9, after 0.
  p = 0.4: 9 x 0.4 = 3.6, k = floor(3.6) = 3, after 6.
  p = 0: k = 0, after 9.
- Bomber hits before the attack, expected = k x 0.3:
  p = 1: 2.7; p = 0.4: 0.9; p = 0: 0.
- Bombers left at the attack, expected = 12 - the before hits. (Cap check: the before hits are at most 9, below the 12 bombers, so the cap never binds.)
  p = 1: 9.3; p = 0.4: 11.1; p = 0: 12.
- Expected bomb hits = attack shots x 0.2 (bomb_hit_chance):
  p = 1: 9.3 x 0.2 = 1.86; p = 0.4: 11.1 x 0.2 = 2.22; p = 0: 12 x 0.2 = 2.4.
- Expected bombers lost to the patrol: before the attack, then after it. The after-attack hits (chapter 5, the hits after come off the bombers that are left, after their attack) are capped by the bombers that remain (12 - before hits - after hits never falls below 0 here, since before <= 3 and after <= 6 at p = 0.4).
  p = 1: before 2.7, after 0, total 2.7.
  p = 0.4: before 0.9, after 1.8, total 2.7.
  p = 0: before 0, after 2.7, total 2.7.
  The total is 2.7 at every p. Only the timing changes.

QUESTION 3 (group 2 = 1 dive bomber, group 3 = 0, group 1 = 5; p = 1, 0.4, 0)
- Escorts 0, so no escorts, no escorts' fire and the patrol's shots at the escorts are 0. D = min(5,0) x 1 = 0, R = 5, X = 5 x (0 - 0) / (0 + 1) = 0. Shots at the bomber = 5, chance 0.3.
- The strike still launches: group 2 able to fly 1 (>0), the contact qualifies, reach is 200 <= 250.
- Before-attack shots k = floor(5 x p): p = 1: 5 (after 0); p = 0.4: 2.0, so k = 2 (after 3); p = 0: 0 (after 5).
- The one bomber attacks only if no before-attack shot hits it, since a hit removes the only aircraft (hits beyond the pool are wasted). Chance it attacks = 0.7^k:
  p = 1: 0.7^5 = 0.16807 (exact).
  p = 0.4: 0.7^2 = 0.49 (exact).
  p = 0: 1.
- Expected bomb hits = chance it attacks x 1 aircraft x 0.2:
  p = 1: 0.16807 x 0.2 = 0.033614 (exact).
  p = 0.4: 0.49 x 0.2 = 0.098.
  p = 0: 0.2.
- Chance it is lost:
  Lost before the attack = 1 - 0.7^k: p = 1: 0.83193; p = 0.4: 0.51; p = 0: 0.
  Lost after the attack = (chance it survived before) x (1 - 0.7^(5-k)): p = 1: 0; p = 0.4: 0.49 x (1 - 0.343) = 0.32193; p = 0: 1 - 0.7^5 = 0.83193.
  Total lost = 0.83193 at each p (it is 1 - 0.7^5 whatever the split, since all five shots hit the one bomber if they hit at all). The split changes only whether it bombs first.

QUESTION 4 (ship 1 = class 5 carrier with guns; group 1 = 6 patrol; group 3 = 0 escorts; group 2 = 12 bombers; (p, b) = (1,1), (0.4,1), (0.4,0), (0,0))
- Class 5 (ship_classes.csv row 5): aircraft_capacity 48, anti_aircraft 20. TF1 has only ship 1, so the guns fire 20 shots at the raid on TF1.
- Patrol P = 6, escorts 0 (no escort fire, no shots at escorts). Patrol shots at the bombers = 6 x 1 chance 0.3.
- Gun shots = 20 at chance anti_aircraft_hit_chance 0.05 x 1 (type 3 has no protection) = 0.05. Expected hits per full set = 20 x 0.05 = 1.0.
- Ordering (chapter 5, The order of the air combat step): patrol before-attack hits come off the bombers first; then the guns fire before the attack on the bombers that are left (if any); then the bombers attack; then the after-attack patrol and gun fire.
- Shot splits: patrol before = floor(6 x p), guns before = floor(20 x b).

  (p, b) = (1, 1): patrol before 6, after 0. Guns before 20, after 0.
  (p, b) = (0.4, 1): patrol before floor(2.4) = 2, after 4. Guns before 20, after 0.
  (p, b) = (0.4, 0): patrol before 2, after 4. Guns before 0, after 20.
  (p, b) = (0, 0): patrol before 0, after 6. Guns before 0, after 20.

- Expected attack shots. Let T = (patrol before hits) + (gun before hits). The bombers left = max(0, 12 - T), so E[left] = 12 - E[T] + E[(T - 12)^+], because max(0, x) = x + max(0, -x).
  (1,1): E[T] = 6 x 0.3 + 20 x 0.05 = 1.8 + 1.0 = 2.8. The cap term E[(T-12)^+] needs T >= 13, which needs the guns to score at least 7 hits (the patrol at most 6). Using the binomial tail of Bin(20, 0.05) and Bin(6, 0.3):
      P(guns >= 7) = 3.1e-5, times the conditional term about 0.0007 gives about 2e-8, and the terms for 8 to 12 hits add about 5e-8 more.
      So E[(T-12)^+] = 0.000000073 to six decimals (about 7e-8). Expected attack shots = 12 - 2.8 + 0.000000073 = 9.200000073, which is 9.200000 to six decimals (cut or round; no difference).
      Expected bomb hits = 9.200000073 x 0.2 = 1.840000015, so 1.840000.
  (0.4,1): E[T] = 2 x 0.3 + 1.0 = 1.6. The cap needs T >= 13, which needs the guns to score at least 11, probability about 5e-10. Negligible. Attack shots = 12 - 1.6 = 10.4. Bomb hits = 10.4 x 0.2 = 2.08.
  (0.4,0): E[T] = 2 x 0.3 = 0.6 (no guns before). Attack shots = 11.4. Bomb hits = 11.4 x 0.2 = 2.28.
  (0,0): E[T] = 0. Attack shots = 12. Bomb hits = 2.4.

- Patrol and gun hit expectations, for the record: patrol before hits (1,1): 1.8; (0.4,1): 0.6; (0.4,0): 0.6; (0,0): 0. Gun before hits: (1,1): 1.0; (0.4,1): 1.0; (0.4,0): 0; (0,0): 0. After-attack patrol hits: (1,1): 0; (0.4,1): 1.2; (0.4,0): 1.2; (0,0): 1.8. After-attack gun hits: (1,1): 0; (0.4,1): 0; (0.4,0): 1.0; (0,0): 1.0.

- Summary table:
  (1, 1): patrol before 6 / after 0; guns before 20 / after 0; attack shots 9.200000; bomb hits 1.840000.
  (0.4, 1): patrol before 2 / after 4; guns before 20 / after 0; attack shots 10.400000; bomb hits 2.080000.
  (0.4, 0): patrol before 2 / after 4; guns before 0 / after 20; attack shots 11.400000; bomb hits 2.280000.
  (0, 0): patrol before 0 / after 6; guns before 0 / after 20; attack shots 12.000000; bomb hits 2.400000.

=====================================================
SCENARIO 04a: THE NIGHT PULSE OF DAY 1
=====================================================

SET-UP, FROM THE FILES
- Map (placeholder_night_action.toml): one region, deep sea, q -1 to 12, r -1 to 1. No land, no closed sides. Every step is therefore clear-way, charged the air distance: 40 nm a step along the axis (rules/02, What a step is charged).
- TF1 (japanese, id 1): ships 1, 2, 3 (class 1 cruisers), fuel 2000 t each, at (0,0). Route: waypoint (10,0). Speed: cruise. Readiness: battle. ROE: engage, break_off 0.5.
- TF2 (allied, id 2): ships 4, 5, 6 (class 1), at (5,0). No route, so idle. Readiness normal (default). Not alert: no contacts, no alarm.
- ship_classes.csv row 1 (cruiser): cruise 16 kn, max 32 kn, fuel_capacity 2000 t, endurance 8000 nm, main_guns 8, gun_rounds_per_min 3, gun_rounds_per_gun 100, gun_damage 20, torpedo_tubes 8, torpedo_tubes_one_side 8, torpedo_damage 200, toughness 400, radar 0, aircraft_capacity 0, anti_aircraft 0.
- surface_combat.toml (frozen): round_minutes 5, max_rounds 6, sighting_chance_night 0.5, sighting_chance_day 1, radar_sighting_bonus 0.2, not_alert_sighting_factor 0.5, gun_hit_chance_night 0.03, gun_hit_chance_day 0.05, torpedo_hit_chance_night 0.04, torpedo_hit_chance_day 0.02, surprised_target_factor 1.5, lit_target_factor 2, gun_hit_flotation 0.75, system_hit_chance_per_point 0.06, degraded_factor 0.5, fire_control_out_factor 0.25, experience strengths 0, unready shares 1, gun_system_hit (propulsion 0.2, fire_control 0.3, main_battery 0.35, torpedo_mounts 0.15), torpedo_system_hit (propulsion 0.8, fire_control 0.1, main_battery 0.1), crippled_flotation 50, claimed_sunk_fire 30.
- The night pulse is dark (rules/01, Darkness: dark exactly in the night pulse). So the night chances apply.
- No fatigue.toml is named, so there is no fatigue factor (fatigue factor 1).

QUESTION 5: WHEN AND WHERE THE FORCES MEET, AND FUEL BURNED
- Night pulse, 12 hours. TF1 speed = cruise 16 kn x propulsion factor 1 = 16 kn (rules/02, Speed).
- Budget (rules/02, The budget of a pulse): time limit = 12 x 16 = 192 nm. Fuel limit: the distance the fuel lasts at 16 kn with multiplier 1 = 2000 x 8000 / 2000 = 8000 nm. Budget = min(192, 8000) = 192 nm, time-limited.
- Path: (0,0) -> (1,0) -> (2,0) -> (3,0) -> (4,0) -> (5,0), five steps of 40 nm each.
- Position changes come at 20 nm into each leg: at 20, 60, 100, 140 and 180 nm. The position rule (rules/02, Legs and position): the hex ahead once the force is half the leg's charge (20 nm of 40) or more along it. So the force enters (5,0) at 180 nm, which is 20 nm into the fifth leg.
- Meeting: a force meets the opposing force in the hex it enters (rules/02, Meeting; rules/04, Meeting). TF2 is in (5,0) with ships afloat. So TF1 and TF2 meet at (5,0) at 180 nm, inside the 192 nm budget.
- Time to 180 nm: 180 / 16 = 11.25 hours after the pulse start, which is 11 h 15 min. The pulse starts at 18:00, so the meeting is at 05:15 clock time in game day 1, elapsed hour 11.25 from the scenario start. (rules/01, The clock: game day 1 runs from 18:00 to 18:00 the next clock day.)
- Fuel burned by each cruiser of TF1 at the meeting (rules/02, Fuel burn): per mile at cruise = fuel_capacity / endurance = 2000 / 8000 = 0.25 t. Multiplier 1 (speed 16 kn is not above cruise). Burned = 180 x 2000 / 8000 x 1 = 45.000000 t, which is below the 2000 t aboard, so no cap.
  Each cruiser of TF1: 2000 - 45 = 1955.000000 t left.
  Three cruisers: 135 t burned in all.
  TF2 burned nothing (it has no route, so it did not steam).
- The segment is written before the fight (rules/04, Meeting: both forces' steaming is recorded before the fight). The fuel is written at the segment's end, so the figure is 45 t at the meeting.
- After the fight, TF1 has 192 - 180 = 12 nm of budget left in the pulse, so it could steam on along its route for 12 nm if it is still able (not asked; noted only).

QUESTION 6: SIGHTING CHANCES IN A ROUND
- Forces roll in ID order (lower ID first): TF1 (A) rolls first, then TF2 (B).
- Chance formula (rules/04, Sighting): chance = min(1, (base + radar_bonus) x alert_factor x experience_factor) x fatigue_factor.
- base = sighting_chance_night 0.5 (the fight is dark).
- radar_bonus = 0 for both forces: class 1 has radar 0 (ship_classes.csv), so radar_sighting_bonus 0.2 is not added.
- A (TF1): alert. Battle readiness makes it alert (rules/03, What alert is). alert_factor = 1. experience_factor = 1 (experience_sighting_strength 0). fatigue_factor = 1 (no fatigue file).
  A's chance = min(1, 0.5 x 1 x 1) x 1 = 0.5.
- B (TF2): not alert (normal readiness, no contact, no alarm, no earlier fight). alert_factor = not_alert_sighting_factor 0.5. experience 1. fatigue 1.
  B's chance = min(1, 0.5 x 0.5 x 1) x 1 = 0.25.
- Per round, in a round where neither has sighted (rolls independent, one draw each from its own stream):
  Both sight: 0.5 x 0.25 = 0.125.
  Only A sights: 0.5 x 0.75 = 0.375.
  Only B sights: 0.5 x 0.25 = 0.125. (A does not: 1 - 0.5 = 0.5, so it is 0.5 x 0.25 = 0.125; the 0.5 is A's miss and 0.25 is B's hit. Check: 0.5 x 0.25 = 0.125.)
  Neither sights: 0.5 x 0.75 = 0.375.
  Check: 0.125 + 0.375 + 0.125 + 0.375 = 1.0.
  (Correction on the line above: "Only B sights" = P(A misses) x P(B sights) = 0.5 x 0.25 = 0.125. "Neither" = 0.5 x 0.75 = 0.375.)

QUESTION 7: THE SIGHTING SEQUENCE OVER THE FIGHT
- The fight has max_rounds = 6 rounds of 5 minutes (rules/04, The fight). Neither force sights in all six rounds: the fight ends passed unseen (rules/04, The ending checks; the last round check).
- Per-round states: neither sights 0.375; only A 0.375; only B 0.125; both 0.125. The per-round chance of each does not change while a force is unsighted (B is not alert until it sights, so its factor stays 0.5).
- Once a force has sighted, it stops rolling. A force fired on in the previous round has sighted without a roll (rules/04, Sighting: "A force that was fired on in the previous round has sighted the enemy now, with no roll and no draw"). So if only A sights in round r, A fires in round r, B is fired on, and B has sighted from round r+1 on.
- "A free round" is not a term the rulebook defines (see the last section). I take it as a round in which one force has sighted and the other has not. The unsighted force is surprised, and it does not fire (rules/04, Surprise: "a force that is surprised does not fire"). So the sighted force fires unopposed. This is the first round in which only one force has sighted.

- Neither ever sights in the fight: 0.375^6 = 0.002780914306640625. Cut to six decimals: 0.002780 (rounded: 0.002781).

- A has a free round: A alone sights in the first round in which anyone sights. Round 1 A-only: 0.375. Round r A-only, after r-1 rounds of neither: 0.375^(r-1) x 0.375. Sum over r = 1 to 6 = 0.375 x (1 - 0.375^6) / (1 - 0.375) = 0.6 x (1 - 0.375^6) = 0.6 x 0.997219085693... = 0.598331451416. Cut: 0.598331.

- B has a free round: B alone sights first. Round 1 B-only 0.125; per round rate 0.125 out of the 0.625 that has a sighting. Sum = (0.125/0.625) x (1 - 0.375^6) = 0.2 x 0.997219085693 = 0.199443817139. Cut: 0.199443.

- Both fire from the start: both sight in round 1: 0.125 exactly. (Both sight in a later round: 0.2 x (1 - 0.375^6) - 0.125 = 0.074443817, cut 0.074443.)

- Check: 0.598331451 (A free) + 0.199443817 (B free) + 0.199443817 (both sight, any round) + 0.002780914 (neither) = 1.000000 (to rounding). And 0.199443817 (both, any round) = 0.125 (round 1) + 0.074443817 (later). So "both fire from the start" = 0.125, not 0.199443.

QUESTION 8: GUN AND TORPEDO SALVOS (cruiser of A; the fight is dark)
- Gun shots per salvo (rules/04, Gun shots in a round): full = gun_rounds_per_min x (main_guns x round_minutes) = 3 x (8 x 5) = 120. Shots = min(120 x main_battery_factor 1 x unready_gun_share 1, ammunition aboard) = min(120, 8 x 100 = 800) = 120 shots. A is battle readiness, so it is ready, and the unready share is 1 anyway (frozen).
- Gun hit chance = base 0.03 (gun_hit_chance_night) x surprised_factor x lit_factor x fire_control_factor x experience_factor 1 (strength 0).
  (a) Target not surprised, not lit, fire control working: chance 0.03. Expected hits = 120 x 0.03 = 3.6.
  (b) Target surprised: 0.03 x 1.5 = 0.045. Expected = 120 x 0.045 = 5.4.
  (c) Target lit (fire damage above 0 at the start of the round): 0.03 x 2 = 0.06. Expected = 120 x 0.06 = 7.2.
  (d) Fire control degraded, target not surprised: 0.03 x 0.5 = 0.015. Expected = 120 x 0.015 = 1.8. Shots still 120 (fire control only changes the chance).
- Torpedo salvo: tubes 8 (torpedo_tubes = torpedo_tubes_one_side = 8, so B = T and one salvo of all 8). Shots = 8 x torpedo_mount_factor 1 x unready share 1 = 8. Torpedo_ready is set at the start. Torpedo fire-control factor is 1 for torpedoes.
  Chance: torpedo_hit_chance_night 0.04.
  Target not surprised: 0.04. Expected = 8 x 0.04 = 0.32.
  Target surprised: 0.04 x 1.5 = 0.06. Expected = 8 x 0.06 = 0.48.

QUESTION 9: DAMAGE POINTS AND SYSTEM CHANCES, PLACEHOLDER CRUISER (class 1)
- Points (rules/04, Where a hit lands): 100 x weapon damage / toughness, one multiply then one divide, cut. Toughness 400.
  Gun hit: 100 x 20 / 400 = 5.000000 points. It lands on flotation if its draw is below gun_hit_flotation 0.75, otherwise on fire (fire track also takes 5).
  Torpedo hit: 100 x 200 / 400 = 50.000000 points, always on flotation.
- System chance per hit (rules/04, Where a hit lands): min(1, points x system_hit_chance_per_point 0.06).
  Gun: min(1, 5 x 0.06) = 0.300000.
  Torpedo: min(1, 50 x 0.06 = 3) = 1.000000.
- The bands are in ShipSystem order: propulsion, fire control, main battery, torpedo mounts, flight deck. Each band's width = system chance x the weapon table's share for that system.
  Gun (system chance 0.3): propulsion 0.3 x 0.2 = 0.060000; fire control 0.3 x 0.3 = 0.090000; main battery 0.3 x 0.35 = 0.105000; torpedo mounts 0.3 x 0.15 = 0.045000; flight deck 0 (not in the table). Sum 0.300000.
  Torpedo (system chance 1.0): propulsion 1.0 x 0.8 = 0.800000; fire control 1.0 x 0.1 = 0.100000; main battery 1.0 x 0.1 = 0.100000; torpedo mounts 0 (no row in the torpedo table); flight deck 0. Sum 1.000000.
- A cruiser has no flight deck (aircraft_capacity 0), so the flight deck band is not used.
- A system that is working becomes degraded; a degraded one becomes out on its next hit (rules/04, Named systems). The chances above are for the first hit on a working system.

QUESTION 10: CHANCE A FREE-ROUND TORPEDO SALVO HITS AT LEAST ONCE
- In a free round the target is surprised (factor 1.5). The target has no fire damage at the start of that round, so no lit factor. Torpedo chance = 0.04 x 1.5 = 0.06.
- The salvo is 8 shots (all tubes, one side). Each is one draw at 0.06, independent.
- P(no hit) = 0.94^8. 0.94^2 = 0.8836, 0.94^4 = 0.78074896, 0.94^8 = 0.78074896^2 = 0.609568938541 (to 12 places).
- P(at least one hit) = 1 - 0.609568938541 = 0.390431061459. Cut to six decimals: 0.390431.

=====================================================
WHERE I HAD TO ASSUME OR INFER
=====================================================
- 19c contact age: the file's [[contact]] has no age. The rulebook says each starting-contact entry names an age (rules/03, Starting contacts), so the missing field is unclear. I assumed 0. The strike result is the same at age 0 or 1, since the limit is strike_contact_max_age_pulses = 1.
- "Free round" (questions 7 and 10): not defined in the rulebook or the glossary. I took it as a round in which one force has sighted and the other has not, so the unsighted force is surprised and does not fire (rules/04, Surprise: "A force is surprised in a round if it has not sighted the enemy after the sighting step, the enemy has, and at least one ship of the enemy has shots"; glossary "Surprised": "It does not fire"). The alternative reading, a round in which the force simply fires with no surprise bonus, is not what the words of the rule give.
- "Fired on" (question 7): the rule "A force that was fired on in the previous round has sighted the enemy now, with no roll and no draw" does not say that a salvo needs to hit. I took a salvo aimed at the force as enough. This is needed for B to sight the round after A's free round.
- Start-of-step counts (question 1 and 2): rules/05, "With P the raid's share, E its escorts and B its bombers at the start of the step", so the patrol's shot count is fixed at 13 even though the escorts' fire has already reduced the patrol.
- Cap in question 4: the expected attack shots are an expected count, with the pool cap worked as a binomial correction. The correction is +0.000000073 at (1,1) and negligible elsewhere. I quote 9.200000 and say so.
- Expected values: all expectations are linear sums over independent draws, which I took as independent (each draw from its own key). Where a cap could bind (only question 4 at (1,1)), I worked it exactly.
- Patrol shots at the bombers in question 3 with no escorts: the formula X = R x (E - D)/(E - D + B) gives X = 0, so all R = P - D shots go to the bombers. I read "never more than E - D" as holding at 0.
- Rounding: products here are exact decimals, so no cut was needed except 0.375^6 (cut to 0.002780) and the other long probabilities (cut to six decimals). The engine's rule is cut toward zero (rules/01, Numbers and chance); I cut all the long results.
- Frozen reference files: I used the reference-19c and reference-04a copies, which the files mark as FROZEN. I did not check them against the live data files (not in this folder). The question's p = b = 1 agrees with the frozen copies.
- Question 8 lit case: "lit" means the target has fire damage above 0 at the start of the round (rules/04, The hit chance: "The target's fire damage is as it stood at the start of the round"). That is not true in the free round of question 10, and I used the unlit chance there.
- Clock in question 5: the zone is not given in 04a. I used the slice's UTC+11 and an 18:00 pulse start (rules/01, The clock: "For the slice the zone is UTC+11"). The meeting time is elapsed hours 11.25 from the start. The clock reading is only for display (rules/01: "This rounding is for display only").
- Question 5, "after how many miles": 180 nm is the distance along the route from the start, counted from the centre of (0,0). The rule that a force "enters a hex" at the half-leg point (rules/02, Legs and position) is what sets the 180 nm rather than 200 nm.
- Question 9, torpedo mounts: the torpedo table has no torpedo_mounts row, so a torpedo hit gives that system 0 chance. I read the missing row as share 0, as rules/04 says ("A system absent from a table has share 0").

FILES READ (all inside [scratch]/ct_rulebook/sitting4/):
1. rules/README.md
2. scenarios/19c_patrol_after_the_attack.toml
3. scenarios/04a_night_action.toml
4. rules/05-air-operations.md
5. rules/04-surface-combat-and-damage.md
6. rules/03-search-contacts-and-claims.md
7. rules/02-movement-fuel-and-cargo.md
8. rules/01-the-day-orders-and-what-a-side-knows.md
9. glossary.md
10. reference-19c/air_combat.toml
11. reference-19c/aircraft_types.csv
12. reference-19c/ship_classes.csv
13. reference-19c/surface_combat.toml
14. reference-19c/search.toml
15. reference-04a/surface_combat.toml
16. reference-04a/search.toml
17. reference-04a/ship_classes.csv
18. map/placeholder_night_action.toml
19. map/placeholder_search.toml
Also one shell command, ls -R on the sitting4 folder only, to list its contents (the Read tool cannot list a directory). It opened no file. Not opened: rules/differences.md, the other reference CSVs and TOMLs (fatigue, air_upkeep, air_supply, port_sizes, aircraft_types_*, ship_classes_*), because neither scenario's loaded rules needed them.
