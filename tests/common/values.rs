//! Small helpers every scenario test used to define for itself: quantities,
//! ids, hexes, the four-sigma tolerance, the words of a reason and a few
//! pieces of scenario text. A scenario test takes them from here and does
//! not define them again (decision 0233, item 5).

use sim_core::command::Reason;
use sim_core::fixed::Fixed;
use sim_core::hex::Hex;
use sim_core::ids::{AirGroupId, BaseId, IntentId, LandUnitId, ShipId, TaskForceId};

/// A quantity from a decimal string.
pub fn fx(text: &str) -> Fixed {
    text.parse().expect("a decimal")
}

/// A whole number as a quantity.
pub fn n(whole: i64) -> Fixed {
    Fixed::from_int(whole)
}

/// `value` hundredths.
pub fn hundredths(value: i64) -> Fixed {
    Fixed::from_ratio(value, 100)
}

/// `value` millionths: the raw form.
pub fn millionths(value: i64) -> Fixed {
    Fixed::from_raw(value)
}

/// A hex by its coordinates.
pub fn hex(q: i32, r: i32) -> Hex {
    Hex { q, r }
}

pub fn ship(raw: u32) -> ShipId {
    ShipId::new(raw)
}

pub fn force(raw: u32) -> TaskForceId {
    TaskForceId::new(raw)
}

pub fn group(raw: u32) -> AirGroupId {
    AirGroupId::new(raw)
}

pub fn base(raw: u32) -> BaseId {
    BaseId::new(raw)
}

pub fn unit(raw: u32) -> LandUnitId {
    LandUnitId::new(raw)
}

pub fn intent(raw: u32) -> IntentId {
    IntentId::new(raw)
}

/// Whether k out of n is within four standard deviations of probability
/// `p` (in millionths), in exact integer arithmetic.
pub fn within_four_sigma(k: i64, n: i64, p: i64) -> bool {
    let (k, n, p) = (i128::from(k), i128::from(n), i128::from(p));
    let scale = 1_000_000;
    let off = k * scale - n * p;
    off * off <= 16 * n * p * (scale - p)
}

/// A reason as one text: "worked | binding | evidence".
pub fn text_of(reason: &Reason) -> String {
    format!(
        "{} | {} | {}",
        reason.worked,
        reason.binding.join("; "),
        reason
            .evidence
            .iter()
            .map(|e| e.what.clone())
            .collect::<Vec<_>>()
            .join("; ")
    )
}

pub fn assert_says(all: &str, parts: &[&str]) {
    for part in parts {
        assert!(all.contains(part), "the reason says {part:?}: {all}");
    }
}

/// Adds `what` at the mark `# MARK: name` of the scenario's text; the mark
/// stays, so that more can be added.
pub fn at(text: &str, name: &str, what: &str) -> String {
    let mark = format!("# MARK: {name}\n");
    super::replace_once(text, &mark, &format!("{what}{mark}"))
}

/// A command table of the Allied side.
pub fn command_table(day: u32, body: &str) -> String {
    format!("\n[[command]]\nday = {day}\nside = \"allied\"\n{body}\n")
}
