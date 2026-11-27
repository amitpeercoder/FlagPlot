//! A field of an event read by its type (decision 0235).
//!
//! The export of an event's `data` reads each field of a kind that is not
//! marked `[by_hand]` through [`ReadField`], once for each type. A type has a
//! reading only if it is read the same way wherever the export uses it.
//! `EventKind::fields` asks for the [`UmpireKey`], so only a tool that shows
//! the truth can call it.

use crate::fixed::Fixed;
use crate::hex::Hex;
use crate::ids::{EntityId, UmpireKey};

/// One field's value as plain data.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum FieldValue {
    /// A whole number.
    Int(i64),
    /// A quantity.
    Num(Fixed),
    /// A flag.
    Bool(bool),
    /// A hex.
    Hex(Hex),
}

/// A type that has one reading wherever an event holds it.
pub trait ReadField {
    /// The value, or `None` where the reading is "nothing" and the field is
    /// left out.
    fn read(&self, key: &UmpireKey) -> Option<FieldValue>;
}

impl ReadField for Fixed {
    fn read(&self, _key: &UmpireKey) -> Option<FieldValue> {
        Some(FieldValue::Num(*self))
    }
}

impl ReadField for u32 {
    fn read(&self, _key: &UmpireKey) -> Option<FieldValue> {
        Some(FieldValue::Int(i64::from(*self)))
    }
}

impl ReadField for u8 {
    fn read(&self, _key: &UmpireKey) -> Option<FieldValue> {
        Some(FieldValue::Int(i64::from(*self)))
    }
}

impl ReadField for bool {
    fn read(&self, _key: &UmpireKey) -> Option<FieldValue> {
        Some(FieldValue::Bool(*self))
    }
}

impl ReadField for Hex {
    fn read(&self, _key: &UmpireKey) -> Option<FieldValue> {
        Some(FieldValue::Hex(*self))
    }
}

/// An id whose number is public (a ship, a task force, a base and the like):
/// its number. The opaque ids (an order, an event, a contact) are not here.
impl<T: EntityId> ReadField for T {
    fn read(&self, _key: &UmpireKey) -> Option<FieldValue> {
        Some(FieldValue::Int(i64::from(self.raw())))
    }
}

/// What `EventKind::fields` gives: each field's key and value, in the order
/// declared.
pub type Fields = Vec<(&'static str, FieldValue)>;
