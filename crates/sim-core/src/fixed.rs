//! Fixed-point arithmetic for all simulation state.
//!
//! A [`Fixed`] is a signed 64-bit integer counting millionths. Every quantity
//! that feeds the state hash (distances, speeds, fuel, hours) uses it, so a
//! run produces the same bits on every machine. Division and multiplication
//! truncate toward zero. Overflow panics in every build profile.

use core::fmt;
use core::ops::{Add, AddAssign, Div, Mul, Neg, Sub, SubAssign};
use core::str::FromStr;

/// A fixed-point number with six decimal places.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Fixed(i64);

impl Fixed {
    /// Raw units per whole number.
    pub const SCALE: i64 = 1_000_000;
    /// Number of decimal places.
    pub const DECIMALS: usize = 6;
    pub const ZERO: Fixed = Fixed(0);
    pub const ONE: Fixed = Fixed(Self::SCALE);

    /// A whole number.
    pub const fn from_int(n: i64) -> Fixed {
        Fixed(n * Self::SCALE)
    }

    /// Builds a value from raw millionths.
    pub const fn from_raw(raw: i64) -> Fixed {
        Fixed(raw)
    }

    /// The value in raw millionths.
    pub const fn raw(self) -> i64 {
        self.0
    }

    /// `num / den`, truncated toward zero.
    pub fn from_ratio(num: i64, den: i64) -> Fixed {
        Fixed(narrow(
            i128::from(num) * i128::from(Self::SCALE) / i128::from(den),
        ))
    }

    /// `self * num / den` with a 128-bit intermediate, truncated toward zero.
    /// Use this instead of a separate multiply and divide to avoid losing
    /// precision in the middle.
    pub fn mul_div(self, num: Fixed, den: Fixed) -> Fixed {
        Fixed(narrow(
            i128::from(self.0) * i128::from(num.0) / i128::from(den.0),
        ))
    }

    /// The whole part, truncated toward zero.
    pub const fn trunc(self) -> i64 {
        self.0 / Self::SCALE
    }

    pub const fn is_negative(self) -> bool {
        self.0 < 0
    }

    pub const fn is_zero(self) -> bool {
        self.0 == 0
    }
}

fn narrow(value: i128) -> i64 {
    i64::try_from(value).expect("fixed-point overflow")
}

impl Add for Fixed {
    type Output = Fixed;
    fn add(self, rhs: Fixed) -> Fixed {
        Fixed(self.0.checked_add(rhs.0).expect("fixed-point overflow"))
    }
}

impl Sub for Fixed {
    type Output = Fixed;
    fn sub(self, rhs: Fixed) -> Fixed {
        Fixed(self.0.checked_sub(rhs.0).expect("fixed-point overflow"))
    }
}

impl Neg for Fixed {
    type Output = Fixed;
    fn neg(self) -> Fixed {
        Fixed(self.0.checked_neg().expect("fixed-point overflow"))
    }
}

impl AddAssign for Fixed {
    fn add_assign(&mut self, rhs: Fixed) {
        *self = *self + rhs;
    }
}

impl SubAssign for Fixed {
    fn sub_assign(&mut self, rhs: Fixed) {
        *self = *self - rhs;
    }
}

impl Mul for Fixed {
    type Output = Fixed;
    fn mul(self, rhs: Fixed) -> Fixed {
        Fixed(narrow(
            i128::from(self.0) * i128::from(rhs.0) / i128::from(Self::SCALE),
        ))
    }
}

impl Mul<i64> for Fixed {
    type Output = Fixed;
    fn mul(self, rhs: i64) -> Fixed {
        Fixed(self.0.checked_mul(rhs).expect("fixed-point overflow"))
    }
}

impl Div for Fixed {
    type Output = Fixed;
    fn div(self, rhs: Fixed) -> Fixed {
        Fixed(narrow(
            i128::from(self.0) * i128::from(Self::SCALE) / i128::from(rhs.0),
        ))
    }
}

impl fmt::Display for Fixed {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let sign = if self.0 < 0 { "-" } else { "" };
        let magnitude = self.0.unsigned_abs();
        let scale = Self::SCALE.unsigned_abs();
        let whole = magnitude / scale;
        let frac = magnitude % scale;
        if frac == 0 {
            write!(f, "{sign}{whole}")
        } else {
            let digits = format!("{frac:06}");
            write!(f, "{sign}{whole}.{}", digits.trim_end_matches('0'))
        }
    }
}

impl fmt::Debug for Fixed {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

/// Why a decimal string could not be read as a [`Fixed`].
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum ParseFixedError {
    Empty,
    InvalidDigit,
    TooManyDecimals,
    Overflow,
}

impl fmt::Display for ParseFixedError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let text = match self {
            ParseFixedError::Empty => "empty number",
            ParseFixedError::InvalidDigit => "not a decimal number",
            ParseFixedError::TooManyDecimals => "more than six decimal places",
            ParseFixedError::Overflow => "number out of range",
        };
        f.write_str(text)
    }
}

impl std::error::Error for ParseFixedError {}

impl FromStr for Fixed {
    type Err = ParseFixedError;

    /// Parses a plain decimal such as `12`, `-0.25` or `3.125000`. No
    /// exponents and no floats on the way: the digits are read exactly.
    fn from_str(text: &str) -> Result<Fixed, ParseFixedError> {
        let (negative, body) = match text.strip_prefix('-') {
            Some(rest) => (true, rest),
            None => (false, text),
        };
        let (whole_text, frac_text) = match body.split_once('.') {
            Some((whole, frac)) => (whole, frac),
            None => (body, ""),
        };
        if whole_text.is_empty() && frac_text.is_empty() {
            return Err(ParseFixedError::Empty);
        }
        if frac_text.len() > Self::DECIMALS {
            return Err(ParseFixedError::TooManyDecimals);
        }
        let all_digits = |part: &str| part.bytes().all(|b| b.is_ascii_digit());
        if !all_digits(whole_text) || !all_digits(frac_text) {
            return Err(ParseFixedError::InvalidDigit);
        }

        let mut raw: i64 = 0;
        let padded_frac = format!("{frac_text:0<6}");
        for byte in whole_text.bytes().chain(padded_frac.bytes()) {
            raw = raw
                .checked_mul(10)
                .and_then(|value| value.checked_add(i64::from(byte - b'0')))
                .ok_or(ParseFixedError::Overflow)?;
        }
        Ok(Fixed(if negative { -raw } else { raw }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn whole_numbers_round_trip() {
        assert_eq!(Fixed::from_int(40).raw(), 40_000_000);
        assert_eq!(Fixed::from_int(40).trunc(), 40);
        assert_eq!(Fixed::from_int(-3).trunc(), -3);
    }

    #[test]
    fn arithmetic_is_exact_for_representable_values() {
        let quarter = Fixed::from_ratio(1, 4);
        assert_eq!(quarter.raw(), 250_000);
        assert_eq!(Fixed::from_int(440) * quarter, Fixed::from_int(110));
        assert_eq!(
            Fixed::from_int(440) / Fixed::from_int(16),
            Fixed::from_raw(27_500_000)
        );
        assert_eq!(Fixed::from_int(16) * 12, Fixed::from_int(192));
    }

    #[test]
    fn mul_div_keeps_precision_a_separate_divide_would_lose() {
        // 100 * 1 / 3: dividing first truncates 1/3 to 0.333333.
        let hundred = Fixed::from_int(100);
        let one = Fixed::ONE;
        let three = Fixed::from_int(3);
        assert_eq!(hundred.mul_div(one, three).raw(), 33_333_333);
        assert_eq!((hundred * (one / three)).raw(), 33_333_300);
    }

    #[test]
    fn division_truncates_toward_zero() {
        assert_eq!(Fixed::from_ratio(1, 3).raw(), 333_333);
        assert_eq!(Fixed::from_ratio(-1, 3).raw(), -333_333);
    }

    #[test]
    fn parses_decimal_strings_exactly() {
        assert_eq!("2000".parse::<Fixed>(), Ok(Fixed::from_int(2000)));
        assert_eq!("0.25".parse::<Fixed>(), Ok(Fixed::from_raw(250_000)));
        assert_eq!("-1.5".parse::<Fixed>(), Ok(Fixed::from_raw(-1_500_000)));
        assert_eq!(".5".parse::<Fixed>(), Ok(Fixed::from_raw(500_000)));
        assert_eq!("0.000001".parse::<Fixed>(), Ok(Fixed::from_raw(1)));
    }

    #[test]
    fn rejects_bad_decimal_strings() {
        assert_eq!("".parse::<Fixed>(), Err(ParseFixedError::Empty));
        assert_eq!("-".parse::<Fixed>(), Err(ParseFixedError::Empty));
        assert_eq!("1e3".parse::<Fixed>(), Err(ParseFixedError::InvalidDigit));
        assert_eq!("1.2.3".parse::<Fixed>(), Err(ParseFixedError::InvalidDigit));
        assert_eq!(
            "0.1234567".parse::<Fixed>(),
            Err(ParseFixedError::TooManyDecimals)
        );
        assert_eq!(
            "99999999999999999999".parse::<Fixed>(),
            Err(ParseFixedError::Overflow)
        );
    }

    #[test]
    fn displays_without_trailing_zeros() {
        assert_eq!(Fixed::from_int(110).to_string(), "110");
        assert_eq!(Fixed::from_raw(27_500_000).to_string(), "27.5");
        assert_eq!(Fixed::from_raw(-250_000).to_string(), "-0.25");
        assert_eq!(Fixed::from_raw(1).to_string(), "0.000001");
    }

    #[test]
    #[should_panic(expected = "fixed-point overflow")]
    fn overflow_panics_instead_of_wrapping() {
        let _ = Fixed::from_raw(i64::MAX) + Fixed::from_raw(1);
    }
}
