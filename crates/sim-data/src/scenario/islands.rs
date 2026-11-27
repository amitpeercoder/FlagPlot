use serde::Deserialize;

/// An island (decision 0151, 1): a number, a name and its places in order.
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct IslandSpec {
    pub(super) number: u32,
    pub(super) name: String,
    pub(super) places: Vec<IslandPlaceSpec>,
}

/// One place of an island: exactly one of a base of the scenario and a coast
/// hex as `[q, r]`.
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct IslandPlaceSpec {
    pub(super) base: Option<u32>,
    pub(super) hex: Option<[i32; 2]>,
}
