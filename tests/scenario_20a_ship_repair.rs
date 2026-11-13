//! Scenario 20a: ship repair, forward at a base and in a yard off the map.
//!
//! Decision 0069 (`docs/decisions/0069-ship-repair.md`, its decided text and
//! the section "As planned for the build"). A base may mend a number of
//! points of damage a day; the staff spends them, at the start of each day,
//! on the forces of its side stopped there. A base may be a port for the
//! yard; a force stopped there can be ordered to the yard, leaves the map and
//! comes back on a day worked out from its damage, repaired, refuelled and
//! rearmed. There is no random draw.
//!
//! # Hand calculation
//!
//! Drafted by the team lead, checked number by number and adopted by the
//! coordinating architect on Nov 13, 2026. The tests below assert these
//! numbers. If the code disagrees, the code or the rule is wrong: do not
//! change a number here to make a test pass; a mismatch goes back to the
//! architect.
//!
//! Quantities are cut to six decimal places at each step. Every quantity in
//! this scenario is exact at two places, so nothing is cut.
//!
//! ## The rule, as worked here
//!
//! **Forward repair**, at the start of each day, after returns from the
//! yard, refuelling and the day's orders. Each base's allowance for the day
//! is its `ship_repair_points_per_day`. Forces of the base's side stopped in
//! its hex (not under way, no `toward_hex`, not away) are served in number
//! order, ships in number order. One ship spends, each step taking the
//! smaller of the allowance left and what she needs:
//!
//! 1. fire damage, down toward 0;
//! 2. flotation damage, down toward 0, only if it is below
//!    `forward_repair_max_flotation`;
//! 3. the first degraded system in system order (propulsion, fire control,
//!    main battery, torpedo mounts, flight deck): it needs
//!    `forward_repair_system_points` less the points already banked on the
//!    ship (`system_repair_points`). If the allowance covers that, the system
//!    is working, the bank is 0, and the next degraded system is taken;
//!    otherwise the allowance left goes into the bank. A system out is passed
//!    over.
//!
//! A ship that takes points records one `ShipRepaired`; one that takes none
//! records nothing.
//!
//! **The yard.** Days of repair of a ship = ceil(2 x flotation + 5 x systems
//! degraded + 30 x systems out); fire adds nothing. Days away of a force =
//! the port's `yard_passage_days` + the largest days of repair of its ships.
//! Day of return = the day the order is taken in + the days away; the force
//! is back at the start of that day, before refuelling and orders. A force
//! whose day of return is after the last day never returns.
//!
//! ## Figures
//!
//! The frozen step 1 figures (`data/reference/step1`), with the five new
//! figures at their first values, the same in the live file:
//!
//! | Figure | Value |
//! | --- | --- |
//! | `forward_repair_max_flotation` | 10 |
//! | `forward_repair_system_points` | 10 |
//! | `yard_days_per_flotation_point` | 2 |
//! | `yard_days_per_system_degraded` | 5 |
//! | `yard_days_per_system_out` | 30 |
//! | `contact_drop_age_pulses` (search, unchanged) | 5 |
//!
//! Ship classes (frozen `ship_classes.csv`): class 1, "Placeholder cruiser":
//! fuel capacity 2,000 t, 8 main guns with 100 rounds a gun (a full load of
//! 800 rounds), 8 torpedo tubes, 8 bearing on one side; it has propulsion,
//! fire control, main battery and torpedo mounts. Class 2, "Placeholder
//! destroyer": 500 t, 4 guns with 150 rounds a gun (600), 8 tubes, 8 on one
//! side; the same four systems. Neither class has a flight deck, and neither
//! ever sets decision 0083's one-side flag.
//!
//! ## The scenario
//!
//! `data/scenarios/20a_ship_repair.toml`, 37 days, seed 1942, on a new
//! placeholder map `data/map/placeholder_repair.toml`: open sea, 40 nm hexes,
//! q from -1 to 10 and r from -1 to 5, with anchorages at (0,0), (4,0),
//! (0,4) and (8,0). The scenario names no fatigue figures, so no force's
//! fatigue changes by the fatigue rule. No base has fuel or supply, so
//! nothing is refuelled in the whole run.
//!
//! | Base | Side | Hex | `ship_repair_points_per_day` | `yard_passage_days` |
//! | --- | --- | --- | --- | --- |
//! | 1, Placeholder repair anchorage | Allied | (0,0) | 6 | none |
//! | 2, Placeholder yard port | Allied | (4,0) | none (0) | 4 |
//! | 3, Placeholder bare anchorage | Allied | (0,4) | none | none |
//! | 4, Placeholder enemy anchorage | Japanese | (8,0) | 6 | 4 |
//!
//! Every task force is Allied, mission transport, stopped at its place at
//! the start, readiness normal unless stated:
//!
//! | Force | At | Ships |
//! | --- | --- | --- |
//! | 1 | base 1 | 1, 2 |
//! | 2 | base 2, readiness battle, fatigue 40 | 3, 4 |
//! | 3 | base 2 | 5 |
//! | 4 | base 2 | 6 |
//! | 5 | base 3 | 7 |
//! | 6 | base 4 | 8 |
//! | 7 | base 1 | 9 |
//!
//! | Ship | Class | Fuel | Guns | Fire | Flotation | Systems not working |
//! | --- | --- | --- | --- | --- | --- | --- |
//! | 1 | cruiser | 2,000 | 800 | 4 | 6 | fire control degraded, main battery out |
//! | 2 | destroyer | 500 | 600 | 2.5 | 9.5 | torpedo mounts degraded |
//! | 3 | cruiser | 1,500 | 500 | 7 | 12.5 | fire control degraded |
//! | 4 | destroyer | 300 | 200 | 6 | 3.25 | torpedo mounts degraded |
//! | 5 | cruiser | 2,000 | 800 | 0 | 0 | propulsion out |
//! | 6 | destroyer | 500 | 600 | 3 | 0 | none |
//! | 7 | destroyer | 500 | 600 | 3 | 2 | none |
//! | 8 | destroyer | 500 | 600 | 3 | 2 | none |
//! | 9 | cruiser | 2,000 | 800 | 1 | 10 | propulsion degraded |
//!
//! The Japanese side has no task force. It starts holding one contact at
//! (4,0), two ships, cruiser, age 0.
//!
//! Orders (Allied), taken in at the start of their day:
//!
//! | Day | Force | Order |
//! | --- | --- | --- |
//! | 1 | 1 | `send_to_yard` |
//! | 1 | 4 | `set_speed` cruise, `set_route` to (2,2), then `send_to_yard` |
//! | 1 | 6 | `send_to_yard` |
//! | 2 | 2 | `send_to_yard` |
//! | 3 | 2 | `send_to_yard` |
//! | 3 | 4 | `send_to_yard` |
//! | 4 | 3 | `send_to_yard` |
//!
//! ## Case A: the orders and their answers
//!
//! | Day | Force | Answer | Why |
//! | --- | --- | --- | --- |
//! | 1 | 1 | refused, `NotAYardPort` | stopped at base 1, its own, which is not a port for the yard |
//! | 1 | 4 | speed and route accepted; yard refused, `UnderWay` | the route, taken in before it, put it under way |
//! | 1 | 6 | refused, `NoOwnBaseHere` | base 4 is Japanese |
//! | 2 | 2 | accepted; `ForceLeftForYard` | below |
//! | 3 | 2 | refused, `AwayAtYard` | away until day 36 |
//! | 3 | 4 | refused, `NoOwnBaseHere` | it arrived at (2,2), two hexes away, on day 1; no base there |
//! | 4 | 3 | accepted; `ForceLeftForYard` | below |
//!
//! A refused order changes nothing: force 1 is still stopped at base 1 and
//! is repaired on day 1 (case B).
//!
//! ## Case B: forward repair at base 1, 6 points a day
//!
//! Base 1 serves force 1 (ships 1 and 2) and then force 7 (ship 9). L is the
//! allowance left.
//!
//! | Day | Ship | Spent on | Points | After | L after |
//! | --- | --- | --- | --- | --- | --- |
//! | 1 | 1 | fire 4 to 0; flotation 6 < 10, 6 to 4 | 4 + 2 = 6 | fire 0, flotation 4 | 0 |
//! | 2 | 1 | flotation 4 to 0; fire control needs 10, banks 2 | 4 + 2 = 6 | flotation 0, bank 2 | 0 |
//! | 3 | 1 | fire control needs 10 - 2 = 8, banks 6 | 6 | bank 8 | 0 |
//! | 4 | 1 | fire control needs 2: working; main battery out, passed over | 2 | fire control working, bank 0 | 4 |
//! | 4 | 2 | fire 2.5 to 0; flotation 9.5 < 10, 9.5 to 8 | 2.5 + 1.5 = 4 | fire 0, flotation 8 | 0 |
//! | 5 | 2 | flotation 8 to 2 (ship 1 takes nothing and records nothing) | 6 | flotation 2 | 0 |
//! | 6 | 2 | flotation 2 to 0; torpedo mounts need 10, banks 4 | 2 + 4 = 6 | flotation 0, bank 4 | 0 |
//! | 7 | 2 | torpedo mounts need 10 - 4 = 6: working | 6 | torpedo mounts working, bank 0 | 0 |
//! | 8 | 9 | fire 1 to 0; flotation 10 is not below 10, untouched; propulsion needs 10, banks 5 | 1 + 5 = 6 | fire 0, bank 5 | 0 |
//! | 9 | 9 | propulsion needs 10 - 5 = 5: working | 5 | propulsion working, bank 0 | 1 |
//!
//! From day 10 nothing at base 1 takes a point and nothing is recorded:
//! ship 1's main battery is out and ship 9's flotation is 10.
//!
//! Points spent: 6 a day on days 1 to 8, 5 on day 9: 53. By ship: ship 1,
//! 4 + 6 + 10 = 20; ship 2, 2.5 + 9.5 + 10 = 22; ship 9, 1 + 10 = 11;
//! 20 + 22 + 11 = 53.
//!
//! `ShipRepaired` events, in order: day 1 ship 1; day 2 ship 1; day 3 ship 1;
//! day 4 ship 1 then ship 2; day 5 ship 2; day 6 ship 2; day 7 ship 2; day 8
//! ship 9; day 9 ship 9. Ten events. Day 4's shows the day's points shared
//! in ship order; days 8 and 9 show force 7, numbered after force 1, served
//! only once force 1 needs nothing. Each names its force and base 1, and
//! cites nothing (the forces began the scenario there and have no arrival
//! event). Day 8's breakdown names `forward_repair_max_flotation` 10.
//!
//! At the end: ship 1 fire 0, flotation 0, fire control working, main
//! battery out, bank 0. Ship 2 all 0, all working. Ship 9 fire 0, flotation
//! 10, all working, bank 0.
//!
//! ## Case C: no repair where it does not apply
//!
//! - Ship 6 (force 4): at base 2 on day 1, whose figure is 0; then at
//!   (2,2), where there is no base. Fire 3 to the end.
//! - Ship 7 (force 5): stopped at base 3, Allied, with no figure. Fire 3,
//!   flotation 2 to the end.
//! - Ship 8 (force 6): stopped at base 4, which mends 6 a day for the
//!   Japanese side only. Fire 3, flotation 2 to the end.
//! - Ships 3, 4 and 5: at base 2 (figure 0) until they leave; nothing while
//!   away.
//!
//! None of them records a `ShipRepaired`.
//!
//! ## Case D: force 2 to the yard and back
//!
//! On day 2, at the order's acceptance:
//!
//! | Ship | Flotation | Degraded | Out | Days of repair |
//! | --- | --- | --- | --- | --- |
//! | 3 | 12.5 | 1 | 0 | ceil(2 x 12.5 + 5 x 1) = ceil(30) = 30 |
//! | 4 | 3.25 | 1 | 0 | ceil(2 x 3.25 + 5 x 1) = ceil(11.5) = 12 |
//!
//! Ship 3's fire of 7 and ship 4's of 6 add nothing. Days away: 4 + 30 = 34.
//! Day of return: 2 + 34 = 36. `ForceLeftForYard`: force 2, base 2, passage
//! 4, ships [(3, 30), (4, 12)], days away 34, day of return 36; caused by
//! the order. Its breakdown: passage 4; ship 3 flotation 12.5, degraded 1,
//! out 0, days 30; ship 4 flotation 3.25, degraded 1, out 0, days 12; days
//! away 34.
//!
//! From then until the start of day 36: ships 3 and 4 are at the yard
//! (location `Yard(2)`), force 2 is at (4,0), stopped, with no ships, its
//! fatigue still 40 and its readiness battle; its ships keep the damage,
//! fuel and guns they left with.
//!
//! **What `measure` says**, asked `yard` of force 2 after day 1 resolves:
//! not refused; port base 2, passage 4; ship 3 flotation 12.5, degraded 1,
//! out 0, 30 days; ship 4 flotation 3.25, degraded 1, out 0, 12 days; days
//! away 34; leaves day 2; returns day 36; returns before the end, the last
//! day being 37. The same numbers as the event of day 2.
//!
//! **Orders the evening before.** `World::check_order` for a `set_speed`
//! to force 2: after day 34 resolves (the order's day would be 35, before
//! its return) `Err(AwayAtYard)`; after day 35 resolves (day 36, its return)
//! `Ok`.
//!
//! **At the start of day 36**, first of the day's steps,
//! `ForceReturnedFromYard`: force 2, base 2, ships [3, 4], caused by the
//! departure event of day 2. Afterwards:
//!
//! | | Ship 3 | Ship 4 |
//! | --- | --- | --- |
//! | Fire, flotation | 0, 0 | 0, 0 |
//! | Systems | all working | all working |
//! | Bank | 0 | 0 |
//! | Fuel | 1,500 to 2,000 | 300 to 500 |
//! | Guns | 500 to 800 | 200 to 600 |
//! | Torpedoes ready | true | true |
//! | One-side flag | false | false |
//! | Location | force 2 | force 2 |
//!
//! Force 2: ships {3, 4}, at (4,0), idle, no route, fatigue 40 to 0, not
//! alerted, readiness battle (kept), arrival event the return. Base 2 has
//! no fuel and no repair figure, so nothing else is recorded for it that
//! day or the next.
//!
//! ## Case E: force 3 does not come back
//!
//! On day 4: ship 5, flotation 0, degraded 0, out 1: ceil(30) = 30 days.
//! Days away 4 + 30 = 34; day of return 4 + 34 = 38, after day 37.
//! `ForceLeftForYard`: force 3, base 2, passage 4, ships [(5, 30)], days 34,
//! day of return 38. `measure` asked after day 3 says returns day 38, not
//! before the end. No `ForceReturnedFromYard` is ever recorded for it. At
//! the end ship 5 is afloat at the yard (`Yard(3)`) with propulsion out.
//!
//! ## Case F: what the Japanese side sees
//!
//! Nothing of the rule. Its contact at (4,0) is never refreshed: it ages at
//! the end of each pulse (1 after day 1's night, 2, 3, 4) and is dropped at
//! the end of day 2's morning, at age 5. Its side events are those four
//! `ContactAged` and one `ContactDropped`, and no event that names any
//! Allied unit; they are the same event for event as in the variant with
//! no Allied orders at all. Its briefings have no line on repair or the
//! yard.
//!
//! ## Variants
//!
//! | Variant | What changes |
//! | --- | --- |
//! | Base 1's figure is 0 | No `ShipRepaired` at all; ships 1, 2 and 9 end as they began |
//! | `forward_repair_max_flotation` 9 (an altered copy of the frozen figures) | Days 1 to 3 and ship 1 on day 4 as case B. Day 4, ship 2: fire 2.5 to 0, flotation 9.5 is not below 9, torpedo mounts bank 1.5 (L 0). Day 5: ship 2 needs 8.5, banks 6: bank 7.5. Day 6: ship 2 needs 2.5: working, L 3.5; ship 9: fire 1 to 0 (L 2.5), flotation untouched, propulsion banks 2.5. Day 7: ship 9 needs 7.5, banks 6: bank 8.5. Day 8: ship 9 needs 1.5: working, L 4.5. Nothing after. Ship 2 ends with flotation 9.5, ship 9 with 10. Points: 6 x 7 + 1.5 = 43.5 = ship 1's 20 + ship 2's 2.5 + 10 + ship 9's 1 + 10 |
//! | Force 2's order is given on day 1 | Same days, 34; returns day 35 |
//! | Base 2's `yard_passage_days` is 0 | The scenario is refused when loaded |
//!
//! ## Unit tests (where the scenario cannot reach)
//!
//! - A ship of the wing-tubes class with `torpedoes_ready` true and the
//!   one-side flag set returns with ready true and the flag false; one with
//!   torpedoes spent returns with them ready.
//! - Two bases of a side in one hex, both with a figure: a ship takes from
//!   the lower-numbered until its allowance is spent, and the next ship from
//!   the other.
//! - A degraded system hit to out while points are banked: the bank goes to
//!   the next degraded system.
//! - The same with no other degraded system: the bank is kept until forward
//!   repair next reaches step 3, which finds no degraded system, sets it to 0
//!   and spends nothing for it.
//! - Decision 0085's projection: a force that lost a ship in a fight that
//!   ended force-sunk by the enemy's loss, and then leaves for the yard, is
//!   still told its own true reason for that fight, not "force sunk".
//! - The objectives tally: a ship at the yard at the end with propulsion
//!   out counts as crippled (in this scenario, ship 5).
//! - A carrier with an air group aboard sent to the yard: the group flies
//!   nothing while away and returns with its aircraft, damaged aircraft and
//!   pilots as they were (pending the architect's choice).
//!
//! ## Properties
//!
//! - No draw: over seeds 1 to 50 the event log (every event's day, pulse,
//!   kind, data and causes) is the same; only the state hashes differ, by
//!   the seed. `rng::Draw::ALL` is unchanged.
//! - Determinism: one seed run twice gives the same hash every day.
//! - No damage track ever rises by this rule; no system out is ever mended
//!   forward; a bank is always from 0 to below 10.
//! - What must not move: with no base given a repair figure or a yard
//!   passage, every other scenario, on the frozen and the live figures,
//!   gives its event log byte for byte as before
//!   (`crates/sim-cli/tests/run_baseline/*.events.txt` unchanged); only the
//!   state hashes move, by the new fields and state version 9.
//!
//! This rule has no chance, so the three parts of `docs/step2-spec.md`,
//! "Tests while figures move", 7, reduce to the hand calculation above and
//! the properties: there is no chance to hand-calculate at a working value
//! and no seed to pin.

#[test]
#[ignore = "not built: the hand calculation awaits the architect's check (decision 0069)"]
fn hand_calculation_awaits_the_architects_check() {}
