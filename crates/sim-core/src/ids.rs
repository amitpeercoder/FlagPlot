//! Stable entity identifiers.
//!
//! Every entity kind has its own ID type, so a ship ID cannot be passed where
//! a task force ID is expected. IDs are handed out by an [`IdAllocator`] that
//! only counts upward, so an ID is never reused, even after its entity is
//! gone. Orders, events and saved games refer to entities only by ID.

use core::fmt;
use core::marker::PhantomData;

/// Implemented by every ID type.
pub trait EntityId: Copy + Ord + fmt::Debug {
    /// Short name used in logs, such as `ship`.
    const KIND: &'static str;
    fn from_raw(raw: u32) -> Self;
    fn raw(self) -> u32;
}

macro_rules! define_id {
    ($(#[$meta:meta])* $name:ident, $kind:literal) => {
        $(#[$meta])*
        #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
        pub struct $name(u32);

        impl $name {
            pub const fn new(raw: u32) -> Self {
                Self(raw)
            }

            pub const fn raw(self) -> u32 {
                self.0
            }
        }

        impl EntityId for $name {
            const KIND: &'static str = $kind;

            fn from_raw(raw: u32) -> Self {
                Self(raw)
            }

            fn raw(self) -> u32 {
                self.0
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "{}:{}", $kind, self.0)
            }
        }
    };
}

define_id!(
    /// A single ship.
    ShipId,
    "ship"
);
define_id!(
    /// A ship class in the reference data.
    ShipClassId,
    "ship_class"
);
define_id!(
    /// A task force.
    TaskForceId,
    "task_force"
);
define_id!(
    /// An air group.
    AirGroupId,
    "air_group"
);
define_id!(
    /// An individual pilot.
    PilotId,
    "pilot"
);
define_id!(
    /// A base: a port, an airfield, or both.
    BaseId,
    "base"
);
define_id!(
    /// A land unit.
    LandUnitId,
    "land_unit"
);
define_id!(
    /// A contact in one side's belief state.
    ContactId,
    "contact"
);
define_id!(
    /// An order.
    OrderId,
    "order"
);
define_id!(
    /// An entry in the event log.
    EventId,
    "event"
);
define_id!(
    /// An intent that an order serves. Intents arrive with the staff planner.
    IntentId,
    "intent"
);

/// Hands out IDs of one kind, counting upward from 1.
#[derive(Clone, Debug)]
pub struct IdAllocator<T: EntityId> {
    next: u32,
    kind: PhantomData<T>,
}

impl<T: EntityId> IdAllocator<T> {
    pub const fn new() -> Self {
        Self {
            next: 1,
            kind: PhantomData,
        }
    }

    /// Returns a fresh ID.
    pub fn alloc(&mut self) -> T {
        let id = T::from_raw(self.next);
        self.next = self.next.checked_add(1).expect("ran out of entity IDs");
        id
    }

    /// Marks an ID that came from a data file as taken, so it is never
    /// handed out again.
    pub fn reserve(&mut self, id: T) {
        let after = id.raw().checked_add(1).expect("ran out of entity IDs");
        if after > self.next {
            self.next = after;
        }
    }

    /// The raw value the next ID will have. Part of the state hash.
    pub const fn next_raw(&self) -> u32 {
        self.next
    }
}

impl<T: EntityId> Default for IdAllocator<T> {
    fn default() -> Self {
        Self::new()
    }
}

/// A reference to any entity that can take part in an event.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum EntityRef {
    Ship(ShipId),
    TaskForce(TaskForceId),
    AirGroup(AirGroupId),
    Pilot(PilotId),
    Base(BaseId),
    LandUnit(LandUnitId),
    Contact(ContactId),
}

impl EntityRef {
    /// A stable number for the kind. Never renumber these: they feed the
    /// random number keys and the state hash.
    pub const fn kind_code(self) -> u8 {
        match self {
            EntityRef::Ship(_) => 1,
            EntityRef::TaskForce(_) => 2,
            EntityRef::AirGroup(_) => 3,
            EntityRef::Pilot(_) => 4,
            EntityRef::Base(_) => 5,
            EntityRef::LandUnit(_) => 6,
            EntityRef::Contact(_) => 7,
        }
    }

    pub const fn raw_id(self) -> u32 {
        match self {
            EntityRef::Ship(id) => id.raw(),
            EntityRef::TaskForce(id) => id.raw(),
            EntityRef::AirGroup(id) => id.raw(),
            EntityRef::Pilot(id) => id.raw(),
            EntityRef::Base(id) => id.raw(),
            EntityRef::LandUnit(id) => id.raw(),
            EntityRef::Contact(id) => id.raw(),
        }
    }

    /// Kind and ID packed into one number, unique across all entities.
    pub const fn stable_key(self) -> u64 {
        ((self.kind_code() as u64) << 32) | self.raw_id() as u64
    }
}

impl fmt::Display for EntityRef {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EntityRef::Ship(id) => id.fmt(f),
            EntityRef::TaskForce(id) => id.fmt(f),
            EntityRef::AirGroup(id) => id.fmt(f),
            EntityRef::Pilot(id) => id.fmt(f),
            EntityRef::Base(id) => id.fmt(f),
            EntityRef::LandUnit(id) => id.fmt(f),
            EntityRef::Contact(id) => id.fmt(f),
        }
    }
}

impl From<ShipId> for EntityRef {
    fn from(id: ShipId) -> Self {
        EntityRef::Ship(id)
    }
}

impl From<TaskForceId> for EntityRef {
    fn from(id: TaskForceId) -> Self {
        EntityRef::TaskForce(id)
    }
}

impl From<BaseId> for EntityRef {
    fn from(id: BaseId) -> Self {
        EntityRef::Base(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn allocator_counts_upward_from_one() {
        let mut ids = IdAllocator::<ShipId>::new();
        assert_eq!(ids.alloc(), ShipId::new(1));
        assert_eq!(ids.alloc(), ShipId::new(2));
        assert_eq!(ids.next_raw(), 3);
    }

    #[test]
    fn reserved_ids_are_never_handed_out() {
        let mut ids = IdAllocator::<ShipId>::new();
        ids.reserve(ShipId::new(7));
        ids.reserve(ShipId::new(3));
        assert_eq!(ids.alloc(), ShipId::new(8));
    }

    #[test]
    fn ids_of_different_kinds_never_share_a_stable_key() {
        let ship = EntityRef::Ship(ShipId::new(5));
        let task_force = EntityRef::TaskForce(TaskForceId::new(5));
        assert_ne!(ship.stable_key(), task_force.stable_key());
        assert_ne!(ship, task_force);
    }

    #[test]
    fn ids_display_with_their_kind() {
        assert_eq!(ShipId::new(4).to_string(), "ship:4");
        assert_eq!(
            EntityRef::TaskForce(TaskForceId::new(2)).to_string(),
            "task_force:2"
        );
    }
}
