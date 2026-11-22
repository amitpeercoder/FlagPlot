//! Carriers serve an order for an end (decision 0202): a commander's "deny
//! an area" or "neutralise a base" uses a free carrier force of his side, and
//! the staff chooses where it works from by the risk, the enemy and the fuel.
//!
//! **Which forces** (rules 1 to 5). Each morning, before the ends are served,
//! one pass over the commander's denies and neutralises whose plans are
//! released and whose forces limit names no task force, in priority order and
//! then by number. Each keeps the carrier task the staff chose for it
//! where it stands, withdrawing it where the force is held or an intent
//! higher in priority names the force; then takes every free carrier force
//! for which it finds a station. A force is free where it is on the carrier
//! mission, has no carrier task, no intent in force is addressed to it, no
//! forces limit in force names it, and it is not held: the island's test
//! ([`Day::may_take_force`]) over every intent in force.
//!
//! **Where** (rules 6 to 17). The candidates are the hexes the force can
//! reach by sea from which the task can be done ([`candidates`]); each enemy
//! base on the side's map casts a circle of 15 hexes and each fresh contact
//! reported as a carrier one of 8, but for what the order is aimed at
//! ([`margin_of`]); the choice by risk ([`order_by_risk`]), each candidate
//! tried for fuel in that order. The station is chosen on the morning the
//! task is formed and kept.
//!
//! What the force does there is the carrier doctrine's (`crate::carrier`):
//! its patrol, its search (with the way ahead while it goes, rule 23), its
//! strikes on ships, and under a neutralise its bombers' raid on the base
//! (`crate::cover`). This module draws no random number and adds no state.

use std::collections::{BTreeMap, BTreeSet};

use sim_core::command::{
    Caution, CautionKind, CommandKind, Intent, IntentKind, IntentLimits, Maker, Plan, Risk,
    StaffCommand,
};
use sim_core::fixed::Fixed;
use sim_core::hex::{Hex, HEX_NM};
use sim_core::ids::{BaseId, EntityRef, IntentId, TaskForceId};
use sim_core::orders::{Mission, SpeedSetting};
use sim_core::refdata::ShipKind;
use sim_core::state::{AirGroupLocation, TaskForce};

use crate::air::Day;
use crate::carrier::{speed_for, CLOSE_MAX_AGE};
use crate::neutralise::bomber_reach;
use crate::reasons::{base_words, force_held_words, serving, waters_words};
use crate::stand::{afloat, home_of, hours_to, margin, stopped, THREAT_HEXES};
use crate::turn_back::DANGER_HEXES;

// ---------------------------------------------------------------------------
// The pure parts.
// ---------------------------------------------------------------------------

/// One circle of danger (rule 8): its centre, its radius in hexes and what
/// casts it, in words.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Circle {
    pub centre: Hex,
    pub hexes: u32,
    pub words: String,
}

/// The signed margin of `hex` against `circles` (rule 10), in hex steps: the
/// least of each circle's distance less its radius. Outside every circle it
/// is the least distance to a circle's edge, above 0; inside one or more it
/// is 0 or below, the depth in the circle that holds it deepest. A hex at
/// exactly a circle's radius is inside, by 0. `None` with no circle.
pub fn margin_of(hex: Hex, circles: &[Circle]) -> Option<i64> {
    circles
        .iter()
        .map(|c| i64::from(hex.distance(c.centre)) - i64::from(c.hexes))
        .min()
}

/// Whether a margin is outside every circle.
pub fn outside(margin: Option<i64>) -> bool {
    margin.is_none_or(|m| m > 0)
}

/// One candidate as the choice reads it: its hex, its passage in hex steps by
/// sea from the force, and its margin.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Candidate {
    pub hex: Hex,
    pub passage: u32,
    pub margin: Option<i64>,
}

/// The greater margin first: no circle at all is the greatest.
fn greater_first(margin: Option<i64>) -> std::cmp::Reverse<i64> {
    std::cmp::Reverse(margin.unwrap_or(i64::MAX))
}

/// The candidates in the order they are tried for fuel at `risk` (rules 13
/// to 15): first those outside every circle, by the least passage, then the
/// greater margin, then hex order; at preserve nothing more; at balanced then
/// those inside, the least deep first, then the least passage, then hex
/// order; at press every candidate by the least passage, the greater margin,
/// then hex order. The first in fuel is the station.
pub fn order_by_risk(risk: Risk, candidates: &[Candidate]) -> Vec<Candidate> {
    let near_first = |list: &mut Vec<Candidate>| {
        list.sort_by_key(|c| (c.passage, greater_first(c.margin), c.hex));
    };
    match risk {
        Risk::Press => {
            let mut all = candidates.to_vec();
            near_first(&mut all);
            all
        }
        Risk::Preserve | Risk::Balanced => {
            let mut out: Vec<Candidate> = candidates
                .iter()
                .copied()
                .filter(|c| outside(c.margin))
                .collect();
            near_first(&mut out);
            if risk == Risk::Balanced {
                let mut inside: Vec<Candidate> = candidates
                    .iter()
                    .copied()
                    .filter(|c| !outside(c.margin))
                    .collect();
                inside.sort_by_key(|c| (greater_first(c.margin), c.passage, c.hex));
                out.extend(inside);
            }
            out
        }
    }
}

/// The radius from the aim's centre within which a candidate lies (rule 6):
/// for a neutralise the bombers' least normal strike radius; for a deny that
/// radius less 40 nm for each hex of the waters' distance, or, where that
/// leaves less than one hex, the radius itself.
pub fn reach_for(reach: Fixed, waters_hexes: Option<u32>) -> Fixed {
    let Some(distance) = waters_hexes else {
        return reach;
    };
    let less = reach - Fixed::from_int(HEX_NM * i64::from(distance));
    if less < Fixed::from_int(HEX_NM) {
        reach
    } else {
        less
    }
}

/// A compass point of `to` from `from`, in words: "east", "north-east" and
/// so on (north is decreasing `r`).
pub fn compass(from: Hex, to: Hex) -> &'static str {
    let (x, y) = sim_core::bearing::offset(from, to);
    let ax = if x < Fixed::ZERO { -x } else { x };
    let ay = if y < Fixed::ZERO { -y } else { y };
    // tan 22.5 degrees.
    let tan = Fixed::from_ratio(4142, 10000);
    if ax.is_zero() && ay.is_zero() {
        return "at";
    }
    if ay <= ax * tan {
        return if x > Fixed::ZERO { "east" } else { "west" };
    }
    if ax <= ay * tan {
        return if y > Fixed::ZERO { "north" } else { "south" };
    }
    match (x > Fixed::ZERO, y > Fixed::ZERO) {
        (true, true) => "north-east",
        (true, false) => "south-east",
        (false, true) => "north-west",
        (false, false) => "south-west",
    }
}

// ---------------------------------------------------------------------------
// The morning.
// ---------------------------------------------------------------------------

/// What the carrier pass found for one end this morning: whether a carrier
/// force serves it, and, for its words where it cannot be met, what each
/// carrier force of the side was.
#[derive(Clone, Default, Debug)]
pub(crate) struct CarrierOutcome {
    pub(crate) served: bool,
    pub(crate) words: Vec<String>,
}

/// What the end is aimed at, as the choice reads it.
pub(crate) struct Aim {
    /// The hex the reach is measured from: the base, or the centre of the
    /// waters (or its nearest sea hex).
    centre: Hex,
    /// The waters' distance for a deny, `None` for a neutralise.
    waters: Option<u32>,
    /// The hexes of a deny's waters, which cast no circle against it.
    waters_hexes: BTreeSet<Hex>,
    /// What it is, in words: "the centre of the waters, (0,0)", "base:2 (E)".
    words: String,
    /// The neutralise's target, which casts its circle.
    target: Option<sim_core::ids::BaseId>,
}

/// A station chosen, with its words.
struct Chosen {
    hex: Hex,
    words: String,
    evidence: Vec<String>,
    would_change: Vec<String>,
    /// What the commander should weigh, apart from the words (decision 0217).
    cautions: Vec<Caution>,
}

/// One candidate's fuel against its passage out and home (rule 11).
pub(crate) struct FuelRead {
    pub(crate) in_fuel: bool,
    pub(crate) words: String,
    /// The ships short, in words.
    pub(crate) short: Vec<String>,
}

impl Day<'_, '_> {
    /// Rules 1 to 5 and 24 to 28: the carrier pass of the morning, before the
    /// ends are served. It records for each end it reads what it found.
    pub(crate) fn carriers_for_ends(&mut self) {
        let ends = self.in_priority_order(|i| {
            matches!(
                i.kind,
                IntentKind::Deny { .. } | IntentKind::Neutralise { .. }
            ) && i.maker == Maker::Commander
        });
        if ends.is_empty() {
            return;
        }
        let mut forces: Vec<TaskForce> = self
            .area
            .task_forces()
            .filter(|f| f.mission == Mission::Carrier)
            .cloned()
            .collect();
        forces.sort_by_key(|f| f.id);
        // The forces the pass has given an end this morning, or that a task
        // the staff chose for an end still holds.
        let mut taken: BTreeMap<TaskForceId, IntentId> = BTreeMap::new();
        for id in ends {
            let Some(end) = self.state.intents.get(&id).cloned() else {
                continue;
            };
            // Rule 24: nothing is formed until the plan is released.
            if !self.released(&end) {
                continue;
            }
            let Some(plan) = self.plan_of(&end) else {
                continue;
            };
            // Rule 1: a forces limit that names task forces uses only those,
            // which keep their hex (decision 0125, 2).
            if self
                .state
                .nearest_forces(end.id)
                .is_some_and(|f| !f.task_forces.is_empty() || !f.ships.is_empty())
            {
                continue;
            }
            let in_scope = self.in_scope(end.id);
            let mut outcome = CarrierOutcome::default();
            self.keep_chosen(&end, &plan, in_scope, &mut taken, &mut outcome);
            for force in &forces {
                let fid = force.id;
                if let Some(other) = taken.get(&fid).copied() {
                    if other == end.id {
                        continue;
                    }
                    let words =
                        format!("{fid} is taken by {other}, higher in priority this morning");
                    if self.carrier_force_aboard(fid).is_some() {
                        let reason = serving(&end, &plan)
                            .evidence(words.clone(), self.today)
                            .binding("free carrier forces are handed out in one pass, in priority order and then by number")
                            .worked(format!(
                                "{}: the one carrier force {fid} is taken by {other}, which is served first in priority order and then by number, so the staff forms no carrier task for it under {}.",
                                end.id, end.id
                            ))
                            .would_change(format!("{} higher in priority than {other}", end.id))
                            .done();
                        self.target(in_scope).no_orders.push(reason);
                        outcome.words.push(words);
                    }
                    continue;
                }
                let Some(aboard) = self.carrier_force_aboard(fid) else {
                    continue;
                };
                if let Err(why) = self.free_carrier(force) {
                    outcome.words.push(format!("{fid} is not free: {why}"));
                    continue;
                }
                match self.choose_station(&end, force, &aboard) {
                    Ok(chosen) => {
                        self.form_station(&end, &plan, fid, &chosen, in_scope);
                        taken.insert(fid, end.id);
                        outcome.served = true;
                    }
                    Err(why) => {
                        let reason = serving(&end, &plan)
                            .evidence(why.clone(), self.today)
                            .binding(format!(
                                "the station of a carrier force by the {} risk",
                                self.state.risk_read_up(end.id).name()
                            ))
                            .worked(format!(
                                "{}: {why}, so the staff forms no carrier task for {fid}.",
                                end.id
                            ))
                            .would_change("a bolder risk level")
                            .would_change("more fuel aboard")
                            .done();
                        self.target(in_scope).no_orders.push(reason);
                        if in_scope {
                            self.lines.push(format!(
                                "{} would form no carrier task for {fid}: {why}.",
                                end.id
                            ));
                        }
                        outcome.words.push(format!("{fid}: {why}"));
                    }
                }
            }
            self.carrier_ends.insert(end.id, outcome);
        }
    }

    /// Rule 28: the carrier forces in the words of "cannot meet", where the
    /// side has any: "and no carrier force can serve it: ...". `None` where
    /// the carrier pass read no carrier force for `end`.
    pub(crate) fn carrier_words(&self, end: IntentId) -> Option<String> {
        let outcome = self.carrier_ends.get(&end)?;
        if outcome.words.is_empty() {
            return None;
        }
        Some(format!(
            "and no carrier force can serve it: {}",
            outcome.words.join("; ")
        ))
    }

    /// The carriers afloat of force `id` with an air group aboard, as the
    /// area holds them: their air groups, or `None` where it has none.
    fn carrier_force_aboard(&self, id: TaskForceId) -> Option<Vec<sim_core::state::AirGroup>> {
        let carriers: Vec<_> = afloat(self.area, id)
            .into_iter()
            .filter(|(_, _, _, kind)| *kind == ShipKind::Carrier)
            .map(|(ship, _, _, _)| ship)
            .collect();
        let aboard: Vec<_> = self
            .area
            .air_groups()
            .filter(|g| matches!(g.location, AirGroupLocation::Ship(s) if carriers.contains(&s)))
            .cloned()
            .collect();
        (!aboard.is_empty()).then_some(aboard)
    }

    /// Rule 2: `Err(words)` where the force is not free.
    fn free_carrier(&self, force: &TaskForce) -> Result<(), String> {
        let id = force.id;
        if let Some(task) = self.state.intents_in_force().find(|i| {
            i.kind.force() == Some(id) && matches!(i.kind, IntentKind::OperateCarriers { .. })
        }) {
            return Err(format!("{id} has a carrier task, {}", task.id));
        }
        let others = self.others_in_force();
        self.may_take_force(force, &others)
            .map_err(|why| why.replace(", an intent higher in priority", ", an intent in force"))
    }

    /// Rules 5, 17 and 25: each carrier task the staff chose under `end`
    /// stands, keeping its station, but where its force is held or an intent
    /// higher in priority than `end` names it: then it is withdrawn.
    fn keep_chosen(
        &mut self,
        end: &Intent,
        plan: &Plan,
        in_scope: bool,
        taken: &mut BTreeMap<TaskForceId, IntentId>,
        outcome: &mut CarrierOutcome,
    ) {
        let today = self.today;
        let tasks: Vec<Intent> = self
            .state
            .children_in_force(end.id)
            .into_iter()
            .filter_map(|c| self.state.intents.get(&c).cloned())
            .filter(|c| c.maker == Maker::Staff)
            .filter(|c| matches!(c.kind, IntentKind::OperateCarriers { .. }))
            .collect();
        let above: Vec<IntentId> = {
            let order = self.in_priority_order(|_| true);
            let at = order
                .iter()
                .position(|i| *i == end.id)
                .unwrap_or(order.len());
            order[..at].to_vec()
        };
        for task in tasks {
            let IntentKind::OperateCarriers { force: fid, hex } = task.kind else {
                continue;
            };
            let ships = self
                .area
                .task_forces()
                .find(|f| f.id == fid)
                .map(|f| f.ships.clone())
                .unwrap_or_default();
            let held = self.state.hold_on_force(fid, &ships);
            let named_above = above.iter().copied().find(|other| {
                self.state.root_of(*other) != self.state.root_of(end.id)
                    && self.state.intents.get(other).is_some_and(|i| {
                        i.addressee == sim_core::command::Addressee::TaskForce(fid)
                            || self
                                .state
                                .forces_as_bound(i)
                                .is_some_and(|f| f.task_forces.contains(&fid))
                    })
            });
            let why = match (held, named_above) {
                (Some(hold), _) => Some(format!(
                    "{hold} is held by the commander: a force held is let go"
                )),
                (None, Some(other)) => Some(format!(
                    "{other}, higher in priority than {}, names {fid}",
                    end.id
                )),
                (None, None) => None,
            };
            match why {
                Some(why) => {
                    let reason = serving(end, plan)
                        .evidence(why.clone(), today)
                        .binding(format!("{} is the staff's own task", task.id))
                        .worked(format!(
                            "{}: {why}, so the staff withdraws {}, the carrier task it chose for {fid} at {hex}.",
                            end.id, task.id
                        ))
                        .would_change("the hold released, or the force named by nothing higher")
                        .done();
                    self.target(in_scope).commands.push(StaffCommand {
                        kind: CommandKind::WithdrawIntent { intent: task.id },
                        reason,
                    });
                    if in_scope {
                        self.lines
                            .push(format!("{} would withdraw {}: {why}.", end.id, task.id));
                    }
                    for below in self.state.descendants_in_force(task.id) {
                        if let Some(found) = self.state.intents.get_mut(&below) {
                            found.withdrawn_day = Some(today);
                        }
                    }
                    if let Some(found) = self.state.intents.get_mut(&task.id) {
                        found.withdrawn_day = Some(today);
                    }
                    outcome.words.push(format!("{fid} is let go: {why}"));
                }
                None => {
                    taken.insert(fid, end.id);
                    outcome.served = true;
                    let reason = serving(end, plan)
                        .evidence(
                            format!("{}, the carrier task the staff chose for {fid} at {hex}, is in force under {}", task.id, end.id),
                            today,
                        )
                        .binding(format!("{} stands", task.id))
                        .worked(format!(
                            "{}, to operate the carrier force {fid} from {hex}, stands under {}: its station was chosen on the morning it was formed and is kept, so the staff forms no other.",
                            task.id, end.id
                        ))
                        .would_change(format!("{} withdrawn, or {fid} held", task.id))
                        .done();
                    self.target(in_scope).no_orders.push(reason);
                }
            }
        }
    }

    /// What `end` is aimed at, read from the side's map.
    pub(crate) fn aim_of(&self, end: &Intent) -> Option<Aim> {
        match end.kind {
            IntentKind::Neutralise { target } => {
                let hex = self
                    .view
                    .enemy_bases()
                    .find(|b| b.id == target)
                    .map(|b| b.hex)?;
                Some(Aim {
                    centre: hex,
                    waters: None,
                    waters_hexes: BTreeSet::new(),
                    words: format!("{} at {hex}", base_words(self.view, target)),
                    target: Some(target),
                })
            }
            IntentKind::Deny { hex, distance } => {
                let hexes: BTreeSet<Hex> = crate::deny::waters(self.view, hex, distance)
                    .into_iter()
                    .collect();
                let navigable = |h: Hex| self.view.hex(h).is_some_and(|d| d.is_navigable());
                // Where the centre is a hex ships cannot enter, the nearest
                // sea hex of the waters stands for it (rule 6).
                let centre = if navigable(hex) {
                    hex
                } else {
                    hexes
                        .iter()
                        .copied()
                        .filter(|h| navigable(*h))
                        .min_by_key(|h| (h.distance(hex), *h))?
                };
                let words = if centre == hex {
                    format!("the centre of the waters, {hex}")
                } else {
                    format!("{centre}, the nearest sea hex of the waters to their centre {hex}")
                };
                Some(Aim {
                    centre,
                    waters: Some(distance),
                    waters_hexes: hexes,
                    words,
                    target: None,
                })
            }
            _ => None,
        }
    }

    /// The enemy bases on the side's map, in the order their circles stand
    /// first among the circles of [`Day::circles_for`]: the base of circle
    /// `i` is entry `i` (decision 0217, rule 4).
    fn circle_bases(&self) -> Vec<(BaseId, Hex)> {
        let side = self.view.side();
        self.view
            .enemy_bases()
            .filter(|b| b.side != side)
            .map(|b| (b.id, b.hex))
            .collect()
    }

    /// Rules 7 to 9: the circles of danger, as the area's picture holds the
    /// enemy, but for what the order is aimed at.
    pub(crate) fn circles_for(&self, aim: &Aim) -> Vec<Circle> {
        let mut circles: Vec<Circle> = self
            .circle_bases()
            .into_iter()
            .map(|(id, hex)| Circle {
                centre: hex,
                hexes: THREAT_HEXES,
                words: format!("{} at {}", base_words(self.view, id), hex),
            })
            .collect();
        let picture = crate::picture::picture_of(self.view, sim_core::command::Addressee::Area);
        let mut contacts: Vec<_> = picture
            .contacts()
            .filter(|c| c.estimated_kind == ShipKind::Carrier && c.age_pulses <= CLOSE_MAX_AGE)
            .filter(|c| !aim.waters_hexes.contains(&c.estimated_hex))
            .map(|c| {
                let number = picture.contact_number(c.id);
                (number, c.estimated_hex, c.age_pulses)
            })
            .collect();
        contacts.sort_unstable();
        for (number, hex, age) in contacts {
            circles.push(Circle {
                centre: hex,
                hexes: DANGER_HEXES,
                words: format!(
                    "{}, reported as a carrier, {age} pulse(s) old, at {hex}",
                    crate::picture::contact_words(&picture, number)
                ),
            });
        }
        circles
    }

    /// Rules 6 to 16, 26 and 27: the station of `force` for `end`, or why
    /// there is none.
    #[allow(clippy::too_many_lines)]
    fn choose_station(
        &self,
        end: &Intent,
        force: &TaskForce,
        aboard: &[sim_core::state::AirGroup],
    ) -> Result<Chosen, String> {
        let id = force.id;
        let held = force_held_words(self.area, id);
        let risk = self.state.risk_read_up(end.id);
        let risk_text = format!("the {} risk", risk.name());
        let here = force.position();
        let bombers: Vec<(sim_core::ids::AirGroupId, Fixed)> = aboard
            .iter()
            .filter_map(|g| bomber_reach(self.view, g).map(|r| (g.id, r)))
            .collect();
        let Some(reach) = bombers.iter().map(|(_, r)| *r).min() else {
            return Err(format!("{held} has no bomber group aboard"));
        };
        let sets = bombers
            .iter()
            .filter(|(_, r)| *r == reach)
            .map(|(g, _)| g.to_string())
            .collect::<Vec<_>>()
            .join(" and ");
        let Some(aim) = self.aim_of(end) else {
            return Err(format!(
                "what {} is aimed at is not on the side's map",
                end.id
            ));
        };
        let within = reach_for(reach, aim.waters);
        let why_within = match aim.waters {
            None => format!("the least normal strike radius of the bomber groups aboard {id}, set by {sets}"),
            Some(d) if within == reach => format!(
                "the least normal strike radius of the bomber groups aboard {id}, set by {sets}: less 40 nm for each of the waters' {d} hexes it would leave less than one hex, so the waters are wider than the bombers can cover from anywhere"
            ),
            Some(d) => format!(
                "the {reach} nm least normal strike radius of the bomber groups aboard {id}, set by {sets}, less 40 nm for each of the waters' {d} hexes, so that all the waters are in reach"
            ),
        };
        let within_words = format!("{within} nm ({why_within})");
        // One sea distance from the force's hex to every hex (rule 12).
        let field = self.view.sea_distances_to(here);
        let circles = self.circles_for(&aim);
        let candidates: Vec<Candidate> = field
            .iter()
            .filter(|(hex, _)| hex.air_distance_within(aim.centre, within))
            .map(|(hex, steps)| Candidate {
                hex: *hex,
                passage: *steps,
                margin: margin_of(*hex, &circles),
            })
            .collect();
        if candidates.is_empty() {
            return Err(format!(
                "no hex within {within_words} of {} can be reached by sea from {here}, where {held} lies",
                aim.words
            ));
        }
        let order = order_by_risk(risk, &candidates);
        if order.is_empty() {
            let holding: Vec<String> = circles
                .iter()
                .filter(|c| {
                    candidates
                        .iter()
                        .any(|k| k.hex.distance(c.centre) <= c.hexes)
                })
                .map(|c| format!("the circle of {} hexes round {}", c.hexes, c.words))
                .collect();
            let raid = if aim.target.is_some() {
                format!(
                    "; a raid on {} is flown from inside its own circle",
                    aim.words
                )
            } else {
                String::new()
            };
            return Err(format!(
                "at {risk_text} a carrier force stands only outside every circle of danger, and of the candidates, {} of them, within {within_words} of {}, none is outside: {} hold every one{raid}",
                candidates.len(),
                aim.words,
                holding.join(" and ")
            ));
        }
        // Fuel, in the order the candidates are tried (rule 11).
        let home = home_of(self.view, force).map(|(b, _)| b);
        let home_hex = home.and_then(|b| self.view.bases().find(|x| x.id == b).map(|x| x.hex));
        let home_field = home_hex.map(|h| self.view.sea_distances_to(h));
        let full = self.weighed_full(force);
        let mut first_short: Option<(Hex, FuelRead)> = None;
        let mut chosen: Option<(Candidate, FuelRead)> = None;
        // Decision 0218, 55: candidates in fuel but in submarine waters, in
        // the order tried. The first outside them is the station; where none
        // is outside, the first of these.
        let mut passed: Vec<(Candidate, String)> = Vec::new();
        let mut first_in_waters: Option<(Candidate, FuelRead)> = None;
        for candidate in &order {
            let read = self.fuel_for(
                force,
                candidate.hex,
                risk,
                home.zip(home_hex),
                home_field.as_ref(),
                full,
            );
            if read.in_fuel {
                match crate::waters::in_submarine_waters(self.area, candidate.hex) {
                    None => {
                        chosen = Some((*candidate, read));
                        break;
                    }
                    Some(why) => {
                        passed.push((*candidate, why));
                        if first_in_waters.is_none() {
                            first_in_waters = Some((*candidate, read));
                        }
                        continue;
                    }
                }
            }
            if first_short.is_none() {
                first_short = Some((candidate.hex, read));
            }
        }
        let mut none_outside = false;
        if chosen.is_none() {
            if let Some(found) = first_in_waters {
                chosen = Some(found);
                none_outside = true;
                passed.clear();
            }
        }
        let Some((station, fuel)) = chosen else {
            let (hex, read) = first_short.expect("one candidate was tried");
            return Err(format!(
                "no candidate within {within_words} of {} is in fuel at {risk_text}: the cheapest tried, {hex}: {}; short: {}",
                aim.words,
                read.words,
                read.short.join(", ")
            ));
        };
        let submarines = if none_outside {
            format!(
                "; no place outside submarine waters serves, so the station is {}, {}",
                station.hex,
                crate::waters::in_submarine_waters(self.area, station.hex)
                    .expect("the station is in submarine waters")
            )
        } else {
            passed
                .iter()
                .map(|(c, why)| format!("; {} passed over: {why}", c.hex))
                .collect()
        };
        // The words of the choice.
        let circle_words: Vec<String> = circles
            .iter()
            .map(|c| {
                let d = station.hex.distance(c.centre);
                if d <= c.hexes {
                    format!(
                        "inside the circle of {} hexes round {}, {d} hexes from it, by {}",
                        c.hexes,
                        c.words,
                        c.hexes - d
                    )
                } else {
                    format!(
                        "outside the circle of {} hexes round {}, {d} hexes from it, by {}",
                        c.hexes,
                        c.words,
                        d - c.hexes
                    )
                }
            })
            .collect();
        let danger = if circle_words.is_empty() {
            "no circle of danger reaches it: the side knows no enemy base and holds no fresh contact reported as a carrier".to_string()
        } else if outside(station.margin) {
            format!("it lies {}", circle_words.join("; "))
        } else {
            format!("it stands in danger: it lies {}", circle_words.join("; "))
        };
        let nearest = candidates
            .iter()
            .min_by_key(|c| (c.passage, c.hex))
            .copied()
            .expect("there is a candidate");
        let nearer = if nearest.hex == station.hex
            || nearest.passage == station.passage
            || passed.iter().any(|(c, _)| c.hex == nearest.hex)
        {
            String::new()
        } else if outside(nearest.margin) {
            format!(
                "; a nearer station, {}, {} hexes of passage, is not in fuel",
                nearest.hex, nearest.passage
            )
        } else {
            let inside: Vec<String> = circles
                .iter()
                .filter(|c| nearest.hex.distance(c.centre) <= c.hexes)
                .map(|c| format!("the circle round {}", c.words))
                .collect();
            format!(
                "; a nearer station, {}, {} hexes of passage, lies inside {}",
                nearest.hex,
                nearest.passage,
                inside.join(" and ")
            )
        };
        let from_aim = station.hex.distance(aim.centre);
        let where_words = format!(
            "{}, {} nm ({from_aim} hexes) {} of {}",
            station.hex,
            station.hex.air_distance_nm(aim.centre),
            compass(aim.centre, station.hex),
            aim.words
        );
        let passage_words = if station.passage == 0 {
            format!("it is the hex {held} lies in")
        } else {
            format!(
                "{} hexes of passage by sea from {here}, where {held} lies",
                station.passage
            )
        };
        let choice = match risk {
            Risk::Preserve => "at the preserve risk the station is outside every circle, with the least passage",
            Risk::Balanced if outside(station.margin) => "at the balanced risk the station is outside every circle where one is in fuel, with the least passage",
            Risk::Balanced => "at the balanced risk, with no candidate in fuel outside every circle, the station is the one least deep in danger, then with the least passage",
            Risk::Press => "at the press risk the station is the one with the least passage, in danger or out",
        };
        let words = format!(
            "{held} is free: on the carrier mission, with no task and named by nothing of the commander's, and not held. The candidates, {} of them, are the hexes the force can reach by sea within {within_words} of {}; {choice}. The station is {where_words}: {passage_words}; {danger}{nearer}{submarines}. Fuel: {}.",
            candidates.len(),
            aim.words,
            fuel.words
        );
        let mut would_change = vec![format!("another risk level than {risk_text}")];
        if circles.iter().any(|c| c.hexes == DANGER_HEXES) {
            would_change
                .push("the contact reported as a carrier growing old or dropped".to_string());
        }
        would_change.push("more fuel aboard".to_string());
        // Decision 0217, rule 4: the cautions, said apart from the words.
        let bases = self.circle_bases();
        let mut cautions: Vec<Caution> = circles
            .iter()
            .enumerate()
            .filter(|(_, c)| station.hex.distance(c.centre) <= c.hexes)
            .map(|(i, c)| Caution {
                kind: CautionKind::StationInDanger,
                of: bases.get(i).map(|(id, _)| EntityRef::Base(*id)),
                hex: Some(c.centre),
                words: format!(
                    "the station {} lies inside the circle of {} hexes round {}",
                    station.hex, c.hexes, c.words
                ),
            })
            .collect();
        if none_outside {
            cautions.push(Caution {
                kind: CautionKind::StationInSubmarineWaters,
                of: None,
                hex: Some(station.hex),
                words: format!(
                    "the station {} lies in submarine waters, and no place outside them serves",
                    station.hex
                ),
            });
        }
        if home.is_none() {
            cautions.push(Caution {
                kind: CautionKind::FuelNotMeasured,
                of: Some(EntityRef::TaskForce(id)),
                hex: None,
                words: format!(
                    "{id} has no home port the staff knows, so its fuel was not measured"
                ),
            });
        }
        Ok(Chosen {
            hex: station.hex,
            words,
            evidence: vec![
                format!("{held} lies at {here} with bombers of a least normal strike radius of {reach} nm"),
                format!("the candidates within {within} nm of {}: {}", aim.words, candidates.len()),
            ],
            would_change,
            cautions,
        })
    }

    /// Rule 11: a force stopped at a base of the side that can fuel it is
    /// weighed with its ships full.
    pub(crate) fn weighed_full(&self, force: &TaskForce) -> bool {
        if !stopped(force) {
            return false;
        }
        let here = force.position();
        self.view.bases().any(|b| {
            b.hex == here && b.fuel_t > b.fuel_reserve_t && crate::stand::fuels_at(self.view, here)
        })
    }

    /// Rule 11: the fuel of the passage to `to` at the carrier doctrine's
    /// speed and from it home at cruise, times the margin of the risk, for
    /// every ship of the force.
    fn fuel_for(
        &self,
        force: &TaskForce,
        to: Hex,
        risk: Risk,
        home: Option<(sim_core::ids::BaseId, Hex)>,
        home_field: Option<&BTreeMap<Hex, u32>>,
        full: bool,
    ) -> FuelRead {
        let id = force.id;
        if home.is_none() {
            return no_home_read(id);
        }
        let here = force.position();
        // The passage out (decision 0130, 1, as the carrier doctrine sails).
        let (out_nm, knots, speed) = if to == here && stopped(force) {
            (Fixed::ZERO, Fixed::ZERO, SpeedSetting::Cruise)
        } else {
            let speed = if risk == Risk::Preserve {
                SpeedSetting::Cruise
            } else {
                speed_for(risk, hours_to(self.view, id, to, SpeedSetting::Cruise))
            };
            match self.view.forecast_route(id, &[to], Some(speed)) {
                Some(f) if f.refusal.is_none() => (f.route_nm, f.speed_kn, speed),
                _ => {
                    return FuelRead {
                        in_fuel: false,
                        words: format!("the engine refuses a passage of {id} to {to}"),
                        short: vec![format!("{id} as a whole")],
                    }
                }
            }
        };
        self.fuel_read(
            force,
            to,
            (out_nm, knots, speed),
            risk,
            home,
            home_field,
            full,
        )
    }

    /// Rule 11 on a passage out already measured, `out` its charged miles,
    /// the knots it is steamed at and the setting: what [`Day::fuel_for`]
    /// reads, and decision 0203's rule 14 on a way's own miles.
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn fuel_read(
        &self,
        force: &TaskForce,
        to: Hex,
        out: (Fixed, Fixed, SpeedSetting),
        risk: Risk,
        home: Option<(sim_core::ids::BaseId, Hex)>,
        home_field: Option<&BTreeMap<Hex, u32>>,
        full: bool,
    ) -> FuelRead {
        let id = force.id;
        let (out_nm, knots, speed) = out;
        let Some((home, home_hex)) = home else {
            return no_home_read(id);
        };
        let home_text = base_words(self.view, home);
        let refused = |words: String| FuelRead {
            in_fuel: false,
            words,
            short: vec![format!("{id} as a whole")],
        };
        let Some(home_nm) = home_field
            .and_then(|field| self.view.sea_charged_nm_in(field, to, home_hex))
            .map(|nm| Fixed::from_int(i64::from(nm)))
        else {
            return refused(format!("no sea route from {to} home to {home_text}"));
        };
        let m = margin(risk);
        let mut lines: Vec<String> = Vec::new();
        let mut short: Vec<String> = Vec::new();
        let mut least: Option<(Fixed, sim_core::ids::ShipId)> = None;
        for (ship, fuel, capacity, _) in afloat(self.area, id) {
            let Some(class) = self
                .view
                .ships()
                .find(|s| s.id == ship)
                .and_then(|s| self.view.ship_class(s.class))
            else {
                continue;
            };
            let out = crate::night::burn_of(self.view, ship, out_nm, knots).unwrap_or(Fixed::ZERO);
            let back = home_nm.mul_div(class.fuel_capacity_t, class.endurance_nm);
            let need = (out + back) * m;
            let aboard = if full { capacity } else { fuel };
            if least.is_none_or(|(l, _)| aboard < l) {
                least = Some((aboard, ship));
            }
            lines.push(format!(
                "{ship} needs {} t ({out} t out and {back} t home, times {m})",
                need
            ));
            if aboard < need {
                short.push(format!("{ship}, {aboard} t aboard against {need} t needed"));
            }
        }
        let speed_words = match speed {
            SpeedSetting::Full => "full speed",
            _ => "cruise",
        };
        let aboard_words = match least {
            Some((l, ship)) if full => format!(
                "{id} is stopped at a base that can fuel it and fuels before it sails, so its ships are weighed full; the least is {ship}'s {l} t"
            ),
            Some((l, ship)) => format!("the least aboard is {ship}'s {l} t"),
            None => "no ship is afloat".to_string(),
        };
        FuelRead {
            in_fuel: short.is_empty(),
            words: format!(
                "the passage out to {to}, {out_nm} nm at {speed_words}, and home to {home_text}, {home_nm} nm at cruise, times the {m} of the {} risk: {}; {aboard_words}",
                risk.name(),
                lines.join(", ")
            ),
            short,
        }
    }

    /// The home port of `force` as the fuel test reads it (rule 11): the
    /// base and its hex, and the sea distances to it.
    pub(crate) fn fuel_home(
        &self,
        force: &TaskForce,
    ) -> (Option<HomePort>, Option<BTreeMap<Hex, u32>>) {
        let home = home_of(self.view, force).map(|(b, _)| b);
        let home_hex = home.and_then(|b| self.view.bases().find(|x| x.id == b).map(|x| x.hex));
        let home_field = home_hex.map(|h| self.view.sea_distances_to(h));
        (home.zip(home_hex), home_field)
    }

    /// Rule 18: the task to operate carriers at the station, formed under
    /// the end.
    fn form_station(
        &mut self,
        end: &Intent,
        plan: &Plan,
        force: TaskForceId,
        chosen: &Chosen,
        in_scope: bool,
    ) {
        let today = self.today;
        let kind = IntentKind::OperateCarriers {
            force,
            hex: chosen.hex,
        };
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
        let aim = match end.kind {
            IntentKind::Deny { hex, distance } => waters_words(hex, distance),
            IntentKind::Neutralise { target } => {
                format!("neutralise {}", base_words(self.view, target))
            }
            _ => String::new(),
        };
        let mut builder = serving(end, plan);
        for e in &chosen.evidence {
            builder = builder.evidence(e.clone(), today);
        }
        builder = builder
            .binding(format!("{} is to {aim} and its plan is released", end.id))
            .binding("a free carrier force serves an order for an end, at a station chosen by the risk, the enemy and the fuel")
            .worked(format!(
                "{} is to {aim}: {} The staff forms {id_new}, to operate the carrier force {force} from {}, under it, for {force} to carry out.",
                end.id, chosen.words, chosen.hex
            ));
        for w in &chosen.would_change {
            builder = builder.would_change(w.clone());
        }
        let reason = builder
            .would_change(format!("{force} held, or named by another order"))
            .cautions(chosen.cautions.clone())
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
                "{} would form {id_new}, to operate the carrier force {force} from {}, under it: {}",
                end.id, chosen.hex, chosen.words
            ));
        }
    }

    /// Rule 23: whether `task` is a carrier task the staff chose a station
    /// for: its own, under a commander's deny or neutralise whose forces
    /// limit names no task force.
    pub(crate) fn station_chosen(&self, task: &Intent) -> bool {
        if task.maker != Maker::Staff {
            return false;
        }
        let Some(parent) = task.parent.and_then(|p| self.state.intents.get(&p)) else {
            return false;
        };
        matches!(
            parent.kind,
            IntentKind::Deny { .. } | IntentKind::Neutralise { .. }
        ) && parent.maker == Maker::Commander
            && !self
                .state
                .nearest_forces(parent.id)
                .is_some_and(|f| !f.task_forces.is_empty() || !f.ships.is_empty())
    }
}

impl Day<'_, '_> {
    /// Rule 21: the target of the commander's neutralise a carrier task the
    /// staff chose stands under, where it is an enemy base on the side's map.
    pub(crate) fn neutralise_aim(&self, task: &Intent) -> Option<crate::cover::StrikeAim> {
        if !self.station_chosen(task) {
            return None;
        }
        let parent = task.parent.and_then(|p| self.state.intents.get(&p))?;
        let IntentKind::Neutralise { target } = parent.kind else {
            return None;
        };
        let hex = self
            .view
            .enemy_bases()
            .find(|b| b.id == target)
            .map(|b| b.hex)?;
        Some(crate::cover::StrikeAim {
            base: target,
            hex,
            of: format!("the base {} is to neutralise", parent.id),
            binding: "the base to neutralise".to_string(),
        })
    }
}

/// A home port and its hex.
pub(crate) type HomePort = (sim_core::ids::BaseId, Hex);

/// The fuel read of a force with no home port the staff knows (rule 11).
fn no_home_read(id: TaskForceId) -> FuelRead {
    FuelRead {
        in_fuel: true,
        words: format!("{id} has no home port the staff knows, so its fuel could not be measured against a passage home"),
        short: Vec::new(),
    }
}

/// The hexes of the way ahead for the coming day (rule 23): the hexes of the
/// sea path from `here` to `station`, as many as a day at `knots` covers, and
/// the station itself.
pub(crate) fn way_ahead(path: &[Hex], station: Hex, knots: Fixed) -> Vec<Hex> {
    let day_nm = knots * Fixed::from_int(24);
    let steps = (day_nm / Fixed::from_int(HEX_NM)).trunc();
    let steps = usize::try_from(steps.max(0)).unwrap_or(usize::MAX);
    let mut way: Vec<Hex> = path.iter().copied().take(steps).collect();
    if !way.contains(&station) {
        way.push(station);
    }
    way
}

#[cfg(test)]
mod tests {
    use super::*;

    fn circle(q: i32, r: i32, hexes: u32) -> Circle {
        Circle {
            centre: Hex::new(q, r),
            hexes,
            words: String::new(),
        }
    }

    #[test]
    fn the_margin_is_signed_and_a_hex_on_the_edge_is_inside() {
        let c = [circle(8, -6, 8)];
        assert_eq!(margin_of(Hex::new(3, 0), &c), Some(-2));
        assert_eq!(margin_of(Hex::new(0, 3), &c), Some(1));
        assert!(!outside(margin_of(Hex::new(0, 2), &c)));
        assert_eq!(margin_of(Hex::new(0, 0), &[]), None);
        assert!(outside(None));
    }

    #[test]
    fn the_choice_by_risk() {
        let a = Candidate {
            hex: Hex::new(3, 0),
            passage: 7,
            margin: Some(-2),
        };
        let b = Candidate {
            hex: Hex::new(0, 3),
            passage: 10,
            margin: Some(1),
        };
        let c = Candidate {
            hex: Hex::new(1, 1),
            passage: 9,
            margin: Some(-5),
        };
        assert_eq!(order_by_risk(Risk::Preserve, &[a, b, c]), vec![b]);
        assert_eq!(order_by_risk(Risk::Balanced, &[a, b, c]), vec![b, a, c]);
        assert_eq!(order_by_risk(Risk::Press, &[a, b, c]), vec![a, c, b]);
        assert_eq!(order_by_risk(Risk::Balanced, &[a, c]), vec![a, c]);
    }

    #[test]
    fn the_reach_of_a_deny() {
        let r = Fixed::from_int(210);
        assert_eq!(reach_for(r, None), r);
        assert_eq!(reach_for(r, Some(2)), Fixed::from_int(130));
        assert_eq!(reach_for(r, Some(6)), r);
        assert_eq!(reach_for(r, Some(5)), r);
        assert_eq!(reach_for(r, Some(4)), Fixed::from_int(50));
    }

    #[test]
    fn the_compass() {
        assert_eq!(compass(Hex::new(0, 0), Hex::new(3, 0)), "east");
        assert_eq!(compass(Hex::new(0, 0), Hex::new(3, -3)), "north-east");
        assert_eq!(compass(Hex::new(0, 0), Hex::new(-3, 0)), "west");
    }

    #[test]
    fn the_way_ahead() {
        let path: Vec<Hex> = (3..=9).rev().map(|q| Hex::new(q, 0)).collect();
        let way = way_ahead(&path, Hex::new(3, 0), Fixed::from_int(30));
        assert_eq!(way.len(), 7);
        let short = way_ahead(&path, Hex::new(3, 0), Fixed::from_int(5));
        assert_eq!(
            short,
            vec![
                Hex::new(9, 0),
                Hex::new(8, 0),
                Hex::new(8 - 1, 0),
                Hex::new(3, 0)
            ]
        );
    }
}
