//! Counter-based random numbers.
//!
//! There is no shared generator that advances as the simulation runs. Each
//! draw is a pure function of a key: seed, day, pulse, subsystem, entity, and
//! the position of the draw in that entity's own sequence. Processing
//! entities in a different order therefore cannot change any result.
//!
//! The mixing function is the SplitMix64 finaliser. It is not cryptographic
//! and does not need to be. See `docs/decisions/0003-random-numbers.md`.
//!
//! The output of this module is part of the save format: changing any
//! constant or the order fields are absorbed in changes every random outcome
//! in every saved game. The pinned-value test below exists to catch that.

use crate::fixed::Fixed;
use crate::ids::EntityRef;
use crate::time::Pulse;

/// The part of the rules asking for a random number.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum Subsystem {
    Movement,
    Search,
    AirCombat,
    SurfaceCombat,
    GroundCombat,
    Logistics,
    Upkeep,
    Intelligence,
}

impl Subsystem {
    /// A stable number for the subsystem. Never renumber.
    pub const fn code(self) -> u64 {
        match self {
            Subsystem::Movement => 1,
            Subsystem::Search => 2,
            Subsystem::AirCombat => 3,
            Subsystem::SurfaceCombat => 4,
            Subsystem::GroundCombat => 5,
            Subsystem::Logistics => 6,
            Subsystem::Upkeep => 7,
            Subsystem::Intelligence => 8,
        }
    }
}

/// Everything that identifies one entity's stream of random numbers.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct RngKey {
    pub seed: u64,
    pub day: u32,
    /// `None` for the end-of-day phases, which belong to no pulse.
    pub pulse: Option<Pulse>,
    pub subsystem: Subsystem,
    pub entity: EntityRef,
}

const GOLDEN: u64 = 0x9E37_79B9_7F4A_7C15;
const TAG_DAY: u64 = 0xA076_1D64_78BD_642F;
const TAG_PULSE: u64 = 0xE703_7ED1_A0B4_28DB;
const TAG_SUBSYSTEM: u64 = 0x8EBC_6AF0_9C88_C6E3;
const TAG_ENTITY: u64 = 0x5899_65CC_7537_4CC3;

/// SplitMix64 finaliser.
const fn mix64(mut z: u64) -> u64 {
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    z ^ (z >> 31)
}

/// Folds one field into the running key. Each field has its own tag so that
/// equal values in different fields do not cancel out.
const fn absorb(state: u64, tag: u64, value: u64) -> u64 {
    mix64(state ^ mix64(value ^ tag))
}

impl RngKey {
    /// Opens this key's stream at its first draw.
    pub fn stream(&self) -> RngStream {
        let pulse_code = match self.pulse {
            Some(pulse) => pulse.code(),
            None => 0,
        };
        let mut base = mix64(self.seed.wrapping_add(GOLDEN));
        base = absorb(base, TAG_DAY, u64::from(self.day));
        base = absorb(base, TAG_PULSE, pulse_code);
        base = absorb(base, TAG_SUBSYSTEM, self.subsystem.code());
        base = absorb(base, TAG_ENTITY, self.entity.stable_key());
        RngStream { base, counter: 0 }
    }
}

/// A sequence of random numbers belonging to one [`RngKey`].
#[derive(Clone, Debug)]
pub struct RngStream {
    base: u64,
    counter: u64,
}

impl RngStream {
    /// The next 64 random bits.
    pub fn next_u64(&mut self) -> u64 {
        self.counter = self.counter.wrapping_add(1);
        mix64(self.base.wrapping_add(self.counter.wrapping_mul(GOLDEN)))
    }

    /// A uniform number in `0..n`. Uses rejection, so there is no bias toward
    /// low values. Panics if `n` is zero.
    pub fn below(&mut self, n: u64) -> u64 {
        assert!(n > 0, "below(0) has no valid result");
        let zone = u64::MAX - (u64::MAX % n);
        loop {
            let value = self.next_u64();
            if value < zone {
                return value % n;
            }
        }
    }

    /// True with the given probability, where `Fixed::ONE` is certainty.
    /// Probabilities outside 0..=1 are clamped.
    pub fn chance(&mut self, probability: Fixed) -> bool {
        let threshold = probability.raw().clamp(0, Fixed::SCALE).unsigned_abs();
        self.below(Fixed::SCALE.unsigned_abs()) < threshold
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use super::*;
    use crate::ids::{ShipId, TaskForceId};

    fn key(entity: EntityRef) -> RngKey {
        RngKey {
            seed: 1942,
            day: 17,
            pulse: Some(Pulse::Morning),
            subsystem: Subsystem::Search,
            entity,
        }
    }

    fn ship(raw: u32) -> EntityRef {
        EntityRef::Ship(ShipId::new(raw))
    }

    fn draws(key: RngKey, count: usize) -> Vec<u64> {
        let mut stream = key.stream();
        (0..count).map(|_| stream.next_u64()).collect()
    }

    #[test]
    fn the_same_key_always_gives_the_same_sequence() {
        assert_eq!(draws(key(ship(3)), 16), draws(key(ship(3)), 16));
    }

    #[test]
    fn results_do_not_depend_on_processing_order() {
        let ids: Vec<u32> = (1..=200).collect();

        let mut forward = BTreeMap::new();
        for &id in &ids {
            forward.insert(id, draws(key(ship(id)), 4));
        }

        let mut backward = BTreeMap::new();
        for &id in ids.iter().rev() {
            backward.insert(id, draws(key(ship(id)), 4));
        }

        // An order that is neither sorted nor reversed.
        let mut scattered = BTreeMap::new();
        for step in 0..ids.len() {
            let id = ids[(step * 73) % ids.len()];
            scattered.insert(id, draws(key(ship(id)), 4));
        }

        assert_eq!(forward, backward);
        assert_eq!(forward, scattered);
        assert_eq!(scattered.len(), ids.len());
    }

    #[test]
    fn interleaving_two_streams_changes_neither() {
        let alone_a = draws(key(ship(1)), 6);
        let alone_b = draws(key(ship(2)), 6);

        let mut a = key(ship(1)).stream();
        let mut b = key(ship(2)).stream();
        let mut mixed_a = Vec::new();
        let mut mixed_b = Vec::new();
        for _ in 0..6 {
            mixed_b.push(b.next_u64());
            mixed_a.push(a.next_u64());
        }

        assert_eq!(alone_a, mixed_a);
        assert_eq!(alone_b, mixed_b);
    }

    #[test]
    fn every_field_of_the_key_changes_the_stream() {
        let base = key(ship(3));
        let first = |key: RngKey| key.stream().next_u64();

        let variants = [
            RngKey { seed: 1943, ..base },
            RngKey { day: 18, ..base },
            RngKey {
                pulse: Some(Pulse::Afternoon),
                ..base
            },
            RngKey {
                pulse: None,
                ..base
            },
            RngKey {
                subsystem: Subsystem::AirCombat,
                ..base
            },
            RngKey {
                entity: ship(4),
                ..base
            },
            // Same raw ID, different kind of entity.
            RngKey {
                entity: EntityRef::TaskForce(TaskForceId::new(3)),
                ..base
            },
        ];

        let mut seen = vec![first(base)];
        for variant in variants {
            let value = first(variant);
            assert!(!seen.contains(&value), "two keys gave the same first draw");
            seen.push(value);
        }
    }

    #[test]
    fn swapping_values_between_fields_changes_the_stream() {
        let a = RngKey {
            seed: 1,
            day: 2,
            pulse: None,
            subsystem: Subsystem::Movement,
            entity: ship(1),
        };
        let b = RngKey {
            seed: 2,
            day: 1,
            ..a
        };
        assert_ne!(a.stream().next_u64(), b.stream().next_u64());
    }

    #[test]
    fn below_stays_in_range_and_is_roughly_uniform() {
        let mut stream = key(ship(9)).stream();
        let mut buckets = [0u32; 10];
        for _ in 0..10_000 {
            let value = stream.below(10);
            buckets[usize::try_from(value).unwrap()] += 1;
        }
        for (bucket, count) in buckets.iter().enumerate() {
            assert!(
                (850..=1150).contains(count),
                "bucket {bucket} holds {count} of 10000 draws"
            );
        }
    }

    #[test]
    fn chance_respects_certainty_and_impossibility() {
        let mut stream = key(ship(9)).stream();
        for _ in 0..1_000 {
            assert!(stream.chance(Fixed::ONE));
            assert!(!stream.chance(Fixed::ZERO));
            assert!(stream.chance(Fixed::from_int(5)));
            assert!(!stream.chance(Fixed::from_int(-5)));
        }
    }

    #[test]
    fn chance_hits_at_about_the_stated_rate() {
        let mut stream = key(ship(11)).stream();
        let quarter = Fixed::from_ratio(1, 4);
        let hits = (0..20_000).filter(|_| stream.chance(quarter)).count();
        assert!((4_700..=5_300).contains(&hits), "{hits} hits of 20000");
    }

    /// These values are part of the save format. If this test fails, the
    /// generator changed and every saved game replays differently. Do not
    /// update the numbers without a decision record saying why.
    #[test]
    fn output_is_pinned() {
        assert_eq!(draws(key(ship(3)), 3), PINNED_SHIP_3);
        let end_of_day = RngKey {
            seed: 0,
            day: 1,
            pulse: None,
            subsystem: Subsystem::Logistics,
            entity: EntityRef::TaskForce(TaskForceId::new(1)),
        };
        assert_eq!(draws(end_of_day, 2), PINNED_END_OF_DAY);
    }

    const PINNED_SHIP_3: [u64; 3] = [
        14_248_313_465_659_663_483,
        11_050_835_108_968_198_041,
        11_523_755_419_223_571_334,
    ];
    const PINNED_END_OF_DAY: [u64; 2] = [6_669_309_778_430_270_743, 10_390_202_447_835_255_326];
}
