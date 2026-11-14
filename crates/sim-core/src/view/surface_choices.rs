//! A side's questions of the surface fight and stragglers: a force alerted, ready when a fight begins, breaking off, its stragglers, and where a straggler or a retiring force goes (decision 0100, 2).
//!
//! Every answer is the engine's function (`alert_choice`, `force_ready`,
//! `break_off_test` in `surface.rs`; `straggler_plan`, `home_hex` in
//! `stragglers.rs`) fed from the side's own forces and ships, its belief, the
//! public bases and the figures. A question that names a force that is not
//! the side's, or a ship that is not the side's, is answered with `None`.

use crate::choices::surface::{SurfaceAnswer, SurfaceQuestion};
use crate::hex::Hex;
use crate::ids::{BaseId, TaskForceId};
use crate::orders::Readiness;
use crate::state::TaskForce;
use crate::stragglers::{home_hex, straggler_plan};
use crate::surface::{alert_choice, break_off_test, force_ready};
use crate::time::Pulse;
use crate::view::SideView;

impl SideView<'_> {
    /// One of the side's own forces, by number.
    fn own_force(&self, id: TaskForceId) -> Option<&TaskForce> {
        self.task_forces().find(|force| force.id == id)
    }

    /// The hex of a base the side can know: one of its own, or one on the
    /// public map ([`SideView::enemy_bases`]).
    fn known_base_hex(&self, id: BaseId) -> Option<Hex> {
        self.bases()
            .find(|base| base.id == id)
            .map(|base| base.hex)
            .or_else(|| {
                self.enemy_bases()
                    .find(|base| base.id == id)
                    .map(|base| base.hex)
            })
    }

    /// The answer to `question` from what this side holds now. `None`
    /// where the question names no unit of this side, or the choice does not
    /// arise (alert from belief skips a force with no ship afloat or at
    /// battle readiness, and is not made at all where the scenario names no
    /// surface combat figures).
    pub(crate) fn ask_surface(&self, question: &SurfaceQuestion) -> Option<SurfaceAnswer> {
        match question {
            SurfaceQuestion::Alert { task_force } => {
                let force = self.own_force(*task_force)?;
                let figures = self.ref_data.surface_combat.as_ref()?;
                // Without search figures no side can hold a contact, so the
                // age limit is never read.
                let fresh_max_age = self
                    .ref_data
                    .search
                    .as_ref()
                    .map_or(0, |search| search.fresh_contact_max_age_pulses);
                alert_choice(
                    force,
                    self.belief.contacts(),
                    figures.alert_contact_distance_hexes,
                    fresh_max_age,
                )
                .map(SurfaceAnswer::Alert)
            }
            SurfaceQuestion::Ready {
                task_force,
                day,
                pulse,
            } => {
                let force = self.own_force(*task_force)?;
                // The event that alerted the force is the force's own.
                let alerted_at = force
                    .alerted_by
                    .and_then(|event| self.log.get(event))
                    .map(|event| (event.day, event.pulse));
                Some(SurfaceAnswer::Ready(force_ready(
                    force, alerted_at, *day, *pulse,
                )))
            }
            SurfaceQuestion::BreakOff {
                task_force,
                began_with,
                fired_on,
                already_avoiding,
            } => {
                let force = self.own_force(*task_force)?;
                let figures = self.ref_data.surface_combat.as_ref()?;
                // Only the side's own ships are read.
                if !began_with.iter().all(|id| {
                    self.truth
                        .ships
                        .get(id)
                        .is_some_and(|ship| ship.side == self.side)
                }) {
                    return None;
                }
                let test = break_off_test(
                    &force.rules_of_engagement,
                    began_with,
                    &self.truth.ships,
                    figures.crippled_flotation,
                    (*already_avoiding, *fired_on),
                );
                Some(SurfaceAnswer::BreakOff(test.breaks_off))
            }
            SurfaceQuestion::Stragglers { task_force } => {
                let force = self.own_force(*task_force)?;
                let figures = self.ref_data.surface_combat.as_ref()?;
                Some(SurfaceAnswer::Stragglers(straggler_plan(
                    force,
                    &self.truth.ships,
                    &self.ref_data.ship_classes,
                    figures,
                )))
            }
            SurfaceQuestion::HeadHome { task_force } => {
                let force = self.own_force(*task_force)?;
                let home = force.home_port.and_then(|port| self.known_base_hex(port));
                Some(SurfaceAnswer::HeadHome(home_hex(force, home)))
            }
        }
    }

    /// Every question of this kind this side could ask now, one for each of
    /// its forces the choice applies to.
    ///
    /// Alert is asked of each force with a ship afloat that is not at battle
    /// readiness. Ready is asked as at the first pulse of the day being
    /// resolved: the view knows the last day resolved, not the pulse, so the
    /// question is for `day + 1` and the night pulse. Break-off is asked of
    /// each force with its present ships as those it began the fight with,
    /// not fired on and not yet avoiding (the three inputs that exist only
    /// inside a fight). Stragglers and head-home are asked of every force
    /// with a ship afloat.
    pub(crate) fn surface_questions(&self) -> Vec<SurfaceQuestion> {
        let mut questions = Vec::new();
        for force in self.task_forces() {
            let task_force = force.id;
            if !force.ships.is_empty() && force.readiness != Readiness::Battle {
                questions.push(SurfaceQuestion::Alert { task_force });
            }
            questions.push(SurfaceQuestion::Ready {
                task_force,
                day: self.truth.day + 1,
                pulse: Pulse::Night,
            });
            if !force.ships.is_empty() {
                questions.push(SurfaceQuestion::BreakOff {
                    task_force,
                    began_with: force.ships.iter().copied().collect(),
                    fired_on: false,
                    already_avoiding: false,
                });
                questions.push(SurfaceQuestion::Stragglers { task_force });
                questions.push(SurfaceQuestion::HeadHome { task_force });
            }
        }
        questions
    }
}
