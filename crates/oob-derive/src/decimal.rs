//! Exact decimals for the derivation arithmetic: whole millionths in an
//! `i64`. No floats. Multiplication and division truncate toward zero.

use std::fmt;

const SCALE: i64 = 1_000_000;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Dec(i64);

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ParseError {
    NotANumber,
    TooManyPlaces,
    TooLarge,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            ParseError::NotANumber => "not a whole number or decimal",
            ParseError::TooManyPlaces => "more than 6 decimal places",
            ParseError::TooLarge => "too large",
        })
    }
}

impl Dec {
    pub const ZERO: Dec = Dec(0);

    pub fn from_int(n: i64) -> Option<Dec> {
        n.checked_mul(SCALE).map(Dec)
    }

    /// `-?[0-9]+` or `-?[0-9]+\.[0-9]+`, at most 6 decimal places. Nothing
    /// is ever rounded.
    pub fn parse(text: &str) -> Result<Dec, ParseError> {
        let (negative, body) = match text.strip_prefix('-') {
            Some(rest) => (true, rest),
            None => (false, text),
        };
        let (whole, fraction) = match body.split_once('.') {
            Some((w, f)) => (w, f),
            None => (body, ""),
        };
        let digits = |s: &str| !s.is_empty() && s.bytes().all(|b| b.is_ascii_digit());
        if !digits(whole) || (body.contains('.') && !digits(fraction)) {
            return Err(ParseError::NotANumber);
        }
        if fraction.len() > 6 {
            return Err(ParseError::TooManyPlaces);
        }
        let whole: i128 = whole.parse().map_err(|_| ParseError::TooLarge)?;
        let mut padded = fraction.to_string();
        while padded.len() < 6 {
            padded.push('0');
        }
        let fraction: i128 = if fraction.is_empty() {
            0
        } else {
            padded.parse().map_err(|_| ParseError::NotANumber)?
        };
        let raw = whole * i128::from(SCALE) + fraction;
        let raw = if negative { -raw } else { raw };
        i64::try_from(raw)
            .map(Dec)
            .map_err(|_| ParseError::TooLarge)
    }

    /// `self + other`. `None` on overflow.
    pub fn checked_add(self, other: Dec) -> Option<Dec> {
        self.0.checked_add(other.0).map(Dec)
    }

    /// `self x other`, truncated toward zero. `None` on overflow.
    pub fn mul(self, other: Dec) -> Option<Dec> {
        let wide = i128::from(self.0) * i128::from(other.0) / i128::from(SCALE);
        i64::try_from(wide).ok().map(Dec)
    }

    /// `self / other`, truncated toward zero. `None` for a zero divisor or
    /// on overflow.
    pub fn div(self, other: Dec) -> Option<Dec> {
        if other.0 == 0 {
            return None;
        }
        let wide = i128::from(self.0) * i128::from(SCALE) / i128::from(other.0);
        i64::try_from(wide).ok().map(Dec)
    }

    /// The whole part, truncated toward zero.
    pub fn whole(self) -> i64 {
        self.0 / SCALE
    }

    pub fn has_fraction(self) -> bool {
        self.0 % SCALE != 0
    }

    pub fn is_negative(self) -> bool {
        self.0 < 0
    }

    pub fn is_positive(self) -> bool {
        self.0 > 0
    }
}

impl fmt::Display for Dec {
    /// No trailing zeros and no trailing point: `12`, `3.5`, `19.666666`.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let sign = if self.0 < 0 { "-" } else { "" };
        let abs = self.0.unsigned_abs();
        let scale = SCALE.unsigned_abs();
        let (whole, fraction) = (abs / scale, abs % scale);
        if fraction == 0 {
            write!(f, "{sign}{whole}")
        } else {
            let digits = format!("{fraction:06}");
            write!(f, "{sign}{whole}.{}", digits.trim_end_matches('0'))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn d(text: &str) -> Dec {
        Dec::parse(text).expect("valid")
    }

    #[test]
    fn parses_and_prints_without_trailing_zeros() {
        assert_eq!(d("12").to_string(), "12");
        assert_eq!(d("3.50").to_string(), "3.5");
        assert_eq!(d("36.8").to_string(), "36.8");
        assert_eq!(d("0.4").to_string(), "0.4");
        assert_eq!(d("-0.5").to_string(), "-0.5");
        assert_eq!(d("0.000001").to_string(), "0.000001");
    }

    #[test]
    fn rejects_bad_text_and_never_rounds() {
        assert_eq!(Dec::parse("0.1234567"), Err(ParseError::TooManyPlaces));
        assert_eq!(Dec::parse("1e5"), Err(ParseError::NotANumber));
        assert_eq!(Dec::parse(""), Err(ParseError::NotANumber));
        assert_eq!(Dec::parse("1."), Err(ParseError::NotANumber));
        assert_eq!(Dec::parse(".5"), Err(ParseError::NotANumber));
        assert_eq!(
            Dec::parse("99999999999999999999"),
            Err(ParseError::TooLarge)
        );
    }

    #[test]
    fn arithmetic_truncates_toward_zero() {
        assert_eq!(d("118").div(d("6")).unwrap().to_string(), "19.666666");
        assert_eq!(d("490").mul(d("0.4")).unwrap().to_string(), "196");
        assert_eq!(d("10000").div(d("25")).unwrap().to_string(), "400");
        assert_eq!(d("1624").div(d("25")).unwrap().to_string(), "64.96");
        assert_eq!(d("-7").div(d("2")).unwrap().to_string(), "-3.5");
        assert_eq!(d("-1").div(d("3")).unwrap().to_string(), "-0.333333");
        assert_eq!(d("19.666666").whole(), 19);
        assert!(d("19.666666").has_fraction());
        assert!(d("1").div(d("0")).is_none());
    }
}
