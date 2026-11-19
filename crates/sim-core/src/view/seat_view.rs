//! A seat at a command (decision 0138, item 5).
//!
//! A seat at a task force or at a base is given what that command holds and
//! nothing else of its side's. [`SideView::at_seat`] hands a view of the same
//! side that is cut to the command: its own-unit lists (ships, task forces,
//! bases, air groups, land units, orders, replacement streams and arrivals),
//! its events and its picture are the command's, so that anything built from
//! a view (the agent's briefing, lists, details, distances and explanations)
//! can say nothing of any other command of the side. The cut is made here and
//! in the methods of [`SideView`] that read these lists; no tool has a way
//! round it.
//!
//! What a command holds:
//!
//! * a task force: itself, its ships (afloat, sunk, or away at a yard to come
//!   back to it), the air groups aboard them, and the land units whose men
//!   are aboard them;
//! * a base: itself, the ships in its port, the air groups at it or aboard
//!   those ships, the land units ashore in its hex (decision 0134, 8) or
//!   aboard those ships, and its replacement streams;
//! * its own picture ([`SideView::command`]): contacts, claims and estimates;
//! * its own events: the side's events that name one of its own units and no
//!   other unit of the side, and the claims and "nothing there" reports its
//!   own units' strikes caused. The reports a command is sent, and the
//!   passing of reports between commands, are not events a command sees:
//!   an event of a report is the side's, with the side's contact number and
//!   whether the report was new to the side, and a command's picture holds
//!   only the contact (decision 0135, 20).
//!
//! The public facts every side has stay: the map, every base's name, place
//! and side ([`SideView::enemy_bases`]), the figures.
//!
//! Reads: the side's own state, the picture of the seat's command and the
//! side's own events. Reads nothing of the enemy. Writes nothing.

use std::collections::BTreeMap;
use std::sync::OnceLock;

use crate::belief::BeliefState;
use crate::command::CommandState;
use crate::events::EventKind;
use crate::ids::EntityRef;
use crate::orders::Order;
use crate::pictures::{self, CommandRef};
use crate::side_events::{SideCause, SideEvent, SideEventKind};
use crate::state::{AirGroup, Base, LandUnit, Replacement, Ship, ShipLocation, TaskForce};

use super::{CommandView, SideView};

static BLANK_COMMAND_STATE: OnceLock<CommandState> = OnceLock::new();

/// The command state a seat at a command is given: none of the side's.
pub(super) fn blank_command_state() -> &'static CommandState {
    BLANK_COMMAND_STATE.get_or_init(CommandState::default)
}

impl<'a> SideView<'a> {
    /// A view of the side cut to one of its commands, for a seat at a task
    /// force or at a base (decision 0138, 5). `None` for the area command
    /// (the area seat is not a cut: [`SideView::known_to`]), for a command
    /// that is not one of the side's, with command levels off (there is no
    /// command then), and for a view that is cut already.
    pub fn at_seat(&self, command: CommandRef) -> Option<SideView<'a>> {
        if self.seat.is_some() || command == CommandRef::Area {
            return None;
        }
        self.commands?.get(&command)?;
        Some(SideView {
            seat: Some(command),
            held: None,
            known_to: None,
            side_picture: false,
            ..*self
        })
    }

    /// The command this view is cut to: `None` for the side's own view and
    /// for the area seat.
    pub fn seat(&self) -> Option<CommandRef> {
        self.seat
    }

    /// This view with its seat picture ([`SideView::seat_picture`]) the
    /// side's own belief, every report at once: the easier setting at the
    /// area seat (decision 0119, answer 5). The side's own belief is the
    /// side's, never the enemy's. A view cut to a command is returned as it
    /// is.
    pub fn with_side_picture(&self) -> SideView<'a> {
        if self.seat.is_some() {
            return *self;
        }
        SideView {
            side_picture: true,
            ..*self
        }
    }

    /// Whether every question of a seat is answered from one picture, the
    /// same for every unit: a seat at a command, or the easier setting.
    pub fn has_one_picture(&self) -> bool {
        self.seat.is_some() || self.side_picture
    }

    /// The picture a seat is shown: its command's at a task force or a base,
    /// the side's own belief (every report at once) under the easier setting,
    /// else the area command's.
    pub fn seat_picture(&self) -> CommandView<'a> {
        if let Some(seat) = self.seat {
            return self.command(seat).expect("a seat holds its own command");
        }
        if self.side_picture {
            return CommandView::of_side_belief(self.side, self.belief);
        }
        self.command(CommandRef::Area)
            .expect("a side always has its area command")
    }

    /// The belief the list readers of the view read: the seat's command's
    /// picture at a seat, else the side's own.
    pub(super) fn own_belief(&self) -> &'a BeliefState {
        match self.seat {
            Some(seat) => self.picture(seat),
            None => self.belief,
        }
    }

    pub(super) fn owns_force(&self, force: &TaskForce) -> bool {
        force.side == self.side
            && match self.seat {
                None => true,
                Some(CommandRef::TaskForce(own)) => force.id == own,
                Some(_) => false,
            }
    }

    pub(super) fn owns_base(&self, base: &Base) -> bool {
        base.side == self.side
            && match self.seat {
                None => true,
                Some(CommandRef::Base(own)) => base.id == own,
                Some(_) => false,
            }
    }

    pub(super) fn owns_ship(&self, ship: &Ship) -> bool {
        ship.side == self.side
            && match self.seat {
                None => true,
                Some(CommandRef::TaskForce(force)) => {
                    matches!(
                        ship.location,
                        ShipLocation::TaskForce(at) | ShipLocation::Yard(at) if at == force
                    ) || self
                        .truth
                        .task_forces
                        .get(&force)
                        .is_some_and(|record| record.ships.contains(&ship.id))
                }
                Some(CommandRef::Base(base)) => ship.location == ShipLocation::Port(base),
                Some(CommandRef::Area) => true,
            }
    }

    pub(super) fn owns_group(&self, group: &AirGroup) -> bool {
        group.side == self.side
            && match self.seat {
                None => true,
                Some(seat) => pictures::command_of_group(self.truth, group.id) == Some(seat),
            }
    }

    pub(super) fn owns_land_unit(&self, unit: &LandUnit) -> bool {
        unit.side == self.side
            && match self.seat {
                None => true,
                Some(seat) => {
                    pictures::command_of_land_unit(self.truth, unit.id) == Some(seat)
                        || self.truth.ships.values().any(|ship| {
                            ship.troops.is_some_and(|troops| troops.unit == unit.id)
                                && self.owns_ship(ship)
                        })
                }
            }
    }

    pub(super) fn owns_stream(&self, stream: &Replacement) -> bool {
        stream.side == self.side
            && match self.seat {
                None => true,
                Some(CommandRef::Base(own)) => stream.base == own,
                Some(_) => false,
            }
    }

    /// Whether `order` is addressed to a unit the view owns.
    pub(super) fn owns_order(&self, order: &Order) -> bool {
        if self.seat.is_none() {
            return true;
        }
        match order.kind.target() {
            EntityRef::TaskForce(id) => self
                .truth
                .task_forces
                .get(&id)
                .is_some_and(|record| self.owns_force(record)),
            EntityRef::AirGroup(id) => self
                .truth
                .air_groups
                .get(&id)
                .is_some_and(|record| self.owns_group(record)),
            EntityRef::LandUnit(id) => self
                .truth
                .land_units
                .get(&id)
                .is_some_and(|record| self.owns_land_unit(record)),
            EntityRef::Base(id) => self
                .truth
                .bases
                .get(&id)
                .is_some_and(|record| self.owns_base(record)),
            _ => false,
        }
    }

    /// Whether a base is one of the public facts of the map for this view:
    /// every base on the map the view does not own.
    pub(super) fn is_public_base(&self, base: &Base) -> bool {
        base.on_map
            && match self.seat {
                None => base.side != self.side,
                Some(_) => !self.owns_base(base),
            }
    }

    /// The men of one of the view's land units aboard the view's ships.
    pub(super) fn men_aboard_in_view(&self, unit: crate::ids::LandUnitId) -> u32 {
        if self.seat.is_none() {
            return self.truth.men_aboard(unit);
        }
        self.ships()
            .filter_map(|ship| ship.troops)
            .filter(|troops| troops.unit == unit)
            .map(|troops| troops.men)
            .sum()
    }

    /// Whether an event names a unit of the seat and no unit of the side's
    /// that is not (a base is public: it counts as none). `None` for an
    /// event that names a pilot, which is nobody's here.
    fn names_only_seat_units(&self, kind: &SideEventKind) -> bool {
        let mut own = false;
        for entity in kind.entities() {
            match entity {
                EntityRef::Ship(id) => match self.truth.ships.get(&id) {
                    Some(ship) if self.owns_ship(ship) => own = true,
                    _ => return false,
                },
                EntityRef::TaskForce(id) => match self.truth.task_forces.get(&id) {
                    Some(force) if self.owns_force(force) => own = true,
                    _ => return false,
                },
                EntityRef::AirGroup(id) => match self.truth.air_groups.get(&id) {
                    Some(group) if self.owns_group(group) => own = true,
                    _ => return false,
                },
                EntityRef::LandUnit(id) => match self.truth.land_units.get(&id) {
                    Some(unit) if self.owns_land_unit(unit) => own = true,
                    _ => return false,
                },
                EntityRef::Base(id) => {
                    if self
                        .truth
                        .bases
                        .get(&id)
                        .is_some_and(|base| self.owns_base(base))
                    {
                        own = true;
                    }
                }
                EntityRef::Contact(_) | EntityRef::Event(_) => {}
                EntityRef::Pilot(_) | EntityRef::Place(_) => return false,
            }
        }
        own
    }

    /// The side's events cut to the seat's command, renumbered 1, 2, 3...
    /// (a gap in the side's numbers would tell the seat that the side had an
    /// event it does not see). A cause that is cut is dropped, and the event
    /// then says it may have causes it does not show. An order is named by its
    /// place among the orders the seat is shown.
    pub(super) fn cut_events(&self, all: Vec<SideEvent>) -> Vec<SideEvent> {
        // The side's own order numbers, as the events carry them, and the
        // seat's, as the cut list of orders numbers them.
        let mut order_numbers: BTreeMap<u32, u32> = BTreeMap::new();
        let mut shown = 0u32;
        for (place, order) in self
            .orders
            .values()
            .filter(|order| order.side == self.side)
            .enumerate()
        {
            if self.owns_order(order) {
                shown += 1;
                let side_number = u32::try_from(place + 1).expect("few orders");
                order_numbers.insert(side_number, shown);
            }
        }
        let mut kept: BTreeMap<u32, u32> = BTreeMap::new();
        let mut out: Vec<SideEvent> = Vec::new();
        for event in all {
            let keep = match &event.kind {
                // The side's, not a command's: see the module's note.
                SideEventKind::Whole(
                    EventKind::ReportReceived { .. }
                    | EventKind::GroundReportReceived { .. }
                    | EventKind::ContactAged { .. }
                    | EventKind::ContactDropped { .. }
                    | EventKind::ReportSent { .. }
                    | EventKind::StateReportLate { .. }
                    | EventKind::StateReportLost { .. },
                ) => false,
                // What the seat's own units' strikes reported: the contact is
                // the number of the picture of the group's command, and the
                // event is caused by the strike.
                SideEventKind::Whole(
                    EventKind::ContactNotThere { .. } | EventKind::ClaimReceived { .. },
                ) => event
                    .causes
                    .iter()
                    .any(|cause| matches!(cause, SideCause::Event(n) if kept.contains_key(n))),
                other => self.names_only_seat_units(other),
            };
            if !keep {
                continue;
            }
            let number = u32::try_from(out.len() + 1).expect("few events");
            kept.insert(event.number, number);
            let mut cut = false;
            let mut causes = Vec::new();
            for cause in &event.causes {
                let mapped = match cause {
                    SideCause::Event(n) => kept.get(n).map(|m| SideCause::Event(*m)),
                    SideCause::Order(n) => order_numbers.get(n).map(|m| SideCause::Order(*m)),
                };
                match mapped {
                    Some(cause) => causes.push(cause),
                    None => cut = true,
                }
            }
            out.push(SideEvent {
                number,
                causes,
                hidden_causes: event.hidden_causes || cut,
                order_number: event
                    .order_number
                    .and_then(|n| order_numbers.get(&n).copied()),
                ..event
            });
        }
        out
    }
}
