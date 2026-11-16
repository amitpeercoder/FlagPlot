//! Scenario 54: a surface force that stands by night. The commander states
//! "deny the waters at (3,0)" and names task force 1. The staff gives the
//! force the task of standing there, and at the force's level sends it
//! out on the nights it believes in a threat and brings it home at
//! morning.
//!
//! Proves decision 0124 against a hand calculation. The data is placeholder
//! data: see `data/scenarios/54_a_force_that_stands_by_night.toml`. It
//! runs on the frozen step 1 figures, which give no fuelling rate, so a
//! ship in port is filled in the pulse. There is no enemy unit, no search
//! and no random draw.
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
//! A hex is 40 nm. A day is a night pulse of 12 hours and two daylight
//! pulses of 6; day d begins at hour 24 x (d - 1), with its night. The
//! placeholder cruiser (class 1): 16 kn at cruise, 2000 t of fuel over
//! 8000 nm, which is 0.25 t a mile at cruise. A force's position is the
//! nearest hex centre. A contact gains a pulse of age at the end of each
//! pulse and is dropped at 5.
//!
//! ## Inputs
//!
//! A placeholder map of open sea, with S at (0,0) a coast hex with a port.
//! Allied base 1 at S, with 5000 t of fuel. The waters are the hex (3,0),
//! 120 nm from S. The game is 4 days. There is no Japanese unit.
//!
//! Task force 1, Allied: two placeholder cruisers, ships 1 and 2, stopped
//! at base 1, full of fuel, on the surface combat mission, at normal
//! readiness, with no home port and the stance avoid.
//!
//! The Allied side starts holding one contact: 3 ships, a cruiser, at
//! (10,0), age 0. It is 7 hexes from the waters.
//!
//! The Allied side's commands of day 1: intent 1, deny the waters within 0
//! hexes of (3,0); its forces limit, task force 1; leave to prepare and
//! leave to launch both given. No risk is set: it is balanced.
//!
//! ## The tree on the first morning
//!
//! | Intent | Kind | Parent | Maker | Addressee |
//! | ------ | ---- | ------ | ----- | --------- |
//! | 1      | deny the waters at (3,0) | none | the commander | the area |
//! | 2      | stand at (3,0), task force 1 | 1 | the staff | task force 1 |
//!
//! Base 1 has no air group, so no strike task and no search task is
//! formed, and no notice is handed in: a task was formed.
//!
//! ## The sortie
//!
//! S to the station is 120 nm: 7.5 hours at 16 kn, inside the night's 12.
//! Out and home is 240 nm: 60 t for each ship. At balanced the force needs
//! 60 x 1.2 = 72 t in each ship.
//!
//! A sortie ordered on day d: the force sails at the start of the night,
//! reaches the station 7.5 hours into it and halts to stand. At the start
//! of the morning it steams on for S: in the morning's 6 hours it makes 96
//! nm and is 24 nm from S, nearest the centre of (1,0). It arrives 1.5
//! hours into the afternoon, 19.5 hours after it sailed, and each ship is
//! filled in that pulse.
//!
//! ## Day by day
//!
//! | Day | The contact at the staff's hour | Threat | The staff's orders to force 1 | End of night | End of morning | End of afternoon |
//! | --- | ------------------------------- | ------ | ----------------------------- | ------------ | -------------- | ---------------- |
//! | 1   | held, age 0 | yes | Home port, base 1. Rules of engagement: engage, break-off level 0.5. Battle readiness. Route to (3,0) and S, (3,0) marked to stand at | (3,0), halted | (1,0), under way | S, stopped, 2000 t in each ship |
//! | 2   | held, age 3 | yes | Route, as day 1 | (3,0), halted | (1,0) | S, stopped, 2000 t |
//! | 3   | dropped at the end of the morning of day 2 | no | Normal readiness | S | S | S |
//! | 4   | none | no | none | S | S | S |
//!
//! The staff gives 4, 1, 1 and 0 orders: 6 in all. On day 1 no speed order
//! is given: the force is at cruise. The straggler instruction and the
//! keep share in the rules of engagement order are those the force had.
//!
//! Each ship burns 60 t on each sortie and takes 60 t at S on the
//! afternoon of days 1 and 2. Base 1 holds 4880 t at the end of day 1 and
//! 4760 t from the end of day 2.
//!
//! The force records a halt at (3,0) at hour 7.5 and at hour 31.5, each
//! with the night as its work, and steams on at hours 12 and 36.
//!
//! Every order serves intent 2 and was decided at the level of task force
//! 1, with a chain of one: intent 1, the commander's. The reasons of days
//! 1 and 2 give the contact, 7 hexes from the station, 2000 t aboard
//! against 72 needed, and 7.5 hours to the station. The reasons of days 3
//! and 4 say there is no threat. No staff order is refused.
//!
//! # Properties
//!
//! - **Press.** With the risk of intent 1 set to press on day 1: the force
//!   sorties on all four nights, the break-off level ordered is 0.75, and
//!   it is never set back to normal readiness. Orders: 4, 1, 1, 1.
//! - **Preserve the force.** The contact reports 3 ships and the force has
//!   2: there is no threat that counts, no sortie on any day, and the
//!   reason gives 3 against 2. Day 1 has two orders, the home port and the
//!   rules of engagement with a break-off level of 0.25. With the contact
//!   reporting 2 ships, the force sorties on days 1 and 2, needing 90 t in
//!   each ship.
//! - **Fuel comes first.** With each ship holding 70 t and base 1 holding
//!   5000 t: on day 1 there is no sortie, and the reason gives 70 against
//!   72 and day 2 as the day from which the force may be used. The ships
//!   are filled in the night of day 1, and the force sorties on day 2.
//!   With base 1 holding no fuel, there is no sortie on any day, the
//!   reason gives no day, and the staff hands in on day 1 that it cannot
//!   meet intent 2.
//! - **Full speed where it must.** With the waters at (6,0), 240 nm: 15
//!   hours at cruise, and 7.5 at the class's 32 kn. The force is ordered
//!   to full speed and reaches the station 7.5 hours into the night. With
//!   the waters at (13,0), 520 nm, 16.25 hours at 32 kn: no sortie, and
//!   the notice.
//! - **Cover from the sea.** With the commander stating "hold base 1" with
//!   a forces limit of task force 1 in place of the deny intent: the stand
//!   task's station is S. The force is given no route on any day; battle
//!   readiness on day 1, normal on day 3; the home port and the rules of
//!   engagement on day 1. The contact is 10 hexes from S.
//! - **No force named, no task.** With no forces limit on intent 1, no
//!   stand task is formed and force 1 is given no order.
//! - **The rule alone.** A force of any mission routed through a waypoint
//!   marked to stand at halts there if it reaches it in the night and
//!   steams on at morning; if it reaches it in a daylight pulse it passes;
//!   and a route with no such mark moves as it did. A side asking the
//!   choice beforehand is told the same.
//! - **A held force is left alone,** and a commander's route order to
//!   force 1 holds it.
//! - The same start gives the same orders; two worlds that differ only in
//!   what the Allied side cannot see give the same intents, orders and
//!   reasons.
//! - Every scenario built before this one gives the events and end states
//!   it gave.
