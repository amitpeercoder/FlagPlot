//! What every answer has in common.

use serde::Serialize;

/// The format name every answer carries.
pub const FORMAT: &str = "flagplot-agent";

/// The version of the format. A field removed or changed in meaning bumps it;
/// a field added does not (see `schemas/README.md`). 6 adds hold, release and
/// preview (decision 0107, step 3). 7 adds set_interrupt, acknowledge, run and
/// report and the `interrupts` of command_state (decision 0107, step 4). 8 adds
/// the tree of intents (decisions 0116 and 0117): the kinds `cover` and `hold`,
/// `parent`, `maker`, `addressee`, `children` and the limits read up the tree
/// in command_state, air groups in a forces limit and a hold on a whole air
/// group, a reason's `chain` up to the commander's intent, and the measure of a
/// hold intent in command_state and report. 9 adds the kinds `neutralise` and
/// `raid` with a `target` (decision 0120), a reason's `level` (the level that
/// decided) in why, and the measure of a neutralise intent in command_state
/// and report; `base` of an intent is `null` for a neutralise. 10 adds the
/// kinds `deny` (`hex`, `distance`), `strike_ships` and `search_waters`
/// (decision 0123), `base` of an intent being `null` for a deny too, the
/// measure of a deny intent in command_state and report (`denials`), the air
/// group order `set_strike_sizing` and a group's `strike_sizing`, and the
/// strike answer's `aircraft` with, for a group that sizes its strikes, `need`
/// and `sent_before` (decision 0118). 11 adds the kinds `stand_at` and
/// `operate_carriers` (`task_force` and `hex`, `base` being `null`), the level
/// `task_force` of an intent and of a reason, `stand_at` in a route order and
/// in a force's line, the halt for the night (`night` among what a halt waits
/// for) and the `pulse` a halt or steam-on question was asked in
/// (decisions 0124 and 0125). 12 adds the task force order `escort`, with a
/// `charge` written `task_force:N` or none to clear it, and a force's
/// `charge` in its detail where it has one (decision 0121). 13 adds a force
/// line's `ammunition_lacked` (rounds, torpedoes, tons and hours) for a force
/// in port at a base of its side that lacks ammunition (decision 0122), and
/// the briefing's `charges_lost` and `rearmed` among the force changes
/// (decisions 0121 and 0122), each left out when empty. 14 adds the air group
/// order `set_search_once` (`on`) and a group's `search_once`, given only when
/// it is on (decision 0131). 15 adds the area seat's reading of its own
/// forces and bases as the area command last had them reported (decision
/// 0136): the briefing, `list_forces`, `list_bases` and `get_entity` give a
/// force or a base with an `as_of` (such as "the morning of day 1") where
/// that is not the pulse just ended, a force's `radio_silence` where it is
/// on, and the task force order `set_radio_silence` (`on`). 16 adds the
/// seat at a command (decision 0138, 5): a seat at a task force or a base,
/// given its command's picture and its own units only, with the nine tools
/// of play; the easier setting at the area seat; `objectives`'
/// `not_given_to_this_seat`; with command levels on, `command_state`'s
/// `picture` and `forces_held`; and the command report's `refusals`
/// (decision 0138, 3 and 4). 17 adds a unit's `available_from` (the day its
/// side may use it, given only while that day has not come) to the lines of
/// `list_forces` and the briefing, to `get_entity` and to `command_state`'s
/// forces limits, a land unit's `formation` and its `parents` in
/// `get_entity`, and the refusal `not yet available` (decision 0140). 18 adds
/// the two kinds `ready_landing` (`task_force`, `hex`, `by_day`) and
/// `build_up` (`base`, `wants`, `defences`) of decision 0141, with `by_day`,
/// `wants` and `defences` in an intent's kind (`null` for the other kinds),
/// `land_units` in a forces limit and `land_units_binding` in what the staff
/// reads up the tree, a land unit (`land_unit`) as the target of a hold, and
/// the notices `ready`, `late` and `met`. 19 adds the kind `take_place`
/// (`hex` or `base` in the tool, `by_day`) of decision 0142, `phase` in an
/// intent of `command_state` (`prepare`, `move`, `land` or `null`), `by_day`
/// for a take_place, and the staff's `set_limit` of forces on an intent it
/// formed. 20 adds the kind `keep_supplied` to a staff's `form_intent`
/// (`floor_days`, `interval_days`) under a hold, a hold that names an enemy
/// base on the side's map, and `intent` in the staff notice `met` of a
/// take_place (decisions 0143 and 0144). 21 adds the intent kind `bombard`
/// (`task_force` and `target`) of decision 0146. 22 adds the air mission
/// `reconnaissance` (with a `hex`) of decision 0147, the grade of an
/// estimate of the enemy's men ashore (`came_from`), and the refusal
/// `AircraftCannotSearch`. 23 lets a take_place's forces limit leave out
/// any of the task forces, the land units and the source, which the staff
/// then chooses, and gives in `command_state` what the staff reads up the
/// tree as it binds: the commander's where he named it, the staff's choice
/// where he did not (decision 0149). 24 adds the intent kind
/// `control_island` (`island`, `by_day`) of decision 0151, `island` in an
/// intent of `command_state` (`null` for the other kinds), its measure, and
/// `islands` in the command report. 25 adds the tool `events` (every seat)
/// and, where a scenario lets a seat stop between pulses, the tool `advance`,
/// orders given inside a day (`for_pulse` and each result's `for` in
/// issue_orders), the refusal of get_briefing at 0600 and 1200, and the note
/// `inside_day` on the answers worked at 0600 or 1200 on the state as it
/// stands (command_state, plans, staff_orders, propose, preview, a queued
/// command, and a passage or yard measure) (decision 0156). 26 adds the tool
/// `order` (a staffed seat) and its answer `plan_order`: the plan of one of
/// the commander's intents as the staff hands it in, with its phases, what it
/// waits for, its men and the day it expects to be ashore (decision 0159,
/// rule E). 27 adds to the load order with a `weight_t` the level `each_t`,
/// a decimal string, the most any one ship is to hold of the cargo, and the
/// refusal `LevelNotAboveZero` (decision 0162).
pub const VERSION: u32 = 27;

/// The answer to a question about something the side does not know of: a
/// unit of the enemy, a number that does not exist, an event the side may not
/// see. It is the same document in every such case and carries nothing of the
/// question, so the cases cannot be told apart (decision 0048, 9 and 10).
#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize)]
pub struct NotKnown {
    pub format: &'static str,
    pub version: u32,
    pub kind: &'static str,
    /// The tool asked: `"get_entity"`, `"explain"`, `"measure"` or `"why"`.
    pub query: &'static str,
}

impl NotKnown {
    pub(crate) fn of(query: &'static str) -> NotKnown {
        NotKnown {
            format: FORMAT,
            version: VERSION,
            kind: "not_known",
            query,
        }
    }
}
