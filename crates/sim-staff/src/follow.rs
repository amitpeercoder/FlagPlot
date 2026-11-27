//! Following a force (decision 0220, "an order against an enemy force"): the
//! staff's part of the order to destroy a force.
//!
//! The order names a contact of the commander's picture and keeps, in the
//! side's command state, the hex and the day and pulse of the last report of
//! the force it follows ([`sim_core::command::Followed`]). At dusk, before
//! every other pass, `follow_forces` works for each such order in force, by
//! priority and then by number (rule 5 as ruled in R5):
//!
//! 1. the contact followed is held and fresh: the order follows it, and its
//!    centre is the contact's hex (a notice, "follows", where the report is
//!    not the one kept);
//! 2. else a contact of the picture is taken by the rule of the sort, the
//!    nearest and the lowest number (rule 7, R4), a notice "follows" naming
//!    the number it changed from;
//! 3. else, on the dusk that begins day `D + lost_contact_days + 1`, `D`
//!    being the day of the last report kept, the order ends (a notice that
//!    the force was not found; the engine withdraws the order and all
//!    beneath it);
//! 4. else the order is lost-contact: its centre stays the last report's
//!    hex, the staff hands in a notice once, and withdraws the strike tasks
//!    and carrier tasks it formed under the order (R6). Its search tasks
//!    stand.
//!
//! Each notice rests on a reason. The centre of the order for the day is
//! kept in the day's working ([`FollowedToday`]) and read by every pass that
//! serves a deny through [`Day::waters_of`]. This module draws no random
//! number and adds no state of its own: what the order keeps is written by
//! the engine when it records the notice.

use std::collections::BTreeMap;

use sim_core::command::{
    report_distance_hexes, report_pulse, CommandKind, FollowNotice, Intent, IntentKind, Maker,
    NoticeBasis, NoticeKind, Plan, StaffCommand, StaffNotice,
};
use sim_core::hex::Hex;
use sim_core::ids::IntentId;
use sim_core::refdata::ShipKind;
use sim_core::time::Pulse;

use crate::air::Day;
use crate::reasons::serving;

/// The knots a followed force is taken to steam at, to say how far it may
/// have gone between two reports (decision 0220, rule 7 and R4). A placeholder
/// and a judgement of the doctrine, in no file.
pub(crate) const FOLLOWED_FORCE_KNOTS: u32 = 20;

/// The days after the last report at which an order that has found nothing
/// ends (decision 0220, rule 10 and R5).
pub(crate) const LOST_CONTACT_DAYS: u32 = 2;

/// The hexes beyond the farthest a station may be from the centre of the
/// waters at which a free carrier force's station is chosen again (decision
/// 0220, R10).
pub(crate) const STATION_RECHOSEN_HEXES: u32 = 3;

/// What the day's working keeps of one order to destroy a force.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) struct FollowedToday {
    /// The centre of the order's waters today.
    pub(crate) centre: Hex,
    /// The hex the order kept before this dusk (decision 0220, R7).
    pub(crate) before: Hex,
    /// Whether the order is lost-contact today.
    pub(crate) lost: bool,
}

/// The words of a pulse of a day: "the morning of day 2".
pub(crate) fn pulse_words(day: u32, pulse: Pulse) -> String {
    let name = match pulse {
        Pulse::Night => "night",
        Pulse::Morning => "morning",
        Pulse::Afternoon => "afternoon",
    };
    format!("the {name} of day {day}")
}

/// A contact the pass reads, as the area command holds it.
#[derive(Clone, Copy, Debug)]
struct Held {
    number: u32,
    hex: Hex,
    count: u32,
    kind: ShipKind,
    age: u32,
    day: u32,
    pulse: Pulse,
}

impl Held {
    fn carrier(&self) -> bool {
        self.kind == ShipKind::Carrier
    }

    fn words(&self) -> String {
        format!(
            "contact {}, {} ship(s), {}, at {}, {} pulse(s) old, last reported in {}",
            self.number,
            self.count,
            self.kind.name(),
            self.hex,
            self.age,
            pulse_words(self.day, self.pulse)
        )
    }
}

impl Day<'_, '_> {
    /// The waters of an intent that serves a deny, as a centre and a distance
    /// in hexes (decision 0220, 2): a deny's own; an order to destroy a force's
    /// are read from the force it follows, as `follow_forces` worked them this
    /// dusk.
    pub(crate) fn waters_of(&self, intent: &Intent) -> Option<(Hex, u32)> {
        match intent.kind {
            IntentKind::DestroyForce { distance, .. } => self
                .followed
                .get(&intent.id)
                .map(|today| today.centre)
                .or_else(|| self.state.followed.get(&intent.id).map(|kept| kept.hex))
                .map(|centre| (centre, distance)),
            other => other.waters(),
        }
    }

    /// Whether `intent` is an order to destroy a force that is lost-contact
    /// today.
    pub(crate) fn is_lost(&self, intent: IntentId) -> bool {
        self.followed.get(&intent).is_some_and(|today| today.lost)
    }

    /// The order's words: what a deny says of its waters, or what the order
    /// to destroy a force is and the waters round the force it follows.
    pub(crate) fn end_words(&self, intent: &Intent) -> String {
        match intent.kind {
            IntentKind::DestroyForce {
                contact,
                distance,
                search_only,
            } => {
                let verb = if search_only { "shadow" } else { "destroy" };
                let centre = self
                    .waters_of(intent)
                    .map_or_else(String::new, |(hex, _)| format!(", round {hex}"));
                format!(
                    "{verb} the force of contact {contact}, the waters within {distance} hexes of it{centre}"
                )
            }
            other => match other.waters() {
                Some((hex, distance)) => crate::reasons::waters_words(hex, distance),
                None => String::new(),
            },
        }
    }

    fn held_contacts(&self) -> Vec<Held> {
        let today = self.today;
        // The commander's picture (decision 0220, 12): the area command's,
        // which with command levels off holds the side's own belief.
        let area = self.view.command(sim_core::pictures::CommandRef::Area);
        let contacts: Vec<&sim_core::belief::Contact> = match &area {
            Some(command) => command.contacts().collect(),
            None => self.area.contacts().collect(),
        };
        let mut found: Vec<Held> = contacts
            .into_iter()
            .map(|c| {
                let (day, pulse) = report_pulse(today, Pulse::Night, c.age_pulses);
                Held {
                    number: self.area.contact_number(c.id),
                    hex: c.estimated_hex,
                    count: c.estimated_ship_count,
                    kind: c.estimated_kind,
                    age: c.age_pulses,
                    day,
                    pulse,
                }
            })
            .collect();
        found.sort_by_key(|c| c.number);
        found
    }

    /// The pass of decision 0220, at dusk and before every other pass.
    pub(crate) fn follow_forces(&mut self) {
        let orders = self.in_priority_order(|i| matches!(i.kind, IntentKind::DestroyForce { .. }));
        if orders.is_empty() {
            return;
        }
        let held = self.held_contacts();
        // The contact each order follows, as it stands as the pass works
        // (a taking this dusk counts for the orders worked after it).
        let mut current: BTreeMap<IntentId, u32> = orders
            .iter()
            .filter_map(|id| self.state.followed.get(id).map(|kept| (*id, kept.contact)))
            .collect();
        for id in orders {
            self.follow_one(id, &held, &mut current);
        }
    }

    #[allow(clippy::too_many_lines)]
    fn follow_one(&mut self, id: IntentId, held: &[Held], current: &mut BTreeMap<IntentId, u32>) {
        let Some(order) = self.state.intents.get(&id).cloned() else {
            return;
        };
        let Some(kept) = self.state.followed.get(&id).copied() else {
            return;
        };
        let Some(plan) = self.plan_of(&order) else {
            return;
        };
        let today = self.today;
        let in_scope = self.in_scope(id);
        let fresh_age = self.area.search_figures().fresh_contact_max_age_pulses;
        let own = |c: &Held| c.number == kept.contact;
        let followed = held.iter().copied().find(own);
        let keeps = |day: u32, pulse: Pulse, hex: Hex| {
            kept.day == day && kept.pulse == pulse && kept.hex == hex
        };
        let kept_words = format!(
            "the order keeps contact {}, at {}, last reported in {}, {}",
            kept.contact,
            kept.hex,
            pulse_words(kept.day, kept.pulse),
            if kept.carrier {
                "taken for a carrier"
            } else {
                "not taken for a carrier"
            }
        );
        let mut today_state = FollowedToday {
            centre: kept.hex,
            before: kept.hex,
            lost: kept.lost,
        };
        if plan.state.ended() {
            self.followed.insert(id, today_state);
            return;
        }
        // 1. A fresh contact is followed.
        if let Some(c) = followed.filter(|c| c.age <= fresh_age) {
            today_state.centre = c.hex;
            today_state.lost = false;
            self.followed.insert(id, today_state);
            if !keeps(c.day, c.pulse, c.hex) {
                let reason = serving(&order, &plan)
                    .evidence(format!("{}; it is fresh (no more than {fresh_age} pulses old)", c.words()), today)
                    .evidence(kept_words.clone(), today)
                    .binding(format!("a contact no more than {fresh_age} pulses old is followed"))
                    .worked(format!(
                        "{}, the order to destroy the force of contact {}, follows {}: it is fresh, so the order follows it and keeps this report, its centre {}.",
                        id, kept.contact, c.words(), c.hex
                    ))
                    .would_change("the contact older than the figure, or dropped")
                    .done();
                self.hand_in_follow(
                    &order,
                    &plan,
                    reason,
                    NoticeKind::Follows,
                    FollowNotice {
                        intent: id,
                        contact: c.number,
                        taken_from: None,
                        other_sort: false,
                        hex: c.hex,
                        day: c.day,
                        pulse: c.pulse,
                        ship_count: c.count,
                        kind: c.kind,
                        carrier: c.carrier(),
                    },
                    in_scope,
                );
            }
            return;
        }
        // The report the order reads rule 7 from: the contact's own where it
        // is held, else what the order kept (R3).
        let (ref_day, ref_pulse, ref_hex) = match followed {
            Some(c) => (c.day, c.pulse, c.hex),
            None => (kept.day, kept.pulse, kept.hex),
        };
        // 2. Take another.
        let mut qualifying: Vec<(&Held, u32)> = Vec::new();
        let mut why_not: Vec<String> = Vec::new();
        for c in held.iter().filter(|c| c.number != kept.contact) {
            if (c.day, c.pulse) <= (ref_day, ref_pulse) {
                why_not.push(format!(
                    "contact {} was last reported no later than the contact followed",
                    c.number
                ));
                continue;
            }
            let room =
                report_distance_hexes(ref_day, ref_pulse, c.day, c.pulse, FOLLOWED_FORCE_KNOTS);
            let apart = c.hex.distance(ref_hex);
            if apart > room {
                why_not.push(format!(
                    "contact {} is {apart} hexes from {ref_hex}, beyond the {room} a force at {FOLLOWED_FORCE_KNOTS} knots makes between the reports",
                    c.number
                ));
                continue;
            }
            if let Some((other, _)) = current
                .iter()
                .find(|(other, number)| **other != id && **number == c.number)
            {
                why_not.push(format!("contact {} is followed by {other}", c.number));
                continue;
            }
            qualifying.push((c, apart));
        }
        qualifying.sort_by_key(|(c, apart)| (c.carrier() != kept.carrier, *apart, c.number));
        if let Some((taken, apart)) = qualifying.first().copied() {
            let taken = *taken;
            let other_sort = taken.carrier() != kept.carrier;
            today_state.centre = taken.hex;
            today_state.lost = false;
            self.followed.insert(id, today_state);
            current.insert(id, taken.number);
            let room = report_distance_hexes(
                ref_day,
                ref_pulse,
                taken.day,
                taken.pulse,
                FOLLOWED_FORCE_KNOTS,
            );
            let sort_words = if other_sort {
                format!(
                    "{} for a force the order took for {}: a taking of the other sort",
                    if taken.carrier() {
                        "a carrier"
                    } else {
                        "not a carrier"
                    },
                    if kept.carrier {
                        "a carrier"
                    } else {
                        "not a carrier"
                    }
                )
            } else {
                "of the same sort as the force followed".to_string()
            };
            let others: Vec<String> = qualifying
                .iter()
                .skip(1)
                .map(|(c, a)| format!("contact {} ({a} hexes)", c.number))
                .collect();
            let others = if others.is_empty() {
                "no other qualified".to_string()
            } else {
                format!("it was chosen before {}", others.join(", "))
            };
            let from_words = match followed {
                Some(c) => format!(
                    "contact {} is held, {} pulse(s) old, not fresh",
                    kept.contact, c.age
                ),
                None => format!("contact {} is no longer held", kept.contact),
            };
            let reason = serving(&order, &plan)
                .evidence(from_words.clone(), today)
                .evidence(kept_words.clone(), today)
                .evidence(taken.words(), today)
                .binding("a contact reported later than the force followed, within the hexes a force at 20 knots steams between the two reports, that no other order follows, is taken: the same sort before the other, then the nearest, then the lowest number")
                .worked(format!(
                    "{id}, the order to destroy the force of contact {}: {from_words}. {} was last reported later than the report read from {ref_hex}, {apart} hexes away against the {room} a force at {FOLLOWED_FORCE_KNOTS} knots makes in the hours from the start of {} to the end of {}, no other order follows it, and it is {sort_words}; {others}. The order takes it: it follows contact {}, its centre {}.",
                    kept.contact,
                    taken.words(),
                    pulse_words(ref_day, ref_pulse),
                    pulse_words(taken.day, taken.pulse),
                    taken.number,
                    taken.hex
                ))
                .would_change("another contact reported later and nearer")
                .would_change("the contact followed refreshed")
                .done();
            self.hand_in_follow(
                &order,
                &plan,
                reason,
                NoticeKind::Follows,
                FollowNotice {
                    intent: id,
                    contact: taken.number,
                    taken_from: Some(kept.contact),
                    other_sort,
                    hex: taken.hex,
                    day: taken.day,
                    pulse: taken.pulse,
                    ship_count: taken.count,
                    kind: taken.kind,
                    carrier: taken.carrier(),
                },
                in_scope,
            );
            return;
        }
        // What the notices of an order lost or ended say of the report kept.
        let (count, kind) = match followed {
            Some(c) => (c.count, c.kind),
            None => (kept.ship_count, kept.kind),
        };
        let notice = |contact: u32| FollowNotice {
            intent: id,
            contact,
            taken_from: None,
            other_sort: false,
            hex: ref_hex,
            day: ref_day,
            pulse: ref_pulse,
            ship_count: count,
            kind,
            carrier: kept.carrier,
        };
        let nothing = if why_not.is_empty() {
            "the picture holds no other contact".to_string()
        } else {
            why_not.join("; ")
        };
        // 3. It ends.
        if today > ref_day + LOST_CONTACT_DAYS {
            let reason = serving(&order, &plan)
                .evidence(kept_words.clone(), today)
                .evidence(nothing.clone(), today)
                .binding(format!(
                    "an order ends on the dusk that begins day {} + {LOST_CONTACT_DAYS} + 1 after its last report, with nothing taken",
                    ref_day
                ))
                .worked(format!(
                    "{id}, the order to destroy the force of contact {}: the last report kept is of day {ref_day}, at {ref_hex}, and {ref_day} + {LOST_CONTACT_DAYS} + 1 is {}; no contact qualifies ({nothing}). The force was not found: the order ends, and everything beneath it is withdrawn with the notice's event as the cause.",
                    kept.contact,
                    ref_day + LOST_CONTACT_DAYS + 1
                ))
                .would_change("a contact reported later within the distance of the last report")
                .done();
            // Nothing beneath it is served today.
            for below in self.state.descendants_in_force(id) {
                if let Some(found) = self.state.intents.get_mut(&below) {
                    found.withdrawn_day = Some(today);
                }
            }
            if let Some(found) = self.state.intents.get_mut(&id) {
                found.withdrawn_day = Some(today);
            }
            today_state.centre = ref_hex;
            self.followed.insert(id, today_state);
            self.hand_in_follow(
                &order,
                &plan,
                reason,
                NoticeKind::ForceNotFound,
                notice(kept.contact),
                in_scope,
            );
            return;
        }
        // 4. It is lost.
        today_state.centre = ref_hex;
        today_state.lost = true;
        self.followed.insert(id, today_state);
        if kept.lost {
            return;
        }
        let reason = serving(&order, &plan)
            .evidence(kept_words, today)
            .evidence(nothing.clone(), today)
            .binding("an order whose contact is not fresh and for which none is taken is lost-contact: its centre stays the last report's hex and only its search tasks serve it")
            .worked(format!(
                "{id}, the order to destroy the force of contact {}: the contact is not fresh and none qualifies ({nothing}). The order is lost-contact: its centre stays {ref_hex}, where it was last reported in {}, and the staff keeps its search tasks and withdraws its strike tasks and carrier tasks.",
                kept.contact,
                pulse_words(ref_day, ref_pulse)
            ))
            .would_change("a contact reported later within the distance of the last report")
            .done();
        self.hand_in_follow(
            &order,
            &plan,
            reason,
            NoticeKind::ContactLost,
            notice(kept.contact),
            in_scope,
        );
        self.withdraw_tasks_of_lost(&order, &plan, in_scope);
    }

    /// Hands in one notice of following, resting on the reason given.
    fn hand_in_follow(
        &mut self,
        order: &Intent,
        plan: &Plan,
        reason: sim_core::command::Reason,
        kind: NoticeKind,
        follow: FollowNotice,
        in_scope: bool,
    ) {
        if in_scope {
            self.lines.push(format!("{}: {}", order.id, reason.worked));
        }
        let out = self.target(in_scope);
        out.no_orders.push(reason);
        let index = out.no_orders.len() - 1;
        out.notices.push(StaffNotice {
            kind,
            plan: plan.id,
            run: None,
            rests_on: NoticeBasis::NoOrder(index),
            pursuit: None,
            follow: Some(follow),
        });
    }

    /// Rule 9 and R6: the strike tasks and carrier tasks the staff formed
    /// under an order that is lost are withdrawn; its search tasks stand.
    fn withdraw_tasks_of_lost(&mut self, order: &Intent, plan: &Plan, in_scope: bool) {
        let tasks: Vec<Intent> = self
            .state
            .children_in_force(order.id)
            .into_iter()
            .filter_map(|c| self.state.intents.get(&c).cloned())
            .filter(|c| c.maker == Maker::Staff)
            .filter(|c| {
                matches!(
                    c.kind,
                    IntentKind::StrikeShips { .. } | IntentKind::OperateCarriers { .. }
                )
            })
            .collect();
        for task in tasks {
            self.withdraw_task(
                order,
                plan,
                &task,
                in_scope,
                &format!(
                    "{} is lost-contact: its strike tasks and carrier tasks are withdrawn and its search tasks stand",
                    order.id
                ),
                "a contact taken again",
            );
        }
    }

    /// The staff withdraws a task it formed under `order`, with the reason
    /// `why`, and the day's working serves nothing beneath it.
    pub(crate) fn withdraw_task(
        &mut self,
        order: &Intent,
        plan: &Plan,
        task: &Intent,
        in_scope: bool,
        why: &str,
        would_change: &str,
    ) {
        let today = self.today;
        let reason = serving(order, plan)
            .evidence(why.to_string(), today)
            .binding(format!("{} is the staff's own task", task.id))
            .worked(format!(
                "{}: {why}, so the staff withdraws {}, {}.",
                order.id,
                task.id,
                crate::reasons::aim(self.view, task)
            ))
            .would_change(would_change)
            .done();
        self.target(in_scope).commands.push(StaffCommand {
            kind: CommandKind::WithdrawIntent { intent: task.id },
            reason,
        });
        if in_scope {
            self.lines
                .push(format!("{} would withdraw {}: {why}.", order.id, task.id));
        }
        for below in self.state.descendants_in_force(task.id) {
            if let Some(found) = self.state.intents.get_mut(&below) {
                found.withdrawn_day = Some(today);
            }
        }
        if let Some(found) = self.state.intents.get_mut(&task.id) {
            found.withdrawn_day = Some(today);
        }
    }
}
