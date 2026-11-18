//! One meeting of two opposing task forces and the rounds of its fight
//! (decisions 0014 and 0022), and which forces meet.

use super::*;

impl World {
    /// One meeting of two opposing task forces (decision 0022): the fight of
    /// decision 0014 in full, then the alarm for any bystander in the hex if
    /// shots were fired. `a` is the lower task force ID. The caller has
    /// closed the forces' steaming segments and deals with stragglers after.
    /// `elapsed_hours` is when the meeting happens, and `arrival` the steaming
    /// event that brought a force into the hex, if one did. `avoiding` is the
    /// set of forces that broke off or slipped away earlier in the pulse.
    /// `alarm_streams` holds
    /// one alarm stream per force per pulse, opened the first time the force
    /// rolls and kept across every fight of the movement step (decision 9 of
    /// 0021): the force's draws come in the order the fights end.
    pub(crate) fn hold_meeting(
        &mut self,
        day: u32,
        pulse: Pulse,
        figures: &SurfaceCombatFigures,
        (a, b): (TaskForceId, TaskForceId),
        (elapsed_hours, arrival): (Fixed, Option<EventId>),
        (avoiding, alarm_streams): (
            &mut BTreeSet<TaskForceId>,
            &mut BTreeMap<TaskForceId, RngStream>,
        ),
    ) {
        let mut fight = self.begin_fight(day, pulse, figures, (a, b), (elapsed_hours, arrival));
        fight.avoiding = std::mem::take(avoiding);
        let ending = self.fight_rounds(&mut fight);
        let ended = self.end_fight(&fight, &ending);
        if fight.fired {
            self.sound_alarm(&fight, ended, alarm_streams);
        }
        *avoiding = fight.avoiding;
    }

    /// Whether a force is ready for action as a fight begins (decision 0068,
    /// 2): at battle readiness by order, or alert since before the present
    /// pulse began. A force alerted in the present pulse (by a contact at the
    /// pulse's start, by being sighted or fired on in an earlier fight of the
    /// pulse, or by the alarm) is not. When the force was alerted is read from
    /// the event that alerted it, `alerted_by`; an alerted force with none
    /// (a state a rule never makes) counts as alerted before.
    pub(super) fn force_is_ready(&self, id: TaskForceId, day: u32, pulse: Pulse) -> bool {
        let force = &self.truth.task_forces[&id];
        let alerted_at = force
            .alerted_by
            .and_then(|event| self.events.get(event))
            .map(|event| (event.day, event.pulse));
        force_ready(force, alerted_at, day, pulse)
    }

    /// The stance a force takes in a fight: avoid if it broke off or slipped
    /// away earlier in the pulse, whatever its orders say (decision 9 of
    /// `docs/decisions/0022-meeting-and-breaking-off.md`), else its ordered
    /// stance.
    pub(super) fn effective_stance(&self, fight: &Fight, force: TaskForceId) -> Stance {
        if fight.avoiding.contains(&force) {
            Stance::Avoid
        } else {
            self.truth.task_forces[&force].rules_of_engagement.stance
        }
    }

    /// Opposing task forces with the same position, lower ID first, in order.
    pub(crate) fn opposing_pairs_in_one_hex(&self) -> Vec<(TaskForceId, TaskForceId)> {
        let forces: Vec<_> = self.truth.task_forces.values().collect();
        let mut pairs = Vec::new();
        for (index, a) in forces.iter().enumerate() {
            for b in &forces[index + 1..] {
                if a.side != b.side && a.position() == b.position() {
                    pairs.push((a.id, b.id));
                }
            }
        }
        pairs
    }

    fn begin_fight(
        &mut self,
        day: u32,
        pulse: Pulse,
        figures: &SurfaceCombatFigures,
        (a, b): (TaskForceId, TaskForceId),
        (elapsed_hours, arrival): (Fixed, Option<EventId>),
    ) -> Fight {
        let hex = self.truth.task_forces[&a].position();
        // The steaming that brought a force into the hex comes first, then
        // the orders.
        let mut causes: Vec<Cause> = arrival.map(Cause::Event).into_iter().collect();
        causes.extend(
            [a, b]
                .into_iter()
                .filter_map(|id| {
                    let force = &self.truth.task_forces[&id];
                    force.rules_order.or(force.route_order)
                })
                .map(Cause::Order),
        );
        let begun = self.events.record(
            day,
            Some(pulse),
            EventKind::EngagementBegun {
                a,
                b,
                hex,
                elapsed_hours,
            },
            causes,
            None,
        );
        // A force in a surface fight breaks radio silence (decision 0136,
        // 17).
        self.break_silence(a, day, pulse);
        self.break_silence(b, day, pulse);
        let force_fight = |world: &World, id: TaskForceId, enemy: TaskForceId| {
            let force = &world.truth.task_forces[&id];
            ForceFight {
                id,
                side: force.side,
                sighted: false,
                sight_roll: None,
                fired_on_last_round: false,
                began_with: force.ships.iter().copied().collect(),
                fired_this_fight: false,
                ready: {
                    let ready = world.force_is_ready(id, day, pulse);
                    world.trace_choice(force.side, || {
                        (
                            Question::Surface(SurfaceQuestion::Ready {
                                task_force: id,
                                day,
                                pulse,
                            }),
                            Answer::Surface(SurfaceAnswer::Ready(ready)),
                        )
                    });
                    ready
                },
                sunk_in_fight: BTreeSet::new(),
                sighting_stream: world.rng_pair(
                    day,
                    Some(pulse),
                    Subsystem::SurfaceCombat,
                    Draw::Legacy,
                    EntityRef::TaskForce(id),
                    EntityRef::TaskForce(enemy),
                ),
                rules_order: force.rules_order,
            }
        };
        Fight {
            day,
            pulse,
            hex,
            dark: time::is_dark(hex, pulse),
            figures: figures.clone(),
            begun,
            forces: [force_fight(self, a, b), force_fight(self, b, a)],
            fire_streams: BTreeMap::new(),
            fired: false,
            avoiding: BTreeSet::new(),
        }
    }

    /// Runs rounds until one of the ending checks ends the fight.
    fn fight_rounds(&mut self, fight: &mut Fight) -> Ending {
        for number in 1..=fight.figures.max_rounds {
            let mut round = Round::new(number);
            self.sight(fight, &mut round);
            if let Some(slipped) = self.avoid(fight, &round) {
                // The fight ends at once: no fire and no damage. Forces that
                // sighted this round still become alert.
                self.become_alert(fight, &round);
                return Ending {
                    round: number,
                    reason: EngagementEndReason::SlippedAway,
                    causes: vec![Cause::Event(slipped)],
                };
            }
            self.fire(fight, &mut round);
            self.apply_damage(fight, &mut round);
            self.become_alert(fight, &round);
            for (index, force) in fight.forces.iter_mut().enumerate() {
                force.fired_on_last_round = round.forces[index].fired_on;
            }
            if let Some(ending) = self.check_ending(fight, &round) {
                return ending;
            }
        }
        unreachable!("the check for the last round always ends the fight")
    }
}
