//! Small exact arithmetic the doctrine shares: whole days from hours,
//! whole numbers from tons, and the words for a quantity. No floating
//! point anywhere: a ceiling is an integer division on raw millionths.

use sim_core::fixed::Fixed;

/// Hours in a day (a night pulse of 12 and two daylight pulses of 6).
pub(crate) const HOURS_PER_DAY: i64 = 24;

/// A whole number of days or tons as a `Fixed`.
pub(crate) fn whole(n: u32) -> Fixed {
    Fixed::from_int(i64::from(n))
}

/// The floor of `value` as a whole number, 0 for 0 or less.
pub(crate) fn floor_whole(value: Fixed) -> u32 {
    if value <= Fixed::ZERO {
        return 0;
    }
    u32::try_from(value.trunc()).unwrap_or(u32::MAX)
}

/// Whole days a stretch of `hours` takes, rounded up: 20 hours is 1 day,
/// 24 hours is 1 day, 24.5 hours is 2 days. 0 hours is 0 days.
pub(crate) fn ceil_days(hours: Fixed) -> u32 {
    if hours <= Fixed::ZERO {
        return 0;
    }
    let raw = hours.raw();
    let den = HOURS_PER_DAY * Fixed::SCALE;
    u32::try_from((raw + den - 1) / den).unwrap_or(u32::MAX)
}

/// `a` over `b` with `b` above 0, else 0.
pub(crate) fn over(a: Fixed, b: Fixed) -> Fixed {
    if b <= Fixed::ZERO {
        Fixed::ZERO
    } else {
        a / b
    }
}

/// Tons in words: "360 t".
pub(crate) fn tons(value: Fixed) -> String {
    format!("{value} t")
}

/// "day" or "days".
/// "day" for exactly one, else "days", for a quantity of days in `Fixed`.
pub(crate) fn days_word_of(n: Fixed) -> &'static str {
    if n == Fixed::from_int(1) {
        "day"
    } else {
        "days"
    }
}

pub(crate) fn days_word(n: u32) -> &'static str {
    if n == 1 {
        "day"
    } else {
        "days"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn days_round_up_in_whole_days() {
        assert_eq!(ceil_days(Fixed::from_int(20)), 1);
        assert_eq!(ceil_days(Fixed::from_int(24)), 1);
        assert_eq!(ceil_days(Fixed::from_ratio(49, 2)), 2);
        assert_eq!(ceil_days(Fixed::ZERO), 0);
        assert_eq!(floor_whole(Fixed::from_ratio(7, 2)), 3);
    }
}
