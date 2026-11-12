//! Air strikes on ships, fighters and anti-aircraft fire, and the flight
//! deck limit shared with air search (decisions
//! `docs/decisions/0026-air-strikes.md` and
//! `docs/decisions/0027-fighters-and-anti-aircraft-fire.md`).
//!
//! A daylight pulse has two steps for a strike. In the `StrikeDecisions`
//! step ([`World::decide_strikes`]) each air group on the naval strike
//! mission decides for itself, on its side's belief and nothing else, and
//! launches or does not. In the `AirCombat` step ([`World::resolve_strikes`])
//! every launched strike flies to the hex it was sent to and attacks an
//! enemy task force that is really there, or finds nothing.
//!
//! Both sides' strikes are launched before either lands (decision 8): every
//! decision is made on the state at the start of the decision step, every
//! attack is worked out on the state at the start of the air combat step,
//! and only then is damage applied, in order of air group number. A group
//! aboard a ship that sinks in the pulse still attacks.
//!
//! # Raids, fighters and guns (0027)
//!
//! The strikes of one pulse from one place at one hex are one raid. The
//! fighters at a place on combat air patrol each split, on the state at the
//! start of the air combat step, into a patrol (the share of the aircraft
//! they can fly, rounded down, which stays over their hex) and escorts (the
//! rest). All the escorts at a place go with the raid from it that has the
//! most bombers, then the lowest first group.
//!
//! Every raid is worked out on the start-of-step counts, whatever the other
//! raids do. The patrol of a side over a hex is divided among the raids that
//! attack that side's forces there, in proportion to their aircraft
//! (decision 0033, 7 to 10; [`divide_patrol`]). Each raid fights its share:
//! escorts fire at the patrol, the share fires at the escorts and then at the
//! bombers, the ships of the force attacked fire at the bombers that are
//! left, and the bombers that are left attack. A raid that finds no task
//! force meets nobody and takes no share. The losses the raids' escorts do
//! to a patrol are added up and taken from its groups, the lowest first.
//!
//! The log of the step, in this order:
//!
//! 1. For each raid, in order of its first strike group: `EscortsSent` for
//!    each fighter group that sends escorts, then `AirFight` (escorts at
//!    the patrol, the raid's share of the patrol at the escorts, at the
//!    bombers), then
//!    `AntiAircraftFire`, then an `AircraftLost` for each group and each of
//!    those events that cost it aircraft, which are taken off `ready` as
//!    they are recorded.
//! 2. For each strike group in order of group number, `StrikeFoundNothing`
//!    or `AirAttack`. A group with no bombers left records neither.
//! 3. Then, in group order, each attack's damage and its report, as in 0026,
//!    and for each group that found nothing its report that nothing is
//!    there: if the side still holds the contact the strike was sent at and
//!    it still names the hex, `ContactNotThere`, caused by that group's
//!    `StrikeFoundNothing`, and the contact is gone (decision 0028). An
//!    earlier group's report in the same step may have moved the contact,
//!    and then nothing is dropped or recorded. No random number is drawn.
//!
//! The attack writes truth only through [`World::apply_hit_set`], the same
//! path a salvo's hits take, and belief only through
//! [`BeliefState::apply_report`](crate::belief::BeliefState::apply_report).
//!
//! # Raids on a base (0034)
//!
//! A group on the base strike mission ([`AirMission::BaseStrike`], which
//! carries its target base) launches in the morning pulse only, with no
//! contact and no belief read ([`World::decide_strikes`]), and its strike
//! joins the raid machinery above unchanged. Strikes at ships and strikes at
//! a base from one place at one hex are two raids, so each has its own escorts
//! decision, patrol share and guns; the patrol of the base's side over the
//! hex is divided among all the raids on that side there, ships and base
//! together. The base's guns (`Base::anti_aircraft`) fire at the bombers of a
//! raid on the base as a force's do, and ships in the hex do not. The bombers
//! left each roll their shots at the bomb hit chance, then one draw for each
//! hit says where it lands (aircraft on the ground, supply, or nothing). A
//! group's hits are applied in group order after the fights, on the ground
//! counts of the start of the step ([`World::aircraft_on_the_ground`]), and
//! the log of the step ends with, for each base strike group: `BaseAttacked`,
//! then `AircraftDestroyedOnGround` for each group hit and
//! `BaseSupplyDestroyed`. No report follows and no belief changes.
//!
//! # Wear (0035)
//!
//! When the wear figures are loaded the step also notes what each group flew
//! for the wear step that follows it ([`crate::upkeep`]): the patrol each
//! fighter group keeps (in every daylight pulse, raid or not), the escorts it
//! sends, and each strike, less what the step shot down. A damaged aircraft
//! is on the ground at a base whatever its group does, and is destroyed
//! before the ready aircraft of its group; the log of the step has, for a
//! group hit, `DamagedAircraftDestroyedOnGround` and then
//! `AircraftDestroyedOnGround`.
//!
//! # Fatigue (0036)
//!
//! What a group can fly in every step is [`World::aircraft_able_to_fly`]: its
//! ready aircraft times the flight deck's factor times 100 less its fatigue
//! over 100, rounded down once, and none if it is stood down. Search, strike
//! launch and the fighters' split into patrol and escorts all use it. The
//! aircraft on the ground at a raided base are, for each group located there
//! in order of group number, its damaged aircraft and then its ready aircraft
//! that are not flying in the pulse ([`World::aircraft_flying`]): a search
//! group's aircraft that search, the aircraft a strike launched, and a fighter
//! group's patrol and the escorts that go. The sorties noted for the wear
//! step are noted too when only the fatigue figures are loaded.
//!
//! # Random streams
//!
//! All in the air combat subsystem. A group's attack: (air group, task
//! force attacked), as in 0026. The escorts' fire: (the first escort group,
//! task force attacked). The patrol's fire: (the first patrol group over the
//! hex, the raid's first strike group), the roll at the escorts first, then the roll
//! at the bombers. The guns: (the lowest-numbered ship afloat in the force
//! attacked, the raid's first strike group), one roll for all the ships. In a
//! raid on a base the escorts' stream is keyed on (the first escort group, the
//! base), the guns on (the base, the raid's first strike group), and a group's
//! attack on the base on (air group, base): one draw for each bomb, then one
//! for each hit for where it lands.

use std::collections::BTreeMap;

use crate::belief::{ApplyOutcome, BattleClaim, Report, SightingReport};
use crate::damage::{
    damage_per_hit, raw_u64, roll_shots, system_for_draw, system_hit_chance, DamageSite, HitSet,
};
use crate::events::{
    AirFirer, AirTarget, Breakdown, Cause, DamageTrack, EventKind, Factor, SortieKind, Weapon,
};
use crate::fixed::Fixed;
use crate::hex::Hex;
use crate::ids::{AirGroupId, BaseId, ContactId, EntityRef, EventId, ShipId, TaskForceId};
use crate::orders::AirMission;
use crate::refdata::{AirCombatFigures, AttackWeapon, ShipKind};
use crate::rng::{RngStream, Subsystem};
use crate::search::build_sighting_report;
use crate::state::{AirGroup, AirGroupLocation, ShipSystem, Side};
use crate::time::{self, Pulse};
use crate::upkeep::Sortie;
use crate::world::World;

/// What a strike was launched at: the contact its side held (decision
/// 0026), or the base it was ordered to strike (decision 0034).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Aim {
    Contact(ContactId),
    Base(BaseId),
}

/// A strike launched in the decision step and not yet landed. It lives from
/// the decision step to the air combat step of one pulse, and is empty
/// between pulses, so it is not part of the state hash.
#[derive(Clone, Debug)]
pub(crate) struct StrikeInFlight {
    air_group: AirGroupId,
    side: Side,
    /// The place the group launched from: with the hex, what makes a raid.
    origin: AirGroupLocation,
    /// What the strike was launched at.
    aim: Aim,
    /// The hex the contact names, or the base's hex: where the strike flies
    /// to.
    hex: Hex,
    /// The aircraft launched.
    aircraft: u32,
    /// The `StrikeLaunched` event.
    launched: EventId,
    /// The kind the contact was taken for when the strike was launched
    /// (decision 6 of 0055). None for a strike at a base.
    #[allow(dead_code)] // read when the strike claims hits (stage 11b)
    taken_for: Option<ShipKind>,
}

/// One strike's attack, worked out and not yet applied.
struct Attack {
    air_group: AirGroupId,
    side: Side,
    task_force: TaskForceId,
    ship: ShipId,
    weapon: Weapon,
    launched: EventId,
    shots: Fixed,
    chance: Fixed,
    expected: Fixed,
    hits: u32,
    damage_per_hit: Fixed,
    /// Hits on flotation and fire, in [`DamageTrack::ALL`] order.
    track_hits: [u32; 2],
    system_hits: Vec<ShipSystem>,
    breakdown: Option<Breakdown>,
    /// The sighting report of the force attacked (decision 12).
    report: SightingReport,
}

/// What the strike found at its hex.
enum Outcome {
    Nothing {
        air_group: AirGroupId,
        side: Side,
        /// The contact the strike was sent at (decision 0028).
        contact: ContactId,
        hex: Hex,
        launched: EventId,
    },
    Attack(Box<Attack>),
    BaseAttack(Box<BaseAttack>),
}

/// One strike group's bombs on a base, worked out and not yet applied
/// (decisions 6 and 7 of 0034).
struct BaseAttack {
    air_group: AirGroupId,
    base: BaseId,
    launched: EventId,
    shots: Fixed,
    chance: Fixed,
    expected: Fixed,
    hits: u32,
    hits_on_aircraft: u32,
    hits_on_supply: u32,
    breakdown: Option<Breakdown>,
}

/// What a raid attacks: the most important enemy task force at its hex
/// (decision 7 of 0026), or a base (decision 0034).
#[derive(Clone, Copy)]
enum RaidTarget {
    Ships(ShipKind, TaskForceId, ShipId),
    Base(BaseId),
}

/// The aircraft of the groups on the ground at a base, by group in order of
/// group number: the pool hits on aircraft are taken from (decision 8 of
/// 0034).
type Ground = BTreeMap<BaseId, Vec<(AirGroupId, Stock, u32)>>;

/// Which of a group's aircraft a ground pool entry counts (decision 11 of
/// 0035).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Stock {
    Damaged,
    Ready,
}

/// A fighter group on combat air patrol, split into patrol and escorts on
/// the state at the start of the air combat step (decision 2 of 0027).
struct FighterGroup {
    air_group: AirGroupId,
    side: Side,
    origin: AirGroupLocation,
    /// Where the patrol is, if the group has a position.
    hex: Option<Hex>,
    /// Aircraft kept over the group's hex.
    patrol: u32,
    /// Aircraft that go with a raid.
    escorts: u32,
    /// The fighter type's strike range: how far its escorts go (decision
    /// 0030).
    radius_nm: Fixed,
    /// The orders that set the mission and the share.
    orders: Vec<Cause>,
}

/// The strikes of one pulse from one place at one hex (decision 4 of 0027).
struct Raid {
    /// In order of group number.
    strikes: Vec<StrikeInFlight>,
    /// The fighter groups, by index into the split, whose escorts go.
    escorts: Vec<usize>,
}

/// The patrol of one side over one hex, by group: the pool the escorts' hits
/// on it are taken from.
type PatrolPool = ((Side, Hex), Vec<(AirGroupId, u32)>);

/// What a raid meets of the patrol over its hex (decision 0033, 7).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
struct PatrolShare {
    /// The defending side's whole patrol over the hex.
    patrol: u32,
    /// The raid's aircraft, bombers and escorts, as sent.
    raid_aircraft: u32,
    /// The aircraft of all the raids on that side and hex.
    all_aircraft: u32,
    /// The raid's share of the patrol.
    share: u32,
}

/// Divides `patrol` fighters among raids, each given as its aircraft and
/// the id of its first strike group (decision 0033, 7). A raid's share is
/// the patrol times its aircraft over all the aircraft, rounded down, in
/// integers. Fighters left over go one each to the raids in order of
/// size, the largest first and, on a tie, the lower first group. Returns the
/// shares in the order the raids were given; the shares add up to the
/// patrol, and do not depend on that order.
pub(crate) fn divide_patrol(patrol: u32, raids: &[(u32, AirGroupId)]) -> Vec<u32> {
    let total: u64 = raids.iter().map(|(aircraft, _)| u64::from(*aircraft)).sum();
    if total == 0 {
        return vec![0; raids.len()];
    }
    let mut shares: Vec<u32> = raids
        .iter()
        .map(|(aircraft, _)| {
            u32::try_from(u64::from(patrol) * u64::from(*aircraft) / total)
                .expect("a share is no more than the patrol")
        })
        .collect();
    let given: u32 = shares.iter().sum();
    let mut order: Vec<usize> = (0..raids.len()).collect();
    order.sort_by_key(|index| (std::cmp::Reverse(raids[*index].0), raids[*index].1));
    for index in order.into_iter().take((patrol - given) as usize) {
        shares[index] += 1;
    }
    shares
}

/// Who fired a roll.
enum Fire {
    Fighters {
        firer: AirGroupId,
        target: AirGroupId,
        firing: AirFirer,
        at: AirTarget,
    },
    Guns {
        task_force: TaskForceId,
        air_group: AirGroupId,
    },
    /// The guns of a base (decision 5 of 0034).
    BaseGuns { base: BaseId, air_group: AirGroupId },
}

/// One roll of fighters or guns, worked out and not yet recorded.
struct FireRoll {
    fire: Fire,
    shots: Fixed,
    chance: Fixed,
    expected: Fixed,
    hits: u32,
    /// The aircraft lost, by group, in order of group number (decision 9).
    /// Empty for the escorts' fire at a patrol: those losses are added up
    /// over the raids and taken from the patrol of `patrol_of` when the
    /// rolls are recorded (decision 0033, 10).
    lost: Vec<(AirGroupId, u32)>,
    /// For the escorts' fire at a patrol: the defending side and hex.
    patrol_of: Option<(Side, Hex)>,
    breakdown: Option<Breakdown>,
}

/// Everything one raid does in the air combat step, worked out on the state
/// at its start.
struct RaidResult {
    /// Each fighter group that sends escorts: group, aircraft, its orders.
    escorts_sent: Vec<(AirGroupId, u32, Vec<Cause>)>,
    rolls: Vec<FireRoll>,
    /// By strike group; a group with no bombers left has none.
    outcomes: Vec<(AirGroupId, Outcome)>,
    /// What the raid found, if anything.
    target: Option<RaidTarget>,
}

/// Which false claim chance a missed shot is claimed at (decision 12 of
/// 0055).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum MissKind {
    Fighter,
    Gun,
}

/// What one side's firing in one raid adds up to, for its claim (decision 2
/// of 0055): the firing events of its own, the aircraft it really shot down
/// and the whole shots that missed.
#[derive(Default)]
struct SideTally {
    /// The unit of the first firing event, which keys the claim's draws.
    first: Option<EntityRef>,
    /// The firing events the claim rests on, in the order they were recorded.
    causes: Vec<EventId>,
    /// Aircraft really lost to this side's fighters and guns in the raid.
    real_shot_down: u32,
    /// Whole shots that missed, by firing event, in event order.
    air_misses: Vec<(u32, MissKind)>,
}

impl SideTally {
    fn fired(&mut self, unit: EntityRef, event: EventId) {
        self.first.get_or_insert(unit);
        self.causes.push(event);
    }
}

/// The claims one raid makes (decision 0055): what each side fired, as
/// recorded, kept until the raid has been resolved.
struct RaidTally {
    hex: Hex,
    /// The raid's first strike group, the second part of each claim's key.
    first_group: AirGroupId,
    attacker_side: Side,
    attacker: SideTally,
    defender_side: Side,
    defender: SideTally,
}

impl RaidTally {
    /// Adds a roll of fighters or guns, recorded as `event`, which really
    /// shot down `real` aircraft.
    fn note_roll(&mut self, roll: &FireRoll, event: EventId, real: u32) {
        let (by_attacker, kind, unit) = match &roll.fire {
            Fire::Fighters { firer, firing, .. } => (
                *firing == AirFirer::Escorts,
                MissKind::Fighter,
                EntityRef::AirGroup(*firer),
            ),
            Fire::Guns { task_force, .. } => {
                (false, MissKind::Gun, EntityRef::TaskForce(*task_force))
            }
            Fire::BaseGuns { base, .. } => (false, MissKind::Gun, EntityRef::Base(*base)),
        };
        let whole = u32::try_from(roll.shots.trunc()).expect("shots are not negative");
        let side = if by_attacker {
            &mut self.attacker
        } else {
            &mut self.defender
        };
        side.fired(unit, event);
        side.real_shot_down += real;
        side.air_misses
            .push((whole.saturating_sub(roll.hits), kind));
    }
}

/// The shots times chance of a roll: shots, expected hits, hits (decision 10
/// of 0027, decision 0033, 3). Each shot is one draw from `stream`
/// ([`roll_shots`]).
fn roll_hits(stream: &mut RngStream, shots: u32, chance: Fixed) -> (Fixed, Fixed, u32) {
    let shots = Fixed::from_int(i64::from(shots));
    let expected = shots * chance;
    let hits = roll_shots(stream, shots, chance);
    (shots, expected, hits)
}

/// The chance of one shot at aircraft: the base chance and the protection
/// factors of the target type that were applied (decision 11 of 0029).
#[derive(Clone, Copy)]
struct ShotChance {
    base: Fixed,
    armour: Option<Fixed>,
    tanks: Option<Fixed>,
    /// Base times armour factor times tanks factor, in that order, each
    /// step truncated.
    chance: Fixed,
}

impl ShotChance {
    fn new(base: Fixed, armour: Option<Fixed>, tanks: Option<Fixed>) -> ShotChance {
        let mut chance = base;
        if let Some(factor) = armour {
            chance = chance * factor;
        }
        if let Some(factor) = tanks {
            chance = chance * factor;
        }
        ShotChance {
            base,
            armour,
            tanks,
            chance,
        }
    }
}

/// Takes `hits` from the pool, the lowest group first, each until it has
/// none left (decision 9 of 0027). Hits beyond what is there are wasted.
/// Returns what was taken from each group.
fn take_hits(hits: u32, pool: &mut [(AirGroupId, u32)]) -> Vec<(AirGroupId, u32)> {
    let mut left = hits;
    let mut taken = Vec::new();
    for (group, count) in pool.iter_mut() {
        let now = left.min(*count);
        if now > 0 {
            *count -= now;
            left -= now;
            taken.push((*group, now));
        }
    }
    taken
}

/// Takes `hits` from a ground pool in its order (decision 11 of 0035): the
/// lowest group first, a group's damaged aircraft before its ready ones. Hits
/// beyond what is there are wasted. Returns what was taken, in pool order.
fn take_ground(hits: u32, pool: &mut [(AirGroupId, Stock, u32)]) -> Vec<(AirGroupId, Stock, u32)> {
    let mut left = hits;
    let mut taken = Vec::new();
    for (group, stock, count) in pool.iter_mut() {
        let now = left.min(*count);
        if now > 0 {
            *count -= now;
            left -= now;
            taken.push((*group, *stock, now));
        }
    }
    taken
}

impl World {
    /// The aircraft of `group` that can be flown this pulse (decision 14 of
    /// 0026 and decision 2 of 0036): its ready aircraft times the flight
    /// deck's factor (all of them from a base, or from a ship whose deck
    /// works; the degraded factor if it is degraded; none if it is out) times
    /// 100 less its fatigue over 100, multiplied in fixed point and rounded
    /// down once. A stood-down group flies none. Without the fatigue
    /// figures a group is never tired or stood down. A group whose pilots
    /// are tracked starts from the smaller of its ready aircraft and its
    /// pilots (decision 2 of 0038). A group at a base with no supply at the
    /// start of the pulse flies none (decision 7 of 0039).
    pub(crate) fn aircraft_able_to_fly(&self, group: &AirGroup) -> u32 {
        // A base with no supply at the start of the pulse flies nothing
        // (decision 7 of 0039). Groups aboard ships are never affected.
        if let AirGroupLocation::Base(base) = group.location {
            if self.grounded_bases.contains(&base) {
                return 0;
            }
        }
        let crewed = group
            .pilots
            .map_or(group.ready, |pilots| group.ready.min(pilots));
        let mut able = Fixed::from_int(i64::from(crewed));
        if let AirGroupLocation::Ship(ship) = group.location {
            if let Some(ship) = self.truth.ships.get(&ship) {
                // Without the figures no system can be damaged (World::new).
                if let Some(figures) = &self.ref_data.surface_combat {
                    let state = ship.systems.get(ShipSystem::FlightDeck);
                    able = able * figures.system_factor(ShipSystem::FlightDeck, state);
                }
            }
        }
        if self.ref_data.fatigue.is_some() {
            if group.stood_down {
                return 0;
            }
            let hundred = Fixed::from_int(100);
            able = able.mul_div(hundred - group.fatigue, hundred);
        }
        u32::try_from(able.trunc()).expect("a share of the aircraft is not negative")
    }

    /// The strike decision step of one daylight pulse (decisions 3 to 6 of
    /// 0026, and 3 of 0034 for a group on the base strike mission, which
    /// decides at [`World::decide_base_strike`]). Each group on the naval strike mission, in group order, that
    /// has aircraft, a position, light and a flight deck that lets it launch
    /// at least one aircraft, takes the contact its side holds that is no
    /// older than the figures allow and within the type's strike range: the
    /// most important reported kind first, then the nearest, then the lowest
    /// contact number. It launches every aircraft it can.
    ///
    /// The decision reads the group's own side's belief and the group's own
    /// state. It never reads the enemy.
    pub(crate) fn decide_strikes(&mut self, day: u32, pulse: Pulse) {
        debug_assert!(
            self.strikes_in_flight.is_empty(),
            "the last pulse's strikes landed"
        );
        let Some(figures) = self.ref_data.air_combat.clone() else {
            return;
        };
        let groups: Vec<AirGroup> = self
            .truth
            .air_groups
            .values()
            .filter(|group| {
                matches!(
                    group.mission,
                    AirMission::NavalStrike | AirMission::BaseStrike(_)
                ) && group.ready > 0
            })
            .cloned()
            .collect();
        for group in groups {
            let Some(origin) = self.truth.air_group_hex(&group) else {
                continue;
            };
            if time::is_dark(origin, pulse) {
                continue;
            }
            let aircraft_type = &self.ref_data.aircraft_types[&group.aircraft_type];
            if aircraft_type.attack_weapon == AttackWeapon::None {
                continue;
            }
            let strike_range_nm = aircraft_type.strike_range_nm;
            let aircraft = self.aircraft_able_to_fly(&group);
            if aircraft == 0 {
                continue;
            }

            if let AirMission::BaseStrike(base) = group.mission {
                self.decide_base_strike(
                    day,
                    pulse,
                    &group,
                    base,
                    origin,
                    strike_range_nm,
                    aircraft,
                );
                continue;
            }

            let chosen = self.beliefs[&group.side]
                .contacts()
                .filter(|contact| contact.age_pulses <= figures.strike_contact_max_age_pulses)
                .filter_map(|contact| {
                    let distance_nm =
                        Fixed::from_int(i64::from(origin.air_distance_nm(contact.estimated_hex)));
                    origin
                        .air_distance_within(contact.estimated_hex, strike_range_nm)
                        .then_some((
                            contact.estimated_kind,
                            distance_nm,
                            contact.id,
                            contact.estimated_hex,
                            contact.last_report_event,
                        ))
                })
                .min_by_key(|(kind, distance_nm, id, _, _)| (*kind, *distance_nm, *id));
            let Some((taken_for, distance_nm, contact, hex, report_event)) = chosen else {
                continue;
            };

            let causes: Vec<Cause> = group
                .mission_order
                .map(Cause::Order)
                .into_iter()
                .chain([Cause::Event(report_event)])
                .collect();
            let launched = self.events.record(
                day,
                Some(pulse),
                EventKind::StrikeLaunched {
                    air_group: group.id,
                    contact,
                    hex,
                    aircraft,
                    distance_nm,
                },
                causes,
                None,
            );
            self.strikes_in_flight.push(StrikeInFlight {
                air_group: group.id,
                side: group.side,
                origin: group.location,
                aim: Aim::Contact(contact),
                hex,
                aircraft,
                launched,
                taken_for: Some(taken_for),
            });
        }
    }

    /// The decision of one group on the base strike mission (decision 3 of
    /// 0034): it launches every aircraft it can fly in the morning pulse if
    /// the base exists, is held by the other side and is within the type's
    /// strike range. It reads no belief and needs no contact. The caller
    /// has checked the light, that the type carries a weapon and that the
    /// group can fly an aircraft.
    #[allow(clippy::too_many_arguments)]
    fn decide_base_strike(
        &mut self,
        day: u32,
        pulse: Pulse,
        group: &AirGroup,
        base: BaseId,
        origin: Hex,
        strike_range_nm: Fixed,
        aircraft: u32,
    ) {
        if pulse != Pulse::Morning {
            return;
        }
        let Some(target) = self.truth.bases.get(&base) else {
            return;
        };
        if target.side == group.side {
            return;
        }
        let hex = target.hex;
        let distance_nm = Fixed::from_int(i64::from(origin.air_distance_nm(hex)));
        if !origin.air_distance_within(hex, strike_range_nm) {
            return;
        }
        let causes: Vec<Cause> = group.mission_order.map(Cause::Order).into_iter().collect();
        let launched = self.events.record(
            day,
            Some(pulse),
            EventKind::BaseStrikeLaunched {
                air_group: group.id,
                base,
                hex,
                aircraft,
                distance_nm,
            },
            causes,
            None,
        );
        self.strikes_in_flight.push(StrikeInFlight {
            air_group: group.id,
            side: group.side,
            origin: group.location,
            aim: Aim::Base(base),
            hex,
            aircraft,
            launched,
            taken_for: None,
        });
    }

    /// The air combat step of one daylight pulse (decisions 7 to 12 and 16
    /// of 0026, and 0027): every strike launched in this pulse reaches its
    /// hex, in raids, and meets the fighters and guns there.
    ///
    /// First every raid is worked out on the state as it stands, the fights
    /// and the losses are recorded and the losses taken off, and the attack
    /// events are recorded. Then, in order of air group number, damage is
    /// applied and the sighting report is sent. The module comment gives the
    /// order of the log.
    pub(crate) fn resolve_strikes(&mut self, day: u32, pulse: Pulse) {
        let strikes = std::mem::take(&mut self.strikes_in_flight);
        if strikes.is_empty() {
            // A patrol flies whether or not a raid came (decision 7 of 0035).
            if self.notes_sorties() {
                let fighters = self.split_fighter_groups();
                self.note_patrols(&fighters, pulse);
            }
            return;
        }

        // Everything is worked out on the state at the start of the step.
        let mut raids = group_raids(&strikes);
        let mut fighters = self.split_fighter_groups();
        send_escorts(&mut raids, &mut fighters);
        let mut ground = {
            let flying = self.aircraft_flying(pulse, &strikes, &raids, &fighters);
            self.aircraft_on_the_ground(&strikes, &flying)
        };
        if self.notes_sorties() {
            // What flew, for the wear step: the patrols and the strikes now;
            // the escorts as they are recorded; the losses as they are taken.
            self.note_patrols(&fighters, pulse);
            self.note_strikes(&strikes);
        }
        let shares = self.patrol_shares(&raids, &fighters);
        let results: Vec<RaidResult> = raids
            .iter()
            .zip(&shares)
            .map(|(raid, share)| self.work_out_raid(day, pulse, raid, &fighters, *share))
            .collect();
        // The patrol of each side over each hex, as it stood at the start of
        // the step: the escorts' hits on it from all the raids come off it
        // in turn, the lowest group first.
        let mut patrol_pools: Vec<PatrolPool> = Vec::new();
        for group in fighters.iter().filter(|group| group.patrol > 0) {
            if let Some(hex) = group.hex {
                match patrol_pools
                    .iter_mut()
                    .find(|(key, _)| *key == (group.side, hex))
                {
                    Some((_, pool)) => pool.push((group.air_group, group.patrol)),
                    None => patrol_pools
                        .push(((group.side, hex), vec![(group.air_group, group.patrol)])),
                }
            }
        }

        // Record the fights, then the losses, of each raid in turn.
        let mut outcomes: Vec<(AirGroupId, Outcome)> = Vec::new();
        let mut losses_of: Vec<(AirGroupId, EventId)> = Vec::new();
        // What each raid that found a target adds up to for the claims, and
        // the raid each strike group belongs to (decision 0055).
        let mut tallies: Vec<RaidTally> = Vec::new();
        for (raid, result) in raids.iter().zip(results) {
            let first = &raid.strikes[0];
            let mut tally = result.target.map(|target| RaidTally {
                hex: first.hex,
                first_group: first.air_group,
                attacker_side: first.side,
                attacker: SideTally::default(),
                defender_side: self.defender_of(target),
                defender: SideTally::default(),
            });
            let mut escort_events = Vec::new();
            for (index, (group, aircraft, orders)) in result.escorts_sent.into_iter().enumerate() {
                let causes: Vec<Cause> = orders
                    .into_iter()
                    .chain([Cause::Event(first.launched)])
                    .collect();
                let escort_hex = fighters[raid.escorts[index]].hex;
                escort_events.push(self.events.record(
                    day,
                    Some(pulse),
                    EventKind::EscortsSent {
                        air_group: group,
                        with_air_group: first.air_group,
                        aircraft,
                    },
                    causes,
                    None,
                ));
                if self.notes_sorties() {
                    self.note_escorts(
                        group,
                        aircraft,
                        escort_hex,
                        first.hex,
                        *escort_events.last().expect("just recorded"),
                    );
                }
            }
            let mut recorded = Vec::new();
            for roll in &result.rolls {
                let mut causes = vec![Cause::Event(first.launched)];
                let kind = match &roll.fire {
                    Fire::Fighters {
                        firer,
                        target,
                        firing,
                        at,
                    } => {
                        // A fight with escorts in it is explained by them.
                        if *firing == AirFirer::Escorts || *at == AirTarget::Escorts {
                            causes.extend(escort_events.iter().copied().map(Cause::Event));
                        }
                        EventKind::AirFight {
                            firer: *firer,
                            target: *target,
                            firing: *firing,
                            at: *at,
                            shots: roll.shots,
                            chance: roll.chance,
                            expected: roll.expected,
                            hits: roll.hits,
                        }
                    }
                    Fire::Guns {
                        task_force,
                        air_group,
                    } => EventKind::AntiAircraftFire {
                        task_force: *task_force,
                        air_group: *air_group,
                        shots: roll.shots,
                        chance: roll.chance,
                        expected: roll.expected,
                        hits: roll.hits,
                    },
                    Fire::BaseGuns { base, air_group } => EventKind::BaseAntiAircraftFire {
                        base: *base,
                        air_group: *air_group,
                        shots: roll.shots,
                        chance: roll.chance,
                        expected: roll.expected,
                        hits: roll.hits,
                    },
                };
                recorded.push(self.events.record(
                    day,
                    Some(pulse),
                    kind,
                    causes,
                    roll.breakdown.clone(),
                ));
            }
            for (roll, cause) in result.rolls.iter().zip(recorded) {
                let taken;
                let lost_list = match roll.patrol_of {
                    Some(key) => {
                        let pool = patrol_pools
                            .iter_mut()
                            .find(|(pool_key, _)| *pool_key == key)
                            .map(|(_, pool)| pool)
                            .expect("a patrol that is fired at is in the pools");
                        taken = take_hits(roll.hits, pool);
                        &taken
                    }
                    None => &roll.lost,
                };
                let lost_from = match (&roll.fire, roll.patrol_of) {
                    (_, Some(_)) => SortieKind::Patrol,
                    (
                        Fire::Fighters {
                            at: AirTarget::Escorts,
                            ..
                        },
                        None,
                    ) => SortieKind::Escort,
                    _ => SortieKind::Strike,
                };
                let mut real = 0;
                for (group, lost) in lost_list {
                    if let Some((event, taken)) =
                        self.lose_aircraft(day, pulse, *group, *lost, lost_from, cause)
                    {
                        losses_of.push((*group, event));
                        self.note_loss(*group, lost_from, *lost);
                        real += taken;
                    }
                }
                if let Some(tally) = tally.as_mut() {
                    tally.note_roll(roll, cause, real);
                }
            }
            tallies.extend(tally);
            outcomes.extend(result.outcomes);
        }
        outcomes.sort_by_key(|(group, _)| *group);

        // Record what each strike found, in group order.
        let mut attack_events: Vec<EventId> = Vec::new();
        for (_, outcome) in &outcomes {
            match outcome {
                Outcome::Nothing {
                    air_group,
                    hex,
                    launched,
                    ..
                } => {
                    let found_nothing = self.events.record(
                        day,
                        Some(pulse),
                        EventKind::StrikeFoundNothing {
                            air_group: *air_group,
                            hex: *hex,
                        },
                        vec![Cause::Event(*launched)],
                        None,
                    );
                    attack_events.push(found_nothing);
                }
                Outcome::BaseAttack(attack) => {
                    let causes: Vec<Cause> = [Cause::Event(attack.launched)]
                        .into_iter()
                        .chain(
                            losses_of
                                .iter()
                                .filter(|(group, _)| *group == attack.air_group)
                                .map(|(_, event)| Cause::Event(*event)),
                        )
                        .collect();
                    let event = self.events.record(
                        day,
                        Some(pulse),
                        EventKind::BaseAttacked {
                            air_group: attack.air_group,
                            base: attack.base,
                            shots: attack.shots,
                            chance: attack.chance,
                            expected: attack.expected,
                            hits: attack.hits,
                            hits_on_aircraft: attack.hits_on_aircraft,
                            hits_on_supply: attack.hits_on_supply,
                        },
                        causes,
                        attack.breakdown.clone(),
                    );
                    attack_events.push(event);
                }
                Outcome::Attack(attack) => {
                    // The aircraft that attack are those the fights left.
                    let causes: Vec<Cause> = [Cause::Event(attack.launched)]
                        .into_iter()
                        .chain(
                            losses_of
                                .iter()
                                .filter(|(group, _)| *group == attack.air_group)
                                .map(|(_, event)| Cause::Event(*event)),
                        )
                        .collect();
                    let event = self.events.record(
                        day,
                        Some(pulse),
                        EventKind::AirAttack {
                            air_group: attack.air_group,
                            task_force: attack.task_force,
                            ship: attack.ship,
                            weapon: attack.weapon,
                            shots: attack.shots,
                            chance: attack.chance,
                            expected: attack.expected,
                            hits: attack.hits,
                        },
                        causes,
                        attack.breakdown.clone(),
                    );
                    attack_events.push(event);
                }
            }
        }

        // Damage lands, then the report goes out, in group order.
        let site = DamageSite {
            day,
            pulse,
            round: 0,
        };
        let search = self
            .ref_data
            .search
            .clone()
            .expect("World::new requires search figures with air groups");
        for ((_, outcome), event) in outcomes.into_iter().zip(attack_events) {
            let attack = match outcome {
                Outcome::Attack(attack) => attack,
                Outcome::BaseAttack(attack) => {
                    self.apply_base_hits(day, pulse, &attack, event, &mut ground);
                    continue;
                }
                Outcome::Nothing {
                    side, contact, hex, ..
                } => {
                    // Decision 0028: the side learns the hex was empty, and
                    // drops the contact if it still names that hex.
                    let dropped = self
                        .beliefs
                        .get_mut(&side)
                        .expect("both sides have a belief state")
                        .apply_report(&Report::NothingThere { contact, hex }, &search);
                    if dropped == ApplyOutcome::Dropped {
                        self.events.record(
                            day,
                            Some(pulse),
                            EventKind::ContactNotThere { side, contact, hex },
                            vec![Cause::Event(event)],
                            None,
                        );
                    }
                    continue;
                }
            };
            let attack_event = event;
            self.apply_hit_set(
                site,
                &HitSet {
                    cause: attack_event,
                    target: attack.ship,
                    target_force: attack.task_force,
                    damage_per_hit: attack.damage_per_hit,
                    track_hits: attack.track_hits,
                    system_hits: attack.system_hits.clone(),
                },
            );

            let mut report = attack.report.clone();
            report.received_event = self.events.next_id();
            let ApplyOutcome::Contact(outcome) = self
                .beliefs
                .get_mut(&attack.side)
                .expect("both sides have a belief state")
                .apply_report(&Report::Sighting(report.clone()), &search)
            else {
                unreachable!("a sighting report makes or refreshes a contact");
            };
            let received = self.events.record(
                day,
                Some(pulse),
                EventKind::ReportReceived {
                    side: attack.side,
                    contact: outcome.contact,
                    hex: report.estimated_hex,
                    ship_count: report.estimated_ship_count,
                    kind: report.estimated_kind,
                    created: outcome.created,
                },
                vec![Cause::Event(attack_event)],
                None,
            );
            assert_eq!(
                received, report.received_event,
                "the report cites the event that records it"
            );
        }

        // Each side that fired in a raid claims what it believes it did, once
        // the raid is resolved (decision 0055).
        self.record_raid_claims(day, pulse, tallies);
    }

    /// The claims of the raids of one pulse (decision 0055): for each raid
    /// that found a target, in order, the raiding side's claim and then the
    /// defending side's, each only if it fired. A claim is what really
    /// happened and the false claims drawn for the whole shots that missed,
    /// from a stream of its own keyed for intelligence on the day, the pulse,
    /// the side's first firing unit and the raid's first strike group. No draw
    /// of the fight itself is touched.
    fn record_raid_claims(&mut self, day: u32, pulse: Pulse, tallies: Vec<RaidTally>) {
        if tallies.is_empty() {
            return;
        }
        let air = self
            .ref_data
            .air_combat
            .clone()
            .expect("a strike needs the air combat figures");
        let search = self
            .ref_data
            .search
            .clone()
            .expect("World::new requires search figures with air groups");
        for tally in tallies {
            for (side, part) in [
                (tally.attacker_side, tally.attacker),
                (tally.defender_side, tally.defender),
            ] {
                let Some(first) = part.first else {
                    continue;
                };
                let mut stream = self.rng_pair(
                    day,
                    Some(pulse),
                    Subsystem::Intelligence,
                    first,
                    EntityRef::AirGroup(tally.first_group),
                );
                let (mut fighter_misses, mut gun_misses) = (0, 0);
                let mut false_down = 0;
                for (misses, kind) in &part.air_misses {
                    let chance = match kind {
                        MissKind::Fighter => {
                            fighter_misses += misses;
                            air.fighter_false_claim_chance
                        }
                        MissKind::Gun => {
                            gun_misses += misses;
                            air.anti_aircraft_false_claim_chance
                        }
                    };
                    false_down +=
                        roll_shots(&mut stream, Fixed::from_int(i64::from(*misses)), chance);
                }
                let claimed_shot_down = part.real_shot_down + false_down;
                let claim = BattleClaim {
                    hex: tally.hex,
                    day,
                    pulse,
                    claimed_sunk: 0,
                    claimed_hits: 0,
                    claimed_kind: None,
                    claimed_shot_down,
                    claimed_on_ground: 0,
                    from_air: true,
                    received_event: self.events.next_id(),
                };
                let outcome = self
                    .beliefs
                    .get_mut(&side)
                    .expect("both sides have a belief state")
                    .apply_report(&Report::BattleClaim(claim.clone()), &search);
                assert_eq!(outcome, ApplyOutcome::Claim, "a claim is stored as a claim");
                let breakdown = self.config.record_breakdowns.then(|| {
                    let count = |n: u32| Fixed::from_int(i64::from(n));
                    Breakdown {
                        factors: vec![
                            Factor {
                                name: "real_shot_down",
                                value: count(part.real_shot_down),
                            },
                            Factor {
                                name: "fighter_misses",
                                value: count(fighter_misses),
                            },
                            Factor {
                                name: "fighter_false_claim_chance",
                                value: air.fighter_false_claim_chance,
                            },
                            Factor {
                                name: "gun_misses",
                                value: count(gun_misses),
                            },
                            Factor {
                                name: "anti_aircraft_false_claim_chance",
                                value: air.anti_aircraft_false_claim_chance,
                            },
                            Factor {
                                name: "false_shot_down",
                                value: count(false_down),
                            },
                        ],
                        result: count(claimed_shot_down),
                    }
                });
                let received = self.events.record(
                    day,
                    Some(pulse),
                    EventKind::ClaimReceived {
                        side,
                        hex: tally.hex,
                        claimed_sunk: 0,
                        claimed_hits: 0,
                        claimed_kind: None,
                        claimed_shot_down,
                        claimed_on_ground: 0,
                        from_air: true,
                    },
                    part.causes.iter().copied().map(Cause::Event).collect(),
                    breakdown,
                );
                assert_eq!(
                    received, claim.received_event,
                    "the claim cites the event that records it"
                );
            }
        }
    }

    /// The aircraft each group flies in this pulse, on the state at the
    /// start of the air combat step, after the escorts are sent (decision 11
    /// of 0036): the aircraft a group on search sends, the aircraft a strike
    /// launched, and the patrol and the escorts that go of a fighter group.
    /// A group not in the map flies none.
    fn aircraft_flying(
        &self,
        pulse: Pulse,
        strikes: &[StrikeInFlight],
        raids: &[Raid],
        fighters: &[FighterGroup],
    ) -> BTreeMap<AirGroupId, u32> {
        let mut flying: BTreeMap<AirGroupId, u32> = BTreeMap::new();
        for group in self.truth.air_groups.values() {
            // The search step: a group with an arc and light sends all it
            // can fly (see `search_from_the_air`).
            if group.mission == AirMission::Search
                && group.ready > 0
                && group.search.is_some()
                && self
                    .truth
                    .air_group_hex(group)
                    .is_some_and(|hex| !time::is_dark(hex, pulse))
            {
                flying.insert(group.id, self.aircraft_able_to_fly(group));
            }
        }
        for strike in strikes {
            *flying.entry(strike.air_group).or_insert(0) += strike.aircraft;
        }
        for (index, group) in fighters.iter().enumerate() {
            let sent = raids.iter().any(|raid| raid.escorts.contains(&index));
            let aircraft = group.patrol + if sent { group.escorts } else { 0 };
            *flying.entry(group.air_group).or_insert(0) += aircraft;
        }
        flying
    }

    /// The aircraft on the ground at each base that a base strike was sent
    /// at, on the state at the start of the air combat step (decision 11 of
    /// 0036, which restates decision 8 of 0034): for each group located at
    /// the base, in order of group number, its damaged aircraft (whatever
    /// the group is doing, decision 11 of 0035) and then its ready aircraft
    /// that are not flying in this pulse, whatever the group's mission. So
    /// the aircraft a tired group leaves behind, the fighters that neither
    /// patrol nor escort, and a stood-down group are all on the ground.
    fn aircraft_on_the_ground(
        &self,
        strikes: &[StrikeInFlight],
        flying: &BTreeMap<AirGroupId, u32>,
    ) -> Ground {
        let mut ground = Ground::new();
        for strike in strikes {
            let Aim::Base(base) = strike.aim else {
                continue;
            };
            ground.entry(base).or_insert_with(|| {
                let mut pool = Vec::new();
                for group in self
                    .truth
                    .air_groups
                    .values()
                    .filter(|group| group.location == AirGroupLocation::Base(base))
                {
                    if group.damaged > 0 {
                        pool.push((group.id, Stock::Damaged, group.damaged));
                    }
                    let resting = group
                        .ready
                        .saturating_sub(flying.get(&group.id).copied().unwrap_or(0));
                    if resting > 0 {
                        pool.push((group.id, Stock::Ready, resting));
                    }
                }
                pool
            });
        }
        ground
    }

    /// Applies the hits of one strike group's bombs on a base (decisions 8
    /// and 9 of 0034): aircraft are destroyed on the ground, the lowest
    /// group first, no more than `ground` says were there at the start of
    /// the step, and supply is destroyed, no more than the base has. Records
    /// each, citing `attack_event`.
    fn apply_base_hits(
        &mut self,
        day: u32,
        pulse: Pulse,
        attack: &BaseAttack,
        attack_event: EventId,
        ground: &mut Ground,
    ) {
        let per_hit = self
            .ref_data
            .air_combat
            .as_ref()
            .expect("a strike needs the air combat figures")
            .base_supply_destroyed_per_hit_t;
        let pool = ground
            .get_mut(&attack.base)
            .expect("a base that is attacked has its ground counted");
        let on_ground: u32 = pool.iter().map(|(_, _, aircraft)| aircraft).sum();
        let breakdown = self.config.record_breakdowns.then(|| Breakdown {
            factors: vec![
                Factor {
                    name: "hits_on_aircraft",
                    value: Fixed::from_int(i64::from(attack.hits_on_aircraft)),
                },
                Factor {
                    name: "aircraft_on_ground",
                    value: Fixed::from_int(i64::from(on_ground)),
                },
            ],
            result: Fixed::from_int(i64::from(attack.hits_on_aircraft.min(on_ground))),
        });
        // The lowest group first, its damaged aircraft before its ready ones
        // (decision 11 of 0035): one event for each kind of aircraft hit.
        for (group, stock, destroyed) in take_ground(attack.hits_on_aircraft, pool) {
            let air_group = self
                .truth
                .air_groups
                .get_mut(&group)
                .expect("the air group exists");
            let event = match stock {
                Stock::Ready => {
                    let destroyed = destroyed.min(air_group.ready);
                    air_group.ready -= destroyed;
                    EventKind::AircraftDestroyedOnGround {
                        air_group: group,
                        base: attack.base,
                        aircraft: destroyed,
                        remaining: air_group.ready,
                    }
                }
                Stock::Damaged => {
                    let destroyed = destroyed.min(air_group.damaged);
                    air_group.damaged -= destroyed;
                    EventKind::DamagedAircraftDestroyedOnGround {
                        air_group: group,
                        base: attack.base,
                        aircraft: destroyed,
                        remaining: air_group.damaged,
                    }
                }
            };
            self.events.record(
                day,
                Some(pulse),
                event,
                vec![Cause::Event(attack_event)],
                breakdown.clone(),
            );
        }

        let target = self
            .truth
            .bases
            .get_mut(&attack.base)
            .expect("the base exists");
        let before = target.supply_t;
        let wanted = per_hit * Fixed::from_int(i64::from(attack.hits_on_supply));
        let destroyed = wanted.min(before);
        if destroyed > Fixed::ZERO {
            target.supply_t = before - destroyed;
            let after = target.supply_t;
            let breakdown = self.config.record_breakdowns.then(|| Breakdown {
                factors: vec![
                    Factor {
                        name: "hits_on_supply",
                        value: Fixed::from_int(i64::from(attack.hits_on_supply)),
                    },
                    Factor {
                        name: "tons_per_hit",
                        value: per_hit,
                    },
                    Factor {
                        name: "supply_before",
                        value: before,
                    },
                ],
                result: destroyed,
            });
            self.events.record(
                day,
                Some(pulse),
                EventKind::BaseSupplyDestroyed {
                    base: attack.base,
                    destroyed_t: destroyed,
                    supply_t: after,
                },
                vec![Cause::Event(attack_event)],
                breakdown,
            );
        }
    }

    /// Splits every fighter group on combat air patrol that has aircraft
    /// into patrol and escorts (decision 2 of 0027): of the aircraft it can
    /// fly, the share, rounded down, is the patrol and the rest escort.
    /// Reads only each group's own state.
    fn split_fighter_groups(&self) -> Vec<FighterGroup> {
        self.truth
            .air_groups
            .values()
            .filter(|group| group.mission == AirMission::CombatAirPatrol && group.ready > 0)
            .map(|group| {
                let flyable = self.aircraft_able_to_fly(group);
                let patrol = (Fixed::from_int(i64::from(flyable)) * group.patrol_share).trunc();
                let patrol =
                    u32::try_from(patrol).expect("a share of the aircraft is not negative");
                FighterGroup {
                    air_group: group.id,
                    side: group.side,
                    origin: group.location,
                    hex: self.truth.air_group_hex(group),
                    patrol,
                    escorts: flyable - patrol,
                    radius_nm: self.ref_data.aircraft_types[&group.aircraft_type].strike_range_nm,
                    orders: group
                        .mission_order
                        .into_iter()
                        .chain(group.share_order)
                        .map(Cause::Order)
                        .collect(),
                }
            })
            .collect()
    }

    /// Takes `aircraft` off the ready aircraft of `group`, but never more
    /// than it has (decision 12 of 0027), and records it, citing `cause`.
    /// Records nothing if it has none to lose. If the group's pilots are
    /// tracked, each aircraft lost takes its pilot with it (never below 0),
    /// recorded as `PilotsLost` right after, unless the aircraft were the
    /// group's patrol over its own hex, whose pilots are saved (decisions 3
    /// and 4 of 0038). `lost_from` says which.
    fn lose_aircraft(
        &mut self,
        day: u32,
        pulse: Pulse,
        group: AirGroupId,
        aircraft: u32,
        lost_from: SortieKind,
        cause: EventId,
    ) -> Option<(EventId, u32)> {
        let air_group = self
            .truth
            .air_groups
            .get_mut(&group)
            .expect("the air group exists");
        let lost = aircraft.min(air_group.ready);
        if lost == 0 {
            return None;
        }
        air_group.ready -= lost;
        let remaining = air_group.ready;
        let pilots_lost = match air_group.pilots {
            Some(pilots) if lost_from != SortieKind::Patrol => {
                let taken = lost.min(pilots);
                air_group.pilots = Some(pilots - taken);
                taken
            }
            _ => 0,
        };
        let pilots_left = air_group.pilots.unwrap_or(0);
        let event = self.events.record(
            day,
            Some(pulse),
            EventKind::AircraftLost {
                air_group: group,
                aircraft: lost,
                remaining,
            },
            vec![Cause::Event(cause)],
            None,
        );
        if pilots_lost > 0 {
            self.events.record(
                day,
                Some(pulse),
                EventKind::PilotsLost {
                    air_group: group,
                    lost: pilots_lost,
                    left: pilots_left,
                },
                vec![Cause::Event(event)],
                None,
            );
        }
        Some((event, lost))
    }

    /// The enemy task force a strike of `side` finds at `hex`: the one with
    /// the most important ship, then the lowest number (decision 7 of 0026),
    /// and that ship. Reads truth, which the umpire may.
    fn find_target(&self, side: Side, hex: Hex) -> Option<(ShipKind, TaskForceId, ShipId)> {
        self.truth
            .task_forces
            .values()
            .filter(|force| force.side != side && force.position() == hex)
            .filter_map(|force| {
                let ship = force
                    .ships
                    .iter()
                    .map(|ship| (self.kind_of(*ship), *ship))
                    .min()?;
                Some((ship.0, force.id, ship.1))
            })
            .min()
    }

    /// What the raid whose first strike is `first` attacks, on the truth at
    /// the start of the step: the enemy task force at its hex, or the base
    /// it was sent at. A base strike always finds its base: the launch
    /// checked it exists and is held by the other side, and nothing changes
    /// that between the decision and the air combat step.
    fn raid_target(&self, first: &StrikeInFlight) -> Option<RaidTarget> {
        match first.aim {
            Aim::Contact(_) => self
                .find_target(first.side, first.hex)
                .map(|(kind, task_force, ship)| RaidTarget::Ships(kind, task_force, ship)),
            Aim::Base(base) => self
                .truth
                .bases
                .get(&base)
                .filter(|target| target.side != first.side)
                .map(|_| RaidTarget::Base(base)),
        }
    }

    /// The side a raid's target belongs to.
    fn defender_of(&self, target: RaidTarget) -> Side {
        match target {
            RaidTarget::Ships(_, task_force, _) => self.truth.task_forces[&task_force].side,
            RaidTarget::Base(base) => self.truth.bases[&base].side,
        }
    }

    /// The chance of a shot at aircraft of the type `group` flies: `base`
    /// times the pilot armour factor if the type has pilot armour, times the
    /// self-sealing tanks factor if it has them (decision 11 of 0029). The
    /// caller passes the first group of the target pool.
    fn shot_chance(&self, base: Fixed, group: AirGroupId, air: &AirCombatFigures) -> ShotChance {
        let aircraft_type =
            &self.ref_data.aircraft_types[&self.truth.air_groups[&group].aircraft_type];
        ShotChance::new(
            base,
            aircraft_type
                .pilot_armour
                .then_some(air.pilot_armour_factor),
            aircraft_type
                .self_sealing_tanks
                .then_some(air.self_sealing_tanks_factor),
        )
    }

    /// What each raid meets of the patrol over its hex (decision 0033, 7):
    /// the raids that attack forces of one side in one hex divide that
    /// side's patrol there. Worked out on the start-of-step counts for all
    /// the raids before any is resolved. A raid that finds nothing takes no
    /// share.
    fn patrol_shares(&self, raids: &[Raid], fighters: &[FighterGroup]) -> Vec<PatrolShare> {
        let mut shares = vec![PatrolShare::default(); raids.len()];
        // (defending side, hex) -> the raids that attack there.
        let mut places: Vec<((Side, Hex), Vec<usize>)> = Vec::new();
        for (index, raid) in raids.iter().enumerate() {
            let first = &raid.strikes[0];
            let Some(target) = self.raid_target(first) else {
                continue;
            };
            let key = (self.defender_of(target), first.hex);
            match places.iter_mut().find(|(place, _)| *place == key) {
                Some((_, members)) => members.push(index),
                None => places.push((key, vec![index])),
            }
        }
        for ((side, hex), members) in places {
            let patrol: u32 = fighters
                .iter()
                .filter(|group| group.side == side && group.hex == Some(hex))
                .map(|group| group.patrol)
                .sum();
            let aircraft: Vec<(u32, AirGroupId)> = members
                .iter()
                .map(|index| {
                    let raid = &raids[*index];
                    let bombers: u32 = raid.strikes.iter().map(|strike| strike.aircraft).sum();
                    let escorts: u32 = raid
                        .escorts
                        .iter()
                        .map(|group| fighters[*group].escorts)
                        .sum();
                    (bombers + escorts, raid.strikes[0].air_group)
                })
                .collect();
            let all_aircraft: u32 = aircraft.iter().map(|(count, _)| count).sum();
            let divided = divide_patrol(patrol, &aircraft);
            for ((index, (raid_aircraft, _)), share) in members.iter().zip(&aircraft).zip(divided) {
                shares[*index] = PatrolShare {
                    patrol,
                    raid_aircraft: *raid_aircraft,
                    all_aircraft,
                    share,
                };
            }
        }
        shares
    }

    /// Works out one raid on the state at the start of the step: the
    /// escorts it has, the fights in the air, the guns, and then what each
    /// strike group that has bombers left does. Reads truth, which the
    /// umpire may; writes nothing.
    fn work_out_raid(
        &self,
        day: u32,
        pulse: Pulse,
        raid: &Raid,
        fighters: &[FighterGroup],
        meets: PatrolShare,
    ) -> RaidResult {
        let first = &raid.strikes[0];
        let escorts_sent: Vec<(AirGroupId, u32, Vec<Cause>)> = raid
            .escorts
            .iter()
            .map(|index| {
                let group = &fighters[*index];
                (group.air_group, group.escorts, group.orders.clone())
            })
            .collect();

        let Some(target) = self.raid_target(first) else {
            // A raid that finds nothing meets nobody (decision 13 of 0027).
            // Only a strike at a contact can find nothing.
            return RaidResult {
                target: None,
                escorts_sent,
                rolls: Vec::new(),
                outcomes: raid
                    .strikes
                    .iter()
                    .filter_map(|strike| {
                        let Aim::Contact(contact) = strike.aim else {
                            return None;
                        };
                        Some((
                            strike.air_group,
                            Outcome::Nothing {
                                air_group: strike.air_group,
                                side: strike.side,
                                contact,
                                hex: strike.hex,
                                launched: strike.launched,
                            },
                        ))
                    })
                    .collect(),
            };
        };
        let defending_side = self.defender_of(target);
        // What the fighters' streams are keyed on besides their group.
        let target_entity = match target {
            RaidTarget::Ships(_, task_force, _) => EntityRef::TaskForce(task_force),
            RaidTarget::Base(base) => EntityRef::Base(base),
        };
        let air = self
            .ref_data
            .air_combat
            .as_ref()
            .expect("a strike needs the air combat figures");

        // The pools the hits are taken from, each in group order.
        let mut bombers: Vec<(AirGroupId, u32)> = raid
            .strikes
            .iter()
            .map(|strike| (strike.air_group, strike.aircraft))
            .collect();
        let mut escorts: Vec<(AirGroupId, u32)> = raid
            .escorts
            .iter()
            .map(|index| (fighters[*index].air_group, fighters[*index].escorts))
            .filter(|(_, aircraft)| *aircraft > 0)
            .collect();
        let patrol: Vec<(AirGroupId, u32)> = fighters
            .iter()
            .filter(|group| {
                group.side == defending_side && group.hex == Some(first.hex) && group.patrol > 0
            })
            .map(|group| (group.air_group, group.patrol))
            .collect();
        let escort_count: u32 = escorts.iter().map(|(_, aircraft)| aircraft).sum();
        // The raid meets its share of the patrol and no more (decision 0033).
        let patrol_count = meets.share;
        let patrol: Vec<(AirGroupId, u32)> = if patrol_count > 0 { patrol } else { Vec::new() };

        let mut rolls = Vec::new();
        let breakdown = |shots: Fixed, chance: ShotChance, expected: Fixed| {
            self.config.record_breakdowns.then(|| {
                let mut factors = vec![
                    Factor {
                        name: "shots",
                        value: shots,
                    },
                    Factor {
                        name: "hit_chance",
                        value: chance.base,
                    },
                ];
                if let Some(value) = chance.armour {
                    factors.push(Factor {
                        name: "pilot_armour_factor",
                        value,
                    });
                }
                if let Some(value) = chance.tanks {
                    factors.push(Factor {
                        name: "self_sealing_tanks_factor",
                        value,
                    });
                }
                Breakdown {
                    factors,
                    result: expected,
                }
            })
        };

        // The factors of the share go on the patrol's own fire (decision
        // 0033, 7), before the shots and chance.
        let patrol_breakdown = |breakdown: Option<Breakdown>| {
            breakdown.map(|mut breakdown| {
                let mut factors = vec![
                    Factor {
                        name: "patrol",
                        value: Fixed::from_int(i64::from(meets.patrol)),
                    },
                    Factor {
                        name: "raid_aircraft",
                        value: Fixed::from_int(i64::from(meets.raid_aircraft)),
                    },
                    Factor {
                        name: "all_raids_aircraft",
                        value: Fixed::from_int(i64::from(meets.all_aircraft)),
                    },
                    Factor {
                        name: "patrol_share",
                        value: Fixed::from_int(i64::from(meets.share)),
                    },
                ];
                factors.append(&mut breakdown.factors);
                breakdown.factors = factors;
                breakdown
            })
        };

        if let (Some((first_escort, _)), Some((first_patrol, _))) =
            (escorts.first().copied(), patrol.first().copied())
        {
            // Escorts at the patrol, then the patrol at the escorts and the
            // bombers. Each fires on the start-of-step counts.
            let mut stream = self.rng_pair(
                day,
                Some(pulse),
                Subsystem::AirCombat,
                EntityRef::AirGroup(first_escort),
                target_entity,
            );
            let shot = self.shot_chance(air.fighter_hit_chance, first_patrol, air);
            let (shots, expected, hits) = roll_hits(&mut stream, escort_count, shot.chance);
            rolls.push(FireRoll {
                fire: Fire::Fighters {
                    firer: first_escort,
                    target: first_patrol,
                    firing: AirFirer::Escorts,
                    at: AirTarget::Patrol,
                },
                shots,
                chance: shot.chance,
                expected,
                hits,
                lost: Vec::new(),
                patrol_of: Some((defending_side, first.hex)),
                breakdown: breakdown(shots, shot, expected),
            });
        }
        if let Some((first_patrol, _)) = patrol.first().copied() {
            let mut stream = self.rng_pair(
                day,
                Some(pulse),
                Subsystem::AirCombat,
                EntityRef::AirGroup(first_patrol),
                EntityRef::AirGroup(first.air_group),
            );
            let at_escorts = patrol_count.min(escort_count);
            if let Some((first_escort, _)) = escorts.first().copied() {
                let shot = self.shot_chance(air.fighter_hit_chance, first_escort, air);
                let (shots, expected, hits) = roll_hits(&mut stream, at_escorts, shot.chance);
                rolls.push(FireRoll {
                    fire: Fire::Fighters {
                        firer: first_patrol,
                        target: first_escort,
                        firing: AirFirer::Patrol,
                        at: AirTarget::Escorts,
                    },
                    shots,
                    chance: shot.chance,
                    expected,
                    hits,
                    lost: take_hits(hits, &mut escorts),
                    patrol_of: None,
                    breakdown: patrol_breakdown(breakdown(shots, shot, expected)),
                });
            }
            let at_bombers = patrol_count - at_escorts;
            if at_bombers > 0 {
                let shot = self.shot_chance(air.fighter_hit_chance, first.air_group, air);
                let (shots, expected, hits) = roll_hits(&mut stream, at_bombers, shot.chance);
                rolls.push(FireRoll {
                    fire: Fire::Fighters {
                        firer: first_patrol,
                        target: first.air_group,
                        firing: AirFirer::Patrol,
                        at: AirTarget::Bombers,
                    },
                    shots,
                    chance: shot.chance,
                    expected,
                    hits,
                    lost: take_hits(hits, &mut bombers),
                    patrol_of: None,
                    breakdown: patrol_breakdown(breakdown(shots, shot, expected)),
                });
            }
        }

        // The guns of the target, at the bombers left: of every ship afloat
        // in the force attacked, or of the base attacked (decision 5 of
        // 0034). Ships do not fire at a raid on the base, and the base does
        // not fire at a raid on ships.
        let bombers_left: u32 = bombers.iter().map(|(_, aircraft)| aircraft).sum();
        let guns = match target {
            RaidTarget::Ships(_, task_force, _) => {
                let force = &self.truth.task_forces[&task_force];
                let gun_shots: u32 = force
                    .ships
                    .iter()
                    .map(|ship| {
                        self.ref_data.ship_classes[&self.truth.ships[ship].class].anti_aircraft
                    })
                    .sum();
                force.ships.iter().next().map(|lowest_ship| {
                    (
                        gun_shots,
                        EntityRef::Ship(*lowest_ship),
                        Fire::Guns {
                            task_force,
                            air_group: first.air_group,
                        },
                    )
                })
            }
            RaidTarget::Base(base) => Some((
                self.truth.bases[&base].anti_aircraft,
                EntityRef::Base(base),
                Fire::BaseGuns {
                    base,
                    air_group: first.air_group,
                },
            )),
        };
        if let Some((gun_shots, firer, fire)) = guns.filter(|(gun_shots, _, _)| *gun_shots > 0) {
            if bombers_left > 0 {
                let mut stream = self.rng_pair(
                    day,
                    Some(pulse),
                    Subsystem::AirCombat,
                    firer,
                    EntityRef::AirGroup(first.air_group),
                );
                let shot = self.shot_chance(air.anti_aircraft_hit_chance, first.air_group, air);
                let (shots, expected, hits) = roll_hits(&mut stream, gun_shots, shot.chance);
                rolls.push(FireRoll {
                    fire,
                    shots,
                    chance: shot.chance,
                    expected,
                    hits,
                    lost: take_hits(hits, &mut bombers),
                    patrol_of: None,
                    breakdown: breakdown(shots, shot, expected),
                });
            }
        }

        // The bombers that are left attack, each group with what it has
        // left (decision 8 of 0027).
        let outcomes = bombers
            .iter()
            .zip(&raid.strikes)
            .filter(|((_, left), _)| *left > 0)
            .map(|((group, left), strike)| {
                let outcome = match target {
                    RaidTarget::Ships(kind, task_force, ship) => {
                        self.work_out_attack(day, pulse, strike, *left, (kind, task_force, ship))
                    }
                    RaidTarget::Base(base) => {
                        self.work_out_base_attack(day, pulse, strike, *left, base)
                    }
                };
                (*group, outcome)
            })
            .collect();
        RaidResult {
            target: Some(target),
            escorts_sent,
            rolls,
            outcomes,
        }
    }

    /// Rolls the attack of one strike group with `aircraft` bombers at the
    /// target its raid found. Reads truth, which the umpire may; writes
    /// nothing. Draws, from one stream keyed on the air group and the task
    /// force attacked: one draw for each shot, then for each hit
    /// its track (a bomb only: a torpedo always hits flotation) and then its
    /// system, then the sighting report.
    fn work_out_attack(
        &self,
        day: u32,
        pulse: Pulse,
        strike: &StrikeInFlight,
        aircraft: u32,
        target: (ShipKind, TaskForceId, ShipId),
    ) -> Outcome {
        let (kind, task_force, ship) = target;

        let air = self
            .ref_data
            .air_combat
            .as_ref()
            .expect("a strike needs the air combat figures");
        let surface = self
            .ref_data
            .surface_combat
            .as_ref()
            .expect("World::new requires surface combat figures");
        let search = self
            .ref_data
            .search
            .as_ref()
            .expect("World::new requires search figures with air groups");
        let group = &self.truth.air_groups[&strike.air_group];
        let aircraft_type = &self.ref_data.aircraft_types[&group.aircraft_type];
        let toughness = self.ref_data.ship_classes[&self.truth.ships[&ship].class].toughness;
        let (weapon, chance, system_table) = match aircraft_type.attack_weapon {
            AttackWeapon::Bomb => (Weapon::Bomb, air.bomb_hit_chance, &air.bomb_system_hit),
            AttackWeapon::Torpedo => (
                Weapon::Torpedo,
                air.torpedo_hit_chance,
                &surface.torpedo_system_hit,
            ),
            AttackWeapon::None => unreachable!("only groups that attack with something launch"),
        };
        let points = damage_per_hit(aircraft_type.attack_damage, toughness);
        // Decision 0033: the same per-point figure for every weapon.
        let system_chance = system_hit_chance(points, surface.system_hit_chance_per_point);

        let shots = Fixed::from_int(i64::from(aircraft));
        let expected = shots * chance;
        let mut stream = self.rng_pair(
            day,
            Some(pulse),
            Subsystem::AirCombat,
            EntityRef::AirGroup(strike.air_group),
            EntityRef::TaskForce(task_force),
        );
        let hits = roll_shots(&mut stream, shots, chance);

        let mut track_hits = [0u32; 2];
        let mut system_hits = Vec::new();
        for _ in 0..hits {
            let track = if weapon == Weapon::Bomb {
                let draw = stream.below(Fixed::SCALE.unsigned_abs());
                if draw < raw_u64(surface.gun_hit_flotation) {
                    DamageTrack::Flotation
                } else {
                    DamageTrack::Fire
                }
            } else {
                DamageTrack::Flotation
            };
            track_hits[track as usize] += 1;
            // One draw for the system, always, even if the chance is 0.
            let draw = stream.below(Fixed::SCALE.unsigned_abs());
            let draw = Fixed::from_raw(i64::try_from(draw).expect("a draw fits in i64"));
            system_hits.extend(system_for_draw(draw, system_chance, system_table));
        }

        let ship_count =
            u32::try_from(self.truth.task_forces[&task_force].ships.len()).expect("few ships");
        let report = build_sighting_report(
            &mut stream,
            &self.map,
            strike.hex,
            ship_count,
            kind,
            search,
            // Replaced with the event that records the report's arrival.
            EventId::new(0),
        );

        let breakdown = self.config.record_breakdowns.then(|| Breakdown {
            factors: vec![
                Factor {
                    name: "aircraft",
                    value: shots,
                },
                Factor {
                    name: "hit_chance",
                    value: chance,
                },
            ],
            result: expected,
        });
        Outcome::Attack(Box::new(Attack {
            air_group: strike.air_group,
            side: strike.side,
            task_force,
            ship,
            weapon,
            launched: strike.launched,
            shots,
            chance,
            expected,
            hits,
            damage_per_hit: points,
            track_hits,
            system_hits,
            breakdown,
            report,
        }))
    }

    /// Rolls the attack of one strike group with `aircraft` bombers on a
    /// base (decisions 6 and 7 of 0034). Reads truth, which the umpire may;
    /// writes nothing. Draws, from one stream keyed on the air group and the
    /// base: one draw for each bomb, at the bomb hit chance whatever the
    /// type's weapon, then one for each hit, below the chance of landing on
    /// aircraft, then below that and the chance of landing on supply, else on
    /// nothing.
    fn work_out_base_attack(
        &self,
        day: u32,
        pulse: Pulse,
        strike: &StrikeInFlight,
        aircraft: u32,
        base: BaseId,
    ) -> Outcome {
        let air = self
            .ref_data
            .air_combat
            .as_ref()
            .expect("a strike needs the air combat figures");
        let chance = air.bomb_hit_chance;
        let shots = Fixed::from_int(i64::from(aircraft));
        let expected = shots * chance;
        let mut stream = self.rng_pair(
            day,
            Some(pulse),
            Subsystem::AirCombat,
            EntityRef::AirGroup(strike.air_group),
            EntityRef::Base(base),
        );
        let hits = roll_shots(&mut stream, shots, chance);
        let on_aircraft_below = raw_u64(air.base_hit_aircraft);
        let on_supply_below = on_aircraft_below + raw_u64(air.base_hit_supply);
        let (mut hits_on_aircraft, mut hits_on_supply) = (0, 0);
        for _ in 0..hits {
            let draw = stream.below(Fixed::SCALE.unsigned_abs());
            if draw < on_aircraft_below {
                hits_on_aircraft += 1;
            } else if draw < on_supply_below {
                hits_on_supply += 1;
            }
        }
        let breakdown = self.config.record_breakdowns.then(|| Breakdown {
            factors: vec![
                Factor {
                    name: "aircraft",
                    value: shots,
                },
                Factor {
                    name: "hit_chance",
                    value: chance,
                },
                Factor {
                    name: "hit_on_aircraft_chance",
                    value: air.base_hit_aircraft,
                },
                Factor {
                    name: "hit_on_supply_chance",
                    value: air.base_hit_supply,
                },
            ],
            result: expected,
        });
        Outcome::BaseAttack(Box::new(BaseAttack {
            air_group: strike.air_group,
            base,
            launched: strike.launched,
            shots,
            chance,
            expected,
            hits,
            hits_on_aircraft,
            hits_on_supply,
            breakdown,
        }))
    }

    /// Notes for the wear step the patrol each fighter group keeps over its
    /// hex (decision 7 of 0035): the aircraft it kept, no distance, caused by
    /// its orders. A group with no position, or no patrol, notes nothing, and
    /// nothing flies where it is dark.
    fn note_patrols(&mut self, fighters: &[FighterGroup], pulse: Pulse) {
        for group in fighters {
            if group.patrol > 0 && group.hex.is_some_and(|hex| !time::is_dark(hex, pulse)) {
                self.sorties.push(Sortie {
                    air_group: group.air_group,
                    kind: SortieKind::Patrol,
                    aircraft: group.patrol,
                    sent: group.patrol,
                    distance_nm: Fixed::ZERO,
                    causes: group.orders.clone(),
                });
            }
        }
    }

    /// Notes for the wear step each strike launched: out to the hex it was
    /// sent to and back, caused by its launch.
    fn note_strikes(&mut self, strikes: &[StrikeInFlight]) {
        for strike in strikes {
            let Some(from) = self
                .truth
                .air_groups
                .get(&strike.air_group)
                .and_then(|group| self.truth.air_group_hex(group))
            else {
                continue;
            };
            let kind = match strike.aim {
                Aim::Contact(_) => SortieKind::Strike,
                Aim::Base(_) => SortieKind::BaseStrike,
            };
            self.sorties.push(Sortie {
                air_group: strike.air_group,
                kind,
                aircraft: strike.aircraft,
                sent: strike.aircraft,
                distance_nm: out_and_back(from, strike.hex),
                causes: vec![Cause::Event(strike.launched)],
            });
        }
    }

    /// Notes for the wear step the escorts a fighter group sent with a raid
    /// to `raid_hex`: out and back, caused by the event that says they were
    /// sent.
    fn note_escorts(
        &mut self,
        group: AirGroupId,
        aircraft: u32,
        from: Option<Hex>,
        raid_hex: Hex,
        sent: EventId,
    ) {
        let Some(from) = from else {
            return;
        };
        self.sorties.push(Sortie {
            air_group: group,
            kind: SortieKind::Escort,
            aircraft,
            sent: aircraft,
            distance_nm: out_and_back(from, raid_hex),
            causes: vec![Cause::Event(sent)],
        });
    }

    fn kind_of(&self, ship: ShipId) -> ShipKind {
        self.ref_data.ship_classes[&self.truth.ships[&ship].class].kind
    }
}

/// A sortie out to `to` and back from `from`: twice the distance (decision 4
/// of 0035).
fn out_and_back(from: Hex, to: Hex) -> Fixed {
    Fixed::from_int(i64::from(from.air_distance_nm(to)) * 2)
}

/// Groups the strikes into raids (decision 4 of 0027): the same place and
/// the same hex. Strikes are in group order, so the raids come out in order
/// of their first group.
fn group_raids(strikes: &[StrikeInFlight]) -> Vec<Raid> {
    let mut raids: Vec<Raid> = Vec::new();
    for strike in strikes {
        match raids.iter_mut().find(|raid| {
            raid.strikes[0].origin == strike.origin
                && raid.strikes[0].hex == strike.hex
                && same_kind_of_target(raid.strikes[0].aim, strike.aim)
        }) {
            Some(raid) => raid.strikes.push(strike.clone()),
            None => raids.push(Raid {
                strikes: vec![strike.clone()],
                escorts: Vec::new(),
            }),
        }
    }
    raids
}

/// Whether two strikes at one hex are at one kind of target: both at
/// contacts, or both at the same base (decision 4 of 0034). A strike at ships
/// and a strike at a base from one place at one hex are two raids.
fn same_kind_of_target(a: Aim, b: Aim) -> bool {
    match (a, b) {
        (Aim::Contact(_), Aim::Contact(_)) => true,
        (Aim::Base(a), Aim::Base(b)) => a == b,
        _ => false,
    }
}

/// Sends the escorts at each place with the raid from it that has the most
/// bombers, then the lowest first group (decision 5 of 0027), if that raid's
/// target hex is within the fighter type's radius of the group (decision 0030).
/// A place with no raid sends none. A group whose escorts do not go keeps
/// all it can fly as patrol (decision 2 of 0030). A group with no position
/// cannot be measured and sends its escorts as before.
fn send_escorts(raids: &mut [Raid], fighters: &mut [FighterGroup]) {
    for (index, group) in fighters.iter_mut().enumerate() {
        if group.escorts == 0 {
            continue;
        }
        let chosen = raids
            .iter()
            .enumerate()
            .filter(|(_, raid)| raid.strikes[0].origin == group.origin)
            .max_by_key(|(_, raid)| {
                let bombers: u32 = raid.strikes.iter().map(|strike| strike.aircraft).sum();
                (bombers, std::cmp::Reverse(raid.strikes[0].air_group))
            })
            .map(|(raid_index, _)| raid_index);
        if let Some(raid_index) = chosen {
            let within = group.hex.is_none_or(|hex| {
                hex.air_distance_within(raids[raid_index].strikes[0].hex, group.radius_nm)
            });
            if within {
                raids[raid_index].escorts.push(index);
            } else {
                group.patrol += group.escorts;
                group.escorts = 0;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::events::Event;
    use crate::ids::{AircraftTypeId, OrderId, ShipClassId};
    use crate::map::{HexData, Map};
    use crate::orders::{AirGroupOrder, Mission, OrderKind, OrderRequest, Origin, RejectReason};
    use crate::refdata::{
        AirCombatFigures, AircraftRole, AircraftType, PortSize, RefData, SearchFigures, ShipClass,
        SurfaceCombatFigures,
    };
    use crate::start::StartingContact;
    use crate::state::{
        AirGroup, Ship, ShipLocation, ShipSystems, SystemState, TaskForce, TruthState,
    };

    // ---- a fixture ----
    //
    // Open sea from q -12 to 12 and r -6 to 6. Classes: 1 a cruiser (toughness
    // 400), 2 a carrier (toughness 600, carries 60 aircraft). Types: 1 a scout
    // (search 300 nm, sweep 20), 2 a dive bomber (bomb, 60 damage, strikes to
    // 250 nm), 3 a torpedo bomber (torpedo, 200 damage, 250 nm), 4 a bomber
    // that attacks with nothing, 5 a fighter, 6 a short-range bomber (bomb, 60
    // damage, strikes to 100 nm).
    //
    // Allied task force 1 at (0,0): carrier ship 1, with air group 1, 18 dive
    // bombers on the naval strike mission, and air group 2, 8 scouts at rest.
    // Japanese task force 2 at (5,0): carrier ship 2, cruisers ship 3 and
    // ship 4, with air group 3, 18 dive bombers at rest.

    const FORCE_A: TaskForceId = TaskForceId::new(1);
    const FORCE_B: TaskForceId = TaskForceId::new(2);
    const BOMBERS: AirGroupId = AirGroupId::new(1);
    const SCOUTS: AirGroupId = AirGroupId::new(2);
    const ENEMY_BOMBERS: AirGroupId = AirGroupId::new(3);

    fn fixed(num: i64, den: i64) -> Fixed {
        Fixed::from_ratio(num, den)
    }

    fn class(id: u32, kind: ShipKind, toughness: i64, capacity: u32) -> ShipClass {
        ShipClass {
            id: ShipClassId::new(id),
            name: format!("Test class {id}"),
            kind,
            cruise_speed_kn: Fixed::from_int(16),
            max_speed_kn: Fixed::from_int(32),
            fuel_capacity_t: Fixed::from_int(2000),
            endurance_nm: Fixed::from_int(8000),
            cargo_capacity_t: Fixed::ZERO,
            main_guns: if capacity == 0 { 8 } else { 0 },
            gun_rounds_per_min: if capacity == 0 {
                Fixed::from_int(3)
            } else {
                Fixed::ZERO
            },
            gun_rounds_per_gun: if capacity == 0 { 100 } else { 0 },
            gun_damage: if capacity == 0 {
                Fixed::from_int(20)
            } else {
                Fixed::ZERO
            },
            torpedo_tubes: 0,
            torpedo_damage: Fixed::ZERO,
            toughness: Fixed::from_int(toughness),
            radar: false,
            aircraft_capacity: capacity,
            anti_aircraft: 0,
        }
    }

    fn aircraft(id: u32, role: AircraftRole, weapon: AttackWeapon, damage: i64) -> AircraftType {
        let scout = role == AircraftRole::Patrol;
        AircraftType {
            id: AircraftTypeId::new(id),
            name: format!("Test type {id}"),
            role,
            search_range_nm: Fixed::from_int(if scout { 300 } else { 0 }),
            sweep_width_nm: Fixed::from_int(if scout { 20 } else { 0 }),
            strike_range_nm: Fixed::from_int(match (weapon, id) {
                // A fighter's radius covers every test target (decision 0030).
                (AttackWeapon::None, _) if role == AircraftRole::Fighter => 250,
                (AttackWeapon::None, _) => 0,
                (_, 6) => 100,
                _ => 250,
            }),
            attack_weapon: weapon,
            attack_damage: Fixed::from_int(damage),
            pilot_armour: false,
            self_sealing_tanks: false,
        }
    }

    fn group(id: AirGroupId, side: Side, ship: u32, type_id: u32, ready: u32) -> AirGroup {
        AirGroup {
            id,
            name: format!("Test group {}", id.raw()),
            side,
            location: AirGroupLocation::Ship(ShipId::new(ship)),
            aircraft_type: AircraftTypeId::new(type_id),
            ready,
            damaged: 0,
            mission: AirMission::Rest,
            search: None,
            mission_order: None,
            arc_order: None,
            patrol_share: Fixed::ONE,
            share_order: None,
            fatigue: Fixed::ZERO,
            rest_above: None,
            rest_order: None,
            stood_down: false,
            pilots: None,
            strength: ready,
        }
    }

    fn ship(id: u32, class: u32, side: Side, force: TaskForceId) -> Ship {
        Ship {
            id: ShipId::new(id),
            name: format!("Test ship {id}"),
            class: ShipClassId::new(class),
            side,
            fuel_t: Fixed::from_int(2000),
            supply_t: Fixed::ZERO,
            cargo_fuel_t: Fixed::ZERO,
            location: ShipLocation::TaskForce(force),
            flotation_damage: Fixed::ZERO,
            fire_damage: Fixed::ZERO,
            gun_rounds: Fixed::ZERO,
            torpedoes_ready: false,
            systems: ShipSystems::ALL_WORKING,
        }
    }

    fn held(hex: Hex, kind: ShipKind, age_pulses: u32) -> StartingContact {
        StartingContact {
            side: Side::Allied,
            hex,
            ship_count: 3,
            kind,
            age_pulses,
        }
    }

    /// The standard world with the Allied side holding `contacts`.
    fn world(seed: u64, contacts: Vec<StartingContact>) -> World {
        let mut map = Map::new();
        for q in -12..=12 {
            for r in -6..=6 {
                map.insert(Hex::new(q, r), HexData::open_sea());
            }
        }
        let mut ref_data = RefData::default();
        ref_data
            .ship_classes
            .insert(ShipClassId::new(1), class(1, ShipKind::Cruiser, 400, 0));
        ref_data
            .ship_classes
            .insert(ShipClassId::new(2), class(2, ShipKind::Carrier, 600, 60));
        ref_data.port_sizes.insert(
            0,
            PortSize {
                berths: 0,
                berth_rate_t_per_h: Fixed::ZERO,
                lighter_rate_t_per_h: Fixed::from_int(20),
            },
        );
        for aircraft_type in [
            aircraft(1, AircraftRole::Patrol, AttackWeapon::None, 0),
            aircraft(2, AircraftRole::Bomber, AttackWeapon::Bomb, 60),
            aircraft(3, AircraftRole::Bomber, AttackWeapon::Torpedo, 200),
            aircraft(4, AircraftRole::Bomber, AttackWeapon::None, 0),
            aircraft(5, AircraftRole::Fighter, AttackWeapon::None, 0),
            aircraft(6, AircraftRole::Bomber, AttackWeapon::Bomb, 60),
        ] {
            ref_data
                .aircraft_types
                .insert(aircraft_type.id, aircraft_type);
        }
        ref_data.search = Some(SearchFigures::from_search_toml());
        ref_data.surface_combat = Some(SurfaceCombatFigures::from_surface_combat_toml());
        ref_data.air_combat = Some(AirCombatFigures::from_air_combat_toml());

        let mut truth = TruthState::default();
        for (id, side, hex, ships) in [
            (FORCE_A, Side::Allied, Hex::new(0, 0), vec![(1, 2)]),
            (
                FORCE_B,
                Side::Japanese,
                Hex::new(5, 0),
                vec![(2, 2), (3, 1), (4, 1)],
            ),
        ] {
            let mut force = TaskForce::at(
                id,
                format!("Test force {}", id.raw()),
                side,
                hex,
                Mission::Carrier,
            );
            for (ship_id, class_id) in ships {
                force.ships.insert(ShipId::new(ship_id));
                truth
                    .ships
                    .insert(ShipId::new(ship_id), ship(ship_id, class_id, side, id));
            }
            truth.task_forces.insert(id, force);
        }
        let mut bombers = group(BOMBERS, Side::Allied, 1, 2, 18);
        bombers.mission = AirMission::NavalStrike;
        for group in [
            bombers,
            group(SCOUTS, Side::Allied, 1, 1, 8),
            group(ENEMY_BOMBERS, Side::Japanese, 2, 2, 18),
        ] {
            truth.air_groups.insert(group.id, group);
        }
        let mut world = World::new(seed, ref_data, map, truth).expect("valid setup");
        world
            .hold_starting_contacts(contacts)
            .expect("the contacts are held");
        world
    }

    /// The Allied side holds a fresh cruiser contact at the enemy's true hex.
    fn world_with_a_good_contact(seed: u64) -> World {
        world(seed, vec![held(Hex::new(5, 0), ShipKind::Cruiser, 0)])
    }

    fn air_figures(world: &mut World) -> &mut AirCombatFigures {
        world.ref_data.air_combat.as_mut().expect("air figures")
    }

    fn surface_figures(world: &mut World) -> &mut SurfaceCombatFigures {
        world
            .ref_data
            .surface_combat
            .as_mut()
            .expect("surface figures")
    }

    fn group_mut(world: &mut World, id: AirGroupId) -> &mut AirGroup {
        world.truth.air_groups.get_mut(&id).expect("group")
    }

    fn ship_of(world: &World, id: u32) -> &Ship {
        &world.truth.ships[&ShipId::new(id)]
    }

    fn ship_mut(world: &mut World, id: u32) -> &mut Ship {
        world.truth.ships.get_mut(&ShipId::new(id)).expect("ship")
    }

    fn strike_pulse(world: &mut World, pulse: Pulse) {
        world.decide_strikes(1, pulse);
        world.resolve_strikes(1, pulse);
    }

    fn events_where(world: &World, keep: impl Fn(&EventKind) -> bool) -> Vec<Event> {
        world
            .events()
            .iter()
            .filter(|event| keep(&event.kind))
            .cloned()
            .collect()
    }

    fn launches(world: &World) -> Vec<Event> {
        events_where(world, |k| matches!(k, EventKind::StrikeLaunched { .. }))
    }

    fn attacks(world: &World) -> Vec<Event> {
        events_where(world, |k| matches!(k, EventKind::AirAttack { .. }))
    }

    fn found_nothing(world: &World) -> Vec<Event> {
        events_where(world, |k| matches!(k, EventKind::StrikeFoundNothing { .. }))
    }

    fn system_hits(world: &World) -> Vec<(u32, ShipSystem, SystemState, SystemState)> {
        events_where(world, |k| matches!(k, EventKind::SystemHit { .. }))
            .iter()
            .map(|event| match event.kind {
                EventKind::SystemHit {
                    ship,
                    system,
                    before,
                    after,
                    ..
                } => (ship.raw(), system, before, after),
                _ => unreachable!(),
            })
            .collect()
    }

    fn launch_of(event: &Event) -> (u32, u32, Hex, u32, Fixed) {
        match event.kind {
            EventKind::StrikeLaunched {
                air_group,
                contact,
                hex,
                aircraft,
                distance_nm,
            } => (air_group.raw(), contact.raw(), hex, aircraft, distance_nm),
            _ => panic!("not a launch"),
        }
    }

    fn attack_of(event: &Event) -> (u32, u32, u32, Weapon, Fixed, Fixed, Fixed, u32) {
        match event.kind {
            EventKind::AirAttack {
                air_group,
                task_force,
                ship,
                weapon,
                shots,
                chance,
                expected,
                hits,
            } => (
                air_group.raw(),
                task_force.raw(),
                ship.raw(),
                weapon,
                shots,
                chance,
                expected,
                hits,
            ),
            _ => panic!("not an attack"),
        }
    }

    // ---- the decision ----

    #[test]
    fn with_no_contact_a_group_launches_nothing_and_records_nothing() {
        let mut world = world(1, vec![]);
        let before = world.events().len();
        strike_pulse(&mut world, Pulse::Morning);
        assert_eq!(world.events().len(), before);
        assert!(world.strikes_in_flight.is_empty());
    }

    #[test]
    fn a_contact_launches_every_aircraft_the_group_can_fly_citing_the_order_and_the_report() {
        let mut world = world_with_a_good_contact(1);
        let order = OrderId::new(7);
        group_mut(&mut world, BOMBERS).mission_order = Some(order);
        let report = world
            .beliefs
            .get(&Side::Allied)
            .expect("belief")
            .contacts()
            .next()
            .expect("a contact")
            .last_report_event;
        world.decide_strikes(1, Pulse::Morning);
        let all = launches(&world);
        assert_eq!(all.len(), 1);
        assert_eq!(
            launch_of(&all[0]),
            (1, 1, Hex::new(5, 0), 18, Fixed::from_int(200))
        );
        assert_eq!(
            all[0].causes,
            vec![Cause::Order(order), Cause::Event(report)]
        );
        assert_eq!(all[0].pulse, Some(Pulse::Morning));
        assert_eq!(
            all[0].kind.to_string(),
            "air_group:1 launched 18 aircraft at contact:1 at (5,0), 200 nm"
        );
        // Nothing lands until the air combat step.
        assert!(attacks(&world).is_empty());
        assert_eq!(world.strikes_in_flight.len(), 1);
    }

    #[test]
    fn a_group_that_does_not_strike_decides_nothing() {
        type Change = fn(&mut World);
        let setups: [(&str, Change); 5] = [
            ("resting", |w| {
                group_mut(w, BOMBERS).mission = AirMission::Rest
            }),
            ("searching", |w| {
                group_mut(w, BOMBERS).mission = AirMission::Search;
            }),
            ("no aircraft", |w| group_mut(w, BOMBERS).ready = 0),
            ("no position", |w| {
                let ship = ship_mut(w, 1);
                ship.location = ShipLocation::Sunk;
                w.truth
                    .task_forces
                    .get_mut(&FORCE_A)
                    .expect("force")
                    .ships
                    .remove(&ShipId::new(1));
            }),
            ("attacks with nothing", |w| {
                group_mut(w, BOMBERS).aircraft_type = AircraftTypeId::new(4);
            }),
        ];
        for (label, change) in setups {
            let mut world = world_with_a_good_contact(1);
            change(&mut world);
            let before = world.events().len();
            strike_pulse(&mut world, Pulse::Morning);
            assert_eq!(world.events().len(), before, "{label}");
        }
    }

    #[test]
    fn there_is_no_strike_decision_in_the_dark() {
        let mut world = world_with_a_good_contact(1);
        let before = world.events().len();
        strike_pulse(&mut world, Pulse::Night);
        assert_eq!(world.events().len(), before);
    }

    #[test]
    fn a_contact_older_than_the_figure_is_not_struck_and_one_exactly_that_old_is() {
        // The figure is one pulse.
        for (age, launches_at) in [(0, true), (1, true), (2, false), (3, false)] {
            let mut world = world(1, vec![held(Hex::new(5, 0), ShipKind::Cruiser, age)]);
            world.decide_strikes(1, Pulse::Morning);
            assert_eq!(
                launches(&world).len(),
                usize::from(launches_at),
                "age {age}"
            );
        }
        let mut world = world(1, vec![held(Hex::new(5, 0), ShipKind::Cruiser, 2)]);
        air_figures(&mut world).strike_contact_max_age_pulses = 2;
        world.decide_strikes(1, Pulse::Morning);
        assert_eq!(launches(&world).len(), 1);
    }

    #[test]
    fn a_contact_beyond_the_strike_range_is_not_struck_and_one_at_the_range_is() {
        // Six hexes is 240 nm and seven is 280; the type reaches 250.
        for (hex, launches_at) in [(Hex::new(6, 0), true), (Hex::new(7, 0), false)] {
            let mut world = world(1, vec![held(hex, ShipKind::Cruiser, 0)]);
            world.decide_strikes(1, Pulse::Morning);
            assert_eq!(launches(&world).len(), usize::from(launches_at), "{hex}");
        }
        // The range is compared exactly: 240 nm reaches six hexes, 239 does not.
        for (range_nm, launches_at) in [(240, true), (239, false)] {
            let mut world = self::world(1, vec![held(Hex::new(6, 0), ShipKind::Cruiser, 0)]);
            world
                .ref_data
                .aircraft_types
                .get_mut(&AircraftTypeId::new(2))
                .expect("type")
                .strike_range_nm = Fixed::from_int(range_nm);
            world.decide_strikes(1, Pulse::Morning);
            assert_eq!(
                launches(&world).len(),
                usize::from(launches_at),
                "{range_nm}"
            );
        }
    }

    #[test]
    fn it_chooses_the_most_important_kind_then_the_nearest_then_the_lowest_contact() {
        // Contacts are made in the order given, so ids 1, 2, 3, 4. They are
        // more than two hexes apart, so none merges with another.
        let chosen = |contacts: Vec<StartingContact>| {
            let mut world = world(1, contacts);
            world.decide_strikes(1, Pulse::Morning);
            let all = launches(&world);
            assert_eq!(all.len(), 1);
            launch_of(&all[0])
        };
        // The carrier is the most important, though the farthest.
        let (_, contact, hex, _, _) = chosen(vec![
            held(Hex::new(3, 0), ShipKind::Cruiser, 0),
            held(Hex::new(-4, 0), ShipKind::Destroyer, 0),
            held(Hex::new(0, 6), ShipKind::Carrier, 0),
        ]);
        assert_eq!((contact, hex), (3, Hex::new(0, 6)));
        // Of equal kind the nearest: 4 hexes against 3 and 6.
        let (_, contact, hex, _, distance) = chosen(vec![
            held(Hex::new(4, 0), ShipKind::Cruiser, 0),
            held(Hex::new(-3, 0), ShipKind::Cruiser, 0),
            held(Hex::new(0, 6), ShipKind::Cruiser, 0),
        ]);
        assert_eq!((contact, hex), (2, Hex::new(-3, 0)));
        assert_eq!(distance, Fixed::from_int(120));
        // Of equal kind and distance the lowest contact number.
        let (_, contact, hex, _, _) = chosen(vec![
            held(Hex::new(4, 0), ShipKind::Cruiser, 0),
            held(Hex::new(-4, 0), ShipKind::Cruiser, 0),
        ]);
        assert_eq!((contact, hex), (1, Hex::new(4, 0)));
        // A contact out of range does not count however important it is.
        let (_, contact, _, _, _) = chosen(vec![
            held(Hex::new(9, 0), ShipKind::Carrier, 0),
            held(Hex::new(4, 0), ShipKind::Destroyer, 0),
        ]);
        assert_eq!(contact, 2);
    }

    #[test]
    fn the_strike_flies_to_the_hex_the_contact_names_not_to_where_the_enemy_is() {
        // The enemy is at (5,0). The contact is a hex out of place.
        let mut world = world(1, vec![held(Hex::new(4, 0), ShipKind::Carrier, 0)]);
        strike_pulse(&mut world, Pulse::Morning);
        assert_eq!(launch_of(&launches(&world)[0]).2, Hex::new(4, 0));
        assert!(attacks(&world).is_empty());
        let nothing = found_nothing(&world);
        assert_eq!(nothing.len(), 1);
        assert_eq!(
            nothing[0].kind.to_string(),
            "air_group:1 found nothing at (4,0)"
        );
        let launch = launches(&world)[0].id;
        assert_eq!(nothing[0].causes, vec![Cause::Event(launch)]);
        // Nothing is hit and the side hears nothing more.
        for id in 1..=4 {
            assert_eq!(ship_of(&world, id).flotation_damage, Fixed::ZERO);
            assert_eq!(ship_of(&world, id).fire_damage, Fixed::ZERO);
        }
        assert!(
            events_where(&world, |k| matches!(k, EventKind::ReportReceived { .. }))
                .iter()
                .all(|event| event.day == 0)
        );
        assert_eq!(group_mut(&mut world, BOMBERS).ready, 18);
    }

    #[test]
    fn a_strike_never_reads_where_the_enemy_really_is() {
        // Moving the enemy changes what the attack finds and nothing about
        // the decision.
        let launched = |enemy_at: Hex| {
            let mut world = world_with_a_good_contact(1);
            world
                .truth
                .task_forces
                .get_mut(&FORCE_B)
                .expect("force")
                .from_hex = enemy_at;
            world.decide_strikes(1, Pulse::Morning);
            launches(&world)
                .iter()
                .map(|e| e.kind.clone())
                .collect::<Vec<_>>()
        };
        assert_eq!(launched(Hex::new(5, 0)), launched(Hex::new(-8, 3)));
    }

    // ---- the attack ----

    /// Lets every bomb hit land on the flight deck: all hits are system
    /// hits, all on the flight deck. Makes the flight deck the only thing a
    /// strike can change besides the two damage tracks.
    fn every_hit_is_a_flight_deck_hit(world: &mut World) {
        surface_figures(world).system_hit_chance_per_point = Fixed::from_int(100);
        let air = air_figures(world);
        air.bomb_system_hit = [(ShipSystem::FlightDeck, Fixed::ONE)].into_iter().collect();
    }

    /// Adds a task force with the given ships, each `(id, class)`.
    fn add_force(world: &mut World, id: u32, side: Side, hex: Hex, ships: &[(u32, u32)]) {
        let id = TaskForceId::new(id);
        let mut force = TaskForce::at(
            id,
            format!("Added {}", id.raw()),
            side,
            hex,
            Mission::Carrier,
        );
        for (ship_id, class_id) in ships {
            force.ships.insert(ShipId::new(*ship_id));
            world
                .truth
                .ships
                .insert(ShipId::new(*ship_id), ship(*ship_id, *class_id, side, id));
        }
        world.truth.task_forces.insert(id, force);
    }

    fn set_class(world: &mut World, ship_id: u32, class_id: u32) {
        ship_mut(world, ship_id).class = ShipClassId::new(class_id);
    }

    #[test]
    fn a_strike_with_no_enemy_force_in_the_hex_finds_nothing_even_if_ours_is_there() {
        // An Allied force in the hex is not an enemy.
        let mut world = world(1, vec![held(Hex::new(0, 0), ShipKind::Cruiser, 0)]);
        strike_pulse(&mut world, Pulse::Morning);
        assert_eq!(found_nothing(&world).len(), 1);
        assert!(attacks(&world).is_empty());
    }

    #[test]
    fn it_attacks_the_force_with_the_most_important_ship_then_the_lowest_number() {
        let target_of = |build: fn(&mut World)| {
            let mut world = world_with_a_good_contact(1);
            build(&mut world);
            strike_pulse(&mut world, Pulse::Morning);
            let all = attacks(&world);
            assert_eq!(all.len(), 1);
            let (_, force, ship, ..) = attack_of(&all[0]);
            (force, ship)
        };
        // Force 2 holds a carrier (ship 2) and two cruisers. Force 3 is
        // cruisers only, so it is not taken though it is in the same hex.
        assert_eq!(
            target_of(|w| add_force(w, 3, Side::Japanese, Hex::new(5, 0), &[(5, 1)])),
            (2, 2)
        );
        // Force 2 has no carrier now and force 3 has: force 3.
        assert_eq!(
            target_of(|w| {
                set_class(w, 2, 1);
                add_force(w, 3, Side::Japanese, Hex::new(5, 0), &[(5, 1), (6, 2)]);
            }),
            (3, 6)
        );
        // Two forces each with a carrier: the lower number.
        assert_eq!(
            target_of(|w| add_force(w, 3, Side::Japanese, Hex::new(5, 0), &[(5, 2)])),
            (2, 2)
        );
        // A force of another hex is not there.
        assert_eq!(
            target_of(|w| add_force(w, 3, Side::Japanese, Hex::new(4, 0), &[(5, 2)])),
            (2, 2)
        );
        // Within the force the most important ship, the lowest number on a
        // tie: ship 2 is made a cruiser, so the carrier is ship 8, not the
        // first ship; with two carriers the lower one.
        assert_eq!(
            target_of(|w| {
                set_class(w, 2, 1);
                w.truth
                    .task_forces
                    .get_mut(&FORCE_B)
                    .expect("force")
                    .ships
                    .insert(ShipId::new(8));
                w.truth
                    .ships
                    .insert(ShipId::new(8), ship(8, 2, Side::Japanese, FORCE_B));
            }),
            (2, 8)
        );
        assert_eq!(
            target_of(|w| {
                w.truth
                    .task_forces
                    .get_mut(&FORCE_B)
                    .expect("force")
                    .ships
                    .insert(ShipId::new(8));
                w.truth
                    .ships
                    .insert(ShipId::new(8), ship(8, 2, Side::Japanese, FORCE_B));
            }),
            (2, 2)
        );
        // Another Allied force is never the target.
        assert_eq!(
            target_of(|w| add_force(w, 0, Side::Allied, Hex::new(5, 0), &[(9, 2)])),
            (2, 2)
        );
    }

    #[test]
    fn each_shot_of_a_strike_is_rolled_on_its_own() {
        // 18 shots at 0.2 expect 3.6 hits (decision 0033, 3): the hits are
        // the shots that hit, so they are what 18 draws at 0.2 give, and over
        // many seeds they are not always 3 or 4.
        let mut seen = std::collections::BTreeSet::new();
        for seed in 1..=60 {
            let mut world = world_with_a_good_contact(seed);
            strike_pulse(&mut world, Pulse::Morning);
            let (_, _, _, weapon, shots, chance, expected, hits) = attack_of(&attacks(&world)[0]);
            assert_eq!(weapon, Weapon::Bomb);
            assert_eq!(shots, Fixed::from_int(18));
            assert_eq!(chance, fixed(2, 10));
            assert_eq!(expected, fixed(36, 10));
            let mut stream = world.rng_pair(
                1,
                Some(Pulse::Morning),
                Subsystem::AirCombat,
                EntityRef::AirGroup(BOMBERS),
                EntityRef::TaskForce(FORCE_B),
            );
            let replayed: u32 = (0..18)
                .map(|_| u32::from(stream.chance(fixed(2, 10))))
                .sum();
            assert_eq!(hits, replayed, "seed {seed}");
            seen.insert(hits);
        }
        assert!(seen.len() > 2, "{seen:?}");
        // Certain hits and no hits.
        for (chance, hits) in [(Fixed::ONE, 18), (Fixed::ZERO, 0)] {
            let mut world = world_with_a_good_contact(1);
            air_figures(&mut world).bomb_hit_chance = chance;
            strike_pulse(&mut world, Pulse::Morning);
            assert_eq!(attack_of(&attacks(&world)[0]).7, hits);
        }
    }

    #[test]
    fn the_attack_event_reads_as_the_decision_says_and_cites_the_launch_with_a_breakdown() {
        let mut world = world_with_a_good_contact(3);
        strike_pulse(&mut world, Pulse::Morning);
        let attack = attacks(&world)[0].clone();
        let (_, _, _, _, _, _, _, hits) = attack_of(&attack);
        assert_eq!(
            attack.kind.to_string(),
            format!(
                "air_group:1 bombs at ship:2: 18 shots x 0.2 = 3.6 expected, {hits} {}",
                if hits == 1 { "hit" } else { "hits" }
            )
        );
        assert_eq!(attack.causes, vec![Cause::Event(launches(&world)[0].id)]);
        let breakdown = attack.breakdown.expect("breakdowns are on");
        assert_eq!(breakdown.result, fixed(36, 10));
        let factors: Vec<(&str, Fixed)> = breakdown
            .factors
            .iter()
            .map(|factor| (factor.name, factor.value))
            .collect();
        assert_eq!(
            factors,
            vec![
                ("aircraft", Fixed::from_int(18)),
                ("hit_chance", fixed(2, 10))
            ]
        );
    }

    #[test]
    fn a_bomb_hit_does_what_a_gun_hit_of_the_same_damage_does() {
        // 100 x 60 / 600 = 10 points a hit, on flotation or fire, and the
        // events have the form of a salvo's damage without a round.
        for seed in 1..=40 {
            let mut world = world_with_a_good_contact(seed);
            strike_pulse(&mut world, Pulse::Morning);
            let attack = attacks(&world)[0].clone();
            let hits = attack_of(&attack).7;
            let damaged = events_where(&world, |k| matches!(k, EventKind::ShipDamaged { .. }));
            let mut total_hits = 0;
            for event in &damaged {
                let EventKind::ShipDamaged {
                    round,
                    ship,
                    track,
                    hits,
                    points,
                    value_after,
                } = event.kind
                else {
                    unreachable!()
                };
                assert_eq!((round, ship.raw()), (0, 2));
                assert_eq!(points, Fixed::from_int(10) * i64::from(hits));
                assert_eq!(
                    value_after,
                    match track {
                        DamageTrack::Flotation => ship_of(&world, 2).flotation_damage,
                        DamageTrack::Fire => ship_of(&world, 2).fire_damage,
                    }
                );
                assert_eq!(event.causes, vec![Cause::Event(attack.id)]);
                assert!(event.kind.to_string().starts_with("ship:2 took "));
                total_hits += hits;
            }
            assert_eq!(total_hits, hits);
            assert_eq!(
                ship_of(&world, 2).flotation_damage + ship_of(&world, 2).fire_damage,
                Fixed::from_int(10) * i64::from(hits)
            );
            // Nothing else is damaged.
            for id in [1, 3, 4] {
                assert_eq!(ship_of(&world, id).flotation_damage, Fixed::ZERO);
                assert_eq!(ship_of(&world, id).fire_damage, Fixed::ZERO);
            }
            // The aircraft all come home.
            assert_eq!(group_mut(&mut world, BOMBERS).ready, 18);
        }
        assert_eq!(
            damage_per_hit(Fixed::from_int(60), Fixed::from_int(600)),
            Fixed::from_int(10)
        );
    }

    #[test]
    fn a_torpedo_from_the_air_hits_flotation_and_uses_the_torpedo_system_table() {
        // Torpedo bombers, certain hits on the force's cruiser (ship 2 is a
        // cruiser now): 9 hits of 100 x 200 / 400 = 50 points would sink it,
        // so the chance is 0.5 of 18 = 9 hits only to count them: set the
        // damage low enough that it survives, 100 x 8 / 400 = 2 points.
        let mut world = world_with_a_good_contact(1);
        set_class(&mut world, 2, 1);
        group_mut(&mut world, BOMBERS).aircraft_type = AircraftTypeId::new(3);
        world
            .ref_data
            .aircraft_types
            .get_mut(&AircraftTypeId::new(3))
            .expect("type")
            .attack_damage = Fixed::from_int(8);
        air_figures(&mut world).torpedo_hit_chance = fixed(1, 2);
        {
            let surface = surface_figures(&mut world);
            surface.system_hit_chance_per_point = Fixed::from_int(100);
            surface.torpedo_system_hit = [(ShipSystem::FireControl, Fixed::ONE)]
                .into_iter()
                .collect();
        }
        strike_pulse(&mut world, Pulse::Morning);
        let (_, _, ship, weapon, shots, chance, expected, hits) = attack_of(&attacks(&world)[0]);
        // Decision 0033: 18 draws at 0.5 give the hits; replay them.
        let mut stream = world.rng_pair(
            1,
            Some(Pulse::Morning),
            Subsystem::AirCombat,
            EntityRef::AirGroup(BOMBERS),
            EntityRef::TaskForce(FORCE_B),
        );
        let replayed: u32 = (0..18).map(|_| u32::from(stream.chance(fixed(1, 2)))).sum();
        assert!(replayed >= 3, "enough hits to walk fire control down");
        assert_eq!(
            (ship, weapon, shots, chance, expected, hits),
            (
                2,
                Weapon::Torpedo,
                Fixed::from_int(18),
                fixed(1, 2),
                Fixed::from_int(9),
                replayed
            )
        );
        assert!(attacks(&world)[0].kind.to_string().starts_with(&format!(
            "air_group:1 torpedoes at ship:2: 18 shots x 0.5 = 9 expected, {replayed} hits"
        )));
        // All on flotation: 2 points a hit, no fire.
        assert_eq!(
            ship_of(&world, 2).flotation_damage,
            Fixed::from_int(2 * i64::from(replayed))
        );
        assert_eq!(ship_of(&world, 2).fire_damage, Fixed::ZERO);
        // One system hit on fire control for each hit: working -> degraded ->
        // out, and then the rest change nothing and say so.
        let hits = system_hits(&world);
        assert_eq!(hits.len(), usize::try_from(replayed).expect("small"));
        assert_eq!(
            hits[0],
            (
                2,
                ShipSystem::FireControl,
                SystemState::Working,
                SystemState::Degraded
            )
        );
        assert_eq!(
            hits[1],
            (
                2,
                ShipSystem::FireControl,
                SystemState::Degraded,
                SystemState::Out
            )
        );
        for hit in &hits[2..] {
            assert_eq!(
                *hit,
                (
                    2,
                    ShipSystem::FireControl,
                    SystemState::Out,
                    SystemState::Out
                )
            );
        }
        let texts: Vec<String> = events_where(&world, |k| matches!(k, EventKind::SystemHit { .. }))
            .iter()
            .map(|e| e.kind.to_string())
            .collect();
        assert_eq!(texts[0], "ship:2 fire control hit, working -> degraded");
        assert_eq!(
            texts[usize::try_from(replayed).expect("small") - 1],
            "ship:2 fire control hit, no change, still out"
        );
        assert_eq!(ship_of(&world, 2).systems.fire_control, SystemState::Out);
    }

    #[test]
    fn a_ship_that_reaches_100_flotation_sinks_with_its_air_groups_and_leaves_its_force() {
        // Torpedoes of 600 damage on the carrier (toughness 600) are 100
        // points a hit: one hit sinks it.
        let mut world = world_with_a_good_contact(1);
        group_mut(&mut world, BOMBERS).aircraft_type = AircraftTypeId::new(3);
        world
            .ref_data
            .aircraft_types
            .get_mut(&AircraftTypeId::new(3))
            .expect("type")
            .attack_damage = Fixed::from_int(600);
        air_figures(&mut world).torpedo_hit_chance = Fixed::ONE;
        strike_pulse(&mut world, Pulse::Morning);
        assert_eq!(ship_of(&world, 2).location, ShipLocation::Sunk);
        assert_eq!(ship_of(&world, 2).flotation_damage, Fixed::from_int(100));
        assert!(!world.truth.task_forces[&FORCE_B]
            .ships
            .contains(&ShipId::new(2)));
        let sunk = events_where(&world, |k| matches!(k, EventKind::ShipSunk { .. }));
        assert_eq!(sunk.len(), 1);
        assert_eq!(sunk[0].kind.to_string(), "ship:2 of task_force:2 sank");
        let lost = events_where(&world, |k| matches!(k, EventKind::AirGroupLost { .. }));
        assert_eq!(lost.len(), 1);
        assert_eq!(lost[0].causes, vec![Cause::Event(sunk[0].id)]);
        assert_eq!(group_mut(&mut world, ENEMY_BOMBERS).ready, 0);
        // The sinking is told once, though 18 hits fell.
        assert_eq!(sunk.len(), 1);
    }

    // ---- the flight deck ----

    fn deck(world: &mut World, ship: u32, state: SystemState) {
        ship_mut(world, ship).systems.flight_deck = state;
    }

    #[test]
    fn a_degraded_flight_deck_flies_half_rounded_down_and_one_that_is_out_flies_none() {
        let flown = |state: SystemState, ready: u32| {
            let mut world = world_with_a_good_contact(1);
            deck(&mut world, 1, state);
            group_mut(&mut world, BOMBERS).ready = ready;
            world.decide_strikes(1, Pulse::Morning);
            launches(&world).first().map(|launch| launch_of(launch).3)
        };
        assert_eq!(flown(SystemState::Working, 18), Some(18));
        assert_eq!(flown(SystemState::Degraded, 18), Some(9));
        assert_eq!(flown(SystemState::Degraded, 7), Some(3));
        assert_eq!(flown(SystemState::Degraded, 1), None);
        assert_eq!(flown(SystemState::Out, 18), None);
        // From a base the deck is not in it.
        let mut world = world_with_a_good_contact(1);
        deck(&mut world, 1, SystemState::Out);
        world.truth.bases.insert(
            crate::ids::BaseId::new(1),
            crate::state::Base {
                id: crate::ids::BaseId::new(1),
                name: "Test base".to_string(),
                side: Side::Allied,
                hex: Hex::new(0, 0),
                supply_t: Fixed::ZERO,
                anti_aircraft: 0,
                aircraft_repairs_per_day: 0,
                fuel_t: Fixed::ZERO,
            },
        );
        group_mut(&mut world, BOMBERS).location =
            AirGroupLocation::Base(crate::ids::BaseId::new(1));
        world.decide_strikes(1, Pulse::Morning);
        assert_eq!(launch_of(&launches(&world)[0]).3, 18);
    }

    #[test]
    fn a_degraded_flight_deck_halves_the_search_and_one_that_is_out_stops_it() {
        let searched = |state: SystemState| {
            let mut world = world_with_a_good_contact(1);
            deck(&mut world, 1, state);
            let scouts = group_mut(&mut world, SCOUTS);
            scouts.mission = AirMission::Search;
            scouts.search = Some(crate::state::SearchArc {
                from_deg: 40,
                to_deg: 130,
                range_nm: Fixed::from_int(300),
            });
            world.search_from_the_air(1, Pulse::Morning);
            let flights = events_where(&world, |k| matches!(k, EventKind::SearchFlown { .. }));
            let attempts = events_where(&world, |k| matches!(k, EventKind::SearchAttempt { .. }));
            let flown: Vec<u32> = flights
                .iter()
                .map(|e| match e.kind {
                    EventKind::SearchFlown { aircraft, .. } => aircraft,
                    _ => unreachable!(),
                })
                .collect();
            let chances: Vec<Fixed> = attempts
                .iter()
                .map(|e| match e.kind {
                    EventKind::SearchAttempt { chance, .. } => chance,
                    _ => unreachable!(),
                })
                .collect();
            (flown, chances, attempts.len())
        };
        // 57.295779 x 8 x 20 / (90 x 200) = 0.509295, and with 4: 0.254647.
        assert_eq!(
            searched(SystemState::Working),
            (vec![8], vec![Fixed::from_raw(509_295)], 1)
        );
        assert_eq!(
            searched(SystemState::Degraded),
            (vec![4], vec![Fixed::from_raw(254_647)], 1)
        );
        assert_eq!(searched(SystemState::Out), (vec![], vec![], 0));
    }

    #[test]
    fn a_flight_deck_hit_on_a_ship_that_carries_no_aircraft_changes_nothing_and_says_so() {
        let mut world = world_with_a_good_contact(1);
        set_class(&mut world, 2, 1);
        every_hit_is_a_flight_deck_hit(&mut world);
        strike_pulse(&mut world, Pulse::Morning);
        let hits = system_hits(&world);
        assert!(!hits.is_empty());
        for (ship, system, before, after) in &hits {
            assert_eq!(
                (*ship, *system, *before, *after),
                (
                    2,
                    ShipSystem::FlightDeck,
                    SystemState::Working,
                    SystemState::Working
                )
            );
        }
        assert_eq!(ship_of(&world, 2).systems, ShipSystems::ALL_WORKING);
        let first = events_where(&world, |k| matches!(k, EventKind::SystemHit { .. }))[0]
            .kind
            .to_string();
        assert_eq!(first, "ship:2 flight deck hit, no change, still working");
    }

    #[test]
    fn flight_deck_hits_on_a_carrier_step_it_down_and_the_hash_sees_it() {
        let mut world = world_with_a_good_contact(1);
        every_hit_is_a_flight_deck_hit(&mut world);
        air_figures(&mut world).bomb_hit_chance = fixed(1, 10);
        let working_hash = world.state_hash();
        world.decide_strikes(1, Pulse::Morning);
        world.resolve_strikes(1, Pulse::Morning);
        world.age_contacts_at_end_of_pulse(1, Pulse::Morning);
        let hits = attack_of(&attacks(&world)[0]).7;
        // 18 x 0.1 = 1.8 expected; each shot is rolled (decision 0033), so the
        // hits are what 18 draws at 0.1 give. This seed has at least one.
        assert!(hits >= 1, "{hits}");
        let expected = if hits == 1 {
            SystemState::Degraded
        } else {
            SystemState::Out
        };
        assert_eq!(ship_of(&world, 2).systems.flight_deck, expected);
        assert_ne!(world.state_hash(), working_hash);
        // Each state hashes differently.
        let hash_of = |state: SystemState| {
            let mut world = world_with_a_good_contact(1);
            deck(&mut world, 2, state);
            world.state_hash()
        };
        let (w, d, o) = (
            hash_of(SystemState::Working),
            hash_of(SystemState::Degraded),
            hash_of(SystemState::Out),
        );
        assert!(w != d && d != o && w != o);
    }

    // ---- both sides strike at once ----

    /// Both sides hold a fresh contact on the other's force, both bomber
    /// groups strike, and every hit lands on a flight deck.
    fn two_sided(seed: u64) -> World {
        let mut world = world(
            seed,
            vec![
                held(Hex::new(5, 0), ShipKind::Carrier, 0),
                StartingContact {
                    side: Side::Japanese,
                    hex: Hex::new(0, 0),
                    ship_count: 1,
                    kind: ShipKind::Carrier,
                    age_pulses: 0,
                },
            ],
        );
        group_mut(&mut world, ENEMY_BOMBERS).mission = AirMission::NavalStrike;
        every_hit_is_a_flight_deck_hit(&mut world);
        world
    }

    #[test]
    fn two_groups_striking_each_others_carriers_in_one_pulse_each_attack_at_full_strength() {
        for seed in 1..=20 {
            let mut world = two_sided(seed);
            strike_pulse(&mut world, Pulse::Morning);
            let all = attacks(&world);
            assert_eq!(all.len(), 2, "seed {seed}");
            let first = attack_of(&all[0]);
            let second = attack_of(&all[1]);
            // Each flies all 18 and attacks the other's carrier, though its
            // own carrier's flight deck is hit in the same pulse.
            assert_eq!((first.0, first.1, first.2), (1, 2, 2));
            assert_eq!((second.0, second.1, second.2), (3, 1, 1));
            assert_eq!(first.4, Fixed::from_int(18));
            assert_eq!(second.4, Fixed::from_int(18));
            assert_eq!(first.6, fixed(36, 10));
            assert_eq!(second.6, fixed(36, 10));
            // Both attacks are recorded before any damage lands...
            let last_attack = all[1].id;
            let first_damage = events_where(&world, |k| matches!(k, EventKind::SystemHit { .. }))
                .first()
                .expect("system hits")
                .id;
            assert!(last_attack < first_damage);
            // ...and damage lands in group order: all of group 1's hits
            // (on ship 2) before group 2's (on ship 1).
            let ships: Vec<u32> = system_hits(&world).iter().map(|h| h.0).collect();
            let split = ships
                .iter()
                .position(|ship| *ship == 1)
                .unwrap_or(ships.len());
            assert!(ships[..split].iter().all(|ship| *ship == 2));
            assert!(ships[split..].iter().all(|ship| *ship == 1));
            assert_eq!(ships[..split].len(), first.7 as usize);
            assert_eq!(ships[split..].len(), second.7 as usize);
            // No hit leaves a deck working, one degrades it, two take it out.
            let deck_after = |hits: u32| match hits {
                0 => SystemState::Working,
                1 => SystemState::Degraded,
                _ => SystemState::Out,
            };
            assert_eq!(
                ship_of(&world, 2).systems.flight_deck,
                deck_after(first.7),
                "seed {seed}"
            );
            assert_eq!(
                ship_of(&world, 1).systems.flight_deck,
                deck_after(second.7),
                "seed {seed}"
            );
        }
    }

    #[test]
    fn the_flight_decks_hit_in_one_pulse_stop_the_next_pulses_strikes_on_both_sides() {
        let mut world = two_sided(1);
        strike_pulse(&mut world, Pulse::Morning);
        assert_eq!(launches(&world).len(), 2);
        // Fresh contacts, decks out: nothing launches in the afternoon.
        strike_pulse(&mut world, Pulse::Afternoon);
        assert_eq!(launches(&world).len(), 2);
        assert_eq!(attacks(&world).len(), 2);
    }

    #[test]
    fn a_strike_launched_by_a_group_whose_ship_sinks_in_the_pulse_still_attacks() {
        // Carriers of toughness 10: a bomb of 60 does 600 points, so one hit
        // sinks one. Both strikes are launched first, so both land.
        for seed in 1..=10 {
            let mut world = two_sided(seed);
            world
                .ref_data
                .ship_classes
                .get_mut(&ShipClassId::new(2))
                .expect("class")
                .toughness = Fixed::from_int(10);
            air_figures(&mut world).bomb_hit_chance = Fixed::ONE;
            strike_pulse(&mut world, Pulse::Morning);
            let all = attacks(&world);
            assert_eq!(all.len(), 2, "seed {seed}");
            assert_eq!(attack_of(&all[0]).7, 18);
            assert_eq!(attack_of(&all[1]).7, 18);
            for ship in [1, 2] {
                assert_eq!(ship_of(&world, ship).location, ShipLocation::Sunk, "{ship}");
            }
            let sunk = events_where(&world, |k| matches!(k, EventKind::ShipSunk { .. }));
            assert_eq!(sunk.len(), 2);
            // Each air group was lost with its ship, after the attacks.
            let lost = events_where(&world, |k| matches!(k, EventKind::AirGroupLost { .. }));
            assert_eq!(
                lost.len(),
                3,
                "bombers and scouts aboard ship 1, bombers aboard ship 2"
            );
            assert!(lost.iter().all(|event| event.id > all[1].id));
            assert_eq!(group_mut(&mut world, BOMBERS).ready, 0);
            assert_eq!(group_mut(&mut world, ENEMY_BOMBERS).ready, 0);
        }
    }

    // ---- the report ----

    #[test]
    fn a_strike_that_attacks_reports_the_force_after_the_damage_citing_the_attack() {
        let mut world = world_with_a_good_contact(2);
        strike_pulse(&mut world, Pulse::Morning);
        let attack = attacks(&world)[0].id;
        let reports = events_where(&world, |k| matches!(k, EventKind::ReportReceived { .. }));
        // The starting contact, then the strike's report.
        let report = reports.last().expect("a report");
        assert_eq!(report.causes, vec![Cause::Event(attack)]);
        assert_eq!(report.day, 1);
        assert!(report.id > attack);
        let EventKind::ReportReceived {
            side,
            contact,
            ship_count,
            created,
            ..
        } = report.kind
        else {
            unreachable!()
        };
        assert_eq!(side, Side::Allied);
        // It refreshes the contact the strike flew at: one belief, one report.
        assert_eq!(contact.raw(), 1);
        assert!(!created);
        assert!((2..=4).contains(&ship_count));
        // The belief changed through the report: the contact was refreshed.
        let belief = world.beliefs.get(&Side::Allied).expect("belief");
        assert_eq!(belief.contacts().count(), 1);
        assert_eq!(
            belief.contact(contact).expect("contact").last_report_event,
            report.id
        );
    }

    #[test]
    fn a_strike_that_finds_nothing_sends_no_sighting_report() {
        let mut world = world(1, vec![held(Hex::new(4, 0), ShipKind::Cruiser, 0)]);
        let before = events_where(&world, |k| matches!(k, EventKind::ReportReceived { .. })).len();
        strike_pulse(&mut world, Pulse::Morning);
        let after = events_where(&world, |k| matches!(k, EventKind::ReportReceived { .. })).len();
        assert_eq!(before, after);
    }

    // ---- a strike that finds nothing drops its contact (decision 0028) ----

    fn not_there(world: &World) -> Vec<Event> {
        events_where(world, |k| matches!(k, EventKind::ContactNotThere { .. }))
    }

    /// A second Allied strike group on the same carrier, of type `type_id`.
    fn add_allied_strike_group(world: &mut World, id: u32, type_id: u32) {
        let mut second = group(AirGroupId::new(id), Side::Allied, 1, type_id, 18);
        second.mission = AirMission::NavalStrike;
        world.truth.air_groups.insert(second.id, second);
    }

    fn allied_contacts(world: &World) -> Vec<(u32, Hex)> {
        world.beliefs[&Side::Allied]
            .contacts()
            .map(|contact| (contact.id.raw(), contact.estimated_hex))
            .collect()
    }

    #[test]
    fn a_strike_that_finds_nothing_drops_the_contact_citing_the_finding() {
        let mut world = world(1, vec![held(Hex::new(4, 0), ShipKind::Cruiser, 0)]);
        assert_eq!(allied_contacts(&world), vec![(1, Hex::new(4, 0))]);
        strike_pulse(&mut world, Pulse::Morning);
        let nothing = found_nothing(&world);
        let dropped = not_there(&world);
        assert_eq!((nothing.len(), dropped.len()), (1, 1));
        assert_eq!(
            dropped[0].kind,
            EventKind::ContactNotThere {
                side: Side::Allied,
                contact: crate::ids::ContactId::new(1),
                hex: Hex::new(4, 0)
            }
        );
        assert_eq!(
            dropped[0].kind.to_string(),
            "Allied contact:1 not there at (4,0): dropped"
        );
        assert_eq!(dropped[0].causes, vec![Cause::Event(nothing[0].id)]);
        assert!(dropped[0].id > nothing[0].id);
        assert_eq!(dropped[0].pulse, Some(Pulse::Morning));
        assert_eq!(
            dropped[0].kind.entities(),
            vec![EntityRef::Contact(crate::ids::ContactId::new(1))]
        );
        // The belief changed through the report: the side holds nothing.
        assert!(allied_contacts(&world).is_empty());
    }

    #[test]
    fn the_next_pulse_launches_nothing_at_the_dropped_contact() {
        let mut world = world(1, vec![held(Hex::new(4, 0), ShipKind::Cruiser, 0)]);
        // Let a contact that is a pulse old still be struck, so that only
        // the drop stops the afternoon's strike.
        air_figures(&mut world).strike_contact_max_age_pulses = 1;
        strike_pulse(&mut world, Pulse::Morning);
        assert_eq!(launches(&world).len(), 1);
        strike_pulse(&mut world, Pulse::Afternoon);
        assert_eq!(launches(&world).len(), 1, "nothing is launched again");
        assert_eq!(found_nothing(&world).len(), 1);
        assert_eq!(not_there(&world).len(), 1);
    }

    #[test]
    fn a_strike_that_attacks_drops_nothing() {
        let mut world = world_with_a_good_contact(2);
        strike_pulse(&mut world, Pulse::Morning);
        assert_eq!(attacks(&world).len(), 1);
        assert!(not_there(&world).is_empty());
        assert_eq!(allied_contacts(&world).len(), 1);
    }

    #[test]
    fn a_second_strike_sent_at_a_contact_already_dropped_changes_nothing_and_records_nothing() {
        // Both groups are sent at the one contact, at a hex nobody holds.
        let mut world = world(1, vec![held(Hex::new(4, 0), ShipKind::Cruiser, 0)]);
        add_allied_strike_group(&mut world, 7, 2);
        strike_pulse(&mut world, Pulse::Morning);
        let nothing = found_nothing(&world);
        assert_eq!(nothing.len(), 2);
        // Group order: group 1 drops it; group 7 finds it gone.
        let dropped = not_there(&world);
        assert_eq!(dropped.len(), 1);
        assert_eq!(dropped[0].causes, vec![Cause::Event(nothing[0].id)]);
        assert!(allied_contacts(&world).is_empty());
    }

    /// Two strike groups of one side, from one carrier. Group 1 (range 250)
    /// is sent at contact 1, a carrier at the enemy's true hex (5,0), and
    /// attacks. Group 7 (range 100) can reach only contact 2, a cruiser at
    /// (2,0), where nothing is. Both contacts are fresh: group 1's report
    /// is applied first.
    fn two_groups_two_contacts(seed: u64, with_second_group: bool, contact_1_age: u32) -> World {
        let mut world = world(
            seed,
            vec![
                held(Hex::new(5, 0), ShipKind::Carrier, contact_1_age),
                held(Hex::new(2, 0), ShipKind::Cruiser, 0),
            ],
        );
        if with_second_group {
            add_allied_strike_group(&mut world, 7, 6);
        }
        world
    }

    #[test]
    fn an_attack_report_that_does_not_reach_the_contact_leaves_the_drop_in_place() {
        // (2,0) is three hexes from (5,0): too far for a report there to
        // merge with it (the figure is two).
        let mut world = two_groups_two_contacts(3, true, 0);
        strike_pulse(&mut world, Pulse::Morning);
        assert_eq!(attacks(&world).len(), 1);
        let dropped = not_there(&world);
        assert_eq!(dropped.len(), 1);
        assert_eq!(
            dropped[0].kind,
            EventKind::ContactNotThere {
                side: Side::Allied,
                contact: crate::ids::ContactId::new(2),
                hex: Hex::new(2, 0)
            }
        );
        // Contact 1 was refreshed and is still held.
        assert_eq!(allied_contacts(&world).len(), 1);
        assert_eq!(allied_contacts(&world)[0].0, 1);
        // The log: findings first, then group 1's damage and report, then
        // group 7's report that nothing is there.
        let last_attack = attacks(&world)[0].id;
        let report = events_where(&world, |k| matches!(k, EventKind::ReportReceived { .. }))
            .pop()
            .expect("a report");
        assert!(found_nothing(&world)[0].id > last_attack);
        assert!(report.id > last_attack);
        assert!(dropped[0].id > report.id);
    }

    #[test]
    fn an_attack_report_that_moved_the_contact_first_means_nothing_is_dropped() {
        // Wide merging (every hex a report can name is within four of
        // (2,0)), and contact 1 a pulse old and so too old to merge,
        // so group 1's report from (5,0) or next to it refreshes contact 2
        // and moves it off (2,0). Group 7's report that nothing is at (2,0)
        // then finds the contact somewhere else: the hex test decides, and
        // the contact stays.
        for seed in 1..=10 {
            let mut world = two_groups_two_contacts(seed, true, 1);
            let search = world.ref_data.search.as_mut().expect("search");
            search.merge_distance_hexes = 4;
            search.fresh_contact_max_age_pulses = 0;
            air_figures(&mut world).strike_contact_max_age_pulses = 1;
            strike_pulse(&mut world, Pulse::Morning);
            assert_eq!(launches(&world).len(), 2, "seed {seed}");
            assert_eq!(attacks(&world).len(), 1, "seed {seed}");
            assert_eq!(found_nothing(&world).len(), 1, "seed {seed}");
            assert!(not_there(&world).is_empty(), "seed {seed}");
            // Contact 2 is held, where the attack's report put it.
            let report = events_where(&world, |k| matches!(k, EventKind::ReportReceived { .. }))
                .pop()
                .expect("a report");
            let EventKind::ReportReceived { contact, hex, .. } = report.kind else {
                unreachable!()
            };
            assert_eq!(contact.raw(), 2, "seed {seed}");
            assert!(allied_contacts(&world).contains(&(2, hex)), "seed {seed}");
            assert_ne!(hex, Hex::new(2, 0));
        }
    }

    #[test]
    fn the_report_that_nothing_is_there_draws_no_random_number() {
        // Every stream is keyed by group and force, so a draw by the empty
        // strike could not move another's. What would show a draw is a
        // change to anything else: group 1's attack, its damage and its
        // report are the same with and without the empty strike of group 7.
        for seed in 1..=10 {
            let mut alone = two_groups_two_contacts(seed, false, 0);
            let mut with_empty = two_groups_two_contacts(seed, true, 0);
            strike_pulse(&mut alone, Pulse::Morning);
            strike_pulse(&mut with_empty, Pulse::Morning);
            let kinds = |world: &World| -> Vec<EventKind> {
                events_where(world, |k| {
                    matches!(
                        k,
                        EventKind::AirAttack { .. }
                            | EventKind::ShipDamaged { .. }
                            | EventKind::SystemHit { .. }
                            | EventKind::ReportReceived { .. }
                    )
                })
                .into_iter()
                .map(|event| event.kind)
                .collect()
            };
            assert_eq!(kinds(&alone), kinds(&with_empty), "seed {seed}");
            assert!(not_there(&alone).is_empty());
            assert_eq!(not_there(&with_empty).len(), 1);
            // The only difference in belief is the dropped contact.
            assert_eq!(allied_contacts(&alone).len(), 2);
            assert_eq!(allied_contacts(&with_empty).len(), 1);
            for id in 1..=4 {
                assert_eq!(
                    ship_of(&alone, id).flotation_damage,
                    ship_of(&with_empty, id).flotation_damage
                );
                assert_eq!(
                    ship_of(&alone, id).fire_damage,
                    ship_of(&with_empty, id).fire_damage
                );
            }
        }
    }

    // ---- randomness ----

    #[test]
    fn a_bomb_hit_is_a_system_hit_with_chance_points_times_the_figure() {
        // Decision 0033. A dive bomber's bomb is 100 x 60 / 600 = 10 points
        // on the carrier: chance 10 x 0.06 = 0.6, so with the flight deck
        // taking the whole table its band is the draws below 600,000. The
        // draws are one per shot, then track and system per
        // hit. Replaying the stream counts the system hits.
        for seed in 1..=25 {
            let mut world = world_with_a_good_contact(seed);
            air_figures(&mut world).bomb_system_hit =
                [(ShipSystem::FlightDeck, Fixed::ONE)].into_iter().collect();
            assert_eq!(
                surface_figures(&mut world).system_hit_chance_per_point,
                fixed(6, 100)
            );
            strike_pulse(&mut world, Pulse::Morning);
            let mut stream = world.rng_pair(
                1,
                Some(Pulse::Morning),
                Subsystem::AirCombat,
                EntityRef::AirGroup(BOMBERS),
                EntityRef::TaskForce(FORCE_B),
            );
            let hits: u32 = (0..18)
                .map(|_| u32::from(stream.chance(fixed(2, 10))))
                .sum();
            let mut expected = 0;
            for _ in 0..hits {
                stream.below(1_000_000);
                expected += u32::from(stream.below(1_000_000) < 600_000);
            }
            assert_eq!(
                system_hits(&world).len(),
                usize::try_from(expected).expect("small"),
                "seed {seed}"
            );
        }
    }

    #[test]
    fn a_torpedo_hit_from_the_air_always_hits_a_system_at_the_figure_of_the_file() {
        // 100 x 200 / 400 = 50 points x 0.06 = 3, capped at 1: every hit.
        let mut world = world_with_a_good_contact(1);
        set_class(&mut world, 2, 1);
        group_mut(&mut world, BOMBERS).aircraft_type = AircraftTypeId::new(3);
        air_figures(&mut world).torpedo_hit_chance = fixed(1, 2);
        strike_pulse(&mut world, Pulse::Morning);
        let (_, _, _, _, _, _, _, hits) = attack_of(&attacks(&world)[0]);
        assert!(hits > 0);
        assert_eq!(system_hits(&world).len(), usize::try_from(hits).unwrap());
    }

    #[test]
    fn the_attack_stream_draws_each_shot_then_track_and_system_per_hit_then_the_report() {
        for seed in 1..=25 {
            let mut world = world_with_a_good_contact(seed);
            every_hit_is_a_flight_deck_hit(&mut world);
            strike_pulse(&mut world, Pulse::Morning);
            let (_, _, _, _, _, _, _, hits) = attack_of(&attacks(&world)[0]);

            let mut stream = world.rng_pair(
                1,
                Some(Pulse::Morning),
                Subsystem::AirCombat,
                EntityRef::AirGroup(BOMBERS),
                EntityRef::TaskForce(FORCE_B),
            );
            let expected_hits: u32 = (0..18)
                .map(|_| u32::from(stream.chance(fixed(2, 10))))
                .sum();
            assert_eq!(hits, expected_hits, "seed {seed}");
            let mut flotation = 0;
            let mut fire = 0;
            for _ in 0..expected_hits {
                if stream.below(1_000_000) < 750_000 {
                    flotation += 1;
                } else {
                    fire += 1;
                }
                // The system draw: every hit is on the flight deck here.
                assert!(stream.below(1_000_000) < 1_000_000);
            }
            let damaged = events_where(&world, |k| matches!(k, EventKind::ShipDamaged { .. }));
            let counted = |wanted: DamageTrack| -> u32 {
                damaged
                    .iter()
                    .map(|e| match e.kind {
                        EventKind::ShipDamaged { track, hits, .. } if track == wanted => hits,
                        _ => 0,
                    })
                    .sum()
            };
            assert_eq!(counted(DamageTrack::Flotation), flotation, "seed {seed}");
            assert_eq!(counted(DamageTrack::Fire), fire, "seed {seed}");

            // The report is the next three draws of the same stream.
            let report = build_sighting_report(
                &mut stream,
                &world.map,
                Hex::new(5, 0),
                3,
                ShipKind::Carrier,
                world.ref_data.search.as_ref().expect("search"),
                EventId::new(0),
            );
            let received = events_where(&world, |k| matches!(k, EventKind::ReportReceived { .. }));
            let EventKind::ReportReceived {
                hex,
                ship_count,
                kind,
                ..
            } = received.last().expect("a report").kind
            else {
                unreachable!()
            };
            assert_eq!(
                (hex, ship_count, kind),
                (
                    report.estimated_hex,
                    report.estimated_ship_count,
                    report.estimated_kind
                ),
                "seed {seed}"
            );
        }
    }

    #[test]
    fn recording_breakdowns_changes_neither_the_outcome_nor_the_hash() {
        let run = |record: bool, seed: u64| {
            let mut world = two_sided(seed);
            world.set_config(crate::world::SimConfig {
                record_breakdowns: record,
            });
            strike_pulse(&mut world, Pulse::Morning);
            world.age_contacts_at_end_of_pulse(1, Pulse::Morning);
            strike_pulse(&mut world, Pulse::Afternoon);
            world.age_contacts_at_end_of_pulse(1, Pulse::Afternoon);
            let breakdowns = attacks(&world)
                .iter()
                .filter(|e| e.breakdown.is_some())
                .count();
            (
                world.state_hash(),
                world.events().len(),
                breakdowns,
                attacks(&world).len(),
            )
        };
        for seed in 1..=10 {
            let on = run(true, seed);
            let off = run(false, seed);
            assert_eq!((on.0, on.1), (off.0, off.1), "seed {seed}");
            // Every attack carries its breakdown when they are on, and none
            // when off (how many attacks there are depends on the hits).
            assert!(on.3 >= 2, "seed {seed}");
            assert_eq!((on.2, off.2), (on.3, 0), "seed {seed}");
        }
    }

    // ---- orders ----

    fn mission_order(group: AirGroupId, mission: AirMission) -> OrderRequest {
        OrderRequest {
            kind: OrderKind::AirGroup {
                target: group,
                order: AirGroupOrder::SetAirMission(mission),
            },
            origin: Origin::PLAYER,
        }
    }

    fn rejections(world: &World) -> Vec<RejectReason> {
        events_where(world, |k| matches!(k, EventKind::OrderRejected { .. }))
            .iter()
            .map(|e| match &e.kind {
                EventKind::OrderRejected { reason, .. } => reason.clone(),
                _ => unreachable!(),
            })
            .collect()
    }

    #[test]
    fn the_strike_mission_is_rejected_without_air_combat_figures_or_for_a_type_that_attacks_with_nothing(
    ) {
        // Accepted for a bomber with the figures.
        let mut world = self::world(1, vec![]);
        group_mut(&mut world, BOMBERS).mission = AirMission::Rest;
        world.submit_orders(
            Side::Allied,
            vec![mission_order(BOMBERS, AirMission::NavalStrike)],
        );
        world.resolve_day().expect("day resolves");
        assert!(rejections(&world).is_empty());
        assert_eq!(
            group_mut(&mut world, BOMBERS).mission,
            AirMission::NavalStrike
        );

        // Rejected with no figures, and the mission is unchanged.
        let mut world = self::world(1, vec![]);
        group_mut(&mut world, BOMBERS).mission = AirMission::Rest;
        world.ref_data.air_combat = None;
        world.submit_orders(
            Side::Allied,
            vec![mission_order(BOMBERS, AirMission::NavalStrike)],
        );
        world.resolve_day().expect("day resolves");
        assert_eq!(rejections(&world), vec![RejectReason::NoAirCombatFigures]);
        assert_eq!(group_mut(&mut world, BOMBERS).mission, AirMission::Rest);

        // Rejected for scouts, whose type attacks with nothing, and for a
        // bomber type that attacks with nothing. Other missions are fine.
        let mut world = self::world(1, vec![]);
        group_mut(&mut world, BOMBERS).aircraft_type = AircraftTypeId::new(4);
        group_mut(&mut world, BOMBERS).mission = AirMission::Rest;
        world.submit_orders(
            Side::Allied,
            vec![
                mission_order(SCOUTS, AirMission::NavalStrike),
                mission_order(BOMBERS, AirMission::NavalStrike),
                mission_order(SCOUTS, AirMission::Search),
            ],
        );
        world.resolve_day().expect("day resolves");
        assert_eq!(
            rejections(&world),
            vec![
                RejectReason::AircraftAttackWithNothing,
                RejectReason::AircraftAttackWithNothing
            ]
        );
        assert_eq!(group_mut(&mut world, SCOUTS).mission, AirMission::Search);
        assert_eq!(group_mut(&mut world, BOMBERS).mission, AirMission::Rest);
        let text = events_where(&world, |k| matches!(k, EventKind::OrderRejected { .. }))[0]
            .kind
            .to_string();
        assert!(text.contains("attacks with nothing"), "{text}");
    }

    #[test]
    fn a_full_day_launches_and_attacks_in_the_daylight_pulses_only() {
        let mut world = world_with_a_good_contact(2);
        world.resolve_day().expect("day resolves");
        for event in launches(&world).iter().chain(&attacks(&world)) {
            assert_ne!(event.pulse, Some(Pulse::Night));
            assert_eq!(event.day, 1);
        }
        // The contact made before day 1 was aged once by the night pulse, so
        // it is a pulse old in the morning: still struck.
        assert!(!launches(&world).is_empty());
        assert!(world.strikes_in_flight.is_empty());
    }

    // ---- fighters, escorts, patrols and guns (decision 0027) ----
    //
    // Fighter type 5 and short-range bomber type 6 are in the fixture. A
    // fighter group is aboard a ship (or at a base) on combat air patrol with
    // a share. Most tests make every fighter shot hit, so that the hits are
    // the shots and the allocation can be read off.

    const PATROL_A: AirGroupId = AirGroupId::new(5);
    const PATROL_B: AirGroupId = AirGroupId::new(10);

    fn cap_group(world: &mut World, id: u32, side: Side, ship: u32, ready: u32, share: Fixed) {
        let mut fighters = group(AirGroupId::new(id), side, ship, 5, ready);
        fighters.mission = AirMission::CombatAirPatrol;
        fighters.patrol_share = share;
        world.truth.air_groups.insert(fighters.id, fighters);
    }

    fn sure_fighters(world: &mut World) {
        air_figures(world).fighter_hit_chance = Fixed::ONE;
    }

    fn set_guns(world: &mut World, class_id: u32, shots: u32) {
        world
            .ref_data
            .ship_classes
            .get_mut(&ShipClassId::new(class_id))
            .expect("class")
            .anti_aircraft = shots;
    }

    fn ready_of(world: &World, id: u32) -> u32 {
        world.truth.air_groups[&AirGroupId::new(id)].ready
    }

    fn escorts_sent(world: &World) -> Vec<Event> {
        events_where(world, |k| matches!(k, EventKind::EscortsSent { .. }))
    }

    fn air_fights(world: &World) -> Vec<Event> {
        events_where(world, |k| matches!(k, EventKind::AirFight { .. }))
    }

    fn gun_fires(world: &World) -> Vec<Event> {
        events_where(world, |k| matches!(k, EventKind::AntiAircraftFire { .. }))
    }

    fn losses(world: &World) -> Vec<Event> {
        events_where(world, |k| matches!(k, EventKind::AircraftLost { .. }))
    }

    /// (firer, target, firing, at, shots, hits) of an `AirFight`.
    fn fight_of(event: &Event) -> (u32, u32, AirFirer, AirTarget, i64, u32) {
        match event.kind {
            EventKind::AirFight {
                firer,
                target,
                firing,
                at,
                shots,
                hits,
                ..
            } => (firer.raw(), target.raw(), firing, at, shots.trunc(), hits),
            _ => panic!("not a fight"),
        }
    }

    /// (task force, air group, shots, hits) of an `AntiAircraftFire`.
    fn guns_of(event: &Event) -> (u32, u32, i64, u32) {
        match event.kind {
            EventKind::AntiAircraftFire {
                task_force,
                air_group,
                shots,
                hits,
                ..
            } => (task_force.raw(), air_group.raw(), shots.trunc(), hits),
            _ => panic!("not a fire"),
        }
    }

    /// (air group, aircraft, remaining) of an `AircraftLost`.
    fn loss_of(event: &Event) -> (u32, u32, u32) {
        match event.kind {
            EventKind::AircraftLost {
                air_group,
                aircraft,
                remaining,
            } => (air_group.raw(), aircraft, remaining),
            _ => panic!("not a loss"),
        }
    }

    fn escort_of(event: &Event) -> (u32, u32, u32) {
        match event.kind {
            EventKind::EscortsSent {
                air_group,
                with_air_group,
                aircraft,
            } => (air_group.raw(), with_air_group.raw(), aircraft),
            _ => panic!("not escorts"),
        }
    }

    /// The standard world with Allied fighters (group 5, aboard ship 1) of
    /// `own` aircraft at share `own_share`, Japanese fighters (group 10,
    /// aboard ship 2) of `enemy` aircraft at share 1, and every fighter
    /// shot a hit. The bombers are group 1, 18 strong, striking (5,0).
    fn with_fighters(seed: u64, own: u32, own_share: Fixed, enemy: u32) -> World {
        let mut world = world_with_a_good_contact(seed);
        if own > 0 {
            cap_group(&mut world, 5, Side::Allied, 1, own, own_share);
        }
        if enemy > 0 {
            cap_group(&mut world, 10, Side::Japanese, 2, enemy, Fixed::ONE);
        }
        sure_fighters(&mut world);
        world
    }

    #[test]
    fn a_fighter_group_keeps_its_share_rounded_down_as_patrol_and_sends_the_rest() {
        let split = |ready: u32, share: Fixed| {
            let mut world = world_with_a_good_contact(1);
            cap_group(&mut world, 5, Side::Allied, 1, ready, share);
            let groups = world.split_fighter_groups();
            assert_eq!(groups.len(), 1);
            (groups[0].patrol, groups[0].escorts)
        };
        assert_eq!(split(12, fixed(1, 2)), (6, 6));
        // Rounded down: half of 7 is 3, so 3 stay and 4 go.
        assert_eq!(split(7, fixed(1, 2)), (3, 4));
        assert_eq!(split(12, Fixed::ZERO), (0, 12));
        assert_eq!(split(12, Fixed::ONE), (12, 0));
        assert_eq!(split(10, fixed(1, 3)), (3, 7));
        assert_eq!(split(10, fixed(7, 10)), (7, 3));
        assert_eq!(split(1, fixed(99, 100)), (0, 1));

        // The flight deck limits what can be flown before the split.
        let on_deck = |state: SystemState| {
            let mut world = world_with_a_good_contact(1);
            cap_group(&mut world, 5, Side::Allied, 1, 12, fixed(1, 2));
            deck(&mut world, 1, state);
            let groups = world.split_fighter_groups();
            (groups[0].patrol, groups[0].escorts)
        };
        assert_eq!(on_deck(SystemState::Working), (6, 6));
        assert_eq!(on_deck(SystemState::Degraded), (3, 3));
        assert_eq!(on_deck(SystemState::Out), (0, 0));

        // From a base all of them fly. Only groups on patrol with aircraft
        // are split.
        let mut world = world_with_a_good_contact(1);
        world.truth.bases.insert(
            crate::ids::BaseId::new(1),
            crate::state::Base {
                id: crate::ids::BaseId::new(1),
                name: "Test base".to_string(),
                side: Side::Allied,
                hex: Hex::new(0, 0),
                supply_t: Fixed::ZERO,
                anti_aircraft: 0,
                aircraft_repairs_per_day: 0,
                fuel_t: Fixed::ZERO,
            },
        );
        deck(&mut world, 1, SystemState::Out);
        cap_group(&mut world, 5, Side::Allied, 1, 12, fixed(1, 2));
        group_mut(&mut world, PATROL_A).location =
            AirGroupLocation::Base(crate::ids::BaseId::new(1));
        cap_group(&mut world, 6, Side::Allied, 1, 0, fixed(1, 2));
        let groups = world.split_fighter_groups();
        assert_eq!(groups.len(), 1, "a group with no aircraft is not split");
        assert_eq!((groups[0].patrol, groups[0].escorts), (6, 6));
        assert_eq!(groups[0].hex, Some(Hex::new(0, 0)));
        // The bombers and the scouts are not fighters on patrol.
        assert!(groups.iter().all(|g| g.air_group == PATROL_A));
    }

    #[test]
    fn strikes_from_one_place_at_one_hex_are_one_raid_in_order_of_their_first_group() {
        let strike = |group: u32, origin: AirGroupLocation, hex: Hex| StrikeInFlight {
            air_group: AirGroupId::new(group),
            side: Side::Allied,
            origin,
            aim: Aim::Contact(ContactId::new(1)),
            hex,
            aircraft: 1,
            launched: EventId::new(group),
            taken_for: None,
        };
        let ship_1 = AirGroupLocation::Ship(ShipId::new(1));
        let base_1 = AirGroupLocation::Base(crate::ids::BaseId::new(1));
        let (here, there) = (Hex::new(5, 0), Hex::new(2, 0));
        let raids = group_raids(&[
            strike(1, ship_1, here),
            strike(2, ship_1, there),
            strike(3, ship_1, here),
            strike(4, base_1, here),
            strike(5, base_1, there),
        ]);
        let groups: Vec<Vec<u32>> = raids
            .iter()
            .map(|raid| raid.strikes.iter().map(|s| s.air_group.raw()).collect())
            .collect();
        // Another place at the same hex, or another hex from the same
        // place, is another raid.
        assert_eq!(groups, vec![vec![1, 3], vec![2], vec![4], vec![5]]);
        assert!(raids.iter().all(|raid| raid.escorts.is_empty()));
    }

    /// Adds a short-range bomber group 6 aboard ship 1 on the strike
    /// mission, and a cruiser contact 80 nm out that only it can reach.
    /// Group 1 keeps flying at the carrier at (5,0).
    fn two_raids_from_one_carrier(own_fighters: u32, short_ready: u32) -> World {
        let mut world = self::world(
            1,
            vec![
                held(Hex::new(5, 0), ShipKind::Carrier, 0),
                held(Hex::new(2, 0), ShipKind::Cruiser, 0),
            ],
        );
        let mut short = group(AirGroupId::new(6), Side::Allied, 1, 6, short_ready);
        short.mission = AirMission::NavalStrike;
        world.truth.air_groups.insert(short.id, short);
        cap_group(&mut world, 5, Side::Allied, 1, own_fighters, Fixed::ZERO);
        world
    }

    #[test]
    fn the_escorts_at_a_place_go_with_the_raid_with_the_most_bombers_then_the_lowest_group() {
        let with = |short_ready: u32| {
            let mut world = two_raids_from_one_carrier(6, short_ready);
            strike_pulse(&mut world, Pulse::Morning);
            assert_eq!(launches(&world).len(), 2);
            let sent = escorts_sent(&world);
            assert_eq!(sent.len(), 1);
            escort_of(&sent[0])
        };
        // 18 bombers against 6: group 1's raid.
        assert_eq!(with(6), (5, 1, 6));
        // 18 against 20: group 6's raid.
        assert_eq!(with(20), (5, 6, 6));
        // A tie: the raid whose first group has the lower number, group 1.
        assert_eq!(with(18), (5, 1, 6));

        // And when group 1 is the short-range one, the tie goes to it.
        let mut world = two_raids_from_one_carrier(6, 18);
        group_mut(&mut world, BOMBERS).aircraft_type = AircraftTypeId::new(6);
        group_mut(&mut world, AirGroupId::new(6)).aircraft_type = AircraftTypeId::new(2);
        strike_pulse(&mut world, Pulse::Morning);
        assert_eq!(escort_of(&escorts_sent(&world)[0]), (5, 1, 6));

        // All the fighter groups at the place go with the one raid, each
        // with its own escorts, in group order.
        let mut world = two_raids_from_one_carrier(6, 6);
        cap_group(&mut world, 7, Side::Allied, 1, 9, fixed(1, 2));
        strike_pulse(&mut world, Pulse::Morning);
        let sent: Vec<_> = escorts_sent(&world).iter().map(escort_of).collect();
        assert_eq!(sent, vec![(5, 1, 6), (7, 1, 5)]);
        assert_eq!(world.split_fighter_groups()[1].patrol, 4);
    }

    #[test]
    fn a_place_with_no_raid_sends_no_escorts_and_a_group_with_no_escorts_sends_none() {
        // Fighters aboard another Allied carrier that launches no strike.
        let mut world = world_with_a_good_contact(1);
        add_force(&mut world, 3, Side::Allied, Hex::new(0, 1), &[(6, 2)]);
        cap_group(&mut world, 7, Side::Allied, 6, 12, Fixed::ZERO);
        // Group 5 aboard the striking carrier keeps everything overhead.
        cap_group(&mut world, 5, Side::Allied, 1, 12, Fixed::ONE);
        strike_pulse(&mut world, Pulse::Morning);
        assert_eq!(launches(&world).len(), 1);
        assert!(escorts_sent(&world).is_empty());
        // Nothing fires: the enemy has no patrol.
        assert!(air_fights(&world).is_empty());
        assert_eq!(ready_of(&world, 7), 12);

        // No strike at all: no escorts, even with fighters ready.
        let mut world = self::world(1, vec![]);
        cap_group(&mut world, 5, Side::Allied, 1, 12, Fixed::ZERO);
        strike_pulse(&mut world, Pulse::Morning);
        assert!(escorts_sent(&world).is_empty());
    }

    #[test]
    fn each_fire_step_has_the_shots_decision_6_gives_it() {
        // (escorts, patrol) -> the fights in order: (firer, target, firing,
        // at, shots).
        type Fight = (u32, u32, AirFirer, AirTarget, i64);
        let fights = |own: u32, own_share: Fixed, enemy: u32| -> Vec<Fight> {
            let mut world = with_fighters(1, own, own_share, enemy);
            strike_pulse(&mut world, Pulse::Morning);
            air_fights(&world)
                .iter()
                .map(|e| {
                    let (a, b, c, d, e, _) = fight_of(e);
                    (a, b, c, d, e)
                })
                .collect()
        };
        use AirFirer::{Escorts, Patrol};
        use AirTarget as T;
        // 6 escorts, 12 patrol: the patrol fires 6 at the escorts and the
        // other 6 at the bombers.
        assert_eq!(
            fights(12, fixed(1, 2), 12),
            vec![
                (5, 10, Escorts, T::Patrol, 6),
                (10, 5, Patrol, T::Escorts, 6),
                (10, 1, Patrol, T::Bombers, 6)
            ]
        );
        // 6 escorts, 3 patrol: the patrol has fewer shots than escorts, so
        // all 3 go at the escorts and none at the bombers.
        assert_eq!(
            fights(12, fixed(1, 2), 3),
            vec![
                (5, 10, Escorts, T::Patrol, 6),
                (10, 5, Patrol, T::Escorts, 3)
            ]
        );
        // As many escorts as patrol: none at the bombers.
        assert_eq!(
            fights(12, fixed(1, 2), 6),
            vec![
                (5, 10, Escorts, T::Patrol, 6),
                (10, 5, Patrol, T::Escorts, 6)
            ]
        );
        // No escorts: the whole patrol at the bombers, and no escort fire.
        assert_eq!(
            fights(12, Fixed::ONE, 12),
            vec![(10, 1, Patrol, T::Bombers, 12)]
        );
        assert_eq!(
            fights(0, Fixed::ONE, 12),
            vec![(10, 1, Patrol, T::Bombers, 12)]
        );
        // No patrol: the escorts have nobody to fire at.
        assert!(fights(12, fixed(1, 2), 0).is_empty());
        // A patrol of one aircraft kept by a share that rounds to none.
        assert_eq!(
            fights(12, Fixed::ZERO, 1),
            vec![
                (5, 10, Escorts, T::Patrol, 12),
                (10, 5, Patrol, T::Escorts, 1)
            ]
        );
    }

    #[test]
    fn the_events_of_a_fight_read_cite_their_causes_and_come_in_a_fixed_order() {
        // 12 escorts and 12 patrol, every shot a hit, 8 shots of guns.
        let mut world = with_fighters(1, 12, Fixed::ZERO, 12);
        set_guns(&mut world, 2, 8);
        let (mission_order, share_order) = (OrderId::new(70), OrderId::new(71));
        group_mut(&mut world, PATROL_A).mission_order = Some(mission_order);
        group_mut(&mut world, PATROL_A).share_order = Some(share_order);
        air_figures(&mut world).anti_aircraft_hit_chance = Fixed::ONE;
        strike_pulse(&mut world, Pulse::Morning);
        let launch = launches(&world)[0].id;
        let sent = escorts_sent(&world);
        assert_eq!(sent.len(), 1);
        assert_eq!(
            sent[0].kind.to_string(),
            "air_group:5 sent 12 escorts with air_group:1"
        );
        assert_eq!(
            sent[0].causes,
            vec![
                Cause::Order(mission_order),
                Cause::Order(share_order),
                Cause::Event(launch)
            ]
        );
        let fights = air_fights(&world);
        assert_eq!(fights.len(), 2);
        assert_eq!(
            fights[0].kind.to_string(),
            "air_group:5 escorts at the patrol of air_group:10: 12 shots x 1 = 12 expected, 12 hits"
        );
        // Every fighter shot hits here (chance 1). With the figure of the
        // file, 0.3, 12 shots expect 3.6 hits and each is rolled.
        let mut world_text = with_fighters(1, 12, Fixed::ZERO, 12);
        air_figures(&mut world_text).fighter_hit_chance = fixed(3, 10);
        strike_pulse(&mut world_text, Pulse::Morning);
        let text = air_fights(&world_text)[0].kind.to_string();
        let mut stream = world_text.rng_pair(
            1,
            Some(Pulse::Morning),
            Subsystem::AirCombat,
            EntityRef::AirGroup(PATROL_A),
            EntityRef::TaskForce(FORCE_B),
        );
        let replayed: u32 = (0..12)
            .map(|_| u32::from(stream.chance(fixed(3, 10))))
            .sum();
        assert_eq!(
            text,
            format!(
                "air_group:5 escorts at the patrol of air_group:10: 12 shots x 0.3 = 3.6 expected, {replayed} hits"
            )
        );
        // Both fights with escorts cite the escorts and the launch.
        for fight in &fights {
            assert_eq!(
                fight.causes,
                vec![Cause::Event(launch), Cause::Event(sent[0].id)]
            );
        }
        assert_eq!(
            fights[1].kind.to_string(),
            "air_group:10 patrol at the escorts of air_group:5: 12 shots x 1 = 12 expected, 12 hits"
        );
        // The guns, at the 18 bombers (all of them left: no patrol shots
        // at the bombers when escorts match the patrol).
        let guns = gun_fires(&world);
        assert_eq!(guns.len(), 1);
        assert_eq!(
            guns[0].kind.to_string(),
            "task_force:2 guns at the raid of air_group:1: 8 shots x 1 = 8 expected, 8 hits"
        );
        assert_eq!(guns[0].causes, vec![Cause::Event(launch)]);
        // Losses: one per group and cause, citing that cause.
        let lost: Vec<(u32, u32, u32)> = losses(&world).iter().map(loss_of).collect();
        assert_eq!(lost, vec![(10, 12, 0), (5, 12, 0), (1, 8, 10)]);
        let loss_events = losses(&world);
        assert_eq!(loss_events[0].causes, vec![Cause::Event(fights[0].id)]);
        assert_eq!(loss_events[1].causes, vec![Cause::Event(fights[1].id)]);
        assert_eq!(loss_events[2].causes, vec![Cause::Event(guns[0].id)]);
        assert_eq!(
            loss_events[2].kind.to_string(),
            "air_group:1 lost 8 aircraft, 10 left"
        );
        // The attack is of the 10 bombers left and cites their loss.
        let attack = attacks(&world)[0].clone();
        assert_eq!(attack_of(&attack).4, Fixed::from_int(10));
        assert_eq!(
            attack.causes,
            vec![Cause::Event(launch), Cause::Event(loss_events[2].id)]
        );
        // The fixed order: launch, escorts, fights, guns, losses, attack,
        // then the damage and the report.
        let ids = [
            launch,
            sent[0].id,
            fights[0].id,
            fights[1].id,
            guns[0].id,
            loss_events[0].id,
            loss_events[1].id,
            loss_events[2].id,
            attack.id,
        ];
        assert!(ids.windows(2).all(|pair| pair[0] < pair[1]), "{ids:?}");
        let first_damage = events_where(&world, |k| matches!(k, EventKind::ShipDamaged { .. }))
            .first()
            .map(|e| e.id);
        assert!(!first_damage.is_some_and(|id| id < attack.id));
        // The counts after.
        assert_eq!(ready_of(&world, 1), 10);
        assert_eq!((ready_of(&world, 5), ready_of(&world, 10)), (0, 0));
        // Fights and guns carry breakdowns (breakdowns are on).
        for (index, fight) in fights.iter().enumerate() {
            let breakdown = fight.breakdown.as_ref().expect("breakdown");
            let names: Vec<&str> = breakdown.factors.iter().map(|f| f.name).collect();
            if index == 0 {
                // The escorts' fire does not depend on the share.
                assert_eq!(names, vec!["shots", "hit_chance"]);
            } else {
                // The patrol's fire says what share of the patrol it is.
                assert_eq!(
                    names,
                    vec![
                        "patrol",
                        "raid_aircraft",
                        "all_raids_aircraft",
                        "patrol_share",
                        "shots",
                        "hit_chance"
                    ]
                );
                let values: Vec<i64> = breakdown.factors[..4]
                    .iter()
                    .map(|f| f.value.trunc())
                    .collect();
                // 12 patrol, a raid of 12 escorts and 18 bombers and no other.
                assert_eq!(values, vec![12, 30, 30, 12]);
            }
        }
        assert_eq!(
            guns[0].breakdown.as_ref().expect("breakdown").result,
            Fixed::from_int(8)
        );
    }

    #[test]
    fn hits_on_bombers_are_taken_from_the_lowest_group_first_and_those_beyond_are_wasted() {
        // A second bomber group 6 aboard ship 1 (type 2, 4 aircraft) on the
        // strike mission: group 1 has 18, group 6 has 4, one raid. The
        // patrol of `patrol` aircraft, every shot a hit, fires them all at
        // the bombers.
        let run = |patrol: u32| {
            let mut world = with_fighters(1, 0, Fixed::ONE, patrol);
            let mut second = group(AirGroupId::new(6), Side::Allied, 1, 2, 4);
            second.mission = AirMission::NavalStrike;
            world.truth.air_groups.insert(second.id, second);
            strike_pulse(&mut world, Pulse::Morning);
            assert_eq!(launches(&world).len(), 2);
            world
        };
        let summary = |world: &World| {
            (
                losses(world).iter().map(loss_of).collect::<Vec<_>>(),
                attacks(world)
                    .iter()
                    .map(|e| (attack_of(e).0, attack_of(e).4.trunc()))
                    .collect::<Vec<_>>(),
            )
        };
        // 5 hits: all from group 1, which keeps 13; group 6 is whole.
        let world = run(5);
        assert_eq!(fight_of(&air_fights(&world)[0]).5, 5);
        assert_eq!(summary(&world), (vec![(1, 5, 13)], vec![(1, 13), (6, 4)]));
        // 20 hits: group 1's 18, then 2 of group 6's 4.
        let world = run(20);
        assert_eq!(summary(&world), (vec![(1, 18, 0), (6, 2, 2)], vec![(6, 2)]));
        // 30 hits, 4 wasted: both groups wiped out, nobody attacks, and
        // the side hears nothing.
        let world = run(30);
        assert_eq!(fight_of(&air_fights(&world)[0]).5, 30);
        assert_eq!(summary(&world), (vec![(1, 18, 0), (6, 4, 0)], vec![]));
        assert!(attacks(&world).is_empty());
        assert!(found_nothing(&world).is_empty());
        assert!(events_where(&world, |k| matches!(k, EventKind::ShipDamaged { .. })).is_empty());
        assert_eq!(ready_of(&world, 1) + ready_of(&world, 6), 0);
        // Guns continue where the fighters stopped: 8 patrol hits leave
        // group 1 with 10 and group 6 with 4, and 14 gun hits (every shot
        // a hit) take 10 and then 4.
        let mut world = with_fighters(1, 0, Fixed::ONE, 8);
        let mut second = group(AirGroupId::new(6), Side::Allied, 1, 2, 4);
        second.mission = AirMission::NavalStrike;
        world.truth.air_groups.insert(second.id, second);
        set_guns(&mut world, 1, 7);
        air_figures(&mut world).anti_aircraft_hit_chance = Fixed::ONE;
        strike_pulse(&mut world, Pulse::Morning);
        // 7 + 7 from the two cruisers (ships 3 and 4), and the carrier has none.
        assert_eq!(guns_of(&gun_fires(&world)[0]), (2, 1, 14, 14));
        assert_eq!(
            summary(&world),
            (vec![(1, 8, 10), (1, 10, 0), (6, 4, 0)], vec![])
        );
    }

    #[test]
    fn hits_on_a_patrol_and_on_escorts_are_taken_from_their_groups_in_order() {
        // Two Japanese patrol groups aboard ship 2: group 10 with 4, group
        // 11 with 3. Allied escorts, group 5, 10 aircraft, every shot a hit.
        let mut world = with_fighters(1, 10, Fixed::ZERO, 4);
        cap_group(&mut world, 11, Side::Japanese, 2, 3, Fixed::ONE);
        strike_pulse(&mut world, Pulse::Morning);
        let fights: Vec<_> = air_fights(&world).iter().map(fight_of).collect();
        // The patrol is 7: 10 escort shots, 7 patrol shots at the escorts,
        // none at the bombers.
        assert_eq!(
            fights,
            vec![
                (5, 10, AirFirer::Escorts, AirTarget::Patrol, 10, 10),
                (10, 5, AirFirer::Patrol, AirTarget::Escorts, 7, 7)
            ]
        );
        // 10 hits on a patrol of 7: group 10's 4, group 11's 3, 3 wasted.
        // 7 hits on 10 escorts: all from group 5.
        let lost: Vec<_> = losses(&world).iter().map(loss_of).collect();
        assert_eq!(lost, vec![(10, 4, 0), (11, 3, 0), (5, 7, 3)]);
        assert_eq!(fight_of(&air_fights(&world)[0]).4, 10);

        // Fewer hits than the patrol: 5 escorts, 5 hits: group 10 loses 4
        // and group 11 loses 1.
        let mut world = with_fighters(1, 5, Fixed::ZERO, 4);
        cap_group(&mut world, 11, Side::Japanese, 2, 3, Fixed::ONE);
        strike_pulse(&mut world, Pulse::Morning);
        let lost: Vec<_> = losses(&world).iter().map(loss_of).collect();
        // The patrol's 7 shots: 5 at the escorts, and the 2 over go at the
        // bombers.
        assert_eq!(lost, vec![(10, 4, 0), (11, 1, 2), (5, 5, 0), (1, 2, 16)]);

        // Two escort groups lose in order too: 6 and 4 aircraft, 7 hits
        // from a patrol of 7.
        let mut world = with_fighters(1, 6, Fixed::ZERO, 7);
        cap_group(&mut world, 6, Side::Allied, 1, 4, Fixed::ZERO);
        strike_pulse(&mut world, Pulse::Morning);
        let lost: Vec<_> = losses(&world).iter().map(loss_of).collect();
        // The escorts' 10 shots take the whole patrol of 7 (group 10), the
        // patrol's 7 take group 5's 6 and one of group 6's 4.
        assert_eq!(lost, vec![(10, 7, 0), (5, 6, 0), (6, 1, 3)]);
    }

    #[test]
    fn the_patrol_over_the_hex_is_every_enemy_group_on_patrol_there_ships_and_bases_alike() {
        let mut world = with_fighters(1, 12, Fixed::ZERO, 4);
        // A Japanese base at the target hex with 3 fighters on patrol, and
        // one at another hex that is no part of it.
        for (id, hex) in [(1, Hex::new(5, 0)), (2, Hex::new(6, 0))] {
            world.truth.bases.insert(
                crate::ids::BaseId::new(id),
                crate::state::Base {
                    id: crate::ids::BaseId::new(id),
                    name: format!("Test base {id}"),
                    side: Side::Japanese,
                    hex,
                    supply_t: Fixed::ZERO,
                    anti_aircraft: 0,
                    aircraft_repairs_per_day: 0,
                    fuel_t: Fixed::ZERO,
                },
            );
        }
        for (id, base, ready) in [(11, 1, 3), (12, 2, 9)] {
            let mut fighters = group(AirGroupId::new(id), Side::Japanese, 2, 5, ready);
            fighters.location = AirGroupLocation::Base(crate::ids::BaseId::new(base));
            fighters.mission = AirMission::CombatAirPatrol;
            world.truth.air_groups.insert(fighters.id, fighters);
        }
        // An Allied group that happens to be there is not the enemy's.
        cap_group(&mut world, 13, Side::Allied, 1, 9, Fixed::ONE);
        // A Japanese group not on patrol is not a patrol.
        let mut resting = group(AirGroupId::new(14), Side::Japanese, 2, 5, 9);
        resting.mission = AirMission::Rest;
        world.truth.air_groups.insert(resting.id, resting);
        strike_pulse(&mut world, Pulse::Morning);
        let fights: Vec<_> = air_fights(&world).iter().map(fight_of).collect();
        // Patrol: group 10's 4 aboard ship 2 and group 11's 3 at the base
        // in the hex: 7. Escorts: 12, whose shots take all of it.
        assert_eq!(
            fights[0],
            (5, 10, AirFirer::Escorts, AirTarget::Patrol, 12, 12)
        );
        assert_eq!(
            fights[1],
            (10, 5, AirFirer::Patrol, AirTarget::Escorts, 7, 7)
        );
        assert_eq!(fights.len(), 2);
        assert_eq!(ready_of(&world, 10) + ready_of(&world, 11), 0);
        assert_eq!(ready_of(&world, 12), 9);
        assert_eq!(ready_of(&world, 13), 9);
        assert_eq!(ready_of(&world, 14), 9);
    }

    #[test]
    fn the_guns_count_only_the_ships_afloat_in_the_force_at_the_start_of_the_step() {
        let shots_with = |sink: Option<u32>| {
            let mut world = world_with_a_good_contact(1);
            // The carrier (ship 2) has 20, each cruiser (ships 3 and 4) 8.
            set_guns(&mut world, 2, 20);
            set_guns(&mut world, 1, 8);
            if let Some(ship) = sink {
                ship_mut(&mut world, ship).location = ShipLocation::Sunk;
                world
                    .truth
                    .task_forces
                    .get_mut(&FORCE_B)
                    .expect("force")
                    .ships
                    .remove(&ShipId::new(ship));
            }
            strike_pulse(&mut world, Pulse::Morning);
            gun_fires(&world).first().map(guns_of)
        };
        let all = shots_with(None).expect("the guns fire");
        assert_eq!((all.0, all.1, all.2), (2, 1, 36));
        // A sunk cruiser fires nothing: 28.
        assert_eq!(shots_with(Some(4)).expect("fire").2, 28);
        // No ship with guns: no roll and no event.
        let mut world = world_with_a_good_contact(1);
        strike_pulse(&mut world, Pulse::Morning);
        assert!(gun_fires(&world).is_empty());
        assert!(losses(&world).is_empty());
        // 36 shots at 0.05, each rolled: the hits are what 36 draws give, and
        // the bombers lost are the hits.
        let mut seen = std::collections::BTreeSet::new();
        for seed in 1..=40 {
            let mut world = world_with_a_good_contact(seed);
            set_guns(&mut world, 2, 20);
            set_guns(&mut world, 1, 8);
            strike_pulse(&mut world, Pulse::Morning);
            let (_, _, shots, hits) = guns_of(&gun_fires(&world)[0]);
            assert_eq!(shots, 36);
            let mut stream = world.rng_pair(
                1,
                Some(Pulse::Morning),
                Subsystem::AirCombat,
                EntityRef::Ship(ShipId::new(2)),
                EntityRef::AirGroup(BOMBERS),
            );
            let replayed: u32 = (0..36)
                .map(|_| u32::from(stream.chance(fixed(5, 100))))
                .sum();
            assert_eq!(hits, replayed, "seed {seed}");
            seen.insert(hits);
            assert_eq!(ready_of(&world, 1), 18 - hits);
            assert_eq!(
                attack_of(&attacks(&world)[0]).4,
                Fixed::from_int(i64::from(18 - hits))
            );
        }
        assert!(seen.len() > 2, "{seen:?}");
    }

    /// An Allied carrier (ship 6, task force 3 at (0,1)) with 18 bombers
    /// (group 7) that also strike (5,0): a second raid, from another place.
    fn with_a_second_raid(world: &mut World) {
        add_force(world, 3, Side::Allied, Hex::new(0, 1), &[(6, 2)]);
        let mut bombers = group(AirGroupId::new(7), Side::Allied, 6, 2, 18);
        bombers.mission = AirMission::NavalStrike;
        world.truth.air_groups.insert(bombers.id, bombers);
    }

    #[test]
    fn two_raids_in_one_pulse_divide_the_patrol_and_each_meet_the_guns_of_the_start_of_the_step() {
        let mut world = with_fighters(1, 6, Fixed::ZERO, 8);
        with_a_second_raid(&mut world);
        cap_group(&mut world, 8, Side::Allied, 6, 6, Fixed::ZERO);
        // The carrier of force 2 is so fragile that the first raid sinks it;
        // the second raid's guns still count its 20 shots.
        set_guns(&mut world, 2, 20);
        set_guns(&mut world, 1, 8);
        world
            .ref_data
            .ship_classes
            .get_mut(&ShipClassId::new(2))
            .expect("class")
            .toughness = Fixed::from_int(10);
        air_figures(&mut world).bomb_hit_chance = Fixed::ONE;
        air_figures(&mut world).anti_aircraft_hit_chance = Fixed::ZERO;
        strike_pulse(&mut world, Pulse::Morning);
        assert_eq!(launches(&world).len(), 2);

        // Both raids have 24 aircraft (6 escorts and 18 bombers), so each
        // meets 4 of the patrol of 8. Each raid's escorts fire 6 shots, and
        // its share fires 4, all at the escorts: none is left for the
        // bombers.
        let fights: Vec<_> = air_fights(&world).iter().map(fight_of).collect();
        assert_eq!(
            fights,
            vec![
                (5, 10, AirFirer::Escorts, AirTarget::Patrol, 6, 6),
                (10, 5, AirFirer::Patrol, AirTarget::Escorts, 4, 4),
                (8, 10, AirFirer::Escorts, AirTarget::Patrol, 6, 6),
                (10, 8, AirFirer::Patrol, AirTarget::Escorts, 4, 4),
            ]
        );
        // 12 hits on a patrol of 8, added up: the first raid's 6 come off
        // group 10, and the second raid's 6 are cut to the 2 that are left.
        let lost: Vec<_> = losses(&world).iter().map(loss_of).collect();
        assert_eq!(lost, vec![(10, 6, 2), (5, 4, 2), (10, 2, 0), (8, 4, 2)]);
        assert_eq!(ready_of(&world, 10), 0);
        // Both raids' guns: 36 shots each (chance 0), at the 18 bombers.
        let guns: Vec<_> = gun_fires(&world).iter().map(guns_of).collect();
        assert_eq!(guns, vec![(2, 1, 36, 0), (2, 7, 36, 0)]);
        // Both attacked with 18, though the first sank the carrier.
        let shots: Vec<_> = attacks(&world)
            .iter()
            .map(|e| (attack_of(e).0, attack_of(e).4.trunc()))
            .collect();
        assert_eq!(shots, vec![(1, 18), (7, 18)]);
        assert_eq!(ship_of(&world, 2).location, ShipLocation::Sunk);
    }

    fn gid(id: u32) -> AirGroupId {
        AirGroupId::new(id)
    }

    #[test]
    fn the_patrol_divides_in_proportion_rounded_down_with_the_leftover_to_the_largest() {
        // One raid meets all of it, and none, and a patrol of none.
        assert_eq!(divide_patrol(12, &[(30, gid(1))]), vec![12]);
        assert_eq!(divide_patrol(0, &[(30, gid(1)), (17, gid(2))]), vec![0, 0]);
        assert_eq!(divide_patrol(5, &[]), Vec::<u32>::new());
        // The hand calculation of the Americans: 6 over 17 and 30 aircraft
        // is 2 (2.17) and 3 (3.83), and the leftover one goes to the larger.
        assert_eq!(divide_patrol(6, &[(17, gid(1)), (30, gid(2))]), vec![2, 4]);
        // The Japanese: 54 over 33 and 32 is 27 (27.4) and 26 (26.6), and
        // the leftover one goes to the larger.
        assert_eq!(
            divide_patrol(54, &[(33, gid(1)), (32, gid(2))]),
            vec![28, 26]
        );
        // The same raids the other way round: the shares follow the raids.
        assert_eq!(
            divide_patrol(54, &[(32, gid(2)), (33, gid(1))]),
            vec![26, 28]
        );
        // A tie in size: the leftover goes to the lower first group, in
        // whatever order the raids are given.
        assert_eq!(divide_patrol(5, &[(10, gid(3)), (10, gid(2))]), vec![2, 3]);
        assert_eq!(divide_patrol(5, &[(10, gid(2)), (10, gid(3))]), vec![3, 2]);
        // Fewer fighters than raids: one each to the largest first, the
        // rest none.
        assert_eq!(
            divide_patrol(2, &[(10, gid(1)), (30, gid(2)), (20, gid(3)), (5, gid(4))]),
            vec![0, 1, 1, 0]
        );
        assert_eq!(
            divide_patrol(1, &[(20, gid(1)), (20, gid(2)), (20, gid(3))]),
            vec![1, 0, 0]
        );
    }

    #[test]
    fn the_shares_add_up_to_the_patrol_and_a_bigger_raid_never_has_a_smaller_share() {
        let sizes = [1u32, 2, 3, 7, 17, 30, 32, 33, 60];
        for patrol in 0..=60u32 {
            for &a in &sizes {
                for &b in &sizes {
                    for &c in &[0u32, 5, 33] {
                        let raids = if c == 0 {
                            vec![(a, gid(1)), (b, gid(2))]
                        } else {
                            vec![(a, gid(1)), (b, gid(2)), (c, gid(3))]
                        };
                        let shares = divide_patrol(patrol, &raids);
                        assert_eq!(shares.iter().sum::<u32>(), patrol, "{patrol} {raids:?}");
                        for (i, x) in raids.iter().enumerate() {
                            for (j, y) in raids.iter().enumerate() {
                                if x.0 > y.0 {
                                    assert!(shares[i] >= shares[j], "{patrol} {raids:?}");
                                }
                            }
                        }
                        // Taken in the opposite order, each raid gets the
                        // same share.
                        let mut reversed = raids.clone();
                        reversed.reverse();
                        let mut back = divide_patrol(patrol, &reversed);
                        back.reverse();
                        assert_eq!(shares, back, "{patrol} {raids:?}");
                    }
                }
            }
        }
    }

    #[test]
    fn two_raids_of_unequal_size_fire_the_shots_their_shares_give_in_either_order() {
        let run = |reversed: bool| {
            // Raid A (group 1 and 6 escorts): 24 aircraft. Raid B (group 7,
            // 6 bombers and the 6 escorts of group 8): 12 aircraft. A
            // patrol of 9 is divided 6 and 3.
            let mut world = with_fighters(1, 6, Fixed::ZERO, 9);
            with_a_second_raid(&mut world);
            group_mut(&mut world, gid(7)).ready = 6;
            cap_group(&mut world, 8, Side::Allied, 6, 6, Fixed::ZERO);
            set_guns(&mut world, 2, 0);
            set_guns(&mut world, 1, 0);
            world.decide_strikes(1, Pulse::Morning);
            if reversed {
                world.strikes_in_flight.reverse();
            }
            world.resolve_strikes(1, Pulse::Morning);
            world
        };
        let world = run(false);
        let mut fights: Vec<_> = air_fights(&world).iter().map(fight_of).collect();
        assert_eq!(
            fights,
            vec![
                (5, 10, AirFirer::Escorts, AirTarget::Patrol, 6, 6),
                (10, 5, AirFirer::Patrol, AirTarget::Escorts, 6, 6),
                (8, 10, AirFirer::Escorts, AirTarget::Patrol, 6, 6),
                (10, 8, AirFirer::Patrol, AirTarget::Escorts, 3, 3),
            ]
        );
        // 12 hits on a patrol of 9: 9 are lost, 3 wasted.
        assert_eq!(ready_of(&world, 10), 0);
        assert_eq!(ready_of(&world, 5), 0);
        assert_eq!(ready_of(&world, 8), 3);

        // Taken the other way round, the shots, the losses and the
        // outcome are the same.
        let other = run(true);
        fights.sort();
        let mut fights_other: Vec<_> = air_fights(&other).iter().map(fight_of).collect();
        fights_other.sort();
        assert_eq!(fights, fights_other);
        for id in [1, 5, 7, 8, 10] {
            assert_eq!(ready_of(&world, id), ready_of(&other, id), "group {id}");
        }
        assert_eq!(format!("{:?}", world.truth), format!("{:?}", other.truth));
        assert_eq!(world.events.len(), other.events.len());
    }

    #[test]
    fn a_raid_alone_meets_the_whole_patrol_and_a_raid_that_finds_nothing_takes_no_share() {
        // One raid: the whole patrol of 8 fires, 6 at the escorts and 2 at
        // the bombers (the same as before the patrol was divided).
        let mut world = with_fighters(1, 6, Fixed::ZERO, 8);
        strike_pulse(&mut world, Pulse::Morning);
        let fights: Vec<_> = air_fights(&world).iter().map(fight_of).collect();
        assert_eq!(
            fights,
            vec![
                (5, 10, AirFirer::Escorts, AirTarget::Patrol, 6, 6),
                (10, 5, AirFirer::Patrol, AirTarget::Escorts, 6, 6),
                (10, 1, AirFirer::Patrol, AirTarget::Bombers, 2, 2),
            ]
        );
        // A second raid on a hex where nothing is takes no share: the
        // first still meets the whole patrol.
        let mut world = with_fighters(1, 6, Fixed::ZERO, 8);
        add_force(&mut world, 3, Side::Allied, Hex::new(0, 1), &[(6, 2)]);
        let mut bombers = group(AirGroupId::new(7), Side::Allied, 6, 2, 18);
        bombers.mission = AirMission::NavalStrike;
        world.truth.air_groups.insert(bombers.id, bombers);
        world.decide_strikes(1, Pulse::Morning);
        for strike in world.strikes_in_flight.iter_mut() {
            if strike.air_group == AirGroupId::new(7) {
                strike.hex = Hex::new(-4, 0);
            }
        }
        world.resolve_strikes(1, Pulse::Morning);
        assert_eq!(found_nothing(&world).len(), 1);
        let fights: Vec<_> = air_fights(&world).iter().map(fight_of).collect();
        assert_eq!(fights.len(), 3);
        assert_eq!(fights[1].4, 6);
        assert_eq!(fights[2].4, 2);
    }

    #[test]
    fn a_raid_that_finds_nothing_meets_nobody_and_a_patrol_over_a_hex_nobody_attacks_fires_nothing()
    {
        // The contact names (4,0), where the enemy is not. Fighters on both
        // sides, and guns: nothing fires, nobody is lost.
        let mut world = self::world(1, vec![held(Hex::new(4, 0), ShipKind::Cruiser, 0)]);
        cap_group(&mut world, 5, Side::Allied, 1, 12, Fixed::ZERO);
        cap_group(&mut world, 10, Side::Japanese, 2, 12, Fixed::ONE);
        sure_fighters(&mut world);
        set_guns(&mut world, 2, 20);
        strike_pulse(&mut world, Pulse::Morning);
        assert_eq!(found_nothing(&world).len(), 1);
        // The escorts went and found nothing: they are recorded, and no
        // more.
        assert_eq!(escorts_sent(&world).len(), 1);
        assert!(air_fights(&world).is_empty());
        assert!(gun_fires(&world).is_empty());
        assert!(losses(&world).is_empty());
        assert!(attacks(&world).is_empty());
        assert_eq!(ready_of(&world, 5), 12);
        assert_eq!(ready_of(&world, 10), 12);
        assert_eq!(ready_of(&world, 1), 18);

        // Fighters that sit over a hex where the raid finds only its own
        // side's ships, or nothing, fire nothing either: a Japanese base
        // with fighters at (3,0), a Japanese patrol there, and a strike at
        // (3,0).
        let mut world = self::world(1, vec![held(Hex::new(3, 0), ShipKind::Cruiser, 0)]);
        world.truth.bases.insert(
            crate::ids::BaseId::new(1),
            crate::state::Base {
                id: crate::ids::BaseId::new(1),
                name: "Test base".to_string(),
                side: Side::Japanese,
                hex: Hex::new(3, 0),
                supply_t: Fixed::ZERO,
                anti_aircraft: 0,
                aircraft_repairs_per_day: 0,
                fuel_t: Fixed::ZERO,
            },
        );
        let mut fighters = group(AirGroupId::new(10), Side::Japanese, 2, 5, 12);
        fighters.location = AirGroupLocation::Base(crate::ids::BaseId::new(1));
        fighters.mission = AirMission::CombatAirPatrol;
        world.truth.air_groups.insert(fighters.id, fighters);
        sure_fighters(&mut world);
        strike_pulse(&mut world, Pulse::Morning);
        assert_eq!(found_nothing(&world).len(), 1);
        assert!(air_fights(&world).is_empty());
        assert!(losses(&world).is_empty());
        assert_eq!(ready_of(&world, 10), 12);
    }

    #[test]
    fn a_raid_with_no_patrol_no_escorts_and_no_guns_attacks_as_a_strike_did_before_fighters() {
        // The plain world and the same world with fighters that take no
        // part (resting, patrolling elsewhere, a patrol of another hex)
        // produce the same attacks, damage and hash.
        for seed in 1..=30 {
            let mut plain = world_with_a_good_contact(seed);
            strike_pulse(&mut plain, Pulse::Morning);
            plain.age_contacts_at_end_of_pulse(1, Pulse::Morning);

            let mut other = world_with_a_good_contact(seed);
            let mut resting = group(AirGroupId::new(10), Side::Japanese, 2, 5, 12);
            resting.mission = AirMission::Rest;
            other.truth.air_groups.insert(resting.id, resting);
            let mut elsewhere = group(AirGroupId::new(11), Side::Japanese, 2, 5, 12);
            elsewhere.mission = AirMission::CombatAirPatrol;
            elsewhere.patrol_share = Fixed::ONE;
            // Aboard a carrier in force 3 at another hex.
            add_force(&mut other, 3, Side::Japanese, Hex::new(5, 3), &[(9, 2)]);
            elsewhere.location = AirGroupLocation::Ship(ShipId::new(9));
            other.truth.air_groups.insert(elsewhere.id, elsewhere);
            strike_pulse(&mut other, Pulse::Morning);
            other.age_contacts_at_end_of_pulse(1, Pulse::Morning);

            assert_eq!(
                attacks(&plain)
                    .iter()
                    .map(|e| (e.kind.clone(), e.causes.clone()))
                    .collect::<Vec<_>>(),
                attacks(&other)
                    .iter()
                    .map(|e| (e.kind.clone(), e.causes.clone()))
                    .collect::<Vec<_>>(),
                "seed {seed}"
            );
            for id in 1..=4 {
                assert_eq!(
                    ship_of(&plain, id).flotation_damage,
                    ship_of(&other, id).flotation_damage
                );
                assert_eq!(
                    ship_of(&plain, id).fire_damage,
                    ship_of(&other, id).fire_damage
                );
                assert_eq!(ship_of(&plain, id).systems, ship_of(&other, id).systems);
            }
            assert!(losses(&other).is_empty());
            assert!(air_fights(&other).is_empty());
            assert!(escorts_sent(&other).is_empty());
            assert_eq!(ready_of(&other, 1), 18);
        }
    }

    #[test]
    fn the_rolls_come_from_the_streams_the_decision_names() {
        // 6 escorts and 12 patrol at 0.3 and 36 gun shots at 0.05 (decision
        // 0033: each shot is a draw, one after another from the stream).
        let draws = |stream: &mut RngStream, shots: u32, chance: Fixed| -> u32 {
            (0..shots).map(|_| u32::from(stream.chance(chance))).sum()
        };
        for seed in 1..=25 {
            let mut world = world_with_a_good_contact(seed);
            cap_group(&mut world, 5, Side::Allied, 1, 12, fixed(1, 2));
            cap_group(&mut world, 10, Side::Japanese, 2, 12, Fixed::ONE);
            set_guns(&mut world, 2, 20);
            set_guns(&mut world, 1, 8);
            strike_pulse(&mut world, Pulse::Morning);
            let fights: Vec<_> = air_fights(&world).iter().map(fight_of).collect();
            assert_eq!(fights.len(), 3);
            assert_eq!((fights[0].4, fights[1].4, fights[2].4), (6, 6, 6));
            let key = |a: EntityRef, b: EntityRef| {
                world.rng_pair(1, Some(Pulse::Morning), Subsystem::AirCombat, a, b)
            };
            // Escorts: the first escort group and the task force attacked.
            let mut stream = key(EntityRef::AirGroup(PATROL_A), EntityRef::TaskForce(FORCE_B));
            assert_eq!(
                fights[0].5,
                draws(&mut stream, 6, fixed(3, 10)),
                "seed {seed}"
            );
            // Patrol: the first patrol group and the raid's first group,
            // the escorts first and then the bombers.
            let mut stream = key(EntityRef::AirGroup(PATROL_B), EntityRef::AirGroup(BOMBERS));
            assert_eq!(
                fights[1].5,
                draws(&mut stream, 6, fixed(3, 10)),
                "seed {seed}"
            );
            assert_eq!(
                fights[2].5,
                draws(&mut stream, 6, fixed(3, 10)),
                "seed {seed}"
            );
            // Guns: the lowest ship afloat in the force and the raid's
            // first group.
            let mut stream = key(
                EntityRef::Ship(ShipId::new(2)),
                EntityRef::AirGroup(BOMBERS),
            );
            let (_, _, shots, hits) = guns_of(&gun_fires(&world)[0]);
            assert_eq!(shots, 36);
            assert_eq!(hits, draws(&mut stream, 36, fixed(5, 100)), "seed {seed}");
            // The attack's own stream is untouched: it is keyed as in 0026
            // and draws one for each bomber left first.
            let left = u32::try_from(attack_of(&attacks(&world)[0]).4.trunc()).expect("small");
            let mut stream = key(EntityRef::AirGroup(BOMBERS), EntityRef::TaskForce(FORCE_B));
            assert_eq!(
                attack_of(&attacks(&world)[0]).7,
                draws(&mut stream, left, fixed(2, 10)),
                "seed {seed}"
            );
        }
    }

    #[test]
    fn a_patrol_with_no_escorts_to_fire_at_rolls_once_at_the_bombers_on_its_own_stream() {
        // The patrol of 1 meets no escorts: one roll at the bombers, and
        // no roll at the escorts, so the bombers' roll is the first draw
        // of the patrol's stream.
        let mut world = with_fighters(1, 0, Fixed::ONE, 1);
        air_figures(&mut world).fighter_hit_chance = fixed(1, 2);
        strike_pulse(&mut world, Pulse::Morning);
        let fights: Vec<_> = air_fights(&world).iter().map(fight_of).collect();
        // 1 shot at 0.5 expects 0.5: whole 0, so a draw at 0.5 decides.
        assert_eq!(fights.len(), 1);
        let mut stream = world.rng_pair(
            1,
            Some(Pulse::Morning),
            Subsystem::AirCombat,
            EntityRef::AirGroup(PATROL_B),
            EntityRef::AirGroup(BOMBERS),
        );
        assert_eq!(fights[0].5, u32::from(stream.chance(fixed(1, 2))));
    }

    #[test]
    fn the_flight_deck_limits_the_patrol_and_the_escorts_as_it_limits_everything_flown() {
        // Carrier A's deck is degraded: 9 bombers, and 12 fighters fly 6,
        // of which half (3) stay and 3 escort. Carrier B's deck is
        // degraded: 12 fighters fly 6, all patrol.
        let mut world = with_fighters(1, 12, fixed(1, 2), 12);
        deck(&mut world, 1, SystemState::Degraded);
        deck(&mut world, 2, SystemState::Degraded);
        strike_pulse(&mut world, Pulse::Morning);
        assert_eq!(escort_of(&escorts_sent(&world)[0]), (5, 1, 3));
        let fights: Vec<_> = air_fights(&world).iter().map(fight_of).collect();
        assert_eq!(
            fights,
            vec![
                (5, 10, AirFirer::Escorts, AirTarget::Patrol, 3, 3),
                (10, 5, AirFirer::Patrol, AirTarget::Escorts, 3, 3),
                (10, 1, AirFirer::Patrol, AirTarget::Bombers, 3, 3),
            ]
        );
        // The aircraft lost come off what is ready, deck or no deck.
        assert_eq!(ready_of(&world, 10), 9);
        assert_eq!(ready_of(&world, 5), 9);
        assert_eq!(ready_of(&world, 1), 15);
        // Carrier B's deck out: no patrol, so nothing fires at the escorts.
        let mut world = with_fighters(1, 12, fixed(1, 2), 12);
        deck(&mut world, 2, SystemState::Out);
        strike_pulse(&mut world, Pulse::Morning);
        assert!(air_fights(&world).is_empty());
    }

    // ---- the radius of escorts (decision 0030) ----

    fn set_fighter_radius(world: &mut World, radius_nm: i64) {
        world
            .ref_data
            .aircraft_types
            .get_mut(&AircraftTypeId::new(5))
            .expect("fighter type")
            .strike_range_nm = Fixed::from_int(radius_nm);
    }

    /// (patrol, escorts) of the one fighter group after `send_escorts`, and
    /// the raids' escorts, for a raid of `bombers` from `origin` at `hex`.
    fn sent_to(
        world: &World,
        origin: AirGroupLocation,
        raids_at: &[(u32, u32, Hex)],
    ) -> (u32, u32, Vec<Vec<usize>>) {
        let strikes: Vec<StrikeInFlight> = raids_at
            .iter()
            .map(|(group, bombers, hex)| StrikeInFlight {
                air_group: AirGroupId::new(*group),
                side: Side::Allied,
                origin,
                aim: Aim::Contact(ContactId::new(1)),
                hex: *hex,
                aircraft: *bombers,
                launched: EventId::new(*group),
                taken_for: None,
            })
            .collect();
        let mut raids = group_raids(&strikes);
        let mut fighters = world.split_fighter_groups();
        send_escorts(&mut raids, &mut fighters);
        (
            fighters[0].patrol,
            fighters[0].escorts,
            raids.iter().map(|raid| raid.escorts.clone()).collect(),
        )
    }

    const SHIP_1: AirGroupLocation = AirGroupLocation::Ship(ShipId::new(1));

    #[test]
    fn escorts_go_at_exactly_the_radius_and_not_one_hex_beyond() {
        // The carrier is at (0,0); the raid's hex (5,0) is 200 nm off.
        for (radius, goes) in [
            (250, true),
            (200, true),
            (199, false),
            (160, false),
            (0, false),
        ] {
            let mut world = with_fighters(1, 12, fixed(1, 2), 12);
            set_fighter_radius(&mut world, radius);
            let (patrol, escorts, raids) = sent_to(&world, SHIP_1, &[(1, 18, Hex::new(5, 0))]);
            if goes {
                assert_eq!((patrol, escorts, raids), (6, 6, vec![vec![0]]));
            } else {
                assert_eq!((patrol, escorts, raids), (12, 0, vec![vec![]]));
            }
            strike_pulse(&mut world, Pulse::Morning);
            assert_eq!(escorts_sent(&world).len(), usize::from(goes), "{radius}");
        }
        // A hex nearer is within a radius of 160, a hex further is not.
        let mut world = with_fighters(1, 12, fixed(1, 2), 12);
        set_fighter_radius(&mut world, 160);
        assert_eq!(sent_to(&world, SHIP_1, &[(1, 18, Hex::new(4, 0))]).1, 6);
        assert_eq!(sent_to(&world, SHIP_1, &[(1, 18, Hex::new(5, 0))]).1, 0);
        // A radius of nothing never escorts.
        set_fighter_radius(&mut world, 0);
        assert_eq!(sent_to(&world, SHIP_1, &[(1, 18, Hex::new(1, 0))]).1, 0);
    }

    #[test]
    fn fighters_that_do_not_escort_are_all_patrol_and_meet_a_raid_on_their_hex() {
        // The Allied fighters (group 5, 12 aboard the carrier at (0,0), half
        // patrol) cannot reach the Allied strike at (5,0). The Japanese
        // bombers strike the Allied carrier's hex from (5,0), 200 nm.
        let raid_on_them = |radius: i64| {
            let mut world = world(
                1,
                vec![
                    held(Hex::new(5, 0), ShipKind::Cruiser, 0),
                    StartingContact {
                        side: Side::Japanese,
                        hex: Hex::new(0, 0),
                        ship_count: 3,
                        kind: ShipKind::Cruiser,
                        age_pulses: 0,
                    },
                ],
            );
            cap_group(&mut world, 5, Side::Allied, 1, 12, fixed(1, 2));
            group_mut(&mut world, ENEMY_BOMBERS).mission = AirMission::NavalStrike;
            sure_fighters(&mut world);
            set_fighter_radius(&mut world, radius);
            strike_pulse(&mut world, Pulse::Morning);
            world
        };
        let world = raid_on_them(160);
        assert!(escorts_sent(&world).is_empty());
        let fights: Vec<_> = air_fights(&world).iter().map(fight_of).collect();
        assert_eq!(
            fights,
            vec![(
                5,
                ENEMY_BOMBERS.raw(),
                AirFirer::Patrol,
                AirTarget::Bombers,
                12,
                12
            )]
        );
        // Within the radius half of them go: the patrol is 6.
        let world = raid_on_them(250);
        assert_eq!(escorts_sent(&world).len(), 1);
        let fights: Vec<_> = air_fights(&world).iter().map(fight_of).collect();
        assert_eq!(
            fights,
            vec![(
                5,
                ENEMY_BOMBERS.raw(),
                AirFirer::Patrol,
                AirTarget::Bombers,
                6,
                6
            )]
        );
    }

    #[test]
    fn the_radius_is_tested_against_the_raid_chosen_by_the_most_bombers_only() {
        // Group 1's raid (18 bombers) is at (5,0), 200 nm; group 6's (6, or
        // 20) is at (2,0), 80 nm. A radius of 160 reaches only the second.
        for (short_ready, goes) in [(6, false), (20, true)] {
            let mut world = two_raids_from_one_carrier(6, short_ready);
            set_fighter_radius(&mut world, 160);
            strike_pulse(&mut world, Pulse::Morning);
            assert_eq!(launches(&world).len(), 2);
            assert_eq!(
                escorts_sent(&world).len(),
                usize::from(goes),
                "{short_ready}"
            );
            let (patrol, escorts, raids) = sent_to(
                &world,
                SHIP_1,
                &[(1, 18, Hex::new(5, 0)), (6, short_ready, Hex::new(2, 0))],
            );
            if goes {
                assert_eq!((patrol, escorts, raids), (0, 6, vec![vec![], vec![0]]));
            } else {
                // No nearer raid is looked for.
                assert_eq!((patrol, escorts, raids), (6, 0, vec![vec![], vec![]]));
            }
        }
    }

    #[test]
    fn a_group_at_a_base_is_held_to_the_radius_too() {
        let base = AirGroupLocation::Base(crate::ids::BaseId::new(1));
        let mut world = world_with_a_good_contact(1);
        world.truth.bases.insert(
            crate::ids::BaseId::new(1),
            crate::state::Base {
                id: crate::ids::BaseId::new(1),
                name: "Test base".to_string(),
                side: Side::Allied,
                hex: Hex::new(0, 0),
                supply_t: Fixed::ZERO,
                anti_aircraft: 0,
                aircraft_repairs_per_day: 0,
                fuel_t: Fixed::ZERO,
            },
        );
        cap_group(&mut world, 5, Side::Allied, 1, 12, fixed(1, 2));
        group_mut(&mut world, PATROL_A).location = base;
        set_fighter_radius(&mut world, 200);
        assert_eq!(
            sent_to(&world, base, &[(1, 18, Hex::new(5, 0))]),
            (6, 6, vec![vec![0]])
        );
        set_fighter_radius(&mut world, 199);
        assert_eq!(
            sent_to(&world, base, &[(1, 18, Hex::new(5, 0))]),
            (12, 0, vec![vec![]])
        );
    }

    #[test]
    fn the_flight_deck_still_limits_a_patrol_that_stays_for_want_of_range() {
        let patrol_with = |state: SystemState| {
            let mut world = with_fighters(1, 12, fixed(1, 2), 12);
            set_fighter_radius(&mut world, 160);
            deck(&mut world, 1, state);
            let (patrol, escorts, _) = sent_to(&world, SHIP_1, &[(1, 18, Hex::new(5, 0))]);
            (patrol, escorts)
        };
        assert_eq!(patrol_with(SystemState::Working), (12, 0));
        assert_eq!(patrol_with(SystemState::Degraded), (6, 0));
        assert_eq!(patrol_with(SystemState::Out), (0, 0));
    }

    // ---- orders and the hash ----

    fn share_order(group: AirGroupId, share: Fixed) -> OrderRequest {
        OrderRequest {
            kind: OrderKind::AirGroup {
                target: group,
                order: AirGroupOrder::SetPatrolShare { share },
            },
            origin: Origin::PLAYER,
        }
    }

    #[test]
    fn combat_air_patrol_is_for_fighters_with_the_figures_and_the_share_is_checked() {
        let mut world = self::world(1, vec![]);
        cap_group(&mut world, 5, Side::Allied, 1, 12, Fixed::ONE);
        group_mut(&mut world, PATROL_A).mission = AirMission::Rest;
        group_mut(&mut world, PATROL_A).patrol_share = Fixed::ONE;
        world.submit_orders(
            Side::Allied,
            vec![
                mission_order(PATROL_A, AirMission::CombatAirPatrol),
                share_order(PATROL_A, fixed(1, 2)),
            ],
        );
        world.resolve_day().expect("day resolves");
        assert!(rejections(&world).is_empty());
        let fighters = &world.truth.air_groups[&PATROL_A];
        assert_eq!(fighters.mission, AirMission::CombatAirPatrol);
        assert_eq!(fighters.patrol_share, fixed(1, 2));
        // Each standing order is recorded on the group.
        assert!(fighters.mission_order.is_some() && fighters.share_order.is_some());
        assert_ne!(fighters.mission_order, fighters.share_order);

        // Bombers and scouts cannot patrol.
        let mut world = self::world(1, vec![]);
        world.submit_orders(
            Side::Allied,
            vec![
                mission_order(BOMBERS, AirMission::CombatAirPatrol),
                mission_order(SCOUTS, AirMission::CombatAirPatrol),
            ],
        );
        world.resolve_day().expect("day resolves");
        assert_eq!(
            rejections(&world),
            vec![RejectReason::NotAFighter, RejectReason::NotAFighter]
        );
        assert_eq!(group_mut(&mut world, SCOUTS).mission, AirMission::Rest);
        let text = events_where(&world, |k| matches!(k, EventKind::OrderRejected { .. }))[0]
            .kind
            .to_string();
        assert!(text.contains("not a fighter"), "{text}");

        // No figures, no patrol.
        let mut world = self::world(1, vec![]);
        cap_group(&mut world, 5, Side::Allied, 1, 12, Fixed::ONE);
        group_mut(&mut world, PATROL_A).mission = AirMission::Rest;
        world.ref_data.air_combat = None;
        world.submit_orders(
            Side::Allied,
            vec![mission_order(PATROL_A, AirMission::CombatAirPatrol)],
        );
        world.resolve_day().expect("day resolves");
        assert_eq!(rejections(&world), vec![RejectReason::NoAirCombatFigures]);

        // A fighter cannot be sent to strike.
        let mut world = self::world(1, vec![]);
        cap_group(&mut world, 5, Side::Allied, 1, 12, Fixed::ONE);
        world.submit_orders(
            Side::Allied,
            vec![mission_order(PATROL_A, AirMission::NavalStrike)],
        );
        world.resolve_day().expect("day resolves");
        assert_eq!(
            rejections(&world),
            vec![RejectReason::AircraftAttackWithNothing]
        );

        // The share is from 0 to 1, ends included, and is the enemy's to
        // set only for its own groups.
        let mut world = self::world(1, vec![]);
        cap_group(&mut world, 5, Side::Allied, 1, 12, Fixed::ONE);
        for share in [Fixed::ZERO, fixed(1, 2), Fixed::ONE] {
            world.submit_orders(Side::Allied, vec![share_order(PATROL_A, share)]);
            world.resolve_day().expect("day resolves");
            assert_eq!(world.truth.air_groups[&PATROL_A].patrol_share, share);
        }
        assert!(rejections(&world).is_empty());
        for bad in [fixed(-1, 10), fixed(11, 10), Fixed::from_int(2)] {
            world.submit_orders(Side::Allied, vec![share_order(PATROL_A, bad)]);
            world.resolve_day().expect("day resolves");
            assert_eq!(world.truth.air_groups[&PATROL_A].patrol_share, Fixed::ONE);
        }
        let rejected = rejections(&world);
        assert_eq!(rejected.len(), 3);
        assert_eq!(
            rejected[0],
            RejectReason::PatrolShareOutOfRange(fixed(-1, 10))
        );
        let text = events_where(&world, |k| matches!(k, EventKind::OrderRejected { .. }))[0]
            .kind
            .to_string();
        assert!(
            text.contains("patrol share -0.1 is not between 0 and 1"),
            "{text}"
        );
        world.submit_orders(Side::Japanese, vec![share_order(PATROL_A, Fixed::ZERO)]);
        world.resolve_day().expect("day resolves");
        assert_eq!(rejections(&world).last(), Some(&RejectReason::NotYourUnit));
        assert_eq!(world.truth.air_groups[&PATROL_A].patrol_share, Fixed::ONE);
    }

    #[test]
    fn the_patrol_share_is_state_and_the_default_leaves_the_hash_as_it_was() {
        let hash_with = |share: Fixed, order: Option<OrderId>| {
            let mut world = world_with_a_good_contact(1);
            cap_group(&mut world, 5, Side::Allied, 1, 12, share);
            group_mut(&mut world, PATROL_A).share_order = order;
            world.state_hash()
        };
        // The default writes nothing: the daily hashes of every scenario
        // built before the share existed are pinned by their golden files.
        let default = hash_with(Fixed::ONE, None);
        // Any other share changes it, and so does an order on the default.
        let half = hash_with(fixed(1, 2), None);
        let zero = hash_with(Fixed::ZERO, None);
        let ordered = hash_with(Fixed::ONE, Some(OrderId::new(9)));
        let hashes = [default, half, zero, ordered];
        for (i, a) in hashes.iter().enumerate() {
            for b in &hashes[i + 1..] {
                assert_ne!(a, b);
            }
        }
        assert_ne!(hash_with(fixed(1, 2), Some(OrderId::new(1))), half);
        // Which group has the share matters.
        let on = |first: Fixed, second: Fixed| {
            let mut world = world_with_a_good_contact(1);
            cap_group(&mut world, 5, Side::Allied, 1, 12, first);
            cap_group(&mut world, 6, Side::Allied, 1, 12, second);
            world.state_hash()
        };
        assert_ne!(on(fixed(1, 2), Fixed::ONE), on(Fixed::ONE, fixed(1, 2)));
        // The value is in the hash: shares a raw unit apart differ.
        assert_ne!(
            hash_with(Fixed::from_raw(500_000), None),
            hash_with(Fixed::from_raw(500_001), None)
        );
    }

    #[test]
    fn recording_breakdowns_changes_neither_a_fight_nor_the_hash() {
        let run = |record: bool, seed: u64| {
            let mut world = world_with_a_good_contact(seed);
            cap_group(&mut world, 5, Side::Allied, 1, 12, fixed(1, 2));
            cap_group(&mut world, 10, Side::Japanese, 2, 12, Fixed::ONE);
            set_guns(&mut world, 2, 20);
            set_guns(&mut world, 1, 8);
            world.set_config(crate::world::SimConfig {
                record_breakdowns: record,
            });
            strike_pulse(&mut world, Pulse::Morning);
            world.age_contacts_at_end_of_pulse(1, Pulse::Morning);
            let breakdowns = air_fights(&world)
                .iter()
                .chain(&gun_fires(&world))
                .filter(|e| e.breakdown.is_some())
                .count();
            (world.state_hash(), world.events().len(), breakdowns)
        };
        for seed in 1..=15 {
            let on = run(true, seed);
            let off = run(false, seed);
            assert_eq!((on.0, on.1), (off.0, off.1), "seed {seed}");
            assert_eq!((on.2, off.2), (4, 0));
        }
    }

    #[test]
    fn the_new_events_name_their_entities() {
        let mut world = with_fighters(1, 12, fixed(1, 2), 12);
        set_guns(&mut world, 2, 20);
        air_figures(&mut world).anti_aircraft_hit_chance = Fixed::ONE;
        strike_pulse(&mut world, Pulse::Morning);
        assert_eq!(
            escorts_sent(&world)[0].kind.entities(),
            vec![EntityRef::AirGroup(PATROL_A), EntityRef::AirGroup(BOMBERS)]
        );
        assert_eq!(
            air_fights(&world)[0].kind.entities(),
            vec![EntityRef::AirGroup(PATROL_A), EntityRef::AirGroup(PATROL_B)]
        );
        assert_eq!(
            gun_fires(&world)[0].kind.entities(),
            vec![EntityRef::TaskForce(FORCE_B), EntityRef::AirGroup(BOMBERS)]
        );
        assert_eq!(
            losses(&world)[0].kind.entities(),
            vec![EntityRef::AirGroup(AirGroupId::new(
                loss_of(&losses(&world)[0]).0
            ))]
        );
    }

    // ---- protection of the target type (decision 11 of 0029) ----

    /// Sets the two protection facts of aircraft type `type_id`.
    fn protect(world: &mut World, type_id: u32, armour: bool, tanks: bool) {
        let aircraft_type = world
            .ref_data
            .aircraft_types
            .get_mut(&AircraftTypeId::new(type_id))
            .expect("type");
        aircraft_type.pilot_armour = armour;
        aircraft_type.self_sealing_tanks = tanks;
    }

    /// Adds fighter type 7, a copy of type 5.
    fn add_fighter_type_7(world: &mut World) {
        let mut copy = world.ref_data.aircraft_types[&AircraftTypeId::new(5)].clone();
        copy.id = AircraftTypeId::new(7);
        world.ref_data.aircraft_types.insert(copy.id, copy);
    }

    /// (chance, expected, breakdown factor names and values) of an
    /// `AirFight` or an `AntiAircraftFire`.
    fn chance_of(event: &Event) -> (Fixed, Fixed) {
        match event.kind {
            EventKind::AirFight {
                chance, expected, ..
            }
            | EventKind::AntiAircraftFire {
                chance, expected, ..
            } => (chance, expected),
            _ => panic!("not a shot"),
        }
    }

    /// Fighters at their placeholder 0.3 and guns at 0.05, the figures of
    /// the file, and the protection factors of the file, 0.75.
    fn file_figures(world: &mut World) {
        air_figures(world).fighter_hit_chance = fixed(3, 10);
        air_figures(world).anti_aircraft_hit_chance = fixed(5, 100);
    }

    /// 12 escorts (group 5, type 5) against 12 patrol (group 10, type 5),
    /// eight gun shots, the figures of the file.
    fn protection_world(seed: u64) -> World {
        let mut world = with_fighters(seed, 12, Fixed::ZERO, 12);
        set_guns(&mut world, 2, 8);
        file_figures(&mut world);
        world
    }

    /// The chances of the three kinds of roll, in a fight with escorts:
    /// escorts at the patrol, patrol at the escorts, guns.
    fn three_chances(world: &mut World) -> [Fixed; 3] {
        strike_pulse(world, Pulse::Morning);
        let fights = air_fights(world);
        let guns = gun_fires(world);
        assert_eq!((fights.len(), guns.len()), (2, 1));
        [
            chance_of(&fights[0]).0,
            chance_of(&fights[1]).0,
            chance_of(&guns[0]).0,
        ]
    }

    #[test]
    fn protection_multiplies_the_chance_of_a_fighters_shot_and_a_gun_shot() {
        // Fighters of type 5 on both sides, bombers of type 2.
        let with = |armour: bool, tanks: bool| {
            let mut world = protection_world(1);
            protect(&mut world, 5, armour, tanks);
            protect(&mut world, 2, armour, tanks);
            three_chances(&mut world)
        };
        let (base, gun) = (fixed(3, 10), fixed(5, 100));
        assert_eq!(with(false, false), [base, base, gun]);
        // 0.3 x 0.75 = 0.225 and 0.05 x 0.75 = 0.0375.
        assert_eq!(
            with(true, false),
            [fixed(225, 1000), fixed(225, 1000), fixed(375, 10000)]
        );
        assert_eq!(
            with(false, true),
            [fixed(225, 1000), fixed(225, 1000), fixed(375, 10000)]
        );
        // 0.3 x 0.75 x 0.75 = 0.16875 and 0.05 x 0.75 x 0.75 = 0.028125.
        assert_eq!(
            with(true, true),
            [
                fixed(16875, 100000),
                fixed(16875, 100000),
                fixed(28125, 1000000)
            ]
        );
    }

    #[test]
    fn the_events_chance_and_expected_hits_are_the_modified_ones() {
        let mut world = protection_world(1);
        protect(&mut world, 5, true, true);
        protect(&mut world, 2, true, true);
        strike_pulse(&mut world, Pulse::Morning);
        let fight = &air_fights(&world)[0];
        // 12 shots x 0.16875 = 2.025 expected.
        assert_eq!(chance_of(fight), (fixed(16875, 100000), fixed(2025, 1000)));
        assert!(fight
            .kind
            .to_string()
            .contains("12 shots x 0.16875 = 2.025 expected"));
        // 8 shots x 0.028125 = 0.225 expected.
        let fire = &gun_fires(&world)[0];
        assert_eq!(chance_of(fire), (fixed(28125, 1000000), fixed(225, 1000)));
        assert!(fire
            .kind
            .to_string()
            .contains("8 shots x 0.028125 = 0.225 expected"));
    }

    #[test]
    fn the_target_pools_first_group_sets_the_factor_for_each_roll() {
        // Escorts group 5 (type 5), patrol group 10. Patrol of type 7 with
        // armour: escorts firing at it use 0.225; the patrol firing at the
        // unprotected escorts uses 0.3.
        let mut world = protection_world(1);
        add_fighter_type_7(&mut world);
        group_mut(&mut world, AirGroupId::new(10)).aircraft_type = AircraftTypeId::new(7);
        protect(&mut world, 7, true, false);
        let [escorts_at_patrol, patrol_at_escorts, _] = three_chances(&mut world);
        assert_eq!(escorts_at_patrol, fixed(225, 1000));
        assert_eq!(patrol_at_escorts, fixed(3, 10));
        // The other way round: the escorts are the armoured type.
        let mut world = protection_world(1);
        add_fighter_type_7(&mut world);
        group_mut(&mut world, PATROL_A).aircraft_type = AircraftTypeId::new(7);
        protect(&mut world, 7, true, false);
        let [escorts_at_patrol, patrol_at_escorts, _] = three_chances(&mut world);
        assert_eq!(escorts_at_patrol, fixed(3, 10));
        assert_eq!(patrol_at_escorts, fixed(225, 1000));

        // A raid of two bomber groups of different types: the first group's
        // type sets the factor for the patrol and the guns, in both orders.
        let bomber_chances = |first_type: u32, second_type: u32| {
            // No escorts, so the whole patrol fires at the bombers.
            let mut world = with_fighters(1, 12, Fixed::ONE, 12);
            set_guns(&mut world, 2, 8);
            file_figures(&mut world);
            let mut second = group(AirGroupId::new(6), Side::Allied, 1, second_type, 18);
            second.mission = AirMission::NavalStrike;
            world.truth.air_groups.insert(second.id, second);
            group_mut(&mut world, BOMBERS).aircraft_type = AircraftTypeId::new(first_type);
            // Type 2 is armoured and has tanks, type 3 has nothing.
            protect(&mut world, 2, true, true);
            strike_pulse(&mut world, Pulse::Morning);
            assert_eq!(launches(&world).len(), 2);
            let fights = air_fights(&world);
            let guns = gun_fires(&world);
            assert_eq!((fights.len(), guns.len()), (1, 1));
            assert_eq!(guns_of(&guns[0]).1, 1, "the guns name the first group");
            (chance_of(&fights[0]).0, chance_of(&guns[0]).0)
        };
        assert_eq!(
            bomber_chances(2, 3),
            (fixed(16875, 100000), fixed(28125, 1000000))
        );
        assert_eq!(bomber_chances(3, 2), (fixed(3, 10), fixed(5, 100)));
    }

    #[test]
    fn breakdowns_list_the_base_chance_and_each_factor_applied() {
        let names = |armour: bool, tanks: bool| {
            let mut world = protection_world(1);
            world.set_config(crate::world::SimConfig {
                record_breakdowns: true,
            });
            protect(&mut world, 5, armour, tanks);
            protect(&mut world, 2, armour, tanks);
            strike_pulse(&mut world, Pulse::Morning);
            let breakdown = |event: &Event| {
                let b = event.breakdown.clone().expect("a breakdown");
                (
                    b.factors
                        .iter()
                        .map(|f| (f.name, f.value))
                        .collect::<Vec<_>>(),
                    b.result,
                )
            };
            (
                breakdown(&air_fights(&world)[0]),
                breakdown(&gun_fires(&world)[0]),
            )
        };
        let (fight, gun) = names(false, false);
        assert_eq!(
            fight.0,
            vec![("shots", Fixed::from_int(12)), ("hit_chance", fixed(3, 10))]
        );
        assert_eq!(fight.1, fixed(36, 10));
        assert_eq!(gun.0.len(), 2);
        let (fight, gun) = names(true, true);
        assert_eq!(
            fight.0,
            vec![
                ("shots", Fixed::from_int(12)),
                ("hit_chance", fixed(3, 10)),
                ("pilot_armour_factor", fixed(3, 4)),
                ("self_sealing_tanks_factor", fixed(3, 4)),
            ]
        );
        assert_eq!(fight.1, fixed(2025, 1000));
        assert_eq!(
            gun.0,
            vec![
                ("shots", Fixed::from_int(8)),
                ("hit_chance", fixed(5, 100)),
                ("pilot_armour_factor", fixed(3, 4)),
                ("self_sealing_tanks_factor", fixed(3, 4)),
            ]
        );
        assert_eq!(gun.1, fixed(225, 1000));
        let (fight, _) = names(false, true);
        assert_eq!(fight.0.len(), 3);
        assert_eq!(fight.0[2].0, "self_sealing_tanks_factor");
    }

    #[test]
    fn recording_breakdowns_with_protection_changes_neither_the_outcome_nor_the_hash() {
        let run = |record: bool, seed: u64| {
            let mut world = protection_world(seed);
            world.set_config(crate::world::SimConfig {
                record_breakdowns: record,
            });
            protect(&mut world, 5, true, true);
            protect(&mut world, 2, true, false);
            strike_pulse(&mut world, Pulse::Morning);
            world.age_contacts_at_end_of_pulse(1, Pulse::Morning);
            (world.state_hash(), world.events().len())
        };
        for seed in 1..=10 {
            assert_eq!(run(true, seed), run(false, seed), "seed {seed}");
        }
    }

    #[test]
    fn protection_never_raises_a_chance() {
        let chances = |armour: bool, tanks: bool| {
            let mut world = protection_world(3);
            protect(&mut world, 5, armour, tanks);
            protect(&mut world, 2, armour, tanks);
            three_chances(&mut world)
        };
        let none = chances(false, false);
        let armour = chances(true, false);
        let tanks = chances(false, true);
        let both = chances(true, true);
        for i in 0..3 {
            assert!(armour[i] <= none[i] && tanks[i] <= none[i]);
            assert!(both[i] <= armour[i] && both[i] <= tanks[i]);
            assert!(both[i] > Fixed::ZERO);
        }
        // The same holds for any factor in 0..=1.
        for (num, den) in [(0, 1), (1, 4), (1, 2), (9, 10), (1, 1)] {
            let mut world = protection_world(3);
            air_figures(&mut world).pilot_armour_factor = fixed(num, den);
            air_figures(&mut world).self_sealing_tanks_factor = fixed(num, den);
            protect(&mut world, 5, true, true);
            protect(&mut world, 2, true, true);
            let protected = three_chances(&mut world);
            for i in 0..3 {
                assert!(protected[i] <= none[i]);
            }
        }
    }
    // ---- raids on a base (decision 0034) ----
    //
    // Allied base 1 at (0,0) and Japanese base 2 at (5,0), 200 nm apart.
    // The standard world's task forces are still there, and no air group
    // but the ones a test adds. Base 2 has 1,000 t and no guns.

    const BASE_ALLIED: BaseId = BaseId::new(1);
    const BASE_JAPANESE: BaseId = BaseId::new(2);

    fn base_world(seed: u64) -> World {
        let mut world = world(seed, vec![]);
        world.truth.air_groups.clear();
        for (id, side, hex, supply) in [
            (BASE_ALLIED, Side::Allied, Hex::new(0, 0), 0),
            (BASE_JAPANESE, Side::Japanese, Hex::new(5, 0), 1000),
        ] {
            world.truth.bases.insert(
                id,
                crate::state::Base {
                    id,
                    name: format!("Test base {}", id.raw()),
                    side,
                    hex,
                    supply_t: Fixed::from_int(supply),
                    anti_aircraft: 0,
                    aircraft_repairs_per_day: 0,
                    fuel_t: Fixed::ZERO,
                },
            );
        }
        world
    }

    /// Adds a group at `base` with `mission`.
    fn base_group(
        world: &mut World,
        id: u32,
        base: BaseId,
        type_id: u32,
        ready: u32,
        mission: AirMission,
    ) {
        let side = world.truth.bases[&base].side;
        let mut added = group(AirGroupId::new(id), side, 1, type_id, ready);
        added.location = AirGroupLocation::Base(base);
        added.mission = mission;
        world.truth.air_groups.insert(added.id, added);
    }

    /// The figures that make every bomb hit and every hit land on `lands`.
    fn sure_bombs(world: &mut World, aircraft: Fixed, supply: Fixed) {
        let figures = air_figures(world);
        figures.bomb_hit_chance = Fixed::ONE;
        figures.base_hit_aircraft = aircraft;
        figures.base_hit_supply = supply;
    }

    fn base_launches(world: &World) -> Vec<Event> {
        events_where(world, |k| matches!(k, EventKind::BaseStrikeLaunched { .. }))
    }

    fn base_attacks(world: &World) -> Vec<Event> {
        events_where(world, |k| matches!(k, EventKind::BaseAttacked { .. }))
    }

    fn ground_losses(world: &World) -> Vec<(u32, u32, u32)> {
        events_where(world, |k| {
            matches!(k, EventKind::AircraftDestroyedOnGround { .. })
        })
        .iter()
        .map(|event| match event.kind {
            EventKind::AircraftDestroyedOnGround {
                air_group,
                aircraft,
                remaining,
                ..
            } => (air_group.raw(), aircraft, remaining),
            _ => unreachable!(),
        })
        .collect()
    }

    fn supply_losses(world: &World) -> Vec<(i64, i64)> {
        events_where(world, |k| {
            matches!(k, EventKind::BaseSupplyDestroyed { .. })
        })
        .iter()
        .map(|event| match event.kind {
            EventKind::BaseSupplyDestroyed {
                destroyed_t,
                supply_t,
                ..
            } => (destroyed_t.trunc(), supply_t.trunc()),
            _ => unreachable!(),
        })
        .collect()
    }

    /// A world with Allied dive bombers (group 1, 18) on a base strike on
    /// base 2.
    fn raid_world(seed: u64) -> World {
        let mut world = base_world(seed);
        base_group(
            &mut world,
            1,
            BASE_ALLIED,
            2,
            18,
            AirMission::BaseStrike(BASE_JAPANESE),
        );
        group_mut(&mut world, AirGroupId::new(1)).mission_order = Some(OrderId::new(7));
        world
    }

    #[test]
    fn a_base_strike_launches_every_aircraft_in_the_morning_citing_its_order_and_needing_no_contact(
    ) {
        let mut world = raid_world(1);
        world.decide_strikes(1, Pulse::Morning);
        let launched = base_launches(&world);
        assert_eq!(launched.len(), 1);
        assert_eq!(
            launched[0].kind,
            EventKind::BaseStrikeLaunched {
                air_group: AirGroupId::new(1),
                base: BASE_JAPANESE,
                hex: Hex::new(5, 0),
                aircraft: 18,
                distance_nm: Fixed::from_int(200),
            }
        );
        assert_eq!(launched[0].causes, vec![Cause::Order(OrderId::new(7))]);
        assert_eq!(world.strikes_in_flight.len(), 1);
        // Nobody holds a contact, and no contact strike was launched.
        assert!(launches(&world).is_empty());
    }

    #[test]
    fn each_launch_condition_alone_stops_a_base_strike_and_a_base_at_the_range_is_struck() {
        type Change = Box<dyn Fn(&mut World)>;
        let stops: Vec<(&str, Change, Pulse)> = vec![
            ("night", Box::new(|_| {}), Pulse::Night),
            ("afternoon", Box::new(|_| {}), Pulse::Afternoon),
            (
                "no aircraft",
                Box::new(|w| group_mut(w, AirGroupId::new(1)).ready = 0),
                Pulse::Morning,
            ),
            (
                "a type with no weapon",
                Box::new(|w| {
                    group_mut(w, AirGroupId::new(1)).aircraft_type = AircraftTypeId::new(4)
                }),
                Pulse::Morning,
            ),
            (
                "no such base",
                Box::new(|w| {
                    group_mut(w, AirGroupId::new(1)).mission =
                        AirMission::BaseStrike(BaseId::new(9))
                }),
                Pulse::Morning,
            ),
            (
                "the base is held by the group's own side",
                Box::new(|w| {
                    w.truth.bases.get_mut(&BASE_JAPANESE).expect("base").side = Side::Allied
                }),
                Pulse::Morning,
            ),
            (
                "beyond the radius",
                Box::new(|w| {
                    w.truth.bases.get_mut(&BASE_JAPANESE).expect("base").hex = Hex::new(7, 0)
                }),
                Pulse::Morning,
            ),
            (
                "a short-range type",
                Box::new(|w| {
                    group_mut(w, AirGroupId::new(1)).aircraft_type = AircraftTypeId::new(6)
                }),
                Pulse::Morning,
            ),
        ];
        for (label, change, pulse) in stops {
            let mut world = raid_world(1);
            change(&mut world);
            world.decide_strikes(1, pulse);
            assert!(world.strikes_in_flight.is_empty(), "{label} launched");
            assert!(
                base_launches(&world).is_empty(),
                "{label} recorded a launch"
            );
        }
        // 6 hexes, 240 nm, is within the 250; 7 hexes is not.
        let mut world = raid_world(1);
        world.truth.bases.get_mut(&BASE_JAPANESE).expect("base").hex = Hex::new(6, 0);
        world.decide_strikes(1, Pulse::Morning);
        assert_eq!(base_launches(&world).len(), 1);
    }

    #[test]
    fn a_base_strike_never_goes_for_a_contact_and_a_naval_strike_never_for_a_base() {
        // A fresh contact on the Japanese force at (5,0), and the base too.
        let mut world = world_with_a_good_contact(1);
        world.truth.air_groups.clear();
        world.truth.bases = base_world(1).truth.bases;
        base_group(
            &mut world,
            1,
            BASE_ALLIED,
            2,
            18,
            AirMission::BaseStrike(BASE_JAPANESE),
        );
        world.decide_strikes(1, Pulse::Morning);
        assert!(launches(&world).is_empty());
        assert_eq!(base_launches(&world).len(), 1);

        let mut world = base_world(1);
        base_group(&mut world, 1, BASE_ALLIED, 2, 18, AirMission::NavalStrike);
        world.decide_strikes(1, Pulse::Morning);
        assert!(world.strikes_in_flight.is_empty());
        assert!(events_where(&world, |_| true).is_empty());
    }

    /// The ground pool the air combat step works on, for the strikes in
    /// flight in a morning pulse.
    fn ground_of(world: &World) -> Ground {
        let strikes = &world.strikes_in_flight;
        let mut raids = group_raids(strikes);
        let mut fighters = world.split_fighter_groups();
        send_escorts(&mut raids, &mut fighters);
        let flying = world.aircraft_flying(Pulse::Morning, strikes, &raids, &fighters);
        world.aircraft_on_the_ground(strikes, &flying)
    }

    #[test]
    fn aircraft_on_the_ground_are_those_resting_and_those_on_a_strike_that_did_not_launch() {
        let mut world = raid_world(1);
        let strike_back = AirMission::BaseStrike(BASE_ALLIED);
        for (id, type_id, ready, mission) in [
            (20, 2, 10, AirMission::Rest),
            (21, 1, 6, AirMission::Search),
            (22, 5, 12, AirMission::CombatAirPatrol),
            // On a naval strike with no contact: stays on the ground.
            (23, 2, 7, AirMission::NavalStrike),
            // Launches at base 1 in this pulse: in the air.
            (24, 2, 4, strike_back),
            // A base strike out of its type's radius: stays on the ground.
            (25, 6, 3, strike_back),
            // Resting, but nothing ready.
            (26, 2, 0, AirMission::Rest),
        ] {
            base_group(&mut world, id, BASE_JAPANESE, type_id, ready, mission);
        }
        // A group at the other base, resting, is not at base 2.
        base_group(&mut world, 27, BASE_ALLIED, 2, 9, AirMission::Rest);
        // Group 21 flies a search because it has an arc.
        group_mut(&mut world, AirGroupId::new(21)).search = Some(crate::state::SearchArc {
            from_deg: 0,
            to_deg: 350,
            range_nm: Fixed::from_int(300),
        });
        world.decide_strikes(1, Pulse::Morning);
        let strikes: Vec<AirGroupId> = world
            .strikes_in_flight
            .iter()
            .map(|strike| strike.air_group)
            .collect();
        assert_eq!(strikes, vec![AirGroupId::new(1), AirGroupId::new(24)]);
        let ground = ground_of(&world);
        assert_eq!(
            ground[&BASE_JAPANESE],
            vec![
                (AirGroupId::new(20), Stock::Ready, 10),
                (AirGroupId::new(23), Stock::Ready, 7),
                (AirGroupId::new(25), Stock::Ready, 3)
            ]
        );
        // Base 1 is counted too, as group 24 strikes it: the bombers of
        // group 1 launched and are not there, the resting group 27 is.
        assert_eq!(
            ground[&BASE_ALLIED],
            vec![(AirGroupId::new(27), Stock::Ready, 9)]
        );
    }

    #[test]
    fn hits_on_aircraft_come_off_the_lowest_group_first_across_raids_and_never_pass_what_was_there()
    {
        let mut world = raid_world(1);
        // A second Allied place and raid on the same base.
        world.truth.bases.insert(
            BaseId::new(3),
            crate::state::Base {
                id: BaseId::new(3),
                name: "Test base 3".to_string(),
                side: Side::Allied,
                hex: Hex::new(1, 0),
                supply_t: Fixed::ZERO,
                anti_aircraft: 0,
                aircraft_repairs_per_day: 0,
                fuel_t: Fixed::ZERO,
            },
        );
        group_mut(&mut world, AirGroupId::new(1)).ready = 12;
        base_group(
            &mut world,
            9,
            BaseId::new(3),
            2,
            6,
            AirMission::BaseStrike(BASE_JAPANESE),
        );
        base_group(&mut world, 20, BASE_JAPANESE, 2, 10, AirMission::Rest);
        base_group(&mut world, 21, BASE_JAPANESE, 2, 7, AirMission::Rest);
        sure_bombs(&mut world, Fixed::ONE, Fixed::ZERO);
        strike_pulse(&mut world, Pulse::Morning);
        // Group 1's 12 hits: 10 from group 20, 2 from group 21. Group 9's 6
        // hits: the 5 left of group 21, and one wasted.
        assert_eq!(
            ground_losses(&world),
            vec![(20, 10, 0), (21, 2, 5), (21, 5, 0)]
        );
        assert_eq!(ready_of(&world, 20), 0);
        assert_eq!(ready_of(&world, 21), 0);
        let attacks = base_attacks(&world);
        assert_eq!(attacks.len(), 2);
        // Each loss cites the attack that did it.
        let losses = events_where(&world, |k| {
            matches!(k, EventKind::AircraftDestroyedOnGround { .. })
        });
        assert_eq!(losses[0].causes, vec![Cause::Event(attacks[0].id)]);
        assert_eq!(losses[2].causes, vec![Cause::Event(attacks[1].id)]);
        // Nothing else was lost: no supply hit, no loss in the air.
        assert!(supply_losses(&world).is_empty());
        assert!(self::losses(&world).is_empty());
    }

    #[test]
    fn hits_on_supply_destroy_the_fixed_weight_and_never_more_than_the_base_has() {
        for (supply, bombers, expected) in [
            // 18 hits would destroy 900 t: only the 120 there.
            (120, vec![18], vec![(120, 0)]),
            // 2 hits of 50 t, then 2 more with 20 t left.
            (120, vec![2, 2], vec![(100, 20), (20, 0)]),
            // Nothing left: nothing destroyed, nothing recorded.
            (0, vec![5], vec![]),
        ] {
            let mut world = raid_world(1);
            world
                .truth
                .bases
                .get_mut(&BASE_JAPANESE)
                .expect("base")
                .supply_t = Fixed::from_int(supply);
            group_mut(&mut world, AirGroupId::new(1)).ready = bombers[0];
            if let Some(second) = bombers.get(1) {
                world.truth.bases.insert(
                    BaseId::new(3),
                    crate::state::Base {
                        id: BaseId::new(3),
                        name: "Test base 3".to_string(),
                        side: Side::Allied,
                        hex: Hex::new(1, 0),
                        supply_t: Fixed::ZERO,
                        anti_aircraft: 0,
                        aircraft_repairs_per_day: 0,
                        fuel_t: Fixed::ZERO,
                    },
                );
                base_group(
                    &mut world,
                    9,
                    BaseId::new(3),
                    2,
                    *second,
                    AirMission::BaseStrike(BASE_JAPANESE),
                );
            }
            sure_bombs(&mut world, Fixed::ZERO, Fixed::ONE);
            strike_pulse(&mut world, Pulse::Morning);
            assert_eq!(supply_losses(&world), expected, "{supply} t, {bombers:?}");
            let left = world.truth.bases[&BASE_JAPANESE].supply_t;
            assert_eq!(left.trunc(), expected.last().map_or(supply, |last| last.1));
            assert!(!left.is_negative());
        }
    }

    #[test]
    fn a_base_raid_and_a_ship_raid_on_one_hex_divide_the_patrol_and_meet_only_their_own_guns() {
        let mut world = world_with_a_good_contact(1);
        world.truth.air_groups.clear();
        world.truth.bases = base_world(1).truth.bases;
        // The base shares the hex of the Japanese force.
        {
            let base = world.truth.bases.get_mut(&BASE_JAPANESE).expect("base");
            base.anti_aircraft = 20;
        }
        base_group(
            &mut world,
            1,
            BASE_ALLIED,
            2,
            12,
            AirMission::BaseStrike(BASE_JAPANESE),
        );
        base_group(&mut world, 11, BASE_ALLIED, 2, 6, AirMission::NavalStrike);
        base_group(
            &mut world,
            4,
            BASE_JAPANESE,
            5,
            12,
            AirMission::CombatAirPatrol,
        );
        sure_fighters(&mut world);
        // Nobody is lost to guns: only the division is under test.
        air_figures(&mut world).anti_aircraft_hit_chance = Fixed::ZERO;
        set_guns(&mut world, 1, 5);
        strike_pulse(&mut world, Pulse::Morning);

        assert_eq!(launches(&world).len(), 1);
        assert_eq!(base_launches(&world).len(), 1);
        // 12 patrol over 18 aircraft: 8 at the base raid, 4 at the ships.
        let fights: Vec<_> = air_fights(&world).iter().map(fight_of).collect();
        assert_eq!(
            fights,
            vec![
                (4, 1, AirFirer::Patrol, AirTarget::Bombers, 8, 8),
                (4, 11, AirFirer::Patrol, AirTarget::Bombers, 4, 4),
            ]
        );
        // The base's guns fire at the base raid only, the ships' at the ships'.
        let base_guns = events_where(&world, |k| {
            matches!(k, EventKind::BaseAntiAircraftFire { .. })
        });
        assert_eq!(base_guns.len(), 1);
        match base_guns[0].kind {
            EventKind::BaseAntiAircraftFire {
                base,
                air_group,
                shots,
                ..
            } => assert_eq!(
                (base, air_group, shots.trunc()),
                (BASE_JAPANESE, AirGroupId::new(1), 20)
            ),
            _ => unreachable!(),
        }
        // Two cruisers of 5 shots each.
        let ship_guns: Vec<_> = gun_fires(&world).iter().map(guns_of).collect();
        assert_eq!(ship_guns, vec![(2, 11, 10, 0)]);
        // 4 and 2 bombers are left to attack: the base is bombed by group 1
        // and the ships by group 11.
        assert_eq!(base_attacks(&world).len(), 1);
        assert_eq!(attacks(&world).len(), 1);
    }

    #[test]
    fn a_base_with_no_guns_fires_none_and_a_raid_with_no_bombers_left_attacks_nothing() {
        let mut world = raid_world(1);
        strike_pulse(&mut world, Pulse::Morning);
        assert!(events_where(&world, |k| {
            matches!(k, EventKind::BaseAntiAircraftFire { .. })
        })
        .is_empty());
        // Every bomber shot down: no attack and nothing destroyed.
        let mut world = raid_world(1);
        world
            .truth
            .bases
            .get_mut(&BASE_JAPANESE)
            .expect("base")
            .anti_aircraft = 40;
        air_figures(&mut world).anti_aircraft_hit_chance = Fixed::ONE;
        base_group(&mut world, 20, BASE_JAPANESE, 2, 10, AirMission::Rest);
        sure_bombs(&mut world, Fixed::ONE, Fixed::ZERO);
        strike_pulse(&mut world, Pulse::Morning);
        assert_eq!(ready_of(&world, 1), 0);
        assert!(base_attacks(&world).is_empty());
        assert!(ground_losses(&world).is_empty());
        assert_eq!(ready_of(&world, 20), 10);
    }

    #[test]
    fn the_base_attack_event_reads_plainly_and_a_hit_lands_where_its_draw_says() {
        let mut world = raid_world(3);
        world.config.record_breakdowns = true;
        base_group(&mut world, 20, BASE_JAPANESE, 2, 10, AirMission::Rest);
        strike_pulse(&mut world, Pulse::Morning);
        let attack = &base_attacks(&world)[0];
        match attack.kind {
            EventKind::BaseAttacked {
                shots,
                hits,
                hits_on_aircraft,
                hits_on_supply,
                ..
            } => {
                assert_eq!(shots, Fixed::from_int(18));
                assert!(hits_on_aircraft + hits_on_supply <= hits);
            }
            _ => unreachable!(),
        }
        assert!(attack.breakdown.is_some());
        let text = attack.kind.to_string();
        assert!(
            text.starts_with("air_group:1 bombs at base:2: 18 shots x 0.2 = 3.6 expected, "),
            "{text}"
        );
        // The draw per hit is from the stream of (group, base): replay it.
        let mut stream = world.rng_pair(
            1,
            Some(Pulse::Morning),
            Subsystem::AirCombat,
            EntityRef::AirGroup(AirGroupId::new(1)),
            EntityRef::Base(BASE_JAPANESE),
        );
        let hits = roll_shots(&mut stream, Fixed::from_int(18), fixed(2, 10));
        let (mut aircraft, mut supply) = (0, 0);
        for _ in 0..hits {
            let draw = stream.below(Fixed::SCALE.unsigned_abs());
            if draw < raw_u64(fixed(3, 10)) {
                aircraft += 1;
            } else if draw < raw_u64(fixed(6, 10)) {
                supply += 1;
            }
        }
        match attack.kind {
            EventKind::BaseAttacked {
                hits: recorded,
                hits_on_aircraft,
                hits_on_supply,
                ..
            } => assert_eq!(
                (recorded, hits_on_aircraft, hits_on_supply),
                (hits, aircraft, supply)
            ),
            _ => unreachable!(),
        }
    }

    #[test]
    fn recording_breakdowns_changes_neither_the_base_raid_nor_the_hash() {
        let run = |record: bool| {
            let mut world = raid_world(5);
            world.config.record_breakdowns = record;
            base_group(&mut world, 20, BASE_JAPANESE, 2, 10, AirMission::Rest);
            world
                .truth
                .bases
                .get_mut(&BASE_JAPANESE)
                .expect("base")
                .anti_aircraft = 20;
            strike_pulse(&mut world, Pulse::Morning);
            let kinds: Vec<EventKind> = world.events().iter().map(|e| e.kind.clone()).collect();
            (kinds, world.state_hash())
        };
        assert_eq!(run(false), run(true));
    }

    #[test]
    fn the_hash_sees_a_base_strike_target_and_a_base_gun_and_a_gunless_base_hashes_as_before() {
        let world = raid_world(1);
        let base_hash = world.state_hash();
        let mut other = raid_world(1);
        group_mut(&mut other, AirGroupId::new(1)).mission = AirMission::BaseStrike(BaseId::new(9));
        assert_ne!(other.state_hash(), base_hash);
        let mut guns = raid_world(1);
        guns.truth
            .bases
            .get_mut(&BASE_JAPANESE)
            .expect("base")
            .anti_aircraft = 1;
        assert_ne!(guns.state_hash(), base_hash);
        // The mission code is part of the hash too.
        let mut rest = raid_world(1);
        group_mut(&mut rest, AirGroupId::new(1)).mission = AirMission::Rest;
        assert_ne!(rest.state_hash(), base_hash);
    }

    #[test]
    fn a_base_strike_order_needs_figures_a_weapon_and_an_enemy_base() {
        let accepted = |setup: &dyn Fn(&mut World), mission: AirMission| {
            let mut world = base_world(1);
            base_group(&mut world, 1, BASE_ALLIED, 2, 18, AirMission::Rest);
            setup(&mut world);
            world.submit_orders(
                Side::Allied,
                vec![mission_order(AirGroupId::new(1), mission)],
            );
            world.resolve_day().expect("day resolves");
            (
                rejections(&world),
                world.truth.air_groups[&AirGroupId::new(1)].mission,
            )
        };
        let at = |base: u32| AirMission::BaseStrike(BaseId::new(base));
        assert_eq!(accepted(&|_| {}, at(2)), (vec![], at(2)));
        assert_eq!(
            accepted(&|_| {}, at(1)),
            (vec![RejectReason::NotAnEnemyBase], AirMission::Rest)
        );
        assert_eq!(
            accepted(&|_| {}, at(9)),
            (vec![RejectReason::NotAnEnemyBase], AirMission::Rest)
        );
        assert_eq!(
            accepted(
                &|w| group_mut(w, AirGroupId::new(1)).aircraft_type = AircraftTypeId::new(4),
                at(2)
            ),
            (
                vec![RejectReason::AircraftAttackWithNothing],
                AirMission::Rest
            )
        );
        assert_eq!(
            accepted(&|w| w.ref_data.air_combat = None, at(2)),
            (vec![RejectReason::NoAirCombatFigures], AirMission::Rest)
        );
    }

    // ---- wear and repair (decision 0035) ----

    use crate::events::SortieKind;
    use crate::refdata::AirUpkeepFigures;
    use crate::state::SearchArc;

    fn upkeep(world: &mut World, base: Fixed, per_100: Fixed, write_off: Fixed, carriers: u32) {
        world.ref_data.air_upkeep = Some(AirUpkeepFigures {
            wear_base_chance: base,
            wear_chance_per_100_nm: per_100,
            write_off_share: write_off,
            carrier_repairs_per_day: carriers,
        });
    }

    /// The `AircraftWorn` events as (group, sortie, distance, aircraft,
    /// chance, worn, written off, ready, damaged).
    #[allow(clippy::type_complexity)]
    fn worn(world: &World) -> Vec<(u32, SortieKind, i64, u32, Fixed, u32, u32, u32, u32)> {
        events_where(world, |k| matches!(k, EventKind::AircraftWorn { .. }))
            .iter()
            .map(|event| match event.kind {
                EventKind::AircraftWorn {
                    air_group,
                    sortie,
                    distance_nm,
                    aircraft,
                    chance,
                    worn,
                    written_off,
                    ready,
                    damaged,
                    ..
                } => (
                    air_group.raw(),
                    sortie,
                    distance_nm.trunc(),
                    aircraft,
                    chance,
                    worn,
                    written_off,
                    ready,
                    damaged,
                ),
                _ => unreachable!(),
            })
            .collect()
    }

    fn repaired(world: &World) -> Vec<(u32, u32, u32, u32)> {
        events_where(world, |k| matches!(k, EventKind::AircraftRepaired { .. }))
            .iter()
            .map(|event| match event.kind {
                EventKind::AircraftRepaired {
                    air_group,
                    repaired,
                    ready,
                    damaged,
                    ..
                } => (air_group.raw(), repaired, ready, damaged),
                _ => unreachable!(),
            })
            .collect()
    }

    /// Both air steps of a pulse and then the wear step.
    fn fly_and_wear(world: &mut World, pulse: Pulse) {
        world.search_from_the_air(1, pulse);
        strike_pulse(world, pulse);
        world.wear_aircraft(1, pulse);
    }

    const SURE: Fixed = Fixed::ONE;
    const NEVER: Fixed = Fixed::ZERO;

    #[test]
    fn the_wear_chance_is_the_base_plus_the_distance_over_100_and_never_more_than_1() {
        let figures = AirUpkeepFigures {
            wear_base_chance: fixed(5, 100),
            wear_chance_per_100_nm: fixed(2, 100),
            write_off_share: fixed(2, 10),
            carrier_repairs_per_day: 2,
        };
        let chance = |nm: i64| figures.wear_chance(Fixed::from_int(nm));
        assert_eq!(chance(0), fixed(5, 100));
        assert_eq!(chance(160), fixed(82, 1000));
        assert_eq!(chance(400), fixed(13, 100));
        assert_eq!(chance(600), fixed(17, 100));
        let steep = AirUpkeepFigures {
            wear_base_chance: fixed(9, 10),
            wear_chance_per_100_nm: fixed(1, 10),
            ..figures
        };
        assert_eq!(steep.wear_chance(Fixed::from_int(100)), Fixed::ONE);
        assert_eq!(steep.wear_chance(Fixed::from_int(4000)), Fixed::ONE);
        assert!(figures.problem().is_none());
        for bad in [
            AirUpkeepFigures {
                wear_base_chance: fixed(11, 10),
                ..figures
            },
            AirUpkeepFigures {
                wear_chance_per_100_nm: fixed(-1, 10),
                ..figures
            },
            AirUpkeepFigures {
                write_off_share: fixed(2, 1),
                ..figures
            },
        ] {
            assert!(bad.problem().is_some());
        }
    }

    #[test]
    fn without_the_figures_nothing_is_noted_worn_or_repaired_and_a_world_hashes_as_before() {
        let scouts = |world: &mut World| {
            base_group(world, 2, BASE_ALLIED, 1, 6, AirMission::Search);
            group_mut(world, AirGroupId::new(2)).search = Some(SearchArc {
                from_deg: 0,
                to_deg: 350,
                range_nm: Fixed::from_int(300),
            });
        };
        let mut world = raid_world(1);
        scouts(&mut world);
        let before = world.state_hash();
        world
            .truth
            .bases
            .get_mut(&BASE_ALLIED)
            .expect("base")
            .aircraft_repairs_per_day = 5;
        group_mut(&mut world, AirGroupId::new(2)).damaged = 3;
        // A damaged aircraft and a base's repairs are in the hash; with none
        // of either, the hash is what it was.
        assert_ne!(world.state_hash(), before);
        for pulse in Pulse::ALL {
            world.repair_aircraft(1, pulse);
            fly_and_wear(&mut world, pulse);
            assert!(world.sorties.is_empty());
        }
        assert!(worn(&world).is_empty() && repaired(&world).is_empty());
        assert_eq!(group_mut(&mut world, AirGroupId::new(2)).damaged, 3);
        let mut again = raid_world(1);
        scouts(&mut again);
        assert_eq!(again.state_hash(), before);
        group_mut(&mut again, AirGroupId::new(2)).damaged = 3;
        again
            .truth
            .bases
            .get_mut(&BASE_ALLIED)
            .expect("base")
            .aircraft_repairs_per_day = 5;
        group_mut(&mut again, AirGroupId::new(2)).damaged = 0;
        again
            .truth
            .bases
            .get_mut(&BASE_ALLIED)
            .expect("base")
            .aircraft_repairs_per_day = 0;
        assert_eq!(again.state_hash(), before);
    }

    #[test]
    fn a_patrol_flies_with_no_raid_at_distance_0_and_cites_the_group_orders() {
        let mut world = base_world(1);
        base_group(
            &mut world,
            5,
            BASE_ALLIED,
            5,
            10,
            AirMission::CombatAirPatrol,
        );
        group_mut(&mut world, AirGroupId::new(5)).mission_order = Some(OrderId::new(3));
        group_mut(&mut world, AirGroupId::new(5)).share_order = Some(OrderId::new(4));
        group_mut(&mut world, AirGroupId::new(5)).patrol_share = fixed(1, 2);
        upkeep(&mut world, SURE, Fixed::ONE, NEVER, 0);
        fly_and_wear(&mut world, Pulse::Morning);
        // Its share, 5, rounded down, flies; the rest do not. Distance 0, so
        // the chance is the base chance, and every one is worn, none lost.
        assert_eq!(
            worn(&world),
            vec![(5, SortieKind::Patrol, 0, 5, SURE, 5, 0, 5, 5)]
        );
        let event = &events_where(&world, |k| matches!(k, EventKind::AircraftWorn { .. }))[0];
        assert_eq!(
            event.causes,
            vec![Cause::Order(OrderId::new(3)), Cause::Order(OrderId::new(4))]
        );
        assert_eq!(
            event.kind.to_string(),
            "air_group:5 patrol sortie of 0 nm: 5 aircraft x 1 = 5 expected worn, 5 worn, 0 written off, 5 ready and 5 damaged left"
        );
        let breakdown = event.breakdown.as_ref().expect("a breakdown");
        assert_eq!(breakdown.result, Fixed::from_int(5));
        assert_eq!(
            breakdown.factors.iter().map(|f| f.name).collect::<Vec<_>>(),
            vec![
                "aircraft",
                "wear_base_chance",
                "wear_chance_per_100_nm",
                "distance_nm",
                "chance",
                "write_off_share"
            ]
        );
        assert_eq!(group_mut(&mut world, AirGroupId::new(5)).ready, 5);
        assert_eq!(group_mut(&mut world, AirGroupId::new(5)).damaged, 5);
    }

    #[test]
    fn a_sortie_of_no_aircraft_is_not_rolled_and_nothing_flies_at_night() {
        // A patrol share of 0 keeps nobody up; a group with none ready flies
        // nothing.
        let mut world = base_world(1);
        base_group(
            &mut world,
            5,
            BASE_ALLIED,
            5,
            10,
            AirMission::CombatAirPatrol,
        );
        group_mut(&mut world, AirGroupId::new(5)).patrol_share = Fixed::ZERO;
        base_group(
            &mut world,
            6,
            BASE_ALLIED,
            5,
            0,
            AirMission::CombatAirPatrol,
        );
        upkeep(&mut world, SURE, SURE, NEVER, 0);
        fly_and_wear(&mut world, Pulse::Morning);
        assert!(worn(&world).is_empty());
        // At night nothing flies and nothing rolls.
        group_mut(&mut world, AirGroupId::new(5)).patrol_share = Fixed::ONE;
        fly_and_wear(&mut world, Pulse::Night);
        assert!(worn(&world).is_empty());
        assert!(world.sorties.is_empty());
    }

    #[test]
    fn a_search_is_twice_the_range_of_its_arc_and_cites_the_search_event() {
        let mut world = base_world(1);
        base_group(&mut world, 5, BASE_ALLIED, 1, 8, AirMission::Search);
        group_mut(&mut world, AirGroupId::new(5)).search = Some(SearchArc {
            from_deg: 0,
            to_deg: 350,
            range_nm: Fixed::from_int(240),
        });
        upkeep(&mut world, fixed(1, 10), fixed(1, 10), NEVER, 0);
        fly_and_wear(&mut world, Pulse::Morning);
        // 480 nm: 0.1 + 0.1 x 4.8 = 0.58.
        let rolls = worn(&world);
        assert_eq!(rolls.len(), 1);
        let (group, sortie, distance, aircraft, chance, ..) = rolls[0];
        assert_eq!(
            (group, sortie, distance, aircraft, chance),
            (5, SortieKind::Search, 480, 8, fixed(58, 100))
        );
        let flown = events_where(&world, |k| matches!(k, EventKind::SearchFlown { .. }));
        let event = &events_where(&world, |k| matches!(k, EventKind::AircraftWorn { .. }))[0];
        assert_eq!(event.causes, vec![Cause::Event(flown[0].id)]);
    }

    #[test]
    fn a_strike_at_a_base_and_a_strike_at_a_contact_roll_the_aircraft_left_at_twice_the_distance() {
        // A raid on a base 200 nm away: 400 nm, 0.1 + 0.1 x 4 = 0.5.
        let mut world = raid_world(1);
        upkeep(&mut world, fixed(1, 10), fixed(1, 10), NEVER, 0);
        // The base's guns bring down some of the bombers.
        world
            .truth
            .bases
            .get_mut(&BASE_JAPANESE)
            .expect("base")
            .anti_aircraft = 20;
        air_figures(&mut world).anti_aircraft_hit_chance = fixed(1, 4);
        fly_and_wear(&mut world, Pulse::Morning);
        let lost: u32 = events_where(&world, |k| matches!(k, EventKind::AircraftLost { .. }))
            .iter()
            .map(|e| match e.kind {
                EventKind::AircraftLost { aircraft, .. } => aircraft,
                _ => 0,
            })
            .sum();
        assert!(lost > 0);
        let rolls = worn(&world);
        assert_eq!(rolls.len(), 1);
        let (group, sortie, distance, aircraft, chance, worn_, off, ready, damaged) = rolls[0];
        assert_eq!(
            (group, sortie, distance, aircraft, chance),
            (1, SortieKind::BaseStrike, 400, 18 - lost, fixed(5, 10))
        );
        assert_eq!((off, ready, damaged), (0, 18 - lost - worn_, worn_));
        let launch = events_where(&world, |k| {
            matches!(k, EventKind::BaseStrikeLaunched { .. })
        });
        let event = &events_where(&world, |k| matches!(k, EventKind::AircraftWorn { .. }))[0];
        assert_eq!(event.causes, vec![Cause::Event(launch[0].id)]);

        // A strike at a contact 200 nm away, in the standard world.
        let mut world = world_with_a_good_contact(1);
        upkeep(&mut world, fixed(1, 10), fixed(1, 10), NEVER, 0);
        fly_and_wear(&mut world, Pulse::Morning);
        let rolls = worn(&world);
        let strike = rolls.iter().find(|roll| roll.0 == 1).expect("group 1 flew");
        assert_eq!(
            (strike.1, strike.2, strike.3, strike.4),
            (SortieKind::Strike, 400, 18, fixed(5, 10))
        );
        let launch = launches(&world);
        let event = &events_where(&world, |k| matches!(k, EventKind::AircraftWorn { .. }))[0];
        assert_eq!(event.causes, vec![Cause::Event(launch[0].id)]);
    }

    /// The draws of one group's stream for a roll of `aircraft` at `chance`
    /// and then of the worn at `share`: (worn, written off).
    fn replay(
        stream: &mut crate::rng::RngStream,
        aircraft: u32,
        chance: Fixed,
        share: Fixed,
    ) -> (u32, u32) {
        let worn = (0..aircraft)
            .map(|_| u32::from(stream.chance(chance)))
            .sum::<u32>();
        let off = (0..worn).map(|_| u32::from(stream.chance(share))).sum();
        (worn, off)
    }

    #[test]
    fn a_fighter_group_rolls_its_patrol_then_its_escorts_from_one_stream_of_its_own() {
        for seed in 1..=20u64 {
            let mut world = raid_world(seed);
            base_group(
                &mut world,
                5,
                BASE_ALLIED,
                5,
                11,
                AirMission::CombatAirPatrol,
            );
            group_mut(&mut world, AirGroupId::new(5)).patrol_share = fixed(1, 2);
            let (base, per_100, share) = (fixed(3, 10), fixed(1, 10), fixed(4, 10));
            upkeep(&mut world, base, per_100, share, 0);
            fly_and_wear(&mut world, Pulse::Morning);
            // 11 x 0.5 = 5 patrol, 6 escorts, none of them shot down (no
            // defenders, no guns). Patrol at 0.3; escorts out 200 nm and back
            // at 0.3 + 0.1 x 4 = 0.7.
            let mut stream = world.rng(
                1,
                Some(Pulse::Morning),
                crate::rng::Subsystem::Upkeep,
                EntityRef::AirGroup(AirGroupId::new(5)),
            );
            let (patrol_worn, patrol_off) = replay(&mut stream, 5, base, share);
            let (escort_worn, escort_off) = replay(&mut stream, 6, fixed(7, 10), share);
            let rolls: Vec<_> = worn(&world).into_iter().filter(|r| r.0 == 5).collect();
            assert_eq!(rolls.len(), 2, "seed {seed}");
            assert_eq!(
                rolls[0],
                (
                    5,
                    SortieKind::Patrol,
                    0,
                    5,
                    base,
                    patrol_worn,
                    patrol_off,
                    11 - patrol_worn,
                    patrol_worn - patrol_off
                ),
                "seed {seed}"
            );
            assert_eq!(
                rolls[1],
                (
                    5,
                    SortieKind::Escort,
                    400,
                    6,
                    fixed(7, 10),
                    escort_worn,
                    escort_off,
                    11 - patrol_worn - escort_worn,
                    patrol_worn - patrol_off + escort_worn - escort_off
                ),
                "seed {seed}"
            );
            // The escorts' event cites the event that sent them.
            let sent = events_where(&world, |k| matches!(k, EventKind::EscortsSent { .. }));
            let events = events_where(&world, |k| matches!(k, EventKind::AircraftWorn { .. }));
            let escort_event = events
                .iter()
                .find(|e| {
                    matches!(
                        e.kind,
                        EventKind::AircraftWorn {
                            sortie: SortieKind::Escort,
                            ..
                        }
                    )
                })
                .expect("escorts rolled");
            assert_eq!(escort_event.causes, vec![Cause::Event(sent[0].id)]);
        }
    }

    #[test]
    fn escorts_kept_home_by_the_radius_are_patrol_and_roll_once_at_distance_0() {
        let mut world = raid_world(1);
        base_group(
            &mut world,
            5,
            BASE_ALLIED,
            5,
            10,
            AirMission::CombatAirPatrol,
        );
        group_mut(&mut world, AirGroupId::new(5)).patrol_share = fixed(1, 2);
        // The raid's hex is 200 nm away, beyond a radius of 100 nm.
        set_fighter_radius(&mut world, 100);
        upkeep(&mut world, fixed(1, 2), Fixed::ONE, NEVER, 0);
        fly_and_wear(&mut world, Pulse::Morning);
        assert!(events_where(&world, |k| matches!(k, EventKind::EscortsSent { .. })).is_empty());
        let fighters: Vec<_> = worn(&world).into_iter().filter(|r| r.0 == 5).collect();
        assert_eq!(fighters.len(), 1);
        assert_eq!(
            (fighters[0].1, fighters[0].2, fighters[0].3, fighters[0].4),
            (SortieKind::Patrol, 0, 10, fixed(1, 2))
        );
    }

    #[test]
    fn a_group_aboard_a_ship_that_sank_in_the_pulse_rolls_nothing() {
        let mut world = world_with_a_good_contact(1);
        cap_group(&mut world, 10, Side::Japanese, 2, 6, Fixed::ONE);
        upkeep(&mut world, SURE, NEVER, NEVER, 0);
        // The strike sinks the Japanese carrier with the patrol aboard.
        group_mut(&mut world, BOMBERS).aircraft_type = AircraftTypeId::new(3);
        world
            .ref_data
            .aircraft_types
            .get_mut(&AircraftTypeId::new(3))
            .expect("type")
            .attack_damage = Fixed::from_int(600);
        air_figures(&mut world).torpedo_hit_chance = Fixed::ONE;
        fly_and_wear(&mut world, Pulse::Morning);
        assert_eq!(ship_of(&world, 2).location, ShipLocation::Sunk);
        let rolls = worn(&world);
        assert!(rolls.iter().all(|r| r.0 != 10), "{rolls:?}");
        assert!(rolls.iter().any(|r| r.0 == 1));
        assert_eq!(group_mut(&mut world, AirGroupId::new(10)).damaged, 0);
    }

    #[test]
    fn a_base_repairs_up_to_its_figure_the_lowest_group_first_and_only_groups_located_there() {
        let mut world = base_world(1);
        for id in [4, 5, 6] {
            base_group(&mut world, id, BASE_ALLIED, 2, 5, AirMission::Rest);
            group_mut(&mut world, AirGroupId::new(id)).damaged = 2;
        }
        // A group at the other base is not repaired by this one.
        base_group(&mut world, 7, BASE_JAPANESE, 2, 5, AirMission::Rest);
        group_mut(&mut world, AirGroupId::new(7)).damaged = 2;
        world
            .truth
            .bases
            .get_mut(&BASE_ALLIED)
            .expect("base")
            .aircraft_repairs_per_day = 3;
        group_mut(&mut world, AirGroupId::new(5)).damaged = 1;
        // Another group's wear is what the repair cites.
        let wear = world.events.record(
            1,
            Some(Pulse::Afternoon),
            EventKind::AircraftWorn {
                air_group: AirGroupId::new(4),
                sortie: SortieKind::Patrol,
                distance_nm: Fixed::ZERO,
                aircraft: 2,
                chance: Fixed::ONE,
                expected: Fixed::from_int(2),
                worn: 2,
                written_off: 0,
                ready: 5,
                damaged: 2,
            },
            vec![],
            None,
        );
        upkeep(&mut world, fixed(1, 10), NEVER, NEVER, 9);
        world.repair_aircraft(2, Pulse::Night);
        assert_eq!(repaired(&world), vec![(4, 2, 7, 0), (5, 1, 6, 0)]);
        let events = events_where(&world, |k| matches!(k, EventKind::AircraftRepaired { .. }));
        assert_eq!(events[0].causes, vec![Cause::Event(wear)]);
        assert!(events[1].causes.is_empty());
        assert_eq!(
            events[0].kind.to_string(),
            "air_group:4 had 2 aircraft repaired at base:1, 7 ready and 0 damaged"
        );
        assert_eq!(group_mut(&mut world, AirGroupId::new(6)).damaged, 2);
        assert_eq!(group_mut(&mut world, AirGroupId::new(7)).damaged, 2);
        // The next day the third group is next in line.
        world.repair_aircraft(3, Pulse::Night);
        assert_eq!(repaired(&world)[2..].to_vec(), vec![(6, 2, 7, 0)]);
    }

    #[test]
    fn a_carrier_repairs_its_groups_up_to_the_carrier_figure_unless_it_has_sunk() {
        let mut world = world(1, vec![]);
        group_mut(&mut world, BOMBERS).damaged = 3;
        group_mut(&mut world, SCOUTS).damaged = 4;
        group_mut(&mut world, ENEMY_BOMBERS).damaged = 5;
        upkeep(&mut world, NEVER, NEVER, NEVER, 5);
        world.repair_aircraft(1, Pulse::Night);
        // Ship 1 carries groups 1 and 2: 5 in all, group 1 first. Ship 2 has
        // 5 for its own group.
        assert_eq!(
            repaired(&world),
            vec![(1, 3, 21, 0), (2, 2, 10, 2), (3, 5, 23, 0)]
        );
        let events = events_where(&world, |k| matches!(k, EventKind::AircraftRepaired { .. }));
        assert_eq!(
            events[0].kind.to_string(),
            "air_group:1 had 3 aircraft repaired aboard ship:1, 21 ready and 0 damaged"
        );
        // A ship that sank repairs nothing.
        ship_mut(&mut world, 1).location = ShipLocation::Sunk;
        group_mut(&mut world, SCOUTS).damaged = 2;
        world.repair_aircraft(2, Pulse::Night);
        assert_eq!(repaired(&world).len(), 3);
    }

    /// Japanese base 2 with its groups: 3 on combat air patrol (ready 4,
    /// damaged 2), 4 at rest (ready 5, damaged 3), 5 searching (ready 2,
    /// damaged 1); and Allied bombers on a base strike with every bomb
    /// landing on aircraft.
    fn ground_world(bombers: u32) -> World {
        let mut world = raid_world(1);
        group_mut(&mut world, AirGroupId::new(1)).ready = bombers;
        for (id, type_id, ready, damaged, mission) in [
            (3, 5, 4, 2, AirMission::CombatAirPatrol),
            (4, 3, 5, 3, AirMission::Rest),
            (5, 1, 2, 1, AirMission::Search),
        ] {
            base_group(&mut world, id, BASE_JAPANESE, type_id, ready, mission);
            group_mut(&mut world, AirGroupId::new(id)).damaged = damaged;
            if mission == AirMission::Search {
                // A group on search flies only with an arc (decision 11 of
                // 0036: a group that does not fly is on the ground).
                group_mut(&mut world, AirGroupId::new(id)).search = Some(SearchArc {
                    from_deg: 0,
                    to_deg: 350,
                    range_nm: Fixed::from_int(300),
                });
            }
        }
        sure_bombs(&mut world, Fixed::ONE, Fixed::ZERO);
        // The fighters and the guns do nothing.
        air_figures(&mut world).fighter_hit_chance = Fixed::ZERO;
        world
    }

    fn ground_events(world: &World) -> Vec<(bool, u32, u32, u32)> {
        events_where(world, |k| {
            matches!(
                k,
                EventKind::AircraftDestroyedOnGround { .. }
                    | EventKind::DamagedAircraftDestroyedOnGround { .. }
            )
        })
        .iter()
        .map(|event| match event.kind {
            EventKind::DamagedAircraftDestroyedOnGround {
                air_group,
                aircraft,
                remaining,
                ..
            } => (true, air_group.raw(), aircraft, remaining),
            EventKind::AircraftDestroyedOnGround {
                air_group,
                aircraft,
                remaining,
                ..
            } => (false, air_group.raw(), aircraft, remaining),
            _ => unreachable!(),
        })
        .collect()
    }

    #[test]
    fn damaged_aircraft_are_on_the_ground_whatever_their_group_does_and_come_off_first() {
        // The ground: group 3's damaged 2, group 4's damaged 3 and ready 5,
        // group 5's damaged 1 (its ready aircraft are searching, group 3's
        // are on patrol). 18 hits destroy all 11, in that order.
        let mut world = ground_world(18);
        let strikes_before = world.strikes_in_flight.len();
        assert_eq!(strikes_before, 0);
        strike_pulse(&mut world, Pulse::Morning);
        assert_eq!(
            ground_events(&world),
            vec![
                (true, 3, 2, 0),
                (true, 4, 3, 0),
                (false, 4, 5, 0),
                (true, 5, 1, 0)
            ]
        );
        for (id, ready, damaged) in [(3, 4, 0), (4, 0, 0), (5, 2, 0)] {
            let group = group_mut(&mut world, AirGroupId::new(id));
            assert_eq!((group.ready, group.damaged), (ready, damaged), "group {id}");
        }
        // The breakdown says how many were on the ground and how many hits.
        let event = &events_where(&world, |k| {
            matches!(k, EventKind::DamagedAircraftDestroyedOnGround { .. })
        })[0];
        assert_eq!(
            event.kind.to_string(),
            "air_group:3 lost 2 damaged aircraft on the ground at base:2, 0 damaged left"
        );
        let names: Vec<_> = event
            .breakdown
            .as_ref()
            .expect("a breakdown")
            .factors
            .iter()
            .map(|f| (f.name, f.value))
            .collect();
        assert_eq!(
            names,
            vec![
                ("hits_on_aircraft", Fixed::from_int(18)),
                ("aircraft_on_ground", Fixed::from_int(11))
            ]
        );
    }

    #[test]
    fn four_hits_on_aircraft_destroy_group_3s_damaged_then_group_4s_damaged_and_no_more() {
        let mut world = ground_world(4);
        strike_pulse(&mut world, Pulse::Morning);
        assert_eq!(
            ground_events(&world),
            vec![(true, 3, 2, 0), (true, 4, 2, 1)]
        );
        let g4 = group_mut(&mut world, AirGroupId::new(4));
        assert_eq!((g4.ready, g4.damaged), (5, 1));
        // The aircraft worn after the raid are not on the ground in it: the
        // wear step comes after the air combat step.
        upkeep(&mut world, SURE, NEVER, NEVER, 0);
        world.wear_aircraft(1, Pulse::Morning);
        assert_eq!(ground_events(&world).len(), 2);
    }

    #[test]
    fn a_group_that_launched_has_its_damaged_aircraft_on_the_ground_but_not_its_ready_ones() {
        // Group 1's damaged aircraft at base 1 are on the ground there; the
        // raid on base 1 would destroy them.
        let mut world = raid_world(1);
        group_mut(&mut world, AirGroupId::new(1)).damaged = 3;
        base_group(
            &mut world,
            9,
            BASE_JAPANESE,
            2,
            18,
            AirMission::BaseStrike(BASE_ALLIED),
        );
        world.decide_strikes(1, Pulse::Morning);
        let ground = ground_of(&world);
        assert_eq!(
            ground[&BASE_ALLIED],
            vec![(AirGroupId::new(1), Stock::Damaged, 3)]
        );
    }

    #[test]
    fn a_group_lost_with_its_ship_loses_its_damaged_aircraft_too() {
        let mut world = world_with_a_good_contact(1);
        group_mut(&mut world, ENEMY_BOMBERS).damaged = 4;
        group_mut(&mut world, BOMBERS).aircraft_type = AircraftTypeId::new(3);
        world
            .ref_data
            .aircraft_types
            .get_mut(&AircraftTypeId::new(3))
            .expect("type")
            .attack_damage = Fixed::from_int(600);
        air_figures(&mut world).torpedo_hit_chance = Fixed::ONE;
        strike_pulse(&mut world, Pulse::Morning);
        let lost = events_where(&world, |k| matches!(k, EventKind::AirGroupLost { .. }));
        assert_eq!(
            lost[0].kind.to_string(),
            "air_group:3 lost with ship:2: 22 aircraft"
        );
        let group = group_mut(&mut world, ENEMY_BOMBERS);
        assert_eq!((group.ready, group.damaged), (0, 0));
    }
    // ---- fatigue and rest (decision 0036) ----

    use crate::events::FatigueChange;
    use crate::refdata::FatigueFigures;

    /// Fatigue figures of 3 a sortie, 2 for each 100 nm and 5 rest a pulse:
    /// those of `data/reference/fatigue.toml`.
    fn tire(world: &mut World) {
        world.ref_data.fatigue = Some(FatigueFigures {
            air_fatigue_per_sortie: Fixed::from_int(3),
            air_fatigue_per_100_nm: Fixed::from_int(2),
            air_rest_per_pulse: Fixed::from_int(5),
            force_fatigue_per_pulse_alert: Fixed::from_int(5),
            force_rest_per_pulse: Fixed::from_int(3),
        });
    }

    fn set_fatigue(world: &mut World, id: u32, fatigue: Fixed) {
        group_mut(world, AirGroupId::new(id)).fatigue = fatigue;
    }

    fn fatigue_of(world: &mut World, id: u32) -> Fixed {
        group_mut(world, AirGroupId::new(id)).fatigue
    }

    /// The `AirGroupFatigue` events as (group, change, amount, fatigue).
    fn fatigue_events(world: &World) -> Vec<(u32, FatigueChange, Fixed, Fixed)> {
        events_where(world, |k| matches!(k, EventKind::AirGroupFatigue { .. }))
            .iter()
            .map(|event| match event.kind {
                EventKind::AirGroupFatigue {
                    air_group,
                    change,
                    amount,
                    fatigue,
                } => (air_group.raw(), change, amount, fatigue),
                _ => unreachable!(),
            })
            .collect()
    }

    fn gained(sortie: SortieKind, nm: i64) -> FatigueChange {
        FatigueChange::Gained {
            sortie,
            distance_nm: Fixed::from_int(nm),
        }
    }

    fn able(world: &World, id: u32) -> u32 {
        world.aircraft_able_to_fly(&world.truth.air_groups[&AirGroupId::new(id)])
    }

    #[test]
    fn fatigue_cuts_the_aircraft_that_fly_and_a_degraded_deck_multiplies_it_with_one_rounding() {
        // 7 ready aboard a carrier whose deck is degraded (half): 3.5, times
        // 90 over 100, is 3.15: 3. Rounding the half first would give 2.
        let mut world = world_with_a_good_contact(1);
        tire(&mut world);
        group_mut(&mut world, BOMBERS).ready = 7;
        deck(&mut world, 1, SystemState::Degraded);
        set_fatigue(&mut world, BOMBERS.raw(), Fixed::from_int(10));
        assert_eq!(able(&world, BOMBERS.raw()), 3);
        // The same group, deck working: 7 x 0.9 = 6.3: 6; deck out: none.
        deck(&mut world, 1, SystemState::Working);
        assert_eq!(able(&world, BOMBERS.raw()), 6);
        deck(&mut world, 1, SystemState::Out);
        assert_eq!(able(&world, BOMBERS.raw()), 0);
        // The cut reaches the strike decision.
        deck(&mut world, 1, SystemState::Degraded);
        world.decide_strikes(1, Pulse::Morning);
        assert_eq!(launch_of(&launches(&world)[0]).3, 3);
    }

    #[test]
    fn a_group_flies_its_ready_aircraft_times_100_less_its_fatigue_over_100_rounded_down() {
        let mut world = base_world(1);
        base_group(&mut world, 2, BASE_ALLIED, 1, 6, AirMission::Search);
        tire(&mut world);
        let flies = |world: &mut World, ready: u32, fatigue: Fixed| {
            group_mut(world, AirGroupId::new(2)).ready = ready;
            set_fatigue(world, 2, fatigue);
            able(world, 2)
        };
        let n = Fixed::from_int;
        // 6 scouts at fatigue 1 fly 5.
        assert_eq!(flies(&mut world, 6, n(0)), 6);
        assert_eq!(flies(&mut world, 6, n(1)), 5);
        assert_eq!(flies(&mut world, 6, n(30)), 4);
        assert_eq!(flies(&mut world, 6, n(85)), 0);
        assert_eq!(flies(&mut world, 6, n(100)), 0);
        assert_eq!(flies(&mut world, 0, n(0)), 0);
        // A fatigue with decimals: 5 x 93.8 / 100 = 4.69.
        assert_eq!(
            flies(&mut world, 5, "6.2".parse::<Fixed>().expect("fixed")),
            4
        );
        // It never rises with the fatigue.
        for ready in 0..=30 {
            let mut last = u32::MAX;
            for fatigue in 0..=100 {
                let now = flies(&mut world, ready, n(fatigue));
                assert!(now <= last && now <= ready, "{ready} at {fatigue}");
                last = now;
            }
        }
        // Without the figures fatigue cuts nothing, and nobody is stood down.
        world.ref_data.fatigue = None;
        group_mut(&mut world, AirGroupId::new(2)).stood_down = true;
        assert_eq!(flies(&mut world, 6, n(50)), 6);
        // With them a stood-down group flies none, tired or not.
        tire(&mut world);
        assert_eq!(flies(&mut world, 6, n(0)), 0);
    }

    #[test]
    fn each_kind_of_sortie_gains_fatigue_for_its_distance_citing_what_says_it_flew() {
        // A search out to 300 nm: 600 nm, 3 + 2 x 6 = 15.
        let mut world = base_world(1);
        base_group(&mut world, 2, BASE_ALLIED, 1, 6, AirMission::Search);
        group_mut(&mut world, AirGroupId::new(2)).search = Some(SearchArc {
            from_deg: 0,
            to_deg: 350,
            range_nm: Fixed::from_int(300),
        });
        tire(&mut world);
        fly_and_wear(&mut world, Pulse::Morning);
        assert_eq!(
            fatigue_events(&world),
            vec![(
                2,
                gained(SortieKind::Search, 600),
                Fixed::from_int(15),
                Fixed::from_int(15)
            )]
        );
        let search = events_where(&world, |k| matches!(k, EventKind::SearchFlown { .. }))[0].id;
        let event = &events_where(&world, |k| matches!(k, EventKind::AirGroupFatigue { .. }))[0];
        assert_eq!(event.causes, vec![Cause::Event(search)]);
        assert_eq!(
            event.kind.to_string(),
            "air_group:2 gained 15 fatigue from its search sortie of 600 nm, now 15"
        );

        // A strike at a contact 200 nm away: 400 nm, 11.
        let mut world = world_with_a_good_contact(1);
        tire(&mut world);
        strike_pulse(&mut world, Pulse::Morning);
        world.wear_aircraft(1, Pulse::Morning);
        let bombers = fatigue_events(&world)
            .into_iter()
            .filter(|event| event.0 == BOMBERS.raw())
            .collect::<Vec<_>>();
        assert_eq!(
            bombers,
            vec![(
                BOMBERS.raw(),
                gained(SortieKind::Strike, 400),
                Fixed::from_int(11),
                Fixed::from_int(11)
            )]
        );

        // A strike at a base 200 nm away: 11.
        let mut world = raid_world(1);
        tire(&mut world);
        strike_pulse(&mut world, Pulse::Morning);
        world.wear_aircraft(1, Pulse::Morning);
        assert_eq!(
            fatigue_events(&world),
            vec![(
                1,
                gained(SortieKind::BaseStrike, 400),
                Fixed::from_int(11),
                Fixed::from_int(11)
            )]
        );
        let launch = base_launches(&world)[0].id;
        let event = &events_where(&world, |k| matches!(k, EventKind::AirGroupFatigue { .. }))[0];
        assert_eq!(event.causes, vec![Cause::Event(launch)]);

        // A patrol with no raid: no distance, 3, citing the group's orders.
        let mut world = base_world(1);
        base_group(
            &mut world,
            5,
            BASE_ALLIED,
            5,
            10,
            AirMission::CombatAirPatrol,
        );
        group_mut(&mut world, AirGroupId::new(5)).mission_order = Some(OrderId::new(3));
        tire(&mut world);
        fly_and_wear(&mut world, Pulse::Morning);
        assert_eq!(
            fatigue_events(&world),
            vec![(
                5,
                gained(SortieKind::Patrol, 0),
                Fixed::from_int(3),
                Fixed::from_int(3)
            )]
        );
        let event = &events_where(&world, |k| matches!(k, EventKind::AirGroupFatigue { .. }))[0];
        assert_eq!(event.causes, vec![Cause::Order(OrderId::new(3))]);
    }

    #[test]
    fn a_fighter_group_that_kept_a_patrol_and_sent_escorts_gains_for_the_longer_once() {
        let mut world = raid_world(1);
        base_group(
            &mut world,
            5,
            BASE_ALLIED,
            5,
            10,
            AirMission::CombatAirPatrol,
        );
        group_mut(&mut world, AirGroupId::new(5)).patrol_share = fixed(1, 2);
        group_mut(&mut world, AirGroupId::new(5)).mission_order = Some(OrderId::new(3));
        tire(&mut world);
        strike_pulse(&mut world, Pulse::Morning);
        world.wear_aircraft(1, Pulse::Morning);
        let sent = events_where(&world, |k| matches!(k, EventKind::EscortsSent { .. }))[0].id;
        let events: Vec<_> = fatigue_events(&world)
            .into_iter()
            .filter(|event| event.0 == 5)
            .collect();
        // 11 for the escorts' 400 nm, once, not 3 + 11.
        assert_eq!(
            events,
            vec![(
                5,
                gained(SortieKind::Escort, 400),
                Fixed::from_int(11),
                Fixed::from_int(11)
            )]
        );
        let event = events_where(&world, |k| matches!(k, EventKind::AirGroupFatigue { .. }))
            .into_iter()
            .find(|event| matches!(event.kind, EventKind::AirGroupFatigue { air_group, .. } if air_group.raw() == 5))
            .expect("the fighters' event");
        // Both sorties say it flew: the patrol's order, the escorts' event.
        assert_eq!(
            event.causes,
            vec![Cause::Order(OrderId::new(3)), Cause::Event(sent)]
        );
    }

    #[test]
    fn a_group_whose_aircraft_were_all_shot_down_still_gained_for_the_sortie() {
        let mut world = raid_world(1);
        // 30 defending fighters, sure to hit: the 18 bombers all fall.
        base_group(
            &mut world,
            9,
            BASE_JAPANESE,
            5,
            30,
            AirMission::CombatAirPatrol,
        );
        air_figures(&mut world).fighter_hit_chance = Fixed::ONE;
        tire(&mut world);
        strike_pulse(&mut world, Pulse::Morning);
        assert_eq!(ready_of(&world, 1), 0, "every bomber was shot down");
        assert!(base_attacks(&world).is_empty());
        world.wear_aircraft(1, Pulse::Morning);
        let events: Vec<_> = fatigue_events(&world)
            .into_iter()
            .filter(|event| event.0 == 1)
            .collect();
        assert_eq!(
            events,
            vec![(
                1,
                gained(SortieKind::BaseStrike, 400),
                Fixed::from_int(11),
                Fixed::from_int(11)
            )]
        );
    }

    #[test]
    fn rest_never_goes_below_0_and_a_gain_never_passes_100_and_neither_records_nothing_at_the_limit(
    ) {
        // Resting: 3 loses 3 (not 5), then at 0 nothing is recorded.
        let mut world = base_world(1);
        base_group(&mut world, 4, BASE_ALLIED, 2, 10, AirMission::Rest);
        group_mut(&mut world, AirGroupId::new(4)).mission_order = Some(OrderId::new(2));
        tire(&mut world);
        set_fatigue(&mut world, 4, Fixed::from_int(3));
        world.wear_aircraft(1, Pulse::Morning);
        assert_eq!(
            fatigue_events(&world),
            vec![(4, FatigueChange::Rested, Fixed::from_int(3), Fixed::ZERO)]
        );
        // With no stand-down the cause is the mission order.
        let event = &events_where(&world, |k| matches!(k, EventKind::AirGroupFatigue { .. }))[0];
        assert_eq!(event.causes, vec![Cause::Order(OrderId::new(2))]);
        world.wear_aircraft(1, Pulse::Afternoon);
        assert_eq!(fatigue_events(&world).len(), 1);
        assert_eq!(fatigue_of(&mut world, 4), Fixed::ZERO);

        // Flying: 98 + 3 stops at 100 (a gain of 2). It can fly only 2 of 100.
        let mut world = base_world(1);
        base_group(
            &mut world,
            5,
            BASE_ALLIED,
            5,
            100,
            AirMission::CombatAirPatrol,
        );
        tire(&mut world);
        set_fatigue(&mut world, 5, Fixed::from_int(98));
        fly_and_wear(&mut world, Pulse::Morning);
        assert_eq!(
            fatigue_events(&world),
            vec![(
                5,
                gained(SortieKind::Patrol, 0),
                Fixed::from_int(2),
                Fixed::from_int(100)
            )]
        );
        // At 100 it flies nothing (100 x 0 = 0) and rests: 95.
        fly_and_wear(&mut world, Pulse::Afternoon);
        assert_eq!(fatigue_of(&mut world, 5), Fixed::from_int(95));
    }

    #[test]
    fn a_group_stands_down_above_its_threshold_and_returns_at_or_under_it() {
        let mut world = base_world(1);
        base_group(
            &mut world,
            5,
            BASE_ALLIED,
            5,
            10,
            AirMission::CombatAirPatrol,
        );
        base_group(
            &mut world,
            6,
            BASE_ALLIED,
            5,
            10,
            AirMission::CombatAirPatrol,
        );
        {
            let group = group_mut(&mut world, AirGroupId::new(5));
            group.rest_above = Some(Fixed::from_int(30));
            group.rest_order = Some(OrderId::new(9));
        }
        tire(&mut world);
        let standing = |world: &World| {
            events_where(world, |k| {
                matches!(
                    k,
                    EventKind::AirGroupStoodDown { .. } | EventKind::AirGroupReturned { .. }
                )
            })
        };
        // Equal to the threshold is not above it.
        set_fatigue(&mut world, 5, Fixed::from_int(30));
        set_fatigue(&mut world, 6, Fixed::from_int(100));
        world.stand_down_and_return_air_groups(2, Pulse::Night);
        assert!(standing(&world).is_empty());
        assert!(!group_mut(&mut world, AirGroupId::new(5)).stood_down);
        // A hair above is. Group 6 has no threshold and is never stood down.
        set_fatigue(&mut world, 5, Fixed::from_raw(30_000_001));
        world.stand_down_and_return_air_groups(3, Pulse::Night);
        let events = standing(&world);
        assert_eq!(events.len(), 1);
        assert_eq!(
            events[0].kind,
            EventKind::AirGroupStoodDown {
                air_group: AirGroupId::new(5),
                fatigue: Fixed::from_raw(30_000_001),
                threshold: Fixed::from_int(30),
            }
        );
        assert_eq!(events[0].causes, vec![Cause::Order(OrderId::new(9))]);
        assert!(group_mut(&mut world, AirGroupId::new(5)).stood_down);
        assert!(!group_mut(&mut world, AirGroupId::new(6)).stood_down);
        // Still above: nothing more is recorded.
        world.stand_down_and_return_air_groups(4, Pulse::Night);
        assert_eq!(standing(&world).len(), 1);
        // Back to the threshold: returned.
        set_fatigue(&mut world, 5, Fixed::from_int(30));
        world.stand_down_and_return_air_groups(5, Pulse::Night);
        let events = standing(&world);
        assert_eq!(events.len(), 2);
        assert_eq!(
            events[1].kind,
            EventKind::AirGroupReturned {
                air_group: AirGroupId::new(5),
                fatigue: Fixed::from_int(30),
                threshold: Fixed::from_int(30),
            }
        );
        assert_eq!(events[1].causes, vec![Cause::Order(OrderId::new(9))]);
        assert!(!group_mut(&mut world, AirGroupId::new(5)).stood_down);
        // Without the figures nobody is stood down.
        world.ref_data.fatigue = None;
        set_fatigue(&mut world, 5, Fixed::from_int(90));
        world.stand_down_and_return_air_groups(6, Pulse::Night);
        assert_eq!(standing(&world).len(), 2);
    }

    #[test]
    fn a_stood_down_group_flies_nothing_keeps_its_mission_rests_and_is_on_the_ground() {
        let mut world = raid_world(1);
        // Scouts at the raided base: stood down, they send none and sit on
        // the ground; not stood down, they search and are in the air.
        base_group(&mut world, 20, BASE_JAPANESE, 1, 6, AirMission::Search);
        group_mut(&mut world, AirGroupId::new(20)).search = Some(SearchArc {
            from_deg: 0,
            to_deg: 350,
            range_nm: Fixed::from_int(300),
        });
        tire(&mut world);
        world.decide_strikes(1, Pulse::Morning);
        assert_eq!(ground_of(&world).get(&BASE_JAPANESE), Some(&Vec::new()));
        group_mut(&mut world, AirGroupId::new(20)).stood_down = true;
        set_fatigue(&mut world, 20, Fixed::from_int(60));
        assert_eq!(
            ground_of(&world)[&BASE_JAPANESE],
            vec![(AirGroupId::new(20), Stock::Ready, 6)]
        );
        world.search_from_the_air(1, Pulse::Morning);
        assert!(events_where(&world, |k| matches!(k, EventKind::SearchFlown { .. })).is_empty());
        assert_eq!(
            group_mut(&mut world, AirGroupId::new(20)).mission,
            AirMission::Search
        );
        // It did not fly, so it rests the figure, citing its stand-down.
        world.stand_down_and_return_air_groups(1, Pulse::Night);
        world.strikes_in_flight.clear();
        world.wear_aircraft(1, Pulse::Morning);
        assert_eq!(
            fatigue_events(&world),
            vec![(
                20,
                FatigueChange::Rested,
                Fixed::from_int(5),
                Fixed::from_int(55)
            )]
        );
        // A stood-down group's patrol and escorts are none either.
        let mut world = base_world(1);
        base_group(
            &mut world,
            5,
            BASE_ALLIED,
            5,
            10,
            AirMission::CombatAirPatrol,
        );
        tire(&mut world);
        group_mut(&mut world, AirGroupId::new(5)).stood_down = true;
        let fighters = world.split_fighter_groups();
        assert_eq!((fighters[0].patrol, fighters[0].escorts), (0, 0));
    }

    #[test]
    fn the_ground_holds_the_ready_aircraft_not_flying_and_the_damaged_ones() {
        let mut world = raid_world(1);
        tire(&mut world);
        // A tired fighter group on patrol (share 1) at the raided base: 12
        // ready at fatigue 50 patrol 6 and leave 6 on the ground.
        base_group(
            &mut world,
            20,
            BASE_JAPANESE,
            5,
            12,
            AirMission::CombatAirPatrol,
        );
        set_fatigue(&mut world, 20, Fixed::from_int(50));
        group_mut(&mut world, AirGroupId::new(20)).damaged = 2;
        // Half of a fighter group with no raid at its place stays on the
        // ground: share 0.5 of 8, 4 patrol, the other 4 do not fly.
        base_group(
            &mut world,
            21,
            BASE_JAPANESE,
            5,
            8,
            AirMission::CombatAirPatrol,
        );
        group_mut(&mut world, AirGroupId::new(21)).patrol_share = fixed(1, 2);
        // A group that flies everything is not on the ground at all.
        base_group(
            &mut world,
            22,
            BASE_JAPANESE,
            5,
            5,
            AirMission::CombatAirPatrol,
        );
        world.decide_strikes(1, Pulse::Morning);
        assert_eq!(
            ground_of(&world)[&BASE_JAPANESE],
            vec![
                (AirGroupId::new(20), Stock::Damaged, 2),
                (AirGroupId::new(20), Stock::Ready, 6),
                (AirGroupId::new(21), Stock::Ready, 4),
            ]
        );
    }

    #[test]
    fn without_the_figures_nothing_is_noted_recorded_or_changed_by_fatigue() {
        let mut world = raid_world(1);
        base_group(
            &mut world,
            5,
            BASE_ALLIED,
            5,
            10,
            AirMission::CombatAirPatrol,
        );
        // A threshold is stored and does nothing; the group's own fatigue is
        // ignored.
        {
            let group = group_mut(&mut world, AirGroupId::new(5));
            group.rest_above = Some(Fixed::ZERO);
            group.fatigue = Fixed::from_int(80);
        }
        for pulse in Pulse::ALL {
            world.stand_down_and_return_air_groups(1, pulse);
            fly_and_wear(&mut world, pulse);
            assert!(world.sorties.is_empty());
        }
        assert!(fatigue_events(&world).is_empty());
        assert!(!group_mut(&mut world, AirGroupId::new(5)).stood_down);
        assert_eq!(fatigue_of(&mut world, 5), Fixed::from_int(80));
        assert_eq!(able(&world, 5), 10);
        // A group with no fatigue state hashes as it did before fatigue.
        let plain = raid_world(1);
        let mut same = raid_world(1);
        assert_eq!(plain.state_hash(), same.state_hash());
        set_fatigue(&mut same, 1, Fixed::from_int(1));
        assert_ne!(plain.state_hash(), same.state_hash());
        set_fatigue(&mut same, 1, Fixed::ZERO);
        assert_eq!(plain.state_hash(), same.state_hash());
        group_mut(&mut same, AirGroupId::new(1)).rest_above = Some(Fixed::ZERO);
        assert_ne!(plain.state_hash(), same.state_hash());
        group_mut(&mut same, AirGroupId::new(1)).rest_above = None;
        group_mut(&mut same, AirGroupId::new(1)).stood_down = true;
        assert_ne!(plain.state_hash(), same.state_hash());
    }

    #[test]
    fn the_rest_threshold_order_is_accepted_stored_and_rejected_out_of_range() {
        let order = |fatigue: i64| OrderRequest {
            kind: OrderKind::AirGroup {
                target: BOMBERS,
                order: AirGroupOrder::SetRestThreshold {
                    fatigue: Fixed::from_int(fatigue),
                },
            },
            origin: Origin::PLAYER,
        };
        // Stored without the figures too.
        let mut world = self::world(1, vec![]);
        world.submit_orders(Side::Allied, vec![order(30), order(100), order(0)]);
        world.resolve_day().expect("day resolves");
        assert!(rejections(&world).is_empty());
        assert_eq!(group_mut(&mut world, BOMBERS).rest_above, Some(Fixed::ZERO));
        assert!(group_mut(&mut world, BOMBERS).rest_order.is_some());
        let mut world = self::world(1, vec![]);
        world.submit_orders(Side::Allied, vec![order(101), order(-1)]);
        world.resolve_day().expect("day resolves");
        assert_eq!(
            rejections(&world),
            vec![
                RejectReason::FatigueOutOfRange(Fixed::from_int(101)),
                RejectReason::FatigueOutOfRange(Fixed::from_int(-1))
            ]
        );
        assert_eq!(group_mut(&mut world, BOMBERS).rest_above, None);
        // An enemy group is not yours.
        let mut world = self::world(1, vec![]);
        world.submit_orders(Side::Japanese, vec![order(30)]);
        world.resolve_day().expect("day resolves");
        assert_eq!(rejections(&world), vec![RejectReason::NotYourUnit]);
    }

    // ---- pilots and replacements (decision 0038) ----

    use crate::state::Replacement;

    fn pilots_of(world: &World, id: u32) -> Option<u32> {
        world.truth.air_groups[&AirGroupId::new(id)].pilots
    }

    fn pilot_losses(world: &World) -> Vec<(u32, u32, u32)> {
        events_where(world, |k| matches!(k, EventKind::PilotsLost { .. }))
            .iter()
            .map(|event| match event.kind {
                EventKind::PilotsLost {
                    air_group,
                    lost,
                    left,
                } => (air_group.raw(), lost, left),
                _ => unreachable!(),
            })
            .collect()
    }

    /// A group at base 2 (Japanese) of `type_id` with `ready` aircraft,
    /// `pilots` and a `strength`.
    fn crewed(
        world: &mut World,
        id: u32,
        type_id: u32,
        ready: u32,
        pilots: Option<u32>,
        strength: u32,
    ) {
        base_group(world, id, BASE_JAPANESE, type_id, ready, AirMission::Rest);
        let group = group_mut(world, AirGroupId::new(id));
        group.pilots = pilots;
        group.strength = strength;
    }

    fn stream(base: BaseId, side: Side, type_id: u32, aircraft: u32, pilots: u32) -> Replacement {
        Replacement {
            side,
            base,
            aircraft_type: AircraftTypeId::new(type_id),
            aircraft_per_day: aircraft,
            pilots_per_day: pilots,
        }
    }

    /// (air group, aircraft, pilots, ready, pilots now) of each
    /// `ReplacementsReceived`.
    fn received(world: &World) -> Vec<(u32, u32, u32, u32, u32)> {
        events_where(world, |k| {
            matches!(k, EventKind::ReplacementsReceived { .. })
        })
        .iter()
        .map(|event| match event.kind {
            EventKind::ReplacementsReceived {
                air_group,
                aircraft,
                pilots,
                ready,
                pilots_now,
                ..
            } => (air_group.raw(), aircraft, pilots, ready, pilots_now),
            _ => unreachable!(),
        })
        .collect()
    }

    #[test]
    fn a_group_flies_the_smaller_of_its_ready_aircraft_and_its_pilots_cut_once_by_deck_and_fatigue()
    {
        let mut world = world_with_a_good_contact(1);
        tire(&mut world);
        let id = BOMBERS.raw();
        group_mut(&mut world, BOMBERS).ready = 7;
        // Not tracked: as before.
        assert_eq!(able(&world, id), 7);
        // More pilots than aircraft: the aircraft.
        group_mut(&mut world, BOMBERS).pilots = Some(9);
        assert_eq!(able(&world, id), 7);
        // Fewer: the pilots. None: none.
        group_mut(&mut world, BOMBERS).pilots = Some(5);
        assert_eq!(able(&world, id), 5);
        group_mut(&mut world, BOMBERS).pilots = Some(0);
        assert_eq!(able(&world, id), 0);
        // 5 crewed, deck degraded (half) and fatigue 10: 5 x 0.5 x 0.9 =
        // 2.25, rounded once: 2. Rounding the half first would give 1.
        group_mut(&mut world, BOMBERS).pilots = Some(5);
        deck(&mut world, 1, SystemState::Degraded);
        set_fatigue(&mut world, id, Fixed::from_int(10));
        assert_eq!(able(&world, id), 2);
        // With 9 pilots the 7 ready: 3.15, 3 (as without pilots).
        group_mut(&mut world, BOMBERS).pilots = Some(9);
        assert_eq!(able(&world, id), 3);
        // The cut reaches the strike decision, the search and the split.
        group_mut(&mut world, BOMBERS).pilots = Some(5);
        world.decide_strikes(1, Pulse::Morning);
        assert_eq!(launch_of(&launches(&world)[0]).3, 2);
    }

    #[test]
    fn the_search_and_the_fighter_split_use_what_the_pilots_allow() {
        let mut world = base_world(1);
        base_group(&mut world, 2, BASE_ALLIED, 1, 6, AirMission::Search);
        group_mut(&mut world, AirGroupId::new(2)).search = Some(SearchArc {
            from_deg: 0,
            to_deg: 350,
            range_nm: Fixed::from_int(300),
        });
        group_mut(&mut world, AirGroupId::new(2)).pilots = Some(4);
        base_group(
            &mut world,
            3,
            BASE_ALLIED,
            5,
            10,
            AirMission::CombatAirPatrol,
        );
        group_mut(&mut world, AirGroupId::new(3)).pilots = Some(6);
        group_mut(&mut world, AirGroupId::new(3)).patrol_share = fixed(1, 2);
        world.search_from_the_air(1, Pulse::Morning);
        let flown = events_where(&world, |k| matches!(k, EventKind::SearchFlown { .. }));
        assert_eq!(flown.len(), 1);
        match flown[0].kind {
            EventKind::SearchFlown { aircraft, .. } => assert_eq!(aircraft, 4),
            _ => unreachable!(),
        }
        // 6 crewed fighters: half patrol, half escort; not 10.
        let fighters = world.split_fighter_groups();
        assert_eq!((fighters[0].patrol, fighters[0].escorts), (3, 3));
    }

    #[test]
    fn an_aircraft_lost_away_from_home_takes_its_pilot_and_a_patrol_aircraft_lost_at_home_does_not()
    {
        let mut world = base_world(1);
        crewed(&mut world, 10, 5, 12, Some(12), 12);
        crewed(&mut world, 11, 5, 12, None, 12);
        let cause = EventId::new(1);
        // A bomber or a scout lost on a strike, and an escort: the pilot is
        // lost with each.
        for (kind, lost, ready, pilots) in [
            (SortieKind::BaseStrike, 3, 9, 9),
            (SortieKind::Strike, 1, 8, 8),
            (SortieKind::Escort, 2, 6, 6),
            (SortieKind::Search, 1, 5, 5),
            // A patrol over its own hex: the pilot is saved.
            (SortieKind::Patrol, 2, 3, 5),
        ] {
            let (event, taken) = world
                .lose_aircraft(1, Pulse::Morning, AirGroupId::new(10), lost, kind, cause)
                .expect("aircraft were lost");
            assert_eq!(taken, lost);
            let group = &world.truth.air_groups[&AirGroupId::new(10)];
            assert_eq!(
                (group.ready, group.pilots),
                (ready, Some(pilots)),
                "{kind:?}"
            );
            let log = world.events.iter().collect::<Vec<_>>();
            let last = log.last().expect("an event");
            if kind == SortieKind::Patrol {
                // Nothing after the loss.
                assert_eq!(last.id, event);
                assert!(matches!(last.kind, EventKind::AircraftLost { .. }));
            } else {
                assert_eq!(last.causes, vec![Cause::Event(event)]);
                assert_eq!(
                    last.kind,
                    EventKind::PilotsLost {
                        air_group: AirGroupId::new(10),
                        lost,
                        left: pilots
                    }
                );
            }
        }
        // The AircraftLost event reads as it did.
        let first = events_where(&world, |k| matches!(k, EventKind::AircraftLost { .. }));
        assert_eq!(
            first[0].kind.to_string(),
            "air_group:10 lost 3 aircraft, 9 left"
        );
        assert_eq!(first[0].causes, vec![Cause::Event(cause)]);
        let texts: Vec<String> =
            events_where(&world, |k| matches!(k, EventKind::PilotsLost { .. }))
                .iter()
                .map(|event| event.kind.to_string())
                .collect();
        assert_eq!(texts[0], "air_group:10 lost 3 pilots, 9 left");
        assert_eq!(texts[1], "air_group:10 lost 1 pilot, 8 left");
        // A group whose pilots are not tracked loses none and records none.
        world
            .lose_aircraft(
                1,
                Pulse::Morning,
                AirGroupId::new(11),
                4,
                SortieKind::Escort,
                cause,
            )
            .expect("lost");
        assert_eq!(pilots_of(&world, 11), None);
        assert_eq!(pilot_losses(&world).len(), 4);
        // Never below 0: 3 ready and 1 pilot, 3 shot down.
        crewed(&mut world, 12, 5, 3, Some(1), 3);
        world
            .lose_aircraft(
                1,
                Pulse::Morning,
                AirGroupId::new(12),
                3,
                SortieKind::Escort,
                cause,
            )
            .expect("lost");
        assert_eq!(pilots_of(&world, 12), Some(0));
        assert_eq!(pilot_losses(&world).last(), Some(&(12, 1, 0)));
        // A group with nothing to lose records nothing.
        let before = world.events.len();
        assert!(world
            .lose_aircraft(
                1,
                Pulse::Morning,
                AirGroupId::new(12),
                2,
                SortieKind::Escort,
                cause
            )
            .is_none());
        assert_eq!(world.events.len(), before);
    }

    #[test]
    fn in_a_raid_the_bombers_and_escorts_shot_down_lose_their_pilots_and_the_patrol_keeps_its_own()
    {
        // Every fighter shot hits. Allied bombers (6) and escorts (4) raid
        // Japanese base 2, where 5 fighters are on patrol.
        let mut world = raid_world(1);
        sure_fighters(&mut world);
        group_mut(&mut world, AirGroupId::new(1)).ready = 6;
        group_mut(&mut world, AirGroupId::new(1)).pilots = Some(6);
        cap_group(&mut world, 2, Side::Allied, 1, 4, Fixed::ZERO);
        {
            let escorts = group_mut(&mut world, AirGroupId::new(2));
            escorts.location = AirGroupLocation::Base(BASE_ALLIED);
            escorts.pilots = Some(4);
        }
        cap_group(&mut world, 3, Side::Japanese, 2, 5, Fixed::ONE);
        {
            let patrol = group_mut(&mut world, AirGroupId::new(3));
            patrol.location = AirGroupLocation::Base(BASE_JAPANESE);
            patrol.pilots = Some(5);
        }
        strike_pulse(&mut world, Pulse::Morning);
        // The escorts' 4 shots bring down 4 of the patrol, whose pilots are
        // saved; the patrol's 4 shots at the escorts bring down all 4, and
        // the one left shoots a bomber: their pilots are lost.
        let lost = losses(&world);
        let per_group = |group: u32| -> u32 {
            lost.iter()
                .map(loss_of)
                .filter(|(g, ..)| *g == group)
                .map(|(_, aircraft, _)| aircraft)
                .sum()
        };
        assert_eq!(per_group(3), 4);
        assert_eq!(per_group(2), 4);
        assert_eq!(per_group(1), 1);
        assert_eq!(pilots_of(&world, 3), Some(5));
        assert_eq!(pilots_of(&world, 2), Some(0));
        assert_eq!(pilots_of(&world, 1), Some(5));
        assert_eq!(group_mut(&mut world, AirGroupId::new(3)).ready, 1);
        // Each PilotsLost follows the AircraftLost it is caused by.
        let log: Vec<Event> = world.events.iter().cloned().collect();
        let mut seen = 0;
        for (index, event) in log.iter().enumerate() {
            if let EventKind::PilotsLost {
                air_group, lost, ..
            } = event.kind
            {
                seen += 1;
                assert_ne!(air_group.raw(), 3);
                let before = &log[index - 1];
                assert_eq!(event.causes, vec![Cause::Event(before.id)]);
                match before.kind {
                    EventKind::AircraftLost {
                        air_group: g,
                        aircraft,
                        ..
                    } => assert_eq!((g, aircraft), (air_group, lost)),
                    _ => panic!("a pilot loss follows an aircraft loss"),
                }
            }
        }
        assert_eq!(seen, 2);
    }

    #[test]
    fn aircraft_written_off_after_a_sortie_and_aircraft_destroyed_on_the_ground_lose_no_pilot() {
        let mut world = ground_world(18);
        for (id, pilots) in [(1, 18), (3, 4), (4, 5), (5, 2)] {
            group_mut(&mut world, AirGroupId::new(id)).pilots = Some(pilots);
        }
        // Every aircraft that flies is worn and written off after the raid.
        upkeep(&mut world, SURE, NEVER, SURE, 0);
        fly_and_wear(&mut world, Pulse::Morning);
        assert!(!ground_events(&world).is_empty());
        let written_off: u32 =
            events_where(&world, |k| matches!(k, EventKind::AircraftWorn { .. }))
                .iter()
                .map(|event| match event.kind {
                    EventKind::AircraftWorn { written_off, .. } => written_off,
                    _ => unreachable!(),
                })
                .sum();
        assert!(written_off > 0);
        assert!(pilot_losses(&world).is_empty());
        for (id, pilots) in [(1, 18), (3, 4), (4, 5), (5, 2)] {
            assert_eq!(pilots_of(&world, id), Some(pilots), "group {id}");
        }
        // The groups really did lose aircraft.
        assert!(group_mut(&mut world, AirGroupId::new(4)).ready < 5);
        assert!(group_mut(&mut world, AirGroupId::new(1)).ready < 18);
    }

    #[test]
    fn a_group_lost_with_its_ship_loses_all_its_pilots_and_says_so() {
        let mut world = world_with_a_good_contact(1);
        group_mut(&mut world, BOMBERS).aircraft_type = AircraftTypeId::new(3);
        world
            .ref_data
            .aircraft_types
            .get_mut(&AircraftTypeId::new(3))
            .expect("type")
            .attack_damage = Fixed::from_int(600);
        air_figures(&mut world).torpedo_hit_chance = Fixed::ONE;
        group_mut(&mut world, ENEMY_BOMBERS).pilots = Some(14);
        // A second group aboard the sinking carrier whose pilots are not
        // tracked.
        let mut other = group(AirGroupId::new(30), Side::Japanese, 2, 5, 3);
        other.pilots = None;
        world.truth.air_groups.insert(other.id, other);
        strike_pulse(&mut world, Pulse::Morning);
        assert_eq!(ship_of(&world, 2).location, ShipLocation::Sunk);
        assert_eq!(pilots_of(&world, ENEMY_BOMBERS.raw()), Some(0));
        assert_eq!(pilots_of(&world, 30), None);
        assert_eq!(pilot_losses(&world), vec![(ENEMY_BOMBERS.raw(), 14, 0)]);
        let lost = events_where(&world, |k| matches!(k, EventKind::AirGroupLost { .. }));
        let pilots = events_where(&world, |k| matches!(k, EventKind::PilotsLost { .. }));
        assert_eq!(pilots[0].causes, vec![Cause::Event(lost[0].id)]);
        assert_eq!(pilots[0].id.raw(), lost[0].id.raw() + 1);
    }

    #[test]
    fn replacements_go_to_the_groups_of_the_type_at_the_base_lowest_first_each_up_to_its_strength()
    {
        let mut world = base_world(1);
        // Two fighter groups at base 2: group 10 is 4 aircraft and 1 pilot
        // short, group 11 is 1 aircraft and 3 pilots short.
        crewed(&mut world, 10, 5, 8, Some(11), 12);
        crewed(&mut world, 11, 5, 11, Some(9), 12);
        // Others get nothing: another type here, and the same type at the
        // other base.
        crewed(&mut world, 12, 2, 1, Some(0), 12);
        base_group(&mut world, 13, BASE_ALLIED, 5, 1, AirMission::Rest);
        group_mut(&mut world, AirGroupId::new(13)).strength = 12;
        group_mut(&mut world, AirGroupId::new(13)).pilots = Some(0);
        world
            .truth
            .replacements
            .push(stream(BASE_JAPANESE, Side::Japanese, 5, 5, 3));
        let hash = world.state_hash();
        world.receive_replacements(2, Pulse::Night);
        assert_ne!(hash, world.state_hash());
        assert_eq!(
            received(&world),
            vec![(10, 4, 1, 12, 12), (11, 1, 2, 12, 11)]
        );
        let events = events_where(&world, |k| {
            matches!(k, EventKind::ReplacementsReceived { .. })
        });
        assert!(events.iter().all(|e| e.causes.is_empty()));
        assert!(events
            .iter()
            .all(|e| e.pulse == Some(Pulse::Night) && e.day == 2));
        assert_eq!(
            events[0].kind.to_string(),
            "air_group:10 received 4 aircraft and 1 pilots at base:2, 12 ready and 12 pilots"
        );
        assert_eq!(
            events[0].kind.entities(),
            vec![
                crate::ids::EntityRef::AirGroup(AirGroupId::new(10)),
                crate::ids::EntityRef::Base(BASE_JAPANESE)
            ]
        );
        for (id, ready, pilots) in [(10, 12, 12), (11, 12, 11), (12, 1, 0), (13, 1, 0)] {
            let group = &world.truth.air_groups[&AirGroupId::new(id)];
            assert_eq!(
                (group.ready, group.pilots),
                (ready, Some(pilots)),
                "group {id}"
            );
        }
    }

    #[test]
    fn a_ceiling_counts_damaged_aircraft_and_replacements_arrive_ready() {
        let mut world = base_world(1);
        crewed(&mut world, 10, 5, 6, Some(12), 12);
        group_mut(&mut world, AirGroupId::new(10)).damaged = 4;
        world
            .truth
            .replacements
            .push(stream(BASE_JAPANESE, Side::Japanese, 5, 5, 0));
        world.receive_replacements(2, Pulse::Night);
        let group = &world.truth.air_groups[&AirGroupId::new(10)];
        // Room for 2 (12 less 6 ready and 4 damaged); they join ready.
        assert_eq!((group.ready, group.damaged, group.pilots), (8, 4, Some(12)));
        assert_eq!(received(&world), vec![(10, 2, 0, 8, 12)]);
    }

    #[test]
    fn what_a_day_cannot_place_is_lost_and_nothing_is_held_over() {
        let mut world = base_world(1);
        crewed(&mut world, 10, 5, 10, Some(12), 12);
        world
            .truth
            .replacements
            .push(stream(BASE_JAPANESE, Side::Japanese, 5, 5, 4));
        world.receive_replacements(2, Pulse::Night);
        // 2 aircraft and 0 pilots placed; 3 and 4 lost.
        assert_eq!(received(&world), vec![(10, 2, 0, 12, 12)]);
        // The next day the group has lost one aircraft: it gets one, not one
        // and the three held over.
        group_mut(&mut world, AirGroupId::new(10)).ready = 11;
        world.receive_replacements(3, Pulse::Night);
        assert_eq!(
            received(&world),
            vec![(10, 2, 0, 12, 12), (10, 1, 0, 12, 12)]
        );
        // Nothing to place records nothing.
        world.receive_replacements(4, Pulse::Night);
        assert_eq!(received(&world).len(), 2);
    }

    #[test]
    fn day_1_has_nothing_to_replace_and_an_untracked_group_takes_aircraft_but_no_pilot() {
        let mut world = base_world(1);
        // Group 10's pilots are not tracked and it is short 3 aircraft:
        // the stream's 2 pilots pass it by to group 11.
        crewed(&mut world, 10, 5, 9, None, 12);
        crewed(&mut world, 11, 5, 12, Some(10), 12);
        world
            .truth
            .replacements
            .push(stream(BASE_JAPANESE, Side::Japanese, 5, 2, 2));
        world.receive_replacements(1, Pulse::Night);
        assert!(received(&world).is_empty());
        world.receive_replacements(2, Pulse::Night);
        assert_eq!(
            received(&world),
            vec![(10, 2, 0, 11, 0), (11, 0, 2, 12, 12)]
        );
        assert_eq!(pilots_of(&world, 10), None);
    }

    #[test]
    fn a_stream_at_a_base_the_side_no_longer_holds_places_nothing_and_streams_apply_in_order() {
        let mut world = base_world(1);
        crewed(&mut world, 10, 5, 6, Some(6), 12);
        for (aircraft, pilots) in [(2, 1), (3, 2)] {
            world.truth.replacements.push(stream(
                BASE_JAPANESE,
                Side::Japanese,
                5,
                aircraft,
                pilots,
            ));
        }
        // The first stream's aircraft come first, then the second's.
        world.receive_replacements(2, Pulse::Night);
        assert_eq!(received(&world), vec![(10, 2, 1, 8, 7), (10, 3, 2, 11, 9)]);
        // The base changes hands: nothing is placed for the Japanese.
        world
            .truth
            .bases
            .get_mut(&BASE_JAPANESE)
            .expect("base")
            .side = Side::Allied;
        world.receive_replacements(3, Pulse::Night);
        assert_eq!(received(&world).len(), 2);
        // Nor does a stream of the other side fill the group.
        world
            .truth
            .replacements
            .push(stream(BASE_JAPANESE, Side::Allied, 5, 3, 3));
        world.receive_replacements(3, Pulse::Night);
        assert_eq!(received(&world).len(), 2);
    }

    #[test]
    fn replacements_arrive_at_night_from_day_2_in_a_resolved_day_and_draw_no_random_number() {
        let build = |seed: u64| {
            let mut world = base_world(seed);
            crewed(&mut world, 10, 5, 8, Some(8), 12);
            world
                .truth
                .replacements
                .push(stream(BASE_JAPANESE, Side::Japanese, 5, 2, 1));
            world
        };
        let mut world = build(1);
        world.resolve_day().expect("day 1 resolves");
        assert!(received(&world).is_empty());
        world.resolve_day().expect("day 2 resolves");
        world.resolve_day().expect("day 3 resolves");
        let events = events_where(&world, |k| {
            matches!(k, EventKind::ReplacementsReceived { .. })
        });
        assert_eq!(
            events.iter().map(|e| (e.day, e.pulse)).collect::<Vec<_>>(),
            vec![(2, Some(Pulse::Night)), (3, Some(Pulse::Night))]
        );
        assert_eq!(
            received(&world),
            vec![(10, 2, 1, 10, 9), (10, 2, 1, 12, 10)]
        );
        // The seed does not matter: no random number is drawn.
        let mut other = build(99);
        for _ in 0..3 {
            other.resolve_day().expect("resolves");
        }
        assert_eq!(received(&other), received(&world));
        assert_eq!(other.day_hashes().len(), 3);
    }

    #[test]
    fn untracked_groups_and_scenarios_without_replacements_hash_and_fly_as_before() {
        let mut plain = base_world(1);
        crewed(&mut plain, 10, 5, 8, None, 12);
        let hash = plain.state_hash();
        // Strength is read by replacements alone: with none, it is not
        // hashed.
        let mut other = base_world(1);
        crewed(&mut other, 10, 5, 8, None, 99);
        assert_eq!(other.state_hash(), hash);
        // Pilots, strength, and a stream are.
        group_mut(&mut other, AirGroupId::new(10)).pilots = Some(8);
        assert_ne!(other.state_hash(), hash);
        let with_pilots = other.state_hash();
        group_mut(&mut other, AirGroupId::new(10)).pilots = Some(7);
        assert_ne!(other.state_hash(), with_pilots);
        group_mut(&mut other, AirGroupId::new(10)).pilots = Some(8);
        group_mut(&mut other, AirGroupId::new(10)).strength = 12;
        assert_ne!(other.state_hash(), with_pilots);
        let mut with_stream = base_world(1);
        crewed(&mut with_stream, 10, 5, 8, None, 12);
        with_stream
            .truth
            .replacements
            .push(stream(BASE_JAPANESE, Side::Japanese, 5, 2, 0));
        assert_ne!(with_stream.state_hash(), hash);
        // No pilots, no stream: every group flies its ready aircraft.
        assert_eq!(able(&plain, 10), 8);
    }

    #[test]
    fn a_side_view_shows_the_sides_own_pilots_and_streams_and_not_the_enemys() {
        let mut world = base_world(1);
        crewed(&mut world, 10, 5, 8, Some(7), 12);
        base_group(&mut world, 11, BASE_ALLIED, 5, 6, AirMission::Rest);
        group_mut(&mut world, AirGroupId::new(11)).pilots = Some(5);
        world
            .truth
            .replacements
            .push(stream(BASE_JAPANESE, Side::Japanese, 5, 2, 1));
        world
            .truth
            .replacements
            .push(stream(BASE_ALLIED, Side::Allied, 5, 3, 0));
        let allied = world.side_view(Side::Allied);
        let streams: Vec<(Side, u32)> = allied
            .replacements()
            .map(|s| (s.side, s.aircraft_per_day))
            .collect();
        assert_eq!(streams, vec![(Side::Allied, 3)]);
        let groups: Vec<(u32, Option<u32>, u32)> = allied
            .air_groups()
            .map(|g| (g.id.raw(), g.pilots, g.strength))
            .collect();
        assert_eq!(groups, vec![(11, Some(5), 6)]);
        assert_eq!(world.side_view(Side::Japanese).replacements().count(), 1);
    }

    #[test]
    fn a_replacement_stream_at_an_unknown_base_breaks_an_invariant() {
        let mut world = base_world(1);
        world
            .truth
            .replacements
            .push(stream(BaseId::new(9), Side::Japanese, 5, 2, 1));
        let error = world.resolve_day().expect_err("the day is refused");
        assert!(error.to_string().contains("replacement stream"), "{error}");
    }
    // ---- arrivals (decision 0041) ----

    use crate::state::Arrival;

    fn arrival(day: u32, group: u32, aircraft: u32, pilots: u32) -> Arrival {
        Arrival {
            day,
            air_group: AirGroupId::new(group),
            aircraft,
            pilots,
        }
    }

    /// (day, air group, aircraft, pilots, ready, pilots now, strength) of
    /// each `AircraftArrived`.
    fn arrived(world: &World) -> Vec<(u32, u32, u32, u32, u32, u32, u32)> {
        events_where(world, |k| matches!(k, EventKind::AircraftArrived { .. }))
            .iter()
            .map(|event| match event.kind {
                EventKind::AircraftArrived {
                    air_group,
                    aircraft,
                    pilots,
                    ready,
                    pilots_now,
                    strength,
                } => (
                    event.day,
                    air_group.raw(),
                    aircraft,
                    pilots,
                    ready,
                    pilots_now,
                    strength,
                ),
                _ => unreachable!(),
            })
            .collect()
    }

    #[test]
    fn an_arrival_adds_aircraft_pilots_and_strength_on_its_day_and_on_no_other() {
        let mut world = base_world(1);
        crewed(&mut world, 10, 5, 8, Some(7), 12);
        world.truth.arrivals.push(arrival(3, 10, 6, 5));
        world.receive_arrivals(2, Pulse::Night);
        assert!(arrived(&world).is_empty());
        assert_eq!(pilots_of(&world, 10), Some(7));
        world.receive_arrivals(3, Pulse::Night);
        let group = &world.truth.air_groups[&AirGroupId::new(10)];
        assert_eq!(
            (group.ready, group.pilots, group.strength),
            (14, Some(12), 18)
        );
        assert_eq!(arrived(&world), vec![(3, 10, 6, 5, 14, 12, 18)]);
        let event = &events_where(&world, |k| matches!(k, EventKind::AircraftArrived { .. }))[0];
        assert!(event.causes.is_empty());
        assert_eq!(
            event.kind.to_string(),
            "air_group:10 received 6 aircraft and 5 pilots by arrival, 14 ready, 12 pilots and strength 18"
        );
        world.receive_arrivals(4, Pulse::Night);
        assert_eq!(arrived(&world).len(), 1);
    }

    #[test]
    fn an_arrival_comes_before_the_replacements_that_night_and_raises_their_ceiling() {
        // 10 aircraft, strength 12; an arrival of 6 makes the strength 18
        // and 16 aircraft; the replacement stream of 2 fills the gap of 2:
        // 18 aircraft and strength 18.
        let mut world = base_world(1);
        crewed(&mut world, 10, 5, 10, Some(10), 12);
        world.truth.arrivals.push(arrival(2, 10, 6, 6));
        world
            .truth
            .replacements
            .push(stream(BASE_JAPANESE, Side::Japanese, 5, 2, 2));
        world.resolve_day().expect("day 1 resolves");
        assert_eq!(arrived(&world).len(), 0);
        let after_day_1 = world.truth.air_groups[&AirGroupId::new(10)].ready;
        assert_eq!(after_day_1, 10);
        world.resolve_day().expect("day 2 resolves");
        let group = &world.truth.air_groups[&AirGroupId::new(10)];
        assert_eq!(
            (group.ready, group.strength, group.pilots),
            (18, 18, Some(18))
        );
        assert_eq!(arrived(&world), vec![(2, 10, 6, 6, 16, 16, 18)]);
        assert_eq!(received(&world), vec![(10, 2, 2, 18, 18)]);
        let order: Vec<bool> = world
            .events()
            .iter()
            .filter_map(|e| match e.kind {
                EventKind::AircraftArrived { .. } => Some(true),
                EventKind::ReplacementsReceived { .. } => Some(false),
                _ => None,
            })
            .collect();
        assert_eq!(order, vec![true, false]);
    }

    #[test]
    fn a_day_1_arrival_flies_that_morning() {
        let mut world = raid_world(1);
        group_mut(&mut world, AirGroupId::new(1)).ready = 0;
        group_mut(&mut world, AirGroupId::new(1)).strength = 0;
        world.truth.arrivals.push(arrival(1, 1, 6, 0));
        world.resolve_day().expect("day 1 resolves");
        assert_eq!(arrived(&world), vec![(1, 1, 6, 0, 6, 0, 6)]);
        let launches = base_launches(&world);
        assert_eq!(launches.len(), 1);
        assert_eq!(launches[0].day, 1);
        assert!(matches!(
            launches[0].kind,
            EventKind::BaseStrikeLaunched { aircraft: 6, .. }
        ));
    }

    #[test]
    fn a_group_that_starts_with_nothing_does_nothing_until_its_arrival() {
        let mut world = raid_world(1);
        tire(&mut world);
        upkeep(&mut world, fixed(1, 10), fixed(1, 10), fixed(1, 2), 0);
        supply_figures(&mut world);
        set_supply(&mut world, BASE_ALLIED, Fixed::from_int(100));
        {
            let group = group_mut(&mut world, AirGroupId::new(1));
            group.ready = 0;
            group.strength = 0;
        }
        // It can be given orders before its aircraft come.
        world.submit_orders(
            Side::Allied,
            vec![mission_order(
                AirGroupId::new(1),
                AirMission::BaseStrike(BASE_JAPANESE),
            )],
        );
        // A replacement stream gives it nothing: its strength is nothing.
        world
            .truth
            .replacements
            .push(stream(BASE_ALLIED, Side::Allied, 2, 3, 0));
        world.truth.arrivals.push(arrival(3, 1, 4, 0));
        let hash_before = world.state_hash();
        world.resolve_day().expect("day 1 resolves");
        world.resolve_day().expect("day 2 resolves");
        assert!(rejections(&world).is_empty());
        let touched = |world: &World| {
            world
                .events()
                .iter()
                .filter(|e| {
                    e.kind
                        .entities()
                        .contains(&EntityRef::AirGroup(AirGroupId::new(1)))
                })
                .map(|e| e.kind.to_string())
                .collect::<Vec<_>>()
        };
        let events = touched(&world);
        assert!(
            events.iter().all(|text| text.contains("order")),
            "{events:?}"
        );
        assert!(received(&world).is_empty());
        assert!(base_launches(&world).is_empty());
        assert!(ground_losses(&world).is_empty());
        let group = &world.truth.air_groups[&AirGroupId::new(1)];
        assert_eq!(
            (group.ready, group.damaged, group.strength, group.fatigue),
            (0, 0, 0, Fixed::ZERO)
        );
        assert_eq!(supply_of(&world, BASE_ALLIED), Fixed::from_int(100));
        assert!(spent(&world).is_empty());
        assert_ne!(world.state_hash(), hash_before);
        world.resolve_day().expect("day 3 resolves");
        assert_eq!(arrived(&world), vec![(3, 1, 4, 0, 4, 0, 4)]);
        let launches = base_launches(&world);
        assert_eq!(launches.len(), 1);
        assert_eq!(launches[0].day, 3);
        assert!(matches!(
            launches[0].kind,
            EventKind::BaseStrikeLaunched { aircraft: 4, .. }
        ));
    }

    #[test]
    fn arrivals_to_a_group_without_tracked_pilots_add_aircraft_and_strength_only() {
        let mut world = base_world(1);
        crewed(&mut world, 10, 5, 8, None, 12);
        world.truth.arrivals.push(arrival(1, 10, 3, 0));
        world.receive_arrivals(1, Pulse::Night);
        let group = &world.truth.air_groups[&AirGroupId::new(10)];
        assert_eq!((group.ready, group.pilots, group.strength), (11, None, 15));
        assert_eq!(arrived(&world), vec![(1, 10, 3, 0, 11, 0, 15)]);
    }

    #[test]
    fn a_group_lost_with_its_ship_receives_nothing_and_records_nothing() {
        let mut world = world(1, vec![]);
        let carried = world
            .truth
            .air_groups
            .values()
            .find_map(|group| match group.location {
                AirGroupLocation::Ship(ship) => Some((group.id, ship)),
                _ => None,
            })
            .expect("the standard world has a carried group");
        world.truth.arrivals.push(arrival(1, carried.0.raw(), 4, 0));
        // Afloat: delivered.
        world.receive_arrivals(1, Pulse::Night);
        assert_eq!(arrived(&world).len(), 1);
        // Sunk: not.
        world
            .truth
            .ships
            .get_mut(&carried.1)
            .expect("the ship")
            .location = ShipLocation::Sunk;
        let before = world.truth.air_groups[&carried.0].clone();
        world.receive_arrivals(1, Pulse::Night);
        assert_eq!(arrived(&world).len(), 1);
        assert_eq!(world.truth.air_groups[&carried.0], before);
    }

    #[test]
    fn no_arrivals_leaves_the_hash_as_it_was_and_arrivals_change_it() {
        let mut plain = base_world(1);
        crewed(&mut plain, 10, 5, 8, None, 12);
        let hash = plain.state_hash();
        let mut other = base_world(1);
        crewed(&mut other, 10, 5, 8, None, 12);
        other.truth.arrivals.push(arrival(2, 10, 1, 0));
        assert_ne!(other.state_hash(), hash);
        let with_one = other.state_hash();
        other.truth.arrivals[0].aircraft = 2;
        assert_ne!(other.state_hash(), with_one);
        other.truth.arrivals.clear();
        assert_eq!(other.state_hash(), hash);
        // Strength is hashed only where something reads it: still not with
        // no arrivals and no replacements.
        group_mut(&mut other, AirGroupId::new(10)).strength = 99;
        assert_eq!(other.state_hash(), hash);
        // A side sees its own arrivals and not the enemy's.
        crewed(&mut other, 11, 5, 2, None, 2);
        other.truth.arrivals.push(arrival(2, 10, 1, 0));
        assert_eq!(other.side_view(Side::Japanese).arrivals().count(), 1);
        assert_eq!(other.side_view(Side::Allied).arrivals().count(), 0);
    }

    #[test]
    fn two_arrivals_for_a_group_on_one_day_are_both_delivered_in_order() {
        let mut world = base_world(1);
        crewed(&mut world, 10, 5, 4, Some(4), 4);
        world.truth.arrivals.push(arrival(2, 10, 3, 2));
        world.truth.arrivals.push(arrival(2, 10, 5, 0));
        world.receive_arrivals(2, Pulse::Night);
        assert_eq!(
            arrived(&world),
            vec![(2, 10, 3, 2, 7, 6, 7), (2, 10, 5, 0, 12, 6, 12)]
        );
    }

    // ---- base supply (decision 0039) ----

    use crate::events::SupplyUse;
    use crate::refdata::AirSupplyFigures;

    /// The figures of `data/reference/air_supply.toml`: 0.5 t a sortie, 0.25
    /// more for each 100 nm, 0.2 t a day for each aircraft, 1 t a repair.
    fn supply_figures(world: &mut World) {
        world.ref_data.air_supply = Some(AirSupplyFigures {
            sortie_supply_t: fixed(1, 2),
            sortie_supply_t_per_100_nm: fixed(1, 4),
            daily_supply_t_per_aircraft: fixed(1, 5),
            repair_supply_t: Fixed::ONE,
        });
    }

    fn set_supply(world: &mut World, base: BaseId, tons: Fixed) {
        world
            .truth
            .bases
            .get_mut(&base)
            .expect("the base exists")
            .supply_t = tons;
    }

    fn supply_of(world: &World, base: BaseId) -> Fixed {
        world.truth.bases[&base].supply_t
    }

    /// The `BaseSupplySpent` events as (base, reason, aircraft, asked, paid,
    /// left).
    fn spent(world: &World) -> Vec<(u32, SupplyUse, u32, Fixed, Fixed, Fixed)> {
        events_where(world, |k| matches!(k, EventKind::BaseSupplySpent { .. }))
            .iter()
            .map(|event| match event.kind {
                EventKind::BaseSupplySpent {
                    base,
                    reason,
                    aircraft,
                    asked_t,
                    paid_t,
                    left_t,
                } => (base.raw(), reason, aircraft, asked_t, paid_t, left_t),
                _ => unreachable!(),
            })
            .collect()
    }

    fn tons(value: i64) -> Fixed {
        Fixed::from_int(value)
    }

    /// Allied base 1 (100 t) sends 18 bombers on a base strike and keeps a
    /// fighter group of 10 on patrol, half of it escorting; Japanese base 2
    /// (1,000 t) has 6 scouts searching to 300 nm and 4 fighters on patrol.
    /// No fighter or gun hits anything and every bomb hits aircraft.
    fn supply_world() -> World {
        let mut world = raid_world(1);
        supply_figures(&mut world);
        set_supply(&mut world, BASE_ALLIED, tons(100));
        base_group(
            &mut world,
            2,
            BASE_ALLIED,
            5,
            10,
            AirMission::CombatAirPatrol,
        );
        group_mut(&mut world, AirGroupId::new(2)).patrol_share = fixed(1, 2);
        base_group(&mut world, 3, BASE_JAPANESE, 1, 6, AirMission::Search);
        group_mut(&mut world, AirGroupId::new(3)).search = Some(SearchArc {
            from_deg: 0,
            to_deg: 350,
            range_nm: Fixed::from_int(300),
        });
        base_group(
            &mut world,
            4,
            BASE_JAPANESE,
            5,
            4,
            AirMission::CombatAirPatrol,
        );
        sure_bombs(&mut world, Fixed::ONE, Fixed::ZERO);
        air_figures(&mut world).fighter_hit_chance = Fixed::ZERO;
        world.work_out_grounded_bases();
        world
    }

    fn fly_pulse(world: &mut World, pulse: Pulse) {
        world.work_out_grounded_bases();
        world.search_from_the_air(1, pulse);
        strike_pulse(world, pulse);
        world.wear_aircraft(1, pulse);
    }

    #[test]
    fn a_sortie_costs_the_figure_plus_the_figure_per_100_nm_for_its_distance() {
        let mut world = world(1, vec![]);
        supply_figures(&mut world);
        let figures = world.ref_data.air_supply.expect("figures");
        for (nm, cost) in [
            (0, fixed(1, 2)),
            (160, fixed(9, 10)),
            (400, fixed(3, 2)),
            (600, tons(2)),
        ] {
            assert_eq!(figures.sortie_cost(tons(nm)), cost, "{nm} nm");
        }
        assert!(figures.problem().is_none());
        let negative = AirSupplyFigures {
            repair_supply_t: fixed(-1, 1),
            ..figures
        };
        assert!(negative.problem().expect("a problem").contains("repair"));
    }

    #[test]
    fn each_base_pays_one_sum_for_the_sorties_of_the_pulse_on_the_aircraft_sent() {
        let mut world = supply_world();
        fly_pulse(&mut world, Pulse::Morning);
        // Base 1: 18 bombers and 5 escorts at 0.5 + 0.25 x 4 = 1.5 t, and 5
        // fighters on patrol at 0.5 t: 27 + 7.5 + 2.5 = 37 t. Base 2: 6
        // scouts at 0.5 + 0.25 x 6 = 2 t and 4 fighters on patrol: 12 + 2.
        assert_eq!(
            spent(&world),
            vec![
                (1, SupplyUse::Sorties, 28, tons(37), tons(37), tons(63)),
                (2, SupplyUse::Sorties, 10, tons(14), tons(14), tons(986)),
            ]
        );
        assert_eq!(supply_of(&world, BASE_ALLIED), tons(63));
        assert_eq!(supply_of(&world, BASE_JAPANESE), tons(986));
        let events = events_where(&world, |k| matches!(k, EventKind::BaseSupplySpent { .. }));
        let launched = base_launches(&world)[0].id;
        assert!(events[0].causes.contains(&Cause::Event(launched)));
        let first = events[0].kind.to_string();
        assert_eq!(
            first,
            "base:1 spent 37 t of supply on 28 aircraft sent on sorties, 63 t left"
        );
        // The afternoon has no raid: the patrol of 10 at 0.5 t each (the
        // bombers fly again, so the raid does too).
        let before = spent(&world).len();
        fly_pulse(&mut world, Pulse::Afternoon);
        assert_eq!(spent(&world).len(), before + 2);
    }

    #[test]
    fn the_aircraft_shot_down_are_paid_for_as_sent_and_a_night_pulse_pays_for_no_sortie() {
        let mut world = supply_world();
        air_figures(&mut world).fighter_hit_chance = Fixed::ONE;
        fly_pulse(&mut world, Pulse::Morning);
        assert!(!events_where(&world, |k| matches!(k, EventKind::AircraftLost { .. })).is_empty());
        let sums = spent(&world);
        assert_eq!(sums[0].3, tons(37));
        assert_eq!(sums[0].2, 28);
        assert_eq!(sums[1].3, tons(14));
        // Nothing flies at night and no sortie is paid for.
        let before = spent(&world).len();
        world.search_from_the_air(1, Pulse::Night);
        strike_pulse(&mut world, Pulse::Night);
        world.wear_aircraft(1, Pulse::Night);
        assert_eq!(spent(&world).len(), before);
    }

    #[test]
    fn a_sum_the_stock_cannot_cover_takes_what_is_left_and_says_so() {
        let mut world = supply_world();
        set_supply(&mut world, BASE_ALLIED, tons(10));
        fly_pulse(&mut world, Pulse::Morning);
        assert_eq!(
            spent(&world)[0],
            (1, SupplyUse::Sorties, 28, tons(37), tons(10), Fixed::ZERO)
        );
        assert_eq!(supply_of(&world, BASE_ALLIED), Fixed::ZERO);
        let events = events_where(&world, |k| matches!(k, EventKind::BaseSupplySpent { .. }));
        assert_eq!(
            events[0].kind.to_string(),
            "base:1 spent 10 t of supply on 28 aircraft sent on sorties: 37 t was asked and the stock could not cover it, 0 t left: base:1 is out of supply"
        );
        // Spending exactly the stock leaves 0 and says the base is out.
        let mut world = supply_world();
        set_supply(&mut world, BASE_ALLIED, tons(37));
        fly_pulse(&mut world, Pulse::Morning);
        let events = events_where(&world, |k| matches!(k, EventKind::BaseSupplySpent { .. }));
        assert_eq!(
            events[0].kind.to_string(),
            "base:1 spent 37 t of supply on 28 aircraft sent on sorties, 0 t left: base:1 is out of supply"
        );
    }

    #[test]
    fn the_daily_cost_counts_ready_and_damaged_aircraft_and_is_capped_at_the_stock() {
        let mut world = base_world(1);
        supply_figures(&mut world);
        base_group(&mut world, 3, BASE_JAPANESE, 5, 4, AirMission::Rest);
        group_mut(&mut world, AirGroupId::new(3)).damaged = 2;
        base_group(&mut world, 4, BASE_JAPANESE, 3, 5, AirMission::Rest);
        group_mut(&mut world, AirGroupId::new(4)).damaged = 3;
        // A group aboard a ship costs no base anything (decision 9).
        world.truth.air_groups.insert(
            AirGroupId::new(20),
            group(AirGroupId::new(20), Side::Allied, 1, 2, 9),
        );
        // Base 1 has aircraft but no stock: it pays nothing and records
        // nothing.
        base_group(&mut world, 5, BASE_ALLIED, 5, 3, AirMission::Rest);
        world.pay_daily_supply(1, Pulse::Night);
        assert_eq!(
            spent(&world),
            vec![(
                2,
                SupplyUse::Upkeep,
                14,
                fixed(14, 5),
                fixed(14, 5),
                fixed(4986, 5)
            )]
        );
        assert_eq!(
            events_where(&world, |k| matches!(k, EventKind::BaseSupplySpent { .. }))[0]
                .kind
                .to_string(),
            "base:2 spent 2.8 t of supply on 14 aircraft at the base for the day, 997.2 t left"
        );
        // A stock of 1 t pays 1 t of the 2.8 t asked.
        set_supply(&mut world, BASE_JAPANESE, tons(1));
        world.pay_daily_supply(2, Pulse::Night);
        assert_eq!(
            spent(&world)[1],
            (2, SupplyUse::Upkeep, 14, fixed(14, 5), tons(1), Fixed::ZERO)
        );
        // Out of supply: nothing to pay, nothing recorded.
        world.pay_daily_supply(3, Pulse::Night);
        assert_eq!(spent(&world).len(), 2);
        // Without the figures nothing is spent.
        let mut world = base_world(1);
        base_group(&mut world, 3, BASE_JAPANESE, 5, 4, AirMission::Rest);
        world.pay_daily_supply(1, Pulse::Night);
        assert!(spent(&world).is_empty());
        assert_eq!(supply_of(&world, BASE_JAPANESE), tons(1000));
    }

    #[test]
    fn a_base_repairs_only_while_its_stock_covers_a_repair_and_pays_for_each() {
        let mut world = base_world(1);
        supply_figures(&mut world);
        upkeep(&mut world, NEVER, NEVER, NEVER, 2);
        world
            .truth
            .bases
            .get_mut(&BASE_JAPANESE)
            .expect("base")
            .aircraft_repairs_per_day = 5;
        base_group(&mut world, 3, BASE_JAPANESE, 5, 4, AirMission::Rest);
        group_mut(&mut world, AirGroupId::new(3)).damaged = 4;
        set_supply(&mut world, BASE_JAPANESE, fixed(5, 2));
        world.repair_aircraft(2, Pulse::Night);
        // 2.5 t covers 2 repairs, and 0.5 t is left.
        assert_eq!(repaired(&world), vec![(3, 2, 6, 2)]);
        assert_eq!(
            spent(&world),
            vec![(2, SupplyUse::Repairs, 2, tons(2), tons(2), fixed(1, 2))]
        );
        let events = events_where(&world, |k| matches!(k, EventKind::BaseSupplySpent { .. }));
        let repair =
            events_where(&world, |k| matches!(k, EventKind::AircraftRepaired { .. }))[0].id;
        assert_eq!(events[0].causes, vec![Cause::Event(repair)]);
        // With less than a repair's worth nothing is repaired and nothing is
        // recorded.
        world.repair_aircraft(3, Pulse::Night);
        assert_eq!(repaired(&world).len(), 1);
        assert_eq!(spent(&world).len(), 1);
        assert_eq!(supply_of(&world, BASE_JAPANESE), fixed(1, 2));
        // A base with no stock repairs nothing either.
        set_supply(&mut world, BASE_JAPANESE, Fixed::ZERO);
        world.repair_aircraft(4, Pulse::Night);
        assert_eq!(repaired(&world).len(), 1);
        // The daily cost comes after the repairs in the night.
        set_supply(&mut world, BASE_JAPANESE, tons(100));
        world.repair_aircraft(5, Pulse::Night);
        world.pay_daily_supply(5, Pulse::Night);
        let reasons: Vec<SupplyUse> = spent(&world).iter().map(|row| row.1).collect();
        assert_eq!(
            reasons,
            vec![SupplyUse::Repairs, SupplyUse::Repairs, SupplyUse::Upkeep]
        );
        // 2 aircraft were left damaged: 2 more repairs, 2 t, and then the 8
        // aircraft at 0.2 t.
        assert_eq!(spent(&world)[1].3, tons(2));
        assert_eq!(spent(&world)[2].3, fixed(8, 5));
        // Without the supply figures repair is as before.
        let mut world = base_world(1);
        upkeep(&mut world, NEVER, NEVER, NEVER, 2);
        world
            .truth
            .bases
            .get_mut(&BASE_JAPANESE)
            .expect("base")
            .aircraft_repairs_per_day = 5;
        base_group(&mut world, 3, BASE_JAPANESE, 5, 4, AirMission::Rest);
        group_mut(&mut world, AirGroupId::new(3)).damaged = 4;
        set_supply(&mut world, BASE_JAPANESE, Fixed::ZERO);
        world.repair_aircraft(2, Pulse::Night);
        assert_eq!(repaired(&world), vec![(3, 4, 8, 0)]);
        assert!(spent(&world).is_empty());
    }

    #[test]
    fn a_carriers_repairs_cost_no_base_anything() {
        let mut world = world(1, vec![]);
        supply_figures(&mut world);
        world.truth.air_groups.clear();
        world
            .truth
            .air_groups
            .insert(BOMBERS, group(BOMBERS, Side::Allied, 1, 2, 10));
        group_mut(&mut world, BOMBERS).damaged = 3;
        upkeep(&mut world, NEVER, NEVER, NEVER, 5);
        world.repair_aircraft(1, Pulse::Night);
        assert_eq!(repaired(&world), vec![(1, 3, 13, 0)]);
        assert!(spent(&world).is_empty());
    }

    /// The world of the grounded tests: Allied base 1 has no stock, 10
    /// fighters on patrol, 6 scouts searching and 8 bombers on a base
    /// strike, and 20 guns. Japanese base 2 sends 12 bombers at it.
    fn grounded_world(stock: i64) -> World {
        let mut world = base_world(1);
        supply_figures(&mut world);
        set_supply(&mut world, BASE_ALLIED, tons(stock));
        world
            .truth
            .bases
            .get_mut(&BASE_ALLIED)
            .expect("base")
            .anti_aircraft = 20;
        base_group(
            &mut world,
            1,
            BASE_ALLIED,
            2,
            8,
            AirMission::BaseStrike(BASE_JAPANESE),
        );
        base_group(
            &mut world,
            2,
            BASE_ALLIED,
            5,
            10,
            AirMission::CombatAirPatrol,
        );
        group_mut(&mut world, AirGroupId::new(2)).patrol_share = fixed(1, 2);
        base_group(&mut world, 6, BASE_ALLIED, 1, 6, AirMission::Search);
        group_mut(&mut world, AirGroupId::new(6)).search = Some(SearchArc {
            from_deg: 0,
            to_deg: 350,
            range_nm: Fixed::from_int(300),
        });
        base_group(
            &mut world,
            7,
            BASE_JAPANESE,
            2,
            12,
            AirMission::BaseStrike(BASE_ALLIED),
        );
        sure_bombs(&mut world, Fixed::ONE, Fixed::ZERO);
        world
    }

    #[test]
    fn a_base_with_no_supply_at_the_start_of_the_pulse_flies_nothing_but_its_guns_fire() {
        let mut world = grounded_world(0);
        fly_pulse(&mut world, Pulse::Morning);
        // No search, no strike, no escorts, no patrol, no sorties paid for.
        assert!(events_where(&world, |k| matches!(k, EventKind::SearchFlown { .. })).is_empty());
        let launched = base_launches(&world);
        assert_eq!(launched.len(), 1, "only the Japanese raid");
        assert_eq!(
            launched[0].kind.entities()[0],
            EntityRef::AirGroup(AirGroupId::new(7))
        );
        assert!(events_where(&world, |k| matches!(k, EventKind::EscortsSent { .. })).is_empty());
        assert!(events_where(&world, |k| matches!(k, EventKind::AirFight { .. })).is_empty());
        assert!(spent(&world).iter().all(|row| row.0 != 1));
        // The guns still fire.
        let guns = events_where(&world, |k| {
            matches!(k, EventKind::BaseAntiAircraftFire { .. })
        });
        assert_eq!(guns.len(), 1);
        // Every ready aircraft is on the ground, and the hits destroy them.
        let lost: u32 = ground_losses(&world).iter().map(|row| row.1).sum();
        assert!(lost > 0);
        assert!(ground_losses(&world)
            .iter()
            .all(|row| [1, 2, 6].contains(&row.0)));
        // With a stock the same base flies.
        let mut world = grounded_world(100);
        fly_pulse(&mut world, Pulse::Morning);
        assert_eq!(
            events_where(&world, |k| matches!(k, EventKind::SearchFlown { .. })).len(),
            1
        );
        assert_eq!(base_launches(&world).len(), 2);
        assert_eq!(
            events_where(&world, |k| matches!(k, EventKind::EscortsSent { .. })).len(),
            1
        );
        assert!(spent(&world).iter().any(|row| row.0 == 1));
    }

    #[test]
    fn a_base_that_runs_out_during_a_pulse_flew_it_and_is_grounded_from_the_next() {
        // The raid's bombs all land on supply and destroy 50 t each.
        let mut world = grounded_world(30);
        sure_bombs(&mut world, Fixed::ZERO, Fixed::ONE);
        fly_pulse(&mut world, Pulse::Morning);
        assert_eq!(supply_of(&world, BASE_ALLIED), Fixed::ZERO);
        // It searched, launched and patrolled in the morning, and the sum
        // for its sorties asked what they cost and could pay nothing.
        assert_eq!(
            events_where(&world, |k| matches!(k, EventKind::SearchFlown { .. })).len(),
            1
        );
        let sums: Vec<_> = spent(&world).into_iter().filter(|row| row.0 == 1).collect();
        assert_eq!(sums.len(), 1);
        assert_eq!(sums[0].1, SupplyUse::Sorties);
        assert!(sums[0].3 > Fixed::ZERO);
        assert_eq!(sums[0].4, Fixed::ZERO);
        // In the afternoon it flies nothing.
        let searches = events_where(&world, |k| matches!(k, EventKind::SearchFlown { .. })).len();
        let launches_before = base_launches(&world).len();
        fly_pulse(&mut world, Pulse::Afternoon);
        assert!(world.grounded_bases.contains(&BASE_ALLIED));
        assert_eq!(
            events_where(&world, |k| matches!(k, EventKind::SearchFlown { .. })).len(),
            searches
        );
        // No raid is launched in the afternoon, and the base pays for no
        // patrol: the sum of the morning is the only one.
        assert_eq!(base_launches(&world).len(), launches_before);
        assert_eq!(spent(&world).iter().filter(|row| row.0 == 1).count(), 1);
        // It flies again from the first pulse that starts with stock.
        set_supply(&mut world, BASE_ALLIED, tons(5));
        world.work_out_grounded_bases();
        assert!(world.grounded_bases.is_empty());
    }

    #[test]
    fn groups_aboard_ships_are_never_grounded_and_pay_no_base() {
        let mut world = world(1, vec![]);
        supply_figures(&mut world);
        world.truth.bases.insert(
            BASE_ALLIED,
            crate::state::Base {
                id: BASE_ALLIED,
                name: "Test base".to_string(),
                side: Side::Allied,
                hex: Hex::new(0, 0),
                supply_t: Fixed::ZERO,
                anti_aircraft: 0,
                aircraft_repairs_per_day: 0,
                fuel_t: Fixed::ZERO,
            },
        );
        world.work_out_grounded_bases();
        assert!(world.grounded_bases.contains(&BASE_ALLIED));
        assert_eq!(able(&world, BOMBERS.raw()), 18);
        fly_pulse(&mut world, Pulse::Morning);
        assert!(spent(&world).is_empty());
    }

    #[test]
    fn a_scenario_without_supply_figures_spends_nothing_and_grounds_nothing() {
        let mut world = grounded_world(0);
        world.ref_data.air_supply = None;
        fly_pulse(&mut world, Pulse::Morning);
        assert!(world.grounded_bases.is_empty());
        assert!(spent(&world).is_empty());
        assert_eq!(supply_of(&world, BASE_ALLIED), Fixed::ZERO);
        assert_eq!(
            events_where(&world, |k| matches!(k, EventKind::SearchFlown { .. })).len(),
            1
        );
        assert!(world.sorties.is_empty());
    }
}
