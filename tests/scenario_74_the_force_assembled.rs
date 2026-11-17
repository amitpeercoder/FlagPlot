//! Scenario 74: the force assembled by the staff. The commander says
//! "take that airfield" and names nothing else. The staff weighs two
//! bases it could sail from, takes the nearer, picks the unit, the
//! transports and a battleship to shell the place on the night of the
//! landing, and takes it.
//!
//! Proves decision 0149, step 7 of stage 3, against a hand calculation.
//! It stands on scenario 69's for the landing and scenario 71's for the
//! bombardment. The data is placeholder data: see
//! `data/scenarios/74_the_force_assembled.toml`. It runs on the frozen
//! step 1 figures, with one ground figure of its own,
//! `preparation_per_day` 100, and scenario 71's three bombardment figures
//! (5 minutes, no hit, no false claim). No enemy man is ashore anywhere,
//! so no die changes anything.
//!
//! # Hand calculation
//!
//! Written by the architect before the code. Everything is asserted
//! exactly. If the code disagrees, the code or the rule is wrong: do not
//! change a number here to make a test pass. A mismatch goes back to the
//! architect.
//!
//! ## Inputs
//!
//! The placeholder map of scenario 27: A at (0,0), port size 5; B at
//! (4,0), port size 0; C at (0,6), port size 0. A to B is 4 hexes,
//! 160 nm; C to B is 6 hexes, 240 nm. The game is 6 days.
//!
//! | Base | Side | Hex | Supply | Fuel |
//! | ---- | ---- | --- | ------ | ---- |
//! | 1, "Placeholder base A" | Allied | A | 1,000 t | 5,000 t |
//! | 2, "Placeholder airfield B" | Japanese | B | 0 | 0 |
//! | 3, "Placeholder base C" | Allied | C | 1,000 t | 0 |
//!
//! Base 2 is on the Allied map and has no gun, aircraft or man. The
//! Allied side has an opening estimate of 300 enemy men at B.
//!
//! | Task force | Mission | Ships | Stopped at |
//! | --- | --- | --- | --- |
//! | 1 | transport | 1 and 2, placeholder troop transports, 600 berths each | base 1 |
//! | 2 | transport | 3, a placeholder troop transport | base 3 |
//! | 3 | bombardment | 4, a placeholder battleship, as scenario 71's | base 1 |
//!
//! | Unit | Where | Men | Experience | Assault skill |
//! | ---- | ----- | --- | ---------- | ------------- |
//! | 1, "Placeholder First" | A | 600 | 2 | 2 |
//! | 2, "Placeholder Second" | A | 600 | 2 | 2 |
//! | 3, "Placeholder Assault" | C | 600 | 2 | 4 |
//!
//! All are Allied fighting units, fed in full.
//!
//! The Allied side's commands of day 1: intent 1, take the place at base
//! 2, with no forces limit; no risk set, so balanced. Leave to prepare
//! and leave to launch are both given.
//!
//! ## The choice, on the morning of day 1
//!
//! - **The enemy.** 300 men, as told, of day 0. At balanced the men
//!   wanted are 600.
//! - **The source.** Base 1: task force 1 has 1,200 berths; unit 1 takes
//!   ship 1 and gives 600 men, the men wanted; the passage is 16 hours.
//!   Base 3: task force 2 has 600 berths; unit 3 gives 600 men; the
//!   passage is 24 hours. Both give the men wanted, and base 1 is the
//!   nearer: the source is base 1.
//! - **The units.** Unit 1.
//! - **The transport force.** Task force 1.
//! - **The risk on the way.** Task force 3, on the bombardment mission,
//!   is taken. There is no carrier force and no force on the escort
//!   mission, and the reason says so.
//!
//! The limit the staff sets on the task it forms: task forces 1 and 3,
//! unit 1, and base 1 as the source.
//!
//! ## Day by day
//!
//! | Day | Hour | The staff's commands and orders | What happens |
//! | --- | --- | --- | --- |
//! | 1 | 0 | Approves plan 1 to preparing. Forms intent 2, ready a landing by task force 1 at B, and sets its forces limit as above. Orders `prepare_landing` at B, roster 1 | Loading 100; unit 1's rehearsal 100 |
//! | 2 | 24 | Notice: intent 2 is ready. Approves plan 1 to released. Orders a load of supply to a weight of 45 t. Forms intent 3, bombard base 2 with task force 3, under intent 1. Task force 3: home port, base 1; rules of engagement, engage, break-off level 0.5 | Ship 1 loads 45 t |
//! | 3 | 48 | Unit 1 set to attack. Load unit 1 | Ship 1 takes 600 men |
//! | 4 | 72 | Task force 1: a route to B and back to A, acting at B, and an unload order. Task force 3: battle readiness; a route to B and A, B marked to act at | The battleship sails at hour 72, halts at B at hour 82 and fires 120 shells in the night. The transports sail at hour 72 and arrive at hour 88. Ship 1 lands 160 men in the morning and 440 in the afternoon, and then 10 t. Base 3, "Landing at (4,0)", is formed. Base 2 changes side at the end of the day |
//! | 5 | 96 | Marks plan 1 complete; notice: intent 1 is met. Task force 3: normal readiness | Ship 1 lands 35 t in 1.75 hours. The transports steam for A at hour 108 and arrive at hour 124. The battleship steamed for A at hour 84 and arrived at hour 94 |
//! | 6 | 120 | none | |
//!
//! The landing force is due on the day it sails, day 4: the battleship
//! goes that morning. The supply is 600 x 0.005 x 15 = 45 t: no hold is
//! stated, so nothing follows.
//!
//! Eleven orders in all: six to the landing and five to the battleship.
//! Six staff commands. Units 2 and 3 and task force 2 are given no order.
//! The Allied men find B empty at the end of day 4 and the estimate is
//! cleared. No staff order is refused.
//!
//! # Properties
//!
//! - **The farther base, when the nearer cannot.** With task force 1
//!   held from day 1: base 1 has no transport force the staff may take.
//!   The source is base 3. On day 1 the staff forms "ready a landing" by
//!   task force 2 at B and sets its forces limit to task forces 2 and 3,
//!   unit 3 and base 3 as the source.
//! - **The commander names the source.** With a forces limit on intent 1
//!   that names base 3 as the source and nothing else: the choice of day
//!   1 is that of the property above, but the limit the staff sets names
//!   no source: the commander's stands.
//! - **The commander names a unit.** With a forces limit that names unit
//!   2 alone: the source is base 1, the unit is unit 2, and the limit the
//!   staff sets names task forces 1 and 3 and base 1 and no land unit.
//! - **A unit another intent has.** With a commander's intent of day 1,
//!   ranked above intent 1, whose forces limit names unit 1: the staff
//!   takes unit 2, and the reason says unit 1 serves the other intent.
//! - **Short of men, at balanced.** With unit 2 left out and an opening
//!   estimate of 500: the men wanted are 1,000. Base 1's lift gives 600
//!   and base 3's gives 600: neither is enough, they are equal, and base
//!   1 is the lower numbered. The staff goes with unit 1 and says it is
//!   short by 400 men.
//! - **Short of men, at preserve the force.** The same at preserve the
//!   force: the men wanted are 1,500. The staff forms nothing and hands
//!   in the notice that it cannot meet intent 1 on each morning, with
//!   600 men against 1,500.
//! - **Nothing to sail in.** With task forces 1 and 2 held from day 1:
//!   no base can be the source. The staff forms nothing and hands in the
//!   notice that it cannot meet intent 1 on each morning, saying that no
//!   transport force it may take lies at a base of the side.
//! - **With all three named** (task forces 1 and 3, unit 1, base 1): the
//!   orders and commands are those of the main case, and the limit the
//!   staff sets on intent 2 names unit 1 alone, as decision 0142 has it.
//! - The same start gives the same orders; two worlds that differ only in
//!   what the Allied side cannot see and has not been told give the same
//!   orders and reasons; a save on day 3 and a resume give the same game;
//!   scenarios 67 to 73 give the orders they gave.
