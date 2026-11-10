//! Fixed-point numbers as JSON numbers.

use serde::ser::Error as _;
use serde::{Serialize, Serializer};
use serde_json::value::RawValue;
use sim_core::fixed::Fixed;

/// A [`Fixed`] written as a JSON number, exactly. `Fixed`'s `Display` is
/// built from the millionths with integer arithmetic (`12`, `-0.5`,
/// `0.333333`) and is always a valid JSON number, so it is passed through as
/// raw text. No float is involved.
#[derive(Clone, Copy, Debug)]
pub struct Num(pub Fixed);

impl Serialize for Num {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let raw = RawValue::from_string(self.0.to_string()).map_err(S::Error::custom)?;
        raw.serialize(serializer)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn text(raw: i64) -> String {
        serde_json::to_string(&Num(Fixed::from_raw(raw))).expect("a number serialises")
    }

    #[test]
    fn numbers_come_out_as_exact_json_text() {
        assert_eq!(text(-500_000), "-0.5");
        assert_eq!(text(1), "0.000001");
        assert_eq!(text(12_000_000), "12");
        assert_eq!(text(0), "0");
        assert_eq!(text(333_333), "0.333333");
        assert_eq!(text(-160_681_400), "-160.6814");
    }

    #[test]
    fn numbers_nest_in_structures() {
        let value = serde_json::to_string(&[Num(Fixed::from_raw(250_000)), Num(Fixed::ONE)])
            .expect("a list serialises");
        assert_eq!(value, "[0.25,1]");
    }

    #[test]
    fn every_output_parses_back_as_json() {
        for raw in [i64::MIN + 1, -1, 0, 1, 999_999, 1_000_000, i64::MAX] {
            let json = text(raw);
            assert!(serde_json::from_str::<serde_json::Value>(&json).is_ok());
        }
    }
}
