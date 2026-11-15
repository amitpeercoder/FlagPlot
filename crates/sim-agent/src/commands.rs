//! Commands and a staff's decisions as JSON, in the form saved games use
//! (decision 0107, "As built").
//!
//! One encoding for the saved game and for the agent interface, as
//! [`crate::orders`] is for orders: a command is an object whose `"kind"`
//! names it, with the same key names a scenario file's `[[command]]` table
//! uses. Every quantity is a whole number or a decimal string (a weight in
//! tons, `"supply_t"`), never a float, so the same command always gives the
//! same bytes and reads back as the same command. A set is written in order.
//!
//! A command's body:
//!
//! | `kind` | other keys |
//! | --- | --- |
//! | `set_intent` | `intent`, `base`, `floor_days`, `interval_days` (left out when not set) |
//! | `withdraw_intent` | `intent` |
//! | `set_limit` | `limit` (`forces`, `area`, `priority`, `protected`, `reserve` or `risk`) and its keys, below |
//! | `set_leave` | `leave` (`prepare` or `launch`), `given` |
//! | `approve`, `cancel_plan` | `plan` |
//! | `commit_ships` | `plan`, `run` (left out when none), `ships`, `from_day`, `to_day` |
//! | `free_ships` | `plan`, `ships` |
//! | `add_run` | `plan`, `ships`, `load_t`, `days` (`load`, `sail`, `arrive`, `free`) |
//! | `time_run` | `plan`, `run`, `days` |
//! | `mark_plan` | `plan`, `outcome` (`complete` or `failed`) |
//! | `hold`, `release` | one of `ship`, `task_force`, or `air_group` with `setting` (`search_arc`) |
//! | `end_run` | `plan`, `run` |
//!
//! The limits: `forces` has `intent`, `ships`, `task_forces`, `source`;
//! `area` has `intent`, `hexes` as `[[q, r], ...]`; `priority` has `intent`
//! and `rank` (`null` for unranked); `protected` has `intent`, `ships`;
//! `reserve` has `base`, `supply_t`, `fuel_t`; `risk` has `intent` and `risk`
//! (`preserve`, `balanced` or `press`).
//!
//! What a staff handed in for a day is `{"orders", "commands", "no_orders"}`:
//! an order is its `unit`, its `order` body (as [`crate::order_to_json`]
//! writes it), the `intent` it serves and its `reason`; a reason is `serves`
//! (`intent`, `plan`, `amendment`, each left out when none), `evidence` (a
//! list of `{what, day}`), `binding`, `worked` and `would_change`.

use std::collections::BTreeSet;

use serde_json::{json, Map, Value};
use sim_core::command::{
    CommandKind, Evidence, ForcesLimit, HeldSetting, HoldTarget, IntentKind, Leave, Limit, PlanId,
    PlanOutcome, Reason, Risk, RunDays, RunSpec, Serves, StaffCommand, StaffDecisions, StaffOrder,
};
use sim_core::fixed::Fixed;
use sim_core::hex::Hex;
use sim_core::ids::{AirGroupId, BaseId, IntentId, ShipId, TaskForceId};

use crate::orders::{order_from_json, order_to_json, unit_text};

// ---------------------------------------------------------------------------
// Reading helpers.
// ---------------------------------------------------------------------------

fn text<'a>(value: &'a Value, key: &str) -> Result<&'a str, String> {
    value
        .get(key)
        .and_then(Value::as_str)
        .ok_or_else(|| format!("\"{key}\" is missing or not text"))
}

fn small(value: &Value, key: &str) -> Result<u32, String> {
    let n = value
        .get(key)
        .and_then(Value::as_i64)
        .ok_or_else(|| format!("\"{key}\" is missing or not a whole number"))?;
    u32::try_from(n).map_err(|_| format!("\"{key}\" is out of range"))
}

/// A whole number that may be left out or `null`.
fn small_opt(value: &Value, key: &str) -> Result<Option<u32>, String> {
    match value.get(key) {
        None | Some(Value::Null) => Ok(None),
        Some(_) => small(value, key).map(Some),
    }
}

fn fixed(value: &Value, key: &str) -> Result<Fixed, String> {
    text(value, key)?
        .parse::<Fixed>()
        .map_err(|e| format!("\"{key}\": {e}"))
}

fn flag(value: &Value, key: &str) -> Result<bool, String> {
    value
        .get(key)
        .and_then(Value::as_bool)
        .ok_or_else(|| format!("\"{key}\" is missing or not true or false"))
}

fn list<'a>(value: &'a Value, key: &str) -> Result<&'a Vec<Value>, String> {
    value
        .get(key)
        .and_then(Value::as_array)
        .ok_or_else(|| format!("\"{key}\" is missing or not a list"))
}

fn numbers(value: &Value, key: &str) -> Result<Vec<u32>, String> {
    list(value, key)?
        .iter()
        .map(|n| {
            n.as_i64()
                .and_then(|n| u32::try_from(n).ok())
                .ok_or_else(|| format!("\"{key}\" holds something that is not a whole number"))
        })
        .collect()
}

fn ship_set(value: &Value, key: &str) -> Result<BTreeSet<ShipId>, String> {
    Ok(numbers(value, key)?.into_iter().map(ShipId::new).collect())
}

fn texts(value: &Value, key: &str) -> Result<Vec<String>, String> {
    list(value, key)?
        .iter()
        .map(|s| {
            s.as_str()
                .map(str::to_string)
                .ok_or_else(|| format!("\"{key}\" holds something that is not text"))
        })
        .collect()
}

fn hex_set(value: &Value, key: &str) -> Result<BTreeSet<Hex>, String> {
    let mut hexes = BTreeSet::new();
    for pair in list(value, key)? {
        let pair = pair
            .as_array()
            .ok_or_else(|| format!("a hex of \"{key}\" is not a pair"))?;
        let get = |i: usize| {
            pair.get(i)
                .and_then(Value::as_i64)
                .and_then(|n| i32::try_from(n).ok())
                .ok_or_else(|| format!("a hex of \"{key}\" is not a pair of whole numbers"))
        };
        hexes.insert(Hex::new(get(0)?, get(1)?));
    }
    Ok(hexes)
}

fn raws(ships: &BTreeSet<ShipId>) -> Vec<u32> {
    ships.iter().map(|s| s.raw()).collect()
}

// ---------------------------------------------------------------------------
// Names.
// ---------------------------------------------------------------------------

fn risk_from(name: &str) -> Result<Risk, String> {
    match name {
        "preserve" => Ok(Risk::Preserve),
        "balanced" => Ok(Risk::Balanced),
        "press" => Ok(Risk::Press),
        other => Err(format!("unknown risk \"{other}\"")),
    }
}

fn leave_from(name: &str) -> Result<Leave, String> {
    match name {
        "prepare" => Ok(Leave::Prepare),
        "launch" => Ok(Leave::Launch),
        other => Err(format!("unknown leave \"{other}\"")),
    }
}

fn outcome_name(outcome: PlanOutcome) -> &'static str {
    match outcome {
        PlanOutcome::Complete => "complete",
        PlanOutcome::Failed => "failed",
    }
}

fn outcome_from(name: &str) -> Result<PlanOutcome, String> {
    match name {
        "complete" => Ok(PlanOutcome::Complete),
        "failed" => Ok(PlanOutcome::Failed),
        other => Err(format!("unknown plan outcome \"{other}\"")),
    }
}

fn setting_from(name: &str) -> Result<HeldSetting, String> {
    match name {
        "search_arc" => Ok(HeldSetting::SearchArc),
        other => Err(format!("unknown setting \"{other}\"")),
    }
}

/// A hold's target as the keys of a command's object: `ship`, `task_force`,
/// or `air_group` with `setting`.
fn target_to_json(body: &mut Value, target: HoldTarget) {
    match target {
        HoldTarget::Ship(ship) => body["ship"] = json!(ship.raw()),
        HoldTarget::TaskForce(force) => body["task_force"] = json!(force.raw()),
        HoldTarget::Setting { group, setting } => {
            body["air_group"] = json!(group.raw());
            body["setting"] = json!(setting.name());
        }
    }
}

fn target_from_json(value: &Value) -> Result<HoldTarget, String> {
    let named: Vec<&str> = ["ship", "task_force", "air_group"]
        .into_iter()
        .filter(|key| value.get(key).is_some())
        .collect();
    match named.as_slice() {
        ["ship"] => Ok(HoldTarget::Ship(ShipId::new(small(value, "ship")?))),
        ["task_force"] => Ok(HoldTarget::TaskForce(TaskForceId::new(small(
            value,
            "task_force",
        )?))),
        ["air_group"] => Ok(HoldTarget::Setting {
            group: AirGroupId::new(small(value, "air_group")?),
            setting: setting_from(text(value, "setting")?)?,
        }),
        [] => Err("a hold names one of \"ship\", \"task_force\" or \"air_group\"".to_string()),
        _ => Err("a hold names only one of \"ship\", \"task_force\" and \"air_group\"".to_string()),
    }
}

// ---------------------------------------------------------------------------
// Commands.
// ---------------------------------------------------------------------------

fn days_to_json(days: &RunDays) -> Value {
    json!({
        "load": days.load,
        "sail": days.sail,
        "arrive": days.arrive,
        "free": days.free,
    })
}

fn days_from_json(value: &Value, key: &str) -> Result<RunDays, String> {
    let days = value
        .get(key)
        .ok_or_else(|| format!("\"{key}\" is missing"))?;
    Ok(RunDays {
        load: small(days, "load")?,
        sail: small(days, "sail")?,
        arrive: small(days, "arrive")?,
        free: small(days, "free")?,
    })
}

fn limit_to_json(limit: &Limit) -> Value {
    match limit {
        Limit::Forces { intent, limit } => json!({
            "kind": "set_limit",
            "limit": "forces",
            "intent": intent.raw(),
            "ships": raws(&limit.ships),
            "task_forces": limit.task_forces.iter().map(|f| f.raw()).collect::<Vec<_>>(),
            "source": limit.source.raw(),
        }),
        Limit::Area { intent, hexes } => json!({
            "kind": "set_limit",
            "limit": "area",
            "intent": intent.raw(),
            "hexes": hexes.iter().map(|h| json!([h.q, h.r])).collect::<Vec<_>>(),
        }),
        Limit::Priority { intent, rank } => json!({
            "kind": "set_limit",
            "limit": "priority",
            "intent": intent.raw(),
            "rank": rank,
        }),
        Limit::Protected { intent, ships } => json!({
            "kind": "set_limit",
            "limit": "protected",
            "intent": intent.raw(),
            "ships": raws(ships),
        }),
        Limit::Reserve {
            base,
            supply_t,
            fuel_t,
        } => json!({
            "kind": "set_limit",
            "limit": "reserve",
            "base": base.raw(),
            "supply_t": supply_t.to_string(),
            "fuel_t": fuel_t.to_string(),
        }),
        Limit::Risk { intent, risk } => json!({
            "kind": "set_limit",
            "limit": "risk",
            "intent": intent.raw(),
            "risk": risk.name(),
        }),
    }
}

fn limit_from_json(value: &Value) -> Result<Limit, String> {
    let intent = || small(value, "intent").map(IntentId::new);
    match text(value, "limit")? {
        "forces" => Ok(Limit::Forces {
            intent: intent()?,
            limit: ForcesLimit {
                ships: ship_set(value, "ships")?,
                task_forces: numbers(value, "task_forces")?
                    .into_iter()
                    .map(TaskForceId::new)
                    .collect(),
                source: BaseId::new(small(value, "source")?),
            },
        }),
        "area" => Ok(Limit::Area {
            intent: intent()?,
            hexes: hex_set(value, "hexes")?,
        }),
        "priority" => Ok(Limit::Priority {
            intent: intent()?,
            rank: small_opt(value, "rank")?,
        }),
        "protected" => Ok(Limit::Protected {
            intent: intent()?,
            ships: ship_set(value, "ships")?,
        }),
        "reserve" => Ok(Limit::Reserve {
            base: BaseId::new(small(value, "base")?),
            supply_t: fixed(value, "supply_t")?,
            fuel_t: fixed(value, "fuel_t")?,
        }),
        "risk" => Ok(Limit::Risk {
            intent: intent()?,
            risk: risk_from(text(value, "risk")?)?,
        }),
        other => Err(format!("unknown limit \"{other}\"")),
    }
}

/// A command as JSON.
pub fn command_to_json(command: &CommandKind) -> Value {
    match command {
        CommandKind::SetIntent { intent, kind } => match kind {
            IntentKind::KeepSupplied {
                base,
                floor_days,
                interval_days,
            } => {
                let mut body = json!({
                    "kind": "set_intent",
                    "intent": intent.raw(),
                    "base": base.raw(),
                    "floor_days": floor_days,
                });
                if let Some(days) = interval_days {
                    body["interval_days"] = json!(days);
                }
                body
            }
        },
        CommandKind::WithdrawIntent { intent } => {
            json!({ "kind": "withdraw_intent", "intent": intent.raw() })
        }
        CommandKind::SetLimit(limit) => limit_to_json(limit),
        CommandKind::SetLeave { leave, given } => {
            json!({ "kind": "set_leave", "leave": leave.name(), "given": given })
        }
        CommandKind::Approve { plan } => json!({ "kind": "approve", "plan": plan.raw() }),
        CommandKind::CancelPlan { plan } => json!({ "kind": "cancel_plan", "plan": plan.raw() }),
        CommandKind::CommitShips {
            plan,
            run,
            ships,
            from_day,
            to_day,
        } => {
            let mut body = json!({
                "kind": "commit_ships",
                "plan": plan.raw(),
                "ships": raws(ships),
                "from_day": from_day,
                "to_day": to_day,
            });
            if let Some(run) = run {
                body["run"] = json!(run);
            }
            body
        }
        CommandKind::FreeShips { plan, ships } => json!({
            "kind": "free_ships",
            "plan": plan.raw(),
            "ships": raws(ships),
        }),
        CommandKind::AddRun { plan, run } => json!({
            "kind": "add_run",
            "plan": plan.raw(),
            "ships": raws(&run.ships),
            "load_t": run.load_t.to_string(),
            "days": days_to_json(&run.days),
        }),
        CommandKind::TimeRun { plan, run, days } => json!({
            "kind": "time_run",
            "plan": plan.raw(),
            "run": run,
            "days": days_to_json(days),
        }),
        CommandKind::MarkPlan { plan, outcome } => json!({
            "kind": "mark_plan",
            "plan": plan.raw(),
            "outcome": outcome_name(*outcome),
        }),
        CommandKind::Hold { target } => {
            let mut body = json!({ "kind": "hold" });
            target_to_json(&mut body, *target);
            body
        }
        CommandKind::Release { target } => {
            let mut body = json!({ "kind": "release" });
            target_to_json(&mut body, *target);
            body
        }
        CommandKind::EndRun { plan, run } => json!({
            "kind": "end_run",
            "plan": plan.raw(),
            "run": run,
        }),
    }
}

/// Reads a command from its JSON.
pub fn command_from_json(value: &Value) -> Result<CommandKind, String> {
    let plan = || small(value, "plan").map(PlanId::new);
    match text(value, "kind")? {
        "set_intent" => Ok(CommandKind::SetIntent {
            intent: IntentId::new(small(value, "intent")?),
            kind: IntentKind::KeepSupplied {
                base: BaseId::new(small(value, "base")?),
                floor_days: small(value, "floor_days")?,
                interval_days: small_opt(value, "interval_days")?,
            },
        }),
        "withdraw_intent" => Ok(CommandKind::WithdrawIntent {
            intent: IntentId::new(small(value, "intent")?),
        }),
        "set_limit" => Ok(CommandKind::SetLimit(limit_from_json(value)?)),
        "set_leave" => Ok(CommandKind::SetLeave {
            leave: leave_from(text(value, "leave")?)?,
            given: flag(value, "given")?,
        }),
        "approve" => Ok(CommandKind::Approve { plan: plan()? }),
        "cancel_plan" => Ok(CommandKind::CancelPlan { plan: plan()? }),
        "commit_ships" => Ok(CommandKind::CommitShips {
            plan: plan()?,
            run: small_opt(value, "run")?,
            ships: ship_set(value, "ships")?,
            from_day: small(value, "from_day")?,
            to_day: small(value, "to_day")?,
        }),
        "free_ships" => Ok(CommandKind::FreeShips {
            plan: plan()?,
            ships: ship_set(value, "ships")?,
        }),
        "add_run" => Ok(CommandKind::AddRun {
            plan: plan()?,
            run: RunSpec {
                ships: ship_set(value, "ships")?,
                load_t: fixed(value, "load_t")?,
                days: days_from_json(value, "days")?,
            },
        }),
        "time_run" => Ok(CommandKind::TimeRun {
            plan: plan()?,
            run: small(value, "run")?,
            days: days_from_json(value, "days")?,
        }),
        "mark_plan" => Ok(CommandKind::MarkPlan {
            plan: plan()?,
            outcome: outcome_from(text(value, "outcome")?)?,
        }),
        "hold" => Ok(CommandKind::Hold {
            target: target_from_json(value)?,
        }),
        "release" => Ok(CommandKind::Release {
            target: target_from_json(value)?,
        }),
        "end_run" => Ok(CommandKind::EndRun {
            plan: plan()?,
            run: small(value, "run")?,
        }),
        other => Err(format!("unknown command \"{other}\"")),
    }
}

// ---------------------------------------------------------------------------
// Reasons and a staff's decisions.
// ---------------------------------------------------------------------------

pub(crate) fn reason_to_json(reason: &Reason) -> Value {
    let mut serves = Map::new();
    if let Some(intent) = reason.serves.intent {
        serves.insert("intent".into(), json!(intent.raw()));
    }
    if let Some(plan) = reason.serves.plan {
        serves.insert("plan".into(), json!(plan.raw()));
    }
    if let Some(amendment) = reason.serves.amendment {
        serves.insert("amendment".into(), json!(amendment));
    }
    json!({
        "serves": Value::Object(serves),
        "evidence": reason
            .evidence
            .iter()
            .map(|e| json!({ "what": e.what, "day": e.day }))
            .collect::<Vec<_>>(),
        "binding": reason.binding,
        "worked": reason.worked,
        "would_change": reason.would_change,
    })
}

fn reason_from_json(value: &Value) -> Result<Reason, String> {
    let serves = value
        .get("serves")
        .ok_or_else(|| "\"serves\" is missing".to_string())?;
    let mut evidence = Vec::new();
    for entry in list(value, "evidence")? {
        evidence.push(Evidence {
            what: text(entry, "what")?.to_string(),
            day: small(entry, "day")?,
        });
    }
    Ok(Reason {
        serves: Serves {
            intent: small_opt(serves, "intent")?.map(IntentId::new),
            plan: small_opt(serves, "plan")?.map(PlanId::new),
            amendment: small_opt(serves, "amendment")?,
        },
        evidence,
        binding: texts(value, "binding")?,
        worked: text(value, "worked")?.to_string(),
        would_change: texts(value, "would_change")?,
    })
}

/// What a staff handed in for a day, as JSON: its orders with their intents
/// and reasons, its commands, and its decisions to give no order.
pub fn staff_decisions_to_json(decisions: &StaffDecisions) -> Value {
    json!({
        "orders": decisions
            .orders
            .iter()
            .map(|order| json!({
                "unit": unit_text(&order.kind),
                "order": order_to_json(&order.kind),
                "intent": order.intent.raw(),
                "reason": reason_to_json(&order.reason),
            }))
            .collect::<Vec<_>>(),
        "commands": decisions
            .commands
            .iter()
            .map(|command| {
                let mut value = command_to_json(&command.kind);
                if let Some(object) = value.as_object_mut() {
                    object.insert("reason".to_string(), reason_to_json(&command.reason));
                }
                value
            })
            .collect::<Vec<_>>(),
        "no_orders": decisions.no_orders.iter().map(reason_to_json).collect::<Vec<_>>(),
    })
}

/// Reads what a staff handed in from its JSON.
pub fn staff_decisions_from_json(value: &Value) -> Result<StaffDecisions, String> {
    let mut decisions = StaffDecisions::default();
    for entry in list(value, "orders")? {
        let unit = text(entry, "unit")?;
        let body = entry
            .get("order")
            .ok_or_else(|| "an order has no \"order\"".to_string())?;
        let kind = order_from_json(unit, body)?;
        decisions.orders.push(StaffOrder {
            kind,
            intent: IntentId::new(small(entry, "intent")?),
            reason: reason_from_json(
                entry
                    .get("reason")
                    .ok_or_else(|| "an order has no \"reason\"".to_string())?,
            )?,
        });
    }
    for entry in list(value, "commands")? {
        decisions.commands.push(StaffCommand {
            kind: command_from_json(entry)?,
            reason: reason_from_json(
                entry
                    .get("reason")
                    .ok_or_else(|| "a command has no \"reason\"".to_string())?,
            )?,
        });
    }
    for entry in list(value, "no_orders")? {
        decisions.no_orders.push(reason_from_json(entry)?);
    }
    Ok(decisions)
}

#[cfg(test)]
mod tests {
    use super::*;
    use sim_core::orders::{Cargo, OrderKind, TaskForceOrder};

    fn ships(list: &[u32]) -> BTreeSet<ShipId> {
        list.iter().copied().map(ShipId::new).collect()
    }

    fn every_command() -> Vec<CommandKind> {
        let intent = IntentId::new(2);
        let days = RunDays {
            load: 3,
            sail: 4,
            arrive: 9,
            free: 14,
        };
        vec![
            CommandKind::SetIntent {
                intent,
                kind: IntentKind::KeepSupplied {
                    base: BaseId::new(5),
                    floor_days: 12,
                    interval_days: Some(15),
                },
            },
            CommandKind::SetIntent {
                intent,
                kind: IntentKind::KeepSupplied {
                    base: BaseId::new(5),
                    floor_days: 12,
                    interval_days: None,
                },
            },
            CommandKind::WithdrawIntent { intent },
            CommandKind::SetLimit(Limit::Forces {
                intent,
                limit: ForcesLimit {
                    ships: ships(&[9, 4]),
                    task_forces: [TaskForceId::new(3), TaskForceId::new(1)]
                        .into_iter()
                        .collect(),
                    source: BaseId::new(2),
                },
            }),
            CommandKind::SetLimit(Limit::Area {
                intent,
                hexes: [Hex::new(3, -2), Hex::new(-1, 4)].into_iter().collect(),
            }),
            CommandKind::SetLimit(Limit::Priority {
                intent,
                rank: Some(1),
            }),
            CommandKind::SetLimit(Limit::Priority { intent, rank: None }),
            CommandKind::SetLimit(Limit::Protected {
                intent,
                ships: ships(&[7]),
            }),
            CommandKind::SetLimit(Limit::Reserve {
                base: BaseId::new(1),
                supply_t: Fixed::from_raw(400_000_001),
                fuel_t: Fixed::from_int(300),
            }),
            CommandKind::SetLimit(Limit::Risk {
                intent,
                risk: Risk::Press,
            }),
            CommandKind::SetLeave {
                leave: Leave::Launch,
                given: true,
            },
            CommandKind::SetLeave {
                leave: Leave::Prepare,
                given: false,
            },
            CommandKind::Approve {
                plan: PlanId::new(1),
            },
            CommandKind::CancelPlan {
                plan: PlanId::new(2),
            },
            CommandKind::CommitShips {
                plan: PlanId::new(1),
                run: Some(2),
                ships: ships(&[4, 5]),
                from_day: 3,
                to_day: 14,
            },
            CommandKind::CommitShips {
                plan: PlanId::new(1),
                run: None,
                ships: ships(&[4]),
                from_day: 3,
                to_day: 4,
            },
            CommandKind::FreeShips {
                plan: PlanId::new(1),
                ships: ships(&[4, 5]),
            },
            CommandKind::AddRun {
                plan: PlanId::new(1),
                run: RunSpec {
                    ships: ships(&[4, 5]),
                    load_t: Fixed::from_raw(1_500_250_000),
                    days,
                },
            },
            CommandKind::TimeRun {
                plan: PlanId::new(1),
                run: 1,
                days,
            },
            CommandKind::MarkPlan {
                plan: PlanId::new(1),
                outcome: PlanOutcome::Complete,
            },
            CommandKind::MarkPlan {
                plan: PlanId::new(1),
                outcome: PlanOutcome::Failed,
            },
            CommandKind::Hold {
                target: HoldTarget::Ship(ShipId::new(5)),
            },
            CommandKind::Hold {
                target: HoldTarget::TaskForce(TaskForceId::new(1_000_001)),
            },
            CommandKind::Hold {
                target: HoldTarget::Setting {
                    group: AirGroupId::new(7),
                    setting: HeldSetting::SearchArc,
                },
            },
            CommandKind::Release {
                target: HoldTarget::Ship(ShipId::new(5)),
            },
            CommandKind::Release {
                target: HoldTarget::Setting {
                    group: AirGroupId::new(7),
                    setting: HeldSetting::SearchArc,
                },
            },
            CommandKind::EndRun {
                plan: PlanId::new(1),
                run: 2,
            },
        ]
    }

    #[test]
    fn a_hold_target_is_written_as_its_raw_id_and_a_bad_one_is_refused() {
        let hold = |target| command_to_json(&CommandKind::Hold { target });
        assert_eq!(
            hold(HoldTarget::Ship(ShipId::new(5))),
            json!({ "kind": "hold", "ship": 5 })
        );
        assert_eq!(
            hold(HoldTarget::TaskForce(TaskForceId::new(9))),
            json!({ "kind": "hold", "task_force": 9 })
        );
        assert_eq!(
            hold(HoldTarget::Setting {
                group: AirGroupId::new(3),
                setting: HeldSetting::SearchArc,
            }),
            json!({ "kind": "hold", "air_group": 3, "setting": "search_arc" })
        );
        assert_eq!(
            command_to_json(&CommandKind::EndRun {
                plan: PlanId::new(1),
                run: 2
            }),
            json!({ "kind": "end_run", "plan": 1, "run": 2 })
        );
        for bad in [
            json!({ "kind": "hold" }),
            json!({ "kind": "hold", "ship": 1, "task_force": 2 }),
            json!({ "kind": "release", "air_group": 3 }),
            json!({ "kind": "release", "air_group": 3, "setting": "mission" }),
            json!({ "kind": "hold", "ship": -1 }),
            json!({ "kind": "end_run", "plan": 1 }),
        ] {
            assert!(command_from_json(&bad).is_err(), "{bad}");
        }
    }

    #[test]
    fn every_command_round_trips_exactly() {
        for command in every_command() {
            let value = command_to_json(&command);
            let text = serde_json::to_string(&value).expect("writes");
            let again: Value = serde_json::from_str(&text).expect("parses");
            assert_eq!(command_from_json(&again), Ok(command.clone()), "{text}");
            // The same command always gives the same bytes.
            assert_eq!(
                serde_json::to_string(&command_to_json(&command)).expect("writes"),
                text
            );
        }
    }

    #[test]
    fn a_weight_is_a_decimal_string_and_a_bad_command_is_refused() {
        let value = command_to_json(&CommandKind::SetLimit(Limit::Reserve {
            base: BaseId::new(1),
            supply_t: Fixed::from_raw(2_500_000),
            fuel_t: Fixed::from_int(3),
        }));
        assert_eq!(value["supply_t"], "2.5");
        assert_eq!(value["fuel_t"], "3");
        assert!(command_from_json(&json!({ "kind": "no_such" })).is_err());
        assert!(command_from_json(&json!({ "kind": "set_limit", "limit": "no_such" })).is_err());
        assert!(command_from_json(&json!({
            "kind": "set_limit", "limit": "reserve", "base": 1, "supply_t": 2.5, "fuel_t": "3"
        }))
        .is_err());
        assert!(command_from_json(&json!({ "kind": "approve", "plan": -1 })).is_err());
    }

    #[test]
    fn a_staffs_decisions_round_trip_exactly() {
        let reason = Reason {
            serves: Serves {
                intent: Some(IntentId::new(1)),
                plan: Some(PlanId::new(1)),
                amendment: Some(2),
            },
            evidence: vec![
                Evidence {
                    what: "stock at base 2 is 40 t".to_string(),
                    day: 3,
                },
                Evidence {
                    what: "ship 5 is in port".to_string(),
                    day: 4,
                },
            ],
            binding: vec!["the forces limit holds ship 5".to_string()],
            worked: "12 days x 10 t = 120 t".to_string(),
            would_change: vec!["a second ship in the pool".to_string()],
        };
        let decisions = StaffDecisions {
            orders: vec![StaffOrder {
                kind: OrderKind::TaskForce {
                    target: TaskForceId::new(3),
                    order: TaskForceOrder::LoadTo {
                        cargo: Cargo::Supply,
                        weight_t: Fixed::from_raw(1_200_500_000),
                    },
                },
                intent: IntentId::new(1),
                reason: reason.clone(),
            }],
            commands: every_command()
                .into_iter()
                .skip(14)
                .map(|kind| StaffCommand {
                    kind,
                    reason: reason.clone(),
                })
                .collect(),
            no_orders: vec![
                reason,
                Reason {
                    worked: "nothing to do".to_string(),
                    ..Reason::default()
                },
            ],
        };
        let value = staff_decisions_to_json(&decisions);
        let text = serde_json::to_string(&value).expect("writes");
        let again: Value = serde_json::from_str(&text).expect("parses");
        assert_eq!(staff_decisions_from_json(&again), Ok(decisions.clone()));
        assert_eq!(
            serde_json::to_string(&staff_decisions_to_json(&decisions)).expect("writes"),
            text
        );
        // Nothing handed in is written as three empty lists and reads back
        // as nothing.
        let none = staff_decisions_to_json(&StaffDecisions::default());
        assert_eq!(
            staff_decisions_from_json(&none),
            Ok(StaffDecisions::default())
        );
    }
}
