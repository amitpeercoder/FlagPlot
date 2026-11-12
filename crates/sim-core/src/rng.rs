//! Counter-based random numbers.
//!
//! There is no shared generator that advances as the simulation runs. Each
//! draw is a pure function of a key: seed, day, pulse, subsystem, entity, and
//! the position of the draw in that entity's own sequence. A draw about a pair
//! of entities (this air group looking for that task force) names a second
//! entity as well; see `docs/decisions/0013-random-keys-for-pairs.md`. Processing
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

/// Which rule is drawing. Two rules that draw for the same entities in the
/// same subsystem and pulse would otherwise open the same stream and see the
/// same numbers, and nothing would say so (decision 0055).
///
/// Every rule that draws names its own value here. The exception is
/// [`Draw::Legacy`], which is the eleven draws that existed before the tag
/// did. It adds nothing to the key, so their streams, and every pinned
/// number, are what they were. They are told apart by the shape of their
/// keys, listed in decision 0055. No new rule may use it.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum Draw {
    Legacy,
    /// The false claims of a raid (decision 0056): one stream for each side's
    /// claim, keyed on its first firing unit and the raid's first strike
    /// group.
    Claim,
}

impl Draw {
    /// Every value, for the test that their codes differ.
    pub const ALL: [Draw; 2] = [Draw::Legacy, Draw::Claim];

    /// A stable number for the rule. Never renumber. 0 is `Legacy` alone
    /// and is not folded into the key.
    pub const fn code(self) -> u64 {
        match self {
            Draw::Legacy => 0,
            Draw::Claim => 1,
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
    /// The second entity of a pair, or `None` for a draw about one entity.
    /// With `None` the stream is exactly what it was before this field
    /// existed. The order of `entity` and `other` is fixed by the rule that
    /// draws (decision 0013).
    pub other: Option<EntityRef>,
    /// The rule that draws. With [`Draw::Legacy`] the stream is exactly
    /// what it was before this field existed.
    pub draw: Draw,
}

const GOLDEN: u64 = 0x9E37_79B9_7F4A_7C15;
const TAG_DAY: u64 = 0xA076_1D64_78BD_642F;
const TAG_PULSE: u64 = 0xE703_7ED1_A0B4_28DB;
const TAG_SUBSYSTEM: u64 = 0x8EBC_6AF0_9C88_C6E3;
const TAG_ENTITY: u64 = 0x5899_65CC_7537_4CC3;
/// Tag for the second entity of a pair key. Part of the save format.
const TAG_OTHER: u64 = 0xD1B5_4A32_D192_ED03;
/// Tag for the rule that draws. Part of the save format.
const TAG_DRAW: u64 = 0x2545_F491_4F6C_DD1D;

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
        if let Some(other) = self.other {
            base = absorb(base, TAG_OTHER, other.stable_key());
        }
        if self.draw.code() != 0 {
            base = absorb(base, TAG_DRAW, self.draw.code());
        }
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
    use crate::ids::{AirGroupId, ShipId, TaskForceId};

    fn key(entity: EntityRef) -> RngKey {
        RngKey {
            seed: 1942,
            day: 17,
            pulse: Some(Pulse::Morning),
            subsystem: Subsystem::Search,
            entity,
            other: None,
            draw: Draw::Legacy,
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
            // A second entity, and a different second entity.
            RngKey {
                other: Some(ship(4)),
                ..base
            },
            RngKey {
                other: Some(ship(5)),
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
    fn every_rule_that_draws_has_its_own_code_and_only_legacy_is_zero() {
        let mut seen = BTreeMap::new();
        for draw in Draw::ALL {
            assert!(
                seen.insert(draw.code(), draw).is_none(),
                "{draw:?} shares a code"
            );
            assert_eq!(draw.code() == 0, draw == Draw::Legacy);
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
            other: None,
            draw: Draw::Legacy,
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
            other: None,
            draw: Draw::Legacy,
        };
        assert_eq!(draws(end_of_day, 2), PINNED_END_OF_DAY);
    }

    fn pair_key(entity: EntityRef, other: EntityRef) -> RngKey {
        RngKey {
            other: Some(other),
            ..key(entity)
        }
    }

    fn air_group(raw: u32) -> EntityRef {
        EntityRef::AirGroup(AirGroupId::new(raw))
    }

    fn task_force(raw: u32) -> EntityRef {
        EntityRef::TaskForce(TaskForceId::new(raw))
    }

    /// Pinned values for the pair form (decision 0013). Part of the save
    /// format, like the single-entity values.
    #[test]
    fn pair_output_is_pinned() {
        assert_eq!(
            draws(pair_key(air_group(1), task_force(1)), 3),
            PINNED_PAIR_AIR_GROUP_1_TASK_FORCE_1
        );
    }

    #[test]
    fn the_order_of_a_pair_matters() {
        let ab = draws(pair_key(air_group(1), task_force(1)), 4);
        let ba = draws(pair_key(task_force(1), air_group(1)), 4);
        assert_ne!(ab, ba);
    }

    #[test]
    fn a_pair_key_differs_from_the_single_key_of_its_first_entity() {
        let single = draws(key(air_group(1)), 4);
        let pair = draws(pair_key(air_group(1), task_force(1)), 4);
        assert_ne!(single, pair);
    }

    #[test]
    fn changing_the_second_entity_changes_the_stream() {
        let one = draws(pair_key(air_group(1), task_force(1)), 4);
        let two = draws(pair_key(air_group(1), task_force(2)), 4);
        assert_ne!(one, two);
    }

    const PINNED_SHIP_3: [u64; 3] = [
        14_248_313_465_659_663_483,
        11_050_835_108_968_198_041,
        11_523_755_419_223_571_334,
    ];
    // Seed 1942, day 17, morning, search; air group 1 and task force 1. Part
    // of the save format: do not change without a decision record.
    const PINNED_PAIR_AIR_GROUP_1_TASK_FORCE_1: [u64; 3] = [
        3_902_777_303_372_789_147,
        4_120_078_374_976_771_759,
        14_060_622_519_450_889_050,
    ];
    const PINNED_END_OF_DAY: [u64; 2] = [6_669_309_778_430_270_743, 10_390_202_447_835_255_326];
}
