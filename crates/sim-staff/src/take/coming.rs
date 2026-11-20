//! Men ashore with no more coming (decision 0193, as amended on review):
//! on the morning of the judgement ashore, whether more is coming to the
//! place, read as "not to starve" reads what is in reach, and the men the
//! staff wants when nothing more is.
//!
//! More is coming where a lift of the take has fighting men aboard (under
//! way, turned back or waiting); or a lift as it stands with fighting men
//! aboard may be sent as a later lift by the end's last day; or a fighting
//! unit of the take's forces limit that is not ashore at the place can be
//! used and carried by then. A unit or force before its day counts from its
//! day. A unit that no ship of the take can carry does not count: the
//! lift's force has no ship afloat, is bound for another base than the
//! source (sent on to a hold's source), its ships cannot berth the unit, or
//! "not to starve" forbids the lift. The test keeps no state and draws
//! nothing.
//!
//! "More is coming" ends with the game (decision 0194, 10): the latest day
//! is the end's date or the game's last day, whichever is the earlier. And
//! in the game's last days (decision 0194, 7 to 9) the staff begins no
//! attack ashore: [`LAST_DAYS`], [`days_left`].

use std::collections::BTreeSet;

use sim_core::command::{Intent, IntentKind, Risk};
use sim_core::hex::Hex;
use sim_core::ids::{BaseId, EntityRef, LandUnitId};
use sim_core::state::{ShipLocation, TaskForce};

use crate::air::Day;

use super::unit_words;

/// The men the staff wants for each man of the estimate where nothing more
/// is coming, by risk (decision 0193, rule 2): 3 at preserve the force (it
/// waits, as while more is coming), 1 at balanced, none at press. The
/// staff's figures, placeholders.
pub(crate) fn men_per_man_with_nothing_coming(risk: Risk) -> u32 {
    match risk {
        Risk::Preserve => 3,
        Risk::Balanced => 1,
        Risk::Press => 0,
    }
}

/// The days left below which the staff begins no attack ashore (decision
/// 0194, 7): with fewer than 3 days of the game left, an attack begun so
/// late only costs men. The staff's figure, a placeholder.
pub(crate) const LAST_DAYS: u32 = 3;

/// The days left on the morning of `today` in a game of `game_days` days
/// (decision 0194, 2 and 11): the game's days less the days resolved, which
/// are `today - 1`, never below 0, as in a trial played past the game's
/// end. `None` for a world given no length, which has no last day.
pub(crate) fn days_left(game_days: Option<u32>, today: u32) -> Option<u32> {
    game_days.map(|days| days.saturating_sub(today.saturating_sub(1)))
}

/// Whether a morning with `left` days left is in the game's last days
/// (decision 0194, 7): fewer than [`LAST_DAYS`] left. A world with no last
/// day never is.
pub(crate) fn in_last_days(left: Option<u32>) -> bool {
    left.is_some_and(|left| left < LAST_DAYS)
}

/// The words of the days left, for the reason of a morning in the last
/// days (decision 0194, 7).
pub(crate) fn last_days_words(left: u32) -> String {
    format!(
        "{left} {} of the game {} left, fewer than {LAST_DAYS}",
        if left == 1 { "day" } else { "days" },
        if left == 1 { "is" } else { "are" }
    )
}

/// The judgement of a morning ashore (decision 0143, items 5 and 6, as
/// decision 0193 amends it).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) enum Verdict {
    /// Each unit that can attack is set to attack.
    Attack,
    /// The staff waits: a unit on attack is set to defend.
    Wait,
    /// Nothing more is coming, the men are too few, and a unit the staff
    /// sent in under this rule is on attack: it is not set back to defend
    /// (rule 12), and no other unit is set to attack.
    Committed,
}

/// The men wanted against an estimate of `estimate` men at `risk`, while
/// more is coming (`more`) or with nothing more coming (rule 2).
pub(crate) fn men_wanted_by(estimate: u32, risk: Risk, more: bool) -> u32 {
    estimate
        * if more {
            super::men_per_man(risk)
        } else {
            men_per_man_with_nothing_coming(risk)
        }
}

/// The judgement from the fit men that can attack, the estimate (`None`
/// where none is held), the risk, whether more is coming, and whether a
/// unit of the operation that the staff sent in under this rule is on
/// attack (`on_attack`). With no estimate, or the men enough
/// for what is wanted while more is coming, it attacks as before; while
/// more is coming and the men are too few it waits as before. With nothing
/// more coming the men wanted fall (rule 2); where they are still too few,
/// at balanced and at press a unit on attack stays in (rule 12), and at
/// preserve the force it waits as before.
pub(crate) fn verdict(
    fit: u32,
    estimate: Option<u32>,
    risk: Risk,
    more: bool,
    on_attack: bool,
) -> Verdict {
    let Some(estimate) = estimate else {
        return Verdict::Attack;
    };
    if fit >= men_wanted_by(estimate, risk, true) {
        return Verdict::Attack;
    }
    if more {
        return Verdict::Wait;
    }
    if fit >= men_wanted_by(estimate, risk, false) {
        Verdict::Attack
    } else if risk != Risk::Preserve && on_attack {
        Verdict::Committed
    } else {
        Verdict::Wait
    }
}

/// What the test of rule 10 found: whether more is coming, and in words
/// what was looked for and what was found.
#[derive(Clone, Debug)]
pub(crate) struct Coming {
    pub(crate) more: bool,
    pub(crate) words: String,
}

/// The hex the force is bound for: the last of its route where a route is
/// left, else where it is.
fn bound_for(force: &TaskForce) -> Hex {
    if force.route.next_index < force.route.waypoints.len() {
        force
            .route
            .waypoints
            .last()
            .copied()
            .unwrap_or_else(|| force.position())
    } else {
        force.position()
    }
}

/// The binding of the reason of a unit sent in to attack with nothing more
/// coming (rule 12): how the staff knows, from its own reasons in the
/// side's log, which units it sent in under this rule. It keeps no state.
pub(crate) const SENT_IN: &str = "sent in with nothing more coming";

impl Day<'_, '_> {
    /// Decision 0194, 9: whether the side's attack at `place` failed on
    /// `day`, as the side is told it (chapter 7, An attack that fails: the
    /// side that attacked and failed sees `GroundAttackFailed`, cut to the
    /// hex).
    pub(crate) fn attack_failed(&mut self, place: Hex, day: u32) -> bool {
        use sim_core::side_events::SideEventKind;
        self.side_events().iter().any(|e| {
            e.day == day
                && matches!(e.kind, SideEventKind::GroundAttackFailed { hex } if hex == place)
        })
    }

    /// Rule 12: whether the staff's last order to `unit` that the side's log
    /// holds was an order sent in under this rule, the reason of which is
    /// bound by [`SENT_IN`]. Read from the side's own events: its orders
    /// accepted and the staff's reasons for them.
    pub(crate) fn sent_in(&mut self, unit: LandUnitId) -> bool {
        use sim_core::events::EventKind;
        use sim_core::side_events::SideEventKind;
        let target = EntityRef::LandUnit(unit);
        let events = self.side_events();
        let orders: BTreeSet<sim_core::ids::OrderId> = events
            .iter()
            .filter_map(|e| match &e.kind {
                SideEventKind::Whole(EventKind::OrderAccepted { order, target: t })
                    if *t == target =>
                {
                    Some(*order)
                }
                _ => None,
            })
            .collect();
        events
            .iter()
            .filter_map(|e| match &e.kind {
                SideEventKind::Whole(EventKind::StaffReason {
                    order: Some(order),
                    reason,
                    ..
                }) if orders.contains(order) => Some(reason.binding.iter().any(|b| b == SENT_IN)),
                _ => None,
            })
            .next_back()
            .unwrap_or(false)
    }

    /// Whether any ship of `force` is afloat in it.
    fn afloat(&self, force: &TaskForce) -> bool {
        !force.ships.is_empty()
            && self
                .view
                .ships()
                .any(|s| s.location == ShipLocation::TaskForce(force.id))
    }

    /// The fighting men aboard the force named, by unit.
    fn fighting_aboard(&self, force: sim_core::ids::TaskForceId) -> u32 {
        self.aboard(force)
            .into_iter()
            .filter(|(u, _)| !u.working)
            .map(|(_, men)| men)
            .sum()
    }

    /// Decision 0193, rule 10: whether more is coming to `place` for the
    /// take `end`, whose landing is `task` and first lift `force`, with
    /// `source` its source as the limit binds, `fit` the fit men ashore
    /// that can attack and `wanted` the men wanted while more is coming
    /// (for the second lift's choice and "not to starve", as today).
    #[allow(clippy::too_many_arguments, clippy::too_many_lines)]
    pub(crate) fn more_coming(
        &self,
        end: &Intent,
        task: &Intent,
        force: &TaskForce,
        place: Hex,
        (source, source_hex): (BaseId, Hex),
        fit: u32,
        wanted: u32,
    ) -> Coming {
        let view = self.view;
        let end_by = match end.kind {
            IntentKind::TakePlace { by_day, .. } => by_day,
            _ => None,
        };
        // Decision 0194, 10: "more is coming" ends with the game. The latest
        // day is the end's date or the game's last day, whichever is the
        // earlier; a world with no last day has the end's date alone.
        let game_last = view.game_days();
        let by_day = match (end_by, game_last) {
            (Some(by), Some(last)) => Some(by.min(last)),
            (by, None) => by,
            (None, last) => last,
        };
        // A unit or force before its day counts from its day: with a date,
        // where its day is no later than the date; with none, always (the
        // seven days the staff looks ahead are counted from its day).
        let in_time = |from: Option<u32>| match (from, by_day) {
            (Some(from), Some(last)) => from <= last,
            _ => true,
        };
        // The words name the end's date as before; where the game's last day
        // is what leaves a unit out, the unit's own words say so.
        let last_words = match end_by {
            Some(last) => format!("by day {last}, the latest day"),
            None => {
                "with no date, a unit or force before its day counting from its day".to_string()
            }
        };
        // Why a day is out of time, in the words the unit's own list uses.
        let after_words = |day: u32| match game_last {
            Some(last) if end_by.is_none_or(|by| day <= by) => {
                format!("after the game's last day, day {last}")
            }
            _ => "after the latest day".to_string(),
        };
        let mut found: Vec<String> = Vec::new();
        let mut not: Vec<String> = Vec::new();

        // 1. The take's own lifts with fighting men aboard: its first lift
        // and the forces its landing's limit names.
        let current = self
            .state
            .intents
            .get(&task.id)
            .cloned()
            .unwrap_or_else(|| task.clone());
        let mut own: BTreeSet<sim_core::ids::TaskForceId> = BTreeSet::new();
        own.insert(force.id);
        if let Some(limit) = &current.limits.forces {
            own.extend(limit.task_forces.iter().copied());
        }
        for id in &own {
            let Some(lift) = view.task_forces().find(|f| f.id == *id) else {
                continue;
            };
            if !self.afloat(lift) {
                continue;
            }
            let men = self.fighting_aboard(*id);
            if men > 0 {
                found.push(format!(
                    "{id}, a lift of the take, has {men} fighting men aboard"
                ));
            }
        }

        // 1b. A force held apart until its day (decision 0195, 13) with
        // fighting men aboard is a lift from its day, where the day is in
        // time: those the limit names, or with none named every transport
        // force. The men aboard are read from the entry that holds it.
        for entry in view.entering() {
            let Some(held_force) = entry.task_force.as_ref() else {
                continue;
            };
            let id = held_force.id;
            let counted = crate::available::counted_by_a_take(view, current.limits.forces.as_ref())
                .task_forces
                .iter()
                .any(|f| f.id == id);
            if !(own.contains(&id) || counted) {
                continue;
            }
            let men: u32 = entry
                .ships
                .values()
                .filter_map(|ship| ship.troops)
                .filter(|t| {
                    entry
                        .land_units
                        .get(&t.unit)
                        .is_some_and(|unit| !unit.working)
                })
                .map(|t| t.men)
                .sum();
            if men == 0 {
                continue;
            }
            let day =
                crate::available::entering_on(view, EntityRef::TaskForce(id)).unwrap_or(entry.day);
            if in_time(Some(day)) {
                found.push(format!(
                    "{id}, a lift of the take, enters on day {day} with {men} fighting men aboard"
                ));
            } else {
                not.push(format!(
                    "{} with {men} fighting men aboard, {}",
                    crate::available::enters_words(id, day),
                    after_words(day)
                ));
            }
        }

        // 2. The lifts as they stand the end may send as later lifts by its
        // last day, as "not to starve" counts them: by the end's date or the
        // game's last day, whichever is the earlier (decision 0194, 10).
        self.as_of.set(Some(by_day.unwrap_or(u32::MAX)));
        let stand = self.later_lift_plan(end, force.id, None);
        self.as_of.set(None);
        for send in stand.map(|s| s.send).unwrap_or_default() {
            if own.contains(&send.force.id) {
                continue;
            }
            let men: u32 = self
                .aboard(send.force.id)
                .into_iter()
                .filter(|(u, _)| send.fighting.contains(&u.id))
                .map(|(_, men)| men)
                .sum();
            if men == 0 {
                continue;
            }
            let from = view
                .available_from(EntityRef::TaskForce(send.force.id))
                .filter(|from| *from > self.today);
            found.push(match from {
                Some(day) => format!(
                    "{} may be sent as a later lift with {men} fighting men aboard from day {day}",
                    send.force.id
                ),
                None => format!(
                    "{} may be sent as a later lift with {men} fighting men aboard",
                    send.force.id
                ),
            });
        }

        // 3. The fighting units of the forces limit not ashore at the place,
        // ashore at the source, that can be used and carried by then.
        let mut units: BTreeSet<LandUnitId> =
            self.state.land_units_on_chain(task.id).unwrap_or_default();
        if let Some(limit) = self.state.forces_as_bound(end) {
            units.extend(limit.land_units.iter().copied());
        }
        let mut at_source: Vec<LandUnitId> = Vec::new();
        for id in units {
            let Some(unit) = view.land_units().find(|u| u.id == id) else {
                // A fighting unit held apart until its day (decision 0195,
                // 13): more is coming from its day where it enters at the
                // source and the day is in time; where it enters elsewhere,
                // or out of time, it is named in the list of what does not
                // count.
                if let Some(entry) = view.entry_of(EntityRef::LandUnit(id)) {
                    let day = crate::available::entering_on(view, EntityRef::LandUnit(id))
                        .unwrap_or(entry.day);
                    let fighting = entry
                        .land_units
                        .get(&id)
                        .is_some_and(|u| !u.working && u.men_ashore > 0);
                    if !fighting {
                        continue;
                    }
                    if entry.task_force.is_some() {
                        // Aboard a held force: counted with that force's lift
                        // where it is the take's, above.
                        continue;
                    }
                    let at_source_base = matches!(
                        entry.place,
                        sim_core::entering::EntryPlace::Base(b)
                            if view.bases().any(|x| x.id == b && x.hex == source_hex)
                    );
                    if !in_time(Some(day)) {
                        not.push(format!(
                            "{}, {}",
                            crate::available::enters_words(id, day),
                            after_words(day)
                        ));
                    } else if !at_source_base {
                        not.push(format!(
                            "{} not at the source {source}",
                            crate::available::enters_words(id, day)
                        ));
                    } else {
                        found.push(format!(
                            "{id} enters at the source {source} on day {day}, to be carried by {}",
                            force.id
                        ));
                    }
                }
                continue;
            };
            if unit.working || unit.men_ashore == 0 || unit.hex == Some(place) {
                continue;
            }
            // Men aboard a ship are counted with the lifts, above.
            if view.men_aboard(id).unwrap_or(0) > 0 {
                continue;
            }
            let from = crate::available::not_before(view, EntityRef::LandUnit(id));
            if !in_time(from) {
                let from = from.unwrap_or_default();
                not.push(match game_last {
                    Some(last) if end_by.is_none_or(|by| from <= by) => format!(
                        "{} may be used from day {from}, after the game's last day, day {last}",
                        unit_words(unit)
                    ),
                    _ => format!(
                        "{} may be used from day {from}, after the latest day",
                        unit_words(unit)
                    ),
                });
                continue;
            }
            let target = sim_core::command::HoldTarget::LandUnit(id);
            if let Some(hold) = self.state.hold_of(target) {
                not.push(crate::reasons::hold_words(target, *hold));
                continue;
            }
            if unit.hex != Some(source_hex) {
                not.push(format!(
                    "{} is not ashore at the source {source}",
                    unit_words(unit)
                ));
                continue;
            }
            at_source.push(id);
        }
        if !at_source.is_empty() {
            let names = at_source
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<_>>()
                .join(" and ");
            if !self.afloat(force) {
                not.push(format!(
                    "{names} at the source: no ship of {}, the take's lift, is afloat to carry them",
                    force.id
                ));
            } else if bound_for(force) != source_hex {
                not.push(format!(
                    "{names} at the source: {} is bound for {}, not its source, and makes no second lift from there",
                    force.id,
                    bound_for(force)
                ));
            } else if let Some(why) =
                self.cannot_carry(end, force, &at_source, (source, source_hex), fit, wanted)
            {
                not.push(format!("{names} at the source: {why}"));
            } else {
                found.push(format!(
                    "{names} at the source, to be carried by {}",
                    force.id
                ));
            }
        }

        let more = !found.is_empty();
        let words = if more {
            format!("more is coming: {}", found.join("; "))
        } else {
            let mut words = format!(
                "nothing more is coming ({last_words}): no lift of the take has fighting men aboard, no lift as it stands with fighting men aboard can be sent, and no fighting unit of its forces limit off {place} can be used and carried"
            );
            if !not.is_empty() {
                words.push_str(&format!(" ({})", not.join("; ")));
            }
            words
        };
        Coming { more, words }
    }

    /// Where the first lift lies stopped at the source with nothing aboard:
    /// why it cannot carry the units at the source, by the choice of a
    /// second lift and "not to starve" as the morning would work them;
    /// `None` where it can, or where it does not lie there yet (it is to
    /// carry them when it does).
    fn cannot_carry(
        &self,
        end: &Intent,
        force: &TaskForce,
        at_source: &[LandUnitId],
        (source, source_hex): (BaseId, Hex),
        fit: u32,
        wanted: u32,
    ) -> Option<String> {
        let view = self.view;
        if !crate::fleet::force_stopped_at(force, source_hex) {
            return None;
        }
        let figures = view.ground_figures()?;
        let left: Vec<sim_core::state::LandUnit> = at_source
            .iter()
            .filter_map(|id| view.land_units().find(|u| u.id == *id).cloned())
            .collect();
        let mut lines = Vec::new();
        let picks = self.lift_picks(force, &left, Some(wanted), fit, &mut lines);
        if picks.is_empty() {
            return Some(format!(
                "no ship of {} can carry them ({})",
                force.id,
                lines.join("; ")
            ));
        }
        let ids: Vec<LandUnitId> = picks.iter().map(|(id, _)| *id).collect();
        let men: u32 = picks.iter().map(|(_, men)| *men).sum();
        let level = self
            .supply(
                force,
                &ids,
                source,
                source_hex,
                figures,
                None,
                self.draw_on(end, source),
            )
            .level;
        let fed = self.with_aboard(force, &ids);
        match self.starve_check(end, force, force.id, fit, men, &fed, level) {
            Some(starve) if !starve.fed => Some(format!(
                "not to starve forbids the lift: {} {}",
                starve.count, starve.fed_words
            )),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const RISKS: [Risk; 3] = [Risk::Preserve, Risk::Balanced, Risk::Press];

    #[test]
    fn the_table_by_risk() {
        assert_eq!(men_wanted_by(3_444, Risk::Preserve, true), 10_332);
        assert_eq!(men_wanted_by(3_444, Risk::Balanced, true), 6_888);
        assert_eq!(men_wanted_by(3_444, Risk::Press, true), 3_444);
        assert_eq!(men_wanted_by(3_444, Risk::Preserve, false), 10_332);
        assert_eq!(men_wanted_by(3_444, Risk::Balanced, false), 3_444);
        assert_eq!(men_wanted_by(3_444, Risk::Press, false), 0);
    }

    #[test]
    fn the_worked_example() {
        // Scenario 109: 2,700 and 2,386 fit, 5,086.
        let fit = 2_700 + 2_386;
        assert_eq!(fit, 5_086);
        // Statement 1: while more is coming, at balanced it waits.
        assert_eq!(
            verdict(fit, Some(3_444), Risk::Balanced, true, false),
            Verdict::Wait
        );
        // Statement 3: with nothing more coming it attacks.
        assert_eq!(
            verdict(fit, Some(3_444), Risk::Balanced, false, false),
            Verdict::Attack
        );
        // Statement 4: 5,200 is more than 5,086: it waits.
        assert_eq!(
            verdict(fit, Some(5_200), Risk::Balanced, false, false),
            Verdict::Wait
        );
        // Statement 5: press attacks with 5,200 and with 20,000.
        for estimate in [5_200, 20_000] {
            assert_eq!(
                verdict(fit, Some(estimate), Risk::Press, false, false),
                Verdict::Attack
            );
        }
        // Statement 6: preserve waits.
        assert_eq!(
            verdict(fit, Some(3_444), Risk::Preserve, false, false),
            Verdict::Wait
        );
        // Statement 9c: on attack, the estimate 9,000: committed.
        assert_eq!(
            verdict(fit, Some(9_000), Risk::Balanced, false, true),
            Verdict::Committed
        );
        // Statement 10: V's 2,386 alone against 3,444: waits.
        assert_eq!(
            verdict(2_386, Some(3_444), Risk::Balanced, false, false),
            Verdict::Wait
        );
        // Statement 11: no estimate: attacks.
        for risk in RISKS {
            for more in [true, false] {
                assert_eq!(verdict(0, None, risk, more, false), Verdict::Attack);
            }
        }
    }

    #[test]
    fn while_more_is_coming_the_judgement_is_as_before() {
        for risk in RISKS {
            for estimate in [0, 1, 500, 3_444, 9_000] {
                for fit in [0, 1, 499, 500, 1_000, 3_444, 6_888, 10_332, 20_000] {
                    for on_attack in [false, true] {
                        let before = if fit >= estimate * super::super::men_per_man(risk) {
                            Verdict::Attack
                        } else {
                            Verdict::Wait
                        };
                        assert_eq!(verdict(fit, Some(estimate), risk, true, on_attack), before);
                    }
                }
            }
        }
    }

    #[test]
    fn preserve_the_force_never_attacks_where_it_did_not() {
        for estimate in [0, 1, 500, 3_444, 9_000] {
            for fit in [0, 1, 499, 1_000, 3_444, 10_331, 10_332, 30_000] {
                for on_attack in [false, true] {
                    let more = verdict(fit, Some(estimate), Risk::Preserve, true, on_attack);
                    let nothing = verdict(fit, Some(estimate), Risk::Preserve, false, on_attack);
                    assert_eq!(more, nothing);
                }
            }
        }
    }

    #[test]
    fn the_men_wanted_with_nothing_coming_are_never_more() {
        for risk in RISKS {
            for estimate in [0, 1, 7, 3_444, 9_000] {
                assert!(
                    men_wanted_by(estimate, risk, false) <= men_wanted_by(estimate, risk, true)
                );
            }
        }
    }

    #[test]
    fn more_fit_men_never_turn_an_attack_into_a_wait() {
        for risk in RISKS {
            for more in [true, false] {
                for on_attack in [false, true] {
                    for estimate in [1, 500, 3_444] {
                        let mut was_attack = false;
                        for fit in 0..=12_000u32 {
                            let now = verdict(fit, Some(estimate), risk, more, on_attack);
                            if was_attack {
                                assert_eq!(now, Verdict::Attack);
                            }
                            was_attack = now == Verdict::Attack;
                        }
                    }
                }
            }
        }
    }

    #[test]
    fn a_larger_estimate_never_turns_a_wait_into_an_attack() {
        for risk in RISKS {
            for more in [true, false] {
                for on_attack in [false, true] {
                    for fit in [0, 1, 2_386, 5_086, 9_000] {
                        let mut was_attack = true;
                        for estimate in 0..=12_000u32 {
                            let now = verdict(fit, Some(estimate), risk, more, on_attack);
                            if !was_attack {
                                assert_ne!(now, Verdict::Attack);
                            }
                            was_attack = now == Verdict::Attack;
                        }
                    }
                }
            }
        }
    }

    #[test]
    fn a_lift_added_never_lowers_the_men_wanted() {
        // A lift added can only turn "nothing more is coming" into "more is
        // coming", never the other way.
        for risk in RISKS {
            for estimate in [0, 1, 3_444, 9_000] {
                assert!(
                    men_wanted_by(estimate, risk, true) >= men_wanted_by(estimate, risk, false)
                );
            }
        }
    }

    #[test]
    fn once_sent_in_they_stay_in_but_not_at_preserve_the_force() {
        assert_eq!(
            verdict(1_000, Some(3_444), Risk::Balanced, false, true),
            Verdict::Committed
        );
        assert_eq!(
            verdict(1_000, Some(3_444), Risk::Preserve, false, true),
            Verdict::Wait
        );
        // While more is coming a unit on attack is set back to defend, as
        // before.
        assert_eq!(
            verdict(1_000, Some(3_444), Risk::Balanced, true, true),
            Verdict::Wait
        );
    }

    #[test]
    fn the_days_left_on_a_morning() {
        let left: Vec<Option<u32>> = (1..=8).map(|d| days_left(Some(6), d)).collect();
        assert_eq!(
            left,
            [6, 5, 4, 3, 2, 1, 0, 0].map(Some).to_vec(),
            "a game of 6 on the mornings of days 1 to 8"
        );
        assert_eq!(days_left(Some(6), 0), Some(6));
        assert_eq!(days_left(Some(6), u32::MAX), Some(0));
        assert_eq!(days_left(None, 3), None);
        assert_eq!(days_left(None, 0), None);
    }

    #[test]
    fn the_last_days_are_fewer_than_three_left() {
        assert_eq!(LAST_DAYS, 3);
        assert!(in_last_days(Some(0)));
        assert!(in_last_days(Some(1)));
        assert!(in_last_days(Some(2)));
        assert!(!in_last_days(Some(3)));
        assert!(!in_last_days(Some(4)));
        assert!(!in_last_days(Some(u32::MAX)));
        assert!(!in_last_days(None));
    }

    #[test]
    fn the_last_days_in_words() {
        assert_eq!(
            last_days_words(2),
            "2 days of the game are left, fewer than 3"
        );
        assert_eq!(
            last_days_words(1),
            "1 day of the game is left, fewer than 3"
        );
        assert_eq!(
            last_days_words(0),
            "0 days of the game are left, fewer than 3"
        );
    }

    #[test]
    fn with_three_days_left_or_more_nothing_is_the_last_days() {
        for game in 1..=40u32 {
            for today in 1..=45u32 {
                let resolved = today - 1;
                let last = in_last_days(days_left(Some(game), today));
                assert_eq!(
                    last,
                    game < resolved + LAST_DAYS,
                    "game {game}, day {today}"
                );
                if game >= resolved + LAST_DAYS {
                    assert!(!last, "game {game}, day {today}");
                }
            }
        }
        // A world with no length never has a last day, on any morning.
        assert!((0..=45).all(|d| !in_last_days(days_left(None, d))));
    }
}
