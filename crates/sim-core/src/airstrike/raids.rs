//! Raids and fighters: the split into patrol and escorts, the raids, the
//! escorts sent, the patrol's shares and the targets (decisions 0027, 0030
//! and 0033).

use super::decision::reach_cause;
use super::*;

impl World {
    /// Splits every fighter group on combat air patrol that has aircraft
    /// into patrol and escorts (decision 2 of 0027): of the aircraft it can
    /// fly, the share, rounded down, is the patrol and the rest escort.
    /// Reads only each group's own state.
    pub(super) fn split_fighter_groups(&self) -> Vec<FighterGroup> {
        self.truth
            .air_groups
            .values()
            .filter(|group| group.mission == AirMission::CombatAirPatrol && group.ready > 0)
            // A group aboard a ship away at the yard flies nothing (decision
            // 0069); its hex is `None`, which must not read as "anywhere".
            .filter(|group| !self.truth.air_group_at_yard(group))
            .map(|group| {
                let flyable = self.aircraft_able_to_fly(group);
                let split = fighter_split(flyable, group.patrol_share);
                self.trace_choice(group.side, || {
                    (
                        Question::Air(AirQuestion::FighterSplit { group: group.id }),
                        Answer::Air(AirAnswer::FighterSplit(split)),
                    )
                });
                FighterGroup {
                    air_group: group.id,
                    side: group.side,
                    origin: group.location,
                    hex: self.truth.air_group_hex(group),
                    patrol: split.patrol,
                    escorts: split.escorts,
                    radius_nm: self.ref_data.reach_nm(group.aircraft_type, group.reach),
                    orders: group
                        .mission_order
                        .into_iter()
                        .chain(group.share_order)
                        .map(Cause::Order)
                        .chain(reach_cause(group))
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
    /// group's patrol over its own hex. Each of those draws for its pilot
    /// (decision 6a of 0059): recovered with `crew_recovered_over_own_base`
    /// for a group at a base, `crew_recovered_beside_own_force` for one
    /// aboard a ship, and a pilot not recovered is lost. The draws are one
    /// stream with [`Draw::PatrolPilot`] in the air combat subsystem, keyed
    /// on the patrol group and `firer`, the escort group whose roll took the
    /// loss: one draw for each aircraft lost. `lost_from` says which.
    #[allow(clippy::too_many_arguments)]
    pub(super) fn lose_aircraft(
        &mut self,
        day: u32,
        pulse: Pulse,
        group: AirGroupId,
        aircraft: u32,
        lost_from: SortieKind,
        cause: EventId,
        firer: Option<AirGroupId>,
    ) -> Option<(EventId, u32)> {
        let (ready, location, pilots_before) = {
            let air_group = &self.truth.air_groups[&group];
            (air_group.ready, air_group.location, air_group.pilots)
        };
        let lost = aircraft.min(ready);
        if lost == 0 {
            return None;
        }
        // A patrol's pilots draw: how many of the crews were recovered.
        let mut recovered_of: Option<(u32, Fixed)> = None;
        if let (SortieKind::Patrol, Some(_), Some(firer), Some(figures)) = (
            lost_from,
            pilots_before,
            firer,
            self.ref_data.air_combat.as_ref(),
        ) {
            let chance = match location {
                // A group lost with its base flies no patrol (decision 0090).
                AirGroupLocation::Base(_) | AirGroupLocation::Lost => {
                    figures.crew_recovered_over_own_base
                }
                AirGroupLocation::Ship(_) => figures.crew_recovered_beside_own_force,
            };
            let mut stream = self.rng_pair(
                day,
                Some(pulse),
                Subsystem::AirCombat,
                Draw::PatrolPilot,
                EntityRef::AirGroup(group),
                EntityRef::AirGroup(firer),
            );
            let recovered = roll_shots(&mut stream, Fixed::from_int(i64::from(lost)), chance);
            recovered_of = Some((recovered, chance));
        }
        let air_group = self
            .truth
            .air_groups
            .get_mut(&group)
            .expect("the air group exists");
        air_group.ready -= lost;
        let remaining = air_group.ready;
        let pilots_lost = match air_group.pilots {
            Some(pilots) if lost_from != SortieKind::Patrol => {
                let taken = lost.min(pilots);
                air_group.pilots = Some(pilots - taken);
                taken
            }
            Some(pilots) => {
                // Only a draw that was made loses a pilot.
                let unrecovered = recovered_of.map_or(0, |(recovered, _)| lost - recovered);
                let taken = unrecovered.min(pilots);
                air_group.pilots = Some(pilots - taken);
                taken
            }
            None => 0,
        };
        let pilots_left = air_group.pilots.unwrap_or(0);
        // Where a draw was made, the breakdown says how many crews were
        // recovered, whether or not a pilot was lost.
        let breakdown = recovered_of.and_then(|(recovered, chance)| {
            self.config.record_breakdowns.then(|| Breakdown {
                factors: vec![
                    Factor {
                        name: "aircraft_lost",
                        value: Fixed::from_int(i64::from(lost)),
                    },
                    Factor {
                        name: "crew_recovered_chance",
                        value: chance,
                    },
                ],
                result: Fixed::from_int(i64::from(recovered)),
            })
        });
        let event = self.events.record(
            day,
            Some(pulse),
            EventKind::AircraftLost {
                air_group: group,
                aircraft: lost,
                remaining,
            },
            vec![Cause::Event(cause)],
            breakdown,
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

    /// The enemy task force a raid of `side` finds at `hex`: the one holding
    /// the best ranked ship afloat by the target list of the raid's first
    /// strike group `first_group` (decision 4 of 0063), then the lowest force
    /// number on a tie. With the default list that is the force with the most
    /// important ship, as decision 7 of 0026 had it. Reads truth, which the
    /// umpire may.
    fn find_target(&self, first_group: AirGroupId, side: Side, hex: Hex) -> Option<TaskForceId> {
        let group = &self.truth.air_groups[&first_group];
        self.truth
            .task_forces
            .values()
            .filter(|force| force.side != side && force.position() == hex)
            .filter_map(|force| {
                let (rank, ship) = force
                    .ships
                    .iter()
                    .map(|ship| (group.target_rank(self.kind_of(*ship)), *ship))
                    .min()?;
                Some((rank, force.id, ship))
            })
            .min()
            .map(|(_, force, _)| force)
    }

    /// The ship a strike group attacks in the force its raid found: the best
    /// ranked ship afloat in it by the group's own target list, the lowest
    /// ship number on a tie (decision 4 of 0063). Reads truth, which the
    /// umpire may.
    pub(super) fn ship_to_attack(&self, group: AirGroupId, force: TaskForceId) -> ShipId {
        let group = &self.truth.air_groups[&group];
        self.truth.task_forces[&force]
            .ships
            .iter()
            .map(|ship| (group.target_rank(self.kind_of(*ship)), *ship))
            .min()
            .map(|(_, ship)| ship)
            .expect("the force the raid found has a ship afloat")
    }

    /// The most important kind among the ships afloat in `force`: the true
    /// kind of the sighting report an attack on the force sends, whichever
    /// ship was attacked (chapter 3, What a report holds).
    /// The most important kind among `ships` (decision 0012: the smallest
    /// kind is the most important).
    pub(crate) fn most_important_kind_of(&self, ships: &[ShipId]) -> ShipKind {
        ships
            .iter()
            .map(|ship| self.kind_of(*ship))
            .min()
            .expect("the force the raid found had a ship afloat")
    }

    /// What the raid whose first strike is `first` attacks, on the truth at
    /// the start of the step: the enemy task force at its hex, or the base
    /// it was sent at. A base strike always finds its base: the launch
    /// checked it exists and is held by the other side, and nothing changes
    /// that between the decision and the air combat step.
    pub(super) fn raid_target(&self, first: &StrikeInFlight) -> Option<RaidTarget> {
        match first.aim {
            Aim::Contact(_) => self
                .find_target(first.air_group, first.side, first.hex)
                .map(RaidTarget::Ships),
            Aim::Base(base) => self
                .truth
                .bases
                .get(&base)
                .filter(|target| target.side != first.side)
                .map(|_| RaidTarget::Base(base)),
        }
    }

    /// The side a raid's target belongs to.
    pub(super) fn defender_of(&self, target: RaidTarget) -> Side {
        match target {
            RaidTarget::Ships(task_force) => self.truth.task_forces[&task_force].side,
            RaidTarget::Base(base) => self.truth.bases[&base].side,
        }
    }

    /// The chance of a shot at aircraft of the type `group` flies: `base`
    /// times the pilot armour factor if the type has pilot armour, times the
    /// self-sealing tanks factor if it has them (decision 11 of 0029). The
    /// caller passes the first group of the target pool.
    pub(super) fn shot_chance(
        &self,
        base: Fixed,
        group: AirGroupId,
        air: &AirCombatFigures,
    ) -> ShotChance {
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
    pub(super) fn patrol_shares(
        &self,
        raids: &[Raid],
        fighters: &[FighterGroup],
    ) -> Vec<PatrolShare> {
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
}

/// How a fighter group that can fly `flyable` aircraft divides them (decision
/// 2 of 0027): the patrol is the share, rounded down, and the rest escort.
pub(crate) fn fighter_split(flyable: u32, patrol_share: Fixed) -> FighterSplit {
    let patrol = (Fixed::from_int(i64::from(flyable)) * patrol_share).trunc();
    let patrol = u32::try_from(patrol).expect("a share of the aircraft is not negative");
    FighterSplit {
        patrol,
        escorts: flyable - patrol,
    }
}

/// What a side's escort choice reads of one of its raids: where it leaves
/// from, where it goes, its first group and its bombers.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) struct RaidSummary {
    /// The lowest-numbered group of the raid.
    pub(crate) lead: AirGroupId,
    pub(crate) origin: AirGroupLocation,
    pub(crate) hex: Hex,
    pub(crate) bombers: u32,
}

fn summarise(raids: &[Raid]) -> Vec<RaidSummary> {
    raids
        .iter()
        .map(|raid| RaidSummary {
            lead: raid.strikes[0].air_group,
            origin: raid.strikes[0].origin,
            hex: raid.strikes[0].hex,
            bombers: raid.strikes.iter().map(|strike| strike.aircraft).sum(),
        })
        .collect()
}

/// The raids of `side` among `strikes` (the strikes in flight), in the order
/// [`group_raids`] gives them. A side's raids are built from its own strikes
/// only.
pub(crate) fn raids_of_side(strikes: &[StrikeInFlight], side: Side) -> Vec<RaidSummary> {
    let own: Vec<StrikeInFlight> = strikes
        .iter()
        .filter(|strike| strike.side == side)
        .cloned()
        .collect();
    summarise(&group_raids(&own))
}

/// The raid, among `raids`, a fighter group whose escorts leave from `origin`
/// would join: of the raids from its place the one with the most bombers, then
/// the lowest first group (decision 5 of 0027). The raid's index in `raids`,
/// and whether its target hex is within `radius_nm` of the group's `hex`
/// (decision 0030): a group with no position cannot be measured and sends its
/// escorts. `None`: no raid leaves from its place.
fn escort_raid(
    raids: &[RaidSummary],
    origin: AirGroupLocation,
    hex: Option<Hex>,
    radius_nm: Fixed,
) -> Option<(usize, bool)> {
    let index = raids
        .iter()
        .enumerate()
        .filter(|(_, raid)| raid.origin == origin)
        .max_by_key(|(_, raid)| (raid.bombers, std::cmp::Reverse(raid.lead)))
        .map(|(index, _)| index)?;
    let within = hex.is_none_or(|hex| hex.air_distance_within(raids[index].hex, radius_nm));
    Some((index, within))
}

/// What the escorts of a fighter group do (decisions 5 of 0027 and 0030): go
/// with the raid from its place that has the most bombers, then the lowest
/// first group, if that raid's target is within the fighter type's radius of
/// the group; else, with a raid from its place, stay with the patrol; with no
/// raid from its place, send none. `raids` are the side's own raids of the
/// pulse. The engine and a side's view both call it.
pub(crate) fn escort_choice(
    raids: &[RaidSummary],
    origin: AirGroupLocation,
    hex: Option<Hex>,
    radius_nm: Fixed,
) -> EscortChoice {
    match escort_raid(raids, origin, hex, radius_nm) {
        None => EscortChoice::NoRaid,
        Some((index, true)) => EscortChoice::Goes {
            lead: raids[index].lead,
            hex: raids[index].hex,
        },
        Some((_, false)) => EscortChoice::Stays,
    }
}

/// A sortie out to `to` and back from `from`: twice the distance (decision 4
/// of 0035).
pub(super) fn out_and_back(from: Hex, to: Hex) -> Fixed {
    Fixed::from_int(i64::from(from.air_distance_nm(to)) * 2)
}

/// Groups the strikes into raids (decision 4 of 0027): the same place and
/// the same hex. Strikes are in group order, so the raids come out in order
/// of their first group.
pub(super) fn group_raids(strikes: &[StrikeInFlight]) -> Vec<Raid> {
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
/// cannot be measured and sends its escorts as before. The choice is
/// [`escort_choice`]. Returns it for each group that has escorts, by index
/// into `fighters`, for the caller to trace.
pub(super) fn send_escorts(
    raids: &mut [Raid],
    fighters: &mut [FighterGroup],
) -> Vec<(usize, EscortChoice)> {
    let summaries = summarise(raids);
    let mut made = Vec::new();
    for (index, group) in fighters.iter_mut().enumerate() {
        if group.escorts == 0 {
            continue;
        }
        let chosen = escort_raid(&summaries, group.origin, group.hex, group.radius_nm);
        made.push((
            index,
            escort_choice(&summaries, group.origin, group.hex, group.radius_nm),
        ));
        if let Some((raid_index, within)) = chosen {
            if within {
                raids[raid_index].escorts.push(index);
            } else {
                group.patrol += group.escorts;
                group.escorts = 0;
            }
        }
    }
    made
}
