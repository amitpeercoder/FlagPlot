//! Dates in the facts files: `YYYY`, `YYYY-MM` or `YYYY-MM-DD`. A partial
//! date stands for an unknown day within that year or month, so whether a
//! row holds on a date can be yes, no or "cannot tell".

use std::fmt;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Day {
    year: u32,
    month: u32,
    day: u32,
}

impl fmt::Display for Day {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:04}-{:02}-{:02}", self.year, self.month, self.day)
    }
}

fn number(text: &str, len: usize) -> Option<u32> {
    if text.len() == len && text.bytes().all(|b| b.is_ascii_digit()) {
        text.parse().ok()
    } else {
        None
    }
}

fn days_in_month(year: u32, month: u32) -> u32 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        _ => {
            if (year.is_multiple_of(4) && !year.is_multiple_of(100)) || year.is_multiple_of(400) {
                29
            } else {
                28
            }
        }
    }
}

impl Day {
    /// This day with `months` taken off. A day that the earlier month does
    /// not have becomes that month's last day.
    pub fn minus_months(self, months: u32) -> Day {
        let total = (self.year * 12 + (self.month - 1)).saturating_sub(months);
        let (year, month) = (total / 12, total % 12 + 1);
        Day {
            year,
            month,
            day: self.day.min(days_in_month(year, month)),
        }
    }

    /// A full `YYYY-MM-DD`.
    pub fn parse_full(text: &str) -> Option<Day> {
        let span = Span::parse(text)?;
        (span.first == span.last).then_some(span.first)
    }
}

/// The first and last possible day of a date as written.
#[derive(Clone, Copy, Debug)]
pub struct Span {
    pub first: Day,
    pub last: Day,
    /// Written as a full date.
    pub full: bool,
}

impl Span {
    pub fn parse(text: &str) -> Option<Span> {
        let parts: Vec<&str> = text.split('-').collect();
        let year = number(parts.first()?, 4)?;
        let make = |first: (u32, u32), last: (u32, u32), full| {
            Some(Span {
                first: Day {
                    year,
                    month: first.0,
                    day: first.1,
                },
                last: Day {
                    year,
                    month: last.0,
                    day: last.1,
                },
                full,
            })
        };
        match parts.as_slice() {
            [_] => make((1, 1), (12, 31), false),
            [_, m] => {
                let month = number(m, 2).filter(|m| (1..=12).contains(m))?;
                make((month, 1), (month, days_in_month(year, month)), false)
            }
            [_, m, d] => {
                let month = number(m, 2).filter(|m| (1..=12).contains(m))?;
                let day = number(d, 2).filter(|d| (1..=days_in_month(year, month)).contains(d))?;
                make((month, day), (month, day), true)
            }
            _ => None,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Tri {
    Yes,
    No,
    CannotTell,
}

impl Tri {
    pub fn word(self) -> &'static str {
        match self {
            Tri::Yes => "holds",
            Tri::No => "does not hold",
            Tri::CannotTell => "cannot tell",
        }
    }
}

/// Whether a row with these `valid_from` and `valid_to` texts holds on
/// `date`. `valid_from` is inclusive and `valid_to` exclusive; an empty one
/// is open. Equal non-empty texts mean "true on this date, limits unknown".
/// The error is the text that is not a date.
pub fn valid_on(from: &str, to: &str, date: Day) -> Result<Tri, String> {
    let span = |text: &str| Span::parse(text).ok_or_else(|| text.to_string());
    if !from.is_empty() && from == to {
        let s = span(from)?;
        return Ok(if s.full {
            if s.first == date {
                Tri::Yes
            } else {
                Tri::No
            }
        } else if s.first <= date && date <= s.last {
            Tri::CannotTell
        } else {
            Tri::No
        });
    }
    let started = if from.is_empty() {
        Tri::Yes
    } else {
        let s = span(from)?;
        if s.last <= date {
            Tri::Yes
        } else if s.first > date {
            Tri::No
        } else {
            Tri::CannotTell
        }
    };
    let still = if to.is_empty() {
        Tri::Yes
    } else {
        let s = span(to)?;
        if date < s.first {
            Tri::Yes
        } else if s.last <= date {
            Tri::No
        } else {
            Tri::CannotTell
        }
    };
    Ok(match (started, still) {
        (Tri::No, _) | (_, Tri::No) => Tri::No,
        (Tri::Yes, Tri::Yes) => Tri::Yes,
        _ => Tri::CannotTell,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn on(from: &str, to: &str, date: &str) -> Tri {
        valid_on(from, to, Day::parse_full(date).unwrap()).unwrap()
    }

    #[test]
    fn full_dates_are_inclusive_then_exclusive() {
        assert_eq!(on("", "", "1942-08-08"), Tri::Yes);
        assert_eq!(on("1942-08-08", "", "1942-08-08"), Tri::Yes);
        assert_eq!(on("1942-08-09", "", "1942-08-08"), Tri::No);
        assert_eq!(on("", "1942-08-08", "1942-08-08"), Tri::No);
        assert_eq!(on("", "1942-08-09", "1942-08-08"), Tri::Yes);
    }

    #[test]
    fn partial_dates_cannot_tell_inside_their_span() {
        assert_eq!(on("1942-08", "", "1942-08-08"), Tri::CannotTell);
        assert_eq!(on("1942-08", "", "1942-08-31"), Tri::Yes);
        assert_eq!(on("1942-09", "", "1942-08-08"), Tri::No);
        assert_eq!(on("", "1942", "1942-08-08"), Tri::CannotTell);
        assert_eq!(on("", "1942", "1941-12-31"), Tri::Yes);
        assert_eq!(on("", "1942", "1943-01-01"), Tri::No);
        assert_eq!(on("1943", "1942-08", "1942-08-08"), Tri::No);
    }

    #[test]
    fn single_date_form_holds_on_that_date_only() {
        assert_eq!(on("1942-08-08", "1942-08-08", "1942-08-08"), Tri::Yes);
        assert_eq!(on("1942-08-08", "1942-08-08", "1942-08-09"), Tri::No);
        assert_eq!(on("1942-08", "1942-08", "1942-08-08"), Tri::CannotTell);
        assert_eq!(on("1942-08", "1942-08", "1942-09-01"), Tri::No);
    }

    #[test]
    fn months_taken_off_a_day() {
        let day = |t: &str| Day::parse_full(t).unwrap();
        assert_eq!(day("1942-08-08").minus_months(3), day("1942-05-08"));
        assert_eq!(day("1942-08-08").minus_months(12), day("1941-08-08"));
        assert_eq!(day("1942-10-11").minus_months(12), day("1941-10-11"));
        assert_eq!(day("1942-01-15").minus_months(2), day("1941-11-15"));
        assert_eq!(day("1942-05-31").minus_months(3), day("1942-02-28"));
        assert_eq!(day("1944-05-31").minus_months(3), day("1944-02-29"));
    }

    #[test]
    fn bad_dates_are_errors() {
        let day = Day::parse_full("1942-08-08").unwrap();
        assert!(valid_on("1942-13", "", day).is_err());
        assert!(valid_on("", "1942-02-30", day).is_err());
        assert!(Day::parse_full("1942-08").is_none());
        assert!(Day::parse_full("1942-8-8").is_none());
    }
}
