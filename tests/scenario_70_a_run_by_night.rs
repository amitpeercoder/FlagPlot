//! Scenario 70: a run by night. Two fast transports keep a garrison fed
//! at a place an enemy base overlooks. At cruising speed they would
//! arrive by day. The staff sends them at full speed: they arrive in the
//! night, land their load in an hour and a half, and are gone at the
//! morning.
//!
//! Proves decision 0145, step 5 of stage 3, first part, against a hand
//! calculation. The data is placeholder data: see
//! `data/scenarios/70_a_run_by_night.toml`. It runs on the frozen step 1
//! figures. Nothing is sighted and nothing fights, so no die changes
//! anything.
//!
//! # Hand calculation
//!
//! Written by the architect before the code. Everything is asserted
//! exactly. If
//! the code disagrees, the code or the rule is wrong: do not change a
//! number here to make a test pass. A mismatch goes back to the
//! architect.
//!
//! ## Figures
//!
//! A man eats 0.005 t a day, paid at the start of the night pulse. A size
//! 5 port works at 150 t an hour; a size 0 port by lighter at 20 t an
//! hour for each ship. Day d begins at hour 24 x (d - 1): a night pulse
//! of 12 hours and two daylight pulses of 6. In a pulse ships move before
//! cargo is worked. A force that has landed its load steams on at the
//! start of the next pulse. A ship burns its cruising rate a mile at or
//! below cruising speed, and that times the square of the speed over the
//! cruising speed above it. The frozen figures give no fuelling rate, so
//! a ship in port is filled in the pulse.
//!
//! The supply doctrine: a run ordered on a morning loads that day, sails
//! the next and, with a passage of a day or less, arrives that same day;
//! it is ordered when the reading of the day after it would arrive is
//! below the floor's stock; its load is the floor's stock on the day it
//! arrives less that day's reading, plus the use of 15 days, cut to the
//! ships' room.
//!
//! ## Inputs
//!
//! The placeholder map of scenario 27: A at (0,0), port size 5; B at
//! (4,0), port size 0; C at (0,6), port size 0. A to C is 6 hexes,
//! 240 nm. C to B is 6 hexes. The game is 14 days.
//!
//! | Base | Side | Hex | Supply | Fuel |
//! | ---- | ---- | --- | ------ | ---- |
//! | 1, "Placeholder base A" | Allied | A | 1,000 t | 2,000 t |
//! | 2, "Placeholder garrison C" | Allied | C | 100 t | 0 |
//! | 3, "Placeholder enemy base B" | Japanese | B | 0 | 0 |
//!
//! Base 3 is on the Allied map and has no ship, aircraft or man. Unit 1,
//! Allied, "Placeholder Garrison", a fighting unit of 2,000 men, is
//! ashore at C on the defence.
//!
//! Task force 1, Allied, on the transport mission: ships 1 and 2,
//! placeholder fast transports with cargo (class 12: 30 t of cargo,
//! 16 kn at cruise, 36 kn at full, 500 t of fuel over 4,000 nm, which is
//! 0.125 t a mile at cruise), stopped at base 1 at cruising speed, full
//! of fuel, nothing aboard.
//!
//! The Allied side's commands of day 1: intent 1, keep base 2 supplied,
//! with a floor of 5 days and no interval, and a forces limit of ships 1
//! and 2 and base 1 as the source. No risk is set, so balanced. Leave to
//! prepare and leave to launch are both given.
//!
//! ## By night
//!
//! Base 3 is 6 hexes from C, within 15, and not in C's hex: the run goes
//! by night. At 16 kn the passage is 240 / 16 = 15 hours: it would arrive
//! at hour 15 of the day, in daylight. At 36 kn it is 240 / 36 = 6 hours
//! 40 minutes: it arrives in the night with 5 hours 20 minutes left. A
//! ship's limit is 20 x 5 1/3 = 106 2/3 t; each carries 30 t, which it
//! lands in 1.5 hours.
//!
//! At 36 kn a ship burns 0.125 x (36 / 16) squared = 0.125 x 5.0625 t a
//! mile: 151.875 t each way and 303.75 t out and home, against 60 t at
//! cruise. Each ship holds 500 t: it has the fuel. Full speed is ordered.
//!
//! ## The supply doctrine's sums
//!
//! The use at C is 2,000 x 0.005 = 10 t a day; the floor's stock is 50 t.
//!
//! | Morning | The stock at C | The day checked | Its reading | The staff |
//! | --- | --- | --- | --- | --- |
//! | 1 | 100 t | 3 | 80 t | No run |
//! | 2 | 90 t | 4 | 70 t | No run |
//! | 3 | 80 t | 5 | 60 t | No run |
//! | 4 | 70 t | 6 | 50 t | Not below 50: no run |
//! | 5 | 60 t | 7 | 40 t | A run |
//! | 7 | 100 t | 9 | 80 t | No run |
//! | 8, 9, 10 | 90, 80, 70 t | | 70, 60, 50 t | No run |
//! | 11 | 60 t | 13 | 40 t | A run |
//! | 13, 14 | 100, 90 t | | | No run |
//!
//! Each run's load: the floor's stock on the day it arrives, 50 t, less
//! that day's reading, 50 t, which is 0; plus 15 days of use, 150 t; cut
//! to the ships' room, 60 t. With the run landed the reading of the day
//! checked is 40 + 60 = 100 t, above the floor: no notice.
//!
//! ## Day by day
//!
//! | Day | Hour | The staff's commands and orders | What happens |
//! | --- | --- | --- | --- |
//! | 1 | 0 | Approves plan 1 to preparing and to released | |
//! | 5 | 96 | `add_run`, `commit_ships`. Orders a load of supply to a weight of 60 t | Each ship loads 30 t |
//! | 6 | 120 | Speed full. A route to C and back to A, acting at C, and an unload order | Sails at hour 120. Arrives at hour 126 and 40 minutes. Each ship lands 30 t by hour 128 and 10 minutes. The force steams for A at hour 132 and arrives at hour 138 and 40 minutes |
//! | 11 | 240 | `add_run`, `commit_ships`. Orders a load of supply to a weight of 60 t | |
//! | 12 | 264 | A route to C and back to A, acting at C, and an unload order | Sails at hour 264, arrives at hour 270 and 40 minutes, has landed 60 t by hour 272 and 10 minutes, steams at hour 276, arrives at hour 282 and 40 minutes |
//!
//! Seven orders in all. On every other morning the staff gives one reason
//! for giving no order. No staff order or command is refused.
//!
//! The force is at C from hour 126 and 40 minutes to hour 132 and from
//! hour 270 and 40 minutes to hour 276, and at no hour of daylight.
//!
//! ## Supply
//!
//! - **Base 2** pays 10 t at the start of each night: 90, 80, 70, 60 and
//!   50 t after the nights of days 1 to 5. On day 6 it pays 10 t and then
//!   takes 60 t: 100 t. Then 90, 80, 70, 60 and 50 t after days 7 to 11;
//!   on day 12 it pays 10 t and takes 60 t: 100 t; 90 t after day 13 and
//!   80 t after day 14.
//! - **Base 1** loads out 120 t and ends with 880 t of supply.
//! - **Fuel.** The force's speed stays at full, so it comes home at 36 kn
//!   too. Each ship burns 303.75 t on each run and is filled at A in the
//!   pulse it arrives. Base 1 gives 4 x 303.75 = 1,215 t and ends with
//!   785 t of fuel; each ship ends full.
//!
//! # Properties
//!
//! - **Press.** At press the run goes as before: no speed order is
//!   given, the force sails at hour 120 at 16 kn, arrives at hour 135, in
//!   the morning pulse, has landed 60 t by hour 136 and 30 minutes,
//!   steams at hour 138 and arrives at A at hour 153. Six orders in all.
//! - **No enemy base near.** With base 3 left out, at balanced: as at
//!   press.
//! - **An enemy base in the place's own hex only.** With base 3 at C and
//!   not on the Allied side's way: as at press.
//! - **No speed arrives in the dark, at balanced.** With base 2 and unit 1
//!   at B, base 3 at C, and the ships placeholder troop transports (class
//!   10: 10 kn at cruise, 12 kn at full): the passage of 160 nm is 16
//!   hours at cruise and 13 hours 20 minutes at full, both by day. The
//!   run of day 5 sails on day 6 at cruising speed with no speed order,
//!   and the reason of its route gives both hours.
//! - **No speed arrives in the dark, at preserve the force.** The same,
//!   at preserve the force: the run is loaded on day 5 and does not sail.
//!   On the morning of day 6 and on each morning after, the staff gives
//!   no order and hands in the notice that it cannot meet intent 1.
//! - **The night's limit.** With the lighter rate of a size 0 port at
//!   4 t an hour: a ship's limit is 4 x 5 1/3 = 21 1/3 t. A load fills
//!   ship 1 first, so the load is cut to 21 1/3 t, all in ship 1, and
//!   ship 2 goes with its force empty. The reason gives the cut.
//! - **Not the fuel for full speed.** With each ship holding 300 t and
//!   base 1 no fuel: at 36 kn a ship needs 303.75 t and has 300. Full
//!   speed is passed over, cruise arrives by day, and at balanced the run
//!   goes by day at cruising speed; the reason gives 300 against 303.75.
//! - The same start gives the same orders; two worlds that differ only in
//!   what the Allied side cannot see give the same orders and reasons; a
//!   save on day 8 and a resume give the same game; scenarios 67, 68 and
//!   69 give the orders they gave.
