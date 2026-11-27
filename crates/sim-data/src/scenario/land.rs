use crate::quantity::Quantity;
use serde::Deserialize;
use sim_core::ids::FormationId;
use sim_core::state::Formation;
use std::collections::BTreeMap;

use super::orders::{PostureSpec, SideSpec};

/// A formation of land units (decision 0140, 7): its number, name, side and
/// the formation it belongs to, if any, which must be there and of the same
/// side. No formation may be its own ancestor.
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct FormationSpec {
    pub(super) id: u32,
    pub(super) name: String,
    pub(super) side: SideSpec,
    pub(super) parent: Option<u32>,
}

/// A land unit at the start (decision 0066, 3): exactly one of a hex, where
/// its men are ashore, and a task force, where they are aboard.
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct LandUnitSpec {
    pub(super) id: u32,
    pub(super) name: String,
    pub(super) side: SideSpec,
    pub(super) men: u32,
    pub(super) q: Option<i32>,
    pub(super) r: Option<i32>,
    pub(super) task_force: Option<u32>,
    /// The posture at the start, `defend` or `attack` (decision 0090, 1).
    /// Defaults to defend.
    pub(super) posture: Option<PostureSpec>,
    /// Fatigue at the start, 0 to 100 (decision 0090, 11). Defaults to 0.
    pub(super) fatigue: Option<Quantity>,
    /// Experience at the start, a whole 0 to 4 (decision 0097). Defaults to 2.
    pub(super) experience: Option<u32>,
    /// Days of training counted at the start, a whole number below the
    /// ground figure `training_days_per_level` (decision 0097). Defaults to
    /// 0.
    pub(super) training_days: Option<u32>,
    /// Days of fighting counted at the start, a whole number below the
    /// ground figure `fight_days_per_level` (decision 0097). Defaults to 0.
    pub(super) fight_days: Option<u32>,
    /// What the unit is good at (decision 0098): each skill a whole number
    /// from 0 to 4; any not given is 2.
    pub(super) skills: Option<SkillsSpec>,
    /// Marks a working unit (decision 1 of 0099): one whose men do not
    /// fight but work. Defaults to false. Not with `posture = "attack"`.
    pub(super) working: Option<bool>,
    /// The fieldworks the unit has dug at the start (decision 0106): from 0
    /// to the ground figure `fieldworks_limit`, and 0 for a working unit,
    /// one on attack or one with no men ashore. Defaults to 0.
    pub(super) fieldworks: Option<Quantity>,
    /// The day from which the side may use it (decision 0140, 1): a whole
    /// number from 1. Without it, from day 1.
    pub(super) available_from: Option<u32>,
    /// The day the unit enters the game (decision 0195, 1): a whole number
    /// from 2 up to the scenario's `days`. Until then it is held apart, in
    /// no hex and in no count. Without it, the unit is there on day 1.
    /// Placed at a base (a land unit ashore in a hex with a base); not one
    /// aboard a force, which enters with its force.
    pub(super) enters: Option<u32>,
    /// The formation the unit belongs to (decision 0140, 7): a formation of
    /// its side.
    pub(super) formation: Option<u32>,
    /// The men the unit should have (decision 0197, 1 and 19): not below
    /// its `men`. Without it, its `men`.
    pub(super) establishment: Option<u32>,
    /// The unit's nights of hunger at the start (decision 0205, 7): a whole
    /// number. Without it, 0. Only the hunger figures read it.
    pub(super) hunger_nights: Option<u32>,
}

/// A land unit's skills (decision 0098), each optional, defaulting to 2:
/// `assault`, `holding`, `weight`, and one movement skill for each kind of
/// terrain, `clear`, `jungle`, `mountain` and `atoll`.
#[derive(Deserialize, Default)]
#[serde(deny_unknown_fields)]
pub(super) struct SkillsSpec {
    pub(super) assault: Option<u32>,
    pub(super) holding: Option<u32>,
    pub(super) weight: Option<u32>,
    pub(super) clear: Option<u32>,
    pub(super) jungle: Option<u32>,
    pub(super) mountain: Option<u32>,
    pub(super) atoll: Option<u32>,
    pub(super) works: Option<u32>,
}

/// The formations of a scenario (decision 0140, 7), checked: each number
/// once, each parent there and of the same side, and none its own ancestor.
pub(super) fn formations(
    specs: &[FormationSpec],
) -> Result<BTreeMap<FormationId, Formation>, String> {
    let mut found: BTreeMap<FormationId, Formation> = BTreeMap::new();
    for spec in specs {
        let id = FormationId::new(spec.id);
        if spec.id == 0 {
            return Err(format!("{id} is numbered 0, which is no formation"));
        }
        let formation = Formation {
            id,
            name: spec.name.clone(),
            side: spec.side.into(),
            parent: spec.parent.map(FormationId::new),
        };
        if found.insert(id, formation).is_some() {
            return Err(format!("{id} appears twice"));
        }
    }
    for formation in found.values() {
        let Some(parent) = formation.parent else {
            continue;
        };
        let Some(above) = found.get(&parent) else {
            return Err(format!(
                "{} has the parent {parent}, which is not there",
                formation.id
            ));
        };
        if above.side != formation.side {
            return Err(format!(
                "{} is {} but its parent {parent} is {}",
                formation.id,
                formation.side.name(),
                above.side.name()
            ));
        }
        // Up the tree: a loop comes back to where it began within as many
        // steps as there are formations.
        let mut at = Some(parent);
        for _ in 0..found.len() {
            match at {
                Some(next) if next == formation.id => {
                    return Err(format!("{} is its own ancestor", formation.id));
                }
                Some(next) => at = found.get(&next).and_then(|f| f.parent),
                None => break,
            }
        }
    }
    Ok(found)
}
