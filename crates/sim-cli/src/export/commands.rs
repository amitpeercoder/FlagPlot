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
    Amendment, AmendmentChange, Awaited, Baseline, CommandState, Commitment, Intent, IntentKind,
    IntentLimits, Knowledge, Limit, Maker, Plan, Run, RunDays, TargetsLimit,
};
use sim_core::ids::ShipId;
use sim_core::state::Side;
use sim_core::World;

use super::num::Num;

/// What an intent asks for: `{ "kind": "keep_supplied", base, floor_days,
/// interval_days }`, the interval left out where the commander has not set it;
/// or `{ "kind": "cover" | "hold", base }`, with no days; or
/// `{ "kind": "neutralise", target }` (no `base`), or
/// `{ "kind": "raid", base, target }` (decision 0120); or `{ "kind": "deny",
/// hex, distance }` (no `base`), or `{ "kind": "strike_ships" |
/// "search_waters", base }` (decision 0123); or `{ "kind": "stand_at" |
/// "operate_carriers", task_force, hex }` (decisions 0124 and 0125); or `{
/// "kind": "ready_landing", task_force, hex, by_day }` (`by_day` left out
/// where there is none) or `{ "kind": "build_up", base, wants, defences }`
/// (decision 0141); or `{ "kind": "take_place", hex, by_day }` (no `base` or
/// `task_force`; `by_day` left out where there is none, decision 0142); or `{
/// "kind": "bombard", task_force, target }` (decision 0146); or `{ "kind":
/// "control_island", island, by_day }` (`by_day` left out where there is none,
/// decision 0151).
#[derive(Serialize)]
struct IntentKindOut {
    kind: &'static str,
    #[serde(skip_serializing_if = "Option::is_none")]
    base: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    target: Option<u32>,
    /// The centre of the waters of a deny intent, `[q, r]`.
    #[serde(skip_serializing_if = "Option::is_none")]
    hex: Option<[i32; 2]>,
    /// The distance in hexes of the waters of a deny intent.
    #[serde(skip_serializing_if = "Option::is_none")]
    distance: Option<u32>,
    /// The task force of a stand_at or operate_carriers intent (decision
    /// 0124 and 0125), the umpire's number; `hex` is its station.
    #[serde(skip_serializing_if = "Option::is_none")]
    task_force: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    floor_days: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    interval_days: Option<u32>,
    /// The morning a ready_landing intent is to be ready by, or a take_place
    /// intent to have taken its place by (decisions 0141 and 0142).
    #[serde(skip_serializing_if = "Option::is_none")]
    by_day: Option<u32>,
    /// What a build_up intent wants, in order (decision 0141).
    #[serde(skip_serializing_if = "Option::is_none")]
    wants: Option<Vec<&'static str>>,
    /// The points of defences a build_up intent wants, where it wants them.
    #[serde(skip_serializing_if = "Option::is_none")]
    defences: Option<u32>,
    /// The number of the island a control_island intent names (decision
    /// 0151).
    #[serde(skip_serializing_if = "Option::is_none")]
    island: Option<u32>,
}

fn intent_kind(kind: &IntentKind) -> IntentKindOut {
    match kind {
        IntentKind::KeepSupplied {
            base,
            floor_days,
            interval_days,
        } => IntentKindOut {
            kind: "keep_supplied",
            island: None,
            base: Some(base.raw()),
            target: None,
            hex: None,
            distance: None,
            task_force: None,
            floor_days: Some(*floor_days),
            interval_days: *interval_days,
            by_day: None,
            wants: None,
            defences: None,
        },
        IntentKind::Cover { base }
        | IntentKind::Hold { base }
        | IntentKind::StrikeShips { base }
        | IntentKind::SearchWaters { base } => IntentKindOut {
            kind: kind.name(),
            island: None,
            base: Some(base.raw()),
            target: None,
            hex: None,
            distance: None,
            task_force: None,
            floor_days: None,
            interval_days: None,
            by_day: None,
            wants: None,
            defences: None,
        },
        IntentKind::Neutralise { target } | IntentKind::Raid { target, .. } => IntentKindOut {
            kind: kind.name(),
            island: None,
            base: kind.own_base().map(|base| base.raw()),
            target: Some(target.raw()),
            hex: None,
            distance: None,
            task_force: None,
            floor_days: None,
            interval_days: None,
            by_day: None,
            wants: None,
            defences: None,
        },
        IntentKind::StandAt { force, hex } | IntentKind::OperateCarriers { force, hex } => {
            IntentKindOut {
                kind: kind.name(),
                island: None,
                base: None,
                target: None,
                hex: Some([hex.q, hex.r]),
                distance: None,
                task_force: Some(force.raw()),
                floor_days: None,
                interval_days: None,
                by_day: None,
                wants: None,
                defences: None,
            }
        }
        // Decision 0141.
        IntentKind::ReadyLanding { force, hex, by_day } => IntentKindOut {
            kind: kind.name(),
            island: None,
            base: None,
            target: None,
            hex: Some([hex.q, hex.r]),
            distance: None,
            task_force: Some(force.raw()),
            floor_days: None,
            interval_days: None,
            by_day: *by_day,
            wants: None,
            defences: None,
        },
        // Decision 0142: a hex of land, the date left out where there is none.
        IntentKind::TakePlace { hex, by_day } => IntentKindOut {
            kind: kind.name(),
            island: None,
            base: None,
            target: None,
            hex: Some([hex.q, hex.r]),
            distance: None,
            task_force: None,
            floor_days: None,
            interval_days: None,
            by_day: *by_day,
            wants: None,
            defences: None,
        },
        // Decision 0146: the task force and the enemy base it bombards.
        IntentKind::Bombard { force, target } => IntentKindOut {
            kind: kind.name(),
            island: None,
            base: None,
            target: Some(target.raw()),
            hex: None,
            distance: None,
            task_force: Some(force.raw()),
            floor_days: None,
            interval_days: None,
            by_day: None,
            wants: None,
            defences: None,
        },
        IntentKind::BuildUp { base, wants } => IntentKindOut {
            kind: kind.name(),
            island: None,
            base: Some(base.raw()),
            target: None,
            hex: None,
            distance: None,
            task_force: None,
            floor_days: None,
            interval_days: None,
            by_day: None,
            wants: Some(wants.iter().map(|want| want.name()).collect()),
            defences: wants.iter().find_map(|want| match want {
                sim_core::command::Want::Defences(points) => Some(points),
                _ => None,
            }),
        },
        // Decision 0151: the island's number, the date left out where there
        // is none.
        IntentKind::ControlIsland { island, by_day } => IntentKindOut {
            kind: kind.name(),
            island: Some(*island),
            base: None,
            target: None,
            hex: None,
            distance: None,
            task_force: None,
            floor_days: None,
            interval_days: None,
            by_day: *by_day,
            wants: None,
            defences: None,
        },
        IntentKind::Deny { hex, distance } => IntentKindOut {
            kind: "deny",
            island: None,
            base: None,
            target: None,
            hex: Some([hex.q, hex.r]),
            distance: Some(*distance),
            task_force: None,
            floor_days: None,
            interval_days: None,
            by_day: None,
            wants: None,
            defences: None,
        },
    }
}

#[derive(Serialize)]
struct ForcesOut {
    ships: Vec<u32>,
    task_forces: Vec<u32>,
    /// Left out where the limit names none, as it was before air groups.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    air_groups: Vec<u32>,
    /// Left out where the limit names none (decision 0141).
    #[serde(skip_serializing_if = "Vec::is_empty")]
    land_units: Vec<u32>,
    /// Left out where the limit names none (a cover or hold intent's need
    /// not).
    #[serde(skip_serializing_if = "Option::is_none")]
    source: Option<u32>,
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
    /// Left out where the intent sets none: it is read up the tree.
    #[serde(skip_serializing_if = "Option::is_none")]
    risk: Option<&'static str>,
    /// Decision 0159, 9: each left out where the intent sets none.
    #[serde(skip_serializing_if = "Option::is_none")]
    not_before: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    wait_for: Option<AwaitedOut>,
    #[serde(skip_serializing_if = "Option::is_none")]
    after: Option<u32>,
    /// Decision 0159, 12: left out where the intent sets none.
    #[serde(skip_serializing_if = "Option::is_none")]
    knowledge: Option<&'static str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    within_days: Option<u32>,
    /// Decision 0190: left out where the intent sets none.
    #[serde(skip_serializing_if = "Option::is_none")]
    targets: Option<TargetsOut>,
}

/// A "targets" limit: the kinds of ship to strike, most wanted first, and
/// whether the waters of the deny come first (decision 0190).
#[derive(Serialize)]
struct TargetsOut {
    kinds: Vec<&'static str>,
    waters_first: bool,
}

fn targets(targets: &TargetsLimit) -> TargetsOut {
    TargetsOut {
        kinds: targets.kinds.iter().map(|kind| kind.name()).collect(),
        waters_first: targets.waters_first,
    }
}

/// What a "wait for" names: `{ "task_force": n }` or `{ "land_unit": n }`.
#[derive(Serialize)]
struct AwaitedOut {
    #[serde(skip_serializing_if = "Option::is_none")]
    task_force: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    land_unit: Option<u32>,
}

fn awaited(awaited: Awaited) -> AwaitedOut {
    match awaited {
        Awaited::TaskForce(force) => AwaitedOut {
            task_force: Some(force.raw()),
            land_unit: None,
        },
        Awaited::LandUnit(unit) => AwaitedOut {
            task_force: None,
            land_unit: Some(unit.raw()),
        },
    }
}

fn ships(set: &BTreeSet<ShipId>) -> Vec<u32> {
    set.iter().map(|ship| ship.raw()).collect()
}

fn limits(limits: &IntentLimits) -> LimitsOut {
    LimitsOut {
        forces: limits.forces.as_ref().map(|forces| ForcesOut {
            ships: ships(&forces.ships),
            task_forces: forces.task_forces.iter().map(|f| f.raw()).collect(),
            air_groups: forces.air_groups.iter().map(|g| g.raw()).collect(),
            land_units: forces.land_units.iter().map(|u| u.raw()).collect(),
            source: forces.source.map(|source| source.raw()),
        }),
        area: limits
            .area
            .as_ref()
            .map(|hexes| hexes.iter().map(|h| [h.q, h.r]).collect()),
        priority: limits.priority,
        protected: ships(&limits.protected),
        risk: limits.risk.map(|risk| risk.name()),
        not_before: limits.not_before,
        wait_for: limits.wait_for.map(awaited),
        after: limits.after.map(|other| other.raw()),
        knowledge: limits.knowledge.map(Knowledge::name),
        within_days: limits.knowledge.and_then(Knowledge::days),
        targets: limits.targets.as_ref().map(targets),
    }
}

#[derive(Serialize)]
struct IntentOut {
    id: u32,
    #[serde(flatten)]
    kind: IntentKindOut,
    /// The intent it stands under; left out for a root.
    #[serde(skip_serializing_if = "Option::is_none")]
    parent: Option<u32>,
    /// `"staff"` for an intent the staff formed; left out for the commander's.
    #[serde(skip_serializing_if = "Option::is_none")]
    maker: Option<&'static str>,
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
        parent: intent.parent.map(|parent| parent.raw()),
        maker: (intent.maker == Maker::Staff).then(|| intent.maker.name()),
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
    /// Decision 0159, 9: the day of a "not before" (`from_day`), what a "wait
    /// for" names and the intent of an "after" (`other_intent`); each left out where the limit is
    /// cleared.
    #[serde(skip_serializing_if = "Option::is_none")]
    from_day: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    task_force: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    land_unit: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    other_intent: Option<u32>,
    /// Decision 0159, 12: the setting of a "knowledge" limit and its days;
    /// each left out where the limit is cleared.
    #[serde(skip_serializing_if = "Option::is_none")]
    knowledge: Option<&'static str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    within_days: Option<u32>,
    /// Decision 0190: the kinds and the waters of a "targets" limit; left
    /// out where the limit is cleared.
    #[serde(skip_serializing_if = "Option::is_none")]
    targets: Option<TargetsOut>,
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
        from_day: None,
        task_force: None,
        land_unit: None,
        other_intent: None,
        knowledge: None,
        within_days: None,
        targets: None,
    };
    match limit {
        Limit::Forces { intent, limit } => LimitOut {
            intent: Some(intent.raw()),
            forces: Some(ForcesOut {
                ships: ships(&limit.ships),
                task_forces: limit.task_forces.iter().map(|f| f.raw()).collect(),
                air_groups: limit.air_groups.iter().map(|g| g.raw()).collect(),
                land_units: limit.land_units.iter().map(|u| u.raw()).collect(),
                source: limit.source.map(|source| source.raw()),
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
        Limit::NotBefore { intent, day } => LimitOut {
            intent: Some(intent.raw()),
            from_day: *day,
            ..blank("not_before")
        },
        Limit::WaitFor { intent, awaited } => {
            let named = awaited.map(self::awaited);
            LimitOut {
                intent: Some(intent.raw()),
                task_force: named.as_ref().and_then(|n| n.task_force),
                land_unit: named.as_ref().and_then(|n| n.land_unit),
                ..blank("wait_for")
            }
        }
        Limit::After { intent, other } => LimitOut {
            intent: Some(intent.raw()),
            other_intent: other.map(|other| other.raw()),
            ..blank("after")
        },
        Limit::Knowledge { intent, knowledge } => LimitOut {
            intent: Some(intent.raw()),
            knowledge: knowledge.map(Knowledge::name),
            within_days: knowledge.and_then(Knowledge::days),
            ..blank("knowledge")
        },
        Limit::Targets {
            intent,
            targets: set,
        } => LimitOut {
            intent: Some(intent.raw()),
            targets: set.as_ref().map(targets),
            ..blank("targets")
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
