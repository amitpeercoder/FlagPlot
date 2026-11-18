//! The measure of a control-an-island intent (decision 0151, 10 and 11): how
//! many of the days the intent has been in force every named place of the
//! island held a base of the side with supply above its reserve and enough
//! days of supply, and, for each place, whether it is the side's and its days
//! of supply today.
//!
//! Worked from a [`SideView`] and nothing else: the side's own events (the
//! `BaseAtDayEnd` of each base it holds), its own bases, the island as the
//! map gives it and the intents in force. Nothing is stored: the same view
//! gives the same measure after a save and a resume.
//!
//! What the view cannot give is as in `hold`: a base the side has lost shows
//! none of its past days, so a place lost counts its earlier days as not
//! controlled. The use of supply is today's planned use in the hex, as the
//! view gives it, applied to every day (the past days' use is not in the
//! side's events), and a day is judged on the supply at its end.

use std::collections::{BTreeMap, BTreeSet};

use sim_core::command::{CommandState, IntentKind};
use sim_core::events::EventKind;
use sim_core::fixed::Fixed;
use sim_core::hex::Hex;
use sim_core::ids::{BaseId, IntentId};
use sim_core::side_events::SideEventKind;
use sim_core::view::SideView;

/// The floor, in days of supply, a place must hold on a day to count, where
/// no supply intent under the place's hold gives one (decision 0151, 10).
/// A placeholder.
pub const DEFAULT_FLOOR_DAYS: u32 = 5;

/// One named place of the island as the side stands in it today.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PlaceMeasure {
    pub hex: Hex,
    /// The base the island names at the place, if it names one.
    pub base: Option<BaseId>,
    /// Whether the side holds a base in the hex today.
    pub ours: bool,
    /// The side's days of supply in the hex today; `None` where it holds no
    /// base there or its planned use is nothing.
    pub days_of_supply: Option<Fixed>,
}

/// What a control-an-island intent has done so far.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct IslandMeasure {
    /// The days the intent has been in force: from the day it was set to the
    /// last day resolved, or to the day before it was withdrawn, inclusive.
    pub days_in_force: u32,
    /// The days of those on which every place was controlled.
    pub days_controlled: u32,
    /// The places, in the island's order.
    pub places: Vec<PlaceMeasure>,
}

/// The hex of a base, the side's own or an enemy base on its map.
fn hex_of(view: &SideView<'_>, base: BaseId) -> Option<Hex> {
    view.bases()
        .find(|b| b.id == base)
        .map(|b| b.hex)
        .or_else(|| view.enemy_bases().find(|b| b.id == base).map(|b| b.hex))
}

/// The floor in days for the place at `hex` under island intent `island`: the
/// `floor_days` of the lowest-numbered "keep supplied" in force at a base in
/// the hex whose parent is a "hold" in force of a base in that hex, standing
/// under the island intent; with none, the default.
fn floor_for(view: &SideView<'_>, state: &CommandState, island: IntentId, hex: Hex) -> u32 {
    for intent in state.intents_in_force() {
        let IntentKind::KeepSupplied {
            base, floor_days, ..
        } = intent.kind
        else {
            continue;
        };
        if hex_of(view, base) != Some(hex) {
            continue;
        }
        let Some(parent) = intent.parent.and_then(|p| state.intents.get(&p)) else {
            continue;
        };
        let IntentKind::Hold { base: held } = parent.kind else {
            continue;
        };
        if !parent.in_force() || hex_of(view, held) != Some(hex) {
            continue;
        }
        if state.ancestors(parent.id).iter().any(|a| a.id == island) {
            return floor_days;
        }
    }
    DEFAULT_FLOOR_DAYS
}

/// The measure of control-an-island intent `intent`, or `None` where it is no
/// intent of the side, not a control-an-island intent, or names an island the
/// map does not have.
pub fn island_measure(view: &SideView<'_>, intent: IntentId) -> Option<IslandMeasure> {
    let state = view.command_state();
    let found = state.intents.get(&intent)?;
    let IntentKind::ControlIsland { island, .. } = found.kind else {
        return None;
    };
    let island = view.island(island)?;
    let last = match found.withdrawn_day {
        Some(day) => day.saturating_sub(1).min(view.day()),
        None => view.day(),
    };
    let first = found.set_day;

    // The side's bases by hex, in base order: today's reserve of each.
    let mut reserve_at: BTreeMap<Hex, Fixed> = BTreeMap::new();
    let mut hex_of_own: BTreeMap<BaseId, Hex> = BTreeMap::new();
    for base in view.bases() {
        hex_of_own.insert(base.id, base.hex);
        *reserve_at.entry(base.hex).or_insert(Fixed::ZERO) += base.supply_reserve_t;
    }

    // The stock at the end of each day in each hex, summed over the side's
    // bases there, from the side's events; and the days on which a hex has
    // any such event.
    //
    // A side is given a base's day-end event only for a day on which the
    // base was its own, so `own_on` is the days each base was the side's. A
    // base named by a place is in its place's hex whoever holds it today.
    let named_hex: BTreeMap<BaseId, Hex> = island
        .places
        .iter()
        .filter_map(|place| place.base.map(|base| (base, place.hex)))
        .collect();
    let mut stock_at: BTreeMap<(u32, Hex), Fixed> = BTreeMap::new();
    let mut own_on: BTreeSet<(u32, BaseId)> = BTreeSet::new();
    for event in view.events() {
        if let SideEventKind::Whole(EventKind::BaseAtDayEnd { base, supply_t, .. }) = &event.kind {
            own_on.insert((event.day, *base));
            if let Some(hex) = hex_of_own.get(base).or_else(|| named_hex.get(base)) {
                *stock_at.entry((event.day, *hex)).or_insert(Fixed::ZERO) += *supply_t;
            }
        }
    }

    // Per place: its hex, today's use of supply there and the floor.
    let mut places = Vec::new();
    let mut rules: Vec<(Hex, Fixed, Fixed)> = Vec::new();
    let mut seen: BTreeSet<Hex> = BTreeSet::new();
    for place in &island.places {
        let today = view.days_of_supply(place.hex);
        places.push(PlaceMeasure {
            hex: place.hex,
            base: place.base,
            // A place that names a base is the side's when that base is: a
            // base formed by a landing beside the enemy's does not make it so.
            ours: match place.base {
                Some(base) => hex_of_own.contains_key(&base),
                None => hex_hold(view, place.hex),
            },
            days_of_supply: today.and_then(|figure| figure.days),
        });
        if seen.insert(place.hex) {
            let use_t = today.map_or(Fixed::ZERO, |figure| figure.planned_use.total_t());
            let floor = Fixed::from_int(i64::from(floor_for(view, state, intent, place.hex)));
            rules.push((place.hex, use_t, floor));
        }
    }

    let mut days_controlled = 0;
    if first <= last {
        for day in first..=last {
            let named_ours = island
                .places
                .iter()
                .filter_map(|place| place.base)
                .all(|base| own_on.contains(&(day, base)));
            let controlled = named_ours
                && rules.iter().all(|(hex, use_t, floor)| {
                    let Some(stock) = stock_at.get(&(day, *hex)) else {
                        return false;
                    };
                    let reserve = reserve_at.get(hex).copied().unwrap_or(Fixed::ZERO);
                    if *stock <= reserve {
                        return false;
                    }
                    // Stock over use at least the floor, as stock at least floor
                    // times use; with no use, the stock above the reserve is all
                    // that is asked.
                    use_t.is_zero() || *stock >= *floor * *use_t
                });
            if controlled {
                days_controlled += 1;
            }
        }
    }
    let days_in_force = if first <= last { last - first + 1 } else { 0 };
    Some(IslandMeasure {
        days_in_force,
        days_controlled,
        places,
    })
}

/// Whether the side holds a base in `hex` today.
fn hex_hold(view: &SideView<'_>, hex: Hex) -> bool {
    view.bases().any(|base| base.hex == hex)
}
