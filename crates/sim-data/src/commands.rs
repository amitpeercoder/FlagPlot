//! A scenario's `[[command]]` tables (decision 0107, item 16): what a side's
//! commander says on a day, as a scenario states the orders it gives, so that
//! a test can begin with an intent in force.
//!
//! Only the commander's kinds are written here: `set_intent`,
//! `withdraw_intent`, `set_limit`, `set_leave`, `approve`, `cancel_plan`,
//! `hold`, `release`, `set_interrupt`, `acknowledge`, `set_pursuit` and `answer_pursuit`. The staff's kinds (`commit_ships`, `end_run` and the
//! rest) are the staff's to give and a scenario cannot state
//! them. Every table has `day`, `side` and `kind`, and the keys its kind
//! takes; a key the kind does not take, a key that is not a key of any
//! command, a TOML float and a number that is not whole are all refused.
//!
//! | `kind` | keys |
//! | --- | --- |
//! | `set_intent` | `intent`, `base`, and `intent_kind`: `"keep_supplied"` (the default when absent, so a scenario written before the kinds reads as it did), which also takes `floor_days` and `interval_days` (optional), `"cover"` (cover `base` from the air) or `"hold"` (hold `base`), which take neither, `"neutralise"` (neutralise the enemy base `target`; takes `target` and no `base`) or `"raid"` (raid `target` from `base`; takes both), `"deny"` (deny the waters within `distance` whole hexes, 0 or more, of `hex` as `[q, r]`; takes `hex` and `distance` and no `base`), `"strike_ships"` (strike ships from `base`; takes `base`), `"search_waters"` (search the waters from `base`; takes `base`, and is the staff's to give, so the game refuses it as a root intent), `"stand_at"` (stand at `hex` as `[q, r]` with the task force `task_force`, decision 0124) or `"operate_carriers"` (operate the carrier force `task_force` from `hex`, decision 0125); each of those two takes `task_force` and `hex` and no `base`; `"ready_landing"` (ready a landing by the task force `task_force` at the coast hex `hex`, with optional `by_day`, a whole number from 1, decision 0141) takes those and no `base`; `"build_up"` (build up `base`, decision 0141) takes `base`, `wants`, a list of any of `"airfield"`, `"mend"`, `"defences"`, `"port"` and `"aircraft"` in order, each at most once, and `defences`, the points from 1 to 100, which is required when `"defences"` is a want and refused otherwise; `"take_place"` (take the hex of land `hex` as `[q, r]`, with optional `by_day`, a whole number from 1, decision 0142) takes those and no `base`, `task_force` or other key; `"control_island"` (control the island `island`, a number the scenario names in an `[[island]]` table, with optional `by_day`, a whole number from 1, decision 0151) takes those and no other key; `"bombard"` (bombard the enemy base `target` with the task force `task_force`, decision 0146) takes both and no `base`, `hex` or other key; `"patrol"` (patrol the waters within `distance` whole hexes, 0 or more, of `hex` as `[q, r]` with submarines, decision 0218, 11) takes `hex` and `distance` and no other key |
//! | `withdraw_intent` | `intent` |
//! | `set_limit` | `limit` and its keys, below |
//! | `set_leave` | `leave` (`"prepare"` or `"launch"`), `given` |
//! | `set_pursuit` | `intent` and `leave` (`"ask"`, `"go"` or `"stay"`; left out clears the setting so that it is read up the tree again), decision 0214 |
//! | `answer_pursuit` | `task_force`, `contact` (the commander's number of the contact) and `allow` (a bool), decision 0214 |
//! | `approve`, `cancel_plan` | `plan` |
//! | `hold`, `release` | exactly one of `ship`, `task_force`, `air_group`, `land_unit` and `submarine`; with `air_group`, `setting` (`"search_arc"`) holds that setting alone, and with no `setting` the whole group is held; no other target takes `setting` |
//!
//! | `set_interrupt` | either `interrupt`, `base`, `below_days` (a threshold set), or `standing` (`"cannot_meet"`, `"waits_for_leave"`, `"ship_lost"`, `"cannot_bring_enough"`, `"lift_turned_back"` or `"asks_leave_to_pursue"`) and `on` (switch a standing kind off or on); never a mixture of the two |
//! | `acknowledge` | either `interrupt`, or `standing` with exactly one of `plan`, `ship` and `task_force`; never a mixture |
//!
//! The limits: `limit = "forces"` has `intent` and, each optional (a list
//! not given is empty), `ships`, `task_forces`, `air_groups`, `land_units` and `submarines`, and `source`
//! (a base; a supply intent's forces limit must name one, another kind's
//! need not); `"area"` has `intent` and `hexes` as `[[q, r], ...]`;
//! `"priority"` has `intent` and `rank`; `"protected"` has `intent` and
//! `ships`; `"reserve"` has `base`, `supply_t` and `fuel_t` as whole numbers
//! or decimal strings; `"risk"` has `intent` and `risk` (`"preserve"`,
//! `"balanced"` or `"press"`). Decision 0159, 9: `"not_before"` has `intent`
//! and `from_day`; `"wait_for"` has `intent` and one of `task_force` and
//! `land_unit`; `"after"` has `intent` and `other_intent`. Each with its key
//! left out clears the limit. Decision 0159, 12: `"knowledge"` has `intent`,
//! `knowledge` (`"none"`, `"count"` or `"recent_count"`; left out clears the
//! limit) and, with `"recent_count"` alone, `within_days` (3 where left
//! out). Decision 0190: `"targets"` has `intent`, `kinds` (ship kind names,
//! most wanted first; left out or empty clears the limit) and `waters_first`
//! (a bool, false where left out).

use std::collections::BTreeSet;

use serde::Deserialize;
use sim_core::command::{
    Awaited, CommandKind, ForcesLimit, HeldSetting, HoldTarget, IntentKind, InterruptCondition,
    InterruptRef, InterruptSetting, Knowledge, Leave, Limit, PlanId, PursueLeave, Risk,
    StandingKind, StandingSubject, TargetsLimit, Want, Wants,
};
use sim_core::hex::Hex;
use sim_core::ids::{
    AirGroupId, BaseId, IntentId, InterruptId, LandUnitId, ShipId, SubmarineId, TaskForceId,
};
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
    hex: Option<[i32; 2]>,
    distance: Option<u32>,
    floor_days: Option<u32>,
    interval_days: Option<u32>,
    ships: Option<Vec<u32>>,
    task_forces: Option<Vec<u32>>,
    air_groups: Option<Vec<u32>>,
    land_units: Option<Vec<u32>>,
    submarines: Option<Vec<u32>>,
    wants: Option<Vec<String>>,
    defences: Option<u32>,
    by_day: Option<u32>,
    island: Option<u32>,
    source: Option<u32>,
    hexes: Option<Vec<[i32; 2]>>,
    rank: Option<u32>,
    supply_t: Option<Quantity>,
    fuel_t: Option<Quantity>,
    risk: Option<String>,
    from_day: Option<u32>,
    other_intent: Option<u32>,
    knowledge: Option<String>,
    within_days: Option<i64>,
    kinds: Option<Vec<String>>,
    waters_first: Option<bool>,
    leave: Option<String>,
    given: Option<bool>,
    plan: Option<u32>,
    ship: Option<u32>,
    task_force: Option<u32>,
    air_group: Option<u32>,
    land_unit: Option<u32>,
    submarine: Option<u32>,
    setting: Option<String>,
    interrupt: Option<u32>,
    below_days: Option<u32>,
    standing: Option<String>,
    on: Option<bool>,
    /// A contact's number in the side's series (`answer_pursuit`).
    contact: Option<u32>,
    /// Yes or no to a force's asking leave to pursue (`answer_pursuit`).
    allow: Option<bool>,
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
        note(self.hex.is_some(), "hex");
        note(self.distance.is_some(), "distance");
        note(self.floor_days.is_some(), "floor_days");
        note(self.interval_days.is_some(), "interval_days");
        note(self.ships.is_some(), "ships");
        note(self.task_forces.is_some(), "task_forces");
        note(self.air_groups.is_some(), "air_groups");
        note(self.land_units.is_some(), "land_units");
        note(self.submarines.is_some(), "submarines");
        note(self.wants.is_some(), "wants");
        note(self.defences.is_some(), "defences");
        note(self.by_day.is_some(), "by_day");
        note(self.island.is_some(), "island");
        note(self.source.is_some(), "source");
        note(self.hexes.is_some(), "hexes");
        note(self.rank.is_some(), "rank");
        note(self.supply_t.is_some(), "supply_t");
        note(self.fuel_t.is_some(), "fuel_t");
        note(self.risk.is_some(), "risk");
        note(self.from_day.is_some(), "from_day");
        note(self.other_intent.is_some(), "other_intent");
        note(self.knowledge.is_some(), "knowledge");
        note(self.within_days.is_some(), "within_days");
        note(self.kinds.is_some(), "kinds");
        note(self.waters_first.is_some(), "waters_first");
        note(self.leave.is_some(), "leave");
        note(self.given.is_some(), "given");
        note(self.plan.is_some(), "plan");
        note(self.ship.is_some(), "ship");
        note(self.task_force.is_some(), "task_force");
        note(self.air_group.is_some(), "air_group");
        note(self.land_unit.is_some(), "land_unit");
        note(self.submarine.is_some(), "submarine");
        note(self.setting.is_some(), "setting");
        note(self.interrupt.is_some(), "interrupt");
        note(self.below_days.is_some(), "below_days");
        note(self.standing.is_some(), "standing");
        note(self.on.is_some(), "on");
        note(self.contact.is_some(), "contact");
        note(self.allow.is_some(), "allow");
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
        let kind = self.build_kind(&format!("the {} command on day {day}", self.kind))?;
        Ok((day, self.side.into(), kind))
    }

    /// The command the table states, its keys checked as its kind takes
    /// them; `what` names the table in a refusal.
    fn build_kind(&self, what: &str) -> Result<CommandKind, String> {
        let day = self.day;
        let what = what.to_string();
        // The keys the kind takes, and so the keys it refuses.
        let allowed: &[&str] = match (self.kind.as_str(), self.limit.as_deref()) {
            ("set_intent", _) => match self.intent_kind.as_deref() {
                None | Some("keep_supplied") => {
                    &["intent", "intent_kind", "base", "floor_days", "interval_days"]
                }
                Some("cover" | "hold" | "strike_ships" | "search_waters") => {
                    &["intent", "intent_kind", "base"]
                }
                Some("deny" | "patrol") => &["intent", "intent_kind", "hex", "distance"],
                Some("stand_at" | "operate_carriers") => {
                    &["intent", "intent_kind", "task_force", "hex"]
                }
                Some("ready_landing") => {
                    &["intent", "intent_kind", "task_force", "hex", "by_day"]
                }
                Some("build_up") => &["intent", "intent_kind", "base", "wants", "defences"],
                Some("take_place") => &["intent", "intent_kind", "hex", "by_day"],
                Some("control_island") => &["intent", "intent_kind", "island", "by_day"],
                Some("neutralise") => &["intent", "intent_kind", "target"],
                Some("bombard") => &["intent", "intent_kind", "task_force", "target"],
                Some("raid") => &["intent", "intent_kind", "base", "target"],
                Some(other) => {
                    return Err(format!(
                        "{what} has intent_kind \"{other}\", which is not keep_supplied, cover, hold, neutralise, raid, deny, strike_ships, search_waters, stand_at, operate_carriers, ready_landing, build_up, take_place, control_island or bombard, or patrol"
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
                "land_units",
                "submarines",
                "source",
            ],
            ("set_limit", Some("area")) => &["limit", "intent", "hexes"],
            ("set_limit", Some("priority")) => &["limit", "intent", "rank"],
            ("set_limit", Some("protected")) => &["limit", "intent", "ships"],
            ("set_limit", Some("reserve")) => &["limit", "base", "supply_t", "fuel_t"],
            ("set_limit", Some("risk")) => &["limit", "intent", "risk"],
            ("set_limit", Some("not_before")) => &["limit", "intent", "from_day"],
            ("set_limit", Some("wait_for")) => {
                if self.task_force.is_some() && self.land_unit.is_some() {
                    return Err(format!(
                        "{what} names both a task_force and a land_unit: a wait_for names one of them, or neither to clear it"
                    ));
                }
                &["limit", "intent", "task_force", "land_unit"]
            }
            ("set_limit", Some("after")) => &["limit", "intent", "other_intent"],
            ("set_limit", Some("knowledge")) => &["limit", "intent", "knowledge", "within_days"],
            ("set_limit", Some("targets")) => &["limit", "intent", "kinds", "waters_first"],
            ("set_limit", Some(other)) => {
                return Err(format!(
                    "{what} has limit \"{other}\", which is not forces, area, priority, protected, reserve, risk, not_before, wait_for, after, knowledge or targets"
                ))
            }
            ("set_limit", None) => {
                return Err(format!("{what} needs a limit"));
            }
            ("set_leave", _) => &["leave", "given"],
            ("set_pursuit", _) => &["intent", "leave"],
            ("answer_pursuit", _) => &["task_force", "contact", "allow"],
            ("approve" | "cancel_plan", _) => &["plan"],
            ("hold" | "release", _) => &[
                "ship",
                "task_force",
                "air_group",
                "land_unit",
                "submarine",
                "setting",
            ],
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
                        self.standing.is_some()
                            || self.plan.is_some()
                            || self.ship.is_some()
                            || self.task_force.is_some(),
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
                    (_, _) => &["standing", "plan", "ship", "task_force"],
                }
            }
            (other, _) => {
                return Err(format!(
                    "the command on day {day} is \"{other}\", which is not set_intent, withdraw_intent, set_limit, set_leave, approve, cancel_plan, hold, release, set_interrupt, acknowledge, set_pursuit or answer_pursuit (the staff's commands are the staff's to give)"
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
                    "{what} has standing kind \"{name}\", which is not cannot_meet, waits_for_leave, ship_lost, cannot_bring_enough, lift_turned_back or asks_leave_to_pursue"
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
                    Some("deny") => {
                        let [q, r] = self.hex.ok_or_else(|| format!("{what} needs \"hex\""))?;
                        IntentKind::Deny {
                            hex: Hex::new(q, r),
                            distance: need(self.distance, "distance")?,
                        }
                    }
                    // Decision 0218, 11.
                    Some("patrol") => {
                        let [q, r] = self.hex.ok_or_else(|| format!("{what} needs \"hex\""))?;
                        IntentKind::Patrol {
                            hex: Hex::new(q, r),
                            distance: need(self.distance, "distance")?,
                        }
                    }
                    Some("strike_ships") => IntentKind::StrikeShips {
                        base: BaseId::new(need(self.base, "base")?),
                    },
                    // Decisions 0124, 7, and 0125, 1.
                    Some(name @ ("stand_at" | "operate_carriers")) => {
                        let [q, r] = self.hex.ok_or_else(|| format!("{what} needs \"hex\""))?;
                        let force = TaskForceId::new(need(self.task_force, "task_force")?);
                        let hex = Hex::new(q, r);
                        if name == "stand_at" {
                            IntentKind::StandAt { force, hex }
                        } else {
                            IntentKind::OperateCarriers { force, hex }
                        }
                    }
                    // Decision 0141, 1.
                    Some("ready_landing") => {
                        let [q, r] = self.hex.ok_or_else(|| format!("{what} needs \"hex\""))?;
                        if self.by_day == Some(0) {
                            return Err(format!("{what} has by_day 0, which is not a day from 1"));
                        }
                        IntentKind::ReadyLanding {
                            force: TaskForceId::new(need(self.task_force, "task_force")?),
                            hex: Hex::new(q, r),
                            by_day: self.by_day,
                        }
                    }
                    // Decision 0142, 1.
                    Some("take_place") => {
                        let [q, r] = self.hex.ok_or_else(|| format!("{what} needs \"hex\""))?;
                        if self.by_day == Some(0) {
                            return Err(format!("{what} has by_day 0, which is not a day from 1"));
                        }
                        IntentKind::TakePlace {
                            hex: Hex::new(q, r),
                            by_day: self.by_day,
                        }
                    }
                    // Decision 0151, 3.
                    Some("control_island") => {
                        if self.by_day == Some(0) {
                            return Err(format!("{what} has by_day 0, which is not a day from 1"));
                        }
                        IntentKind::ControlIsland {
                            island: need(self.island, "island")?,
                            by_day: self.by_day,
                        }
                    }
                    // Decision 0146.
                    Some("bombard") => IntentKind::Bombard {
                        force: TaskForceId::new(need(self.task_force, "task_force")?),
                        target: BaseId::new(need(self.target, "target")?),
                    },
                    // Decision 0141, 8 and 10.
                    Some("build_up") => {
                        let names = self
                            .wants
                            .as_ref()
                            .ok_or_else(|| format!("{what} needs \"wants\""))?;
                        let asks_defences = names.iter().any(|n| n == "defences");
                        if asks_defences != self.defences.is_some() {
                            return Err(if asks_defences {
                                format!(
                                    "{what} wants \"defences\" and needs \"defences\", the points"
                                )
                            } else {
                                format!("{what} gives \"defences\" but does not want \"defences\"")
                            });
                        }
                        let mut list = Vec::new();
                        for name in names {
                            list.push(Want::from_name(name, self.defences).ok_or_else(|| {
                                format!(
                                    "{what} has want \"{name}\", which is not airfield, mend, defences, port or aircraft"
                                )
                            })?);
                        }
                        IntentKind::BuildUp {
                            base: BaseId::new(need(self.base, "base")?),
                            wants: Wants::new(&list).ok_or_else(|| {
                                format!(
                                    "{what} has wants that are empty, name a kind twice, or give defences outside 1 to 100 points"
                                )
                            })?,
                        }
                    }
                    Some("search_waters") => IntentKind::SearchWaters {
                        base: BaseId::new(need(self.base, "base")?),
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
                        land_units: self
                            .land_units
                            .iter()
                            .flatten()
                            .copied()
                            .map(LandUnitId::new)
                            .collect(),
                        source: self.source.map(BaseId::new),
                        submarines: self
                            .submarines
                            .iter()
                            .flatten()
                            .copied()
                            .map(SubmarineId::new)
                            .collect(),
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
                // Decision 0159, 9: a key left out clears the limit.
                Some("not_before") => Limit::NotBefore {
                    intent: intent()?,
                    day: self.from_day,
                },
                Some("wait_for") => Limit::WaitFor {
                    intent: intent()?,
                    awaited: match (self.task_force, self.land_unit) {
                        (Some(force), _) => Some(Awaited::TaskForce(TaskForceId::new(force))),
                        (None, Some(unit)) => Some(Awaited::LandUnit(LandUnitId::new(unit))),
                        (None, None) => None,
                    },
                },
                Some("after") => Limit::After {
                    intent: intent()?,
                    other: self.other_intent.map(IntentId::new),
                },
                // Decision 0159, 12: `knowledge` left out clears the limit;
                // `within_days` goes only with a recent count, 3 where it is
                // left out. A count of 0 days reaches the game and is refused
                // there; one below 0 cannot be written.
                Some("knowledge") => Limit::Knowledge {
                    intent: intent()?,
                    knowledge: match (self.knowledge.as_deref(), self.within_days) {
                        (Some("recent_count"), None) => {
                            Some(Knowledge::RecentCount(Knowledge::RECENT_DAYS))
                        }
                        (Some("recent_count"), Some(days)) => Some(Knowledge::RecentCount(
                            u32::try_from(days).map_err(|_| {
                                format!(
                                    "{what} has within_days {days}: a recent count is at least 1 day"
                                )
                            })?,
                        )),
                        (_, Some(_)) => {
                            return Err(format!(
                                "{what} has within_days, which only a knowledge of recent_count takes"
                            ))
                        }
                        (Some("none"), None) => Some(Knowledge::NoCount),
                        (Some("count"), None) => Some(Knowledge::Count),
                        (Some(other), None) => {
                            return Err(format!(
                                "{what} has knowledge \"{other}\", which is not none, count or recent_count"
                            ))
                        }
                        (None, None) => None,
                    },
                },
                // Decision 0190: `kinds` (ship kind names, most wanted first)
                // left out or empty clears the limit; `waters_first` is
                // false where left out.
                Some("targets") => Limit::Targets {
                    intent: intent()?,
                    targets: match self.kinds.as_deref() {
                        None | Some([]) => None,
                        Some(names) => {
                            let mut kinds = Vec::new();
                            for name in names {
                                kinds.push(crate::reference::parse_kind(name).ok_or_else(|| {
                                    format!(
                                        "{what} has kind \"{name}\", which is not a kind of ship (carrier, battleship, cruiser, destroyer, transport or auxiliary)"
                                    )
                                })?);
                            }
                            Some(TargetsLimit {
                                kinds,
                                waters_first: self.waters_first.unwrap_or(false),
                            })
                        }
                    },
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
            // Decision 0214, rule 4: `leave` left out clears the setting.
            "set_pursuit" => CommandKind::SetPursuit {
                intent: intent()?,
                leave: match self.leave.as_deref() {
                    None => None,
                    Some(name) => Some(PursueLeave::from_name(name).ok_or_else(|| {
                        format!("{what} has leave \"{name}\", which is not ask, go or stay")
                    })?),
                },
            },
            // Decision 0214, rule 8.
            "answer_pursuit" => CommandKind::AnswerPursuit {
                force: TaskForceId::new(need(self.task_force, "task_force")?),
                contact: need(self.contact, "contact")?,
                allow: self
                    .allow
                    .ok_or_else(|| format!("{what} needs \"allow\""))?,
            },
            "approve" => CommandKind::Approve {
                plan: PlanId::new(need(self.plan, "plan")?),
            },
            "hold" | "release" => {
                let named = [
                    self.ship,
                    self.task_force,
                    self.air_group,
                    self.land_unit,
                    self.submarine,
                ]
                .iter()
                    .filter(|n| n.is_some())
                    .count();
                if named != 1 {
                    return Err(format!(
                        "{what} needs exactly one of \"ship\", \"task_force\", \"air_group\", \"land_unit\" and \"submarine\""
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
                } else if let Some(unit) = self.land_unit {
                    if self.setting.is_some() {
                        return Err(format!("{what} takes \"setting\" only with \"air_group\""));
                    }
                    HoldTarget::LandUnit(LandUnitId::new(unit))
                } else if let Some(boat) = self.submarine {
                    if self.setting.is_some() {
                        return Err(format!("{what} takes \"setting\" only with \"air_group\""));
                    }
                    HoldTarget::Submarine(SubmarineId::new(boat))
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
                            (Some(plan), None) if self.task_force.is_none() => {
                                StandingSubject::Plan(PlanId::new(plan))
                            }
                            (None, Some(ship)) if self.task_force.is_none() => {
                                StandingSubject::Ship(ShipId::new(ship))
                            }
                            (None, None) if self.task_force.is_some() => {
                                StandingSubject::TaskForce(TaskForceId::new(
                                    self.task_force.unwrap_or_default(),
                                ))
                            }
                            _ => {
                                return Err(format!(
                                    "{what} needs exactly one of \"plan\", \"ship\" and \"task_force\" with \"standing\""
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
        Ok(kind)
    }
}

/// One `[[believed_intent]]` table (decision 0173, 7): an intent a side
/// believes the enemy holds. It has `side`, the side that believes it,
/// `intent_kind` (required) and the keys that kind of `set_intent` takes, as
/// in a `[[command]]` table, with no `day`, `kind` or `intent` number; and
/// `placeholder` and `note`. The keys are checked by the same code as a
/// command's, for the enemy's side.
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct BelievedIntentSpec {
    pub(crate) side: SideSpec,
    intent_kind: Option<String>,
    base: Option<u32>,
    target: Option<u32>,
    hex: Option<[i32; 2]>,
    distance: Option<u32>,
    floor_days: Option<u32>,
    interval_days: Option<u32>,
    task_force: Option<u32>,
    wants: Option<Vec<String>>,
    defences: Option<u32>,
    by_day: Option<u32>,
    island: Option<u32>,
    pub(crate) placeholder: Option<bool>,
    pub(crate) note: Option<String>,
}

impl BelievedIntentSpec {
    /// The intent the believing side's enemy is believed to hold, its keys
    /// checked as those of a `set_intent` command of that kind; `what` names
    /// the table in a refusal. What the game would refuse of it (a base, a
    /// force, a hex) is checked after, against the world.
    pub(crate) fn intent_kind(&self, what: &str) -> Result<IntentKind, String> {
        match self.intent_kind.as_deref() {
            None => return Err(format!("{what} needs \"intent_kind\"")),
            // The architect's ruling on reading 10 of decision 0173's "As
            // built": a side cannot name a force of the enemy's by its
            // number, so no kind that takes a task force is believed.
            Some(name @ ("stand_at" | "operate_carriers" | "ready_landing" | "bombard")) => {
                return Err(format!(
                    "{what} has intent_kind \"{name}\", which names a task force: a side cannot name a force of the enemy's, so a believed intent is of a kind that names only a base, a hex or an island"
                ))
            }
            Some(_) => {}
        }
        let enemy = match Side::from(self.side) {
            Side::Allied => SideSpec::Japanese,
            Side::Japanese => SideSpec::Allied,
        };
        let command = CommandSpec {
            day: 1,
            side: enemy,
            kind: "set_intent".to_string(),
            limit: None,
            intent: Some(1),
            intent_kind: self.intent_kind.clone(),
            base: self.base,
            target: self.target,
            hex: self.hex,
            distance: self.distance,
            floor_days: self.floor_days,
            interval_days: self.interval_days,
            ships: None,
            task_forces: None,
            air_groups: None,
            land_units: None,
            submarines: None,
            wants: self.wants.clone(),
            defences: self.defences,
            by_day: self.by_day,
            island: self.island,
            source: None,
            hexes: None,
            rank: None,
            supply_t: None,
            fuel_t: None,
            risk: None,
            from_day: None,
            other_intent: None,
            knowledge: None,
            within_days: None,
            kinds: None,
            waters_first: None,
            leave: None,
            given: None,
            plan: None,
            ship: None,
            task_force: self.task_force,
            air_group: None,
            land_unit: None,
            submarine: None,
            setting: None,
            interrupt: None,
            below_days: None,
            standing: None,
            on: None,
            contact: None,
            allow: None,
        };
        match command.build_kind(what)? {
            CommandKind::SetIntent { kind, .. } => Ok(kind),
            _ => unreachable!("a set_intent table builds a set_intent command"),
        }
    }
}
