//! The aircraft flying and the aircraft on the ground in a pulse, and the
//! hits of a raid on a base (decisions 0034, 0035 and 0036).

use super::*;

impl World {
    /// The aircraft each group flies in this pulse, on the state at the
    /// start of the air combat step, after the escorts are sent (decision 11
    /// of 0036): the aircraft a group on search sends, the aircraft a strike
    /// launched, and the patrol and the escorts that go of a fighter group.
    /// A group not in the map flies none.
    pub(super) fn aircraft_flying(
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
            if self.searches_in(group, pulse)
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
    pub(super) fn aircraft_on_the_ground(
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

    /// Applies the hits of one strike group's bombs, or of one force's
    /// shells, on a base (decisions 8 and 9 of 0034, decisions 4 and 9 of
    /// 0092): aircraft are destroyed on the ground, the lowest group first,
    /// no more than `pool` says were there at the start, supply is
    /// destroyed, no more than the base has, and men ashore are lost, no
    /// more than are there. Records each, citing `attack_event`. Gives the
    /// aircraft really destroyed, by group, in the order taken (a group
    /// twice where its damaged and its ready aircraft were both hit), each
    /// as really taken from the group (decision 0179, 3).
    pub(crate) fn apply_base_hits(
        &mut self,
        day: u32,
        pulse: Option<Pulse>,
        base: BaseId,
        hits: BaseHits,
        attack_event: EventId,
        pool: &mut [(AirGroupId, Stock, u32)],
    ) -> Vec<(AirGroupId, u32)> {
        let per_hit = self
            .ref_data
            .air_combat
            .as_ref()
            .expect("hits on a base need the air combat figures")
            .base_supply_destroyed_per_hit_t;
        let on_ground: u32 = pool.iter().map(|(_, _, aircraft)| aircraft).sum();
        let breakdown = self.config.record_breakdowns.then(|| Breakdown {
            factors: vec![
                Factor {
                    name: "hits_on_aircraft",
                    value: Fixed::from_int(i64::from(hits.aircraft)),
                },
                Factor {
                    name: "aircraft_on_ground",
                    value: Fixed::from_int(i64::from(on_ground)),
                },
            ],
            result: Fixed::from_int(i64::from(hits.aircraft.min(on_ground))),
        });
        // The lowest group first, its damaged aircraft before its ready ones
        // (decision 11 of 0035): one event for each kind of aircraft hit.
        let mut destroyed_by_group = Vec::new();
        for (group, stock, destroyed) in take_ground(hits.aircraft, pool) {
            let air_group = self
                .truth
                .air_groups
                .get_mut(&group)
                .expect("the air group exists");
            let event = match stock {
                Stock::Ready => {
                    let destroyed = destroyed.min(air_group.ready);
                    air_group.ready -= destroyed;
                    destroyed_by_group.push((group, destroyed));
                    EventKind::AircraftDestroyedOnGround {
                        air_group: group,
                        base,
                        aircraft: destroyed,
                        remaining: air_group.ready,
                    }
                }
                Stock::Damaged => {
                    let destroyed = destroyed.min(air_group.damaged);
                    air_group.damaged -= destroyed;
                    destroyed_by_group.push((group, destroyed));
                    EventKind::DamagedAircraftDestroyedOnGround {
                        air_group: group,
                        base,
                        aircraft: destroyed,
                        remaining: air_group.damaged,
                    }
                }
            };
            self.events.record(
                day,
                pulse,
                event,
                vec![Cause::Event(attack_event)],
                breakdown.clone(),
            );
        }

        let target = self.truth.bases.get_mut(&base).expect("the base exists");
        let before = target.supply_t;
        let wanted = per_hit * Fixed::from_int(i64::from(hits.supply));
        let destroyed = wanted.min(before);
        if destroyed > Fixed::ZERO {
            target.supply_t = before - destroyed;
            let after = target.supply_t;
            let breakdown = self.config.record_breakdowns.then(|| Breakdown {
                factors: vec![
                    Factor {
                        name: "hits_on_supply",
                        value: Fixed::from_int(i64::from(hits.supply)),
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
                pulse,
                EventKind::BaseSupplyDestroyed {
                    base,
                    destroyed_t: destroyed,
                    supply_t: after,
                },
                vec![Cause::Event(attack_event)],
                breakdown,
            );
        }
        self.apply_hits_on_men(day, pulse, base, hits.men, attack_event);
        self.apply_hits_on_airfield(day, pulse, base, hits.airfield, attack_event);
        destroyed_by_group
    }
}
