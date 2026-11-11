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
use crate::hex::Hex;

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

/// Whether it is dark at `hex` in `pulse`. Dark pulses allow no search.
///
/// For now it is dark exactly in the night pulse, everywhere
/// (`docs/decisions/0001-day-boundary.md`, decision 7 of
/// `docs/decisions/0012-search-and-contacts.md`). The hex is a parameter
/// because later rules will vary darkness by place; callers pass it now so
/// they do not change then.
pub fn is_dark(_hex: Hex, pulse: Pulse) -> bool {
    pulse == Pulse::Night
}

/// A moment shown the way a person reads it: game day and clock time.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct ClockTime {
    pub day: u32,
    pub hour: u8,
    pub minute: u8,
}

impl ClockTime {
    /// Converts elapsed hours to a game day and clock time, rounded to the
    /// nearest minute (half a minute rounds up; decision 0032). A moment
    /// within half a minute of the end of a game day reads as the start of
    /// the next.
    pub fn from_elapsed(elapsed_hours: Fixed) -> ClockTime {
        let raw = elapsed_hours.raw();
        let total_minutes = (raw * 60 + Fixed::SCALE / 2).div_euclid(Fixed::SCALE);
        let day_minutes = HOURS_PER_DAY * 60;
        let day_index = total_minutes.div_euclid(day_minutes);
        let minutes_into_day = total_minutes.rem_euclid(day_minutes);
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
    fn it_is_dark_only_in_the_night_pulse() {
        let hex = Hex::new(3, -2);
        assert!(is_dark(hex, Pulse::Night));
        assert!(!is_dark(hex, Pulse::Morning));
        assert!(!is_dark(hex, Pulse::Afternoon));
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
    fn clock_time_rounds_to_the_nearest_minute() {
        // 27.5 elapsed hours is 3.5 hours into day 2: 21:30.
        let time = ClockTime::from_elapsed(Fixed::from_raw(27_500_000));
        assert_eq!(time.to_string(), "D2 21:30");
        // 14/3 hours is 4 hours 40 minutes after 18:00: 22:40. It is held as
        // 4.666666 h, which is 39.99996 min past the hour and used to print
        // as 22:39.
        let time = ClockTime::from_elapsed(Fixed::from_ratio(14, 3));
        assert_eq!(time.to_string(), "D1 22:40");
        // Half a minute is 8333.33 millionths of an hour, so no stored value
        // is exactly on it: 8334 is just over and rounds up, 8333 is just
        // under and rounds down.
        let time = ClockTime::from_elapsed(Fixed::from_int(1) + Fixed::from_raw(8334));
        assert_eq!(time.to_string(), "D1 19:01");
        let time = ClockTime::from_elapsed(Fixed::from_int(1) + Fixed::from_raw(8333));
        assert_eq!(time.to_string(), "D1 19:00");
        // A whole minute after the half rounds the same way.
        let time = ClockTime::from_elapsed(Fixed::from_int(1) + Fixed::from_raw(25_000));
        assert_eq!(time.to_string(), "D1 19:02");
    }

    #[test]
    fn clock_time_rounds_across_midnight_and_the_end_of_the_game_day() {
        // 5 h 59 min 30 s elapsed is 23:59:30, 5.991667 h as stored (just
        // over the half minute): it rounds to 00:00, still game day 1 (the
        // game day changes at 18:00, not at midnight).
        let time = ClockTime::from_elapsed(Fixed::from_raw(5_991_667));
        assert_eq!(time.to_string(), "D1 00:00");
        // One millionth less is just under the half minute and stays 23:59.
        let time = ClockTime::from_elapsed(Fixed::from_raw(5_991_666));
        assert_eq!(time.to_string(), "D1 23:59");
        // 17:59:30 on game day 1 is 23.991667 h elapsed: it rounds up into
        // the next game day, D2 18:00, and not into minute 60 of D1.
        let time = ClockTime::from_elapsed(Fixed::from_raw(23_991_667));
        assert_eq!(time.to_string(), "D2 18:00");
        let time = ClockTime::from_elapsed(Fixed::from_raw(23_991_666));
        assert_eq!(time.to_string(), "D1 17:59");
    }
}
