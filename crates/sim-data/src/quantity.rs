use serde::Deserialize;
use sim_core::fixed::Fixed;

/// A quantity written as a whole number or a decimal string. A TOML float
/// matches neither form and is rejected.
#[derive(Deserialize)]
#[serde(untagged)]
pub(crate) enum Quantity {
    Whole(i64),
    Decimal(String),
}

impl Quantity {
    pub(crate) fn to_fixed(&self) -> Result<Fixed, String> {
        match self {
            Quantity::Whole(n) => Ok(Fixed::from_int(*n)),
            Quantity::Decimal(text) => text
                .parse::<Fixed>()
                .map_err(|e| format!("\"{text}\" is invalid: {e}")),
        }
    }
}
