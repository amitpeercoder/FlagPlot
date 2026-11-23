//! The estimate of the enemy's boats (decision 0223, stage D1): what a
//! command makes of its own reports of submarines, a reading and no more.
//!
//! It is made from one command's picture, the side's own bases and the
//! side's own submarine doctrine, and from nothing else: not a world, not the
//! umpire's view, no event. It is kept nowhere, changes no state, no hash and
//! no event log, and no rule and no staff reads it.
//!
//! The reports the picture holds are taken in hex order. A report within
//! [`ESTIMATE_JOIN_HEXES`] of the first report of an area joins that area (the
//! earliest begun, where it is within reach of two), else it begins one; no
//! area joins another, so none chains across the map. The count of an area is
//! its reports that are more than one hex from every report counted before
//! them in the area, and is said "about N", never "at least". A side whose
//! scenario says it expects boats off its own bases is told each of its own
//! bases, in base number order, as waters where boats may be expected, with
//! no count and whatever reports it holds.

use crate::command::SubmarineDoctrine;
use crate::hex::Hex;
use crate::ids::BaseId;
use crate::view::CommandView;

/// How near a report must be to the first report of an area to join it, in
/// hexes. A judgement: no row sizes it (decision 0223, stage D1).
pub const ESTIMATE_JOIN_HEXES: u32 = 2;

/// An area of the estimate, on the basis "reported".
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct EstimateArea {
    /// The hex of its first report.
    pub centre: Hex,
    /// How many hexes round the centre it is told as: every report of the
    /// area lies within this distance of the centre.
    pub distance_hexes: u32,
    /// The hexes of its reports, in hex order.
    pub reports: Vec<Hex>,
    /// The count, said "about": a whole number of 1 or more.
    pub count: u32,
    /// The days of its reports, each once, in order.
    pub days: Vec<u32>,
}

/// One of the side's own bases, named as waters where boats may be expected,
/// on the basis "by habit". It has no count.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct HabitWaters {
    pub base: BaseId,
    pub hex: Hex,
}

/// The estimate: the reported areas in the order they were begun, then the
/// bases named by habit in base number order.
#[derive(Clone, PartialEq, Eq, Debug, Default)]
pub struct SubmarineEstimate {
    pub areas: Vec<EstimateArea>,
    pub by_habit: Vec<HabitWaters>,
}

impl SubmarineEstimate {
    /// Whether the estimate holds nothing.
    pub fn is_empty(&self) -> bool {
        self.areas.is_empty() && self.by_habit.is_empty()
    }
}

/// The estimate one command makes. `own_bases` is the side's own bases with
/// their hexes; they are read only when the doctrine expects boats off them.
pub fn submarine_estimate(
    picture: &CommandView<'_>,
    own_bases: &[(BaseId, Hex)],
    doctrine: SubmarineDoctrine,
) -> SubmarineEstimate {
    let mut reports: Vec<(Hex, u32)> = picture
        .submarine_reports()
        .map(|report| (report.hex, report.day))
        .collect();
    reports.sort_by_key(|(hex, _)| (hex.q, hex.r));
    let mut estimate = SubmarineEstimate {
        areas: areas_of(&reports),
        by_habit: Vec::new(),
    };
    if doctrine.expects_boats_off_own_bases {
        let mut bases: Vec<HabitWaters> = own_bases
            .iter()
            .map(|(base, hex)| HabitWaters {
                base: *base,
                hex: *hex,
            })
            .collect();
        bases.sort_by_key(|waters| waters.base);
        estimate.by_habit = bases;
    }
    estimate
}

/// The reported areas of reports given in hex order.
fn areas_of(reports: &[(Hex, u32)]) -> Vec<EstimateArea> {
    let mut areas: Vec<EstimateArea> = Vec::new();
    // For each area, the reports counted in it.
    let mut counted: Vec<Vec<Hex>> = Vec::new();
    for &(hex, day) in reports {
        let joined = areas
            .iter()
            .position(|area| area.centre.distance(hex) <= ESTIMATE_JOIN_HEXES);
        let at = match joined {
            Some(at) => at,
            None => {
                areas.push(EstimateArea {
                    centre: hex,
                    distance_hexes: ESTIMATE_JOIN_HEXES,
                    reports: Vec::new(),
                    count: 0,
                    days: Vec::new(),
                });
                counted.push(Vec::new());
                areas.len() - 1
            }
        };
        let area = &mut areas[at];
        area.reports.push(hex);
        if !area.days.contains(&day) {
            area.days.push(day);
            area.days.sort_unstable();
        }
        if counted[at].iter().all(|other| other.distance(hex) > 1) {
            counted[at].push(hex);
            area.count += 1;
        }
    }
    areas
}
