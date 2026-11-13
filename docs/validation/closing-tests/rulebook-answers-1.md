# Rulebook-only test, sitting 1: the model's answer

Scenarios 16 and 15b. Model `haiku`, Nov 13, 2026. Word for word as handed back. The prompt is in `rulebook-sittings.md`, the marking in `../closing-tests.md`.

---

WORKED ANSWER: Flag Plot night action, scenarios 16 and 15b (day 1, night pulse; Q10 morning hypothetical)

Sources and conventions
- Rule references are chapter and section of rules/ (ch1 = 01, ch2 = 02, ch3 = 03, ch4 = 04). Figures are file and key. Scenario figures are entry and field.
- Ship class 1 (Placeholder cruiser), from reference-16/ship_classes.csv and reference-15b/ship_classes.csv (identical for class 1): cruise_speed_kn 16, max_speed_kn 32, fuel_capacity_t 2000, endurance_nm 8000, main_guns 8, gun_rounds_per_min 3, gun_rounds_per_gun 100, gun_damage 20, torpedo_tubes 8, torpedo_tubes_one_side 8, torpedo_damage 200, toughness 400, radar 0.
- Round length: surface_combat.toml round_minutes = 5 (both scenarios).
- All products here are exact at six decimals, so no cutting or rounding changed any result. Where a division or cut could apply, it is stated.

Preliminary points that affect the answers
A. Waypoint. The question says task force 4 steams toward (2,-4). The scenario file says otherwise: scenarios/16_all_together.toml, [[order]] day 1, side japanese, task_force 4, kind set_route: waypoints = [[0, -4]]. I used the file. The route runs west along the row r = -4, so the force passes through (2,-4) on the way to (0,-4). The entry into (2,-4) is the same at 180 miles whether the waypoint is (0,-4) or (2,-4); only the arrival at the centre differs.
B. Experience. Scenario 16 uses reference-16/surface_combat.toml with experience strengths 0.4 (gun, torpedo, sighting). Scenario 15b uses reference-15b/surface_combat.toml with all three at 0, so every experience factor there is 1 (ch4, The hit chance: "a factor of 1 changes nothing"; ch4, Sighting: strength 0 gives factor 1).
C. Unready shares. Scenario 16: unready_gun_share 0.5, unready_torpedo_share 1. Scenario 15b: unready_gun_share 0.5, unready_torpedo_share 0.5. (ch4, Ready, and the shares of the guns and the torpedoes.)
D. Fatigue. Neither scenario names fatigue.toml, so there is no fatigue (ch1, Where figures come from and when a rule is loaded): fatigue factor 1, and no fatigue changes.
E. Air. Night pulse air phases (repair, arrivals, replacements, supply, rest) and the empty night search do not touch the ships' guns, torpedoes or sighting, so I took them as not affecting these answers. I did not open chapter 5 (air) or chapter 6, so this rests on the chapter 1 order-of-a-day table and chapter 3 ("the night pulse has an empty search phase").

=====================================================
SCENARIO 16 (night pulse of day 1)
=====================================================

Q1. Distance and time to entry of (2,-4)

Steps:
1. Start. TF4 (Japanese cruisers, ships 6, 7, 8) at (7,-4) (scenario task_force 4, q 7, r -4). Route set to waypoint (0,-4) (A above). Speed setting cruise (set_speed "cruise", [[order]] day 1 TF4). Status underway after the route order (ch2, Orders that move a force).
2. Speed (ch2, Speed). Cruise uses cruise_speed_kn 16 times propulsion factor 1 (no damage). Force speed = lowest ship speed = 16 kn.
3. Budget for the night pulse (ch2, The budget of a pulse). Pulse length 12 h (ch1 table, night pulse).
   - Time limit = 16 x 12 = 192.000000 miles.
   - Fuel limit: each cruiser has fuel_t 2000 (scenario ship entries 6 to 8). Distance = 2000 / 1 (multiplier 1 at cruise) x 8000 / 2000 = 8000.000000 miles. Least-fuelled ship is 2000 for all.
   - Budget = min(192, 8000) = 192.000000 miles, time-limited.
4. Charges per step (ch2, What a step is charged). The way is clear: the map is deep sea across q -2..24, r -6..6 (map/placeholder_search_wide.toml, region; the only coast is (0,0), not on this row), so the sea path has as many steps as the hex distance. Each step west along r = -4 is a hex-axis direction, so each step is charged 40.000000 miles (ch2: "Along any of the six directions ... the air distance falls by 40 a step"). Check: first step charge = air distance (7,-4) to (0,-4) = 280, less (6,-4) to (0,-4) = 240, gives 40.
5. Position changes (ch2, Legs and position). A force is in the hex ahead at exactly half the leg charge (20 miles of 40). Cumulative miles at which the position changes:
   - into (6,-4) at 20
   - into (5,-4) at 60
   - into (4,-4) at 100
   - into (3,-4) at 140
   - into (2,-4) at 180.
   Centres reached at 40, 80, 120, 160, 200.
6. Entry into (2,-4) at 180.000000 miles.
7. Time at entry = 180 / 16 = 11.250000 hours after the start of the night pulse (ch2, What a segment records: hours = distance / speed).
8. Clock. The night pulse starts at 18:00 (ch1, The day and its pulses; table). 18:00 + 11 h 15 min = 05:15, still game day 1 (the day runs 18:00 to 18:00). Display: D1 05:15. Assumption: the scenario states no reference zone, and the zone only labels the clock; the arithmetic is the elapsed hours.
9. Fight. TF3 (Allied cruisers, ships 3 to 5) is idle at (2,-4) with no route, so it is not moving. When TF4's position changes into (2,-4), it meets TF3 (ch4, Meeting; ch2, Meeting). The fight begins at 180 miles, at 05:15 on D1. Fights take no time (ch4, Meeting).
10. Steaming recorded before the fight (ch2, What a segment records): 180.000000 miles. Fuel burned per cruiser = 180 x 2000 / 8000 = 45.000000 t, leaving 1955.000000 t. Budget left for later in the pulse = 192 - 180 = 12.000000 miles (ch2, After a fight: "distance left is unchanged" at same speed).

Answer: after 180 miles, at 11.25 h, i.e. D1 05:15. (The fight begins on entry, not at the centre, which would be 200 miles.)

Q2. Alertness and readiness

Steps:
1. No contacts exist at the start of the night pulse. The scenario gives no [[contact]] entries, and the night pulse has no search (ch3, Where this sits in a day; ch1 table, night search empty). So alert from belief finds nothing for either force.
2. TF3 (Allied, normal readiness, flag unset). Alert from belief: no contact counts, flag unset, nothing recorded (ch3, Alert from belief). Alert: a normal-readiness force is alert only while the flag is set (ch3, What alert is). TF3 is NOT ALERT.
   - Ready (ch4, Ready): "A force at normal readiness that is not alert is not ready." TF3 is NOT READY.
3. TF4 (Japanese, battle readiness, scenario task_force 4 readiness = "battle"). Alert from belief skips a battle-readiness force (ch3, Alert from belief). A battle-readiness force is alert whatever the flag (ch3, What alert is). TF4 is ALERT.
   - Ready: battle readiness from the start of the scenario (ch4, Ready). TF4 is READY.
4. During the fight:
   - TF3. It becomes alert from the end of the round in which it sights the enemy or is fired on (ch3, Alerted inside a pulse). That changes its alert factor for its own later rolls only, and a force that has sighted stops rolling (ch4, Sighting). If it is fired on in a round, it counts as sighted in the next round with no roll (ch4, Sighting). So once TF3 is alert nothing in this fight reads that change, except a later round's roll which it no longer makes. Its readiness does not change in the fight ("no round changes it", ch4, Ready), so it stays NOT READY. Its gun and torpedo shares stay at the unready values.
   - TF4. Already alert and ready. No change.
   - Alarm. The alarm rolls only for other forces in the hex (ch3, The alarm). There are none, so no alarm is rolled.
5. What changes what it fires: the readiness, which is fixed at the start of the fight. The alertness change in TF3 does not change its shots (ch4, Ready: "The force's readiness does not rise during the fight").

Answer: At the fight's start, TF3 is not alert and not ready; TF4 is alert and ready. TF3 may become alert during the fight, but its readiness stays "not ready" throughout, and alertness changes nothing it fires. TF4 changes nothing.

Q3. Experience factors and round-1 sighting chances

Experience factor, for sighting (ch4, Sighting): 1 + experience_sighting_strength x (mean - 2), in a dark fight, mean = mean experience of the ships in the force's list.
- TF3 ships 3, 4, 5 each experience 1 (scenario [[ship]] "experience = 1"). Mean = 1.000000. Factor = 1 + 0.4 x (1 - 2) = 1 + (-0.4) = 0.600000.
- TF4 ships 6, 7, 8 each experience 3. Mean = 3.000000. Factor = 1 + 0.4 x (3 - 2) = 1.400000.

Experience factor for each ship's guns and torpedoes (ch4, The hit chance): 1 + strength x (own experience - 2), with the firing ship's own value.
- Allied ship: 1 + 0.4 x (1 - 2) = 0.600000 (gun and torpedo strengths are both 0.4).
- Japanese ship: 1 + 0.4 x (3 - 2) = 1.400000.

Round 1 sighting chance (ch4, Sighting):
chance = min(1, (base + radar_bonus) x alert_factor x experience_factor) x fatigue_factor
- base = sighting_chance_night = 0.5 (dark). radar_bonus = 0 (no class with radar 1 in either force; ship_classes.csv radar column 0 for class 1).
- TF3 (Allied) rolling for TF4: (0.5 + 0) x alert factor 0.5 (not_alert_sighting_factor, since TF3 is not alert) x 0.6 = 0.150000. Cap: no effect. Fatigue factor 1. Chance = 0.150000.
- TF4 (Japanese) rolling for TF3: (0.5 + 0) x alert factor 1 (alert) x 1.4 = 0.700000. Cap: no effect. Fatigue 1. Chance = 0.700000.

Order: TF3 rolls first (lower ID), then TF4. Each uses its own stream keyed on the force and the enemy (ch4, Sighting; ch1, Every draw is keyed), so the two rolls are independent. Round-1 outcomes (rounded to none; exact products):
- Both sight: 0.15 x 0.70 = 0.105000. Both fire.
- TF3 sights, TF4 does not: 0.15 x 0.30 = 0.045000. TF4 is surprised (ch4, Surprise: it has not sighted, the enemy has, and the enemy has shots). TF3 fires; TF4 does not.
- TF4 sights, TF3 does not: 0.85 x 0.70 = 0.595000. TF3 is surprised; TF4 fires; TF3 does not.
- Neither: 0.85 x 0.30 = 0.255000. No fire in round 1.

Q4. Gun shots per round

Full shots (ch4, Gun shots in a round): full = gun_rounds_per_min x (main_guns x round_minutes) = 3 x (8 x 5) = 120.000000 per cruiser per round with the main battery working.
shots = min(full x main_battery_factor x unready_gun_share, ammunition aboard). Share is applied before the ammunition cap.
Ammunition aboard = main_guns x gun_rounds_per_gun = 8 x 100 = 800 rounds. Six rounds at 120 is 720, so ammunition never binds in this fight.

(a) One Japanese cruiser, main battery working. TF4 is ready, so unready share = 1.
   shots = min(120 x 1 x 1, 800) = 120.000000.
(b) One Allied cruiser, main battery working. TF3 is not ready, so unready_gun_share = 0.5.
   shots = min(120 x 1 x 0.5, 800) = 60.000000.
(c) One Allied cruiser, main battery degraded. Factor = degraded_factor 0.5 (surface_combat.toml), unready 0.5.
   shots = min(120 x 0.5 x 0.5, 800) = 30.000000.

Note: these are shots if the ship fires in the round (its force has sighted, or was fired on, and it has a target: ch4, Who fires).

Q5. Chance per shot (ch4, The hit chance)

chance = base x surprised_factor x lit_factor x fire_control_factor x experience_factor, multiplied in that order.
Base: dark, so gun = gun_hit_chance_night 0.03, torpedo = torpedo_hit_chance_night 0.04.
surprised_factor = surprised_target_factor 1.5 if the target's force is surprised, else 1.
lit_factor = lit_target_factor 2 if the fight is dark and the target has fire damage above 0, else 1.
fire_control_factor (guns only) = 1 working, degraded_factor 0.5 degraded.
experience_factor: Japanese 1.4, Allied 0.6 (Q3).

Results (cut and rounded: none; all exact):

Japanese firer
- Gun, (a) neither surprised nor lit: 0.03 x 1 x 1 x 1 x 1.4 = 0.042000
- Gun, (b) surprised: 0.03 x 1.5 x 1 x 1 x 1.4 = 0.063000
- Gun, (c) lit: 0.03 x 1 x 2 x 1 x 1.4 = 0.084000
- Torpedo, (a) neither: 0.04 x 1 x 1 x 1 x 1.4 = 0.056000
- Torpedo, (b) surprised: 0.04 x 1.5 x 1 x 1 x 1.4 = 0.084000

Allied firer
- Gun, (a) neither: 0.03 x 0.6 = 0.018000
- Gun, (b) surprised: 0.03 x 1.5 x 0.6 = 0.027000
- Gun, (c) lit: 0.03 x 2 x 0.6 = 0.036000
- Torpedo, (a) neither: 0.04 x 0.6 = 0.024000
- Torpedo, (b) surprised: 0.04 x 1.5 x 0.6 = 0.036000

Lit torpedo (not asked; see Assumptions): Japanese 0.04 x 2 x 1.4 = 0.112000; Allied 0.04 x 2 x 0.6 = 0.048000.

Allied gun, fire control degraded, target neither surprised nor lit:
0.03 x 1 x 1 x 0.5 x 0.6 = 0.009000.

Q6. Expected gun hits per salvo; torpedo shots

Expected hits = shots x chance (ch4, The hit chance: "expected hits recorded are shots x chance").
- One Allied gun salvo (60 shots at 0.018): 60 x 0.018 = 1.080000 hits.
- One Japanese gun salvo (120 shots at 0.042): 120 x 0.042 = 5.040000 hits.

Torpedo shots (ch4, Torpedo shots):
- Class 1: torpedo_tubes T = 8, torpedo_tubes_one_side B = 8. B = T, so one salvo of all 8 tubes; no second side (ch4, Torpedo shots).
- Shots = tubes x torpedo_mount_factor x unready_torpedo_share, with torpedoes_ready set at scenario start (ch4).
  - Japanese cruiser (TF4 ready): 8 x 1 x 1 = 8.000000 shots.
  - Allied cruiser (TF3 not ready, but unready_torpedo_share = 1 in reference-16): 8 x 1 x 1 = 8.000000 shots.
- Each cruiser fires one torpedo salvo of 8 shots in the first round its force fires. Three cruisers per side, so 24 torpedo shots per side in a fight in which all three fire.
- For information only: expected torpedo hits, neither surprised nor lit: Japanese 8 x 0.056 = 0.448000; Allied 8 x 0.024 = 0.192000.

=====================================================
SCENARIO 15b (night fight, TF1 Japanese attacking, TF2 Allied)
=====================================================

Setup and meeting (needed for Q7 to Q9)
- TF1 (Japanese cruisers 1 to 3) at (0,0), battle readiness, route to (10,0), cruise. TF2 (Allied cruisers 4 to 6) at (5,0), normal readiness, no route, idle. Map: placeholder_night_action.toml, deep sea q -1..12, r -1..1. Way is clear along row r = 0; every step is 40.
- TF1 budget: time 16 x 12 = 192.000000; fuel 2000 x 8000 / 2000 = 8000.000000; budget 192.000000 miles.
- Entry into (5,0): five steps east. Position changes at 20, 60, 100, 140, 180 miles. Entry at 180.000000 miles.
- Time: 180 / 16 = 11.250000 h, so D1 05:15 (night pulse, dark).
- TF2 is idle and not moving, so the fight is held at (5,0) (ch4, Meeting).
- Readiness of TF2 at the fight's start: normal, not alert (no contacts at start; no alert from belief), so NOT READY (ch4, Ready; ch3, What alert is).

Q7. Is force B ready, and gun shots and hits of one of its cruisers
- Force B ready when the fight begins? NO (as above).
- Gun shots per round, main battery working: full 120; x unready_gun_share 0.5 = 60.000000 shots (ammunition 800 not limiting).
- Expected hits, target neither surprised nor lit: chance = 0.03 x 1 x 1 x 1 x 1 (experience factor 1: strength 0) = 0.030000. Hits = 60 x 0.03 = 1.800000.
- Applies once the cruiser fires, i.e. once force B has sighted or been fired on.

Q8. Torpedo shots of one of force B's cruisers and expected hits
Tubes T = 8, B = 8: one salvo of all tubes. Not ready, so unready_torpedo_share = 0.5 applies (surface_combat.toml, reference-15b).
Chance = torpedo_hit_chance_night 0.04 x 1 x 1 x 1 (torpedo has no fire-control factor) x 1 (experience strength 0) = 0.040000.
(a) Mounts sound: shots = 8 x 1 x 0.5 = 4.000000. Expected hits = 4 x 0.04 = 0.160000.
(b) Mounts degraded: shots = 8 x degraded_factor 0.5 x 0.5 = 2.000000. Expected hits = 2 x 0.04 = 0.080000.
After the salvo: NO, she cannot fire her remaining tubes later. Because B = T there is no second side. A salvo with shots above 0 spends its side "however few tubes fired" (ch4, Torpedo shots). Its class has no second side, so the rule "After any other salvo, torpedoes_ready and torpedoes_one_side_fired are both cleared" applies: torpedoes_ready is cleared and no torpedo is fired afterwards. The book also says each tube fires once in a game (no reloads).

Q9. Same cruiser, force B ordered to battle readiness, at night
- Battle readiness makes the force ready (ch4, Ready). Share = 1.
- Shots = 8 x 1 x 1 = 8.000000. Chance 0.040000 as before. Expected hits = 8 x 0.04 = 0.320000.
- (Not asked: the battle order also makes TF2 alert, so its sighting alert factor is 1 rather than 0.5.)

Q10. The same fight by day, in the morning pulse
Changes by day (ch1, Darkness; ch4): not dark. Sighting base becomes sighting_chance_day 1 (not used here). Torpedo base becomes torpedo_hit_chance_day 0.02. No lit factor. Experience factor is 1 in any case (strength 0). Neither surprised nor lit is assumed.

Assumed geometry for the day case: the scenario as written puts the fight at (5,0) at 180 miles, which is more than the morning budget (6 h x 16 kn = 96 miles, ch2 budget). I therefore changed only the pulse and kept the positions. The numbers below depend on pulse and readiness only.

(a) Force B alerted in the night pulse, still alert at the start of the morning pulse, so ready (ch4, Ready: "A force alerted in an earlier pulse and still alert is ready"). Share 1.
- Torpedo shots (sound mounts): 8 x 1 x 1 = 8.000000.
- Chance: 0.02 x 1 x 1 x 1 x 1 = 0.020000.
- Expected hits: 8 x 0.02 = 0.160000.
- Degraded mounts (not asked, for completeness): 8 x 0.5 x 1 = 4.000000 shots; hits 0.080000.

(b) Force B never alerted: not ready (ch4, Ready). Share 0.5.
- Torpedo shots (sound mounts): 8 x 1 x 0.5 = 4.000000.
- Chance 0.020000.
- Expected hits: 4 x 0.02 = 0.080000.

=====================================================
Where I had to assume or infer
=====================================================
1. Scenario 16 waypoint. The question says task force 4 steams toward (2,-4). The file says waypoints = [[0, -4]]. I used the file; the entry into (2,-4) is 180 miles either way.
2. Entry at half a leg. The rulebook says the force is in the hex ahead "once it is half the leg's charge or more along it (20 miles of a leg of 40)", so entry is at 180, not at the centre (200).
3. Clock zone. Chapter 1 says the day starts at 18:00 "in the scenario's reference zone", and the scenarios state none. I took 18:00 plus elapsed hours, giving D1 05:15. The zone changes only the label.
4. Torpedo lit factor. The question says lit "(guns only)". The chance formula in chapter 4 ("chance = base x surprised_factor x lit_factor x fire_control_factor x experience_factor") and the glossary's "Lit" entry say nothing that limits lit to guns, so by the text a lit target also takes the factor for torpedoes. I gave the torpedo lit values as a note; the question's guns-only answer is the gun figure.
5. Target states. Where the question says "neither surprised nor lit", I took it as stated. For (b) I assumed the target is not lit, and for (c) that it is not surprised. No ship has fire damage at the start, so none is lit before a hit.
6. Round 1 independence. The two sighting rolls use separate streams keyed on each force and its enemy (ch4, Sighting; ch1, "Every draw is keyed"), so I multiplied them. The book states the streams, not the independence in words.
7. Alertness change in TF3 affects nothing in the fight. A force that has sighted does not roll again, and one fired on counts as sighted the next round (ch4, Sighting). The book does not say whether alertness has any other effect inside a fight; I found none in chapters 3 and 4, so I took it as none.
8. Experience for sighting. "The mean is the sum of the experience of the ships in the force's list ... over their count" (ch4, Sighting). All of TF3's ships have experience 1 and all of TF4's have 3, so the mean is those values.
9. Fatigue. Neither scenario names fatigue.toml. ch1 says a rule whose file is not named "does nothing", so fatigue factor 1 (ch4 says "when the scenario names fatigue figures, else 1").
10. Night air. The night pulse air phases (ch1 table) and empty night search do not touch ship gunnery, torpedoes or sighting. I did not open chapter 5 to confirm this.
11. Other forces in 16. TF2 (Japanese carrier) steams from (14,3) to (9,0), and TF1 (Allied carrier) sits at (2,0). Neither shares a hex with the row r = -4 that TF4 uses, so I took them as not meeting TF4 or TF3 in this pulse.
12. Ammunition. The rule says a scenario ship starts with main_guns x gun_rounds_per_gun "unless the scenario gives another figure" (ch4, Gun shots). Neither scenario gives one, so 800.
13. Morning hypothetical (15b Q10). The scenario's positions put the fight 180 miles from TF1's start, which a morning budget of 96 miles cannot reach. I changed only the pulse and kept the hexes, since the numbers depend on pulse and readiness only. The book's "A pulse is a time bucket" (ch1) is consistent with this reading.
14. How force B was alerted in the night pulse (15b Q10a). Scenario 15b gives no contact and no earlier fight. I took the question's "had been alerted" as given. For it to still be alert at the morning start, a fresh contact within 3 hexes must be present, because "Alert from belief ... clears the flag when no fresh contact is near" (ch3, Alert from belief). If it stood down, Q10(a) becomes Q10(b).
15. Torpedoes after one salvo (15b Q8). The rule says "After any other salvo, torpedoes_ready and torpedoes_one_side_fired are both cleared" (ch4, Torpedo shots). With B equal to T, the salvo is "any other salvo", so the mounts are spent. The book's "There are no reloads" supports this. I read the clause as applying, and the book does not say it in those words for this case.
16. Degraded torpedo mounts. I used the torpedo_mounts system degraded, factor degraded_factor 0.5 (ch4, Named systems table, "Torpedo shots times degraded_factor"), with no other reading.
17. Rounding. No quotient or cut changed any result: every product here is exact at six decimals, and every chance is inside 0 to 1.

Files read (all inside sitting1):
- rules/README.md
- rules/01-the-day-orders-and-what-a-side-knows.md
- rules/02-movement-fuel-and-cargo.md
- rules/03-search-contacts-and-claims.md
- rules/04-surface-combat-and-damage.md
- rules/differences.md
- glossary.md
- scenarios/16_all_together.toml
- scenarios/15b_unready_torpedoes.toml
- reference-16/surface_combat.toml
- reference-16/ship_classes.csv
- reference-16/search.toml
- reference-16/port_sizes.csv
- reference-15b/surface_combat.toml
- reference-15b/ship_classes.csv
- reference-15b/search.toml
- reference-15b/port_sizes.csv
- map/placeholder_search_wide.toml
- map/placeholder_night_action.toml
Not opened: rules/05-air-operations.md, rules/06-objectives-and-results.md, the air and aircraft files in reference-16 and reference-15b, and the other-scenario ship-class files. No fatigue.toml exists in either reference folder that the scenarios name. The shell was used only to list the folder and to print the reference and map files.
