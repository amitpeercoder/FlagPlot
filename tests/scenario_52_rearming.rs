//! Scenario 52: rearming. A ship in port takes main battery rounds and
//! torpedoes from the base's ammunition through the hours of the day, at
//! a berth or by lighter.
//!
//! Proves decision 0122 against a hand calculation. The data is placeholder
//! data: see `data/scenarios/52_rearming.toml`. It runs on the frozen step
//! 1 figures, with the rearming rates and weights set in the scenario's
//! own files, since the frozen figures give none. There is no enemy, no
//! search and no random draw.
//!
//! # Hand calculation
//!
//! Written by the architect before the code. Everything is asserted
//! exactly. If the code disagrees, the code or the rule is wrong: do not
//! change a number here to make a test pass. A mismatch goes back to the
//! architect.
//!
//! ## Figures
//!
//! | Port size | Berths | Ammunition at a berth | Ammunition by lighter |
//! | --------- | ------ | --------------------- | --------------------- |
//! | 0         | 0      | none                  | 2 t an hour           |
//! | 1         | 1      | 5 t an hour           | 2 t an hour           |
//!
//! A round weighs 0.1 t for a cruiser and 0.02 t for a destroyer. A
//! torpedo weighs 2 t. These are this scenario's own figures and not the
//! live ones.
//!
//! The placeholder cruiser (class 1): 8 main guns and 100 rounds a gun, an
//! outfit of 800 rounds; 8 torpedo tubes. The placeholder destroyer (class
//! 2): 4 main guns and 150 rounds a gun, an outfit of 600 rounds; 8 tubes.
//! A day is a night pulse of 12 hours and two daylight pulses of 6. A ship
//! does its cargo first, then takes its fuel, then takes ammunition in the
//! hours of the pulse that are left, rounds before torpedoes.
//!
//! So at a berth the cruiser takes 50 rounds an hour or 2.5 torpedoes an
//! hour, and by lighter a destroyer takes 100 rounds an hour.
//!
//! ## Inputs
//!
//! A placeholder map: S at (0,0), a coast hex with port size 1, and U at
//! (5,0), a coast hex with port size 0. The game is 1 day. Everything is
//! Allied, and every ship is full of fuel and carries no cargo.
//!
//! | Base | Hex | Ammunition |
//! | ---- | --- | ---------- |
//! | 1    | S   | 80 t       |
//! | 2    | U   | 3 t        |
//!
//! | Force | Ship | Rounds aboard | Tubes fired | Where | Lacks |
//! | ----- | ---- | ------------- | ----------- | ----- | ----- |
//! | 1     | 1, cruiser   | 300 of 800 | all 8 | stopped at base 1 | 500 rounds, 50 t; 8 torpedoes, 16 t: 66 t |
//! | 2     | 2, destroyer | 100 of 600 | none  | stopped at base 1 | 500 rounds, 10 t |
//! | 3     | 3, destroyer | 100 of 600 | none  | stopped at base 2 | 500 rounds, 10 t |
//!
//! There is no order.
//!
//! ## At S: one berth
//!
//! | Pulse | Force 1, ship 1 | Force 2, ship 2 | Base 1's ammunition after |
//! | ----- | --------------- | --------------- | ------------------------- |
//! | Night | Has the berth. Takes 500 rounds, 50 t, in 10 hours; then in the 2 hours left 5 torpedoes, 10 t | By lighter: 500 rounds, 10 t, in 5 hours: full at hour 5 | 80 - 60 - 10 = 10 t |
//! | Morning | Has the berth. Takes her last 3 torpedoes, 6 t, in 1.2 hours: full at hour 13.2 | Nothing to do | 4 t |
//! | Afternoon | Nothing to do | Nothing to do | 4 t |
//!
//! At the end of the night ship 1 has 800 rounds and tubes 1 to 5 loaded;
//! tubes 6, 7 and 8 are still fired. At the end of the morning all 8 are
//! loaded.
//!
//! Three events are recorded at S: ship 1 in the night, 500 rounds, 5
//! torpedoes, 60 t; ship 2 in the night, 500 rounds, no torpedo, 10 t;
//! ship 1 in the morning, no round, 3 torpedoes, 6 t.
//!
//! Asked before the day is resolved, the list of forces says force 1 lacks
//! 500 rounds and 8 torpedoes, 66 t, and that at base 1 at a berth it
//! would take 13.2 hours; and force 2 lacks 500 rounds, 10 t, 5 hours by
//! lighter.
//!
//! ## At U: the stock runs out
//!
//! Ship 3 takes by lighter 100 rounds an hour. Base 2 holds 3 t, which is
//! 150 rounds: she takes them in the first 1.5 hours of the night and no
//! more. She ends the day with 250 rounds, and base 2 with none. One
//! event: 150 rounds, 3 t. Nothing is recorded for her in the morning or
//! the afternoon.
//!
//! # Properties
//!
//! - **The game as it was.** With no ammunition at either base, or with no
//!   ammunition rate in the port sizes, no ship takes any and nothing is
//!   recorded.
//! - **After cargo and fuel.** With ship 1 also ordered to load 500 t of
//!   supply at 50 t an hour, which takes 10 hours of the night: she takes
//!   100 rounds in the 2 hours left; in the morning 300 rounds in 6 hours;
//!   in the afternoon her last 100 rounds in 2 hours and then, in the 4
//!   hours left, her 8 torpedoes in 3.2: full at hour 23.2.
//! - **Whole ones only.** With base 1 holding 61 t in place of 80, and
//!   force 2 absent: in the night ship 1 takes 60 t as above; in the
//!   morning 1 t is left, which is less than a torpedo, and she takes
//!   nothing. Base 1 keeps its 1 t.
//! - **Short stock, in the queue's order.** With base 1 holding 65 t: in
//!   the night ship 1, served first, takes her 60 t, and ship 2 takes the
//!   5 t that are left, 250 rounds, in 2.5 hours. In the morning there is
//!   none.
//! - **The marker.** A ship whose class has 9,999 rounds a gun lacks no
//!   rounds and takes none, and takes torpedoes as any ship does.
//! - **She can fire again.** In a fight built for it on the day after,
//!   ship 1 has torpedo shots to fire; with base 1 holding no ammunition
//!   she has none.
//! - **A ship that sails keeps what she took.** With force 1 routed to U
//!   on the morning of day 1, she leaves with 800 rounds and 5 tubes
//!   loaded.
//! - Every existing scenario's events and end states are what they were.
