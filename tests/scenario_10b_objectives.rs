//! Objectives and results (decision 0054) for Savo Island, Rabaul against
//! Guadalcanal and the full slice.
//!
//! # Hand calculation
//!
//! Written by the architect before the test code, from the event log of
//! each scenario as it runs today with its own seed (1942). The tests below
//! assert these numbers. If the code disagrees, the code or the rule is
//! wrong: do not change a number here to make a test pass.
//!
//! Every scenario uses the placeholder scale of decision 0054: a ship sunk
//! is carrier 10, battleship 8, cruiser 4, destroyer 1, transport 2,
//! auxiliary 1; a ship crippled is half of that; an aircraft destroyed is
//! 0.05. Levels in all three: minor 3, major 8, decisive 16.
//!
//! ## Scenario 4, Savo Island (`04_savo_island.toml`)
//!
//! Both sides hold two objectives: ships sunk and ships crippled.
//!
//! | Fact | |
//! | ---- | - |
//! | Japanese ships sunk | ship 3 Kako and ship 5 Furutaka, cruisers |
//! | Allied ships sunk | ship 9 Canberra, a cruiser; ship 11 Patterson, a destroyer |
//! | Ships afloat with propulsion out | none on either side (Chokai, Aoba and Chicago have it degraded, which does not count) |
//!
//! Allied points: 2 cruisers x 4 = 8. Japanese points: 4 + 1 = 5. Allied
//! margin 8 - 5 = 3, which is exactly the minor level: a minor Allied
//! victory and a minor Japanese defeat.
//!
//! The night itself was a decisive Japanese victory. This result is the
//! scenario's known fault (decision 0024) set on the scale, not a claim
//! about history.
//!
//! ## Scenario 6, Rabaul against Guadalcanal (`06_rabaul_guadalcanal.toml`)
//!
//! The Allied side holds aircraft destroyed (0.05) and base 1, Henderson
//! Field, open at the end (10). The Japanese side holds aircraft destroyed
//! (0.05) and base 1 closed at the end (10).
//!
//! | Aircraft lost over the 28 days | Shot down | On the ground | Written off | Total |
//! | ------------------------------ | --------- | ------------- | ----------- | ----- |
//! | Japanese | 37 | 0 | 24 | 61 |
//! | Allied   | 38 | 6 | 8  | 52 |
//!
//! Henderson Field ends with 2747 t of supply and aircraft ready (groups 11
//! to 17 hold 9, 11, 1, 12, 13, 16 and 12), so it is open.
//!
//! Allied points: 61 x 0.05 = 3.05, and 10 for the open field: 13.05.
//! Japanese points: 52 x 0.05 = 2.6, and 0: 2.6. Allied margin 10.45, at
//! least 8 and under 16: a major Allied victory.
//!
//! ## Scenario 7, the full slice (`07_full_slice.toml`)
//!
//! The Allied side holds ships sunk, ships crippled, aircraft destroyed
//! (0.05), cargo landed at base 1 at 0.005 a ton, and base 1 open at the end
//! (10). The Japanese side holds ships sunk, ships crippled, aircraft
//! destroyed (0.05) and base 1 closed at the end (10).
//!
//! | Fact | |
//! | ---- | - |
//! | Japanese ships sunk | ship 33 Ryujo, a carrier |
//! | Allied ships sunk | ships 42 and 43, cruisers; ship 44, a destroyer |
//! | Allied ships crippled | ship 1 Enterprise, a carrier |
//! | Japanese ships crippled | none (ship 40 has propulsion degraded only) |
//! | Japanese aircraft lost | shot down 78, lost with Ryujo 29, written off 17: 124 |
//! | Allied aircraft lost | shot down 32, on the ground 4, written off 7: 43 |
//! | Cargo landed at base 1 by Allied ships | ships 37 and 38, each 40 + 120 + 240 + 80 = 480 t: 960 t |
//! | Henderson Field at the end | 4300.955 t of supply, aircraft ready: open |
//!
//! | Objective | Allied | Japanese |
//! | --------- | ------ | -------- |
//! | Ships sunk | 10 | 4 + 4 + 1 = 9 |
//! | Ships crippled | 0 | 5 |
//! | Aircraft destroyed | 124 x 0.05 = 6.2 | 43 x 0.05 = 2.15 |
//! | Cargo landed | 960 x 0.005 = 4.8 | |
//! | Base open / closed | 10 | 0 |
//! | Total | 31 | 16.15 |
//!
//! Allied margin 31 - 16.15 = 14.85, at least 8 and under 16: a major
//! Allied victory. The full slice is not a reconstruction and has no
//! history to set beside it.
//!
//! # Properties
//!
//! - Each scenario's daily hashes and events are what they were without the
//!   objectives section.
//! - In each, the two margins sum to zero and the levels mirror.
//! - The result from a saved game replayed is the same.
