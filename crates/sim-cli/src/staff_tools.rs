//! The tools of a seat whose side has a staff, as the MCP server and `play`
//! list them (decision 0107, "As built", "The tools"): each with the
//! description a language model commands by and its input schema. The
//! answers are the agent interface's (`sim_agent`), reached through
//! `Session::call`.

use serde_json::{json, Value};

/// What the instructions say of commanding by intent, for a seat whose side
/// has a staff.
pub const STAFF_INSTRUCTIONS: &str = "\n\nCommanding by intent. Beside orders to units you can state an intent and let your staff carry it out. There are nine kinds of intent, set with set_intent {intent_kind, base, target, hex, distance, task_force, ...}. keep_supplied (the default where you give no intent_kind): set_intent {base, floor_days, interval_days} asks the staff to keep the stock at your base N at or above floor_days days of supply, each shipment bringing interval_days days of the base's use above the floor (the staff uses 15 where you give none). For keep_supplied: set_limit {limit: \"forces\", intent, ships: [your ship numbers], task_forces: [], source: base number}, the ships it may use and the one base it loads them at; set_limit {limit: \"reserve\", base, supply_t, fuel_t} keeps a stock at a base that the staff's loads never take. hold {intent_kind: \"hold\", base}: hold one of your bases, or an enemy base on your map, in which case the staff waits until it is yours (a hold of a base that is not yet yours forms nothing); an end that has no date; you state it and the staff forms what serves it, a sub-intent cover {intent_kind: \"cover\", base}: cover the base from the air (its fighters fly the patrol and its search groups search by the staff's doctrine; you may also state a cover intent yourself as a root). Intents form a tree: a sub-intent the staff formed stands under the intent that asked for it (command_state gives each intent's parent, its maker \"commander\" or \"staff\", its addressee and its children, and lists a sub-intent under its parent). You cannot set or withdraw an intent the staff made (it is refused with the reason); you shape it by its limits and holds, or withdraw the intent it stands under, which withdraws it too. A limit may be set on any intent, the staff's included; one not set on an intent is read up the tree to the root (limits_read_up in command_state shows the value and the intent it is read from), and a limit on a sub-intent cannot be wider than what its parent allows. A hold or cover intent has no forces limit needed: with none above it the staff may use the air groups based at its base that are not held, and set_limit {limit: \"forces\", intent, air_groups: [your air group numbers]} names the groups it may use instead (ships, task_forces and source are optional there; a supply intent's forces limit must name a source); set_limit {limit: \"risk\", intent, risk} (preserve, balanced or press; balanced where none is set) sets how hard the cover works its groups. hold {air_group} with no setting keeps a whole air group from the staff; with setting \"search_arc\" only that setting. neutralise {intent_kind: \"neutralise\", target}: neutralise an enemy base on your map, an end addressed to the area that has no date and takes no base of yours; the staff forms under it a sub-intent raid {intent_kind: \"raid\", base, target} for each of your bases with bombers that reach the target (you may also state a raid yourself as a root, or under a neutralise of the same target); a raid is a task addressed to your base, whose bomber groups are the air groups based there that are not held unless you limit them (set_limit forces with air_groups). A neutralise or a deny whose forces limit names no task force also takes each free carrier force of yours (on the carrier mission, with no carrier task, no intent addressed to it, named in no forces limit, not held) once its plan is released, at a station the staff chooses by the risk, the enemy bases and fresh carrier contacts you know, and the fuel; under a neutralise its bombers raid the target when no ship is in their reach. To keep a carrier force out of it, hold the force (hold {task_force}). deny {intent_kind: \"deny\", hex: [q, r], distance}: deny the enemy the waters within distance hexes of hex, an end addressed to the area that has no date and takes no base; the staff searches the waters and forms under it a sub-intent strike_ships {intent_kind: \"strike_ships\", base} for each of your bases whose groups reach them (you may also state a strike_ships intent yourself as a root, giving a base the task of striking ships; search_waters {base} is the staff's own and a commander may not state it). stand_at {intent_kind: \"stand_at\", task_force, hex}: stand one of your task forces at hex for the night, a task addressed to the task force; operate_carriers {intent_kind: \"operate_carriers\", task_force, hex}: operate your carrier task force from hex, a task addressed to the task force (both take no base). bombard {intent_kind: \"bombard\", task_force, target}: have one of your task forces bombard an enemy base on your map from the sea, a task addressed to the task force (decision 0146). A deny intent has a measure, in command_state and report: the days it has been in force, the days of them that ended with no contact of the enemy held in the waters, the contacts held in the waters at the end of a day, the strikes launched from the bases given the task, and the ships sunk and hits your people claim from the air in the waters (claims: never below the truth, never corrected). A neutralise intent has a measure, in command_state and report: the days it has been in force, the raids flown on the target (each pulse in which a bomber group of yours launched at it), the aircraft your people claim destroyed on the ground at the target's hex (a claim: never below the truth, never corrected), the air raids on your own bases and the days of them on which none of your bases was raided from the air. Every decision of your staff's carries the level that decided, the addressee of the intent served (the area, one of your bases, or one of your task forces), in why and in each reason's serves: \"decided at the level of the area\". A hold intent has a measure, in command_state and report: the days it has been in force, the days of them on which the base was yours at the end of the day with its airfield open, the air raids on the base and how many a patrol of yours fought, and the base's days of supply today. The staff does nothing for your side until you set an intent (for keep_supplied AND give it a forces limit). The staff drafts a plan for the intent. propose {intent} shows what it would do (the shipments, their loads and days, what binds; for a hold, cover, neutralise, raid or deny intent, the tree it would form: the sub-intents, the air groups each would use with their missions, and the search arc with the threats that gave it); approve {plan} prepares a draft and releases a plan that is preparing; with leave to prepare (given by default) the staff prepares drafts itself, and with leave to launch (withheld by default; set_leave {leave: \"launch\", given: true}) it releases them itself. staff_orders shows what the staff will hand in at the start of the coming day, each with its reason; hold {ship}, {task_force}, {air_group} or {air_group, setting: \"search_arc\"} keeps a unit from the staff (it gives it no order and plans without it) until release lifts the hold; preview {command: \"set_intent\" or \"set_limit\", and that tool's arguments} shows what a changed intent or limit would do and changes nothing, and approve {preview: N} queues exactly that command; why {number} gives the reason of one decision it made, with the chain of intents it served up to your own (command_state lists yesterday's decisions with their numbers); command_state and plans show what stands. patrol: set_intent {intent_kind: \"patrol\", hex, distance} patrols the waters within distance hexes of hex with your submarines, an end addressed to the area: the staff gives each boat it may use a station of the waters in the order of the boats and orders her there (the boats are named by set_limit {limit: \"forces\", intent, submarines: [your submarine numbers]}; with none named, every boat of yours that is not held). hold {submarine: N} keeps a boat from the staff, as an order of yours to her does (release {submarine: N} frees her); list_forces shows whose hands each boat is in (the staff's under an intent, yours, or free), and list_contacts your reports of enemy submarines and the waters you suspect them in, which are reports and belief only. Commands are queued for the coming day like orders, checked at once (queued or refused with the reason) and take effect at the start of the coming day, before the staff is asked; then call end_day as always.

Interrupts and running time. An interrupt is a condition that stops the clock for you. set_interrupt {base, below_days} sets one: your base's days of supply below that number. It is tested each morning after the staff has handed in; it fires once, when the figure is below the number, and is armed again on a morning the figure is back at or above it. Six standing interrupts are on by default and need no setting: the staff cannot meet an intent (cannot_meet), a plan waits for your leave (waits_for_leave), a ship of yours was lost (ship_lost), the staff of a take cannot bring the men it wants on any day in reach (cannot_bring_enough), the staff turned a take's lift back for enemy carriers near its way (lift_turned_back), and a carrier force asks your leave to go after what it has found (asks_leave_to_pursue); set_interrupt {standing, on: false} switches one off and {standing, on: true} on again. run {days: N} resolves up to N days, each exactly as end_day does (the orders and commands you queued go in on the first day), and stops at the end of the first day on which any interrupt of yours fired, or when the game is over, or after N days: nothing is ordered inside a day, so you act on the next morning (a force's asking leave to pursue is the one exception: run stops in the evening before the day is played, with the asking in its answer, so that you can answer it with answer_pursuit or leave it, and the day is then played when you call run again; set_pursuit gives each of your orders ask, go or stay). Its answer first lists the staff's orders for the first day (worked before that day was resolved), then says the days resolved, why it stopped (interrupt, game_over, days or asks_leave) and everything that fired on the last day, each firing with its why numbers for the staff's reasons. run is offered only where one seat is held by a player. acknowledge {interrupt: N} or {standing, plan: N} or {standing: ship_lost, ship: N} or {standing: asks_leave_to_pursue, task_force: N} marks a firing seen: it does not arm the interrupt again (that happens on a morning its condition does not hold), and report lists what has fired and not been acknowledged first. report shows what is open in your command of the staff (a plan that waits for your leave, cannot meet its intent, or whose run has ended, each with the days it was handed in and the why number of each day's reason), what closed, and the interrupts you set. A quiet game is commanded with a few calls: state the intents, limits and leave, set an interrupt, run many days, read what stopped it, act, run again.";

const STAFF_RULE: &str = "The staff does nothing for your side until you set an intent (set_intent); a supply intent needs a forces limit (set_limit with limit forces), a hold or cover intent may use the air groups at its base: with no intent in force it hands in nothing.";

const COMMAND_STATE_DESCRIPTION: &str = "What stands in your command of the staff, as it will stand when the coming day begins (the commands you have queued taken in): your intents as a tree, each root followed by the intents under it (number, kind keep_supplied, cover, hold, neutralise, raid, deny, strike_ships, search_waters, stand_at, operate_carriers, ready_landing, build_up or take_place, the base (null for neutralise, deny, stand_at, operate_carriers, ready_landing and take_place), the task_force (the side's number, for stand_at, operate_carriers and ready_landing, null for the other kinds), by_day (the morning a ready_landing is to be ready by, or a take_place to have taken its place by), phase (for a take_place: prepare, move or land; null for the other kinds), wants and defences (a build_up's, in order, and the points of its defences), null for the other kinds, the target (the enemy base of a neutralise or a raid, null for the other kinds), hex and distance (the waters of a deny, null for the other kinds; hex is also the station of a stand_at or operate_carriers, the landing hex of a ready_landing and the place of a take_place), floor_days and interval_days or \"not set: the staff uses 15\" for keep_supplied, parent, maker commander or staff, addressee, children, the limits set on the intent itself: forces with air groups, area, priority, protected ships, risk, not_before, wait_for, after and knowledge (null where unset), and limits_read_up: each limit as read up the tree with the intent it is read from, and what holds where none is set (not_before the latest day on the chain, wait_for and after every one on the chain, each with the intent it is set on, knowledge the nearest with knowledge_from); for a hold intent its measure: days in force, days met, raids, raids fought and the base's days of supply; for a neutralise intent its measure: days in force, days met (days on which none of your bases was raided from the air), raids flown on the target, aircraft claimed destroyed on the ground there and the air raids on your bases; and for a deny intent its measure: days in force, days met (days that ended with no contact held in the waters), contacts held in the waters, strikes launched, and ships sunk and hits claimed from the air in the waters), and for each intent in force the staff's projection of the base's days of supply (today's figure and the first day the projection falls below the floor); the two leaves (prepare, given by default, lets the staff approve a draft; launch, withheld by default, lets it release a plan that is preparing); the reserves your bases keep (and their stocks); your plans (state draft, preparing, released, complete, failed or cancelled; the day it was approved; amendments; the runs with their four days load, sail, arrive and free, their ships and load in tons; and need_to_know, as plans gives it, for a take_place's plan that is preparing); the commitments of your ships; your holds (each with its target, the day, whether you made it by a hold command or an order of yours made it, and that order's number; held units get no order from the staff until released); your interrupts (each one you set with its base, below_days, the day set, whether armed, the day it last fired and whether that is acknowledged; each standing record that has fired; the standing kinds switched off; those fired and not acknowledged first); the commands you have queued for the coming day, in order; the staff's decisions of yesterday, each with the number why takes; notes saying what waits for you; and, with command levels on, picture (the area command's picture of the enemy: reports_travel and its contacts, each with its number as list_contacts writes it, place, the kind taken for, ships, age in pulses and own, true where the report that last made or refreshed the contact was the area command's own search, strike or fight and not one it was told of) and forces_held (each task force and base the area command holds, as it holds it: handle, place, as_of in the words list_forces uses, always given here, current, true when that is of the pulse just ended, and radio_silence for a force; empty where nothing is held). With command levels off neither is given. STAFF_RULE Takes no arguments. Quantities are decimal strings.";

const PLANS_DESCRIPTION: &str = "Your plans and what each run is doing, as the coming day will find them (the commands you have queued taken in). A plan is the staff's way of meeting one intent: it is a draft until approved (approve, or the staff's own approval with leave to prepare), preparing once approved and released once launched (approve again, or the staff's with leave to launch), and complete, failed or cancelled when it ends. A run is one convoy of the plan: its ships, its load in tons of supply from the source base, and its four days: load (loading begins at the source), sail, arrive (due at the destination) and free (its ships are back and refuelled). Each run shows the phase its days put it in (waiting, loading, under_way, returning, finished), a sentence saying what it is doing and, for each of its ships, where it is and the supply aboard. A take_place's plan that is preparing has need_to_know (null for every other plan, a released one too): what must be known of the enemy at the place before the landing goes, as the staff works it this morning: asked (setting none, count or recent_count, within_days, and from, the intent your knowledge limit is set on, null where the risk decides), held (your estimate: men, came_from, day, age_days, and could_be, the lowest and highest it could be in truth from its grade's error range, null for a count you were told), looking (the air group on reconnaissance of the place after this morning's orders, and since_day), met, and text, the need in one line. STAFF_RULE Takes no arguments.";

const SET_INTENT_DESCRIPTION: &str = "Queue an intent for the coming day. Arguments: intent_kind (optional: \"keep_supplied\", the default, keeps the stock at one of your bases at or above a floor of days of supply; \"hold\" holds one of your bases, or an enemy base on your map, which the staff then waits for until it is yours; once it is yours the staff sets the men of the hold there on attack to defend and, where the hold's forces limit names a source, forms keep_supplied under it; for a base that is yours it is an end the staff serves by forming cover from the air under it; \"cover\" covers one of your bases from the air, with no days; \"neutralise\" neutralises an enemy base on your map, an end the staff serves by forming a raid from each of your bases with bombers that reach it, and takes target and no base; \"raid\" raids the target from your base, a task, and takes base and target; \"deny\" denies the enemy the waters within distance hexes of hex, an end addressed to the area that takes hex and distance and no base; \"patrol\" patrols the waters within distance hexes of hex with your submarines, an end addressed to the area that takes hex and distance and no base, the boats it may use being named in a forces limit with submarines; \"destroy_force\" destroys an enemy force you hold a contact of (decision 0220): an order addressed to the area that takes contact (the number of the contact in your picture, as list_contacts and command_state give it; a number your picture does not hold, or none, is refused with the reason that the contact is not held, and a contact held but no longer fresh may be named), distance (a whole number of hexes, 0 or more: the waters round where the force is reported) and, if wanted, search_only (true or false, false where left out: the staff only searches for the force and strikes nothing); it takes no hex, and the staff follows the force from report to report, takes another contact for it when the one it follows has gone stale, and tells you when it takes a contact of the other sort (a carrier force for a force that was not, or the other way), when it has lost the force, and when it has not found it and the order is ended (command_state shows the order's paper: what it follows now by the report, each taking with its day, whether it is lost and the ring); a seat that is not the area command's cannot give it; \"strike_ships\" gives your base the task of striking ships, which its air groups strike by their missions, and takes base; \"search_waters\" is the staff's own task under a deny and a commander may not state it as a root: it is refused; \"stand_at\" stands one of your task forces at hex for the night and \"operate_carriers\" operates your carrier task force from hex, each a task addressed to the task force that takes task_force and hex and no base; \"ready_landing\" readies a landing by one of your task forces at a coast hex, a task addressed to the task force that takes task_force, hex and an optional by_day (the morning it is to be ready by) and no base, the land units to land being named in a forces limit with land_units; \"take_place\" takes a hex of land, an end addressed to the area that takes hex (or base, the number of an enemy base on your map, whose hex is the place; exactly one of the two) and an optional by_day (a whole number from 1, the morning it is to be taken by), no task_force and no other key, and is refused where the hex is not land; \"control_island\" controls an island the scenario names, an end addressed to the area that takes island (its number) and an optional by_day (a whole number from 1) and no other key, the staff drawing the takes, holds, supply, denial and neutralising under it; \"bombard\" has one of your task forces bombard an enemy base on your map from the sea, a task addressed to the task force that takes task_force and target and no base (decision 0146); \"build_up\" builds up one of your bases with what is wanted there, a task addressed to the base that takes base, wants (a list in order of airfield, mend, defences, port and aircraft, each at most once) and defences (the points, 1 to 100, required when defences is among the wants and refused otherwise)), task_force (stand_at, operate_carriers, ready_landing and bombard only: the number of one of your task forces; the same refusal for an enemy force and for one that does not exist), base (a base number, one of your own; not for neutralise, deny, stand_at, operate_carriers, ready_landing or bombard; for take_place only in place of hex), hex ([q, r], deny, stand_at, operate_carriers and ready_landing only: the centre of the waters of a deny, the station of the next two, the coast hex of a landing), distance (deny and destroy_force only, a whole number of hexes, 0 or more: the waters are every hex within it of hex, or of where the force is reported; a hex off the map is refused), target (neutralise, raid and bombard only: the number of an enemy base on your map; the same refusal for a base that is yours or does not exist), floor_days (keep_supplied only, a whole number of days, at least 1: the staff keeps the base at or above this many days of supply), interval_days (keep_supplied only, optional, a whole number of days: each shipment brings this many days of the base's use above the floor; with none the staff uses 15), parent (optional, decision 0232: the number of an intent in force that the new intent stands under; in this build a take_place may be stated under a control_island, the hex being one of the island's named places, and it is then your own node of the island's plan, which the staff does not form a take beside; left out for a new intent it is a root, and for an intent in force it is read as the parent it has, so it is never needed to set an intent again; an intent cannot be moved to another parent), intent (optional: the intent's number; with none the side's next free number is used, one more than the highest ever set, and the answer says which; a number once withdrawn is never used again; setting a number that is in force changes that intent, and an intent whose plan is approved is amended). Examples: {\"base\": 2, \"floor_days\": 30, \"interval_days\": 15} and {\"intent_kind\": \"hold\", \"base\": 1}, {\"intent_kind\": \"neutralise\", \"target\": 7} and {\"intent_kind\": \"raid\", \"base\": 1, \"target\": 7}, {\"intent_kind\": \"deny\", \"hex\": [12, -3], \"distance\": 4}, {\"intent_kind\": \"destroy_force\", \"contact\": 3, \"distance\": 2} and {\"intent_kind\": \"strike_ships\", \"base\": 1}. An intent the staff formed is refused here with the reason: you cannot set or withdraw it. The command is checked at once by taking it in after the commands you have already queued: the answer says queued for day N, or refused with the engine's reason (a base that is not yours, an intent number not allowed, floor_days of 0; the same words for an enemy base and for one that does not exist). A queued command takes effect at the start of day N. Setting an intent drafts a plan for it. STAFF_RULE For keep_supplied give it a forces limit next (set_limit); a hold, cover, raid or strike_ships intent may use the air groups at its base, a neutralise intent the air groups at your bases that reach the target, and a deny intent the air groups at your bases that reach the waters, unless you limit them. \"clear\": true empties your queued commands first; \"reason\" is for the record.";

const WITHDRAW_INTENT_DESCRIPTION: &str = "Queue the withdrawal of an intent that is in force, for the coming day. Argument: intent (its number). Its plan ends (complete if it was released, else cancelled) and the number is never used again; the intents the staff formed under it are withdrawn with it. An intent the staff made cannot be withdrawn by you: it is refused with the reason. Example: {\"intent\": 1}. Checked at once: queued for day N, or refused with the reason (no such intent in force). STAFF_RULE Withdrawing the intent stops the staff acting for it. \"clear\": true empties your queued commands first.";

const SET_LIMIT_DESCRIPTION: &str = "Queue one limit for the coming day. \"limit\" names it and the other arguments are that limit's: forces {intent, ships: [your ship numbers], task_forces: [your task force numbers], air_groups: [your air group numbers], land_units: [your land unit numbers], source: base number}: the pool the staff may use for the intent, and for a supply intent the ONE base it loads from (ships, task_forces, air_groups and land_units default to none; land_units are the land units the staff may order for a ready_landing or build_up intent; a task force in the pool brings all its ships; a supply intent's forces limit must name a source, a cover or hold intent's need not; it may be set on any intent, one the staff formed too, and may not be wider than what the parent allows; with none on an intent it is the nearest one above it, and with none anywhere a hold or cover intent may use the air groups based at its base that are not held); area {intent, hexes: [[q, r], ...]}: the only hexes the intent's ships may enter (with none set the sea is open); priority {intent, rank: whole number, 1 first, or null}: its rank; protected {intent, ships: [ship numbers]}: ships the staff may not take from the intent; reserve {base, supply_t, fuel_t}: the tons of supply and of fuel the base keeps, which the staff's loads and refuelling never take (decimal strings, or whole numbers); risk {intent, risk: preserve, balanced or press} (read up the tree where not set on the intent; balanced where none is set above it; a sub-intent may not be bolder than its parent); and three that tell a take_place or control_island of yours to wait, each read up the tree and each binding, set only on one of those (refused on any other kind): not_before {intent, from_day}: the staff readies the landing but does not release the plan, nor ask leave to launch, before that day; wait_for {intent, task_force or land_unit}: not until you may use that force or unit and it lies at the source the forces limit names (a force stopped there, a unit ashore there); after {intent, other_intent}: not until, on a morning, the other take_place's place (every place of a control_island) is yours (an after that would have two intents wait for each other, directly or through a third, is refused). Each with its other key left out clears it. While one is unmet the staff's reason says what it waits for; one that can never be met (an after naming an intent withdrawn, a wait_for naming a force or unit you no longer have) is said once with the notice that the intent cannot be met. And knowledge {intent, knowledge: none, count or recent_count, within_days}, also only on a take_place or control_island of yours, read up the tree: how much must be known of the enemy at the place before the landing goes, apart from the risk (none: on any count or none; count: on a count seen from the air or by men in contact, of any day; recent_count: on such a count no more than within_days days old on the morning it goes, 3 where within_days is left out, and at least 1). Unset, the risk decides as before (press none, balanced a count, preserve a recent count of 3 days); set, it decides how much must be known and the risk still decides the men wanted. knowledge left out clears it. Examples: {\"limit\": \"forces\", \"intent\": 1, \"ships\": [1, 2], \"task_forces\": [], \"source\": 1} and {\"limit\": \"reserve\", \"base\": 1, \"supply_t\": \"1000\", \"fuel_t\": \"0\"}. Ships, task forces, air groups, submarines and land units are in YOUR numbers (those the briefing shows). Checked at once after the commands already queued: queued for day N, or refused with the engine's reason (a ship or base that is not yours, an intent not in force, a weight below zero). A change to a limit of an intent whose plan is approved is an amendment of the plan. STAFF_RULE The forces limit is the one that matters first. \"clear\": true empties your queued commands first.";

const SET_LEAVE_DESCRIPTION: &str = "Queue the giving or withholding of one of your two leaves for the coming day. Arguments: leave (\"prepare\" or \"launch\") and given (true or false). prepare: the staff may make a draft plan inside your limits ready (preparing) without you; given by default. launch: the staff may release a plan that is preparing without you; withheld by default, so until you give it you release a plan yourself with approve. Example: {\"leave\": \"launch\", \"given\": true}. Checked at once: queued for day N. STAFF_RULE \"clear\": true empties your queued commands first.";

const SET_PURSUIT_DESCRIPTION: &str = "Queue, for the coming day, what a carrier force of an order of yours may do about what it finds (decision 0214): whether it may leave the order to go after a carrier or other enemy it holds a fresh contact of that is too far for one night's steaming. Arguments: intent (the number of one of your intents in force) and leave (\"ask\", \"go\" or \"stay\"; null, or left out, clears it). ask: your staff asks your leave for each find: it hands in a notice that the force asks leave to pursue, naming the force, the contact, where the force would go, the hours to it and the order it would leave, and the clock stops in the evening, before the day is played, so that you can answer (answer_pursuit). go: your staff takes the leave as given and tells you. stay: your staff does not look for such a find; nothing changes from before. The setting is read up the tree like a limit (command_state gives each intent's leave to pursue and the intent it is read from, or \"default\"): with none set it is ask, or go where you have given leave to launch. Example: {\"intent\": 1, \"leave\": \"go\"}. Checked at once after the commands already queued: queued for day N, or refused with the reason (an intent that is not in force). STAFF_RULE \"clear\": true empties your queued commands first.";

const ANSWER_PURSUIT_DESCRIPTION: &str = "Queue your answer, yes or no, to a force's asking leave to go after what it has found (decision 0214), for the coming day. Arguments: task_force (your task force's number), contact (the number of the contact the asking named) and allow (true for yes, false for no). It is taken in at the start of the coming day, before the staff is asked: given at the stop in the evening that the asking caused, a yes sends the force that night, and the staff hands in no asking for it. The leave is the force's, not the contact's: it stands while the force has something to go after, and ends, with a notice, when it has none; a contact seen again under a new number keeps it. A no is kept for that contact while your side holds it. With no answer there is no leave: the force goes on with its order and the staff asks again at the next dusk if the find still stands. Your order is not withdrawn or changed by a pursuit; you can cancel or change it yourself. Example: {\"task_force\": 1, \"contact\": 1, \"allow\": true}. Checked at once after the commands already queued: queued for day N, or refused with the reason (a force that is not yours). STAFF_RULE \"clear\": true empties your queued commands first.";

const HOLD_DESCRIPTION: &str = "Queue a hold for the coming day: keep one unit, or one setting of a unit, from your staff. Give exactly one of: ship (a ship number), task_force (a task force number), air_group (an air group number), submarine (a submarine number: the boat is kept from the staff's patrol) or land_unit (a land unit number: the staff gives it no order), all in YOUR numbers; an air_group alone holds the whole group, and with setting \"search_arc\" (the one setting that can be held) only its search arc, which holds its mission too. What a hold does: the staff gives no order to what is held and plans without it. A held ship is not free to the staff, and neither is any ship of a force that has a held ship in it (an order to that force would move the held ship): a held ship takes its whole force out of the pool of every intent. A run not yet sailed whose ship is held is ended by the staff for good, and its cargo stays aboard; a release does not take it up again, and the staff sizes a new run. A run already under way goes on by the orders it has, lands its load and comes home. A hold does not stop the game's own rules: a held base would still use supply. Your own orders also hold: an order you give yourself (issue_orders) to a ship or task force that is in the forces limit of an intent holds it from that morning, before the staff is asked, and command_state shows it as a hold made by your order, with the order's number. Example: {\"ship\": 1}. Checked at once after the commands already queued: queued for day N (the hold takes effect at the start of day N), or refused with the engine's reason (not yours, already held). STAFF_RULE \"clear\": true empties your queued commands first.";

const RELEASE_DESCRIPTION: &str = "Queue the release of a hold for the coming day. Give exactly one of: ship, task_force, air_group (alone for the whole group, or with setting \"search_arc\"), submarine or land_unit, in YOUR numbers, the same as hold. It lifts any hold on that target, whether you made it with hold or an order of yours made it (command_state shows which): from the start of day N the unit is free to the staff again, newly available, and the staff plans from where it is and what is wanted then. A run that was ended because its ship was held is not taken up again. Example: {\"ship\": 1}. Checked at once after the commands already queued: queued for day N, or refused with the engine's reason (not held, not yours). STAFF_RULE \"clear\": true empties your queued commands first.";

const PREVIEW_DESCRIPTION: &str = "What changing an intent or a limit would do, worked on copies of your state and stored nowhere: it changes nothing (no state, no event, nothing queued). Arguments: command (\"set_intent\", \"set_limit\" or \"withdraw_intent\") and the arguments of that tool, as set_intent, set_limit or withdraw_intent take them. The change is built and checked as that tool would (a bad one is refused with the reason, as there) and the staff is asked twice for the coming day on the state with your queued commands and orders taken in: as things stand, and with this change queued last. The answer gives the preview's number (1, 2, 3 among the previews you were shown), the command and its words, and for the intent concerned (the intent set, or of the limit; a reserve is read as about the first intent whose forces limit loads at that base): as_things_stand and with_change, each with the staff's orders, commands and decisions to give no order for that intent on the coming day (as staff_orders shows them), the runs the staff would make (ships, load in tons, days), the next of them, the first day the base's days of supply fall below the floor, and what binds; held (the holds that keep a ship of the intent's pool from the staff); runs_in_hand (each run of the plan not finished, and what the change does to it: a run that has sailed goes on to the base it was sent to, a run still loading is sent to the new base, a change that is not of the base leaves runs as ordered); made_ready_lost (nothing is lost in a supply plan: stock aboard stays aboard and is carried by the ship's next run, stock landed stays where it lies); other_plans_give_up (what another intent would give up). For a set_intent, \"limits\" (a list of limits, each as set_limit takes one without \"intent\", e.g. {\"limit\": \"forces\", \"task_forces\": [1], \"none_of\": [\"air_groups\"]}, {\"limit\": \"risk\", \"risk\": \"press\"}, {\"limit\": \"area\", \"hexes\": [[0, 1]]}, {\"limit\": \"forces\", \"cleared\": true}) is taken with the order as one: they are the order's limits, the staff is read on the order with all of them, the answer gives them in limits and shown_limits (empty lists where none), and if the engine would refuse one the answer is that refusal, saying which it is (of_batch), and nothing is numbered. \"limits\" is not an argument of a set_limit preview. \"under\" (a list of sub-intents written under the order, each a set_intent body without parent, with its own \"limits\", e.g. {\"intent_kind\": \"take_place\", \"hex\": [0, 6], \"limits\": [{\"limit\": \"forces\", \"task_forces\": [3], \"land_units\": [3]}]}; each takes the number given, else one past the highest the side has set, queued or has before it in the batch; an \"after\" may name an earlier sub-intent of the batch) is taken with the order as one: the order, its limits, then each sub-intent followed by its own limits, worked as one, the staff read on the whole draft; the answer gives them in under (decision 0232, rule 20), and a refusal says its place in the whole batch (of_batch, the order being 1). A sub-intent under an order that stands (queued or in force) is previewed as one set_intent with parent and its limits. For a withdraw_intent of a sub-intent you wrote under an order (decision 0232, rule 30) the answer is the staff's reading of that order, as things stand and with the sub-intent withdrawn on the coming morning, which is the staff planning that place itself with what was already ordered standing: intent is the order, and \"limits\" and \"under\" are not arguments of it; the withdrawal of an order that stands under none is an error of the call, and a preview of a withdrawal is not tried (trial the sub-intent itself). Then approve {preview: N} queues exactly that command, its limits and its sub-intents, in order, all or none (limits_queued is how many came with the order; under_queued gives each sub-intent's number and how many limits came with it). A preview is void once anything is queued after it (a command, an order) or a day is resolved: approve then refuses it and says why, and you preview again. Example: {\"command\": \"set_intent\", \"intent\": 1, \"base\": 2, \"floor_days\": 40, \"interval_days\": 15}. STAFF_RULE";

const APPROVE_DESCRIPTION: &str = "Queue the approval of a plan for the coming day, or of a preview: a draft becomes preparing (the staff has leave to prepare, so it does this itself unless you withheld the leave), and a plan that is preparing is released (the staff does this itself only with leave to launch). Arguments: plan (its number, as command_state and plans show it), or preview (a preview's number, as preview gave it) in its place: it queues exactly the command that preview showed, through the same check as set_intent or set_limit, and is refused, saying why, if something was queued or a day was resolved after the preview (it is void). Give plan or preview, not both. Example: {\"plan\": 1} or {\"preview\": 1}. Checked at once after the commands already queued, so a draft you approve in this call is preparing for the next: queued for day N, or refused with the reason (no such plan, a plan already released or ended, a void preview). STAFF_RULE \"clear\": true empties your queued commands first (not with a preview).";

const CANCEL_PLAN_DESCRIPTION: &str = "Queue the cancelling of a plan that has not ended, for the coming day. Argument: plan (its number). Example: {\"plan\": 1}. Checked at once: queued for day N, or refused with the reason (no such plan, a plan already ended). A convoy already under way is not recalled in this version: its ships come home by the route they hold. STAFF_RULE \"clear\": true empties your queued commands first.";

const PROPOSE_DESCRIPTION: &str = "What the staff would do for one intent, worked from your state as the coming day will find it (the commands you have queued taken in) and stored nowhere: it changes nothing. Argument: intent (its number). The answer gives the plan and its state; the projection of the base's days of supply over the next 30 days (the stock, the planned use and the figure of each day, today's reading and the first day it falls below the floor); the runs the staff would make over the next 30 days, each with its ships, load in tons, its four days (load, sail, arrive, free) and its reason in five parts; what binds if it cannot meet the intent (no ship free until a day, the source's stock, no route inside the area, a ship without the fuel, the use more than the pool can carry) with the shortfall in days and the day the source falls to its reserve; what this intent is displaced by (the ships or tons at the source that a higher intent's run holds this morning, each by plan, run, ships or tons, and days) and which lower intents the proposal would displace, each with its shortfall in days at the reading it checked (nothing, with one intent); and what the staff would hand in for the coming day if the plan were released (orders, commands and decisions to give no order, in your numbers). For a patrol intent the answer has stations: each boat the staff would give a station, as {submarine, name, hex, place} in place order, beside the words. For a hold or cover intent the answer has a tree: the sub-intents the staff would form (each with its number as your next free number, its parent and its reason), the air groups each intent would use with the order each is given (mission, search arc, patrol share, rest), the search arcs with how the threats gave them, and the decisions to give no order; for a supply intent it is null. Example: {\"intent\": 1}. STAFF_RULE With no forces limit the proposal says so.";

const ORDER_DESCRIPTION: &str = "The plan of one of your own intents as your staff hands it in (decision 0159, rule E): worked from your state as the coming day will find it (the commands you have queued taken in) and stored nowhere: it changes nothing. STAFF_RULE Argument: intent (the number of one of your intents in force that the staff did not make, or of a take_place in force that the staff formed (decision 0164); command_state lists them). The answer gives the mission in words, the plan and its state, the phases in order (for a take a place: prepare, move and land; for a hold: wait and hold) each with what ends it and where it stands (done, under_way or to_come), what the plan waits for (the men wanted when the first lift would sail short, a leave you withhold, a day you set, a task force or land unit you named, another intent, the count of the enemy you asked) with whether each is met and the day it is expected to be, the men wanted against the men to hand, the day the plan expects its first men ashore (null where it cannot say or they are ashore), the need to know of a take that is preparing, the staff's own intents under it with the phase each serves, and a brief in words. An intent that is not in force, is the staff's own and not a take_place or is not yours is refused.";

const STAFF_ORDERS_DESCRIPTION: &str = "What the staff would hand in at the start of the coming day, asked on your state as that day will find it (the commands you have queued taken in): its orders (a unit of yours and the order, written as issue_orders writes them, ships and task forces in your numbers; a task force the staff will form is the next number you would give it), its commands (approve, add_run, commit_ships, free_ships, time_run, mark_plan) and its decisions to give no order, each with its reason in five parts: what it serves, the evidence, what binds it, the calculation (\"worked\") and what would change it. It changes nothing: the staff is asked again when the day begins, on the same state, and gives the same answer. A day on which the staff gives an intent no order it still gives one reason, saying what the runs are doing and what the next run waits for. STAFF_RULE Takes no arguments.";

const WHY_DESCRIPTION: &str = "The reason of one decision your staff made, by its number: the number command_state gives for yesterday's decisions (they are numbered among your events, as explain numbers them). The answer gives the day, what the decision is for (an order of the staff's: its unit, its body and whether it was accepted or refused and why; a command of the staff's: the command and whether it was taken or refused; or a decision to give no order) and its reason in five parts: what it serves, the evidence with its day, what bound it, the calculation (\"worked\", the arithmetic in tons, days and hours) and what would change it. The answer also gives level (the level that decided: the area, or one of your bases, the addressee of the intent served; null where a reason has none; serves ends \"decided at the level of ...\" before the chain) and serves and chain: the intents above the one the decision served, nearest first and ending at the one you stated, for example \"under intent 1 (the commander's): hold base:1\". Argument: number (a whole number). Example: {\"number\": 17}. A number that is not a reason of your staff answers \"not known\", as explain does for an event you may not see. It changes nothing: it only reads. STAFF_RULE";

const SET_INTERRUPT_DESCRIPTION: &str = "Queue an interrupt for the coming day: a condition that stops the clock of run for you. Two forms. (1) A condition you set: {base, below_days, interrupt (optional)}: it holds on a morning your base's days of supply (as list_bases, the briefing and command_state give them) are below below_days (35 days is not below 35); where the base's hex has no figure, it does not hold. interrupt is its number (with none, the side's next free number is used, one more than the highest set, and the answer says which; setting a number again replaces that interrupt with a new one, armed and never fired). It is tested each morning after the staff has handed in and before your orders are taken. It is armed when set; on a morning its condition holds and it is armed it fires and is no longer armed; on a morning its condition does not hold it is armed again. So one already true when you set it fires on that first morning, once, and does not fire again while the figure stays below the number. A firing is an event you see; run stops at the end of that day. (2) A standing interrupt switched off or on: {standing: \"cannot_meet\" | \"waits_for_leave\" | \"ship_lost\" | \"cannot_bring_enough\" | \"lift_turned_back\" | \"asks_leave_to_pursue\", on: true | false}. These six stand without being set, and are ON by default: cannot_meet (the staff cannot meet one of your plans' intents: it needs a run and can have none), waits_for_leave (a plan waits for your leave to prepare or to launch) ship_lost (a ship of yours was lost the day before) and cannot_bring_enough (the staff of a take cannot bring the men it wants on any day in reach and waits for you) and lift_turned_back (the staff turned the lift of a take back to its source for enemy carriers near its way) and asks_leave_to_pursue (a carrier force asks your leave to go after what it has found, decision 0214; it stops the clock in the evening, before the day is played; switching it off gives no leave: set_pursuit is what gives or withholds it). Each is armed apart for each plan, ship or task force. Examples: {\"base\": 2, \"below_days\": 35} and {\"standing\": \"ship_lost\", \"on\": false}. Checked at once after the commands already queued: queued for day N, or refused with the engine's reason (a base that is not yours, below_days of 0). STAFF_RULE \"clear\": true empties your queued commands first; \"reason\" is for the record.";

const ACKNOWLEDGE_DESCRIPTION: &str = "Queue the acknowledgement of an interrupt that fired, for the coming day: it marks the last firing seen. It does not arm the interrupt again: an interrupt is armed again only on a morning its condition does not hold. report lists the interrupts that fired and have not been acknowledged first. Name the interrupt in one of four ways: {interrupt: N} (a number you set); {standing: \"cannot_meet\", \"waits_for_leave\", \"cannot_bring_enough\" or \"lift_turned_back\", plan: N} (your plan's number, as plans shows it); {standing: \"ship_lost\", ship: N} (your ship's number, as the briefing shows it); {standing: \"asks_leave_to_pursue\", task_force: N} (your task force's number). Examples: {\"interrupt\": 1} and {\"standing\": \"cannot_meet\", \"plan\": 2}. Checked at once after the commands already queued: queued for day N, or refused with the engine's reason (an interrupt you never set, one that has not fired, one whose last firing is acknowledged already, a plan or ship that is not yours). STAFF_RULE \"clear\": true empties your queued commands first; \"reason\" is for the record.";

const RUN_DESCRIPTION: &str = "Let time pass: resolve up to `days` days, each exactly as end_day resolves one (the orders and commands you have queued go in on the first day, as end_day would hand them in; nothing more is queued for the later days), and stop at the end of the first day on which any interrupt of yours fired, or when the game is over, or after `days` days. A firing stops the clock at the end of its day, not before: nothing is ordered inside a day, and you act on the next morning. The one exception is a force's asking leave to pursue (decision 0214): where your staff would hand in such an asking for the coming day, run stops in the evening BEFORE that day is played, with no day resolved by that call if it is the first, and nothing in the game changed; answer it (answer_pursuit) or leave it, and call run again: the day is then played without stopping before it again. It also stops after a day on which the staff handed in a notice of following you asked for in stop_on (follows_other_sort, contact_lost or force_not_found, decision 0220), naming the notice; none is asked for unless you give it, and none fires an interrupt. Use set_interrupt to say what should stop it (the staff's standing interrupts, on by default, stop it when the staff cannot meet an intent, a plan waits for your leave, or a ship is lost). Argument: days (a whole number, at least 1). Example: {\"days\": 30}. The answer gives: staff_orders, the staff's orders, commands and decisions for the first day, worked BEFORE that day was resolved (the same as staff_orders gives); days_resolved (first, last, count); stopped (\"interrupt\", \"game_over\", \"days\", \"told\" (with told, the notice) or \"asks_leave\", with asking: each asking, giving task_force, intent, contact, kind, age_pulses, reported_at, to, hours and the interrupt); fired, everything that fired on the last day together, each firing with what it is (the interrupt you set with its number, base, below_days and the figure read; or the standing kind with its plan or ship, in your numbers), its day, whether acknowledged and, for a standing firing on staff notices, the why numbers of the reasons those notices rest on (why {number} gives each); game_over and the day now; the result when the game ends; and a summary line in words. Offered only where exactly one seat is held by a player; refused when the game is over. After it, read the briefing (or command_state and report), act, and run again. STAFF_RULE";

const REPORT_DESCRIPTION: &str = "The command report: what is open in your command of the staff, what closed and the interrupts you set, worked from your events (it stores nothing, so it reads the same after a save and a resume). A matter is a plan, a run where there is one and a kind of notice from the staff: cannot_meet (the staff cannot meet the plan's intent), cannot_bring_enough (the staff of a take cannot bring the men it wants on any day in reach), lift_turned_back (the staff turned a take's lift back for enemy carriers near its way), waits_for_leave_to_prepare or waits_for_leave_to_launch (a plan waits for your leave) or run_ended (the staff has ended a run). A convoy late for a week is one matter, not seven. Each matter gives plan (and its intent), run (or null), kind, words (for example \"plan:1 waits for leave to launch\"), open (true if the staff handed it in on the last day resolved), opened (its first day), closed (the first day after its last day, or null while open) and history: each day it was handed in with the why number of that day's reason (why {number} gives the reason; a matter handed in again after a gap is the same matter, with a gap in its history). Open matters come first, then closed, each by first day. Then interrupts: every numbered interrupt you set (its base, below_days, the day it was set, whether it is armed, the day it last fired, whether that firing is acknowledged), every standing record that has fired (kind, plan or ship, last fired, acknowledged) and the standing kinds you switched off, those fired and not acknowledged first; fired_last_day (what fired on the last day resolved); holds (each hold intent in force with its measure: days in force, days met, raids, raids fought and the base's days of supply); neutralisations (each neutralise intent in force with its measure: days in force, days met, raids flown on the target, aircraft claimed destroyed on the ground there and the air raids on your bases); refusals (over the whole game, from your events: staff_orders_not_as_held, your staff's orders refused because a force was not as the area command held it, staff_orders_other, your staff's orders refused for any other cause, and commands, your commands refused; the summary says so only when one is not zero); and a summary in a few lines (for example \"Day 18. No open matter. 1 matter closed. Interrupts: interrupt:1 (base:2 below 35 days), last fired day 13, acknowledged.\"). All numbers are your own. Before any day is resolved nothing is open. Takes no arguments. STAFF_RULE";

/// The arguments of `hold` and `release`: one of `ship`, `task_force` or
/// `air_group` (with `setting`).
fn hold_schema(what: &str) -> Value {
    let clear =
        json!({"type": "boolean", "description": "true empties your queued commands first."});
    let reason = json!({"type": "string", "description": "Why you give the command, for the record of the game (the game never reads it)."});
    json!({
        "type": "object",
        "description": what,
        "additionalProperties": false,
        "properties": {
            "ship": {"type": "integer", "minimum": 1, "description": "Your ship's number."},
            "task_force": {"type": "integer", "minimum": 1, "description": "Your task force's number."},
            "air_group": {"type": "integer", "minimum": 1, "description": "Your air group's number; alone it is the whole group, with setting only that setting."},
            "land_unit": {"type": "integer", "minimum": 1, "description": "Your land unit's number."},
            "submarine": {"type": "integer", "minimum": 1, "description": "Your submarine's number: a hold keeps her from the staff's patrol, and an order of yours to her does the same."},
            "setting": {"enum": ["search_arc"], "description": "With air_group: the one setting held. The only one is search_arc."},
            "clear": clear,
            "reason": reason
        }
    })
}

/// The staff tools: name, description and input schema, in the order of
/// `sim_agent::COMMAND_TOOLS`.
pub fn staff_tool_definitions() -> Vec<(&'static str, String, Value)> {
    let intent = json!({"type": "integer", "minimum": 1, "description": "An intent's number."});
    let plan = json!({"type": "integer", "minimum": 1, "description": "A plan's number."});
    let clear =
        json!({"type": "boolean", "description": "true empties your queued commands first."});
    let reason = json!({"type": "string", "description": "Why you give the command, for the record of the game (the game never reads it)."});
    let numbers = |what: &str| json!({"type": "array", "items": {"type": "integer", "minimum": 1}, "description": what});
    let described = |text: &str| text.replace("STAFF_RULE", STAFF_RULE);
    let no_arguments = json!({"type": "object", "additionalProperties": false});
    vec![
        (
            "command_state",
            described(COMMAND_STATE_DESCRIPTION),
            no_arguments.clone(),
        ),
        ("plans", described(PLANS_DESCRIPTION), no_arguments.clone()),
        (
            "set_intent",
            described(SET_INTENT_DESCRIPTION),
            json!({
                "type": "object",
                "required": [],
                "additionalProperties": false,
                "properties": {
                    "intent": {"type": "integer", "minimum": 1, "description": "The intent's number; with none, the side's next free number."},
                    "parent": {"type": "integer", "minimum": 1, "description": "(optional) the number of an intent in force this one stands under: a take_place under a control_island, a node of the plan you write yourself. Left out, a root for a new intent, and for one in force the parent it has."},
                    "intent_kind": {"enum": ["keep_supplied", "cover", "hold", "neutralise", "raid", "deny", "patrol", "destroy_force", "strike_ships", "search_waters", "stand_at", "operate_carriers", "ready_landing", "build_up", "take_place", "control_island", "bombard"], "description": "keep_supplied (the default): keep the base supplied; cover: cover the base from the air; hold: hold the base; neutralise: neutralise the enemy base target (takes no base); raid: raid the target from the base; deny: deny the waters within distance hexes of hex (takes no base); strike_ships: the base's groups strike ships; search_waters is the staff's own and is refused here; stand_at: stand the task force at hex for the night; operate_carriers: operate the carrier task force from hex (both take task_force and hex and no base); ready_landing: ready a landing by the task force at the coast hex (takes task_force, hex and an optional by_day, the morning it is to be ready by; no base); build_up: build up your base with what is wanted there, in order (takes base, wants and defences); control_island: control an island the scenario names, an end addressed to the area (takes island and an optional by_day; no other key); take_place: take a hex of land, an end addressed to the area (takes hex, or the number of an enemy base on your map as base, whose hex is the place, and an optional by_day); bombard: the task force bombards the enemy base target from the sea (takes task_force and target; no base); patrol: patrol the waters within distance hexes of hex with your submarines, an end addressed to the area that takes hex and distance and no base (the boats it may use are named in a forces limit with submarines; with none named, every boat of yours that is not held)."},
                    "base": {"type": "integer", "minimum": 1, "description": "The number of one of your bases: the base to keep supplied, cover or hold (a hold may also name an enemy base on your map, which the staff waits for until it is yours), or to raid from. Required for every kind but neutralise, deny, stand_at, operate_carriers, ready_landing, take_place and bombard, which take none (take_place may name an enemy base on your map here instead of hex)."},
                    "hex": {"type": "array", "items": {"type": "integer"}, "minItems": 2, "maxItems": 2, "description": "(deny, patrol; required there) the centre of the waters as [q, r]; (stand_at, operate_carriers; required there) the hex to stand at or to operate the carriers from; (ready_landing; required there) the coast hex of the landing; (take_place; this or base) the hex of land to take."},
                    "task_force": {"type": "integer", "minimum": 1, "description": "(stand_at, operate_carriers, ready_landing, bombard; required there) the number of one of your task forces."},
                    "island": {"type": "integer", "minimum": 1, "description": "(control_island; required there) the number of an island the scenario names."},
                    "by_day": {"type": "integer", "minimum": 1, "description": "(ready_landing, take_place, control_island; optional) the morning of the day the landing is to be ready by, or the place taken by, or the date the staff gives each take under the island."},
                    "wants": {"type": "array", "items": {"enum": ["airfield", "mend", "defences", "port", "aircraft"]}, "minItems": 1, "maxItems": 5, "description": "(build_up; required there) what is wanted at the base, in order, each at most once."},
                    "defences": {"type": "integer", "minimum": 1, "maximum": 100, "description": "(build_up) the points of defences wanted; required when defences is among the wants, and refused otherwise."},
                    "distance": {"type": "integer", "minimum": 0, "description": "(deny, patrol, destroy_force; required there) the waters are every hex within this many hexes of hex, or of where the force is reported."},
                    "contact": {"type": "integer", "minimum": 1, "description": "(destroy_force; required there) the number of a contact in your picture, as list_contacts and command_state give it: the force to destroy. One your picture does not hold, or none, is refused: the contact is not held."},
                    "search_only": {"type": "boolean", "description": "(destroy_force; optional, false where left out) only search for the force and strike nothing."},
                    "target": {"type": "integer", "minimum": 1, "description": "(neutralise, raid, bombard; required there) the number of an enemy base on your map."},
                    "floor_days": {"type": "integer", "minimum": 1, "description": "(keep_supplied, required there) days of supply to keep the base at or above."},
                    "interval_days": {"type": "integer", "minimum": 1, "description": "(keep_supplied) days of use each shipment brings above the floor; with none the staff uses 15."},
                    "clear": clear.clone(),
                    "reason": reason.clone()
                }
            }),
        ),
        (
            "withdraw_intent",
            described(WITHDRAW_INTENT_DESCRIPTION),
            json!({
                "type": "object",
                "required": ["intent"],
                "additionalProperties": false,
                "properties": {"intent": intent.clone(), "clear": clear.clone(), "reason": reason.clone()}
            }),
        ),
        (
            "set_limit",
            described(SET_LIMIT_DESCRIPTION),
            json!({
                "type": "object",
                "required": ["limit"],
                "additionalProperties": false,
                "properties": {
                    "limit": {"enum": ["forces", "area", "priority", "protected", "reserve", "risk", "not_before", "wait_for", "after", "knowledge", "targets"], "description": "Which limit. forces takes intent, ships, task_forces, air_groups, land_units, submarines and source; area takes intent and hexes; priority takes intent and rank; protected takes intent and ships; reserve takes base, supply_t and fuel_t; risk takes intent and risk; not_before takes intent and from_day; wait_for takes intent and one of task_force and land_unit; after takes intent and other_intent; knowledge takes intent, knowledge and (with recent_count) within_days; targets takes intent, kinds and waters_first. Each of the last five with its other key (for targets, kinds, left out or empty) left out clears it."},
                    "intent": intent.clone(),
                    "ships": numbers("(forces, protected) your ship numbers."),
                    "task_forces": numbers("(forces) your task force numbers."),
                    "air_groups": numbers("(forces) your air group numbers: the groups the staff may use for the intent."),
                    "submarines": numbers("(forces) your submarine numbers: the boats a patrol intent may use. A limit that names none leaves the boats unbound."),
                    "land_units": numbers("(forces) your land unit numbers: the land units the staff may order for the intent."),
                    "source": {"type": "integer", "minimum": 1, "description": "(forces) the number of the one base the staff loads from; a supply intent needs one."},
                    "hexes": {"type": "array", "items": {"type": "array", "items": {"type": "integer"}, "minItems": 2, "maxItems": 2}, "description": "(area) hexes as [q, r] pairs."},
                    "rank": {"type": ["integer", "null"], "minimum": 1, "description": "(priority) 1 first; null for unranked."},
                    "base": {"type": "integer", "minimum": 1, "description": "(reserve) the base's number."},
                    "supply_t": {"description": "(reserve) tons of supply kept: a decimal string such as \"1000\", or a whole number.", "oneOf": [{"type": "string"}, {"type": "integer"}]},
                    "fuel_t": {"description": "(reserve) tons of fuel kept: a decimal string or a whole number.", "oneOf": [{"type": "string"}, {"type": "integer"}]},
                    "risk": {"enum": ["preserve", "balanced", "press"], "description": "(risk) the risk level."},
                    "from_day": {"type": "integer", "minimum": 1, "description": "(not_before) the first day the staff may release the plan; left out to clear the limit."},
                    "task_force": {"type": "integer", "minimum": 1, "description": "(wait_for) the number of one of your task forces the plan waits for."},
                    "land_unit": {"type": "integer", "minimum": 1, "description": "(wait_for) the number of one of your land units the plan waits for."},
                    "other_intent": {"type": "integer", "minimum": 1, "description": "(after) the number of another take_place or control_island of yours whose place must be yours first; left out to clear the limit."},
                    "knowledge": {"enum": ["none", "count", "recent_count"], "description": "(knowledge) how much must be known before the landing goes; left out to clear the limit."},
                    "within_days": {"type": "integer", "description": "(knowledge recent_count) the most days old the count may be on the morning it goes, at least 1; 3 where left out."},
                    "kinds": {"type": "array", "items": {"enum": ["carrier", "battleship", "cruiser", "destroyer", "transport", "auxiliary"]}, "maxItems": 6, "description": "(targets) the kinds of ship the bombers strike, most wanted first, each once; left out or empty to clear the limit."},
                    "waters_first": {"type": "boolean", "description": "(targets) whether the waters of the deny the intent is, or stands under, are struck first; false where left out."},
                    "none_of": {"type": "array", "items": {"enum": ["ships", "air_groups", "land_units", "submarines"]}, "description": "(forces) the kinds of which the staff may use nothing (ships and task forces are one kind); a kind named in the limit may not be listed."},
                    "in_place": {"type": "boolean", "description": "(forces) true: the task forces named work from where they lie, and are not stationed by the staff; needs a task force named. False where left out."},
                    "cleared": {"type": "boolean", "description": "(forces, area) true clears the limit, given with nothing else but intent: the limit above, or the staff's choice, binds."},
                    "clear": clear.clone(),
                    "reason": reason.clone()
                }
            }),
        ),
        (
            "set_leave",
            described(SET_LEAVE_DESCRIPTION),
            json!({
                "type": "object",
                "required": ["leave", "given"],
                "additionalProperties": false,
                "properties": {
                    "leave": {"enum": ["prepare", "launch"], "description": "Which leave."},
                    "given": {"type": "boolean", "description": "true to give it, false to withhold it."},
                    "clear": clear.clone(),
                    "reason": reason.clone()
                }
            }),
        ),
        (
            "hold",
            described(HOLD_DESCRIPTION),
            hold_schema("A hold: give one of ship, task_force, air_group, land_unit or submarine."),
        ),
        (
            "release",
            described(RELEASE_DESCRIPTION),
            hold_schema(
                "A release: give one of ship, task_force, air_group, land_unit or submarine.",
            ),
        ),
        (
            "preview",
            described(PREVIEW_DESCRIPTION),
            preview_with_targets(json!({
                "type": "object",
                "required": ["command"],
                "additionalProperties": false,
                "properties": {
                    "command": {"enum": ["set_intent", "set_limit", "withdraw_intent"], "description": "Which command to preview; the other arguments are that tool's own."},
                    "intent": {"type": "integer", "minimum": 1, "description": "(set_intent) the intent's number, as set_intent takes it, or (set_limit) the intent the limit is for, or (withdraw_intent) the sub-intent you wrote that is withdrawn."},
                    "intent_kind": {"enum": ["keep_supplied", "cover", "hold", "neutralise", "raid", "deny", "patrol", "destroy_force", "strike_ships", "search_waters", "stand_at", "operate_carriers", "ready_landing", "build_up", "take_place", "control_island", "bombard"], "description": "(set_intent) the kind of intent, as set_intent takes it."},
                    "hex": {"type": "array", "items": {"type": "integer"}, "minItems": 2, "maxItems": 2, "description": "(set_intent deny, patrol) the centre of the waters as [q, r]; (set_intent stand_at, operate_carriers) the hex to stand at or operate from; (set_intent ready_landing) the coast hex of the landing; (set_intent take_place) the hex of land to take."},
                    "task_force": {"type": "integer", "minimum": 1, "description": "(set_intent stand_at, operate_carriers, ready_landing, bombard) the number of one of your task forces, or (set_limit wait_for) the task force waited for."},
                    "island": {"type": "integer", "minimum": 1, "description": "(set_intent control_island; required there) the number of an island the scenario names."},
                    "by_day": {"type": "integer", "minimum": 1, "description": "(set_intent ready_landing, take_place, control_island; optional) the morning of the day the landing is to be ready by, or the place taken by, or the date the staff gives each take under the island."},
                    "wants": {"type": "array", "items": {"enum": ["airfield", "mend", "defences", "port", "aircraft"]}, "minItems": 1, "maxItems": 5, "description": "(set_intent build_up) what is wanted at the base, in order, each at most once."},
                    "defences": {"type": "integer", "minimum": 1, "maximum": 100, "description": "(set_intent build_up) the points of defences wanted, when defences is among the wants."},
                    "distance": {"type": "integer", "minimum": 0, "description": "(set_intent deny, patrol, destroy_force) the waters are every hex within this many hexes of hex, or of where the force is reported."},
                    "target": {"type": "integer", "minimum": 1, "description": "(set_intent neutralise, raid, bombard) the number of an enemy base on your map."},
                    "base": {"type": "integer", "minimum": 1, "description": "(set_intent) the base to keep supplied, cover or hold, or (set_intent take_place) an enemy base on your map whose hex is the place, or (set_limit reserve) the base's number."},
                    "floor_days": {"type": "integer", "minimum": 1, "description": "(set_intent keep_supplied) days of supply to keep the base at or above."},
                    "interval_days": {"type": "integer", "minimum": 1, "description": "(set_intent) days of use each shipment brings above the floor."},
                    "limit": {"enum": ["forces", "area", "priority", "protected", "reserve", "risk", "not_before", "wait_for", "after", "knowledge", "targets"], "description": "(set_limit) which limit, as set_limit takes it."},
                    "ships": numbers("(set_limit forces, protected) your ship numbers."),
                    "task_forces": numbers("(set_limit forces) your task force numbers."),
                    "air_groups": numbers("(set_limit forces) your air group numbers."),
                    "submarines": numbers("(set_limit forces) your submarine numbers."),
                    "land_units": numbers("(set_limit forces) your land unit numbers."),
                    "source": {"type": "integer", "minimum": 1, "description": "(set_limit forces) the number of the one base the staff loads from."},
                    "hexes": {"type": "array", "items": {"type": "array", "items": {"type": "integer"}, "minItems": 2, "maxItems": 2}, "description": "(set_limit area) hexes as [q, r] pairs."},
                    "rank": {"type": ["integer", "null"], "minimum": 1, "description": "(set_limit priority) 1 first; null for unranked."},
                    "supply_t": {"description": "(set_limit reserve) tons of supply kept.", "oneOf": [{"type": "string"}, {"type": "integer"}]},
                    "fuel_t": {"description": "(set_limit reserve) tons of fuel kept.", "oneOf": [{"type": "string"}, {"type": "integer"}]},
                    "risk": {"enum": ["preserve", "balanced", "press"], "description": "(set_limit risk) the risk level."},
                    "from_day": {"type": "integer", "minimum": 1, "description": "(set_limit not_before) the first day the staff may release the plan."},
                    "land_unit": {"type": "integer", "minimum": 1, "description": "(set_limit wait_for) the number of one of your land units."},
                    "other_intent": {"type": "integer", "minimum": 1, "description": "(set_limit after) another take_place or control_island of yours."},
                    "knowledge": {"enum": ["none", "count", "recent_count"], "description": "(set_limit knowledge) how much must be known before the landing goes."},
                    "within_days": {"type": "integer", "description": "(set_limit knowledge recent_count) the most days old the count may be, at least 1; 3 where left out."},
                    "limits": {"type": "array", "items": {"type": "object"}, "description": "(set_intent) limits of the order, each a set_limit body without intent (the limit is of this order), taken with it as one."},
                    "under": {"type": "array", "items": {"type": "object"}, "description": "(preview of a set_intent) sub-intents the commander writes under the order, each a set_intent body without parent, with its own limits as limits; taken in with the order as one, after the order's limits, each followed by its own."},
                    "reason": reason.clone()
                }
            })),
        ),
        (
            "approve",
            described(APPROVE_DESCRIPTION),
            json!({
                "type": "object",
                "additionalProperties": false,
                "properties": {
                    "plan": plan.clone(),
                    "preview": {"type": "integer", "minimum": 1, "description": "A preview's number, in place of plan: queues exactly the command it showed, the limits given with it and the sub-intents written under it."},
                    "clear": clear.clone(),
                    "reason": reason.clone()
                }
            }),
        ),
        (
            "cancel_plan",
            described(CANCEL_PLAN_DESCRIPTION),
            json!({
                "type": "object",
                "required": ["plan"],
                "additionalProperties": false,
                "properties": {"plan": plan, "clear": clear, "reason": reason}
            }),
        ),
        (
            "set_pursuit",
            described(SET_PURSUIT_DESCRIPTION),
            json!({
                "type": "object",
                "required": ["intent"],
                "additionalProperties": false,
                "properties": {
                    "intent": intent.clone(),
                    "leave": {"enum": ["ask", "go", "stay", null], "description": "ask: the staff asks your leave for each find; go: it takes the leave as given and tells you; stay: it looks for no such find. null or left out clears the setting."},
                    "clear": clear.clone(),
                    "reason": reason.clone()
                }
            }),
        ),
        (
            "answer_pursuit",
            described(ANSWER_PURSUIT_DESCRIPTION),
            json!({
                "type": "object",
                "required": ["task_force", "contact", "allow"],
                "additionalProperties": false,
                "properties": {
                    "task_force": {"type": "integer", "minimum": 1, "description": "Your task force's number, as the asking names it."},
                    "contact": {"type": "integer", "minimum": 1, "description": "The contact's number, as the asking names it."},
                    "allow": {"type": "boolean", "description": "true: yes, the force may go after it; false: no."},
                    "clear": clear.clone(),
                    "reason": reason.clone()
                }
            }),
        ),
        (
            "propose",
            described(PROPOSE_DESCRIPTION),
            json!({
                "type": "object",
                "required": ["intent"],
                "additionalProperties": false,
                "properties": {"intent": intent}
            }),
        ),
        (
            "order",
            described(ORDER_DESCRIPTION),
            json!({
                "type": "object",
                "required": ["intent"],
                "additionalProperties": false,
                "properties": {"intent": intent}
            }),
        ),
        (
            "staff_orders",
            described(STAFF_ORDERS_DESCRIPTION),
            no_arguments.clone(),
        ),
        (
            "set_interrupt",
            described(SET_INTERRUPT_DESCRIPTION),
            json!({
                "type": "object",
                "additionalProperties": false,
                "description": "Either a condition (base and below_days, interrupt optional) or a standing kind switched (standing and on).",
                "properties": {
                    "interrupt": {"type": "integer", "minimum": 1, "description": "(condition) the interrupt's number; with none, the side's next free number."},
                    "base": {"type": "integer", "minimum": 1, "description": "(condition) the number of one of your bases."},
                    "below_days": {"type": "integer", "minimum": 1, "description": "(condition) fires when the base's days of supply are below this number."},
                    "standing": {"enum": ["cannot_meet", "waits_for_leave", "ship_lost", "cannot_bring_enough", "lift_turned_back", "asks_leave_to_pursue"], "description": "(standing) which standing interrupt to switch; give on with it."},
                    "on": {"type": "boolean", "description": "(standing) true to switch it on, false to switch it off."},
                    "clear": clear.clone(),
                    "reason": reason.clone()
                }
            }),
        ),
        (
            "acknowledge",
            described(ACKNOWLEDGE_DESCRIPTION),
            json!({
                "type": "object",
                "additionalProperties": false,
                "description": "Give interrupt, or standing with plan, ship or task_force.",
                "properties": {
                    "interrupt": {"type": "integer", "minimum": 1, "description": "The number of an interrupt you set."},
                    "standing": {"enum": ["cannot_meet", "waits_for_leave", "ship_lost", "cannot_bring_enough", "lift_turned_back", "asks_leave_to_pursue"], "description": "A standing interrupt: give plan with cannot_meet, waits_for_leave, cannot_bring_enough or lift_turned_back, ship with ship_lost, task_force with asks_leave_to_pursue."},
                    "plan": {"type": "integer", "minimum": 1, "description": "(standing cannot_meet, waits_for_leave, cannot_bring_enough or lift_turned_back) your plan's number."},
                    "ship": {"type": "integer", "minimum": 1, "description": "(standing ship_lost) your ship's number."},
                    "task_force": {"type": "integer", "minimum": 1, "description": "(standing asks_leave_to_pursue) your task force's number."},
                    "clear": clear.clone(),
                    "reason": reason.clone()
                }
            }),
        ),
        (
            "run",
            described(RUN_DESCRIPTION),
            json!({
                "type": "object",
                "required": ["days"],
                "additionalProperties": false,
                "properties": {
                    "days": {"type": "integer", "minimum": 1, "description": "The most days to resolve."},
                    "stop_on": {
                        "type": "array",
                        "items": {"enum": ["follows_other_sort", "contact_lost", "force_not_found"]},
                        "description": "Notices of an order to destroy a force to stop after a day on (decision 0220), off unless asked: follows_other_sort (the staff took a contact of the other sort for the force: a carrier force where it was not one, or the other way; a taking of the same sort stops nothing), contact_lost (the staff has lost the force and searches where it was last reported) and force_not_found (the order is ended, the force not found). The call stops after the day on which the notice was handed in, never before it, and names the notice."
                    },
                    "reason": reason.clone()
                }
            }),
        ),
        (
            "report",
            described(REPORT_DESCRIPTION),
            no_arguments.clone(),
        ),
        (
            "why",
            described(WHY_DESCRIPTION),
            json!({
                "type": "object",
                "required": ["number"],
                "additionalProperties": false,
                "properties": {"number": {"type": "integer", "minimum": 1, "description": "The number of a staff decision, as command_state lists it."}}
            }),
        ),
    ]
}

const TRIAL_DESCRIPTION: &str = "Try an intent of yours, or a draft you previewed, before it is taken in, and be told what it would cost (decisions 0178 and 0181). The trial plays the coming days in a number of worlds that could be the one you are in, each made from what you know and believe of the enemy, with your staff and his carrying out what each side has handed in, and answers with the estimate: in how many worlds the end was met and kept and by what day, met and lost again, or not met; what it cost your side and what the trial's enemy lost, each as the lowest, the middle and the highest over the worlds; what the plan needs and has and when it could go; what your other plans with an end lose by it, where you have any; what was assumed of the enemy and what each thing assumed was made from; and where the worlds part by a thing you do not know. It speaks in counts of worlds, never in chances or points, and gives no advice. Asked at dusk, between days, it tries the coming morning with everything you have queued for it. Asked inside a day, at 0600 or 1200, it is played from the state as it stands at that hour, as if the day were closed then: it tries the morning of the day after, when what you queue now is taken in, and the rest of that day is not played (forces at sea start where they are at that hour, and what is ashore is a day behind); the answer says so. Your orders to units are not tried, those given for the day's pulses with them, and the answer says so. It changes nothing: nothing is queued, no preview is spent, and the same question at the same hour gives the same answer (asked at 0600 and again at dusk, the answers differ, since the state does). What-ifs: with compare true it is tried beside your side without the thing tried, in the same worlds, and the two are set side by side (a draft of a limit, or of a change to an order that stands, beside the plan as it stands, world for world; a new or queued order beside your side without it; an order in force beside your side with it withdrawn on the coming morning; and for landings you wrote under an order (decision 0232), the same order with the landings left to the staff, in the same worlds, and the order's end in the two world for world: a draft of an order with landings, or a queued order with landings under it, beside the same order with nothing written under it; a new or queued landing beside the same order with that place left to the staff; a landing in force beside the same order with it given back to the staff on the coming morning, what was already ordered standing). Compare is true where none is given for a draft of a limit or of a change, for a new or queued order where you have another plan with an end, and for any written landing or order with landings written under it; false for an order in force and wherever dates are given. With dates (\"chosen\", or a list of one or two days after the coming morning) the whole order is tried at the coming morning and at those later dates, and the answer is a window: for each date what came due on it, the men its take chose, the enemy at the place on its eve, and the estimate; and each later date against the first, world for world. An order in force has no later date, and no window is offered for a written landing or an order with landings written under it (a written landing is held to a day by its \"not before\", which can be previewed and compared). With describe true nothing is played: the answer says what would be tried, the dates with what is due on each, and the worlds and days of each trial, and it is not kept. A trial takes seconds: the small size under about a second on a large game, the full size much longer; a compared trial is two, and a window of three dates three and the days between. Arguments: intent (the number of an intent of yours, in force or queued) or preview (the number of a preview you were shown, still good), one of the two; size (small, standard or full; small where none is given); compare (true or false); dates (\"chosen\" or a list of days); describe (true or false). Example: {\"intent\": 1, \"size\": \"small\", \"compare\": true}. Refused after the game is over, for an intent that is not yours, is the staff's, is withdrawn or has its withdrawal queued, for a preview that is void or not yours, for a size the trial's figures do not name, for dates with compare true, for dates of an order in force or of a written landing or an order with landings written under it, for dates that are not one or two later days in order within the size and the game, and where a command you have queued names the order the trial would move or leave out.";

const TRIALS_DESCRIPTION: &str = "The estimates you have been shown by trial, each with the evening it was asked on and what was asked, marked where it was asked on an earlier evening or before your queue changed; or one again by its number, unchanged. Argument: estimate (optional, the number of an estimate you were shown). It changes nothing.";

/// The tools of a seat in a game that has a trial (decision 0178): `trial`
/// and `trials`, each with its description and input schema.
pub fn trial_tool_definitions() -> Vec<(&'static str, String, Value)> {
    vec![
        (
            "trial",
            TRIAL_DESCRIPTION.to_string(),
            json!({
                "type": "object",
                "additionalProperties": false,
                "properties": {
                    "intent": {"type": "integer", "minimum": 1, "description": "The number of an intent of yours, in force or queued for the morning."},
                    "preview": {"type": "integer", "minimum": 1, "description": "The number of a preview you were shown, to try that draft."},
                    "size": {"type": "string", "description": "The size by its name in the trial's figures: small, standard or full; small where none is given."},
                    "compare": {"type": "boolean", "description": "Whether it is tried beside your side without the thing tried, in the same worlds. Where none is given: true for a draft of a limit or of a change, true for a new or queued order where you have another plan with an end, false otherwise."},
                    "dates": {"description": "\"chosen\", or a list of one or two days of the game after the coming morning, ascending: the order is tried at the coming morning and at those dates, and answered as a window."},
                    "describe": {"type": "boolean", "description": "With true nothing is played: the answer says what would be tried."}
                }
            }),
        ),
        (
            "trials",
            TRIALS_DESCRIPTION.to_string(),
            json!({
                "type": "object",
                "additionalProperties": false,
                "properties": {
                    "estimate": {"type": "integer", "minimum": 1, "description": "The number of an estimate you were shown, to be given it again."}
                }
            }),
        ),
    ]
}

/// The preview schema's two keys of the "targets" limit (decision 0190),
/// added apart from the `json!` that holds the rest, which is at the
/// macro's recursion limit.
fn preview_with_targets(mut schema: Value) -> Value {
    let properties = schema["properties"]
        .as_object_mut()
        .expect("the preview schema has properties");
    properties.insert(
        "kinds".to_string(),
        json!({"type": "array", "items": {"enum": ["carrier", "battleship", "cruiser", "destroyer", "transport", "auxiliary"]}, "maxItems": 6, "description": "(set_limit targets) the kinds of ship the bombers strike, most wanted first, each once; left out or empty to clear the limit."}),
    );
    properties.insert(
        "contact".to_string(),
        json!({"type": "integer", "minimum": 1, "description": "(set_intent destroy_force; required there) the number of a contact in your picture."}),
    );
    properties.insert(
        "search_only".to_string(),
        json!({"type": "boolean", "description": "(set_intent destroy_force; optional, false where left out) only search for the force and strike nothing."}),
    );
    properties.insert(
        "waters_first".to_string(),
        json!({"type": "boolean", "description": "(set_limit targets) whether the waters of the deny the intent is, or stands under, are struck first; false where left out."}),
    );
    schema
}
