//! Belief state: what one side thinks is out there.
//!
//! A side never reads the truth about the enemy. It holds contacts, and a
//! contact only ever comes from a [`Report`]. The single way to change a
//! belief state is [`BeliefState::apply_report`], which keeps the invariant
//! "no belief-state data was written from anything except a report" true by
//! construction.
//!
//! This is a scaffold. No rule generates reports yet. Search arrives with
//! scenario 3, and with it the detection and error model, contact ageing and
//! merging.

use std::collections::BTreeMap;

use crate::fixed::Fixed;
use crate::hex::Hex;
use crate::ids::{ContactId, IdAllocator};
use crate::state::Side;

/// What a sighting claims. It may be wrong about everything.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Report {
    pub estimated_hex: Hex,
    pub estimated_ship_count: u32,
    /// How sure the observer is, where `Fixed::ONE` is certain.
    pub confidence: Fixed,
}

/// Something one side believes is at sea.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Contact {
    pub id: ContactId,
    pub estimated_hex: Hex,
    pub estimated_ship_count: u32,
    /// Pulses since the last fresh sighting.
    pub age_pulses: u32,
    pub confidence: Fixed,
}

/// Everything one side believes about the enemy.
#[derive(Clone, Debug)]
pub struct BeliefState {
    side: Side,
    contacts: BTreeMap<ContactId, Contact>,
    contact_ids: IdAllocator<ContactId>,
}

impl BeliefState {
    pub(crate) fn new(side: Side) -> BeliefState {
        BeliefState {
            side,
            contacts: BTreeMap::new(),
            contact_ids: IdAllocator::new(),
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

    /// The raw value the next contact ID will have. Part of the state hash.
    pub(crate) fn next_contact_raw(&self) -> u32 {
        self.contact_ids.next_raw()
    }

    /// Writes a report into belief. This is the only way belief changes.
    // Not called by any rule yet: search arrives with scenario 3.
    #[allow(dead_code)]
    pub(crate) fn apply_report(&mut self, report: &Report) -> ContactId {
        let id = self.contact_ids.alloc();
        self.contacts.insert(
            id,
            Contact {
                id,
                estimated_hex: report.estimated_hex,
                estimated_ship_count: report.estimated_ship_count,
                age_pulses: 0,
                confidence: report.confidence,
            },
        );
        id
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_report_becomes_a_contact() {
        let mut belief = BeliefState::new(Side::Allied);
        assert_eq!(belief.contacts().count(), 0);

        let id = belief.apply_report(&Report {
            estimated_hex: Hex::new(4, 2),
            estimated_ship_count: 3,
            confidence: Fixed::from_ratio(1, 2),
        });

        let contact = belief.contact(id).expect("contact exists");
        assert_eq!(contact.estimated_hex, Hex::new(4, 2));
        assert_eq!(contact.estimated_ship_count, 3);
        assert_eq!(contact.age_pulses, 0);
        assert_eq!(belief.contacts().count(), 1);
    }
}
