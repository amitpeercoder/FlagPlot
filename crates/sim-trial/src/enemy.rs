//! The enemy's half of a picture, by rule (decision 0174, 5 to 20).
//!
//! Reads: the side's own half (its belief: contacts, claims and estimates;
//! and the numbers of its own units), the side's view (the bases of the
//! enemy's on its map, the map's distances by sea, the classes and types
//! and whose each is, the search figures' merging distance), what the side
//! believed, the trial's figures and settings. Writes nothing: it returns
//! the enemy's half and what was assumed in making it.

use std::collections::{BTreeMap, BTreeSet};

use sim_core::believed::{BelievedForce, LastKnown, Standing};
use sim_core::command::CommandState;
use sim_core::fixed::Fixed;
use sim_core::hex::{Hex, HEX_NM};
use sim_core::ids::{
    AirGroupId, AircraftTypeId, BaseId, LandUnitId, ShipClassId, ShipId, TaskForceId,
};
use sim_core::orders::{Activity, AirMission, Mission, Posture, Reach, Readiness, SpeedSetting};
use sim_core::refdata::{AircraftRole, ShipKind};
use sim_core::rng::Draw;
use sim_core::state::{
    AirGroup, AirGroupLocation, Base, LandUnit, Route, RulesOfEngagement, Ship, ShipLocation,
    ShipSystems, Side, Skills, TaskForce, TaskForceStatus,
};
use sim_core::time::Pulse;
use sim_core::view::SideView;
use sim_core::world::MADE_IDS_FLOOR;
use sim_core::world_picture::{PictureBelief, PictureContact, PictureCounters, PictureHalf};

use crate::assumption::{Assumption, NotMade, SetAside, Source};
use crate::draw;
use crate::{Believed, Refused, Trial};
use sim_core::refdata::TrialFigures;

/// The first number of a ship, task force, air group or land unit made by
/// these rules (rule 20), each kind counted by itself.
pub(crate) const FIRST_MADE: u32 = 900_001;

/// The other side.
pub(crate) const fn other(side: Side) -> Side {
    match side {
        Side::Allied => Side::Japanese,
        Side::Japanese => Side::Allied,
    }
}

/// What is checked once for a trial, before any world: every belief handed
/// in is the side's own; the view has search figures; and no unit of the
/// side's own, numbered by its scenario, has a number the made units take
/// (rule 20). A unit the side made in play is numbered from its side's
/// series above 1,000,000 and cannot meet them.
pub(crate) fn check(
    half: &PictureHalf,
    view: &SideView<'_>,
    believed: &Believed,
) -> Result<(), Refused> {
    let side = half.side;
    if let Some(force) = believed.forces.iter().find(|force| force.side != side) {
        return Err(Refused(format!(
            "the believed force {} is the {} side's, and the {} side is tried",
            force.name,
            force.side.name(),
            side.name()
        )));
    }
    if let Some(men) = believed.men.iter().find(|men| men.side != side) {
        return Err(Refused(format!(
            "the believed men {} are the {} side's belief, and the {} side is tried",
            men.name,
            men.side.name(),
            side.name()
        )));
    }
    if let Some(intent) = believed.intents.iter().find(|intent| intent.side != side) {
        return Err(Refused(format!(
            "a believed intent is the {} side's, and the {} side is tried",
            intent.side.name(),
            side.name()
        )));
    }
    if view.search_figures().is_none() {
        return Err(Refused(
            "the view has no search figures, and so no merging distance".to_string(),
        ));
    }
    let reaches = |raw: u32| (FIRST_MADE..=MADE_IDS_FLOOR).contains(&raw);
    let own = half
        .ships
        .keys()
        .map(|id| (id.to_string(), id.raw()))
        .chain(half.task_forces.keys().map(|id| (id.to_string(), id.raw())))
        .chain(half.air_groups.keys().map(|id| (id.to_string(), id.raw())))
        .chain(half.land_units.keys().map(|id| (id.to_string(), id.raw())));
    for (name, raw) in own {
        if reaches(raw) {
            return Err(Refused(format!(
                "the {} side's {name} reaches the numbers of the units a trial makes, from \
                 {FIRST_MADE}",
                side.name()
            )));
        }
    }
    Ok(())
}

/// The hours of a contact's age (rule 13): its pulses counted back from the
/// picture's evening by their own lengths, the afternoon, the morning, the
/// night, and so on round.
fn hours_of_age(age: u32) -> u32 {
    let back = [Pulse::Afternoon, Pulse::Morning, Pulse::Night];
    let per_day: u32 = back
        .iter()
        .map(|pulse| u32::try_from(pulse.hours()).expect("a pulse's hours"))
        .sum();
    let part: u32 = back
        .iter()
        .take(usize::try_from(age % 3).expect("below 3"))
        .map(|pulse| u32::try_from(pulse.hours()).expect("a pulse's hours"))
        .sum();
    (age / 3).saturating_mul(per_day).saturating_add(part)
}

/// The miles and steps `hours` at `speed_kn` give (rule 13): steps at 40
/// miles a step, rounded down.
fn reach(hours: u32, speed_kn: Fixed) -> (Fixed, u32) {
    let miles = Fixed::from_int(i64::from(hours)) * speed_kn;
    let steps = (miles / Fixed::from_int(HEX_NM)).trunc();
    (miles, u32::try_from(steps.max(0)).unwrap_or(u32::MAX))
}

/// The ships of a force of `kind` and `count` (rule 7), by kind: one of the
/// kind and destroyers for the rest; all of the kind for a transport or an
/// auxiliary.
fn parts(kind: ShipKind, count: u32) -> Vec<(ShipKind, u32)> {
    let parts = match kind {
        ShipKind::Transport | ShipKind::Auxiliary | ShipKind::Destroyer => vec![(kind, count)],
        ShipKind::Carrier | ShipKind::Battleship | ShipKind::Cruiser => {
            vec![(kind, 1), (ShipKind::Destroyer, count.saturating_sub(1))]
        }
    };
    parts.into_iter().filter(|(_, n)| *n > 0).collect()
}

/// A force's mission by its kind (rule 10).
fn mission(kind: ShipKind) -> Mission {
    match kind {
        ShipKind::Carrier => Mission::Carrier,
        ShipKind::Battleship | ShipKind::Cruiser | ShipKind::Destroyer => Mission::SurfaceCombat,
        ShipKind::Transport | ShipKind::Auxiliary => Mission::Transport,
    }
}

/// The ships a force of `kind` and `count` would have, in the order made,
/// each by its class, and the parts left out for want of a class (rules 7
/// and 8); with the cruising speed of its slowest class built, if any.
struct Composition {
    ships: Vec<(ShipKind, ShipClassId)>,
    left_out: Vec<(ShipKind, u32)>,
    slowest_kn: Option<Fixed>,
}

/// What the rules read of the view and the trial, for one world.
struct Rules<'v, 'a> {
    view: &'v SideView<'a>,
    enemy: Side,
    day: u32,
    trial: Trial,
    world: u32,
    merge: u32,
}

impl Rules<'_, '_> {
    /// The lowest-numbered class of `kind` the file of sides gives to the
    /// enemy, alone or with the side (rule 8).
    fn class_of(&self, kind: ShipKind) -> Option<ShipClassId> {
        self.view
            .ship_classes()
            .filter(|class| class.kind == kind)
            .find(|class| {
                self.view
                    .ship_class_side(class.id)
                    .is_some_and(|whose| whose.used_by(self.enemy))
            })
            .map(|class| class.id)
    }

    /// The lowest-numbered type of `role` the file of sides gives to the
    /// enemy (rule 9).
    fn type_of(&self, role: AircraftRole) -> Option<AircraftTypeId> {
        self.view
            .aircraft_types()
            .filter(|aircraft| aircraft.role == role)
            .find(|aircraft| {
                self.view
                    .aircraft_type_side(aircraft.id)
                    .is_some_and(|whose| whose.used_by(self.enemy))
            })
            .map(|aircraft| aircraft.id)
    }

    fn compose(&self, kind: ShipKind, count: u32) -> Composition {
        let mut ships = Vec::new();
        let mut left_out = Vec::new();
        for (part, n) in parts(kind, count) {
            match self.class_of(part) {
                Some(class) => ships.extend((0..n).map(|_| (part, class))),
                None => left_out.push((part, n)),
            }
        }
        let slowest_kn = ships
            .iter()
            .filter_map(|(_, class)| self.view.ship_class(*class))
            .map(|class| class.cruise_speed_kn)
            .min();
        Composition {
            ships,
            left_out,
            slowest_kn,
        }
    }

    /// The fewest steps by sea from `from` to `to`, if ships can go.
    fn sea_steps(&self, from: Hex, to: Hex) -> Option<u32> {
        self.view.sea_distances_to(to).get(&from).copied()
    }

    /// A hex drawn evenly from those within `steps` by sea of `from`, in
    /// their own order (rule 13), on `stream`'s first draw; `None` where
    /// ships cannot be at `from`.
    fn draw_hex(&self, from: Hex, steps: u32, draw: Draw, place: u32) -> Option<Hex> {
        let within: Vec<Hex> = self
            .view
            .sea_distances_to(from)
            .into_iter()
            .filter(|(_, distance)| *distance <= steps)
            .map(|(hex, _)| hex)
            .collect();
        if within.is_empty() {
            return None;
        }
        let count = u64::try_from(within.len()).expect("a count of hexes");
        let pick = draw::stream(self.trial, self.world, self.day, draw, place, None).below(count);
        within
            .get(usize::try_from(pick).expect("below the count"))
            .copied()
    }
}

/// A contact as the rules take it: its place in the side's list, its hex,
/// its count, its kind after rule 11, and its age.
#[derive(Clone, Copy)]
struct Taken {
    place: u32,
    hex: Hex,
    count: u32,
    kind: ShipKind,
    age: u32,
}

/// The enemy's half as it is made, and what was assumed.
struct Making {
    half: PictureHalf,
    assumptions: Vec<Assumption>,
    next_ship: u32,
    next_force: u32,
    next_group: u32,
    next_unit: u32,
}

fn next(counter: &mut u32, what: &str) -> Result<u32, Refused> {
    let number = *counter;
    if number > MADE_IDS_FLOOR {
        return Err(Refused(format!(
            "more {what} are made than the numbers from {FIRST_MADE} to {MADE_IDS_FLOOR} hold"
        )));
    }
    *counter += 1;
    Ok(number)
}

/// Where a force is to be put: at a hex drawn by rule 13, or at a base.
enum Place {
    /// From `from`, with the hours since, drawn on `draw` for `place`.
    Reach {
        from: Hex,
        hours: u32,
        draw: Draw,
        place: u32,
    },
    /// A base of the enemy's, drawn on [`Draw::TrialBelievedBase`] for a
    /// believed force's `place` (rule 17).
    EnemyBase { place: u32 },
}

impl Making {
    /// Makes a force of `kind` and `count` for `source`, put by `place`
    /// (rules 7 to 10, 13, 16 and 17). Nothing is made where no ship can be
    /// built or no place found.
    fn force(
        &mut self,
        rules: &Rules<'_, '_>,
        source: Source,
        kind: ShipKind,
        count: u32,
        place: Place,
    ) -> Result<(), Refused> {
        let composition = rules.compose(kind, count);
        for (part, n) in &composition.left_out {
            self.assumptions.push(Assumption::ShipsLeftOut {
                source: source.clone(),
                kind: *part,
                count: *n,
            });
        }
        let Some(speed_kn) = composition.slowest_kn else {
            self.assumptions.push(Assumption::ForceNotMade {
                source,
                why: NotMade::NoShip,
            });
            return Ok(());
        };
        // Where it is.
        let (hex, placed) = match place {
            Place::Reach {
                from,
                hours,
                draw,
                place,
            } => {
                let (miles, steps) = reach(hours, speed_kn);
                let at = if hours == 0 {
                    rules.sea_steps(from, from).map(|_| from)
                } else {
                    rules.draw_hex(from, steps, draw, place)
                };
                let Some(at) = at else {
                    self.assumptions.push(Assumption::ForceNotMade {
                        source,
                        why: NotMade::NowhereToBe,
                    });
                    return Ok(());
                };
                (
                    at,
                    Placement::Reach {
                        from,
                        hours,
                        speed_kn,
                        miles,
                        steps,
                    },
                )
            }
            Place::EnemyBase { place } => {
                let bases: Vec<(BaseId, Hex)> = rules
                    .view
                    .enemy_bases()
                    .map(|base| (base.id, base.hex))
                    .collect();
                if bases.is_empty() {
                    self.assumptions.push(Assumption::ForceNotMade {
                        source,
                        why: NotMade::NoEnemyBase,
                    });
                    return Ok(());
                }
                let count = u64::try_from(bases.len()).expect("a count of bases");
                let pick = draw::stream(
                    rules.trial,
                    rules.world,
                    rules.day,
                    Draw::TrialBelievedBase,
                    place,
                    None,
                )
                .below(count);
                let (base, hex) = bases[usize::try_from(pick).expect("below the count")];
                (hex, Placement::Base(base))
            }
        };

        let force = TaskForceId::new(next(&mut self.next_force, "task forces")?);
        let mut ships = BTreeSet::new();
        let mut groups = Vec::new();
        for (part, class) in composition.ships {
            let id = ShipId::new(next(&mut self.next_ship, "ships")?);
            let figures = rules
                .view
                .ship_class(class)
                .expect("a class the view gives");
            self.half.ships.insert(
                id,
                Ship {
                    id,
                    name: format!("Assumed {}", part.name()),
                    class,
                    side: rules.enemy,
                    fuel_t: figures.fuel_capacity_t,
                    supply_t: Fixed::ZERO,
                    cargo_fuel_t: Fixed::ZERO,
                    location: ShipLocation::TaskForce(force),
                    flotation_damage: Fixed::ZERO,
                    fire_damage: Fixed::ZERO,
                    gun_rounds: Fixed::from_int(
                        i64::from(figures.main_guns) * i64::from(figures.gun_rounds_per_gun),
                    ),
                    torpedoes_ready: figures.torpedo_tubes > 0,
                    torpedoes_one_side_fired: false,
                    torpedoes_fired: 0,
                    systems: ShipSystems::ALL_WORKING,
                    experience: Ship::DEFAULT_EXPERIENCE,
                    system_repair_points: Fixed::ZERO,
                    troops: None,
                    available_from: None,
                    aircraft_fire: None,
                },
            );
            ships.insert(id);
            if part == ShipKind::Carrier {
                let capacity = figures.aircraft_capacity;
                let fighters = capacity / 3;
                for (role, aircraft) in [
                    (AircraftRole::Fighter, fighters),
                    (AircraftRole::Bomber, capacity - fighters),
                ] {
                    match rules.type_of(role).filter(|_| aircraft > 0) {
                        Some(aircraft_type) => {
                            let group = AirGroupId::new(next(&mut self.next_group, "air groups")?);
                            self.half.air_groups.insert(
                                group,
                                air_group(group, rules.enemy, id, aircraft_type, aircraft, role),
                            );
                            groups.push(group);
                        }
                        None => self.assumptions.push(Assumption::GroupLeftOut {
                            source: source.clone(),
                            role,
                            aircraft,
                        }),
                    }
                }
            }
        }
        let ship_count = u32::try_from(ships.len()).expect("a count of ships");
        self.half.task_forces.insert(
            force,
            TaskForce {
                id: force,
                name: format!("Assumed force for {source}"),
                side: rules.enemy,
                ships,
                mission: mission(kind),
                speed: SpeedSetting::Cruise,
                route: Route::default(),
                from_hex: hex,
                toward_hex: None,
                progress_nm: Fixed::ZERO,
                leg_nm: Fixed::from_int(HEX_NM),
                status: TaskForceStatus::Idle,
                route_order: None,
                speed_order: None,
                cargo_order: None,
                arrived_at_hours: None,
                arrival_event: None,
                readiness: Readiness::Normal,
                readiness_order: None,
                alerted: false,
                alerted_by: None,
                rules_of_engagement: RulesOfEngagement::DEFAULT,
                rules_order: None,
                home_port: None,
                fatigue: Fixed::ZERO,
                yard_trip: None,
                halt: None,
                preparation: None,
                landing_at: None,
                charge: None,
                fuel_from: None,
                available_from: None,
            },
        );
        self.assumptions.push(Assumption::ForceMade {
            force,
            source,
            kind,
            ships: ship_count,
            groups,
        });
        self.assumptions.push(match placed {
            Placement::Reach {
                from,
                hours,
                speed_kn,
                miles,
                steps,
            } => Assumption::Placed {
                force,
                from,
                hours,
                speed_kn,
                miles,
                steps,
                at: hex,
            },
            Placement::Base(base) => Assumption::AtEnemyBase { force, base, hex },
        });
        Ok(())
    }
}

/// How a force was put, for its assumption.
enum Placement {
    Reach {
        from: Hex,
        hours: u32,
        speed_kn: Fixed,
        miles: Fixed,
        steps: u32,
    },
    Base(BaseId),
}

/// A carrier's air group (rule 9): ready, at rest, with no fatigue, no
/// standing order and no pilots counted.
fn air_group(
    id: AirGroupId,
    side: Side,
    ship: ShipId,
    aircraft_type: AircraftTypeId,
    aircraft: u32,
    role: AircraftRole,
) -> AirGroup {
    let what = match role {
        AircraftRole::Fighter => "fighters",
        AircraftRole::Bomber => "bombers",
        AircraftRole::Patrol => "patrol aircraft",
    };
    AirGroup {
        id,
        name: format!("Assumed {what} of {ship}"),
        side,
        location: AirGroupLocation::Ship(ship),
        aircraft_type,
        ready: aircraft,
        damaged: 0,
        mission: AirMission::Rest,
        search: None,
        mission_order: None,
        arc_order: None,
        patrol_share: Fixed::ONE,
        share_order: None,
        fatigue: Fixed::ZERO,
        rest_above: None,
        rest_order: None,
        stood_down: false,
        pilots: None,
        strength: aircraft,
        reach: Reach::Normal,
        reach_order: None,
        target_priority: ShipKind::ALL.to_vec(),
        target_order: None,
        last_base_strike_day: None,
        rebased_day: None,
        strike_contact_age: None,
        strike_age_order: None,
        strike_held_share: Fixed::ZERO,
        strike_share_order: None,
        strike_sizing: false,
        strike_sizing_order: None,
        search_once: false,
        search_once_order: None,
        strike_first_waters: BTreeSet::new(),
        waters_order: None,
        available_from: None,
        // A group made for a trial's enemy starts at nothing (decision
        // 0180, 6).
        claims: sim_core::state::ClaimTally::default(),
    }
}

/// The contacts as the rules take them (rule 11): each with its kind
/// raised by the most important kind a claim of the side's own, from the
/// air, within the merging distance of its place, names, where that is more
/// important than its own; and what was raised.
fn taken_contacts(
    contacts: &[PictureContact],
    belief: &PictureBelief,
    merge: u32,
) -> (Vec<Taken>, Vec<Assumption>) {
    let mut assumptions = Vec::new();
    let taken = (1..)
        .zip(contacts)
        .map(|(place, contact)| {
            let raised = (1..)
                .zip(&belief.claims)
                .filter(|(_, claim)| claim.from_air && claim.hex.distance(contact.hex) <= merge)
                .filter_map(|(at, claim)| claim.claimed_kind.map(|kind| (kind, at)))
                .filter(|(kind, _)| *kind < contact.kind)
                .min();
            let kind = match raised {
                Some((kind, claim)) => {
                    assumptions.push(Assumption::KindRaised {
                        contact: place,
                        claim,
                        from: contact.kind,
                        to: kind,
                    });
                    kind
                }
                None => contact.kind,
            };
            Taken {
                place,
                hex: contact.hex,
                count: contact.ship_count,
                kind,
                age: contact.age_pulses,
            }
        })
        .collect();
    (taken, assumptions)
}

/// The pairs of contacts that may be one force (rule 12), in the order of
/// the side's own numbers, each contact in one pair at most: the older
/// could have steamed to the newer, the hexes between them by sea no more
/// than the merging distance and the steps of rule 13 for the pulses
/// between their ages, at the cruising speed of the older's slowest class.
fn pairs(rules: &Rules<'_, '_>, taken: &[Taken]) -> Vec<(usize, usize)> {
    let mut paired = BTreeSet::new();
    let mut found = Vec::new();
    for (i, first) in taken.iter().enumerate() {
        if paired.contains(&i) {
            continue;
        }
        for (j, second) in taken.iter().enumerate().skip(i + 1) {
            if paired.contains(&j) {
                continue;
            }
            let (older, newer) = if second.age > first.age {
                (second, first)
            } else {
                (first, second)
            };
            let Some(speed) = rules.compose(older.kind, older.count).slowest_kn else {
                continue;
            };
            let hours = hours_of_age(older.age) - hours_of_age(newer.age);
            let (_, steps) = reach(hours, speed);
            let may_be_one = rules
                .sea_steps(older.hex, newer.hex)
                .is_some_and(|between| between <= rules.merge.saturating_add(steps));
            if may_be_one {
                paired.insert(i);
                paired.insert(j);
                found.push((i, j));
                break;
            }
        }
    }
    found
}

/// The enemy's half of one world of a trial, and what was assumed in making
/// it (rules 5 to 20).
pub(crate) fn make(
    own: &PictureHalf,
    view: &SideView<'_>,
    believed: &Believed,
    figures: &TrialFigures,
    trial: Trial,
    world: u32,
) -> Result<(PictureHalf, Vec<Assumption>), Refused> {
    let enemy = other(own.side);
    let merge = view
        .search_figures()
        .map(|search| search.merge_distance_hexes)
        .ok_or_else(|| Refused("the view has no search figures".to_string()))?;
    let rules = Rules {
        view,
        enemy,
        day: own.day,
        trial,
        world,
        merge,
    };
    let mut making = Making {
        half: PictureHalf {
            day: own.day,
            side: enemy,
            ships: BTreeMap::new(),
            task_forces: BTreeMap::new(),
            bases: BTreeMap::new(),
            air_groups: BTreeMap::new(),
            land_units: BTreeMap::new(),
            // Decision 0195, 20: the enemy's made picture holds nothing
            // apart; what it is given in the state it has.
            entering: BTreeMap::new(),
            formations: BTreeMap::new(),
            replacements: Vec::new(),
            // Decision 0196: the enemy's made picture holds no supply line.
            supply_lines: Vec::new(),
            // Decision 0197: nor a stream of men or a unit's strength.
            men_replacement: Default::default(),
            arrivals: Vec::new(),
            orders: Vec::new(),
            command: CommandState::new(),
            // Rule 18: the enemy is given no belief.
            belief: PictureBelief::default(),
            counters: PictureCounters::first(enemy),
            // Decision 0194, 11: the game's days are the game's, the same
            // for both sides.
            game_days: own.game_days,
        },
        assumptions: Vec::new(),
        next_ship: FIRST_MADE,
        next_force: FIRST_MADE,
        next_group: FIRST_MADE,
        next_unit: FIRST_MADE,
    };

    // Rule 5: the bases of the enemy's on the side's map.
    for base in view.enemy_bases() {
        making.half.bases.insert(
            base.id,
            Base {
                id: base.id,
                name: base.name.clone(),
                side: base.side,
                hex: base.hex,
                supply_t: figures.enemy_base_supply_t,
                anti_aircraft: 0,
                aircraft_repairs_per_day: 0,
                fuel_t: figures.enemy_base_fuel_t,
                ship_repair_points_per_day: Fixed::ZERO,
                yard_passage_days: None,
                on_map: true,
                airfield: true,
                airfield_work: Fixed::ZERO,
                defences: Fixed::ZERO,
                day_work: None,
                airfield_damage: Fixed::ZERO,
                supply_reserve_t: Fixed::ZERO,
                fuel_reserve_t: Fixed::ZERO,
                ammunition_t: Fixed::ZERO,
                // Whether the enemy has landed rations there is not known:
                // a base of the picture has had none (decision 0205, 5).
                rations_landed: false,
            },
        );
        making.assumptions.push(Assumption::EnemyBase {
            base: base.id,
            hex: base.hex,
            supply_t: figures.enemy_base_supply_t,
            fuel_t: figures.enemy_base_fuel_t,
        });
    }

    // Rule 6: the men the side estimates ashore, in the order of their
    // hexes.
    for estimate in &own.belief.estimates {
        let id = LandUnitId::new(next(&mut making.next_unit, "land units")?);
        making.half.land_units.insert(
            id,
            men_unit(
                id,
                format!("Assumed men at {}", estimate.hex),
                enemy,
                estimate.hex,
                estimate.men,
            ),
        );
        making.assumptions.push(Assumption::MenAshore {
            unit: id,
            hex: estimate.hex,
            men: estimate.men,
        });
    }

    // Rule 11: the kinds, raised by the side's claims.
    let (taken, raised) = taken_contacts(&own.belief.contacts, &own.belief, merge);
    making.assumptions.extend(raised);

    // Rule 12: the pairs that may be one force, and in this world whether
    // each is.
    let mut one_with: BTreeMap<usize, usize> = BTreeMap::new();
    let mut second_of_one = BTreeSet::new();
    for (i, j) in pairs(&rules, &taken) {
        let (first, second) = (taken[i].place, taken[j].place);
        let one = draw::stream(trial, world, own.day, Draw::TrialPair, first, Some(second))
            .chance(Fixed::from_ratio(1, 2));
        making
            .assumptions
            .push(Assumption::Pair { first, second, one });
        if one {
            one_with.insert(i, j);
            second_of_one.insert(j);
        }
    }

    // Rules 7 to 10 and 13: a force for each contact, or for each pair that
    // is one, made at the first's turn.
    for (i, contact) in taken.iter().enumerate() {
        if second_of_one.contains(&i) {
            continue;
        }
        let (source, kind, count, at) = match one_with.get(&i) {
            Some(&j) => {
                let partner = &taken[j];
                let newer = if partner.age < contact.age {
                    partner
                } else {
                    contact
                };
                (
                    Source::Pair(contact.place, partner.place),
                    contact.kind.min(partner.kind),
                    contact.count.max(partner.count),
                    newer,
                )
            }
            None => (
                Source::Contact(contact.place),
                contact.kind,
                contact.count,
                contact,
            ),
        };
        making.force(
            &rules,
            source,
            kind,
            count,
            Place::Reach {
                from: at.hex,
                hours: hours_of_age(at.age),
                draw: Draw::TrialContactHex,
                place: at.place,
            },
        )?;
    }

    // Rules 14 to 17: the forces the side believes in and has not seen.
    let mut unknown = 0u32;
    let mut stood_for = BTreeSet::new();
    for (place, force) in (1..).zip(&believed.forces) {
        let in_world = match force.standing {
            Standing::Present => true,
            Standing::Absent => {
                making.assumptions.push(Assumption::RuledOut {
                    place,
                    name: force.name.clone(),
                });
                false
            }
            Standing::Unknown => {
                let digit = unknown;
                unknown += 1;
                let bits = u64::from(world - 1);
                let here = bits.checked_shr(digit).unwrap_or(0) & 1 == 1;
                if !here {
                    making.assumptions.push(Assumption::UnknownNotInWorld {
                        place,
                        name: force.name.clone(),
                    });
                }
                here
            }
        };
        if !in_world {
            continue;
        }
        let hours = force
            .last_known
            .day()
            .map(|day| own.day.saturating_sub(day).saturating_mul(24));
        if let Some(contact) = standing_contact(&rules, &taken, &stood_for, force, hours) {
            stood_for.insert(contact);
            making.assumptions.push(Assumption::TakenForContact {
                place,
                name: force.name.clone(),
                contact,
            });
            continue;
        }
        let source = Source::Believed(place, force.name.clone());
        let at = match (force.last_known.hex(), hours) {
            (Some(from), Some(hours)) => Place::Reach {
                from,
                hours,
                draw: Draw::TrialBelievedHex,
                place,
            },
            _ => Place::EnemyBase { place },
        };
        making.force(&rules, source, force.kind, force.ships, at)?;
    }

    // Decision 0176, rule 2 and amended 6: the men the side believes may be
    // ashore beyond its estimate, in the order of its list, each of unknown
    // standing taking its turn after the believed forces of unknown standing.
    for (place, men) in (1..).zip(&believed.men) {
        let in_world = if men.standing == Standing::Absent {
            making.assumptions.push(Assumption::BelievedMenRuledOut {
                place,
                name: men.name.clone(),
                hex: men.hex,
                men: men.men,
            });
            false
        } else if let Some(why) = set_aside(own, view, men.hex) {
            making.assumptions.push(Assumption::BelievedMenSetAside {
                place,
                name: men.name.clone(),
                hex: men.hex,
                men: men.men,
                standing: men.standing,
                why,
            });
            false
        } else if men.standing == Standing::Unknown {
            let digit = unknown;
            unknown += 1;
            let bits = u64::from(world - 1);
            let here = bits.checked_shr(digit).unwrap_or(0) & 1 == 1;
            if !here {
                making.assumptions.push(Assumption::BelievedMenNotInWorld {
                    place,
                    name: men.name.clone(),
                    hex: men.hex,
                    men: men.men,
                });
            }
            here
        } else {
            true
        };
        if !in_world {
            continue;
        }
        let id = LandUnitId::new(next(&mut making.next_unit, "land units")?);
        making.half.land_units.insert(
            id,
            men_unit(
                id,
                format!("Assumed men believed at {} ({})", men.hex, men.name),
                enemy,
                men.hex,
                men.men,
            ),
        );
        making.assumptions.push(Assumption::BelievedMenMade {
            unit: id,
            place,
            name: men.name.clone(),
            hex: men.hex,
            men: men.men,
            standing: men.standing,
        });
    }

    // Decision 0209, rule 12: where a fight costs supply, the enemy made is
    // taken as supplied in full, for the side does not know his stocks.
    if view
        .ground_figures()
        .and_then(|figures| figures.hunger)
        .and_then(|hunger| hunger.fight_supply)
        .is_some()
    {
        making.assumptions.push(Assumption::EnemySupplied);
    }

    // Rules 18 and 19.
    making.assumptions.push(Assumption::EnemyGivenNoBelief);
    if believed.intents.is_empty() {
        making.assumptions.push(Assumption::EnemyGivenNoIntent);
    }
    for intent in &believed.intents {
        making
            .assumptions
            .push(Assumption::EnemyIntent { kind: intent.kind });
    }
    Ok((making.half, making.assumptions))
}

/// A fighting unit of `men` men at `hex`, on defend, as rule 6 makes the
/// unit of an estimate: the standard experience and every skill at the
/// standard, fed, rested, with no nights of hunger, no fieldworks, no rehearsal, no march, no
/// formation and no standing order.
fn men_unit(id: LandUnitId, name: String, side: Side, hex: Hex, men: u32) -> LandUnit {
    LandUnit {
        id,
        name,
        side,
        men_ashore: men,
        hex: (men > 0).then_some(hex),
        posture: Posture::Defend,
        posture_order: None,
        fatigue: Fixed::ZERO,
        fed_share: Fixed::ONE,
        march: None,
        rehearsal: Fixed::ZERO,
        rehearsal_hex: None,
        activity: Activity::Ready,
        activity_order: None,
        experience: LandUnit::DEFAULT_EXPERIENCE,
        training_days: 0,
        fight_days: 0,
        skills: Skills::STANDARD_ALL,
        working: false,
        when_spent: None,
        fieldworks: Fixed::ZERO,
        available_from: None,
        formation: None,
        hunger_nights: 0,
    }
}

/// Why believed men at `hex` are set aside (decision 0176, amended 6), if
/// they are: the side holds an estimate of the hex of a day after the
/// scenario's start, a count it has had since; or the place is the side's,
/// a base of its own in the hex and none of the enemy's there on its map.
fn set_aside(own: &PictureHalf, view: &SideView<'_>, hex: Hex) -> Option<SetAside> {
    if let Some(estimate) = own
        .belief
        .estimates
        .iter()
        .find(|estimate| estimate.hex == hex && estimate.day > 0)
    {
        return Some(SetAside::CountSince { day: estimate.day });
    }
    let ours = own.bases.values().any(|base| base.hex == hex)
        && !view
            .enemy_bases()
            .any(|base| base.hex == hex && base.side != own.side);
    ours.then_some(SetAside::PlaceIsTheSides)
}

/// The contact that may stand for a believed force (rule 15), if any: the
/// first in the side's list of the force's kind (after rule 11) not already
/// taken for another, which it could be: any such contact where the force's
/// place is not known; else one whose reported hex is within the reach of
/// rule 13, by sea, of where it was last known, for the hours since.
fn standing_contact(
    rules: &Rules<'_, '_>,
    taken: &[Taken],
    stood_for: &BTreeSet<u32>,
    force: &BelievedForce,
    hours: Option<u32>,
) -> Option<u32> {
    let reach_of = match (force.last_known, hours) {
        (LastKnown::NotKnown, _) | (_, None) => None,
        (known, Some(hours)) => {
            let speed = rules.compose(force.kind, force.ships).slowest_kn?;
            Some((known.hex()?, reach(hours, speed).1))
        }
    };
    taken
        .iter()
        .filter(|contact| contact.kind == force.kind && !stood_for.contains(&contact.place))
        .find(|contact| match reach_of {
            None => true,
            Some((from, steps)) => rules
                .sea_steps(contact.hex, from)
                .is_some_and(|between| between <= steps),
        })
        .map(|contact| contact.place)
}
