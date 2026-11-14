//! Shore bombardment, and hits on men ashore from shells and bombs
//! (`docs/decisions/0092-shells-bombs-and-the-enemy-ashore.md`, decisions 1
//! to 10 and 17).
//!
//! The shore bombardment phase runs in the night pulse after the surface
//! engagements. Forces are taken in force order. A force on the bombardment
//! mission with a ship afloat whose position is a hex with an enemy base on
//! its side's map bombards the lowest-numbered such base. Each ship, in ship
//! order, fires its guns for `bombardment_minutes` by the gun rule of a
//! round of fire (decision 2), and its shells are weighed by its gun damage
//! (decision 5). A ship with no shell to fire does not fire. Each weighed
//! shot hits at `bombardment_hit_chance`, and each hit lands by the table of
//! a hit on a base (decision 4 of 0034 and decision 8 of 0092). The force's
//! hits then land together: aircraft, every one at the base being on the
//! ground at night, supply and men. The base's side is given a sighting
//! report of the force (decision 7) and the bombarding side a claim
//! (decision 17).
//!
//! Draws: one stream for each firing ship, keyed on the day, the night
//! pulse, the surface combat subsystem, `Draw::Bombardment`, the ship and
//! the base: the ship's shots, then where each hit lands, then, on the first
//! firing ship's stream only, the sighting report. The claim draws on a
//! stream keyed on the day, the pulse, the intelligence subsystem,
//! `Draw::Claim`, the first firing ship and the base.
//!
//! Reads: a force's mission, position and ships; a ship's guns, rounds and
//! main battery; the bases in the hex, their side, stock and whether they
//! are on the map; the air groups at a base; the land units ashore there;
//! the figures. Writes: a ship's rounds; an air group's aircraft; a base's
//! supply; a land unit's men; the base's side's contacts; the bombarding
//! side's claims; the events.

use crate::airstrike::{BaseHits, Stock};
use crate::belief::{ApplyOutcome, BattleClaim, Report};
use crate::damage::{raw_u64, roll_shots};
use crate::events::{Breakdown, Cause, EventKind, Factor};
use crate::fixed::Fixed;
use crate::ground::take_in_proportion;
use crate::hex::Hex;
use crate::ids::{AirGroupId, BaseId, EntityRef, EventId, LandUnitId, ShipId, TaskForceId};
use crate::orders::Mission;
use crate::refdata::{AirCombatFigures, BombardmentFigures};
use crate::rng::{Draw, RngStream, Subsystem};
use crate::search::draw_report;
use crate::state::ShipSystem;
use crate::surface::gun_shots;
use crate::time::Pulse;
use crate::world::World;

/// One ship's fire in a bombardment, worked out before anything is written.
struct ShipFire {
    ship: ShipId,
    rounds_aboard: Fixed,
    shells: Fixed,
    weighed_shots: Fixed,
    expected: Fixed,
    hits: u32,
    landed: BaseHits,
    breakdown: Option<Breakdown>,
}

impl World {
    /// The share of the hits on `base` that land on men (decision 8 of
    /// 0092): the table's share for men when the base's side has men ashore
    /// in its hex and a hit on men removes some, else `None`, and then the
    /// share lands on nothing. Needs the air combat and the ground figures.
    pub(crate) fn base_hit_men_share(&self, base: BaseId) -> Option<Fixed> {
        let air = self.ref_data.air_combat.as_ref()?;
        let ground = self.ref_data.ground.as_ref()?;
        if ground.men_lost_per_base_hit <= Fixed::ZERO {
            return None;
        }
        let base = self.truth.bases.get(&base)?;
        let manned = self.truth.land_units.values().any(|unit| {
            unit.side == base.side && unit.hex == Some(base.hex) && unit.men_ashore > 0
        });
        manned.then_some(air.base_hit_men)
    }

    /// Where `hits` on `base` land (decision 7 of 0034, decision 8 of 0092):
    /// one draw below one million for each hit, below the share for aircraft,
    /// then below that and the share for supply, then below those and the
    /// share for men, else on nothing. Without men to hit, the share for men
    /// is nothing, so the draws land as they did before decision 0092.
    pub(crate) fn land_base_hits(
        &self,
        stream: &mut RngStream,
        hits: u32,
        base: BaseId,
        air: &AirCombatFigures,
    ) -> BaseHits {
        let on_aircraft_below = raw_u64(air.base_hit_aircraft);
        let on_supply_below = on_aircraft_below + raw_u64(air.base_hit_supply);
        let on_men_below = on_supply_below + self.base_hit_men_share(base).map_or(0, raw_u64);
        let mut landed = BaseHits::default();
        for _ in 0..hits {
            let draw = stream.below(Fixed::SCALE.unsigned_abs());
            if draw < on_aircraft_below {
                landed.aircraft += 1;
            } else if draw < on_supply_below {
                landed.supply += 1;
            } else if draw < on_men_below {
                landed.men += 1;
            }
        }
        landed
    }

    /// The men lost to `hits` on men at `base` (decision 9 of 0092): each
    /// removes `men_lost_per_base_hit` men, no more than the base's side has
    /// ashore in the hex, taken from its units there in proportion by the
    /// rule of decision 20 of 0066. Each unit's loss is an event citing
    /// `cause`; a unit with no man left anywhere is destroyed.
    pub(crate) fn apply_hits_on_men(
        &mut self,
        day: u32,
        pulse: Option<Pulse>,
        base: BaseId,
        hits: u32,
        cause: EventId,
    ) {
        if hits == 0 {
            return;
        }
        let Some(figures) = self.ref_data.ground else {
            return;
        };
        let (side, hex) = {
            let base = &self.truth.bases[&base];
            (base.side, base.hex)
        };
        let units: Vec<(LandUnitId, u32)> = self
            .truth
            .land_units
            .values()
            .filter(|unit| unit.side == side && unit.hex == Some(hex) && unit.men_ashore > 0)
            .map(|unit| (unit.id, unit.men_ashore))
            .collect();
        let men: Vec<u32> = units.iter().map(|(_, men)| *men).collect();
        let total: u32 = men.iter().sum();
        let per_hit = u32::try_from(figures.men_lost_per_base_hit.trunc()).unwrap_or(u32::MAX);
        let loss = hits.saturating_mul(per_hit).min(total);
        let parts = take_in_proportion(loss, &men);
        for ((unit_id, unit_men), part) in units.into_iter().zip(parts) {
            if part == 0 {
                continue;
            }
            let unit = self
                .truth
                .land_units
                .get_mut(&unit_id)
                .expect("the land unit exists");
            unit.men_ashore -= part;
            if unit.men_ashore == 0 {
                unit.hex = None;
            }
            let ashore = unit.men_ashore;
            let breakdown = self.config.record_breakdowns.then(|| Breakdown {
                factors: vec![
                    Factor {
                        name: "hits_on_men",
                        value: Fixed::from_int(i64::from(hits)),
                    },
                    Factor {
                        name: "men_lost_per_base_hit",
                        value: figures.men_lost_per_base_hit,
                    },
                    Factor {
                        name: "loss_in_hex",
                        value: Fixed::from_int(i64::from(loss)),
                    },
                    Factor {
                        name: "unit_men_ashore",
                        value: Fixed::from_int(i64::from(unit_men)),
                    },
                    Factor {
                        name: "men_ashore_in_hex",
                        value: Fixed::from_int(i64::from(total)),
                    },
                ],
                result: Fixed::from_int(i64::from(part)),
            });
            let lost = self.events.record(
                day,
                pulse,
                EventKind::MenLostToFire {
                    unit: unit_id,
                    base,
                    hex,
                    lost: part,
                    ashore,
                },
                vec![Cause::Event(cause)],
                breakdown,
            );
            self.destroy_if_no_men(day, pulse, unit_id, lost);
        }
    }

    /// The shore bombardment phase of the night pulse (decision 0092, 1).
    /// Does nothing without the bombardment figures and the air combat
    /// figures, whose table says where a hit lands.
    pub(crate) fn bombard_bases(&mut self, day: u32, pulse: Pulse) {
        let Some(figures) = self
            .ref_data
            .surface_combat
            .as_ref()
            .and_then(|surface| surface.bombardment)
        else {
            return;
        };
        if self.ref_data.air_combat.is_none() {
            return;
        }
        let forces: Vec<(TaskForceId, BaseId)> = self
            .truth
            .task_forces
            .values()
            .filter(|force| {
                force.mission == Mission::Bombardment
                    && force.yard_trip.is_none()
                    && !force.ships.is_empty()
            })
            .filter_map(|force| {
                let hex = force.position();
                self.truth
                    .bases
                    .values()
                    .find(|base| base.hex == hex && base.side != force.side && base.on_map)
                    .map(|base| (force.id, base.id))
            })
            .collect();
        for (force, base) in forces {
            self.bombard(day, pulse, force, base, &figures);
        }
    }

    /// One force's bombardment of one base (decisions 2 to 7 and 17).
    fn bombard(
        &mut self,
        day: u32,
        pulse: Pulse,
        force_id: TaskForceId,
        base: BaseId,
        figures: &BombardmentFigures,
    ) {
        let air = self
            .ref_data
            .air_combat
            .clone()
            .expect("checked by the caller");
        let surface = self
            .ref_data
            .surface_combat
            .clone()
            .expect("checked by the caller");
        let force = &self.truth.task_forces[&force_id];
        let (side, hex) = (force.side, force.position());
        let ships: Vec<ShipId> = force.ships.iter().copied().collect();
        // Why the force is there: the order that set its route and its
        // arrival, if it arrived.
        let mut causes: Vec<Cause> = Vec::new();
        causes.extend(force.route_order.map(Cause::Order));
        causes.extend(force.arrival_event.map(Cause::Event));

        // Each ship's fire, on the truth at the start (decisions 2, 3, 5).
        let reference = figures.reference_gun_damage;
        let mut fires: Vec<ShipFire> = Vec::new();
        let mut first_stream: Option<RngStream> = None;
        for ship_id in &ships {
            let ship = &self.truth.ships[ship_id];
            let class = &self.ref_data.ship_classes[&ship.class];
            let shells = gun_shots(ship, class, figures.minutes, &surface, Fixed::ONE);
            if shells <= Fixed::ZERO {
                continue;
            }
            let weighed_shots = shells.mul_div(class.gun_damage, reference);
            let chance = figures.hit_chance;
            let expected = weighed_shots * chance;
            let mut stream = self.rng_pair(
                day,
                Some(pulse),
                Subsystem::SurfaceCombat,
                Draw::Bombardment,
                EntityRef::Ship(*ship_id),
                EntityRef::Base(base),
            );
            let hits = roll_shots(&mut stream, weighed_shots, chance);
            let landed = self.land_base_hits(&mut stream, hits, base, &air);
            let breakdown = self.config.record_breakdowns.then(|| Breakdown {
                factors: vec![
                    Factor {
                        name: "rate",
                        value: class.gun_rounds_per_min,
                    },
                    Factor {
                        name: "guns",
                        value: Fixed::from_int(i64::from(class.main_guns)),
                    },
                    Factor {
                        name: "minutes",
                        value: Fixed::from_int(i64::from(figures.minutes)),
                    },
                    Factor {
                        name: "main_battery_factor",
                        value: surface
                            .system_factor(ShipSystem::MainBattery, ship.systems.main_battery),
                    },
                    Factor {
                        name: "rounds_aboard",
                        value: ship.gun_rounds,
                    },
                    Factor {
                        name: "shells",
                        value: shells,
                    },
                    Factor {
                        name: "gun_damage",
                        value: class.gun_damage,
                    },
                    Factor {
                        name: "reference_gun_damage",
                        value: reference,
                    },
                    Factor {
                        name: "weighed_shots",
                        value: weighed_shots,
                    },
                    Factor {
                        name: "chance",
                        value: chance,
                    },
                ],
                result: expected,
            });
            fires.push(ShipFire {
                ship: *ship_id,
                rounds_aboard: ship.gun_rounds,
                shells,
                weighed_shots,
                expected,
                hits,
                landed,
                breakdown,
            });
            if first_stream.is_none() {
                first_stream = Some(stream);
            }
        }
        let Some(mut first_stream) = first_stream else {
            // No ship had a shell to fire: nothing happens.
            return;
        };
        let first_ship = fires[0].ship;

        // The rounds are spent, and each ship's fire recorded.
        let mut fire_events: Vec<Cause> = Vec::new();
        for fire in &fires {
            let rounds_left = fire.rounds_aboard - fire.shells;
            self.truth
                .ships
                .get_mut(&fire.ship)
                .expect("the ship exists")
                .gun_rounds = rounds_left;
            let event = self.events.record(
                day,
                Some(pulse),
                EventKind::BombardmentFire {
                    ship: fire.ship,
                    base,
                    shells: fire.shells,
                    weighed_shots: fire.weighed_shots,
                    chance: figures.hit_chance,
                    expected: fire.expected,
                    hits: fire.hits,
                    hits_on_aircraft: fire.landed.aircraft,
                    hits_on_supply: fire.landed.supply,
                    hits_on_men: fire.landed.men,
                    rounds_left,
                },
                causes.clone(),
                fire.breakdown.clone(),
            );
            fire_events.push(Cause::Event(event));
        }

        // The force's fire, added.
        let mut total = BaseHits::default();
        let (mut shells, mut weighed_shots, mut hits) = (Fixed::ZERO, Fixed::ZERO, 0);
        for fire in &fires {
            total.aircraft += fire.landed.aircraft;
            total.supply += fire.landed.supply;
            total.men += fire.landed.men;
            shells += fire.shells;
            weighed_shots += fire.weighed_shots;
            hits += fire.hits;
        }
        let bombardment = self.events.record(
            day,
            Some(pulse),
            EventKind::ShoreBombardment {
                task_force: force_id,
                base,
                hex,
                shells,
                weighed_shots,
                hits,
                hits_on_aircraft: total.aircraft,
                hits_on_supply: total.supply,
                hits_on_men: total.men,
            },
            fire_events.clone(),
            None,
        );

        // The hits land (decision 4): every aircraft at the base is on the
        // ground at night, each group's damaged aircraft before its ready.
        let mut pool: Vec<(AirGroupId, Stock, u32)> = Vec::new();
        for group in self
            .truth
            .air_groups
            .values()
            .filter(|group| group.location == crate::state::AirGroupLocation::Base(base))
        {
            if group.damaged > 0 {
                pool.push((group.id, Stock::Damaged, group.damaged));
            }
            if group.ready > 0 {
                pool.push((group.id, Stock::Ready, group.ready));
            }
        }
        let destroyed = self.apply_base_hits(day, Some(pulse), base, total, bombardment, &mut pool);

        // The base's side sights the force (decision 7), on the first firing
        // ship's stream after its hits and where they landed.
        let base_side = self.truth.bases[&base].side;
        let search = self
            .ref_data
            .search
            .clone()
            .expect("World::new requires search figures with surface combat figures");
        let afloat: Vec<ShipId> = self.truth.task_forces[&force_id]
            .ships
            .iter()
            .copied()
            .collect();
        let report = draw_report(&mut first_stream, &search).read(
            &self.map,
            hex,
            u32::try_from(afloat.len()).expect("few ships"),
            self.most_important_kind_of(&afloat),
            &search,
            self.events.next_id(),
        );
        let ApplyOutcome::Contact(outcome) = self
            .beliefs
            .get_mut(&base_side)
            .expect("both sides have a belief state")
            .apply_report(&Report::Sighting(report.clone()), &search)
        else {
            unreachable!("a sighting report makes or refreshes a contact");
        };
        let received = self.events.record(
            day,
            Some(pulse),
            EventKind::ReportReceived {
                side: base_side,
                contact: outcome.contact,
                hex: report.estimated_hex,
                ship_count: report.estimated_ship_count,
                kind: report.estimated_kind,
                created: outcome.created,
            },
            vec![Cause::Event(bombardment)],
            None,
        );
        assert_eq!(
            received, report.received_event,
            "the report cites the event that records it"
        );

        self.claim_bombardment(
            day,
            pulse,
            side,
            hex,
            (first_ship, base),
            (weighed_shots, destroyed),
            figures,
            fire_events,
        );
    }

    /// The bombarding side's claim (decision 17): the aircraft really
    /// destroyed and false ones, rolled from the weighed shots less those
    /// aircraft at the false claim chance. Never below the truth, never
    /// corrected.
    #[allow(clippy::too_many_arguments)]
    fn claim_bombardment(
        &mut self,
        day: u32,
        pulse: Pulse,
        side: crate::state::Side,
        hex: Hex,
        (first_ship, base): (ShipId, BaseId),
        (weighed_shots, destroyed): (Fixed, u32),
        figures: &BombardmentFigures,
        causes: Vec<Cause>,
    ) {
        let search = self
            .ref_data
            .search
            .clone()
            .expect("World::new requires search figures with surface combat figures");
        let mut stream = self.rng_pair(
            day,
            Some(pulse),
            Subsystem::Intelligence,
            Draw::Claim,
            EntityRef::Ship(first_ship),
            EntityRef::Base(base),
        );
        let real = Fixed::from_int(i64::from(destroyed));
        let misses = (weighed_shots - real).max(Fixed::ZERO);
        let false_claims = roll_shots(&mut stream, misses, figures.false_claim_chance);
        let claimed_on_ground = destroyed + false_claims;
        let claim = BattleClaim {
            hex,
            day,
            pulse,
            claimed_sunk: 0,
            claimed_hits: 0,
            claimed_kind: None,
            claimed_shot_down: 0,
            claimed_on_ground,
            from_air: false,
            from_bombardment: true,
            received_event: self.events.next_id(),
        };
        let outcome = self
            .beliefs
            .get_mut(&side)
            .expect("both sides have a belief state")
            .apply_report(&Report::BattleClaim(claim.clone()), &search);
        assert_eq!(outcome, ApplyOutcome::Claim, "a claim is stored as a claim");
        let breakdown = self.config.record_breakdowns.then(|| Breakdown {
            factors: vec![
                Factor {
                    name: "real_on_ground",
                    value: real,
                },
                Factor {
                    name: "weighed_shots",
                    value: weighed_shots,
                },
                Factor {
                    name: "false_claim_chance",
                    value: figures.false_claim_chance,
                },
                Factor {
                    name: "false_on_ground",
                    value: Fixed::from_int(i64::from(false_claims)),
                },
            ],
            result: Fixed::from_int(i64::from(claimed_on_ground)),
        });
        let received = self.events.record(
            day,
            Some(pulse),
            EventKind::ClaimReceived {
                side,
                hex,
                claimed_sunk: 0,
                claimed_hits: 0,
                claimed_kind: None,
                claimed_shot_down: 0,
                claimed_on_ground,
                from_air: false,
                from_bombardment: true,
            },
            causes,
            breakdown,
        );
        assert_eq!(
            received, claim.received_event,
            "the claim cites the event that records it"
        );
    }
}
