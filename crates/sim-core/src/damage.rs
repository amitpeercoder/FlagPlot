//! Damage to a ship, shared by every way a ship can be hit.
//!
//! A hit does the same to a ship whatever delivered it (decision 10 of
//! `docs/decisions/0026-air-strikes.md`, the rule-complexity rule of
//! `CLAUDE.md`): damage points on the two tracks, a system knocked down one
//! level, a ship that reaches 100 flotation sinks and takes its air groups
//! with it. Surface combat (`surface.rs`, decisions 0014 and 0016) and air
//! strikes (`airstrike.rs`) both come here.
//!
//! The caller rolls the hits and hands over a [`HitSet`]: how many hits fell
//! on each track, which systems were hit, and the event that caused them.
//! [`World::apply_hit_set`] then writes the state and the events.

use std::collections::BTreeMap;

use crate::events::{Cause, DamageTrack, EventKind};
use crate::fixed::Fixed;
use crate::ids::{AirGroupId, EventId, ShipId, TaskForceId};
use crate::refdata::ShipClass;
use crate::rng::RngStream;
use crate::state::{AirGroupLocation, Ship, ShipLocation, ShipSystem};
use crate::time::Pulse;
use crate::world::World;

/// A damage track at its limit.
pub(crate) const FULL_DAMAGE: Fixed = Fixed::from_int(100);

/// Where and when the hits land. `round` is the surface fight round the
/// events carry, or 0 for hits that fall outside a fight (an air attack).
#[derive(Clone, Copy)]
pub(crate) struct DamageSite {
    pub day: u32,
    pub pulse: Pulse,
    pub round: u32,
}

/// The hits of one salvo or one air attack on one ship.
pub(crate) struct HitSet {
    /// The event that caused the hits: the salvo or the air attack.
    pub cause: EventId,
    pub target: ShipId,
    pub target_force: TaskForceId,
    pub damage_per_hit: Fixed,
    /// Hits on flotation and fire, in [`DamageTrack::ALL`] order.
    pub track_hits: [u32; 2],
    /// The systems hit, in hit order. One entry per system hit.
    pub system_hits: Vec<ShipSystem>,
}

/// What [`World::apply_hit_set`] recorded.
#[derive(Default)]
pub(crate) struct Applied {
    /// The `ShipDamaged` events, in order.
    pub damaged: Vec<EventId>,
    /// The `ShipSunk` events, in order.
    pub sunk: Vec<(ShipId, EventId)>,
}

/// A chance as an unsigned number to compare a draw against.
pub(crate) fn raw_u64(value: Fixed) -> u64 {
    u64::try_from(value.raw()).expect("a chance is not negative")
}

/// Damage points one hit does: 100 x damage / toughness. The product comes
/// first and there is one division, truncating, so whole-number figures give
/// the same bits as before the figures could be decimals. Toughness is above
/// zero (`ShipClass::problem`).
pub(crate) fn damage_per_hit(damage: Fixed, toughness: Fixed) -> Fixed {
    damage.mul_div(Fixed::from_int(100), toughness)
}

pub(crate) fn track_value(ship: &mut Ship, track: DamageTrack) -> &mut Fixed {
    match track {
        DamageTrack::Flotation => &mut ship.flotation_damage,
        DamageTrack::Fire => &mut ship.fire_damage,
    }
}

/// Rolls each shot of a salvo on its own (decision 0033, 3 and 4) and returns
/// the hits. Each whole shot is one `stream.chance(chance)` draw. A part of a
/// shot (`shots` is not a whole number) is one more draw after them, at
/// `chance` times the part; with no part there is no extra draw. The number of
/// draws depends only on `shots`, so a cap applied to the hits afterwards
/// cannot change what the stream gives next.
pub(crate) fn roll_shots(stream: &mut RngStream, shots: Fixed, chance: Fixed) -> u32 {
    let whole = shots.trunc();
    let part = shots - Fixed::from_int(whole);
    let mut hits = 0;
    for _ in 0..whole {
        hits += u32::from(stream.chance(chance));
    }
    if !part.is_zero() {
        hits += u32::from(stream.chance(chance * part));
    }
    hits
}

/// Chance a hit also knocks a system down one level (decision 0033): its
/// damage points times `per_point`, 1 at most. The product is a [`Fixed`]
/// product (truncating) and the cap comes after it.
pub(crate) fn system_hit_chance(points: Fixed, per_point: Fixed) -> Fixed {
    (points * per_point).min(Fixed::ONE)
}

/// Which system one system draw lands on, if any. `draw` is a uniform draw
/// below one million, read as a [`Fixed`] in millionths. The systems take
/// consecutive bands in [`ShipSystem::ALL`] order, each of width `chance` x
/// the system's share in `table` (a system absent from it has share 0). A
/// draw at or above the end of the last band hits no system.
pub(crate) fn system_for_draw(
    draw: Fixed,
    chance: Fixed,
    table: &BTreeMap<ShipSystem, Fixed>,
) -> Option<ShipSystem> {
    let mut start = Fixed::ZERO;
    for system in ShipSystem::ALL {
        let share = table.get(&system).copied().unwrap_or(Fixed::ZERO);
        let end = start + chance * share;
        if draw >= start && draw < end {
            return Some(system);
        }
        start = end;
    }
    None
}

impl World {
    fn damage_class_of(&self, ship: ShipId) -> &ShipClass {
        &self.ref_data.ship_classes[&self.truth.ships[&ship].class]
    }

    /// Damage lands: one `ShipDamaged` event per track that took hits, then
    /// one `SystemHit` event per system hit, in hit order, each caused by
    /// `hits.cause`. A ship whose flotation reaches 100 sinks.
    pub(crate) fn apply_hit_set(&mut self, site: DamageSite, hits: &HitSet) -> Applied {
        let mut applied = Applied::default();
        let target = hits.target;
        for track in DamageTrack::ALL {
            let count = hits.track_hits[track as usize];
            if count == 0 {
                continue;
            }
            let ship = self.truth.ships.get_mut(&target).expect("target exists");
            let value = track_value(ship, track);
            let before = *value;
            let points = hits.damage_per_hit * i64::from(count);
            let after = (before + points).min(FULL_DAMAGE);
            *value = after;
            let damaged = self.record_at(
                site,
                EventKind::ShipDamaged {
                    round: site.round,
                    ship: target,
                    track,
                    hits: count,
                    points,
                    value_after: after,
                },
                vec![Cause::Event(hits.cause)],
            );
            applied.damaged.push(damaged);
            if track == DamageTrack::Flotation && before < FULL_DAMAGE && after >= FULL_DAMAGE {
                let sunk = self.sink_ship(site, hits.target_force, target, damaged);
                applied.sunk.push((target, sunk));
            }
        }
        for system in &hits.system_hits {
            let system = *system;
            let has_system = self.damage_class_of(target).has_system(system);
            let ship = self.truth.ships.get_mut(&target).expect("target exists");
            let before = ship.systems.get(system);
            let after = if has_system {
                before.step_down()
            } else {
                before
            };
            ship.systems.set(system, after);
            self.record_at(
                site,
                EventKind::SystemHit {
                    round: site.round,
                    ship: target,
                    system,
                    before,
                    after,
                },
                vec![Cause::Event(hits.cause)],
            );
        }
        applied
    }

    /// Takes a ship out of its task force and records it, with the loss of
    /// every air group aboard (decision 5 of
    /// `docs/decisions/0025-carrier-air-groups.md`).
    fn sink_ship(
        &mut self,
        site: DamageSite,
        task_force: TaskForceId,
        ship: ShipId,
        cause: EventId,
    ) -> EventId {
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
        let sunk = self.record_at(
            site,
            EventKind::ShipSunk {
                round: site.round,
                ship,
                task_force,
            },
            vec![Cause::Event(cause)],
        );
        // The group stays aboard in the state, with no aircraft and no
        // position.
        let aboard: Vec<AirGroupId> = self
            .truth
            .air_groups
            .values()
            .filter(|group| group.location == AirGroupLocation::Ship(ship))
            .map(|group| group.id)
            .collect();
        for air_group in aboard {
            let group = self
                .truth
                .air_groups
                .get_mut(&air_group)
                .expect("air group exists");
            // Damaged aircraft go down with the ship too (decision 0035).
            let aircraft = group.ready + group.damaged;
            group.ready = 0;
            group.damaged = 0;
            self.record_at(
                site,
                EventKind::AirGroupLost {
                    air_group,
                    ship,
                    aircraft,
                },
                vec![Cause::Event(sunk)],
            );
        }
        sunk
    }

    fn record_at(&mut self, site: DamageSite, kind: EventKind, causes: Vec<Cause>) -> EventId {
        self.events
            .record(site.day, Some(site.pulse), kind, causes, None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fx(text: &str) -> Fixed {
        text.parse().expect("a decimal")
    }

    fn stream(entity: u32) -> RngStream {
        use crate::ids::EntityRef;
        use crate::rng::{RngKey, Subsystem};
        RngKey {
            seed: 7,
            day: 1,
            pulse: None,
            subsystem: Subsystem::SurfaceCombat,
            entity: EntityRef::Ship(ShipId::new(entity)),
            other: None,
        }
        .stream()
    }

    #[test]
    fn a_certain_chance_hits_every_whole_shot_and_a_part_at_the_chance_times_the_part() {
        for entity in 1..=20 {
            assert_eq!(
                roll_shots(&mut stream(entity), Fixed::from_int(7), Fixed::ONE),
                7
            );
            assert_eq!(
                roll_shots(&mut stream(entity), Fixed::from_int(7), Fixed::ZERO),
                0
            );
            assert_eq!(roll_shots(&mut stream(entity), Fixed::ZERO, Fixed::ONE), 0);
        }
        // 2.5 shots at 1: two sure hits and a third with chance 0.5.
        let extra: u32 = (1..=1000)
            .map(|entity| roll_shots(&mut stream(entity), fx("2.5"), Fixed::ONE) - 2)
            .sum();
        // 1,000 draws at 0.5: sd 15.8, so within 4 sd of 500.
        assert!((437..=563).contains(&extra), "{extra}");
    }

    #[test]
    fn the_draws_are_one_per_whole_shot_and_one_more_only_for_a_part() {
        for (shots, draws) in [
            ("0", 0),
            ("1", 1),
            ("6", 6),
            ("6.5", 7),
            ("0.25", 1),
            ("67.5", 68),
        ] {
            for chance in ["0", "0.3", "1"] {
                let mut used = stream(3);
                roll_shots(&mut used, fx(shots), fx(chance));
                let mut fresh = stream(3);
                for _ in 0..draws {
                    fresh.next_u64();
                }
                assert_eq!(
                    used.next_u64(),
                    fresh.next_u64(),
                    "{shots} shots at {chance}"
                );
            }
        }
    }

    #[test]
    fn twenty_shots_at_a_fifth_hit_four_a_salvo_on_average() {
        // 1,000 streams: the sum of 20,000 draws at 0.2 has mean 4,000 and
        // sd sqrt(1000 x 20 x 0.2 x 0.8) = 56.57; four sd is 226.
        let total: u32 = (1..=1000)
            .map(|entity| roll_shots(&mut stream(entity), Fixed::from_int(20), fx("0.2")))
            .sum();
        assert!((3774..=4226).contains(&total), "{total}");
    }

    #[test]
    fn the_system_hit_chance_is_points_times_the_figure() {
        // Decision 0033's table at 0.06 a point.
        let per_point = fx("0.06");
        assert_eq!(system_hit_chance(Fixed::from_int(5), per_point), fx("0.3"));
        assert_eq!(system_hit_chance(fx("1.25"), per_point), fx("0.075"));
        assert_eq!(
            system_hit_chance(Fixed::from_int(20), per_point),
            Fixed::ONE
        );
        assert_eq!(
            system_hit_chance(Fixed::from_int(50), per_point),
            Fixed::ONE
        );
        assert_eq!(system_hit_chance(Fixed::from_int(10), per_point), fx("0.6"));
    }

    #[test]
    fn the_system_hit_chance_is_capped_at_one_and_not_below_zero() {
        let per_point = fx("0.06");
        // 16.666666 x 0.06 = 0.99999996 truncates to 0.999999: below the cap.
        assert_eq!(
            system_hit_chance(fx("16.666666"), per_point),
            Fixed::from_raw(999_999)
        );
        assert_eq!(system_hit_chance(fx("16.7"), per_point), Fixed::ONE);
        assert_eq!(
            system_hit_chance(Fixed::from_int(1000), fx("5")),
            Fixed::ONE
        );
        // A hit that does no damage never hits a system, and a zero figure
        // turns system hits off.
        assert_eq!(system_hit_chance(Fixed::ZERO, per_point), Fixed::ZERO);
        assert_eq!(
            system_hit_chance(Fixed::from_int(50), Fixed::ZERO),
            Fixed::ZERO
        );
        let table: BTreeMap<ShipSystem, Fixed> =
            [(ShipSystem::Propulsion, Fixed::ONE)].into_iter().collect();
        let none = system_hit_chance(Fixed::ZERO, per_point);
        for raw in [0, 1, 500_000, 999_999] {
            assert_eq!(system_for_draw(Fixed::from_raw(raw), none, &table), None);
        }
    }

    #[test]
    fn the_cap_leaves_the_bands_whole_at_one() {
        // At the cap the table's shares are the bands: the draws are the
        // same as for any other chance, only the width changes.
        let table: BTreeMap<ShipSystem, Fixed> = [
            (ShipSystem::Propulsion, fx("0.8")),
            (ShipSystem::MainBattery, fx("0.2")),
        ]
        .into_iter()
        .collect();
        let chance = system_hit_chance(Fixed::from_int(50), fx("0.06"));
        let draw = |raw: i64| system_for_draw(Fixed::from_raw(raw), chance, &table);
        assert_eq!(draw(799_999), Some(ShipSystem::Propulsion));
        assert_eq!(draw(800_000), Some(ShipSystem::MainBattery));
        assert_eq!(draw(999_999), Some(ShipSystem::MainBattery));
    }
}
