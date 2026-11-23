//! A lift turned back (decision 0191, as amended on review and in the
//! build): each morning, a lift of a released "take a place" under way with
//! men aboard and not yet landing reads the danger to it, enemy carriers
//! reported near its way to the place, and whether it has been hurt since
//! the plan was released. By the risk it is turned back to its source, kept
//! back, or let go on.
//!
//! - **The danger** (rules 2, 17 and 18): a contact of the picture the take
//!   reads, reported as a carrier, no more than [`FRESH_PULSES`] old, within
//!   [`DANGER_HEXES`] of the lift's hex or of any hex of the engine's sea
//!   path from the lift to the place.
//! - **Hurt** (rules 15 and 16): since the plan was released, a ship of the
//!   lift's force sunk, or of a force that keeps or kept station on it as
//!   its escort, or any carrier of the side; read from the side's events.
//! - **By the risk** (rule 4): preserve the force turns back on the danger;
//!   balanced once hurt; press never.
//! - **Turning back** (rules 5 and 13): routed to its source at cruise, its
//!   order to unload taken away by the order to do no cargo work. The
//!   notice `lift_turned_back` is handed in that morning.
//! - **Going on** (rules 14 and 19): the take's own step sends a lift on
//!   once no contact that makes the danger has been reported in the last
//!   [`CLEAR_PULSES`]: the second clear morning running.
//! - **No state** (rule 12): a lift is turned back when it is under way with
//!   men aboard and the place is not a hex it is to act at on its route.

use sim_core::command::LimitKind;
use sim_core::command::PlanState;
use sim_core::command::{Intent, NoticeBasis, NoticeKind, Plan, Risk, StaffNotice, StaffOrder};
use sim_core::events::EventKind;
use sim_core::hex::{Hex, HEX_NM};
use sim_core::ids::{BaseId, TaskForceId};
use sim_core::orders::{OrderKind, SpeedSetting, TaskForceOrder};
use sim_core::refdata::ShipKind;
use sim_core::side_events::SideEventKind;
use sim_core::state::{ShipLocation, TaskForce, TaskForceStatus};

use crate::air::Day;
use crate::reasons::{hold_words, serving};

/// The danger's distance (decision 0191, 18): 320 nm, an aircraft's reach
/// and a night's steaming toward the lift. The staff's figure, a
/// placeholder; not the supply run's threat of 15 hexes.
pub(crate) const DANGER_HEXES: u32 = 8;

/// A fresh contact (decision 0191, 2): no more than 3 pulses old, the
/// staff's figure for a fresh contact (decision 0130).
pub(crate) const FRESH_PULSES: u32 = crate::carrier::CLOSE_MAX_AGE;

/// Two clear mornings (decision 0191, 19): a lift kept back goes on once no
/// contact that makes the danger has been reported in the last 6 pulses.
pub(crate) const CLEAR_PULSES: u32 = 2 * FRESH_PULSES;

/// A contact as the danger reads it: its number in the picture, the words
/// that name it, its age in pulses and its distance in hexes from the
/// nearest hex of the lift's way.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct Seen {
    pub(crate) number: u32,
    pub(crate) words: String,
    pub(crate) kind: ShipKind,
    pub(crate) age: u32,
    pub(crate) hexes: u32,
}

/// The contact that makes the danger among `contacts`, of those no more
/// than `max_age` pulses old, reported as a carrier and within
/// [`DANGER_HEXES`]: the nearest, then the newest, then the lowest number.
pub(crate) fn danger_of(contacts: &[Seen], max_age: u32) -> Option<Seen> {
    contacts
        .iter()
        .filter(|c| c.kind == ShipKind::Carrier)
        .filter(|c| c.age <= max_age)
        .filter(|c| c.hexes <= DANGER_HEXES)
        .min_by_key(|c| (c.hexes, c.age, c.number))
        .cloned()
}

/// Rule 4: whether a lift turns back on the danger at `risk`, `hurt` or not.
pub(crate) fn turns(risk: Risk, hurt: bool) -> bool {
    match risk {
        Risk::Preserve => true,
        Risk::Balanced => hurt,
        Risk::Press => false,
    }
}

/// The distance in hexes from `hex` to the nearest of `way`.
pub(crate) fn hexes_from(hex: Hex, way: &[Hex]) -> u32 {
    way.iter()
        .map(|h| hex.distance(*h))
        .min()
        .unwrap_or(u32::MAX)
}

fn nm_words(hexes: u32) -> String {
    format!(
        "{hexes} {} ({} nm)",
        if hexes == 1 { "hex" } else { "hexes" },
        i64::from(hexes) * HEX_NM
    )
}

fn pulses_words(age: u32) -> String {
    format!("{age} {}", if age == 1 { "pulse" } else { "pulses" })
}

impl Seen {
    fn danger_words(&self) -> String {
        format!(
            "{}, reported as a carrier, {} old, is {} from the lift's way to the place, within {DANGER_HEXES}",
            self.words,
            pulses_words(self.age),
            nm_words(self.hexes)
        )
    }
}

/// What the rule makes of a lift this morning.
pub(crate) enum Reading {
    /// Not a lift the rule reads, or no danger and nothing to say: the take
    /// goes on as it would have.
    Untouched,
    /// The rule has handed in its decision: the take gives nothing more.
    Decided,
}

/// The danger and the hurt as read this morning.
struct Morning {
    risk: Risk,
    /// The contact that makes the danger now (fresh), if any.
    now: Option<Seen>,
    /// The newest contact that makes the danger in the last 6 pulses.
    recent: Option<Seen>,
    /// Any contact reported as a carrier within the distance, of any age.
    any: Option<Seen>,
    /// The ships lost since the release, in words; empty where none.
    lost: Vec<String>,
}

impl Morning {
    fn hurt(&self) -> bool {
        !self.lost.is_empty()
    }

    fn turns(&self) -> bool {
        self.now.is_some() && turns(self.risk, self.hurt())
    }

    fn kept(&self) -> bool {
        self.recent.is_some() && turns(self.risk, self.hurt())
    }

    fn risk_words(&self) -> String {
        format!("at {} risk", self.risk.name())
    }

    fn lost_words(&self) -> String {
        if self.lost.is_empty() {
            "no ship of the lift, of its escort or carrier of the side has been sunk since the plan was released".to_string()
        } else {
            format!("since the plan was released {} ", self.lost.join("; "))
                + if self.lost.len() == 1 {
                    "has been sunk"
                } else {
                    "have been sunk"
                }
        }
    }

    /// Why a threatened lift is not turned back (rule 8).
    fn not_turned_words(&self) -> String {
        match self.risk {
            Risk::Press => "at press risk a lift is never turned back".to_string(),
            Risk::Balanced => format!(
                "at balanced risk a lift turns back on the danger only once it has been hurt, and {}",
                self.lost_words()
            ),
            Risk::Preserve => String::new(),
        }
    }
}

impl Day<'_, '_> {
    /// The men aboard the ships of `force`.
    fn men_aboard_force(&self, force: &TaskForce) -> u32 {
        self.view
            .ships()
            .filter(|s| s.location == ShipLocation::TaskForce(force.id))
            .filter_map(|s| s.troops)
            .map(|t| t.men)
            .sum()
    }

    /// The lift's way to the place (decision 0191, 17): its hex and each hex
    /// of the engine's sea path from it to the place, along the route the
    /// take would give it, and not the way home.
    fn way_to(&self, end: &Intent, force: &TaskForce, place: Hex) -> Vec<Hex> {
        let from = force.position();
        let mut way = vec![from];
        let area = self.state.area_read_up(end.id);
        let waypoints = crate::route::route_between(self.view, from, place, area.as_ref())
            .map_or_else(|| vec![place], |r| r.out);
        let mut here = from;
        for waypoint in waypoints {
            match self.view.sea_path(here, waypoint) {
                Some(path) => way.extend(path),
                None => way.push(waypoint),
            }
            here = waypoint;
        }
        way
    }

    /// The contacts of the picture the take reads (decision 0134, 13), as
    /// the danger reads them against `way`.
    fn seen_against(&self, end: &Intent, way: &[Hex]) -> Vec<Seen> {
        let picture = crate::picture::picture_of(self.view, end.kind.addressee());
        let mut seen: Vec<Seen> = picture
            .contacts()
            .map(|c| {
                let number = picture.contact_number(c.id);
                Seen {
                    number,
                    words: crate::picture::contact_words(&picture, number),
                    kind: c.estimated_kind,
                    age: c.age_pulses,
                    hexes: hexes_from(c.estimated_hex, way),
                }
            })
            .collect();
        seen.sort_by_key(|c| c.number);
        seen
    }

    /// Rules 15 and 16: the ships sunk since the plan was last released, of
    /// the lift's force, of a force that keeps or kept station on it as its
    /// escort, or any carrier of the side, in words, from the side's events.
    fn lost_since_release(&mut self, plan: &Plan, lift: TaskForceId) -> Vec<String> {
        let charged: Vec<TaskForceId> = self
            .view
            .task_forces()
            .filter(|f| f.charge.is_some_and(|c| c.force == lift))
            .map(|f| f.id)
            .collect();
        let carrier_ships: Vec<(sim_core::ids::ShipId, String, bool)> = self
            .view
            .ships()
            .map(|s| {
                let carrier = self
                    .view
                    .ship_class(s.class)
                    .is_some_and(|c| c.kind == ShipKind::Carrier);
                (s.id, s.name.clone(), carrier)
            })
            .collect();
        let plan_id = plan.id;
        let events = self.side_events();
        let release = events
            .iter()
            .filter(|e| {
                matches!(
                    &e.kind,
                    SideEventKind::Whole(EventKind::PlanStateChanged { plan: p, to: PlanState::Released, .. })
                        if *p == plan_id
                )
            })
            .map(|e| e.number)
            .max()
            .unwrap_or(0);
        let mut escorts = charged;
        for event in events.iter().filter(|e| e.number > release) {
            if let SideEventKind::Whole(EventKind::StationKept { escort, charge, .. }) = &event.kind
            {
                if *charge == lift && !escorts.contains(escort) {
                    escorts.push(*escort);
                }
            }
        }
        let mut lost = Vec::new();
        for event in events.iter().filter(|e| e.number > release) {
            let SideEventKind::Whole(EventKind::ShipSunk {
                ship, task_force, ..
            }) = &event.kind
            else {
                continue;
            };
            let (name, carrier) = carrier_ships
                .iter()
                .find(|(id, _, _)| id == ship)
                .map_or((String::new(), false), |(_, n, c)| (n.clone(), *c));
            let whose = if *task_force == lift {
                Some(format!("of the lift {lift}"))
            } else if escorts.contains(task_force) {
                Some(format!("of {task_force}, the lift's escort"))
            } else if carrier {
                Some(format!("a carrier of the side, of {task_force}"))
            } else {
                None
            };
            if let Some(whose) = whose {
                let named = if name.is_empty() {
                    ship.to_string()
                } else {
                    format!("{ship} ({name})")
                };
                lost.push(format!("{named}, {whose}, sunk on day {}", event.day));
            }
        }
        lost
    }

    fn morning(&mut self, end: &Intent, plan: &Plan, force: &TaskForce, place: Hex) -> Morning {
        let way = self.way_to(end, force, place);
        let seen = self.seen_against(end, &way);
        let now = danger_of(&seen, FRESH_PULSES);
        let recent = danger_of(&seen, CLEAR_PULSES);
        let any = danger_of(&seen, u32::MAX);
        let risk = self.state.risk_read_up(end.id);
        // The events are read only where a danger was ever seen: a game
        // with no carrier near a lift reads nothing more.
        let lost = if any.is_some() && risk == Risk::Balanced {
            self.lost_since_release(plan, force.id)
        } else {
            Vec::new()
        };
        Morning {
            risk,
            now,
            recent,
            any,
            lost,
        }
    }

    /// Decision 0191 for the first lift of a take, before the take's own
    /// step that routes it on: a lift under way with men aboard and not
    /// landing, read against the danger and the hurt. `Decided` where the
    /// rule has handed in the morning's decision for the lift.
    #[allow(clippy::too_many_lines)]
    pub(crate) fn turn_back_step(
        &mut self,
        end: &Intent,
        plan: &Plan,
        force: &TaskForce,
        place: Hex,
        (source, source_hex): (BaseId, Hex),
    ) -> Reading {
        if force.status != TaskForceStatus::Underway
            || force.landing_at.is_some()
            || self.men_aboard_force(force) == 0
        {
            return Reading::Untouched;
        }
        let in_scope = self.in_scope(end.id);
        let today = self.today;
        let force_id = force.id;
        let on_way = crate::take::on_its_way(force, place);
        let m = self.morning(end, plan, force, place);
        let Some(any) = m.any.clone() else {
            return Reading::Untouched;
        };
        // Rule 10: a held lift is given nothing.
        if m.turns() || (!on_way && m.kept()) {
            if let Some(hold) = self.state.hold_on_force(force_id, &force.ships) {
                let words = self
                    .state
                    .hold_of(hold)
                    .map_or_else(|| format!("{hold} is held"), |h| hold_words(hold, *h));
                let reason = serving(end, plan)
                    .evidence(words.clone(), today)
                    .binding(words.clone())
                    .worked(format!(
                        "{words}: the lift {force_id} is threatened ({}), but a held lift is given nothing, so the staff gives it no order under {}.",
                        m.now.as_ref().or(m.recent.as_ref()).map_or_else(String::new, Seen::danger_words),
                        end.id
                    ))
                    .would_change("the hold released")
                    .done();
                self.target(in_scope).no_orders.push(reason);
                return Reading::Decided;
            }
        }
        if on_way {
            let Some(danger) = m.now.clone() else {
                return Reading::Untouched;
            };
            if !m.turns() {
                // Rule 8: a threatened lift not turned back says why.
                let reason = serving(end, plan)
                    .evidence(danger.danger_words(), today)
                    .worked(format!(
                        "{force_id} is routed to {place} with men aboard and is threatened: {}. {}, so it sails on and its orders stand: the staff gives no order.",
                        danger.danger_words(),
                        capitalised(&m.not_turned_words())
                    ))
                    .would_change(if m.risk == Risk::Press {
                        "a more careful risk level"
                    } else {
                        "a ship of the lift, of its escort or a carrier of the side sunk, or preserve the force"
                    })
                    .speaks_of(&self.state, LimitKind::Risk)
                    .done();
                self.target(in_scope).no_orders.push(reason);
                return Reading::Decided;
            }
            self.turn_back(end, plan, force, place, (source, source_hex), &m, &danger);
            return Reading::Decided;
        }
        // Not on its way to the place: turned back. Kept back while the
        // danger stands, and on the first clear morning after it.
        if m.kept() {
            let recent = m.recent.clone().unwrap_or(any);
            let why = if m.now.is_some() {
                format!("the danger stands: {}", recent.danger_words())
            } else {
                format!(
                    "no contact that makes the danger is fresh, but one was reported within the last {CLEAR_PULSES} pulses ({}): the lift goes on only after two clear mornings running",
                    recent.danger_words()
                )
            };
            let reason = serving(end, plan)
                .evidence(recent.danger_words(), today)
                .binding(format!(
                    "a lift turned back goes on after two mornings running with no danger {}",
                    m.risk_words()
                ))
                .worked(format!(
                    "{force_id} is turned back, under way with men aboard and not routed to {place}: {why}. {}, so the staff keeps it back and gives no order.",
                    capitalised(&m.risk_words())
                ))
                .would_change(format!(
                    "no contact reported as a carrier within {DANGER_HEXES} hexes of the way in the last {CLEAR_PULSES} pulses, or press"
                ))
                .speaks_of(&self.state, LimitKind::Risk)
                .done();
            self.target(in_scope).no_orders.push(reason);
            return Reading::Decided;
        }
        // Clear: the take's own step sends it on, and its reason says the
        // danger has passed ([`Self::before_sailing`]).
        Reading::Untouched
    }

    /// Rules 14, 19 and 20 at the take's passage step, for the first lift
    /// with men aboard that is not under way to the place: held where the
    /// danger, or a contact that made it in the last 6 pulses, would turn
    /// it back at the risk; else the words its reason gains where a carrier
    /// was ever seen near its way. `Err` holds the reason it does not sail.
    pub(crate) fn before_sailing(
        &mut self,
        end: &Intent,
        plan: &Plan,
        force: &TaskForce,
        place: Hex,
    ) -> Result<String, Box<sim_core::command::Reason>> {
        if self.men_aboard_force(force) == 0 {
            return Ok(String::new());
        }
        let m = self.morning(end, plan, force, place);
        if m.any.is_none() {
            // A lift the staff has turned back, by its notice among the
            // side's events, and that is not on its way to the place: its
            // reason says the danger has passed, though no contact of it is
            // held now (a contact is dropped before it is 6 pulses old).
            let turned = !crate::take::on_its_way(force, place)
                && self.noticed(NoticeKind::LiftTurnedBack, plan.id);
            return Ok(if turned {
                self.goes_on_words(&m)
            } else {
                String::new()
            });
        }
        if m.kept() {
            let recent = m.recent.clone().expect("kept reads a recent danger");
            let why = if m.now.is_some() {
                format!("on a morning of danger ({})", recent.danger_words())
            } else {
                format!(
                    "after a morning of danger: a contact that made it was reported within the last {CLEAR_PULSES} pulses ({}), and a lift goes on only after two clear mornings running",
                    recent.danger_words()
                )
            };
            let force_id = force.id;
            return Err(Box::new(serving(end, plan)
                .evidence(recent.danger_words(), self.today)
                .binding(format!("{} a lift does not sail into danger", m.risk_words()))
                .worked(format!(
                    "Every unit is aboard {force_id} and its supply is aboard, but it is held for the danger {why}. {}{}, so the force does not sail today.",
                    capitalised(&m.risk_words()),
                    if m.risk == Risk::Balanced {
                        format!(" and {}", m.lost_words())
                    } else {
                        String::new()
                    }
                ))
                .would_change(format!(
                    "no contact reported as a carrier within {DANGER_HEXES} hexes of the way in the last {CLEAR_PULSES} pulses, or press"
                ))
                .speaks_of(&self.state, LimitKind::Risk)
                .done()));
        }
        Ok(if m.now.is_some() {
            // At balanced not yet hurt, or at press: it sails into it.
            format!(
                " It is threatened ({}), but {}.",
                m.now.as_ref().map_or_else(String::new, Seen::danger_words),
                m.not_turned_words()
            )
        } else {
            self.goes_on_words(&m)
        })
    }

    fn goes_on_words(&self, m: &Morning) -> String {
        match &m.any {
            Some(c) => format!(
                " The danger has passed for two mornings: the newest contact reported as a carrier within {DANGER_HEXES} hexes of the way, {}, is {} old, and none has been reported in the last {CLEAR_PULSES} pulses.",
                c.words,
                pulses_words(c.age)
            ),
            None => format!(" The danger has passed for two mornings: no contact reported as a carrier is held within {DANGER_HEXES} hexes of the way, and none that made the danger has been reported in the last {CLEAR_PULSES} pulses."),
        }
    }

    /// Rules 5, 8, 9 and 13: the lift is routed to its source at cruise,
    /// its order to unload taken away, and the notice handed in.
    #[allow(clippy::too_many_arguments)]
    fn turn_back(
        &mut self,
        end: &Intent,
        plan: &Plan,
        force: &TaskForce,
        place: Hex,
        (source, source_hex): (BaseId, Hex),
        m: &Morning,
        danger: &Seen,
    ) {
        let in_scope = self.in_scope(end.id);
        let today = self.today;
        let force_id = force.id;
        let area = self.state.area_read_up(end.id);
        let from = force.position();
        let waypoints = crate::route::route_between(self.view, from, source_hex, area.as_ref())
            .map_or_else(|| vec![source_hex], |r| r.out);
        let source_text = crate::reasons::base_words(self.view, source);
        let hurt_words = if m.risk == Risk::Balanced {
            format!(", and {}", m.lost_words())
        } else {
            String::new()
        };
        let worked = format!(
            "{force_id} is under way to {place} with {} men aboard and is threatened: {}. {} a lift turns back on the danger{}{hurt_words}. The staff turns it back to its source {source_text} at cruising speed: its men stay aboard, its order to unload is taken away, and the plan stays released.",
            self.men_aboard_force(force),
            danger.danger_words(),
            capitalised(&m.risk_words()),
            if m.risk == Risk::Balanced { " once it has been hurt" } else { "" }
        );
        let reason = |extra: &str| {
            serving(end, plan)
                .evidence(danger.danger_words(), today)
                .binding(format!("{} a threatened lift turns back", m.risk_words()))
                .worked(format!("{worked} {extra}"))
                .would_change("the danger passed for two mornings, or press")
                .speaks_of(&self.state, LimitKind::Risk)
                .done()
        };
        if in_scope {
            self.lines
                .push(format!("{force_id} is turned back to {source}."));
        }
        let route_reason = reason("This is the route order: back to the source.");
        let speed_reason = reason("This is the speed order: cruising speed.");
        let cargo_reason = reason(
            "This is the order to do no cargo work: the men do not go ashore at the source.",
        );
        let notice_reason =
            reason("The staff hands in the notice that it has turned the lift back.");
        let out = self.target(in_scope);
        out.orders.push(StaffOrder {
            kind: OrderKind::TaskForce {
                target: force_id,
                order: TaskForceOrder::route(waypoints),
            },
            intent: end.id,
            reason: route_reason,
        });
        if force.speed != SpeedSetting::Cruise {
            out.orders.push(StaffOrder {
                kind: OrderKind::TaskForce {
                    target: force_id,
                    order: TaskForceOrder::SetSpeed(SpeedSetting::Cruise),
                },
                intent: end.id,
                reason: speed_reason,
            });
        }
        out.orders.push(StaffOrder {
            kind: OrderKind::TaskForce {
                target: force_id,
                order: TaskForceOrder::NoCargoWork,
            },
            intent: end.id,
            reason: cargo_reason,
        });
        out.no_orders.push(notice_reason);
        let index = out.no_orders.len() - 1;
        out.notices.push(StaffNotice {
            kind: NoticeKind::LiftTurnedBack,
            plan: plan.id,
            run: None,
            rests_on: NoticeBasis::NoOrder(index),
            pursuit: None,
            follow: None,
        });
    }
}

fn capitalised(words: &str) -> String {
    let mut chars = words.chars();
    match chars.next() {
        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
        None => String::new(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn seen(number: u32, kind: ShipKind, age: u32, hexes: u32) -> Seen {
        Seen {
            number,
            words: format!("contact {number}"),
            kind,
            age,
            hexes,
        }
    }

    #[test]
    fn the_table_by_risk() {
        assert!(turns(Risk::Preserve, false));
        assert!(turns(Risk::Preserve, true));
        assert!(!turns(Risk::Balanced, false));
        assert!(turns(Risk::Balanced, true));
        assert!(!turns(Risk::Press, false));
        assert!(!turns(Risk::Press, true));
    }

    #[test]
    fn turned_back_at_a_bolder_risk_is_turned_back_at_a_more_careful_one() {
        for hurt in [false, true] {
            if turns(Risk::Press, hurt) {
                assert!(turns(Risk::Balanced, hurt));
            }
            if turns(Risk::Balanced, hurt) {
                assert!(turns(Risk::Preserve, hurt));
            }
        }
    }

    #[test]
    fn what_counts_as_the_danger() {
        // Fresh, a carrier, within 8 hexes.
        assert!(danger_of(&[seen(1, ShipKind::Carrier, 3, 8)], FRESH_PULSES).is_some());
        // 4 pulses old.
        assert!(danger_of(&[seen(1, ShipKind::Carrier, 4, 6)], FRESH_PULSES).is_none());
        // 9 hexes.
        assert!(danger_of(&[seen(1, ShipKind::Carrier, 1, 9)], FRESH_PULSES).is_none());
        // Reported as a battleship.
        assert!(danger_of(&[seen(1, ShipKind::Battleship, 1, 6)], FRESH_PULSES).is_none());
        // Within the last 6 pulses for the second clear morning.
        assert!(danger_of(&[seen(1, ShipKind::Carrier, 6, 6)], CLEAR_PULSES).is_some());
        assert!(danger_of(&[seen(1, ShipKind::Carrier, 7, 6)], CLEAR_PULSES).is_none());
    }

    #[test]
    fn the_nearest_then_newest_makes_the_danger() {
        let found = danger_of(
            &[
                seen(1, ShipKind::Carrier, 2, 5),
                seen(2, ShipKind::Carrier, 0, 5),
                seen(3, ShipKind::Carrier, 0, 7),
            ],
            FRESH_PULSES,
        );
        assert_eq!(found.map(|c| c.number), Some(2));
    }

    #[test]
    fn an_older_or_farther_contact_never_makes_a_danger_the_newer_or_nearer_did_not() {
        for age in 0..10 {
            for hexes in 0..12 {
                let base = danger_of(&[seen(1, ShipKind::Carrier, age, hexes)], FRESH_PULSES);
                let older = danger_of(&[seen(1, ShipKind::Carrier, age + 1, hexes)], FRESH_PULSES);
                let farther =
                    danger_of(&[seen(1, ShipKind::Carrier, age, hexes + 1)], FRESH_PULSES);
                if base.is_none() {
                    assert!(older.is_none() && farther.is_none());
                }
            }
        }
    }

    #[test]
    fn the_way_is_read_hex_by_hex() {
        let way = [Hex::new(0, 0), Hex::new(1, 0), Hex::new(2, 0)];
        assert_eq!(hexes_from(Hex::new(1, 0), &way), 0);
        assert_eq!(hexes_from(Hex::new(5, 0), &way), 3);
    }
}
