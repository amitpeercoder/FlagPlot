//! Scenario 104: men put ashore score. A side with the objective "men
//! landed" at a base is scored for the men its ships land in that base's
//! hex and do not take off again, for each hundred men.
//!
//! Proves decision 0186. The arithmetic is the architect's, written before
//! the code. If the code disagrees, the code or the rule is wrong: a
//! mismatch goes back to the architect, and nobody else changes a number
//! here.
//!
//! # The sum
//!
//! For each land unit a number that starts at 0. The events are read in
//! order. Men of the unit landed in the base's hex by a ship of the side
//! raise it by those men. Men of the unit embarked by a ship of the side,
//! in any hex, lower it by those men, and not below 0.
//! count = the sum of the units' numbers.
//! points = points for each 100 men x count / 100.
//!
//! # Statements on made-up events (a log built for the test)
//!
//! Base B is in hex H. The Japanese side has the objective men landed at
//! B, 1 point for each 100 men. Ships J1 and J2 are Japanese, ship A1 is
//! Allied. A placeholder setting: no real ship or unit. Unless a
//! statement says otherwise all the men are of one unit, U.
//!
//! 1. **One landing.** J1 lands 350 men in H. Count 350, points
//!    1 x 350 / 100 = 3.5.
//! 2. **Landings add, over ships and days.** J1 lands 350 on day 1 and 600
//!    on day 2; J2 lands 152 and 48 on day 2. Count 350 + 600 + 152 + 48 =
//!    1,150, points 11.5.
//! 3. **A part of 100 scores its part.** J2 lands 48 men and nothing else
//!    is landed. Count 48, points 0.48.
//! 4. **Men taken off again were not landed.** J1 lands 350; later J2
//!    embarks 120 in H. Count 350 - 120 = 230, points 2.3.
//! 5. **Never below nothing.** Nothing is landed; J1 embarks 100 men who
//!    began the game ashore in H. Count 0, points 0, and not -1. J1 then
//!    lands those 100 in H again: count 100, points 1.
//! 6. **Another hex does not count.** J1 lands 350 in a hex beside H and
//!    nothing in H. Count 0.
//! 7. **The other side's ships do not count, either way.** J1 lands 350 in
//!    H; A1 lands 200 in H and embarks 50 there. The Japanese count is
//!    350, points 3.5. An Allied objective of men landed at B at 2 points
//!    for each 100 men counts 200 - 50 = 150, points 2 x 150 / 100 = 3.
//! 8. **Men landed and then lost still count.** J1 lands 350 in H; 200 of
//!    them are then lost in a fight. The Japanese count is 350, points
//!    3.5. An Allied objective of men lost at 1 point for each 100 men
//!    counts those 200, points 2.
//! 9. **The base may be the enemy's, and may change side.** B is Allied
//!    when J1 lands 350 in H, and Japanese at the end. Count 350.
//! 10. **Other points for each 100 men.** With 0.5 points for each 100 men
//!     and statement 2's landings: 0.5 x 1,150 / 100 = 5.75.
//! 11. **Landing the same men twice does not count them twice.** J1 lands
//!     350 of U in H. They march to hex K. J1 embarks the 350 at K: count
//!     350 - 350 = 0. J1 lands them in H again: count 350, and not 700.
//! 12. **Men who march away still count.** J1 lands 350 of U in H and they
//!     march to K and stay ashore. Count 350.
//! 13. **Another unit's men who embark do not lower it.** J1 lands 350 of
//!     U in H. Unit V, 200 men who were never landed from a ship, marches
//!     into H and J2 embarks 120 of V there. Count 350.
//! 14. **Unit by unit.** J1 lands 350 of U and 600 of V in H; J2 embarks
//!     400 of U at K, more than U landed. U's number is 0, not -50; V's is
//!     600. Count 600, points 6.
//! 15. **What the side knows** of its own progress in each of statements 1
//!     to 5 and 11 to 14 is exact, with the same count and points. The other side is
//!     told nothing of the count.
//!
//! # Properties, on logs made at random by the test's own fixed seeds
//!
//! - A further landing in H by the side's ships never lowers the count.
//! - An embarking never raises it.
//! - The points never fall when men are landed and never rise when men are
//!   embarked.
//! - Nothing the other side's ships do changes it.
//! - The count is never below 0 and never above the men the side's ships
//!   landed in H.
//!
//! # Statements on a scenario file
//!
//! 16. **The file's key.** An objective written `kind = "men_landed"`,
//!     `base = 5`, `points_per_100_men = 1` loads as men landed at base 5
//!     at 1 point for each 100 men. One that names a base the scenario
//!     does not have is refused, and so is one with points below 0, each
//!     as supply landed and men lost are refused.
//!
//! # Statements on scenario 103, file a, on its own seed (1942)
//!
//! The game is the one the second version of the orders gives (decision
//! 0184): the convoy lands 350, 350, 600, 152 and 48 men at Taivu, base 5,
//! hex (3,-4), in the night pulse of day 2, and embarks none.
//!
//! 17. **The same game.** The five daily hashes are what they are without
//!     the objective. (Objectives are not in the state and nothing reads
//!     them.) The test takes the hashes from the same file with the
//!     objective taken out, not from a number written here.
//! 18. **The count.** 350 + 350 + 600 + 152 + 48 = 1,500 men, at 1 point
//!     for each 100: 15 points.
//! 19. **The result.** Without the objective the Japanese side has 8.87
//!     points and the Allied side 22.69, an Allied margin of 13.82, a
//!     major victory. With it the Japanese side has 8.87 + 15 = 23.87,
//!     the Allied side 22.69 still: an Allied margin of 22.69 - 23.87 =
//!     -1.18, inside the minor level of 3 either way, a stalemate.
//! 20. **What the Japanese side knows** at the end of day 1: 0 men, exact.
//!     At the end of day 2 and after: 1,500 men, 15 points, exact.
//! 21. **The words.** The Japanese briefing has the line "Objective: men
//!     you land from ships at BASE and do not take off again, points for
//!     each hundred men: 1", BASE as that seat names Taivu. The Allied
//!     briefing has, among what the enemy is scored for, "men the enemy
//!     lands from ships at (3,-4) and does not take off again, points for
//!     each hundred men: 1": Taivu is not on the Allied map, so it is
//!     named by its hex and not by a number or a name (decision 0183,
//!     rule 4). The line for supply landed there names it the same way.
//!     Every number in either line is in the briefing's data.
//! 22. **File c and file d,** where the Japanese side is given no orders
//!     and the convoy does not sail: count 0, points 0.
//!
//! # Statements on a scenario for play (decision 0187, rule 22)
//!
//! 23. **The key.** `for_play = true` in a scenario's `[objectives]` table
//!     loads; with the key absent or `false` the scenario is of record.
//!     Scenario 103's four files have it. No other scenario file has it.
//! 24. **It changes no game.** File a's five daily hashes on seed 1942 are
//!     the same with the key as with it taken out.
//! 25. **The result's words.** `sim-cli run` on file a prints its result
//!     under "Result (the scenario's own scale, for play):". On scenario
//!     5, `05_eastern_solomons.toml`, it prints "Result (the placeholder
//!     scale of decision 0054):" as it does today.
//! 26. **The batch file's header** of file a carries `"for_play": true`;
//!     scenario 5's carries `false`.
//! 27. **`sim-cli validate` refuses a scenario for play,** saying that its
//!     results are not for the record, and exits with an error. It runs
//!     scenario 5 as it does today.
//!
//! The tests are written by the build under these statements.
