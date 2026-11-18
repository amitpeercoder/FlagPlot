//! Step (d) of a round: damage lands, ships sink, forces become alert
//! (decisions 0014 and 0016).

use super::*;

impl World {
    // ---- (d) damage ----

    /// Damage lands, in salvo order: one `ShipDamaged` event per salvo per
    /// track that took hits, then one `SystemHit` event per system hit of the
    /// salvo, in hit order. A ship whose flotation reaches 100 sinks. The
    /// work is [`World::apply_hit_set`], shared with air attacks.
    pub(super) fn apply_damage(&mut self, fight: &mut Fight, round: &mut Round) {
        let site = DamageSite {
            day: fight.day,
            pulse: fight.pulse,
            round: round.number,
        };
        for salvo_index in 0..round.salvos.len() {
            let (target_force, hits) = {
                let salvo = &round.salvos[salvo_index];
                (
                    salvo.target_force,
                    HitSet {
                        cause: salvo.event,
                        target: salvo.target,
                        target_force: fight.forces[salvo.target_force].id,
                        damage_per_hit: salvo.damage_per_hit,
                        track_hits: salvo.track_hits,
                        system_hits: salvo.system_hits.clone(),
                    },
                )
            };
            let applied = self.apply_hit_set(site, &hits);
            round.damaged[target_force].extend(applied.damaged);
            for (ship, sunk) in applied.sunk {
                fight.forces[target_force].sunk_in_fight.insert(ship);
                round.sunk.push(sunk);
            }
        }
    }

    /// A force that sighted the enemy this round, or was fired on this
    /// round, is at normal readiness and is not already alerted becomes
    /// alerted (decision 4 of `docs/decisions/0021-readiness-and-alarm.md`).
    /// A force at battle readiness is alert by its readiness and changes
    /// nothing.
    pub(super) fn become_alert(&mut self, fight: &Fight, round: &Round) {
        for (index, force) in fight.forces.iter().enumerate() {
            let this_round = &round.forces[index];
            if !(this_round.newly_sighted || this_round.fired_on) {
                continue;
            }
            let task_force = self
                .truth
                .task_forces
                .get_mut(&force.id)
                .expect("task force exists");
            // A force at battle readiness is alert by its readiness: nothing
            // changes and nothing is recorded. Its flag changes only in
            // `alert_from_belief`, which records it.
            if task_force.alerted || task_force.readiness == Readiness::Battle {
                continue;
            }
            task_force.alerted = true;
            let cause = this_round
                .rolled
                .or(this_round.first_salvo_against)
                .unwrap_or(fight.begun);
            let event = self.record_in_fight(
                fight,
                EventKind::ForceBecameAlert {
                    round: round.number,
                    task_force: force.id,
                },
                vec![Cause::Event(cause)],
                None,
            );
            self.truth
                .task_forces
                .get_mut(&force.id)
                .expect("task force exists")
                .alerted_by = Some(event);
        }
    }
}
