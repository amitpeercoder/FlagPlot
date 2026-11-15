//! The route of a run: the sea path the engine would take from the source to
//! the destination and back, kept inside the intent's area where one is set
//! (decision 0107, 41). Where the engine's own path leaves the area,
//! waypoints are added along the shortest path inside it until the engine's
//! path between each pair stays in; where none does, there is no route in
//! the area and the staff says so.

use std::collections::{BTreeMap, BTreeSet, VecDeque};

use sim_core::hex::Hex;
use sim_core::view::SideView;

/// The waypoints out and back, and the miles each way the engine charges.
#[derive(Clone, Debug)]
pub(crate) struct RouteInfo {
    /// Waypoints from the source to the destination, the last the
    /// destination.
    pub(crate) out: Vec<Hex>,
    /// Waypoints from the destination to the source, the last the source.
    pub(crate) back: Vec<Hex>,
    pub(crate) out_nm: u32,
    pub(crate) back_nm: u32,
    /// Whether the route stays inside the area. `false` only for the route
    /// the engine would take, kept to time a run that cannot be made.
    pub(crate) in_area: bool,
}

/// The route between the two hexes, or `None` where ships cannot go from one
/// to the other at all. Where an area is set and no route lies inside it,
/// the route the engine would take is given with `in_area` false.
pub(crate) fn route_between(
    view: &SideView<'_>,
    source: Hex,
    destination: Hex,
    area: Option<&BTreeSet<Hex>>,
) -> Option<RouteInfo> {
    // Ships must be able to go both ways at all.
    view.sea_path(source, destination)?;
    view.sea_path(destination, source)?;
    let inside = match area {
        None => Some((vec![destination], vec![source])),
        Some(area) => legs_inside(view, source, destination, area).zip(legs_inside(
            view,
            destination,
            source,
            area,
        )),
    };
    let (out, back, in_area) = match inside {
        Some((out, back)) => (out, back, true),
        None => (vec![destination], vec![source], false),
    };
    let out_nm = leg_nm(view, source, &out)?;
    let back_nm = leg_nm(view, destination, &back)?;
    Some(RouteInfo {
        out,
        back,
        out_nm,
        back_nm,
        in_area,
    })
}

/// The miles the engine charges to steam from `start` through `waypoints`
/// in turn.
fn leg_nm(view: &SideView<'_>, start: Hex, waypoints: &[Hex]) -> Option<u32> {
    let mut here = start;
    let mut miles = 0;
    for waypoint in waypoints {
        miles += view.sea_charged_nm(here, *waypoint)?;
        here = *waypoint;
    }
    Some(miles)
}

/// Whether every hex of the engine's path from `from` to `to` is in `area`.
fn path_stays_in(view: &SideView<'_>, from: Hex, to: Hex, area: &BTreeSet<Hex>) -> bool {
    view.sea_path(from, to)
        .is_some_and(|path| path.iter().all(|hex| area.contains(hex)))
}

/// Waypoints from `from` to `to` that keep the engine's path inside `area`:
/// just `to` where its own path does, else points along the shortest path
/// of single steps inside the area. `None` where there is none.
fn legs_inside(view: &SideView<'_>, from: Hex, to: Hex, area: &BTreeSet<Hex>) -> Option<Vec<Hex>> {
    if !area.contains(&from) || !area.contains(&to) {
        return None;
    }
    if path_stays_in(view, from, to, area) {
        return Some(vec![to]);
    }
    // The shortest path of single steps inside the area.
    let mut parent: BTreeMap<Hex, Hex> = BTreeMap::new();
    let mut queue = VecDeque::from([from]);
    while let Some(here) = queue.pop_front() {
        if here == to {
            break;
        }
        for next in here.neighbors() {
            if next == from || !area.contains(&next) || parent.contains_key(&next) {
                continue;
            }
            if view.sea_path(here, next) == Some(vec![next]) {
                parent.insert(next, here);
                queue.push_back(next);
            }
        }
    }
    parent.get(&to)?;
    let mut steps = vec![to];
    let mut here = to;
    while let Some(before) = parent.get(&here) {
        if *before == from {
            break;
        }
        steps.push(*before);
        here = *before;
    }
    steps.reverse();
    // Fewest waypoints: from each one, the farthest step the engine can
    // reach without leaving the area.
    let mut waypoints = Vec::new();
    let mut here = from;
    let mut index = 0;
    while index < steps.len() {
        let mut reach = index;
        for candidate in (index..steps.len()).rev() {
            if path_stays_in(view, here, steps[candidate], area) {
                reach = candidate;
                break;
            }
        }
        waypoints.push(steps[reach]);
        here = steps[reach];
        index = reach + 1;
    }
    Some(waypoints)
}
