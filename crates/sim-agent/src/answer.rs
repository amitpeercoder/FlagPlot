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
/// and report; `base` of an intent is `null` for a neutralise.
pub const VERSION: u32 = 9;

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
