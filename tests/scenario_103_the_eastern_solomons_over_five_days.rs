//! Scenario 103: the Eastern Solomons over five days, 23 to 27 August
//! 1942. Three Japanese carriers against two American and an airfield,
//! with a Japanese convoy that must reach Guadalcanal. It has no scripted
//! order: each side is played by a commander or a staff.
//!
//! Proves decision 0184 as amended and confirmed. The scenario comes as
//! four files made by one script from one base, in
//! `data/scenarios/five-days/`, which is kept out of the loops that walk
//! `data/scenarios/*.toml` (decision 0072):
//!
//! - `01a_eastern_solomons.toml`: both sides given the orders below;
//! - `01b_eastern_solomons.toml`: the Japanese side given its orders, the
//!   Allied side none (a person or an agent takes the Allied seat);
//! - `01c_eastern_solomons.toml`: the Allied side given its orders, the
//!   Japanese side none;
//! - `01d_eastern_solomons.toml`: neither side given orders.
//!
//! They differ in their `[[command]]` tables and their names and in
//! nothing else. It runs on the live figures.
//!
//! # Hand calculation
//!
//! Written by the architect before the scenario was built. Everything in
//! this section is asserted exactly. If the code or the file disagrees,
//! one of them or the rule is wrong: do not change a number here to make
//! a test pass. A mismatch goes back to the architect.
//!
//! What the staffs then do with it, day by day, is not worked here: it
//! is reported by the build for the architect to state (see "Reported").
//!
//! ## The map's arithmetic
//!
//! From the header of `data/map/south_pacific.csv`: east in nm is
//! (longitude - 160) x 60 x 0.978148; north in nm is (latitude + 12) x
//! 60, south latitude negative. The centre of hex (q, r) is 40 x (q +
//! r/2) east and 34.641016 x r south. So r = -north / 34.641016 and q =
//! east / 40 - r / 2, rounded to the nearest hex. The distance in the air
//! between two hexes is 40 x the square root of (dq x dq + dr x dr + dq x
//! dr), rounded to a whole mile.
//!
//! ## Where each force starts, and why
//!
//! The game begins at 1800 on 22 August; day 1 is 23 August. No row
//! places any force on the map on the 22nd. Each force starts where a row
//! first places it on the 23rd, or where scenario 5 has it; every start
//! is a placeholder for the evening before, and is marked so in the file
//! with its row.
//!
//! | Force | Hex | From |
//! | --- | --- | --- |
//! | Task forces 1 and 2, the American carriers | (6,-3) | position row 262: Task Force 61 at 10 S, 163 E at 1040 on the 23rd |
//! | Task force 3, the carrier striking force | (9,-18) | position row 413: 1.8 S, 160.2 E at 0600 on the 23rd, read off a chart, of low confidence |
//! | Task force 4, the Ryujo detachment | (7,-18) | scenario 5's own start for it |
//! | Task force 5, the convoy | (6,-12) | position row 291: 250 miles north of Guadalcanal at 1040 on the 23rd, taken as due north of Lunga Point (9.42806 S, 160.05472 E). Rows 294 and 352 say 200 and 350 miles |
//!
//! The arithmetic:
//!
//! - **10 S, 163 E:** east 3 x 60 x 0.978148 = 176.07; north 2 x 60 =
//!   120; r = -3.46, q = 4.40 + 1.73 = 6.13: hex (6,-3).
//! - **1.8 S, 160.2 E:** east 0.2 x 60 x 0.978148 = 11.74; north 10.2 x
//!   60 = 612; r = -17.67, q = 0.29 + 8.83 = 9.13: hex (9,-18).
//! - **250 nm north of Lunga Point:** latitude -9.42806 + 250/60 =
//!   -5.2614; east 0.05472 x 60 x 0.978148 = 3.21; north 6.7386 x 60 =
//!   404.32; r = -11.67, q = 0.08 + 5.84 = 5.92: hex (6,-12).
//!
//! The places are the 39 days': Lunga, base 1, at (2,-4); Taivu at
//! (3,-4); Tulagi at (3,-5); Rabaul at (-5,-13).
//!
//! ## Distances, in the air
//!
//! | From | To | dq, dr | Miles |
//! | --- | --- | --- | --- |
//! | The convoy, (6,-12) | Taivu, (3,-4) | -3, 8 | 40 x root 49 = 280 |
//! | Lunga, (2,-4) | the convoy's start | 4, -8 | 40 x root 48 = 277 |
//! | The American carriers, (6,-3) | Lunga | -4, -1 | 40 x root 21 = 183 |
//! | The American carriers | Taivu | -3, -1 | 40 x root 13 = 144 |
//! | The American carriers | the convoy's start | 0, -9 | 360 |
//! | The American carriers | the carrier striking force, (9,-18) | 3, -15 | 40 x root 189 = 550 |
//! | The carrier striking force | the convoy's start | -3, 6 | 40 x root 27 = 208 |
//! | Rabaul, (-5,-13) | Lunga | 7, 9 | 40 x root 193 = 556 |
//!
//! ## What can reach what on the first evening
//!
//! With the live aircraft figures (search and strike radius in miles:
//! SBD-3 397 and 200; TBF-1 420 and 190; F4F-4 105 and 105; G4M1 611 and
//! 611; A6M2 from a base 556 and 556; from a carrier 261 and 261; D3A1
//! 252 and 252; B5N2 358 and 175):
//!
//! - **Lunga's dive bombers** can search to the convoy's start (277 is
//!   within 397) and cannot strike it there (277 is beyond 200).
//! - **The American carriers** can search to the convoy's start (360 is
//!   within 397 and 420) and can strike a ship off Lunga or Taivu (183
//!   and 144 are within 200 and 190). They can neither search nor strike
//!   to the carrier striking force (550).
//! - **Rabaul's bombers and fighters** can reach Lunga (556 is within
//!   611, and is the fighters' radius exactly).
//! - **Neither side's carriers can strike the other's** on the first
//!   evening.
//!
//! ## The convoy
//!
//! Task force 5, on the transport mission. Its men are the reading of
//! troop lift rows 8 and 9: 700 army and 800 naval, 1,500 in all (row 7
//! agrees on the total; the ships' own rows give about 1,100 and about
//! 800, which the file notes and does not use).
//!
//! - **Three placeholder troop transports** (the 39 days' class 10: 10 kn
//!   cruising, 12 kn at full speed, 600 men), named "Placeholder
//!   transport" 1 to 3. No real transport is given a rating.
//! - **Four destroyers the rows name with the convoy and the game rates:**
//!   Kagero, Kawakaze, Umikaze and Suzukaze. Jintsu, the four patrol
//!   boats, Isokaze, Mutsuki and Yayoi are left out, and the file says
//!   so.
//! - **Three land units aboard,** fighting units: 350 and 350 men of the
//!   Ichiki Detachment's second echelon on transports 1 and 2, and 800 of
//!   the Yokosuka 5th Special Naval Landing Force on transport 3.
//! - **Supply aboard:** 30 t on each transport, 90 t, a placeholder (no
//!   row gives a figure): 1,500 men x 0.005 t x 12 days.
//!
//! The force's speed is its slowest ship's: 10 kn cruising, 12 at full
//! speed. The passage to Taivu is 280 miles: 28 hours at 10 kn, 23 hours
//! 20 minutes at 12.
//!
//! **Landing where there is no port.** A hex has port size 0: lighters at
//! 20 t an hour, and for men landing unprepared 0.13 of that, 2.6 t an
//! hour for each ship. A man is 0.02 t, so a ship lands 130 men an hour.
//! Transports 1 and 2 land their 350 men in 350 / 130 hours, 2 hours 42
//! minutes; transport 3 its 800 in 6 hours 9 minutes.
//!
//! ## The rest of the forces
//!
//! - **Task forces 1 to 4** are scenario 5's, ship for ship: 10, 8, 14
//!   and 4 ships. Their air groups are scenario 5's, group for group,
//!   each at the strength scenario 5 gives it.
//! - **Lunga,** base 1, Allied, with its airfield. Its squadrons are at
//!   the strength of the rows of 20 August that the 39 days uses: 19
//!   fighters of VMF-223 and 12 dive bombers of VMSB-232.
//! - **The Marines on Guadalcanal** are the 39 days' four land units
//!   there, 4,306, 4,561, 1,373 and 660 men, 10,900 in all, ashore at
//!   Lunga on the defence.
//! - **Lunga's supply** is 1,000 t, a placeholder: no row gives the stock
//!   on the 22nd. The men alone eat 10,900 x 0.005 = 54.5 t a day, 272.5
//!   t in five days, so the stock does not run out in the game by their
//!   eating.
//! - **Taivu** is a Japanese stock with no supply and one land unit of
//!   100 men, from land unit row 270 ("little more than 100", a floor),
//!   on the defence.
//! - **Rabaul:** the land bombers and the fighters the 39 days has there,
//!   at its strengths, and its supply.
//! - **Tulagi** is Allied, with the 39 days' units there, and is given
//!   no part.
//!
//! ## What a side the engine plays is given
//!
//! On day 1, as commands of the scenario:
//!
//! - **Japanese:** hold Taivu, its forces the convoy and its three land
//!   units; neutralise Lunga, its forces the bombers at Rabaul; operate
//!   task force 3 from (8,-11) and task force 4 from (7,-9), each with
//!   its own air groups; leave to prepare and leave to launch given.
//! - **Allied:** hold Lunga, its forces the four land units and the two
//!   squadrons; operate task force 1 from (6,-3) and task force 2 from
//!   (7,-3), each with its own air groups; leave to prepare and leave to
//!   launch given.
//!
//! No risk is set: every order is at balanced.
//!
//! ## The objectives
//!
//! The game's one scale, as scenarios 5 and 7 state it. Levels: minor 3,
//! major 8, decisive 16. A horizon of 30 days.
//!
//! | Kind | Allied | Japanese |
//! | --- | --- | --- |
//! | Ships sunk: carrier 10, battleship 8, cruiser 4, destroyer 1, transport 2, auxiliary 1 | yes | yes |
//! | Ships out of action, over the horizon: carrier 5, battleship 4, cruiser 2, destroyer 0.5, transport 1, auxiliary 0.5 | yes | yes |
//! | Aircraft destroyed, 0.05 each | yes | yes |
//! | Enemy men lost, 1 for each 100 | yes | yes |
//! | Lunga open, 10 points over the days | open | shut |
//! | Supply landed at Taivu, 0.005 a ton | | yes |
//!
//! # Properties
//!
//! - **Each file loads and plays five days,** and the same seed gives
//!   the same daily hashes twice.
//! - **The four files are one scenario:** with their `[[command]]`
//!   tables and names taken away they are the same, byte for byte; and
//!   the script that makes them, asked to check, says they are what it
//!   makes.
//! - **The forces are scenario 5's:** every ship of task forces 1 to 4
//!   and every air group of theirs has the class, the type and the
//!   strength scenario 5 gives it.
//! - **Every start is where the calculation puts it,** and every
//!   distance and reach above is what the game's own measure gives.
//! - **Nothing waits for a later day:** no unit has a day from which it
//!   may be used.
//! - **A side given no orders does nothing of its own:** in file d, with
//!   both seats the engine's, no order is given by either staff in five
//!   days.
//! - **A side given its orders acts on the first morning:** in file a,
//!   on day 1 the Japanese staff gives orders to the convoy and to the
//!   bombers at Rabaul, and each side's staff to its carrier forces.
//! - **It can be played from a seat:** file b with the Allied seat
//!   played and file c with the Japanese seat played each open, list
//!   the tools of command, and answer `trial`.
//! - **Scenario 5 is untouched:** its file, its frozen copy and its
//!   variants are what they were.
//!
//! # Reported, for the architect to state
//!
//! Not asserted here; given in the build's report and in
//! `docs/validation/`:
//!
//! - In file a on the scenario's seed, day by day: when the convoy
//!   sails, when and where it lands and how many men and tons, whether it
//!   is found and struck and by whom; what the bombers at Rabaul do; where
//!   each carrier force goes and whether the carriers find and strike one
//!   another; Lunga's supply and whether it is open each day; and the
//!   result.
//! - **The first run:** 1,000 games of file a, staff against staff: how
//!   often each side wins and at which level, and the spread of each
//!   count behind the points (carriers sunk and out of action each side,
//!   transports sunk, men and tons landed at Taivu, men lost each side,
//!   aircraft lost each side, days Lunga was open).

// The tests are written by the build, to the statements above.
