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
//! | `set_intent` | `intent`, `base`, `intent_kind` (`keep_supplied`, `cover` or `hold`; left out for `keep_supplied`, which is what a key's absence reads as), and for `keep_supplied` alone `floor_days` and `interval_days` (left out when not set) |
//! | `form_intent` | `parent`, `intent_kind` (`cover`), `base`: the staff's, a sub-intent under `parent` |
//! | `withdraw_intent` | `intent` |
//! | `set_limit` | `limit` (`forces`, `area`, `priority`, `protected`, `reserve` or `risk`) and its keys, below |
//! | `set_leave` | `leave` (`prepare` or `launch`), `given` |
//! | `approve`, `cancel_plan` | `plan` |
//! | `commit_ships` | `plan`, `run` (left out when none), `ships`, `from_day`, `to_day` |
//! | `free_ships` | `plan`, `ships` |
//! | `add_run` | `plan`, `ships`, `load_t`, `days` (`load`, `sail`, `arrive`, `free`) |
//! | `time_run` | `plan`, `run`, `days` |
//! | `mark_plan` | `plan`, `outcome` (`complete` or `failed`) |
//! | `hold`, `release` | one of `ship`, `task_force`, or `air_group`; with `air_group`, `setting` (`search_arc`) holds that setting alone and no `setting` holds the whole group |
//! | `end_run` | `plan`, `run` |
//! | `set_interrupt` | `interrupt`, `base`, `below_days` (a threshold set), or `standing` (`cannot_meet`, `waits_for_leave` or `ship_lost`) and `on` |
//! | `acknowledge` | `interrupt`, or `standing` with one of `plan` or `ship` |
//!
//! The limits: `forces` has `intent` and `ships`, `task_forces`, `air_groups`
//! and `source` (each left out when empty or not set, and each read as empty
//! or not set when absent);
//! `area` has `intent`, `hexes` as `[[q, r], ...]`; `priority` has `intent`
//! and `rank` (`null` for unranked); `protected` has `intent`, `ships`;
//! `reserve` has `base`, `supply_t`, `fuel_t`; `risk` has `intent` and `risk`
//! (`preserve`, `balanced` or `press`).
//!
//! What a staff handed in for a day is `{"orders", "commands", "no_orders"}`
//! and, left out when there is none, `"notices"` (each `{kind, plan, run,
//! rests_on}`: the kind's name, the plan, the run or `null`, and the reason
//! it rests on as `{"command": i}` or `{"no_order": i}`):
//! an order is its `unit`, its `order` body (as [`crate::order_to_json`]
//! writes it), the `intent` it serves and its `reason`; a reason is `serves`
//! (`intent`, `plan`, `amendment`, each left out when none; `statement`; and
//! `chain`, left out when empty, a list of `{intent, maker, statement}`, the
//! intents above it nearest first, `maker` being `commander` or `staff`), `evidence` (a
//! list of `{what, day}`), `binding`, `worked` and `would_change`.

use std::collections::BTreeSet;

use serde_json::{json, Map, Value};
use sim_core::command::{
    ChainLink, CommandKind, Evidence, ForcesLimit, HeldSetting, HoldTarget, IntentKind,
    InterruptCondition, InterruptRef, InterruptSetting, Leave, Limit, Maker, NoticeBasis,
    NoticeKind, PlanId, PlanOutcome, Reason, Risk, RunDays, RunSpec, Serves, StaffCommand,
    StaffDecisions, StaffNotice, StaffOrder, StandingKind, StandingSubject,
};
use sim_core::fixed::Fixed;
use sim_core::hex::Hex;
use sim_core::ids::{AirGroupId, BaseId, IntentId, InterruptId, ShipId, TaskForceId};

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

/// A list of whole numbers that may be left out or `null`: then it is empty.
fn numbers_or_none(value: &Value, key: &str) -> Result<Vec<u32>, String> {
    match value.get(key) {
        None | Some(Value::Null) => Ok(Vec::new()),
        Some(_) => numbers(value, key),
    }
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
        HoldTarget::AirGroup(group) => body["air_group"] = json!(group.raw()),
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
        ["air_group"] => {
            let group = AirGroupId::new(small(value, "air_group")?);
            match value.get("setting") {
                None | Some(Value::Null) => Ok(HoldTarget::AirGroup(group)),
                Some(_) => Ok(HoldTarget::Setting {
                    group,
                    setting: setting_from(text(value, "setting")?)?,
                }),
            }
        }
        [] => Err("a hold names one of \"ship\", \"task_force\" or \"air_group\"".to_string()),
        _ => Err("a hold names only one of \"ship\", \"task_force\" and \"air_group\"".to_string()),
    }
}

fn standing_from(name: &str) -> Result<StandingKind, String> {
    StandingKind::from_name(name).ok_or_else(|| format!("unknown standing interrupt \"{name}\""))
}

/// The interrupt a command names, as the keys of its object: `interrupt`, or
/// `standing` with one of `plan` and `ship`.
fn interrupt_ref_to_json(body: &mut Value, interrupt: InterruptRef) {
    match interrupt {
        InterruptRef::Set(id) => body["interrupt"] = json!(id.raw()),
        InterruptRef::Standing { kind, subject } => {
            body["standing"] = json!(kind.name());
            match subject {
                StandingSubject::Plan(plan) => body["plan"] = json!(plan.raw()),
                StandingSubject::Ship(ship) => body["ship"] = json!(ship.raw()),
            }
        }
    }
}

fn interrupt_ref_from_json(value: &Value) -> Result<InterruptRef, String> {
    match (value.get("interrupt"), value.get("standing")) {
        (Some(_), None) => Ok(InterruptRef::Set(InterruptId::new(small(
            value,
            "interrupt",
        )?))),
        (None, Some(_)) => {
            let kind = standing_from(text(value, "standing")?)?;
            let subject = match (value.get("plan"), value.get("ship")) {
                (Some(_), None) => StandingSubject::Plan(PlanId::new(small(value, "plan")?)),
                (None, Some(_)) => StandingSubject::Ship(ShipId::new(small(value, "ship")?)),
                _ => {
                    return Err(
                        "a standing interrupt names exactly one of \"plan\" and \"ship\""
                            .to_string(),
                    )
                }
            };
            Ok(InterruptRef::Standing { kind, subject })
        }
        _ => Err("an interrupt names exactly one of \"interrupt\" and \"standing\"".to_string()),
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
        Limit::Forces { intent, limit } => {
            let mut body = json!({
                "kind": "set_limit",
                "limit": "forces",
                "intent": intent.raw(),
                "ships": raws(&limit.ships),
                "task_forces": limit.task_forces.iter().map(|f| f.raw()).collect::<Vec<_>>(),
            });
            // Left out when empty or not set, so a limit with neither is
            // written as it always was.
            if !limit.air_groups.is_empty() {
                body["air_groups"] =
                    json!(limit.air_groups.iter().map(|g| g.raw()).collect::<Vec<_>>());
            }
            if let Some(source) = limit.source {
                body["source"] = json!(source.raw());
            }
            body
        }
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
                ships: numbers_or_none(value, "ships")?
                    .into_iter()
                    .map(ShipId::new)
                    .collect(),
                task_forces: numbers_or_none(value, "task_forces")?
                    .into_iter()
                    .map(TaskForceId::new)
                    .collect(),
                air_groups: numbers_or_none(value, "air_groups")?
                    .into_iter()
                    .map(AirGroupId::new)
                    .collect(),
                source: small_opt(value, "source")?.map(BaseId::new),
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

/// An intent's kind as the keys of a command's object: `intent_kind` (left
/// out for keeping a base supplied), `base`, and for that kind `floor_days`
/// and `interval_days` (left out when not set).
fn intent_kind_to_json(body: &mut Value, kind: IntentKind) {
    match kind {
        IntentKind::KeepSupplied {
            base,
            floor_days,
            interval_days,
        } => {
            body["base"] = json!(base.raw());
            body["floor_days"] = json!(floor_days);
            if let Some(days) = interval_days {
                body["interval_days"] = json!(days);
            }
        }
        IntentKind::Cover { base } | IntentKind::Hold { base } => {
            body["intent_kind"] = json!(kind.name());
            body["base"] = json!(base.raw());
        }
    }
}

/// An intent's kind from the keys of a command's object; no `intent_kind` is
/// keeping a base supplied.
fn intent_kind_from_json(value: &Value) -> Result<IntentKind, String> {
    let base = BaseId::new(small(value, "base")?);
    match value.get("intent_kind") {
        None | Some(Value::Null) => keep_supplied_from_json(value, base),
        Some(_) => match text(value, "intent_kind")? {
            "keep_supplied" => keep_supplied_from_json(value, base),
            "cover" => Ok(IntentKind::Cover { base }),
            "hold" => Ok(IntentKind::Hold { base }),
            other => Err(format!(
                "unknown intent kind \"{other}\" (keep_supplied, cover or hold)"
            )),
        },
    }
}

fn keep_supplied_from_json(value: &Value, base: BaseId) -> Result<IntentKind, String> {
    Ok(IntentKind::KeepSupplied {
        base,
        floor_days: small(value, "floor_days")?,
        interval_days: small_opt(value, "interval_days")?,
    })
}

/// A command as JSON.
pub fn command_to_json(command: &CommandKind) -> Value {
    match command {
        CommandKind::SetIntent { intent, kind } => {
            let mut body = json!({ "kind": "set_intent", "intent": intent.raw() });
            intent_kind_to_json(&mut body, *kind);
            body
        }
        CommandKind::FormIntent { parent, kind } => {
            let mut body = json!({ "kind": "form_intent", "parent": parent.raw() });
            // The kind's name is always written here: a formed intent has no
            // default kind.
            body["intent_kind"] = json!(kind.name());
            body["base"] = json!(kind.base().raw());
            body
        }
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
        CommandKind::SetInterrupt(setting) => match setting {
            InterruptSetting::Set {
                interrupt,
                condition,
            } => {
                let InterruptCondition::SupplyBelow { base, days } = condition;
                json!({
                    "kind": "set_interrupt",
                    "interrupt": interrupt.raw(),
                    "base": base.raw(),
                    "below_days": days,
                })
            }
            InterruptSetting::Standing { kind, on } => json!({
                "kind": "set_interrupt",
                "standing": kind.name(),
                "on": on,
            }),
        },
        CommandKind::Acknowledge { interrupt } => {
            let mut body = json!({ "kind": "acknowledge" });
            interrupt_ref_to_json(&mut body, *interrupt);
            body
        }
    }
}

/// Reads a command from its JSON.
pub fn command_from_json(value: &Value) -> Result<CommandKind, String> {
    let plan = || small(value, "plan").map(PlanId::new);
    match text(value, "kind")? {
        "set_intent" => Ok(CommandKind::SetIntent {
            intent: IntentId::new(small(value, "intent")?),
            kind: intent_kind_from_json(value)?,
        }),
        "form_intent" => {
            let kind = intent_kind_from_json(value)?;
            if matches!(kind, IntentKind::KeepSupplied { .. }) {
                return Err("form_intent needs \"intent_kind\" \"cover\" or \"hold\"".to_string());
            }
            Ok(CommandKind::FormIntent {
                parent: IntentId::new(small(value, "parent")?),
                kind,
            })
        }
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
        "set_interrupt" => match (value.get("interrupt"), value.get("standing")) {
            (Some(_), None) => Ok(CommandKind::SetInterrupt(InterruptSetting::Set {
                interrupt: InterruptId::new(small(value, "interrupt")?),
                condition: InterruptCondition::SupplyBelow {
                    base: BaseId::new(small(value, "base")?),
                    days: small(value, "below_days")?,
                },
            })),
            (None, Some(_)) => Ok(CommandKind::SetInterrupt(InterruptSetting::Standing {
                kind: standing_from(text(value, "standing")?)?,
                on: flag(value, "on")?,
            })),
            _ => {
                Err("set_interrupt names exactly one of \"interrupt\" and \"standing\"".to_string())
            }
        },
        "acknowledge" => Ok(CommandKind::Acknowledge {
            interrupt: interrupt_ref_from_json(value)?,
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
    if !reason.serves.statement.is_empty() {
        serves.insert("statement".into(), json!(reason.serves.statement));
    }
    if !reason.serves.chain.is_empty() {
        serves.insert(
            "chain".into(),
            Value::Array(
                reason
                    .serves
                    .chain
                    .iter()
                    .map(|link| {
                        json!({
                            "intent": link.intent.raw(),
                            "maker": link.maker.name(),
                            "statement": link.statement,
                        })
                    })
                    .collect(),
            ),
        );
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
            // A game saved before the statement was handed in has none.
            statement: match serves.get("statement") {
                None | Some(Value::Null) => String::new(),
                Some(_) => text(serves, "statement")?.to_string(),
            },
            // A game saved before the tree has no chain.
            chain: match serves.get("chain") {
                None | Some(Value::Null) => Vec::new(),
                Some(_) => list(serves, "chain")?
                    .iter()
                    .map(|link| {
                        Ok(ChainLink {
                            intent: IntentId::new(small(link, "intent")?),
                            maker: match text(link, "maker")? {
                                "commander" => Maker::Commander,
                                "staff" => Maker::Staff,
                                other => return Err(format!("unknown maker \"{other}\"")),
                            },
                            statement: text(link, "statement")?.to_string(),
                        })
                    })
                    .collect::<Result<_, String>>()?,
            },
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
    let mut body = json!({
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
    });
    // Left out when there is none, so that a hand-in without notices is
    // written as it always was.
    if !decisions.notices.is_empty() {
        body["notices"] = Value::Array(decisions.notices.iter().map(notice_to_json).collect());
    }
    body
}

fn notice_to_json(notice: &StaffNotice) -> Value {
    let rests_on = match notice.rests_on {
        NoticeBasis::Command(i) => json!({ "command": i }),
        NoticeBasis::NoOrder(i) => json!({ "no_order": i }),
    };
    json!({
        "kind": notice.kind.name(),
        "plan": notice.plan.raw(),
        "run": notice.run,
        "rests_on": rests_on,
    })
}

fn notice_from_json(value: &Value) -> Result<StaffNotice, String> {
    let name = text(value, "kind")?;
    let kind = NoticeKind::from_name(name).ok_or_else(|| format!("unknown notice \"{name}\""))?;
    let rests_on = value
        .get("rests_on")
        .ok_or_else(|| "a notice has no \"rests_on\"".to_string())?;
    let index = |key: &str| -> Result<usize, String> {
        usize::try_from(small(rests_on, key)?).map_err(|_| format!("\"{key}\" is too large"))
    };
    let rests_on = match (rests_on.get("command"), rests_on.get("no_order")) {
        (Some(_), None) => NoticeBasis::Command(index("command")?),
        (None, Some(_)) => NoticeBasis::NoOrder(index("no_order")?),
        _ => {
            return Err("a notice rests on exactly one of \"command\" and \"no_order\"".to_string())
        }
    };
    Ok(StaffNotice {
        kind,
        plan: PlanId::new(small(value, "plan")?),
        run: small_opt(value, "run")?,
        rests_on,
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
    // A hand-in saved before notices has no key: it has none.
    if value.get("notices").is_some() {
        for entry in list(value, "notices")? {
            decisions.notices.push(notice_from_json(entry)?);
        }
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
            CommandKind::SetIntent {
                intent,
                kind: IntentKind::Cover {
                    base: BaseId::new(5),
                },
            },
            CommandKind::SetIntent {
                intent,
                kind: IntentKind::Hold {
                    base: BaseId::new(5),
                },
            },
            CommandKind::FormIntent {
                parent: IntentId::new(1),
                kind: IntentKind::Cover {
                    base: BaseId::new(5),
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
                    air_groups: BTreeSet::new(),
                    source: Some(BaseId::new(2)),
                },
            }),
            CommandKind::SetLimit(Limit::Forces {
                intent,
                limit: ForcesLimit {
                    ships: BTreeSet::new(),
                    task_forces: BTreeSet::new(),
                    air_groups: [AirGroupId::new(8), AirGroupId::new(2)]
                        .into_iter()
                        .collect(),
                    source: None,
                },
            }),
            CommandKind::SetLimit(Limit::Forces {
                intent,
                limit: ForcesLimit {
                    ships: ships(&[4]),
                    task_forces: BTreeSet::new(),
                    air_groups: [AirGroupId::new(2)].into_iter().collect(),
                    source: Some(BaseId::new(1)),
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
            CommandKind::Hold {
                target: HoldTarget::AirGroup(AirGroupId::new(7)),
            },
            CommandKind::Release {
                target: HoldTarget::AirGroup(AirGroupId::new(7)),
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
            CommandKind::SetInterrupt(InterruptSetting::Set {
                interrupt: InterruptId::new(3),
                condition: InterruptCondition::SupplyBelow {
                    base: BaseId::new(2),
                    days: 35,
                },
            }),
            CommandKind::SetInterrupt(InterruptSetting::Standing {
                kind: StandingKind::CannotMeet,
                on: false,
            }),
            CommandKind::SetInterrupt(InterruptSetting::Standing {
                kind: StandingKind::ShipLost,
                on: true,
            }),
            CommandKind::Acknowledge {
                interrupt: InterruptRef::Set(InterruptId::new(3)),
            },
            CommandKind::Acknowledge {
                interrupt: InterruptRef::Standing {
                    kind: StandingKind::WaitsForLeave,
                    subject: StandingSubject::Plan(PlanId::new(1)),
                },
            },
            CommandKind::Acknowledge {
                interrupt: InterruptRef::Standing {
                    kind: StandingKind::ShipLost,
                    subject: StandingSubject::Ship(ShipId::new(4)),
                },
            },
        ]
    }

    #[test]
    fn an_interrupt_command_is_written_with_its_keys_and_a_bad_one_is_refused() {
        assert_eq!(
            command_to_json(&CommandKind::SetInterrupt(InterruptSetting::Set {
                interrupt: InterruptId::new(1),
                condition: InterruptCondition::SupplyBelow {
                    base: BaseId::new(2),
                    days: 35
                }
            })),
            json!({ "kind": "set_interrupt", "interrupt": 1, "base": 2, "below_days": 35 })
        );
        assert_eq!(
            command_to_json(&CommandKind::SetInterrupt(InterruptSetting::Standing {
                kind: StandingKind::WaitsForLeave,
                on: false
            })),
            json!({ "kind": "set_interrupt", "standing": "waits_for_leave", "on": false })
        );
        assert_eq!(
            command_to_json(&CommandKind::Acknowledge {
                interrupt: InterruptRef::Standing {
                    kind: StandingKind::CannotMeet,
                    subject: StandingSubject::Plan(PlanId::new(2))
                }
            }),
            json!({ "kind": "acknowledge", "standing": "cannot_meet", "plan": 2 })
        );
        assert_eq!(
            command_to_json(&CommandKind::Acknowledge {
                interrupt: InterruptRef::Set(InterruptId::new(7))
            }),
            json!({ "kind": "acknowledge", "interrupt": 7 })
        );
        for bad in [
            json!({ "kind": "set_interrupt" }),
            json!({ "kind": "set_interrupt", "interrupt": 1, "base": 2 }),
            json!({ "kind": "set_interrupt", "interrupt": 1, "standing": "ship_lost", "on": true }),
            json!({ "kind": "set_interrupt", "standing": "cannot_swim", "on": true }),
            json!({ "kind": "set_interrupt", "standing": "ship_lost" }),
            json!({ "kind": "set_interrupt", "interrupt": 1, "base": 2, "below_days": 2.5 }),
            json!({ "kind": "acknowledge" }),
            json!({ "kind": "acknowledge", "standing": "ship_lost" }),
            json!({ "kind": "acknowledge", "standing": "ship_lost", "plan": 1, "ship": 2 }),
            json!({ "kind": "acknowledge", "interrupt": 1, "standing": "ship_lost", "ship": 2 }),
        ] {
            assert!(command_from_json(&bad).is_err(), "{bad}");
        }
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
            hold(HoldTarget::AirGroup(AirGroupId::new(3))),
            json!({ "kind": "hold", "air_group": 3 })
        );
        assert_eq!(
            command_from_json(&json!({ "kind": "release", "air_group": 3 })),
            Ok(CommandKind::Release {
                target: HoldTarget::AirGroup(AirGroupId::new(3))
            })
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
            json!({ "kind": "release", "air_group": 3, "setting": "mission" }),
            json!({ "kind": "hold", "ship": -1 }),
            json!({ "kind": "end_run", "plan": 1 }),
        ] {
            assert!(command_from_json(&bad).is_err(), "{bad}");
        }
    }

    #[test]
    fn the_intent_kinds_and_the_forces_limit_are_written_as_the_scenario_keys_are() {
        let intent = IntentId::new(2);
        // Keeping supplied has no `intent_kind` key, as before the kinds.
        assert_eq!(
            command_to_json(&CommandKind::SetIntent {
                intent,
                kind: IntentKind::KeepSupplied {
                    base: BaseId::new(5),
                    floor_days: 12,
                    interval_days: None
                }
            }),
            json!({ "kind": "set_intent", "intent": 2, "base": 5, "floor_days": 12 })
        );
        assert_eq!(
            command_to_json(&CommandKind::SetIntent {
                intent,
                kind: IntentKind::Hold {
                    base: BaseId::new(5)
                }
            }),
            json!({ "kind": "set_intent", "intent": 2, "intent_kind": "hold", "base": 5 })
        );
        assert_eq!(
            command_to_json(&CommandKind::FormIntent {
                parent: IntentId::new(1),
                kind: IntentKind::Cover {
                    base: BaseId::new(5)
                }
            }),
            json!({ "kind": "form_intent", "parent": 1, "intent_kind": "cover", "base": 5 })
        );
        // A key's absence reads as keep_supplied; an unknown kind is refused;
        // a formed intent has no default kind.
        assert_eq!(
            command_from_json(
                &json!({ "kind": "set_intent", "intent": 1, "base": 2, "floor_days": 9 })
            ),
            Ok(CommandKind::SetIntent {
                intent: IntentId::new(1),
                kind: IntentKind::KeepSupplied {
                    base: BaseId::new(2),
                    floor_days: 9,
                    interval_days: None
                }
            })
        );
        for bad in [
            json!({ "kind": "set_intent", "intent": 1, "intent_kind": "win", "base": 2 }),
            json!({ "kind": "set_intent", "intent": 1, "intent_kind": "keep_supplied", "base": 2 }),
            json!({ "kind": "form_intent", "parent": 1, "base": 2 }),
            json!({ "kind": "form_intent", "parent": 1, "intent_kind": "keep_supplied", "base": 2, "floor_days": 3 }),
            json!({ "kind": "form_intent", "intent_kind": "cover", "base": 2 }),
        ] {
            assert!(command_from_json(&bad).is_err(), "{bad}");
        }
        // A forces limit written before air groups, with ships, task forces
        // and a source, reads as before; each key is optional now.
        assert_eq!(
            command_from_json(&json!({
                "kind": "set_limit", "limit": "forces", "intent": 1,
                "ships": [4], "task_forces": [], "source": 2
            })),
            Ok(CommandKind::SetLimit(Limit::Forces {
                intent: IntentId::new(1),
                limit: ForcesLimit {
                    ships: ships(&[4]),
                    task_forces: BTreeSet::new(),
                    air_groups: BTreeSet::new(),
                    source: Some(BaseId::new(2)),
                }
            }))
        );
        let only_groups = command_to_json(&CommandKind::SetLimit(Limit::Forces {
            intent,
            limit: ForcesLimit {
                ships: BTreeSet::new(),
                task_forces: BTreeSet::new(),
                air_groups: [AirGroupId::new(3), AirGroupId::new(1)]
                    .into_iter()
                    .collect(),
                source: None,
            },
        }));
        assert_eq!(
            only_groups,
            json!({
                "kind": "set_limit", "limit": "forces", "intent": 2,
                "ships": [], "task_forces": [], "air_groups": [1, 3]
            })
        );
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
                statement:
                    "keep base:2 (Placeholder base T) at 30 days of supply, a run every 15 days"
                        .to_string(),
                chain: vec![
                    ChainLink {
                        intent: IntentId::new(2),
                        maker: Maker::Staff,
                        statement: "cover base:1 from the air".to_string(),
                    },
                    ChainLink {
                        intent: IntentId::new(1),
                        maker: Maker::Commander,
                        statement: "hold base:1".to_string(),
                    },
                ],
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
            notices: vec![
                StaffNotice {
                    kind: NoticeKind::CannotMeet,
                    plan: PlanId::new(1),
                    run: None,
                    rests_on: NoticeBasis::Command(0),
                },
                StaffNotice {
                    kind: NoticeKind::WaitsForLeave(Leave::Prepare),
                    plan: PlanId::new(2),
                    run: None,
                    rests_on: NoticeBasis::NoOrder(1),
                },
                StaffNotice {
                    kind: NoticeKind::WaitsForLeave(Leave::Launch),
                    plan: PlanId::new(2),
                    run: None,
                    rests_on: NoticeBasis::NoOrder(0),
                },
                StaffNotice {
                    kind: NoticeKind::RunEnded,
                    plan: PlanId::new(1),
                    run: Some(3),
                    rests_on: NoticeBasis::NoOrder(0),
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
        assert_eq!(value["notices"][3]["run"], 3);
        assert_eq!(value["notices"][0]["run"], Value::Null);
        assert_eq!(value["notices"][1]["rests_on"], json!({ "no_order": 1 }));
        assert_eq!(value["notices"][0]["rests_on"], json!({ "command": 0 }));
        assert_eq!(value["notices"][1]["kind"], "waits_for_leave_to_prepare");
        // Nothing handed in is written as three empty lists and reads back
        // as nothing.
        let none = staff_decisions_to_json(&StaffDecisions::default());
        assert_eq!(
            staff_decisions_from_json(&none),
            Ok(StaffDecisions::default())
        );
        // With no notice the key is left out, so a hand-in is written as it
        // was before notices; and a hand-in saved before them, with no key,
        // reads as none.
        assert!(none.get("notices").is_none());
        // A reason with no chain writes none and reads as none; so does one
        // saved before the tree.
        assert!(reason_to_json(&Reason::default())["serves"]
            .get("chain")
            .is_none());
        assert_eq!(
            reason_from_json(&json!({
                "serves": {}, "evidence": [], "binding": [], "worked": "", "would_change": []
            }))
            .map(|r| r.serves.chain),
            Ok(Vec::new())
        );
        let chain = &value["orders"][0]["reason"]["serves"]["chain"];
        assert_eq!(
            chain[1],
            json!({ "intent": 1, "maker": "commander", "statement": "hold base:1" })
        );
        let old = json!({ "orders": [], "commands": [], "no_orders": [] });
        assert_eq!(
            staff_decisions_from_json(&old),
            Ok(StaffDecisions::default())
        );
    }

    #[test]
    fn a_bad_notice_is_refused() {
        let hand_in = |notice: Value| {
            staff_decisions_from_json(&json!({
                "orders": [], "commands": [], "no_orders": [], "notices": [notice]
            }))
        };
        let good = json!({
            "kind": "cannot_meet", "plan": 1, "run": null, "rests_on": { "no_order": 0 }
        });
        assert!(hand_in(good.clone()).is_ok());
        for bad in [
            json!({ "kind": "cannot_swim", "plan": 1, "run": null, "rests_on": { "no_order": 0 } }),
            json!({ "kind": "cannot_meet", "plan": 1, "run": null }),
            json!({ "kind": "cannot_meet", "plan": 1, "run": null, "rests_on": {} }),
            json!({ "kind": "cannot_meet", "plan": 1, "run": null, "rests_on": { "command": 0, "no_order": 0 } }),
            json!({ "kind": "cannot_meet", "run": null, "rests_on": { "no_order": 0 } }),
            json!({ "kind": "cannot_meet", "plan": 1, "run": null, "rests_on": { "no_order": -1 } }),
            json!({ "kind": "cannot_meet", "plan": 1, "run": 1.5, "rests_on": { "no_order": 0 } }),
        ] {
            assert!(hand_in(bad.clone()).is_err(), "{bad}");
        }
    }
}
