//! Step (e) of a round: sunk forces, break-off and the last round
//! (decisions 0014 and 0022).

use super::*;

/// The test of whether a force breaks off after a round (decision 9 of
/// `docs/decisions/0022-meeting-and-breaking-off.md`), with the counts it
/// was made on.
pub(crate) struct BreakOffTest {
    /// Of the ships it began with, how many are sunk or crippled.
    pub(crate) lost: i64,
    /// The ships it began with.
    pub(crate) began: i64,
    pub(crate) breaks_off: bool,
}

/// A force breaks off when the share of the ships it began with (`began_with`)
/// that are sunk or at or past `crippled_flotation` reaches its break-off
/// level, or when it avoids and was fired on this round. It avoids if
/// `already_avoiding` (it broke off or slipped away earlier in the pulse) or
/// its ordered stance is to avoid. The share is compared without dividing:
/// count x 1 >= break-off x ships. `ships` must hold every ship of
/// `began_with`; only those are read.
pub(crate) fn break_off_test(
    rules: &RulesOfEngagement,
    began_with: &[ShipId],
    ships: &BTreeMap<ShipId, Ship>,
    crippled_flotation: Fixed,
    (already_avoiding, fired_on): (bool, bool),
) -> BreakOffTest {
    let began = i64::try_from(began_with.len()).expect("few ships");
    let lost = i64::try_from(
        began_with
            .iter()
            .filter(|id| {
                let ship = &ships[*id];
                ship.location == ShipLocation::Sunk || ship.flotation_damage >= crippled_flotation
            })
            .count(),
    )
    .expect("few ships");
    let over_level = Fixed::from_int(lost) >= rules.break_off * began;
    let avoids = already_avoiding || rules.stance == Stance::Avoid;
    BreakOffTest {
        lost,
        began,
        breaks_off: over_level || (avoids && fired_on),
    }
}

impl World {
    // ---- (e) ending ----

    /// The ending checks, in order: a force with no ship afloat, break-off,
    /// the last round. Returns how the fight ended, or `None` if it goes on.
    pub(super) fn check_ending(&mut self, fight: &mut Fight, round: &Round) -> Option<Ending> {
        let number = round.number;

        // 1. A force with no ship afloat. A force can only empty by sinking
        // in this round, so a ShipSunk event is there to cite.
        if fight
            .forces
            .iter()
            .any(|force| self.truth.task_forces[&force.id].ships.is_empty())
        {
            let last_sunk = *round.sunk.last().expect("a force emptied by sinking");
            return Some(Ending {
                round: number,
                reason: EngagementEndReason::ForceSunk,
                causes: vec![Cause::Event(last_sunk)],
            });
        }

        // 2. Break-off.
        let mut broke_off = Vec::new();
        for index in 0..2 {
            if let Some(event) = self.break_off(fight, round, index) {
                broke_off.push(Cause::Event(event));
            }
        }
        if !broke_off.is_empty() {
            return Some(Ending {
                round: number,
                reason: EngagementEndReason::BrokeOff,
                causes: broke_off,
            });
        }

        // 3. The last round.
        if number == fight.figures.max_rounds {
            if !fight.forces[0].sighted && !fight.forces[1].sighted {
                let unseen = self.record_in_fight(
                    fight,
                    EventKind::ForcesPassedUnseen {
                        round: number,
                        a: fight.forces[0].id,
                        b: fight.forces[1].id,
                    },
                    vec![Cause::Event(fight.begun)],
                    None,
                );
                return Some(Ending {
                    round: number,
                    reason: EngagementEndReason::PassedUnseen,
                    causes: vec![Cause::Event(unseen)],
                });
            }
            return Some(Ending {
                round: number,
                reason: EngagementEndReason::MaxRounds,
                causes: vec![Cause::Event(fight.begun)],
            });
        }
        None
    }

    /// A force breaks off when the share of the ships it began with that are
    /// sunk or crippled reaches its break-off level, or when it avoids and
    /// was fired on this round. The share is compared without dividing:
    /// count x 1 >= break-off x ships. Returns the `ForceBrokeOff` event.
    fn break_off(&mut self, fight: &mut Fight, round: &Round, index: usize) -> Option<EventId> {
        let force = &fight.forces[index];
        let task_force = force.id;
        let rules = self.truth.task_forces[&force.id].rules_of_engagement;
        let already_avoiding = fight.avoiding.contains(&force.id);
        let fired_on = round.forces[index].fired_on;
        let test = break_off_test(
            &rules,
            &force.began_with,
            &self.truth.ships,
            fight.figures.crippled_flotation,
            (already_avoiding, fired_on),
        );
        self.trace_choice(force.side, || {
            (
                Question::Surface(SurfaceQuestion::BreakOff {
                    task_force,
                    began_with: force.began_with.clone(),
                    fired_on,
                    already_avoiding,
                }),
                Answer::Surface(SurfaceAnswer::BreakOff(test.breaks_off)),
            )
        });
        if !test.breaks_off {
            return None;
        }
        let (lost, began) = (test.lost, test.began);

        let share = Fixed::from_ratio(lost, began);
        let breakdown = self.config.record_breakdowns.then(|| Breakdown {
            factors: vec![
                Factor {
                    name: "sunk_or_crippled",
                    value: Fixed::from_int(lost),
                },
                Factor {
                    name: "began_with",
                    value: Fixed::from_int(began),
                },
                Factor {
                    name: "break_off",
                    value: rules.break_off,
                },
            ],
            result: share,
        });
        // The damage this round is what took it over its level; failing
        // that, the fire it avoided.
        let mut causes: Vec<Cause> = round.damaged[index]
            .iter()
            .map(|event| Cause::Event(*event))
            .collect();
        if causes.is_empty() {
            causes.extend(round.forces[index].first_salvo_against.map(Cause::Event));
        }
        if causes.is_empty() {
            causes.push(Cause::Event(fight.begun));
        }
        causes.extend(force.rules_order.map(Cause::Order));
        let event = self.record_in_fight(
            fight,
            EventKind::ForceBrokeOff {
                round: round.number,
                task_force,
                share,
            },
            causes,
            breakdown,
        );
        // A force that breaks off avoids for the rest of the pulse.
        fight.avoiding.insert(task_force);
        Some(event)
    }
}
