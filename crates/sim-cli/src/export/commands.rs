//! Each side's command state as JSON (decision 0107, "As built"): its intents
//! with their limits, its two leaves, its plans with their baselines,
//! amendments and runs, and the commitments of its ships. It is the umpire's
//! record, as everything in a replay file is.
//!
//! It is written only for a side whose command state is not empty
//! ([`CommandState::is_empty`]), so the replay of a run in which no command
//! was ever given is byte for byte what it was before commands. A weight is
//! a JSON number written from its millionths ([`Num`]); ids, days and counts
//! are integers; a set is written in order.

use std::collections::BTreeSet;

use serde::Serialize;
use serde_json::value::RawValue;
use sim_core::command::{
    Amendment, AmendmentChange, Baseline, CommandState, Commitment, Intent, IntentKind,
    IntentLimits, Limit, Plan, Run, RunDays,
};
use sim_core::ids::ShipId;
use sim_core::state::Side;
use sim_core::World;

use super::num::Num;

/// What an intent asks for: `{ "kind": "keep_supplied", base, floor_days,
/// interval_days }`, the interval left out where the commander has not set it.
#[derive(Serialize)]
struct IntentKindOut {
    kind: &'static str,
    base: u32,
    floor_days: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    interval_days: Option<u32>,
}

fn intent_kind(kind: &IntentKind) -> IntentKindOut {
    match kind {
        IntentKind::KeepSupplied {
            base,
            floor_days,
            interval_days,
        } => IntentKindOut {
            kind: "keep_supplied",
            base: base.raw(),
            floor_days: *floor_days,
            interval_days: *interval_days,
        },
    }
}

#[derive(Serialize)]
struct ForcesOut {
    ships: Vec<u32>,
    task_forces: Vec<u32>,
    source: u32,
}

/// An intent's limits as they stand. A limit not set is left out; the
/// protected ships are left out when there are none.
#[derive(Serialize)]
struct LimitsOut {
    #[serde(skip_serializing_if = "Option::is_none")]
    forces: Option<ForcesOut>,
    #[serde(skip_serializing_if = "Option::is_none")]
    area: Option<Vec<[i32; 2]>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    priority: Option<u32>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    protected: Vec<u32>,
    risk: &'static str,
}

fn ships(set: &BTreeSet<ShipId>) -> Vec<u32> {
    set.iter().map(|ship| ship.raw()).collect()
}

fn limits(limits: &IntentLimits) -> LimitsOut {
    LimitsOut {
        forces: limits.forces.as_ref().map(|forces| ForcesOut {
            ships: ships(&forces.ships),
            task_forces: forces.task_forces.iter().map(|f| f.raw()).collect(),
            source: forces.source.raw(),
        }),
        area: limits
            .area
            .as_ref()
            .map(|hexes| hexes.iter().map(|h| [h.q, h.r]).collect()),
        priority: limits.priority,
        protected: ships(&limits.protected),
        risk: limits.risk.name(),
    }
}

#[derive(Serialize)]
struct IntentOut {
    id: u32,
    #[serde(flatten)]
    kind: IntentKindOut,
    set_day: u32,
    plan: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    withdrawn_day: Option<u32>,
    limits: LimitsOut,
}

fn intent(intent: &Intent) -> IntentOut {
    IntentOut {
        id: intent.id.raw(),
        kind: intent_kind(&intent.kind),
        set_day: intent.set_day,
        plan: intent.plan.raw(),
        withdrawn_day: intent.withdrawn_day,
        limits: limits(&intent.limits),
    }
}

#[derive(Serialize)]
struct BaselineOut {
    day: u32,
    #[serde(flatten)]
    kind: IntentKindOut,
    limits: LimitsOut,
}

fn baseline(baseline: &Baseline) -> BaselineOut {
    BaselineOut {
        day: baseline.day,
        kind: intent_kind(&baseline.kind),
        limits: limits(&baseline.limits),
    }
}

/// One limit as an amendment records it: `limit` names which, and the keys
/// are those of the command that set it.
#[derive(Serialize)]
struct LimitOut {
    limit: &'static str,
    #[serde(skip_serializing_if = "Option::is_none")]
    intent: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    base: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    forces: Option<ForcesOut>,
    #[serde(skip_serializing_if = "Option::is_none")]
    hexes: Option<Vec<[i32; 2]>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rank: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ships: Option<Vec<u32>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    supply_t: Option<Num>,
    #[serde(skip_serializing_if = "Option::is_none")]
    fuel_t: Option<Num>,
    #[serde(skip_serializing_if = "Option::is_none")]
    risk: Option<&'static str>,
}

fn limit(limit: &Limit) -> LimitOut {
    let blank = |name: &'static str| LimitOut {
        limit: name,
        intent: None,
        base: None,
        forces: None,
        hexes: None,
        rank: None,
        ships: None,
        supply_t: None,
        fuel_t: None,
        risk: None,
    };
    match limit {
        Limit::Forces { intent, limit } => LimitOut {
            intent: Some(intent.raw()),
            forces: Some(ForcesOut {
                ships: ships(&limit.ships),
                task_forces: limit.task_forces.iter().map(|f| f.raw()).collect(),
                source: limit.source.raw(),
            }),
            ..blank("forces")
        },
        Limit::Area { intent, hexes } => LimitOut {
            intent: Some(intent.raw()),
            hexes: Some(hexes.iter().map(|h| [h.q, h.r]).collect()),
            ..blank("area")
        },
        Limit::Priority { intent, rank } => LimitOut {
            intent: Some(intent.raw()),
            rank: *rank,
            ..blank("priority")
        },
        Limit::Protected { intent, ships: set } => LimitOut {
            intent: Some(intent.raw()),
            ships: Some(ships(set)),
            ..blank("protected")
        },
        Limit::Reserve {
            base,
            supply_t,
            fuel_t,
        } => LimitOut {
            base: Some(base.raw()),
            supply_t: Some(Num(*supply_t)),
            fuel_t: Some(Num(*fuel_t)),
            ..blank("reserve")
        },
        Limit::Risk { intent, risk } => LimitOut {
            intent: Some(intent.raw()),
            risk: Some(risk.name()),
            ..blank("risk")
        },
    }
}

/// What an amendment changed: `{ "intent": {...} }` or `{ "limit": {...} }`.
#[derive(Serialize)]
struct ChangeOut {
    #[serde(skip_serializing_if = "Option::is_none")]
    intent: Option<IntentKindOut>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<LimitOut>,
}

#[derive(Serialize)]
struct AmendmentOut {
    number: u32,
    day: u32,
    change: ChangeOut,
}

fn amendment(amendment: &Amendment) -> AmendmentOut {
    AmendmentOut {
        number: amendment.number,
        day: amendment.day,
        change: match &amendment.change {
            AmendmentChange::Intent(kind) => ChangeOut {
                intent: Some(intent_kind(kind)),
                limit: None,
            },
            AmendmentChange::Limit(changed) => ChangeOut {
                intent: None,
                limit: Some(limit(changed)),
            },
        },
    }
}

#[derive(Serialize)]
struct DaysOut {
    load: u32,
    sail: u32,
    arrive: u32,
    free: u32,
}

fn days(days: &RunDays) -> DaysOut {
    DaysOut {
        load: days.load,
        sail: days.sail,
        arrive: days.arrive,
        free: days.free,
    }
}

#[derive(Serialize)]
struct RunOut {
    number: u32,
    ships: Vec<u32>,
    load_t: Num,
    source: u32,
    days: DaysOut,
}

fn run(run: &Run) -> RunOut {
    RunOut {
        number: run.number,
        ships: ships(&run.ships),
        load_t: Num(run.load_t),
        source: run.source.raw(),
        days: days(&run.days),
    }
}

#[derive(Serialize)]
struct PlanOut {
    id: u32,
    intent: u32,
    state: &'static str,
    #[serde(skip_serializing_if = "Option::is_none")]
    baseline: Option<BaselineOut>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    amendments: Vec<AmendmentOut>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    runs: Vec<RunOut>,
}

fn plan(plan: &Plan) -> PlanOut {
    PlanOut {
        id: plan.id.raw(),
        intent: plan.intent.raw(),
        state: plan.state.name(),
        baseline: plan.baseline.as_ref().map(baseline),
        amendments: plan.amendments.iter().map(amendment).collect(),
        runs: plan.runs.values().map(run).collect(),
    }
}

#[derive(Serialize)]
struct CommitmentOut {
    ship: u32,
    plan: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    run: Option<u32>,
    from_day: u32,
    to_day: u32,
}

fn commitment(ship: ShipId, commitment: &Commitment) -> CommitmentOut {
    CommitmentOut {
        ship: ship.raw(),
        plan: commitment.plan.raw(),
        run: commitment.run,
        from_day: commitment.from_day,
        to_day: commitment.to_day,
    }
}

#[derive(Serialize)]
struct LeavesOut {
    prepare: bool,
    launch: bool,
}

/// One side's command state.
#[derive(Serialize)]
pub struct CommandStateOut {
    intents: Vec<IntentOut>,
    leaves: LeavesOut,
    plans: Vec<PlanOut>,
    commitments: Vec<CommitmentOut>,
    next_plan: u32,
}

fn state(state: &CommandState) -> CommandStateOut {
    CommandStateOut {
        intents: state.intents.values().map(intent).collect(),
        leaves: LeavesOut {
            prepare: state.leaves.prepare,
            launch: state.leaves.launch,
        },
        plans: state.plans.values().map(plan).collect(),
        commitments: state
            .commitments
            .iter()
            .flat_map(|(ship, list)| list.iter().map(move |c| commitment(*ship, c)))
            .collect(),
        next_plan: state.next_plan,
    }
}

/// Each side's command state, for the side that has one that is not empty.
#[derive(Serialize, Default)]
pub struct CommandsOut {
    #[serde(skip_serializing_if = "Option::is_none")]
    allied: Option<CommandStateOut>,
    #[serde(skip_serializing_if = "Option::is_none")]
    japanese: Option<CommandStateOut>,
}

impl CommandsOut {
    pub fn is_empty(&self) -> bool {
        self.allied.is_none() && self.japanese.is_none()
    }
}

/// The command state of each side of `world` that has any.
pub fn commands(world: &World) -> CommandsOut {
    let of = |side: Side| {
        let held = world.command_state(side);
        (!held.is_empty()).then(|| state(held))
    };
    CommandsOut {
        allied: of(Side::Allied),
        japanese: of(Side::Japanese),
    }
}

/// Each side's command state as JSON text, for the side that has one that is
/// not empty: what a step compares with the step before.
pub fn texts(world: &World) -> [Option<String>; 2] {
    let of = |side: Side| {
        let held = world.command_state(side);
        (!held.is_empty())
            .then(|| serde_json::to_string(&state(held)).expect("a command state serialises"))
    };
    [of(Side::Allied), of(Side::Japanese)]
}

/// A step's command states: each side's whole state, for the side whose state
/// differs from the step before.
#[derive(Serialize)]
pub struct StepCommands {
    #[serde(skip_serializing_if = "Option::is_none")]
    allied: Option<Box<RawValue>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    japanese: Option<Box<RawValue>>,
}

impl StepCommands {
    /// What differs between `last` (the step before) and `now`; `None` where
    /// nothing does, so a step of a run with no commands has no key.
    pub fn diff(last: &mut [Option<String>; 2], now: [Option<String>; 2]) -> Option<StepCommands> {
        let mut changed: [Option<Box<RawValue>>; 2] = [None, None];
        for i in 0..2 {
            if now[i].is_some() && now[i] != last[i] {
                changed[i] = now[i]
                    .clone()
                    .map(|text| RawValue::from_string(text).expect("a state is JSON"));
            }
        }
        *last = now;
        let [allied, japanese] = changed;
        (allied.is_some() || japanese.is_some()).then_some(StepCommands { allied, japanese })
    }
}
