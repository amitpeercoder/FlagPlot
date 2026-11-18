//! A plan told to wait (decision 0159, rule C, items 9 to 11): the limits
//! "not before" a day, "wait for" a task force or a land unit, and "after"
//! another "take a place" or "control an island", read up the tree from a
//! "take a place" ([`sim_core::command::CommandState::waits_read_up`]).
//!
//! - **Unmet** (item 9): "not before" on a morning before its day; "wait
//!   for" until the side may use the force or unit and it lies at the source
//!   the take's forces limit names, a force stopped there or a unit with men
//!   ashore in its hex; "after" until, on a morning, the named intent's place
//!   is the side's (for an island, every place of it), read from the side's
//!   own view.
//! - **Not released** (item 10): while any is unmet the staff readies the
//!   landing as it does without them and does not release the plan; it does
//!   not ask leave to launch, and the morning's reason names what is waited
//!   for, with the day where there is one. The gate sits beside the need to
//!   know of decision 0147 ([`Day::need_to_know`]) and is read after it, in
//!   [`Day::launch_place`](crate::take): all must hold before a release.
//! - **Never** (item 11): a limit that can never be met as it stands (an
//!   "after" naming an intent that is withdrawn, a "wait for" naming a force
//!   or unit the side no longer has) is said once, with the notice that the
//!   intent cannot be met; the reason names the limit and the plan stays
//!   preparing.
//!
//! The doctrine reads the side's own view and its planned command state and
//! nothing else.

use sim_core::command::{Awaited, Intent, IntentKind, NoticeKind, Plan};
use sim_core::hex::Hex;
use sim_core::ids::EntityRef;

use crate::air::Day;
use crate::available;
use crate::fleet::force_stopped_at;
use crate::reasons::serving;

impl Day<'_, '_> {
    /// Items 9 to 11, for a "take a place" whose landing is ready and whose
    /// plan is preparing: `true` where a limit that tells it to wait is unmet
    /// and the reason (or the notice) is handed in, so that the plan is not
    /// released this morning; `false` where none is set or every one is met.
    pub(crate) fn told_to_wait(&mut self, end: &Intent, plan: &Plan, ready_words: &str) -> bool {
        let limits = self.state.waits_read_up(end.id);
        if limits.is_empty() {
            return false;
        }
        let IntentKind::TakePlace { hex: place, .. } = end.kind else {
            return false;
        };
        let in_scope = self.in_scope(end.id);
        let today = self.today;
        let view = self.view;
        // What is waited for, and what can never come, each in words.
        let mut unmet: Vec<String> = Vec::new();
        let mut never: Vec<String> = Vec::new();
        let set_on = |on: sim_core::ids::IntentId| {
            if on == end.id {
                String::new()
            } else {
                format!(", set on {on}")
            }
        };

        if let Some((day, on)) = limits.not_before {
            if today < day {
                unmet.push(format!(
                    "the commander's limit not before day {day}{}: it is not to go before day {day}",
                    set_on(on)
                ));
            }
        }

        let source = self
            .state
            .forces_as_bound(end)
            .and_then(|f| f.source)
            .and_then(|s| view.bases().find(|b| b.id == s).map(|b| (s, b.hex)));
        for (awaited, on) in &limits.wait_for {
            let limit_words = format!("the commander's limit wait for {awaited}{}", set_on(*on));
            match *awaited {
                Awaited::TaskForce(id) => {
                    let Some(force) = view
                        .task_forces()
                        .find(|f| f.id == id && !f.ships.is_empty())
                    else {
                        never.push(format!("{limit_words}: the side no longer has {awaited}"));
                        continue;
                    };
                    let from = available::force_waiting(view, id);
                    let at = source.is_some_and(|(_, hex)| force_stopped_at(force, hex));
                    if let Some(words) = awaited_words(&limit_words, *awaited, from, at, source) {
                        unmet.push(words);
                    }
                }
                Awaited::LandUnit(id) => {
                    let Some(unit) = view.land_units().find(|u| u.id == id) else {
                        never.push(format!("{limit_words}: the side no longer has {awaited}"));
                        continue;
                    };
                    let aboard = view
                        .ships()
                        .any(|s| s.troops.as_ref().is_some_and(|t| t.unit == id && t.men > 0));
                    if unit.men_ashore == 0 && !aboard {
                        never.push(format!(
                            "{limit_words}: the side no longer has a man of {awaited}"
                        ));
                        continue;
                    }
                    let from = available::not_before(view, EntityRef::LandUnit(id));
                    let at =
                        source.is_some_and(|(_, hex)| unit.men_ashore > 0 && unit.hex == Some(hex));
                    if let Some(words) = awaited_words(&limit_words, *awaited, from, at, source) {
                        unmet.push(words);
                    }
                }
            }
        }

        for (other, on) in &limits.after {
            let limit_words = format!("the commander's limit after {other}{}", set_on(*on));
            let named = self.state.intents.get(other).cloned();
            let places: Vec<Hex> = match named.as_ref().map(|i| i.kind) {
                Some(IntentKind::TakePlace { hex, .. }) => vec![hex],
                Some(IntentKind::ControlIsland { island, .. }) => view
                    .island(island)
                    .map(|found| found.places.iter().map(|p| p.hex).collect())
                    .unwrap_or_default(),
                _ => Vec::new(),
            };
            let met = !places.is_empty() && places.iter().all(|hex| self.place_is_ours(*hex));
            if met {
                continue;
            }
            match named {
                Some(found) if found.in_force() => {
                    let not_yet: Vec<String> = places
                        .iter()
                        .filter(|hex| !self.place_is_ours(**hex))
                        .map(ToString::to_string)
                        .collect();
                    unmet.push(format!(
                        "{limit_words}: the place of {other} at {} is not yet the side's",
                        not_yet.join(" and ")
                    ));
                }
                _ => never.push(format!(
                    "{limit_words}: {other} is withdrawn, so its place will not be taken for it"
                )),
            }
        }

        if unmet.is_empty() && never.is_empty() {
            return false;
        }
        let mut reason = serving(end, plan).evidence(ready_words.to_string(), today);
        for words in never.iter().chain(unmet.iter()) {
            reason = reason.binding(words.clone());
        }
        if never.is_empty() {
            let reason = reason
                .worked(format!(
                    "The landing at {place} is ready ({ready_words}), but the commander has told the staff to wait: {}. It does not ask leave to launch and does not release {} until then.",
                    unmet.join("; "),
                    plan.id
                ))
                .would_change("the limit met, or the commander clearing it")
                .done();
            self.target(in_scope).no_orders.push(reason);
            return true;
        }
        let noticed = self.noticed(NoticeKind::CannotMeet, plan.id);
        let also = if unmet.is_empty() {
            String::new()
        } else {
            format!(" It also waits for this: {}.", unmet.join("; "))
        };
        let reason = reason
            .worked(format!(
                "The landing at {place} is ready ({ready_words}), but a limit the commander set can never be met as it stands: {}.{also} The staff does not ask leave to launch and does not release {}{}.",
                never.join("; "),
                plan.id,
                if noticed {
                    ""
                } else {
                    ", and hands in the notice that it cannot meet the intent"
                }
            ))
            .would_change("the commander clearing or changing the limit")
            .done();
        if noticed {
            self.target(in_scope).no_orders.push(reason);
        } else {
            self.notice(in_scope, reason, NoticeKind::CannotMeet, plan);
        }
        true
    }
}

/// What a "wait for" still waits for, in words, or `None` where it is met:
/// the day the side may use the force or unit, and whether it lies at the
/// source.
fn awaited_words(
    limit_words: &str,
    awaited: Awaited,
    from: Option<u32>,
    at: bool,
    source: Option<(sim_core::ids::BaseId, Hex)>,
) -> Option<String> {
    let mut parts: Vec<String> = Vec::new();
    if let Some(day) = from {
        parts.push(available::named_with_day(awaited, day));
    }
    if !at {
        let lies = match awaited {
            Awaited::TaskForce(_) => "is not stopped at",
            Awaited::LandUnit(_) => "has no men ashore at",
        };
        match source {
            Some((base, hex)) => {
                parts.push(format!("{awaited} {lies} the source, {base} at {hex}"))
            }
            None => parts.push(format!(
                "the take's forces limit names no source of the side for {awaited} to lie at"
            )),
        }
    }
    if parts.is_empty() {
        None
    } else {
        Some(format!("{limit_words}: {}", parts.join(", and ")))
    }
}
