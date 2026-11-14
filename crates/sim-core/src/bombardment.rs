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
use crate::cover::{after_cover, piece_of_part, unit_cover};
use crate::damage::{raw_u64, roll_shots};
use crate::events::{Breakdown, Cause, EventKind, Factor, FieldworksChange};
use crate::fixed::Fixed;
use crate::ground::take_in_proportion;
use crate::hex::Hex;
use crate::ids::{AirGroupId, BaseId, EntityRef, EventId, LandUnitId, ShipId, TaskForceId};
use crate::movement::HaltGround;
use crate::orders::Mission;
use crate::refdata::{AirCombatFigures, BombardmentFigures};
use crate::rng::{Draw, RngStream, Subsystem};
use crate::search::draw_report;
use crate::state::{ShipSystem, TaskForce};
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

/// The base `force` bombards in `hex` by the rule of the phase (decision
/// 0092, 1): with the bombardment mission, a ship afloat and not away at the
/// yard, the lowest-numbered base of the other side in `hex` that is on the
/// force's side's map.
///
/// `enemy_base_at` gives that base: the lowest-numbered base of the other
/// side on the map in a hex. Where a base is and which side holds it are the
/// map, which every side has (decision 0034, 2); nothing else of a base is
/// read here (decision 0100, 1).
pub(crate) fn bombardment_target_of(
    force: &TaskForce,
    hex: Hex,
    enemy_base_at: &dyn Fn(Hex) -> Option<BaseId>,
) -> Option<BaseId> {
    if force.mission != Mission::Bombardment || force.yard_trip.is_some() || force.ships.is_empty()
    {
        return None;
    }
    enemy_base_at(hex)
}

/// Whether `force` has a bombardment to make in `hex` (decision 0096, 2):
/// the phase would have it bombard a base there, the figures the phase needs
/// are loaded, and a ship of it can fire, that is, would fire a shell by the
/// gun rule of the phase. It reads the force, its ships, the figures and the
/// enemy bases on the map (`enemy_base_at`, see [`bombardment_target_of`]).
pub(crate) fn has_bombardment_work(
    ground: &HaltGround,
    force: &TaskForce,
    hex: Hex,
    enemy_base_at: &dyn Fn(Hex) -> Option<BaseId>,
) -> bool {
    let Some(surface) = ground.ref_data.surface_combat.as_ref() else {
        return false;
    };
    let Some(figures) = surface.bombardment else {
        return false;
    };
    if ground.ref_data.air_combat.is_none()
        || bombardment_target_of(force, hex, enemy_base_at).is_none()
    {
        return false;
    }
    force.ships.iter().any(|ship_id| {
        let ship = &ground.ships[ship_id];
        let class = &ground.ref_data.ship_classes[&ship.class];
        gun_shots(ship, class, figures.minutes, surface, Fixed::ONE) > Fixed::ZERO
    })
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

    /// The share of the hits on `base` that land on its airfield (decision 2
    /// of 0103): the table's share for the airfield when the base exists and
    /// has an airfield and the ground figures are loaded, else `None`, and
    /// then the share lands on nothing. Needs the air combat figures.
    pub(crate) fn base_hit_airfield_share(&self, base: BaseId) -> Option<Fixed> {
        let air = self.ref_data.air_combat.as_ref()?;
        self.ref_data.ground.as_ref()?;
        let base = self.truth.bases.get(&base)?;
        base.airfield.then_some(air.base_hit_airfield)
    }

    /// Where `hits` on `base` land (decision 7 of 0034, decision 8 of 0092,
    /// decision 2 of 0103): one draw below one million for each hit, below
    /// the share for aircraft, then below that and the share for supply, then
    /// below those and the share for men, then below those and the share for
    /// the airfield, else on nothing. Without men to hit, or an airfield, the
    /// share for it is nothing, so the draws land as they did before.
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
        let on_airfield_below =
            on_men_below + self.base_hit_airfield_share(base).map_or(0, raw_u64);
        let mut landed = BaseHits::default();
        for _ in 0..hits {
            let draw = stream.below(Fixed::SCALE.unsigned_abs());
            if draw < on_aircraft_below {
                landed.aircraft += 1;
            } else if draw < on_supply_below {
                landed.supply += 1;
            } else if draw < on_men_below {
                landed.men += 1;
            } else if draw < on_airfield_below {
                landed.airfield += 1;
            }
        }
        landed
    }

    /// The damage of `hits` on the airfield of `base` (decision 3 of 0103):
    /// each adds `airfield_damage_per_hit`, to 100 at most. Records
    /// `AirfieldDamaged` citing `cause`, with the damage after, even when it
    /// was 100 already. Nothing happens for no hits.
    pub(crate) fn apply_hits_on_airfield(
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
        let target = self.truth.bases.get_mut(&base).expect("the base exists");
        let before = target.airfield_damage;
        let per_hit = figures.airfield_damage_per_hit;
        target.airfield_damage =
            (before + per_hit * Fixed::from_int(i64::from(hits))).min(Fixed::from_int(100));
        let (after, side) = (target.airfield_damage, target.side);
        let breakdown = self.config.record_breakdowns.then(|| Breakdown {
            factors: vec![
                Factor {
                    name: "hits_on_airfield",
                    value: Fixed::from_int(i64::from(hits)),
                },
                Factor {
                    name: "airfield_damage_per_hit",
                    value: per_hit,
                },
                Factor {
                    name: "damage_before",
                    value: before,
                },
            ],
            result: after,
        });
        self.events.record(
            day,
            pulse,
            EventKind::AirfieldDamaged {
                base,
                side,
                hits,
                damage: after,
            },
            vec![Cause::Event(cause)],
            breakdown,
        );
    }

    /// The men lost to `hits` on men at `base` (decision 9 of 0092, and
    /// decisions 2, 10 and 12 of 0106): each removes `men_lost_per_base_hit`
    /// men, no more than the base's side has ashore in the hex, shared
    /// between its units there in proportion by the rule of decision 20 of
    /// 0066. Each unit's part is then taken hit by hit, spread evenly over
    /// the hits ([`piece_of_part`]), and each piece is cut by the cover the
    /// unit has at that hit; after each hit the cover is worn: the defences
    /// of the side's lowest-numbered base in the hex and the fieldworks of
    /// each of its units there lose `cover_wear_per_hit`, not below 0. The
    /// hits are counted for the day's strain of the side in the hex. Each
    /// unit's loss is an event citing `cause`; a unit with no man left
    /// anywhere is destroyed.
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

        // The day's hits on the side's men in the hex (decision 10 of 0106).
        let day_hits = self.truth.hits_on_men.entry((hex, side)).or_default();
        day_hits.hits = day_hits.hits.saturating_add(hits);
        if !day_hits.causes.contains(&cause) {
            day_hits.causes.push(cause);
        }

        // Each part taken hit by hit, behind the cover of the moment, and the
        // cover worn after each hit (decisions 2 and 12 of 0106).
        let worn_base: Option<BaseId> = self
            .truth
            .bases
            .values()
            .find(|at| at.side == side && at.hex == hex)
            .map(|at| at.id);
        let defences_before = worn_base.map(|id| self.truth.bases[&id].defences);
        let fieldworks_before: Vec<Fixed> = units
            .iter()
            .map(|(unit, _)| self.truth.land_units[unit].fieldworks)
            .collect();
        let wear = figures.cover_wear_per_hit;
        let mut lost = vec![0u32; units.len()];
        let mut first_cover: Vec<Option<Fixed>> = vec![None; units.len()];
        for k in 1..=hits {
            for (i, (unit_id, _)) in units.iter().enumerate() {
                let piece = piece_of_part(parts[i], k, hits);
                if piece == 0 {
                    continue;
                }
                let cover = unit_cover(&self.truth, &self.truth.land_units[unit_id]);
                first_cover[i].get_or_insert(cover);
                lost[i] += after_cover(piece, cover, figures.cover_from_fire);
            }
            if wear > Fixed::ZERO {
                if let Some(id) = worn_base {
                    let at = self.truth.bases.get_mut(&id).expect("the base exists");
                    at.defences = (at.defences - wear).max(Fixed::ZERO);
                }
                for (unit_id, _) in &units {
                    let unit = self
                        .truth
                        .land_units
                        .get_mut(unit_id)
                        .expect("the land unit exists");
                    unit.fieldworks = (unit.fieldworks - wear).max(Fixed::ZERO);
                }
            }
        }

        for (i, ((unit_id, unit_men), part)) in units.iter().copied().zip(parts).enumerate() {
            let lost = lost[i];
            if lost == 0 {
                continue;
            }
            let unit = self
                .truth
                .land_units
                .get_mut(&unit_id)
                .expect("the land unit exists");
            unit.men_ashore -= lost;
            if unit.men_ashore == 0 {
                unit.hex = None;
            }
            let ashore = unit.men_ashore;
            let covered = first_cover[i]
                .filter(|cover| figures.cover_from_fire > Fixed::ZERO && *cover > Fixed::ZERO);
            let breakdown = self.config.record_breakdowns.then(|| {
                let mut factors = vec![
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
                ];
                // Only where cover saves men (decision 2 of 0106).
                if let Some(cover) = covered {
                    factors.push(Factor {
                        name: "men_before_cover",
                        value: Fixed::from_int(i64::from(part)),
                    });
                    factors.push(Factor {
                        name: "cover",
                        value: cover,
                    });
                    factors.push(Factor {
                        name: "cover_from_fire",
                        value: figures.cover_from_fire,
                    });
                    if wear > Fixed::ZERO && hits > 1 {
                        factors.push(Factor {
                            name: "cover_wear_per_hit",
                            value: wear,
                        });
                    }
                }
                Breakdown {
                    factors,
                    result: Fixed::from_int(i64::from(lost)),
                }
            });
            let lost_event = self.events.record(
                day,
                pulse,
                EventKind::MenLostToFire {
                    unit: unit_id,
                    base,
                    hex,
                    lost,
                    ashore,
                },
                vec![Cause::Event(cause)],
                breakdown,
            );
            self.destroy_if_no_men(day, pulse, unit_id, lost_event);
        }

        // What the fire wore away (decision 12 of 0106), where it did.
        if let (Some(id), Some(before)) = (worn_base, defences_before) {
            let defences = self.truth.bases[&id].defences;
            if defences != before {
                let breakdown = self.config.record_breakdowns.then(|| Breakdown {
                    factors: vec![
                        Factor {
                            name: "defences_before",
                            value: before,
                        },
                        Factor {
                            name: "hits_on_men",
                            value: Fixed::from_int(i64::from(hits)),
                        },
                        Factor {
                            name: "cover_wear_per_hit",
                            value: wear,
                        },
                    ],
                    result: defences,
                });
                self.events.record(
                    day,
                    pulse,
                    EventKind::DefencesWorn {
                        base: id,
                        side,
                        hex,
                        hits,
                        defences,
                    },
                    vec![Cause::Event(cause)],
                    breakdown,
                );
            }
        }
        for ((unit_id, _), before) in units.iter().zip(fieldworks_before) {
            let fieldworks = self.truth.land_units[unit_id].fieldworks;
            if fieldworks == before {
                continue;
            }
            let breakdown = self.config.record_breakdowns.then(|| Breakdown {
                factors: vec![
                    Factor {
                        name: "fieldworks_before",
                        value: before,
                    },
                    Factor {
                        name: "hits_on_men",
                        value: Fixed::from_int(i64::from(hits)),
                    },
                    Factor {
                        name: "cover_wear_per_hit",
                        value: wear,
                    },
                ],
                result: fieldworks,
            });
            self.events.record(
                day,
                pulse,
                EventKind::FieldworksChanged {
                    unit: *unit_id,
                    change: FieldworksChange::Worn,
                    fieldworks,
                },
                vec![Cause::Event(cause)],
                breakdown,
            );
        }
    }

    /// The base `force` bombards in `hex` by the rule of the phase
    /// ([`bombardment_target_of`]), the enemy bases on its side's map read
    /// from the world.
    fn bombardment_target(&self, force: &TaskForce, hex: Hex) -> Option<BaseId> {
        bombardment_target_of(force, hex, &|at| {
            self.truth
                .bases
                .values()
                .find(|base| base.hex == at && base.side != force.side && base.on_map)
                .map(|base| base.id)
        })
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
            .filter_map(|force| {
                self.bombardment_target(force, force.position())
                    .map(|base| (force.id, base))
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
        // arrival, if it arrived, or its halt, if it halted (decision 0096).
        let mut causes: Vec<Cause> = Vec::new();
        causes.extend(force.route_order.map(Cause::Order));
        causes.extend(force.arrival_event.map(Cause::Event));
        causes.extend(force.halt.map(|halt| Cause::Event(halt.event)));

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
            let breakdown = self.config.record_breakdowns.then(|| {
                let mut factors = vec![
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
                ];
                // Named only where the airfield can be hit (decision 0103).
                if let Some(share) = self
                    .base_hit_airfield_share(base)
                    .filter(|share| *share > Fixed::ZERO)
                {
                    factors.push(Factor {
                        name: "hit_on_airfield_chance",
                        value: share,
                    });
                }
                Breakdown {
                    factors,
                    result: expected,
                }
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
            total.airfield += fire.landed.airfield;
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
        // A halted force has now bombarded, and may steam on (decision 0096,
        // 4).
        if let Some(halt) = self
            .truth
            .task_forces
            .get_mut(&force_id)
            .and_then(|force| force.halt.as_mut())
        {
            halt.bombarded = Some(bombardment);
        }

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ids::{AircraftTypeId, ShipClassId};
    use crate::map::{HexData, Map};
    use crate::refdata::{
        AircraftRole, AircraftType, AttackWeapon, GroundFigures, PortSize, RefData, SearchFigures,
        ShipClass, ShipKind, SurfaceCombatFigures,
    };
    use crate::state::{AirGroup, Base, LandUnit, Ship, ShipLocation, Side, TaskForce, TruthState};

    const AT: Hex = Hex::new(0, 0);
    const ELSEWHERE: Hex = Hex::new(4, 0);

    fn n(whole: i64) -> Fixed {
        Fixed::from_int(whole)
    }

    fn fx(text: &str) -> Fixed {
        text.parse().expect("a decimal")
    }

    /// A gun ship: guns, rounds a minute, rounds a gun and gun damage.
    fn class(id: u32, guns: u32, rate: i64, rounds_per_gun: u32, damage: i64) -> ShipClass {
        ShipClass {
            id: ShipClassId::new(id),
            name: format!("Test class {id}"),
            kind: ShipKind::Cruiser,
            cruise_speed_kn: n(16),
            max_speed_kn: n(32),
            fuel_capacity_t: n(1000),
            endurance_nm: n(8000),
            cargo_capacity_t: Fixed::ZERO,
            main_guns: guns,
            gun_rounds_per_min: n(rate),
            gun_rounds_per_gun: rounds_per_gun,
            gun_damage: n(damage),
            torpedo_tubes: 0,
            torpedo_tubes_one_side: 0,
            torpedo_damage: Fixed::ZERO,
            toughness: n(100),
            radar: false,
            aircraft_capacity: 0,
            anti_aircraft: 0,
            troop_capacity_men: 0,
        }
    }

    /// Class 1 is the cruiser of scenario 24 (8 guns at 3 a minute, gun
    /// damage 20), class 2 its destroyer (4 guns at 6 a minute, gun damage
    /// 5), class 3 a heavy gun ship (2 guns at 2 a minute, gun damage 60).
    fn classes() -> [ShipClass; 3] {
        [
            class(1, 8, 3, 100, 20),
            class(2, 4, 6, 150, 5),
            class(3, 2, 2, 100, 60),
        ]
    }

    /// Allied base 1 at AT, with an air group of 20 ready and 4 damaged
    /// fighters and 2000 t of supply; Japanese force 1 at AT on the
    /// bombardment mission, with one ship of each of `ship_classes` (ship
    /// numbers 1, 2, ... in the order given, each full of rounds); and,
    /// if `men` is not 0, Allied unit 1 of that many men ashore at AT.
    fn world(ship_classes: &[u32], men: u32) -> World {
        let mut map = Map::new();
        for q in -1..=9 {
            for r in -1..=1 {
                map.insert(Hex::new(q, r), HexData::open_sea());
            }
        }
        let mut ref_data = RefData::default();
        for class in classes() {
            ref_data.ship_classes.insert(class.id, class);
        }
        ref_data.aircraft_types.insert(
            AircraftTypeId::new(5),
            AircraftType {
                id: AircraftTypeId::new(5),
                name: "Test fighter".to_string(),
                role: AircraftRole::Fighter,
                search_range_nm: Fixed::ZERO,
                sweep_width_nm: Fixed::ZERO,
                strike_range_nm: n(250),
                attack_weapon: AttackWeapon::None,
                attack_damage: Fixed::ZERO,
                pilot_armour: false,
                self_sealing_tanks: false,
            },
        );
        ref_data.search = Some(SearchFigures::from_search_toml());
        ref_data.surface_combat = Some(SurfaceCombatFigures::from_surface_combat_toml());
        ref_data.air_combat = Some(AirCombatFigures::from_air_combat_toml());
        ref_data.ground = Some(GroundFigures::FIRST);
        ref_data.port_sizes.insert(
            0,
            PortSize {
                berths: 0,
                berth_rate_t_per_h: Fixed::ZERO,
                lighter_rate_t_per_h: n(20),
            },
        );

        let mut truth = TruthState::default();
        truth.bases.insert(
            BaseId::new(1),
            Base {
                supply_t: n(2000),
                ..Base::new(BaseId::new(1), "Base 1".to_string(), Side::Allied, AT)
            },
        );
        truth.air_groups.insert(
            AirGroupId::new(1),
            AirGroup {
                ready: 20,
                damaged: 4,
                strength: 26,
                ..AirGroup::new(
                    AirGroupId::new(1),
                    "Air group 1".to_string(),
                    Side::Allied,
                    crate::state::AirGroupLocation::Base(BaseId::new(1)),
                    AircraftTypeId::new(5),
                )
            },
        );
        if men > 0 {
            truth.land_units.insert(
                LandUnitId::new(1),
                LandUnit::new(
                    LandUnitId::new(1),
                    "Unit 1".to_string(),
                    Side::Allied,
                    men,
                    Some(AT),
                ),
            );
        }
        let mut force = TaskForce::at(
            TaskForceId::new(1),
            "Force 1".to_string(),
            Side::Japanese,
            AT,
            Mission::Bombardment,
        );
        for (index, class_id) in ship_classes.iter().enumerate() {
            let id = ShipId::new(u32::try_from(index).expect("a few ships") + 1);
            force.ships.insert(id);
            let class = &classes()[(*class_id - 1) as usize];
            truth.ships.insert(
                id,
                Ship {
                    gun_rounds: n(i64::from(class.main_guns * class.gun_rounds_per_gun)),
                    ..Ship::new(
                        id,
                        format!("Ship {index}"),
                        class.id,
                        Side::Japanese,
                        ShipLocation::TaskForce(force.id),
                    )
                },
            );
        }
        truth.task_forces.insert(force.id, force);
        World::new(7, ref_data, map, truth).expect("valid setup")
    }

    fn fires(world: &World) -> Vec<(ShipId, Fixed, Fixed, Fixed, Fixed, Fixed)> {
        world
            .events()
            .iter()
            .filter_map(|e| match &e.kind {
                EventKind::BombardmentFire {
                    ship,
                    shells,
                    weighed_shots,
                    chance,
                    expected,
                    rounds_left,
                    ..
                } => Some((
                    *ship,
                    *shells,
                    *weighed_shots,
                    *chance,
                    *expected,
                    *rounds_left,
                )),
                _ => None,
            })
            .collect()
    }

    #[test]
    fn a_shell_is_weighed_by_its_ships_gun_damage_over_the_reference() {
        let mut world = world(&[1, 2, 3], 1000);
        world.bombard_bases(1, Pulse::Night);
        let fired = fires(&world);
        // Ship 1, the cruiser: 3 x 8 x 30 = 720 shells of 800, gun damage 20
        // over the reference 20: 720 weighed shots.
        // Ship 2, the destroyer: 6 x 4 x 30 = 720 but 600 aboard, so 600
        // shells at 5/20: 150 weighed shots, at 0.05 expecting 7.5 hits.
        // Ship 3, the heavy: 2 x 2 x 30 = 120 shells at 60/20: 360 weighed
        // shots, which is more than its shells.
        assert_eq!(
            fired,
            vec![
                (ShipId::new(1), n(720), n(720), fx("0.05"), n(36), n(80)),
                (ShipId::new(2), n(600), n(150), fx("0.05"), fx("7.5"), n(0)),
                (ShipId::new(3), n(120), n(360), fx("0.05"), n(18), n(80)),
            ]
        );
        // The rounds are spent: before = after + shells, for every ship.
        for (ship, shells, _, _, _, left) in fired {
            let aboard = {
                let class = &classes()[(ship.raw() - 1) as usize];
                n(i64::from(class.main_guns * class.gun_rounds_per_gun))
            };
            assert_eq!(aboard, left + shells, "{ship:?}");
            assert_eq!(world.truth.ships[&ship].gun_rounds, left);
        }
        // A second night with the same ships: only the ships with rounds
        // left fire, and what they fire is what they have.
        let before = world.events().len();
        world.bombard_bases(2, Pulse::Night);
        let second: Vec<_> = world.events().iter().skip(before).cloned().collect();
        let second_fires: Vec<(ShipId, Fixed, Fixed)> = second
            .iter()
            .filter_map(|e| match &e.kind {
                EventKind::BombardmentFire {
                    ship,
                    shells,
                    weighed_shots,
                    ..
                } => Some((*ship, *shells, *weighed_shots)),
                _ => None,
            })
            .collect();
        assert_eq!(
            second_fires,
            vec![
                (ShipId::new(1), n(80), n(80)),
                (ShipId::new(3), n(80), n(240))
            ],
            "the destroyer has nothing left and does not fire"
        );
        for ship in [1, 2, 3] {
            assert_eq!(
                world.truth.ships[&ShipId::new(ship)].gun_rounds,
                Fixed::ZERO
            );
        }
        // A third night: nothing to fire, so nothing happens at all.
        let before = world.events().len();
        world.bombard_bases(3, Pulse::Night);
        assert_eq!(world.events().len(), before);
    }

    #[test]
    fn the_men_share_applies_only_with_men_of_the_bases_side_ashore_in_its_hex() {
        let base = BaseId::new(1);
        let unit = LandUnitId::new(1);
        // Men of the base's side in its hex: the table's share.
        let world_with_men = world(&[1], 500);
        assert_eq!(world_with_men.base_hit_men_share(base), Some(fx("0.2")));
        // No unit at all.
        assert_eq!(world(&[1], 0).base_hit_men_share(base), None);
        // An unknown base.
        assert_eq!(world_with_men.base_hit_men_share(BaseId::new(9)), None);

        // Enemy men in the hex are not the base's side's.
        let mut enemy = world(&[1], 500);
        enemy.truth.land_units.get_mut(&unit).unwrap().side = Side::Japanese;
        assert_eq!(enemy.base_hit_men_share(base), None);
        // Its own men in another hex.
        let mut elsewhere = world(&[1], 500);
        elsewhere.truth.land_units.get_mut(&unit).unwrap().hex = Some(ELSEWHERE);
        assert_eq!(elsewhere.base_hit_men_share(base), None);
        // Its own men still aboard ship, not ashore.
        let mut aboard = world(&[1], 500);
        aboard.truth.land_units.get_mut(&unit).unwrap().hex = None;
        assert_eq!(aboard.base_hit_men_share(base), None);
        // A unit in the hex with no man ashore.
        let mut empty = world(&[1], 500);
        empty.truth.land_units.get_mut(&unit).unwrap().men_ashore = 0;
        assert_eq!(empty.base_hit_men_share(base), None);
        // Men of both sides there: the base's side's men count.
        let mut both = world(&[1], 500);
        let mut other = LandUnit::new(
            LandUnitId::new(2),
            "Unit 2".to_string(),
            Side::Japanese,
            300,
            Some(AT),
        );
        other.men_ashore = 300;
        both.truth.land_units.insert(other.id, other);
        assert_eq!(both.base_hit_men_share(base), Some(fx("0.2")));
        both.truth.land_units.get_mut(&unit).unwrap().men_ashore = 0;
        assert_eq!(
            both.base_hit_men_share(base),
            None,
            "only the enemy's are left"
        );

        // Without the ground figures, or with a hit that removes nobody, or
        // without the air combat figures, the share lands on nothing.
        let mut no_ground = world(&[1], 500);
        no_ground.ref_data.ground = None;
        assert_eq!(no_ground.base_hit_men_share(base), None);
        let mut removes_nobody = world(&[1], 500);
        removes_nobody
            .ref_data
            .ground
            .as_mut()
            .unwrap()
            .men_lost_per_base_hit = Fixed::ZERO;
        assert_eq!(removes_nobody.base_hit_men_share(base), None);
        let mut no_air = world(&[1], 500);
        no_air.ref_data.air_combat = None;
        assert_eq!(no_air.base_hit_men_share(base), None);
    }

    #[test]
    fn hit_draws_land_the_same_with_and_without_men_when_no_hit_lands_in_the_men_band() {
        let with_men = world(&[1], 500);
        let without = world(&[1], 0);
        let air = AirCombatFigures::from_air_combat_toml();
        let base = BaseId::new(1);
        // The bands, in millionths: aircraft below 0.3, supply to 0.6, men
        // to 0.8, nothing above.
        let (men_from, men_to) = (600_000u64, 800_000u64);
        let (mut identical, mut in_band) = (0, 0);
        for hits in [1u32, 3, 8, 30] {
            for ship in 1..=400u32 {
                let stream = |world: &World| {
                    world.rng_pair(
                        1,
                        Some(Pulse::Night),
                        Subsystem::SurfaceCombat,
                        Draw::Bombardment,
                        EntityRef::Ship(ShipId::new(ship)),
                        EntityRef::Base(base),
                    )
                };
                // The same draws, read once on the side.
                let mut reading = stream(&without);
                let draws: Vec<u64> = (0..hits)
                    .map(|_| reading.below(Fixed::SCALE.unsigned_abs()))
                    .collect();
                let none_in_band = draws.iter().all(|d| *d < men_from || *d >= men_to);
                let a = without.land_base_hits(&mut stream(&without), hits, base, &air);
                let b = with_men.land_base_hits(&mut stream(&with_men), hits, base, &air);
                // Aircraft and supply are the same whatever the men: the
                // band for men sits after theirs.
                assert_eq!((a.aircraft, a.supply), (b.aircraft, b.supply));
                // With no man to hit nothing lands on men.
                assert_eq!(a.men, 0);
                let expected_men = u32::try_from(
                    draws
                        .iter()
                        .filter(|d| **d >= men_from && **d < men_to)
                        .count(),
                )
                .unwrap();
                assert_eq!(b.men, expected_men);
                if none_in_band {
                    assert_eq!(a, b);
                    identical += 1;
                } else {
                    in_band += 1;
                }
            }
        }
        assert!(
            identical > 100 && in_band > 100,
            "{identical} and {in_band}"
        );
    }

    #[test]
    fn the_airfield_share_applies_only_to_a_base_with_an_airfield_and_the_ground_figures() {
        let base = BaseId::new(1);
        let mut world = world(&[1], 0);
        world
            .ref_data
            .air_combat
            .as_mut()
            .unwrap()
            .base_hit_airfield = fx("0.2");
        assert_eq!(world.base_hit_airfield_share(base), Some(fx("0.2")));
        // An unknown base, a base with no airfield, no ground figures and
        // no air combat figures: nothing lands on the airfield.
        assert_eq!(world.base_hit_airfield_share(BaseId::new(9)), None);
        let mut none = world.clone();
        none.truth.bases.get_mut(&base).unwrap().airfield = false;
        assert_eq!(none.base_hit_airfield_share(base), None);
        let mut no_ground = world.clone();
        no_ground.ref_data.ground = None;
        assert_eq!(no_ground.base_hit_airfield_share(base), None);
        let mut no_air = world.clone();
        no_air.ref_data.air_combat = None;
        assert_eq!(no_air.base_hit_airfield_share(base), None);
    }

    #[test]
    fn the_airfield_band_comes_after_men_and_a_share_of_0_changes_no_draw() {
        let base = BaseId::new(1);
        let air = AirCombatFigures::from_air_combat_toml();
        let mut with_field = world(&[1], 500);
        with_field
            .ref_data
            .air_combat
            .as_mut()
            .unwrap()
            .base_hit_airfield = fx("0.2");
        let mut with_field_air = air.clone();
        with_field_air.base_hit_airfield = fx("0.2");
        let no_field = world(&[1], 500);
        // The bands, in millionths: aircraft below 0.3, supply to 0.6, men
        // to 0.8, the airfield to 1.0.
        let (men_to, field_to) = (800_000u64, 1_000_000u64);
        let mut on_field = 0;
        for hits in [1u32, 4, 20] {
            for ship in 1..=300u32 {
                let stream = |world: &World| {
                    world.rng_pair(
                        1,
                        Some(Pulse::Night),
                        Subsystem::SurfaceCombat,
                        Draw::Bombardment,
                        EntityRef::Ship(ShipId::new(ship)),
                        EntityRef::Base(base),
                    )
                };
                let mut reading = stream(&no_field);
                let draws: Vec<u64> = (0..hits)
                    .map(|_| reading.below(Fixed::SCALE.unsigned_abs()))
                    .collect();
                let a = no_field.land_base_hits(&mut stream(&no_field), hits, base, &air);
                let b = with_field.land_base_hits(
                    &mut stream(&with_field),
                    hits,
                    base,
                    &with_field_air,
                );
                // A share of 0: no hit lands on the airfield, the rest as
                // before.
                assert_eq!(a.airfield, 0);
                assert_eq!((a.aircraft, a.supply, a.men), (b.aircraft, b.supply, b.men));
                let expected = u32::try_from(
                    draws
                        .iter()
                        .filter(|d| **d >= men_to && **d < field_to)
                        .count(),
                )
                .unwrap();
                assert_eq!(b.airfield, expected);
                on_field += expected;
            }
        }
        assert!(on_field > 100, "{on_field}");
        // A base with no airfield never gets one.
        let mut none = with_field;
        none.truth.bases.get_mut(&base).unwrap().airfield = false;
        for ship in 1..=100u32 {
            let mut stream = none.rng_pair(
                1,
                Some(Pulse::Night),
                Subsystem::SurfaceCombat,
                Draw::Bombardment,
                EntityRef::Ship(ShipId::new(ship)),
                EntityRef::Base(base),
            );
            let landed = none.land_base_hits(&mut stream, 20, base, &with_field_air);
            assert_eq!(landed.airfield, 0);
        }
    }

    #[test]
    fn hits_on_the_airfield_add_damage_up_to_100_and_say_so_even_at_100() {
        let base = BaseId::new(1);
        let mut world = world(&[1], 0);
        let cause = world.events.next_id();
        let damage_of = |world: &World| world.truth.bases[&base].airfield_damage;
        let events = |world: &World| -> Vec<(u32, Fixed, Vec<Cause>)> {
            world
                .events()
                .iter()
                .filter_map(|e| match &e.kind {
                    EventKind::AirfieldDamaged {
                        hits, damage, side, ..
                    } => {
                        assert_eq!(*side, Side::Allied);
                        Some((*hits, *damage, e.causes.clone()))
                    }
                    _ => None,
                })
                .collect()
        };
        // No hit, no event.
        world.apply_hits_on_airfield(1, Some(Pulse::Night), base, 0, cause);
        assert!(events(&world).is_empty());
        // 3 hits at 2 each.
        world.apply_hits_on_airfield(1, Some(Pulse::Night), base, 3, cause);
        assert_eq!(damage_of(&world), n(6));
        // 60 hits take it to 100 and no more.
        world.apply_hits_on_airfield(1, Some(Pulse::Night), base, 60, cause);
        assert_eq!(damage_of(&world), n(100));
        // One more at 100: recorded, saying 100.
        world.apply_hits_on_airfield(1, Some(Pulse::Night), base, 1, cause);
        assert_eq!(damage_of(&world), n(100));
        assert_eq!(
            events(&world),
            vec![
                (3, n(6), vec![Cause::Event(cause)]),
                (60, n(100), vec![Cause::Event(cause)]),
                (1, n(100), vec![Cause::Event(cause)]),
            ]
        );
    }

    #[test]
    fn work_on_a_damaged_airfield_mends_it_to_0_at_most_and_records_the_mending() {
        use crate::orders::Job;
        let base = BaseId::new(1);
        let mut world = world(&[1], 1000);
        world.config.record_breakdowns = true;
        world.truth.bases.get_mut(&base).unwrap().airfield_damage = n(30);
        let unit = LandUnitId::new(1);
        world.truth.land_units.get_mut(&unit).unwrap().working = true;
        let figures = GroundFigures::FIRST;
        let mut on_airfields = std::collections::BTreeMap::new();
        // The default job of a damaged field is the airfield.
        assert_eq!(
            crate::works::default_job(&world.truth.bases[&base]),
            Job::Airfield
        );
        // 1000 of work, 50 a point: 20 points, 30 to 10.
        world.work_day(1, unit, None, &figures, &mut on_airfields);
        assert_eq!(world.truth.bases[&base].airfield_damage, n(10));
        // And again: 10 is all there is to mend, not 20.
        world.work_day(2, unit, None, &figures, &mut on_airfields);
        assert_eq!(world.truth.bases[&base].airfield_damage, Fixed::ZERO);
        // Whole: the default job is the port, and nothing is mended.
        assert_eq!(
            crate::works::default_job(&world.truth.bases[&base]),
            Job::Port
        );
        world.work_day(3, unit, Some(Job::Airfield), &figures, &mut on_airfields);
        assert_eq!(world.truth.bases[&base].airfield_damage, Fixed::ZERO);
        let mended: Vec<_> = world
            .events()
            .iter()
            .filter_map(|e| match &e.kind {
                EventKind::AirfieldMended { points, damage, .. } => {
                    Some((*points, *damage, e.causes.clone(), e.breakdown.clone()))
                }
                _ => None,
            })
            .collect();
        assert_eq!(mended.len(), 2, "none on the day it had nothing to mend");
        assert_eq!((mended[0].0, mended[0].1), (n(20), n(10)));
        assert_eq!((mended[1].0, mended[1].1), (n(10), Fixed::ZERO));
        // Each cites the work event of its day, and says how it was made.
        let worked: Vec<EventId> = world
            .events()
            .iter()
            .filter(|e| matches!(e.kind, EventKind::UnitWorked { .. }))
            .map(|e| e.id)
            .collect();
        assert_eq!(mended[0].2, vec![Cause::Event(worked[0])]);
        assert_eq!(mended[1].2, vec![Cause::Event(worked[1])]);
        let breakdown = mended[0].3.as_ref().expect("recorded");
        let names: Vec<&str> = breakdown.factors.iter().map(|f| f.name).collect();
        assert_eq!(
            names,
            ["work", "work_per_airfield_damage_point", "damage_before"]
        );
        assert_eq!(breakdown.result, n(20));
        // Nothing mends itself: a day with no work leaves the damage.
        world.truth.bases.get_mut(&base).unwrap().airfield_damage = n(40);
        world.truth.land_units.get_mut(&unit).unwrap().men_ashore = 0;
        world.work_day(4, unit, None, &figures, &mut on_airfields);
        assert_eq!(world.truth.bases[&base].airfield_damage, n(40));
    }

    #[test]
    fn hits_on_men_remove_ten_a_hit_from_the_units_in_proportion_and_no_more_than_are_there() {
        let base = BaseId::new(1);
        let mut world = world(&[1], 1000);
        // A second Allied unit of 3000 men in the hex.
        world.truth.land_units.insert(
            LandUnitId::new(2),
            LandUnit::new(
                LandUnitId::new(2),
                "Unit 2".to_string(),
                Side::Allied,
                3000,
                Some(AT),
            ),
        );
        let cause = world.events.next_id();
        // 6 hits remove 60 men, 15 from unit 1 and 45 from unit 2.
        world.apply_hits_on_men(1, Some(Pulse::Night), base, 6, cause);
        assert_eq!(world.truth.land_units[&LandUnitId::new(1)].men_ashore, 985);
        assert_eq!(world.truth.land_units[&LandUnitId::new(2)].men_ashore, 2955);
        // The losses cite the event that caused them.
        let lost: Vec<_> = world
            .events()
            .iter()
            .filter(|e| matches!(e.kind, EventKind::MenLostToFire { .. }))
            .collect();
        assert_eq!(lost.len(), 2);
        assert!(lost.iter().all(|e| e.causes == vec![Cause::Event(cause)]));
        // No hit, no loss.
        let before = world.events().len();
        world.apply_hits_on_men(1, Some(Pulse::Night), base, 0, cause);
        assert_eq!(world.events().len(), before);
        // Far more hits than men: everyone is lost, no more, and the units
        // are destroyed and leave the hex.
        world.apply_hits_on_men(1, Some(Pulse::Night), base, 1_000_000, cause);
        for id in [1, 2] {
            let unit = &world.truth.land_units[&LandUnitId::new(id)];
            assert_eq!((unit.men_ashore, unit.hex), (0, None));
        }
        let destroyed = world
            .events()
            .iter()
            .filter(|e| matches!(e.kind, EventKind::LandUnitDestroyed { .. }))
            .count();
        assert_eq!(destroyed, 2);
        let total: u32 = world
            .events()
            .iter()
            .filter_map(|e| match &e.kind {
                EventKind::MenLostToFire { lost, .. } => Some(*lost),
                _ => None,
            })
            .sum();
        assert_eq!(total, 4000, "the men lost are the men there were");
        // Without the ground figures a hit on men removes nobody.
        let mut no_ground = super::tests::world(&[1], 1000);
        no_ground.ref_data.ground = None;
        no_ground.apply_hits_on_men(1, Some(Pulse::Night), base, 50, cause);
        assert_eq!(
            no_ground.truth.land_units[&LandUnitId::new(1)].men_ashore,
            1000
        );
    }
}
