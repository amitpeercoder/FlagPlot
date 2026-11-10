//! Ordered objects of mixed fields, for the `data` of an event and for the
//! systems of a ship. Keys come out in the order they are pushed, so the
//! output is stable.

use serde::ser::{SerializeMap, SerializeSeq};
use serde::{Serialize, Serializer};
use sim_core::fixed::Fixed;
use sim_core::hex::Hex;
use sim_core::ids::EntityId;

use super::num::Num;

/// One value in a [`Data`] object.
pub enum Field {
    Int(i64),
    Num(Fixed),
    Bool(bool),
    Text(&'static str),
    Str(String),
    /// A hex as `[q, r]`.
    Hex(Hex),
    Hexes(Vec<Hex>),
    Ids(Vec<u32>),
    Obj(Data),
}

impl Serialize for Field {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Field::Int(n) => serializer.serialize_i64(*n),
            Field::Num(n) => Num(*n).serialize(serializer),
            Field::Bool(b) => serializer.serialize_bool(*b),
            Field::Text(t) => serializer.serialize_str(t),
            Field::Str(t) => serializer.serialize_str(t),
            Field::Hex(h) => (h.q, h.r).serialize(serializer),
            Field::Hexes(hexes) => {
                let mut seq = serializer.serialize_seq(Some(hexes.len()))?;
                for h in hexes {
                    seq.serialize_element(&(h.q, h.r))?;
                }
                seq.end()
            }
            Field::Ids(ids) => ids.serialize(serializer),
            Field::Obj(data) => data.serialize(serializer),
        }
    }
}

/// An object whose keys keep the order they were added in.
#[derive(Default)]
pub struct Data(Vec<(&'static str, Field)>);

impl Data {
    pub fn new() -> Data {
        Data::default()
    }

    pub fn with(mut self, key: &'static str, field: Field) -> Data {
        self.0.push((key, field));
        self
    }

    pub fn int(self, key: &'static str, n: u32) -> Data {
        self.with(key, Field::Int(i64::from(n)))
    }

    pub fn id<T: EntityId>(self, key: &'static str, id: T) -> Data {
        self.int(key, id.raw())
    }

    pub fn ids<T: EntityId>(self, key: &'static str, ids: &[T]) -> Data {
        self.with(key, Field::Ids(ids.iter().map(|id| id.raw()).collect()))
    }

    pub fn num(self, key: &'static str, n: Fixed) -> Data {
        self.with(key, Field::Num(n))
    }

    pub fn flag(self, key: &'static str, b: bool) -> Data {
        self.with(key, Field::Bool(b))
    }

    pub fn text(self, key: &'static str, t: &'static str) -> Data {
        self.with(key, Field::Text(t))
    }

    pub fn hex(self, key: &'static str, h: Hex) -> Data {
        self.with(key, Field::Hex(h))
    }
}

impl Serialize for Data {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(Some(self.0.len()))?;
        for (key, field) in &self.0 {
            map.serialize_entry(key, field)?;
        }
        map.end()
    }
}
