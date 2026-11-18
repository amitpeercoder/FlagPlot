//! Step (c) of a round: every ship of a force that has sighted fires
//! (decisions 0014, 0016 and 0083).

use super::*;

/// The gun shots a ship would fire in a round of `minutes`: guns x rate x
/// minutes x the main battery factor x `share` (the share of its guns a force
/// not ready fires, 1 for a force that is ready: decision 0068), then capped
/// by the ammunition aboard (decision 6 of
/// `docs/decisions/0016-systems-and-stragglers.md`).
pub(crate) fn gun_shots(
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
///
/// A salvo fires only the tubes that are loaded (decision 0122, 8): no more
/// than the tubes less those fired and not loaded again. A ship whose tubes
/// were never reloaded fires the salvo it always did.
fn torpedo_salvo_tubes(ship: &Ship, class: &ShipClass) -> u32 {
    let side = if ship.torpedoes_one_side_fired {
        class.torpedo_tubes - class.torpedo_tubes_one_side
    } else {
        class.torpedo_tubes_one_side
    };
    let loaded =
        class.torpedo_tubes - ship.tubes_fired(class.torpedo_tubes, class.torpedo_tubes_one_side);
    side.min(loaded)
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
    // ---- (c) fire ----

    /// Every ship afloat at the start of the round, of every force that has
    /// sighted, fires. Salvos are rolled now and kept in `round`; damage
    /// waits for [`World::apply_damage`].
    pub(super) fn fire(&mut self, fight: &mut Fight, round: &mut Round) {
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

    pub(super) fn ships_afloat(&self, force: TaskForceId) -> Vec<ShipId> {
        self.truth.task_forces[&force]
            .ships
            .iter()
            .copied()
            .collect()
    }

    pub(super) fn class_of(&self, ship: ShipId) -> &ShipClass {
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
        let salvo_tubes = torpedo_salvo_tubes(ship, class);
        let fired_before = ship.tubes_fired(class.torpedo_tubes, class.torpedo_tubes_one_side);
        let second_side = ship.torpedoes_one_side_fired;
        let class_has_second_side = class.torpedo_tubes > class.torpedo_tubes_one_side;
        let class_tubes = class.torpedo_tubes;
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
            // The tubes of the salvo have fired (decision 0122, 8).
            let fired = (fired_before + salvo_tubes).min(class_tubes);
            if !second_side && class_has_second_side && fired < class_tubes {
                ship.torpedoes_one_side_fired = true;
                ship.torpedoes_fired = fired;
            } else {
                ship.torpedoes_ready = false;
                ship.torpedoes_one_side_fired = false;
                ship.torpedoes_fired = class_tubes;
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
}
