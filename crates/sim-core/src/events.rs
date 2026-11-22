//! The event log.
//!
//! Every state change is written here as an event, with references to the
//! orders and earlier events that caused it. The log can answer "what
//! happened to this ship today" and "why" without re-running the day.

use core::fmt;

use crate::belief::GroundGrade;
use crate::claims::UnitClaim;
use crate::command::{
    CommandKind, CommandRefusal, HoldTarget, InterruptCondition, InterruptRef, NoticeKind, PlanId,
    PlanState, Reason, StandingKind, StandingSubject,
};
use crate::fixed::Fixed;
use crate::held::AsOf;
use crate::hex::Hex;
use crate::ids::{
    AirGroupId, AircraftTypeId, BaseId, ContactId, EntityRef, EventId, IntentId, LandUnitId,
    OrderId, ShipId, TaskForceId,
};
use crate::orders::{Cargo, Issuer, Job, RejectReason};
use crate::pictures::CommandRef;
use crate::refdata::ShipKind;
use crate::state::{AirGroupLocation, HaltWork, ShipSystem, Side, SystemState};
use crate::time::{ClockTime, Pulse};

/// Something that led to an event.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum Cause {
    Order(OrderId),
    Event(EventId),
}

impl fmt::Display for Cause {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Cause::Order(id) => write!(f, "{}", id.shown()),
            Cause::Event(id) => write!(f, "#{}", id.raw()),
        }
    }
}

/// One input to a computed quantity.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Factor {
    pub name: &'static str,
    pub value: Fixed,
}

/// How a computed quantity came about: its inputs and its result.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Breakdown {
    pub factors: Vec<Factor>,
    pub result: Fixed,
}

/// The weapon a salvo was fired with.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum Weapon {
    Gun,
    Torpedo,
    /// Dropped by aircraft (decision 0026). Added at the end of the enum.
    Bomb,
}

impl Weapon {
    /// The weapon's name as the event log writes it.
    pub const fn name(self) -> &'static str {
        match self {
            Weapon::Gun => "guns",
            Weapon::Torpedo => "torpedoes",
            Weapon::Bomb => "bombs",
        }
    }
}

/// Who fires in a fight in the air (decision 6 of
/// `docs/decisions/0027-fighters-and-anti-aircraft-fire.md`).
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum AirFirer {
    /// The fighters that went with a raid.
    Escorts,
    /// The fighters kept over their own hex.
    Patrol,
}

impl AirFirer {
    /// The name as the event log writes it.
    pub const fn name(self) -> &'static str {
        match self {
            AirFirer::Escorts => "escorts",
            AirFirer::Patrol => "patrol",
        }
    }
}

/// Who is fired at in a fight in the air (decision 6 of 0027).
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum AirTarget {
    Patrol,
    Escorts,
    /// The bombers of the raid.
    Bombers,
}

impl AirTarget {
    /// The name as the event log writes it.
    pub const fn name(self) -> &'static str {
        match self {
            AirTarget::Patrol => "patrol",
            AirTarget::Escorts => "escorts",
            AirTarget::Bombers => "bombers",
        }
    }
}

/// What an air group flew when it rolled for wear (decision 4 of
/// `docs/decisions/0035-wear-repair-and-operational-losses.md`).
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum SortieKind {
    /// A search out to the range of its arc and back.
    Search,
    /// A strike at a contact, out and back.
    Strike,
    /// A strike at a base, out and back.
    BaseStrike,
    /// A patrol over the group's own hex.
    Patrol,
    /// Escorts with a raid, out and back.
    Escort,
}

impl SortieKind {
    /// The name as the event log writes it.
    pub const fn name(self) -> &'static str {
        match self {
            SortieKind::Search => "search",
            SortieKind::Strike => "strike",
            SortieKind::BaseStrike => "base strike",
            SortieKind::Patrol => "patrol",
            SortieKind::Escort => "escort",
        }
    }
}

/// What a base spent supply on (decisions 1 to 3 of
/// `docs/decisions/0039-base-supply-consumption.md`).
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum SupplyUse {
    /// The sorties flown from the base in a daylight pulse.
    Sorties,
    /// The aircraft at the base, for the day.
    Upkeep,
    /// Repairs made at the base.
    Repairs,
    /// The men of the side ashore in the base's hex, for the day (decision
    /// 0066, 16). Added at the end.
    Men,
    /// The supply a land unit took with it when it marched out of the base's
    /// hex (decision 0094, 9). Added at the end.
    March,
    /// What the side's men at grips in a day's fight in the base's hex cost,
    /// where the base's ammunition did not cover it (decision 0209, 5 and
    /// 14). Added at the end.
    Fight,
}

impl SupplyUse {
    /// The name as the event log writes it.
    pub const fn name(self) -> &'static str {
        match self {
            SupplyUse::Sorties => "sorties",
            SupplyUse::Upkeep => "upkeep",
            SupplyUse::Repairs => "repairs",
            SupplyUse::Men => "men",
            SupplyUse::March => "march",
            SupplyUse::Fight => "fight",
        }
    }
}

/// How an air group's fatigue changed (decisions 4 and 5 of
/// `docs/decisions/0036-fatigue-and-rest.md`).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum FatigueChange {
    /// It flew: the longest of its sorties in the pulse, and its distance.
    Gained {
        sortie: SortieKind,
        distance_nm: Fixed,
    },
    /// It did not fly.
    Rested,
}

/// How a task force's fatigue changed (decision 2 of
/// `docs/decisions/0037-fatigue-at-battle-readiness.md`).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ForceFatigueChange {
    /// The force was alert at the start of the pulse.
    Gained,
    /// The force was not alert at the start of the pulse.
    Rested,
}

/// How a land unit's fatigue changed in the ground combat phase (decision
/// 12 of 0090).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum LandFatigueChange {
    /// The unit was on attack and fired in the day's fight.
    Gained,
    /// Any other unit with men ashore.
    Rested,
    /// The unit marched that day and did not fire on attack (decision 8 of
    /// 0094). Added at the end.
    Marched,
    /// The unit marched that day and also fired on attack: it gains both
    /// (decision 8 of 0094). Added at the end.
    MarchedAndAttacked,
    /// The unit's first men landed from a ship (decision 6 of 0095): the
    /// fatigue of a landing, less for a rehearsal. Added at the end.
    Landed,
    /// Bombs or shells landed on men of the unit's side in its hex that day,
    /// so it did not rest, and it gained the strain (decision 10 of 0106).
    /// Added at the end.
    Strained,
}

/// How a land unit's fieldworks changed (decisions 5, 6 and 12 of 0106).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum FieldworksChange {
    /// The unit dug in where it stands.
    Dug,
    /// Hits on men wore them down.
    Worn,
    /// The unit left them: it marched, fell back, was taken aboard or was
    /// set to attack.
    Lost,
}

impl FieldworksChange {
    /// A stable number for the change. Never renumber it; add new changes
    /// at the end.
    pub const fn code(self) -> u8 {
        match self {
            FieldworksChange::Dug => 0,
            FieldworksChange::Worn => 1,
            FieldworksChange::Lost => 2,
        }
    }

    /// The change as the event log writes it.
    pub const fn name(self) -> &'static str {
        match self {
            FieldworksChange::Dug => "dug",
            FieldworksChange::Worn => "worn",
            FieldworksChange::Lost => "lost",
        }
    }
}

/// Why a force's order to fuel from another ended (decision 0201, 7).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum FuelOrderEnd {
    /// Every ship of the force is full.
    Full,
    /// The giver's ships with the mark hold no cargo fuel.
    GiverEmpty,
    /// The giver has no ship with the mark afloat in her force: sunk,
    /// detached, joined to another force, stood down or sent to the yard.
    NoOiler,
    /// A new route order was given to the force.
    NewRoute,
}

impl FuelOrderEnd {
    /// A stable number for the reason. Never renumber it; add new reasons at
    /// the end.
    pub const fn code(self) -> u8 {
        match self {
            FuelOrderEnd::Full => 0,
            FuelOrderEnd::GiverEmpty => 1,
            FuelOrderEnd::NoOiler => 2,
            FuelOrderEnd::NewRoute => 3,
        }
    }

    /// The reason as the event log writes it.
    pub const fn name(self) -> &'static str {
        match self {
            FuelOrderEnd::Full => "full",
            FuelOrderEnd::GiverEmpty => "giver_empty",
            FuelOrderEnd::NoOiler => "no_oiler",
            FuelOrderEnd::NewRoute => "new_route",
        }
    }
}

/// Why a force lost loading (decision 0095, 5, 7, 8 and 10).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum LoadingLoss {
    /// The force was ordered to prepare a landing at another hex.
    NewLandingPlace,
    /// A unit was added to its roster: it kept a share.
    UnitAdded,
    /// Its unloading at the hex was complete: the loading is spent and the
    /// order done.
    Spent,
}

impl LoadingLoss {
    /// A stable number for the reason. Never renumber it; add new reasons at
    /// the end.
    pub const fn code(self) -> u8 {
        match self {
            LoadingLoss::NewLandingPlace => 0,
            LoadingLoss::UnitAdded => 1,
            LoadingLoss::Spent => 2,
        }
    }

    /// The reason as the event log writes it.
    pub const fn name(self) -> &'static str {
        match self {
            LoadingLoss::NewLandingPlace => "new landing place",
            LoadingLoss::UnitAdded => "unit added",
            LoadingLoss::Spent => "spent by the landing",
        }
    }
}

/// Why a unit lost rehearsal (decision 0095, 7, 8 and 3b of 0099).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum RehearsalLoss {
    /// Its force was ordered to prepare a landing at another hex.
    NewLandingPlace,
    /// Its first men landed: the rehearsal is spent.
    Landed,
    /// The unit rehearses a landing at another hex than the one it had
    /// rehearsed, with another force.
    OtherLandingPlace,
}

impl RehearsalLoss {
    /// A stable number for the reason. Never renumber it; add new reasons at
    /// the end.
    pub const fn code(self) -> u8 {
        match self {
            RehearsalLoss::NewLandingPlace => 0,
            RehearsalLoss::Landed => 1,
            RehearsalLoss::OtherLandingPlace => 2,
        }
    }

    /// The reason as the event log writes it.
    pub const fn name(self) -> &'static str {
        match self {
            RehearsalLoss::NewLandingPlace => "new landing place",
            RehearsalLoss::Landed => "landed",
            RehearsalLoss::OtherLandingPlace => "another landing place",
        }
    }
}

/// Why a unit left a force's roster (decision 0095, 7 and 3c of 0099).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum RosterLeft {
    /// Another force's prepare order named it.
    TakenByOrder,
    /// A new order of the same force left it out.
    DroppedByOrder,
    /// Its first men landed from a ship.
    Landed,
}

impl RosterLeft {
    /// A stable number for the reason. Never renumber it; add new reasons at
    /// the end.
    pub const fn code(self) -> u8 {
        match self {
            RosterLeft::TakenByOrder => 0,
            RosterLeft::DroppedByOrder => 1,
            RosterLeft::Landed => 2,
        }
    }

    /// The reason as the event log writes it.
    pub const fn name(self) -> &'static str {
        match self {
            RosterLeft::TakenByOrder => "taken by another force's order",
            RosterLeft::DroppedByOrder => "dropped by order",
            RosterLeft::Landed => "landed",
        }
    }
}

/// What raised a land unit's experience (decisions 3 and 5 of 0097).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ExperienceSource {
    Training,
    Fighting,
}

impl ExperienceSource {
    /// A stable number for the source. Never renumber it; add new sources at
    /// the end.
    pub const fn code(self) -> u8 {
        match self {
            ExperienceSource::Training => 0,
            ExperienceSource::Fighting => 1,
        }
    }

    /// The source as the event log writes it.
    pub const fn name(self) -> &'static str {
        match self {
            ExperienceSource::Training => "training",
            ExperienceSource::Fighting => "fighting",
        }
    }
}

/// Why a land unit's march stopped short of its hex (decision 0094, 3).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum MarchStop {
    /// An order named the unit's own hex.
    Ordered,
    /// The unit had no man ashore left to march.
    NoMenAshore,
    /// The unit's men ashore are in a hex no land link joins to the hex it
    /// was marching to: they left the shore and landed elsewhere.
    NoLandLink,
}

impl MarchStop {
    /// The name as the event log writes it.
    pub const fn name(self) -> &'static str {
        match self {
            MarchStop::Ordered => "ordered",
            MarchStop::NoMenAshore => "no_men_ashore",
            MarchStop::NoLandLink => "no_land_link",
        }
    }
}

/// One of the two damage tracks of a ship. Named systems are separate
/// (decision 0016).
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum DamageTrack {
    Flotation,
    Fire,
}

impl DamageTrack {
    /// Every track, in the order gun hits are assigned to them.
    pub const ALL: [DamageTrack; 2] = [DamageTrack::Flotation, DamageTrack::Fire];

    /// The track's name as the event log writes it.
    pub const fn name(self) -> &'static str {
        match self {
            DamageTrack::Flotation => "flotation",
            DamageTrack::Fire => "fire",
        }
    }
}

/// Why a surface engagement ended.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum EngagementEndReason {
    /// A force that avoids sighted the enemy first and left.
    SlippedAway,
    /// A force had no ship afloat.
    ForceSunk,
    /// A force broke off.
    BrokeOff,
    /// Neither force sighted the other in all the rounds.
    PassedUnseen,
    /// The last round was fought.
    MaxRounds,
}

impl EngagementEndReason {
    /// The reason as the event log writes it.
    pub const fn name(self) -> &'static str {
        match self {
            EngagementEndReason::SlippedAway => "slipped away",
            EngagementEndReason::ForceSunk => "a force was sunk",
            EngagementEndReason::BrokeOff => "a force broke off",
            EngagementEndReason::PassedUnseen => "passed unseen",
            EngagementEndReason::MaxRounds => "reached the last round",
        }
    }
}

/// Why ships were split off a task force by the straggler rule (decision 12
/// of `docs/decisions/0016-systems-and-stragglers.md`).
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum DetachReason {
    /// Stragglers that can steam, sent for the home port with escorts.
    SentHome,
    /// Stragglers left where they are.
    LeftBehind,
    /// Ships detached by a detach order.
    Ordered,
}

impl DetachReason {
    /// A stable number for the reason. Never renumber it; add new reasons at
    /// the end.
    pub const fn code(self) -> u8 {
        match self {
            DetachReason::SentHome => 1,
            DetachReason::LeftBehind => 2,
            DetachReason::Ordered => 3,
        }
    }

    /// The reason as the event log writes it.
    pub const fn name(self) -> &'static str {
        match self {
            DetachReason::SentHome => "sent home",
            DetachReason::LeftBehind => "left behind",
            DetachReason::Ordered => "ordered",
        }
    }
}

/// The way a report is passed between commands (decision 0135, 3 to 5).
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum Direction {
    /// To the sender's superior.
    Up,
    /// To another command within the sender's radio reach.
    Sideways,
    /// From the area command to a command under it.
    Down,
}

impl Direction {
    /// A stable number. Part of the state hash. Never renumber.
    pub const fn code(self) -> u8 {
        match self {
            Direction::Up => 1,
            Direction::Sideways => 2,
            Direction::Down => 3,
        }
    }

    /// The word the log uses.
    pub const fn name(self) -> &'static str {
        match self {
            Direction::Up => "up",
            Direction::Sideways => "sideways",
            Direction::Down => "down",
        }
    }
}

/// When a passed report reaches its listener (decision 0135, 8).
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum Arrival {
    /// In the pulse it was sent.
    OnTime,
    /// In the next pulse.
    Late,
    /// Never.
    Never,
}

impl Arrival {
    /// A stable number. Part of the state hash. Never renumber.
    pub const fn code(self) -> u8 {
        match self {
            Arrival::OnTime => 1,
            Arrival::Late => 2,
            Arrival::Never => 3,
        }
    }

    /// The words the log uses.
    pub const fn name(self) -> &'static str {
        match self {
            Arrival::OnTime => "on time",
            Arrival::Late => "a pulse late",
            Arrival::Never => "never",
        }
    }
}

/// One listener of a report sent (decision 0135, 21): the command, the
/// direction it was sent in and when it arrives.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct ReportListener {
    pub command: CommandRef,
    pub direction: Direction,
    pub arrival: Arrival,
}

/// What happened.
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum EventKind {
    OrderAccepted {
        order: OrderId,
        target: EntityRef,
    },
    OrderRejected {
        order: OrderId,
        target: EntityRef,
        reason: RejectReason,
    },
    /// A task force moved during a pulse.
    TaskForceSteamed {
        task_force: TaskForceId,
        from: Hex,
        to: Hex,
        distance_nm: Fixed,
        speed_kn: Fixed,
        /// Each hex entered, in order.
        hexes_entered: Vec<Hex>,
    },
    FuelBurned {
        ship: ShipId,
        task_force: TaskForceId,
        burned_t: Fixed,
        remaining_t: Fixed,
    },
    /// A task force reached the last waypoint of its route.
    TaskForceArrived {
        task_force: TaskForceId,
        hex: Hex,
        /// Elapsed hours since the scenario began.
        elapsed_hours: Fixed,
    },
    /// A task force stopped because a ship ran dry.
    TaskForceOutOfFuel {
        task_force: TaskForceId,
        hex: Hex,
    },
    /// A ship took supply or fuel cargo aboard from a base. `cargo` says
    /// which (decision 0046); the tons, and the stock, are of that cargo.
    CargoLoaded {
        ship: ShipId,
        task_force: TaskForceId,
        base: BaseId,
        cargo: Cargo,
        loaded_t: Fixed,
        /// Tons of that cargo aboard the ship after loading.
        aboard_t: Fixed,
        /// The base's stock of that cargo after loading.
        base_stock_t: Fixed,
    },
    /// A ship put supply or fuel cargo ashore into a base's stock. `cargo`
    /// says which (decision 0046); the tons, and the stock, are of that cargo.
    CargoUnloaded {
        ship: ShipId,
        task_force: TaskForceId,
        base: BaseId,
        cargo: Cargo,
        unloaded_t: Fixed,
        /// Tons of that cargo aboard the ship after unloading.
        aboard_t: Fixed,
        /// The base's stock of that cargo after unloading.
        base_stock_t: Fixed,
    },
    /// Every ship of a task force is full, or the base has no stock left of
    /// the cargo loaded.
    LoadingComplete {
        task_force: TaskForceId,
        base: BaseId,
        cargo: Cargo,
        /// Elapsed hours since the scenario began.
        elapsed_hours: Fixed,
    },
    /// Every ship of a task force has put all its supply and fuel cargo
    /// ashore.
    UnloadingComplete {
        task_force: TaskForceId,
        base: BaseId,
        /// Elapsed hours since the scenario began.
        elapsed_hours: Fixed,
    },
    /// An air group flew its search arc from its base, or from the hex of the
    /// ship it is aboard (decision 4 of 0025). `hex` is the hex it flew from.
    SearchFlown {
        air_group: AirGroupId,
        from: AirGroupLocation,
        hex: Hex,
        from_deg: u32,
        to_deg: u32,
        range_nm: Fixed,
        aircraft: u32,
    },
    /// One roll of an air group looking for one task force in its arc.
    SearchAttempt {
        air_group: AirGroupId,
        task_force: TaskForceId,
        distance_nm: Fixed,
        chance: Fixed,
        sighted: bool,
    },
    /// A sighting report reached the searching side's belief.
    ReportReceived {
        side: Side,
        contact: ContactId,
        hex: Hex,
        ship_count: u32,
        kind: ShipKind,
        /// True if the report made a new contact, false if it refreshed one.
        created: bool,
    },
    /// A contact went a pulse without a fresh report.
    ContactAged {
        side: Side,
        contact: ContactId,
        age_pulses: u32,
    },
    /// A contact reached the age at which it is forgotten and was dropped.
    ContactDropped {
        side: Side,
        contact: ContactId,
        age_pulses: u32,
    },
    /// Two opposing task forces met in one hex and a fight began.
    EngagementBegun {
        a: TaskForceId,
        b: TaskForceId,
        hex: Hex,
        /// Elapsed hours since the scenario began at the meeting: the start
        /// of the pulse for forces that were in one hex at its start, else
        /// the start of the pulse plus the hours the arriving force had
        /// steamed.
        elapsed_hours: Fixed,
    },
    /// One roll of a force looking for the enemy force.
    SightingRoll {
        round: u32,
        task_force: TaskForceId,
        enemy: TaskForceId,
        /// The chance after every factor, fatigue's included.
        chance: Fixed,
        /// The factor of the force's fatigue, (100 less fatigue) over 100
        /// (decision 3 of 0037). One where the scenario has no fatigue, and
        /// then neither the text nor the export mentions it.
        fatigue_factor: Fixed,
        sighted: bool,
    },
    /// A force that has not sighted the enemy is fired on by an enemy that
    /// has.
    ForceSurprised {
        round: u32,
        task_force: TaskForceId,
        enemy: TaskForceId,
    },
    /// One ship fired one weapon at one ship.
    Salvo {
        round: u32,
        firer: ShipId,
        target: ShipId,
        weapon: Weapon,
        shots: Fixed,
        chance: Fixed,
        expected: Fixed,
        hits: u32,
    },
    /// The hits of one salvo on one damage track of the target.
    ShipDamaged {
        round: u32,
        ship: ShipId,
        track: DamageTrack,
        hits: u32,
        points: Fixed,
        value_after: Fixed,
    },
    /// A ship's flotation damage reached 100 and it left its task force.
    ShipSunk {
        round: u32,
        ship: ShipId,
        task_force: TaskForceId,
    },
    /// A force sighted the enemy or was fired on and was not alert.
    ForceBecameAlert {
        round: u32,
        task_force: TaskForceId,
    },
    /// A force broke off. `share` is the share of the ships it began the
    /// fight with that are sunk or crippled.
    ForceBrokeOff {
        round: u32,
        task_force: TaskForceId,
        share: Fixed,
    },
    /// A force that avoids sighted the enemy first and left unseen.
    ForceSlippedAway {
        round: u32,
        task_force: TaskForceId,
        enemy: TaskForceId,
    },
    /// Neither force sighted the other in the last round.
    ForcesPassedUnseen {
        round: u32,
        a: TaskForceId,
        b: TaskForceId,
    },
    /// A fight ended.
    EngagementEnded {
        round: u32,
        a: TaskForceId,
        b: TaskForceId,
        hex: Hex,
        reason: EngagementEndReason,
    },
    /// A claim reached a side's belief: of a surface fight (ships believed
    /// sunk only) or, since decision 0056, of a raid (also hits believed
    /// scored, the kind of ship believed attacked, aircraft believed shot
    /// down and aircraft believed destroyed on the ground).
    ClaimReceived {
        side: Side,
        hex: Hex,
        claimed_sunk: u32,
        claimed_hits: u32,
        claimed_kind: Option<ShipKind>,
        claimed_shot_down: u32,
        claimed_on_ground: u32,
        /// True for the claim of a raid.
        from_air: bool,
        /// True for the claim of a shore bombardment (decision 0092, 17).
        from_bombardment: bool,
        /// The types of the aircraft claimed destroyed on the ground, each
        /// with its number, in order of the type's number (decision 0179):
        /// empty, or adding to `claimed_on_ground`.
        ground_types: Vec<(AircraftTypeId, u32)>,
        /// Who claims what (decision 0180): each unit of the side whose fire
        /// a raid's claim rests on and that claims something, in the order
        /// it first fired. Empty for a surface fight's and a bombardment's.
        by_unit: Vec<UnitClaim>,
        /// The types of the aircraft claimed shot down, each with its
        /// number, in order of the type's number (decision 0210): empty, or
        /// adding to `claimed_shot_down`.
        air_types: Vec<(AircraftTypeId, u32)>,
    },
    /// A hit knocked, or tried to knock, one named system of a ship down one
    /// level (decision 0016). `before == after` when the ship lacks the
    /// system or it was already out.
    SystemHit {
        round: u32,
        ship: ShipId,
        system: ShipSystem,
        before: SystemState,
        after: SystemState,
    },
    /// The straggler rule split ships off a task force into a new one
    /// (decision 0016). `ships` are the stragglers moved and `escorts` the
    /// escorts moved, each in ID order. `escorts` is empty when the ships
    /// are left behind.
    ShipsDetached {
        parent: TaskForceId,
        new_force: TaskForceId,
        ships: Vec<ShipId>,
        escorts: Vec<ShipId>,
        reason: DetachReason,
    },
    /// A task force retired under the straggler rule: it is set to avoid
    /// combat and, if it has a home port, heads for it.
    ForceRetired {
        task_force: TaskForceId,
    },
    /// At the start of a pulse a force's side held a recent contact near it,
    /// and the force became alerted (decision 3 of
    /// `docs/decisions/0021-readiness-and-alarm.md`). `contact` is the
    /// nearest such contact, the lowest ID on a tie.
    ForceAlertedByContact {
        task_force: TaskForceId,
        contact: ContactId,
    },
    /// At the start of a pulse an alerted force no longer had such a contact
    /// and stood down (decision 3 of 0021).
    ForceStoodDown {
        task_force: TaskForceId,
    },
    /// One roll of the alarm of a fight in which shots were fired, for a
    /// force in its hex that was not alert (decision 5 of 0021). Recorded
    /// whether or not it succeeded. `by_a` and `by_b` are the two forces that
    /// fought, the lower ID first. On success the force is alerted by this
    /// event.
    AlarmRoll {
        task_force: TaskForceId,
        by_a: TaskForceId,
        by_b: TaskForceId,
        chance: Fixed,
        alarmed: bool,
    },
    /// Aircraft of a group that were destroyed on the ground at a base were
    /// damaged ones (decision 11 of 0035). `remaining` is what the group has
    /// damaged afterwards. The ready aircraft destroyed in the same raid are
    /// an `AircraftDestroyedOnGround` right after.
    DamagedAircraftDestroyedOnGround {
        air_group: AirGroupId,
        base: BaseId,
        aircraft: u32,
        remaining: u32,
    },
    /// An air group rolled for wear after a sortie (decisions 3 to 7 of
    /// 0035): `aircraft` flew and came back, each was worn at `chance`, and
    /// `worn` of them were, of which `written_off` are lost for good. `ready`
    /// and `damaged` are what the group has afterwards.
    AircraftWorn {
        air_group: AirGroupId,
        sortie: SortieKind,
        distance_nm: Fixed,
        aircraft: u32,
        chance: Fixed,
        expected: Fixed,
        worn: u32,
        written_off: u32,
        ready: u32,
        damaged: u32,
    },
    /// A base or a carrier repaired damaged aircraft of a group at the start
    /// of the day (decision 9 of 0035). `at` is where the group is.
    AircraftRepaired {
        air_group: AirGroupId,
        at: AirGroupLocation,
        repaired: u32,
        ready: u32,
        damaged: u32,
    },
    /// An air group aboard a ship that sank was lost with it (decision 5 of
    /// 0025). `aircraft` is the number of aircraft it lost, ready and damaged.
    AirGroupLost {
        air_group: AirGroupId,
        ship: ShipId,
        aircraft: u32,
    },
    /// An air group on the naval strike mission launched at a contact its
    /// side holds (decisions 3 to 6 of `docs/decisions/0026-air-strikes.md`).
    /// `hex` is the hex the contact names, and where the strike flies to.
    StrikeLaunched {
        air_group: AirGroupId,
        contact: ContactId,
        hex: Hex,
        aircraft: u32,
        distance_nm: Fixed,
    },
    /// A strike reached the hex it flew to and no enemy task force was there
    /// (decision 7 of 0026).
    StrikeFoundNothing {
        air_group: AirGroupId,
        hex: Hex,
    },
    /// A strike attacked one ship of an enemy task force (decisions 7 to 9
    /// of 0026).
    AirAttack {
        air_group: AirGroupId,
        task_force: TaskForceId,
        ship: ShipId,
        weapon: Weapon,
        shots: Fixed,
        chance: Fixed,
        expected: Fixed,
        hits: u32,
        /// What the draw of decision 0167 gave: the aircraft's type where the
        /// people of the ship recognised it, `None` where they did not. Never
        /// a type other than the group's own (decision 0167, 4).
        recognised: Option<AircraftTypeId>,
    },
    /// A fighter group sent escorts with a raid (decision 5 of
    /// `docs/decisions/0027-fighters-and-anti-aircraft-fire.md`).
    /// `with_air_group` is the first strike group of the raid.
    EscortsSent {
        air_group: AirGroupId,
        with_air_group: AirGroupId,
        aircraft: u32,
    },
    /// Fighters fired at fighters or bombers over the hex a raid attacked
    /// (decision 6 of 0027). `firer` is the first group of the firing
    /// fighters and `target` the first group of those fired at.
    /// `after_attack` is true for the part of the patrol's fire at the
    /// bombers worked after the raid's attack (decision 0087).
    AirFight {
        firer: AirGroupId,
        target: AirGroupId,
        firing: AirFirer,
        at: AirTarget,
        shots: Fixed,
        chance: Fixed,
        expected: Fixed,
        hits: u32,
        after_attack: bool,
    },
    /// The ships of a task force fired their anti-aircraft guns at the
    /// bombers of a raid (decision 7 of 0027). `air_group` is the raid's
    /// first strike group. `after_attack` is true for the part of the guns'
    /// fire worked after the raid's attack (decision 0079).
    AntiAircraftFire {
        task_force: TaskForceId,
        air_group: AirGroupId,
        shots: Fixed,
        chance: Fixed,
        expected: Fixed,
        hits: u32,
        after_attack: bool,
    },
    /// An air group lost aircraft to one fight or one round of fire
    /// (decision 11 of 0027). `remaining` is what it has ready afterwards.
    AircraftLost {
        air_group: AirGroupId,
        aircraft: u32,
        remaining: u32,
    },
    /// An air group lost pilots with aircraft lost away from home, or with
    /// its ship (decisions 3 and 5 of 0038). `left` is what it has after.
    PilotsLost {
        air_group: AirGroupId,
        lost: u32,
        left: u32,
    },
    /// An air group received replacement aircraft, pilots or both at its base
    /// at the start of the day (decisions 6 to 8 of 0038). `ready` and
    /// `pilots_now` are what it has after; `pilots_now` is 0 for a group
    /// whose pilots are not tracked.
    ReplacementsReceived {
        air_group: AirGroupId,
        base: BaseId,
        aircraft: u32,
        pilots: u32,
        ready: u32,
        pilots_now: u32,
    },
    /// Aircraft, pilots or both the scenario flew in to an air group at the
    /// start of the day (decision 0041). `ready` and `pilots_now` are what
    /// it has after, `strength` is its new ceiling; `pilots_now` is 0 for a
    /// group whose pilots are not tracked.
    AircraftArrived {
        air_group: AirGroupId,
        aircraft: u32,
        pilots: u32,
        ready: u32,
        pilots_now: u32,
        strength: u32,
    },
    /// A strike that found nothing told its side so, and the side dropped the
    /// contact the strike was sent at (decision 0028).
    ContactNotThere {
        side: Side,
        contact: ContactId,
        hex: Hex,
    },
    /// An air group on the base strike mission launched at the base it was
    /// ordered to strike (decision 3 of
    /// `docs/decisions/0034-raids-on-a-base.md`). `hex` is the base's hex.
    BaseStrikeLaunched {
        air_group: AirGroupId,
        base: BaseId,
        hex: Hex,
        aircraft: u32,
        distance_nm: Fixed,
    },
    /// A base fired its anti-aircraft guns at the bombers of a raid on it
    /// (decision 5 of 0034). `air_group` is the raid's first strike group.
    /// `after_attack` is as for `AntiAircraftFire` (decision 0079).
    BaseAntiAircraftFire {
        base: BaseId,
        air_group: AirGroupId,
        shots: Fixed,
        chance: Fixed,
        expected: Fixed,
        hits: u32,
        after_attack: bool,
    },
    /// A strike group's bombs fell on a base (decisions 6 and 7 of 0034).
    /// `hits` is the bombs that hit, and `hits_on_aircraft` and
    /// `hits_on_supply` how many of them landed on each; the rest landed on
    /// nothing.
    BaseAttacked {
        air_group: AirGroupId,
        base: BaseId,
        shots: Fixed,
        chance: Fixed,
        expected: Fixed,
        hits: u32,
        hits_on_aircraft: u32,
        hits_on_supply: u32,
        /// Hits that landed on men ashore (decision 0092, 8 and 10): 0 when
        /// the base's side had no men in its hex.
        hits_on_men: u32,
        /// What the draw of decision 0167 gave: the aircraft's type where the
        /// people of the base recognised it, `None` where they did not.
        recognised: Option<AircraftTypeId>,
    },
    /// Hits on aircraft destroyed aircraft of an air group on the ground
    /// (decision 8 of 0034). `remaining` is what the group has ready
    /// afterwards.
    AircraftDestroyedOnGround {
        air_group: AirGroupId,
        base: BaseId,
        aircraft: u32,
        remaining: u32,
    },
    /// Hits on supply destroyed supply of a base (decision 9 of 0034).
    /// `supply_t` is what the base has afterwards, in tons.
    BaseSupplyDestroyed {
        base: BaseId,
        destroyed_t: Fixed,
        supply_t: Fixed,
    },
    /// A base spent supply (decisions 1 to 6 of 0039). `aircraft` is the
    /// aircraft sent on sorties, the aircraft at the base for the day, or
    /// the repairs paid for, by `reason`; `men` the men of its side ashore
    /// in its hex for the reason `Men` (decision 0066, 16), and 0 for every
    /// other reason. `asked_t` is what the rule asked for, `paid_t` what the
    /// stock covered, and `left_t` what the base has afterwards.
    BaseSupplySpent {
        base: BaseId,
        reason: SupplyUse,
        aircraft: u32,
        men: u32,
        asked_t: Fixed,
        paid_t: Fixed,
        left_t: Fixed,
    },
    /// An air group's fatigue changed in a daylight pulse (decisions 4 and 5
    /// of 0036). `amount` is what it actually gained or lost, after the cap
    /// at 100 or the floor at 0; `fatigue` is what it has afterwards.
    AirGroupFatigue {
        air_group: AirGroupId,
        change: FatigueChange,
        amount: Fixed,
        fatigue: Fixed,
    },
    /// The staff stood an air group down at the start of the day, because
    /// its fatigue is above its rest threshold (decision 8 of 0036).
    AirGroupStoodDown {
        air_group: AirGroupId,
        fatigue: Fixed,
        threshold: Fixed,
    },
    /// The staff returned a stood-down air group to flying at the start of
    /// the day, its fatigue being no longer above its rest threshold
    /// (decision 8 of 0036).
    AirGroupReturned {
        air_group: AirGroupId,
        fatigue: Fixed,
        threshold: Fixed,
    },
    /// A task force's fatigue changed at the start of a pulse (decision 2 of
    /// 0037). `amount` is what it actually gained or lost, after the cap at
    /// 100 or the floor at 0; `fatigue` is what it has afterwards.
    TaskForceFatigue {
        task_force: TaskForceId,
        change: ForceFatigueChange,
        amount: Fixed,
        fatigue: Fixed,
    },
    /// A route order turned a task force round between two hexes (decision 5
    /// of 0005, `TaskForce::turn_back`): the hex it left and the hex ahead
    /// swapped and its progress became the miles still to go. `from` and
    /// `toward` are the two hexes as they are afterwards, `progress_nm` is
    /// the progress along that leg, and `position` is the hex the force is in
    /// afterwards (the hex ahead from the midpoint on, so exactly 20 nm is in
    /// `toward`). Added at the end of the enum.
    TaskForceTurnedBack {
        task_force: TaskForceId,
        from: Hex,
        toward: Hex,
        progress_nm: Fixed,
        position: Hex,
    },
    /// A ship took fuel aboard from a base in the loading phase of a pulse
    /// (decision 0045, as amended by 0112: through the hours of the pulse at
    /// the port's fuelling rate). `taken_t` is what it took, `fuel_t` what it
    /// has afterwards and
    /// `base_fuel_t` what the base has left. Nothing is recorded for a ship
    /// that takes nothing. `force_restored` is true on the first refuelling
    /// of a task force that had run out of fuel in the hex of the base: the
    /// force is idle again, with no route, and an order can move it
    /// (decision 6 of 0045).
    ShipRefuelled {
        ship: ShipId,
        task_force: TaskForceId,
        base: BaseId,
        taken_t: Fixed,
        fuel_t: Fixed,
        base_fuel_t: Fixed,
        force_restored: bool,
    },
    /// An air group rolled for running out of fuel after a strike or escort
    /// sortie that went beyond its type's radius (decisions 3 to 6 of
    /// `docs/decisions/0059-aircraft-that-cannot-land-where-they-took-off.md`):
    /// `aircraft` came back from a sortie of one-way distance `distance_nm`,
    /// `radius_nm` being the type's strike radius, each ran out of fuel at
    /// `chance`, and `ditched` of them did. A ditched aircraft is gone from
    /// the group for good. `crews_recovered` is how many of their crews were
    /// picked up and `pilots_lost` the pilots the group lost, where its
    /// pilots are counted (`None` and 0 where they are not). `ready` is what
    /// the group has ready afterwards. Recorded for each such sortie that
    /// rolled, whether or not an aircraft ditched.
    AircraftDitched {
        air_group: AirGroupId,
        sortie: SortieKind,
        distance_nm: Fixed,
        radius_nm: Fixed,
        aircraft: u32,
        chance: Fixed,
        expected: Fixed,
        ditched: u32,
        crews_recovered: Option<u32>,
        pilots_lost: u32,
        ready: u32,
    },
    /// Aircraft of `from` that were in the air when their own deck was lost
    /// (the ship sank, or her flight deck went out) landed at another
    /// carrier or at a base (decisions 7 and 8 of
    /// `docs/decisions/0059-aircraft-that-cannot-land-where-they-took-off.md`).
    /// They joined the group `to` located at `at`, or formed it if `formed`.
    /// `pilots` came with them, where pilots are counted.
    AircraftLandedElsewhere {
        from: AirGroupId,
        to: AirGroupId,
        at: AirGroupLocation,
        aircraft: u32,
        pilots: u32,
        formed: bool,
    },
    /// Aircraft of an air group that were in the air when their own deck was
    /// lost found no other carrier with room and no base within reach, and
    /// came down in the sea at `hex` (decision 9 of 0059). The crew of each
    /// is recovered with the chance `chance`, which is the figure for coming
    /// down beside ships of their own side if `beside_own_force`, and else
    /// the figure for the open sea. `crews_recovered` is none where the
    /// group's pilots are not counted. `ready` is what the group has ready
    /// afterwards.
    AircraftDitchedNoDeck {
        air_group: AirGroupId,
        hex: Hex,
        aircraft: u32,
        beside_own_force: bool,
        chance: Fixed,
        crews_recovered: Option<u32>,
        pilots_lost: u32,
        ready: u32,
    },
    /// What a base held at the end of a day, after the afternoon pulse
    /// (decision 0067): its supply and the ready aircraft of the air groups
    /// at it. Recorded for every base on every day, whatever the objectives
    /// say, and changes no state. `base_open` is counted from it.
    BaseAtDayEnd {
        base: BaseId,
        supply_t: Fixed,
        aircraft_ready: u32,
    },
    /// A forward repair base mended a ship that day (decision 0069): `points`
    /// were spent on her, in the order fire, flotation, then the first
    /// degraded system. `fire_damage` and `flotation_damage` are what she has
    /// afterwards, `restored` the systems put back to working that day (in
    /// system order) and `system_repair_points` the points banked in the
    /// system still being mended.
    ShipRepaired {
        ship: ShipId,
        task_force: TaskForceId,
        base: BaseId,
        points: Fixed,
        fire_damage: Fixed,
        flotation_damage: Fixed,
        restored: Vec<ShipSystem>,
        system_repair_points: Fixed,
    },
    /// A task force left for the yard by order (decision 0069): `ships` are
    /// the ships afloat in number order, each with her days of repair,
    /// `passage_days` the base's passage, `days_away` the passage plus the
    /// largest days of repair and `return_day` the day it is back.
    ForceLeftForYard {
        task_force: TaskForceId,
        base: BaseId,
        passage_days: u32,
        ships: Vec<(ShipId, u32)>,
        days_away: u32,
        return_day: u32,
    },
    /// A task force came back from the yard at the start of its day of
    /// return, its ships repaired, refuelled and rearmed (decision 0069).
    ForceReturnedFromYard {
        task_force: TaskForceId,
        base: BaseId,
        ships: Vec<ShipId>,
    },
    /// Men of a land unit went aboard a ship from the shore (decision 0066,
    /// 8). `aboard` is the men of the unit in the ship afterwards and
    /// `ashore` the unit's men left ashore in `hex`.
    MenEmbarked {
        unit: LandUnitId,
        ship: ShipId,
        task_force: TaskForceId,
        hex: Hex,
        men: u32,
        aboard: u32,
        ashore: u32,
    },
    /// Men of a land unit were landed from a ship (decisions 9 and 10 of
    /// 0066). `aboard` is what is left in the ship and `ashore` the unit's
    /// men ashore in `hex` afterwards.
    MenLanded {
        unit: LandUnitId,
        ship: ShipId,
        task_force: TaskForceId,
        hex: Hex,
        men: u32,
        aboard: u32,
        ashore: u32,
    },
    /// The first man landed in a hex with no base of his side formed one
    /// (decision 0066, 11): `base`, of `side`, in `hex`, with the port of
    /// the hex and nothing else. It is on its own side's map only.
    BaseFormed {
        base: BaseId,
        side: Side,
        hex: Hex,
        task_force: TaskForceId,
    },
    /// A unit was divided when its men would be ashore in two hexes
    /// (decision 0066, 14): the `men` of `from` aboard `task_force`'s ships
    /// became the new unit `unit`, and `from` keeps its men ashore elsewhere.
    UnitDivided {
        from: LandUnitId,
        unit: LandUnitId,
        task_force: TaskForceId,
        men: u32,
    },
    /// The men aboard a ship that sank were lost with her (decision 0066,
    /// 15).
    MenLostWithShip {
        unit: LandUnitId,
        ship: ShipId,
        men: u32,
    },
    /// The men of `side` ashore in `hex` were fed for the day, or not
    /// (decisions 16 and 17 of 0066). `base` is the base that was asked,
    /// `None` where the side has none in the hex. `fed` and `unfed` add up to
    /// `men`.
    MenFed {
        side: Side,
        hex: Hex,
        base: Option<BaseId>,
        men: u32,
        fed: u32,
        unfed: u32,
    },
    /// A unit lost men unfed (decisions 19 and 20 of 0066): `lost`, its part
    /// of the loss in `hex`, and `ashore` what it has left there.
    MenLostUnfed {
        unit: LandUnitId,
        hex: Hex,
        lost: u32,
        ashore: u32,
    },
    /// A land unit has no men left ashore or aboard (decision 0066, 1).
    LandUnitDestroyed {
        unit: LandUnitId,
    },
    /// A load order with a land unit is done (decision 0066, "Settled before
    /// the build"): no man of the unit is left ashore in the hex or no ship
    /// has a berth for one, and nothing remains of the named cargo. `base`
    /// is the base loaded from, `None` off a bare shore, and `cargo` the
    /// cargo named, `None` for none.
    UnitLoadingComplete {
        task_force: TaskForceId,
        unit: LandUnitId,
        base: Option<BaseId>,
        cargo: Option<Cargo>,
        /// Elapsed hours since the scenario began.
        elapsed_hours: Fixed,
    },
    /// The damaged aircraft of an air group, which cannot fly, were written
    /// off at the base it left by a rebase order (decision 0091, 5). They are
    /// lost for good. Recorded before the group's `AirGroupRebased`, and only
    /// if it had damaged aircraft.
    AircraftWrittenOff {
        air_group: AirGroupId,
        base: BaseId,
        aircraft: u32,
    },
    /// An air group moved by order from base `from` to base `to`,
    /// `distance_nm` apart by air, within its type's `transfer_range_nm`
    /// (decision 0091). Its `ready` aircraft and all its pilots went with it
    /// (`pilots`, `None` where they are not counted). It flies nothing that
    /// day.
    AirGroupRebased {
        air_group: AirGroupId,
        from: BaseId,
        to: BaseId,
        distance_nm: Fixed,
        transfer_range_nm: Fixed,
        ready: u32,
        pilots: Option<u32>,
    },
    /// A land unit's fed share went back to 1 at the night's charge, because
    /// none of its men were ashore to be charged (decision 0090, 8). The
    /// share of a unit with men ashore is the `MenFed` of its side and hex.
    FedShareRestored {
        unit: LandUnitId,
    },
    /// The day's fight in a contested hex (decisions 4 and 5 of 0090): both
    /// sides have men ashore there and a unit on attack. Each side attacks
    /// if any of its units there is on attack.
    GroundFight {
        hex: Hex,
        allied_attacks: bool,
        japanese_attacks: bool,
    },
    /// A land unit fired in the day's fight (decisions 6 and 7 of 0090):
    /// `firing_men` of its men ashore fired `shots`, each at `chance`, and
    /// `hits` hit. The breakdown gives the firing men's three factors.
    GroundFire {
        unit: LandUnitId,
        hex: Hex,
        firing_men: u32,
        shots: Fixed,
        chance: Fixed,
        hits: u32,
    },
    /// A land unit lost men to the enemy's hits in the day's fight (decision
    /// 9 of 0090): `lost`, its part of the loss in `hex`, and `ashore` what
    /// it has left there.
    MenLostInFight {
        unit: LandUnitId,
        hex: Hex,
        lost: u32,
        ashore: u32,
    },
    /// A land unit's fatigue changed in the ground combat phase (decision 12
    /// of 0090). `fatigue` is what it has afterwards.
    LandUnitFatigue {
        unit: LandUnitId,
        change: LandFatigueChange,
        fatigue: Fixed,
    },
    /// A unit on attack too tired to go on went back to defend, and its
    /// attack order is spent (decision 14 of 0090).
    LandUnitBackToDefend {
        unit: LandUnitId,
        fatigue: Fixed,
    },
    /// A side was beaten in a hex (decisions 15 and 16 of 0090): `men`, its
    /// men ashore there, against `enemy_men`, the other side's, counted
    /// after the day's losses. `fought` is false for a base with no men,
    /// which falls without a fight.
    SideBeatenAshore {
        side: Side,
        hex: Hex,
        men: u32,
        enemy_men: u32,
        fought: bool,
    },
    /// A base of a beaten side changed side (decisions 18 and 19 of 0090).
    /// It keeps `supply_t` of its supply and `fuel_t` of its fuel, and
    /// `supply_lost_t` and `fuel_lost_t` are lost. It is on both sides' maps
    /// from now on.
    BaseChangedSide {
        base: BaseId,
        hex: Hex,
        from: Side,
        to: Side,
        supply_t: Fixed,
        fuel_t: Fixed,
        supply_lost_t: Fixed,
        fuel_lost_t: Fixed,
    },
    /// An air group at a base that changed side was lost with it, with
    /// `aircraft`, ready and damaged (decision 20 of 0090). It has no place
    /// from now on.
    AirGroupLostWithBase {
        air_group: AirGroupId,
        base: BaseId,
        aircraft: u32,
    },
    /// A ship in port at a base that changed side was lost with it, and is
    /// sunk (decision 21 of 0090).
    ShipLostWithBase {
        ship: ShipId,
        base: BaseId,
    },
    /// A task force of the beaten side whose home port changed side has no
    /// home port (decision 22 of 0090).
    HomePortLost {
        task_force: TaskForceId,
        base: BaseId,
    },
    /// An air group of the winning side whose mission was a strike on a base
    /// that is now its own side's has no mission (decision 22 of 0090).
    StrikeMissionCleared {
        air_group: AirGroupId,
        base: BaseId,
    },
    /// A ship of a force on the bombardment mission fired its guns at an
    /// enemy base in a night's shore bombardment (decision 0092, 2 to 5).
    /// `shells` were fired and spent, leaving `rounds_left`; `weighed_shots`
    /// are the shells times the class's gun damage over the reference gun
    /// damage, each hitting at `chance`. Of the `hits`, the three counts
    /// landed on aircraft, supply and men; the rest landed on nothing.
    BombardmentFire {
        ship: ShipId,
        base: BaseId,
        shells: Fixed,
        weighed_shots: Fixed,
        chance: Fixed,
        expected: Fixed,
        hits: u32,
        hits_on_aircraft: u32,
        hits_on_supply: u32,
        hits_on_men: u32,
        rounds_left: Fixed,
    },
    /// A force bombarded an enemy base in its hex in the night (decision
    /// 0092, 1): its ships' fire, added. It cites each ship's fire; the
    /// aircraft, supply and men destroyed cite it.
    ShoreBombardment {
        task_force: TaskForceId,
        base: BaseId,
        hex: Hex,
        shells: Fixed,
        weighed_shots: Fixed,
        hits: u32,
        hits_on_aircraft: u32,
        hits_on_supply: u32,
        hits_on_men: u32,
    },
    /// A land unit lost men ashore to a bomb or a shell that hit men at a
    /// base (decision 0092, 9 and 20), `ashore` left.
    MenLostToFire {
        unit: LandUnitId,
        base: BaseId,
        hex: Hex,
        lost: u32,
        ashore: u32,
    },
    /// A side was given a ground report of the enemy's men ashore in a hex
    /// (decision 0092, 12 to 16): its estimate there is now `men`, dated
    /// `day`. `replaced` is true if it held one before. Day 0 is a
    /// scenario's opening estimate.
    GroundReportReceived {
        side: Side,
        hex: Hex,
        men: u32,
        replaced: bool,
        /// Where the count comes from (decision 0147, 1).
        grade: GroundGrade,
    },
    /// An air group on the reconnaissance mission flew to `hex` and back
    /// (decision 0147, 5 and 10): one aircraft, `distance_nm` out from
    /// `from`, at a cost of `supply_t` of supply (the cost of one search
    /// sortie of twice the distance). The ground report it gave is the
    /// next event.
    ReconnaissanceFlown {
        air_group: AirGroupId,
        from: AirGroupLocation,
        hex: Hex,
        distance_nm: Fixed,
        supply_t: Fixed,
    },
    /// A ground report put an enemy base formed by a landing on the
    /// reporting side's map (decision 0092, 15).
    BaseFound {
        side: Side,
        base: BaseId,
        hex: Hex,
    },
    /// A land unit marched a day from `from` toward `to` in the ground
    /// combat phase (decision 0094, 4): `days` days marched, counted as
    /// `counted` of the `of` the march takes (decision 4 of 0098: each day
    /// counts as the unit's movement factor, so `counted` is `days` for a
    /// unit whose factor is 1). Its men are still at `from` unless `counted`
    /// has reached `of`.
    MarchDay {
        unit: LandUnitId,
        from: Hex,
        to: Hex,
        days: u32,
        counted: Fixed,
        of: u32,
    },
    /// A march formed a base for its side in the hex it reached, to hold the
    /// supply it carried (decision 0094, 9): no guns, no repairs, and on its
    /// own side's map only.
    MarchBaseFormed {
        base: BaseId,
        side: Side,
        hex: Hex,
        unit: LandUnitId,
    },
    /// A land unit's `men` ashore arrived in `to` from `from`, and its march
    /// order is spent (decision 0094, 4 and 9). `supply_t` is the supply it
    /// carried from its side's bases at `from`, put into `base`; `base` is
    /// `None` when it carried nothing.
    MarchArrived {
        unit: LandUnitId,
        from: Hex,
        to: Hex,
        men: u32,
        supply_t: Fixed,
        base: Option<BaseId>,
    },
    /// A land unit's march to `to` stopped after `days` days marched, and the
    /// unit stays where it is (decision 0094, 3).
    MarchStopped {
        unit: LandUnitId,
        to: Hex,
        days: u32,
        reason: MarchStop,
    },
    /// A task force halted at `hex`, a waypoint of its route marked to act
    /// at, because it has work there (decision 0096, 2): a bombardment, cargo
    /// or both, as `work` says; or one marked to stand at, reached in the
    /// night pulse, to stand there for the night (decision 0124, 2). It
    /// halted at `elapsed_hours` since the scenario began.
    TaskForceHalted {
        task_force: TaskForceId,
        hex: Hex,
        work: HaltWork,
        elapsed_hours: Fixed,
    },
    /// A halted task force steamed on from `hex` at the start of a pulse's
    /// movement, its work done (decision 0096, 4). `work` is what it halted
    /// for; `fired` is whether it bombarded while halted (with `work.bombard`
    /// and not `fired`, no ship of it could fire any longer).
    TaskForceSteamedOn {
        task_force: TaskForceId,
        hex: Hex,
        work: HaltWork,
        fired: bool,
    },
    /// A force stopped at a base of its side gained `loading` (now) for the
    /// landing it prepares at `hex` (decision 0095, 4).
    LoadingGained {
        task_force: TaskForceId,
        hex: Hex,
        loading: Fixed,
    },
    /// A force's loading for `hex` fell from `loading_before` to `loading`
    /// (decision 0095, 5, 8 and 10). A `Spent` loss means the order is done.
    LoadingLost {
        task_force: TaskForceId,
        hex: Hex,
        loading_before: Fixed,
        loading: Fixed,
        reason: LoadingLoss,
    },
    /// A land unit of a force's roster rehearsed a day for the landing at
    /// `hex` and now has `rehearsal` (decision 0095, 4).
    RehearsalGained {
        unit: LandUnitId,
        task_force: TaskForceId,
        hex: Hex,
        rehearsal: Fixed,
    },
    /// A land unit's rehearsal for `hex` fell from `rehearsal_before` to 0
    /// (decision 0095, 7, 8 and 3b of 0099).
    RehearsalLost {
        unit: LandUnitId,
        hex: Hex,
        rehearsal_before: Fixed,
        reason: RehearsalLoss,
    },
    /// A land unit left a force's roster (decision 0095, 7 and 3c of 0099).
    LeftRoster {
        unit: LandUnitId,
        task_force: TaskForceId,
        reason: RosterLeft,
    },
    /// A force that split gave the new part its loading at `hex` and the
    /// roster `units` (decision 12 of 0095).
    PreparationShared {
        parent: TaskForceId,
        new_force: TaskForceId,
        hex: Hex,
        loading: Fixed,
        units: Vec<LandUnitId>,
    },
    /// A land unit trained a day: `days` counted toward the next step of
    /// experience, at `experience` (decision 3 of 0097).
    TrainingDay {
        unit: LandUnitId,
        days: u32,
        experience: u8,
    },
    /// A land unit fired in a fight on a day that counts: `days` counted
    /// toward the next step of experience, at `experience` (decision 5 of
    /// 0097).
    FightDayCounted {
        unit: LandUnitId,
        days: u32,
        experience: u8,
    },
    /// A land unit's experience rose to `experience` (decisions 3 and 5 of
    /// 0097).
    ExperienceRaised {
        unit: LandUnitId,
        experience: u8,
        by: ExperienceSource,
    },
    /// A land unit worked a day at `job` at `base` (decisions 3, 3a and 4 of
    /// 0099): it made `work`, and the base's `total` for the job is then the
    /// work done on its airfield, the day's work on its port or on its
    /// aircraft, or its defences. Added at the end.
    UnitWorked {
        unit: LandUnitId,
        base: BaseId,
        job: Job,
        work: Fixed,
        total: Fixed,
    },
    /// The airfield of `side`'s base was built: its work done reached the
    /// figure (decision 5 of 0099). Added at the end.
    AirfieldBuilt {
        base: BaseId,
        side: Side,
        hex: Hex,
        work_done: Fixed,
    },
    /// Hits on the airfield of `side`'s base added to its damage (decision 3
    /// of 0103): `hits` hits, and `damage` is the damage after, from 0 to
    /// 100. Added at the end.
    AirfieldDamaged {
        base: BaseId,
        side: Side,
        hits: u32,
        damage: Fixed,
    },
    /// An arrival due at an air group's field was held back because the
    /// field takes no aircraft (decision 5 of 0103): its damage is at or
    /// above the level. The arrival waits, whole, for the first night the
    /// field takes aircraft. Recorded on each night it is held back. Added
    /// at the end.
    ArrivalWaiting {
        air_group: AirGroupId,
        base: BaseId,
        aircraft: u32,
        pilots: u32,
        damage: Fixed,
    },
    /// A land unit's work on an airfield lowered its damage by `points` to
    /// `damage` (decision 7 of 0103). Added at the end.
    AirfieldMended {
        unit: LandUnitId,
        base: BaseId,
        points: Fixed,
        damage: Fixed,
    },
    /// A unit of a side beaten in `from` was set to march out of it to `to`
    /// (decision 1 of 0104), with `men` ashore, tired by the defeat:
    /// `fatigue` is what it has afterwards. Added at the end.
    UnitFellBack {
        unit: LandUnitId,
        side: Side,
        from: Hex,
        to: Hex,
        men: u32,
        fatigue: Fixed,
    },
    /// A unit of a side beaten in `hex` with nowhere to fall back to
    /// surrendered (decision 7 of 0104): its `men` ashore are lost. Added at
    /// the end.
    UnitSurrendered {
        unit: LandUnitId,
        side: Side,
        hex: Hex,
        men: u32,
    },
    /// The units of a side beaten in `hex` that had nowhere to fall back to
    /// surrendered, `men` ashore in all (decision 7 of 0104): one event for
    /// the hex, so that what the enemy is told shows no count of units. It
    /// names no unit. Added at the end.
    MenSurrenderedAshore {
        side: Side,
        hex: Hex,
        men: u32,
    },
    /// The attack of `side` in `hex` failed that day (decision 4 of 0105):
    /// it attacked and its weight of fire (`weight`, the sum over its units
    /// of shots times chance, after the limit on engaged men) was below the
    /// other side's (`enemy_weight`). Worked out before any die is rolled.
    /// Added at the end.
    GroundAttackFailed {
        side: Side,
        hex: Hex,
        weight: Fixed,
        enemy_weight: Fixed,
    },
    /// A land unit whose attack was spent fell back from `from` toward `to`
    /// by its order's instruction or its default (decisions 6 and 7 of
    /// 0105), with `men` ashore, tired by `beaten_fatigue`: `fatigue` is what
    /// it has afterwards. Its side was not beaten. Added at the end.
    SpentUnitFellBack {
        unit: LandUnitId,
        side: Side,
        from: Hex,
        to: Hex,
        men: u32,
        fatigue: Fixed,
    },
    /// A land unit's fieldworks changed (decisions 5, 6 and 12 of 0106):
    /// `fieldworks` is what it has afterwards. Added at the end.
    FieldworksChanged {
        unit: LandUnitId,
        change: FieldworksChange,
        fieldworks: Fixed,
    },
    /// Hits on men of `side` in `hex` wore down the defences of its base
    /// there (decision 12 of 0106): `hits` hits, and `defences` is what the
    /// base has afterwards. Added at the end.
    DefencesWorn {
        base: BaseId,
        side: Side,
        hex: Hex,
        hits: u32,
        defences: Fixed,
    },
    /// Ships in port at a base, in no force, were formed into a task force
    /// (decision 12 of 0111), citing the order. Added at the end.
    TaskForceFormed {
        task_force: TaskForceId,
        side: Side,
        base: BaseId,
        ships: Vec<ShipId>,
    },
    /// A force was joined to another at a base (decision 13 of 0111), citing
    /// the order: `ships` moved from `joined`, which ceased to be, into
    /// `into`. Added at the end.
    TaskForceJoined {
        joined: TaskForceId,
        into: TaskForceId,
        base: BaseId,
        ships: Vec<ShipId>,
    },
    /// A force was stood down at a base (decision 14 of 0111), citing the
    /// order: `ships` lie in port there in no force, and the force ceased to
    /// be. Added at the end.
    TaskForceStoodDownInPort {
        task_force: TaskForceId,
        base: BaseId,
        ships: Vec<ShipId>,
    },
    /// A load the commander ordered may take `below_t` tons of the base's
    /// stock below its reserve (decision 9 of 0111): the answer to the
    /// order, recorded after its acceptance and citing it. Added at the end.
    LoadBelowReserve {
        task_force: TaskForceId,
        base: BaseId,
        cargo: Cargo,
        below_t: Fixed,
        reserve_t: Fixed,
    },
    /// A ship's refuelling stopped at the fuel reserve of its side's bases in
    /// the hex (decision 7 of 0111): it still lacks `short_t` tons, and those
    /// bases hold fuel no longer above their reserves. Recorded after the
    /// ship's refuelling, if it took any, in the pulse it is first held and
    /// not again while nothing changes (decision 0112). Added at the end.
    RefuellingHeldAtReserve {
        ship: ShipId,
        task_force: TaskForceId,
        hex: Hex,
        short_t: Fixed,
    },
    /// A load the staff ordered is complete because the base holds no more
    /// of the cargo above its reserve (decisions 4 and 8 of 0111), with
    /// `aboard_t` of it aboard the force, short of its weight or its room.
    /// Recorded after the load's completion, citing it. Added at the end.
    LoadHeldAtReserve {
        task_force: TaskForceId,
        base: BaseId,
        cargo: Cargo,
        aboard_t: Fixed,
        reserve_t: Fixed,
    },
    /// A command was taken in and applied (decision 0107). The root record
    /// of the change it made: no causes. Added at the end (decision 0107).
    CommandTaken {
        side: Side,
        issuer: Issuer,
        command: CommandKind,
    },
    /// A command was refused, with the reason; nothing changed (decision
    /// 0107). No causes. Added at the end (decision 0107).
    CommandRefused {
        side: Side,
        issuer: Issuer,
        command: CommandKind,
        reason: CommandRefusal,
    },
    /// A plan was drafted for an intent, when its number was first set or
    /// set again after its plan ended. Cites the `CommandTaken`. Added at
    /// the end (decision 0107).
    PlanDrafted {
        side: Side,
        plan: PlanId,
        intent: IntentId,
    },
    /// A plan changed state. Cites the `CommandTaken`. Added at the end
    /// (decision 0107).
    PlanStateChanged {
        side: Side,
        plan: PlanId,
        from: PlanState,
        to: PlanState,
    },
    /// A plan that is approved was amended by a change to its intent or one
    /// of its limits; `number` is the amendment's. Cites the `CommandTaken`.
    /// Added at the end (decision 0107).
    PlanAmended {
        side: Side,
        plan: PlanId,
        number: u32,
    },
    /// The reason for a staff decision: for the order it handed in, for a
    /// command it handed in (`command`, the command as handed in), or, with
    /// neither, for giving no order (decision 0107). Cites the order and its
    /// acceptance or refusal, if there is an order, or the command's
    /// `CommandTaken` or `CommandRefused`. Added at the end (decision 0107).
    StaffReason {
        side: Side,
        order: Option<OrderId>,
        command: Option<Box<CommandKind>>,
        reason: Box<Reason>,
    },
    /// A side's men, fighting men standing in a hex where it holds an
    /// estimate of the enemy ashore, found no enemy man there (decision
    /// 0110, item 10): the estimate is cleared. `men` and `reported_day` are
    /// the estimate cleared. Cites the report that made the estimate and
    /// the day's fight there, if there was one. Added at the end (decision
    /// 0110).
    GroundFoundEmpty {
        side: Side,
        hex: Hex,
        men: u32,
        reported_day: u32,
    },
    /// The commander's order to a unit in an intent's forces limit held it
    /// from the staff, at step 2 of the day (decision 0107, step 3, ruling
    /// 23). `target` is what is held. Cites the order and its acceptance.
    /// Added at the end (decision 0107).
    HeldByOrder {
        side: Side,
        target: HoldTarget,
        order: OrderId,
    },
    /// A notice the staff handed in beside its orders (decision 0107, ruling
    /// 34): the plan, the run where there is one, and the kind. Cites the
    /// `StaffReason` it rests on, if its basis resolves to one. Added at the
    /// end (decision 0107, step 4). `intent` names the intent whose words the
    /// notice says: the end of a `met` notice of "take a place" (the ruling
    /// on decision 0142's reading 4); `None` for every other notice.
    StaffNotice {
        side: Side,
        kind: NoticeKind,
        plan: PlanId,
        run: Option<u32>,
        intent: Option<IntentId>,
        /// What a pursuit notice says (decision 0214): the force, the
        /// order it would leave and the find. `None` for every other
        /// notice. Added at the end.
        pursuit: Option<crate::command::PursuitNotice>,
    },
    /// An interrupt fired at the morning's test (decision 0107, rulings 31
    /// to 33). A set one carries its condition and the figure it was
    /// tested on; a standing one carries neither. Cites the command that set
    /// it, the staff's notices of the morning, or the event that sank the
    /// ship. Added at the end (decision 0107, step 4).
    InterruptFired {
        side: Side,
        interrupt: InterruptRef,
        condition: Option<InterruptCondition>,
        figure: Option<Fixed>,
    },
    /// The staff formed a sub-intent (decision 0117, 3): `intent` is its
    /// number, `parent` the intent it stands under. Cites the
    /// `CommandTaken` of the staff's `form_intent`. Added at the end (step 2
    /// of stage 2).
    IntentFormed {
        side: Side,
        intent: IntentId,
        parent: IntentId,
    },
    /// An intent was withdrawn because an intent above it was (decision
    /// 0117, 6): `parent` is the one it stood under. Cites the `CommandTaken`
    /// of the withdrawal. Added at the end.
    IntentWithdrawn {
        side: Side,
        intent: IntentId,
        parent: IntentId,
    },
    /// An escort took its station for the pulse (decision 0121, 3): in
    /// station, its charge's route still to steam, cut where the charge
    /// halts, at the charge's speed in `knots`; out of station, its
    /// charge's hex at cruise (`knots` none). An empty `waypoints` is an
    /// escort in station whose charge does not steam this pulse: it stops
    /// where it is. Recorded only where the escort's route, speed or status
    /// changes. Cites the escort order and, in station, the charge's route
    /// order. Added at the end (decision 0121).
    StationKept {
        escort: TaskForceId,
        charge: TaskForceId,
        in_station: bool,
        waypoints: Vec<Hex>,
        knots: Option<Fixed>,
    },
    /// An escort's charge has no ship afloat, was stood down or was joined
    /// into another force (decision 0121, 7): the escort has no charge from
    /// this moment and stops where it is, at `hex`. Cites the escort order
    /// and the event that emptied the charge, where there is one. Added at
    /// the end (decision 0121).
    ChargeLost {
        escort: TaskForceId,
        charge: TaskForceId,
        hex: Hex,
    },
    /// The meeting of `enemy` with `charge` at `hex` was not held (decision
    /// 0121, 4 and 5): not every escort in station on the charge, `escorts`,
    /// ended its fight with `enemy` sunk or as the force that left it. The
    /// charge is not met by that enemy again in the pulse. Cites each
    /// escort's order and the end of each escort's fight. The umpire's and
    /// the charge's side's; the enemy is told nothing. Added at the end
    /// (decision 0121).
    MeetingNotHeld {
        charge: TaskForceId,
        enemy: TaskForceId,
        hex: Hex,
        escorts: Vec<TaskForceId>,
    },
    /// A ship of `task_force` took ammunition from `base` in a pulse
    /// (decision 0122, 9): whole main battery `rounds` and `torpedoes`,
    /// `taken_t` tons in all. `rounds_aboard` and `tubes_fired` are the
    /// ship's after it, `base_ammunition_t` the base's. Caused as a
    /// refuelling is: by the force's arrival or halt where there is one.
    /// The side's own. Added at the end (decision 0122).
    ShipRearmed {
        ship: ShipId,
        task_force: TaskForceId,
        base: BaseId,
        rounds: u32,
        torpedoes: u32,
        taken_t: Fixed,
        rounds_aboard: Fixed,
        tubes_fired: u32,
        base_ammunition_t: Fixed,
    },
    /// A command passed a report on (decision 0135, 21): the report's event,
    /// the sender and, for each listener in the order it was drawn, the
    /// direction and whether it arrives on time, a pulse late or never. The
    /// report's own event is its cause. The side's own. Added at the end
    /// (decision 0135).
    ReportSent {
        side: Side,
        sender: CommandRef,
        report: EventId,
        listeners: Vec<ReportListener>,
    },
    /// A command's state report, of the moment `as_of`, arrives at the area
    /// command a pulse late (decision 0136, 6). It carries the draw's
    /// breakdown and no cause. The side's own. Added at the end (decision
    /// 0136).
    StateReportLate {
        side: Side,
        command: CommandRef,
        as_of: AsOf,
    },
    /// A command's state report, of the moment `as_of`, never arrives at the
    /// area command (decision 0136, 6). As [`EventKind::StateReportLate`].
    StateReportLost {
        side: Side,
        command: CommandRef,
        as_of: AsOf,
    },
    /// The world was built from a picture of the evening of `day` (decision
    /// 0170, 7): the first event of such a world, event 1, on that day with
    /// no pulse and no cause. What the state and belief held as reported or
    /// done before the picture cites it. Both sides see it. Added at the
    /// end.
    BuiltFromPicture {
        day: u32,
    },
    /// A task force, with its ships, the air groups aboard them and the land
    /// units aboard them, came onto the map at `hex` on its day (decision
    /// 0195, 2 and 3): `base` is a base of its side in the hex, if any. The
    /// lists are in number order. It cites no cause. Its own side's, whole.
    /// Added at the end.
    ForceEntered {
        force: TaskForceId,
        side: Side,
        hex: Hex,
        base: Option<BaseId>,
        ships: Vec<ShipId>,
        air_groups: Vec<AirGroupId>,
        land_units: Vec<LandUnitId>,
    },
    /// A land unit or an air group came onto the map at its base on its day
    /// (decision 0195, 2): `unit` is a land unit or an air group, `men` the
    /// men ashore of a land unit and `aircraft` the ready and damaged
    /// aircraft of a group, 0 for the other. It cites no cause. Its own
    /// side's, whole. Added at the end.
    UnitEntered {
        unit: EntityRef,
        side: Side,
        base: BaseId,
        hex: Hex,
        men: u32,
        aircraft: u32,
    },
    /// A land unit or an air group could not enter at its base on `day`, the
    /// day the scenario named, or on a day after it (decision 0195, 2 and
    /// 16): the base is not its side's, or takes no aircraft and the group
    /// has some. Recorded on each day it waits, with no cause. Its own
    /// side's, whole. Added at the end.
    EntryWaiting {
        unit: EntityRef,
        side: Side,
        base: BaseId,
        day: u32,
        wait: crate::entering::EntryWait,
    },
    /// A supply line gave its base its tons of supply and fuel on a night
    /// of its days, the base being the line's side's (decision 0196, 2 and
    /// 3): one for each line that gave. `side` is the line's side, which
    /// holds the base. It cites no cause: nothing in the game led to it
    /// (rule 11). Its own side's, whole; the enemy is told nothing. Added at
    /// the end.
    StockReceived {
        base: BaseId,
        side: Side,
        supply_t: Fixed,
        fuel_t: Fixed,
    },
    /// Replacement men from a base's stream joined a land unit ashore there
    /// on a night (decision 0197, 3, 8 and 12): `men` joined, the unit has
    /// `men_after` ashore and aboard together, its experience went from
    /// `experience_before` to `experience_after` (one level less on the night
    /// its count of men joined reached its share of the establishment, else
    /// unchanged), and `joined` is its count after the night, 0 when it lost
    /// a level. It cites no cause, as a stream of aircraft cites none. Its
    /// own side's, whole; the enemy is told nothing. Added at the end.
    MenJoined {
        unit: LandUnitId,
        side: Side,
        base: BaseId,
        men: u32,
        men_after: u32,
        experience_before: u8,
        experience_after: u8,
        joined: u32,
    },
    /// A ship of `task_force` took `tons` of fuel at sea from `oiler`, a
    /// ship with the mark of the force it fuels from, `giver` (decision
    /// 0201, 17): `fuel_t` is the ship's fuel after it, `oiler_cargo_fuel_t`
    /// the oiler's cargo fuel left, and `restored` whether it set a force
    /// out of fuel idle (15). Its breakdown holds what she lacked, what the
    /// oiler held, the hours she had the lines and their rate. Cites the
    /// order. The side's own. Added at the end.
    ShipFuelledAtSea {
        ship: ShipId,
        task_force: TaskForceId,
        oiler: ShipId,
        giver: TaskForceId,
        tons: Fixed,
        fuel_t: Fixed,
        oiler_cargo_fuel_t: Fixed,
        restored: bool,
    },
    /// No fuel passed at sea between `task_force` and `giver` at `hex` in
    /// the pulse: a raid attacked a ship of either, or either fought a
    /// surface action (decision 0201, 10 and 17). Cites the order and the
    /// attacks. The side's own. Added at the end.
    FuellingBrokenOff {
        task_force: TaskForceId,
        giver: TaskForceId,
        hex: Hex,
    },
    /// The order of `task_force` to fuel from `giver` ended, for `reason`
    /// (decision 0201, 7 and 17). Cites the order, and the order or event
    /// that ended it where there is one. The side's own. Added at the end.
    FuelOrderEnded {
        task_force: TaskForceId,
        giver: TaskForceId,
        reason: FuelOrderEnd,
    },
    /// The first landing of men at `base` put the rations of the men aboard
    /// the ships of `task_force` into its stock (decision 0205, 4, 5 and 10):
    /// the `men` aboard at the moment its first man landed there, `nights` of
    /// their daily charge, `supply_t` in all; a base is given them once.
    /// Recorded only where the hunger figures are given. Cites the
    /// `BaseFormed` where the landing formed the base, and the unload order
    /// and the force's arrival where the base stood. The side's own, whole;
    /// the enemy is told nothing. Added at the end.
    RationsLanded {
        base: BaseId,
        side: Side,
        task_force: TaskForceId,
        men: u32,
        nights: Fixed,
        supply_t: Fixed,
    },
    /// A land unit's nights of hunger changed at the night's charge
    /// (decision 0205, 7 and 10): from `before` to `nights`, by one. Recorded
    /// only when the count changes, and only where the hunger figures are
    /// given. Cites the night's `MenFed` of its side and hex. The unit's own
    /// side's, whole; the enemy is told nothing. Added at the end.
    HungerNightsChanged {
        unit: LandUnitId,
        hex: Hex,
        before: u32,
        nights: u32,
    },
    /// What `side`'s men at grips in the day's fight at `hex` cost (decision
    /// 0209, 3 to 6 and 14): `men_at_grips`, a quantity not cut to a whole
    /// man; `wanted_t`, their want; what was charged of it from the
    /// ammunition and from the supply of the side's bases in the hex; and
    /// `supplied_share`, what was charged over what was wanted. Recorded
    /// for each side in a fight that wanted anything, only where the figures
    /// of 0209 are given, after the fight's event and before any unit's
    /// fire, the Allied side first. Cites the fight. The side's own, whole;
    /// the enemy is told nothing. Added at the end.
    FightSupplyCharged {
        side: Side,
        hex: Hex,
        men_at_grips: Fixed,
        wanted_t: Fixed,
        from_ammunition_t: Fixed,
        from_supply_t: Fixed,
        supplied_share: Fixed,
    },
    /// A base spent ammunition on its side's men at grips in a day's fight
    /// in its hex (decision 0209, 5 and 14): `asked_t` what was asked of it,
    /// `paid_t` what its stock covered, `left_t` what it has afterwards.
    /// Recorded only for a base with ammunition that was asked something.
    /// Cites the side's `FightSupplyCharged`. The base's side's own, whole;
    /// the enemy is told nothing. Added at the end.
    BaseAmmunitionSpent {
        base: BaseId,
        asked_t: Fixed,
        paid_t: Fixed,
        left_t: Fixed,
    },
    /// A force began to land men at `base` and put days of fighting into its
    /// ammunition (decision 0209, 10 and 16): the `men` aboard the ships of
    /// `task_force` at that moment, `days` of fighting at the tons for a man
    /// at grips, `ammunition_t` in all. Given at every landing, not once for
    /// a base. Recorded only where the figures of 0209 are given and it
    /// puts something down. Cites what the rations' record cites: the
    /// `BaseFormed` where the landing formed the base, and the unload order
    /// and the force's arrival where the base stood. The side's own, whole;
    /// the enemy is told nothing. Added at the end.
    AmmunitionLanded {
        base: BaseId,
        side: Side,
        task_force: TaskForceId,
        men: u32,
        days: Fixed,
        ammunition_t: Fixed,
    },
}

impl EventKind {
    /// The entities this event is about.
    pub fn entities(&self) -> Vec<EntityRef> {
        match self {
            EventKind::OrderAccepted { target, .. } | EventKind::OrderRejected { target, .. } => {
                vec![*target]
            }
            EventKind::TaskForceSteamed { task_force, .. }
            | EventKind::TaskForceArrived { task_force, .. }
            | EventKind::TaskForceOutOfFuel { task_force, .. } => {
                vec![EntityRef::TaskForce(*task_force)]
            }
            EventKind::FuelBurned {
                ship, task_force, ..
            } => vec![EntityRef::Ship(*ship), EntityRef::TaskForce(*task_force)],
            EventKind::ShipRefuelled {
                ship,
                task_force,
                base,
                ..
            } => vec![
                EntityRef::Ship(*ship),
                EntityRef::TaskForce(*task_force),
                EntityRef::Base(*base),
            ],
            EventKind::CargoLoaded {
                ship,
                task_force,
                base,
                ..
            }
            | EventKind::CargoUnloaded {
                ship,
                task_force,
                base,
                ..
            } => vec![
                EntityRef::Ship(*ship),
                EntityRef::TaskForce(*task_force),
                EntityRef::Base(*base),
            ],
            EventKind::LoadingComplete {
                task_force, base, ..
            }
            | EventKind::UnloadingComplete {
                task_force, base, ..
            } => vec![EntityRef::TaskForce(*task_force), EntityRef::Base(*base)],
            EventKind::SearchFlown {
                air_group, from, ..
            } => std::iter::once(EntityRef::AirGroup(*air_group))
                .chain(place_entity(*from))
                .collect(),
            EventKind::AirGroupLost {
                air_group, ship, ..
            } => vec![EntityRef::AirGroup(*air_group), EntityRef::Ship(*ship)],
            EventKind::StrikeLaunched {
                air_group, contact, ..
            } => vec![
                EntityRef::AirGroup(*air_group),
                EntityRef::Contact(*contact),
            ],
            EventKind::StrikeFoundNothing { air_group, .. } => {
                vec![EntityRef::AirGroup(*air_group)]
            }
            EventKind::ContactNotThere { contact, .. } => vec![EntityRef::Contact(*contact)],
            EventKind::BaseStrikeLaunched {
                air_group, base, ..
            }
            | EventKind::BaseAttacked {
                air_group, base, ..
            }
            | EventKind::AircraftDestroyedOnGround {
                air_group, base, ..
            }
            | EventKind::BaseAntiAircraftFire {
                air_group, base, ..
            } => vec![EntityRef::AirGroup(*air_group), EntityRef::Base(*base)],
            EventKind::BaseSupplyDestroyed { base, .. }
            | EventKind::BaseSupplySpent { base, .. }
            | EventKind::BaseAtDayEnd { base, .. } => vec![EntityRef::Base(*base)],
            EventKind::DamagedAircraftDestroyedOnGround {
                air_group, base, ..
            } => vec![EntityRef::AirGroup(*air_group), EntityRef::Base(*base)],
            EventKind::AircraftLandedElsewhere { from, to, at, .. } => {
                [EntityRef::AirGroup(*from), EntityRef::AirGroup(*to)]
                    .into_iter()
                    .chain(place_entity(*at))
                    .collect()
            }
            EventKind::AircraftDitchedNoDeck { air_group, .. }
            | EventKind::AircraftWorn { air_group, .. }
            | EventKind::AircraftDitched { air_group, .. }
            | EventKind::AirGroupFatigue { air_group, .. }
            | EventKind::AirGroupStoodDown { air_group, .. }
            | EventKind::AirGroupReturned { air_group, .. } => {
                vec![EntityRef::AirGroup(*air_group)]
            }
            EventKind::AircraftRepaired { air_group, at, .. } => {
                std::iter::once(EntityRef::AirGroup(*air_group))
                    .chain(place_entity(*at))
                    .collect()
            }
            EventKind::AirAttack {
                air_group,
                task_force,
                ship,
                ..
            } => vec![
                EntityRef::AirGroup(*air_group),
                EntityRef::TaskForce(*task_force),
                EntityRef::Ship(*ship),
            ],
            EventKind::EscortsSent {
                air_group,
                with_air_group,
                ..
            } => vec![
                EntityRef::AirGroup(*air_group),
                EntityRef::AirGroup(*with_air_group),
            ],
            EventKind::AirFight { firer, target, .. } => {
                vec![EntityRef::AirGroup(*firer), EntityRef::AirGroup(*target)]
            }
            EventKind::AntiAircraftFire {
                task_force,
                air_group,
                ..
            } => vec![
                EntityRef::TaskForce(*task_force),
                EntityRef::AirGroup(*air_group),
            ],
            EventKind::AircraftLost { air_group, .. } | EventKind::PilotsLost { air_group, .. } => {
                vec![EntityRef::AirGroup(*air_group)]
            }
            EventKind::AircraftArrived { air_group, .. } => vec![EntityRef::AirGroup(*air_group)],
            EventKind::ReplacementsReceived {
                air_group, base, ..
            } => vec![EntityRef::AirGroup(*air_group), EntityRef::Base(*base)],
            EventKind::SearchAttempt {
                air_group,
                task_force,
                ..
            } => vec![
                EntityRef::AirGroup(*air_group),
                EntityRef::TaskForce(*task_force),
            ],
            EventKind::ReportReceived { contact, .. }
            | EventKind::ContactAged { contact, .. }
            | EventKind::ContactDropped { contact, .. } => vec![EntityRef::Contact(*contact)],
            EventKind::EngagementBegun { a, b, .. }
            | EventKind::ForcesPassedUnseen { a, b, .. }
            | EventKind::EngagementEnded { a, b, .. } => {
                vec![EntityRef::TaskForce(*a), EntityRef::TaskForce(*b)]
            }
            EventKind::SightingRoll {
                task_force, enemy, ..
            }
            | EventKind::ForceSurprised {
                task_force, enemy, ..
            }
            | EventKind::ForceSlippedAway {
                task_force, enemy, ..
            } => vec![
                EntityRef::TaskForce(*task_force),
                EntityRef::TaskForce(*enemy),
            ],
            EventKind::Salvo { firer, target, .. } => {
                vec![EntityRef::Ship(*firer), EntityRef::Ship(*target)]
            }
            EventKind::ShipDamaged { ship, .. } | EventKind::SystemHit { ship, .. } => {
                vec![EntityRef::Ship(*ship)]
            }
            EventKind::ShipSunk {
                ship, task_force, ..
            } => vec![EntityRef::Ship(*ship), EntityRef::TaskForce(*task_force)],
            EventKind::ForceBecameAlert { task_force, .. }
            | EventKind::ForceBrokeOff { task_force, .. } => {
                vec![EntityRef::TaskForce(*task_force)]
            }
            // A side is not an entity, and the claim names no enemy unit: it
            // names the side's own units that claim, in the order of its list
            // (decision 0180, on the architect's word), and nothing else.
            EventKind::ClaimReceived { by_unit, .. } => {
                by_unit.iter().map(|line| line.unit).collect()
            }
            EventKind::ShipsDetached {
                parent,
                new_force,
                ships,
                escorts,
                ..
            } => {
                let mut entities = vec![
                    EntityRef::TaskForce(*parent),
                    EntityRef::TaskForce(*new_force),
                ];
                entities.extend(
                    ships
                        .iter()
                        .chain(escorts)
                        .map(|ship| EntityRef::Ship(*ship)),
                );
                entities
            }
            EventKind::TaskForceFatigue { task_force, .. }
            | EventKind::TaskForceTurnedBack { task_force, .. } => {
                vec![EntityRef::TaskForce(*task_force)]
            }
            EventKind::ForceRetired { task_force } | EventKind::ForceStoodDown { task_force } => {
                vec![EntityRef::TaskForce(*task_force)]
            }
            EventKind::ForceAlertedByContact {
                task_force,
                contact,
            } => vec![
                EntityRef::TaskForce(*task_force),
                EntityRef::Contact(*contact),
            ],
            EventKind::AlarmRoll {
                task_force,
                by_a,
                by_b,
                ..
            } => vec![
                EntityRef::TaskForce(*task_force),
                EntityRef::TaskForce(*by_a),
                EntityRef::TaskForce(*by_b),
            ],
            EventKind::ShipRepaired {
                ship,
                task_force,
                base,
                ..
            } => vec![
                EntityRef::Ship(*ship),
                EntityRef::TaskForce(*task_force),
                EntityRef::Base(*base),
            ],
            EventKind::ForceLeftForYard {
                task_force,
                base,
                ships,
                ..
            } => {
                let mut entities = vec![EntityRef::TaskForce(*task_force), EntityRef::Base(*base)];
                entities.extend(ships.iter().map(|(ship, _)| EntityRef::Ship(*ship)));
                entities
            }
            EventKind::ForceReturnedFromYard {
                task_force,
                base,
                ships,
            } => {
                let mut entities = vec![EntityRef::TaskForce(*task_force), EntityRef::Base(*base)];
                entities.extend(ships.iter().map(|ship| EntityRef::Ship(*ship)));
                entities
            }
            EventKind::MenEmbarked {
                unit,
                ship,
                task_force,
                ..
            }
            | EventKind::MenLanded {
                unit,
                ship,
                task_force,
                ..
            } => vec![
                EntityRef::LandUnit(*unit),
                EntityRef::Ship(*ship),
                EntityRef::TaskForce(*task_force),
            ],
            EventKind::BaseFormed {
                base, task_force, ..
            } => vec![EntityRef::Base(*base), EntityRef::TaskForce(*task_force)],
            EventKind::UnitDivided {
                from,
                unit,
                task_force,
                ..
            } => vec![
                EntityRef::LandUnit(*from),
                EntityRef::LandUnit(*unit),
                EntityRef::TaskForce(*task_force),
            ],
            EventKind::MenLostWithShip { unit, ship, .. } => {
                vec![EntityRef::LandUnit(*unit), EntityRef::Ship(*ship)]
            }
            EventKind::MenFed { base, .. } => base.iter().map(|b| EntityRef::Base(*b)).collect(),
            EventKind::MenLostUnfed { unit, .. } | EventKind::LandUnitDestroyed { unit } => {
                vec![EntityRef::LandUnit(*unit)]
            }
            EventKind::UnitLoadingComplete {
                task_force,
                unit,
                base,
                ..
            } => {
                let mut entities = vec![
                    EntityRef::TaskForce(*task_force),
                    EntityRef::LandUnit(*unit),
                ];
                entities.extend(base.iter().map(|b| EntityRef::Base(*b)));
                entities
            }
            EventKind::AircraftWrittenOff {
                air_group, base, ..
            } => vec![EntityRef::AirGroup(*air_group), EntityRef::Base(*base)],
            EventKind::AirGroupRebased {
                air_group,
                from,
                to,
                ..
            } => vec![
                EntityRef::AirGroup(*air_group),
                EntityRef::Base(*from),
                EntityRef::Base(*to),
            ],
            EventKind::FedShareRestored { unit }
            | EventKind::GroundFire { unit, .. }
            | EventKind::MenLostInFight { unit, .. }
            | EventKind::LandUnitFatigue { unit, .. }
            | EventKind::LandUnitBackToDefend { unit, .. } => vec![EntityRef::LandUnit(*unit)],
            EventKind::GroundFight { .. }
            | EventKind::SideBeatenAshore { .. }
            | EventKind::MenSurrenderedAshore { .. }
            | EventKind::GroundAttackFailed { .. } => Vec::new(),
            EventKind::SpentUnitFellBack { unit, .. }
            | EventKind::FieldworksChanged { unit, .. } => vec![EntityRef::LandUnit(*unit)],
            EventKind::DefencesWorn { base, .. } => vec![EntityRef::Base(*base)],
            EventKind::TaskForceFormed {
                task_force,
                base,
                ships,
                ..
            }
            | EventKind::TaskForceStoodDownInPort {
                task_force,
                base,
                ships,
            } => {
                let mut entities = vec![EntityRef::TaskForce(*task_force), EntityRef::Base(*base)];
                entities.extend(ships.iter().map(|ship| EntityRef::Ship(*ship)));
                entities
            }
            EventKind::TaskForceJoined {
                joined,
                into,
                base,
                ships,
            } => {
                let mut entities = vec![
                    EntityRef::TaskForce(*joined),
                    EntityRef::TaskForce(*into),
                    EntityRef::Base(*base),
                ];
                entities.extend(ships.iter().map(|ship| EntityRef::Ship(*ship)));
                entities
            }
            EventKind::LoadBelowReserve {
                task_force, base, ..
            }
            | EventKind::LoadHeldAtReserve {
                task_force, base, ..
            } => vec![EntityRef::TaskForce(*task_force), EntityRef::Base(*base)],
            EventKind::RefuellingHeldAtReserve {
                ship, task_force, ..
            } => vec![EntityRef::Ship(*ship), EntityRef::TaskForce(*task_force)],
            // The side's command state is not an entity of the world.
            EventKind::CommandTaken { .. }
            | EventKind::CommandRefused { .. }
            | EventKind::PlanDrafted { .. }
            | EventKind::PlanStateChanged { .. }
            | EventKind::PlanAmended { .. }
            | EventKind::StaffReason { .. }
            | EventKind::HeldByOrder { .. }
            | EventKind::StaffNotice { .. }
            | EventKind::InterruptFired { .. }
            | EventKind::IntentFormed { .. }
            | EventKind::IntentWithdrawn { .. } => Vec::new(),
            EventKind::StationKept { escort, charge, .. }
            | EventKind::ChargeLost { escort, charge, .. } => {
                vec![EntityRef::TaskForce(*escort), EntityRef::TaskForce(*charge)]
            }
            EventKind::ShipRearmed {
                ship,
                task_force,
                base,
                ..
            } => vec![
                EntityRef::Ship(*ship),
                EntityRef::TaskForce(*task_force),
                EntityRef::Base(*base),
            ],
            EventKind::ReportSent {
                sender, listeners, ..
            } => std::iter::once(*sender)
                .chain(listeners.iter().map(|listener| listener.command))
                .filter_map(command_entity)
                .collect(),
            EventKind::StateReportLate { command, .. }
            | EventKind::StateReportLost { command, .. } => {
                command_entity(*command).into_iter().collect()
            }
            // About the whole world, and no unit of it (decision 0170, 7).
            EventKind::BuiltFromPicture { .. } => Vec::new(),
            EventKind::MeetingNotHeld {
                charge, escorts, ..
            } => std::iter::once(*charge)
                .chain(escorts.iter().copied())
                .map(EntityRef::TaskForce)
                .collect(),
            EventKind::GroundFoundEmpty { .. } => Vec::new(),
            EventKind::BaseChangedSide { base, .. } => vec![EntityRef::Base(*base)],
            EventKind::AirGroupLostWithBase {
                air_group, base, ..
            }
            | EventKind::StrikeMissionCleared { air_group, base } => {
                vec![EntityRef::AirGroup(*air_group), EntityRef::Base(*base)]
            }
            EventKind::ShipLostWithBase { ship, base } => {
                vec![EntityRef::Ship(*ship), EntityRef::Base(*base)]
            }
            EventKind::HomePortLost { task_force, base } => {
                vec![EntityRef::TaskForce(*task_force), EntityRef::Base(*base)]
            }
            EventKind::BombardmentFire { ship, base, .. } => {
                vec![EntityRef::Ship(*ship), EntityRef::Base(*base)]
            }
            EventKind::ShoreBombardment {
                task_force, base, ..
            } => vec![EntityRef::TaskForce(*task_force), EntityRef::Base(*base)],
            EventKind::MenLostToFire { unit, base, .. } => {
                vec![EntityRef::LandUnit(*unit), EntityRef::Base(*base)]
            }
            EventKind::GroundReportReceived { .. } => Vec::new(),
            EventKind::ReconnaissanceFlown {
                air_group, from, ..
            } => std::iter::once(EntityRef::AirGroup(*air_group))
                .chain(place_entity(*from))
                .collect(),
            EventKind::BaseFound { base, .. } => vec![EntityRef::Base(*base)],
            EventKind::TaskForceHalted { task_force, .. }
            | EventKind::TaskForceSteamedOn { task_force, .. } => {
                vec![EntityRef::TaskForce(*task_force)]
            }
            EventKind::MarchDay { unit, .. } | EventKind::MarchStopped { unit, .. } => {
                vec![EntityRef::LandUnit(*unit)]
            }
            EventKind::MarchBaseFormed { base, unit, .. } => {
                vec![EntityRef::Base(*base), EntityRef::LandUnit(*unit)]
            }
            EventKind::MarchArrived { unit, base, .. } => {
                let mut entities = vec![EntityRef::LandUnit(*unit)];
                entities.extend(base.iter().map(|b| EntityRef::Base(*b)));
                entities
            }
            EventKind::LoadingGained { task_force, .. }
            | EventKind::LoadingLost { task_force, .. } => {
                vec![EntityRef::TaskForce(*task_force)]
            }
            EventKind::RehearsalGained {
                unit, task_force, ..
            }
            | EventKind::LeftRoster {
                unit, task_force, ..
            } => vec![
                EntityRef::LandUnit(*unit),
                EntityRef::TaskForce(*task_force),
            ],
            EventKind::RehearsalLost { unit, .. }
            | EventKind::TrainingDay { unit, .. }
            | EventKind::FightDayCounted { unit, .. }
            | EventKind::ExperienceRaised { unit, .. } => vec![EntityRef::LandUnit(*unit)],
            EventKind::UnitWorked { unit, base, .. } => {
                vec![EntityRef::LandUnit(*unit), EntityRef::Base(*base)]
            }
            EventKind::AirfieldBuilt { base, .. } | EventKind::AirfieldDamaged { base, .. } => {
                vec![EntityRef::Base(*base)]
            }
            EventKind::ArrivalWaiting {
                air_group, base, ..
            } => vec![EntityRef::AirGroup(*air_group), EntityRef::Base(*base)],
            EventKind::AirfieldMended { unit, base, .. } => {
                vec![EntityRef::LandUnit(*unit), EntityRef::Base(*base)]
            }
            EventKind::UnitFellBack { unit, .. } | EventKind::UnitSurrendered { unit, .. } => {
                vec![EntityRef::LandUnit(*unit)]
            }
            EventKind::PreparationShared {
                parent,
                new_force,
                units,
                ..
            } => {
                let mut entities = vec![
                    EntityRef::TaskForce(*parent),
                    EntityRef::TaskForce(*new_force),
                ];
                entities.extend(units.iter().map(|unit| EntityRef::LandUnit(*unit)));
                entities
            }
            EventKind::ForceEntered {
                force,
                ships,
                air_groups,
                land_units,
                ..
            } => {
                let mut entities = vec![EntityRef::TaskForce(*force)];
                entities.extend(ships.iter().map(|ship| EntityRef::Ship(*ship)));
                entities.extend(air_groups.iter().map(|group| EntityRef::AirGroup(*group)));
                entities.extend(land_units.iter().map(|unit| EntityRef::LandUnit(*unit)));
                entities
            }
            EventKind::UnitEntered { unit, base, .. }
            | EventKind::EntryWaiting { unit, base, .. } => {
                vec![*unit, EntityRef::Base(*base)]
            }
            EventKind::StockReceived { base, .. } => vec![EntityRef::Base(*base)],
            EventKind::MenJoined { unit, base, .. } => {
                vec![EntityRef::LandUnit(*unit), EntityRef::Base(*base)]
            }
            EventKind::ShipFuelledAtSea {
                ship,
                task_force,
                oiler,
                giver,
                ..
            } => vec![
                EntityRef::Ship(*ship),
                EntityRef::TaskForce(*task_force),
                EntityRef::Ship(*oiler),
                EntityRef::TaskForce(*giver),
            ],
            EventKind::FuellingBrokenOff {
                task_force, giver, ..
            }
            | EventKind::FuelOrderEnded {
                task_force, giver, ..
            } => vec![
                EntityRef::TaskForce(*task_force),
                EntityRef::TaskForce(*giver),
            ],
            EventKind::RationsLanded {
                base, task_force, ..
            } => vec![EntityRef::Base(*base), EntityRef::TaskForce(*task_force)],
            EventKind::HungerNightsChanged { unit, .. } => vec![EntityRef::LandUnit(*unit)],
            EventKind::FightSupplyCharged { .. } => Vec::new(),
            EventKind::BaseAmmunitionSpent { base, .. } => vec![EntityRef::Base(*base)],
            EventKind::AmmunitionLanded {
                base, task_force, ..
            } => vec![EntityRef::Base(*base), EntityRef::TaskForce(*task_force)],
        }
    }
}

/// "s" for a count other than one.
const fn plural(count: usize) -> &'static str {
    if count == 1 {
        ""
    } else {
        "s"
    }
}

/// The entity a command names: its task force or its base, and none for the
/// area command.
fn command_entity(command: CommandRef) -> Option<EntityRef> {
    match command {
        CommandRef::Area => None,
        CommandRef::TaskForce(force) => Some(EntityRef::TaskForce(force)),
        CommandRef::Base(base) => Some(EntityRef::Base(base)),
    }
}

/// The entity an air group's place names: its base or its ship, and none
/// for a group lost with its base (decision 0090, 20).
fn place_entity(place: AirGroupLocation) -> Option<EntityRef> {
    match place {
        AirGroupLocation::Base(base) => Some(EntityRef::Base(base)),
        AirGroupLocation::Ship(ship) => Some(EntityRef::Ship(ship)),
        AirGroupLocation::Lost => None,
    }
}

/// "round N: " before the text of an event of a surface fight. Damage
/// events carry round 0 when the hits fell outside a fight (an air attack,
/// decision 0026), and then write nothing.
struct RoundPrefix(u32);

impl fmt::Display for RoundPrefix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.0 == 0 {
            Ok(())
        } else {
            write!(f, "round {}: ", self.0)
        }
    }
}

impl fmt::Display for EventKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EventKind::OrderAccepted { order, target } => {
                write!(f, "{} accepted for {target}", order.shown())
            }
            EventKind::OrderRejected {
                order,
                target,
                reason,
            } => write!(f, "{} rejected for {target}: {reason}", order.shown()),
            EventKind::TaskForceSteamed {
                task_force,
                from,
                to,
                distance_nm,
                speed_kn,
                ..
            } => write!(
                f,
                "{task_force} steamed {distance_nm} nm at {speed_kn} kn from {from} to {to}"
            ),
            EventKind::FuelBurned {
                ship,
                burned_t,
                remaining_t,
                ..
            } => write!(f, "{ship} burned {burned_t} t, {remaining_t} t remaining"),
            EventKind::TaskForceArrived {
                task_force,
                hex,
                elapsed_hours,
            } => write!(
                f,
                "{task_force} arrived at {hex} at {}",
                ClockTime::from_elapsed(*elapsed_hours)
            ),
            EventKind::TaskForceOutOfFuel { task_force, hex } => {
                write!(f, "{task_force} ran out of fuel at {hex}")
            }
            EventKind::ShipRefuelled {
                ship,
                base,
                task_force,
                taken_t,
                fuel_t,
                base_fuel_t,
                force_restored,
            } => {
                write!(
                    f,
                    "{ship} took {taken_t} t of fuel at {base}, {fuel_t} t aboard, "
                )?;
                if *base_fuel_t == Fixed::ZERO {
                    write!(f, "{base} is out of fuel")?;
                } else {
                    write!(f, "{base} has {base_fuel_t} t left")?;
                }
                if *force_restored {
                    write!(f, "; {task_force} is no longer out of fuel")?;
                }
                Ok(())
            }
            // The text for supply is what it always was; fuel says so.
            EventKind::CargoLoaded {
                ship,
                base,
                cargo,
                loaded_t,
                aboard_t,
                base_stock_t,
                ..
            } => match cargo {
                Cargo::Supply => write!(
                    f,
                    "{ship} loaded {loaded_t} t from {base}, {aboard_t} t aboard, {base_stock_t} t in stock"
                ),
                Cargo::Fuel => write!(
                    f,
                    "{ship} loaded {loaded_t} t of fuel cargo from {base}, {aboard_t} t of fuel cargo aboard, {base_stock_t} t of fuel in stock"
                ),
            },
            EventKind::CargoUnloaded {
                ship,
                base,
                cargo,
                unloaded_t,
                aboard_t,
                base_stock_t,
                ..
            } => match cargo {
                Cargo::Supply => write!(
                    f,
                    "{ship} unloaded {unloaded_t} t at {base}, {aboard_t} t aboard, {base_stock_t} t in stock"
                ),
                Cargo::Fuel => write!(
                    f,
                    "{ship} unloaded {unloaded_t} t of fuel cargo at {base}, {aboard_t} t of fuel cargo aboard, {base_stock_t} t of fuel in stock"
                ),
            },
            EventKind::LoadingComplete {
                task_force,
                base,
                cargo,
                elapsed_hours,
            } => match cargo {
                Cargo::Supply => write!(
                    f,
                    "{task_force} completed loading at {base} at {}",
                    ClockTime::from_elapsed(*elapsed_hours)
                ),
                Cargo::Fuel => write!(
                    f,
                    "{task_force} completed loading fuel at {base} at {}",
                    ClockTime::from_elapsed(*elapsed_hours)
                ),
            },
            EventKind::UnloadingComplete {
                task_force,
                base,
                elapsed_hours,
            } => write!(
                f,
                "{task_force} completed unloading at {base} at {}",
                ClockTime::from_elapsed(*elapsed_hours)
            ),
            EventKind::SearchFlown {
                air_group,
                from,
                hex,
                from_deg,
                to_deg,
                range_nm,
                aircraft,
            } => match from {
                AirGroupLocation::Base(base) => write!(
                    f,
                    "{air_group} searched {from_deg:03}-{to_deg:03} out to {range_nm} nm from {base} with {aircraft} aircraft"
                ),
                AirGroupLocation::Ship(ship) => write!(
                    f,
                    "{air_group} searched {from_deg:03}-{to_deg:03} out to {range_nm} nm from {ship} at {hex} with {aircraft} aircraft"
                ),
                // A group lost with its base flies nothing (decision 0090).
                AirGroupLocation::Lost => write!(
                    f,
                    "{air_group} searched {from_deg:03}-{to_deg:03} out to {range_nm} nm from {hex} with {aircraft} aircraft"
                ),
            },
            EventKind::AirGroupLost {
                air_group,
                ship,
                aircraft,
            } => write!(f, "{air_group} lost with {ship}: {aircraft} aircraft"),
            EventKind::StrikeLaunched {
                air_group,
                contact,
                hex,
                aircraft,
                distance_nm,
            } => write!(
                f,
                "{air_group} launched {aircraft} aircraft at {contact} at {hex}, {distance_nm} nm",
                contact = contact.shown()
            ),
            EventKind::StrikeFoundNothing { air_group, hex } => {
                write!(f, "{air_group} found nothing at {hex}")
            }
            EventKind::ContactNotThere { side, contact, hex } => {
                write!(
                    f,
                    "{} {} not there at {hex}: dropped",
                    side.name(),
                    contact.shown()
                )
            }
            EventKind::BaseStrikeLaunched {
                air_group,
                base,
                hex,
                aircraft,
                distance_nm,
            } => write!(
                f,
                "{air_group} launched {aircraft} aircraft at {base} at {hex}, {distance_nm} nm"
            ),
            EventKind::BaseAntiAircraftFire {
                base,
                air_group,
                shots,
                chance,
                expected,
                hits,
                after_attack,
            } => write!(
                f,
                "{base} guns at the raid of {air_group}: {shots} shots x {chance} = {expected} expected, {hits} {}{}",
                if *hits == 1 { "hit" } else { "hits" },
                if *after_attack { " after the attack" } else { "" }
            ),
            EventKind::BaseAttacked {
                air_group,
                base,
                shots,
                chance,
                expected,
                hits,
                hits_on_aircraft,
                hits_on_supply,
                hits_on_men,
                recognised,
            } => {
                write!(
                    f,
                    "{air_group} bombs at {base}: {shots} shots x {chance} = {expected} expected, {hits} {}: {hits_on_aircraft} on aircraft, {hits_on_supply} on supply, ",
                    if *hits == 1 { "hit" } else { "hits" },
                )?;
                // Men are named only when a hit landed on them, so a raid
                // that hit no man reads as it did before decision 0092.
                if *hits_on_men > 0 {
                    write!(f, "{hits_on_men} on men, ")?;
                }
                write!(
                    f,
                    "{} on nothing",
                    hits - hits_on_aircraft - hits_on_supply - hits_on_men
                )?;
                recognised_suffix(f, *recognised)
            }
            EventKind::AircraftDestroyedOnGround {
                air_group,
                base,
                aircraft,
                remaining,
            } => write!(
                f,
                "{air_group} lost {aircraft} aircraft on the ground at {base}, {remaining} left"
            ),
            EventKind::DamagedAircraftDestroyedOnGround {
                air_group,
                base,
                aircraft,
                remaining,
            } => write!(
                f,
                "{air_group} lost {aircraft} damaged aircraft on the ground at {base}, {remaining} damaged left"
            ),
            EventKind::AircraftWorn {
                air_group,
                sortie,
                distance_nm,
                aircraft,
                chance,
                expected,
                worn,
                written_off,
                ready,
                damaged,
            } => write!(
                f,
                "{air_group} {} sortie of {distance_nm} nm: {aircraft} aircraft x {chance} = {expected} expected worn, {worn} worn, {written_off} written off, {ready} ready and {damaged} damaged left",
                sortie.name()
            ),
            EventKind::AircraftLandedElsewhere {
                from,
                to,
                at,
                aircraft,
                pilots,
                formed,
            } => {
                write!(
                    f,
                    "{from} had no deck: {aircraft} aircraft landed at {at}, {} {to}",
                    if *formed { "forming" } else { "joining" }
                )?;
                if *pilots > 0 {
                    write!(f, " with {pilots} pilots")?;
                }
                Ok(())
            }
            EventKind::AircraftDitchedNoDeck {
                air_group,
                hex,
                aircraft,
                beside_own_force,
                chance,
                crews_recovered,
                pilots_lost,
                ready,
            } => {
                write!(
                    f,
                    "{air_group} had no deck: {aircraft} aircraft came down in the sea at {hex}, {} (chance {chance})",
                    if *beside_own_force {
                        "beside ships of its own side"
                    } else {
                        "in the open sea"
                    }
                )?;
                if let Some(recovered) = crews_recovered {
                    write!(
                        f,
                        ", {recovered} {} recovered, {pilots_lost} {} lost",
                        if *recovered == 1 { "crew" } else { "crews" },
                        if *pilots_lost == 1 { "pilot" } else { "pilots" }
                    )?;
                }
                write!(f, ", {ready} ready left")
            }
            EventKind::AircraftDitched {
                air_group,
                sortie,
                distance_nm,
                radius_nm,
                aircraft,
                chance,
                expected,
                ditched,
                crews_recovered,
                pilots_lost,
                ready,
            } => {
                write!(
                    f,
                    "{air_group} {} sortie of {distance_nm} nm, {} beyond its radius of {radius_nm}: {aircraft} aircraft x {chance} = {expected} expected out of fuel, {ditched} ditched",
                    sortie.name(),
                    *distance_nm - *radius_nm
                )?;
                if let Some(recovered) = crews_recovered {
                    write!(
                        f,
                        ", {recovered} {} recovered, {pilots_lost} {} lost",
                        if *recovered == 1 { "crew" } else { "crews" },
                        if *pilots_lost == 1 { "pilot" } else { "pilots" }
                    )?;
                }
                write!(f, ", {ready} ready left")
            }
            EventKind::AirGroupFatigue {
                air_group,
                change,
                amount,
                fatigue,
            } => match change {
                FatigueChange::Gained {
                    sortie,
                    distance_nm,
                } => write!(
                    f,
                    "{air_group} gained {amount} fatigue from its {} sortie of {distance_nm} nm, now {fatigue}",
                    sortie.name()
                ),
                FatigueChange::Rested => {
                    write!(f, "{air_group} rested {amount} fatigue, now {fatigue}")
                }
            },
            EventKind::AirGroupStoodDown {
                air_group,
                fatigue,
                threshold,
            } => write!(
                f,
                "{air_group} stood down: fatigue {fatigue} is above its threshold of {threshold}"
            ),
            EventKind::AirGroupReturned {
                air_group,
                fatigue,
                threshold,
            } => write!(
                f,
                "{air_group} returned to flying: fatigue {fatigue} is not above its threshold of {threshold}"
            ),
            EventKind::AircraftRepaired {
                air_group,
                at,
                repaired,
                ready,
                damaged,
            } => match at {
                AirGroupLocation::Base(base) => write!(
                    f,
                    "{air_group} had {repaired} aircraft repaired at {base}, {ready} ready and {damaged} damaged"
                ),
                AirGroupLocation::Ship(ship) => write!(
                    f,
                    "{air_group} had {repaired} aircraft repaired aboard {ship}, {ready} ready and {damaged} damaged"
                ),
                // A group lost with its base has none to repair (decision
                // 0090).
                AirGroupLocation::Lost => write!(
                    f,
                    "{air_group} had {repaired} aircraft repaired, {ready} ready and {damaged} damaged"
                ),
            },
            EventKind::BaseSupplyDestroyed {
                base,
                destroyed_t,
                supply_t,
            } => write!(
                f,
                "{base} lost {destroyed_t} t of supply, {supply_t} t left"
            ),
            EventKind::BaseSupplySpent {
                base,
                reason,
                aircraft,
                men,
                asked_t,
                paid_t,
                left_t,
            } => {
                let what = match reason {
                    SupplyUse::Sorties => format!("{aircraft} aircraft sent on sorties"),
                    SupplyUse::Upkeep => format!("{aircraft} aircraft at the base for the day"),
                    SupplyUse::Repairs => format!("{aircraft} repairs"),
                    SupplyUse::Men => format!("{men} men ashore for the day"),
                    SupplyUse::March => format!("{men} men marching out of the hex"),
                    SupplyUse::Fight => "the men at grips in a fight".to_string(),
                };
                if paid_t < asked_t {
                    write!(
                        f,
                        "{base} spent {paid_t} t of supply on {what}: {asked_t} t was asked and the stock could not cover it, {left_t} t left"
                    )?;
                } else {
                    write!(f, "{base} spent {paid_t} t of supply on {what}, {left_t} t left")?;
                }
                if left_t.is_zero() {
                    write!(f, ": {base} is out of supply")?;
                }
                Ok(())
            }
            EventKind::AirAttack {
                air_group,
                ship,
                weapon,
                shots,
                chance,
                expected,
                hits,
                recognised,
                ..
            } => {
                write!(
                    f,
                    "{air_group} {} at {ship}: {shots} shots x {chance} = {expected} expected, {hits} {}",
                    weapon.name(),
                    if *hits == 1 { "hit" } else { "hits" },
                )?;
                // Named only where recognised, so a world where no type is
                // recognised reads as it did before decision 0167.
                recognised_suffix(f, *recognised)
            }
            EventKind::EscortsSent {
                air_group,
                with_air_group,
                aircraft,
            } => write!(f, "{air_group} sent {aircraft} escorts with {with_air_group}"),
            EventKind::AirFight {
                firer,
                target,
                firing,
                at,
                shots,
                chance,
                expected,
                hits,
                after_attack,
            } => write!(
                f,
                "{firer} {} at the {} of {target}: {shots} shots x {chance} = {expected} expected, {hits} {}{}",
                firing.name(),
                at.name(),
                if *hits == 1 { "hit" } else { "hits" },
                if *after_attack { " after the attack" } else { "" }
            ),
            EventKind::AntiAircraftFire {
                task_force,
                air_group,
                shots,
                chance,
                expected,
                hits,
                after_attack,
            } => write!(
                f,
                "{task_force} guns at the raid of {air_group}: {shots} shots x {chance} = {expected} expected, {hits} {}{}",
                if *hits == 1 { "hit" } else { "hits" },
                if *after_attack { " after the attack" } else { "" }
            ),
            EventKind::AircraftLost {
                air_group,
                aircraft,
                remaining,
            } => write!(f, "{air_group} lost {aircraft} aircraft, {remaining} left"),
            EventKind::PilotsLost {
                air_group,
                lost,
                left,
            } => write!(
                f,
                "{air_group} lost {lost} {}, {left} left",
                if *lost == 1 { "pilot" } else { "pilots" }
            ),
            EventKind::AircraftArrived {
                air_group,
                aircraft,
                pilots,
                ready,
                pilots_now,
                strength,
            } => write!(
                f,
                "{air_group} received {aircraft} aircraft and {pilots} {} by arrival, {ready} ready, {pilots_now} {} and strength {strength}",
                if *pilots == 1 { "pilot" } else { "pilots" },
                if *pilots_now == 1 { "pilot" } else { "pilots" },
            ),
            EventKind::ReplacementsReceived {
                air_group,
                base,
                aircraft,
                pilots,
                ready,
                pilots_now,
            } => write!(
                f,
                "{air_group} received {aircraft} aircraft and {pilots} pilots at {base}, {ready} ready and {pilots_now} pilots"
            ),
            EventKind::SearchAttempt {
                air_group,
                task_force,
                distance_nm,
                chance,
                sighted,
            } => write!(
                f,
                "{air_group} searched for {task_force} at {distance_nm} nm, chance {chance}: {}",
                if *sighted { "sighted" } else { "not sighted" }
            ),
            EventKind::ReportReceived {
                side,
                contact,
                hex,
                ship_count,
                kind,
                created,
            } => write_report(
                f,
                (*side, *contact, *hex, *ship_count, *kind),
                *created,
                false,
            ),
            EventKind::ContactAged {
                side,
                contact,
                age_pulses,
            } => write!(
                f,
                "{} {} aged to {age_pulses} pulses",
                side.name(),
                contact.shown()
            ),
            EventKind::ContactDropped {
                side,
                contact,
                age_pulses,
            } => write!(
                f,
                "{} {} dropped at {age_pulses} pulses",
                side.name(),
                contact.shown()
            ),
            EventKind::EngagementBegun {
                a,
                b,
                hex,
                elapsed_hours,
            } => write!(
                f,
                "{a} and {b} met at {hex} at {}: engagement begun",
                ClockTime::from_elapsed(*elapsed_hours)
            ),
            EventKind::SightingRoll {
                round,
                task_force,
                enemy,
                chance,
                fatigue_factor,
                sighted,
            } => {
                write!(
                    f,
                    "round {round}: {task_force} looked for {enemy}, chance {chance}"
                )?;
                // Shown only where fatigue is in play (decision 0037).
                if *fatigue_factor != Fixed::ONE {
                    write!(f, " (fatigue factor {fatigue_factor})")?;
                }
                write!(f, ": {}", if *sighted { "sighted" } else { "not sighted" })
            }
            EventKind::ForceSurprised {
                round,
                task_force,
                enemy,
            } => write!(f, "round {round}: {task_force} was surprised by {enemy}"),
            EventKind::Salvo {
                round,
                firer,
                target,
                weapon,
                shots,
                chance,
                expected,
                hits,
            } => write!(
                f,
                "round {round}: {firer} {} at {target}: {shots} shots x {chance} = {expected} expected, {hits} {}",
                weapon.name(),
                if *hits == 1 { "hit" } else { "hits" }
            ),
            EventKind::ShipDamaged {
                round,
                ship,
                track,
                hits,
                points,
                value_after,
            } => write!(
                f,
                "{}{ship} took {hits} {} on {}, {points} points, now {value_after}",
                RoundPrefix(*round),
                if *hits == 1 { "hit" } else { "hits" },
                track.name()
            ),
            EventKind::SystemHit {
                round,
                ship,
                system,
                before,
                after,
            } => {
                if before == after {
                    write!(
                        f,
                        "{}{ship} {} hit, no change, still {}",
                        RoundPrefix(*round),
                        system.name(),
                        before.name()
                    )
                } else {
                    write!(
                        f,
                        "{}{ship} {} hit, {} -> {}",
                        RoundPrefix(*round),
                        system.name(),
                        before.name(),
                        after.name()
                    )
                }
            }
            EventKind::ShipSunk {
                round,
                ship,
                task_force,
            } => write!(f, "{}{ship} of {task_force} sank", RoundPrefix(*round)),
            EventKind::ForceBecameAlert { round, task_force } => {
                write!(f, "round {round}: {task_force} became alert")
            }
            EventKind::ForceBrokeOff {
                round,
                task_force,
                share,
            } => write!(
                f,
                "round {round}: {task_force} broke off, {share} of its ships sunk or crippled"
            ),
            EventKind::ForceSlippedAway {
                round,
                task_force,
                enemy,
            } => write!(f, "round {round}: {task_force} slipped away from {enemy}"),
            EventKind::ForcesPassedUnseen { round, a, b } => {
                write!(f, "round {round}: {a} and {b} passed unseen")
            }
            EventKind::EngagementEnded {
                round,
                a,
                b,
                hex,
                reason,
            } => write!(
                f,
                "engagement of {a} and {b} at {hex} ended in round {round}: {}",
                reason.name()
            ),
            EventKind::ClaimReceived {
                side,
                hex,
                claimed_sunk,
                claimed_hits,
                claimed_kind,
                claimed_shot_down,
                claimed_on_ground,
                from_air,
                from_bombardment,
                ground_types,
                by_unit,
                air_types,
            } => f.write_str(&claim_text(
                *side,
                *hex,
                *claimed_sunk,
                *claimed_hits,
                *claimed_kind,
                *claimed_shot_down,
                *claimed_on_ground,
                *from_air,
                *from_bombardment,
                ground_types,
                by_unit,
                air_types,
            )),
            EventKind::ShipsDetached {
                parent,
                new_force,
                ships,
                escorts,
                reason,
            } => {
                let ships = join_ships(ships);
                match reason {
                    DetachReason::SentHome => {
                        write!(f, "{parent} sent {ships} home as {new_force}")?;
                        if !escorts.is_empty() {
                            write!(f, " with escorts {}", join_ships(escorts))?;
                        }
                        Ok(())
                    }
                    DetachReason::LeftBehind => {
                        write!(f, "{parent} left {ships} behind as {new_force}")
                    }
                    DetachReason::Ordered => {
                        write!(f, "{parent} detached {ships} as {new_force}")
                    }
                }
            }
            EventKind::ForceRetired { task_force } => write!(f, "{task_force} retires"),
            EventKind::ForceAlertedByContact {
                task_force,
                contact,
            } => write!(f, "{task_force} alerted by {}", contact.shown()),
            EventKind::ForceStoodDown { task_force } => write!(f, "{task_force} stood down"),
            EventKind::TaskForceFatigue {
                task_force,
                change,
                amount,
                fatigue,
            } => match change {
                ForceFatigueChange::Gained => write!(
                    f,
                    "{task_force} gained {amount} fatigue, alert at the start of the pulse, now {fatigue}"
                ),
                ForceFatigueChange::Rested => write!(
                    f,
                    "{task_force} rested {amount} fatigue, not alert at the start of the pulse, now {fatigue}"
                ),
            },
            EventKind::TaskForceTurnedBack {
                task_force,
                from,
                toward,
                progress_nm,
                position,
            } => write!(
                f,
                "{task_force} turned back toward {toward}, {progress_nm} nm from {from}, now in {position}"
            ),
            EventKind::AlarmRoll {
                task_force,
                by_a,
                by_b,
                chance,
                alarmed,
            } => write!(
                f,
                "{task_force} heard the fight of {by_a} and {by_b}, chance {chance}: {}",
                if *alarmed { "alarmed" } else { "not alarmed" }
            ),
            EventKind::BaseAtDayEnd {
                base,
                supply_t,
                aircraft_ready,
            } => write!(
                f,
                "{base} at the end of the day: {supply_t} t of supply, {aircraft_ready} aircraft ready: {}",
                if base_is_open(*supply_t, *aircraft_ready) {
                    "open"
                } else {
                    "not open"
                }
            ),
            EventKind::ShipRepaired {
                ship,
                base,
                points,
                fire_damage,
                flotation_damage,
                restored,
                ..
            } => {
                write!(
                    f,
                    "{ship} repaired {points} points at {base}: fire {fire_damage}, flotation {flotation_damage}"
                )?;
                if !restored.is_empty() {
                    let names: Vec<&str> = restored.iter().map(|system| system.name()).collect();
                    write!(f, ", {} working", names.join(" and "))?;
                }
                Ok(())
            }
            EventKind::ForceLeftForYard {
                task_force,
                base,
                passage_days,
                ships,
                return_day,
                ..
            } => {
                write!(f, "{task_force} left {base} for the yard: ")?;
                for (index, (ship, days)) in ships.iter().enumerate() {
                    if index > 0 {
                        f.write_str(", ")?;
                    }
                    write!(f, "{ship} {days} days")?;
                }
                write!(f, ", passage {passage_days}, back on day {return_day}")
            }
            EventKind::ForceReturnedFromYard {
                task_force,
                base,
                ships,
            } => {
                write!(f, "{task_force} back at {base} from the yard, ")?;
                for (index, ship) in ships.iter().enumerate() {
                    if index > 0 {
                        f.write_str(if index + 1 == ships.len() { " and " } else { ", " })?;
                    }
                    write!(f, "{ship}")?;
                }
                f.write_str(" repaired, refuelled and rearmed")
            }
            EventKind::MenEmbarked {
                unit,
                ship,
                hex,
                men,
                aboard,
                ashore,
                ..
            } => write!(
                f,
                "{men} men of {unit} went aboard {ship} at {hex}: {aboard} aboard, {ashore} left ashore"
            ),
            EventKind::MenLanded {
                unit,
                ship,
                hex,
                men,
                aboard,
                ashore,
                ..
            } => write!(
                f,
                "{ship} landed {men} men of {unit} at {hex}: {ashore} ashore, {aboard} left aboard"
            ),
            EventKind::BaseFormed {
                base,
                side,
                hex,
                task_force,
            } => write!(
                f,
                "{base} formed at {hex} for the {} side by the landing of {task_force}",
                side.name()
            ),
            EventKind::UnitDivided {
                from,
                unit,
                task_force,
                men,
            } => write!(
                f,
                "the {men} men of {from} aboard {task_force} became {unit}, since {from} has men ashore elsewhere"
            ),
            EventKind::MenLostWithShip { unit, ship, men } => {
                write!(f, "{men} men of {unit} were lost with {ship}")
            }
            EventKind::MenFed {
                side,
                hex,
                base,
                men,
                fed,
                unfed,
            } => {
                write!(f, "{} men ashore at {hex}: ", side.name())?;
                match base {
                    Some(base) => write!(f, "{fed} of {men} fed from {base}")?,
                    None => write!(f, "no base of the side, none of {men} fed")?,
                }
                write!(f, ", {unfed} unfed")
            }
            EventKind::MenLostUnfed {
                unit,
                hex,
                lost,
                ashore,
            } => write!(f, "{unit} lost {lost} men unfed at {hex}, {ashore} left ashore"),
            EventKind::LandUnitDestroyed { unit } => {
                write!(f, "{unit} has no men left and is destroyed")
            }
            EventKind::UnitLoadingComplete {
                task_force,
                unit,
                base,
                cargo,
                elapsed_hours,
            } => {
                write!(f, "{task_force} finished loading {unit}")?;
                if let Some(cargo) = cargo {
                    write!(f, " and {}", cargo.name())?;
                }
                match base {
                    Some(base) => write!(f, " at {base}")?,
                    None => f.write_str(" off the shore")?,
                }
                write!(f, " at {}", ClockTime::from_elapsed(*elapsed_hours))
            }
            EventKind::AircraftWrittenOff {
                air_group,
                base,
                aircraft,
            } => write!(
                f,
                "{air_group} wrote off {aircraft} damaged aircraft at {base}, which could not fly"
            ),
            EventKind::AirGroupRebased {
                air_group,
                from,
                to,
                distance_nm,
                transfer_range_nm,
                ready,
                pilots,
            } => {
                write!(
                    f,
                    "{air_group} moved from {from} to {to}, {distance_nm} nm within its transfer range of {transfer_range_nm} nm, with {ready} ready"
                )?;
                if let Some(pilots) = pilots {
                    write!(f, " and {pilots} pilots")?;
                }
                f.write_str("; it flies nothing today")
            }
            EventKind::FedShareRestored { unit } => {
                write!(f, "{unit} has no men ashore and its fed share is 1")
            }
            EventKind::GroundFight {
                hex,
                allied_attacks,
                japanese_attacks,
            } => {
                let attackers = match (allied_attacks, japanese_attacks) {
                    (true, true) => "both sides attack",
                    (true, false) => "the Allied side attacks",
                    (false, true) => "the Japanese side attacks",
                    (false, false) => "nobody attacks",
                };
                write!(f, "fight ashore at {hex}: {attackers}")
            }
            EventKind::GroundFire {
                unit,
                hex,
                firing_men,
                shots,
                chance,
                hits,
            } => write!(
                f,
                "{unit} fired at {hex} with {firing_men} men: {shots} shots at {chance}, {hits} {}",
                if *hits == 1 { "hit" } else { "hits" }
            ),
            EventKind::MenLostInFight {
                unit,
                hex,
                lost,
                ashore,
            } => write!(
                f,
                "{unit} lost {lost} men in the fight at {hex}, {ashore} left ashore"
            ),
            EventKind::LandUnitFatigue {
                unit,
                change,
                fatigue,
            } => match change {
                LandFatigueChange::Gained => {
                    write!(f, "{unit} attacked: fatigue {fatigue}")
                }
                LandFatigueChange::Rested => write!(f, "{unit} rested: fatigue {fatigue}"),
                LandFatigueChange::Marched => write!(f, "{unit} marched: fatigue {fatigue}"),
                LandFatigueChange::Landed => write!(f, "{unit} landed: fatigue {fatigue}"),
                LandFatigueChange::MarchedAndAttacked => {
                    write!(f, "{unit} marched and attacked: fatigue {fatigue}")
                }
                LandFatigueChange::Strained => {
                    write!(f, "{unit} was under fire and did not rest: fatigue {fatigue}")
                }
            },
            EventKind::LandUnitBackToDefend { unit, fatigue } => write!(
                f,
                "{unit} is too tired to attack at fatigue {fatigue} and goes back to defend"
            ),
            EventKind::SideBeatenAshore {
                side,
                hex,
                men,
                enemy_men,
                fought,
            } => {
                write!(
                    f,
                    "the {} side is beaten at {hex}: {men} men ashore against {enemy_men}",
                    side.name()
                )?;
                if !fought {
                    f.write_str(", without a fight")?;
                }
                Ok(())
            }
            EventKind::BaseChangedSide {
                base,
                hex,
                from,
                to,
                supply_t,
                fuel_t,
                supply_lost_t,
                fuel_lost_t,
            } => write!(
                f,
                "{base} at {hex} changed from the {} side to the {} side: {supply_t} t of supply and {fuel_t} t of fuel kept, {supply_lost_t} t and {fuel_lost_t} t lost",
                from.name(),
                to.name()
            ),
            EventKind::AirGroupLostWithBase {
                air_group,
                base,
                aircraft,
            } => write!(f, "{air_group} was lost with {base}: {aircraft} aircraft"),
            EventKind::ShipLostWithBase { ship, base } => {
                write!(f, "{ship} was lost in port with {base}")
            }
            EventKind::HomePortLost { task_force, base } => {
                write!(f, "{task_force} lost its home port {base} and has none")
            }
            EventKind::StrikeMissionCleared { air_group, base } => write!(
                f,
                "{air_group} has no mission: the base it was to strike, {base}, is its own side's"
            ),
            EventKind::BombardmentFire {
                ship,
                base,
                shells,
                weighed_shots,
                chance,
                expected,
                hits,
                hits_on_aircraft,
                hits_on_supply,
                hits_on_men,
                rounds_left,
            } => write!(
                f,
                "{ship} shelled {base}: {shells} shells, {weighed_shots} weighed shots x {chance} = {expected} expected, {hits} {}: {hits_on_aircraft} on aircraft, {hits_on_supply} on supply, {hits_on_men} on men, {} on nothing; {rounds_left} rounds left",
                if *hits == 1 { "hit" } else { "hits" },
                hits - hits_on_aircraft - hits_on_supply - hits_on_men
            ),
            EventKind::ShoreBombardment {
                task_force,
                base,
                hex,
                shells,
                weighed_shots,
                hits,
                hits_on_aircraft,
                hits_on_supply,
                hits_on_men,
            } => write!(
                f,
                "{task_force} bombarded {base} at {hex}: {shells} shells, {weighed_shots} weighed shots, {hits} {}: {hits_on_aircraft} on aircraft, {hits_on_supply} on supply, {hits_on_men} on men, {} on nothing",
                if *hits == 1 { "hit" } else { "hits" },
                hits - hits_on_aircraft - hits_on_supply - hits_on_men
            ),
            EventKind::MenLostToFire {
                unit,
                base,
                hex,
                lost,
                ashore,
            } => write!(
                f,
                "{unit} lost {lost} men to fire on {base} at {hex}, {ashore} left ashore"
            ),
            EventKind::GroundReportReceived {
                side,
                hex,
                men,
                replaced,
                grade,
            } => write!(
                f,
                "{} ground report at {hex}: {men} enemy men ashore ({}){}",
                side.name(),
                grade.name(),
                if *replaced { ", replacing the estimate held" } else { "" }
            ),
            EventKind::ReconnaissanceFlown {
                air_group,
                from,
                hex,
                distance_nm,
                supply_t,
            } => match from {
                AirGroupLocation::Base(base) => write!(
                    f,
                    "{air_group} flew {distance_nm} nm from {base} to look at {hex}, at a cost of {supply_t} t"
                ),
                AirGroupLocation::Ship(ship) => write!(
                    f,
                    "{air_group} flew {distance_nm} nm from {ship} to look at {hex}, at a cost of {supply_t} t"
                ),
                AirGroupLocation::Lost => write!(f, "{air_group} flew to look at {hex}"),
            },
            EventKind::BaseFound { side, base, hex } => write!(
                f,
                "{} found enemy {base} at {hex}: it is on the {} side's map",
                side.name(),
                side.name()
            ),
            EventKind::MarchDay {
                unit,
                from,
                to,
                days,
                counted,
                of,
            } => {
                let of_days = if *of == 1 { "day" } else { "days" };
                if *counted == Fixed::from_int(i64::from(*days)) {
                    write!(
                        f,
                        "{unit} marched from {from} toward {to}: {days} of {of} {of_days}"
                    )
                } else {
                    write!(
                        f,
                        "{unit} marched from {from} toward {to}: {days} {} marched, counted as {counted} of {of} {of_days}",
                        if *days == 1 { "day" } else { "days" }
                    )
                }
            }
            EventKind::MarchBaseFormed {
                base,
                side,
                hex,
                unit,
            } => write!(
                f,
                "{base} formed at {hex} for the {} side to hold the supply {unit} carried",
                side.name()
            ),
            EventKind::MarchArrived {
                unit,
                from,
                to,
                men,
                supply_t,
                base,
            } => {
                write!(f, "{unit} arrived at {to} from {from} with {men} men")?;
                match base {
                    Some(base) => write!(f, ", carrying {supply_t} t of supply into {base}"),
                    None => f.write_str(", carrying no supply"),
                }
            }
            EventKind::MarchStopped {
                unit,
                to,
                days,
                reason,
            } => write!(
                f,
                "{unit} stopped its march to {to} after {days} {}: {}",
                if *days == 1 { "day" } else { "days" },
                match reason {
                    MarchStop::Ordered => "ordered to stay",
                    MarchStop::NoMenAshore => "it has no men ashore",
                    MarchStop::NoLandLink => "no land link joins its hex to it",
                }
            ),
            EventKind::TaskForceHalted {
                task_force,
                hex,
                work,
                elapsed_hours,
            } => write!(
                f,
                "{task_force} halted at {hex} at {} to {}",
                ClockTime::from_elapsed(*elapsed_hours),
                halt_work_words(*work)
            ),
            EventKind::TaskForceSteamedOn {
                task_force,
                hex,
                work,
                fired,
            } => {
                write!(f, "{task_force} steamed on from {hex}: ")?;
                let mut done: Vec<&str> = Vec::new();
                if work.bombard {
                    done.push(if *fired {
                        "it has bombarded"
                    } else {
                        "it can bombard no longer"
                    });
                }
                if work.cargo {
                    done.push("its cargo work is done");
                }
                if work.night {
                    done.push("the night is over");
                }
                f.write_str(&done.join(" and "))
            }
            EventKind::LoadingGained {
                task_force,
                hex,
                loading,
            } => write!(
                f,
                "{task_force} loaded for the landing at {hex}: loading {loading}"
            ),
            EventKind::LoadingLost {
                task_force,
                hex,
                loading_before,
                loading,
                reason,
            } => match reason {
                LoadingLoss::Spent => write!(
                    f,
                    "{task_force} completed its unloading at {hex}: loading {loading_before} spent, its order to prepare the landing done"
                ),
                _ => write!(
                    f,
                    "{task_force} lost loading for the landing at {hex} ({}): {loading_before} to {loading}",
                    reason.name()
                ),
            },
            EventKind::RehearsalGained {
                unit,
                task_force,
                hex,
                rehearsal,
            } => write!(
                f,
                "{unit} rehearsed the landing at {hex} with {task_force}: rehearsal {rehearsal}"
            ),
            EventKind::RehearsalLost {
                unit,
                hex,
                rehearsal_before,
                reason,
            } => write!(
                f,
                "{unit} lost its rehearsal of {rehearsal_before} for the landing at {hex} ({})",
                reason.name()
            ),
            EventKind::LeftRoster {
                unit,
                task_force,
                reason,
            } => write!(
                f,
                "{unit} left the roster of {task_force} ({})",
                reason.name()
            ),
            EventKind::PreparationShared {
                parent,
                new_force,
                hex,
                loading,
                units,
            } => {
                write!(
                    f,
                    "{new_force} took loading {loading} for the landing at {hex} from {parent}"
                )?;
                if units.is_empty() {
                    f.write_str(", and no unit of its roster")
                } else {
                    let names: Vec<String> = units.iter().map(LandUnitId::to_string).collect();
                    write!(f, ", and the roster units {}", names.join(", "))
                }
            }
            EventKind::TrainingDay {
                unit,
                days,
                experience,
            } => write!(
                f,
                "{unit} trained: {days} {} counted at experience {experience}",
                if *days == 1 { "day" } else { "days" }
            ),
            EventKind::FightDayCounted {
                unit,
                days,
                experience,
            } => write!(
                f,
                "{unit} fired in a fight: {days} {} counted at experience {experience}",
                if *days == 1 { "day" } else { "days" }
            ),
            EventKind::ExperienceRaised {
                unit,
                experience,
                by,
            } => write!(
                f,
                "{unit} rose to experience {experience} by {}",
                by.name()
            ),
            EventKind::UnitWorked {
                unit,
                base,
                job,
                work,
                total,
            } => {
                write!(f, "{unit} worked at the {} of {base}: {work} of work", job.name())?;
                match job {
                    Job::Airfield => write!(f, ", {total} done on the airfield"),
                    Job::Port => write!(f, ", {total} on the port today"),
                    Job::Aircraft => write!(f, ", {total} on the aircraft today"),
                    Job::Defences => write!(f, ", defences {total}"),
                }
            }
            EventKind::AirfieldBuilt {
                base,
                side,
                hex,
                work_done,
            } => write!(
                f,
                "{base} ({}) at {hex} has an airfield: {work_done} of work done",
                side.name()
            ),
            EventKind::AirfieldDamaged {
                base,
                side,
                hits,
                damage,
            } => write!(
                f,
                "{} hit{} on the airfield of {base} ({}), damage {damage}",
                hits,
                if *hits == 1 { "" } else { "s" },
                side.name()
            ),
            EventKind::ArrivalWaiting {
                air_group,
                base,
                aircraft,
                pilots,
                damage,
            } => write!(
                f,
                "{aircraft} aircraft and {pilots} {} due at {air_group} wait: the airfield of {base} is damaged {damage} and takes no aircraft",
                if *pilots == 1 { "pilot" } else { "pilots" }
            ),
            EventKind::AirfieldMended {
                unit,
                base,
                points,
                damage,
            } => write!(
                f,
                "{unit} mended {points} of damage on the airfield of {base}, damage {damage}"
            ),
            EventKind::UnitFellBack {
                unit,
                side,
                from,
                to,
                men,
                fatigue,
            } => write!(
                f,
                "{unit} of the {} side fell back from {from} toward {to} with {men} men: fatigue {fatigue}",
                side.name()
            ),
            EventKind::UnitSurrendered {
                unit,
                side,
                hex,
                men,
            } => write!(
                f,
                "{unit} of the {} side surrendered at {hex}: {men} men lost",
                side.name()
            ),
            EventKind::ForceEntered {
                force,
                side,
                hex,
                base,
                ships,
                air_groups,
                land_units,
            } => {
                write!(
                    f,
                    "{force} of the {} side entered at {hex} with {} ship{}",
                    side.name(),
                    ships.len(),
                    if ships.len() == 1 { "" } else { "s" }
                )?;
                if !air_groups.is_empty() {
                    write!(f, ", {} air group{}", air_groups.len(), plural(air_groups.len()))?;
                }
                if !land_units.is_empty() {
                    write!(f, ", {} land unit{}", land_units.len(), plural(land_units.len()))?;
                }
                if let Some(base) = base {
                    write!(f, " (at {base})")?;
                }
                Ok(())
            }
            EventKind::UnitEntered {
                unit,
                side,
                base,
                hex,
                men,
                aircraft,
            } => {
                write!(
                    f,
                    "{unit} of the {} side entered at {base} ({hex})",
                    side.name()
                )?;
                if *men > 0 {
                    write!(f, " with {men} men")?;
                }
                if *aircraft > 0 {
                    write!(f, " with {aircraft} aircraft")?;
                }
                Ok(())
            }
            EventKind::EntryWaiting {
                unit,
                side,
                base,
                day,
                wait,
            } => write!(
                f,
                "{unit} of the {} side, to enter at {base} on day {day}, waits: {}",
                side.name(),
                match wait {
                    crate::entering::EntryWait::NotTheSidesBase => "the base is not its side's",
                    crate::entering::EntryWait::TakesNoAircraft => "the base takes no aircraft",
                }
            ),
            EventKind::StockReceived {
                base,
                side,
                supply_t,
                fuel_t,
            } => write!(
                f,
                "{base} of the {} side received {supply_t} t of supply and {fuel_t} t of fuel by its supply line",
                side.name()
            ),
            EventKind::MenJoined {
                unit,
                side,
                base,
                men,
                men_after,
                experience_before,
                experience_after,
                joined,
            } => {
                write!(
                    f,
                    "{men} replacement men of the {} side joined {unit} at {base}: {men_after} men",
                    side.name()
                )?;
                if experience_after == experience_before {
                    write!(
                        f,
                        ", experience {experience_after}, {joined} joined since its experience last changed"
                    )
                } else {
                    write!(
                        f,
                        ", experience from {experience_before} to {experience_after}"
                    )
                }
            }
            EventKind::MenSurrenderedAshore { side, hex, men } => write!(
                f,
                "men of the {} side surrendered at {hex}: {men} men ashore in all",
                side.name()
            ),
            EventKind::GroundAttackFailed {
                side,
                hex,
                weight,
                enemy_weight,
            } => write!(
                f,
                "the {} side's attack at {hex} failed: weight of fire {weight} against {enemy_weight}",
                side.name()
            ),
            EventKind::SpentUnitFellBack {
                unit,
                side,
                from,
                to,
                men,
                fatigue,
            } => write!(
                f,
                "{unit} of the {} side, its attack spent, fell back from {from} toward {to} with {men} men: fatigue {fatigue}",
                side.name()
            ),
            EventKind::FieldworksChanged {
                unit,
                change,
                fieldworks,
            } => match change {
                FieldworksChange::Dug => write!(f, "{unit} dug in: fieldworks {fieldworks}"),
                FieldworksChange::Worn => {
                    write!(f, "{unit}'s fieldworks were worn by fire: {fieldworks}")
                }
                FieldworksChange::Lost => write!(f, "{unit} left its fieldworks"),
            },
            EventKind::DefencesWorn {
                base,
                side,
                hex,
                hits,
                defences,
            } => write!(
                f,
                "{hits} {} on men of the {} side at {hex} wore the defences of {base} to {defences}",
                if *hits == 1 { "hit" } else { "hits" },
                side.name()
            ),
            EventKind::TaskForceFormed {
                task_force,
                base,
                ships,
                ..
            } => write!(
                f,
                "{} formed into {task_force} at {base}",
                join_ships(ships)
            ),
            EventKind::TaskForceJoined {
                joined,
                into,
                base,
                ships,
            } => write!(
                f,
                "{joined} joined {into} at {base} with {}; {joined} has ceased to be",
                join_ships(ships)
            ),
            EventKind::TaskForceStoodDownInPort {
                task_force,
                base,
                ships,
            } => write!(
                f,
                "{task_force} stood down at {base}: {} lie in port in no force; {task_force} has ceased to be",
                join_ships(ships)
            ),
            EventKind::LoadBelowReserve {
                task_force,
                base,
                cargo,
                below_t,
                reserve_t,
            } => write!(
                f,
                "{task_force}'s load of {} may take {below_t} t below {base}'s reserve of {reserve_t} t",
                cargo.name()
            ),
            EventKind::RefuellingHeldAtReserve {
                ship,
                task_force,
                hex,
                short_t,
            } => write!(
                f,
                "{ship} of {task_force} is {short_t} t short of full: the fuel at {hex} is at its reserve"
            ),
            EventKind::LoadHeldAtReserve {
                task_force,
                base,
                cargo,
                aboard_t,
                reserve_t,
            } => write!(
                f,
                "{task_force}'s load of {} stopped at {base}'s reserve of {reserve_t} t, with {aboard_t} t aboard",
                cargo.name()
            ),
            EventKind::CommandTaken {
                side,
                issuer,
                command,
            } => write!(f, "{side:?} {} command taken: {command}", issuer_name(*issuer)),
            EventKind::CommandRefused {
                side,
                issuer,
                command,
                reason,
            } => write!(
                f,
                "{side:?} {} command refused ({reason}): {command}",
                issuer_name(*issuer)
            ),
            EventKind::PlanDrafted {
                side,
                plan,
                intent,
            } => write!(f, "{side:?} {plan} drafted for {intent}"),
            EventKind::PlanStateChanged {
                side,
                plan,
                from,
                to,
            } => write!(f, "{side:?} {plan} went from {} to {}", from.name(), to.name()),
            EventKind::PlanAmended { side, plan, number } => {
                write!(f, "{side:?} {plan} amended, amendment {number}")
            }
            EventKind::StaffReason {
                side,
                order,
                command,
                reason,
            } => {
                match (order, command) {
                    (Some(order), _) => write!(f, "{side:?} staff's reason for {}", order.shown())?,
                    (None, Some(command)) => {
                        write!(f, "{side:?} staff's reason for the command to {command}")?
                    }
                    (None, None) => write!(f, "{side:?} staff's reason for giving no order")?,
                }
                let serves = reason.serves.to_string();
                if !serves.is_empty() {
                    write!(f, " [serves {serves}]")?;
                }
                write!(f, ": {}", reason.worked)
            }
            EventKind::HeldByOrder {
                side,
                target,
                order,
            } => write!(f, "{side:?} holds {target} by {}", order.shown()),
            EventKind::StaffNotice {
                side,
                kind,
                plan,
                run,
                intent,
                pursuit,
            } => {
                write!(f, "{side:?} staff's notice: ")?;
                match (kind, run) {
                    (NoticeKind::CannotMeet, _) => write!(f, "{plan} cannot meet its intent"),
                    (NoticeKind::WaitsForLeave(leave), _) => {
                        write!(f, "{plan} waits for leave to {}", leave.name())
                    }
                    (NoticeKind::RunEnded, Some(run)) => {
                        write!(f, "run {run} of {plan} has ended")
                    }
                    (NoticeKind::RunEnded, None) => write!(f, "a run of {plan} has ended"),
                    (NoticeKind::Ready, _) => write!(f, "{plan}'s landing is ready"),
                    (NoticeKind::Late, _) => {
                        write!(f, "{plan}'s landing cannot be ready by its date")
                    }
                    (NoticeKind::Met, _) => match intent {
                        Some(intent) => write!(f, "{intent} is met"),
                        None => write!(f, "every want of {plan} is met"),
                    },
                    (NoticeKind::CannotBringEnough, _) => {
                        write!(f, "{plan} cannot bring the men it wants")
                    }
                    (NoticeKind::LiftTurnedBack, _) => {
                        write!(f, "{plan}'s lift is turned back")
                    }
                    (NoticeKind::AsksLeaveToPursue, _) => match pursuit {
                        Some(notice) => {
                            write!(f, "{} asks leave to pursue", notice.force)?;
                            write_pursuit_find(f, notice)
                        }
                        None => write!(f, "{plan} asks leave to pursue"),
                    },
                    (NoticeKind::Pursues, _) => match pursuit {
                        Some(notice) => {
                            write!(f, "{} pursues", notice.force)?;
                            write_pursuit_find(f, notice)
                        }
                        None => write!(f, "{plan} pursues"),
                    },
                    (NoticeKind::PursuitEnded, _) => match pursuit {
                        Some(notice) => write!(f, "{}'s pursuit is ended", notice.force),
                        None => write!(f, "{plan}'s pursuit is ended"),
                    },
                }
            }
            EventKind::InterruptFired {
                side,
                interrupt,
                condition,
                figure,
            } => match (interrupt, condition) {
                (InterruptRef::Set(id), Some(InterruptCondition::SupplyBelow { base, days })) => {
                    write!(f, "{side:?} {id} fired: {base} at ")?;
                    match figure {
                        Some(figure) => write!(f, "{figure}")?,
                        None => write!(f, "no figure of")?,
                    }
                    write!(f, " days of supply, below {days}")
                }
                (InterruptRef::Set(id), None) => write!(f, "{side:?} {id} fired"),
                (InterruptRef::Standing { kind, subject }, _) => {
                    write!(f, "{side:?} standing interrupt fired: ")?;
                    match (kind, subject) {
                        (StandingKind::CannotMeet, subject) => {
                            write!(f, "{subject} cannot meet its intent")
                        }
                        (StandingKind::WaitsForLeave, subject) => {
                            write!(f, "{subject} waits for the commander's leave")
                        }
                        (StandingKind::ShipLost, StandingSubject::Ship(ship)) => {
                            write!(f, "{ship} was lost")
                        }
                        (StandingKind::ShipLost, subject) => write!(f, "{subject} was lost"),
                        (StandingKind::CannotBringEnough, subject) => {
                            write!(f, "{subject} cannot bring the men it wants")
                        }
                        (StandingKind::LiftTurnedBack, subject) => {
                            write!(f, "{subject}'s lift is turned back")
                        }
                        (StandingKind::AsksLeaveToPursue, subject) => {
                            write!(f, "{subject} asks leave to pursue")
                        }
                    }
                }
            },
            EventKind::GroundFoundEmpty {
                side,
                hex,
                men,
                reported_day,
            } => write!(
                f,
                "{} men at {hex} find no enemy men ashore; the estimate of {men} men of day {reported_day} is cleared",
                side.name().to_lowercase()
            ),
            EventKind::IntentFormed {
                side,
                intent,
                parent,
            } => write!(f, "{side:?} staff forms {intent} under {parent}"),
            EventKind::IntentWithdrawn {
                side,
                intent,
                parent,
            } => write!(f, "{side:?} {intent} is withdrawn with {parent}"),
            EventKind::StationKept {
                escort,
                charge,
                in_station,
                waypoints,
                knots,
            } => {
                if !in_station {
                    let to = waypoints
                        .first()
                        .map_or_else(String::new, |hex| format!(" {hex}"));
                    return write!(
                        f,
                        "{escort} is out of station on its charge {charge} and steams for its hex{to} at cruise"
                    );
                }
                if waypoints.is_empty() {
                    return write!(
                        f,
                        "{escort} keeps station on its charge {charge}, which does not steam: it stops"
                    );
                }
                let to: Vec<String> = waypoints.iter().map(ToString::to_string).collect();
                match knots {
                    Some(knots) => write!(
                        f,
                        "{escort} keeps station on its charge {charge}: it takes its route through {} at {knots} kn",
                        to.join(", ")
                    ),
                    None => write!(
                        f,
                        "{escort} keeps station on its charge {charge}: it takes its route through {}",
                        to.join(", ")
                    ),
                }
            }
            EventKind::ChargeLost {
                escort,
                charge,
                hex,
            } => write!(
                f,
                "{escort} has lost its charge {charge} and stops at {hex}"
            ),
            EventKind::MeetingNotHeld {
                charge,
                enemy,
                hex,
                escorts,
            } => {
                let escorts: Vec<String> = escorts.iter().map(ToString::to_string).collect();
                write!(
                    f,
                    "{enemy} does not meet {charge} at {hex}: its escort {} was not passed",
                    escorts.join(", ")
                )
            }
            EventKind::ShipRearmed {
                ship,
                base,
                rounds,
                torpedoes,
                taken_t,
                rounds_aboard,
                tubes_fired,
                base_ammunition_t,
                ..
            } => write!(
                f,
                "{ship} takes {rounds} rounds and {torpedoes} torpedoes, {taken_t} t, from {base}: {rounds_aboard} rounds aboard and {tubes_fired} tubes fired; {base_ammunition_t} t of ammunition left at the base"
            ),
            EventKind::ReportSent {
                side,
                sender,
                report,
                listeners,
            } => {
                write!(f, "{} {sender} sends the report of #{}", side.name(), report.raw())?;
                for (at, listener) in listeners.iter().enumerate() {
                    write!(
                        f,
                        "{} {} to {}, {}",
                        if at == 0 { ":" } else { ";" },
                        listener.direction.name(),
                        listener.command,
                        listener.arrival.name()
                    )?;
                }
                Ok(())
            }
            EventKind::StateReportLate {
                command, as_of, ..
            } => write!(f, "{command}'s state report of {as_of} is late"),
            EventKind::StateReportLost {
                command, as_of, ..
            } => write!(f, "{command}'s state report of {as_of} is lost"),
            EventKind::BuiltFromPicture { day } => write!(
                f,
                "the world was built from a picture of the evening of day {day}"
            ),
            EventKind::ShipFuelledAtSea {
                ship,
                task_force,
                oiler,
                tons,
                fuel_t,
                oiler_cargo_fuel_t,
                restored,
                ..
            } => {
                write!(
                    f,
                    "{ship} took {tons} t of fuel at sea from {oiler}, {fuel_t} t aboard, {oiler} has {oiler_cargo_fuel_t} t of cargo fuel left"
                )?;
                if *restored {
                    write!(f, "; {task_force} is no longer out of fuel")?;
                }
                Ok(())
            }
            EventKind::FuellingBrokenOff {
                task_force,
                giver,
                hex,
            } => write!(
                f,
                "{task_force} took no fuel from {giver} at {hex}: the fuelling was broken off by an attack"
            ),
            EventKind::FuelOrderEnded {
                task_force,
                giver,
                reason,
            } => write!(
                f,
                "{task_force}'s order to fuel from {giver} ended: {}",
                match reason {
                    FuelOrderEnd::Full => "every ship is full",
                    FuelOrderEnd::GiverEmpty => "the giver holds no cargo fuel",
                    FuelOrderEnd::NoOiler =>
                        "the giver has no ship afloat that fuels others at sea",
                    FuelOrderEnd::NewRoute => "a new route was ordered",
                }
            ),
            EventKind::RationsLanded {
                base,
                task_force,
                men,
                nights,
                supply_t,
                ..
            } => write!(
                f,
                "the landing of {task_force} put {supply_t} t of rations into {base}: {nights} nights for the {men} men aboard"
            ),
            EventKind::HungerNightsChanged {
                unit,
                hex,
                before,
                nights,
            } => write!(
                f,
                "{unit} at {hex} has been hungry {nights} nights, from {before}"
            ),
            EventKind::FightSupplyCharged {
                side,
                hex,
                men_at_grips,
                wanted_t,
                from_ammunition_t,
                from_supply_t,
                supplied_share,
            } => write!(
                f,
                "the {} side's fight at {hex} wanted {wanted_t} t for {men_at_grips} men at grips: {from_ammunition_t} t from ammunition and {from_supply_t} t from supply, a supplied share of {supplied_share}",
                side.name()
            ),
            EventKind::BaseAmmunitionSpent {
                base,
                asked_t,
                paid_t,
                left_t,
            } => {
                if paid_t < asked_t {
                    write!(
                        f,
                        "{base} spent {paid_t} t of ammunition on the men at grips in a fight: {asked_t} t was asked and the stock could not cover it, {left_t} t left"
                    )
                } else {
                    write!(
                        f,
                        "{base} spent {paid_t} t of ammunition on the men at grips in a fight, {left_t} t left"
                    )
                }
            }
            EventKind::AmmunitionLanded {
                base,
                task_force,
                men,
                days,
                ammunition_t,
                ..
            } => write!(
                f,
                "the landing of {task_force} put {ammunition_t} t of ammunition into {base}: {days} days of fighting for the {men} men aboard"
            ),
        }
    }
}

/// An issuer's name in words, for the log's text.
fn issuer_name(issuer: Issuer) -> &'static str {
    match issuer {
        Issuer::Player => "the commander's",
        Issuer::Staff => "the staff's",
        Issuer::LocalCommander => "the local commander's",
    }
}

/// What a halted force waits to do, in words.
fn halt_work_words(work: HaltWork) -> &'static str {
    // The night stand (decision 0124, 2).
    if work.night {
        return match (work.bombard, work.cargo) {
            (true, true) => "stand for the night, bombard and work cargo",
            (true, false) => "stand for the night and bombard",
            (false, true) => "stand for the night and work cargo",
            (false, false) => "stand for the night",
        };
    }
    match (work.bombard, work.cargo) {
        (true, true) => "bombard and work cargo",
        (true, false) => "bombard",
        (false, true) => "work cargo",
        (false, false) => "do nothing",
    }
}

/// Whether a base is open: supply above zero and at least one ready aircraft
/// of an air group at it (decisions 0054 and 0067). The one place the test
/// is written.
pub fn base_is_open(supply_t: Fixed, aircraft_ready: u32) -> bool {
    supply_t > Fixed::ZERO && aircraft_ready > 0
}

/// Ship IDs written as a list, such as `ship:2, ship:4`.
/// The true event's words of what the people under an attack made of the
/// aircraft (decision 0167): "; recognised as aircraft_type:3" where they
/// recognised them, nothing where they did not, so that a world where no type
/// is recognised reads as it did before the rule.
fn recognised_suffix(
    f: &mut fmt::Formatter<'_>,
    recognised: Option<AircraftTypeId>,
) -> fmt::Result {
    match recognised {
        Some(_) => write!(f, "; {}", RecognisedWords(recognised)),
        None => Ok(()),
    }
}

/// What the people under an air attack or a raid made of the aircraft
/// (decision 0167): "recognised as aircraft_type:3", or "type not made out".
/// The attacked side's event says it either way.
pub struct RecognisedWords(pub Option<AircraftTypeId>);

impl fmt::Display for RecognisedWords {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.0 {
            Some(aircraft_type) => write!(f, "recognised as {aircraft_type}"),
            None => write!(f, "type not made out"),
        }
    }
}

fn join_ships(ships: &[ShipId]) -> String {
    let names: Vec<String> = ships.iter().map(ShipId::to_string).collect();
    names.join(", ")
}

/// One entry in the log.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Event {
    pub id: EventId,
    pub day: u32,
    /// `None` for orders intake and the end-of-day phases.
    pub pulse: Option<Pulse>,
    pub kind: EventKind,
    pub causes: Vec<Cause>,
    /// Present for computed quantities when breakdown recording is on.
    pub breakdown: Option<Breakdown>,
}

/// The text of a `ReportReceived` event. A report logged on day 0, before the
/// first day, is a contact a scenario started a side with, and says so
/// (decision 8 of `docs/decisions/0021-readiness-and-alarm.md`). Every other
/// report reads as before.
fn write_report(
    f: &mut fmt::Formatter<'_>,
    (side, contact, hex, ship_count, kind): (Side, ContactId, Hex, u32, ShipKind),
    created: bool,
    held_at_start: bool,
) -> fmt::Result {
    write!(
        f,
        "{} report: {ship_count} {}, {}, at {hex}{}: {} {}",
        side.name(),
        if ship_count == 1 { "ship" } else { "ships" },
        kind.name(),
        if held_at_start {
            ", held at the start"
        } else {
            ""
        },
        if created { "new" } else { "refreshed" },
        contact.shown()
    )
}

impl fmt::Display for Event {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let pulse = self.pulse.map_or("-", Pulse::name);
        write!(f, "#{} D{} {pulse:<9} ", self.id.raw(), self.day)?;
        match &self.kind {
            EventKind::ReportReceived {
                side,
                contact,
                hex,
                ship_count,
                kind,
                created,
            } if self.day == 0 => write_report(
                f,
                (*side, *contact, *hex, *ship_count, *kind),
                *created,
                true,
            )?,
            kind => write!(f, "{kind}")?,
        }
        if !self.causes.is_empty() {
            let causes: Vec<String> = self.causes.iter().map(Cause::to_string).collect();
            write!(f, "  <- {}", causes.join(", "))?;
        }
        Ok(())
    }
}

/// All events of a game, in the order they happened.
#[derive(Clone, PartialEq, Eq, Debug, Default)]
pub struct EventLog {
    events: Vec<Event>,
}

impl EventLog {
    pub(crate) fn record(
        &mut self,
        day: u32,
        pulse: Option<Pulse>,
        kind: EventKind,
        causes: Vec<Cause>,
        breakdown: Option<Breakdown>,
    ) -> EventId {
        let raw = u32::try_from(self.events.len() + 1).expect("ran out of event IDs");
        let id = EventId::new(raw);
        self.events.push(Event {
            id,
            day,
            pulse,
            kind,
            causes,
            breakdown,
        });
        id
    }

    /// The latest event, if any, whose kind `keep` accepts.
    pub(crate) fn latest_where(&self, keep: impl Fn(&EventKind) -> bool) -> Option<EventId> {
        self.events
            .iter()
            .rev()
            .find(|event| keep(&event.kind))
            .map(|event| event.id)
    }

    /// The events of one day, latest first, walked back from the end of the
    /// log (days run in order, so the walk stops at the day before).
    pub(crate) fn of_day_latest_first(&self, day: u32) -> impl Iterator<Item = &Event> {
        self.events
            .iter()
            .rev()
            .take_while(move |event| event.day >= day)
            .filter(move |event| event.day == day)
    }

    /// The ID the next recorded event will get.
    pub(crate) fn next_id(&self) -> EventId {
        EventId::new(u32::try_from(self.events.len() + 1).expect("ran out of event IDs"))
    }

    pub fn len(&self) -> usize {
        self.events.len()
    }

    pub fn is_empty(&self) -> bool {
        self.events.is_empty()
    }

    pub fn iter(&self) -> impl Iterator<Item = &Event> {
        self.events.iter()
    }

    /// The events from position `start` onward. A [`crate::DaySummary`]
    /// gives the position where its day begins.
    pub fn from_index(&self, start: usize) -> &[Event] {
        self.events.get(start..).unwrap_or(&[])
    }

    pub fn get(&self, id: EventId) -> Option<&Event> {
        let index = usize::try_from(id.raw()).ok()?.checked_sub(1)?;
        self.events.get(index)
    }

    /// Every event that involves the entity, in order.
    pub fn involving(&self, entity: EntityRef) -> impl Iterator<Item = &Event> {
        self.events
            .iter()
            .filter(move |event| event.kind.entities().contains(&entity))
    }

    /// Everything that led to an event, following causes back through earlier
    /// events until only orders remain. Nearest causes come first and nothing
    /// is listed twice.
    pub fn trace(&self, id: EventId) -> Vec<Cause> {
        let mut found: Vec<Cause> = Vec::new();
        let mut frontier = vec![id];
        while let Some(current) = frontier.pop() {
            let Some(event) = self.get(current) else {
                continue;
            };
            for cause in &event.causes {
                if found.contains(cause) {
                    continue;
                }
                found.push(*cause);
                if let Cause::Event(earlier) = cause {
                    frontier.push(*earlier);
                }
            }
        }
        found
    }
}

/// How a claim reads, in the event log and wherever a claim is listed
/// (decision 0056). A surface fight's claim reads as it always has: "Allied
/// claims 2 enemy ships sunk at (5,0)". A claim from the air reads "Allied
/// claim over (7,-5): 12 aircraft shot down" or "Allied claim at (9,0): 5
/// hits on a carrier, 0 sunk".
#[allow(clippy::too_many_arguments)]
pub fn claim_text(
    side: Side,
    hex: Hex,
    claimed_sunk: u32,
    claimed_hits: u32,
    claimed_kind: Option<ShipKind>,
    claimed_shot_down: u32,
    claimed_on_ground: u32,
    from_air: bool,
    from_bombardment: bool,
    ground_types: &[(AircraftTypeId, u32)],
    by_unit: &[UnitClaim],
    air_types: &[(AircraftTypeId, u32)],
) -> String {
    let named = |list: &[(AircraftTypeId, u32)]| -> Vec<(String, u32)> {
        list.iter()
            .map(|(aircraft_type, aircraft)| (aircraft_type.to_string(), *aircraft))
            .collect()
    };
    let types = named(ground_types);
    let in_the_air = named(air_types);
    let units: Vec<(String, UnitClaim)> = by_unit
        .iter()
        .map(|line| (line.unit.to_string(), *line))
        .collect();
    claim_words(
        side,
        hex,
        claimed_sunk,
        claimed_hits,
        claimed_kind,
        claimed_shot_down,
        claimed_on_ground,
        from_air,
        from_bombardment,
        &types,
        &units,
        &in_the_air,
    )
}

/// A claim's words, with its types and its units each given in the words
/// to say them by: the log's own (`aircraft_type:3`, `air_group:2`) in
/// [`claim_text`], names in a seat's tools.
#[allow(clippy::too_many_arguments)]
pub fn claim_words(
    side: Side,
    hex: Hex,
    claimed_sunk: u32,
    claimed_hits: u32,
    claimed_kind: Option<ShipKind>,
    claimed_shot_down: u32,
    claimed_on_ground: u32,
    from_air: bool,
    from_bombardment: bool,
    types: &[(String, u32)],
    units: &[(String, UnitClaim)],
    air_types: &[(String, u32)],
) -> String {
    if from_bombardment {
        return format!(
            "{} claim from the sea at {hex}: {claimed_on_ground} aircraft destroyed on the ground",
            side.name()
        );
    }
    if !from_air {
        return format!(
            "{} claims {claimed_sunk} enemy {} sunk at {hex}",
            side.name(),
            if claimed_sunk == 1 { "ship" } else { "ships" }
        );
    }
    let (place, words) = air_claim_words(
        claimed_sunk,
        claimed_hits,
        claimed_kind.map(ShipKind::name),
        claimed_shot_down,
        claimed_on_ground,
        types,
        air_types,
    );
    format!(
        "{} claim {place} {hex}: {words}{}",
        side.name(),
        claimed_by_words(units)
    )
}

/// The clause at the end of a raid's claim that names who claims what
/// (decision 0180, 10): "; claimed by U: 2 shot down; by V: 16 on the
/// ground, 3 hits", each unit by the words given for it, and only what it
/// claims. Nothing where the claim names no unit.
pub fn claimed_by_words(units: &[(String, UnitClaim)]) -> String {
    let mut words = String::new();
    for (index, (unit, line)) in units.iter().enumerate() {
        let mut parts: Vec<String> = Vec::new();
        if line.shot_down != 0 {
            parts.push(format!("{} shot down", line.shot_down));
        }
        if line.on_ground != 0 {
            parts.push(format!("{} on the ground", line.on_ground));
        }
        if line.hits != 0 {
            parts.push(format!(
                "{} {}",
                line.hits,
                if line.hits == 1 { "hit" } else { "hits" }
            ));
        }
        let by = if index == 0 { "claimed by" } else { "by" };
        words.push_str(&format!("; {by} {unit}: {}", parts.join(", ")));
    }
    words
}

/// What a claim from the air says, in words, and whether it is a claim
/// "over" a place (aircraft shot down only) or "at" it: `("at", "5 hits on a
/// carrier, 0 sunk")`.
pub fn air_claim_words(
    claimed_sunk: u32,
    claimed_hits: u32,
    claimed_kind: Option<&str>,
    claimed_shot_down: u32,
    claimed_on_ground: u32,
    ground_types: &[(String, u32)],
    air_types: &[(String, u32)],
) -> (&'static str, String) {
    // The types made out, after the number (decisions 0179, 8, and 0210,
    // 8): " (3 of T, 2 of U)", or nothing where the list is empty.
    let types_of = |list: &[(String, u32)]| -> String {
        if list.is_empty() {
            return String::new();
        }
        let each: Vec<String> = list
            .iter()
            .map(|(aircraft_type, aircraft)| format!("{aircraft} of {aircraft_type}"))
            .collect();
        format!(" ({})", each.join(", "))
    };
    let mut parts: Vec<String> = Vec::new();
    if let Some(kind) = claimed_kind {
        parts.push(format!(
            "{claimed_hits} {} on a {kind}",
            if claimed_hits == 1 { "hit" } else { "hits" },
        ));
        parts.push(format!("{claimed_sunk} sunk"));
    }
    // Aircraft shot down are always told, if only as none, when the claim
    // says nothing else of what was hit.
    if claimed_shot_down != 0 || (claimed_kind.is_none() && claimed_on_ground == 0) {
        parts.push(format!(
            "{claimed_shot_down} aircraft shot down{}",
            types_of(air_types)
        ));
    }
    if claimed_on_ground != 0 {
        parts.push(format!(
            "{claimed_on_ground} aircraft destroyed on the ground{}",
            types_of(ground_types)
        ));
    }
    // Only aircraft shot down: a claim over the hex.
    let place = if claimed_kind.is_none() && claimed_on_ground == 0 {
        "over"
    } else {
        "at"
    };
    (place, parts.join(", "))
}

/// The find of a pursuit notice in words: "contact 3, a carrier, 2 pulses
/// old, reported at (4, 5); would close to (3, 4) in 7 hours".
fn write_pursuit_find(
    f: &mut fmt::Formatter<'_>,
    notice: &crate::command::PursuitNotice,
) -> fmt::Result {
    if let Some(find) = &notice.find {
        write!(
            f,
            " contact {}, {}, {} pulses old, reported at {}; would close to {}",
            find.contact,
            find.kind.name(),
            find.age,
            find.reported_at,
            find.to
        )?;
        if let Some(hours) = find.hours {
            write!(f, " in {hours} hours")?;
        }
        write!(f, ", leaving {}", notice.intent)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_system_hit_reads_as_a_change_or_as_no_change() {
        let hit = |before, after| EventKind::SystemHit {
            round: 2,
            ship: ShipId::new(5),
            system: ShipSystem::Propulsion,
            before,
            after,
        };
        assert_eq!(
            hit(SystemState::Working, SystemState::Degraded).to_string(),
            "round 2: ship:5 propulsion hit, working -> degraded"
        );
        assert_eq!(
            hit(SystemState::Out, SystemState::Out).to_string(),
            "round 2: ship:5 propulsion hit, no change, still out"
        );
    }

    #[test]
    fn a_system_hit_is_about_its_ship() {
        let kind = EventKind::SystemHit {
            round: 1,
            ship: ShipId::new(3),
            system: ShipSystem::MainBattery,
            before: SystemState::Working,
            after: SystemState::Degraded,
        };
        assert_eq!(kind.entities(), vec![EntityRef::Ship(ShipId::new(3))]);
    }

    #[test]
    fn detaching_ships_reads_clearly() {
        let ships = |raw: &[u32]| raw.iter().map(|n| ShipId::new(*n)).collect::<Vec<_>>();
        let sent = EventKind::ShipsDetached {
            parent: TaskForceId::new(1),
            new_force: TaskForceId::new(2),
            ships: ships(&[2]),
            escorts: ships(&[4, 5]),
            reason: DetachReason::SentHome,
        };
        assert_eq!(
            sent.to_string(),
            "task_force:1 sent ship:2 home as task_force:2 with escorts ship:4, ship:5"
        );
        let alone = EventKind::ShipsDetached {
            parent: TaskForceId::new(1),
            new_force: TaskForceId::new(2),
            ships: ships(&[2, 3]),
            escorts: Vec::new(),
            reason: DetachReason::SentHome,
        };
        assert_eq!(
            alone.to_string(),
            "task_force:1 sent ship:2, ship:3 home as task_force:2"
        );
        let left = EventKind::ShipsDetached {
            parent: TaskForceId::new(1),
            new_force: TaskForceId::new(3),
            ships: ships(&[6]),
            escorts: Vec::new(),
            reason: DetachReason::LeftBehind,
        };
        assert_eq!(
            left.to_string(),
            "task_force:1 left ship:6 behind as task_force:3"
        );
        assert_eq!(
            left.entities(),
            vec![
                EntityRef::TaskForce(TaskForceId::new(1)),
                EntityRef::TaskForce(TaskForceId::new(3)),
                EntityRef::Ship(ShipId::new(6)),
            ]
        );
        let retired = EventKind::ForceRetired {
            task_force: TaskForceId::new(1),
        };
        assert_eq!(retired.to_string(), "task_force:1 retires");
        assert_eq!(
            retired.entities(),
            vec![EntityRef::TaskForce(TaskForceId::new(1))]
        );
    }

    #[test]
    fn detach_reason_codes_are_stable() {
        assert_eq!(DetachReason::SentHome.code(), 1);
        assert_eq!(DetachReason::LeftBehind.code(), 2);
        assert_eq!(DetachReason::SentHome.name(), "sent home");
        assert_eq!(DetachReason::LeftBehind.name(), "left behind");
    }
}
