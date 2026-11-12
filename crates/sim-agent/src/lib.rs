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
//! * [`get_entity`]: everything the side knows of one unit or contact.
//! * [`explain`]: an event as the side may see it and its visible causes.
//!
//! Every answer serialises to JSON with `"format": "flagplot-agent"`,
//! `"version": 1` and a `"kind"`, and has a schema under `schemas/`. A
//! quantity of the game is a whole number if it counts things and a decimal
//! string if it is a [`sim_core::fixed::Fixed`], never a float.

mod answer;
mod briefing;
mod common;
mod explain;
mod lines;
mod queries;
mod text;
mod words;

pub use answer::{NotKnown, FORMAT, VERSION};
pub use briefing::{briefing, Briefing};
pub use briefing::{
    ActionPart, AircraftLoss, ArrivalsPart, AttackOnShip, BriefingBase, Claim, ContactsPart,
    DamagedShip, DroppedContact, DueArrival, Fight, Fired, FuelWarning, Heading, LossesPart,
    NotThere, OrdersPart, PilotWarning, Raid, RefusedOrder, ReplacementStream, ReportedContact,
    ShipRangeWarning, Strike, SunkShip, SupplyWarning, SystemChange, TiredWarning, Warnings,
};
pub use common::{Dec, Handle, Place};
pub use explain::{
    explain, Explain, ExplainAnswer, ExplainNode, Explainer, NodeBreakdown, NodeFactor, DEPTH_LIMIT,
};
pub use lines::{AirGroupLine, BaseLine, ContactLine, ForceLine, FuelLine};
pub use queries::{
    get_entity, list_bases, list_contacts, list_forces, AirGroupDetail, Area, BaseDetail,
    ContactDetail, Detail, Entity, EntityAnswer, ForceDetail, ListBases, ListContacts, ListForces,
    ShipDetail, ShipLine, SystemStates,
};
