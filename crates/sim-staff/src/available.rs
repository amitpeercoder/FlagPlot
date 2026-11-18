//! A day from which a unit may be used (decision 0140, 3): the one place the
//! staff asks whether it may use a unit of its side, and the one wording of
//! the day in a reason. Every doctrine that chooses units to order reads the
//! day through here, which reads [`SideView::may_use`]; the staff leaves out
//! a unit before its day and a reason names the day.
//!
//! The staff still reads a unit before its day where it reads its own side
//! for anything but choosing what to order (decision 0140, 4): it is there,
//! it can be seen and attacked. Only a choice of what to order calls this.

use sim_core::ids::{AirGroupId, EntityRef, ShipId, TaskForceId};
use sim_core::view::SideView;

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
    fn a_list_names_each_unit_with_its_day() {
        let list = [(ShipId::new(2), 5), (ShipId::new(3), 7)];
        assert_eq!(
            list_words(&list),
            "ship:2 may be used from day 5, ship:3 may be used from day 7"
        );
        assert_eq!(list_words::<ShipId>(&[]), "");
    }
}
