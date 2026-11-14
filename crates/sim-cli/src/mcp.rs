//! `sim-cli mcp`: one game as a Model Context Protocol server over standard
//! input and output (decisions 1 to 4 of 0051).
//!
//! A thin wrapper. Every tool call goes to [`Session::call`] for the player's
//! seat and comes back as one text content item. Nothing here reads a clock,
//! a file other than the schemas compiled in, or the network, and nothing but
//! protocol messages is written to standard output.
//!
//! # Revisions
//!
//! The current revision of the specification is 2026-07-28. It has no
//! `initialize` handshake: every request carries `_meta` with the protocol
//! version and the client's capabilities, and `server/discover` stands for the
//! handshake. Earlier revisions (2025-11-25 and before) open with
//! `initialize`. The specification lets a server serve both ("dual-era"), and
//! this one does, choosing by how a request opens:
//!
//! * A request whose `params._meta` carries
//!   `io.modelcontextprotocol/protocolVersion` is served as 2026-07-28: the
//!   version must be one we serve (else error -32022 with `supported` and
//!   `requested`), `io.modelcontextprotocol/clientCapabilities` must be
//!   present (else -32602), results carry `resultType: "complete"` and the
//!   server's identity in `_meta`.
//! * `initialize` selects the legacy handshake: the answer names the client's
//!   version if it is one of 2025-11-25, 2025-06-18, 2025-03-26, 2024-11-05
//!   and otherwise 2025-11-25. After it the other requests need no `_meta`.
//! * Any other request without that `_meta`, before `initialize`, is refused
//!   with -32602 saying both ways to begin.

use std::collections::BTreeMap;

use serde_json::{json, Map, Value};
use sim_agent::{parse_seat, CallError, Session, TOOLS};
use sim_core::state::Side;

pub const MODERN: &str = "2026-07-28";
/// Legacy revisions answered to `initialize`, latest first.
pub const LEGACY: [&str; 4] = ["2025-11-25", "2025-06-18", "2025-03-26", "2024-11-05"];

const PARSE_ERROR: i64 = -32700;
const INVALID_REQUEST: i64 = -32600;
const METHOD_NOT_FOUND: i64 = -32601;
const INVALID_PARAMS: i64 = -32602;
const UNSUPPORTED_VERSION: i64 = -32022;

const META_VERSION: &str = "io.modelcontextprotocol/protocolVersion";
const META_CAPS: &str = "io.modelcontextprotocol/clientCapabilities";
const META_SERVER: &str = "io.modelcontextprotocol/serverInfo";

pub const INSTRUCTIONS: &str = concat!(
    "Flag Plot is an operational wargame of the Pacific War in 1942: you command one side's task forces and air groups by orders, and a staff carries them out on a hex map of the Solomon Islands. The game is played in days; a day has three pulses (night, morning, afternoon) that the game resolves by itself once you end the day, so you give orders for the coming day and then read what happened. You see your own units exactly and the enemy only as contacts, which are reports that can be old or wrong. Work in this order each day: call get_briefing and read it; ask list_forces, list_contacts, list_bases and get_entity for detail; use measure to find distances, routes, passage times and air reach before committing; call issue_orders with your orders for the day (an order is checked at once and answered queued or refused with the reason); then call end_day, and read the new briefing. Repeat until end_day says game_over is true. A scenario can be only one day long (the briefing then says \"Day 1 of 1\" and \"This is the last day\"): in that case you must give all your orders for the whole game before the first end_day, and that end_day ends the game. Use explain with an event number from a briefing to learn why something happened.",
    "\n\nLength and aim. The game runs for the scenario's fixed number of days: the briefing's Day line says it (\"Day 2 of 5\"), and end_day says game_over is true once the last day has been resolved, which can be after the first day of a short scenario. A scenario may give your side a mission and objectives, and then the briefing's OBJECTIVES part states them, with the points each is worth, what you can know of your progress and the levels (the margin of your points over the enemy's that makes a minor, a major or a decisive victory; a margin smaller than the minor level either way is a stalemate); much of the enemy's losses you cannot see until the end, so the briefing says where your progress is not observable. What you learn of your own attacks comes as claims, listed in the briefing under Claims yesterday: after a fight at sea or a raid, the number of enemy ships your people believe they sank, the hits they believe they scored and the kind of ship they believed they attacked, the aircraft they believe they shot down and the aircraft they believe they destroyed on the ground. A claim is what your people believe, not the truth: it is never below what really happened and can be well above it, more the more your side fired, and it is never corrected later. Use explain on a claim's event number to see the firing of your own that it rests on. When the game is over the final briefing and the answer of end_day carry the result: each side's points by objective, the margin and the outcome in words (for example \"minor victory\" or \"stalemate\"; a stalemate has no level). The result also carries a history note: it says what happened in the real battle, not in your game. A scenario without objectives has no result; then set your own aim from the briefing (keep your ships and aircraft, hurt the enemy, supply your bases) and judge it by what the briefings report.",
    "\n\nMissions are standing orders, and the staff acts on them inside the day. Search, the strike decision and the strike itself all happen within the pulses of one day, so you cannot see a contact in the briefing and then order a strike at it: set the missions BEFORE the contact appears. A group flies only the mission it is on, and only with aircraft that are ready, have a pilot (where pilots are counted) and a usable flight deck. An air group on naval_strike is launched by the staff, in the same pulse and with no further order from you, in a daylight pulse (morning or afternoon), at an enemy contact your side holds, reported in that pulse or the one before, that lies within the aircraft type's strike radius (the kind first on its target list, which is the most important kind first until you order set_target_priority, then the nearest); with no such contact, or none of a kind on its list, it launches nothing. The briefing's line for a naval_strike group with ready aircraft says which contact it would strike if that decision were made now (would_strike); a contact that is not sighted again may be too old by the morning. On base_strike {target_base: N} it launches at that enemy base, needing no contact, in the first daylight pulse of the day (morning, else afternoon) in which the base is within the radius, and once a day; its bombs can hit aircraft on the ground and supply there. On combat_air_patrol (fighters) a share of the group stays over its own hex as patrol, defending against raids, and the rest escort raids from that place within the fighters' radius; set_patrol_share {share} is the share of the group kept over its own hex: 1 (where it starts) keeps every fighter at home and sends no escort, and 0.5 keeps half at home and lets half escort. On search it flies its arc in the daylight pulses (set_search_arc: bearings are multiples of 10 from 0 to 350, 0 is north and the arc runs clockwise from from_deg to to_deg, range_nm up to the type's search range) and sights enemy task forces in it; a group on search with no arc set flies nothing. On rest it does nothing. A usual pairing is a scouting group on search and a strike group on naval_strike, ordered before the enemy is found. The briefing's air group lines give each type's radius (the distance within which it can strike, or for a fighter escort) and its search range, in miles.",
    "\n\nThe day and its pulses. A day is three pulses: night, morning and afternoon. The night pulse lasts 12 hours and begins at 18:00 on the evening before the day (so the game's clock reads, for day 1, \"D1 18:00\" at its start); the morning pulse is 06:00 to 12:00 and the afternoon pulse 12:00 to 18:00, under the same D number. Searches and strikes fly only in the morning and afternoon pulses; the afternoon pulse is the last of the day in which a group can fly, and nothing can be launched at night. What a contact is: the kind it is taken for and the number of ships are what the sighting reported. They can be wrong, and can change when the contact is seen again, so a contact taken for a battleship may be a carrier or a cruiser; a strike makes its own report on what it finds.",
    "\n\nWhat the briefing's figures mean. An air group's pilots of null mean the scenario does not count that group's pilots: no pilot limit applies and every ready aircraft can fly; a number is a limit on how many fly. Where pilots are counted, a fighter shot down on patrol over its own base or ships does not always keep its pilot: each such pilot is recovered by chance or lost, as the pilots lost in your briefing show. If a carrier of yours sinks, or her flight deck goes out, while her aircraft are in the air in a pulse, they land at another carrier of yours in the hex that has room, else at a base of yours within the aircraft type's radius (a scout's search range), forming a new air group there with the next free number (it rests, and is shown in the next briefing), else they come down in the sea and their crews are recovered by chance. Aircraft that were aboard when she sank are lost with her. A group's arrival \"due day N\" joins the group during the night pulse that begins day N, before that day's morning, so it is still listed in the briefing for day N and is in the group, ready to fly, in the briefing for day N+1. A base's airfield size and port size come from the map and are 0 where the map gives none: no rule reads the airfield size, and the port size only sets how fast cargo is worked (size 0 means loading and unloading by lighter), so a base of size 0 still launches and receives aircraft and ships. A task force that cannot move (a ship of it has propulsion out, so the force has no speed) or is slowed (a ship has it degraded) is flagged on its line and in the warnings, whatever its status says. No event names an enemy ship, task force or air group: a raid, an attack or a fight at sea says what your own units did and what was done to them, and the enemy in it is known only by the contacts and claims you hold. An enemy base is named, as it is on the map; get_entity answers \"not known\" for every enemy ship, task force and air group.",
    "\n\nGround. Where your side has land units, you attack and hold ground with set_posture {posture: attack or defend} (unit \"land_unit:N\"). A unit on attack fights the enemy's men in its hex once a day, after the third pulse; a unit on defend fires only when the enemy attacks. Hunger and fatigue cut the men who fire, and a unit that has become too tired goes back to defend on its own. A side left with none of its men in the hex, or too few against the attacker's, loses every base it holds there, with the air groups at it, a ship in its port and part of its stocks; a base with no man of its side ashore falls to one man on attack in its hex. Fly your air groups out by rebase a day before. The briefing and the tool descriptions give the rest.",
    "\n\nShells and bombs ashore, and what you know of the enemy's men. To bombard, set a task force's mission to bombardment (set_mission {mission: bombardment}) and end the night's move in the hex of an enemy base that is on your map (list_bases shows the enemy bases you know of). In the night its ships fire their guns at the base, spending rounds from their magazines, and a heavy gun counts for more than a light one. The shells hit what is at the base: aircraft on the ground, supply and men ashore, or nothing. You are told your own shells and are given a claim of aircraft destroyed on the ground, never the true hits; the claim can be above the truth. A base that is not on your map cannot be bombarded or struck until it is found, and a ground report can find one. The enemy is told its base was shelled and what was hit, and that ships were there, but not which. Bombs from the air now also hit men ashore, by the same rule. You never see the enemy's men ashore, only an estimate: each day, in each hex where both sides have men ashore, your side is given a report of the enemy's men there, wrong by up to half either way (a report may say 600 where there are 1000, or 1400). The briefing's ENEMY MEN ASHORE part and list_contacts give your estimate of each hex, the day it was reported and how old it is; a new report replaces the old one, and an estimate is never corrected otherwise. A scenario may give your side an opening estimate of the enemy's men, dated day 0, which can be far wrong. Plan an attack on an estimate with care: it can be too high or too low.",
);

// ---------------------------------------------------------------------
// Tool descriptions and input schemas.
// ---------------------------------------------------------------------

const ISSUE_ORDERS_DESCRIPTION: &str = "Queue orders for your side for the coming day. Each order is {\"unit\": \"task_force:N\", \"air_group:N\" or \"land_unit:N\", \"order\": {\"kind\": ..., ...}}; the handles are those the briefing shows. The answer says, for each order in the order given, \"queued\" or \"refused\" with the reason (the engine's own check, made against the state as it is now), and counts them (queued_in_call, refused_in_call, and a summary sentence). A refused order is not queued but the other orders in the call stand, including other orders for the same unit, so check each result. An order can still be refused when the day begins if the state has changed; the next briefing says so. \"clear\": true empties your queue first, so a call with {\"orders\": [], \"clear\": true} withdraws everything. \"reason\": \"...\" says in your own words why you are giving these orders; give one with every call, since it is kept in the record of the game for whoever reads it afterwards (the game itself never reads it). Orders stand after the day ends until another replaces them. Order kinds for a task force (unit \"task_force:N\"): set_route {waypoints: [[q, r], ...]} (hexes to steam to, in order); set_speed {speed: \"cruise\" or \"full\"} or {knots: \"17\"}; set_mission {mission: transport, surface_combat, carrier, bombardment or escort}; load {cargo: supply or fuel} or, to take a land unit's men aboard, load {unit: \"land_unit:N\", cargo: supply, fuel or none (default supply)}; unload {}; set_rules_of_engagement {stance: engage or avoid, break_off: decimal, stragglers: keep, detach, leave or retire, keep_share: decimal}; set_home_port {base: N}; detach {ships: [ship numbers]}; set_readiness {readiness: normal or battle}; send_to_yard {} (no other key). BOMBARDMENT. set_mission {mission: bombardment} is a standing order. A task force on it that ends the night's move in the hex of an enemy base on your map bombards that base in that night, each ship firing its guns and spending rounds (the lowest-numbered base there if there are two); a base not on your map cannot be bombarded, and a ship with no rounds left fires nothing. Its shells hit aircraft, supply and men at the base. You are told your shells and given a claim of aircraft destroyed on the ground, never the hits. SEND TO THE YARD. A task force stopped at a base of yours that is a port for the yard can be sent to the yard: its ships leave the map and the force is away, taking no order, until its day of return, when it is back at the base repaired, refuelled and rearmed. The days away are the base's passage plus the largest days of repair of any ship, worked from the ships' damage as it stands, with no chance in it; the answer to a queued send_to_yard says \"day_of_return\" and \"returns\" (false if that day is after the game's last day, when the force does not come back before the game ends). `measure` question yard gives the same days before you order. The briefing lists the forces away and the day each is back. Order kinds for an air group (unit \"air_group:N\"): set_air_mission {mission: rest, search, naval_strike or combat_air_patrol} or {mission: \"base_strike\", target_base: N}; set_search_arc {from_deg, to_deg, range_nm: decimal}; set_patrol_share {share: decimal}; set_rest_threshold {fatigue: decimal}; set_reach {reach: normal or extended}; set_target_priority {kinds: [carrier, battleship, cruiser, destroyer, transport or auxiliary, first choice first]}; rebase {base: N}. REBASE. rebase moves the whole air group to another base of your side within its transfer range (twice the longer of its aircraft type's search range and strike radius; `measure` air_reach gives it). It is not a standing order: it is done at the start of the day and spent. The group flies nothing that day, its damaged aircraft are written off at the base it leaves, and it flies its mission from the new base from the next day. It is refused for a group aboard a ship. TARGET PRIORITY. A naval_strike group has a target list, every kind in the order carrier, battleship, cruiser, destroyer, transport, auxiliary until you order otherwise, and the order stands until you replace it. It names at least one kind and none twice. The staff launches the group only at a contact taken for a kind on the list: the kind earliest on the list first, then the nearest. A contact of any other kind is never launched at, so a short list holds the strike back (a group whose list is just carrier stays on deck until a carrier is reported in reach, and the briefing warns that it is holding when it holds contacts in reach and none is of a listed kind). A report can name the wrong kind and sends the strike wrong. Over the target the group attacks what it finds: the raid attacks the enemy force at its hex that holds the ship ranking first by the list of the raid's first strike group, and each group attacks the ship of that force ranking first by its own list, kinds not on the list following in the order of importance, so two groups of one raid can attack different ships. REACH. A group has a reach, normal until you order otherwise, and the order stands until you replace it. On normal reach a strike or an escort goes only as far as the aircraft type's strike radius (the briefing shows each group's strike_radius_nm). On extended reach it goes a share further (the briefing shows the reach in force as reach_nm when it is extended; `measure` air_reach gives the radius, the reach in force and the chance an aircraft runs out of fuel at a distance): a naval_strike group strikes a contact within that, a base_strike group a base within that, and a fighter group sends its escorts with a raid within that. The price is paid on the way home: each aircraft that came back from a strike or escort sortie whose one-way distance was beyond the radius is rolled for running out of fuel, with a chance that rises with the distance beyond the radius, and an aircraft that runs out of fuel is lost for good; where pilots are counted, the crew of each such aircraft may be recovered or lost. Nothing is rolled for a sortie within the radius, for a search or for a patrol. WHAT AN AIR MISSION DOES. A mission is a standing order the staff acts on inside the day: search, the strike decision and the strike itself all happen within one day's pulses, so you cannot see a contact in a briefing and then order a strike at it; set missions BEFORE the contact appears. A group flies only the mission it is on. naval_strike: the staff launches the group, in the same pulse and with no further order from you, in a daylight pulse (morning or afternoon), at an enemy contact your side holds (reported in that pulse or the one before) that lies within the aircraft type's strike radius (the kind first on its target list, which is the most important kind first until you order otherwise, then the nearest); with no such contact it launches nothing and the aircraft stay on the ground. base_strike {target_base: N}: in the first daylight pulse of a day (morning, else afternoon) in which that enemy base is within the radius, once a day, the group launches at it, needing no contact; its bombs can hit aircraft on the ground and supply there. combat_air_patrol (fighters): a share of the group stays over its own hex as patrol and the rest escort raids from that place within the fighters' radius; set_patrol_share {share} is the share of the group kept over its own hex: 1 (where it starts) keeps every fighter at home and sends no escort, 0.5 keeps half at home and lets half escort, 0 keeps none at home and sends all as escort. search: the group flies its search arc in the daylight pulses and may sight enemy task forces in it; with no arc set it flies nothing. rest: the group does nothing. LOAD WITH A LAND UNIT. load {unit: \"land_unit:N\"} takes the men of your land unit ashore in the force's hex aboard first, up to the ships' troop berths, then the named cargo (\"none\" takes the men only). It is accepted where you have no base, and then it loads men only; it is refused if the unit is not yours, has no men ashore where the force is, or no ship of the force can carry men. An unload {} lands the men aboard; the first landing in a hex where you hold no base forms a base there, known to your side only. Men ashore are fed from a base of yours in their hex, and are lost when unfed. FIGHT ASHORE. A land unit (unit \"land_unit:N\") takes one order, set_posture {posture: attack or defend}; it stands until another replaces it, and it is refused if the unit is not yours or has no man ashore. A unit starts on defend. Once a day, after the third pulse, a hex where both sides have men ashore and a unit of one side is on attack is fought over: that side attacks with all its units there, and both sides' men fire on the strengths they had at the start. A unit on attack with no enemy men in its hex does nothing and is not tired. Unfed men and tired men fire less: the men who fire are the men ashore, cut by the share of them fed at the last night's charge and by the unit's fatigue. A unit on attack that fires gains fatigue, any other unit ashore rests, and a unit on attack too tired to go on goes back to defend and its order is spent, so order attack again after it has rested. A side that was attacked and is left with no man ashore in the hex, or with less than the rule's share of the attacker's men, is beaten there: it loses every base it holds in the hex. The base changes side: the air groups at it are lost whole, a ship in its port is lost, and the base keeps only part of its supply and fuel. A base with no man of its side ashore falls to a single man of yours on attack in its hex, with no fight. So fly your air groups out by rebase a day before a base can fall; one still there when it falls is lost. Both sides are told when a base changes side; you see your own units' posture, fatigue and fed share exactly and the fight only as your own men's firing, never the enemy's men or hits. LOAD AND UNLOAD are standing cargo orders for a task force, and they work only while the force is stopped (idle or arrived) in the hex of a base of your own side. load {cargo: supply or fuel} needs exactly that: it is checked against the state now when you give it, and again when the day begins, with the day's orders taken in the order given, and it is refused if the force is then under way (a set_route given earlier in the same day sends it under way, so a load in the same day as a sailing is refused) or is not in the hex of a base of your own side. The engine's words for the first are \"the task force is under way\" and for the second \"there is no base of your side here\"; the refusal in your answer adds what the order needs. unload {} is never refused, wherever the force is, but it moves cargo only in the pulses in which the force is stopped at a base of your own side, so an unload given while the force is at sea is queued, stands as the force's cargo order, and is carried out after the force arrives and stops at such a base (a later load, unload or similar cargo order replaces it); at any other place it does nothing. Loading and unloading move supply or fuel between the ships and the base's stock at the port's rate, a load of fuel cargo is not fuel for the ships' own tanks. SEARCH ARCS. Bearings are in degrees, multiples of 10 from 0 to 350; 0 is north (up the map, toward smaller r), 90 east, 180 south, 270 west; the arc runs clockwise from from_deg to to_deg, so from_deg 270 to_deg 90 is the northern half, equal bearings mean the whole circle, and arcs over 180 degrees are allowed. range_nm is in miles, above 0 and at most the aircraft type's search range (a longer one is refused and the refusal names the maximum). Every quantity is a decimal string such as \"0.5\" and every count a whole number; the input schema lists the fields of each kind exactly. Example: {\"orders\": [{\"unit\": \"task_force:1\", \"order\": {\"kind\": \"set_route\", \"waypoints\": [[5, -4], [3, -4]]}}, {\"unit\": \"air_group:1\", \"order\": {\"kind\": \"set_air_mission\", \"mission\": \"combat_air_patrol\"}}], \"reason\": \"Close the anchorage by night and keep fighters over the carrier.\"}. Use it after reading the briefing and before end_day; you cannot order after end_day until the day has been resolved.";

const MEASURE_DESCRIPTION: &str = "Ask the game a question about distance, route, passage time, a force's days at the yard or air reach, answered with the functions the rules themselves use, so you can check a plan before ordering it. Give one question as the arguments; \"question\" names it. A place is {\"q\": 2, \"r\": -4} (a hex), or the handle of one of your own ships, task forces, air groups, bases or land units, or of a contact you hold (which stands for the hex you believe it is in). \"air_distance\" {from, to}: the straight-line air distance in whole miles. \"sea_route\" {from, to}: whether ships can get there, the hexes entered in order and the miles. \"passage\" {task_force: \"task_force:N\", to: place, speed (optional): \"cruise\", \"full\" or knots such as 17}: a forecast of that force's passage, the route, speed, hours, the day, pulse and clock time of arrival and each ship's fuel; it assumes the force leaves at the start of the coming day, meets nobody and keeps its speed, and says \"refused\" if the engine would refuse the route order. The passage \"leaves\" at the start of the coming day's night pulse, which is 18:00 on the evening before the day (the clock reads \"D1 18:00\" for the start of day 1; that day's morning is \"D1 06:00\" and its afternoon \"D1 12:00\", the same D number until 18:00), so a force arrives in the pulse and at the clock time the answer gives. \"yard\" {task_force: \"task_force:N\"}: what a send_to_yard order given now would do: whether the engine would take it (order accepted or refused, and the reason), the port and its passage in days, for each ship afloat her flotation damage, systems degraded and out and her days of repair, the days away (the passage plus the largest days of repair), the day it leaves (the coming day), the day it is back (return_day) and returns_before_end with the game's last day; for a force already away it gives its port and the day it is back. No random draw is involved: the answer is the days the order will state. \"air_reach\" {air_group: \"air_group:N\", to: [places]}: the aircraft type's radius, the reach in force (normal or extended) and its length in miles, the transfer range a rebase order can cover, and, for each place, the distance, whether it is within the reach in force (the test a strike decision makes), whether it is within the radius, and the chance an aircraft that came back would run out of fuel at that distance; to is a list of places, and one place alone is taken as a list of one. Anything you do not know of (an enemy unit, a number that does not exist, a wrong kind of unit, a hex that is off the map) answers \"not known\", and one such place anywhere in the question makes the whole answer \"not known\", so a list with one hex off the map gives \"not known\" for all of them; a question that is not in the shapes above is refused as an error that says what was expected. Nothing is changed by asking. Example: {\"question\": \"passage\", \"task_force\": \"task_force:1\", \"to\": {\"q\": 2, \"r\": -4}, \"speed\": \"full\"}.";

fn tool_definitions() -> Vec<(&'static str, String, Value)> {
    let area = json!({
        "type": "object",
        "description": "Only items within radius_nm miles of the hex (q, r).",
        "required": ["q", "r", "radius_nm"],
        "additionalProperties": false,
        "properties": {
            "q": {"type": "integer"},
            "r": {"type": "integer"},
            "radius_nm": {"type": "integer", "minimum": 0}
        }
    });
    let area_only = json!({
        "type": "object",
        "additionalProperties": false,
        "properties": {"area": area}
    });
    let place = json!({
        "description": "A hex {\"q\": 2, \"r\": -4}, or a handle: your own \"ship:N\", \"task_force:N\", \"air_group:N\", \"base:N\" or \"land_unit:N\", or a contact \"contact:N\" you hold.",
        "oneOf": [
            {"type": "string"},
            {
                "type": "object",
                "required": ["q", "r"],
                "additionalProperties": false,
                "properties": {"q": {"type": "integer"}, "r": {"type": "integer"}}
            }
        ]
    });
    // Flat on purpose: some clients refuse a top-level oneOf in a tool's
    // input schema. Which fields go with which question is in the description.
    let measure = json!({
        "type": "object",
        "required": ["question"],
        "additionalProperties": false,
        "properties": {
            "question": {
                "enum": ["air_distance", "sea_route", "passage", "yard", "air_reach"],
                "description": "air_distance and sea_route take from and to; passage takes task_force, to and optionally speed; yard takes task_force; air_reach takes air_group and to (a list of places)."
            },
            "from": place,
            "to": {
                "description": "A place; for air_reach, a list of places (or one place, taken as a list of one). One place that is not known, such as a hex off the map, makes the whole answer \"not known\".",
                "oneOf": [place, {"type": "array", "items": place}]
            },
            "task_force": {"type": "string", "description": "\"task_force:N\" (passage, yard)."},
            "air_group": {"type": "string", "description": "\"air_group:N\" (air_reach)."},
            "speed": {
                "description": "(passage) \"cruise\", \"full\", or knots above zero, 17 or \"17.5\".",
                "oneOf": [
                    {"enum": ["cruise", "full"]},
                    {"type": "number", "exclusiveMinimum": 0},
                    {"type": "string"}
                ]
            }
        }
    });
    vec![
        (
            "get_briefing",
            "Your side's situation at the start of the day: what your orders did yesterday, new and dropped contacts, your estimates of the enemy's men ashore, fights and shore bombardments, losses, your task forces, air groups and bases with their handles, arrivals due, and warnings (including task forces that cannot move or are slowed, and strike groups that hold contacts or a target but have none within their radius). Task force lines show the cargo aboard, air group lines each type's radius and search range in miles, and a base that took or gave supply or fuel yesterday shows its stocks yesterday by cause. Its heading says the day and the game's last day, such as \"Day 2 of 5\", and says so in a plain line when the day is the last. Its OBJECTIVES part gives your side's mission and objectives, if the scenario has them, the levels that make a minor, major or decisive victory, and, once the game is over, the result for both sides and a history note of what happened in the real battle (not in your game). It answers as readable text by default; call it with {\"text\": false} for the same briefing as data. Read it first every day, and again after end_day. Example arguments: {} or {\"text\": false}.".to_string(),
            json!({
                "type": "object",
                "additionalProperties": false,
                "properties": {
                    "text": {"type": "boolean", "description": "true (the default here) for the briefing as text, false for data."}
                }
            }),
        ),
        (
            "list_forces",
            "One line for each of your land units (handle, men ashore and the hex, men aboard ships) and one for each of your task forces: its handle, place, status, mission, speed, destination, readiness, ships afloat, least-fuelled ship, the cargo it carries (cargo_supply_t and cargo_fuel_t in tons, shown when it carries any or its mission is transport) and whether it can move (movement: normal, slowed or cannot_move, with the ships whose propulsion is damaged). Use it for a quick roll call. Optional {\"area\": {\"q\": 7, \"r\": -5, \"radius_nm\": 120}} keeps only forces within that distance of the hex. Example arguments: {}.".to_string(),
            area_only.clone(),
        ),
        (
            "list_contacts",
            "The enemy contacts your side holds, each with its handle, where you believe it is, how old the report is, how many ships and what the most important of them is taken for. You see the enemy only as contacts: reports that can be old, wrong or missing. The kind (\"taken_for\") and the count are what the sighting reported; they can be wrong, and can change when the contact is seen again. Use it before deciding where to search or strike. It also lists your estimates of the enemy's men ashore, one for each hex where you hold a report (ground_estimates: the men, the day reported and the age in days). An estimate is from a ground report and can be wrong by up to half either way; it is never corrected until a new report replaces it. Optional {\"area\": {\"q\", \"r\", \"radius_nm\"}} keeps only contacts within that distance of the hex. Example arguments: {}.".to_string(),
            area_only.clone(),
        ),
        (
            "list_bases",
            "Your own bases (handle, place, airfield and port size, supply and fuel; the sizes come from the map and are 0 where it gives none: no rule reads the airfield size and the port size only sets how fast cargo is worked, so a base of size 0 still launches and receives aircraft and ships) and the enemy's bases by number, name, place and holder only: where every base is is on the map, not intelligence. Use it to find a base number for a base_strike or a home port. Optional {\"area\": {\"q\", \"r\", \"radius_nm\"}}. Example arguments: {}.".to_string(),
            area_only,
        ),
        (
            "get_entity",
            "Everything your side knows of one unit or contact: a ship, task force, air group, base, land unit or contact, named by its handle. Your own units are known exactly; an enemy unit only as the contact you hold, and for a contact the kind it is \"taken for\" and the ship count are what the sighting reported: they can be wrong, and can change when the contact is seen again. An air group's pilots of null mean the scenario does not count its pilots, so no pilot limit applies. A handle you do not know of, an enemy unit you hold no contact for or a number that does not exist all answer \"not known\" alike. Example arguments: {\"handle\": \"task_force:1\"}.".to_string(),
            json!({
                "type": "object",
                "required": ["handle"],
                "additionalProperties": false,
                "properties": {
                    "handle": {"type": "string", "description": "Such as \"task_force:3\", \"ship:9\", \"air_group:2\", \"base:1\", \"land_unit:4\" or \"contact:4\"."}
                }
            }),
        ),
        ("measure", MEASURE_DESCRIPTION.to_string(), measure),
        (
            "explain",
            "Why something happened: an event as your side may see it and the visible events and orders that caused it, with the figures behind each. Give the number of an event the briefing showed you. An event you were not shown answers \"not known\". Use it to learn why a strike missed, a ship was lost or an order was refused. Example arguments: {\"event\": 42}.".to_string(),
            json!({
                "type": "object",
                "required": ["event"],
                "additionalProperties": false,
                "properties": {
                    "event": {"type": "integer", "minimum": 0, "description": "The number of an event shown in a briefing."}
                }
            }),
        ),
        ("issue_orders", ISSUE_ORDERS_DESCRIPTION.to_string(), issue_orders_schema()),
        (
            "end_day",
            "Say you have finished giving orders for the coming day. The game then resolves the day (night, morning and afternoon pulses): your queued orders and the other side's are carried out, and you may read a new briefing. The answer says whether the day was resolved, how many days have been resolved and whether the game is over; when game_over is true there is nothing more to order, though you can still read, and the answer carries the result (points by objective, margin and level for both sides) if the scenario has objectives, else null. Call it once per day after issue_orders; orders cannot be given between end_day and the next day. Example arguments: {}.".to_string(),
            json!({"type": "object", "additionalProperties": false}),
        ),
    ]
}

// The schema files, compiled in. `issue_orders`' input schema is the
// arguments schema with every `$ref` replaced by what it points at, because a
// client cannot be expected to fetch `order.schema.json` or resolve
// `defs.schema.json#/$defs/dec`.
const ARGUMENTS_SCHEMA: &str =
    include_str!("../../sim-agent/schemas/issue_orders_arguments.schema.json");
const ORDER_SCHEMA: &str = include_str!("../../sim-agent/schemas/order.schema.json");
const DEFS_SCHEMA: &str = include_str!("../../sim-agent/schemas/defs.schema.json");

fn issue_orders_schema() -> Value {
    let docs: BTreeMap<&str, Value> = [
        (
            "issue_orders_arguments.schema.json",
            serde_json::from_str(ARGUMENTS_SCHEMA).expect("the arguments schema is JSON"),
        ),
        (
            "order.schema.json",
            serde_json::from_str(ORDER_SCHEMA).expect("the order schema is JSON"),
        ),
        (
            "defs.schema.json",
            serde_json::from_str(DEFS_SCHEMA).expect("the defs schema is JSON"),
        ),
    ]
    .into_iter()
    .collect();
    let root = &docs["issue_orders_arguments.schema.json"];
    let mut out = inline(root, "issue_orders_arguments.schema.json", &docs, 0);
    if let Some(map) = out.as_object_mut() {
        for key in ["$schema", "$id", "$defs"] {
            map.remove(key);
        }
    }
    out
}

/// Replaces every `$ref` of `value` (a part of the document `doc`) by its
/// target, resolved against the files compiled in.
fn inline(value: &Value, doc: &str, docs: &BTreeMap<&str, Value>, depth: u32) -> Value {
    assert!(depth < 32, "the schema files refer to themselves");
    match value {
        Value::Object(map) => {
            if let Some(Value::String(target)) = map.get("$ref") {
                let (file, pointer) = target.split_once('#').unwrap_or((target, ""));
                let file = if file.is_empty() { doc } else { file };
                let root = docs
                    .get(file)
                    .unwrap_or_else(|| panic!("the schema {file} is not compiled in"));
                let part = if pointer.is_empty() {
                    root
                } else {
                    root.pointer(pointer)
                        .unwrap_or_else(|| panic!("{file} has no {pointer}"))
                };
                let mut resolved = inline(part, file, docs, depth + 1);
                if pointer.is_empty() {
                    if let Some(m) = resolved.as_object_mut() {
                        for key in ["$schema", "$id", "$defs"] {
                            m.remove(key);
                        }
                    }
                }
                // Keywords beside the `$ref` (a description of the field)
                // stay, over what the reference brings.
                if let Some(m) = resolved.as_object_mut() {
                    for (key, v) in map {
                        if key != "$ref" {
                            m.insert(key.clone(), inline(v, doc, docs, depth));
                        }
                    }
                }
                return resolved;
            }
            let mut out = Map::new();
            for (key, v) in map {
                out.insert(key.clone(), inline(v, doc, docs, depth));
            }
            Value::Object(out)
        }
        Value::Array(items) => {
            Value::Array(items.iter().map(|v| inline(v, doc, docs, depth)).collect())
        }
        other => other.clone(),
    }
}

/// The tools as `tools/list` gives them, in the order of [`TOOLS`] (which is
/// the interface's own order; it never varies).
pub fn tool_list() -> Vec<Value> {
    tool_definitions()
        .into_iter()
        .map(|(name, description, schema)| {
            json!({"name": name, "description": description, "inputSchema": schema})
        })
        .collect()
}

// ---------------------------------------------------------------------
// The server.
// ---------------------------------------------------------------------

/// One game, one seat for the player, and the protocol state: whether a legacy
/// `initialize` has been seen.
pub struct Server {
    pub session: Session,
    side: Side,
    initialized: bool,
}

fn rpc_error(id: Value, code: i64, message: &str, data: Option<Value>) -> String {
    let mut error = json!({"code": code, "message": message});
    if let Some(data) = data {
        error["data"] = data;
    }
    serde_json::to_string(&json!({"jsonrpc": "2.0", "id": id, "error": error}))
        .expect("an answer serialises")
}

fn rpc_result(id: Value, result: Value) -> String {
    serde_json::to_string(&json!({"jsonrpc": "2.0", "id": id, "result": result}))
        .expect("an answer serialises")
}

fn server_info() -> Value {
    json!({"name": "flag-plot", "title": "Flag Plot", "version": env!("CARGO_PKG_VERSION")})
}

fn tool_text(text: String, is_error: bool) -> Value {
    json!({"content": [{"type": "text", "text": text}], "isError": is_error})
}

impl Server {
    pub fn new(session: Session, side: Side) -> Server {
        Server {
            session,
            side,
            initialized: false,
        }
    }

    /// The answer to one line of input: `None` for a notification (and for a
    /// response, which a server has no use for), else one line of output.
    pub fn handle_line(&mut self, line: &str) -> Option<String> {
        let message: Value = match serde_json::from_str(line) {
            Ok(v) => v,
            Err(e) => {
                return Some(rpc_error(
                    Value::Null,
                    PARSE_ERROR,
                    &format!("parse error: {e}"),
                    None,
                ))
            }
        };
        let Some(object) = message.as_object() else {
            return Some(rpc_error(
                Value::Null,
                INVALID_REQUEST,
                "a message is one JSON object (batches are not accepted)",
                None,
            ));
        };
        let id = object.get("id").cloned();
        if object.get("jsonrpc") != Some(&json!("2.0")) {
            return Some(rpc_error(
                id.unwrap_or(Value::Null),
                INVALID_REQUEST,
                "\"jsonrpc\" must be \"2.0\"",
                None,
            ));
        }
        let Some(method) = object.get("method").and_then(Value::as_str) else {
            // A response from the client: nothing is waiting for one.
            if object.contains_key("result") || object.contains_key("error") {
                return None;
            }
            return Some(rpc_error(
                id.unwrap_or(Value::Null),
                INVALID_REQUEST,
                "\"method\" is needed",
                None,
            ));
        };
        let Some(id) = id else {
            // A notification: no answer, whatever it says.
            return None;
        };
        if !(id.is_string() || id.is_i64() || id.is_u64()) {
            return Some(rpc_error(
                Value::Null,
                INVALID_REQUEST,
                "\"id\" must be a string or an integer",
                None,
            ));
        }
        let params = object.get("params").cloned().unwrap_or(Value::Null);
        if !(params.is_null() || params.is_object()) {
            return Some(rpc_error(
                id,
                INVALID_PARAMS,
                "\"params\" must be an object",
                None,
            ));
        }
        Some(match self.request(method, &params) {
            Ok(result) => rpc_result(id, result),
            Err((code, message, data)) => rpc_error(id, code, &message, data),
        })
    }

    #[allow(clippy::type_complexity)]
    fn request(
        &mut self,
        method: &str,
        params: &Value,
    ) -> Result<Value, (i64, String, Option<Value>)> {
        let err = |code: i64, message: &str| Err((code, message.to_string(), None));
        let meta = params.get("_meta");
        let modern = meta.and_then(|m| m.get(META_VERSION));
        let modern = match modern {
            None => false,
            Some(Value::String(version)) if version == MODERN => {
                if meta.and_then(|m| m.get(META_CAPS)).is_none() {
                    return err(
                        INVALID_PARAMS,
                        "_meta needs io.modelcontextprotocol/clientCapabilities",
                    );
                }
                true
            }
            Some(other) => {
                let mut supported = vec![MODERN];
                supported.extend(LEGACY);
                return Err((
                    UNSUPPORTED_VERSION,
                    "Unsupported protocol version".to_string(),
                    Some(json!({"supported": supported, "requested": other})),
                ));
            }
        };
        // Results of the modern revision say they are complete and who we are.
        let finish = |mut result: Value| {
            if modern {
                result["resultType"] = json!("complete");
                result["_meta"] = json!({ META_SERVER: server_info() });
            }
            Ok(result)
        };
        match method {
            "initialize" => {
                let asked = params.get("protocolVersion").and_then(Value::as_str);
                let version = match asked {
                    Some(v) if LEGACY.contains(&v) => v,
                    _ => LEGACY[0],
                };
                self.initialized = true;
                Ok(json!({
                    "protocolVersion": version,
                    "capabilities": {"tools": {"listChanged": false}},
                    "serverInfo": server_info(),
                    "instructions": INSTRUCTIONS,
                }))
            }
            "ping" => finish(json!({})),
            "server/discover" => {
                let mut supported = vec![MODERN];
                supported.extend(LEGACY);
                let mut result = json!({
                    "supportedVersions": supported,
                    "capabilities": {"tools": {"listChanged": false}},
                    "instructions": INSTRUCTIONS,
                });
                result["resultType"] = json!("complete");
                result["_meta"] = json!({ META_SERVER: server_info() });
                Ok(result)
            }
            "tools/list" | "tools/call" => {
                if !modern && !self.initialized {
                    return err(
                        INVALID_PARAMS,
                        "send initialize first, or put io.modelcontextprotocol/protocolVersion (2026-07-28) and io.modelcontextprotocol/clientCapabilities in params._meta",
                    );
                }
                if method == "tools/list" {
                    finish(json!({"tools": tool_list()}))
                } else {
                    let result = self.call_tool(params)?;
                    finish(result)
                }
            }
            other => Err((METHOD_NOT_FOUND, format!("method not found: {other}"), None)),
        }
    }

    #[allow(clippy::type_complexity)]
    fn call_tool(&mut self, params: &Value) -> Result<Value, (i64, String, Option<Value>)> {
        let bad = |message: String| Err((INVALID_PARAMS, message, None));
        let Some(name) = params.get("name").and_then(Value::as_str) else {
            return bad("\"name\" is needed: the tool to call".to_string());
        };
        if !TOOLS.contains(&name) {
            return bad(format!("Unknown tool: {name}"));
        }
        let mut arguments = match params.get("arguments") {
            None | Some(Value::Null) => json!({}),
            Some(value @ Value::Object(_)) => value.clone(),
            Some(_) => return bad("\"arguments\" must be an object".to_string()),
        };
        // The briefing is text unless asked for data (decision 3).
        let mut as_text = false;
        if name == "get_briefing" {
            let map = arguments.as_object_mut().expect("arguments is an object");
            match map.get("text") {
                None | Some(Value::Null) => {
                    map.insert("text".to_string(), json!(true));
                    as_text = true;
                }
                Some(Value::Bool(b)) => as_text = *b,
                Some(_) => {}
            }
        }
        Ok(match self.session.call(self.side, name, &arguments) {
            Ok(answer) => {
                let text = match (as_text, answer.get("text").and_then(Value::as_str)) {
                    (true, Some(text)) => text.to_string(),
                    _ => serde_json::to_string_pretty(&answer).expect("an answer serialises"),
                };
                tool_text(text, false)
            }
            Err(error) => tool_text(error_text(&error), true),
        })
    }
}

fn error_text(error: &CallError) -> String {
    if error.allowed.is_empty() {
        error.message.clone()
    } else {
        format!("{} (one of: {})", error.message, error.allowed.join(", "))
    }
}

// ---------------------------------------------------------------------
// The command.
// ---------------------------------------------------------------------

struct Options<'a> {
    scenario: &'a str,
    side: Side,
    seed: Option<u64>,
    save: Option<&'a str>,
    transcript: Option<&'a str>,
}

fn parse<'a>(args: &'a [String], usage: &str) -> Result<Options<'a>, String> {
    let mut scenario = None;
    let mut side = None;
    let mut seed = None;
    let mut save = None;
    let mut transcript = None;
    let mut words = args.iter().map(String::as_str);
    while let Some(word) = words.next() {
        let mut value = |what: &str| {
            words
                .next()
                .ok_or_else(|| format!("{what} needs a value\n\n{usage}"))
        };
        match word {
            "--seat" => {
                let v = value("--seat")?;
                side = Some(
                    parse_seat(v)
                        .ok_or_else(|| format!("--seat \"{v}\" is not allied or japanese"))?,
                );
            }
            "--seed" => {
                let v = value("--seed")?;
                seed = Some(
                    v.parse::<u64>()
                        .map_err(|_| format!("--seed \"{v}\" is not a whole number"))?,
                );
            }
            "--save" => save = Some(value("--save")?),
            "--transcript" => transcript = Some(value("--transcript")?),
            flag if flag.starts_with('-') => {
                return Err(format!("unknown option \"{flag}\"\n\n{usage}"))
            }
            path if scenario.is_none() => scenario = Some(path),
            extra => return Err(format!("unexpected argument \"{extra}\"\n\n{usage}")),
        }
    }
    Ok(Options {
        scenario: scenario.ok_or_else(|| format!("no scenario given\n\n{usage}"))?,
        side: side
            .ok_or_else(|| format!("--seat allied or --seat japanese is needed\n\n{usage}"))?,
        seed,
        save,
        transcript,
    })
}

pub fn command(args: &[String], usage: &str) -> Result<(), String> {
    use std::io::{BufRead, Write};
    let options = parse(args, usage)?;
    let seats = BTreeMap::from([(options.side, sim_agent::SeatKind::Player)]);
    let (session, kept) = crate::serve::open(options.scenario, options.seed, seats)?;
    let mut server = Server::new(session, options.side);
    let stdin = std::io::stdin();
    let stdout = std::io::stdout();
    let mut out = stdout.lock();
    for line in stdin.lock().lines() {
        let line = line.map_err(|e| format!("standard input: {e}"))?;
        if line.trim().is_empty() {
            continue;
        }
        if let Some(answer) = server.handle_line(&line) {
            writeln!(out, "{answer}")
                .and_then(|()| out.flush())
                .map_err(|e| format!("standard output: {e}"))?;
        }
    }
    kept.finish(&server.session, options.save, options.transcript)
}
