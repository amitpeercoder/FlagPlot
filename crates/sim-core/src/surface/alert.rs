//! Alertness: the alarm after a fight, alerting from belief, and the
//! tiring and resting of forces (decisions 0021 and 0068).

use super::*;

/// What alert from belief does to a force as a pulse begins (decision 3 of
/// `docs/decisions/0021-readiness-and-alarm.md`, decision 0100, 1). `None`
/// for a force the rule skips: one with no ship afloat, or at battle
/// readiness, which a contact neither alerts nor stands down. Otherwise the
/// nearest of `contacts` no older than `fresh_max_age` pulses and within
/// `alert_distance` hexes of the force alerts it if it is not alerted
/// (the lowest ID on a tie, as the contacts come in ID order), and with
/// none near an alerted force stands down. The caller hands in the belief
/// of the force's own side. It reads nothing else and draws nothing.
pub(crate) fn alert_choice<'a>(
    force: &TaskForce,
    contacts: impl Iterator<Item = &'a Contact>,
    alert_distance: u32,
    fresh_max_age: u32,
) -> Option<AlertChoice> {
    if force.ships.is_empty() || force.readiness == Readiness::Battle {
        return None;
    }
    let position = force.position();
    // Contacts come in ID order and min_by_key keeps the first minimum, so a
    // tie goes to the lowest ID.
    let nearest = contacts
        .filter(|contact| {
            contact.age_pulses <= fresh_max_age
                && contact.estimated_hex.distance(position) <= alert_distance
        })
        .min_by_key(|contact| contact.estimated_hex.distance(position))
        .map(|contact| contact.id);
    Some(match (force.alerted, nearest) {
        (false, Some(contact)) => AlertChoice::AlertedBy(contact),
        (true, None) => AlertChoice::StandDown,
        (true, Some(_)) | (false, None) => AlertChoice::NoChange,
    })
}

/// Whether a force is ready for action as a fight begins in `pulse` of `day`
/// (decision 0068, 2): at battle readiness by order, or alert since before
/// the present pulse began. `alerted_at` is the day and pulse of the event
/// that alerted the force (its `alerted_by`), `None` where it has none or the
/// log does not hold it: an alerted force with none counts as alerted
/// before. It reads the force only.
pub(crate) fn force_ready(
    force: &TaskForce,
    alerted_at: Option<(u32, Option<Pulse>)>,
    day: u32,
    pulse: Pulse,
) -> bool {
    if force.readiness == Readiness::Battle {
        return true;
    }
    if !force.alerted {
        return false;
    }
    alerted_at.is_none_or(|(alerted_day, alerted_pulse)| {
        alerted_day < day || (alerted_day == day && alerted_pulse.is_none_or(|p| p < pulse))
    })
}

impl World {
    /// The alarm of a fight in which shots were fired (decision 5 of
    /// `docs/decisions/0021-readiness-and-alarm.md`). Every other task force
    /// in the fight hex with a ship afloat that is not alert draws once
    /// against the alarm chance, in task force ID order. Every roll is
    /// recorded, whether or not it succeeds, by an `AlarmRoll` event citing
    /// the end of the fight. On success the force is alerted for the rest of
    /// the pulse and `alerted_by` is that event; a failed roll changes nothing
    /// else.
    ///
    /// The draw is `RngStream::below(SCALE)` compared with the chance, which
    /// is exactly what `RngStream::chance` does, taken in two steps so the
    /// draw can go in the breakdown.
    pub(super) fn sound_alarm(
        &mut self,
        fight: &Fight,
        ended: EventId,
        streams: &mut BTreeMap<TaskForceId, RngStream>,
    ) {
        let parties = [fight.forces[0].id, fight.forces[1].id];
        let chance = fight.figures.alarm_chance;
        let hearers: Vec<TaskForceId> = self
            .truth
            .task_forces
            .values()
            .filter(|force| {
                force.position() == fight.hex
                    && !force.ships.is_empty()
                    && !parties.contains(&force.id)
                    && !force.is_alert()
            })
            .map(|force| force.id)
            .collect();
        for id in hearers {
            let stream = streams.entry(id).or_insert_with(|| {
                self.rng(
                    fight.day,
                    Some(fight.pulse),
                    Subsystem::SurfaceCombat,
                    Draw::Legacy,
                    EntityRef::TaskForce(id),
                )
            });
            let draw = stream.below(Fixed::SCALE.unsigned_abs());
            let alarmed = draw < raw_u64(chance.clamp(Fixed::ZERO, Fixed::ONE));
            let breakdown = self.config.record_breakdowns.then(|| Breakdown {
                factors: vec![
                    Factor {
                        name: "alarm_chance",
                        value: chance,
                    },
                    Factor {
                        name: "draw",
                        value: Fixed::from_raw(i64::try_from(draw).expect("a draw fits in i64")),
                    },
                ],
                result: chance,
            });
            let event = self.record_in_fight(
                fight,
                EventKind::AlarmRoll {
                    task_force: id,
                    by_a: parties[0],
                    by_b: parties[1],
                    chance,
                    alarmed,
                },
                vec![Cause::Event(ended)],
                breakdown,
            );
            if !alarmed {
                continue;
            }
            let force = self
                .truth
                .task_forces
                .get_mut(&id)
                .expect("task force exists");
            force.alerted = true;
            force.alerted_by = Some(event);
        }
    }

    /// Alert from belief (decision 3 of
    /// `docs/decisions/0021-readiness-and-alarm.md`), at the very start of
    /// every pulse, before its first phase. A task force with a ship afloat
    /// is alerted by a contact when its side's belief holds one no older than
    /// `fresh_contact_max_age_pulses` within `alert_contact_distance_hexes`
    /// of the force. The contact cited is the nearest, the lowest ID on a
    /// tie. A task force at battle readiness is neither alerted nor stood
    /// down by a contact: the rule skips it, records nothing for it and
    /// leaves its `alerted` and `alerted_by` alone. Reads belief only, never
    /// the truth of enemy units.
    pub(crate) fn alert_from_belief(&mut self, day: u32, pulse: Pulse) {
        let figures = self.ref_data.surface_combat.clone();
        let fresh_max_age = self.ref_data.search.fresh_contact_max_age_pulses;
        let decisions: Vec<(TaskForceId, AlertChoice)> = self
            .truth
            .task_forces
            .values()
            .filter_map(|force| {
                let choice = alert_choice(
                    force,
                    self.picture(force.side, self.command_of_force(force.id))
                        .contacts(),
                    figures.alert_contact_distance_hexes,
                    fresh_max_age,
                )?;
                self.trace_choice(force.side, || {
                    (
                        Question::Surface(SurfaceQuestion::Alert {
                            task_force: force.id,
                        }),
                        Answer::Surface(SurfaceAnswer::Alert(choice)),
                    )
                });
                Some((force.id, choice))
            })
            .collect();
        for (id, choice) in decisions {
            let alerted_by = self.truth.task_forces[&id].alerted_by;
            match choice {
                AlertChoice::AlertedBy(contact) => {
                    let side = self.truth.task_forces[&id].side;
                    let report = self
                        .picture(side, self.command_of_force(id))
                        .contact(contact)
                        .expect("the contact is in the belief")
                        .last_report_event;
                    let event = self.events.record(
                        day,
                        Some(pulse),
                        EventKind::ForceAlertedByContact {
                            task_force: id,
                            contact,
                        },
                        vec![Cause::Event(report)],
                        None,
                    );
                    let force = self.truth.task_forces.get_mut(&id).expect("force exists");
                    force.alerted = true;
                    force.alerted_by = Some(event);
                }
                AlertChoice::StandDown => {
                    self.events.record(
                        day,
                        Some(pulse),
                        EventKind::ForceStoodDown { task_force: id },
                        alerted_by.into_iter().map(Cause::Event).collect(),
                        None,
                    );
                    let force = self.truth.task_forces.get_mut(&id).expect("force exists");
                    force.alerted = false;
                    force.alerted_by = None;
                }
                // Still alerted by a contact: alerted_by stays. Not alerted
                // and no contact: nothing.
                AlertChoice::NoChange => {}
            }
        }
    }

    /// Fatigue of the task forces (decision 2 of
    /// `docs/decisions/0037-fatigue-at-battle-readiness.md`), at the start of
    /// every pulse, right after alert from belief has settled which forces
    /// are alert. Each task force with a ship afloat, in ID order, that is
    /// alert gains the alert figure, 100 at most; one that is not loses the
    /// rest figure, 0 at least. An event is recorded only when the fatigue
    /// changes. It cites the readiness order when the force is at battle
    /// readiness by order, and otherwise the event that alerted it; a force
    /// resting cites its readiness order if it has one. Does nothing where
    /// the scenario names no fatigue figures. No randomness.
    pub(crate) fn tire_and_rest_forces(&mut self, day: u32, pulse: Pulse) {
        let Some(figures) = self.ref_data.fatigue else {
            return;
        };
        let full = Fixed::from_int(100);
        let ids: Vec<TaskForceId> = self
            .truth
            .task_forces
            .values()
            .filter(|force| !force.ships.is_empty())
            .map(|force| force.id)
            .collect();
        for id in ids {
            let force = &self.truth.task_forces[&id];
            let before = force.fatigue;
            let alert = force.is_alert();
            let (change, after, figure, causes) = if alert {
                let cause = if force.readiness == Readiness::Battle {
                    force.readiness_order.map(Cause::Order)
                } else {
                    force.alerted_by.map(Cause::Event)
                };
                (
                    ForceFatigueChange::Gained,
                    (before + figures.force_fatigue_per_pulse_alert).min(full),
                    figures.force_fatigue_per_pulse_alert,
                    cause.into_iter().collect::<Vec<_>>(),
                )
            } else {
                (
                    ForceFatigueChange::Rested,
                    (before - figures.force_rest_per_pulse).max(Fixed::ZERO),
                    figures.force_rest_per_pulse,
                    force
                        .readiness_order
                        .map(Cause::Order)
                        .into_iter()
                        .collect::<Vec<_>>(),
                )
            };
            if after == before {
                continue;
            }
            let breakdown = self.config.record_breakdowns.then(|| Breakdown {
                factors: vec![
                    Factor {
                        name: "fatigue_before",
                        value: before,
                    },
                    Factor {
                        name: match change {
                            ForceFatigueChange::Gained => "force_fatigue_per_pulse_alert",
                            ForceFatigueChange::Rested => "force_rest_per_pulse",
                        },
                        value: figure,
                    },
                ],
                result: after,
            });
            self.events.record(
                day,
                Some(pulse),
                EventKind::TaskForceFatigue {
                    task_force: id,
                    change,
                    amount: match change {
                        ForceFatigueChange::Gained => after - before,
                        ForceFatigueChange::Rested => before - after,
                    },
                    fatigue: after,
                },
                causes,
                breakdown,
            );
            self.truth
                .task_forces
                .get_mut(&id)
                .expect("task force exists")
                .fatigue = after;
        }
    }
}
