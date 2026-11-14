//! Days of supply: a side's stock in a hex over its planned use there for a
//! day (decision 0107, 31 to 36, with rulings 1 to 3 of its hand
//! calculations). A function of the side's view: it reads the side's own
//! bases, men, forces and air groups in the hex, its own forces bound there
//! with men aboard, and the figures. Nothing of the enemy. Writes nothing and
//! draws no random number.
//!
//! It is right when read where a staff is asked: at the start of a day,
//! after ships in port have refuelled and before the day's orders are taken
//! in. Read between days it gives the same, since nothing between the end of
//! a day and that step changes a stock or a use.

use crate::fixed::Fixed;
use crate::hex::Hex;

use super::SideView;

/// The parts of a side's planned use of supply in a hex for one day, each in
/// tons (decision 0107, 32).
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct PlannedUse {
    /// The men's supply for the side's men ashore in the hex, each unit at
    /// its weight.
    pub men_ashore_t: Fixed,
    /// The men's supply for men aboard the side's own forces that are
    /// ordered to land in the hex and are due that day, or were due earlier
    /// and have not landed (ruling 1).
    pub men_due_t: Fixed,
    /// The daily upkeep of the side's aircraft at its bases there, ready
    /// and damaged.
    pub aircraft_upkeep_t: Fixed,
    /// The supply of the repairs its bases there would make that night: for
    /// each, the smaller of its repairs a day and its damaged aircraft, at
    /// the supply of a repair.
    pub repairs_t: Fixed,
    /// The sorties of its groups there on search or patrol, at each of the
    /// day's two daylight pulses, with the aircraft able to fly (ruling 2).
    /// A strike mission counts nothing (decision 33).
    pub sorties_t: Fixed,
}

impl PlannedUse {
    pub fn total_t(&self) -> Fixed {
        self.men_ashore_t + self.men_due_t + self.aircraft_upkeep_t + self.repairs_t + self.sorties_t
    }
}

/// A side's days of supply in a hex where it holds a base.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct DaysOfSupply {
    pub hex: Hex,
    /// The supply of all the side's bases in the hex.
    pub stock_t: Fixed,
    pub planned_use: PlannedUse,
    /// The stock over the planned use, cut at six decimals. `None` where
    /// the planned use is 0: no planned use, no figure (decision 36).
    pub days: Option<Fixed>,
}

impl SideView<'_> {
    /// The side's days of supply in `hex`, with its parts. `None` if the
    /// side holds no base there (decision 36).
    pub fn days_of_supply(&self, hex: Hex) -> Option<DaysOfSupply> {
        let _ = hex;
        todo!("decision 0107, 31 to 36: built by the days-of-supply task")
    }
}
