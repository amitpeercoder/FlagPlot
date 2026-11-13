//! Surface combat: opposing task forces in one hex fight in rounds.
//!
//! The rule is `docs/decisions/0014-surface-combat.md`; the random number
//! keys follow `docs/decisions/0013-random-keys-for-pairs.md`. Since
//! decision 0022 the fights are run from the movement step (`movement.rs`),
//! at the moment two opposing forces meet; the surface engagement phase of a
//! pulse is an empty slot. [`World::hold_meeting`] runs one meeting.
//!
//! A fight is a run of rounds. Each round is
//! worked in five steps, each its own function below, and everything in a
//! round is computed on the state at its start:
//!
//! - (a) [`World::sight`]: forces that have not sighted the enemy roll to.
//! - (b) [`World::avoid`]: a force that avoids slips away from an enemy that
//!   has not sighted it, and the fight ends at once.
//! - (c) [`World::fire`]: every ship of a force that has sighted fires.
//! - (d) [`World::apply_damage`] and [`World::become_alert`]: damage lands,
//!   ships sink, forces become alert.
//! - (e) [`World::check_ending`]: sunk forces, break-off, last round.
//!
//! After the fight each force that sighted the enemy reports it, and each
//! force that fired claims ships sunk. Both go into belief through
//! [`BeliefState::apply_report`](crate::belief::BeliefState::apply_report)
//! and nothing else.
//!
//! The fight writes truth only in ship damage, named system states (decision
//! 0016), ammunition, torpedoes, sinking (the ship's location and its task
//! force's set) and task force alertness.
//! The per-fight state lives in [`Fight`] and is gone when the fight ends.

use std::collections::{BTreeMap, BTreeSet};

use crate::belief::{ApplyOutcome, BattleClaim, Report};
use crate::damage::{
    damage_per_hit, raw_u64, roll_shots, system_for_draw, system_hit_chance, DamageSite, HitSet,
};
use crate::events::{
    Breakdown, Cause, DamageTrack, EngagementEndReason, EventKind, Factor, ForceFatigueChange,
    Weapon,
};
use crate::fixed::Fixed;
use crate::hex::Hex;
use crate::ids::{ContactId, EntityRef, EventId, OrderId, ShipId, TaskForceId};
use crate::orders::{Readiness, Stance};
use crate::refdata::{experience_factor, ShipClass, ShipKind, SurfaceCombatFigures};
use crate::rng::{Draw, RngStream, Subsystem};
use crate::search::build_sighting_report;
use crate::state::{Ship, ShipLocation, ShipSystem, Side};
use crate::time::{self, Pulse};
use crate::world::World;

/// One force's side of a fight.
struct ForceFight {
    id: TaskForceId,
    side: Side,
    /// Has sighted the enemy: by a roll, or by being fired on.
    sighted: bool,
    /// The round and event of the successful sighting roll, if it sighted
    /// by rolling.
    sight_roll: Option<(u32, EventId)>,
    fired_on_last_round: bool,
    /// The ships afloat when the fight began, in ID order.
    began_with: Vec<ShipId>,
    fired_this_fight: bool,
    /// Ready for action when the fight began, decided once and kept for the
    /// whole fight (decision 0068, 2): at battle readiness by order, or alert
    /// since before the pulse began. A force that is not ready fires its guns
    /// at `unready_gun_share` and its torpedoes at `unready_torpedo_share`
    /// (decision 0071).
    ready: bool,
    /// Ships of this force that sank in this fight.
    sunk_in_fight: BTreeSet<ShipId>,
    /// One `chance()` draw per round in which the force rolls.
    sighting_stream: RngStream,
    rules_order: Option<OrderId>,
}

/// Everything that lasts for one fight.
struct Fight {
    day: u32,
    pulse: Pulse,
    hex: Hex,
    dark: bool,
    figures: SurfaceCombatFigures,
    /// The `EngagementBegun` event, cause of most of what follows.
    begun: EventId,
    /// The two forces, the lower task force ID first.
    forces: [ForceFight; 2],
    /// Each ship's fire stream, opened when it first fires and kept for the
    /// whole fight.
    fire_streams: BTreeMap<ShipId, RngStream>,
    /// At least one salvo with shots above zero was fired in this fight
    /// (decision 5 of `docs/decisions/0021-readiness-and-alarm.md`: the
    /// alarm sounds only for a fight with fire).
    fired: bool,
    /// The forces that broke off or slipped away earlier in this pulse,
    /// copied in from the step and written back when the fight ends.
    avoiding: BTreeSet<TaskForceId>,
}

impl Fight {
    /// The index of the other force.
    const fn enemy(index: usize) -> usize {
        1 - index
    }

    /// What multiplies the gun shots of the force at `index`: the figure
    /// `unready_gun_share` if it was not ready when the fight began, else 1
    /// (decision 0068).
    fn gun_share(&self, index: usize) -> Fixed {
        if self.forces[index].ready {
            Fixed::ONE
        } else {
            self.figures.unready_gun_share
        }
    }

    /// What multiplies the torpedo shots of the force at `index`: the figure
    /// `unready_torpedo_share` if it was not ready when the fight began, else 1
    /// (decision 0071).
    fn torpedo_share(&self, index: usize) -> Fixed {
        if self.forces[index].ready {
            Fixed::ONE
        } else {
            self.figures.unready_torpedo_share
        }
    }
}

/// What one force did and had done to it in one round.
#[derive(Default)]
struct RoundForce {
    /// The event of its successful sighting roll this round.
    rolled: Option<EventId>,
    /// It sighted the enemy this round, by rolling or by having been fired on.
    newly_sighted: bool,
    surprised: bool,
    fired_on: bool,
    /// The first salvo fired at it this round.
    first_salvo_against: Option<EventId>,
}

/// One salvo, kept until damage is applied at the end of the round.
struct SalvoRecord {
    event: EventId,
    target: ShipId,
    target_force: usize,
    damage_per_hit: Fixed,
    /// Hits on flotation and fire, in [`DamageTrack::ALL`] order.
    track_hits: [u32; 2],
    /// The systems hit, in hit order. One entry per system hit.
    system_hits: Vec<ShipSystem>,
}

/// One round's working state.
struct Round {
    number: u32,
    forces: [RoundForce; 2],
    /// Salvos in the order they were fired.
    salvos: Vec<SalvoRecord>,
    /// `ShipDamaged` events this round, by the force of the ship hit.
    damaged: [Vec<EventId>; 2],
    /// `ShipSunk` events this round, in order.
    sunk: Vec<EventId>,
}

impl Round {
    fn new(number: u32) -> Round {
        Round {
            number,
            forces: [RoundForce::default(), RoundForce::default()],
            salvos: Vec::new(),
            damaged: [Vec::new(), Vec::new()],
            sunk: Vec::new(),
        }
    }
}

/// How and when a fight ended.
struct Ending {
    round: u32,
    reason: EngagementEndReason,
    causes: Vec<Cause>,
}

/// One weapon fired by one ship in one round, before it is rolled.
struct Shot {
    firer: ShipId,
    target: ShipId,
    target_force: usize,
    weapon: Weapon,
    shots: Fixed,
    base_chance: Fixed,
    /// The factors behind `shots`: guns or tubes, rate, minutes and the
    /// firer's main battery or torpedo mount factor.
    shot_factors: [Fixed; 4],
    /// The share of its guns or torpedoes a force not ready fires (decisions
    /// 0068 and 0071): `Some` only for a salvo of a force that was not ready
    /// when the fight began, with the share below 1. For a gun salvo it is
    /// multiplied into `shots` after the main battery factor and before the
    /// ammunition cap and named in the breakdown (`unready_gun_share`) after
    /// `main_battery_factor`; for a torpedo salvo it is multiplied in after the
    /// torpedo mount factor and named `unready_torpedo_share` after
    /// `torpedo_mount_factor`.
    unready_share: Option<Fixed>,
    /// The firer's fire control factor. One for torpedoes.
    fire_control_factor: Fixed,
    /// The firer's experience factor for this weapon and `Some` only in a
    /// dark fight with the weapon's strength above zero (decision 0064).
    /// Multiplied in last.
    experience_factor: Option<Fixed>,
    damage_per_hit: Fixed,
}

/// The gun shots a ship would fire in a round of `minutes`: guns x rate x
/// minutes x the main battery factor x `share` (the share of its guns a force
/// not ready fires, 1 for a force that is ready: decision 0068), then capped
/// by the ammunition aboard (decision 6 of
/// `docs/decisions/0016-systems-and-stragglers.md`).
fn gun_shots(
    ship: &Ship,
    class: &ShipClass,
    minutes: u32,
    figures: &SurfaceCombatFigures,
    share: Fixed,
) -> Fixed {
    // Guns x minutes is a whole number; the exact `Fixed * i64` multiplies it
    // into the rate before anything is divided.
    let full = class.gun_rounds_per_min * (i64::from(class.main_guns) * i64::from(minutes));
    let factor = figures.system_factor(ShipSystem::MainBattery, ship.systems.main_battery);
    (full * factor * share).min(ship.gun_rounds)
}

/// The tubes of a ship's next torpedo salvo (decision 0083): the tubes that
/// bear on one side, B, for the first salvo, and the rest, T - B, for the
/// second. For a class with B equal to T there is one salvo, of T.
fn torpedo_salvo_tubes(ship: &Ship, class: &ShipClass) -> u32 {
    if ship.torpedoes_one_side_fired {
        class.torpedo_tubes - class.torpedo_tubes_one_side
    } else {
        class.torpedo_tubes_one_side
    }
}

/// The torpedo shots a ship fires in its next firing round: the tubes of the
/// salvo (see [`torpedo_salvo_tubes`]) x the torpedo mount factor x `share`
/// (the share of its torpedoes a force not ready fires, 1 for a force that is
/// ready: decision 0071). Zero if it has no torpedoes left to fire.
fn torpedo_shots(
    ship: &Ship,
    class: &ShipClass,
    figures: &SurfaceCombatFigures,
    share: Fixed,
) -> Fixed {
    if !(ship.torpedoes_ready && class.torpedo_tubes > 0) {
        return Fixed::ZERO;
    }
    let factor = figures.system_factor(ShipSystem::TorpedoMounts, ship.systems.torpedo_mounts);
    Fixed::from_int(i64::from(torpedo_salvo_tubes(ship, class))) * factor * share
}

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
    fn force_is_ready(&self, id: TaskForceId, day: u32, pulse: Pulse) -> bool {
        let force = &self.truth.task_forces[&id];
        if force.readiness == Readiness::Battle {
            return true;
        }
        if !force.alerted {
            return false;
        }
        force
            .alerted_by
            .and_then(|event| self.events.get(event))
            .is_none_or(|event| {
                event.day < day || (event.day == day && event.pulse.is_none_or(|p| p < pulse))
            })
    }

    /// The stance a force takes in a fight: avoid if it broke off or slipped
    /// away earlier in the pulse, whatever its orders say (decision 9 of
    /// `docs/decisions/0022-meeting-and-breaking-off.md`), else its ordered
    /// stance.
    fn effective_stance(&self, fight: &Fight, force: TaskForceId) -> Stance {
        if fight.avoiding.contains(&force) {
            Stance::Avoid
        } else {
            self.truth.task_forces[&force].rules_of_engagement.stance
        }
    }

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
    fn sound_alarm(
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
        let Some(figures) = self.ref_data.surface_combat.clone() else {
            return;
        };
        // Without search figures no side can hold a contact, so the age
        // limit is never read.
        let fresh_max_age = self
            .ref_data
            .search
            .as_ref()
            .map_or(0, |search| search.fresh_contact_max_age_pulses);
        let decisions: Vec<(TaskForceId, Option<(ContactId, EventId)>)> = self
            .truth
            .task_forces
            .values()
            .filter(|force| !force.ships.is_empty() && force.readiness != Readiness::Battle)
            .map(|force| {
                let position = force.position();
                // Contacts come in ID order and min_by_key keeps the first
                // minimum, so a tie goes to the lowest ID.
                let nearest = self.beliefs[&force.side]
                    .contacts()
                    .filter(|contact| {
                        contact.age_pulses <= fresh_max_age
                            && contact.estimated_hex.distance(position)
                                <= figures.alert_contact_distance_hexes
                    })
                    .min_by_key(|contact| contact.estimated_hex.distance(position))
                    .map(|contact| (contact.id, contact.last_report_event));
                (force.id, nearest)
            })
            .collect();
        for (id, nearest) in decisions {
            let (alerted, alerted_by) = {
                let force = &self.truth.task_forces[&id];
                (force.alerted, force.alerted_by)
            };
            match (alerted, nearest) {
                (false, Some((contact, report))) => {
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
                (true, None) => {
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
                (true, Some(_)) | (false, None) => {}
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
                ready: world.force_is_ready(id, day, pulse),
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

    // ---- (a) sighting ----

    /// Each force that has not sighted the enemy rolls to, lower ID first. A
    /// force that was fired on last round has sighted, with no roll and no
    /// draw.
    fn sight(&mut self, fight: &mut Fight, round: &mut Round) {
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
    fn mean_experience(&self, force: TaskForceId) -> Fixed {
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
    fn avoid(&mut self, fight: &mut Fight, round: &Round) -> Option<EventId> {
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

    // ---- (c) fire ----

    /// Every ship afloat at the start of the round, of every force that has
    /// sighted, fires. Salvos are rolled now and kept in `round`; damage
    /// waits for [`World::apply_damage`].
    fn fire(&mut self, fight: &mut Fight, round: &mut Round) {
        let afloat: [Vec<ShipId>; 2] = [
            self.ships_afloat(fight.forces[0].id),
            self.ships_afloat(fight.forces[1].id),
        ];

        // A force is surprised if it has not sighted the enemy and the enemy
        // has and fires at least one salvo.
        for index in 0..2 {
            let enemy = Fight::enemy(index);
            let surprised = !fight.forces[index].sighted
                && fight.forces[enemy].sighted
                && afloat[enemy]
                    .iter()
                    .any(|ship| self.has_any_salvo(fight, enemy, *ship));
            round.forces[index].surprised = surprised;
            if surprised {
                let cause = fight.forces[enemy]
                    .sight_roll
                    .map_or(fight.begun, |(_, event)| event);
                self.record_in_fight(
                    fight,
                    EventKind::ForceSurprised {
                        round: round.number,
                        task_force: fight.forces[index].id,
                        enemy: fight.forces[enemy].id,
                    },
                    vec![Cause::Event(cause)],
                    None,
                );
            }
        }

        // Each ship chooses a gun target and a torpedo target among the enemy
        // ships afloat at the start of the round (decision 0078).
        for index in 0..2 {
            if !fight.forces[index].sighted {
                continue;
            }
            let enemy = Fight::enemy(index);
            for (place, firer) in afloat[index].iter().enumerate() {
                let (gun_target, torpedo_target) =
                    self.choose_targets(&afloat[index], &afloat[enemy], place);
                self.fire_ship(fight, round, index, *firer, gun_target, torpedo_target);
            }
        }
    }

    /// The gun target and the torpedo target of the ship in place `place` of
    /// `own`, the ships afloat in its force, among `enemy`, the enemy ships
    /// afloat, both in ship ID order (decision 0078).
    ///
    /// Guns go like at like: at the enemy ships of the firer's own kind; if
    /// none is afloat, at those of the nearest more important kind afloat; if
    /// none of those, at those of the most important kind afloat. Among them
    /// the ship in place (p modulo their number), p being the firer's place
    /// among the ships of its own kind in `own`. Torpedoes go at the enemy
    /// ships of the most important kind afloat, the one in place (i modulo
    /// their number), i being the firer's place in `own`. The order of kinds
    /// is that of `ShipKind`, where the most important is the least.
    fn choose_targets(&self, own: &[ShipId], enemy: &[ShipId], place: usize) -> (ShipId, ShipId) {
        let kind_of = |ship: &ShipId| self.class_of(*ship).kind;
        let firer_kind = kind_of(&own[place]);
        let enemy_kinds: BTreeSet<ShipKind> = enemy.iter().map(kind_of).collect();
        let most_important = *enemy_kinds
            .iter()
            .next()
            .expect("an enemy ship is afloat when a force fires");

        let gun_kind = if enemy_kinds.contains(&firer_kind) {
            firer_kind
        } else {
            enemy_kinds
                .range(..firer_kind)
                .next_back()
                .copied()
                .unwrap_or(most_important)
        };
        let among = |kind: ShipKind| -> Vec<ShipId> {
            enemy
                .iter()
                .copied()
                .filter(|ship| kind_of(ship) == kind)
                .collect()
        };
        let guns_at = among(gun_kind);
        let place_among_own_kind = own[..place]
            .iter()
            .filter(|ship| kind_of(ship) == firer_kind)
            .count();
        let gun_target = guns_at[place_among_own_kind % guns_at.len()];

        let torpedoes_at = among(most_important);
        let torpedo_target = torpedoes_at[place % torpedoes_at.len()];
        (gun_target, torpedo_target)
    }

    fn ships_afloat(&self, force: TaskForceId) -> Vec<ShipId> {
        self.truth.task_forces[&force]
            .ships
            .iter()
            .copied()
            .collect()
    }

    fn class_of(&self, ship: ShipId) -> &ShipClass {
        &self.ref_data.ship_classes[&self.truth.ships[&ship].class]
    }

    /// True if the ship has a gun or a torpedo salvo to fire this round.
    fn has_any_salvo(&self, fight: &Fight, force: usize, ship: ShipId) -> bool {
        let state = &self.truth.ships[&ship];
        let class = self.class_of(ship);
        let share = fight.gun_share(force);
        let torpedo_share = fight.torpedo_share(force);
        gun_shots(
            state,
            class,
            fight.figures.round_minutes,
            &fight.figures,
            share,
        ) > Fixed::ZERO
            || torpedo_shots(state, class, &fight.figures, torpedo_share) > Fixed::ZERO
    }

    /// One ship's fire for the round: a gun salvo at `gun_target` if it has
    /// shots, then a torpedo salvo at `torpedo_target` if its torpedoes are
    /// ready and its mounts give shots. The two targets may be different
    /// ships (decision 0078). Torpedoes are gone after this round whatever
    /// the mounts gave.
    fn fire_ship(
        &mut self,
        fight: &mut Fight,
        round: &mut Round,
        force: usize,
        firer: ShipId,
        gun_target: ShipId,
        torpedo_target: ShipId,
    ) {
        let enemy = Fight::enemy(force);
        let ship = &self.truth.ships[&firer];
        let class = self.class_of(firer);
        let gun_target_class = self.class_of(gun_target);
        let torpedo_target_class = self.class_of(torpedo_target);
        let figures = &fight.figures;
        let share = fight.gun_share(force);
        let shots = gun_shots(ship, class, figures.round_minutes, figures, share);
        let torpedo_share = fight.torpedo_share(force);
        let torpedo_shot_count = torpedo_shots(ship, class, figures, torpedo_share);
        let torpedoes_fired = ship.torpedoes_ready && class.torpedo_tubes > 0;
        let second_side = ship.torpedoes_one_side_fired;
        let class_has_second_side = class.torpedo_tubes > class.torpedo_tubes_one_side;
        let fire_control =
            figures.system_factor(ShipSystem::FireControl, ship.systems.fire_control);
        // The firer's own experience, in a dark fight, when the weapon's
        // strength is above zero (decision 0064).
        let experience_for = |strength: Fixed| {
            (fight.dark && strength > Fixed::ZERO)
                .then(|| experience_factor(strength, Fixed::from_int(i64::from(ship.experience))))
        };
        let gun = (shots > Fixed::ZERO).then(|| Shot {
            firer,
            target: gun_target,
            target_force: enemy,
            weapon: Weapon::Gun,
            shots,
            base_chance: if fight.dark {
                figures.gun_hit_chance_night
            } else {
                figures.gun_hit_chance_day
            },
            shot_factors: [
                Fixed::from_int(i64::from(class.main_guns)),
                class.gun_rounds_per_min,
                Fixed::from_int(i64::from(figures.round_minutes)),
                figures.system_factor(ShipSystem::MainBattery, ship.systems.main_battery),
            ],
            // Named in the breakdown only for a force that is not ready and
            // only when the share is below 1, so that on the frozen figures
            // no breakdown changes.
            unready_share: (share < Fixed::ONE).then_some(share),
            fire_control_factor: fire_control,
            experience_factor: experience_for(figures.experience_gun_strength),
            damage_per_hit: damage_per_hit(class.gun_damage, gun_target_class.toughness),
        });
        let torpedo = (torpedo_shot_count > Fixed::ZERO).then(|| Shot {
            firer,
            target: torpedo_target,
            target_force: enemy,
            weapon: Weapon::Torpedo,
            shots: torpedo_shot_count,
            base_chance: if fight.dark {
                figures.torpedo_hit_chance_night
            } else {
                figures.torpedo_hit_chance_day
            },
            shot_factors: [
                Fixed::from_int(i64::from(torpedo_salvo_tubes(ship, class))),
                Fixed::ONE,
                Fixed::ONE,
                figures.system_factor(ShipSystem::TorpedoMounts, ship.systems.torpedo_mounts),
            ],
            // As for the guns: named only for a force that is not ready and
            // only when the share is below 1 (decision 0071).
            unready_share: (torpedo_share < Fixed::ONE).then_some(torpedo_share),
            fire_control_factor: Fixed::ONE,
            experience_factor: experience_for(figures.experience_torpedo_strength),
            damage_per_hit: damage_per_hit(class.torpedo_damage, torpedo_target_class.toughness),
        });

        // The stream is opened the first time the ship fires and kept for
        // the whole fight.
        let mut stream = fight.fire_streams.remove(&firer).unwrap_or_else(|| {
            self.rng_pair(
                fight.day,
                Some(fight.pulse),
                Subsystem::SurfaceCombat,
                Draw::Legacy,
                EntityRef::Ship(firer),
                EntityRef::TaskForce(fight.forces[enemy].id),
            )
        });
        if let Some(gun) = gun {
            let ship = self.truth.ships.get_mut(&firer).expect("firer exists");
            ship.gun_rounds -= gun.shots;
            self.fire_weapon(fight, round, force, &mut stream, &gun);
        }
        // Torpedoes are used up only when a salvo is fired. With the mounts
        // out none are fired and none are used, so no state changes without
        // an event (decision 0016, 6).
        //
        // A class with a second side keeps its torpedoes ready after the
        // first side and fires the rest in the next round it fires
        // (decision 0083).
        if torpedoes_fired && torpedo.is_some() {
            let ship = self.truth.ships.get_mut(&firer).expect("firer exists");
            if !second_side && class_has_second_side {
                ship.torpedoes_one_side_fired = true;
            } else {
                ship.torpedoes_ready = false;
                ship.torpedoes_one_side_fired = false;
            }
        }
        if let Some(torpedo) = torpedo {
            if second_side {
                // The second side's salvo has a stream of its own, opened for
                // that salvo alone.
                let mut other_side = self.rng_pair(
                    fight.day,
                    Some(fight.pulse),
                    Subsystem::SurfaceCombat,
                    Draw::TorpedoOtherSide,
                    EntityRef::Ship(firer),
                    EntityRef::TaskForce(fight.forces[enemy].id),
                );
                self.fire_weapon(fight, round, force, &mut other_side, &torpedo);
            } else {
                self.fire_weapon(fight, round, force, &mut stream, &torpedo);
            }
        }
        fight.fire_streams.insert(firer, stream);
    }

    /// Rolls one salvo and records it. Draws, in order: one for each shot (and
    /// one more for a part of a shot), then for each hit of a gun its track and then its
    /// system, and for each hit of a torpedo its system (a torpedo hit is
    /// always on flotation, so it has no track draw).
    fn fire_weapon(
        &mut self,
        fight: &mut Fight,
        round: &mut Round,
        force: usize,
        stream: &mut RngStream,
        shot: &Shot,
    ) {
        let figures = &fight.figures;
        let surprised_factor = if round.forces[shot.target_force].surprised {
            figures.surprised_target_factor
        } else {
            Fixed::ONE
        };
        let lit = fight.dark && self.truth.ships[&shot.target].fire_damage > Fixed::ZERO;
        let lit_factor = if lit {
            figures.lit_target_factor
        } else {
            Fixed::ONE
        };
        let chance = shot.base_chance
            * surprised_factor
            * lit_factor
            * shot.fire_control_factor
            * shot.experience_factor.unwrap_or(Fixed::ONE);
        let expected = shot.shots * chance;
        let hits = roll_shots(stream, shot.shots, chance);

        let system_table = match shot.weapon {
            Weapon::Gun => &figures.gun_system_hit,
            Weapon::Torpedo => &figures.torpedo_system_hit,
            Weapon::Bomb => unreachable!("a surface fight fires no bombs"),
        };
        // Decision 0033: the same per-point figure for every weapon.
        let system_chance =
            system_hit_chance(shot.damage_per_hit, figures.system_hit_chance_per_point);
        let mut track_hits = [0u32; 2];
        let mut system_hits = Vec::new();
        for _ in 0..hits {
            let track = if shot.weapon == Weapon::Gun {
                let draw = stream.below(Fixed::SCALE.unsigned_abs());
                if draw < raw_u64(figures.gun_hit_flotation) {
                    DamageTrack::Flotation
                } else {
                    DamageTrack::Fire
                }
            } else {
                DamageTrack::Flotation
            };
            track_hits[track as usize] += 1;
            // One draw for the system, always, even if the chance is 0.
            let draw = stream.below(Fixed::SCALE.unsigned_abs());
            let draw = Fixed::from_raw(i64::try_from(draw).expect("a draw fits in i64"));
            system_hits.extend(system_for_draw(draw, system_chance, system_table));
        }

        let breakdown = self.config.record_breakdowns.then(|| {
            let [count, rate, minutes, system_factor] = shot.shot_factors;
            let factor = |name, value| Factor { name, value };
            let mut factors = match shot.weapon {
                Weapon::Gun => {
                    let mut factors = vec![
                        factor("guns", count),
                        factor("rate", rate),
                        factor("minutes", minutes),
                        factor("main_battery_factor", system_factor),
                    ];
                    if let Some(share) = shot.unready_share {
                        factors.push(factor("unready_gun_share", share));
                    }
                    factors
                }
                Weapon::Torpedo => {
                    let mut factors = vec![
                        factor("tubes", count),
                        factor("rate", rate),
                        factor("minutes", minutes),
                        factor("torpedo_mount_factor", system_factor),
                    ];
                    if let Some(share) = shot.unready_share {
                        factors.push(factor("unready_torpedo_share", share));
                    }
                    factors
                }
                Weapon::Bomb => unreachable!("a surface fight fires no bombs"),
            };
            factors.push(factor("base_chance", shot.base_chance));
            factors.push(factor("surprised_factor", surprised_factor));
            factors.push(factor("lit_factor", lit_factor));
            if shot.weapon == Weapon::Gun {
                factors.push(factor("fire_control_factor", shot.fire_control_factor));
            }
            if let Some(value) = shot.experience_factor {
                factors.push(factor("experience_factor", value));
            }
            Breakdown {
                factors,
                result: expected,
            }
        });
        let event = self.record_in_fight(
            fight,
            EventKind::Salvo {
                round: round.number,
                firer: shot.firer,
                target: shot.target,
                weapon: shot.weapon,
                shots: shot.shots,
                chance,
                expected,
                hits,
            },
            vec![Cause::Event(fight.begun)],
            breakdown,
        );

        fight.forces[force].fired_this_fight = true;
        // A `Shot` exists only when its shots are above zero.
        fight.fired = true;
        let against = &mut round.forces[shot.target_force];
        against.fired_on = true;
        against.first_salvo_against.get_or_insert(event);
        round.salvos.push(SalvoRecord {
            event,
            target: shot.target,
            target_force: shot.target_force,
            damage_per_hit: shot.damage_per_hit,
            track_hits,
            system_hits,
        });
    }

    // ---- (d) damage ----

    /// Damage lands, in salvo order: one `ShipDamaged` event per salvo per
    /// track that took hits, then one `SystemHit` event per system hit of the
    /// salvo, in hit order. A ship whose flotation reaches 100 sinks. The
    /// work is [`World::apply_hit_set`], shared with air attacks.
    fn apply_damage(&mut self, fight: &mut Fight, round: &mut Round) {
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
    fn become_alert(&mut self, fight: &Fight, round: &Round) {
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

    // ---- (e) ending ----

    /// The ending checks, in order: a force with no ship afloat, break-off,
    /// the last round. Returns how the fight ended, or `None` if it goes on.
    fn check_ending(&mut self, fight: &mut Fight, round: &Round) -> Option<Ending> {
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
        let began = i64::try_from(force.began_with.len()).expect("few ships");
        let lost = i64::try_from(
            force
                .began_with
                .iter()
                .filter(|id| {
                    let ship = &self.truth.ships[*id];
                    ship.location == ShipLocation::Sunk
                        || ship.flotation_damage >= fight.figures.crippled_flotation
                })
                .count(),
        )
        .expect("few ships");

        let over_level = Fixed::from_int(lost) >= rules.break_off * began;
        let avoiding_under_fire =
            self.effective_stance(fight, force.id) == Stance::Avoid && round.forces[index].fired_on;
        if !(over_level || avoiding_under_fire) {
            return None;
        }

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

    // ---- after the fight ----

    /// Records the end of the fight, then each force reports and claims, the
    /// lower ID first.
    fn end_fight(&mut self, fight: &Fight, ending: &Ending) -> EventId {
        let ended = self.record_in_fight(
            fight,
            EventKind::EngagementEnded {
                round: ending.round,
                a: fight.forces[0].id,
                b: fight.forces[1].id,
                hex: fight.hex,
                reason: ending.reason,
            },
            ending.causes.clone(),
            None,
        );
        for index in 0..2 {
            self.report_sighting(fight, index, ended);
            self.claim_victory(fight, index, ended);
        }
        ended
    }

    /// A force that sighted the enemy reports the enemy force: at the fight
    /// hex, with the ships it left afloat (or began with, if none are left),
    /// through the same error model as an air search report.
    ///
    /// A force fired on in the last round has seen the enemy's gun flashes,
    /// so it reports too, even though the fight ended before the round in
    /// which it would have counted as having sighted (decision 0014, 4).
    fn report_sighting(&mut self, fight: &Fight, index: usize, ended: EventId) {
        let force = &fight.forces[index];
        if !(force.sighted || force.fired_on_last_round) {
            return;
        }
        let enemy = &fight.forces[Fight::enemy(index)];
        let afloat = self.ships_afloat(enemy.id);
        let seen = if afloat.is_empty() {
            &enemy.began_with
        } else {
            &afloat
        };
        // The smallest kind is the most important one (decision 0012,
        // decision 9).
        let kind: ShipKind = seen
            .iter()
            .map(|ship| self.class_of(*ship).kind)
            .min()
            .expect("a force that fought had ships");
        let ship_count = u32::try_from(seen.len()).expect("few ships");

        let figures = self
            .ref_data
            .search
            .clone()
            .expect("World::new requires search figures with surface combat figures");
        let mut stream = self.rng_pair(
            fight.day,
            Some(fight.pulse),
            Subsystem::Intelligence,
            Draw::Legacy,
            EntityRef::TaskForce(force.id),
            EntityRef::TaskForce(enemy.id),
        );
        let report = build_sighting_report(
            &mut stream,
            &self.map,
            fight.hex,
            ship_count,
            kind,
            &figures,
            self.events.next_id(),
        );
        let ApplyOutcome::Contact(outcome) = self
            .beliefs
            .get_mut(&force.side)
            .expect("both sides have a belief state")
            .apply_report(&Report::Sighting(report.clone()), &figures)
        else {
            unreachable!("a sighting report makes or refreshes a contact");
        };
        let received = self.record_in_fight(
            fight,
            EventKind::ReportReceived {
                side: force.side,
                contact: outcome.contact,
                hex: report.estimated_hex,
                ship_count: report.estimated_ship_count,
                kind: report.estimated_kind,
                created: outcome.created,
            },
            vec![Cause::Event(ended)],
            None,
        );
        assert_eq!(
            received, report.received_event,
            "the report cites the event that records it"
        );
    }

    /// A force that fired claims as sunk every enemy ship that sank in the
    /// fight and every enemy ship afloat with fire damage at the claim level.
    fn claim_victory(&mut self, fight: &Fight, index: usize, ended: EventId) {
        let force = &fight.forces[index];
        if !force.fired_this_fight {
            return;
        }
        let enemy = &fight.forces[Fight::enemy(index)];
        let burning = self.truth.task_forces[&enemy.id]
            .ships
            .iter()
            .filter(|ship| self.truth.ships[*ship].fire_damage >= fight.figures.claimed_sunk_fire)
            .count();
        let claimed_sunk = u32::try_from(enemy.sunk_in_fight.len() + burning).expect("few ships");

        let figures = self
            .ref_data
            .search
            .clone()
            .expect("World::new requires search figures with surface combat figures");
        let claim = BattleClaim {
            hex: fight.hex,
            day: fight.day,
            pulse: fight.pulse,
            claimed_sunk,
            claimed_hits: 0,
            claimed_kind: None,
            claimed_shot_down: 0,
            claimed_on_ground: 0,
            from_air: false,
            received_event: self.events.next_id(),
        };
        let outcome = self
            .beliefs
            .get_mut(&force.side)
            .expect("both sides have a belief state")
            .apply_report(&Report::BattleClaim(claim.clone()), &figures);
        assert_eq!(outcome, ApplyOutcome::Claim, "a claim is stored as a claim");
        let received = self.record_in_fight(
            fight,
            EventKind::ClaimReceived {
                side: force.side,
                hex: fight.hex,
                claimed_sunk,
                claimed_hits: 0,
                claimed_kind: None,
                claimed_shot_down: 0,
                claimed_on_ground: 0,
                from_air: false,
            },
            vec![Cause::Event(ended)],
            None,
        );
        assert_eq!(
            received, claim.received_event,
            "the claim cites the event that records it"
        );
    }

    fn record_in_fight(
        &mut self,
        fight: &Fight,
        kind: EventKind,
        causes: Vec<Cause>,
        breakdown: Option<Breakdown>,
    ) -> EventId {
        self.events
            .record(fight.day, Some(fight.pulse), kind, causes, breakdown)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::events::Event;
    use crate::ids::ShipClassId;
    use crate::invariants;
    use crate::map::{HexData, Map};
    use crate::orders::{Mission, Readiness};
    use crate::refdata::{PortSize, RefData, SearchFigures};
    use crate::resolve::PulsePhase;
    use crate::state::{RulesOfEngagement, ShipSystems, SystemState, TaskForce, TruthState};
    use crate::world::SimConfig;

    // ---- a fixture to build fights from ----
    //
    // A `Scene` is a list of task forces on a small sea, a seed and the pulse
    // the fight happens in. `Scene::world` builds the world, `Scene::fight`
    // builds it and runs the surface engagement step once, on day 1.
    //
    // Classes are the placeholder classes of `data/reference/ship_classes.csv`:
    // 1 is a cruiser (8 guns, 3 rounds a minute, 100 rounds a gun, gun damage
    // 20, 8 tubes, torpedo damage 200, toughness 400), 2 a destroyer (4 guns,
    // 6 rounds a minute, 150 rounds a gun, gun damage 5, 8 tubes, torpedo
    // damage 200, toughness 100), 3 an unarmed transport (toughness 300) and 4
    // a copy of the cruiser with radar.

    /// One task force to place: ID, side, hex, the class of each ship (ship
    /// IDs are handed out in order across the forces, from 1), whether it is
    /// readiness (the old "alert" is battle, "not alert" is normal), and its
    /// rules of engagement.
    #[derive(Clone)]
    struct ForceSpec {
        id: u32,
        side: Side,
        hex: Hex,
        classes: Vec<u32>,
        readiness: Readiness,
        rules: RulesOfEngagement,
    }

    impl ForceSpec {
        fn new(id: u32, side: Side, hex: Hex, classes: &[u32]) -> ForceSpec {
            ForceSpec {
                id,
                side,
                hex,
                classes: classes.to_vec(),
                readiness: Readiness::Battle,
                rules: RulesOfEngagement::DEFAULT,
            }
        }

        fn not_alert(mut self) -> ForceSpec {
            self.readiness = Readiness::Normal;
            self
        }

        fn stance(mut self, stance: Stance) -> ForceSpec {
            self.rules.stance = stance;
            self
        }

        fn break_off(mut self, break_off: Fixed) -> ForceSpec {
            self.rules.break_off = break_off;
            self
        }
    }

    #[derive(Clone)]
    struct Scene {
        seed: u64,
        pulse: Pulse,
        forces: Vec<ForceSpec>,
    }

    impl Scene {
        fn new(seed: u64, pulse: Pulse, forces: Vec<ForceSpec>) -> Scene {
            Scene {
                seed,
                pulse,
                forces,
            }
        }

        fn world(&self) -> World {
            let mut map = Map::new();
            for q in -5..=5 {
                for r in -5..=5 {
                    map.insert(Hex::new(q, r), HexData::open_sea());
                }
            }
            let mut ref_data = RefData::default();
            let classes = [
                (
                    1,
                    "cruiser",
                    ShipKind::Cruiser,
                    [8, 3, 100, 20, 8, 200, 400],
                    false,
                ),
                (
                    2,
                    "destroyer",
                    ShipKind::Destroyer,
                    [4, 6, 150, 5, 8, 200, 100],
                    false,
                ),
                (
                    3,
                    "transport",
                    ShipKind::Transport,
                    [0, 0, 0, 0, 0, 0, 300],
                    false,
                ),
                (
                    4,
                    "radar cruiser",
                    ShipKind::Cruiser,
                    [8, 3, 100, 20, 8, 200, 400],
                    true,
                ),
                (
                    8,
                    "cruiser with wing tubes",
                    ShipKind::Cruiser,
                    [8, 3, 100, 20, 8, 200, 400],
                    false,
                ),
                (
                    9,
                    "destroyer with wing and centreline tubes",
                    ShipKind::Destroyer,
                    [4, 6, 150, 5, 12, 200, 100],
                    false,
                ),
            ];
            for (
                id,
                name,
                kind,
                [guns, rate, rounds, gun_damage, tubes, torpedo_damage, tough],
                radar,
            ) in classes
            {
                ref_data.ship_classes.insert(
                    ShipClassId::new(id),
                    ShipClass {
                        id: ShipClassId::new(id),
                        name: format!("Test {name}"),
                        kind,
                        cruise_speed_kn: Fixed::from_int(16),
                        max_speed_kn: Fixed::from_int(32),
                        fuel_capacity_t: Fixed::from_int(2000),
                        endurance_nm: Fixed::from_int(8000),
                        cargo_capacity_t: Fixed::ZERO,
                        main_guns: guns,
                        gun_rounds_per_min: Fixed::from_int(i64::from(rate)),
                        gun_rounds_per_gun: rounds,
                        gun_damage: Fixed::from_int(i64::from(gun_damage)),
                        torpedo_tubes: tubes,
                        torpedo_tubes_one_side: tubes,
                        torpedo_damage: Fixed::from_int(i64::from(torpedo_damage)),
                        toughness: Fixed::from_int(i64::from(tough)),
                        radar,
                        aircraft_capacity: 0,
                        anti_aircraft: 0,
                    },
                );
            }
            // Class 8 has 4 of its 8 tubes on a side, class 9 (Mahan-like) 8
            // of its 12 (decision 0083). Every other class has B equal to T.
            for (id, one_side) in [(8, 4), (9, 8)] {
                ref_data
                    .ship_classes
                    .get_mut(&ShipClassId::new(id))
                    .expect("class")
                    .torpedo_tubes_one_side = one_side;
            }
            ref_data.port_sizes.insert(
                0,
                PortSize {
                    berths: 0,
                    berth_rate_t_per_h: Fixed::ZERO,
                    lighter_rate_t_per_h: Fixed::from_int(20),
                },
            );
            ref_data.search = Some(SearchFigures::from_search_toml());
            ref_data.surface_combat = Some(SurfaceCombatFigures::from_surface_combat_toml());

            let mut truth = TruthState::default();
            let mut next_ship = 1;
            for spec in &self.forces {
                let id = TaskForceId::new(spec.id);
                let mut force = TaskForce::at(
                    id,
                    format!("Test force {}", spec.id),
                    spec.side,
                    spec.hex,
                    Mission::SurfaceCombat,
                );
                force.readiness = spec.readiness;
                force.rules_of_engagement = spec.rules;
                for class in &spec.classes {
                    let ship_id = ShipId::new(next_ship);
                    next_ship += 1;
                    let class_data = &ref_data.ship_classes[&ShipClassId::new(*class)];
                    force.ships.insert(ship_id);
                    truth.ships.insert(
                        ship_id,
                        Ship {
                            fuel_t: Fixed::from_int(2000),
                            gun_rounds: Fixed::from_int(i64::from(
                                class_data.main_guns * class_data.gun_rounds_per_gun,
                            )),
                            torpedoes_ready: class_data.torpedo_tubes > 0,
                            ..Ship::new(
                                ship_id,
                                format!("Test ship {}", ship_id.raw()),
                                ShipClassId::new(*class),
                                spec.side,
                                ShipLocation::TaskForce(id),
                            )
                        },
                    );
                }
                truth.task_forces.insert(id, force);
            }
            World::new(self.seed, ref_data, map, truth).expect("valid setup")
        }

        fn fight(&self) -> World {
            let mut world = self.world();
            self.run_step(&mut world);
            world
        }

        /// The surface engagement step, then what the end of the pulse does
        /// to belief (contacts age), so that the state can be hashed.
        fn run_step(&self, world: &mut World) {
            world.run_pulse_phase(1, self.pulse, PulsePhase::Movement);
            let figures = world.ref_data.search.clone().expect("search figures");
            for belief in world.beliefs.values_mut() {
                belief.age_contacts(&figures);
            }
        }
    }

    /// Task force 1 (Japanese) and 2 (Allied), three cruisers each, both in
    /// hex (5,0), the Allied one not alert: the shape of scenario 4a.
    fn night_action(seed: u64) -> Scene {
        let hex = Hex::new(5, 0);
        Scene::new(
            seed,
            Pulse::Night,
            vec![
                ForceSpec::new(1, Side::Japanese, hex, &[1, 1, 1]),
                ForceSpec::new(2, Side::Allied, hex, &[1, 1, 1]).not_alert(),
            ],
        )
    }

    fn events_of(world: &World, keep: impl Fn(&EventKind) -> bool) -> Vec<Event> {
        world
            .events()
            .iter()
            .filter(|event| keep(&event.kind))
            .cloned()
            .collect()
    }

    // ---- smoke tests ----

    // Decimal figures. A gun damage of 19.666666 (118 kg / 6) against a
    // toughness of 405.44 (10,136 long tons / 25):
    //   100 x 19.666666 = 1966.6666, then / 405.44.
    //   405.44 x 4.850697 = 1966.66659168, which is not above 1966.6666.
    //   405.44 x 4.850698 = 1966.66699712, which is above it.
    // So the truncated result is 4.850697 points per hit (the exact quotient
    // is 4.8506970205...). Fixed raw values: 4_850_697.
    #[test]
    fn damage_per_hit_with_decimal_figures_multiplies_first_and_truncates() {
        let damage = "19.666666".parse::<Fixed>().expect("decimal");
        let toughness = "405.44".parse::<Fixed>().expect("decimal");
        assert_eq!(damage_per_hit(damage, toughness).raw(), 4_850_697);
        // Whole numbers are unchanged: 100 x 20 / 400 = 5.
        assert_eq!(
            damage_per_hit(Fixed::from_int(20), Fixed::from_int(400)),
            Fixed::from_int(5)
        );
        assert_eq!(
            damage_per_hit(Fixed::from_int(200), Fixed::from_int(400)),
            Fixed::from_ratio(100 * 200, 400)
        );
    }

    #[test]
    fn a_decimal_rate_of_fire_gives_whole_shots_multiplied_before_any_division() {
        // 8 guns x 3.5 rounds a minute x 5 minutes = 140 shots, at a main
        // battery factor of 1 (fully working). The ship holds 800 rounds, so
        // the ammunition cap does not bite.
        let mut world = night_action(1).world();
        world
            .ref_data
            .ship_classes
            .get_mut(&ShipClassId::new(1))
            .expect("class 1")
            .gun_rounds_per_min = "3.5".parse().expect("3.5");
        let figures = world.ref_data.surface_combat.clone().expect("figures");
        assert_eq!(figures.round_minutes, 5);
        let ship = ship(&world, 1);
        let class = &world.ref_data.ship_classes[&ship.class];
        assert_eq!(
            figures.system_factor(ShipSystem::MainBattery, ship.systems.main_battery),
            Fixed::ONE
        );
        assert_eq!(
            gun_shots(ship, class, 5, &figures, Fixed::ONE),
            Fixed::from_int(140)
        );
    }

    #[test]
    fn two_opposing_forces_in_one_hex_at_night_fight_and_the_fight_ends() {
        for seed in 1..=30 {
            let world = night_action(seed).fight();
            let begun = events_of(&world, |k| matches!(k, EventKind::EngagementBegun { .. }));
            let ended = events_of(&world, |k| matches!(k, EventKind::EngagementEnded { .. }));
            assert_eq!(begun.len(), 1, "seed {seed}: one fight begins");
            assert_eq!(ended.len(), 1, "seed {seed}: one fight ends");
            let EventKind::EngagementEnded { round, .. } = ended[0].kind else {
                unreachable!();
            };
            assert!(
                (1..=6).contains(&round),
                "seed {seed}: ended in round {round}"
            );

            // The rules left a state that breaks no invariant.
            let violations = invariants::check(world.umpire(), world.map(), world.ref_data());
            assert!(violations.is_empty(), "seed {seed}: {violations:?}");
        }
    }

    #[test]
    fn a_first_gun_salvo_at_night_is_120_shots_at_the_hand_calculated_chance() {
        // Placeholder cruisers: 8 x 3 x 5 = 120 shots. The chance is 0.03,
        // times 1.5 if the target is surprised, times 2 if it is lit.
        for seed in 1..=30 {
            let world = night_action(seed).fight();
            let first = events_of(&world, |k| {
                matches!(
                    k,
                    EventKind::Salvo {
                        weapon: Weapon::Gun,
                        round: 1,
                        ..
                    }
                )
            });
            for event in first {
                let EventKind::Salvo {
                    shots,
                    chance,
                    expected,
                    ..
                } = event.kind
                else {
                    unreachable!();
                };
                assert_eq!(shots, Fixed::from_int(120));
                assert!(
                    chance == Fixed::from_ratio(3, 100) || chance == Fixed::from_ratio(45, 1000),
                    "seed {seed}: chance {chance}"
                );
                assert_eq!(expected, shots * chance);
            }
        }
    }

    #[test]
    fn forces_of_one_side_do_not_fight() {
        let hex = Hex::new(5, 0);
        let scene = Scene::new(
            1,
            Pulse::Night,
            vec![
                ForceSpec::new(1, Side::Japanese, hex, &[1, 1]),
                ForceSpec::new(2, Side::Japanese, hex, &[1, 1]),
            ],
        );
        let world = scene.fight();
        assert!(world.events().is_empty());
        assert_eq!(world.umpire().ships.len(), 4);
    }

    #[test]
    fn forces_in_different_hexes_do_not_fight() {
        let scene = Scene::new(
            1,
            Pulse::Night,
            vec![
                ForceSpec::new(1, Side::Japanese, Hex::new(5, 0), &[1]),
                ForceSpec::new(2, Side::Allied, Hex::new(4, 0), &[1]),
            ],
        );
        assert!(scene.fight().events().is_empty());
    }

    #[test]
    fn recording_breakdowns_changes_neither_the_outcome_nor_the_hash() {
        for seed in 1..=10 {
            let scene = night_action(seed);
            let mut with = scene.world();
            let mut without = scene.world();
            with.set_config(SimConfig {
                record_breakdowns: true,
            });
            without.set_config(SimConfig {
                record_breakdowns: false,
            });
            scene.run_step(&mut with);
            scene.run_step(&mut without);

            assert_eq!(with.state_hash(), without.state_hash(), "seed {seed}");
            assert_eq!(with.events().len(), without.events().len());
            for (a, b) in with.events().iter().zip(without.events().iter()) {
                assert_eq!(a.kind, b.kind);
                assert_eq!(a.causes, b.causes);
                assert!(b.breakdown.is_none());
            }
            // And the breakdowns are there when asked for.
            assert!(with
                .events()
                .iter()
                .any(|event| matches!(event.kind, EventKind::Salvo { .. })
                    && event.breakdown.is_some()));
        }
    }

    #[test]
    fn a_force_with_no_ships_does_not_fight() {
        let hex = Hex::new(5, 0);
        let scene = Scene::new(
            1,
            Pulse::Night,
            vec![
                ForceSpec::new(1, Side::Japanese, hex, &[1]),
                ForceSpec::new(2, Side::Allied, hex, &[]),
            ],
        );
        assert!(scene.fight().events().is_empty());
    }

    #[test]
    fn an_avoiding_force_that_sights_first_slips_away_with_no_fire() {
        // Find a seed in which the avoiding force alone sights in round 1.
        let hex = Hex::new(5, 0);
        let mut slipped = 0;
        for seed in 1..=60 {
            let scene = Scene::new(
                seed,
                Pulse::Night,
                vec![
                    ForceSpec::new(1, Side::Japanese, hex, &[1, 1, 1]).stance(Stance::Avoid),
                    ForceSpec::new(2, Side::Allied, hex, &[1, 1, 1])
                        .not_alert()
                        .break_off(Fixed::ONE),
                ],
            );
            let world = scene.fight();
            let slips = events_of(&world, |k| matches!(k, EventKind::ForceSlippedAway { .. }));
            if slips.is_empty() {
                continue;
            }
            slipped += 1;
            let fired = events_of(&world, |k| matches!(k, EventKind::Salvo { .. }));
            assert!(fired.is_empty(), "seed {seed}: no fire when a force slips");
        }
        assert!(slipped > 0, "some seed makes the avoider sight first");
    }
    // ---- helpers for the tests below ----

    /// A decimal such as "0.5" as a `Fixed`.
    fn fx(text: &str) -> Fixed {
        text.parse().expect("a decimal number")
    }

    fn night_scene(forces: Vec<ForceSpec>) -> Scene {
        Scene::new(1, Pulse::Night, forces)
    }

    fn day_scene(forces: Vec<ForceSpec>) -> Scene {
        Scene::new(1, Pulse::Morning, forces)
    }

    /// Builds the world, lets the test change it, runs the step once.
    fn run(scene: &Scene, setup: impl FnOnce(&mut World)) -> World {
        let mut world = scene.world();
        setup(&mut world);
        scene.run_step(&mut world);
        world
    }

    fn figures_mut(world: &mut World) -> &mut SurfaceCombatFigures {
        world
            .ref_data
            .surface_combat
            .as_mut()
            .expect("combat figures")
    }

    fn ship_mut(world: &mut World, id: u32) -> &mut Ship {
        world
            .truth
            .ships
            .get_mut(&ShipId::new(id))
            .expect("ship exists")
    }

    fn ship(world: &World, id: u32) -> &Ship {
        &world.truth.ships[&ShipId::new(id)]
    }

    /// Nothing is ever sunk or crippled and no torpedo hits: every gun hit
    /// lands on the fire track (which does nothing by day), no hit knocks
    /// down a system and torpedoes always miss. A fight with these figures
    /// runs to the last round.
    fn harmless(world: &mut World) {
        let figures = figures_mut(world);
        figures.gun_hit_flotation = Fixed::ZERO;
        figures.system_hit_chance_per_point = Fixed::ZERO;
        figures.torpedo_hit_chance_night = Fixed::ZERO;
        figures.torpedo_hit_chance_day = Fixed::ZERO;
    }

    /// Every gun hit lands on flotation and torpedoes always miss, so by
    /// day a salvo does exactly 6 hits x 5 points = 30. Each shot is rolled
    /// (decision 0033), so to keep the 6 hits sure the day chance is 1 and a
    /// salvo is 6 shots: 8 guns x 0.15 rounds a minute x 5 minutes, in every
    /// class.
    fn flotation_only(world: &mut World) {
        for class in world.ref_data.ship_classes.values_mut() {
            class.gun_rounds_per_min = fx("0.15");
        }
        let figures = figures_mut(world);
        figures.gun_hit_chance_day = Fixed::ONE;
        figures.gun_hit_flotation = Fixed::ONE;
        figures.system_hit_chance_per_point = Fixed::ZERO;
        figures.torpedo_hit_chance_night = Fixed::ZERO;
        figures.torpedo_hit_chance_day = Fixed::ZERO;
    }

    /// Force 1 sights for sure, force 2 never does (it is not alert, and a
    /// not alert force has its sighting chance multiplied by zero).
    fn only_force_1_sights_by_roll(world: &mut World) {
        let figures = figures_mut(world);
        figures.sighting_chance_night = Fixed::ONE;
        figures.not_alert_sighting_factor = Fixed::ZERO;
    }

    /// A `Salvo` event, with the entities as raw numbers.
    struct Fired {
        id: EventId,
        round: u32,
        firer: u32,
        target: u32,
        weapon: Weapon,
        shots: Fixed,
        chance: Fixed,
        expected: Fixed,
        hits: u32,
    }

    fn salvos(world: &World) -> Vec<Fired> {
        world
            .events()
            .iter()
            .filter_map(|event| match event.kind {
                EventKind::Salvo {
                    round,
                    firer,
                    target,
                    weapon,
                    shots,
                    chance,
                    expected,
                    hits,
                } => Some(Fired {
                    id: event.id,
                    round,
                    firer: firer.raw(),
                    target: target.raw(),
                    weapon,
                    shots,
                    chance,
                    expected,
                    hits,
                }),
                _ => None,
            })
            .collect()
    }

    /// The gun salvos of one ship.
    fn gun_salvos_of(world: &World, ship: u32) -> Vec<Fired> {
        salvos(world)
            .into_iter()
            .filter(|s| s.firer == ship && s.weapon == Weapon::Gun)
            .collect()
    }

    fn torpedo_salvos_of(world: &World, ship: u32) -> Vec<Fired> {
        salvos(world)
            .into_iter()
            .filter(|s| s.firer == ship && s.weapon == Weapon::Torpedo)
            .collect()
    }

    /// A `SightingRoll` event: (round, task force, chance, sighted).
    fn rolls(world: &World) -> Vec<(u32, u32, Fixed, bool)> {
        world
            .events()
            .iter()
            .filter_map(|event| match event.kind {
                EventKind::SightingRoll {
                    round,
                    task_force,
                    chance,
                    sighted,
                    ..
                } => Some((round, task_force.raw(), chance, sighted)),
                _ => None,
            })
            .collect()
    }

    /// The round 1 sighting chances of forces 1 and 2.
    fn first_round_chances(world: &World) -> [Fixed; 2] {
        let first: Vec<_> = rolls(world)
            .into_iter()
            .filter(|(round, ..)| *round == 1)
            .collect();
        assert_eq!(first.len(), 2, "both forces roll in round 1");
        assert_eq!((first[0].1, first[1].1), (1, 2));
        [first[0].2, first[1].2]
    }

    fn count(world: &World, keep: impl Fn(&EventKind) -> bool) -> usize {
        events_of(world, keep).len()
    }

    /// How each fight ended: (round, reason).
    fn endings(world: &World) -> Vec<(u32, EngagementEndReason)> {
        world
            .events()
            .iter()
            .filter_map(|event| match event.kind {
                EventKind::EngagementEnded { round, reason, .. } => Some((round, reason)),
                _ => None,
            })
            .collect()
    }

    // ---- 1. sighting chances ----

    #[test]
    fn sighting_chance_by_night_with_radar_and_alertness() {
        // chance = min(1, (base + radar bonus) x alert factor), with the
        // placeholder figures: night base 0.5, radar bonus 0.2, not alert
        // factor 0.5.
        let hex = Hex::new(5, 0);
        let chances = |a: ForceSpec, b: ForceSpec| {
            first_round_chances(&run(&night_scene(vec![a, b]), |_| {}))
        };
        let japanese = |classes: &[u32]| ForceSpec::new(1, Side::Japanese, hex, classes);
        let allied = |classes: &[u32]| ForceSpec::new(2, Side::Allied, hex, classes);

        // Alert, no radar: (0.5 + 0) x 1 = 0.5.
        assert_eq!(
            chances(japanese(&[1]), allied(&[1])),
            [fx("0.5"), fx("0.5")]
        );
        // Not alert, no radar: (0.5 + 0) x 0.5 = 0.25.
        assert_eq!(
            chances(japanese(&[1]).not_alert(), allied(&[1])),
            [fx("0.25"), fx("0.5")]
        );
        // Alert with radar: (0.5 + 0.2) x 1 = 0.7. Not alert with radar:
        // (0.5 + 0.2) x 0.5 = 0.35.
        assert_eq!(
            chances(japanese(&[4]), allied(&[4]).not_alert()),
            [fx("0.7"), fx("0.35")]
        );
        // One radar ship among two is enough: (0.5 + 0.2) x 1 = 0.7.
        assert_eq!(
            chances(japanese(&[1, 4]), allied(&[1, 1])),
            [fx("0.7"), fx("0.5")]
        );
    }

    #[test]
    fn sighting_chance_by_day_is_capped_at_one() {
        // Day base 1. Alert: min(1, 1 x 1) = 1. Alert with radar:
        // min(1, 1.2 x 1) = 1 (the cap). Not alert: 1 x 0.5 = 0.5. Not alert
        // with radar: min(1, 1.2 x 0.5) = 0.6.
        let hex = Hex::new(5, 0);
        let chances =
            |a: ForceSpec, b: ForceSpec| first_round_chances(&run(&day_scene(vec![a, b]), |_| {}));
        let japanese = |classes: &[u32]| ForceSpec::new(1, Side::Japanese, hex, classes);
        let allied = |classes: &[u32]| ForceSpec::new(2, Side::Allied, hex, classes);
        assert_eq!(
            chances(japanese(&[1]), allied(&[1])),
            [Fixed::ONE, Fixed::ONE]
        );
        assert_eq!(
            chances(japanese(&[4]), allied(&[4])),
            [Fixed::ONE, Fixed::ONE]
        );
        assert_eq!(
            chances(japanese(&[1]).not_alert(), allied(&[1])),
            [fx("0.5"), Fixed::ONE]
        );
        assert_eq!(
            chances(japanese(&[4]), allied(&[4]).not_alert()),
            [Fixed::ONE, fx("0.6")]
        );
    }

    #[test]
    fn sighting_breakdown_lists_base_radar_bonus_and_alert_factor() {
        // Not alert radar force by night: (0.5 + 0.2) x 0.5 = 0.35.
        let hex = Hex::new(5, 0);
        let scene = night_scene(vec![
            ForceSpec::new(1, Side::Japanese, hex, &[1]),
            ForceSpec::new(2, Side::Allied, hex, &[4]).not_alert(),
        ]);
        let world = run(&scene, |w| {
            w.set_config(SimConfig {
                record_breakdowns: true,
            });
        });
        let roll = world
            .events()
            .iter()
            .find(|e| {
                matches!(
                    e.kind,
                    EventKind::SightingRoll {
                        round: 1,
                        task_force,
                        ..
                    } if task_force == TaskForceId::new(2)
                )
            })
            .expect("force 2 rolled in round 1");
        let breakdown = roll.breakdown.as_ref().expect("breakdown recorded");
        let factors: Vec<(&str, Fixed)> = breakdown
            .factors
            .iter()
            .map(|f| (f.name, f.value))
            .collect();
        assert_eq!(
            factors,
            vec![
                ("base", fx("0.5")),
                ("radar_bonus", fx("0.2")),
                ("alert_factor", fx("0.5")),
            ]
        );
        assert_eq!(breakdown.result, fx("0.35"));
    }

    #[test]
    fn by_day_both_alert_forces_sight_in_round_1_and_nobody_is_surprised() {
        // Day chance 1 for both alert forces: both roll once, both sight.
        let hex = Hex::new(5, 0);
        let scene = day_scene(vec![
            ForceSpec::new(1, Side::Japanese, hex, &[1]),
            ForceSpec::new(2, Side::Allied, hex, &[1]),
        ]);
        let world = run(&scene, harmless);
        assert_eq!(
            rolls(&world),
            vec![(1, 1, Fixed::ONE, true), (1, 2, Fixed::ONE, true)],
            "two rolls, both in round 1, both sighted, and no later rolls"
        );
        assert_eq!(
            count(&world, |k| matches!(k, EventKind::ForceSurprised { .. })),
            0
        );
        // Both fired in round 1.
        let firers: Vec<u32> = salvos(&world)
            .iter()
            .filter(|s| s.round == 1 && s.weapon == Weapon::Gun)
            .map(|s| s.firer)
            .collect();
        assert_eq!(firers, vec![1, 2]);
    }

    // ---- 2. fired on means sighted ----

    #[test]
    fn a_force_that_was_fired_on_sights_next_round_with_no_roll_and_fires() {
        // Night. Force 1 is alert: chance (1 + 0) x 1 = 1. Force 2 is not
        // alert: chance (1 + 0) x 0 = 0, so it never sights by rolling.
        // Round 1: force 1 sights, force 2 does not and is surprised; force 1
        // fires. Round 2: force 2 was fired on, so it has sighted with no
        // roll; force 1 has sighted already, so nobody rolls. Force 2 fires.
        let hex = Hex::new(5, 0);
        let scene = night_scene(vec![
            ForceSpec::new(1, Side::Japanese, hex, &[1]),
            ForceSpec::new(2, Side::Allied, hex, &[1]).not_alert(),
        ]);
        let world = run(&scene, |w| {
            only_force_1_sights_by_roll(w);
            harmless(w);
        });
        assert_eq!(
            rolls(&world),
            vec![(1, 1, Fixed::ONE, true), (1, 2, Fixed::ZERO, false)],
            "no roll after round 1 by either force"
        );
        let shots = salvos(&world);
        assert!(
            shots.iter().all(|s| s.round != 1 || s.firer == 1),
            "force 2 does not fire in round 1"
        );
        assert!(
            shots.iter().any(|s| s.round == 2 && s.firer == 2),
            "force 2 fires in round 2"
        );
        // Surprised in round 1 only.
        let surprised = events_of(&world, |k| matches!(k, EventKind::ForceSurprised { .. }));
        assert_eq!(surprised.len(), 1);
        assert!(matches!(
            surprised[0].kind,
            EventKind::ForceSurprised { round: 1, task_force, .. }
                if task_force == TaskForceId::new(2)
        ));
    }

    // ---- 3. stance Avoid ----

    #[test]
    fn an_avoider_that_sights_while_the_enemy_does_not_slips_away_unseen() {
        // Night. Force 1 avoids and is alert: chance 1. Force 2 is not alert:
        // chance 0. Round 1: force 1 sights, force 2 does not, so force 1
        // slips away: no fire, and the fight ends in round 1.
        let hex = Hex::new(5, 0);
        let scene = night_scene(vec![
            ForceSpec::new(1, Side::Japanese, hex, &[1, 1]).stance(Stance::Avoid),
            ForceSpec::new(2, Side::Allied, hex, &[1, 1]).not_alert(),
        ]);
        let world = run(&scene, only_force_1_sights_by_roll);

        assert!(salvos(&world).is_empty(), "no salvo");
        let slips = events_of(&world, |k| matches!(k, EventKind::ForceSlippedAway { .. }));
        assert_eq!(slips.len(), 1);
        assert!(matches!(
            slips[0].kind,
            EventKind::ForceSlippedAway { round: 1, task_force, enemy }
                if task_force == TaskForceId::new(1) && enemy == TaskForceId::new(2)
        ));
        assert_eq!(endings(&world), vec![(1, EngagementEndReason::SlippedAway)]);
        // It cites its successful sighting roll.
        let roll = world
            .events()
            .iter()
            .find(|e| matches!(e.kind, EventKind::SightingRoll { sighted: true, .. }))
            .expect("force 1 sighted by rolling");
        assert!(slips[0].causes.contains(&Cause::Event(roll.id)));
        // Nobody was damaged, and nobody broke off.
        assert_eq!(
            count(&world, |k| matches!(
                k,
                EventKind::ShipDamaged { .. } | EventKind::ForceBrokeOff { .. }
            )),
            0
        );
        // The avoider reported what it saw; the force that did not sight did
        // not, and nobody claimed anything, since nobody fired.
        assert_eq!(world.beliefs[&Side::Japanese].contacts().count(), 1);
        assert_eq!(world.beliefs[&Side::Allied].contacts().count(), 0);
        assert!(world.beliefs[&Side::Japanese].claims().is_empty());
        assert!(world.beliefs[&Side::Allied].claims().is_empty());
    }

    #[test]
    fn two_avoiders_that_both_sight_fire_and_both_break_off_after_round_1() {
        // Day: both sight in round 1 (chance 1), so neither slips (a slip
        // needs an enemy that has not sighted). Both fire in round 1, both
        // were fired on, and an avoider breaks off at the end of the first
        // round it was fired on: both break off at the end of round 1.
        let hex = Hex::new(5, 0);
        let scene = day_scene(vec![
            ForceSpec::new(1, Side::Japanese, hex, &[1]).stance(Stance::Avoid),
            ForceSpec::new(2, Side::Allied, hex, &[1]).stance(Stance::Avoid),
        ]);
        let world = run(&scene, harmless);

        assert_eq!(
            count(&world, |k| matches!(k, EventKind::ForceSlippedAway { .. })),
            0
        );
        let guns: Vec<u32> = salvos(&world)
            .iter()
            .filter(|s| s.weapon == Weapon::Gun)
            .map(|s| s.firer)
            .collect();
        assert_eq!(guns, vec![1, 2], "both fire in round 1 and not after");
        let broke_off = events_of(&world, |k| matches!(k, EventKind::ForceBrokeOff { .. }));
        assert_eq!(broke_off.len(), 2);
        for (event, force) in broke_off.iter().zip([1, 2]) {
            // No ship was crippled, so the share is 0: it left because it
            // was fired on.
            assert!(matches!(
                event.kind,
                EventKind::ForceBrokeOff { round: 1, task_force, share }
                    if task_force == TaskForceId::new(force) && share == Fixed::ZERO
            ));
        }
        assert_eq!(endings(&world), vec![(1, EngagementEndReason::BrokeOff)]);
    }

    #[test]
    fn an_avoider_surprised_in_round_1_does_not_fire_and_breaks_off_at_its_end() {
        // Night. Force 1 avoids and is not alert: chance 0. Force 2 is alert:
        // chance 1, so only force 2 sights. Force 1 is not the one that has
        // sighted, so it cannot slip away. Force 2 fires; force 1 is
        // surprised and does not fire. Force 1 avoids and was fired on, so it
        // breaks off at the end of round 1. Force 2 was not hit, so it stays.
        let hex = Hex::new(5, 0);
        let scene = night_scene(vec![
            ForceSpec::new(1, Side::Japanese, hex, &[1, 1])
                .stance(Stance::Avoid)
                .not_alert(),
            ForceSpec::new(2, Side::Allied, hex, &[1, 1]),
        ]);
        let world = run(&scene, |w| {
            let figures = figures_mut(w);
            figures.sighting_chance_night = Fixed::ONE;
            figures.not_alert_sighting_factor = Fixed::ZERO;
            harmless(w);
        });

        assert_eq!(
            rolls(&world),
            vec![(1, 1, Fixed::ZERO, false), (1, 2, Fixed::ONE, true)]
        );
        assert_eq!(
            count(&world, |k| matches!(k, EventKind::ForceSlippedAway { .. })),
            0
        );
        assert!(
            salvos(&world).iter().all(|s| s.firer == 3 || s.firer == 4),
            "only force 2's ships (3 and 4) fire"
        );
        assert_eq!(salvos(&world).iter().filter(|s| s.round > 1).count(), 0);
        assert_eq!(
            count(
                &world,
                |k| matches!(k, EventKind::ForceSurprised { round: 1, task_force, .. }
                if *task_force == TaskForceId::new(1))
            ),
            1
        );
        let broke_off = events_of(&world, |k| matches!(k, EventKind::ForceBrokeOff { .. }));
        assert_eq!(broke_off.len(), 1);
        assert!(matches!(
            broke_off[0].kind,
            EventKind::ForceBrokeOff { round: 1, task_force, .. }
                if task_force == TaskForceId::new(1)
        ));
        assert_eq!(endings(&world), vec![(1, EngagementEndReason::BrokeOff)]);
    }

    // ---- 4. break-off ----

    /// Night, nobody sights (chance 0), so nobody fires. The ships of force 1
    /// are as many as `ships`, and ship 1 has `flotation` damage.
    fn pre_damaged(ships: usize, flotation: Fixed, break_off: Fixed) -> World {
        let hex = Hex::new(5, 0);
        let scene = night_scene(vec![
            ForceSpec::new(1, Side::Japanese, hex, &vec![1; ships]).break_off(break_off),
            ForceSpec::new(2, Side::Allied, hex, &[1, 1]),
        ]);
        run(&scene, |w| {
            figures_mut(w).sighting_chance_night = Fixed::ZERO;
            ship_mut(w, 1).flotation_damage = flotation;
        })
    }

    fn broke_off_forces(world: &World) -> Vec<(u32, u32, Fixed)> {
        world
            .events()
            .iter()
            .filter_map(|event| match event.kind {
                EventKind::ForceBrokeOff {
                    round,
                    task_force,
                    share,
                } => Some((round, task_force.raw(), share)),
                _ => None,
            })
            .collect()
    }

    #[test]
    fn a_force_breaks_off_at_exactly_its_level_and_not_just_under_it() {
        // Force 1 has 2 ships; ship 1 is crippled at flotation 50 (the
        // placeholder level). Lost: 1 of 2 = 0.5. Nobody fires, so nothing
        // else happens.
        //
        // Break-off 0.5: 1 x 1 >= 0.5 x 2 = 1: breaks off at the end of
        // round 1 with share 0.5.
        let world = pre_damaged(2, Fixed::from_int(50), fx("0.5"));
        assert_eq!(broke_off_forces(&world), vec![(1, 1, fx("0.5"))]);
        assert_eq!(endings(&world), vec![(1, EngagementEndReason::BrokeOff)]);

        // Break-off 0.500001: 1 x 1 >= 0.500001 x 2 = 1.000002 is false. No
        // break-off by anyone (force 2 has lost nothing), so the fight runs
        // the six rounds and they pass unseen.
        let world = pre_damaged(2, Fixed::from_int(50), fx("0.500001"));
        assert!(broke_off_forces(&world).is_empty());
        assert_eq!(
            endings(&world),
            vec![(6, EngagementEndReason::PassedUnseen)]
        );

        // Flotation 49.999999 is not crippled: lost 0 of 2, 0 >= 1 is false.
        let world = pre_damaged(2, Fixed::from_raw(49_999_999), fx("0.5"));
        assert!(broke_off_forces(&world).is_empty());
        assert_eq!(
            endings(&world),
            vec![(6, EngagementEndReason::PassedUnseen)]
        );
    }

    #[test]
    fn break_off_compares_without_dividing() {
        // Three ships, one crippled: lost 1 of 3, which is not a number
        // `Fixed` can hold, so the rule compares count x 1 with
        // break-off x ships. Break-off 0.333333: 1 >= 0.333333 x 3 = 0.999999
        // is true. Break-off 0.333334: 1 >= 1.000002 is false.
        let world = pre_damaged(3, Fixed::from_int(50), fx("0.333333"));
        assert_eq!(broke_off_forces(&world).len(), 1);
        let world = pre_damaged(3, Fixed::from_int(50), fx("0.333334"));
        assert!(broke_off_forces(&world).is_empty());
    }

    // ---- 5. a ship sunk in a round fires in it and not after ----

    #[test]
    fn a_ship_sunk_in_a_round_fires_in_that_round_and_not_after() {
        // Day, both sight in round 1. Three cruisers each. Gun hits all land
        // on flotation, torpedoes miss. A gun salvo is 8 x 3 x 5 = 120 shots
        // x 0.05 = 6 expected hits. Each shot is rolled (decision 0033), so
        // the test sets the day chance to 1 and the salvo to 6 shots
        // (`flotation_only`): 6 sure hits, each 100 x 20 / 400 = 5 points:
        // 30 points per salvo.
        //
        // Ship 4 (force 2) starts at flotation 95. Round 1: ship 1 -> 4,
        // 2 -> 5, 3 -> 6 and 4 -> 1, 5 -> 2, 6 -> 3. Ship 4 takes 30:
        // min(100, 125) = 100, and sinks. It still fires in round 1 (afloat
        // at the start). Ships 5 and 6 have 30. Share lost by force 2 is
        // 1 of 3 = 0.33 < 0.5, force 1 lost none.
        //
        // Round 2: force 2 has ships 5 and 6 afloat. Force 1 firers 1, 2, 3
        // -> 5, 6, 5; force 2 firers 5, 6 -> 1, 2. Ship 5 = 30 + 30 + 30 =
        // 90, ship 6 = 30 + 30 = 60, both crippled, plus the sunk ship 4:
        // 3 of 3 >= 0.5. Ships 1 and 2 = 60 (crippled), ship 3 = 30: 2 of 3
        // = 0.667 >= 0.5. Both break off at the end of round 2.
        let hex = Hex::new(5, 0);
        let scene = day_scene(vec![
            ForceSpec::new(1, Side::Japanese, hex, &[1, 1, 1]),
            ForceSpec::new(2, Side::Allied, hex, &[1, 1, 1]),
        ]);
        let world = run(&scene, |w| {
            flotation_only(w);
            ship_mut(w, 4).flotation_damage = Fixed::from_int(95);
        });

        let sunk = events_of(&world, |k| matches!(k, EventKind::ShipSunk { .. }));
        assert_eq!(sunk.len(), 1);
        assert!(matches!(
            sunk[0].kind,
            EventKind::ShipSunk { round: 1, ship, task_force }
                if ship == ShipId::new(4) && task_force == TaskForceId::new(2)
        ));
        assert_eq!(ship(&world, 4).flotation_damage, Fixed::from_int(100));
        assert_eq!(ship(&world, 4).location, ShipLocation::Sunk);
        assert!(!world.truth.task_forces[&TaskForceId::new(2)]
            .ships
            .contains(&ShipId::new(4)));

        let fired_by_4: Vec<u32> = salvos(&world)
            .iter()
            .filter(|s| s.firer == 4)
            .map(|s| s.round)
            .collect();
        assert_eq!(fired_by_4, vec![1, 1], "a gun and a torpedo salvo, round 1");
        assert!(
            salvos(&world).iter().all(|s| s.round == 1 || s.target != 4),
            "nothing is fired at the sunk ship after round 1"
        );
        let round_2: Vec<(u32, u32)> = salvos(&world)
            .iter()
            .filter(|s| s.round == 2 && s.weapon == Weapon::Gun)
            .map(|s| (s.firer, s.target))
            .collect();
        assert_eq!(round_2, vec![(1, 5), (2, 6), (3, 5), (5, 1), (6, 2)]);
        assert_eq!(endings(&world), vec![(2, EngagementEndReason::BrokeOff)]);
        assert_eq!(ship(&world, 5).flotation_damage, Fixed::from_int(90));
        assert_eq!(ship(&world, 6).flotation_damage, Fixed::from_int(60));
    }

    #[test]
    fn air_groups_aboard_a_ship_that_sinks_are_lost_with_it() {
        use crate::ids::{AirGroupId, AircraftTypeId};

        use crate::state::{AirGroup, AirGroupLocation};

        // The sinking of ship 4 in round 1 as in the test above. Two groups
        // are aboard it (8 aircraft and none) and one aboard ship 5, which
        // survives.
        let hex = Hex::new(5, 0);
        let scene = day_scene(vec![
            ForceSpec::new(1, Side::Japanese, hex, &[1, 1, 1]),
            ForceSpec::new(2, Side::Allied, hex, &[1, 1, 1]),
        ]);
        let group = |id: u32, ship: u32, ready: u32| AirGroup {
            ready,
            strength: ready,
            ..AirGroup::new(
                AirGroupId::new(id),
                format!("Group {id}"),
                Side::Allied,
                AirGroupLocation::Ship(ShipId::new(ship)),
                AircraftTypeId::new(1),
            )
        };
        let world = run(&scene, |w| {
            flotation_only(w);
            ship_mut(w, 4).flotation_damage = Fixed::from_int(95);
            for (id, ship, ready) in [(1, 4, 8), (2, 4, 0), (3, 5, 6)] {
                w.truth
                    .air_groups
                    .insert(AirGroupId::new(id), group(id, ship, ready));
            }
        });

        let sunk = events_of(&world, |k| matches!(k, EventKind::ShipSunk { .. }));
        assert_eq!(sunk.len(), 1);
        let lost = events_of(&world, |k| matches!(k, EventKind::AirGroupLost { .. }));
        assert_eq!(lost.len(), 2, "only groups aboard the sunk ship are lost");
        let summary: Vec<(AirGroupId, ShipId, u32)> = lost
            .iter()
            .map(|e| match e.kind {
                EventKind::AirGroupLost {
                    air_group,
                    ship,
                    aircraft,
                } => (air_group, ship, aircraft),
                _ => unreachable!(),
            })
            .collect();
        assert_eq!(
            summary,
            vec![
                (AirGroupId::new(1), ShipId::new(4), 8),
                (AirGroupId::new(2), ShipId::new(4), 0),
            ]
        );
        for event in &lost {
            assert_eq!(event.causes, vec![Cause::Event(sunk[0].id)]);
        }
        assert_eq!(
            lost[0].kind.to_string(),
            "air_group:1 lost with ship:4: 8 aircraft"
        );
        // The groups stay aboard the sunk ship with no aircraft; the other
        // ship's group is untouched.
        let groups = &world.truth.air_groups;
        assert_eq!(groups[&AirGroupId::new(1)].ready, 0);
        assert_eq!(
            groups[&AirGroupId::new(1)].location,
            AirGroupLocation::Ship(ShipId::new(4))
        );
        assert_eq!(groups[&AirGroupId::new(3)].ready, 6);
        assert_eq!(
            world.truth.air_group_hex(&groups[&AirGroupId::new(1)]),
            None
        );
    }

    // ---- 6. matching ----

    #[test]
    fn three_firers_against_two_targets_go_1_to_a_2_to_b_3_to_a() {
        // Round 1, day, both sight. Force 1 has ships 1, 2, 3; force 2 has
        // ships 4, 5. Firer i takes target i mod 2: 1 -> 4, 2 -> 5, 3 -> 4,
        // and for force 2 (two firers, three targets): 4 -> 1, 5 -> 2.
        let hex = Hex::new(5, 0);
        let scene = day_scene(vec![
            ForceSpec::new(1, Side::Japanese, hex, &[1, 1, 1]),
            ForceSpec::new(2, Side::Allied, hex, &[1, 1]),
        ]);
        let world = run(&scene, harmless);
        let pairs: Vec<(u32, u32)> = salvos(&world)
            .iter()
            .filter(|s| s.round == 1 && s.weapon == Weapon::Gun)
            .map(|s| (s.firer, s.target))
            .collect();
        assert_eq!(pairs, vec![(1, 4), (2, 5), (3, 4), (4, 1), (5, 2)]);
    }

    #[test]
    fn one_firer_against_three_targets_fires_at_the_first() {
        // Force 1 has ship 1; force 2 has ships 2, 3, 4. Ship 1 takes target
        // 0 mod 3 = ship 2. Force 2's firers 2, 3, 4 against one target all
        // take ship 1.
        let hex = Hex::new(5, 0);
        let scene = day_scene(vec![
            ForceSpec::new(1, Side::Japanese, hex, &[1]),
            ForceSpec::new(2, Side::Allied, hex, &[1, 1, 1]),
        ]);
        let world = run(&scene, harmless);
        let pairs: Vec<(u32, u32)> = salvos(&world)
            .iter()
            .filter(|s| s.round == 1 && s.weapon == Weapon::Gun)
            .map(|s| (s.firer, s.target))
            .collect();
        assert_eq!(pairs, vec![(1, 2), (2, 1), (3, 1), (4, 1)]);
    }

    // ---- 7. ammunition ----

    #[test]
    fn gun_shots_are_capped_by_ammunition_and_use_it_up() {
        // Day, harmless, the fight runs 6 rounds. Ship 1 has 130 rounds: a
        // full salvo is 120 shots, so round 1 uses 120 (10 left), round 2 is
        // min(120, 10) = 10 shots (0 left), and rounds 3 to 6 have no gun
        // salvo at all. Its salvo of 10 shots at day chance 0.05 expects 0.5.
        //
        // Ship 2 keeps its 8 x 100 = 800 rounds less 6 salvos of 120 = 80.
        let hex = Hex::new(5, 0);
        let scene = day_scene(vec![
            ForceSpec::new(1, Side::Japanese, hex, &[1]),
            ForceSpec::new(2, Side::Allied, hex, &[1]),
        ]);
        let world = run(&scene, |w| {
            harmless(w);
            ship_mut(w, 1).gun_rounds = Fixed::from_int(130);
        });
        let guns = gun_salvos_of(&world, 1);
        let seen: Vec<(u32, Fixed)> = guns.iter().map(|s| (s.round, s.shots)).collect();
        assert_eq!(
            seen,
            vec![(1, Fixed::from_int(120)), (2, Fixed::from_int(10))]
        );
        assert_eq!(guns[1].chance, fx("0.05"));
        assert_eq!(guns[1].expected, fx("0.5"));
        assert_eq!(ship(&world, 1).gun_rounds, Fixed::ZERO);
        assert_eq!(gun_salvos_of(&world, 2).len(), 6);
        assert_eq!(ship(&world, 2).gun_rounds, Fixed::from_int(80));
    }

    #[test]
    fn fifty_rounds_make_one_salvo_of_fifty_shots_and_then_no_gun_salvo() {
        // min(120, 50) = 50 shots in round 1, 0 left, none after.
        let hex = Hex::new(5, 0);
        let scene = day_scene(vec![
            ForceSpec::new(1, Side::Japanese, hex, &[1]),
            ForceSpec::new(2, Side::Allied, hex, &[1]),
        ]);
        let world = run(&scene, |w| {
            harmless(w);
            ship_mut(w, 1).gun_rounds = Fixed::from_int(50);
        });
        let guns = gun_salvos_of(&world, 1);
        assert_eq!(guns.len(), 1);
        assert_eq!((guns[0].round, guns[0].shots), (1, Fixed::from_int(50)));
        // 50 x 0.05 = 2.5 expected; the hits are what 50 draws at 0.05 give.
        assert_eq!(guns[0].expected, fx("2.5"));
        let mut stream = ship_1_stream(&world);
        assert_eq!(guns[0].hits, replay_shots(&mut stream, 50, fx("0.05")));
        assert_eq!(ship(&world, 1).gun_rounds, Fixed::ZERO);
    }

    #[test]
    fn a_ship_with_no_ammunition_fires_torpedoes_only() {
        // Ship 1 has no gun rounds: no gun salvo ever, but its 8 tubes fire
        // once, in round 1: 8 shots x 0.02 (day) = 0.16 expected.
        let hex = Hex::new(5, 0);
        let scene = day_scene(vec![
            ForceSpec::new(1, Side::Japanese, hex, &[1]),
            ForceSpec::new(2, Side::Allied, hex, &[1]),
        ]);
        let world = run(&scene, |w| {
            ship_mut(w, 1).gun_rounds = Fixed::ZERO;
        });
        assert!(gun_salvos_of(&world, 1).is_empty());
        let torpedoes = torpedo_salvos_of(&world, 1);
        assert_eq!(torpedoes.len(), 1);
        assert_eq!(torpedoes[0].round, 1);
        assert_eq!(torpedoes[0].shots, Fixed::from_int(8));
        assert_eq!(torpedoes[0].chance, fx("0.02"));
        assert_eq!(torpedoes[0].expected, fx("0.16"));
    }

    // ---- 8. named systems (decision 0016) ----

    /// Day, one cruiser each (ships 1 and 2), nothing sunk or hit.
    fn duel() -> Scene {
        let hex = Hex::new(5, 0);
        day_scene(vec![
            ForceSpec::new(1, Side::Japanese, hex, &[1]),
            ForceSpec::new(2, Side::Allied, hex, &[1]),
        ])
    }

    #[test]
    fn a_degraded_main_battery_fires_60_shots_a_round() {
        // Shots = 8 guns x 3 rate x 5 minutes x 0.5 = 60, in each of the six
        // rounds (harmless: no hit knocks a system down further), using
        // 6 x 60 = 360 of the 800 rounds. Chance is the plain day 0.05, so
        // 60 x 0.05 = 3 expected hits.
        let world = run(&duel(), |w| {
            harmless(w);
            ship_mut(w, 1).systems.main_battery = SystemState::Degraded;
        });
        let guns = gun_salvos_of(&world, 1);
        assert_eq!(guns.len(), 6);
        assert!(guns.iter().all(|s| s.shots == Fixed::from_int(60)));
        assert_eq!(guns[0].expected, Fixed::from_int(3));
        assert_eq!(ship(&world, 1).gun_rounds, Fixed::from_int(440));
        // The other ship is untouched: 120 shots a round.
        assert!(gun_salvos_of(&world, 2)
            .iter()
            .all(|s| s.shots == Fixed::from_int(120)));
    }

    #[test]
    fn a_main_battery_that_is_out_fires_no_guns_but_still_its_torpedoes() {
        // Shots = 120 x 0 = 0: no gun salvo (no event, no draws), the
        // torpedoes go in round 1 and no ammunition is used.
        let world = run(&duel(), |w| {
            harmless(w);
            ship_mut(w, 1).systems.main_battery = SystemState::Out;
        });
        assert!(gun_salvos_of(&world, 1).is_empty());
        let torpedoes = torpedo_salvos_of(&world, 1);
        assert_eq!(torpedoes.len(), 1);
        assert_eq!(torpedoes[0].round, 1);
        assert_eq!(ship(&world, 1).gun_rounds, Fixed::from_int(800));
    }

    #[test]
    fn degraded_fire_control_halves_the_gun_chance_and_out_quarters_it() {
        // Day, round 1. Gun chance = 0.05 base x 1 surprised x 1 lit x the
        // firer's fire control factor.
        //   degraded: 0.05 x 0.5 = 0.025, 120 x 0.025 = 3 expected
        //   out:      0.05 x 0.25 = 0.0125, 120 x 0.0125 = 1.5 expected
        // The torpedo chance (0.02, 8 x 0.02 = 0.16 expected) is not touched.
        for (state, chance, expected) in [
            (SystemState::Working, fx("0.05"), fx("6")),
            (SystemState::Degraded, fx("0.025"), fx("3")),
            (SystemState::Out, fx("0.0125"), fx("1.5")),
        ] {
            let world = run(&duel(), |w| {
                harmless(w);
                figures_mut(w).torpedo_hit_chance_day = fx("0.02");
                ship_mut(w, 1).systems.fire_control = state;
            });
            let gun = first_round_salvo(&world, 1, Weapon::Gun);
            assert_eq!(gun.shots, Fixed::from_int(120), "{state:?}");
            assert_eq!(gun.chance, chance, "{state:?}");
            assert_eq!(gun.expected, expected, "{state:?}");
            let torpedo = first_round_salvo(&world, 1, Weapon::Torpedo);
            assert_eq!(torpedo.chance, fx("0.02"), "{state:?}");
            assert_eq!(torpedo.shots, Fixed::from_int(8), "{state:?}");
        }
    }

    #[test]
    fn fire_control_is_the_firers_and_multiplies_after_surprise_and_light() {
        // Night. Ship 2 (not alert) is surprised and lit. Ship 1's fire
        // control is degraded. Gun chance = 0.03 x 1.5 x 2 x 0.5 = 0.045;
        // 120 x 0.045 = 5.4 expected. The target's own fire control is not
        // read: ship 2's is out, and ship 1's chance stays the same.
        let hex = Hex::new(5, 0);
        let scene = night_scene(vec![
            ForceSpec::new(1, Side::Japanese, hex, &[1]),
            ForceSpec::new(2, Side::Allied, hex, &[1]).not_alert(),
        ]);
        let world = run(&scene, |w| {
            only_force_1_sights_by_roll(w);
            ship_mut(w, 2).fire_damage = Fixed::from_int(10);
            ship_mut(w, 2).systems.fire_control = SystemState::Out;
            ship_mut(w, 1).systems.fire_control = SystemState::Degraded;
        });
        let gun = first_round_salvo(&world, 1, Weapon::Gun);
        assert_eq!(gun.chance, fx("0.045"));
        assert_eq!(gun.expected, fx("5.4"));
    }

    #[test]
    fn degraded_torpedo_mounts_fire_4_tubes_and_out_fire_none() {
        // Degraded: 8 tubes x 0.5 = 4 shots, at 0.02 = 0.08 expected, and the
        // torpedoes are then gone. Out: 8 x 0 = 0 shots, no torpedo salvo and
        // no draws. Nothing was fired, so the torpedoes are still aboard.
        let world = run(&duel(), |w| {
            harmless(w);
            figures_mut(w).torpedo_hit_chance_day = fx("0.02");
            ship_mut(w, 1).systems.torpedo_mounts = SystemState::Degraded;
        });
        let torpedoes = torpedo_salvos_of(&world, 1);
        assert_eq!(torpedoes.len(), 1);
        assert_eq!(
            (torpedoes[0].round, torpedoes[0].shots),
            (1, Fixed::from_int(4))
        );
        assert_eq!(torpedoes[0].expected, fx("0.08"));
        assert!(!ship(&world, 1).torpedoes_ready);

        let world = run(&duel(), |w| {
            harmless(w);
            ship_mut(w, 1).systems.torpedo_mounts = SystemState::Out;
        });
        assert!(torpedo_salvos_of(&world, 1).is_empty());
        assert!(ship(&world, 1).torpedoes_ready);
        // Its guns are not affected.
        assert_eq!(gun_salvos_of(&world, 1).len(), 6);
        // The other ship's torpedoes went as usual.
        assert_eq!(torpedo_salvos_of(&world, 2).len(), 1);
    }

    /// Ship 1 has two gun rounds and no torpedoes, ship 2 fires nothing, and
    /// every gun hit lands on flotation and always knocks down `system`:
    /// ship 1's salvo is 2 shots at chance 1, so exactly 2 hits.
    fn two_forced_hits_on(system: ShipSystem) -> impl FnOnce(&mut World) {
        move |w| {
            let figures = figures_mut(w);
            figures.gun_hit_chance_day = Fixed::ONE;
            figures.gun_hit_flotation = Fixed::ONE;
            figures.system_hit_chance_per_point = Fixed::from_int(100);
            figures.gun_system_hit = [(system, Fixed::ONE)].into_iter().collect();
            let attacker = ship_mut(w, 1);
            attacker.gun_rounds = Fixed::from_int(2);
            attacker.torpedoes_ready = false;
            let target = ship_mut(w, 2);
            target.gun_rounds = Fixed::ZERO;
            target.torpedoes_ready = false;
        }
    }

    fn system_hits(world: &World) -> Vec<(EventId, u32, ShipSystem, SystemState, SystemState)> {
        world
            .events()
            .iter()
            .filter_map(|event| match event.kind {
                EventKind::SystemHit {
                    ship,
                    system,
                    before,
                    after,
                    ..
                } => Some((event.id, ship.raw(), system, before, after)),
                _ => None,
            })
            .collect()
    }

    #[test]
    fn two_hits_on_one_system_in_one_round_take_it_from_working_to_out() {
        // 2 shots x chance 1 = 2 hits, each a system hit on propulsion with
        // chance 1 x share 1. Working -> degraded -> out, in hit order, in
        // round 1. Flotation takes 2 x 5 = 10 points, and the events of the
        // salvo come in the order: track damage, then system hits.
        let world = run(&duel(), two_forced_hits_on(ShipSystem::Propulsion));
        let salvo = gun_salvos_of(&world, 1);
        assert_eq!(salvo.len(), 1);
        assert_eq!(salvo[0].hits, 2);
        let hits = system_hits(&world);
        assert_eq!(
            hits.iter()
                .map(|(_, ship, system, before, after)| (*ship, *system, *before, *after))
                .collect::<Vec<_>>(),
            vec![
                (
                    2,
                    ShipSystem::Propulsion,
                    SystemState::Working,
                    SystemState::Degraded
                ),
                (
                    2,
                    ShipSystem::Propulsion,
                    SystemState::Degraded,
                    SystemState::Out
                ),
            ]
        );
        assert_eq!(ship(&world, 2).systems.propulsion, SystemState::Out);
        assert_eq!(ship(&world, 2).systems.fire_control, SystemState::Working);
        assert_eq!(ship(&world, 2).flotation_damage, Fixed::from_int(10));
        // Each cites the salvo, and comes after the salvo's track damage.
        let kinds_after_salvo: Vec<&'static str> = world
            .events()
            .iter()
            .filter(|e| e.causes == vec![Cause::Event(salvo[0].id)])
            .map(|e| match e.kind {
                EventKind::ShipDamaged { .. } => "damaged",
                EventKind::SystemHit { .. } => "system",
                _ => "other",
            })
            .collect();
        assert_eq!(kinds_after_salvo, vec!["damaged", "system", "system"]);
        // The rest of the cruiser's fire is unaffected the next round: it
        // fired nothing after the two rounds of ammunition anyway.
        assert!(gun_salvos_of(&world, 1).iter().all(|s| s.round == 1));
    }

    #[test]
    fn a_hit_on_a_system_already_out_changes_nothing_but_is_recorded() {
        // Ship 2's propulsion is already out. Two hits on it: out -> out
        // twice, each recorded with before == after.
        let world = run(&duel(), |w| {
            two_forced_hits_on(ShipSystem::Propulsion)(w);
            ship_mut(w, 2).systems.propulsion = SystemState::Out;
        });
        let hits = system_hits(&world);
        assert_eq!(hits.len(), 2);
        for (_, ship_id, system, before, after) in hits {
            assert_eq!(ship_id, 2);
            assert_eq!(system, ShipSystem::Propulsion);
            assert_eq!(before, SystemState::Out);
            assert_eq!(after, SystemState::Out);
        }
        assert_eq!(ship(&world, 2).systems.propulsion, SystemState::Out);
    }

    #[test]
    fn a_hit_on_a_system_the_ship_lacks_changes_nothing_but_is_recorded() {
        // Ship 2 is a transport (class 3): no guns, no tubes, so only
        // propulsion. Two hits on its main battery change nothing.
        let hex = Hex::new(5, 0);
        let scene = day_scene(vec![
            ForceSpec::new(1, Side::Japanese, hex, &[1]),
            ForceSpec::new(2, Side::Allied, hex, &[3]),
        ]);
        let world = run(&scene, two_forced_hits_on(ShipSystem::MainBattery));
        let hits = system_hits(&world);
        assert_eq!(hits.len(), 2);
        for (_, ship_id, system, before, after) in hits {
            assert_eq!(ship_id, 2);
            assert_eq!(system, ShipSystem::MainBattery);
            assert_eq!(before, SystemState::Working);
            assert_eq!(after, SystemState::Working);
        }
        assert_eq!(ship(&world, 2).systems, ShipSystems::ALL_WORKING);
        let violations = invariants::check(world.umpire(), world.map(), world.ref_data());
        assert!(violations.is_empty(), "{violations:?}");
    }

    #[test]
    fn system_hits_land_at_the_end_of_the_round_and_bite_in_the_next() {
        // Day. Every gun hit knocks the main battery down one level. Ship 1
        // fires 120 shots at 0.05 (6 expected, each shot rolled) at ship 2
        // in round 1, and ship 2 the same at ship 1. Each gun hit knocks the
        // main battery down a level, so with at least two hits (as both have
        // with this seed) both main batteries are out at the end of round 1
        // (working -> degraded -> out -> out ...). Both still fired their
        // full 120 shots in round 1, and neither fires a gun in round 2.
        let world = run(&duel(), |w| {
            let figures = figures_mut(w);
            figures.system_hit_chance_per_point = Fixed::from_int(100);
            figures.gun_system_hit = [(ShipSystem::MainBattery, Fixed::ONE)]
                .into_iter()
                .collect();
            figures.torpedo_hit_chance_day = Fixed::ZERO;
        });
        for firer in [1, 2] {
            let guns = gun_salvos_of(&world, firer);
            assert_eq!(guns.len(), 1, "ship {firer}");
            assert_eq!(guns[0].shots, Fixed::from_int(120));
            assert!(guns[0].hits >= 2, "ship {firer}: {}", guns[0].hits);
            assert_eq!(ship(&world, firer).systems.main_battery, SystemState::Out);
        }
    }

    #[test]
    fn a_force_whose_salvos_are_all_dead_surprises_nobody() {
        // Night. Force 1 sights (chance 1), force 2 never does. Ship 1's main
        // battery and torpedo mounts are out: it has ammunition and torpedoes
        // aboard but no real salvo, so force 2 is not surprised and nothing
        // is fired.
        let hex = Hex::new(5, 0);
        let scene = night_scene(vec![
            ForceSpec::new(1, Side::Japanese, hex, &[1]),
            ForceSpec::new(2, Side::Allied, hex, &[1]).not_alert(),
        ]);
        let world = run(&scene, |w| {
            only_force_1_sights_by_roll(w);
            let ship = ship_mut(w, 1);
            ship.systems.main_battery = SystemState::Out;
            ship.systems.torpedo_mounts = SystemState::Out;
        });
        assert!(salvos(&world).is_empty());
        assert_eq!(
            count(&world, |k| matches!(k, EventKind::ForceSurprised { .. })),
            0
        );
        // No torpedo salvo was fired, so the torpedoes are still aboard.
        assert!(ship(&world, 1).torpedoes_ready);
        assert_eq!(endings(&world), vec![(6, EngagementEndReason::MaxRounds)]);

        // With only the main battery out, the torpedoes are a real salvo and
        // do surprise force 2.
        let world = run(&scene, |w| {
            only_force_1_sights_by_roll(w);
            ship_mut(w, 1).systems.main_battery = SystemState::Out;
        });
        assert_eq!(
            count(&world, |k| matches!(k, EventKind::ForceSurprised { .. })),
            1
        );
    }

    /// The factors of an event's breakdown, as (name, value).
    fn breakdown_of(event: &Event) -> Vec<(&'static str, Fixed)> {
        let breakdown = event.breakdown.as_ref().expect("a breakdown");
        breakdown
            .factors
            .iter()
            .map(|f| (f.name, f.value))
            .collect()
    }

    #[test]
    fn salvo_breakdowns_name_the_system_factors_in_order() {
        // Night, ship 2 surprised and lit; ship 1 has degraded main battery,
        // degraded torpedo mounts and fire control out.
        //   gun:     120 x 0.5 shots, chance 0.03 x 1.5 x 2 x 0.25 = 0.0225
        //   torpedo: 4 shots, chance 0.04 x 1.5 x 2 = 0.12 (no fire control)
        let hex = Hex::new(5, 0);
        let scene = night_scene(vec![
            ForceSpec::new(1, Side::Japanese, hex, &[1]),
            ForceSpec::new(2, Side::Allied, hex, &[1]).not_alert(),
        ]);
        let mut world = scene.world();
        world.set_config(SimConfig {
            record_breakdowns: true,
        });
        only_force_1_sights_by_roll(&mut world);
        ship_mut(&mut world, 2).fire_damage = Fixed::from_int(10);
        let systems = &mut ship_mut(&mut world, 1).systems;
        systems.main_battery = SystemState::Degraded;
        systems.torpedo_mounts = SystemState::Degraded;
        systems.fire_control = SystemState::Out;
        scene.run_step(&mut world);

        let gun = first_round_salvo(&world, 1, Weapon::Gun);
        let event = world.events().iter().find(|e| e.id == gun.id).unwrap();
        assert_eq!(
            breakdown_of(event),
            vec![
                ("guns", Fixed::from_int(8)),
                ("rate", Fixed::from_int(3)),
                ("minutes", Fixed::from_int(5)),
                ("main_battery_factor", fx("0.5")),
                ("base_chance", fx("0.03")),
                ("surprised_factor", fx("1.5")),
                ("lit_factor", Fixed::from_int(2)),
                ("fire_control_factor", fx("0.25")),
            ]
        );
        assert_eq!(gun.shots, Fixed::from_int(60));
        assert_eq!(gun.chance, fx("0.0225"));

        let torpedo = first_round_salvo(&world, 1, Weapon::Torpedo);
        let event = world.events().iter().find(|e| e.id == torpedo.id).unwrap();
        assert_eq!(
            breakdown_of(event),
            vec![
                ("tubes", Fixed::from_int(8)),
                ("rate", Fixed::ONE),
                ("minutes", Fixed::ONE),
                ("torpedo_mount_factor", fx("0.5")),
                ("base_chance", fx("0.04")),
                ("surprised_factor", fx("1.5")),
                ("lit_factor", Fixed::from_int(2)),
            ]
        );
        assert_eq!(torpedo.shots, Fixed::from_int(4));
        assert_eq!(torpedo.chance, fx("0.12"));
    }

    // ---- the system draw ----

    /// Propulsion, fire control, main battery, torpedo mounts: 0.2, 0.2, 0.2,
    /// 0.4.
    fn even_table() -> BTreeMap<ShipSystem, Fixed> {
        [
            (ShipSystem::Propulsion, fx("0.2")),
            (ShipSystem::FireControl, fx("0.2")),
            (ShipSystem::MainBattery, fx("0.2")),
            (ShipSystem::TorpedoMounts, fx("0.4")),
        ]
        .into_iter()
        .collect()
    }

    #[test]
    fn the_system_draw_bands_are_exact_at_their_boundaries() {
        // Chance 0.5 x shares 0.2 / 0.2 / 0.2 / 0.4 = widths 0.1, 0.1, 0.1,
        // 0.2: bands [0, 0.1), [0.1, 0.2), [0.2, 0.3), [0.3, 0.5), in system
        // code order. A draw is in millionths. 0.5 and above is no hit.
        let table = even_table();
        let draw = |raw: i64| system_for_draw(Fixed::from_raw(raw), fx("0.5"), &table);
        use ShipSystem::*;
        let expected = [
            (0, Some(Propulsion)),
            (99_999, Some(Propulsion)),
            (100_000, Some(FireControl)),
            (199_999, Some(FireControl)),
            (200_000, Some(MainBattery)),
            (299_999, Some(MainBattery)),
            (300_000, Some(TorpedoMounts)),
            (499_999, Some(TorpedoMounts)),
            (500_000, None),
            (999_999, None),
        ];
        for (raw, system) in expected {
            assert_eq!(draw(raw), system, "draw {raw}");
        }
    }

    #[test]
    fn a_system_absent_from_the_table_has_no_band_and_zero_chance_hits_nothing() {
        // Torpedo-like table: fire control absent. Chance 1, shares 0.8 /
        // 0 / 0.1 / 0.1: bands [0, 0.8) propulsion, none for fire control,
        // [0.8, 0.9) main battery, [0.9, 1) torpedo mounts.
        let table: BTreeMap<ShipSystem, Fixed> = [
            (ShipSystem::Propulsion, fx("0.8")),
            (ShipSystem::MainBattery, fx("0.1")),
            (ShipSystem::TorpedoMounts, fx("0.1")),
        ]
        .into_iter()
        .collect();
        let draw = |raw: i64| system_for_draw(Fixed::from_raw(raw), Fixed::ONE, &table);
        assert_eq!(draw(799_999), Some(ShipSystem::Propulsion));
        assert_eq!(draw(800_000), Some(ShipSystem::MainBattery));
        assert_eq!(draw(899_999), Some(ShipSystem::MainBattery));
        assert_eq!(draw(900_000), Some(ShipSystem::TorpedoMounts));
        assert_eq!(draw(999_999), Some(ShipSystem::TorpedoMounts));
        // Chance 0: every band is empty.
        for raw in [0, 1, 500_000, 999_999] {
            assert_eq!(
                system_for_draw(Fixed::from_raw(raw), Fixed::ZERO, &table),
                None
            );
        }
    }

    #[test]
    fn a_zero_system_chance_still_draws_but_never_hits() {
        // harmless() sets the system chance per point to 0: hits land, nothing is
        // knocked down, no SystemHit event. The draws are still taken, so the
        // stream stays aligned (checked by the draw order tests below).
        let world = run(&duel(), harmless);
        assert!(system_hits(&world).is_empty());
        assert_eq!(ship(&world, 1).systems, ShipSystems::ALL_WORKING);
        assert_eq!(ship(&world, 2).systems, ShipSystems::ALL_WORKING);
    }

    #[test]
    fn a_force_that_sighted_but_has_nothing_to_fire_surprises_nobody() {
        // Night. Force 1 sights (chance 1), force 2 never does (chance 0).
        // But ship 1 has no gun rounds and no torpedoes, so force 1 fires no
        // salvo, and a force is surprised only by a salvo. Nothing is fired in
        // six rounds, and the fight ends by the last round, not unseen: force
        // 1 sighted.
        let hex = Hex::new(5, 0);
        let scene = night_scene(vec![
            ForceSpec::new(1, Side::Japanese, hex, &[1]),
            ForceSpec::new(2, Side::Allied, hex, &[1]).not_alert(),
        ]);
        let world = run(&scene, |w| {
            only_force_1_sights_by_roll(w);
            let ship = ship_mut(w, 1);
            ship.gun_rounds = Fixed::ZERO;
            ship.torpedoes_ready = false;
        });
        assert!(salvos(&world).is_empty());
        assert_eq!(
            count(&world, |k| matches!(k, EventKind::ForceSurprised { .. })),
            0
        );
        assert_eq!(endings(&world), vec![(6, EngagementEndReason::MaxRounds)]);
    }

    // ---- 9. the lit factor ----

    fn first_round_salvo(world: &World, firer: u32, weapon: Weapon) -> Fired {
        salvos(world)
            .into_iter()
            .find(|s| s.round == 1 && s.firer == firer && s.weapon == weapon)
            .expect("the ship fired in round 1")
    }

    #[test]
    fn the_lit_factor_applies_at_night() {
        // Night, both alert with sighting chance 1, both sight in round 1.
        // Ship 2 has fire damage 0.000001 (any fire lights it).
        // Ship 1 -> 2: gun 0.03 x 2 = 0.06 (7.2 expected of 120 shots),
        // torpedo 0.04 x 2 = 0.08. Ship 2 -> 1 (ship 1 not lit): 0.03, 0.04.
        let hex = Hex::new(5, 0);
        let scene = night_scene(vec![
            ForceSpec::new(1, Side::Japanese, hex, &[1]),
            ForceSpec::new(2, Side::Allied, hex, &[1]),
        ]);
        let world = run(&scene, |w| {
            figures_mut(w).sighting_chance_night = Fixed::ONE;
            ship_mut(w, 2).fire_damage = Fixed::from_raw(1);
            w.set_config(SimConfig {
                record_breakdowns: true,
            });
        });
        let gun = first_round_salvo(&world, 1, Weapon::Gun);
        assert_eq!(gun.chance, fx("0.06"));
        assert_eq!(gun.expected, fx("7.2"));
        assert_eq!(
            first_round_salvo(&world, 1, Weapon::Torpedo).chance,
            fx("0.08")
        );
        assert_eq!(first_round_salvo(&world, 2, Weapon::Gun).chance, fx("0.03"));
        assert_eq!(
            first_round_salvo(&world, 2, Weapon::Torpedo).chance,
            fx("0.04")
        );
        // The breakdown names the factor.
        let event = world.events().get(gun.id).expect("the salvo event");
        let lit = event
            .breakdown
            .as_ref()
            .expect("breakdown")
            .factors
            .iter()
            .find(|f| f.name == "lit_factor")
            .expect("lit_factor listed");
        assert_eq!(lit.value, Fixed::from_int(2));
    }

    #[test]
    fn the_lit_factor_does_not_apply_by_day() {
        // Day: both sight, both ships burn (fire damage 50). Day chances are
        // 0.05 for guns and 0.02 for torpedoes with no lit factor.
        let hex = Hex::new(5, 0);
        let scene = day_scene(vec![
            ForceSpec::new(1, Side::Japanese, hex, &[1]),
            ForceSpec::new(2, Side::Allied, hex, &[1]),
        ]);
        let world = run(&scene, |w| {
            ship_mut(w, 1).fire_damage = Fixed::from_int(50);
            ship_mut(w, 2).fire_damage = Fixed::from_int(50);
        });
        for firer in [1, 2] {
            assert_eq!(
                first_round_salvo(&world, firer, Weapon::Gun).chance,
                fx("0.05")
            );
            assert_eq!(
                first_round_salvo(&world, firer, Weapon::Torpedo).chance,
                fx("0.02")
            );
        }
    }

    #[test]
    fn surprise_and_lit_factors_multiply() {
        // Night. Force 1 sights (chance 1), force 2 does not (chance 0) and
        // is surprised in round 1: gun chance 0.03 x 1.5 = 0.045. If its ship
        // also burns: 0.03 x 1.5 x 2 = 0.09, torpedo 0.04 x 1.5 x 2 = 0.12.
        let hex = Hex::new(5, 0);
        let scene = night_scene(vec![
            ForceSpec::new(1, Side::Japanese, hex, &[1]),
            ForceSpec::new(2, Side::Allied, hex, &[1]).not_alert(),
        ]);
        let world = run(&scene, only_force_1_sights_by_roll);
        let gun = first_round_salvo(&world, 1, Weapon::Gun);
        assert_eq!(gun.chance, fx("0.045"));
        assert_eq!(gun.expected, fx("5.4"));
        let world = run(&scene, |w| {
            only_force_1_sights_by_roll(w);
            ship_mut(w, 2).fire_damage = Fixed::from_int(10);
        });
        assert_eq!(first_round_salvo(&world, 1, Weapon::Gun).chance, fx("0.09"));
        assert_eq!(
            first_round_salvo(&world, 1, Weapon::Torpedo).chance,
            fx("0.12")
        );
    }

    // ---- 10. torpedoes fire once ----

    #[test]
    fn torpedoes_fire_once_in_the_first_round_and_never_in_a_later_fight() {
        // Day, harmless: the fight runs six rounds. Each cruiser fires its 8
        // tubes in round 1 and never again. A second fight in the same hex
        // (the step run again) has guns only.
        let hex = Hex::new(5, 0);
        let scene = day_scene(vec![
            ForceSpec::new(1, Side::Japanese, hex, &[1]),
            ForceSpec::new(2, Side::Allied, hex, &[1]),
        ]);
        let mut world = run(&scene, harmless);
        for firer in [1, 2] {
            let torpedoes = torpedo_salvos_of(&world, firer);
            assert_eq!(torpedoes.len(), 1);
            assert_eq!(
                (torpedoes[0].round, torpedoes[0].shots),
                (1, Fixed::from_int(8))
            );
            assert!(!ship(&world, firer).torpedoes_ready);
        }
        let first_fight = world.events().len();
        scene.run_step(&mut world);
        assert!(
            world.events().len() > first_fight,
            "a second fight happened"
        );
        for firer in [1, 2] {
            assert_eq!(torpedo_salvos_of(&world, firer).len(), 1);
            // Guns: 6 salvos of 120 in the first fight use 720 of 800 rounds,
            // so the second fight has one salvo of the 80 left and then none.
            let guns = gun_salvos_of(&world, firer);
            assert_eq!(guns.len(), 7);
            assert_eq!(guns[6].shots, Fixed::from_int(80));
        }
        assert_eq!(
            count(&world, |k| matches!(k, EventKind::EngagementBegun { .. })),
            2
        );
    }

    // ---- 10b. the other side's torpedo tubes (decision 0083) ----
    //
    // Class 8 is class 1 with 8 tubes of which 4 bear on one side (B 4, T 8).
    // Class 9 is a Mahan-like destroyer (T 12, B 8). Class 1 has B equal to T.
    // Round 1's first side is B tubes; the next round in which the ship has
    // torpedo shots she fires T - B, and then her torpedoes are spent.

    /// The listed ships fire nothing: no gun rounds, no torpedoes.
    fn silent(world: &mut World, ships: &[u32]) {
        for id in ships {
            let ship = ship_mut(world, *id);
            ship.gun_rounds = Fixed::ZERO;
            ship.torpedoes_ready = false;
        }
    }

    /// (round, shots) of each torpedo salvo of one ship.
    fn torpedo_rounds_of(world: &World, ship: u32) -> Vec<(u32, Fixed)> {
        torpedo_salvos_of(world, ship)
            .iter()
            .map(|s| (s.round, s.shots))
            .collect()
    }

    #[test]
    fn a_ship_with_wing_tubes_fires_b_and_then_t_minus_b_in_the_next_round() {
        // Case A, shots only (harmless: every torpedo misses so the fight
        // runs on): round 1 4 tubes, round 2 4 tubes, then none. 4 x 1 x 1 = 4
        // shots each.
        let hex = Hex::new(5, 0);
        let scene = day_scene(vec![
            ForceSpec::new(1, Side::Japanese, hex, &[8]),
            ForceSpec::new(2, Side::Allied, hex, &[1]),
        ]);
        let world = run(&scene, harmless);
        assert_eq!(
            torpedo_rounds_of(&world, 1),
            vec![(1, Fixed::from_int(4)), (2, Fixed::from_int(4))]
        );
        assert!(!ship(&world, 1).torpedoes_ready);
        assert!(!ship(&world, 1).torpedoes_one_side_fired);
        // The class 1 ship fires its 8 tubes once.
        assert_eq!(torpedo_rounds_of(&world, 2), vec![(1, Fixed::from_int(8))]);
        assert!(!ship(&world, 2).torpedoes_one_side_fired);
    }

    #[test]
    fn the_first_side_leaves_the_torpedoes_ready_and_the_flag_set() {
        // A fight that ends after round 1 (the silent allied force breaks
        // off, having been fired on) leaves the ship with one side fired.
        let hex = Hex::new(5, 0);
        let scene = day_scene(vec![
            ForceSpec::new(1, Side::Japanese, hex, &[8]),
            ForceSpec::new(2, Side::Allied, hex, &[1, 1]).stance(Stance::Avoid),
        ]);
        let world = run(&scene, |w| {
            harmless(w);
            silent(w, &[2, 3]);
        });
        assert_eq!(endings(&world), vec![(1, EngagementEndReason::BrokeOff)]);
        assert_eq!(torpedo_salvos_of(&world, 1).len(), 1);
        assert!(ship(&world, 1).torpedoes_ready);
        assert!(ship(&world, 1).torpedoes_one_side_fired);
        let violations = invariants::check(world.umpire(), world.map(), world.ref_data());
        assert!(violations.is_empty(), "{violations:?}");
    }

    #[test]
    fn case_b_the_second_side_goes_into_a_second_fight_from_its_own_stream() {
        // The first fight ends after round 1 (as above). A second fight in the
        // same pulse: round 1 is her first firing round, so she fires the
        // other 4 tubes at the target of decision 0078 (the most important
        // kind afloat, the one in place 0 of 2: ship 2), from the stream
        // keyed on `TorpedoOtherSide`, her and force 2. The hit chance 0.125
        // is set for the second fight only: 4 x 0.125 = 0.5 expected.
        let hex = Hex::new(5, 0);
        let scene = day_scene(vec![
            ForceSpec::new(1, Side::Japanese, hex, &[8]),
            ForceSpec::new(2, Side::Allied, hex, &[1, 1]).stance(Stance::Avoid),
        ]);
        let mut world = run(&scene, |w| {
            harmless(w);
            silent(w, &[2, 3]);
        });
        let before = world.events().len();
        figures_mut(&mut world).torpedo_hit_chance_day = fx("0.125");
        scene.run_step(&mut world);
        assert!(world.events().len() > before, "a second fight happened");

        let torpedoes = torpedo_salvos_of(&world, 1);
        assert_eq!(torpedoes.len(), 2);
        let second = &torpedoes[1];
        assert_eq!(second.shots, Fixed::from_int(4));
        assert_eq!(second.target, 2);
        assert_eq!(second.chance, fx("0.125"));
        assert_eq!(second.expected, fx("0.5"));
        assert!(!ship(&world, 1).torpedoes_ready);
        assert!(!ship(&world, 1).torpedoes_one_side_fired);

        let figures = world.ref_data.surface_combat.clone().unwrap();
        let mut stream = world.rng_pair(
            1,
            Some(Pulse::Morning),
            Subsystem::SurfaceCombat,
            Draw::TorpedoOtherSide,
            EntityRef::Ship(ShipId::new(1)),
            EntityRef::TaskForce(TaskForceId::new(2)),
        );
        let hits = replay_shots(&mut stream, 4, fx("0.125"));
        let systems = replay_torpedo_hits(&mut stream, &figures, hits);
        assert_eq!(second.hits, hits);
        let (tracks, recorded) = outcome_of_salvo(&world, second.id);
        assert_eq!(tracks, [hits, 0]);
        assert_eq!(recorded, systems);
    }

    #[test]
    fn case_d_prime_mounts_out_at_the_second_sides_turn_fire_nothing_and_change_nothing() {
        // State after the first side: ready and the flag set. Mounts out:
        // no salvo, so no event and no change; the side waits.
        let hex = Hex::new(5, 0);
        let scene = day_scene(vec![
            ForceSpec::new(1, Side::Japanese, hex, &[8]),
            ForceSpec::new(2, Side::Allied, hex, &[1]),
        ]);
        let world = run(&scene, |w| {
            harmless(w);
            let ship = ship_mut(w, 1);
            ship.torpedoes_one_side_fired = true;
            ship.systems.torpedo_mounts = SystemState::Out;
        });
        assert!(torpedo_salvos_of(&world, 1).is_empty());
        assert!(ship(&world, 1).torpedoes_ready);
        assert!(ship(&world, 1).torpedoes_one_side_fired);
        // Her guns were not touched.
        assert_eq!(gun_salvos_of(&world, 1).len(), 6);
        // Mounts out before the first side: the same, flag clear.
        let world = run(&scene, |w| {
            harmless(w);
            ship_mut(w, 1).systems.torpedo_mounts = SystemState::Out;
        });
        assert!(torpedo_salvos_of(&world, 1).is_empty());
        assert!(ship(&world, 1).torpedoes_ready);
        assert!(!ship(&world, 1).torpedoes_one_side_fired);
    }

    #[test]
    fn case_e_a_ship_sunk_in_round_1_fires_its_first_side_and_no_second() {
        // Ship 2 (class 8) starts at flotation 95 and sinks to the 30 points
        // of ship 1's gun salvo, still firing its first 4 tubes in round 1.
        // The fight ends with its force sunk, so no second salvo.
        let hex = Hex::new(5, 0);
        let scene = day_scene(vec![
            ForceSpec::new(1, Side::Japanese, hex, &[1]),
            ForceSpec::new(2, Side::Allied, hex, &[8]),
        ]);
        let world = run(&scene, |w| {
            flotation_only(w);
            ship_mut(w, 2).flotation_damage = Fixed::from_int(95);
        });
        assert_eq!(endings(&world), vec![(1, EngagementEndReason::ForceSunk)]);
        assert_eq!(torpedo_rounds_of(&world, 2), vec![(1, Fixed::from_int(4))]);
        assert_eq!(ship(&world, 2).location, ShipLocation::Sunk);
        assert!(ship(&world, 2).torpedoes_one_side_fired);
        let violations = invariants::check(world.umpire(), world.map(), world.ref_data());
        assert!(violations.is_empty(), "{violations:?}");
    }

    #[test]
    fn case_g_with_no_gun_ammunition_the_second_side_still_fires() {
        let hex = Hex::new(5, 0);
        let scene = day_scene(vec![
            ForceSpec::new(1, Side::Japanese, hex, &[8]),
            ForceSpec::new(2, Side::Allied, hex, &[1]),
        ]);
        let world = run(&scene, |w| {
            harmless(w);
            ship_mut(w, 1).gun_rounds = Fixed::ZERO;
        });
        assert!(gun_salvos_of(&world, 1).is_empty());
        assert_eq!(
            torpedo_rounds_of(&world, 1),
            vec![(1, Fixed::from_int(4)), (2, Fixed::from_int(4))]
        );
    }

    #[test]
    fn a_ship_with_only_its_second_side_left_counts_for_the_surprise_test() {
        // Night, force 1 sights and force 2 (not alert) does not. Ship 1 has
        // no gun rounds; its only salvo is the second side's 4 tubes. That
        // makes force 2 surprised; with the torpedoes spent it is not.
        let hex = Hex::new(5, 0);
        let scene = night_scene(vec![
            ForceSpec::new(1, Side::Japanese, hex, &[8]),
            ForceSpec::new(2, Side::Allied, hex, &[1]).not_alert(),
        ]);
        let surprised = |world: &World| {
            count(world, |k| {
                matches!(k, EventKind::ForceSurprised { round: 1, .. })
            })
        };
        let world = run(&scene, |w| {
            only_force_1_sights_by_roll(w);
            let ship = ship_mut(w, 1);
            ship.gun_rounds = Fixed::ZERO;
            ship.torpedoes_one_side_fired = true;
        });
        assert_eq!(surprised(&world), 1);
        assert_eq!(torpedo_salvos_of(&world, 1)[0].shots, Fixed::from_int(4));
        let world = run(&scene, |w| {
            only_force_1_sights_by_roll(w);
            let ship = ship_mut(w, 1);
            ship.gun_rounds = Fixed::ZERO;
            ship.torpedoes_ready = false;
        });
        assert_eq!(surprised(&world), 0);
    }

    #[test]
    fn case_f_a_class_with_b_equal_to_t_fires_one_salvo_and_never_sets_the_flag() {
        let hex = Hex::new(5, 0);
        let scene = day_scene(vec![
            ForceSpec::new(1, Side::Japanese, hex, &[1]),
            ForceSpec::new(2, Side::Allied, hex, &[1]),
        ]);
        let world = run(&scene, harmless);
        for firer in [1, 2] {
            assert_eq!(
                torpedo_rounds_of(&world, firer),
                vec![(1, Fixed::from_int(8))]
            );
            assert!(!ship(&world, firer).torpedoes_ready);
            assert!(!ship(&world, firer).torpedoes_one_side_fired);
        }
    }

    #[test]
    fn a_mahan_like_class_fires_8_and_then_4() {
        let hex = Hex::new(5, 0);
        let scene = day_scene(vec![
            ForceSpec::new(1, Side::Japanese, hex, &[9]),
            ForceSpec::new(2, Side::Allied, hex, &[1]),
        ]);
        let world = run(&scene, harmless);
        assert_eq!(
            torpedo_rounds_of(&world, 1),
            vec![(1, Fixed::from_int(8)), (2, Fixed::from_int(4))]
        );
        assert!(!ship(&world, 1).torpedoes_ready);
        assert!(!ship(&world, 1).torpedoes_one_side_fired);
    }

    #[test]
    fn a_degraded_mount_spends_a_side_with_the_shots_it_gave() {
        // Case D: degraded mounts give 4 x 0.5 = 2 shots in each salvo; the
        // two tubes of the side that did not fire are not kept.
        let hex = Hex::new(5, 0);
        let scene = day_scene(vec![
            ForceSpec::new(1, Side::Japanese, hex, &[8]),
            ForceSpec::new(2, Side::Allied, hex, &[1]),
        ]);
        let world = run(&scene, |w| {
            harmless(w);
            ship_mut(w, 1).systems.torpedo_mounts = SystemState::Degraded;
        });
        assert_eq!(
            torpedo_rounds_of(&world, 1),
            vec![(1, Fixed::from_int(2)), (2, Fixed::from_int(2))]
        );
        assert!(!ship(&world, 1).torpedoes_ready);
    }

    // ---- 11. three forces in one hex ----

    fn begun_pairs(world: &World) -> Vec<(u32, u32)> {
        world
            .events()
            .iter()
            .filter_map(|event| match event.kind {
                EventKind::EngagementBegun { a, b, .. } => Some((a.raw(), b.raw())),
                _ => None,
            })
            .collect()
    }

    #[test]
    fn three_forces_in_one_hex_fight_as_ordered_pairs() {
        // Force 1 (Japanese), 2 and 3 (Allied) in one hex: the opposing
        // pairs are (1,2) and (1,3), not (2,3), in that order. Each fight
        // ends before the next begins.
        let hex = Hex::new(5, 0);
        let scene = day_scene(vec![
            ForceSpec::new(1, Side::Japanese, hex, &[1]),
            ForceSpec::new(2, Side::Allied, hex, &[1]),
            ForceSpec::new(3, Side::Allied, hex, &[1]),
        ]);
        let world = run(&scene, harmless);
        assert_eq!(begun_pairs(&world), vec![(1, 2), (1, 3)]);
        let order: Vec<&str> = world
            .events()
            .iter()
            .filter_map(|event| match event.kind {
                EventKind::EngagementBegun { .. } => Some("begun"),
                EventKind::EngagementEnded { .. } => Some("ended"),
                _ => None,
            })
            .collect();
        assert_eq!(order, vec!["begun", "ended", "begun", "ended"]);
    }

    #[test]
    fn a_later_pair_is_skipped_if_an_earlier_fight_sank_a_force() {
        // Force 1's only ship has flotation 95 and sinks in the first fight
        // (30 points from force 2's salvo). Pair (1,3) comes up with force 1
        // empty and is skipped.
        let hex = Hex::new(5, 0);
        let scene = day_scene(vec![
            ForceSpec::new(1, Side::Japanese, hex, &[1]),
            ForceSpec::new(2, Side::Allied, hex, &[1]),
            ForceSpec::new(3, Side::Allied, hex, &[1]),
        ]);
        let world = run(&scene, |w| {
            flotation_only(w);
            ship_mut(w, 1).flotation_damage = Fixed::from_int(95);
        });
        assert_eq!(begun_pairs(&world), vec![(1, 2)]);
        assert_eq!(endings(&world), vec![(1, EngagementEndReason::ForceSunk)]);
        assert!(salvos(&world).iter().all(|s| s.target != 3));
    }

    // ---- 13. breakdowns do not change the hash, with a fight in the log ----

    #[test]
    fn breakdowns_leave_the_hash_alone_for_a_fight_with_salvos_damage_and_a_sinking() {
        // Day, both sight in round 1; ship 4 sinks. The log has every kind of
        // fight event, and the hash agrees with breakdowns on and off.
        let hex = Hex::new(5, 0);
        let scene = day_scene(vec![
            ForceSpec::new(1, Side::Japanese, hex, &[1, 1]),
            ForceSpec::new(2, Side::Allied, hex, &[1, 1]),
        ]);
        let setup = |record: bool| {
            run(&scene, move |w| {
                flotation_only(w);
                ship_mut(w, 3).flotation_damage = Fixed::from_int(95);
                w.set_config(SimConfig {
                    record_breakdowns: record,
                });
            })
        };
        let with = setup(true);
        let without = setup(false);
        assert_eq!(with.state_hash(), without.state_hash());
        assert_eq!(with.events().len(), without.events().len());
        for (kind, name) in [
            (
                count(&with, |k| matches!(k, EventKind::Salvo { .. })),
                "salvo",
            ),
            (
                count(&with, |k| matches!(k, EventKind::ShipDamaged { .. })),
                "damage",
            ),
            (
                count(&with, |k| matches!(k, EventKind::ShipSunk { .. })),
                "sunk",
            ),
            (
                count(&with, |k| matches!(k, EventKind::ClaimReceived { .. })),
                "claim",
            ),
            (
                count(&with, |k| matches!(k, EventKind::ReportReceived { .. })),
                "report",
            ),
        ] {
            assert!(kind > 0, "the log has {name} events");
        }
        for (a, b) in with.events().iter().zip(without.events().iter()) {
            assert_eq!(a.kind, b.kind);
            assert_eq!(a.causes, b.causes);
            assert!(b.breakdown.is_none());
        }
    }

    #[test]
    fn breakdowns_leave_the_hash_alone_for_a_fight_with_system_hits() {
        // Day, every gun hit may knock a system down (chance 1, spread over
        // all four) and so may every torpedo hit. The log has system hits,
        // and the hash and events agree with breakdowns on and off.
        let hex = Hex::new(5, 0);
        let scene = day_scene(vec![
            ForceSpec::new(1, Side::Japanese, hex, &[1, 1]),
            ForceSpec::new(2, Side::Allied, hex, &[1, 1]),
        ]);
        let setup = |record: bool| {
            run(&scene, move |w| {
                let figures = figures_mut(w);
                figures.system_hit_chance_per_point = Fixed::from_int(100);
                figures.torpedo_hit_chance_day = fx("0.125");
                w.set_config(SimConfig {
                    record_breakdowns: record,
                });
            })
        };
        let with = setup(true);
        let without = setup(false);
        assert!(count(&with, |k| matches!(k, EventKind::SystemHit { .. })) > 0);
        assert_eq!(with.state_hash(), without.state_hash());
        assert_eq!(with.events().len(), without.events().len());
        for (a, b) in with.events().iter().zip(without.events().iter()) {
            assert_eq!(a.kind, b.kind);
            assert_eq!(a.causes, b.causes);
            assert!(b.breakdown.is_none());
        }
        let violations = invariants::check(with.umpire(), with.map(), with.ref_data());
        assert!(violations.is_empty(), "{violations:?}");
    }

    // ---- 14. the claim counts burning ships ----

    fn claim_of(world: &World, side: Side) -> u32 {
        let claims = world.beliefs[&side].claims();
        assert_eq!(claims.len(), 1, "one fight, one claim");
        claims[0].claimed_sunk
    }

    #[test]
    fn a_claim_counts_enemy_ships_afloat_with_fire_damage_of_30_or_more() {
        // Day, both sight in round 1, and every hit misses (gun and torpedo
        // chances 0), so fire damage stays as set. Force 2's ships have fire
        // damage 30 (counted) and 100 (counted): force 1 claims 2. Force 1's
        // ships have 29.999999 (not counted) and 0: force 2 claims 0.
        let hex = Hex::new(5, 0);
        let scene = day_scene(vec![
            ForceSpec::new(1, Side::Japanese, hex, &[1, 1]),
            ForceSpec::new(2, Side::Allied, hex, &[1, 1]),
        ]);
        let world = run(&scene, |w| {
            let figures = figures_mut(w);
            figures.gun_hit_chance_day = Fixed::ZERO;
            figures.torpedo_hit_chance_day = Fixed::ZERO;
            ship_mut(w, 1).fire_damage = Fixed::from_raw(29_999_999);
            ship_mut(w, 3).fire_damage = Fixed::from_int(30);
            ship_mut(w, 4).fire_damage = Fixed::from_int(100);
        });
        assert_eq!(claim_of(&world, Side::Japanese), 2);
        assert_eq!(claim_of(&world, Side::Allied), 0);
    }

    #[test]
    fn a_claim_counts_ships_sunk_in_the_fight_once_and_burning_ones_besides() {
        // Day, gun hits all on flotation. Ship 3 (flotation 95, fire 50) sinks
        // to force 1's first salvo; it left the force, so it is counted once
        // as sunk and not again as burning. Ship 4 afloat has fire 40:
        // counted. Force 1 claims 1 + 1 = 2. Force 2's salvo does not sink
        // anything (30 points on ships at 0).
        let hex = Hex::new(5, 0);
        let scene = day_scene(vec![
            ForceSpec::new(1, Side::Japanese, hex, &[1, 1]),
            ForceSpec::new(2, Side::Allied, hex, &[1, 1]).break_off(Fixed::ONE),
        ]);
        let world = run(&scene, |w| {
            flotation_only(w);
            ship_mut(w, 3).flotation_damage = Fixed::from_int(95);
            ship_mut(w, 3).fire_damage = Fixed::from_int(50);
            ship_mut(w, 4).fire_damage = Fixed::from_int(40);
        });
        assert_eq!(claim_of(&world, Side::Japanese), 2);
        assert_eq!(claim_of(&world, Side::Allied), 0);
        let claims = world.beliefs[&Side::Japanese].claims();
        assert_eq!(claims[0].hex, hex);
        assert_eq!(claims[0].day, 1);
        assert_eq!(claims[0].pulse, Pulse::Morning);
    }

    // ---- 15. reports and claims after a fight ----

    #[test]
    fn a_fight_report_cites_the_end_of_the_engagement_and_a_force_that_never_sighted_sends_none() {
        // The slipped-away fight of test 3: only force 1 sighted. One report,
        // from the Japanese side, citing EngagementEnded, creating a contact.
        let hex = Hex::new(5, 0);
        let scene = night_scene(vec![
            ForceSpec::new(1, Side::Japanese, hex, &[1]).stance(Stance::Avoid),
            ForceSpec::new(2, Side::Allied, hex, &[1]).not_alert(),
        ]);
        let world = run(&scene, only_force_1_sights_by_roll);
        let ended = world
            .events()
            .iter()
            .find(|e| matches!(e.kind, EventKind::EngagementEnded { .. }))
            .expect("the fight ended")
            .id;
        let reports = events_of(&world, |k| matches!(k, EventKind::ReportReceived { .. }));
        assert_eq!(reports.len(), 1);
        assert!(matches!(
            reports[0].kind,
            EventKind::ReportReceived {
                side: Side::Japanese,
                created: true,
                ..
            }
        ));
        assert_eq!(reports[0].causes, vec![Cause::Event(ended)]);
        assert_eq!(world.beliefs[&Side::Japanese].contacts().count(), 1);
        assert_eq!(world.beliefs[&Side::Allied].contacts().count(), 0);
    }

    #[test]
    fn after_a_fight_each_force_reports_then_claims_lower_force_first() {
        // Day, both sight and fire: Japanese report, Japanese claim, Allied
        // report, Allied claim, all citing EngagementEnded.
        let hex = Hex::new(5, 0);
        let scene = day_scene(vec![
            ForceSpec::new(1, Side::Japanese, hex, &[1]),
            ForceSpec::new(2, Side::Allied, hex, &[1]),
        ]);
        let world = run(&scene, harmless);
        let ended = world
            .events()
            .iter()
            .find(|e| matches!(e.kind, EventKind::EngagementEnded { .. }))
            .expect("the fight ended")
            .id;
        let after: Vec<(Side, bool, Vec<Cause>)> = world
            .events()
            .iter()
            .filter_map(|event| match event.kind {
                EventKind::ReportReceived { side, .. } => Some((side, true, event.causes.clone())),
                EventKind::ClaimReceived { side, .. } => Some((side, false, event.causes.clone())),
                _ => None,
            })
            .collect();
        let cause = vec![Cause::Event(ended)];
        assert_eq!(
            after,
            vec![
                (Side::Japanese, true, cause.clone()),
                (Side::Japanese, false, cause.clone()),
                (Side::Allied, true, cause.clone()),
                (Side::Allied, false, cause),
            ]
        );
    }

    // ---- 16. the fire stream draw order ----

    /// Replays the shot draws of a salvo of a whole number of `shots` at
    /// `chance` (decision 0033): one draw each. Returns the hits.
    fn replay_shots(stream: &mut RngStream, shots: u32, chance: Fixed) -> u32 {
        (0..shots).map(|_| u32::from(stream.chance(chance))).sum()
    }

    /// Replays the system draws of `hits` torpedo hits (one each, no track
    /// draw) for a hit of 50 points, and returns the systems hit.
    fn replay_torpedo_hits(
        stream: &mut RngStream,
        figures: &SurfaceCombatFigures,
        hits: u32,
    ) -> Vec<ShipSystem> {
        let chance = system_hit_chance(Fixed::from_int(50), figures.system_hit_chance_per_point);
        let mut systems = Vec::new();
        for _ in 0..hits {
            let draw = Fixed::from_raw(stream.below(Fixed::SCALE.unsigned_abs()) as i64);
            systems.extend(system_for_draw(draw, chance, &figures.torpedo_system_hit));
        }
        systems
    }

    /// Replays `hits` gun hits from `stream` as the fire rule draws them: for
    /// each, a track draw (flotation if below the flotation share, else
    /// fire), then a system draw. Returns the hits on flotation and fire and
    /// the systems hit, in order.
    fn replay_gun_hits(
        stream: &mut RngStream,
        figures: &SurfaceCombatFigures,
        points: Fixed,
        hits: u32,
    ) -> ([u32; 2], Vec<ShipSystem>) {
        let flotation = u64::try_from(figures.gun_hit_flotation.raw()).unwrap();
        let mut tracks = [0u32; 2];
        let mut systems = Vec::new();
        for _ in 0..hits {
            let draw = stream.below(Fixed::SCALE.unsigned_abs());
            tracks[usize::from(draw >= flotation)] += 1;
            let draw = Fixed::from_raw(stream.below(Fixed::SCALE.unsigned_abs()) as i64);
            systems.extend(system_for_draw(
                draw,
                system_hit_chance(points, figures.system_hit_chance_per_point),
                &figures.gun_system_hit,
            ));
        }
        (tracks, systems)
    }

    /// The track hits and system hits that cite one salvo.
    fn outcome_of_salvo(world: &World, salvo: EventId) -> ([u32; 2], Vec<ShipSystem>) {
        let mut tracks = [0u32; 2];
        let mut systems = Vec::new();
        for event in world.events().iter() {
            if event.causes != vec![Cause::Event(salvo)] {
                continue;
            }
            match event.kind {
                EventKind::ShipDamaged { track, hits, .. } => tracks[track as usize] += hits,
                EventKind::SystemHit { system, .. } => systems.push(system),
                _ => {}
            }
        }
        (tracks, systems)
    }

    /// Ship 2 fires nothing, so ship 1 is never hit and its own shots stay
    /// the same in every round.
    fn only_ship_1_fires(world: &mut World) {
        let target = ship_mut(world, 2);
        target.gun_rounds = Fixed::ZERO;
        target.torpedoes_ready = false;
    }

    fn ship_1_stream(world: &World) -> RngStream {
        world.rng_pair(
            1,
            Some(Pulse::Morning),
            Subsystem::SurfaceCombat,
            Draw::Legacy,
            EntityRef::Ship(ShipId::new(1)),
            EntityRef::TaskForce(TaskForceId::new(2)),
        )
    }

    #[test]
    fn the_fire_stream_draws_each_shot_then_track_and_system_per_gun_hit_then_the_torpedo() {
        // Day, one cruiser each, ship 2 silent. Ship 1, round 1, from the
        // stream keyed on ship 1 and task force 2 (decision 0033):
        //   120 gun shots at 0.05 (6 expected, a whole number of shots, so
        //   no draw for a part): 120 draws, n of them hits
        //   n gun hits, each a track draw then a system draw: 2n draws
        //   8 torpedo shots at 0.125 (1 expected): 8 draws, m of them hits
        //   m torpedo hits: no track draw, one system draw each
        // A torpedo hit is on flotation whatever the stream says.
        let world = run(&duel(), |w| {
            only_ship_1_fires(w);
            figures_mut(w).torpedo_hit_chance_day = fx("0.125");
        });
        let figures = world.ref_data.surface_combat.clone().unwrap();
        let mut stream = ship_1_stream(&world);

        let n = replay_shots(&mut stream, 120, fx("0.05"));
        let (tracks, systems) = replay_gun_hits(&mut stream, &figures, Fixed::from_int(5), n);
        let gun = first_round_salvo(&world, 1, Weapon::Gun);
        assert_eq!(gun.expected, Fixed::from_int(6));
        assert_eq!(gun.hits, n);
        assert_eq!(outcome_of_salvo(&world, gun.id), (tracks, systems));

        let m = replay_shots(&mut stream, 8, fx("0.125"));
        let torpedo_systems = replay_torpedo_hits(&mut stream, &figures, m);
        let torpedo = first_round_salvo(&world, 1, Weapon::Torpedo);
        assert_eq!(torpedo.expected, Fixed::ONE);
        assert_eq!(torpedo.hits, m);
        let (torpedo_tracks, recorded_systems) = outcome_of_salvo(&world, torpedo.id);
        assert_eq!(torpedo_tracks, [m, 0]);
        assert_eq!(recorded_systems, torpedo_systems);
    }

    #[test]
    fn the_fire_stream_carries_on_in_the_next_round_one_draw_per_shot() {
        // Day, one cruiser each, ship 2 silent, ship 1's torpedoes gone and
        // every gun hit forced onto fire control with chance 0.5 (so some
        // system draws miss). Each round: 120 shot draws at 0.05, then n x
        // (track, system). Replaying the stream by hand over two rounds must
        // give the same hits and system hits: it fails if a shot draw or
        // either draw per hit is skipped.
        let world = run(&duel(), |w| {
            only_ship_1_fires(w);
            ship_mut(w, 1).torpedoes_ready = false;
            let figures = figures_mut(w);
            // 5 points x 0.1 = chance 0.5.
            figures.system_hit_chance_per_point = fx("0.1");
            figures.gun_system_hit = [(ShipSystem::FireControl, Fixed::ONE)]
                .into_iter()
                .collect();
        });
        let figures = world.ref_data.surface_combat.clone().unwrap();
        let mut stream = ship_1_stream(&world);
        for round in 1..=2 {
            let n = replay_shots(&mut stream, 120, fx("0.05"));
            let (tracks, systems) = replay_gun_hits(&mut stream, &figures, Fixed::from_int(5), n);
            let salvo = gun_salvos_of(&world, 1)
                .into_iter()
                .find(|s| s.round == round)
                .expect("a gun salvo this round");
            assert_eq!(salvo.expected, Fixed::from_int(6));
            assert_eq!(salvo.hits, n);
            assert_eq!(
                outcome_of_salvo(&world, salvo.id),
                (tracks, systems),
                "round {round}"
            );
        }
    }

    #[test]
    fn the_draws_are_the_same_whatever_the_chance_of_a_system_hit() {
        // Decision 0033 changes the width of a band, never the number or
        // order of draws: with the figure at 0 (chance 0), 0.06 (a gun hit
        // 0.3, a torpedo hit 1) and 100 (1 for both), the stream replays
        // the same way and the system hits follow the replay.
        for per_point in ["0", "0.06", "0.2", "100"] {
            let world = run(&duel(), |w| {
                only_ship_1_fires(w);
                figures_mut(w).system_hit_chance_per_point = fx(per_point);
                figures_mut(w).torpedo_hit_chance_day = fx("0.125");
            });
            let figures = world.ref_data.surface_combat.clone().unwrap();
            let mut stream = ship_1_stream(&world);
            let n = replay_shots(&mut stream, 120, fx("0.05"));
            let (tracks, systems) = replay_gun_hits(&mut stream, &figures, Fixed::from_int(5), n);
            let gun = first_round_salvo(&world, 1, Weapon::Gun);
            assert_eq!(
                outcome_of_salvo(&world, gun.id),
                (tracks, systems),
                "figure {per_point}"
            );
            let m = replay_shots(&mut stream, 8, fx("0.125"));
            let torpedo = first_round_salvo(&world, 1, Weapon::Torpedo);
            assert_eq!(torpedo.hits, m, "figure {per_point}");
            let (_, torpedo_systems) = outcome_of_salvo(&world, torpedo.id);
            assert_eq!(
                torpedo_systems,
                replay_torpedo_hits(&mut stream, &figures, m),
                "figure {per_point}"
            );
        }
    }

    #[test]
    fn one_figure_per_point_serves_gun_and_torpedo_hits() {
        // Placeholder cruiser: a gun hit is 5 points (chance 0.3), a torpedo
        // hit 50 (chance 1). Over many seeds each torpedo hit always knocks a
        // system down and the gun hit does not.
        let mut torpedo_hits = 0u32;
        let mut gun_hits = 0u32;
        let mut gun_system_hits = 0usize;
        for seed in 1..=20 {
            let mut scene = duel();
            scene.seed = seed;
            let world = run(&scene, |w| {
                only_ship_1_fires(w);
                figures_mut(w).torpedo_hit_chance_day = fx("0.125");
            });
            let torpedo = first_round_salvo(&world, 1, Weapon::Torpedo);
            let (_, torpedo_systems) = outcome_of_salvo(&world, torpedo.id);
            assert_eq!(
                torpedo_systems.len(),
                usize::try_from(torpedo.hits).unwrap(),
                "seed {seed}"
            );
            torpedo_hits += torpedo.hits;
            let gun = first_round_salvo(&world, 1, Weapon::Gun);
            gun_hits += gun.hits;
            gun_system_hits += outcome_of_salvo(&world, gun.id).1.len();
        }
        assert!(torpedo_hits > 0);
        // 120 gun hits at 0.3 is about 36, never all and never none.
        assert!(gun_system_hits > 0 && gun_system_hits < usize::try_from(gun_hits).unwrap());
    }

    // ---- alertness and alarm (decision 0021) ----
    //
    // Alert from belief runs at the start of a pulse (`alert_from_belief`);
    // events during the pulse and the alarm run in `surface_engagements`.
    // The placeholder figures are: contact distance 3 hexes, contact age 2
    // pulses, alarm chance 0.5. A contact enters belief the way a report
    // does, through `apply_report`, after a `ReportReceived` event.

    use crate::belief::SightingReport;

    /// Puts a contact into one side's belief and returns it with the event
    /// that recorded the report's arrival.
    fn give_contact(world: &mut World, side: Side, hex: Hex) -> (ContactId, EventId) {
        let figures = world.ref_data.search.clone().expect("search figures");
        let received = world.events.next_id();
        let report = Report::Sighting(SightingReport {
            estimated_hex: hex,
            estimated_ship_count: 1,
            estimated_kind: ShipKind::Cruiser,
            received_event: received,
        });
        let ApplyOutcome::Contact(outcome) = world
            .beliefs
            .get_mut(&side)
            .expect("belief")
            .apply_report(&report, &figures)
        else {
            unreachable!("a sighting makes a contact");
        };
        let event = world.events.record(
            1,
            Some(Pulse::Night),
            EventKind::ReportReceived {
                side,
                contact: outcome.contact,
                hex,
                ship_count: 1,
                kind: ShipKind::Cruiser,
                created: outcome.created,
            },
            Vec::new(),
            None,
        );
        assert_eq!(event, received);
        (outcome.contact, event)
    }

    /// Ages every contact by `pulses` pulses. The first call only clears the
    /// "refreshed this pulse" mark of contacts just made, so it takes one more
    /// call than the age wanted. A contact is still held at age 4 and dropped
    /// at age 5.
    fn age_contacts_by(world: &mut World, pulses: u32) {
        let figures = world.ref_data.search.clone().expect("search figures");
        for _ in 0..=pulses {
            for belief in world.beliefs.values_mut() {
                belief.age_contacts(&figures);
            }
        }
    }

    fn force(world: &World, id: u32) -> &TaskForce {
        &world.truth.task_forces[&TaskForceId::new(id)]
    }

    fn force_mut(world: &mut World, id: u32) -> &mut TaskForce {
        world
            .truth
            .task_forces
            .get_mut(&TaskForceId::new(id))
            .expect("force exists")
    }

    /// One Allied cruiser force (task force 1, normal readiness) at (0,0)
    /// and nothing else.
    fn lone_allied() -> World {
        Scene::new(
            1,
            Pulse::Night,
            vec![ForceSpec::new(1, Side::Allied, Hex::new(0, 0), &[1]).not_alert()],
        )
        .world()
    }

    /// Whether force 1 is alerted at the start of a pulse by one Allied
    /// contact at `hex`, aged `age` pulses.
    fn alerted_by_contact_at(hex: Hex, age: u32) -> bool {
        let mut world = lone_allied();
        give_contact(&mut world, Side::Allied, hex);
        age_contacts_by(&mut world, age);
        world.alert_from_belief(1, Pulse::Night);
        force(&world, 1).alerted
    }

    fn alerts_by_contact(world: &World) -> Vec<Event> {
        events_of(world, |k| {
            matches!(k, EventKind::ForceAlertedByContact { .. })
        })
    }

    #[test]
    fn a_contact_at_the_start_of_a_pulse_alerts_a_force_citing_its_report() {
        // Allied force 1 (normal readiness) at (0,0), Allied force 2 at
        // battle readiness at (1,0). A fresh Allied contact at (2,0) is 2
        // hexes from force 1 and 1 from force 2: force 1 is alerted, and
        // force 2, at battle readiness, is not (differences.md row 2). The
        // event cites the report behind the contact, and alerted_by is the
        // event.
        let scene = Scene::new(
            1,
            Pulse::Night,
            vec![
                ForceSpec::new(1, Side::Allied, Hex::new(0, 0), &[1]).not_alert(),
                ForceSpec::new(2, Side::Allied, Hex::new(1, 0), &[1]),
            ],
        );
        let mut world = scene.world();
        let (contact, report) = give_contact(&mut world, Side::Allied, Hex::new(2, 0));
        world.alert_from_belief(1, Pulse::Night);
        let alerts = alerts_by_contact(&world);
        assert_eq!(alerts.len(), 1);
        for (alert, id) in alerts.iter().zip([1]) {
            assert_eq!(
                alert.kind,
                EventKind::ForceAlertedByContact {
                    task_force: TaskForceId::new(id),
                    contact
                }
            );
            assert_eq!(alert.causes, vec![Cause::Event(report)]);
            assert_eq!(alert.pulse, Some(Pulse::Night));
            assert!(force(&world, id).alerted);
            assert_eq!(force(&world, id).alerted_by, Some(alert.id));
            assert!(force(&world, id).is_alert());
        }
        // Force 2 is at battle readiness: not flagged, no event for it.
        assert!(!force(&world, 2).alerted);
        assert_eq!(force(&world, 2).alerted_by, None);
        assert!(!alerts.iter().any(|alert| matches!(
            alert.kind,
            EventKind::ForceAlertedByContact { task_force, .. } if task_force == TaskForceId::new(2)
        )));
        // Nothing but the alert and the report was recorded.
        assert_eq!(world.events().len(), 2);

        // Still alerted by the same contact: nothing happens, alerted_by
        // stays.
        world.alert_from_belief(1, Pulse::Morning);
        assert_eq!(world.events().len(), 2);
        assert_eq!(force(&world, 1).alerted_by, Some(alerts[0].id));
    }

    #[test]
    fn a_force_at_battle_readiness_is_neither_alerted_nor_stood_down_by_a_contact() {
        // Force 1 is at battle readiness (the default of a spec), with a
        // fresh contact in range: no alert event, flag stays clear.
        let mut world = Scene::new(
            1,
            Pulse::Night,
            vec![ForceSpec::new(1, Side::Allied, Hex::new(0, 0), &[1])],
        )
        .world();
        assert_eq!(force(&world, 1).readiness, Readiness::Battle);
        force_mut(&mut world, 1).alerted = false;
        give_contact(&mut world, Side::Allied, Hex::new(1, 0));
        world.alert_from_belief(1, Pulse::Night);
        assert!(alerts_by_contact(&world).is_empty());
        assert!(!force(&world, 1).alerted);
        assert_eq!(force(&world, 1).alerted_by, None);

        // With the flag already set and no contact left, it is not stood
        // down and the flag is untouched.
        let mut world = Scene::new(
            1,
            Pulse::Night,
            vec![ForceSpec::new(1, Side::Allied, Hex::new(0, 0), &[1])],
        )
        .world();
        force_mut(&mut world, 1).alerted = true;
        world.alert_from_belief(1, Pulse::Night);
        let stood = events_of(&world, |k| matches!(k, EventKind::ForceStoodDown { .. }));
        assert!(stood.is_empty());
        assert!(force(&world, 1).alerted);
    }

    #[test]
    fn a_force_stands_down_when_no_such_contact_remains_citing_what_alerted_it() {
        // Alerted at age 2 (the limit). Two pulses on, the contact is 4
        // pulses old: too old, so the force stands down. The event cites
        // alerted_by, and alerted_by is cleared.
        let mut world = lone_allied();
        give_contact(&mut world, Side::Allied, Hex::new(1, 0));
        age_contacts_by(&mut world, 2);
        world.alert_from_belief(1, Pulse::Night);
        let alerted_by = force(&world, 1).alerted_by.expect("alerted");
        assert!(force(&world, 1).alerted);

        age_contacts_by(&mut world, 2);
        world.alert_from_belief(1, Pulse::Morning);
        let stood = events_of(&world, |k| matches!(k, EventKind::ForceStoodDown { .. }));
        assert_eq!(stood.len(), 1);
        assert_eq!(
            stood[0].kind,
            EventKind::ForceStoodDown {
                task_force: TaskForceId::new(1)
            }
        );
        assert_eq!(stood[0].causes, vec![Cause::Event(alerted_by)]);
        assert_eq!(stood[0].pulse, Some(Pulse::Morning));
        assert!(!force(&world, 1).alerted);
        assert_eq!(force(&world, 1).alerted_by, None);

        // Not alerted and no contact: nothing more is recorded.
        let len = world.events().len();
        world.alert_from_belief(1, Pulse::Afternoon);
        assert_eq!(world.events().len(), len);
    }

    #[test]
    fn a_contact_exactly_at_the_distance_limit_alerts_and_one_further_does_not() {
        // Limit 3 hexes. From (0,0): (3,0) is 3, (2,1) is (2+1+3)/2 = 3 and
        // (-3,3) is (3+3+0)/2 = 3, all alert. (4,0) is 4 and (2,2) is
        // (2+2+4)/2 = 4, neither does.
        for hex in [Hex::new(3, 0), Hex::new(2, 1), Hex::new(-3, 3)] {
            assert_eq!(Hex::new(0, 0).distance(hex), 3);
            assert!(alerted_by_contact_at(hex, 0), "{hex:?} at 3 alerts");
        }
        for hex in [Hex::new(4, 0), Hex::new(2, 2)] {
            assert_eq!(Hex::new(0, 0).distance(hex), 4);
            assert!(!alerted_by_contact_at(hex, 0), "{hex:?} at 4 does not");
        }
    }

    #[test]
    fn a_contact_exactly_at_the_age_limit_alerts_and_one_pulse_older_does_not() {
        // Limit 2 pulses: age 0 and 2 alert, age 3 does not.
        assert!(alerted_by_contact_at(Hex::new(1, 0), 0));
        assert!(alerted_by_contact_at(Hex::new(1, 0), 2));
        assert!(!alerted_by_contact_at(Hex::new(1, 0), 3));
    }

    #[test]
    fn the_alert_reads_the_one_freshness_figure_the_merge_reads() {
        // The age limit for alerting is `fresh_contact_max_age_pulses` in
        // the search figures (decision 0032): moving it moves the alert.
        let alerted_with = |limit: u32, age: u32| {
            let mut world = lone_allied();
            world
                .ref_data
                .search
                .as_mut()
                .expect("search figures")
                .fresh_contact_max_age_pulses = limit;
            give_contact(&mut world, Side::Allied, Hex::new(1, 0));
            age_contacts_by(&mut world, age);
            world.alert_from_belief(1, Pulse::Night);
            force(&world, 1).alerted
        };
        assert!(alerted_with(2, 2));
        assert!(!alerted_with(2, 3));
        assert!(alerted_with(3, 3));
        assert!(!alerted_with(1, 2));
    }

    #[test]
    fn only_the_forces_own_sides_belief_alerts_it() {
        // A Japanese contact next to the Allied force alerts nobody: the
        // Allied force reads the Allied belief, which is empty.
        let mut world = lone_allied();
        give_contact(&mut world, Side::Japanese, Hex::new(1, 0));
        world.alert_from_belief(1, Pulse::Night);
        assert!(!force(&world, 1).alerted);
        assert!(alerts_by_contact(&world).is_empty());
    }

    #[test]
    fn a_force_without_ships_is_not_alerted_and_without_combat_figures_nothing_runs() {
        let mut world = lone_allied();
        give_contact(&mut world, Side::Allied, Hex::new(1, 0));
        force_mut(&mut world, 1).ships.clear();
        world.alert_from_belief(1, Pulse::Night);
        assert!(!force(&world, 1).alerted);

        let mut world = lone_allied();
        give_contact(&mut world, Side::Allied, Hex::new(1, 0));
        world.ref_data.surface_combat = None;
        world.alert_from_belief(1, Pulse::Night);
        assert!(!force(&world, 1).alerted);
        assert_eq!(world.events().len(), 1, "only the report");
    }

    #[test]
    fn the_nearest_contact_is_cited_and_an_equal_distance_goes_to_the_lower_id() {
        // Contact 1 at (3,0) is 3 from force 1, contact 2 at (-2,0) is 2:
        // the nearer one is cited though its ID is higher. They are 5 apart,
        // beyond the merge distance of 2, so they are two contacts.
        let mut world = lone_allied();
        let (far, _) = give_contact(&mut world, Side::Allied, Hex::new(3, 0));
        let (near, near_report) = give_contact(&mut world, Side::Allied, Hex::new(-2, 0));
        assert_ne!(far, near);
        world.alert_from_belief(1, Pulse::Night);
        let alert = &alerts_by_contact(&world)[0];
        assert_eq!(
            alert.kind,
            EventKind::ForceAlertedByContact {
                task_force: TaskForceId::new(1),
                contact: near
            }
        );
        assert_eq!(alert.causes, vec![Cause::Event(near_report)]);

        // Both 3 away (6 apart): the lower ID, whichever side it is on.
        for (first, second) in [
            (Hex::new(3, 0), Hex::new(-3, 0)),
            (Hex::new(-3, 0), Hex::new(3, 0)),
        ] {
            let mut world = lone_allied();
            let (lower, lower_report) = give_contact(&mut world, Side::Allied, first);
            let (higher, _) = give_contact(&mut world, Side::Allied, second);
            assert!(lower < higher);
            world.alert_from_belief(1, Pulse::Night);
            let alert = &alerts_by_contact(&world)[0];
            assert_eq!(
                alert.kind,
                EventKind::ForceAlertedByContact {
                    task_force: TaskForceId::new(1),
                    contact: lower
                }
            );
            assert_eq!(alert.causes, vec![Cause::Event(lower_report)]);
        }
    }

    #[test]
    fn belief_made_by_a_fight_alerts_nobody_until_the_next_pulse() {
        // Night. Japanese force 1 (battle readiness, sights for sure) fights
        // Allied force 2 at (0,0). Japanese force 3 (normal readiness) is at
        // (1,0): 1 hex from the fight, so the report (at most 1 hex out)
        // puts a contact within 2 of it. But force 3 is not in the fight
        // hex, so it cannot hear the alarm, and belief does not alert in the
        // middle of the pulse: it is not alerted after the fight. At the
        // start of the next pulse it is, citing the fight's report.
        let scene = night_scene(vec![
            ForceSpec::new(1, Side::Japanese, Hex::new(0, 0), &[1]),
            ForceSpec::new(2, Side::Allied, Hex::new(0, 0), &[1]).not_alert(),
            ForceSpec::new(3, Side::Japanese, Hex::new(1, 0), &[1]).not_alert(),
        ]);
        let mut world = run(&scene, |w| {
            only_force_1_sights_by_roll(w);
            harmless(w);
        });
        assert!(count(&world, |k| matches!(k, EventKind::Salvo { .. })) > 0);
        let reports = events_of(&world, |k| {
            matches!(
                k,
                EventKind::ReportReceived {
                    side: Side::Japanese,
                    ..
                }
            )
        });
        assert_eq!(reports.len(), 1);
        assert!(!force(&world, 3).alerted);
        assert_eq!(
            count(&world, |k| matches!(
                k,
                EventKind::ForceAlertedByContact { .. } | EventKind::AlarmRoll { .. }
            )),
            0
        );

        world.alert_from_belief(1, Pulse::Morning);
        let alerts: Vec<Event> = alerts_by_contact(&world)
            .into_iter()
            .filter(|e| {
                matches!(e.kind, EventKind::ForceAlertedByContact { task_force, .. } if task_force.raw() == 3)
            })
            .collect();
        assert_eq!(alerts.len(), 1);
        assert_eq!(alerts[0].causes, vec![Cause::Event(reports[0].id)]);
        assert_eq!(alerts[0].pulse, Some(Pulse::Morning));
        assert!(force(&world, 3).alerted);
        assert_eq!(force(&world, 3).alerted_by, Some(alerts[0].id));
    }

    #[test]
    fn a_normal_force_fired_on_gets_a_became_alert_event_and_a_battle_force_that_sights_none() {
        // Night. Force 1 (battle readiness) sights for sure by roll in round
        // 1; force 2 (normal) never sights by rolling, but is fired on.
        // Force 2: ForceBecameAlert citing the first salvo against it, and
        // alerted_by is that event. Force 1: sighting changes nothing and
        // records nothing (it is alert by its readiness): not alerted, no
        // alerted_by, no ForceBecameAlert.
        let scene = night_scene(vec![
            ForceSpec::new(1, Side::Japanese, Hex::new(0, 0), &[1]),
            ForceSpec::new(2, Side::Allied, Hex::new(0, 0), &[1]).not_alert(),
        ]);
        let world = run(&scene, |w| {
            only_force_1_sights_by_roll(w);
            harmless(w);
        });
        let roll = events_of(
            &world,
            |k| matches!(k, EventKind::SightingRoll { task_force, sighted: true, .. } if task_force.raw() == 1),
        );
        assert_eq!(roll.len(), 1);
        let first_against_2 = salvos(&world)
            .into_iter()
            .find(|s| s.target == 2)
            .expect("force 1 fired at ship 2")
            .id;

        let became = events_of(&world, |k| matches!(k, EventKind::ForceBecameAlert { .. }));
        assert_eq!(became.len(), 1);
        assert_eq!(
            became[0].kind,
            EventKind::ForceBecameAlert {
                round: 1,
                task_force: TaskForceId::new(2)
            }
        );
        assert_eq!(became[0].causes, vec![Cause::Event(first_against_2)]);
        assert!(force(&world, 2).alerted);
        assert_eq!(force(&world, 2).alerted_by, Some(became[0].id));

        assert!(!force(&world, 1).alerted);
        assert_eq!(force(&world, 1).alerted_by, None);
        assert!(force(&world, 1).is_alert());
    }

    /// Four forces in one hex by day, all sighting for sure in round 1 and
    /// all firing: Japanese 1 and 3, Allied 2 and 4. Force 3 is the one at
    /// normal readiness. Fights in order: (1,2), (1,4), (2,3), (3,4).
    fn crowded_hex(seed: u64) -> Scene {
        let hex = Hex::new(0, 0);
        Scene::new(
            seed,
            Pulse::Morning,
            vec![
                ForceSpec::new(1, Side::Japanese, hex, &[1]),
                ForceSpec::new(2, Side::Allied, hex, &[1]),
                ForceSpec::new(3, Side::Japanese, hex, &[1]).not_alert(),
                ForceSpec::new(4, Side::Allied, hex, &[1]),
            ],
        )
    }

    /// Fights that harm nobody, with the given alarm chance.
    fn alarm_chance_is(chance: Fixed) -> impl FnOnce(&mut World) {
        move |world| {
            harmless(world);
            figures_mut(world).alarm_chance = chance;
        }
    }

    /// Every alarm roll, whether or not it succeeded.
    fn alarm_rolls(world: &World) -> Vec<Event> {
        events_of(world, |k| matches!(k, EventKind::AlarmRoll { .. }))
    }

    /// The alarm rolls that succeeded.
    fn alarms(world: &World) -> Vec<Event> {
        events_of(world, |k| {
            matches!(k, EventKind::AlarmRoll { alarmed: true, .. })
        })
    }

    fn ending_ids(world: &World) -> Vec<EventId> {
        world
            .events()
            .iter()
            .filter(|e| matches!(e.kind, EventKind::EngagementEnded { .. }))
            .map(|e| e.id)
            .collect()
    }

    /// The first draw of a force's alarm stream on day 1, in the morning.
    fn alarm_stream(world: &World, id: u32) -> RngStream {
        world.rng(
            1,
            Some(Pulse::Morning),
            Subsystem::SurfaceCombat,
            Draw::Legacy,
            EntityRef::TaskForce(TaskForceId::new(id)),
        )
    }

    #[test]
    fn an_alarm_alerts_a_force_in_the_hex_citing_the_end_of_the_fight_with_its_breakdown() {
        // Forces 1 (Japanese) and 2 (Allied) fight first, with fire. Force 3
        // is in the hex at normal readiness and not alerted; with the alarm
        // chance 1 it is alarmed by the fight of 1 and 2, citing that fight's
        // EngagementEnded. Forces 1, 2 and 4 are at battle readiness, so
        // they never roll. Force 3 hears only that fight: its later fights
        // (2,3) and (3,4) are its own.
        let mut hash_with = None;
        for record in [true, false] {
            let world = run(&crowded_hex(1), move |w| {
                alarm_chance_is(Fixed::ONE)(w);
                w.set_config(SimConfig {
                    record_breakdowns: record,
                });
            });
            let alarms = alarm_rolls(&world);
            assert_eq!(alarms.len(), 1);
            let first_end = ending_ids(&world)[0];
            assert_eq!(
                alarms[0].kind,
                EventKind::AlarmRoll {
                    task_force: TaskForceId::new(3),
                    by_a: TaskForceId::new(1),
                    by_b: TaskForceId::new(2),
                    chance: Fixed::ONE,
                    alarmed: true
                }
            );
            assert_eq!(alarms[0].causes, vec![Cause::Event(first_end)]);
            assert_eq!(alarms[0].pulse, Some(Pulse::Morning));
            assert!(force(&world, 3).alerted);
            assert_eq!(force(&world, 3).alerted_by, Some(alarms[0].id));
            // The alarm comes after the fight's report and claim, before
            // the next fight begins.
            let next_begun = world
                .events()
                .iter()
                .filter(|e| matches!(e.kind, EventKind::EngagementBegun { .. }))
                .nth(1)
                .expect("a second fight")
                .id;
            assert!(first_end < alarms[0].id && alarms[0].id < next_begun);
            let first_claim =
                events_of(&world, |k| matches!(k, EventKind::ClaimReceived { .. }))[0].id;
            assert!(first_claim < alarms[0].id);
            if record {
                let breakdown = alarms[0].breakdown.as_ref().expect("a breakdown");
                assert_eq!(breakdown.result, Fixed::ONE);
                assert_eq!(breakdown.factors[0].name, "alarm_chance");
                assert_eq!(breakdown.factors[0].value, Fixed::ONE);
                assert_eq!(breakdown.factors[1].name, "draw");
                // The draw is the first of force 3's own stream.
                let draw = alarm_stream(&world, 3).below(Fixed::SCALE.unsigned_abs());
                assert_eq!(
                    breakdown.factors[1].value.raw(),
                    i64::try_from(draw).expect("fits")
                );
                hash_with = Some(world.state_hash());
            } else {
                assert!(alarms[0].breakdown.is_none());
                assert_eq!(Some(world.state_hash()), hash_with);
            }
        }
    }

    #[test]
    fn an_alarm_chance_of_zero_alarms_nobody_and_the_parties_are_never_alarmed() {
        // Chance 0: no draw is below 0, nobody is alarmed, force 3 included.
        // Force 3 still rolls, and fails, for each of the two fights it
        // hears: (1,2) and (1,4).
        let world = run(&crowded_hex(1), alarm_chance_is(Fixed::ZERO));
        assert!(count(&world, |k| matches!(k, EventKind::Salvo { .. })) > 0);
        assert!(alarms(&world).is_empty());
        let rolls: Vec<(u32, u32, u32, Fixed, bool)> = alarm_rolls(&world)
            .iter()
            .map(|e| match e.kind {
                EventKind::AlarmRoll {
                    task_force,
                    by_a,
                    by_b,
                    chance,
                    alarmed,
                } => (task_force.raw(), by_a.raw(), by_b.raw(), chance, alarmed),
                _ => unreachable!(),
            })
            .collect();
        assert_eq!(
            rolls,
            vec![(3, 1, 2, Fixed::ZERO, false), (3, 1, 4, Fixed::ZERO, false)]
        );

        // Two forces only, both at normal readiness, and chance 1: there is
        // nobody but the parties, and they are never alarmed.
        let hex = Hex::new(0, 0);
        let scene = day_scene(vec![
            ForceSpec::new(1, Side::Japanese, hex, &[1]).not_alert(),
            ForceSpec::new(2, Side::Allied, hex, &[1]).not_alert(),
        ]);
        let world = run(&scene, |w| {
            alarm_chance_is(Fixed::ONE)(w);
            figures_mut(w).not_alert_sighting_factor = Fixed::ONE;
        });
        assert!(count(&world, |k| matches!(k, EventKind::Salvo { .. })) > 0);
        assert!(alarm_rolls(&world).is_empty());
    }

    /// Night: Japanese force 1 (avoids, battle readiness), Allied force 2
    /// (normal) and Japanese force 3 (normal) in one hex.
    fn night_crowded() -> Scene {
        let hex = Hex::new(0, 0);
        night_scene(vec![
            ForceSpec::new(1, Side::Japanese, hex, &[1]).stance(Stance::Avoid),
            ForceSpec::new(2, Side::Allied, hex, &[1]).not_alert(),
            ForceSpec::new(3, Side::Japanese, hex, &[1]).not_alert(),
        ])
    }

    #[test]
    fn no_alarm_after_a_fight_in_which_nobody_fired() {
        // Passed unseen: nobody sights (night chance 0), no salvo, no alarm,
        // though force 3 is in the hex, normal and not alerted, and the
        // alarm chance is 1.
        let world = run(&night_crowded(), |w| {
            alarm_chance_is(Fixed::ONE)(w);
            figures_mut(w).sighting_chance_night = Fixed::ZERO;
        });
        assert!(salvos(&world).is_empty());
        assert!(
            count(&world, |k| matches!(
                k,
                EventKind::ForcesPassedUnseen { .. }
            )) > 0,
            "the forces passed unseen"
        );
        assert!(alarm_rolls(&world).is_empty());
        assert!(!force(&world, 3).alerted);

        // Slipped away before any fire: force 1 avoids and sights, the
        // normal forces never do (their chance is multiplied by 0), so the
        // fight with force 2 ends with no salvo, and force 2 and 3 never
        // sight each other. Force 3 draws no alarm.
        let world = run(&night_crowded(), |w| {
            alarm_chance_is(Fixed::ONE)(w);
            only_force_1_sights_by_roll(w);
        });
        assert!(salvos(&world).is_empty());
        assert!(count(&world, |k| matches!(k, EventKind::ForceSlippedAway { .. })) > 0);
        assert!(alarm_rolls(&world).is_empty());
        assert!(!force(&world, 3).alerted);
    }

    #[test]
    fn an_alarm_is_never_rolled_for_a_force_that_is_already_alert() {
        // Alarm chance 1. Japanese force 1 and Allied force 2 fight first.
        // Force 3 is at battle readiness, force 5 is normal but already
        // alerted, force 6 is normal and not alerted. Only force 6 is
        // alarmed, and only by the first fight.
        let hex = Hex::new(0, 0);
        let scene = day_scene(vec![
            ForceSpec::new(1, Side::Japanese, hex, &[1]),
            ForceSpec::new(2, Side::Allied, hex, &[1]),
            ForceSpec::new(3, Side::Japanese, hex, &[1]),
            ForceSpec::new(5, Side::Japanese, hex, &[1]).not_alert(),
            ForceSpec::new(6, Side::Japanese, hex, &[1]).not_alert(),
        ]);
        let world = run(&scene, |w| {
            alarm_chance_is(Fixed::ONE)(w);
            force_mut(w, 5).alerted = true;
        });
        let alarmed: Vec<(u32, u32, u32)> = alarm_rolls(&world)
            .iter()
            .map(|e| match e.kind {
                EventKind::AlarmRoll {
                    task_force,
                    by_a,
                    by_b,
                    alarmed: true,
                    ..
                } => (task_force.raw(), by_a.raw(), by_b.raw()),
                _ => unreachable!(),
            })
            .collect();
        assert_eq!(alarmed, vec![(6, 1, 2)]);
        // Force 5 was alerted by nothing the log knows about: it stays so.
        assert_eq!(force(&world, 5).alerted_by, None);
    }

    #[test]
    fn a_force_that_hears_two_fights_draws_twice_from_one_stream() {
        // The crowded hex at alarm chance 0.5, so a draw succeeds below
        // 500000 of 1000000. Force 3 hears fight (1,2) and then fight (1,4)
        // if the first alarm failed. Its stream is keyed on the force alone:
        // the first draw is for the first fight, the second for the second.
        //   draw 1 below the threshold: alarmed by (1,2);
        //   else draw 2 below it: alarmed by (1,4);
        //   else not alarmed, after two failed rolls (it then fights as a
        //   party).
        // A fresh stream for the second fight would repeat draw 1 and never
        // alarm in the second fight, so seeing outcome 2 shows the stream is
        // carried on.
        let threshold = u64::try_from(fx("0.5").raw()).expect("positive");
        let mut seen = [0u32; 3];
        for seed in 1..=60 {
            let world = run(&crowded_hex(seed), alarm_chance_is(fx("0.5")));
            let mut stream = alarm_stream(&world, 3);
            let draw_1 = stream.below(Fixed::SCALE.unsigned_abs());
            let draw_2 = stream.below(Fixed::SCALE.unsigned_abs());
            let ends = ending_ids(&world);
            let rolls = alarm_rolls(&world);
            let roll = |by_b: u32, alarmed: bool| EventKind::AlarmRoll {
                task_force: TaskForceId::new(3),
                by_a: TaskForceId::new(1),
                by_b: TaskForceId::new(by_b),
                chance: fx("0.5"),
                alarmed,
            };
            if draw_1 < threshold {
                seen[0] += 1;
                assert_eq!(rolls.len(), 1, "seed {seed}");
                assert_eq!(rolls[0].kind, roll(2, true));
                assert_eq!(rolls[0].causes, vec![Cause::Event(ends[0])]);
            } else if draw_2 < threshold {
                seen[1] += 1;
                assert_eq!(rolls.len(), 2, "seed {seed}");
                assert_eq!(rolls[0].kind, roll(2, false));
                assert_eq!(rolls[0].causes, vec![Cause::Event(ends[0])]);
                assert_eq!(rolls[1].kind, roll(4, true));
                assert_eq!(rolls[1].causes, vec![Cause::Event(ends[1])]);
            } else {
                seen[2] += 1;
                assert_eq!(rolls.len(), 2, "seed {seed}");
                assert_eq!(rolls[0].kind, roll(2, false));
                assert_eq!(rolls[1].kind, roll(4, false));
                assert!(alarms(&world).is_empty(), "seed {seed}");
            }
        }
        assert!(seen.iter().all(|n| *n > 0), "every outcome seen: {seen:?}");
    }

    #[test]
    fn two_forces_are_alarmed_independently_each_from_its_own_stream() {
        // Force 1 (Japanese) and 2 (Allied) fight first, at battle
        // readiness. Forces 3 (Japanese) and 5 (Allied) are normal and not
        // alerted. Each hears the first fight and draws the first number of
        // its own stream, against 0.5. Over the seeds all four combinations
        // of alarmed and not occur: different streams.
        let hex = Hex::new(0, 0);
        let threshold = u64::try_from(fx("0.5").raw()).expect("positive");
        let mut outcomes = BTreeSet::new();
        for seed in 1..=60 {
            let scene = Scene::new(
                seed,
                Pulse::Morning,
                vec![
                    ForceSpec::new(1, Side::Japanese, hex, &[1]),
                    ForceSpec::new(2, Side::Allied, hex, &[1]),
                    ForceSpec::new(3, Side::Japanese, hex, &[1]).not_alert(),
                    ForceSpec::new(5, Side::Allied, hex, &[1]).not_alert(),
                ],
            );
            let world = run(&scene, alarm_chance_is(fx("0.5")));
            let first_end = ending_ids(&world)[0];
            let mut outcome = Vec::new();
            for id in [3, 5] {
                let alarmed =
                    alarm_stream(&world, id).below(Fixed::SCALE.unsigned_abs()) < threshold;
                let rolls = alarm_rolls(&world);
                let by_first_fight = rolls.iter().find(|e| {
                    e.causes == vec![Cause::Event(first_end)]
                        && matches!(e.kind, EventKind::AlarmRoll { task_force, .. } if task_force.raw() == id)
                });
                let roll = by_first_fight.expect("every hearer rolls");
                assert!(
                    matches!(roll.kind, EventKind::AlarmRoll { alarmed: a, .. } if a == alarmed),
                    "seed {seed}, force {id}"
                );
                outcome.push(alarmed);
            }
            outcomes.insert(outcome);
        }
        assert_eq!(
            outcomes.len(),
            4,
            "all four combinations occur: {outcomes:?}"
        );
    }

    #[test]
    fn breakdowns_leave_the_hash_alone_for_a_run_with_an_alarm_and_an_alert_from_belief() {
        // Force 7 (Japanese, normal) is at (5,0) with a Japanese contact at
        // (4,0): alerted at the start of the pulse by belief. The contact is
        // 4 hexes from the crowded hex at (0,0), so no one there is. Then the
        // crowded hex fights, and force 3 is alarmed (chance 1). The log has
        // both kinds, and events, causes and hash agree with breakdowns on
        // and off.
        let mut forces = crowded_hex(1).forces;
        forces.push(ForceSpec::new(7, Side::Japanese, Hex::new(5, 0), &[1]).not_alert());
        let scene = Scene::new(1, Pulse::Morning, forces);
        let build = |record: bool| {
            let mut world = scene.world();
            alarm_chance_is(Fixed::ONE)(&mut world);
            world.set_config(SimConfig {
                record_breakdowns: record,
            });
            give_contact(&mut world, Side::Japanese, Hex::new(4, 0));
            world.alert_from_belief(1, scene.pulse);
            scene.run_step(&mut world);
            world.alert_from_belief(1, Pulse::Afternoon);
            world
        };
        let with = build(true);
        let without = build(false);
        // One alert at the start of the morning pulse (force 7). Those at
        // the start of the afternoon follow from the fight's reports.
        let morning = alerts_by_contact(&with)
            .into_iter()
            .filter(|e| e.pulse == Some(Pulse::Morning))
            .count();
        assert_eq!(morning, 1);
        assert_eq!(alarms(&with).len(), 1);
        assert_eq!(with.state_hash(), without.state_hash());
        assert_eq!(with.events().len(), without.events().len());
        for (a, b) in with.events().iter().zip(without.events().iter()) {
            assert_eq!(a.kind, b.kind);
            assert_eq!(a.causes, b.causes);
            assert!(b.breakdown.is_none());
        }
    }

    #[test]
    fn a_resolved_day_runs_alert_from_belief_before_the_first_phase_of_the_first_pulse() {
        // A day through `resolve_day`: the alert event is the first recorded
        // in the first pulse (Night), and the force stays alerted by the
        // fresh contact in the pulses after it (no second event).
        let mut world = lone_allied();
        give_contact(&mut world, Side::Allied, Hex::new(1, 0));
        let before = world.events().len();
        world.resolve_day().expect("a day resolves");
        let first = &world.events().iter().collect::<Vec<_>>()[before];
        assert!(matches!(
            first.kind,
            EventKind::ForceAlertedByContact { .. }
        ));
        assert_eq!(first.pulse, Some(Pulse::ALL[0]));
        assert_eq!(alerts_by_contact(&world).len(), 1);
    }

    // ---- starting contacts (decision 0021, 8; `start.rs`) ----
    //
    // The placeholder search figures: a contact is dropped at age 5 (held at
    // age 4), merge distance 2 hexes and age 2 pulses.

    use crate::start::StartingContact;

    fn starting(side: Side, q: i32, r: i32, age: u32) -> StartingContact {
        StartingContact {
            side,
            hex: Hex::new(q, r),
            ship_count: 3,
            kind: ShipKind::Cruiser,
            age_pulses: age,
        }
    }

    fn contacts_of(world: &World, side: Side) -> Vec<crate::belief::Contact> {
        world.beliefs[&side].contacts().cloned().collect()
    }

    fn day_zero_kinds(world: &World) -> Vec<&'static str> {
        world
            .events()
            .iter()
            .map(|event| {
                assert_eq!(event.day, 0);
                assert_eq!(event.pulse, None);
                match event.kind {
                    EventKind::ReportReceived { .. } => "report",
                    EventKind::ContactAged { .. } => "aged",
                    _ => panic!("only reports and ageing are recorded before day 1"),
                }
            })
            .collect()
    }

    #[test]
    fn a_fresh_starting_contact_is_one_report_at_age_zero() {
        // One Allied contact, age 0: one report, no ageing events, age 0.
        let mut world = lone_allied();
        world
            .hold_starting_contacts(vec![starting(Side::Allied, 3, 0, 0)])
            .expect("held");
        let contacts = contacts_of(&world, Side::Allied);
        assert_eq!(contacts.len(), 1);
        let contact = &contacts[0];
        assert_eq!(contact.estimated_hex, Hex::new(3, 0));
        assert_eq!(contact.estimated_ship_count, 3);
        assert_eq!(contact.estimated_kind, ShipKind::Cruiser);
        assert_eq!(contact.age_pulses, 0);
        assert_eq!(day_zero_kinds(&world), ["report"]);
        let event = world.events().iter().next().expect("one event");
        assert_eq!(event.id, EventId::new(1));
        assert!(event.causes.is_empty());
        assert_eq!(contact.last_report_event, event.id);
        assert_eq!(
            event.to_string(),
            "#1 D0 -         Allied report: 3 ships, cruiser, at (3,0), held at the start: new contact:1"
        );
        assert!(contacts_of(&world, Side::Japanese).is_empty());
        // Nothing is left refreshed, so the state can be hashed.
        world.state_hash();
    }

    #[test]
    fn a_starting_contact_has_exactly_its_age() {
        // Age 2: the report, then 2 ageing events citing it.
        let mut world = lone_allied();
        world
            .hold_starting_contacts(vec![starting(Side::Allied, 3, 0, 2)])
            .expect("held");
        let contact = &contacts_of(&world, Side::Allied)[0];
        assert_eq!(contact.age_pulses, 2);
        assert_eq!(day_zero_kinds(&world), ["report", "aged", "aged"]);
        let events: Vec<Event> = world.events().iter().cloned().collect();
        for (event, age) in events[1..].iter().zip([1, 2]) {
            assert_eq!(
                event.kind,
                EventKind::ContactAged {
                    side: Side::Allied,
                    contact: contact.id,
                    age_pulses: age
                }
            );
            assert_eq!(event.causes, vec![Cause::Event(events[0].id)]);
        }

        let mut world = lone_allied();
        world
            .hold_starting_contacts(vec![starting(Side::Allied, 3, 0, 3)])
            .expect("held");
        let contact = &contacts_of(&world, Side::Allied)[0];
        assert_eq!(contact.age_pulses, 3);
        assert_eq!(day_zero_kinds(&world), ["report", "aged", "aged", "aged"]);

        // Age 4 is the oldest held: age 5 would be dropped.
        let mut world = lone_allied();
        world
            .hold_starting_contacts(vec![starting(Side::Allied, 3, 0, 4)])
            .expect("held");
        assert_eq!(contacts_of(&world, Side::Allied)[0].age_pulses, 4);
    }

    #[test]
    fn a_starting_contact_that_would_be_dropped_is_an_error_naming_it() {
        // Age 6 is past the drop age of 5 (it already is dropped at age 5).
        // The error names the entry and nothing is left behind.
        let mut world = lone_allied();
        let hash = world.state_hash();
        let error = world
            .hold_starting_contacts(vec![
                starting(Side::Allied, 3, 0, 0),
                starting(Side::Japanese, -4, 2, 6),
            ])
            .expect_err("dropped");
        let crate::SimError::InvalidSetup(message) = error else {
            panic!("an invalid setup");
        };
        assert!(
            message.contains("Japanese starting contact at (-4,2)"),
            "{message}"
        );
        assert!(message.contains("dropped"), "{message}");
        assert!(world.events().is_empty());
        assert!(contacts_of(&world, Side::Allied).is_empty());
        assert_eq!(world.state_hash(), hash);
    }

    #[test]
    fn entries_of_different_ages_each_end_with_their_own() {
        // Allied A (3,0) age 3 and B (-3,0) age 1, 6 hexes apart (no merge).
        // Virtual pulse 3: A is reported. Pulse 2: A ages to 1. Pulse 1: B is
        // reported, A ages to 2 and B is skipped. Pulse 0: A ages to 3, B to
        // 1. So A ends at 3 and B at 1: 2 reports and 4
        // ageing events, in the order they happened.
        let mut world = lone_allied();
        world
            .hold_starting_contacts(vec![
                starting(Side::Allied, 3, 0, 3),
                starting(Side::Allied, -3, 0, 1),
            ])
            .expect("held");
        let contacts = contacts_of(&world, Side::Allied);
        assert_eq!(contacts[0].estimated_hex, Hex::new(3, 0));
        assert_eq!(contacts[0].age_pulses, 3);
        assert_eq!(contacts[1].estimated_hex, Hex::new(-3, 0));
        assert_eq!(contacts[1].age_pulses, 1);
        assert_eq!(
            day_zero_kinds(&world),
            ["report", "aged", "report", "aged", "aged", "aged"]
        );
    }

    #[test]
    fn a_side_is_finished_before_the_next_and_the_allied_side_goes_first() {
        // File order puts the Japanese entry first, but the Allied side is
        // done first (Side::ALL): its report is event 1.
        let mut world = lone_allied();
        world
            .hold_starting_contacts(vec![
                starting(Side::Japanese, 3, 0, 1),
                starting(Side::Allied, -3, 0, 1),
            ])
            .expect("held");
        let sides: Vec<Side> = world
            .events()
            .iter()
            .map(|event| match event.kind {
                EventKind::ReportReceived { side, .. } | EventKind::ContactAged { side, .. } => {
                    side
                }
                _ => panic!("unexpected event"),
            })
            .collect();
        assert_eq!(
            sides,
            [Side::Allied, Side::Allied, Side::Japanese, Side::Japanese]
        );
        assert_eq!(contacts_of(&world, Side::Japanese)[0].age_pulses, 1);
    }

    #[test]
    fn entries_that_would_merge_are_an_error() {
        // (3,0) and (4,0) are 1 hex apart, within the merge distance of 2,
        // both age 0: the second report would refresh the first.
        let mut world = lone_allied();
        let error = world
            .hold_starting_contacts(vec![
                starting(Side::Allied, 3, 0, 0),
                starting(Side::Allied, 4, 0, 0),
            ])
            .expect_err("merge");
        assert!(error.to_string().contains("too close"), "{error}");
        assert!(world.events().is_empty());

        // The older contact is only 2 pulses old when the younger is
        // reported (age 2 <= 2): still a merge.
        let error = world
            .hold_starting_contacts(vec![
                starting(Side::Allied, 3, 0, 2),
                starting(Side::Allied, 3, 0, 0),
            ])
            .expect_err("merge");
        assert!(error.to_string().contains("too close"), "{error}");

        // The same places on the two sides are two beliefs, not a merge.
        world
            .hold_starting_contacts(vec![
                starting(Side::Allied, 3, 0, 0),
                starting(Side::Japanese, 3, 0, 0),
            ])
            .expect("held");
    }

    #[test]
    fn a_starting_contact_needs_ships_and_search_figures_and_a_game_not_begun() {
        let mut world = lone_allied();
        let mut none = starting(Side::Allied, 3, 0, 0);
        none.ship_count = 0;
        let error = world.hold_starting_contacts(vec![none]).expect_err("0");
        assert!(error.to_string().contains("no ships"), "{error}");

        world.ref_data.search = None;
        assert!(world
            .hold_starting_contacts(vec![starting(Side::Allied, 3, 0, 0)])
            .is_err());

        // A second call is refused as well (it would age the first call's
        // contacts again). An empty list changes nothing.
        let mut world = lone_allied();
        world
            .hold_starting_contacts(vec![starting(Side::Allied, 3, 0, 0)])
            .expect("held");
        assert!(world
            .hold_starting_contacts(vec![starting(Side::Japanese, -3, 0, 0)])
            .is_err());

        // After day 1 it is refused.
        let mut world = lone_allied();
        world.resolve_day().expect("a day");
        assert!(world
            .hold_starting_contacts(vec![starting(Side::Allied, 3, 0, 0)])
            .is_err());
        assert!(world.hold_starting_contacts(Vec::new()).is_err());
    }

    #[test]
    fn the_state_hash_includes_starting_contacts() {
        // hash_world hashes each side's contacts (id, hex, count, kind, age,
        // last report event), so a contact, and its age, change
        // the hash.
        let bare = lone_allied().state_hash();
        let hash_with = |age: u32| {
            let mut world = lone_allied();
            world
                .hold_starting_contacts(vec![starting(Side::Allied, 3, 0, age)])
                .expect("held");
            world.state_hash()
        };
        assert_ne!(hash_with(0), bare);
        assert_ne!(hash_with(0), hash_with(2));
        assert_eq!(hash_with(2), hash_with(2));
    }

    #[test]
    fn a_fresh_starting_contact_alerts_a_normal_force_at_the_start_of_the_night_pulse() {
        // Allied force 1 at normal readiness at (5,0), a fresh Allied contact
        // at (3,0): 2 hexes away, within 3, and age 0 is within 2. Day 1's
        // first event is the alert, in the night pulse, citing the day-0
        // report (event 1). The contact then ages once, at the end of the
        // night pulse, like any other.
        let mut world = Scene::new(
            1,
            Pulse::Night,
            vec![ForceSpec::new(1, Side::Allied, Hex::new(5, 0), &[1]).not_alert()],
        )
        .world();
        world
            .hold_starting_contacts(vec![starting(Side::Allied, 3, 0, 0)])
            .expect("held");
        let contact = contacts_of(&world, Side::Allied)[0].id;
        let summary = world.resolve_day().expect("a day");
        assert_eq!(summary.first_event, 1);
        let events: Vec<Event> = world.events().iter().cloned().collect();
        let alert = &events[1];
        assert_eq!(alert.day, 1);
        assert_eq!(alert.pulse, Some(Pulse::Night));
        assert_eq!(
            alert.kind,
            EventKind::ForceAlertedByContact {
                task_force: TaskForceId::new(1),
                contact
            }
        );
        assert_eq!(alert.causes, vec![Cause::Event(events[0].id)]);
        assert_eq!(events[0].day, 0);
        let aged = events
            .iter()
            .find(|e| matches!(e.kind, EventKind::ContactAged { .. }) && e.day == 1)
            .expect("aged on day 1");
        assert_eq!(aged.pulse, Some(Pulse::Night));
    }

    // ---- fatigue of a task force (decision 0037) ----

    use crate::events::ForceFatigueChange;
    use crate::ids::OrderId;
    use crate::refdata::FatigueFigures;

    /// The figures of `data/reference/fatigue.toml`: 5 gained a pulse by an
    /// alert force, 3 lost by one that is not.
    fn tire(world: &mut World) {
        world.ref_data.fatigue = Some(FatigueFigures {
            air_fatigue_per_sortie: Fixed::from_int(3),
            air_fatigue_per_100_nm: Fixed::from_int(2),
            air_rest_per_pulse: Fixed::from_int(5),
            force_fatigue_per_pulse_alert: Fixed::from_int(5),
            force_rest_per_pulse: Fixed::from_int(3),
        });
    }

    /// The `TaskForceFatigue` events as (force, change, amount, fatigue).
    fn force_fatigue_events(world: &World) -> Vec<(u32, ForceFatigueChange, Fixed, Fixed)> {
        world
            .events()
            .iter()
            .filter_map(|event| match event.kind {
                EventKind::TaskForceFatigue {
                    task_force,
                    change,
                    amount,
                    fatigue,
                } => Some((task_force.raw(), change, amount, fatigue)),
                _ => None,
            })
            .collect()
    }

    fn at_fatigue(world: &mut World, id: u32, fatigue: i64) {
        force_mut(world, id).fatigue = Fixed::from_int(fatigue);
    }

    fn fatigue_of(world: &World, id: u32) -> Fixed {
        force(world, id).fatigue
    }

    #[test]
    fn a_force_at_battle_readiness_gains_fatigue_citing_its_readiness_order() {
        // Battle readiness is alert: it gains 5 at the start of a pulse. With
        // a readiness order the event cites it; a force that started at
        // battle readiness has no order and no cause.
        let mut world = lone_allied();
        force_mut(&mut world, 1).readiness = Readiness::Battle;
        tire(&mut world);
        world.tire_and_rest_forces(1, Pulse::Night);
        assert_eq!(fatigue_of(&world, 1), Fixed::from_int(5));
        assert_eq!(
            force_fatigue_events(&world),
            vec![(
                1,
                ForceFatigueChange::Gained,
                Fixed::from_int(5),
                Fixed::from_int(5)
            )]
        );
        assert!(world
            .events()
            .iter()
            .next()
            .expect("event")
            .causes
            .is_empty());

        force_mut(&mut world, 1).readiness_order = Some(OrderId::new(7));
        world.tire_and_rest_forces(1, Pulse::Morning);
        assert_eq!(fatigue_of(&world, 1), Fixed::from_int(10));
        let last = world.events().iter().last().expect("event");
        assert_eq!(last.causes, vec![Cause::Order(OrderId::new(7))]);
        assert_eq!(last.pulse, Some(Pulse::Morning));
    }

    #[test]
    fn a_force_alerted_by_a_contact_at_normal_readiness_gains_fatigue() {
        let mut world = lone_allied();
        force_mut(&mut world, 1).readiness = Readiness::Normal;
        tire(&mut world);
        // Not alert: nothing to rest at 0, so no event.
        world.tire_and_rest_forces(1, Pulse::Night);
        assert!(force_fatigue_events(&world).is_empty());
        // A contact two hexes away alerts it at the start of the next pulse,
        // and the fatigue event cites the event that alerted it.
        give_contact(&mut world, Side::Allied, Hex::new(2, 0));
        world.alert_from_belief(1, Pulse::Morning);
        let alerted_by = force(&world, 1).alerted_by.expect("alerted");
        world.tire_and_rest_forces(1, Pulse::Morning);
        assert_eq!(fatigue_of(&world, 1), Fixed::from_int(5));
        let last = world.events().iter().last().expect("event");
        assert_eq!(last.causes, vec![Cause::Event(alerted_by)]);
    }

    #[test]
    fn a_force_that_is_not_alert_rests_and_stops_at_zero() {
        let mut world = lone_allied();
        force_mut(&mut world, 1).readiness = Readiness::Normal;
        force_mut(&mut world, 1).readiness_order = Some(OrderId::new(9));
        tire(&mut world);
        at_fatigue(&mut world, 1, 7);
        for expected in [4, 1, 0] {
            world.tire_and_rest_forces(1, Pulse::Night);
            assert_eq!(fatigue_of(&world, 1), Fixed::from_int(expected));
        }
        // 7 to 4 to 1 to 0: the last rest is 1, not 3. At 0 nothing more.
        world.tire_and_rest_forces(1, Pulse::Morning);
        let events = force_fatigue_events(&world);
        assert_eq!(
            events,
            vec![
                (
                    1,
                    ForceFatigueChange::Rested,
                    Fixed::from_int(3),
                    Fixed::from_int(4)
                ),
                (
                    1,
                    ForceFatigueChange::Rested,
                    Fixed::from_int(3),
                    Fixed::from_int(1)
                ),
                (
                    1,
                    ForceFatigueChange::Rested,
                    Fixed::from_int(1),
                    Fixed::ZERO
                ),
            ]
        );
        let first = world.events().iter().next().expect("event");
        assert_eq!(first.causes, vec![Cause::Order(OrderId::new(9))]);
    }

    #[test]
    fn fatigue_stops_at_100() {
        let mut world = lone_allied();
        force_mut(&mut world, 1).readiness = Readiness::Battle;
        tire(&mut world);
        at_fatigue(&mut world, 1, 98);
        world.tire_and_rest_forces(1, Pulse::Night);
        world.tire_and_rest_forces(1, Pulse::Morning);
        assert_eq!(fatigue_of(&world, 1), Fixed::from_int(100));
        assert_eq!(
            force_fatigue_events(&world),
            vec![(
                1,
                ForceFatigueChange::Gained,
                Fixed::from_int(2),
                Fixed::from_int(100)
            )]
        );
    }

    #[test]
    fn without_the_fatigue_figures_nothing_is_gained_rested_or_recorded() {
        let mut world = lone_allied();
        at_fatigue(&mut world, 1, 40);
        world.tire_and_rest_forces(1, Pulse::Night);
        assert_eq!(fatigue_of(&world, 1), Fixed::from_int(40));
        assert_eq!(world.events().len(), 0);
        // And a starting fatigue cuts no chance to sight.
        let scene = night_scene(vec![
            ForceSpec::new(1, Side::Japanese, Hex::new(5, 0), &[1]),
            ForceSpec::new(2, Side::Allied, Hex::new(5, 0), &[1]),
        ]);
        let world = run(&scene, |w| at_fatigue(w, 1, 40));
        assert_eq!(first_round_chances(&world), [fx("0.5"), fx("0.5")]);
        assert_eq!(
            count(&world, |k| matches!(
                k,
                EventKind::SightingRoll { fatigue_factor, .. } if *fatigue_factor != Fixed::ONE
            )),
            0
        );
    }

    #[test]
    fn a_force_with_no_ship_afloat_neither_tires_nor_rests() {
        let mut world = lone_allied();
        force_mut(&mut world, 1).readiness = Readiness::Battle;
        tire(&mut world);
        at_fatigue(&mut world, 1, 20);
        force_mut(&mut world, 1).ships.clear();
        world.tire_and_rest_forces(1, Pulse::Night);
        assert_eq!(fatigue_of(&world, 1), Fixed::from_int(20));
        assert!(force_fatigue_events(&world).is_empty());
    }

    #[test]
    fn forces_are_worked_in_id_order_each_with_its_own_alertness() {
        let scene = Scene::new(
            1,
            Pulse::Night,
            vec![
                ForceSpec::new(2, Side::Allied, Hex::new(0, 0), &[1]),
                ForceSpec::new(1, Side::Japanese, Hex::new(3, 0), &[1]).not_alert(),
            ],
        );
        let mut world = scene.world();
        tire(&mut world);
        at_fatigue(&mut world, 1, 10);
        world.tire_and_rest_forces(1, Pulse::Night);
        assert_eq!(
            force_fatigue_events(&world),
            vec![
                (
                    1,
                    ForceFatigueChange::Rested,
                    Fixed::from_int(3),
                    Fixed::from_int(7)
                ),
                (
                    2,
                    ForceFatigueChange::Gained,
                    Fixed::from_int(5),
                    Fixed::from_int(5)
                ),
            ]
        );
    }

    #[test]
    fn fatigue_cuts_the_chance_to_sight_by_100_less_fatigue_over_100() {
        // Night base 0.5 for an alert force: x 1.00 at fatigue 0, x 0.65 at
        // 35 (0.325), x 0 at 100. The other force is untouched. The event
        // carries the factor and the chance is the final one.
        let scene = night_scene(vec![
            ForceSpec::new(1, Side::Japanese, Hex::new(5, 0), &[1]),
            ForceSpec::new(2, Side::Allied, Hex::new(5, 0), &[1]),
        ]);
        for (fatigue, chance, factor) in [(0, "0.5", "1"), (35, "0.325", "0.65"), (100, "0", "0")] {
            let world = run(&scene, |w| {
                tire(w);
                at_fatigue(w, 1, fatigue);
                w.set_config(SimConfig {
                    record_breakdowns: true,
                });
            });
            assert_eq!(
                first_round_chances(&world),
                [fx(chance), fx("0.5")],
                "fatigue {fatigue}"
            );
            let roll = world
                .events()
                .iter()
                .find(|e| {
                    matches!(
                        e.kind,
                        EventKind::SightingRoll { round: 1, task_force, .. }
                            if task_force == TaskForceId::new(1)
                    )
                })
                .expect("force 1 rolled");
            assert!(matches!(
                roll.kind,
                EventKind::SightingRoll { fatigue_factor, .. } if fatigue_factor == fx(factor)
            ));
            let factors: Vec<(&str, Fixed)> = roll
                .breakdown
                .as_ref()
                .expect("breakdown")
                .factors
                .iter()
                .map(|f| (f.name, f.value))
                .collect();
            assert_eq!(
                factors,
                vec![
                    ("base", fx("0.5")),
                    ("radar_bonus", Fixed::ZERO),
                    ("alert_factor", Fixed::ONE),
                    ("fatigue_factor", fx(factor)),
                ]
            );
        }
    }

    #[test]
    fn the_fatigue_factor_multiplies_after_the_cap_and_the_alert_factor() {
        // By day, not alert with radar: min(1, 1.2 x 0.5) = 0.6, times 0.7
        // at fatigue 30 is 0.42. Alert with radar by day: min(1, 1.2) = 1,
        // times 0.7 is 0.7.
        let hex = Hex::new(5, 0);
        let scene = day_scene(vec![
            ForceSpec::new(1, Side::Japanese, hex, &[4]),
            ForceSpec::new(2, Side::Allied, hex, &[4]).not_alert(),
        ]);
        let world = run(&scene, |w| {
            tire(w);
            at_fatigue(w, 1, 30);
            at_fatigue(w, 2, 30);
        });
        assert_eq!(first_round_chances(&world), [fx("0.7"), fx("0.42")]);
    }

    // ---- experience (decision 0064) ----

    /// Sets the three strengths of experience.
    fn strengths(world: &mut World, gun: &str, torpedo: &str, sighting: &str) {
        let figures = figures_mut(world);
        figures.experience_gun_strength = fx(gun);
        figures.experience_torpedo_strength = fx(torpedo);
        figures.experience_sighting_strength = fx(sighting);
    }

    /// Both forces sight in round 1 for sure, so each ship fires in round 1
    /// at a target that is neither surprised nor lit.
    fn both_sight(world: &mut World) {
        figures_mut(world).sighting_chance_night = Fixed::ONE;
    }

    fn night_duel() -> Scene {
        let hex = Hex::new(5, 0);
        night_scene(vec![
            ForceSpec::new(1, Side::Japanese, hex, &[1]),
            ForceSpec::new(2, Side::Allied, hex, &[1]),
        ])
    }

    fn event_with_id(world: &World, id: EventId) -> &Event {
        world
            .events()
            .iter()
            .find(|e| e.id == id)
            .expect("the event exists")
    }

    fn factor_names(event: &Event) -> Vec<&'static str> {
        event
            .breakdown
            .as_ref()
            .expect("a breakdown")
            .factors
            .iter()
            .map(|f| f.name)
            .collect()
    }

    #[test]
    fn the_factor_is_one_plus_strength_times_experience_less_two() {
        // 1 + strength x (experience - 2), for each value and strength.
        let table = [
            ("0", ["1", "1", "1", "1", "1"]),
            ("0.25", ["0.5", "0.75", "1", "1.25", "1.5"]),
            ("0.5", ["0", "0.5", "1", "1.5", "2"]),
        ];
        for (strength, factors) in table {
            for (experience, factor) in factors.iter().enumerate() {
                let experience = i64::try_from(experience).expect("small");
                assert_eq!(
                    experience_factor(fx(strength), Fixed::from_int(experience)),
                    fx(factor),
                    "strength {strength}, experience {experience}"
                );
            }
        }
    }

    #[test]
    fn in_a_dark_fight_a_ships_gun_and_torpedo_chances_read_her_own_experience() {
        // Night, round 1, nobody surprised or lit: gun 0.03 x factor,
        // torpedo 0.04 x factor, with different strengths for the two:
        // guns 0.25, torpedoes 0.5.
        //   experience 0: 0.03 x 0.5 = 0.015    0.04 x 0 = 0
        //   experience 1: 0.03 x 0.75 = 0.0225  0.04 x 0.5 = 0.02
        //   experience 2: 0.03                  0.04
        //   experience 3: 0.03 x 1.25 = 0.0375  0.04 x 1.5 = 0.06
        //   experience 4: 0.03 x 1.5 = 0.045    0.04 x 2 = 0.08
        let table = [
            (0, "0.015", "0"),
            (1, "0.0225", "0.02"),
            (2, "0.03", "0.04"),
            (3, "0.0375", "0.06"),
            (4, "0.045", "0.08"),
        ];
        for (experience, gun, torpedo) in table {
            let world = run(&night_duel(), |w| {
                both_sight(w);
                strengths(w, "0.25", "0.5", "0");
                // Ship 2 stays at 2, so only the firer's own value counts.
                ship_mut(w, 1).experience = experience;
            });
            let first_gun = first_round_salvo(&world, 1, Weapon::Gun);
            assert_eq!(first_gun.chance, fx(gun), "gun, experience {experience}");
            assert_eq!(first_gun.expected, Fixed::from_int(120) * fx(gun));
            let first_torpedo = first_round_salvo(&world, 1, Weapon::Torpedo);
            assert_eq!(
                first_torpedo.chance,
                fx(torpedo),
                "torpedo, experience {experience}"
            );
            // Ship 2 fires as before.
            assert_eq!(first_round_salvo(&world, 2, Weapon::Gun).chance, fx("0.03"));
            assert_eq!(
                first_round_salvo(&world, 2, Weapon::Torpedo).chance,
                fx("0.04")
            );
        }
    }

    #[test]
    fn a_crew_of_experience_0_at_the_top_strength_hits_nothing() {
        // 1 + 0.5 x (0 - 2) = 0: the chance is 0, the expected hits 0, and
        // no shot hits whatever the draws.
        for seed in 1..=20 {
            let scene = Scene::new(seed, Pulse::Night, night_duel().forces);
            let world = run(&scene, |w| {
                both_sight(w);
                strengths(w, "0.5", "0.5", "0");
                ship_mut(w, 1).experience = 0;
            });
            for weapon in [Weapon::Gun, Weapon::Torpedo] {
                let salvo = first_round_salvo(&world, 1, weapon);
                assert_eq!(salvo.chance, Fixed::ZERO);
                assert_eq!(salvo.expected, Fixed::ZERO);
                assert_eq!(salvo.hits, 0);
            }
            assert!(gun_salvos_of(&world, 1).iter().all(|s| s.hits == 0));
        }
    }

    #[test]
    fn the_experience_factor_multiplies_last_and_is_in_the_breakdown_only_when_it_acts() {
        // Degraded fire control halves the gun chance before the factor:
        // 0.03 x 0.5 x 1.5 (experience 3 at strength 0.5) = 0.0225.
        let world = run(&night_duel(), |w| {
            both_sight(w);
            strengths(w, "0.5", "0.5", "0");
            ship_mut(w, 1).experience = 3;
            ship_mut(w, 1).systems.fire_control = SystemState::Degraded;
        });
        let gun = first_round_salvo(&world, 1, Weapon::Gun);
        assert_eq!(gun.chance, fx("0.0225"));
        let event = event_with_id(&world, gun.id);
        let names = factor_names(event);
        assert_eq!(
            &names[names.len() - 2..],
            ["fire_control_factor", "experience_factor"]
        );
        let value = event
            .breakdown
            .as_ref()
            .expect("a breakdown")
            .factors
            .iter()
            .find(|f| f.name == "experience_factor")
            .expect("named")
            .value;
        assert_eq!(value, fx("1.5"));
        let torpedo = first_round_salvo(&world, 1, Weapon::Torpedo);
        let event = event_with_id(&world, torpedo.id);
        assert_eq!(factor_names(event).last(), Some(&"experience_factor"));

        // At a strength of 0 the factor is not in the breakdown, and the
        // chance is the plain one to the last digit.
        let world = run(&night_duel(), |w| {
            both_sight(w);
            ship_mut(w, 1).experience = 4;
        });
        let gun = first_round_salvo(&world, 1, Weapon::Gun);
        assert_eq!(gun.chance, fx("0.03"));
        let event = event_with_id(&world, gun.id);
        assert!(!factor_names(event).contains(&"experience_factor"));
        for roll in world
            .events()
            .iter()
            .filter(|e| matches!(e.kind, EventKind::SightingRoll { .. }))
        {
            assert!(!factor_names(roll).contains(&"experience_factor"));
        }
    }

    #[test]
    fn by_day_nothing_reads_experience() {
        // Day figures: sighting 1, gun 0.05, torpedo 0.02. Every strength
        // at the top and a crew of 4 and a crew of 0 change none of them,
        // and no breakdown names the factor.
        let world = run(&duel(), |w| {
            strengths(w, "0.5", "0.5", "0.5");
            ship_mut(w, 1).experience = 4;
            ship_mut(w, 2).experience = 0;
        });
        assert_eq!(first_round_chances(&world), [Fixed::ONE, Fixed::ONE]);
        for firer in [1, 2] {
            assert_eq!(
                first_round_salvo(&world, firer, Weapon::Gun).chance,
                fx("0.05")
            );
            assert_eq!(
                first_round_salvo(&world, firer, Weapon::Torpedo).chance,
                fx("0.02")
            );
        }
        for event in world.events().iter() {
            if event.breakdown.is_some() {
                assert!(!factor_names(event).contains(&"experience_factor"));
            }
        }
    }

    #[test]
    fn a_forces_sighting_reads_the_mean_of_its_ships_inside_the_cap_and_before_fatigue() {
        // Ships of 3, 3 and 2: the mean is 8 / 3 = 2.666666 in the fixed
        // arithmetic, the factor at 0.4 is 1 + 0.4 x 0.666666 = 1.266666
        // (0.2666664 cut), and the chance is 0.5 x 1.266666 = 0.633333.
        let hex = Hex::new(5, 0);
        let scene = night_scene(vec![
            ForceSpec::new(1, Side::Japanese, hex, &[1, 1, 1]),
            ForceSpec::new(2, Side::Allied, hex, &[1]),
        ]);
        let world = run(&scene, |w| {
            strengths(w, "0", "0", "0.4");
            ship_mut(w, 1).experience = 3;
            ship_mut(w, 2).experience = 3;
            ship_mut(w, 3).experience = 2;
            ship_mut(w, 4).experience = 2;
        });
        let [mixed, plain] = first_round_chances(&world);
        assert_eq!(mixed.raw(), 633_333);
        assert_eq!(plain, fx("0.5"));
        let roll = world
            .events()
            .iter()
            .find(|e| {
                matches!(
                    e.kind,
                    EventKind::SightingRoll { round: 1, task_force, .. }
                        if task_force == TaskForceId::new(1)
                )
            })
            .expect("force 1 rolled");
        assert_eq!(
            factor_names(roll),
            vec!["base", "radar_bonus", "alert_factor", "experience_factor"]
        );
        let value = roll.breakdown.as_ref().expect("a breakdown").factors[3].value;
        assert_eq!(value.raw(), 1_266_666);

        // The cap holds the experience factor inside it: base 0.9 at a
        // factor of 2 is 1, not 1.8, and fatigue (30) is applied after: 0.7.
        let world = run(&scene, |w| {
            strengths(w, "0", "0", "0.5");
            figures_mut(w).sighting_chance_night = fx("0.9");
            ship_mut(w, 1).experience = 4;
            ship_mut(w, 2).experience = 4;
            ship_mut(w, 3).experience = 4;
            tire(w);
            at_fatigue(w, 1, 30);
        });
        assert_eq!(first_round_chances(&world)[0], fx("0.7"));
    }

    #[test]
    fn the_mean_is_over_the_ships_afloat_at_the_moment_of_the_roll() {
        // Experience 4, 0 and 2: mean 2. With the second ship gone from the
        // force the mean is (4 + 2) / 2 = 3.
        let hex = Hex::new(5, 0);
        let scene = night_scene(vec![
            ForceSpec::new(1, Side::Japanese, hex, &[1, 1, 1]),
            ForceSpec::new(2, Side::Allied, hex, &[1]),
        ]);
        let mut world = scene.world();
        ship_mut(&mut world, 1).experience = 4;
        ship_mut(&mut world, 2).experience = 0;
        ship_mut(&mut world, 3).experience = 2;
        let force = TaskForceId::new(1);
        assert_eq!(world.mean_experience(force), Fixed::from_int(2));
        world
            .truth
            .task_forces
            .get_mut(&force)
            .expect("force")
            .ships
            .remove(&ShipId::new(2));
        ship_mut(&mut world, 2).location = ShipLocation::Sunk;
        assert_eq!(world.mean_experience(force), Fixed::from_int(3));
        // Ships of 3, 3 and 2: 8 / 3, cut at the sixth place.
        ship_mut(&mut world, 1).experience = 3;
        ship_mut(&mut world, 3).experience = 3;
        world
            .truth
            .task_forces
            .get_mut(&force)
            .expect("force")
            .ships
            .insert(ShipId::new(2));
        ship_mut(&mut world, 2).experience = 2;
        ship_mut(&mut world, 2).location = ShipLocation::TaskForce(force);
        assert_eq!(world.mean_experience(force).raw(), 2_666_666);
    }

    #[test]
    fn two_worlds_that_differ_only_in_a_ships_experience_hash_differently() {
        let scene = night_duel();
        let a = scene.world();
        let mut b = scene.world();
        assert_eq!(a.state_hash(), b.state_hash());
        ship_mut(&mut b, 1).experience = 3;
        assert_ne!(a.state_hash(), b.state_hash());
        let mut c = scene.world();
        ship_mut(&mut c, 1).experience = 4;
        assert_ne!(b.state_hash(), c.state_hash());
    }

    #[test]
    fn recording_breakdowns_does_not_change_what_experience_does() {
        let build = |record: bool| {
            let scene = night_duel();
            let mut world = scene.world();
            world.config = SimConfig {
                record_breakdowns: record,
            };
            strengths(&mut world, "0.4", "0.4", "0.4");
            ship_mut(&mut world, 1).experience = 3;
            ship_mut(&mut world, 2).experience = 1;
            scene.run_step(&mut world);
            world
        };
        assert_eq!(build(true).state_hash(), build(false).state_hash());
    }

    // ---- a force not ready fires part of its guns (decision 0068) ----

    /// Both forces sight in round 1 for sure, and the share is 0.5.
    fn unready_duel(world: &mut World) {
        both_sight(world);
        figures_mut(world).unready_gun_share = fx("0.5");
    }

    /// Force 1 at battle readiness, force 2 at normal readiness and never
    /// alerted, at night.
    fn unready_scene() -> Scene {
        let hex = Hex::new(5, 0);
        night_scene(vec![
            ForceSpec::new(1, Side::Japanese, hex, &[1]),
            ForceSpec::new(2, Side::Allied, hex, &[1]).not_alert(),
        ])
    }

    /// Records an event that alerted force 2 in the given day and pulse and
    /// sets the force alerted by it.
    fn alerted_at(world: &mut World, day: u32, pulse: Pulse) {
        let event = world.events.record(
            day,
            Some(pulse),
            EventKind::ForceAlertedByContact {
                task_force: TaskForceId::new(2),
                contact: ContactId::new(1),
            },
            Vec::new(),
            None,
        );
        let force = force_mut(world, 2);
        force.alerted = true;
        force.alerted_by = Some(event);
    }

    #[test]
    fn a_force_is_ready_at_battle_readiness_or_alert_since_before_the_pulse_and_not_otherwise() {
        let mut world = unready_scene().world();
        let id = TaskForceId::new(2);
        // Normal readiness, never alerted: not ready.
        assert!(!world.force_is_ready(id, 1, Pulse::Night));
        // Alerted in the present pulse: not ready in it.
        alerted_at(&mut world, 1, Pulse::Night);
        assert!(!world.force_is_ready(id, 1, Pulse::Night));
        // The same event is an earlier pulse when asked of a later one, and
        // an earlier day when asked of the next day's first pulse.
        assert!(world.force_is_ready(id, 1, Pulse::Morning));
        assert!(world.force_is_ready(id, 1, Pulse::Afternoon));
        assert!(world.force_is_ready(id, 2, Pulse::Night));
        // An event of a later pulse of the same day is not before.
        alerted_at(&mut world, 1, Pulse::Afternoon);
        assert!(!world.force_is_ready(id, 1, Pulse::Morning));
        assert!(!world.force_is_ready(id, 1, Pulse::Afternoon));
        // An event of an earlier day.
        alerted_at(&mut world, 0, Pulse::Afternoon);
        assert!(world.force_is_ready(id, 1, Pulse::Night));
        // Alerted by nothing the log knows about: alerted before.
        force_mut(&mut world, 2).alerted_by = None;
        assert!(world.force_is_ready(id, 1, Pulse::Night));
        // Not alerted any more: not ready, whatever the flag's cause was.
        force_mut(&mut world, 2).alerted = false;
        assert!(!world.force_is_ready(id, 1, Pulse::Night));
        // Battle readiness by order is ready, alerted or not.
        force_mut(&mut world, 2).readiness = Readiness::Battle;
        assert!(world.force_is_ready(id, 1, Pulse::Night));
        // Force 1, at battle readiness from the start.
        assert!(world.force_is_ready(TaskForceId::new(1), 1, Pulse::Night));
    }

    #[test]
    fn a_force_alerted_by_a_contact_at_the_start_of_this_pulse_is_not_ready_in_it_but_is_in_the_next(
    ) {
        // Decision 0068, 2: alert_from_belief at the start of a pulse records
        // its event in that pulse, so the force it alerts is not ready in it.
        let mut world = lone_allied();
        give_contact(&mut world, Side::Allied, Hex::new(1, 0));
        world.alert_from_belief(1, Pulse::Night);
        assert!(force(&world, 1).alerted);
        let id = TaskForceId::new(1);
        assert!(!world.force_is_ready(id, 1, Pulse::Night));
        // Still alerted by the same contact in the next pulse: ready.
        assert!(world.force_is_ready(id, 1, Pulse::Morning));
    }

    #[test]
    fn a_force_not_ready_fires_the_share_of_its_guns_in_every_round_of_the_fight() {
        // Share 0.5: 8 x 3 x 5 = 120, x 0.5 = 60 shots, in all six rounds
        // though the force sights, is fired on and becomes alert in round 1.
        // Force 1 is ready and fires 120. Torpedoes are 8 and untouched.
        let world = run(&unready_scene(), |w| {
            harmless(w);
            unready_duel(w);
        });
        assert!(
            force(&world, 2).alerted,
            "force 2 became alert in the fight"
        );
        let theirs = gun_salvos_of(&world, 2);
        assert_eq!(theirs.len(), 6);
        for salvo in &theirs {
            assert_eq!(salvo.shots, Fixed::from_int(60), "round {}", salvo.round);
            assert_eq!(salvo.expected, salvo.shots * salvo.chance);
        }
        let ours = gun_salvos_of(&world, 1);
        assert_eq!(ours.len(), 6);
        assert!(ours.iter().all(|s| s.shots == Fixed::from_int(120)));
        for ship in [1, 2] {
            let torpedoes = torpedo_salvos_of(&world, ship);
            assert_eq!(torpedoes.len(), 1);
            assert_eq!(torpedoes[0].shots, Fixed::from_int(8));
        }
        // The ammunition used is the shots fired: 800 less 6 x 60.
        assert_eq!(ship(&world, 2).gun_rounds, Fixed::from_int(440));
        assert_eq!(ship(&world, 1).gun_rounds, Fixed::from_int(80));
    }

    #[test]
    fn the_share_multiplies_with_the_main_battery_factor_before_the_ammunition_cap() {
        // A degraded battery halves the shots: 120 x 0.5 x 0.5 = 30. With it
        // out there is no gun salvo at all. The cap comes after: 50 rounds
        // left make min(60, 50) = 50 shots, and 70 make 60, where without the
        // share 70 would make 70.
        let shots_with = |change: fn(&mut World)| {
            let world = run(&unready_scene(), |w| {
                harmless(w);
                unready_duel(w);
                change(w);
            });
            gun_salvos_of(&world, 2)
                .first()
                .map(|s| (s.shots, ship(&world, 2).gun_rounds))
        };
        let degraded = shots_with(|w| ship_mut(w, 2).systems.main_battery = SystemState::Degraded);
        assert_eq!(degraded.map(|d| d.0), Some(Fixed::from_int(30)));
        assert_eq!(
            shots_with(|w| ship_mut(w, 2).systems.main_battery = SystemState::Out),
            None
        );
        let fifty = shots_with(|w| ship_mut(w, 2).gun_rounds = Fixed::from_int(50));
        assert_eq!(fifty.map(|d| d.0), Some(Fixed::from_int(50)));
        let seventy = shots_with(|w| ship_mut(w, 2).gun_rounds = Fixed::from_int(70));
        assert_eq!(seventy.map(|d| d.0), Some(Fixed::from_int(60)));
    }

    #[test]
    fn the_unready_gun_share_is_in_a_not_ready_forces_gun_breakdowns_after_the_battery_factor() {
        let world = run(&unready_scene(), |w| {
            harmless(w);
            unready_duel(w);
        });
        let share_of = |event: &Event| {
            event
                .breakdown
                .as_ref()
                .expect("a breakdown")
                .factors
                .iter()
                .find(|f| f.name == "unready_gun_share")
                .map(|f| f.value)
        };
        for salvo in gun_salvos_of(&world, 2) {
            let event = event_with_id(&world, salvo.id);
            let names = factor_names(event);
            assert_eq!(
                &names[..6],
                &[
                    "guns",
                    "rate",
                    "minutes",
                    "main_battery_factor",
                    "unready_gun_share",
                    "base_chance"
                ]
            );
            assert_eq!(share_of(event), Some(fx("0.5")));
        }
        // None in a ready force's gun salvo, and none in a torpedo salvo.
        for salvo in gun_salvos_of(&world, 1) {
            assert_eq!(share_of(event_with_id(&world, salvo.id)), None);
        }
        for ship in [1, 2] {
            for salvo in torpedo_salvos_of(&world, ship) {
                assert_eq!(share_of(event_with_id(&world, salvo.id)), None);
            }
        }
    }

    #[test]
    fn at_a_share_of_1_nothing_changes_and_no_breakdown_names_the_factor() {
        let world = run(&unready_scene(), |w| {
            harmless(w);
            both_sight(w);
        });
        for salvo in salvos(&world) {
            let event = event_with_id(&world, salvo.id);
            assert!(!factor_names(event).contains(&"unready_gun_share"));
            if salvo.weapon == Weapon::Gun {
                assert_eq!(salvo.shots, Fixed::from_int(120));
            }
        }
    }

    #[test]
    fn readiness_is_decided_when_the_fight_begins_and_a_force_alerted_in_it_stays_not_ready() {
        // Force 2 is at normal readiness, never alerted; force 1 sights only
        // by roll. Force 2 is not ready at the start, becomes alert in round
        // 1 by being fired on, and fires 60 shots in every round it fires.
        let world = run(&unready_scene(), |w| {
            harmless(w);
            only_force_1_sights_by_roll(w);
            figures_mut(w).unready_gun_share = fx("0.5");
        });
        let theirs = gun_salvos_of(&world, 2);
        assert!(theirs.len() >= 2, "force 2 fired in several rounds");
        assert!(theirs.iter().all(|s| s.shots == Fixed::from_int(60)));
        assert!(force(&world, 2).alerted);
    }

    #[test]
    fn a_force_alerted_before_the_pulse_fires_its_full_salvo() {
        // Alerted in an earlier pulse (here an earlier day): ready. In the
        // same pulse: not ready.
        for (day, shots) in [(0, 120), (1, 60)] {
            let world = run(&unready_scene(), |w| {
                harmless(w);
                unready_duel(w);
                alerted_at(w, day, Pulse::Night);
            });
            assert!(
                gun_salvos_of(&world, 2)
                    .iter()
                    .all(|s| s.shots == Fixed::from_int(shots)),
                "alerted on day {day}"
            );
        }
        // At battle readiness by order: ready.
        let world = run(&unready_scene(), |w| {
            harmless(w);
            unready_duel(w);
            force_mut(w, 2).readiness = Readiness::Battle;
        });
        assert!(gun_salvos_of(&world, 2)
            .iter()
            .all(|s| s.shots == Fixed::from_int(120)));
    }

    #[test]
    fn a_force_alarmed_in_the_pulse_and_one_alerted_by_an_earlier_fight_of_it_are_not_ready() {
        // Four forces in one hex (see `crowded_hex`); force 3 is at normal
        // readiness. With the alarm sure, force 3 is alerted by the alarm of
        // the first fight (1,2) and fires 60 shots in (2,3) and (3,4). With
        // the alarm never sounding it is alerted by being fired on in (2,3),
        // and is not ready in (3,4) either. The three other forces are at
        // battle readiness and fire 120.
        for alarm in [Fixed::ONE, Fixed::ZERO] {
            let world = run(&crowded_hex(1), move |w| {
                alarm_chance_is(alarm)(w);
                figures_mut(w).unready_gun_share = fx("0.5");
            });
            // The ready forces' first salvos are full (later ones may be cut
            // by the ammunition after several fights).
            for ship in [1, 2, 4] {
                assert_eq!(gun_salvos_of(&world, ship)[0].shots, Fixed::from_int(120));
            }
            let theirs = gun_salvos_of(&world, 3);
            assert!(!theirs.is_empty());
            assert!(
                theirs.iter().all(|s| s.shots == Fixed::from_int(60)),
                "alarm chance {alarm}"
            );
            let by = force(&world, 3).alerted_by.expect("alerted");
            let kind_of_cause = &event_with_id(&world, by).kind;
            assert_eq!(
                matches!(kind_of_cause, EventKind::AlarmRoll { .. }),
                alarm == Fixed::ONE,
                "{kind_of_cause:?}"
            );
        }
    }

    // ---- a force not ready fires part of its torpedoes (decision 0071) ----

    /// Both forces sight in round 1 for sure, and the torpedo share is 0.5.
    fn unready_torpedo_duel(world: &mut World) {
        both_sight(world);
        figures_mut(world).unready_torpedo_share = fx("0.5");
    }

    #[test]
    fn a_force_not_ready_fires_the_share_of_its_tubes_in_its_one_torpedo_round() {
        // Share 0.5: 8 tubes x 1 x 0.5 = 4 shots, in a salvo that is the
        // force's only one. Force 1 is ready and fires 8. The guns are
        // untouched by this share (120 for each force in each round).
        let world = run(&unready_scene(), |w| {
            harmless(w);
            unready_torpedo_duel(w);
        });
        let theirs = torpedo_salvos_of(&world, 2);
        assert_eq!(theirs.len(), 1);
        assert_eq!(theirs[0].shots, Fixed::from_int(4));
        assert_eq!(theirs[0].expected, theirs[0].shots * theirs[0].chance);
        let ours = torpedo_salvos_of(&world, 1);
        assert_eq!(ours.len(), 1);
        assert_eq!(ours[0].shots, Fixed::from_int(8));
        for ship in [1, 2] {
            assert!(gun_salvos_of(&world, ship)
                .iter()
                .all(|s| s.shots == Fixed::from_int(120)));
        }
    }

    #[test]
    fn the_torpedo_share_multiplies_with_the_mount_factor() {
        // A degraded mount halves the shots: 8 x 0.5 x 0.5 = 2. With it out
        // there is no torpedo salvo at all.
        let shots_with = |state: SystemState| {
            let world = run(&unready_scene(), |w| {
                harmless(w);
                unready_torpedo_duel(w);
                ship_mut(w, 2).systems.torpedo_mounts = state;
            });
            torpedo_salvos_of(&world, 2).first().map(|s| s.shots)
        };
        assert_eq!(shots_with(SystemState::Degraded), Some(Fixed::from_int(2)));
        assert_eq!(shots_with(SystemState::Out), None);
    }

    #[test]
    fn the_unready_torpedo_share_is_in_a_not_ready_forces_torpedo_breakdowns_after_the_mount_factor(
    ) {
        let world = run(&unready_scene(), |w| {
            harmless(w);
            unready_torpedo_duel(w);
        });
        let share_of = |event: &Event| {
            event
                .breakdown
                .as_ref()
                .expect("a breakdown")
                .factors
                .iter()
                .find(|f| f.name == "unready_torpedo_share")
                .map(|f| f.value)
        };
        for salvo in torpedo_salvos_of(&world, 2) {
            let event = event_with_id(&world, salvo.id);
            let names = factor_names(event);
            assert_eq!(
                &names[..6],
                &[
                    "tubes",
                    "rate",
                    "minutes",
                    "torpedo_mount_factor",
                    "unready_torpedo_share",
                    "base_chance"
                ]
            );
            assert_eq!(share_of(event), Some(fx("0.5")));
        }
        // None in a ready force's torpedo salvo, and none in a gun salvo.
        for salvo in torpedo_salvos_of(&world, 1) {
            assert_eq!(share_of(event_with_id(&world, salvo.id)), None);
        }
        for ship in [1, 2] {
            for salvo in gun_salvos_of(&world, ship) {
                assert_eq!(share_of(event_with_id(&world, salvo.id)), None);
            }
        }
    }

    #[test]
    fn at_a_torpedo_share_of_1_nothing_changes_and_no_breakdown_names_the_factor() {
        let world = run(&unready_scene(), |w| {
            harmless(w);
            both_sight(w);
            figures_mut(w).unready_gun_share = fx("0.5");
        });
        for ship in [1, 2] {
            let torpedoes = torpedo_salvos_of(&world, ship);
            assert_eq!(torpedoes.len(), 1);
            assert_eq!(torpedoes[0].shots, Fixed::from_int(8));
        }
        for salvo in salvos(&world) {
            let event = event_with_id(&world, salvo.id);
            assert!(!factor_names(event).contains(&"unready_torpedo_share"));
        }
    }

    #[test]
    fn torpedoes_are_spent_after_a_round_fired_at_a_share() {
        // The four tubes that did not fire are not kept for a later round: the
        // ship has fired its torpedoes, and fires no further torpedo salvo.
        let world = run(&unready_scene(), |w| {
            harmless(w);
            unready_torpedo_duel(w);
        });
        assert_eq!(torpedo_salvos_of(&world, 2).len(), 1);
        assert!(
            gun_salvos_of(&world, 2).len() > 1,
            "later rounds were fought"
        );
        assert!(!ship(&world, 2).torpedoes_ready);
        assert!(!ship(&world, 1).torpedoes_ready);
    }

    #[test]
    fn a_force_with_only_torpedoes_left_still_has_a_salvo_at_a_share() {
        // `has_any_salvo` reads the share too: with no guns to fire, the
        // torpedoes at 8 x 0.5 = 4 shots make the salvo.
        let world = run(&unready_scene(), |w| {
            harmless(w);
            unready_torpedo_duel(w);
            ship_mut(w, 2).gun_rounds = Fixed::ZERO;
        });
        let theirs = torpedo_salvos_of(&world, 2);
        assert_eq!(theirs.len(), 1);
        assert_eq!(theirs[0].shots, Fixed::from_int(4));
    }

    #[test]
    fn recording_breakdowns_does_not_change_what_the_share_does() {
        let build = |record: bool| {
            let scene = unready_scene();
            let mut world = scene.world();
            world.config = SimConfig {
                record_breakdowns: record,
            };
            unready_duel(&mut world);
            scene.run_step(&mut world);
            world
        };
        assert_eq!(build(true).state_hash(), build(false).state_hash());
    }
}
