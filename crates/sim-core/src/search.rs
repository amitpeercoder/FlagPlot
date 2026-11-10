//! Air search: air groups look for enemy task forces in their arcs, sightings
//! become reports, and a sighted task force may notice the aircraft.
//!
//! This is the `AirSearch` phase of a pulse. The rules are decisions 1 to 11
//! and 16 to 19 of `docs/decisions/0012-search-and-contacts.md`, and the
//! random number keys are those of
//! `docs/decisions/0013-random-keys-for-pairs.md`.
//!
//! Search writes nothing in truth except the hour a task force last noticed
//! a sighting. It reads truth (the enemy's position, ship count and kinds)
//! only to make a report, and the report goes into belief through
//! [`BeliefState::apply_report`](crate::belief::BeliefState::apply_report).

use crate::bearing;
use crate::belief::{ApplyOutcome, Report, SightingReport};
use crate::events::{Breakdown, Cause, EventKind, Factor};
use crate::fixed::Fixed;
use crate::hex::{Hex, HEX_NM};
use crate::ids::{EntityRef, EventId, TaskForceId};
use crate::map::Map;
use crate::orders::AirMission;
use crate::refdata::{SearchFigures, ShipKind};
use crate::rng::{RngStream, Subsystem};
use crate::state::Side;
use crate::time::{self, Pulse};
use crate::world::World;

/// Degrees in one radian, 57.295779 to six decimals (decision 0012,
/// decision 5). It turns "miles of sea swept over miles of arc" into a
/// number that can be worked in degrees.
const DEGREES_PER_RADIAN: Fixed = Fixed::from_raw(57_295_779);

/// The chance that `aircraft` aircraft with sweep width `sweep_width_nm`
/// sight a task force `distance_nm` away, in an arc `arc_deg` degrees wide
/// (decision 0012, decision 5):
///
/// `(57.295779 x aircraft x sweep) / (arc x distance)`, capped at 1.
///
/// The numerator is multiplied out first and divided once, truncating to six
/// decimals. A target at distance zero is always sighted.
pub(crate) fn sighting_chance(
    aircraft: u32,
    sweep_width_nm: Fixed,
    arc_deg: u32,
    distance_nm: Fixed,
) -> Fixed {
    if distance_nm.is_zero() {
        return Fixed::ONE;
    }
    let numerator = DEGREES_PER_RADIAN * i64::from(aircraft) * sweep_width_nm;
    let denominator = distance_nm * i64::from(arc_deg);
    (numerator / denominator).min(Fixed::ONE)
}

/// The hex a report names: the neighbour in `direction` (an index into
/// [`Hex::DIRECTIONS`]) if the position error struck and that neighbour is
/// on the map and not land, otherwise the true hex (decision 0012,
/// decision 10).
pub(crate) fn report_hex(map: &Map, true_hex: Hex, direction: Option<usize>) -> Hex {
    let Some(index) = direction else {
        return true_hex;
    };
    let (dq, dr) = Hex::DIRECTIONS[index];
    let neighbour = Hex::new(true_hex.q + dq, true_hex.r + dr);
    if map.is_navigable(neighbour) {
        neighbour
    } else {
        true_hex
    }
}

/// A raw chance as an unsigned number to compare a draw against.
fn raw_u64(value: Fixed) -> u64 {
    u64::try_from(value.raw()).expect("a chance is not negative")
}

/// The ship count a report claims, from one draw `v` in `0..1_000_000`:
/// one too few below the under chance (never below 1), one too many up to
/// the under and over chances together, otherwise the truth.
pub(crate) fn reported_count(true_count: u32, v: u64, figures: &SearchFigures) -> u32 {
    let under = raw_u64(figures.count_under_chance);
    let over = raw_u64(figures.count_over_chance);
    if v < under {
        true_count.saturating_sub(1).max(1)
    } else if v < under + over {
        true_count + 1
    } else {
        true_count
    }
}

/// The kind a report claims, from one draw `v` in `0..1_000_000`: walk the
/// kinds in importance order, adding up the chance of each in the true
/// kind's confusion row, and take the first whose running total passes `v`.
pub(crate) fn reported_kind(true_kind: ShipKind, v: u64, figures: &SearchFigures) -> ShipKind {
    let row = figures
        .kind_confusion
        .get(&true_kind)
        .expect("every kind has a confusion row");
    let mut cumulative = 0u64;
    for kind in ShipKind::ALL {
        cumulative += row.get(&kind).map_or(0, |chance| raw_u64(*chance));
        if v < cumulative {
            return kind;
        }
    }
    unreachable!("a confusion row adds to exactly 1, so a draw below 1 lands in it")
}

/// Draws the rest of a sighting report from `stream`, in a fixed order:
/// position error, then count, then kind (decision 0012). Air search and
/// surface combat both build their reports here, so a report means the same
/// thing whoever made it. `received_event` is the event that will record the
/// report's arrival: the caller passes the log's next ID.
pub(crate) fn build_sighting_report(
    stream: &mut RngStream,
    map: &Map,
    true_hex: Hex,
    true_count: u32,
    true_kind: ShipKind,
    figures: &SearchFigures,
    received_event: EventId,
) -> SightingReport {
    let direction = if stream.chance(figures.position_error_chance) {
        Some(usize::try_from(stream.below(6)).expect("below 6"))
    } else {
        None
    };
    let estimated_hex = report_hex(map, true_hex, direction);
    let estimated_ship_count = reported_count(
        true_count,
        stream.below(Fixed::SCALE.unsigned_abs()),
        figures,
    );
    let estimated_kind = reported_kind(
        true_kind,
        stream.below(Fixed::SCALE.unsigned_abs()),
        figures,
    );
    SightingReport {
        estimated_hex,
        estimated_ship_count,
        estimated_kind,
        confidence: figures.initial_confidence,
        received_event,
    }
}

/// What search needs to know about a task force that can be sighted.
struct Target {
    id: TaskForceId,
    side: Side,
    position: Hex,
    ship_count: u32,
    /// The most important kind aboard.
    kind: ShipKind,
}

impl World {
    /// The air search phase of one daylight pulse.
    pub(crate) fn search_from_the_air(&mut self, day: u32, pulse: Pulse) {
        let Some(figures) = self.ref_data.search.clone() else {
            // Without search figures there are no air groups (World::new).
            return;
        };

        // Search runs before movement, so these are the positions at the
        // start of the pulse. A task force with no ships has nothing to see.
        let targets: Vec<Target> = self
            .truth
            .task_forces
            .values()
            .filter_map(|task_force| {
                // `ShipKind` is declared in order of importance, carrier
                // first, so the smallest kind is the most important one
                // (decision 0012, decision 9).
                let kind = task_force
                    .ships
                    .iter()
                    .map(|ship| {
                        let class = self.truth.ships[ship].class;
                        self.ref_data.ship_classes[&class].kind
                    })
                    .min()?;
                Some(Target {
                    id: task_force.id,
                    side: task_force.side,
                    position: task_force.position(),
                    ship_count: u32::try_from(task_force.ships.len()).expect("few ships"),
                    kind,
                })
            })
            .collect();

        let groups: Vec<_> = self.truth.air_groups.values().cloned().collect();
        for group in groups {
            if group.mission != AirMission::Search || group.ready == 0 {
                continue;
            }
            let Some(arc) = group.search else {
                continue;
            };
            let base_hex = self.truth.bases[&group.base].hex;
            if time::is_dark(base_hex, pulse) {
                continue;
            }
            let sweep_width_nm = self.ref_data.aircraft_types[&group.aircraft_type].sweep_width_nm;

            let causes: Vec<Cause> = [group.mission_order, group.arc_order]
                .into_iter()
                .flatten()
                .map(Cause::Order)
                .collect();
            let flown = self.events.record(
                day,
                Some(pulse),
                EventKind::SearchFlown {
                    air_group: group.id,
                    base: group.base,
                    from_deg: arc.from_deg,
                    to_deg: arc.to_deg,
                    range_nm: arc.range_nm,
                    aircraft: group.ready,
                },
                causes,
                None,
            );

            let arc_deg = bearing::arc_degrees(arc.from_deg, arc.to_deg);
            for target in &targets {
                if target.side == group.side
                    || !bearing::in_arc(base_hex, target.position, arc.from_deg, arc.to_deg)
                {
                    continue;
                }
                let distance_nm =
                    Fixed::from_int(i64::from(base_hex.distance(target.position)) * HEX_NM);
                if distance_nm > arc.range_nm {
                    continue;
                }

                let chance = sighting_chance(group.ready, sweep_width_nm, arc_deg, distance_nm);
                let mut stream = self.rng_pair(
                    day,
                    Some(pulse),
                    Subsystem::Search,
                    EntityRef::AirGroup(group.id),
                    EntityRef::TaskForce(target.id),
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
                            value: Fixed::from_int(i64::from(group.ready)),
                        },
                        Factor {
                            name: "sweep_width_nm",
                            value: sweep_width_nm,
                        },
                        Factor {
                            name: "arc_deg",
                            value: Fixed::from_int(i64::from(arc_deg)),
                        },
                        Factor {
                            name: "distance_nm",
                            value: distance_nm,
                        },
                    ],
                    result: chance,
                });
                let attempt = self.events.record(
                    day,
                    Some(pulse),
                    EventKind::SearchAttempt {
                        air_group: group.id,
                        task_force: target.id,
                        distance_nm,
                        chance,
                        sighted,
                    },
                    vec![Cause::Event(flown)],
                    breakdown,
                );
                if !sighted {
                    continue;
                }

                // The rest of the report is drawn from the same stream.
                let report = build_sighting_report(
                    &mut stream,
                    &self.map,
                    target.position,
                    target.ship_count,
                    target.kind,
                    &figures,
                    self.events.next_id(),
                );
                let estimated_hex = report.estimated_hex;
                let estimated_ship_count = report.estimated_ship_count;
                let estimated_kind = report.estimated_kind;

                let ApplyOutcome::Contact(outcome) = self
                    .beliefs
                    .get_mut(&group.side)
                    .expect("both sides have a belief state")
                    .apply_report(&Report::Sighting(report.clone()), &figures)
                else {
                    unreachable!("a sighting report makes or refreshes a contact");
                };
                let received = self.events.record(
                    day,
                    Some(pulse),
                    EventKind::ReportReceived {
                        side: group.side,
                        contact: outcome.contact,
                        hex: estimated_hex,
                        ship_count: estimated_ship_count,
                        kind: estimated_kind,
                        confidence: report.confidence,
                        created: outcome.created,
                    },
                    vec![Cause::Event(attempt)],
                    None,
                );
                assert_eq!(
                    received, report.received_event,
                    "the report cites the event that records it"
                );

                // Decisions 16 to 19: the sighted force may notice, on a
                // stream of its own.
                let mut notice = self.rng_pair(
                    day,
                    Some(pulse),
                    Subsystem::Search,
                    EntityRef::TaskForce(target.id),
                    EntityRef::AirGroup(group.id),
                );
                if notice.chance(figures.notice_chance) {
                    let elapsed_hours = time::pulse_start(day, pulse);
                    self.truth
                        .task_forces
                        .get_mut(&target.id)
                        .expect("target exists")
                        .noticed_sighted_at_hours = Some(elapsed_hours);
                    self.events.record(
                        day,
                        Some(pulse),
                        EventKind::SightingNoticed {
                            task_force: target.id,
                            air_group: group.id,
                            elapsed_hours,
                        },
                        vec![Cause::Event(attempt)],
                        None,
                    );
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::events::Event;
    use crate::ids::{AirGroupId, AircraftTypeId, BaseId, EventId, OrderId, ShipClassId, ShipId};
    use crate::map::{HexData, HexKind};
    use crate::orders::{AirGroupOrder, Mission, OrderKind, OrderRequest, Origin};
    use crate::refdata::{AircraftRole, AircraftType, PortSize, RefData, ShipClass};
    use crate::state::{AirGroup, Base, SearchArc, Ship, ShipLocation, TaskForce, TruthState};
    use crate::world::SimConfig;

    const GROUP: AirGroupId = AirGroupId::new(1);
    const BASE_HEX: Hex = Hex::new(0, 0);

    fn fixed(num: i64, den: i64) -> Fixed {
        Fixed::from_ratio(num, den)
    }

    // ---- sighting_chance ----

    fn chance(aircraft: u32, arc_deg: u32, distance_nm: i64) -> Fixed {
        sighting_chance(
            aircraft,
            Fixed::from_int(40),
            arc_deg,
            Fixed::from_int(distance_nm),
        )
    }

    #[test]
    fn the_hand_calculation_chances_come_out_exactly() {
        // 4 aircraft, 40 nm sweep, 60 degree arc. The numerator is
        // 57.295779 x 4 x 40 = 9167.32464, and the denominator is 60 x
        // distance. Each quotient truncates to six decimals.
        // 600 nm: 9167.32464 / 36000 = 0.254647(9) -> 0.254647
        assert_eq!(chance(4, 60, 600), Fixed::from_raw(254_647));
        // 320 nm: 9167.32464 / 19200 = 0.477464(8) -> 0.477464
        assert_eq!(chance(4, 60, 320), Fixed::from_raw(477_464));
        // 200 nm: 9167.32464 / 12000 = 0.763943(7) -> 0.763943
        assert_eq!(chance(4, 60, 200), Fixed::from_raw(763_943));
        // 160 nm: 9167.32464 / 9600 = 0.954929(2) -> 0.954929
        assert_eq!(chance(4, 60, 160), Fixed::from_raw(954_929));
    }

    #[test]
    fn more_aircraft_raise_the_chance_and_it_is_capped_at_one() {
        // 320 nm, 60 degrees, so the denominator is 19200.
        // 1 aircraft: 57.295779 x 40 = 2291.83116 / 19200 = 0.119366(2)
        assert_eq!(chance(1, 60, 320), Fixed::from_raw(119_366));
        // 2 aircraft: 4583.66232 / 19200 = 0.238732(4)
        assert_eq!(chance(2, 60, 320), Fixed::from_raw(238_732));
        // 8 aircraft: 18334.64928 / 19200 = 0.954929(2)
        assert_eq!(chance(8, 60, 320), Fixed::from_raw(954_929));
        // 16 aircraft: 36669.29856 / 19200 = 1.9 -> capped at 1
        assert_eq!(chance(16, 60, 320), Fixed::ONE);
    }

    #[test]
    fn a_wider_arc_lowers_the_chance() {
        // 4 aircraft, 320 nm, numerator 9167.32464.
        // 30 degrees: / 9600 = 0.954929(2)
        assert_eq!(chance(4, 30, 320), Fixed::from_raw(954_929));
        // 120 degrees: / 38400 = 0.238732(4)
        assert_eq!(chance(4, 120, 320), Fixed::from_raw(238_732));
        // 360 degrees: / 115200 = 0.079577(4)
        assert_eq!(chance(4, 360, 320), Fixed::from_raw(79_577));
    }

    #[test]
    fn a_target_in_the_bases_own_hex_is_always_sighted() {
        assert_eq!(chance(4, 60, 0), Fixed::ONE);
        assert_eq!(chance(1, 360, 0), Fixed::ONE);
    }

    // ---- report pieces ----

    fn sea_map_with_land_and_edge() -> Map {
        let mut map = Map::new();
        for q in -2..=2 {
            for r in -2..=2 {
                map.insert(Hex::new(q, r), HexData::open_sea());
            }
        }
        map.insert(
            Hex::new(1, 0),
            HexData {
                kind: HexKind::Land,
                ..HexData::open_sea()
            },
        );
        map
    }

    #[test]
    fn a_report_hex_falls_back_to_the_truth_on_land_or_off_the_map() {
        let map = sea_map_with_land_and_edge();
        let centre = Hex::new(0, 0);
        // No error: the true hex.
        assert_eq!(report_hex(&map, centre, None), centre);
        // Direction 0 is (+1, 0): land, so the true hex.
        assert_eq!(report_hex(&map, centre, Some(0)), centre);
        // Direction 3 is (-1, 0): open sea, so the neighbour.
        assert_eq!(report_hex(&map, centre, Some(3)), Hex::new(-1, 0));
        // From the map's edge at (2,0), direction 0 is (3,0): off the map,
        // so the true hex. Direction 3 is (1,0): land, so the true hex too.
        let edge = Hex::new(2, 0);
        assert_eq!(report_hex(&map, edge, Some(0)), edge);
        assert_eq!(report_hex(&map, edge, Some(3)), edge);
        // Direction 5 is (0,+1): open sea.
        assert_eq!(report_hex(&map, edge, Some(5)), Hex::new(2, 1));
        // Every direction from the centre is the true hex or a sea neighbour.
        for index in 0..6 {
            let hex = report_hex(&map, centre, Some(index));
            assert!(hex == centre || (centre.distance(hex) == 1 && map.is_navigable(hex)));
        }
    }

    #[test]
    fn a_reported_count_follows_the_bands_and_never_drops_below_one() {
        // The figures: under 0.2 (raw 200000), over 0.2, so the bands are
        // v < 200000 under, v < 400000 over, otherwise the truth.
        let figures = SearchFigures::from_search_toml();
        // True count 1: under would be 0, so it stays 1.
        assert_eq!(reported_count(1, 0, &figures), 1);
        assert_eq!(reported_count(1, 199_999, &figures), 1);
        assert_eq!(reported_count(1, 200_000, &figures), 2);
        assert_eq!(reported_count(1, 399_999, &figures), 2);
        assert_eq!(reported_count(1, 400_000, &figures), 1);
        // True count 3: 2 below 200000, 4 up to 399999, 3 from 400000.
        assert_eq!(reported_count(3, 0, &figures), 2);
        assert_eq!(reported_count(3, 199_999, &figures), 2);
        assert_eq!(reported_count(3, 200_000, &figures), 4);
        assert_eq!(reported_count(3, 399_999, &figures), 4);
        assert_eq!(reported_count(3, 400_000, &figures), 3);
        assert_eq!(reported_count(3, 999_999, &figures), 3);
    }

    #[test]
    fn a_reported_kind_follows_the_confusion_row() {
        let figures = SearchFigures::from_search_toml();
        // Cruiser row in importance order: carrier 0.1, battleship 0.1,
        // cruiser 0.7, destroyer 0.1. Running totals 100000, 200000, 900000,
        // 1000000.
        let cruiser = |v| reported_kind(ShipKind::Cruiser, v, &figures);
        assert_eq!(cruiser(0), ShipKind::Carrier);
        assert_eq!(cruiser(99_999), ShipKind::Carrier);
        assert_eq!(cruiser(100_000), ShipKind::Battleship);
        assert_eq!(cruiser(199_999), ShipKind::Battleship);
        assert_eq!(cruiser(200_000), ShipKind::Cruiser);
        assert_eq!(cruiser(899_999), ShipKind::Cruiser);
        assert_eq!(cruiser(900_000), ShipKind::Destroyer);
        assert_eq!(cruiser(999_999), ShipKind::Destroyer);
        // Carrier row: carrier 0.7, battleship 0.1, cruiser 0.1, transport
        // 0.1 (destroyer has no entry and counts 0).
        let carrier = |v| reported_kind(ShipKind::Carrier, v, &figures);
        assert_eq!(carrier(699_999), ShipKind::Carrier);
        assert_eq!(carrier(700_000), ShipKind::Battleship);
        assert_eq!(carrier(800_000), ShipKind::Cruiser);
        assert_eq!(carrier(900_000), ShipKind::Transport);
        assert_eq!(carrier(999_999), ShipKind::Transport);
    }

    // ---- a world to search in ----

    /// A task force to place: ID, side, hex and the ship classes aboard.
    /// Class 1 is a cruiser, 2 a destroyer, 3 a carrier.
    struct Force {
        id: u32,
        side: Side,
        hex: Hex,
        classes: Vec<u32>,
    }

    fn japanese(id: u32, hex: Hex, classes: &[u32]) -> Force {
        Force {
            id,
            side: Side::Japanese,
            hex,
            classes: classes.to_vec(),
        }
    }

    /// Open sea from q -30 to 30 and r -10 to 10, an Allied base at (0,0)
    /// and Allied air group 1 there: 4 aircraft, sweep 40 nm, searching 060
    /// to 120 out to 600 nm. Land at (16,0). Test-only figures.
    fn world(seed: u64, forces: Vec<Force>) -> World {
        let mut map = Map::new();
        for q in -30..=30 {
            for r in -10..=10 {
                map.insert(Hex::new(q, r), HexData::open_sea());
            }
        }
        map.insert(
            Hex::new(16, 0),
            HexData {
                kind: HexKind::Land,
                ..HexData::open_sea()
            },
        );

        let mut ref_data = RefData::default();
        for (id, kind) in [
            (1, ShipKind::Cruiser),
            (2, ShipKind::Destroyer),
            (3, ShipKind::Carrier),
        ] {
            ref_data.ship_classes.insert(
                ShipClassId::new(id),
                ShipClass {
                    id: ShipClassId::new(id),
                    name: format!("Test class {id}"),
                    kind,
                    cruise_speed_kn: Fixed::from_int(16),
                    max_speed_kn: Fixed::from_int(32),
                    fuel_capacity_t: Fixed::from_int(2000),
                    endurance_nm: Fixed::from_int(8000),
                    cargo_capacity_t: Fixed::ZERO,
                    main_guns: 0,
                    gun_rounds_per_min: 0,
                    gun_rounds_per_gun: 0,
                    gun_damage: 0,
                    torpedo_tubes: 0,
                    torpedo_damage: 0,
                    toughness: 100,
                    radar: false,
                },
            );
        }
        ref_data.port_sizes.insert(
            0,
            PortSize {
                berths: 0,
                berth_rate_t_per_h: Fixed::ZERO,
                lighter_rate_t_per_h: Fixed::from_int(20),
            },
        );
        ref_data.aircraft_types.insert(
            AircraftTypeId::new(1),
            AircraftType {
                id: AircraftTypeId::new(1),
                name: "Test flying boat".to_string(),
                role: AircraftRole::Patrol,
                search_range_nm: Fixed::from_int(600),
                sweep_width_nm: Fixed::from_int(40),
            },
        );
        ref_data.search = Some(SearchFigures::from_search_toml());

        let mut truth = TruthState::default();
        truth.bases.insert(
            BaseId::new(1),
            Base {
                id: BaseId::new(1),
                name: "Test base".to_string(),
                side: Side::Allied,
                hex: BASE_HEX,
                supply_t: Fixed::ZERO,
            },
        );
        truth.air_groups.insert(
            GROUP,
            AirGroup {
                id: GROUP,
                name: "Test group".to_string(),
                side: Side::Allied,
                base: BaseId::new(1),
                aircraft_type: AircraftTypeId::new(1),
                ready: 4,
                mission: AirMission::Search,
                search: Some(SearchArc {
                    from_deg: 60,
                    to_deg: 120,
                    range_nm: Fixed::from_int(600),
                }),
                mission_order: None,
                arc_order: None,
            },
        );
        let mut next_ship = 1;
        for force in forces {
            let id = TaskForceId::new(force.id);
            let mut task_force = TaskForce::at(
                id,
                format!("Test force {}", force.id),
                force.side,
                force.hex,
                Mission::SurfaceCombat,
            );
            for class in force.classes {
                let ship = ShipId::new(next_ship);
                next_ship += 1;
                task_force.ships.insert(ship);
                truth.ships.insert(
                    ship,
                    Ship {
                        id: ship,
                        name: format!("Test ship {next_ship}"),
                        class: ShipClassId::new(class),
                        side: force.side,
                        fuel_t: Fixed::from_int(2000),
                        supply_t: Fixed::ZERO,
                        location: ShipLocation::TaskForce(id),
                        flotation_damage: Fixed::ZERO,
                        system_damage: Fixed::ZERO,
                        fire_damage: Fixed::ZERO,
                        gun_rounds: Fixed::ZERO,
                        torpedoes_ready: false,
                    },
                );
            }
            truth.task_forces.insert(id, task_force);
        }
        World::new(seed, ref_data, map, truth).expect("valid setup")
    }

    /// A second Allied group at the same base with the same arc.
    fn add_second_group(world: &mut World) -> AirGroupId {
        let mut second = world.truth.air_groups[&GROUP].clone();
        second.id = AirGroupId::new(2);
        world.truth.air_groups.insert(second.id, second);
        AirGroupId::new(2)
    }

    fn resolve_days(world: &mut World, days: u32) {
        for _ in 0..days {
            world.resolve_day().expect("day resolves");
        }
    }

    fn events_where(world: &World, keep: impl Fn(&Event) -> bool) -> Vec<Event> {
        world.events().iter().filter(|e| keep(e)).cloned().collect()
    }

    fn flown(world: &World) -> Vec<Event> {
        events_where(world, |e| matches!(e.kind, EventKind::SearchFlown { .. }))
    }

    fn attempts(world: &World) -> Vec<Event> {
        events_where(world, |e| matches!(e.kind, EventKind::SearchAttempt { .. }))
    }

    /// One task force of a cruiser and a destroyer, 400 nm east.
    fn target_at(q: i32, r: i32) -> Vec<Force> {
        vec![japanese(1, Hex::new(q, r), &[1, 2])]
    }

    // ---- who searches ----

    #[test]
    fn a_group_that_rests_has_no_arc_or_has_no_ready_aircraft_searches_nothing() {
        type Change = fn(&mut AirGroup);
        let setups: [(&str, Change); 3] = [
            ("resting", |g| g.mission = AirMission::Rest),
            ("no arc", |g| g.search = None),
            ("none ready", |g| g.ready = 0),
        ];
        for (label, change) in setups {
            let mut world = world(1, target_at(10, 0));
            change(world.truth.air_groups.get_mut(&GROUP).expect("group"));
            resolve_days(&mut world, 2);
            assert!(flown(&world).is_empty(), "{label}: SearchFlown");
            assert!(attempts(&world).is_empty(), "{label}: SearchAttempt");
        }
    }

    #[test]
    fn there_is_no_search_in_the_night_pulse_and_one_flight_per_daylight_pulse() {
        let mut world = world(1, target_at(10, 0));
        resolve_days(&mut world, 3);
        let flights = flown(&world);
        assert_eq!(flights.len(), 6);
        for day in 1..=3 {
            let pulses: Vec<Option<Pulse>> = flights
                .iter()
                .filter(|e| e.day == day)
                .map(|e| e.pulse)
                .collect();
            assert_eq!(pulses, vec![Some(Pulse::Morning), Some(Pulse::Afternoon)]);
        }
        assert!(attempts(&world)
            .iter()
            .all(|e| e.pulse != Some(Pulse::Night)));
    }

    #[test]
    fn search_flown_records_the_arc_the_aircraft_and_the_orders_that_set_them() {
        let mut world = world(1, vec![]);
        {
            let group = world.truth.air_groups.get_mut(&GROUP).expect("group");
            group.mission = AirMission::Rest;
            group.search = None;
        }
        let request = |order| OrderRequest {
            kind: OrderKind::AirGroup {
                target: GROUP,
                order,
            },
            origin: Origin::PLAYER,
        };
        world.submit_orders(
            Side::Allied,
            vec![
                request(AirGroupOrder::SetAirMission(AirMission::Search)),
                request(AirGroupOrder::SetSearchArc {
                    from_deg: 60,
                    to_deg: 120,
                    range_nm: Fixed::from_int(600),
                }),
            ],
        );
        resolve_days(&mut world, 1);
        let flights = flown(&world);
        assert_eq!(flights.len(), 2);
        let first = &flights[0];
        assert_eq!(
            first.kind.to_string(),
            "air_group:1 searched 060-120 out to 600 nm from base:1 with 4 aircraft"
        );
        assert_eq!(
            first.causes,
            vec![Cause::Order(OrderId::new(1)), Cause::Order(OrderId::new(2))]
        );
        assert_eq!(first.kind.entities().len(), 2);
    }

    // ---- who is a target ----

    #[test]
    fn a_task_force_with_every_ship_sunk_is_not_a_search_target() {
        let sink = |world: &mut World, ships: &[u32]| {
            for raw in ships {
                let id = ShipId::new(*raw);
                let ship = world.truth.ships.get_mut(&id).expect("ship");
                ship.location = ShipLocation::Sunk;
                ship.flotation_damage = Fixed::from_int(100);
                for task_force in world.truth.task_forces.values_mut() {
                    task_force.ships.remove(&id);
                }
            }
        };

        // With both ships afloat the force is searched for.
        let mut afloat = world(1, target_at(10, 0));
        resolve_days(&mut afloat, 1);
        assert!(!attempts(&afloat).is_empty());

        // Ships 1 and 2 are the force's. With one of them sunk it still is.
        let mut some = world(1, target_at(10, 0));
        sink(&mut some, &[1]);
        resolve_days(&mut some, 1);
        assert!(!attempts(&some).is_empty());

        // With both sunk the air group flies its search and finds nothing to
        // roll for.
        let mut none = world(1, target_at(10, 0));
        sink(&mut none, &[1, 2]);
        resolve_days(&mut none, 1);
        assert_eq!(flown(&none).len(), 2);
        assert!(attempts(&none).is_empty());
    }

    #[test]
    fn a_group_never_searches_for_its_own_side() {
        let mut forces = target_at(10, 0);
        forces.push(Force {
            id: 2,
            side: Side::Allied,
            hex: Hex::new(8, 0),
            classes: vec![1],
        });
        let mut world = world(1, forces);
        resolve_days(&mut world, 1);
        let seen: Vec<TaskForceId> = attempts(&world)
            .iter()
            .map(|e| match e.kind {
                EventKind::SearchAttempt { task_force, .. } => task_force,
                _ => unreachable!(),
            })
            .collect();
        // Only the Japanese force, once in each of the two daylight pulses.
        assert_eq!(seen, vec![TaskForceId::new(1); 2]);
    }

    #[test]
    fn only_forces_with_ships_inside_the_arc_and_range_are_targets() {
        let forces = vec![
            // 15 hexes = 600 nm, exactly at range: a target.
            japanese(1, Hex::new(15, 0), &[1]),
            // 40 hexes... off the map east is not possible, so 16 hexes
            // (640 nm) is beyond range. (16,0) is land but a force may sit
            // anywhere in a test.
            japanese(2, Hex::new(17, 0), &[1]),
            // Due south, 5 hexes: outside 060-120.
            japanese(3, Hex::new(-2, 5), &[1]),
            // In the arc, in range, but no ships.
            japanese(4, Hex::new(5, 0), &[]),
        ];
        let mut world = world(1, forces);
        resolve_days(&mut world, 1);
        let seen: Vec<(TaskForceId, Fixed)> = attempts(&world)
            .iter()
            .map(|e| match e.kind {
                EventKind::SearchAttempt {
                    task_force,
                    distance_nm,
                    ..
                } => (task_force, distance_nm),
                _ => unreachable!(),
            })
            .collect();
        let at_range = (TaskForceId::new(1), Fixed::from_int(600));
        assert_eq!(seen, vec![at_range, at_range]);
    }

    #[test]
    fn a_search_attempt_records_its_chance_breakdown_and_cites_the_flight() {
        let mut world = world(1, target_at(15, 0));
        resolve_days(&mut world, 1);
        let attempt = &attempts(&world)[0];
        let flight = &flown(&world)[0];
        assert_eq!(attempt.causes, vec![Cause::Event(flight.id)]);
        let EventKind::SearchAttempt {
            chance,
            distance_nm,
            ..
        } = attempt.kind
        else {
            unreachable!()
        };
        assert_eq!(chance, Fixed::from_raw(254_647));
        assert_eq!(distance_nm, Fixed::from_int(600));
        let breakdown = attempt.breakdown.as_ref().expect("breakdown recorded");
        let factors: Vec<(&str, Fixed)> = breakdown
            .factors
            .iter()
            .map(|f| (f.name, f.value))
            .collect();
        assert_eq!(
            factors,
            vec![
                ("degrees_per_radian", Fixed::from_raw(57_295_779)),
                ("aircraft", Fixed::from_int(4)),
                ("sweep_width_nm", Fixed::from_int(40)),
                ("arc_deg", Fixed::from_int(60)),
                ("distance_nm", Fixed::from_int(600)),
            ]
        );
        assert_eq!(breakdown.result, Fixed::from_raw(254_647));
        assert!(attempt
            .kind
            .to_string()
            .starts_with("air_group:1 searched for task_force:1 at 600 nm, chance 0.254647: "));
    }

    // ---- independence (decision 0013) ----

    fn sighting_of(world: &World, group: AirGroupId, day: u32, pulse: Pulse) -> bool {
        attempts(world)
            .iter()
            .find_map(|e| match e.kind {
                EventKind::SearchAttempt {
                    air_group, sighted, ..
                } if air_group == group && e.day == day && e.pulse == Some(pulse) => Some(sighted),
                _ => None,
            })
            .expect("the group searched")
    }

    #[test]
    fn two_groups_roll_separately_each_on_its_own_pair_stream() {
        // Target 10 hexes = 400 nm: 9167.32464 / 24000 = 0.381971(8).
        let expected_chance = sighting_chance(4, Fixed::from_int(40), 60, Fixed::from_int(400));
        assert_eq!(expected_chance, Fixed::from_raw(381_971));
        let mut differed = false;
        for seed in 1..=50 {
            let mut world = world(seed, target_at(10, 0));
            let second = add_second_group(&mut world);
            resolve_days(&mut world, 1);
            let mut outcomes = Vec::new();
            for group in [GROUP, second] {
                // The first draw of the pair stream is the sighting roll.
                let mut stream = world.rng_pair(
                    1,
                    Some(Pulse::Morning),
                    Subsystem::Search,
                    EntityRef::AirGroup(group),
                    EntityRef::TaskForce(TaskForceId::new(1)),
                );
                let predicted = stream.chance(expected_chance);
                let actual = sighting_of(&world, group, 1, Pulse::Morning);
                assert_eq!(actual, predicted, "seed {seed}, {group}");
                outcomes.push(actual);
            }
            differed |= outcomes[0] != outcomes[1];
        }
        assert!(differed, "two groups never disagreed in 50 seeds");
    }

    /// What the first target's attempts and reports look like, leaving out
    /// event and contact IDs, which other targets may shift.
    fn first_target_record(world: &World) -> Vec<String> {
        let mut record = Vec::new();
        for event in world.events().iter() {
            if let EventKind::SearchAttempt { task_force, .. } = event.kind {
                if task_force != TaskForceId::new(1) {
                    continue;
                }
                record.push(format!("{} {:?} {}", event.day, event.pulse, event.kind));
                for follower in world.events().iter() {
                    if follower.causes.contains(&Cause::Event(event.id)) {
                        match &follower.kind {
                            EventKind::ReportReceived {
                                hex,
                                ship_count,
                                kind,
                                confidence,
                                ..
                            } => record.push(format!(
                                "report {hex} {ship_count} {} {confidence}",
                                kind.name()
                            )),
                            other => record.push(format!("{other}")),
                        }
                    }
                }
            }
        }
        record
    }

    #[test]
    fn adding_a_second_target_changes_nothing_for_the_first() {
        let mut sightings = 0;
        for seed in 1..=50 {
            let mut alone = world(seed, target_at(10, 0));
            let mut with_second = {
                let mut forces = target_at(10, 0);
                // Far enough from the first that their contacts never merge.
                forces.push(japanese(2, Hex::new(3, 0), &[3]));
                world(seed, forces)
            };
            resolve_days(&mut alone, 1);
            resolve_days(&mut with_second, 1);
            let record = first_target_record(&alone);
            assert_eq!(record, first_target_record(&with_second), "seed {seed}");
            sightings += record
                .iter()
                .filter(|line| line.starts_with("report"))
                .count();
        }
        assert!(sightings > 10, "too few sightings to mean anything");
    }

    // ---- reports ----

    #[test]
    fn every_report_names_the_true_hex_or_a_sea_neighbour_and_the_hex_never_lies_elsewhere() {
        // The target at (15,0) has land at (16,0) to its east (direction 0).
        let mut at_true = 0;
        let mut at_neighbour = 0;
        for seed in 1..=300 {
            let mut world = world(seed, target_at(15, 0));
            resolve_days(&mut world, 1);
            for event in world.events().iter() {
                if let EventKind::ReportReceived { hex, .. } = event.kind {
                    let truth = Hex::new(15, 0);
                    assert!(
                        hex == truth || (hex.distance(truth) == 1 && world.map.is_navigable(hex))
                    );
                    assert_ne!(hex, Hex::new(16, 0), "seed {seed}: reported land");
                    if hex == truth {
                        at_true += 1;
                    } else {
                        at_neighbour += 1;
                    }
                }
            }
        }
        // Chance 0.254647 per search over 600 searches, so about 153
        // reports, 70 per cent of them true and the rest mostly neighbours.
        assert!(at_true > 0 && at_neighbour > 0, "{at_true} {at_neighbour}");
    }

    #[test]
    fn a_report_records_what_was_seen_cites_its_attempt_and_makes_a_contact() {
        // A target in the base's own hex is always sighted.
        let mut world = world(3, target_at(0, 0));
        resolve_days(&mut world, 1);
        let attempt = &attempts(&world)[0];
        let report = world
            .events()
            .iter()
            .find(|e| matches!(e.kind, EventKind::ReportReceived { .. }))
            .expect("a report");
        assert_eq!(report.causes, vec![Cause::Event(attempt.id)]);
        assert_eq!(report.pulse, Some(Pulse::Morning));
        assert_eq!(report.id.raw(), attempt.id.raw() + 1);
        let EventKind::ReportReceived {
            side,
            contact,
            created,
            confidence,
            ..
        } = report.kind
        else {
            unreachable!()
        };
        assert_eq!(side, Side::Allied);
        assert!(created);
        assert_eq!(confidence, fixed(8, 10));
        let belief = &world.beliefs[&Side::Allied];
        // Both pulses reported, and the second refreshed the first contact.
        assert_eq!(belief.contacts().count(), 1);
        assert!(belief.contact(contact).is_some());
        assert_eq!(world.beliefs[&Side::Japanese].contacts().count(), 0);
        let text = report.kind.to_string();
        assert!(text.starts_with("Allied report: "), "{text}");
        assert!(text.ends_with("new contact:1"), "{text}");
        let second = world
            .events()
            .iter()
            .filter(|e| matches!(e.kind, EventKind::ReportReceived { .. }))
            .nth(1)
            .expect("second report");
        assert!(second.kind.to_string().ends_with("refreshed contact:1"));
    }

    // ---- ageing ----

    #[test]
    fn a_contact_ages_by_the_figures_and_is_dropped_citing_its_last_report() {
        // A target at the base is sighted with certainty in both day 1
        // pulses. Then the group rests, so the contact ages every pulse from
        // the end of day 1's afternoon: that pulse refreshed it, so no ageing.
        // Day 2 night 0.8 x 0.8 = 0.64, morning 0.512, afternoon 0.4096,
        // day 3 night 0.32768, day 3 morning 0.262144 < 0.3: dropped.
        let mut world = world(5, target_at(0, 0));
        resolve_days(&mut world, 1);
        world.submit_orders(
            Side::Allied,
            vec![OrderRequest {
                kind: OrderKind::AirGroup {
                    target: GROUP,
                    order: AirGroupOrder::SetAirMission(AirMission::Rest),
                },
                origin: Origin::PLAYER,
            }],
        );
        resolve_days(&mut world, 2);

        let last_report: EventId = world
            .events()
            .iter()
            .filter(|e| matches!(e.kind, EventKind::ReportReceived { .. }))
            .last()
            .expect("reports")
            .id;
        let ageing = events_where(&world, |e| {
            matches!(
                e.kind,
                EventKind::ContactAged { .. } | EventKind::ContactDropped { .. }
            )
        });
        let summary: Vec<(u32, Option<Pulse>, u32, Fixed, bool)> = ageing
            .iter()
            .map(|e| match e.kind {
                EventKind::ContactAged {
                    age_pulses,
                    confidence,
                    ..
                } => (e.day, e.pulse, age_pulses, confidence, false),
                EventKind::ContactDropped {
                    age_pulses,
                    confidence,
                    ..
                } => (e.day, e.pulse, age_pulses, confidence, true),
                _ => unreachable!(),
            })
            .collect();
        assert_eq!(
            summary,
            vec![
                (2, Some(Pulse::Night), 1, Fixed::from_raw(640_000), false),
                (2, Some(Pulse::Morning), 2, Fixed::from_raw(512_000), false),
                (
                    2,
                    Some(Pulse::Afternoon),
                    3,
                    Fixed::from_raw(409_600),
                    false
                ),
                (3, Some(Pulse::Night), 4, Fixed::from_raw(327_680), false),
                (3, Some(Pulse::Morning), 5, Fixed::from_raw(262_144), true),
            ]
        );
        for event in &ageing {
            assert_eq!(event.causes, vec![Cause::Event(last_report)]);
        }
        assert!(ageing[4]
            .kind
            .to_string()
            .starts_with("Allied contact:1 dropped at 5 pulses"));
        assert_eq!(world.beliefs[&Side::Allied].contacts().count(), 0);
    }

    // ---- noticing ----

    #[test]
    fn a_noticed_sighting_records_the_pulse_start_and_cites_the_attempt() {
        let mut noticed_runs = 0;
        let mut unnoticed_runs = 0;
        for seed in 1..=100 {
            // At the base, so both searches sight.
            let mut world = world(seed, target_at(0, 0));
            resolve_days(&mut world, 1);
            let notices = events_where(&world, |e| {
                matches!(e.kind, EventKind::SightingNoticed { .. })
            });
            let recorded = world.truth.task_forces[&TaskForceId::new(1)].noticed_sighted_at_hours;
            for notice in &notices {
                let EventKind::SightingNoticed {
                    task_force,
                    air_group,
                    elapsed_hours,
                } = notice.kind
                else {
                    unreachable!()
                };
                assert_eq!((task_force, air_group), (TaskForceId::new(1), GROUP));
                let expected = match notice.pulse {
                    Some(Pulse::Morning) => Fixed::from_int(12),
                    Some(Pulse::Afternoon) => Fixed::from_int(18),
                    other => panic!("noticed in {other:?}"),
                };
                assert_eq!(elapsed_hours, expected);
                let attempt = world
                    .events()
                    .get(match notice.causes[..] {
                        [Cause::Event(id)] => id,
                        _ => panic!("one cause"),
                    })
                    .expect("cited event");
                assert!(matches!(
                    attempt.kind,
                    EventKind::SearchAttempt { sighted: true, .. }
                ));
                assert_eq!(attempt.pulse, notice.pulse);
            }
            match notices.last() {
                Some(last) => {
                    noticed_runs += 1;
                    let EventKind::SightingNoticed { elapsed_hours, .. } = last.kind else {
                        unreachable!()
                    };
                    assert_eq!(recorded, Some(elapsed_hours));
                }
                None => {
                    unnoticed_runs += 1;
                    assert_eq!(recorded, None);
                }
            }
            if let Some(event) = notices.first() {
                assert!(event
                    .kind
                    .to_string()
                    .starts_with("task_force:1 noticed it was sighted by air_group:1 at D1 "));
            }
        }
        // Two sightings at 0.5 each: about 75 of 100 runs notice at least once.
        assert!(noticed_runs > 0 && unnoticed_runs > 0);
    }

    #[test]
    fn a_noticed_sighting_reads_as_the_clock_time_of_the_pulse_start() {
        // Elapsed hour 12 is 06:00 on day 1 (the day starts at 18:00).
        let kind = EventKind::SightingNoticed {
            task_force: TaskForceId::new(1),
            air_group: GROUP,
            elapsed_hours: Fixed::from_int(18),
        };
        assert_eq!(
            kind.to_string(),
            "task_force:1 noticed it was sighted by air_group:1 at D1 12:00"
        );
    }

    // ---- recording and determinism ----

    type LogLine = (EventId, u32, Option<Pulse>, String, Vec<Cause>);

    fn log_without_breakdowns(world: &World) -> Vec<LogLine> {
        world
            .events()
            .iter()
            .map(|e| (e.id, e.day, e.pulse, e.kind.to_string(), e.causes.clone()))
            .collect()
    }

    #[test]
    fn breakdown_recording_changes_neither_the_log_nor_the_state_hash() {
        let mut sightings = 0;
        for seed in 1..=5 {
            let mut on = world(seed, target_at(6, 0));
            let mut off = world(seed, target_at(6, 0));
            off.set_config(SimConfig {
                record_breakdowns: false,
            });
            resolve_days(&mut on, 4);
            resolve_days(&mut off, 4);
            assert_eq!(on.day_hashes(), off.day_hashes(), "seed {seed}");
            assert_eq!(log_without_breakdowns(&on), log_without_breakdowns(&off));
            assert!(attempts(&on).iter().all(|e| e.breakdown.is_some()));
            assert!(attempts(&off).iter().all(|e| e.breakdown.is_none()));
            sightings +=
                events_where(&on, |e| matches!(e.kind, EventKind::ReportReceived { .. })).len();
        }
        assert!(sightings > 0);
    }

    #[test]
    fn the_same_seed_gives_the_same_event_log() {
        let run = || {
            let mut world = world(77, target_at(8, 0));
            add_second_group(&mut world);
            resolve_days(&mut world, 4);
            (
                world.events().iter().cloned().collect::<Vec<_>>(),
                world.day_hashes().to_vec(),
            )
        };
        let (first, first_hashes) = run();
        let (second, second_hashes) = run();
        assert!(!first.is_empty());
        assert_eq!(first, second);
        assert_eq!(first_hashes, second_hashes);
    }

    #[test]
    fn the_new_events_name_their_entities_and_read_clearly() {
        let attempt = EventKind::SearchAttempt {
            air_group: GROUP,
            task_force: TaskForceId::new(1),
            distance_nm: Fixed::from_int(600),
            chance: Fixed::from_raw(254_647),
            sighted: false,
        };
        assert_eq!(
            attempt.to_string(),
            "air_group:1 searched for task_force:1 at 600 nm, chance 0.254647: not sighted"
        );
        assert_eq!(
            attempt.entities(),
            vec![
                EntityRef::AirGroup(GROUP),
                EntityRef::TaskForce(TaskForceId::new(1))
            ]
        );
        let report = EventKind::ReportReceived {
            side: Side::Allied,
            contact: crate::ids::ContactId::new(1),
            hex: Hex::new(15, 0),
            ship_count: 3,
            kind: ShipKind::Cruiser,
            confidence: fixed(8, 10),
            created: true,
        };
        assert_eq!(
            report.to_string(),
            "Allied report: 3 ships, cruiser, at (15,0), confidence 0.8: new contact:1"
        );
        assert_eq!(
            report.entities(),
            vec![EntityRef::Contact(crate::ids::ContactId::new(1))]
        );
    }
}
