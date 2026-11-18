//! Surface combat: opposing task forces in one hex fight in rounds.
//!
//! The rule is `docs/decisions/0014-surface-combat.md`; the random number
//! keys follow `docs/decisions/0013-random-keys-for-pairs.md`. Since
//! decision 0022 the fights are run from the movement step (`movement.rs`),
//! at the moment two opposing forces meet; the surface engagement phase of a
//! pulse is an empty slot. [`World::hold_meeting`] runs one meeting.
//!
//! A fight is a run of rounds. Each round is
//! worked in five steps, each its own function below, and everything in a
//! round is computed on the state at its start:
//!
//! - (a) [`World::sight`]: forces that have not sighted the enemy roll to.
//! - (b) [`World::avoid`]: a force that avoids slips away from an enemy that
//!   has not sighted it, and the fight ends at once.
//! - (c) [`World::fire`]: every ship of a force that has sighted fires.
//! - (d) [`World::apply_damage`] and [`World::become_alert`]: damage lands,
//!   ships sink, forces become alert.
//! - (e) [`World::check_ending`]: sunk forces, break-off, last round.
//!
//! After the fight each force that sighted the enemy reports it, and each
//! force that fired claims ships sunk. Both go into belief through
//! [`BeliefState::apply_report`](crate::belief::BeliefState::apply_report)
//! and nothing else.
//!
//! The fight writes truth only in ship damage, named system states (decision
//! 0016), ammunition, torpedoes, sinking (the ship's location and its task
//! force's set) and task force alertness.
//! The per-fight state lives in [`Fight`] and is gone when the fight ends.

use std::collections::{BTreeMap, BTreeSet};

use crate::belief::{ApplyOutcome, BattleClaim, Contact, Report};
use crate::choices::surface::{AlertChoice, SurfaceAnswer, SurfaceQuestion};
use crate::choices::{Answer, Question};
use crate::damage::{
    damage_per_hit, raw_u64, roll_shots, system_for_draw, system_hit_chance, DamageSite, HitSet,
};
use crate::events::{
    Breakdown, Cause, DamageTrack, EngagementEndReason, EventKind, Factor, ForceFatigueChange,
    Weapon,
};
use crate::fixed::Fixed;
use crate::hex::Hex;
use crate::ids::{EntityRef, EventId, OrderId, ShipId, TaskForceId};
use crate::orders::{Readiness, Stance};
use crate::pictures::Holders;
use crate::refdata::{experience_factor, ShipClass, ShipKind, SurfaceCombatFigures};
use crate::rng::{Draw, RngStream, Subsystem};
use crate::search::build_sighting_report;
use crate::state::{RulesOfEngagement, Ship, ShipLocation, ShipSystem, Side, TaskForce};
use crate::time::{self, Pulse};
use crate::world::World;

mod alert;
mod ending;
mod fire;
mod hits;
mod meeting;
mod reports;
mod sighting;
#[cfg(test)]
mod tests;

pub(crate) use alert::{alert_choice, force_ready};
pub(crate) use ending::break_off_test;
pub(crate) use fire::gun_shots;

/// One force's side of a fight.
struct ForceFight {
    id: TaskForceId,
    side: Side,
    /// Has sighted the enemy: by a roll, or by being fired on.
    sighted: bool,
    /// The round and event of the successful sighting roll, if it sighted
    /// by rolling.
    sight_roll: Option<(u32, EventId)>,
    fired_on_last_round: bool,
    /// The ships afloat when the fight began, in ID order.
    began_with: Vec<ShipId>,
    fired_this_fight: bool,
    /// Ready for action when the fight began, decided once and kept for the
    /// whole fight (decision 0068, 2): at battle readiness by order, or alert
    /// since before the pulse began. A force that is not ready fires its guns
    /// at `unready_gun_share` and its torpedoes at `unready_torpedo_share`
    /// (decision 0071).
    ready: bool,
    /// Ships of this force that sank in this fight.
    sunk_in_fight: BTreeSet<ShipId>,
    /// One `chance()` draw per round in which the force rolls.
    sighting_stream: RngStream,
    rules_order: Option<OrderId>,
}

/// Everything that lasts for one fight.
struct Fight {
    day: u32,
    pulse: Pulse,
    hex: Hex,
    dark: bool,
    figures: SurfaceCombatFigures,
    /// The `EngagementBegun` event, cause of most of what follows.
    begun: EventId,
    /// The two forces, the lower task force ID first.
    forces: [ForceFight; 2],
    /// Each ship's fire stream, opened when it first fires and kept for the
    /// whole fight.
    fire_streams: BTreeMap<ShipId, RngStream>,
    /// At least one salvo with shots above zero was fired in this fight
    /// (decision 5 of `docs/decisions/0021-readiness-and-alarm.md`: the
    /// alarm sounds only for a fight with fire).
    fired: bool,
    /// The forces that broke off or slipped away earlier in this pulse,
    /// copied in from the step and written back when the fight ends.
    avoiding: BTreeSet<TaskForceId>,
}

impl Fight {
    /// The index of the other force.
    const fn enemy(index: usize) -> usize {
        1 - index
    }

    /// What multiplies the gun shots of the force at `index`: the figure
    /// `unready_gun_share` if it was not ready when the fight began, else 1
    /// (decision 0068).
    fn gun_share(&self, index: usize) -> Fixed {
        if self.forces[index].ready {
            Fixed::ONE
        } else {
            self.figures.unready_gun_share
        }
    }

    /// What multiplies the torpedo shots of the force at `index`: the figure
    /// `unready_torpedo_share` if it was not ready when the fight began, else 1
    /// (decision 0071).
    fn torpedo_share(&self, index: usize) -> Fixed {
        if self.forces[index].ready {
            Fixed::ONE
        } else {
            self.figures.unready_torpedo_share
        }
    }
}

/// What one force did and had done to it in one round.
#[derive(Default)]
struct RoundForce {
    /// The event of its successful sighting roll this round.
    rolled: Option<EventId>,
    /// It sighted the enemy this round, by rolling or by having been fired on.
    newly_sighted: bool,
    surprised: bool,
    fired_on: bool,
    /// The first salvo fired at it this round.
    first_salvo_against: Option<EventId>,
}

/// One salvo, kept until damage is applied at the end of the round.
struct SalvoRecord {
    event: EventId,
    target: ShipId,
    target_force: usize,
    damage_per_hit: Fixed,
    /// Hits on flotation and fire, in [`DamageTrack::ALL`] order.
    track_hits: [u32; 2],
    /// The systems hit, in hit order. One entry per system hit.
    system_hits: Vec<ShipSystem>,
}

/// One round's working state.
struct Round {
    number: u32,
    forces: [RoundForce; 2],
    /// Salvos in the order they were fired.
    salvos: Vec<SalvoRecord>,
    /// `ShipDamaged` events this round, by the force of the ship hit.
    damaged: [Vec<EventId>; 2],
    /// `ShipSunk` events this round, in order.
    sunk: Vec<EventId>,
}

impl Round {
    fn new(number: u32) -> Round {
        Round {
            number,
            forces: [RoundForce::default(), RoundForce::default()],
            salvos: Vec::new(),
            damaged: [Vec::new(), Vec::new()],
            sunk: Vec::new(),
        }
    }
}

/// How and when a fight ended.
struct Ending {
    round: u32,
    reason: EngagementEndReason,
    causes: Vec<Cause>,
}

/// One weapon fired by one ship in one round, before it is rolled.
struct Shot {
    firer: ShipId,
    target: ShipId,
    target_force: usize,
    weapon: Weapon,
    shots: Fixed,
    base_chance: Fixed,
    /// The factors behind `shots`: guns or tubes, rate, minutes and the
    /// firer's main battery or torpedo mount factor.
    shot_factors: [Fixed; 4],
    /// The share of its guns or torpedoes a force not ready fires (decisions
    /// 0068 and 0071): `Some` only for a salvo of a force that was not ready
    /// when the fight began, with the share below 1. For a gun salvo it is
    /// multiplied into `shots` after the main battery factor and before the
    /// ammunition cap and named in the breakdown (`unready_gun_share`) after
    /// `main_battery_factor`; for a torpedo salvo it is multiplied in after the
    /// torpedo mount factor and named `unready_torpedo_share` after
    /// `torpedo_mount_factor`.
    unready_share: Option<Fixed>,
    /// The firer's fire control factor. One for torpedoes.
    fire_control_factor: Fixed,
    /// The firer's experience factor for this weapon and `Some` only in a
    /// dark fight with the weapon's strength above zero (decision 0064).
    /// Multiplied in last.
    experience_factor: Option<Fixed>,
    damage_per_hit: Fixed,
}
