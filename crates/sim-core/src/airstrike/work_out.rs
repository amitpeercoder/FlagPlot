//! Working out a raid, an attack on ships and an attack on a base, on the
//! state at the start of the air combat step (decisions 0026, 0027 and 0034).

use super::*;

impl World {
    /// Works out one raid on the state at the start of the step: the
    /// escorts it has, the fights in the air, the guns, and then what each
    /// strike group that has bombers left does. Reads truth, which the
    /// umpire may; writes nothing.
    pub(super) fn work_out_raid(
        &self,
        day: u32,
        pulse: Pulse,
        raid: &Raid,
        fighters: &[FighterGroup],
        meets: PatrolShare,
    ) -> RaidResult {
        let first = &raid.strikes[0];
        let escorts_sent: Vec<(AirGroupId, u32, Vec<Cause>)> = raid
            .escorts
            .iter()
            .map(|index| {
                let group = &fighters[*index];
                (group.air_group, group.escorts, group.orders.clone())
            })
            .collect();

        let Some(target) = self.raid_target(first) else {
            // A raid that finds nothing meets nobody (decision 13 of 0027).
            // Only a strike at a contact can find nothing.
            return RaidResult {
                target: None,
                escorts_sent,
                rolls: Vec::new(),
                patrol_after_attack: None,
                after_attack: None,
                outcomes: raid
                    .strikes
                    .iter()
                    .filter_map(|strike| {
                        let Aim::Contact(contact) = strike.aim else {
                            return None;
                        };
                        Some((
                            strike.air_group,
                            Outcome::Nothing {
                                air_group: strike.air_group,
                                side: strike.side,
                                contact,
                                hex: strike.hex,
                                launched: strike.launched,
                            },
                        ))
                    })
                    .collect(),
            };
        };
        let defending_side = self.defender_of(target);
        // What the fighters' streams are keyed on besides their group.
        let target_entity = match target {
            RaidTarget::Ships(task_force) => EntityRef::TaskForce(task_force),
            RaidTarget::Base(base) => EntityRef::Base(base),
        };
        let air = self
            .ref_data
            .air_combat
            .as_ref()
            .expect("a strike needs the air combat figures");

        // The pools the hits are taken from, each in group order.
        let mut bombers: Vec<(AirGroupId, u32)> = raid
            .strikes
            .iter()
            .map(|strike| (strike.air_group, strike.aircraft))
            .collect();
        let mut escorts: Vec<(AirGroupId, u32)> = raid
            .escorts
            .iter()
            .map(|index| (fighters[*index].air_group, fighters[*index].escorts))
            .filter(|(_, aircraft)| *aircraft > 0)
            .collect();
        let patrol: Vec<(AirGroupId, u32)> = fighters
            .iter()
            .filter(|group| {
                group.side == defending_side && group.hex == Some(first.hex) && group.patrol > 0
            })
            .map(|group| (group.air_group, group.patrol))
            .collect();
        let escort_count: u32 = escorts.iter().map(|(_, aircraft)| aircraft).sum();
        // The raid meets its share of the patrol and no more (decision 0033).
        let patrol_count = meets.share;
        let patrol: Vec<(AirGroupId, u32)> = if patrol_count > 0 { patrol } else { Vec::new() };
        // Decision 0080: whom the share fires at, on the start-of-step counts.
        let bomber_count: u32 = bombers.iter().map(|(_, aircraft)| aircraft).sum();
        let patrol_drawn_off_share = air.patrol_drawn_off_share;
        let split = patrol_split(
            patrol_drawn_off_share,
            patrol_count,
            escort_count,
            bomber_count,
        );

        // Decision 0087: the share of the patrol's shots at the bombers that
        // are worked before the attack.
        let patrol_before = air.patrol_before_attack_share;
        let mut patrol_after_attack: Option<FireRoll> = None;
        let mut rolls = Vec::new();
        let breakdown = |shots: Fixed, chance: ShotChance, expected: Fixed| {
            self.config.record_breakdowns.then(|| {
                let mut factors = vec![
                    Factor {
                        name: "shots",
                        value: shots,
                    },
                    Factor {
                        name: "hit_chance",
                        value: chance.base,
                    },
                ];
                if let Some(value) = chance.armour {
                    factors.push(Factor {
                        name: "pilot_armour_factor",
                        value,
                    });
                }
                if let Some(value) = chance.tanks {
                    factors.push(Factor {
                        name: "self_sealing_tanks_factor",
                        value,
                    });
                }
                Breakdown {
                    factors,
                    result: expected,
                }
            })
        };

        // The factors of the share go on the patrol's own fire (decision
        // 0033, 7), before the shots and chance.
        let patrol_breakdown = |breakdown: Option<Breakdown>| {
            breakdown.map(|mut breakdown| {
                let mut factors = vec![
                    Factor {
                        name: "patrol",
                        value: Fixed::from_int(i64::from(meets.patrol)),
                    },
                    Factor {
                        name: "raid_aircraft",
                        value: Fixed::from_int(i64::from(meets.raid_aircraft)),
                    },
                    Factor {
                        name: "all_raids_aircraft",
                        value: Fixed::from_int(i64::from(meets.all_aircraft)),
                    },
                    Factor {
                        name: "patrol_share",
                        value: Fixed::from_int(i64::from(meets.share)),
                    },
                ];
                // Decision 0080: below a share of 1, the split's own factors.
                if patrol_drawn_off_share < Fixed::ONE {
                    factors.push(Factor {
                        name: "patrol_drawn_off_share",
                        value: patrol_drawn_off_share,
                    });
                    factors.push(Factor {
                        name: "drawn_off",
                        value: Fixed::from_int(i64::from(split.drawn_off)),
                    });
                    factors.push(Factor {
                        name: "rest_at_escorts",
                        value: Fixed::from_int(i64::from(split.rest_at_escorts)),
                    });
                }
                factors.append(&mut breakdown.factors);
                breakdown.factors = factors;
                breakdown
            })
        };

        if let (Some((first_escort, _)), Some((first_patrol, _))) =
            (escorts.first().copied(), patrol.first().copied())
        {
            // Escorts at the patrol, then the patrol at the escorts and the
            // bombers. Each fires on the start-of-step counts.
            let mut stream = self.rng_pair(
                day,
                Some(pulse),
                Subsystem::AirCombat,
                Draw::Legacy,
                EntityRef::AirGroup(first_escort),
                target_entity,
            );
            let shot = self.shot_chance(air.fighter_hit_chance, first_patrol, air);
            let (shots, expected, hits, hit_shots) =
                roll_hits(&mut stream, escort_count, shot.chance);
            rolls.push(FireRoll {
                fire: Fire::Fighters {
                    firer: first_escort,
                    target: first_patrol,
                    firing: AirFirer::Escorts,
                    at: AirTarget::Patrol,
                },
                shots,
                chance: shot.chance,
                expected,
                hits,
                lost: Vec::new(),
                patrol_of: Some((defending_side, first.hex)),
                breakdown: breakdown(shots, shot, expected),
                in_firing: escorts.clone(),
                hit_shots,
            });
        }
        if let Some((first_patrol, _)) = patrol.first().copied() {
            let mut stream = self.rng_pair(
                day,
                Some(pulse),
                Subsystem::AirCombat,
                Draw::Legacy,
                EntityRef::AirGroup(first_patrol),
                EntityRef::AirGroup(first.air_group),
            );
            let at_escorts = split.at_escorts;
            // A roll with no shots records nothing (decision 0080).
            if let Some((first_escort, _)) = escorts.first().copied().filter(|_| at_escorts > 0) {
                let shot = self.shot_chance(air.fighter_hit_chance, first_escort, air);
                let (shots, expected, hits, hit_shots) =
                    roll_hits(&mut stream, at_escorts, shot.chance);
                rolls.push(FireRoll {
                    fire: Fire::Fighters {
                        firer: first_patrol,
                        target: first_escort,
                        firing: AirFirer::Patrol,
                        at: AirTarget::Escorts,
                    },
                    shots,
                    chance: shot.chance,
                    expected,
                    hits,
                    lost: take_hits(hits, &mut escorts),
                    patrol_of: None,
                    breakdown: patrol_breakdown(breakdown(shots, shot, expected)),
                    in_firing: patrol.clone(),
                    hit_shots,
                });
            }
            let at_bombers = split.at_bombers;
            if at_bombers > 0 {
                let shot = self.shot_chance(air.fighter_hit_chance, first.air_group, air);
                // Decision 0087: the shots at the bombers are split. The first
                // k = share times n, cut to a whole number, are worked before
                // the attack, the other n - k after it, rolled in order from
                // the one stream, so the draws are the same at every share.
                let before_shots =
                    u32::try_from((Fixed::from_int(i64::from(at_bombers)) * patrol_before).trunc())
                        .expect("a share from 0 to 1 of the shots is not negative")
                        .min(at_bombers);
                // Below a share of 1 the roll names the shots at the bombers
                // in all and the share, after the factors of the patrol's
                // fire and before the part's shots.
                let bombers_breakdown = |shots: Fixed, expected: Fixed| {
                    patrol_breakdown(breakdown(shots, shot, expected)).map(|mut breakdown| {
                        if patrol_before < Fixed::ONE {
                            let at = breakdown
                                .factors
                                .iter()
                                .position(|factor| factor.name == "shots")
                                .expect("a roll's breakdown has its shots");
                            breakdown.factors.insert(
                                at,
                                Factor {
                                    name: "patrol_before_attack_share",
                                    value: patrol_before,
                                },
                            );
                            breakdown.factors.insert(
                                at,
                                Factor {
                                    name: "patrol_shots_at_bombers",
                                    value: Fixed::from_int(i64::from(at_bombers)),
                                },
                            );
                        }
                        breakdown
                    })
                };
                let fire = Fire::Fighters {
                    firer: first_patrol,
                    target: first.air_group,
                    firing: AirFirer::Patrol,
                    at: AirTarget::Bombers,
                };
                if before_shots > 0 {
                    let (shots, expected, hits, hit_shots) =
                        roll_hits(&mut stream, before_shots, shot.chance);
                    rolls.push(FireRoll {
                        fire: fire.clone(),
                        shots,
                        chance: shot.chance,
                        expected,
                        hits,
                        lost: take_hits(hits, &mut bombers),
                        patrol_of: None,
                        breakdown: bombers_breakdown(shots, expected),
                        in_firing: patrol.clone(),
                        hit_shots,
                    });
                }
                let after_shots = at_bombers - before_shots;
                if after_shots > 0 {
                    // Rolled now, from the same stream, but its hits come off
                    // the bombers after the attack is worked out.
                    let (shots, expected, hits, hit_shots) =
                        roll_hits(&mut stream, after_shots, shot.chance);
                    patrol_after_attack = Some(FireRoll {
                        fire,
                        shots,
                        chance: shot.chance,
                        expected,
                        hits,
                        lost: Vec::new(),
                        patrol_of: None,
                        breakdown: bombers_breakdown(shots, expected),
                        in_firing: patrol.clone(),
                        hit_shots,
                    });
                }
            }
        }

        // The guns of the target, at the bombers left: of every ship afloat
        // in the force attacked and in its escorts in station (decision 0121,
        // 6), or of the base attacked (decision 5 of 0034). Ships do not fire
        // at a raid on the base, and the base does not fire at a raid on
        // ships.
        let bombers_left: u32 = bombers.iter().map(|(_, aircraft)| aircraft).sum();
        let guns = match target {
            RaidTarget::Ships(task_force) => {
                let force = &self.truth.task_forces[&task_force];
                let escorts = self.escorts_in_station(task_force);
                let gun_shots: u32 = force
                    .ships
                    .iter()
                    .chain(
                        escorts
                            .iter()
                            .flat_map(|escort| self.truth.task_forces[escort].ships.iter()),
                    )
                    .map(|ship| {
                        self.ref_data.ship_classes[&self.truth.ships[ship].class].anti_aircraft
                    })
                    .sum();
                force.ships.iter().next().map(|lowest_ship| {
                    (
                        gun_shots,
                        EntityRef::Ship(*lowest_ship),
                        Fire::Guns {
                            task_force,
                            air_group: first.air_group,
                        },
                    )
                })
            }
            RaidTarget::Base(base) => Some((
                self.truth.bases[&base].anti_aircraft,
                EntityRef::Base(base),
                Fire::BaseGuns {
                    base,
                    air_group: first.air_group,
                },
            )),
        };
        // Decision 0079: the shots are split. The first k = share times n,
        // cut to a whole number, are worked before the attack, the other n - k
        // after it, rolled in order from the one stream, so the draws are the
        // same at every share.
        let share = air.anti_aircraft_before_attack_share;
        let mut after_attack: Option<FireRoll> = None;
        if let Some((gun_shots, firer, fire)) = guns.filter(|(gun_shots, _, _)| *gun_shots > 0) {
            if bombers_left > 0 {
                let mut stream = self.rng_pair(
                    day,
                    Some(pulse),
                    Subsystem::AirCombat,
                    Draw::Legacy,
                    firer,
                    EntityRef::AirGroup(first.air_group),
                );
                let shot = self.shot_chance(air.anti_aircraft_hit_chance, first.air_group, air);
                let before_shots =
                    u32::try_from((Fixed::from_int(i64::from(gun_shots)) * share).trunc())
                        .expect("a share from 0 to 1 of the shots is not negative")
                        .min(gun_shots);
                // The guns' breakdown names the shots in all and the share,
                // before the rest, whenever the share is below 1.
                let guns_breakdown = |shots: Fixed, expected: Fixed| {
                    breakdown(shots, shot, expected).map(|mut breakdown| {
                        if share < Fixed::ONE {
                            let mut factors = vec![
                                Factor {
                                    name: "guns",
                                    value: Fixed::from_int(i64::from(gun_shots)),
                                },
                                Factor {
                                    name: "anti_aircraft_before_attack_share",
                                    value: share,
                                },
                            ];
                            factors.append(&mut breakdown.factors);
                            breakdown.factors = factors;
                        }
                        breakdown
                    })
                };
                if before_shots > 0 {
                    let (shots, expected, hits, hit_shots) =
                        roll_hits(&mut stream, before_shots, shot.chance);
                    rolls.push(FireRoll {
                        fire: fire.clone(),
                        shots,
                        chance: shot.chance,
                        expected,
                        hits,
                        lost: take_hits(hits, &mut bombers),
                        patrol_of: None,
                        breakdown: guns_breakdown(shots, expected),
                        in_firing: Vec::new(),
                        hit_shots,
                    });
                }
                let after_shots = gun_shots - before_shots;
                if after_shots > 0 {
                    // Rolled now, from the same stream, but its hits come off
                    // the bombers after the attack is worked out.
                    let (shots, expected, hits, hit_shots) =
                        roll_hits(&mut stream, after_shots, shot.chance);
                    after_attack = Some(FireRoll {
                        fire,
                        shots,
                        chance: shot.chance,
                        expected,
                        hits,
                        lost: Vec::new(),
                        patrol_of: None,
                        breakdown: guns_breakdown(shots, expected),
                        in_firing: Vec::new(),
                        hit_shots,
                    });
                }
            }
        }

        // The bombers that are left attack, each group with what it has
        // left (decision 8 of 0027).
        let outcomes: Vec<(AirGroupId, Outcome)> = bombers
            .iter()
            .zip(&raid.strikes)
            .filter(|((_, left), _)| *left > 0)
            .map(|((group, left), strike)| {
                let outcome = match target {
                    RaidTarget::Ships(task_force) => {
                        // Each group attacks the ship its own list ranks
                        // first in the force the raid found (decision 4 of
                        // 0063).
                        let ship = self.ship_to_attack(strike.air_group, task_force);
                        self.work_out_attack(day, pulse, strike, *left, (task_force, ship))
                    }
                    RaidTarget::Base(base) => {
                        self.work_out_base_attack(day, pulse, strike, *left, base)
                    }
                };
                (*group, outcome)
            })
            .collect();
        // The hits of the patrol's fire after the attack come off what is left
        // of the bombers, lowest group first, then the guns' (decision 0087,
        // 6). The attack stands (decision 5 of 0079).
        if let Some(roll) = patrol_after_attack.as_mut() {
            roll.lost = take_hits(roll.hits, &mut bombers);
        }
        if let Some(roll) = after_attack.as_mut() {
            roll.lost = take_hits(roll.hits, &mut bombers);
        }
        RaidResult {
            target: Some(target),
            escorts_sent,
            rolls,
            patrol_after_attack,
            after_attack,
            outcomes,
        }
    }

    /// Rolls the attack of one strike group with `aircraft` bombers at the
    /// target its raid found. Reads truth, which the umpire may; writes
    /// nothing. Draws, from one stream keyed on the air group and the task
    /// force attacked: one draw for each shot, then for each hit
    /// its track (a bomb only: a torpedo always hits flotation) and then its
    /// system, then the sighting report.
    fn work_out_attack(
        &self,
        day: u32,
        pulse: Pulse,
        strike: &StrikeInFlight,
        aircraft: u32,
        target: (TaskForceId, ShipId),
    ) -> Outcome {
        let (task_force, ship) = target;

        let air = self
            .ref_data
            .air_combat
            .as_ref()
            .expect("a strike needs the air combat figures");
        let surface = self
            .ref_data
            .surface_combat
            .as_ref()
            .expect("World::new requires surface combat figures");
        let search = self
            .ref_data
            .search
            .as_ref()
            .expect("World::new requires search figures with air groups");
        let group = &self.truth.air_groups[&strike.air_group];
        let aircraft_type = &self.ref_data.aircraft_types[&group.aircraft_type];
        let toughness = self.ref_data.ship_classes[&self.truth.ships[&ship].class].toughness;
        let (weapon, chance, system_table) = match aircraft_type.attack_weapon {
            AttackWeapon::Bomb => (Weapon::Bomb, air.bomb_hit_chance, &air.bomb_system_hit),
            AttackWeapon::Torpedo => (
                Weapon::Torpedo,
                air.torpedo_hit_chance,
                &surface.torpedo_system_hit,
            ),
            AttackWeapon::None => unreachable!("only groups that attack with something launch"),
        };
        let points = damage_per_hit(aircraft_type.attack_damage, toughness);
        // Decision 0033: the same per-point figure for every weapon.
        let system_chance = system_hit_chance(points, surface.system_hit_chance_per_point);

        let shots = Fixed::from_int(i64::from(aircraft));
        let expected = shots * chance;
        let mut stream = self.rng_pair(
            day,
            Some(pulse),
            Subsystem::AirCombat,
            Draw::Legacy,
            EntityRef::AirGroup(strike.air_group),
            EntityRef::TaskForce(task_force),
        );
        let hits = roll_shots(&mut stream, shots, chance);

        let mut track_hits = [0u32; 2];
        let mut system_hits = Vec::new();
        for _ in 0..hits {
            let track = if weapon == Weapon::Bomb {
                let draw = stream.below(Fixed::SCALE.unsigned_abs());
                if draw < raw_u64(surface.gun_hit_flotation) {
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

        // The report's draws come last from the stream, as they always have;
        // what they report is read once the damage has landed.
        let report = draw_report(&mut stream, search);
        let began_with: Vec<ShipId> = self.truth.task_forces[&task_force]
            .ships
            .iter()
            .copied()
            .collect();

        let breakdown = self.config.record_breakdowns.then(|| Breakdown {
            factors: vec![
                Factor {
                    name: "aircraft",
                    value: shots,
                },
                Factor {
                    name: "hit_chance",
                    value: chance,
                },
            ],
            result: expected,
        });
        Outcome::Attack(Box::new(Attack {
            air_group: strike.air_group,
            side: strike.side,
            task_force,
            ship,
            weapon,
            launched: strike.launched,
            shots,
            chance,
            expected,
            hits,
            damage_per_hit: points,
            track_hits,
            system_hits,
            breakdown,
            report,
            hex: strike.hex,
            began_with,
        }))
    }

    /// Rolls the attack of one strike group with `aircraft` bombers on a
    /// base (decisions 6 and 7 of 0034). Reads truth, which the umpire may;
    /// writes nothing. Draws, from one stream keyed on the air group and the
    /// base: one draw for each bomb, at the bomb hit chance whatever the
    /// type's weapon, then one for each hit, below the chance of landing on
    /// aircraft, then below that and the chance of landing on supply, else on
    /// nothing.
    fn work_out_base_attack(
        &self,
        day: u32,
        pulse: Pulse,
        strike: &StrikeInFlight,
        aircraft: u32,
        base: BaseId,
    ) -> Outcome {
        let air = self
            .ref_data
            .air_combat
            .as_ref()
            .expect("a strike needs the air combat figures");
        let chance = air.bomb_hit_chance;
        let shots = Fixed::from_int(i64::from(aircraft));
        let expected = shots * chance;
        let mut stream = self.rng_pair(
            day,
            Some(pulse),
            Subsystem::AirCombat,
            Draw::Legacy,
            EntityRef::AirGroup(strike.air_group),
            EntityRef::Base(base),
        );
        let hits = roll_shots(&mut stream, shots, chance);
        let landed = self.land_base_hits(&mut stream, hits, base, air);
        let breakdown = self.config.record_breakdowns.then(|| {
            let mut factors = vec![
                Factor {
                    name: "aircraft",
                    value: shots,
                },
                Factor {
                    name: "hit_chance",
                    value: chance,
                },
                Factor {
                    name: "hit_on_aircraft_chance",
                    value: air.base_hit_aircraft,
                },
                Factor {
                    name: "hit_on_supply_chance",
                    value: air.base_hit_supply,
                },
            ];
            // The share for men is named only where it can land on men
            // (decision 0092, 8), so a raid on a base with no men reads as
            // it did before.
            if let Some(share) = self.base_hit_men_share(base) {
                factors.push(Factor {
                    name: "hit_on_men_chance",
                    value: share,
                });
            }
            // And the share for the airfield only where it counts (decision
            // 0103).
            if let Some(share) = self
                .base_hit_airfield_share(base)
                .filter(|share| *share > Fixed::ZERO)
            {
                factors.push(Factor {
                    name: "hit_on_airfield_chance",
                    value: share,
                });
            }
            Breakdown {
                factors,
                result: expected,
            }
        });
        let BaseHits {
            aircraft: hits_on_aircraft,
            supply: hits_on_supply,
            men: hits_on_men,
            airfield: hits_on_airfield,
        } = landed;
        Outcome::BaseAttack(Box::new(BaseAttack {
            air_group: strike.air_group,
            base,
            launched: strike.launched,
            shots,
            chance,
            expected,
            hits,
            hits_on_aircraft,
            hits_on_supply,
            hits_on_men,
            hits_on_airfield,
            breakdown,
        }))
    }
}
