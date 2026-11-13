//! The briefing: the situation of one side at the start of a day
//! (decisions 3 to 7 of 0048).
//!
//! It is built from a [`SideView`] and nothing else. It reports the day just
//! resolved, from the events the side may see (decision 0047), and the state
//! now, from the view. The view's day is the last day resolved, so a
//! briefing taken at the start of day D, before orders, is taken when the
//! view's day is D - 1: it is "day D" and "yesterday" is D - 1. At day 1
//! nothing has been resolved and only the state is reported.
//!
//! The data here is the truth of the briefing. [`Briefing::to_text`] makes
//! the text from it and from nothing else.

use std::collections::{BTreeMap, BTreeSet};

use serde::Serialize;
use sim_core::events::{EventKind, SupplyUse, Weapon};
use sim_core::fixed::Fixed;
use sim_core::hex::Hex;
use sim_core::ids::{AirGroupId, BaseId, ContactId, EntityRef, ShipId};
use sim_core::orders::{AirMission, Cargo, Reach};
use sim_core::side_events::{SideEvent, SideEventKind};
use sim_core::state::{ShipLocation, TaskForceStatus};
use sim_core::view::SideView;

use crate::answer::{FORMAT, VERSION};
use crate::common::{cruise_range_nm, Ctx, Dec, Handle, Place};
use crate::lines::{
    air_group_line, fed_last_night, force_line, is_afloat, is_listed, land_unit_line, AirGroupLine,
    ContactLine, ForceLine, LandUnitLine, PropulsionLine,
};
use crate::objectives::{objectives_part, result_part, ObjectivesPart};
use crate::words;

/// The briefing: eleven parts, present every day. The objectives (decision
/// 0054) come second, after the heading; they say `none` when the scenario
/// has none.
#[derive(Clone, PartialEq, Eq, Debug, Serialize)]
pub struct Briefing {
    pub format: &'static str,
    pub version: u32,
    pub kind: &'static str,
    /// 1. The side, the day, the scenario.
    pub heading: Heading,
    /// 1b. The side's mission and objectives, its progress as far as it can
    /// know it, and at the end of the game the umpire's result (decision
    /// 0054).
    pub objectives: ObjectivesPart,
    /// 2. Orders accepted yesterday and those refused, with the reason.
    pub orders: OrdersPart,
    /// 3. Contacts that changed yesterday, then every contact held now.
    pub contacts: ContactsPart,
    /// 4. Fights, strikes, raids and attacks yesterday.
    pub action: ActionPart,
    /// 5. Ships and aircraft lost, and what the side claims.
    pub losses: LossesPart,
    /// 6. Task forces with a ship afloat, and those away at the yard.
    pub task_forces: Vec<ForceLine>,
    /// 6b. The forces away at the yard (decision 0069), with the day each is
    /// back. Empty, and left out of the data and the text, when none is away,
    /// so a side that has nothing at the yard sees no such part.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub away_at_yard: Vec<AwayForce>,
    /// 7. Air groups.
    pub air_groups: Vec<AirGroupLine>,
    /// 8. Bases.
    pub bases: Vec<BriefingBase>,
    /// 8b. The side's land units and what happened to their men yesterday
    /// (decision 0066, 21). Left out of the data and the text when the side
    /// has no land unit, so a scenario without any is briefed as it was.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub land_units: Option<LandUnitsPart>,
    /// 9. Arrivals due and replacement streams.
    pub arrivals: ArrivalsPart,
    /// 10. The five warnings.
    pub warnings: Warnings,
}

#[derive(Clone, PartialEq, Eq, Debug, Serialize)]
pub struct Heading {
    /// `allied` or `japanese`.
    pub side: &'static str,
    /// The day that is about to be played.
    pub day: u32,
    /// The last day of the game: it ends when this day has been resolved.
    /// `None` if the caller of `briefing` did not say.
    pub last_day: Option<u32>,
    /// The day the report covers: `None` on day 1.
    pub yesterday: Option<u32>,
    pub scenario: String,
    pub rules_in_play: Rules,
}

/// Which optional rules are in play in this game. The text leaves out what
/// a rule that is not in play would make meaningless.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize)]
pub struct Rules {
    /// Air wear and repair: aircraft come back damaged and are repaired.
    pub wear: bool,
    /// Fatigue and rest, of air groups and task forces.
    pub fatigue: bool,
    /// Some air group has its pilots counted.
    pub pilots: bool,
    /// Bases spend supply.
    pub supply: bool,
}

#[derive(Clone, PartialEq, Eq, Debug, Serialize)]
pub struct OrdersPart {
    pub accepted: u32,
    pub refused: Vec<RefusedOrder>,
}

#[derive(Clone, PartialEq, Eq, Debug, Serialize)]
pub struct RefusedOrder {
    pub order: u32,
    pub target: Handle,
    pub reason: String,
    pub event: u32,
}

#[derive(Clone, PartialEq, Eq, Debug, Serialize)]
pub struct ContactsPart {
    /// Contacts made yesterday.
    pub new: Vec<ReportedContact>,
    /// Every refresh of a contact yesterday, in order, with what it changed.
    pub refreshed: Vec<ReportedContact>,
    pub dropped: Vec<DroppedContact>,
    /// Contacts a strike or search found not to be there.
    pub not_there: Vec<NotThere>,
    /// Every contact held now.
    pub held: Vec<ContactLine>,
}

#[derive(Clone, PartialEq, Eq, Debug, Serialize)]
pub struct ReportedContact {
    pub contact: Handle,
    pub place: Place,
    pub ship_count: u32,
    pub taken_for: &'static str,
    pub event: u32,
    /// For a refresh: what the contact held before, where the report changes
    /// it. `None` where it does not change, and for a new contact.
    pub was_ship_count: Option<u32>,
    pub was_taken_for: Option<&'static str>,
    pub was_place: Option<Place>,
}

#[derive(Clone, PartialEq, Eq, Debug, Serialize)]
pub struct DroppedContact {
    pub contact: Handle,
    pub age_pulses: u32,
    pub event: u32,
}

#[derive(Clone, PartialEq, Eq, Debug, Serialize)]
pub struct NotThere {
    pub contact: Handle,
    pub place: Place,
    pub event: u32,
}

#[derive(Clone, PartialEq, Eq, Debug, Serialize)]
pub struct ActionPart {
    /// Surface fights the side knew of, from the event that ended each.
    pub fights: Vec<Fight>,
    /// What the side's own ships fired, by firer and weapon.
    pub fired: Vec<Fired>,
    /// What the side's own fighters fired in fights in the air.
    pub air_fire: Vec<AirFire>,
    /// What the side's own ships and bases fired at raids.
    pub guns: Vec<Guns>,
    /// Strikes and attacks flown by the side's own air groups.
    pub strikes: Vec<Strike>,
    /// Raids on the side's bases.
    pub raids_on_bases: Vec<Raid>,
    /// Attacks on the side's ships, by target and weapon.
    pub attacks_on_ships: Vec<AttackOnShip>,
}

#[derive(Clone, PartialEq, Eq, Debug, Serialize)]
pub struct Fight {
    /// The side's own force in the fight. The enemy's is not named
    /// (decision 0065).
    pub force: Handle,
    pub place: Place,
    /// The round in which it ended.
    pub rounds: u32,
    /// `slipped_away`, `force_sunk` or `broke_off` (the side's own force was
    /// sunk or broke off), `passed_unseen`, `max_rounds`, or `contact_lost`
    /// (the enemy's force was gone, sunk or broken off, and the side cannot
    /// tell which).
    pub ended: &'static str,
    pub event: u32,
}

#[derive(Clone, PartialEq, Eq, Debug, Serialize)]
pub struct Fired {
    pub firer: Handle,
    pub weapon: &'static str,
    pub salvos: u32,
    pub shots: Dec,
}

#[derive(Clone, PartialEq, Eq, Debug, Serialize)]
pub struct Strike {
    /// `naval_strike_launched`, `base_strike_launched`,
    /// `strike_found_nothing`, `air_attack` or `base_attack`.
    pub kind: &'static str,
    pub air_group: Handle,
    /// The contact or base it was flown at. An attack on a ship names none:
    /// the ship is the enemy's (decision 0065).
    pub target: Option<Handle>,
    pub place: Option<Place>,
    /// Aircraft launched or attacking.
    pub aircraft: Option<Dec>,
    pub weapon: Option<&'static str>,
    pub distance_nm: Option<Dec>,
    pub event: u32,
    /// What the strike reported back: the events that cite its attack or its
    /// finding nothing. Empty for a launch, and for an attack that reported
    /// nothing.
    pub reports: Vec<StrikeReport>,
}

/// What a strike led to.
#[derive(Clone, PartialEq, Eq, Debug, Serialize)]
pub struct StrikeReport {
    /// `contact_new`, `contact_refreshed`, `contact_dropped`,
    /// `contact_not_there` or `claim`.
    pub result: &'static str,
    pub contact: Option<Handle>,
    pub ship_count: Option<u32>,
    pub taken_for: Option<&'static str>,
    pub place: Option<Place>,
    pub claimed_sunk: Option<u32>,
    pub event: u32,
}

/// Fire of the side's own fighters in a fight in the air: shots, never hits.
#[derive(Clone, PartialEq, Eq, Debug, Serialize)]
pub struct AirFire {
    /// `patrol` or `escorts`: which of its own aircraft fired.
    pub firing: &'static str,
    pub firer: Handle,
    /// `patrol`, `escorts` or `bombers`: what they fired at.
    pub at: &'static str,
    pub shots: Dec,
    pub event: u32,
}

/// Fire of the side's own guns, in ships or at a base, at a raid.
#[derive(Clone, PartialEq, Eq, Debug, Serialize)]
pub struct Guns {
    /// The task force or base that fired.
    pub by: Handle,
    pub shots: Dec,
    pub event: u32,
}

#[derive(Clone, PartialEq, Eq, Debug, Serialize)]
pub struct Raid {
    pub base: Handle,
    /// Aircraft that attacked.
    pub aircraft: Dec,
    pub hits: u32,
    pub hits_on_aircraft: u32,
    pub hits_on_supply: u32,
    pub event: u32,
}

#[derive(Clone, PartialEq, Eq, Debug, Serialize)]
pub struct AttackOnShip {
    /// `ship` for a salvo fired at it, `aircraft` for an attack from the air.
    /// The ship or air group that attacked is not named (decision 0065).
    pub by: &'static str,
    pub against: Handle,
    /// `None` for an air attack, whose weapon the side is not told.
    pub weapon: Option<&'static str>,
    pub attacks: u32,
    /// The aircraft counted in the air attacks.
    pub aircraft: Option<Dec>,
    /// The side's own numbers of the air attacks counted here; empty for
    /// salvos fired at the ship.
    pub events: Vec<u32>,
}

#[derive(Clone, PartialEq, Eq, Debug, Serialize)]
pub struct LossesPart {
    pub ships_sunk: Vec<SunkShip>,
    /// Ships damaged and not sunk.
    pub ships_damaged: Vec<DamagedShip>,
    /// Aircraft and pilots lost, by group, with the cause.
    pub aircraft: Vec<AircraftLoss>,
    /// Aircraft that were in the air when their own deck was lost and landed
    /// at another carrier or a base (decision 0059). Left out when none.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub aircraft_landed_elsewhere: Vec<LandedLine>,
    /// Air groups moved to another base by a rebase order on the day just
    /// resolved (decision 0091). Left out when none. Damaged aircraft left
    /// behind are counted in `aircraft` as written off.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub air_groups_rebased: Vec<RebasedLine>,
    /// What the side claims of the enemy: claims received yesterday.
    pub claims: Vec<Claim>,
}

#[derive(Clone, PartialEq, Eq, Debug, Serialize)]
pub struct LandedLine {
    pub from: Handle,
    /// The group that received them: an existing one, or one formed by it.
    pub to: Handle,
    /// The ship or base they landed at.
    pub at: Handle,
    pub aircraft: u32,
    pub pilots: u32,
    /// Whether `to` was formed by this landing.
    pub formed: bool,
    pub event: u32,
}

/// An air group moved by a rebase order (decision 0091).
#[derive(Clone, PartialEq, Eq, Debug, Serialize)]
pub struct RebasedLine {
    pub air_group: Handle,
    pub from: Handle,
    pub to: Handle,
    pub distance_nm: Dec,
    pub transfer_range_nm: Dec,
    /// Ready aircraft that went with the group.
    pub ready: u32,
    /// Pilots that went with it; `None` where the group's pilots are not
    /// counted.
    pub pilots: Option<u32>,
    pub event: u32,
}

#[derive(Clone, PartialEq, Eq, Debug, Serialize)]
pub struct SunkShip {
    pub ship: Handle,
    pub task_force: Handle,
    pub event: u32,
}

#[derive(Clone, PartialEq, Eq, Debug, Serialize)]
pub struct DamagedShip {
    pub ship: Handle,
    pub hits: u32,
    pub flotation_after: Option<Dec>,
    pub fire_after: Option<Dec>,
    pub systems: Vec<SystemChange>,
    pub events: Vec<u32>,
}

#[derive(Clone, PartialEq, Eq, Debug, Serialize)]
pub struct SystemChange {
    pub system: &'static str,
    pub before: &'static str,
    pub after: &'static str,
}

fn is_zero(n: &u32) -> bool {
    *n == 0
}

#[derive(Clone, PartialEq, Eq, Debug, Serialize)]
pub struct AircraftLoss {
    pub air_group: Handle,
    pub shot_down: u32,
    pub destroyed_on_ground: u32,
    pub written_off: u32,
    pub lost_with_ship: u32,
    /// Aircraft that ran out of fuel on the way home from a sortie beyond
    /// their radius (decision 0059). Left out when 0, so a game with no
    /// reach order reads as it did.
    #[serde(skip_serializing_if = "is_zero")]
    pub ditched: u32,
    /// Aircraft that came down in the sea for want of a deck (decision 9 of
    /// 0059). Left out when 0.
    #[serde(skip_serializing_if = "is_zero")]
    pub no_deck: u32,
    pub pilots_lost: u32,
    pub events: Vec<u32>,
}

/// What the side's own people believe they did in a fight or a raid
/// (decisions 0014 and 0056). It can be wrong.
#[derive(Clone, PartialEq, Eq, Debug, Serialize)]
pub struct Claim {
    pub place: Place,
    /// Enemy ships believed sunk.
    pub claimed_sunk: u32,
    /// Hits believed scored on ships, from the air.
    pub claimed_hits: u32,
    /// The kind of ship the attackers took the contact for, in a claim of
    /// ships from the air.
    pub taken_for: Option<&'static str>,
    /// Enemy aircraft believed shot down.
    pub claimed_shot_down: u32,
    /// Enemy aircraft believed destroyed on the ground.
    pub claimed_on_ground: u32,
    /// True for a claim of a raid, false for one of a fight at sea.
    pub from_air: bool,
    /// The event that told the side. `explain` on it shows the side's own
    /// firing it rests on.
    pub event: u32,
}

/// A force away at the yard (decision 0069).
#[derive(Clone, PartialEq, Eq, Debug, Serialize)]
pub struct AwayForce {
    pub task_force: Handle,
    /// The base it left from.
    pub port: Handle,
    pub ships: Vec<Handle>,
    /// The day it is back, at the start of that day.
    pub return_day: u32,
    /// Whether that day is within the game: `None` when the last day is not
    /// known.
    pub returns_before_end: Option<bool>,
    pub last_day: Option<u32>,
}

/// A ship a base mended yesterday (decision 0069), from the side's own
/// `ShipRepaired` event and the ship as she is now.
#[derive(Clone, PartialEq, Eq, Debug, Serialize)]
pub struct RepairedShip {
    pub ship: Handle,
    pub task_force: Handle,
    /// The points spent on her yesterday.
    pub points: Dec,
    /// What she has left of fire and flotation damage after it.
    pub fire_damage: Dec,
    pub flotation_damage: Dec,
    /// Systems put back to working yesterday.
    pub restored: Vec<&'static str>,
    /// The systems she still has degraded, and the ones out.
    pub systems_degraded: Vec<&'static str>,
    pub systems_out: Vec<&'static str>,
    pub event: u32,
}

/// A force that came back from the yard at a base yesterday.
#[derive(Clone, PartialEq, Eq, Debug, Serialize)]
pub struct ReturnedForce {
    pub task_force: Handle,
    pub ships: Vec<Handle>,
    pub event: u32,
}

#[derive(Clone, PartialEq, Eq, Debug, Serialize)]
pub struct BriefingBase {
    pub base: Handle,
    pub place: Place,
    pub supply_t: Dec,
    pub fuel_t: Dec,
    /// Supply paid out yesterday, all uses together.
    pub spent_yesterday_t: Dec,
    pub destroyed_yesterday_t: Dec,
    /// Supply divided by yesterday's spending, truncated to one decimal;
    /// `None` when nothing was spent.
    pub days_of_supply: Option<Dec>,
    /// Where the base's supply and fuel went and came from yesterday, by
    /// cause. `None` when nothing moved in or out of either stock.
    pub stocks_yesterday: Option<StocksYesterday>,
    /// Ships mended here yesterday by forward repair (decision 0069).
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub repaired_yesterday: Vec<RepairedShip>,
    /// Forces that came back here from the yard yesterday.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub returned_yesterday: Vec<ReturnedForce>,
}

/// The land units part (decision 0066, 21): every land unit of the side with
/// its men ashore and aboard, then what yesterday's events of the side say
/// happened to its men, then for each hex with men ashore the days of supply
/// its base holds.
#[derive(Clone, PartialEq, Eq, Debug, Serialize)]
pub struct LandUnitsPart {
    /// Every land unit of the side, in number order, destroyed ones too.
    pub units: Vec<LandUnitLine>,
    /// Men put ashore yesterday.
    pub landed_yesterday: Vec<MenMoved>,
    /// Men taken aboard from the shore yesterday.
    pub embarked_yesterday: Vec<MenMoved>,
    /// Bases formed yesterday by a landing.
    pub bases_formed_yesterday: Vec<BaseFormedLine>,
    /// Units divided yesterday.
    pub divided_yesterday: Vec<UnitDividedLine>,
    /// A load of a unit that finished yesterday.
    pub loading_complete_yesterday: Vec<LoadingCompleteLine>,
    /// How the men ashore were fed in the last night, a line for each hex.
    pub fed_last_night: Vec<FedLine>,
    /// Men lost yesterday, and to what.
    pub lost_yesterday: Vec<MenLostLine>,
    /// Units that lost their last man yesterday.
    pub destroyed_yesterday: Vec<UnitDestroyedLine>,
    /// For each hex with men of the side ashore, the days of supply its base
    /// holds at the present charge.
    pub supply_by_hex: Vec<HexSupply>,
}

/// Men of a unit put ashore from, or taken aboard, a ship.
#[derive(Clone, PartialEq, Eq, Debug, Serialize)]
pub struct MenMoved {
    pub land_unit: Handle,
    pub ship: Handle,
    pub task_force: Handle,
    pub place: Place,
    pub men: u32,
    /// The unit's men in the ship afterwards.
    pub aboard: u32,
    /// The unit's men ashore in the hex afterwards.
    pub ashore: u32,
    pub event: u32,
}

#[derive(Clone, PartialEq, Eq, Debug, Serialize)]
pub struct BaseFormedLine {
    pub base: Handle,
    pub place: Place,
    pub task_force: Handle,
    pub event: u32,
}

#[derive(Clone, PartialEq, Eq, Debug, Serialize)]
pub struct UnitDividedLine {
    /// The unit that kept its men ashore elsewhere.
    pub from: Handle,
    /// The new unit made of the men aboard.
    pub land_unit: Handle,
    pub task_force: Handle,
    pub men: u32,
    pub event: u32,
}

#[derive(Clone, PartialEq, Eq, Debug, Serialize)]
pub struct LoadingCompleteLine {
    pub task_force: Handle,
    pub land_unit: Handle,
    /// The base loaded from; `None` off a bare shore.
    pub base: Option<Handle>,
    /// `supply` or `fuel`; `None` for a load of men only.
    pub cargo: Option<&'static str>,
    pub event: u32,
}

/// The side's men ashore in one hex, fed or not, in the last night pulse.
#[derive(Clone, PartialEq, Eq, Debug, Serialize)]
pub struct FedLine {
    pub place: Place,
    /// The base that was asked; `None` where the side has none in the hex.
    pub base: Option<Handle>,
    pub men: u32,
    pub fed: u32,
    pub unfed: u32,
    pub event: u32,
}

#[derive(Clone, PartialEq, Eq, Debug, Serialize)]
pub struct MenLostLine {
    pub land_unit: Handle,
    /// `unfed` or `lost_with_ship`.
    pub cause: &'static str,
    pub men: u32,
    /// The ship lost with, for `lost_with_ship`.
    pub ship: Option<Handle>,
    /// The hex of the unfed loss.
    pub place: Option<Place>,
    /// What the unit has left ashore there, for `unfed`.
    pub ashore: Option<u32>,
    pub event: u32,
}

#[derive(Clone, PartialEq, Eq, Debug, Serialize)]
pub struct UnitDestroyedLine {
    pub land_unit: Handle,
    pub event: u32,
}

/// What the base in a hex with men ashore holds, against what the aircraft
/// and the men at it cost it a day.
#[derive(Clone, PartialEq, Eq, Debug, Serialize)]
pub struct HexSupply {
    pub place: Place,
    /// The side's men ashore in the hex, all units together.
    pub men_ashore: u32,
    /// The base the men are fed from, the side's lowest-numbered in the hex;
    /// `None` where the side has no base there.
    pub base: Option<Handle>,
    /// Aircraft at that base, ready or damaged.
    pub aircraft: u32,
    /// The base's supply stock.
    pub supply_t: Option<Dec>,
    /// What the aircraft and the men cost a day: aircraft times the daily
    /// charge for an aircraft, plus men times the daily charge for a man.
    /// A charge whose figures are not loaded is 0.
    pub daily_charge_t: Dec,
    /// Stock over the daily charge, truncated to one decimal; `None` without
    /// a base or with no charge.
    pub days_of_supply: Option<Dec>,
}

#[derive(Clone, PartialEq, Eq, Debug, Serialize)]
pub struct ArrivalsPart {
    /// Arrivals due today and on the next two days.
    pub due: Vec<DueArrival>,
    pub replacements: Vec<ReplacementStream>,
}

#[derive(Clone, PartialEq, Eq, Debug, Serialize)]
pub struct DueArrival {
    pub day: u32,
    pub air_group: Handle,
    pub aircraft: u32,
    pub pilots: u32,
}

#[derive(Clone, PartialEq, Eq, Debug, Serialize)]
pub struct ReplacementStream {
    pub base: Handle,
    pub aircraft_type: String,
    pub aircraft_per_day: u32,
    pub pilots_per_day: u32,
}

/// The five warnings of decision 6. Each is a fact about the side's own
/// state with the figure that makes it one.
#[derive(Clone, PartialEq, Eq, Debug, Serialize)]
pub struct Warnings {
    pub ship_cannot_reach_base: Vec<ShipRangeWarning>,
    pub base_supply_low: Vec<SupplyWarning>,
    pub base_without_fuel_with_force: Vec<FuelWarning>,
    pub air_group_short_of_pilots: Vec<PilotWarning>,
    pub air_group_tired: Vec<TiredWarning>,
    /// A sixth, from the review of scenario 9 play: task forces that cannot
    /// move or are slowed by a ship of theirs with damaged propulsion.
    pub task_force_propulsion: Vec<PropulsionWarning>,
    /// A seventh, from the second round of scenario 9 play: an air group on
    /// naval_strike that holds contacts and has none within its type's
    /// strike radius, or on base_strike at a base beyond it, at the time of
    /// the briefing. It would launch nothing.
    pub strike_group_out_of_reach: Vec<OutOfReachWarning>,
    /// An eighth, from decision 0063: an air group on naval_strike that holds
    /// contacts within its reach, none of a kind on its target list, so that
    /// it holds its strike. Left out when there is none, so the briefing of a
    /// game with no target order is as it was.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub strike_group_holding: Vec<HoldingWarning>,
}

#[derive(Clone, PartialEq, Eq, Debug, Serialize)]
pub struct HoldingWarning {
    pub air_group: Handle,
    /// The group's target list, in order.
    pub target_priority: Vec<&'static str>,
    /// The aircraft type's strike radius in miles.
    pub radius_nm: Dec,
    /// The reach in force in miles, given only for a group on extended
    /// reach.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reach_nm: Option<Dec>,
    /// The nearest contact held within reach.
    pub nearest: Handle,
    /// The kind the contact is taken for.
    pub nearest_taken_for: &'static str,
    /// Its air distance from the group in whole miles.
    pub distance_nm: u32,
}

#[derive(Clone, PartialEq, Eq, Debug, Serialize)]
pub struct OutOfReachWarning {
    pub air_group: Handle,
    /// `naval_strike` or `base_strike`.
    pub mission: &'static str,
    /// The aircraft type's strike radius in miles.
    pub radius_nm: Dec,
    /// The reach in force in miles, given only for a group on extended
    /// reach (decision 0059): the test the warning makes is against it.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reach_nm: Option<Dec>,
    /// For naval_strike, the nearest contact held; for base_strike, the
    /// target base.
    pub nearest: Handle,
    /// Its air distance from the group in whole miles, by the engine's own
    /// distance (`measure` air_reach gives the same figure).
    pub distance_nm: u32,
}

#[derive(Clone, PartialEq, Eq, Debug, Serialize)]
pub struct PropulsionWarning {
    pub task_force: Handle,
    /// `cannot_move` if a ship has propulsion out (the force has no speed),
    /// `slowed` if the worst is degraded.
    pub effect: &'static str,
    pub ships: Vec<PropulsionLine>,
}

#[derive(Clone, PartialEq, Eq, Debug, Serialize)]
pub struct ShipRangeWarning {
    pub ship: Handle,
    /// The nearest base of the side by sea; `None` if the ship can reach none.
    pub nearest_base: Option<Handle>,
    /// Sea distance to it in miles, 40 to a hex.
    pub distance_nm: Option<u32>,
    /// Whole miles the ship's fuel gives at cruising speed.
    pub range_nm: u32,
}

#[derive(Clone, PartialEq, Eq, Debug, Serialize)]
pub struct SupplyWarning {
    pub base: Handle,
    pub supply_t: Dec,
    pub spent_yesterday_t: Dec,
    pub days_of_supply: Option<Dec>,
    /// `no_supply` or `under_three_days`.
    pub reason: &'static str,
}

#[derive(Clone, PartialEq, Eq, Debug, Serialize)]
pub struct FuelWarning {
    pub base: Handle,
    pub fuel_t: Dec,
    /// The own task forces stopped at the base.
    pub task_forces: Vec<Handle>,
}

#[derive(Clone, PartialEq, Eq, Debug, Serialize)]
pub struct PilotWarning {
    pub air_group: Handle,
    pub pilots: u32,
    pub ready: u32,
}

#[derive(Clone, PartialEq, Eq, Debug, Serialize)]
pub struct TiredWarning {
    pub air_group: Handle,
    pub fatigue: Dec,
    pub rest_above: Option<Dec>,
    pub stood_down: bool,
}

/// The briefing of a side at the start of a day. `scenario` is the name the
/// caller gives the game for the heading.
pub fn briefing(view: &SideView<'_>, scenario: &str) -> Briefing {
    briefing_for_game(view, scenario, None)
}

/// The briefing of a game that runs through day `last_day`. How long the
/// game runs is a fact of the scenario file, the same for both sides and
/// known to the one who started the game, not a fact of the world; a view
/// does not hold it, so whoever holds the game hands it in (the session does).
/// `None` if the caller does not know it, and the heading then leaves it out.
pub fn briefing_for_game(view: &SideView<'_>, scenario: &str, last_day: Option<u32>) -> Briefing {
    let all = view.events();
    let mut ctx = Ctx::new(*view);
    ctx.set_events(&all);
    let today = view.day() + 1;
    let yesterday = (view.day() >= 1).then_some(view.day());
    let events: Vec<SideEvent> = match yesterday {
        Some(day) => all.iter().filter(|e| e.day == day).cloned().collect(),
        None => Vec::new(),
    };
    let mut day = Day::read(&ctx, &events, &all);
    let played = view.rules_in_play();
    let rules = Rules {
        wear: played.wear,
        fatigue: played.fatigue,
        pilots: ctx.groups.values().any(|g| g.pilots.is_some()),
        // Bases spend supply on aircraft (decision 0039) and, where the
        // ground figures are loaded, on men ashore (decision 0066, 16).
        supply: played.supply || view.ground_figures().is_some(),
    };

    let task_forces: Vec<ForceLine> = ctx
        .forces
        .values()
        .filter(|f| is_listed(&ctx, f))
        .map(|f| force_line(&ctx, f))
        .collect();
    let away_at_yard: Vec<AwayForce> = ctx
        .forces
        .values()
        .filter_map(|f| {
            let trip = f.yard_trip?;
            Some(AwayForce {
                task_force: ctx.force(f.id),
                port: ctx.base(trip.base),
                ships: ctx
                    .ships
                    .values()
                    .filter(|s| s.location == ShipLocation::Yard(f.id))
                    .map(|s| ctx.ship(s.id))
                    .collect(),
                return_day: trip.return_day,
                returns_before_end: last_day.map(|last| trip.return_day <= last),
                last_day,
            })
        })
        .collect();
    let air_groups: Vec<AirGroupLine> = ctx
        .groups
        .values()
        .map(|g| air_group_line(&ctx, g))
        .collect();
    let bases: Vec<BriefingBase> = ctx
        .bases
        .values()
        .map(|b| {
            let spent = day.spent.get(&b.id).copied().unwrap_or(Fixed::ZERO);
            BriefingBase {
                base: ctx.base(b.id),
                place: ctx.place(b.hex),
                supply_t: Dec(b.supply_t),
                fuel_t: Dec(b.fuel_t),
                spent_yesterday_t: Dec(spent),
                destroyed_yesterday_t: Dec(day
                    .destroyed
                    .get(&b.id)
                    .copied()
                    .unwrap_or(Fixed::ZERO)),
                days_of_supply: days_of_supply(b.supply_t, spent).map(Dec),
                stocks_yesterday: stocks_yesterday(b, day.stocks.get(&b.id)),
                repaired_yesterday: day.repaired.get(&b.id).cloned().unwrap_or_default(),
                returned_yesterday: day.returned.get(&b.id).cloned().unwrap_or_default(),
            }
        })
        .collect();

    let land_units = land_units_part(&ctx, view, std::mem::take(&mut day.ground));

    let arrivals = ArrivalsPart {
        due: view
            .arrivals()
            .filter(|a| a.day >= today && a.day < today + 3)
            .map(|a| DueArrival {
                day: a.day,
                air_group: ctx.group(a.air_group),
                aircraft: a.aircraft,
                pilots: a.pilots,
            })
            .collect(),
        replacements: view
            .replacements()
            .map(|r| ReplacementStream {
                base: ctx.base(r.base),
                aircraft_type: view
                    .aircraft_type(r.aircraft_type)
                    .map_or_else(|| r.aircraft_type.to_string(), |t| t.name.clone()),
                aircraft_per_day: r.aircraft_per_day,
                pilots_per_day: r.pilots_per_day,
            })
            .collect(),
    };

    let warnings = warnings(&ctx, &day, rules);

    Briefing {
        format: FORMAT,
        version: VERSION,
        kind: "briefing",
        heading: Heading {
            side: words::side(view.side()),
            day: today,
            last_day,
            yesterday,
            scenario: scenario.to_string(),
            rules_in_play: rules,
        },
        objectives: objectives_part(&ctx, view),
        orders: day.orders,
        contacts: ContactsPart {
            held: view
                .contacts()
                .map(|c| ContactLine {
                    contact: ctx.entity(EntityRef::Contact(c.id)),
                    place: ctx.place(c.estimated_hex),
                    ship_count: c.estimated_ship_count,
                    taken_for: c.estimated_kind.name(),
                    age_pulses: c.age_pulses,
                })
                .collect(),
            ..day.contacts
        },
        action: day.action,
        losses: day.losses,
        task_forces,
        away_at_yard,
        air_groups,
        bases,
        land_units,
        arrivals,
        warnings,
    }
}

/// The land units part, or `None` when the side has no land unit (decision
/// 0066, 21).
fn land_units_part(ctx: &Ctx<'_>, view: &SideView<'_>, ground: GroundDay) -> Option<LandUnitsPart> {
    if ctx.land_units.is_empty() {
        return None;
    }
    let fed = fed_last_night(ctx);
    let units: Vec<LandUnitLine> = ctx
        .land_units
        .values()
        .map(|u| land_unit_line(ctx, u, &fed))
        .collect();
    // The hexes with men of the side ashore, in hex order.
    let mut men_by_hex: BTreeMap<Hex, u32> = BTreeMap::new();
    for unit in ctx.land_units.values() {
        if let Some(hex) = unit.hex {
            *men_by_hex.entry(hex).or_insert(0) += unit.men_ashore;
        }
    }
    let per_aircraft = view
        .air_supply_figures()
        .map_or(Fixed::ZERO, |f| f.daily_supply_t_per_aircraft);
    let per_man = view
        .ground_figures()
        .map_or(Fixed::ZERO, |f| f.daily_supply_t_per_man);
    let supply_by_hex = men_by_hex
        .into_iter()
        .map(|(hex, men)| {
            // The base that feeds the men: the side's lowest-numbered in the
            // hex (decision 0066, 16).
            let base = ctx.bases.values().find(|b| b.hex == hex);
            let aircraft: u32 = base.map_or(0, |b| {
                ctx.groups
                    .values()
                    .filter(|g| g.location == sim_core::state::AirGroupLocation::Base(b.id))
                    .map(|g| g.ready + g.damaged)
                    .sum()
            });
            let charge = per_aircraft * Fixed::from_int(i64::from(aircraft))
                + per_man * Fixed::from_int(i64::from(men));
            HexSupply {
                place: ctx.place(hex),
                men_ashore: men,
                base: base.map(|b| ctx.base(b.id)),
                aircraft,
                supply_t: base.map(|b| Dec(b.supply_t)),
                daily_charge_t: Dec(charge),
                days_of_supply: base
                    .and_then(|b| (!charge.is_zero()).then(|| b.supply_t / charge))
                    .map(|days| Dec(Fixed::from_raw(days.raw() / 100_000 * 100_000))),
            }
        })
        .collect();
    Some(LandUnitsPart {
        units,
        landed_yesterday: ground.landed,
        embarked_yesterday: ground.embarked,
        bases_formed_yesterday: ground.bases_formed,
        divided_yesterday: ground.divided,
        loading_complete_yesterday: ground.loading_complete,
        fed_last_night: ground.fed,
        lost_yesterday: ground.lost,
        destroyed_yesterday: ground.destroyed,
        supply_by_hex,
    })
}

impl Briefing {
    /// Adds the umpire's result, for the final briefing of a finished game
    /// (decision 8 of 0054). Whoever holds the game hands the result in; the
    /// view is used for the names of bases only.
    pub fn with_result(
        mut self,
        view: &SideView<'_>,
        result: &sim_core::objectives::GameResult,
    ) -> Briefing {
        self.objectives.result = Some(result_part(view, result));
        self
    }
}

/// Supply divided by spending, truncated to one decimal. `None` if nothing
/// was spent.
fn days_of_supply(supply: Fixed, spent: Fixed) -> Option<Fixed> {
    if spent.is_zero() {
        return None;
    }
    let days = supply / spent;
    Some(Fixed::from_raw(days.raw() / 100_000 * 100_000))
}

/// What the side's events of the day add up to.
struct Day {
    orders: OrdersPart,
    contacts: ContactsPart,
    action: ActionPart,
    losses: LossesPart,
    spent: BTreeMap<BaseId, Fixed>,
    destroyed: BTreeMap<BaseId, Fixed>,
    stocks: BTreeMap<BaseId, StockMoves>,
    repaired: BTreeMap<BaseId, Vec<RepairedShip>>,
    returned: BTreeMap<BaseId, Vec<ReturnedForce>>,
    ground: GroundDay,
}

/// What yesterday's events of the side say of its land units (decision
/// 0066, 23).
#[derive(Default)]
struct GroundDay {
    landed: Vec<MenMoved>,
    embarked: Vec<MenMoved>,
    bases_formed: Vec<BaseFormedLine>,
    divided: Vec<UnitDividedLine>,
    loading_complete: Vec<LoadingCompleteLine>,
    fed: Vec<FedLine>,
    lost: Vec<MenLostLine>,
    destroyed: Vec<UnitDestroyedLine>,
}

/// What moved in and out of a base's stocks yesterday, by cause, from the
/// side's own events.
#[derive(Clone, Copy, Default)]
struct StockMoves {
    supply_landed: Fixed,
    supply_loaded: Fixed,
    sorties_and_upkeep: Fixed,
    repairs: Fixed,
    men: Fixed,
    destroyed: Fixed,
    fuel_landed: Fixed,
    fuel_loaded: Fixed,
    fuel_refuelling: Fixed,
}

impl StockMoves {
    fn supply_net(&self) -> Fixed {
        self.supply_landed
            - self.supply_loaded
            - self.sorties_and_upkeep
            - self.repairs
            - self.men
            - self.destroyed
    }

    fn fuel_net(&self) -> Fixed {
        self.fuel_landed - self.fuel_loaded - self.fuel_refuelling
    }
}

/// How a base's supply stock moved yesterday. Every cause is a total of the
/// side's own events; `start_t` is `end_t` less the net of the causes, so the
/// line adds up by construction.
#[derive(Clone, PartialEq, Eq, Debug, Serialize)]
pub struct SupplyChange {
    pub start_t: Dec,
    pub end_t: Dec,
    /// Put ashore by ships.
    pub landed_t: Dec,
    /// Taken aboard ships as cargo.
    pub loaded_onto_ships_t: Dec,
    /// Paid for sorties flown and for aircraft kept at the base.
    pub sorties_and_upkeep_t: Dec,
    /// Paid for repairs.
    pub repairs_t: Dec,
    /// Destroyed by raids.
    pub destroyed_by_raids_t: Dec,
    /// Paid to feed men ashore (decision 0066, 16). Left out when nothing
    /// was paid for men.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub men_fed_t: Option<Dec>,
}

/// How a base's fuel stock moved yesterday.
#[derive(Clone, PartialEq, Eq, Debug, Serialize)]
pub struct FuelChange {
    pub start_t: Dec,
    pub end_t: Dec,
    /// Put ashore by ships.
    pub landed_t: Dec,
    /// Taken aboard ships as cargo.
    pub loaded_onto_ships_t: Dec,
    /// Taken by ships to fill their own tanks.
    pub refuelling_ships_t: Dec,
}

/// What changed in a base's stocks yesterday: a part is `None` when nothing
/// moved in or out of that stock.
#[derive(Clone, PartialEq, Eq, Debug, Serialize)]
pub struct StocksYesterday {
    pub supply: Option<SupplyChange>,
    pub fuel: Option<FuelChange>,
}

fn stocks_yesterday(
    base: &sim_core::state::Base,
    moves: Option<&StockMoves>,
) -> Option<StocksYesterday> {
    let moves = moves?;
    let supply_moved = [
        moves.supply_landed,
        moves.supply_loaded,
        moves.sorties_and_upkeep,
        moves.repairs,
        moves.men,
        moves.destroyed,
    ]
    .iter()
    .any(|t| !t.is_zero());
    let fuel_moved = [moves.fuel_landed, moves.fuel_loaded, moves.fuel_refuelling]
        .iter()
        .any(|t| !t.is_zero());
    if !supply_moved && !fuel_moved {
        return None;
    }
    Some(StocksYesterday {
        supply: supply_moved.then(|| SupplyChange {
            start_t: Dec(base.supply_t - moves.supply_net()),
            end_t: Dec(base.supply_t),
            landed_t: Dec(moves.supply_landed),
            loaded_onto_ships_t: Dec(moves.supply_loaded),
            sorties_and_upkeep_t: Dec(moves.sorties_and_upkeep),
            repairs_t: Dec(moves.repairs),
            destroyed_by_raids_t: Dec(moves.destroyed),
            men_fed_t: (!moves.men.is_zero()).then_some(Dec(moves.men)),
        }),
        fuel: fuel_moved.then(|| FuelChange {
            start_t: Dec(base.fuel_t - moves.fuel_net()),
            end_t: Dec(base.fuel_t),
            landed_t: Dec(moves.fuel_landed),
            loaded_onto_ships_t: Dec(moves.fuel_loaded),
            refuelling_ships_t: Dec(moves.fuel_refuelling),
        }),
    })
}

impl Day {
    fn read(ctx: &Ctx<'_>, events: &[SideEvent], all: &[SideEvent]) -> Day {
        let mut orders = OrdersPart {
            accepted: 0,
            refused: Vec::new(),
        };
        let mut new: BTreeMap<ContactId, ReportedContact> = BTreeMap::new();
        let mut refreshed: Vec<ReportedContact> = Vec::new();
        // What each contact's report said, before the report that follows it.
        let mut last: BTreeMap<ContactId, (u32, sim_core::refdata::ShipKind, sim_core::hex::Hex)> =
            BTreeMap::new();
        let mut previous: BTreeMap<
            sim_core::ids::EventId,
            (u32, sim_core::refdata::ShipKind, sim_core::hex::Hex),
        > = BTreeMap::new();
        for e in all {
            if let SideEventKind::Whole(EventKind::ReportReceived {
                contact,
                hex,
                ship_count,
                kind,
                ..
            }) = &e.kind
            {
                if let Some(before) = last.get(contact) {
                    previous.insert(e.id, *before);
                }
                last.insert(*contact, (*ship_count, *kind, *hex));
            }
        }
        let mut dropped = Vec::new();
        let mut not_there = Vec::new();
        let mut fights = Vec::new();
        let mut fired: BTreeMap<(ShipId, Weapon), (u32, Fixed)> = BTreeMap::new();
        let mut strikes = Vec::new();
        let mut raids = Vec::new();
        let mut air_fire = Vec::new();
        let mut guns = Vec::new();
        let mut ship_attacks: BTreeMap<(ShipId, Weapon), AttackOnShip> = BTreeMap::new();
        let mut air_attacks: BTreeMap<ShipId, AttackOnShip> = BTreeMap::new();
        let mut sunk = Vec::new();
        let mut sunk_ids: BTreeSet<ShipId> = BTreeSet::new();
        let mut damaged: BTreeMap<ShipId, DamagedShip> = BTreeMap::new();
        let mut aircraft: BTreeMap<AirGroupId, AircraftLoss> = BTreeMap::new();
        let mut claims = Vec::new();
        let mut landed: Vec<LandedLine> = Vec::new();
        let mut rebased: Vec<RebasedLine> = Vec::new();
        let mut spent: BTreeMap<BaseId, Fixed> = BTreeMap::new();
        let mut destroyed: BTreeMap<BaseId, Fixed> = BTreeMap::new();
        let mut stocks: BTreeMap<BaseId, StockMoves> = BTreeMap::new();
        let mut repaired: BTreeMap<BaseId, Vec<RepairedShip>> = BTreeMap::new();
        let mut returned: BTreeMap<BaseId, Vec<ReturnedForce>> = BTreeMap::new();
        let mut ground = GroundDay::default();

        for e in events {
            let id = ctx.number(e.id);
            match &e.kind {
                SideEventKind::Whole(kind) => match kind {
                    EventKind::OrderAccepted { .. } => orders.accepted += 1,
                    EventKind::OrderRejected {
                        order,
                        target,
                        reason,
                    } => orders.refused.push(RefusedOrder {
                        order: ctx.order_number(*order),
                        target: ctx.entity(*target),
                        reason: ctx.map_text(&words::refusal(reason)),
                        event: id,
                    }),
                    EventKind::ReportReceived {
                        contact,
                        hex,
                        ship_count,
                        kind,
                        created,
                        ..
                    } => {
                        let before = previous.get(&e.id);
                        let line = ReportedContact {
                            contact: ctx.entity(EntityRef::Contact(*contact)),
                            place: ctx.place(*hex),
                            ship_count: *ship_count,
                            taken_for: kind.name(),
                            event: id,
                            was_ship_count: before
                                .and_then(|b| (b.0 != *ship_count).then_some(b.0)),
                            was_taken_for: before.and_then(|b| (b.1 != *kind).then(|| b.1.name())),
                            was_place: before.and_then(|b| (b.2 != *hex).then(|| ctx.place(b.2))),
                        };
                        if *created {
                            new.entry(*contact).or_insert(line);
                        } else {
                            refreshed.push(line);
                        }
                    }
                    EventKind::ContactDropped {
                        contact,
                        age_pulses,
                        ..
                    } => dropped.push(DroppedContact {
                        contact: ctx.entity(EntityRef::Contact(*contact)),
                        age_pulses: *age_pulses,
                        event: id,
                    }),
                    EventKind::ContactNotThere { contact, hex, .. } => {
                        not_there.push(NotThere {
                            contact: ctx.entity(EntityRef::Contact(*contact)),
                            place: ctx.place(*hex),
                            event: id,
                        });
                    }
                    EventKind::StrikeLaunched {
                        air_group,
                        contact,
                        hex,
                        aircraft,
                        distance_nm,
                    } => strikes.push(Strike {
                        kind: "naval_strike_launched",
                        air_group: ctx.group(*air_group),
                        target: Some(ctx.entity(EntityRef::Contact(*contact))),
                        place: Some(ctx.place(*hex)),
                        aircraft: Some(Dec(Fixed::from_int(i64::from(*aircraft)))),
                        weapon: None,
                        distance_nm: Some(Dec(*distance_nm)),
                        event: id,
                        reports: Vec::new(),
                    }),
                    EventKind::BaseStrikeLaunched {
                        air_group,
                        base,
                        hex,
                        aircraft,
                        distance_nm,
                    } => strikes.push(Strike {
                        kind: "base_strike_launched",
                        air_group: ctx.group(*air_group),
                        target: Some(ctx.base(*base)),
                        place: Some(ctx.place(*hex)),
                        aircraft: Some(Dec(Fixed::from_int(i64::from(*aircraft)))),
                        weapon: None,
                        distance_nm: Some(Dec(*distance_nm)),
                        event: id,
                        reports: Vec::new(),
                    }),
                    EventKind::StrikeFoundNothing { air_group, hex } => strikes.push(Strike {
                        kind: "strike_found_nothing",
                        air_group: ctx.group(*air_group),
                        target: None,
                        place: Some(ctx.place(*hex)),
                        aircraft: None,
                        weapon: None,
                        distance_nm: None,
                        event: id,
                        reports: Vec::new(),
                    }),
                    EventKind::ShipSunk {
                        ship, task_force, ..
                    } => {
                        sunk_ids.insert(*ship);
                        sunk.push(SunkShip {
                            ship: ctx.ship(*ship),
                            task_force: ctx.force(*task_force),
                            event: id,
                        });
                    }
                    EventKind::ShipDamaged {
                        ship,
                        track,
                        hits,
                        value_after,
                        ..
                    } => {
                        let entry = damaged.entry(*ship).or_insert_with(|| DamagedShip {
                            ship: ctx.ship(*ship),
                            hits: 0,
                            flotation_after: None,
                            fire_after: None,
                            systems: Vec::new(),
                            events: Vec::new(),
                        });
                        entry.hits += *hits;
                        entry.events.push(id);
                        match words::track(*track) {
                            "flotation" => entry.flotation_after = Some(Dec(*value_after)),
                            _ => entry.fire_after = Some(Dec(*value_after)),
                        }
                    }
                    EventKind::SystemHit {
                        ship,
                        system,
                        before,
                        after,
                        ..
                    } => {
                        let entry = damaged.entry(*ship).or_insert_with(|| DamagedShip {
                            ship: ctx.ship(*ship),
                            hits: 0,
                            flotation_after: None,
                            fire_after: None,
                            systems: Vec::new(),
                            events: Vec::new(),
                        });
                        entry.events.push(id);
                        entry.systems.push(SystemChange {
                            system: words::system(*system),
                            before: words::system_state(*before),
                            after: words::system_state(*after),
                        });
                    }
                    EventKind::AircraftLost {
                        air_group,
                        aircraft: n,
                        ..
                    } => {
                        loss(ctx, &mut aircraft, *air_group, id).shot_down += n;
                    }
                    EventKind::AircraftDestroyedOnGround {
                        air_group,
                        aircraft: n,
                        ..
                    }
                    | EventKind::DamagedAircraftDestroyedOnGround {
                        air_group,
                        aircraft: n,
                        ..
                    } => {
                        loss(ctx, &mut aircraft, *air_group, id).destroyed_on_ground += n;
                    }
                    EventKind::AircraftWorn {
                        air_group,
                        written_off,
                        ..
                    } if *written_off > 0 => {
                        loss(ctx, &mut aircraft, *air_group, id).written_off += written_off;
                    }
                    EventKind::AircraftWrittenOff {
                        air_group,
                        aircraft: n,
                        ..
                    } if *n > 0 => {
                        loss(ctx, &mut aircraft, *air_group, id).written_off += n;
                    }
                    EventKind::AirGroupRebased {
                        air_group,
                        from,
                        to,
                        distance_nm,
                        transfer_range_nm,
                        ready,
                        pilots,
                    } => rebased.push(RebasedLine {
                        air_group: ctx.group(*air_group),
                        from: ctx.base(*from),
                        to: ctx.base(*to),
                        distance_nm: Dec(*distance_nm),
                        transfer_range_nm: Dec(*transfer_range_nm),
                        ready: *ready,
                        pilots: *pilots,
                        event: id,
                    }),
                    EventKind::AirGroupLost {
                        air_group,
                        aircraft: n,
                        ..
                    } => {
                        loss(ctx, &mut aircraft, *air_group, id).lost_with_ship += n;
                    }
                    EventKind::AircraftDitchedNoDeck {
                        air_group,
                        aircraft: n,
                        ..
                    } if *n > 0 => {
                        loss(ctx, &mut aircraft, *air_group, id).no_deck += n;
                    }
                    EventKind::AircraftLandedElsewhere {
                        from,
                        to,
                        at,
                        aircraft: n,
                        pilots,
                        formed,
                    } => landed.push(LandedLine {
                        from: ctx.group(*from),
                        to: ctx.group(*to),
                        at: match at {
                            sim_core::state::AirGroupLocation::Base(base) => ctx.base(*base),
                            sim_core::state::AirGroupLocation::Ship(ship) => ctx.ship(*ship),
                        },
                        aircraft: *n,
                        pilots: *pilots,
                        formed: *formed,
                        event: id,
                    }),
                    EventKind::AircraftDitched {
                        air_group, ditched, ..
                    } if *ditched > 0 => {
                        loss(ctx, &mut aircraft, *air_group, id).ditched += ditched;
                    }
                    EventKind::PilotsLost {
                        air_group, lost, ..
                    } => {
                        loss(ctx, &mut aircraft, *air_group, id).pilots_lost += lost;
                    }
                    EventKind::ClaimReceived {
                        hex,
                        claimed_sunk,
                        claimed_hits,
                        claimed_kind,
                        claimed_shot_down,
                        claimed_on_ground,
                        from_air,
                        ..
                    } => claims.push(Claim {
                        place: ctx.place(*hex),
                        claimed_sunk: *claimed_sunk,
                        claimed_hits: *claimed_hits,
                        taken_for: claimed_kind.map(sim_core::refdata::ShipKind::name),
                        claimed_shot_down: *claimed_shot_down,
                        claimed_on_ground: *claimed_on_ground,
                        from_air: *from_air,
                        event: id,
                    }),
                    EventKind::BaseSupplySpent {
                        base,
                        paid_t,
                        reason,
                        ..
                    } => {
                        let total = spent.entry(*base).or_insert(Fixed::ZERO);
                        *total += *paid_t;
                        let moves = stocks.entry(*base).or_default();
                        match reason {
                            SupplyUse::Sorties | SupplyUse::Upkeep => {
                                moves.sorties_and_upkeep += *paid_t;
                            }
                            SupplyUse::Repairs => moves.repairs += *paid_t,
                            // The men's charge is its own line, not the
                            // aircraft's (decision 0066, 16).
                            SupplyUse::Men => moves.men += *paid_t,
                        }
                    }
                    EventKind::BaseSupplyDestroyed {
                        base, destroyed_t, ..
                    } => {
                        let total = destroyed.entry(*base).or_insert(Fixed::ZERO);
                        *total += *destroyed_t;
                        stocks.entry(*base).or_default().destroyed += *destroyed_t;
                    }
                    EventKind::CargoUnloaded {
                        base,
                        cargo,
                        unloaded_t,
                        ..
                    } => {
                        let moves = stocks.entry(*base).or_default();
                        match cargo {
                            Cargo::Supply => moves.supply_landed += *unloaded_t,
                            Cargo::Fuel => moves.fuel_landed += *unloaded_t,
                        }
                    }
                    EventKind::CargoLoaded {
                        base,
                        cargo,
                        loaded_t,
                        ..
                    } => {
                        let moves = stocks.entry(*base).or_default();
                        match cargo {
                            Cargo::Supply => moves.supply_loaded += *loaded_t,
                            Cargo::Fuel => moves.fuel_loaded += *loaded_t,
                        }
                    }
                    EventKind::ShipRefuelled { base, taken_t, .. } => {
                        stocks.entry(*base).or_default().fuel_refuelling += *taken_t;
                    }
                    EventKind::ShipRepaired {
                        ship,
                        task_force,
                        base,
                        points,
                        fire_damage,
                        flotation_damage,
                        restored,
                        ..
                    } => {
                        // What she has left in systems is read from the ship
                        // as she is now: the briefing is taken before the
                        // day's orders, so it is what yesterday left.
                        let with_state =
                            |wanted: sim_core::state::SystemState| -> Vec<&'static str> {
                                ctx.ships
                                    .get(ship)
                                    .map(|s| {
                                        sim_core::state::ShipSystem::ALL
                                            .iter()
                                            .filter(|system| s.systems.get(**system) == wanted)
                                            .map(|system| words::system(*system))
                                            .collect()
                                    })
                                    .unwrap_or_default()
                            };
                        repaired.entry(*base).or_default().push(RepairedShip {
                            ship: ctx.ship(*ship),
                            task_force: ctx.force(*task_force),
                            points: Dec(*points),
                            fire_damage: Dec(*fire_damage),
                            flotation_damage: Dec(*flotation_damage),
                            restored: restored.iter().map(|s| words::system(*s)).collect(),
                            systems_degraded: with_state(sim_core::state::SystemState::Degraded),
                            systems_out: with_state(sim_core::state::SystemState::Out),
                            event: id,
                        });
                    }
                    EventKind::ForceReturnedFromYard {
                        task_force,
                        base,
                        ships,
                    } => returned.entry(*base).or_default().push(ReturnedForce {
                        task_force: ctx.force(*task_force),
                        ships: ships.iter().map(|s| ctx.ship(*s)).collect(),
                        event: id,
                    }),
                    EventKind::MenEmbarked {
                        unit,
                        ship,
                        task_force,
                        hex,
                        men,
                        aboard,
                        ashore,
                    } => ground.embarked.push(MenMoved {
                        land_unit: ctx.land_unit(*unit),
                        ship: ctx.ship(*ship),
                        task_force: ctx.force(*task_force),
                        place: ctx.place(*hex),
                        men: *men,
                        aboard: *aboard,
                        ashore: *ashore,
                        event: id,
                    }),
                    EventKind::MenLanded {
                        unit,
                        ship,
                        task_force,
                        hex,
                        men,
                        aboard,
                        ashore,
                    } => ground.landed.push(MenMoved {
                        land_unit: ctx.land_unit(*unit),
                        ship: ctx.ship(*ship),
                        task_force: ctx.force(*task_force),
                        place: ctx.place(*hex),
                        men: *men,
                        aboard: *aboard,
                        ashore: *ashore,
                        event: id,
                    }),
                    EventKind::BaseFormed {
                        base,
                        hex,
                        task_force,
                        ..
                    } => ground.bases_formed.push(BaseFormedLine {
                        base: ctx.base(*base),
                        place: ctx.place(*hex),
                        task_force: ctx.force(*task_force),
                        event: id,
                    }),
                    EventKind::UnitDivided {
                        from,
                        unit,
                        task_force,
                        men,
                    } => ground.divided.push(UnitDividedLine {
                        from: ctx.land_unit(*from),
                        land_unit: ctx.land_unit(*unit),
                        task_force: ctx.force(*task_force),
                        men: *men,
                        event: id,
                    }),
                    EventKind::UnitLoadingComplete {
                        task_force,
                        unit,
                        base,
                        cargo,
                        ..
                    } => ground.loading_complete.push(LoadingCompleteLine {
                        task_force: ctx.force(*task_force),
                        land_unit: ctx.land_unit(*unit),
                        base: base.map(|b| ctx.base(b)),
                        cargo: cargo.map(Cargo::name),
                        event: id,
                    }),
                    EventKind::MenFed {
                        hex,
                        base,
                        men,
                        fed,
                        unfed,
                        ..
                    } => ground.fed.push(FedLine {
                        place: ctx.place(*hex),
                        base: base.map(|b| ctx.base(b)),
                        men: *men,
                        fed: *fed,
                        unfed: *unfed,
                        event: id,
                    }),
                    EventKind::MenLostUnfed {
                        unit,
                        hex,
                        lost,
                        ashore,
                    } => ground.lost.push(MenLostLine {
                        land_unit: ctx.land_unit(*unit),
                        cause: "unfed",
                        men: *lost,
                        ship: None,
                        place: Some(ctx.place(*hex)),
                        ashore: Some(*ashore),
                        event: id,
                    }),
                    EventKind::MenLostWithShip { unit, ship, men } => {
                        ground.lost.push(MenLostLine {
                            land_unit: ctx.land_unit(*unit),
                            cause: "lost_with_ship",
                            men: *men,
                            ship: Some(ctx.ship(*ship)),
                            place: None,
                            ashore: None,
                            event: id,
                        });
                    }
                    EventKind::LandUnitDestroyed { unit } => {
                        ground.destroyed.push(UnitDestroyedLine {
                            land_unit: ctx.land_unit(*unit),
                            event: id,
                        });
                    }
                    _ => {}
                },
                SideEventKind::Salvo {
                    firer,
                    weapon,
                    shots,
                    ..
                } => {
                    let entry = fired.entry((*firer, *weapon)).or_insert((0, Fixed::ZERO));
                    entry.0 += 1;
                    entry.1 += *shots;
                }
                SideEventKind::SalvoAgainst { target, weapon, .. } => {
                    ship_attacks
                        .entry((*target, *weapon))
                        .or_insert_with(|| AttackOnShip {
                            by: "ship",
                            against: ctx.ship(*target),
                            weapon: Some(words::weapon(*weapon)),
                            attacks: 0,
                            aircraft: None,
                            events: Vec::new(),
                        })
                        .attacks += 1;
                }
                SideEventKind::AirAttackAgainst { ship, shots, .. } => {
                    let entry = air_attacks.entry(*ship).or_insert_with(|| AttackOnShip {
                        by: "aircraft",
                        against: ctx.ship(*ship),
                        weapon: None,
                        attacks: 0,
                        aircraft: Some(Dec(Fixed::ZERO)),
                        events: Vec::new(),
                    });
                    entry.attacks += 1;
                    entry.events.push(id);
                    entry.aircraft = Some(Dec(entry.aircraft.map_or(*shots, |d| d.0 + *shots)));
                }
                SideEventKind::AirAttack {
                    air_group,
                    weapon,
                    shots,
                } => strikes.push(Strike {
                    kind: "air_attack",
                    air_group: ctx.group(*air_group),
                    target: None,
                    place: None,
                    aircraft: Some(Dec(*shots)),
                    weapon: Some(words::weapon(*weapon)),
                    distance_nm: None,
                    event: id,
                    reports: Vec::new(),
                }),
                SideEventKind::BaseAttacked {
                    air_group,
                    base,
                    shots,
                } => strikes.push(Strike {
                    kind: "base_attack",
                    air_group: ctx.group(*air_group),
                    target: Some(ctx.base(*base)),
                    place: None,
                    aircraft: Some(Dec(*shots)),
                    weapon: None,
                    distance_nm: None,
                    event: id,
                    reports: Vec::new(),
                }),
                SideEventKind::BaseAttackedAgainst {
                    base,
                    shots,
                    hits,
                    hits_on_aircraft,
                    hits_on_supply,
                    ..
                } => raids.push(Raid {
                    base: ctx.base(*base),
                    aircraft: Dec(*shots),
                    hits: *hits,
                    hits_on_aircraft: *hits_on_aircraft,
                    hits_on_supply: *hits_on_supply,
                    event: id,
                }),
                SideEventKind::AirFight {
                    firer,
                    firing,
                    at,
                    shots,
                    ..
                } => air_fire.push(AirFire {
                    firing: firing.name(),
                    firer: ctx.group(*firer),
                    at: at.name(),
                    shots: Dec(*shots),
                    event: id,
                }),
                SideEventKind::AntiAircraftFire {
                    task_force, shots, ..
                } => guns.push(Guns {
                    by: ctx.force(*task_force),
                    shots: Dec(*shots),
                    event: id,
                }),
                SideEventKind::BaseAntiAircraftFire { base, shots, .. } => guns.push(Guns {
                    by: ctx.base(*base),
                    shots: Dec(*shots),
                    event: id,
                }),
                SideEventKind::EngagementEnded {
                    round,
                    task_force,
                    hex,
                    reason,
                } => fights.push(Fight {
                    force: ctx.force(*task_force),
                    place: ctx.place(*hex),
                    rounds: *round,
                    ended: words::side_end_reason(*reason),
                    event: id,
                }),
                // The rest of a fight reaches the briefing as its end, the
                // fire and the losses; an alarm as the force's readiness.
                SideEventKind::EngagementBegun { .. }
                | SideEventKind::SightingRoll { .. }
                | SideEventKind::ForceSurprised { .. }
                | SideEventKind::ForceSlippedAway { .. }
                | SideEventKind::AlarmRoll { .. } => {}
                // A search that sighted reaches the briefing as the contact
                // its report made.
                SideEventKind::SearchAttempt { .. } => {}
            }
        }

        for strike in &mut strikes {
            if strike.kind == "air_attack" || strike.kind == "strike_found_nothing" {
                let log = all[strike.event as usize - 1].id;
                strike.reports = reports_of(ctx, all, log);
            }
        }
        let damaged: Vec<DamagedShip> = damaged
            .into_iter()
            .filter(|(ship, _)| !sunk_ids.contains(ship))
            .map(|(_, d)| d)
            .collect();
        Day {
            orders,
            contacts: ContactsPart {
                new: new.into_values().collect(),
                refreshed,
                dropped,
                not_there,
                held: Vec::new(),
            },
            action: ActionPart {
                fights,
                fired: fired
                    .into_iter()
                    .map(|((firer, weapon), (salvos, shots))| Fired {
                        firer: ctx.ship(firer),
                        weapon: words::weapon(weapon),
                        salvos,
                        shots: Dec(shots),
                    })
                    .collect(),
                air_fire,
                guns,
                strikes,
                raids_on_bases: raids,
                attacks_on_ships: ship_attacks
                    .into_values()
                    .chain(air_attacks.into_values())
                    .collect(),
            },
            losses: LossesPart {
                ships_sunk: sunk,
                ships_damaged: damaged,
                aircraft: aircraft.into_values().collect(),
                aircraft_landed_elsewhere: landed,
                air_groups_rebased: rebased,
                claims,
            },
            spent,
            destroyed,
            stocks,
            repaired,
            returned,
            ground,
        }
    }
}

/// What the events that cite `strike` say the strike reported: the contact
/// made or refreshed, dropped, found not there, or the claim.
fn reports_of(
    ctx: &Ctx<'_>,
    all: &[SideEvent],
    strike: sim_core::ids::EventId,
) -> Vec<StrikeReport> {
    use sim_core::side_events::SideCause;
    let strike_number = SideCause::Event(ctx.number(strike));
    let mut out = Vec::new();
    for e in all.iter().filter(|e| e.causes.contains(&strike_number)) {
        let number = ctx.number(e.id);
        let report = match &e.kind {
            SideEventKind::Whole(EventKind::ReportReceived {
                contact,
                hex,
                ship_count,
                kind,
                created,
                ..
            }) => StrikeReport {
                result: if *created {
                    "contact_new"
                } else {
                    "contact_refreshed"
                },
                contact: Some(ctx.entity(EntityRef::Contact(*contact))),
                ship_count: Some(*ship_count),
                taken_for: Some(kind.name()),
                place: Some(ctx.place(*hex)),
                claimed_sunk: None,
                event: number,
            },
            SideEventKind::Whole(EventKind::ContactDropped { contact, .. }) => StrikeReport {
                result: "contact_dropped",
                contact: Some(ctx.entity(EntityRef::Contact(*contact))),
                ship_count: None,
                taken_for: None,
                place: None,
                claimed_sunk: None,
                event: number,
            },
            SideEventKind::Whole(EventKind::ContactNotThere { contact, hex, .. }) => StrikeReport {
                result: "contact_not_there",
                contact: Some(ctx.entity(EntityRef::Contact(*contact))),
                ship_count: None,
                taken_for: None,
                place: Some(ctx.place(*hex)),
                claimed_sunk: None,
                event: number,
            },
            SideEventKind::Whole(EventKind::ClaimReceived {
                hex,
                claimed_sunk,
                from_air: false,
                ..
            }) => StrikeReport {
                result: "claim",
                contact: None,
                ship_count: None,
                taken_for: None,
                place: Some(ctx.place(*hex)),
                claimed_sunk: Some(*claimed_sunk),
                event: number,
            },
            _ => continue,
        };
        out.push(report);
    }
    out
}

/// The entry of a group in the losses, made on first use, with the event
/// that told of the loss added.
fn loss<'m>(
    ctx: &Ctx<'_>,
    map: &'m mut BTreeMap<AirGroupId, AircraftLoss>,
    group: AirGroupId,
    event: u32,
) -> &'m mut AircraftLoss {
    let entry = map.entry(group).or_insert_with(|| AircraftLoss {
        air_group: ctx.group(group),
        shot_down: 0,
        destroyed_on_ground: 0,
        written_off: 0,
        lost_with_ship: 0,
        ditched: 0,
        no_deck: 0,
        pilots_lost: 0,
        events: Vec::new(),
    });
    entry.events.push(event);
    entry
}

fn warnings(ctx: &Ctx<'_>, day: &Day, rules: Rules) -> Warnings {
    // 1. A ship that cannot reach the nearest base of its side at cruising
    // speed, by the map's shortest sea path, charged as a force is charged
    // for steaming it (decision 0058).
    let mut ship_cannot_reach_base = Vec::new();
    if !ctx.bases.is_empty() {
        let fields: BTreeMap<BaseId, (Hex, BTreeMap<Hex, u32>)> = ctx
            .bases
            .values()
            .map(|b| (b.id, (b.hex, ctx.view.sea_distances_to(b.hex))))
            .collect();
        for ship in ctx.ships.values() {
            if ship.location == ShipLocation::Sunk {
                continue;
            }
            let (Some(hex), Some(range)) = (ctx.ship_hex(ship), cruise_range_nm(&ctx.view, ship))
            else {
                continue;
            };
            let nearest = fields
                .iter()
                .filter_map(|(base, (base_hex, field))| {
                    ctx.view
                        .sea_charged_nm_in(field, hex, *base_hex)
                        .map(|miles| (miles, *base))
                })
                .min();
            match nearest {
                Some((miles, base)) if miles <= range => {
                    let _ = base;
                }
                Some((miles, base)) => ship_cannot_reach_base.push(ShipRangeWarning {
                    ship: ctx.ship(ship.id),
                    nearest_base: Some(ctx.base(base)),
                    distance_nm: Some(miles),
                    range_nm: range,
                }),
                None => ship_cannot_reach_base.push(ShipRangeWarning {
                    ship: ctx.ship(ship.id),
                    nearest_base: None,
                    distance_nm: None,
                    range_nm: range,
                }),
            }
        }
    }

    // 2. A base whose supply lasts under three days at yesterday's spending,
    // or that has none.
    let mut base_supply_low = Vec::new();
    // 3. A base with no fuel and a force stopped at it.
    let mut base_without_fuel_with_force = Vec::new();
    for base in ctx.bases.values() {
        let spent = day.spent.get(&base.id).copied().unwrap_or(Fixed::ZERO);
        let reason = if !rules.supply {
            None
        } else if base.supply_t.is_zero() {
            Some("no_supply")
        } else if !spent.is_zero() && base.supply_t.raw() < spent.raw().saturating_mul(3) {
            Some("under_three_days")
        } else {
            None
        };
        if let Some(reason) = reason {
            base_supply_low.push(SupplyWarning {
                base: ctx.base(base.id),
                supply_t: Dec(base.supply_t),
                spent_yesterday_t: Dec(spent),
                days_of_supply: days_of_supply(base.supply_t, spent).map(Dec),
                reason,
            });
        }
        if base.fuel_t.is_zero() {
            let stopped: Vec<Handle> = ctx
                .forces
                .values()
                .filter(|f| {
                    f.position() == base.hex
                        && f.status != TaskForceStatus::Underway
                        && is_afloat(ctx, f)
                })
                .map(|f| ctx.force(f.id))
                .collect();
            if !stopped.is_empty() {
                base_without_fuel_with_force.push(FuelWarning {
                    base: ctx.base(base.id),
                    fuel_t: Dec(base.fuel_t),
                    task_forces: stopped,
                });
            }
        }
    }

    // 4 and 5. Air groups.
    let mut air_group_short_of_pilots = Vec::new();
    let mut air_group_tired = Vec::new();
    for group in ctx.groups.values() {
        if let Some(pilots) = group.pilots {
            if pilots < group.ready {
                air_group_short_of_pilots.push(PilotWarning {
                    air_group: ctx.group(group.id),
                    pilots,
                    ready: group.ready,
                });
            }
        }
        let above = group.rest_above.is_some_and(|t| group.fatigue > t);
        if rules.fatigue && (group.stood_down || above) {
            air_group_tired.push(TiredWarning {
                air_group: ctx.group(group.id),
                fatigue: Dec(group.fatigue),
                rest_above: group.rest_above.map(Dec),
                stood_down: group.stood_down,
            });
        }
    }

    // 6. A task force with a ship whose propulsion is not working.
    let task_force_propulsion: Vec<PropulsionWarning> = ctx
        .forces
        .values()
        .filter(|f| is_afloat(ctx, f))
        .map(|f| force_line(ctx, f))
        .filter(|line| line.movement != "normal")
        .map(|line| PropulsionWarning {
            task_force: line.task_force,
            effect: line.movement,
            ships: line.propulsion_damaged,
        })
        .collect();

    // 7. A strike group that could launch nothing for want of a target in
    // reach, by the radius test the strike decision and `measure` make.
    let mut strike_group_out_of_reach = Vec::new();
    let mut strike_group_holding = Vec::new();
    for group in ctx.groups.values() {
        let (Some(origin), Some(aircraft)) = (
            ctx.group_hex(group),
            ctx.view.aircraft_type(group.aircraft_type),
        ) else {
            continue;
        };
        let radius = aircraft.strike_range_nm;
        // The reach in force, by the one function the strike decision uses.
        let reach = ctx.view.reach_nm(group);
        let reach_nm = (group.reach == Reach::Extended).then_some(Dec(reach));
        match group.mission {
            AirMission::NavalStrike => {
                let nearest = ctx
                    .view
                    .contacts()
                    .map(|c| {
                        (
                            origin.air_distance_nm(c.estimated_hex),
                            c.id,
                            c.estimated_hex,
                        )
                    })
                    .min();
                // 8. Contacts in reach, none of a kind on the group's list.
                let in_reach = ctx
                    .view
                    .contacts()
                    .filter(|c| origin.air_distance_within(c.estimated_hex, reach))
                    .collect::<Vec<_>>();
                if !in_reach.is_empty()
                    && in_reach
                        .iter()
                        .all(|c| !group.target_priority.contains(&c.estimated_kind))
                {
                    if let Some(c) = in_reach
                        .iter()
                        .min_by_key(|c| (origin.air_distance_nm(c.estimated_hex), c.id))
                    {
                        strike_group_holding.push(HoldingWarning {
                            air_group: ctx.group(group.id),
                            target_priority: group
                                .target_priority
                                .iter()
                                .map(|kind| kind.name())
                                .collect(),
                            radius_nm: Dec(radius),
                            reach_nm,
                            nearest: ctx.entity(EntityRef::Contact(c.id)),
                            nearest_taken_for: c.estimated_kind.name(),
                            distance_nm: origin.air_distance_nm(c.estimated_hex),
                        });
                    }
                }
                if let Some((distance, id, hex)) = nearest {
                    if !origin.air_distance_within(hex, reach) {
                        strike_group_out_of_reach.push(OutOfReachWarning {
                            air_group: ctx.group(group.id),
                            mission: "naval_strike",
                            radius_nm: Dec(radius),
                            reach_nm,
                            nearest: ctx.entity(EntityRef::Contact(id)),
                            distance_nm: distance,
                        });
                    }
                }
            }
            AirMission::BaseStrike(target) => {
                if let Some(enemy) = ctx.enemy_bases.get(&target) {
                    if !origin.air_distance_within(enemy.hex, reach) {
                        strike_group_out_of_reach.push(OutOfReachWarning {
                            air_group: ctx.group(group.id),
                            mission: "base_strike",
                            radius_nm: Dec(radius),
                            reach_nm,
                            nearest: ctx.base(target),
                            distance_nm: origin.air_distance_nm(enemy.hex),
                        });
                    }
                }
            }
            _ => {}
        }
    }

    Warnings {
        ship_cannot_reach_base,
        base_supply_low,
        base_without_fuel_with_force,
        air_group_short_of_pilots,
        air_group_tired,
        task_force_propulsion,
        strike_group_out_of_reach,
        strike_group_holding,
    }
}
