//! Suspected waters (decision 0218, 23 to 26 and 47): the waters where a
//! side believes enemy submarines work, a belief a scenario gives it on a
//! day, right or wrong, never made from where the boats are.
//!
//! It is told as the opening estimates are (decision 0160): taken in as a
//! report through [`BeliefState::apply_report`], held by every command at
//! once, recorded as an event the side sees, hashed, and in every picture.
//! An entry of day 1 is told at setup, on day 0. An entry of day D after it
//! is told at the close of day D - 1, after its end phases, so that day D's
//! briefing and view hold it and day D - 1's do not; until then it waits in
//! `TruthState::waters_waiting`. In play the waters never change.
//!
//! No draw is made and nothing here reads or writes any other state.
//!
//! [`BeliefState::apply_report`]: crate::belief::BeliefState

use crate::belief::{ApplyOutcome, Report, SuspectedWaters};
use crate::events::EventKind;
use crate::pictures::Holders;
use crate::state::WatersWaiting;
use crate::world::{SimError, World};

impl World {
    /// Gives each side the waters a scenario says it suspects, before day 1:
    /// those of day 1 are told now, as the opening estimates are, and the
    /// rest are kept to be told at the close of the day before their day.
    /// What was kept before is replaced, so that a world made again from the
    /// same scenario holds the same. Fails, changing nothing, if the game
    /// has begun, or if an entry is off the map or has day 0. An error names the
    /// entry.
    pub fn hold_suspected_waters(&mut self, entries: Vec<WatersWaiting>) -> Result<(), SimError> {
        if self.truth.day > 0 {
            return Err(SimError::InvalidSetup(
                "suspected waters can only be given before day 1".to_string(),
            ));
        }
        if entries.is_empty() {
            return Ok(());
        }
        let search = self.ref_data.search.clone();
        for entry in &entries {
            let describe = || {
                format!(
                    "the {} suspected waters at {} told on day {}",
                    entry.side.name(),
                    entry.hex,
                    entry.day
                )
            };
            if self.map.get(entry.hex).is_none() {
                return Err(SimError::InvalidSetup(format!(
                    "{} are off the map",
                    describe()
                )));
            }
            if entry.day == 0 {
                return Err(SimError::InvalidSetup(format!(
                    "{} cannot be told on day 0",
                    describe()
                )));
            }
        }
        let mut staged = self.clone();
        staged.truth.waters_waiting = Vec::new();
        for entry in entries {
            if entry.day == 1 {
                staged.tell_waters(0, &entry, &search);
            } else {
                staged.truth.waters_waiting.push(entry);
            }
        }
        *self = staged;
        Ok(())
    }

    /// Tells every side the waters whose day is `day_closed + 1` or earlier
    /// and not yet told, in the order kept: called at the close of day
    /// `day_closed`, after its end phases.
    pub(crate) fn tell_waters_due(&mut self, day_closed: u32) {
        if self.truth.waters_waiting.is_empty() {
            return;
        }
        let search = self.ref_data.search.clone();
        let (due, later): (Vec<WatersWaiting>, Vec<WatersWaiting>) =
            std::mem::take(&mut self.truth.waters_waiting)
                .into_iter()
                .partition(|waters| waters.day <= day_closed + 1);
        self.truth.waters_waiting = later;
        for entry in due {
            self.tell_waters(day_closed, &entry, &search);
        }
    }

    /// Takes one entry into the side's belief and every command's picture
    /// and records that it was told, in no pulse and citing nothing, on
    /// `event_day`.
    fn tell_waters(
        &mut self,
        event_day: u32,
        entry: &WatersWaiting,
        search: &crate::refdata::SearchFigures,
    ) {
        let received_event = self.events.next_id();
        let waters = SuspectedWaters {
            hex: entry.hex,
            distance_hexes: entry.distance_hexes,
            words: entry.words.clone(),
            rows: entry.rows.clone(),
            reading: entry.reading.clone(),
            day: entry.day,
            received_event,
        };
        let outcome = self.apply_report(
            entry.side,
            &Report::SuspectedWaters(waters),
            &Holders::Every,
            search,
        );
        debug_assert_eq!(outcome, ApplyOutcome::SuspectedWaters);
        let recorded = self.events.record(
            event_day,
            None,
            EventKind::SuspectedWatersTold {
                side: entry.side,
                hex: entry.hex,
                distance: entry.distance_hexes,
                day: entry.day,
                words: entry.words.clone(),
            },
            Vec::new(),
            None,
        );
        assert_eq!(
            recorded, received_event,
            "the report cites the event that records it"
        );
    }
}
