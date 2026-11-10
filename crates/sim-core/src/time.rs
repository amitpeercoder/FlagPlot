//! Game time: days, pulses and the clock.
//!
//! A day resolves as a 12-hour night pulse followed by two 6-hour day pulses.
//! Because the night pulse comes first, a game day starts at 18:00 and ends at
//! 18:00. See `docs/decisions/0001-day-boundary.md`.
//!
//! Time inside the simulation is "elapsed hours since the scenario began" as a
//! [`Fixed`]. Day 1 starts at elapsed hour 0.

use core::fmt;

use crate::fixed::Fixed;

pub const HOURS_PER_DAY: i64 = 24;
/// Clock hour at which every game day starts.
pub const DAY_START_CLOCK_HOUR: i64 = 18;

/// One of the three pulses a day resolves in.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum Pulse {
    Night,
    Morning,
    Afternoon,
}

impl Pulse {
    /// The pulses in the order they resolve.
    pub const ALL: [Pulse; 3] = [Pulse::Night, Pulse::Morning, Pulse::Afternoon];

    /// Length of the pulse in hours.
    pub const fn hours(self) -> i64 {
        match self {
            Pulse::Night => 12,
            Pulse::Morning | Pulse::Afternoon => 6,
        }
    }

    /// Hours from the start of the day to the start of the pulse.
    pub const fn offset_hours(self) -> i64 {
        match self {
            Pulse::Night => 0,
            Pulse::Morning => 12,
            Pulse::Afternoon => 18,
        }
    }

    /// A stable number for the pulse. Never renumber: it feeds the random
    /// number keys.
    pub const fn code(self) -> u64 {
        match self {
            Pulse::Night => 1,
            Pulse::Morning => 2,
            Pulse::Afternoon => 3,
        }
    }

    pub const fn name(self) -> &'static str {
        match self {
            Pulse::Night => "night",
            Pulse::Morning => "morning",
            Pulse::Afternoon => "afternoon",
        }
    }
}

/// Elapsed hours at the start of a pulse. Days count from 1.
pub fn pulse_start(day: u32, pulse: Pulse) -> Fixed {
    Fixed::from_int((i64::from(day) - 1) * HOURS_PER_DAY + pulse.offset_hours())
}

/// A moment shown the way a person reads it: game day and clock time.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct ClockTime {
    pub day: u32,
    pub hour: u8,
    pub minute: u8,
}

impl ClockTime {
    /// Converts elapsed hours to a game day and clock time, truncated to the
    /// minute.
    pub fn from_elapsed(elapsed_hours: Fixed) -> ClockTime {
        let raw = elapsed_hours.raw();
        let day_raw = HOURS_PER_DAY * Fixed::SCALE;
        let day_index = raw.div_euclid(day_raw);
        let into_day = raw.rem_euclid(day_raw);
        let minutes_into_day = into_day * 60 / Fixed::SCALE;
        let clock_minutes = (DAY_START_CLOCK_HOUR * 60 + minutes_into_day) % (HOURS_PER_DAY * 60);
        ClockTime {
            day: u32::try_from(day_index + 1).expect("elapsed time before day 1"),
            hour: u8::try_from(clock_minutes / 60).expect("hour in range"),
            minute: u8::try_from(clock_minutes % 60).expect("minute in range"),
        }
    }
}

impl fmt::Display for ClockTime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "D{} {:02}:{:02}", self.day, self.hour, self.minute)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pulses_cover_a_full_day() {
        let total: i64 = Pulse::ALL.iter().map(|pulse| pulse.hours()).sum();
        assert_eq!(total, HOURS_PER_DAY);
        assert_eq!(pulse_start(1, Pulse::Night), Fixed::ZERO);
        assert_eq!(pulse_start(1, Pulse::Afternoon), Fixed::from_int(18));
        assert_eq!(pulse_start(2, Pulse::Night), Fixed::from_int(24));
    }

    #[test]
    fn a_day_starts_at_1800() {
        let start = ClockTime::from_elapsed(Fixed::ZERO);
        assert_eq!(start.to_string(), "D1 18:00");
        let morning = ClockTime::from_elapsed(pulse_start(1, Pulse::Morning));
        assert_eq!(morning.to_string(), "D1 06:00");
        let next_day = ClockTime::from_elapsed(Fixed::from_int(24));
        assert_eq!(next_day.to_string(), "D2 18:00");
    }

    #[test]
    fn clock_time_truncates_to_the_minute() {
        // 27.5 elapsed hours is 3.5 hours into day 2: 21:30.
        let time = ClockTime::from_elapsed(Fixed::from_raw(27_500_000));
        assert_eq!(time.to_string(), "D2 21:30");
    }
}
