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

use std::collections::{BTreeMap, BTreeSet};

use sim_core::command::{
    Addressee, Intent, IntentKind, NoticeBasis, NoticeKind, Reason, Risk, StaffNotice, StaffOrder,
};
use sim_core::commander::Initiative;
use sim_core::fixed::Fixed;
use sim_core::hex::Hex;
use sim_core::ids::{AirGroupId, TaskForceId};
use sim_core::orders::{
    AirGroupOrder, AirMission, OrderKind, Reach, Readiness, RejectReason, SpeedSetting, Stance,
    TaskForceOrder,
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
use crate::search::{carrier_once_a_day, carrier_once_words, least_tired, TurnCandidate};
use crate::stand::{
    afloat, break_off, expected_words, fuel_needs, home_of, hours_to, margin, needs_words,
    stance_words, stopped, NIGHT_HOURS,
};
use crate::strike::{StrikeGroup, StrikeRole};
use crate::targets::{kinds_words, StrikeTargets};

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
    /// Its setting to search once a day (decision 0131, 1).
    pub search_once: bool,
    /// Its waters to strike first as they stand (decision 0190).
    pub waters: BTreeSet<Hex>,
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

/// The rule of who searches (rule 4, as decision 0131, 7 amends it): every
/// group that can search and cannot attack a ship; where there is none, the
/// least tired of those that can do both and can fly, then the lowest
/// numbered, chosen each morning (where none of them can fly, the lowest
/// numbered of them). The rest that can attack a ship strike.
pub fn roles(groups: &[CarrierGroup]) -> Vec<(AirGroupId, CarrierRole)> {
    let mut sorted: Vec<&CarrierGroup> = groups.iter().collect();
    sorted.sort_by_key(|g| g.id);
    let pure: Vec<AirGroupId> = sorted
        .iter()
        .filter(|g| !g.fighter && g.can_search && !g.can_strike)
        .map(|g| g.id)
        .collect();
    let both = if pure.is_empty() {
        let candidates: Vec<TurnCandidate> = sorted
            .iter()
            .filter(|g| !g.fighter && g.can_search && g.can_strike)
            .map(|g| TurnCandidate {
                id: g.id,
                fatigue: g.fatigue,
                able: g.able,
            })
            .collect();
        least_tired(&candidates).or_else(|| candidates.first().map(|c| c.id))
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
/// (equal for the whole circle). A bomber's list and waters to strike first
/// are those `targets` gives (decision 0190).
pub fn decide(
    risk: Risk,
    figure: u32,
    attacked: bool,
    arc_from: u32,
    arc_to: u32,
    targets: &StrikeTargets,
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
                let once = carrier_once_a_day(risk);
                if !rests && g.search_once != once {
                    orders.push((g.id, AirGroupOrder::SetSearchOnce { on: once }));
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
                    waters: g.waters.clone(),
                };
                let strike_orders = crate::strike::decide(risk, figure, targets, &[strike]);
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

/// Why a group that can search and attack a ship takes the search this
/// morning (decision 0131, 7), with the numbers of each such group.
fn turn_words(groups: &[CarrierGroup], chosen: AirGroupId) -> String {
    let list = groups
        .iter()
        .filter(|g| !g.fighter && g.can_search && g.can_strike)
        .map(|g| {
            format!(
                "{} at fatigue {} with {} able to fly",
                g.id, g.fatigue, g.able
            )
        })
        .collect::<Vec<_>>()
        .join(", ");
    format!(
        "can search and attack a ship and, the force having no group that can only search, is the least tired of those that can fly, then the lowest numbered ({list}): {chosen}",
    )
}

// ---------------------------------------------------------------------------
// Closing the range (decision 0130), the pure parts.
// ---------------------------------------------------------------------------

/// The oldest a contact may be, in pulses, for the force to close on it or
/// to stay for it (decision 0130, 2 and 5).
pub const CLOSE_MAX_AGE: u32 = 3;

/// The kinds a force closes on at `risk` (decision 0130, 2): nothing at
/// preserve the force, a carrier at balanced, any kind at press.
pub fn closes_on(risk: Risk) -> &'static [ShipKind] {
    match risk {
        Risk::Preserve => &[],
        Risk::Balanced => &[ShipKind::Carrier],
        Risk::Press => &ShipKind::ALL,
    }
}

/// The kinds a force closes on, in words.
pub fn closes_on_words(risk: Risk) -> &'static str {
    match risk {
        Risk::Preserve => "nothing",
        Risk::Balanced => "a carrier",
        Risk::Press => "any kind",
    }
}

/// The speed of a passage (decision 0130, 1): cruise where that brings the
/// force there within the night's hours (`cruise_hours`, by the engine's
/// measure, `None` where cruise does not bring it there), otherwise full
/// speed; always cruise at preserve the force.
pub fn speed_for(risk: Risk, cruise_hours: Option<Fixed>) -> SpeedSetting {
    if risk == Risk::Preserve {
        return SpeedSetting::Cruise;
    }
    match cruise_hours {
        Some(h) if h <= Fixed::from_int(NIGHT_HOURS) => SpeedSetting::Cruise,
        _ => SpeedSetting::Full,
    }
}

/// One contact of ships as the closing rule reads it: its number in the
/// side's series, its hex, its kind, its age in pulses and its air distance
/// from the force.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Held {
    pub number: u32,
    pub hex: Hex,
    pub kind: ShipKind,
    pub age: u32,
    pub distance_nm: Fixed,
    /// Whether its hex is in the waters to strike first of a deny that
    /// names the force (decision 0190, 15); false where there are none.
    pub in_waters: bool,
}

/// What the closing rule makes of one morning's contacts (decision 0130, 2).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Closing {
    /// At preserve the force it closes on nothing.
    Preserve,
    /// No contact of ships is fresh.
    NothingFresh,
    /// No fresh contact is of a kind the risk closes on.
    NothingOfKind,
    /// The first fresh contact of a kind it closes on within a night and
    /// the reach is within reach: it does not close, and a force that has
    /// left its hex stays.
    InReach(Held),
    /// Every fresh contact of a kind it closes on is beyond a night at full
    /// speed and the reach: this one, the first of them.
    TooFar(Held),
    /// The first within a night and the reach is beyond reach: it closes on
    /// this one.
    CloseOn(Held),
}

/// The first of `contacts` by the order of decision 0130, 2: one in the
/// waters to strike first before one outside them (decision 0190, 15), then
/// the kind first in the order of importance, then the nearest, then the
/// lowest number.
fn first(contacts: impl Iterator<Item = Held>) -> Option<Held> {
    contacts.min_by_key(|c| (!c.in_waters, c.kind, c.distance_nm, c.number))
}

/// Whether a force weighing a contact at `risk` closes on it (decision 0130,
/// 2, and 0190, 15): a kind the risk closes on, or, at balanced and at
/// press, any kind in the waters to strike first.
fn closes_on_contact(risk: Risk, c: &Held) -> bool {
    closes_on(risk).contains(&c.kind) || (c.in_waters && risk != Risk::Preserve)
}

/// The level of risk at which a commander of `initiative` weighs a contact
/// that is his force's own, where his task's risk is `risk` (decision 0137,
/// 7): a cautious one a level more careful, a bold one a level bolder, a
/// steady one at the task's. Preserve the force stays so for a cautious
/// commander and press stays press for a bold one.
pub fn weighed_at(initiative: Initiative, risk: Risk) -> Risk {
    match (initiative, risk) {
        (Initiative::Steady, risk) => risk,
        (Initiative::Cautious, Risk::Press) => Risk::Balanced,
        (Initiative::Cautious, Risk::Balanced | Risk::Preserve) => Risk::Preserve,
        (Initiative::Bold, Risk::Preserve) => Risk::Balanced,
        (Initiative::Bold, Risk::Balanced | Risk::Press) => Risk::Press,
    }
}

/// The contacts fresh and of a kind they are weighed to close on, each at
/// the level `weigh` gives it.
fn wanted_by<'a>(
    weigh: &'a dyn Fn(&Held) -> Risk,
    contacts: &'a [Held],
) -> impl Iterator<Item = Held> + 'a {
    contacts
        .iter()
        .copied()
        .filter(|c| c.age <= CLOSE_MAX_AGE)
        .filter(move |c| closes_on_contact(weigh(c), c))
}

/// The first fresh contact of a kind `risk` closes on, wherever it is: the
/// one whose passage gives the force's full speed.
pub fn first_wanted(risk: Risk, contacts: &[Held]) -> Option<Held> {
    first_wanted_by(&|_| risk, contacts)
}

/// [`first_wanted`], each contact weighed at the level `weigh` gives it
/// (decision 0137, 7 and 8).
pub fn first_wanted_by(weigh: &dyn Fn(&Held) -> Risk, contacts: &[Held]) -> Option<Held> {
    first(wanted_by(weigh, contacts))
}

/// The closing rule (decision 0130, 2, with the architect's ruling of Nov
/// 16, 2026, that the first contact decides): of the fresh contacts of a
/// kind the risk closes on within `night_nm` (a night at full speed) and
/// `reach`, the first. In reach, the force does not close (and one that has
/// left its hex stays, rule 5); beyond reach, it closes on it. With none
/// within, it is too far, naming the first of those beyond.
pub fn closing(risk: Risk, reach: Fixed, night_nm: Fixed, contacts: &[Held]) -> Closing {
    closing_by(risk, &|_| risk, reach, night_nm, contacts)
}

/// The closing rule with each contact weighed at the level `weigh` gives it
/// (decision 0137, 7 and 8): a contact that is the force's own at the level
/// its commander's rating reads the task's `risk` at, one it was told of at
/// `risk`. At preserve the force it closes on nothing unless a fresh
/// contact is weighed at a bolder level. With every contact weighed at
/// `risk` it is [`closing`].
pub fn closing_by(
    risk: Risk,
    weigh: &dyn Fn(&Held) -> Risk,
    reach: Fixed,
    night_nm: Fixed,
    contacts: &[Held],
) -> Closing {
    if risk == Risk::Preserve
        && contacts
            .iter()
            .filter(|c| c.age <= CLOSE_MAX_AGE)
            .all(|c| weigh(c) == Risk::Preserve)
    {
        return Closing::Preserve;
    }
    if !contacts.iter().any(|c| c.age <= CLOSE_MAX_AGE) {
        return Closing::NothingFresh;
    }
    let of_kind: Vec<Held> = wanted_by(weigh, contacts).collect();
    let Some(far) = first(of_kind.iter().copied()) else {
        return Closing::NothingOfKind;
    };
    match first(
        of_kind
            .into_iter()
            .filter(|c| c.distance_nm <= night_nm + reach),
    ) {
        Some(c) if c.distance_nm <= reach => Closing::InReach(c),
        Some(c) => Closing::CloseOn(c),
        None => Closing::TooFar(far),
    }
}

/// Where the force goes to close (decision 0130, 3): the first hex of
/// `path` (the hexes entered along the shortest sea route to `contact`, in
/// order) within `reach` of it.
pub fn close_to(path: &[Hex], contact: Hex, reach: Fixed) -> Option<Hex> {
    path.iter()
        .copied()
        .find(|h| Fixed::from_int(i64::from(h.air_distance_nm(contact))) <= reach)
}

fn setting_words(speed: SpeedSetting) -> &'static str {
    match speed {
        SpeedSetting::Full => "full speed",
        _ => "cruise",
    }
}

// ---------------------------------------------------------------------------
// The morning, from the side's view.
// ---------------------------------------------------------------------------

/// The speed of a passage of `force` to `to` by decision 0130, 1, read from
/// the engine's measure, and the words for it: "cruise", or full speed with
/// the hours cruise would take.
pub(crate) fn passage_speed(
    view: &sim_core::view::SideView<'_>,
    force: TaskForceId,
    risk: Risk,
    to: Hex,
) -> (SpeedSetting, String) {
    if risk == Risk::Preserve {
        return (SpeedSetting::Cruise, "cruise".to_string());
    }
    let cruise = hours_to(view, force, to, SpeedSetting::Cruise);
    match speed_for(risk, cruise) {
        SpeedSetting::Cruise => (SpeedSetting::Cruise, "cruise".to_string()),
        speed => (
            speed,
            match cruise {
                Some(h) => format!(
                    "full speed, cruise taking {h} hours, more than the night's {NIGHT_HOURS}"
                ),
                None => {
                    "full speed, cruise not bringing it there by the engine's measure".to_string()
                }
            },
        ),
    }
}

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
            // Under "take a place" the task is served after the landing's
            // morning, by `take_carriers` (decision 0148), and so is one under
            // an island's denial, whose strike reads the takes' plans
            // (decision 0151, 8a).
            if self.under_take(&task).is_some() || self.under_island(&task).is_some() {
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

    pub(crate) fn serve_carrier(&mut self, task: &Intent) {
        self.serve_carrier_with(task, true);
    }

    /// The carrier doctrine's morning for `task`. With `released` false (a
    /// force moved to look while its take's plan is preparing, decision
    /// 0159, 14) the strike on the place is not given; and a group the look
    /// keeps on reconnaissance of the place is given no order at any time.
    #[allow(clippy::too_many_lines)]
    pub(crate) fn serve_carrier_with(&mut self, task: &Intent, released: bool) {
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
        // What the targets limits give its bombers (decision 0190).
        let targets = self.strike_targets(task, Some(id));
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
        // A group before its day is left out (decision 0140, 3): the staff
        // reads and orders only those it may use, and names the others.
        let aboard_waiting = crate::available::groups_waiting(view, aboard.iter().map(|g| g.id));
        aboard.retain(|g| !aboard_waiting.iter().any(|(id, _)| *id == g.id));
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
            (Some(_), _) if crate::available::force_waiting(view, id).is_some() => {
                // Before its day the force is no force a task may use
                // (decision 0140, 3): one reason, no order to it or its groups.
                let day = crate::available::force_waiting(view, id).unwrap_or(today);
                let words = crate::available::from_day_words(day);
                notes.push(
                    serving(task, &plan)
                        .evidence(format!("{id} {words}"), today)
                        .binding(format!("the day {id} may be used from, {day}"))
                        .worked(format!(
                            "{intent_words}: {} is to be served by {id}, which {words}, so the staff gives {id} and its groups no order.",
                            task.id
                        ))
                        .would_change("the day come")
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
                // A force that cannot move for want of fuel is given no route
                // and no speed order, and the staff cannot meet the task
                // (the architect's ruling of Nov 16, 2026, recorded in
                // decision 0131): a ship with no fuel aboard, or the engine's
                // forecast refusing the passage to the hex for fuel.
                let dry_ships: Vec<String> = ships
                    .iter()
                    .filter(|(_, fuel, _, _)| *fuel <= Fixed::ZERO)
                    .map(|(ship, _, _, _)| ship.to_string())
                    .collect();
                let refused_for_fuel = !at_hex
                    && view
                        .forecast_route(id, &[hex], None)
                        .and_then(|f| f.refusal)
                        == Some(RejectReason::OutOfFuel);
                let dry = !dry_ships.is_empty() || refused_for_fuel;
                if dry {
                    cannot = true;
                    let which = if dry_ships.is_empty() {
                        format!("the engine refuses a passage of {id} to {hex} as out of fuel")
                    } else {
                        format!(
                            "{} {} no fuel aboard",
                            dry_ships.join(" and "),
                            if dry_ships.len() == 1 { "has" } else { "have" }
                        )
                    };
                    notes.push(
                        serving(task, &plan)
                            .evidence(format!("{which}; {id} lies at {here}"), today)
                            .binding("a force that cannot move for want of fuel is given no route")
                            .worked(format!("{intent_words}: {which}, and {id} lies at {here}: it cannot move, so it is given no route and no speed order, and the staff cannot meet {}. Its air groups are given their orders as on any morning.", task.id))
                            .would_change("fuel brought to the force")
                            .done(),
                    );
                }
                // A route, the speed it is steamed at where the doctrine sets
                // one (decision 0130, 1), and its reason.
                let mut route: Option<(Vec<Hex>, Option<SpeedSetting>, Reason)> = None;
                let mut going_home = on_way_home;
                // At its home port and short of the fuel to go back: it fuels.
                let mut fuelling = false;
                // The reason the force stays on its task, which the words on
                // closing join.
                let mut place_note: Option<usize> = None;
                let margin = margin(risk);
                let fuel_words = |needs: &[crate::stand::ShipFuel]| needs_words(needs);
                // Fuel (rule 8) and its place (rule 7).
                if !dry {
                    match home_hex {
                        Some(home_at) if at_home && !at_hex => {
                            // The speed of the passage back (decision 0130, 1),
                            // at which its fuel is measured.
                            let (speed, speed_text) = passage_speed(view, id, risk, hex);
                            if let Some((needs, _)) =
                                fuel_needs(view, id, &[hex, home_at], Some(speed), margin)
                            {
                                let text = fuel_words(&needs);
                                if !needs.iter().any(|n| n.short()) {
                                    route = Some((
                                    vec![hex],
                                    Some(speed),
                                    serving(task, &plan)
                                        .evidence(format!("the passage to {hex} and home, by the engine's measure, times the {margin} of the {risk_text}: {text}"), today)
                                        .binding("back to its hex once every ship has the fuel for the passage out and home with the margin")
                                        .worked(format!("{intent_words}: {id} is at its home port {home_text} and every ship has what the passage to {hex} and home needs ({text}): it is routed back to {hex} at {speed_text}."))
                                        .would_change("less fuel aboard")
                                        .done(),
                                ));
                                } else {
                                    fuelling = true;
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
                            // The passage home for fuel is at cruise at every
                            // risk, and its fuel is measured at cruise (the
                            // architect's ruling on decision 0130, 1).
                            let needs = fuel_needs(
                                view,
                                id,
                                &[home_at],
                                Some(SpeedSetting::Cruise),
                                margin,
                            );
                            let short = needs
                                .as_ref()
                                .is_some_and(|(n, _)| n.iter().any(|n| n.short()));
                            let text = needs
                                .as_ref()
                                .map_or_else(String::new, |(n, _)| fuel_words(n));
                            let fuel_text = format!("the passage home to {home_text}, by the engine's measure, times the {margin} of the {risk_text}, for the ship with least against its need: {text}");
                            if short {
                                going_home = true;
                                route = Some((
                                vec![home_at],
                                Some(SpeedSetting::Cruise),
                                serving(task, &plan)
                                    .evidence(fuel_text.clone(), today)
                                    .binding("a ship with less fuel than the passage home needs with the margin takes the force home")
                                    .worked(format!("{intent_words}: {fuel_text}: a ship has less than it needs, so {id} is routed home to {home_text} to fuel, at cruise."))
                                    .would_change("more fuel aboard")
                                    .done(),
                            ));
                            } else {
                                place_note = Some(notes.len());
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
                                place_note = Some(notes.len());
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
                }
                // Closing the range (decision 0130, 2 to 5): on a morning the
                // force is not on its way home and not fuelling at home.
                let mut closes = false;
                let mut stays = false;
                let mut close_words: Option<String> = None;
                // It left its task's hex to close: its last route ends at a
                // hex that is neither the task's nor its home port's.
                let left_to_close = !at_hex
                    && force
                        .route
                        .waypoints
                        .last()
                        .is_some_and(|w| *w != hex && Some(*w) != home_hex);
                if !going_home && !fuelling && !dry {
                    // The reach: the least strike radius among the bomber
                    // groups aboard that can fly.
                    let reach = aboard
                        .iter()
                        .filter(|g| view.able_to_fly(g.id).unwrap_or(0) > 0)
                        .filter_map(|g| bomber_reach(view, g))
                        .min();
                    // The force's own picture: the command the task is
                    // addressed to (decision 0134, 13).
                    let picture = crate::picture::picture_of(view, task.kind.addressee());
                    let mut held_contacts: Vec<Held> = picture
                        .contacts()
                        .map(|c| Held {
                            number: picture.contact_number(c.id),
                            hex: c.estimated_hex,
                            kind: c.estimated_kind,
                            age: c.age_pulses,
                            distance_nm: Fixed::from_int(i64::from(
                                here.air_distance_nm(c.estimated_hex),
                            )),
                            in_waters: targets.waters.contains(&c.estimated_hex),
                        })
                        .collect();
                    held_contacts.sort_by_key(|c| c.number);
                    let waters_on = !targets.waters.is_empty();
                    // What the force closes on at a level, in words, with
                    // the waters where a deny gives them (decision 0190).
                    let closes_words = |level: Risk| -> String {
                        if waters_on && level != Risk::Preserve {
                            format!(
                                "{}, or any kind in the waters to strike first",
                                closes_on_words(level)
                            )
                        } else {
                            closes_on_words(level).to_string()
                        }
                    };
                    // How each contact is weighed (decision 0137, 7 and 8):
                    // one that is the force's own at the level its
                    // commander's rating reads the task's risk at, one it was
                    // told of at the task's risk. With command levels off
                    // there is no commander and no contact is its own.
                    let commander = picture.commander();
                    let initiative = commander.map_or(Initiative::Steady, |c| c.initiative);
                    let own: Vec<u32> = picture
                        .contacts()
                        .filter(|c| picture.is_own(c.id))
                        .map(|c| picture.contact_number(c.id))
                        .collect();
                    let weigh = |c: &Held| {
                        if own.contains(&c.number) {
                            weighed_at(initiative, risk)
                        } else {
                            risk
                        }
                    };
                    // The words where a rating changed how a contact was
                    // weighed (item 10), for each fresh contact it changed,
                    // and none where it changed none.
                    let weighed_words = |c: &Held| {
                        let level = weigh(c);
                        let commander = commander?;
                        (level != risk).then(|| {
                            let how = if level > risk {
                                "bolder"
                            } else {
                                "more careful"
                            };
                            format!(
                                "{}, commanding {id}, is {}: {} is {id}'s own, so he weighs it at the {} risk, one level {how} than the task's {risk_text}, and at the {} risk the force closes on {}",
                                commander.name,
                                commander.initiative.name(),
                                crate::picture::contact_words(&picture, c.number),
                                level.name(),
                                level.name(),
                                closes_words(level)
                            )
                        })
                    };
                    let rating_words: Vec<String> = held_contacts
                        .iter()
                        .filter(|c| c.age <= CLOSE_MAX_AGE)
                        .filter_map(weighed_words)
                        .collect();
                    let rating_text = if rating_words.is_empty() {
                        String::new()
                    } else {
                        format!(" {}.", rating_words.join(". "))
                    };
                    let held_words = |c: &Held| {
                        let waters_text = targets.deny_of(c.hex).map_or_else(String::new, |d| {
                            format!(
                                ", in the waters of {} that the targets limit set on {} puts first",
                                d.deny, d.limit_on
                            )
                        });
                        format!(
                            "{}, a {}, {} pulse(s) old, is {} nm from {here}{waters_text}",
                            crate::picture::contact_words(&picture, c.number),
                            c.kind.name(),
                            c.age,
                            c.distance_nm
                        )
                    };
                    match reach {
                        None => {
                            close_words = Some(format!(
                                "{id} does not close: no bomber group aboard can fly."
                            ));
                        }
                        Some(reach) => {
                            // A night at full speed, by the engine's measure.
                            let night_nm = first_wanted_by(&weigh, &held_contacts)
                                .and_then(|c| {
                                    view.forecast_passage(id, c.hex, Some(SpeedSetting::Full))
                                })
                                .map_or(Fixed::ZERO, |f| f.speed_kn * Fixed::from_int(NIGHT_HOURS));
                            let reach_words = format!(
                                    "{} nm ({night_nm} nm in the night's {NIGHT_HOURS} hours at full speed and the {reach} nm normal reach of its bombers)",
                                    night_nm + reach
                                );
                            match closing_by(risk, &weigh, reach, night_nm, &held_contacts) {
                                Closing::Preserve => {
                                    close_words = Some(format!(
                                            "{id} does not close: at the {risk_text} it closes on nothing."
                                        ));
                                }
                                Closing::NothingFresh => {
                                    close_words = Some(format!(
                                            "{id} does not close: the side holds no contact of ships {CLOSE_MAX_AGE} pulses old or less."
                                        ));
                                }
                                Closing::NothingOfKind if !rating_words.is_empty() => {
                                    // A rating changed how a contact was
                                    // weighed (decision 0137, 10).
                                    let list = held_contacts
                                        .iter()
                                        .filter(|c| c.age <= CLOSE_MAX_AGE)
                                        .map(|c| {
                                            format!(
                                                "{}, a {}, weighed at the {} risk, at which it closes on {}",
                                                crate::picture::contact_words(&picture, c.number),
                                                c.kind.name(),
                                                weigh(c).name(),
                                                closes_words(weigh(c))
                                            )
                                        })
                                        .collect::<Vec<_>>()
                                        .join("; ");
                                    close_words = Some(format!(
                                            "{id} does not close: no contact of ships {CLOSE_MAX_AGE} pulses old or less is of a kind it closes on at the risk it is weighed at ({list}).{rating_text}"
                                        ));
                                }
                                Closing::NothingOfKind => {
                                    let list = held_contacts
                                        .iter()
                                        .filter(|c| c.age <= CLOSE_MAX_AGE)
                                        .map(|c| {
                                            format!(
                                                "{}, a {}",
                                                crate::picture::contact_words(&picture, c.number),
                                                c.kind.name()
                                            )
                                        })
                                        .collect::<Vec<_>>()
                                        .join("; ");
                                    close_words = Some(format!(
                                            "{id} does not close: at the {risk_text} it closes on {}, and no contact of ships {CLOSE_MAX_AGE} pulses old or less is of that kind ({list}).",
                                            closes_words(risk)
                                        ));
                                }
                                Closing::InReach(c) if left_to_close => {
                                    // It stays while the first contact is
                                    // in reach (rule 5).
                                    stays = true;
                                    close_words = Some(format!(
                                            "{id} left its hex {hex} to close, and {}, within the {reach} nm normal reach of its bombers, the first of the contacts it closes on ({}) within the {reach_words}: it stays where it is and is given no route.{rating_text}",
                                            held_words(&c),
                                            closes_words(weigh(&c))
                                        ));
                                }
                                Closing::InReach(c) => {
                                    close_words = Some(format!(
                                            "{id} does not close: {}, within the {reach} nm normal reach of its bombers, the first of the contacts it closes on ({}) within the {reach_words}.{rating_text}",
                                            held_words(&c),
                                            closes_words(weigh(&c))
                                        ));
                                }
                                Closing::TooFar(c) => {
                                    close_words = Some(format!(
                                            "{id} does not close: {}, beyond the {reach} nm normal reach of its bombers and more than the {reach_words}.{rating_text}",
                                            held_words(&c)
                                        ));
                                }
                                Closing::CloseOn(c) => {
                                    let target = view
                                        .sea_path(here, c.hex)
                                        .and_then(|path| close_to(&path, c.hex, reach));
                                    match target {
                                        None => {
                                            close_words = Some(format!(
                                                    "{id} does not close: {}, but no sea route from {here} reaches a hex within the {reach} nm normal reach of its bombers of {}.{rating_text}",
                                                    held_words(&c),
                                                    c.hex
                                                ));
                                        }
                                        Some(target) => {
                                            let (speed, speed_text) =
                                                passage_speed(view, id, risk, target);
                                            let hours = hours_to(view, id, target, speed);
                                            let hours_text = hours.map_or_else(
                                                || {
                                                    "it does not arrive by the engine's measure"
                                                        .to_string()
                                                },
                                                |h| {
                                                    format!("{h} hours at {}", setting_words(speed))
                                                },
                                            );
                                            let target_nm = here.air_distance_nm(target);
                                            let contact_text = format!(
                                                    "{}, beyond the {reach} nm normal reach of its bombers and within the {reach_words}",
                                                    held_words(&c)
                                                );
                                            let place_text = format!(
                                                    "the first hex along the shortest sea route to {} within that reach of it is {target}, {target_nm} nm from {here}: {hours_text}",
                                                    c.hex
                                                );
                                            // Fuel first (rule 4).
                                            let fuel = home_hex.map(|home_at| {
                                                (
                                                    home_at,
                                                    fuel_needs(
                                                        view,
                                                        id,
                                                        &[target, home_at],
                                                        Some(speed),
                                                        margin,
                                                    )
                                                    .map(|(needs, _)| needs),
                                                )
                                            });
                                            match fuel {
                                                Some((_, None)) => {
                                                    close_words = Some(format!(
                                                            "{id} does not close: {contact_text}; {place_text}; but the engine would refuse the passage there and home.{rating_text}"
                                                        ));
                                                }
                                                Some((_, Some(needs)))
                                                    if needs.iter().any(|n| n.short()) =>
                                                {
                                                    close_words = Some(format!(
                                                            "{id} does not close on {}: {contact_text}; {place_text}; but the passage to {target} and from there home to {home_text}, by the engine's measure at {}, times the {margin} of the {risk_text}: {}: a ship has less than it needs.{rating_text}",
                                                            crate::picture::contact_words(&picture, c.number),
                                                            setting_words(speed),
                                                            needs_words(&needs)
                                                        ));
                                                }
                                                fuel => {
                                                    let fuel_text = match fuel {
                                                            Some((_, Some(needs))) => format!(
                                                                "the passage to {target} and from there home to {home_text}, by the engine's measure at {}, times the {margin} of the {risk_text}: {}: it has the fuel",
                                                                setting_words(speed),
                                                                needs_words(&needs)
                                                            ),
                                                            _ => format!("{id} has no home port the staff knows, so its fuel is not measured against a passage home"),
                                                        };
                                                    closes = true;
                                                    // Where a rating changed how
                                                    // the contact was weighed,
                                                    // the reason says so
                                                    // (decision 0137, 10).
                                                    let (binding, why) = match weighed_words(&c) {
                                                        None => (
                                                            format!("at the {risk_text} the force closes on {} it holds beyond its bombers' reach and within a night at full speed and that reach", closes_words(risk)),
                                                            format!("at the {risk_text} it closes on {}", closes_words(risk)),
                                                        ),
                                                        Some(words) => (
                                                            format!(
                                                                "a {} commander weighs a contact of his own at the {} risk, and at that risk the force closes on {} it holds beyond its bombers' reach and within a night at full speed and that reach",
                                                                initiative.name(),
                                                                weigh(&c).name(),
                                                                closes_words(weigh(&c))
                                                            ),
                                                            words,
                                                        ),
                                                    };
                                                    // Any other contact whose
                                                    // weighing a rating changed.
                                                    let others: Vec<String> = held_contacts
                                                        .iter()
                                                        .filter(|o| {
                                                            o.age <= CLOSE_MAX_AGE
                                                                && o.number != c.number
                                                        })
                                                        .filter_map(weighed_words)
                                                        .collect();
                                                    let others_text = if others.is_empty() {
                                                        String::new()
                                                    } else {
                                                        format!(" {}.", others.join(". "))
                                                    };
                                                    route = Some((
                                                            vec![target],
                                                            Some(speed),
                                                            serving(task, &plan)
                                                                .evidence(contact_text.clone(), today)
                                                                .evidence(fuel_text.clone(), today)
                                                                .evidence(hours_text.clone(), today)
                                                                .binding(binding)
                                                                .binding("it goes no nearer than its bombers need")
                                                                .worked(format!(
                                                                    "{intent_words}: {contact_text}: {why}. {place_text}. {fuel_text}. {id} is routed to {target} at {speed_text}.{others_text}"
                                                                ))
                                                                .would_change("the contact within reach, older, dropped or of another kind")
                                                                .would_change("less fuel aboard")
                                                                .done(),
                                                        ));
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                if route.is_none() && !going_home && !at_hex && !at_home && !stays && !dry {
                    // Its place (decision 0125, 7), at the speed of decision
                    // 0130, 1.
                    let (speed, speed_text) = passage_speed(view, id, risk, hex);
                    let back = if left_to_close {
                        ", it left the hex to close and holds nothing in reach and nothing to close on"
                    } else {
                        ""
                    };
                    route = Some((
                        vec![hex],
                        Some(speed),
                        serving(task, &plan)
                            .evidence(format!("{id} is at {here}, not at {hex}"), today)
                            .binding("the force is kept at the task's hex")
                            .worked(format!("{intent_words}: {id} is at {here}, not in the task's hex and not on its way home{back}, so it is routed to {hex} at {speed_text}."))
                            .would_change("the force in the hex")
                            .done(),
                    ));
                }
                // The words on closing join the reason it stays on its task,
                // or stand as a reason of their own.
                if let Some(words) = close_words {
                    match place_note.and_then(|i| notes.get_mut(i)) {
                        Some(note) => {
                            note.worked = format!("{} {words}", note.worked);
                        }
                        None => notes.push(
                            serving(task, &plan)
                                .evidence(format!("{id} is at {here}"), today)
                                .worked(format!("{intent_words}: {words}"))
                                .would_change("a fresh contact of a kind it closes on beyond its bombers' reach and within a night at full speed and that reach")
                                .done(),
                        ),
                    }
                }
                let give = |orders: &mut Vec<StaffOrder>, order: TaskForceOrder, reason: Reason| {
                    orders.push(StaffOrder {
                        kind: OrderKind::TaskForce { target: id, order },
                        intent: task.id,
                        reason,
                    });
                };
                // Readiness and rules of engagement (decision 0125, 9, and
                // decision 0130, 7: battle readiness on a day it closes, and
                // on one it stays within reach, as at its hex).
                if ((at_hex && !going_home) || closes || stays)
                    && force.readiness != Readiness::Battle
                {
                    let (evidence, binding, worked) = if at_hex {
                        (
                            format!("{id} is at its hex {hex}"),
                            "battle readiness while it is at its hex",
                            format!("{intent_words}: {id} is at {hex}, so it is set to battle readiness."),
                        )
                    } else {
                        (
                            format!("{id} is at {here}, closing the range"),
                            "battle readiness on a day it closes, as at its hex",
                            format!("{intent_words}: {id} closes the range, so it is set to battle readiness."),
                        )
                    };
                    give(
                        &mut orders,
                        TaskForceOrder::SetReadiness(Readiness::Battle),
                        serving(task, &plan)
                            .evidence(evidence, today)
                            .binding(binding)
                            .worked(worked)
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
                // Radio silence at preserve the force (decision 0136, 18).
                if let Some((order, reason)) =
                    crate::stand::silence_order(view, task, &plan, id, risk, today)
                {
                    give(&mut orders, order, reason);
                }
                if let Some((waypoints, speed, reason)) = route {
                    if let Some(speed) = speed.filter(|s| *s != force.speed) {
                        give(&mut orders, TaskForceOrder::SetSpeed(speed), reason.clone());
                    }
                    give(&mut orders, TaskForceOrder::route(waypoints), reason);
                }

                // The groups aboard (rules 3 to 6).
                let attacked = self.force_attacked(id);
                let mut threats: Vec<Threat> = self.threats(task.kind.addressee());
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
                // The waters to strike first of a deny that names the force
                // and is not above its task (decision 0190, 29).
                for d in targets.denies.iter().filter(|d| !d.above) {
                    for w in &d.hexes {
                        threats.push(Threat {
                            hex: *w,
                            words: format!("the waters to strike first of {} at {w}", d.deny),
                        });
                    }
                }
                let hexes: Vec<Hex> = threats.iter().map(|t| t.hex).collect();
                let arc = arc_for(here, &hexes);
                let arc_text = crate::air::arc_working(&arc, &threats);
                let figure = view
                    .air_combat_figures()
                    .map_or(0, |f| f.strike_contact_max_age_pulses);
                let mut groups: Vec<CarrierGroup> = Vec::new();
                let mut reach_of: BTreeMap<AirGroupId, Fixed> = BTreeMap::new();
                let mut held_lines: Vec<String> = aboard_waiting
                    .iter()
                    .map(|(group, day)| {
                        format!(
                            "{}: no order",
                            crate::available::named_with_day(group, *day)
                        )
                    })
                    .collect();
                for g in &aboard {
                    if let Some(hold) = self.state.hold_on_group(g.id) {
                        held_lines.push(format!("{hold} is held by the commander: no order"));
                        continue;
                    }
                    let kind = view.aircraft_type(g.aircraft_type);
                    let fighter = kind.is_some_and(|t| t.role == AircraftRole::Fighter);
                    let range = kind.map_or(Fixed::ZERO, |t| t.search_range_nm);
                    if let Some(reach) = bomber_reach(view, g) {
                        reach_of.insert(g.id, reach);
                    }
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
                        search_once: g.search_once,
                        waters: g.strike_first_waters.clone(),
                    });
                }
                let mut air = decide(
                    risk,
                    figure,
                    attacked.is_some(),
                    arc.from,
                    arc.to,
                    &targets,
                    &groups,
                );
                // Under "take a place" the bombers strike the enemy's base at
                // the place on a morning with no ship to strike (decision
                // 0148, 4 to 6): the raid's rules decide the mission and the
                // held share of the groups that go.
                let mut cover = crate::cover::CoverStrike::default();
                // Under "control an island" the task stands under the denial,
                // and the strike on the place is for the first take of the
                // tree in the island's order whose plan is released and not
                // met, its place in the bombers' reach (decision 0151, 8a).
                let end = self
                    .under_take(task)
                    .or_else(|| self.island_strike_take(task, here, &reach_of))
                    .filter(|_| released);
                if let Some(end) = end {
                    cover = self.cover_strike(
                        task, &plan, &end, here, risk, &groups, &reach_of, &air, &targets,
                    );
                    air.orders.retain(|(g, order)| {
                        !(cover.taken.contains(g)
                            && matches!(
                                order,
                                AirGroupOrder::SetAirMission(_)
                                    | AirGroupOrder::SetStrikeHeldShare { .. }
                            ))
                    });
                    if let Some(note) = cover.note.take() {
                        notes.push(note);
                    }
                }
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
                            "{}, a group that {} searches, has {fatigue}: it searches {} out to {} nm from {here} ({arc_text}); {}",
                            g.id,
                            if g.can_strike {
                                turn_words(&groups, g.id)
                            } else {
                                "can search and cannot attack a ship,".to_string()
                            },
                            arc_words(&arc),
                            g.search_range_nm,
                            carrier_once_words(risk)
                        ),
                        CarrierRole::Striker => match &targets.kinds {
                            None => format!("{}, a bomber group, has {fatigue}: it strikes ships by the table of the {risk_text}, with its strikes sized", g.id),
                            Some((kinds, on)) => format!(
                                "{}, a bomber group, has {fatigue}: it strikes ships by the table of the {risk_text}, with its strikes sized, going for {} in that order by the targets limit set on {on}{}",
                                g.id,
                                kinds_words(kinds),
                                crate::targets::wanted_waters_words(&targets)
                            ),
                        },
                        CarrierRole::Other => format!("{}, neither a fighter nor a group that can search or attack a ship: no order", g.id),
                    };
                    lines.insert(g.id, line);
                }
                for (group, line) in &cover.lines {
                    lines.insert(*group, line.clone());
                }
                // A group the look keeps on reconnaissance of the place is
                // the look's, not the carrier task's (decision 0159, 14).
                let looking: Vec<AirGroupId> = groups
                    .iter()
                    .map(|g| g.id)
                    .filter(|g| self.looking_groups.contains(g))
                    .collect();
                air.orders.retain(|(g, _)| !looking.contains(g));
                for group in &looking {
                    lines.insert(
                        *group,
                        format!("{group} looks at the place of the take this task stands under, on reconnaissance: no order"),
                    );
                }
                orders.extend(std::mem::take(&mut cover.orders));
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
                        (AirGroupOrder::SetSearchOnce { .. }, Some(b)) => format!(
                            "its search once a day is {}",
                            if b.search_once { "on" } else { "off" }
                        ),
                        (AirGroupOrder::SetTargetPriority { .. }, Some(b))
                            if targets.kinds.is_some() =>
                        {
                            format!("its target list is {}", kinds_words(&b.target_priority))
                        }
                        (AirGroupOrder::SetStrikeFirstWaters { .. }, Some(b)) => {
                            format!("it has {}", crate::targets::waters_had(&b.waters))
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
                    let mut builder = serving(task, &plan)
                        .evidence(line.clone(), today)
                        .evidence(has, today)
                        .binding(format!("rest above fatigue {level} at the {risk_text}"));
                    // The limit and the deny that shaped it (decision 0190, 6).
                    match order {
                        AirGroupOrder::SetTargetPriority { kinds } => {
                            if let Some((_, on)) = &targets.kinds {
                                builder = builder.binding(format!(
                                    "the targets limit set on {on}: {}, in that order, at every risk",
                                    kinds_words(kinds)
                                ));
                            }
                        }
                        AirGroupOrder::SetStrikeFirstWaters { .. } => {
                            builder = builder.binding(crate::targets::waters_binding(&targets));
                        }
                        _ => {}
                    }
                    let mut reason = builder
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
                    if matches!(order, AirGroupOrder::SetStrikeFirstWaters { .. }) {
                        self.waters_serve(&targets, &mut reason);
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
            search_once: false,
            waters: BTreeSet::new(),
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

    fn held(number: u32, q: i32, kind: ShipKind, age: u32) -> Held {
        let hex = Hex { q, r: 0 };
        Held {
            number,
            hex,
            kind,
            age,
            distance_nm: Fixed::from_int(i64::from(Hex { q: 0, r: 0 }.air_distance_nm(hex))),
            in_waters: false,
        }
    }

    #[test]
    fn the_closing_rule() {
        let reach = Fixed::from_int(250);
        let night = Fixed::from_int(360);
        let carrier = held(1, 12, ShipKind::Carrier, 0);
        assert_eq!(
            closing(Risk::Press, reach, night, &[carrier]),
            Closing::CloseOn(carrier)
        );
        assert_eq!(
            closing(Risk::Preserve, reach, night, &[carrier]),
            Closing::Preserve
        );
        let old = held(1, 12, ShipKind::Carrier, 4);
        assert_eq!(
            closing(Risk::Press, reach, night, &[old]),
            Closing::NothingFresh
        );
        let cruiser = held(1, 12, ShipKind::Cruiser, 0);
        assert_eq!(
            closing(Risk::Balanced, reach, night, &[cruiser]),
            Closing::NothingOfKind
        );
        assert_eq!(
            closing(Risk::Press, reach, night, &[cruiser]),
            Closing::CloseOn(cruiser)
        );
        let near = held(1, 5, ShipKind::Carrier, 0);
        assert_eq!(
            closing(Risk::Press, reach, night, &[near]),
            Closing::InReach(near)
        );
        let far = held(1, 30, ShipKind::Carrier, 0);
        assert_eq!(
            closing(Risk::Press, reach, night, &[far]),
            Closing::TooFar(far)
        );
        // The kind first, then the nearest, then the lowest number.
        let a = held(2, 11, ShipKind::Cruiser, 0);
        let b = held(3, 12, ShipKind::Carrier, 0);
        assert_eq!(
            closing(Risk::Press, reach, night, &[a, b]),
            Closing::CloseOn(b)
        );
        // The first contact decides (the architect's ruling of Nov 16, 2026):
        // a carrier in reach and a destroyer beyond, no close; a destroyer in
        // reach and a carrier beyond, close on the carrier; a destroyer
        // beyond a night does not stop the close on the carrier.
        let destroyer_far = held(4, 12, ShipKind::Destroyer, 0);
        assert_eq!(
            closing(Risk::Press, reach, night, &[near, destroyer_far]),
            Closing::InReach(near)
        );
        let destroyer_near = held(5, 5, ShipKind::Destroyer, 0);
        assert_eq!(
            closing(Risk::Press, reach, night, &[destroyer_near, carrier]),
            Closing::CloseOn(carrier)
        );
        let destroyer_beyond = held(6, 20, ShipKind::Destroyer, 0);
        assert_eq!(
            closing(Risk::Press, reach, night, &[carrier, destroyer_beyond]),
            Closing::CloseOn(carrier)
        );
        assert_eq!(first_wanted(Risk::Balanced, &[a, near]), Some(near));
        assert_eq!(first_wanted(Risk::Balanced, &[a]), None);
    }

    fn in_waters(mut c: Held) -> Held {
        c.in_waters = true;
        c
    }

    #[test]
    fn a_contact_in_the_waters_is_closed_on_first_of_any_kind_but_at_preserve() {
        let reach = Fixed::from_int(250);
        let night = Fixed::from_int(360);
        // A cruiser in the waters beyond reach; a carrier outside them in
        // reach: at balanced and at press the force leaves to close on the
        // cruiser (decision 0190, 15 and 32).
        let cruiser = in_waters(held(1, 12, ShipKind::Cruiser, 0));
        let carrier = held(2, 5, ShipKind::Carrier, 0);
        for risk in [Risk::Balanced, Risk::Press] {
            assert_eq!(
                closing(risk, reach, night, &[cruiser, carrier]),
                Closing::CloseOn(cruiser),
                "{risk:?}"
            );
        }
        assert_eq!(
            closing(Risk::Preserve, reach, night, &[cruiser, carrier]),
            Closing::Preserve
        );
        // Outside the waters a cruiser is not closed on at balanced, as today.
        let plain = held(1, 12, ShipKind::Cruiser, 0);
        assert_eq!(
            closing(Risk::Balanced, reach, night, &[plain]),
            Closing::NothingOfKind
        );
        // Two in the waters: the kind, then the nearest.
        let transport = in_waters(held(3, 11, ShipKind::Transport, 0));
        assert_eq!(
            first_wanted(Risk::Balanced, &[transport, cruiser]),
            Some(cruiser)
        );
    }

    #[test]
    fn where_it_goes_and_how_fast() {
        let path: Vec<Hex> = (1..=12).map(|q| Hex { q, r: 0 }).collect();
        assert_eq!(
            close_to(&path, Hex { q: 12, r: 0 }, Fixed::from_int(250)),
            Some(Hex { q: 6, r: 0 })
        );
        assert_eq!(
            speed_for(Risk::Balanced, Some(Fixed::from_int(15))),
            SpeedSetting::Full
        );
        assert_eq!(
            speed_for(Risk::Balanced, Some(Fixed::from_int(12))),
            SpeedSetting::Cruise
        );
        assert_eq!(
            speed_for(Risk::Preserve, Some(Fixed::from_int(15))),
            SpeedSetting::Cruise
        );
        assert_eq!(speed_for(Risk::Press, None), SpeedSetting::Full);
    }

    #[test]
    fn a_raid_raises_the_share() {
        assert_eq!(patrol_share(Risk::Balanced, true), Fixed::from_ratio(3, 4));
        assert_eq!(patrol_share(Risk::Preserve, true), Fixed::ONE);
        assert_eq!(patrol_share(Risk::Press, false), Fixed::from_ratio(1, 4));
    }
}
