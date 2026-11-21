//! A task force as an addressee (decision 0124, rules 6 to 8, and decision
//! 0125, rule 2), and the doctrine for standing at a hex by night, at the
//! force's level (decision 0124, rules 9 to 15).
//!
//! **Forming, at the level above.** Under a deny or a hold intent whose
//! forces limit names task forces, the doctrine of that level forms, for
//! each named force in task force number order where none for that force is
//! in force under it: a stand task at the end's hex (the centre of the
//! waters, or the base's hex) for a force on the surface combat mission with
//! a ship afloat; a carrier task at the force's own hex for a force on the
//! carrier mission with a carrier afloat with an air group aboard. With no
//! task force named, nothing is formed.
//!
//! **The stand doctrine.** Each morning, for a stand task whose force is not
//! held: the threat (a contact of ships within 15 hexes of the station; at
//! preserve the force, one that reports no more ships than the force has
//! afloat), whether it sorties (press every night, otherwise on a threat),
//! fuel first (the passage out and home by the engine's own measure, times
//! the risk's margin), the sortie (from its home port, at cruise where that
//! reaches the station in the night's 12 hours, else at full speed, the
//! station marked to stand at), readiness, and rules of engagement set once
//! from the risk.
//!
//! The pure parts are [`margin`], [`break_off`] and [`threat`]; the rest
//! reads the side's view, and the engine's measures through it.

use std::collections::BTreeSet;

use sim_core::command::{
    CommandKind, Intent, IntentKind, IntentLimits, Maker, NoticeBasis, NoticeKind, Plan, Reason,
    Risk, StaffCommand, StaffNotice, StaffOrder,
};
use sim_core::fixed::Fixed;
use sim_core::hex::Hex;
use sim_core::ids::{BaseId, TaskForceId};
use sim_core::orders::{Mission, OrderKind, Readiness, SpeedSetting, Stance, TaskForceOrder};
use sim_core::refdata::ShipKind;
use sim_core::state::{
    AirGroupLocation, RulesOfEngagement, ShipLocation, TaskForce, TaskForceStatus,
};
use sim_core::time::Pulse;
use sim_core::view::SideView;
use sim_core::PassageEnd;

use crate::air::Day;
use crate::reasons::{base_words, serving};

// ---------------------------------------------------------------------------
// The risk table (decision 0124) and the threat.
// ---------------------------------------------------------------------------

/// The hexes from the station within which a contact of ships is a threat
/// (rule 9). A placeholder: a day's steaming at 25 knots.
pub const THREAT_HEXES: u32 = 15;

/// The hours of the night pulse: a sortie must reach the station in them
/// (rule 12).
pub const NIGHT_HOURS: i64 = 12;

/// The fuel a sortie needs, as a share of the passage out and home (the
/// table): one and a half times, one and a fifth, once.
pub fn margin(risk: Risk) -> Fixed {
    match risk {
        Risk::Preserve => Fixed::from_ratio(3, 2),
        Risk::Balanced => Fixed::from_ratio(6, 5),
        Risk::Press => Fixed::ONE,
    }
}

/// The break-off level of the rules of engagement (the table): a quarter, a
/// half, three quarters.
pub fn break_off(risk: Risk) -> Fixed {
    match risk {
        Risk::Preserve => Fixed::from_ratio(1, 4),
        Risk::Balanced => Fixed::from_ratio(1, 2),
        Risk::Press => Fixed::from_ratio(3, 4),
    }
}

/// One contact of ships as the threat reads it: its number in the side's
/// series, its hexes from the station and the ships it reports.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Sighted {
    pub number: u32,
    pub hexes: u32,
    pub ships: u32,
}

/// The threat (rule 9): the nearest contact within [`THREAT_HEXES`] of the
/// station that counts, the lowest number on a tie. At preserve the force a
/// contact counts only where it reports no more ships than the force has
/// `afloat`.
pub fn threat(risk: Risk, afloat: u32, contacts: &[Sighted]) -> Option<Sighted> {
    contacts
        .iter()
        .filter(|c| c.hexes <= THREAT_HEXES)
        .filter(|c| risk != Risk::Preserve || c.ships <= afloat)
        .min_by_key(|c| (c.hexes, c.number))
        .copied()
}

/// The nearest contact within [`THREAT_HEXES`] of the station that reports
/// more ships than the force has `afloat`, the lowest number on a tie: at
/// preserve the force, a force with no home port does not go to its station
/// while one stands there (decision 0192, 11).
pub fn outnumbering(afloat: u32, contacts: &[Sighted]) -> Option<Sighted> {
    contacts
        .iter()
        .filter(|c| c.hexes <= THREAT_HEXES && c.ships > afloat)
        .min_by_key(|c| (c.hexes, c.number))
        .copied()
}

/// The speed a force with no home port goes to its station at (decision
/// 0192, 2): cruise where the engine's measure at cruise (`cruise`, the hours
/// where it arrives) brings it there within the night's [`NIGHT_HOURS`], else
/// full, whether or not it arrives at full: a force that would run dry is
/// then short of fuel where its fuel is weighed (rule 5), so that more fuel
/// aboard never stops it going. `None` where ships cannot get from where it
/// is to the station at all (`reachable` false).
pub fn speed_from_sea(cruise: Option<Fixed>, reachable: bool) -> Option<SpeedSetting> {
    match cruise {
        Some(h) if h <= Fixed::from_int(NIGHT_HOURS) => Some(SpeedSetting::Cruise),
        _ if reachable => Some(SpeedSetting::Full),
        _ => None,
    }
}

/// The day from which a force that fuels where it lies may be used (rule
/// 11): the day after it has fuelled, by `hours` of fuelling from the start
/// of `today`.
pub fn usable_day(today: u32, hours: Fixed) -> u32 {
    let day = Fixed::from_int(24);
    let mut whole = (hours / day).trunc();
    if Fixed::from_int(whole) * day < hours {
        whole += 1;
    }
    let days = u32::try_from(whole.max(1)).unwrap_or(u32::MAX);
    today.saturating_add(days)
}

fn risk_words(risk: Risk) -> String {
    format!("{} risk", risk.name())
}

fn pulse_words(pulse: Pulse) -> &'static str {
    match pulse {
        Pulse::Night => "night",
        Pulse::Morning => "morning",
        Pulse::Afternoon => "afternoon",
    }
}

// ---------------------------------------------------------------------------
// What the force's level reads of a force.
// ---------------------------------------------------------------------------

/// The ships of `force` afloat, in ship order, each with its fuel aboard,
/// its capacity and its kind.
pub(crate) fn afloat(
    view: &SideView<'_>,
    force: TaskForceId,
) -> Vec<(sim_core::ids::ShipId, Fixed, Fixed, ShipKind)> {
    let mut found: Vec<_> = view
        .ships()
        .filter(|ship| ship.location == ShipLocation::TaskForce(force))
        .filter_map(|ship| {
            let class = view.ship_class(ship.class)?;
            Some((ship.id, ship.fuel_t, class.fuel_capacity_t, class.kind))
        })
        .collect();
    found.sort_by_key(|(id, _, _, _)| *id);
    found
}

/// Whether the force is stopped: not steaming and not halted.
pub(crate) fn stopped(force: &TaskForce) -> bool {
    !matches!(
        force.status,
        TaskForceStatus::Underway | TaskForceStatus::Halted
    ) && force.toward_hex.is_none()
}

/// The side's own base in `hex`, the lowest number first.
pub(crate) fn own_base_at(view: &SideView<'_>, hex: Hex) -> Option<BaseId> {
    let mut bases: Vec<_> = view
        .bases()
        .filter(|b| b.hex == hex)
        .map(|b| b.id)
        .collect();
    bases.sort();
    bases.first().copied()
}

/// The force's home port as the staff knows it: the one an order set, or
/// else the base it lies at, stopped. The second is `true` where it has none
/// set and an order must set it.
pub(crate) fn home_of(view: &SideView<'_>, force: &TaskForce) -> Option<(BaseId, bool)> {
    match force.home_port {
        Some(base) => Some((base, false)),
        None if stopped(force) => own_base_at(view, force.position()).map(|b| (b, true)),
        None => None,
    }
}

/// Whether the force's route is done: no waypoint left to steam to.
pub(crate) fn route_done(force: &TaskForce) -> bool {
    force.route.next_index >= force.route.waypoints.len()
}

/// Where a force under way is bound: the last waypoint of its route, or the
/// hex it steams toward where it has no waypoint left. `None` for a force
/// that is stopped.
pub(crate) fn bound_for(force: &TaskForce) -> Option<Hex> {
    if stopped(force) {
        return None;
    }
    if route_done(force) {
        force.toward_hex
    } else {
        force.route.waypoints.last().copied()
    }
}

/// Whether the engine would fuel a ship at the base in `hex` (decision 0192,
/// 14, "gives a rate"): its port has a row, and the rate it fuels a ship at
/// (at a berth where it has one, else by lighter) is given and above 0, or
/// not given, which is fuelling with no limit (chapter 2, Port size), as
/// [`fuelling_hours`] reads it.
pub(crate) fn fuels_at(view: &SideView<'_>, hex: Hex) -> bool {
    let Some(size) = view.hex(hex).map(|h| h.port_size) else {
        return false;
    };
    let Some(port) = view.port_size(size) else {
        return false;
    };
    let rate = if port.berths > 0 {
        port.fuel_berth_rate_t_per_h
    } else {
        port.fuel_lighter_rate_t_per_h
    };
    rate.is_none_or(|r| r > Fixed::ZERO)
}

/// The base a force with no home port weighs its fuel to, and goes in to
/// when short (decision 0192, 14): the nearest base of the side by the
/// engine's sea route from `from` that holds fuel above its reserve and
/// fuels a ship, the lower number on a tie. With its hex and the miles the
/// engine charges to it.
pub(crate) fn fuel_base(view: &SideView<'_>, from: Hex) -> Option<(BaseId, Hex, u32)> {
    view.bases()
        .filter(|b| b.fuel_t > b.fuel_reserve_t && fuels_at(view, b.hex))
        .filter_map(|b| {
            let nm = if b.hex == from {
                0
            } else {
                crate::route::route_between(view, from, b.hex, None)?.out_nm
            };
            Some((nm, b.id, b.hex))
        })
        .min()
        .map(|(nm, id, hex)| (id, hex, nm))
}

/// One ship's fuel against a passage: aboard, the engine's measure of the
/// burn times the margin, and whether the engine has it run dry short of the
/// passage, when the burn it gives is only what is aboard (the architect's
/// ruling on decision 0130, Nov 16, 2026: a ship that runs dry short of a
/// passage is short of the fuel for it).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) struct ShipFuel {
    pub(crate) ship: sim_core::ids::ShipId,
    pub(crate) aboard: Fixed,
    pub(crate) need: Fixed,
    pub(crate) runs_dry: bool,
}

impl ShipFuel {
    /// Less aboard than the need, or run dry short of the passage.
    pub(crate) fn short(self) -> bool {
        self.runs_dry || self.aboard < self.need
    }

    /// "ship:N has A t aboard against B t needed", and for a ship that runs
    /// dry "against more than B t needed, running dry short of the passage".
    pub(crate) fn words(self) -> String {
        if self.runs_dry {
            format!(
                "{} has {} t aboard against more than {} t needed, running dry short of the passage",
                self.ship, self.aboard, self.need
            )
        } else {
            format!(
                "{} has {} t aboard against {} t needed",
                self.ship, self.aboard, self.need
            )
        }
    }
}

/// Each ship's fuel aboard and the fuel it needs, in ship order.
pub(crate) type Needs = Vec<ShipFuel>;

/// The words of each ship's fuel against its need, joined.
pub(crate) fn needs_words(needs: &[ShipFuel]) -> String {
    needs
        .iter()
        .map(|n| n.words())
        .collect::<Vec<_>>()
        .join(", ")
}

/// The fuel each ship of the force needs for a passage through `waypoints`
/// at `speed`, by the engine's own measure, times `margin`; `None` where the
/// engine would refuse the route. With the hours to the end of the passage.
pub(crate) fn fuel_needs(
    view: &SideView<'_>,
    force: TaskForceId,
    waypoints: &[Hex],
    speed: Option<SpeedSetting>,
    margin: Fixed,
) -> Option<(Needs, Fixed)> {
    let forecast = view.forecast_route(force, waypoints, speed)?;
    if forecast.refusal.is_some() {
        return None;
    }
    let needs = forecast
        .ships
        .iter()
        .map(|line| ShipFuel {
            ship: line.ship,
            aboard: line.fuel_t,
            need: line.burned_t * margin,
            runs_dry: line.runs_dry,
        })
        .collect();
    Some((needs, forecast.hours))
}

/// The hours a passage to `hex` at `speed` takes by the engine's own
/// measure, where it arrives.
pub(crate) fn hours_to(
    view: &SideView<'_>,
    force: TaskForceId,
    hex: Hex,
    speed: SpeedSetting,
) -> Option<Fixed> {
    let forecast = view.forecast_route(force, &[hex], Some(speed))?;
    match forecast.end {
        PassageEnd::Arrives { .. } => Some(forecast.hours),
        _ => None,
    }
}

/// Where a force away from home is expected home: the day and pulse of its
/// arrival at `home` by the engine's own measure, in words.
pub(crate) fn expected_words(view: &SideView<'_>, force: TaskForceId, home: Hex) -> String {
    match view.forecast_passage(force, home, None).map(|f| f.end) {
        Some(PassageEnd::Arrives { day, pulse, .. }) => {
            format!(
                "it is expected there in the {} of day {day}",
                pulse_words(pulse)
            )
        }
        _ => "it is not expected there by the engine's measure".to_string(),
    }
}

/// What the engine's fuelling rule would take at `base` for `ships` short of
/// their need (decision 0112): `None` where the base cannot fuel them (no
/// port that fuels, or less fuel in its stock than they lack of their need);
/// else the hours it takes to fill them, the longest ship's, by the port's
/// berth rate where it has a berth and else its lighter rate, no hours where
/// no rate is given.
pub(crate) fn fuelling_hours(
    view: &SideView<'_>,
    base: BaseId,
    short: &[(Fixed, Fixed, Fixed)],
) -> Option<Fixed> {
    let found = view.bases().find(|b| b.id == base)?;
    let lacking = short.iter().fold(Fixed::ZERO, |sum, (aboard, need, _)| {
        sum + (*need - *aboard).max(Fixed::ZERO)
    });
    if found.fuel_t < lacking || found.fuel_t <= Fixed::ZERO {
        return None;
    }
    let size = view.hex(found.hex)?.port_size;
    let port = view.port_size(size)?;
    let rate = if port.berths > 0 {
        port.fuel_berth_rate_t_per_h
    } else {
        port.fuel_lighter_rate_t_per_h
    };
    let Some(rate) = rate else {
        return Some(Fixed::ZERO);
    };
    if rate <= Fixed::ZERO {
        return None;
    }
    Some(
        short
            .iter()
            .fold(Fixed::ZERO, |most, (aboard, _, capacity)| {
                most.max((*capacity - *aboard).max(Fixed::ZERO) / rate)
            }),
    )
}

// ---------------------------------------------------------------------------
// The morning.
// ---------------------------------------------------------------------------

/// What one morning's working of a stand task hands in.
struct Out {
    orders: Vec<(TaskForceOrder, Reason)>,
    /// The reason of the morning, where it gives no route.
    summary: Option<Reason>,
    /// The intent cannot be met: the notice rests on the summary.
    cannot: bool,
}

/// What the morning has read of a stand task whose force has no home port,
/// for the branch that stands it from where it is (decision 0192).
struct FromSea<'r> {
    station: Hex,
    risk: Risk,
    threat_text: &'r str,
    /// Whether it would sortie by the test of rule 10: its readiness.
    sorties: bool,
    /// At preserve the force, the contact that outnumbers it, in words.
    outnumbered: Option<&'r str>,
    /// The home port it has that is no longer the side's.
    lost_home: Option<BaseId>,
}

impl Day<'_, '_> {
    /// The forming rule (decisions 0124, 8, and 0125, 2): under `end`, a deny
    /// or a hold intent, a stand task at `station` for each surface force its
    /// forces limit names and a carrier task for each carrier force, in task
    /// force number order, where no task of that kind for the force is in force,
    /// whoever made it (the architect's ruling of Nov 16, 2026).
    /// Whether any such task was formed or stands.
    pub(crate) fn form_force_tasks(
        &mut self,
        end: &Intent,
        plan: &Plan,
        station: Hex,
        in_scope: bool,
        surface_only: bool,
    ) -> bool {
        let named: BTreeSet<TaskForceId> = self
            .state
            .nearest_forces(end.id)
            .map(|f| f.task_forces.clone())
            .unwrap_or_default();
        let today = self.today;
        let mut any = false;
        for id in named {
            let Some(force) = self.area.task_forces().find(|f| f.id == id) else {
                continue;
            };
            // A force before its day is no force a task may use (decision
            // 0140, 3): no task is formed for it.
            if crate::available::force_waiting(self.area, id).is_some() {
                continue;
            }
            let ships = afloat(self.area, id);
            // The force as the area command holds it, and the moment that is
            // of where it is not the pulse just ended (decision 0136, 11).
            let held = crate::reasons::force_held_words(self.area, id);
            let (kind, words, why) = match force.mission {
                Mission::SurfaceCombat if !ships.is_empty() => (
                    IntentKind::StandAt {
                        force: id,
                        hex: station,
                    },
                    format!("stand at {station} with {id}"),
                    format!(
                        "{held} is on the surface combat mission with {} ship(s) afloat",
                        ships.len()
                    ),
                ),
                Mission::Carrier if !surface_only => {
                    let carriers: Vec<_> = ships
                        .iter()
                        .filter(|(_, _, _, kind)| *kind == ShipKind::Carrier)
                        .map(|(ship, _, _, _)| *ship)
                        .collect();
                    let aboard = self
                        .area
                        .air_groups()
                        .filter(|g| matches!(g.location, AirGroupLocation::Ship(s) if carriers.contains(&s)))
                        .count();
                    if aboard == 0 {
                        continue;
                    }
                    let hex = force.position();
                    (
                        IntentKind::OperateCarriers { force: id, hex },
                        format!("operate the carrier force {id} from {hex}"),
                        format!(
                            "{held} is on the carrier mission with {} carrier(s) afloat and {aboard} air group(s) aboard, and lies at {hex}",
                            carriers.len()
                        ),
                    )
                }
                _ => continue,
            };
            any = true;
            // One task of a kind a force, whoever made it and wherever it
            // stands in the side's trees (the architect's ruling of Nov 16,
            // 2026, amending decisions 0124, 8, and 0125, 2).
            let standing = self
                .state
                .intents_in_force()
                .find(|other| other.kind.force() == Some(id) && other.kind.code() == kind.code())
                .map(|other| (other.id, other.kind, other.parent, other.maker));
            if let Some((task, task_kind, parent, maker)) = standing {
                let aim = crate::reasons::aim(
                    self.view,
                    &Intent {
                        kind: task_kind,
                        ..end.clone()
                    },
                );
                let reason = if parent == Some(end.id) {
                    // A task under this end: the reason it always gave.
                    serving(end, plan)
                        .evidence(why.clone(), today)
                        .evidence(
                            format!("{task}, a task of {id}, is in force under {}", end.id),
                            today,
                        )
                        .binding(format!("{task} stands"))
                        .worked(format!(
                            "{task} ({aim}) stands under {}: the staff forms no other for {id}. {why}.",
                            end.id
                        ))
                        .would_change(format!("{task} withdrawn"))
                        .done()
                } else {
                    let by = match maker {
                        Maker::Commander => "the commander's",
                        Maker::Staff => "the staff's own",
                    };
                    let place = match parent {
                        Some(p) => format!("under {p}"),
                        None => "as a root".to_string(),
                    };
                    serving(end, plan)
                        .evidence(why.clone(), today)
                        .evidence(format!("{task}, {by} task of {id}, is in force {place}"), today)
                        .binding(format!("{task} stands"))
                        .worked(format!(
                            "{task} ({aim}), {by}, stands {place}: a force has one task of a kind, so the staff forms no other for {id}. {why}."
                        ))
                        .would_change(format!("{task} withdrawn"))
                        .done()
                };
                self.target(in_scope).no_orders.push(reason);
                continue;
            }
            let id_new = self.state.next_intent_number();
            self.state.intents.insert(
                id_new,
                Intent {
                    id: id_new,
                    kind,
                    parent: Some(end.id),
                    maker: Maker::Staff,
                    addressee: kind.addressee(),
                    limits: IntentLimits::default(),
                    set_day: today,
                    plan: end.plan,
                    withdrawn_day: None,
                },
            );
            let reason = serving(end, plan)
                .evidence(why.clone(), today)
                .binding(format!("{id} is named in the forces limit of {}", end.id))
                .worked(format!(
                    "{id} is named in the forces limit of {}, its plan is released and no task of {id} stands under it: {why}, so the staff forms {id_new}, to {words}, under it, for {id} to carry out.",
                    end.id
                ))
                .would_change(format!("{id} not named in the forces limit"))
                .would_change("such a task already standing")
                .done();
            self.target(in_scope).commands.push(StaffCommand {
                kind: CommandKind::FormIntent {
                    parent: end.id,
                    kind,
                },
                reason,
            });
            if in_scope {
                self.lines.push(format!(
                    "{} would form {id_new}, to {words}, under it ({why}).",
                    end.id
                ));
            }
        }
        any
    }

    /// Covering forces (decision 0126, 7): under each keep-supplied intent
    /// whose forces limit names task forces, a stand task at the
    /// destination's hex for each named force on the surface combat mission,
    /// by the forming rule of decision 0124, 8. The supply doctrine approves
    /// and releases the intent's plan this morning where the leaves allow
    /// (decision 0107, 2); the air doctrine's planned state takes it so, so
    /// that the task it forms is served this morning too.
    pub(crate) fn supply_covers(&mut self) {
        let ends = self.in_priority_order(|i| {
            matches!(i.kind, IntentKind::KeepSupplied { .. })
                && i.limits
                    .forces
                    .as_ref()
                    .is_some_and(|f| !f.task_forces.is_empty())
        });
        let leaves = self.state.leaves;
        for id in ends {
            let Some(end) = self.state.intents.get(&id).cloned() else {
                continue;
            };
            let IntentKind::KeepSupplied { base, .. } = end.kind else {
                continue;
            };
            let Some(plan) = self.state.plans.get_mut(&end.plan) else {
                continue;
            };
            let released = match plan.state {
                sim_core::command::PlanState::Released => true,
                sim_core::command::PlanState::Draft => leaves.prepare && leaves.launch,
                sim_core::command::PlanState::Preparing => leaves.launch,
                _ => false,
            };
            if !released {
                continue;
            }
            plan.state = sim_core::command::PlanState::Released;
            let plan = plan.clone();
            let Some(hex) = self.area.bases().find(|b| b.id == base).map(|b| b.hex) else {
                continue;
            };
            let in_scope = self.in_scope(id);
            self.form_force_tasks(&end, &plan, hex, in_scope, true);
        }
    }

    /// The stand doctrine for the morning (decision 0124, 9 to 15).
    pub(crate) fn stands(&mut self) {
        let tasks = self.in_priority_order(|i| matches!(i.kind, IntentKind::StandAt { .. }));
        for id in tasks {
            let Some(task) = self.state.intents.get(&id).cloned() else {
                continue;
            };
            if !self.released(&task) {
                continue;
            }
            self.serve_stand(&task);
        }
    }

    #[allow(clippy::too_many_lines)]
    fn serve_stand(&mut self, task: &Intent) {
        let Some(plan) = self.plan_of(task) else {
            return;
        };
        let IntentKind::StandAt {
            force: id,
            hex: station,
        } = task.kind
        else {
            return;
        };
        let in_scope = self.in_scope(task.id);
        let today = self.today;
        let view = self.view;
        let risk = self.state.risk_read_up(task.id);
        let risk_text = risk_words(risk);
        let intent_words = format!("{}, standing at {station} with {id}", task.id);
        let mut out = Out {
            orders: Vec::new(),
            summary: None,
            cannot: false,
        };
        let force = view.task_forces().find(|f| f.id == id).cloned();
        let Some(force) = force else {
            if let Some(day) =
                crate::available::entering_on(view, sim_core::ids::EntityRef::TaskForce(id))
            {
                // Held apart until its day (decision 0195, 13): one reason, no
                // order, and the intent is neither failed nor finished.
                let words = crate::available::enters_words(id, day);
                let reason = serving(task, &plan)
                    .evidence(words, today)
                    .binding(format!("the day {id} enters, {day}"))
                    .worked(format!(
                        "{intent_words}: {} is to be served by {id}, which enters on day {day}, so the staff gives {id} no order.",
                        task.id
                    ))
                    .would_change("the day come")
                    .done();
                out.summary = Some(reason);
                self.hand_in_stand(task, &plan, id, out, in_scope, &intent_words);
                return;
            }
            let reason = serving(task, &plan)
                .evidence(format!("{id} is not one of the side's forces now"), today)
                .worked(format!(
                    "{intent_words}: {id} is gone, so the staff gives no order and cannot meet {}.",
                    task.id
                ))
                .would_change("the force back")
                .done();
            out.summary = Some(reason);
            out.cannot = true;
            self.hand_in_stand(task, &plan, id, out, in_scope, &intent_words);
            return;
        };
        if let Some(day) = crate::available::force_waiting(view, id) {
            // Before its day the force is no force a task may use (decision
            // 0140, 3): one reason, no order.
            let words = crate::available::from_day_words(day);
            let reason = serving(task, &plan)
                .evidence(format!("{id} {words}"), today)
                .binding(format!("the day {id} may be used from, {day}"))
                .worked(format!(
                    "{intent_words}: {} is to be served by {id}, which {words}, so the staff gives {id} no order.",
                    task.id
                ))
                .would_change("the day come")
                .done();
            out.summary = Some(reason);
            self.hand_in_stand(task, &plan, id, out, in_scope, &intent_words);
            return;
        }
        if let Some(hold) = self.state.hold_on_force(id, &force.ships) {
            let reason = serving(task, &plan)
                .evidence(format!("{hold} is held by the commander"), today)
                .binding(format!("the hold on {hold}"))
                .worked(format!("{intent_words}: {hold} is held by the commander, so the staff gives {id} no order."))
                .would_change("the hold released")
                .done();
            out.summary = Some(reason);
            self.hand_in_stand(task, &plan, id, out, in_scope, &intent_words);
            return;
        }
        let ships = afloat(view, id);
        if ships.is_empty() {
            let reason = serving(task, &plan)
                .evidence(format!("{id} has no ship afloat"), today)
                .worked(format!("{intent_words}: {id} has no ship afloat, so the staff gives no order and cannot meet {}.", task.id))
                .would_change("a ship afloat in the force")
                .done();
            out.summary = Some(reason);
            out.cannot = true;
            self.hand_in_stand(task, &plan, id, out, in_scope, &intent_words);
            return;
        }
        let here = force.position();

        // Radio silence at preserve the force (decision 0136, 18).
        if let Some(order) = silence_order(view, task, &plan, id, risk, today) {
            out.orders.push(order);
        }

        // The threat (rule 9), read before the home port: a force with no
        // home port reads it too (decision 0192). Reading it gives no order.
        let afloat_count = u32::try_from(ships.len()).unwrap_or(u32::MAX);
        // The force's own picture: the command the task is addressed to
        // (decision 0134, 13).
        let picture = crate::picture::picture_of(view, task.kind.addressee());
        let mut sighted: Vec<Sighted> = picture
            .contacts()
            .map(|c| Sighted {
                number: picture.contact_number(c.id),
                hexes: c.estimated_hex.distance(station),
                ships: c.estimated_ship_count,
            })
            .collect();
        sighted.sort_by_key(|c| (c.hexes, c.number));
        let found = threat(risk, afloat_count, &sighted);
        let threat_text = match found {
            Some(c) => format!(
                "{} of {} ship(s) is {} hexes from the station {station}, within {THREAT_HEXES}: a threat",
                crate::picture::contact_words(&picture, c.number),
                c.ships,
                c.hexes
            ),
            None => {
                let near = sighted.iter().find(|c| c.hexes <= THREAT_HEXES);
                match near {
                    Some(c) if risk == Risk::Preserve => format!(
                        "{} is {} hexes from the station {station} but reports {} ship(s) against the {afloat_count} {id} has afloat: at preserve the force it is no threat that counts, so there is no threat",
                        crate::picture::contact_words(&picture, c.number),
                        c.hexes,
                        c.ships
                    ),
                    _ => format!(
                        "the side holds no contact of ships within {THREAT_HEXES} hexes of the station {station}: there is no threat"
                    ),
                }
            }
        };
        let sorties = risk == Risk::Press || found.is_some();
        let sortie_rule = match risk {
            Risk::Press => "at press the force sorties every night",
            Risk::Balanced => "at balanced the force sorties on a night with a threat",
            Risk::Preserve => {
                "at preserve the force it sorties on a threat it is not outnumbered by"
            }
        };
        // At preserve the force, the nearest contact that outnumbers it within
        // the threat's hexes (decision 0192, 11): a force with no home port
        // keeps clear of it.
        let outnumbered = (risk == Risk::Preserve)
            .then(|| outnumbering(afloat_count, &sighted))
            .flatten()
            .map(|c| {
                format!(
                    "{} of {} ship(s) is {} hexes from the station {station}, within {THREAT_HEXES}, and outnumbers the {afloat_count} {id} has afloat",
                    crate::picture::contact_words(&picture, c.number),
                    c.ships,
                    c.hexes
                )
            });

        // Under the order to fuel at sea it is given no route while the
        // order stands (decision 0201, 20).
        if let Some(giver) = crate::fuelling::fuelling_from(&force) {
            out.summary = Some(
                serving(task, &plan)
                    .evidence(format!("{id} is under the order to fuel from {giver} at sea"), today)
                    .binding("a force fuelling at sea is given no route while the order stands")
                    .worked(format!("{intent_words}: {id} fuels from {giver} at sea, so it is given no route; when the order ends it goes back to its task."))
                    .would_change("the order ended")
                    .done(),
            );
            self.hand_in_stand(task, &plan, id, out, in_scope, &intent_words);
            return;
        }

        // The home port (rule 12): the base it lies at when it has none, set
        // by order. A home port that is no longer the side's is treated as
        // none (decision 0192, 16).
        let lost_home = force
            .home_port
            .filter(|port| !view.bases().any(|b| b.id == *port));
        let home = match force.home_port {
            Some(port) if lost_home.is_none() => Some((port, false)),
            _ if stopped(&force) => own_base_at(view, here).map(|b| (b, true)),
            _ => None,
        };
        let Some((home, set_home)) = home else {
            // No home port and not stopped at a base of the side: it stands
            // from where it is (decision 0192).
            let read = FromSea {
                station,
                risk,
                threat_text: &threat_text,
                sorties,
                outnumbered: outnumbered.as_deref(),
                lost_home,
            };
            self.serve_from_sea(task, &plan, &force, &read, out, in_scope, &intent_words);
            return;
        };
        let home_text = base_words(view, home);
        let Some(home_hex) = view.bases().find(|b| b.id == home).map(|b| b.hex) else {
            return;
        };
        if set_home {
            let mut reason = serving(task, &plan);
            if let Some(lost) = lost_home {
                reason = reason.evidence(
                    format!("{id}'s home port {lost} is no longer its side's"),
                    today,
                );
            }
            out.orders.push((
                TaskForceOrder::SetHomePort(home),
                reason
                    .evidence(format!("{id} has no home port and lies stopped at {home_text}"), today)
                    .binding("the force's home port is the base it lies at when the task is formed")
                    .worked(format!("{id} has no home port and lies stopped at {home_text}: its home port is set to {home_text}, where it sorties from and comes back to."))
                    .would_change("a home port set by the commander")
                    .done(),
            ));
        }

        // Away from home (rule 14).
        if !(stopped(&force) && here == home_hex) {
            // Stopped at sea on no route, it goes home at cruise (decision
            // 0192, 16).
            if stopped(&force) && route_done(&force) && own_base_at(view, here).is_none() {
                // Short of fuel by the measure of a sortie from here, with an
                // oiler's force nearer than home: it fuels from her in place
                // of going home (decision 0201, 20).
                let needs = fuel_needs(
                    view,
                    id,
                    &[station, home_hex],
                    Some(SpeedSetting::Cruise),
                    margin(risk),
                );
                let short = needs
                    .as_ref()
                    .is_some_and(|(n, _)| n.iter().any(|n| n.short()));
                let home_nm = view.sea_charged_nm(here, home_hex);
                let oiler = crate::fuelling::oiler_for(view, &force)
                    .filter(|o| home_nm.is_some_and(|h| o.nm < h));
                if let (true, Some(oiler), Some((needs, _))) = (short, oiler, needs.as_ref()) {
                    let fuel_text = format!(
                        "the passage to the station and home, at cruise by the engine's measure, times the {} of the {risk_text}: {}",
                        margin(risk),
                        needs_words(needs)
                    );
                    let passages = format!(
                        "the passage to {} at {} is {} nm, and home to {home_text} {} nm",
                        oiler.giver,
                        oiler.hex,
                        oiler.nm,
                        home_nm.unwrap_or(0)
                    );
                    out.orders.push((
                        TaskForceOrder::FuelFrom(oiler.giver),
                        serving(task, &plan)
                            .evidence(format!("{id} lies stopped at {here}, at sea on no route, away from its home port {home_text}"), today)
                            .evidence(fuel_text.clone(), today)
                            .evidence(format!("{} holds {} t of cargo fuel in ships that fuel others at sea", oiler.giver, oiler.cargo_fuel_t), today)
                            .evidence(passages.clone(), today)
                            .binding("an oiler's force nearer than home is a place to fuel")
                            .worked(format!(
                                "{intent_words}: {id} lies stopped at sea away from its home port, and {fuel_text}: a ship has less than it needs; {passages}, so {id} is given the order to fuel from {} at sea in place of going home.",
                                oiler.giver
                            ))
                            .would_change("more fuel aboard, or the oiler's force farther than home or empty")
                            .done(),
                    ));
                    self.hand_in_stand(task, &plan, id, out, in_scope, &intent_words);
                    return;
                }
                // Fuelled at an oiler since the staff was last asked, and not
                // short: it goes back to its station by the sortie's own rule
                // (decision 0201, 20, on the architect's ruling of Nov 21,
                // 2026). Short, or not fuelled at sea, it goes home.
                let fuelled_from = (!short)
                    .then(|| crate::fuelling::fuel_order_ended(view, id))
                    .flatten();
                if let (Some(giver), Some((needs, _))) = (fuelled_from, needs.as_ref()) {
                    let cruise = hours_to(view, id, station, SpeedSetting::Cruise);
                    let night = Fixed::from_int(NIGHT_HOURS);
                    let (speed, speed_text) = match cruise {
                        Some(h) if h <= night => (SpeedSetting::Cruise, "cruise"),
                        _ => (SpeedSetting::Full, "full speed"),
                    };
                    let fuel_text = format!(
                        "the passage to the station and home, at cruise by the engine's measure, times the {} of the {risk_text}: {}",
                        margin(risk),
                        needs_words(needs)
                    );
                    let evidence = format!("{id} fuelled at sea from {giver}, and its order to fuel ended; it lies stopped at {here}, at sea on no route");
                    if force.speed != speed {
                        out.orders.push((
                            TaskForceOrder::SetSpeed(speed),
                            serving(task, &plan)
                                .evidence(evidence.clone(), today)
                                .binding(format!("cruise where it reaches the station within the night's {NIGHT_HOURS} hours, else full speed"))
                                .worked(format!("{id} goes back to its station {station}: it is set to {speed_text}."))
                                .would_change("a station nearer, or farther")
                                .done(),
                        ));
                    }
                    out.orders.push((
                        TaskForceOrder::SetRoute {
                            waypoints: vec![station, home_hex],
                            act_at: BTreeSet::new(),
                            stand_at: BTreeSet::from([station]),
                        },
                        serving(task, &plan)
                            .evidence(evidence, today)
                            .evidence(fuel_text.clone(), today)
                            .binding("a force that has fuelled at an oiler's force goes back to its station")
                            .worked(format!(
                                "{intent_words}: {id} has fuelled from {giver} at sea and lies stopped at {here}; {fuel_text}: it has the fuel. It goes back to its station {station} at {speed_text}, the station marked to stand at, and home to {home_text} from there."
                            ))
                            .would_change("less fuel aboard, or the force at its home port")
                            .done(),
                    ));
                    self.hand_in_stand(task, &plan, id, out, in_scope, &intent_words);
                    return;
                }
                let forecast = view
                    .forecast_passage(id, home_hex, Some(SpeedSetting::Cruise))
                    .map(|f| f.end);
                let when = match forecast {
                    Some(PassageEnd::Arrives { day, pulse, .. }) => format!(
                        "at cruise it is expected there in the {} of day {day}",
                        pulse_words(pulse)
                    ),
                    _ => "it is not expected there by the engine's measure".to_string(),
                };
                if force.speed != SpeedSetting::Cruise {
                    out.orders.push((
                        TaskForceOrder::SetSpeed(SpeedSetting::Cruise),
                        serving(task, &plan)
                            .evidence(format!("{id} lies stopped at {here}, at sea on no route, away from its home port {home_text}"), today)
                            .binding("a force stopped at sea goes home at cruise")
                            .worked(format!("{id} lies stopped at sea away from its home port {home_text}: it is set to cruise to go home."))
                            .would_change("the force at its home port")
                            .done(),
                    ));
                }
                out.orders.push((
                    TaskForceOrder::SetRoute {
                        waypoints: vec![home_hex],
                        act_at: BTreeSet::new(),
                        stand_at: BTreeSet::new(),
                    },
                    serving(task, &plan)
                        .evidence(format!("{id} lies stopped at {here}, at sea on no route, away from its home port {home_text}"), today)
                        .binding("a force stopped at sea goes home at cruise")
                        .worked(format!(
                            "{intent_words}: {id} lies stopped at {here}, at sea on no route, and is not at its home port {home_text}, so it would lie there for ever: it is routed home to {home_text} at cruise, and {when}. From there it stands as a force at its home port."
                        ))
                        .would_change("the force at its home port, or under way")
                        .done(),
                ));
                self.hand_in_stand(task, &plan, id, out, in_scope, &intent_words);
                return;
            }
            let reason = serving(task, &plan)
                .evidence(format!("{id} is at {here}, not stopped at its home port {home_text}"), today)
                .binding("the force sorties only from its home port")
                .worked(format!(
                    "{intent_words}: {id} is at {here}, not stopped at its home port {home_text}; {}. The staff gives it no order.",
                    expected_words(view, id, home_hex)
                ))
                .would_change("the force stopped at its home port")
                .done();
            out.summary = Some(reason);
            self.hand_in_stand(task, &plan, id, out, in_scope, &intent_words);
            return;
        }

        // Rules of engagement, set once from the risk (rule 13).
        if let Some(order) = roe_order(task, &plan, &force, risk, today) {
            out.orders.push(order);
        }

        let mut battle = false;
        let mut route: Option<(TaskForceOrder, Reason)> = None;
        let mut speed_order: Option<(TaskForceOrder, Reason)> = None;
        if station == here {
            // Its own hex: no route (rule 12).
            battle = sorties;
            out.summary = Some(
                serving(task, &plan)
                    .evidence(threat_text.clone(), today)
                    .binding(sortie_rule)
                    .worked(format!(
                        "{intent_words}: the station is the hex {id} lies at, so it is given no route; {threat_text}, so it is set to {} readiness.",
                        if battle { "battle" } else { "normal" }
                    ))
                    .would_change("a contact near the station, or none")
                    .done(),
            );
        } else if !sorties {
            out.summary = Some(
                serving(task, &plan)
                    .evidence(threat_text.clone(), today)
                    .binding(sortie_rule)
                    .worked(format!(
                        "{intent_words}: {threat_text}, and {sortie_rule}: {id} does not sortie tonight."
                    ))
                    .would_change("a contact of ships near the station")
                    .done(),
            );
        } else {
            // The speed (rule 12).
            let cruise = hours_to(view, id, station, SpeedSetting::Cruise);
            let full = hours_to(view, id, station, SpeedSetting::Full);
            let night = Fixed::from_int(NIGHT_HOURS);
            let chosen = match (cruise, full) {
                (Some(h), _) if h <= night => Some((SpeedSetting::Cruise, h)),
                (_, Some(h)) if h <= night => Some((SpeedSetting::Full, h)),
                _ => None,
            };
            let hours_text = format!(
                "the station is {} hours away at cruise and {} at full speed",
                cruise.map_or_else(|| "no".to_string(), |h| h.to_string()),
                full.map_or_else(|| "no".to_string(), |h| h.to_string())
            );
            match chosen {
                None => {
                    out.cannot = true;
                    out.summary = Some(
                        serving(task, &plan)
                            .evidence(threat_text.clone(), today)
                            .evidence(hours_text.clone(), today)
                            .binding(format!("a sortie must reach the station within the night's {NIGHT_HOURS} hours"))
                            .worked(format!(
                                "{intent_words}: {threat_text}, but {hours_text}: not even full speed brings {id} to the station within the night's {NIGHT_HOURS} hours, so there is no sortie and the staff cannot meet {}.",
                                task.id
                            ))
                            .would_change("a station nearer the force's home port")
                            .done(),
                    );
                }
                Some((speed, hours)) => {
                    // Fuel first (rule 11).
                    let needs =
                        fuel_needs(view, id, &[station, home_hex], Some(speed), margin(risk));
                    let Some((needs, _)) = needs else {
                        return;
                    };
                    let short: Vec<ShipFuel> =
                        needs.iter().copied().filter(|n| n.short()).collect();
                    let fuel_text = needs_words(&needs);
                    let fuel_text = format!(
                        "the passage to the station and home, by the engine's measure, times the {} of the {risk_text}: {fuel_text}",
                        margin(risk)
                    );
                    // Ammunition, read with the fuel in one pass (decision
                    // 0127, 5).
                    let ammunition = crate::ammunition::short(view, id, risk);
                    if short.is_empty() && ammunition.is_none() {
                        battle = true;
                        let speed_text = match speed {
                            SpeedSetting::Full => "full speed",
                            _ => "cruise",
                        };
                        if force.speed != speed {
                            speed_order = Some((
                                TaskForceOrder::SetSpeed(speed),
                                serving(task, &plan)
                                    .evidence(hours_text.clone(), today)
                                    .binding(format!("a sortie must reach the station within the night's {NIGHT_HOURS} hours"))
                                    .worked(format!("{hours_text}: {id} is set to {speed_text}, the slowest that reaches the station within the night."))
                                    .would_change("a station nearer, or farther")
                                    .done(),
                            ));
                        }
                        let stand_at: BTreeSet<Hex> = BTreeSet::from([station]);
                        route = Some((
                            TaskForceOrder::SetRoute {
                                waypoints: vec![station, home_hex],
                                act_at: BTreeSet::new(),
                                stand_at,
                            },
                            serving(task, &plan)
                                .evidence(threat_text.clone(), today)
                                .evidence(fuel_text.clone(), today)
                                .evidence(format!("{hours} hours to the station at {speed_text}"), today)
                                .binding(sortie_rule)
                                .worked(format!(
                                    "{intent_words}: {threat_text}, and {sortie_rule}. {fuel_text}: it has the fuel. It is routed to the station {station} and home to {home_text}, the station marked to stand at: {hours} hours to the station at {speed_text}, it stands there for the night and steams for home at morning."
                                ))
                                .would_change("no threat, a bolder or more careful risk, or less fuel aboard")
                                .done(),
                        ));
                    } else if short.is_empty() {
                        // Short of ammunition alone (decision 0127, 2 to 4).
                        let ammunition = ammunition.expect("short of ammunition");
                        let when_text = match ammunition.hours.map(|h| usable_day(today, h)) {
                            Some(day) => {
                                format!("it rearms at {home_text} and may be used from day {day}")
                            }
                            None => {
                                out.cannot = true;
                                format!(
                                    "{home_text} cannot rearm it, so there is no day from which it may be used, and the staff cannot meet {}",
                                    task.id
                                )
                            }
                        };
                        out.summary = Some(
                            serving(task, &plan)
                                .evidence(threat_text.clone(), today)
                                .evidence(fuel_text.clone(), today)
                                .evidence(ammunition.words.clone(), today)
                                .binding("a force short of ammunition does not sortie")
                                .worked(format!(
                                    "{intent_words}: {threat_text}, and {sortie_rule}. {fuel_text}: it has the fuel. But {}, and it does not sortie; {when_text}.",
                                    ammunition.words
                                ))
                                .would_change("more ammunition aboard")
                                .done(),
                        );
                    } else {
                        let triples: Vec<(Fixed, Fixed, Fixed)> = short
                            .iter()
                            .map(|n| {
                                let capacity = ships
                                    .iter()
                                    .find(|(s, _, _, _)| *s == n.ship)
                                    .map_or(n.need, |(_, _, c, _)| *c);
                                (n.aboard, n.need, capacity)
                            })
                            .collect();
                        let when =
                            fuelling_hours(view, home, &triples).map(|h| usable_day(today, h));
                        // The later of fuel and ammunition (decision 0127, 5).
                        let ammunition_day = ammunition
                            .as_ref()
                            .map(|a| a.hours.map(|h| usable_day(today, h)));
                        let when = match ammunition_day {
                            None => when,
                            Some(day) => when.zip(day).map(|(a, b)| a.max(b)),
                        };
                        let ammunition_words = ammunition
                            .as_ref()
                            .map_or_else(String::new, |a| format!("; and {}", a.words));
                        let when_text = match when {
                            Some(day) if ammunition.is_some() => format!(
                                "it fuels and rearms at {home_text} and may be used from day {day}"
                            ),
                            Some(day) => {
                                format!("it fuels at {home_text} and may be used from day {day}")
                            }
                            None => {
                                out.cannot = true;
                                format!(
                                    "{home_text} cannot fuel it, so there is no day from which it may be used, and the staff cannot meet {}",
                                    task.id
                                )
                            }
                        };
                        out.summary = Some(
                            serving(task, &plan)
                                .evidence(threat_text.clone(), today)
                                .evidence(fuel_text.clone(), today)
                                .binding("fuel comes first: a force with less aboard than the sortie needs does not sortie")
                                .worked(format!(
                                    "{intent_words}: {threat_text}, and {sortie_rule}, but {fuel_text}: it has less than it needs{ammunition_words}; it does not sortie; {when_text}."
                                ))
                                .would_change("more fuel aboard")
                                .done(),
                        );
                    }
                }
            }
        }

        // Readiness (rule 13).
        let wanted = if battle {
            Readiness::Battle
        } else {
            Readiness::Normal
        };
        if force.readiness != wanted {
            out.orders.push((
                TaskForceOrder::SetReadiness(wanted),
                serving(task, &plan)
                    .evidence(threat_text.clone(), today)
                    .binding("battle readiness on a night it sorties or stands at its own hex with a threat, normal on one it does not")
                    .worked(format!(
                        "{intent_words}: {threat_text}; {}, so {id} is set to {} readiness.",
                        if battle { "it sorties or stands" } else { "it does not sortie" },
                        if battle { "battle" } else { "normal" }
                    ))
                    .would_change("a threat near the station, or none")
                    .done(),
            ));
        }
        if let Some(order) = speed_order {
            out.orders.push(order);
        }
        if let Some(order) = route {
            out.orders.push(order);
        }
        self.hand_in_stand(task, &plan, id, out, in_scope, &intent_words);
    }

    /// A stand task's force with no home port, not stopped at a base of the
    /// side, stands from where it is (decision 0192, rules 1 to 9 as amended
    /// on review by 11 to 16): it goes to its station, one way, on the first
    /// morning it is not there and not under way to it, and lies stopped
    /// there; its fuel is weighed to the nearest base that can fuel it, and
    /// it goes in to that base, which becomes its home port, when a ship is
    /// short of the passage there at cruise.
    #[allow(clippy::too_many_arguments, clippy::too_many_lines)]
    fn serve_from_sea(
        &mut self,
        task: &Intent,
        plan: &Plan,
        force: &TaskForce,
        read: &FromSea<'_>,
        mut out: Out,
        in_scope: bool,
        intent_words: &str,
    ) {
        let view = self.view;
        let today = self.today;
        let id = force.id;
        let here = force.position();
        let station = read.station;
        let risk = read.risk;
        let risk_text = risk_words(risk);
        let threat_text = read.threat_text;
        let at_station = here == station;
        let under_way_to = !at_station && bound_for(force) == Some(station);
        let lead = match read.lost_home {
            Some(lost) => format!(
                "{id}'s home port {lost} is no longer its side's, so it has no home port and stands from where it is"
            ),
            None => format!("{id} has no home port and stands from where it is"),
        };
        let lies = if stopped(force) {
            format!("{id} has no home port and lies stopped at {here}, at no base of the side")
        } else {
            format!("{id} has no home port and is under way at {here}")
        };
        let lies = match read.lost_home {
            Some(lost) => format!("{lies}; its home port {lost} is no longer its side's"),
            None => lies,
        };

        // The warning where nothing can be done: a ship below a quarter and
        // no oiler's force holding cargo fuel (decision 0201, 19).
        let below = crate::fuelling::below_a_quarter(view, id);
        if !below.is_empty() && crate::fuelling::oiler_for(view, force).is_none() {
            let warning =
                crate::fuelling::warning(serving(task, plan), force, &below, intent_words, today);
            self.target(in_scope).no_orders.push(warning);
        }

        // Preserve the force keeps clear of what outnumbers it (11).
        if let (Some(outnumbered), false) = (read.outnumbered, under_way_to) {
            let what = if at_station {
                "it lies at its station and is given nothing"
            } else {
                "it does not go to its station"
            };
            out.summary = Some(
                serving(task, plan)
                    .evidence(lies.clone(), today)
                    .evidence(outnumbered.to_string(), today)
                    .binding("at preserve the force, a force with no home port does not go to its station, and is given nothing there, while a contact that outnumbers it is within 15 hexes of the station")
                    .worked(format!("{intent_words}: {lead}; but {outnumbered}, so at preserve the force {what}."))
                    .would_change("the contact farther or gone, fewer ships in it, or a bolder risk")
                    .done(),
            );
            self.hand_in_stand(task, plan, id, out, in_scope, intent_words);
            return;
        }

        // The base its fuel is weighed to (14), and ammunition in the same
        // pass (15).
        let base = fuel_base(view, station).filter(|(_, hex, _)| *hex != here);
        let ammunition = crate::ammunition::short(view, id, risk);
        let no_base_text = "no base of the side holds fuel above its reserve and can fuel it, so its fuel is not weighed";

        let mut home_order: Option<(TaskForceOrder, Reason)> = None;
        let mut speed_order: Option<(TaskForceOrder, Reason)> = None;
        let mut route: Option<(TaskForceOrder, Reason)> = None;
        if under_way_to {
            // Under way, it is left alone (13).
            out.summary = Some(
                serving(task, plan)
                    .evidence(lies.clone(), today)
                    .binding("a force with no home port under way to its station is given no route")
                    .worked(format!("{intent_words}: {lead}; it is under way to its station {station}, so it is given no route."))
                    .would_change("the force at its station, or bound elsewhere")
                    .done(),
            );
        } else if at_station {
            // At the station it stays, or goes in for fuel (6, 14, 15).
            let going_in = base.and_then(|(b, hex, nm)| {
                let (needs, _) =
                    fuel_needs(view, id, &[hex], Some(SpeedSetting::Cruise), margin(risk))?;
                let short: Vec<ShipFuel> = needs.iter().copied().filter(|n| n.short()).collect();
                if short.is_empty() && ammunition.is_none() {
                    return None;
                }
                Some((b, hex, nm, needs, short))
            });
            // An oiler's force nearer than the base it would go in to, or,
            // with a ship below a quarter, any (decision 0201, 20).
            let oiler = crate::fuelling::oiler_for(view, force).filter(|o| match going_in {
                Some((_, _, nm, _, ref short)) => !short.is_empty() && o.nm < nm,
                None => !below.is_empty(),
            });
            if let Some(oiler) = oiler {
                let why = match going_in {
                    Some((b, _, nm, ref needs, _)) => format!(
                        "it would go in to {} for fuel, {nm} nm, short of the fuel it needs: {}; {} lies {} nm away at {}",
                        base_words(view, b),
                        needs_words(needs),
                        oiler.giver,
                        oiler.nm,
                        oiler.hex
                    ),
                    None => format!(
                        "{}: below a quarter of capacity; {} lies {} nm away at {}",
                        crate::fuelling::below_words(&below),
                        oiler.giver,
                        oiler.nm,
                        oiler.hex
                    ),
                };
                out.orders.push((
                    TaskForceOrder::FuelFrom(oiler.giver),
                    serving(task, plan)
                        .evidence(lies.clone(), today)
                        .evidence(format!("{} holds {} t of cargo fuel in ships that fuel others at sea", oiler.giver, oiler.cargo_fuel_t), today)
                        .binding("an oiler's force is a place to fuel")
                        .worked(format!(
                            "{intent_words}: {lead}. It lies at its station {station}; {why}, so it is given the order to fuel from {} at sea.",
                            oiler.giver
                        ))
                        .would_change("more fuel aboard, or the oiler's force farther or empty")
                        .done(),
                ));
                self.hand_in_stand(task, plan, id, out, in_scope, intent_words);
                return;
            }
            match going_in {
                Some((b, hex, nm, needs, short)) => {
                    let base_text = base_words(view, b);
                    let fuel_text = format!(
                        "the passage from the station to {base_text}, {nm} nm, at cruise by the engine's measure, times the {} of the {risk_text}: {}",
                        margin(risk),
                        needs_words(&needs)
                    );
                    let why = match (short.is_empty(), &ammunition) {
                        (false, Some(a)) => {
                            format!(
                                "short of the fuel it needs: {}; and {}",
                                needs_words(&short),
                                a.words
                            )
                        }
                        (false, None) => {
                            format!("short of the fuel it needs: {}", needs_words(&short))
                        }
                        (true, Some(a)) => a.words.clone(),
                        (true, None) => String::new(),
                    };
                    home_order = Some((
                        TaskForceOrder::SetHomePort(b),
                        serving(task, plan)
                            .evidence(lies.clone(), today)
                            .evidence(fuel_text.clone(), today)
                            .binding("a force with no home port that goes in for fuel or ammunition takes the base it goes to as its home port")
                            .worked(format!("{id} goes in to {base_text}: its home port is set to {base_text}, and from there it stands as a force with a home port."))
                            .would_change("a home port set by the commander")
                            .done(),
                    ));
                    if force.speed != SpeedSetting::Cruise {
                        speed_order = Some((
                            TaskForceOrder::SetSpeed(SpeedSetting::Cruise),
                            serving(task, plan)
                                .evidence(fuel_text.clone(), today)
                                .binding("a force going in for fuel goes at cruise")
                                .worked(format!(
                                    "{id} goes in to {base_text} at cruise: it is set to cruise."
                                ))
                                .would_change("the ships' fuel and ammunition enough")
                                .done(),
                        ));
                    }
                    route = Some((
                        TaskForceOrder::SetRoute {
                            waypoints: vec![hex],
                            act_at: BTreeSet::new(),
                            stand_at: BTreeSet::new(),
                        },
                        serving(task, plan)
                            .evidence(lies.clone(), today)
                            .evidence(fuel_text.clone(), today)
                            .binding("a force at its station with a ship short of the passage to the base its fuel is weighed to goes in to that base")
                            .worked(format!(
                                "{intent_words}: {lead}. It lies at its station {station}; {fuel_text}: {why}. It goes in: it is routed to {base_text}, the nearest base of the side by sea that holds fuel above its reserve and can fuel it, at cruise."
                            ))
                            .would_change("more fuel or ammunition aboard")
                            .done(),
                    ));
                }
                None => {
                    let fuel_text = match base {
                        Some((b, _, nm)) => format!(
                            "its fuel is weighed to {}, {nm} nm from the station, and every ship has the passage there at cruise times the {} of the {risk_text}",
                            base_words(view, b),
                            margin(risk)
                        ),
                        None => no_base_text.to_string(),
                    };
                    let ammunition_text = match (&ammunition, base) {
                        (Some(a), None) => {
                            format!("; {}, but with no base to go in to it stays", a.words)
                        }
                        _ => String::new(),
                    };
                    out.summary = Some(
                        serving(task, plan)
                            .evidence(lies.clone(), today)
                            .evidence(threat_text.to_string(), today)
                            .evidence(fuel_text.clone(), today)
                            .binding("a force with no home port lies stopped at its station")
                            .worked(format!(
                                "{intent_words}: {lead}. It lies stopped at its station {station} and is given no route; {fuel_text}{ammunition_text}; {threat_text}, so it is at {} readiness.",
                                if read.sorties { "battle" } else { "normal" }
                            ))
                            .would_change("a ship short of fuel, or a contact near the station, or none")
                            .done(),
                    );
                }
            }
        } else {
            // Routed to the station, one way (11, 2, 5).
            let cruise = hours_to(view, id, station, SpeedSetting::Cruise);
            let full = hours_to(view, id, station, SpeedSetting::Full);
            let hours_text = format!(
                "the station is {} hours away at cruise and {} at full speed",
                cruise.map_or_else(|| "no".to_string(), |h| h.to_string()),
                full.map_or_else(|| "no".to_string(), |h| h.to_string())
            );
            let chosen = speed_from_sea(cruise, view.sea_path(here, station).is_some());
            let waypoints: Vec<Hex> = match base {
                Some((_, hex, _)) if hex != station => vec![station, hex],
                _ => vec![station],
            };
            let needs = chosen.and_then(|speed| {
                base.map(|_| fuel_needs(view, id, &waypoints, Some(speed), margin(risk)))
            });
            match (chosen, needs) {
                (None, _) | (Some(_), Some(None)) => {
                    out.cannot = true;
                    out.summary = Some(
                        serving(task, plan)
                            .evidence(lies.clone(), today)
                            .evidence(hours_text.clone(), today)
                            .binding("a force goes to its station by the engine's sea route")
                            .worked(format!(
                                "{intent_words}: {lead}, but the engine gives no passage from {here} to the station by sea ({hours_text}), so it is given no route and the staff cannot meet {}.",
                                task.id
                            ))
                            .would_change("a station the force can reach")
                            .done(),
                    );
                }
                (Some(speed), needs) => {
                    let needs = needs.flatten().map(|(n, _)| n);
                    let speed_text = match speed {
                        SpeedSetting::Full => "full speed",
                        _ => "cruise",
                    };
                    let fuel_text = match (&needs, base) {
                        (Some(needs), Some((b, _, nm))) => format!(
                            "its fuel is weighed to {}, the nearest base of the side by sea that holds fuel above its reserve and can fuel it, {nm} nm from the station: the passage to the station and from there to {}, at {speed_text} by the engine's measure, times the {} of the {risk_text}: {}",
                            base_words(view, b),
                            base_words(view, b),
                            margin(risk),
                            needs_words(needs)
                        ),
                        _ => no_base_text.to_string(),
                    };
                    let short: Vec<ShipFuel> = needs
                        .iter()
                        .flatten()
                        .copied()
                        .filter(|n| n.short())
                        .collect();
                    if !short.is_empty() || ammunition.is_some() {
                        let why = match (short.is_empty(), &ammunition) {
                            (false, Some(a)) => {
                                format!(
                                    "short of the fuel it needs: {}; and {}",
                                    needs_words(&short),
                                    a.words
                                )
                            }
                            (false, None) => {
                                format!("short of the fuel it needs: {}", needs_words(&short))
                            }
                            (true, Some(a)) => a.words.clone(),
                            (true, None) => String::new(),
                        };
                        out.summary = Some(
                            serving(task, plan)
                                .evidence(lies.clone(), today)
                                .evidence(fuel_text.clone(), today)
                                .binding("fuel comes first: a force with a ship short of the fuel or the ammunition it needs does not go to its station")
                                .worked(format!(
                                    "{intent_words}: {lead}; {hours_text}; {fuel_text}: {why}, so it does not go to its station."
                                ))
                                .would_change("more fuel or ammunition aboard")
                                .done(),
                        );
                    } else {
                        if force.speed != speed {
                            speed_order = Some((
                                TaskForceOrder::SetSpeed(speed),
                                serving(task, plan)
                                    .evidence(hours_text.clone(), today)
                                    .binding(format!("cruise where it reaches the station within {NIGHT_HOURS} hours, else full speed"))
                                    .worked(format!("{hours_text}: {id} is set to {speed_text}."))
                                    .would_change("a station nearer, or farther")
                                    .done(),
                            ));
                        }
                        route = Some((
                            TaskForceOrder::SetRoute {
                                waypoints: vec![station],
                                act_at: BTreeSet::new(),
                                stand_at: BTreeSet::new(),
                            },
                            serving(task, plan)
                                .evidence(lies.clone(), today)
                                .evidence(fuel_text.clone(), today)
                                .evidence(hours_text.clone(), today)
                                .binding("a force with no home port is routed to its station, one way, and lies stopped there")
                                .worked(format!(
                                    "{intent_words}: {lead}. It is not at its station and not under way to it; {fuel_text}: it has the fuel. It is routed to the station {station} at {speed_text}, one way: {hours_text}. It arrives and lies stopped there."
                                ))
                                .would_change("the force at its station, or under way to it, or less fuel aboard")
                                .done(),
                        ));
                    }
                }
            }
        }

        // Home port, rules of engagement, readiness, speed, route (rule 15's
        // order of the orders).
        if let Some(order) = home_order {
            out.orders.push(order);
        }
        if let Some(order) = roe_order(task, plan, force, risk, today) {
            out.orders.push(order);
        }
        // Readiness by the test of a sortie, whether or not it goes (11).
        let wanted = if read.sorties {
            Readiness::Battle
        } else {
            Readiness::Normal
        };
        if force.readiness != wanted {
            out.orders.push((
                TaskForceOrder::SetReadiness(wanted),
                serving(task, plan)
                    .evidence(threat_text.to_string(), today)
                    .binding("a force with no home port is at battle readiness on a day it would sortie, normal on one it would not")
                    .worked(format!(
                        "{intent_words}: {threat_text}; it {} sortie by the test of a stand, so {id} is set to {} readiness.",
                        if read.sorties { "would" } else { "would not" },
                        if read.sorties { "battle" } else { "normal" }
                    ))
                    .would_change("a threat near the station, or none")
                    .done(),
            ));
        }
        if let Some(order) = speed_order {
            out.orders.push(order);
        }
        if let Some(order) = route {
            out.orders.push(order);
        }
        self.hand_in_stand(task, plan, id, out, in_scope, intent_words);
    }

    fn hand_in_stand(
        &mut self,
        task: &Intent,
        plan: &Plan,
        force: TaskForceId,
        out: Out,
        in_scope: bool,
        intent_words: &str,
    ) {
        let mut lines: Vec<String> = Vec::new();
        let target = self.target(in_scope);
        for (order, reason) in out.orders {
            lines.push(reason.worked.clone());
            target.orders.push(StaffOrder {
                kind: OrderKind::TaskForce {
                    target: force,
                    order,
                },
                intent: task.id,
                reason,
            });
        }
        if let Some(reason) = out.summary {
            lines.push(reason.worked.clone());
            target.no_orders.push(reason);
            if out.cannot {
                let index = target.no_orders.len() - 1;
                target.notices.push(StaffNotice {
                    kind: NoticeKind::CannotMeet,
                    plan: plan.id,
                    run: None,
                    rests_on: NoticeBasis::NoOrder(index),
                });
            }
        }
        if in_scope {
            self.lines
                .push(format!("{intent_words}: {}", lines.join(" ")));
        }
    }
}

/// The order that sets a force to radio silence (decision 0136, 18): at
/// preserve the force, where reports travel between commands, a force the
/// side's view shows is not silent. At balanced and at press, and where it is
/// silent already, nothing; the staff never turns silence off (20).
pub(crate) fn silence_order(
    view: &SideView<'_>,
    task: &Intent,
    plan: &Plan,
    force: TaskForceId,
    risk: Risk,
    today: u32,
) -> Option<(TaskForceOrder, Reason)> {
    if risk != Risk::Preserve || !view.reports_travel() || view.radio_silence(force) != Some(false)
    {
        return None;
    }
    Some((
        TaskForceOrder::SetRadioSilence { on: true },
        serving(task, plan)
            .evidence(
                format!("{force} is not on radio silence and reports travel between commands"),
                today,
            )
            .binding("at preserve the force the force keeps radio silence")
            .worked(format!(
                "At preserve the force, {force} keeps radio silence: it is set to it, and sends nothing until it fights, is attacked or strikes. What silence buys is not built: nothing listens to radio traffic yet (decision 0136, 19)."
            ))
            .would_change("a different risk level")
            .done(),
    ))
}

/// The order that sets a stand's rules of engagement once from the risk
/// (decision 0124, 13): the stance engage and the risk's break-off level,
/// the straggler instruction and keep share as they are. `None` where the
/// force has them already.
fn roe_order(
    task: &Intent,
    plan: &Plan,
    force: &TaskForce,
    risk: Risk,
    today: u32,
) -> Option<(TaskForceOrder, Reason)> {
    let id = force.id;
    let risk_text = risk_words(risk);
    let wanted_roe = RulesOfEngagement {
        stance: Stance::Engage,
        break_off: break_off(risk),
        ..force.rules_of_engagement
    };
    if force.rules_of_engagement == wanted_roe {
        return None;
    }
    Some((
        TaskForceOrder::SetRulesOfEngagement {
            stance: wanted_roe.stance,
            break_off: wanted_roe.break_off,
            stragglers: wanted_roe.stragglers,
            keep_share: wanted_roe.keep_share,
        },
        serving(task, plan)
            .evidence(format!(
                "{id} has the stance {} and a break-off level of {}",
                stance_words(force.rules_of_engagement.stance),
                force.rules_of_engagement.break_off
            ), today)
            .binding(format!("the break-off level of the {risk_text}: {}", break_off(risk)))
            .worked(format!(
                "A force that stands to meet the enemy by night engages: the rules of engagement of {id} are set to the stance engage and the break-off level of the {risk_text}, {}; its straggler instruction and keep share are left as they are.",
                break_off(risk)
            ))
            .would_change("a different risk level")
            .done(),
    ))
}

pub(crate) fn stance_words(stance: Stance) -> &'static str {
    match stance {
        Stance::Engage => "engage",
        Stance::Avoid => "avoid",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_table() {
        assert_eq!(margin(Risk::Balanced), "1.2".parse::<Fixed>().unwrap());
        assert_eq!(break_off(Risk::Press), "0.75".parse::<Fixed>().unwrap());
    }

    #[test]
    fn preserve_counts_only_a_contact_it_is_not_outnumbered_by() {
        let three = Sighted {
            number: 1,
            hexes: 7,
            ships: 3,
        };
        assert_eq!(threat(Risk::Preserve, 2, &[three]), None);
        assert_eq!(threat(Risk::Balanced, 2, &[three]), Some(three));
        let far = Sighted { hexes: 16, ..three };
        assert_eq!(threat(Risk::Press, 5, &[far]), None);
    }

    #[test]
    fn a_contact_that_outnumbers_the_force_near_the_station() {
        let nine = Sighted {
            number: 2,
            hexes: 10,
            ships: 9,
        };
        let three = Sighted {
            number: 1,
            ships: 3,
            ..nine
        };
        assert_eq!(outnumbering(4, &[three, nine]), Some(nine));
        assert_eq!(outnumbering(4, &[three]), None);
        assert_eq!(outnumbering(9, &[nine]), None, "as many is not more");
        let far = Sighted { hexes: 16, ..nine };
        assert_eq!(outnumbering(4, &[far]), None);
        let nearer = Sighted {
            number: 3,
            hexes: 5,
            ..nine
        };
        assert_eq!(outnumbering(4, &[nine, nearer]), Some(nearer));
    }

    #[test]
    fn the_speed_from_the_sea() {
        let h = |t: &str| t.parse::<Fixed>().unwrap();
        // 160 nm at 15 kn: 10.666666 hours, within the night.
        assert_eq!(
            speed_from_sea(Some(h("10.666666")), true),
            Some(SpeedSetting::Cruise)
        );
        assert_eq!(
            speed_from_sea(Some(h("12")), true),
            Some(SpeedSetting::Cruise)
        );
        // 320 nm at 15 kn: 21.333333 hours, so full, arrive or not.
        assert_eq!(
            speed_from_sea(Some(h("21.333333")), true),
            Some(SpeedSetting::Full)
        );
        assert_eq!(speed_from_sea(None, true), Some(SpeedSetting::Full));
        assert_eq!(speed_from_sea(None, false), None);
    }

    #[test]
    fn the_day_after_fuelling() {
        assert_eq!(usable_day(1, Fixed::ZERO), 2);
        assert_eq!(usable_day(1, Fixed::from_int(24)), 2);
        assert_eq!(usable_day(1, Fixed::from_int(25)), 3);
    }
}
