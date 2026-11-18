//! Steps (a) and (b) of a round: sighting, and a force that avoids
//! slipping away (decision 0014).

use super::*;

impl World {
    // ---- (a) sighting ----

    /// Each force that has not sighted the enemy rolls to, lower ID first. A
    /// force that was fired on last round has sighted, with no roll and no
    /// draw.
    pub(super) fn sight(&mut self, fight: &mut Fight, round: &mut Round) {
        for index in 0..2 {
            if fight.forces[index].sighted {
                continue;
            }
            if fight.forces[index].fired_on_last_round {
                fight.forces[index].sighted = true;
                round.forces[index].newly_sighted = true;
                continue;
            }
            let id = fight.forces[index].id;
            let enemy = fight.forces[Fight::enemy(index)].id;
            let figures = &fight.figures;
            let base = if fight.dark {
                figures.sighting_chance_night
            } else {
                figures.sighting_chance_day
            };
            let radar_bonus = if self.has_radar(id) {
                figures.radar_sighting_bonus
            } else {
                Fixed::ZERO
            };
            let alert_factor = if self.truth.task_forces[&id].is_alert() {
                Fixed::ONE
            } else {
                figures.not_alert_sighting_factor
            };
            // Fatigue cuts the chance to sight, and only that (decision 3 of
            // `docs/decisions/0037-fatigue-at-battle-readiness.md`). The
            // force's fatigue is as it stood at the start of the pulse. With
            // no fatigue figures the factor is one and is left out of the
            // breakdown.
            let fatigue_in_play = self.ref_data.fatigue.is_some();
            let fatigue_factor = if fatigue_in_play {
                (Fixed::from_int(100) - self.truth.task_forces[&id].fatigue) / Fixed::from_int(100)
            } else {
                Fixed::ONE
            };
            // Experience reads in a dark fight only, and only when its
            // strength is above zero (decision 0064). The force's value is
            // the mean of the ships it has afloat now, in the fixed
            // arithmetic.
            let experience_factor =
                (fight.dark && figures.experience_sighting_strength > Fixed::ZERO).then(|| {
                    experience_factor(
                        figures.experience_sighting_strength,
                        self.mean_experience(id),
                    )
                });
            let chance =
                ((base + radar_bonus) * alert_factor * experience_factor.unwrap_or(Fixed::ONE))
                    .min(Fixed::ONE)
                    * fatigue_factor;
            let sighted = fight.forces[index].sighting_stream.chance(chance);

            let breakdown = self.config.record_breakdowns.then(|| {
                let mut factors = vec![
                    Factor {
                        name: "base",
                        value: base,
                    },
                    Factor {
                        name: "radar_bonus",
                        value: radar_bonus,
                    },
                    Factor {
                        name: "alert_factor",
                        value: alert_factor,
                    },
                ];
                if let Some(value) = experience_factor {
                    factors.push(Factor {
                        name: "experience_factor",
                        value,
                    });
                }
                if fatigue_in_play {
                    factors.push(Factor {
                        name: "fatigue_factor",
                        value: fatigue_factor,
                    });
                }
                Breakdown {
                    factors,
                    result: chance,
                }
            });
            let event = self.record_in_fight(
                fight,
                EventKind::SightingRoll {
                    round: round.number,
                    task_force: id,
                    enemy,
                    chance,
                    fatigue_factor,
                    sighted,
                },
                vec![Cause::Event(fight.begun)],
                breakdown,
            );
            if sighted {
                fight.forces[index].sighted = true;
                fight.forces[index].sight_roll = Some((round.number, event));
                round.forces[index].newly_sighted = true;
                round.forces[index].rolled = Some(event);
            }
        }
    }

    /// The mean experience of the ships a force has afloat: the sum of their
    /// whole-number values over their count, in `Fixed` (decision 0064, 6).
    /// A force with no ship afloat has the standard value, though no fight
    /// holds one.
    pub(super) fn mean_experience(&self, force: TaskForceId) -> Fixed {
        let ships = &self.truth.task_forces[&force].ships;
        if ships.is_empty() {
            return Fixed::from_int(i64::from(Ship::DEFAULT_EXPERIENCE));
        }
        let sum: i64 = ships
            .iter()
            .map(|ship| i64::from(self.truth.ships[ship].experience))
            .sum();
        let count = i64::try_from(ships.len()).expect("a force's ships fit in i64");
        Fixed::from_int(sum) / Fixed::from_int(count)
    }

    /// True if any ship afloat in the force has a radar class.
    fn has_radar(&self, force: TaskForceId) -> bool {
        self.truth.task_forces[&force]
            .ships
            .iter()
            .any(|ship| self.class_of(*ship).radar)
    }

    // ---- (b) avoid ----

    /// A force that avoids, has sighted and faces an enemy that has not,
    /// slips away. Returns the `ForceSlippedAway` event if one did. If both
    /// have sighted, neither slips.
    pub(super) fn avoid(&mut self, fight: &mut Fight, round: &Round) -> Option<EventId> {
        for index in 0..2 {
            let force = &fight.forces[index];
            let enemy = &fight.forces[Fight::enemy(index)];
            if self.effective_stance(fight, force.id) != Stance::Avoid
                || !force.sighted
                || enemy.sighted
            {
                continue;
            }
            let sighted_now = force
                .sight_roll
                .filter(|(number, _)| *number == round.number);
            let mut causes = vec![Cause::Event(sighted_now.map_or(fight.begun, |(_, e)| e))];
            causes.extend(force.rules_order.map(Cause::Order));
            let (slipper, enemy_id) = (force.id, enemy.id);
            let event = self.record_in_fight(
                fight,
                EventKind::ForceSlippedAway {
                    round: round.number,
                    task_force: slipper,
                    enemy: enemy_id,
                },
                causes,
                None,
            );
            // A force that slips away avoids for the rest of the pulse.
            fight.avoiding.insert(slipper);
            return Some(event);
        }
        None
    }
}
