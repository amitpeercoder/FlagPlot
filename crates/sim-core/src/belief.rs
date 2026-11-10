//! Belief state: what one side thinks is out there.
//!
//! A side never reads the truth about the enemy. It holds contacts and battle
//! claims, and both only ever come from a [`Report`].
//! [`BeliefState::apply_report`] is the only way a contact is created or
//! refreshed or a claim is stored, which keeps "belief is written only from
//! reports" true by construction.
//!
//! [`BeliefState::age_contacts`] is the second method. It takes no truth and
//! no report: it only ages contacts and forgets what reports once said, so it
//! adds no information and the rule still holds.
//!
//! The set of contacts refreshed in the current pulse is transient. It is
//! filled by `apply_report`, read and cleared by `age_contacts` at the end of
//! the pulse, and so is always empty between pulses. It is therefore not part
//! of the state hash (the hash is taken at the end of a day, after the last
//! pulse's ageing). [`hash::hash_world`](crate::hash) asserts, in debug
//! builds, that it is empty.
//!
//! Neither a [`Report`] nor a [`Contact`] nor a [`BattleClaim`] may ever hold
//! a truth ID (a task force, ship or air group ID): that would let a side tell that two
//! sightings were the same force. They do hold an [`EventId`]. That is a
//! reference to an event in the log, not truth, and `SideView` does not
//! expose events.
//!
//! Claims never age. See `docs/decisions/0012-search-and-contacts.md`,
//! decisions 12 to 14, and `docs/decisions/0014-surface-combat.md`.

use std::collections::{BTreeMap, BTreeSet};

use crate::fixed::Fixed;
use crate::hex::Hex;
use crate::ids::{ContactId, EventId, IdAllocator};
use crate::refdata::{SearchFigures, ShipKind};
use crate::state::Side;
use crate::time::Pulse;

/// Something a side is told. It may be wrong about everything.
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Report {
    /// An air search or a fight sighted an enemy force.
    Sighting(SightingReport),
    /// A force that fired claims enemy ships sunk.
    BattleClaim(BattleClaim),
}

/// What a sighting claims. It may be wrong about everything.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SightingReport {
    pub estimated_hex: Hex,
    pub estimated_ship_count: u32,
    /// The most important kind of ship the report claims to have seen.
    pub estimated_kind: ShipKind,
    /// How sure the observer is, where `Fixed::ONE` is certain.
    pub confidence: Fixed,
    /// The event that records this report's arrival.
    pub received_event: EventId,
}

/// What a side believes came of a fight: how many enemy ships it believes
/// sank. Real damage goes into truth, and the two can differ.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct BattleClaim {
    /// The hex of the fight.
    pub hex: Hex,
    /// The day of the fight.
    pub day: u32,
    pub pulse: Pulse,
    pub claimed_sunk: u32,
    /// The event that records this claim's arrival.
    pub received_event: EventId,
}

/// Something one side believes is at sea.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Contact {
    pub id: ContactId,
    pub estimated_hex: Hex,
    pub estimated_ship_count: u32,
    pub estimated_kind: ShipKind,
    /// Pulses since the last fresh sighting.
    pub age_pulses: u32,
    pub confidence: Fixed,
    /// The event that recorded the arrival of the report that last created or
    /// refreshed this contact.
    pub last_report_event: EventId,
}

/// What a sighting report did to the contacts.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct ReportOutcome {
    pub contact: ContactId,
    /// True if the report made a new contact, false if it refreshed one.
    pub created: bool,
}

/// What [`BeliefState::apply_report`] did with a report.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ApplyOutcome {
    /// A sighting made or refreshed a contact.
    Contact(ReportOutcome),
    /// A battle claim was stored.
    Claim,
}

/// What [`BeliefState::age_contacts`] did to one contact.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct AgeOutcome {
    pub contact: ContactId,
    /// The age the contact has, or would have had if it was dropped.
    pub age_pulses: u32,
    /// The confidence the contact has, or would have had if it was dropped.
    pub confidence: Fixed,
    pub dropped: bool,
    pub last_report_event: EventId,
}

/// Everything one side believes about the enemy.
#[derive(Clone, Debug)]
pub struct BeliefState {
    side: Side,
    contacts: BTreeMap<ContactId, Contact>,
    contact_ids: IdAllocator<ContactId>,
    /// Battle claims, in the order they arrived. They never age.
    claims: Vec<BattleClaim>,
    /// Contacts created or refreshed since the last ageing. Always empty
    /// between pulses, so not hashed.
    refreshed_this_pulse: BTreeSet<ContactId>,
}

impl BeliefState {
    pub(crate) fn new(side: Side) -> BeliefState {
        BeliefState {
            side,
            contacts: BTreeMap::new(),
            contact_ids: IdAllocator::new(),
            claims: Vec::new(),
            refreshed_this_pulse: BTreeSet::new(),
        }
    }

    pub fn side(&self) -> Side {
        self.side
    }

    /// The side's contacts, in ID order.
    pub fn contacts(&self) -> impl Iterator<Item = &Contact> {
        self.contacts.values()
    }

    pub fn contact(&self, id: ContactId) -> Option<&Contact> {
        self.contacts.get(&id)
    }

    /// The side's battle claims, in the order they arrived.
    pub fn claims(&self) -> &[BattleClaim] {
        &self.claims
    }

    /// The raw value the next contact ID will have. Part of the state hash.
    pub(crate) fn next_contact_raw(&self) -> u32 {
        self.contact_ids.next_raw()
    }

    /// True if a contact was created or refreshed and not yet aged. Must be
    /// false whenever the state is hashed.
    pub(crate) fn has_pending_refreshes(&self) -> bool {
        !self.refreshed_this_pulse.is_empty()
    }

    /// Writes a report into belief. With [`BeliefState::age_contacts`] this
    /// is the only way belief changes, and it is the only way a contact is
    /// created or refreshed or a claim stored (decision 13).
    ///
    /// A sighting: among contacts within `merge_distance_hexes` of the
    /// reported hex and no older than `merge_max_age_pulses`, the nearest is
    /// refreshed, the lowest ID on a tie. Otherwise the report makes a new
    /// contact. A battle claim is appended to the claims.
    pub(crate) fn apply_report(
        &mut self,
        report: &Report,
        figures: &SearchFigures,
    ) -> ApplyOutcome {
        match report {
            Report::Sighting(sighting) => {
                ApplyOutcome::Contact(self.apply_sighting(sighting, figures))
            }
            Report::BattleClaim(claim) => {
                self.claims.push(claim.clone());
                ApplyOutcome::Claim
            }
        }
    }

    fn apply_sighting(
        &mut self,
        report: &SightingReport,
        figures: &SearchFigures,
    ) -> ReportOutcome {
        // Iteration is in ID order and min_by_key keeps the first minimum, so
        // a tie goes to the lowest ID.
        let nearest = self
            .contacts
            .values()
            .filter(|contact| {
                contact.estimated_hex.distance(report.estimated_hex) <= figures.merge_distance_hexes
                    && contact.age_pulses <= figures.merge_max_age_pulses
            })
            .min_by_key(|contact| contact.estimated_hex.distance(report.estimated_hex))
            .map(|contact| contact.id);

        let (id, created) = match nearest {
            Some(id) => (id, false),
            None => (self.contact_ids.alloc(), true),
        };
        self.contacts.insert(
            id,
            Contact {
                id,
                estimated_hex: report.estimated_hex,
                estimated_ship_count: report.estimated_ship_count,
                estimated_kind: report.estimated_kind,
                age_pulses: 0,
                confidence: report.confidence,
                last_report_event: report.received_event,
            },
        );
        self.refreshed_this_pulse.insert(id);
        ReportOutcome {
            contact: id,
            created,
        }
    }

    /// Ages every contact not created or refreshed this pulse, drops those
    /// whose confidence falls below the floor, and clears the refreshed set
    /// (decision 14). Takes no truth. Returns one outcome per aged or dropped
    /// contact, in contact ID order.
    pub(crate) fn age_contacts(&mut self, figures: &SearchFigures) -> Vec<AgeOutcome> {
        let mut outcomes = Vec::new();
        let mut dropped = Vec::new();
        for contact in self.contacts.values_mut() {
            if self.refreshed_this_pulse.contains(&contact.id) {
                continue;
            }
            contact.age_pulses = contact.age_pulses.saturating_add(1);
            contact.confidence = contact.confidence * figures.confidence_decay_per_pulse;
            let drop = contact.confidence < figures.drop_below_confidence;
            if drop {
                dropped.push(contact.id);
            }
            outcomes.push(AgeOutcome {
                contact: contact.id,
                age_pulses: contact.age_pulses,
                confidence: contact.confidence,
                dropped: drop,
                last_report_event: contact.last_report_event,
            });
        }
        for id in dropped {
            self.contacts.remove(&id);
        }
        self.refreshed_this_pulse.clear();
        outcomes
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn figures() -> SearchFigures {
        SearchFigures::from_search_toml()
    }

    fn report(q: i32, r: i32, count: u32, event: u32) -> Report {
        Report::Sighting(SightingReport {
            estimated_hex: Hex::new(q, r),
            estimated_ship_count: count,
            estimated_kind: ShipKind::Cruiser,
            confidence: figures().initial_confidence,
            received_event: EventId::new(event),
        })
    }

    /// Applies a sighting and returns what it did to the contacts.
    fn apply(belief: &mut BeliefState, report: &Report) -> ReportOutcome {
        match belief.apply_report(report, &figures()) {
            ApplyOutcome::Contact(outcome) => outcome,
            ApplyOutcome::Claim => panic!("a sighting is not a claim"),
        }
    }

    /// Puts a contact straight into the state, to set up an age and a place
    /// the rules would not reach in one step.
    fn put_contact(belief: &mut BeliefState, q: i32, r: i32, age: u32) -> ContactId {
        let id = belief.contact_ids.alloc();
        belief.contacts.insert(
            id,
            Contact {
                id,
                estimated_hex: Hex::new(q, r),
                estimated_ship_count: 1,
                estimated_kind: ShipKind::Destroyer,
                age_pulses: age,
                confidence: Fixed::from_ratio(1, 2),
                last_report_event: EventId::new(1),
            },
        );
        id
    }

    #[test]
    fn a_report_becomes_a_contact() {
        let mut belief = BeliefState::new(Side::Allied);
        assert_eq!(belief.contacts().count(), 0);

        let outcome = apply(&mut belief, &report(4, 2, 3, 17));
        assert!(outcome.created);

        let contact = belief.contact(outcome.contact).expect("contact exists");
        assert_eq!(contact.estimated_hex, Hex::new(4, 2));
        assert_eq!(contact.estimated_ship_count, 3);
        assert_eq!(contact.estimated_kind, ShipKind::Cruiser);
        assert_eq!(contact.age_pulses, 0);
        assert_eq!(contact.confidence, Fixed::from_ratio(8, 10));
        assert_eq!(contact.last_report_event, EventId::new(17));
        assert_eq!(belief.contacts().count(), 1);
        assert!(belief.has_pending_refreshes());
    }

    #[test]
    fn a_merge_refreshes_the_contact_with_the_new_report() {
        let mut belief = BeliefState::new(Side::Allied);
        let id = put_contact(&mut belief, 0, 0, 1);
        let outcome = apply(&mut belief, &report(1, 0, 4, 20));
        // Distance 1 <= 2 and age 1 <= 2: refreshed, not created.
        assert_eq!(
            outcome,
            ReportOutcome {
                contact: id,
                created: false
            }
        );
        let contact = belief.contact(id).expect("contact");
        assert_eq!(contact.estimated_hex, Hex::new(1, 0));
        assert_eq!(contact.estimated_ship_count, 4);
        assert_eq!(contact.estimated_kind, ShipKind::Cruiser);
        assert_eq!(contact.age_pulses, 0);
        assert_eq!(contact.confidence, Fixed::from_ratio(8, 10));
        assert_eq!(contact.last_report_event, EventId::new(20));
        assert_eq!(belief.contacts().count(), 1);
    }

    #[test]
    fn a_merge_picks_the_nearest_qualifying_contact() {
        let mut belief = BeliefState::new(Side::Allied);
        // Report at (0,0). Contact 1 at (2,0) is distance 2, contact 2 at
        // (-1,0) is distance 1. Both qualify; the nearer, contact 2, wins
        // although its ID is higher.
        let far = put_contact(&mut belief, 2, 0, 0);
        let near = put_contact(&mut belief, -1, 0, 0);
        let outcome = apply(&mut belief, &report(0, 0, 1, 5));
        assert_eq!(outcome.contact, near);
        assert!(!outcome.created);
        assert_eq!(
            belief.contact(far).expect("far").estimated_hex,
            Hex::new(2, 0)
        );
    }

    #[test]
    fn a_tie_goes_to_the_lowest_contact_id() {
        let mut belief = BeliefState::new(Side::Allied);
        // Report at (0,0). (1,0) and (-1,0) are both distance 1.
        let first = put_contact(&mut belief, 1, 0, 0);
        let second = put_contact(&mut belief, -1, 0, 0);
        assert!(first < second);
        let outcome = apply(&mut belief, &report(0, 0, 1, 5));
        assert_eq!(outcome.contact, first);
        assert_eq!(
            belief.contact(second).expect("second").estimated_hex,
            Hex::new(-1, 0)
        );
    }

    #[test]
    fn a_contact_too_far_does_not_merge() {
        let mut belief = BeliefState::new(Side::Allied);
        // Distance 3 with a merge distance of 2.
        put_contact(&mut belief, 3, 0, 0);
        let outcome = apply(&mut belief, &report(0, 0, 1, 5));
        assert!(outcome.created);
        assert_eq!(belief.contacts().count(), 2);
    }

    #[test]
    fn a_contact_too_old_does_not_merge() {
        let mut belief = BeliefState::new(Side::Allied);
        // Age 3 with a maximum of 2, in the same hex.
        put_contact(&mut belief, 0, 0, 3);
        let outcome = apply(&mut belief, &report(0, 0, 1, 5));
        assert!(outcome.created);
        assert_eq!(belief.contacts().count(), 2);
    }

    #[test]
    fn a_contact_at_exactly_the_limits_merges() {
        let mut belief = BeliefState::new(Side::Allied);
        // Distance exactly 2 and age exactly 2.
        let id = put_contact(&mut belief, 2, 0, 2);
        let outcome = apply(&mut belief, &report(0, 0, 1, 5));
        assert_eq!(outcome.contact, id);
        assert!(!outcome.created);
    }

    #[test]
    fn a_lone_contact_ages_and_is_dropped_by_the_figures() {
        let mut belief = BeliefState::new(Side::Allied);
        let figures = figures();
        let id = apply(&mut belief, &report(0, 0, 1, 9)).contact;
        // The pulse that made it ages nothing.
        assert!(belief.age_contacts(&figures).is_empty());
        assert!(!belief.has_pending_refreshes());

        // Each ageing multiplies by 0.8, truncating to six decimals.
        // 0.8 x 0.8 = 0.64, x 0.8 = 0.512, x 0.8 = 0.4096, x 0.8 = 0.32768.
        let expected = [
            Fixed::from_ratio(64, 100),
            Fixed::from_ratio(512, 1000),
            Fixed::from_ratio(4096, 10_000),
            Fixed::from_ratio(32768, 100_000),
        ];
        for (index, confidence) in expected.into_iter().enumerate() {
            let outcomes = belief.age_contacts(&figures);
            let age = u32::try_from(index + 1).expect("small");
            assert_eq!(
                outcomes,
                vec![AgeOutcome {
                    contact: id,
                    age_pulses: age,
                    confidence,
                    dropped: false,
                    last_report_event: EventId::new(9),
                }]
            );
            assert_eq!(belief.contact(id).expect("kept").confidence, confidence);
        }

        // 0.32768 x 0.8 = 0.262144, which is below 0.3: dropped, and the
        // outcome still reports age 5 and the confidence it would have had.
        let outcomes = belief.age_contacts(&figures);
        assert_eq!(
            outcomes,
            vec![AgeOutcome {
                contact: id,
                age_pulses: 5,
                confidence: Fixed::from_ratio(262_144, 1_000_000),
                dropped: true,
                last_report_event: EventId::new(9),
            }]
        );
        assert!(belief.contact(id).is_none());
        assert!(belief.age_contacts(&figures).is_empty());
    }

    #[test]
    fn a_contact_refreshed_this_pulse_is_not_aged_until_the_next() {
        let mut belief = BeliefState::new(Side::Allied);
        let figures = figures();
        let old = put_contact(&mut belief, 20, 20, 0);
        let fresh = apply(&mut belief, &report(0, 0, 1, 9)).contact;

        // Only the old contact ages: 0.5 x 0.8 = 0.4 (>= 0.3, kept).
        let outcomes = belief.age_contacts(&figures);
        assert_eq!(outcomes.len(), 1);
        assert_eq!(outcomes[0].contact, old);
        assert_eq!(outcomes[0].age_pulses, 1);
        assert_eq!(outcomes[0].confidence, Fixed::from_ratio(4, 10));
        assert_eq!(belief.contact(fresh).expect("fresh").age_pulses, 0);

        // Next pulse the fresh one ages too: 0.8 x 0.8 = 0.64. The old one
        // goes 0.4 x 0.8 = 0.32, still kept.
        let outcomes = belief.age_contacts(&figures);
        assert_eq!(
            outcomes.iter().map(|o| o.contact).collect::<Vec<_>>(),
            vec![old, fresh]
        );
        assert_eq!(outcomes[1].age_pulses, 1);
        assert_eq!(outcomes[1].confidence, Fixed::from_ratio(64, 100));
        assert_eq!(outcomes[0].confidence, Fixed::from_ratio(32, 100));
    }

    #[test]
    fn a_refresh_in_the_pulse_a_contact_would_drop_saves_it() {
        let mut belief = BeliefState::new(Side::Allied);
        let figures = figures();
        let id = put_contact(&mut belief, 0, 0, 2);
        belief.contacts.get_mut(&id).expect("contact").confidence = Fixed::from_ratio(31, 100);
        apply(&mut belief, &report(0, 0, 1, 9));
        // Refreshed: no ageing, so no drop, and confidence is the report's.
        assert!(belief.age_contacts(&figures).is_empty());
        assert_eq!(
            belief.contact(id).expect("kept").confidence,
            Fixed::from_ratio(8, 10)
        );
    }

    #[test]
    fn a_battle_claim_is_stored_and_holds_no_truth_id() {
        let mut belief = BeliefState::new(Side::Japanese);
        assert!(belief.claims().is_empty());
        let claim = BattleClaim {
            hex: Hex::new(5, 0),
            day: 1,
            pulse: Pulse::Night,
            claimed_sunk: 2,
            received_event: EventId::new(40),
        };
        let outcome = belief.apply_report(&Report::BattleClaim(claim.clone()), &figures());
        assert_eq!(outcome, ApplyOutcome::Claim);
        assert_eq!(belief.claims(), std::slice::from_ref(&claim));
        // A claim is not a contact, and needs no ageing.
        assert_eq!(belief.contacts().count(), 0);
        assert!(!belief.has_pending_refreshes());
        assert!(belief.age_contacts(&figures()).is_empty());
        assert_eq!(belief.claims(), std::slice::from_ref(&claim));

        // Claims keep the order they arrived in.
        let later = BattleClaim {
            hex: Hex::new(6, 0),
            day: 2,
            pulse: Pulse::Morning,
            claimed_sunk: 0,
            received_event: EventId::new(90),
        };
        belief.apply_report(&Report::BattleClaim(later.clone()), &figures());
        assert_eq!(belief.claims().len(), 2);
        assert_eq!(belief.claims()[1], later);

        // The struct is exhaustive here on purpose: adding a field, such as
        // a truth ID, breaks this destructuring and forces a look.
        let BattleClaim {
            hex: _,
            day: _,
            pulse: _,
            claimed_sunk: _,
            received_event: _,
        } = belief.claims()[0].clone();
    }
}
