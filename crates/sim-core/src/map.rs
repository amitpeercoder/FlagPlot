//! The hex map: what each hex is and what sites it holds.

use std::collections::BTreeMap;

use crate::hex::Hex;

/// What a hex is made of.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum HexKind {
    DeepSea,
    ShallowSea,
    Coast,
    Land,
}

impl HexKind {
    /// A stable number for the kind. Part of the state hash.
    pub const fn code(self) -> u8 {
        match self {
            HexKind::DeepSea => 1,
            HexKind::ShallowSea => 2,
            HexKind::Coast => 3,
            HexKind::Land => 4,
        }
    }
}

/// Ground cover on a hex that has land.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum Terrain {
    Clear,
    Jungle,
    Mountain,
    Atoll,
}

/// Reference data for one hex.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct HexData {
    pub kind: HexKind,
    pub terrain: Option<Terrain>,
    /// 0 means no port. Sizes run 1 to 10.
    pub port_size: u8,
    /// 0 means no airfield. Sizes run 1 to 10.
    pub airfield_size: u8,
    pub anchorage: bool,
}

impl HexData {
    pub const fn open_sea() -> HexData {
        HexData {
            kind: HexKind::DeepSea,
            terrain: None,
            port_size: 0,
            airfield_size: 0,
            anchorage: false,
        }
    }

    /// Whether ships can enter the hex.
    pub const fn is_navigable(&self) -> bool {
        matches!(
            self.kind,
            HexKind::DeepSea | HexKind::ShallowSea | HexKind::Coast
        )
    }
}

/// The whole map. Read-only once a game starts.
#[derive(Clone, PartialEq, Eq, Debug, Default)]
pub struct Map {
    hexes: BTreeMap<Hex, HexData>,
}

impl Map {
    pub fn new() -> Map {
        Map::default()
    }

    /// Adds a hex, replacing any earlier data for it.
    pub fn insert(&mut self, hex: Hex, data: HexData) {
        self.hexes.insert(hex, data);
    }

    pub fn get(&self, hex: Hex) -> Option<&HexData> {
        self.hexes.get(&hex)
    }

    pub fn contains(&self, hex: Hex) -> bool {
        self.hexes.contains_key(&hex)
    }

    /// True when the hex is on the map and ships can enter it.
    pub fn is_navigable(&self, hex: Hex) -> bool {
        self.get(hex).is_some_and(HexData::is_navigable)
    }

    pub fn len(&self) -> usize {
        self.hexes.len()
    }

    pub fn is_empty(&self) -> bool {
        self.hexes.is_empty()
    }

    /// All hexes in sorted order.
    pub fn iter(&self) -> impl Iterator<Item = (&Hex, &HexData)> {
        self.hexes.iter()
    }
}
