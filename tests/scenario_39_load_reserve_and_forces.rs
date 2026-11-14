//! Scenario 39: a load up to a weight, a reserve at a base, and forces
//! formed, joined and stood down. A refuelling stops at the fuel reserve
//! with a second force refuelling the same morning; a load ordered by the
//! staff stops at the supply reserve and one ordered by the commander does
//! not; a weight is shared among a force's ships in ship order.
//!
//! Proves decision 0111 against a hand calculation. The data is placeholder
//! data: see `data/scenarios/39_load_reserve_and_forces.toml`. It runs on
//! the frozen step 1 figures. There is no enemy and no random draw.
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
//! A size 1 port has one berth, at 50 t an hour, and lighters at 20 t an
//! hour; a size 0 port works by lighter at 20 t an hour. Berths are given
//! out at the start of each pulse, to forces in number order, to a ship
//! that has cargo to move. The placeholder cargo ship: 3000 t of cargo,
//! 1000 t of fuel. The placeholder fast transport with cargo: 30 t of
//! cargo. A day is a night pulse of 12 hours and two daylight pulses of 6.
//! Forces stopped at a base of their side refuel at the start of a day, in
//! force order, before the day's orders.
//!
//! ## Inputs
//!
//! A placeholder map: S at (0,0), a coast hex with port size 1, and T at
//! (10,0), a coast hex with port size 0. The game is 4 days. Everything is
//! Allied.
//!
//! | Base | Hex | Supply | Fuel  | Supply reserve | Fuel reserve |
//! | ---- | --- | ------ | ----- | -------------- | ------------ |
//! | 1    | S   | 1000 t | 500 t | 400 t          | 300 t        |
//! | 2    | T   | 100 t  | 0     | 0              | 0            |
//!
//! | Force | Ships                                               | Where          |
//! | ----- | --------------------------------------------------- | -------------- |
//! | 1     | 1, cargo ship, 850 t of fuel                        | stopped at base 1 |
//! | 2     | 2, cargo ship, 900 t of fuel                        | stopped at base 1 |
//! | 3     | 3 and 4, fast transports with cargo, full of fuel   | stopped at base 2 |
//!
//! Ships 5 and 6, cargo ships full of fuel, lie in port at base 2 in no
//! force. No ship has cargo aboard.
//!
//! Orders:
//!
//! - Day 1: force 1 loads supply up to 250 t, by the commander's order.
//!   Force 2 loads supply up to 500 t, by an order whose origin is the
//!   staff. Force 3 loads supply up to 50 t. Ships 5 and 6 are formed into
//!   a force.
//! - Day 2: force 1 loads supply up to 450 t, by the commander's order.
//!   Force 3 is joined to the force formed on day 1.
//! - Day 3: that force loads supply up to 60 t.
//! - Day 4: that force is stood down.
//!
//! ## At S: a reserve
//!
//! Day 1, the refuelling. Base 1 holds 500 t of fuel and may give 200 t,
//! down to its reserve of 300.
//!
//! - Force 1: ship 1 wants 150 t and takes it. Base 1 holds 350 t.
//! - Force 2: ship 2 wants 100 t. Base 1 may give 50 t: ship 2 takes 50 and
//!   has 950 t. Base 1 holds 300 t, its reserve.
//!
//! Day 1, the loads. Base 1 holds 1000 t of supply; its reserve is 400.
//!
//! | Pulse     | Force 1, to 250 t, the commander's       | Force 2, to 500 t, the staff's              | Base 1 after |
//! | --------- | ---------------------------------------- | ------------------------------------------- | ------------ |
//! | Night     | Has the berth. 250 t in 5 hours: complete | By lighter: 20 x 12 = 240 t                | 510 t        |
//! | Morning   | Nothing to move                          | Has the berth. It may take 510 - 400 = 110 t: loaded in 2.2 hours. 350 t aboard. Complete: the stock it may draw on is gone | 400 t |
//! | Afternoon |                                          |                                             | 400 t        |
//!
//! In the night force 1 is worked first and leaves 750 t; force 2 may then
//! take up to 350 t and takes its 240. Force 2's load is complete with 350
//! t aboard, not 500: the staff's load never takes the stock below 400.
//!
//! Day 2, the refuelling. Base 1 is at its fuel reserve. Ship 2 wants 50 t
//! and takes none.
//!
//! Day 2, force 1's second order, the commander's. Force 1 has 250 t aboard
//! and is to have 450: it wants 200 t more. Base 1 holds nothing above its
//! reserve. The order is accepted, and its answer says it may take 200 t
//! below the reserve. In the night pulse it has the berth and loads 200 t
//! in 4 hours. Base 1 holds 200 t.
//!
//! At the end: base 1 holds 200 t of supply and 300 t of fuel. Ship 1 has
//! 450 t of supply and 1000 t of fuel. Ship 2 has 350 t of supply and 950 t
//! of fuel.
//!
//! ## At T: a weight shared, and forces formed, joined and stood down
//!
//! Day 1. Force 3 loads up to 50 t, by lighter, both ships at once. Ship 3
//! is filled first: 30 t, its room, in 1.5 hours. Ship 4 takes the other 20
//! t in 1 hour. Base 2 holds 50 t. The load is complete.
//!
//! Ships 5 and 6 are formed into a force. It takes the first number of the
//! side's made series (decision 0086), and the side sees it as its next
//! force: stopped at base 2, with no mission, at cruise speed, with base 2
//! as its home port.
//!
//! Day 2. Force 3 is joined to the formed force. The formed force has
//! ships 3, 4, 5 and 6, with the 30 t and 20 t aboard ships 3 and 4. Force
//! 3 has ceased to be, and an order to it is refused.
//!
//! Day 3. The formed force loads up to 60 t. It has 50 t aboard and wants
//! 10 more. Its ships are taken in ship order: ship 3 is full; ship 4 has
//! room for 10 and takes them, in half an hour. Ships 5 and 6 take nothing.
//! Base 2 holds 40 t.
//!
//! Day 4. The formed force is stood down. Ships 3, 4, 5 and 6 lie in port
//! at base 2 in no force, with 30, 30, 0 and 0 t aboard. The formed force
//! has ceased to be.
//!
//! # Properties
//!
//! - With no weight, no reserve and none of the three orders, every
//!   scenario gives what it gave before, apart from the hash version.
//! - A load with a weight never puts more aboard a force than the weight,
//!   and never more in a ship than its room.
//! - Refuelling never takes a base's fuel below its reserve, and a load
//!   whose origin is the staff never takes its supply below its reserve,
//!   on any day of a run in which the men's supply is also drawn from the
//!   base.
//! - Where a side has two bases in a hex, a force that refuels draws on
//!   them in base order, each down to its reserve.
//! - Forming, joining and standing down are each refused at sea, at a base
//!   of the other side, for a force that is under way, and for a force
//!   away at the yard; a join is refused between forces at different bases.
//! - The enemy's view is the same in two worlds that differ only in which
//!   of the other side's ships are in which force at a base.
