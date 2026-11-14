//! What a side knows of the enemy ashore: the daily ground report and a
//! scenario's opening estimate
//! (`docs/decisions/0092-shells-bombs-and-the-enemy-ashore.md`, decisions 11
//! to 16).
//!
//! A ground report is made at the end of each hex's part of the ground
//! combat phase, after the day's fight, its losses and any base that changed
//! side, in each hex where both sides still have men ashore: first the
//! Allied side's report of the Japanese men, then the Japanese side's of the
//! Allied. It is the true count times an error drawn evenly between
//! `report_error_low` and `report_error_high`, cut to a whole man, and it
//! goes into belief through [`BeliefState::apply_report`]. It then puts on
//! the reporting side's map every enemy base in the hex not on it.
//!
//! Draws: one stream for each reporting side, hex and day, keyed on the
//! day, no pulse, the intelligence subsystem, `Draw::GroundReport` and the
//! reporting side's lowest-numbered unit in the hex: one draw, the error.
//!
//! [`BeliefState::apply_report`]: crate::belief::BeliefState

use crate::belief::{ApplyOutcome, GroundReport, Report};
use crate::events::{Breakdown, Cause, EventKind, Factor};
use crate::fixed::Fixed;
use crate::hex::Hex;
use crate::ids::{BaseId, EntityRef, EventId, LandUnitId};
use crate::refdata::ReportError;
use crate::rng::{Draw, Subsystem};
use crate::state::Side;
use crate::world::{SimError, World};

/// An estimate of the enemy's men ashore that a side holds when the game
/// begins (decision 16 of 0092): what that side believed, right or wrong.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct StartingEstimate {
    pub side: Side,
    pub hex: Hex,
    pub men: u32,
}

/// The report a true count gives with an error drawn as `draw` from a
/// stream's `below(span + 1)`, where the span is the high bound less the
/// low in millionths: the low bound plus the draw, times the truth, cut to
/// a whole man. Returns the report and the error.
pub(crate) fn reported_men(true_men: u32, error: ReportError, draw: u64) -> (u32, Fixed) {
    let factor =
        Fixed::from_raw(error.low.raw() + i64::try_from(draw).expect("a draw within the span"));
    let men = (Fixed::from_int(i64::from(true_men)) * factor)
        .trunc()
        .max(0);
    (
        u32::try_from(men).expect("a report is at most the high share of a count of men"),
        factor,
    )
}

/// The span of the error in millionths, which `below(span + 1)` draws in.
pub(crate) fn error_span(error: ReportError) -> u64 {
    u64::try_from(error.high.raw() - error.low.raw()).expect("the low is not above the high")
}

impl World {
    /// The ground reports of one hex (decisions 13 to 15). `fight` is the
    /// day's fight there, if there was one: the report cites it.
    pub(crate) fn report_ashore(&mut self, day: u32, hex: Hex, fight: Option<EventId>) {
        let Some(error) = self.ref_data.ground.and_then(|ground| ground.report_error) else {
            return;
        };
        let ashore = |side: Side| -> Vec<(LandUnitId, u32)> {
            self.truth
                .land_units
                .values()
                .filter(|unit| unit.side == side && unit.hex == Some(hex) && unit.men_ashore > 0)
                .map(|unit| (unit.id, unit.men_ashore))
                .collect()
        };
        let units = [ashore(Side::Allied), ashore(Side::Japanese)];
        if units.iter().any(Vec::is_empty) {
            return;
        }
        let search = self
            .ref_data
            .search
            .clone()
            .expect("World::new requires search figures with the ground report figures");
        for (index, side) in Side::ALL.into_iter().enumerate() {
            let reporter = units[index][0].0;
            let true_men: u32 = units[1 - index].iter().map(|(_, men)| men).sum();
            let mut stream = self.rng(
                day,
                None,
                Subsystem::Intelligence,
                Draw::GroundReport,
                EntityRef::LandUnit(reporter),
            );
            let draw = stream.below(error_span(error) + 1);
            let (men, factor) = reported_men(true_men, error, draw);
            let report = GroundReport {
                hex,
                men,
                day,
                received_event: self.events.next_id(),
            };
            let ApplyOutcome::Estimate { replaced } = self
                .beliefs
                .get_mut(&side)
                .expect("both sides have a belief state")
                .apply_report(&Report::Ground(report), &search)
            else {
                unreachable!("a ground report makes or replaces an estimate");
            };
            // The truth and the error drawn are the umpire's: a side is
            // given this event with no breakdown (decision 19).
            let breakdown = self.config.record_breakdowns.then(|| Breakdown {
                factors: vec![
                    Factor {
                        name: "true_men",
                        value: Fixed::from_int(i64::from(true_men)),
                    },
                    Factor {
                        name: "report_error_low",
                        value: error.low,
                    },
                    Factor {
                        name: "report_error_high",
                        value: error.high,
                    },
                    Factor {
                        name: "error",
                        value: factor,
                    },
                ],
                result: Fixed::from_int(i64::from(men)),
            });
            let received = self.events.record(
                day,
                None,
                EventKind::GroundReportReceived {
                    side,
                    hex,
                    men,
                    replaced,
                },
                fight.map(Cause::Event).into_iter().collect(),
                breakdown,
            );
            assert_eq!(
                received, report.received_event,
                "the report cites the event that records it"
            );

            // The enemy's bases in the hex are found (decision 15).
            let hidden: Vec<BaseId> = self
                .truth
                .bases
                .values()
                .filter(|base| base.hex == hex && base.side != side && !base.on_map)
                .map(|base| base.id)
                .collect();
            for base in hidden {
                self.truth
                    .bases
                    .get_mut(&base)
                    .expect("the base exists")
                    .on_map = true;
                self.events.record(
                    day,
                    None,
                    EventKind::BaseFound { side, base, hex },
                    vec![Cause::Event(received)],
                    None,
                );
            }
        }
    }

    /// Gives sides opening estimates of the enemy's men ashore (decision 16
    /// of 0092). Each goes in through [`BeliefState::apply_report`] on day
    /// 0, with no error and no cause, as a starting contact does. Takes
    /// effect completely or, on an error, not at all.
    ///
    /// Fails if a day has been resolved, if there are no search figures
    /// (belief is written with them), or if two entries are for the same
    /// side and hex: a side holds one estimate for a hex. An error names the
    /// entry.
    ///
    /// [`BeliefState::apply_report`]: crate::belief::BeliefState
    pub fn hold_starting_estimates(
        &mut self,
        estimates: Vec<StartingEstimate>,
    ) -> Result<(), SimError> {
        if self.truth.day > 0 {
            return Err(SimError::InvalidSetup(
                "opening estimates can only be given before day 1".to_string(),
            ));
        }
        if estimates.is_empty() {
            return Ok(());
        }
        let Some(search) = self.ref_data.search.clone() else {
            return Err(SimError::InvalidSetup(
                "there are opening estimates but no search figures".to_string(),
            ));
        };
        let mut staged = self.clone();
        for estimate in &estimates {
            let describe = || {
                format!(
                    "the {} opening estimate of {} men at {}",
                    estimate.side.name(),
                    estimate.men,
                    estimate.hex
                )
            };
            if staged.map.get(estimate.hex).is_none() {
                return Err(SimError::InvalidSetup(format!(
                    "{} is off the map",
                    describe()
                )));
            }
            let report = GroundReport {
                hex: estimate.hex,
                men: estimate.men,
                day: 0,
                received_event: staged.events.next_id(),
            };
            let ApplyOutcome::Estimate { replaced } = staged
                .beliefs
                .get_mut(&estimate.side)
                .expect("both sides have a belief state")
                .apply_report(&Report::Ground(report), &search)
            else {
                unreachable!("a ground report makes or replaces an estimate");
            };
            if replaced {
                return Err(SimError::InvalidSetup(format!(
                    "{} is a second estimate for that side and hex",
                    describe()
                )));
            }
            let received = staged.events.record(
                0,
                None,
                EventKind::GroundReportReceived {
                    side: estimate.side,
                    hex: estimate.hex,
                    men: estimate.men,
                    replaced: false,
                },
                Vec::new(),
                None,
            );
            assert_eq!(
                received, report.received_event,
                "the report cites the event that records it"
            );
        }
        *self = staged;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fx(text: &str) -> Fixed {
        text.parse().expect("a decimal")
    }

    fn error() -> ReportError {
        ReportError {
            low: fx("0.5"),
            high: fx("1.5"),
        }
    }

    #[test]
    fn a_report_is_the_truth_times_the_error_cut_to_a_whole_man() {
        assert_eq!(error_span(error()), 1_000_000);
        // The least draw gives the low share, the most the high share.
        assert_eq!(reported_men(2840, error(), 0), (1420, fx("0.5")));
        assert_eq!(reported_men(2840, error(), 1_000_000), (4260, fx("1.5")));
        assert_eq!(reported_men(3, error(), 0), (1, fx("0.5")));
        assert_eq!(reported_men(7, error(), 300_000), (5, fx("0.8")));
        assert_eq!(reported_men(0, error(), 500_000).0, 0);
    }

    #[test]
    fn a_report_always_lies_between_the_low_and_high_share_of_the_truth() {
        for truth in [1u32, 3, 7, 100, 799, 2840, 3000] {
            for draw in (0..=1_000_000).step_by(37_037) {
                let (men, _) = reported_men(truth, error(), draw);
                let low = (Fixed::from_int(i64::from(truth)) * fx("0.5")).trunc();
                let high = (Fixed::from_int(i64::from(truth)) * fx("1.5")).trunc();
                assert!(i64::from(men) >= low && i64::from(men) <= high);
            }
        }
    }

    // ---- By hand-built worlds: the opening estimate and the daily report.

    use crate::events::Event;
    use crate::map::{HexData, Map};
    use crate::refdata::{GroundFigures, PortSize, RefData, SearchFigures};
    use crate::state::{Base, LandUnit, TruthState};

    const AT: Hex = Hex::new(0, 0);
    const OTHER: Hex = Hex::new(4, 0);

    fn world(
        search: bool,
        units: &[(u32, Side, u32, Hex)],
        bases: &[(u32, Side, Hex, bool)],
    ) -> World {
        let mut map = Map::new();
        for q in -1..=9 {
            for r in -1..=1 {
                map.insert(Hex::new(q, r), HexData::open_sea());
            }
        }
        let mut ref_data = RefData::default();
        if search {
            ref_data.search = Some(SearchFigures::from_search_toml());
        }
        if search {
            ref_data.ground = Some(GroundFigures::FIRST);
        }
        ref_data.port_sizes.insert(
            0,
            PortSize {
                berths: 0,
                berth_rate_t_per_h: Fixed::ZERO,
                lighter_rate_t_per_h: Fixed::from_int(20),
            },
        );
        let mut truth = TruthState::default();
        for (id, side, hex, on_map) in bases {
            truth.bases.insert(
                BaseId::new(*id),
                Base {
                    on_map: *on_map,
                    ..Base::new(BaseId::new(*id), format!("Base {id}"), *side, *hex)
                },
            );
        }
        for (id, side, men, hex) in units {
            truth.land_units.insert(
                LandUnitId::new(*id),
                LandUnit::new(
                    LandUnitId::new(*id),
                    format!("Unit {id}"),
                    *side,
                    *men,
                    Some(*hex),
                ),
            );
        }
        World::new(7, ref_data, map, truth).expect("valid setup")
    }

    fn estimate(side: Side, hex: Hex, men: u32) -> StartingEstimate {
        StartingEstimate { side, hex, men }
    }

    fn log(world: &World) -> Vec<Event> {
        world.events().iter().cloned().collect()
    }

    #[test]
    fn opening_estimates_are_held_on_day_0_through_belief() {
        let mut world = world(true, &[], &[]);
        world
            .hold_starting_estimates(vec![
                estimate(Side::Japanese, AT, 300),
                estimate(Side::Allied, AT, 0),
                estimate(Side::Japanese, OTHER, 1),
            ])
            .expect("held");
        let held = world.side_view(Side::Japanese);
        assert_eq!(
            held.ground_estimate(AT).map(|e| (e.men, e.day)),
            Some((300, 0))
        );
        assert_eq!(
            held.ground_estimate(OTHER).map(|e| (e.men, e.day)),
            Some((1, 0))
        );
        assert_eq!(
            world
                .side_view(Side::Allied)
                .ground_estimate(AT)
                .map(|e| e.men),
            Some(0)
        );
        // One event each, on day 0, with no cause and no breakdown.
        let events = log(&world);
        assert_eq!(events.len(), 3);
        assert!(events
            .iter()
            .all(|e| e.day == 0 && e.causes.is_empty() && e.breakdown.is_none()));
        // None given is nothing, even with no search figures.
        let mut bare = super::tests::world(false, &[], &[]);
        bare.hold_starting_estimates(Vec::new())
            .expect("nothing to hold");
        assert!(bare
            .hold_starting_estimates(vec![estimate(Side::Allied, AT, 5)])
            .expect_err("needs search figures")
            .to_string()
            .contains("search figures"));
    }

    #[test]
    fn a_bad_opening_estimate_changes_nothing() {
        let mut world = world(true, &[], &[]);
        let hash = world.state_hash();
        let events = log(&world);
        let refused = |world: &mut World, estimates: Vec<StartingEstimate>| -> String {
            world
                .hold_starting_estimates(estimates)
                .expect_err("refused")
                .to_string()
        };
        // Off the map, after a good one: the good one is not kept.
        let message = refused(
            &mut world,
            vec![
                estimate(Side::Japanese, AT, 300),
                estimate(Side::Japanese, Hex::new(50, 0), 5),
            ],
        );
        assert!(message.contains("off the map"), "{message}");
        // Two for the same side and hex.
        let message = refused(
            &mut world,
            vec![
                estimate(Side::Japanese, AT, 300),
                estimate(Side::Japanese, AT, 400),
            ],
        );
        assert!(message.contains("second estimate"), "{message}");
        assert_eq!(world.state_hash(), hash);
        assert_eq!(log(&world), events);
        assert_eq!(
            world.side_view(Side::Japanese).ground_estimates().count(),
            0
        );
        // After a day has been resolved.
        world.truth.day = 1;
        let message = refused(&mut world, vec![estimate(Side::Allied, AT, 5)]);
        assert!(message.contains("before day 1"), "{message}");
    }

    #[test]
    fn a_ground_report_is_made_where_both_sides_have_men_and_finds_the_enemys_bases() {
        // Allied unit 1 (1000 men) and Japanese unit 2 (400) at AT; the
        // Japanese base there is off the Allied map; Allied unit 3 alone at
        // OTHER.
        let mut world = world(
            true,
            &[
                (1, Side::Allied, 1000, AT),
                (2, Side::Japanese, 400, AT),
                (3, Side::Allied, 50, OTHER),
            ],
            &[(1, Side::Japanese, AT, false), (2, Side::Allied, AT, true)],
        );
        world.report_ashore(1, OTHER, None);
        assert!(log(&world).is_empty(), "one side alone: no report");
        world.report_ashore(1, AT, None);
        let events = log(&world);
        let reports: Vec<(Side, u32)> = events
            .iter()
            .filter_map(|e| match &e.kind {
                EventKind::GroundReportReceived {
                    side,
                    men,
                    replaced,
                    ..
                } => {
                    assert!(!replaced);
                    Some((*side, *men))
                }
                _ => None,
            })
            .collect();
        // The Allied side reports first, of the Japanese 400; then the
        // Japanese side, of the Allied 1000. Each is between half and one
        // and a half times the truth.
        assert_eq!(
            reports.iter().map(|r| r.0).collect::<Vec<_>>(),
            [Side::Allied, Side::Japanese]
        );
        assert!((200..=600).contains(&reports[0].1), "{reports:?}");
        assert!((500..=1500).contains(&reports[1].1), "{reports:?}");
        // The Allied report found the Japanese base; the Japanese side found
        // nothing (the Allied base was on its map).
        let found: Vec<(Side, u32)> = events
            .iter()
            .filter_map(|e| match &e.kind {
                EventKind::BaseFound { side, base, .. } => Some((*side, base.raw())),
                _ => None,
            })
            .collect();
        assert_eq!(found, vec![(Side::Allied, 1)]);
        assert!(world.truth.bases[&BaseId::new(1)].on_map);
        // A report is replaced the next day.
        world.report_ashore(2, AT, None);
        let replaced = log(&world)
            .iter()
            .filter(|e| {
                matches!(
                    e.kind,
                    EventKind::GroundReportReceived { replaced: true, .. }
                )
            })
            .count();
        assert_eq!(replaced, 2);
        assert_eq!(
            world
                .side_view(Side::Allied)
                .ground_estimate(AT)
                .map(|e| e.day),
            Some(2)
        );
        // Found once: no second event for the base.
        assert_eq!(
            log(&world)
                .iter()
                .filter(|e| matches!(e.kind, EventKind::BaseFound { .. }))
                .count(),
            1
        );
    }

    #[test]
    fn no_ground_report_is_made_without_the_report_error_figures() {
        let mut world = world(
            true,
            &[(1, Side::Allied, 1000, AT), (2, Side::Japanese, 400, AT)],
            &[],
        );
        world.ref_data.ground.as_mut().unwrap().report_error = None;
        world.report_ashore(1, AT, None);
        assert!(log(&world).is_empty());
        assert_eq!(world.side_view(Side::Allied).ground_estimates().count(), 0);
        // A unit with no man ashore does not count as a side being there.
        let mut world = super::tests::world(
            true,
            &[(1, Side::Allied, 1000, AT), (2, Side::Japanese, 400, AT)],
            &[],
        );
        world
            .truth
            .land_units
            .get_mut(&LandUnitId::new(2))
            .unwrap()
            .men_ashore = 0;
        world.report_ashore(1, AT, None);
        assert!(log(&world).is_empty());
    }
}
