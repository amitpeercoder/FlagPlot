# Movement, fuel and cargo

This chapter covers where ships can go, how a task force moves, what it burns, how it refuels and loads, and what becomes of its damaged ships. Every quantity is a fixed-point number, and "cut" below means cut toward zero at six decimals (chapter 1, Numbers and chance).

Where these rules run in a day (chapter 1, The order in which a day resolves, gives the whole of it):

| When | Rule |
| --- | --- |
| Start of the day, before orders | Refuelling in port |
| Start of the day | Orders are taken in, including route, speed, readiness, cargo and detach orders (chapter 1, Orders) |
| Start of each pulse | Alert from belief, then fatigue of the task forces (chapter 3, Readiness, alertness and fatigue of a force) |
| Movement phase of each pulse | Stragglers, movement, meetings and fights |
| Loading phase, last in each pulse | Cargo handling |

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

A map lists its land links: pairs of neighbouring hexes that men can march between (chapter 7, Marching overland). A link joins two hexes that are neighbours, are on the map and are each land or coast, and it runs both ways. A link is listed once. The map does not work out where men can walk from the kinds of hex: a hex of 40 miles can hold parts of two islands, so a link says that one landmass runs through both hexes and that the game treats the men in each as being on it. A map with no links allows no march, so no scenario on such a map changes. The links are map data: they are not part of the state and are not hashed. [0094 items 1 and 2]

A map file in the placeholder TOML form lists them as `[[land_link]]` entries with `a = [q, r]`, `b = [q, r]` and an optional `note` for the reader. A scenario may also name a links file with the key `land_links`, a path relative to the scenario file, whatever the map's format. It is a CSV with the header `q1,r1,q2,r2,landmass,reason`, one link on each row, and lines that start with `#` are comments. Each row needs a landmass, which says which landmass the link is for, and a reason. The links of the file are added to those of the map. The loader refuses a link between hexes that are not neighbours, a hex that is off the map or is neither land nor coast, a link listed twice (in one file or across the map and the file), a row without a landmass or a reason, and a header other than the one above. The file is a data file of the scenario. The real map has no links file yet. [0094 items 1 and 2]

A side sees: not an event. The map, and so every link, is common knowledge.

### Bases on hexes

A base is on one hex. A hex may hold several bases, of one side or of both. A base has a side, a supply stock and a fuel stock, both in tons (see Cargo and Refuelling). The scenario's base entry gives the stocks (`supply_t`, `fuel_t`), the base's anti-aircraft shots (`anti_aircraft`, chapter 5) and the damaged aircraft it repairs each day (`aircraft_repairs_per_day`, chapter 5), each 0 if not given. It also gives two keys for ship repair (chapter 4, Repair): `ship_repair_points_per_day`, the points of ship damage the base mends each day for the forces of its side stopped there (a whole number or a decimal string; 0 if not given; the loader refuses a negative value), and `yard_passage_days`, which makes the base a port for the yard with a passage there and back of that many days (a whole number of at least 1; none if not given; the loader refuses 0, a negative or a fractional value). A TOML float is refused for either. A base must be on the map. It need not be on a navigable hex, but a force can use a base only if it can be at the base's hex, so a base on land serves nobody. Where a rule looks for a base of a side at a hex, it takes the lowest-numbered base that matches. A base can also be formed in play: the first man a force lands in a hex where its side has no base forms one, with the next free number, the port of the hex, no stock, no guns and no repairs. It is on its own side's map only (chapter 7, A base formed by a landing). A base's side does not change in the movement and cargo rules; it changes only when its side is beaten in its hex, in the ground combat phase after the third pulse. The base keeps its number, name, hex, guns and repair figures, passes to the other side with part of its stocks and is on both sides' maps from then on (chapter 7, A base that changes side). Every rule that asks whose a base is reads the side it has at that moment, so from the next day cargo, refuelling and repair at it serve its new side. [0011, 0045, 0066, 0069, 0090]

A ship that is not in a task force may be in a port, which is a base. Such a ship does not move, refuel, work cargo or take ship repair: every rule here acts on task forces. Its position is the base's hex, and an air group aboard it uses that position. [0025]

A side sees: bases are known to their own side. Chapter 1, What a side may know, gives what a side is told of the enemy's.

### Port size

A hex has a port size from 0 to 10. A base's port is the port of the hex it stands on. `port_sizes.csv` gives, for each size, `berths`, `berth_rate_t_per_h` and `lighter_rate_t_per_h`. The file must have a row for the port size of every hex that holds a base, or the game does not start. A row may not have berths and no berth rate. Only cargo handling reads port size. The generated map gives every hex port size 0, so every base on it works cargo by lighter. [0011, 0052 item 15]

A side sees: not an event.

### What reads nothing

Airfield size is stored on a hex and no rule reads it. Terrain is stored and no rule reads it: the generated map marks every land hex jungle. The anchorage flag on a hex is stored and no rule reads it. Shallow sea is treated as sea. [0052 items 14 and 15]

## Task forces and ships

### A task force

A task force is a set of ships of one side, with one position, one route, one speed setting and one set of standing orders. Its fields that this chapter reads:

| Field | Meaning |
| --- | --- |
| `ships` | The ships in it. A ship afloat is in exactly one task force, one port or away at the yard. A sunk ship is in none. A force away at the yard lists none |
| `from_hex`, `toward_hex`, `progress_nm`, `leg_nm` | Where it is on a leg, and what the leg is charged (see What a step is charged and Legs and position) |
| `route` | Waypoints and the index of the next one |
| `status` | idle (no route), underway, arrived (reached the last waypoint), or out of fuel |
| `speed` | cruise, full, or a speed in knots |
| `readiness`, `alerted`, `fatigue` | See Readiness and fatigue |
| `rules_of_engagement` | Stance, break-off level, straggler instruction and share to keep |
| `home_port` | A base of its own side, set by order |
| `cargo_order` | The standing cargo order, if any |
| `mission` | Set by order. No rule reads it. [glossary] |
| `yard_trip` | Set while the force is away at the yard: its port, its day of return and the event that sent it. None otherwise (chapter 4, Repair) |

A task force with no ship afloat is never ordered, never moves and never fights. Every order for it is rejected with "no ship afloat". [0016 item 16, 0044]

A force that is **away at the yard** (chapter 4, Repair) has no ship in its list, because its ships are away. It is not sunk. It stays at its base's hex, idle, but no rule moves it, fights it, sights it, searches for it, refuels it or tires it: every rule that walks the forces skips a force with no ship. It takes no order until its day of return (chapter 1, Every reason an order is refused). Its ships are afloat and are counted as such (chapter 6). [0069]

What the enemy knows of a force away: nothing new. Nothing can sight a force with no ships, so the enemy's contact of it is not refreshed. The contact ages and is dropped by the existing rule (`search.toml: contact_drop_age_pulses`) as for any force that steams out of sight, and no report is made of the departure or the return. The enemy's events in a game in which a force goes to the yard are those of the same game with the force left stopped in port, as long as nothing of the enemy's would have sighted it there. [0069]

A scenario's `[[ship]]` entry may also start a ship damaged: `flotation_damage` and `fire_damage` (chapter 4, Damage tracks), each 0 if not given. A ship's bank of forward repair points and a force's `yard_trip` cannot be given by a scenario: every game starts with no work in hand and no force away. [0069]

A side sees: its own task forces exactly. The enemy's task forces reach a side only as contacts (chapter 3, Belief).

### Detaching ships by order

A detach order splits named ships off into a new task force. Chapter 1, Task force orders, gives the order, its checks and what the new force copies. Stragglers are detached by the same split, with changes (see Stragglers below).

A side sees: its own `ShipsDetached` event whole. The enemy sees nothing.

## Movement

### Orders that move a force

A route order replaces the force's route with a list of waypoints. Chapter 1, Every reason an order is refused, gives when it is refused: a waypoint off the map, a waypoint that cannot be reached by sea from where the route starts (the turn-back rule below picks where that is), or a route that goes nowhere.

An accepted route order sets the route to the waypoints with the next index 0, sets the status to underway, sets the route order, and clears the arrival hour and the arrival event. It never clears the cargo order, the readiness or the rules of engagement. A route order given to an arrived force sets it underway. A route order to a force that is out of fuel is refused (differences.md row 34): it stays out of fuel where it is until it takes fuel in port. [0005, 0017, 0044]

A route order may also carry marks: a list of hexes to act at, each one of the route's waypoints, else the order is refused (chapter 1, Every reason an order is refused). A mark is by hex: every waypoint at a marked hex is marked. The last waypoint is never treated as marked, so a force arrives there as it always did. A new route order replaces the marks with its own. What a marked waypoint does is in Arriving, below. [0096 items 1 and 2]

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

Within one pulse forces move in order of clock time. The engine works out, for each force that can still steam, the distance to its next change of position: the rest of the way to half the charge of its leg or, if it is already at half or past it, the rest of the leg plus half the charge of the next, if the path has a next hex. It considers only a change that its remaining budget reaches. For each such force the time of the change is its hours used so far, plus the open segment plus that distance divided by its speed. The force with the earliest time is advanced to that point, and the lower task force number wins a tie. This repeats until no force has a change within its budget. Each force then steams out what is left of its budget. [0022 items 1 and 2]

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

A force with a route that has marks may halt at one. The engine looks for the halt when the force's mover is made, at the start of each pulse's movement, and again after each fight. The halt is at the first marked waypoint, from the force's next one on, where the force has work. Work is one of two things, and a force may have both:

- **A bombardment.** The force's mission is bombardment, it is not away at the yard, it has a ship afloat, the hex holds a base of the other side that is on the force's side's map, the bombardment and air combat figures are loaded, and a ship of it would fire a shell by the rule of the shore bombardment phase (chapter 4, Shore bombardment).
- **Cargo.** The force has a cargo order that can work in that hex by the cargo rule below (a base of its side there, or men to land on a coast hex, or men of the order's unit ashore to load), and that is not complete by the cargo rule's own test of when nothing remains to move.

With no work the force passes the waypoint as any other. A mark never stops a force that has nothing to do there. The route forecast (`measure`) does not foresee halts.

A force that reaches the centre of the waypoint it halts at stops there, as a force that arrives does. Its status is halted and it is not moving. It records `TaskForceHalted` (the hex, the work, and the hours since the scenario began), citing its route order and the segment that brought it there, or its route and speed orders if it steamed no distance in the pulse. It is stopped as an arrived force is: it can be met and fought, it refuels and is repaired at a base of its side, it can be given a load or unload order, and it can be sent to the yard. It works cargo from the hour it halted, as an arrived force does from its arrival (Working cargo, below), and it bombards in the night pulse's shore bombardment phase (chapter 4, Shore bombardment). A force that halts by day to bombard waits for the night: the rule does not move the bombardment out of the night pulse.

At the start of each pulse's movement, before the straggler pass, each halted force in force order steams on if its work is done:

- **For a bombardment,** it has bombarded since it halted, or it can no longer bombard there: no ship of it can fire, or its mission or the base changed.
- **For cargo,** it has none left to work there, by the same test as above.

A force with both waits for both. It records `TaskForceSteamedOn` (the hex, the work, and whether it fired), citing its route order, the halt, the bombardment and the cargo order's completion event. It is then under way toward the waypoint after the one it halted at and moves in that pulse with the pulse's full hours.

A new route order ends a halt, and records nothing of its own: the order accepted is the record. So does a force retiring after a fight or a trip to the yard. A force that halts where its fuel also runs out is halted, and when it steams on it has no fuel and is out of fuel by the rule above. A halted force with no ship left stays as it is. [0096]

A side sees: its own `TaskForceHalted` and `TaskForceSteamedOn`, whole. The enemy sees neither.

### Turning back and re-routing

A route order given to a force between two centres starts the new route from the centre the force reaches first. The force goes on to the hex ahead or turns back to the hex it left, whichever is the shorter trip to the first waypoint of the new route:

- Going on costs the rest of the leg (the leg's charge less the progress) plus the charged miles of the sea path from the hex ahead to the waypoint, as What a step is charged gives them.
- Turning back costs the progress plus the charged miles of the sea path from the hex it left to the waypoint.
- A tie goes ahead. If only one side can reach the waypoint, that side is taken. If neither can, the order is rejected. A route that ends at the hex ahead or the hex left is a real route.

Turning back swaps the two hexes and sets progress to the leg's charge less its old value; the charge is kept. It happens when the order is taken in. The leg is then steamed as usual, and a re-route never gains distance. A force turned back at exactly 20 miles changes position, from the hex ahead to the hex it left, without steaming. A turn-back records `TaskForceTurnedBack` (the hex it now leaves from, the hex it now heads for, the progress and the position), citing the order and recorded just after the order's own event. A route order at a centre records none. [0005 item 5, 0017 item 14, 0044 item 7, 0058 item 7]

A side sees: its own `TaskForceTurnedBack`.

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

A side sees: `FuelBurned` for its own ships.

### The fuel limit on a pulse's budget

The distance a ship's fuel lasts at a speed is its fuel divided by the multiplier, then times `endurance_nm` over `fuel_capacity_t`. The force's fuel limit is the lowest of these over its ships. [0006 item 5]

A side sees: not an event. The breakdown of the steaming event holds the limit.

### Refuelling in port

At the start of each day, before the day's orders are taken in, every task force that is stopped at a base of its own side refuels. Stopped means: the status is not underway and the force is exactly at a hex centre, with no hex ahead. The base must be at that centre. Cargo handling asks the same (see Cargo orders; differences.md row 32). A force that arrived, never left, or ran out of fuel at that centre is stopped. A force out of fuel between two hexes takes nothing.

Forces are served in order of their number and ships within a force in order of theirs. For each ship the engine takes the lowest-numbered base of the force's side in the force's hex that has fuel above 0. The ship takes what it lacks up to its class's `fuel_capacity_t` or what the base has, whichever is less. If no base there has fuel, the rest of the force gets nothing. A ship that lacks nothing takes nothing and records nothing. Each refuelling records `ShipRefuelled`, with the tons taken, the ship's fuel, the base's fuel left and whether the force was restored. It cites the force's arrival event. Nobody orders it, it takes no time and no berth, and loading cargo does not stop it. A scenario whose bases have no fuel has no refuelling.

A force that was out of fuel is restored by the first ship that takes fuel: its status goes to idle and its route and route order are dropped, so it waits for a new order. An arrived force stays arrived. The fuel a base holds as stock is not the fuel for aircraft: air sorties use supply (chapter 5, Supply for sorties). [0045]

A side sees: `ShipRefuelled` for its own ships.

Limits: there is no refuelling at sea, no rate by port size, no port damage and no reserve held back at a base. [0045 deferred, 0046]

## Cargo

### Cargo and capacity

A ship carries supply and fuel as cargo, in tons. The two together never exceed the class's `cargo_capacity_t`. A ship also carries men up to the class's `troop_capacity_men`, a limit of its own: men take no cargo room and cargo takes no berths (chapter 7). A ship's cargo fuel is not the fuel it steams on: it cannot burn it. A base holds a supply stock and a fuel stock. A scenario may start a ship with either. [0011 item 1, 0046]

A side sees: its own cargo events (below).

### Cargo orders

There are four standing cargo orders. They are one kind of order, so a new one replaces the old. A completed order stays standing until it is replaced, and a route order does not clear it.

| Order | Takes effect |
| --- | --- |
| Load (supply) | Fills each ship's room, which is capacity less supply and fuel cargo aboard, from the base's supply stock |
| Load fuel | The same, from the base's fuel stock |
| Load with a land unit | Takes the unit's men ashore in the force's hex aboard first, then the named cargo (supply, fuel or none) as the rows above do. It needs no base: off a shore where the side has none it loads men only (chapter 7) |
| Unload | Lands the men aboard first, on a coast hex only, then moves everything aboard into the base's two stocks, supply first and then fuel. The first man landed where the side has no base forms one (chapter 7) |

A load order is rejected at intake if the force is underway, or if no base of the side is at the force's position. A load order with a unit has its own reasons and needs no base (chapter 1, Every reason an order is refused). An unload order is always accepted. A load order to a force that is out of fuel is refused (differences.md row 17). Both take effect in a loading phase in which the force is idle, arrived or halted (above, Halting at a marked waypoint) and is exactly at the centre of a hex that holds a base of its own side, as refuelling asks (differences.md row 32). Two cases need no base there, if the ground figures are loaded: a load order with a unit, which takes men off the shore, and an unload order with men aboard in a coast hex, which lands them (chapter 7). A force that is underway, or that is out of fuel, works no cargo, and neither does a force idle part-way along a leg, whatever hex it is nearest to. A load order is still accepted at intake for a force idle part-way along a leg whose position, the nearest centre, holds a base of its side; it then works no cargo until the force is at that centre (differences.md row 41). Cargo moves automatically only through an order: an arrival with no unload order does nothing. A force made by a split has no cargo order. [0011 items 6, 7, 10 and 13, 0046 items 2 and 3, 0066]

A side sees: its own `OrderAccepted` or `OrderRejected`.

### Berths and lighters

Berths belong to a base, or, for a force that lands men where its side has no base, to the base that landing will form. They are counted for each side's landing place in a hex, which is its lowest-numbered base there. At the start of each loading phase, before any cargo moves, berths are given out. Forces that hold a cargo order and can work cargo are taken in order of number, and the ships of each in order of number. A ship is given a berth if the base has one free and the ship has cargo to move. A ship has cargo to move if, for a load, it has room and the base's stock of that cargo is above 0 at this moment, or, for a load with a unit, men of the unit are ashore and it has room for them; and if, for an unload, it has supply or fuel cargo aboard, or men aboard in a coast hex. A force's scan stops at the first ship that finds no berth free. The stock read is the stock at the start of the phase, so a ship that loads from stock which another force unloads in the same phase has no berth, and works by lighter.

A ship with a berth moves cargo at the berth rate of the port. A ship with no berth works by lighter at the lighter rate, at the same time. A size 0 port has no berths, so every ship there works by lighter. There is no other limit on a port's daily traffic, and no limit on the size of ship a port takes. The figures are the placeholders of `port_sizes.csv`. [0011 items 2 to 4, 0066]

A side sees: not an event of its own. The effect is in the cargo events.

### Working cargo

Cargo moves in the loading phase, the last phase of every pulse, for the hours the force was stopped in the pulse. The force works from the later of the pulse's start and its arrival hour (the start of the pulse if it has no arrival hour) to the end of the pulse. A force halted at a marked waypoint works from the hour it halted in the same way, and its cargo events cite the halt in place of an arrival. A force that arrived eight hours into the night pulse works four hours of it. A force with no hours works nothing. The base used is the lowest-numbered base of the side at the force's position.

Forces are worked in order of number, ships in order of number, and each change to a stock applies at once, so a force that unloads can supply a force with a higher number that loads at the same base in the same pulse. For each ship:

- The most it can move in the pulse is its rate (berth or lighter) times the hours worked. Each man moved takes `ground.toml: handling_t_per_man` of it, and men move first, in whole men (chapter 7, Men worked as cargo). The rest of the bullets count the tons left.
- A load moves the least of that, the ship's room, and the stock.
- An unload moves supply first, the least of the most it can move and the supply aboard, and then fuel, the least of what the pulse allows after the supply and the fuel cargo aboard. A ship works one cargo at a time, in that order.

Before the cargo, a landing divides units and forms a base where chapter 7 says, and each ship moves its men. Each move above 0 records `CargoLoaded` or `CargoUnloaded`, naming the cargo, the tons moved, the tons now aboard and the base's stock, citing the cargo order and the force's arrival event (or, for a halted force, its halt). When a force has moved something this pulse and nothing remains to move, one more event is recorded: `LoadingComplete` or `UnloadingComplete`, with the hour the slowest ship finished (the start of work plus the longest of each ship's tons over its rate). It cites the cargo events. Nothing remains for a load when the stock is 0 or every ship is full. For an unload nothing remains when no ship has supply or fuel cargo aboard and, in a coast hex, no ship has men aboard. A load order with a unit records `UnitLoadingComplete` in place of `LoadingComplete` (chapter 7). [0011 items 5 and 12, 0046 items 4 and 5, 0066]

A side sees: `CargoLoaded`, `CargoUnloaded`, `LoadingComplete`, `UnitLoadingComplete`, `UnloadingComplete`, `MenEmbarked` and `MenLanded` for its own ships and forces. The enemy sees none.

### At sea

A ship at sea does not load or unload, and there is no refuelling at sea, no tanker and no transfer between ships. A ship that is low on fuel with a hold full of fuel cargo must reach a base and land the cargo before it can use it. [0046]

## Stragglers

A straggler is a ship whose propulsion is degraded or out. A ship with flotation damage of 50 or more is heavily damaged, which is a separate matter. A task force has a standing instruction for stragglers and a share to keep, set with its rules of engagement: keep (the default), detach, leave or retire. The instruction is applied to every force that has a straggler and an instruction other than keep, at the start of every movement step, in order of force number, and straight after each fight, to the two forces that fought, lower number first. A force made by the pass is not processed in the same pass. [0016 items 10 to 12, 0022 item 5]

The ships of the force are sorted by propulsion into sound (working), able to steam (degraded) and dead in the water (out).

- **Keep:** nothing happens.
- **Leave:** every straggler, degraded and out, goes into one new force that stays where it is. The parent carries on.
- **Detach:** the ships dead in the water are left behind as above. The degraded ones go into a new force with escorts and head for the home port. The number of escorts is the sum over the degraded ships of the `straggler_escorts` figure of `surface_combat.toml` for each one's kind. Escorts are taken from sound ships that have main guns, the least important kind first and the lowest number on a tie. The importance of a kind is in chapter 3, What a report holds. If there are not enough, all of those are taken. If the parent would keep fewer than its share to keep of the sound ships it had, or none, the force retires instead. A ship with degraded propulsion is not an escort and does not count toward the share to keep.
- **Retire:** the ships dead in the water are left, and the rest of the force, degraded ships included, heads for the home port together. The parent records `ForceRetired`, gets no route and idle status, and then takes the home-port order below. A force with no ship left to send home stays as it is.

A force made by the pass takes the side, mission, speed setting, home port and the rest from its parent as a split does, and is then set to the avoid stance with the straggler instruction keep and no share to keep, keeping the parent's break-off level. A force that retires is set the same way. Its name says whether it was sent home, left behind or detached.

Heading for the home port is a set-route order to the home port's hex, issued by the local commander (chapter 1, Who can issue) and taken in at once, so it is checked and applied as any route order. Its acceptance cites the event that detached or retired the force, and movement then cites the order. A force with no home port, or one already at the centre of its home port's hex, gets no order and stays where it is. A force that is left behind has no route and stays where it is, idle, even part-way along a leg. A force with no ship afloat is never sent anywhere.

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
- Objectives, missions and results: chapter 6. No rule here reads a task force's mission.
- Ship repair, forward and at the yard, and a force away at the yard: chapter 4, Repair.
- The bombardment a halted force makes: chapter 4, Shore bombardment.
- Land units, men worked as cargo, troop capacity, the men's supply and marching overland: chapter 7.
- Base orders and port damage: not built (0052, 0066). The fight ashore and a base that changes side: chapter 7. Shore bombardment: chapter 4.
