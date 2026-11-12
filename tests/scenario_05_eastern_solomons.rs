//! Scenario 5: Eastern Solomons, 24 August 1942.
//!
//! Stage 5e of scenario 5, and the first air battle with real ships and
//! aircraft. Two American carrier forces lie east of Guadalcanal; the
//! Japanese carrier striking force and the Ryujo detachment come down from
//! the north.
//!
//! The figures are derived from sourced facts
//! (`data/reference/ship_classes_eastern_solomons.csv` and
//! `aircraft_types_eastern_solomons.csv`, decisions 0020 and 0029). What is
//! in the scenario and what is left out is
//! `docs/decisions/0031-eastern-solomons-scenario.md`. The search, air
//! combat and scaling figures are placeholders, so the losses are not a
//! historical check yet. This file asserts what the rules fix and prints the
//! spread beside what happened.
//!
//! # Hand calculation
//!
//! Written by the architect before the test code. The tests below assert
//! these numbers. If the code disagrees, the code or the rule is wrong: do
//! not change a number here to make a test pass.
//!
//! Quantities are cut to six decimal places at each step.
//!
//! Brought to the scenario's new orders on Nov 12, 2026 (the `ryujo-raid`
//! change, after decisions 0059 and 0063), by the rules it already works by:
//! Ryujo's detachment reaches (7,-9) at noon and not (7,-10) (200 nm from
//! Henderson Field, as the record has her launch, engagement_events 331 and
//! engagement_positions 310); her attack unit, group 19, raids Henderson
//! Field on extended reach; her fighters keep 0.35 and send the rest; and
//! the dive bomber groups of the fleet carriers, 12 and 15, hold a target
//! list. Each section below says what changed. The figures are the frozen
//! figures of step 1.
//!
//! ## The forces
//!
//! | Task force | Ships, by ID | Air groups, by ID |
//! | ---------- | ------------ | ----------------- |
//! | 1, Task Force 16, Allied | 1 Enterprise, 2 North Carolina, 3 Portland, 4 Atlanta, 5 to 10 destroyers | 1 VF-6, 27 F4F-4, patrol; 2 VB-6, 17 SBD-3, strike; 3 VS-5, 16 SBD-3, search; 4 VT-3, 14 TBF-1, strike |
//! | 2, Task Force 11, Allied | 11 Saratoga, 12 New Orleans, 13 Minneapolis, 14 to 18 destroyers | 5 VF-5, 27 F4F-4, patrol; 6 VB-3, 15 SBD-3, strike; 7 VS-3, 15 SBD-3, strike; 8 VT-8, 13 TBF-1, strike |
//! | 3, carrier striking force, Japanese | 19 Shokaku, 20 Zuikaku, 21 Chikuma, 22 Hiei, 23 Kirishima, 24 Kumano, 25 Suzuya, 26 Nagara, 27 to 32 destroyers | 11 fighters, 26 A6M2, patrol share 0.8; 12 bombers, 27 D3A1, strike, target list carrier, battleship, cruiser, transport; 13 attack, 18 B5N2, strike (Shokaku). 14 fighters, 25, share 0.8; 15 bombers, 27, the same list; 16 attack, 18 (Zuikaku). 17 floatplanes, 5 E13A1, search (Chikuma) |
//! | 4, Ryujo detachment, Japanese | 33 Ryujo, 34 Tone, 35 Amatsukaze, 36 Tokitsukaze | 18 fighters, 23 A6M2, patrol share 0.35; 19 attack, 6 B5N2, base strike on base 1, extended reach (Ryujo). 20 floatplanes, 5 E13A1, search (Tone) |
//!
//! At Henderson Field, base 1, in (2,-4): 9 VMF-223, 13 F4F-4, patrol; 10
//! VMSB-232, 11 SBD-3, strike. The base has no guns and no supply here.
//!
//! | Aircraft | Radius | Attacks with | Pilot armour | Self-sealing tanks |
//! | -------- | ------ | ------------ | ------------ | ------------------ |
//! | F4F-4    | 105 nm | nothing      | yes          | yes                |
//! | SBD-3    | 200 nm | a bomb, damage 75.6 | yes   | yes                |
//! | TBF-1    | 190 nm | a torpedo, damage 73.2 | yes | yes               |
//! | A6M2     | 261 nm | nothing      | no           | no                 |
//! | D3A1     | 252 nm | a bomb, damage 41.666666 | no | no               |
//! | B5N2     | 175 nm | a torpedo, damage 81.64 | no | no                |
//! | E13A1    | 253 nm | nothing      | no           | no                 |
//!
//! ## Where everyone is
//!
//! Both American forces lie stopped in (7,-5) all day.
//!
//! Ships are charged the straight-line miles of their course where the way
//! is clear (decision 0058); decision 0031, amended by it, gives the starts.
//!
//! The carrier striking force starts in (9,-19) and steams for (8,-11), 8
//! hexes, at 17 knots. The way is clear, so it is charged the air distance:
//! dq -1, dr 8, 1 + 64 - 8 = 57, 40 x root 57 = 301.99, 302 nm. The first
//! step, to (8,-18), is charged 302 - 280 = 22 (from (8,-18) the target is 7
//! hexes down an axis: 280), and each of the seven after it 40: 22 + 7 x 40 =
//! 302. It enters each hex after half the step's charge: (8,-18) after 11
//! nm, then (8,-17) after 22 + 20 = 42, (8,-16) 82, (8,-15) 122, (8,-14) 162,
//! (8,-13) 202, (8,-12) 242 and (8,-11) after 282 nm.
//!
//! The Ryujo detachment starts in (7,-18) and steams for (7,-9), 9 hexes down
//! the q = 7 column, an axis: 9 x 40 = 360 nm, at 20 knots, every step 40. It
//! enters (7,-17) after 20 nm and each hex after it 40 nm later, (7,-9) after
//! 340 nm. A force's position is the nearest hex centre. (It started in
//! (7,-19) for (7,-10) before: (7,-10) is 223 nm from Henderson Field by hex
//! centres, which is the grid's error, and the record has her launch from 200
//! to 202 nm of it.)
//!
//! | At | Striking force | Steamed | Ryujo detachment | Steamed |
//! | -- | -------------- | ------- | ---------------- | ------- |
//! | 06:00 | (8,-13), 22 nm past the centre of (8,-14) on a step of 40 | 204 nm | (7,-12), at its centre | 240 nm |
//! | 12:00 | (8,-11), arrived at hour 17.764705 (11:46) | 302 nm | (7,-9), arrived at 12:00, hour 18 | 360 nm |
//! | 18:00 | (8,-11) | 302 nm | (7,-9) | 360 nm |
//!
//! The striking force arrives 302 / 17 = 17.764705 hours after the start:
//! 12 hours for the night's 204 nm and 98 / 17 = 5.764705 into the morning, 14
//! minutes before the noon the fix is for. It steams 98 nm in the morning.
//!
//! Air distances, in nm: the straight line between hex centres, 40 times
//! the square root of (dq x dq + dr x dr + dq x dr), to the nearest mile
//! (decision 0040). The hex count times 40 is given beside each for
//! comparison; the rules do not use it for aircraft.
//!
//! | At | American carriers to striking force | American carriers to Ryujo | Henderson Field to Ryujo |
//! | -- | ----------------------------------- | -------------------------- | ------------------------ |
//! | 06:00 | (1,-8): 40 x root 57 = 302 (by hexes 320) | (0,-7): 280 (280) | (5,-8): 40 x root 49 = 280 (320) |
//! | 12:00 | (1,-6): 40 x root 31 = 223 (240) | (0,-4): 160 (160) | (5,-5): 40 x root 25 = 200 (200) |
//!
//! (Before decision 0040 every air distance here was the hex count times
//! 40. The figures below that changed with it are marked.)
//!
//! ## Fuel
//!
//! Shokaku cruises at 18 knots, so at 17 she burns at her cruise rate, 4100
//! t over 9700 nm: 86.226804 t in the night (204 nm) and 41.42268 t in the
//! morning (98 nm), nothing in the afternoon, 127.649484 t in all, and she has
//! 3972.350516 t left. Ryujo cruises at 14
//! knots, so at 20 her burn is multiplied by (20 / 14) squared: 1.428571
//! squared is 2.040815. She burns 240 x 2490 / 10000 = 59.76, times that,
//! 121.959104 t in the night, and 60.979552 t in the morning, and has
//! 2307.061344 t left.
//!
//! ## The morning
//!
//! Every search is flown. No force is within reach of any search (the nearest
//! enemy is 280 nm away, and the searches are 200 and 253 nm), so there is no
//! attempt, no contact, no strike and no loss in the morning of any run. Nor
//! does group 19's base strike launch: Henderson Field is 280 nm from Ryujo
//! at 06:00, and the group's extended reach is 175 x 1.25 = 218.75, cut to 218
//! (decision 2 of 0059). (A launch needs the base in reach when the pulse
//! begins, decision 7 of 0063.)
//!
//! ## The afternoon: search
//!
//! The chance of a sighting is 57.295779 x aircraft x 20 / (arc x distance).
//!
//! | Searcher | From | Arc | Target | Bearing | Distance | Chance |
//! | -------- | ---- | --- | ------ | ------- | -------- | ------ |
//! | Group 3, 16 SBD-3 | (7,-5) | 290 to 090, 160 degrees, to 200 nm | Ryujo detachment | 330 | 160 | 57.295779 x 16 x 20 / (160 x 160) = 18334.64928 / 25600 = 0.716197 |
//! | Group 3 | (7,-5) | | Striking force | | 223 | beyond 200 nm: no attempt |
//! | Group 17, 5 E13A1 | (8,-11) | 110 to 200, 90 degrees, to 253 nm | Each American force | about 159 | 223 | 57.295779 x 5 x 20 / (90 x 223) = 0.285479 |
//! | Group 20, 5 E13A1 | (7,-9) | 110 to 200 | Each American force | 150 | 160 | 57.295779 x 5 x 20 / (90 x 160) = 5729.5779 / 14400 = 0.397887 |
//!
//! (Ryujo at (7,-10) was 200 nm from the carriers: group 3's chance was
//! 0.572957 and group 20's 0.318309.)
//!
//! Each floatplane group rolls once for each of the two American forces, so
//! the Japanese make four attempts. The chance that none sights is
//! 0.714521 x 0.714521 x 0.602113 x 0.602113 = 0.185090, so the Japanese
//! side holds a contact in the afternoon with chance 0.814910. (At 240 nm,
//! before decision 0040, group 17's chance was 0.265258.)
//!
//! | Count of runs, over seeds 1 to 1000 | p        | Mean   | Std dev | Allowed    |
//! | ----------------------------------- | -------- | ------ | ------- | ---------- |
//! | The Allied side sights the Ryujo detachment | 0.716197 | 716.20 | 14.26 | 660 to 773 |
//! | The Japanese side sights an American force  | 0.814910 | 814.91 | 12.28 | 766 to 864 |
//!
//! ## The afternoon: who launches
//!
//! A group launches at a contact its side holds within its radius of where
//! the group is. A report names the true hex or one next to it. A group with
//! a target list (decision 0063) launches only at a contact whose reported
//! kind is on it, the one that comes first on the list, then the nearest.
//!
//! Distances from the American carriers' (7,-5) and from Henderson Field
//! (2,-4) to the true hex (7,-9) and the six hexes next to it, 40 x root (dq x
//! dq + dr x dr + dq x dr) to the nearest mile:
//!
//! | Hex | From (7,-5) | From (2,-4) |
//! | --- | ----------- | ----------- |
//! | (7,-9), the true hex | 160 | 200 |
//! | (8,-9) | 144 | 223 |
//! | (8,-10) | 183 | 240 |
//! | (7,-10) | 200 | 223 |
//! | (6,-9) | 183 | 183 |
//! | (6,-8) | 144 | 160 |
//! | (7,-8) | 120 | 183 |
//!
//! - **Groups 2, 6 and 7, the SBD-3s (radius 200):** launch if the contact's
//!   hex is within 200 nm of (7,-5), which every one of the seven is. Only
//!   the true hex has Ryujo; at any other they find nothing.
//! - **Groups 4 and 8, the TBF-1s (radius 190):** launch if it is within 190
//!   nm, which is every hex but (7,-10), at 200. (They stayed on the deck
//!   when the true hex was 200 nm away, and now fly at 160.)
//! - **Group 10 at Henderson Field (radius 200):** launches if the hex is
//!   within 200 nm of (2,-4): (7,-9), at 200 exactly, (6,-9), (6,-8) and
//!   (7,-8). It does not at (8,-9), (8,-10) or (7,-10). (The true hex was 223
//!   nm away, and group 10 launched only at a neighbour.)
//! - **Groups 12 and 15, the D3A1s (radius 252, target list carrier,
//!   battleship, cruiser, transport):** launch if the Japanese side holds a contact
//!   reported as one of those kinds within 252 nm of (8,-11), which the true
//!   hex (7,-5) is, at 223 nm. Of the hexes next to it only (7,-4), at 262
//!   nm, is out of reach. A contact reported as a destroyer or an
//!   auxiliary is held for: the groups stay on deck. A report of a carrier
//!   is read through the confusion table of the search figures, so some are.
//!   The launch cites the mission order, the target order and the report.
//! - **Groups 13 and 16, the striking force's B5N2s:** radius 175 nm, and
//!   every hex a report of the American forces can name is 183 nm or more
//!   from (8,-11). They never launch.
//! - **Group 19, Ryujo's B5N2s, a base strike on base 1:** needs no contact.
//!   It launches in the first daylight pulse in which base 1 is within its
//!   reach (decision 7 of 0063): 280 nm from Ryujo at 06:00, beyond its
//!   extended reach of 218, and 200 nm at noon, within it. So it launches in
//!   the afternoon, all 6 aircraft, once, in every run, and cites its orders
//!   (the mission and the reach). Ryujo's 15 escorts go with it.
//!
//! A strike attacks only if it flew to the hex its target is really in.
//!
//! ## An American raid on the Ryujo detachment
//!
//! Every American group that launches at the true hex (7,-9) attacks Ryujo
//! there, and all six launch at it (see above), so a run has all three raids
//! or none. The raids are grouped by where they leave from (decision 0027):
//! Enterprise's, groups 2 (17 SBD-3) and 4 (14 TBF-1), 31 aircraft;
//! Saratoga's, groups 6 and 7 (15 SBD-3 each) and 8 (13 TBF-1), 43 aircraft;
//! and Henderson Field's, group 10 (11 SBD-3), 11 aircraft. None has an
//! escort: the target is 160 nm from the carriers and 200 nm from Henderson
//! Field and the F4F-4's radius is 105 (decision 0030). Before the change
//! only the SBD-3s of the carriers flew, 17 and 30 aircraft, at 200 nm.
//!
//! Ryujo's fighters, 23 with a share of 0.35, keep 8 overhead (23 x 0.35 =
//! 8.05, rounded down). The other 15 are escorts, and fly only because group
//! 19 launches (below). The patrol over a hex is divided among the raids that
//! attack there in a pulse (decision 0033): a raid's share is the patrol times
//! its aircraft, bombers and escorts together, over the aircraft of all the
//! raids, rounded down, and the fighters left over go one each to the raids in
//! order of size, the largest first. The patrol and the raids are counted as
//! they stood at the start of the pulse. The three raids, of 31, 43 and 11
//! aircraft, 85 in all, divide the 8:
//!
//! | Raid | Share of the 8 |
//! | ---- | -------------- |
//! | Enterprise's, 31 | 8 x 31 / 85 = 2.92: 2 |
//! | Saratoga's, 43 | 8 x 43 / 85 = 4.05: 4, and the one left over: 5 |
//! | Henderson Field's, 11 | 8 x 11 / 85 = 1.04: 1 |
//!
//! A shot at an SBD-3 or a TBF-1 has both protection factors against it: a
//! fighter's 0.3 x 0.75 x 0.75 = 0.16875, a gun's 0.05 x 0.75 x 0.75 =
//! 0.028125. The detachment's ships have 13 + 11 + 1 + 1 = 26 anti-aircraft
//! shots.
//!
//! | Step | Shots | Chance | Expected | Std dev |
//! | ---- | ----- | ------ | -------- | ------- |
//! | Patrol at Enterprise's bombers | 2 | 0.16875 | 0.3375 | 0.529668 |
//! | Patrol at Saratoga's bombers | 5 | 0.16875 | 0.84375 | 0.837477 |
//! | Patrol at Henderson Field's bombers | 1 | 0.16875 | 0.16875 | 0.374531 |
//! | Guns at the bombers, each raid | 26 | 0.028125 | 0.73125 | 0.843021 |
//!
//! Each shot is rolled on its own (decision 0033), so a roll scores from no
//! hit to its shots. The bombers that attack are the raid's less both losses.
//! Over the runs the hits of each roll are within four standard deviations of
//! its expected figure summed over the raids. A group whose bombers are all
//! shot down attacks nothing, and records neither an attack nor a strike that
//! found nothing. A raid's losses come off its lowest group first (2 before
//! 4; 6, then 7, then 8). Every bomber attacks Ryujo, ship 33: a dive
//! bomber's bombs at 0.2 a bomb, a hit doing 100 x 75.6 / 424 = 17.830188
//! points; a torpedo plane's torpedo at 0.15, a hit doing 100 x 73.2 / 424 =
//! 17.264150 points, always on the flotation track. (Before the change the
//! raids were Enterprise's 17 and Saratoga's 30, sharing 6 as 2 and 4, a
//! raid alone meeting all 6.)
//!
//! ## A Japanese raid on the American carriers
//!
//! Shokaku's raid is group 12, 27 D3A1, and Zuikaku's is group 15, 27 D3A1.
//! They are two raids, because they leave from two ships. The target is 223
//! nm away and the A6M2's radius is 261, so the escorts go: Shokaku's 26
//! fighters keep 20 (20.8 rounded down) and send 6; Zuikaku's 25 keep 20 and
//! send 5.
//!
//! The American patrol over (7,-5) as it stood at the start of the pulse is
//! groups 1 and 5, 54 fighters. Henderson Field's fighters are in another
//! hex. The two raids divide it (decision 0033). When both attack whole,
//! Shokaku's has 27 + 6 = 33 aircraft and Zuikaku's 27 + 5 = 32:
//!
//! | Raid | Share of the 54 |
//! | ---- | --------------- |
//! | Shokaku's, 33 | 54 x 33 / 65 = 27.41: 27, and the one left over: 28 |
//! | Zuikaku's, 32 | 54 x 32 / 65 = 26.58: 26 |
//!
//! Each raid's share fires at its escorts one for one, and what is left of
//! the share at its bombers. A raid alone meets all 54.
//!
//! A shot at an F4F-4 is at 0.16875. A shot at an A6M2 or a D3A1, which have
//! no protection, is at 0.3, and a gun's at 0.05. The strike attacks task
//! force 1, whose ships have 22 + 34 + 15 + 20 + 10 + 5 + 5 + 4 + 4 + 4 =
//! 123 anti-aircraft shots.
//!
//! | Step | Shokaku's raid | Zuikaku's raid |
//! | ---- | -------------- | -------------- |
//! | Escorts at the patrol | 6 shots x 0.16875 = 1.0125 expected | 5 x 0.16875 = 0.84375 |
//! | Patrol at the escorts | 6 x 0.3 = 1.8 | 5 x 0.3 = 1.5 |
//! | Patrol at the bombers | (28 - 6) = 22 x 0.3 = 6.6 | (26 - 5) = 21 x 0.3 = 6.3 |
//! | Guns at the bombers | 123 x 0.05 = 6.15 | 6.15 |
//! | Bombers that attack | 27 less both losses: 14.25 on average | 14.55 on average |
//!
//! Each shot is rolled on its own (decision 0033): a roll of n shots at
//! chance p scores from no hit to n, but no more than the aircraft it fires
//! at, with a standard deviation of the square root of n x p x (1 - p):
//! 0.917410 and 0.837477 for the escorts, 1.122497 and 1.024695 for the
//! patrol at the escorts, 2.149418 and 2.1 for the patrol at the
//! bombers, 2.417126 for the guns. Over the runs the hits of each roll are
//! within four standard deviations of its expected figure times the number
//! of such raids. (Before decision 0033 each roll scored the whole part of
//! its expected figure or one more, each raid met all 54 of the patrol,
//! and 5 to 7 bombers of each attacked.)
//!
//! Losses to the American patrol come off group 1 first. Every bomber
//! attacks Enterprise, ship 1, at 0.2 a bomb. A hit does 100 x 41.666666 /
//! 792 = 5.260942 points. Saratoga's force is never attacked.
//!
//! ## Ryujo's raid on Henderson Field
//!
//! Group 19, 6 B5N2, has a base strike order on base 1 and extended reach
//! (the record has her launch 15 fighters and 6 attack planes at it from 200
//! to 202 nm: engagement_outcomes 1171, engagement_events 331). It launches
//! in the afternoon (above): 6 aircraft at 200 nm, which is 25 beyond its
//! radius of 175 and within its reach of 218. Group 18 flies 23 x 0.35 =
//! 8.05, so 8 on patrol and 15 escorts with the raid (decision 2 of 0027); the
//! escorts fly 200 nm, within the A6M2's radius of 261, and roll nothing for
//! fuel. The raid is one raid, of 6 + 15 = 21 aircraft, on Henderson Field.
//!
//! Over Henderson Field the defending patrol is VMF-223, group 9: 13 F4F-4 on
//! a share of 1, all 13 on patrol. A raid alone meets all of it: 13 x 21 / 21 =
//! 13. The base has no guns here, so no gun roll. The escorts fire first, one
//! shot each at the patrol, then the patrol at the escorts, one shot each, up
//! to its share, and what is left of the share at the bombers (none: 13 of
//! 15 escorts are fired at, none at the bombers). The streams are keyed on the
//! base for the raid's rolls, and on (the group, the base) for the bombs.
//!
//! | Step | Shots | Chance | Expected | Std dev |
//! | ---- | ----- | ------ | -------- | ------- |
//! | Escorts at the patrol | 15 | 0.16875 (an F4F-4) | 2.53125 | 1.450552 |
//! | Patrol at the escorts | 13 | 0.3 (an A6M2) | 3.9 | 1.652271 |
//! | Patrol at the bombers | 0 | | | |
//! | Bombs on the base | 6 | 0.2 | 1.2 | 0.979796 |
//!
//! Each hit of the patrol destroys an escort (group 18, no more than its 15);
//! each hit of the escorts a fighter of group 9 (no more than 13). All 6 B5N2
//! attack. The bombs fall at the bomb hit chance whatever the weapon the type
//! carries (decisions 6 and 7 of 0034), and each hit lands on aircraft, on
//! supply or on nothing, by one more draw. The aircraft on the ground are
//! those at the base that are not flying in the pulse: VMF-223's 13 are all on
//! patrol; VMSB-232's 11 are in the air if the group launched at Ryujo, and on
//! the ground if it did not (in 427 of 1000 runs). Each hit on aircraft
//! destroys one aircraft on the ground, the lowest group first, no more than
//! were there; Henderson Field has no supply, so a hit on supply destroys
//! nothing.
//!
//! Fuel (decisions 3 to 5 of 0059): a sortie whose one-way distance d was
//! beyond its type's radius R rolls each aircraft that came back, on its own,
//! at the ditch figure 2 times (d - R) / R. Here d = 200, R = 175: 25 / 175 =
//! 0.142857, times 2, 0.285714 each, and 6 x 0.285714 = 1.714284 expected
//! aircraft out of fuel (std dev 1.106566). The roll is one stream for the
//! group, after the air combat step and before the aircraft whose deck is
//! lost come home. A ditched aircraft is gone for good, and the pilots are
//! not counted in this scenario. No other sortie of the scenario is beyond
//! its radius: group 10 flies 200 nm on a radius of 200.
//!
//! Over the runs the hits of each roll are within four standard deviations of
//! its expected figure summed over the runs.
//!
//! ## When a deck is lost
//!
//! A ship that sinks, or whose flight deck goes out, in the afternoon's air
//! combat step sends the aircraft her groups have in the air to another deck,
//! a base or the sea (decisions 7 to 10 of 0059). The aircraft in the air are
//! those each group flew in the pulse and did not lose: its search, its
//! strike, its patrol and the escorts that went, less those that ran out of
//! fuel on the way home. Those aboard a ship that sinks, ready and damaged, go
//! down with her, so `AirGroupLost` names the aircraft aboard and not the
//! aircraft in the air. Ryujo's groups fly everything they can in the pulse:
//! group 18's 23 are 8 on patrol and 15 escorts, group 19's 6 are the raid, so
//! when she sinks in the air combat step nothing of either is aboard and
//! `AirGroupLost` names 0 for each (before the change it named 17 and 6, 23
//! lost with her). The test works it
//! for every run: where each group's aircraft go is the first of these that
//! has room, in order of group number: another carrier of the group's side in
//! the same hex, afloat, her flight deck not out, lowest ship number first and
//! as many as fit under her aircraft capacity (a ship with floatplanes is not
//! a carrier); the nearest base of the side within the aircraft type's radius
//! of the hex (Henderson Field is 183 nm from the American force: within the
//! SBD-3's radius of 200 and beyond the F4F-4's of 105); the sea, beside the ships
//! of the group's own side if one is afloat in the hex. Each landing and each
//! ditching is asserted whole: the groups, the place, the aircraft, whether
//! the group was formed, the causes (the event that closed the deck, then the
//! events that say the group flew), and the chance, 1 on the frozen figures,
//! so no pilot is lost. On the scenario's own seed, Ryujo sinks with all of
//! group 18's and group 19's aircraft in the air, lost with her none: group
//! 18's 21 (23 less the 2 that VMF-223 shot down over Henderson Field) and
//! group 19's 3 (6 less the 3 that ran out of fuel) come down in the sea
//! beside her detachment's other ships. Enterprise's deck goes out with
//! groups 1, 2, 3 and 4 in the air: 21 of group 1 land on Saratoga and join
//! group 5 (she has room), the other 3 come down in the sea, groups 2 and 3,
//! 16 each, land at Henderson Field, within their radius of 200 nm, and join
//! group 10, and group 4's 14 (TBF-1, radius 190) land there too, 183 nm from
//! the carriers, and form group 21 (decision 8 of 0059).
//!
//! ## What cannot happen
//!
//! - No ship fires a gun or a torpedo at another ship: no two opposing
//!   forces are ever in one hex.
//! - No F4F-4 ever escorts a raid, and no B5N2 of the striking force ever
//!   launches. No B5N2 attacks a ship: Ryujo's attack unit launches only its
//!   base strike.
//! - No aircraft at Henderson Field is ever lost with a ship, and no ship of
//!   task force 2 or task force 3 is ever damaged. (Its fighters are shot
//!   down by Ryujo's escorts, and its dive bombers on the ground or over
//!   Ryujo; aircraft land there when a deck is lost, and group 10 receives
//!   them.)
//! - No aircraft lands elsewhere or comes down for want of a deck unless its
//!   group's ship sank or lost her flight deck in that pulse; none for a
//!   group at Henderson Field; none in a pulse with no air combat step (the
//!   night, the morning).
//! - The striking force is never sighted and never attacked.
//!
//! # Properties
//!
//! - Every naval strike is launched at a contact its own side holds, reported
//!   in that pulse or the one before, within the launching type's radius, and
//!   of a kind on the group's target list.
//! - Every escort sent is within its fighter type's radius of the raid's
//!   target.
//! - Every group's aircraft at the end are its aircraft at the start less
//!   the aircraft its loss events record (shot down, lost with a sunk ship,
//!   destroyed on the ground, ditched for fuel, and ditched for want of a
//!   deck), less what landed elsewhere from it, plus what landed elsewhere to
//!   it, and none goes below zero. A group aboard a ship that sinks has none.
//! - No ship has negative fuel, and no damage track is above 100.
//! - Every damage event is caused by an attack, every attack by a launch,
//!   and every launch by a report to the launching side.
//! - A replay of any seed reproduces the daily hash.
//!
//! # What happened, for comparison only
//!
//! Saratoga's 30 dive bombers and 8 torpedo planes sank Ryujo for no loss
//! or one. The Japanese first wave, 27 dive bombers and 10 fighters, hit
//! Enterprise with three bombs and lost 24 aircraft; a second wave of the
//! same size did not find the American force. No American ship was sunk.
//! The test prints how often each carrier is sunk and how many aircraft each
//! side loses over seeds 1 to 1000, and asserts nothing about those counts.

mod common;

use std::cmp::Reverse;
use std::collections::{BTreeMap, BTreeSet};
use std::path::PathBuf;
use std::sync::OnceLock;

use sim_core::bearing;
use sim_core::events::{
    AirFirer, AirTarget, Cause, DamageTrack, Event, EventKind, SortieKind, Weapon,
};
use sim_core::fixed::Fixed;
use sim_core::hex::Hex;
use sim_core::ids::{
    AirGroupId, BaseId, ContactId, EntityRef, EventId, OrderId, ShipId, TaskForceId,
};
use sim_core::map::Map;
use sim_core::orders::{AirMission, Reach};
use sim_core::refdata::{
    AirCombatFigures, AircraftType, AttackWeapon, SearchFigures, ShipKind, SurfaceCombatFigures,
};
use sim_core::rng::{Draw, RngKey, RngStream, Subsystem};
use sim_core::state::{AirGroupLocation, ShipLocation, ShipSystem, Side, SystemState};
use sim_core::time::{self, Pulse};
use sim_data::Scenario;

const SEEDS: u64 = 1000;

// ---------------------------------------------------------------------
// The forces and groups, as the hand calculation's tables give them.
// ---------------------------------------------------------------------

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Place {
    Ship(u32),
    Base(u32),
}

struct Spec {
    id: u32,
    side: Side,
    ready: u32,
    /// The aircraft type's ID.
    ty: u32,
    mission: AirMission,
    /// The patrol share in hundredths.
    share: i64,
    place: Place,
}

const fn group(
    id: u32,
    side: Side,
    ready: u32,
    ty: u32,
    mission: AirMission,
    share: i64,
    place: Place,
) -> Spec {
    Spec {
        id,
        side,
        ready,
        ty,
        mission,
        share,
        place,
    }
}

const F4F4: u32 = 2000;
const SBD3: u32 = 2001;
const TBF1: u32 = 2002;
const A6M2: u32 = 2003;
const D3A1: u32 = 2004;
const B5N2: u32 = 2005;
const E13A1: u32 = 2006;

const PATROL: AirMission = AirMission::CombatAirPatrol;
const STRIKE: AirMission = AirMission::NavalStrike;
/// Ryujo's attack unit: a base strike on Henderson Field, base 1.
const RAID: AirMission = AirMission::BaseStrike(BaseId::new(1));
const SEARCH: AirMission = AirMission::Search;

/// The first table: the air groups, by ID.
const GROUPS: [Spec; 20] = [
    group(1, Side::Allied, 27, F4F4, PATROL, 100, Place::Ship(1)),
    group(2, Side::Allied, 17, SBD3, STRIKE, 100, Place::Ship(1)),
    group(3, Side::Allied, 16, SBD3, SEARCH, 100, Place::Ship(1)),
    group(4, Side::Allied, 14, TBF1, STRIKE, 100, Place::Ship(1)),
    group(5, Side::Allied, 27, F4F4, PATROL, 100, Place::Ship(11)),
    group(6, Side::Allied, 15, SBD3, STRIKE, 100, Place::Ship(11)),
    group(7, Side::Allied, 15, SBD3, STRIKE, 100, Place::Ship(11)),
    group(8, Side::Allied, 13, TBF1, STRIKE, 100, Place::Ship(11)),
    group(9, Side::Allied, 13, F4F4, PATROL, 100, Place::Base(1)),
    group(10, Side::Allied, 11, SBD3, STRIKE, 100, Place::Base(1)),
    group(11, Side::Japanese, 26, A6M2, PATROL, 80, Place::Ship(19)),
    group(12, Side::Japanese, 27, D3A1, STRIKE, 100, Place::Ship(19)),
    group(13, Side::Japanese, 18, B5N2, STRIKE, 100, Place::Ship(19)),
    group(14, Side::Japanese, 25, A6M2, PATROL, 80, Place::Ship(20)),
    group(15, Side::Japanese, 27, D3A1, STRIKE, 100, Place::Ship(20)),
    group(16, Side::Japanese, 18, B5N2, STRIKE, 100, Place::Ship(20)),
    group(17, Side::Japanese, 5, E13A1, SEARCH, 100, Place::Ship(21)),
    group(18, Side::Japanese, 23, A6M2, PATROL, 35, Place::Ship(33)),
    group(19, Side::Japanese, 6, B5N2, RAID, 100, Place::Ship(33)),
    group(20, Side::Japanese, 5, E13A1, SEARCH, 100, Place::Ship(34)),
];

/// The orders each group's standing orders came from, in the order they were
/// accepted (the mission first, then the arc or the share).
const ORDERS: [(u32, &[u32]); 20] = [
    (1, &[1]),
    (2, &[2]),
    (3, &[3, 4]),
    (4, &[5]),
    (5, &[6]),
    (6, &[7]),
    (7, &[8]),
    (8, &[9]),
    (9, &[10]),
    (10, &[11]),
    (11, &[16, 17]),
    // The mission, then the target list (carrier, battleship, cruiser,
    // transport).
    (12, &[18, 19]),
    (13, &[20]),
    (14, &[21, 22]),
    (15, &[23, 24]),
    (16, &[25]),
    (17, &[26, 27]),
    (18, &[28, 29]),
    // The base strike on base 1, then extended reach.
    (19, &[30, 31]),
    (20, &[32, 33]),
];

/// The second table: the aircraft. ID, name part, radius in nm, weapon,
/// damage in millionths, pilot armour, self-sealing tanks.
const AIRCRAFT: [(u32, &str, i64, AttackWeapon, i64, bool, bool); 7] = [
    (F4F4, "F4F-4", 105, AttackWeapon::None, 0, true, true),
    (
        SBD3,
        "SBD-3",
        200,
        AttackWeapon::Bomb,
        75_600_000,
        true,
        true,
    ),
    (
        TBF1,
        "TBF-1",
        190,
        AttackWeapon::Torpedo,
        73_200_000,
        true,
        true,
    ),
    (A6M2, "A6M2", 261, AttackWeapon::None, 0, false, false),
    (
        D3A1,
        "D3A1",
        252,
        AttackWeapon::Bomb,
        41_666_666,
        false,
        false,
    ),
    (
        B5N2,
        "B5N2",
        175,
        AttackWeapon::Torpedo,
        81_640_000,
        false,
        false,
    ),
    (E13A1, "E13A1", 253, AttackWeapon::None, 0, false, false),
];

/// The ships of each force, by the force's ID.
fn ships_of_force(force: u32) -> std::ops::RangeInclusive<u32> {
    match force {
        1 => 1..=10,
        2 => 11..=18,
        3 => 19..=32,
        4 => 33..=36,
        _ => panic!("no such force"),
    }
}

fn force_of(ship: u32) -> u32 {
    (1..=4)
        .find(|force| ships_of_force(*force).contains(&ship))
        .expect("a ship of a force")
}

fn side_of_force(force: u32) -> Side {
    if force <= 2 {
        Side::Allied
    } else {
        Side::Japanese
    }
}

/// Where each force is at the start of the afternoon, 12:00.
fn force_hex(force: u32) -> Hex {
    match force {
        1 | 2 => Hex::new(7, -5),
        3 => Hex::new(8, -11),
        4 => Hex::new(7, -9),
        _ => panic!("no such force"),
    }
}

const HENDERSON: Hex = Hex::new(2, -4);

fn place_hex(place: Place) -> Hex {
    match place {
        Place::Ship(ship) => force_hex(force_of(ship)),
        Place::Base(_) => HENDERSON,
    }
}

fn spec_of(group: u32) -> &'static Spec {
    &GROUPS[usize::try_from(group).expect("small") - 1]
}

/// The target list of a strike group that has one (decision 0063): Shokaku's
/// and Zuikaku's dive bomber groups hold their strike for the heavy ships.
/// Every other group has the default list.
fn target_list(group: u32) -> Option<&'static [ShipKind]> {
    match group {
        12 | 15 => Some(&[
            ShipKind::Carrier,
            ShipKind::Battleship,
            ShipKind::Cruiser,
            ShipKind::Transport,
        ]),
        _ => None,
    }
}

fn orders_of(group: u32) -> Vec<Cause> {
    ORDERS
        .iter()
        .find(|(id, _)| *id == group)
        .expect("a group")
        .1
        .iter()
        .map(|order| Cause::Order(OrderId::new(*order)))
        .collect()
}

fn side_index(side: Side) -> usize {
    match side {
        Side::Allied => 0,
        Side::Japanese => 1,
    }
}

fn nm(hexes: u32) -> i64 {
    40 * i64::from(hexes)
}

/// How far an aircraft flies between two hexes: the function the
/// simulation uses (decision 0040).
fn air(from: Hex, to: Hex) -> i64 {
    i64::from(from.air_distance_nm(to))
}

// ---------------------------------------------------------------------
// Helpers.
// ---------------------------------------------------------------------

fn scenario_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../data/scenarios/05_eastern_solomons.toml")
}

fn load() -> Scenario {
    common::load(&scenario_path()).expect("scenario 5 (Eastern Solomons) loads")
}

fn millionths(value: i64) -> Fixed {
    Fixed::from_raw(value)
}

fn tenths(value: i64) -> Fixed {
    Fixed::from_ratio(value, 10)
}

fn hundredths(value: i64) -> Fixed {
    Fixed::from_ratio(value, 100)
}

fn int(value: u32) -> Fixed {
    Fixed::from_int(i64::from(value))
}

fn count(value: usize) -> i64 {
    i64::try_from(value).expect("small")
}

/// Whether k out of n is within four standard deviations of probability
/// `p` (in millionths), in exact integer arithmetic.
fn within_four_sigma(k: i64, n: i64, p: i64) -> bool {
    let (k, n, p) = (i128::from(k), i128::from(n), i128::from(p));
    let scale = 1_000_000;
    let off = k * scale - n * p;
    off * off <= 16 * n * p * (scale - p)
}

/// The hits a roll of `shots` at `chance` gives from its stream: one draw at
/// the chance for each shot (decision 0033), before any cap at what the
/// target has.
fn hits_from(stream: &mut RngStream, shots: u32, chance: Fixed) -> u32 {
    (0..shots).map(|_| u32::from(stream.chance(chance))).sum()
}

fn stream_of(
    seed: u64,
    pulse: Pulse,
    subsystem: Subsystem,
    entity: EntityRef,
    other: EntityRef,
) -> RngStream {
    RngKey {
        seed,
        day: 1,
        pulse: Some(pulse),
        subsystem,
        entity,
        other: Some(other),
        draw: Draw::Legacy,
    }
    .stream()
}

fn shots_text(shots: u32, chance: Fixed, expected: Fixed, hits: u32) -> String {
    format!(
        "{shots} shots x {chance} = {expected} expected, {hits} {}",
        if hits == 1 { "hit" } else { "hits" }
    )
}

fn scale() -> u64 {
    Fixed::SCALE.unsigned_abs()
}

fn raw_u64(value: Fixed) -> u64 {
    u64::try_from(value.raw()).expect("a chance is not negative")
}

fn event_ids(causes: &[EventId]) -> Vec<Cause> {
    causes.iter().copied().map(Cause::Event).collect()
}

/// The hand calculation's own arithmetic for a search chance: 57.295779 x
/// aircraft x 20 / (arc x distance), the numerator multiplied out first and
/// divided once.
fn search_chance(aircraft: i64, arc: i64, distance: i64) -> Fixed {
    Fixed::from_raw(57_295_779) * aircraft * Fixed::from_int(20) / (Fixed::from_int(distance) * arc)
}

// ---------------------------------------------------------------------
// The loaded reference data the rebuild reads.
// ---------------------------------------------------------------------

struct ShipInfo {
    name: String,
    kind: ShipKind,
    force: u32,
    aa: u32,
    toughness: Fixed,
    fuel: Fixed,
    cruise: Fixed,
    endurance: Fixed,
}

struct Ctx {
    map: Map,
    search: SearchFigures,
    air: AirCombatFigures,
    surface: SurfaceCombatFigures,
    types: BTreeMap<u32, AircraftType>,
    ships: BTreeMap<u32, ShipInfo>,
}

fn ctx() -> &'static Ctx {
    static CTX: OnceLock<Ctx> = OnceLock::new();
    CTX.get_or_init(|| {
        let scenario = load();
        let truth = scenario.world.umpire();
        let data = scenario.world.ref_data();
        let ships = truth
            .ships
            .values()
            .map(|ship| {
                let class = &data.ship_classes[&ship.class];
                let ShipLocation::TaskForce(force) = ship.location else {
                    panic!("every ship starts in a task force");
                };
                (
                    ship.id.raw(),
                    ShipInfo {
                        name: ship.name.clone(),
                        kind: class.kind,
                        force: force.raw(),
                        aa: class.anti_aircraft,
                        toughness: class.toughness,
                        fuel: ship.fuel_t,
                        cruise: class.cruise_speed_kn,
                        endurance: class.endurance_nm,
                    },
                )
            })
            .collect();
        Ctx {
            map: scenario.world.map().clone(),
            search: data.search.clone().expect("search figures"),
            air: data.air_combat.clone().expect("air combat figures"),
            surface: data.surface_combat.clone().expect("surface combat figures"),
            types: data
                .aircraft_types
                .values()
                .map(|t| (t.id.raw(), t.clone()))
                .collect(),
            ships,
        }
    })
}

// ---------------------------------------------------------------------
// What one run did, read from its events and final state.
// ---------------------------------------------------------------------

struct Final {
    flotation: Fixed,
    fire: Fixed,
    sunk: bool,
    systems: [SystemState; 5],
    fuel: Fixed,
}

struct Run {
    seed: u64,
    events: Vec<Event>,
    hashes: Vec<u64>,
    ships: BTreeMap<u32, Final>,
    /// The aircraft ready at the end of each group.
    ready: BTreeMap<u32, u32>,
    positions: BTreeMap<u32, Hex>,
}

fn run_of(base: &Scenario, seed: u64) -> Run {
    let mut run = base.with_seed(seed).expect("a seed can be set");
    run.run_to_end().expect("the run resolves");
    digest(&run, seed)
}

fn digest(run: &Scenario, seed: u64) -> Run {
    let truth = run.world.umpire();
    Run {
        seed,
        // The claims from the air (decision 0056) are not part of what this
        // scenario works out: they come after each raid is resolved and are
        // asserted in scenario_11_claims.rs.
        events: run
            .world
            .events()
            .iter()
            .filter(|e| {
                !matches!(
                    e.kind,
                    sim_core::events::EventKind::ClaimReceived { from_air: true, .. }
                )
            })
            .cloned()
            .collect(),
        hashes: run.world.day_hashes().to_vec(),
        ships: truth
            .ships
            .values()
            .map(|ship| {
                (
                    ship.id.raw(),
                    Final {
                        flotation: ship.flotation_damage,
                        fire: ship.fire_damage,
                        sunk: ship.location == ShipLocation::Sunk,
                        systems: ShipSystem::ALL.map(|system| ship.systems.get(system)),
                        fuel: ship.fuel_t,
                    },
                )
            })
            .collect(),
        ready: truth
            .air_groups
            .values()
            .map(|group| (group.id.raw(), group.ready))
            .collect(),
        positions: truth
            .task_forces
            .values()
            .map(|force| (force.id.raw(), force.position()))
            .collect(),
    }
}

fn batch() -> &'static Vec<Run> {
    static BATCH: OnceLock<Vec<Run>> = OnceLock::new();
    BATCH.get_or_init(|| {
        let base = load();
        (1..=SEEDS).map(|seed| run_of(&base, seed)).collect()
    })
}

/// Every run worked by the rule, asserting every event of the afternoon on
/// the way.
fn worked() -> &'static Vec<Worked> {
    static WORKED: OnceLock<Vec<Worked>> = OnceLock::new();
    WORKED.get_or_init(|| batch().iter().map(|run| work_the_run(run, ctx())).collect())
}

fn afternoon(run: &Run) -> Vec<&Event> {
    run.events
        .iter()
        .filter(|event| event.day == 1 && event.pulse == Some(Pulse::Afternoon))
        .collect()
}

fn morning(run: &Run) -> Vec<&Event> {
    run.events
        .iter()
        .filter(|event| event.day == 1 && event.pulse == Some(Pulse::Morning))
        .collect()
}

fn night(run: &Run) -> Vec<&Event> {
    run.events
        .iter()
        .filter(|event| event.day == 1 && event.pulse == Some(Pulse::Night))
        .collect()
}

// ---------------------------------------------------------------------
// Working the afternoon of a run from the state at its start.
// ---------------------------------------------------------------------

/// A contact a side holds: what its latest report said.
#[derive(Clone, Debug)]
struct Held {
    hex: Hex,
    kind: ShipKind,
    report: EventId,
}

/// A side's belief, as far as the afternoon needs it.
#[derive(Default)]
struct Belief {
    contacts: BTreeMap<u32, Held>,
    allocated: u32,
}

impl Belief {
    /// The contact a sighting at `hex` refreshes, else the next new ID (a
    /// contact's ID is never reused). Every contact in the afternoon is of
    /// age 0, so only the distance matters.
    fn contact_for(&self, cx: &Ctx, hex: Hex) -> (u32, bool) {
        let nearest = self
            .contacts
            .iter()
            .filter(|(_, held)| held.hex.distance(hex) <= cx.search.merge_distance_hexes)
            .min_by_key(|(_, held)| held.hex.distance(hex))
            .map(|(id, _)| *id);
        match nearest {
            Some(id) => (id, false),
            None => (self.allocated + 1, true),
        }
    }

    fn hold(&mut self, id: u32, held: Held) {
        self.allocated = self.allocated.max(id);
        self.contacts.insert(id, held);
    }
}

struct Cursor<'a> {
    events: &'a [&'a Event],
    at: usize,
    seed: u64,
}

impl<'a> Cursor<'a> {
    fn next(&mut self, what: &str) -> &'a Event {
        let event =
            self.events.get(self.at).copied().unwrap_or_else(|| {
                panic!("seed {}: the log ends where {what} should be", self.seed)
            });
        self.at += 1;
        event
    }

    fn peek(&self) -> Option<&'a Event> {
        self.events.get(self.at).copied()
    }
}

/// One launch, as the rule works it.
#[derive(Clone, Debug)]
struct Launch {
    group: u32,
    /// The contact a naval strike was launched at; none for a base strike.
    contact: Option<u32>,
    hex: Hex,
    aircraft: u32,
    distance_nm: i64,
    event: EventId,
    /// The report a naval strike cites; none for a base strike.
    report: Option<EventId>,
    /// The base a base strike was launched at.
    base: Option<u32>,
}

/// One raid, as the rule works it.
#[derive(Clone, Debug)]
struct RaidWork {
    place: Place,
    side: Side,
    hex: Hex,
    /// The strike groups, in group order, and the aircraft each launched.
    strikes: Vec<(u32, u32)>,
    /// The fighter groups that sent escorts, and how many.
    escorts: Vec<(u32, u32)>,
    found: bool,
    target_force: Option<u32>,
    target_ship: Option<u32>,
    /// The base a raid on a base is aimed at.
    target_base: Option<u32>,
    /// The raid's share of the patrol over its hex (decision 0033, 7): what
    /// it meets.
    patrol: u32,
    /// The defending side's whole patrol over the hex.
    patrol_over_hex: u32,
    /// The aircraft of all the raids that attack forces of that side there.
    all_raids_aircraft: u32,
    escort_count: u32,
    /// (shots, hits) of each roll, or `None` for a roll never made.
    escorts_at_patrol: Option<(u32, u32)>,
    patrol_at_escorts: Option<(u32, u32)>,
    patrol_at_bombers: Option<(u32, u32)>,
    guns: Option<(u32, u32)>,
    bombers: u32,
    bombers_attacking: u32,
}

/// One strike group's bombs on a base, as the rule works them.
#[derive(Clone, Debug)]
struct BaseAttackWork {
    group: u32,
    shots: u32,
    hits: u32,
    on_aircraft: u32,
    on_supply: u32,
    /// The aircraft destroyed on the ground by those hits, by group.
    destroyed: Vec<(u32, u32)>,
}

/// One sortie that went beyond its type's radius and rolled for fuel.
#[derive(Clone, Debug)]
struct FuelRoll {
    group: u32,
    came_back: u32,
    chance: Fixed,
    ditched: u32,
}

#[derive(Clone, Debug)]
struct AttackWork {
    group: u32,
    ship: u32,
    shots: u32,
    hits: u32,
    flotation_hits: u32,
    fire_hits: u32,
}

#[derive(Debug)]
struct Worked {
    seed: u64,
    /// Each search attempt: (group, force searched for, sighted).
    attempts: Vec<(u32, u32, bool)>,
    /// The hex each side's last report of the afternoon's search named.
    last_report: [Option<Hex>; 2],
    /// The contacts each side held when its groups decided (contact, hex,
    /// kind the report named).
    held: [Vec<(u32, Hex, ShipKind)>; 2],
    launches: Vec<Launch>,
    raids: Vec<RaidWork>,
    attacks: Vec<AttackWork>,
    base_attacks: Vec<BaseAttackWork>,
    fuel_rolls: Vec<FuelRoll>,
    /// Aircraft lost in the air, by group.
    lost: BTreeMap<u32, u32>,
    /// Ships sunk.
    sunk: Vec<u32>,
}

/// The report a sighting draws from the rest of its stream: position error,
/// then count, then kind (decision 0012).
fn report_from(
    cx: &Ctx,
    stream: &mut RngStream,
    true_hex: Hex,
    true_count: u32,
    true_kind: ShipKind,
) -> (Hex, u32, ShipKind) {
    let direction = if stream.chance(cx.search.position_error_chance) {
        Some(usize::try_from(stream.below(6)).expect("below 6"))
    } else {
        None
    };
    let hex = match direction {
        None => true_hex,
        Some(index) => {
            let (dq, dr) = Hex::DIRECTIONS[index];
            let neighbour = Hex::new(true_hex.q + dq, true_hex.r + dr);
            if cx.map.is_navigable(neighbour) {
                neighbour
            } else {
                true_hex
            }
        }
    };
    let v = stream.below(scale());
    let under = raw_u64(cx.search.count_under_chance);
    let over = raw_u64(cx.search.count_over_chance);
    let count = if v < under {
        true_count.saturating_sub(1).max(1)
    } else if v < under + over {
        true_count + 1
    } else {
        true_count
    };
    let v = stream.below(scale());
    let row = &cx.search.kind_confusion[&true_kind];
    let mut cumulative = 0u64;
    let mut kind = None;
    for candidate in ShipKind::ALL {
        cumulative += row.get(&candidate).map_or(0, |chance| raw_u64(*chance));
        if v < cumulative {
            kind = Some(candidate);
            break;
        }
    }
    (hex, count, kind.expect("a confusion row adds to 1"))
}

/// Takes `hits` from the pool, the lowest group first, each until it has none
/// left. Hits beyond what is there are wasted.
fn take_hits(hits: u32, pool: &mut [(u32, u32)]) -> Vec<(u32, u32)> {
    let mut left = hits;
    let mut taken = Vec::new();
    for (group, aircraft) in pool.iter_mut() {
        let now = left.min(*aircraft);
        if now > 0 {
            *aircraft -= now;
            left -= now;
            taken.push((*group, now));
        }
    }
    taken
}

/// One roll of fighters or guns, worked and not yet recorded.
struct Roll {
    event: EventKind,
    text: String,
    causes: Vec<Cause>,
    factors: Vec<(&'static str, Fixed)>,
    expected: Fixed,
    lost: Vec<(u32, u32)>,
}

/// The chance of one shot at aircraft of `group`'s type: the base times the
/// pilot armour factor if the type has armour, times the tanks factor if it
/// has self-sealing tanks (decision 11 of 0029). Returns the chance and the
/// factors applied.
fn shot_chance(cx: &Ctx, base: Fixed, group: u32) -> (Fixed, Vec<(&'static str, Fixed)>) {
    let ty = &cx.types[&spec_of(group).ty];
    let mut chance = base;
    let mut factors = vec![("hit_chance", base)];
    if ty.pilot_armour {
        chance = chance * cx.air.pilot_armour_factor;
        factors.push(("pilot_armour_factor", cx.air.pilot_armour_factor));
    }
    if ty.self_sealing_tanks {
        chance = chance * cx.air.self_sealing_tanks_factor;
        factors.push((
            "self_sealing_tanks_factor",
            cx.air.self_sealing_tanks_factor,
        ));
    }
    (chance, factors)
}

/// Asserts one recorded roll against the rule.
fn check_roll(seed: u64, event: &Event, roll: &Roll) {
    assert_eq!(event.kind, roll.event, "seed {seed}");
    assert_eq!(event.kind.to_string(), roll.text, "seed {seed}");
    assert_eq!(event.causes, roll.causes, "seed {seed}");
    let breakdown = event.breakdown.as_ref().expect("a breakdown");
    assert_eq!(breakdown.result, roll.expected, "seed {seed}");
    assert_eq!(
        breakdown
            .factors
            .iter()
            .map(|f| (f.name, f.value))
            .collect::<Vec<_>>(),
        roll.factors,
        "seed {seed}"
    );
}

/// What a strike group did, before damage.
enum Outcome {
    Nothing,
    Attack {
        left: u32,
        ship: u32,
        force: u32,
        hits: u32,
        tracks: [u32; 2],
        report: (Hex, u32, ShipKind),
    },
    /// Bombs on a base: the bombs that hit, and where they landed.
    BaseAttack {
        left: u32,
        base: u32,
        hits: u32,
        on_aircraft: u32,
        on_supply: u32,
    },
    /// No bomber left: it records nothing.
    Spent,
}

/// Works the afternoon of `run` by the rule of the hand calculation from the
/// state at its start, and asserts every event of it. The morning has no
/// events but search and movement, and is asserted in its own test.
#[allow(clippy::too_many_lines)]
/// The ships, groups and bases of the scenario as loaded, for the model of
/// where aircraft go when their deck is lost.
#[allow(clippy::type_complexity)]
fn no_deck_facts() -> &'static (
    Vec<common::no_deck::ModelShip>,
    Vec<common::no_deck::ModelGroup>,
    Vec<common::no_deck::ModelBase>,
) {
    static FACTS: OnceLock<(
        Vec<common::no_deck::ModelShip>,
        Vec<common::no_deck::ModelGroup>,
        Vec<common::no_deck::ModelBase>,
    )> = OnceLock::new();
    FACTS.get_or_init(|| common::no_deck::initial(&load().world))
}

/// The aircraft that landed elsewhere to `group` in the run (decisions 7 to
/// 10 of 0059).
fn landed_to(run: &Run, group: u32) -> u32 {
    run.events
        .iter()
        .map(|e| match e.kind {
            EventKind::AircraftLandedElsewhere { to, aircraft, .. } if to.raw() == group => {
                aircraft
            }
            _ => 0,
        })
        .sum()
}

fn work_the_run(run: &Run, cx: &Ctx) -> Worked {
    let seed = run.seed;
    let pulse = Pulse::Afternoon;
    let events = afternoon(run);
    let mut cur = Cursor {
        events: &events,
        at: 0,
        seed,
    };
    let mut work = Worked {
        seed,
        attempts: Vec::new(),
        last_report: [None, None],
        held: [Vec::new(), Vec::new()],
        launches: Vec::new(),
        raids: Vec::new(),
        attacks: Vec::new(),
        base_attacks: Vec::new(),
        fuel_rolls: Vec::new(),
        lost: BTreeMap::new(),
        sunk: Vec::new(),
    };
    let mut beliefs = [Belief::default(), Belief::default()];
    let mut afloat: BTreeSet<u32> = (1..=36).collect();
    let mut now: BTreeMap<u32, u32> = GROUPS.iter().map(|s| (s.id, s.ready)).collect();
    // The events that say each searching group flew (decisions 7 to 10 of
    // 0059 cite them when its aircraft come home).
    let mut flown_ids: BTreeMap<u32, EventId> = BTreeMap::new();

    // ---- The search: each searching group, on its own stream. ----
    // Group, ship, hex, arc, range, aircraft, and each attempt's force,
    // distance and chance: the afternoon search table.
    type Attempts = &'static [(u32, i64, i64)];
    type Search = (u32, u32, Hex, (u32, u32), i64, u32, Attempts);
    let searches: [Search; 3] = [
        (
            3,
            1,
            Hex::new(7, -5),
            (290, 90),
            200,
            16,
            &[(4, 160, 716_197)],
        ),
        (
            17,
            21,
            Hex::new(8, -11),
            (110, 200),
            253,
            5,
            &[(1, 223, 285_479), (2, 223, 285_479)],
        ),
        (
            20,
            34,
            Hex::new(7, -9),
            (110, 200),
            253,
            5,
            &[(1, 160, 397_887), (2, 160, 397_887)],
        ),
    ];
    let force_counts = [0u32, 10, 8, 14, 4];
    for (group, ship, hex, arc, range, aircraft, attempts) in searches {
        let flown = cur.next("a search flown");
        assert_eq!(
            flown.kind,
            EventKind::SearchFlown {
                air_group: AirGroupId::new(group),
                from: AirGroupLocation::Ship(ShipId::new(ship)),
                hex,
                from_deg: arc.0,
                to_deg: arc.1,
                range_nm: Fixed::from_int(range),
                aircraft,
            },
            "seed {seed}"
        );
        assert_eq!(flown.causes, orders_of(group), "seed {seed}");
        flown_ids.insert(group, flown.id);
        let side = spec_of(group).side;
        for &(force, distance, chance) in attempts {
            let attempt = cur.next("a search attempt");
            let mut stream = stream_of(
                seed,
                pulse,
                Subsystem::Search,
                EntityRef::AirGroup(AirGroupId::new(group)),
                EntityRef::TaskForce(TaskForceId::new(force)),
            );
            let sighted = stream.chance(millionths(chance));
            assert_eq!(
                attempt.kind,
                EventKind::SearchAttempt {
                    air_group: AirGroupId::new(group),
                    task_force: TaskForceId::new(force),
                    distance_nm: Fixed::from_int(distance),
                    chance: millionths(chance),
                    sighted,
                },
                "seed {seed}"
            );
            assert_eq!(attempt.causes, vec![Cause::Event(flown.id)], "seed {seed}");
            let breakdown = attempt.breakdown.as_ref().expect("a breakdown");
            assert_eq!(breakdown.result, millionths(chance));
            assert_eq!(
                breakdown
                    .factors
                    .iter()
                    .map(|f| (f.name, f.value))
                    .collect::<Vec<_>>(),
                vec![
                    ("degrees_per_radian", Fixed::from_raw(57_295_779)),
                    ("aircraft", Fixed::from_int(i64::from(aircraft))),
                    ("sweep_width_nm", Fixed::from_int(20)),
                    (
                        "arc_deg",
                        Fixed::from_int(i64::from(bearing::arc_degrees(arc.0, arc.1)))
                    ),
                    ("distance_nm", Fixed::from_int(distance)),
                ]
            );
            work.attempts.push((group, force, sighted));
            if !sighted {
                continue;
            }
            let (hex, ship_count, kind) = report_from(
                cx,
                &mut stream,
                force_hex(force),
                force_counts[usize::try_from(force).expect("small")],
                ShipKind::Carrier,
            );
            let belief = &mut beliefs[side_index(side)];
            let (contact, created) = belief.contact_for(cx, hex);
            let report = cur.next("a report");
            assert_eq!(
                report.kind,
                EventKind::ReportReceived {
                    side,
                    contact: ContactId::new(contact),
                    hex,
                    ship_count,
                    kind,
                    created,
                },
                "seed {seed}"
            );
            assert_eq!(report.causes, vec![Cause::Event(attempt.id)]);
            belief.hold(
                contact,
                Held {
                    hex,
                    kind,
                    report: report.id,
                },
            );
            work.last_report[side_index(side)] = Some(hex);
        }
    }

    // ---- The decisions: each strike group on its own side's belief, and each
    // base strike group on the reach of its own order. ----
    for (slot, belief) in beliefs.iter().enumerate() {
        work.held[slot] = belief
            .contacts
            .iter()
            .map(|(id, held)| (*id, held.hex, held.kind))
            .collect();
    }
    for spec in &GROUPS {
        let ty = &cx.types[&spec.ty];
        if ty.attack_weapon == AttackWeapon::None || spec.ready == 0 {
            continue;
        }
        let origin = place_hex(spec.place);
        if let AirMission::BaseStrike(base) = spec.mission {
            // A base strike launches in the first daylight pulse of the day
            // in which its base is within its reach (decision 7 of 0063). The
            // group is on extended reach: the radius times 1.25, cut to a
            // whole mile (decision 2 of 0059). Henderson Field is 312 nm from
            // Ryujo at 06:00, so the morning launches nothing (asserted in
            // its own test), and 200 nm from her at noon.
            let reach = cx.air.reach_nm(ty.strike_range_nm, Reach::Extended);
            assert_eq!(reach, Fixed::from_int(218), "175 x 1.25 = 218.75");
            let distance = air(origin, HENDERSON);
            assert_eq!(distance, 200);
            assert!(Fixed::from_int(distance) <= reach);
            let event = cur.next("a base strike launched");
            assert_eq!(
                event.kind,
                EventKind::BaseStrikeLaunched {
                    air_group: AirGroupId::new(spec.id),
                    base,
                    hex: HENDERSON,
                    aircraft: spec.ready,
                    distance_nm: Fixed::from_int(distance),
                },
                "seed {seed}"
            );
            assert_eq!(
                event.kind.to_string(),
                format!(
                    "air_group:{} launched {} aircraft at base:{} at {HENDERSON}, {distance} nm",
                    spec.id,
                    spec.ready,
                    base.raw()
                )
            );
            // Its causes are its orders: the mission, then the reach.
            assert_eq!(event.causes, orders_of(spec.id), "seed {seed}");
            work.launches.push(Launch {
                group: spec.id,
                contact: None,
                hex: HENDERSON,
                aircraft: spec.ready,
                distance_nm: distance,
                event: event.id,
                report: None,
                base: Some(base.raw()),
            });
            continue;
        }
        if spec.mission != STRIKE {
            continue;
        }
        // The target list of groups 12 and 15 (carrier, battleship, cruiser,
        // transport):
        // a contact of a kind not on it is never launched at. The default is
        // every kind in the order of importance.
        let list = target_list(spec.id);
        let rank = |kind: ShipKind| -> Option<usize> {
            match list {
                Some(list) => list.iter().position(|k| *k == kind),
                None => ShipKind::ALL.iter().position(|k| *k == kind),
            }
        };
        let chosen = beliefs[side_index(spec.side)]
            .contacts
            .iter()
            .filter_map(|(id, held)| {
                let distance = air(origin, held.hex);
                (Fixed::from_int(distance) <= ty.strike_range_nm)
                    .then(|| rank(held.kind))
                    .flatten()
                    .map(|rank| (rank, distance, *id, held.hex, held.report))
            })
            .min_by_key(|(rank, distance, id, _, _)| (*rank, *distance, *id));
        let Some((_, distance, contact, hex, report)) = chosen else {
            continue;
        };
        let event = cur.next("a launch");
        assert_eq!(
            event.kind,
            EventKind::StrikeLaunched {
                air_group: AirGroupId::new(spec.id),
                contact: ContactId::new(contact),
                hex,
                aircraft: spec.ready,
                distance_nm: Fixed::from_int(distance),
            },
            "seed {seed}"
        );
        assert_eq!(
            event.kind.to_string(),
            format!(
                "air_group:{} launched {} aircraft at contact:{contact} at {hex}, {distance} nm",
                spec.id, spec.ready
            )
        );
        // Its causes: the mission order, the target order if the list is not
        // the default, and the report (decision 0063).
        let mut causes = if list.is_some() {
            orders_of(spec.id)
        } else {
            vec![orders_of(spec.id)[0]]
        };
        causes.push(Cause::Event(report));
        assert_eq!(event.causes, causes, "seed {seed}");
        work.launches.push(Launch {
            group: spec.id,
            contact: Some(contact),
            hex,
            aircraft: spec.ready,
            distance_nm: distance,
            event: event.id,
            report: Some(report),
            base: None,
        });
    }

    // ---- Raids: the strikes of one place at one hex. ----
    struct Raid {
        place: Place,
        hex: Hex,
        side: Side,
        /// The base a raid on a base is aimed at.
        base: Option<u32>,
        strikes: Vec<usize>,
        escorts: Vec<usize>,
    }
    let mut raids: Vec<Raid> = Vec::new();
    for (index, launch) in work.launches.iter().enumerate() {
        let spec = spec_of(launch.group);
        match raids.iter_mut().find(|raid| {
            raid.place == spec.place && raid.hex == launch.hex && raid.base == launch.base
        }) {
            Some(raid) => raid.strikes.push(index),
            None => raids.push(Raid {
                place: spec.place,
                hex: launch.hex,
                side: spec.side,
                base: launch.base,
                strikes: vec![index],
                escorts: Vec::new(),
            }),
        }
    }
    // The fighters, each split into patrol and escorts.
    struct Fighter {
        group: u32,
        side: Side,
        place: Place,
        hex: Hex,
        patrol: u32,
        escorts: u32,
        radius: Fixed,
    }
    let mut fighters: Vec<Fighter> = GROUPS
        .iter()
        .filter(|s| s.mission == PATROL && s.ready > 0)
        .map(|s| {
            let patrol = (int(s.ready) * hundredths(s.share)).trunc();
            let patrol = u32::try_from(patrol).expect("small");
            Fighter {
                group: s.id,
                side: s.side,
                place: s.place,
                hex: place_hex(s.place),
                patrol,
                escorts: s.ready - patrol,
                radius: cx.types[&s.ty].strike_range_nm,
            }
        })
        .collect();
    #[allow(clippy::needless_range_loop)]
    for index in 0..fighters.len() {
        if fighters[index].escorts == 0 {
            continue;
        }
        let chosen = raids
            .iter()
            .enumerate()
            .filter(|(_, raid)| raid.place == fighters[index].place)
            .max_by_key(|(_, raid)| {
                let bombers: u32 = raid
                    .strikes
                    .iter()
                    .map(|&i| work.launches[i].aircraft)
                    .sum();
                (bombers, Reverse(work.launches[raid.strikes[0]].group))
            })
            .map(|(raid_index, _)| raid_index);
        if let Some(raid_index) = chosen {
            let distance = air(fighters[index].hex, raids[raid_index].hex);
            if Fixed::from_int(distance) <= fighters[index].radius {
                raids[raid_index].escorts.push(index);
            } else {
                fighters[index].patrol += fighters[index].escorts;
                fighters[index].escorts = 0;
            }
        }
    }

    // ---- The air combat step, raid by raid, on the start-of-step counts. ----
    // The task force a raid finds: the one with the most important ship,
    // then the lowest number.
    let find_target = |raid: &Raid| {
        (1..=4u32)
            .filter(|&force| side_of_force(force) != raid.side && force_hex(force) == raid.hex)
            .filter_map(|force| {
                ships_of_force(force)
                    .filter(|ship| afloat.contains(ship))
                    .map(|ship| (cx.ships[&ship].kind, ship))
                    .min()
                    .map(|(kind, ship)| (kind, force, ship))
            })
            .min()
    };
    // The side a raid attacks and what it is aimed at: a raid on a base
    // attacks the base's side (Henderson Field is Allied), and a raid on ships
    // the force `find_target` gives.
    let aim_of = |raid: &Raid| -> Option<(Side, Option<(u32, u32)>)> {
        if raid.base.is_some() {
            return Some((Side::Allied, None));
        }
        find_target(raid).map(|(_, force, ship)| (side_of_force(force), Some((force, ship))))
    };
    // Each raid's share of the patrol over its hex, from the counts at the
    // start of the step, before any raid is worked (decision 0033, 7): the
    // patrol times the raid's aircraft over the aircraft of all the raids
    // that attack forces of that side in that hex, rounded down, and the
    // fighters left over one each to the largest raids, the lower first
    // group first. Written out here on its own, not by the rule's function.
    // (patrol over the hex, raid aircraft, all raids' aircraft, share)
    let mut meets: Vec<(u32, u32, u32, u32)> = vec![(0, 0, 0, 0); raids.len()];
    {
        let aircraft_of = |raid: &Raid| -> u32 {
            let bombers: u32 = raid
                .strikes
                .iter()
                .map(|&i| work.launches[i].aircraft)
                .sum();
            let escorts: u32 = raid.escorts.iter().map(|&i| fighters[i].escorts).sum();
            bombers + escorts
        };
        let mut hexes: Vec<(Side, Hex)> = Vec::new();
        for raid in &raids {
            if let Some((defending, _)) = aim_of(raid) {
                let key = (defending, raid.hex);
                if !hexes.contains(&key) {
                    hexes.push(key);
                }
            }
        }
        for (defending, hex) in hexes {
            let members: Vec<usize> = (0..raids.len())
                .filter(|&i| {
                    aim_of(&raids[i]).is_some_and(|(side, _)| side == defending)
                        && raids[i].hex == hex
                })
                .collect();
            let patrol: u32 = fighters
                .iter()
                .filter(|f| f.side == defending && f.hex == hex)
                .map(|f| f.patrol)
                .sum();
            let all: u32 = members.iter().map(|&i| aircraft_of(&raids[i])).sum();
            let mut shares: Vec<u32> = members
                .iter()
                .map(|&i| patrol * aircraft_of(&raids[i]) / all)
                .collect();
            let mut left = patrol - shares.iter().sum::<u32>();
            let mut by_size: Vec<usize> = (0..members.len()).collect();
            by_size.sort_by_key(|&m| {
                (
                    Reverse(aircraft_of(&raids[members[m]])),
                    work.launches[raids[members[m]].strikes[0]].group,
                )
            });
            for m in by_size {
                if left == 0 {
                    break;
                }
                shares[m] += 1;
                left -= 1;
            }
            for (m, &i) in members.iter().enumerate() {
                meets[i] = (patrol, aircraft_of(&raids[i]), all, shares[m]);
            }
        }
    }
    // The patrol of each side over each hex, from which the escorts' hits of
    // all the raids are taken in turn, the lowest group first.
    type Pool = (Side, Hex, Vec<(u32, u32)>);
    let mut patrol_pools: Vec<Pool> = Vec::new();
    for f in fighters.iter().filter(|f| f.patrol > 0) {
        match patrol_pools
            .iter_mut()
            .find(|(side, hex, _)| *side == f.side && *hex == f.hex)
        {
            Some((_, _, pool)) => pool.push((f.group, f.patrol)),
            None => patrol_pools.push((f.side, f.hex, vec![(f.group, f.patrol)])),
        }
    }
    let mut outcomes: BTreeMap<u32, Outcome> = BTreeMap::new();
    let mut loss_ids: BTreeMap<u32, Vec<EventId>> = BTreeMap::new();
    for (raid_index, raid) in raids.iter().enumerate() {
        let first = work.launches[raid.strikes[0]].clone();
        let mut raid_work = RaidWork {
            place: raid.place,
            side: raid.side,
            hex: raid.hex,
            strikes: raid
                .strikes
                .iter()
                .map(|&i| (work.launches[i].group, work.launches[i].aircraft))
                .collect(),
            escorts: raid
                .escorts
                .iter()
                .map(|&i| (fighters[i].group, fighters[i].escorts))
                .collect(),
            found: false,
            target_force: None,
            target_ship: None,
            target_base: raid.base,
            patrol: 0,
            patrol_over_hex: 0,
            all_raids_aircraft: 0,
            escort_count: 0,
            escorts_at_patrol: None,
            patrol_at_escorts: None,
            patrol_at_bombers: None,
            guns: None,
            bombers: raid_work_bombers(&work, &raid.strikes),
            bombers_attacking: 0,
        };
        // The escorts go with the raid, whether or not it finds anything.
        let mut escort_events: Vec<EventId> = Vec::new();
        for &index in &raid.escorts {
            let fighter = &fighters[index];
            let event = cur.next("escorts sent");
            assert_eq!(
                event.kind,
                EventKind::EscortsSent {
                    air_group: AirGroupId::new(fighter.group),
                    with_air_group: AirGroupId::new(first.group),
                    aircraft: fighter.escorts,
                },
                "seed {seed}"
            );
            assert_eq!(
                event.kind.to_string(),
                format!(
                    "air_group:{} sent {} escorts with air_group:{}",
                    fighter.group, fighter.escorts, first.group
                )
            );
            let mut causes = orders_of(fighter.group);
            causes.push(Cause::Event(first.event));
            assert_eq!(event.causes, causes, "seed {seed}");
            escort_events.push(event.id);
        }
        let Some((defending, ships)) = aim_of(raid) else {
            for &(group, _) in &raid_work.strikes {
                outcomes.insert(group, Outcome::Nothing);
            }
            work.raids.push(raid_work);
            continue;
        };
        raid_work.found = true;
        raid_work.target_force = ships.map(|(force, _)| force);
        raid_work.target_ship = ships.map(|(_, ship)| ship);
        // What the streams of the raid are keyed on: the task force attacked,
        // or the base.
        let target_ref = match (raid.base, ships) {
            (Some(base), _) => EntityRef::Base(BaseId::new(base)),
            (None, Some((force, _))) => EntityRef::TaskForce(TaskForceId::new(force)),
            (None, None) => unreachable!("a raid is aimed at something"),
        };
        let (force, ship) = ships.unwrap_or((0, 0));

        // The pools the hits are taken from, each in group order.
        let mut bombers: Vec<(u32, u32)> = raid_work.strikes.clone();
        let mut escorts: Vec<(u32, u32)> = raid_work
            .escorts
            .iter()
            .copied()
            .filter(|(_, aircraft)| *aircraft > 0)
            .collect();
        let (whole, raid_aircraft, all_aircraft, share) = meets[raid_index];
        // A raid with no share meets no patrol.
        let patrol: Vec<(u32, u32)> = if share == 0 {
            Vec::new()
        } else {
            fighters
                .iter()
                .filter(|f| f.side == defending && f.hex == raid.hex && f.patrol > 0)
                .map(|f| (f.group, f.patrol))
                .collect()
        };
        let escort_count: u32 = escorts.iter().map(|(_, n)| n).sum();
        let patrol_count: u32 = share;
        raid_work.patrol = share;
        raid_work.patrol_over_hex = whole;
        raid_work.all_raids_aircraft = all_aircraft;
        let lead = vec![
            ("patrol", int(whole)),
            ("raid_aircraft", int(raid_aircraft)),
            ("all_raids_aircraft", int(all_aircraft)),
            ("patrol_share", int(share)),
        ];
        raid_work.escort_count = escort_count;

        let mut rolls: Vec<Roll> = Vec::new();
        let escort_causes: Vec<Cause> = event_ids(&escort_events);
        let fight = |firer: u32,
                     target: u32,
                     firing: AirFirer,
                     at: AirTarget,
                     shots: u32,
                     chance: Fixed,
                     lead: Vec<(&'static str, Fixed)>,
                     factors: Vec<(&'static str, Fixed)>,
                     hits: u32,
                     lost: Vec<(u32, u32)>|
         -> Roll {
            let expected = int(shots) * chance;
            let who = match firing {
                AirFirer::Escorts => "escorts",
                AirFirer::Patrol => "patrol",
            };
            let what = match at {
                AirTarget::Patrol => "patrol",
                AirTarget::Escorts => "escorts",
                AirTarget::Bombers => "bombers",
            };
            let mut causes = vec![Cause::Event(first.event)];
            if firing == AirFirer::Escorts || at == AirTarget::Escorts {
                causes.extend(escort_causes.iter().cloned());
            }
            let mut all = lead;
            all.push(("shots", int(shots)));
            all.extend(factors);
            Roll {
                event: EventKind::AirFight {
                    firer: AirGroupId::new(firer),
                    target: AirGroupId::new(target),
                    firing,
                    at,
                    shots: int(shots),
                    chance,
                    expected,
                    hits,
                },
                text: format!(
                    "air_group:{firer} {who} at the {what} of air_group:{target}: {}",
                    shots_text(shots, chance, expected, hits)
                ),
                causes,
                factors: all,
                expected,
                lost,
            }
        };
        if let (Some(&(first_escort, _)), Some(&(first_patrol, _))) =
            (escorts.first(), patrol.first())
        {
            let mut stream = stream_of(
                seed,
                pulse,
                Subsystem::AirCombat,
                EntityRef::AirGroup(AirGroupId::new(first_escort)),
                target_ref,
            );
            let (chance, factors) = shot_chance(cx, cx.air.fighter_hit_chance, first_patrol);
            let hits = hits_from(&mut stream, escort_count, chance);
            raid_work.escorts_at_patrol = Some((escort_count, hits));
            rolls.push(fight(
                first_escort,
                first_patrol,
                AirFirer::Escorts,
                AirTarget::Patrol,
                escort_count,
                chance,
                Vec::new(),
                factors,
                hits,
                // Taken from the whole patrol over the hex, over all the
                // raids (decision 0033, 10).
                {
                    let pool = patrol_pools
                        .iter_mut()
                        .find(|(side, hex, _)| *side == defending && *hex == raid.hex)
                        .map(|(_, _, pool)| pool)
                        .expect("the patrol is in its pool");
                    take_hits(hits, pool)
                },
            ));
        }
        if let Some(&(first_patrol, _)) = patrol.first() {
            let mut stream = stream_of(
                seed,
                pulse,
                Subsystem::AirCombat,
                EntityRef::AirGroup(AirGroupId::new(first_patrol)),
                EntityRef::AirGroup(AirGroupId::new(first.group)),
            );
            let at_escorts = patrol_count.min(escort_count);
            if let Some(&(first_escort, _)) = escorts.first() {
                let (chance, factors) = shot_chance(cx, cx.air.fighter_hit_chance, first_escort);
                let hits = hits_from(&mut stream, at_escorts, chance);
                raid_work.patrol_at_escorts = Some((at_escorts, hits));
                let lost = take_hits(hits, &mut escorts);
                rolls.push(fight(
                    first_patrol,
                    first_escort,
                    AirFirer::Patrol,
                    AirTarget::Escorts,
                    at_escorts,
                    chance,
                    lead.clone(),
                    factors,
                    hits,
                    lost,
                ));
            }
            let at_bombers = patrol_count - at_escorts;
            if at_bombers > 0 {
                let (chance, factors) = shot_chance(cx, cx.air.fighter_hit_chance, first.group);
                let hits = hits_from(&mut stream, at_bombers, chance);
                raid_work.patrol_at_bombers = Some((at_bombers, hits));
                let lost = take_hits(hits, &mut bombers);
                rolls.push(fight(
                    first_patrol,
                    first.group,
                    AirFirer::Patrol,
                    AirTarget::Bombers,
                    at_bombers,
                    chance,
                    lead.clone(),
                    factors,
                    hits,
                    lost,
                ));
            }
        }
        // The guns of every ship afloat in the force, at the bombers left. The
        // base's guns are none in this scenario (asserted with the forces).
        let gun_shots: u32 = if ships.is_some() {
            ships_of_force(force)
                .filter(|ship| afloat.contains(ship))
                .map(|ship| cx.ships[&ship].aa)
                .sum()
        } else {
            0
        };
        let bombers_left: u32 = bombers.iter().map(|(_, n)| n).sum();
        let lowest = if ships.is_some() {
            ships_of_force(force).find(|ship| afloat.contains(ship))
        } else {
            None
        };
        if let (true, true, Some(lowest)) = (gun_shots > 0, bombers_left > 0, lowest) {
            let mut stream = stream_of(
                seed,
                pulse,
                Subsystem::AirCombat,
                EntityRef::Ship(ShipId::new(lowest)),
                EntityRef::AirGroup(AirGroupId::new(first.group)),
            );
            let (chance, factors) = shot_chance(cx, cx.air.anti_aircraft_hit_chance, first.group);
            let hits = hits_from(&mut stream, gun_shots, chance);
            let expected = int(gun_shots) * chance;
            raid_work.guns = Some((gun_shots, hits));
            let mut all = vec![("shots", int(gun_shots))];
            all.extend(factors);
            rolls.push(Roll {
                event: EventKind::AntiAircraftFire {
                    task_force: TaskForceId::new(force),
                    air_group: AirGroupId::new(first.group),
                    shots: int(gun_shots),
                    chance,
                    expected,
                    hits,
                },
                text: format!(
                    "task_force:{force} guns at the raid of air_group:{}: {}",
                    first.group,
                    shots_text(gun_shots, chance, expected, hits)
                ),
                causes: vec![Cause::Event(first.event)],
                factors: all,
                expected,
                lost: take_hits(hits, &mut bombers),
            });
        }
        // Record the rolls, then the losses they cost.
        let mut recorded: Vec<EventId> = Vec::new();
        for roll in &rolls {
            let event = cur.next("a roll");
            check_roll(seed, event, roll);
            recorded.push(event.id);
        }
        for (roll, cause) in rolls.iter().zip(recorded) {
            for &(group, taken) in &roll.lost {
                let lost = taken.min(now[&group]);
                if lost == 0 {
                    continue;
                }
                let remaining = now[&group] - lost;
                now.insert(group, remaining);
                let event = cur.next("a loss");
                assert_eq!(
                    event.kind,
                    EventKind::AircraftLost {
                        air_group: AirGroupId::new(group),
                        aircraft: lost,
                        remaining,
                    },
                    "seed {seed}"
                );
                assert_eq!(
                    event.kind.to_string(),
                    format!("air_group:{group} lost {lost} aircraft, {remaining} left")
                );
                assert_eq!(event.causes, vec![Cause::Event(cause)], "seed {seed}");
                loss_ids.entry(group).or_default().push(event.id);
                *work.lost.entry(group).or_default() += lost;
            }
        }
        // What each strike group has left to attack with.
        raid_work.bombers_attacking = bombers.iter().map(|(_, n)| n).sum();
        if let Some(base) = raid.base {
            // A raid on a base: one draw for each bomb at the bomb hit chance
            // whatever the type's weapon, then one for each hit, below the
            // chance of landing on aircraft, then below that and the chance of
            // landing on supply, else on nothing (decisions 6 and 7 of 0034).
            for &(group, left) in &bombers {
                if left == 0 {
                    outcomes.insert(group, Outcome::Spent);
                    continue;
                }
                let mut stream = stream_of(
                    seed,
                    pulse,
                    Subsystem::AirCombat,
                    EntityRef::AirGroup(AirGroupId::new(group)),
                    target_ref,
                );
                let hits = hits_from(&mut stream, left, cx.air.bomb_hit_chance);
                let on_aircraft_below = raw_u64(cx.air.base_hit_aircraft);
                let on_supply_below = on_aircraft_below + raw_u64(cx.air.base_hit_supply);
                let (mut on_aircraft, mut on_supply) = (0, 0);
                for _ in 0..hits {
                    let draw = stream.below(scale());
                    if draw < on_aircraft_below {
                        on_aircraft += 1;
                    } else if draw < on_supply_below {
                        on_supply += 1;
                    }
                }
                outcomes.insert(
                    group,
                    Outcome::BaseAttack {
                        left,
                        base,
                        hits,
                        on_aircraft,
                        on_supply,
                    },
                );
            }
            work.raids.push(raid_work);
            continue;
        }
        let ship_count =
            u32::try_from(ships_of_force(force).filter(|s| afloat.contains(s)).count())
                .expect("few ships");
        for (&(group, left), launch_index) in bombers.iter().zip(&raid.strikes) {
            if left == 0 {
                outcomes.insert(group, Outcome::Spent);
                continue;
            }
            let launch = &work.launches[*launch_index];
            let ty = &cx.types[&spec_of(group).ty];
            let weapon_chance = match ty.attack_weapon {
                AttackWeapon::Bomb => cx.air.bomb_hit_chance,
                AttackWeapon::Torpedo => cx.air.torpedo_hit_chance,
                AttackWeapon::None => unreachable!(),
            };
            let mut stream = stream_of(
                seed,
                pulse,
                Subsystem::AirCombat,
                EntityRef::AirGroup(AirGroupId::new(group)),
                target_ref,
            );
            let hits = hits_from(&mut stream, left, weapon_chance);
            let mut tracks = [0u32; 2];
            for _ in 0..hits {
                let track = if ty.attack_weapon == AttackWeapon::Bomb {
                    let draw = stream.below(scale());
                    usize::from(draw >= raw_u64(cx.surface.gun_hit_flotation))
                } else {
                    0
                };
                tracks[track] += 1;
                // One draw for the system, always.
                let _ = stream.below(scale());
            }
            let report = report_from(cx, &mut stream, launch.hex, ship_count, ShipKind::Carrier);
            outcomes.insert(
                group,
                Outcome::Attack {
                    left,
                    ship,
                    force,
                    hits,
                    tracks,
                    report,
                },
            );
        }
        work.raids.push(raid_work);
    }
    assert!(
        cur.peek().is_none_or(|e| !matches!(
            e.kind,
            EventKind::EscortsSent { .. }
                | EventKind::AirFight { .. }
                | EventKind::AntiAircraftFire { .. }
                | EventKind::AircraftLost { .. }
        )),
        "seed {seed}: nothing else in the air"
    );

    // ---- What each strike found, in group order. ----
    let mut finding: BTreeMap<u32, EventId> = BTreeMap::new();
    for launch in &work.launches {
        match &outcomes[&launch.group] {
            Outcome::Spent => {
                finding.insert(launch.group, launch.event);
            }
            Outcome::Nothing => {
                let event = cur.next("a strike that found nothing");
                assert_eq!(
                    event.kind,
                    EventKind::StrikeFoundNothing {
                        air_group: AirGroupId::new(launch.group),
                        hex: launch.hex,
                    },
                    "seed {seed}"
                );
                assert_eq!(
                    event.kind.to_string(),
                    format!("air_group:{} found nothing at {}", launch.group, launch.hex)
                );
                assert_eq!(event.causes, vec![Cause::Event(launch.event)]);
                finding.insert(launch.group, event.id);
            }
            Outcome::Attack {
                left,
                ship,
                force,
                hits,
                ..
            } => {
                let ty = &cx.types[&spec_of(launch.group).ty];
                // A dive bomber's bombs and a torpedo plane's torpedoes, each
                // at its own hit chance.
                let (weapon, verb, chance) = match ty.attack_weapon {
                    AttackWeapon::Bomb => (Weapon::Bomb, "bombs", cx.air.bomb_hit_chance),
                    AttackWeapon::Torpedo => {
                        (Weapon::Torpedo, "torpedoes", cx.air.torpedo_hit_chance)
                    }
                    AttackWeapon::None => unreachable!(),
                };
                let expected = int(*left) * chance;
                let event = cur.next("an attack");
                assert_eq!(
                    event.kind,
                    EventKind::AirAttack {
                        air_group: AirGroupId::new(launch.group),
                        task_force: TaskForceId::new(*force),
                        ship: ShipId::new(*ship),
                        weapon,
                        shots: int(*left),
                        chance,
                        expected,
                        hits: *hits,
                    },
                    "seed {seed}"
                );
                assert_eq!(
                    event.kind.to_string(),
                    format!(
                        "air_group:{} {verb} at ship:{ship}: {}",
                        launch.group,
                        shots_text(*left, chance, expected, *hits)
                    )
                );
                let mut causes = vec![Cause::Event(launch.event)];
                causes.extend(event_ids(
                    loss_ids
                        .get(&launch.group)
                        .map_or(&[][..], |v| v.as_slice()),
                ));
                assert_eq!(event.causes, causes, "seed {seed}");
                let breakdown = event.breakdown.as_ref().expect("a breakdown");
                assert_eq!(breakdown.result, expected);
                assert_eq!(
                    breakdown
                        .factors
                        .iter()
                        .map(|f| (f.name, f.value))
                        .collect::<Vec<_>>(),
                    vec![("aircraft", int(*left)), ("hit_chance", chance)]
                );
                finding.insert(launch.group, event.id);
            }
            Outcome::BaseAttack {
                left,
                base,
                hits,
                on_aircraft,
                on_supply,
            } => {
                let chance = cx.air.bomb_hit_chance;
                let expected = int(*left) * chance;
                let event = cur.next("an attack on a base");
                assert_eq!(
                    event.kind,
                    EventKind::BaseAttacked {
                        air_group: AirGroupId::new(launch.group),
                        base: BaseId::new(*base),
                        shots: int(*left),
                        chance,
                        expected,
                        hits: *hits,
                        hits_on_aircraft: *on_aircraft,
                        hits_on_supply: *on_supply,
                    },
                    "seed {seed}"
                );
                assert_eq!(
                    event.kind.to_string(),
                    format!(
                        "air_group:{} bombs at base:{base}: {}: {on_aircraft} on aircraft, \
                         {on_supply} on supply, {} on nothing",
                        launch.group,
                        shots_text(*left, chance, expected, *hits),
                        hits - on_aircraft - on_supply
                    )
                );
                let mut causes = vec![Cause::Event(launch.event)];
                causes.extend(event_ids(
                    loss_ids
                        .get(&launch.group)
                        .map_or(&[][..], |v| v.as_slice()),
                ));
                assert_eq!(event.causes, causes, "seed {seed}");
                let breakdown = event.breakdown.as_ref().expect("a breakdown");
                assert_eq!(breakdown.result, expected);
                assert_eq!(
                    breakdown
                        .factors
                        .iter()
                        .map(|f| (f.name, f.value))
                        .collect::<Vec<_>>(),
                    vec![
                        ("aircraft", int(*left)),
                        ("hit_chance", chance),
                        ("hit_on_aircraft_chance", cx.air.base_hit_aircraft),
                        ("hit_on_supply_chance", cx.air.base_hit_supply),
                    ]
                );
                finding.insert(launch.group, event.id);
            }
        }
    }

    // ---- Damage lands and the reports go out, in group order. ----
    let mut flotation: BTreeMap<u32, Fixed> = BTreeMap::new();
    let mut systems: BTreeMap<(u32, ShipSystem), SystemState> = BTreeMap::new();
    let mut fire: BTreeMap<u32, Fixed> = BTreeMap::new();
    // The aircraft each group has in the air at the end of the air combat
    // step (decision 7 of 0059): what it flew in the pulse, a search, a
    // strike, its patrol and the escorts that went, less what it lost.
    let mut in_air: BTreeMap<u32, u32> = BTreeMap::new();
    for (group, _, _, _, _, aircraft, _) in searches {
        *in_air.entry(group).or_default() += aircraft;
    }
    for launch in &work.launches {
        *in_air.entry(launch.group).or_default() += launch.aircraft;
    }
    for (index, f) in fighters.iter().enumerate() {
        let escorts_went = raids.iter().any(|raid| raid.escorts.contains(&index));
        *in_air.entry(f.group).or_default() += f.patrol + if escorts_went { f.escorts } else { 0 };
    }
    for (group, aircraft) in in_air.iter_mut() {
        *aircraft -= spec_of(*group).ready - now[group];
    }
    let mut deck_losses: Vec<common::no_deck::DeckLoss> = Vec::new();
    for launch in &work.launches {
        let side = spec_of(launch.group).side;
        let own = side_index(side);
        match &outcomes[&launch.group] {
            Outcome::Spent => {}
            Outcome::BaseAttack {
                left,
                base,
                hits,
                on_aircraft,
                on_supply,
            } => {
                // The hits on aircraft destroy aircraft on the ground, the
                // lowest group first, no more than were there at the start
                // of the step: for each group at the base, the ready aircraft
                // that are not flying in the pulse (decision 8 of 0034). A
                // group's patrol and its launched strike are flying; the
                // others rest. Supply is destroyed up to what the base has,
                // and Henderson Field has none here (decision 9 of 0034).
                let attack_id = finding[&launch.group];
                let mut ground: Vec<(u32, u32)> = Vec::new();
                for spec in GROUPS.iter().filter(|s| s.place == Place::Base(*base)) {
                    let flying = match spec.mission {
                        AirMission::CombatAirPatrol => {
                            fighters.iter().find(|f| f.group == spec.id).map_or(0, |f| {
                                f.patrol
                                    + if raids.iter().any(|raid| {
                                        raid.escorts.iter().any(|&i| fighters[i].group == spec.id)
                                    }) {
                                        f.escorts
                                    } else {
                                        0
                                    }
                            })
                        }
                        AirMission::NavalStrike => work
                            .launches
                            .iter()
                            .find(|l| l.group == spec.id)
                            .map_or(0, |l| l.aircraft),
                        _ => 0,
                    };
                    let resting = spec.ready.saturating_sub(flying);
                    if resting > 0 {
                        ground.push((spec.id, resting));
                    }
                }
                let mut destroyed_by_group: Vec<(u32, u32)> = Vec::new();
                let mut hits_left = *on_aircraft;
                for (group, on_the_ground) in ground {
                    let destroyed = hits_left.min(on_the_ground);
                    if destroyed == 0 {
                        continue;
                    }
                    hits_left -= destroyed;
                    let remaining = now[&group] - destroyed;
                    now.insert(group, remaining);
                    let event = cur.next("aircraft destroyed on the ground");
                    assert_eq!(
                        event.kind,
                        EventKind::AircraftDestroyedOnGround {
                            air_group: AirGroupId::new(group),
                            base: BaseId::new(*base),
                            aircraft: destroyed,
                            remaining,
                        },
                        "seed {seed}"
                    );
                    assert_eq!(event.causes, vec![Cause::Event(attack_id)], "seed {seed}");
                    destroyed_by_group.push((group, destroyed));
                }
                assert!(on_supply + on_aircraft <= *hits, "seed {seed}");
                work.base_attacks.push(BaseAttackWork {
                    group: launch.group,
                    shots: *left,
                    hits: *hits,
                    on_aircraft: *on_aircraft,
                    on_supply: *on_supply,
                    destroyed: destroyed_by_group,
                });
            }
            Outcome::Nothing => {
                let contact = launch.contact.expect("a naval strike has a contact");
                let names = beliefs[own]
                    .contacts
                    .get(&contact)
                    .is_some_and(|held| held.hex == launch.hex);
                if names {
                    beliefs[own].contacts.remove(&contact);
                    let event = cur.next("a contact dropped");
                    assert_eq!(
                        event.kind,
                        EventKind::ContactNotThere {
                            side,
                            contact: ContactId::new(contact),
                            hex: launch.hex,
                        },
                        "seed {seed}"
                    );
                    assert_eq!(event.causes, vec![Cause::Event(finding[&launch.group])]);
                }
            }
            Outcome::Attack {
                left,
                ship,
                force,
                hits,
                tracks,
                report,
            } => {
                let attack_id = finding[&launch.group];
                let info = &cx.ships[ship];
                let damage = cx.types[&spec_of(launch.group).ty].attack_damage;
                let per_hit = Fixed::from_int(100) * damage / info.toughness;
                let mut total = 0;
                for (slot, track) in DamageTrack::ALL.into_iter().enumerate() {
                    if tracks[slot] == 0 {
                        continue;
                    }
                    let values = match track {
                        DamageTrack::Flotation => &mut flotation,
                        DamageTrack::Fire => &mut fire,
                    };
                    let before = values.get(ship).copied().unwrap_or(Fixed::ZERO);
                    let points = per_hit * i64::from(tracks[slot]);
                    let after = (before + points).min(Fixed::from_int(100));
                    values.insert(*ship, after);
                    let event = cur.next("damage");
                    assert_eq!(
                        event.kind,
                        EventKind::ShipDamaged {
                            round: 0,
                            ship: ShipId::new(*ship),
                            track,
                            hits: tracks[slot],
                            points,
                            value_after: after,
                        },
                        "seed {seed}"
                    );
                    assert_eq!(event.causes, vec![Cause::Event(attack_id)]);
                    total += tracks[slot];
                    if track == DamageTrack::Flotation
                        && before < Fixed::from_int(100)
                        && after >= Fixed::from_int(100)
                    {
                        let sunk = cur.next("a sinking");
                        assert_eq!(
                            sunk.kind,
                            EventKind::ShipSunk {
                                round: 0,
                                ship: ShipId::new(*ship),
                                task_force: TaskForceId::new(*force),
                            },
                            "seed {seed}"
                        );
                        assert_eq!(sunk.causes, vec![Cause::Event(event.id)]);
                        afloat.remove(ship);
                        work.sunk.push(*ship);
                        deck_losses.push(common::no_deck::DeckLoss {
                            ship: *ship,
                            hex: force_hex(*force),
                            cause: sunk.id,
                        });
                        for spec in GROUPS.iter().filter(|s| s.place == Place::Ship(*ship)) {
                            // Decision 10 of 0059: what was aboard goes down
                            // with her; the aircraft in the air are kept and
                            // come home below.
                            let kept = in_air.get(&spec.id).copied().unwrap_or(0);
                            let aboard = now[&spec.id] - kept;
                            let lost = cur.next("an air group lost with its ship");
                            assert_eq!(
                                lost.kind,
                                EventKind::AirGroupLost {
                                    air_group: AirGroupId::new(spec.id),
                                    ship: ShipId::new(*ship),
                                    aircraft: aboard,
                                },
                                "seed {seed}"
                            );
                            assert_eq!(
                                lost.kind.to_string(),
                                format!(
                                    "air_group:{} lost with ship:{ship}: {aboard} aircraft",
                                    spec.id
                                )
                            );
                            assert_eq!(lost.causes, vec![Cause::Event(sunk.id)]);
                            now.insert(spec.id, kept);
                        }
                    }
                }
                assert_eq!(total, *hits, "seed {seed}: every hit is on a track");
                // The systems hit, in hit order.
                let mut system_hits = 0;
                while let Some(event) = cur.peek() {
                    let EventKind::SystemHit {
                        round,
                        ship: hit,
                        system,
                        before,
                        after,
                    } = event.kind
                    else {
                        break;
                    };
                    cur.next("a system hit");
                    assert_eq!(event.causes, vec![Cause::Event(attack_id)]);
                    assert_eq!(round, 0);
                    assert_eq!(hit, ShipId::new(*ship));
                    let held = systems
                        .get(&(*ship, system))
                        .copied()
                        .unwrap_or(SystemState::Working);
                    assert_eq!(before, held, "seed {seed}");
                    assert!(after == before || after == before.step_down());
                    // A deck that goes out closes the deck to the aircraft
                    // in the air (a ship already sunk has none).
                    if system == ShipSystem::FlightDeck
                        && after == SystemState::Out
                        && before != SystemState::Out
                        && afloat.contains(ship)
                    {
                        deck_losses.push(common::no_deck::DeckLoss {
                            ship: *ship,
                            hex: force_hex(*force),
                            cause: event.id,
                        });
                    }
                    systems.insert((*ship, system), after);
                    system_hits += 1;
                }
                assert!(system_hits <= *hits, "seed {seed}");
                // The report, citing the attack.
                let belief = &mut beliefs[own];
                let (contact, created) = belief.contact_for(cx, report.0);
                let received = cur.next("a report");
                assert_eq!(
                    received.kind,
                    EventKind::ReportReceived {
                        side,
                        contact: ContactId::new(contact),
                        hex: report.0,
                        ship_count: report.1,
                        kind: report.2,
                        created,
                    },
                    "seed {seed}"
                );
                assert_eq!(received.causes, vec![Cause::Event(attack_id)]);
                belief.hold(
                    contact,
                    Held {
                        hex: report.0,
                        kind: report.2,
                        report: received.id,
                    },
                );
                work.attacks.push(AttackWork {
                    group: launch.group,
                    ship: *ship,
                    shots: *left,
                    hits: *hits,
                    flotation_hits: tracks[0],
                    fire_hits: tracks[1],
                });
            }
        }
    }

    // ---- Fuel (decisions 3 to 5 of 0059): each strike or escort sortie whose
    // one-way distance d was beyond its type's radius R rolls each aircraft
    // that came back, on its own, at the ditch figure times (d - R) over R. A
    // sortie within the radius rolls nothing: every escort here flies within
    // its radius, and so does every strike but group 19's, which flies 200 nm
    // on a radius of 175. The roll is from a stream of the group alone, after
    // the air combat step and before the aircraft whose deck is lost come
    // home. No pilots are counted in this scenario. ----
    for raid in &raids {
        for &index in &raid.escorts {
            let distance = air(fighters[index].hex, raid.hex);
            assert!(
                Fixed::from_int(distance) <= fighters[index].radius,
                "seed {seed}: no escort of this scenario flies beyond its radius"
            );
        }
    }
    for launch in &work.launches {
        let spec = spec_of(launch.group);
        let radius = cx.types[&spec.ty].strike_range_nm;
        let distance = Fixed::from_int(launch.distance_nm);
        if distance <= radius {
            continue;
        }
        assert_eq!(launch.group, 19, "seed {seed}: only group 19 flies beyond");
        let came_back = launch.aircraft - work.lost.get(&launch.group).copied().unwrap_or(0);
        if came_back == 0 {
            continue;
        }
        // 200 nm on a radius of 175: 25 beyond, a share of 25 / 175 =
        // 0.142857, and the ditch figure 2 times it, 0.285714.
        let share = Fixed::from_int(25).mul_div(Fixed::ONE, Fixed::from_int(175));
        assert_eq!(share, millionths(142_857));
        assert_eq!(cx.air.ditch_chance_per_radius_beyond, Fixed::from_int(2));
        let chance = cx.air.ditch_chance_per_radius_beyond * share;
        assert_eq!(chance, millionths(285_714));
        let mut stream = RngKey {
            seed,
            day: 1,
            pulse: Some(pulse),
            subsystem: Subsystem::Upkeep,
            entity: EntityRef::AirGroup(AirGroupId::new(launch.group)),
            other: None,
            draw: Draw::Ditch,
        }
        .stream();
        let ditched = hits_from(&mut stream, came_back, chance);
        let expected = int(came_back) * chance;
        let ready = now[&launch.group] - ditched;
        let event = cur.next("aircraft ditched for fuel");
        assert_eq!(
            event.kind,
            EventKind::AircraftDitched {
                air_group: AirGroupId::new(launch.group),
                sortie: SortieKind::BaseStrike,
                distance_nm: distance,
                radius_nm: radius,
                aircraft: came_back,
                chance,
                expected,
                ditched,
                crews_recovered: None,
                pilots_lost: 0,
                ready,
            },
            "seed {seed}"
        );
        assert_eq!(
            event.causes,
            vec![Cause::Event(launch.event)],
            "seed {seed}"
        );
        let breakdown = event.breakdown.as_ref().expect("a breakdown");
        assert_eq!(breakdown.result, expected);
        assert_eq!(
            breakdown
                .factors
                .iter()
                .map(|f| (f.name, f.value))
                .collect::<Vec<_>>(),
            vec![
                ("aircraft", int(came_back)),
                ("distance_nm", distance),
                ("radius_nm", radius),
                ("share_beyond_radius", share),
                (
                    "ditch_chance_per_radius_beyond",
                    cx.air.ditch_chance_per_radius_beyond
                ),
                ("chance", chance),
                (
                    "crew_recovered_short_of_home",
                    cx.air.crew_recovered_short_of_home
                ),
            ]
        );
        // A ditched aircraft is gone from the group and is no longer in the
        // air when the deck is lost.
        now.insert(launch.group, ready);
        if let Some(aircraft) = in_air.get_mut(&launch.group) {
            *aircraft -= ditched;
        }
        work.fuel_rolls.push(FuelRoll {
            group: launch.group,
            came_back,
            chance,
            ditched,
        });
    }

    // ---- The aircraft whose deck is lost come home (decisions 7 to 10 of
    // 0059): to another carrier of their side in the hex with room, else to a
    // base within the type's radius, else the sea. ----
    {
        let (mut ships, mut model_groups, bases) = no_deck_facts().clone();
        for ship in &mut ships {
            // The forces have moved since the scenario was loaded.
            ship.hex = force_hex(force_of(ship.id));
            ship.afloat = afloat.contains(&ship.id);
            ship.deck_out = systems
                .get(&(ship.id, ShipSystem::FlightDeck))
                .copied()
                .unwrap_or(SystemState::Working)
                == SystemState::Out;
        }
        for group in &mut model_groups {
            group.ready = now[&group.id];
            group.in_air = in_air.get(&group.id).copied().unwrap_or(0);
        }
        let expected = common::no_deck::homecoming(&ships, &mut model_groups, &bases, &deck_losses);
        for (home, closing) in &expected {
            let event = cur.next("a homecoming");
            let source = home.source();
            // The events that say the group flew, in the order its sorties
            // are noted: a search, a strike, or a patrol and its escorts.
            let mut causes: Vec<Cause> = closing.map(Cause::Event).into_iter().collect();
            let mut flew: Vec<Cause> = Vec::new();
            if let Some(id) = flown_ids.get(&source) {
                flew.push(Cause::Event(*id));
            }
            if let Some(launch) = work.launches.iter().find(|l| l.group == source) {
                flew.push(Cause::Event(launch.event));
            }
            if let Some((index, f)) = fighters.iter().enumerate().find(|(_, f)| f.group == source) {
                if f.patrol > 0 {
                    flew.extend(orders_of(source));
                }
                if raids.iter().any(|raid| raid.escorts.contains(&index)) {
                    let sent = events
                        .iter()
                        .find(|e| {
                            matches!(e.kind, EventKind::EscortsSent { air_group, .. }
                                if air_group == AirGroupId::new(source))
                        })
                        .expect("escorts sent");
                    flew.push(Cause::Event(sent.id));
                }
            }
            for cause in flew {
                if !causes.contains(&cause) {
                    causes.push(cause);
                }
            }
            let ready_of = |group: u32| {
                model_groups
                    .iter()
                    .find(|g| g.id == group)
                    .map(|g| g.ready)
                    .expect("the group")
            };
            let kind = match *home {
                common::no_deck::Home::Landed {
                    from,
                    to,
                    at,
                    aircraft,
                    formed,
                } => EventKind::AircraftLandedElsewhere {
                    from: AirGroupId::new(from),
                    to: AirGroupId::new(to),
                    at,
                    aircraft,
                    pilots: 0,
                    formed,
                },
                common::no_deck::Home::NoDeck {
                    group,
                    hex,
                    aircraft,
                    beside_own_force,
                } => EventKind::AircraftDitchedNoDeck {
                    air_group: AirGroupId::new(group),
                    hex,
                    aircraft,
                    beside_own_force,
                    // The frozen figures: both crew figures are 1.
                    chance: Fixed::ONE,
                    crews_recovered: None,
                    pilots_lost: 0,
                    ready: ready_of(group),
                },
            };
            assert_eq!(event.kind, kind, "seed {seed}");
            assert_eq!(event.causes, causes, "seed {seed}");
        }
        for group in &model_groups {
            now.insert(group.id, group.ready);
        }
    }

    // ---- What is left is movement. ----
    while let Some(event) = cur.peek() {
        assert!(
            matches!(
                event.kind,
                EventKind::TaskForceSteamed { .. }
                    | EventKind::FuelBurned { .. }
                    | EventKind::TaskForceArrived { .. }
            ),
            "seed {seed}: unexpected {}",
            event.kind
        );
        cur.next("movement");
    }

    // ---- The end: the aircraft and damage the events leave. ----
    let ready_now: BTreeMap<u32, u32> = now;
    assert_eq!(run.ready, ready_now, "seed {seed}");
    for (id, last) in &run.ships {
        assert_eq!(
            (last.flotation, last.fire),
            (
                flotation.get(id).copied().unwrap_or(Fixed::ZERO),
                fire.get(id).copied().unwrap_or(Fixed::ZERO)
            ),
            "seed {seed}, ship {id}"
        );
        assert_eq!(last.sunk, !afloat.contains(id), "seed {seed}");
        for (slot, system) in ShipSystem::ALL.into_iter().enumerate() {
            assert_eq!(
                last.systems[slot],
                systems
                    .get(&(*id, system))
                    .copied()
                    .unwrap_or(SystemState::Working),
                "seed {seed}, ship {id}"
            );
        }
    }
    work
}

fn raid_work_bombers(work: &Worked, strikes: &[usize]) -> u32 {
    strikes.iter().map(|&i| work.launches[i].aircraft).sum()
}

// ---------------------------------------------------------------------
// The setting: the forces, the groups, the aircraft.
// ---------------------------------------------------------------------

/// The first table's ships: ID, a part of the name, the kind.
const SHIPS: [(u32, &str, ShipKind); 36] = [
    (1, "Enterprise", ShipKind::Carrier),
    (2, "North Carolina", ShipKind::Battleship),
    (3, "Portland", ShipKind::Cruiser),
    (4, "Atlanta", ShipKind::Cruiser),
    (5, "Balch", ShipKind::Destroyer),
    (6, "Benham", ShipKind::Destroyer),
    (7, "Ellet", ShipKind::Destroyer),
    (8, "Maury", ShipKind::Destroyer),
    (9, "Grayson", ShipKind::Destroyer),
    (10, "Monssen", ShipKind::Destroyer),
    (11, "Saratoga", ShipKind::Carrier),
    (12, "New Orleans", ShipKind::Cruiser),
    (13, "Minneapolis", ShipKind::Cruiser),
    (14, "Farragut", ShipKind::Destroyer),
    (15, "Macdonough", ShipKind::Destroyer),
    (16, "Worden", ShipKind::Destroyer),
    (17, "Dale", ShipKind::Destroyer),
    (18, "Phelps", ShipKind::Destroyer),
    (19, "Shokaku", ShipKind::Carrier),
    (20, "Zuikaku", ShipKind::Carrier),
    (21, "Chikuma", ShipKind::Cruiser),
    (22, "Hiei", ShipKind::Battleship),
    (23, "Kirishima", ShipKind::Battleship),
    (24, "Kumano", ShipKind::Cruiser),
    (25, "Suzuya", ShipKind::Cruiser),
    (26, "Nagara", ShipKind::Cruiser),
    (27, "Akigumo", ShipKind::Destroyer),
    (28, "Kazagumo", ShipKind::Destroyer),
    (29, "Makigumo", ShipKind::Destroyer),
    (30, "Yugumo", ShipKind::Destroyer),
    (31, "Akizuki", ShipKind::Destroyer),
    (32, "Hatsukaze", ShipKind::Destroyer),
    (33, "Ryujo", ShipKind::Carrier),
    (34, "Tone", ShipKind::Cruiser),
    (35, "Amatsukaze", ShipKind::Destroyer),
    (36, "Tokitsukaze", ShipKind::Destroyer),
];

#[test]
fn the_forces_and_air_groups_are_as_the_first_two_tables_say() {
    let cx = ctx();
    let scenario = load();
    assert_eq!(scenario.days, 1);
    let truth = scenario.world.umpire();
    // Four task forces: the first table's names and sides.
    assert_eq!(truth.task_forces.len(), 4);
    for (id, name, side) in [
        (1, "Task Force 16", Side::Allied),
        (2, "Task Force 11", Side::Allied),
        (3, "striking force", Side::Japanese),
        (4, "Ryujo", Side::Japanese),
    ] {
        let force = &truth.task_forces[&TaskForceId::new(id)];
        assert!(force.name.contains(name), "{}", force.name);
        assert_eq!(force.side, side);
        assert_eq!(
            force.ships.iter().map(|s| s.raw()).collect::<Vec<_>>(),
            ships_of_force(id).collect::<Vec<_>>()
        );
    }
    // 36 ships, by ID, with the names, kinds and forces of the table.
    assert_eq!(truth.ships.len(), 36);
    for (id, name, kind) in SHIPS {
        let info = &cx.ships[&id];
        assert!(info.name.contains(name), "ship {id} is {}", info.name);
        assert_eq!(info.kind, kind, "ship {id}");
        assert_eq!(info.force, force_of(id), "ship {id}");
        assert_eq!(
            truth.ships[&ShipId::new(id)].side,
            side_of_force(force_of(id))
        );
    }
    // Both American forces start in (7,-5), the Japanese in (9,-19) and
    // (7,-18); Henderson Field is base 1, in (2,-4).
    for (id, hex) in [
        (1, Hex::new(7, -5)),
        (2, Hex::new(7, -5)),
        (3, Hex::new(9, -19)),
        (4, Hex::new(7, -18)),
    ] {
        assert_eq!(truth.task_forces[&TaskForceId::new(id)].position(), hex);
    }
    assert_eq!(
        truth.bases[&sim_core::ids::BaseId::new(1)].hex,
        Hex::new(2, -4)
    );
    assert_eq!(truth.bases.len(), 1);
    // Henderson Field has no guns here, so a raid on it meets no gun fire.
    assert_eq!(truth.bases[&BaseId::new(1)].anti_aircraft, 0);
    // Twenty air groups: counts, types, places.
    assert_eq!(truth.air_groups.len(), 20);
    for spec in &GROUPS {
        let air = &truth.air_groups[&AirGroupId::new(spec.id)];
        assert_eq!(air.ready, spec.ready, "group {}", spec.id);
        assert_eq!(air.aircraft_type.raw(), spec.ty, "group {}", spec.id);
        assert_eq!(air.side, spec.side, "group {}", spec.id);
        assert_eq!(
            air.location,
            match spec.place {
                Place::Ship(ship) => AirGroupLocation::Ship(ShipId::new(ship)),
                Place::Base(base) => AirGroupLocation::Base(sim_core::ids::BaseId::new(base)),
            },
            "group {}",
            spec.id
        );
        assert_eq!(
            air.side,
            match spec.place {
                Place::Ship(ship) => side_of_force(force_of(ship)),
                Place::Base(_) => Side::Allied,
            }
        );
    }
    // The table's totals by force.
    let aboard = |ships: &[u32]| -> u32 {
        GROUPS
            .iter()
            .filter(|s| matches!(s.place, Place::Ship(ship) if ships.contains(&ship)))
            .map(|s| s.ready)
            .sum()
    };
    assert_eq!(aboard(&[1]), 27 + 17 + 16 + 14);
    assert_eq!(aboard(&[11]), 27 + 15 + 15 + 13);
    assert_eq!(aboard(&[19, 20, 21]), 26 + 27 + 18 + 25 + 27 + 18 + 5);
    assert_eq!(aboard(&[33, 34]), 23 + 6 + 5);

    // Missions, shares and arcs, as the orders leave them.
    let mut ran = load();
    ran.run_to_end().expect("runs");
    let truth = ran.world.umpire();
    for spec in &GROUPS {
        let air = &truth.air_groups[&AirGroupId::new(spec.id)];
        assert_eq!(air.mission, spec.mission, "group {}", spec.id);
        if spec.mission == PATROL {
            assert_eq!(
                air.patrol_share,
                hundredths(spec.share),
                "group {}",
                spec.id
            );
        }
        // The target list (decision 0063): groups 12 and 15 hold their strike
        // for carrier, battleship, cruiser and transport; every other group has the
        // default, all six kinds in the order of importance. And the reach
        // (decision 0059): Ryujo's attack unit is on extended reach, the
        // others on normal.
        assert_eq!(
            air.target_priority,
            target_list(spec.id).map_or_else(|| ShipKind::ALL.to_vec(), <[ShipKind]>::to_vec),
            "group {}",
            spec.id
        );
        assert_eq!(
            air.reach,
            if spec.id == 19 {
                Reach::Extended
            } else {
                Reach::Normal
            },
            "group {}",
            spec.id
        );
        let arc = air.search.map(|a| (a.from_deg, a.to_deg, a.range_nm));
        match spec.id {
            3 => assert_eq!(arc, Some((290, 90, Fixed::from_int(200)))),
            17 | 20 => assert_eq!(arc, Some((110, 200, Fixed::from_int(253)))),
            _ => assert_eq!(arc, None, "group {}", spec.id),
        }
    }
    // Patrol shares: the Japanese carriers keep 0.8, Ryujo 0.35, the American
    // fighters everything.
    for id in [1, 5, 9] {
        assert_eq!(
            truth.air_groups[&AirGroupId::new(id)].patrol_share,
            Fixed::ONE
        );
    }
}

#[test]
fn the_orders_are_the_ones_the_raids_cite() {
    for run in batch() {
        let mut seen: BTreeMap<EntityRef, Vec<u32>> = BTreeMap::new();
        for event in &run.events {
            if let EventKind::OrderAccepted { order, target } = event.kind {
                assert_eq!(event.causes, vec![Cause::Order(order)]);
                seen.entry(target).or_default().push(order.raw());
            }
        }
        for (group, orders) in ORDERS {
            assert_eq!(
                seen[&EntityRef::AirGroup(AirGroupId::new(group))],
                orders,
                "seed {}, group {group}",
                run.seed
            );
        }
        assert_eq!(seen[&EntityRef::TaskForce(TaskForceId::new(3))], [12, 13]);
        assert_eq!(seen[&EntityRef::TaskForce(TaskForceId::new(4))], [14, 15]);
    }
}

#[test]
fn the_aircraft_table_is_read_from_the_reference_data() {
    let cx = ctx();
    for (id, name, radius, weapon, damage, armour, tanks) in AIRCRAFT {
        let ty = &cx.types[&id];
        assert!(ty.name.contains(name), "{}", ty.name);
        assert_eq!(ty.strike_range_nm, Fixed::from_int(radius), "{name}");
        assert_eq!(ty.attack_weapon, weapon, "{name}");
        assert_eq!(ty.attack_damage, millionths(damage), "{name}");
        assert_eq!(ty.pilot_armour, armour, "{name}");
        assert_eq!(ty.self_sealing_tanks, tanks, "{name}");
    }
    // The floatplane's radius is its search range too.
    assert_eq!(cx.types[&E13A1].search_range_nm, Fixed::from_int(253));
    // The factors the hand calculation uses.
    assert_eq!(cx.air.fighter_hit_chance, tenths(3));
    assert_eq!(cx.air.anti_aircraft_hit_chance, hundredths(5));
    assert_eq!(cx.air.bomb_hit_chance, tenths(2));
    assert_eq!(cx.air.pilot_armour_factor, hundredths(75));
    assert_eq!(cx.air.self_sealing_tanks_factor, hundredths(75));
    assert_eq!(cx.air.strike_contact_max_age_pulses, 0);
    // A shot at an SBD-3 or an F4F-4: both factors. At an A6M2 or a D3A1:
    // none. A fighter's 0.3 x 0.75 x 0.75 = 0.16875, a gun's 0.05 x 0.75 x
    // 0.75 = 0.028125.
    for group in [1, 2, 5, 6, 7] {
        assert_eq!(
            shot_chance(cx, cx.air.fighter_hit_chance, group).0,
            millionths(168_750)
        );
        assert_eq!(
            shot_chance(cx, cx.air.anti_aircraft_hit_chance, group).0,
            millionths(28_125)
        );
    }
    for group in [11, 12, 14, 15, 18] {
        assert_eq!(
            shot_chance(cx, cx.air.fighter_hit_chance, group).0,
            tenths(3)
        );
        assert_eq!(
            shot_chance(cx, cx.air.anti_aircraft_hit_chance, group).0,
            hundredths(5)
        );
    }
    // The anti-aircraft shots: 123 for task force 1, 26 for task force 4.
    let aa = |ships: &[u32]| -> Vec<u32> { ships.iter().map(|s| cx.ships[s].aa).collect() };
    assert_eq!(
        aa(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10]),
        [22, 34, 15, 20, 10, 5, 5, 4, 4, 4]
    );
    assert_eq!(
        aa(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10]).iter().sum::<u32>(),
        123
    );
    assert_eq!(aa(&[33, 34, 35, 36]), [13, 11, 1, 1]);
    assert_eq!(aa(&[33, 34, 35, 36]).iter().sum::<u32>(), 26);
    // A hit does 100 x damage / toughness: 17.830188 on Ryujo, 5.260942 on
    // Enterprise.
    assert_eq!(cx.ships[&33].toughness, Fixed::from_int(424));
    assert_eq!(cx.ships[&1].toughness, Fixed::from_int(792));
    assert_eq!(
        Fixed::from_int(100) * cx.types[&SBD3].attack_damage / cx.ships[&33].toughness,
        millionths(17_830_188)
    );
    assert_eq!(
        Fixed::from_int(100) * cx.types[&D3A1].attack_damage / cx.ships[&1].toughness,
        millionths(5_260_942)
    );
    // How many keep station and how many escort: 26 x 0.8 = 20.8, 25 x 0.8,
    // 23 x 0.35 = 8.05, all rounded down.
    for (ready, share, keep) in [(26, 80, 20), (25, 80, 20), (23, 35, 8)] {
        assert_eq!((int(ready) * hundredths(share)).trunc(), keep);
    }
}

// ---------------------------------------------------------------------
// Where everyone is.
// ---------------------------------------------------------------------

/// The steaming of one force in a pulse: from, to, distance, speed.
fn steamed(events: &[&Event], force: u32) -> Option<(Hex, Hex, Fixed, Fixed)> {
    let mut found = None;
    for event in events {
        if let EventKind::TaskForceSteamed {
            task_force,
            from,
            to,
            distance_nm,
            speed_kn,
            ..
        } = event.kind
        {
            if task_force == TaskForceId::new(force) {
                assert!(found.is_none(), "one steaming a pulse");
                found = Some((from, to, distance_nm, speed_kn));
            }
        }
    }
    found
}

#[test]
fn positions_distances_and_arrivals_are_as_the_tables_say() {
    let start3 = Hex::new(9, -19);
    let start4 = Hex::new(7, -18);
    let dest3 = Hex::new(8, -11);
    let dest4 = Hex::new(7, -9);
    // Eight hexes, charged the straight line, 302 nm (22 and seven 40s);
    // nine hexes down an axis, 360 nm (decision 0058).
    assert_eq!(start3.distance(dest3), 8);
    assert_eq!(air(start3, dest3), 302);
    assert_eq!(
        air(start3, dest3) - air(Hex::new(8, -18), dest3),
        22,
        "the first step"
    );
    assert_eq!(air(Hex::new(8, -18), dest3), 7 * 40);
    assert_eq!(start4.distance(dest4), 9);
    assert_eq!(air(start4, dest4), 360);
    assert_eq!(nm(start4.distance(dest4)), 360);
    let american = Hex::new(7, -5);
    // 06:00: (8,-13) and (7,-12).
    let (a3, a4) = (Hex::new(8, -13), Hex::new(7, -12));
    // Air distance (hex count times 40 in brackets): 302 (320), 280 (280),
    // 280 (320): Henderson Field is dq 5, dr -8, 25 + 64 - 40 = 49, 40 x 7.
    assert_eq!(air(american, a3), 302);
    assert_eq!(nm(american.distance(a3)), 320);
    assert_eq!(american.distance(a3), 8);
    assert_eq!(air(american, a4), 280);
    assert_eq!(nm(american.distance(a4)), 280);
    assert_eq!(american.distance(a4), 7);
    assert_eq!(air(HENDERSON, a4), 280);
    assert_eq!(nm(HENDERSON.distance(a4)), 320);
    assert_eq!(HENDERSON.distance(a4), 8);
    // 12:00: (8,-11) and (7,-9).
    // Air distance: 223 (240), 160 (160), 200 (200): Henderson Field is dq 5,
    // dr -5, 25 + 25 - 25 = 25, 40 x 5.
    assert_eq!(air(american, dest3), 223);
    assert_eq!(nm(american.distance(dest3)), 240);
    assert_eq!(american.distance(dest3), 6);
    assert_eq!(air(american, dest4), 160);
    assert_eq!(nm(american.distance(dest4)), 160);
    assert_eq!(american.distance(dest4), 4);
    assert_eq!(air(HENDERSON, dest4), 200);
    assert_eq!(nm(HENDERSON.distance(dest4)), 200);
    assert_eq!(HENDERSON.distance(dest4), 5);

    for run in batch() {
        let night = night(run);
        let morning = morning(run);
        let afternoon = afternoon(run);
        let seed = run.seed;
        // The striking force: 204 nm in the night to (8,-13), 22 nm past the
        // centre of (8,-14) (at 182 nm) on a step of 40, then 98 nm to
        // (8,-11): 302 nm in all, at 17 knots, and nothing in the afternoon.
        let n3 = steamed(&night, 3).expect("steamed");
        let m3 = steamed(&morning, 3).expect("steamed");
        assert_eq!(n3, (start3, a3, Fixed::from_int(204), Fixed::from_int(17)));
        assert_eq!(m3, (a3, dest3, Fixed::from_int(98), Fixed::from_int(17)));
        assert!(steamed(&afternoon, 3).is_none(), "seed {seed}");
        assert_eq!(204 + 98, 302, "seed {seed}");
        assert_eq!(22 + 4 * 40, 182, "the centre of (8,-14)");
        assert_eq!(204 - 182, 22, "22 nm past it");
        // The Ryujo detachment: 240 nm to (7,-12), at its centre; 120 nm to
        // (7,-9), 360 nm; at 20 knots; nothing in the afternoon.
        let n4 = steamed(&night, 4).expect("steamed");
        let m4 = steamed(&morning, 4).expect("steamed");
        assert_eq!(n4, (start4, a4, Fixed::from_int(240), Fixed::from_int(20)));
        assert_eq!(m4, (a4, dest4, Fixed::from_int(120), Fixed::from_int(20)));
        assert_eq!(nm(start4.distance(a4)), 240, "at its centre");
        assert_eq!(240 + 120, 360);
        assert!(steamed(&afternoon, 4).is_none(), "seed {seed}");
        // Arrivals: the striking force at hour 17.764705 (12 + 98 / 17), the
        // Ryujo detachment at 12:00, hour 18, both in the morning pulse.
        let arrivals: Vec<(TaskForceId, Hex, Fixed, Pulse)> = run
            .events
            .iter()
            .filter_map(|e| match e.kind {
                EventKind::TaskForceArrived {
                    task_force,
                    hex,
                    elapsed_hours,
                } => Some((task_force, hex, elapsed_hours, e.pulse.expect("a pulse"))),
                _ => None,
            })
            .collect();
        assert_eq!(
            arrivals,
            vec![
                (
                    TaskForceId::new(3),
                    dest3,
                    millionths(17_764_705),
                    Pulse::Morning
                ),
                (
                    TaskForceId::new(4),
                    dest4,
                    Fixed::from_int(18),
                    Pulse::Morning
                ),
            ],
            "seed {seed}"
        );
        assert_eq!(
            Fixed::from_int(12) + Fixed::from_int(98) / Fixed::from_int(17),
            millionths(17_764_705)
        );
        assert_eq!(time::pulse_start(1, Pulse::Afternoon), Fixed::from_int(18));
        // The searches fly from where the forces are: (7,-5) all day; the
        // floatplanes from (8,-13) and (7,-12) at 06:00 and from (8,-11) and
        // (7,-9) at 12:00.
        for (events, expected) in [
            (&morning, [(3, american), (17, a3), (20, a4)]),
            (&afternoon, [(3, american), (17, dest3), (20, dest4)]),
        ] {
            let flown: Vec<(u32, Hex)> = events
                .iter()
                .filter_map(|e| match e.kind {
                    EventKind::SearchFlown { air_group, hex, .. } => Some((air_group.raw(), hex)),
                    _ => None,
                })
                .collect();
            assert_eq!(flown, expected, "seed {seed}");
        }
        // The American forces never move; the end positions.
        assert!(run.events.iter().all(
            |e| !matches!(e.kind, EventKind::TaskForceSteamed { task_force, .. }
                if task_force.raw() <= 2)
        ));
        assert_eq!(run.positions[&1], american);
        assert_eq!(run.positions[&2], american);
        assert_eq!(run.positions[&3], dest3);
        assert_eq!(run.positions[&4], dest4);
    }
}

// ---------------------------------------------------------------------
// Fuel.
// ---------------------------------------------------------------------

#[test]
fn the_fuel_figures_are_exact() {
    let cx = ctx();
    // Shokaku cruises at 18 knots, so at 17 she burns at her cruise rate,
    // 4100 t over 9700 nm.
    let shokaku = &cx.ships[&19];
    assert_eq!(shokaku.cruise, Fixed::from_int(18));
    assert_eq!(shokaku.fuel, Fixed::from_int(4100));
    assert_eq!(shokaku.endurance, Fixed::from_int(9700));
    let burn = |distance: i64| shokaku.fuel * distance / shokaku.endurance;
    assert_eq!(burn(204), millionths(86_226_804));
    assert_eq!(burn(98), millionths(41_422_680));
    assert_eq!(
        millionths(86_226_804) + millionths(41_422_680),
        millionths(127_649_484)
    );
    // Ryujo cruises at 14 knots, so at 20 her burn is multiplied by (20 /
    // 14) squared: 1.428571 squared is 2.040815. She burns 240 x 2490 /
    // 10000 = 59.76, times that, 121.959104 t in the night, and 60.979552 t
    // in the morning, and has 2307.061344 t left.
    let ryujo = &cx.ships[&33];
    assert_eq!(ryujo.cruise, Fixed::from_int(14));
    assert_eq!(ryujo.fuel, Fixed::from_int(2490));
    assert_eq!(ryujo.endurance, Fixed::from_int(10_000));
    let ratio = Fixed::from_int(20) / ryujo.cruise;
    assert_eq!(ratio, millionths(1_428_571));
    let factor = ratio * ratio;
    assert_eq!(factor, millionths(2_040_815));
    let at_cruise = Fixed::from_int(240) * ryujo.fuel / ryujo.endurance;
    assert_eq!(at_cruise, millionths(59_760_000));
    assert_eq!(at_cruise * factor, millionths(121_959_104));
    assert_eq!(
        at_cruise / Fixed::from_int(2) * factor,
        millionths(60_979_552)
    );
    assert_eq!(
        ryujo.fuel - millionths(121_959_104) - millionths(60_979_552),
        millionths(2_307_061_344)
    );

    for run in batch() {
        let seed = run.seed;
        let mut burns: BTreeMap<u32, Vec<(Pulse, Fixed, Fixed)>> = BTreeMap::new();
        for event in &run.events {
            if let EventKind::FuelBurned {
                ship,
                burned_t,
                remaining_t,
                ..
            } = event.kind
            {
                burns.entry(ship.raw()).or_default().push((
                    event.pulse.expect("a pulse"),
                    burned_t,
                    remaining_t,
                ));
            }
        }
        // Every ship: what it burned is what it had less what it has, none
        // is negative, and the American forces burn nothing.
        for (id, info) in &cx.ships {
            let mut fuel = info.fuel;
            for (_, burned, remaining) in burns.get(id).map_or(&[][..], |v| v.as_slice()) {
                assert!(*burned >= Fixed::ZERO);
                fuel -= *burned;
                assert_eq!(*remaining, fuel, "seed {seed}, ship {id}");
                assert!(*remaining >= Fixed::ZERO);
            }
            assert_eq!(run.ships[id].fuel, fuel, "seed {seed}, ship {id}");
            assert!(run.ships[id].fuel >= Fixed::ZERO);
            if info.force <= 2 {
                assert!(!burns.contains_key(id), "seed {seed}");
            }
        }
        // Shokaku: night, morning; Ryujo: night, morning.
        assert_eq!(
            burns[&19],
            vec![
                (
                    Pulse::Night,
                    millionths(86_226_804),
                    millionths(4_100_000_000 - 86_226_804)
                ),
                (
                    Pulse::Morning,
                    millionths(41_422_680),
                    millionths(4_100_000_000 - 127_649_484)
                ),
            ],
            "seed {seed}"
        );
        assert_eq!(run.ships[&19].fuel, millionths(3_972_350_516));
        assert_eq!(
            burns[&33],
            vec![
                (
                    Pulse::Night,
                    millionths(121_959_104),
                    millionths(2_490_000_000 - 121_959_104)
                ),
                (
                    Pulse::Morning,
                    millionths(60_979_552),
                    millionths(2_307_061_344)
                ),
            ],
            "seed {seed}"
        );
        assert_eq!(run.ships[&33].fuel, millionths(2_307_061_344));
    }
}

// ---------------------------------------------------------------------
// The morning.
// ---------------------------------------------------------------------

#[test]
fn the_morning_flies_every_search_and_finds_nothing_and_does_nothing() {
    let cx = ctx();
    // No force is within reach of any search: the searches are 200 nm and
    // 253 nm and every enemy is 280 nm or more away.
    let american = Hex::new(7, -5);
    let (a3, a4) = (Hex::new(8, -13), Hex::new(7, -12));
    for (from, range, enemies) in [
        (american, 200, [a3, a4]),
        (a3, 253, [american, american]),
        (a4, 253, [american, american]),
    ] {
        for enemy in enemies {
            assert!(air(from, enemy) >= 280);
            assert!(air(from, enemy) > range);
        }
    }
    assert_eq!(cx.types[&SBD3].search_range_nm, Fixed::from_int(200));
    // Nor is Henderson Field within the reach of Ryujo's attack unit at
    // 06:00: 280 nm, against its extended reach of 218 (175 x 1.25 = 218.75,
    // cut to a whole mile), so its base strike does not launch in the
    // morning (decision 7 of 0063).
    assert_eq!(air(HENDERSON, a4), 280);
    assert_eq!(
        cx.air
            .reach_nm(cx.types[&B5N2].strike_range_nm, Reach::Extended),
        Fixed::from_int(218)
    );
    assert!(air(HENDERSON, a4) > 218);
    for run in batch() {
        let seed = run.seed;
        let events = morning(run);
        let mut flown: BTreeMap<u32, u32> = BTreeMap::new();
        for event in &events {
            match event.kind {
                EventKind::SearchFlown {
                    air_group,
                    aircraft,
                    ..
                } => {
                    flown.insert(air_group.raw(), aircraft);
                }
                // The morning is movement, and the searches.
                EventKind::TaskForceSteamed { .. }
                | EventKind::FuelBurned { .. }
                | EventKind::TaskForceArrived { .. } => {}
                ref other => panic!("seed {seed}: the morning holds {other}"),
            }
        }
        // Every search is flown, with every aircraft of its group.
        assert_eq!(
            flown,
            BTreeMap::from([(3, 16), (17, 5), (20, 5)]),
            "seed {seed}"
        );
        // No attempt, no report, no contact, no strike and no loss: the
        // morning's kinds above allow none of them. The same from the whole
        // run's events before noon.
        for event in &run.events {
            if event.pulse == Some(Pulse::Afternoon) {
                break;
            }
            assert!(
                !matches!(
                    event.kind,
                    EventKind::SearchAttempt { .. }
                        | EventKind::ReportReceived { .. }
                        | EventKind::StrikeLaunched { .. }
                        | EventKind::BaseStrikeLaunched { .. }
                        | EventKind::AirAttack { .. }
                        | EventKind::BaseAttacked { .. }
                        | EventKind::AircraftLost { .. }
                        | EventKind::ContactAged { .. }
                ),
                "seed {seed}"
            );
        }
        // The aircraft at noon are the aircraft at the start.
        for spec in &GROUPS {
            assert!(run.ready.contains_key(&spec.id));
        }
    }
}

// ---------------------------------------------------------------------
// The afternoon: search.
// ---------------------------------------------------------------------

#[test]
fn the_afternoon_search_attempts_are_the_table_and_the_counts_are_in_their_bands() {
    let american = Hex::new(7, -5);
    let (striking, ryujo) = (Hex::new(8, -11), Hex::new(7, -9));
    // Group 3, from (7,-5), arc 290 to 090, 160 degrees, to 200 nm: the
    // Ryujo detachment at bearing 330 and 160 nm, and the striking force at
    // 223 nm, beyond 200: no attempt.
    assert_eq!(bearing::arc_degrees(290, 90), 160);
    assert!(bearing::in_arc(american, ryujo, 290, 90));
    // Bearing 330 exactly: on the edge of 320-330 and of 330-340 and in no
    // other ten-degree arc.
    assert!(bearing::in_arc(american, ryujo, 320, 330));
    assert!(bearing::in_arc(american, ryujo, 330, 340));
    assert!(!bearing::in_arc(american, ryujo, 340, 350));
    assert!(!bearing::in_arc(american, ryujo, 310, 320));
    assert_eq!(air(american, ryujo), 160);
    assert_eq!(air(american, striking), 223);
    assert!(air(american, striking) > 200);
    // Group 17, from (8,-11), arc 110 to 200, 90 degrees, to 253 nm: about
    // bearing 159, 223 nm.
    assert_eq!(bearing::arc_degrees(110, 200), 90);
    assert!(bearing::in_arc(striking, american, 150, 160));
    assert!(!bearing::in_arc(striking, american, 160, 170));
    assert!(!bearing::in_arc(striking, american, 140, 150));
    assert!(bearing::in_arc(striking, american, 110, 200));
    assert_eq!(air(striking, american), 223);
    // Group 20, from (7,-9): bearing 150 exactly, 160 nm.
    assert!(bearing::in_arc(ryujo, american, 150, 160));
    assert!(bearing::in_arc(ryujo, american, 140, 150));
    assert!(!bearing::in_arc(ryujo, american, 160, 170));
    assert!(!bearing::in_arc(ryujo, american, 130, 140));
    assert!(bearing::in_arc(ryujo, american, 110, 200));
    assert_eq!(air(ryujo, american), 160);
    // The chances, by the formula and as the table gives them: 57.295779 x 16
    // x 20 / (160 x 160) = 18334.64928 / 25600 = 0.716197 for group 3, and
    // 57.295779 x 5 x 20 / (90 x 160) = 5729.5779 / 14400 = 0.397887 for
    // group 20 (0.285479 for group 17, unchanged).
    assert_eq!(search_chance(16, 160, 160), millionths(716_197));
    assert_eq!(search_chance(5, 90, 223), millionths(285_479));
    assert_eq!(search_chance(5, 90, 160), millionths(397_887));
    // The chance that none of the four attempts sights, and its complement.
    // 0.714521 x 0.714521 x 0.602113 x 0.602113 = 0.185091 and 0.814909.
    let none = (Fixed::ONE - millionths(285_479))
        * (Fixed::ONE - millionths(285_479))
        * (Fixed::ONE - millionths(397_887))
        * (Fixed::ONE - millionths(397_887));
    assert_eq!(none, millionths(185_090));
    assert_eq!(Fixed::ONE - none, millionths(814_910));

    let mut allied = 0;
    let mut japanese = 0;
    for work in worked() {
        // Exactly the five attempts of the table.
        let pairs: Vec<(u32, u32)> = work.attempts.iter().map(|a| (a.0, a.1)).collect();
        assert_eq!(
            pairs,
            vec![(3, 4), (17, 1), (17, 2), (20, 1), (20, 2)],
            "seed {}",
            work.seed
        );
        allied += i64::from(work.attempts[0].2);
        japanese += i64::from(work.attempts[1..].iter().any(|a| a.2));
    }
    assert_eq!(count(worked().len()), 1000);
    eprintln!("afternoon sightings over 1000 seeds: Allied {allied}, Japanese {japanese}");
    // The two counts, in the bands the table allows (and four standard
    // deviations of the exact chance).
    assert!((660..=773).contains(&allied), "Allied sightings {allied}");
    assert!(
        (766..=864).contains(&japanese),
        "Japanese sightings {japanese}"
    );
    assert!(within_four_sigma(allied, 1000, 716_197));
    assert!(within_four_sigma(japanese, 1000, 814_910));
}

// ---------------------------------------------------------------------
// The afternoon: who launches.
// ---------------------------------------------------------------------

fn launched(work: &Worked, group: u32) -> Option<&Launch> {
    work.launches.iter().find(|l| l.group == group)
}

fn attacked(run: &Run, group: u32) -> bool {
    run.events.iter().any(
        |e| matches!(e.kind, EventKind::AirAttack { air_group, .. } if air_group.raw() == group),
    )
}

fn found_nothing(run: &Run, group: u32) -> bool {
    run.events.iter().any(|e| {
        matches!(e.kind, EventKind::StrikeFoundNothing { air_group, .. } if air_group.raw() == group)
    })
}

#[test]
fn each_group_launches_when_its_own_side_holds_a_contact_within_its_radius_and_not_otherwise() {
    let cx = ctx();
    let american = Hex::new(7, -5);
    let (striking, ryujo) = (Hex::new(8, -11), Hex::new(7, -9));
    // What the radii and the hexes say, before any run: a hex is reached if
    // its distance in nm is within the radius.
    let reaches = |from: Hex, to: Hex, ty: u32| {
        Fixed::from_int(air(from, to)) <= cx.types[&ty].strike_range_nm
    };
    // The true hex is 160 nm from the American carriers: within the SBD-3's
    // 200 and the TBF-1's 190. It is 200 nm from Henderson Field: within the
    // SBD-3's 200 exactly.
    assert!(reaches(american, ryujo, SBD3));
    assert!(reaches(american, ryujo, TBF1));
    assert!(reaches(HENDERSON, ryujo, SBD3));
    assert_eq!(air(HENDERSON, ryujo), 200);
    assert!(reaches(striking, american, D3A1));
    assert!(!reaches(striking, american, B5N2));
    // A report of the American forces names (7,-5) or a navigable
    // neighbour. Every one of them is 200 nm or more from the striking
    // force's carriers, beyond the B5N2's 175.
    let mut named = vec![american];
    named.extend(
        american
            .neighbors()
            .into_iter()
            .filter(|h| cx.map.is_navigable(*h)),
    );
    for hex in &named {
        assert!(air(striking, *hex) >= 183, "{hex}");
    }
    // The hexes next to the true hex (7,-9) and their distances from the
    // American carriers' (7,-5) and from Henderson Field, by the formula 40 x
    // root (dq x dq + dr x dr + dq x dr), to the nearest mile. A report
    // names the true hex or one of these.
    let at = |q: i32, r: i32| Hex::new(q, r);
    for (hex, from_american, from_henderson) in [
        (at(7, -9), 160, 200),
        (at(8, -9), 144, 223),
        (at(8, -10), 183, 240),
        (at(7, -10), 200, 223),
        (at(6, -9), 183, 183),
        (at(6, -8), 144, 160),
        (at(7, -8), 120, 183),
    ] {
        assert_eq!(air(american, hex), from_american, "{hex}");
        assert_eq!(air(HENDERSON, hex), from_henderson, "{hex}");
    }
    for (hex, nm_from_striking) in [(at(7, -4), 262), (at(6, -4), 250)] {
        assert_eq!(air(striking, hex), nm_from_striking, "{hex}");
    }

    // The Japanese side's contacts are reports of the American carriers, true
    // kind carrier, and a carrier is never reported as a destroyer or an
    // auxiliary (the confusion row gives carrier 0.7, battleship, cruiser and
    // transport 0.1 each). So with the list carrier, battleship, cruiser,
    // transport no report of this scenario is held for, and groups 12 and 15
    // launch in every run in which the Japanese hold a contact within 252 nm
    // (809 of 1,000 sight an American force; the 551 whose report names the
    // true hex attack Enterprise). The hold itself (a kind not on the list is never
    // launched at) is shown in scenarios 13a and 13b. The model above still
    // asserts the rule in every run.
    let row = &cx.search.kind_confusion[&ShipKind::Carrier];
    assert!(!row.contains_key(&ShipKind::Destroyer) && !row.contains_key(&ShipKind::Auxiliary));
    let mut counts: BTreeMap<&str, i64> = BTreeMap::new();
    let mut bump = |name: &'static str| *counts.entry(name).or_default() += 1;
    for (run, work) in batch().iter().zip(worked()) {
        let seed = run.seed;
        let allied_hex = work.last_report[0];
        // Groups 2, 6 and 7, the SBD-3s, and groups 4 and 8, the TBF-1s,
        // launch if the contact's hex is within their radius of (7,-5): 200
        // and 190. Group 10, at Henderson Field, if it is within 200 of
        // (2,-4). The side holds one contact in the afternoon, the report of
        // its only search.
        for (groups, radius) in [(&[2, 6, 7][..], 200), (&[4, 8][..], 190)] {
            for &group in groups {
                let within = allied_hex.is_some_and(|hex| air(american, hex) <= radius);
                assert_eq!(launched(work, group).is_some(), within, "seed {seed}");
                if let Some(launch) = launched(work, group) {
                    assert_eq!(Some(launch.hex), allied_hex);
                    assert_eq!(launch.aircraft, spec_of(group).ready);
                    assert_eq!(launch.distance_nm, air(american, launch.hex));
                }
            }
        }
        let within = allied_hex.is_some_and(|hex| air(HENDERSON, hex) <= 200);
        assert_eq!(launched(work, 10).is_some(), within, "seed {seed}");
        // The true hex: all six groups launch, at 160 nm (200 for group 10),
        // and each finds the force there.
        if allied_hex == Some(ryujo) {
            for group in [2, 4, 6, 7, 8] {
                assert_eq!(launched(work, group).expect("launches").distance_nm, 160);
            }
            assert_eq!(launched(work, 10).expect("launches").distance_nm, 200);
            bump("allied report names the true hex: six groups launch");
        }
        // A report at (7,-10), 200 nm from the carriers: the SBD-3s of the
        // carriers launch (200) and the torpedo groups (190) and group 10
        // (223 from Henderson Field) do not.
        if allied_hex == Some(at(7, -10)) {
            assert!(launched(work, 2).is_some() && launched(work, 6).is_some());
            assert!(launched(work, 4).is_none() && launched(work, 8).is_none());
            assert!(launched(work, 10).is_none(), "seed {seed}");
            bump("allied report at (7,-10): the torpedo groups and group 10 stay");
        }
        // A report beyond Henderson Field's reach: group 10 stays on the
        // ground.
        if allied_hex.is_some_and(|hex| air(HENDERSON, hex) > 200) {
            assert!(launched(work, 10).is_none(), "seed {seed}");
            bump("allied report beyond group 10's 200 nm: it stays");
        }
        for group in [4, 8, 10] {
            if let Some(launch) = launched(work, group) {
                if launch.hex != ryujo {
                    assert!(found_nothing(run, group), "seed {seed}");
                    assert!(!attacked(run, group), "seed {seed}");
                    bump(match group {
                        4 => "group 4 launches and finds nothing",
                        8 => "group 8 launches and finds nothing",
                        _ => "group 10 launches and finds nothing",
                    });
                } else {
                    bump(match group {
                        4 => "group 4 launches at the true hex",
                        8 => "group 8 launches at the true hex",
                        _ => "group 10 launches at the true hex",
                    });
                }
            }
        }
        // Groups 12 and 15: a contact the Japanese side holds, of a kind on
        // their list (carrier, battleship, cruiser, transport), within 252 nm of
        // (8,-11), which the true hex (7,-5) is, at 223. A contact reported as
        // a destroyer or an auxiliary is not launched at.
        for group in [12, 15] {
            let list = target_list(group).expect("a list");
            let wanted = work.held[1].iter().any(|(_, hex, kind)| {
                list.contains(kind)
                    && Fixed::from_int(air(striking, *hex)) <= cx.types[&D3A1].strike_range_nm
            });
            assert_eq!(launched(work, group).is_some(), wanted, "seed {seed}");
            if let Some(launch) = launched(work, group) {
                assert_eq!(launch.aircraft, 27);
                assert!(work.held[1]
                    .iter()
                    .any(|(id, hex, kind)| Some(*id) == launch.contact
                        && *hex == launch.hex
                        && list.contains(kind)));
            }
        }
        if work.held[1].iter().any(|(_, hex, _)| *hex == american) {
            bump("japanese report names the true hex");
        }
        // Groups 13 and 16, the B5N2s of the striking force, never launch.
        assert!(launched(work, 13).is_none(), "seed {seed}");
        assert!(launched(work, 16).is_none(), "seed {seed}");
        // Group 19, Ryujo's attack unit, launches its base strike in every
        // run: Henderson Field is 200 nm from her at noon and 280 at 06:00,
        // within its extended reach of 218 in the afternoon and not in the
        // morning. All 6 aircraft fly, at base 1, at 200 nm, and every launch
        // of it is the base strike.
        let raid = launched(work, 19).expect("the raid launches in every run");
        assert_eq!(
            (raid.base, raid.hex, raid.distance_nm, raid.aircraft),
            (Some(1), HENDERSON, 200, 6),
            "seed {seed}"
        );
        assert!(raid.contact.is_none() && raid.report.is_none());
        assert_eq!(
            work.launches.iter().filter(|l| l.group == 19).count(),
            1,
            "once a day"
        );
        // A strike attacks only if it flew to the hex its target is really
        // in.
        for launch in work.launches.iter().filter(|l| l.base.is_none()) {
            let side = spec_of(launch.group).side;
            let enemy_hex = if side == Side::Allied {
                ryujo
            } else {
                american
            };
            let attack = attacked(run, launch.group);
            if launch.hex == enemy_hex {
                // At the right hex it attacks, unless every one of its
                // bombers was shot down on the way in (each shot is rolled
                // on its own, decision 0033): then it records nothing.
                let left = launch.aircraft - work.lost.get(&launch.group).copied().unwrap_or(0);
                assert_eq!(attack, left > 0, "seed {seed}");
                assert!(!found_nothing(run, launch.group), "seed {seed}");
            } else {
                assert!(!attack, "seed {seed}");
                assert!(found_nothing(run, launch.group), "seed {seed}");
            }
        }
    }
    eprintln!("launch cases over 1000 seeds: {counts:?}");
    for name in [
        "allied report names the true hex: six groups launch",
        "allied report at (7,-10): the torpedo groups and group 10 stay",
        "allied report beyond group 10's 200 nm: it stays",
        "group 4 launches at the true hex",
        "group 8 launches at the true hex",
        "group 10 launches at the true hex",
        "group 4 launches and finds nothing",
        "group 8 launches and finds nothing",
        "group 10 launches and finds nothing",
        "japanese report names the true hex",
    ] {
        assert!(counts.get(name).copied().unwrap_or(0) > 0, "{name}");
    }
}

// ---------------------------------------------------------------------
// The raids.
// ---------------------------------------------------------------------

#[test]
fn an_american_raid_on_the_ryujo_detachment_is_as_the_table_says() {
    let cx = ctx();
    let mut raids = 0;
    let mut runs_with_raids = 0;
    // Shots and hits over all raids, for the bands: (shots, hits) of the
    // patrol at the bombers and of the guns.
    let mut patrol_total = (0i64, 0i64);
    let mut gun_total = (0i64, 0i64);
    for (run, work) in batch().iter().zip(worked()) {
        let seed = run.seed;
        let found: Vec<&RaidWork> = work
            .raids
            .iter()
            .filter(|r| r.found && r.side == Side::Allied)
            .collect();
        // Enterprise's raid is groups 2 and 4, Saratoga's groups 6, 7 and 8,
        // and Henderson Field's group 10: three raids, because they leave
        // from three places. A raid is found only at the true hex, where all
        // six groups launch, so a run has all three raids or none.
        let mut shapes: Vec<Vec<u32>> = found
            .iter()
            .map(|r| r.strikes.iter().map(|s| s.0).collect())
            .collect();
        shapes.sort();
        assert!(
            shapes.is_empty() || shapes == vec![vec![2, 4], vec![6, 7, 8], vec![10]],
            "seed {seed}: {shapes:?}"
        );
        runs_with_raids += i64::from(!found.is_empty());
        for raid in found {
            raids += 1;
            let bombers: u32 = raid.strikes.iter().map(|s| s.1).sum();
            assert!(matches!(bombers, 31 | 43 | 11));
            assert_eq!(raid.bombers, bombers);
            assert_eq!(raid.target_force, Some(4));
            assert_eq!(raid.target_ship, Some(33));
            // Enterprise's raid (31) leaves Enterprise, Saratoga's (43)
            // Saratoga, group 10's (11) Henderson Field.
            assert_eq!(
                raid.place,
                match bombers {
                    31 => Place::Ship(1),
                    43 => Place::Ship(11),
                    _ => Place::Base(1),
                }
            );
            assert_eq!(raid.hex, Hex::new(7, -9));
            // No escort: the target is 160 nm from the carriers and 200 nm
            // from Henderson Field, and the F4F-4's radius is 105 (decision
            // 0030).
            assert!(raid.escorts.is_empty(), "seed {seed}");
            assert_eq!(raid.escort_count, 0);
            assert!(raid.escorts_at_patrol.is_none() && raid.patrol_at_escorts.is_none());
            // Ryujo's fighters: 23 with a share of 0.35 keep 8 overhead (23 x
            // 0.35 = 8.05, rounded down), as they stood at the start of the
            // pulse. The three raids, of 31, 43 and 11 aircraft, divide them
            // (decision 0033): 8 x 31 / 85 = 2.9, 8 x 43 / 85 = 4.05 and
            // 8 x 11 / 85 = 1.03, so 2, 4 and 1, and the one left over goes to
            // the largest raid: 2, 5 and 1.
            assert_eq!(raid.patrol_over_hex, 8, "seed {seed}");
            let expected_share = match bombers {
                31 => 2,
                43 => 5,
                _ => 1,
            };
            assert_eq!(raid.patrol, expected_share, "seed {seed}");
            assert_eq!(raid.all_raids_aircraft, 85, "seed {seed}");
            let (shots, patrol_hits) = raid.patrol_at_bombers.expect("a patrol roll");
            assert_eq!(shots, expected_share);
            // The shots at 0.16875 expect 0.3375, 0.84375 or 0.16875; each is
            // rolled on its own, so from no hit to the shots, and the loss is
            // no more than the bombers there are.
            let first_group = raid.strikes[0].0;
            let (chance, _) = shot_chance(cx, cx.air.fighter_hit_chance, first_group);
            assert_eq!(chance, millionths(168_750));
            assert_eq!(
                int(shots) * chance,
                match shots {
                    2 => millionths(337_500),
                    5 => millionths(843_750),
                    1 => millionths(168_750),
                    other => panic!("seed {seed}: {other} shots"),
                }
            );
            assert!(patrol_hits <= shots, "seed {seed}");
            patrol_total.0 += i64::from(shots);
            patrol_total.1 += i64::from(patrol_hits);
            let lost_to_patrol = patrol_hits.min(bombers);
            let (gun_shots, gun_hits) = raid.guns.expect("a gun roll");
            assert_eq!(gun_shots, 26);
            // 26 shots at 0.028125 expect 0.73125.
            let (chance, _) = shot_chance(cx, cx.air.anti_aircraft_hit_chance, first_group);
            assert_eq!(chance, millionths(28_125));
            assert_eq!(int(gun_shots) * chance, millionths(731_250));
            assert!(gun_hits <= gun_shots, "seed {seed}");
            gun_total.0 += i64::from(gun_shots);
            gun_total.1 += i64::from(gun_hits);
            let lost_to_guns = gun_hits.min(bombers - lost_to_patrol);
            // Of the raid's bombers, those less both losses attack.
            let attacking = bombers - lost_to_patrol - lost_to_guns;
            assert_eq!(raid.bombers_attacking, attacking, "seed {seed}");
            // The losses come off the lowest group first: each group attacks
            // with what is left of it.
            let attacks: Vec<&AttackWork> = work
                .attacks
                .iter()
                .filter(|a| raid.strikes.iter().any(|s| s.0 == a.group))
                .collect();
            let lost = bombers - attacking;
            let mut pool = lost;
            for &(group, aircraft) in &raid.strikes {
                let took = pool.min(aircraft);
                pool -= took;
                let shots_of = attacks
                    .iter()
                    .find(|a| a.group == group)
                    .map_or(0, |a| a.shots);
                assert_eq!(shots_of, aircraft - took, "seed {seed}, group {group}");
            }
        }
        // Every bomber that attacks attacks Ryujo, ship 33: the dive bombers
        // with bombs, the torpedo groups (4 and 8) with torpedoes.
        for attack in work
            .attacks
            .iter()
            .filter(|a| spec_of(a.group).side == Side::Allied)
        {
            assert_eq!(attack.ship, 33, "seed {seed}");
            assert_eq!(attack.flotation_hits + attack.fire_hits, attack.hits);
            assert!(attack.hits <= attack.shots);
            if matches!(attack.group, 4 | 8) {
                assert_eq!(
                    attack.fire_hits, 0,
                    "a torpedo hit is always below the line"
                );
            }
        }
    }
    eprintln!(
        "American raids on Ryujo {raids}, runs with the three raids {runs_with_raids}, \
         patrol hits {} of {} shots, gun hits {} of {}",
        patrol_total.1, patrol_total.0, gun_total.1, gun_total.0
    );
    // Over the runs the hits of each roll are within four standard
    // deviations of the sum of the expected figures.
    assert!(within_four_sigma(patrol_total.1, patrol_total.0, 168_750));
    assert!(within_four_sigma(gun_total.1, gun_total.0, 28_125));
    assert_eq!(raids, 3 * runs_with_raids);
    assert!(runs_with_raids > 400, "{runs_with_raids} runs with raids");
}

#[test]
fn ryujos_raid_on_henderson_field_is_as_the_table_says() {
    let cx = ctx();
    // The sums of each roll over the runs, (shots, hits), for the bands.
    let mut escorts_at_patrol = (0i64, 0i64);
    let mut patrol_at_escorts = (0i64, 0i64);
    let mut bombs = (0i64, 0i64);
    let mut fuel = (0i64, 0i64);
    let mut on_the_ground_runs = 0;
    let mut destroyed_runs = 0;
    for (run, work) in batch().iter().zip(worked()) {
        let seed = run.seed;
        let raid = work
            .raids
            .iter()
            .find(|r| r.target_base.is_some())
            .expect("the raid on the base is flown in every run");
        // Group 19, 6 B5N2, launches from Ryujo at base 1: one raid, with
        // Ryujo's 15 escorts (23 x 0.35 = 8.05, so 8 on patrol and 15 sent).
        assert_eq!(raid.place, Place::Ship(33));
        assert_eq!(raid.side, Side::Japanese);
        assert_eq!(raid.hex, HENDERSON);
        assert_eq!(raid.strikes, vec![(19, 6)]);
        assert_eq!(raid.escorts, vec![(18, 15)], "seed {seed}");
        assert_eq!(raid.escort_count, 15);
        assert_eq!((raid.target_force, raid.target_ship), (None, None));
        assert_eq!(raid.target_base, Some(1));
        // The escorts fly 200 nm, within the A6M2's radius of 261: no fuel
        // roll for them. The strike flies 200 nm on a radius of 175.
        assert!(air(place_hex(Place::Ship(33)), HENDERSON) <= 261);
        // Over Henderson Field the patrol is VMF-223's 13 (shares of 1.0), one
        // raid alone meets all 13: 13 x 21 / 21. The 6 bombers and 15 escorts
        // make 21 aircraft.
        assert_eq!(raid.patrol_over_hex, 13, "seed {seed}");
        assert_eq!(raid.patrol, 13, "seed {seed}");
        assert_eq!(raid.all_raids_aircraft, 21, "seed {seed}");
        // Escorts at the patrol: 15 shots at 0.16875 (an F4F-4 has both
        // protections), expecting 2.53125. Patrol at the escorts: 13 shots at
        // 0.3, expecting 3.9. The whole share goes at the escorts, 13 of 15,
        // so none is left for the bombers.
        let (shots, hits) = raid.escorts_at_patrol.expect("the escorts fire");
        assert_eq!(shots, 15);
        let (chance, _) = shot_chance(cx, cx.air.fighter_hit_chance, 9);
        assert_eq!(chance, millionths(168_750));
        assert_eq!(int(shots) * chance, millionths(2_531_250));
        assert!(hits <= shots);
        escorts_at_patrol.0 += i64::from(shots);
        escorts_at_patrol.1 += i64::from(hits);
        let (shots_pe, hits_pe) = raid.patrol_at_escorts.expect("the patrol fires");
        assert_eq!(shots_pe, 13);
        let (chance, _) = shot_chance(cx, cx.air.fighter_hit_chance, 18);
        assert_eq!(chance, tenths(3));
        assert_eq!(int(shots_pe) * chance, millionths(3_900_000));
        assert!(hits_pe <= shots_pe);
        patrol_at_escorts.0 += i64::from(shots_pe);
        patrol_at_escorts.1 += i64::from(hits_pe);
        assert!(raid.patrol_at_bombers.is_none(), "seed {seed}");
        // The base has no guns here.
        assert!(raid.guns.is_none(), "seed {seed}");
        // Losses: VMF-223 loses what the escorts hit (of its 13), Ryujo's
        // fighters what the patrol hits (of the 15 escorts). The bombers are
        // not fired at, and all 6 attack.
        assert_eq!(work.lost.get(&9).copied().unwrap_or(0), hits.min(13));
        assert_eq!(work.lost.get(&18).copied().unwrap_or(0), hits_pe.min(15));
        assert_eq!(work.lost.get(&19).copied().unwrap_or(0), 0);
        assert_eq!(raid.bombers_attacking, 6);
        // The bombs: 6 shots at 0.2, expecting 1.2, each hit landing on
        // aircraft, on supply (none there to destroy) or on nothing.
        let [attack] = work.base_attacks.as_slice() else {
            panic!("seed {seed}: one attack on the base");
        };
        assert_eq!((attack.group, attack.shots), (19, 6));
        assert_eq!(
            int(attack.shots) * cx.air.bomb_hit_chance,
            millionths(1_200_000)
        );
        assert!(attack.hits <= attack.shots);
        assert!(attack.on_aircraft + attack.on_supply <= attack.hits);
        bombs.0 += i64::from(attack.shots);
        bombs.1 += i64::from(attack.hits);
        // The aircraft on the ground are VMSB-232's 11 when it did not launch
        // at Ryujo (VMF-223's are all on patrol), and none when it did. Each
        // hit on aircraft destroys one of them.
        let resting = if launched(work, 10).is_some() { 0 } else { 11 };
        on_the_ground_runs += i64::from(resting > 0);
        let destroyed = attack.on_aircraft.min(resting);
        let expected_destroyed: Vec<(u32, u32)> = if destroyed > 0 {
            vec![(10, destroyed)]
        } else {
            Vec::new()
        };
        assert_eq!(attack.destroyed, expected_destroyed, "seed {seed}");
        destroyed_runs += i64::from(destroyed > 0);
        // The fuel: 200 nm is 25 beyond the radius of 175, 25 / 175 =
        // 0.142857 of it, and the ditch figure is 2 times that, 0.285714. All
        // 6 came back to roll: 6 x 0.285714 = 1.714284 expected.
        let [roll] = work.fuel_rolls.as_slice() else {
            panic!("seed {seed}: one fuel roll");
        };
        assert_eq!((roll.group, roll.came_back), (19, 6));
        assert_eq!(roll.chance, millionths(285_714));
        assert_eq!(int(roll.came_back) * roll.chance, millionths(1_714_284));
        assert!(roll.ditched <= roll.came_back);
        fuel.0 += i64::from(roll.came_back);
        fuel.1 += i64::from(roll.ditched);
    }
    eprintln!(
        "Ryujo's raid: escorts at patrol {escorts_at_patrol:?}, patrol at escorts \
         {patrol_at_escorts:?}, bombs {bombs:?}, fuel {fuel:?}, group 10 on the ground in \
         {on_the_ground_runs} runs and hit in {destroyed_runs}"
    );
    // Over the runs the hits of each roll are within four standard
    // deviations of the sum of the expected figures.
    assert!(within_four_sigma(
        escorts_at_patrol.1,
        escorts_at_patrol.0,
        168_750
    ));
    assert!(within_four_sigma(
        patrol_at_escorts.1,
        patrol_at_escorts.0,
        300_000
    ));
    assert!(within_four_sigma(bombs.1, bombs.0, 200_000));
    assert!(within_four_sigma(fuel.1, fuel.0, 285_714));
    assert!(on_the_ground_runs > 100 && destroyed_runs > 20);
}

#[test]
fn a_japanese_raid_on_the_american_carriers_is_as_the_table_says() {
    let cx = ctx();
    let mut raids = 0;
    let mut both = 0;
    // (shots, hits) over all raids of each roll, for the bands.
    let mut totals = [(0i64, 0i64); 4];
    let mut alone = 0;
    for (run, work) in batch().iter().zip(worked()) {
        let seed = run.seed;
        let found: Vec<&RaidWork> = work
            .raids
            .iter()
            .filter(|r| r.found && r.side == Side::Japanese && r.target_base.is_none())
            .collect();
        // Two raids, because they leave from two ships: Shokaku's is group
        // 12, Zuikaku's group 15. Neither is ever combined with another.
        let firsts: Vec<u32> = found.iter().map(|r| r.strikes[0].0).collect();
        assert!(
            firsts.is_empty() || firsts == [12, 15] || firsts == [12] || firsts == [15],
            "seed {seed}: {firsts:?}"
        );
        both += i64::from(found.len() == 2);
        let found_count = found.len();
        for raid in found {
            raids += 1;
            assert_eq!(raid.strikes.len(), 1);
            assert_eq!(raid.strikes[0].1, 27, "27 D3A1");
            assert_eq!(raid.bombers, 27);
            // The target is 223 nm away and the A6M2's radius is 261, so the
            // escorts go: Shokaku's 26 fighters keep 20 and send 6;
            // Zuikaku's 25 keep 20 and send 5.
            let (escort_group, escorts) = match raid.strikes[0].0 {
                12 => (11, 6),
                15 => (14, 5),
                other => panic!("seed {seed}: raid of group {other}"),
            };
            assert_eq!(raid.escorts, vec![(escort_group, escorts)], "seed {seed}");
            assert_eq!(raid.escort_count, escorts);
            // The American patrol over (7,-5) as it stood at the start of
            // the pulse is groups 1 and 5, 54 fighters; Henderson Field's
            // are in another hex. The two raids divide it, Shokaku's of 33
            // aircraft 28 and Zuikaku's of 32 aircraft 26; a raid alone
            // meets all 54 (decision 0033).
            assert_eq!(raid.patrol_over_hex, 54, "seed {seed}");
            let share = if found_count == 1 {
                54
            } else if escort_group == 11 {
                28
            } else {
                26
            };
            assert_eq!(raid.patrol, share, "seed {seed}");
            assert_eq!(
                raid.all_raids_aircraft,
                if found_count == 1 { 27 + escorts } else { 65 },
                "seed {seed}"
            );
            alone += i32::from(found_count == 1);
            assert_eq!(raid.target_force, Some(1));
            assert_eq!(raid.target_ship, Some(1));
            assert_eq!(
                raid.place,
                Place::Ship(if escort_group == 11 { 19 } else { 20 })
            );
            assert_eq!(raid.hex, Hex::new(7, -5));
            // Every roll is one draw for each shot (decision 0033): the
            // shots and chance are the table's, the expected figure is their
            // product, and the hits are from none to the shots.
            let group = raid.strikes[0].0;
            let fighter = |target: u32| shot_chance(cx, cx.air.fighter_hit_chance, target).0;
            // Escorts at the patrol: 6 x 0.16875 = 1.0125; 5 x 0.16875 =
            // 0.84375.
            let (shots, hits) = raid.escorts_at_patrol.expect("escorts fire");
            assert_eq!(shots, escorts);
            assert_eq!(fighter(1), millionths(168_750));
            assert_eq!(
                int(shots) * fighter(1),
                if escorts == 6 {
                    millionths(1_012_500)
                } else {
                    millionths(843_750)
                }
            );
            assert!(hits <= shots, "seed {seed}");
            totals[0].0 += i64::from(shots);
            totals[0].1 += i64::from(hits);
            // Patrol at the escorts: 6 x 0.3 = 1.8; 5 x 0.3 = 1.5.
            let (shots, at_escorts_hits) = raid.patrol_at_escorts.expect("patrol fires");
            assert_eq!(shots, escorts);
            assert_eq!(fighter(escort_group), millionths(300_000));
            assert_eq!(
                int(shots) * fighter(escort_group),
                if escorts == 6 {
                    millionths(1_800_000)
                } else {
                    millionths(1_500_000)
                }
            );
            assert!(at_escorts_hits <= shots, "seed {seed}");
            totals[1].0 += i64::from(shots);
            totals[1].1 += i64::from(at_escorts_hits);
            // Patrol at the bombers: what is left of the share after the
            // escorts, at 0.3 a shot: 22 x 0.3 = 6.6 and 21 x 0.3 = 6.3 when
            // both raids attack, 48 x 0.3 = 14.4 and 49 x 0.3 = 14.7 alone.
            let (shots, at_bombers_hits) = raid.patrol_at_bombers.expect("patrol fires");
            assert_eq!(shots, share - escorts);
            assert_eq!(
                shots,
                match (found_count, escorts) {
                    (2, 6) => 22,
                    (2, _) => 21,
                    (_, 6) => 48,
                    (_, _) => 49,
                }
            );
            assert_eq!(fighter(group), millionths(300_000));
            assert_eq!(
                int(shots) * fighter(group),
                match shots {
                    22 => millionths(6_600_000),
                    21 => millionths(6_300_000),
                    48 => millionths(14_400_000),
                    _ => millionths(14_700_000),
                }
            );
            assert!(at_bombers_hits <= shots, "seed {seed}");
            totals[2].0 += i64::from(shots);
            totals[2].1 += i64::from(at_bombers_hits);
            // The whole share is committed: shots at the escorts and at the
            // bombers add to it.
            assert_eq!(raid.patrol_at_escorts.expect("fires").0 + shots, share);
            // Guns at the bombers: 123 x 0.05 = 6.15.
            let (gun_shots, gun_hits) = raid.guns.expect("guns fire");
            assert_eq!(gun_shots, 123);
            let (chance, _) = shot_chance(cx, cx.air.anti_aircraft_hit_chance, group);
            assert_eq!(chance, millionths(50_000));
            assert_eq!(int(gun_shots) * chance, millionths(6_150_000));
            assert!(gun_hits <= gun_shots, "seed {seed}");
            totals[3].0 += i64::from(gun_shots);
            totals[3].1 += i64::from(gun_hits);
            // Bombers that attack: 27 less both losses, each loss no more
            // than the bombers there are.
            let lost_to_patrol = at_bombers_hits.min(27);
            let lost_to_guns = gun_hits.min(27 - lost_to_patrol);
            assert_eq!(
                raid.bombers_attacking,
                27 - lost_to_patrol - lost_to_guns,
                "seed {seed}"
            );
        }
        // Every bomber attacks Enterprise, ship 1.
        for attack in work
            .attacks
            .iter()
            .filter(|a| spec_of(a.group).side == Side::Japanese)
        {
            assert_eq!(attack.flotation_hits + attack.fire_hits, attack.hits);
            assert_eq!(attack.ship, 1, "seed {seed}");
            assert!(attack.shots <= 27, "seed {seed}");
        }
    }
    eprintln!("Japanese raids {raids}, runs with both raids {both}, (shots, hits) {totals:?}");
    assert!(raids > 0 && both > 0);
    eprintln!("Japanese raids that met the patrol alone {alone}");
    // Over the runs the hits of each roll are within four standard
    // deviations of the sum of the expected figures.
    for (total, chance) in totals.iter().zip([168_750, 300_000, 300_000, 50_000]) {
        assert!(within_four_sigma(total.1, total.0, chance), "{total:?}");
    }
}

#[test]
fn the_shot_numbers_and_chances_of_the_tables_are_in_the_events() {
    // The first time each kind of roll appears, its event reads as the tables
    // say, with its figures.
    let mut seen: BTreeSet<&str> = BTreeSet::new();
    for run in batch() {
        for event in &run.events {
            let (name, shots, chance, expected) = match event.kind {
                EventKind::AirFight {
                    firer,
                    firing,
                    at,
                    shots,
                    chance,
                    expected,
                    ..
                } => {
                    let name = match (firer.raw(), firing, at) {
                        (18, AirFirer::Patrol, AirTarget::Bombers) => "ryujo patrol at bombers",
                        (18, AirFirer::Escorts, AirTarget::Patrol) => "ryujo escorts at patrol",
                        (9, AirFirer::Patrol, AirTarget::Escorts) => "henderson patrol at escorts",
                        (11, AirFirer::Escorts, AirTarget::Patrol) => "shokaku escorts at patrol",
                        (14, AirFirer::Escorts, AirTarget::Patrol) => "zuikaku escorts at patrol",
                        (1, AirFirer::Patrol, AirTarget::Escorts) => "patrol at escorts",
                        (1, AirFirer::Patrol, AirTarget::Bombers) => "patrol at bombers",
                        _ => "other",
                    };
                    (name, shots, chance, expected)
                }
                EventKind::AntiAircraftFire {
                    task_force,
                    shots,
                    chance,
                    expected,
                    ..
                } => (
                    if task_force.raw() == 4 {
                        "guns of task force 4"
                    } else {
                        "guns of task force 1"
                    },
                    shots,
                    chance,
                    expected,
                ),
                _ => continue,
            };
            match name {
                "ryujo patrol at bombers" => {
                    // Enterprise's raid meets 2 of the 8, Saratoga's 5 and
                    // Henderson Field's 1 (when all three attack whole).
                    assert!(
                        shots == int(2) || shots == int(5) || shots == int(1),
                        "{shots}"
                    );
                    assert_eq!(chance, millionths(168_750));
                    assert_eq!(expected, shots * millionths(168_750));
                }
                "ryujo escorts at patrol" => {
                    // Over Henderson Field: 15 escorts at VMF-223's 13.
                    assert_eq!(
                        (shots, chance, expected),
                        (int(15), millionths(168_750), millionths(2_531_250))
                    );
                }
                "henderson patrol at escorts" => {
                    assert_eq!(
                        (shots, chance, expected),
                        (int(13), tenths(3), millionths(3_900_000))
                    );
                }
                "guns of task force 4" => {
                    assert_eq!(
                        (shots, chance, expected),
                        (int(26), millionths(28_125), millionths(731_250))
                    );
                }
                "shokaku escorts at patrol" => {
                    assert_eq!(
                        (shots, chance, expected),
                        (int(6), millionths(168_750), millionths(1_012_500))
                    );
                }
                "zuikaku escorts at patrol" => {
                    assert_eq!(
                        (shots, chance, expected),
                        (int(5), millionths(168_750), millionths(843_750))
                    );
                }
                "patrol at escorts" => {
                    assert_eq!(chance, tenths(3));
                    assert!(shots == int(6) || shots == int(5));
                    assert_eq!(expected, shots * tenths(3));
                }
                "patrol at bombers" => {
                    assert_eq!(chance, tenths(3));
                    assert!(
                        // Both raids attack: 22 and 21 of the shares. A raid
                        // alone: 48 and 49.
                        (shots, expected) == (int(22), tenths(66))
                            || (shots, expected) == (int(21), tenths(63))
                            || (shots, expected) == (int(48), tenths(144))
                            || (shots, expected) == (int(49), tenths(147)),
                        "{shots} {expected}"
                    );
                }
                "guns of task force 1" => {
                    assert_eq!(
                        (shots, chance, expected),
                        (int(123), hundredths(5), millionths(6_150_000))
                    );
                }
                _ => {}
            }
            seen.insert(name);
        }
    }
    for name in [
        "ryujo patrol at bombers",
        "ryujo escorts at patrol",
        "henderson patrol at escorts",
        "guns of task force 4",
        "shokaku escorts at patrol",
        "zuikaku escorts at patrol",
        "patrol at escorts",
        "patrol at bombers",
        "guns of task force 1",
    ] {
        assert!(seen.contains(name), "{name}");
    }
    assert_eq!(tenths(144), millionths(14_400_000));
}

// ---------------------------------------------------------------------
// What cannot happen.
// ---------------------------------------------------------------------

#[test]
fn what_cannot_happen_never_does() {
    for (run, work) in batch().iter().zip(worked()) {
        let seed = run.seed;
        // No ship fires a gun or a torpedo at another ship: no two opposing
        // forces are ever in one hex.
        for event in &run.events {
            assert!(
                !matches!(
                    event.kind,
                    EventKind::Salvo { .. }
                        | EventKind::EngagementBegun { .. }
                        | EventKind::SightingRoll { .. }
                        | EventKind::AlarmRoll { .. }
                        | EventKind::ForceBrokeOff { .. }
                ),
                "seed {seed}: {}",
                event.kind
            );
            if let EventKind::TaskForceSteamed {
                task_force,
                from,
                to,
                ref hexes_entered,
                ..
            } = event.kind
            {
                // Only the Japanese forces move, and none enters (7,-5) or
                // any hex south of row -9.
                assert!(task_force.raw() >= 3, "seed {seed}");
                for hex in std::iter::once(&from)
                    .chain(std::iter::once(&to))
                    .chain(hexes_entered)
                {
                    assert!(hex.r <= -9, "seed {seed}: {hex}");
                    assert_ne!(*hex, Hex::new(7, -5));
                }
            }
        }
        assert_ne!(run.positions[&1], run.positions[&3]);
        assert_ne!(run.positions[&1], run.positions[&4]);
        assert_ne!(run.positions[&2], run.positions[&3]);
        assert_ne!(run.positions[&2], run.positions[&4]);
        // No F4F-4 ever escorts a raid, and no B5N2 of the striking force
        // ever launches. Group 19, Ryujo's B5N2s, launches only the base
        // strike on Henderson Field, 200 nm away, and never a naval strike;
        // no B5N2 ever attacks a ship.
        for event in &run.events {
            match event.kind {
                EventKind::EscortsSent { air_group, .. } => {
                    assert!(
                        matches!(air_group.raw(), 11 | 14 | 18),
                        "seed {seed}: group {} escorts",
                        air_group.raw()
                    );
                    for fighters in [1, 5, 9] {
                        assert_ne!(air_group.raw(), fighters);
                    }
                }
                EventKind::StrikeLaunched { air_group, .. } => {
                    assert!(!matches!(air_group.raw(), 13 | 16), "seed {seed}");
                }
                _ => {}
            }
        }
        // Group 19 launches only at base 1, 200 nm, in every run, and attacks
        // no ship; Ryujo's 15 escorts are sent exactly when it launches.
        if let Some(l) = launched(work, 19) {
            assert_eq!(l.hex, HENDERSON, "seed {seed}");
            assert_eq!(l.distance_nm, 200);
            assert_eq!(l.base, Some(1));
            assert!(!found_nothing(run, 19) && !attacked(run, 19), "seed {seed}");
            let sent: Vec<u32> = run
                .events
                .iter()
                .filter_map(|e| match e.kind {
                    EventKind::EscortsSent {
                        air_group,
                        with_air_group,
                        aircraft,
                    } if air_group.raw() == 18 => {
                        assert_eq!(with_air_group.raw(), 19);
                        Some(aircraft)
                    }
                    _ => None,
                })
                .collect();
            assert_eq!(sent, [15], "seed {seed}");
        }
        for e in &run.events {
            if let EventKind::AirAttack { air_group, .. } = e.kind {
                assert!(!matches!(air_group.raw(), 13 | 16 | 19), "seed {seed}");
            }
        }
        if launched(work, 19).is_none() {
            assert!(
                !run.events.iter().any(|e| matches!(
                    e.kind,
                    EventKind::EscortsSent { air_group, .. } if air_group.raw() == 18
                )),
                "seed {seed}"
            );
        }
        // No aircraft at Henderson Field is lost with a ship, and no ship of
        // task force 2 or task force 3 is ever damaged. (VMF-223 now loses
        // aircraft to Ryujo's escorts over the field, and VMSB-232 to Ryujo's
        // patrol and guns and, when it did not launch, on the ground.)
        for event in &run.events {
            match event.kind {
                EventKind::AirGroupLost { air_group, .. } => {
                    assert!(!matches!(air_group.raw(), 9 | 10), "seed {seed}");
                }
                EventKind::ShipDamaged { ship, .. }
                | EventKind::SystemHit { ship, .. }
                | EventKind::ShipSunk { ship, .. } => {
                    assert!(!(11..=32).contains(&ship.raw()), "seed {seed}");
                }
                EventKind::AirAttack { task_force, .. } => {
                    // Saratoga's force is never attacked; neither is the
                    // striking force.
                    assert!(!matches!(task_force.raw(), 2 | 3), "seed {seed}");
                }
                EventKind::SearchAttempt { task_force, .. } => {
                    // The striking force is never sighted.
                    assert_ne!(task_force.raw(), 3, "seed {seed}");
                }
                _ => {}
            }
        }
        // Henderson Field's groups have what they started with, less what
        // was shot down, and what was destroyed on the ground, plus what
        // landed there when Enterprise's deck went out.
        let lost_of = |group: u32| -> u32 {
            run.events
                .iter()
                .map(|e| match e.kind {
                    EventKind::AircraftLost {
                        air_group,
                        aircraft,
                        ..
                    }
                    | EventKind::AircraftDestroyedOnGround {
                        air_group,
                        aircraft,
                        ..
                    } if air_group.raw() == group => aircraft,
                    _ => 0,
                })
                .sum()
        };
        assert_eq!(
            run.ready[&9],
            13 - lost_of(9) + landed_to(run, 9),
            "seed {seed}"
        );
        assert_eq!(
            run.ready[&10],
            11 - lost_of(10) + landed_to(run, 10),
            "seed {seed}"
        );
        for ship in 11..=32u32 {
            let last = &run.ships[&ship];
            assert_eq!(last.flotation, Fixed::ZERO);
            assert_eq!(last.fire, Fixed::ZERO);
            assert!(!last.sunk);
            assert!(last
                .systems
                .iter()
                .all(|state| *state == SystemState::Working));
        }
        // The striking force is never sighted and never attacked.
        assert!(work.attempts.iter().all(|a| a.1 != 3), "seed {seed}");
        assert!(work.attacks.iter().all(|a| force_of(a.ship) != 3));
    }
}

#[test]
fn losses_come_off_the_lower_group_first() {
    // Saratoga's losses come off group 6 before 7, and the American patrol's
    // off group 1 before 5: the losses are a few aircraft, so groups 7 and 5
    // never lose any.
    let mut six = 0;
    let mut one = 0;
    for run in batch() {
        for event in &run.events {
            if let EventKind::AircraftLost { air_group, .. } = event.kind {
                match air_group.raw() {
                    6 => six += 1,
                    1 => one += 1,
                    7 | 5 => panic!("seed {}: group {} lost aircraft", run.seed, air_group.raw()),
                    _ => {}
                }
            }
        }
        // Their own losses are none; what landed on Saratoga's groups when
        // Enterprise's deck went out is theirs now (decisions 7 to 10 of 0059).
        assert_eq!(run.ready[&7], 15 + landed_to(run, 7), "seed {}", run.seed);
        assert_eq!(run.ready[&5], 27 + landed_to(run, 5), "seed {}", run.seed);
        assert!(run.ready[&6] <= 15 + landed_to(run, 6) && run.ready[&1] <= 27 + landed_to(run, 1));
    }
    assert!(six > 100 && one > 100, "{six} {one}");
}

// ---------------------------------------------------------------------
// Properties.
// ---------------------------------------------------------------------

#[test]
fn every_strike_is_launched_at_a_contact_its_own_side_holds_within_its_types_radius() {
    let cx = ctx();
    let mut launches = 0;
    for (run, work) in batch().iter().zip(worked()) {
        let by_id: BTreeMap<EventId, &Event> = run.events.iter().map(|e| (e.id, e)).collect();
        for launch in work.launches.iter().filter(|l| l.base.is_none()) {
            launches += 1;
            let spec = spec_of(launch.group);
            // It cites a report of its own side, from this pulse's search
            // (there was nothing the pulse before).
            let report = by_id[&launch.report.expect("a naval strike cites a report")];
            let EventKind::ReportReceived { side, hex, .. } = report.kind else {
                panic!("a launch cites a report");
            };
            assert_eq!(side, spec.side);
            assert_eq!(hex, launch.hex);
            assert_eq!(report.pulse, Some(Pulse::Afternoon));
            assert!(report.id < launch.event);
            // The report was this side's own search.
            let Cause::Event(attempt) = report.causes[0] else {
                panic!("a report cites an event");
            };
            let EventKind::SearchAttempt { air_group, .. } = by_id[&attempt].kind else {
                panic!("an early report is a search's");
            };
            assert_eq!(spec_of(air_group.raw()).side, spec.side);
            // Within the launching type's radius of where the group is.
            let distance = air(place_hex(spec.place), launch.hex);
            assert_eq!(distance, launch.distance_nm);
            assert!(Fixed::from_int(distance) <= cx.types[&spec.ty].strike_range_nm);
        }
    }
    assert!(launches > 2000, "{launches} launches");
}

#[test]
fn every_escort_sent_is_within_its_types_radius_of_the_raids_target() {
    let cx = ctx();
    let mut sent = 0;
    for work in worked() {
        for raid in &work.raids {
            for &(group, _) in &raid.escorts {
                sent += 1;
                let spec = spec_of(group);
                let distance = air(place_hex(spec.place), raid.hex);
                assert!(
                    Fixed::from_int(distance) <= cx.types[&spec.ty].strike_range_nm,
                    "seed {}: group {group} escorts {distance} nm",
                    work.seed
                );
            }
        }
    }
    assert!(sent > 1000, "{sent} escort groups sent");
}

#[test]
fn aircraft_come_home_elsewhere_only_when_their_deck_sank_or_went_out_in_the_pulse() {
    // Decisions 7 to 10 of 0059. Only a group aboard a ship that sank or lost
    // her flight deck in the afternoon is ever affected; never a group at
    // Henderson Field, and never in a pulse with no air combat step. (That
    // the model in `work_the_run` expects each event whole, with its place,
    // receiving group, causes and chance, is checked for every run there.)
    let ship_of = |group: AirGroupId| match spec_of_any(group.raw()) {
        Some(Place::Ship(ship)) => Some(ShipId::new(ship)),
        _ => None,
    };
    let mut homecomings = 0;
    for run in batch() {
        homecomings += common::no_deck::assert_only_for_lost_decks(&run.events, ship_of);
    }
    assert!(homecomings > 500, "{homecomings}");
}

/// The place of a group of the first table, or of one formed by a landing,
/// which is aboard a ship or at a base but never the source of another.
fn spec_of_any(group: u32) -> Option<Place> {
    GROUPS.iter().find(|s| s.id == group).map(|s| s.place)
}

#[test]
fn every_groups_aircraft_at_the_end_are_those_at_the_start_less_its_recorded_losses() {
    for run in batch() {
        for spec in &GROUPS {
            let mut left = spec.ready;
            for event in &run.events {
                let lost = match event.kind {
                    EventKind::AircraftLost {
                        air_group,
                        aircraft,
                        remaining,
                    } if air_group.raw() == spec.id => {
                        left = left.checked_sub(aircraft).expect("none goes below zero");
                        assert_eq!(left, remaining, "seed {}", run.seed);
                        continue;
                    }
                    // A sunk ship takes what was aboard; the aircraft in the
                    // air are kept until they come home (decision 10 of 0059).
                    EventKind::AirGroupLost {
                        air_group,
                        aircraft,
                        ..
                    } if air_group.raw() == spec.id => aircraft,
                    // What ran out of fuel on the way home from a sortie
                    // beyond its radius (decision 3 of 0059).
                    EventKind::AircraftDitched {
                        air_group,
                        ditched,
                        ready,
                        ..
                    } if air_group.raw() == spec.id => {
                        left = left.checked_sub(ditched).expect("none goes below zero");
                        assert_eq!(left, ready, "seed {}", run.seed);
                        continue;
                    }
                    // What was destroyed on the ground at Henderson Field.
                    EventKind::AircraftDestroyedOnGround {
                        air_group,
                        aircraft,
                        remaining,
                        ..
                    } if air_group.raw() == spec.id => {
                        left = left.checked_sub(aircraft).expect("none goes below zero");
                        assert_eq!(left, remaining, "seed {}", run.seed);
                        continue;
                    }
                    // What came down in the sea for want of a deck.
                    EventKind::AircraftDitchedNoDeck {
                        air_group,
                        aircraft,
                        ready,
                        ..
                    } if air_group.raw() == spec.id => {
                        left = left.checked_sub(aircraft).expect("none goes below zero");
                        assert_eq!(left, ready, "seed {}", run.seed);
                        continue;
                    }
                    // What landed elsewhere leaves the group, and what landed
                    // elsewhere to it joins it.
                    EventKind::AircraftLandedElsewhere { from, aircraft, .. }
                        if from.raw() == spec.id =>
                    {
                        left = left.checked_sub(aircraft).expect("none goes below zero");
                        continue;
                    }
                    EventKind::AircraftLandedElsewhere { to, aircraft, .. }
                        if to.raw() == spec.id =>
                    {
                        left += aircraft;
                        continue;
                    }
                    _ => continue,
                };
                left = left
                    .checked_sub(lost)
                    .expect("a sunk ship takes no more than is left");
            }
            assert_eq!(
                run.ready[&spec.id], left,
                "seed {}, group {}",
                run.seed, spec.id
            );
            assert!(run.ready[&spec.id] <= spec.ready + landed_to(run, spec.id));
        }
        // A group aboard a ship that sinks has none.
        for spec in &GROUPS {
            if let Place::Ship(ship) = spec.place {
                if run.ships[&ship].sunk {
                    assert_eq!(run.ready[&spec.id], 0, "seed {}", run.seed);
                }
            }
        }
    }
}

#[test]
fn no_ship_has_negative_fuel_and_no_damage_track_is_above_100() {
    for run in batch() {
        for (id, last) in &run.ships {
            assert!(last.fuel >= Fixed::ZERO, "seed {}, ship {id}", run.seed);
            for damage in [last.flotation, last.fire] {
                assert!(damage >= Fixed::ZERO && damage <= Fixed::from_int(100));
            }
            // A ship is sunk exactly when its flotation reached 100.
            assert_eq!(last.sunk, last.flotation == Fixed::from_int(100));
        }
        for event in &run.events {
            match event.kind {
                EventKind::ShipDamaged { value_after, .. } => {
                    assert!(value_after <= Fixed::from_int(100));
                }
                EventKind::FuelBurned { remaining_t, .. } => {
                    assert!(remaining_t >= Fixed::ZERO);
                }
                _ => {}
            }
        }
    }
}

#[test]
fn every_damage_event_is_caused_by_an_attack_every_attack_by_a_launch_and_every_launch_by_a_report()
{
    let mut damage_events = 0;
    for run in batch() {
        let by_id: BTreeMap<EventId, &Event> = run.events.iter().map(|e| (e.id, e)).collect();
        let cause_of = |event: &Event| -> &Event {
            assert_eq!(event.causes.len(), 1, "seed {}", run.seed);
            let Cause::Event(id) = event.causes[0] else {
                panic!("seed {}: an event cites an event", run.seed);
            };
            by_id[&id]
        };
        for event in &run.events {
            match event.kind {
                EventKind::ShipDamaged { .. } | EventKind::SystemHit { .. } => {
                    assert!(matches!(cause_of(event).kind, EventKind::AirAttack { .. }));
                    damage_events += 1;
                }
                EventKind::ShipSunk { .. } => {
                    assert!(matches!(
                        cause_of(event).kind,
                        EventKind::ShipDamaged { .. }
                    ));
                }
                EventKind::AirGroupLost { .. } => {
                    assert!(matches!(cause_of(event).kind, EventKind::ShipSunk { .. }));
                }
                EventKind::AirAttack { air_group, .. } => {
                    let Cause::Event(launch) = event.causes[0] else {
                        panic!("an attack cites its launch");
                    };
                    assert!(matches!(
                        by_id[&launch].kind,
                        EventKind::StrikeLaunched { air_group: launched, .. } if launched == air_group
                    ));
                    for cause in &event.causes[1..] {
                        let Cause::Event(loss) = cause else {
                            panic!("an attack cites events");
                        };
                        assert!(matches!(
                            by_id[loss].kind,
                            EventKind::AircraftLost { air_group: lost, .. } if lost == air_group
                        ));
                    }
                }
                EventKind::StrikeLaunched { air_group, .. } => {
                    // The mission order, the target order of groups 12 and
                    // 15, and the report, last.
                    let Some(&Cause::Event(report)) = event.causes.last() else {
                        panic!("a launch cites a report");
                    };
                    assert!(matches!(
                        by_id[&report].kind,
                        EventKind::ReportReceived { side, .. } if side == spec_of(air_group.raw()).side
                    ));
                }
                EventKind::ReportReceived { side, .. } => {
                    // A report is caused by a search sighting or an attack,
                    // both of its own side's groups.
                    match cause_of(event).kind {
                        EventKind::SearchAttempt {
                            air_group, sighted, ..
                        } => assert!(spec_of(air_group.raw()).side == side && sighted),
                        EventKind::AirAttack { air_group, .. } => {
                            assert_eq!(spec_of(air_group.raw()).side, side);
                        }
                        ref other => panic!("seed {}: a report cites {other}", run.seed),
                    }
                }
                _ => {}
            }
        }
    }
    assert!(damage_events > 3000, "{damage_events} damage events");
}

#[test]
fn a_replay_reproduces_every_daily_hash() {
    let base = load();
    let mut all_hashes: Vec<Vec<u64>> = Vec::new();
    for seed in [1u64, 2, 77, 1942] {
        let hashes: Vec<Vec<u64>> = (0..2)
            .map(|_| {
                let mut run = base.with_seed(seed).expect("a seed can be set");
                run.run_to_end().expect("the run resolves");
                run.world.day_hashes().to_vec()
            })
            .collect();
        assert_eq!(hashes[0].len(), 1);
        assert_eq!(hashes[0], hashes[1], "seed {seed}");
        all_hashes.push(hashes[0].clone());
    }
    // The seed matters.
    assert!(all_hashes.windows(2).any(|pair| pair[0] != pair[1]));
    // Every run of the batch has the hash of a fresh run of its seed.
    for run in batch().iter().step_by(25) {
        let mut again = base.with_seed(run.seed).expect("a seed can be set");
        again.run_to_end().expect("the run resolves");
        assert_eq!(run.hashes, again.world.day_hashes(), "seed {}", run.seed);
        // The claims from the air are not in the batch's events (see `digest`).
        let events: Vec<Event> = again
            .world
            .events()
            .iter()
            .filter(|e| {
                !matches!(
                    e.kind,
                    sim_core::events::EventKind::ClaimReceived { from_air: true, .. }
                )
            })
            .cloned()
            .collect();
        assert_eq!(run.events, events, "seed {}", run.seed);
    }
    assert_eq!(batch()[76].seed, 77);
    // The file's own seed gives the same hashes as a seeded copy.
    let mut own = load();
    own.run_to_end().expect("the run resolves");
    let mut copy = base.with_seed(1942).expect("a seed can be set");
    copy.run_to_end().expect("the run resolves");
    assert_eq!(own.world.day_hashes(), copy.world.day_hashes());
}

#[test]
fn every_afternoon_of_every_run_is_worked_by_the_rule_from_its_start() {
    // `work_the_run` asserts every event of the afternoon, search to damage
    // and movement, and the end state, for every seed.
    assert_eq!(worked().len(), 1000);
    for (run, work) in batch().iter().zip(worked()) {
        assert_eq!(run.seed, work.seed);
    }
}

// ---------------------------------------------------------------------
// The summary, for the architect. It asserts nothing.
// ---------------------------------------------------------------------

#[test]
fn print_the_summary() {
    let runs = count(batch().len());
    let mut sunk: BTreeMap<u32, i64> = BTreeMap::new();
    let mut attacks = [0i64; 2];
    let mut both = 0i64;
    let mut lost_by_side = [Vec::new(), Vec::new()];
    let mut hits_on_enterprise = 0i64;
    let mut hits_on_ryujo = 0i64;
    let mut other_sunk = 0i64;
    for (run, work) in batch().iter().zip(worked()) {
        for ship in &work.sunk {
            if matches!(ship, 1 | 11 | 19 | 20 | 33) {
                *sunk.entry(*ship).or_default() += 1;
            } else {
                other_sunk += 1;
            }
        }
        for (slot, side) in [Side::Allied, Side::Japanese].into_iter().enumerate() {
            let attacked = work.attacks.iter().any(|a| spec_of(a.group).side == side);
            attacks[slot] += i64::from(attacked);
            // The aircraft a side lost: what it started with less what its
            // groups have at the end, those formed by landings included.
            let started: u32 = GROUPS
                .iter()
                .filter(|s| s.side == side)
                .map(|s| s.ready)
                .sum();
            let formed: u32 = run
                .ready
                .iter()
                .filter(|(id, _)| **id > 20)
                .filter(|(id, _)| {
                    run.events.iter().any(|e| {
                        matches!(e.kind, EventKind::AircraftLandedElsewhere { from, to, .. }
                            if to.raw() == **id && spec_of(from.raw()).side == side)
                    })
                })
                .map(|(_, ready)| *ready)
                .sum();
            let at_the_end: u32 = GROUPS
                .iter()
                .filter(|s| s.side == side)
                .map(|s| run.ready[&s.id])
                .sum();
            let lost = started - (at_the_end + formed);
            lost_by_side[slot].push(i64::from(lost));
        }
        both += i64::from(
            work.attacks
                .iter()
                .any(|a| spec_of(a.group).side == Side::Allied)
                && work
                    .attacks
                    .iter()
                    .any(|a| spec_of(a.group).side == Side::Japanese),
        );
        for attack in &work.attacks {
            match attack.ship {
                1 => hits_on_enterprise += i64::from(attack.hits),
                33 => hits_on_ryujo += i64::from(attack.hits),
                _ => {}
            }
        }
    }
    let mean = |values: &[i64]| -> String {
        let total: i64 = values.iter().sum();
        format!(
            "{}.{:02}",
            total / runs,
            (total * 100 / runs).rem_euclid(100)
        )
    };
    eprintln!("SUMMARY over seeds 1 to {SEEDS}");
    for (ship, name) in [
        (1, "Enterprise"),
        (11, "Saratoga"),
        (19, "Shokaku"),
        (20, "Zuikaku"),
        (33, "Ryujo"),
    ] {
        eprintln!(
            "  {name} ends sunk in {} of {runs} runs",
            sunk.get(&ship).copied().unwrap_or(0)
        );
    }
    eprintln!("  other ships sunk: {other_sunk}");
    eprintln!(
        "  runs in which the Allied side attacks {}, the Japanese side {}, both {both}",
        attacks[0], attacks[1]
    );
    for (slot, name) in ["Allied", "Japanese"].into_iter().enumerate() {
        let values = &lost_by_side[slot];
        eprintln!(
            "  {name} aircraft lost: mean {}, range {} to {}",
            mean(values),
            values.iter().min().expect("runs"),
            values.iter().max().expect("runs")
        );
    }
    eprintln!(
        "  hits on Enterprise: mean {}.{:02} a run; hits on Ryujo: mean {}.{:02} a run",
        hits_on_enterprise / runs,
        (hits_on_enterprise * 100 / runs) % 100,
        hits_on_ryujo / runs,
        (hits_on_ryujo * 100 / runs) % 100
    );
    eprintln!("  what happened, for comparison only:");
    eprintln!("    Saratoga's 30 dive bombers and 8 torpedo planes sank Ryujo for no loss or one.");
    eprintln!("    The Japanese first wave, 27 dive bombers and 10 fighters, hit Enterprise with");
    eprintln!("    three bombs and lost 24 aircraft; a second wave of the same size did not find");
    eprintln!("    the American force. No American ship was sunk.");
}
