//! A model of where aircraft in the air go when their own deck is lost
//! (decisions 7 to 10 of `docs/decisions/0059-aircraft-that-cannot-land-where-they-took-off.md`),
//! written from the decision's words and not from the engine's code, for the
//! scenario tests that work a run's events by hand.
//!
//! The caller says what it knows at the end of the air combat step: the ships
//! (afloat or not, deck out or not, where), the groups with their aircraft
//! ready, damaged and in the air, the bases, and the decks lost in the pulse
//! with the event that lost each. [`homecoming`] works out, group by group
//! in group number order, where the aircraft in the air of each group whose
//! deck is lost go, and updates the groups as the rule does.

use sim_core::hex::Hex;
use sim_core::ids::EventId;
use sim_core::state::{AirGroupLocation, Side};

#[derive(Clone, Debug)]
pub struct ModelShip {
    pub id: u32,
    pub side: Side,
    pub carrier: bool,
    pub capacity: u32,
    pub afloat: bool,
    pub deck_out: bool,
    pub hex: Hex,
}

#[derive(Clone, Debug)]
pub struct ModelGroup {
    pub id: u32,
    pub side: Side,
    pub aircraft_type: u32,
    /// The type's strike radius, or its search range if that is 0.
    pub radius_nm: i64,
    pub place: AirGroupLocation,
    pub ready: u32,
    pub damaged: u32,
    pub in_air: u32,
}

#[derive(Clone, Copy, Debug)]
pub struct ModelBase {
    pub id: u32,
    pub side: Side,
    pub hex: Hex,
}

/// A deck lost in the pulse: where the ship was and the event that did it.
#[derive(Clone, Copy, Debug)]
pub struct DeckLoss {
    pub ship: u32,
    pub hex: Hex,
    pub cause: EventId,
}

/// What became of some of a group's aircraft in the air.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Home {
    Landed {
        from: u32,
        to: u32,
        at: AirGroupLocation,
        aircraft: u32,
        formed: bool,
    },
    NoDeck {
        group: u32,
        hex: Hex,
        aircraft: u32,
        beside_own_force: bool,
    },
}

impl Home {
    /// The group whose aircraft they were.
    pub fn source(&self) -> u32 {
        match self {
            Home::Landed { from, .. } => *from,
            Home::NoDeck { group, .. } => *group,
        }
    }
}

/// Works out the homecoming of the pulse. `losses` are the decks lost in the
/// pulse (the last one of a ship counts). Each entry of the answer has the
/// event that closed the deck. `groups` are updated: aircraft leave the
/// source group, join or form the receiving group (a formed group has the
/// next number of its side's series, ready and in the air as landed), and the
/// ditched are gone.
pub fn homecoming(
    ships: &[ModelShip],
    groups: &mut Vec<ModelGroup>,
    bases: &[ModelBase],
    losses: &[DeckLoss],
) -> Vec<(Home, Option<EventId>)> {
    let mut out = Vec::new();
    let mut ids: Vec<u32> = groups.iter().map(|g| g.id).collect();
    ids.sort_unstable();
    for id in ids {
        let group = groups.iter().find(|g| g.id == id).expect("group").clone();
        let AirGroupLocation::Ship(own) = group.place else {
            continue;
        };
        let own = own.raw();
        let ship = ships.iter().find(|s| s.id == own).expect("the ship");
        if group.in_air == 0 || !(!ship.afloat || ship.deck_out) {
            continue;
        }
        let loss = losses.iter().rev().find(|l| l.ship == own);
        let hex = loss.map_or(ship.hex, |l| l.hex);
        let closing = loss.map(|l| l.cause);
        let mut remaining = group.in_air;
        // 1. Another carrier of the side in the same hex, lowest number
        // first, as many as fit under her aircraft capacity.
        let mut carriers: Vec<&ModelShip> = ships
            .iter()
            .filter(|s| {
                s.side == group.side
                    && s.id != own
                    && s.carrier
                    && s.afloat
                    && !s.deck_out
                    && s.hex == hex
            })
            .collect();
        carriers.sort_by_key(|s| s.id);
        for carrier in carriers {
            if remaining == 0 {
                break;
            }
            let aboard: u32 = groups
                .iter()
                .filter(|g| {
                    g.place == AirGroupLocation::Ship(sim_core::ids::ShipId::new(carrier.id))
                })
                .map(|g| g.ready + g.damaged)
                .sum();
            let take = carrier.capacity.saturating_sub(aboard).min(remaining);
            if take == 0 {
                continue;
            }
            let at = AirGroupLocation::Ship(sim_core::ids::ShipId::new(carrier.id));
            out.push((land(groups, &group, at, take), closing));
            remaining -= take;
        }
        // 2. The nearest base of the side within the type's radius, then the
        // lowest number.
        if remaining > 0 {
            let base = bases
                .iter()
                .filter(|b| {
                    b.side == group.side && i64::from(hex.air_distance_nm(b.hex)) <= group.radius_nm
                })
                .min_by_key(|b| (hex.air_distance_nm(b.hex), b.id));
            if let Some(base) = base {
                let at = AirGroupLocation::Base(sim_core::ids::BaseId::new(base.id));
                out.push((land(groups, &group, at, remaining), closing));
                remaining = 0;
            }
        }
        // 3. The sea.
        if remaining > 0 {
            let beside = ships
                .iter()
                .any(|s| s.side == group.side && s.afloat && s.hex == hex);
            let source = groups.iter_mut().find(|g| g.id == id).expect("group");
            source.ready -= remaining;
            source.in_air -= remaining;
            out.push((
                Home::NoDeck {
                    group: id,
                    hex,
                    aircraft: remaining,
                    beside_own_force: beside,
                },
                closing,
            ));
        }
    }
    out
}

/// Lands `n` aircraft of `group` at `at`: they leave the source, and join the
/// lowest-numbered group of the type and side there or form a new one.
fn land(groups: &mut Vec<ModelGroup>, group: &ModelGroup, at: AirGroupLocation, n: u32) -> Home {
    {
        let source = groups.iter_mut().find(|g| g.id == group.id).expect("group");
        source.ready -= n;
        source.in_air -= n;
    }
    let existing = groups
        .iter()
        .filter(|g| g.place == at && g.side == group.side && g.aircraft_type == group.aircraft_type)
        .map(|g| g.id)
        .min();
    match existing {
        Some(to) => {
            let receiver = groups.iter_mut().find(|g| g.id == to).expect("group");
            receiver.ready += n;
            receiver.in_air += n;
            Home::Landed {
                from: group.id,
                to,
                at,
                aircraft: n,
                formed: false,
            }
        }
        None => {
            // Decision 0086: a formed group takes the next number of its own
            // side's series, which starts at 1,000,001 (Allied) or 2,000,001
            // (Japanese).
            let series: u32 = match group.side {
                sim_core::state::Side::Allied => 1_000_000,
                sim_core::state::Side::Japanese => 2_000_000,
            };
            let to = groups
                .iter()
                .filter(|g| g.side == group.side && g.id > series)
                .map(|g| g.id)
                .max()
                .unwrap_or(series)
                + 1;
            groups.push(ModelGroup {
                id: to,
                side: group.side,
                aircraft_type: group.aircraft_type,
                radius_nm: group.radius_nm,
                place: at,
                ready: n,
                damaged: 0,
                in_air: n,
            });
            Home::Landed {
                from: group.id,
                to,
                at,
                aircraft: n,
                formed: true,
            }
        }
    }
}

/// The ships, groups and bases of a freshly loaded world, as the model reads
/// them: everything afloat with its deck working, every group's aircraft
/// ready and none in the air. The caller updates what its run has changed.
pub fn initial(world: &sim_core::World) -> (Vec<ModelShip>, Vec<ModelGroup>, Vec<ModelBase>) {
    use sim_core::refdata::ShipKind;
    use sim_core::state::ShipLocation;
    let truth = world.umpire();
    let data = world.ref_data();
    let ships = truth
        .ships
        .values()
        .map(|ship| {
            let class = &data.ship_classes[&ship.class];
            let hex = match ship.location {
                ShipLocation::TaskForce(force) => truth.task_forces[&force].position(),
                ShipLocation::Port(base) => truth.bases[&base].hex,
                // Off the map either way: the model only places ships afloat
                // in a hex, and a ship at the yard flies nothing.
                ShipLocation::Sunk | ShipLocation::Yard(_) => Hex::new(0, 0),
            };
            ModelShip {
                id: ship.id.raw(),
                side: ship.side,
                carrier: class.kind == ShipKind::Carrier,
                capacity: class.aircraft_capacity,
                afloat: ship.location != ShipLocation::Sunk,
                deck_out: false,
                hex,
            }
        })
        .collect();
    let groups = truth
        .air_groups
        .values()
        .map(|group| {
            let aircraft = &data.aircraft_types[&group.aircraft_type];
            let radius = if aircraft.strike_range_nm > sim_core::fixed::Fixed::ZERO {
                aircraft.strike_range_nm
            } else {
                aircraft.search_range_nm
            };
            ModelGroup {
                id: group.id.raw(),
                side: group.side,
                aircraft_type: group.aircraft_type.raw(),
                radius_nm: radius.trunc(),
                place: group.location,
                ready: group.ready,
                damaged: group.damaged,
                in_air: 0,
            }
        })
        .collect();
    let bases = truth
        .bases
        .values()
        .map(|base| ModelBase {
            id: base.id.raw(),
            side: base.side,
            hex: base.hex,
        })
        .collect();
    (ships, groups, bases)
}

/// Property: a landing elsewhere or a ditching for want of a deck happens
/// only for a group aboard a ship, in a daylight pulse, and only when that
/// ship sank or had her flight deck go out in that same pulse: the first
/// cause of the event is that sinking or hit. `ship_of_group` gives the ship
/// a group is aboard (none for a group at a base, which is never affected).
/// Returns how many such events there were.
pub fn assert_only_for_lost_decks(
    events: &[sim_core::events::Event],
    ship_of_group: impl Fn(sim_core::ids::AirGroupId) -> Option<sim_core::ids::ShipId>,
) -> usize {
    use sim_core::events::{Cause, EventKind};
    use sim_core::state::SystemState;
    use sim_core::time::Pulse;
    let mut found = 0;
    for event in events {
        let source = match event.kind {
            EventKind::AircraftLandedElsewhere { from, .. } => from,
            EventKind::AircraftDitchedNoDeck { air_group, .. } => air_group,
            _ => continue,
        };
        found += 1;
        let ship = ship_of_group(source)
            .unwrap_or_else(|| panic!("{source} is at a base and is never moved or ditched"));
        assert!(
            matches!(event.pulse, Some(Pulse::Morning | Pulse::Afternoon)),
            "no homecoming in a pulse with no air combat step"
        );
        let Some(Cause::Event(closing)) = event.causes.first().copied() else {
            panic!("a homecoming cites the event that closed the deck first");
        };
        let closing = events
            .iter()
            .find(|e| e.id == closing)
            .expect("the closing event");
        assert_eq!((closing.day, closing.pulse), (event.day, event.pulse));
        let closes = match closing.kind {
            EventKind::ShipSunk { ship: sunk, .. } => sunk == ship,
            EventKind::SystemHit {
                ship: hit, after, ..
            } => hit == ship && after == SystemState::Out,
            _ => false,
        };
        assert!(closes, "{} closes no deck of {ship}", closing.kind);
    }
    found
}
