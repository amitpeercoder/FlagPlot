//! Scenario 51: an escort force. A force of warships with a charge keeps
//! station on it, and an enemy force must pass the escort to reach the
//! charge.
//!
//! Proves decision 0121 against a hand calculation. The data is placeholder
//! data: see `data/scenarios/51_an_escort_force.toml` and the variants
//! named below. It runs on the frozen step 1 figures.
//!
//! # Hand calculation
//!
//! Written by the architect before the code. Everything stated as a number
//! or a hex is asserted exactly. If the code disagrees, the code or the
//! rule is wrong: do not change a number here to make a test pass. A
//! mismatch goes back to the architect.
//!
//! ## Figures
//!
//! A hex is 40 nm. A day is a night pulse of 12 hours and two daylight
//! pulses of 6. The placeholder cargo ship: 10 kn at cruise. The
//! placeholder destroyer: 16 kn at cruise, 36 kn at most.
//!
//! ## Part 1: keeping station
//!
//! A placeholder map: S at (0,0), a coast hex with a port, and T at (5,0),
//! a coast hex with a port: 200 nm. There is no Japanese unit. The game is
//! 2 days.
//!
//! Task force 1, Allied, on the transport mission: one placeholder cargo
//! ship, at S. Task force 2, Allied, on the escort mission: two
//! placeholder destroyers, at S. The orders of day 1: force 1, a route to
//! T at 10 kn; force 2, escort force 1. Force 2 is given no route.
//!
//! Force 1 steams 200 nm at 10 kn: 20 hours. In the night pulse of day 1 it
//! makes 120 nm, which is 3 hexes: it ends the pulse at (3,0). It arrives
//! at T at hour 20, in the afternoon pulse, and ends day 1 at (5,0).
//!
//! Force 2 is in station at the start of every pulse and takes force 1's
//! route and speed:
//!
//! | End of | Force 1 | Force 2 |
//! | ------ | ------- | ------- |
//! | Day 1, night | (3,0) | (3,0) |
//! | Day 1, afternoon | (5,0), stopped at T | (5,0), stopped |
//! | Day 2, every pulse | (5,0) | (5,0) |
//!
//! Force 2 is in the hex of force 1 at the end of the morning pulse too.
//! It arrives at the hour force 1 does. It has burnt the fuel that a force
//! of the same two destroyers ordered along the same route at 10 kn burns:
//! the test runs that force beside it and compares.
//!
//! Properties of part 1:
//!
//! - **Out of station, it closes.** With force 2 starting at (0,2), 80 nm
//!   from S, and force 1 not sailing until day 2: on day 1 force 2 steams
//!   for S at 16 kn, 5 hours, and is at S and in station by the end of the
//!   night pulse. From day 2 it moves with force 1 as above.
//! - **A slow escort falls behind and follows.** With force 1 ordered at
//!   12 kn and force 2 made of a ship that can make no more than 10: force
//!   2 is not in force 1's hex at the end of the night pulse of day 1,
//!   steams for it, and is at T, stopped, by the end of day 2.
//! - **A route order to an escort with a charge is refused,** and names
//!   the charge. After the charge is cleared, a route order is taken.
//! - **The charge sunk or stood down:** force 2 has no charge from that
//!   moment and stops.
//! - An escort order naming a force of the other side, the force itself, a
//!   force that has a charge, or given to a force that is some force's
//!   charge, is refused.
//!
//! ## Part 2: who is met first
//!
//! No number of a fight is asserted here: a fight rolls dice. What is
//! asserted, on each of 200 seeds, is which fights begin and in what
//! order.
//!
//! The map of part 1. Task force 1, Allied, a cargo ship, and its escort
//! arrive at T in the night pulse of day 2 from one hex away, together. A
//! Japanese force lies stopped at T. The Allied forces are numbered so
//! that the charge has the lower number, which without the rule would put
//! its fight first.
//!
//! - **The escort holds.** Escort: four placeholder cruisers. Japanese
//!   force: one placeholder destroyer, set to break off at the first loss.
//!   On every seed a fight begins between the escort and the Japanese
//!   force before any other. On every seed on which that fight ends with
//!   the escort afloat and not the one that broke off, no fight begins
//!   between the cargo force and the Japanese force in that pulse, and the
//!   event of a meeting not held is recorded, caused by the escort order
//!   and the escort's fight. That is so on at least 150 of the 200 seeds.
//! - **The escort is passed.** Escort: one placeholder destroyer, set to
//!   break off at the first loss. Japanese force: four placeholder
//!   cruisers. On every seed the escort's fight begins first. On every
//!   seed on which it ends with the escort sunk or the one that broke off,
//!   a fight begins between the cargo force and the Japanese force in the
//!   same pulse, after it. That is so on at least 150 of the 200 seeds.
//! - **Unseen is not passed.** On every seed of either case on which the
//!   escort and the Japanese force pass unseen, no fight begins between
//!   the cargo force and the Japanese force.
//! - **Two escorts.** With a second escort force of one destroyer beside
//!   the four cruisers: the cargo force is met only on a seed on which
//!   both escorts' fights ended with the escort sunk or broken off.
//! - **Not in station, no screen.** With the escort a hex behind its
//!   charge when the charge arrives, the cargo force is met as chapter 4
//!   has it today.
//! - **No charge, no screen.** With the same forces and no escort order,
//!   the fights begin in task force number order, the cargo force first:
//!   the game as it was.
//! - **The enemy is told nothing** of the cargo force on a seed on which
//!   it was not met: nothing of it is in the Japanese side's events of the
//!   pulse.
//!
//! ## Part 3: against aircraft
//!
//! In a view built for it: a raid whose target is a cargo force with no
//! guns, with an escort of two placeholder destroyers in station. The
//! guns' shots at the raid are the two destroyers' anti-aircraft shots,
//! split before and after the attack by the share of chapter 5; with the
//! escort a hex away, or with no escort order, there are none. Every bomb
//! is aimed at a ship of the cargo force.
//!
//! # Properties
//!
//! - Every existing scenario's events and end states are what they were:
//!   no force in them has a charge.
//! - The same start and seed give the same events.
