//! The tools of a seat whose side has a staff (decision 0107, "As built",
//! "The tools"): `command_state`, `plans`, `propose`, `staff_orders`, `why`
//! and the answers of the six commands a seat queues.
//!
//! Every function here takes a [`SideView`] and nothing else of the game: a
//! view of the world as the coming day will find it (the commands the seat
//! has queued taken in, the forces back from the yard and the refuelling
//! done), which `session.rs` makes and nothing else may. The staff is asked
//! on that view, as it will be asked when the day resolves.
//!
//! A unit is shown in the side's own numbers (decision 0086): the staff's
//! words and commands carry the engine's ids (`ship:1`, `task_force:1000001`)
//! and [`Names`] writes them in the side's numbers, a task force the staff
//! will form being the next number the side would give it. A base keeps its
//! number. A quantity is a decimal string, never a float.

use std::collections::BTreeMap;

use serde_json::{json, Map, Value};
use sim_core::command::{
    AmendmentChange, CommandKind, CommandRefusal, CommandState, Committed, Displacement, HoldMaker,
    HoldTarget, Intent, IntentKind, IntentLimits, InterruptRef, InterruptSetting, Limit, Plan,
    PlanState, Proposal, Run, RunDays, Staff, StaffDecisions, StandingSubject, SupplyProjection,
};
use sim_core::events::EventKind;
use sim_core::fixed::Fixed;
use sim_core::ids::{AirGroupId, IntentId, LandUnitId, ShipId, TaskForceId};
use sim_core::orders::Issuer;
use sim_core::side_events::{SideCause, SideEvent, SideEventKind};
use sim_core::state::ShipLocation;
use sim_core::view::SideView;
use sim_core::world::MADE_IDS_FLOOR;

use crate::answer::{NotKnown, FORMAT, VERSION};
use crate::commands::{
    command_from_json, command_to_json, reason_to_json, staff_decisions_to_json,
};
use crate::common::{map_unit_tokens, Ctx};
use crate::orders::{order_to_json, unit_text};
use crate::record::CallError;

pub(crate) use crate::reporting::{fired_on_day, firing_words, firings_json, report};

/// The days between shipments the staff uses where the commander set none.
/// It is the doctrine's own figure (`DEFAULT_INTERVAL_DAYS` of `sim-staff`);
/// a test in `sim-cli` holds the two together.
pub const STAFF_DEFAULT_INTERVAL_DAYS: u32 = 15;

/// What every answer about the staff says first: it does nothing for a side
/// until an intent with a forces limit is set.
const STAFF_NOTE: &str = "The staff does nothing for your side until you set an intent. To keep a base supplied, give the intent a forces limit (set_intent, then set_limit forces). To hold a place or cover a base from the air (intent_kind hold or cover), the staff may use the air groups based at the base unless you limit them (set_limit forces with air_groups); under a hold intent the staff forms a cover intent by itself. To neutralise an enemy base (intent_kind neutralise, with target), the staff forms a raid from each of your bases with bombers that reach it, using the air groups there unless you limit them. To deny the enemy an area of sea (intent_kind deny, with hex as [q, r] and distance in whole hexes), the staff searches the waters and strikes ships there from your bases whose groups reach them (it forms strike_ships and search_waters intents under it, which you do not state); a strike_ships intent (intent_kind strike_ships, with base) gives a base that task by itself. Commands you queue take effect at the start of the coming day.";

/// The ships and forces of the umpire's ids shown in the side's numbers,
/// from the side's view.
pub(crate) struct Names<'a> {
    ctx: Ctx<'a>,
    next_force: u32,
}

impl<'a> Names<'a> {
    pub(crate) fn new(view: &SideView<'a>) -> Names<'a> {
        Names {
            ctx: Ctx::new(*view),
            next_force: view.next_task_force_id().raw(),
        }
    }

    /// The side's number of a task force: one it has, or one its next
    /// forming order makes (the next number past its last, and so on).
    fn force_number(&self, raw: u32) -> Option<u32> {
        self.ctx.force_number(TaskForceId::new(raw)).or_else(|| {
            let made = raw.checked_sub(self.next_force)?;
            (made < MADE_IDS_FLOOR).then(|| self.ctx.force_count() + 1 + made)
        })
    }

    /// Text with each unit token written in the side's numbers. `ships` and
    /// `forces` are the numbers a refusal's stand-ins stand for.
    fn text_with(
        &self,
        text: &str,
        ships: &BTreeMap<u32, u32>,
        forces: &BTreeMap<u32, u32>,
        groups: &BTreeMap<u32, u32>,
    ) -> String {
        map_unit_tokens(text, |kind, raw| match kind {
            "ship" => ships
                .get(&raw)
                .copied()
                .or_else(|| self.ctx.ship_number(ShipId::new(raw))),
            "task_force" => forces.get(&raw).copied().or_else(|| self.force_number(raw)),
            "land_unit" => self.ctx.land_unit_number(LandUnitId::new(raw)),
            _ => groups
                .get(&raw)
                .copied()
                .or_else(|| self.ctx.group_number(AirGroupId::new(raw))),
        })
    }

    pub(crate) fn text(&self, text: &str) -> String {
        self.text_with(text, &BTreeMap::new(), &BTreeMap::new(), &BTreeMap::new())
    }

    /// A JSON value with every string's unit tokens, and every number under
    /// `ships`, `task_forces` or `task_force`, written in the side's numbers.
    pub(crate) fn value(&self, value: &Value) -> Value {
        match value {
            Value::String(text) => Value::String(self.text(text)),
            Value::Array(items) => Value::Array(items.iter().map(|v| self.value(v)).collect()),
            Value::Object(map) => Value::Object(
                map.iter()
                    .map(|(key, v)| {
                        let shown = match (key.as_str(), v) {
                            ("ships", Value::Array(ids)) => Value::Array(
                                ids.iter()
                                    .map(|id| {
                                        id.as_u64()
                                            .and_then(|n| u32::try_from(n).ok())
                                            .and_then(|raw| self.ctx.ship_number(ShipId::new(raw)))
                                            .map_or_else(|| id.clone(), |n| json!(n))
                                    })
                                    .collect(),
                            ),
                            ("task_forces", Value::Array(ids)) => Value::Array(
                                ids.iter()
                                    .map(|id| {
                                        id.as_u64()
                                            .and_then(|n| u32::try_from(n).ok())
                                            .and_then(|raw| self.force_number(raw))
                                            .map_or_else(|| id.clone(), |n| json!(n))
                                    })
                                    .collect(),
                            ),
                            ("air_groups", Value::Array(ids)) => Value::Array(
                                ids.iter()
                                    .map(|id| {
                                        id.as_u64()
                                            .and_then(|n| u32::try_from(n).ok())
                                            .and_then(|raw| {
                                                self.ctx.group_number(AirGroupId::new(raw))
                                            })
                                            .map_or_else(|| id.clone(), |n| json!(n))
                                    })
                                    .collect(),
                            ),
                            // The task force of a stand or carrier intent and
                            // the task force level of a reason (decisions
                            // 0124 and 0125): the side's number.
                            ("task_force", Value::Number(number)) => number
                                .as_u64()
                                .and_then(|n| u32::try_from(n).ok())
                                .and_then(|raw| self.force_number(raw))
                                .map_or_else(|| v.clone(), |n| json!(n)),
                            _ => self.value(v),
                        };
                        (key.clone(), shown)
                    })
                    .collect(),
            ),
            other => other.clone(),
        }
    }

    pub(crate) fn ship(&self, id: ShipId) -> u32 {
        self.ctx.ship_number(id).unwrap_or_else(|| id.raw())
    }

    fn ships<'s>(&self, ids: impl IntoIterator<Item = &'s ShipId>) -> Vec<u32> {
        let mut numbers: Vec<u32> = ids.into_iter().map(|id| self.ship(*id)).collect();
        numbers.sort_unstable();
        numbers
    }

    /// The side's numbers of air groups, in order.
    fn groups<'s>(&self, ids: impl IntoIterator<Item = &'s AirGroupId>) -> Vec<u32> {
        let mut numbers: Vec<u32> = ids
            .into_iter()
            .map(|id| self.ctx.group_number(*id).unwrap_or_else(|| id.raw()))
            .collect();
        numbers.sort_unstable();
        numbers
    }

    pub(crate) fn base(&self, id: sim_core::ids::BaseId) -> Value {
        serde_json::to_value(self.ctx.base(id)).expect("a handle serialises")
    }
}

pub(crate) fn dec(value: Fixed) -> Value {
    Value::String(value.to_string())
}

fn days_json(days: &RunDays) -> Value {
    json!({
        "load": days.load,
        "sail": days.sail,
        "arrive": days.arrive,
        "free": days.free,
    })
}

fn issuer_word(issuer: Issuer) -> &'static str {
    match issuer {
        Issuer::Player => "commander",
        Issuer::Staff => "staff",
        Issuer::LocalCommander => "local_commander",
    }
}

pub(crate) fn answer(kind: &str, mut fields: Value) -> Value {
    let map = fields.as_object_mut().expect("an answer is an object");
    let mut out = Map::new();
    out.insert("format".into(), json!(FORMAT));
    out.insert("version".into(), json!(VERSION));
    out.insert("kind".into(), json!(kind));
    out.append(map);
    Value::Object(out)
}

// ---------------------------------------------------------------------------
// Intents, limits, plans.
// ---------------------------------------------------------------------------

fn force_numbers<'s>(n: &Names<'_>, forces: impl IntoIterator<Item = &'s TaskForceId>) -> Vec<u32> {
    let mut numbers: Vec<u32> = forces
        .into_iter()
        .map(|f| n.ctx.force_number(*f).unwrap_or_else(|| f.raw()))
        .collect();
    numbers.sort_unstable();
    numbers
}

fn forces_json(n: &Names<'_>, forces: &sim_core::command::ForcesLimit) -> Value {
    json!({
        "ships": n.ships(&forces.ships),
        "task_forces": force_numbers(n, &forces.task_forces),
        "air_groups": n.groups(&forces.air_groups),
        "source": forces.source.map(|source| n.base(source)),
    })
}

/// The limits set on one intent itself. A limit not set on it is `null` (a
/// risk, too: `limits_read_up` says what it is read as, from the tree).
fn limits_json(n: &Names<'_>, limits: &IntentLimits) -> Value {
    json!({
        "forces": limits.forces.as_ref().map(|forces| forces_json(n, forces)),
        "area": limits.area.as_ref().map(|hexes| {
            hexes.iter().map(|h| json!([h.q, h.r])).collect::<Vec<_>>()
        }),
        "priority": limits.priority,
        "protected": n.ships(&limits.protected),
        "risk": limits.risk.map(|risk| risk.name()),
    })
}

/// `intent` and the intents above it, nearest first.
fn up_the_tree<'a>(state: &'a CommandState, intent: &'a Intent) -> Vec<&'a Intent> {
    let mut chain = vec![intent];
    chain.extend(state.ancestors(intent.id));
    chain
}

/// The limits of an intent as the staff reads them, up the tree (decision
/// 0117, 7): for each limit the value, the intent it is read from, and where
/// none is set the words for what then holds.
fn limits_read_up_json(n: &Names<'_>, state: &CommandState, intent: &Intent) -> Value {
    let chain = up_the_tree(state, intent);
    let from = |found: Option<&Intent>| found.map(|i| i.id.raw());
    let forces = chain.iter().find(|i| i.limits.forces.is_some()).copied();
    // Every limit on the chain binds: the least bold risk and the hexes in
    // every area (the architect's ruling after the build of decision 0117).
    let bound_risk = state.risk_read_up(intent.id);
    let risk = chain
        .iter()
        .find(|i| i.limits.risk == Some(bound_risk))
        .copied();
    let bound_area = state.area_read_up(intent.id);
    let bound_groups = state.groups_on_chain(intent.id);
    let priority = chain.iter().find(|i| i.limits.priority.is_some()).copied();
    let area = chain.iter().find(|i| i.limits.area.is_some()).copied();
    json!({
        "forces": forces.and_then(|i| i.limits.forces.as_ref()).map(|f| forces_json(n, f)),
        "forces_from": from(forces),
        "air_groups_binding": bound_groups.map(|groups| n.groups(&groups)),
        "forces_if_unset": if forces.is_some() {
            Value::Null
        } else {
            json!(match intent.addressee {
                sim_core::command::Addressee::Area =>
                    "not set on it or above it: the staff may use nothing (the area commands nothing)",
                sim_core::command::Addressee::Base(_) =>
                    "not set on it or above it: the staff may use the air groups based at its base that are not held",
                sim_core::command::Addressee::AirGroup(_) =>
                    "not set on it or above it: the staff may use the air group itself",
                sim_core::command::Addressee::TaskForce(_) =>
                    "not set on it or above it: the staff may use the task force itself",
            })
        },
        "risk": bound_risk.name(),
        "risk_from": from(risk),
        "risk_set": risk.is_some(),
        "risk_if_unset": if risk.is_some() {
            Value::Null
        } else {
            json!("not set on it or above it: balanced")
        },
        "area": bound_area.map(|hexes| {
            hexes.iter().map(|h| json!([h.q, h.r])).collect::<Vec<_>>()
        }),
        "area_from": from(area),
        "priority": priority.and_then(|i| i.limits.priority),
        "priority_from": from(priority),
    })
}

/// An intent's kind as the interface writes it: its name, its own base
/// (`null` for neutralising a base and for denying an area), its target
/// (`null` for the kinds that have none), its waters (`hex` as `[q, r]` and
/// `distance`, `null` for the kinds but a deny; `hex` is also the station of
/// a stand_at and an operate_carriers, whose `distance` is `null`), its task
/// force (`task_force`, the side's number; `null` for the kinds that have
/// none), and for keeping a base supplied the days (`null` for the other
/// kinds).
fn kind_json(n: &Names<'_>, kind: IntentKind) -> Value {
    let (floor_days, interval_days) = match kind {
        IntentKind::KeepSupplied {
            floor_days,
            interval_days,
            ..
        } => (Some(floor_days), interval_days),
        IntentKind::Cover { .. }
        | IntentKind::Hold { .. }
        | IntentKind::Neutralise { .. }
        | IntentKind::Raid { .. }
        | IntentKind::Deny { .. }
        | IntentKind::StrikeShips { .. }
        | IntentKind::SearchWaters { .. }
        | IntentKind::StandAt { .. }
        | IntentKind::OperateCarriers { .. } => (None, None),
    };
    json!({
        "kind": kind.name(),
        "base": kind.own_base().map(|base| n.base(base)),
        "target": kind.target().map(|target| n.base(target)),
        "task_force": kind.force().map(|force| n.force_number(force.raw()).unwrap_or_else(|| force.raw())),
        "hex": kind
            .waters()
            .map(|(hex, _)| hex)
            .or_else(|| kind.station())
            .map(|hex| json!([hex.q, hex.r])),
        "distance": kind.waters().map(|(_, distance)| distance),
        "floor_days": floor_days,
        "interval_days": interval_days,
    })
}

fn projection_json(projection: &SupplyProjection) -> Value {
    let readings: Vec<Value> = projection
        .readings
        .iter()
        .map(|r| {
            json!({
                "day": r.day,
                "stock_t": dec(r.stock_t),
                "use_t": dec(r.use_t),
                "days": r.days.map(dec),
            })
        })
        .collect();
    json!({
        "hex": {"q": projection.hex.q, "r": projection.hex.r},
        "floor_days": projection.floor_days,
        "today": readings.first().cloned(),
        "first_day_below_floor": projection.first_day_below,
        "readings": readings,
    })
}

/// The short form of a projection for `command_state`: today's reading and
/// the first day below the floor.
fn projection_summary(projection: &SupplyProjection) -> Value {
    let today = projection.readings.first();
    json!({
        "floor_days": projection.floor_days,
        "today": today.map(|r| json!({
            "day": r.day,
            "stock_t": dec(r.stock_t),
            "use_t": dec(r.use_t),
            "days": r.days.map(dec),
        })),
        "first_day_below_floor": projection.first_day_below,
    })
}

fn intent_json(n: &Names<'_>, staff: &dyn Staff, view: &SideView<'_>, intent: &Intent) -> Value {
    let state = view.command_state();
    let interval = match intent.kind {
        IntentKind::KeepSupplied {
            interval_days: Some(days),
            ..
        } => Some(days.to_string()),
        IntentKind::KeepSupplied { .. } => Some(format!(
            "not set: the staff uses {STAFF_DEFAULT_INTERVAL_DAYS}"
        )),
        _ => None,
    };
    let proposal = if intent.in_force() {
        staff.propose(*view, intent.id)
    } else {
        None
    };
    let displaced_by: Vec<Value> = proposal
        .as_ref()
        .and_then(|p| p.shortfall.as_ref())
        .map(|s| {
            s.displaced_by
                .iter()
                .map(|d| displacement_json(n, d))
                .collect()
        })
        .unwrap_or_default();
    let projection = proposal
        .and_then(|p| p.projection)
        .as_ref()
        .map(projection_summary);
    // Its place in the order the staff serves: ranked first, then by number,
    // the rank read up the tree (decision 0117, 7 and 11).
    let served = intent.in_force().then(|| {
        let mut order: Vec<&Intent> = state.intents_in_force().collect();
        order.sort_by_key(|i| {
            let rank = state.priority_read_up(i.id);
            (rank.is_none(), rank.unwrap_or(0), i.id)
        });
        order
            .iter()
            .position(|i| i.id == intent.id)
            .map_or(0, |at| at + 1)
    });
    let mut body = json!({
        "number": intent.id.raw(),
        "in_force": intent.in_force(),
        "interval": interval,
        "set_day": intent.set_day,
        "withdrawn_day": intent.withdrawn_day,
        "plan": intent.plan.raw(),
        "parent": intent.parent.map(|p| p.raw()),
        "maker": intent.maker.name(),
        "addressee": n.text(&intent.addressee.to_string()),
        "children": state
            .children_in_force(intent.id)
            .iter()
            .map(|c| c.raw())
            .collect::<Vec<_>>(),
        "depth": state.ancestors(intent.id).len(),
        "limits": limits_json(n, &intent.limits),
        "limits_read_up": limits_read_up_json(n, state, intent),
        "projection": projection,
        "served": served,
        "displaced_by": displaced_by,
    });
    if let (Some(map), Value::Object(kind)) = (body.as_object_mut(), kind_json(n, intent.kind)) {
        map.extend(kind);
    }
    if matches!(intent.kind, IntentKind::Hold { .. }) {
        body["measure"] = hold_measure_json(&crate::hold_measure(view, intent.id));
    }
    if matches!(intent.kind, IntentKind::Neutralise { .. }) {
        body["measure"] = neutralise_measure_json(&crate::neutralise_measure(view, intent.id));
    }
    if matches!(intent.kind, IntentKind::Deny { .. }) {
        body["measure"] = deny_measure_json(&crate::deny_measure(view, intent.id));
    }
    body
}

/// The measure of a deny intent as the interface writes it, with its words
/// (decision 0123, 4): `null` where it is not a deny intent.
pub(crate) fn deny_measure_json(measure: &Option<crate::DenyMeasure>) -> Value {
    let Some(m) = measure else {
        return Value::Null;
    };
    json!({
        "days_in_force": m.days_in_force,
        "contacts_held": m.contacts_held,
        "strikes_launched": m.strikes_launched,
        "claimed_sunk": m.claimed_sunk,
        "claimed_hits": m.claimed_hits,
        "days_met": m.days_met,
        "text": format!(
            "{} of the {} days in force ended with no contact held in the waters; {} contacts held in the waters at the end of a day; {} strikes launched from the bases given the task of striking ships; {} ships claimed sunk and {} hits claimed from the air in the waters (claims of your people, never below the truth)",
            m.days_met, m.days_in_force, m.contacts_held, m.strikes_launched, m.claimed_sunk, m.claimed_hits
        ),
    })
}

/// The measure of a neutralise intent as the interface writes it, with its
/// words (decision 0120, 8): `null` where it is not a neutralise intent.
pub(crate) fn neutralise_measure_json(measure: &Option<crate::NeutraliseMeasure>) -> Value {
    let Some(m) = measure else {
        return Value::Null;
    };
    json!({
        "days_in_force": m.days_in_force,
        "days_met": m.days_met,
        "raids_flown": m.raids_flown,
        "claimed_on_ground": m.claimed_on_ground,
        "raids_on_own_bases": m.raids_on_own_bases,
        "text": format!(
            "{} raids flown on the target in {} days in force; {} aircraft claimed destroyed on the ground there (a claim of your people, never below the truth); {} air raids on your bases, {} of the {} days without a raid on any of them",
            m.raids_flown, m.days_in_force, m.claimed_on_ground, m.raids_on_own_bases, m.days_met, m.days_in_force
        ),
    })
}

/// The measure of a hold intent as the interface writes it, with its words
/// (decision 0117, 14): `null` where it is not a hold intent.
pub(crate) fn hold_measure_json(measure: &Option<crate::HoldMeasure>) -> Value {
    let Some(m) = measure else {
        return Value::Null;
    };
    let supply = match m.days_of_supply {
        Some(days) => format!("{days} days of supply at the base today"),
        None => "no figure for the base's days of supply".to_string(),
    };
    json!({
        "days_in_force": m.days_in_force,
        "days_met": m.days_met,
        "raids": m.raids,
        "raids_fought": m.raids_fought,
        "days_of_supply": m.days_of_supply.map(dec),
        "text": format!(
            "held on {} of {} days in force (the base the side's at the end of the day with its airfield open); {} air raids, {} fought by a patrol of the side's; {supply}",
            m.days_met, m.days_in_force, m.raids, m.raids_fought
        ),
    })
}

fn amendment_text(n: &Names<'_>, plan: &Plan, change: &AmendmentChange) -> String {
    let command = match change {
        AmendmentChange::Intent(kind) => CommandKind::SetIntent {
            intent: plan.intent,
            kind: *kind,
        },
        AmendmentChange::Limit(limit) => CommandKind::SetLimit(limit.clone()),
    };
    n.text(&command.to_string())
}

/// Where a ship is, in words, and the supply aboard.
fn ship_state(n: &Names<'_>, id: ShipId) -> Value {
    let Some(ship) = n.ctx.ships.get(&id) else {
        return json!({"ship": n.ship(id), "at": "not known", "supply_aboard_t": "0"});
    };
    let at = match ship.location {
        ShipLocation::Port(base) => format!("in port at {base}"),
        ShipLocation::TaskForce(force) => {
            let number = n.ctx.force_number(force).unwrap_or_else(|| force.raw());
            match n.ctx.forces.get(&force) {
                Some(f) => format!(
                    "in task_force:{number} at ({},{})",
                    f.position().q,
                    f.position().r
                ),
                None => format!("in task_force:{number}"),
            }
        }
        ShipLocation::Yard(force) => {
            let number = n.ctx.force_number(force).unwrap_or_else(|| force.raw());
            format!("away at the yard with task_force:{number}")
        }
        ShipLocation::Sunk => "sunk".to_string(),
    };
    json!({
        "ship": n.ship(id),
        "at": at,
        "supply_aboard_t": dec(ship.supply_t),
    })
}

/// What a run is doing by the staff's days.
fn run_phase(run: &Run, today: u32) -> (&'static str, String) {
    let days = run.days;
    if today < days.load {
        ("waiting", format!("not begun: loads on day {}", days.load))
    } else if today < days.sail {
        (
            "loading",
            format!(
                "loading {} t of supply at base:{} for the sail on day {}",
                run.load_t,
                run.source.raw(),
                days.sail
            ),
        )
    } else if today <= days.arrive {
        (
            "under_way",
            format!(
                "sailed on day {}, due at the destination on day {}",
                days.sail, days.arrive
            ),
        )
    } else if today < days.free {
        (
            "returning",
            format!(
                "delivered or unloading, and coming home: its ships are free on day {}",
                days.free
            ),
        )
    } else {
        (
            "finished",
            format!("its ships were due free on day {}", days.free),
        )
    }
}

fn run_json(n: &Names<'_>, run: &Run, today: u32, doing: bool) -> Value {
    let mut out = json!({
        "number": run.number,
        "ships": n.ships(&run.ships),
        "load_t": dec(run.load_t),
        "source": n.base(run.source),
        "days": days_json(&run.days),
    });
    if doing {
        let (phase, text) = run_phase(run, today);
        out["phase"] = json!(phase);
        out["doing"] = json!(text);
        out["ship_states"] = Value::Array(run.ships.iter().map(|s| ship_state(n, *s)).collect());
    }
    out
}

fn plan_json(n: &Names<'_>, view: &SideView<'_>, plan: &Plan, doing: bool) -> Value {
    let today = view.day() + 1;
    let state = view.command_state();
    let intent_in_force = state
        .intents
        .get(&plan.intent)
        .is_some_and(|intent| intent.in_force());
    json!({
        "number": plan.id.raw(),
        "intent": plan.intent.raw(),
        "intent_in_force": intent_in_force,
        "state": plan.state.name(),
        "baseline": plan.baseline.as_ref().map(|b| {
            let mut body = json!({
                "day": b.day,
                "limits": limits_json(n, &b.limits),
            });
            if let (Some(map), Value::Object(kind)) = (body.as_object_mut(), kind_json(n, b.kind)) {
                map.extend(kind);
            }
            body
        }),
        "baseline_day": plan.baseline.as_ref().map(|b| b.day),
        "amendments": plan
            .amendments
            .iter()
            .map(|a| json!({
                "number": a.number,
                "day": a.day,
                "change": amendment_text(n, plan, &a.change),
            }))
            .collect::<Vec<_>>(),
        "runs": plan
            .runs
            .values()
            .map(|run| run_json(n, run, today, doing))
            .collect::<Vec<_>>(),
    })
}

fn leaves_json(view: &SideView<'_>) -> Value {
    let leaves = view.command_state().leaves;
    json!({
        "prepare": leaves.prepare,
        "launch": leaves.launch,
        "meaning": "prepare: the staff may approve a draft plan that is inside your limits (given by default). launch: the staff may release a plan that is preparing (withheld by default: until you give it, you release a plan yourself with approve).",
    })
}

// ---------------------------------------------------------------------------
// Holds (decision 0107, step 3, rulings 17 to 23).
// ---------------------------------------------------------------------------

/// A hold's target as the interface writes it, in the side's numbers. With
/// the `request` it was read from, a number the side wrote that names none
/// of its units is shown as the side wrote it.
fn hold_target_json(n: &Names<'_>, target: HoldTarget, request: Option<&Request>) -> Value {
    let written = |map: fn(&Request) -> &BTreeMap<u32, u32>, raw: u32| {
        request.and_then(|r| map(r).get(&raw).copied())
    };
    match target {
        HoldTarget::Ship(ship) => json!({
            "ship": written(|r| &r.ship_stand_ins, ship.raw()).unwrap_or_else(|| n.ship(ship)),
        }),
        HoldTarget::TaskForce(force) => json!({
            "task_force": written(|r| &r.force_stand_ins, force.raw())
                .or_else(|| n.force_number(force.raw()))
                .unwrap_or_else(|| force.raw()),
        }),
        HoldTarget::Setting { group, setting } => json!({
            "air_group": written(|r| &r.group_stand_ins, group.raw())
                .or_else(|| n.ctx.group_number(group))
                .unwrap_or_else(|| group.raw()),
            "setting": setting.name(),
        }),
        HoldTarget::AirGroup(group) => json!({
            "air_group": written(|r| &r.group_stand_ins, group.raw())
                .or_else(|| n.ctx.group_number(group))
                .unwrap_or_else(|| group.raw()),
        }),
    }
}

/// A command as the interface writes it, in the side's numbers.
fn command_json(n: &Names<'_>, command: &CommandKind, request: Option<&Request>) -> Value {
    let target = match command {
        CommandKind::Hold { target } | CommandKind::Release { target } => Some(*target),
        _ => None,
    };
    // An acknowledgement of a ship lost names the ship in the side's number,
    // or the number the side wrote for one that is not its own.
    if let CommandKind::Acknowledge {
        interrupt:
            InterruptRef::Standing {
                subject: StandingSubject::Ship(ship),
                ..
            },
    } = command
    {
        let mut body = n.value(&command_to_json(command));
        body["ship"] = json!(request
            .and_then(|r| r.ship_stand_ins.get(&ship.raw()).copied())
            .unwrap_or_else(|| n.ship(*ship)));
        return body;
    }
    match target {
        Some(target) => {
            let mut body = json!({"kind": command.name()});
            if let (Some(body), Value::Object(keys)) =
                (body.as_object_mut(), hold_target_json(n, target, request))
            {
                body.extend(keys);
            }
            body
        }
        None => {
            let mut body = n.value(&command_to_json(command));
            if let CommandKind::SetIntent { kind, .. } | CommandKind::FormIntent { kind, .. } =
                command
            {
                if let Some(force) = kind.force() {
                    body["task_force"] = json!(written_force(n, force, request));
                }
            }
            body
        }
    }
}

/// The side's number of a task force an intent names, or the number the side
/// wrote for one that is not its own.
fn written_force(n: &Names<'_>, force: TaskForceId, request: Option<&Request>) -> u32 {
    request
        .and_then(|r| r.force_stand_ins.get(&force.raw()).copied())
        .or_else(|| n.force_number(force.raw()))
        .unwrap_or_else(|| force.raw())
}

/// A command in words, in the side's numbers.
fn command_text(n: &Names<'_>, command: &CommandKind, request: Option<&Request>) -> String {
    match request {
        Some(r) => n.text_with(
            &command.to_string(),
            &r.ship_stand_ins,
            &r.force_stand_ins,
            &r.group_stand_ins,
        ),
        None => n.text(&command.to_string()),
    }
}

/// A hold's target in words, in the side's numbers.
fn hold_target_words(n: &Names<'_>, target: HoldTarget) -> String {
    match target {
        HoldTarget::Ship(ship) => format!("ship {}", n.ship(ship)),
        HoldTarget::TaskForce(force) => format!(
            "task force {}",
            n.ctx.force_number(force).unwrap_or_else(|| force.raw())
        ),
        HoldTarget::Setting { group, setting } => format!(
            "the {} of air group {}",
            setting.name().replace('_', " "),
            n.ctx.group_number(group).unwrap_or_else(|| group.raw())
        ),
        HoldTarget::AirGroup(group) => format!(
            "air group {}",
            n.ctx.group_number(group).unwrap_or_else(|| group.raw())
        ),
    }
}

/// What made a hold, in words: "your hold command of day 8" or "your order
/// 3 of day 2".
fn hold_maker_words(n: &Names<'_>, hold: &sim_core::command::Hold) -> String {
    match hold.by {
        HoldMaker::Command => format!("your hold command of day {}", hold.day),
        HoldMaker::Order(id) => match n.ctx.order_number_if_own(id) {
            Some(number) => format!("your order {number} of day {}", hold.day),
            None => format!("an order of yours of day {}", hold.day),
        },
    }
}

/// The holds of the side, each with its target, the day, what made it and
/// the words.
fn holds_json(n: &Names<'_>, view: &SideView<'_>) -> Vec<Value> {
    view.command_state()
        .holds
        .iter()
        .map(|(target, hold)| {
            let (made_by, order) = match hold.by {
                HoldMaker::Command => ("command", None),
                HoldMaker::Order(id) => ("order", n.ctx.order_number_if_own(id)),
            };
            json!({
                "target": hold_target_json(n, *target, None),
                "day": hold.day,
                "made_by": made_by,
                "order": order,
                "text": format!(
                    "{}, held by {}",
                    hold_target_words(n, *target),
                    hold_maker_words(n, hold)
                ),
            })
        })
        .collect()
}

// ---------------------------------------------------------------------------
// What the staff decided, by event.
// ---------------------------------------------------------------------------

/// The decision a `StaffReason` event records: what it is for and its
/// reason, in the side's numbers. `None` for an event that is no reason.
fn decision_of(
    n: &Names<'_>,
    view: &SideView<'_>,
    events: &[SideEvent],
    event: &SideEvent,
) -> Option<(Value, Value)> {
    let SideEventKind::Whole(EventKind::StaffReason { order, reason, .. }) = &event.kind else {
        return None;
    };
    let reason_json = n.value(&reason_to_json(reason));
    let cited = |number: u32| number.checked_sub(1).and_then(|i| events.get(i as usize));
    let for_json = if let Some(id) = order {
        let number = n.ctx.order_number(*id);
        let order = view.orders().find(|o| o.id == *id);
        let verdict = events.iter().find_map(|e| match &e.kind {
            SideEventKind::Whole(EventKind::OrderAccepted { order, .. }) if *order == *id => {
                Some((e.number, None))
            }
            SideEventKind::Whole(EventKind::OrderRejected { order, reason, .. })
                if *order == *id =>
            {
                Some((e.number, Some(crate::words::refusal(reason))))
            }
            _ => None,
        });
        let (accepted, refusal, verdict_event) = match &verdict {
            Some((number, None)) => (Some(true), None, Some(*number)),
            Some((number, Some(why))) => (Some(false), Some(n.text(why)), Some(*number)),
            None => (None, None, None),
        };
        json!({
            "type": "order",
            "order": number,
            "unit": order.map(|o| n.text(&unit_text(&o.kind))),
            "body": order.map(|o| n.value(&order_to_json(&o.kind))),
            "intent": order.and_then(|o| o.origin.intent).map(|i| i.raw()),
            "accepted": accepted,
            "refusal": refusal,
            "event": verdict_event,
        })
    } else if let Some((cause, kind)) = event.causes.iter().find_map(|c| match c {
        SideCause::Event(k) => cited(*k).map(|e| (*k, e)),
        SideCause::Order(_) => None,
    }) {
        match &kind.kind {
            SideEventKind::Whole(EventKind::CommandTaken {
                issuer, command, ..
            }) => json!({
                "type": "command",
                "command": command_json(n, command, None),
                "text": n.text(&command.to_string()),
                "issuer": issuer_word(*issuer),
                "taken": true,
                "refusal": Value::Null,
                "event": cause,
            }),
            SideEventKind::Whole(EventKind::CommandRefused {
                issuer,
                command,
                reason,
                ..
            }) => json!({
                "type": "command",
                "command": command_json(n, command, None),
                "text": n.text(&command.to_string()),
                "issuer": issuer_word(*issuer),
                "taken": false,
                "refusal": n.text(&reason.to_string()),
                "event": cause,
            }),
            _ => json!({"type": "no_order"}),
        }
    } else {
        json!({"type": "no_order"})
    };
    Some((for_json, reason_json))
}

/// The staff's decisions of one day, each with the number `why` takes: what
/// it was for in a line, and whether it was carried out (an order accepted,
/// a command taken; none for a decision to give no order).
fn decisions_of_day(n: &Names<'_>, view: &SideView<'_>, day: u32) -> Vec<Value> {
    let events = view.events();
    events
        .iter()
        .filter(|e| e.day == day)
        .filter_map(|e| {
            let (for_json, reason) = decision_of(n, view, &events, e)?;
            let text = |key: &str| for_json[key].as_str().unwrap_or("").to_string();
            let (kind, summary, carried_out) = match for_json["type"].as_str() {
                Some("order") => (
                    "order",
                    format!(
                        "order to {}: {}",
                        text("unit"),
                        for_json["body"]["kind"].as_str().unwrap_or("")
                    ),
                    for_json["accepted"].clone(),
                ),
                Some("command") => ("command", text("text"), for_json["taken"].clone()),
                _ => (
                    "no_order",
                    reason["worked"].as_str().unwrap_or("").to_string(),
                    Value::Null,
                ),
            };
            Some(json!({
                "number": e.number,
                "day": e.day,
                "for": kind,
                "summary": summary,
                "carried_out": carried_out,
            }))
        })
        .collect()
}

// ---------------------------------------------------------------------------
// The tools.
// ---------------------------------------------------------------------------

/// `command_state`: the side's command state as the coming day will find it.
/// `queued` are the commands the seat has queued, in order.
pub fn command_state(view: &SideView<'_>, staff: &dyn Staff, queued: &[CommandKind]) -> Value {
    let n = Names::new(view);
    let state = view.command_state();
    let today = view.day() + 1;
    let mut notes: Vec<String> = vec![STAFF_NOTE.to_string()];
    for intent in state.intents_in_force() {
        // An intent given to the area commands nothing without a forces
        // limit, its own or one above it; one given to a base may use the
        // groups based there (decision 0117, 8).
        if intent.addressee == sim_core::command::Addressee::Area
            && state.forces_on_chain(intent.id).is_empty()
        {
            notes.push(format!(
                "intent:{} has no forces limit: the staff gives no order for it until you set one (set_limit forces).",
                intent.id.raw()
            ));
        }
        if intent.maker == sim_core::command::Maker::Staff {
            notes.push(format!(
                "intent:{} was formed by the staff under intent:{}: you shape it by its limits and holds, or by withdrawing the intent it stands under; set_intent and withdraw_intent are refused for it.",
                intent.id.raw(),
                intent.parent.map_or(0, |p| p.raw())
            ));
        }
    }
    for plan in state.plans.values() {
        match plan.state {
            PlanState::Draft => notes.push(format!(
                "plan:{} is a draft: {}",
                plan.id.raw(),
                if state.leaves.prepare {
                    "the staff will prepare it if the intent has a forces limit, or you can approve it."
                } else {
                    "approve it to prepare it (you withheld the leave to prepare)."
                }
            )),
            PlanState::Preparing => notes.push(format!(
                "plan:{} is preparing: {}",
                plan.id.raw(),
                if state.leaves.launch {
                    "the staff will release it."
                } else {
                    "approve it to release it (the leave to launch is withheld)."
                }
            )),
            _ => {}
        }
    }
    answer(
        "command_state",
        json!({
            "for_day": today,
            "as_at": format!("the start of day {today}, with the commands you have queued taken in"),
            "notes": notes,
            "leaves": leaves_json(view),
            "intents": in_tree_order(state)
                .into_iter()
                .map(|intent| intent_json(&n, staff, view, intent))
                .collect::<Vec<_>>(),
            "reserves": view
                .bases()
                .map(|b| json!({
                    "base": n.base(b.id),
                    "supply_t": dec(b.supply_t),
                    "fuel_t": dec(b.fuel_t),
                    "supply_reserve_t": dec(b.supply_reserve_t),
                    "fuel_reserve_t": dec(b.fuel_reserve_t),
                }))
                .collect::<Vec<_>>(),
            "plans": state
                .plans
                .values()
                .map(|plan| plan_json(&n, view, plan, false))
                .collect::<Vec<_>>(),
            "commitments": state
                .commitments
                .iter()
                .flat_map(|(ship, list)| list.iter().map(move |c| (ship, c)))
                .map(|(ship, c)| json!({
                    "ship": n.ship(*ship),
                    "plan": c.plan.raw(),
                    "run": c.run,
                    "from_day": c.from_day,
                    "to_day": c.to_day,
                }))
                .collect::<Vec<_>>(),
            "holds": holds_json(&n, view),
            "interrupts": crate::reporting::interrupts_json(&n, view),
            "queued": queued
                .iter()
                .enumerate()
                .map(|(i, command)| json!({
                    "index": i,
                    "command": command_json(&n, command, None),
                    "text": command_text(&n, command, None),
                }))
                .collect::<Vec<_>>(),
            "staff_decisions_yesterday": decisions_of_day(&n, view, view.day()),
        }),
    )
}

/// Every intent of the side, each root by number with the intents beneath it
/// after it, depth first, each branch by number (decision 0117, 14: a
/// sub-intent is listed under its parent). With no tree it is by number.
fn in_tree_order(state: &CommandState) -> Vec<&Intent> {
    fn walk<'a>(state: &'a CommandState, intent: &'a Intent, out: &mut Vec<&'a Intent>) {
        out.push(intent);
        for child in state
            .intents
            .values()
            .filter(|c| c.parent == Some(intent.id))
        {
            // A parent is always a lower number than its child, so this
            // cannot loop.
            if child.id != intent.id {
                walk(state, child, out);
            }
        }
    }
    let mut out = Vec::new();
    for root in state
        .intents
        .values()
        .filter(|i| i.parent.is_none_or(|p| !state.intents.contains_key(&p)))
    {
        walk(state, root, &mut out);
    }
    out
}

/// `plans`: the plans with their runs and what each run is doing.
pub fn plans(view: &SideView<'_>) -> Value {
    let n = Names::new(view);
    let today = view.day() + 1;
    answer(
        "plans",
        json!({
            "for_day": today,
            "as_at": format!("the start of day {today}, with the commands you have queued taken in"),
            "plans": view
                .command_state()
                .plans
                .values()
                .map(|plan| plan_json(&n, view, plan, true))
                .collect::<Vec<_>>(),
        }),
    )
}

/// `propose`: what the staff would do for one intent.
pub fn propose(view: &SideView<'_>, staff: &dyn Staff, intent: u32) -> Result<Value, CallError> {
    let n = Names::new(view);
    let today = view.day() + 1;
    let Some(proposal) = staff.propose(*view, IntentId::new(intent)) else {
        return Err(CallError::new(format!(
            "intent:{intent} is not an intent in force (command_state lists the intents)"
        )));
    };
    let in_force = view.command_state().intents_in_force().count();
    let displaces = if in_force <= 1 {
        "nothing: one intent".to_string()
    } else {
        displaces_text(&n, &proposal)
    };
    let displaced_by: Vec<Value> = proposal
        .shortfall
        .as_ref()
        .map(|s| {
            s.displaced_by
                .iter()
                .map(|d| displacement_json(&n, d))
                .collect()
        })
        .unwrap_or_default();
    let displaced_intents: Vec<Value> = proposal
        .displaces
        .iter()
        .map(|d| {
            json!({
                "intent": d.intent.raw(),
                "plan": d.plan.raw(),
                "shortfall_days": dec(d.shortfall.shortfall_days),
                "check_day": d.shortfall.check_day,
                "by": d.shortfall.displaced_by.iter().map(|b| displacement_json(&n, b)).collect::<Vec<_>>(),
            })
        })
        .collect();
    Ok(answer(
        "propose",
        json!({
            "for_day": today,
            "intent": intent,
            "plan": {"number": proposal.plan.raw(), "state": proposal.plan_state.name()},
            "projection": proposal.projection.as_ref().map(projection_json),
            "runs": proposal
                .runs
                .iter()
                .enumerate()
                .map(|(i, run)| json!({
                    "index": i + 1,
                    "ships": n.ships(&run.ships),
                    "load_t": dec(run.load_t),
                    "days": days_json(&run.days),
                    "reason": n.value(&reason_to_json(&run.reason)),
                }))
                .collect::<Vec<_>>(),
            "binds": proposal.shortfall.as_ref().map(|s| json!({
                "binds": n.text(&s.binds),
                "shortfall_days": dec(s.shortfall_days),
                "source_dry_day": s.source_dry_day,
                "check_day": s.check_day,
                "displaced_by": s.displaced_by.iter().map(|d| displacement_json(&n, d)).collect::<Vec<_>>(),
            })),
            "displaces": displaces,
            "displaced_by": displaced_by,
            "displaced_intents": displaced_intents,
            "tree": tree_json(&n, view, &proposal),
            "would_hand_in": with_day(decisions_json(&n, &proposal.decisions), today),
            "reason": n.value(&reason_to_json(&proposal.reason)),
        }),
    ))
}

/// The tree the staff would form for a hold, cover, neutralise or raid
/// intent (decisions 0117, 14, and 0120), read from the decisions of its proposal and nothing else: the
/// sub-intents it would form (each `form_intent` command, numbered as the
/// side's next free numbers), the air groups each intent would order and the
/// order each is given (a mission, a search arc, a patrol share, a rest), and
/// the search arc with its threats as the reason of its order words them.
/// `null` for keeping a base supplied (the tree of a neutralise intent holds
/// the raids the staff would form under it).
fn tree_json(n: &Names<'_>, view: &SideView<'_>, proposal: &Proposal) -> Value {
    let state = view.command_state();
    let Some(found) = state.intents.get(&proposal.intent) else {
        return Value::Null;
    };
    if matches!(found.kind, IntentKind::KeepSupplied { .. }) {
        return Value::Null;
    }
    let mut next = state.next_intent_number().raw();
    let mut sub_intents: Vec<Value> = Vec::new();
    for command in &proposal.decisions.commands {
        let CommandKind::FormIntent { parent, kind } = &command.kind else {
            continue;
        };
        let mut entry = json!({
            "number": next,
            "parent": parent.raw(),
            "maker": "staff",
            "text": command_text(n, &command.kind, None),
            "reason": n.value(&reason_to_json(&command.reason)),
        });
        if let (Some(map), Value::Object(kind)) = (entry.as_object_mut(), kind_json(n, *kind)) {
            map.extend(kind);
        }
        sub_intents.push(entry);
        next += 1;
    }
    // The orders to air groups, by the intent each serves: the groups that
    // intent would use, with their missions and settings.
    let mut by_intent: BTreeMap<u32, Vec<Value>> = BTreeMap::new();
    let mut arcs: Vec<Value> = Vec::new();
    for order in &proposal.decisions.orders {
        let sim_core::orders::OrderKind::AirGroup {
            target,
            order: body,
        } = &order.kind
        else {
            continue;
        };
        let group = n.ctx.group_number(*target).unwrap_or_else(|| target.raw());
        by_intent
            .entry(order.intent.raw())
            .or_default()
            .push(json!({
                "air_group": group,
                "order": n.value(&order_to_json(&order.kind)),
                "text": n.text(&unit_text(&order.kind)),
                "worked": n.text(&order.reason.worked),
            }));
        if let sim_core::orders::AirGroupOrder::SetSearchArc {
            from_deg,
            to_deg,
            range_nm,
        } = body
        {
            arcs.push(json!({
                "intent": order.intent.raw(),
                "air_group": group,
                "from_deg": from_deg,
                "to_deg": to_deg,
                "whole_circle": from_deg == to_deg,
                "range_nm": dec(*range_nm),
                "how": n.text(&order.reason.worked),
            }));
        }
    }
    let groups: Vec<Value> = by_intent
        .into_iter()
        .map(|(intent, orders)| {
            let mut used: Vec<u64> = orders
                .iter()
                .filter_map(|o| o["air_group"].as_u64())
                .collect();
            used.sort_unstable();
            used.dedup();
            json!({"intent": intent, "air_groups": used, "orders": orders})
        })
        .collect();
    json!({
        "intent": proposal.intent.raw(),
        "kind": found.kind.name(),
        "sub_intents": sub_intents,
        "groups": groups,
        "arcs": arcs,
        "no_orders": proposal
            .decisions
            .no_orders
            .iter()
            .map(|r| n.text(&r.worked))
            .collect::<Vec<_>>(),
        "note": "The tree the staff would form on the coming day if the plan were released; nothing is formed until the day is resolved.",
    })
}

/// One commitment that displaces, in the side's numbers.
fn displacement_json(n: &Names<'_>, d: &Displacement) -> Value {
    let (ships, tons_t) = match &d.committed {
        Committed::Ships(ships) => (Some(n.ships(ships)), None),
        Committed::Tons(t) => (None, Some(dec(*t))),
    };
    json!({
        "intent": d.intent.raw(),
        "plan": d.plan.raw(),
        "run": d.run,
        "ships": ships,
        "tons_t": tons_t,
        "from_day": d.from_day,
        "to_day": d.to_day,
    })
}

/// A commitment in words, in the side's numbers.
fn displacement_text(n: &Names<'_>, d: &Displacement) -> String {
    let what = match &d.committed {
        Committed::Ships(ships) => {
            let numbers: Vec<String> = n.ships(ships).iter().map(u32::to_string).collect();
            format!(
                "{} {}",
                if numbers.len() == 1 { "ship" } else { "ships" },
                numbers.join(", ")
            )
        }
        Committed::Tons(t) => format!("{t} t at the source"),
    };
    format!(
        "run {} of plan {} (intent {}): {what}, days {} to {}",
        d.run,
        d.plan.raw(),
        d.intent.raw(),
        d.from_day,
        d.to_day
    )
}

/// What a proposal is displaced by and what it displaces, in a sentence.
fn displaces_text(n: &Names<'_>, proposal: &Proposal) -> String {
    let by = match proposal.shortfall.as_ref().map(|s| &s.displaced_by) {
        Some(list) if !list.is_empty() => format!(
            "displaced by {}",
            list.iter()
                .map(|d| displacement_text(n, d))
                .collect::<Vec<_>>()
                .join("; ")
        ),
        _ => "displaced by nothing".to_string(),
    };
    let displaces = if proposal.displaces.is_empty() {
        "displaces nothing".to_string()
    } else {
        proposal
            .displaces
            .iter()
            .map(|d| {
                let days = d.shortfall.shortfall_days;
                format!(
                    "displaces intent {}: {days} {} short at the reading of day {}",
                    d.intent.raw(),
                    if days == Fixed::ONE { "day" } else { "days" },
                    d.shortfall.check_day
                )
            })
            .collect::<Vec<_>>()
            .join("; ")
    };
    format!("{by}; {displaces}")
}

fn decisions_json(n: &Names<'_>, decisions: &StaffDecisions) -> Value {
    n.value(&staff_decisions_to_json(decisions))
}

/// `staff_orders`: what the staff would hand in for the coming day.
pub fn staff_orders(view: &SideView<'_>, staff: &dyn Staff) -> Value {
    let n = Names::new(view);
    let today = view.day() + 1;
    let decisions = staff.decide(*view);
    let summary = if decisions.is_empty() {
        "The staff would hand in nothing for the coming day: with no intent in force and a forces limit, it does nothing.".to_string()
    } else {
        format!(
            "The staff would hand in {} orders, {} commands and {} decisions to give no order on day {today}.",
            decisions.orders.len(),
            decisions.commands.len(),
            decisions.no_orders.len()
        )
    };
    let mut body = with_day(decisions_json(&n, &decisions), today);
    body["summary"] = json!(summary);
    answer("staff_orders", body)
}

/// The decisions' JSON with the day they are for, as `for_day`.
fn with_day(mut decisions: Value, day: u32) -> Value {
    decisions["for_day"] = json!(day);
    decisions
}

/// `why`: the staff's reason recorded in event number `number`, whole, with
/// what it is for. Anything that is not one of the side's own staff reasons
/// gets the one `not_known` answer.
pub fn why(view: &SideView<'_>, number: u32) -> Value {
    let events = view.events();
    let unknown = || serde_json::to_value(NotKnown::of("why")).expect("not_known serialises");
    let Some(event) = number.checked_sub(1).and_then(|i| events.get(i as usize)) else {
        return unknown();
    };
    let n = Names::new(view);
    let Some((for_json, reason)) = decision_of(&n, view, &events, event) else {
        return unknown();
    };
    // What the reason serves, in words, and the intents above it, nearest
    // first, the last the commander's (decision 0117, 10).
    let (serves, level, chain) = match &event.kind {
        SideEventKind::Whole(EventKind::StaffReason { reason, .. }) => (
            n.text(&reason.serves.to_string()),
            // The level that decided (decision 0120, 2): the area, or the
            // base, in words; `null` where the reason has none.
            reason.serves.level.map(|level| match level {
                sim_core::command::Addressee::Area => "the area".to_string(),
                sim_core::command::Addressee::Base(base) => n.text(&base.to_string()),
                sim_core::command::Addressee::AirGroup(group) => n.text(&group.to_string()),
                sim_core::command::Addressee::TaskForce(force) => n.text(&force.to_string()),
            }),
            reason
                .serves
                .chain
                .iter()
                .map(|link| {
                    let statement = n.text(&link.statement);
                    let head = format!(
                        "under intent {} (the {}'s)",
                        link.intent.raw(),
                        link.maker.name()
                    );
                    if statement.is_empty() {
                        head
                    } else {
                        format!("{head}: {statement}")
                    }
                })
                .collect::<Vec<_>>(),
        ),
        _ => (String::new(), None, Vec::new()),
    };
    answer(
        "why",
        json!({
            "number": number,
            "day": event.day,
            "pulse": event.pulse.map(|p| p.name()),
            "for": for_json,
            "reason": reason,
            "serves": serves,
            "level": level,
            "chain": chain,
        }),
    )
}

// ---------------------------------------------------------------------------
// The commands a seat queues.
// ---------------------------------------------------------------------------

/// A command as the seat wrote it, read into the engine's own, with the
/// numbers written for ships and forces the side does not own kept so that a
/// refusal can show them.
#[derive(Clone)]
pub(crate) struct Request {
    pub kind: CommandKind,
    /// A stand-in id (above any unit's) to the ship number the side wrote.
    ship_stand_ins: BTreeMap<u32, u32>,
    force_stand_ins: BTreeMap<u32, u32>,
    group_stand_ins: BTreeMap<u32, u32>,
    /// The intent number was the side's next free one, chosen by the tool.
    pub number_chosen: bool,
}

/// The first id given to a number that names no unit of the side's.
const FIRST_STAND_IN: u32 = 4_000_000_000;

const COMMON_KEYS: [&str; 2] = ["clear", "reason"];

fn allowed_keys(tool: &str) -> &'static [&'static str] {
    match tool {
        "set_intent" => &[
            "intent",
            "intent_kind",
            "base",
            "target",
            "hex",
            "distance",
            "task_force",
            "floor_days",
            "interval_days",
        ],
        "withdraw_intent" => &["intent"],
        "set_limit" => &[
            "limit",
            "intent",
            "ships",
            "task_forces",
            "air_groups",
            "source",
            "hexes",
            "rank",
            "base",
            "supply_t",
            "fuel_t",
            "risk",
        ],
        "set_leave" => &["leave", "given"],
        "approve" | "cancel_plan" => &["plan"],
        "hold" | "release" => &["ship", "task_force", "air_group", "setting"],
        "set_interrupt" => &["interrupt", "base", "below_days", "standing", "on"],
        "acknowledge" => &["interrupt", "standing", "plan", "ship"],
        _ => &[],
    }
}

/// Reads a seat's arguments of a command tool into a command. `next_intent`
/// is the side's next free intent number and `next_interrupt` its next free
/// interrupt number.
pub(crate) fn request(
    view: &SideView<'_>,
    tool: &str,
    args: &Map<String, Value>,
    next_intent: u32,
    next_interrupt: u32,
) -> Result<Request, CallError> {
    let allowed = allowed_keys(tool);
    for key in args.keys() {
        if !allowed.contains(&key.as_str()) && !COMMON_KEYS.contains(&key.as_str()) {
            return Err(CallError::naming(
                format!("\"{key}\" is not an argument of {tool}"),
                allowed,
            ));
        }
    }
    let ctx = Ctx::new(*view);
    let mut ship_stand_ins: BTreeMap<u32, u32> = BTreeMap::new();
    let mut force_stand_ins: BTreeMap<u32, u32> = BTreeMap::new();
    let mut group_stand_ins: BTreeMap<u32, u32> = BTreeMap::new();
    let mut made = 0;
    let mut body = Map::new();
    for (key, value) in args {
        if !COMMON_KEYS.contains(&key.as_str()) {
            body.insert(key.clone(), value.clone());
        }
    }
    body.insert("kind".into(), json!(tool));
    if tool == "set_intent" {
        let kind = body.get("intent_kind").and_then(Value::as_str);
        // The keys each kind takes besides "intent" and "intent_kind".
        let takes: &[&str] = match kind {
            Some("cover" | "hold") => &["base"],
            Some("neutralise") => &["target"],
            Some("raid") => &["base", "target"],
            Some("deny") => &["hex", "distance"],
            Some("strike_ships" | "search_waters") => &["base"],
            Some("stand_at" | "operate_carriers") => &["task_force", "hex"],
            _ => &["base", "floor_days", "interval_days"],
        };
        if let (Some(name), Some(key)) = (
            kind.filter(|name| *name != "keep_supplied"),
            [
                "base",
                "target",
                "hex",
                "distance",
                "task_force",
                "floor_days",
                "interval_days",
            ]
            .into_iter()
            .find(|k| body.contains_key(*k) && !takes.contains(k)),
        ) {
            let what = match name {
                "cover" | "hold" => "a base and nothing else",
                "neutralise" => "a target (the enemy base) and nothing else",
                "raid" => "a base (your own) and a target (the enemy base) and nothing else",
                "deny" => "a hex ([q, r]) and a distance (whole hexes) and nothing else",
                "strike_ships" | "search_waters" => "a base and nothing else",
                "stand_at" | "operate_carriers" => {
                    "a task_force (your own force's number) and a hex ([q, r]) and nothing else"
                }
                _ => "no such key",
            };
            return Err(CallError::new(format!(
                "\"{key}\" is not an argument of set_intent for intent_kind {name}: {name} takes {what}"
            )));
        }
        if (kind.is_none() || kind == Some("keep_supplied")) && body.contains_key("target") {
            return Err(CallError::new(
                "\"target\" is an argument of set_intent only for intent_kind neutralise or raid",
            ));
        }
        if (kind.is_none() || kind == Some("keep_supplied")) && body.contains_key("task_force") {
            return Err(CallError::new(
                "\"task_force\" is an argument of set_intent only for intent_kind stand_at or operate_carriers",
            ));
        }
        if (kind.is_none() || kind == Some("keep_supplied"))
            && (body.contains_key("hex") || body.contains_key("distance"))
        {
            return Err(CallError::new(
                "\"hex\" and \"distance\" are arguments of set_intent only for intent_kind deny",
            ));
        }
    }
    let mut number_chosen = false;
    if tool == "set_intent" && !body.contains_key("intent") {
        body.insert("intent".into(), json!(next_intent));
        number_chosen = true;
    }
    if tool == "set_interrupt" {
        let standing = body.contains_key("standing");
        if standing {
            if let Some(key) = ["interrupt", "base", "below_days"]
                .into_iter()
                .find(|k| body.contains_key(*k))
            {
                return Err(CallError::new(format!(
                    "\"{key}\" is not an argument of set_interrupt with \"standing\": a standing interrupt takes \"standing\" and \"on\" only"
                )));
            }
        } else {
            if body.contains_key("on") {
                return Err(CallError::new(
                    "\"on\" is an argument of set_interrupt only with \"standing\" (cannot_meet, waits_for_leave or ship_lost)",
                ));
            }
            if !body.contains_key("interrupt") {
                body.insert("interrupt".into(), json!(next_interrupt));
                number_chosen = true;
            }
        }
    }
    if tool == "set_limit" {
        let limit = body.get("limit").and_then(Value::as_str).unwrap_or("");
        if limit == "forces" {
            body.entry("ships").or_insert_with(|| json!([]));
            body.entry("task_forces").or_insert_with(|| json!([]));
        }
        // A weight written as a whole number is the same weight as its
        // decimal string.
        for key in ["supply_t", "fuel_t"] {
            if let Some(whole) = body.get(key).and_then(Value::as_i64) {
                body.insert(key.into(), json!(whole.to_string()));
            }
        }
        for key in ["ships", "task_forces", "air_groups"] {
            let Some(Value::Array(numbers)) = body.get_mut(key) else {
                continue;
            };
            for item in numbers.iter_mut() {
                let Some(number) = item.as_u64().and_then(|n| u32::try_from(n).ok()) else {
                    continue;
                };
                let (raw, stand_ins) = match key {
                    "task_forces" => (
                        ctx.own_force(number).map(|f| f.id.raw()),
                        &mut force_stand_ins,
                    ),
                    "air_groups" => (
                        ctx.own_group(number).map(|g| g.id.raw()),
                        &mut group_stand_ins,
                    ),
                    _ => (
                        ctx.own_ship(number).map(|s| s.id.raw()),
                        &mut ship_stand_ins,
                    ),
                };
                let id = match raw {
                    Some(raw) => raw,
                    None => match stand_ins.iter().find(|(_, written)| **written == number) {
                        Some((id, _)) => *id,
                        None => {
                            let id = FIRST_STAND_IN + made;
                            made += 1;
                            stand_ins.insert(id, number);
                            id
                        }
                    },
                };
                *item = json!(id);
            }
        }
    }
    // A stand or carrier task names the force by the side's number; a number
    // that names none of its forces becomes one no force has, so the engine
    // refuses an enemy force and one that does not exist alike.
    if tool == "set_intent" {
        if let Some(number) = body
            .get("task_force")
            .and_then(Value::as_u64)
            .and_then(|n| u32::try_from(n).ok())
        {
            let id = match ctx.own_force(number).map(|f| f.id.raw()) {
                Some(raw) => raw,
                None => match force_stand_ins
                    .iter()
                    .find(|(_, written)| **written == number)
                {
                    Some((id, _)) => *id,
                    None => {
                        let id = FIRST_STAND_IN + made;
                        made += 1;
                        force_stand_ins.insert(id, number);
                        id
                    }
                },
            };
            body.insert("task_force".into(), json!(id));
        }
    }
    if tool == "hold" || tool == "release" || tool == "acknowledge" {
        if body.contains_key("setting") && !body.contains_key("air_group") {
            return Err(CallError::new(format!(
                "\"setting\" is an argument of {tool} only with \"air_group\" (the one setting that can be held is \"search_arc\")"
            )));
        }
        // The target is named by the side's number; a number that names none
        // of the side's units becomes one no unit has, so the engine refuses
        // an enemy unit and one that does not exist alike.
        for key in ["ship", "task_force", "air_group"] {
            let Some(number) = body
                .get(key)
                .and_then(Value::as_u64)
                .and_then(|n| u32::try_from(n).ok())
            else {
                continue;
            };
            let (raw, stand_ins) = match key {
                "ship" => (
                    ctx.own_ship(number).map(|s| s.id.raw()),
                    &mut ship_stand_ins,
                ),
                "task_force" => (
                    ctx.own_force(number).map(|f| f.id.raw()),
                    &mut force_stand_ins,
                ),
                _ => (
                    ctx.own_group(number).map(|g| g.id.raw()),
                    &mut group_stand_ins,
                ),
            };
            let id = match raw {
                Some(raw) => raw,
                None => match stand_ins.iter().find(|(_, written)| **written == number) {
                    Some((id, _)) => *id,
                    None => {
                        let id = FIRST_STAND_IN + made;
                        made += 1;
                        stand_ins.insert(id, number);
                        id
                    }
                },
            };
            body.insert(key.into(), json!(id));
        }
    }
    let kind = command_from_json(&Value::Object(body))
        .map_err(|e| CallError::new(format!("malformed command: {e}")))?;
    Ok(Request {
        kind,
        ship_stand_ins,
        force_stand_ins,
        group_stand_ins,
        number_chosen,
    })
}

/// The engine's verdict on the `index`th command of the side taken in on the
/// coming day, from the view of the world that took them in: `Ok` for
/// taken, `Err` for the refusal. `None` if there is no such command.
pub(crate) fn verdict(view: &SideView<'_>, index: usize) -> Option<Result<(), CommandRefusal>> {
    let side = view.side();
    let day = view.day() + 1;
    view.events_on_day(day)
        .into_iter()
        .filter_map(|e| match e.kind {
            SideEventKind::Whole(EventKind::CommandTaken { side: s, .. }) if s == side => {
                Some(Ok(()))
            }
            SideEventKind::Whole(EventKind::CommandRefused {
                side: s, reason, ..
            }) if s == side => Some(Err(reason)),
            _ => None,
        })
        .nth(index)
}

/// The answer to a command a seat queued: queued, or refused with the
/// engine's reason in words. `after` is the view of the world with the
/// command taken in (or refused).
pub(crate) fn command_answer(
    after: &SideView<'_>,
    request: &Request,
    tool: &str,
    verdict: &Result<(), CommandRefusal>,
    queued: usize,
) -> Value {
    let n = Names::new(after);
    let day = after.day() + 1;
    let command = command_json(&n, &request.kind, Some(request));
    let text = command_text(&n, &request.kind, Some(request));
    let mut out = json!({
        "tool": tool,
        "for_day": day,
        "command": command,
        "text": text,
        "queued": queued,
    });
    match verdict {
        Ok(()) => {
            out["status"] = json!("queued");
            let mut notes: Vec<String> = Vec::new();
            if let CommandKind::SetIntent { intent, .. } = &request.kind {
                if request.number_chosen {
                    out["intent"] = json!(intent.raw());
                    notes.push(format!(
                        "You gave no intent number: intent:{} is the side's next free number.",
                        intent.raw()
                    ));
                }
                let state = after.command_state();
                match request.kind {
                    CommandKind::SetIntent {
                        kind: IntentKind::Hold { .. },
                        ..
                    } => notes.push(format!(
                        "intent:{} is an end: the staff forms a cover intent for the base under it by itself. Its forces are the air groups based at the base that are not held, unless you limit them (set_limit forces with air_groups, or hold an air group). command_state shows the tree and the measure of the hold.",
                        intent.raw()
                    )),
                    CommandKind::SetIntent {
                        kind: IntentKind::Neutralise { .. },
                        ..
                    } => notes.push(format!(
                        "intent:{} is an end, addressed to the area: the staff forms a raid for each base of yours with bombers that reach the target, under it. Its forces are the air groups based at your bases that are not held, unless you limit them (set_limit forces with air_groups, or hold an air group). command_state shows the tree and the measure of the intent.",
                        intent.raw()
                    )),
                    CommandKind::SetIntent {
                        kind: IntentKind::Raid { .. },
                        ..
                    } => notes.push(format!(
                        "intent:{} is a raid on the target from your base: its bombers are the air groups based at the base that are not held, unless you limit them (set_limit forces with air_groups).",
                        intent.raw()
                    )),
                    CommandKind::SetIntent {
                        kind: IntentKind::Deny { .. },
                        ..
                    } => notes.push(format!(
                        "intent:{} is an end, addressed to the area: the staff searches the waters and forms a strike_ships intent for each base of yours whose groups reach them, under it. Its forces are the air groups based at your bases that are not held, unless you limit them (set_limit forces with air_groups, or hold an air group). command_state shows the tree and the measure of the intent.",
                        intent.raw()
                    )),
                    CommandKind::SetIntent {
                        kind: IntentKind::StrikeShips { .. },
                        ..
                    } => notes.push(format!(
                        "intent:{} gives the base the task of striking ships: its groups strike the ships they reach. Its forces are the air groups based at the base that are not held, unless you limit them (set_limit forces with air_groups).",
                        intent.raw()
                    )),
                    CommandKind::SetIntent {
                        kind: IntentKind::Cover { .. },
                        ..
                    } => notes.push(format!(
                        "intent:{} covers the base from the air: its fighters fly the patrol and its search groups search. Its forces are the air groups based at the base that are not held, unless you limit them (set_limit forces with air_groups).",
                        intent.raw()
                    )),
                    _ => {
                        if state.forces_on_chain(*intent).is_empty() {
                            notes.push(format!(
                                "intent:{} has no forces limit yet: the staff gives no order for it until you set one (set_limit forces).",
                                intent.raw()
                            ));
                        }
                    }
                }
            }
            if matches!(request.kind, CommandKind::Approve { .. }) {
                notes.push(
                    "approve prepares a draft plan, or releases a plan that is preparing."
                        .to_string(),
                );
            }
            match &request.kind {
                CommandKind::SetInterrupt(InterruptSetting::Set { interrupt, .. }) => {
                    if request.number_chosen {
                        out["interrupt"] = json!(interrupt.raw());
                        notes.push(format!(
                            "You gave no interrupt number: interrupt:{} is the side's next free number.",
                            interrupt.raw()
                        ));
                    }
                    notes.push(
                        "A set interrupt is tested each morning, after the staff has handed in: it fires once when its base's days of supply are below the number (and a morning that reads no figure does not fire it), and is armed again on a morning the figure is back at or above it. run stops the clock at the end of a day on which it fired."
                            .to_string(),
                    );
                }
                CommandKind::SetInterrupt(InterruptSetting::Standing { kind, on }) => notes.push(format!(
                    "The standing interrupt {} is {} from the start of day {day}.",
                    kind.name(),
                    if *on { "on" } else { "off" }
                )),
                CommandKind::Acknowledge { .. } => notes.push(
                    "Acknowledging marks the last firing seen. It does not arm the interrupt again: that happens on a morning its condition does not hold."
                        .to_string(),
                ),
                CommandKind::Hold { target } => notes.push(format!(
                    "{} is held from the start of day {day}: the staff gives no order to it and plans without it. release lifts the hold.",
                    hold_target_words(&n, *target)
                )),
                CommandKind::Release { target } => notes.push(format!(
                    "{} is released from the start of day {day}: it is free to the staff again from that morning.",
                    hold_target_words(&n, *target)
                )),
                _ => {}
            }
            out["notes"] = json!(notes);
            out["summary"] = json!(format!(
                "Queued for the start of day {day}. {queued} command{} queued for day {day}.",
                if queued == 1 { " is" } else { "s are" }
            ));
        }
        Err(reason) => {
            out["status"] = json!("refused");
            out["reason"] = json!(n.text_with(
                &reason.to_string(),
                &request.ship_stand_ins,
                &request.force_stand_ins,
                &request.group_stand_ins
            ));
            out["notes"] = json!([]);
            out["summary"] = json!(format!(
                "Refused, and not queued. {queued} command{} queued for day {day}.",
                if queued == 1 { " is" } else { "s are" }
            ));
        }
    }
    answer("command", out)
}

/// The intent numbers ever set, the commands in `queued` counted: the side's
/// next free number is one past the highest.
pub(crate) fn next_free_intent(view: &SideView<'_>, queued: &[CommandKind]) -> u32 {
    let set = view
        .command_state()
        .intents
        .keys()
        .map(|id| id.raw())
        .max()
        .unwrap_or(0);
    let queued = queued
        .iter()
        .filter_map(|c| match c {
            CommandKind::SetIntent { intent, .. } => Some(intent.raw()),
            _ => None,
        })
        .max()
        .unwrap_or(0);
    set.max(queued) + 1
}

/// The interrupt numbers ever set, the commands in `queued` counted: the
/// side's next free number is one past the highest.
pub(crate) fn next_free_interrupt(view: &SideView<'_>, queued: &[CommandKind]) -> u32 {
    let set = view
        .command_state()
        .interrupts
        .keys()
        .map(|id| id.raw())
        .max()
        .unwrap_or(0);
    let queued = queued
        .iter()
        .filter_map(|c| match c {
            CommandKind::SetInterrupt(InterruptSetting::Set { interrupt, .. }) => {
                Some(interrupt.raw())
            }
            _ => None,
        })
        .max()
        .unwrap_or(0);
    set.max(queued) + 1
}

// ---------------------------------------------------------------------------
// Preview (decision 0107, step 3, rulings 25 to 28).
// ---------------------------------------------------------------------------

/// What a change that was previewed leaves of the plan's runs in hand
/// (ruling 28), written for a preview that makes no promise it cannot keep.
const MADE_READY_LOST: &str = "nothing: in a supply plan stock aboard a ship stays aboard and is carried by its next run, and stock landed stays where it lies";

/// The note a preview ends with.
const PREVIEW_NOTE: &str = "Nothing is changed by a preview. approve with {\"preview\": <n>} queues exactly this command. The preview is void once anything is queued or a day is resolved.";

/// The words for the commands in `commands`, in the side's numbers.
pub(crate) fn commands_text(view: &SideView<'_>, commands: &[CommandKind]) -> Vec<String> {
    let n = Names::new(view);
    commands
        .iter()
        .map(|command| command_text(&n, command, None))
        .collect()
}

/// The intent a previewed command is about: the intent set, or the intent
/// the limit is for. A reserve is for a base, not an intent: it is read as
/// about the lowest intent in force whose forces limit loads at that base.
fn previewed_intent(view: &SideView<'_>, kind: &CommandKind) -> Option<IntentId> {
    match kind {
        CommandKind::SetIntent { intent, .. } => Some(*intent),
        CommandKind::SetLimit(Limit::Reserve { base, .. }) => view
            .command_state()
            .intents_in_force()
            .find(|i| {
                i.limits
                    .forces
                    .as_ref()
                    .is_some_and(|f| f.source == Some(*base))
            })
            .map(|i| i.id),
        CommandKind::SetLimit(limit) => limit.intent(),
        _ => None,
    }
}

/// The staff's decisions that serve `intent`; all of them where none.
fn decisions_for(decisions: &StaffDecisions, intent: Option<IntentId>) -> StaffDecisions {
    let Some(intent) = intent else {
        return decisions.clone();
    };
    StaffDecisions {
        orders: decisions
            .orders
            .iter()
            .filter(|o| o.intent == intent)
            .cloned()
            .collect(),
        commands: decisions
            .commands
            .iter()
            .filter(|c| c.reason.serves.intent == Some(intent))
            .cloned()
            .collect(),
        no_orders: decisions
            .no_orders
            .iter()
            .filter(|r| r.serves.intent == Some(intent))
            .cloned()
            .collect(),
        // A notice points into the whole hand-in by index; the filtered
        // decisions of a preview carry none.
        notices: Vec::new(),
    }
}

fn binds_json(n: &Names<'_>, proposal: &Proposal) -> Value {
    proposal.shortfall.as_ref().map_or(Value::Null, |s| {
        json!({
            "binds": n.text(&s.binds),
            "shortfall_days": dec(s.shortfall_days),
            "source_dry_day": s.source_dry_day,
            "check_day": s.check_day,
            "displaced_by": s.displaced_by.iter().map(|d| displacement_json(n, d)).collect::<Vec<_>>(),
        })
    })
}

/// What the staff would do for the intent on the coming day on one view: its
/// decisions (orders, commands, decisions to give no order), the runs of its
/// proposal, the next of them, the first day below the floor and what binds.
fn staff_side_json(
    n: &Names<'_>,
    staff: &dyn Staff,
    view: &SideView<'_>,
    intent: Option<IntentId>,
) -> (Value, StaffDecisions, Option<Proposal>) {
    let decisions = decisions_for(&staff.decide(*view), intent);
    let proposal = intent.and_then(|i| staff.propose(*view, i));
    let shown = decisions_json(n, &decisions);
    let runs: Vec<Value> = proposal
        .iter()
        .flat_map(|p| p.runs.iter())
        .map(|run| {
            json!({
                "ships": n.ships(&run.ships),
                "load_t": dec(run.load_t),
                "days": days_json(&run.days),
            })
        })
        .collect();
    let json = json!({
        "orders": shown["orders"],
        "commands": shown["commands"],
        "no_orders": shown["no_orders"],
        "next_run": runs.first().cloned(),
        "runs": runs,
        "first_day_below": proposal
            .as_ref()
            .and_then(|p| p.projection.as_ref())
            .and_then(|p| p.first_day_below),
        "binds": proposal.as_ref().map_or(Value::Null, |p| binds_json(n, p)),
    });
    (json, decisions, proposal)
}

/// The holds that keep a ship of the intent's pool from the staff (ruling
/// 18), in words.
fn held_words(n: &Names<'_>, view: &SideView<'_>, intent: Option<IntentId>) -> Vec<String> {
    let state = view.command_state();
    let Some(forces) = intent.and_then(|i| state.nearest_forces(i)) else {
        return Vec::new();
    };
    let mut pool: std::collections::BTreeSet<ShipId> = forces.ships.clone();
    for force in &forces.task_forces {
        if let Some(force) = n.ctx.forces.get(force) {
            pool.extend(force.ships.iter().copied());
        }
    }
    let mut by_target: BTreeMap<HoldTarget, Vec<ShipId>> = BTreeMap::new();
    for ship in pool {
        let in_force = match n.ctx.ships.get(&ship).map(|s| s.location) {
            Some(ShipLocation::TaskForce(force)) => {
                n.ctx.forces.get(&force).map(|f| (force, f.ships.clone()))
            }
            _ => None,
        };
        let target = state.hold_on_ship(ship, in_force.as_ref().map(|(f, ships)| (*f, ships)));
        if let Some(target) = target {
            by_target.entry(target).or_default().push(ship);
        }
    }
    let mut groups: Vec<String> = Vec::new();
    for group in &forces.air_groups {
        if let Some(target) = state.hold_on_group(*group) {
            if let Some(hold) = state.holds.get(&target) {
                groups.push(format!(
                    "{}, one of the groups the intent may use, is held by {}: the staff gives it no order",
                    hold_target_words(n, target),
                    hold_maker_words(n, hold)
                ));
            }
        }
    }
    groups
        .into_iter()
        .chain(by_target.into_iter().filter_map(|(target, ships)| {
            let hold = state.holds.get(&target)?;
            let maker = hold_maker_words(n, hold);
            let numbers = |own: Option<ShipId>| -> String {
                ships
                    .iter()
                    .filter(|s| Some(**s) != own)
                    .map(|s| n.ship(*s).to_string())
                    .collect::<Vec<_>>()
                    .join(", ")
            };
            let own = match target {
                HoldTarget::Ship(ship) => Some(ship),
                _ => None,
            };
            let others = numbers(own);
            let mut text = format!("{} is held by {maker}", hold_target_words(n, target));
            if !others.is_empty() {
                text.push_str(&format!(
                    ", so ship {others} of the pool {} not free either",
                    if others.contains(',') { "are" } else { "is" }
                ));
            }
            Some(text)
        }))
        .collect()
}

/// For each run of the intent's plan that has not finished, what the change
/// does to it (ruling 28): a run that has sailed goes on to the base it was
/// sent to; a run still loading is sent to the new base; a change that is
/// not of the base leaves every run as ordered.
fn runs_in_hand(view: &SideView<'_>, intent: Option<IntentId>, kind: &CommandKind) -> Vec<String> {
    let state = view.command_state();
    let Some(intent) = intent.and_then(|i| state.intents.get(&i)) else {
        return Vec::new();
    };
    let Some(plan) = state.plans.get(&intent.plan) else {
        return Vec::new();
    };
    let IntentKind::KeepSupplied { base: old, .. } = intent.kind else {
        return Vec::new();
    };
    let new_base = match kind {
        CommandKind::SetIntent {
            kind: IntentKind::KeepSupplied { base, .. },
            ..
        } if *base != old => Some(*base),
        _ => None,
    };
    let today = view.day() + 1;
    plan.runs
        .values()
        .filter(|run| run.ended.is_none())
        .filter_map(|run| {
            let (phase, _) = run_phase(run, today);
            let number = run.number;
            Some(match (phase, new_base) {
                ("finished", _) => return None,
                ("under_way", Some(_)) => format!(
                    "run {number} has sailed and goes on to base {}",
                    old.raw()
                ),
                ("returning", Some(_)) => format!(
                    "run {number} has delivered at base {} and is coming home; the change does not touch it",
                    old.raw()
                ),
                ("loading" | "waiting", Some(base)) => format!(
                    "run {number} is {} and is sent to base {}",
                    if phase == "loading" { "loading" } else { "not yet loading" },
                    base.raw()
                ),
                (phase, None) => format!(
                    "run {number} is {} and goes on as ordered",
                    match phase {
                        "waiting" => "not yet loading",
                        "loading" => "loading",
                        "under_way" => "under way",
                        _ => "coming home",
                    }
                ),
                _ => return None,
            })
        })
        .collect()
}

/// What other intents give up with the change: each other intent in force
/// the with-change proposal newly displaces, and each whose decisions for
/// the coming day differ between the two.
fn other_plans_give_up(
    staff: &dyn Staff,
    as_is: &SideView<'_>,
    with: &SideView<'_>,
    intent: Option<IntentId>,
    before: Option<&Proposal>,
    after: Option<&Proposal>,
) -> Vec<String> {
    let mut words = Vec::new();
    if let Some(after) = after {
        for d in &after.displaces {
            let already = before.is_some_and(|b| b.displaces.iter().any(|o| o.intent == d.intent));
            if already {
                continue;
            }
            let days = d.shortfall.shortfall_days;
            words.push(format!(
                "intent {} (plan {}) would be {days} {} short at the reading of day {}",
                d.intent.raw(),
                d.plan.raw(),
                if days == Fixed::ONE { "day" } else { "days" },
                d.shortfall.check_day
            ));
        }
    }
    let day = with.day() + 1;
    let (all_before, all_after) = (staff.decide(*as_is), staff.decide(*with));
    for other in with.command_state().intents_in_force() {
        if Some(other.id) == intent {
            continue;
        }
        let (a, b) = (
            decisions_for(&all_before, Some(other.id)),
            decisions_for(&all_after, Some(other.id)),
        );
        if a != b {
            if words
                .iter()
                .any(|w| w.starts_with(&format!("intent {} ", other.id.raw())))
            {
                continue;
            }
            words.push(format!(
                "intent {}: what the staff would do for it on day {day} differs with the change ({} orders, {} commands and {} decisions to give no order as things stand; {}, {} and {} with it)",
                other.id.raw(),
                a.orders.len(),
                a.commands.len(),
                a.no_orders.len(),
                b.orders.len(),
                b.commands.len(),
                b.no_orders.len()
            ));
        }
    }
    words
}

/// `preview`: what a changed intent or limit would do, worked on two copies
/// of the world as the coming day will find it: `as_is` with the side's
/// queued commands, and `with` the same and the command queued last. It is
/// stored nowhere in the game. `number` is the preview's number in the
/// session.
pub(crate) fn preview(
    number: u32,
    as_is: &SideView<'_>,
    with: &SideView<'_>,
    staff: &dyn Staff,
    request: &Request,
) -> Value {
    let n = Names::new(with);
    let intent = previewed_intent(with, &request.kind);
    let (now_json, _, now_proposal) = staff_side_json(&Names::new(as_is), staff, as_is, intent);
    let (with_json, _, with_proposal) = staff_side_json(&n, staff, with, intent);
    answer(
        "preview",
        json!({
            "preview": number,
            "day": with.day() + 1,
            "command": command_json(&n, &request.kind, Some(request)),
            "shown": command_text(&n, &request.kind, Some(request)),
            "intent": intent.map(|i| i.raw()),
            "as_things_stand": now_json,
            "with_change": with_json,
            "held": held_words(&n, with, intent),
            "runs_in_hand": runs_in_hand(as_is, intent, &request.kind),
            "made_ready_lost": MADE_READY_LOST,
            "other_plans_give_up": other_plans_give_up(
                staff,
                as_is,
                with,
                intent,
                now_proposal.as_ref(),
                with_proposal.as_ref(),
            ),
            "valid": true,
            "note": PREVIEW_NOTE,
        }),
    )
}

/// The refusal of a command that was not taken in as the tool read it: the
/// shape of a refused `command` answer, with words of the caller's own (a
/// void preview).
pub(crate) fn refused_answer(
    view: &SideView<'_>,
    request: &Request,
    tool: &str,
    reason: String,
    queued: usize,
) -> Value {
    let n = Names::new(view);
    let day = view.day() + 1;
    answer(
        "command",
        json!({
            "tool": tool,
            "for_day": day,
            "command": command_json(&n, &request.kind, Some(request)),
            "text": command_text(&n, &request.kind, Some(request)),
            "queued": queued,
            "status": "refused",
            "reason": reason,
            "notes": [],
            "summary": format!(
                "Refused, and not queued. {queued} command{} queued for day {day}.",
                if queued == 1 { " is" } else { "s are" }
            ),
        }),
    )
}
