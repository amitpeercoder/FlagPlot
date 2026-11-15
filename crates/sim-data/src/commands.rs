//! A scenario's `[[command]]` tables (decision 0107, item 16): what a side's
//! commander says on a day, as a scenario states the orders it gives, so that
//! a test can begin with an intent in force.
//!
//! Only the commander's kinds are written here: `set_intent`,
//! `withdraw_intent`, `set_limit`, `set_leave`, `approve` and `cancel_plan`.
//! The staff's kinds are the staff's to give and a scenario cannot state
//! them. Every table has `day`, `side` and `kind`, and the keys its kind
//! takes; a key the kind does not take, a key that is not a key of any
//! command, a TOML float and a number that is not whole are all refused.
//!
//! | `kind` | keys |
//! | --- | --- |
//! | `set_intent` | `intent`, `base`, `floor_days`, `interval_days` (optional) |
//! | `withdraw_intent` | `intent` |
//! | `set_limit` | `limit` and its keys, below |
//! | `set_leave` | `leave` (`"prepare"` or `"launch"`), `given` |
//! | `approve`, `cancel_plan` | `plan` |
//!
//! The limits: `limit = "forces"` has `intent`, `ships`, `task_forces` and
//! `source` (a base); `"area"` has `intent` and `hexes` as `[[q, r], ...]`;
//! `"priority"` has `intent` and `rank`; `"protected"` has `intent` and
//! `ships`; `"reserve"` has `base`, `supply_t` and `fuel_t` as whole numbers
//! or decimal strings; `"risk"` has `intent` and `risk` (`"preserve"`,
//! `"balanced"` or `"press"`).

use std::collections::BTreeSet;

use serde::Deserialize;
use sim_core::command::{CommandKind, ForcesLimit, IntentKind, Leave, Limit, PlanId, Risk};
use sim_core::hex::Hex;
use sim_core::ids::{BaseId, IntentId, ShipId, TaskForceId};
use sim_core::state::Side;

use crate::quantity::Quantity;
use crate::scenario::SideSpec;

/// One `[[command]]` table, every key a command can take. Which keys a kind
/// may carry is checked in [`CommandSpec::build`].
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct CommandSpec {
    /// The day the command is given. Day 1 is the first day.
    day: u32,
    side: SideSpec,
    kind: String,
    limit: Option<String>,
    intent: Option<u32>,
    base: Option<u32>,
    floor_days: Option<u32>,
    interval_days: Option<u32>,
    ships: Option<Vec<u32>>,
    task_forces: Option<Vec<u32>>,
    source: Option<u32>,
    hexes: Option<Vec<[i32; 2]>>,
    rank: Option<u32>,
    supply_t: Option<Quantity>,
    fuel_t: Option<Quantity>,
    risk: Option<String>,
    leave: Option<String>,
    given: Option<bool>,
    plan: Option<u32>,
}

impl CommandSpec {
    /// The names of the keys this table gives besides `day`, `side` and
    /// `kind`.
    fn given(&self) -> Vec<&'static str> {
        let mut keys = Vec::new();
        let mut note = |present: bool, name: &'static str| {
            if present {
                keys.push(name);
            }
        };
        note(self.limit.is_some(), "limit");
        note(self.intent.is_some(), "intent");
        note(self.base.is_some(), "base");
        note(self.floor_days.is_some(), "floor_days");
        note(self.interval_days.is_some(), "interval_days");
        note(self.ships.is_some(), "ships");
        note(self.task_forces.is_some(), "task_forces");
        note(self.source.is_some(), "source");
        note(self.hexes.is_some(), "hexes");
        note(self.rank.is_some(), "rank");
        note(self.supply_t.is_some(), "supply_t");
        note(self.fuel_t.is_some(), "fuel_t");
        note(self.risk.is_some(), "risk");
        note(self.leave.is_some(), "leave");
        note(self.given.is_some(), "given");
        note(self.plan.is_some(), "plan");
        keys
    }

    /// The command, its side and its day, or why the table is wrong.
    pub(crate) fn build(&self, days: u32) -> Result<(u32, Side, CommandKind), String> {
        let day = self.day;
        if day == 0 || day > days {
            return Err(format!(
                "a command is set for day {day}, outside days 1 to {days}"
            ));
        }
        let what = format!("the {} command on day {day}", self.kind);
        // The keys the kind takes, and so the keys it refuses.
        let allowed: &[&str] = match (self.kind.as_str(), self.limit.as_deref()) {
            ("set_intent", _) => &["intent", "base", "floor_days", "interval_days"],
            ("withdraw_intent", _) => &["intent"],
            ("set_limit", Some("forces")) => &["limit", "intent", "ships", "task_forces", "source"],
            ("set_limit", Some("area")) => &["limit", "intent", "hexes"],
            ("set_limit", Some("priority")) => &["limit", "intent", "rank"],
            ("set_limit", Some("protected")) => &["limit", "intent", "ships"],
            ("set_limit", Some("reserve")) => &["limit", "base", "supply_t", "fuel_t"],
            ("set_limit", Some("risk")) => &["limit", "intent", "risk"],
            ("set_limit", Some(other)) => {
                return Err(format!(
                    "{what} has limit \"{other}\", which is not forces, area, priority, protected, reserve or risk"
                ))
            }
            ("set_limit", None) => {
                return Err(format!("{what} needs a limit"));
            }
            ("set_leave", _) => &["leave", "given"],
            ("approve" | "cancel_plan", _) => &["plan"],
            (other, _) => {
                return Err(format!(
                    "the command on day {day} is \"{other}\", which is not set_intent, withdraw_intent, set_limit, set_leave, approve or cancel_plan (the staff's commands are the staff's to give)"
                ))
            }
        };
        for key in self.given() {
            if !allowed.contains(&key) {
                return Err(format!("{what} does not take \"{key}\""));
            }
        }
        let need =
            |value: Option<u32>, key: &str| value.ok_or_else(|| format!("{what} needs \"{key}\""));
        let intent = || need(self.intent, "intent").map(IntentId::new);
        let ships = |list: &Option<Vec<u32>>, key: &str| -> Result<BTreeSet<ShipId>, String> {
            let list = list
                .as_ref()
                .ok_or_else(|| format!("{what} needs \"{key}\""))?;
            Ok(list.iter().copied().map(ShipId::new).collect())
        };
        let weight = |value: &Option<Quantity>, key: &str| {
            let value = value
                .as_ref()
                .ok_or_else(|| format!("{what} needs \"{key}\""))?;
            value
                .to_fixed()
                .map_err(|e| format!("{what}, \"{key}\" {e}"))
        };
        let kind = match self.kind.as_str() {
            "set_intent" => CommandKind::SetIntent {
                intent: intent()?,
                kind: IntentKind::KeepSupplied {
                    base: BaseId::new(need(self.base, "base")?),
                    floor_days: need(self.floor_days, "floor_days")?,
                    interval_days: self.interval_days,
                },
            },
            "withdraw_intent" => CommandKind::WithdrawIntent { intent: intent()? },
            "set_limit" => CommandKind::SetLimit(match self.limit.as_deref() {
                Some("forces") => Limit::Forces {
                    intent: intent()?,
                    limit: ForcesLimit {
                        ships: ships(&self.ships, "ships")?,
                        task_forces: self
                            .task_forces
                            .as_ref()
                            .ok_or_else(|| format!("{what} needs \"task_forces\""))?
                            .iter()
                            .copied()
                            .map(TaskForceId::new)
                            .collect(),
                        source: BaseId::new(need(self.source, "source")?),
                    },
                },
                Some("area") => Limit::Area {
                    intent: intent()?,
                    hexes: self
                        .hexes
                        .as_ref()
                        .ok_or_else(|| format!("{what} needs \"hexes\""))?
                        .iter()
                        .map(|[q, r]| Hex::new(*q, *r))
                        .collect(),
                },
                Some("priority") => Limit::Priority {
                    intent: intent()?,
                    rank: self.rank,
                },
                Some("protected") => Limit::Protected {
                    intent: intent()?,
                    ships: ships(&self.ships, "ships")?,
                },
                Some("reserve") => Limit::Reserve {
                    base: BaseId::new(need(self.base, "base")?),
                    supply_t: weight(&self.supply_t, "supply_t")?,
                    fuel_t: weight(&self.fuel_t, "fuel_t")?,
                },
                _ => {
                    let name = self
                        .risk
                        .as_deref()
                        .ok_or_else(|| format!("{what} needs \"risk\""))?;
                    Limit::Risk {
                        intent: intent()?,
                        risk: match name {
                            "preserve" => Risk::Preserve,
                            "balanced" => Risk::Balanced,
                            "press" => Risk::Press,
                            other => {
                                return Err(format!(
                                    "{what} has risk \"{other}\", which is not preserve, balanced or press"
                                ))
                            }
                        },
                    }
                }
            }),
            "set_leave" => CommandKind::SetLeave {
                leave: match self.leave.as_deref() {
                    Some("prepare") => Leave::Prepare,
                    Some("launch") => Leave::Launch,
                    Some(other) => {
                        return Err(format!(
                            "{what} has leave \"{other}\", which is not prepare or launch"
                        ))
                    }
                    None => return Err(format!("{what} needs \"leave\"")),
                },
                given: self
                    .given
                    .ok_or_else(|| format!("{what} needs \"given\""))?,
            },
            "approve" => CommandKind::Approve {
                plan: PlanId::new(need(self.plan, "plan")?),
            },
            _ => CommandKind::CancelPlan {
                plan: PlanId::new(need(self.plan, "plan")?),
            },
        };
        Ok((day, self.side.into(), kind))
    }
}
