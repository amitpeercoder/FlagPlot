//! The submarine's rules in play (decision 0218, stage 2): her passage to a
//! station or to home. The attack on forces in her hex, the hunt, the
//! claims and the reports come in the next phase; this one is the part that
//! needs only the boat and the map.
//!
//! The passage is worked at the very end of the movement step
//! ([`World::submarines_after_movement`], called from
//! `move_task_forces`), where the hours each force spent in each hex are in
//! hand (decision 0218, 37 and 38) for the attack.
//!
//! - A passage that ended (`arrived`) in an earlier pulse is cleared first
//!   thing: she is on station from the pulse after she arrives (rule 43).
//! - In the night pulse of her leave day, or in her first pulse on the map
//!   after it, a boat not already going home leaves for home (rule 3).
//! - A boat on passage steams her class's passage speed times the pulse's
//!   hours along the sea route a force would take: the same distance field,
//!   the same step and the same charge, entering a hex at half a leg
//!   (decisions 0017 and 0058). She attacks nothing on the way.
//! - At the end of a passage she is on station, or, going home, she is
//!   taken off the map at her base's hex.

use crate::events::{Cause, EventKind, SubmarineLeftReason};
use crate::fixed::Fixed;
use crate::hex::Hex;
use crate::ids::{EventId, SubmarineId};
use crate::map::Map;
use crate::movement::HoursInHex;
use crate::state::{SubmarineCondition, SubmarinePassage};
use crate::time::Pulse;
use crate::world::World;

/// Steams a boat's passage up to `budget_nm` and returns the miles steamed.
/// `hex` is where she is (the hex she has entered); it changes when she
/// passes half a leg and at a centre. Stops at the end of the passage, or
/// where the sea gives no way on.
fn steam_passage(
    map: &Map,
    hex: &mut Hex,
    passage: &mut SubmarinePassage,
    budget_nm: Fixed,
) -> Fixed {
    let field = map.sea_distances_to(passage.to);
    let mut left_nm = budget_nm;
    let mut steamed_nm = Fixed::ZERO;
    loop {
        if passage.toward_hex.is_none() {
            // At a centre: she has arrived, or sets out on the next leg.
            if *hex == passage.to || left_nm.is_zero() {
                break;
            }
            let Some(next) = map.next_step_in(&field, *hex) else {
                break;
            };
            let charge = map.step_charge_nm(&field, *hex, next, passage.to);
            passage.leg_nm = Fixed::from_int(i64::from(charge));
            passage.toward_hex = Some(next);
            passage.progress_nm = Fixed::ZERO;
        }
        if left_nm.is_zero() {
            break;
        }
        let toward = passage.toward_hex.expect("she is on a leg");
        let half_leg = passage.leg_nm / Fixed::from_int(2);
        let to_centre_nm = passage.leg_nm - passage.progress_nm;
        let piece_nm = left_nm.min(to_centre_nm);
        if passage.progress_nm < half_leg && passage.progress_nm + piece_nm >= half_leg {
            *hex = toward;
        }
        left_nm -= piece_nm;
        steamed_nm += piece_nm;
        if piece_nm == to_centre_nm {
            *hex = toward;
            passage.toward_hex = None;
            passage.progress_nm = Fixed::ZERO;
            passage.leg_nm = Fixed::ZERO;
        } else {
            passage.progress_nm += piece_nm;
        }
    }
    steamed_nm
}

impl World {
    /// What the submarines do at the end of the movement step of a pulse,
    /// with `hours` the hours each force with a ship afloat spent in each
    /// hex in it (decision 0218, 37 and 38). This phase works the passage
    /// only; the attack on the forces in a boat's hex is the next.
    pub(crate) fn submarines_after_movement(
        &mut self,
        day: u32,
        pulse: Pulse,
        _hours: &HoursInHex,
    ) {
        let boats: Vec<SubmarineId> = self
            .truth
            .submarines
            .values()
            .filter(|boat| boat.on_map(day))
            .map(|boat| boat.id)
            .collect();
        for id in boats {
            self.run_passage(day, pulse, id);
        }
    }

    /// Sends a boat for home (decision 0218, 3, 21 and 43): a passage to her
    /// home base's hex, with `SubmarineLeftForHome` for `reason` citing
    /// `causes`. A leg she is on is finished first, as a force finishes its
    /// leg when its route changes. Nothing if she is already going home.
    /// Returns the event.
    pub(crate) fn send_home(
        &mut self,
        day: u32,
        pulse: Pulse,
        boat: SubmarineId,
        reason: SubmarineLeftReason,
        causes: Vec<Cause>,
    ) -> Option<EventId> {
        let home = self.truth.submarines.get(&boat)?.home;
        let base = self.truth.bases.get(&home)?.hex;
        let boat_state = self.truth.submarines.get_mut(&boat)?;
        if boat_state.passage.is_some_and(|passage| passage.home) {
            return None;
        }
        let (toward_hex, progress_nm, leg_nm) = match boat_state.passage {
            Some(passage) if passage.toward_hex.is_some() => {
                (passage.toward_hex, passage.progress_nm, passage.leg_nm)
            }
            _ => (None, Fixed::ZERO, Fixed::ZERO),
        };
        boat_state.passage = Some(SubmarinePassage {
            to: base,
            home: true,
            toward_hex,
            progress_nm,
            leg_nm,
            arrived: false,
            order: None,
        });
        Some(self.events.record(
            day,
            Some(pulse),
            EventKind::SubmarineLeftForHome {
                submarine: boat,
                base: home,
                reason,
            },
            causes,
            None,
        ))
    }

    /// One boat's part of the pulse: the end of an earlier arrival, the
    /// leave day, and the passage.
    fn run_passage(&mut self, day: u32, pulse: Pulse, id: SubmarineId) {
        let boat = self.truth.submarines.get_mut(&id).expect("the boat exists");
        if boat.passage.is_some_and(|passage| passage.arrived) {
            boat.passage = None;
        }
        let going_home = boat.passage.is_some_and(|passage| passage.home);
        let leaves = boat
            .leave_day
            .is_some_and(|leave| day > leave || (day == leave && pulse == Pulse::Night));
        if leaves && !going_home {
            self.send_home(day, pulse, id, SubmarineLeftReason::LeaveDay, Vec::new());
        }

        let boat = &self.truth.submarines[&id];
        let Some(passage) = boat.passage else {
            return;
        };
        let Some(class) = self
            .ref_data
            .submarines
            .as_ref()
            .and_then(|data| data.classes.get(&boat.class))
        else {
            return;
        };
        let budget_nm = class.passage_speed_kn * Fixed::from_int(pulse.hours());
        let (side_home, from) = (boat.home, boat.hex);
        let mut hex = from;
        let mut passage = passage;
        let steamed_nm = steam_passage(&self.map, &mut hex, &mut passage, budget_nm);
        let reached = passage.toward_hex.is_none() && hex == passage.to;

        // Her steaming cites the order that sent her, or, going home, what
        // sent her.
        let mut causes: Vec<Cause> = passage.order.map(Cause::Order).into_iter().collect();
        if causes.is_empty() && passage.home {
            let sent = self
                .events
                .iter()
                .filter(|event| {
                    matches!(
                        event.kind,
                        EventKind::SubmarineLeftForHome { submarine, .. } if submarine == id
                    )
                })
                .map(|event| event.id)
                .last();
            causes.extend(sent.map(Cause::Event));
        }
        let steamed = (!steamed_nm.is_zero()).then(|| {
            self.events.record(
                day,
                Some(pulse),
                EventKind::SubmarineSteamed {
                    submarine: id,
                    from,
                    to: hex,
                },
                causes.clone(),
                None,
            )
        });
        let causes = steamed.map_or(causes, |event| vec![Cause::Event(event)]);

        let boat = self.truth.submarines.get_mut(&id).expect("the boat exists");
        boat.hex = hex;
        boat.passage = Some(passage);
        if !reached {
            return;
        }
        if passage.home {
            boat.condition = SubmarineCondition::Home;
            boat.passage = None;
            self.events.record(
                day,
                Some(pulse),
                EventKind::SubmarineReachedHome {
                    submarine: id,
                    base: side_home,
                },
                causes,
                None,
            );
        } else {
            boat.passage = Some(SubmarinePassage {
                arrived: true,
                ..passage
            });
            self.events.record(
                day,
                Some(pulse),
                EventKind::SubmarineOnStation { submarine: id, hex },
                causes,
                None,
            );
        }
    }
}
