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
/// (decision 0138, 3 and 4).
pub const VERSION: u32 = 16;

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
