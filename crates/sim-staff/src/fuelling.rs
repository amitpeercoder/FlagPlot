//! Fuelling at sea as the staff reads it (decision 0201, rules 19 to 21).
//!
//! **The warning (19).** Each morning every force of the side that an intent
//! in force names (its task's force, or in its forces limit), with a ship afloat and no home port, one of whose
//! ships holds less than a quarter of her class's fuel capacity, is named in
//! the staff's report: a reason that serves no intent and gives no order.
//!
//! **An oiler's force is a place to fuel (20).** The carrier and stand
//! doctrines ask [`oiler_for`] where they would send a force home for fuel,
//! and give it the order to fuel from the oiler's force instead where the
//! passage to her present place is less than the passage home; a force with
//! no home port is given it when a ship is below a quarter. The oiler must
//! hold cargo fuel. While the order stands the staff gives the force no
//! route ([`fuelling_from`]). The staff places no oiler (21).

use sim_core::command::Reason;
use sim_core::fixed::Fixed;
use sim_core::hex::Hex;
use sim_core::ids::{ShipId, TaskForceId};
use sim_core::state::{ShipLocation, TaskForce};
use sim_core::view::SideView;

/// The share of her class's capacity below which a ship of a force with no
/// home port is warned of and, under a carrier or stand task, sent to an
/// oiler (decision 0201, 25). A figure of the doctrine, a placeholder.
pub fn quarter() -> Fixed {
    Fixed::from_ratio(1, 4)
}

/// The ships of `force` afloat below a quarter of their class's capacity,
/// in ship order, each with its fuel and its capacity.
pub(crate) fn below_a_quarter(
    view: &SideView<'_>,
    force: TaskForceId,
) -> Vec<(ShipId, Fixed, Fixed)> {
    crate::stand::afloat(view, force)
        .into_iter()
        .filter(|(_, fuel, capacity, _)| *fuel < *capacity * quarter())
        .map(|(ship, fuel, capacity, _)| (ship, fuel, capacity))
        .collect()
}

/// The words of the ships below a quarter.
pub(crate) fn below_words(below: &[(ShipId, Fixed, Fixed)]) -> String {
    below
        .iter()
        .map(|(ship, fuel, capacity)| format!("{ship} holds {fuel} t of {capacity}"))
        .collect::<Vec<_>>()
        .join(", ")
}

/// The warning of rule 19: `force`, with no home port, has ships below a
/// quarter of capacity, and no oiler's force holds cargo fuel for it. A
/// reason of the doctrine that serves the force's task, and gives no order.
pub(crate) fn warning(
    builder: crate::reasons::ReasonBuilder,
    force: &TaskForce,
    below: &[(ShipId, Fixed, Fixed)],
    intent_words: &str,
    today: u32,
) -> Reason {
    let id = force.id;
    let here = force.position();
    let ships = below_words(below);
    builder
        .evidence(format!("{id} has no home port, and {ships}"), today)
        .binding("a ship of a force with no home port below a quarter of her capacity")
        .worked(format!(
            "{intent_words}: {id}, at {here}, has no home port, and {ships}: below a quarter of capacity, with no oiler's force holding cargo fuel to send it to."
        ))
        .would_change("a home port set, an oiler's force with cargo fuel, or the ship fuelled")
        .done()
}

/// The ships of `force` afloat with the mark, in ship order.
fn oilers_of(view: &SideView<'_>, force: TaskForceId) -> Vec<ShipId> {
    let mut found: Vec<ShipId> = view
        .ships()
        .filter(|ship| ship.location == ShipLocation::TaskForce(force))
        .filter(|ship| view.ship_class(ship.class).is_some_and(|c| c.fuels_at_sea))
        .map(|ship| ship.id)
        .collect();
    found.sort();
    found
}

/// The cargo fuel the ships of `force` with the mark hold together.
fn cargo_fuel_of_oilers(view: &SideView<'_>, force: TaskForceId) -> Fixed {
    let oilers = oilers_of(view, force);
    view.ships()
        .filter(|ship| oilers.contains(&ship.id))
        .fold(Fixed::ZERO, |sum, ship| sum + ship.cargo_fuel_t)
}

/// An oiler's force a force may be sent to (decision 0201, 20).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) struct OilerForce {
    pub giver: TaskForceId,
    pub hex: Hex,
    /// The miles of the passage from the force to her, by the engine's
    /// charge of the sea route.
    pub nm: u32,
    pub cargo_fuel_t: Fixed,
}

/// The nearest force of the side by the sea route from `force` whose ships
/// with the mark hold cargo fuel, and that the order to fuel from could
/// name: not the force itself, not herself fuelling, not an escort and with
/// no escort. The lower number on a tie.
pub(crate) fn oiler_for(view: &SideView<'_>, force: &TaskForce) -> Option<OilerForce> {
    let here = force.position();
    let escorted: Vec<TaskForceId> = view
        .task_forces()
        .filter_map(|other| other.charge.map(|c| c.force))
        .collect();
    let mut found: Vec<OilerForce> = view
        .task_forces()
        .filter(|other| other.id != force.id && !other.ships.is_empty())
        .filter(|other| other.fuel_from.is_none() && other.charge.is_none())
        .filter(|other| !escorted.contains(&other.id))
        .filter_map(|other| {
            let cargo_fuel_t = cargo_fuel_of_oilers(view, other.id);
            if cargo_fuel_t <= Fixed::ZERO {
                return None;
            }
            let hex = other.position();
            let nm = view.sea_charged_nm(here, hex)?;
            Some(OilerForce {
                giver: other.id,
                hex,
                nm,
                cargo_fuel_t,
            })
        })
        .collect();
    found.sort_by_key(|o| (o.nm, o.giver));
    found.first().copied()
}

/// The giver of `force`'s order to fuel at sea, while it stands.
pub(crate) fn fuelling_from(force: &TaskForce) -> Option<TaskForceId> {
    force.fuel_from.map(|f| f.giver)
}
