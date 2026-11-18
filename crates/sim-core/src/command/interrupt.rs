//! The test of interrupts (decision 0107, step 4, rulings 31 to 33): each
//! morning, after the staff has handed in and its notices are recorded and
//! before the day's orders are taken in, each side's interrupts are tested on
//! the reading of that day and on what the staff handed in.
//!
//! An interrupt is armed when it is set. On a morning its condition holds and
//! it is armed, it fires and is no longer armed; on a morning its condition
//! does not hold, it is armed again. One already true when it is set fires
//! that morning, once. Acknowledging marks a firing seen and arms nothing.
//!
//! Two kinds. The commander sets one: a base's days of supply below a number,
//! which does not hold where the hex has no figure. Three stand without being
//! set and may be switched off: the staff cannot meet a plan's intent and a
//! plan waits for the commander's leave (both read from the staff's notices
//! of the morning), and a ship of the side was lost the day before (read from
//! the log, and only for a side with an intent in force). A standing
//! interrupt is armed apart for each plan, or each ship.
//!
//! Reads: the side's set interrupts and standing records, the staff's notices
//! of the morning, the days of supply of the bases the set interrupts name,
//! the truth's bases and ships (the side that owns each), and the log of the
//! day before. Writes: the interrupts' status in the side's command state,
//! and an `InterruptFired` event for each firing, citing the command that
//! set it, the notices of the morning, or the event that sank the ship, with
//! a breakdown of a set one's factors when breakdowns are on (a breakdown
//! never changes what happens or the hash). Draws no random number.

use std::collections::BTreeMap;

use crate::command::CommandKind;
use crate::command::{
    InterruptCondition, InterruptId, InterruptRef, InterruptSetting, InterruptStatus, NoticeKind,
    StaffNotice, StandingKind, StandingSubject,
};
use crate::events::{Breakdown, Cause, EventKind, Factor};
use crate::fixed::Fixed;
use crate::ids::{EventId, ShipId};
use crate::state::Side;
use crate::view::DaysOfSupply;
use crate::world::World;

/// What the morning's reading gave for each set interrupt of a side: the
/// days of supply of its base's hex, `None` where the hex has none.
pub(crate) type Readings = BTreeMap<Side, BTreeMap<InterruptId, Option<DaysOfSupply>>>;

/// Whether `figure` is below `days` days: the condition of a set interrupt.
/// 35 days is not below 35.
pub(crate) fn figure_is_below(figure: Option<Fixed>, days: u32) -> bool {
    figure.is_some_and(|figure| figure < Fixed::from_int(i64::from(days)))
}

/// Fires `status` if it is armed, as the test of a morning does: it is no
/// longer armed, it last fired today and is not acknowledged. Returns whether
/// it fired.
fn fire(status: &mut InterruptStatus, day: u32) -> bool {
    if !status.armed {
        return false;
    }
    status.armed = false;
    status.last_fired = Some(day);
    status.acknowledged = false;
    true
}

impl World {
    /// Reads, for each side, the days of supply of the hex of every base a
    /// set interrupt names, as the staff was asked on it. A side with no set
    /// interrupt is not in the answer.
    pub(crate) fn read_interrupt_figures(&self) -> Readings {
        let mut readings = Readings::new();
        for side in Side::ALL {
            let state = &self.command[&side];
            if state.interrupts.is_empty() {
                continue;
            }
            // The commander and the area staff read what the area command
            // holds, not the truth (decision 0136, 9): the base's place and
            // the days of supply of its hex.
            let known = self
                .side_view(side)
                .known_to(crate::pictures::CommandRef::Area);
            let view = known.view();
            let mut figures = BTreeMap::new();
            for (id, interrupt) in &state.interrupts {
                let InterruptCondition::SupplyBelow { base, .. } = interrupt.condition;
                let reading = view
                    .bases()
                    .find(|found| found.id == base)
                    .and_then(|found| view.days_of_supply(found.hex));
                figures.insert(*id, reading);
            }
            readings.insert(side, figures);
        }
        readings
    }

    /// Step 5 of the day, its second part: tests each side's interrupts, in
    /// side order, on `readings` and on the `notices` the staff handed in
    /// (each with the event that recorded it).
    pub(crate) fn test_interrupts(
        &mut self,
        day: u32,
        readings: &Readings,
        notices: &BTreeMap<Side, Vec<(StaffNotice, EventId)>>,
    ) {
        for side in Side::ALL {
            self.test_set_interrupts(day, side, readings);
            self.test_standing_interrupts(day, side, notices);
        }
    }

    /// The set interrupts of `side`, by number: a condition that holds fires
    /// an armed interrupt; one that does not hold arms it again.
    fn test_set_interrupts(&mut self, day: u32, side: Side, readings: &Readings) {
        let ids: Vec<InterruptId> = self.command[&side].interrupts.keys().copied().collect();
        for id in ids {
            let interrupt = self.command[&side].interrupts[&id];
            let InterruptCondition::SupplyBelow { days, .. } = interrupt.condition;
            let reading = readings
                .get(&side)
                .and_then(|figures| figures.get(&id))
                .copied()
                .flatten();
            let figure = reading.and_then(|reading| reading.days);
            let state = self.command.get_mut(&side).expect("side has command state");
            let status = &mut state
                .interrupts
                .get_mut(&id)
                .expect("interrupt exists")
                .status;
            if !figure_is_below(figure, days) {
                status.armed = true;
                continue;
            }
            if !fire(status, day) {
                continue;
            }
            let causes: Vec<Cause> = self
                .events
                .latest_where(|kind| {
                    matches!(
                        kind,
                        EventKind::CommandTaken {
                            side: owner,
                            command: CommandKind::SetInterrupt(InterruptSetting::Set {
                                interrupt: set,
                                ..
                            }),
                            ..
                        } if *owner == side && *set == id
                    )
                })
                .map(Cause::Event)
                .into_iter()
                .collect();
            let breakdown = self.config.record_breakdowns.then(|| {
                let reading = reading.expect("a figure came from a reading");
                Breakdown {
                    factors: vec![
                        Factor {
                            name: "stock_t",
                            value: reading.stock_t,
                        },
                        Factor {
                            name: "planned_use_t",
                            value: reading.planned_use.total_t(),
                        },
                        Factor {
                            name: "days_of_supply",
                            value: figure.expect("the condition held"),
                        },
                        Factor {
                            name: "threshold_days",
                            value: Fixed::from_int(i64::from(days)),
                        },
                    ],
                    result: figure.expect("the condition held"),
                }
            });
            self.events.record(
                day,
                None,
                EventKind::InterruptFired {
                    side,
                    interrupt: InterruptRef::Set(id),
                    condition: Some(interrupt.condition),
                    figure,
                },
                causes,
                breakdown,
            );
        }
    }

    /// The standing interrupts of `side`, each kind not switched off, in
    /// kind order. The subjects whose condition holds this morning fire
    /// where armed; every record of the kind whose subject does not hold is
    /// armed again. A kind switched off is not tested and its records stay.
    fn test_standing_interrupts(
        &mut self,
        day: u32,
        side: Side,
        notices: &BTreeMap<Side, Vec<(StaffNotice, EventId)>>,
    ) {
        for kind in StandingKind::ALL {
            if self.command[&side].standing_off.contains(&kind) {
                continue;
            }
            let holding = self.standing_subjects_holding(day, side, kind, notices);
            for (subject, causes) in &holding {
                let key = (kind, *subject);
                let state = self.command.get_mut(&side).expect("side has command state");
                let mut status = state.standing.get(&key).copied().unwrap_or_default();
                if !fire(&mut status, day) {
                    continue;
                }
                state.standing.insert(key, status);
                self.events.record(
                    day,
                    None,
                    EventKind::InterruptFired {
                        side,
                        interrupt: InterruptRef::Standing {
                            kind,
                            subject: *subject,
                        },
                        condition: None,
                        figure: None,
                    },
                    causes.iter().copied().map(Cause::Event).collect(),
                    None,
                );
            }
            let state = self.command.get_mut(&side).expect("side has command state");
            for ((record_kind, subject), status) in state.standing.iter_mut() {
                if *record_kind == kind && !holding.contains_key(subject) {
                    status.armed = true;
                }
            }
        }
    }

    /// The subjects of a standing kind whose condition holds this morning,
    /// each with the events that make it hold: the notices of the morning for
    /// a plan's kinds, the event that sank the ship for a ship lost.
    fn standing_subjects_holding(
        &self,
        day: u32,
        side: Side,
        kind: StandingKind,
        notices: &BTreeMap<Side, Vec<(StaffNotice, EventId)>>,
    ) -> BTreeMap<StandingSubject, Vec<EventId>> {
        let mut holding: BTreeMap<StandingSubject, Vec<EventId>> = BTreeMap::new();
        match kind {
            StandingKind::CannotMeet
            | StandingKind::WaitsForLeave
            | StandingKind::CannotBringEnough => {
                for (notice, event) in notices.get(&side).into_iter().flatten() {
                    let counts = matches!(
                        (kind, notice.kind),
                        (StandingKind::CannotMeet, NoticeKind::CannotMeet)
                            | (StandingKind::WaitsForLeave, NoticeKind::WaitsForLeave(_))
                            | (
                                StandingKind::CannotBringEnough,
                                NoticeKind::CannotBringEnough
                            )
                    );
                    if counts {
                        holding
                            .entry(StandingSubject::Plan(notice.plan))
                            .or_default()
                            .push(*event);
                    }
                }
            }
            StandingKind::ShipLost => {
                // No game without an intent gets a new event.
                if self.command[&side].intents_in_force().next().is_none() {
                    return holding;
                }
                let Some(before) = day.checked_sub(1) else {
                    return holding;
                };
                for (ship, sinking) in self.ships_lost_on(side, before) {
                    holding
                        .entry(StandingSubject::Ship(ship))
                        .or_default()
                        .push(sinking);
                }
            }
        }
        holding
    }

    /// The ships of `side` that were lost on `day`, each with the event that
    /// sank it: a `ShipSunk` in a fight, or a `ShipLostWithBase` in port.
    /// The log is walked back from its end to the start of the day.
    fn ships_lost_on(&self, side: Side, day: u32) -> Vec<(ShipId, EventId)> {
        let mut lost: BTreeMap<ShipId, EventId> = BTreeMap::new();
        for event in self.events.of_day_latest_first(day) {
            let ship = match &event.kind {
                EventKind::ShipSunk { ship, .. } | EventKind::ShipLostWithBase { ship, .. } => {
                    *ship
                }
                _ => continue,
            };
            if self
                .truth
                .ships
                .get(&ship)
                .is_some_and(|found| found.side == side)
            {
                lost.entry(ship).or_insert(event.id);
            }
        }
        lost.into_iter().collect()
    }
}
