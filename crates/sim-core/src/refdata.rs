//! Static reference data, loaded once and never changed by resolution.
//!
//! Only the fields scenarios 1 and 2 need exist so far: ship classes (speed,
//! fuel, cargo capacity) and the port size table. The spec lists more for a
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
}
