# The day, orders, and what a side knows

This chapter holds the rules that frame every other rule: how time is counted, the order in which a day resolves, the orders a side can give and how they are checked, what a side may know, how numbers and chance work, what is checked when a day closes, and what a saved game holds. Chapters 2 to 7 hold what happens inside the phases named here.

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

1. **Returns from the yard.** Each force whose day of return is this day, in force number order, comes back to its base with its ships repaired, refuelled and rearmed. Chapter 4, Repair. [0069]
2. **Refuelling in port.** A force stopped at its own side's base fills its ships from the base's fuel stock. Chapter 2, Refuelling in port. [0045]
3. **Orders intake.** Every order submitted since the last day is numbered, checked and applied, or refused. Events carry no pulse. A yard order, a rebase order, a posture order, a march order, a prepare-landing order and an activity order are applied here, at their acceptance. This chapter, Orders; chapter 5, Rebasing an air group; chapter 7, The posture and its order, Marching overland, Preparing a landing, and The day's activity. [0008, 0043, 0069, 0090, 0091, 0094, 0095, 0099]
4. **Forward repair.** Each base that mends points of ship damage a day spends them on the forces stopped in its hex. Chapter 4, Repair. [0069]

Returns come first, so that a force back from the yard is at its base for the day's refuelling and for its orders. Refuelling comes before intake, so the orders of the day are checked against a state in which refuelling has happened. Forward repair comes after intake, so that the days a yard order states are the days the force leaves with, and a force ordered to sail that morning is not repaired that day. None of the four is a pulse phase: each runs once a day, outside the pulses. [0045, 0069]

### Each pulse

For each pulse in order (night, morning, afternoon):

1. **Alert from belief.** Each force with a ship afloat is alerted or not, from its side's contacts. Chapter 3, Alert from belief. [0021]
2. **Force fatigue.** Each force with a ship afloat gains fatigue if alert and loses it if not. Chapter 3, Fatigue of a force. [0037]
3. **Grounded bases.** The bases with no supply at this moment are listed: their air groups fly nothing in this pulse. The damage to each base's airfield is read with them: its groups fly the share of their aircraft the field works in this pulse. Chapter 5, A base out of supply and A damaged airfield. [0039, 0103]
4. The pulse's phases, in the order below.
5. **Contact ageing.** Each side's contacts age, and old ones are dropped, after the pulse's last phase, night included. Chapter 3, Ageing and dropping. [0012]

Steps 1 to 3 run before the pulse's first phase. Each of them does nothing unless its figures are loaded: step 1 needs `surface_combat.toml`, step 2 needs `fatigue.toml`, step 3 needs `air_supply.toml` for the grounded bases (the airfield damage it reads needs no file, and cuts the aircraft whatever is loaded). Contact ageing needs `search.toml`. See Where figures come from and when a rule is loaded. [0021, 0037, 0039]

### The night pulse

| # | Phase | What runs in it | Where |
| --- | --- | --- | --- |
| 1 | Aircraft repair | Damaged aircraft are repaired at bases and carriers. | Chapter 5, Repair |
| 2 | Air arrival | Aircraft and pilots the scenario delivers on this day join their groups. | Chapter 5, Aircraft that arrive on a day |
| 3 | Air replacement | Replacement aircraft and pilots arrive at the bases the scenario names, from day 2. | Chapter 5, Replacements |
| 4 | Air supply upkeep | Each base pays the daily supply for the aircraft at it. | Chapter 5, Daily supply |
| 5 | Men's supply | Each side's men ashore are fed from every base of its side in their hex, the unfed are lost in part, and each unit's fed share is set. Runs only if the ground figures are loaded. | Chapter 7, The men's supply |
| 6 | Air group rest | The staff stands tired groups down and returns rested ones. | Chapter 5, Stand-down and return |
| 7 | Movement | Halted forces whose work is done steam on, then the straggler pass, then forces move in order of clock time (a force halts at a marked waypoint where it has work), and fight where they meet. | Chapter 2, Movement, Halting at a marked waypoint and steaming on, and Stragglers; chapter 4 |
| 8 | Night search | Empty. No search flies where it is dark. | Chapter 3, Air search |
| 9 | Surface engagements | Empty slot. The fights happen inside movement. | Chapter 4, Meeting |
| 10 | Shore bombardment | A force on the bombardment mission, in the hex of an enemy base on its side's map, fires its guns at the base: aircraft, supply and men there are hit, the base's side is given a sighting report of the force, and the bombarding side a claim. Runs only if the bombardment figures and the air combat figures are loaded. | Chapter 4, Shore bombardment |
| 11 | Loading | Cargo is loaded and unloaded at bases, and men are taken aboard and landed. A force that lands men works at the rate its loading gives, and a unit's first men landing spend its rehearsal. | Chapter 2, Cargo; chapter 7 |

The men's supply follows the daily supply for aircraft, so both charges fall on one stock with the aircraft's first. Shore bombardment follows movement, so a force fires from the hex it ended the night's movement in. [0001, 0022, 0035, 0036, 0038, 0039, 0041, 0052, 0066, 0092]

### The morning and afternoon pulses

Both use the same list.

| # | Phase | What runs in it | Where |
| --- | --- | --- | --- |
| 1 | Air search | Air groups on the search mission fly their arcs and sighting reports follow. | Chapter 3, Air search |
| 2 | Strike decisions | Groups on a strike mission decide, on their side's belief, and launch. | Chapter 5, The strike decision |
| 3 | Air combat | Escorts, patrols, guns and attacks, then the damage (a hit on a base may land on the men ashore there). Reports and claims follow. | Chapter 5, The raid, step by step; chapter 3, Claims |
| 4 | Aircraft wear | Aircraft that flew beyond their radius may run out of fuel, aircraft whose deck is lost land elsewhere or come down in the sea, the rest roll for wear, air groups gain fatigue, and bases pay for sorties. | Chapter 5, The end of a daylight pulse for aircraft |
| 5 | Movement | Halted forces whose work is done steam on, then the straggler pass, then forces move in order of clock time, and fight where they meet. | Chapter 2, Movement, Halting at a marked waypoint and steaming on, and Stragglers; chapter 4 |
| 6 | Surface engagements | Empty slot. The fights happen inside movement. | Chapter 4, Meeting |
| 7 | Loading | Cargo is loaded and unloaded at bases, and men are taken aboard and landed. | Chapter 2, Cargo; chapter 7 |

Air search runs before movement, so it sees the positions at the start of the pulse. No aircraft repair, arrival, replacement, upkeep or rest phase runs in a daylight pulse. Aircraft fly only in daylight pulses. [0012, 0026, 0035, 0036, 0039]

### Inside the movement phase

The movement phase is one step. In it the straggler pass runs first. Then each force gets its hours for the pulse. Forces that share a hex and will not move meet at the start. Then forces move in order of clock time, and a force that enters the hex of an opposing force fights there. The fights have no phase of their own. Chapter 2, Order of movement within a pulse, and chapter 4, Meeting, give the rules. [0022]

### After the pulses

The five end-of-day phases run in this order. Ground combat is built and runs only if the ground figures are loaded; the next three are empty. [0052, 0067, 0090]

| Phase | State |
| --- | --- |
| Ground combat | First the marches: each unit with a march, in unit order, stops or marches a day, and arrives on the last day with the supply it carries (a unit that fell back carries none). Then, for each hex with men ashore after the marches, in hex order: the day's fight if both sides have men of fighting units there and one is on attack (each side's fire limited to a multiple of the other's men, and the test of an attack that fails worked before any die), each unit's fatigue (with the fatigue of a failed day, and the strain of a day under fire in place of rest), the tired back to defend, what each spent attacker does (holds, or falls back to a linked hex, set to march, tired and carrying nothing), the test of a side beaten, the bases of a beaten side changing side with what they hold and what pointed at them, the beaten side's units with men ashore falling back to a linked hex (set to march, on the defence, tired by the defeat) or, with no hex to go to, surrendering, and last the ground reports of the hex. Then, once every hex is done, the day's activity: each force's loading, then each unit's one activity (rehearsal, training or work; the work of a unit on the airfield of its base mends the field's damage), then the days of fighting counted and the experience they and training raise. Then the day's digging (each fighting unit on defend that did not march gains fieldworks), and last the day's hits on men, counted by the shelling and bombing of the pulses to give the strain, are emptied. Chapter 7; chapter 3, The ground report. It runs after the third pulse, so no aircraft is in the air, and before the base count, so the count reads the day's change of side and the supply a march put down. A game with no ground figures has no such phase. [0090, 0092, 0094, 0095, 0097, 0099, 0103, 0104, 0105, 0106] |
| Logistics | Empty. Refuelling is done before intake, and base supply is spent inside the air phases. [0045, 0039] |
| Air group upkeep | Empty. Repair, arrival, replacement, supply and rest run at the start of the night pulse. [0035, 0036, 0038] |
| Intelligence update | Empty. Contacts age at the end of every pulse, and reports merge when they arrive. [0012] |
| Base count | For every base, in base order, a `BaseAtDayEnd` event records its supply and the ready aircraft of the air groups at it. It is recorded whatever the objectives say, belongs to no pulse, has no cause and changes no state. `base_open` is counted from it (chapter 6). [0067] |

Then the day closes:

1. The invariants are checked (see "Invariants"). The state's day is still the day before: a rule that runs inside the day is told its day as an argument.
2. The day counter advances.
3. The state hash is computed and appended to the daily hashes. [0004, 0043]

Chapter 6 holds the result of a game. It is worked out from the final state and the log and is read by no phase. [0054]

A side sees: it sees the events of its own units, as the event table below says, and its own briefing. It does not see the phases. A base that changed side in the ground combat phase is the new holder's for the base count and for the invariants that follow. [0090 item 24]

## Orders

### What an order is

An order is addressed to one unit: a task force, an air group or a land unit. It carries an origin: who issued it, and the intent it serves if any. An order that is taken in gets a number, is kept whether accepted or refused, and has one event: accepted, or refused with a reason. [0008, 0055]

The orders that exist are the ones in the tables below. The spec lists more (form, disband, add ships, range limit, base orders, land unit orders other than posture, march and activity, and an order that joins forces). They are not built. [0008, 0050, 0052, 0090, 0094, 0095, 0099]

### Task force orders

| Order | What it sets | Standing slot |
| --- | --- | --- |
| Set route (list of waypoints, and optionally `act_at`, a list of hexes to act at) | The route, its marks, the route's next waypoint index (to 0), the status (to under way), the order that set the route; clears the recorded arrival and its event, and any halt (chapter 2, Halting at a marked waypoint and steaming on). It may turn a force round between two hexes (chapter 2, Turning back and re-routing). | Route |
| Set speed (cruise, full, or a speed in knots) | The speed setting and the order that set it. | Speed |
| Set mission (transport, surface combat, carrier, bombardment, escort) | The task force's mission. Only one rule reads it: a force on the bombardment mission fires at an enemy base (chapter 4, Shore bombardment). | Mission |
| Load | A standing cargo order to fill every ship with supply from the base here. | Cargo |
| Load fuel | A standing cargo order to fill every ship with fuel as cargo from the base here. | Cargo |
| Load with a land unit (`load` with a `unit` and a cargo of supply, fuel or none) | A standing cargo order to take the unit's men ashore here aboard, then the named cargo. It needs no base (chapter 7, The load order with a unit). | Cargo |
| Unload | A standing cargo order to land all cargo, men first, then supply, then fuel. | Cargo |
| Set rules of engagement (stance, break-off level, straggler instruction, keep share) | All four figures together, and the order that set them. | Rules of engagement |
| Set home port (a base) | The base a force heads for when it retires or sends stragglers home. | Home port |
| Set readiness (normal or battle) | The readiness and the order that set it. It does not touch whether the force is alerted. | Readiness |
| Detach ships (a list of ships) | Not a standing order. Splits the listed ships into a new force at once. | None |
| Send to yard (`send_to_yard`, no arguments) | Not a standing order. Sends the whole force, stopped at a base that is a port for the yard, away to the yard at once (below, and chapter 4, Repair). The port is the base it is stopped at. | None |
| Prepare a landing (`prepare_landing`, a hex and a roster of land units) | The force's preparation: the coast hex of the landing and the roster, with the loading it keeps and the order that set it. It is applied when taken in, and what a new place, a unit added or dropped and the same order again do is in chapter 7, Preparing a landing. It stands until replaced or until the landing is done, and may be given at sea. | Preparation |

[0008, 0011, 0014, 0016, 0017, 0021, 0046, 0066, 0069, 0092, 0095, 0096]

A route, load, load-fuel or load-with-a-unit order to a force that is out of fuel is refused (Every reason an order is refused, below; differences.md rows 17 and 34).

The stance is engage or avoid. The straggler instruction is keep, detach, leave or retire. Chapter 4, Breaking off, and chapter 2, Stragglers, give what they do. The readiness is read by chapter 3, Readiness, alertness and fatigue of a force. [0014, 0016, 0021]

Detaching ships creates a new force with the next number of its side's series (the Allied side's from 1,000,001, the Japanese side's from 2,000,001). It starts at the parent's exact position and leg (the charge of the leg with it), idle, with no route. It copies the parent's side, mission, speed setting and speed order, home port, readiness and readiness order, alerted flag and the event that alerted it, fatigue, and rules of engagement and rules order. The event that accepts the order is recorded first, then `ShipsDetached`, with the same day and pulse, citing the order (differences.md row 24). [0016, 0037, 0086]

A send to yard order, when it is taken in, is applied after its acceptance: the ships leave the force for the yard, the force is left idle at its base with no route, cargo order, leg or alert, and `ForceLeftForYard` is recorded, citing the order, with the day of return. What the force keeps and what it gets on return are in chapter 4, Repair. An answer to a queued yard order states the day of return, or says the force does not return if that day is after the last day of the game. [0069]

### Air group orders

| Order | What it sets | Standing slot |
| --- | --- | --- |
| Set air mission: rest, search, naval strike, combat air patrol, or base strike on a named base | The mission and the order that set it. | Air mission |
| Set search arc (from bearing, to bearing, range in miles) | The arc and the order that set it. | Search arc |
| Set patrol share (0 to 1) | The share of its flyable aircraft a group on patrol keeps over its own hex. | Patrol share |
| Set rest threshold (0 to 100) | The fatigue above which the staff stands the group down at the start of the day. | Rest threshold |
| Set reach (normal or extended) | How far its strikes and escorts may go: the type's strike radius, or a share more (chapter 5, Radius and reach). Normal is the default. | Reach |
| Set target priority (a list of kinds of ship: carrier, battleship, cruiser, destroyer, transport, auxiliary) | The kinds a group's strikes go for, in order, and the order that set the list. The default is all six kinds in the order of importance. A kind not on the list is never launched at (chapter 5, Naval strike: the contact), and the list ranks the ships a group attacks (chapter 5, The attack on a ship). | Target priority |
| Rebase (`rebase`, an air group and a base) | Not a standing order. Moves a group at a base to another base of its side at once and writes off its damaged aircraft (below, and chapter 5, Rebasing an air group). | None |

[0012, 0026, 0027, 0034, 0036, 0059, 0063, 0091]

### Land unit orders

| Order | What it sets | Standing slot |
| --- | --- | --- |
| Set posture (`set_posture`, a land unit and `attack` or `defend`; an attack order may add `when_spent`, `hold` or `fall_back`) | The unit's posture, at once, the order that set it and its instruction for when its attack is spent (chapter 7, The posture and its order). A tired unit going back to defend spends the order and the instruction. A unit set to attack leaves its fieldworks. | Posture |
| March (`march`, a land unit and a hex) | A march of the unit's men ashore to the hex, a neighbour joined to the unit's hex by a land link, taking `march_days_per_hex` days; the unit's own hex stops a march (chapter 7, Marching overland). A unit sent to another hex leaves its fieldworks. It stands until the unit arrives or another march order replaces it. | March |
| Set activity (`set_activity`, a land unit and `ready`, `train` or `work`, with a `job` for `work`: `airfield`, `port`, `aircraft` or `defences`) | The unit's activity for the days it neither marches nor fights, its job if it works, and the order that set it, at once (chapter 7, The day's activity). A unit set to train gains days of training toward experience (chapter 7, Experience and training); a unit set to work makes work for its job (chapter 7, Units that work and what they build). A working unit takes only `work`. Needs no man ashore. | Activity |

[0090, 0094, 0097, 0099, 0106]

### When an order is checked, and against what

An order is checked once, at the moment it is taken in, against the state as it then stands. For the scenario's script and for orders a side hands in, that is the start of the day, after refuelling. For an order a rule gives inside a day, it is the moment the rule gives it. [0008, 0050]

An order is taken in after the day's returns from the yard and refuelling, so a force that is back from the yard that day is checked as back. [0069]

A side may ask the engine's own check early, with `check_order`. It numbers nothing, records nothing and changes nothing. Its answer is a forecast. The state may change before the day begins, and an order that passed the forecast can be refused at intake. [0050]

The check reads the truth, but a refusal may cite only facts the issuing side can see: its own units, the map, ownership and the reference data. A fact the side cannot see is never a reason. "Not your unit" is the single reason for a target that does not exist and for a target that belongs to the enemy, so a refusal never shows that an enemy unit exists. "Not your base" does the same for bases. "Not your land unit" does the same for land units. The base strike reason "not an enemy base" is worded to cover a base that does not exist, a base the group's own side holds and a base the enemy formed by a landing or a march, which is not on the group's side's map. [0008, 0034, 0050, 0066, 0094]

Limit: 0008 said an order that would fail on something the side does not know is accepted and fails in execution. No such failure exists, because every check today reads only what the side can see. The source comment on the check says orders are checked against the truth. [0008]

### Every reason an order is refused

The reasons are tried in the order written. The first that applies is the one recorded. All other orders are accepted.

For a task force order:

| # | Reason | When |
| --- | --- | --- |
| 1 | Not your unit | The force does not exist, or its side is not the issuer's. |
| 2 | Away at the yard | The force is away at the yard and its day of return is after the day the order is taken in for. A force that is back by that day is checked as back, with the ships it has at the yard, so a side may give orders the evening before the day of return (`check_order` asks for the next day). Every task force order is tried against this, so a force away takes no orders. [0069] |
| 3 | No ship afloat | The force lists no ships (every ship it had is sunk). A force away at the yard is read by the ships it has there. |

Then, by kind:

| Order | Reason | When |
| --- | --- | --- |
| Set route | Out of fuel | The force's status is out of fuel. Tried before the waypoints. [differences.md row 34] |
| Set route | Waypoint off the map | A waypoint is not on the map. The first waypoint is tried first, then each in turn. |
| Set route | Waypoint unreachable | The first waypoint cannot be reached by sea from the force's position (or the hex it would turn back to), or a later one cannot be reached from the one before. A land hex is unreachable. |
| Set route | Empty route | The force does not move anywhere: no waypoint other than where it already is, and no leg in progress to finish. |
| Set route | Mark not on route | A hex in `act_at` is not one of the route's waypoints (the last waypoint counts as one, though it is never treated as marked): "H is marked to act at but is not one of the route's waypoints". Tried after the other reasons above. |
| Load, Load fuel | Under way | The force's status is under way. |
| Load, Load fuel | Out of fuel | The force's status is out of fuel. [differences.md row 17] |
| Load, Load fuel | No own base here | No base of the issuer's side is in the force's position. |
| Load with a unit | Under way | The force's status is under way. |
| Load with a unit | Out of fuel | The force's status is out of fuel. |
| Load with a unit | Not your land unit | The unit does not exist or is not the issuer's. One reason for both. |
| Load with a unit | No men ashore here | The unit has no men ashore, or they are not in the force's position. |
| Load with a unit | No troop capacity | No ship afloat in the force is of a class with a troop capacity above 0. No base is needed. [0066 item 6] |
| Set rules of engagement | Break-off level out of range | The level is below 0 or above 1. |
| Set rules of engagement | Keep share out of range | The share is below 0 or above 1. Tried after the break-off level. |
| Set home port | Not your base | The base does not exist or is not the issuer's. |
| Detach ships | No ships to detach | The list is empty. |
| Detach ships | Ship not in task force | A listed ship is not afloat in the force. The first such ship in list order is named. |
| Detach ships | Would leave the task force empty | The listed ships, counted once each, are every ship of the force. |
| Send to yard | Under way | The force's status is under way, or it has a hex ahead. Out of fuel at a hex centre counts as stopped. |
| Send to yard | No own base here | No base of the issuer's side is in the force's position. |
| Send to yard | Not a port for the yard | A base of the side is in the force's hex but none has a `yard_passage_days`. |
| Prepare a landing | Not your land unit | A unit on the roster does not exist or is not the issuer's. One reason for both. The first such unit in roster order is the one that counts. |
| Prepare a landing | Not a coast hex | The hex is off the map or is not a coast hex: "H is not a coast hex on the map", with the hex. The map is common knowledge. Tried after the roster's owners. |
| Prepare a landing | Land unit destroyed | A unit on the roster has no men ashore and none aboard any ship: "the land unit is destroyed". Tried after the hex. |
| Set speed in knots | Speed not above zero | The speed is zero or below. |
| Set speed (cruise, full), Set mission, Set readiness, Unload | None | Always accepted. |

For an air group order:

| # | Reason | When |
| --- | --- | --- |
| 1 | Not your unit | The group does not exist, or its side is not the issuer's. |
| 2 | Air group lost with its base | The group's place is lost: it was at a base when the base changed side (chapter 5, A group lost with its base). "the air group was lost with its base". [0090] |
| 3 | Air group lost | The group is aboard a ship that is sunk or does not exist. "the air group was lost with its ship". |

Then, by kind:

| Order | Reason | When |
| --- | --- | --- |
| Naval strike | No air combat figures | The scenario has none. |
| Naval strike | Aircraft attack with nothing | The group's aircraft type has no attack weapon. |
| Base strike | No air combat figures; aircraft attack with nothing | As for a naval strike, tried in that order. |
| Base strike | Not an enemy base | The base does not exist, is the group's own side's, or is one the enemy formed by a landing or a march and is not on the group's side's map (chapter 7). Tried after the two above. |
| Combat air patrol | No air combat figures | The scenario has none. |
| Set target priority | Target list empty | The list names no kind. |
| Set target priority | Target kind repeated | The list names a kind twice. The first kind named twice, in list order, is named. Tried after the empty list. |
| Combat air patrol | Not a fighter | The group's aircraft type is not a fighter. |
| Rest, Search | None | Always accepted. |
| Set patrol share | Patrol share out of range | Below 0 or above 1. |
| Set rest threshold | Fatigue out of range | Below 0 or above 100. |
| Set reach | No air combat figures | The order is extended and the scenario has none. Normal reach is always accepted, so a group can go back to it. |
| Set search arc | Invalid bearing | A bearing is not a multiple of 10 from 0 to 350. The from bearing is tried first. |
| Set search arc | Invalid search range | The range is zero or below, or above the aircraft type's search range (`aircraft_types` file: `search_range_nm`). |
| Rebase | Waits for its base | The group has no aircraft and is at a base that is not its side's or has no airfield (chapter 5, A group with no aircraft at a base that is not its side's): "the air group has no aircraft and waits for its base". Tried after the group is found not to be lost and not aboard a ship, and before "already moved today". The group is the issuer's own, so the reason shows nothing the side cannot see. [0101] |
| Rebase | Away at the yard | The group is aboard a ship that is away at the yard: "the air group is aboard a ship that is away at a yard". Tried before the next, so a group aboard a ship at the yard is told this. [0091] |
| Rebase | Aboard a ship | The group is aboard any other ship: "the air group is aboard a ship". |
| Rebase | Already moved today | The group has already moved to another base that day, by an earlier order of the day: "the air group has already moved to another base today". |
| Rebase | Not your base | The base does not exist or is not the issuer's. One reason for both, as for a home port. A base a landing formed (chapter 7) is the side's own and may be named. |
| Rebase | Already at the base | The group is at the base named: "the air group is already at that base". |
| Rebase | No airfield | The base has no airfield (chapter 7, Units that work and what they build): "the base has no airfield". Tried after the check that the group is not already there and before the range. A base is shown to its own side with its airfield, so the reason shows nothing the side cannot see. |
| Rebase | Airfield too damaged | The base's airfield has damage at or above `ground.toml: airfield_landing_refused_at` and so takes no aircraft (chapter 5, A damaged airfield): "the airfield's damage is D, and a field at L or more takes no aircraft", with the damage and the level. Tried after "no airfield" and before the transfer range. Without the ground figures no field is refused for its damage. The base is the side's own, so the reason shows nothing the side cannot see. [0103] |
| Rebase | Beyond the transfer range | The air distance between the two bases' hexes is above the type's transfer range: "the base is D nm away, beyond the air group's transfer range of R nm", with the distance and the range (chapter 5, Rebasing an air group). Both are figures the side knows. |

For a land unit order:

| Order | Reason | When |
| --- | --- | --- |
| Set posture | Not your land unit | The unit does not exist or is not the issuer's. One reason for both. |
| Set posture | Land unit destroyed | The unit has no men ashore and none aboard any ship: "the land unit is destroyed". |
| Set posture | A working unit | The order is attack and the unit is a working unit: "a working unit does not attack: it is always on defend". Tried before the next. |
| Set posture | An instruction on a defend order | The order is defend and carries `when_spent`: "only an attack order says what the unit does when its attack is spent". Tried before the next. |
| Set posture | No men ashore | The unit has no man ashore: "the land unit has no men ashore". |
| March | Not your land unit | The unit does not exist or is not the issuer's. One reason for both. |
| March | Land unit destroyed | The unit has no men ashore and none aboard any ship: "the land unit is destroyed". |
| March | No men ashore | The unit has no man ashore: "the land unit has no men ashore". |
| March | No land link | No land link joins the unit's hex to the hex named, and the hex named is not the unit's own: "no land link joins the land unit's hex to H: men can march only to a neighbouring hex the map links by land", with the hex. The map is common knowledge, so the reason shows nothing the side cannot see (chapter 2, Land links). |
| Set activity | Not your land unit | The unit does not exist or is not the issuer's. One reason for both. |
| Set activity | Land unit destroyed | The unit has no men ashore and none aboard any ship: "the land unit is destroyed". A unit with men aboard only is accepted. |
| Set activity | A working unit works | The unit is a working unit and the activity is `ready` or `train`: "a working unit works: it does not stand ready or train". `work` is accepted, with or without a job. |

[0008, 0016, 0017, 0021, 0026, 0027, 0034, 0036, 0044, 0059, 0066, 0090, 0091, 0094, 0095, 0096, 0097, 0099]

A refused order changes nothing but the log and the order counter. It is kept in the order record. [0008]

The air group's aircraft type and the figures it names are checked when the game is built, not when an order is taken in. [0025]

A side sees: it sees its own acceptances and refusals whole, with the reason. The enemy sees nothing of an order. In what a player is shown, "not your unit" reads as "not known". [0047, 0050]

### What an accepted order does

An accepted order replaces the standing order of its slot and nothing else. A new route does not cancel a speed order. A new mission does not cancel a search arc. Cargo orders share one slot: Load, Load fuel, Load with a land unit and Unload replace each other, and a Load or Load fuel clears the unit a Load with a unit named. The four figures of the rules of engagement are replaced together. A posture order is applied when it is taken in: the unit's posture is set and the order is kept as the one that set it, and a tired unit going back to defend spends it (chapter 7, The tired stop attacking). A march order is applied when it is taken in too: it starts a march with 0 days marched, or keeps the days marched if the hex is the one the unit already marches to, or, if it names the unit's own hex, stops a march and records `MarchStopped` (chapter 7, The march order). It is spent when the unit arrives or the march stops. A prepare-landing order is applied when it is taken in too: it sets the force's preparation, or changes it as chapter 7, Preparing a landing, says, and changes nothing if it names the same hex and the same roster as the one standing. It stays until another replaces it or the landing is done, when its unloading is complete. An activity order is applied when it is taken in: the unit's activity is set and the order is kept as the one that set it, and it stays until another replaces it. [0008, 0027, 0036, 0046, 0090, 0094, 0095, 0099]

A standing order stays until another of its slot replaces it. Nothing clears a cargo, speed, mission, readiness, rules, home port, patrol share, rest threshold, reach or arc order when it has been carried out. Two rules do drop the route order of a force: a force that ran dry and takes fuel in port, and a force retiring because of its stragglers. Chapter 2, Refuelling in port and Stragglers, says when. [0008, 0016, 0045]

A task force remembers which order set its route, speed, rules of engagement and readiness. An air group remembers which order set its mission, arc, patrol share, rest threshold and reach. Events cite them as causes. [0008, 0059]

A route order that finds a force between two hexes may turn it round. The event `TaskForceTurnedBack` follows the order's acceptance and cites the order. [0005, 0017]

A side sees: its own turn-back and detach events whole, with the cause. The enemy sees nothing. [0047]

A force halted at a marked waypoint records `TaskForceHalted`, and `TaskForceSteamedOn` when it goes on (chapter 2, Halting at a marked waypoint and steaming on). Each is recorded in the movement step of a pulse, citing the route order, the halt, and the bombardment or cargo completion that ended the wait. A side sees its own two events whole, and the enemy sees neither. The side's briefing, `list_forces` and `get_entity` show its own marks (`act_at`, the marks of the route not yet passed), its force's halt (where, since when, what it waits for, and whether it has bombarded), and, in the briefing, yesterday's halts and steamings on with the reason. Nothing of a halt or a mark reaches the enemy. [0096 item 7]

A side sees the same of the yard events: its own `ForceLeftForYard`, with the order as cause, whole. [0069]

A rebase order, when it is taken in, is applied after its acceptance: `AircraftWrittenOff` is recorded if the group had damaged aircraft, then the group's place becomes the new base and `AirGroupRebased` is recorded. Both cite the order. Chapter 5, Rebasing an air group, gives what the move does and what follows the group. A side sees its own `AircraftWrittenOff` and `AirGroupRebased`, whole, and the enemy sees nothing of them. [0091]

### How orders are numbered and sorted

Orders are submitted per side and wait for the next day. At the start of the next day, intake takes the sides in a fixed order, Allied then Japanese. [0008]

Each side's orders are put in order of the unit they are for before they are numbered: task forces, then air groups, then land units, each by unit number. That order is the order of the three kinds of unit in the code (`EntityRef`), and nothing in a rule depends on it. The sort is stable, so one unit's orders keep their order and a later order to the same unit replaces an earlier one. The sort is made where orders are taken in, so a script, a player and a staff are treated alike. [0043]

Each order takes the next order number, in one sequence for both sides in one game, starting at 1. The Allied side's orders on a day are numbered before the Japanese side's. An order a rule gives inside a day takes the next number at the moment it is given. The number appears in the event that accepts or refuses the order and in the causes of what follows. No roll is keyed on an order's number. [0008, 0043, 0055]

A player is shown a side's own count of its orders, 1, 2, 3, in the order they were taken in, and never the log's number. [0055] Outside the simulation core an order's number cannot be read at all: the side's view gives the side's count, and only the umpire's tools read the log's number. [0086]

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

The truth state is the real world: ships, task forces, bases, air groups, land units, arrivals, replacements and the day. A belief state holds one side's contacts, claims and estimates of the enemy's men ashore. A side's belief is written only from reports (see chapter 3, Belief): a sighting report makes or refreshes a contact, a battle claim is stored, a "nothing there" report drops a contact, and a ground report makes or replaces an estimate. Ageing only removes what reports once said and adds nothing. A contact, a claim or an estimate never holds the true number of an enemy unit or of its men. [0007, 0012, 0028, 0092]

Anything that plays the game is handed a side's view and never the truth. The truth is read only by the runner, the tests, the replay viewer and the hash. [0007, 0053]

### What a side may know

| What | How it is known |
| --- | --- |
| Its own ships, task forces, bases and air groups | Exactly. There is no fog over a side's own units. Supply and fuel stocks of its own bases and ships are shown. A base's fuel is for ships: no air rule reads it. A base's airfield, the work done on it, the damage to it, its defences with their grade and the day's work are shown to its own side, and so are the share of its aircraft a damaged field lets fly and the aircraft it keeps on the ground (chapter 5, A damaged airfield; chapter 7, Units that work and what they build). |
| Its own replacement streams and arrivals | Exactly. |
| The enemy's forces | Only as contacts: a hex, a count, a kind and an age, each made and refreshed by reports, each possibly wrong. |
| What it believes came of its fights and raids | Its claims. The damage it does to the enemy is truth and is not shown to it. The damage done to its own ships and aircraft is shown to it whole. |
| Its own land units | Exactly: the men ashore and where, the men aboard and in which ships (chapter 7), each unit's skills (chapter 7, What a land unit is good at), and whether it is a working unit, its job and its work (chapter 7, Units that work and what they build), and its fieldworks and cover (chapter 7, Cover and fieldworks). |
| The enemy's land units | Only an estimate of the enemy's men in a hex, from a ground report or a scenario's opening estimate, which may be wrong, and the day it was reported. A fight ashore tells a side that the enemy has men in the hex (chapter 7, The fight ashore, and What a side knows of the enemy ashore). |
| The enemy's bases | Number, name, hex and the holding side, and nothing else, of every base the scenario starts with: no word on whether it has an airfield. This is the map, not intelligence. A base formed in play by a landing or a march, or a base a scenario holds off the map, is on its own side's map only: it is not among the enemy's bases, an order against it is refused as for a base that does not exist, it cannot be bombarded, and the enemy finds it by a ground report or when it is taken (chapter 3, The ground report). A base that changes side (chapter 7) is on both sides' maps from then on, with the side that holds it now, whatever it was before. |
| The map | In full. |
| Reference data (ship classes, aircraft types, figures) | In full. |
| Its own orders | All of them, accepted or refused, with the causes it may see. |
| Its own mission and objectives | Its own only (chapter 6, What a side knows of its progress). |
| What happened | The events it may see, cut as the next section says. |

A side learns of what its forces did when a day is over. [0007, 0034, 0047, 0050, 0055]

Limit: within a fight a side cannot tell one enemy ship or air group from another. It knows that its ship fired and that its ship was fired on, not at which ship or by which. [0065]

A side's own ships, task forces, air groups and land units are shown to it numbered 1, 2, 3 for each kind, in the order of their numbers in the engine, counting every unit it has had (a destroyed land unit too, so a number never changes). Every answer it is given and every handle it writes uses these numbers; the engine keeps the scenario's numbers. Bases keep theirs: they are on the map for both sides, and a base formed by a landing or a march keeps its number too. [0086, 0066]

Units made in play are numbered by side. A task force made by a detach and an air group formed by a landing away from home take the next number of their own side's series, one counter for each side and kind, kept in the state: the Allied side's from 1,000,001 and the Japanese side's from 2,000,001. A scenario's own numbers are below 1,000,000, and the loader refuses any at or above it. A made unit's number, and so the draws keyed on it, depends on its own side's history only: an enemy unit that is merely there changes nothing. A land unit made by division and a base formed by a landing or a march are not in these series: each takes the next free number counted over the units or the bases of both sides (the highest plus one), as chapter 7 says. [0086, 0066]

Limit: there are no claims for aircraft shot down in an air fight except the raid claim of decision 0056. A side sees its own air fight losses and nothing of the enemy's. [0047, 0056]

### What a commander decides and what is decided at the point of contact

Inside a day the rules make choices for a side's units: how many aircraft fly, whether a strike launches, where a straggler goes. Each choice the rules make for a side's own unit from what the side knows (its own units, its standing orders, its belief and the figures) is one function of that and nothing else, and the engine makes the choice by that function. A side can ask any of them for its own unit before it is made, through its view, and the answer is what the engine would choose at that moment with what the side holds. It is not a forecast: it says nothing of what the enemy will do or what the dice will give, and something that changes inside the day (a ship sunk that morning, a contact reported at noon) can change the choice. Asking changes nothing. The agent asks them with `measure`'s `choices` question. [0100]

| Choice | Question | Chapter |
| --- | --- | --- |
| The contact a naval strike group would strike | `strike` | 5, Naval strike: the contact |
| How many aircraft of a group can fly | `aircraft_able_to_fly` | 5, The order of the limits |
| Whether a base strike launches in a pulse | `base_strike` | 5, Base strike |
| How a fighter group splits into patrol and escort | `fighter_split` | 5, The split |
| Which raid the escorts go with, or whether they stay | `escort` | 5, Which raid the escorts join, and the reach |
| Where aircraft land when their deck is gone | `landing` | 5, Coming home to no deck |
| Whether a tired group stands down or returns | `rest` | 5, Stand-down and return |
| Whether a force is alerted by a contact | `alert` | 3, Alert from belief |
| Whether a force is ready when a fight begins | `ready` | 4, Ready, and the shares of the guns and the torpedoes |
| Whether a force breaks off | `break_off` | 4, The ending checks |
| What a force does with its stragglers | `stragglers` | 2, Stragglers |
| Where a straggler or a retiring force goes | `head_home` | 2, Stragglers |
| Whether a force turns back or goes on when its route changes | `reroute` | 2, Turning back and re-routing |
| Whether a force halts at a marked waypoint, and when it steams on | `halt`, `steam_on` | 2, Halting at a marked waypoint and steaming on |
| Whether a land unit goes back to defend | `back_to_defend` | 7, The tired stop attacking |
| What a spent attacker does: hold or fall back | `attack_spent` | 7, What a spent attacker does |
| What a land unit's day is | `unit_day` | 7, One activity a day |

A choice that needs a fact only the step that makes it has, such as the ships a fight began with, the fatigue after the day's change, or whether a unit marched or its hex is contested, takes it as part of the question: the side states it, and a side cannot know for certain whether its hex is contested. [0100]

Five choices are made at the point of contact, by the men who see what is there, and read what is truly there: which enemy force a raid attacks when it arrives and whether it finds anything; which ship each strike group attacks; how a patrol divides between the raids over it and between escorts and bombers; whether a force that avoids slips away; and who fires at whom in a surface fight. They stay with the rules. A side cannot ask them. [0100]

Which base feeds, refuels or repairs is not a choice: it is a fixed rule of allocation (the lowest-numbered base of the side in the hex, in order), and a side reads the rule. [0100]

The figures these choices use are reference data, the same for both sides, and a side's view hands them over in full: the surface combat, air combat, air wear and fatigue figures among them. [0100]

## What a side sees of an event

### The rule

The event log is the truth. It holds both sides' orders, rolls, hits and losses. A side is given its own version, and each event is, for that side, whole, cut down, or absent. Nothing is added or altered. A cut event is a variant with no field for what is removed, so nothing removed can be carried in it. A cut event keeps its number in the full log, which only the umpire can read. A side numbers the events it is given 1, 2, 3, in the order it is given them, and their causes are given by those numbers and by its own count of its orders. Event, order and contact numbers cannot be read, printed or made outside the simulation core; the umpire's tools read them with the umpire's key, which only the world gives. [0047, 0086]

The rule is one exhaustive `match` over every kind of event in `side_events.rs`. A new kind of event does not compile until it is placed in the table. [0047]

No event a side may see names an enemy ship, task force or air group. The side's form of the event has no field for it and its text has no word for it. Where the log names two forces of a fight, the side's event holds its own. The enemy reaches a side only as contacts and claims, which hold no identity either, so a side cannot tell that two sightings, two raids or two fights were the same enemy. Two things are still named: an enemy base, which is on the map for both sides, and the target a side itself wrote in an order, in that order's acceptance or rejection. [0065, 0012]

### The table

"Own" means the unit named is the side's own. Every kind not named is shown to no side.

**Whole if own**

| Event | Whole to the side when |
| --- | --- |
| Order accepted, order rejected | The side issued the order. (The order decides. Where the order is unknown, the target's side decides.) |
| Task force steamed, arrived, out of fuel, turned back, fatigue | Own force |
| Task force halted, task force steamed on | Own force. The enemy sees neither (chapter 2, Halting at a marked waypoint and steaming on) |
| Loading complete, unit loading complete, unloading complete | Own force |
| Force retired, force stood down | Own force |
| Force alerted by contact, force became alert | Own force |
| Force broke off | Own force |
| Ships detached | The parent force is own |
| Loading gained, loading lost, preparation shared | Own force. The enemy sees nothing of a force's preparation |
| Rehearsal gained, rehearsal lost, left roster | Own land unit. The enemy sees nothing |
| Training day, fight day counted, experience raised | Own land unit. The enemy sees nothing of a unit's experience |
| Unit worked | Own land unit. The enemy sees nothing of a unit's work or its job |
| Airfield built | Own side's base. The enemy sees nothing |
| Airfield damaged | The side that holds the base. The raid or the shelling it cites is cut as for supply destroyed. The raider, and the enemy, see nothing of it |
| Airfield mended | Own land unit. The enemy sees nothing |
| Force left for the yard, force returned from the yard | Own force |
| Fuel burned, ship refuelled, ship repaired | Own ship |
| Cargo loaded, cargo unloaded | Own ship |
| Ship damaged, system hit, ship sunk | Own ship |
| Search flown | Own air group |
| Aircraft worn, aircraft ditched, aircraft ditched for want of a deck, aircraft landed elsewhere, aircraft repaired, aircraft arrived, arrival waiting, replacements received | Own air group |
| Damaged aircraft destroyed on the ground, aircraft destroyed on the ground | Own air group |
| Air group lost, air group fatigue, air group stood down, air group returned | Own air group |
| Strike launched, base strike launched, strike found nothing, escorts sent | Own air group |
| Aircraft lost, pilots lost | Own air group |
| Air group rebased, aircraft written off | Own air group |
| Air group lost with a base, strike mission cleared | Own air group |
| Ship lost with a base | Own ship |
| Home port lost | Own force |
| Base supply destroyed, base supply spent (aircraft, men or a march), base at day end, base formed | Own base |
| Ground report received, base found | The side that was given the report |
| Men embarked, men landed, men lost with a ship, men lost unfed, men lost to fire, land unit destroyed | Own land unit |
| Fed share restored, land unit fatigue (gained, rested, marched, marched and attacked, strained, landed), land unit back to defend, men lost in a fight, unit fell back, spent unit fell back, unit surrendered | Own land unit |
| Fieldworks changed (dug, worn, lost) | Own land unit, whole. The enemy sees nothing: a unit's fieldworks are not in anything it is told (decision 0106) |
| Defences worn | The side that holds the base, whole: the base, the hex, the hits and the defences after. The side that fired, and the enemy, see nothing of it (decision 0106) |
| March day, march arrived, march stopped | Own land unit. The enemy sees nothing of a march. A march day carries the days counted (decision 0098) |
| Base formed by a march | Own base. The enemy sees nothing |
| Unit divided | The unit it came from is own |
| Men fed | The side whose men were fed |
| Report received, claim received, contact aged, contact dropped, contact not there | The side that holds the contact or made the report |

**Cut of the enemy's name and nothing else** [0065]

Each keeps every other field and its whole breakdown.

| Event | The side sees |
| --- | --- |
| Engagement begun, engagement ended | A side that has come to know of the fight (below): its own force of the two, and the rest. Not the enemy's force. The beginning carries the hour of the round the side first knew of the fight, and the ending's reason is the side's own (below). [0085] |
| Sighting roll | Own force, and only if the roll sighted: the round, the force, the chance and the fatigue factor. Not the force sighted. A roll that did not sight is hidden from both sides: it would say there was something to be found. |
| Force surprised | The force surprised and the round. Not the force that surprised it. |
| Force slipped away | The force that slipped away and the round. Not the force it slipped away from. |
| Alarm roll | Own force, and only if the roll succeeded (alarmed): the force and the chance. Not the forces that fought. A failed alarm means the force noticed nothing. Its breakdown is shown without the draw (below). [0085] |
| Base attacked, for the base's side | The base, the shots, the chance, the expected hits and the hits, on aircraft, on supply and on men. Not the air group that raided. A hit on the airfield is told in `AirfieldDamaged` and appears here as a hit on nothing. |

**Cut**

| Event | The firer sees | The target sees |
| --- | --- | --- |
| Salvo | Its own salvo: the round, firer, weapon and shots. Not the target, the chance, the expected hits or the hits scored. | That it was fired on: the round, the target and the weapon. Not the firing ship or the shots. The hits it took reach it as ship damaged. |
| Air attack on a ship | Its own attack: group, weapon and shots. Not the ship or its force. | That its ship was attacked: the task force, the ship and the shots (one shot for each aircraft that attacked). Not the air group, the chance or the hits. |
| Air fight | Its own fighters firing: firer, what fired and at what (patrol, escorts or bombers), and shots. Not the group fired at. | Nothing. |
| Anti-aircraft fire (a force's) | Its own force's fire: force and shots. Not the raid's group. | Nothing. |
| Base anti-aircraft fire | Its own base's fire: base and shots. Not the raid's group. | Nothing. |
| Base attacked | The attacker sees that it attacked: group, base and shots. Not what was hit. | The base's side: see the table above. |
| Search attempt | Only if it sighted, its own search: the air group. Not the force sighted, and not the distance or the chance, which are worked from where the force really was, while the report may name a neighbouring hex. An attempt that did not sight is hidden from everyone. [0061, 0065] | Nothing. |

**Cut by the fight ashore** [0090]

| Event | The side sees |
| --- | --- |
| Ground fight | A side that has men in the hex: the hex. Not which side attacked. |
| Ground fire | Its own unit's fire: the unit, the hex, the firing men and the shots (after the limit on engaged men). Not the chance or the hits. The enemy sees nothing of it. |
| Ground attack failed | The side that attacked and failed: the hex. Not either weight of fire. The enemy sees nothing of it (chapter 7, An attack that fails) |
| Men surrendered ashore | Both sides: whole to the side whose men they are. To the enemy, cut to the hex and the men, once for the hex and with no unit, so that it cannot count how many units surrendered (chapter 7, A beaten side's men) |
| Base changed side | Both sides: the base, the hex and the two sides. The side that now holds the base is also given the supply and fuel that stayed and those that were lost. The events of a base are seen by the side that held it when they were recorded (chapter 7, A base that changes side). |

**Absent for both sides**

| Event |
| --- |
| Forces passed unseen (a pass in the dark) |
| Side beaten ashore (the test weighs both sides' men) |

**Cut by shore bombardment** [0092]

| Event | The side sees |
| --- | --- |
| Bombardment fire (one ship's) | The side whose ship it is: the ship, the base, its shells and weighed shots and the rounds it has left. Not the chance, the expected hits or the hits. The base's side sees nothing of it, because it would say how many ships fired. |
| Shore bombardment (a force's) | The side whose force it is: the force, the base, the hex, and the shells and weighed shots. Not the hits. The base's side is given the force's event cut of the force: the base, the hex, the shells, the weighed shots, the hits and where they landed. |

A side that is neither named sees nothing of the cut events. [0047]

### A surface fight

A side knows of a surface fight from the earliest round in which one of these happens, found from the fight's own events between its `EngagementBegun` and `EngagementEnded`: its force's successful sighting roll, a salvo fired at its ships or by them, hits on its ships, its force being surprised, or its force slipping away. A fight the side never knows of is wholly absent for it, including its begun and ended events. For a fight it knows, those two events are shown at their place in the log, cut as the table says. [0047]

A fight is told from the round the side first knew of it. That round is the side's round 1, and every event of the fight it is given that carries a round (its salvoes, the salvoes at it, its sighting, surprise, slip, alert and break-off, the hits on its ships and the end) carries the side's count: the log's round less the rounds before it knew. The beginning carries the hour of that round: the hour the forces met plus `round_minutes` for each round before. A side that knew from round 1 is told the log's rounds and hour. The event's number is the log's, so two sides can still name one event. This is the one change made to an event otherwise given whole. [0085]

The ending is told as the side's own force made it. "A force was sunk" is told only to the side whose force lost its last ship in the fight (its force has no ship, and its last sinking was in this fight); "a force broke off" only to the side whose force broke off in the last round. Otherwise the side is told **contact lost**. Slipped away, passed unseen and the last round fought are told as they are. What the side learns of the enemy's losses is its claim. [0085]

An event of a fight that carries a round is shown only if its round is not before the round the side first knew. Round 0 hits are outside a fight. A force that breaks off in a round in which it has neither sighted nor been fired on, because its ships were sunk or heavily damaged before the meeting, is not shown the break-off. The force's state still changes. [0014, 0047]

### Causes and breakdowns

The causes of an event are cut the same way. A cause that is an order is kept if the side issued it. A cause that is an event is kept if the side may see that event. A dropped cause is not marked on the event. Instead every event of a kind that can have causes the side cannot see carries a flag saying it may have more, whether or not this one does, so the flag says nothing of what the enemy did. The kinds flagged are: engagement begun and ended, force surprised, an alarm roll, an air attack on the side, a raid on its base, its fighters' and guns' fire at a raid (the enemy's launch is the cause), aircraft lost, a fight ashore (it cites both sides' posture orders), men lost in a fight (they cite the enemy's fire), a base changed side, a unit that fell back and the men surrendered in a hex (each cites the test that beat the side), an attack that failed (it cites the enemy's fire beside the side's own), and the shore bombardment of the side's base (it cites the enemy ships' fire). A ground report cites the day's fight, which both sides are given, and a base found cites the side's own report: neither is flagged. One exhaustive `match` in `side_events.rs` decides each kind. [0047, 0085]

A side sees the whole breakdown, with its result, of an event it sees whole, and of an event cut of the enemy's name and nothing else. Any other cut event carries only the factors that are the side's own, and no result. A salvo keeps `guns`, `tubes`, `rate`, `minutes`, `main_battery_factor`, `unready_gun_share`, `torpedo_mount_factor`, `unready_torpedo_share` and `fire_control_factor`. An air attack and a base attack keep `aircraft`. An air fight and anti-aircraft fire keep `shots`. A search attempt keeps `aircraft`, `sweep_width_nm` and `arc_deg`. A ground fire keeps `men_ashore`, `fed_share`, `fatigue_factor`, `firing_men`, `men_per_shot` and `shots` (and, where the limit on engaged men cuts the side's own fire, `shots_before_limit`, `side_firing_men` and `engaged_men`), and a ground fight and a base changing side keep none. A bombardment fire keeps `rate`, `guns`, `minutes`, `main_battery_factor`, `rounds_aboard`, `shells`, `gun_damage`, `reference_gun_damage` and `weighed_shots`, and a force's shore bombardment has no breakdown. A ground report is the exception, as a claim is: a side sees it whole and no breakdown of it, because the breakdown holds the true count and the error drawn. A salvo against the side and an air attack against it keep no factor. A side's breakdown never carries a raw random draw: the alarm roll's `draw` is dropped, and the log keeps it for the umpire. [0047, 0061, 0068, 0071, 0085, 0090, 0092]

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
| `air_combat` | `air_combat.toml` | Strike and base strike missions, combat air patrol, escorts, raids, anti-aircraft fire, the false claims of a raid, reach beyond the radius with running out of fuel, the pilots of a patrol shot down over its own hex, and, with `surface_combat.toml`'s bombardment figures, shore bombardment (its table says where a hit lands). Without it the mission orders and the extended reach order are refused. A file that names the scenario must hold every key but `[base_hit] men` and `[base_hit] airfield`, which give 0 if missing: any other missing key is refused when the scenario loads. |
| `air_upkeep` | `air_upkeep.toml` | Wear and repair of aircraft. |
| `fatigue` | `fatigue.toml` | Fatigue of task forces and of air groups, stand-down and return, and the fatigue factor of a sighting. |
| `air_supply` | `air_supply.toml` | Supply for sorties and the daily supply, bases out of supply, and the supply a repair costs. |
| `ground` | `ground.toml` | Men worked as cargo, the men's supply and the loss of the unfed, the fight ashore, fatigue and a side beaten, a base that changes side (chapter 7), the men a hit on a base removes and the ground report (`men_lost_per_base_hit`, `report_error_low` and `report_error_high`; chapter 3, The ground report), marching overland (`march_days_per_hex`, `fatigue_per_march_day` and `march_supply_days`), preparing a landing (`preparation_per_day`, `unprepared_landing_share`, `preparation_kept_on_change` and `unprepared_landing_fatigue`), the experience of a land unit (`experience_fire_strength`, `training_days_per_level`, `training_ceiling` and `fight_days_per_level`) and the skills of a land unit (`assault_skill_strength`, `movement_skill_strength`, `holding_skill_strength` and `weight_strength`; decision 0098). The damage a hit does to an airfield, the work to mend it and the damage at which a field takes no aircraft (`airfield_damage_per_hit`, `work_per_airfield_damage_point` and `airfield_landing_refused_at`) are read from it, as is the fatigue a unit gains when its side is beaten and it falls back (`beaten_fatigue`; chapter 7, A beaten side's men), and a hit lands on an airfield only where it is loaded (chapter 5, The attack on a base). A scenario with a land unit must name it. |

A scenario with air groups must also name search figures, and a scenario with surface combat figures must also name them. [0021, 0026, 0035, 0036, 0037, 0039, 0066, 0090]

The five repair figures of decision 0069 are keys of `surface_combat.toml`, so they are always loaded and every one is required: `forward_repair_max_flotation`, `forward_repair_system_points`, `yard_days_per_flotation_point`, `yard_days_per_system_degraded` and `yard_days_per_system_out`. Chapter 4, Repair, gives what each means and what the loader refuses. The frozen step 1 copy holds the same values. The rules do nothing until a base has `ship_repair_points_per_day` or `yard_passage_days` (chapter 2, Bases on hexes). [0069]

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

The values of `Draw` are `Legacy` (0), `Claim` (1, the false claims of a raid, one stream for each side's claim keyed on its first firing unit and the raid's first strike group) and `Ditch` (2, the aircraft that run out of fuel and the recovery of their crews, one stream for each air group and pulse, in the upkeep subsystem, keyed on the air group alone) `NoDeck` (4, the crews of aircraft that came down in the sea for want of a deck, one stream for each source air group and pulse, in the upkeep subsystem, keyed on the air group alone) and `PatrolPilot` (3, the pilots of a patrol's aircraft shot down over its own hex, one stream for each firing roll's losses to a patrol group, in the air combat subsystem, keyed on the patrol group and the firing escort group). `TorpedoOtherSide` (5) is the second side's torpedo salvo of a round, one stream for that salvo, in the surface combat subsystem, keyed on the firing ship and the enemy force (chapter 4, Random draws). `GroundFire` (6) is the shots of one land unit in the day's fight ashore, one stream for each unit, in the ground combat subsystem, with no pulse, keyed on the unit alone. `Bombardment` (7) is a ship's shells in a night's shore bombardment, where each hit lands and, on the first firing ship's stream only, the sighting report the bombardment gives: one stream for each firing ship and night, in the surface combat subsystem, keyed on the ship and the base. `GroundReport` (8) is the error of one ground report: one stream for each reporting side, hex and day, in the intelligence subsystem, with no pulse, keyed on the reporting side's lowest-numbered unit in the hex. The claim of a bombardment is a `Claim` (1) stream keyed on the first firing ship and the base, in the intelligence subsystem. [0055, 0056, 0059, 0083, 0090, 0092]

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

At the end of every day, after the third pulse and the end-of-day phases, the engine checks the state. They are also checked when a game is built. A violation is always a bug in the rules, never a legal state. If any is broken, `resolve_day` returns an error naming the day and every violation, and no hash is recorded for the day. The invariants are checked before the day counter advances, so the counter still names the day before; the events are kept (differences.md row 25). [0004]

The checks:

| Group | What must hold |
| --- | --- |
| Ships | Fuel, supply aboard and cargo fuel are not negative. Fuel does not exceed the class's capacity. Supply and cargo fuel together do not exceed the class's cargo capacity. Flotation and fire damage are from 0 to 100. Gun rounds are not negative. A ship at 100 flotation damage is sunk. A ship has no damaged system its class lacks. A ship's bank of forward repair points is from 0 to below `forward_repair_system_points`. A ship's men aboard are at least 1 and not above its class's troop capacity, are of a unit that exists and is of the ship's side, and a sunk ship has none. |
| Where a ship is | A ship in a task force is listed by that force. A ship in a port is at a base that exists. A force lists only ships that are in it, that exist, that are not sunk and that are of its side. No ship is listed by more than one force. A ship away at the yard is in no force's list and its location names a force that is away; a force that is away lists no ship. |
| Task forces | The force's hex is on the map and navigable. A hex ahead is set if and only if progress toward it is above 0; it is adjacent, on the map, navigable, and the step to it crosses no closed side. Progress is at least 0 and below the leg's charge, `leg_nm`; `leg_nm` is from 1 to 40 miles and is 40 for a force at a centre. The keep share is from 0 to 1. Fatigue is from 0 to 100. A home port exists and is of the force's side. A force is halted exactly when it holds a halt, and a halted force is at the centre of its hex, on the waypoint its route's next index names, and that waypoint is not the route's last. [0096] |
| Task force preparation | A force's loading is from 0 to 100 and every unit on its roster exists and is of the force's side. A land unit is on the roster of one force at most. [0095, 0099] |
| Bases | The hex is on the map. Supply and fuel stocks are not negative. A replacement stream names a base that exists. A base's repair points a day are not negative and its yard passage is not 0. The work done on a base's airfield is not negative and its defences are from 0 to 100 (fire wears them, never below 0). The damage to a base's airfield is from 0 to 100 and is above 0 only where the base has an airfield. [0099, 0103] |
| Land units | A unit's hex is set if and only if it has men ashore, and is on the map. Fatigue is from 0 to 100 and the fed share from 0 to 1. [0090] A unit with men ashore that has a march marches to a hex a land link joins to its hex, and has marched fewer days than `march_days_per_hex`. A unit that lost all its men ashore after the day's marches keeps its march until the next day's, which stops it. [0094] A unit's rehearsal is from 0 to 100 and its hex is set if and only if the rehearsal is above 0; its experience is at most 4; its days of training are below `training_days_per_level` and its days of fighting below `fight_days_per_level`. [0095, 0097] A marching unit's days counted are from 0 up to below `march_days_per_hex`, and none of a unit's eight skills is above 4. [0098, 0099] A unit's fieldworks are from 0 to `fieldworks_limit`, and 0 for a working unit, a unit on attack and a unit with a march standing. [0106] |
| Air groups | Each is at a base that exists or aboard a ship that exists, of its own side, or has been lost with its base, in which case it has no aircraft. [0090] A group at a base that is not its side's, or that has no airfield, has no ready aircraft, no damaged aircraft and no pilots (chapter 5, A group with no aircraft at a base that is not its side's). [0101] The ready and damaged aircraft of the groups aboard a ship do not exceed the class's aircraft capacity. Fatigue and the rest threshold are from 0 to 100. |

[0004, 0016, 0017, 0025, 0035, 0036, 0037, 0038, 0045, 0046, 0066, 0069, 0090, 0094, 0095, 0097, 0101]

Three invariants hold by construction and are not checked: an air group's ready aircraft cannot be negative; an air group has exactly one location; and a side's belief is written only from reports. [0052]

## The state hash and saved games

### The state hash

At the end of each day the engine computes one 64-bit hash of the state and appends it to the daily hashes. It is FNV-1a over a fixed-order walk of the state, begun by the version string `flagplot-state-v22`. A task force's `leg_nm` is written right after its `progress_nm`; version 3 added it. An air group's reach code and the order that set it are written last, after its arc order; version 4 added them. An air group's target list (its count, then each kind's code in list order), the order that set it and the game day of its last base strike (written as an optional number) are written after that; version 5 added them. A ship's experience is written after its location; version 6 added it. A ship's `torpedoes_one_side_fired` is written straight after her `torpedoes_ready`; version 7 added it. Version 8 replaced the one counter of task forces made in play with a counter for each side and each kind of unit made in play, written Allied then Japanese, task forces then air groups. Version 9 added a base's `ship_repair_points_per_day` and `yard_passage_days` after its `fuel_t`, a ship's `system_repair_points` after her `experience`, the ship location `Yard` (code 4) and a task force's `yard_trip` after its `fatigue`. Version 10 added a ship's `troops` (the unit and the men, or none) after her `system_repair_points`, a base's `on_map` after its `yard_passage_days`, a cargo order's land unit (an optional number) after its order, the cargo action `LoadNoCargo` (code 4), and the land units (number, side, men ashore and hex, not the name) after the arrivals. Version 11 added an air group's `rebased_day` (an optional number), written after its last base strike day. Version 12 added a land unit's posture (code 1 for defend, 2 for attack), the order that set it (an optional number), its fatigue and its fed share, written after its hex, and the air group location `Lost` (code 3). Version 13 added a claim's `from_bombardment`, written after its `from_air`, and a side's ground estimates (the count, then for each, in hex order, the hex, the men, the day and the report's event) after its claims. Version 14 added a land unit's march after its fed share: a flag, then if there is a march the hex it marches to, the days marched and the march order. Version 15 added a route's marks after its next index (their count, then each hex in order), the task force status halted (code 5), and a task force's halt after its yard trip. Version 16 added a task force's preparation after its halt (a flag, then if there is one the hex, the roster's count and each unit in order, the loading and the order) and its `landing_at` (an optional hex), and a land unit's rehearsal, the hex of the rehearsal (optional), the activity (a code), the order that set it (optional), the experience and the days of training and of fighting, written after its march. Version 17 added a march's `counted` (the days counted, a fixed-point number) after its days marched, and a land unit's seven skills (assault, holding, weight, clear, jungle, mountain, atoll, in that order) after its days of fighting. Version 18 added a base's `airfield`, `airfield_work`, `defences` and the day's work (the day, the port work and the aircraft work, or none) after its `on_map`, the activity `Work` (code 2) with its job after the activity's code, a skill's `works` after its `atoll`, and a land unit's `working` mark after its skills. Version 19 added a base's `airfield_damage` after its day's work, and an arrival's `waiting` mark after its pilots. Version 20 made a march's order its cause: after the days counted, a code (1 for an order, 2 for the event of a falling back) and the number of the order or of the event. Version 21 added a land unit's instruction for when its attack is spent (`when_spent`: 0 for none, 1 hold, 2 fall back), written after its working mark. Version 22 added a land unit's `fieldworks` (a fixed-point number), written after its `when_spent`, and the day's hits on men, written after the land units: their count, then for each, in hex order and then side order, the hex, the side, the hits and the events of the fire that made them (their count, then each in order). The day's hits are empty at the end of every ground combat phase, so they are empty whenever the state is hashed, and the version names the layout all the same. [0004, 0057, 0058 item 7, 0059, 0063, 0064, 0066, 0069, 0083, 0086, 0090, 0091, 0092, 0094, 0095, 0096, 0097, 0098, 0099, 0103, 0104, 0105, 0106]

The hash covers: the seed; the day; every ship, task force, base, air group, land unit, arrival and replacement stream; both sides' contacts, claims and ground estimates; the count of orders and the next order number and next task force number; and the count of events. It does not cover the names of ships, task forces, bases, air groups or land units, the contents of the events or orders, the map, the reference data, the orders waiting for the next day or the objectives. No tag byte stands in for a field that is not in use. Every field of every unit is written, always, in one order. A field added to the state is added to the hash in the same change, and the version is raised when what is hashed or its order changes. [0004, 0055, 0057]

Two runs of the same scenario, orders and seed give the same hash on any machine. The same hash is given with or without breakdown recording, in two processes, and with the scenario's tables listed in a different order. [0043]

Shuffling means the order the data and the orders are given in, not the order the rules work in. The rules' own order is part of the rules: losses come off the lowest-numbered group first, forces move in order of clock time, a base repairs the lowest-numbered group first. [0043]

### What a saved game holds

A saved game is a file with: the scenario file's name and its hash; each data file the scenario names with its hash; the seed; who held each seat (player or script); the orders of each day for both sides as they were handed in, with their unit, order and origin; and the daily hashes of the run. It holds no state. The starting state is the scenario's. The orders a rule gave inside a day are not saved, because the rules give them again. [0043, 0050]

A saved game also records what it was played under: the version of the state's layout (the version string of the state hash) and the rules label, a number that began at 1 with decision 0100 and that a change raises when it changes a rule in a way that changes what an existing game would do. This is the saved game's version 2. The label is 2 since decision 0103, which added a share to the hit table of a base and so changed what scenario 24 does, 3 since decision 0104, which makes a beaten side's men fall back or surrender and so changes what scenarios 23 and 30 do, 4 since decision 0105, which limits the fire of the men who cannot come to grips and makes a failed attack tire its men, and so changes every live fight ashore, and 5 since decision 0106, which cuts the men a defender and a shelled unit lose by cover, tires men under fire and wears cover, and so changes every live scenario with men ashore under fire or in a fight. [0100, 0103, 0104, 0105, 0106]

A saved game made before seats existed is read as both seats held by the script. [0050]

A batch summary and a replay file for the viewer carry `inputs`: the version of the hash layout, the hash of the scenario file, and each data file the scenario names with its hash. This lets a reader see that two files were made from different inputs. `batch --label` adds the caller's own name for the engine that ran, since the engine does not know its own revision. [0057]

### Replay

`replay`, and every command that reads a saved game, first refuses a saved game whose state layout or rules label differs from the engine's, naming both, and a saved game of version 1, made before the rules label existed, which is not converted. Then `replay` loads the scenario the saved game names, refuses a scenario file or data file whose hash has changed or whose list of data files has changed, hands in the saved orders day by day with no player present, and compares each day's hash with the saved one. It fails on the first day that differs. [0043, 0050]

A replay must reproduce every daily hash. [0043]

Limit: a saved game does not survive a change of rules. A change that raises the rules label is refused before the replay runs; a change that should have raised it and did not shows as a daily hash that does not match. A game in progress is not carried over a change of rules. [0055, 0100]

Limit: a game cannot be saved in the middle and resumed. It is replayed from the start. [0043, 0057]

A side sees: none of this. A saved game, the hashes and the seed belong to the umpire.

## Not in this chapter

- How forces move, burn fuel, refuel, and load and unload cargo, and what becomes of stragglers: chapter 2.
- Air search, sighting reports, contacts, their ageing, readiness and alertness, the alarm, fatigue of forces, claims, and the ground report and the estimate: chapter 3.
- Surface fights, the hit mechanism, damage to ships and shore bombardment: chapter 4.
- Land units, troop capacity, landings, bases formed by a landing or a march, the men's supply, marching overland, preparing a landing, the day's activity, experience and training: chapter 7.
- Air groups, strikes, escorts, patrols, anti-aircraft fire, wear, repair, fatigue of groups, pilots, replacements and base supply: chapter 5.
- Missions, objectives, points, margins and results: chapter 6.
- The agent tools, the briefing, `explain`, `measure`, the serve command and the MCP server: not rules.
- The figures: the data files in `data/reference`.
