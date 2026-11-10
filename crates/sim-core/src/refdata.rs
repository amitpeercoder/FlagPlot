//! Static reference data, loaded once and never changed by resolution.
//!
//! Only the fields scenario 1 needs exist so far. The spec lists more for a
//! ship class (guns, armour, aircraft capacity and so on) and two more
//! entities (aircraft types and weapons). They arrive with the scenarios
//! that use them.

use std::collections::BTreeMap;

use crate::fixed::Fixed;
use crate::ids::ShipClassId;

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
        None
    }
}

/// All reference data for a game.
#[derive(Clone, PartialEq, Eq, Debug, Default)]
pub struct RefData {
    pub ship_classes: BTreeMap<ShipClassId, ShipClass>,
}
