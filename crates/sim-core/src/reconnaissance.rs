//! Reconnaissance: an air group on the reconnaissance mission flies to a hex
//! once a day and reports the enemy's men there
//! (`docs/decisions/0147-reconnaissance-for-an-operation.md`, items 4 to 12).
//!
//! The flight is made in the search phase of the morning pulse, by each air
//! group whose mission is [`AirMission::Reconnaissance`], in air group order,
//! after the ordinary searches of the pulse. One aircraft flies. The group
//! must have an aircraft able to fly by the rules of a search (a base or a
//! deck to fly from, supply at the base, the fatigue and the flight deck),
//! and the hex must lie within the search range of its type. Its cost is
//! that of one search sortie of twice the distance: the flight is noted as
//! one sortie of the search kind, so the wear, fatigue and supply steps of
//! the pulse treat it as they treat one. It is not flown to a hex where its
//! side has men of a fighting unit ashore (item 6), and it is not made at
//! all unless `ground.toml` gives both `reconnaissance_error_low` and
//! `reconnaissance_error_high` (item 7).
//!
//! The report is the true count of the enemy's men ashore in the hex times
//! an error drawn as a whole number of millionths between the two figures,
//! cut to a whole man. It goes into belief through
//! [`BeliefState::apply_report`] at the grade seen from the air, and puts on
//! the side's map every enemy base in the hex not on it. Nothing meets the
//! flight (item 12).
//!
//! Draws: one stream for each air group, keyed on the day, the morning
//! pulse, the intelligence subsystem, [`Draw::Reconnaissance`] and the air
//! group: one draw, the error.
//!
//! [`BeliefState::apply_report`]: crate::belief::BeliefState

use crate::belief::{ApplyOutcome, GroundGrade, GroundReport, Report};
use crate::estimate::{error_span, reported_men};
use crate::events::{Breakdown, Cause, EventKind, Factor, SortieKind};
use crate::fixed::Fixed;
use crate::ids::EntityRef;
use crate::orders::AirMission;
use crate::pictures::Holders;
use crate::rng::{Draw, Subsystem};
use crate::state::AirGroup;
use crate::time::{self, Pulse};
use crate::upkeep::Sortie;
use crate::world::World;

impl World {
    /// The reconnaissance flights of one pulse: made in the morning only.
    pub(crate) fn reconnoitre(&mut self, day: u32, pulse: Pulse) {
        if pulse != Pulse::Morning {
            return;
        }
        let Some(error) = self
            .ref_data
            .ground
            .and_then(|ground| ground.reconnaissance_error)
        else {
            return;
        };
        let Some(search) = self.ref_data.search.clone() else {
            return;
        };
        let groups: Vec<(AirGroup, crate::hex::Hex)> = self
            .truth
            .air_groups
            .values()
            .filter_map(|group| match group.mission {
                AirMission::Reconnaissance(hex) => Some((group.clone(), hex)),
                _ => None,
            })
            .collect();
        for (group, hex) in groups {
            if group.ready == 0 {
                continue;
            }
            // The hex it flies from is its base's hex or its ship's position
            // at the start of the pulse (decision 4 of 0025).
            let Some(origin) = self.truth.air_group_hex(&group) else {
                continue;
            };
            if time::is_dark(origin, pulse) {
                continue;
            }
            // The base and deck rules of a search (decision 14 of 0026).
            if self.aircraft_able_to_fly(&group) == 0 {
                continue;
            }
            let range_nm = self.ref_data.aircraft_types[&group.aircraft_type].search_range_nm;
            let distance_nm = Fixed::from_int(i64::from(origin.air_distance_nm(hex)));
            if distance_nm > range_nm {
                continue;
            }
            // Item 6: the side's own men of a fighting unit there see more.
            let own_fighting_men = self.truth.land_units.values().any(|unit| {
                unit.side == group.side
                    && unit.hex == Some(hex)
                    && unit.men_ashore > 0
                    && !unit.working
            });
            if own_fighting_men {
                continue;
            }

            let supply_t = self.ref_data.air_supply.map_or(Fixed::ZERO, |figures| {
                figures.sortie_cost(distance_nm * Fixed::from_int(2))
            });
            let flown = self.events.record(
                day,
                Some(pulse),
                EventKind::ReconnaissanceFlown {
                    air_group: group.id,
                    from: group.location,
                    hex,
                    distance_nm,
                    supply_t,
                },
                group.mission_order.map(Cause::Order).into_iter().collect(),
                None,
            );
            // One aircraft, out and back: the wear, fatigue and supply steps
            // take it as one search sortie of twice the distance.
            if self.notes_sorties() {
                self.sorties.push(Sortie {
                    air_group: group.id,
                    kind: SortieKind::Search,
                    aircraft: 1,
                    sent: 1,
                    distance_nm: distance_nm * Fixed::from_int(2),
                    causes: vec![Cause::Event(flown)],
                });
            }

            let true_men: u32 = self
                .truth
                .land_units
                .values()
                .filter(|unit| unit.side != group.side && unit.hex == Some(hex))
                .map(|unit| unit.men_ashore)
                .sum();
            let mut stream = self.rng(
                day,
                Some(pulse),
                Subsystem::Intelligence,
                Draw::Reconnaissance,
                EntityRef::AirGroup(group.id),
            );
            let draw = stream.below(error_span(error) + 1);
            let (men, factor) = reported_men(true_men, error, draw);
            let report = GroundReport {
                hex,
                men,
                grade: GroundGrade::SeenFromAir,
                day,
                received_event: self.events.next_id(),
            };
            // The flying group's command holds the count first (decision
            // 0135, 1).
            let holders = Holders::Commands {
                commands: vec![self.command_of_group(group.id)],
                key: report.received_event,
                day,
                pulse: Some(pulse),
            };
            let ApplyOutcome::Estimate { replaced } =
                self.apply_report(group.side, &Report::Ground(report), &holders, &search)
            else {
                unreachable!("a ground report makes or replaces an estimate");
            };
            // The truth and the error drawn are the umpire's: a side is
            // given this event with no breakdown.
            let breakdown = self.config.record_breakdowns.then(|| Breakdown {
                factors: vec![
                    Factor {
                        name: "true_men",
                        value: Fixed::from_int(i64::from(true_men)),
                    },
                    Factor {
                        name: "reconnaissance_error_low",
                        value: error.low,
                    },
                    Factor {
                        name: "reconnaissance_error_high",
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
                Some(pulse),
                EventKind::GroundReportReceived {
                    side: group.side,
                    hex,
                    men,
                    replaced,
                    grade: GroundGrade::SeenFromAir,
                },
                vec![Cause::Event(flown)],
                breakdown,
            );
            assert_eq!(
                received, report.received_event,
                "the report cites the event that records it"
            );
            self.pass_report(group.side, &Report::Ground(report), &holders);
            self.find_hidden_bases(day, Some(pulse), hex, group.side, received);
        }
    }
}
