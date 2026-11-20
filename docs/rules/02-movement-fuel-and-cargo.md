# Movement, fuel and cargo

This chapter covers where ships can go, how a task force moves, what it burns, how it refuels and loads, and what becomes of its damaged ships. Every quantity is a fixed-point number, and "cut" below means cut toward zero at six decimals (chapter 1, Numbers and chance).

Where these rules run in a day (chapter 1, The order in which a day resolves, gives the whole of it):

| When | Rule |
| --- | --- |
| Start of the day, after the staff is asked | Orders are taken in, the commander's and the staff's, including route, speed, readiness, cargo, detach, form, join and stand-down orders (chapter 1, Orders; chapter 8) |
| Start of each pulse | Alert from belief, then fatigue of the task forces (chapter 3, Readiness, alertness and fatigue of a force) |
| Movement phase of each pulse | Halted forces whose work is done steam on, then stragglers, then escorts keep station (Keeping station), then movement, meetings and fights |
| Loading phase, last in each pulse | Cargo handling, then refuelling in port, then rearming, from one queue of berths (decisions 0112 and 0122) |

## The map as the rules read it

### Hex kinds

Every hex on the map has one of four kinds: deep sea, shallow sea, coast or land. A hex is navigable, which means a ship may enter it, if it is deep sea, shallow sea or coast. Land is not navigable. A hex that is not on the map is not navigable. The generated map uses deep sea, coast and land. Nothing marks a hex shallow sea yet. [0017]

A side of a hex may be closed. A closed side is stored for each of the six directions of a hex. A ship cannot cross it. [0017]

A side sees: the map is common knowledge. Neither side's events change it.

### The six directions

Hexes are pointy-topped, in axial coordinates (q, r), 40 miles between centres. The six neighbours of a hex, in the fixed order every tie is broken in, are east (1,0), north-east (1,-1), north-west (0,-1), west (-1,0), south-west (-1,1) and south-east (0,1). North is the direction of falling r. The 40 miles is a constant in `hex.rs`. The order is also fixed in `hex.rs` and changing it changes every route. [0005, 0017]

A side sees: not an event.

### What a ship may enter

A ship steps from one hex to a neighbour only if both hexes are on the map and navigable and neither hex marks the side between them closed. Either hex's mark is enough to close it, so an error in one row of the map file cannot open a side. The test is symmetric: a step from A to B is open exactly when the step from B to A is. A direction above 5 is not a step. [0017]

A side sees: not an event.

### Land links

A map lists its land links: pairs of neighbouring hexes that men can march between (chapter 7, Marching overland, and A beaten side's men, which fall back along them). A link joins two hexes that are neighbours, are on the map and are each land or coast, and it runs both ways. A link is listed once. The map does not work out where men can walk from the kinds of hex: a hex of 40 miles can hold parts of two islands, so a link says that one landmass runs through both hexes and that the game treats the men in each as being on it. A map with no links allows no march, so no scenario on such a map changes. The links are map data: they are not part of the state and are not hashed. [0094 items 1 and 2]

A map file in the placeholder TOML form lists them as `[[land_link]]` entries with `a = [q, r]`, `b = [q, r]` and an optional `note` for the reader. A scenario may also name a links file with the key `land_links`, a path relative to the scenario file, whatever the map's format. It is a CSV with the header `q1,r1,q2,r2,landmass,reason`, one link on each row, and lines that start with `#` are comments. Each row needs a landmass, which says which landmass the link is for, and a reason. The links of the file are added to those of the map. The loader refuses a link between hexes that are not neighbours, a hex that is off the map or is neither land nor coast, a link listed twice (in one file or across the map and the file), a row without a landmass or a reason, and a header other than the one above. The file is a data file of the scenario. The real map's links file is `data/map/south_pacific_land_links.csv`, which the full slice names: it joins the airfield's hex at Lunga, (2,-4), to (3,-4) to its east and to (2,-5) to its north-west, and nothing else, so Tulagi's hex is reached by sea only. [0094 items 1 and 2, 0101 item 5]

A side sees: not an event. The map, and so every link, is common knowledge.

### Bases on hexes

A base is on one hex. A hex may hold several bases, of one side or of both. A base has a side, a supply stock and a fuel stock, both in tons (see Cargo and Refuelling). The scenario's base entry gives the stocks (`supply_t`, `fuel_t`), the base's anti-aircraft shots (`anti_aircraft`, chapter 5) and the damaged aircraft it repairs each day (`aircraft_repairs_per_day`, chapter 5), each 0 if not given. It also gives `ammunition_t`, the tons of ammunition it holds for ships' guns and torpedo tubes (see Rearming in port), 0 if not given (the loader refuses a negative value and a TOML float). It also gives two keys for the reserves it keeps (see Cargo orders and Refuelling in port): `supply_reserve_t` and `fuel_reserve_t`, each a number of tons, a whole number or a decimal string, 0 if not given (the loader refuses a negative value and a TOML float). It also gives two keys for ship repair (chapter 4, Repair): `ship_repair_points_per_day`, the points of ship damage the base mends each day for the forces of its side stopped there (a whole number or a decimal string; 0 if not given; the loader refuses a negative value), and `yard_passage_days`, which makes the base a port for the yard with a passage there and back of that many days (a whole number of at least 1; none if not given; the loader refuses 0, a negative or a fractional value). A TOML float is refused for either. It also gives three keys for what units build (chapter 7, Units that work and what they build): `airfield`, `true` if not given, `airfield_work`, the work already done on an airfield, 0 if not given and allowed only with `airfield = false`, and `defences`, from 0 to 100, 0 if not given. A base has an airfield or it does not: a base the scenario states has one unless it says otherwise, and a base formed by a landing or a march has none until work builds one. Rebasing an air group and landing elsewhere read it (chapter 5). A base must be on the map. It need not be on a navigable hex, but a force can use a base only if it can be at the base's hex, so a base on land serves nobody. Where a rule looks for a base of a side at a hex, it takes the lowest-numbered base that matches. A base can also be formed in play: the first man a force lands in a hex where its side has no base forms one, with the next free number, the port of the hex, no stock, no guns and no repairs. It is on its own side's map only (chapter 7, A base formed by a landing). A base's side does not change in the movement and cargo rules; it changes only when its side is beaten in its hex, in the ground combat phase after the third pulse. The base keeps its number, name, hex, guns and repair figures, passes to the other side with part of its stocks and is on both sides' maps from then on (chapter 7, A base that changes side). Every rule that asks whose a base is reads the side it has at that moment, so from the next day cargo, refuelling and repair at it serve its new side. Its two reserves stay with it when it changes side, and are kept by the side that takes it, as numbers of no meaning to that side until it sets its own. Its ammunition stays with it whole: only supply and fuel are cut to the captured share (chapter 7, A base that changes side), and the decision that built rearming says nothing of a base's ammunition changing hands. [0011, 0045, 0066, 0069, 0090, 0111, 0122]

A ship that is not in a task force may be in a port, which is a base. Such a ship does not move, refuel, work cargo or take ship repair: every rule here acts on task forces. Its position is the base's hex, and an air group aboard it uses that position. A scenario may start a ship so, and a force stood down in port leaves its ships so (The task force formed, joined and stood down). The word for it is "in port in no force". [0025, 0111]

A side sees: bases are known to their own side. Chapter 1, What a side may know, gives what a side is told of the enemy's.

### Port size

A hex has a port size from 0 to 10. A base's port is the port of the hex it stands on. `port_sizes.csv` gives, for each size, `berths`, `berth_rate_t_per_h` and `lighter_rate_t_per_h`, and two figures for fuel, `fuel_berth_rate_t_per_h` and `fuel_lighter_rate_t_per_h` (the fuelling rates: tons of fuel an hour one ship takes at a berth, and by lighter), and two for ammunition, `ammunition_berth_rate_t_per_h` and `ammunition_lighter_rate_t_per_h` (the rearming rates: tons of ammunition an hour one ship takes at a berth, and by lighter). A column the file lacks is empty. The file must have a row for the port size of every hex that holds a base, or the game does not start. A row may not have berths and no berth rate. A fuelling rate or a rearming rate is empty, which is not given, or above 0 (the loader refuses 0 and a negative rate). Where a fuelling rate is not given, fuelling by that means has no limit (Refuelling in port); where a rearming rate is not given, no ship rearms by that means (Rearming in port): the two differ. Only cargo handling, refuelling and rearming read port size. The generated map gives every hex port size 0, so every base on it works cargo by lighter. The live figures give fuel at a berth 200 t an hour for sizes 1 to 10 (size 0 has no berth) and fuel by lighter 130 t an hour for sizes 0 to 10, one value for every size, sized from rows and not checked against any battle (decision 0112, The first figures). The live figures give ammunition at a berth 20 t an hour for sizes 1 to 10 (size 0 has no berth) and by lighter 10 t an hour for sizes 0 to 10, placeholders: no rate was found. The frozen step 1 figures (`data/reference/step1`) give no fuelling rate (no limit) and no rearming rate (no rearming). [0011, 0052 item 15, 0112, 0122]

A side sees: not an event.

### What reads nothing

Airfield size is stored on a hex and no rule reads it. Terrain is stored and only the march of a land unit reads it, for the unit's movement skill (chapter 7, Skills, decision 0098): the generated map marks every land hex jungle. The anchorage flag on a hex is stored and no rule reads it. Shallow sea is treated as sea. [0052 items 14 and 15]

## Task forces and ships

### A task force

A task force is a set of ships of one side, with one position, one route, one speed setting and one set of standing orders. Its fields that this chapter reads:

| Field | Meaning |
| --- | --- |
| `ships` | The ships in it. A ship afloat is in exactly one task force, one port or away at the yard. A sunk ship is in none. A force away at the yard lists none |
| `from_hex`, `toward_hex`, `progress_nm`, `leg_nm` | Where it is on a leg, and what the leg is charged (see What a step is charged and Legs and position) |
| `route` | Waypoints, the index of the next one, and its marks: the hexes whose waypoints are marked to act at (`act_at`) and those marked to stand at for the night (`stand_at`); none for a route with no mark, and the last waypoint is never marked. See Halting at a marked waypoint and steaming on |
| `status` | idle (no route), underway, arrived (reached the last waypoint), out of fuel, or halted (stopped at a marked waypoint, holding a halt) |
| `speed` | cruise, full, or a speed in knots |
| `readiness`, `alerted`, `fatigue` | See Readiness and fatigue |
| `rules_of_engagement` | Stance, break-off level, straggler instruction and share to keep |
| `home_port` | A base of its own side, set by order |
| `cargo_order` | The standing cargo order, if any |
| `charge` | Set by the escort order, on a force on the escort mission only: the force it keeps station on, and the order that gave it. None otherwise (Keeping station) |
| `mission` | Set by order. Two rules of the engine read it: a force on the escort mission may be given a charge (Keeping station), and a force on the bombardment mission fires at an enemy base (chapter 4, Shore bombardment), and it is what a bombardment halt is for (Halting at a marked waypoint and steaming on). The staff reads it too, to know which task to form for a force: surface combat for a stand task, carrier for a carrier task (chapter 8, A task force as an addressee). It is copied by a split. [0092, 0096, 0121, 0124, 0125] |
| `halt` | Set while the status is halted, and only then: the work it halted for (a bombardment, cargo, or both, or the night), the hour it halted, its `TaskForceHalted` event, the bombardment it has fired since and the completion of its cargo order since, each if any. See Halting at a marked waypoint and steaming on. [0096, 0124] |
| `yard_trip` | Set while the force is away at the yard: its port, its day of return and the event that sent it. None otherwise (chapter 4, Repair) |

A task force with no ship afloat is never ordered, never moves and never fights. Every order for it is rejected with "no ship afloat". A force that was joined to another or stood down in port has no ship either, and has ceased to be: it stays in the state, as a force whose ships were all sunk does, and its number is never used again. An order to it is refused with its own reason, "the task force no longer exists: it was joined to F" or "... it was stood down in port", read from the log (chapter 1, Every reason an order is refused). [0016 item 16, 0044, 0111]

A force that is **held apart** until the day a scenario gives it (`enters`; chapter 1, A unit not yet entered) is not present: it is not in the state's task forces or ships, so it is in no hex, and every rule that walks the forces, the ships and the bases' cargo skips it. It is not searched for, sighted, fought, refuelled, tired or fed, burns no fuel and works no cargo. Its ships are not afloat on the map and are in no count. On its day it enters at the hex the scenario gave it, stopped, with all aboard it, and is a force like any other. It is not the same as a force away at the yard, which is on the map in its own way, below. [0195]

A force that is **away at the yard** (chapter 4, Repair) has no ship in its list, because its ships are away. It is not sunk. It stays at its base's hex, idle, but no rule moves it, fights it, sights it, searches for it, refuels it or tires it: every rule that walks the forces skips a force with no ship. It takes no order until its day of return (chapter 1, Every reason an order is refused). Its ships are afloat and are counted as such (chapter 6). [0069]

What the enemy knows of a force away: nothing new. Nothing can sight a force with no ships, so the enemy's contact of it is not refreshed. The contact ages and is dropped by the existing rule (`search.toml: contact_drop_age_pulses`) as for any force that steams out of sight, and no report is made of the departure or the return. The enemy's events in a game in which a force goes to the yard are those of the same game with the force left stopped in port, as long as nothing of the enemy's would have sighted it there. [0069]

A scenario's `[[ship]]` entry may also start a ship damaged: `flotation_damage` and `fire_damage` (chapter 4, Damage tracks), each 0 if not given. A ship's bank of forward repair points and a force's `yard_trip` cannot be given by a scenario: every game starts with no work in hand and no force away. [0069]

A side sees: its own task forces exactly. The enemy's task forces reach a side only as contacts (chapter 3, Belief).

### Detaching ships by order

A detach order splits named ships off into a new task force. Chapter 1, Task force orders, gives the order, its checks and what the new force copies. Stragglers are detached by the same split, with changes (see Stragglers below).

A side sees: its own `ShipsDetached` event whole. The enemy sees nothing.

### The task force formed, joined and stood down

Three orders change which ships are in which force at a base of the side. Each is checked and done when it is taken in, at the start of the day, and records its event after the acceptance, citing the order. An order to the force a forming order makes may be given the same day: forming orders are taken in before every other order of the day, so the new force exists for the orders that follow it, and a seat's orders queued through the agent are checked beside each other for it (chapter 1, When an order is checked, and against what). A staff's forming order and its orders to the new force are checked as a staff's orders are, against the forces limit of the intent they serve (chapter 8, The forces limit on a staff's orders). Chapter 1, Every reason an order is refused, gives the refusals. There is no randomness. [0111]

**Forming.** A base order, `form_task_force` with a list of ships (in a scenario an `[[order]]` with `base = N`), forms ships of the side that lie in port at the base in no force into a new task force. Orders that form a force are taken in before every other order of the day (chapter 1, How orders are numbered and sorted). The new force takes the next number of its side's made series (chapter 1, What a side may know), as a force split off does, and is its side's next force by its own numbering. It is stopped (idle) at the base's hex, with no route, mission transport (no rule but bombardment reads a mission), cruise speed, the rules of engagement and readiness a force has when a scenario gives it none, and the base as its home port. It has no cargo order. An air group aboard a ship goes with her: its place is the ship, as for any ship of a force. `TaskForceFormed` names the force, the base and the ships. [0111, 0086]

**Joining.** A force order `join`, with `into` naming another force of the side, moves every ship of the force into the other, which is the receiving force. Both forces are stopped at a base of the side, and at the same base: idle or arrived, not under way, not halted with a route to follow, not out of fuel, and at the centre of a hex (not part-way along a leg) where the side has a base. The base is the lowest-numbered base of the side in the hex. The receiving force keeps its number, orders, mission, speed, settings and fatigue, and takes the ships with everything aboard them: cargo, men, air groups, fuel and damage. A landing in preparation is joined by the rule of decision 12 of 0095 (chapter 7, Preparing a landing): the preparation of the receiving force keeps its hex and its order, the rosters are put together, the loading is the lower of the two, and the other force's preparation counts only if it is for the same hex; if the receiving force has none, the result is none. The force joined loses its route, its cargo order, a halt and a landing begun, and ceases to be. `TaskForceJoined` names the force joined, the receiving force, the base and the ships. [0111, 0095]

**Standing down.** A force order `stand_down` puts a force stopped at a base of the side (the same conditions as for a join) out of being. Its ships lie in port at the lowest-numbered base of the side in the hex, in no force, with what they carry: cargo, men, air groups, fuel and damage. Its orders end, as for a force joined, its preparation is dropped and it ceases to be. `TaskForceStoodDownInPort` names the force, the base and the ships. A ship in port in no force does not move, refuel, work cargo or take repair, as before (Bases on hexes). A force can be formed from it again by a forming order. This is not the stand-down of an alerted force or of an air group (chapter 3, Alert from belief; chapter 5, Stand-down and return). [0111]

A force cannot be joined or stood down at sea, part-way along a leg, or at a base of the other side, nor while it is under way, halted with a route to follow, out of fuel or away at the yard. Joining at sea and a ship changing force at sea are not built. [0111]

A side sees: its own `TaskForceFormed`, `TaskForceJoined` and `TaskForceStoodDownInPort`, whole, with the order as cause. The enemy sees nothing of them: a force it holds as a contact is a contact still, and it is not told what the contact was of. [0111]

## Movement

### Orders that move a force

A route order replaces the force's route with a list of waypoints. Chapter 1, Every reason an order is refused, gives when it is refused: a waypoint off the map, a waypoint that cannot be reached by sea from where the route starts (the turn-back rule below picks where that is), or a route that goes nowhere.

An accepted route order sets the route to the waypoints with the next index 0, sets the status to underway, sets the route order, and clears the arrival hour and the arrival event. It never clears the cargo order, the readiness or the rules of engagement. A route order given to an arrived force sets it underway. A route order to a force that is out of fuel is refused (differences.md row 34): it stays out of fuel where it is until it takes fuel in port. [0005, 0017, 0044]

A route order may also carry marks: a list of hexes to act at, each one of the route's waypoints, else the order is refused (chapter 1, Every reason an order is refused). A mark is by hex: every waypoint at a marked hex is marked. The last waypoint is never treated as marked, so a force arrives there as it always did. A new route order replaces the marks with its own. A route order may carry a second list of marks in the same way, `stand_at`, the hexes to stand at for the night: each is one of the route's waypoints, else the order is refused with the same words, a mark is by hex, the last waypoint is never treated as marked, and a new route order replaces them with its own. A route with none gives what a route gave before. What a marked waypoint does is in Halting at a marked waypoint and steaming on, below. [0096 items 1 and 2; 0124 item 1]

A speed order sets the speed setting to cruise, full or a number of knots, which must be above zero. It does not change the status. [0021 item 7]

A side sees: its own `OrderAccepted` or `OrderRejected`, with the reason. The enemy sees neither.

### Speed

A ship's speed is set by the force's speed setting:

| Setting | Ship's speed |
| --- | --- |
| Cruise | the class's `cruise_speed_kn`, times the ship's propulsion factor |
| Full | the class's `max_speed_kn`, times the ship's propulsion factor |
| Knots | the ordered knots, or the class's `max_speed_kn` times the propulsion factor, whichever is lower |

The propulsion factor is 1 for working propulsion, the `degraded_factor` of `surface_combat.toml` for degraded propulsion, and 0 for propulsion that is out. A game with no surface combat figures has no damaged ships and every factor is 1. A ship with a factor of 0 has a speed of 0. [0005, 0016 item 7, 0021 item 7]

The speed of the task force is the lowest speed of its ships. On a tie the lowest ship number is the slowest ship, which matters only for the breakdown. One damaged ship therefore slows the whole force. A force whose speed is 0 is dead in the water: it does not move, records no event, burns no fuel and stays underway. [0005, 0016 item 8]

A side sees: its own steaming events, which carry the speed.

### The budget of a pulse

A force moves in a pulse only if its status is underway, it has a ship afloat, and its speed is above 0. Its budget for the pulse, in miles, is the lower of two limits:

- time: speed times the hours of the pulse not yet used (12 for the night pulse, 6 for each daylight pulse);
- fuel: the distance the least-fuelled ship can steam at this speed (see Fuel limit), for the ships in the force.

If the fuel limit is the lower, the budget is "fuel-limited". The budget is spent as distance. Time is worked out afterwards as distance over speed. [0005, 0006 item 5, 0022]

A side sees: its own steaming events.

### The path

A force steams from one waypoint to the next by the shortest sea path. The path is not stored. For the waypoint being steamed to, the engine works out for every reachable hex the fewest steps to the waypoint, by a search from the waypoint that takes neighbours in the fixed order and never crosses a closed side. The next hex from a hex is then the first neighbour, in the fixed order, whose fewest steps is one less and whose side is open. A hex that cannot reach the waypoint is not in the result. If the waypoint is off the map or not navigable, nothing can reach it. The field is worked out again when the force turns to the next waypoint, and is the same every time because the map never changes. On open water this is the straight greedy path. Around land it is the shortest way round. What each step of the path costs in miles is the rule of the next section. [0005 item 3 as replaced, 0017 items 10 to 13]

A side sees: not an event.

### What a step is charged

Distance, time and fuel for a ship are worked in miles, and the miles of a step from one hex centre to the next are not always 40. A step from H to the next hex N on the path, for the waypoint W the force is steaming for, is charged as follows. [0058 items 1 to 4]

- **The way is clear from H** when the shortest sea path from H to W has as many steps as the hex distance from H to W (the number of hexes between them if there were no land). Land forces no detour from there.
- **Where the way is clear** the step is charged the miles it brings the force closer to W in a straight line: the air distance (chapter 5, Distance) from H to W less the air distance from N to W, and never less than 1. The charges of the steps from H to W then add up to the air distance from H to W, whatever path the steps take. An air distance is in whole miles, so a charge wobbles by a mile either way. A step is never charged more than 40.
- **Where the way is not clear** the step is charged 40 miles. Once the force is past the land the rest of the way is clear and is charged as above, so a path round an island is 40 miles a step until the way is clear, and the straight miles from there.
- Along any of the six directions of the grid the air distance falls by 40 a step, so a route along a hex axis is charged 40 a step, as it always was.

The charge of a step is worked out once, from the force's hex at the start of the step, with the waypoint the force is steaming for then, and is kept (see Legs and position). Everything that states a sea distance uses this one function: movement, the passage forecast, `measure`'s sea route and passage, the briefing's warning of a ship that cannot reach a base (the nearest base is the one with the fewest charged miles), and the viewer's measuring tool. Thresholds counted in hexes, such as how near a contact must be to alert a force, stay in hexes. [0058 item 8, 0049]

A side sees: the miles steamed in its own `TaskForceSteamed` events, and, in the breakdown, the charge of each step begun in the segment. The enemy sees neither.

A side sees: not an event.

### Legs and position

A leg is the step between the centres of two neighbouring hexes, charged by the rule above. A force on a leg has a `from_hex`, the `toward_hex` it steams to, the miles covered, `progress_nm`, and what the leg is charged, `leg_nm`, which is set when the leg begins. Progress runs from 0 up to but not including `leg_nm`. At a centre the force has no `toward_hex`, progress 0 and `leg_nm` 40. `toward_hex` is set exactly when progress is above 0. `leg_nm` is at least 1 and at most 40. It is state and is in the state hash. [0058 item 7]

A leg once started is always finished at the charge it began with. The next centre the force reaches is the hex ahead, whatever the route says: a new route given on the leg or a turn-back keeps the leg's charge. At a centre the next leg starts toward the next hex of the path, with the charge the map gives that step, and only if there is distance left to steam it.

The position of a force is the hex whose centre is nearest: the hex ahead once it is half the leg's charge or more along it (20 miles of a leg of 40), and the hex it left before that. A force is in the hex ahead at exactly half. Every other rule that asks where a force is reads its position. A force is always in a navigable hex. A force split off a leg copies the leg whole, charge included. [0005 item 1, 0017, 0058 items 6 and 7]

A side sees: its own position changes in `TaskForceSteamed` events (the `from` and `to` are positions, not centres reached, and `hexes_entered` lists the hexes entered).

### Order of movement within a pulse

Within one pulse forces move in order of clock time. The engine works out, for each force that can still steam, the distance to its next change of position: the rest of the way to half the charge of its leg or, if it is already at half or past it, the rest of the leg plus half the charge of the next, if the path has a next hex. It considers only a change that its remaining budget reaches. For each such force the time of the change is its hours used so far, plus the open segment plus that distance divided by its speed. The force with the earliest time is advanced to that point, and the lower task force number wins a tie, except that a force that is some force's charge comes after every force that is not (Keeping station). This repeats until no force has a change within its budget. Each force then steams out what is left of its budget. [0022 items 1 and 2]

A side sees: not an event.

### Meeting

A force that changes position meets the opposing forces in the hex it enters, and two opposing forces that begin a pulse in one hex and will not move meet at its start. A meeting is a fight, held inside this step. Chapter 4, Meeting, gives the rules, and what a side learns of a fight.

### After a fight

Before a fight both forces' open steaming is recorded, so the log shows the steaming before the fight. A fight takes no time. After it the stragglers of the two forces are dealt with by their instructions, then each force carries on:

- With no ship afloat, not underway, or a speed of 0, the force stops for the pulse.
- If it could steam before and can now at the same speed, its distance left is unchanged.
- If its speed changed, its distance left is the old distance left times the new speed over the old speed, cut, and then limited by how far its fuel now aboard goes at the new speed. The budget is fuel-limited if the fuel limit is the lower.
- If it could not steam before and can now, because the ship that held it sank or was left behind, or because it was sent home, it has a fresh budget from the hours of the pulse it has not used.
- A force made by the straggler pass during the fight moves in the same step, with the hours of the pulse left after its parent's hours.

[0022 items 5 and 6]

A side sees: its own steaming events, in log order before the fight's events.

### What a segment records

A force's steaming is recorded in segments: from the start of the pulse or the last fight to the next fight or the end of the pulse. A force that fights nobody has one segment. For a segment with distance above 0 the engine records:

1. `TaskForceSteamed`, with the force, where it was, where it is, the miles, the speed and the hexes entered. It cites the force's route order and speed order, and records a breakdown of the speed, the propulsion factor, the pulse hours, the time limit, the fuel limit and the charge of each step begun in the segment (a step begun in one segment and finished in the next is listed in the one that began it).
2. `FuelBurned` for each ship, citing the steamed event, and the fuel removed from the ship (see Fuel burn).

The hours of a segment are its distance divided by its speed, cut. They are added to the force's hours used in the pulse. [0005, 0006, 0022]

A side sees: both events, for its own forces and ships. The enemy sees neither.

### Arriving

A force arrives when it reaches the centre of its last waypoint. Reaching a waypoint's centre moves the route index past it, and waypoints the force is already on are passed at once. On arriving the engine sets the status to arrived, sets the arrival hour (the start of the pulse plus the hours used) and records `TaskForceArrived`, which cites the segment that brought it there, or the standing orders if that segment steamed no distance. If the force reaches its last waypoint in a segment that ends in a fight, it is marked arrived with that segment, before the fight. A force that arrives stays a task force. It does not dock or disband and it does no cargo work unless it holds a cargo order. [0005 item 6, 0022 item 6]

A side sees: its own `TaskForceArrived`.

### Halting at a marked waypoint and steaming on

A force with a route that has marks may halt at one. The engine looks for the halt when the force's mover is made, at the start of each pulse's movement, and again after each fight, for the pulse that movement is in. The halt is at the first marked waypoint, to act at or to stand at, from the force's next one on, where the force has work in that pulse. Work is one of three things, and a force may have more than one:

- **A bombardment.** The force's mission is bombardment, it is not away at the yard, it has a ship afloat, the hex holds a base of the other side that is on the force's side's map, the bombardment and air combat figures are loaded, and a ship of it would fire a shell by the rule of the shore bombardment phase (chapter 4, Shore bombardment).
- **Cargo.** The force has a cargo order that can work in that hex by the cargo rule below (a base of its side there, or men to land on a coast hex, or men of the order's unit ashore to load), and that is not complete by the cargo rule's own test of when nothing remains to move.
- **The night.** The waypoint is marked to stand at and the pulse is the night pulse. This is the **night stand**: it is the work of the mark, whatever the force's mission, and needs no base, no cargo and no target. A force that reaches a waypoint marked to stand at in a daylight pulse has no work there and passes it. A waypoint marked both to act at and to stand at has the work of the first, and the night as well in the night pulse. [0124 items 1 and 2]

With no work the force passes the waypoint as any other. A mark never stops a force that has nothing to do there. A route with no mark to stand at halts where it halted before, in every pulse. The route forecast (`measure`) does not foresee halts: it steams the force through every waypoint to the last, and the staff's forecast of a route through several waypoints, which it uses to measure a sortie out and home, marks none (`SideView::forecast_route`).

A force that reaches the centre of the waypoint it halts at stops there, as a force that arrives does. Its status is halted and it is not moving. It records `TaskForceHalted` (the hex, the work, and the hours since the scenario began; for the night, the words "to stand for the night"), citing its route order and the segment that brought it there, or its route and speed orders if it steamed no distance in the pulse. It is stopped as an arrived force is: it can be met and fought, it refuels and is repaired at a base of its side, it can be given a load or unload order, and it can be sent to the yard. It works cargo from the hour it halted, as an arrived force does from its arrival (Working cargo, below), and it bombards in the night pulse's shore bombardment phase (chapter 4, Shore bombardment). A force that halts by day to bombard waits for the night: the rule does not move the bombardment out of the night pulse. A force halted for the night is stopped: it meets what enters its hex and what it finds there, by chapter 4, as any force that is not moving does. [0096, 0124 item 4]

At the start of each pulse's movement, before the straggler pass, each halted force in force order steams on if its work is done:

- **For a bombardment,** it has bombarded since it halted, or it can no longer bombard there: no ship of it can fire, or its mission or the base changed.
- **For cargo,** it has none left to work there, by the same test as above.
- **For the night,** the pulse is not the night pulse. So a force that stood for the night steams on at the start of the morning pulse's movement.

A force with more than one kind of work waits for all. It records `TaskForceSteamedOn` (the hex, the work, and whether it fired; for the night, the words "the night is over"), citing its route order, the halt, the bombardment and the cargo order's completion event. It is then under way toward the waypoint after the one it halted at and moves in that pulse with the pulse's full hours, whatever the work it stood for. [0124 item 3]

A new route order ends a halt, and records nothing of its own: the order accepted is the record. So does a force retiring after a fight or a trip to the yard. A force that halts where its fuel also runs out is halted, and when it steams on it has no fuel and is out of fuel by the rule above. A halted force with no ship left stays as it is. [0096]

A side can ask this choice for its own forces before it is made (`measure`'s `choices`, `halt`): the answer is the marked waypoint on its route where the rule would halt the force now, or none, from what the side holds; and, for a halted force, whether it would steam on now (`steam_on`). Each question is asked for a pulse, since the answer for a mark to stand at depends on it: the side's list of questions asks both for the night, the first movement of the day it gives orders for. [0100, 0124 item 5]

A side sees: its own `TaskForceHalted` and `TaskForceSteamedOn`, whole. The enemy sees neither.

### Turning back and re-routing

A route order given to a force between two centres starts the new route from the centre the force reaches first. The force goes on to the hex ahead or turns back to the hex it left, whichever is the shorter trip to the first waypoint of the new route:

- Going on costs the rest of the leg (the leg's charge less the progress) plus the charged miles of the sea path from the hex ahead to the waypoint, as What a step is charged gives them.
- Turning back costs the progress plus the charged miles of the sea path from the hex it left to the waypoint.
- A tie goes ahead. If only one side can reach the waypoint, that side is taken. If neither can, the order is rejected. A route that ends at the hex ahead or the hex left is a real route.

Turning back swaps the two hexes and sets progress to the leg's charge less its old value; the charge is kept. It happens when the order is taken in. The leg is then steamed as usual, and a re-route never gains distance. A force turned back at exactly 20 miles changes position, from the hex ahead to the hex it left, without steaming. A turn-back records `TaskForceTurnedBack` (the hex it now leaves from, the hex it now heads for, the progress and the position), citing the order and recorded just after the order's own event. A route order at a centre records none. [0005 item 5, 0017 item 14, 0044 item 7, 0058 item 7]

A side can ask this choice for its own forces before it is made (`measure`'s `choices`, `reroute`): the side states the first waypoint of the route it is considering, and the answer is whether the force would turn back or go on, and the hex the new route steers from, from the force's own position on its leg and the map. [0100]

A side sees: its own `TaskForceTurnedBack`.

### Keeping station

A task force on the escort mission may be given a charge, another force of its side, by the escort order (chapter 1, Task force orders). Such a force is an escort. A force has at most one charge; a charge may have several escorts; an escort cannot be a charge. [0121]

1. **In station.** An escort is in station when it is in its charge's hex (the nearest hex centre, as chapter 4 reads a position) and both it and its charge have a ship afloat.
2. **When.** Keeping station runs at the start of each pulse's movement, after the halts steamed on and the straggler pass, and before the budgets are made, so a force the pass made moves with its station kept. It runs for every escort with a ship afloat, in task force number order. A force that is out of fuel takes no route and keeps none it has.
3. **In station.** The escort takes its charge's route and speed for the pulse. Its route is the charge's waypoints still to reach, cut at the waypoint where the charge will halt in this pulse, if it will halt at one (Halting at a marked waypoint and steaming on); its speed is the charge's speed in knots, which is its slowest ship's at the charge's speed setting. The escort then moves as a force ordered so would (chapter 1, Task force orders): its own ships may make less than the charge's speed, and the escort then falls behind. Its route has no marks, so it never halts of itself. If its charge will not steam in the pulse, because it is stopped, halted, out of fuel, has no route left or has a speed of 0, the escort stops where it is: at a hex centre it keeps no route and is idle; between two centres it is routed to the hex it is in, and turns back where that is the shorter way, as a route order would (Turning back and re-routing). So an escort stops with its charge where the charge halts to work cargo.
4. **Out of station.** The escort steams for its charge's hex by the shortest sea route, at its cruising speed, until it is in that hex, and is in station again from the next pulse's start. An escort that fell behind is out of station in this way.
5. **Its own fuel.** The escort burns its own fuel by the rules of Fuel, and the fuel limit may cut its budget below its charge's.
6. **A route order is refused** to an escort while it has a charge: "keeps station" (chapter 1, Every reason an order is refused). A speed order is accepted, but the speed keeping station gives replaces it at the start of the next pulse. The escort order that clears the charge frees it.
7. **The charge lost or cleared.** An escort has no charge from the moment its charge has no ship afloat, is joined into another force or is stood down in port, or its escort order is one that clears the charge, and it stops where it is as in 3. A join or a stand-down order of the charge records `ChargeLost` for each of its escorts when it is taken in, as one of the events the order makes, citing that order. A join or a stand-down order to the escort itself ends its own charge, with no `ChargeLost`. A charge sunk in a fight is dealt with in the fight (chapter 4, Meeting). A charge sunk by aircraft is noticed at the start of the next movement step, when keeping station runs, and `ChargeLost` then cites the escort order only. An escort order that clears the charge by order records no `ChargeLost`.
8. **Order of movement.** On a tie in time (Order of movement within a pulse), a force that is some force's charge at the start of the step comes after every force that is not, so an escort that moves with its charge enters a hex first and its meeting there comes first (chapter 4, Meeting).

What is written is only what changes: keeping station writes the escort's route, speed and status when they are not already what the rule gives, so an escort that has kept station for a pulse and has nothing new to do records nothing. Where it writes, the route order and the speed order of the escort are the escort order that gave the charge. [0121]

A side sees: its own `StationKept`, with the escort, its charge, whether it is in station, the waypoints it was given and the knots; it cites the escort order and, for an escort in station, the charge's route order. Its own `ChargeLost`, with the escort, the charge and the hex, citing the escort order where the charge is lost in keeping station or in a fight, and the order or the end of the fight that took it. A turning back follows, citing the one that caused it. The enemy sees none of them. [0121]

Known limits: the number of escorts a force is given is the staff's judgement, and no figure is set for it. Stations inside a hex (pickets, day and night stations, the distance of a screen) are below the grain of the map. An escort does not go out to an enemy in a neighbouring hex, because forces in neighbouring hexes do not fight. What the charge does when its escort fights is its route and its work, unchanged. [0121]

### Running out of fuel

When the budget is fuel-limited and the force has not reached its last waypoint at the end of its steaming, the force stops where its fuel ends, wherever that is on a leg. Its status is out of fuel and the engine records `TaskForceOutOfFuel` with its position, citing its last steaming of the pulse or its orders. The force keeps its leg and its route. A force whose ships hold no fuel has a budget of 0, and is marked out of fuel at the end of the next movement step in which it is underway. A route order sets an out-of-fuel force underway again. It will only move if a ship has fuel. A fuel limit that is exactly enough for the route is not out of fuel. [0006 item 5]

A side sees: its own `TaskForceOutOfFuel`.

### A force that breaks off

A force that breaks off a fight or slips away avoids action for the rest of the pulse, and keeps to its route: a force with a route out of the hex leaves, a force with no route stays. Chapter 4, Avoiding for the pulse, gives the rule. [0022 items 9 and 10]

A side sees: its own force's `ForceBrokeOff` or `ForceSlippedAway` (chapter 4, Breaking off).

### Limits

Weather, formation speed loss, zigzagging, and any difference between day and night steaming are not modelled. A fight takes no time and a force does not pursue one that breaks off. [0005, 0022]

## Fuel

### Fuel burn

A ship's fuel is in tons, given at the start by its entry in the scenario file (`fuel_t`). A class has `fuel_capacity_t`, `endurance_nm` (the distance a full load lasts at cruise speed) and `cruise_speed_kn`. For a distance steamed at a speed:

- the burn per mile at cruise is `fuel_capacity_t` over `endurance_nm`;
- the multiplier is 1 if the speed is at or below the ship's cruise speed, and otherwise (speed over cruise speed) squared, the ratio cut to six decimals before it is squared;
- the fuel burned is the distance times `fuel_capacity_t` over `endurance_nm`, worked as one multiply-then-divide, times the multiplier, and then no more than the fuel aboard.

Each ship uses its own class figures at the speed of the force. A ship in a slow force burns at its own cruise rate per mile, never less. A ship that does not move burns nothing. A ship at half speed from damage is below cruise and burns the cruise rate. The fuel taken off a ship is written at the end of each segment, and each ship has a `FuelBurned` event with the tons burned and the tons left. [0006, 0016 item 9]

The staff reckons the fuel of a run by night at the speed it tries, by this rule and the engine's forecast (chapter 8, A run by night). [0145]

A side sees: `FuelBurned` for its own ships.

### The fuel limit on a pulse's budget

The distance a ship's fuel lasts at a speed is its fuel divided by the multiplier, then times `endurance_nm` over `fuel_capacity_t`. The force's fuel limit is the lowest of these over its ships. [0006 item 5]

A side sees: not an event. The breakdown of the steaming event holds the limit.

### Refuelling in port

Refuelling takes time. It is done in the loading phase of every pulse, after movement, in the cargo rule's queue (Berths and lighters, Working cargo), and not at the start of the day: nothing is refuelled at the start of a day, whatever the force's state then. [0112]

**Who takes part.** A task force takes part in a pulse if it is not under way, has no hex ahead (it is exactly at a hex centre), has a ship afloat, and either has a cargo order that can work there (Cargo orders) or lies at the centre of a hex where a base of its own side is. So a force that is idle, has arrived, is halted at a marked waypoint or is out of fuel at a base of its side takes fuel without any order. A force out of fuel between two hexes takes nothing. A force that ran out of fuel in this pulse takes no fuel until the next pulse: the hour it ran dry is not known. A force away at the yard takes none (chapter 4, Repair).

**The hours.** A force works from the later of the pulse's start and the hour it stopped (its arrival, or its halt) to the pulse's end, as cargo does (Working cargo). A force with no hours takes none.

**The berth.** Berths are given out at the start of the work of each pulse, to forces in order of number and ships within a force in order of theirs, to a ship that has work: cargo to move (Berths and lighters) or fuel to take. A ship has fuel to take if it lacks fuel, a base of its side in the hex holds fuel above its fuel reserve, and the port's fuel by lighter rate is given. Where that rate is not given, fuelling has no limit and a berth cannot speed it, so fuel alone claims no berth. A berth given in a pulse is that ship's for the pulse: a ship that finishes early does not hand it on until the next pulse. A ship with no berth works by lighter. So a force that fuels can keep another force's ship from a berth, and a small port fuels a large force slowly.

**The work.** Each force in order of number works its cargo first, as before. Then each of its ships in order of number takes fuel in the hours of the pulse its cargo left: the hours of the force less the hours that ship spent on cargo. The ship takes the least of: what it lacks up to its class's `fuel_capacity_t`; what the serving base holds above its fuel reserve; and its fuelling rate times the hours left, which is the port's fuel berth rate if the ship has a berth and its fuel lighter rate if not. A ship does one thing at a time, so a ship with a long cargo job fuels in what is left of the pulse. A rate that is not given is no limit: the ship takes all it lacks, up to what the base holds above its reserve, in that pulse, whatever hours its cargo used. Each change to a stock applies at once, so a force served first leaves the base the less.

**The serving base.** The serving base is the lowest-numbered base of the force's side in the force's hex that has fuel above its fuel reserve (`fuel_reserve_t`, 0 if the base has none). One base serves a ship in a pulse. A ship the base cannot fill is left short, and the next ship is served by the lowest-numbered base that then has fuel above its reserve, so a force draws on the bases of its side in the hex in base order, each down to its reserve. The reserve holds for every force, the commander's own included: refuelling is a rule and not an order. A reserve the commander set by command binds it from that morning (chapter 8, The six limits). A ship that lacks nothing takes nothing and records nothing. A scenario with no reserve refuels without one. A scenario whose bases have no fuel has no refuelling.

**Out of fuel.** A force that was out of fuel is restored by the first of its ships to take fuel: its status goes to idle and its route and route order are dropped, so it waits for a new order. An arrived force stays arrived. The fuel a base holds as stock is not the fuel for aircraft: air sorties use supply (chapter 5, Supply for sorties). [0045]

**Sailing short.** A force ordered to sail leaves with the fuel it has: an order to sail is not held back for fuel, and there is no order to sail when fuelled. A force under way takes part in no loading phase, so once it has left it takes no more fuel. The tools say what a force lacks and how long it would take (below), so that a player or a staff can time the order. [0112]

**The events.** Each ship that takes fuel records `ShipRefuelled` in the pulse it is taken, with the tons taken, the ship's fuel, the base's fuel left and whether the force was restored. It cites the force's arrival event (or its halt) and has a breakdown of its factors: the hours the force worked, the hours the ship's cargo used, the hours left, whether it was on a berth, the rate where one is given, the tons it lacked, and what the base held above its reserve; the result is the tons taken. A ship left short while a base of its side in the hex still holds fuel, at its reserve (the base that served it, or, if none did, any base of the side there), records `RefuellingHeldAtReserve`, with the tons it still lacks. It cites the ship's `ShipRefuelled` if it took any, else the force's arrival event if it has one. It is recorded only for a ship that is short because of the reserve, so a ship filled records none, a ship short because its rate or the hours ran out with fuel above the reserve left records none, and a base emptied with no reserve records none. It is recorded once, in the pulse the ship is first held: it is not recorded again while the latest fuel event of the ship is a hold at the same hex with the same shortfall. [0111, 0112]

**What the tools say.** A force line (the list of forces and the briefing) of a force stopped at a base of its own side whose ships lack fuel gives `fuel_lacked_t` (the tons its ships lack together), `fuelling_hours` (the longest ship's lack over the fuelling rate of the port: the berth rate where the port has a berth, else the lighter rate; left out where that rate is not given) and `fuelling_at` (the base). The briefing says, for example, "lacks 540 t of fuel: 13.5 hours at base:2", or, where no rate is given, "lacks 540 t of fuel, taken in its first pulse in port". [0112]

A side sees: `ShipRefuelled` and `RefuellingHeldAtReserve` for its own ships. [0111, 0112]

Limits: there is no refuelling at sea (no oiler, no transfer between ships), no port damage and no daily ceiling on a port beyond its rates. A ship in port in no force takes no fuel and no ammunition. No reserve is heeded in any rule but those of refuelling and of a load the staff orders (Cargo orders). [0045 deferred, 0046, 0111, 0112]

### Rearming in port

A ship in port takes ammunition aboard as it takes fuel: in the loading phase of every pulse, in the cargo rule's queue, after its cargo and its fuel, in the hours they leave. A game with no ammunition at any base, no rate or no weights is the game it was. [0122]

**The stock.** A base holds ammunition in tons, `ammunition_t`, given by the scenario and 0 where it gives none. It is a stock as supply and fuel are, for ships' main batteries and torpedo tubes alone. Nothing carries it or adds to it: no cargo order moves it, and the only rule that takes it away is rearming. A base with none rearms nobody. Unlike fuel, it has no reserve: the staff's reserve limits cover supply and fuel. [0122]

**What a ship lacks.** Rounds: its outfit, which is its class's main guns times `gun_rounds_per_gun`, less the rounds aboard, cut to a whole number and never below 0. A class whose `gun_rounds_per_gun` is 9,999 carries the marker for ammunition that is not limiting, and a ship of it lacks no rounds. Torpedoes: its tubes that have fired and not been loaded again (chapter 4, Torpedo shots): all its class's tubes if it has no torpedo ready, else the count the ship keeps (`torpedoes_fired`, or the first side's tubes if that is more). A scenario ship may start with some tubes fired (`tubes_fired` on its `[[ship]]` entry, 0 if not given, at most its class's tubes; the loader refuses more). A ship with every tube fired has no torpedo ready. [0122]

**The weights.** A round weighs `surface_combat.toml: [rearming] tons_per_round` for the ship's kind (battleship, cruiser, destroyer, and `others` for every other kind) and a torpedo weighs `tons_per_torpedo`, each above 0 (the loader refuses otherwise). A file with no `[rearming]` table gives no weights, and no ship rearms. The live figures are 1, 0.15, 0.04 and 0.04 t a round and 1.5 t a torpedo, placeholders sized to the shell and the torpedo. [0122]

**The rates.** A ship takes ammunition at the port's ammunition berth rate if it has a berth, and its ammunition lighter rate if not, in tons an hour (Port size). A rate not given means no rearming by that means: a ship that has no berth, at a port with no lighter rate, takes none, and a ship that has a berth, at a port with no berth rate for ammunition, takes none. The live figures are 20 t an hour at a berth and 10 by lighter. [0122]

**The queue.** A ship has ammunition to take, and claims a berth for it, if it lacks some (rounds or torpedoes, of tons above 0), the weights are given, a base of its side in the hex holds ammunition above 0, and the port gives an ammunition berth rate. A ship whose port gives no berth rate for ammunition claims no berth for it, and rearms by lighter if the port gives that rate. It claims in the order of Berths and lighters, in the one scan, beside cargo and fuel. In a pulse each force in order of number works its cargo, then each ship in order of number takes its fuel (Refuelling in port), and then each ship in order of number takes ammunition, in the hours that are left it: the hours of the force less the hours that ship used for cargo and for fuel (a fuelling with no rate given uses none). A ship with no hours left takes none. [0122]

**The serving base.** The serving base is the lowest-numbered base of the force's side in the force's hex that holds ammunition above 0. One base serves a ship in a pulse. [0122]

**Whole rounds, then whole torpedoes.** The room a ship has in the pulse is the least of its rate times its hours and the serving base's stock. It takes as many whole rounds as the room allows, up to what it lacks, and from what room is left as many whole torpedoes, up to what it lacks. A part of a round or a torpedo is not taken, and the room not used is not carried to the next pulse. The tons taken are the rounds times the round's weight plus the torpedoes times the torpedo's. The ship's rounds aboard rise by the rounds taken, and the base's stock falls by the tons. Rounds are taken first, so a ship short of both takes no torpedo until its rounds are full or the pulse's room runs out. [0122]

**Short stock.** Each change to a stock applies at once, so ships take from a base in the order the queue serves them: a force with a lower number, and a ship with a lower number within a force, before the next. A ship served when the stock is out takes nothing and records nothing. [0122]

**A tube loaded again.** A torpedo taken loads a tube that has fired, the lowest numbered fired tube first. In terms of the state: the count of fired tubes falls by the torpedoes taken; the second-side mark is cleared (`torpedoes_one_side_fired`); and the ship has a torpedo ready if any tube is loaded. That tube may fire again (chapter 4, Torpedo shots). [0122]

**The event.** Each ship that takes any rounds or torpedoes records `ShipRearmed` in the pulse it takes them: the ship, its force, the base, the rounds, the torpedoes and the tons taken, the ship's rounds aboard and tubes fired after it, and the base's ammunition left. It cites the force's arrival event (or its halt), as `ShipRefuelled` does, and has a breakdown of its factors: the hours the force worked, the hours the ship had used, whether it was on a berth, the rate, the tons a round and a torpedo weigh, the rounds and the torpedoes it lacked, and the base's ammunition before; the result is the tons taken. A ship that takes nothing records nothing. [0122]

**What the tools say.** A force line (the list of forces and the briefing) of a force stopped at a base of its own side (idle, arrived, halted or out of fuel, with no hex ahead) whose ships lack rounds or torpedoes gives `ammunition_lacked`: `rounds`, `torpedoes` and `tons` (what its ships lack together, by the weights; 0 tons where the scenario gives no weights) and `hours`. It is left out when the force lacks nothing. The `hours` are worked as the queue would work them where the force lies, cargo aside: the berths are given out, in the order of Berths and lighters, to the ships of the side's stopped forces in the hex that lack fuel or ammunition, as the queue would give them (a ship lacking fuel claims where the port gives a fuel lighter rate, a ship lacking ammunition where it gives an ammunition berth rate, and the serving base must hold fuel above its reserve or ammunition above 0); each ship's hours are its fuel first, its lack of fuel to capacity over its fuel rate, then its ammunition, the tons it lacks over its ammunition rate; and the hours are the longest ship's. Hours are worked through and not counted in pulses. They are `null` where the base holds less ammunition than the force lacks (or none), or where a ship that lacks ammunition has no rate where it lies (so no weights, no rate or no base means no hours). The briefing says, for example, ", lacks 120 rounds and 4 torpedoes, 24 t of ammunition: 1.2 hours to rearm here", or ": it cannot rearm here" where the hours are `null`. [0122]

**What the frozen figures give.** None: the frozen step 1 figures have no `[rearming]` table, no ammunition rate and no base with ammunition, so nothing is rearmed and the hand-calculation tests built before the rule are as they were. [0122]

Limits: no anti-aircraft ammunition (it is not counted); no bombs or torpedoes for aircraft (a base's air supply covers them, chapter 5); torpedoes come from the same stock as rounds, wherever a base has it (the record has them from tenders at particular places); no ammunition carried as cargo; no rearming at sea or from another ship; no reserve of ammunition at a base. [0122]

A side sees: `ShipRearmed` for its own ships. The enemy sees none. [0122]

## Cargo

### Cargo and capacity

A ship carries supply and fuel as cargo, in tons. The two together never exceed the class's `cargo_capacity_t`. A ship also carries men up to the class's `troop_capacity_men`, a limit of its own: men take no cargo room and cargo takes no berths (chapter 7). A ship's cargo fuel is not the fuel it steams on: it cannot burn it. A base holds a supply stock and a fuel stock. A scenario may start a ship with either. [0011 item 1, 0046]

A side sees: its own cargo events (below).

### Cargo orders

There are five standing cargo orders. They are one kind of order, so a new one replaces the old. A completed order stays standing until it is replaced, and a route order does not clear it. The order to do no cargo work (`no_cargo_work`) clears it: the force then holds no cargo order and works no cargo, as a force made by a split or by forming does, until it is given another. It moves nothing, is accepted whatever the force's status, so it is the one way to take a cargo order away from a force under way, and a force halted at a marked waypoint for its cargo alone steams on at the next pulse's movement, its cargo work being done. The staff gives it to a lift it turns back (chapter 8, Taking a place, A lift turned back). [0191, amended in the build]

| Order | Takes effect |
| --- | --- |
| Load (supply) | Fills each ship's room, which is capacity less supply and fuel cargo aboard, from the base's supply stock |
| Load fuel | The same, from the base's fuel stock |
| Load to a weight | The same, from the base's supply stock, or from its fuel stock if the order names fuel, until the force has the weight aboard (below) |
| Load with a land unit | Takes the unit's men ashore in the force's hex aboard first, then the named cargo (supply, fuel or none) as the rows above do. It needs no base: off a shore where the side has none it loads men only (chapter 7) |
| Unload | Lands the men aboard first, on a coast hex only, then moves everything aboard into the base's two stocks, supply first and then fuel. The first man landed where the side has no base forms one (chapter 7) |

A load order is rejected at intake if the force is underway, or if no base of the side is at the force's position, and a load to a weight also if its weight, or its level, is not above 0. A load order with a unit has its own reasons and needs no base (chapter 1, Every reason an order is refused). An unload order is always accepted. A load order to a force that is out of fuel is refused (differences.md row 17). Both take effect in a loading phase in which the force is idle, arrived or halted (above, Halting at a marked waypoint) and is exactly at the centre of a hex that holds a base of its own side, as refuelling asks (differences.md row 32). Two cases need no base there, if the ground figures are loaded: a load order with a unit, which takes men off the shore, and an unload order with men aboard in a coast hex, which lands them (chapter 7). A force that is underway, or that is out of fuel, works no cargo, and neither does a force idle part-way along a leg, whatever hex it is nearest to. A load order is still accepted at intake for a force idle part-way along a leg whose position, the nearest centre, holds a base of its side; it then works no cargo until the force is at that centre (differences.md row 41). Cargo moves automatically only through an order: an arrival with no unload order does nothing. A force made by a split or by forming has no cargo order. [0011 items 6, 7, 10 and 13, 0046 items 2 and 3, 0066, 0111, 0162]

**A load to a weight.** A load of supply or of fuel (`load`, with `weight_t`, tons, and for fuel `cargo` fuel; a load of supply is the default) may name the tons the force is to have aboard of that cargo. The weight is the force's, all its ships together, and it is a level and not an addition: the force loads until the cargo it has aboard in all its ships is the weight, and a force that has that much aboard loads nothing. No state is kept of it but the order's weight: the order and what is aboard say what is left. The ships are filled in ship order: each takes what its room allows, as a load with no weight does, until the weight is aboard. The berths, rates and hours are those of the sections below. A load with no weight fills the room, as before. A weight does not apply to men: a load with a unit takes the unit, and a weight with a unit is refused when the scenario is read and by the agent's decoder. A weight must be above 0 (chapter 1, Every reason an order is refused). [0111]

**A level for each ship.** A load to a weight may also name `each_t`, tons: the most any one ship is to hold of that cargo. The ships are filled in ship order as before, and each takes the least of three figures: its room, what is left of the weight, and `each_t` less what it already holds of that cargo, not below 0. Like the weight it is a level and not an addition: a ship that holds `each_t` or more takes none and gives none up. Where the levels together are below the weight, the force loads what they allow and the order is complete. It is complete when the stock it may draw on is gone, when the weight is aboard, or when no ship can take more by its room and the level. A ship at its level has no cargo to move and takes no berth for it. The warning below the reserve (below) counts a ship's room only up to the level. With no `each_t` a load to a weight plays as before. A level is refused with no weight, with a unit (when the scenario is read and by the agent's decoder) and at 0 or below (chapter 1, Every reason an order is refused). No event is added; the loaded event's breakdown gains a factor `each_t` where a level is named. [0162]

**The reserve and the two issuers.** A base keeps a supply reserve and a fuel reserve (Bases on hexes), each a number of tons. A cargo order records who issued it. A load the staff issues (the order's origin is the staff) never takes the base's stock below its reserve: the load is capped as it is worked, at each moment, to what the base holds above the reserve, so whatever the night took from the stock, the staff's own load does not go below it. A load the commander issues (the order's origin is the player) is not capped and may take a stock to nothing. When such an order is taken in and the base holds less above its reserve than the order asks, the order is accepted and `LoadBelowReserve` follows the acceptance, citing the order, with the tons it may take below the reserve and the reserve. What the order asks is the weight less what is aboard (or, with no weight, the force's room), and no more than the stock, all as they stand at intake. Nothing else heeds a reserve: the men's supply, aircraft upkeep, repairs, sorties, bombs and shells take a stock below it. A reserve is a line the planner keeps, not a second store. [0111]

A side sees: its own `OrderAccepted` or `OrderRejected`, and its own `LoadBelowReserve`, whole, citing the order. The enemy sees none. [0111]

### Berths and lighters

Berths belong to a base, or, for a force that lands men where its side has no base, to the base that landing will form. They are counted for each side's landing place in a hex, which is its lowest-numbered base there. At the start of the work of each loading phase, before any cargo moves or fuel is taken, berths are given out. Forces that take part in the phase (Refuelling in port: a force with a cargo order that can work there, or at a base of its side) are taken in order of number, and the ships of each in order of number. A ship is given a berth if the base has one free and the ship has work: cargo to move, fuel to take (Refuelling in port) or ammunition to take (Rearming in port). Cargo, fuel and ammunition share the one queue and the one count of berths. [0112, 0122] A ship has cargo to move if, for a load, it has room and the stock it may draw on is above 0 at this moment (the base's stock of that cargo, or for a load the staff ordered the stock above the base's reserve, and in either case no more than the weight of a load to a weight still allows, and no more than its level allows, so a ship of a force whose weight is aboard, or a ship at its level, has none), or, for a load with a unit, men of the unit are ashore and it has room for them; and if, for an unload, it has supply or fuel cargo aboard, or men aboard in a coast hex. A force's scan stops at the first ship that finds no berth free. The stock read is the stock at the start of the phase, so a ship that loads from stock which another force unloads in the same phase has no berth, and works by lighter. A ship of a force whose staff-ordered load has nothing above the reserve to draw on has no berth either. [0111]

A ship with a berth moves cargo at the berth rate of the port. A ship with no berth works by lighter at the lighter rate, at the same time. A size 0 port has no berths, so every ship there works by lighter. A ship fuels at the fuel berth rate if it has a berth and the fuel lighter rate if not (Refuelling in port), and rearms at the ammunition berth rate or the ammunition lighter rate in the same way (Rearming in port). There is no other limit on a port's daily traffic, and no limit on the size of ship a port takes. The cargo figures are the placeholders of `port_sizes.csv`; the fuelling figures are sized from rows (Port size). [0011 items 2 to 4, 0066, 0112]

A side sees: not an event of its own. The effect is in the cargo events.

### Working cargo

Cargo moves in the loading phase, the last phase of every pulse, for the hours the force was stopped in the pulse. The force works from the later of the pulse's start and its arrival hour (the start of the pulse if it has no arrival hour) to the end of the pulse. A force halted at a marked waypoint works from the hour it halted in the same way, and its cargo events cite the halt in place of an arrival. A force that arrived eight hours into the night pulse works four hours of it. A force with no hours works nothing. The base used is the lowest-numbered base of the side at the force's position.

Forces are worked in order of number, ships in order of number, and each change to a stock applies at once, so a force that unloads can supply a force with a higher number that loads at the same base in the same pulse. A force's cargo is worked before its ships take fuel (Refuelling in port), and the hours of a ship's cargo work are the hours it does not fuel in. For each ship:

- The most it can move in the pulse is its rate (berth or lighter) times the hours worked. Each man moved takes `ground.toml: handling_t_per_man` of it, and men move first, in whole men (chapter 7, Men worked as cargo). The rest of the bullets count the tons left.
- A load moves the least of that, the ship's room, and the stock it may draw on: the base's stock, or for a load the staff ordered the stock above the base's reserve, read at this moment; and for a load to a weight no more than the weight less what is aboard the force's ships. [0111]
- An unload moves supply first, the least of the most it can move and the supply aboard, and then fuel, the least of what the pulse allows after the supply and the fuel cargo aboard. A ship works one cargo at a time, in that order.

Before the cargo, a landing divides units and forms a base where chapter 7 says, and each ship moves its men. Each move above 0 records `CargoLoaded` or `CargoUnloaded`, naming the cargo, the tons moved, the tons now aboard and the base's stock, citing the cargo order and the force's arrival event (or, for a halted force, its halt). When a force has moved something this pulse and nothing remains to move, one more event is recorded: `LoadingComplete` or `UnloadingComplete`, with the hour the slowest ship finished (the start of work plus the longest of each ship's tons over its rate). It cites the cargo events. Nothing remains for a load when the stock it may draw on is 0 or every ship is full, or, for a load to a weight, the weight is aboard. The stock a load may draw on is gone, for a load the staff ordered, when the base is at its reserve. A force that acts and steams on (Halting at a marked waypoint and steaming on) steams on then. [0111]

A load the staff ordered that is complete because the base holds nothing above its reserve, while the base still holds some, the force is short of its weight and a ship has room, records one more event, `LoadHeldAtReserve`, citing the `LoadingComplete`: the base, the cargo, the tons aboard and the reserve. [0111] For an unload nothing remains when no ship has supply or fuel cargo aboard and, in a coast hex, no ship has men aboard. A load order with a unit records `UnitLoadingComplete` in place of `LoadingComplete` (chapter 7). [0011 items 5 and 12, 0046 items 4 and 5, 0066]

**The loading factor.** While a force lands men on a coast hex, each ship's rate, berth or lighter, is multiplied by a factor that follows the force's loading: `ground.toml: unprepared_landing_share` plus the rest of 1 times the loading over 100 (chapter 7, Landing men and Preparing a landing). It scales the tons of men, supply and fuel alike, from the first man landed until the unloading there is complete, and so the hour the unloading completes. A force that carries no men and has begun no landing, and every load order, work at the rate of the port as before. With the frozen share of 1 the factor is 1. The breakdowns of `CargoUnloaded` and `MenLanded` then name `port_rate_t_per_h`, `unprepared_landing_share`, `loading` and `loading_factor`, only where the share is below 1. [0095 item 5]

**The port factor.** Work done the day before on a base's port by land units (chapter 7, Units that work and what they build) multiplies every ship's rate at the base, berth or lighter, for men and for cargo alike, load and unload alike: by `1 + work / work_to_double_unloading` and by no more than 2. It is 1 where no work was done on the port the day before, and on the first day. It multiplies the port's rate before the loading factor above, so the two multiply together. The breakdowns of `CargoLoaded`, `CargoUnloaded` and `MenLanded` then name `port_work` (the work) and `port_work_factor`, only where there was work, and cite the day's `UnitWorked` events at the base. The base is the one the force works at; a base formed by the landing in this pulse has no work. [0099 item 4]

A side sees: `CargoLoaded`, `CargoUnloaded`, `LoadingComplete`, `UnitLoadingComplete`, `UnloadingComplete`, `LoadHeldAtReserve`, `MenEmbarked` and `MenLanded` for its own ships and forces. The enemy sees none. [0111]

### At sea

A ship at sea does not load or unload, and there is no refuelling at sea, no tanker and no transfer between ships. A ship that is low on fuel with a hold full of fuel cargo must reach a base and land the cargo before it can use it. [0046]

## Stragglers

A straggler is a ship whose propulsion is degraded or out. A ship with flotation damage of 50 or more is heavily damaged, which is a separate matter. A task force has a standing instruction for stragglers and a share to keep, set with its rules of engagement: keep (the default), detach, leave or retire. The instruction is applied to every force that has a straggler and an instruction other than keep, at the start of every movement step, in order of force number, and straight after each fight, to the two forces that fought, lower number first. A force made by the pass is not processed in the same pass. [0016 items 10 to 12, 0022 item 5]

The ships of the force are sorted by propulsion into sound (working), able to steam (degraded) and dead in the water (out).

- **Keep:** nothing happens.
- **Leave:** every straggler, degraded and out, goes into one new force that stays where it is. The parent carries on.
- **Detach:** the ships dead in the water are left behind as above. The degraded ones go into a new force with escorts and head for the home port. The number of escorts is the sum over the degraded ships of the `straggler_escorts` figure of `surface_combat.toml` for each one's kind. Escorts are taken from sound ships that have main guns, the least important kind first and the lowest number on a tie. The importance of a kind is in chapter 3, What a report holds. If there are not enough, all of those are taken. If the parent would keep fewer than its share to keep of the sound ships it had, or none, the force retires instead. A ship with degraded propulsion is not an escort and does not count toward the share to keep.
- **Retire:** the ships dead in the water are left, and the rest of the force, degraded ships included, heads for the home port together. The parent records `ForceRetired`, gets no route and idle status, and then takes the home-port order below. A force with no ship left to send home stays as it is.

A side can ask this choice for its own forces before it is made (`measure`'s `choices`, `stragglers`): the answer is the step of this rule the force would take now with its stragglers, from what the side holds. [0100]

A force made by the pass takes the side, mission, speed setting, home port and the rest from its parent as a split does, and is then set to the avoid stance with the straggler instruction keep and no share to keep, keeping the parent's break-off level. A force that retires is set the same way. Its name says whether it was sent home, left behind or detached.

Heading for the home port is a set-route order to the home port's hex, issued by the local commander (chapter 1, Who can issue) and taken in at once, so it is checked and applied as any route order. Its acceptance cites the event that detached or retired the force, and movement then cites the order. A force with no home port, or one already at the centre of its home port's hex, gets no order and stays where it is. A force that is left behind has no route and stays where it is, idle, even part-way along a leg. A force with no ship afloat is never sent anywhere.

A side can ask this choice for its own forces before it is made (`measure`'s `choices`, `head_home`): the answer is the hex a straggler force or a retiring force would head for, its home port, from what the side holds. [0100]

Each decision records `ShipsDetached` (the parent, the new force, the ships, the escorts and the reason: ordered, sent home or left behind), with a breakdown of the sound ships, the escorts needed, the escorts taken and the share to keep.

A side sees: its own `ShipsDetached`, `ForceRetired`, and the order events. The enemy sees none.

Limits: towing and scuttling, damage control, and forming and merging forces by order are not built. Ship repair is in chapter 4, Repair. [0016 deferred, 0052 items 7 and 11, 0069]

## Readiness and fatigue

A task force's readiness, whether it is alert, the alarm, and the fatigue that alertness causes are in chapter 3, Readiness, alertness and fatigue of a force. A readiness order is in chapter 1, Task force orders. Nothing in this chapter reads them.

## Not in this chapter

- The day, the pulse, the clock and the order of phases: chapter 1, Time and The order in which a day resolves.
- Order intake in general, who may issue an order, origin and intent, saved orders: chapter 1, Orders.
- Search, reports, contacts and what a side believes about enemy forces: chapter 3.
- Readiness, alertness, the alarm and the fatigue of a force: chapter 3.
- Surface fights, sighting rolls, break-off levels, damage, hits and systems: chapter 4.
- Air groups aboard ships, air range, strikes, raids on a base, wear, repair and air supply: chapter 5.
- Fuel for aircraft: the base's supply stock, spent for sorties (chapter 5, Supply for sorties).
- The briefing, `measure` (passage, route and reach forecasts) and `explain`: not rules.
- Which events each side may see, in full: chapter 1, What a side sees of an event.
- Objectives, missions and results: chapter 6. The only rule of the engine that reads a task force's mission is shore bombardment (chapter 4), and the halt for it (above); the staff reads it to form a task (chapter 8, A task force as an addressee).
- Ship repair, forward and at the yard, and a force away at the yard: chapter 4, Repair.
- The bombardment a halted force makes: chapter 4, Shore bombardment.
- Land units, men worked as cargo, troop capacity, the men's supply, marching overland and preparing a landing, whose loading scales the rate of a landing: chapter 7.
- Base orders other than forming a task force, and port damage: not built (0052, 0066, 0111). Forming is in The task force formed, joined and stood down. The fight ashore and a base that changes side: chapter 7. Shore bombardment: chapter 4.
