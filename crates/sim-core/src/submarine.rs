//! The submarine's rules in play (decision 0218, stage 2): her passage to a
//! station or to home. The attack on forces in her hex, the hunt, the
//! claims and the reports come in the next phase; this one is the part that
//! needs only the boat and the map.
//!
//! The passage is worked at the very end of the movement step
//! ([`World::submarines_after_movement`], called from
//! `move_task_forces`), where the hours each force spent in each hex are in
//! hand (decision 0218, 37 and 38) for the attack.
//!
//! - A passage that ended (`arrived`) in an earlier pulse is cleared first
//!   thing: she is on station from the pulse after she arrives (rule 43).
//! - In the night pulse of her leave day, or in her first pulse on the map
//!   after it, a boat not already going home leaves for home (rule 3).
//! - A boat on passage steams her class's passage speed times the pulse's
//!   hours along the sea route a force would take: the same distance field,
//!   the same step and the same charge, entering a hex at half a leg
//!   (decisions 0017 and 0058). She attacks nothing on the way.
//! - At the end of a passage she is on station, or, going home, she is
//!   taken off the map at her base's hex.

use crate::bearing;
use crate::belief::{ApplyOutcome, BattleClaim, Report, SubmarineReport};
use crate::damage::{damage_per_hit, system_for_draw, system_hit_chance, DamageSite, HitSet};
use crate::events::{Breakdown, Cause, DamageTrack, EventKind, Factor, SubmarineLeftReason};
use crate::fixed::Fixed;
use crate::hex::{Hex, HEX_NM};
use crate::ids::{EntityRef, EventId, ShipId, SubmarineId, TaskForceId};
use crate::map::Map;
use crate::movement::HoursInHex;
use crate::pictures::{CommandRef, Holders};
use crate::refdata::ShipKind;
use crate::rng::{Draw, Subsystem};
use crate::search::{
    report_hex, reported_kind, sighting_chance, SubmarineSearch, DEGREES_PER_RADIAN,
};
use crate::state::{ShipSystem, SubmarineCondition, SubmarinePassage};
use crate::time::Pulse;
use crate::world::World;

/// The place of a kind in the order a boat fires at: carrier, battleship,
/// transport, auxiliary, cruiser (decision 0218, 40). A kind not in it is
/// not fired at.
fn target_rank(kind: ShipKind) -> Option<u8> {
    match kind {
        ShipKind::Carrier => Some(0),
        ShipKind::Battleship => Some(1),
        ShipKind::Transport => Some(2),
        ShipKind::Auxiliary => Some(3),
        ShipKind::Cruiser => Some(4),
        ShipKind::Destroyer => None,
    }
}

/// The kinds of ship that hunt a boat that fired on their force (decision
/// 0218, 9).
fn is_warship(kind: ShipKind) -> bool {
    matches!(
        kind,
        ShipKind::Carrier | ShipKind::Battleship | ShipKind::Cruiser | ShipKind::Destroyer
    )
}

/// The pulse `n` pulses after `(day, pulse)`.
fn pulses_after(day: u32, pulse: Pulse, n: u32) -> (u32, Pulse) {
    let index = (day - 1) * 3 + u32::try_from(pulse.code() - 1).expect("a pulse code") + n;
    (index / 3 + 1, Pulse::ALL[(index % 3) as usize])
}

/// A pulse as a number that orders pulses in time.
fn pulse_order(day: u32, pulse: Pulse) -> (u32, u64) {
    (day, pulse.code())
}

/// What a salvo came to, for what each side is told of it.
struct SalvoTold {
    real_hits: u32,
    missed: u32,
    sunk: u32,
    taken_for: ShipKind,
    false_hit_chance: Fixed,
}

/// What the attack chance for a pulse is (decision 0218, 39): one less the
/// chance that every hour fails, worked as one less the hourly chance
/// multiplied by itself for each whole hour, each product cut, and once
/// more by one less the hourly chance times the part of an hour left over.
/// Returns the chance, the miss, the whole hours and the part.
fn attack_chance(hourly: Fixed, hours: Fixed) -> (Fixed, Fixed, i64, Fixed) {
    let whole = hours.trunc();
    let part = hours - Fixed::from_int(whole);
    let fails = Fixed::ONE - hourly;
    let mut miss = Fixed::ONE;
    for _ in 0..whole {
        miss = miss * fails;
    }
    miss = miss * (Fixed::ONE - hourly * part);
    (Fixed::ONE - miss, miss, whole, part)
}

/// Steams a boat's passage up to `budget_nm` and returns the miles steamed.
/// `hex` is where she is (the hex she has entered); it changes when she
/// passes half a leg and at a centre. Stops at the end of the passage, or
/// where the sea gives no way on.
fn steam_passage(
    map: &Map,
    hex: &mut Hex,
    passage: &mut SubmarinePassage,
    budget_nm: Fixed,
) -> Fixed {
    let field = map.sea_distances_to(passage.to);
    let mut left_nm = budget_nm;
    let mut steamed_nm = Fixed::ZERO;
    loop {
        if passage.toward_hex.is_none() {
            // At a centre: she has arrived, or sets out on the next leg.
            if *hex == passage.to || left_nm.is_zero() {
                break;
            }
            let Some(next) = map.next_step_in(&field, *hex) else {
                break;
            };
            let charge = map.step_charge_nm(&field, *hex, next, passage.to);
            passage.leg_nm = Fixed::from_int(i64::from(charge));
            passage.toward_hex = Some(next);
            passage.progress_nm = Fixed::ZERO;
        }
        if left_nm.is_zero() {
            break;
        }
        let toward = passage.toward_hex.expect("she is on a leg");
        let half_leg = passage.leg_nm / Fixed::from_int(2);
        let to_centre_nm = passage.leg_nm - passage.progress_nm;
        let piece_nm = left_nm.min(to_centre_nm);
        if passage.progress_nm < half_leg && passage.progress_nm + piece_nm >= half_leg {
            *hex = toward;
        }
        left_nm -= piece_nm;
        steamed_nm += piece_nm;
        if piece_nm == to_centre_nm {
            *hex = toward;
            passage.toward_hex = None;
            passage.progress_nm = Fixed::ZERO;
            passage.leg_nm = Fixed::ZERO;
        } else {
            passage.progress_nm += piece_nm;
        }
    }
    steamed_nm
}

impl World {
    /// What the submarines do at the end of the movement step of a pulse,
    /// with `hours` the hours each force with a ship afloat spent in each
    /// hex in it (decision 0218, 37 and 38): first every boat's passage, then
    /// every boat's attack.
    pub(crate) fn submarines_after_movement(&mut self, day: u32, pulse: Pulse, hours: &HoursInHex) {
        let boats: Vec<SubmarineId> = self
            .truth
            .submarines
            .values()
            .filter(|boat| boat.on_map(day))
            .map(|boat| boat.id)
            .collect();
        for id in &boats {
            self.run_passage(day, pulse, *id);
        }
        for id in boats {
            self.submarine_attacks(day, pulse, id, hours);
        }
    }

    /// The kind of a ship.
    fn ship_kind(&self, ship: ShipId) -> ShipKind {
        self.ref_data.ship_classes[&self.truth.ships[&ship].class].kind
    }

    /// Whether a ship is dead in the water: its propulsion gives no speed,
    /// as the movement rules decide a ship makes none.
    fn dead_in_the_water(&self, ship: ShipId) -> bool {
        self.ref_data
            .surface_combat
            .system_factor(
                ShipSystem::Propulsion,
                self.truth.ships[&ship].systems.propulsion,
            )
            .is_zero()
    }

    /// One boat's attack in the pulse (decision 0218, 4 to 10, 37 to 42):
    /// the chance for each enemy force that spent hours in her hex, the
    /// salvo at the force that draws first by the order of kinds, and the
    /// hunt of her by the force fired on.
    fn submarine_attacks(&mut self, day: u32, pulse: Pulse, id: SubmarineId, hours: &HoursInHex) {
        let boat = &self.truth.submarines[&id];
        let able = boat.condition == SubmarineCondition::Sound
            && boat.on_station(day)
            && boat.salvoes > 0
            && boat.available_from.is_none_or(|from| from <= day)
            && boat
                .held_down_until
                .is_none_or(|(d, p)| pulse_order(d, p) < pulse_order(day, pulse));
        if !able {
            return;
        }
        let Some(data) = self.ref_data.submarines.as_ref() else {
            return;
        };
        let (side, hex, station_order, class_id) =
            (boat.side, boat.hex, boat.station_order, boat.class);
        let class = data.classes[&class_id].clone();
        let figures = data.figures.clone();
        let torpedo = data.torpedoes[&class.torpedo].clone();
        let hit_chance = figures.hit_chance[&class.torpedo];

        // The forces that spent hours in her hex, in number order, each with
        // its chance and its own draw.
        let mut drawn: Vec<(u8, TaskForceId, EventId)> = Vec::new();
        for (force_id, spent) in hours {
            let Some((_, spent_h)) = spent.iter().find(|(at, h)| *at == hex && *h > Fixed::ZERO)
            else {
                continue;
            };
            let force = &self.truth.task_forces[force_id];
            if force.side == side {
                continue;
            }
            let best = force
                .ships
                .iter()
                .filter_map(|ship| target_rank(self.ship_kind(*ship)))
                .min();
            let Some(best) = best else {
                continue;
            };
            // A force lying where a base of its own side is not attacked
            // (decision 0218, 41).
            if self
                .truth
                .bases
                .values()
                .any(|base| base.hex == hex && base.side == force.side && base.on_map)
            {
                continue;
            }
            let dead = force.ships.iter().any(|ship| self.dead_in_the_water(*ship));
            let multiple = if dead {
                figures.attacks_ship_dead_in_water
            } else {
                Fixed::ONE
            };
            let hourly = (figures.attacks_per_hour * multiple).min(Fixed::ONE);
            let (chance, miss, whole, part) = attack_chance(hourly, *spent_h);
            let breakdown = self.config.record_breakdowns.then(|| Breakdown {
                factors: vec![
                    Factor {
                        name: "attacks_per_hour",
                        value: figures.attacks_per_hour,
                    },
                    Factor {
                        name: "attacks_ship_dead_in_water",
                        value: multiple,
                    },
                    Factor {
                        name: "hours",
                        value: *spent_h,
                    },
                    Factor {
                        name: "whole_hours",
                        value: Fixed::from_int(whole),
                    },
                    Factor {
                        name: "part_hour",
                        value: part,
                    },
                    Factor {
                        name: "miss",
                        value: miss,
                    },
                ],
                result: chance,
            });
            let event = self.events.record(
                day,
                Some(pulse),
                EventKind::SubmarineAttackChance {
                    submarine: id,
                    task_force: *force_id,
                    hex,
                    hours: *spent_h,
                    chance,
                    attacks: hourly,
                },
                station_order.map(Cause::Order).into_iter().collect(),
                breakdown,
            );
            let attacks = self
                .rng_pair(
                    day,
                    Some(pulse),
                    Subsystem::Submarine,
                    Draw::SubmarineAttack,
                    EntityRef::Submarine(id),
                    EntityRef::TaskForce(*force_id),
                )
                .chance(chance);
            if attacks {
                drawn.push((best, *force_id, event));
            }
        }
        // Of the forces whose draw succeeded she attacks the one whose best
        // ship ranks first, then the lowest number (rule 6).
        let Some(&(_, force_id, chance_event)) =
            drawn.iter().min_by_key(|(rank, f, _)| (*rank, *f))
        else {
            return;
        };

        // The ship aimed at: first by the order of kinds, a ship dead in the
        // water before another of its kind, then the lowest number (rule 7).
        let ship_id = self.truth.task_forces[&force_id]
            .ships
            .iter()
            .filter_map(|ship| {
                target_rank(self.ship_kind(*ship))
                    .map(|rank| (rank, !self.dead_in_the_water(*ship), *ship))
            })
            .min()
            .map(|(_, _, ship)| ship)
            .expect("the force has a ship she may fire at");
        // Her side's claim is one stream of the boat and the ship (decision
        // 0218, 45): first the kind her side takes the ship for, through the
        // search figures' confusion row of its true kind, then one chance for each torpedo that
        // missed, drawn below.
        let search = self.ref_data.search.clone();
        let mut claim_stream = self.rng_pair(
            day,
            Some(pulse),
            Subsystem::Submarine,
            Draw::SubmarineClaim,
            EntityRef::Submarine(id),
            EntityRef::Ship(ship_id),
        );
        let true_kind = self.ship_kind(ship_id);
        let taken_for = reported_kind(
            true_kind,
            claim_stream.below(Fixed::SCALE.unsigned_abs()),
            &search,
        );

        // The salvo: one salvo of her class, each torpedo the next number of
        // the stream of the boat and the ship (rule 42).
        self.truth
            .submarines
            .get_mut(&id)
            .expect("the boat exists")
            .salvoes -= 1;
        let mut stream = self.rng_pair(
            day,
            Some(pulse),
            Subsystem::Submarine,
            Draw::SubmarineTorpedo,
            EntityRef::Submarine(id),
            EntityRef::Ship(ship_id),
        );
        let results: Vec<bool> = (0..class.salvo)
            .map(|_| stream.chance(hit_chance))
            .collect();
        let hits = u32::try_from(results.iter().filter(|hit| **hit).count()).expect("few hits");
        let ship_class = &self.ref_data.ship_classes[&self.truth.ships[&ship_id].class];
        let per_hit = damage_per_hit(torpedo.damage, ship_class.toughness);
        // A hit may also knock a system down, as a torpedo's hit does in a
        // surface fight: one draw for each hit, after the torpedoes'.
        let mut system_hits = Vec::new();
        let surface = &self.ref_data.surface_combat;
        let system_chance = system_hit_chance(per_hit, surface.system_hit_chance_per_point);
        for _ in 0..hits {
            let draw = stream.below(Fixed::SCALE.unsigned_abs());
            let draw = Fixed::from_raw(i64::try_from(draw).expect("a draw fits in i64"));
            system_hits.extend(system_for_draw(
                draw,
                system_chance,
                &surface.torpedo_system_hit,
            ));
        }
        let breakdown = self.config.record_breakdowns.then(|| {
            let mut factors = vec![Factor {
                name: "hit_chance",
                value: hit_chance,
            }];
            for hit in &results {
                factors.push(Factor {
                    name: "torpedo_hit",
                    value: if *hit { Fixed::ONE } else { Fixed::ZERO },
                });
            }
            Breakdown {
                factors,
                result: Fixed::from_int(i64::from(hits)),
            }
        });
        let salvo = self.events.record(
            day,
            Some(pulse),
            EventKind::SubmarineSalvo {
                submarine: id,
                task_force: force_id,
                ship: ship_id,
                hex,
                torpedoes: class.salvo,
                hits,
                taken_for,
            },
            vec![Cause::Event(chance_event)],
            breakdown,
        );
        let applied = self.apply_hit_set(
            DamageSite {
                day,
                pulse,
                round: 0,
            },
            &HitSet {
                cause: salvo,
                target: ship_id,
                target_force: force_id,
                damage_per_hit: per_hit,
                track_hits: {
                    let mut track_hits = [0u32; 2];
                    track_hits[DamageTrack::Flotation as usize] = hits;
                    track_hits
                },
                system_hits,
            },
        );
        self.submarine_tells(
            day,
            pulse,
            id,
            force_id,
            ship_id,
            hex,
            salvo,
            SalvoTold {
                real_hits: hits,
                missed: class.salvo - hits,
                sunk: u32::try_from(applied.sunk.len()).expect("few ships"),
                taken_for,
                false_hit_chance: figures.false_hit_chance,
            },
            &mut claim_stream,
            &search,
        );

        // The hunt (rules 9, 10, 22 and 53): a force with a warship afloat
        // that is not the ship aimed at hunts her, hit or miss.
        let hunts = self.truth.task_forces[&force_id]
            .ships
            .iter()
            .any(|ship| *ship != ship_id && is_warship(self.ship_kind(*ship)));
        if hunts {
            let until = pulses_after(day, pulse, figures.held_down_pulses);
            let mut stream = self.rng_pair(
                day,
                Some(pulse),
                Subsystem::Submarine,
                Draw::SubmarineHunt,
                EntityRef::Submarine(id),
                EntityRef::TaskForce(force_id),
            );
            let damaged = stream.chance(figures.hunt_damages_chance);
            let sunk = damaged && stream.chance(figures.hunt_sinks_share);
            let boat = self.truth.submarines.get_mut(&id).expect("the boat exists");
            boat.held_down_until = Some(until);
            let hunted = self.events.record(
                day,
                Some(pulse),
                EventKind::SubmarineHunted {
                    submarine: id,
                    task_force: force_id,
                    hex,
                    held_down_until: until,
                    damaged,
                    sunk,
                },
                vec![Cause::Event(salvo)],
                None,
            );
            if sunk {
                let boat = self.truth.submarines.get_mut(&id).expect("the boat exists");
                boat.condition = SubmarineCondition::Sunk;
                boat.passage = None;
                self.events.record(
                    day,
                    Some(pulse),
                    EventKind::SubmarineSunk { submarine: id },
                    vec![Cause::Event(hunted)],
                    None,
                );
                return;
            }
            if damaged {
                self.truth
                    .submarines
                    .get_mut(&id)
                    .expect("the boat exists")
                    .condition = SubmarineCondition::Damaged;
                self.send_home(
                    day,
                    pulse,
                    id,
                    SubmarineLeftReason::Damaged,
                    vec![Cause::Event(hunted)],
                );
            }
        }
        // With no salvo left she goes home as a damaged boat does.
        if self.truth.submarines[&id].salvoes == 0 {
            self.send_home(
                day,
                pulse,
                id,
                SubmarineLeftReason::NoSalvo,
                vec![Cause::Event(salvo)],
            );
        }
    }

    /// What each side is told of a salvo (decision 0218, 13, 14, 45 and
    /// 46), after its hits have landed. Her side gets her claim, made as a
    /// raid's claim is: the hits that landed and a chance of a false hit for
    /// each torpedo that missed, the kind she took her target for, the ships
    /// her hits sank; it is held by her side's highest command. The side
    /// fired on gets a report of a submarine at her hex through the
    /// report's error, held first by the command of the force fired on.
    #[allow(clippy::too_many_arguments)]
    fn submarine_tells(
        &mut self,
        day: u32,
        pulse: Pulse,
        id: SubmarineId,
        force_id: TaskForceId,
        ship_id: ShipId,
        hex: Hex,
        salvo: EventId,
        told: SalvoTold,
        claim_stream: &mut crate::rng::RngStream,
        search: &crate::refdata::SearchFigures,
    ) {
        let side = self.truth.submarines[&id].side;
        let fired_on = self.truth.task_forces[&force_id].side;
        let command_fired_on = self.command_of_force(force_id);

        // Her side's claim.
        let false_hits = u32::try_from(
            (0..told.missed)
                .filter(|_| claim_stream.chance(told.false_hit_chance))
                .count(),
        )
        .expect("few torpedoes");
        let claimed_hits = told.real_hits + false_hits;
        let claim = BattleClaim {
            hex,
            day,
            pulse,
            claimed_sunk: told.sunk,
            claimed_hits,
            claimed_kind: Some(told.taken_for),
            claimed_shot_down: 0,
            claimed_on_ground: 0,
            from_air: false,
            from_bombardment: false,
            from_submarine: true,
            received_event: self.events.next_id(),
            ground_types: Vec::new(),
            by_unit: Vec::new(),
            air_types: Vec::new(),
        };
        let holders = Holders::Commands {
            commands: vec![CommandRef::Area],
            key: claim.received_event,
            day,
            pulse: Some(pulse),
        };
        self.apply_report(side, &Report::BattleClaim(claim.clone()), &holders, search);
        let breakdown = self.config.record_breakdowns.then(|| Breakdown {
            factors: vec![
                Factor {
                    name: "real_hits",
                    value: Fixed::from_int(i64::from(told.real_hits)),
                },
                Factor {
                    name: "torpedoes_missed",
                    value: Fixed::from_int(i64::from(told.missed)),
                },
                Factor {
                    name: "false_hit_chance",
                    value: told.false_hit_chance,
                },
                Factor {
                    name: "false_hits",
                    value: Fixed::from_int(i64::from(false_hits)),
                },
            ],
            result: Fixed::from_int(i64::from(claimed_hits)),
        });
        let received = self.events.record(
            day,
            Some(pulse),
            EventKind::ClaimReceived {
                side,
                hex,
                claimed_sunk: told.sunk,
                claimed_hits,
                claimed_kind: Some(told.taken_for),
                claimed_shot_down: 0,
                claimed_on_ground: 0,
                from_air: false,
                from_bombardment: false,
                from_submarine: true,
                ground_types: Vec::new(),
                by_unit: Vec::new(),
                air_types: Vec::new(),
            },
            vec![Cause::Event(salvo)],
            breakdown,
        );
        assert_eq!(
            received, claim.received_event,
            "the claim cites the event that records it"
        );
        self.pass_report(side, &Report::BattleClaim(claim), &holders);

        // The side fired on: a report of a submarine at her hex through the
        // report's error (one stream of the boat and the ship; the count
        // and the kind are not drawn).
        let mut stream = self.rng_pair(
            day,
            Some(pulse),
            Subsystem::Submarine,
            Draw::SubmarineReportError,
            EntityRef::Submarine(id),
            EntityRef::Ship(ship_id),
        );
        let (struck, direction) = if stream.chance(search.position_error_chance) {
            (
                Fixed::ONE,
                Some(usize::try_from(stream.below(6)).expect("below 6")),
            )
        } else {
            (Fixed::ZERO, None)
        };
        let reported_hex = report_hex(&self.map, hex, direction);
        let report = SubmarineReport {
            hex: reported_hex,
            day,
            pulse: Some(pulse),
            received_event: self.events.next_id(),
            age_pulses: 0,
        };
        let holders = Holders::Commands {
            commands: vec![command_fired_on],
            key: report.received_event,
            day,
            pulse: Some(pulse),
        };
        let ApplyOutcome::SubmarineReport { replaced } =
            self.apply_report(fired_on, &Report::Submarine(report), &holders, search)
        else {
            unreachable!("a report of a submarine is taken in as one");
        };
        let breakdown = self.config.record_breakdowns.then(|| Breakdown {
            factors: vec![
                Factor {
                    name: "position_error_chance",
                    value: search.position_error_chance,
                },
                Factor {
                    name: "position_error_struck",
                    value: struck,
                },
            ],
            result: Fixed::ZERO,
        });
        let received = self.events.record(
            day,
            Some(pulse),
            EventKind::SubmarineReported {
                side: fired_on,
                hex: reported_hex,
                replaced,
            },
            vec![Cause::Event(salvo)],
            breakdown,
        );
        assert_eq!(
            received, report.received_event,
            "the report cites the event that records it"
        );
        self.pass_report(fired_on, &Report::Submarine(report), &holders);
    }

    /// An air group's look for boats in its arc (decision 0218, 17 to 20 and
    /// 48 to 52), after its look for forces: for each boat on the map in
    /// number order whose hex is in the arc and the range, the chance of
    /// sighting her is the chance of sighting a force at the same distance
    /// (taken as at least one hex) times her surfaced share when she is on
    /// station, whole when she is on passage. Sighted, she is reported to
    /// the group's side through the report's error, held down for the rest
    /// of the pulse and the next, and attacked by the aircraft: damaged under
    /// the air attack's chance, then sunk under its share. No aircraft is
    /// lost, no bomb used and no sortie added.
    pub(crate) fn search_for_submarines(
        &mut self,
        day: u32,
        pulse: Pulse,
        search_flight: &SubmarineSearch,
        search: &crate::refdata::SearchFigures,
    ) {
        if self.truth.submarines.is_empty() {
            return;
        }
        let Some(figures) = self
            .ref_data
            .submarines
            .as_ref()
            .map(|data| data.figures.clone())
        else {
            return;
        };
        let flight = search_flight;
        let boats: Vec<SubmarineId> = self
            .truth
            .submarines
            .values()
            .filter(|boat| boat.side != flight.side && boat.on_map(day))
            .map(|boat| boat.id)
            .collect();
        for id in boats {
            let boat = &self.truth.submarines[&id];
            let hex = boat.hex;
            if !bearing::in_arc(flight.origin, hex, flight.arc.from_deg, flight.arc.to_deg) {
                continue;
            }
            // At least one hex away, so that a boat in the group's own hex
            // is not sighted for certain (rule 48).
            let distance = i64::from(flight.origin.air_distance_nm(hex)).max(HEX_NM);
            let distance_nm = Fixed::from_int(distance);
            if distance_nm > flight.arc.range_nm {
                continue;
            }
            let on_station = boat.on_station(day);
            let share = if on_station {
                figures.surfaced_share
            } else {
                Fixed::ONE
            };
            let of_a_force = sighting_chance(
                flight.aircraft,
                flight.sweep_width_nm,
                flight.arc_deg,
                distance_nm,
            );
            let chance = of_a_force * share;
            let mut stream = self.rng_pair(
                day,
                Some(pulse),
                Subsystem::Submarine,
                Draw::SubmarineSighted,
                EntityRef::Submarine(id),
                EntityRef::AirGroup(flight.group),
            );
            let sighted = stream.chance(chance);
            let breakdown = self.config.record_breakdowns.then(|| Breakdown {
                factors: vec![
                    Factor {
                        name: "degrees_per_radian",
                        value: DEGREES_PER_RADIAN,
                    },
                    Factor {
                        name: "aircraft",
                        value: Fixed::from_int(i64::from(flight.aircraft)),
                    },
                    Factor {
                        name: "sweep_width_nm",
                        value: flight.sweep_width_nm,
                    },
                    Factor {
                        name: "arc_deg",
                        value: Fixed::from_int(i64::from(flight.arc_deg)),
                    },
                    Factor {
                        name: "distance_nm",
                        value: distance_nm,
                    },
                    Factor {
                        name: "chance_of_a_force",
                        value: of_a_force,
                    },
                    Factor {
                        name: "surfaced_share",
                        value: share,
                    },
                ],
                result: chance,
            });
            let attempt = self.events.record(
                day,
                Some(pulse),
                EventKind::SubmarineSearchAttempt {
                    air_group: flight.group,
                    submarine: id,
                    distance_nm,
                    chance,
                    sighted,
                },
                vec![Cause::Event(flight.flown)],
                breakdown,
            );
            if !sighted {
                continue;
            }

            // Reported to the group's side at her hex through the report's
            // error, held first by the group's command (statement 19).
            let mut stream = self.rng_pair(
                day,
                Some(pulse),
                Subsystem::Submarine,
                Draw::SubmarineReportError,
                EntityRef::Submarine(id),
                EntityRef::AirGroup(flight.group),
            );
            let (struck, direction) = if stream.chance(search.position_error_chance) {
                (
                    Fixed::ONE,
                    Some(usize::try_from(stream.below(6)).expect("below 6")),
                )
            } else {
                (Fixed::ZERO, None)
            };
            let reported_hex = report_hex(&self.map, hex, direction);
            let report = SubmarineReport {
                hex: reported_hex,
                day,
                pulse: Some(pulse),
                received_event: self.events.next_id(),
                age_pulses: 0,
            };
            let holders = Holders::Commands {
                commands: vec![self.command_of_group(flight.group)],
                key: report.received_event,
                day,
                pulse: Some(pulse),
            };
            let ApplyOutcome::SubmarineReport { replaced } =
                self.apply_report(flight.side, &Report::Submarine(report), &holders, search)
            else {
                unreachable!("a report of a submarine is taken in as one");
            };
            let breakdown = self.config.record_breakdowns.then(|| Breakdown {
                factors: vec![
                    Factor {
                        name: "position_error_chance",
                        value: search.position_error_chance,
                    },
                    Factor {
                        name: "position_error_struck",
                        value: struck,
                    },
                ],
                result: Fixed::ZERO,
            });
            let received = self.events.record(
                day,
                Some(pulse),
                EventKind::SubmarineReported {
                    side: flight.side,
                    hex: reported_hex,
                    replaced,
                },
                vec![Cause::Event(attempt)],
                breakdown,
            );
            assert_eq!(
                received, report.received_event,
                "the report cites the event that records it"
            );
            self.pass_report(flight.side, &Report::Submarine(report), &holders);

            // She is held down for the rest of the pulse and the next (rule
            // 20): the later of what it was and the next pulse.
            let next = pulses_after(day, pulse, 1);
            let boat = self.truth.submarines.get_mut(&id).expect("the boat exists");
            if boat
                .held_down_until
                .is_none_or(|(d, p)| pulse_order(d, p) < pulse_order(next.0, next.1))
            {
                boat.held_down_until = Some(next);
            }

            // The aircraft attack her in the same pulse (rules 20, 50, 51).
            let mut stream = self.rng_pair(
                day,
                Some(pulse),
                Subsystem::Submarine,
                Draw::SubmarineAirAttack,
                EntityRef::Submarine(id),
                EntityRef::AirGroup(flight.group),
            );
            let damaged = stream.chance(figures.air_attack_damages_chance);
            let sunk = damaged && stream.chance(figures.air_attack_sinks_share);
            let breakdown = self.config.record_breakdowns.then(|| Breakdown {
                factors: vec![
                    Factor {
                        name: "air_attack_damages_chance",
                        value: figures.air_attack_damages_chance,
                    },
                    Factor {
                        name: "air_attack_sinks_share",
                        value: figures.air_attack_sinks_share,
                    },
                ],
                result: Fixed::ZERO,
            });
            let attacked = self.events.record(
                day,
                Some(pulse),
                EventKind::SubmarineAttackedFromAir {
                    air_group: flight.group,
                    submarine: id,
                    hex,
                    damaged,
                    sunk,
                },
                vec![Cause::Event(attempt)],
                breakdown,
            );
            if sunk {
                let boat = self.truth.submarines.get_mut(&id).expect("the boat exists");
                boat.condition = SubmarineCondition::Sunk;
                boat.passage = None;
                self.events.record(
                    day,
                    Some(pulse),
                    EventKind::SubmarineSunk { submarine: id },
                    vec![Cause::Event(attacked)],
                    None,
                );
            } else if damaged {
                self.truth
                    .submarines
                    .get_mut(&id)
                    .expect("the boat exists")
                    .condition = SubmarineCondition::Damaged;
                self.send_home(
                    day,
                    pulse,
                    id,
                    SubmarineLeftReason::Damaged,
                    vec![Cause::Event(attacked)],
                );
            }
        }
    }

    /// Sends a boat for home (decision 0218, 3, 21 and 43): a passage to her
    /// home base's hex, with `SubmarineLeftForHome` for `reason` citing
    /// `causes`. A leg she is on is finished first, as a force finishes its
    /// leg when its route changes. Nothing if she is already going home.
    /// Returns the event.
    pub(crate) fn send_home(
        &mut self,
        day: u32,
        pulse: Pulse,
        boat: SubmarineId,
        reason: SubmarineLeftReason,
        causes: Vec<Cause>,
    ) -> Option<EventId> {
        let home = self.truth.submarines.get(&boat)?.home;
        let base = self.truth.bases.get(&home)?.hex;
        let boat_state = self.truth.submarines.get_mut(&boat)?;
        if boat_state.passage.is_some_and(|passage| passage.home) {
            return None;
        }
        let (toward_hex, progress_nm, leg_nm) = match boat_state.passage {
            Some(passage) if passage.toward_hex.is_some() => {
                (passage.toward_hex, passage.progress_nm, passage.leg_nm)
            }
            _ => (None, Fixed::ZERO, Fixed::ZERO),
        };
        boat_state.passage = Some(SubmarinePassage {
            to: base,
            home: true,
            toward_hex,
            progress_nm,
            leg_nm,
            arrived: false,
            order: None,
        });
        Some(self.events.record(
            day,
            Some(pulse),
            EventKind::SubmarineLeftForHome {
                submarine: boat,
                base: home,
                reason,
            },
            causes,
            None,
        ))
    }

    /// One boat's part of the pulse: the end of an earlier arrival, the
    /// leave day, and the passage.
    fn run_passage(&mut self, day: u32, pulse: Pulse, id: SubmarineId) {
        let boat = self.truth.submarines.get_mut(&id).expect("the boat exists");
        if boat.passage.is_some_and(|passage| passage.arrived) {
            boat.passage = None;
        }
        let going_home = boat.passage.is_some_and(|passage| passage.home);
        let leaves = boat
            .leave_day
            .is_some_and(|leave| day > leave || (day == leave && pulse == Pulse::Night));
        if leaves && !going_home {
            self.send_home(day, pulse, id, SubmarineLeftReason::LeaveDay, Vec::new());
        }

        let boat = &self.truth.submarines[&id];
        let Some(passage) = boat.passage else {
            return;
        };
        let Some(class) = self
            .ref_data
            .submarines
            .as_ref()
            .and_then(|data| data.classes.get(&boat.class))
        else {
            return;
        };
        let budget_nm = class.passage_speed_kn * Fixed::from_int(pulse.hours());
        let (side_home, from) = (boat.home, boat.hex);
        let mut hex = from;
        let mut passage = passage;
        let steamed_nm = steam_passage(&self.map, &mut hex, &mut passage, budget_nm);
        let reached = passage.toward_hex.is_none() && hex == passage.to;

        // Her steaming cites the order that sent her, or, going home, what
        // sent her.
        let mut causes: Vec<Cause> = passage.order.map(Cause::Order).into_iter().collect();
        if causes.is_empty() && passage.home {
            let sent = self
                .events
                .iter()
                .filter(|event| {
                    matches!(
                        event.kind,
                        EventKind::SubmarineLeftForHome { submarine, .. } if submarine == id
                    )
                })
                .map(|event| event.id)
                .last();
            causes.extend(sent.map(Cause::Event));
        }
        let steamed = (!steamed_nm.is_zero()).then(|| {
            self.events.record(
                day,
                Some(pulse),
                EventKind::SubmarineSteamed {
                    submarine: id,
                    from,
                    to: hex,
                },
                causes.clone(),
                None,
            )
        });
        let causes = steamed.map_or(causes, |event| vec![Cause::Event(event)]);

        let boat = self.truth.submarines.get_mut(&id).expect("the boat exists");
        boat.hex = hex;
        boat.passage = Some(passage);
        if !reached {
            return;
        }
        if passage.home {
            boat.condition = SubmarineCondition::Home;
            boat.passage = None;
            self.events.record(
                day,
                Some(pulse),
                EventKind::SubmarineReachedHome {
                    submarine: id,
                    base: side_home,
                },
                causes,
                None,
            );
        } else {
            boat.passage = Some(SubmarinePassage {
                arrived: true,
                ..passage
            });
            self.events.record(
                day,
                Some(pulse),
                EventKind::SubmarineOnStation { submarine: id, hex },
                causes,
                None,
            );
        }
    }
}
