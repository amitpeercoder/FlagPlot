//! Surface combat: opposing task forces in one hex fight in rounds.
//!
//! This is the `SurfaceEngagements` phase of a pulse. The rule is
//! `docs/decisions/0014-surface-combat.md`; the random number keys follow
//! `docs/decisions/0013-random-keys-for-pairs.md`.
//!
//! Every pair of opposing task forces in one hex fights in turn, in
//! (lower ID, higher ID) order. A fight is a run of rounds. Each round is
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
//! The fight writes truth only in ship damage, ammunition, torpedoes, sinking
//! (the ship's location and its task force's set) and task force alertness.
//! The per-fight state lives in [`Fight`] and is gone when the fight ends.

use std::collections::{BTreeMap, BTreeSet};

use crate::belief::{ApplyOutcome, BattleClaim, Report};
use crate::events::{
    Breakdown, Cause, DamageTrack, EngagementEndReason, EventKind, Factor, Weapon,
};
use crate::fixed::Fixed;
use crate::hex::Hex;
use crate::ids::{EntityRef, EventId, OrderId, ShipId, TaskForceId};
use crate::orders::Stance;
use crate::refdata::{ShipClass, ShipKind, SurfaceCombatFigures};
use crate::rng::{RngStream, Subsystem};
use crate::search::build_sighting_report;
use crate::state::{Ship, ShipLocation, Side};
use crate::time::{self, Pulse};
use crate::world::World;

/// A damage track at its limit.
const FULL_DAMAGE: Fixed = Fixed::from_int(100);

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
}

impl Fight {
    /// The index of the other force.
    const fn enemy(index: usize) -> usize {
        1 - index
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
    /// Hits on flotation, system and fire, in [`DamageTrack::ALL`] order.
    track_hits: [u32; 3],
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
    /// firer's system damage.
    shot_factors: [Fixed; 4],
    damage_per_hit: Fixed,
}

/// A chance as an unsigned number to compare a draw against.
fn raw_u64(value: Fixed) -> u64 {
    u64::try_from(value.raw()).expect("a chance is not negative")
}

/// The gun shots a ship would fire in a round of `minutes`: guns x rate x
/// minutes, cut by system damage (multiplied first, divided once), then
/// capped by the ammunition aboard.
fn gun_shots(ship: &Ship, class: &ShipClass, minutes: u32) -> Fixed {
    let full = Fixed::from_int(
        i64::from(class.main_guns) * i64::from(class.gun_rounds_per_min) * i64::from(minutes),
    );
    full.mul_div(FULL_DAMAGE - ship.system_damage, FULL_DAMAGE)
        .min(ship.gun_rounds)
}

/// True if the ship has torpedoes to fire.
fn has_torpedo_salvo(ship: &Ship, class: &ShipClass) -> bool {
    ship.torpedoes_ready && class.torpedo_tubes > 0
}

fn track_value(ship: &mut Ship, track: DamageTrack) -> &mut Fixed {
    match track {
        DamageTrack::Flotation => &mut ship.flotation_damage,
        DamageTrack::System => &mut ship.system_damage,
        DamageTrack::Fire => &mut ship.fire_damage,
    }
}

impl World {
    /// The surface engagement phase of one pulse: every pair of opposing task
    /// forces in one hex fights, in (lower ID, higher ID) order. A pair is
    /// skipped when its turn comes if either force has no ship afloat, so a
    /// later pair sees what earlier pairs did.
    pub(crate) fn surface_engagements(&mut self, day: u32, pulse: Pulse) {
        let Some(figures) = self.ref_data.surface_combat.clone() else {
            return;
        };
        for (a, b) in self.opposing_pairs_in_one_hex() {
            if self.truth.task_forces[&a].ships.is_empty()
                || self.truth.task_forces[&b].ships.is_empty()
            {
                continue;
            }
            let mut fight = self.begin_fight(day, pulse, &figures, a, b);
            let ending = self.fight_rounds(&mut fight);
            self.end_fight(&fight, &ending);
        }
    }

    /// Opposing task forces with the same position, lower ID first, in order.
    fn opposing_pairs_in_one_hex(&self) -> Vec<(TaskForceId, TaskForceId)> {
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
        a: TaskForceId,
        b: TaskForceId,
    ) -> Fight {
        let hex = self.truth.task_forces[&a].position();
        let causes: Vec<Cause> = [a, b]
            .into_iter()
            .filter_map(|id| {
                let force = &self.truth.task_forces[&id];
                force.rules_order.or(force.route_order)
            })
            .map(Cause::Order)
            .collect();
        let begun = self.events.record(
            day,
            Some(pulse),
            EventKind::EngagementBegun { a, b, hex },
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
                sunk_in_fight: BTreeSet::new(),
                sighting_stream: world.rng_pair(
                    day,
                    Some(pulse),
                    Subsystem::SurfaceCombat,
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
            let alert_factor = if self.truth.task_forces[&id].alert {
                Fixed::ONE
            } else {
                figures.not_alert_sighting_factor
            };
            let chance = ((base + radar_bonus) * alert_factor).min(Fixed::ONE);
            let sighted = fight.forces[index].sighting_stream.chance(chance);

            let breakdown = self.config.record_breakdowns.then(|| Breakdown {
                factors: vec![
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
                ],
                result: chance,
            });
            let event = self.record_in_fight(
                fight,
                EventKind::SightingRoll {
                    round: round.number,
                    task_force: id,
                    enemy,
                    chance,
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
    fn avoid(&mut self, fight: &Fight, round: &Round) -> Option<EventId> {
        for index in 0..2 {
            let force = &fight.forces[index];
            let enemy = &fight.forces[Fight::enemy(index)];
            let task_force = &self.truth.task_forces[&force.id];
            if task_force.rules_of_engagement.stance != Stance::Avoid
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
            let event = self.record_in_fight(
                fight,
                EventKind::ForceSlippedAway {
                    round: round.number,
                    task_force: force.id,
                    enemy: enemy.id,
                },
                causes,
                None,
            );
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
                    .any(|ship| self.has_any_salvo(fight, *ship));
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

        // Ships are matched in ID order: firer i takes target i mod count.
        for index in 0..2 {
            if !fight.forces[index].sighted {
                continue;
            }
            let enemy = Fight::enemy(index);
            for (place, firer) in afloat[index].iter().enumerate() {
                let target = afloat[enemy][place % afloat[enemy].len()];
                self.fire_ship(fight, round, index, *firer, target);
            }
        }
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
    fn has_any_salvo(&self, fight: &Fight, ship: ShipId) -> bool {
        let state = &self.truth.ships[&ship];
        let class = self.class_of(ship);
        gun_shots(state, class, fight.figures.round_minutes) > Fixed::ZERO
            || has_torpedo_salvo(state, class)
    }

    /// One ship's fire for the round: a gun salvo if it has shots, then a
    /// torpedo salvo if its torpedoes are ready. Both go at the same target.
    fn fire_ship(
        &mut self,
        fight: &mut Fight,
        round: &mut Round,
        force: usize,
        firer: ShipId,
        target: ShipId,
    ) {
        let enemy = Fight::enemy(force);
        let ship = &self.truth.ships[&firer];
        let class = self.class_of(firer);
        let target_class = self.class_of(target);
        let figures = &fight.figures;
        let shots = gun_shots(ship, class, figures.round_minutes);
        let torpedoes = has_torpedo_salvo(ship, class);
        let per_hit = |damage: u32| {
            Fixed::from_ratio(100 * i64::from(damage), i64::from(target_class.toughness))
        };
        let gun = (shots > Fixed::ZERO).then(|| Shot {
            firer,
            target,
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
                Fixed::from_int(i64::from(class.gun_rounds_per_min)),
                Fixed::from_int(i64::from(figures.round_minutes)),
                ship.system_damage,
            ],
            damage_per_hit: per_hit(class.gun_damage),
        });
        let torpedo = torpedoes.then(|| Shot {
            firer,
            target,
            target_force: enemy,
            weapon: Weapon::Torpedo,
            shots: Fixed::from_int(i64::from(class.torpedo_tubes)),
            base_chance: if fight.dark {
                figures.torpedo_hit_chance_night
            } else {
                figures.torpedo_hit_chance_day
            },
            shot_factors: [
                Fixed::from_int(i64::from(class.torpedo_tubes)),
                Fixed::ONE,
                Fixed::ONE,
                Fixed::ZERO,
            ],
            damage_per_hit: per_hit(class.torpedo_damage),
        });

        // The stream is opened the first time the ship fires and kept for
        // the whole fight.
        let mut stream = fight.fire_streams.remove(&firer).unwrap_or_else(|| {
            self.rng_pair(
                fight.day,
                Some(fight.pulse),
                Subsystem::SurfaceCombat,
                EntityRef::Ship(firer),
                EntityRef::TaskForce(fight.forces[enemy].id),
            )
        });
        if let Some(gun) = gun {
            let ship = self.truth.ships.get_mut(&firer).expect("firer exists");
            ship.gun_rounds -= gun.shots;
            self.fire_weapon(fight, round, force, &mut stream, &gun);
        }
        if let Some(torpedo) = torpedo {
            self.truth
                .ships
                .get_mut(&firer)
                .expect("firer exists")
                .torpedoes_ready = false;
            self.fire_weapon(fight, round, force, &mut stream, &torpedo);
        }
        fight.fire_streams.insert(firer, stream);
    }

    /// Rolls one salvo and records it. Draws, in order: the fraction of the
    /// expected hits, then for a gun one track for each hit.
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
        let chance = shot.base_chance * surprised_factor * lit_factor;
        let expected = shot.shots * chance;
        let whole = expected.trunc();
        let fraction = expected - Fixed::from_int(whole);
        let hits = u32::try_from(whole).expect("expected hits are not negative")
            + u32::from(stream.chance(fraction));

        let mut track_hits = [0u32; 3];
        if shot.weapon == Weapon::Gun {
            let flotation = raw_u64(figures.gun_hit_flotation);
            let system = flotation + raw_u64(figures.gun_hit_system);
            for _ in 0..hits {
                let draw = stream.below(Fixed::SCALE.unsigned_abs());
                let track = if draw < flotation {
                    DamageTrack::Flotation
                } else if draw < system {
                    DamageTrack::System
                } else {
                    DamageTrack::Fire
                };
                track_hits[track as usize] += 1;
            }
        } else {
            track_hits[DamageTrack::Flotation as usize] = hits;
        }

        let breakdown = self.config.record_breakdowns.then(|| {
            let [count, rate, minutes, system_damage] = shot.shot_factors;
            let count_name = match shot.weapon {
                Weapon::Gun => "guns",
                Weapon::Torpedo => "tubes",
            };
            let factor = |name, value| Factor { name, value };
            Breakdown {
                factors: vec![
                    factor(count_name, count),
                    factor("rate", rate),
                    factor("minutes", minutes),
                    factor("system_damage", system_damage),
                    factor("base_chance", shot.base_chance),
                    factor("surprised_factor", surprised_factor),
                    factor("lit_factor", lit_factor),
                ],
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
        let against = &mut round.forces[shot.target_force];
        against.fired_on = true;
        against.first_salvo_against.get_or_insert(event);
        round.salvos.push(SalvoRecord {
            event,
            target: shot.target,
            target_force: shot.target_force,
            damage_per_hit: shot.damage_per_hit,
            track_hits,
        });
    }

    // ---- (d) damage ----

    /// Damage lands, in salvo order: one `ShipDamaged` event per salvo per
    /// track that took hits. A ship whose flotation reaches 100 sinks.
    fn apply_damage(&mut self, fight: &mut Fight, round: &mut Round) {
        for salvo_index in 0..round.salvos.len() {
            let (salvo_event, target, target_force, damage_per_hit, track_hits) = {
                let salvo = &round.salvos[salvo_index];
                (
                    salvo.event,
                    salvo.target,
                    salvo.target_force,
                    salvo.damage_per_hit,
                    salvo.track_hits,
                )
            };
            for track in DamageTrack::ALL {
                let hits = track_hits[track as usize];
                if hits == 0 {
                    continue;
                }
                let ship = self.truth.ships.get_mut(&target).expect("target exists");
                let value = track_value(ship, track);
                let before = *value;
                let points = damage_per_hit * i64::from(hits);
                let after = (before + points).min(FULL_DAMAGE);
                *value = after;
                let damaged = self.record_in_fight(
                    fight,
                    EventKind::ShipDamaged {
                        round: round.number,
                        ship: target,
                        track,
                        hits,
                        points,
                        value_after: after,
                    },
                    vec![Cause::Event(salvo_event)],
                    None,
                );
                round.damaged[target_force].push(damaged);
                if track == DamageTrack::Flotation && before < FULL_DAMAGE && after >= FULL_DAMAGE {
                    let sunk = self.sink(fight, round, target_force, target, damaged);
                    round.sunk.push(sunk);
                }
            }
        }
    }

    /// Takes a ship out of its task force and records it.
    fn sink(
        &mut self,
        fight: &mut Fight,
        round: &Round,
        force: usize,
        ship: ShipId,
        cause: EventId,
    ) -> EventId {
        let task_force = fight.forces[force].id;
        self.truth
            .ships
            .get_mut(&ship)
            .expect("ship exists")
            .location = ShipLocation::Sunk;
        self.truth
            .task_forces
            .get_mut(&task_force)
            .expect("task force exists")
            .ships
            .remove(&ship);
        fight.forces[force].sunk_in_fight.insert(ship);
        self.record_in_fight(
            fight,
            EventKind::ShipSunk {
                round: round.number,
                ship,
                task_force,
            },
            vec![Cause::Event(cause)],
            None,
        )
    }

    /// A force that sighted the enemy this round, or was fired on this
    /// round, and is not alert becomes alert.
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
            if task_force.alert {
                continue;
            }
            task_force.alert = true;
            let cause = this_round
                .rolled
                .or(this_round.first_salvo_against)
                .unwrap_or(fight.begun);
            self.record_in_fight(
                fight,
                EventKind::ForceBecameAlert {
                    round: round.number,
                    task_force: force.id,
                },
                vec![Cause::Event(cause)],
                None,
            );
        }
    }

    // ---- (e) ending ----

    /// The ending checks, in order: a force with no ship afloat, break-off,
    /// the last round. Returns how the fight ended, or `None` if it goes on.
    fn check_ending(&mut self, fight: &Fight, round: &Round) -> Option<Ending> {
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
    fn break_off(&mut self, fight: &Fight, round: &Round, index: usize) -> Option<EventId> {
        let force = &fight.forces[index];
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
        let avoiding_under_fire = rules.stance == Stance::Avoid && round.forces[index].fired_on;
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
        Some(self.record_in_fight(
            fight,
            EventKind::ForceBrokeOff {
                round: round.number,
                task_force: force.id,
                share,
            },
            causes,
            breakdown,
        ))
    }

    // ---- after the fight ----

    /// Records the end of the fight, then each force reports and claims, the
    /// lower ID first.
    fn end_fight(&mut self, fight: &Fight, ending: &Ending) {
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
                confidence: report.confidence,
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
    use crate::orders::Mission;
    use crate::refdata::{PortSize, RefData, SearchFigures};
    use crate::state::{RulesOfEngagement, TaskForce, TruthState};
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
    /// alert, and its rules of engagement.
    #[derive(Clone)]
    struct ForceSpec {
        id: u32,
        side: Side,
        hex: Hex,
        classes: Vec<u32>,
        alert: bool,
        rules: RulesOfEngagement,
    }

    impl ForceSpec {
        fn new(id: u32, side: Side, hex: Hex, classes: &[u32]) -> ForceSpec {
            ForceSpec {
                id,
                side,
                hex,
                classes: classes.to_vec(),
                alert: true,
                rules: RulesOfEngagement::DEFAULT,
            }
        }

        fn not_alert(mut self) -> ForceSpec {
            self.alert = false;
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
                        gun_rounds_per_min: rate,
                        gun_rounds_per_gun: rounds,
                        gun_damage,
                        torpedo_tubes: tubes,
                        torpedo_damage,
                        toughness: tough,
                        radar,
                    },
                );
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
                force.alert = spec.alert;
                force.rules_of_engagement = spec.rules;
                for class in &spec.classes {
                    let ship_id = ShipId::new(next_ship);
                    next_ship += 1;
                    let class_data = &ref_data.ship_classes[&ShipClassId::new(*class)];
                    force.ships.insert(ship_id);
                    truth.ships.insert(
                        ship_id,
                        Ship {
                            id: ship_id,
                            name: format!("Test ship {}", ship_id.raw()),
                            class: ShipClassId::new(*class),
                            side: spec.side,
                            fuel_t: Fixed::from_int(2000),
                            supply_t: Fixed::ZERO,
                            location: ShipLocation::TaskForce(id),
                            flotation_damage: Fixed::ZERO,
                            system_damage: Fixed::ZERO,
                            fire_damage: Fixed::ZERO,
                            gun_rounds: Fixed::from_int(i64::from(
                                class_data.main_guns * class_data.gun_rounds_per_gun,
                            )),
                            torpedoes_ready: class_data.torpedo_tubes > 0,
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
            world.surface_engagements(1, self.pulse);
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
    /// lands on the fire track (which does nothing by day) and torpedoes
    /// always miss. A fight with these figures runs to the last round.
    fn harmless(world: &mut World) {
        let figures = figures_mut(world);
        figures.gun_hit_flotation = Fixed::ZERO;
        figures.gun_hit_system = Fixed::ZERO;
        figures.gun_hit_fire = Fixed::ONE;
        figures.torpedo_hit_chance_night = Fixed::ZERO;
        figures.torpedo_hit_chance_day = Fixed::ZERO;
    }

    /// Every gun hit lands on flotation and torpedoes always miss, so by
    /// day a salvo of 120 shots does exactly 6 hits x 5 points = 30.
    fn flotation_only(world: &mut World) {
        let figures = figures_mut(world);
        figures.gun_hit_flotation = Fixed::ONE;
        figures.gun_hit_system = Fixed::ZERO;
        figures.gun_hit_fire = Fixed::ZERO;
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
        // x 0.05 = 6 hits (a whole number, so no fraction roll hits), each 100
        // x 20 / 400 = 5 points: 30 points per salvo.
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
        // 50 x 0.05 = 2.5 expected: 2 whole hits, and 3 if the fraction hits.
        assert_eq!(guns[0].expected, fx("2.5"));
        assert!(guns[0].hits == 2 || guns[0].hits == 3);
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

    // ---- 8. system damage ----

    #[test]
    fn a_ship_with_100_system_damage_fires_no_guns_but_still_its_torpedoes() {
        // Shots = 120 x (100 - 100) / 100 = 0: no gun salvo (no event), and
        // the torpedoes go in round 1.
        let hex = Hex::new(5, 0);
        let scene = day_scene(vec![
            ForceSpec::new(1, Side::Japanese, hex, &[1]),
            ForceSpec::new(2, Side::Allied, hex, &[1]),
        ]);
        let world = run(&scene, |w| {
            harmless(w);
            ship_mut(w, 1).system_damage = Fixed::from_int(100);
        });
        assert!(gun_salvos_of(&world, 1).is_empty());
        let torpedoes = torpedo_salvos_of(&world, 1);
        assert_eq!(torpedoes.len(), 1);
        assert_eq!(torpedoes[0].round, 1);
        assert_eq!(ship(&world, 1).gun_rounds, Fixed::from_int(800));
    }

    #[test]
    fn fifteen_system_damage_leaves_102_shots() {
        // Shots = 120 x (100 - 15) / 100 = 120 x 85 / 100 = 102, in each of
        // the six rounds (harmless: no further system damage), using 6 x 102
        // = 612 of the 800 rounds.
        let hex = Hex::new(5, 0);
        let scene = day_scene(vec![
            ForceSpec::new(1, Side::Japanese, hex, &[1]),
            ForceSpec::new(2, Side::Allied, hex, &[1]),
        ]);
        let world = run(&scene, |w| {
            harmless(w);
            ship_mut(w, 1).system_damage = Fixed::from_int(15);
        });
        let guns = gun_salvos_of(&world, 1);
        assert_eq!(guns.len(), 6);
        assert!(guns.iter().all(|s| s.shots == Fixed::from_int(102)));
        assert_eq!(ship(&world, 1).gun_rounds, Fixed::from_int(188));
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

    // ---- 16. the fire stream draw count ----

    #[test]
    fn the_fire_stream_draws_the_fraction_even_when_it_is_zero_then_tracks_then_the_torpedo() {
        // Day, one cruiser each, torpedo chance set to 0 (no torpedo hit, but
        // the fraction is still rolled). A gun salvo is 120 shots x 0.05 = 6
        // expected hits: a whole number, fraction 0. The stream of ship 1
        // (keyed on ship 1 and task force 2) is drawn in this order:
        //   round 1: gun fraction (1 draw), 6 track draws, torpedo fraction
        //   round 2: gun fraction (1 draw), 6 track draws
        // Replaying the same stream by hand must give the same hits on each
        // track in both rounds, which fails if the fraction draw is skipped
        // when it is 0 or if the torpedo fraction is skipped.
        let hex = Hex::new(5, 0);
        let scene = day_scene(vec![
            ForceSpec::new(1, Side::Japanese, hex, &[1]),
            ForceSpec::new(2, Side::Allied, hex, &[1]),
        ]);
        let world = run(&scene, |w| {
            let figures = figures_mut(w);
            figures.torpedo_hit_chance_day = Fixed::ZERO;
            // No hit lands on the system track, which would cut the shots
            // (and so the whole expected hits) in round 2.
            figures.gun_hit_system = Fixed::ZERO;
            figures.gun_hit_fire = fx("0.5");
        });
        let mut stream = world.rng_pair(
            1,
            Some(Pulse::Morning),
            Subsystem::SurfaceCombat,
            EntityRef::Ship(ShipId::new(1)),
            EntityRef::TaskForce(TaskForceId::new(2)),
        );
        let figures = world.ref_data.surface_combat.clone().unwrap();
        let flotation = u64::try_from(figures.gun_hit_flotation.raw()).unwrap();
        let system = flotation + u64::try_from(figures.gun_hit_system.raw()).unwrap();

        for round in 1..=2 {
            // The fraction is 0, so this draw is false whatever its value.
            assert!(!stream.chance(Fixed::ZERO));
            let mut expected = [0u32; 3];
            for _ in 0..6 {
                let draw = stream.below(Fixed::SCALE.unsigned_abs());
                let track = if draw < flotation {
                    0
                } else if draw < system {
                    1
                } else {
                    2
                };
                expected[track] += 1;
            }
            if round == 1 {
                // The torpedo fraction (0.16 x 0 = 0): one draw.
                assert!(!stream.chance(Fixed::ZERO));
            }

            let salvo = gun_salvos_of(&world, 1)
                .into_iter()
                .find(|s| s.round == round)
                .expect("a gun salvo this round");
            assert_eq!(salvo.expected, Fixed::from_int(6));
            assert_eq!(salvo.hits, 6);
            let mut actual = [0u32; 3];
            for event in world.events().iter() {
                if let EventKind::ShipDamaged { track, hits, .. } = event.kind {
                    if event.causes == vec![Cause::Event(salvo.id)] {
                        actual[track as usize] += hits;
                    }
                }
            }
            assert_eq!(actual, expected, "round {round}: tracks line up");
        }
    }
}
