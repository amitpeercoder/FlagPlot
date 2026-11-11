//! Contacts a side starts the game holding (decision 8 of
//! `docs/decisions/0021-readiness-and-alarm.md`).
//!
//! Each is entered as a report, through [`BeliefState::apply_report`], and
//! aged by the number of pulses given, through [`BeliefState::age_contacts`],
//! so belief is still written only from reports. Every report and every
//! ageing is an event on day 0, before the first day.
//!
//! [`BeliefState::age_contacts`] skips a contact created in the same pulse,
//! so one report followed by one ageing leaves the contact at age 0. To give
//! each contact exactly its age the set-up runs virtual pulses before day 1.
//! For a side whose oldest entry is `M` pulses old, for `t` from `M` down to 0
//! the entries of age `t` are applied (in the order given) and then the
//! contacts age once. An entry of age `N` is applied in virtual pulse `N`,
//! where the ageing skips it because it was just made, and then ages in each
//! of the `N` pulses after, so it ends at age `N` with `N` ageing events. The
//! refreshed set is clear when the last ageing is done, so an age-0 contact
//! ages at the end of day 1's night pulse like any other.

use std::collections::BTreeMap;

use crate::belief::{ApplyOutcome, Report, SightingReport};
use crate::events::{Cause, EventKind};
use crate::hex::Hex;
use crate::ids::ContactId;
use crate::refdata::ShipKind;
use crate::state::Side;
use crate::world::{SimError, World};

/// A contact one side holds when the game begins.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct StartingContact {
    pub side: Side,
    /// Where the contact is believed to be, exactly.
    pub hex: Hex,
    /// How many ships the contact is believed to be, exactly. At least 1.
    pub ship_count: u32,
    pub kind: ShipKind,
    /// How many pulses old the contact is when day 1 begins.
    pub age_pulses: u32,
}

impl StartingContact {
    fn describe(&self) -> String {
        format!(
            "{} starting contact at {} ({} {}, {} pulses old)",
            self.side.name(),
            self.hex,
            self.ship_count,
            self.kind.name(),
            self.age_pulses
        )
    }
}

impl World {
    /// Gives sides contacts to hold before day 1. Takes effect completely or,
    /// on an error, not at all.
    ///
    /// Fails if the game has begun (a day has been resolved or any event is
    /// recorded, including an earlier call: ageing again would change the
    /// ages already given), if there are no search figures, if an entry has
    /// no ships, if an entry's report would refresh a contact instead of
    /// making a new one (two entries too close together, as the search
    /// figures merge them), or if an entry would be dropped by ageing before
    /// day 1. An error names the entry.
    pub fn hold_starting_contacts(
        &mut self,
        contacts: Vec<StartingContact>,
    ) -> Result<(), SimError> {
        if self.truth.day > 0 || !self.events.is_empty() {
            return Err(SimError::InvalidSetup(
                "starting contacts can only be given before day 1, and once".to_string(),
            ));
        }
        if contacts.is_empty() {
            return Ok(());
        }
        if self.ref_data.search.is_none() {
            return Err(SimError::InvalidSetup(
                "there are starting contacts but no search figures".to_string(),
            ));
        }
        for contact in &contacts {
            if contact.ship_count == 0 {
                return Err(SimError::InvalidSetup(format!(
                    "{} has no ships",
                    contact.describe()
                )));
            }
        }
        let mut staged = self.clone();
        for side in Side::ALL {
            let entries: Vec<&StartingContact> =
                contacts.iter().filter(|c| c.side == side).collect();
            staged.hold_for_side(side, &entries)?;
        }
        *self = staged;
        Ok(())
    }

    fn hold_for_side(&mut self, side: Side, entries: &[&StartingContact]) -> Result<(), SimError> {
        let Some(oldest) = entries.iter().map(|c| c.age_pulses).max() else {
            return Ok(());
        };
        let figures = self.ref_data.search.clone().expect("checked by the caller");
        // Which entry made which contact, to name it in an error.
        let mut made: BTreeMap<ContactId, &StartingContact> = BTreeMap::new();
        for pulse in (0..=oldest).rev() {
            for entry in entries.iter().filter(|c| c.age_pulses == pulse) {
                let report = SightingReport {
                    estimated_hex: entry.hex,
                    estimated_ship_count: entry.ship_count,
                    estimated_kind: entry.kind,
                    received_event: self.events.next_id(),
                };
                let ApplyOutcome::Contact(outcome) = self
                    .beliefs
                    .get_mut(&side)
                    .expect("both sides have a belief state")
                    .apply_report(&Report::Sighting(report.clone()), &figures)
                else {
                    unreachable!("a sighting report makes or refreshes a contact");
                };
                if !outcome.created {
                    return Err(SimError::InvalidSetup(format!(
                        "{} is too close to another starting contact: its report would refresh that contact, not make a new one",
                        entry.describe()
                    )));
                }
                let received = self.events.record(
                    0,
                    None,
                    EventKind::ReportReceived {
                        side,
                        contact: outcome.contact,
                        hex: report.estimated_hex,
                        ship_count: report.estimated_ship_count,
                        kind: report.estimated_kind,
                        created: true,
                    },
                    Vec::new(),
                    None,
                );
                assert_eq!(
                    received, report.received_event,
                    "the report cites the event that records it"
                );
                made.insert(outcome.contact, entry);
            }
            let outcomes = self
                .beliefs
                .get_mut(&side)
                .expect("both sides have a belief state")
                .age_contacts(&figures);
            for outcome in outcomes {
                if outcome.dropped {
                    let entry = made.get(&outcome.contact).expect("every contact was made");
                    return Err(SimError::InvalidSetup(format!(
                        "{} would be dropped before day 1: it reaches {} pulses, the age at which a contact is dropped",
                        entry.describe(),
                        figures.contact_drop_age_pulses
                    )));
                }
                self.events.record(
                    0,
                    None,
                    EventKind::ContactAged {
                        side,
                        contact: outcome.contact,
                        age_pulses: outcome.age_pulses,
                    },
                    vec![Cause::Event(outcome.last_report_event)],
                    None,
                );
            }
        }
        Ok(())
    }
}
