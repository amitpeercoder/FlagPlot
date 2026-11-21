//! A day from which a unit may be used (decision 0140, 3): the one place the
//! staff asks whether it may use a unit of its side, and the one wording of
//! the day in a reason. Every doctrine that chooses units to order reads the
//! day through here, which reads [`SideView::may_use`]; the staff leaves out
//! a unit before its day and a reason names the day.
//!
//! The staff still reads a unit before its day where it reads its own side
//! for anything but choosing what to order (decision 0140, 4): it is there,
//! it can be seen and attacked. Only a choice of what to order calls this.

use sim_core::command::ForcesLimit;
use sim_core::ids::{AirGroupId, EntityRef, ShipId, TaskForceId};
use sim_core::orders::Mission;
use sim_core::state::{LandUnit, Ship, ShipLocation, TaskForce};
use sim_core::view::SideView;

/// What a take counts as its to use, whether now or later (decision 0159,
/// rule B, item 7): the task forces and the land units its forces limit
/// names; of a kind the limit does not name, and with no limit, every
/// transport force and every fighting unit of the side. Each in number
/// order. A force or a land unit held apart until its day (decision 0195,
/// 13) is counted too, as a unit that may be used from its day: it is read
/// from the entry that holds it, and so is in no pool of units on the map.
/// The one notion: the take reads it where it names the units it may use
/// later and not today, and a trial's window reads it for the days on which
/// something the course may use first becomes usable (decision 0177,
/// amended 3). A force whose every ship has the mark that she fuels others
/// at sea is a force of oilers and is not counted where the limit names no
/// task force (decision 0206, rule 21).
#[derive(Clone, Debug, Default)]
pub struct Counted<'a> {
    pub task_forces: Vec<&'a TaskForce>,
    pub land_units: Vec<&'a LandUnit>,
}

/// [`Counted`] for a take whose forces limit is `limit`.
pub fn counted_by_a_take<'a>(view: &SideView<'a>, limit: Option<&ForcesLimit>) -> Counted<'a> {
    let named_forces = limit.filter(|f| !f.task_forces.is_empty());
    let named_units = limit.filter(|f| !f.land_units.is_empty());
    let force_counts = |force: &TaskForce, oilers: bool| {
        named_forces.map_or(force.mission == Mission::Transport && !oilers, |f| {
            f.task_forces.contains(&force.id)
        })
    };
    let on_map = |force: &TaskForce| {
        oilers_only(
            view,
            view.ships()
                .filter(|ship| ship.location == ShipLocation::TaskForce(force.id)),
        )
    };
    let unit_counts = |unit: &LandUnit| {
        !unit.working && named_units.is_none_or(|f| f.land_units.contains(&unit.id))
    };
    let mut task_forces: Vec<&'a TaskForce> = view
        .task_forces()
        .filter(|f| force_counts(f, on_map(f)))
        .collect();
    let mut land_units: Vec<&'a LandUnit> = view.land_units().filter(|u| unit_counts(u)).collect();
    // What is held apart for the side, from its entries (decision 0195, 13).
    for entry in view.entering() {
        let oilers = oilers_only(view, entry.ships.values());
        if let Some(force) = entry
            .task_force
            .as_ref()
            .filter(|f| force_counts(f, oilers))
        {
            task_forces.push(force);
        }
        land_units.extend(entry.land_units.values().filter(|u| unit_counts(u)));
    }
    task_forces.sort_by_key(|f| f.id);
    land_units.sort_by_key(|u| u.id);
    Counted {
        task_forces,
        land_units,
    }
}

/// Whether `ships`, the ships of one force, are a force of oilers: at least
/// one, and every one of a class with the mark that she fuels others at sea
/// (decision 0206, rule 21). A force with no ship is not.
fn oilers_only<'s>(view: &SideView<'_>, ships: impl Iterator<Item = &'s Ship>) -> bool {
    let mut any = false;
    for ship in ships {
        if !view.ship_class(ship.class).is_some_and(|c| c.fuels_at_sea) {
            return false;
        }
        any = true;
    }
    any
}

/// The day from which `unit` may be used, where the side may not use it yet
/// on the day it gives orders for: `Some(day)`. `None` where it may be used,
/// has no day, or is not the side's.
pub(crate) fn not_before(view: &SideView<'_>, unit: EntityRef) -> Option<u32> {
    if view.may_use(unit) {
        None
    } else {
        view.available_from(unit)
    }
}

/// The day from which `unit` may be used, where the side may not use it yet
/// on `day`: `Some(from)` with `from` after `day`. `None` where it may be used
/// by then, has no day, or is not the side's. With `day` the day orders are
/// given for it is [`not_before`] (decision 0159, 4: a take works the first
/// day that brings the men it wants).
pub(crate) fn not_before_on(view: &SideView<'_>, unit: EntityRef, day: u32) -> Option<u32> {
    view.available_from(unit).filter(|from| *from > day)
}

/// Whether the side may use the air group on the day it gives orders for.
pub(crate) fn group_may_be_used(view: &SideView<'_>, group: AirGroupId) -> bool {
    view.may_use(EntityRef::AirGroup(group))
}

/// The words for the day: "may be used from day 5". Every reason that
/// names a unit left out for its day uses this.
pub(crate) fn from_day_words(day: u32) -> String {
    format!("may be used from day {day}")
}

/// The day a unit held apart for the side enters (decision 0195, 13): the
/// day from which it may be used, which is no earlier than its entry's day.
/// `None` for a unit on the map and for a unit that is not the side's.
pub(crate) fn entering_on(view: &SideView<'_>, unit: EntityRef) -> Option<u32> {
    let entry = view.entry_of(unit)?;
    Some(view.available_from(unit).unwrap_or(entry.day))
}

/// The words for a unit held apart: "task_force:5 enters on day 3". Every
/// reason that names a force or a unit not yet on the map uses this
/// (decision 0195, 13).
pub(crate) fn enters_words(unit: impl std::fmt::Display, day: u32) -> String {
    format!("{unit} enters on day {day}")
}

/// A unit with its day: "ship:2 may be used from day 5".
pub(crate) fn named_with_day(unit: impl std::fmt::Display, day: u32) -> String {
    format!("{unit} {}", from_day_words(day))
}

/// The ships of `ships` the side may not use yet, each with its day, in ship
/// order.
pub(crate) fn ships_waiting(
    view: &SideView<'_>,
    ships: impl IntoIterator<Item = ShipId>,
) -> Vec<(ShipId, u32)> {
    let mut found: Vec<(ShipId, u32)> = ships
        .into_iter()
        .filter_map(|ship| Some((ship, not_before(view, EntityRef::Ship(ship))?)))
        .collect();
    found.sort_unstable();
    found
}

/// The task force with its day, where the side may not use it yet.
pub(crate) fn force_waiting(view: &SideView<'_>, force: TaskForceId) -> Option<u32> {
    not_before(view, EntityRef::TaskForce(force))
}

/// The groups of `groups` the side may not use yet, each with its day, in
/// group order.
pub(crate) fn groups_waiting(
    view: &SideView<'_>,
    groups: impl IntoIterator<Item = AirGroupId>,
) -> Vec<(AirGroupId, u32)> {
    let mut found: Vec<(AirGroupId, u32)> = groups
        .into_iter()
        .filter_map(|group| Some((group, not_before(view, EntityRef::AirGroup(group))?)))
        .collect();
    found.sort_unstable();
    found
}

/// A list of units with their days as one phrase: "ship:2 may be used from
/// day 5, ship:3 may be used from day 7".
pub(crate) fn list_words<T: std::fmt::Display + Copy>(waiting: &[(T, u32)]) -> String {
    waiting
        .iter()
        .map(|(unit, day)| named_with_day(unit, *day))
        .collect::<Vec<_>>()
        .join(", ")
}

impl crate::air::Day<'_, '_> {
    /// The day from which `unit` may be used where it may not be used on
    /// the day a choice of units reads ([`crate::air::Day::as_of`]): the day
    /// orders are given for, unless a take is working a later day.
    pub(crate) fn not_before_choice(&self, unit: EntityRef) -> Option<u32> {
        match self.as_of.get() {
            None => not_before(self.view, unit),
            Some(day) => not_before_on(self.view, unit, day),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_day_is_worded_one_way() {
        assert_eq!(from_day_words(5), "may be used from day 5");
        assert_eq!(
            named_with_day(ShipId::new(2), 5),
            "ship:2 may be used from day 5"
        );
    }

    #[test]
    fn a_unit_held_apart_is_worded_with_its_day() {
        assert_eq!(
            enters_words(TaskForceId::new(5), 3),
            "task_force:5 enters on day 3"
        );
    }

    #[test]
    fn a_list_names_each_unit_with_its_day() {
        let list = [(ShipId::new(2), 5), (ShipId::new(3), 7)];
        assert_eq!(
            list_words(&list),
            "ship:2 may be used from day 5, ship:3 may be used from day 7"
        );
        assert_eq!(list_words::<ShipId>(&[]), "");
    }
}
