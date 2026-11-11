//! What every answer has in common.

use serde::Serialize;

/// The format name every answer carries.
pub const FORMAT: &str = "flagplot-agent";

/// The version of the format. A field removed or changed in meaning bumps it;
/// a field added does not (see `schemas/README.md`).
pub const VERSION: u32 = 1;

/// The answer to a question about something the side does not know of: a
/// unit of the enemy, a number that does not exist, an event the side may not
/// see. It is the same document in every such case and carries nothing of the
/// question, so the cases cannot be told apart (decision 0048, 9 and 10).
#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize)]
pub struct NotKnown {
    pub format: &'static str,
    pub version: u32,
    pub kind: &'static str,
    /// The tool asked: `"get_entity"` or `"explain"`.
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
