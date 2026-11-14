//! Preparing a landing (`docs/decisions/0095-preparing-a-landing.md`).
//!
//! A force is loaded for a landing and each land unit of its roster rehearses
//! it. Loading speeds the landing (the cargo rule asks
//! [`landing_rate_factor`]) and rehearsal spares the men on landing (the
//! cargo rule asks [`landing_fatigue_gain`]). Both are gained by the day, in
//! the ground combat phase, and lost by change (the order's application
//! here, a split) or spent by the landing (`cargo.rs`). There is no
//! randomness.
//!
//! Reads: a force's side, position, status, ships, roster and loading; a
//! land unit's men, place and rehearsal; the bases of the side at the force's
//! hex; the ground figures. Writes: a force's preparation; a land unit's
//! rehearsal and its hex; the rosters; the events.

use std::collections::{BTreeMap, BTreeSet};

use crate::events::{
    Breakdown, Cause, EventKind, Factor, LandFatigueChange, LoadingLoss, RehearsalLoss, RosterLeft,
};
use crate::fixed::Fixed;
use crate::hex::Hex;
use crate::ids::{EventId, LandUnitId, OrderId, TaskForceId};
use crate::refdata::GroundFigures;
use crate::state::{Preparation, TaskForceStatus};
use crate::time::{pulse_start, Pulse};
use crate::world::World;

const HUNDRED: Fixed = Fixed::from_int(100);

/// The factor a force's work is multiplied by while it lands men (decision 5
/// of 0095): the share of the rate an unprepared force works at plus the rest
/// of 1 times its loading over 100. Loading counts only at the hex of the
/// order; at any other hex the force works as one with none. Returns the
/// factor and the loading counted.
pub(crate) fn landing_rate_factor(
    figures: &GroundFigures,
    preparation: Option<&Preparation>,
    hex: Hex,
) -> (Fixed, Fixed) {
    let counted = match preparation {
        Some(preparation) if preparation.hex == hex => preparation.loading,
        _ => Fixed::ZERO,
    };
    let share = figures.unprepared_landing_share;
    (
        share + (Fixed::ONE - share).mul_div(counted, HUNDRED),
        counted,
    )
}

/// The fatigue a unit gains when its first men land in `hex` (decision 6 of
/// 0095): `unprepared_landing_fatigue` times 100 less its rehearsal over 100.
/// Rehearsal counts only if it is for that hex. Returns the gain and the
/// rehearsal counted.
pub(crate) fn landing_fatigue_gain(
    figures: &GroundFigures,
    rehearsal: Fixed,
    rehearsal_hex: Option<Hex>,
    hex: Hex,
) -> (Fixed, Fixed) {
    let counted = if rehearsal_hex == Some(hex) {
        rehearsal
    } else {
        Fixed::ZERO
    };
    (
        figures
            .unprepared_landing_fatigue
            .mul_div(HUNDRED - counted, HUNDRED),
        counted,
    )
}

/// The preparation of two forces that join (decision 12 of 0095): the
/// receiving force's hex and order, the roster of both, and the lower of the
/// two loadings (the other force's counting only if it is for the same hex,
/// else 0). `None` if the receiving force has none. Nothing in the game joins
/// forces yet: this waits on the join order of step 3, stage 1.
#[allow(dead_code)]
pub(crate) fn joined(
    into: Option<&Preparation>,
    from: Option<&Preparation>,
) -> Option<Preparation> {
    let into = into?;
    let from_loading = match from {
        Some(from) if from.hex == into.hex => from.loading,
        _ => Fixed::ZERO,
    };
    let mut roster = into.roster.clone();
    if let Some(from) = from {
        roster.extend(from.roster.iter().copied());
    }
    Some(Preparation {
        hex: into.hex,
        roster,
        loading: into.loading.min(from_loading),
        order: into.order,
    })
}

/// A force's preparation when a ship joins it (decision 9 of 0095): its
/// loading times the share kept. Waits on the join order of step 3, stage 1.
#[allow(dead_code)]
pub(crate) fn ship_joined(preparation: &Preparation, kept: Fixed) -> Preparation {
    Preparation {
        loading: preparation.loading * kept,
        ..preparation.clone()
    }
}

impl World {
    /// Applies a prepare-landing order to `force` (decision 0095, 1 and 8 to
    /// 11), at intake. Returns the follow-up events, which the caller records
    /// after the order's acceptance, each citing the order. The same order
    /// again changes nothing.
    pub(crate) fn apply_prepare_landing(
        &mut self,
        force: TaskForceId,
        hex: Hex,
        roster: &BTreeSet<LandUnitId>,
        order: OrderId,
    ) -> Vec<EventKind> {
        let old = self.truth.task_forces[&force].preparation.clone();
        if old
            .as_ref()
            .is_some_and(|old| old.hex == hex && old.roster == *roster)
        {
            return Vec::new();
        }
        let kept = self
            .ref_data
            .ground
            .map_or(Fixed::ONE, |figures| figures.preparation_kept_on_change);
        let mut events = Vec::new();

        // A unit on another force's roster is taken from it, at no cost to
        // that force (decision 3c of 0099).
        for unit in roster {
            for (other, other_force) in &mut self.truth.task_forces {
                if *other == force {
                    continue;
                }
                if let Some(preparation) = other_force.preparation.as_mut() {
                    if preparation.roster.remove(unit) {
                        events.push(EventKind::LeftRoster {
                            unit: *unit,
                            task_force: *other,
                            reason: RosterLeft::TakenByOrder,
                        });
                    }
                }
            }
        }

        let mut loading = Fixed::ZERO;
        match &old {
            // Another hex, or no order before: a new landing place loses
            // everything (decision 8).
            Some(old) if old.hex != hex => {
                if old.loading > Fixed::ZERO {
                    events.push(EventKind::LoadingLost {
                        task_force: force,
                        hex: old.hex,
                        loading_before: old.loading,
                        loading: Fixed::ZERO,
                        reason: LoadingLoss::NewLandingPlace,
                    });
                }
                for unit in &old.roster {
                    let land_unit = self
                        .truth
                        .land_units
                        .get_mut(unit)
                        .expect("a rostered unit exists");
                    if land_unit.rehearsal_hex == Some(old.hex) {
                        let before = land_unit.rehearsal;
                        land_unit.rehearsal = Fixed::ZERO;
                        land_unit.rehearsal_hex = None;
                        events.push(EventKind::RehearsalLost {
                            unit: *unit,
                            hex: old.hex,
                            rehearsal_before: before,
                            reason: RehearsalLoss::NewLandingPlace,
                        });
                    }
                }
                for unit in old.roster.difference(roster) {
                    events.push(EventKind::LeftRoster {
                        unit: *unit,
                        task_force: force,
                        reason: RosterLeft::DroppedByOrder,
                    });
                }
            }
            // The same hex with another roster: a unit left out costs
            // nothing, and units added cost the loading a share, once
            // (decisions 10 and 3c of 0099).
            Some(old) => {
                for unit in old.roster.difference(roster) {
                    events.push(EventKind::LeftRoster {
                        unit: *unit,
                        task_force: force,
                        reason: RosterLeft::DroppedByOrder,
                    });
                }
                loading = old.loading;
                if roster.difference(&old.roster).next().is_some() && loading > Fixed::ZERO {
                    let after = loading * kept;
                    if after != loading {
                        events.push(EventKind::LoadingLost {
                            task_force: force,
                            hex,
                            loading_before: loading,
                            loading: after,
                            reason: LoadingLoss::UnitAdded,
                        });
                    }
                    loading = after;
                }
            }
            None => {}
        }

        self.truth
            .task_forces
            .get_mut(&force)
            .expect("the force exists")
            .preparation = Some(Preparation {
            hex,
            roster: roster.clone(),
            loading,
            order,
        });
        events
    }

    /// The breakdown of a `LoadingLost` for a unit added, when breakdowns are
    /// on: the loading before, the share kept and the loading after. `None`
    /// for any other event.
    pub(crate) fn preparation_breakdown(&self, event: &EventKind) -> Option<Breakdown> {
        let EventKind::LoadingLost {
            loading_before,
            loading,
            reason: LoadingLoss::UnitAdded,
            ..
        } = event
        else {
            return None;
        };
        if !self.config.record_breakdowns {
            return None;
        }
        let kept = self
            .ref_data
            .ground
            .map_or(Fixed::ONE, |figures| figures.preparation_kept_on_change);
        Some(Breakdown {
            factors: vec![
                Factor {
                    name: "loading_before",
                    value: *loading_before,
                },
                Factor {
                    name: "preparation_kept_on_change",
                    value: kept,
                },
            ],
            result: *loading,
        })
    }

    /// The `PreparationShared` event for a force just split off `parent`, if
    /// it was given a preparation (decision 12 of 0095). Both callers of a
    /// split record it after `ShipsDetached`, citing that event.
    pub(crate) fn preparation_shared(
        &self,
        parent: TaskForceId,
        new_force: TaskForceId,
    ) -> Option<EventKind> {
        let preparation = self.truth.task_forces[&new_force].preparation.as_ref()?;
        Some(EventKind::PreparationShared {
            parent,
            new_force,
            hex: preparation.hex,
            loading: preparation.loading,
            units: preparation.roster.iter().copied().collect(),
        })
    }

    /// The forces that qualify to gain loading today, and the loading they
    /// gain (decision 4 of 0095): with the order and a roster that is not
    /// empty, stopped at a base of their side in the centre of their hex,
    /// and not under way at any hour of the day. Returns the qualifying
    /// forces: the day's rehearsal reads them.
    pub(crate) fn gain_loading(
        &mut self,
        day: u32,
        figures: &GroundFigures,
    ) -> BTreeSet<TaskForceId> {
        let day_start = pulse_start(day, Pulse::Night);
        let qualifying: Vec<TaskForceId> = self
            .truth
            .task_forces
            .values()
            .filter(|force| {
                let Some(preparation) = &force.preparation else {
                    return false;
                };
                !preparation.roster.is_empty()
                    && force.yard_trip.is_none()
                    && matches!(
                        force.status,
                        TaskForceStatus::Idle | TaskForceStatus::Arrived | TaskForceStatus::Halted
                    )
                    && force.toward_hex.is_none()
                    && self
                        .truth
                        .bases
                        .values()
                        .any(|base| base.side == force.side && base.hex == force.from_hex)
                    && force
                        .arrived_at_hours
                        .is_none_or(|arrived| arrived < day_start)
                    && force.halt.is_none_or(|halt| halt.at_hours < day_start)
            })
            .map(|force| force.id)
            .collect();
        for id in &qualifying {
            let force = self
                .truth
                .task_forces
                .get_mut(id)
                .expect("the force exists");
            let preparation = force
                .preparation
                .as_mut()
                .expect("a qualifying force has a preparation");
            let before = preparation.loading;
            let after = (before + figures.preparation_per_day).min(HUNDRED);
            preparation.loading = after;
            let (hex, order) = (preparation.hex, preparation.order);
            if after != before {
                let breakdown = self.config.record_breakdowns.then(|| Breakdown {
                    factors: vec![
                        Factor {
                            name: "loading_before",
                            value: before,
                        },
                        Factor {
                            name: "preparation_per_day",
                            value: figures.preparation_per_day,
                        },
                    ],
                    result: after,
                });
                self.events.record(
                    day,
                    None,
                    EventKind::LoadingGained {
                        task_force: *id,
                        hex,
                        loading: after,
                    },
                    vec![Cause::Order(order)],
                    breakdown,
                );
            }
        }
        qualifying.into_iter().collect()
    }

    /// A day's rehearsal for a unit of the roster of the qualifying `force`
    /// (decision 4 of 0095). If the unit had rehearsed another hex, that is
    /// lost first.
    pub(crate) fn rehearse_day(
        &mut self,
        day: u32,
        unit_id: LandUnitId,
        force: TaskForceId,
        figures: &GroundFigures,
    ) {
        let preparation = self.truth.task_forces[&force]
            .preparation
            .clone()
            .expect("a qualifying force has a preparation");
        let unit = &self.truth.land_units[&unit_id];
        let mut causes = vec![Cause::Order(preparation.order)];
        causes.extend(unit.activity_order.map(Cause::Order));
        if let Some(other) = unit.rehearsal_hex.filter(|other| *other != preparation.hex) {
            let before = unit.rehearsal;
            let unit = self
                .truth
                .land_units
                .get_mut(&unit_id)
                .expect("the land unit exists");
            unit.rehearsal = Fixed::ZERO;
            unit.rehearsal_hex = None;
            self.events.record(
                day,
                None,
                EventKind::RehearsalLost {
                    unit: unit_id,
                    hex: other,
                    rehearsal_before: before,
                    reason: RehearsalLoss::OtherLandingPlace,
                },
                causes.clone(),
                None,
            );
        }
        let unit = self
            .truth
            .land_units
            .get_mut(&unit_id)
            .expect("the land unit exists");
        let before = unit.rehearsal;
        let after = (before + figures.preparation_per_day).min(HUNDRED);
        unit.rehearsal = after;
        unit.rehearsal_hex = (after > Fixed::ZERO).then_some(preparation.hex);
        if after != before {
            let breakdown = self.config.record_breakdowns.then(|| Breakdown {
                factors: vec![
                    Factor {
                        name: "rehearsal_before",
                        value: before,
                    },
                    Factor {
                        name: "preparation_per_day",
                        value: figures.preparation_per_day,
                    },
                ],
                result: after,
            });
            self.events.record(
                day,
                None,
                EventKind::RehearsalGained {
                    unit: unit_id,
                    task_force: force,
                    hex: preparation.hex,
                    rehearsal: after,
                },
                causes,
                breakdown,
            );
        }
    }

    /// What a unit's first men landing in `hex` does (decisions 6 and 7 of
    /// 0095): the unit gains the fatigue of a landing, less for its rehearsal
    /// of that hex; its rehearsal is spent; and it leaves its force's roster
    /// (the ruling of Nov 14, 2026: when its first men land from a ship, at
    /// any hex). Each change is recorded, citing the `MenLanded` event.
    pub(crate) fn first_men_landed(
        &mut self,
        day: u32,
        pulse: Pulse,
        hex: Hex,
        unit_id: LandUnitId,
        landed: EventId,
    ) {
        let Some(figures) = self.ref_data.ground else {
            return;
        };
        let causes = vec![Cause::Event(landed)];
        let unit = self
            .truth
            .land_units
            .get_mut(&unit_id)
            .expect("the land unit exists");
        let (gain, counted) =
            landing_fatigue_gain(&figures, unit.rehearsal, unit.rehearsal_hex, hex);
        let fatigue_before = unit.fatigue;
        unit.fatigue = (fatigue_before + gain).min(HUNDRED);
        let fatigue = unit.fatigue;
        let rehearsal_lost = unit
            .rehearsal_hex
            .take()
            .map(|rehearsed| (rehearsed, unit.rehearsal));
        unit.rehearsal = Fixed::ZERO;
        if fatigue != fatigue_before {
            let breakdown = self.config.record_breakdowns.then(|| Breakdown {
                factors: vec![
                    Factor {
                        name: "fatigue_before",
                        value: fatigue_before,
                    },
                    Factor {
                        name: "unprepared_landing_fatigue",
                        value: figures.unprepared_landing_fatigue,
                    },
                    Factor {
                        name: "rehearsal",
                        value: counted,
                    },
                ],
                result: fatigue,
            });
            self.events.record(
                day,
                Some(pulse),
                EventKind::LandUnitFatigue {
                    unit: unit_id,
                    change: LandFatigueChange::Landed,
                    fatigue,
                },
                causes.clone(),
                breakdown,
            );
        }
        if let Some((rehearsed, before)) = rehearsal_lost {
            self.events.record(
                day,
                Some(pulse),
                EventKind::RehearsalLost {
                    unit: unit_id,
                    hex: rehearsed,
                    rehearsal_before: before,
                    reason: RehearsalLoss::Landed,
                },
                causes.clone(),
                None,
            );
        }
        let rostered = self.truth.task_forces.iter_mut().find_map(|(id, force)| {
            let preparation = force.preparation.as_mut()?;
            preparation.roster.remove(&unit_id).then_some(*id)
        });
        if let Some(task_force) = rostered {
            self.events.record(
                day,
                Some(pulse),
                EventKind::LeftRoster {
                    unit: unit_id,
                    task_force,
                    reason: RosterLeft::Landed,
                },
                causes,
                None,
            );
        }
    }

    /// For each unit on the roster of a force in `forces`, the force it is
    /// rostered with.
    pub(crate) fn rostered_with(
        &self,
        forces: &BTreeSet<TaskForceId>,
    ) -> BTreeMap<LandUnitId, TaskForceId> {
        let mut rostered = BTreeMap::new();
        for id in forces {
            if let Some(preparation) = &self.truth.task_forces[id].preparation {
                for unit in &preparation.roster {
                    rostered.insert(*unit, *id);
                }
            }
        }
        rostered
    }
}
