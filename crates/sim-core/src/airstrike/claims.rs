//! The claims of the raids of one pulse (decision 0056).

use super::*;
use crate::claims::{FiringTypes, UnitClaim};

/// Adds a list of types to a claim's breakdown as pairs of factors: the
/// type's number under `name_type`, then its aircraft under `name_aircraft`
/// (decisions 0179, 10, and 0210, 11).
fn push_types(
    factors: &mut Vec<Factor>,
    name_type: &'static str,
    name_aircraft: &'static str,
    list: &[(AircraftTypeId, u32)],
) {
    for (kind, n) in list {
        factors.push(Factor {
            name: name_type,
            value: Fixed::from_int(i64::from(kind.raw())),
        });
        factors.push(Factor {
            name: name_aircraft,
            value: Fixed::from_int(i64::from(*n)),
        });
    }
}

/// The chance the types are made out and whether they were, once for the
/// claim (decision 0210, 11).
fn push_made_out(factors: &mut Vec<Factor>, chance: Fixed, made_out: bool) {
    factors.push(Factor {
        name: "raid_recognised_chance",
        value: chance,
    });
    factors.push(Factor {
        name: "types_made_out",
        value: if made_out { Fixed::ONE } else { Fixed::ZERO },
    });
}

impl World {
    /// The claims of the raids of one pulse (decision 0056): for each raid
    /// that found a target, in order, the raiding side's claim and then the
    /// defending side's, each only if it fired. A claim is what really
    /// happened and the false claims drawn for the whole shots that missed,
    /// from a stream of its own keyed for intelligence on the day, the pulse,
    /// the side's first firing unit and the raid's first strike group. No draw
    /// of the fight itself is touched.
    pub(super) fn record_raid_claims(&mut self, day: u32, pulse: Pulse, tallies: Vec<RaidTally>) {
        if tallies.is_empty() {
            return;
        }
        let air = self
            .ref_data
            .air_combat
            .clone()
            .expect("a strike needs the air combat figures");
        let search = self.ref_data.search.clone();
        for tally in tallies {
            for (side, part, attacking) in [
                (tally.attacker_side, tally.attacker, true),
                (tally.defender_side, tally.defender, false),
            ] {
                let Some(first) = part.first else {
                    continue;
                };
                let mut stream = self.rng_pair(
                    day,
                    Some(pulse),
                    Subsystem::Intelligence,
                    Draw::Claim,
                    first,
                    EntityRef::AirGroup(tally.first_group),
                );
                let (mut fighter_misses, mut gun_misses) = (0, 0);
                let mut false_down = 0;
                // Who claims what (decision 0180): a line for each unit, in
                // the order it first fired.
                let mut lines: Vec<UnitClaim> = part
                    .claimers
                    .iter()
                    .map(|unit| UnitClaim {
                        unit: *unit,
                        shot_down: 0,
                        on_ground: 0,
                        hits: 0,
                    })
                    .collect();
                let line_of = |lines: &mut Vec<UnitClaim>, unit: EntityRef| -> usize {
                    lines
                        .iter()
                        .position(|line| line.unit == unit)
                        .expect("every unit that claims is a claimer")
                };
                // The false claims of each firing, unit by unit in the order
                // it is parted (decision 0180, 4): the same draws, in the
                // same order, as one drawing for the firing. Each firing's
                // list of types, should they be made out, is worked on its
                // own pool (decision 0210, 2 and 3).
                let mut air_firings: Vec<FiringTypes> = Vec::new();
                for firing in &part.firings {
                    let chance = match firing.kind {
                        MissKind::Fighter => air.fighter_false_claim_chance,
                        MissKind::Gun => air.anti_aircraft_false_claim_chance,
                    };
                    let mut firing_false = 0;
                    for (unit, real, misses) in &firing.parts {
                        match firing.kind {
                            MissKind::Fighter => fighter_misses += misses,
                            MissKind::Gun => gun_misses += misses,
                        }
                        let drawn =
                            roll_shots(&mut stream, Fixed::from_int(i64::from(*misses)), chance);
                        false_down += drawn;
                        firing_false += drawn;
                        let at = line_of(&mut lines, *unit);
                        lines[at].shot_down += real + drawn;
                    }
                    air_firings.push(FiringTypes {
                        real: firing.real_by_type.clone(),
                        false_claimed: firing_false,
                        pool: firing.pool.clone(),
                    });
                }
                let claimed_shot_down = part.real_shot_down + false_down;
                // What the raiding side believes it did to what it attacked:
                // hits on ships and ships sunk (decisions 4 and 5), or
                // aircraft destroyed on the ground (decision 7). The draws
                // come after those for aircraft shot down, on the same
                // stream, group by group in the order the groups attacked
                // (decision 0180, 5).
                let count = |n: u32| Fixed::from_int(i64::from(n));
                let (mut claimed_hits, mut claimed_sunk, mut claimed_on_ground) = (0, 0, 0);
                let (mut false_hits, mut burning, mut false_ground) = (0, 0, 0);
                let mut claimed_kind = None;
                let mut ground_types = Vec::new();
                // The breakdown's part for the types (decision 0179, 10).
                let mut type_factors: Vec<Factor> = Vec::new();
                if attacking {
                    let on_base = matches!(tally.target, RaidTarget::Base(_));
                    let mut drawn_false = 0;
                    let strike_lines =
                        crate::claims::strike_lines(&part.strikes, on_base, |missed| {
                            let drawn =
                                roll_shots(&mut stream, count(missed), air.bomb_false_claim_chance);
                            drawn_false += drawn;
                            drawn
                        });
                    for strike_line in strike_lines {
                        let at = line_of(&mut lines, strike_line.unit);
                        lines[at].on_ground += strike_line.on_ground;
                        lines[at].hits += strike_line.hits;
                    }
                    match tally.target {
                        RaidTarget::Ships(force) => {
                            false_hits = drawn_false;
                            claimed_hits = part.hits + false_hits;
                            claimed_kind = tally.taken_for;
                            // A ship left burning, as a surface fight's claim
                            // counts it (decision 5).
                            let level = self.ref_data.surface_combat.claimed_sunk_fire;
                            burning = self.truth.task_forces[&force]
                                .ships
                                .iter()
                                .filter(|ship| self.truth.ships[*ship].fire_damage >= level)
                                .count()
                                .try_into()
                                .expect("few ships");
                            claimed_sunk =
                                u32::try_from(part.sunk.len()).expect("few ships") + burning;
                        }
                        RaidTarget::Base(_) => {
                            false_ground = drawn_false;
                            claimed_on_ground = part.real_on_ground + false_ground;
                        }
                    }
                }
                // The types of what is claimed, on the ground and in the air,
                // made out or not by one draw for the claim, on a stream of
                // its own (decisions 0179, 3 to 5, and 0210, 5). The draw is
                // made where the chance is above 0 and the claim has
                // something to name: aircraft claimed on the ground with a
                // field there, or aircraft claimed shot down.
                let chance = air.raid_recognised_chance;
                let on_the_ground = attacking && claimed_on_ground > 0 && !tally.field.is_empty();
                let in_the_air = claimed_shot_down > 0;
                let made_out = chance > Fixed::ZERO
                    && (on_the_ground || in_the_air)
                    && self
                        .rng_pair(
                            day,
                            Some(pulse),
                            Subsystem::Intelligence,
                            Draw::ClaimType,
                            first,
                            EntityRef::AirGroup(tally.first_group),
                        )
                        .chance(chance);
                let mut air_types = Vec::new();
                if on_the_ground {
                    let real: Vec<(AircraftTypeId, u32)> = part
                        .real_on_ground_by_type
                        .iter()
                        .map(|(kind, n)| (*kind, *n))
                        .collect();
                    if made_out {
                        ground_types =
                            crate::claims::ground_claim_types(&real, false_ground, &tally.field);
                    }
                    if self.config.record_breakdowns {
                        let falsely =
                            crate::claims::ground_claim_types(&[], false_ground, &tally.field);
                        for (name_type, name_aircraft, list) in [
                            ("field_type", "field_aircraft", &tally.field),
                            ("real_type", "real_aircraft", &real),
                            ("false_type", "false_aircraft", &falsely),
                        ] {
                            push_types(&mut type_factors, name_type, name_aircraft, list);
                        }
                        push_made_out(&mut type_factors, chance, made_out);
                    }
                }
                if in_the_air {
                    if made_out {
                        air_types = crate::claims::air_claim_types(&air_firings);
                    }
                    // The air's own factors, firing by firing, only where
                    // the chance is above 0, so that no breakdown on the
                    // frozen figures changes (decision 0210, 11).
                    if self.config.record_breakdowns && chance > Fixed::ZERO {
                        for firing in &air_firings {
                            let falsely = crate::claims::ground_claim_types(
                                &[],
                                firing.false_claimed,
                                &firing.pool,
                            );
                            for (name_type, name_aircraft, list) in [
                                ("pool_fired_on_type", "pool_fired_on_aircraft", &firing.pool),
                                ("real_down_type", "real_down_aircraft", &firing.real),
                                ("false_down_type", "false_down_aircraft", &falsely),
                            ] {
                                push_types(&mut type_factors, name_type, name_aircraft, list);
                            }
                        }
                        if !on_the_ground {
                            push_made_out(&mut type_factors, chance, made_out);
                        }
                    }
                }
                lines.retain(|line| !line.is_nothing());
                let claim = BattleClaim {
                    hex: tally.hex,
                    day,
                    pulse,
                    claimed_sunk,
                    claimed_hits,
                    claimed_kind,
                    claimed_shot_down,
                    claimed_on_ground,
                    from_air: true,
                    from_bombardment: false,
                    from_submarine: false,
                    received_event: self.events.next_id(),
                    ground_types: ground_types.clone(),
                    by_unit: lines.clone(),
                    air_types: air_types.clone(),
                };
                // The commands that fired hold the claim first (decision
                // 0135, 1): the attacking side's, its strike groups'; the
                // defending side's, the commands of the patrols and guns
                // that fired, and the base for a raid on a base.
                let mut commands: Vec<CommandRef> = if attacking {
                    tally
                        .strike_groups
                        .iter()
                        .map(|group| self.command_of_group(*group))
                        .collect()
                } else {
                    part.units
                        .iter()
                        .map(|unit| match unit {
                            EntityRef::AirGroup(group) => self.command_of_group(*group),
                            EntityRef::TaskForce(force) => self.command_of_force(*force),
                            EntityRef::Base(base) => CommandRef::Base(*base),
                            _ => CommandRef::Area,
                        })
                        .collect()
                };
                if let (false, RaidTarget::Base(base)) = (attacking, tally.target) {
                    commands.push(CommandRef::Base(base));
                }
                let holders = Holders::Commands {
                    commands,
                    key: claim.received_event,
                    day,
                    pulse: Some(pulse),
                };
                let outcome =
                    self.apply_report(side, &Report::BattleClaim(claim.clone()), &holders, &search);
                assert_eq!(outcome, ApplyOutcome::Claim, "a claim is stored as a claim");
                let breakdown = self.config.record_breakdowns.then(|| {
                    let mut factors = vec![
                        Factor {
                            name: "real_shot_down",
                            value: count(part.real_shot_down),
                        },
                        Factor {
                            name: "fighter_misses",
                            value: count(fighter_misses),
                        },
                        Factor {
                            name: "fighter_false_claim_chance",
                            value: air.fighter_false_claim_chance,
                        },
                        Factor {
                            name: "gun_misses",
                            value: count(gun_misses),
                        },
                        Factor {
                            name: "anti_aircraft_false_claim_chance",
                            value: air.anti_aircraft_false_claim_chance,
                        },
                        Factor {
                            name: "false_shot_down",
                            value: count(false_down),
                        },
                    ];
                    if attacking {
                        match tally.target {
                            RaidTarget::Ships(_) => factors.extend([
                                Factor {
                                    name: "real_hits",
                                    value: count(part.hits),
                                },
                                Factor {
                                    name: "missed_bombs",
                                    value: count(part.hit_misses),
                                },
                                Factor {
                                    name: "bomb_false_claim_chance",
                                    value: air.bomb_false_claim_chance,
                                },
                                Factor {
                                    name: "false_hits",
                                    value: count(false_hits),
                                },
                                Factor {
                                    name: "real_sunk",
                                    value: count(
                                        u32::try_from(part.sunk.len()).expect("few ships"),
                                    ),
                                },
                                Factor {
                                    name: "burning",
                                    value: count(burning),
                                },
                            ]),
                            RaidTarget::Base(_) => factors.extend([
                                Factor {
                                    name: "bombs",
                                    value: count(part.bombs),
                                },
                                Factor {
                                    name: "real_on_ground",
                                    value: count(part.real_on_ground),
                                },
                                Factor {
                                    name: "bomb_false_claim_chance",
                                    value: air.bomb_false_claim_chance,
                                },
                                Factor {
                                    name: "false_on_ground",
                                    value: count(false_ground),
                                },
                            ]),
                        }
                    }
                    factors.append(&mut type_factors);
                    Breakdown {
                        factors,
                        result: count(claimed_shot_down + claimed_hits + claimed_on_ground),
                    }
                });
                let received = self.events.record(
                    day,
                    Some(pulse),
                    EventKind::ClaimReceived {
                        side,
                        hex: tally.hex,
                        claimed_sunk,
                        claimed_hits,
                        claimed_kind,
                        claimed_shot_down,
                        claimed_on_ground,
                        from_air: true,
                        from_bombardment: false,
                        from_submarine: false,
                        ground_types,
                        by_unit: lines.clone(),
                        air_types,
                    },
                    part.causes.iter().copied().map(Cause::Event).collect(),
                    breakdown,
                );
                assert_eq!(
                    received, claim.received_event,
                    "the claim cites the event that records it"
                );
                // Each air group's tally takes its line, and the claim's
                // event is the event of the change (decision 0180, 6 and 11).
                for line in &lines {
                    if let EntityRef::AirGroup(group) = line.unit {
                        if let Some(group) = self.truth.air_groups.get_mut(&group) {
                            group.claims.shot_down += line.shot_down;
                            group.claims.on_ground += line.on_ground;
                            group.claims.hits += line.hits;
                        }
                    }
                }
                self.pass_report(side, &Report::BattleClaim(claim), &holders);
            }
        }
    }
}
