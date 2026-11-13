//! What `batch` counts from one run's event log and end state (decision
//! 0062; `shots_by` was added at version 3, and who sighted first in each
//! fight at version 4). It only reads: the log, the truth after each day and at the end.
//! Every quantity is a whole number or, for a `Fixed`, a decimal string. A
//! list is empty when there is nothing, and every key is always written, so
//! a reader can tell an old file (key missing) from a run with none.
//!
//! The counts are those of the events the decision names; none of them is
//! read from the world's state except the systems out at the end and the
//! aircraft ready at the end of each day.

use std::collections::{BTreeMap, BTreeSet};

use serde::Serialize;
use sim_core::events::{Cause, EventKind, SupplyUse, Weapon};
use sim_core::fixed::Fixed;
use sim_core::ids::{AirGroupId, BaseId, EventId, ShipId, TaskForceId};
use sim_core::state::{ShipLocation, ShipSystem, Side, SystemState, TruthState};
use sim_core::time::Pulse;
use sim_core::World;

/// The sides in the order the file lists them.
const SIDES: [Side; 2] = [Side::Japanese, Side::Allied];

fn pulse_name(pulse: Option<Pulse>) -> &'static str {
    match pulse {
        Some(Pulse::Night) => "night",
        Some(Pulse::Morning) => "morning",
        Some(Pulse::Afternoon) => "afternoon",
        None => "",
    }
}

/// A system's name in lower snake case, as the file writes it.
fn system_name(system: ShipSystem) -> &'static str {
    match system {
        ShipSystem::Propulsion => "propulsion",
        ShipSystem::FireControl => "fire_control",
        ShipSystem::MainBattery => "main_battery",
        ShipSystem::TorpedoMounts => "torpedo_mounts",
        ShipSystem::FlightDeck => "flight_deck",
    }
}

fn side_name(side: Side) -> String {
    side.name().to_lowercase()
}

// ---------------------------------------------------------------- header

#[derive(Serialize)]
pub struct TaskForceOut {
    id: u32,
    name: String,
    side: String,
}

#[derive(Serialize)]
pub struct AirGroupOut {
    id: u32,
    name: String,
    side: String,
    aircraft_type: String,
}

#[derive(Serialize)]
pub struct BaseOut {
    id: u32,
    name: String,
    side: String,
}

/// The tables written once, from the state before day 1.
pub struct Header {
    pub task_forces: Vec<TaskForceOut>,
    pub air_groups: Vec<AirGroupOut>,
    pub bases: Vec<BaseOut>,
}

/// The initial task force of each ship (none for a ship in port).
pub fn initial_force_of(initial: &TruthState) -> BTreeMap<ShipId, TaskForceId> {
    let mut out = BTreeMap::new();
    for force in initial.task_forces.values() {
        for ship in &force.ships {
            out.insert(*ship, force.id);
        }
    }
    out
}

pub fn header(initial: &TruthState, world: &World) -> Header {
    let types = &world.ref_data().aircraft_types;
    Header {
        task_forces: initial
            .task_forces
            .values()
            .map(|force| TaskForceOut {
                id: force.id.raw(),
                name: force.name.clone(),
                side: side_name(force.side),
            })
            .collect(),
        air_groups: initial
            .air_groups
            .values()
            .map(|group| AirGroupOut {
                id: group.id.raw(),
                name: group.name.clone(),
                side: side_name(group.side),
                aircraft_type: types[&group.aircraft_type].name.clone(),
            })
            .collect(),
        bases: initial
            .bases
            .values()
            .map(|base| BaseOut {
                id: base.id.raw(),
                name: base.name.clone(),
                side: side_name(base.side),
            })
            .collect(),
    }
}

// --------------------------------------------------------------- per run

#[derive(Serialize)]
pub struct HitsOn {
    ship: u32,
    gun: u64,
    torpedo: u64,
    bomb: u64,
}

/// The shots one ship fired in surface fights over a run, by weapon: exact
/// decimal strings, as `fire` writes them. A ship that fired nothing has no
/// entry.
#[derive(Serialize)]
pub struct ShotsBy {
    ship: u32,
    gun: String,
    torpedo: String,
}

#[derive(Serialize, Default)]
pub struct WeaponFire {
    /// The sum of the salvos' shots, exact, as a decimal string.
    shots: String,
    hits: u64,
}

#[derive(Serialize)]
pub struct SideFire {
    gun: WeaponFire,
    torpedo: WeaponFire,
}

#[derive(Serialize)]
pub struct Fire {
    japanese: SideFire,
    allied: SideFire,
}

#[derive(Serialize)]
pub struct SystemOut {
    ship: u32,
    system: &'static str,
}

#[derive(Serialize, Default)]
pub struct AirLoss {
    air_group: u32,
    shot_down: u64,
    destroyed_on_ground: u64,
    damaged_destroyed_on_ground: u64,
    lost_with_ship: u64,
    worn_out: u64,
    ditched_fuel: u64,
    ditched_no_deck: u64,
    pilots_lost: u64,
}

#[derive(Serialize)]
pub struct ReadyRow {
    air_group: u32,
    /// The day `by_day[0]` is: always 1, so `by_day[i]` is day `i + 1`.
    first_day: u32,
    /// The aircraft ready at the end of each day, day 1 first. 0 for a day
    /// before the group existed.
    by_day: Vec<u32>,
}

#[derive(Serialize, Default)]
pub struct SideClaims {
    sunk: u64,
    hits: u64,
    shot_down: u64,
    on_ground: u64,
}

#[derive(Serialize)]
pub struct Claims {
    japanese: SideClaims,
    allied: SideClaims,
}

#[derive(Serialize)]
pub struct AttackOut {
    day: u32,
    pulse: &'static str,
    air_group: u32,
    /// `ship` or `base`.
    target_kind: &'static str,
    /// The ship attacked, or the base.
    target: u32,
    /// For a ship attack: `gun`, `torpedo` or `bomb`.
    #[serde(skip_serializing_if = "Option::is_none")]
    weapon: Option<&'static str>,
    /// The aircraft that attacked.
    aircraft: u64,
}

#[derive(Serialize)]
pub struct EscortOut {
    day: u32,
    pulse: &'static str,
    air_group: u32,
    with_air_group: u32,
    aircraft: u32,
}

#[derive(Serialize)]
pub struct BrokeOff {
    day: u32,
    pulse: &'static str,
    task_force: u32,
    side: String,
    round: u32,
}

#[derive(Serialize)]
pub struct FightForceOut {
    task_force: u32,
    side: String,
    /// The first round in which the force had sighted the enemy (a
    /// successful roll in it or an earlier round, or fire on it in an earlier
    /// round), within the rounds fought; null if there is none.
    first_sighted_round: Option<u32>,
    /// The rounds in which this force had not sighted and the enemy had.
    rounds_surprised: u32,
}

#[derive(Serialize)]
pub struct FightOut {
    day: u32,
    pulse: &'static str,
    /// The rounds fought: the round the fight ended in, whatever the reason.
    rounds: u32,
    /// `allied`, `japanese`, `both` or `neither`: whose force first had
    /// sighted the enemy.
    sighted_first: &'static str,
    /// The two forces, lower task force ID first.
    forces: Vec<FightForceOut>,
}

/// What the log says of one force in one fight, the input of the sighting
/// record.
#[derive(Default, Clone)]
struct ForceFacts {
    /// The round of the force's successful sighting roll, if it made one.
    roll_round: Option<u32>,
    /// The rounds in which a ship of the force was the target of a salvo.
    fired_on: BTreeSet<u32>,
}

impl ForceFacts {
    /// Whether the force has sighted the enemy in `round`: it rolled a
    /// sighting in this round or an earlier one, or it was fired on in an
    /// earlier round (docs/rules/04, Sighting).
    fn has_sighted(&self, round: u32) -> bool {
        self.roll_round.is_some_and(|rolled| rolled <= round)
            || self.fired_on.iter().any(|fired| *fired < round)
    }

    /// The first round, within `rounds`, in which the force has sighted.
    fn first_sighted(&self, rounds: u32) -> Option<u32> {
        let by_fire = self.fired_on.iter().next().map(|fired| fired + 1);
        [self.roll_round, by_fire]
            .into_iter()
            .flatten()
            .min()
            .filter(|round| *round <= rounds)
    }
}

/// The sighting record of one fight of `rounds` rounds from the facts of its
/// two forces: each force's first round of having sighted and the rounds it
/// was surprised, meaning that it had not sighted and the enemy had. It does
/// not depend on whether a `ForceSurprised` event was recorded: that needs
/// the enemy to have shots, and this is a record of who knew.
fn sighting_record(rounds: u32, facts: &[ForceFacts; 2]) -> ([Option<u32>; 2], [u32; 2]) {
    let first = [
        facts[0].first_sighted(rounds),
        facts[1].first_sighted(rounds),
    ];
    let surprised = [0, 1].map(|me| {
        (1..=rounds)
            .filter(|round| !facts[me].has_sighted(*round) && facts[1 - me].has_sighted(*round))
            .count() as u32
    });
    (first, surprised)
}

/// Whose force had sighted first, given the forces' sides and first rounds.
fn sighted_first(sides: [Side; 2], first: [Option<u32>; 2]) -> &'static str {
    match (first[0], first[1]) {
        (None, None) => "neither",
        (Some(a), Some(b)) if a == b => "both",
        (Some(_), None) => side_label(sides[0]),
        (None, Some(_)) => side_label(sides[1]),
        (Some(a), Some(b)) => side_label(sides[usize::from(b < a)]),
    }
}

fn side_label(side: Side) -> &'static str {
    match side {
        Side::Japanese => "japanese",
        Side::Allied => "allied",
    }
}

/// A fight whose `EngagementEnded` has not come yet.
struct OpenFight {
    a: TaskForceId,
    b: TaskForceId,
    facts: BTreeMap<TaskForceId, ForceFacts>,
}

#[derive(Default)]
struct SupplyTotals {
    sortie_aircraft: u64,
    sorties: Fixed,
    upkeep: Fixed,
    repairs: Fixed,
}

#[derive(Serialize)]
pub struct SupplyOut {
    base: u32,
    sortie_aircraft: u64,
    sorties_t: String,
    upkeep_t: String,
    repairs_t: String,
}

#[derive(Serialize)]
pub struct FormedGroup {
    id: u32,
    name: String,
    side: String,
    aircraft_type: String,
    /// The air group whose aircraft landed and formed it.
    from: u32,
}

/// Everything one run adds to its record.
#[derive(Serialize)]
pub struct RunCounts {
    pub hits_on: Vec<HitsOn>,
    pub fire: Fire,
    pub shots_by: Vec<ShotsBy>,
    pub systems_out: Vec<SystemOut>,
    pub air_losses: Vec<AirLoss>,
    pub ready: Vec<ReadyRow>,
    pub claims: Claims,
    pub attacks: Vec<AttackOut>,
    pub escorts: Vec<EscortOut>,
    pub broke_off: Vec<BrokeOff>,
    pub fights: Vec<FightOut>,
    pub supply: Vec<SupplyOut>,
    pub formed_groups: Vec<FormedGroup>,
}

/// The aircraft ready in each air group, taken after a day is resolved.
pub fn ready_now(truth: &TruthState) -> BTreeMap<AirGroupId, u32> {
    truth
        .air_groups
        .values()
        .map(|group| (group.id, group.ready))
        .collect()
}

pub fn count(
    world: &World,
    initial: &TruthState,
    ready_by_day: &[BTreeMap<AirGroupId, u32>],
) -> RunCounts {
    let truth = world.umpire();
    let side_of_ship = |ship: &ShipId| truth.ships.get(ship).map(|s| s.side);
    let types = &world.ref_data().aircraft_types;

    let mut hits_on: BTreeMap<ShipId, [u64; 3]> = BTreeMap::new();
    // [side][gun, torpedo] -> (shots, hits)
    let mut fire = [[(Fixed::ZERO, 0u64); 2]; 2];
    // The shots each ship fired in surface fights: [gun, torpedo].
    let mut shots_by: BTreeMap<ShipId, [Fixed; 2]> = BTreeMap::new();
    let mut losses: BTreeMap<AirGroupId, AirLoss> = BTreeMap::new();
    let mut claims = [[0u64; 4]; 2];
    let mut attacks = Vec::new();
    let mut escorts = Vec::new();
    let mut broke_off = Vec::new();
    let mut fights = Vec::new();
    let mut supply: BTreeMap<BaseId, SupplyTotals> = BTreeMap::new();
    let mut formed_from: BTreeMap<AirGroupId, AirGroupId> = BTreeMap::new();
    // Fights not yet ended, by the `EngagementBegun` event that the fight's
    // sighting rolls and salvos cite as a cause. The force a ship is in is
    // taken from the start and from `ShipsDetached`; a ship that sinks stays
    // in the map, so a salvo at it is still tied to its force.
    let mut open_fights: BTreeMap<EventId, OpenFight> = BTreeMap::new();
    let mut force_of = initial_force_of(initial);

    fn loss(losses: &mut BTreeMap<AirGroupId, AirLoss>, group: AirGroupId) -> &mut AirLoss {
        losses.entry(group).or_insert_with(|| AirLoss {
            air_group: group.raw(),
            ..AirLoss::default()
        })
    }
    let slot = |side: Side| SIDES.iter().position(|s| *s == side).expect("a side");

    // The fight an event belongs to: the open one it cites as a cause.
    fn fight_of<'a>(
        open: &'a mut BTreeMap<EventId, OpenFight>,
        causes: &[Cause],
    ) -> Option<&'a mut OpenFight> {
        let id = causes.iter().find_map(|cause| match cause {
            Cause::Event(id) if open.contains_key(id) => Some(*id),
            _ => None,
        })?;
        open.get_mut(&id)
    }

    for event in world.events().iter() {
        match &event.kind {
            EventKind::EngagementBegun { a, b, .. } => {
                open_fights.insert(
                    event.id,
                    OpenFight {
                        a: *a,
                        b: *b,
                        facts: BTreeMap::new(),
                    },
                );
            }
            EventKind::SightingRoll {
                round,
                task_force,
                sighted: true,
                ..
            } => {
                if let Some(fight) = fight_of(&mut open_fights, &event.causes) {
                    let facts = fight.facts.entry(*task_force).or_default();
                    facts.roll_round = Some(facts.roll_round.map_or(*round, |old| old.min(*round)));
                }
            }
            EventKind::ShipsDetached {
                new_force,
                ships,
                escorts,
                ..
            } => {
                for ship in ships.iter().chain(escorts) {
                    force_of.insert(*ship, *new_force);
                }
            }
            EventKind::Salvo {
                firer,
                target,
                weapon,
                shots,
                hits,
                round,
                ..
            } => {
                if let (Some(fight), Some(force)) = (
                    fight_of(&mut open_fights, &event.causes),
                    force_of.get(target),
                ) {
                    fight
                        .facts
                        .entry(*force)
                        .or_default()
                        .fired_on
                        .insert(*round);
                }
                let hits = u64::from(*hits);
                let column = match weapon {
                    Weapon::Torpedo => 1,
                    _ => 0,
                };
                if hits > 0 {
                    hits_on.entry(*target).or_default()[column] += hits;
                }
                shots_by.entry(*firer).or_insert([Fixed::ZERO; 2])[column] += *shots;
                if let Some(side) = side_of_ship(firer) {
                    let cell = &mut fire[slot(side)][column];
                    cell.0 += *shots;
                    cell.1 += hits;
                }
            }
            EventKind::AirAttack {
                air_group,
                ship,
                weapon,
                shots,
                hits,
                ..
            } => {
                let hits = u64::from(*hits);
                let (column, name) = match weapon {
                    Weapon::Gun => (0, "gun"),
                    Weapon::Torpedo => (1, "torpedo"),
                    Weapon::Bomb => (2, "bomb"),
                };
                if hits > 0 {
                    hits_on.entry(*ship).or_default()[column] += hits;
                }
                attacks.push(AttackOut {
                    day: event.day,
                    pulse: pulse_name(event.pulse),
                    air_group: air_group.raw(),
                    target_kind: "ship",
                    target: ship.raw(),
                    weapon: Some(name),
                    aircraft: shots.trunc().unsigned_abs(),
                });
            }
            EventKind::BaseAttacked {
                air_group,
                base,
                shots,
                ..
            } => attacks.push(AttackOut {
                day: event.day,
                pulse: pulse_name(event.pulse),
                air_group: air_group.raw(),
                target_kind: "base",
                target: base.raw(),
                weapon: None,
                aircraft: shots.trunc().unsigned_abs(),
            }),
            EventKind::EscortsSent {
                air_group,
                with_air_group,
                aircraft,
            } => escorts.push(EscortOut {
                day: event.day,
                pulse: pulse_name(event.pulse),
                air_group: air_group.raw(),
                with_air_group: with_air_group.raw(),
                aircraft: *aircraft,
            }),
            EventKind::AircraftLost {
                air_group,
                aircraft,
                ..
            } => loss(&mut losses, *air_group).shot_down += u64::from(*aircraft),
            EventKind::AircraftDestroyedOnGround {
                air_group,
                aircraft,
                ..
            } => loss(&mut losses, *air_group).destroyed_on_ground += u64::from(*aircraft),
            EventKind::DamagedAircraftDestroyedOnGround {
                air_group,
                aircraft,
                ..
            } => loss(&mut losses, *air_group).damaged_destroyed_on_ground += u64::from(*aircraft),
            EventKind::AirGroupLost {
                air_group,
                aircraft,
                ..
            } => loss(&mut losses, *air_group).lost_with_ship += u64::from(*aircraft),
            EventKind::AircraftWorn {
                air_group,
                written_off,
                ..
            } => loss(&mut losses, *air_group).worn_out += u64::from(*written_off),
            EventKind::AircraftDitched {
                air_group, ditched, ..
            } => loss(&mut losses, *air_group).ditched_fuel += u64::from(*ditched),
            EventKind::AircraftDitchedNoDeck {
                air_group,
                aircraft,
                ..
            } => loss(&mut losses, *air_group).ditched_no_deck += u64::from(*aircraft),
            EventKind::PilotsLost {
                air_group, lost, ..
            } => loss(&mut losses, *air_group).pilots_lost += u64::from(*lost),
            EventKind::AircraftLandedElsewhere {
                from, to, formed, ..
            } if *formed => {
                formed_from.insert(*to, *from);
            }
            EventKind::ClaimReceived {
                side,
                claimed_sunk,
                claimed_hits,
                claimed_shot_down,
                claimed_on_ground,
                ..
            } => {
                let cell = &mut claims[slot(*side)];
                cell[0] += u64::from(*claimed_sunk);
                cell[1] += u64::from(*claimed_hits);
                cell[2] += u64::from(*claimed_shot_down);
                cell[3] += u64::from(*claimed_on_ground);
            }
            EventKind::ForceBrokeOff {
                round, task_force, ..
            } => {
                let side = truth
                    .task_forces
                    .get(task_force)
                    .or_else(|| initial.task_forces.get(task_force))
                    .map(|force| side_name(force.side))
                    .unwrap_or_default();
                broke_off.push(BrokeOff {
                    day: event.day,
                    pulse: pulse_name(event.pulse),
                    task_force: task_force.raw(),
                    side,
                    round: *round,
                });
            }
            EventKind::EngagementEnded { round, a, b, .. } => {
                // The fights of a pulse are held one after another, so one
                // open fight has this pair.
                let begun = open_fights
                    .iter()
                    .rev()
                    .find(|(_, fight)| fight.a == *a && fight.b == *b)
                    .map(|(id, _)| *id);
                let fight = begun
                    .and_then(|id| open_fights.remove(&id))
                    .expect("an engagement ends a fight that began");
                let [low, high] = if a <= b { [*a, *b] } else { [*b, *a] };
                let facts = [low, high].map(|id| fight.facts.get(&id).cloned().unwrap_or_default());
                let (first, surprised) = sighting_record(*round, &facts);
                let sides = [low, high].map(|id| truth.task_forces[&id].side);
                fights.push(FightOut {
                    day: event.day,
                    pulse: pulse_name(event.pulse),
                    rounds: *round,
                    sighted_first: sighted_first(sides, first),
                    forces: [low, high]
                        .iter()
                        .enumerate()
                        .map(|(index, id)| FightForceOut {
                            task_force: id.raw(),
                            side: side_name(sides[index]),
                            first_sighted_round: first[index],
                            rounds_surprised: surprised[index],
                        })
                        .collect(),
                });
            }
            EventKind::BaseSupplySpent {
                base,
                reason,
                aircraft,
                paid_t,
                ..
            } => {
                let totals = supply.entry(*base).or_default();
                match reason {
                    SupplyUse::Sorties => {
                        totals.sortie_aircraft += u64::from(*aircraft);
                        totals.sorties += *paid_t;
                    }
                    SupplyUse::Upkeep => totals.upkeep += *paid_t,
                    SupplyUse::Repairs => totals.repairs += *paid_t,
                }
            }
            _ => {}
        }
    }

    // Systems out, for ships afloat or in port at the end.
    let mut systems_out = Vec::new();
    for ship in truth.ships.values() {
        if ship.location == ShipLocation::Sunk {
            continue;
        }
        for system in ShipSystem::ALL {
            if ship.systems.get(system) == SystemState::Out {
                systems_out.push(SystemOut {
                    ship: ship.id.raw(),
                    system: system_name(system),
                });
            }
        }
    }

    // Aircraft ready at the end of each day, for every group that ever
    // existed in the run.
    let mut ready = Vec::new();
    for id in truth.air_groups.keys() {
        ready.push(ReadyRow {
            air_group: id.raw(),
            first_day: 1,
            by_day: ready_by_day
                .iter()
                .map(|day| day.get(id).copied().unwrap_or(0))
                .collect(),
        });
    }

    let formed_groups = truth
        .air_groups
        .values()
        .filter(|group| !initial.air_groups.contains_key(&group.id))
        .map(|group| FormedGroup {
            id: group.id.raw(),
            name: group.name.clone(),
            side: side_name(group.side),
            aircraft_type: types[&group.aircraft_type].name.clone(),
            from: formed_from.get(&group.id).map_or(0, |from| from.raw()),
        })
        .collect();

    let weapon_fire = |cell: (Fixed, u64)| WeaponFire {
        shots: cell.0.to_string(),
        hits: cell.1,
    };
    let side_fire = |index: usize| SideFire {
        gun: weapon_fire(fire[index][0]),
        torpedo: weapon_fire(fire[index][1]),
    };
    let side_claims = |index: usize| SideClaims {
        sunk: claims[index][0],
        hits: claims[index][1],
        shot_down: claims[index][2],
        on_ground: claims[index][3],
    };

    RunCounts {
        hits_on: hits_on
            .into_iter()
            .map(|(ship, [gun, torpedo, bomb])| HitsOn {
                ship: ship.raw(),
                gun,
                torpedo,
                bomb,
            })
            .collect(),
        fire: Fire {
            japanese: side_fire(0),
            allied: side_fire(1),
        },
        shots_by: shots_by
            .into_iter()
            .map(|(ship, [gun, torpedo])| ShotsBy {
                ship: ship.raw(),
                gun: gun.to_string(),
                torpedo: torpedo.to_string(),
            })
            .collect(),
        systems_out,
        air_losses: losses.into_values().collect(),
        ready,
        claims: Claims {
            japanese: side_claims(0),
            allied: side_claims(1),
        },
        attacks,
        escorts,
        broke_off,
        fights,
        supply: supply
            .into_iter()
            .map(|(base, totals)| SupplyOut {
                base: base.raw(),
                sortie_aircraft: totals.sortie_aircraft,
                sorties_t: totals.sorties.to_string(),
                upkeep_t: totals.upkeep.to_string(),
                repairs_t: totals.repairs.to_string(),
            })
            .collect(),
        formed_groups,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The sighting record of a fight, from its facts (the architect's made
    /// cases, A to F; the same text is in `tests/batch.rs`). X is task force
    /// 1, Japanese; Y is task force 2, Allied.
    ///
    /// A. Both roll and sight in round 1; 3 rounds. "both"; X first 1,
    ///    surprised 0; Y first 1, surprised 0.
    /// B. Neither sights by roll and nobody fires; 6 rounds. "neither"; both
    ///    first null, surprised 0.
    /// C. X sights by roll in round 2 and fires on Y in rounds 2, 3 and 4; Y
    ///    never sights by roll; 4 rounds. X first 2, Y first 3. "japanese".
    ///    Y surprised 1 (round 2); X surprised 0. In round 1 neither had
    ///    sighted, so nobody was surprised.
    /// D. X sights by roll in round 1 and never fires (it has no shots); Y
    ///    fails in rounds 1 and 2 and sights by roll in round 3; 3 rounds.
    ///    "japanese"; X first 1, surprised 0; Y first 3, surprised 2.
    /// E. Y sights by roll in round 1 and never fires; X sights by roll in
    ///    round 3; 4 rounds. "allied"; Y first 1, surprised 0; X first 3,
    ///    surprised 2.
    /// F. X sights by roll in round 1 and fires on Y in round 1; the fight
    ///    ends in round 1. "japanese"; X first 1, surprised 0; Y first null,
    ///    surprised 1.
    #[test]
    fn the_sighting_record_of_the_architects_six_cases() {
        let roll = |round: u32| ForceFacts {
            roll_round: Some(round),
            fired_on: BTreeSet::new(),
        };
        let none = ForceFacts::default;
        let fired = |rounds: &[u32]| ForceFacts {
            roll_round: None,
            fired_on: rounds.iter().copied().collect(),
        };
        let sides = [Side::Japanese, Side::Allied];
        // (rounds, [X, Y], who, first, surprised)
        type Case = (
            u32,
            [ForceFacts; 2],
            &'static str,
            [Option<u32>; 2],
            [u32; 2],
        );
        let cases: [Case; 6] = [
            (3, [roll(1), roll(1)], "both", [Some(1), Some(1)], [0, 0]),
            (6, [none(), none()], "neither", [None, None], [0, 0]),
            (
                4,
                [roll(2), fired(&[2, 3, 4])],
                "japanese",
                [Some(2), Some(3)],
                [0, 1],
            ),
            (
                3,
                [roll(1), roll(3)],
                "japanese",
                [Some(1), Some(3)],
                [0, 2],
            ),
            (4, [roll(3), roll(1)], "allied", [Some(3), Some(1)], [2, 0]),
            (
                1,
                [
                    roll(1),
                    ForceFacts {
                        roll_round: None,
                        fired_on: BTreeSet::from([1]),
                    },
                ],
                "japanese",
                [Some(1), None],
                [0, 1],
            ),
        ];
        for (index, (rounds, facts, who, first, surprised)) in cases.into_iter().enumerate() {
            let (got_first, got_surprised) = sighting_record(rounds, &facts);
            assert_eq!(got_first, first, "case {index}: first rounds");
            assert_eq!(got_surprised, surprised, "case {index}: rounds surprised");
            assert_eq!(sighted_first(sides, got_first), who, "case {index}: who");
        }
    }
}
