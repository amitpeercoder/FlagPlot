//! The tools of a seat whose side has a staff, as the MCP server and `play`
//! list them (decision 0107, "As built", "The tools"): each with the
//! description a language model commands by and its input schema. The
//! answers are the agent interface's (`sim_agent`), reached through
//! `Session::call`.

use serde_json::{json, Value};

/// What the instructions say of commanding by intent, for a seat whose side
/// has a staff.
pub const STAFF_INSTRUCTIONS: &str = "\n\nCommanding by intent. Beside orders to units you can state an intent and let your staff carry it out. There is one kind of intent: keep a base supplied. set_intent {base, floor_days, interval_days} asks the staff to keep the stock at your base N at or above floor_days days of supply, each shipment bringing interval_days days of the base's use above the floor (the staff uses 15 where you give none). The staff does nothing for your side until you set an intent AND give it a forces limit: set_limit {limit: \"forces\", intent, ships: [your ship numbers], task_forces: [], source: base number}, the ships it may use and the one base it loads them at; set_limit {limit: \"reserve\", base, supply_t, fuel_t} keeps a stock at a base that the staff's loads never take. The staff drafts a plan for the intent. propose {intent} shows what it would do (the shipments, their loads and days, what binds); approve {plan} prepares a draft and releases a plan that is preparing; with leave to prepare (given by default) the staff prepares drafts itself, and with leave to launch (withheld by default; set_leave {leave: \"launch\", given: true}) it releases them itself. staff_orders shows what the staff will hand in at the start of the coming day, each with its reason; why {number} gives the reason of one decision it made (command_state lists yesterday's decisions with their numbers); command_state and plans show what stands. Commands are queued for the coming day like orders, checked at once (queued or refused with the reason) and take effect at the start of the coming day, before the staff is asked; then call end_day as always.";

const STAFF_RULE: &str = "The staff does nothing for your side until you set an intent (set_intent) and give it a forces limit (set_limit with limit forces): with no intent in force it hands in nothing.";

const COMMAND_STATE_DESCRIPTION: &str = "What stands in your command of the staff, as it will stand when the coming day begins (the commands you have queued taken in): your intents (number, the base, floor_days, interval_days or \"not set: the staff uses 15\", the limits of each: forces, area, priority, protected ships, risk), and for each intent in force the staff's projection of the base's days of supply (today's figure and the first day the projection falls below the floor); the two leaves (prepare, given by default, lets the staff approve a draft; launch, withheld by default, lets it release a plan that is preparing); the reserves your bases keep (and their stocks); your plans (state draft, preparing, released, complete, failed or cancelled; the day it was approved; amendments; the runs with their four days load, sail, arrive and free, their ships and load in tons); the commitments of your ships; the commands you have queued for the coming day, in order; the staff's decisions of yesterday, each with the number why takes; and notes saying what waits for you. STAFF_RULE Takes no arguments. Quantities are decimal strings.";

const PLANS_DESCRIPTION: &str = "Your plans and what each run is doing, as the coming day will find them (the commands you have queued taken in). A plan is the staff's way of meeting one intent: it is a draft until approved (approve, or the staff's own approval with leave to prepare), preparing once approved and released once launched (approve again, or the staff's with leave to launch), and complete, failed or cancelled when it ends. A run is one convoy of the plan: its ships, its load in tons of supply from the source base, and its four days: load (loading begins at the source), sail, arrive (due at the destination) and free (its ships are back and refuelled). Each run shows the phase its days put it in (waiting, loading, under_way, returning, finished), a sentence saying what it is doing and, for each of its ships, where it is and the supply aboard. STAFF_RULE Takes no arguments.";

const SET_INTENT_DESCRIPTION: &str = "Queue an intent for the coming day: keep the stock at one of your bases at or above a floor of days of supply. Arguments: base (a base number, one of your own: the base to keep supplied), floor_days (a whole number of days, at least 1: the staff keeps the base at or above this many days of supply), interval_days (optional, a whole number of days: each shipment brings this many days of the base's use above the floor; with none the staff uses 15), intent (optional: the intent's number; with none the side's next free number is used, one more than the highest ever set, and the answer says which; a number once withdrawn is never used again; setting a number that is in force changes that intent, and an intent whose plan is approved is amended). Example: {\"base\": 2, \"floor_days\": 30, \"interval_days\": 15}. The command is checked at once by taking it in after the commands you have already queued: the answer says queued for day N, or refused with the engine's reason (a base that is not yours, an intent number not allowed, floor_days of 0; the same words for an enemy base and for one that does not exist). A queued command takes effect at the start of day N. Setting an intent drafts a plan for it. STAFF_RULE Give it a forces limit next (set_limit). \"clear\": true empties your queued commands first; \"reason\" is for the record.";

const WITHDRAW_INTENT_DESCRIPTION: &str = "Queue the withdrawal of an intent that is in force, for the coming day. Argument: intent (its number). Its plan ends (complete if it was released, else cancelled) and the number is never used again. Example: {\"intent\": 1}. Checked at once: queued for day N, or refused with the reason (no such intent in force). STAFF_RULE Withdrawing the intent stops the staff acting for it. \"clear\": true empties your queued commands first.";

const SET_LIMIT_DESCRIPTION: &str = "Queue one limit for the coming day. \"limit\" names it and the other arguments are that limit's: forces {intent, ships: [your ship numbers], task_forces: [your task force numbers], source: base number}: the pool the staff may use for the intent and the ONE base it loads from (ships and task_forces default to none; a task force in the pool brings all its ships); area {intent, hexes: [[q, r], ...]}: the only hexes the intent's ships may enter (with none set the sea is open); priority {intent, rank: whole number, 1 first, or null}: its rank; protected {intent, ships: [ship numbers]}: ships the staff may not take from the intent; reserve {base, supply_t, fuel_t}: the tons of supply and of fuel the base keeps, which the staff's loads and refuelling never take (decimal strings, or whole numbers); risk {intent, risk: preserve, balanced or press}. Examples: {\"limit\": \"forces\", \"intent\": 1, \"ships\": [1, 2], \"task_forces\": [], \"source\": 1} and {\"limit\": \"reserve\", \"base\": 1, \"supply_t\": \"1000\", \"fuel_t\": \"0\"}. Ships and task forces are in YOUR numbers (those the briefing shows). Checked at once after the commands already queued: queued for day N, or refused with the engine's reason (a ship or base that is not yours, an intent not in force, a weight below zero). A change to a limit of an intent whose plan is approved is an amendment of the plan. STAFF_RULE The forces limit is the one that matters first. \"clear\": true empties your queued commands first.";

const SET_LEAVE_DESCRIPTION: &str = "Queue the giving or withholding of one of your two leaves for the coming day. Arguments: leave (\"prepare\" or \"launch\") and given (true or false). prepare: the staff may make a draft plan inside your limits ready (preparing) without you; given by default. launch: the staff may release a plan that is preparing without you; withheld by default, so until you give it you release a plan yourself with approve. Example: {\"leave\": \"launch\", \"given\": true}. Checked at once: queued for day N. STAFF_RULE \"clear\": true empties your queued commands first.";

const APPROVE_DESCRIPTION: &str = "Queue the approval of a plan for the coming day: a draft becomes preparing (the staff has leave to prepare, so it does this itself unless you withheld the leave), and a plan that is preparing is released (the staff does this itself only with leave to launch). Argument: plan (its number, as command_state and plans show it). Example: {\"plan\": 1}. Checked at once after the commands already queued, so a draft you approve in this call is preparing for the next: queued for day N, or refused with the reason (no such plan, a plan already released or ended). STAFF_RULE \"clear\": true empties your queued commands first.";

const CANCEL_PLAN_DESCRIPTION: &str = "Queue the cancelling of a plan that has not ended, for the coming day. Argument: plan (its number). Example: {\"plan\": 1}. Checked at once: queued for day N, or refused with the reason (no such plan, a plan already ended). A convoy already under way is not recalled in this version: its ships come home by the route they hold. STAFF_RULE \"clear\": true empties your queued commands first.";

const PROPOSE_DESCRIPTION: &str = "What the staff would do for one intent, worked from your state as the coming day will find it (the commands you have queued taken in) and stored nowhere: it changes nothing. Argument: intent (its number). The answer gives the plan and its state; the projection of the base's days of supply over the next 30 days (the stock, the planned use and the figure of each day, today's reading and the first day it falls below the floor); the runs the staff would make over the next 30 days, each with its ships, load in tons, its four days (load, sail, arrive, free) and its reason in five parts; what binds if it cannot meet the intent (no ship free until a day, the source's stock, no route inside the area, a ship without the fuel, the use more than the pool can carry) with the shortfall in days and the day the source falls to its reserve; what the proposal displaces (nothing: one intent); and what the staff would hand in for the coming day if the plan were released (orders, commands and decisions to give no order, in your numbers). Example: {\"intent\": 1}. STAFF_RULE With no forces limit the proposal says so.";

const STAFF_ORDERS_DESCRIPTION: &str = "What the staff would hand in at the start of the coming day, asked on your state as that day will find it (the commands you have queued taken in): its orders (a unit of yours and the order, written as issue_orders writes them, ships and task forces in your numbers; a task force the staff will form is the next number you would give it), its commands (approve, add_run, commit_ships, free_ships, time_run, mark_plan) and its decisions to give no order, each with its reason in five parts: what it serves, the evidence, what binds it, the calculation (\"worked\") and what would change it. It changes nothing: the staff is asked again when the day begins, on the same state, and gives the same answer. A day on which the staff gives an intent no order it still gives one reason, saying what the runs are doing and what the next run waits for. STAFF_RULE Takes no arguments.";

const WHY_DESCRIPTION: &str = "The reason of one decision your staff made, by its number: the number command_state gives for yesterday's decisions (they are numbered among your events, as explain numbers them). The answer gives the day, what the decision is for (an order of the staff's: its unit, its body and whether it was accepted or refused and why; a command of the staff's: the command and whether it was taken or refused; or a decision to give no order) and its reason in five parts: what it serves, the evidence with its day, what bound it, the calculation (\"worked\", the arithmetic in tons, days and hours) and what would change it. Argument: number (a whole number). Example: {\"number\": 17}. A number that is not a reason of your staff answers \"not known\", as explain does for an event you may not see. It changes nothing: it only reads. STAFF_RULE";

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
                "required": ["base", "floor_days"],
                "additionalProperties": false,
                "properties": {
                    "intent": {"type": "integer", "minimum": 1, "description": "The intent's number; with none, the side's next free number."},
                    "base": {"type": "integer", "minimum": 1, "description": "The number of one of your bases: the base to keep supplied."},
                    "floor_days": {"type": "integer", "minimum": 1, "description": "Days of supply to keep the base at or above."},
                    "interval_days": {"type": "integer", "minimum": 1, "description": "Days of use each shipment brings above the floor; with none the staff uses 15."},
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
                    "limit": {"enum": ["forces", "area", "priority", "protected", "reserve", "risk"], "description": "Which limit. forces takes intent, ships, task_forces and source; area takes intent and hexes; priority takes intent and rank; protected takes intent and ships; reserve takes base, supply_t and fuel_t; risk takes intent and risk."},
                    "intent": intent.clone(),
                    "ships": numbers("(forces, protected) your ship numbers."),
                    "task_forces": numbers("(forces) your task force numbers."),
                    "source": {"type": "integer", "minimum": 1, "description": "(forces) the number of the one base the staff loads from."},
                    "hexes": {"type": "array", "items": {"type": "array", "items": {"type": "integer"}, "minItems": 2, "maxItems": 2}, "description": "(area) hexes as [q, r] pairs."},
                    "rank": {"type": ["integer", "null"], "minimum": 1, "description": "(priority) 1 first; null for unranked."},
                    "base": {"type": "integer", "minimum": 1, "description": "(reserve) the base's number."},
                    "supply_t": {"description": "(reserve) tons of supply kept: a decimal string such as \"1000\", or a whole number.", "oneOf": [{"type": "string"}, {"type": "integer"}]},
                    "fuel_t": {"description": "(reserve) tons of fuel kept: a decimal string or a whole number.", "oneOf": [{"type": "string"}, {"type": "integer"}]},
                    "risk": {"enum": ["preserve", "balanced", "press"], "description": "(risk) the risk level."},
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
            "approve",
            described(APPROVE_DESCRIPTION),
            json!({
                "type": "object",
                "required": ["plan"],
                "additionalProperties": false,
                "properties": {"plan": plan.clone(), "clear": clear.clone(), "reason": reason.clone()}
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
            "staff_orders",
            described(STAFF_ORDERS_DESCRIPTION),
            no_arguments,
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
