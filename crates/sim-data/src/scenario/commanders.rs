use serde::Deserialize;
use sim_core::command::{SubmarineDoctrine, SubmarinePattern};
use sim_core::ids::{BaseId, TaskForceId};
use sim_core::pictures::CommandRef;
use sim_core::state::Side;

use super::orders::SideSpec;

/// The `[side.<side>]` tables (decision 0223, stage 0).
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct SidesSpec {
    pub(super) allied: Option<SideDoctrineSpec>,
    pub(super) japanese: Option<SideDoctrineSpec>,
}

/// How a side's staff uses its boats (decision 0223, stage 0):
/// `submarine_pattern` (`off_bases` or `lines`) and
/// `expects_boats_off_own_bases` (true or false), each optional. A side
/// given neither has no pattern and expects nothing.
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct SideDoctrineSpec {
    pub(super) submarine_pattern: Option<String>,
    pub(super) expects_boats_off_own_bases: Option<bool>,
    /// Whether the side's staff chooses its sources of supply (decision
    /// 0228, A). Left out, it does not.
    pub(super) staff_chooses_supply_sources: Option<bool>,
}

impl SideDoctrineSpec {
    /// The doctrine, or why the word for the pattern is refused, naming the
    /// side and the word.
    pub(super) fn doctrine(&self, side: Side) -> Result<SubmarineDoctrine, String> {
        let pattern = match &self.submarine_pattern {
            None => None,
            Some(word) => Some(SubmarinePattern::from_name(word).ok_or_else(|| {
                format!(
                    "the {} side's submarine_pattern \"{word}\" is not a pattern: it is \"off_bases\" or \"lines\" (decision 0223)",
                    side.name()
                )
            })?),
        };
        Ok(SubmarineDoctrine {
            pattern,
            expects_boats_off_own_bases: self.expects_boats_off_own_bases.unwrap_or(false),
        })
    }
}

/// A commander of one command (decision 0137, 1 and 2): the side, exactly
/// one of `task_force`, `base` or `area = true`, his name, his initiative
/// rating (`cautious`, `steady` or `bold`) and the note of what the rating
/// rests on, which is required. Every rating is a placeholder.
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct CommanderSpec {
    pub(super) side: SideSpec,
    pub(super) task_force: Option<u32>,
    pub(super) base: Option<u32>,
    pub(super) area: Option<bool>,
    pub(super) name: String,
    pub(super) initiative: String,
    pub(super) note: Option<String>,
}

/// The command a task force or a base is placed under (decision 0134, 4):
/// exactly one of `task_force` or `base`, as in
/// `superior = { task_force = 1 }`.
#[derive(Deserialize, Clone, Copy)]
#[serde(deny_unknown_fields)]
pub(super) struct SuperiorSpec {
    pub(super) task_force: Option<u32>,
    pub(super) base: Option<u32>,
}

impl SuperiorSpec {
    pub(super) fn command(&self) -> Result<CommandRef, String> {
        match (self.task_force, self.base) {
            (Some(force), None) => Ok(CommandRef::TaskForce(TaskForceId::new(force))),
            (None, Some(base)) => Ok(CommandRef::Base(BaseId::new(base))),
            _ => Err("a superior names exactly one of task_force or base".to_string()),
        }
    }
}
