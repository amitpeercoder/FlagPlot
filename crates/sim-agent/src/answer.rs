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
/// refusal `LevelNotAboveZero` (decision 0162). 28 tells the side what an
/// air attack on its ship was made with and, where its people recognised
/// them, the aircraft's type: the briefing's `attacks_on_ships` from the air
/// are counted by weapon, with `weapon`, `recognised` and `not_recognised`,
/// a raid on a base has `weapon` and `recognised`, and the words of both
/// events say it (decision 0167). 29 adds, in a game that has a trial, the
/// tools `trial` and `trials` of a seat with a staff that plays a whole side,
/// and their answers `trial`, the estimate of an intent or a previewed draft
/// in a commander's words, and `trials`, the estimates the seat was shown
/// (decision 0178). 30 adds to a claim from the air the types of the aircraft
/// claimed destroyed on the ground, by name, where the raiders made them out
/// (`ground_types`), and who claims what (`claimed_by`: each unit with what
/// it claims, a unit no longer in the game `since_dissolved`), with both in
/// the claim's words, and to an air group's line its tally of its claims
/// since the game began (`claimed_to_date`) (decisions 0179 and 0180). 31
/// adds to `trial` the arguments `compare`, `dates` and `describe`, to its
/// answer the second trial beside the first (`beside`) and in what was asked
/// `compare`, `beside`, `in_force` and `dates`; the answers `window`, the
/// same order at the coming morning and later dates, and `trial_described`,
/// what would be tried with nothing played; and to each estimate `trials`
/// lists its `kind` (decision 0181). 32 adds to the briefing's objectives
/// part what the enemy is scored for, `enemy_objectives`: each of his
/// objectives with its kind, its base, open or shut and its worth, and
/// nothing of his mission, progress or state (decision 0183). 33 adds the
/// kind men_landed (decision 0186) and names a base an objective names that
/// is not on the side's map by its hex. 34 adds the notice kind and the
/// standing interrupt `lift_turned_back`, and the task force order
/// `no_cargo_work` (decision 0191). 35 adds the limit `targets` (`kinds`,
/// `waters_first`) of decision 0190 to `set_limit` and `command_state`, and the
/// air group order `set_strike_first_waters` (`hexes`). 36 adds what is held apart
/// for the side until its day (decision 0195): the briefing's `arrivals.to_enter`,
/// `entered_yesterday` and `entry_waiting_yesterday`, `list_forces`' `to_enter`,
/// and the entity answer `to_enter` for a held unit; and the refusal `is not on
/// the map yet`. 37 adds the side's supply lines (decision 0196). 38 adds the
/// side's streams of replacement men (decision 0197): the briefing's
/// `arrivals.men_replacements` and `men_joined_yesterday`, and the
/// `establishment` of a land unit's line. 39 adds fuelling at sea (decision
/// 0201): the task force order `fuel_from` (`giver`), a force's `fuel_from`
/// in its line where it is under that order, a ship line's `fuels_at_sea`
/// for an oiler, and the briefing's
/// `fuelled_at_sea`, `fuelling_broken_off` and `fuel_orders_ended` among the
/// force changes. 40 lets a trial be asked inside a day, at 0600 and 1200
/// (decision 0204): the refusal "a trial is asked at dusk" is no longer
/// given, and what was asked gains `inside_day` (`null` at dusk, else the
/// day and the clock) in the answers `trial`, `window` and
/// `trial_described` and in each row of `trials`. 41 adds to a claim from
/// the air the types of the aircraft claimed shot down, by name, where its
/// men made them out (`air_types`), with them in the claim's words (decision
/// 0210). 43 adds the pursuit of decision 0214: the commands `set_pursuit`
/// and `answer_pursuit`, the stop `asks_leave` of `run` and `advance` with
/// the `asking` it gives, the leave to pursue, the pursuits and the
/// refusals in `command_state`, the three notices in the report and the
/// briefing, and `acknowledge` of the standing interrupt by task force. 44 adds a fire among a carrier's aircraft (decision
/// 0212): a ship line's and the ship entity's `on_fire_among_aircraft` (true,
/// left out otherwise), the briefing's `losses.aircraft_fires` and a sunk
/// ship's `cause` (`fire_among_aircraft`), and `advance`'s "loss" taking in a
/// fire set. 45 adds a reason's `cautions` (decision 0217: each with its
/// `kind`, `station_in_danger` or `fuel_not_measured`, an optional `of` and
/// `hex`, and its `words`; left out where there is none) and each sub-intent's
/// `cautions` in `propose`'s tree; the briefing's `arrivals.came_today` and
/// `arrivals.comes_tomorrow` (decision 0215); and, inside a day, an order not
/// of the seven kinds kept for the next dusk rather than refused (decision
/// 0219): its result's `kept` and `acts`, and the answer's `kept_in_call` and
/// `kept`.
pub const VERSION: u32 = 45;

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
