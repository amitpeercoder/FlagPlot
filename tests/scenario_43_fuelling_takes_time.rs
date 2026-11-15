//! Scenario 43: refuelling that takes time. A ship in port takes fuel
//! through the hours of the day at the port's fuelling rate, at a berth or
//! by lighter, in the cargo rule's queue. Two forces share one berth, one
//! of them loading; a cruiser at a port with no berth sails short and
//! finishes fuelling where she arrives; and a reserve holds a destroyer
//! short.
//!
//! Proves decision 0112 against a hand calculation. The data is placeholder
//! data: see `data/scenarios/43_fuelling_takes_time.toml`. It runs on the
//! frozen step 1 figures, with the fuelling rates set in the scenario's own
//! port sizes, since the frozen figures give none (no limit). There is no
//! enemy, no search and no random draw.
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
//! | Port size | Berths | Cargo at a berth | Cargo by lighter | Fuel at a berth | Fuel by lighter |
//! | --------- | ------ | ---------------- | ---------------- | --------------- | --------------- |
//! | 0         | 0      | none             | 20 t an hour     | none            | 40 t an hour    |
//! | 1         | 1      | 50 t an hour     | 20 t an hour     | 100 t an hour   | 40 t an hour    |
//!
//! The placeholder cargo ship (class 3): 3000 t of cargo, 1000 t of fuel.
//! The placeholder cruiser (class 1): 2000 t of fuel over 8000 nm, which is
//! 0.25 t a mile, 16 kn at cruise. The placeholder destroyer (class 2): 500
//! t of fuel. A day is a night pulse of 12 hours and two daylight pulses of
//! 6. In a pulse ships move first; then berths are given out, to forces in
//! number order, to a ship that has work to do, cargo to move or fuel to
//! take; then the work is done. A ship with no berth works by lighter. A
//! ship does its cargo first and then takes fuel in the hours of the pulse
//! that are left. A berth given in a pulse is that ship's for the pulse.
//!
//! ## Inputs
//!
//! A placeholder map: S at (0,0), a coast hex with port size 1; U at (5,0),
//! 200 nm from S, and V at (0,5), coast hexes with port size 0. The game is
//! 2 days. Everything is Allied.
//!
//! | Base | Hex | Supply | Fuel   | Fuel reserve |
//! | ---- | --- | ------ | ------ | ------------ |
//! | 1    | S   | 2000 t | 5000 t | 0            |
//! | 2    | U   | 0      | 3000 t | 0            |
//! | 3    | V   | 0      | 300 t  | 100 t        |
//!
//! | Force | Ship                     | Fuel aboard | Where             |
//! | ----- | ------------------------ | ----------- | ----------------- |
//! | 1     | 1, cargo ship            | 600 t       | stopped at base 1 |
//! | 2     | 2, cruiser               | 800 t       | stopped at base 1 |
//! | 3     | 3, cruiser               | 500 t       | stopped at base 2 |
//! | 4     | 4, destroyer             | 0           | stopped at base 3 |
//!
//! Orders: on day 1 force 1 loads supply up to 500 t. On day 2 force 3 is
//! routed to S.
//!
//! ## At S: one berth, cargo and fuel
//!
//! Ship 1 lacks 400 t of fuel and ship 2 lacks 1200 t.
//!
//! | Pulse              | Force 1, ship 1                                   | Force 2, ship 2                         | Base 1's fuel after |
//! | ------------------ | ------------------------------------------------- | --------------------------------------- | ------------------- |
//! | Day 1, night       | Has the berth. Loads 500 t in 10 hours, then takes 2 x 100 = 200 t of fuel | By lighter: 12 x 40 = 480 t | 4320 t |
//! | Day 1, morning     | Has the berth. Takes its last 200 t in 2 hours: full at hour 14 | By lighter: 6 x 40 = 240 t | 3880 t |
//! | Day 1, afternoon   | Nothing to do                                     | Has the berth. Takes its last 480 t in 4.8 hours: full at hour 22.8 | 3400 t |
//!
//! Base 1 holds 1500 t of supply from the night of day 1. The load is
//! complete at hour 10. At no start of a day is any ship refuelled: the
//! morning's refuelling is gone.
//!
//! ## At U: no berth, and sailing short
//!
//! Ship 3 lacks 1500 t. On day 1 she fuels by lighter in all three pulses:
//! 24 x 40 = 960 t. She has 1460 t at the end of day 1 and base 2 holds
//! 2040 t.
//!
//! Asked before day 2 is resolved, the list of forces says force 3 lacks
//! 540 t of fuel and that at base 2 it would take 13.5 hours.
//!
//! On day 2 she is ordered to S and sails at hour 24 with 1460 t: the order
//! is not held back. She steams 200 nm in 12.5 hours, burns 50 t and
//! arrives at S at hour 36.5 with 1410 t. She lacks 590 t.
//!
//! | Pulse            | Force 3, ship 3 at S                                             | Base 1's fuel after |
//! | ---------------- | ---------------------------------------------------------------- | ------------------- |
//! | Day 2, morning   | Arrives half an hour into the pulse. No other ship has work: she has the berth for the 5.5 hours left and takes 550 t | 2850 t |
//! | Day 2, afternoon | Has the berth. Takes her last 40 t in 0.4 hours: full at hour 42.4 | 2810 t             |
//!
//! ## At V: a reserve
//!
//! Base 3 holds 300 t of fuel and may give 200 t, down to its reserve of
//! 100. Ship 4 lacks 500 t. In the night of day 1 she takes 200 t by
//! lighter in 5 hours and base 3 is at its reserve. She is held at the
//! reserve, 300 t short: that is recorded once, in the night of day 1, and
//! not again in any later pulse while nothing changes.
//!
//! ## At the end of day 2
//!
//! | Ship | Fuel   | Where |
//! | ---- | ------ | ----- |
//! | 1    | 1000 t | at base 1, with 500 t of supply aboard |
//! | 2    | 2000 t | at base 1 |
//! | 3    | 2000 t | at base 1 |
//! | 4    | 200 t  | at base 3 |
//!
//! Base 1 holds 1500 t of supply and 2810 t of fuel; base 2 holds 2040 t of
//! fuel; base 3 holds 100 t of fuel.
//!
//! # Properties
//!
//! - **With no fuelling rate given,** as on the frozen figures, a ship in
//!   port takes all it lacks in the first pulse it lies there, and every
//!   scenario's stocks and fuel at the end of each day are what they were
//!   under the morning's refuelling, apart from a force that begins a game
//!   in port short of fuel and sails on day 1, which now sails as it began.
//! - Fuel is conserved: what the ships take is what the bases give, in
//!   every pulse.
//! - No ship takes more than the hours of a pulse at its rate, and none
//!   takes fuel in a pulse in which it moved for the whole of it.
//! - Refuelling never takes a base's fuel below its reserve.
//! - A force that ran out of fuel at a base's hex is stopped and idle again
//!   in the first pulse in which a ship of it takes fuel.
//! - With force 2 given the lower number, ship 2 has the berth in the night
//!   of day 1 and takes 1200 t in 12 hours; ship 1 loads 240 t by lighter
//!   and takes no fuel that pulse.
