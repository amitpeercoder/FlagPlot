//! A field of an event read by its type (decision 0235).
//!
//! The export of an event's `data` reads each field of a kind that is not
//! marked `[by_hand]` through [`ReadField`], once for each type. A type has a
//! reading only if it is read the same way wherever the export uses it.
//! `EventKind::fields` asks for the [`UmpireKey`], so only a tool that shows
//! the truth can call it.

use crate::belief::GroundGrade;
use crate::command::{CommandKind, PlanId, PlanState};
use crate::entering::EntryWait;
use crate::events::{
    AirFirer, AirTarget, DamageTrack, DetachReason, EngagementEndReason, ExperienceSource,
    FieldworksChange, ForceFatigueChange, FuelOrderEnd, LandFatigueChange, LoadingLoss, MarchStop,
    RehearsalLoss, RosterLeft, SortieKind, SubmarineLeftReason, Weapon,
};
use crate::fixed::Fixed;
use crate::hex::Hex;
use crate::ids::{ContactId, EntityId, EntityRef, EventId, OrderId, UmpireKey};
use crate::orders::Job;
use crate::refdata::ShipKind;
use crate::state::{ShipSystem, Side, SystemState};

/// One field's value as plain data.
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum FieldValue {
    /// A whole number.
    Int(i64),
    /// A quantity.
    Num(Fixed),
    /// A flag.
    Bool(bool),
    /// A hex.
    Hex(Hex),
    /// A fixed word.
    Text(&'static str),
    /// Words made of the value, such as a unit named by its kind and number.
    Str(String),
    /// A list of hexes.
    Hexes(Vec<Hex>),
    /// A list of ids, as their numbers.
    Ids(Vec<u32>),
}

/// A type that has one reading wherever an event holds it.
pub trait ReadField {
    /// The value, or `None` where the reading is "nothing" and the field is
    /// left out.
    fn read(&self, key: &UmpireKey) -> Option<FieldValue>;
}

impl ReadField for Fixed {
    fn read(&self, _key: &UmpireKey) -> Option<FieldValue> {
        Some(FieldValue::Num(*self))
    }
}

impl ReadField for u32 {
    fn read(&self, _key: &UmpireKey) -> Option<FieldValue> {
        Some(FieldValue::Int(i64::from(*self)))
    }
}

impl ReadField for u8 {
    fn read(&self, _key: &UmpireKey) -> Option<FieldValue> {
        Some(FieldValue::Int(i64::from(*self)))
    }
}

impl ReadField for bool {
    fn read(&self, _key: &UmpireKey) -> Option<FieldValue> {
        Some(FieldValue::Bool(*self))
    }
}

impl ReadField for Hex {
    fn read(&self, _key: &UmpireKey) -> Option<FieldValue> {
        Some(FieldValue::Hex(*self))
    }
}

/// An id whose number is public (a ship, a task force, a base and the like):
/// its number. The opaque ids (an order, an event, a contact) are not here.
impl<T: EntityId> ReadField for T {
    fn read(&self, _key: &UmpireKey) -> Option<FieldValue> {
        Some(FieldValue::Int(i64::from(self.raw())))
    }
}

/// A list of ids with a public number: their numbers.
impl<T: EntityId> ReadField for Vec<T> {
    fn read(&self, _key: &UmpireKey) -> Option<FieldValue> {
        Some(FieldValue::Ids(self.iter().map(|id| id.raw()).collect()))
    }
}

impl ReadField for Vec<Hex> {
    fn read(&self, _key: &UmpireKey) -> Option<FieldValue> {
        Some(FieldValue::Hexes(self.clone()))
    }
}

/// A value that may be missing: the value's own reading, or nothing, so the
/// field is left out.
impl<T: ReadField> ReadField for Option<T> {
    fn read(&self, key: &UmpireKey) -> Option<FieldValue> {
        self.as_ref().and_then(|value| value.read(key))
    }
}

/// An order, an event or a contact: its number, which only the umpire reads
/// (decision 0086).
macro_rules! read_opaque_id {
    ($($id:ty),*) => {$(
        impl ReadField for $id {
            fn read(&self, key: &UmpireKey) -> Option<FieldValue> {
                Some(FieldValue::Int(i64::from(self.umpire_raw(key))))
            }
        }
    )*};
}
read_opaque_id!(ContactId, OrderId, EventId);

impl ReadField for PlanId {
    fn read(&self, _key: &UmpireKey) -> Option<FieldValue> {
        Some(FieldValue::Int(i64::from(self.raw())))
    }
}

/// A unit or a place named as `EntityRef`'s `Display` names it.
impl ReadField for EntityRef {
    fn read(&self, _key: &UmpireKey) -> Option<FieldValue> {
        Some(FieldValue::Str(self.to_string()))
    }
}

/// A type whose word is its own `name()`.
macro_rules! read_by_name {
    ($($ty:ty),*) => {$(
        impl ReadField for $ty {
            fn read(&self, _key: &UmpireKey) -> Option<FieldValue> {
                Some(FieldValue::Text(self.name()))
            }
        }
    )*};
}
read_by_name!(
    AirFirer,
    AirTarget,
    CommandKind,
    ExperienceSource,
    FieldworksChange,
    FuelOrderEnd,
    GroundGrade,
    Job,
    MarchStop,
    PlanState,
    ShipKind,
    SortieKind,
    SubmarineLeftReason
);

/// A type whose word is one of the functions below.
macro_rules! read_by_word {
    ($($ty:ty => $word:ident),*) => {$(
        impl ReadField for $ty {
            fn read(&self, _key: &UmpireKey) -> Option<FieldValue> {
                Some(FieldValue::Text($word(*self)))
            }
        }
    )*};
}
read_by_word!(
    Side => side_word,
    Weapon => weapon_word,
    DamageTrack => track_word,
    EngagementEndReason => end_reason_word,
    DetachReason => detach_reason_word,
    ShipSystem => system_word,
    SystemState => system_state_word,
    LoadingLoss => loading_loss_word,
    RehearsalLoss => rehearsal_loss_word,
    RosterLeft => roster_left_word,
    ForceFatigueChange => force_fatigue_word,
    LandFatigueChange => land_fatigue_word,
    EntryWait => entry_wait_word
);

/// The word of a side in the export.
pub fn side_word(side: Side) -> &'static str {
    match side {
        Side::Allied => "allied",
        Side::Japanese => "japanese",
    }
}

/// The word of a weapon in the export.
pub fn weapon_word(weapon: Weapon) -> &'static str {
    match weapon {
        Weapon::Gun => "gun",
        Weapon::Torpedo => "torpedo",
        Weapon::Bomb => "bomb",
    }
}

/// The word of a damage track in the export.
pub fn track_word(track: DamageTrack) -> &'static str {
    match track {
        DamageTrack::Flotation => "flotation",
        DamageTrack::Fire => "fire",
    }
}

/// The word of the end of a fight in the export.
pub fn end_reason_word(reason: EngagementEndReason) -> &'static str {
    match reason {
        EngagementEndReason::SlippedAway => "slipped_away",
        EngagementEndReason::ForceSunk => "force_sunk",
        EngagementEndReason::BrokeOff => "broke_off",
        EngagementEndReason::PassedUnseen => "passed_unseen",
        EngagementEndReason::MaxRounds => "max_rounds",
    }
}

/// The word of why ships were detached in the export.
pub fn detach_reason_word(reason: DetachReason) -> &'static str {
    match reason {
        DetachReason::SentHome => "sent_home",
        DetachReason::LeftBehind => "left_behind",
        DetachReason::Ordered => "ordered",
    }
}

/// The word of a ship's system in the export.
pub fn system_word(system: ShipSystem) -> &'static str {
    match system {
        ShipSystem::Propulsion => "propulsion",
        ShipSystem::FireControl => "fire_control",
        ShipSystem::MainBattery => "main_battery",
        ShipSystem::TorpedoMounts => "torpedo_mounts",
        ShipSystem::FlightDeck => "flight_deck",
    }
}

/// The word of a system's state in the export.
pub fn system_state_word(state: SystemState) -> &'static str {
    match state {
        SystemState::Working => "working",
        SystemState::Degraded => "degraded",
        SystemState::Out => "out",
    }
}

/// Why a force's loading fell (decision 0095).
pub fn loading_loss_word(reason: LoadingLoss) -> &'static str {
    match reason {
        LoadingLoss::NewLandingPlace => "new_landing_place",
        LoadingLoss::UnitAdded => "unit_added",
        LoadingLoss::Spent => "spent",
    }
}

/// Why a unit's rehearsal fell (decision 0095).
pub fn rehearsal_loss_word(reason: RehearsalLoss) -> &'static str {
    match reason {
        RehearsalLoss::NewLandingPlace => "new_landing_place",
        RehearsalLoss::Landed => "landed",
        RehearsalLoss::OtherLandingPlace => "other_landing_place",
    }
}

/// Why a unit left a roster (decision 0095).
pub fn roster_left_word(reason: RosterLeft) -> &'static str {
    match reason {
        RosterLeft::TakenByOrder => "taken_by_another_force",
        RosterLeft::DroppedByOrder => "dropped_by_order",
        RosterLeft::Landed => "landed",
    }
}

/// How a task force's fatigue changed, as the export words it.
pub fn force_fatigue_word(change: ForceFatigueChange) -> &'static str {
    match change {
        ForceFatigueChange::Gained => "gained",
        ForceFatigueChange::Rested => "rested",
    }
}

/// How a land unit's fatigue changed, as the export words it.
pub fn land_fatigue_word(change: LandFatigueChange) -> &'static str {
    match change {
        LandFatigueChange::Gained => "gained",
        LandFatigueChange::Rested => "rested",
        LandFatigueChange::Marched => "marched",
        LandFatigueChange::MarchedAndAttacked => "marched_and_attacked",
        LandFatigueChange::Landed => "landed",
        LandFatigueChange::Strained => "strained",
    }
}

/// Why an entry waits, as the export words it.
pub fn entry_wait_word(wait: EntryWait) -> &'static str {
    match wait {
        EntryWait::NotTheSidesBase => "not_the_sides_base",
        EntryWait::TakesNoAircraft => "takes_no_aircraft",
    }
}

/// What `EventKind::fields` gives: each field's key and value, in the order
/// declared.
pub type Fields = Vec<(&'static str, FieldValue)>;
