# The day, orders, and what a side knows

This chapter holds the rules that frame every other rule: how time is counted, the order in which a day resolves, the orders a side can give and how they are checked, what a side may know, how numbers and chance work, what is checked when a day closes, and what a saved game holds. Chapters 2 to 6 hold what happens inside the phases named here.

## Time

### The day and its pulses

A game day has three pulses, resolved in this order: night, morning, afternoon. [0001]

| Pulse | Hours | Starts (hours after the day starts) | Code |
| --- | --- | --- | --- |
| Night | 12 | 0 | 1 |
| Morning | 6 | 12 | 2 |
| Afternoon | 6 | 18 | 3 |

The codes feed the random number keys and the state hash. They are never renumbered. [0001, 0003]

A pulse is a time bucket. It does not mean darkness. Pulses are simultaneous across the whole map: there are no local-time pulses. [0001]

A side gives its orders before a day and cannot interrupt or give orders inside it. What happens inside a day is decided by the standing orders a unit already has and by the rules. The one exception is an order a rule gives for a unit on the spot (see "Who can issue"). [0055]

A side sees: nothing of the clock is hidden. Every side knows the day and the pulse.

### The clock

The simulation counts time as elapsed hours since the scenario began, held as a fixed-point number. Day 1 starts at elapsed hour 0. The start of a pulse is `(day - 1) * 24 + offset` hours, with the offsets in the table above. [0001]

A game day starts at 18:00 in the scenario's reference zone and ends at 18:00 the next clock day. The game day changes at 18:00, not at midnight. For the slice the zone is UTC+11. [0001]

| Pulse | Clock |
| --- | --- |
| Night | 18:00 to 06:00 |
| Morning | 06:00 to 12:00 |
| Afternoon | 12:00 to 18:00 |

A moment is shown as game day and clock time, `D2 21:30`. Elapsed hours are turned into minutes and rounded to the nearest minute, half a minute rounding up. A moment within half a minute of the end of a game day reads as the start of the next. This rounding is for display only and no rule reads it. [0001, 0032]

Limit: the clock is not anchored to a UTC instant, and no calendar date is kept. A scenario file carries no date field for the engine. 0001 describes both as intended and neither is built. The real light calculation, a clock for the whole map, and a scenario that starts at dawn are deferred. [0001]

### Darkness

Whether a hex is dark in a pulse is answered by one function of the hex and the pulse. Today it is dark exactly in the night pulse, in every hex. Rules that depend on light ask this function and never read the pulse name. Chapters 3, 4 and 5 use it. [0001, 0012]

A side sees: nothing is hidden. It is common knowledge.

## The order in which a day resolves

`resolve_day` runs one day. The steps are fixed. The phase lists are written in `resolve.rs`. [0001, 0022, 0055]

### Before the pulses

1. **Refuelling in port.** A force stopped at its own side's base fills its ships from the base's fuel stock. Chapter 2, Refuelling in port. [0045]
2. **Orders intake.** Every order submitted since the last day is numbered, checked and applied, or refused. Events carry no pulse. This chapter, Orders. [0008, 0043]

Refuelling comes before intake, so the orders of the day are checked against a state in which refuelling has happened.

### Each pulse

For each pulse in order (night, morning, afternoon):

1. **Alert from belief.** Each force with a ship afloat is alerted or not, from its side's contacts. Chapter 3, Alert from belief. [0021]
2. **Force fatigue.** Each force with a ship afloat gains fatigue if alert and loses it if not. Chapter 3, Fatigue of a force. [0037]
3. **Grounded bases.** The bases with no supply at this moment are listed: their air groups fly nothing in this pulse. Chapter 5, A base out of supply. [0039]
4. The pulse's phases, in the order below.
5. **Contact ageing.** Each side's contacts age, and old ones are dropped, after the pulse's last phase, night included. Chapter 3, Ageing and dropping. [0012]

Steps 1 to 3 run before the pulse's first phase. Each of them does nothing unless its figures are loaded: step 1 needs `surface_combat.toml`, step 2 needs `fatigue.toml`, step 3 needs `air_supply.toml`. Contact ageing needs `search.toml`. See Where figures come from and when a rule is loaded. [0021, 0037, 0039]

### The night pulse

| # | Phase | What runs in it | Where |
| --- | --- | --- | --- |
| 1 | Aircraft repair | Damaged aircraft are repaired at bases and carriers. | Chapter 5, Repair |
| 2 | Air arrival | Aircraft and pilots the scenario delivers on this day join their groups. | Chapter 5, Aircraft that arrive on a day |
| 3 | Air replacement | Replacement aircraft and pilots arrive at the bases the scenario names, from day 2. | Chapter 5, Replacements |
| 4 | Air supply upkeep | Each base pays the daily supply for the aircraft at it. | Chapter 5, Daily supply |
| 5 | Air group rest | The staff stands tired groups down and returns rested ones. | Chapter 5, Stand-down and return |
| 6 | Movement | The straggler pass, then forces move in order of clock time, and fight where they meet. | Chapter 2, Movement and Stragglers; chapter 4 |
| 7 | Night search | Empty. No search flies where it is dark. | Chapter 3, Air search |
| 8 | Surface engagements | Empty slot. The fights happen inside movement. | Chapter 4, Meeting |
| 9 | Shore bombardment | Empty. Not built. | None |
| 10 | Loading | Cargo is loaded and unloaded at bases. | Chapter 2, Cargo |

[0001, 0022, 0035, 0036, 0038, 0039, 0041, 0052]

### The morning and afternoon pulses

Both use the same list.

| # | Phase | What runs in it | Where |
| --- | --- | --- | --- |
| 1 | Air search | Air groups on the search mission fly their arcs and sighting reports follow. | Chapter 3, Air search |
| 2 | Strike decisions | Groups on a strike mission decide, on their side's belief, and launch. | Chapter 5, The strike decision |
| 3 | Air combat | Escorts, patrols, guns and attacks, then the damage. Reports and claims follow. | Chapter 5, The raid, step by step; chapter 3, Claims |
| 4 | Aircraft wear | Aircraft that flew beyond their radius may run out of fuel, the rest roll for wear, air groups gain fatigue, and bases pay for sorties. | Chapter 5, The end of a daylight pulse for aircraft |
| 5 | Movement | The straggler pass, then forces move in order of clock time, and fight where they meet. | Chapter 2, Movement and Stragglers; chapter 4 |
| 6 | Surface engagements | Empty slot. The fights happen inside movement. | Chapter 4, Meeting |
| 7 | Loading | Cargo is loaded and unloaded at bases. | Chapter 2, Cargo |

Air search runs before movement, so it sees the positions at the start of the pulse. No aircraft repair, arrival, replacement, upkeep or rest phase runs in a daylight pulse. Aircraft fly only in daylight pulses. [0012, 0026, 0035, 0036, 0039]

### Inside the movement phase

The movement phase is one step. In it the straggler pass runs first. Then each force gets its hours for the pulse. Forces that share a hex and will not move meet at the start. Then forces move in order of clock time, and a force that enters the hex of an opposing force fights there. The fights have no phase of their own. Chapter 2, Order of movement within a pulse, and chapter 4, Meeting, give the rules. [0022]

### After the pulses

The four end-of-day phases run in this order. All four are empty. [0052]

| Phase | State |
| --- | --- |
| Ground combat | Not built. Land units and ground combat are deferred to the step whose scenario first needs them. [0052] |
| Logistics | Empty. Refuelling is done before intake, and base supply is spent inside the air phases. [0045, 0039] |
| Air group upkeep | Empty. Repair, arrival, replacement, supply and rest run at the start of the night pulse. [0035, 0036, 0038] |
| Intelligence update | Empty. Contacts age at the end of every pulse, and reports merge when they arrive. [0012] |

Then the day closes:

1. The day counter advances. Until this point the state's day is the day before: a rule that runs inside the day is told its day as an argument.
2. The invariants are checked (see "Invariants").
3. The state hash is computed and appended to the daily hashes. [0004, 0043]

Chapter 6 holds the result of a game. It is worked out from the final state and the log and is read by no phase. [0054]

A side sees: it sees the events of its own units, as the event table below says, and its own briefing. It does not see the phases.

## Orders

### What an order is

An order is addressed to one unit: a task force or an air group. It carries an origin: who issued it, and the intent it serves if any. An order that is taken in gets a number, is kept whether accepted or refused, and has one event: accepted, or refused with a reason. [0008, 0055]

The orders that exist are the ones in the tables below. The spec lists more (form, disband, add ships, rebase, target priority, range limit, base orders, land unit orders). They are not built. [0008, 0050, 0052]

### Task force orders

| Order | What it sets | Standing slot |
| --- | --- | --- |
| Set route (list of waypoints) | The route, the route's next waypoint index (to 0), the status (to under way), the order that set the route; clears the recorded arrival and its event. It may turn a force round between two hexes (chapter 2, Turning back and re-routing). | Route |
| Set speed (cruise, full, or a speed in knots) | The speed setting and the order that set it. | Speed |
| Set mission (transport, surface combat, carrier, bombardment, escort) | The task force's mission. No rule reads it. | Mission |
| Load | A standing cargo order to fill every ship with supply from the base here. | Cargo |
| Load fuel | A standing cargo order to fill every ship with fuel as cargo from the base here. | Cargo |
| Unload | A standing cargo order to land all cargo, supply first, then fuel. | Cargo |
| Set rules of engagement (stance, break-off level, straggler instruction, keep share) | All four figures together, and the order that set them. | Rules of engagement |
| Set home port (a base) | The base a force heads for when it retires or sends stragglers home. | Home port |
| Set readiness (normal or battle) | The readiness and the order that set it. It does not touch whether the force is alerted. | Readiness |
| Detach ships (a list of ships) | Not a standing order. Splits the listed ships into a new force at once. | None |

[0008, 0011, 0014, 0016, 0017, 0021, 0046]

To change: differences.md row 17, a load order to a force that is out of fuel is to be refused, and row 34, a route order to a force with no fuel is to be refused.

The stance is engage or avoid. The straggler instruction is keep, detach, leave or retire. Chapter 4, Breaking off, and chapter 2, Stragglers, give what they do. The readiness is read by chapter 3, Readiness, alertness and fatigue of a force. [0014, 0016, 0021]

Detaching ships creates a new force with the next free task force number. It starts at the parent's exact position and leg (the charge of the leg with it), idle, with no route. It copies the parent's side, mission, speed setting and speed order, home port, readiness and readiness order, alerted flag and the event that alerted it, fatigue, and rules of engagement and rules order. The event `ShipsDetached` is recorded before the event that accepts the order, and cites the order. To change: differences.md row 24, the acceptance is to be recorded first. [0016, 0037]

### Air group orders

| Order | What it sets | Standing slot |
| --- | --- | --- |
| Set air mission: rest, search, naval strike, combat air patrol, or base strike on a named base | The mission and the order that set it. | Air mission |
| Set search arc (from bearing, to bearing, range in miles) | The arc and the order that set it. | Search arc |
| Set patrol share (0 to 1) | The share of its flyable aircraft a group on patrol keeps over its own hex. | Patrol share |
| Set rest threshold (0 to 100) | The fatigue above which the staff stands the group down at the start of the day. | Rest threshold |
| Set reach (normal or extended) | How far its strikes and escorts may go: the type's strike radius, or a share more (chapter 5, Radius and reach). Normal is the default. | Reach |

[0012, 0026, 0027, 0034, 0036, 0059]

### When an order is checked, and against what

An order is checked once, at the moment it is taken in, against the state as it then stands. For the scenario's script and for orders a side hands in, that is the start of the day, after refuelling. For an order a rule gives inside a day, it is the moment the rule gives it. [0008, 0050]

A side may ask the engine's own check early, with `check_order`. It numbers nothing, records nothing and changes nothing. Its answer is a forecast. The state may change before the day begins, and an order that passed the forecast can be refused at intake. [0050]

The check reads the truth, but a refusal may cite only facts the issuing side can see: its own units, the map, ownership and the reference data. A fact the side cannot see is never a reason. "Not your unit" is the single reason for a target that does not exist and for a target that belongs to the enemy, so a refusal never shows that an enemy unit exists. "Not your base" does the same for bases. The base strike reason "not an enemy base" is worded to cover a base that does not exist and a base the group's own side holds. [0008, 0034, 0050]

Limit: 0008 said an order that would fail on something the side does not know is accepted and fails in execution. No such failure exists, because every check today reads only what the side can see. The source comment on the check says orders are checked against the truth. [0008]

### Every reason an order is refused

The reasons are tried in the order written. The first that applies is the one recorded. All other orders are accepted.

For a task force order:

| # | Reason | When |
| --- | --- | --- |
| 1 | Not your unit | The force does not exist, or its side is not the issuer's. |
| 2 | No ship afloat | The force lists no ships (every ship it had is sunk). |

Then, by kind:

| Order | Reason | When |
| --- | --- | --- |
| Set route | Waypoint off the map | A waypoint is not on the map. The first waypoint is tried first, then each in turn. |
| Set route | Waypoint unreachable | The first waypoint cannot be reached by sea from the force's position (or the hex it would turn back to), or a later one cannot be reached from the one before. A land hex is unreachable. |
| Set route | Empty route | The force does not move anywhere: no waypoint other than where it already is, and no leg in progress to finish. |
| Load, Load fuel | Under way | The force's status is under way. |
| Load, Load fuel | No own base here | No base of the issuer's side is in the force's position. |
| Set rules of engagement | Break-off level out of range | The level is below 0 or above 1. |
| Set rules of engagement | Keep share out of range | The share is below 0 or above 1. Tried after the break-off level. |
| Set home port | Not your base | The base does not exist or is not the issuer's. |
| Detach ships | No ships to detach | The list is empty. |
| Detach ships | Ship not in task force | A listed ship is not afloat in the force. The first such ship in list order is named. |
| Detach ships | Would leave the task force empty | The listed ships, counted once each, are every ship of the force. |
| Set speed in knots | Speed not above zero | The speed is zero or below. |
| Set speed (cruise, full), Set mission, Set readiness, Unload | None | Always accepted. |

For an air group order:

| # | Reason | When |
| --- | --- | --- |
| 1 | Not your unit | The group does not exist, or its side is not the issuer's. |
| 2 | Air group lost | The group is aboard a ship that is sunk or does not exist. |

Then, by kind:

| Order | Reason | When |
| --- | --- | --- |
| Naval strike | No air combat figures | The scenario has none. |
| Naval strike | Aircraft attack with nothing | The group's aircraft type has no attack weapon. |
| Base strike | No air combat figures; aircraft attack with nothing | As for a naval strike, tried in that order. |
| Base strike | Not an enemy base | The base does not exist or is the group's own side's. Tried after the two above. |
| Combat air patrol | No air combat figures | The scenario has none. |
| Combat air patrol | Not a fighter | The group's aircraft type is not a fighter. |
| Rest, Search | None | Always accepted. |
| Set patrol share | Patrol share out of range | Below 0 or above 1. |
| Set rest threshold | Fatigue out of range | Below 0 or above 100. |
| Set reach | No air combat figures | The order is extended and the scenario has none. Normal reach is always accepted, so a group can go back to it. |
| Set search arc | Invalid bearing | A bearing is not a multiple of 10 from 0 to 350. The from bearing is tried first. |
| Set search arc | Invalid search range | The range is zero or below, or above the aircraft type's search range (`aircraft_types` file: `search_range_nm`). |

[0008, 0016, 0017, 0021, 0026, 0027, 0034, 0036, 0044, 0059]

A refused order changes nothing but the log and the order counter. It is kept in the order record. [0008]

The air group's aircraft type and the figures it names are checked when the game is built, not when an order is taken in. [0025]

A side sees: it sees its own acceptances and refusals whole, with the reason. The enemy sees nothing of an order. In what a player is shown, "not your unit" reads as "not known". [0047, 0050]

### What an accepted order does

An accepted order replaces the standing order of its slot and nothing else. A new route does not cancel a speed order. A new mission does not cancel a search arc. Cargo orders share one slot: Load, Load fuel and Unload replace each other. The four figures of the rules of engagement are replaced together. [0008, 0027, 0036, 0046]

A standing order stays until another of its slot replaces it. Nothing clears a cargo, speed, mission, readiness, rules, home port, patrol share, rest threshold, reach or arc order when it has been carried out. Two rules do drop the route order of a force: a force that ran dry and takes fuel in port, and a force retiring because of its stragglers. Chapter 2, Refuelling in port and Stragglers, says when. [0008, 0016, 0045]

A task force remembers which order set its route, speed, rules of engagement and readiness. An air group remembers which order set its mission, arc, patrol share, rest threshold and reach. Events cite them as causes. [0008, 0059]

A route order that finds a force between two hexes may turn it round. The event `TaskForceTurnedBack` follows the order's acceptance and cites the order. [0005, 0017]

A side sees: its own turn-back and detach events whole, with the cause. The enemy sees nothing. [0047]

### How orders are numbered and sorted

Orders are submitted per side and wait for the next day. At the start of the next day, intake takes the sides in a fixed order, Allied then Japanese. [0008]

Each side's orders are put in order of the unit they are for before they are numbered: task forces before air groups, then by unit number. That order is the order of the two kinds of unit in the code (`EntityRef`), and nothing in a rule depends on it. The sort is stable, so one unit's orders keep their order and a later order to the same unit replaces an earlier one. The sort is made where orders are taken in, so a script, a player and a staff are treated alike. [0043]

Each order takes the next order number, in one sequence for both sides in one game, starting at 1. The Allied side's orders on a day are numbered before the Japanese side's. An order a rule gives inside a day takes the next number at the moment it is given. The number appears in the event that accepts or refuses the order and in the causes of what follows. No roll is keyed on an order's number. [0008, 0043, 0055]

A player is shown a side's own count of its orders, 1, 2, 3, in the order they were taken in, and never the log's number. [0055]

### Who can issue

The issuer of an order is one of three, and is recorded as the order's origin. The engine treats them alike. The issuer is not hashed. [0008, 0055]

| Issuer | Meaning |
| --- | --- |
| Player | The holder of the seat gave the order by hand. An order a script gives for a scenario, and an order a model gives, are marked player. |
| Staff | The staff planner, between days. A staff working for an agent marks its orders staff. The planner is not built; a scenario file may script an order as staff. |
| Local commander | A rule acting for the commander on the spot, inside a day, on the standing orders a unit already has. |

Who holds a seat (a person, an agent or the script) is a different fact and is recorded once for a game, not on each order. [0008, 0050]

Today one rule gives an order inside a day: the straggler rule sends a force for its home port with a set route order, issued by the local commander. The order is checked like any other, may be refused, and cites the event that detached the force. A force with no home port, or already at the centre of the home port's hex, gets no order. [0016, 0055]

A rule's choice of a mission target, a break-off or a straggler's fate is not an order. Chapter 5, The strike decision, chapter 4, Breaking off, and chapter 2, Stragglers, give them. [0055]

## Sides, truth and belief

### Truth and belief

There are two sides, Allied (code 1) and Japanese (code 2). Where sides are taken in turn, the Allied side comes first. [0008]

The truth state is the real world: ships, task forces, bases, air groups, arrivals, replacements and the day. A belief state holds one side's contacts and claims. A side's belief is written only from reports (see chapter 3, Belief): a sighting report makes or refreshes a contact, a battle claim is stored, and a "nothing there" report drops a contact. Ageing only removes what reports once said and adds nothing. A contact or a claim never holds the true number of an enemy unit. [0007, 0012, 0028]

Anything that plays the game is handed a side's view and never the truth. The truth is read only by the runner, the tests, the replay viewer and the hash. [0007, 0053]

### What a side may know

| What | How it is known |
| --- | --- |
| Its own ships, task forces, bases and air groups | Exactly. There is no fog over a side's own units. Supply and fuel stocks of its own bases and ships are shown. |
| Its own replacement streams and arrivals | Exactly. |
| The enemy's forces | Only as contacts: a hex, a count, a kind and an age, each made and refreshed by reports, each possibly wrong. |
| What it believes came of its fights and raids | Its claims. The damage it does to the enemy is truth and is not shown to it. The damage done to its own ships and aircraft is shown to it whole. |
| The enemy's bases | Number, name, hex and the holding side, and nothing else. This is the map, not intelligence. |
| The map | In full. |
| Reference data (ship classes, aircraft types, figures) | In full. |
| Its own orders | All of them, accepted or refused, with the causes it may see. |
| Its own mission and objectives | Its own only (chapter 6, What a side knows of its progress). |
| What happened | The events it may see, cut as the next section says. |

A side learns of what its forces did when a day is over. [0007, 0034, 0047, 0050, 0055]

Limit: a side knows the log's own number for an enemy ship it has fought, for the length of that fight and in the log afterwards. It could tell that two enemy ships are the same ship. Contacts are meant to hide that. It is accepted for step 1. [0047, 0055]

Limit: a force made during a game takes the next free task force number, and the enemy's forces use the same numbers. A side's new force number therefore depends on how many forces the enemy has, and because draws are keyed on a force's number, the luck of later forces depends on it. [0047]

Limit: there are no claims for aircraft shot down in an air fight except the raid claim of decision 0056. A side sees its own air fight losses and nothing of the enemy's. [0047, 0056]

## What a side sees of an event

### The rule

The event log is the truth. It holds both sides' orders, rolls, hits and losses. A side is given its own version, and each event is, for that side, whole, cut down, or absent. Nothing is added or altered. A cut event is a variant with no field for what is removed, so nothing removed can be carried in it. A cut event keeps its number in the full log. [0047]

The rule is one exhaustive `match` over every kind of event in `side_events.rs`. A new kind of event does not compile until it is placed in the table. [0047]

An enemy unit in an event a side may see is named as the log names it. [0047]

### The table

"Own" means the unit named is the side's own. Every kind not named is shown to no side.

**Whole if own**

| Event | Whole to the side when |
| --- | --- |
| Order accepted, order rejected | The side issued the order. (The order decides. Where the order is unknown, the target's side decides.) |
| Task force steamed, arrived, out of fuel, turned back, fatigue | Own force |
| Loading complete, unloading complete | Own force |
| Force retired, force stood down | Own force |
| Force alerted by contact, force became alert | Own force |
| Force surprised | The force surprised, not the one that surprised it |
| Force broke off | Own force |
| Force slipped away | The force that slipped away, not the one it slipped away from |
| Ships detached | The parent force is own |
| Alarm roll | Own force, and only if the roll succeeded (alarmed). A failed alarm means the force noticed nothing. |
| Fuel burned, ship refuelled | Own ship |
| Cargo loaded, cargo unloaded | Own ship |
| Ship damaged, system hit, ship sunk | Own ship |
| Search flown | Own air group |
| Aircraft worn, aircraft ditched, aircraft repaired, aircraft arrived, replacements received | Own air group |
| Damaged aircraft destroyed on the ground, aircraft destroyed on the ground | Own air group |
| Air group lost, air group fatigue, air group stood down, air group returned | Own air group |
| Strike launched, base strike launched, strike found nothing, escorts sent | Own air group |
| Aircraft lost, pilots lost | Own air group |
| Base supply destroyed, base supply spent | Own base |
| Report received, claim received, contact aged, contact dropped, contact not there | The side that holds the contact or made the report |
| Engagement begun, engagement ended | A side that has come to know of the fight (below) |
| Sighting roll | Own force, and only if the roll sighted. A roll that did not is hidden from both sides: it would say there was something to be found. |

**Cut**

| Event | The firer sees | The target sees |
| --- | --- | --- |
| Salvo | Its own salvo: the round, firer, target, weapon and shots. Not the chance, the expected hits or the hits scored. | That it was fired on: the round, the firing ship, the target and the weapon. Not the shots. The hits it took reach it as ship damaged. |
| Air attack on a ship | Its own attack: group, force, ship, weapon and shots. | That its ship was attacked: the group, the task force, the ship and the shots (one shot for each aircraft that attacked). Not the chance or the hits. |
| Air fight | Its own fighters firing: firer, target, what fired and at what, and shots. | Nothing. |
| Anti-aircraft fire (a force's) | Its own force's fire: force, raid group and shots. | Nothing. |
| Base anti-aircraft fire | Its own base's fire: base, raid group and shots. | Nothing. |
| Base attacked | The attacker sees that it attacked: group, base and shots. Not what was hit. | The base's side sees the event whole. |
| Search attempt | Only if it sighted, its own search: the air group and the task force sighted. Not the distance or the chance, which are worked from where the force really was, while the report may name a neighbouring hex. An attempt that did not sight is hidden from everyone. [0061] | Nothing. |

**Absent for both sides**

| Event |
| --- |
| Forces passed unseen (a pass in the dark) |

A side that is neither named sees nothing of the cut events. [0047]

### A surface fight

A side knows of a surface fight from the earliest round in which one of these happens, found from the fight's own events between its `EngagementBegun` and `EngagementEnded`: its force's successful sighting roll, a salvo fired at its ships or by them, hits on its ships, its force being surprised, or its force slipping away. A fight the side never knows of is wholly absent for it, including its begun and ended events. For a fight it knows, those two events are shown whole at their place in the log. [0047]

An event of a fight that carries a round is shown only if its round is not before the round the side first knew. Round 0 hits are outside a fight. A force that breaks off in a round in which it has neither sighted nor been fired on, because its ships were crippled before the meeting, is not shown the break-off. The force's state still changes. [0014, 0047]

### Causes and breakdowns

The causes of an event are cut the same way. A cause that is an order is kept if the side issued it. A cause that is an event is kept if the side may see that event. A cause that is dropped sets a flag on the event saying it has causes the side cannot see. [0047]

A side sees the whole breakdown, with its result, of an event it sees whole. A cut event carries only the factors that are the side's own, and no result. A salvo keeps `guns`, `tubes`, `rate`, `minutes`, `main_battery_factor`, `torpedo_mount_factor` and `fire_control_factor`. An air attack and a base attack keep `aircraft`. An air fight and anti-aircraft fire keep `shots`. A search attempt keeps `aircraft`, `sweep_width_nm` and `arc_deg`. A salvo against the side and an air attack against it keep no factor. [0047, 0061]

A claim is the exception. A side sees its own `ClaimReceived` whole and no breakdown of it, because the breakdown says how much of the claim is real. The log keeps it for the umpire. [0060]

Recording breakdowns never changes what happens or the hash. [0004]

### Own events, enemy knowledge

What the enemy learns of an event of the side's units is only what the rules give it by other means: a report, a claim or a contact. The first shot at a force is shown to that force as a cut event. A strike is seen by the defender as the attack on its ship. Nothing else of the enemy's orders, movements, stocks, fatigue, repairs or decisions reaches a side. [0047]

## Numbers and chance

### Fixed-point numbers

All simulation state is fixed-point. A `Fixed` is a signed 64-bit integer counting millionths: six decimal places. There is no floating point in the simulation. [0002]

- Adding and subtracting are exact. Overflow panics in every build profile.
- Multiplying and dividing use a 128-bit intermediate and cut the result toward zero, so a result loses up to one millionth in each operation. Cutting toward zero applies to negative values too.
- `mul_div(a, b, c)` computes `a * b / c` in one step, to avoid cutting in the middle. A ratio `num / den` is cut toward zero.
- A fixed value times a whole number is exact.
- Dividing by zero panics.
- The whole part of a value is cut toward zero.
- The order of operations in a chain changes the result in the last digits, so each rule states its order.
- A data file gives a quantity as a whole number or a decimal string with at most six decimals. A float is refused.

[0002]

### Where figures come from and when a rule is loaded

A figure comes from one of three places. A **data file** in `data/reference/` holds the figures of a rule, named in this book as `file: key`. The **scenario file** holds the starting state, named in this book as the entry and its field (for example a base entry's `anti_aircraft`). The **code** holds a few figures that have no file, and the text says so where it uses one.

A rule's figures are **loaded** when the scenario file names the file that holds them. A rule whose file the scenario does not name does nothing, and the state it would have written stays as the scenario started it. This is the one meaning of "loaded" in the book. A scenario file always names `ship_classes`, `port_sizes`, `aircraft_types`, `search` and `surface_combat`, so the rules that read those are always loaded. It may also name these, and each is loaded only if it does:

| Scenario key | File | Rules that run only if it is loaded |
| --- | --- | --- |
| `air_combat` | `air_combat.toml` | Strike and base strike missions, combat air patrol, escorts, raids, anti-aircraft fire, the false claims of a raid, reach beyond the radius with running out of fuel, and the pilots of a patrol shot down over its own hex. Without it the mission orders and the extended reach order are refused. A file that names the scenario must hold every key: one without a key is refused when the scenario loads. |
| `air_upkeep` | `air_upkeep.toml` | Wear and repair of aircraft. |
| `fatigue` | `fatigue.toml` | Fatigue of task forces and of air groups, stand-down and return, and the fatigue factor of a sighting. |
| `air_supply` | `air_supply.toml` | Supply for sorties and the daily supply, bases out of supply, and the supply a repair costs. |

A scenario with air groups must also name search figures, and a scenario with surface combat figures must also name them. [0021, 0026, 0035, 0036, 0037, 0039]

### How a chance is rolled

A chance is a fixed-point value from 0 to 1. It is rolled as follows. One draw is taken from the stream of the entity involved. The draw is a uniform whole number from 0 to 999,999. The roll succeeds if the draw is below the chance in millionths. A chance below 0 is read as 0 and a chance above 1 as 1. A chance of 0 never succeeds and a chance of 1 always does. The draw uses rejection, so no value is favoured. [0003]

### Every draw is keyed

There is no generator that advances as the day runs. Each draw is a pure function of a key, and so is independent of the order in which entities are processed. [0003]

A key is made of:

| Part | What |
| --- | --- |
| Seed | The game's seed. It is hashed. |
| Day | The day. |
| Pulse | The pulse code, or 0 for a phase that belongs to no pulse. |
| Subsystem | Movement 1, Search 2, Air combat 3, Surface combat 4, Ground combat 5, Logistics 6, Upkeep 7, Intelligence 8. |
| Entity | The kind and number of the entity together, so ship 3 and task force 3 differ. |
| Second entity | Present for a draw about a pair, such as an air group and the force it looks for. The order of the two is fixed by the rule and swapping them gives a different stream. Absent otherwise. |
| Rule | The `Draw` value of the rule that draws. `Legacy` (code 0) adds nothing to the key and is the eleven draws that existed when the tag was added. A new rule takes its own value at the end. |

Each part has its own tag so equal values in different parts cannot cancel. A key opens a stream, and the n-th draw of the stream is mixed from the key and n. A rule that needs several numbers for one key takes them from that one stream in a fixed order. No rule shares one stream between entities and no stream is carried from one pulse to the next. [0003, 0013, 0055]

The values of `Draw` are `Legacy` (0), `Claim` (1, the false claims of a raid, one stream for each side's claim keyed on its first firing unit and the raid's first strike group) and `Ditch` (2, the aircraft that run out of fuel and the recovery of their crews, one stream for each air group and pulse, in the upkeep subsystem, keyed on the air group alone) and `PatrolPilot` (3, the pilots of a patrol's aircraft shot down over its own hex, one stream for each firing roll's losses to a patrol group, in the air combat subsystem, keyed on the patrol group and the firing escort group). [0055, 0056, 0059]

The eleven legacy draws, told apart by the shape of their keys:

| Rule | Subsystem | Entity | Second entity |
| --- | --- | --- | --- |
| Search sighting | Search | air group | task force |
| Escorts fire at the patrol | Air combat | escort air group | target task force or base |
| Patrol fires | Air combat | patrol air group | raid's air group |
| Anti-aircraft fire | Air combat | ship or base | raid's air group |
| Air attack on a ship | Air combat | striking air group | task force |
| Air attack on a base | Air combat | striking air group | base |
| Aircraft wear | Upkeep | air group | none |
| Alarm roll | Surface combat | task force | none |
| Sighting in a fight | Surface combat | task force | task force |
| A ship's fire | Surface combat | ship | task force |
| Sighting report | Intelligence | task force | task force |

[0055]

Limit: escort fire and an air attack have the same key shape. They do not meet because a group has one mission. A rule that lets one group do both in a pulse must give escort fire its own `Draw`. [0055]

The rules for each draw are in chapters 3, 4 and 5. The output of the generator is pinned by a test, because it is part of the save format. A change to it needs a decision record. [0003]

## Invariants

At the end of every day, after the third pulse and the end-of-day phases, the engine checks the state. They are also checked when a game is built. A violation is always a bug in the rules, never a legal state. If any is broken, `resolve_day` returns an error naming the day and every violation, and no hash is recorded for the day. The day counter has already advanced and the events are kept. To change: differences.md row 25, the invariants are to be checked before the counter advances. [0004]

The checks:

| Group | What must hold |
| --- | --- |
| Ships | Fuel, supply aboard and cargo fuel are not negative. Fuel does not exceed the class's capacity. Supply and cargo fuel together do not exceed the class's cargo capacity. Flotation and fire damage are from 0 to 100. Gun rounds are not negative. A ship at 100 flotation damage is sunk. A ship has no damaged system its class lacks. |
| Where a ship is | A ship in a task force is listed by that force. A ship in a port is at a base that exists. A force lists only ships that are in it, that exist, that are not sunk and that are of its side. No ship is listed by more than one force. |
| Task forces | The force's hex is on the map and navigable. A hex ahead is set if and only if progress toward it is above 0; it is adjacent, on the map, navigable, and the step to it crosses no closed side. Progress is at least 0 and below the leg's charge, `leg_nm`; `leg_nm` is from 1 to 40 miles and is 40 for a force at a centre. The keep share is from 0 to 1. Fatigue is from 0 to 100. A home port exists and is of the force's side. |
| Bases | The hex is on the map. Supply and fuel stocks are not negative. A replacement stream names a base that exists. |
| Air groups | Each is at a base that exists or aboard a ship that exists, of its own side. The ready and damaged aircraft of the groups aboard a ship do not exceed the class's aircraft capacity. Fatigue and the rest threshold are from 0 to 100. |

[0004, 0016, 0017, 0025, 0035, 0036, 0037, 0038, 0045, 0046]

Three invariants hold by construction and are not checked: an air group's ready aircraft cannot be negative; an air group has exactly one location; and a side's belief is written only from reports. [0052]

## The state hash and saved games

### The state hash

At the end of each day the engine computes one 64-bit hash of the state and appends it to the daily hashes. It is FNV-1a over a fixed-order walk of the state, begun by the version string `flagplot-state-v4`. A task force's `leg_nm` is written right after its `progress_nm`; version 3 added it. An air group's reach code and the order that set it are written last, after its arc order; version 4 added them. [0004, 0057, 0058 item 7, 0059]

The hash covers: the seed; the day; every ship, task force, base, air group, arrival and replacement stream; both sides' contacts and claims; the count of orders and the next order number and next task force number; and the count of events. It does not cover the names of ships, task forces, bases or air groups, the contents of the events or orders, the map, the reference data, the orders waiting for the next day or the objectives. No tag byte stands in for a field that is not in use. Every field of every unit is written, always, in one order. A field added to the state is added to the hash in the same change, and the version is raised when what is hashed or its order changes. [0004, 0055, 0057]

Two runs of the same scenario, orders and seed give the same hash on any machine. The same hash is given with or without breakdown recording, in two processes, and with the scenario's tables listed in a different order. [0043]

Shuffling means the order the data and the orders are given in, not the order the rules work in. The rules' own order is part of the rules: losses come off the lowest-numbered group first, forces move in order of clock time, a base repairs the lowest-numbered group first. [0043]

### What a saved game holds

A saved game is a file with: the scenario file's name and its hash; each data file the scenario names with its hash; the seed; who held each seat (player or script); the orders of each day for both sides as they were handed in, with their unit, order and origin; and the daily hashes of the run. It holds no state. The starting state is the scenario's. The orders a rule gave inside a day are not saved, because the rules give them again. [0043, 0050]

A saved game made before seats existed is read as both seats held by the script. [0050]

A batch summary and a replay file for the viewer carry `inputs`: the version of the hash layout, the hash of the scenario file, and each data file the scenario names with its hash. This lets a reader see that two files were made from different inputs. `batch --label` adds the caller's own name for the engine that ran, since the engine does not know its own revision. [0057]

### Replay

`replay` loads the scenario the saved game names, refuses a scenario file or data file whose hash has changed or whose list of data files has changed, hands in the saved orders day by day with no player present, and compares each day's hash with the saved one. It fails on the first day that differs. [0043, 0050]

A replay must reproduce every daily hash. [0043]

Limit: a saved game is not promised to survive a change of rules. After a rule change a replay may fail partway with a hash that does not match. [0055]

Limit: a game cannot be saved in the middle and resumed. It is replayed from the start. The version of the hash layout is not written into a saved game, only the hashes of the files. A saved game made before the change to hash version 2 fails at day 1. [0043, 0057]

A side sees: none of this. A saved game, the hashes and the seed belong to the umpire.

## Not in this chapter

- How forces move, burn fuel, refuel, and load and unload cargo, and what becomes of stragglers: chapter 2.
- Air search, sighting reports, contacts, their ageing, readiness and alertness, the alarm, fatigue of forces, and claims: chapter 3.
- Surface fights, the hit mechanism and damage to ships: chapter 4.
- Air groups, strikes, escorts, patrols, anti-aircraft fire, wear, repair, fatigue of groups, pilots, replacements and base supply: chapter 5.
- Missions, objectives, points, margins and results: chapter 6.
- The agent tools, the briefing, `explain`, `measure`, the serve command and the MCP server: not rules.
- The figures: the data files in `data/reference`.
