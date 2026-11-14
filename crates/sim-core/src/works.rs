//! Units that work, and what they build
//! (`docs/decisions/0099-units-that-work-and-what-they-build.md`).
//!
//! A unit whose day is work (decision 3b) makes work in the ground combat
//! phase, after the fights, when its day is settled: its men ashore, times
//! its fed share, times `work_per_man_day`, times its works skill over 2,
//! and for a fighting unit set to work times `fighting_unit_work_share`
//! (decisions 3 and 3a). The work goes to its job at its side's
//! lowest-numbered base in its hex (decision 4): the airfield's work done,
//! the day's work on the port or on the aircraft, which the next day reads,
//! or the base's defences. A unit with no job named builds the airfield if
//! the base has none or its field is damaged, and else works the port. Work
//! on the airfield of a base that has one mends its damage, by the work over
//! `work_per_airfield_damage_point`, to 0 at most (decision 0103, 7). Once every unit has worked,
//! each base whose work done has reached `airfield_work` has an airfield
//! (decision 5): the day's work is added whole on the day it is built, and
//! nothing is added after (ruling 2).
//!
//! The port's work speeds the next day's cargo at the base (`cargo.rs`), the
//! aircraft's work the next night's repairs (`upkeep.rs`), and defences the
//! defender's chance to hit (`ground_combat.rs`).
//!
//! Reads: a land unit's mark, activity, men ashore, hex, fed share and works
//! skill and its activity order; the bases of its side in its hex, their
//! airfield and work done; the night's feeding event; the ground figures.
//! Writes: a base's work done, airfield, defences, airfield damage and the
//! day's work; the events `UnitWorked`, `AirfieldMended` and `AirfieldBuilt`.
//! Draws no random number.

use std::collections::BTreeMap;

use crate::events::{Breakdown, Cause, EventKind, Factor};
use crate::fixed::Fixed;
use crate::ids::{BaseId, EventId, LandUnitId};
use crate::orders::Job;
use crate::refdata::GroundFigures;
use crate::state::{Base, DayWork, Skills};
use crate::world::World;

/// The job a unit with no job named works at, at `base` (decision 4 of
/// 0099, decision 8 of 0103): the airfield if the base has none or its field
/// is damaged, and else the port.
pub fn default_job(base: &Base) -> Job {
    if base.airfield && base.airfield_damage <= Fixed::ZERO {
        Job::Port
    } else {
        Job::Airfield
    }
}

/// The work a unit makes in a day (decisions 3 and 3a of 0099): its men
/// ashore, times its fed share, times `work_per_man_day`, times its works
/// skill over 2, and for a fighting unit times `fighting_unit_work_share`.
pub fn unit_work(
    men_ashore: u32,
    fed_share: Fixed,
    works: u8,
    working: bool,
    figures: &GroundFigures,
) -> Fixed {
    let made = (Fixed::from_int(i64::from(men_ashore)) * fed_share * figures.work_per_man_day)
        .mul_div(
            Fixed::from_int(i64::from(works)),
            Fixed::from_int(i64::from(Skills::STANDARD)),
        );
    if working {
        made
    } else {
        made * figures.fighting_unit_work_share
    }
}

/// The factor a day's work on a port gives the next day's cargo rate at the
/// base (decision 4 of 0099): 1 plus the work over
/// `work_to_double_unloading`, and no more than 2.
pub fn port_factor(work: Fixed, figures: &GroundFigures) -> Fixed {
    (Fixed::ONE + work / figures.work_to_double_unloading).min(Fixed::from_int(2))
}

/// The aircraft more a base repairs the night after a day's work on the
/// aircraft (decision 4 of 0099): one for each `work_per_aircraft_repair`,
/// cut to a whole aircraft.
pub fn extra_repairs(work: Fixed, figures: &GroundFigures) -> u32 {
    u32::try_from((work / figures.work_per_aircraft_repair).trunc().max(0)).unwrap_or(u32::MAX)
}

impl World {
    /// A day of work for `unit_id`, whose day is work at `job` (`None` for
    /// the default job). Makes nothing if it has no man ashore or its side
    /// has no base in its hex; records nothing if it makes no work. The
    /// event of each day's work on an airfield is added to `on_airfields`
    /// by base, for [`World::finish_airfields`].
    pub(crate) fn work_day(
        &mut self,
        day: u32,
        unit_id: LandUnitId,
        job: Option<Job>,
        figures: &GroundFigures,
        on_airfields: &mut BTreeMap<BaseId, Vec<EventId>>,
    ) {
        let unit = &self.truth.land_units[&unit_id];
        let Some(hex) = unit.hex.filter(|_| unit.men_ashore > 0) else {
            return;
        };
        let side = unit.side;
        let Some(base) = self
            .truth
            .bases
            .values()
            .find(|base| base.side == side && base.hex == hex)
        else {
            return;
        };
        let base_id = base.id;
        let job = job.unwrap_or_else(|| default_job(base));
        let (men, fed_share, works, working) = (
            unit.men_ashore,
            unit.fed_share,
            unit.skills.works,
            unit.working,
        );
        let work = unit_work(men, fed_share, works, working, figures);
        if work <= Fixed::ZERO {
            return;
        }
        let mut causes: Vec<Cause> = unit.activity_order.map(Cause::Order).into_iter().collect();
        let fed = self.events.latest_where(|kind| {
            matches!(kind, EventKind::MenFed { side: owner, hex: at_hex, .. }
                if *owner == side && *at_hex == hex)
        });
        causes.extend(
            fed.filter(|event| {
                self.events
                    .get(*event)
                    .is_some_and(|event| event.day == day)
            })
            .map(Cause::Event),
        );

        let base = self.truth.bases.get_mut(&base_id).expect("the base exists");
        let refresh = |base: &mut Base| {
            if base.day_work.is_none_or(|work| work.day != day) {
                base.day_work = Some(DayWork {
                    day,
                    port: Fixed::ZERO,
                    aircraft: Fixed::ZERO,
                });
            }
        };
        // The damage a day's work on a built airfield mends (decision 7 of
        // 0103): the points, and the damage before.
        let mut mended: Option<(Fixed, Fixed)> = None;
        let total = match job {
            // Nothing is added once the airfield is built (ruling 2), but
            // its damage is mended.
            Job::Airfield => {
                if !base.airfield {
                    base.airfield_work += work;
                } else if base.airfield_damage > Fixed::ZERO {
                    let before = base.airfield_damage;
                    let points = (work / figures.work_per_airfield_damage_point).min(before);
                    base.airfield_damage = before - points;
                    mended = Some((points, before));
                }
                base.airfield_work
            }
            Job::Port => {
                refresh(base);
                let day_work = base.day_work.as_mut().expect("refreshed");
                day_work.port += work;
                day_work.port
            }
            Job::Aircraft => {
                refresh(base);
                let day_work = base.day_work.as_mut().expect("refreshed");
                day_work.aircraft += work;
                day_work.aircraft
            }
            // Not cut to a whole number (ruling 3), and no more than 100.
            Job::Defences => {
                base.defences = (base.defences + work / figures.work_per_defence_point)
                    .min(Fixed::from_int(100));
                base.defences
            }
        };
        let breakdown = self.config.record_breakdowns.then(|| {
            let mut factors = vec![
                Factor {
                    name: "men_ashore",
                    value: Fixed::from_int(i64::from(men)),
                },
                Factor {
                    name: "fed_share",
                    value: fed_share,
                },
                Factor {
                    name: "work_per_man_day",
                    value: figures.work_per_man_day,
                },
                Factor {
                    name: "works_skill",
                    value: Fixed::from_int(i64::from(works)),
                },
            ];
            if !working {
                factors.push(Factor {
                    name: "fighting_unit_work_share",
                    value: figures.fighting_unit_work_share,
                });
            }
            if job == Job::Defences {
                factors.push(Factor {
                    name: "work_per_defence_point",
                    value: figures.work_per_defence_point,
                });
            }
            Breakdown {
                factors,
                result: work,
            }
        });
        let event = self.events.record(
            day,
            None,
            EventKind::UnitWorked {
                unit: unit_id,
                base: base_id,
                job,
                work,
                total,
            },
            causes,
            breakdown,
        );
        if let Some((points, before)) = mended {
            let damage = self.truth.bases[&base_id].airfield_damage;
            let breakdown = self.config.record_breakdowns.then(|| Breakdown {
                factors: vec![
                    Factor {
                        name: "work",
                        value: work,
                    },
                    Factor {
                        name: "work_per_airfield_damage_point",
                        value: figures.work_per_airfield_damage_point,
                    },
                    Factor {
                        name: "damage_before",
                        value: before,
                    },
                ],
                result: points,
            });
            self.events.record(
                day,
                None,
                EventKind::AirfieldMended {
                    unit: unit_id,
                    base: base_id,
                    points,
                    damage,
                },
                vec![Cause::Event(event)],
                breakdown,
            );
        }
        if job == Job::Airfield {
            on_airfields.entry(base_id).or_default().push(event);
        }
    }

    /// After every unit has worked: each base worked on today whose work
    /// done on its airfield has reached `airfield_work` has an airfield
    /// (decision 5 of 0099).
    pub(crate) fn finish_airfields(
        &mut self,
        day: u32,
        on_airfields: &BTreeMap<BaseId, Vec<EventId>>,
        figures: &GroundFigures,
    ) {
        for (base_id, events) in on_airfields {
            let base = self.truth.bases.get_mut(base_id).expect("the base exists");
            if base.airfield || base.airfield_work < figures.airfield_work {
                continue;
            }
            base.airfield = true;
            let (side, hex, work_done) = (base.side, base.hex, base.airfield_work);
            let breakdown = self.config.record_breakdowns.then(|| Breakdown {
                factors: vec![
                    Factor {
                        name: "work_done",
                        value: work_done,
                    },
                    Factor {
                        name: "airfield_work",
                        value: figures.airfield_work,
                    },
                ],
                result: work_done,
            });
            self.events.record(
                day,
                None,
                EventKind::AirfieldBuilt {
                    base: *base_id,
                    side,
                    hex,
                    work_done,
                },
                events.iter().map(|event| Cause::Event(*event)).collect(),
                breakdown,
            );
        }
    }

    /// The events of `day` whose kind `keep` accepts, in order: for the
    /// causes of what the next day's work does.
    pub(crate) fn events_of_day_where(
        &self,
        day: u32,
        keep: impl Fn(&EventKind) -> bool,
    ) -> Vec<Cause> {
        let mut found: Vec<Cause> = self
            .events
            .from_index(0)
            .iter()
            .rev()
            .skip_while(|event| event.day > day)
            .take_while(|event| event.day == day)
            .filter(|event| keep(&event.kind))
            .map(|event| Cause::Event(event.id))
            .collect();
        found.reverse();
        found
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fx(text: &str) -> Fixed {
        text.parse().expect("a decimal")
    }

    #[test]
    fn work_is_men_times_fed_share_times_skill_over_two_and_half_for_a_fighting_unit() {
        let figures = GroundFigures::FIRST;
        assert_eq!(unit_work(1000, Fixed::ONE, 2, true, &figures), fx("1000"));
        assert_eq!(unit_work(400, Fixed::ONE, 4, true, &figures), fx("800"));
        assert_eq!(unit_work(900, Fixed::ONE, 2, false, &figures), fx("450"));
        assert_eq!(unit_work(585, fx("0.5"), 2, true, &figures), fx("292.5"));
        assert_eq!(unit_work(556, Fixed::ZERO, 2, true, &figures), Fixed::ZERO);
    }

    #[test]
    fn the_port_factor_is_one_plus_the_work_over_the_figure_and_at_most_two() {
        let figures = GroundFigures::FIRST;
        assert_eq!(port_factor(Fixed::ZERO, &figures), Fixed::ONE);
        assert_eq!(port_factor(fx("500"), &figures), fx("1.5"));
        assert_eq!(port_factor(fx("2000"), &figures), fx("2"));
    }

    #[test]
    fn extra_repairs_are_cut_to_a_whole_aircraft() {
        let figures = GroundFigures::FIRST;
        assert_eq!(extra_repairs(fx("600"), &figures), 3);
        assert_eq!(extra_repairs(fx("292.5"), &figures), 1);
        assert_eq!(extra_repairs(fx("199"), &figures), 0);
    }
}
