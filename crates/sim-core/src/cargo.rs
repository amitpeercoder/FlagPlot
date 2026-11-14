//! Cargo handling in the loading phase.
//!
//! The rule is `docs/decisions/0011-cargo-handling.md`. A task force that is
//! stopped at a base of its own side moves supply between its ships and the
//! base's stock, at the rates of the port. There is no randomness.
//!
//! Men are worked as cargo (decision 0066): each man moved takes
//! `handling_t_per_man` of a ship's tons of work in the pulse, men first and
//! in whole men, and what is left moves supply and then fuel by the rule as
//! it is. A load order with a land unit takes the unit's men ashore aboard
//! first, and may be worked off a shore where the side has no base (men
//! only). An unload lands men first, on a coast hex only; the first man
//! landed where the side has no base forms one, and a unit whose men would be
//! ashore in two hexes is divided first.

use std::collections::{BTreeMap, BTreeSet};

use crate::events::LoadingLoss;
use crate::events::{Breakdown, Cause, EventKind, Factor};
use crate::fixed::Fixed;
use crate::hex::Hex;
use crate::ids::{BaseId, EventId, LandUnitId, ShipId, TaskForceId};
use crate::map::HexKind;
use crate::orders::{Activity, Cargo, Job};
use crate::preparation::landing_rate_factor;
use crate::refdata::PortSize;
use crate::state::{Base, CargoOrder, LandUnit, Side, TaskForce, TaskForceStatus, Troops};
use crate::time::{pulse_start, Pulse};
use crate::world::World;

/// What a landing multiplies a ship's rate by, for a breakdown (decision 5 of
/// 0095): the figures behind the factor. Held only where the factor can
/// differ from 1, so a breakdown with the share at 1 is as it was.
#[derive(Clone, Copy)]
struct LandingRate {
    share: Fixed,
    loading: Fixed,
    factor: Fixed,
}

/// A task force that works cargo in this pulse.
struct Participant {
    task_force: TaskForceId,
    side: Side,
    hex: Hex,
    /// The lowest-numbered base of the force's side in its hex, if any. A
    /// load with a land unit and an unload with men aboard on a coast hex
    /// are worked with none (decision 0066, 6 and 11).
    base: Option<BaseId>,
    /// Whether men can be landed here: a coast hex (decision 0066, 10).
    coast: bool,
    cargo: CargoOrder,
    /// The hour the task force starts working: the later of the pulse start
    /// and its arrival.
    work_start: Fixed,
    hours: Fixed,
    ships: Vec<ShipId>,
    port: PortSize,
    /// The force's arrival, or its halt (decision 0096): a cause of its
    /// cargo events.
    arrival_event: Option<EventId>,
}

impl World {
    /// The tons of work one man takes, if the ground figures are loaded.
    fn handling_t_per_man(&self) -> Option<Fixed> {
        self.ref_data
            .ground
            .as_ref()
            .map(|figures| figures.handling_t_per_man)
    }

    /// Whole men that `tons` of work can move: the tons over the weight of a
    /// man, cut (decision 0066, "Whole-number arithmetic").
    fn men_by_time(tons: Fixed, per_man: Fixed) -> u32 {
        u32::try_from((tons / per_man).trunc().max(0)).unwrap_or(u32::MAX)
    }

    /// Room for men of `unit` aboard a ship: its troop capacity less the men
    /// aboard, or none if men of another unit are aboard (decision 0066, 8).
    fn berths_for(&self, ship: ShipId, unit: LandUnitId) -> u32 {
        let ship = &self.truth.ships[&ship];
        let capacity = self.ref_data.ship_classes[&ship.class].troop_capacity_men;
        match ship.troops {
            Some(troops) if troops.unit != unit => 0,
            Some(troops) => capacity.saturating_sub(troops.men),
            None => capacity,
        }
    }

    /// The men of `unit` ashore in `hex`: 0 if they are elsewhere or it
    /// has none.
    fn men_ashore_in(&self, unit: LandUnitId, hex: Hex) -> u32 {
        self.truth
            .land_units
            .get(&unit)
            .filter(|land_unit| land_unit.hex == Some(hex))
            .map_or(0, |land_unit| land_unit.men_ashore)
    }

    /// Whether `task_force`'s cargo order can work in `hex` by the cargo
    /// rule, and if so the lowest-numbered base of its side there, if any,
    /// and whether the hex is a coast hex. A base of its side is needed,
    /// except for a load with a land unit, which loads men off a bare shore,
    /// and an unload with men aboard on a coast hex, which lands them and
    /// forms a base there (decision 0066, 6 and 11).
    fn cargo_place(
        &self,
        task_force: &TaskForce,
        cargo: CargoOrder,
        hex: Hex,
    ) -> Option<(Option<BaseId>, bool)> {
        let men_rules = self.ref_data.ground.is_some();
        let base = self
            .truth
            .bases
            .values()
            .find(|base| base.side == task_force.side && base.hex == hex)
            .map(|base| base.id);
        let coast = self
            .map
            .get(hex)
            .is_some_and(|data| data.kind == HexKind::Coast);
        let works_without_base = men_rules
            && if cargo.action.is_load() {
                // A load with a unit loads men off a bare shore.
                cargo.unit.is_some()
            } else {
                // An unload with men aboard lands them on a coast hex
                // and forms a base there.
                coast
                    && task_force
                        .ships
                        .iter()
                        .any(|ship| self.truth.ships[ship].troops.is_some())
            };
        if base.is_none() && !works_without_base {
            return None;
        }
        Some((base, coast))
    }

    /// Whether `task_force` has cargo to work in `hex` (decision 0096, 2):
    /// a cargo order that can work there by the cargo rule, and something
    /// of it not yet done by the test that completes the order.
    pub(crate) fn has_cargo_work(&self, task_force: &TaskForce, hex: Hex) -> bool {
        let Some(cargo) = task_force.cargo_order else {
            return false;
        };
        let Some((base, coast)) = self.cargo_place(task_force, cargo, hex) else {
            return false;
        };
        let landing_here = !cargo.action.is_load() && coast && self.handling_t_per_man().is_some();
        let ships: Vec<ShipId> = task_force.ships.iter().copied().collect();
        !self.nothing_remains_of(cargo, &ships, hex, base, landing_here)
    }

    /// Moves cargo for every task force stopped at a base of its own side,
    /// and men for every force with a land unit to load or men to land
    /// (decision 0066).
    pub(crate) fn handle_cargo(&mut self, day: u32, pulse: Pulse) {
        let start = pulse_start(day, pulse);
        let end = start + Fixed::from_int(pulse.hours());
        let men_rules = self.ref_data.ground.is_some();

        let mut participants: Vec<Participant> = Vec::new();
        for task_force in self.truth.task_forces.values() {
            let Some(cargo) = task_force.cargo_order else {
                continue;
            };
            // Only a task force that has stopped on purpose works cargo:
            // idle, arrived, or halted at a marked waypoint (decision 0096,
            // 3). One that ran dry may be miles from any base, whatever hex
            // it is nearest to.
            if !matches!(
                task_force.status,
                TaskForceStatus::Idle | TaskForceStatus::Arrived | TaskForceStatus::Halted
            ) {
                continue;
            }
            // Cargo asks what refuelling asks: the force is exactly at the
            // centre of the base's hex. One idle part-way along a leg works
            // no cargo, whatever hex it is nearest to.
            if task_force.toward_hex.is_some() {
                continue;
            }
            let hex = task_force.from_hex;
            let Some((base, coast)) = self.cargo_place(task_force, cargo, hex) else {
                continue;
            };
            // From its arrival, or from its halt (decision 0096, 3).
            let stopped_at = task_force
                .halt
                .map(|halt| halt.at_hours)
                .or(task_force.arrived_at_hours);
            let work_start = match stopped_at {
                Some(stopped) => start.max(stopped),
                None => start,
            };
            let hours = end - work_start;
            if hours <= Fixed::ZERO {
                continue;
            }
            let size = self.map.get(hex).map_or(0, |data| data.port_size);
            participants.push(Participant {
                task_force: task_force.id,
                side: task_force.side,
                hex,
                base,
                coast,
                cargo,
                work_start,
                hours,
                ships: task_force.ships.iter().copied().collect(),
                port: self.ref_data.port_sizes[&size],
                arrival_event: task_force
                    .halt
                    .map(|halt| halt.event)
                    .or(task_force.arrival_event),
            });
        }

        // Berths are given out before any cargo moves, for each side's
        // landing place in a hex: the lowest-numbered base of the side
        // there, or the one a landing will form.
        let mut berths_used: BTreeMap<(Side, Hex), u32> = BTreeMap::new();
        let mut berthed: Vec<ShipId> = Vec::new();
        for participant in &participants {
            // The stock a load draws on: supply or fuel (decision 0046).
            let stock = participant.base.map_or(Fixed::ZERO, |id| {
                let base = &self.truth.bases[&id];
                match participant.cargo.action.loads() {
                    Some(Cargo::Fuel) => base.fuel_t,
                    _ => base.supply_t,
                }
            });
            let unit_ashore = match participant.cargo.unit {
                Some(unit) if men_rules => self.men_ashore_in(unit, participant.hex),
                _ => 0,
            };
            for ship_id in &participant.ships {
                let used = berths_used
                    .entry((participant.side, participant.hex))
                    .or_insert(0);
                if *used >= participant.port.berths {
                    break;
                }
                let ship = &self.truth.ships[ship_id];
                let capacity = self.ref_data.ship_classes[&ship.class].cargo_capacity_t;
                let has_cargo_to_move = if participant.cargo.action.is_load() {
                    // A ship has cargo to move if it has men to move
                    // (decision 0066, 7).
                    let men = participant
                        .cargo
                        .unit
                        .is_some_and(|unit| unit_ashore > 0 && self.berths_for(*ship_id, unit) > 0);
                    let cargo = participant.cargo.action.loads().is_some()
                        && participant.base.is_some()
                        && capacity - ship.supply_t - ship.cargo_fuel_t > Fixed::ZERO
                        && stock > Fixed::ZERO;
                    men || cargo
                } else {
                    let men = men_rules && participant.coast && ship.troops.is_some();
                    men || ship.supply_t > Fixed::ZERO || ship.cargo_fuel_t > Fixed::ZERO
                };
                if has_cargo_to_move {
                    *used += 1;
                    berthed.push(*ship_id);
                }
            }
        }

        for participant in &participants {
            self.work_cargo(day, pulse, participant, &berthed);
        }
    }

    /// Before a force lands men, in unit order for each unit with men to
    /// land this pulse: a unit with men ashore in another hex is divided
    /// (decision 0066, 14), and its men aboard the force's ships become a new
    /// unit. Then, if the side has no base in the hex, the first man landed
    /// forms one (decision 11). Returns the events, each unit's division
    /// event by its new number, and the base to land into.
    #[allow(clippy::too_many_arguments)]
    fn prepare_landing(
        &mut self,
        day: u32,
        pulse: Pulse,
        p: &Participant,
        landing: &BTreeSet<LandUnitId>,
        causes: &[Cause],
    ) -> (BTreeMap<LandUnitId, EventId>, Option<EventId>, BaseId) {
        let mut divided: BTreeMap<LandUnitId, EventId> = BTreeMap::new();
        for unit in landing {
            let land_unit = &self.truth.land_units[unit];
            if land_unit.men_ashore == 0 || land_unit.hex == Some(p.hex) {
                continue;
            }
            let new_id = LandUnitId::new(
                self.truth
                    .land_units
                    .keys()
                    .next_back()
                    .map_or(1, |id| id.raw() + 1),
            );
            let name = format!("{}, detachment {}", land_unit.name, new_id.raw());
            let side = land_unit.side;
            let mut men = 0;
            for ship_id in &p.ships {
                let ship = self.truth.ships.get_mut(ship_id).expect("ship exists");
                if let Some(troops) = ship.troops.as_mut() {
                    if troops.unit == *unit {
                        troops.unit = new_id;
                        men += troops.men;
                    }
                }
            }
            // A unit made by division takes its parent's experience and
            // skills and nothing else of its parent's: no rehearsal, no days
            // counted, no activity, no roster (decision 6 of 0097, decision
            // 7 of 0098). A part of a working unit is a working unit, and
            // works at the default job (decision 1 of 0099, As built).
            let (experience, skills, working) =
                (land_unit.experience, land_unit.skills, land_unit.working);
            self.truth.land_units.insert(
                new_id,
                LandUnit {
                    experience,
                    skills,
                    working,
                    activity: if working {
                        Activity::Work(None)
                    } else {
                        Activity::Ready
                    },
                    ..LandUnit::new(new_id, name, side, 0, None)
                },
            );
            let event = self.events.record(
                day,
                Some(pulse),
                EventKind::UnitDivided {
                    from: *unit,
                    unit: new_id,
                    task_force: p.task_force,
                    men,
                },
                causes.to_vec(),
                None,
            );
            divided.insert(new_id, event);
        }

        // A hex that holds a base of the side already, formed or not, forms
        // none (decision 0066, 11). The base is looked for again here, not
        // taken from when the participants were built, so that a second
        // force of the side landing in the same hex in the same pulse finds
        // the base the first one formed: the lowest-numbered of the side
        // there, as `cargo_place` finds it.
        let existing = p.base.or_else(|| {
            self.truth
                .bases
                .values()
                .find(|base| base.side == p.side && base.hex == p.hex)
                .map(|base| base.id)
        });
        if let Some(base) = existing {
            return (divided, None, base);
        }
        let id = BaseId::new(
            self.truth
                .bases
                .keys()
                .next_back()
                .map_or(1, |id| id.raw() + 1),
        );
        self.truth.bases.insert(
            id,
            // A base formed by a landing has no airfield (decision 5 of 0099).
            Base {
                on_map: false,
                airfield: false,
                ..Base::new(id, format!("Landing at {}", p.hex), p.side, p.hex)
            },
        );
        let formed = self.events.record(
            day,
            Some(pulse),
            EventKind::BaseFormed {
                base: id,
                side: p.side,
                hex: p.hex,
                task_force: p.task_force,
            },
            causes.to_vec(),
            None,
        );
        (divided, Some(formed), id)
    }

    fn work_cargo(&mut self, day: u32, pulse: Pulse, p: &Participant, berthed: &[ShipId]) {
        let mut causes = vec![Cause::Order(p.cargo.order)];
        if let Some(arrival) = p.arrival_event {
            causes.push(Cause::Event(arrival));
        }
        let mut cargo_events: Vec<Cause> = Vec::new();
        let mut longest = Fixed::ZERO;
        let mut moved_any = false;
        let loading = p.cargo.action.is_load();
        let per_man = self.handling_t_per_man();
        // While a force lands men, from the first man landed until its
        // unloading there is complete, every ship's rate, for men, supply and
        // fuel, is multiplied by the factor of its loading (decision 5 of
        // 0095). A force with no men aboard and no landing begun here works
        // as it always has.
        let landing_here = !loading && p.coast && per_man.is_some();
        let force_now = &self.truth.task_forces[&p.task_force];
        let lands_men = landing_here
            && (force_now.landing_at == Some(p.hex)
                || p.ships
                    .iter()
                    .any(|ship| self.truth.ships[ship].troops.is_some()));
        let landing_rate = match (lands_men, self.ref_data.ground) {
            (true, Some(figures)) => {
                let (factor, loading_counted) =
                    landing_rate_factor(&figures, force_now.preparation.as_ref(), p.hex);
                Some(LandingRate {
                    share: figures.unprepared_landing_share,
                    loading: loading_counted,
                    factor,
                })
            }
            _ => None,
        };
        let factor = landing_rate.map_or(Fixed::ONE, |landing| landing.factor);
        // The breakdown names the figures only where the share is below 1.
        let shown = landing_rate.filter(|landing| landing.share < Fixed::ONE);
        // Work on the base's port the day before speeds every ship's rate,
        // up to twice (decision 4 of 0099). With none the factor is 1.
        let port_work = match (p.base, self.ref_data.ground) {
            (Some(base), Some(figures)) if day > 1 => {
                let work = self.truth.bases[&base].work_of_day(day - 1, Job::Port);
                (work > Fixed::ZERO).then(|| (work, crate::works::port_factor(work, &figures)))
            }
            _ => None,
        };
        let port_factor = port_work.map_or(Fixed::ONE, |(_, factor)| factor);
        if let (Some((_, _)), Some(base)) = (port_work, p.base) {
            causes.extend(self.events_of_day_where(day - 1, |kind| {
                matches!(kind, EventKind::UnitWorked { base: at, job: Job::Port, .. } if *at == base)
            }));
        }
        let port_rate_of = |ship_id: &ShipId| {
            let own = if berthed.contains(ship_id) {
                p.port.berth_rate_t_per_h
            } else {
                p.port.lighter_rate_t_per_h
            };
            own * port_factor
        };
        let rate_of = |ship_id: &ShipId| port_rate_of(ship_id) * factor;
        let mut men_landed = false;

        // An unload lands men first, on a coast hex only (decision 0066, 9
        // and 10). Before the first man lands, units are divided and a base
        // is formed where needed.
        let mut base = p.base;
        let mut divided: BTreeMap<LandUnitId, EventId> = BTreeMap::new();
        if let (true, Some(per_man)) = (landing_here, per_man) {
            let landing: BTreeSet<LandUnitId> = p
                .ships
                .iter()
                .filter_map(|ship_id| {
                    let troops = self.truth.ships[ship_id].troops?;
                    let by_time = Self::men_by_time(rate_of(ship_id) * p.hours, per_man);
                    (by_time.min(troops.men) > 0).then_some(troops.unit)
                })
                .collect();
            if !landing.is_empty() {
                let (units, formed, landing_base) =
                    self.prepare_landing(day, pulse, p, &landing, &causes);
                divided = units;
                base = Some(landing_base);
                if let Some(formed) = formed {
                    causes.push(Cause::Event(formed));
                }
            }
        }

        for ship_id in &p.ships {
            let on_berth = berthed.contains(ship_id);
            let rate = rate_of(ship_id);
            let capacity_by_time = rate * p.hours;
            let mut work_t = capacity_by_time;
            let mut ship_total = Fixed::ZERO;

            // Men first (decision 0066, 7 to 9).
            if let Some(per_man) = per_man {
                if let Some(moved) = self.move_men(
                    day,
                    pulse,
                    p,
                    *ship_id,
                    on_berth,
                    (rate, port_rate_of(ship_id), shown, port_work),
                    per_man,
                    &causes,
                    &divided,
                ) {
                    let (men_t, event) = moved;
                    work_t -= men_t;
                    ship_total += men_t;
                    cargo_events.push(Cause::Event(event));
                    moved_any = true;
                    men_landed |= !loading;
                }
            }
            let men_t = capacity_by_time - work_t;

            // Then cargo, by the rule as it is, with the tons left. With no
            // base there is no stock to load from or land into.
            let Some(base_id) = base else {
                if ship_total > Fixed::ZERO {
                    longest = longest.max(ship_total / rate);
                }
                continue;
            };
            let ship = &self.truth.ships[ship_id];
            let capacity_t = self.ref_data.ship_classes[&ship.class].cargo_capacity_t;
            let supply_before = ship.supply_t;
            let fuel_before = ship.cargo_fuel_t;
            let room = capacity_t - supply_before - fuel_before;

            // What moves this pulse for this ship, in order: one entry per
            // cargo. A load moves one cargo, or none. An unload moves supply
            // and then fuel, within the tons the berth or lighter allows
            // (decisions 3 and 4 of 0046).
            let mut moves: Vec<(Cargo, Fixed)> = Vec::new();
            if loading {
                match p.cargo.action.loads() {
                    Some(Cargo::Supply) => {
                        let stock = self.truth.bases[&base_id].supply_t;
                        moves.push((Cargo::Supply, work_t.min(room).min(stock)));
                    }
                    Some(Cargo::Fuel) => {
                        let stock = self.truth.bases[&base_id].fuel_t;
                        moves.push((Cargo::Fuel, work_t.min(room).min(stock)));
                    }
                    None => {}
                }
            } else {
                let supply_moved = work_t.min(supply_before);
                let fuel_moved = (work_t - supply_moved).min(fuel_before);
                moves.push((Cargo::Supply, supply_moved));
                moves.push((Cargo::Fuel, fuel_moved));
            }

            let mut supply_moved = Fixed::ZERO;
            for (cargo, moved) in moves {
                if moved <= Fixed::ZERO {
                    continue;
                }
                let ship_record = self.truth.ships.get_mut(ship_id).expect("ship exists");
                let base_record = self.truth.bases.get_mut(&base_id).expect("base exists");
                let (aboard_before, stock_before) = match cargo {
                    Cargo::Supply => (supply_before, base_record.supply_t),
                    Cargo::Fuel => (fuel_before, base_record.fuel_t),
                };
                let (aboard_after, stock_after) = if loading {
                    (aboard_before + moved, stock_before - moved)
                } else {
                    (aboard_before - moved, stock_before + moved)
                };
                match cargo {
                    Cargo::Supply => {
                        ship_record.supply_t = aboard_after;
                        base_record.supply_t = stock_after;
                        supply_moved = moved;
                    }
                    Cargo::Fuel => {
                        ship_record.cargo_fuel_t = aboard_after;
                        base_record.fuel_t = stock_after;
                    }
                }

                let breakdown = self.config.record_breakdowns.then(|| {
                    let mut factors = vec![
                        Factor {
                            name: "hours_worked",
                            value: p.hours,
                        },
                        Factor {
                            name: "on_berth",
                            value: if on_berth { Fixed::ONE } else { Fixed::ZERO },
                        },
                        Factor {
                            name: "rate_t_per_h",
                            value: rate,
                        },
                        Factor {
                            name: "capacity_by_time_t",
                            value: capacity_by_time,
                        },
                    ];
                    factors.extend(landing_factors(shown, port_rate_of(ship_id)));
                    factors.extend(port_work_factors(port_work));
                    // The tons the ship's men took first (decision 0066, 7).
                    // Only where men moved, so a breakdown with none is as
                    // it was.
                    if men_t > Fixed::ZERO {
                        factors.push(Factor {
                            name: "men_t",
                            value: men_t,
                        });
                    }
                    if loading {
                        factors.push(Factor {
                            name: "room_t",
                            value: room,
                        });
                        factors.push(Factor {
                            name: "base_stock_t",
                            value: stock_before,
                        });
                    } else if cargo == Cargo::Supply {
                        factors.push(Factor {
                            name: "supply_aboard_t",
                            value: aboard_before,
                        });
                    } else {
                        factors.push(Factor {
                            name: "supply_unloaded_t",
                            value: supply_moved,
                        });
                        factors.push(Factor {
                            name: "fuel_cargo_aboard_t",
                            value: aboard_before,
                        });
                    }
                    Breakdown {
                        factors,
                        result: moved,
                    }
                });
                let kind = if loading {
                    EventKind::CargoLoaded {
                        ship: *ship_id,
                        task_force: p.task_force,
                        base: base_id,
                        cargo,
                        loaded_t: moved,
                        aboard_t: aboard_after,
                        base_stock_t: stock_after,
                    }
                } else {
                    EventKind::CargoUnloaded {
                        ship: *ship_id,
                        task_force: p.task_force,
                        base: base_id,
                        cargo,
                        unloaded_t: moved,
                        aboard_t: aboard_after,
                        base_stock_t: stock_after,
                    }
                };
                let event = self
                    .events
                    .record(day, Some(pulse), kind, causes.clone(), breakdown);
                cargo_events.push(Cause::Event(event));
                ship_total += moved;
                moved_any = true;
            }
            if ship_total > Fixed::ZERO {
                longest = longest.max(ship_total / rate);
            }
        }

        // The force has begun landing here (decision 5 of 0095).
        if men_landed {
            self.truth
                .task_forces
                .get_mut(&p.task_force)
                .expect("the force exists")
                .landing_at = Some(p.hex);
        }
        if !moved_any {
            return;
        }
        if !self.nothing_remains(p, base, landing_here) {
            return;
        }
        let elapsed_hours = p.work_start + longest;
        let kind = match (loading, p.cargo.unit, base) {
            (true, Some(unit), _) => EventKind::UnitLoadingComplete {
                task_force: p.task_force,
                unit,
                base,
                cargo: p.cargo.action.loads(),
                elapsed_hours,
            },
            (true, None, Some(base)) => EventKind::LoadingComplete {
                task_force: p.task_force,
                base,
                cargo: p
                    .cargo
                    .action
                    .loads()
                    .expect("a load with no unit names a cargo"),
                elapsed_hours,
            },
            (false, _, Some(base)) => EventKind::UnloadingComplete {
                task_force: p.task_force,
                base,
                elapsed_hours,
            },
            // Nothing moves where there is no base and no unit.
            (_, _, None) => return,
        };
        let breakdown = self.config.record_breakdowns.then(|| Breakdown {
            factors: vec![
                Factor {
                    name: "work_start_h",
                    value: p.work_start,
                },
                Factor {
                    name: "longest_ship_hours",
                    value: longest,
                },
            ],
            result: elapsed_hours,
        });
        let completed = self
            .events
            .record(day, Some(pulse), kind, cargo_events, breakdown);
        // A halted force's cargo work is done, and it may steam on (decision
        // 0096, 4).
        if let Some(halt) = self
            .truth
            .task_forces
            .get_mut(&p.task_force)
            .and_then(|force| force.halt.as_mut())
        {
            halt.cargo_completed = Some(completed);
        }
        // The landing is complete: the force has no landing begun, and if its
        // loading was for this hex it is spent and the order done (decision 7
        // of 0095).
        if lands_men && !loading {
            let force = self
                .truth
                .task_forces
                .get_mut(&p.task_force)
                .expect("the force exists");
            force.landing_at = None;
            if let Some(preparation) = force.preparation.take_if(|prep| prep.hex == p.hex) {
                self.events.record(
                    day,
                    Some(pulse),
                    EventKind::LoadingLost {
                        task_force: p.task_force,
                        hex: p.hex,
                        loading_before: preparation.loading,
                        loading: Fixed::ZERO,
                        reason: LoadingLoss::Spent,
                    },
                    vec![Cause::Event(completed)],
                    None,
                );
            }
        }
    }

    /// Whether the order is done. An unload: nothing aboard any ship, men
    /// counted only where they can be landed. A load: nothing remains of the
    /// named cargo by the rule as it is, and, with a unit, no man of it is
    /// ashore in the hex or no ship has a berth for one ("Settled before the
    /// build").
    fn nothing_remains(&self, p: &Participant, base: Option<BaseId>, landing_here: bool) -> bool {
        self.nothing_remains_of(p.cargo, &p.ships, p.hex, base, landing_here)
    }

    /// [`World::nothing_remains`] for `cargo` worked by `ships` in `hex`.
    fn nothing_remains_of(
        &self,
        cargo: CargoOrder,
        ships: &[ShipId],
        hex: Hex,
        base: Option<BaseId>,
        landing_here: bool,
    ) -> bool {
        if !cargo.action.is_load() {
            return ships.iter().all(|id| {
                let ship = &self.truth.ships[id];
                ship.supply_t.is_zero()
                    && ship.cargo_fuel_t.is_zero()
                    && (ship.troops.is_none() || !landing_here)
            });
        }
        let men_done = match cargo.unit {
            Some(unit) => {
                self.men_ashore_in(unit, hex) == 0
                    || ships.iter().all(|id| self.berths_for(*id, unit) == 0)
            }
            None => true,
        };
        let cargo_done = match (cargo.action.loads(), base) {
            (Some(named), Some(base)) => {
                let base = &self.truth.bases[&base];
                let stock = match named {
                    Cargo::Supply => base.supply_t,
                    Cargo::Fuel => base.fuel_t,
                };
                stock.is_zero()
                    || ships.iter().all(|id| {
                        let ship = &self.truth.ships[id];
                        ship.supply_t + ship.cargo_fuel_t
                            == self.ref_data.ship_classes[&ship.class].cargo_capacity_t
                    })
            }
            // No cargo named, or none to load where the side has no base.
            _ => true,
        };
        men_done && cargo_done
    }

    /// Takes men aboard one ship from the shore, or lands them from it, as
    /// the force's order says. Returns the tons of work they took and the
    /// event, or `None` if no man moved.
    #[allow(clippy::too_many_arguments)]
    fn move_men(
        &mut self,
        day: u32,
        pulse: Pulse,
        p: &Participant,
        ship_id: ShipId,
        on_berth: bool,
        (rate, port_rate, landing, port_work): (
            Fixed,
            Fixed,
            Option<LandingRate>,
            Option<(Fixed, Fixed)>,
        ),
        per_man: Fixed,
        causes: &[Cause],
        divided: &BTreeMap<LandUnitId, EventId>,
    ) -> Option<(Fixed, EventId)> {
        let capacity_by_time = rate * p.hours;
        let by_time = Self::men_by_time(capacity_by_time, per_man);
        let loading = p.cargo.action.is_load();
        let (unit, men, limit_name, limit) = if loading {
            let unit = p.cargo.unit?;
            let ashore = self.men_ashore_in(unit, p.hex);
            let berths = self.berths_for(ship_id, unit);
            let men = by_time.min(ashore).min(berths);
            (unit, men, "berths", berths.min(ashore))
        } else {
            if !p.coast {
                return None;
            }
            let troops = self.truth.ships[&ship_id].troops?;
            (troops.unit, by_time.min(troops.men), "aboard", troops.men)
        };
        if men == 0 {
            return None;
        }
        let men_t = per_man * i64::from(men);

        let land_unit = self
            .truth
            .land_units
            .get_mut(&unit)
            .expect("the land unit exists");
        let ship = self.truth.ships.get_mut(&ship_id).expect("ship exists");
        // The unit's first men in the hex (decision 6 of 0095): a unit just
        // made by division counts.
        let first_men = !loading && land_unit.men_ashore == 0;
        let (aboard, ashore) = if loading {
            land_unit.men_ashore -= men;
            if land_unit.men_ashore == 0 {
                land_unit.hex = None;
            }
            let aboard = ship.troops.map_or(0, |troops| troops.men) + men;
            ship.troops = Some(Troops { unit, men: aboard });
            (aboard, land_unit.men_ashore)
        } else {
            land_unit.men_ashore += men;
            land_unit.hex = Some(p.hex);
            let aboard = ship.troops.map_or(0, |troops| troops.men) - men;
            ship.troops = (aboard > 0).then_some(Troops { unit, men: aboard });
            (aboard, land_unit.men_ashore)
        };

        let breakdown = self.config.record_breakdowns.then(|| {
            let mut factors = vec![
                Factor {
                    name: "hours_worked",
                    value: p.hours,
                },
                Factor {
                    name: "on_berth",
                    value: if on_berth { Fixed::ONE } else { Fixed::ZERO },
                },
                Factor {
                    name: "rate_t_per_h",
                    value: rate,
                },
                Factor {
                    name: "capacity_by_time_t",
                    value: capacity_by_time,
                },
            ];
            factors.extend(landing_factors(landing, port_rate));
            factors.extend(port_work_factors(port_work));
            factors.extend([
                Factor {
                    name: "handling_t_per_man",
                    value: per_man,
                },
                Factor {
                    name: "men_by_time",
                    value: Fixed::from_int(i64::from(by_time)),
                },
                Factor {
                    name: limit_name,
                    value: Fixed::from_int(i64::from(limit)),
                },
            ]);
            Breakdown {
                factors,
                result: Fixed::from_int(i64::from(men)),
            }
        });
        let mut causes = causes.to_vec();
        if let Some(division) = divided.get(&unit) {
            causes.push(Cause::Event(*division));
        }
        let kind = if loading {
            EventKind::MenEmbarked {
                unit,
                ship: ship_id,
                task_force: p.task_force,
                hex: p.hex,
                men,
                aboard,
                ashore,
            }
        } else {
            EventKind::MenLanded {
                unit,
                ship: ship_id,
                task_force: p.task_force,
                hex: p.hex,
                men,
                aboard,
                ashore,
            }
        };
        let event = self
            .events
            .record(day, Some(pulse), kind, causes, breakdown);
        if first_men {
            self.first_men_landed(day, pulse, p.hex, unit, event);
        }
        Some((men_t, event))
    }
}

/// The factors the day before's work on the port adds to a breakdown
/// (decision 4 of 0099): the work and the factor it gives. None where no
/// work was made.
fn port_work_factors(port_work: Option<(Fixed, Fixed)>) -> Vec<Factor> {
    let Some((work, factor)) = port_work else {
        return Vec::new();
    };
    vec![
        Factor {
            name: "port_work",
            value: work,
        },
        Factor {
            name: "port_work_factor",
            value: factor,
        },
    ]
}

/// The factors a landing adds to a breakdown (decision 5 of 0095): the port's
/// rate before the factor, the share an unprepared force works at, the
/// loading counted and the factor they make. None where the share is 1.
fn landing_factors(landing: Option<LandingRate>, port_rate: Fixed) -> Vec<Factor> {
    let Some(landing) = landing else {
        return Vec::new();
    };
    vec![
        Factor {
            name: "port_rate_t_per_h",
            value: port_rate,
        },
        Factor {
            name: "unprepared_landing_share",
            value: landing.share,
        },
        Factor {
            name: "loading",
            value: landing.loading,
        },
        Factor {
            name: "loading_factor",
            value: landing.factor,
        },
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::events::EventKind;
    use crate::hex::Hex;
    use crate::ids::ShipClassId;
    use crate::map::{HexData, Map};
    use crate::orders::{Mission, OrderKind, OrderRequest, Origin, TaskForceOrder};
    use crate::refdata::{RefData, ShipClass, ShipKind};
    use crate::state::{Base, Ship, ShipLocation, Side, TaskForce, TruthState};
    use crate::world::SimConfig;

    /// One task force for `world`: its ID, its hex, and its ships as
    /// (ship ID, supply aboard in tons).
    type Force<'a> = (u32, Hex, &'a [(u32, i64)]);

    fn tf(id: u32) -> TaskForceId {
        TaskForceId::new(id)
    }

    fn ship(id: u32) -> ShipId {
        ShipId::new(id)
    }

    /// Open sea, base 1 at (0,0) with port size `port` and `stock_t` tons,
    /// base 2 at (4,0) with port size 1 and no stock. Cargo ships carry 3000 t
    /// and steam at 16 kn. Test-only port figures:
    ///
    /// - size 0: no berths, lighters 20 t/h
    /// - size 1: `berths` berths at 50 t/h, lighters 20 t/h
    /// - size 2: 2 berths at 250 t/h, lighters 20 t/h
    fn world(port: u8, berths: u32, stock_t: i64, forces: &[Force]) -> World {
        let mut map = Map::new();
        for q in -1..=8 {
            for r in -1..=2 {
                map.insert(Hex::new(q, r), HexData::open_sea());
            }
        }
        for (hex, size) in [(Hex::new(0, 0), port), (Hex::new(4, 0), 1)] {
            map.insert(
                hex,
                HexData {
                    port_size: size,
                    ..HexData::open_sea()
                },
            );
        }

        let class = ShipClass {
            id: ShipClassId::new(1),
            name: "Cargo".to_string(),
            kind: ShipKind::Transport,
            cruise_speed_kn: Fixed::from_int(16),
            max_speed_kn: Fixed::from_int(20),
            fuel_capacity_t: Fixed::from_int(2000),
            endurance_nm: Fixed::from_int(8000),
            cargo_capacity_t: Fixed::from_int(3000),
            main_guns: 0,
            gun_rounds_per_min: Fixed::ZERO,
            gun_rounds_per_gun: 0,
            gun_damage: Fixed::ZERO,
            torpedo_tubes: 0,
            torpedo_tubes_one_side: 0,
            torpedo_damage: Fixed::ZERO,
            toughness: Fixed::from_int(100),
            radar: false,
            aircraft_capacity: 0,
            anti_aircraft: 0,
            troop_capacity_men: 0,
        };
        let mut ref_data = RefData::default();
        ref_data.ship_classes.insert(class.id, class);
        for (size, number, rate) in [(0, 0, 0), (1, berths, 50), (2, 2, 250)] {
            ref_data.port_sizes.insert(
                size,
                PortSize {
                    berths: number,
                    berth_rate_t_per_h: Fixed::from_int(rate),
                    lighter_rate_t_per_h: Fixed::from_int(20),
                },
            );
        }

        let mut truth = TruthState::default();
        for (id, hex, ships) in forces {
            let mut force = TaskForce::at(
                tf(*id),
                format!("Force {id}"),
                Side::Allied,
                *hex,
                Mission::Transport,
            );
            for (ship_id, supply) in *ships {
                force.ships.insert(ship(*ship_id));
                truth.ships.insert(
                    ship(*ship_id),
                    Ship {
                        fuel_t: Fixed::from_int(2000),
                        supply_t: Fixed::from_int(*supply),
                        ..Ship::new(
                            ship(*ship_id),
                            format!("Ship {ship_id}"),
                            ShipClassId::new(1),
                            Side::Allied,
                            ShipLocation::TaskForce(tf(*id)),
                        )
                    },
                );
            }
            truth.task_forces.insert(tf(*id), force);
        }
        for (id, hex, stock) in [(1, Hex::new(0, 0), stock_t), (2, Hex::new(4, 0), 0)] {
            truth.bases.insert(
                BaseId::new(id),
                Base {
                    supply_t: Fixed::from_int(stock),
                    ..Base::new(BaseId::new(id), format!("Base {id}"), Side::Allied, hex)
                },
            );
        }
        World::new(7, ref_data, map, truth).expect("valid setup")
    }

    fn order(target: u32, order: TaskForceOrder) -> OrderRequest {
        OrderRequest {
            kind: OrderKind::TaskForce {
                target: tf(target),
                order,
            },
            origin: Origin::PLAYER,
        }
    }

    fn on_berth(event: &crate::events::Event) -> Fixed {
        event
            .breakdown
            .as_ref()
            .expect("breakdown recorded")
            .factors
            .iter()
            .find(|factor| factor.name == "on_berth")
            .expect("on_berth factor")
            .value
    }

    #[test]
    fn load_fills_ships_in_id_order_when_the_stock_runs_short() {
        // Port size 2: 2 berths at 250 t/h, so the night pulse (12 h) can move
        // 3000 t per ship. Stock 4000 t: ship 1 takes 3000 t, ship 2 the
        // remaining 1000 t. Ship 1 works 12 h, ship 2 works 1000/250 = 4 h.
        // The stock is gone in the night pulse, so loading completes at
        // elapsed hour 0 + 12.
        let mut world = world(2, 1, 4000, &[(1, Hex::new(0, 0), &[(1, 0), (2, 0)])]);
        world.submit_orders(Side::Allied, vec![order(1, TaskForceOrder::Load)]);
        world.resolve_day().expect("day resolves");

        let truth = world.umpire();
        assert_eq!(truth.ships[&ship(1)].supply_t, Fixed::from_int(3000));
        assert_eq!(truth.ships[&ship(2)].supply_t, Fixed::from_int(1000));
        assert_eq!(truth.bases[&BaseId::new(1)].supply_t, Fixed::ZERO);

        let loaded: Vec<_> = world
            .events()
            .iter()
            .filter(|event| matches!(event.kind, EventKind::CargoLoaded { .. }))
            .collect();
        assert_eq!(loaded.len(), 2);
        let completes: Vec<_> = world
            .events()
            .iter()
            .filter(|event| matches!(event.kind, EventKind::LoadingComplete { .. }))
            .collect();
        assert_eq!(completes.len(), 1);
        let complete = completes[0];
        assert_eq!(complete.pulse, Some(Pulse::Night));
        assert_eq!(
            complete.kind,
            EventKind::LoadingComplete {
                task_force: tf(1),
                base: BaseId::new(1),
                cargo: Cargo::Supply,
                elapsed_hours: Fixed::from_int(12),
            }
        );
        let expected_causes: Vec<Cause> = loaded.iter().map(|e| Cause::Event(e.id)).collect();
        assert_eq!(complete.causes, expected_causes);
    }

    #[test]
    fn a_task_force_whose_ships_are_all_sunk_works_no_cargo() {
        for action in [TaskForceOrder::Load, TaskForceOrder::Unload] {
            let mut world = world(1, 1, 1000, &[(1, Hex::new(0, 0), &[(1, 0), (2, 500)])]);
            for raw in [1, 2] {
                let record = world.truth.ships.get_mut(&ship(raw)).expect("ship");
                record.location = ShipLocation::Sunk;
                record.flotation_damage = Fixed::from_int(100);
            }
            world
                .truth
                .task_forces
                .get_mut(&tf(1))
                .expect("task force")
                .ships
                .clear();
            world.submit_orders(Side::Allied, vec![order(1, action)]);
            world.resolve_day().expect("day resolves");

            assert!(world.events().iter().all(|event| !matches!(
                event.kind,
                EventKind::CargoLoaded { .. }
                    | EventKind::CargoUnloaded { .. }
                    | EventKind::LoadingComplete { .. }
                    | EventKind::UnloadingComplete { .. }
            )));
            let truth = world.umpire();
            assert_eq!(truth.bases[&BaseId::new(1)].supply_t, Fixed::from_int(1000));
            // The sunk ship keeps what it carried: nothing is unloaded from it.
            assert_eq!(truth.ships[&ship(2)].supply_t, Fixed::from_int(500));
        }
    }

    #[test]
    fn unload_without_a_base_here_does_nothing() {
        let mut world = world(1, 1, 0, &[(1, Hex::new(2, 0), &[(1, 500)])]);
        world.submit_orders(Side::Allied, vec![order(1, TaskForceOrder::Unload)]);
        world.resolve_day().expect("day resolves");

        assert!(!world.events().iter().any(|event| matches!(
            event.kind,
            EventKind::CargoUnloaded { .. } | EventKind::UnloadingComplete { .. }
        )));
        assert_eq!(
            world.umpire().ships[&ship(1)].supply_t,
            Fixed::from_int(500)
        );
        assert_eq!(world.umpire().bases[&BaseId::new(1)].supply_t, Fixed::ZERO);
    }

    #[test]
    fn a_task_force_that_ran_dry_does_not_work_cargo() {
        // 1 t of fuel at 0.25 t/nm lasts 4 nm. The force runs dry 4 nm out of
        // (0,0). It still counts as being in that hex, where base 1 is, but it
        // is adrift and must not unload.
        let mut world = world(1, 1, 0, &[(1, Hex::new(0, 0), &[(1, 1000)])]);
        world
            .truth
            .ships
            .get_mut(&ship(1))
            .expect("ship exists")
            .fuel_t = Fixed::ONE;
        world.submit_orders(
            Side::Allied,
            vec![
                order(1, TaskForceOrder::route(vec![Hex::new(4, 0)])),
                order(1, TaskForceOrder::Unload),
            ],
        );
        world.resolve_day().expect("day resolves");

        let truth = world.umpire();
        assert_eq!(truth.task_forces[&tf(1)].status, TaskForceStatus::OutOfFuel);
        let task_force = &truth.task_forces[&tf(1)];
        // 4 nm along its first leg: still nearest (0,0).
        assert_eq!(task_force.position(), Hex::new(0, 0));
        assert_eq!(task_force.from_hex, Hex::new(0, 0));
        assert_eq!(task_force.toward_hex, Some(Hex::new(1, 0)));
        assert_eq!(task_force.progress_nm, Fixed::from_int(4));
        assert_eq!(truth.ships[&ship(1)].supply_t, Fixed::from_int(1000));
        assert!(!world
            .events()
            .iter()
            .any(|event| matches!(event.kind, EventKind::CargoUnloaded { .. })));
    }

    #[test]
    fn an_idle_force_part_way_along_a_leg_works_no_cargo() {
        // Idle, 4 nm out of (0,0) toward (1,0): nearest base 1's hex, but not
        // at its centre, as refuelling asks. The load is not worked.
        let mut world = world(1, 1, 5000, &[(1, Hex::new(0, 0), &[(1, 0)])]);
        {
            let force = world.truth.task_forces.get_mut(&tf(1)).expect("force");
            force.toward_hex = Some(Hex::new(1, 0));
            force.progress_nm = Fixed::from_int(4);
            force.leg_nm = Fixed::from_int(crate::hex::HEX_NM);
            force.cargo_order = Some(crate::state::CargoOrder {
                action: crate::orders::CargoAction::Load,
                order: crate::ids::OrderId::new(1),
                unit: None,
            });
        }
        assert_eq!(world.truth.task_forces[&tf(1)].position(), Hex::new(0, 0));
        world.run_pulse_phase(1, Pulse::Night, crate::resolve::PulsePhase::Loading);
        assert!(!world
            .events()
            .iter()
            .any(|event| matches!(event.kind, EventKind::CargoLoaded { .. })));
        assert_eq!(world.umpire().ships[&ship(1)].supply_t, Fixed::ZERO);
    }

    /// Unloaded tons by ship and whether it was on a berth, in the night pulse.
    fn night_unloads(world: &World) -> Vec<(ShipId, Fixed, Fixed)> {
        world
            .events()
            .iter()
            .filter(|event| event.pulse == Some(Pulse::Night))
            .filter_map(|event| match event.kind {
                EventKind::CargoUnloaded {
                    ship, unloaded_t, ..
                } => Some((ship, unloaded_t, on_berth(event))),
                _ => None,
            })
            .collect()
    }

    #[test]
    fn berths_go_by_task_force_then_ship_id() {
        let forces: [Force; 2] = [
            (1, Hex::new(0, 0), &[(1, 1000), (2, 1000)]),
            (2, Hex::new(0, 0), &[(3, 1000), (4, 1000)]),
        ];
        // Port size 1 with one berth at 50 t/h, lighters 20 t/h, 12 hours:
        // the berth moves 600 t, a lighter 240 t.
        let mut one = world(1, 1, 0, &forces);
        for id in [1, 2] {
            one.submit_orders(Side::Allied, vec![order(id, TaskForceOrder::Unload)]);
        }
        one.resolve_day().expect("day resolves");
        let (t600, t240) = (Fixed::from_int(600), Fixed::from_int(240));
        assert_eq!(
            night_unloads(&one),
            vec![
                (ship(1), t600, Fixed::ONE),
                (ship(2), t240, Fixed::ZERO),
                (ship(3), t240, Fixed::ZERO),
                (ship(4), t240, Fixed::ZERO),
            ]
        );

        // Two berths: ships 1 and 2 are berthed, ships 3 and 4 use lighters.
        let mut two = world(1, 2, 0, &forces);
        for id in [1, 2] {
            two.submit_orders(Side::Allied, vec![order(id, TaskForceOrder::Unload)]);
        }
        two.resolve_day().expect("day resolves");
        assert_eq!(
            night_unloads(&two),
            vec![
                (ship(1), t600, Fixed::ONE),
                (ship(2), t600, Fixed::ONE),
                (ship(3), t240, Fixed::ZERO),
                (ship(4), t240, Fixed::ZERO),
            ]
        );
    }

    #[test]
    fn unload_waits_for_arrival_and_counts_hours_from_it() {
        // (0,0) to (4,0) is 160 nm: 10 hours at 16 kn. The force arrives
        // 10 hours into the 12-hour night pulse and unloads for 2 hours at
        // the berth rate of 50 t/h: 100 t.
        let mut world = world(1, 1, 0, &[(1, Hex::new(0, 0), &[(1, 1000)])]);
        world.submit_orders(
            Side::Allied,
            vec![
                order(1, TaskForceOrder::route(vec![Hex::new(4, 0)])),
                order(1, TaskForceOrder::Unload),
            ],
        );
        world.resolve_day().expect("day resolves");

        let truth = world.umpire();
        let force = &truth.task_forces[&tf(1)];
        let unload_order = force.cargo_order.expect("cargo order").order;
        let arrival = force.arrival_event.expect("arrival event");
        assert_eq!(force.arrived_at_hours, Some(Fixed::from_int(10)));

        let unloads: Vec<_> = world
            .events()
            .iter()
            .filter(|event| matches!(event.kind, EventKind::CargoUnloaded { .. }))
            .collect();
        let first = unloads[0];
        assert_eq!(first.pulse, Some(Pulse::Night));
        assert_eq!(
            first.kind,
            EventKind::CargoUnloaded {
                ship: ship(1),
                task_force: tf(1),
                base: BaseId::new(2),
                cargo: Cargo::Supply,
                unloaded_t: Fixed::from_int(100),
                aboard_t: Fixed::from_int(900),
                base_stock_t: Fixed::from_int(100),
            }
        );
        assert_eq!(
            first.causes,
            vec![Cause::Order(unload_order), Cause::Event(arrival)]
        );
        // Nothing moved at base 1 where the force started.
        assert_eq!(truth.bases[&BaseId::new(1)].supply_t, Fixed::ZERO);
        // Day 1 total: 2 h, then 6 h and 6 h of full pulses at 50 t/h.
        assert_eq!(
            truth.ships[&ship(1)].supply_t,
            Fixed::from_int(1000 - 100 - 300 - 300)
        );
    }

    #[test]
    fn breakdown_recording_does_not_change_the_hash_with_cargo() {
        let run = |record_breakdowns: bool| {
            let mut world = world(2, 1, 4000, &[(1, Hex::new(0, 0), &[(1, 0), (2, 0)])]);
            world.set_config(SimConfig { record_breakdowns });
            world.submit_orders(Side::Allied, vec![order(1, TaskForceOrder::Load)]);
            world.resolve_day().expect("day resolves");
            let cargo_events: Vec<_> = world
                .events()
                .iter()
                .filter(|event| {
                    matches!(
                        event.kind,
                        EventKind::CargoLoaded { .. } | EventKind::LoadingComplete { .. }
                    )
                })
                .collect();
            let all_have = cargo_events.iter().all(|event| event.breakdown.is_some());
            let none_have = cargo_events.iter().all(|event| event.breakdown.is_none());
            (
                world.state_hash(),
                !cargo_events.is_empty(),
                all_have,
                none_have,
            )
        };
        let (with_hash, with_events, with_all, _) = run(true);
        let (without_hash, without_events, _, without_none) = run(false);
        assert!(with_events && without_events);
        assert!(with_all);
        assert!(without_none);
        assert_eq!(with_hash, without_hash);
    }

    fn set_fuel_stock(world: &mut World, base: u32, tons: i64) {
        world
            .truth
            .bases
            .get_mut(&BaseId::new(base))
            .expect("base")
            .fuel_t = Fixed::from_int(tons);
    }

    fn set_fuel_cargo(world: &mut World, id: u32, tons: i64) {
        world
            .truth
            .ships
            .get_mut(&ship(id))
            .expect("ship")
            .cargo_fuel_t = Fixed::from_int(tons);
    }

    fn fuel_stock(world: &World, base: u32) -> Fixed {
        world.umpire().bases[&BaseId::new(base)].fuel_t
    }

    fn fuel_cargo(world: &World, id: u32) -> Fixed {
        world.umpire().ships[&ship(id)].cargo_fuel_t
    }

    /// The cargo events of a pulse as (ship, cargo, tons, aboard, stock).
    fn moves_in(world: &World, pulse: Pulse) -> Vec<(ShipId, Cargo, i64, i64, i64)> {
        world
            .events()
            .iter()
            .filter(|event| event.pulse == Some(pulse))
            .filter_map(|event| match event.kind {
                EventKind::CargoLoaded {
                    ship,
                    cargo,
                    loaded_t,
                    aboard_t,
                    base_stock_t,
                    ..
                } => Some((ship, cargo, loaded_t, aboard_t, base_stock_t)),
                EventKind::CargoUnloaded {
                    ship,
                    cargo,
                    unloaded_t,
                    aboard_t,
                    base_stock_t,
                    ..
                } => Some((ship, cargo, unloaded_t, aboard_t, base_stock_t)),
                _ => None,
            })
            .map(|(ship, cargo, a, b, c)| (ship, cargo, a.trunc(), b.trunc(), c.trunc()))
            .collect()
    }

    #[test]
    fn a_load_of_fuel_fills_only_the_remaining_capacity() {
        // Ship 1 carries 1000 t of supply, so it has room for 2000 t of
        // fuel. Port size 2 moves 250 t/h: 3000 t in the night pulse, so the
        // room is the limit. Ship 2 is empty and takes 3000 t. Supply is
        // not touched.
        let mut world = world(2, 1, 0, &[(1, Hex::new(0, 0), &[(1, 1000), (2, 0)])]);
        set_fuel_stock(&mut world, 1, 10_000);
        world.submit_orders(Side::Allied, vec![order(1, TaskForceOrder::LoadFuel)]);
        world.resolve_day().expect("day resolves");

        assert_eq!(fuel_cargo(&world, 1), Fixed::from_int(2000));
        assert_eq!(fuel_cargo(&world, 2), Fixed::from_int(3000));
        assert_eq!(
            world.umpire().ships[&ship(1)].supply_t,
            Fixed::from_int(1000)
        );
        assert_eq!(fuel_stock(&world, 1), Fixed::from_int(5000));
        assert_eq!(world.umpire().bases[&BaseId::new(1)].supply_t, Fixed::ZERO);
        // Their own tanks are not touched.
        assert_eq!(world.umpire().ships[&ship(1)].fuel_t, Fixed::from_int(2000));
        assert_eq!(
            moves_in(&world, Pulse::Night),
            vec![
                (ship(1), Cargo::Fuel, 2000, 2000, 8000),
                (ship(2), Cargo::Fuel, 3000, 3000, 5000),
            ]
        );
        let complete = world
            .events()
            .iter()
            .find(|event| matches!(event.kind, EventKind::LoadingComplete { .. }))
            .expect("loading completes");
        assert_eq!(
            complete.kind,
            EventKind::LoadingComplete {
                task_force: tf(1),
                base: BaseId::new(1),
                cargo: Cargo::Fuel,
                elapsed_hours: Fixed::from_int(12),
            }
        );
    }

    #[test]
    fn a_load_of_supply_leaves_room_for_the_fuel_already_aboard() {
        let mut world = world(2, 1, 10_000, &[(1, Hex::new(0, 0), &[(1, 0)])]);
        set_fuel_cargo(&mut world, 1, 1000);
        world.submit_orders(Side::Allied, vec![order(1, TaskForceOrder::Load)]);
        world.resolve_day().expect("day resolves");
        assert_eq!(
            world.umpire().ships[&ship(1)].supply_t,
            Fixed::from_int(2000)
        );
        assert_eq!(fuel_cargo(&world, 1), Fixed::from_int(1000));
        assert!(crate::invariants::check(&world.truth, &world.map, &world.ref_data).is_empty());
    }

    #[test]
    fn a_load_that_names_no_cargo_is_supply() {
        let mut world = world(2, 1, 4000, &[(1, Hex::new(0, 0), &[(1, 0)])]);
        set_fuel_stock(&mut world, 1, 5000);
        world.submit_orders(Side::Allied, vec![order(1, TaskForceOrder::Load)]);
        world.resolve_day().expect("day resolves");
        assert_eq!(
            world.umpire().ships[&ship(1)].supply_t,
            Fixed::from_int(3000)
        );
        assert_eq!(fuel_cargo(&world, 1), Fixed::ZERO);
        assert_eq!(fuel_stock(&world, 1), Fixed::from_int(5000));
    }

    #[test]
    fn fuel_running_short_mid_phase_follows_ship_order() {
        // 4000 t of fuel: ship 1 takes 3000 t, ship 2 the 1000 t left.
        let mut world = world(2, 1, 0, &[(1, Hex::new(0, 0), &[(1, 0), (2, 0)])]);
        set_fuel_stock(&mut world, 1, 4000);
        world.submit_orders(Side::Allied, vec![order(1, TaskForceOrder::LoadFuel)]);
        world.resolve_day().expect("day resolves");
        assert_eq!(fuel_cargo(&world, 1), Fixed::from_int(3000));
        assert_eq!(fuel_cargo(&world, 2), Fixed::from_int(1000));
        assert_eq!(fuel_stock(&world, 1), Fixed::ZERO);
        let complete = world
            .events()
            .iter()
            .find(|event| matches!(event.kind, EventKind::LoadingComplete { .. }))
            .expect("loading completes");
        assert_eq!(complete.pulse, Some(Pulse::Night));
    }

    #[test]
    fn an_unload_lands_supply_first_and_then_fuel_within_the_pulse_tons() {
        // One berth at 50 t/h. The force is held to the morning pulse (6 h:
        // 300 t) by an arrival hour of 12. Ship 1 has 100 t of supply and
        // 1000 t of fuel: it lands 100 t of supply and 200 t of fuel.
        let mut world = world(1, 1, 0, &[(1, Hex::new(0, 0), &[(1, 100)])]);
        set_fuel_cargo(&mut world, 1, 1000);
        set_fuel_stock(&mut world, 1, 7);
        world
            .truth
            .task_forces
            .get_mut(&tf(1))
            .expect("task force")
            .arrived_at_hours = Some(Fixed::from_int(12));
        world.submit_orders(Side::Allied, vec![order(1, TaskForceOrder::Unload)]);
        world.resolve_day().expect("day resolves");

        assert_eq!(moves_in(&world, Pulse::Night), Vec::new());
        assert_eq!(
            moves_in(&world, Pulse::Morning),
            vec![
                (ship(1), Cargo::Supply, 100, 0, 100),
                (ship(1), Cargo::Fuel, 200, 800, 207),
            ]
        );
        // The afternoon moves 300 t more of fuel.
        assert_eq!(
            moves_in(&world, Pulse::Afternoon),
            vec![(ship(1), Cargo::Fuel, 300, 500, 507)]
        );
        assert_eq!(
            world.umpire().bases[&BaseId::new(1)].supply_t,
            Fixed::from_int(100)
        );
        assert_eq!(fuel_stock(&world, 1), Fixed::from_int(507));
        assert_eq!(fuel_cargo(&world, 1), Fixed::from_int(500));
        // Not complete while fuel is aboard.
        assert!(!world
            .events()
            .iter()
            .any(|event| matches!(event.kind, EventKind::UnloadingComplete { .. })));
    }

    #[test]
    fn an_unload_is_complete_when_nothing_of_either_kind_is_aboard() {
        let mut world = world(1, 1, 0, &[(1, Hex::new(0, 0), &[(1, 100)])]);
        set_fuel_cargo(&mut world, 1, 200);
        world.submit_orders(Side::Allied, vec![order(1, TaskForceOrder::Unload)]);
        world.resolve_day().expect("day resolves");
        // 300 t in 6 hours at 50 t/h: done in the night pulse.
        let complete = world
            .events()
            .iter()
            .find(|event| matches!(event.kind, EventKind::UnloadingComplete { .. }))
            .expect("unloading completes");
        assert_eq!(complete.pulse, Some(Pulse::Night));
        assert_eq!(
            complete.kind,
            EventKind::UnloadingComplete {
                task_force: tf(1),
                base: BaseId::new(1),
                elapsed_hours: Fixed::from_int(6),
            }
        );
        assert_eq!(fuel_stock(&world, 1), Fixed::from_int(200));
    }

    #[test]
    fn nothing_is_unloaded_or_loaded_at_an_enemy_base() {
        let mut world = world(1, 1, 500, &[(1, Hex::new(0, 0), &[(1, 100)])]);
        set_fuel_cargo(&mut world, 1, 200);
        set_fuel_stock(&mut world, 1, 900);
        world
            .truth
            .bases
            .get_mut(&BaseId::new(1))
            .expect("base")
            .side = Side::Japanese;
        world.submit_orders(Side::Allied, vec![order(1, TaskForceOrder::Unload)]);
        world.resolve_day().expect("day resolves");
        assert!(!world.events().iter().any(|event| matches!(
            event.kind,
            EventKind::CargoUnloaded { .. } | EventKind::UnloadingComplete { .. }
        )));
        assert_eq!(
            world.umpire().ships[&ship(1)].supply_t,
            Fixed::from_int(100)
        );
        assert_eq!(fuel_cargo(&world, 1), Fixed::from_int(200));
        assert_eq!(fuel_stock(&world, 1), Fixed::from_int(900));
        assert_eq!(
            world.umpire().bases[&BaseId::new(1)].supply_t,
            Fixed::from_int(500)
        );
    }

    #[test]
    fn a_ship_cannot_burn_its_cargo() {
        // 10 t of fuel at 0.25 t/nm lasts 40 nm. A hold full of fuel does not
        // help: the force runs dry and the cargo is as it was.
        let mut world = world(1, 1, 0, &[(1, Hex::new(0, 0), &[(1, 0)])]);
        set_fuel_cargo(&mut world, 1, 3000);
        world.truth.ships.get_mut(&ship(1)).expect("ship").fuel_t = Fixed::from_int(10);
        world.submit_orders(
            Side::Allied,
            vec![order(1, TaskForceOrder::route(vec![Hex::new(8, 0)]))],
        );
        world.resolve_day().expect("day resolves");
        let truth = world.umpire();
        assert_eq!(truth.task_forces[&tf(1)].status, TaskForceStatus::OutOfFuel);
        assert_eq!(truth.ships[&ship(1)].fuel_t, Fixed::ZERO);
        assert_eq!(truth.ships[&ship(1)].cargo_fuel_t, Fixed::from_int(3000));
        assert!(crate::invariants::check(&world.truth, &world.map, &world.ref_data).is_empty());
    }

    #[test]
    fn carrying_no_fuel_cargo_leaves_the_hash_as_it_was() {
        let mut world = world(2, 1, 4000, &[(1, Hex::new(0, 0), &[(1, 0)])]);
        let before = world.state_hash();
        set_fuel_cargo(&mut world, 1, 1);
        assert_ne!(world.state_hash(), before);
        set_fuel_cargo(&mut world, 1, 0);
        assert_eq!(world.state_hash(), before);
    }
}
