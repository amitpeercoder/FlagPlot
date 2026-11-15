//! A scenario's `[[command]]` tables (decision 0107, item 16): what a side's
//! commander says on a day, as a scenario states the orders it gives, so that
//! a test can begin with an intent in force.
//!
//! Only the commander's kinds are written here: `set_intent`,
//! `withdraw_intent`, `set_limit`, `set_leave`, `approve`, `cancel_plan`,
//! `hold`, `release`, `set_interrupt` and `acknowledge`. The staff's kinds (`commit_ships`, `end_run` and the
//! rest) are the staff's to give and a scenario cannot state
//! them. Every table has `day`, `side` and `kind`, and the keys its kind
//! takes; a key the kind does not take, a key that is not a key of any
//! command, a TOML float and a number that is not whole are all refused.
//!
//! | `kind` | keys |
//! | --- | --- |
//! | `set_intent` | `intent`, `base`, and `intent_kind`: `"keep_supplied"` (the default when absent, so a scenario written before the kinds reads as it did), which also takes `floor_days` and `interval_days` (optional), `"cover"` (cover `base` from the air) or `"hold"` (hold `base`), which take neither, `"neutralise"` (neutralise the enemy base `target`; takes `target` and no `base`) or `"raid"` (raid `target` from `base`; takes both) |
//! | `withdraw_intent` | `intent` |
//! | `set_limit` | `limit` and its keys, below |
//! | `set_leave` | `leave` (`"prepare"` or `"launch"`), `given` |
//! | `approve`, `cancel_plan` | `plan` |
//! | `hold`, `release` | exactly one of `ship`, `task_force` and `air_group`; with `air_group`, `setting` (`"search_arc"`) holds that setting alone, and with no `setting` the whole group is held; no other target takes `setting` |
//!
//! | `set_interrupt` | either `interrupt`, `base`, `below_days` (a threshold set), or `standing` (`"cannot_meet"`, `"waits_for_leave"` or `"ship_lost"`) and `on` (switch a standing kind off or on); never a mixture of the two |
//! | `acknowledge` | either `interrupt`, or `standing` with exactly one of `plan` and `ship`; never a mixture |
//!
//! The limits: `limit = "forces"` has `intent` and, each optional (a list
//! not given is empty), `ships`, `task_forces` and `air_groups`, and `source`
//! (a base; a supply intent's forces limit must name one, another kind's
//! need not); `"area"` has `intent` and `hexes` as `[[q, r], ...]`;
//! `"priority"` has `intent` and `rank`; `"protected"` has `intent` and
//! `ships`; `"reserve"` has `base`, `supply_t` and `fuel_t` as whole numbers
//! or decimal strings; `"risk"` has `intent` and `risk` (`"preserve"`,
//! `"balanced"` or `"press"`).

use std::collections::BTreeSet;

use serde::Deserialize;
use sim_core::command::{
    CommandKind, ForcesLimit, HeldSetting, HoldTarget, IntentKind, InterruptCondition,
    InterruptRef, InterruptSetting, Leave, Limit, PlanId, Risk, StandingKind, StandingSubject,
};
use sim_core::hex::Hex;
use sim_core::ids::{AirGroupId, BaseId, IntentId, InterruptId, ShipId, TaskForceId};
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
    intent_kind: Option<String>,
    base: Option<u32>,
    target: Option<u32>,
    floor_days: Option<u32>,
    interval_days: Option<u32>,
    ships: Option<Vec<u32>>,
    task_forces: Option<Vec<u32>>,
    air_groups: Option<Vec<u32>>,
    source: Option<u32>,
    hexes: Option<Vec<[i32; 2]>>,
    rank: Option<u32>,
    supply_t: Option<Quantity>,
    fuel_t: Option<Quantity>,
    risk: Option<String>,
    leave: Option<String>,
    given: Option<bool>,
    plan: Option<u32>,
    ship: Option<u32>,
    task_force: Option<u32>,
    air_group: Option<u32>,
    setting: Option<String>,
    interrupt: Option<u32>,
    below_days: Option<u32>,
    standing: Option<String>,
    on: Option<bool>,
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
        note(self.intent_kind.is_some(), "intent_kind");
        note(self.base.is_some(), "base");
        note(self.target.is_some(), "target");
        note(self.floor_days.is_some(), "floor_days");
        note(self.interval_days.is_some(), "interval_days");
        note(self.ships.is_some(), "ships");
        note(self.task_forces.is_some(), "task_forces");
        note(self.air_groups.is_some(), "air_groups");
        note(self.source.is_some(), "source");
        note(self.hexes.is_some(), "hexes");
        note(self.rank.is_some(), "rank");
        note(self.supply_t.is_some(), "supply_t");
        note(self.fuel_t.is_some(), "fuel_t");
        note(self.risk.is_some(), "risk");
        note(self.leave.is_some(), "leave");
        note(self.given.is_some(), "given");
        note(self.plan.is_some(), "plan");
        note(self.ship.is_some(), "ship");
        note(self.task_force.is_some(), "task_force");
        note(self.air_group.is_some(), "air_group");
        note(self.setting.is_some(), "setting");
        note(self.interrupt.is_some(), "interrupt");
        note(self.below_days.is_some(), "below_days");
        note(self.standing.is_some(), "standing");
        note(self.on.is_some(), "on");
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
            ("set_intent", _) => match self.intent_kind.as_deref() {
                None | Some("keep_supplied") => {
                    &["intent", "intent_kind", "base", "floor_days", "interval_days"]
                }
                Some("cover" | "hold") => &["intent", "intent_kind", "base"],
                Some("neutralise") => &["intent", "intent_kind", "target"],
                Some("raid") => &["intent", "intent_kind", "base", "target"],
                Some(other) => {
                    return Err(format!(
                        "{what} has intent_kind \"{other}\", which is not keep_supplied, cover, hold, neutralise or raid"
                    ))
                }
            },
            ("withdraw_intent", _) => &["intent"],
            ("set_limit", Some("forces")) => &[
                "limit",
                "intent",
                "ships",
                "task_forces",
                "air_groups",
                "source",
            ],
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
            ("hold" | "release", _) => &["ship", "task_force", "air_group", "setting"],
            ("set_interrupt" | "acknowledge", _) => {
                // The two forms: a number, or a standing kind. A table that
                // carries keys of both is refused as a mixture, not as a key
                // the kind does not take.
                let (set_form, standing_form) = if self.kind == "set_interrupt" {
                    (
                        self.interrupt.is_some()
                            || self.base.is_some()
                            || self.below_days.is_some(),
                        self.standing.is_some() || self.on.is_some(),
                    )
                } else {
                    (
                        self.interrupt.is_some(),
                        self.standing.is_some() || self.plan.is_some() || self.ship.is_some(),
                    )
                };
                if set_form && standing_form {
                    return Err(format!(
                        "{what} mixes the two forms: it names an interrupt by number or a standing kind, not both"
                    ));
                }
                match (self.kind.as_str(), self.standing.is_some()) {
                    ("set_interrupt", false) => &["interrupt", "base", "below_days"],
                    ("set_interrupt", true) => &["standing", "on"],
                    ("acknowledge", false) => &["interrupt"],
                    (_, _) => &["standing", "plan", "ship"],
                }
            }
            (other, _) => {
                return Err(format!(
                    "the command on day {day} is \"{other}\", which is not set_intent, withdraw_intent, set_limit, set_leave, approve, cancel_plan, hold, release, set_interrupt or acknowledge (the staff's commands are the staff's to give)"
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
        let standing_kind = |name: &str| {
            StandingKind::from_name(name).ok_or_else(|| {
                format!(
                    "{what} has standing kind \"{name}\", which is not cannot_meet, waits_for_leave or ship_lost"
                )
            })
        };
        let kind = match self.kind.as_str() {
            "set_intent" => CommandKind::SetIntent {
                intent: intent()?,
                kind: match self.intent_kind.as_deref() {
                    Some("neutralise") => IntentKind::Neutralise {
                        target: BaseId::new(need(self.target, "target")?),
                    },
                    Some("raid") => IntentKind::Raid {
                        base: BaseId::new(need(self.base, "base")?),
                        target: BaseId::new(need(self.target, "target")?),
                    },
                    Some("cover") => IntentKind::Cover {
                        base: BaseId::new(need(self.base, "base")?),
                    },
                    Some("hold") => IntentKind::Hold {
                        base: BaseId::new(need(self.base, "base")?),
                    },
                    _ => IntentKind::KeepSupplied {
                        base: BaseId::new(need(self.base, "base")?),
                        floor_days: need(self.floor_days, "floor_days")?,
                        interval_days: self.interval_days,
                    },
                },
            },
            "withdraw_intent" => CommandKind::WithdrawIntent { intent: intent()? },
            "set_limit" => CommandKind::SetLimit(match self.limit.as_deref() {
                Some("forces") => Limit::Forces {
                    intent: intent()?,
                    limit: ForcesLimit {
                        ships: self
                            .ships
                            .iter()
                            .flatten()
                            .copied()
                            .map(ShipId::new)
                            .collect(),
                        task_forces: self
                            .task_forces
                            .iter()
                            .flatten()
                            .copied()
                            .map(TaskForceId::new)
                            .collect(),
                        air_groups: self
                            .air_groups
                            .iter()
                            .flatten()
                            .copied()
                            .map(AirGroupId::new)
                            .collect(),
                        source: self.source.map(BaseId::new),
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
            "hold" | "release" => {
                let named = [self.ship, self.task_force, self.air_group]
                    .iter()
                    .filter(|n| n.is_some())
                    .count();
                if named != 1 {
                    return Err(format!(
                        "{what} needs exactly one of \"ship\", \"task_force\" and \"air_group\""
                    ));
                }
                let target = if let Some(ship) = self.ship {
                    if self.setting.is_some() {
                        return Err(format!("{what} takes \"setting\" only with \"air_group\""));
                    }
                    HoldTarget::Ship(ShipId::new(ship))
                } else if let Some(force) = self.task_force {
                    if self.setting.is_some() {
                        return Err(format!("{what} takes \"setting\" only with \"air_group\""));
                    }
                    HoldTarget::TaskForce(TaskForceId::new(force))
                } else {
                    let group = AirGroupId::new(self.air_group.expect("one target is named"));
                    match self.setting.as_deref() {
                        Some("search_arc") => HoldTarget::Setting {
                            group,
                            setting: HeldSetting::SearchArc,
                        },
                        Some(other) => {
                            return Err(format!(
                                "{what} has setting \"{other}\", which is not search_arc"
                            ))
                        }
                        // No setting: the whole group is held.
                        None => HoldTarget::AirGroup(group),
                    }
                };
                if self.kind == "hold" {
                    CommandKind::Hold { target }
                } else {
                    CommandKind::Release { target }
                }
            }
            "set_interrupt" => match self.standing.as_deref() {
                Some(name) => CommandKind::SetInterrupt(InterruptSetting::Standing {
                    kind: standing_kind(name)?,
                    on: self.on.ok_or_else(|| format!("{what} needs \"on\""))?,
                }),
                None => CommandKind::SetInterrupt(InterruptSetting::Set {
                    interrupt: InterruptId::new(need(self.interrupt, "interrupt")?),
                    condition: InterruptCondition::SupplyBelow {
                        base: BaseId::new(need(self.base, "base")?),
                        days: need(self.below_days, "below_days")?,
                    },
                }),
            },
            "acknowledge" => CommandKind::Acknowledge {
                interrupt: match self.standing.as_deref() {
                    Some(name) => {
                        let subject = match (self.plan, self.ship) {
                            (Some(plan), None) => StandingSubject::Plan(PlanId::new(plan)),
                            (None, Some(ship)) => StandingSubject::Ship(ShipId::new(ship)),
                            _ => {
                                return Err(format!(
                                    "{what} needs exactly one of \"plan\" and \"ship\" with \"standing\""
                                ))
                            }
                        };
                        InterruptRef::Standing {
                            kind: standing_kind(name)?,
                            subject,
                        }
                    }
                    None => InterruptRef::Set(InterruptId::new(need(self.interrupt, "interrupt")?)),
                },
            },
            _ => CommandKind::CancelPlan {
                plan: PlanId::new(need(self.plan, "plan")?),
            },
        };
        Ok((day, self.side.into(), kind))
    }
}
