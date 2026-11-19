//! Flag Plot agent interface: what a player is told and what it can ask
//! (decision 0048). Server mode and the MCP wrapper are still to come.
//!
//! # The one rule
//!
//! Everything here is built from a [`sim_core::view::SideView`] and nothing
//! else. No function of this crate takes the world, the umpire's view or the
//! full event log, so what a side is told is what decisions 0012 and 0047
//! allow, by construction. This crate has no access to files and does not
//! depend on `sim-data` or `sim-cli`.
//!
//! # The answers
//!
//! * [`briefing`] and [`Briefing::to_text`]: the situation of one side at the
//!   start of a day, as data and as text made from that data.
//! * [`list_forces`], [`list_bases`], [`list_contacts`]: one line per item,
//!   optionally within an area.
//! * [`get_entity`]: everything the side knows of one unit or contact (a
//!   ship, task force, air group, land unit or base of its own).
//! * [`explain`]: an event as the side may see it and its visible causes.
//! * [`command_state`], [`plans`], [`propose`], [`plan_order`], [`staff_orders`], [`why`] and
//!   the commands a seat queues (`set_intent`, `withdraw_intent`,
//!   `set_limit`, `set_leave`, `approve`, `cancel_plan`): the tools of a seat
//!   whose side has a staff (decision 0107). Each answers from a view of the
//!   world as the coming day will find it, which only the session makes.
//! * [`measure`]: air distance, sea route, the passage of a task force, its
//!   days at the yard and the air reach of an air group, with the functions the rules use (decision
//!   0049).
//!
//! Every answer serialises to JSON with `"format": "flagplot-agent"`,
//! `"version": 10` and a `"kind"`, and has a schema under `schemas/`. A
//! quantity of the game is a whole number if it counts things and a decimal
//! string if it is a [`sim_core::fixed::Fixed`], never a float.

mod answer;
mod briefing;
mod choices;
mod commands;
mod common;
mod deny;
mod explain;
mod hold;
mod island;
mod lines;
mod measure;
mod neutralise;
mod objectives;
mod orders;
mod queries;
mod record;
mod reporting;
mod session;
mod staffing;
mod text;
mod told;
mod words;

pub use answer::{NotKnown, FORMAT, VERSION};
pub use briefing::{briefing, briefing_for_game, Briefing};
pub use briefing::{
    ActionPart, AircraftLoss, ArrivalsPart, AttackOnShip, AwayForce, BackToDefendLine,
    BaseChangesPart, BaseFormedLine, BaseLostLine, BaseTakenLine, BriefingBase, Claim,
    ContactsPart, DamagedShip, DefencesWornLine, DroppedContact, DueArrival, FatigueLine, FedLine,
    FedShareRestoredLine, FieldworksLine, Fight, FightLine, FireLine, Fired, FuelChange,
    FuelWarning, GroupLostWithBaseLine, HaltedLine, HaltsPart, Heading, HexSupply,
    HomePortLostLine, LandUnitsPart, LoadingCompleteLine, LossesPart, MenLostLine, MenMoved,
    NotThere, OrdersPart, OutOfReachWarning, PilotWarning, PropulsionWarning, Raid, RebasedLine,
    RefusedOrder, RepairedShip, ReplacementStream, ReportedContact, ReturnedForce,
    ShipLostWithBaseLine, ShipRangeWarning, SteamedOnLine, StocksYesterday, Strike,
    StrikeClearedLine, SunkShip, SupplyChange, SupplyWarning, SystemChange, TiredWarning,
    UnitDestroyedLine, UnitDividedLine, Warnings,
};
pub use briefing::{
    ForceChangesPart, FormedLine, JoinedLine, LoadReserveLine, RefuellingHeldLine, StoodDownLine,
};
pub use choices::{ChoiceLine, Choices};
pub use commands::{
    command_from_json, command_to_json, staff_decisions_from_json, staff_decisions_to_json,
};
pub use common::{Dec, Handle, Place};
pub use deny::{deny_measure, DenyMeasure};
pub use explain::{
    explain, Explain, ExplainAnswer, ExplainNode, Explainer, NodeBreakdown, NodeFactor, DEPTH_LIMIT,
};
pub use hold::{hold_measure, HoldMeasure};
pub use island::{island_measure, IslandMeasure, PlaceMeasure, DEFAULT_FLOOR_DAYS};
pub use lines::{
    AboardLine, AirGroupLine, BaseLine, ContactLine, ForceLine, FuelLine, HaltLine, LandUnitLine,
    PropulsionLine,
};
pub use measure::{
    measure, measure_in, measure_json, AirDistance, AirReach, Assumptions, MeasureAnswer,
    MeasureContext, Moment, Passage, PassageSpeed, PlaceArg, Question, ReachLine, Resolved,
    SeaRoute, ShipFuel, SpeedArg, Yard, YardCheck, YardShip,
};
pub use neutralise::{neutralise_measure, NeutraliseMeasure};
pub use objectives::{
    result_part, KnownLine, LevelsPart, ObjectiveLine, ObjectivesPart, ResultPart, ScoreLine,
    ShipKindPoints, ShipsOfKind, SideResultPart,
};
pub use orders::{is_orderable_unit, order_from_json, order_to_json, unit_text};
pub use queries::{
    get_entity, list_bases, list_contacts, list_forces, AirGroupDetail, Area, BaseDetail,
    ContactDetail, Detail, EnemyBaseDetail, EnemyBaseLine, Entity, EntityAnswer, ForceDetail,
    LandUnitDetail, ListBases, ListContacts, ListForces, ShipDetail, ShipLine, SystemStates,
    TroopsAboard,
};
pub use record::{
    parse_seat, seat_name, CallError, Post, Seat, SeatKind, TranscriptEntry, COMMAND_TOOLS, TOOLS,
};
pub use session::{OrdersInsideDay, Session, NOT_KNOWN};
pub use staffing::{
    command_state, plan_order, plans, propose, staff_orders, why, STAFF_DEFAULT_INTERVAL_DAYS,
};
pub use told::{told_as, DayPart, STOP_ON};
