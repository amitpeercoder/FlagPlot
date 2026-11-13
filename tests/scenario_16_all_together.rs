//! Scenario 16: every rule of step 2 acting together, in one night and one
//! day (`docs/step2-spec.md`, "Tests while figures move", 8).
//!
//! It is the complexity review of step 2. Each rule below has its own
//! scenario and its own hand calculation; this one checks that they give the
//! same numbers when they act in one day, in the order the day resolves them,
//! and that where two of them read or write the same thing the later one reads
//! what the earlier one wrote. The decisions are 0058 (ships charged
//! straight-line miles), 0059 (reach, the fuel roll, a lost deck, landing
//! elsewhere, ditching for want of a deck, the patrol pilot), 0063 (target
//! lists, the afternoon base strike), 0064 (experience), 0067 (the base count
//! at the day's end) and 0068 (a force not ready fires part of its guns).
//!
//! # The hand calculation (the architect's; nobody else changes a number)
//!
//! ## The figures
//!
//! The frozen step 1 figures (`data/reference/step1`) with these changed.
//! Each change either switches a rule on that the frozen value switches off,
//! or takes a chance to 0 or 1 so that the day is determined.
//!
//! | File | Figure | Frozen | Here | Why |
//! | --- | --- | --- | --- | --- |
//! | surface_combat | experience_gun_strength, _torpedo_, _sighting_ | 0 | 0.4 | 0064 on, as scenario 14a |
//! | surface_combat | unready_gun_share | 1 | 0.5 | 0068 on, as scenario 15a |
//! | surface_combat | system_hit_chance_per_point | 0.06 | 0.1 | a bomb of 10 points knocks a system with chance 1 (12c, case D) |
//! | air_combat | bomb_hit_chance | 0.2 | 1 | every bomb hits |
//! | air_combat | bomb_system_hit: flight_deck | 0.5 | 1 | every knock is on the flight deck |
//! | air_combat | bomb_system_hit: propulsion, fire_control, main_battery | 0.2, 0.1, 0.2 | 0 | the same |
//! | air_combat | crew_recovered_beside_own_force | 1 | 0.95 | a pilot can be lost |
//! | air_combat | crew_recovered_open_sea | 1 | 0.5 | not reached; set so that reaching it would show |
//! | air_combat | crew_recovered_over_own_base | 1 | 0.6 | the same |
//! | air_upkeep | wear_base_chance, wear_chance_per_100_nm | 0.05, 0.02 | 0 | the afternoon has every aircraft the morning had |
//! | search | position_error_chance, count_over_chance, count_under_chance | 0.3, 0.2, 0.2 | 0 | a report is the truth |
//! | search | every row of kind confusion | spread | the kind itself at 1 | a cruiser is reported as a cruiser |
//!
//! Kept at the frozen value and read below: extended_reach_share 0.25,
//! ditch_chance_per_radius_beyond 2, crew_recovered_short_of_home 0.7,
//! fighter_hit_chance 0.3, strike_contact_max_age_pulses 0, the base hit
//! shares 0.3 (aircraft) and 0.3 (supply), base_supply_destroyed_per_hit_t 50,
//! the night sighting chance 0.5, the factor 0.5 for a force not alert, the
//! night chances to hit 0.03 (guns) and 0.04 (torpedoes), the factor 1.5
//! against a surprised force and 2 against a lit one.
//!
//! ## Where everything is
//!
//! Air distance is 40 x sqrt(dq^2 + dr^2 + dq x dr), rounded to whole miles.
//!
//! | From | To | dq, dr | Sum | Miles |
//! | --- | --- | --- | --- | --- |
//! | Carrier A (2,0) | Carrier J at 06:00 (11,0) | 9, 0 | 81 | 360 |
//! | Carrier A (2,0) | Carrier J at noon (9,0) | 7, 0 | 49 | 280 |
//! | Carrier A (2,0) | The field (9,-6) | 7, -6 | 43 | 262 |
//! | Carrier A (2,0) | The night's fight (2,-4) | 0, -4 | 16 | 160 |
//! | Carrier J at 06:00 (11,0) | The field (9,-6) | -2, -6 | 52 | 288 |
//! | Carrier J at noon (9,0) | The field (9,-6) | 0, -6 | 36 | 240 |
//! | Carrier J at noon (9,0) | The night's fight (2,-4) | -7, -4 | 93 | 386 |
//!
//! Radius of the dive bombers and fighters: 250. Extended reach:
//! 250 x 1.25 = 312.5, cut to 312. The scouts have no strike radius and
//! search to 300.
//!
//! ## 1. Steaming (0058)
//!
//! Carrier J, (14,3) to (9,0) at 16 knots: 280 miles in a straight line, eight
//! steps off the grid's axes. The first three steps are charged the miles
//! each brings her nearer, 30, 27 and 23; the other five are along an axis,
//! 40 each: 30 + 27 + 23 + 5 x 40 = 280.
//!
//! - Night, 12 hours: 192 miles. Five steps done (160) and 32 into the sixth.
//!   The breakdown's steps are 30, 27, 23, 40, 40, 40. She is in (11,0) at
//!   06:00 and burns 64 t.
//! - Morning: the 88 miles left, in 5.5 hours. She arrives at (9,0) at 11:30.
//!
//! The Japanese cruisers, (7,-4) west along the row at 16 knots, are charged
//! 40 a step. They enter (2,-4) after 180 miles, at 05:15, and the fight
//! begins there.
//!
//! ## 2. The night fight (0064 and 0068 together)
//!
//! Force 3 (Allied): three cruisers of experience 1, stopped, normal
//! readiness, its side holding no contact. So it is not alert, and not ready.
//! Force 4 (Japanese): three cruisers of experience 3, at battle readiness.
//! So it is alert, and ready.
//!
//! Experience factor, 1 + 0.4 x (experience - 2): 0.6 for the Allied ships,
//! 1.4 for the Japanese.
//!
//! **Sighting, round 1.** Base x alert x experience, capped at 1:
//!
//! - Force 3 looks: 0.5 x 0.5 x 0.6 = 0.15.
//! - Force 4 looks: 0.5 x 1 x 1.4 = 0.7.
//!
//! **Gun shots.** A full salvo is 8 guns x 3 a minute x 5 minutes = 120.
//!
//! - A Japanese ship, ready: 120 x her main battery factor. No
//!   `unready_gun_share` in her breakdown.
//! - An Allied ship, not ready: 120 x her main battery factor x 0.5, so 60
//!   with her battery working and 30 with it degraded, in every round of the
//!   fight. Force 3 becomes alert in round 1 and still fires the share to the
//!   end: readiness is fixed when the fight begins. Her breakdown names
//!   `unready_gun_share`, after the main battery factor.
//!
//! **Chance to hit.** Base x surprised x lit x fire control x experience, the
//! experience factor last. With fire control working:
//!
//! | Firer | Target as it is | Guns | Torpedoes |
//! | --- | --- | --- | --- |
//! | Japanese | neither | 0.03 x 1.4 = 0.042 | 0.04 x 1.4 = 0.056 |
//! | Japanese | surprised | 0.03 x 1.5 x 1.4 = 0.063 | 0.04 x 1.5 x 1.4 = 0.084 |
//! | Japanese | lit | 0.03 x 2 x 1.4 = 0.084 | |
//! | Allied | neither | 0.03 x 0.6 = 0.018 | 0.04 x 0.6 = 0.024 |
//! | Allied | surprised | 0.03 x 1.5 x 0.6 = 0.027 | 0.04 x 1.5 x 0.6 = 0.036 |
//! | Allied | lit | 0.03 x 2 x 0.6 = 0.036 | |
//!
//! A firer whose fire control is degraded has half the chance of her row
//! (seed 1942, round 2: an Allied ship at 60 shots x 0.009).
//!
//! The two rules never touch the same number. The share changes the shots and
//! experience changes the chance; they meet only in the expected hits. An
//! Allied ship with everything working, against a force neither surprised nor
//! lit: 60 x 0.018 = 1.08 expected, where a Japanese ship has
//! 120 x 0.042 = 5.04. Torpedoes read experience and never the share: 8 shots.
//!
//! **After the fight** each side reports the other: cruisers, at (2,-4), the
//! true count of ships afloat. That is contact 1 for each side.
//!
//! ## 3. The morning: the hold (0063 on a contact from 0068's fight)
//!
//! - Searches. Carrier J is 360 miles from carrier A, beyond the 300 of both
//!   sides' scouts. Nothing is sighted.
//! - The Allied dive bomber (group 2, one aircraft). The one contact its side
//!   holds is the cruisers at (2,-4): age 0, 160 miles, within reach. Its
//!   target list is "carrier" alone. It does not launch. With the target list
//!   order taken out of the file it launches one aircraft at contact 1, at
//!   160 miles: the list is what holds it.
//! - The Japanese dive bombers (group 5). Their side's contact 1 is 461 miles
//!   from (11,0): beyond 312. They do not launch.
//! - The Japanese attack unit (group 6), ordered to strike the field on
//!   normal reach. The field is 288 miles from (11,0): beyond 250. It does not
//!   launch.
//!
//! No strike is launched by either side in the morning.
//!
//! ## 4. The afternoon
//!
//! Positions are read as the pulse begins: carrier J is at (9,0).
//!
//! **Search.** Five scouts on an arc 20 degrees wide, the target at 280:
//! 57.295779 x 5 x 20 / (280 x 20) = 5729.5779 / 5600 = 1.0231, capped at 1.
//! Each side sights the other's carrier: a carrier, one ship, at its true
//! hex. Contact 2 for each side.
//!
//! **Launches** (0059's reach read by 0063's choice, in one function):
//!
//! | Group | Launches | Why |
//! | --- | --- | --- |
//! | 2, Allied dive bomber | 1 at contact 2, 280 miles | A carrier, on its list; 250 < 280 <= 312, and it is on extended reach. On normal reach it does not launch |
//! | 5, Japanese dive bombers | 6 at contact 2, 280 miles | The same reach |
//! | 6, Japanese attack unit | 4 at the field, 240 miles | 240 <= 250. Its one base strike of the day, in the afternoon (0063, decision 7) |
//! | 7, Japanese fighters | 12 escorts with group 5 | Patrol share 0; group 5's raid has more bombers than group 6's (6 against 4) and is within the fighters' reach of 312 |
//!
//! Carrier A's deck is working when the launches are decided, so her strike
//! goes though her deck is lost later in the pulse.
//!
//! **Over carrier A.** Her patrol is 12 fighters and the escorts are 12, so
//! every patrol shot is at an escort and none at a bomber: 12 shots x 0.3 =
//! 3.6 expected each way. There are no anti-aircraft guns. A patrol fighter
//! shot down over its own force loses its pilot with chance 1 - 0.95 = 0.05
//! (0059, the patrol pilot). An escort shot down loses its pilot.
//!
//! **Bombs**, at chance 1:
//!
//! - Group 5 on carrier A: 6 shots, 6 hits, 10 points each, flotation 60 (she
//!   sinks at 100, so she cannot sink). Each hit knocks a system with chance
//!   10 x 0.1 = 1, and the system is the flight deck: working to degraded,
//!   degraded to out, then four hits with no change. **The deck is lost.**
//! - Group 2 on carrier J: 1 shot, 1 hit, flotation 10, flight deck working to
//!   degraded. Her deck is not lost, and her aircraft come home to it.
//! - Group 6 on the field: 4 shots, 4 hits. Each is on aircraft with share
//!   0.3, on supply with 0.3 and on nothing with 0.4. No aircraft are on the
//!   ground: the landing comes later in the pulse, so the raid never meets the
//!   aircraft that land. A hit on supply destroys 50 t. With S hits on supply
//!   the stock is 500 - 50 x S.
//!
//! **The fuel roll** (0059), for a sortie beyond the radius: 280 is 30 beyond
//! 250, a share of 30 / 250 = 0.12, and the chance is 0.12 x 2 = 0.24 an
//! aircraft. A crew is recovered with chance 0.7.
//!
//! - Group 2: 1 aircraft x 0.24 = 0.24 expected.
//! - Group 5: 6 x 0.24 = 1.44 expected.
//! - Group 7: the escorts not shot down, each at 0.24.
//! - Group 6: no roll. 240 is not beyond 250.
//!
//! **Coming home to no deck** (0059), after the fuel roll and before wear.
//! Carrier A has no other carrier in her hex.
//!
//! - Group 1, the 5 scouts, which searched this pulse: the field is 262 miles
//!   off, within their 300. All 5 land there and form group 8, with 5 pilots.
//!   Group 1 is left with 0 aircraft and 0 pilots.
//! - Group 2, its aircraft if the fuel roll spared it: the field at 262 is
//!   beyond 250. It comes down in the sea at (2,0), beside ships of its own
//!   side: the crew is recovered with chance 0.95.
//! - Group 3, the patrol fighters not shot down: on patrol they are in the
//!   air (0059, decision 7). The field is beyond 250. They come down in the
//!   sea beside their own ships, each crew recovered with chance 0.95.
//!
//! The Japanese groups land aboard carrier J, with no event.
//!
//! **Supply.** The sortie of the scouts went with them to group 8, so the
//! field pays for it: 5 aircraft x 2 t = 10 t. A carrier would have paid
//! nothing.
//!
//! ## 5. The day's end (0067 reading what 0059 and 0063 left)
//!
//! The field: 500 - 10 - 50 x S tonnes, which is above zero for any S up to
//! 4 (290 at the least), and 5 aircraft ready, the landed scouts. So it is
//! **open**, in every run. Without the landing it would hold no aircraft and
//! be closed.
//!
//! The result. The Allied side has the field open for 1 of 1 days: 5 points.
//! The Japanese side has carrier A crippled, her flight deck out: 10 points,
//! and 0 for the field closed. Aircraft destroyed are 0.5 each: the aircraft
//! shot down and those ditched for fuel or for want of a deck. The 5 scouts
//! that landed are not destroyed.
//!
//! ## 6. The order within the afternoon
//!
//! By event number: the searches and reports; the launches and the escorts;
//! the fighters' shots and the losses, with the patrol pilot's draw; the
//! bombs; the damage, with the deck going out; the fuel roll; the homecoming
//! to no deck; wear; the supply paid; and, last of the day, the field's
//! count. Each reads what the one before wrote: the fuel roll takes its
//! ditched aircraft off before the homecoming counts who is left, and the
//! homecoming moves the sortie before supply is paid.
//!
//! ## 7. Seed 1942, pinned
//!
//! Not worked by hand: what the draws gave on the scenario's own seed, pinned
//! so that a change shows. It is remade only when a rule changes.
//!
//! - Night: both forces sight in round 1; force 3 breaks off in round 2; no
//!   ship is sunk.
//! - Over carrier A: 5 patrol fighters shot down, no pilot lost; 2 escorts
//!   shot down.
//! - Fuel: group 2, none ditched. Group 5, 3 ditched, 2 crews recovered, 1
//!   pilot lost, 3 ready left. Group 7, 10 rolled, 6 ditched, 5 crews
//!   recovered, 1 pilot lost, 4 ready left.
//! - No deck: group 1 lands 5 at the field. Group 2, 1 into the sea, the crew
//!   recovered. Group 3, 7 into the sea, 6 crews recovered, 1 pilot lost.
//! - The raid on the field: 1 hit on aircraft, 0 on supply, 3 on nothing.
//! - The field at the day's end: 490 t, 5 aircraft ready, open.
//! - Aircraft destroyed: 11 Japanese (2 + 3 + 6) and 13 Allied (5 + 1 + 7).
//! - Result: Allied 5.5 + 5 = 10.5, Japanese 6.5 + 10 = 16.5, margin 6, a
//!   minor Japanese victory. 166 events.
//!
//! ## 8. Properties, over seeds 1 to 1,000
//!
//! In every run, on the figures above:
//!
//! - The night fight takes place. Sighting chances in round 1 are 0.15 and
//!   0.7. Every gun salvo of a force 3 ship in the night's fight names
//!   `unready_gun_share` 0.5 and has 60 or 30 shots; no salvo of a force 4
//!   ship names it; every torpedo salvo has no share. Every salvo's chance is
//!   a row of the table in 2, or half of one.
//! - No strike is launched in the morning.
//! - In the afternoon the four launches are as in 4, carrier A's flight deck
//!   goes out, carrier J's is degraded and not out, and no carrier sinks.
//! - No patrol shot is at a bomber.
//! - The 5 scouts land at the field; the field pays 10 t; at the day's end
//!   it holds 500 - 10 - 50 x S tonnes and 5 ready aircraft, and is open.
//! - Each group's aircraft are all accounted for: those sent are shot down,
//!   ditched for fuel, landed, in the sea, or home aboard. No group has fewer
//!   than 0 pilots or loses more pilots than aircraft.
//! - No event comes before its cause, and a replay gives the same hash.
//! - The enemy's view of a ditching or a landing elsewhere is cut as
//!   `side_events.rs` has it.
//!
//! Across the runs, each within four standard deviations of its mean:
//!
//! - Patrol fighters shot down, and escorts shot down: 12 at 0.3 each, mean
//!   3.6 a run.
//! - Aircraft ditched for fuel: 0.24 of those rolled. Crews recovered: 0.7 of
//!   those ditched.
//! - Crews recovered beside their own force: 0.95 of the aircraft in the sea.
//! - Hits on the field's supply: 0.3 of 4, mean 1.2 a run.
//!
//! A surface fight by day happens only in a run where a force lies stopped in
//! the fight's hex with a ship's propulsion out (the stragglers rule). The
//! test counts those runs and asserts nothing more of them.
//!
//! ## 9. Two days: the raid meets the aircraft that landed
//!
//! In one day the raid on the field comes before the landing. The same file
//! with `days = 2` brings them together, on seeds 1 to 200:
//!
//! - Group 6 launches at the field once on day 2, and in the morning: carrier
//!   J is still at (9,0), 240 miles off, from the day's first pulse.
//! - Group 8's 5 scouts are on the ground when the raid arrives, and the
//!   raid's hits on aircraft fall on them.
//! - The field's count at the end of day 2 is the ready aircraft group 8 has
//!   left in the state, and the tonnes the state holds; it is open only if
//!   both are above zero.
//! - No aircraft takes off from carrier A on day 2: her deck is out.
//!
//! ## 10. On the frozen figures, nothing altered
//!
//! No breakdown names `experience_factor` or `unready_gun_share`, and every
//! Allied gun salvo with a working battery has 120 shots.
