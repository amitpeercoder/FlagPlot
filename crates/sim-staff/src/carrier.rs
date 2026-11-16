//! The doctrine for operating a carrier force, at the force's level
//! (decision 0125, rules 3 to 11). It reuses the pieces of the base's
//! doctrines for the groups aboard: the patrol share and the rest of the
//! cover (decision 0115), the arc of its rule 9 taken from the force's hex,
//! and the strike table and sizing of decision 0123.
//!
//! The decision for the groups is a pure function of plain numbers,
//! [`decide`], so that it can be tested in a view built for it; the
//! [`Day`](crate::air::Day) reads the side's view into those numbers, gives
//! the force its orders and hands in the orders and reasons.
//!
//! Each morning, for a task whose force is not held, over the air groups
//! aboard its ships that are not held, in group number order:
//!
//! - **fighters** fly the patrol at the risk's share (three quarters, a half,
//!   a quarter), raised by a quarter and not above 1 on a morning when the
//!   force was attacked from the air on any of the three days before;
//! - **who searches:** a group that can search and cannot attack a ship;
//!   where the force has none, the lowest numbered that can do both. The arc
//!   is decision 0115's, from the force's hex, with the enemy's bases on the
//!   side's map, the contacts it holds and the waters of a deny intent above;
//! - **the rest strike ships** by decision 0123's table, with sizing on;
//! - **rest:** above the risk's fatigue level, except the last fighter group
//!   that can fly and the last group that searches.
//!
//! The force: kept at the task's hex (routed there at cruise); taken home
//! when a ship has less fuel than the passage home needs with the margin,
//! and sent back when every ship has what the passage out and home needs;
//! battle readiness at its hex; the stance avoid and the break-off level of
//! the risk.

use std::collections::BTreeMap;

use sim_core::command::{
    Addressee, Intent, IntentKind, NoticeBasis, NoticeKind, Reason, Risk, StaffNotice, StaffOrder,
};
use sim_core::fixed::Fixed;
use sim_core::hex::Hex;
use sim_core::ids::{AirGroupId, TaskForceId};
use sim_core::orders::{
    AirGroupOrder, AirMission, OrderKind, Reach, Readiness, SpeedSetting, Stance, TaskForceOrder,
};
use sim_core::refdata::{AircraftRole, ShipKind};
use sim_core::side_events::SideEventKind;
use sim_core::state::{AirGroupLocation, RulesOfEngagement, SearchArc, TaskForceStatus};

use crate::air::{
    arc_for, arc_words, base_share_of, fighter_verdicts, mission_words, raise, Day, Threat,
    RAID_DAYS,
};
use crate::deny::waters;
use crate::neutralise::bomber_reach;
use crate::raid::rest_level;
use crate::reasons::{base_words, serving};
use crate::stand::{
    afloat, break_off, expected_words, fuel_needs, home_of, margin, stance_words, stopped,
};
use crate::strike::{StrikeGroup, StrikeRole};

// ---------------------------------------------------------------------------
// The pure decision.
// ---------------------------------------------------------------------------

/// The patrol share of the fighters aboard (rule 3): the risk's share,
/// raised by a quarter and not above 1 when the force was attacked from the
/// air on any of the three days before.
pub fn patrol_share(risk: Risk, attacked: bool) -> Fixed {
    let share = base_share_of(risk);
    if attacked {
        std::cmp::min(Fixed::ONE, share + raise())
    } else {
        share
    }
}

/// One group aboard as the decision reads it.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct CarrierGroup {
    pub id: AirGroupId,
    /// Its type is a fighter.
    pub fighter: bool,
    /// Not a fighter, and its type has a search range.
    pub can_search: bool,
    /// Not a fighter, and its type carries a weapon that can hit a ship.
    pub can_strike: bool,
    pub fatigue: Fixed,
    /// Aircraft able to fly now.
    pub able: u32,
    pub mission: AirMission,
    pub patrol_share: Fixed,
    pub search: Option<SearchArc>,
    pub search_range_nm: Fixed,
    /// Its oldest contact as it stands, its own setting or the figure.
    pub contact_age: u32,
    pub target_priority: Vec<ShipKind>,
    pub reach: Reach,
    pub held_share: Fixed,
    pub sizing: bool,
}

/// What a group is to the carrier task this morning.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum CarrierRole {
    /// Flies the patrol, or rests.
    Fighter,
    /// Searches, or rests.
    Searcher,
    /// Strikes ships by the table, or rests.
    Striker,
    /// None of these: no order.
    Other,
}

/// The decision of one morning for the groups aboard.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct CarrierAir {
    /// For each group, in group number order, its orders: the mission first,
    /// then its settings, each only where it differs from what it has.
    pub orders: Vec<(AirGroupId, AirGroupOrder)>,
    /// The patrol share the fighters fly.
    pub share: Fixed,
    /// Each group's role, in group number order.
    pub roles: Vec<(AirGroupId, CarrierRole)>,
    /// The groups that rest today.
    pub rested: Vec<AirGroupId>,
}

/// The rule of who searches (rule 4): every group that can search and cannot
/// attack a ship; where there is none, the lowest numbered that can do both.
/// The rest that can attack a ship strike.
pub fn roles(groups: &[CarrierGroup]) -> Vec<(AirGroupId, CarrierRole)> {
    let mut sorted: Vec<&CarrierGroup> = groups.iter().collect();
    sorted.sort_by_key(|g| g.id);
    let pure: Vec<AirGroupId> = sorted
        .iter()
        .filter(|g| !g.fighter && g.can_search && !g.can_strike)
        .map(|g| g.id)
        .collect();
    let both = if pure.is_empty() {
        sorted
            .iter()
            .find(|g| !g.fighter && g.can_search && g.can_strike)
            .map(|g| g.id)
    } else {
        None
    };
    sorted
        .iter()
        .map(|g| {
            let role = if g.fighter {
                CarrierRole::Fighter
            } else if pure.contains(&g.id) || both == Some(g.id) {
                CarrierRole::Searcher
            } else if g.can_strike {
                CarrierRole::Striker
            } else {
                CarrierRole::Other
            };
            (g.id, role)
        })
        .collect()
}

/// The orders of one morning for the groups aboard (rules 3 to 6): at
/// `risk`, with the oldest contact `figure`, `attacked` from the air on the
/// three days before or not, and the search arc from `arc_from` to `arc_to`
/// (equal for the whole circle).
pub fn decide(
    risk: Risk,
    figure: u32,
    attacked: bool,
    arc_from: u32,
    arc_to: u32,
    groups: &[CarrierGroup],
) -> CarrierAir {
    let level = rest_level(risk);
    let share = patrol_share(risk, attacked);
    let roles = roles(groups);
    let role_of = |id: AirGroupId| {
        roles
            .iter()
            .find(|(g, _)| *g == id)
            .map_or(CarrierRole::Other, |(_, r)| *r)
    };
    // The rest rule for fighters and for searchers: the last of each that
    // can fly is never rested (rule 6), most tired first, then by number.
    let mut rested: Vec<AirGroupId> = Vec::new();
    for kind in [CarrierRole::Fighter, CarrierRole::Searcher] {
        let mut these: Vec<&CarrierGroup> =
            groups.iter().filter(|g| role_of(g.id) == kind).collect();
        these.sort_by_key(|g| (std::cmp::Reverse(g.fatigue), g.id));
        let tuples: Vec<(Fixed, u32)> = these.iter().map(|g| (g.fatigue, g.able)).collect();
        for (g, verdict) in these.iter().zip(fighter_verdicts(&tuples, level)) {
            if verdict.rests() {
                rested.push(g.id);
            }
        }
    }
    let mut sorted: Vec<&CarrierGroup> = groups.iter().collect();
    sorted.sort_by_key(|g| g.id);
    let mut orders = Vec::new();
    for g in sorted {
        let rests = rested.contains(&g.id);
        match role_of(g.id) {
            CarrierRole::Fighter => {
                let wanted = if rests {
                    AirMission::Rest
                } else {
                    AirMission::CombatAirPatrol
                };
                if g.mission != wanted {
                    orders.push((g.id, AirGroupOrder::SetAirMission(wanted)));
                }
                if !rests && g.patrol_share != share {
                    orders.push((g.id, AirGroupOrder::SetPatrolShare { share }));
                }
            }
            CarrierRole::Searcher => {
                let wanted = if rests {
                    AirMission::Rest
                } else {
                    AirMission::Search
                };
                if g.mission != wanted {
                    orders.push((g.id, AirGroupOrder::SetAirMission(wanted)));
                }
                let arc = SearchArc {
                    from_deg: arc_from,
                    to_deg: arc_to,
                    range_nm: g.search_range_nm,
                };
                if !rests && g.search != Some(arc) {
                    orders.push((
                        g.id,
                        AirGroupOrder::SetSearchArc {
                            from_deg: i32::try_from(arc_from).unwrap_or(0),
                            to_deg: i32::try_from(arc_to).unwrap_or(0),
                            range_nm: g.search_range_nm,
                        },
                    ));
                }
            }
            CarrierRole::Striker => {
                let strike = StrikeGroup {
                    id: g.id,
                    role: StrikeRole::Bomber,
                    fatigue: g.fatigue,
                    mission: g.mission,
                    contact_age: g.contact_age,
                    target_priority: g.target_priority.clone(),
                    reach: g.reach,
                    held_share: g.held_share,
                    sizing: g.sizing,
                    patrol_share: g.patrol_share,
                };
                let strike_orders = crate::strike::decide(risk, figure, &[strike]);
                if g.fatigue > level {
                    rested.push(g.id);
                }
                orders.extend(strike_orders);
            }
            CarrierRole::Other => {}
        }
    }
    rested.sort();
    CarrierAir {
        orders,
        share,
        roles,
        rested,
    }
}

// ---------------------------------------------------------------------------
// The morning, from the side's view.
// ---------------------------------------------------------------------------

impl Day<'_, '_> {
    /// The carrier doctrine for the morning (decision 0125, 3 to 11).
    pub(crate) fn carrier_forces(&mut self) {
        let tasks =
            self.in_priority_order(|i| matches!(i.kind, IntentKind::OperateCarriers { .. }));
        for id in tasks {
            let Some(task) = self.state.intents.get(&id).cloned() else {
                continue;
            };
            if !self.released(&task) {
                continue;
            }
            self.serve_carrier(&task);
        }
    }

    /// The latest day before today, in the three days before, on which the
    /// side's force `force` was attacked from the air, from its own events.
    fn force_attacked(&mut self, force: TaskForceId) -> Option<u32> {
        let today = self.today;
        let first = today.saturating_sub(RAID_DAYS);
        self.side_events()
            .iter()
            .filter(|e| e.day >= first && e.day < today)
            .filter(|e| {
                matches!(&e.kind, SideEventKind::AirAttackAgainst { task_force, .. } if *task_force == force)
            })
            .map(|e| e.day)
            .max()
    }

    #[allow(clippy::too_many_lines)]
    fn serve_carrier(&mut self, task: &Intent) {
        let Some(plan) = self.plan_of(task) else {
            return;
        };
        let IntentKind::OperateCarriers { force: id, hex } = task.kind else {
            return;
        };
        let in_scope = self.in_scope(task.id);
        let today = self.today;
        let view = self.view;
        let risk = self.state.risk_read_up(task.id);
        let risk_text = format!("{} risk", risk.name());
        let level = rest_level(risk);
        let intent_words = format!("{}, operating the carrier force {id} from {hex}", task.id);
        let mut orders: Vec<StaffOrder> = Vec::new();
        let mut notes: Vec<Reason> = Vec::new();
        let mut cannot = false;

        let force = view.task_forces().find(|f| f.id == id).cloned();
        let held = force
            .as_ref()
            .and_then(|f| self.state.hold_on_force(id, &f.ships));
        let ships = afloat(view, id);
        let carriers: Vec<_> = ships
            .iter()
            .filter(|(_, _, _, kind)| *kind == ShipKind::Carrier)
            .map(|(ship, _, _, _)| *ship)
            .collect();
        let aboard_ids: Vec<_> = ships.iter().map(|(s, _, _, _)| *s).collect();
        let mut aboard: Vec<_> = view
            .air_groups()
            .filter(|g| matches!(g.location, AirGroupLocation::Ship(s) if aboard_ids.contains(&s)))
            .collect();
        aboard.sort_by_key(|g| g.id);
        let able_total: u32 = aboard
            .iter()
            .map(|g| view.able_to_fly(g.id).unwrap_or(0))
            .sum();

        match (&force, held) {
            (None, _) => {
                cannot = true;
                notes.push(
                    serving(task, &plan)
                        .evidence(format!("{id} is not one of the side's forces now"), today)
                        .worked(format!(
                            "{intent_words}: {id} is gone, so the staff cannot meet {}.",
                            task.id
                        ))
                        .would_change("the force back")
                        .done(),
                );
            }
            (Some(_), Some(hold)) => {
                notes.push(
                    serving(task, &plan)
                        .evidence(format!("{hold} is held by the commander"), today)
                        .binding(format!("the hold on {hold}"))
                        .worked(format!("{intent_words}: {hold} is held by the commander, so the staff gives {id} and its groups no order."))
                        .would_change("the hold released")
                        .done(),
                );
            }
            (Some(_), None) if carriers.is_empty() || able_total == 0 => {
                // When it cannot (rule 10).
                cannot = true;
                let why = if carriers.is_empty() {
                    format!("{id} has no carrier afloat")
                } else {
                    format!("no air group aboard {id} has an aircraft able to fly")
                };
                notes.push(
                    serving(task, &plan)
                        .evidence(why.clone(), today)
                        .binding("a carrier afloat with an air group able to fly")
                        .worked(format!(
                            "{intent_words}: {why}, so the staff cannot meet {}.",
                            task.id
                        ))
                        .would_change("a carrier afloat with aircraft able to fly")
                        .done(),
                );
            }
            (Some(force), None) => {
                let here = force.position();
                let at_hex = here == hex && force.toward_hex.is_none();
                let home = home_of(view, force).map(|(b, _)| b);
                let home_hex = home.and_then(|b| view.bases().find(|x| x.id == b).map(|x| x.hex));
                let home_text =
                    home.map_or_else(|| "no home port".to_string(), |b| base_words(view, b));
                let underway = matches!(
                    force.status,
                    TaskForceStatus::Underway | TaskForceStatus::Halted
                );
                let on_way_home = underway
                    && home_hex.is_some()
                    && force.route.waypoints.last().copied() == home_hex
                    && force.route.next_index < force.route.waypoints.len();
                let at_home = stopped(force) && Some(here) == home_hex;
                let mut route: Option<(Vec<Hex>, Reason)> = None;
                let mut going_home = on_way_home;
                let margin = margin(risk);
                let fuel_words = |needs: &[(sim_core::ids::ShipId, Fixed, Fixed)]| {
                    needs
                        .iter()
                        .map(|(ship, aboard, need)| {
                            format!("{ship} has {aboard} t aboard against {need} t needed")
                        })
                        .collect::<Vec<_>>()
                        .join(", ")
                };
                // Fuel (rule 8) and its place (rule 7).
                match home_hex {
                    Some(home_at) if at_home && !at_hex => {
                        if let Some((needs, _)) = fuel_needs(
                            view,
                            id,
                            &[hex, home_at],
                            Some(SpeedSetting::Cruise),
                            margin,
                        ) {
                            let text = fuel_words(&needs);
                            if needs.iter().all(|(_, aboard, need)| aboard >= need) {
                                route = Some((
                                    vec![hex],
                                    serving(task, &plan)
                                        .evidence(format!("the passage to {hex} and home, by the engine's measure, times the {margin} of the {risk_text}: {text}"), today)
                                        .binding("back to its hex once every ship has the fuel for the passage out and home with the margin")
                                        .worked(format!("{intent_words}: {id} is at its home port {home_text} and every ship has what the passage to {hex} and home needs ({text}): it is routed back to {hex} at cruise."))
                                        .would_change("less fuel aboard")
                                        .done(),
                                ));
                            } else {
                                notes.push(
                                    serving(task, &plan)
                                        .evidence(format!("the passage to {hex} and home, by the engine's measure, times the {margin} of the {risk_text}: {text}"), today)
                                        .binding("back to its hex once every ship has the fuel for the passage out and home with the margin")
                                        .worked(format!("{intent_words}: {id} is at its home port {home_text} and fuels there: not every ship has what the passage to {hex} and home needs ({text}), so it stays."))
                                        .would_change("more fuel aboard")
                                        .done(),
                                );
                            }
                        }
                    }
                    Some(home_at) if !on_way_home && here != home_at => {
                        let needs = fuel_needs(view, id, &[home_at], None, margin);
                        let short = needs
                            .as_ref()
                            .is_some_and(|(n, _)| n.iter().any(|(_, aboard, need)| aboard < need));
                        let text = needs
                            .as_ref()
                            .map_or_else(String::new, |(n, _)| fuel_words(n));
                        let fuel_text = format!("the passage home to {home_text}, by the engine's measure, times the {margin} of the {risk_text}, for the ship with least against its need: {text}");
                        if short {
                            going_home = true;
                            route = Some((
                                vec![home_at],
                                serving(task, &plan)
                                    .evidence(fuel_text.clone(), today)
                                    .binding("a ship with less fuel than the passage home needs with the margin takes the force home")
                                    .worked(format!("{intent_words}: {fuel_text}: a ship has less than it needs, so {id} is routed home to {home_text} to fuel."))
                                    .would_change("more fuel aboard")
                                    .done(),
                            ));
                        } else {
                            notes.push(
                                serving(task, &plan)
                                    .evidence(fuel_text.clone(), today)
                                    .worked(format!("{intent_words}: {fuel_text}: every ship has what it needs, so {id} stays on its task."))
                                    .would_change("less fuel aboard")
                                    .done(),
                            );
                        }
                    }
                    Some(home_at) if on_way_home => {
                        notes.push(
                            serving(task, &plan)
                                .evidence(format!("{id} is at {here}, on its way home to {home_text}"), today)
                                .worked(format!("{intent_words}: {id} is on its way home to {home_text}; {}. It is given no route.", expected_words(view, id, home_at)))
                                .would_change("the force stopped at its home port")
                                .done(),
                        );
                    }
                    _ => {
                        if home_hex.is_none() {
                            notes.push(
                                serving(task, &plan)
                                    .evidence(format!("{id} has no home port and is stopped at no base of the side"), today)
                                    .worked(format!("{intent_words}: {id} has no home port the staff knows, so its fuel is not measured against a passage home."))
                                    .would_change("a home port set")
                                    .done(),
                            );
                        }
                    }
                }
                if route.is_none() && !going_home && !at_hex && !at_home {
                    route = Some((
                        vec![hex],
                        serving(task, &plan)
                            .evidence(format!("{id} is at {here}, not at {hex}"), today)
                            .binding("the force is kept at the task's hex")
                            .worked(format!("{intent_words}: {id} is at {here}, not in the task's hex and not on its way home, so it is routed to {hex} at cruise."))
                            .would_change("the force in the hex")
                            .done(),
                    ));
                }
                let give = |orders: &mut Vec<StaffOrder>, order: TaskForceOrder, reason: Reason| {
                    orders.push(StaffOrder {
                        kind: OrderKind::TaskForce { target: id, order },
                        intent: task.id,
                        reason,
                    });
                };
                // Readiness and rules of engagement (rule 9).
                if at_hex && !going_home && force.readiness != Readiness::Battle {
                    give(
                        &mut orders,
                        TaskForceOrder::SetReadiness(Readiness::Battle),
                        serving(task, &plan)
                            .evidence(format!("{id} is at its hex {hex}"), today)
                            .binding("battle readiness while it is at its hex")
                            .worked(format!("{intent_words}: {id} is at {hex}, so it is set to battle readiness."))
                            .would_change("the force away from its hex")
                            .done(),
                    );
                }
                let wanted_roe = RulesOfEngagement {
                    stance: Stance::Avoid,
                    break_off: break_off(risk),
                    ..force.rules_of_engagement
                };
                if force.rules_of_engagement != wanted_roe {
                    give(
                        &mut orders,
                        TaskForceOrder::SetRulesOfEngagement {
                            stance: wanted_roe.stance,
                            break_off: wanted_roe.break_off,
                            stragglers: wanted_roe.stragglers,
                            keep_share: wanted_roe.keep_share,
                        },
                        serving(task, &plan)
                            .evidence(format!(
                                "{id} has the stance {} and a break-off level of {}",
                                stance_words(force.rules_of_engagement.stance),
                                force.rules_of_engagement.break_off
                            ), today)
                            .binding(format!("the break-off level of the {risk_text}: {}", break_off(risk)))
                            .worked(format!(
                                "A carrier force does not seek a gun action: the rules of engagement of {id} are set to the stance avoid and the break-off level of the {risk_text}, {}.",
                                break_off(risk)
                            ))
                            .would_change("a different risk level")
                            .done(),
                    );
                }
                if let Some((waypoints, reason)) = route {
                    let to_hex = waypoints.first() == Some(&hex);
                    if to_hex && force.speed != SpeedSetting::Cruise {
                        give(
                            &mut orders,
                            TaskForceOrder::SetSpeed(SpeedSetting::Cruise),
                            reason.clone(),
                        );
                    }
                    give(&mut orders, TaskForceOrder::route(waypoints), reason);
                }

                // The groups aboard (rules 3 to 6).
                let attacked = self.force_attacked(id);
                let mut threats: Vec<Threat> = self.threats();
                for above in self.state.ancestors(task.id) {
                    if let Some((centre, distance)) = above.kind.waters() {
                        for w in waters(view, centre, distance) {
                            threats.push(Threat {
                                hex: w,
                                words: format!("the waters to deny at {w}"),
                            });
                        }
                    }
                }
                let hexes: Vec<Hex> = threats.iter().map(|t| t.hex).collect();
                let arc = arc_for(here, &hexes);
                let arc_text = crate::air::arc_working(&arc, &threats);
                let figure = view
                    .air_combat_figures()
                    .map_or(0, |f| f.strike_contact_max_age_pulses);
                let mut groups: Vec<CarrierGroup> = Vec::new();
                let mut held_lines: Vec<String> = Vec::new();
                for g in &aboard {
                    if let Some(hold) = self.state.hold_on_group(g.id) {
                        held_lines.push(format!("{hold} is held by the commander: no order"));
                        continue;
                    }
                    let kind = view.aircraft_type(g.aircraft_type);
                    let fighter = kind.is_some_and(|t| t.role == AircraftRole::Fighter);
                    let range = kind.map_or(Fixed::ZERO, |t| t.search_range_nm);
                    groups.push(CarrierGroup {
                        id: g.id,
                        fighter,
                        can_search: !fighter && range > Fixed::ZERO,
                        can_strike: !fighter && bomber_reach(view, g).is_some(),
                        fatigue: g.fatigue,
                        able: view.able_to_fly(g.id).unwrap_or(0),
                        mission: g.mission,
                        patrol_share: g.patrol_share,
                        search: g.search,
                        search_range_nm: range,
                        contact_age: g.strike_contact_age.unwrap_or(figure),
                        target_priority: g.target_priority.clone(),
                        reach: g.reach,
                        held_share: g.strike_held_share,
                        sizing: g.strike_sizing,
                    });
                }
                let air = decide(risk, figure, attacked.is_some(), arc.from, arc.to, &groups);
                let share_note = match attacked {
                    Some(day) => format!(
                        "{} at {} raised by {} because {id} was attacked from the air on day {day}",
                        base_share_of(risk),
                        risk.name(),
                        raise()
                    ),
                    None => format!(
                        "{} at {}, the force not attacked from the air on the three days before",
                        base_share_of(risk),
                        risk.name()
                    ),
                };
                let mut lines: BTreeMap<AirGroupId, String> = BTreeMap::new();
                for g in &groups {
                    let role = air
                        .roles
                        .iter()
                        .find(|(x, _)| *x == g.id)
                        .map_or(CarrierRole::Other, |(_, r)| *r);
                    let rests = air.rested.contains(&g.id);
                    let fatigue = format!(
                        "fatigue {} {} the {level} of the {risk_text}",
                        g.fatigue,
                        if g.fatigue > level {
                            "above"
                        } else {
                            "not above"
                        }
                    );
                    let line = match role {
                        CarrierRole::Fighter if rests => format!("{}, a fighter group, has {fatigue} and another fighter group can fly: it rests", g.id),
                        CarrierRole::Fighter => format!("{}, a fighter group, has {fatigue}: it flies the patrol at a share of {} ({share_note}); the rest are escorts", g.id, air.share),
                        CarrierRole::Searcher if rests => format!("{}, a group that searches, has {fatigue}: it rests", g.id),
                        CarrierRole::Searcher => format!(
                            "{}, a group that {} searches, has {fatigue}: it searches {} out to {} nm from {here} ({arc_text})",
                            g.id,
                            if g.can_strike { "can search and attack a ship and, the force having no group that can only search, the lowest numbered such," } else { "can search and cannot attack a ship," },
                            arc_words(&arc),
                            g.search_range_nm
                        ),
                        CarrierRole::Striker => format!("{}, a bomber group, has {fatigue}: it strikes ships by the table of the {risk_text}, with its strikes sized", g.id),
                        CarrierRole::Other => format!("{}, neither a fighter nor a group that can search or attack a ship: no order", g.id),
                    };
                    lines.insert(g.id, line);
                }
                for (group, order) in &air.orders {
                    let line = lines.get(group).cloned().unwrap_or_default();
                    let before = groups.iter().find(|g| g.id == *group);
                    let has = match (order, before) {
                        (AirGroupOrder::SetAirMission(_), Some(b)) => {
                            format!("its mission is {}", mission_words(b.mission))
                        }
                        (AirGroupOrder::SetPatrolShare { .. }, Some(b)) => {
                            format!("its patrol share is {}", b.patrol_share)
                        }
                        (AirGroupOrder::SetSearchArc { .. }, Some(b)) => match b.search {
                            Some(a) => format!(
                                "its arc is {} to {} at {} nm",
                                crate::air::bearing(a.from_deg),
                                crate::air::bearing(a.to_deg),
                                a.range_nm
                            ),
                            None => "it has no search arc".to_string(),
                        },
                        _ => "its standing orders differ from the table".to_string(),
                    };
                    let mut reason = serving(task, &plan)
                        .evidence(line.clone(), today)
                        .evidence(has, today)
                        .binding(format!("rest above fatigue {level} at the {risk_text}"))
                        .worked(match order {
                            AirGroupOrder::SetStrikeSizing { .. } => format!(
                                "{line}. The group decides the size of its strikes from the task it is under: {} operates a carrier force, which has more to hit than one force, so each strike is sized by doctrine. Its strike sizing is set on.",
                                task.id
                            ),
                            _ => format!("{line}."),
                        })
                        .would_change("a different fatigue or risk level")
                        .would_change("a new contact, or a contact dropped")
                        .done();
                    if matches!(order, AirGroupOrder::SetStrikeSizing { .. }) {
                        reason.serves.level = Some(Addressee::AirGroup(*group));
                    }
                    orders.push(StaffOrder {
                        kind: OrderKind::AirGroup {
                            target: *group,
                            order: order.clone(),
                        },
                        intent: task.id,
                        reason,
                    });
                }
                let ordered: Vec<AirGroupId> = air.orders.iter().map(|(g, _)| *g).collect();
                let mut unordered: Vec<String> = groups
                    .iter()
                    .filter(|g| !ordered.contains(&g.id))
                    .filter_map(|g| lines.get(&g.id).cloned())
                    .collect();
                unordered.extend(held_lines);
                if !unordered.is_empty() {
                    notes.push(
                        serving(task, &plan)
                            .evidence(format!("{} group(s) aboard {id}", aboard.len()), today)
                            .binding(format!("search arc: {arc_text}"))
                            .worked(format!("No order is given for these groups for {intent_words} this morning: {}.", unordered.join("; ")))
                            .would_change("a group whose fatigue or standing orders differ from what the doctrine wants")
                            .done(),
                    );
                }
            }
        }

        let mut words: Vec<String> = orders.iter().map(|o| o.reason.worked.clone()).collect();
        words.extend(notes.iter().map(|r| r.worked.clone()));
        let out = self.target(in_scope);
        out.orders.extend(orders);
        let mut basis = None;
        for note in notes {
            out.no_orders.push(note);
            basis.get_or_insert(out.no_orders.len() - 1);
        }
        if cannot {
            if let Some(index) = basis {
                out.notices.push(StaffNotice {
                    kind: NoticeKind::CannotMeet,
                    plan: plan.id,
                    run: None,
                    rests_on: NoticeBasis::NoOrder(index),
                });
            }
        }
        if in_scope {
            self.lines
                .push(format!("{intent_words}: {}", words.join(" ")));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn group(id: u32, fighter: bool, search: bool, strike: bool) -> CarrierGroup {
        CarrierGroup {
            id: AirGroupId::new(id),
            fighter,
            can_search: search,
            can_strike: strike,
            fatigue: Fixed::ZERO,
            able: 6,
            mission: AirMission::Rest,
            patrol_share: Fixed::ONE,
            search: None,
            search_range_nm: Fixed::from_int(300),
            contact_age: 0,
            target_priority: ShipKind::ALL.to_vec(),
            reach: Reach::Normal,
            held_share: Fixed::ZERO,
            sizing: false,
        }
    }

    #[test]
    fn a_scout_searches_and_with_none_the_lowest_that_can_do_both() {
        let r = roles(&[
            group(1, true, false, false),
            group(2, false, true, false),
            group(3, false, true, true),
        ]);
        assert_eq!(r[1].1, CarrierRole::Searcher);
        assert_eq!(r[2].1, CarrierRole::Striker);
        let r = roles(&[group(3, false, true, true), group(4, false, true, true)]);
        assert_eq!(r[0].1, CarrierRole::Searcher);
        assert_eq!(r[1].1, CarrierRole::Striker);
    }

    #[test]
    fn a_raid_raises_the_share() {
        assert_eq!(patrol_share(Risk::Balanced, true), Fixed::from_ratio(3, 4));
        assert_eq!(patrol_share(Risk::Preserve, true), Fixed::ONE);
        assert_eq!(patrol_share(Risk::Press, false), Fixed::from_ratio(1, 4));
    }
}
