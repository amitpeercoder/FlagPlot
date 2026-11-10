//! Static reference data, loaded once and never changed by resolution.
//!
//! Only the fields the scenarios so far need exist: ship classes (speed,
//! fuel, cargo capacity), the port size table, aircraft types (search range
//! and sweep width) and the search figures of decision 0012. The spec lists
//! more for a ship class (guns, armour, aircraft capacity and so on) and one
//! more entity (weapons). They arrive with the scenarios that use them.

use std::collections::BTreeMap;

use crate::fixed::Fixed;
use crate::ids::{AircraftTypeId, ShipClassId};

/// Broad type of a ship class.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum ShipKind {
    Carrier,
    Battleship,
    Cruiser,
    Destroyer,
    Transport,
    Auxiliary,
}

impl ShipKind {
    /// Every kind, in importance order (decision 9 of
    /// `docs/decisions/0012-search-and-contacts.md`): the carrier is the most
    /// important. The derived `Ord` matches this order, so the most important
    /// kind of a set is its minimum.
    pub const ALL: [ShipKind; 6] = [
        ShipKind::Carrier,
        ShipKind::Battleship,
        ShipKind::Cruiser,
        ShipKind::Destroyer,
        ShipKind::Transport,
        ShipKind::Auxiliary,
    ];

    /// A stable number for the kind. Part of the state hash. Never renumber
    /// it; add new kinds at the end.
    pub const fn code(self) -> u8 {
        match self {
            ShipKind::Carrier => 1,
            ShipKind::Battleship => 2,
            ShipKind::Cruiser => 3,
            ShipKind::Destroyer => 4,
            ShipKind::Transport => 5,
            ShipKind::Auxiliary => 6,
        }
    }

    /// The kind's name as the event log writes it.
    pub const fn name(self) -> &'static str {
        match self {
            ShipKind::Carrier => "carrier",
            ShipKind::Battleship => "battleship",
            ShipKind::Cruiser => "cruiser",
            ShipKind::Destroyer => "destroyer",
            ShipKind::Transport => "transport",
            ShipKind::Auxiliary => "auxiliary",
        }
    }
}

/// One class of ship.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ShipClass {
    pub id: ShipClassId,
    pub name: String,
    pub kind: ShipKind,
    /// Economical speed, in knots. Endurance is quoted at this speed.
    pub cruise_speed_kn: Fixed,
    /// Top speed, in knots.
    pub max_speed_kn: Fixed,
    /// Full fuel load, in tons.
    pub fuel_capacity_t: Fixed,
    /// Distance a full fuel load lasts at cruise speed, in nautical miles.
    pub endurance_nm: Fixed,
    /// Supply the ship can carry, in tons. May be zero.
    pub cargo_capacity_t: Fixed,
}

impl ShipClass {
    /// Describes what is wrong with the class's numbers, if anything.
    pub fn problem(&self) -> Option<String> {
        let positive = [
            ("cruise speed", self.cruise_speed_kn),
            ("max speed", self.max_speed_kn),
            ("fuel capacity", self.fuel_capacity_t),
            ("endurance", self.endurance_nm),
        ];
        for (label, value) in positive {
            if value <= Fixed::ZERO {
                return Some(format!("{} has no positive {label}", self.id));
            }
        }
        if self.max_speed_kn < self.cruise_speed_kn {
            return Some(format!(
                "{} has a max speed below its cruise speed",
                self.id
            ));
        }
        if self.cargo_capacity_t.is_negative() {
            return Some(format!("{} has a negative cargo capacity", self.id));
        }
        None
    }
}

/// What an aircraft type is for.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum AircraftRole {
    /// Long-range search aircraft.
    Patrol,
}

impl AircraftRole {
    /// A stable number for the role. Never renumber it; add new roles at the
    /// end.
    pub const fn code(self) -> u8 {
        match self {
            AircraftRole::Patrol => 1,
        }
    }
}

/// One type of aircraft.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct AircraftType {
    pub id: AircraftTypeId,
    pub name: String,
    pub role: AircraftRole,
    /// The furthest a search by this type can reach, in nautical miles.
    pub search_range_nm: Fixed,
    /// The width of sea one aircraft covers as it flies, in nautical miles.
    pub sweep_width_nm: Fixed,
}

impl AircraftType {
    /// Describes what is wrong with the type's numbers, if anything.
    pub fn problem(&self) -> Option<String> {
        let positive = [
            ("search range", self.search_range_nm),
            ("sweep width", self.sweep_width_nm),
        ];
        for (label, value) in positive {
            if value <= Fixed::ZERO {
                return Some(format!("{} has no positive {label}", self.id));
            }
        }
        None
    }
}

/// The figures of the search, report and contact rules. See
/// `docs/decisions/0012-search-and-contacts.md`. Every chance-like value is
/// between 0 and 1.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SearchFigures {
    /// Confidence of a fresh report.
    pub initial_confidence: Fixed,
    /// A contact's confidence is multiplied by this each pulse it goes
    /// unrefreshed.
    pub confidence_decay_per_pulse: Fixed,
    /// A contact whose confidence falls below this is dropped.
    pub drop_below_confidence: Fixed,
    /// A report refreshes a contact at most this many hexes away...
    pub merge_distance_hexes: u32,
    /// ...and no older than this many pulses.
    pub merge_max_age_pulses: u32,
    /// Chance a report names a neighbouring hex.
    pub position_error_chance: Fixed,
    /// Chance a report counts one ship too many.
    pub count_over_chance: Fixed,
    /// Chance a report counts one ship too few.
    pub count_under_chance: Fixed,
    /// Chance a sighted task force notices the aircraft that sighted it.
    pub notice_chance: Fixed,
    /// True kind, then reported kind, then the chance of that report. Each
    /// true kind's row adds to exactly 1.
    pub kind_confusion: BTreeMap<ShipKind, BTreeMap<ShipKind, Fixed>>,
}

impl SearchFigures {
    /// Describes what is wrong with the figures, if anything.
    pub fn problem(&self) -> Option<String> {
        let in_unit = |value: Fixed| value >= Fixed::ZERO && value <= Fixed::ONE;
        let chances = [
            ("initial confidence", self.initial_confidence),
            ("confidence decay", self.confidence_decay_per_pulse),
            ("drop-below confidence", self.drop_below_confidence),
            ("position error chance", self.position_error_chance),
            ("count over chance", self.count_over_chance),
            ("count under chance", self.count_under_chance),
            ("notice chance", self.notice_chance),
        ];
        for (label, value) in chances {
            if !in_unit(value) {
                return Some(format!("search {label} {value} is not between 0 and 1"));
            }
        }
        if self.count_over_chance + self.count_under_chance > Fixed::ONE {
            return Some("search count over and under chances add to more than 1".to_string());
        }
        for kind in ShipKind::ALL {
            let Some(row) = self.kind_confusion.get(&kind) else {
                return Some(format!("search kind confusion has no row for {kind:?}"));
            };
            let mut total = Fixed::ZERO;
            for (reported, chance) in row {
                if !in_unit(*chance) {
                    return Some(format!(
                        "search kind confusion {kind:?} as {reported:?} is {chance}, not between 0 and 1"
                    ));
                }
                total += *chance;
            }
            if total != Fixed::ONE {
                return Some(format!(
                    "search kind confusion row for {kind:?} adds to {total}, not 1"
                ));
            }
        }
        None
    }
}

/// What a port of one size can handle. See
/// `docs/decisions/0011-cargo-handling.md`.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct PortSize {
    /// Ships that can work cargo alongside at once.
    pub berths: u32,
    /// Tons per hour moved by one ship on a berth.
    pub berth_rate_t_per_h: Fixed,
    /// Tons per hour moved by one ship without a berth, working by lighter.
    pub lighter_rate_t_per_h: Fixed,
}

impl PortSize {
    /// Describes what is wrong with the figures for port size `size`, if
    /// anything.
    pub fn problem(&self, size: u8) -> Option<String> {
        if self.berth_rate_t_per_h.is_negative() {
            return Some(format!("port size {size} has a negative berth rate"));
        }
        if self.lighter_rate_t_per_h.is_negative() {
            return Some(format!("port size {size} has a negative lighter rate"));
        }
        if self.berths > 0 && self.berth_rate_t_per_h.is_zero() {
            return Some(format!(
                "port size {size} has berths but no positive berth rate"
            ));
        }
        None
    }
}

/// All reference data for a game.
#[derive(Clone, PartialEq, Eq, Debug, Default)]
pub struct RefData {
    pub ship_classes: BTreeMap<ShipClassId, ShipClass>,
    /// Cargo handling figures by port size, 0 to 10.
    pub port_sizes: BTreeMap<u8, PortSize>,
    pub aircraft_types: BTreeMap<AircraftTypeId, AircraftType>,
    /// The search figures. `None` only in unit tests that have no air
    /// groups; the loader always supplies it.
    pub search: Option<SearchFigures>,
}

#[cfg(test)]
impl SearchFigures {
    /// The figures of `data/reference/search.toml`, written out.
    pub(crate) fn from_search_toml() -> SearchFigures {
        let tenth = |n: i64| Fixed::from_ratio(n, 10);
        let row = |entries: &[(ShipKind, i64)]| -> BTreeMap<ShipKind, Fixed> {
            entries.iter().map(|(k, n)| (*k, tenth(*n))).collect()
        };
        let mut kind_confusion = BTreeMap::new();
        kind_confusion.insert(
            ShipKind::Carrier,
            row(&[
                (ShipKind::Carrier, 7),
                (ShipKind::Battleship, 1),
                (ShipKind::Cruiser, 1),
                (ShipKind::Transport, 1),
            ]),
        );
        kind_confusion.insert(
            ShipKind::Battleship,
            row(&[
                (ShipKind::Battleship, 7),
                (ShipKind::Carrier, 1),
                (ShipKind::Cruiser, 2),
            ]),
        );
        kind_confusion.insert(
            ShipKind::Cruiser,
            row(&[
                (ShipKind::Cruiser, 7),
                (ShipKind::Battleship, 1),
                (ShipKind::Destroyer, 1),
                (ShipKind::Carrier, 1),
            ]),
        );
        kind_confusion.insert(
            ShipKind::Destroyer,
            row(&[
                (ShipKind::Destroyer, 7),
                (ShipKind::Cruiser, 2),
                (ShipKind::Transport, 1),
            ]),
        );
        kind_confusion.insert(
            ShipKind::Transport,
            row(&[
                (ShipKind::Transport, 7),
                (ShipKind::Auxiliary, 2),
                (ShipKind::Carrier, 1),
            ]),
        );
        kind_confusion.insert(
            ShipKind::Auxiliary,
            row(&[(ShipKind::Auxiliary, 7), (ShipKind::Transport, 3)]),
        );
        SearchFigures {
            initial_confidence: tenth(8),
            confidence_decay_per_pulse: tenth(8),
            drop_below_confidence: tenth(3),
            merge_distance_hexes: 2,
            merge_max_age_pulses: 2,
            position_error_chance: tenth(3),
            count_over_chance: tenth(2),
            count_under_chance: tenth(2),
            notice_chance: tenth(5),
            kind_confusion,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ship_kinds_are_in_importance_order_with_codes_one_to_six() {
        let mut sorted = ShipKind::ALL;
        sorted.sort();
        assert_eq!(sorted, ShipKind::ALL);
        let codes: Vec<u8> = ShipKind::ALL.iter().map(|kind| kind.code()).collect();
        assert_eq!(codes, vec![1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn the_figures_of_the_search_file_are_sound() {
        assert_eq!(SearchFigures::from_search_toml().problem(), None);
    }

    #[test]
    fn an_aircraft_type_needs_a_positive_range_and_sweep_width() {
        let good = AircraftType {
            id: AircraftTypeId::new(1),
            name: "Test".to_string(),
            role: AircraftRole::Patrol,
            search_range_nm: Fixed::from_int(600),
            sweep_width_nm: Fixed::from_int(40),
        };
        assert_eq!(good.problem(), None);
        let mut bad = good.clone();
        bad.search_range_nm = Fixed::ZERO;
        assert!(bad.problem().expect("problem").contains("search range"));
        let mut bad = good.clone();
        bad.sweep_width_nm = Fixed::from_int(-1);
        assert!(bad.problem().expect("problem").contains("sweep width"));
    }

    fn problem_of(change: impl FnOnce(&mut SearchFigures)) -> String {
        let mut figures = SearchFigures::from_search_toml();
        change(&mut figures);
        figures.problem().expect("the change makes a problem")
    }

    #[test]
    fn each_chance_must_be_between_zero_and_one() {
        let over = Fixed::from_ratio(11, 10);
        let under = Fixed::from_ratio(-1, 10);
        assert!(problem_of(|f| f.initial_confidence = over).contains("initial confidence"));
        assert!(problem_of(|f| f.initial_confidence = under).contains("initial confidence"));
        assert!(problem_of(|f| f.confidence_decay_per_pulse = over).contains("decay"));
        assert!(problem_of(|f| f.drop_below_confidence = under).contains("drop-below"));
        assert!(problem_of(|f| f.position_error_chance = over).contains("position error"));
        assert!(problem_of(|f| f.count_over_chance = under).contains("count over"));
        assert!(problem_of(|f| f.count_under_chance = over).contains("count under"));
        assert!(problem_of(|f| f.notice_chance = over).contains("notice"));
        // 0 and 1 themselves are allowed.
        let mut edge = SearchFigures::from_search_toml();
        edge.notice_chance = Fixed::ZERO;
        edge.initial_confidence = Fixed::ONE;
        assert_eq!(edge.problem(), None);
    }

    #[test]
    fn count_chances_may_not_add_to_more_than_one() {
        let message = problem_of(|f| {
            f.count_over_chance = Fixed::from_ratio(6, 10);
            f.count_under_chance = Fixed::from_ratio(5, 10);
        });
        assert!(message.contains("more than 1"));
        // Exactly 1 is allowed.
        let mut figures = SearchFigures::from_search_toml();
        figures.count_over_chance = Fixed::from_ratio(6, 10);
        figures.count_under_chance = Fixed::from_ratio(4, 10);
        assert_eq!(figures.problem(), None);
    }

    #[test]
    fn every_kind_needs_a_confusion_row_that_adds_to_one() {
        assert!(problem_of(|f| {
            f.kind_confusion.remove(&ShipKind::Destroyer);
        })
        .contains("no row"));
        assert!(problem_of(|f| {
            f.kind_confusion
                .get_mut(&ShipKind::Carrier)
                .expect("row")
                .insert(ShipKind::Carrier, Fixed::from_ratio(6, 10));
        })
        .contains("not 1"));
        assert!(problem_of(|f| {
            f.kind_confusion
                .get_mut(&ShipKind::Carrier)
                .expect("row")
                .insert(ShipKind::Carrier, Fixed::from_ratio(8, 10));
        })
        .contains("not 1"));
    }

    #[test]
    fn a_confusion_entry_outside_zero_to_one_is_caught() {
        // Carrier row 0.7 + 0.1 + 0.1 + 0.1 = 1. Make one entry -0.1 and
        // another 0.9 higher so the row still adds to 1: only the range check
        // can catch it.
        let message = problem_of(|f| {
            let row = f.kind_confusion.get_mut(&ShipKind::Carrier).expect("row");
            row.insert(ShipKind::Battleship, Fixed::from_ratio(-1, 10));
            row.insert(ShipKind::Carrier, Fixed::from_ratio(9, 10));
        });
        assert!(message.contains("not between 0 and 1"));
    }
}
