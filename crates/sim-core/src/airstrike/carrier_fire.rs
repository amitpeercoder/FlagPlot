//! Fire among a carrier's aircraft (decision 0212, its sections from
//! "Measured and reviewed again" on, which govern, with a figure by side
//! from decision 0213).
//!
//! It is of carriers alone: a ship of the carrier kind in a task force. A
//! raid whose bombs hit her may set her bombers aboard alight; the fire is
//! then a state of her own, a level from 1 to the figures'
//! `aircraft_fire_levels`, apart from her fire damage, which it neither
//! reads nor writes. It is fought at the end of each pulse after the one a
//! raid last set it in: one draw, under `fire_spreads_chance` it rises a
//! level and otherwise falls one; at nothing it is out; a fire that would
//! rise above the top loses her, by the ordinary sinking. While it burns she
//! flies nothing off and lands nothing on ([`Ship::deck_for_flying`]).
//!
//! # Setting it
//!
//! After every attack of the air combat step has landed, for each raid, in
//! the order of its first group, and each carrier it put one bomb hit or
//! more on, in ship order, if she is still afloat in a task force: her
//! bombers are the ready and damaged aircraft of the groups aboard her of a
//! type with the bomber role; those flying are the ready ones the pulse's
//! own count has in the air ([`World::aircraft_flying`], counted once at
//! the start of the step); those aboard the rest. The share is those aboard
//! over all of them. Where none is aboard (none at all, or all flying) no
//! draw is made. Otherwise the chance for one bomb is her side's
//! `aircraft_fire_chance` times the share, and for the raid one less the
//! chance that every bomb fails, each product cut to millionths in turn;
//! one draw on a stream keyed on the ship and the raid's first strike group
//! ([`Draw::AircraftFireSet`]): a draw below the chance sets her fire at the
//! top, whatever it was, and that pulse is the one it was last set in.
//!
//! # Fighting it
//!
//! [`World::fight_aircraft_fires`], the last phase of every pulse, night
//! too: for each ship in a task force, in ship order, whose fire was not set
//! in this pulse, one draw on a stream keyed on the ship, by day and pulse
//! ([`Draw::AircraftFireFought`]).

use super::*;
use crate::state::{AircraftFire, ShipLocation};

/// A carrier's bombers at the start of the air combat step: aboard, and
/// flying in the pulse.
#[derive(Clone, Copy, Default)]
pub(super) struct BombersAboard {
    aboard: u32,
    flying: u32,
}

/// The bombs one raid put on one ship, and the attacks that put them there.
#[derive(Default)]
pub(super) struct RaidBombs {
    pub(super) bombs: u32,
    pub(super) attacks: Vec<EventId>,
}

impl World {
    /// The bombers of each carrier afloat in a task force, aboard and
    /// flying, by the pulse's count of aircraft flying. Where the fire's
    /// figures are absent, nothing: no fire is set.
    pub(super) fn bombers_aboard(
        &self,
        flying: &BTreeMap<AirGroupId, u32>,
    ) -> BTreeMap<ShipId, BombersAboard> {
        let mut counts: BTreeMap<ShipId, BombersAboard> = BTreeMap::new();
        if self.aircraft_fire_figures().is_none() {
            return counts;
        }
        for group in self.truth.air_groups.values() {
            let AirGroupLocation::Ship(ship) = group.location else {
                continue;
            };
            if !self.is_carrier_at_sea(ship) {
                continue;
            }
            let bomber = self
                .ref_data
                .aircraft_types
                .get(&group.aircraft_type)
                .is_some_and(|kind| kind.role == crate::refdata::AircraftRole::Bomber);
            if !bomber {
                continue;
            }
            let in_air = flying.get(&group.id).copied().unwrap_or(0).min(group.ready);
            let count = counts.entry(ship).or_default();
            count.aboard += group.ready - in_air + group.damaged;
            count.flying += in_air;
        }
        counts
    }

    fn aircraft_fire_figures(&self) -> Option<AircraftFireFigures> {
        self.ref_data
            .air_combat
            .as_ref()
            .and_then(|air| air.aircraft_fire)
    }

    /// Whether `ship` is of the carrier kind and afloat in a task force.
    fn is_carrier_at_sea(&self, ship: ShipId) -> bool {
        self.truth.ships.get(&ship).is_some_and(|state| {
            matches!(state.location, ShipLocation::TaskForce(_))
                && self
                    .ref_data
                    .ship_classes
                    .get(&state.class)
                    .is_some_and(|class| class.kind == ShipKind::Carrier)
        })
    }

    /// Sets the fires among aircraft of the step (decision 0212, as
    /// measured, 3): see the module comment. `bombs` is by raid (its index
    /// in the step's raids, in order of first group) and ship; `counts` the
    /// bombers counted at the start of the step; `raids` the step's raids.
    pub(super) fn set_aircraft_fires(
        &mut self,
        day: u32,
        pulse: Pulse,
        raids: &[Raid],
        bombs: &BTreeMap<(usize, ShipId), RaidBombs>,
        counts: &BTreeMap<ShipId, BombersAboard>,
    ) {
        let Some(figures) = self.aircraft_fire_figures() else {
            return;
        };
        for ((raid_index, ship), raid_bombs) in bombs {
            if raid_bombs.bombs == 0 || !self.is_carrier_at_sea(*ship) {
                continue;
            }
            let Some(count) = counts.get(ship).copied() else {
                continue;
            };
            if count.aboard == 0 {
                continue;
            }
            let ShipLocation::TaskForce(task_force) = self.truth.ships[ship].location else {
                continue;
            };
            let side = self.truth.ships[ship].side;
            let all = count.aboard + count.flying;
            let share = Fixed::from_ratio(i64::from(count.aboard), i64::from(all));
            let one = figures.chance.of(side) * share;
            let mut fails = Fixed::ONE;
            for _ in 0..raid_bombs.bombs {
                fails = fails * (Fixed::ONE - one);
            }
            let chance = Fixed::ONE - fails;
            let first = raids[*raid_index].strikes[0].air_group;
            let mut stream = self.rng_pair(
                day,
                Some(pulse),
                Subsystem::AirCombat,
                Draw::AircraftFireSet,
                EntityRef::Ship(*ship),
                EntityRef::AirGroup(first),
            );
            let raw = stream.below(Fixed::SCALE.unsigned_abs());
            let draw = Fixed::from_raw(i64::try_from(raw).expect("a draw fits in i64"));
            let set = draw < chance;
            let before = self.truth.ships[ship]
                .aircraft_fire
                .map_or(0, |fire| fire.level);
            let level_after = if set { figures.levels } else { before };
            let breakdown = self.config.record_breakdowns.then(|| Breakdown {
                factors: vec![
                    Factor {
                        name: "bombs",
                        value: Fixed::from_int(i64::from(raid_bombs.bombs)),
                    },
                    Factor {
                        name: "bombers_aboard",
                        value: Fixed::from_int(i64::from(count.aboard)),
                    },
                    Factor {
                        name: "bombers_flying",
                        value: Fixed::from_int(i64::from(count.flying)),
                    },
                    Factor {
                        name: "share_aboard",
                        value: share,
                    },
                    Factor {
                        name: "aircraft_fire_chance",
                        value: figures.chance.of(side),
                    },
                    Factor {
                        name: "chance_one_bomb",
                        value: one,
                    },
                ],
                result: chance,
            });
            let event = self.events.record(
                day,
                Some(pulse),
                EventKind::AircraftFireRoll {
                    ship: *ship,
                    task_force,
                    bombs: raid_bombs.bombs,
                    aboard: count.aboard,
                    flying: count.flying,
                    share,
                    chance,
                    draw,
                    set,
                    level_after,
                },
                raid_bombs
                    .attacks
                    .iter()
                    .copied()
                    .map(Cause::Event)
                    .collect(),
                breakdown,
            );
            if set {
                self.truth
                    .ships
                    .get_mut(ship)
                    .expect("the ship exists")
                    .aircraft_fire = Some(AircraftFire {
                    level: figures.levels,
                    set_day: day,
                    set_pulse: pulse,
                    record: event,
                });
                // Her deck is closed to her aircraft in the air, as when a
                // deck goes out (decision 7 of 0059).
                if let Some(hex) = self.ship_hex(*ship) {
                    self.deck_losses.push(crate::landing::DeckLoss {
                        ship: *ship,
                        hex,
                        cause: event,
                    });
                }
            }
        }
    }

    /// The last phase of every pulse (decision 0212, as measured, 4 and 5):
    /// each fire among a carrier's aircraft not set in this pulse is fought.
    /// See the module comment. Does nothing where the figures are absent.
    pub(crate) fn fight_aircraft_fires(&mut self, day: u32, pulse: Pulse) {
        let Some(figures) = self.aircraft_fire_figures() else {
            return;
        };
        let burning: Vec<(ShipId, TaskForceId, AircraftFire)> = self
            .truth
            .ships
            .values()
            .filter_map(|ship| {
                let fire = ship.aircraft_fire?;
                let ShipLocation::TaskForce(force) = ship.location else {
                    return None;
                };
                Some((ship.id, force, fire))
            })
            .collect();
        for (ship, task_force, fire) in burning {
            if (fire.set_day, fire.set_pulse) == (day, pulse) {
                continue;
            }
            let mut stream = self.rng(
                day,
                Some(pulse),
                Subsystem::AirCombat,
                Draw::AircraftFireFought,
                EntityRef::Ship(ship),
            );
            let raw = stream.below(Fixed::SCALE.unsigned_abs());
            let draw = Fixed::from_raw(i64::try_from(raw).expect("a draw fits in i64"));
            let rises = draw < figures.spreads_chance;
            let lost = rises && fire.level >= figures.levels;
            let level_after = if rises {
                fire.level + 1
            } else {
                fire.level - 1
            };
            let breakdown = self.config.record_breakdowns.then(|| Breakdown {
                factors: vec![
                    Factor {
                        name: "level_before",
                        value: Fixed::from_int(i64::from(fire.level)),
                    },
                    Factor {
                        name: "fire_spreads_chance",
                        value: figures.spreads_chance,
                    },
                    Factor {
                        name: "draw",
                        value: draw,
                    },
                ],
                result: Fixed::from_int(i64::from(level_after)),
            });
            let event = self.events.record(
                day,
                Some(pulse),
                EventKind::AircraftFireFought {
                    ship,
                    task_force,
                    level_before: fire.level,
                    draw,
                    level_after,
                    lost,
                },
                vec![Cause::Event(fire.record)],
                breakdown,
            );
            let state = self.truth.ships.get_mut(&ship).expect("the ship exists");
            state.aircraft_fire = (level_after > 0 && !lost).then_some(AircraftFire {
                level: level_after,
                record: event,
                ..fire
            });
            if lost {
                let site = crate::damage::DamageSite {
                    day,
                    pulse,
                    round: 0,
                };
                self.sink_ship(site, task_force, ship, event);
            }
        }
    }
}
