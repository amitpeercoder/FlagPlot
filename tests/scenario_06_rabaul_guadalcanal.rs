//! Scenario 6: Rabaul against Guadalcanal in the air, 21 August to 17
//! September 1942.
//!
//! Stage 6e, which closes scenario 6 (decision 0042). The data is
//! `data/scenarios/06_rabaul_guadalcanal.toml`, run with seeds 1 to 1000.
//! The air groups, aircraft types, arrivals and airfields are historical.
//! The combat, wear, fatigue and supply figures, each base's supply and
//! repairs a day, and the replacement rates are placeholders.
//!
//! It is the first scenario in which raids on a base (decision 0034), wear
//! and repair (0035), fatigue and rest (0036), pilots and replacements
//! (0038), base supply (0039), straight-line air distance (0040) and
//! arrivals (0041) all run together.
//!
//! # Hand calculation
//!
//! ## The setting
//!
//! Henderson Field, base 1, Allied, is in (2,-4): 4,000 t of supply, 8
//! anti-aircraft shots, 3 repairs a day. Rabaul, base 2, Japanese, is in
//! (-5,-13): 10,000 t, no guns, 4 repairs a day.
//!
//! The air distance between them is 40 x the square root of (7 x 7 + 9 x 9
//! + 7 x 9) = 40 x root 193 = 555.7, so 556 nm. A sortie from one to the
//! other is 1,112 nm.
//!
//! | Type | Radius | Weapon | Pilot armour | Protected tanks |
//! | ---- | ------ | ------ | ------------ | --------------- |
//! | F4F-4 | 105 nm | none | yes | yes |
//! | SBD-3 | 200 nm | a bomb | yes | yes |
//! | A6M2 | 556 nm | none | no | no |
//! | G4M1 | 611 nm | a bomb | no | no |
//!
//! So the G4M1s and the A6M2s can fly from Rabaul to Henderson Field, the
//! A6M2s with nothing to spare, and nothing at Henderson Field can reach
//! Rabaul.
//!
//! | Group | At | Type | Starts with | Mission | Rests above |
//! | ----- | -- | ---- | ----------- | ------- | ----------- |
//! | 1, the land attack bombers | Rabaul | G4M1 | 41 | base strike on Henderson Field | 45 |
//! | 2, Tainan Air Group | Rabaul | A6M2 | 24 | combat air patrol, share 0 | 45 |
//! | 3, 6th Air Group | Rabaul | A6M2 | none | combat air patrol, share 0 | 45 |
//! | 10, VMF-223 | Henderson | F4F-4 | 19 | combat air patrol, share 1 | 40 |
//! | 11, VMSB-232 | Henderson | SBD-3 | 12 | naval strike | never |
//! | 12, Flight 300 | Henderson | SBD-3 | none | naval strike | never |
//! | 13, VMF-224 | Henderson | F4F-4 | none | combat air patrol, share 1 | 40 |
//! | 14, VMSB-231 | Henderson | SBD-3 | none | naval strike | never |
//! | 15, VF-5 | Henderson | F4F-4 | none | combat air patrol, share 1 | 40 |
//! | 16, fighters from Hornet and Wasp | Henderson | F4F-4 | none | combat air patrol, share 1 | 40 |
//! | 17, VS-3 | Henderson | SBD-3 | none | naval strike | never |
//!
//! Every group has a pilot for each aircraft it starts with, and none
//! damaged. A group that starts with none has a strength of none.
//!
//! | Day | Arrival | Aircraft and pilots |
//! | --- | ------- | ------------------- |
//! | 4 | Group 12 | 11 |
//! | 10 | Group 13 | 19 |
//! | 10 | Group 14 | 12 |
//! | 11 | Group 3 | 13 |
//! | 22 | Group 15 | 24 |
//! | 24 | Group 16 | 18 |
//! | 24 | Group 17 | 12 |
//!
//! Rabaul receives two G4M1 and two A6M2 a day, each with a pilot.
//! Henderson Field receives nothing but the arrivals.
//!
//! ## The figures for one sortie
//!
//! | | A patrol | Rabaul to Henderson Field and back, 1,112 nm |
//! | - | -------- | -------------------------------------------- |
//! | Chance an aircraft is worn | 0.05 | 0.05 + 0.02 x 11.12 = 0.2724 |
//! | Fatigue the group gains | 3 | 3 + 2 x 11.12 = 25.24 |
//! | Supply for each aircraft sent | 0.5 t | 0.5 + 0.25 x 11.12 = 3.28 t |
//!
//! A worn aircraft is written off with chance 0.2. A group that did not fly
//! in a daylight pulse loses 5 fatigue. An aircraft at a base costs 0.2 t a
//! day and a repair 1 t.
//!
//! A fighter's shot hits an A6M2 or a G4M1 with chance 0.3, and an F4F-4
//! with 0.3 x 0.75 x 0.75 = 0.16875. A gun's shot hits a G4M1 with 0.05. A
//! bomb hits with 0.2, and lands on aircraft on the ground with 0.3, on
//! supply with 0.3, destroying 50 t, and on nothing with 0.4.
//!
//! A group with R ready aircraft, P pilots and fatigue f flies the smaller
//! of R and P, times (100 - f) / 100, rounded down, and nothing if it is
//! stood down or its base has no supply at the start of the pulse. Write
//! that fly(group).
//!
//! ## A day
//!
//! **Night, at the start of the day,** in this order:
//!
//! 1. Repair. Each base repairs damaged aircraft, the lowest-numbered
//!    group's first, up to its repairs a day and while it has 1 t for each,
//!    and pays 1 t for each.
//! 2. Arrivals due that day.
//! 3. Replacements, from day 2: at Rabaul up to two G4M1 and two pilots to
//!    group 1, up to its strength in each; up to two A6M2 and two pilots to
//!    group 2 and then group 3, up to each one's strength.
//! 4. The daily cost: 0.2 t for each aircraft at the base, ready or
//!    damaged.
//! 5. The staff stands down each group whose fatigue is above its
//!    threshold and returns each that is not.
//!
//! **Morning.** On the state at the start of the pulse:
//!
//! - Group 1 launches fly(1) bombers at Henderson Field, if that is at
//!   least one.
//! - Groups 2 and 3 keep nothing over Rabaul. If a raid launches, each sends
//!   every fighter it can fly as escort. If none does, they do not fly.
//! - Each Allied fighter group's patrol is all it can fly. The dive bombers
//!   do not fly: no contact is ever held.
//! - The raid is one raid and meets the whole patrol over Henderson Field,
//!   P fighters in all, with E escorts and B bombers:
//!   - the escorts fire E shots at the patrol, at 0.16875;
//!   - the patrol fires the smaller of P and E shots at the escorts and the
//!     rest of P at the bombers, at 0.3;
//!   - the guns fire 8 shots at the bombers left, at 0.05;
//!   - each bomber left drops a bomb at 0.2, and each hit lands by one
//!     draw.
//! - Losses to the patrol come off the lowest-numbered Allied fighter group
//!   first, and to the escorts off group 2 before group 3.
//! - The aircraft on the ground at Henderson Field are every ready aircraft
//!   there that is not flying and every damaged one: all the dive bombers,
//!   and whatever a fighter group leaves behind. Those destroyed are taken
//!   from the lowest-numbered group first, its damaged before its ready.
//! - A pilot is lost with every bomber and every escort shot down. No
//!   Allied pilot is ever lost: an Allied aircraft is lost only over its
//!   own field or on the ground.
//! - Then wear: each group that flew rolls for the aircraft that came back,
//!   at 0.2724 for the raid and the escort and 0.05 for a patrol.
//! - Then fatigue: 25.24 for groups 1, 2 and 3 if they flew, 3 for a patrol,
//!   and 5 off a group that did not fly.
//! - Then supply: Rabaul pays 3.28 t for each bomber and escort sent,
//!   Henderson Field 0.5 t for each fighter on patrol.
//!
//! **Afternoon.** No raid. The Japanese groups do not fly and rest. The
//! Allied fighter groups patrol, roll for wear at 0.05, gain 3, and
//! Henderson Field pays 0.5 t for each.
//!
//! ## The first day
//!
//! Night: nothing is damaged, nothing arrives, and nobody is stood down.
//! Rabaul pays (41 + 24) x 0.2 = 13 t and has 9,987 t. Henderson Field pays
//! (19 + 12) x 0.2 = 6.2 t and has 3,993.8 t.
//!
//! Morning: 41 bombers launch with 24 escorts, and VMF-223's patrol is 19.
//!
//! | Step | Shots | Chance | Expected | Std dev |
//! | ---- | ----- | ------ | -------- | ------- |
//! | Escorts at the patrol | 24 | 0.16875 | 4.05 | 1.834819 |
//! | Patrol at the escorts | 19 | 0.3 | 5.7 | 1.997498 |
//! | Patrol at the bombers | none: the 24 escorts hold all 19 | | | |
//! | Guns at the bombers | 8 | 0.05 | 0.4 | 0.616441 |
//!
//! The bombers that attack are 41 less the guns' hits, 40.6 on average, and
//! score 0.2 of their number in hits, 8.12 on average. The aircraft on the
//! ground are VMSB-232's twelve.
//!
//! Then, with e escorts and g bombers shot down:
//!
//! - Group 1 rolls 41 - g aircraft at 0.2724 and group 2 rolls 24 - e at
//!   0.2724. VMF-223 rolls its survivors at 0.05.
//! - Groups 1 and 2 are at a fatigue of 25.24, and VMF-223 at 3.
//! - Rabaul pays (41 + 24) x 3.28 = 213.2 t and has 9,773.8 t. Henderson
//!   Field pays 19 x 0.5 = 9.5 t, after 50 t for each bomb on its supply.
//!
//! Afternoon: groups 1 and 2 rest to 20.24. VMF-223 patrols with what it
//! can fly at a fatigue of 3 and ends the day at 6.
//!
//! Over seeds 1 to 1000 the first raid always flies, and the hits of each
//! of the three rolls, summed over the runs, are within four standard
//! deviations of the expected figure times 1,000. So are the bomb hits,
//! against 0.2 of the bombers that attacked.
//!
//! ## Fatigue and the days a raid flies
//!
//! Fatigue takes no roll. A group that is not stood down flies as long as
//! fly(group) is at least 1, and the tables below hold for every run in
//! which that is so on every day; the test works the rule for the others.
//!
//! Group 1, threshold 45. A day flown is +25.24 and -5. A day stood down is
//! -5 and -5.
//!
//! | Day | At the start | The staff | After the morning | After the afternoon |
//! | --- | ------------ | --------- | ----------------- | ------------------- |
//! | 1 | 0 | flies | 25.24 | 20.24 |
//! | 2 | 20.24 | flies | 45.48 | 40.48 |
//! | 3 | 40.48 | flies | 65.72 | 60.72 |
//! | 4 | 60.72 | stood down | 55.72 | 50.72 |
//! | 5 | 50.72 | stood down | 45.72 | 40.72 |
//! | 6 | 40.72 | returned, flies | 65.96 | 60.96 |
//! | 7 | 60.96 | stood down | 55.96 | 50.96 |
//! | 8 | 50.96 | stood down | 45.96 | 40.96 |
//! | 9 | 40.96 | returned, flies | 66.2 | 61.2 |
//!
//! From day 6 it flies every third day, starting each raid day 0.24 higher
//! than the last: 40.72 on day 6 and 40.72 + 0.24 x k on day 6 + 3 x k. So
//! raids fly on days 1, 2, 3, 6, 9, 12, 15, 18, 21, 24 and 27: eleven
//! raids. On day 1 it flies every bomber it has pilots for; on day 2, at
//! 20.24, 0.7976 of them; from day 3, at about 41, about 0.59 of them.
//!
//! Group 2 has the same threshold, gains the same on a raid day and flies
//! on no other day, so its fatigue is group 1's and it escorts every raid.
//!
//! Group 3 arrives on day 11 at a fatigue of 0 and escorts the raids from
//! day 12: it ends each raid day at 20.24 more than it began, less 5, and
//! rests 10 on each of the two days between, so it begins day 12 at 0, day
//! 15 at 0.24, day 18 at 0.48, and is never stood down.
//!
//! An Allied fighter group, threshold 40, gains 3 in each daylight pulse it
//! patrols: 6 a day. From the day it has aircraft:
//!
//! | Group | First flies | Stood down on days |
//! | ----- | ----------- | ------------------ |
//! | 10, VMF-223 | 1 | 8, 11, 14, 16, 19, 22, 24, 27 |
//! | 13, VMF-224 | 10 | 17, 20, 23, 25, 28 |
//! | 15, VF-5 | 22 | none |
//! | 16, Hornet and Wasp | 24 | none |
//!
//! VMF-223 reaches 42 at the end of day 7 and is stood down on day 8. A day
//! stood down takes 10 off and a day flown puts 6 on. It is on the ground
//! for the raids of days 24 and 27, where bombs can find its fighters.
//!
//! ## Over the four weeks
//!
//! The test works every pulse of every run from the state at its start, by
//! the rule above: repairs, arrivals, replacements, the daily cost, who is
//! stood down, what flies, the raid, the pilots lost, wear, fatigue and
//! supply.
//!
//! For every group in every run:
//!
//! - The aircraft it started with, those that arrived and those it received
//!   add up to its ready and damaged aircraft at the end, those shot down,
//!   those destroyed on the ground and those written off.
//! - The pilots it started with, those that arrived and those it received,
//!   less those lost, are its pilots at the end.
//! - Its strength is what it started with and what arrived.
//!
//! For each base, the supply at the end is the supply at the start less
//! what it paid by the day, for sorties and for repairs, and less what
//! bombs destroyed.
//!
//! ## Variants
//!
//! Each over seeds 1 to 100.
//!
//! | Variant | What changes |
//! | ------- | ------------ |
//! | The A6M2's radius is 261 nm, the figure from a carrier | No escort ever flies to Henderson Field: every raid is unescorted, and the whole patrol fires at the bombers. The fighters that cannot go are a patrol over Rabaul (decision 0030), which meets nobody |
//! | The G4M1's radius is 513 nm, a third of its other range row | No raid is ever launched. Nothing is lost at Henderson Field, which still patrols and pays for it |
//! | The scenario names no fatigue file | Nobody is stood down and everybody flies all they can: a raid on every morning group 1 has a bomber and a pilot |
//! | No arrivals | Groups 3 and 12 to 17 never have an aircraft. VMF-223 defends alone |
//! | No replacements at Rabaul | Groups 1, 2 and 3 receive nothing after their arrivals |
//!
//! ## What cannot happen
//!
//! - No Allied aircraft flies to Rabaul, nothing attacks Rabaul, and no
//!   Japanese aircraft is destroyed on the ground.
//! - No Allied pilot is lost.
//! - No raid flies on a day group 1 is stood down, in an afternoon or at
//!   night.
//! - No dive bomber flies, is worn or gains fatigue.
//! - No contact is held and no report is made: there are no ships.
//! - No group has aircraft before its arrival day.
//!
//! # Properties
//!
//! - Aircraft, pilots and supply are conserved, as "Over the four weeks"
//!   has it, in every run and every variant.
//! - No count and no stock is below nothing, no fatigue is outside 0 to
//!   100, and no group flies more than the smaller of its ready aircraft
//!   and its pilots.
//! - A raid is launched only at a base within the bombers' radius by the
//!   air distance, and escorts go only within theirs.
//! - Recording breakdowns changes neither the events nor the hashes.
//! - The same seed gives the same hashes twice.

// The test code is written by a developer from the hand calculation above.
