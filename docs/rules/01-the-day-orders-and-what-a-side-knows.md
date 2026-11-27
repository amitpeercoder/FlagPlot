# The day, orders, and what a side knows

This chapter holds the rules that frame every other rule: how time is counted, the order in which a day resolves, the orders a side can give and how they are checked, what a side may know, how numbers and chance work, what is checked when a day closes, and what a saved game holds. Chapters 2 to 8 hold what happens inside the phases named here.

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

A side gives its orders, and its commands (chapter 8), before a day, at dusk. In a scenario that turns orders inside a day on, a side may also stop the clock at two moments inside the day, 0600 and 1200, and give its own units seven kinds of order there that act inside the day (Orders inside a day, below); a seat's other orders and its commands to the staff given there are kept and taken in at the next dusk. In any other scenario it cannot interrupt a day or give an order inside it. What happens inside a day is otherwise decided by the standing orders a unit already has and by the rules. The other exception is an order a rule gives for a unit on the spot (see "Who can issue"). [0055, 0156, 0219]

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

0. **Entries.** What is held apart until this day, or until an earlier day it waited from, enters (A unit not yet entered, below). It is the first step: it comes before the commands are taken in, before the staff is asked and before any order is checked, so on its day the staff sees the unit and an order to it is taken. Entries are taken in the order of their keys: task forces by number, then air groups, then land units. With nothing held apart it does nothing and records nothing. No random number is drawn. A submarine held apart until the day comes onto the map after them, by a field of her own and not an entry, in boat number order, with `SubmarineEntered` (chapter 10, The boat). [0195, rules 2 and 12; 0218, rule 33]
1. **Commands.** The commander's commands handed in for the day are taken in: the Allied side's first, each side's in the order handed in. A command changes a side's command state, never a unit. A reserve set here binds from this morning, and so the day's refuelling (chapter 2, Refuelling in port), and a hold or a release set here binds from this morning, before the staff is asked; an interrupt set here is tested this same morning, and an acknowledgement here stands for an earlier firing. Chapter 8, Commands. [0107]
2. **The override.** Each side with an intent in force has its commander's pending orders to a unit inside an intent's forces limit (a boat a patrol may use or a limit names among them, decision 0218) taken in at once, ahead of the day's other orders, and the unit they were for is held from the staff; an order refused holds nothing. The rule's place in the numbering is here, and in the code it runs after step 3. Chapter 8, Holds and the override. [0107, rulings 23 and 30]
3. **Returns from the yard.** Each force whose day of return is this day, in force number order, comes back to its base with its ships repaired, refuelled and rearmed. Chapter 4, Repair. [0069]
4. **The staff is asked.** Each side that has a staff is asked once, on its view as it now stands: after the commands, the returns and the override, and before any other order. Nothing has been refuelled on the morning, so the view holds the fuel as it stood at the end of the day before. It hands in orders, staff commands, decisions to give no order (each of these with a reason) and notices. A side with no staff is not asked. A saved game's hand-in is taken in here in place of an answer. Then the days of supply of the base of each interrupt the commander set are read on that same view. Chapter 8, The staff and Interrupts. [0107]
5. **Staff commands, notices, interrupts, then orders intake.** The staff's commands are taken in, then its decisions to give no order are recorded, then its notices (each a `StaffNotice` event citing the reason it rests on). Then each side's interrupts are tested, Allied side first, on the readings of step 4 and on those notices: each that fires is an `InterruptFired` event, seen by its own side only. Chapter 8, Notices and Interrupts. Then every order submitted since the last day that the override has not taken in, the commander's and the staff's, is numbered, checked and applied, or refused. Events carry no pulse. Each side's orders to form a task force are taken in first, before every other order of the day, and then the rest in the order of the unit they are for, a staff's order after the commander's to the same unit (How orders are numbered and sorted). A staff's order is also refused if its unit is held by the commander or outside the forces limit of the intent it serves (Every reason an order is refused). A yard order, a rebase order, a posture order, a march order, a prepare-landing order, an activity order, a forming order, a join order and a stand-down order are applied here, at their acceptance. This chapter, Orders; chapter 2, The task force formed, joined and stood down; chapter 5, Rebasing an air group; chapter 7, The posture and its order, Marching overland, Preparing a landing, and The day's activity; chapter 8. [0008, 0043, 0069, 0090, 0091, 0094, 0095, 0099, 0107, 0111]
6. **Forward repair.** Each base that mends points of ship damage a day spends them on the forces stopped in its hex. Chapter 4, Repair. [0069]

Entries come before the commands, so that a force that enters today is there for them: a command may name it (A unit not yet entered, below). Commands come next, so that an intent, a limit, a hold or a reserve set today holds from this morning. Returns come next, so that a force back from the yard is at its base for its orders. [0111, 0107] The override comes after the returns and before the staff is asked: the staff sees the holds the commander's orders made that morning, and the orders taken in for it have been numbered ahead of the rest. [0107, ruling 23] Forward repair comes after intake, so that the days a yard order states are the days the force leaves with, and a force ordered to sail that morning is not repaired that day. Steps 3 and 6 are not pulse phases: each runs once a day, outside the pulses. [0069]

Refuelling in port is not among the steps of the morning. Since decision 0112 it is done in the loading phase of each pulse (phase 11 of the night pulse and phase 7 of the morning and afternoon pulses, below), so a force that arrives, or that is formed at intake, takes fuel in the hours it lies in port and not at the start of the next day, and the staff is asked on the fuel as it stood at the end of the day before. [0112]

With no command waiting, no intent in force and no staff, steps 1, 2 and 4 do nothing, as does the staff's part of step 5, and the day is the day it was before decision 0107: there is no notice to record, no interrupt set to test and no standing interrupt that can fire (the loss of a ship fires one only for a side with an intent in force). [0107 item 8, ruling 33]

### Each pulse

For each pulse in order (night, morning, afternoon), the orders a side handed in inside the day for that pulse are first taken in, the morning's those given at 0600 and the afternoon's those given at 1200, with the override they make (Orders inside a day, below; there are none in the night, and none in a scenario without the key) [0156]. Then the reports passed a pulse ago arrive (with command levels on and report figures loaded; Commands and their pictures, below) [0135]:

1. **Alert from belief.** Each force with a ship afloat is alerted or not, from its side's contacts. Chapter 3, Alert from belief. [0021]
2. **Force fatigue.** Each force with a ship afloat gains fatigue if alert and loses it if not. Chapter 3, Fatigue of a force. [0037]
3. **Grounded bases.** The bases with no supply at this moment are listed: their air groups fly nothing in this pulse. The damage to each base's airfield is read with them: its groups fly the share of their aircraft the field works in this pulse. Chapter 5, A base out of supply and A damaged airfield. [0039, 0103]
4. The pulse's phases, in the order below.
5. **Contact ageing.** Each side's contacts age, and old ones are dropped, after the pulse's last phase, night included. Before them, each side's reports of submarines age and are dropped by their own figure, where the scenario has submarine data. Chapter 3, Ageing and dropping; chapter 10, Reports of submarines. [0012, 0218, rule 44]

Steps 1 to 3 run before the pulse's first phase. Each of them does nothing unless its figures are loaded: step 1 needs `surface_combat.toml`, step 2 needs `fatigue.toml`, step 3 needs `air_supply.toml` for the grounded bases (the airfield damage it reads needs no file, and cuts the aircraft whatever is loaded). The last phase, the fires among aircraft, needs the fire figures of `air_combat.toml`. Contact ageing needs `search.toml`. See Where figures come from and when a rule is loaded. [0021, 0037, 0039]

### The night pulse

| # | Phase | What runs in it | Where |
| --- | --- | --- | --- |
| 1 | Aircraft repair | Damaged aircraft are repaired at bases and carriers. | Chapter 5, Repair |
| 2 | Air arrival | Aircraft and pilots the scenario delivers on this day join their groups. | Chapter 5, Aircraft that arrive on a day |
| 3 | Air replacement | Replacement aircraft and pilots arrive at the bases the scenario names, from day 2. | Chapter 5, Replacements |
| 3a | Supply lines | Each supply line whose days hold the day, and whose base is its side's, adds its tons of supply and fuel to the base's stock. A step of its own, after replacements and before the aircraft and the men are paid for. | Chapter 2, Stock from off the map |
| 3b | Replacement men | From day 2, each stream of replacement men whose days hold the day, and whose base is its side's, gives its men to the side's units ashore in the base's hex that are below their establishment and not in a fight, in unit order; a unit above the replacements' level may lose one. A step of its own, after the supply lines and before the aircraft and the men are paid for. | Chapter 7, Men who replace the lost |
| 4 | Air supply upkeep | Each base pays the daily supply for the aircraft at it. | Chapter 5, Daily supply |
| 5 | Men's supply | Each side's men ashore are fed from every base of its side in their hex, the unfed are lost in part, and each unit's fed share is set. Runs only if the ground figures are loaded. | Chapter 7, The men's supply |
| 6 | Air group rest | The staff stands tired groups down and returns rested ones. | Chapter 5, Stand-down and return |
| 7 | Movement | Halted forces whose work is done steam on, then the straggler pass, then forces move in order of clock time (a force halts at a marked waypoint where it has work), and fight where they meet. Last in the step, where a boat is on the map, each submarine's passage and then each submarine's attack on a force that spent hours in her hex, and the hunt of her. | Chapter 2, Movement, Halting at a marked waypoint and steaming on, and Stragglers; chapter 4; chapter 10 |
| 8 | Night search | Empty. No search flies where it is dark. | Chapter 3, Air search |
| 9 | Surface engagements | Empty slot. The fights happen inside movement. | Chapter 4, Meeting |
| 10 | Shore bombardment | A force on the bombardment mission, in the hex of an enemy base on its side's map, fires its guns at the base: aircraft, supply and men there are hit, the base's side is given a sighting report of the force, and the bombarding side a claim. Runs only if the bombardment figures and the air combat figures are loaded. | Chapter 4, Shore bombardment |
| 11 | Loading | Cargo is loaded and unloaded at bases, and men are taken aboard and landed. A force that lands men works at the rate its loading gives, and a unit's first men landing spend its rehearsal. Then ships of forces in port take fuel and then ammunition: berths are given to ships with cargo, fuel or ammunition to take, each force works its cargo, then its ships fuel in the hours left, then take ammunition in the hours that are left after that, rounds before torpedoes. [0112, 0122] | Chapter 2, Cargo, Refuelling in port and Rearming in port; chapter 7 |
| 12 | Aircraft fires | Each fire among a carrier's aircraft that was not set in this pulse is fought: it rises a level or falls one, and a carrier whose fire would rise above the top is lost. Runs only if the fire figures are loaded. The last phase of the pulse. [0212] | Chapter 4, Fire among a carrier's aircraft |

The supply lines are given before any charge on the night's stock, so the men and the aircraft are paid for out of the night's tons. The replacement men join after them and before the men are fed, so they are fed that night. The men's supply follows the daily supply for aircraft, so both charges fall on one stock with the aircraft's first. Shore bombardment follows movement, so a force fires from the hex it ended the night's movement in. The fires among aircraft are fought last, after loading, so a fire is fought in the night pulse as in any other; no raid flies by night, so none is set in it. [0001, 0022, 0035, 0036, 0038, 0039, 0041, 0052, 0066, 0092, 0197]

### The morning and afternoon pulses

Both use the same list.

| # | Phase | What runs in it | Where |
| --- | --- | --- | --- |
| 1 | Air search | Air groups on the search mission fly their arcs and sighting reports follow. A group that flies also looks for boats in its arc: a boat sighted is reported, held down and attacked. | Chapter 3, Air search; chapter 10, Aircraft and the boat |
| 2 | Strike decisions | Groups on a strike mission decide, on their side's belief, and launch. | Chapter 5, The strike decision |
| 3 | Air combat | Escorts, patrols, guns and attacks, then the damage (a hit on a base may land on the men ashore there), then the fires the bombs on a carrier may set among her aircraft. Reports and claims follow. | Chapter 5, The raid, step by step, The raids at a force, taken in turn, and Fire among a carrier's aircraft; chapter 3, Claims |
| 4 | Aircraft wear | Aircraft that flew beyond their radius may run out of fuel, aircraft whose deck is lost land elsewhere or come down in the sea, the rest roll for wear, air groups gain fatigue, and bases pay for sorties. | Chapter 5, The end of a daylight pulse for aircraft |
| 5 | Movement | Halted forces whose work is done steam on, then the straggler pass, then forces move in order of clock time, and fight where they meet. Last in the step, the submarines' passage and attack, as in the night pulse. | Chapter 2, Movement, Halting at a marked waypoint and steaming on, and Stragglers; chapter 4; chapter 10 |
| 6 | Surface engagements | Empty slot. The fights happen inside movement. | Chapter 4, Meeting |
| 7 | Loading | Cargo is loaded and unloaded at bases, and men are taken aboard and landed. Then ships of forces in port take fuel and then ammunition: berths are given to ships with cargo, fuel or ammunition to take, each force works its cargo, then its ships fuel in the hours left, then take ammunition in the hours that are left after that, rounds before torpedoes. [0112, 0122] | Chapter 2, Cargo, Refuelling in port and Rearming in port; chapter 7 |
| 8 | Aircraft fires | As in the night pulse, phase 12: each fire among a carrier's aircraft that was not set in this pulse is fought. A fire a raid set in this pulse's air combat is first fought in the next pulse. [0212] | Chapter 4, Fire among a carrier's aircraft |

Air search runs before movement, so it sees the positions at the start of the pulse. No aircraft repair, arrival, replacement, upkeep or rest phase runs in a daylight pulse. Aircraft fly only in daylight pulses. [0012, 0026, 0035, 0036, 0039]

### Inside the movement phase

The movement phase is one step. In it the straggler pass runs first. Then each force gets its hours for the pulse. Forces that share a hex and will not move meet at the start. Then forces move in order of clock time, and a force that enters the hex of an opposing force fights there. The fights have no phase of their own. Where a boat is on the map the step also counts the hours each force spent in each hex, and when the forces have moved each boat steams and then attacks (chapter 10). Chapter 2, Order of movement within a pulse, and chapter 4, Meeting, give the rules. [0022]

### After the pulses

The five end-of-day phases run in this order. Ground combat is built and runs only if the ground figures are loaded; the next three are empty. [0052, 0067, 0090]

| Phase | State |
| --- | --- |
| Ground combat | First the marches: each unit with a march, in unit order, stops or marches a day, and arrives on the last day with the supply it carries (a unit that fell back carries none). Then, for each hex with men ashore after the marches, in hex order: the day's fight if both sides have men of fighting units there and one is on attack (each side's fire limited to a multiple of the other's men, and the test of an attack that fails worked before any die), each unit's fatigue (with the fatigue of a failed day, and the strain of a day under fire in place of rest), the tired back to defend, what each spent attacker does (holds, or falls back to a linked hex, set to march, tired and carrying nothing), the test of a side beaten, the bases of a beaten side changing side with what they hold and what pointed at them, the beaten side's units with men ashore falling back to a linked hex (set to march, on the defence, tired by the defeat) or, with no hex to go to, surrendering, and last the ground reports of the hex, with the report that a side's men find the hex empty. Then, once every hex is done, the day's activity: each force's loading, then each unit's one activity (rehearsal, training or work; the work of a unit on the airfield of its base mends the field's damage), then the days of fighting counted and the experience they and training raise. Then the day's digging (each fighting unit on defend that did not march gains fieldworks), and last the day's hits on men, counted by the shelling and bombing of the pulses to give the strain, are emptied. Chapter 7; chapter 3, The ground report. It runs after the third pulse, so no aircraft is in the air, and before the base count, so the count reads the day's change of side and the supply a march put down. A game with no ground figures has no such phase. [0090, 0092, 0094, 0095, 0097, 0099, 0103, 0104, 0105, 0106] |
| Logistics | Empty. Refuelling is done in the loading phase of each pulse, and base supply is spent inside the air phases. [0045, 0039, 0112] |
| Air group upkeep | Empty. Repair, arrival, replacement, supply and rest run at the start of the night pulse. [0035, 0036, 0038] |
| Intelligence update | Empty. Contacts age at the end of every pulse, and reports merge when they arrive. [0012] |
| Base count | For every base, in base order, a `BaseAtDayEnd` event records its supply and the ready aircraft of the air groups at it. It is recorded whatever the objectives say, belongs to no pulse, has no cause and changes no state. `base_open` is counted from it (chapter 6). [0067] |

The afternoon's state reports are made after these phases, and then each suspected waters whose day is the next is told to its side (chapter 10, Suspected waters). [0218, rule 47]

Then the day closes:

1. The invariants are checked (see "Invariants"). The state's day is still the day before: a rule that runs inside the day is told its day as an argument.
2. The day counter advances.
3. The state hash is computed and appended to the daily hashes. [0004, 0043]

Chapter 6 holds the result of a game. It is worked out from the final state and the log and is read by no phase. [0054]

A side sees: it sees the events of its own units, as the event table below says, and its own briefing. It does not see the phases. A base that changed side in the ground combat phase is the new holder's for the base count and for the invariants that follow. [0090 item 24]

## Orders

### What an order is

An order is addressed to one unit: a task force, an air group, a land unit, a submarine or, to form a task force, a base. It carries an origin: who issued it, and the intent it serves if any. An order that is taken in gets a number, is kept whether accepted or refused, and has one event: accepted, or refused with a reason. [0008, 0055]

The orders that exist are the ones in the tables below. The spec lists more (add ships, range limit, base orders other than forming a task force, and land unit orders other than posture, march and activity). They are not built. Forming a task force, joining two and standing one down in port are built (decision 0111); standing a force down is what the spec called disbanding it. [0008, 0050, 0052, 0090, 0094, 0095, 0099, 0111]

### Task force orders

| Order | What it sets | Standing slot |
| --- | --- | --- |
| Set route (list of waypoints, and optionally `act_at`, a list of hexes to act at, and `stand_at`, a list of hexes to stand at for the night) | The route, its marks, the route's next waypoint index (to 0), the status (to under way), the order that set the route; clears the recorded arrival and its event, and any halt (chapter 2, Halting at a marked waypoint and steaming on). It may turn a force round between two hexes (chapter 2, Turning back and re-routing). | Route |
| Set speed (cruise, full, or a speed in knots) | The speed setting and the order that set it. | Speed |
| Set mission (transport, surface combat, carrier, bombardment, escort) | The task force's mission. Two rules read it: a force on the bombardment mission fires at an enemy base (chapter 4, Shore bombardment), and a force on the escort mission may be given a charge by the escort order below. The other missions are read by no rule. | Mission |
| Escort (`escort`, and `charge`, a force of the side; with no `charge` it clears the charge) | The force's charge, another task force of its side, and the escort order that set it. A force with a charge keeps station on it and has no route of its own (chapter 2, Keeping station); an enemy meets the charge only past it (chapter 4, Meeting); its guns fire with the charge's at a raid on the charge (chapter 5, Anti-aircraft fire). It is applied when taken in. With no `charge` it ends the charge and the force stops where it is. Only a force on the escort mission may be given one. | Charge |
| Load | A standing cargo order to fill every ship with supply from the base here. | Cargo |
| Load fuel | A standing cargo order to fill every ship with fuel as cargo from the base here. | Cargo |
| Load to a weight (`load` with `weight_t`, a cargo of supply, the default, or fuel, and no unit; and optionally `each_t`, the most any one ship is to hold of it) | A standing cargo order to fill the force's ships, in ship order, until the force has that many tons of the cargo aboard, all its ships together. With `each_t`, no ship is filled past that many tons of the cargo. Both are levels, not additions (chapter 2, Cargo orders). | Cargo |
| Load with a land unit (`load` with a `unit` and a cargo of supply, fuel or none) | A standing cargo order to take the unit's men ashore here aboard, then the named cargo. It needs no base (chapter 7, The load order with a unit). | Cargo |
| Unload | A standing cargo order to land all cargo, men first, then supply, then fuel. | Cargo |
| Do no cargo work (`no_cargo_work`, no arguments) | Clears the force's cargo order: the force holds none, and works no cargo until it is given another (chapter 2, Cargo orders). It moves nothing, aboard or ashore. It is accepted from a force of the side whatever its status, under way, stopped, halted or out of fuel. Added at the end (decision 0191, amended in the build). | Cargo |
| Fuel from (`fuel_from`, and `giver`, a force of the side with an oiler) | The force's own order to fuel at sea from the giver, and the order that set it (`fuel_from`). The force steers for the giver and takes her route; in a pulse the two are paired they make no more than the fuelling speed and her oilers pass it fuel. It ends of itself, and a route order ends it (chapter 2, Fuelling at sea). [0201] | Fuel from |
| Set rules of engagement (stance, break-off level, straggler instruction, keep share) | All four figures together, and the order that set them. | Rules of engagement |
| Set home port (a base) | The base a force heads for when it retires or sends stragglers home. | Home port |
| Set readiness (normal or battle) | The readiness and the order that set it. It does not touch whether the force is alerted. | Readiness |
| Set radio silence (`set_radio_silence`, `on`) | The force's radio silence, on or off (Commands and their pictures, Radio silence). Taken in at once; it records its acceptance and nothing else. It has an effect only where reports travel between commands. | None |
| Detach ships (a list of ships) | Not a standing order. Splits the listed ships into a new force at once. | None |
| Join (`join`, and `into`, a force of the side) | Not a standing order. Moves every ship of the force, with all aboard, into the named force at once, and the force joined ceases to be. Both are stopped at the same base of the side (chapter 2, The task force formed, joined and stood down). | None |
| Stand down (`stand_down`, no arguments) | Not a standing order. Puts every ship of the force in port at the base it is stopped at, in no force, at once, and the force ceases to be (chapter 2, The task force formed, joined and stood down). | None |
| Send to yard (`send_to_yard`, no arguments) | Not a standing order. Sends the whole force, stopped at a base that is a port for the yard, away to the yard at once (below, and chapter 4, Repair). The port is the base it is stopped at. | None |
| Prepare a landing (`prepare_landing`, a hex and a roster of land units) | The force's preparation: the coast hex of the landing and the roster, with the loading it keeps and the order that set it. It is applied when taken in, and what a new place, a unit added or dropped and the same order again do is in chapter 7, Preparing a landing. It stands until replaced or until the landing is done, and may be given at sea. | Preparation |

[0008, 0011, 0014, 0016, 0017, 0021, 0046, 0066, 0069, 0092, 0095, 0096, 0111, 0121, 0136, 0191]

An escort order that gives a charge changes no position, route or speed when it is taken in, and records nothing beyond its acceptance. The escort takes its station at the start of the next pulse's movement (chapter 2, Keeping station). An escort order that clears a charge records no `ChargeLost`: the force stops where it is, and only a turning back, if it makes one, is recorded. A force's charge ends of itself where the charge has no ship afloat, or is joined into another force or stood down in port (chapter 2, Keeping station). [0121]

A route, load, load-fuel, load-to-a-weight or load-with-a-unit order to a force that is out of fuel is refused (Every reason an order is refused, below; differences.md rows 17 and 34).

The stance is engage or avoid. The straggler instruction is keep, detach, leave or retire. Chapter 4, Breaking off, and chapter 2, Stragglers, give what they do. The readiness is read by chapter 3, Readiness, alertness and fatigue of a force. [0014, 0016, 0021]

Detaching ships creates a new force with the next number of its side's series (the Allied side's from 1,000,001, the Japanese side's from 2,000,001). It starts at the parent's exact position and leg (the charge of the leg with it), idle, with no route. It copies the parent's side, mission, speed setting and speed order, home port, readiness and readiness order, alerted flag and the event that alerted it, fatigue, and rules of engagement and rules order. The event that accepts the order is recorded first, then `ShipsDetached`, with the same day and pulse, citing the order (differences.md row 24). [0016, 0037, 0086]

A join or a stand-down order, when it is taken in, is applied after its acceptance, and `TaskForceJoined` or `TaskForceStoodDownInPort` is recorded, citing the order. The ships' places change, the orders of the force joined or stood down end (its route, its cargo order, a halt and a landing begun) and it ceases to be: it stays in the state with no ship, as a force whose ships were all sunk does, and its number is never used again. Chapter 2, The task force formed, joined and stood down, gives the rest. [0111]

A send to yard order, when it is taken in, is applied after its acceptance: the ships leave the force for the yard, the force is left idle at its base with no route, cargo order, leg or alert, and `ForceLeftForYard` is recorded, citing the order, with the day of return. What the force keeps and what it gets on return are in chapter 4, Repair. An answer to a queued yard order states the day of return, or says the force does not return if that day is after the last day of the game. [0069]

### Base orders

| Order | What it sets | Standing slot |
| --- | --- | --- |
| Form a task force (`form_task_force`, to a base, and `ships`, a list of ships) | Not a standing order. Forms the listed ships, which are the side's, lie in port at the base and are in no force, into a new task force at once (chapter 2, The task force formed, joined and stood down). In a scenario it is an `[[order]]` with `base = N`. | None |

[0111]

A forming order, when it is taken in, is applied after its acceptance, and `TaskForceFormed` is recorded, citing the order.

### Air group orders

| Order | What it sets | Standing slot |
| --- | --- | --- |
| Set air mission: rest, search, naval strike, combat air patrol, or base strike on a named base | The mission and the order that set it. | Air mission |
| Set search arc (from bearing, to bearing, range in miles) | The arc and the order that set it. | Search arc |
| Set patrol share (0 to 1) | The share of its flyable aircraft a group on patrol keeps over its own hex. | Patrol share |
| Set rest threshold (0 to 100) | The fatigue above which the staff stands the group down at the start of the day. | Rest threshold |
| Set reach (normal or extended) | How far its strikes and escorts may go: the type's strike radius, or a share more (chapter 5, Radius and reach). Normal is the default. | Reach |
| Set target priority (a list of kinds of ship: carrier, battleship, cruiser, destroyer, transport, auxiliary) | The kinds a group's strikes go for, in order, and the order that set the list. The default is all six kinds in the order of importance. A kind not on the list is never launched at (chapter 5, Naval strike: the contact), and the list ranks the ships a group attacks (chapter 5, The attack on a ship). | Target priority |
| Set strike contact age (`set_strike_contact_age`, whole pulses) | The oldest contact, in pulses, the group launches a naval strike on, and the order that set it. With none set, the figure `air_combat.toml: strike_contact_max_age_pulses` serves. Not read by a base strike (chapter 5, Naval strike: the contact). [0114] | Strike contact age |
| Set strike held share (`set_strike_held_share`, 0 to 1) | The share of the aircraft a group is able to fly that a naval strike or a base strike holds back, and the order that set it. 0 is the default (chapter 5, Held back from a strike). [0114] | Strike held share |
| Set strike sizing (`set_strike_sizing`, `on` true or false) | Whether a naval strike of the group is sized to the contact it goes for, and the order that set it. Off is the default (chapter 5, A strike sized to its target). It is never refused for its value, and a scenario gives it as an `[[order]]` with `on`, not as a key of the group. [0118] | Strike sizing |
| Set search once a day (`set_search_once`, `on` true or false) | Whether a group on the search mission searches in the morning pulse only, and the order that set it. Off is the default: it searches in both daylight pulses (chapter 5, Air group orders). It is never refused for its value, and a scenario gives it as an `[[order]]` with `on`, not as a key of the group. [0131] | Search once a day |
| Set strike first waters (`set_strike_first_waters`, `hexes`, a list of hexes) | The waters whose contacts a naval strike of the group takes before any outside them, and the order that set it. Empty is the default and clears it; the strike cites the order while there are waters (chapter 5, Naval strike: the contact). A scenario gives it as an `[[order]]` with `hexes`, not as a key of the group. [0190] | Strike first waters |
| Rebase (`rebase`, an air group and a base) | Not a standing order. Moves a group at a base to another base of its side at once and writes off its damaged aircraft (below, and chapter 5, Rebasing an air group). | None |

[0012, 0026, 0027, 0034, 0036, 0059, 0063, 0091, 0114]

### Land unit orders

| Order | What it sets | Standing slot |
| --- | --- | --- |
| Set posture (`set_posture`, a land unit and `attack` or `defend`; an attack order may add `when_spent`, `hold` or `fall_back`) | The unit's posture, at once, the order that set it and its instruction for when its attack is spent (chapter 7, The posture and its order). A tired unit going back to defend spends the order and the instruction. A unit set to attack leaves its fieldworks. | Posture |
| March (`march`, a land unit and a hex) | A march of the unit's men ashore to the hex, a neighbour joined to the unit's hex by a land link, taking `march_days_per_hex` days; the unit's own hex stops a march (chapter 7, Marching overland). A unit sent to another hex leaves its fieldworks. It stands until the unit arrives or another march order replaces it. | March |
| Set activity (`set_activity`, a land unit and `ready`, `train` or `work`, with a `job` for `work`: `airfield`, `port`, `aircraft` or `defences`) | The unit's activity for the days it neither marches nor fights, its job if it works, and the order that set it, at once (chapter 7, The day's activity). A unit set to train gains days of training toward experience (chapter 7, Experience and training); a unit set to work makes work for its job (chapter 7, Units that work and what they build). A working unit takes only `work`. Needs no man ashore. | Activity |

[0090, 0094, 0097, 0099, 0106]

### Submarine orders

| Order | What it sets | Standing slot |
| --- | --- | --- |
| Station (`station`, a submarine and a hex) | The hex a boat keeps, at once, and the order that gave it. She goes there by a passage at her class's passage speed and is on station the pulse after she arrives (chapter 10, The station order). It stands until another station order replaces it. It is not one of the seven kinds that may be given inside a day, so it is kept for the next dusk (Orders inside a day, below). | Station |

[0218, rules 3 and 43]

### When an order is checked, and against what

An order is checked once, at the moment it is taken in, against the state as it then stands. For the scenario's script and for orders a side hands in, that is the start of the day. An order a staff hands in is taken in at the same moment, after the staff's commands, and is checked the same way. For an order a side hands in inside a day, at 0600 or 1200, it is the start of the pulse that follows (Orders inside a day, below). For an order a rule gives inside a day, it is the moment the rule gives it. [0008, 0050, 0156]

An order is taken in after the day's returns from the yard, and after the staff is asked, so a force that is back from the yard that day is checked as back. [0069, 0107]

A side may ask the engine's own check early, with `check_order`. It numbers nothing, records nothing and changes nothing. Its answer is a forecast. The state may change before the day begins, and an order that passed the forecast can be refused at intake. [0050]

A side that queues several orders for a day may ask the check beside them, with `check_order_queued`: the same check, for an order handed in beside the other orders of the same side already queued for the day. It differs from `check_order` in one case. An order to a task force that does not exist yet, where a forming order is queued (among those given with it, or already submitted), is checked as intake will check it: on a copy of the world into which the queued forming orders have been taken, in intake's order, so that the force they make is there, with the number it will have. Any other order is checked as `check_order` checks it. Nothing is changed. The agent's session uses it for a player's orders. It does not know what a staff will hand in, and intake takes a staff's forming orders in with the commander's, so where a staff also forms a force that day the number the check gave a new force may not be the one intake gives it. A staff's orders meet no check before intake. [0111, 0107 As built]

The check reads the truth, but a refusal may cite only facts the issuing side can see: its own units, the map, ownership and the reference data. A fact the side cannot see is never a reason. "Not your unit" is the single reason for a target that does not exist and for a target that belongs to the enemy, so a refusal never shows that an enemy unit exists. "Not your base" does the same for bases. "Not your land unit" does the same for land units, and "not your unit" for a submarine. The base strike reason "not an enemy base" is worded to cover a base that does not exist, a base the group's own side holds and a base the enemy formed by a landing or a march, which is not on the group's side's map. [0008, 0034, 0050, 0066, 0094]

Limit: 0008 said an order that would fail on something the side does not know is accepted and fails in execution. No such failure exists, because every check today reads only what the side can see. The source comment on the check says orders are checked against the truth. [0008]

### Orders inside a day

A scenario may turn orders inside a day on with `orders_inside_a_day = true` at its top; it is off when absent. With it on, a side may hand in orders at two moments inside a day as well as at dusk: at 0600, when the night pulse is done and the morning is next, and at 1200, when the morning is done and the afternoon is next. With it off, no order is given inside a day, and every game is as it was. [0156, rules 1 and 2]

Seven kinds of order may be given at 0600 and 1200, each a standing setting the pulses read and none that makes or unmakes a force:

| Unit | Order | What it changes from the next pulse |
| --- | --- | --- |
| Task force | Set route | Where it steams |
| Task force | Set speed | How fast |
| Task force | Set readiness | Its readiness for a fight |
| Task force | Set rules of engagement | When it breaks off |
| Air group | Set air mission | What it flies; rest stands it down |
| Air group | Set search arc | Where it searches |
| Land unit | Set posture | Whether it attacks in the day's fight ashore |

Any other order that reaches the engine at 0600 or 1200 is refused when it is taken in, as given at dusk: "this kind of order is given at dusk: inside a day only a task force's route, speed, readiness or rules of engagement, an air group's mission or search arc, or a land unit's posture may be ordered". The reason is tried before every other. Like any refused order it changes nothing but the log and the order counter. A scenario's script gives its orders inside a day so, and is refused so. [0156, rule 3]

**An order a seat gives at a stop** (decision 0219). A seat may give, at 0600 or 1200, any order it may give at dusk. An order of the seven kinds acts as above. Any other is not handed to the engine there: the seat's session keeps it and hands it in at the next dusk with the seat's orders of that dusk, in the order given, as if given then. It is checked at once against the state as it stands, beside the orders already kept, and refused at once where it would be refused whatever the hour (a unit not the side's, a place off the map); it is checked again when it is handed in, as every order of dusk is, and may be refused then for what has changed. Its answer says it is kept and acts at 1800 for the coming day. Clearing the seat's orders at a stop inside a day clears those kept for dusk with those for the next pulse. A kept order holds its unit from the staff only when it is taken in, at dusk, before the staff is asked. What is kept is the seat's session's, not in the state or its hash; the engine's intake is not changed. Two tools of the session list what a seat has queued, the orders for the next pulse, the orders kept for dusk and the commands, each with its place, and take one entry back with what goes with it (chapter 8, The tools; decision 0221); a kept order is listed and taken back as any other, and what is taken back is gone before it is handed in. Commands to the staff given at 0600 or 1200 are likewise queued and taken in at the next dusk, and their answers say so. [0219]

**When it is taken in.** An order handed in at 0600 is taken in at the start of the morning pulse, and one handed in at 1200 at the start of the afternoon pulse, before anything else of that pulse: before the reports passed a pulse ago arrive and before alert from belief (Each pulse, above). The sides are taken Allied first, and each side's orders in the order intake sorts them (How orders are numbered and sorted, below). Each is numbered then, checked against the state as it then stands, and applied or refused as an order of dusk is. Its acceptance or refusal, and every event its acceptance leads to there, carries the pulse it was taken in. So an order of 0600 acts in the morning pulse and one of 1200 in the afternoon, as an order of dusk acts in the night that follows it; no delay is added. With none handed in, nothing is recorded and no random number is drawn. [0156, rule 4]

**The override inside a day.** An accepted order whose unit lies in the forces limit of an intent in force (or is a boat a patrol in force may use) holds the unit from the staff, as the override does at dusk (chapter 8, Holds and the override), with a held by order event in the same pulse, citing the order and its acceptance. A unit already held is not held again. The staff is not asked inside a day: it reads the hold when it is next asked, at dusk, and gives the held unit no order. [0156, rule 5]

**Nothing waits at a day's end.** Orders handed in inside a day wait, not hashed, only until the start of the pulse they are for; none is handed in after the afternoon begins, so none waits when the day closes and the state hash is taken. [0156]

**In a scenario's script.** An order of the script may carry `pulse = "morning"` (given at 0600) or `pulse = "afternoon"` (given at 1200), only in a scenario with the key on. An order with any other pulse, or with a pulse in a scenario without the key, is an error when the scenario is loaded. An order with no pulse is an order of dusk. A scripted side and a staff otherwise give their orders at dusk only. [0156, rule 10]

**At the tools.** A seat stops the clock at 0600 and 1200 with the tool `advance`, offered where the key is on and one seat is a player's; it gives orders there with `issue_orders`, each checked at once and refused there already if it is not of the seven kinds. A command to the staff given there with the command tools is queued, as at dusk, and handed to the engine at the next dusk, so it is taken in at the start of the day after; it is checked at once on a copy of the game with the open day treated as closed, and again when it is taken in. A trial (chapter 9, A seat's trial, rule 14) may be asked there too: it is begun on that copy, its first day is the day after, and the rest of the open day is not played. The tools are the interface's, not rules: `crates/sim-agent/schemas/README.md` and `docs/commands.md`. [0156, rules 6 and 8]

A side sees: its own orders given inside a day accepted or refused, with the reason and the pulse they were taken in, and the held by order events they make, as it sees its orders of dusk. The enemy sees nothing of them. [0047, 0156]

### The stop in the evening

A seat that runs time is stopped, in one case, at dusk **before** the day is played and not after it: where its staff would ask the commander's leave to go after what a carrier force has found (chapter 8, Operating a carrier force, items 28 to 41; Interrupts). Every other stop of a seat is after a pulse or a day, as above, and so are the three a seat may ask for of the staff's notices of an order to destroy a force: a taking of the other sort, a loss and an end (chapter 8, Destroying a force, Stops), off unless asked for, after the day on which the notice was handed in and never before it. [0214, rule 7; 0220, R11]

1. **Who and when.** A call of `run` or `advance` made at dusk, with no day open, is stopped before the day opens where the seat's side has a staff, has an intent in force, has not switched the standing interrupt `asks_leave_to_pursue` off, and the staff, asked on a copy of the coming day with the side's queued commands taken in, hands in a notice that a force asks leave to pursue for a force whose record of that interrupt is absent or armed. A seat at a task force or a base is not stopped. `end_day` is not stopped, and the stop is never inside a day: at 0600 and 1200 the staff is not asked. [0214, rules 7 and 9]
2. **What it is.** Nothing is resolved and nothing in the game changes: no state, no event, no draw, no hash. The call is answered with `stopped` of `asks_leave` and the askings, the clock standing at the dusk. For `run` a stop before the first day resolves none and the firings of the day before are not the call's. [0214, rule 7]
3. **Once for a day.** The stop is made once for a side for a day. The next call of time at that dusk plays the day, with the queued commands taken in at its first step, before the staff is asked (chapter 8, The staff, and the day's order). [0214, rule 7]
4. **The answer goes in the same dusk.** An `answer_pursuit` given at the stop is queued as any command is and taken in at the first step of the day, so a yes begins the force's pursuit before the staff is asked, the staff asks nothing more for that force, and the force sails that night. With no answer the staff asks again at the next dusk if the find still stands; the seat is not stopped again for it unless a morning has passed on which the force was not asked (the standing interrupt is armed again). [0214, rules 8 and 9]
5. **A firing after the day does not stop `run` again.** The interrupt still fires on the morning of the day played and is listed in the command report; `run` does not stop after that day for it. [0214, rule 7]
6. **In a watched game** a held call of `run` or `advance` is stopped at the same point and answered Allied first when two seats stop on one step; a held `end_day` is not stopped. [0214, 0182]

### A watched game

A game may be opened as watched, with `Session::watched` or `sim-cli serve --watched`. It has a third party beside the two seats, the watcher, who holds the clock. He has no seat, is no side and gives no order. His one act that moves time is a step: one pulse where the scenario has orders inside a day, one day where it has not. A step is refused while a played seat is not ready, naming the seats waited for, and when the game is over, stopped on an error or not watched. A game not opened as watched is as it was. [0182, rules 1 to 3]

**Ready.** A played seat is not ready at the start. It becomes ready by a call of time, each with its own stop: `end_day`, until the next dusk; `run`, for that many days or until a dusk after a day on which an interrupt of its side fired, or until the dusk before a day for which its staff would ask leave to pursue (The stop in the evening, above); `advance`, for that many pulses or until the end of a pulse in which its side was told one of the things the call names, or a dusk after an interrupt, or the dusk before a day it would be asked in. A scripted seat is always ready. [0182, rule 5, amended 3]

**The held call.** A call of time whose arguments stand is held and the seat is ready; a call refused for its arguments is answered at once and the seat is not ready. A held call is answered when a step brings its stop, as the same tool answers in a game with that seat alone played, and its entry in the transcript is written then. Each seat's stop is its own, and an interrupt stops only the seat that set it; two stops on one step are answered Allied first. If the engine stops on an error every held call is answered with the words of the stop. Queued orders and commands go in at the first dusk only, as for `run`. [0182, rules 6 and 7, amended 2 to 6, 9 and 10]

**A held seat.** While a seat holds a call, any other call of it is refused, saying it is ready and waits for the clock; the refusal changes no state and is in the transcript. This changes decision 0050, rule 9, in a watched game only: there a seat that has ended its day no longer reads. [0182, amended 8]

**Run and advance offered to both.** In a watched game `run` is offered to every played seat that has the command tools, and `advance` too where the scenario has orders inside a day, whatever the other seat is. A seat at a task force or a base still has no `run`. [0182, rule 5]

**The watcher's reads.** He reads as a seat's commander reads, from either side, one at a time, with the read tools of the seat: `get_briefing`, `list_forces`, `list_bases`, `list_contacts`, `get_entity`, `measure`, `explain`, `events`, `intelligence`, `command_state`, `plans`, `staff_orders`, `propose`, `order`, `why`, `report` and `trials`, those of them the seat has. He has no tool that orders, commands, acknowledges or moves time, and neither `preview` nor `trial`, which leave a record: each is refused as a tool he does not have. His reads are in no transcript and change no state, no hash and no number of a preview or a trial. He is given nothing a seat is not: no truth of the enemy. He is told where the game stands (the day and the moment, and for each played seat whether it is ready and until when, or waited for) and what a seat did since its last stop. [0182, rules 10 to 14, amended 13 and 14]

**Kept.** A watched game left inside a day is saved with `stopped_inside_day` (decision 0172). The saved game replays without a watcher to the same hashes: who held the clock is no part of it. No state, draw or hash changes. Built so far: the session and `serve --watched` (step 1). The table's bridge, its doors and its page (step 2) are not built. [0182, rule 20, amended 19 and 24]

### Every reason an order is refused

The reasons are tried in the order written. The first that applies is the one recorded. All other orders are accepted.

### A day from which a unit may be used

A scenario may give a task force, a ship, an air group or a land unit a day from which its side may use it, `available_from = N`, a whole number from 1. With none it may be used from day 1. A unit's day is the later of its own and the days of what carries it: a ship takes the later of hers and her task force's (the force she is in, or is away from at the yard); a task force takes the later of its own and each of its ships' own, since an order to the force moves every ship in it; an air group aboard a ship takes the later of its own and the ship's, as the ship's is read; a land unit takes the later of its own and that of each ship with its men aboard. A base has no day. A force made in play has no day of its own. The day is set by the scenario and changes in play for nothing. [0140 items 1 and 2]

Before its day no order to the unit is taken, from the commander or from a staff. The test is tried before every other reason (and, for a staff's order, after the hold and before the forces limit): an order of the side for a day before the day of its own unit is refused **not yet available**, "U may be used from day N", naming the unit and the day. The units it reads are the order's own unit, the ships a base's order to form a task force names, and the land unit a task force's order to load a unit names. An order to a unit that is not the side's is left to "not your unit", so that no refusal gives a side the enemy's day. A unit before its day is still there: it lies where the scenario put it, at the settings the scenario gave it, and every rule acts on it as on any unit. It can be seen and attacked and it defends itself. A unit that is not on the map at all until a later day is another thing (A unit not yet entered, below). After its day a unit is the side's to use as it sees fit. [0140 items 2, 4 and 6]

A side sees: its own units' days, in its view and in the tools ("available from day 9", while the day has not come). Nothing gives a side the enemy's days. [0140 item 5]

### A unit not yet entered

A scenario may give a task force, or a land unit or an air group at a base, a day on which it enters the game, `enters = N`, a whole number from 2 to the game's last day. Until that day the unit is **held apart**: it is not on the map. It is in no hex. It is not searched for, sighted, struck, engaged, fed, tired, worn or fuelled, and it burns nothing. No rule of a day acts on it, because every rule of a day walks the ships, task forces, air groups and land units of the state and a unit held apart is not among them. It is kept whole, as the scenario wrote it, in a list of its own in the state (`TruthState::entering`), one entry for each unit that enters, keyed by that unit. This is not "a unit before its day" (A day from which a unit may be used, above): that unit is on the map, and it is seen and attacked; this one is not there. [0195, rules 1 and 2]

**What enters.** A task force enters with everything aboard it: its ships, the air groups aboard them and the land units aboard them, none of which carries the key itself. A land unit or an air group enters alone, at a base. [0195, rules 1 and 2]

**When.** It enters at step 0 of its day, the first step of the day (The order in which a day resolves, above), and is tried again on each later day while it waits. Nothing of the day's pulses has happened. On its day it is a unit like any other. [0195, rules 3 and 12]

**Where.**

- A task force enters at the hex the scenario gives it, stopped, at the settings the scenario gives it, whoever is in the hex. A route and a speed the scenario gave are kept: it sets out on them when it enters. The scenario is refused where the hex is off the map or land (the loader's reasons are in `docs/commands.md`, Scenario keys).
- A land unit enters at its base, its men ashore in the base's hex, as written. It waits while the base is not its side's on the day; it waits, and is tried again each day, with `EntryWaiting` recorded each day it waits.
- An air group enters at its base, as written. It waits while the base is not its side's, and, if it has aircraft (ready or damaged), while the base takes none (chapter 5, A damaged airfield). A group with no aircraft enters wherever its base is its side's.

The base is the base in the unit's hex, its side's where both sides have one there. [0195, rules 3, 4, 16 and 17]

**Events.** Each is its own side's alone, whole, and cites no cause (as an arrival of aircraft cites none), with no pulse and the day of entry. `ForceEntered` names the force, its side, the hex, the base of its side in the hex if there is one, and its ships, air groups and land units. `UnitEntered` names the land unit or air group, its side, the base, the hex, and the men ashore (a land unit) or the aircraft, ready and damaged (an air group). `EntryWaiting` names the unit, its side, the base, the day the scenario named and why it waits: the base is not its side's, or it takes no aircraft. [0195, rules 6 and 24]

**Command levels.** With command levels on, a force that enters becomes a command at that moment, as one formed in play does (Commands and their pictures, below): under the command the scenario placed it in (`superior`) if that is still a command of its side, else under the area command, with the commander the scenario named for it. A force the scenario put under radio silence keeps it from the moment it enters. With command levels off a force that enters is no command, and a commander named for it is not kept. A force held apart is not a command at the start of the game: a scenario that names it as another command's superior, or as the `seen_by` of a contact, is refused as for any force that is not a command of the side at the start. A superior named for a held force must itself be a command of the side at the start, and not the force itself. [0195, rules 11 and 19]

**What a side knows.** Its own units held apart, exactly: each with its day, its place (a hex, or a base) and what it is (its ships by kind, its air groups with their aircraft, its land units with their men), and whether it waits. The side's view lists them (`SideView::entering`), and a unit held apart is its own (`owns_unit` is true) with the day it enters for its `available_from`; it may not be used (`may_use` is false) whatever the day. The aircraft arriving to a group held apart are in its arrivals. The briefing and the tools give them (`to_enter`, `entered_yesterday`, `entry_waiting_yesterday`). The evening's briefing also says what came today and what comes tomorrow (decision 0215): `came_today`, each task force and land unit of the side's that entered in the day just played, each air group that received aircraft, and each unit whose day to be used was that day, with where it is, what it carries and what is explicit of what the staff did with it (that the staff gave it an order that day and the order it serves, that an order's forces limit names it, that the commander holds it or gave it an order, or else that the staff gave it no order that day; never that no order uses it); and `comes_tomorrow`, each that enters, receives aircraft or may first be used the next day, so that the commander may hold it or order it before the staff does. A unit that does more than one has one line, and what a force carries is in the force's line. Neither list has anything of the first day. Nothing of the enemy is in either. [0215, as amended on review] Of the other side's units held apart a side knows nothing, not that they are coming and not their day, until it sights them on the map: nothing of them is reachable through the view, and the three events are not shown to the enemy. [0195, rules 6, 7, 13 and 15]

**Orders.** An order of a side to its own unit held apart, or naming one (the order's own unit; a ship a base's order forms a task force from; the land unit a task force's order is to load), is refused, before every other reason, before even "not yet available": **not yet entered**, "U is not on the map yet: it enters on day N" naming the unit and the day the scenario gave, or, where that day has come and the unit waits for its base, "U is not on the map yet: it was to enter on day N and waits for its base". This is so for a staff's order as for the commander's. An order naming the other side's unit held apart is refused as for a unit that is not one's own ("not your unit"), saying nothing of a day, so that no refusal tells a side what the enemy has coming. [0195, rules 8 and 15]

**Commands.** A forces limit (ships, task forces, air groups, land units, submarines), an intent given to a task force (operate a carrier force, stand at, ready a landing, bombard, any kind that names one) and a hold may name a unit of the side's own held apart. The other side's is refused as it is today, as not the side's. Where a command reads the named unit's own state (a force's mission, its ships), it reads it from the entry. The staff reads a unit held apart as one that may be used from its day, counts it as coming and leaves it out of every pool until then (chapter 8, A unit before its day). [0195, rules 8 and 14]

**With `available_from`.** A unit may have both keys. It enters on its day, and the later of the two days binds its orders: the day a unit held apart may be used is the later of the day it is next tried and the days its own `available_from` and those of what carries it give. A unit that waits is next tried on the later of its day and the day after the last day resolved. `available_from` alone is unchanged. [0195, rule 9]

**Numbers.** The scenario's numbers are taken from the first day. A land unit made by division takes the next number above the highest of the state's land units and those held apart together; with nothing held apart that is the number it took before. Bases are never held apart, so a base formed in play takes its number as before. A task force or air group made in play takes its number from the series of its own side (above), which never meets a scenario's. [0195, rules 5 and 18]

**Arrivals.** An arrival of aircraft to a group held apart waits, silently (Chapter 5, Aircraft that arrive on a day). [0195, rule 5]

**Objectives and the end.** A ship that has not entered was never at risk and is in no count. A game that ends before a unit's day ends without it. [0195, rule 10]

**Not done.** Nothing leaves the map. A force that has entered stays; a force sent to the yard (chapter 4, Repair) is the one way a force goes off the map. [0195, rule 22]

A side sees: its own units held apart, exactly, with their days and places, and the three events of its own units. Nothing of the other side's. [0195, rules 7 and 24]

An order handed in inside a day, at 0600 or 1200, is first tried against one reason before every other: given at dusk, if it is not one of the seven kinds an order inside a day may be (Orders inside a day, above). [0156]

An order a staff hands in is first tried against two more reasons, in this order, before every other below:

| Reason | When |
| --- | --- |
| Held | The order is for what the commander holds: a task force that is held or has a held ship in it; an order to form a task force that names a held ship, or a ship in a held force; or a search-arc or air mission order for an air group whose search arc is held; any order for an air group held whole; or any order for a land unit that is held (decision 0141, 14), or any order for a submarine that is held (decision 0218, 34). "The unit is held by its commander", with the target. Tried before the forces limit. A base order other than forming is never held. Chapter 8, Holds and the override. [0107, 0115, 0141] |
| Outside the forces limit | The order's intent is not in force or has no forces limit, or the order is for a unit that is not inside that limit: "the order is for a unit outside the forces limit of the intent it serves". A task force is inside the limit if the limit lists it or if every ship of it, and at least one, is a ship the limit lists. A base order is inside it only to form a task force at the intent's source base from ships in the pool. An air group is inside it if the intent may use it, a land unit if the intent may use it (decision 0141), and a submarine if the intent is a patrol in force that may use her (decision 0218, 12 and 34); any other base order is inside no limit. Chapter 8, The forces limit on a staff's orders. [0107, 0115, 0141] |

The order is then numbered, kept and refused for the first of these that applies with no other check. An order that is neither held nor outside the limit is tried against the reasons below as any order is. The checks bind orders handed in by a staff, not the commander's orders and not an order a scenario's script gives with the issuer staff, which is a scenario's statement. [0107]

For a task force order:

| # | Reason | When |
| --- | --- | --- |
| 1 | Not your unit | The force does not exist, or its side is not the issuer's. |
| 2 | Away at the yard | The force is away at the yard and its day of return is after the day the order is taken in for. A force that is back by that day is checked as back, with the ships it has at the yard, so a side may give orders the evening before the day of return (`check_order` asks for the next day). Every task force order is tried against this, so a force away takes no orders. [0069] |
| 3 | No ship afloat | The force lists no ships (every ship it had is sunk). A force away at the yard is read by the ships it has there. A force with no ship that was joined to another or stood down in port is refused with its own reason in place of this one, read from the log: "the task force no longer exists: it was joined to F", with the force it was joined to, or "the task force no longer exists: it was stood down in port". [0111] |

Then, by kind:

| Order | Reason | When |
| --- | --- | --- |
| Set route | Out of fuel | The force's status is out of fuel. Tried before the waypoints. [differences.md row 34] |
| Set route | Keeps station | The force has a charge: "the task force keeps station on its charge F and has no route of its own", naming the charge. Tried after the out-of-fuel reason and before the waypoints. [0121] |
| Set route | Waypoint off the map | A waypoint is not on the map. The first waypoint is tried first, then each in turn. |
| Set route | Waypoint unreachable | The first waypoint cannot be reached by sea from the force's position (or the hex it would turn back to), or a later one cannot be reached from the one before. A land hex is unreachable. |
| Set route | Empty route | The force does not move anywhere: no waypoint other than where it already is, and no leg in progress to finish. |
| Set route | Mark not on route | A hex in `act_at` or `stand_at` is not one of the route's waypoints (the last waypoint counts as one, though it is never treated as marked): "H is marked to act at but is not one of the route's waypoints", in the same words for either mark. Tried after the other reasons above. [0096, 0124] |
| Load to a weight | Weight not above zero | The weight is 0 or below: "weight W t is not above 0". Tried before the three reasons below. [0111] |
| Load to a weight | Level not above zero | The level `each_t`, the most a ship is to hold, is 0 or below: "the most a ship is to hold, X t, is not above 0". Tried after the weight and before the three reasons below. [0162] |
| Load, Load fuel, Load to a weight | Under way | The force's status is under way. |
| Load, Load fuel, Load to a weight | Out of fuel | The force's status is out of fuel. [differences.md row 17] |
| Load, Load fuel, Load to a weight | No own base here | No base of the issuer's side is in the force's position. |
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
| Join | Cannot join itself | The force named is the force the order is for: "a task force cannot join itself". [0111] |
| Join | Not your unit | The force named does not exist or is not the issuer's. One reason for both. [0111] |
| Join | Away at the yard | The force named is away at the yard and its day of return is after the day of the order (the same test as row 2, for the force named). [0111] |
| Join | No ship afloat | The force named lists no ships: the reasons of row 3, including the two for a force joined or stood down. [0111] |
| Join, Stand down | Under way | The force's status is under way. Tried for the force the order is for, and then for the force named. [0111] |
| Join, Stand down | Route to follow | The force's status is halted at a marked waypoint, with a route to follow: "the task force is stopped on its way and has a route to follow". [0111] |
| Join, Stand down | Out of fuel | The force's status is out of fuel. [0111] |
| Join, Stand down | No own base here | The force is idle or arrived but no base of the issuer's side is in its hex, or it is part-way along a leg: the force is at sea, part-way between two hexes, or at a base of the other side only. The base is the lowest-numbered base of the side in the hex. [0111] |
| Join | Not at the same base | The two forces are stopped, each at a base of the side, but not at the same base: "F is not stopped at the same base". Tried last. [0111] |
| Send to yard | Under way | The force's status is under way, or it has a hex ahead. Out of fuel at a hex centre counts as stopped. |
| Send to yard | No own base here | No base of the issuer's side is in the force's position. |
| Send to yard | Not a port for the yard | A base of the side is in the force's hex but none has a `yard_passage_days`. |
| Prepare a landing | Not your land unit | A unit on the roster does not exist or is not the issuer's. One reason for both. The first such unit in roster order is the one that counts. |
| Prepare a landing | Not a coast hex | The hex is off the map or is not a coast hex: "H is not a coast hex on the map", with the hex. The map is common knowledge. Tried after the roster's owners. |
| Prepare a landing | Land unit destroyed | A unit on the roster has no men ashore and none aboard any ship: "the land unit is destroyed". Tried after the hex. |
| Escort | Not on the escort mission | The force's mission is not escort. [0121] |
| Escort | Is a charge | Another force has this force as its charge, so it cannot be an escort: "the task force is the charge of F, and a charge cannot be an escort", naming that escort. Tried for a charge named and for a clearing order alike. [0121] |
| Escort | Cannot escort itself | The charge named is the force the order is for. [0121] |
| Escort | Not your unit | The charge named does not exist or is not the issuer's. One reason for both. [0121] |
| Escort | No ship afloat | The charge named lists no ships: the reasons of row 3, including the two for a force joined or stood down. [0121] |
| Escort | Charge is an escort | The charge named has a charge of its own: "F has a charge of its own, and an escort cannot be a charge". [0121] |
| Escort | Fuels from, is a giver | The escort is under the order to fuel at sea from F ("the task force is fuelling from F"), or is the force F fuels from ("the task force is the force F fuels from"). [0201] |
| Escort | Charge takes part in fuelling | The charge named is under the order to fuel at sea or is a giver: "F is fuelling from another force or is the force another fuels from". Tried last. [0201] |
| Fuel from | Cannot fuel from itself | The giver named is the force the order is for. [0201] |
| Fuel from | Not your unit, no ship afloat | The giver named does not exist, is not the issuer's or lists no ships: the reasons of row 3. [0201] |
| Fuel from | Giver has no oiler | The giver has no ship afloat whose class fuels others at sea: "F has no ship afloat that fuels others at sea". [0201] |
| Fuel from | Giver is fuelling | The giver is under the order to fuel from another herself. [0201] |
| Fuel from | Giver is an escort | The giver has a charge. [0201] |
| Fuel from | Giver has an escort | Another force has the giver as its charge, naming it. [0201] |
| Fuel from | Is a giver | Another force is under the order to fuel from this one, naming it. [0201] |
| Fuel from | Escort cannot fuel | The force has a charge: "the task force is an escort, keeping station on F". [0201] |
| Fuel from | Has an escort | Another force has this one as its charge, naming it. Tried last. [0201] |
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
| Set strike contact age | No air combat figures | The scenario has none: "this scenario has no air combat figures". Tried first. [0114] |
| Set strike contact age | Strike contact age out of range | Below 0, or above `search.toml: contact_drop_age_pulses` less 1, since an older contact has been dropped: "strike contact age P is not between 0 and M pulses". [0114, 0236] |
| Set strike first waters | Waters off the map | A hex of the list is not on the map: "H of the waters to strike first is off the map". [0190] |
| Set strike first waters | Waters hex twice | A hex is named twice in the list: "H is named twice in the waters to strike first". Tried after "Waters off the map", hex by hex in list order. [0190] |
| Set strike held share | Held share out of range | Below 0 or above 1: "held share S is not between 0 and 1". [0114] |
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

For a submarine order:

| Order | Reason | When |
| --- | --- | --- |
| Station | Not your unit | The boat does not exist or is not the issuer's. One reason for both. [0218] |
| Station | Off the map | The boat is sunk or home: "the submarine is off the map". [0218] |
| Station | Going home | The boat is damaged or is going home: "the submarine is going home". [0218] |
| Station | Waypoint off the map | The hex is not on the map. [0218] |
| Station | Waypoint unreachable | The hex is not one ships can enter, or no sea route joins the boat's hex to it. [0218] |

A boat held apart until a later day is refused before these as **not yet entered**, and a boat whose day has not come as **not yet available** (A unit not yet entered; A day from which a unit may be used). A staff's order for a boat is refused as held where the boat is held, and otherwise as outside the forces limit of its intent unless the intent is a patrol in force that may use her (chapter 8, Patrolling waters with submarines). [0218, rules 3, 12 and 34]

For a base order:

| Order | Reason | When |
| --- | --- | --- |
| Form a task force | Not your base | The base does not exist or is not the issuer's. One reason for both. [0111] |
| Form a task force | No ships to form | The list is empty: "no ships are listed to form a force". [0111] |
| Form a task force | Ship named twice | A ship is listed more than once. The first ship in list order that was already listed is named. [0111] |
| Form a task force | Ship not in port here | A listed ship is not one of the issuer's ships lying in port at this base in no force: "S is not one of your ships in port here in no force". One reason for a ship that does not exist, an enemy's, one in a force, one at another base, one sunk and one away at the yard. The first such ship in list order is named. [0111] |

[0008, 0016, 0017, 0021, 0026, 0027, 0034, 0036, 0044, 0059, 0066, 0090, 0091, 0094, 0095, 0096, 0097, 0099, 0111]

A refused order changes nothing but the log and the order counter. It is kept in the order record. [0008]

The air group's aircraft type and the figures it names are checked when the game is built, not when an order is taken in. [0025]

A side sees: it sees its own acceptances and refusals whole, with the reason, and for an order given inside a day the pulse it was taken in. The enemy sees nothing of an order. In what a player is shown, "not your unit" reads as "not known". [0047, 0050]

### What an accepted order does

An accepted order replaces the standing order of its slot and nothing else. A new route does not cancel a speed order. A new mission does not cancel a search arc. Cargo orders share one slot: Load, Load fuel, Load to a weight, Load with a land unit and Unload replace each other, and a Load, Load fuel or Load to a weight clears the unit a Load with a unit named. A cargo order also records whether it heeds the reserve of a base: it does if the staff issued it and does not if the commander did (chapter 2, Cargo orders). [0111] The four figures of the rules of engagement are replaced together. A posture order is applied when it is taken in: the unit's posture is set and the order is kept as the one that set it, and a tired unit going back to defend spends it (chapter 7, The tired stop attacking). A march order is applied when it is taken in too: it starts a march with 0 days marched, or keeps the days marched if the hex is the one the unit already marches to, or, if it names the unit's own hex, stops a march and records `MarchStopped` (chapter 7, The march order). It is spent when the unit arrives or the march stops. A prepare-landing order is applied when it is taken in too: it sets the force's preparation, or changes it as chapter 7, Preparing a landing, says, and changes nothing if it names the same hex and the same roster as the one standing. It stays until another replaces it or the landing is done, when its unloading is complete. An activity order is applied when it is taken in: the unit's activity is set and the order is kept as the one that set it, and it stays until another replaces it. [0008, 0027, 0036, 0046, 0090, 0094, 0095, 0099]

A standing order stays until another of its slot replaces it. Nothing clears a cargo, speed, mission, readiness, rules, home port, patrol share, rest threshold, reach or arc order when it has been carried out. Two rules do drop the route order of a force: a force that ran dry and takes fuel in port, and a force retiring because of its stragglers. Chapter 2, Refuelling in port and Stragglers, says when. [0008, 0016, 0045]

A task force remembers which order set its route, speed, rules of engagement and readiness. An air group remembers which order set its mission, arc, patrol share, rest threshold and reach. Events cite them as causes. [0008, 0059]

A route order that finds a force between two hexes may turn it round. The event `TaskForceTurnedBack` follows the order's acceptance and cites the order. [0005, 0017]

A side sees: its own turn-back and detach events whole, with the cause. The enemy sees nothing. [0047]

A side sees the same of the events of forming, joining and standing down: its own `TaskForceFormed`, `TaskForceJoined` and `TaskForceStoodDownInPort` whole, with the order as cause, and its own `LoadBelowReserve`, `LoadHeldAtReserve` and `RefuellingHeldAtReserve` whole (chapter 2, Refuelling in port and Cargo orders). The enemy sees none of them: a force it holds as a contact is a contact still, and it is not told what the contact was of. [0111]

A force halted at a marked waypoint records `TaskForceHalted`, and `TaskForceSteamedOn` when it goes on (chapter 2, Halting at a marked waypoint and steaming on). Each is recorded in the movement step of a pulse, citing the route order, the halt, and the bombardment or cargo completion that ended the wait. A side sees its own two events whole, and the enemy sees neither. The side's briefing, `list_forces` and `get_entity` show its own marks (`act_at`, the marks of the route not yet passed), its force's halt (where, since when, what it waits for, and whether it has bombarded), and, in the briefing, yesterday's halts and steamings on with the reason. Nothing of a halt or a mark reaches the enemy. [0096 item 7]

A side sees the same of the yard events: its own `ForceLeftForYard`, with the order as cause, whole. [0069]

A rebase order, when it is taken in, is applied after its acceptance: `AircraftWrittenOff` is recorded if the group had damaged aircraft, then the group's place becomes the new base and `AirGroupRebased` is recorded. Both cite the order. Chapter 5, Rebasing an air group, gives what the move does and what follows the group. A side sees its own `AircraftWrittenOff` and `AirGroupRebased`, whole, and the enemy sees nothing of them. [0091]

### How orders are numbered and sorted

Orders are submitted per side and wait for the next day. At the start of the next day, intake takes the sides in a fixed order, Allied then Japanese. [0008]

Each side's orders are put in order before they are numbered. First come the orders that form a task force, by base number. Then come all the others in order of the unit they are for: task forces, then air groups, then land units, then submarines, each by unit number. That order is the order of the kinds of unit in the code (`EntityRef`), and nothing in a rule depends on it. The forming orders come first so that an order to the force they form, given for the same day, finds the force: the force formed is numbered when its order is taken in and exists for every later order of the day. The sort is stable, so one unit's orders keep their order and a later order to the same unit replaces an earlier one. A staff's order is numbered after the commander's, and so, if it is for the same unit, replaces it. The sort is made where orders are taken in, so a script, a player and a staff are treated alike. [0043, 0111, 0107]

A force formed has a number only once its order is taken in. An order to it for the same day is accepted: the forming order is taken in first, so the force exists for every later order of the day. A side that queues orders through the agent has them checked beside one another (`check_order_queued`, above), so a player or a staff may give a forming order and the first order to the new force on one day. A scenario's own orders are not checked early, so a script may give both on one day. [0111, 0107 As built]

Limit: the agent's session judges each order against the orders queued before it, so an order to the new force that is given before the forming order is refused as "not your unit"; the forming order must come first. [0107 As built]

Each order takes the next order number, in one sequence for both sides in one game, starting at 1. The Allied side's orders on a day are numbered before the Japanese side's. An order a side hands in inside a day is numbered at the start of the pulse it is for, Allied side first and each side's sorted as above. An order a rule gives inside a day takes the next number at the moment it is given. The number appears in the event that accepts or refuses the order and in the causes of what follows. No roll is keyed on an order's number. [0008, 0043, 0055]

A player is shown a side's own count of its orders, 1, 2, 3, in the order they were taken in, and never the log's number. [0055] Outside the simulation core an order's number cannot be read at all: the side's view gives the side's count, and only the umpire's tools read the log's number. [0086]

### Who can issue

The issuer of an order is one of three, and is recorded as the order's origin. The engine treats them alike, with two differences: an order a staff hands in is also checked against the commander's holds and a forces limit and has a reason (chapter 8), and a cargo order the staff gives heeds a base's reserve (chapter 2, Cargo orders). The issuer is not hashed. [0008, 0055, 0107, 0111]

| Issuer | Meaning |
| --- | --- |
| Player | The holder of the seat gave the order by hand. An order a script gives for a scenario, and an order a model gives, are marked player. |
| Staff | A staff, asked once a day before the orders are taken in (chapter 8). Its orders are marked staff and carry the intent they serve; each is checked against the commander's holds and that intent's forces limit and has a reason. Every runner gives each side the supply staff, which hands in nothing for a side with no intent in force; a scenario file may script an order as staff, and such an order is not checked against a hold or a forces limit and does not override. |
| Local commander | A rule acting for the commander on the spot, inside a day, on the standing orders a unit already has. |

Who holds a seat (a person, an agent or the script) is a different fact and is recorded once for a game, not on each order. [0008, 0050]

Today one rule gives an order inside a day: the straggler rule sends a force for its home port with a set route order, issued by the local commander. The order is checked like any other, may be refused, and cites the event that detached the force. A force with no home port, or already at the centre of the home port's hex, gets no order. [0016, 0055]

A rule's choice of a mission target, a break-off or a straggler's fate is not an order. Chapter 5, The strike decision, chapter 4, Breaking off, and chapter 2, Stragglers, give them. [0055]

## Sides, truth and belief

### Truth and belief

There are two sides, Allied (code 1) and Japanese (code 2). Where sides are taken in turn, the Allied side comes first. [0008]

The truth state is the real world: ships, task forces, bases, air groups, land units, arrivals, replacements and the day. A belief state holds one side's contacts, claims, estimates of the enemy's men ashore, reports of submarines and the waters it suspects enemy submarines work. A side's belief is written only from reports (see chapter 3, Belief): a sighting report makes or refreshes a contact, a battle claim is stored, a "nothing there" report drops a contact, and a ground report makes or replaces an estimate. Ageing only removes what reports once said and adds nothing. A contact, a claim or an estimate never holds the true number of an enemy unit or of its men. [0007, 0012, 0028, 0092]

Anything that plays the game is handed a side's view and never the truth. The truth is read only by the runner, the tests, the replay viewer and the hash. [0007, 0053]

### What a side may know

| What | How it is known |
| --- | --- |
| Its own ships, task forces, bases and air groups | Exactly. There is no fog over a side's own units. Supply and fuel stocks of its own bases and ships are shown. A base's fuel is for ships: no air rule reads it. A base's airfield, the work done on it, the damage to it, its defences with their grade and the day's work are shown to its own side, and so are the share of its aircraft a damaged field lets fly and the aircraft it keeps on the ground (chapter 5, A damaged airfield; chapter 7, Units that work and what they build). |
| Its own replacement streams and arrivals | Exactly. |
| Its own supply lines | Exactly: the base, the tons of supply and of fuel a day, the first day, the last day and the side. Nothing of the enemy's: not that he has one, not what it gives. From a seat at a base, only that base's lines, as with a replacement stream. [0196] |
| Its own streams of replacement men, and its units' establishments | Exactly: the base, the men a day, the first day, the last day and the side; each of its own units' establishment, count of men joined and last day fought. Nothing of the enemy's. From a seat at a base, only that base's streams. [0197] |
| Its own units held apart until their day | Exactly: each with its day, its place (a hex, or a base), what it is and whether it waits for its base (A unit not yet entered, above). Nothing of the enemy's: not that one is coming, not its day. They are not on the map, so no enemy sees them until they enter and are sighted. [0195, rules 7, 13 and 15] |
| The next numbers of its own units made in play | Its own side's: the number its next task force and its next air group made in play will take (below). Nothing of the enemy's. [0171] |
| Its own submarines | Exactly (chapter 10): each with her class, hex, station, passage, salvoes left, the last pulse she is held down, her condition and, held apart, the day she comes. From a seat, the area command only. Nothing of the enemy's. [0218, rules 13 and 35] |
| The enemy's submarines | Only as reports of submarines, one for a hex, each with its day and age and possibly out of place; the boat is never shown. And the waters the side was told it suspects enemy boats work, which it was told on a day and which never change in play. [0218, rules 14, 16, 44 and 47] |
| The enemy's forces | Only as contacts: a hex, a count, a kind and an age, each made and refreshed by reports, each possibly wrong. |
| What it believes came of its fights and raids | Its claims. The damage it does to the enemy is truth and is not shown to it. The damage done to its own ships and aircraft is shown to it whole. |
| Its own land units | Exactly: the men ashore and where, the men aboard and in which ships (chapter 7), each unit's skills (chapter 7, What a land unit is good at), and whether it is a working unit, its job and its work (chapter 7, Units that work and what they build), its fieldworks and cover (chapter 7, Cover and fieldworks), its nights of hunger (chapter 7, Nights of hunger), and its condition and fighting share where the figures of decision 0208 are given (chapter 7, Condition and the fighting share). |
| The enemy's land units | Only an estimate of the enemy's men in a hex, from a ground report or a scenario's opening estimate, which may be wrong, and the day it was reported; it is cleared by a report that the side's own men find the hex empty. A fight ashore tells a side that the enemy has men in the hex (chapter 7, The fight ashore, and What a side knows of the enemy ashore). |
| The enemy's bases | Number, name, hex and the holding side, and nothing else, of every base the scenario starts with: no word on whether it has an airfield. This is the map, not intelligence. A base formed in play by a landing or a march, or a base a scenario holds off the map, is on its own side's map only: it is not among the enemy's bases, an order against it is refused as for a base that does not exist, it cannot be bombarded, and the enemy finds it by a ground report or when it is taken (chapter 3, The ground report). A base that changes side (chapter 7) is on both sides' maps from then on, with the side that holds it now, whatever it was before. |
| The map | In full. |
| Reference data (ship classes, aircraft types, figures) | In full; and whose each class and type is, where the scenario names a file of sides (Where figures come from, below). [0173] |
| Its own orders | All of them, accepted or refused, with the causes it may see. |
| Its own command state, commands, staff reasons, notices and interrupts | All of them, whole: its intents, limits, leaves, plans, runs, commitments, holds and interrupts, each command taken or refused with the reason, the reason of each decision its staff made, each notice its staff handed in and each interrupt that fired (chapter 8). The enemy's, none. |
| Its own mission and objectives | Its own only (chapter 6, What a side knows of its progress). |
| What the enemy is scored for | The enemy's objectives as the scenario gives them: each one's kind, the base it names, open or shut, and what it is worth, and for ships out of action the horizon. Not his mission, not his progress and nothing of his state. It is a rule of the game, as the levels are, and no rule of the simulation, staff or trial reads it (chapter 6, What a side sees of the enemy's objectives). Not at a seat at a command. [0183] |
| How many days the game has | The game's days, the same for both sides and for a seat at any command, like the clock. It is the scenario's configuration (its `days`), set once where the game is made and held beside the objectives: not state, never hashed, and the same figure the briefing's "Day 2 of 5" gives. It is not the objectives' horizon (chapter 6), which is the days the result is judged over and can be longer: the Coral Sea has 5 days and a horizon of 30. A world given no length, such as one made in a test, has none. The staff reads it (chapter 8, Taking a place, Ashore). [0194, 1 and 12] |
| What happened | The events it may see, cut as the next section says. |

A side learns of what its forces did when a day is over. [0007, 0034, 0047, 0050, 0055]

Limit: within a fight a side cannot tell one enemy ship or air group from another. It knows that its ship fired and that its ship was fired on, not at which ship or by which. [0065]

A side's own ships, task forces, air groups and land units are shown to it numbered 1, 2, 3 for each kind, in the order of their numbers in the engine, counting every unit it has had (a destroyed land unit too, so a number never changes). Every answer it is given and every handle it writes uses these numbers; the engine keeps the scenario's numbers. Bases keep theirs: they are on the map for both sides, and a base formed by a landing or a march keeps its number too. [0086, 0066]

Units made in play are numbered by side. A task force made by a detach and an air group formed by a landing away from home take the next number of their own side's series, one counter for each side and kind, kept in the state: the Allied side's from 1,000,001 and the Japanese side's from 2,000,001. A scenario's own numbers are below 1,000,000, and the loader refuses any at or above it. A made unit's number, and so the draws keyed on it, depends on its own side's history only: an enemy unit that is merely there changes nothing. A land unit made by division and a base formed by a landing or a march are not in these series: each takes the next free number counted over the units or the bases of both sides (the highest plus one), as chapter 7 says. [0086, 0066]

Limit: there are no claims for aircraft shot down in an air fight except the raid claim of decision 0056. A side sees its own air fight losses and nothing of the enemy's. [0047, 0056]

### What a seat holds of the enemy, on one paper

The tool `intelligence` (decision 0226; agent interface version 52) gathers what the asking seat holds of the enemy on one answer. It takes no argument. It reads the seat's picture (its contacts, claims, estimates of the enemy's men ashore, reports of submarines and suspected waters), the public bases and the side's own events, day 0 among them, and nothing else: it adds no knowledge, makes no estimate, keeps no state and is in no hash, in no saved game and in no event. A watcher has it for the seat watched. The answer names its parts, `parts`, in this order, each left out where it has nothing:

- **seen:** the contacts held, as `list_contacts` gives them, the least age first and of one age the higher number first.
- **lost_sight_of:** each contact dropped and not held again, the latest dropped first: the day, pulse and content of the last report that made or refreshed it (day 0 with no pulse for a contact given at the start and never refreshed), and the day and pulse of the dropping with one of two words, `no_report_since` with the age it was dropped at, or `strike_found_nothing` with the place flown to.
- **claimed:** every claim the picture holds, by day, the latest first, each as the briefing gives it with its pulse; and four totals (ships believed sunk, hits, aircraft shot down, aircraft destroyed on the ground), never a total by kind of ship, and no claim tied to a contact.
- **enemy_has_done:** by day, the latest first, the raids on the side's bases, the shellings of them from the sea, the attacks on its ships and the fights at sea its forces met, as the briefing gives them, naming the side's own force only. A salvo a submarine fired at one of its ships is a fifth list, `submarine_attacks`, read from the side's own events: the ship, the place, the pulse and the event, and nothing of torpedoes, hits or the boat. A fight ashore is not in it.
- **ashore:** the estimates of the enemy's men held, as `list_contacts` gives them; and every hex the side's men found empty, with the day.
- **enemy_bases:** only the enemy's bases of which something is held: an estimate of men there, or aircraft claimed destroyed on the ground in its hex from the air (a claim from the sea lists no base).
- **enemy_submarines:** the reports of submarines held and the waters suspected, as told. A report dropped is not shown.
- **before_the_game:** the contacts and the estimates of men the side was given at the start, as given, whatever has been reported since.

With command levels on, a seat at a task force or a base, and the area seat as it stands, are given **seen**, **claimed** (the claims of the seat's picture), **ashore**, **enemy_bases** (the enemy's only) and **enemy_submarines** from the seat's own picture; **lost_sight_of**, **enemy_has_done** and **before_the_game** are given only at a seat that plays the whole side, the area seat under the easier setting, and the others are named in `not_kept`. When the game is over the answer is still belief only. It writes no sentence of its own: its only words are those of suspected waters, as told. Its test is that two worlds that differ only in what the side cannot know give the same answer byte for byte, and that every number and handle in it is one the seat's other tools give or the side's events hold. [0226]

### Commands and their pictures

This section is about a **command in the chain**, which holds a picture of the enemy. It is not the commander's command of chapter 8 (a change to the command state). [0134]

**The switch.** A scenario turns command levels on with the top-level key `command_levels = true`. The key is optional and off by default, and no scenario built before decision 0134 has it on. It can be turned on only before day 1 and before any event. With it off, no command exists and the only picture is the side's own belief, which this chapter has described so far.

**Commands.** With command levels on, each side has an **area command**, made when the game starts. Each task force of the side that has a ship (or has one away at a yard to rejoin it, chapter 4, Repair) and each base of the side is a command under the area command. A scenario may place a task force or a base under another command of its side with `superior = { task_force = N }` or `superior = { base = N }` on its `[[task_force]]` or `[[base]]` entry (a force or base a command of the side at the start; not itself; named once; no loop; the area command is under none). The superior is loaded and checked and nothing reads it in this step: two levels are what is built. A command whose superior has ceased to be is put under the area command. [0134]

**A picture.** Each command has a picture: contacts, claims, estimates of the enemy's men ashore, reports of submarines and suspected waters, the same things as a side's belief and held the same way. At the start every picture is a copy of the side's belief. A picture is changed only by a report applied to it through `BeliefState::apply_report` and by ageing; every ageing of the side's belief ages every picture. A side's starting contacts and opening estimates reach every picture as they reach the side's. A side still knows its own units exactly. Which pictures a report reaches, and when, is the next paragraph. Each picture numbers its own contacts. [0134, 0135]

**Reports between commands.** The side's own belief gets every report at once, as it always did, and with command levels on it is read by nothing that decides. A scenario that names the report figures (`reports = "../reference/reports.toml"`, optional, with `command_levels` on) has reports travel; with no report figures, every report is applied to every command's picture at once, in the pulse it is made and in command order (the area command, then task forces, then bases, each by number), so every picture holds what the side's belief holds, contact for contact and number for number, as decision 0134 has it. With report figures: [0135]

| The report | The command that holds it first |
| --- | --- |
| A sighting by an air group's search | The group's command |
| A sighting, a claim or "nothing there" from a strike | The command of the groups that flew it |
| A sighting or a claim from a fight at sea | Each task force of the side that fought |
| A base's sighting of a bombarding force | The base |
| A bombarding force's claim | That force |
| The claim of the defending side of a raid | The commands of its units that fired, and for a raid on a base, the base |
| A ground count and "none found" | The base of the side in that hex, else the area command |
| What a scenario starts a side with | Every command; a contact the scenario says one command saw, that command alone, and it is passed to no one (A contact of its own, below) |

A holder that is no longer a command is replaced by the area command. The holder applies the report to its picture in the pulse it is made. It then passes it on, and only its own reports: [0135]

- **Up**, to its superior.
- **Sideways**, to every other command of the side within its radio reach. A force's place is its hex and a base's is its hex; the area command has no place and is reached by passing up, never sideways. A command is within reach when the distance between the two hex centres, in nautical miles, is not more than the sender's reach. A task force's reach is that of its ship with the longest, by the ship's kind; a base's is its own (`reports.toml: radio_reach_nm`).
- **Down**, from the area command, in the pulse it first hears a report, to every command under it but the one that sent it. The area command passes down the reports it holds first too (a ground count away from a base, for one). The area limits of the intents addressed to a command, each read up the tree (chapter 8), limit what it is sent: a report about a hex outside their union is not sent to it; a command with no intent addressed to it, or one with an intent that has no area limit, is sent every report. Nobody but the area command passes on what it was told.

**The draw.** Each passing to each listener arrives in the pulse it is sent, a pulse later, or never, by one draw: one number is taken for the report and the listener, keyed on seed, day, pulse, the report's event and the listener, with `Draw::ReportPassed` for a passing up or sideways and `Draw::ReportPassedDown` for one down (intelligence subsystem; no second entity where the listener is the area command), and compared with the chances of the direction (`reports.toml: up`, `sideways`, `down`, each `on_time` and `late`; the rest is lost). A number is always taken, whatever the chances. A sideways and a down passing of the same report to the same listener take two numbers, one from each stream: the second road is a second chance. [0135]

**When a late report arrives.** A report that arrives a pulse late is applied at the start of the next pulse, before the alert from belief, in the order of the reports' events and then of the listeners. A listener that is no longer a command is dropped. An area command that hears a report late and for the first time passes it down then. A report made at the end of a day (a ground count, "none found") is counted as made in the afternoon, so one that is late arrives in the next night. A command **hears** a report once: a report passed to it by two roads is applied the first time and ignored after; it remembers the reports it heard until they are as old as `search.toml: contact_drop_age_pulses`. A report a picture hears late is applied as chapter 3 says (A sighting report makes or refreshes a contact, and "Nothing there"). [0135]

**A report sent.** Each passing from one sender is an event, "report sent": the report, the sender, and for each listener the direction and whether it arrives on time, a pulse late or never, with the draw's breakdown (nothing is recorded where there is no listener). The report's own event is its cause, and the "report sent" that brought a report to the area command is a cause of the area command's passing down. A side sees its own "report sent" whole and the enemy's not at all (What a side sees of an event, below). [0135]

**Reasons.** A staff reason that names a contact names the command whose picture it is ("contact 1 held by task_force:2") where reports travel, and "contact 1" where they do not. `sim-cli events --why` on an event passed on as a report prints each "report sent" of it, the commands it was sent to and when it arrived. [0135]

**A force formed.** A task force made by a detach starts as a command with a copy of the picture of the force it was detached from. A force formed from ships in port starts with a copy of the area command's. A base formed by a landing starts with a copy of the picture of the command of the landing force. Each is under the area command from the pulse it is made.

**A base formed by a march** (chapter 7, "The supply is put down") is a command of its side from the moment it is formed, under the area command. Its picture starts as a copy of the picture of the command the marching unit belonged to in the hex it left: the lowest-numbered base of its side there, else the area command; where that command is no longer one, the area command's. (Today a march cannot reach "else the area command", because a dump's unit always left a base of its side; the words are kept for a base lost on the day and for later ways of carrying, and are tested only by calling the hook.) It has heard what that picture has heard, every contact in it is one it was told of, and it has no commander. Where reports travel the area command holds the base's state from the moment it is formed, as it stands then, before the supply carried is put into it, and from then on it reports its state as every base command does. With command levels off no command exists and nothing changes. [0239, rules 1 to 4]

**A force gone.** A task force with no ship in it and none away at a yard to rejoin it (joined to another, stood down or sunk) stops being a command at the end of the pulse, and its picture is gone. A command that was under it is under the area command.

**A base that changes side** (chapter 7) stops being a command of the side that lost it and becomes a command of the side that took it, under that side's area command, with a copy of that area command's picture.

**Who belongs to which command.** A ship belongs to its task force. An air group belongs to the task force of the ship it is aboard (a ship at a yard: the force she will rejoin; a ship in port in no force: the base she is at), or to its base. A land unit belongs to the base of its side in its hex, the lowest-numbered if there are two, else to the area command. A force that has ceased to be, a place no longer the side's, or a group that is lost belongs to the area command.

**Who reads which picture.**

| The decision | Reads the picture of |
| --- | --- |
| Whether an air group strikes and at what, and the reason given for a group that launched nothing | The group's command |
| Alert from belief: whether a force is alerted by a fresh contact (chapter 3) | The task force |
| Each staff doctrine (chapter 8) | The command its intent is addressed to: the area command, a base or a task force. A doctrine of the area also reads the side's units as the area command last had them reported (State reports, below) |
| Whether a fight ashore is certain, the estimate of the enemy ashore a land fight reads (chapter 7), and the check before a report that a hex is empty | The base of the side in that hex, or the area command if the side has no base there |
| What an agent or a player is shown: the briefing, the list of contacts, the measuring tool | The area command, and the side's own forces as the area command last had them reported (State reports, below) |

A command's view (`SideView::command`) returns that command's picture only: nothing returns another command's picture or the truth, and what a side knows of its own units does not pass through it.

**State reports.** With report figures loaded, a task force and a base report their own state to the area command, and the area command reads the side's own units through what it has last had reported and not through the truth. This is the second thing a command knows: its picture is what it knows of the enemy, and what it holds is what it knows of its own. Where there are no report figures, or command levels are off, none of this exists and the area command knows every unit of its side exactly. [0136]

1. **When a state report is made.** For every task force and every base command of the side, at the end of the night and at the end of the morning (after the forces that ceased to be in the pulse are ended, chapter 2), and for the afternoon at the end of the day, after the end-of-day phases. A task force that ceased to be makes one last report. The report is a copy of the command's own state at that moment. [0136]
2. **What it holds.** A task force's: its record (where it is and where it is going), its ships (those in it and those away at a yard to rejoin it) and the air groups of its command. A base's: its record (its stocks and airfield), its air groups and the land units at it. Anything no state names (a ship in port in no force, men aboard ship) is read as it is. [0136]
3. **How it travels.** Each report is sent up to the area command only, never sideways and never down. It is drawn once, with the `up` chances of the report figures (`Draw::StateReport`, intelligence subsystem, keyed on the sender): on time it is held at once; late it is held at the next report moment; lost it is never held. A report late or lost is an event the side sees, "state report late" or "state report lost", with its draw's breakdown; one on time is no event. [0136]
4. **What the area command holds.** For each command under it, the latest state report that has arrived, with the moment it is of (the start, or the end of a pulse). A report older than the one held changes nothing. At the start every command is held exactly. A command formed in the game is held as it was formed. A base that changes side is dropped by the side that lost it (and held, as formed, by the side that took it). [0136]
5. **As last reported.** The area command's reading of a unit of a command under it is the truth with the held state written over what it names. A reason of an area doctrine says "(as of the morning of day 1)" after a figure that rests on a held state older than the pulse just ended. [0136]
6. **Who reads it.** The doctrines addressed to the area (keeping a base supplied, neutralise, deny, and the forming of a force's stand or carrier task, chapter 8) and the area command's seat (the commander's lists and briefing). The passages, hours and fuel the engine forecasts for them start from the place and the fuel last reported. A base's doctrine and a task force's doctrine read their own units exactly, and the units of other commands as the side's view gives them, as in decision 0134; what a command knows of its neighbours' forces is left for later. [0136]

**An order that meets a different world.** The engine takes every order against the truth, as it always did. An order from the area, that is the commander's order, or the staff's order under an intent addressed to the area, that the truth refuses where the state held would have accepted it is refused as usual and the refusal adds "; the area command held ..." and the moment the held state is of. Such a refusal is counted apart: it is not a fault of the staff, and decision 0132's condition that no order of the staff is refused is read as "none but for this cause". An order the held state would have refused too is refused as before, with no addition. [0136]

**Radio silence.** A task force's setting, off unless set. It is set by a scenario (the `[[task_force]]` key `radio_silence = true`, loaded only before day 1 and before any event), by the commander's order `set_radio_silence` (on or off), or by the staff at preserve the force (chapter 8, Standing at a hex by night and Operating a carrier force). It has an effect only where reports travel between commands. It does not hold a force from the staff. [0136]

1. **What a silent force sends.** No state report, and none of its own reports up or sideways; it keeps those reports. It hears everything sent to it. [0136]
2. **It breaks silence** in a pulse in which it fights (a surface engagement begun, or a bombardment it fires), is attacked (an air attack on it) or launches a strike or a raid. In that pulse it sends the reports it kept that are younger than a contact may be kept (`search.toml: contact_drop_age_pulses`), oldest first, each as old as it is, and that pulse's state report; the reports it makes later in the pulse are sent as made. In the next pulse it is silent again. [0136]
3. **Turning it off** drops what the force kept. [0136]
4. **What it buys.** Nothing yet: nothing listens to radio, so silence costs a superior his knowledge of the force and gains nothing, and the staff's setting of it is a placeholder for the choice the intelligence layer will make real (decision 0136, rule 19; `docs/design/intelligence.md`). [0136]

**A contact of its own.** With command levels on, each contact in a command's picture is marked as the command's **own** or as one it **was told of**, by the report that last made or refreshed it there: a report the command held first (its own search, its own strike, its own fight, the rows of the table above) makes it its own, and a report sent to it, from above or from a neighbour, makes it one it was told of. A report heard that changes nothing (a later report made or refreshed the contact) leaves the mark as it was; a contact dropped loses it. With no report figures every command gets every report at once, and the commands that held it first mark it their own while the rest were told of it. A scenario's starting contact is one every command was told of, unless its `[[contact]]` entry names the one command that saw it, `seen_by = { task_force = N }` or `seen_by = { base = N }` (a command of the side at the start): it is then in that command's picture alone, as its own, and in the side's own belief, and is sent to no one. With command levels off there is no mark and `seen_by` is read but changes nothing: the contact is the side's. A force formed in the game takes the contacts of the picture it copies as ones it was told of. Only the closing rule of a carrier force reads the mark (chapter 8, Operating a carrier force). [0137 items 5 and 6]

**Commanders.** A command may have a **commander**, named in the scenario by a `[[commander]]` entry: the side, exactly one of `task_force = N`, `base = N` or `area = true`, his `name`, his `initiative` (`cautious`, `steady` or `bold`) and a `note` of what the rating rests on. Every rating is a placeholder, and the loader refuses a rating without its note (an empty note too), a commander for a task force or base the scenario does not give the side, an unknown rating and a second commander for one command, whether command levels are on or off. A commander is held in his command, in the state, only with command levels on; with them off he is not held and not read. A command with no commander named is **steady**. A commander goes with his command: a force that ceases to be, and a base that changes side, take theirs with them; a command formed in the game has none. Changing a commander, relief and refusal are not built. His rating is read by the closing rule of a carrier force and by nothing else (chapter 8). [0137 items 1 to 4]

**Seats and the tools that show who knew what.** A seat may be at the area command (the default), at a task force or at a base (decision 0138, 5). The area seat is shown the area command's picture and its forces as last reported; its easier setting (decision 0119, answer 5) shows the side's own picture and its forces exactly. A seat at a task force or a base, with command levels on only, is shown its command's picture, its own units exactly (numbered from 1 in the seat), every base's public facts and the events that name its own units, and nothing else its command does not hold; it may order its own units only, any other order being refused "not one of your command's units", and it states no intent. The umpire's tools read the truth and change nothing: `sim-cli events --report` gives when each command first held a report and by which road, `--strike` the answer the strike rule gave a group's command at a past pulse, contact by contact, and `--why` on a launch or an alert follows a report only along the road to the command that decided; the export carries each command's picture in each step and every report sent, which the viewer shows beside the truth. The command report counts the staff orders refused because a force was not as the area command held it apart from the rest (`docs/commands.md`). [0138]

**With command levels off** a command's picture is the side's own belief, by one path through the code, so nothing differs: every event, order, chance and hash of a game with them off is what it was. With them on and no report figures, every event, order and reason and every field of the state but the commands is the same as with them off; the daily hashes differ, because they cover the commands and their pictures. With report figures the pictures differ from the side's belief, so a decision that reads a picture can differ, and the log holds the "report sent" events. The side's own belief stays: it is what a side with command levels off plays on. The state reports, what the area command holds and radio silence change the game only with report figures loaded; with none of this on, every event, order, chance and hash of a game is what it was before decision 0136 but for the state's layout. A commander and the mark of a contact of a command's own change the game only where a carrier force's commander is cautious or bold and a fresh contact is its own; with command levels off neither exists. [0134, 0135, 0136, 0137, 0132]

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

The figures these choices use are reference data, the same for both sides, and a side's view hands them over in full: the surface combat, air combat, air wear and fatigue figures among them. [0100] The exception is a figure given by side (Figures by side, below): a side's view shows it the side's own value in both places, and does not tell it the enemy's. A dry run of a side's trial plays the enemy it makes up on the side's own value, so a commander takes the enemy to do as his own navy does. [0213]

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
| Task force formed | Own side. The enemy sees nothing |
| Command taken, command refused, plan drafted, plan state changed, plan amended, staff reason, held by order, staff notice, interrupt fired | Own side. The enemy sees nothing (decision 0107; chapter 8) |
| Report sent | Own side, whole: the report, the sender and for each listener when it arrives. The enemy sees nothing (decision 0135) |
| Task force joined | The force joined into is own. The enemy sees nothing |
| Station kept, charge lost | The escort is own. The enemy sees nothing (decision 0121, 8) |
| Ship fuelled at sea, fuelling broken off, fuel order ended | The force fuelling is own, whole. The enemy sees nothing (decision 0201, 17) |
| Meeting not held | The charge is own. The enemy sees nothing: it is told nothing of a force it did not meet, and the fights of its own force with the escorts are told as any fight is (decision 0121, 8) |
| Task force stood down in port, load below reserve, load held at reserve, refuelling held at reserve | Own force. The enemy sees nothing (decision 0111) |
| Loading gained, loading lost, preparation shared | Own force. The enemy sees nothing of a force's preparation |
| Rehearsal gained, rehearsal lost, left roster | Own land unit. The enemy sees nothing |
| Training day, fight day counted, experience raised | Own land unit. The enemy sees nothing of a unit's experience |
| Unit worked | Own land unit. The enemy sees nothing of a unit's work or its job |
| Airfield built | Own side's base. The enemy sees nothing |
| Airfield damaged | The side that holds the base. The raid or the shelling it cites is cut as for supply destroyed. The raider, and the enemy, see nothing of it |
| Airfield mended | Own land unit. The enemy sees nothing |
| Force left for the yard, force returned from the yard | Own force |
| Fuel burned, ship refuelled, ship rearmed, ship repaired | Own ship |
| Cargo loaded, cargo unloaded | Own ship |
| Ship damaged, system hit, ship sunk | Own ship |
| Aircraft fire roll, aircraft fire fought | Own ship, whole. The enemy sees nothing of either: it is told of a carrier lost to fire as of any sinking (decision 0212) |
| Search flown | Own air group |
| Submarine entered, ordered to a station, steamed, on station, left for home, reached home, sunk | Own submarine. The enemy sees nothing (chapter 10) |
| Submarine reported, submarine report dropped | The side that holds the report. The enemy sees nothing (chapter 10, Reports of submarines) |
| Suspected waters told | The side told. The enemy sees nothing (chapter 10, Suspected waters) |
| Aircraft worn, aircraft ditched, aircraft ditched for want of a deck, aircraft landed elsewhere, aircraft repaired, aircraft arrived, arrival waiting, replacements received | Own air group |
| Stock received | The side of the supply line that gave it, whole, and nobody else. The event cites no cause. [0196] |
| Men joined | The unit's side, whole, and nobody else. The event cites no cause. [0197] |
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
| Nights of hunger changed | Own land unit, whole. The enemy sees nothing (decision 0205) |
| Rations landed | The side whose landing formed the base, whole: the base, the force, the men aboard, the nights and the tons. The enemy sees nothing (decision 0205) |
| A fight's charge | The side charged, whole: the hex, the men at grips, the tons wanted, the tons from ammunition and from supply, and the supplied share. The enemy sees nothing (decision 0209) |
| Ammunition spent in a fight | Own base, whole: the tons asked, paid and left. A base's supply spent in a fight is the supply spent of its own base, with the reason `fight`. The enemy sees nothing (decision 0209) |
| Ammunition landed | The landing's side, whole: the base, the force, the men aboard, the days and the tons. The enemy sees nothing (decision 0209) |
| Fed share restored, land unit fatigue (gained, rested, marched, marched and attacked, strained, landed), land unit back to defend, men lost in a fight, unit fell back, spent unit fell back, unit surrendered | Own land unit |
| Fieldworks changed (dug, worn, lost) | Own land unit, whole. The enemy sees nothing: a unit's fieldworks are not in anything it is told (decision 0106) |
| Defences worn | The side that holds the base, whole: the base, the hex, the hits and the defences after. The side that fired, and the enemy, see nothing of it (decision 0106) |
| March day, march arrived, march stopped | Own land unit. The enemy sees nothing of a march. A march day carries the days counted (decision 0098) |
| Base formed by a march | Own base. The enemy sees nothing |
| Unit divided | The unit it came from is own |
| Men fed | The side whose men were fed |
| Report received, claim received, contact aged, contact dropped, contact not there | The side that holds the contact or made the report. A raid's claim is told whole, with the types of aircraft it claims on the ground and shot down and the units that claim (chapter 3, What a raid's claim names); the enemy never sees them. [0179, 0180, 0210] |

**Whole to both sides**

| Event | Whole to |
| --- | --- |
| World built from a picture | Both sides: the day of the picture's evening. It names no unit and has no cause (How a world begins, A world built from a picture). [0170] |

**Cut of the enemy's name and nothing else** [0065]

Each keeps every other field and its whole breakdown.

| Event | The side sees |
| --- | --- |
| Engagement begun, engagement ended | A side that has come to know of the fight (below): its own force of the two, and the rest. Not the enemy's force. The beginning carries the hour of the round the side first knew of the fight, and the ending's reason is the side's own (below). [0085] |
| Sighting roll | Own force, and only if the roll sighted: the round, the force, the chance and the fatigue factor. Not the force sighted. A roll that did not sight is hidden from both sides: it would say there was something to be found. |
| Force surprised | The force surprised and the round. Not the force that surprised it. |
| Force slipped away | The force that slipped away and the round. Not the force it slipped away from. |
| Alarm roll | Own force, and only if the roll succeeded (alarmed): the force and the chance. Not the forces that fought. A failed alarm means the force noticed nothing. Its breakdown is shown without the draw (below). [0085] |
| Base attacked, for the base's side | The base, the shots, the chance, the expected hits and the hits, on aircraft, on supply and on men; that it was with bombs; and the aircraft's type where the base's people recognised it, or that the type was not made out (chapter 5, What the attacked saw). Not the air group that raided. A hit on the airfield is told in `AirfieldDamaged` and appears here as a hit on nothing. [0167] |

**Cut**

| Event | The firer sees | The target sees |
| --- | --- | --- |
| Salvo | Its own salvo: the round, firer, weapon and shots. Not the target, the chance, the expected hits or the hits scored. | That it was fired on: the round, the target and the weapon. Not the firing ship or the shots. The hits it took reach it as ship damaged. |
| Air attack on a ship | Its own attack: group, weapon and shots. Not the ship or its force, nor whether its aircraft were recognised. | That its ship was attacked: the task force, the ship, the shots (one shot for each aircraft that attacked), the weapon, and the aircraft's type where the ship's people recognised it, or that the type was not made out (chapter 5, What the attacked saw). Not the air group, the chance or the hits. [0167] |
| Air fight | Its own fighters firing: firer, what fired and at what (patrol, escorts or bombers), and shots. Not the group fired at. | Nothing. |
| Anti-aircraft fire (a force's) | Its own force's fire: force and shots. Not the raid's group. | Nothing. |
| Base anti-aircraft fire | Its own base's fire: base and shots. Not the raid's group. | Nothing. |
| Base attacked | The attacker sees that it attacked: group, base and shots. Not what was hit. | The base's side: see the table above. |
| Search attempt | Only if it sighted, its own search: the air group. Not the force sighted, and not the distance or the chance, which are worked from where the force really was, while the report may name a neighbouring hex. An attempt that did not sight is hidden from everyone. [0061, 0065] | Nothing. |

**Cut by the fight ashore** [0090]

| Event | The side sees |
| --- | --- |
| Ground fight | A side that has men in the hex: the hex. Not which side attacked. |
| Ground fire | Its own unit's fire: the unit, the hex, the firing men and the shots (after the limit on engaged men), with its own unit's fed share and, where the figures of decision 0208 are given, its nights of hunger, condition and fighting share in the breakdown. Not the chance or the hits. The enemy sees nothing of it. |
| Ground attack failed | The side that attacked and failed: the hex. Not either weight of fire. The enemy sees nothing of it (chapter 7, An attack that fails) |
| Men surrendered ashore | Both sides: whole to the side whose men they are. To the enemy, cut to the hex and the men, once for the hex and with no unit, so that it cannot count how many units surrendered (chapter 7, A beaten side's men) |
| Base changed side | Both sides: the base, the hex and the two sides. The side that now holds the base is also given the supply and fuel that stayed and those that were lost. The events of a base are seen by the side that held it when they were recorded (chapter 7, A base that changes side). |

**Cut by a submarine's attack** [0218]

| Event | The side sees |
| --- | --- |
| Submarine salvo | Her side: that she fired, where, how many torpedoes and the kind her people took the target for; not the force, the ship, the hits or the chance. The side fired on: that its ship was fired on from a submarine, and where; not the boat, the torpedoes or the hits, which reach it as ship damaged, system hit and ship sunk. [0218, rules 13, 14 and 46] |
| Submarine hunted | Her side: where, until when she is held down and whether she was damaged; not the force. The force's side: that its force hunted a submarine and where; not the boat or the result. |
| Submarine attacked from the air | Her side: where, and whether she was damaged. The group's side: that its group attacked a submarine and where; not the result. |
| Submarine attack chance, submarine search attempt | Neither side. The first names the enemy force; the second is told by the report that follows. |

A claim marked a submarine's is told whole to her side with the other claims (chapter 3, The claim after a submarine's salvo). A submarine sunk is told to her side only.

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

A side sees the whole breakdown, with its result, of an event it sees whole, and of an event cut of the enemy's name and nothing else. Any other cut event carries only the factors that are the side's own, and no result. A salvo keeps `guns`, `tubes`, `rate`, `minutes`, `main_battery_factor`, `unready_gun_share`, `torpedo_mount_factor`, `unready_torpedo_share` and `fire_control_factor`. An air attack and a base attack keep `aircraft`. An air fight and anti-aircraft fire keep `shots`. A search attempt keeps `aircraft`, `sweep_width_nm` and `arc_deg`. A ground fire keeps `men_ashore`, `fed_share`, `fatigue_factor`, `firing_men`, `men_per_shot` and `shots` (and its own unit's `hunger_nights`, `condition` and `fighting_share` where they are given, decision 0208; and, where the limit on engaged men cuts the side's own fire, `shots_before_limit`, `side_firing_men` and `engaged_men`; and its own side's `supplied_share` and `fire_share` where the figures of a fight that costs supply are given, decision 0209), and a ground fight and a base changing side keep none. A bombardment fire keeps `rate`, `guns`, `minutes`, `main_battery_factor`, `rounds_aboard`, `shells`, `gun_damage`, `reference_gun_damage` and `weighed_shots`, and a force's shore bombardment has no breakdown. A ground report is the exception, as a claim is: a side sees it whole and no breakdown of it, because the breakdown holds the true count and the error drawn. A salvo against the side and an air attack against it keep no factor. A side's breakdown never carries a raw random draw: the alarm roll's `draw` is dropped, and the log keeps it for the umpire. [0047, 0061, 0068, 0071, 0085, 0090, 0092]

A claim is the exception. A side sees its own `ClaimReceived` whole, with the types and the units it names (decisions 0179, 0180 and 0210), and no breakdown of it, because the breakdown says how much of the claim is real. The log keeps it for the umpire. [0060]

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
| `air_combat` | `air_combat.toml` | Strike and base strike missions, combat air patrol, escorts, raids, anti-aircraft fire, the false claims of a raid, reach beyond the radius with running out of fuel, the pilots of a patrol shot down over its own hex, and, with `surface_combat.toml`'s bombardment figures, shore bombardment (its table says where a hit lands). The switch `strike_is_spread` spreads the raids at a force over its ships (chapter 5, The raids at a force, taken in turn), and its three keys `aircraft_fire_chance`, `fire_spreads_chance` and `aircraft_fire_levels`, given all or none, give fire among a carrier's aircraft (chapter 5, Fire among a carrier's aircraft; chapter 4); a file without them has neither. Without it the mission orders and the extended reach order are refused. A file that names the scenario must hold every key but `[base_hit] men` and `[base_hit] airfield`, which give 0 if missing: any other missing key is refused when the scenario loads. |
| `air_upkeep` | `air_upkeep.toml` | Wear and repair of aircraft. |
| `fatigue` | `fatigue.toml` | Fatigue of task forces and of air groups, stand-down and return, and the fatigue factor of a sighting. |
| `air_supply` | `air_supply.toml` | Supply for sorties and the daily supply, bases out of supply, and the supply a repair costs. |
| `ground` | `ground.toml` | Men worked as cargo, the men's supply and the loss of the unfed, the fight ashore, fatigue and a side beaten, a base that changes side (chapter 7), the men a hit on a base removes and the ground report (`men_lost_per_base_hit`, `report_error_low` and `report_error_high`; chapter 3, The ground report), marching overland (`march_days_per_hex`, `fatigue_per_march_day` and `march_supply_days`), preparing a landing (`preparation_per_day`, `unprepared_landing_share`, `preparation_kept_on_change` and `unprepared_landing_fatigue`), the experience of a land unit (`experience_fire_strength`, `training_days_per_level`, `training_ceiling` and `fight_days_per_level`) and the skills of a land unit (`assault_skill_strength`, `movement_skill_strength`, `holding_skill_strength` and `weight_strength`; decision 0098). The damage a hit does to an airfield, the work to mend it and the damage at which a field takes no aircraft (`airfield_damage_per_hit`, `work_per_airfield_damage_point` and `airfield_landing_refused_at`) are read from it, as is the fatigue a unit gains when its side is beaten and it falls back (`beaten_fatigue`; chapter 7, A beaten side's men), and a hit lands on an airfield only where it is loaded (chapter 5, The attack on a base). A scenario with a land unit must name it. |

A scenario may also name a **file of sides**, `class_sides`, a hand-written file in `data/reference/` that gives each class of ship and type of aircraft of its reference files the side that uses it: Allied, Japanese or both. The scenario does not load if a class or type of its files has no row, a row names one they do not give, one is given twice, or a ship or air group is of a class or type given to the other side alone. A derived class takes the side of the ships its derivation list gives it; a placeholder class takes both. No rule of play reads it, and a scenario that names none has none. [0173, rules 1 to 4]

Every scenario names its search figures and its surface combat figures. [0021, 0026, 0035, 0036, 0037, 0039, 0066, 0090, 0236]

The five repair figures of decision 0069 are keys of `surface_combat.toml`, so they are always loaded and every one is required: `forward_repair_max_flotation`, `forward_repair_system_points`, `yard_days_per_flotation_point`, `yard_days_per_system_degraded` and `yard_days_per_system_out`. Chapter 4, Repair, gives what each means and what the loader refuses. The frozen step 1 copy holds the same values. The rules do nothing until a base has `ship_repair_points_per_day` or `yard_passage_days` (chapter 2, Bases on hexes). [0069]

### Figures by side

Every figure of a figures file is one number for both sides, with one exception. A figure may be given a value for each side, where a decision record names that figure as given so and a row of a difference in doctrine or practice between the two navies supports it. The first and so far the only one is `air_combat.toml: aircraft_fire_chance` (decision 0212). It keeps the rule one set for every battle: the rules are the same for both sides in every battle, and only the number a side's units read differs. [0213]

- **The table.** In `air_combat.toml`, a table `[[by_side]]` holds one entry for a figure and a side: `figure`, `side` (`allied` or `japanese`), `value` and `rests_on`, what it rests on. The figure keeps its plain value, which a side with no entry reads.
- **What it rests on.** A value by side rests on a row of a difference in doctrine or practice between the navies. It may agree with results across battles, but it is not fitted to one battle, it is never keyed to a scenario, a ship or a place, and it does not stand in for something the model lacks that both sides share. A navy within a side is not told apart: the Allied side is one side, and a British carrier on it would read the American value.
- **The loader refuses** an entry for a figure no decision record names, an unknown side, two entries for one figure and side, an entry for a figure the file does not hold, a value outside the figure's range, a TOML float, and an entry with no `rests_on`. Only the air combat file has the table: the others refuse the key. The frozen step 1 copy has none.
- **It is resolved when the scenario is read.** The loader gives the simulation two numbers for the figure, one for each side, as `BySide`; the simulation reads no file. A rule reads the number of the side whose unit it works on. A scenario has no date and no key for one: a value from a date waits for the campaign.
- **A side is not told the enemy's value.** The figures are common knowledge because they are the same for both sides (What a commander decides and what is decided at the point of contact, above); a figure by side is not. A side's view, `SideView::air_combat_figures`, gives the side's own value in both places, and a dry run of a trial is built on the figures as the side knows them. [0213]

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
| Subsystem | Movement 1, Search 2, Air combat 3, Surface combat 4, Ground combat 5, Logistics 6, Upkeep 7, Intelligence 8, Trial 9 (a trial's own draws in making a picture's enemy, never a rule of play: chapter 9), Submarine 10 (the boat's draws: chapter 10). |
| Entity | The kind and number of the entity together, so ship 3 and task force 3 differ. A trial's draw names a place in one of the side's own lists, counted from 1, as the entity `Place` (kind 9), which no event and no state names (chapter 9). |
| Second entity | Present for a draw about a pair, such as an air group and the force it looks for. The order of the two is fixed by the rule and swapping them gives a different stream. Absent otherwise. |
| Rule | The `Draw` value of the rule that draws. `Legacy` (code 0) adds nothing to the key and is the eleven draws that existed when the tag was added. A new rule takes its own value at the end. |
| Domain | The trial and the world within it, for a world built from a picture of a world: mixed in after every other part, the trial and then the world, each with its own tag, and only where there is one. A live game has none, so its streams are what they were before the part existed. Two worlds of a trial, or of two trials, draw from different streams; one world built twice on one seed and domain draws the same. [0170, rule 12] |

Each part has its own tag so equal values in different parts cannot cancel. A key opens a stream, and the n-th draw of the stream is mixed from the key and n. A rule that needs several numbers for one key takes them from that one stream in a fixed order. No rule shares one stream between entities and no stream is carried from one pulse to the next. [0003, 0013, 0055]

The values of `Draw` are `Legacy` (0), `Claim` (1, the false claims of a raid, one stream for each side's claim keyed on its first firing unit and the raid's first strike group) and `Ditch` (2, the aircraft that run out of fuel and the recovery of their crews, one stream for each air group and pulse, in the upkeep subsystem, keyed on the air group alone) `NoDeck` (4, the crews of aircraft that came down in the sea for want of a deck, one stream for each source air group and pulse, in the upkeep subsystem, keyed on the air group alone) and `PatrolPilot` (3, the pilots of a patrol's aircraft shot down over its own hex, one stream for each firing roll's losses to a patrol group, in the air combat subsystem, keyed on the patrol group and the firing escort group). `TorpedoOtherSide` (5) is the second side's torpedo salvo of a round, one stream for that salvo, in the surface combat subsystem, keyed on the firing ship and the enemy force (chapter 4, Random draws). `GroundFire` (6) is the shots of one land unit in the day's fight ashore, one stream for each unit, in the ground combat subsystem, with no pulse, keyed on the unit alone. `Bombardment` (7) is a ship's shells in a night's shore bombardment, where each hit lands and, on the first firing ship's stream only, the sighting report the bombardment gives: one stream for each firing ship and night, in the surface combat subsystem, keyed on the ship and the base. `GroundReport` (8) is the error of one ground report: one stream for each reporting side, hex and day, in the intelligence subsystem, with no pulse, keyed on the reporting side's lowest-numbered unit in the hex. The claim of a bombardment is a `Claim` (1) stream keyed on the first firing ship and the base, in the intelligence subsystem. `ReportPassed` (9) is the passing of one report to one listener between commands (On time, a pulse late or never): one stream for each report and listener, in the intelligence subsystem, keyed on the report's event and the listener (a task force or a base, as the second entity; the area command has none), at the pulse the report is passed, or none for a report passed in an end-of-day phase (Commands and their pictures); up and sideways only. `ReportPassedDown` (10) is the same for a report the area command sends down, keyed alike on a stream of its own, so that a report sent to one command sideways and down is drawn twice. `Reconnaissance` (12) is the error of a reconnaissance flight's count of the enemy ashore: one stream for each air group, keyed on the day, the morning pulse, the intelligence subsystem and the air group (chapter 3, The ground report and the estimate). `RaidRecognised` (13) is whether the people of a ship attacked from the air, or of a base raided, recognised the aircraft: one stream for each ship or base attacked and pulse, in the air combat subsystem, keyed on the ship or the base alone, drawn once for each attack on it in the order the attacks are recorded (chapter 5, What the attacked saw). `ClaimType` (18) is whether the types of the aircraft a raid claims, destroyed on the ground and shot down, are made out: one draw for each claim with something to name, on the claim's own stream (the intelligence subsystem, the claiming side's first unit to fire, the raid's first strike group), made only where `raid_recognised_chance` is above 0 (chapter 3, What a raid's claim names). `StrikeSpread` (19) is the draw of the first raid of one kind at a force that takes its ship by lot where the strike is spread: one stream in the air combat subsystem keyed on the raid's first strike group and the force (chapter 5, The raids at a force, taken in turn). `AircraftFireSet` (20) is whether a raid's bombs set a carrier's aircraft alight: one stream in the air combat subsystem keyed on the ship and the raid's first strike group (chapter 5, Fire among a carrier's aircraft). `AircraftFireFought` (21) is the fight of a fire among a carrier's aircraft at a pulse's end: one stream in the air combat subsystem keyed on the ship alone, for each pulse (chapter 4, Fire among a carrier's aircraft). `TrialContactHex` (14), `TrialPair` (15), `TrialBelievedHex` (16) and `TrialBelievedBase` (17) are a trial's own draws in making the enemy's half of a picture, in the trial subsystem, with no pulse, on the picture's day, keyed on a place in the side's own list and in the trial's domain; no game draws them (chapter 9, The pictures of a trial). `SubmarineAttack` (22), `SubmarineTorpedo` (23), `SubmarineHunt` (24), `SubmarineSighted` (25), `SubmarineAirAttack` (26), `SubmarineReportError` (27) and `SubmarineClaim` (28) are the boat's draws, in the submarine subsystem, each keyed on the day and the pulse and on the boat and a second entity (a task force, a ship or an air group): chapter 10, Draws. [0055, 0056, 0059, 0083, 0090, 0092, 0135, 0147, 0167, 0174, 0179, 0210, 0211, 0212, 0218]

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
| Bases | The hex is on the map. Supply and fuel stocks are not negative. A replacement stream names a base that exists. A supply line names a base that exists, gives tons that are not negative, gives some supply or fuel, and has a first day of at least 1 and a last day not before it; the fault is `SupplyLineUnsound`. [0196] A base's repair points a day are not negative and its yard passage is not 0. The work done on a base's airfield is not negative and its defences are from 0 to 100 (fire wears them, never below 0). The damage to a base's airfield is from 0 to 100 and is above 0 only where the base has an airfield. [0099, 0103] |
| Land units | A unit's hex is set if and only if it has men ashore, and is on the map. Fatigue is from 0 to 100 and the fed share from 0 to 1. [0090] A unit with men ashore that has a march marches to a hex a land link joins to its hex, and has marched fewer days than `march_days_per_hex`. A unit that lost all its men ashore after the day's marches keeps its march until the next day's, which stops it. [0094] A unit's rehearsal is from 0 to 100 and its hex is set if and only if the rehearsal is above 0; its experience is at most 4; its days of training are below `training_days_per_level` and its days of fighting below `fight_days_per_level`. [0095, 0097] A marching unit's days counted are from 0 up to below `march_days_per_hex`, and none of a unit's eight skills is above 4. [0098, 0099] A unit's fieldworks are from 0 to `fieldworks_limit`, and 0 for a working unit, a unit on attack and a unit with a march standing. [0106] Where establishments are kept, each is for a unit in the state or held apart, and a unit in the state has no more men ashore and aboard than its establishment; the fault is `StrengthUnsound`. A stream of men names a base that exists, gives men above 0, and has a first day of at least 1 and a last day not before it; the fault is `MenStreamUnsound`. [0197] |
| Submarines | A boat's class is in the data; her salvoes are at most her class carries; a boat sunk or home has no passage; a boat not sunk or home is in a hex ships can enter and, on no passage, at her station; her station is a hex ships can enter; she is not held down before day 1; her number is her key and below 1,000,000. The fault is `SubmarineUnsound`. [0218] |
| Air groups | Each is at a base that exists or aboard a ship that exists, of its own side, or has been lost with its base, in which case it has no aircraft. [0090] A group at a base that is not its side's, or that has no airfield, has no ready aircraft, no damaged aircraft and no pilots (chapter 5, A group with no aircraft at a base that is not its side's). [0101] The ready and damaged aircraft of the groups aboard a ship do not exceed the class's aircraft capacity. Fatigue and the rest threshold are from 0 to 100. |

[0004, 0016, 0017, 0025, 0035, 0036, 0037, 0038, 0045, 0046, 0066, 0069, 0090, 0094, 0095, 0097, 0101]

Three invariants hold by construction and are not checked: an air group's ready aircraft cannot be negative; an air group has exactly one location; and a side's belief is written only from reports, but for the belief a picture of a world gives a built world at its start, each entry held as reported by the world's first event. A built world's state and command states are checked when it is built (How a world begins). [0052, 0170]

## How a world begins

### A game from a scenario

A game begins from a scenario, at day 0: the state the scenario gives, with an empty belief and an empty command state for each side, the counters of units made in play at their first numbers, and an empty order book and event log. A scenario's starting contacts, opening estimates, command levels, commanders and radio silence are given before the first day, and each setup call refuses once a day has passed. A saved game holds no state: it is replayed from day 0 (What a saved game holds, below). [0054, 0086, 0100, 0170]

**What a side believed of the enemy.** A scenario may also state, for each side, the enemy forces it believed in and held no contact of (`[[believed_force]]`: a name, the kind of its most important ship, a count of ships, a standing, present, absent or unknown, and where it was last known, an enemy base on the side's map or a hex with its day, or not known), and the intents it believed the enemy held (`[[believed_intent]]`: the kind of a `set_intent` command for the enemy and its keys, checked as that command would be for the enemy's side, with any base it names on the believing side's map), and the men of the enemy's it believed may be ashore at a place beyond its estimate (`[[believed_men]]`: a name, the place, a base of the enemy's on the side's map or a hex of land, the men and a standing, decision 0176). They are the scenario's, not the world's: they are not state and not belief, are never hashed, are put into no world and change no game, and no rule of the game reads them. The loader checks them and keeps them with the scenario, for the constructor of the enemy's half of a picture to be handed with a side's view (chapter 9). No seat is shown them. A scenario that is to be tried may name a trial's figures with its `trial` key (`trial.toml`), kept with the scenario in the same way. [0173, rules 5 to 9; 0174; 0176, 1 and amended 7]

### A scenario built on another

A scenario file may state that it is **built on** another and then hold only what differs: the loader reads the file below, lays this one over it and loads the result as it loads any scenario. No rule of the game is added: only how a scenario is written. [0234]

1. **`built_on`.** A scenario file may state `built_on = "path"`, one path, relative to its own folder. The file named is **the file it is built on**; the file that states the key is **an overlay**. A file with no `built_on` is read exactly as a scenario always was, error for error.
2. **A chain.** The file built on may itself be an overlay. A chain that comes back to a file it has read is refused, and so is one of more than 8 files. A file states one `built_on`, never a list.
3. **What is read.** The loader reads the lowest file's table, lays each file above over it in turn, and gives the result to the one loader of scenarios, with the folder of the file that was asked for. Unknown keys, missing keys, floats and every other check are made on the result, as for a file written out in full.
4. **A plain key** (anything that is not a list of tables: `name`, `days`, `ground`, `ship_classes` whether one name or a list, the `[objectives]` table, the `[side]` table) stated by the overlay takes the place of the whole value below. `[side.allied]` in an overlay therefore drops a `[side.japanese]` below unless the overlay states it too.
5. **A list of units by id** (`[[base]]`, `[[ship]]`, `[[task_force]]`, `[[air_group]]`, `[[land_unit]]`, `[[submarine]]`, `[[formation]]`). A row of the overlay whose `id` the list below has changes that row: each key the row states takes the place of that key's whole value, and the row's other keys and its place in the list stand. A row with a new `id` is put after the rows below, in the overlay's order. A row with no `id`, or two rows of one `id` in one file, are refused.
6. **Any other list of tables** (`[[order]]`, `[[command]]`, `[[contact]]`, `[[estimate]]`, `[[replacement]]`, `[[arrival]]`, `[[supply_line]]`, `[[men_replacement]]`, `[[suspected_waters]]`, the three `[[believed_...]]` lists, `[[commander]]`, `[[island]]`): the overlay's rows are put after the rows below, in its order.
7. **Nothing is taken away.** An overlay cannot remove a row, remove a key of a row, or change the order of what is below.
8. **A file a scenario names** (`map`, `land_links`, `ship_classes`, `port_sizes`, `aircraft_types`, `search`, `surface_combat`, `air_combat`, `air_upkeep`, `fatigue`, `air_supply`, `ground`, `reports`, `class_sides`, `trial`, `submarine_classes`, `submarine_torpedoes`, `submarines`) is read relative to the folder of the file that states it. Where the overlay is in another folder than the file it is built on, a name it takes from below is given the path from the one folder to the other before it, and each plain folder name followed by `..` is then struck out: `../coral-sea/` before `../../map/x.csv` is `../../map/x.csv`. A name the file asked for states itself is used as written. A key that names a file, added to the scenario format later, is added to this list in the same change.
9. **`built_on` is not a reference file.** It is read where it is named, through the same `Source` as every other file, and `LoadOptions::reference_root` does not move it.
10. **The file built on is a whole scenario.** It loads and runs alone. No file is "only a base": whether a loop over `data/scenarios/*.toml` walks a file is decided, as before, by the folder it is in (decision 0072), for an overlay and for the file it is built on alike. An overlay directly in `data/scenarios` is a scenario the loops run; none is made by this record.
11. **What a saved game records.** `Scenario::data_files` ends with the files the scenario is built on, nearest first, after every file listed today. A saved game, a game file, a batch file and an export therefore record the hash of the bytes of the file named, as now, and the hash of the bytes of each file below it, and a saved game is refused when any of them has changed. A pack holds them too, with nothing added to `pack_of`. No version moves: a file with no `built_on` records what it recorded.
12. **The resolved text.** `sim_data::Scenario::resolved_text(source, path)` gives the resolved scenario as TOML: it holds no `built_on`, and every file name in it is written for the folder of the file asked for (item 8), so the text put in that folder, or handed to `from_text` with that path, loads as the overlay does. `sim_data::Scenario::built_on_chain(source, path)` gives the files read, the one asked for first. **`sim-cli resolve FILE`** only prints the first on standard output and the second on standard error. Neither builds a world or reads a file but the chain. It is the umpire's tool, for reading an overlay whole and for the proof of a conversion (`scripts/overlay-proof.sh`). `resolved_text_of` and `built_on_chain_of` do the same for text that is not the file's own bytes, as it would be found at a path. Decision 0207's rule 12 reads a scenario so: the classes it loads are those of the resolved scenario, and the words of the mark may be in the header of the file or of any file it is built on, the headers joined, nearest first.
13. **`from_text`.** Text that states `built_on`, handed to `Scenario::from_text` or `from_text_with`, is laid over the file it names, read relative to the path given, as a file at that path would be. Text with none is read as before.
14. **Where an overlay may not be.** No file in `data/scenarios/step2-frozen`, no file of a proof (`orders-inside-proof`, `second-proof`, `stage-2b-proof`, `stage-3-proof`, `stage-4-proof`, `staff`) and no scenario a recorded game names is an overlay. Any of them may be built on: that reads it and changes nothing in it.

### A world built from a picture

A trial plays a course forward in worlds that are not the live one, and such a world is built from a **picture of a world**: plain data of one evening, the clock at 1800 between two days. It is not state: it is never saved and never hashed, and a world built from it holds nothing that refers to it. [0170, rules 1 and 2]

1. **What a picture of a world holds.** The day (the last day resolved; the first day the world resolves is the day after); the game's days, where the world is to have them, so that the staffs of a world built from it judge the game's last days as the live game's do [0194, 11]; the state of both sides as the engine's own state holds it (ships, task forces, bases, air groups, land units, formations, replacements and arrivals, and what is held apart until its day, each side's entries), whose own day is the picture's; each side's belief: its contacts (place, count, kind and age in pulses), its claims (place, day, pulse and the claim's figures) and its estimates of the enemy's men ashore (place, men, day and grade), none with an event's number; each side's command state, whole, with its next plan's number; the orders in force, as a list, each referred to by its place in it from 1; for each side the next number of a task force and of an air group made in play; and the objectives, where the world is to have them. It holds no event log, no order book of a live game, no daily hash and no seed: its orders and contacts are counted from 1, so a picture made from a side's view says nothing of how many orders or contacts the other side has had. A side, a command or an order book not given is empty, and a side's counters not given are its first numbers. [0170, rules 1 and 2]
2. **An evening only, and command levels off.** A picture of a world is of an evening, never of 0600 or 1200; a trial asked at 0600 or 1200 takes its picture of the copy with the open day treated as closed, so the state of that hour stands for the evening, and the paper says so (chapter 9, A seat's trial, rule 14; decision 0204). Command levels are off in a world built from one: no command's own picture, no reports on their way between commands, no superior's holding of his forces and no radio silence. [0170, rules 3 and 4]
3. **What is refused.** `World::from_picture` refuses a picture that is not sound, naming what is wrong, and builds nothing: the state's day not the picture's; an order referred to by the state or a command state that is not in the list; a task force or air group made in play (numbered above 1,000,000) at or above its side's counter of its kind, or a counter below its side's first number; a command state's next plan not above every plan it holds; a contact with no ship or off the map; a claim or an estimate of a day after the picture's; two estimates for one hex; hits on men waiting to be worked; a unit number that is both in the state and held apart, or held apart twice (a ship, a task force, an air group or a land unit), naming it [0195, rule 20]; whatever a game from a scenario refuses of the reference data and the state (the units held apart read as those on the map do), and the invariants of the state; a command state that breaks the invariants of the day's end (chapter 8); objectives a game would refuse. [0170, rule 6]
4. **The first event is the picture.** The built world's log begins with one event, "world built from a picture", on the picture's day with no pulse and no cause: event 1. The first thing that happens in the world is event 2. Both sides see it whole (What a side sees of an event, below). [0170, rule 7]
5. **What would cite an earlier cause cites the picture.** Every event number the state holds is set to event 1: a force's arrival, what alerted it, its halt and what it did since (its bombardment, its cargo completed), its yard trip, a march begun by an event, the day's hits on men. Every contact, claim and estimate is held as reported by event 1. A force alerted before the picture stays alerted, by a cause the world does not have. [0170, rule 8]
6. **Orders keep their place.** The order book is the picture's list in its order, numbered from 1, each as issued on the day the picture gives it; the next order taken in has the next number. [0170, rule 9]
7. **Counters stand where the picture put them.** The numbers of units made in play are the picture's; each side's contacts are numbered from 1 in the picture's order and its next contact is the one after; each command state's next plan is the picture's. [0170, rule 10]
8. **A world so built has no history.** Its daily hashes begin with its first day. A rule that reads the log of earlier days finds none: an order to a force gone before the picture is refused in the general words, a ship lost before the picture is not counted by an interrupt that counts a day's losses, and yard trips begun before the picture are not in the score. A trial does not score, and none of these moves a fight. [0170, rule 11]
9. **Its own stream.** A built world draws in a domain of its own: the number of a trial and the number of a world within it, given with its seed by whoever builds it (Every draw is keyed, below). [0170, rule 12]
10. **The umpire's rebuild.** With the umpire's key, the whole picture of a live world may be taken (`World::whole_picture`: both sides as they are, belief and all; the orders the state and the command states refer to, renumbered from 1 in the order of their numbers; the contacts renumbered from 1; nothing while a day is open) and a world rebuilt from it on the live game's seed with no domain (`World::rebuild_from_picture`), which draws what the live game draws. It is for the test that the build loses nothing and for the umpire's tools. Nothing that plays the game holds the key, and the agent's library may not name either call. [0170, rule 13]

A side sees: of the picture, only the event that begins its world. What it believes in a built world is the belief the picture gave it.

### A side's own half of a picture

A picture of a world is made of two halves, one for each side. A side's **own half** is what its view gives it whole on an evening, copied from that view and nothing else. Two halves, one of each side, are joined into a picture. [0171]

1. **What a half holds.** The day; the side; the game's days, as the side's view gives them [0194, 11]; its ships, task forces, bases, air groups, land units, formations, replacement streams, arrivals, supply lines and streams of replacement men with its own units' establishments (its own only; in a trial the enemy's made half holds none), its own submarines (the enemy's made half holds none, and a world built from a picture has no enemy boat; the waters a side is yet to be told are not in it), as the engine's own state holds them; the units held apart until their day that are the side's own (its entries, and no entry of the enemy's); its orders in force, as a list, each referred to by its place in it from 1; its command state, whole; its belief as a picture holds one (above, item 1); and its two counters, the next number of a task force and of an air group made in play. It holds no number of a live game's order or event. Its fields are open, so that a half can also be made by rule; a half made so names its orders by their place and can hold no halted force and no force on a yard trip, having no event to cite. [0171, rules 1 and 8]
2. **A side's view gives its own half** (`SideView::own_half`), on an evening. It is refused while a day is open, and at a view cut to a seat or whose units are as a command holds them: only a whole side has a half. [0171, rule 2]
3. **The orders in force** are the orders the side's units and its command state refer to, and no other, in the order of the side's own numbers for them (How orders are numbered and sorted); every reference is re-pointed to the order's place in the list. [0171, rule 3]
4. **Every event a unit cites is the picture's:** the arrival of a force, what alerted it, its halt and what it did since, its yard trip and a march begun by an event cite the first event of a world built from a picture, as the umpire's whole picture has them. [0171, rule 4]
5. **The belief** is the side's contacts in the order of its own numbers for them, its claims in the order held and its estimates in the order of their hexes, each without the event that reported it. [0171, rule 5]
6. **The counters** are the side's own: the next number of a force and of an air group made in play (What a side may know). [0171, rule 6]
7. **Two halves make a picture** (`WorldPicture::from_halves`): the units of both, with the entries of both; the orders of the Allied half and then of the Japanese, each re-pointed to its new place; each side's belief, command state and counters; the game's days, which both halves give; no objectives. The replacement streams and arrivals of both are put in the one order the state keeps them in. The halves may be given in either order. It refuses two halves of one side, halves of different days, halves of games of different lengths, two units of one number (a unit in one half's state and held apart in the other's included), and a half that refers to an order not in its own list. [0171, rule 7]
8. **No peek.** Two worlds that differ only in what a side cannot see and has not been told give it equal halves. The staff and the agent's library may not take or join a half. [0171, rule 9]

The enemy's half of a picture a side is tried in is made by rule from what the side holds and believes: chapter 9, How a picture's enemy is made. [0174]

## The state hash and saved games

### The state hash

At the end of each day the engine computes one 64-bit hash of the state and appends it to the daily hashes. It is FNV-1a over a fixed-order walk of the state, begun by the version string `flagplot-state-v56`. Version 56 adds a ship's `aircraft_fire` (0 for none, else 1, its level, the day it was set, the pulse's code and its record) after her `available_from` (decision 0212). Version 55 adds to a side's command state, after its other fields, the leave to pursue of each order (the count, then each intent's number and the setting's code), each force's pursuit (the count, then the force, the way's code with the answer's event for an answer, the contact and the day) and the commander's refusals (the count, then each force and contact) (decision 0214). Version 54 adds a claim's `air_types` (their count, then each type and its number) after its `by_unit`, in a side's belief, a command's picture and a report in flight alike (decision 0210). Versions 52 and 53 are listed with the version string in the engine's hash code. Version 51 adds to an intent's limits, after "knowledge", "targets" (0 for none, else 1, the count of kinds, each kind's code and `waters_first`), and the code 11 for it to a limit set by a command, and, to an air group, its `strike_first_waters` (the count, then each hex) and `waters_order` after its `claims` (decision 0190). Version 50 adds the standing interrupt kind `lift_turned_back` (code 5) to a side's command state, with no new field (decision 0191). Version 49 adds a claim's `ground_types` (their count, then each type and its number) and `by_unit` (their count, then each unit by its kind code and number, with its aircraft shot down, aircraft on the ground and hits) after its `from_bombardment`, in a side's belief, a command's picture and a report in flight alike, and an air group's `claims` (aircraft shot down, on the ground, hits) after its `available_from` (decisions 0179 and 0180). Versions 45 to 47 (decision 0159) and 48 (decision 0162) are listed with the version string in the engine's hash code; chapter 8, What is saved, tells 48. Version 44 adds the intent kind control an island (code 15: its island's number, then its date as an optional number) (decision 0151). Version 43 added an estimate's grade (a byte: 1 told, 2 seen from the air, 3 men in contact) after its event, the same in a ground report in flight, and the air mission reconnaissance (code 6) followed by its hex's q and r (decision 0147). An air group's strike sizing (a byte, 1 for on) and the order that set it are written last in the group, after its held share's order; version 30 added them with the intent kinds deny an area (code 6: its hex's q and r, then its distance), strike ships (code 7) and search waters (code 8), each of those two with its base (decisions 0118 and 0123). Version 31 added a route's marks to stand at (their count, then each hex in order) after its marks to act at, a halt's work `night` (a byte) after its cargo, the addressee task force (code 4, its number) and the intent kinds stand at a hex (code 9) and operate a carrier force (code 10), each with its force's number and then its hex's q and r (decisions 0124 and 0125). Version 32 added a task force's charge, written after its `landing_at`: a flag, then if there is a charge the force it keeps station on and the order that gave it (decision 0121). Version 33 added a ship's `torpedoes_fired` (the tubes that have fired and not been loaded again, a number), written straight after her `torpedoes_one_side_fired`, and a base's `ammunition_t`, written after its `fuel_reserve_t` (decision 0122). A task force's `leg_nm` is written right after its `progress_nm`; version 3 added it. An air group's reach code and the order that set it are written last, after its arc order; version 4 added them. An air group's target list (its count, then each kind's code in list order), the order that set it and the game day of its last base strike (written as an optional number) are written after that; version 5 added them. A ship's experience is written after its location; version 6 added it. A ship's `torpedoes_one_side_fired` is written straight after her `torpedoes_ready`; version 7 added it. Version 8 replaced the one counter of task forces made in play with a counter for each side and each kind of unit made in play, written Allied then Japanese, task forces then air groups. Version 9 added a base's `ship_repair_points_per_day` and `yard_passage_days` after its `fuel_t`, a ship's `system_repair_points` after her `experience`, the ship location `Yard` (code 4) and a task force's `yard_trip` after its `fatigue`. Version 10 added a ship's `troops` (the unit and the men, or none) after her `system_repair_points`, a base's `on_map` after its `yard_passage_days`, a cargo order's land unit (an optional number) after its order, the cargo action `LoadNoCargo` (code 4), and the land units (number, side, men ashore and hex, not the name) after the arrivals. Version 11 added an air group's `rebased_day` (an optional number), written after its last base strike day. Version 12 added a land unit's posture (code 1 for defend, 2 for attack), the order that set it (an optional number), its fatigue and its fed share, written after its hex, and the air group location `Lost` (code 3). Version 13 added a claim's `from_bombardment`, written after its `from_air`, and a side's ground estimates (the count, then for each, in hex order, the hex, the men, the day and the report's event) after its claims. Version 14 added a land unit's march after its fed share: a flag, then if there is a march the hex it marches to, the days marched and the march order. Version 15 added a route's marks after its next index (their count, then each hex in order), the task force status halted (code 5), and a task force's halt after its yard trip. Version 16 added a task force's preparation after its halt (a flag, then if there is one the hex, the roster's count and each unit in order, the loading and the order) and its `landing_at` (an optional hex), and a land unit's rehearsal, the hex of the rehearsal (optional), the activity (a code), the order that set it (optional), the experience and the days of training and of fighting, written after its march. Version 17 added a march's `counted` (the days counted, a fixed-point number) after its days marched, and a land unit's seven skills (assault, holding, weight, clear, jungle, mountain, atoll, in that order) after its days of fighting. Version 18 added a base's `airfield`, `airfield_work`, `defences` and the day's work (the day, the port work and the aircraft work, or none) after its `on_map`, the activity `Work` (code 2) with its job after the activity's code, a skill's `works` after its `atoll`, and a land unit's `working` mark after its skills. Version 19 added a base's `airfield_damage` after its day's work, and an arrival's `waiting` mark after its pilots. Version 20 made a march's order its cause: after the days counted, a code (1 for an order, 2 for the event of a falling back) and the number of the order or of the event. Version 21 added a land unit's instruction for when its attack is spent (`when_spent`: 0 for none, 1 hold, 2 fall back), written after its working mark. Version 22 added a land unit's `fieldworks` (a fixed-point number), written after its `when_spent`, and the day's hits on men, written after the land units: their count, then for each, in hex order and then side order, the hex, the side, the hits and the events of the fire that made them (their count, then each in order). The day's hits are empty at the end of every ground combat phase, so they are empty whenever the state is hashed, and the version names the layout all the same. Version 23 added a cargo order's weight, written after its unit (a flag, then if there is a weight the weight, a fixed-point number), and the flag that the order heeds a base's reserve, written after the weight, and a base's two reserves, written after its `airfield_damage`: the supply reserve, then the fuel reserve. Version 24 added each side's command state, written after the beliefs, Allied then Japanese, each side's led by its side code: its intents in number order (the number, the kind, the limits, the day set, the plan and the day withdrawn, as an optional number), the two leaves, its plans in number order (the number, the intent, the state's code, the baseline as a flag and then the day, kind and limits, the amendments, and the runs), its commitments by ship, and the next plan number. Version 25 added each side's holds, written after its commitments and before the next plan number (their count, then each in target order: the target's code, 1 for a ship, 2 for a task force and 3 for a setting of an air group, then its number, and for a setting the group's number and the setting's code, 1 for the search arc; the day; and what made it, 1 for a command, or 2 and the order's number), and a run's day of ending, written after its free day as an optional number. Version 26 added each side's interrupts, written after its holds and before the next plan number: the count of the interrupts set, then each in number order (the number, the condition's code, 1 for a base's days of supply below a number, then the base and the number of days, the day set, and its status: armed as 1 or 0, the day it last fired as an optional number, acknowledged as 1 or 0); then the count of standing records, then each in kind then subject order (the kind's code, 1 for cannot meet, 2 for waits for leave and 3 for ship lost, the subject's code, 1 for a plan and 2 for a ship, its number, and a status as above); then the count of standing kinds switched off, then each kind's code. Version 27 added an air group's strike contact age (an optional number) and the order that set it (an optional number), then its held share (a fixed-point number) and the order that set it (an optional number), written after its `rebased_day`. Version 35 added whether command levels are on and each side's commands (decision 0134). Version 36 added, after each command's picture, the reports it has heard (count, then each in the order of the reports' events: the event, the day and the pulse's code it was made), and, after the commands, the reports in flight in the order they were sent (count, then each: the side, the report, its event, the day and pulse it was made, the sender, the direction, the listener, the "report sent" event and the day and pulse it arrives) (decision 0135). Version 37 added what the area command holds of each command under it (the moment it is of, and the record, ships, air groups and land units held), the state reports in flight, the task forces on radio silence and the reports a silent force keeps, and the "state report late" and "state report lost" events (decision 0136). Version 38 added, after each command's reports heard, the contacts of its picture that are its own (count, then each number in order) and its commander (a byte, 0 for none, else 1, then the name's length and bytes, the rating's code, 1 cautious, 2 steady, 3 bold, and the note's length and bytes) (decision 0137). Version 39 added the day from which a unit may be used (an optional number) last in a ship, after her troops, in a task force, after its charge, in an air group, after its search once order, and in a land unit, after its fieldworks, then the land unit's formation (an optional number); and, after the day's hits on men, the formations (their count, then each in number order: the number, the side and the parent as an optional number; not the name) (decision 0140). Version 40 added the intent kinds ready a landing (code 11: its force's number, its hex's q and r, then its date as an optional number) and build up a base (code 12: its base, then the count of its wants and each want's code in order, 1 an airfield, 2 its damage mended, 3 defences followed by the points, 4 the port, 5 aircraft repair), a forces limit's land units (their count, then each number in order) after its source, and a hold's target land unit (code 5, its number) (decision 0141). Version 41 added the intent kind take a place (code 13: its hex's q and r, then its date as an optional number) (decision 0142). Version 42 added the intent kind bombard a base (code 14: its force's number, then its target's) (decision 0146). [0004, 0057, 0058 item 7, 0059, 0063, 0064, 0066, 0069, 0083, 0086, 0090, 0091, 0092, 0094, 0095, 0096, 0097, 0098, 0099, 0103, 0104, 0105, 0106, 0107, 0111, 0114, 0122, 0134, 0135, 0140, 0141, 0146, 0151]

A world built from a picture of a world writes its domain last, after the count of events: the trial and then the world. Then, only where something is held apart until its day (decision 0195, rules 23 and 24), come the entries: their count, then each entry in key order (the key's kind code and number, the day, the side's code, the place, whether it waits, its force, its ships, air groups and land units, its superior, its commander and radio silence, each as the state's own walkers write them). A game with nothing held apart writes nothing there, so its hash is what it was and the version was not raised for it. [0195, rule 23] A world with no domain, which is every live game and the umpire's rebuild, writes nothing there, so the walk of a live game is what it was and the version was not raised for it. [0170, rule 12]

The hash covers: the seed; the day; every ship, task force, base, air group, land unit, arrival and replacement stream; every supply line, hashed last and only where there is one (decision 0196; the version is not raised, since no game without a line is changed); the streams of replacement men and each unit's establishment, count of men joined and last day fought, hashed after the supply lines and only where a scenario gives a stream or an establishment (decision 0197; the version is not raised either); the submarines, the reports of submarines and the suspected waters each belief and each command's picture holds, the places of the claims marked a submarine's, and the suspected waters still waiting to be told, hashed last of all and only where the game has a boat, waters waiting or one of those held (decision 0218; the version is not raised either, and a game with none writes nothing there); both sides' contacts, claims and ground estimates; both sides' command state (its holds and interrupts included); whether command levels are on and, if so, each side's commands, each in command order with its code, its number, its superior and its picture (version 35, decision 0134), what each has heard, and the reports in flight (version 36, decision 0135); the count of orders and the next order number and next task force number; and the count of events. It does not cover the names of ships, task forces, bases, air groups or land units, the contents of the events or orders, the map, the reference data, the orders, commands and staff hand-in waiting for the next day, the orders handed in inside a day waiting for the next pulse (none waits when the hash is taken), what the staff handed in on the last day or the objectives. No tag byte stands in for a field that is not in use. Every field of every unit is written, always, in one order. A field added to the state is added to the hash in the same change, and the version is raised when what is hashed or its order changes. [0004, 0055, 0057]

Two runs of the same scenario, orders and seed give the same hash on any machine. The same hash is given with or without breakdown recording, in two processes, and with the scenario's tables listed in a different order. [0043]

Shuffling means the order the data and the orders are given in, not the order the rules work in. The rules' own order is part of the rules: losses come off the lowest-numbered group first, forces move in order of clock time, a base repairs the lowest-numbered group first. [0043]

### What a saved game holds

A saved game is a file with: the scenario file's name and its hash; each data file the scenario names with its hash; the seed; who held each seat (player or script); the orders of each day for both sides as they were handed in, with their unit, order and origin; the commander's commands of each day, as they were handed in; what each side's staff handed in on each day, whole (its orders with the intent and the reason of each, its staff commands and its decisions to give no order, each with its reason, and its notices, each with the reason it rests on), only for the days and sides that handed in anything; and the daily hashes of the run. It holds no state. The starting state is the scenario's. An order a side gave inside a day is saved with its pulse, `morning` for one given at 0600 and `afternoon` for one given at 1200; an order of dusk has none. The orders a rule gave inside a day are not saved, because the rules give them again. A staff's orders are kept in what the staff handed in and not among the orders. [0043, 0050, 0107]

A saved game also records what it was played under: the version of the state's layout (the version string of the state hash) and the rules label, a number that began at 1 with decision 0100 and that a change raises when it changes a rule in a way that changes what an existing game would do. This is the saved game's version 2. Decision 0107 makes it version 3: a saved game also records the doctrine version, the version of the doctrine whose staff was passed in (chapter 8, What is saved), beside the commands and the staff's hand-in. Decisions 0115 and 0117 make it version 4: its commands and hand-ins may hold the intent tree (the kinds hold and cover, the staff's `form_intent`, air groups in a forces limit, a hold on a whole air group) and each reason's chain. A saved game of version 3, 2 or 1 is refused, not converted. Decision 0120 makes it version 5 and decisions 0118 and 0123 version 6: its commands and hand-ins may hold the kinds deny, strike ships and search waters, the order `set_strike_sizing` and a reason decided at an air group's level; a saved game of version 5 or before is refused. Decisions 0124 and 0125 make it version 7: its commands and hand-ins may hold the kinds stand at a hex and operate a carrier force (each with its task force and its hex), a reason decided at a task force's level and a route order's marks to stand at; a saved game of version 6 or before is refused. Decision 0121 makes it version 8: its hand-ins may hold the escort order, with its charge or none; a saved game of version 7 or before is refused. Decision 0131 makes it version 9: its hand-ins and a commander's orders may hold the order `set_search_once`; a saved game of version 8 or before is refused. The label is 18 since decision 0133 (a contact reported this pulse is refreshed again in it only from within `search.toml: merge_distance_same_pulse_hexes`, which changes every live game with two reports two hexes apart in one pulse, chapter 3, A sighting report makes or refreshes a contact; the state's layout and the saved game's version are as they were), was 17 since decision 0131 (a search flown once a day, and a search's own fatigue figure, which changes every live game in which a group searches with the fatigue figures loaded, chapter 5, Fatigue gained and lost; the state's layout is `flagplot-state-v34` since it), was 16 since decision 0122 (rearming in port: a base's ammunition and a ship's tubes loaded again, chapter 2, Rearming in port; the state's layout is `flagplot-state-v33` since it; the saved game's version is as it was, 8, and the doctrine version is `supply-13` since decision 0127, which changed neither the state nor the label), was 15 since decision 0121 (the escort, chapter 2, Keeping station: a task force's charge, which a game saved before could not hold; the state's layout is `flagplot-state-v32` since it; decision 0126 left both as they were), was 14 from decision 0124 (the night stand, chapter 2, Halting at a marked waypoint and steaming on) and was 13 since decision 0118 (a strike sized to its target). The label is 2 since decision 0103, which added a share to the hit table of a base and so changed what scenario 24 does, 3 since decision 0104, which makes a beaten side's men fall back or surrender and so changes what scenarios 23 and 30 do, 4 since decision 0105, which limits the fire of the men who cannot come to grips and makes a failed attack tire its men, and so changes every live fight ashore, 5 since decision 0106, which cuts the men a defender and a shelled unit lose by cover, tires men under fire and wears cover, and so changes every live scenario with men ashore under fire or in a fight, 6 since decision 0111, which adds a load to a weight, the reserves of a base and the orders that form, join and stand down a force: a game played under the old rules could not hold them, 7 since decision 0109, which takes a unit falling back out of a fight its own side starts, 8 since the architect's ruling on decision 0106, which cuts a unit's loss to fire to a whole man once and so changes every live scenario with men under fire behind cover, 9 since decision 0110, which has a side's fighting men report a hex the enemy has left and so changes every live scenario where a side holds an estimate of such a hex, and 10 since step 3 of decision 0107, which has a commander's order to a unit inside an intent's forces limit hold it and the staff's orders to what is held refused: a game saved under the old rules in which the commander ordered such a unit could play differently, and 11 since decision 0112, which makes refuelling in port take hours in the loading phase of each pulse and so changes every live scenario with a force in port short of fuel. Step 4 of decision 0107 leaves the label at 10: an interrupt and a staff's notice change no order, unit, stock or chance, so no game saved under label 10 plays differently for them (chapter 8, What is saved). The state's layout is `flagplot-state-v26` since it, and a game saved under an earlier layout is refused for that. The label is 12 since decision 0114, which adds the strike contact age and the held share of an air group and the orders that set them: a game saved under the old rules could not hold them. The state's layout is `flagplot-state-v27` since it. Decision 0134 makes the state's layout `flagplot-state-v35` (the commands and their pictures, above) and the saved game's version 10, with the same keys and the rules label unchanged: a game saved under an earlier layout is refused, not converted. Decision 0135 makes the state's layout `flagplot-state-v36` (the reports each command has heard and the reports in flight, above) and the saved game's version 11, with the same keys and the rules label unchanged (18): a game saved under an earlier layout or version is refused, not converted. Decision 0136 makes the state's layout `flagplot-state-v37` (what the area command holds, the state reports in flight and radio silence, above) and the saved game's version 12: its orders may include `set_radio_silence`, and the keys of the file are unchanged; a game saved under an earlier layout or version is refused, not converted. Decision 0137 makes the state's layout `flagplot-state-v38` (the commanders and the mark of a contact of a command's own, above) and the saved game's version 13, with the same keys and the rules label unchanged (18): no game without a commander plays differently. Decision 0147 makes the state's layout `flagplot-state-v43` (the grade of an estimate and the air mission reconnaissance, above) and the saved game's version 18: its orders may hold `set_air_mission` with the mission `reconnaissance` and a `hex`; the rules label is 19: a game with the two reconnaissance figures and the mission plays differently. A game saved under an earlier layout or version is refused, not converted. Decision 0140 makes the state's layout `flagplot-state-v39` (a unit's day and the formations, above) and the saved game's version 14, with the same keys and the rules label unchanged (18): no game without a day plays differently. A game saved under an earlier layout or version is refused, not converted. Decision 0141 makes the state's layout `flagplot-state-v40` (the kinds ready a landing and build up a base, a forces limit's land units and a land unit as a hold's target, above) and the saved game's version 15: its commands and hand-ins may hold the two kinds, a forces limit's `land_units`, a hold's `land_unit` and the notices `ready`, `late` and `met`; the rules label is unchanged (18): no game without either task plays differently. Decision 0142 makes the state's layout `flagplot-state-v41` (the kind take a place, with its hex and its date, above) and the saved game's version 16: its commands may hold the kind `take_place` and the staff's `set_limit` of forces; the rules label is unchanged (18): no game without the kind plays differently. Decision 0146 makes the state's layout `flagplot-state-v42` (the kind bombard a base, with its force and its target, above) and the saved game's version 17: its commands and hand-ins may hold the kind `bombard`; the rules label is unchanged (18): no game without the kind plays differently. Decision 0151 makes the state's layout `flagplot-state-v44` (the intent kind control an island, with its island and its date, above) and the saved game's version 19: its commands and hand-ins may hold the kind `control_island` and the staff's `withdraw_intent`; the rules label is unchanged (19): no game without the kind plays differently. Decision 0156 makes the saved game's version 20: an order given inside a day is saved with its pulse (What a saved game holds, above); the state's layout (`flagplot-state-v44`) and the rules label (19) are unchanged: no game without an order inside a day plays differently. Decision 0159, rule B, makes the state's layout `flagplot-state-v45` (the standing interrupt kind `cannot_bring_enough`, code 4, in a side's standing records and the kinds switched off); the saved game's version (20) and the rules label (19) are unchanged: the file's keys are as they were and no game in which a take is never short of men plays differently. Decision 0159, rule C, makes the state's layout `flagplot-state-v46` (three limits on an intent after its risk, "not before", "wait for" and "after", and the codes 7, 8 and 9 of a limit an amendment records) and the saved game's version 21: its commands may hold the limits `not_before` (`from_day`), `wait_for` (`task_force` or `land_unit`) and `after` (`other_intent`); the rules label is unchanged (19): no game without such a limit plays differently. A game saved under an earlier layout or version is refused, not converted. Decision 0159, rule D, makes the state's layout `flagplot-state-v47` (the limit "knowledge" on an intent after its "after", and the code 10 of a limit an amendment records) and the saved game's version 22: its commands may hold the limit `knowledge` (`knowledge` and, for a recent count, `within_days`); the rules label is unchanged (19): no game without such a limit plays differently. A game saved under an earlier layout or version is refused, not converted. Decision 0162 makes the state's layout `flagplot-state-v48` (a cargo order's level, `each_t`, hashed after `heeds_reserve`: a byte, 0 for none, else 1 and the tons) and the saved game's version 23, the rules label 20 (a supply run by night now spreads its load over its ships, which changes every live game with a run that the night's limit cut, chapter 8, A run by night) and the agent interface version 27 (`each_t` in the load order, and the refusal `LevelNotAboveZero`). A game saved under an earlier layout or version is refused, not converted. Decision 0190 makes the state's layout `flagplot-state-v51` (the targets limit and an air group's waters to strike first, above) and the rules label 23 (a game with the limit or the order, which a game saved under the old rules could not hold, and the choice of contact that reads the waters); the saved game's version stays 23, and it writes the limit and the order. A game saved under an earlier layout is refused, not converted. Decisions 0211 and 0212 make the state's layout `flagplot-state-v56` (a ship's fire among her aircraft, above) and the rules label 28 (a strike spread over the ships of its kind and the fire among a carrier's aircraft change every game in which raids attack a force or a carrier is bombed); the saved game's version is as it was. Decision 0218, stage 2, makes the rules label 29 (a boat as a unit of her own kind, with her state, which a game saved under the old rules could not hold) and the agent interface version 47; the state's layout stays `flagplot-state-v56`, because a game with no boat, no waters and no report of a submarine hashes as it did, and the saved game's version stays as it was: a station order is saved as any order is, with the unit `submarine:N`. Stage 3 makes the rules label 30 (the patrol order, a hold on a boat and a forces limit that names boats, which a game saved under the old rules could not hold), the agent interface version 48 and the doctrine version `supply-55`; the layout and the saved game's version are as they were (chapter 8, What is saved). Decision 0222, rule 1, raises the doctrine version to `supply-56` (the words of a hold's reason with no source) and nothing else. Decision 0223, stages 0 and A, makes the state's layout `flagplot-state-v58` (a side's submarine doctrine, in its command state), the saved game's version 26 (the file's keys are unchanged), the rules label 32 (the doctrine, which a game saved under the old rules could not hold, and the pattern of a patrol, which changes the stations of a side given one), the agent interface version 51 (the briefing's `submarines.doctrine`) and the doctrine version `supply-58`; no game that states no doctrine plays differently. Decision 0228 makes the state's layout `flagplot-state-v59` (whether a side's staff chooses its sources of supply, a key its scenario states), the saved game's version 27 (the file's keys are unchanged), the agent interface version 53 (`supplied_from` in `command_state`) and the doctrine version `supply-59`; the rules label stays 32, since no game on file states the key, so no game on file plays differently. Decision 0239 makes the rules label 36 (with command levels on a base formed by a march is a command); the state's layout is unchanged, and no game with command levels off plays differently. A game saved under an earlier layout or version is refused, not converted. [0100, 0103, 0104, 0105, 0106, 0107, 0109, 0110, 0111, 0112, 0114, 0121, 0124, 0125, 0131, 0133, 0134, 0135, 0136, 0137, 0140, 0141, 0142, 0146, 0151, 0156, 0159, 0162, 0190, 0211, 0212, 0218, 0223, 0228, 0239]

A saved game made before seats existed is read as both seats held by the script. [0050]

A batch summary and a replay file for the viewer carry `inputs`: the version of the hash layout, the hash of the scenario file, and each data file the scenario names with its hash. This lets a reader see that two files were made from different inputs. `batch --label` adds the caller's own name for the engine that ran, since the engine does not know its own revision. [0057]

### Replay

`replay`, and every command that reads a saved game, first refuses a saved game whose state layout or rules label differs from the engine's, naming both, and a saved game of an earlier version than 3, made before the rules label existed (version 1) or before commands, the staff's hand-in and the doctrine version were kept (version 2), which is not converted. Then `replay` loads the scenario the saved game names, refuses a scenario file or data file whose hash has changed or whose list of data files has changed, hands in the saved orders (an order given inside a day at its pulse), the saved commands and each day's saved staff hand-in (its notices among it) day by day with no player present and no staff asked, and compares each day's hash with the saved one. It fails on the first day that differs. The doctrine version is not checked: a replay is what was saved. `replay --check-staff` also asks the doctrine's staff on each saved day and fails on the first day and side where it gives something other than what was saved (chapter 8, What is saved). [0043, 0050, 0107]

A replay must reproduce every daily hash. [0043]

Limit: a saved game does not survive a change of rules. A change that raises the rules label is refused before the replay runs; a change that should have raised it and did not shows as a daily hash that does not match. A game in progress is not carried over a change of rules. [0055, 0100]

Limit: a game cannot be saved in the middle and resumed. It is replayed from the start. A game kept by `play --game` is rebuilt from its calls each time, and is resumed only under the doctrine version it was begun under (chapter 8, What is saved). [0043, 0057, 0107]

A side sees: none of this. A saved game, the hashes and the seed belong to the umpire.

## Not in this chapter

- How forces move, burn fuel, refuel, and load and unload cargo, what becomes of stragglers, and how a force is formed, joined and stood down: chapter 2.
- Air search, sighting reports, contacts, their ageing, readiness and alertness, the alarm, fatigue of forces, claims, and the ground report and the estimate: chapter 3.
- Surface fights, the hit mechanism, damage to ships and shore bombardment: chapter 4.
- Land units, troop capacity, landings, bases formed by a landing or a march, the men's supply, marching overland, preparing a landing, the day's activity, experience and training: chapter 7.
- Air groups, strikes, escorts, patrols, anti-aircraft fire, wear, repair, fatigue of groups, pilots, replacements and base supply: chapter 5.
- Missions, objectives, points, margins and results: chapter 6.
- Submarines, their passage, attack and hunt, reports of submarines and suspected waters: chapter 10.
- Intents, limits, plans, commands, the staff and its reasons, and days of supply: chapter 8.
- The pictures of a trial and how a picture's enemy is made: chapter 9.
- The agent tools, the briefing, `explain`, `measure`, the serve command and the MCP server: not rules.
- The figures: the data files in `data/reference`.
