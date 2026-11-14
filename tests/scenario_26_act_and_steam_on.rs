//! Scenario 26: a force bombards by night and is gone by day; a convoy
//! unloads and comes home by itself; and a marked waypoint with nothing to
//! do there is passed.
//!
//! Proves decision 0096 (a force that acts at a place and steams on) against
//! a hand calculation. The data is placeholder data: see
//! `data/scenarios/26_act_and_steam_on.toml`. The test runs on the frozen
//! step 1 figures (`data/reference/step1`).
//!
//! # Hand calculation
//!
//! Written by the architect before the code. What the dice decide (the hits
//! of the bombardment and what they destroy) is read from the run; the rest
//! is asserted exactly. If the code disagrees, the code or the rule is
//! wrong: do not change a number here to make a test pass. A mismatch goes
//! back to the architect.
//!
//! ## Inputs
//!
//! A placeholder map of open sea with three coast hexes: X at (0,0), Y at
//! (0,3) with port size 0, and Z at (4,3). The game is 2 days. Hour 0 is
//! 18:00 on day 1; the night pulse is 12 hours and each daylight pulse 6.
//!
//! Figures, frozen: `bombardment_minutes` 30,
//! `bombardment_reference_gun_damage` 20; a port of size 0 works cargo by
//! lighter at 20 t an hour. Classes, frozen: the placeholder cruiser (8 guns
//! at 3 rounds a minute, gun damage 20, 16 kn at cruise, 0.25 t of fuel a
//! mile), the placeholder destroyer (16 kn at cruise, 0.125 t a mile) and
//! the placeholder cargo ship (10 kn at cruise, 0.1 t a mile, 3000 t of
//! cargo).
//!
//! - Allied base 1 at X, supply 1000 t, with an air group of 20 ready
//!   aircraft and no mission. Allied base 2 at Y, supply 0. Allied base 3 at
//!   Z, supply 0.
//! - Japanese task force 1 at sea at (4,0), mission bombardment: ship 1, a
//!   placeholder cruiser with 800 rounds and 2000 t of fuel.
//! - Allied task force 2 at base 3: ship 2, a placeholder cargo ship with
//!   100 t of supply aboard and 1000 t of fuel.
//! - Allied task force 3 at sea at (0,5): ship 3, a placeholder destroyer
//!   with 500 t of fuel. It has no cargo order and its mission is surface
//!   combat.
//!
//! Orders, all on day 1, all at cruise:
//!
//! - Task force 1: route to (0,0), marked to act there, then (4,0).
//! - Task force 2: unload; route to (0,3), marked to act there, then (4,3).
//! - Task force 3: route to (2,5), marked to act there, then (4,5).
//!
//! ## Task force 1: a raid
//!
//! Day 1, night. 4 hexes, 160 nm, at 16 kn is 10 hours: it reaches (0,0) at
//! hour 10, 04:00 on day 1. The waypoint is marked and it has work there (a
//! bombardment mission, a ship that can fire, an enemy base on its map): it
//! halts. In the shore bombardment phase ship 1 fires 3 x 8 x 30 = 720
//! shells, which are 720 weighed shots, and has 80 rounds left.
//!
//! Day 1, morning. It has bombarded since it halted: it steams on, with the
//! pulse's 6 hours. 96 nm toward (4,0): two hexes and 16 nm, so its position
//! at the end of the pulse is (2,0).
//!
//! Day 1, afternoon. The last 64 nm take 4 hours. It arrives at (4,0) at
//! hour 22, 16:00 on day 1.
//!
//! Fuel: 320 nm at 0.25 t is 80 t. Ship 1 has 1920 t. On the night of day 2
//! it is at (4,0), where there is no enemy base, and does not bombard. It
//! ends with 80 rounds.
//!
//! In no daylight pulse is its position (0,0).
//!
//! ## Task force 2: a supply run
//!
//! Day 1. 4 hexes, 160 nm, at 10 kn is 16 hours: 120 nm in the night pulse
//! and the last 40 nm in the first 4 hours of the morning. It reaches (0,3)
//! at hour 16, 10:00 on day 1. The waypoint is marked and it has work there
//! (an unload order, supply aboard, a base of its side): it halts.
//!
//! | Pulse            | Hours worked | Unloaded          | Aboard | Base 2 |
//! | ---------------- | ------------ | ----------------- | ------ | ------ |
//! | Day 1, morning   | 2            | 2 x 20 = 40 t     | 60 t   | 40 t   |
//! | Day 1, afternoon | 6            | 60 t, the rest    | 0 t    | 100 t  |
//!
//! Unloading is complete 3 hours into the afternoon, at hour 21, 15:00 on
//! day 1.
//!
//! Day 2, night. Its cargo work is complete: it steams on. 120 nm in the 12
//! hours. Day 2, morning: the last 40 nm in 4 hours. It arrives at (4,3) at
//! hour 24 + 12 + 4 = 40, 10:00 on day 2.
//!
//! Fuel: 320 nm at 0.1 t is 32 t. Ship 2 has 968 t. Base 2 holds 100 t and
//! base 3 holds 0.
//!
//! ## Task force 3: nothing to do
//!
//! The waypoint (2,5) is marked, but the force has no bombardment mission
//! and no cargo order: it has no work and does not halt. 4 hexes, 160 nm, at
//! 16 kn is 10 hours. It arrives at (4,5) at hour 10, 04:00 on day 1, with
//! no halt recorded. Ship 3 burns 160 x 0.125 = 20 t and has 480 t.
//!
//! # Properties
//!
//! - A route with no mark gives the movement it gave before, apart from the
//!   hash version, in every existing scenario.
//! - A halted force never moves in a pulse in which its work is not done.
//! - The enemy's view is the same in two worlds that differ only in a mark
//!   on a waypoint where the force has no work.
