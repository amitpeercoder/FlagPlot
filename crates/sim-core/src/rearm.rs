//! Rearming in port, in the hours of each pulse (decision 0122).
//!
//! A ship stopped at a base of its side that lacks main battery rounds or
//! torpedoes, where the base holds ammunition and the port's size gives a
//! rate for it, takes them in the cargo rule's queue
//! ([`World::handle_cargo`]): after its cargo and its fuel, in the hours of
//! the pulse they left, at the port's ammunition rate at a berth or by
//! lighter, rounds first and then torpedoes, whole ones only. There is no
//! randomness and no order. With no ammunition at a base, no rate or no
//! `[rearming]` weights, nothing is done.

use std::collections::BTreeMap;

use crate::events::{Breakdown, Cause, EventKind, Factor};
use crate::fixed::Fixed;
use crate::hex::Hex;
use crate::ids::{BaseId, ShipId};
use crate::refdata::{PortSize, RearmingFigures, ShipClass};
use crate::refuel::FuelStop;
use crate::state::{Ship, ShipLocation, Side};
use crate::time::Pulse;
use crate::world::World;

/// The gun rounds a class carries a gun of which marks ammunition that is
/// not limiting (decision 0122, 2): a ship of it lacks no rounds.
pub const UNLIMITED_ROUNDS_PER_GUN: u32 = 9_999;

/// What a ship lacks (decision 0122, 2): whole main battery rounds below its
/// outfit (none for the marker), and its tubes that have fired.
pub fn ammunition_lacked(ship: &Ship, class: &ShipClass) -> (u32, u32) {
    let rounds = if class.gun_rounds_per_gun >= UNLIMITED_ROUNDS_PER_GUN {
        0
    } else {
        let outfit =
            Fixed::from_int(i64::from(class.main_guns) * i64::from(class.gun_rounds_per_gun));
        u32::try_from((outfit - ship.gun_rounds).max(Fixed::ZERO).trunc()).unwrap_or(u32::MAX)
    };
    let torpedoes = ship.tubes_fired(class.torpedo_tubes, class.torpedo_tubes_one_side);
    (rounds, torpedoes)
}

/// The tons of what a ship lacks, by the weights (decision 0122, 3).
pub fn ammunition_lacked_t(ship: &Ship, class: &ShipClass, weights: &RearmingFigures) -> Fixed {
    let (rounds, torpedoes) = ammunition_lacked(ship, class);
    weights.tons_per_round(class.kind) * Fixed::from_int(i64::from(rounds))
        + weights.tons_per_torpedo * Fixed::from_int(i64::from(torpedoes))
}

/// The ammunition rate of `port` for a ship at a berth or by lighter.
pub fn ammunition_rate(port: &PortSize, on_berth: bool) -> Option<Fixed> {
    if on_berth {
        port.ammunition_berth_rate_t_per_h
    } else {
        port.ammunition_lighter_rate_t_per_h
    }
}

/// What a ship takes in `hours` at `rate` from `stock` tons (decision 0122,
/// 6): whole rounds up to `lacks.0`, then whole torpedoes up to `lacks.1`,
/// each within the tons the rate and the stock allow. The rounds, the
/// torpedoes, the tons and the hours they take.
pub fn ammunition_taken(
    lacks: (u32, u32),
    round_t: Fixed,
    torpedo_t: Fixed,
    rate: Fixed,
    hours: Fixed,
    stock: Fixed,
) -> (u32, u32, Fixed, Fixed) {
    let mut room = (rate * hours).min(stock).max(Fixed::ZERO);
    let whole = |room: Fixed, weight: Fixed, most: u32| -> u32 {
        if weight <= Fixed::ZERO {
            return 0;
        }
        u32::try_from((room / weight).trunc().max(0))
            .unwrap_or(u32::MAX)
            .min(most)
    };
    let rounds = whole(room, round_t, lacks.0);
    let rounds_t = round_t * Fixed::from_int(i64::from(rounds));
    room -= rounds_t;
    let torpedoes = whole(room, torpedo_t, lacks.1);
    let tons = rounds_t + torpedo_t * Fixed::from_int(i64::from(torpedoes));
    let hours = if rate > Fixed::ZERO {
        tons / rate
    } else {
        Fixed::ZERO
    };
    (rounds, torpedoes, tons, hours)
}

impl World {
    /// The lowest-numbered base of `side` in `hex` that holds ammunition.
    fn ammunition_base(&self, side: Side, hex: Hex) -> Option<BaseId> {
        self.truth
            .bases
            .values()
            .find(|base| base.side == side && base.hex == hex && base.ammunition_t > Fixed::ZERO)
            .map(|base| base.id)
    }

    /// Whether `ship` has ammunition to take that a berth would serve
    /// (decision 0122, 5): it lacks rounds or torpedoes, the weights are
    /// given, a base of the side here holds ammunition, and the port gives a
    /// rate at a berth.
    pub(crate) fn has_ammunition_to_take(
        &self,
        side: Side,
        hex: Hex,
        port: &PortSize,
        ship: ShipId,
    ) -> bool {
        let Some(weights) = self.rearming_figures() else {
            return false;
        };
        let record = &self.truth.ships[&ship];
        if record.location == ShipLocation::Sunk || port.ammunition_berth_rate_t_per_h.is_none() {
            return false;
        }
        let class = &self.ref_data.ship_classes[&record.class];
        ammunition_lacked_t(record, class, &weights) > Fixed::ZERO
            && self.ammunition_base(side, hex).is_some()
    }

    /// The weights of rearming, where the scenario gives them.
    pub(crate) fn rearming_figures(&self) -> Option<RearmingFigures> {
        self.ref_data
            .surface_combat
            .as_ref()
            .and_then(|figures| figures.rearming)
    }

    /// Rearms the ships of a force stopped at a base of its side, for the
    /// hours of this pulse that its cargo and fuel left them (`used`), in
    /// ship order (decision 0122, 5 to 9). Each ship taking anything records
    /// `ShipRearmed`, caused as its refuelling is.
    pub(crate) fn take_ammunition(
        &mut self,
        day: u32,
        pulse: Pulse,
        stop: &FuelStop,
        berthed: &[ShipId],
        used: &BTreeMap<ShipId, Fixed>,
    ) {
        let Some(weights) = self.rearming_figures() else {
            return;
        };
        let causes: Vec<Cause> = stop.arrival_event.into_iter().map(Cause::Event).collect();
        for &ship_id in stop.ships {
            let ship = &self.truth.ships[&ship_id];
            if ship.location == ShipLocation::Sunk {
                continue;
            }
            let class = &self.ref_data.ship_classes[&ship.class];
            let lacks = ammunition_lacked(ship, class);
            if lacks == (0, 0) {
                continue;
            }
            let on_berth = berthed.contains(&ship_id);
            let Some(rate) = ammunition_rate(&stop.port, on_berth) else {
                continue;
            };
            let Some(base_id) = self.ammunition_base(stop.side, stop.hex) else {
                continue;
            };
            let stock = self.truth.bases[&base_id].ammunition_t;
            let hours_used = used.get(&ship_id).copied().unwrap_or(Fixed::ZERO);
            let hours_left = (stop.hours - hours_used).max(Fixed::ZERO);
            let round_t = weights.tons_per_round(class.kind);
            let torpedo_t = weights.tons_per_torpedo;
            let (rounds, torpedoes, tons, _) =
                ammunition_taken(lacks, round_t, torpedo_t, rate, hours_left, stock);
            if rounds == 0 && torpedoes == 0 {
                continue;
            }
            let tubes = class.torpedo_tubes;
            let fired = ship.tubes_fired(tubes, class.torpedo_tubes_one_side);
            let ship = self.truth.ships.get_mut(&ship_id).expect("ship exists");
            ship.gun_rounds += Fixed::from_int(i64::from(rounds));
            if torpedoes > 0 {
                // The lowest numbered fired tubes are loaded first (8).
                let left = fired - torpedoes;
                ship.torpedoes_fired = left;
                ship.torpedoes_ready = left < tubes;
                ship.torpedoes_one_side_fired = false;
            }
            let rounds_aboard = ship.gun_rounds;
            let tubes_fired = ship.tubes_fired(tubes, class.torpedo_tubes_one_side);
            let base = self.truth.bases.get_mut(&base_id).expect("base exists");
            base.ammunition_t -= tons;
            let left = base.ammunition_t;
            let breakdown = self.config.record_breakdowns.then(|| Breakdown {
                factors: vec![
                    Factor {
                        name: "hours_worked",
                        value: stop.hours,
                    },
                    Factor {
                        name: "hours_used",
                        value: hours_used,
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
                        name: "tons_per_round",
                        value: round_t,
                    },
                    Factor {
                        name: "tons_per_torpedo",
                        value: torpedo_t,
                    },
                    Factor {
                        name: "rounds_lacked",
                        value: Fixed::from_int(i64::from(lacks.0)),
                    },
                    Factor {
                        name: "torpedoes_lacked",
                        value: Fixed::from_int(i64::from(lacks.1)),
                    },
                    Factor {
                        name: "base_ammunition_t",
                        value: stock,
                    },
                ],
                result: tons,
            });
            self.events.record(
                day,
                Some(pulse),
                EventKind::ShipRearmed {
                    ship: ship_id,
                    task_force: stop.task_force,
                    base: base_id,
                    rounds,
                    torpedoes,
                    taken_t: tons,
                    rounds_aboard,
                    tubes_fired,
                    base_ammunition_t: left,
                },
                causes.clone(),
                breakdown,
            );
        }
    }
}

/// One ship's part in a force's rearming where it lies, as the list of
/// forces and the staff read it (decision 0122, 10; decision 0127, 3).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct ShipAmmunition {
    pub ship: ShipId,
    /// What it lacks: whole rounds, and its tubes fired.
    pub rounds: u32,
    pub torpedoes: u32,
    /// Its rounds aboard and its outfit; `None` for the marker.
    pub rounds_aboard: Fixed,
    pub outfit: Option<u32>,
    /// Its class's tubes.
    pub tubes: u32,
    /// Tons a round of its kind and a torpedo weigh; 0 with no weights.
    pub round_t: Fixed,
    pub torpedo_t: Fixed,
    /// The rate it would take ammunition at where it lies, at a berth or by
    /// lighter; `None` where none is given or no base of its side there
    /// holds ammunition.
    pub rate: Option<Fixed>,
    pub on_berth: bool,
    /// The hours its fuel would take before it, from the start of a pulse.
    pub hours_before: Fixed,
}

/// What a force lacks in ammunition and how it would rearm where it lies
/// (decision 0122, 10).
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct AmmunitionWork {
    pub ships: Vec<ShipAmmunition>,
    /// The base of its side in its hex that holds ammunition, and its stock.
    pub base: Option<(BaseId, Fixed)>,
}

impl AmmunitionWork {
    /// Rounds, torpedoes and tons the force lacks.
    pub fn lacks(&self) -> (u32, u32, Fixed) {
        self.ships
            .iter()
            .fold((0, 0, Fixed::ZERO), |(r, t, w), ship| {
                (
                    r + ship.rounds,
                    t + ship.torpedoes,
                    w + ship.round_t * Fixed::from_int(i64::from(ship.rounds))
                        + ship.torpedo_t * Fixed::from_int(i64::from(ship.torpedoes)),
                )
            })
    }

    /// The hours from the start of a pulse until every ship has taken what
    /// it lacks, behind its fuel, by its rate where it lies: the longest
    /// ship's. `None` where a ship that lacks something has no rate, or the
    /// base's stock is less than the force lacks. Pulses are not counted:
    /// hours are worked through, as the queue works them.
    pub fn hours_to_full(&self) -> Option<Fixed> {
        let (_, _, tons) = self.lacks();
        if tons.is_zero() {
            return Some(Fixed::ZERO);
        }
        if self.base.is_none_or(|(_, stock)| stock < tons) {
            return None;
        }
        let mut most = Fixed::ZERO;
        for ship in &self.ships {
            let need = ship.round_t * Fixed::from_int(i64::from(ship.rounds))
                + ship.torpedo_t * Fixed::from_int(i64::from(ship.torpedoes));
            if need.is_zero() {
                continue;
            }
            let rate = ship.rate?;
            most = most.max(ship.hours_before + need / rate);
        }
        Some(most)
    }
}

/// What `force` of `side` lacks and how it would rearm where it lies, read
/// from the side's own truth only: its ships, its bases in the hex, the port
/// and the weights. A berth goes to the ships of the side's stopped forces in
/// the hex that lack fuel or ammunition, in force and ship order, as the
/// queue gives them out, cargo aside. `None` for a force not the side's.
pub fn ammunition_work(
    truth: &crate::state::TruthState,
    map: &crate::map::Map,
    ref_data: &crate::refdata::RefData,
    side: Side,
    force: crate::ids::TaskForceId,
) -> Option<AmmunitionWork> {
    let record = truth.task_forces.get(&force)?;
    if record.side != side {
        return None;
    }
    let hex = record.position();
    let weights = ref_data.surface_combat.as_ref().and_then(|f| f.rearming);
    let base = truth
        .bases
        .values()
        .find(|b| b.side == side && b.hex == hex && b.ammunition_t > Fixed::ZERO)
        .map(|b| (b.id, b.ammunition_t));
    let size = map.get(hex).map_or(0, |data| data.port_size);
    let port = ref_data.port_sizes.get(&size).copied();
    let fuel_base = truth.bases.values().any(|b| {
        b.side == side && b.hex == hex && b.above_reserve(crate::orders::Cargo::Fuel) > Fixed::ZERO
    });
    let lacks_of = |ship: &Ship| {
        let class = &ref_data.ship_classes[&ship.class];
        let (rounds, torpedoes) = ammunition_lacked(ship, class);
        let ammunition = weights.is_some() && base.is_some() && (rounds > 0 || torpedoes > 0);
        let fuel = fuel_base && class.fuel_capacity_t > ship.fuel_t;
        (ammunition, fuel)
    };
    // The berths as the queue would give them, ammunition and fuel alone.
    let mut berthed: Vec<ShipId> = Vec::new();
    if let Some(port) = port {
        for other in truth.task_forces.values().filter(|f| {
            f.side == side
                && f.toward_hex.is_none()
                && f.from_hex == hex
                && !matches!(f.status, crate::state::TaskForceStatus::Underway)
        }) {
            for id in &other.ships {
                if berthed.len() >= port.berths as usize {
                    break;
                }
                let ship = &truth.ships[id];
                let (ammunition, fuel) = lacks_of(ship);
                let claims = (ammunition && port.ammunition_berth_rate_t_per_h.is_some())
                    || (fuel && port.fuel_lighter_rate_t_per_h.is_some());
                if claims {
                    berthed.push(*id);
                }
            }
        }
    }
    let ships = record
        .ships
        .iter()
        .map(|id| {
            let ship = &truth.ships[id];
            let class = &ref_data.ship_classes[&ship.class];
            let (rounds, torpedoes) = ammunition_lacked(ship, class);
            let on_berth = berthed.contains(id);
            let rate = port
                .filter(|_| base.is_some() && weights.is_some())
                .and_then(|port| ammunition_rate(&port, on_berth));
            let fuel_rate = port.and_then(|port| {
                if on_berth {
                    port.fuel_berth_rate_t_per_h
                } else {
                    port.fuel_lighter_rate_t_per_h
                }
            });
            let fuel_lacks = (class.fuel_capacity_t - ship.fuel_t).max(Fixed::ZERO);
            let hours_before = match fuel_rate {
                Some(rate) if fuel_base && rate > Fixed::ZERO => fuel_lacks / rate,
                _ => Fixed::ZERO,
            };
            ShipAmmunition {
                ship: *id,
                rounds,
                torpedoes,
                rounds_aboard: ship.gun_rounds,
                outfit: (class.gun_rounds_per_gun < UNLIMITED_ROUNDS_PER_GUN)
                    .then(|| class.main_guns * class.gun_rounds_per_gun),
                tubes: class.torpedo_tubes,
                round_t: weights.map_or(Fixed::ZERO, |w| w.tons_per_round(class.kind)),
                torpedo_t: weights.map_or(Fixed::ZERO, |w| w.tons_per_torpedo),
                rate,
                on_berth,
                hours_before,
            }
        })
        .collect();
    Some(AmmunitionWork { ships, base })
}
