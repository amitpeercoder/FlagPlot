# Rulebook-only test, sitting 4 run again after the fixes: the model's answer

Scenarios 19c and 04a, the control sitting, run a second time on the architect's ruling of Nov 13, 2026, **after** the rulebook and glossary fixes of this pull request (findings R1 to R5, R7, R8 and L1, L4, L9, L11). A fresh session on the model `haiku`, given the same folder as the first run with `rules/` and `glossary.md` replaced by the fixed ones, and the same prompt but for question 7, which now reads: "The chance that neither ever sights in the fight; and the chances that A has a free round, that B has a free round, and that both fire from the start, each as the rulebook and the glossary define these terms." Word for word as handed back. The marking is in `../closing-tests.md`.

---

WORKED ANSWER. Figures are read from the copies in the scenarios' reference folders (each is headed "FROZEN", a step 1 copy). Rounding: every product in a chain is cut toward zero at six decimals, as chapter 1, Numbers and chance says. Where I combine chances into a probability I give the exact value and then cut it to six decimals. Shot counts "cut to a whole number" are cut toward zero. Where a result is an expectation over random bomber numbers, I give the expectation by linearity and say so.

NAMES USED. Chapter n = rules/0n-*.md. Figure "file: key" = the file in the scenario's reference folder.

=====================================================================
PART A. SCENARIO 19c (day 1, morning pulse)
=====================================================================

A0. Set-up as the files give it
- Allied task force 1 (id 1) at hex (4,0), ship 1 = class 4 carrier, 0 anti-aircraft, aircraft_capacity 36 (ship_classes.csv row 4).
- Japanese task force 2 at hex (9,0), ship 2 = class 4 carrier.
- Air group 1 (Allied, type 4 fighter, ship 1): ready 13, mission combat_air_patrol, patrol share not ordered so default 1 (chapter 5, Air group orders).
- Air group 2 (Japanese, type 3 dive bomber, ship 2): ready 12, mission naval_strike.
- Air group 3 (Japanese, type 4 fighter, ship 2): ready 4, combat_air_patrol, patrol share set to 0.
- Japanese starting contact: carrier, count 1, hex (4,0), age 0 (no age_pulses given).
- Scenario names air_combat only. air_upkeep, fatigue and air_supply are not named, so there is no wear, no fatigue factor and no supply (chapter 1, Where figures come from and when a rule is loaded).
- Values used (air_combat.toml): patrol_before_attack_share p = 1 and anti_aircraft_before_attack_share b = 1 (as the file stands), patrol_drawn_off_share = 1, strike_contact_max_age_pulses = 1, fighter_hit_chance = 0.3, bomb_hit_chance = 0.2, anti_aircraft_hit_chance = 0.05, pilot_armour_factor = 0.75, self_sealing_tanks_factor = 0.75.
- Type figures (aircraft_types.csv): type 3 = bomb, attack_damage 60, pilot_armour 0, self_sealing 0, strike_range 250. Type 4 = fighter, pilot_armour 0, self_sealing 0, strike_range 250.
- Ship figures (ship_classes.csv): class 4 anti_aircraft 0. Class 5 anti_aircraft 20, aircraft_capacity 48.
- Map: placeholder_search.toml region deep_sea q -2..24, r -3..3, so (4,0) and (9,0) are navigable.

A1. Does the strike launch? (chapter 5, The strike decision, When and who; Naval strike: the contact)
- Group 2: mission naval strike, ready 12, has a position (ship 2 afloat), morning so not dark, type 3 has a weapon (bomb), able to fly = 12 (pilots not tracked, so start from ready; deck factor 1 since ship 2 is undamaged; no fatigue file, so no cut). All five conditions pass.
- Contact age: the starting contact is age 0. The night pulse ends with ageing: "each contact that was not made or refreshed in that pulse gains one pulse of age" (chapter 3, Ageing and dropping). So it is age 1 in the morning. 1 is at most strike_contact_max_age_pulses = 1, so it qualifies. (If the night's ageing did not touch a starting contact, it would be age 0, which also qualifies. The result is the same.)
- Reach: air distance from (9,0) to (4,0) = 40 times sqrt(dq^2 + dr^2 + dq*dr) = 40 times sqrt(25 + 0 + 0) = 200 nm. Radius = strike_range 250. 200 is at most 250, so it is within reach.
- Kind carrier is on the default target list.
- Result: group 2 launches 12 aircraft at (4,0), one-way 200 nm. One raid: place ship 2, hex (4,0), bombers 12.

A2. Escorts (chapter 5, The split; Which raid the escorts join, and the reach)
- Group 3: F = 4 (ready 4, pilots not tracked, deck factor 1, no fatigue).
- Patrol = cut(F x share 0) = 0. Escorts = F - patrol = 4.
- The escorts go with the raid from their own place (ship 2) because the raid's hex (4,0) is 200 nm from the group, within its reach 250. So escorts E = 4.
- Group 1 (Allied CAP, share 1): F = 13, patrol = cut(13 x 1) = 13 over hex (4,0). Escorts 0.
- One raid, so it takes the whole patrol: P = 13.

A3. Q1: the file as it stands (p = 1)
Air combat step, in order (chapter 5, The order of the air combat step):

(a) Escorts at the patrol (chapter 5, Escorts against the patrol)
- Shots = escort count = 4 (whatever the share).
- Chance = fighter_hit_chance 0.3 times protection of the patrol's first group, type 4: no armour, no tanks, factor 1. Chance = 0.3.
- Expected hits = 4 x 0.3 = 1.2 on the Allied patrol (group 1).

(b) Patrol at the escorts and the bombers (chapter 5, The patrol against the escorts and the bombers)
- P = 13, E = 4, B = 12.
- D = min(P, E) x patrol_drawn_off_share 1 = min(13, 4) = 4.
- R = P - D = 9.
- X = cut(R x (E - D) / (E - D + B)) = cut(9 x 0 / (0 + 12)) = 0. (The cap "never more than E - D" is 0, which also holds.)
- Shots at escorts = D + X = 4. Shots at bombers = R - X = 9.
- At escorts: shots 4, chance 0.3 x protection of the first escort group, type 4 = 0.3. Expected hits 1.2.
- At bombers: shots 9, chance 0.3 x protection of the first strike group, type 3 = 0.3. Expected hits 9 x 0.3 = 2.7. p = 1, so k = cut(9 x 1) = 9 shots before the attack, 0 after.

(c) Guns: none. Both carriers have anti_aircraft 0, so no anti-aircraft shots (chapter 5, Anti-aircraft fire).

ANSWER Q1: Patrol 13 (group 1, fighter), escorts 4 (group 3, fighter), bombers 12 (group 2, dive bomber).
- Escorts at the patrol: 4 shots, chance 0.3, expected hits 1.2.
- Patrol at the escorts: 4 shots, chance 0.3, expected hits 1.2.
- Patrol at the bombers: 9 shots, chance 0.3, expected hits 2.7 (9 before the attack, 0 after).

A4. Q2: p = 1, 0.4 and 0 (file as it stands; the shots at bombers do not depend on p, only their split does)
- Shots at bombers in total = 9.
- Before the attack: k = cut(9 x p).
  - p = 1: k = 9, after = 0.
  - p = 0.4: k = cut(3.6) = 3, after = 6.
  - p = 0: k = 0, after = 9.
- Bombers hit before the attack: expected 0.3 x k = 2.7, 0.9, 0.
- Bombers left to attack = 12 - hits before. The hits can never exceed 12 (at most 9 shots), so no cap binds and the expectation is exact: 9.3, 11.1, 12.
- Expected attack shots = 9.3 / 11.1 / 12.0.
- Expected bomb hits = 0.2 x attack shots = 1.86 / 2.22 / 2.40 (chance 0.2 each, no protection because bombs against ships do not take protection; chapter 5, The weapon and its chance).
- Expected bombers lost to the patrol after the attack = 0.3 x (9 - k) = 0 / 1.8 / 2.7.
- Expected bombers lost to the patrol, total = 0.3 x 9 = 2.7 in every case. Before the attack: 2.7 / 0.9 / 0.

A5. Q3: group 2 = 1 dive bomber, group 3 = 0, group 1 = 5
- Group 3 has no ready aircraft, so it is not split (chapter 5, The split). No escorts, E = 0, D = 0.
- Group 2 launches 1 aircraft (able to fly 1). The strike decision is otherwise unchanged.
- P = 5 (group 1), R = P - D = 5. X = cut(5 x 0 / (0 + 1)) = 0. Escort shots 0, so the escort skip applies. Bomber shots = R - X = 5 at chance 0.3.
- Shots at the single bomber total 5. Any hit takes the one aircraft ("hits come off the strike groups, lowest group first, no more than they have"), so the bomber is lost when at least one of 5 independent draws hits. Chance lost = 1 - 0.7^5 = 1 - 0.16807 = 0.83193 (for every p, since all 5 shots are made whatever p is).
- Before the attack: k = cut(5 x p).
  - p = 1: k = 5 before, 0 after.
  - p = 0.4: k = cut(2.0) = 2 before, 3 after.
  - p = 0: k = 0 before, 5 after.
- Chance the bomber attacks = chance it survives the k shots before = 0.7^k.
  - p = 1: 0.7^5 = 0.16807.
  - p = 0.4: 0.7^2 = 0.49.
  - p = 0: 1.
- Expected bomb hits = attack chance x 0.2 (a single bomb, chance 0.2).
  - p = 1: 0.16807 x 0.2 = 0.033614.
  - p = 0.4: 0.49 x 0.2 = 0.098.
  - p = 0: 0.2.
- Chance it is lost, split:
  - p = 1: before the attack 1 - 0.16807 = 0.83193; after the attack 0 (it never attacks if lost before). Total 0.83193.
  - p = 0.4: before the attack 1 - 0.49 = 0.51; after the attack 0.49 x (1 - 0.7^3) = 0.49 x 0.657 = 0.32193. Total 0.83193.
  - p = 0: before 0; after 1 x (1 - 0.7^5) = 0.83193. Total 0.83193.

A6. Q4: class 5 carrier in task force 1 (ship 1), group 1 = 6, group 3 = 0, group 2 = 12
- Class 5 has aircraft_capacity 48 and anti_aircraft 20. Ship 1 is the only ship of the Allied force, so the guns of the attacked force fire 20 shots (chapter 5, Anti-aircraft fire, raid on ships). Chance per shot = anti_aircraft_hit_chance 0.05 x protection of the first strike group's type 3, factor 1 = 0.05.
- Group 1 patrol P = 6, no escorts. Patrol shots at bombers = 6, chance 0.3.
- Split of patrol shots: k_p = cut(6 x p). Split of guns: k_g = cut(20 x b).
- Order: patrol before-hits, then guns' before-fire (only if bombers left), then the attack, then patrol after-hits, then guns after.
- Expected hits on the bombers before the attack = 0.3 x k_p + 0.05 x k_g.
- Attack shots = 12 - that. The cap matters only if hits before exceed 12. The most possible is 6 + 20 = 26, so I check the case that matters: for (p,b)=(1,1) the hits before are Bin(6,0.3) + Bin(20,0.05). P(hits >= 13) is about 6.3e-8, so the cap changes the expectation by less than 1e-6. I treat the expectations below as exact to six decimals.

Table:
(p,b) = (1,1): patrol before 6, after 0. Guns before 20, after 0. Expected hits before = 1.8 + 1.0 = 2.8. Expected attack shots = 9.200000. Expected bomb hits = 1.840000.
(0.4,1): patrol before cut(2.4) = 2, after 4. Guns before 20, after 0. Expected hits before = 0.6 + 1.0 = 1.6. Attack shots 10.400000. Bomb hits 2.080000.
(0.4,0): patrol before 2, after 4. Guns before cut(0) = 0, after 20. Expected hits before = 0.6 + 0 = 0.6. Attack shots 11.400000. Bomb hits 2.280000.
(0,0): patrol before 0, after 6. Guns before 0, after 20. Attack shots 12.000000. Bomb hits 2.400000.

ANSWER Q4 table (patrol before/after, guns before/after, expected attack shots, expected bomb hits) as above.

=====================================================================
PART B. SCENARIO 04a (day 1, night pulse, surface fight)
=====================================================================

B0. Set-up
- Japanese task force 1 (force A): cruisers ships 1, 2, 3 (class 1), at hex (0,0), readiness battle (so alert and ready), route to waypoint (10,0), speed cruise, ROE engage, break-off 0.5. Each ship fuel 2000 t.
- Allied task force 2 (force B): cruisers ships 4, 5, 6 (class 1), at hex (5,0), readiness normal (default), no route, idle, ROE engage, break-off 0.5. Each ship fuel 2000 t.
- No contacts, no air figures, no fatigue or supply. Only surface_combat and search are in play.
- Map placeholder_night_action.toml: deep sea q -1..12, r -1..1. So (0,0) to (10,0) is navigable along the axis, and the way is clear.
- Class 1 (ship_classes.csv): cruise 16 kn, max 32, fuel_capacity 2000, endurance 8000, main_guns 8, gun_rounds_per_min 3, gun_rounds_per_gun 100, gun_damage 20, torpedo_tubes 8, torpedo_tubes_one_side 8, torpedo_damage 200, toughness 400, radar 0.
- Surface figures (surface_combat.toml): round_minutes 5, max_rounds 6, sighting_chance_night 0.5, sighting_chance_day 1, radar_sighting_bonus 0.2, not_alert_sighting_factor 0.5, gun_hit_chance_night 0.03, torpedo_hit_chance_night 0.04, surprised_target_factor 1.5, lit_target_factor 2, gun_hit_flotation 0.75, system_hit_chance_per_point 0.06, degraded_factor 0.5, fire_control_out_factor 0.25, crippled_flotation 50, experience_*_strength 0, unready_gun_share 1, unready_torpedo_share 1. Systems: gun_system_hit propulsion 0.2, fire_control 0.3, main_battery 0.35, torpedo_mounts 0.15. torpedo_system_hit propulsion 0.8, fire_control 0.1, main_battery 0.1.

B1. Q5: when and after how many miles do the forces meet; fuel burned
Movement, night pulse, movement step (chapter 1 order; chapter 2, Order of movement within a pulse):
- Budget (chapter 2, The budget of a pulse): time limit = speed 16 x hours left of the night pulse 12 = 192 nm. Fuel limit (chapter 2, The fuel limit): fuel 2000 / multiplier 1 x endurance 8000 / capacity 2000 = 8000 nm. Budget = min(192, 8000) = 192 nm, time-limited.
- Charges (chapter 2, What a step is charged): the way from (0,0) to (10,0) is clear (10 steps = hex distance 10). Each step is charged air distance from H to W minus from N to W = 400 - 360 = 40 for the first step, and so on. Every step is 40 nm.
- Position (chapter 2, Legs and position): the position becomes the hex ahead at half the leg, 20 nm into the leg. Changes of position for force A are at 20, 60, 100, 140, 180 nm.
- Force B at (5,0) is at a centre, idle, not moving. Force A enters (5,0) at 180 nm, when it is 20 nm along the leg (4,0)->(5,0). The meeting is at that change of position (chapter 4, Meeting: "one's position becomes a hex where the other is"). The hex centre (5,0) itself would only be reached at 200 nm, so the meeting comes before the centre.
- Time to meeting = 180 / 16 = 11.25 h into the pulse (chapter 2, The segment hours are distance / speed). Night starts 18:00, so this is 05:15 on the night of day 1 (elapsed hour 11.25). The night pulse ends at 06:00. The display clock is for display only.
- Fuel per cruiser of force A (chapter 2, Fuel burn): distance 180 x fuel_capacity 2000 / endurance 8000 = 45.000000 t. Multiplier 1, because speed 16 is not above cruise 16. Each of ships 1, 2 and 3 burns 45 t, so 135 t in all. Fuel left per ship 2000 - 45 = 1955 t.
- After the fight the force has 12 nm (192 - 180) left in the pulse. This does not change the answer.

ANSWER Q5: The forces meet in the night pulse at elapsed hour 11.25 (05:15) after 180 nm of steaming by force A, when its position reaches (5,0). Each cruiser of A has burned 45.000000 t; 1955 t left.

B2. Q6: each force's chance to sight the other in a round (chapter 4, Sighting)
- chance = min(1, (base + radar_bonus) x alert_factor x experience_factor) x fatigue_factor.
- base: the fight is dark (night pulse, chapter 1, Darkness) so base = sighting_chance_night 0.5.
- radar_bonus: no class 1 ship has radar, so 0.
- Force A (id 1): alert, because it is at battle readiness (chapter 3, What alert is), so alert_factor = 1. experience_factor = 1 (strength 0). fatigue_factor = 1 (no fatigue file). Chance = min(1, 0.5) x 1 x 1 x 1 = 0.5.
- Force B (id 2): normal readiness, not alerted. No contacts, so alert from belief sets nothing (chapter 3, Alert from belief). Not alert, so alert_factor = not_alert_sighting_factor 0.5. Chance = min(1, 0.5 x 0.5 x 1) x 1 = 0.25.
- Neither force is alerted in a round where neither has sighted, since alertness is set only by sighting or being fired on (chapter 3, Alerted inside a pulse), so these chances hold in every round until someone sights.
- In a round where neither has sighted, each force rolls independently:
  - both sight = 0.5 x 0.25 = 0.125
  - only A sights = 0.5 x 0.75 = 0.375
  - only B sights = 0.5 x 0.25 = 0.125 (the ratio is (1 - 0.5) x 0.25)
  - neither = 0.5 x 0.75 = 0.375 (check: the four add to 1)

B3. Q7: neither sights ever; free rounds; both firing from the start (chapter 4, Surprise, and glossary)
- Per round, with neither yet sighted: neither 0.375, A alone 0.375, B alone 0.125, both 0.125. The fight has max_rounds 6.
- A round in which one force sights and the other has not is one in which the first fires and the other is surprised: the rule "surprised if it has not sighted the enemy after the sighting step, the enemy has, and at least one ship of the enemy has shots" holds, since class 1 has 120 gun shots (B3 check below). So that round is the first force's free round. The sighted-from-the-next-round rule makes the other force sighted in the next round, and then both fire.
- Neither ever sights: the last-round check (chapter 4, The ending checks, point 3). P = 0.375^6 = 0.002780914... cut 0.002780. The chain of cut products gives 0.002780 too (0.375, 0.140625, 0.052734, 0.019775, 0.007415, 0.002780). Break-off cannot change this, since no one fires without a sighting.
- Sum over the round of first sighting r = 1..6: the probability that the first sighting round has only A sighting = 0.375/0.625 x (1 - 0.375^6) = 0.6 x 0.997219085 = 0.598331451, cut 0.598331. So:
  - A has a free round: 0.598331.
  - B has a free round: (0.125/0.625) x 0.997219085 = 0.2 x 0.997219085 = 0.199443817, cut 0.199443.
  - Both firing from the start (both sight in the same round, round 1 or later): 0.199443.
  - Neither ever sights: 0.002780.
  - Sum of cut values 0.999997; the 0.000003 is cut-off error.
- Check of B3 gun shot count: gun_rounds_per_min 3 x (main_guns 8 x round_minutes 5) = 120 shots (see B4 below), so surprise applies.

B4. Q8: gun and torpedo salvos (chapter 4, Shots and The hit chance)
- Gun shots (chapter 4, Shots): full = gun_rounds_per_min 3 x (main_guns 8 x round_minutes 5) = 120. Main battery working, factor 1. Unready share: TF A is ready, so 1. Ammunition = 8 x 100 = 800, not limiting. Shots = min(120, 800) = 120.
- Gun hit chance = gun_hit_chance_night 0.03 x surprised factor x lit factor x fire control factor x experience factor (1, strength 0).
  - (a) not surprised, not lit, fire control working: 0.03. Expected hits = 120 x 0.03 = 3.6.
  - (b) surprised: 0.03 x 1.5 = 0.045. Expected hits = 120 x 0.045 = 5.4.
  - (c) lit (target has fire damage above zero, not surprised): 0.03 x 2 = 0.06. Expected hits = 120 x 0.06 = 7.2.
  - Fire control degraded (not surprised): fire_control_factor = degraded_factor 0.5. Chance = 0.03 x 0.5 = 0.015. Expected hits = 120 x 0.015 = 1.8. Shots are unchanged (only the hit chance is cut).
- Torpedo salvo: tubes of the first salvo = torpedo_tubes_one_side B = 8 (B = T = 8, so only one salvo). Torpedo shots = 8 x mount factor 1 x unready 1 = 8. Torpedoes_ready is set at start.
  - Not surprised: chance 0.04, expected hits 8 x 0.04 = 0.32.
  - Surprised: chance 0.04 x 1.5 = 0.06, expected hits 8 x 0.06 = 0.48.

B5. Q9: points and system chances
- Damage points = 100 x weapon damage / toughness, one multiply, one truncating divide (chapter 4, Where a hit lands). Toughness 400.
  - Gun hit: 100 x 20 / 400 = 5.000000 points.
  - Torpedo hit: 100 x 200 / 400 = 50.000000 points.
- Where the hit lands: gun hit on flotation if draw < 0.75, else fire. Torpedo always flotation, so 50 points to flotation, which equals crippled_flotation 50, making the cruiser heavily damaged (chapter 4, Damage tracks).
- System draw made for every hit. System chance = min(1, points x 0.06).
  - Gun: 5 x 0.06 = 0.3. Bands in order propulsion, fire control, main battery, torpedo mounts, flight deck:
    - propulsion 0.3 x 0.2 = 0.06
    - fire control 0.3 x 0.3 = 0.09
    - main battery 0.3 x 0.35 = 0.105
    - torpedo mounts 0.3 x 0.15 = 0.045
    - flight deck 0 (no surface table names it)
    - Total any system = 0.3.
  - Torpedo: 50 x 0.06 = 3, capped at 1. Bands:
    - propulsion 1 x 0.8 = 0.8
    - fire control 1 x 0.1 = 0.1
    - main battery 1 x 0.1 = 0.1
    - torpedo mounts 0 (not in the torpedo table)
    - Total any system = 1.0.
- Each band is a level down from working to degraded (chapter 4, Named systems). The cruiser has propulsion, main battery, fire control and torpedo mounts, so all four bands act.

ANSWER Q9: one gun hit = 5 points, and each system is knocked down with chance: propulsion 0.06, fire control 0.09, main battery 0.105, torpedo mounts 0.045 (any 0.3). One torpedo hit = 50 points, with propulsion 0.8, fire control 0.1, main battery 0.1, torpedo mounts 0 (any 1.0).

B6. Q10: torpedo hit in a free round at a surprised target
- A cruiser of A fires one salvo of 8 torpedoes (B4). In its force's free round the target is surprised, so chance per torpedo = 0.04 x 1.5 = 0.06. Lit factor 1, since no firing has happened before the first sighting round, so the target has no fire damage.
- Probability of no hit = 0.94^8. Chain cut at each product: 0.94^2 = 0.8836; 0.94^4 = 0.780748 (cut from 0.78074896); 0.94^8 = 0.609567 (cut from 0.609567439504).
- Probability of at least one hit = 1 - 0.609567 = 0.390433 on the chain, or 1 - 0.6095689385 = 0.390431 exact (cut). The exact value is 0.3904310615, so I report 0.390431.

=====================================================================
Where I had to assume or infer
=====================================================================
1. Starting contact age in 19c (chapter 3, Ageing and dropping): "each contact that was not made or refreshed in that pulse gains one pulse of age", while chapter 3, Starting contacts says "Events are recorded on day 0, with no pulse". The book does not say whether the night's ageing reaches a starting contact. I took age 1; age 0 gives the same answer because the limit is 1.
2. The attack's bomber count is random (19c Q2-Q4). The book says "Hits come off the strike groups, lowest group first, no more than they have" and "a cap by the aircraft there happens after". I gave expected attack shots by linearity, with the cap checked and found below 1e-6 in every case.
3. Patrol and anti-aircraft split (chapter 5, The patrol against the escorts and the bombers): "the first k = the shots times the share, cut to a whole number". I cut toward zero, so 3.6 gives 3, and 2.4 gives 2.
4. 19c Q3: one bomber, five shots. The book says the hits of a roll "are never capped by the draws" and the bomber is lost if any shot hits. I took each of the five draws as independent and any hit as the loss.
5. 19c Q1, the escorts' share for X: X = 0 because E - D = 0, so all of R goes to the bombers. This follows the formula; the book does not discuss the case.
6. 04a Q5, the meeting: "Two task forces meet when one's position becomes a hex where the other is" (chapter 4, Meeting). I took 180 nm, the change of position, not 200 nm, the hex centre.
7. 04a Q5, the clock: chapter 1 gives the clock display "for display only", so I give 05:15 for the display and 11.25 elapsed hours for the rule.
8. 04a Q6: "Alert from belief" with no contacts sets no flag. The book states the rule for a contact; with no contacts I took the flag as not set, and that force not alert.
9. 04a Q7, free round: "A round in which one force fires and the other, surprised, does not" (glossary, Free round). I read a surprised force as one that has not sighted and whose enemy has shots. The book's phrase "fired on" I read to include the first force's salvo in the round, so the other force sights from the next round.
10. 04a Q7, break-off: the book's break-off test (chapter 4, The ending checks) needs damage or an avoiding stance, and neither can happen before someone sights. So the chance of passing unseen is unaffected.
11. 04a Q8 lit (c): "lit_factor is lit_target_factor if the fight is dark and the target has any fire damage above zero". I assumed the target has fire damage, as the question stated, and did not combine it with surprise.
12. 04a Q10: I assumed no fire damage on the target at the start of a free round, since no firing precedes the first sighting round.
13. 04a readiness: force B is not ready (normal, not alert), so unready_gun_share and unready_torpedo_share would apply, but the frozen figures are 1, so they change nothing. Force A is ready (battle). I used A's shots, which is consistent with the rule.
14. The reference copies in both scenario folders are labelled "FROZEN ... step 1" and say the live figures differ (for example the live patrol_before_attack_share is 0.4, and the live surface shares are placeholders). I used only the copies in the folder, as instructed.
15. Scenario 19c does not name port_sizes data as used; I did not open reference-19c/port_sizes.csv because no cargo rule applies in these questions. I did open reference-04a/port_sizes.csv and it is irrelevant to the questions.

=====================================================================
FILES READ (all inside the sitting4b folder)
=====================================================================
rules/README.md
rules/01-the-day-orders-and-what-a-side-knows.md
rules/02-movement-fuel-and-cargo.md
rules/03-search-contacts-and-claims.md
rules/04-surface-combat-and-damage.md
rules/05-air-operations.md
glossary.md
scenarios/19c_patrol_after_the_attack.toml
scenarios/04a_night_action.toml
reference-19c/air_combat.toml
reference-19c/aircraft_types.csv
reference-19c/ship_classes.csv
reference-19c/surface_combat.toml
reference-19c/search.toml
reference-04a/surface_combat.toml
reference-04a/ship_classes.csv
reference-04a/search.toml
reference-04a/port_sizes.csv
map/placeholder_search.toml
map/placeholder_night_action.toml

No file outside the folder was opened, no program was run, and nothing from the web was used.
