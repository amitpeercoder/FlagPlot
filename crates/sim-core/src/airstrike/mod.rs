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
//!    Then, for each raid in order of its first strike group, first the
//!    patrol's fire at the bombers after the attack (decision 0087): the same
//!    `AirFight` with `after_attack` true, citing the launch and the raid's
//!    attack events, then the `AircraftLost` it caused. The patrol's shots at
//!    the bombers are split by `air_combat.toml: patrol_before_attack_share`:
//!    the first k = share times n (cut to a whole number) are worked in
//!    item 1, the other n - k here, from the one stream, in order, and their
//!    hits come off the bombers the attack left. Then the guns' fire
//!    after the attack (decision 0079): `AntiAircraftFire` (or
//!    `BaseAntiAircraftFire`) with `after_attack` true, citing the launch and
//!    the raid's `AirAttack` or `BaseAttacked` events, then the `AircraftLost`
//!    it caused. The guns' shots are split by
//!    `air_combat.toml: anti_aircraft_before_attack_share`: the first
//!    k = share times n (cut to a whole number) are worked in item 1, the
//!    other n - k here, from the one stream, in order. Both parts come off the
//!    one pool of bombers, the lowest group first; the hits after the attack
//!    come off the bombers the fire before left and do not reduce the attack's
//!    shots. A part with no shots records nothing.
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

use crate::belief::{ApplyOutcome, BattleClaim, BeliefState, Contact, Report};
use crate::choices::air::{AirAnswer, AirQuestion, BaseStrikeLaunch, EscortChoice, FighterSplit};
use crate::choices::{Answer, Question};
use crate::damage::{
    damage_per_hit, raw_u64, roll_shots, system_for_draw, system_hit_chance, DamageSite, HitSet,
};
use crate::events::{
    AirFirer, AirTarget, Breakdown, Cause, DamageTrack, EventKind, Factor, SortieKind, Weapon,
};
use crate::fixed::Fixed;
use crate::hex::Hex;
use crate::ids::{
    AirGroupId, AircraftTypeId, BaseId, ContactId, EntityRef, EventId, ShipId, TaskForceId,
};
use crate::orders::{AirMission, Reach};
use crate::pictures::{CommandRef, Holders};
use crate::refdata::{AirCombatFigures, AttackWeapon, ShipKind, SurfaceCombatFigures};
use crate::rng::{Draw, RngStream, Subsystem};
use crate::search::{draw_report, ReportDraws};
use crate::state::{AirGroup, AirGroupLocation, ShipSystem, Side, SystemState};
use crate::time::{self, Pulse};
use crate::upkeep::Sortie;
use crate::view::strike_review::{ContactVerdict, StrikeStop};
use crate::world::World;

mod air_combat;
mod claims;
mod decision;
mod flight_deck;
mod ground;
mod raids;
#[cfg(test)]
mod tests;
mod wear;
mod work_out;

#[cfg(test)]
pub(crate) use decision::sized_strike;
pub(crate) use decision::{
    base_strike_launch, contact_verdict, contacts_to_strike, naval_strike_launch, sent_at_contacts,
    strike_age_limit, strike_aircraft, strike_group_gate, strike_walk, StrikeGate,
};
pub use decision::{NavalStrike, StrikeChoice, StrikeSizing};
pub(crate) use flight_deck::able_to_fly;
pub(crate) use raids::{escort_choice, fighter_split, raids_of_side};

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
    /// (decision 6 of 0056). None for a strike at a base.
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
    /// The draws of the sighting report of the force attacked (decision 12).
    /// They are taken with the attack's other draws and read against the
    /// force after the damage has landed (`docs/rules/differences.md`,
    /// row 7).
    report: ReportDraws,
    /// The hex the strike was sent to, where the report puts the force.
    hex: Hex,
    /// The force's ships afloat at the start of the step, which the report
    /// reads if the damage leaves none afloat, as a fight's report does.
    began_with: Vec<ShipId>,
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
    hits_on_men: u32,
    /// Not in the event (decision 0103): the damage it does is its own
    /// event.
    hits_on_airfield: u32,
    breakdown: Option<Breakdown>,
}

/// What a raid attacks: the enemy task force at its hex that holds the best
/// ranked ship by the target list of its first strike group (decision 4 of
/// 0063), or a base (decision 0034). Each strike group then picks its own
/// ship in the force.
#[derive(Clone, Copy)]
enum RaidTarget {
    Ships(TaskForceId),
    Base(BaseId),
}

/// The aircraft of the groups on the ground at a base, by group in order of
/// group number: the pool hits on aircraft are taken from (decision 8 of
/// 0034).
type Ground = BTreeMap<BaseId, Vec<(AirGroupId, Stock, u32)>>;

/// Where the hits on a base landed (decision 7 of 0034, decision 8 of
/// 0092). The rest landed on nothing.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub(crate) struct BaseHits {
    pub(crate) aircraft: u32,
    pub(crate) supply: u32,
    pub(crate) men: u32,
    pub(crate) airfield: u32,
}

/// Which of a group's aircraft a ground pool entry counts (decision 11 of
/// 0035).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) enum Stock {
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
    /// How far its escorts go: the fighter type's strike radius on normal
    /// reach, more on extended (decisions 0030 and 2 of 0059).
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

/// How a raid's share of the patrol is split between its escorts and its
/// bombers (decision 0080). `share` is `patrol_drawn_off_share`, `patrol`
/// the raid's share of the patrol (P), `escorts` its escorts (E) and
/// `bombers` its bombers at the start of the step (B, at least 1). Gives
/// the shots at the escorts, D + X, and at the bombers, R - X, with
/// D = share times the smaller of P and E, cut to a whole number toward
/// zero, R = P - D, and X = R (E - D) / (E - D + B) in whole numbers, never
/// more than E - D. At a share of 1 this is step 1's rule: the smaller of P
/// and E at the escorts, the rest at the bombers.
pub(crate) fn patrol_split(share: Fixed, patrol: u32, escorts: u32, bombers: u32) -> PatrolSplit {
    let drawn_off =
        u32::try_from((share * Fixed::from_int(i64::from(patrol.min(escorts)))).trunc())
            .expect("a share from 0 to 1 of the patrol is not negative");
    let rest = patrol - drawn_off;
    let escorts_left = escorts - drawn_off;
    let denominator = u64::from(escorts_left) + u64::from(bombers);
    assert!(denominator > 0, "a raid has at least one bomber");
    let rest_at_escorts = u32::try_from(u64::from(rest) * u64::from(escorts_left) / denominator)
        .expect("the rest's shots at the escorts are no more than the rest")
        .min(escorts_left);
    PatrolSplit {
        drawn_off,
        rest_at_escorts,
        at_escorts: drawn_off + rest_at_escorts,
        at_bombers: rest - rest_at_escorts,
    }
}

/// The result of [`patrol_split`].
#[derive(Debug, PartialEq, Eq)]
pub(crate) struct PatrolSplit {
    /// D: the patrol drawn off by the escorts, one for one.
    drawn_off: u32,
    /// X: the rest's shots at the escorts not drawn off.
    rest_at_escorts: u32,
    /// D + X.
    at_escorts: u32,
    /// R - X.
    at_bombers: u32,
}

/// Who fired a roll.
#[derive(Clone)]
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
    /// For fighters, each group that fires in the roll and the aircraft it
    /// has in it, in order of group number: the groups the roll's shots are
    /// parted among for the claim (decision 0180, 3). Empty for guns.
    in_firing: Vec<(AirGroupId, u32)>,
    /// The shots that hit, counted from 1 in the order drawn (decision
    /// 0180, 3).
    hit_shots: Vec<u32>,
    /// The pool the roll fires at, by the type of each group in it, in
    /// order of the type's number, as first counted when the step began
    /// (decision 0210, 2): the patrol over the hex for the escorts' fire,
    /// the raid's escorts for the patrol's fire at them, and the raid's
    /// strike aircraft for the patrol's fire at the bombers and the guns'.
    pool: Vec<(AircraftTypeId, u32)>,
}

/// Everything one raid does in the air combat step, worked out on the state
/// at its start.
struct RaidResult {
    /// Each fighter group that sends escorts: group, aircraft, its orders.
    escorts_sent: Vec<(AirGroupId, u32, Vec<Cause>)>,
    rolls: Vec<FireRoll>,
    /// The patrol's fire at the bombers after the attack (decision 0087), if
    /// the share leaves any shots for it: recorded after the raid's attack
    /// events, before the guns' fire after the attack.
    patrol_after_attack: Option<FireRoll>,
    /// The guns' fire after the attack (decision 0079), if the share leaves
    /// any shots for it: recorded after the raid's attack events.
    after_attack: Option<FireRoll>,
    /// By strike group; a group with no bombers left has none.
    outcomes: Vec<(AirGroupId, Outcome)>,
    /// What the raid found, if anything.
    target: Option<RaidTarget>,
}

/// The whole shots of a roll: the part of a shot is not one that can miss.
fn whole_shots(shots: Fixed) -> u32 {
    u32::try_from(shots.trunc()).expect("shots are not negative")
}

/// Which false claim chance a missed shot is claimed at (decision 12 of
/// 0056).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum MissKind {
    Fighter,
    Gun,
}

/// One unit's part of a firing: the unit, what its shots really shot down
/// and its whole shots that missed (decision 0180, 3 and 4).
type FiringLine = (EntityRef, u32, u32);

/// One firing event of fighters or guns that a side's claim rests on.
struct Firing {
    /// The false claim chance its missed shots are claimed at.
    kind: MissKind,
    /// For each unit it is parted among, in order, what its shots really
    /// shot down and its whole shots that missed (decision 0180, 3 and 4).
    parts: Vec<FiringLine>,
    /// The pool it fired at, by type, as first counted (decision 0210, 2).
    pool: Vec<(AircraftTypeId, u32)>,
    /// The aircraft it really took from each group, added up by the
    /// group's type, in order of the type's number (decision 0210, 3).
    real_by_type: Vec<(AircraftTypeId, u32)>,
}

/// What one side's firing in one raid adds up to, for its claim (decision 2
/// of 0056): the firing events of its own, the aircraft it really shot down
/// and the whole shots that missed.
#[derive(Default)]
struct SideTally {
    /// The unit of the first firing event, which keys the claim's draws.
    first: Option<EntityRef>,
    /// Every unit that fired, in the order of its first firing event: the
    /// commands that hold the side's claim first (decision 0135, 1).
    units: Vec<EntityRef>,
    /// The firing events the claim rests on, in the order they were recorded.
    causes: Vec<EventId>,
    /// Aircraft really lost to this side's fighters and guns in the raid.
    real_shot_down: u32,
    /// Each firing event of fighters or guns, in event order (decision
    /// 0180, 3 and 4, and 0210, 2).
    firings: Vec<Firing>,
    /// The units that claim, in the order they first fired, a firing of
    /// several groups giving them in the order of their numbers (decision
    /// 0180, 1). Not the holders of the claim, which are `units`.
    claimers: Vec<EntityRef>,
    /// Each strike group that attacked, in the order the groups attacked:
    /// its whole bombs or torpedoes, and what they really did (aircraft
    /// destroyed on a base, hits on ships) (decision 0180, 5).
    strikes: Vec<(AirGroupId, u32, u32)>,
    /// Aircraft the bombs really destroyed on the ground, by the type of
    /// the group on the ground they belonged to (decision 0179, 3).
    real_on_ground_by_type: BTreeMap<AircraftTypeId, u32>,
    /// Bombs and torpedoes that really hit the ships attacked.
    hits: u32,
    /// Whole bombs and torpedoes that missed the ships attacked.
    hit_misses: u32,
    /// The ships the raid really sank.
    sunk: Vec<ShipId>,
    /// Whole bombs dropped on a base.
    bombs: u32,
    /// Aircraft the bombs really destroyed on the ground, ready or damaged.
    real_on_ground: u32,
}

impl SideTally {
    fn fired(&mut self, unit: EntityRef, event: EventId) {
        self.first.get_or_insert(unit);
        if !self.units.contains(&unit) {
            self.units.push(unit);
        }
        self.causes.push(event);
    }

    /// Notes a unit that claims, in the order units first fire.
    fn claims_for(&mut self, unit: EntityRef) {
        if !self.claimers.contains(&unit) {
            self.claimers.push(unit);
        }
    }
}

/// The claims one raid makes (decision 0056): what each side fired, as
/// recorded, kept until the raid has been resolved.
struct RaidTally {
    hex: Hex,
    /// What the raid attacked.
    target: RaidTarget,
    /// The kind the contact was taken for at the launch, for a raid on ships.
    taken_for: Option<ShipKind>,
    /// For a raid on a base, the aircraft on the ground there as first
    /// counted at the start of the step, by the type of each group, in
    /// order of the type's number (decision 0179, 2). Empty otherwise.
    field: Vec<(AircraftTypeId, u32)>,
    /// The raid's first strike group, the second part of each claim's key.
    first_group: AirGroupId,
    /// Every strike group of the raid, whose commands hold the attacking
    /// side's claim first (decision 0135, 1).
    strike_groups: Vec<AirGroupId>,
    attacker_side: Side,
    attacker: SideTally,
    defender_side: Side,
    defender: SideTally,
}

impl RaidTally {
    /// Adds a roll of fighters or guns, recorded as `event`, which really
    /// shot down `real` aircraft, `real_by_type` of them by the type of the
    /// group each was taken from.
    fn note_roll(
        &mut self,
        roll: &FireRoll,
        event: EventId,
        real: u32,
        real_by_type: Vec<(AircraftTypeId, u32)>,
    ) {
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
        let whole = whole_shots(roll.shots);
        let side = if by_attacker {
            &mut self.attacker
        } else {
            &mut self.defender
        };
        side.fired(unit, event);
        side.real_shot_down += real;
        // A firing of fighters is parted among its groups shot by shot
        // (decision 0180, 3); a firing of guns is its recorded unit's (4).
        let parts: Vec<FiringLine> = if roll.in_firing.is_empty() {
            vec![(unit, real, whole.saturating_sub(roll.hits))]
        } else {
            crate::claims::part_firing(&roll.in_firing, whole, &roll.hit_shots, real)
                .into_iter()
                .map(|part| (EntityRef::AirGroup(part.group), part.real, part.missed))
                .collect()
        };
        for (unit, _, _) in &parts {
            side.claims_for(*unit);
        }
        side.firings.push(Firing {
            kind,
            parts,
            pool: roll.pool.clone(),
            real_by_type,
        });
    }
}

/// The shots times chance of a roll: shots, expected hits, hits, and which
/// shots hit, counted from 1 in the order drawn (decision 10 of 0027,
/// decision 0033, 3, decision 0180, 3). Each shot is one draw from
/// `stream`, as [`roll_shots`] draws them.
fn roll_hits(stream: &mut RngStream, shots: u32, chance: Fixed) -> (Fixed, Fixed, u32, Vec<u32>) {
    let mut hit_shots = Vec::new();
    for shot in 1..=shots {
        if stream.chance(chance) {
            hit_shots.push(shot);
        }
    }
    let hits = u32::try_from(hit_shots.len()).expect("few shots");
    let shots = Fixed::from_int(i64::from(shots));
    let expected = shots * chance;
    (shots, expected, hits, hit_shots)
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
pub(crate) fn take_ground(
    hits: u32,
    pool: &mut [(AirGroupId, Stock, u32)],
) -> Vec<(AirGroupId, Stock, u32)> {
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
