//! The doctrine for "patrol the waters with submarines", at the area's level
//! (decision 0218, rules 11, 12, 34 and 54). It draws no random number and
//! keeps no state.
//!
//! Each morning, for each patrol intent in force, in priority order, whose
//! plan is released (its leaves are the same as any root's, `Day::leaves`):
//!
//! 1. **The waters** are the hexes on the map within the order's distance of
//!    its hex ([`crate::deny::waters`]).
//! 2. **The boats of the order**: those the forces limits on its chain name
//!    (their intersection), in boat number order; with none named, every boat
//!    of the side's own, in number order. A boat of place `n` in that list
//!    has station `n`.
//! 3. **The stations** are given so: the first is the centre where ships can
//!    enter it, else the navigable hex of the waters nearest the centre; each
//!    next is the navigable hex of the waters farthest from the nearest
//!    station already given; the lowest hex (q, then r) where hexes tie. When
//!    every navigable hex of the waters is a station, the stations begin
//!    again from the first. No station is on land.
//! 4. **A boat that cannot serve** (sunk, home, damaged, going home, held
//!    from the staff, held apart or before her day, or, where the order names
//!    no boat, named by the forces limit of another intent in force or used
//!    by a patrol order served before this one) is passed over, and her
//!    station goes to no one: stations do not move up.
//! 5. **The orders**: a boat whose station already is the station she would
//!    be given is given no order; the others are given the station order.
//!    One decision to give no order, with its reason, names each boat that
//!    keeps her station and each boat passed over with why.
//!
//! A boat given a station by an earlier patrol order this morning, whether
//! she was ordered or kept it, is passed over by a later one. The doctrine
//! reads the area command's picture of the side's own boats and the side's
//! command state, and nothing else.

use std::collections::BTreeMap;

use sim_core::command::{HoldTarget, Intent, IntentKind, PatrolStation, Plan, StaffOrder};
use sim_core::hex::Hex;
use sim_core::ids::{EntityRef, IntentId, SubmarineId};
use sim_core::orders::{OrderKind, SubmarineOrder};
use sim_core::state::{Submarine, SubmarineCondition};
use sim_core::view::SideView;

use crate::air::Day;
use crate::reasons::{hold_words, plan_for, serving};

/// One station of the waters, with how it was chosen.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct Station {
    pub(crate) hex: Hex,
    /// In words: "the centre of the waters", "station 2 of the waters, 2
    /// hexes from the nearest before it".
    pub(crate) how: String,
}

fn hexes(n: u32) -> String {
    if n == 1 {
        "1 hex".to_string()
    } else {
        format!("{n} hexes")
    }
}

/// The first `count` stations of `waters` about `centre` (decision 0218,
/// 54), each with how it was chosen. Fewer where the waters have no
/// navigable hex (none).
pub(crate) fn stations(
    view: &SideView<'_>,
    centre: Hex,
    waters: &[Hex],
    count: usize,
) -> Vec<Station> {
    let navigable: Vec<Hex> = waters
        .iter()
        .copied()
        .filter(|hex| view.hex(*hex).is_some_and(|data| data.is_navigable()))
        .collect();
    if navigable.is_empty() || count == 0 {
        return Vec::new();
    }
    let mut given: Vec<(Hex, String)> = Vec::new();
    // The first: the centre, else the navigable hex nearest it, the lowest
    // where they tie (`navigable` is in hex order and `min_by_key` keeps the
    // first).
    if navigable.contains(&centre) {
        given.push((centre, "the centre of the waters".to_string()));
    } else if let Some(nearest) = navigable.iter().copied().min_by_key(|h| h.distance(centre)) {
        given.push((
            nearest,
            format!(
                "the navigable hex of the waters nearest their centre ({}), {} from it",
                centre,
                hexes(nearest.distance(centre))
            ),
        ));
    }
    // Each next: the navigable hex farthest from the nearest station given;
    // the lowest where they tie.
    while given.len() < count.min(navigable.len()) {
        let mut best: Option<(u32, Hex)> = None;
        for hex in &navigable {
            if given.iter().any(|(g, _)| g == hex) {
                continue;
            }
            let near = given
                .iter()
                .map(|(g, _)| g.distance(*hex))
                .min()
                .unwrap_or(0);
            if best.is_none_or(|(far, _)| near > far) {
                best = Some((near, *hex));
            }
        }
        let Some((near, hex)) = best else { break };
        given.push((
            hex,
            format!(
                "station {} of the waters, {} from the nearest before it",
                given.len() + 1,
                hexes(near)
            ),
        ));
    }
    let unique = given.len();
    (0..count)
        .map(|n| {
            let (hex, how) = &given[n % unique];
            Station {
                hex: *hex,
                how: if n < unique {
                    how.clone()
                } else {
                    format!(
                        "station {} of the waters again, the boats being more than the hexes",
                        n % unique + 1
                    )
                },
            }
        })
        .collect()
}

/// Why a boat cannot serve a patrol order this morning, if she cannot.
fn cannot_serve(
    day: &Day<'_, '_>,
    boat: &Submarine,
    patrol: &Intent,
    named: bool,
    used: &BTreeMap<SubmarineId, IntentId>,
) -> Option<String> {
    let today = day.today;
    match boat.condition {
        SubmarineCondition::Sunk => return Some("sunk".to_string()),
        SubmarineCondition::Home => return Some("home".to_string()),
        SubmarineCondition::Damaged | SubmarineCondition::Sound => {}
    }
    if !day.area.may_use(EntityRef::Submarine(boat.id)) {
        return Some(match (boat.enters_day, boat.available_from) {
            (Some(enters), _) if enters > today => format!("enters on day {enters}"),
            (_, Some(from)) => format!("may be used from day {from}"),
            (Some(enters), None) => format!("enters on day {enters}"),
            (None, None) => "may not be used".to_string(),
        });
    }
    if boat.condition == SubmarineCondition::Damaged {
        return Some("damaged".to_string());
    }
    if boat.passage.is_some_and(|passage| passage.home) {
        return Some("going home".to_string());
    }
    let target = HoldTarget::Submarine(boat.id);
    if let Some(hold) = day.state.hold_of(target) {
        return Some(format!(
            "held by the commander: {}",
            hold_words(target, *hold)
        ));
    }
    if let Some(earlier) = used.get(&boat.id) {
        return Some(format!("used by {earlier}"));
    }
    if !named {
        if let Some(other) = day
            .state
            .intents_in_force()
            .filter(|other| other.id != patrol.id)
            .find(|other| {
                other
                    .limits
                    .forces
                    .as_ref()
                    .is_some_and(|forces| forces.submarines.contains(&boat.id))
            })
        {
            return Some(format!("used by {}", other.id));
        }
    }
    None
}

impl Day<'_, '_> {
    /// The patrol doctrine for the morning (decision 0218, 11, 12, 34 and
    /// 54).
    pub(crate) fn patrols(&mut self) {
        let ends = self.in_priority_order(|i| matches!(i.kind, IntentKind::Patrol { .. }));
        let mut used: BTreeMap<SubmarineId, IntentId> = BTreeMap::new();
        for id in ends {
            let Some(end) = self.state.intents.get(&id).cloned() else {
                continue;
            };
            if !self.released(&end) {
                continue;
            }
            let Some(plan) = self.plan_of(&end) else {
                continue;
            };
            self.patrol(&end, &plan, &mut used);
        }
    }

    #[allow(clippy::too_many_lines)]
    fn patrol(&mut self, end: &Intent, plan: &Plan, used: &mut BTreeMap<SubmarineId, IntentId>) {
        let IntentKind::Patrol { hex, distance } = end.kind else {
            return;
        };
        let in_scope = self.in_scope(end.id);
        let today = self.today;
        let area = self.area;
        let waters = crate::deny::waters(area, hex, distance);
        let waters_text = format!(
            "the waters within {distance} {} of {hex} ({} hex(es) on the map)",
            if distance == 1 { "hex" } else { "hexes" },
            waters.len()
        );
        // The boats of the order, in boat number order.
        let named_set = self.state.submarines_on_chain(end.id);
        let named = named_set.is_some();
        let mut boats: Vec<Submarine> = area
            .own_submarines()
            .filter(|boat| named_set.as_ref().is_none_or(|set| set.contains(&boat.id)))
            .cloned()
            .collect();
        boats.sort_by_key(|boat| boat.id);
        if boats.is_empty() {
            let reason = serving(end, plan)
                .evidence(
                    format!("{} has no submarine of the side's to use", end.id),
                    today,
                )
                .worked(format!(
                    "{} has no submarine of the side's own to use, so the staff gives no station.",
                    plan_for(area, end, plan)
                ))
                .would_change("a boat of the side's own, or a forces limit that names one")
                .done();
            self.target(in_scope).no_orders.push(reason);
            return;
        }
        let stations = stations(area, hex, &waters, boats.len());
        if stations.is_empty() {
            let reason = serving(end, plan)
                .evidence(
                    format!("{waters_text}: no hex of them is one ships can enter"),
                    today,
                )
                .worked(format!(
                    "{} has no hex in {waters_text} that ships can enter, so the staff gives no station.",
                    plan_for(area, end, plan)
                ))
                .would_change("waters with a hex that ships can enter")
                .done();
            self.target(in_scope).no_orders.push(reason);
            return;
        }
        let mut orders: Vec<StaffOrder> = Vec::new();
        let mut keeps: Vec<String> = Vec::new();
        let mut passed: Vec<String> = Vec::new();
        for (index, boat) in boats.iter().enumerate() {
            let place = u32::try_from(index + 1).unwrap_or(u32::MAX);
            let station = &stations[index];
            let who = format!("{} ({})", boat.id, boat.name);
            if let Some(why) = cannot_serve(self, boat, end, named, used) {
                passed.push(format!(
                    "{who} is passed over, {why}, and her place {place} station {} goes to no one",
                    station.hex
                ));
                continue;
            }
            used.insert(boat.id, end.id);
            if in_scope {
                self.patrol_stations.push(PatrolStation {
                    boat: boat.id,
                    hex: station.hex,
                    place,
                });
            }
            if boat.station == station.hex {
                keeps.push(format!(
                    "{} ({}) keeps her station at {}",
                    boat.id, boat.name, station.hex
                ));
                if in_scope {
                    self.lines
                        .push(format!("{} keeps her station at {}.", boat.id, station.hex));
                }
                continue;
            }
            let reason = serving(end, plan)
                .evidence(
                    format!(
                        "{who} is in place {place} of the boats of {}; she is at {} with her station at {}",
                        end.id, boat.hex, boat.station
                    ),
                    today,
                )
                .binding(waters_text.clone())
                .worked(format!(
                    "{} patrols {waters_text}. {who} is in place {place} of the boats it may use, so her station is {}, {}: the staff orders her to the station.",
                    end.id, station.hex, station.how
                ))
                .would_change("another boat in a place before hers, or a hold on her")
                .would_change("the waters or the boats the order names")
                .done();
            if in_scope {
                self.lines.push(format!(
                    "{} would be ordered to station {} ({}).",
                    boat.id, station.hex, station.how
                ));
            }
            orders.push(StaffOrder {
                kind: OrderKind::Submarine {
                    target: boat.id,
                    order: SubmarineOrder::Station(station.hex),
                },
                intent: end.id,
                reason,
            });
        }
        self.target(in_scope).orders.extend(orders);
        if keeps.is_empty() && passed.is_empty() {
            return;
        }
        let mut parts = Vec::new();
        if !keeps.is_empty() {
            parts.push(keeps.join("; "));
        }
        if !passed.is_empty() {
            parts.push(passed.join("; "));
        }
        let reason = serving(end, plan)
            .evidence(
                format!(
                    "{} keeps {} boat(s) on their stations and passes over {}",
                    end.id,
                    keeps.len(),
                    passed.len()
                ),
                today,
            )
            .worked(format!(
                "{} patrols {waters_text}. {}. The station of a boat passed over goes to no one.",
                end.id,
                parts.join(". ")
            ))
            .would_change("a boat that could serve, or a hold released")
            .done();
        self.target(in_scope).no_orders.push(reason);
    }
}
