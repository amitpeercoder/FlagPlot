//! The air combat step: every strike launched in the pulse reaches its
//! hex, in raids, and meets the fighters and guns there (decisions 0026 and
//! 0027).

use super::raids::{group_raids, send_escorts};
use super::*;
use crate::ids::AircraftTypeId;

impl World {
    /// The air combat step of one daylight pulse (decisions 7 to 12 and 16
    /// of 0026, and 0027): every strike launched in this pulse reaches its
    /// hex, in raids, and meets the fighters and guns there.
    ///
    /// First every raid is worked out on the state as it stands, the fights
    /// and the losses are recorded and the losses taken off, and the attack
    /// events are recorded. Then, in order of air group number, damage is
    /// applied and the sighting report is sent. The module comment gives the
    /// order of the log.
    pub(crate) fn resolve_strikes(&mut self, day: u32, pulse: Pulse) {
        let strikes = std::mem::take(&mut self.strikes_in_flight);
        // The decks lost in an earlier phase are not this pulse's (decision
        // 7 of 0059).
        self.deck_losses.clear();
        if strikes.is_empty() {
            // A patrol flies whether or not a raid came (decision 7 of 0035).
            if self.notes_sorties() {
                let fighters = self.split_fighter_groups();
                self.note_patrols(&fighters, pulse);
            }
            return;
        }

        // Everything is worked out on the state at the start of the step.
        let mut raids = group_raids(&strikes);
        let mut fighters = self.split_fighter_groups();
        let escorts = send_escorts(&mut raids, &mut fighters);
        for (index, choice) in escorts {
            let group = &fighters[index];
            self.trace_choice(group.side, || {
                (
                    Question::Air(AirQuestion::Escort {
                        group: group.air_group,
                    }),
                    Answer::Air(AirAnswer::Escort(choice)),
                )
            });
        }
        let mut ground = {
            let flying = self.aircraft_flying(pulse, &strikes, &raids, &fighters);
            self.aircraft_on_the_ground(&strikes, &flying)
        };
        if self.notes_sorties() {
            // What flew, for the wear step: the patrols and the strikes now;
            // the escorts as they are recorded; the losses as they are taken.
            self.note_patrols(&fighters, pulse);
            self.note_strikes(&strikes);
        }
        let shares = self.patrol_shares(&raids, &fighters);
        let results: Vec<RaidResult> = raids
            .iter()
            .zip(&shares)
            .map(|(raid, share)| self.work_out_raid(day, pulse, raid, &fighters, *share))
            .collect();
        // The patrol of each side over each hex, as it stood at the start of
        // the step: the escorts' hits on it from all the raids come off it
        // in turn, the lowest group first.
        let mut patrol_pools: Vec<PatrolPool> = Vec::new();
        for group in fighters.iter().filter(|group| group.patrol > 0) {
            if let Some(hex) = group.hex {
                match patrol_pools
                    .iter_mut()
                    .find(|(key, _)| *key == (group.side, hex))
                {
                    Some((_, pool)) => pool.push((group.air_group, group.patrol)),
                    None => patrol_pools
                        .push(((group.side, hex), vec![(group.air_group, group.patrol)])),
                }
            }
        }

        // Record the fights, then the losses, of each raid in turn.
        let mut outcomes: Vec<(AirGroupId, Outcome)> = Vec::new();
        let mut losses_of: Vec<(AirGroupId, EventId)> = Vec::new();
        // What each raid that found a target adds up to for the claims, and
        // the raid each strike group belongs to (decision 0056).
        let mut tallies: Vec<RaidTally> = Vec::new();
        let mut tally_of: BTreeMap<AirGroupId, usize> = BTreeMap::new();
        // The guns' fire after the attack, by raid, recorded once the attacks
        // are (decision 0079): the roll, the raid's index and its tally's.
        let mut fire_after: Vec<(FireRoll, usize, Option<usize>)> = Vec::new();
        for (raid_index, (raid, result)) in raids.iter().zip(results).enumerate() {
            let first = &raid.strikes[0];
            let mut tally = result.target.map(|target| RaidTally {
                hex: first.hex,
                target,
                taken_for: first.taken_for,
                first_group: first.air_group,
                strike_groups: raid.strikes.iter().map(|strike| strike.air_group).collect(),
                attacker_side: first.side,
                attacker: SideTally::default(),
                defender_side: self.defender_of(target),
                defender: SideTally::default(),
            });
            let mut escort_events = Vec::new();
            for (index, (group, aircraft, orders)) in result.escorts_sent.into_iter().enumerate() {
                let causes: Vec<Cause> = orders
                    .into_iter()
                    .chain([Cause::Event(first.launched)])
                    .collect();
                let escort_hex = fighters[raid.escorts[index]].hex;
                escort_events.push(self.events.record(
                    day,
                    Some(pulse),
                    EventKind::EscortsSent {
                        air_group: group,
                        with_air_group: first.air_group,
                        aircraft,
                    },
                    causes,
                    None,
                ));
                if self.notes_sorties() {
                    self.note_escorts(
                        group,
                        aircraft,
                        escort_hex,
                        first.hex,
                        *escort_events.last().expect("just recorded"),
                    );
                }
            }
            let mut recorded = Vec::new();
            for roll in &result.rolls {
                let mut causes = vec![Cause::Event(first.launched)];
                let kind = match &roll.fire {
                    Fire::Fighters {
                        firer,
                        target,
                        firing,
                        at,
                    } => {
                        // A fight with escorts in it is explained by them.
                        if *firing == AirFirer::Escorts || *at == AirTarget::Escorts {
                            causes.extend(escort_events.iter().copied().map(Cause::Event));
                        }
                        EventKind::AirFight {
                            firer: *firer,
                            target: *target,
                            firing: *firing,
                            at: *at,
                            shots: roll.shots,
                            chance: roll.chance,
                            expected: roll.expected,
                            hits: roll.hits,
                            after_attack: false,
                        }
                    }
                    Fire::Guns {
                        task_force,
                        air_group,
                    } => EventKind::AntiAircraftFire {
                        task_force: *task_force,
                        air_group: *air_group,
                        shots: roll.shots,
                        chance: roll.chance,
                        expected: roll.expected,
                        hits: roll.hits,
                        after_attack: false,
                    },
                    Fire::BaseGuns { base, air_group } => EventKind::BaseAntiAircraftFire {
                        base: *base,
                        air_group: *air_group,
                        shots: roll.shots,
                        chance: roll.chance,
                        expected: roll.expected,
                        hits: roll.hits,
                        after_attack: false,
                    },
                };
                recorded.push(self.events.record(
                    day,
                    Some(pulse),
                    kind,
                    causes,
                    roll.breakdown.clone(),
                ));
            }
            for (roll, cause) in result.rolls.iter().zip(recorded) {
                let taken;
                let lost_list = match roll.patrol_of {
                    Some(key) => {
                        let pool = patrol_pools
                            .iter_mut()
                            .find(|(pool_key, _)| *pool_key == key)
                            .map(|(_, pool)| pool)
                            .expect("a patrol that is fired at is in the pools");
                        taken = take_hits(roll.hits, pool);
                        &taken
                    }
                    None => &roll.lost,
                };
                let lost_from = match (&roll.fire, roll.patrol_of) {
                    (_, Some(_)) => SortieKind::Patrol,
                    (
                        Fire::Fighters {
                            at: AirTarget::Escorts,
                            ..
                        },
                        None,
                    ) => SortieKind::Escort,
                    _ => SortieKind::Strike,
                };
                // The firing group: with the patrol group it keys the draws for
                // the pilots of a patrol's losses (decision 6a of 0059).
                let firer = match &roll.fire {
                    Fire::Fighters { firer, .. } => Some(*firer),
                    _ => None,
                };
                let mut real = 0;
                for (group, lost) in lost_list {
                    if let Some((event, taken)) =
                        self.lose_aircraft(day, pulse, *group, *lost, lost_from, cause, firer)
                    {
                        losses_of.push((*group, event));
                        self.note_loss(*group, lost_from, *lost);
                        real += taken;
                    }
                }
                if let Some(tally) = tally.as_mut() {
                    tally.note_roll(roll, cause, real);
                }
            }
            if tally.is_some() {
                for strike in &raid.strikes {
                    tally_of.insert(strike.air_group, tallies.len());
                }
            }
            // The patrol's fire after the attack comes before the guns' in the
            // same raid (decision 0087, 6).
            for roll in [result.patrol_after_attack, result.after_attack]
                .into_iter()
                .flatten()
            {
                fire_after.push((roll, raid_index, tally.is_some().then_some(tallies.len())));
            }
            tallies.extend(tally);
            outcomes.extend(result.outcomes);
        }
        outcomes.sort_by_key(|(group, _)| *group);

        // Record what each strike found, in group order. Each attack on a
        // ship and each raid on a base draws whether the people under it
        // recognised the aircraft (decision 0167, 2 and 5): one stream for
        // each ship or base attacked in the pulse, its own, so that no fight
        // draws differently.
        let recognised_chance = self
            .ref_data
            .air_combat
            .as_ref()
            .map_or(Fixed::ZERO, |air| air.raid_recognised_chance);
        let mut recognition: BTreeMap<EntityRef, RngStream> = BTreeMap::new();
        let mut attack_events: Vec<EventId> = Vec::new();
        for (_, outcome) in &outcomes {
            match outcome {
                Outcome::Nothing {
                    air_group,
                    hex,
                    launched,
                    ..
                } => {
                    let found_nothing = self.events.record(
                        day,
                        Some(pulse),
                        EventKind::StrikeFoundNothing {
                            air_group: *air_group,
                            hex: *hex,
                        },
                        vec![Cause::Event(*launched)],
                        None,
                    );
                    attack_events.push(found_nothing);
                }
                Outcome::BaseAttack(attack) => {
                    let causes: Vec<Cause> = [Cause::Event(attack.launched)]
                        .into_iter()
                        .chain(
                            losses_of
                                .iter()
                                .filter(|(group, _)| *group == attack.air_group)
                                .map(|(_, event)| Cause::Event(*event)),
                        )
                        .collect();
                    let recognised = self.recognised(
                        &mut recognition,
                        day,
                        pulse,
                        EntityRef::Base(attack.base),
                        attack.air_group,
                        recognised_chance,
                    );
                    let event = self.events.record(
                        day,
                        Some(pulse),
                        EventKind::BaseAttacked {
                            air_group: attack.air_group,
                            base: attack.base,
                            shots: attack.shots,
                            chance: attack.chance,
                            expected: attack.expected,
                            hits: attack.hits,
                            hits_on_aircraft: attack.hits_on_aircraft,
                            hits_on_supply: attack.hits_on_supply,
                            hits_on_men: attack.hits_on_men,
                            recognised,
                        },
                        causes,
                        attack.breakdown.clone(),
                    );
                    attack_events.push(event);
                    if let Some(index) = tally_of.get(&attack.air_group) {
                        let side = &mut tallies[*index].attacker;
                        side.fired(EntityRef::AirGroup(attack.air_group), event);
                        side.bombs += whole_shots(attack.shots);
                    }
                }
                Outcome::Attack(attack) => {
                    // The aircraft that attack are those the fights left.
                    let causes: Vec<Cause> = [Cause::Event(attack.launched)]
                        .into_iter()
                        .chain(
                            losses_of
                                .iter()
                                .filter(|(group, _)| *group == attack.air_group)
                                .map(|(_, event)| Cause::Event(*event)),
                        )
                        .collect();
                    let recognised = self.recognised(
                        &mut recognition,
                        day,
                        pulse,
                        EntityRef::Ship(attack.ship),
                        attack.air_group,
                        recognised_chance,
                    );
                    let event = self.events.record(
                        day,
                        Some(pulse),
                        EventKind::AirAttack {
                            air_group: attack.air_group,
                            task_force: attack.task_force,
                            ship: attack.ship,
                            weapon: attack.weapon,
                            shots: attack.shots,
                            chance: attack.chance,
                            expected: attack.expected,
                            hits: attack.hits,
                            recognised,
                        },
                        causes,
                        attack.breakdown.clone(),
                    );
                    attack_events.push(event);
                    // A force an air strike reaches breaks radio silence
                    // (decision 0136, 17).
                    self.break_silence(attack.task_force, day, pulse);
                    if let Some(index) = tally_of.get(&attack.air_group) {
                        let side = &mut tallies[*index].attacker;
                        side.fired(EntityRef::AirGroup(attack.air_group), event);
                        side.hits += attack.hits;
                        side.hit_misses += whole_shots(attack.shots).saturating_sub(attack.hits);
                    }
                }
            }
        }

        // The guns' fire after the attack, raid by raid (decision 0079): the
        // event, citing the raid's launch and what its groups attacked, then
        // the aircraft it shot down, as the fire before the attack does. The
        // attacks are worked out and stand: these losses are not their causes.
        let attack_of: BTreeMap<AirGroupId, EventId> = outcomes
            .iter()
            .zip(&attack_events)
            .filter(|((_, outcome), _)| !matches!(outcome, Outcome::Nothing { .. }))
            .map(|((group, _), event)| (*group, *event))
            .collect();
        for (roll, raid_index, tally_index) in fire_after {
            let raid = &raids[raid_index];
            let first = &raid.strikes[0];
            let causes: Vec<Cause> = [Cause::Event(first.launched)]
                .into_iter()
                .chain(
                    raid.strikes
                        .iter()
                        .filter_map(|strike| attack_of.get(&strike.air_group))
                        .map(|event| Cause::Event(*event)),
                )
                .collect();
            let kind = match &roll.fire {
                Fire::Guns {
                    task_force,
                    air_group,
                } => EventKind::AntiAircraftFire {
                    task_force: *task_force,
                    air_group: *air_group,
                    shots: roll.shots,
                    chance: roll.chance,
                    expected: roll.expected,
                    hits: roll.hits,
                    after_attack: true,
                },
                Fire::BaseGuns { base, air_group } => EventKind::BaseAntiAircraftFire {
                    base: *base,
                    air_group: *air_group,
                    shots: roll.shots,
                    chance: roll.chance,
                    expected: roll.expected,
                    hits: roll.hits,
                    after_attack: true,
                },
                // Decision 0087: the patrol's fire at the bombers.
                Fire::Fighters {
                    firer,
                    target,
                    firing,
                    at,
                } => EventKind::AirFight {
                    firer: *firer,
                    target: *target,
                    firing: *firing,
                    at: *at,
                    shots: roll.shots,
                    chance: roll.chance,
                    expected: roll.expected,
                    hits: roll.hits,
                    after_attack: true,
                },
            };
            let cause = self
                .events
                .record(day, Some(pulse), kind, causes, roll.breakdown.clone());
            // The firing group keys the draws for the pilots of a patrol's
            // losses, as for its fire before the attack.
            let firer = match &roll.fire {
                Fire::Fighters { firer, .. } => Some(*firer),
                _ => None,
            };
            let mut real = 0;
            for (group, lost) in &roll.lost {
                if let Some((_, taken)) =
                    self.lose_aircraft(day, pulse, *group, *lost, SortieKind::Strike, cause, firer)
                {
                    self.note_loss(*group, SortieKind::Strike, *lost);
                    real += taken;
                }
            }
            if let Some(index) = tally_index {
                tallies[index].note_roll(&roll, cause, real);
            }
        }

        // Damage lands, then the report goes out, in group order.
        let site = DamageSite {
            day,
            pulse,
            round: 0,
        };
        let search = self
            .ref_data
            .search
            .clone()
            .expect("World::new requires search figures with air groups");
        for ((_, outcome), event) in outcomes.into_iter().zip(attack_events) {
            let attack = match outcome {
                Outcome::Attack(attack) => attack,
                Outcome::BaseAttack(attack) => {
                    let pool = ground
                        .get_mut(&attack.base)
                        .expect("a base that is attacked has its ground counted");
                    let destroyed = self.apply_base_hits(
                        day,
                        Some(pulse),
                        attack.base,
                        BaseHits {
                            aircraft: attack.hits_on_aircraft,
                            supply: attack.hits_on_supply,
                            men: attack.hits_on_men,
                            airfield: attack.hits_on_airfield,
                        },
                        event,
                        pool,
                    );
                    if let Some(index) = tally_of.get(&attack.air_group) {
                        tallies[*index].attacker.real_on_ground += destroyed;
                    }
                    continue;
                }
                Outcome::Nothing {
                    air_group,
                    side,
                    contact,
                    hex,
                    ..
                } => {
                    // Decision 0028: the side learns the hex was empty, and
                    // drops the contact if it still names that hex. The
                    // contact is the group's command's, by the number its
                    // picture gives it (decision 0135, 16): that command
                    // holds the report first.
                    let command = self.command_of_group(air_group);
                    let held = self
                        .picture(side, command)
                        .contact(contact)
                        .is_some_and(|held| held.estimated_hex == hex);
                    let holders = Holders::Commands {
                        commands: vec![command],
                        key: event,
                        day,
                        pulse: Some(pulse),
                    };
                    let report = Report::NothingThere {
                        contact: Some(contact),
                        hex,
                    };
                    self.apply_report(side, &report, &holders, &search);
                    if held {
                        self.events.record(
                            day,
                            Some(pulse),
                            EventKind::ContactNotThere { side, contact, hex },
                            vec![Cause::Event(event)],
                            None,
                        );
                    }
                    self.pass_report(side, &report, &holders);
                    continue;
                }
            };
            let attack_event = event;
            let applied = self.apply_hit_set(
                site,
                &HitSet {
                    cause: attack_event,
                    target: attack.ship,
                    target_force: attack.task_force,
                    damage_per_hit: attack.damage_per_hit,
                    track_hits: attack.track_hits,
                    system_hits: attack.system_hits.clone(),
                },
            );
            if let Some(index) = tally_of.get(&attack.air_group) {
                let sunk = &mut tallies[*index].attacker.sunk;
                for (ship, _) in applied.sunk {
                    if !sunk.contains(&ship) {
                        sunk.push(ship);
                    }
                }
            }

            // The report reads the force after the damage: the ships still
            // afloat in it, or those it had at the start of the step if none
            // is left, as a fight's report does (`docs/rules/differences.md`,
            // row 7).
            let afloat: Vec<ShipId> = self.truth.task_forces[&attack.task_force]
                .ships
                .iter()
                .copied()
                .collect();
            let seen = if afloat.is_empty() {
                &attack.began_with
            } else {
                &afloat
            };
            let report = attack.report.read(
                &self.map,
                attack.hex,
                u32::try_from(seen.len()).expect("few ships"),
                self.most_important_kind_of(seen),
                &search,
                self.events.next_id(),
            );
            // The attacking group's command holds the sighting first
            // (decision 0135, 1).
            let holders = Holders::Commands {
                commands: vec![self.command_of_group(attack.air_group)],
                key: report.received_event,
                day,
                pulse: Some(pulse),
            };
            let ApplyOutcome::Contact(outcome) = self.apply_report(
                attack.side,
                &Report::Sighting(report.clone()),
                &holders,
                &search,
            ) else {
                unreachable!("a sighting report makes or refreshes a contact");
            };
            let received = self.events.record(
                day,
                Some(pulse),
                EventKind::ReportReceived {
                    side: attack.side,
                    contact: outcome.contact,
                    hex: report.estimated_hex,
                    ship_count: report.estimated_ship_count,
                    kind: report.estimated_kind,
                    created: outcome.created,
                },
                vec![Cause::Event(attack_event)],
                None,
            );
            assert_eq!(
                received, report.received_event,
                "the report cites the event that records it"
            );
            self.pass_report(attack.side, &Report::Sighting(report), &holders);
        }

        // Each side that fired in a raid claims what it believes it did, once
        // the raid is resolved (decision 0056).
        self.record_raid_claims(day, pulse, tallies);
    }
}

impl World {
    /// Whether the people of the ship or base `target` recognised the
    /// aircraft of `air_group` that attacked it (decision 0167, 2 and 5): one
    /// draw with `chance` from the target's stream of the pulse, opened at
    /// its first attack and drawn from in turn by each attack after it. The
    /// type recognised is the group's own: never another (decision 0167, 4).
    fn recognised(
        &self,
        streams: &mut BTreeMap<EntityRef, RngStream>,
        day: u32,
        pulse: Pulse,
        target: EntityRef,
        air_group: AirGroupId,
        chance: Fixed,
    ) -> Option<AircraftTypeId> {
        let stream = streams.entry(target).or_insert_with(|| {
            self.rng(
                day,
                Some(pulse),
                Subsystem::AirCombat,
                Draw::RaidRecognised,
                target,
            )
        });
        if !stream.chance(chance) {
            return None;
        }
        self.truth
            .air_groups
            .get(&air_group)
            .map(|group| group.aircraft_type)
    }
}
