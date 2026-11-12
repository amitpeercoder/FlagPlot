//! Scenario 13a: strike groups with target lists choose among three
//! contacts, and one holds its strike.
//!
//! Stage 1 of decision 0063. It proves the target list as an order: which
//! contact a group goes for, a group that holds, which force a raid attacks
//! and which ship each group attacks. Nobody opposes the strikes: there are
//! no fighters and no anti-aircraft fire.
//!
//! The data is placeholder data: see
//! `data/scenarios/13a_target_priority.toml`. The rule is
//! `docs/decisions/0063-strike-target-priority.md`.
//!
//! # Hand calculation
//!
//! Written by the architect before the code. The tests below assert these
//! numbers. If the code disagrees, the code or the rule is wrong: do not
//! change a number here to make a test pass.
//!
//! ## The figures
//!
//! The frozen step 1 figures, with one changed for this test through
//! `common::altered_reference`: `air_combat.toml:
//! strike_contact_max_age_pulses` is 1, not 0. A contact a side starts the
//! game holding at age 0 is aged at the end of the night pulse, so it is 1
//! pulse old at the morning's decision. With the figure at 1 it can be
//! struck in the morning, and the morning needs no search and no random
//! report. Everything asserted exactly is of the morning pulse of day 1.
//!
//! ## The setting
//!
//! The map is `placeholder_search.toml`. One day.
//!
//! The Allied carrier force, task force 1, lies stopped at (4,0): carrier A,
//! ship 1 (class 4, 36 aircraft), and a destroyer, ship 2. Aboard carrier A
//! are six groups of 6 dive bombers each (type 3: radius 250 nm, a bomb of
//! damage 60), air groups 1 to 6, all ordered to strike ships.
//!
//! The Japanese forces all lie stopped, and have no aircraft:
//!
//! | Force | Hex    | Ships                                  | Air distance from (4,0) |
//! | ----- | ------ | -------------------------------------- | ----------------------- |
//! | 2     | (7,0)  | Cruiser, ship 3; destroyer, ship 4     | 120 nm                  |
//! | 3     | (7,3)  | Cruiser, ship 5; destroyer, ship 6     | 208 nm                  |
//! | 4     | (7,3)  | Cargo ship, ship 7                     | 208 nm                  |
//! | 5     | (9,-3) | Cruiser, ship 8; destroyer, ship 9     | 174 nm                  |
//!
//! The distances: to (7,0), 3 hexes along an axis, 120. To (7,3), dq 3 and
//! dr 3, 40 x the square root of (9 + 9 + 9) = 40 x 5.196152 = 207.85, to
//! the nearest mile 208. To (9,-3), dq 5 and dr -3, 40 x the square root of
//! (25 + 9 - 15) = 40 x 4.358899 = 174.36, 174.
//!
//! The Allied side starts holding three contacts, each at age 0, entered in
//! this order, so they are contacts 1, 2 and 3:
//!
//! | Contact | Hex    | Kind      | Ships | What is really there         |
//! | ------- | ------ | --------- | ----- | ---------------------------- |
//! | 1       | (7,0)  | cruiser   | 2     | Force 2                      |
//! | 2       | (7,3)  | transport | 3     | Forces 3 and 4               |
//! | 3       | (9,-3) | carrier   | 2     | Force 5, which has no carrier |
//!
//! The three hexes are 3, 3 and 6 hexes from one another, more than the
//! merge distance of 2, so each entry makes its own contact.
//!
//! ## The orders
//!
//! | Group | Target list             |
//! | ----- | ----------------------- |
//! | 1     | None given: the default |
//! | 2     | transport, cruiser      |
//! | 3     | battleship              |
//! | 4     | cruiser                 |
//! | 5     | destroyer, cruiser      |
//! | 6     | carrier, transport      |
//!
//! ## The morning's decisions
//!
//! All three contacts are 1 pulse old and within 250 nm. A group takes the
//! contact whose kind comes first on its list, then the nearest, then the
//! lowest number. A kind not on its list is never launched at.
//!
//! | Group | Contact taken | Why                                                        | Launch                 |
//! | ----- | ------------- | ---------------------------------------------------------- | ---------------------- |
//! | 1     | 3             | The default list has carrier first, as the rule was        | 6 to (9,-3), 174 nm    |
//! | 2     | 2             | Transport is first on its list, though contact 1 is nearer | 6 to (7,3), 208 nm     |
//! | 3     | None          | No contact is a battleship                                 | None. Nothing recorded |
//! | 4     | 1             | The only cruiser                                           | 6 to (7,0), 120 nm     |
//! | 5     | 1             | No contact is a destroyer; cruiser is next on its list     | 6 to (7,0), 120 nm     |
//! | 6     | 3             | Carrier is first on its list                               | 6 to (9,-3), 174 nm    |
//!
//! Five `StrikeLaunched` events, in group order, and none for group 3. Each
//! cites the group's mission order, the group's target order if it was
//! given one (so group 1's cites none), and the report that made its
//! contact.
//!
//! ## The raids
//!
//! Strikes from one place at one hex are one raid, so there are three, in
//! the order of their first groups: groups 1 and 6 at (9,-3); group 2 at
//! (7,3); groups 4 and 5 at (7,0).
//!
//! A group ranks ships by its list first, in the list's order, and then by
//! the kinds not on its list, in the order of importance. The raid attacks
//! the force at its hex that holds the best-ranked ship afloat by the list
//! of the raid's first group, the lowest force number on a tie. Each group
//! attacks the best-ranked ship afloat in that force by its own list, the
//! lowest ship number on a tie.
//!
//! | Raid at | Force attacked | Why                                                                 |
//! | ------- | -------------- | ------------------------------------------------------------------- |
//! | (9,-3)  | 5              | The only force there                                                |
//! | (7,3)   | 4              | Group 2's list has transport first, and force 4 holds the cargo ship. By the order of importance it would be force 3, for its cruiser |
//! | (7,0)   | 2              | The only force there                                                |
//!
//! | Group | Ship attacked | Why                                                                          |
//! | ----- | ------------- | ---------------------------------------------------------------------------- |
//! | 1     | 8, cruiser    | The most important ship of force 5                                           |
//! | 2     | 7, cargo ship | Transport is first on its list                                               |
//! | 4     | 3, cruiser    | Cruiser is its list                                                          |
//! | 5     | 4, destroyer  | Destroyer is first on its list. Two groups of one raid attack different ships |
//! | 6     | 8, cruiser    | Force 5 has no carrier and no transport. Off its list the order of importance gives battleship, then cruiser. A strike sent at the wrong kind attacks what it finds |
//!
//! Each attack is 6 shots at a bomb's chance of 0.2: 1.2 expected hits,
//! from 0 to 6. So five `AirAttack` events in group order, each with the
//! force and ship above, a bomb, shots 6, chance 0.2, expected 1.2.
//!
//! A hit does 100 x 60 / toughness points: 15 on a cruiser (400), 20 on the
//! cargo ship (300), 60 on a destroyer (100).
//!
//! No aircraft is lost: nothing fires at them. After the morning, damage is
//! on ships 3, 4, 7 and 8 only, and ships 5, 6 and 9 are untouched.
//!
//! Over seeds 1 to 1000, each group's morning hits in all are within four
//! standard deviations of 1,200. One run's hits have a standard deviation
//! of the square root of (6 x 0.2 x 0.8) = 0.979796, so 1,000 runs have
//! 30.98, and the allowed range is 1,077 to 1,323.
//!
//! ## With no target order
//!
//! The same scenario with the five target orders taken out is the rule as
//! it was. All six groups take contact 3, the carrier, and fly as one raid
//! of 36 to (9,-3). All attack ship 8: six `AirAttack` events of 6 shots
//! each, 7.2 expected hits in all. No `StrikeLaunched` cites a target
//! order. Ships 3 to 7 are untouched in the morning.
//!
//! ## The afternoon
//!
//! Not asserted exactly. A strike that attacked sends a report, which may
//! name a neighbouring hex or another kind, and by the afternoon the
//! contacts differ from seed to seed. The properties below hold in both
//! pulses.
//!
//! # Properties
//!
//! - Over seeds 1 to 1000 and both pulses, a group is never launched at a
//!   contact whose estimated kind, when it was launched, is not on its
//!   list. Group 3 is never launched in the morning, and in the afternoon
//!   only at a contact reported as a battleship. (Corrected by the
//!   architect on Nov 12, 2026. First written as "Group 3 is never
//!   launched", which forgot that a strike's report of the cruiser force it
//!   attacked can name a battleship, by the confusion table of
//!   `search.toml`. The developer measured 111 such launches in 1,000 runs
//!   and reported the mismatch. No number of the morning changed.)
//! - A group attacks a ship whose kind is off its list only when no ship
//!   afloat in the force attacked is of a kind on its list.
//! - Every group of a raid attacks the same force.
//! - Giving a group the default list by order changes nothing but the
//!   cause its launch cites: the same launches, attacks and hits, seed for
//!   seed, as with no order.
//! - A target order with an empty list, or with a kind named twice, is
//!   refused, and the group keeps the list it had.
//! - The aircraft a group flies never exceed its aircraft, and no track is
//!   above 100.
//! - A replay of any seed reproduces the daily hash.

mod common;

use std::collections::BTreeMap;
use std::fs;
use std::path::PathBuf;

use sim_core::events::{Cause, Event, EventKind, Weapon};
use sim_core::fixed::Fixed;
use sim_core::hex::Hex;
use sim_core::ids::{AirGroupId, EventId, ShipId};
use sim_core::orders::{AirGroupOrder, AirMission, OrderKind, OrderRequest, Origin, RejectReason};
use sim_core::refdata::ShipKind;
use sim_core::state::Side;
use sim_core::time::Pulse;
use sim_core::World;
use sim_data::Scenario;

const ROOT: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/..");
const SCENARIO: &str = "13a_target_priority.toml";
const SEEDS: u64 = 1000;
const GROUPS: [u32; 6] = [1, 2, 3, 4, 5, 6];

/// Where the scenario file starts the lists of its target orders.
const LISTS_START: &str = "# The target lists.";

fn scenario_text() -> (String, PathBuf) {
    let path = PathBuf::from(ROOT).join("data/scenarios").join(SCENARIO);
    (fs::read_to_string(&path).expect("the scenario reads"), path)
}

/// The frozen figures with one changed: a contact the side starts with can be
/// struck in the morning.
fn options() -> sim_data::LoadOptions {
    common::altered_reference("target_priority", |dir| {
        common::edit_file_by(
            dir,
            "air_combat.toml",
            &[(
                "strike_contact_max_age_pulses = 0",
                "strike_contact_max_age_pulses = 1",
            )],
        );
    })
}

/// The scenario as the file gives it, on the altered figures.
fn with_orders() -> Scenario {
    let (text, path) = scenario_text();
    common::from_text_with(&text, &path, &options()).expect("the scenario loads")
}

/// The scenario with its five target orders taken out.
fn without_target_orders() -> Scenario {
    let (text, path) = scenario_text();
    let cut = text.find(LISTS_START).expect("the target lists are marked");
    common::from_text_with(&text[..cut], &path, &options()).expect("the variant loads")
}

/// The scenario with a target order for each group, each with the default
/// list written out.
fn with_the_default_list_by_order() -> Scenario {
    let (text, path) = scenario_text();
    let cut = text.find(LISTS_START).expect("the target lists are marked");
    let mut text = text[..cut].to_string();
    for group in GROUPS {
        text.push_str(&format!(
            "[[order]]\nday = 1\nside = \"allied\"\nair_group = {group}\nkind = \"set_target_priority\"\nkinds = [\"carrier\", \"battleship\", \"cruiser\", \"destroyer\", \"transport\", \"auxiliary\"]\n\n"
        ));
    }
    common::from_text_with(&text, &path, &options()).expect("the variant loads")
}

fn run_seed(base: &Scenario, seed: u64) -> World {
    let mut scenario = base.with_seed(seed).expect("a seed can be set");
    scenario.run_to_end().expect("the run resolves");
    scenario.world
}

fn runs(base: &Scenario) -> Vec<World> {
    (1..=SEEDS).map(|seed| run_seed(base, seed)).collect()
}

// ---------------------------------------------------------------------
// The hand calculation's tables.
// ---------------------------------------------------------------------

/// The lists of the orders. Group 1 has none given.
fn list_of(group: u32) -> Vec<ShipKind> {
    match group {
        1 => ShipKind::ALL.to_vec(),
        2 => vec![ShipKind::Transport, ShipKind::Cruiser],
        3 => vec![ShipKind::Battleship],
        4 => vec![ShipKind::Cruiser],
        5 => vec![ShipKind::Destroyer, ShipKind::Cruiser],
        6 => vec![ShipKind::Carrier, ShipKind::Transport],
        other => panic!("no group {other}"),
    }
}

/// (group, contact taken, hex, distance in nm) of each launch of the
/// morning, in group order. Group 3 launches none.
const LAUNCHES: [(u32, u32, Hex, i64); 5] = [
    (1, 3, Hex::new(9, -3), 174),
    (2, 2, Hex::new(7, 3), 208),
    (4, 1, Hex::new(7, 0), 120),
    (5, 1, Hex::new(7, 0), 120),
    (6, 3, Hex::new(9, -3), 174),
];

/// (group, force attacked, ship attacked) of each morning attack, in group
/// order.
const ATTACKS: [(u32, u32, u32); 5] = [(1, 5, 8), (2, 4, 7), (4, 2, 3), (5, 2, 4), (6, 5, 8)];

/// What a hit does, in points, on each Japanese ship: 100 x 60 over the
/// toughness (400 a cruiser, 300 the cargo ship, 100 a destroyer).
fn points_per_hit(ship: u32) -> i64 {
    match ship {
        3 | 5 | 8 => 15,
        7 => 20,
        4 | 6 | 9 => 60,
        other => panic!("no Japanese ship {other}"),
    }
}

/// The ships of the scenario: kind, and which force each is in.
struct Setup {
    kind: BTreeMap<u32, ShipKind>,
    ships_of: BTreeMap<u32, Vec<u32>>,
    hex_of: BTreeMap<u32, Hex>,
}

fn setup() -> Setup {
    let scenario = with_orders();
    let world = &scenario.world;
    let truth = world.umpire();
    let mut kind = BTreeMap::new();
    let mut ships_of: BTreeMap<u32, Vec<u32>> = BTreeMap::new();
    let mut hex_of = BTreeMap::new();
    for (id, ship) in &truth.ships {
        kind.insert(id.raw(), world.ref_data().ship_classes[&ship.class].kind);
    }
    for (id, force) in &truth.task_forces {
        hex_of.insert(id.raw(), force.position());
        ships_of.insert(id.raw(), force.ships.iter().map(|s| s.raw()).collect());
    }
    Setup {
        kind,
        ships_of,
        hex_of,
    }
}

// ---------------------------------------------------------------------
// What a run says, read from its log.
// ---------------------------------------------------------------------

fn events_of(world: &World, keep: impl Fn(&Event) -> bool) -> Vec<Event> {
    world
        .events()
        .iter()
        .filter(|event| keep(event))
        .cloned()
        .collect()
}

fn in_pulse(event: &Event, pulse: Pulse) -> bool {
    event.day == 1 && event.pulse == Some(pulse)
}

fn launches(world: &World, pulse: Pulse) -> Vec<Event> {
    events_of(world, |e| {
        in_pulse(e, pulse) && matches!(e.kind, EventKind::StrikeLaunched { .. })
    })
}

fn attacks(world: &World, pulse: Pulse) -> Vec<Event> {
    events_of(world, |e| {
        in_pulse(e, pulse) && matches!(e.kind, EventKind::AirAttack { .. })
    })
}

/// (group, contact, hex, aircraft, distance) of a `StrikeLaunched`.
fn launch_of(event: &Event) -> (u32, u32, Hex, u32, Fixed) {
    match event.kind {
        EventKind::StrikeLaunched {
            air_group,
            contact,
            hex,
            aircraft,
            distance_nm,
        } => (air_group.raw(), contact.raw(), hex, aircraft, distance_nm),
        ref other => panic!("not a launch: {other:?}"),
    }
}

/// An `AirAttack`, read.
struct Attack {
    id: EventId,
    group: u32,
    force: u32,
    ship: u32,
    weapon: Weapon,
    shots: Fixed,
    chance: Fixed,
    expected: Fixed,
    hits: u32,
}

fn attack_of(event: &Event) -> Attack {
    match event.kind {
        EventKind::AirAttack {
            air_group,
            task_force,
            ship,
            weapon,
            shots,
            chance,
            expected,
            hits,
        } => Attack {
            id: event.id,
            group: air_group.raw(),
            force: task_force.raw(),
            ship: ship.raw(),
            weapon,
            shots,
            chance,
            expected,
            hits,
        },
        ref other => panic!("not an attack: {other:?}"),
    }
}

fn mission_order(world: &World, group: u32) -> sim_core::ids::OrderId {
    world.umpire().air_groups[&AirGroupId::new(group)]
        .mission_order
        .expect("the group's mission order")
}

fn target_order(world: &World, group: u32) -> Option<sim_core::ids::OrderId> {
    world.umpire().air_groups[&AirGroupId::new(group)].target_order
}

/// The `ReportReceived` event of a contact the side started with, by contact
/// number, on day 0.
fn starting_report(world: &World, contact: u32) -> EventId {
    events_of(world, |e| {
        e.day == 0
            && matches!(e.kind, EventKind::ReportReceived { contact: c, side: Side::Allied, .. } if c.raw() == contact)
    })
    .first()
    .expect("a report that made the contact")
    .id
}

/// The points of damage the morning's attack on `ship` did: the sum of the
/// `ShipDamaged` events that cite the attack, each asserted to be a whole
/// number of hits at the figure of the ship.
fn points_of_attack(world: &World, attack: &Attack) -> Fixed {
    let mut total = Fixed::ZERO;
    let mut hits = 0;
    for event in events_of(world, |e| e.causes == vec![Cause::Event(attack.id)]) {
        if let EventKind::ShipDamaged {
            ship,
            hits: track_hits,
            points,
            ..
        } = event.kind
        {
            assert_eq!(ship.raw(), attack.ship);
            assert_eq!(
                points,
                Fixed::from_int(points_per_hit(attack.ship) * i64::from(track_hits))
            );
            total += points;
            hits += track_hits;
        }
    }
    assert_eq!(hits, attack.hits, "the damage adds up to the hits");
    total
}

fn damaged_ships_in_the_morning(world: &World) -> Vec<u32> {
    let mut ships: Vec<u32> = events_of(world, |e| {
        in_pulse(e, Pulse::Morning) && matches!(e.kind, EventKind::ShipDamaged { .. })
    })
    .iter()
    .map(|event| match event.kind {
        EventKind::ShipDamaged { ship, .. } => ship.raw(),
        _ => unreachable!(),
    })
    .collect();
    ships.sort_unstable();
    ships.dedup();
    ships
}

// ---------------------------------------------------------------------
// The setting.
// ---------------------------------------------------------------------

#[test]
fn the_distances_are_120_208_and_174_nm_and_the_figure_is_changed_to_1() {
    let home = Hex::new(4, 0);
    assert_eq!(home.air_distance_nm(Hex::new(7, 0)), 120);
    assert_eq!(home.air_distance_nm(Hex::new(7, 3)), 208);
    assert_eq!(home.air_distance_nm(Hex::new(9, -3)), 174);
    let scenario = with_orders();
    let figures = scenario
        .world
        .ref_data()
        .air_combat
        .as_ref()
        .expect("air combat figures");
    assert_eq!(figures.strike_contact_max_age_pulses, 1);
    // The bomber's radius is 250 and its bomb is 60 damage: the three
    // contacts are in reach.
    let bomber = &scenario.world.ref_data().aircraft_types[&sim_core::ids::AircraftTypeId::new(3)];
    assert_eq!(bomber.strike_range_nm, Fixed::from_int(250));
    assert_eq!(bomber.attack_damage, Fixed::from_int(60));
}

#[test]
fn the_allied_side_starts_with_the_three_contacts_of_the_table_and_the_orders_give_the_lists() {
    let scenario = with_orders();
    let view = scenario.world.side_view(Side::Allied);
    let contacts: Vec<(u32, Hex, ShipKind, u32, u32)> = view
        .contacts()
        .map(|c| {
            (
                c.id.raw(),
                c.estimated_hex,
                c.estimated_kind,
                c.estimated_ship_count,
                c.age_pulses,
            )
        })
        .collect();
    assert_eq!(
        contacts,
        vec![
            (1, Hex::new(7, 0), ShipKind::Cruiser, 2, 0),
            (2, Hex::new(7, 3), ShipKind::Transport, 3, 0),
            (3, Hex::new(9, -3), ShipKind::Carrier, 2, 0),
        ]
    );
    // Every group is on naval strike and the lists are those of the orders,
    // once the day's orders are in.
    let world = run_seed(&scenario, 1);
    for group in GROUPS {
        let g = &world.umpire().air_groups[&AirGroupId::new(group)];
        assert_eq!(g.mission, AirMission::NavalStrike);
        assert_eq!(g.ready, 6);
        assert_eq!(g.target_priority, list_of(group), "group {group}");
        assert_eq!(g.target_order.is_some(), group != 1, "group {group}");
    }
}

// ---------------------------------------------------------------------
// The morning's decisions.
// ---------------------------------------------------------------------

#[test]
fn the_morning_launches_five_strikes_in_group_order_and_group_3_holds() {
    for world in runs(&with_orders()) {
        let found = launches(&world, Pulse::Morning);
        assert_eq!(found.len(), 5);
        for (event, (group, contact, hex, distance)) in found.iter().zip(LAUNCHES) {
            assert_eq!(
                launch_of(event),
                (group, contact, hex, 6, Fixed::from_int(distance))
            );
            // The group's mission order, its target order if it was given
            // one, and the report that made its contact.
            let mut causes = vec![Cause::Order(mission_order(&world, group))];
            if let Some(order) = target_order(&world, group) {
                causes.push(Cause::Order(order));
            }
            causes.push(Cause::Event(starting_report(&world, contact)));
            assert_eq!(event.causes, causes, "group {group}");
        }
        // Group 1 was given no target order, so cites none.
        assert_eq!(found[0].causes.len(), 2);
        assert_eq!(found[1].causes.len(), 3);
        // Nothing is recorded for group 3 in the morning. (Not "in any
        // pulse", as the hand calculation's property says: see the
        // afternoon property below.)
        assert!(events_of(&world, |e| {
            in_pulse(e, Pulse::Morning)
                && matches!(
                    e.kind,
                    EventKind::StrikeLaunched { air_group, .. } if air_group.raw() == 3
                )
        })
        .is_empty());
    }
}

#[test]
fn the_morning_attacks_are_the_ships_of_the_table_six_shots_each_at_0_2() {
    let mut hits_of = [0u32; 7];
    for world in runs(&with_orders()) {
        let found = attacks(&world, Pulse::Morning);
        assert_eq!(found.len(), 5);
        for (event, (group, force, ship)) in found.iter().zip(ATTACKS) {
            let attack = attack_of(event);
            assert_eq!(
                (attack.group, attack.force, attack.ship),
                (group, force, ship)
            );
            assert_eq!(attack.weapon, Weapon::Bomb);
            assert_eq!(attack.shots, Fixed::from_int(6));
            assert_eq!(attack.chance, Fixed::from_ratio(1, 5));
            assert_eq!(attack.expected, Fixed::from_ratio(6, 5));
            assert!(attack.hits <= 6);
            hits_of[group as usize] += attack.hits;
            // Each hit does 100 x 60 over the toughness.
            let points = points_of_attack(&world, &attack);
            assert_eq!(
                points,
                Fixed::from_int(points_per_hit(ship) * i64::from(attack.hits))
            );
        }
        // Damage is on ships 3, 4, 7 and 8 only: 5, 6 and 9 are untouched.
        let damaged = damaged_ships_in_the_morning(&world);
        assert!(damaged.iter().all(|ship| [3, 4, 7, 8].contains(ship)));
        // Nothing fires at the aircraft: none is lost, none is shot at.
        assert!(events_of(&world, |e| matches!(
            e.kind,
            EventKind::AircraftLost { .. }
                | EventKind::AirFight { .. }
                | EventKind::AntiAircraftFire { .. }
        ))
        .is_empty());
        for group in GROUPS {
            assert_eq!(world.umpire().air_groups[&AirGroupId::new(group)].ready, 6);
        }
    }
    // Each group's morning hits in all, over 1,000 seeds, are within four
    // standard deviations of 1,200: 1,077 to 1,323.
    for group in [1, 2, 4, 5, 6] {
        let total = hits_of[group];
        assert!(
            (1077..=1323).contains(&total),
            "group {group}: {total} hits in 1,000 runs"
        );
    }
    assert_eq!(hits_of[3], 0);
}

#[test]
fn every_group_of_a_raid_attacks_one_force_and_the_three_raids_are_at_three_hexes() {
    for world in runs(&with_orders()) {
        let found = launches(&world, Pulse::Morning);
        let mut hexes: Vec<Hex> = found.iter().map(|e| launch_of(e).2).collect();
        hexes.sort_unstable();
        hexes.dedup();
        assert_eq!(hexes, [Hex::new(7, 0), Hex::new(7, 3), Hex::new(9, -3)]);
        let by_hex: BTreeMap<Hex, Vec<u32>> =
            found.iter().fold(BTreeMap::new(), |mut map, event| {
                let (group, _, hex, _, _) = launch_of(event);
                map.entry(hex).or_default().push(group);
                map
            });
        assert_eq!(by_hex[&Hex::new(9, -3)], [1, 6]);
        assert_eq!(by_hex[&Hex::new(7, 3)], [2]);
        assert_eq!(by_hex[&Hex::new(7, 0)], [4, 5]);
        // The raid at (7,3) attacks force 4, for its cargo ship, though by
        // the order of importance it would be force 3, for its cruiser.
        let forces: BTreeMap<u32, u32> = attacks(&world, Pulse::Morning)
            .iter()
            .map(|e| {
                let a = attack_of(e);
                (a.group, a.force)
            })
            .collect();
        assert_eq!(forces[&2], 4);
        assert_eq!(forces[&1], forces[&6]);
        assert_eq!(forces[&4], forces[&5]);
        // Two groups of one raid attack different ships.
        let ships: BTreeMap<u32, u32> = attacks(&world, Pulse::Morning)
            .iter()
            .map(|e| {
                let a = attack_of(e);
                (a.group, a.ship)
            })
            .collect();
        assert_ne!(ships[&4], ships[&5]);
    }
}

// ---------------------------------------------------------------------
// With no target order.
// ---------------------------------------------------------------------

#[test]
fn with_no_target_order_all_six_groups_fly_as_one_raid_at_the_carrier_and_the_rule_is_as_it_was() {
    let mut expected_total = Fixed::ZERO;
    for world in runs(&without_target_orders()) {
        let found = launches(&world, Pulse::Morning);
        assert_eq!(found.len(), 6);
        for (event, group) in found.iter().zip(GROUPS) {
            assert_eq!(
                launch_of(event),
                (group, 3, Hex::new(9, -3), 6, Fixed::from_int(174))
            );
            // No launch cites a target order.
            assert_eq!(
                event.causes,
                vec![
                    Cause::Order(mission_order(&world, group)),
                    Cause::Event(starting_report(&world, 3)),
                ]
            );
            assert_eq!(target_order(&world, group), None);
        }
        let found = attacks(&world, Pulse::Morning);
        assert_eq!(found.len(), 6);
        let mut expected = Fixed::ZERO;
        for (event, group) in found.iter().zip(GROUPS) {
            let attack = attack_of(event);
            assert_eq!((attack.group, attack.force, attack.ship), (group, 5, 8));
            assert_eq!(attack.shots, Fixed::from_int(6));
            expected += attack.expected;
        }
        // 7.2 expected hits in all.
        assert_eq!(expected, Fixed::from_ratio(36, 5));
        expected_total = expected;
        // Ships 3 to 7 are untouched in the morning.
        let damaged = damaged_ships_in_the_morning(&world);
        assert!(damaged.iter().all(|ship| !(3..=7).contains(ship)));
    }
    assert_eq!(expected_total, Fixed::from_ratio(36, 5));
}

#[test]
fn giving_a_group_the_default_list_by_order_changes_nothing_but_the_cause_its_launch_cites() {
    let plain = without_target_orders();
    let ordered = with_the_default_list_by_order();
    // What a run did, without the events that only record an order.
    let story = |world: &World| -> Vec<(u32, Option<Pulse>, String)> {
        world
            .events()
            .iter()
            .filter(|e| {
                matches!(
                    e.kind,
                    EventKind::StrikeLaunched { .. }
                        | EventKind::AirAttack { .. }
                        | EventKind::ShipDamaged { .. }
                        | EventKind::StrikeFoundNothing { .. }
                        | EventKind::ShipSunk { .. }
                )
            })
            .map(|e| (e.day, e.pulse, format!("{:?}", e.kind)))
            .collect()
    };
    for seed in 1..=SEEDS {
        let a = run_seed(&plain, seed);
        let b = run_seed(&ordered, seed);
        assert_eq!(story(&a), story(&b), "seed {seed}");
        // The launches differ in their causes only: one more, the order.
        let la = events_of(&a, |e| matches!(e.kind, EventKind::StrikeLaunched { .. }));
        let lb = events_of(&b, |e| matches!(e.kind, EventKind::StrikeLaunched { .. }));
        assert_eq!(la.len(), lb.len());
        for (x, y) in la.iter().zip(&lb) {
            assert_eq!(x.causes.len() + 1, y.causes.len(), "seed {seed}");
        }
        // The state is not the same: the order is in the hash.
        assert_ne!(a.day_hashes(), b.day_hashes());
    }
}

// ---------------------------------------------------------------------
// The orders that are refused.
// ---------------------------------------------------------------------

fn target_request(group: u32, kinds: Vec<ShipKind>) -> OrderRequest {
    OrderRequest {
        kind: OrderKind::AirGroup {
            target: AirGroupId::new(group),
            order: AirGroupOrder::SetTargetPriority { kinds },
        },
        origin: Origin::PLAYER,
    }
}

#[test]
fn an_empty_list_and_a_kind_named_twice_are_refused_and_the_group_keeps_its_list() {
    let base = without_target_orders();
    let empty = target_request(2, vec![]);
    let twice = target_request(
        2,
        vec![ShipKind::Cruiser, ShipKind::Transport, ShipKind::Cruiser],
    );
    assert_eq!(
        base.world.check_order(Side::Allied, &empty),
        Err(RejectReason::TargetListEmpty)
    );
    assert_eq!(
        base.world.check_order(Side::Allied, &twice),
        Err(RejectReason::TargetKindRepeated(ShipKind::Cruiser))
    );
    // Refused when the day begins as well, and the group keeps what it had:
    // here the default, and then the list of an accepted order.
    let mut scenario = base.with_seed(1).expect("a seed");
    scenario.world.submit_orders(
        Side::Allied,
        vec![
            target_request(2, vec![ShipKind::Transport, ShipKind::Cruiser]),
            empty,
            twice,
        ],
    );
    scenario.run_to_end().expect("the run resolves");
    let reasons: Vec<RejectReason> = events_of(&scenario.world, |e| {
        matches!(e.kind, EventKind::OrderRejected { .. })
    })
    .iter()
    .map(|e| match &e.kind {
        EventKind::OrderRejected { reason, .. } => reason.clone(),
        _ => unreachable!(),
    })
    .collect();
    assert_eq!(
        reasons,
        vec![
            RejectReason::TargetListEmpty,
            RejectReason::TargetKindRepeated(ShipKind::Cruiser)
        ]
    );
    assert_eq!(
        scenario.world.umpire().air_groups[&AirGroupId::new(2)].target_priority,
        vec![ShipKind::Transport, ShipKind::Cruiser]
    );
    // Another side's group is not yours.
    assert_eq!(
        base.world.check_order(
            Side::Japanese,
            &target_request(2, vec![ShipKind::Transport])
        ),
        Err(RejectReason::NotYourUnit)
    );
}

// ---------------------------------------------------------------------
// Properties, over seeds 1 to 1000 and both pulses.
// ---------------------------------------------------------------------

/// Where a ship comes in a group's order: the kinds on its list in order,
/// then the others in the order of importance (written from decision 4 of
/// 0063, not from the engine).
fn rank(list: &[ShipKind], kind: ShipKind) -> (u8, usize) {
    match list.iter().position(|k| *k == kind) {
        Some(index) => (0, index),
        None => (
            1,
            ShipKind::ALL
                .iter()
                .position(|k| *k == kind)
                .expect("a kind"),
        ),
    }
}

#[test]
fn a_group_is_never_launched_at_a_kind_off_its_list_and_attacks_by_its_list_in_both_pulses() {
    let setup = setup();
    let mut afternoon_launches = 0;
    let mut group_3_launches = 0;
    for world in runs(&with_orders()) {
        let sunk: Vec<(EventId, u32)> =
            events_of(&world, |e| matches!(e.kind, EventKind::ShipSunk { .. }))
                .iter()
                .map(|e| match e.kind {
                    EventKind::ShipSunk { ship, .. } => (e.id, ship.raw()),
                    _ => unreachable!(),
                })
                .collect();
        for pulse in [Pulse::Morning, Pulse::Afternoon] {
            let found = launches(&world, pulse);
            if pulse == Pulse::Afternoon {
                afternoon_launches += found.len();
            }
            for event in &found {
                let (group, contact, hex, aircraft, _) = launch_of(event);
                assert!(aircraft <= 6);
                // The kind the contact was taken for when launched, by the
                // report it cites, is on the group's list.
                let report = event
                    .causes
                    .iter()
                    .find_map(|cause| match cause {
                        Cause::Event(id) => Some(*id),
                        _ => None,
                    })
                    .expect("a launch cites its report");
                let kind = match world.events().get(report).map(|e| &e.kind) {
                    Some(EventKind::ReportReceived {
                        contact: c, kind, ..
                    }) => {
                        assert_eq!(c.raw(), contact);
                        *kind
                    }
                    other => panic!("the cause is not a report: {other:?}"),
                };
                assert!(
                    list_of(group).contains(&kind),
                    "group {group} launched at a {kind:?}"
                );
                if group == 3 {
                    // A MISMATCH WITH THE HAND CALCULATION, reported to the
                    // architect and not adjusted here. Its property says
                    // group 3 is never launched. A strike that attacked
                    // sends a sighting report, and the report's kind goes
                    // through the confusion table (search.toml): a cruiser
                    // force is sometimes reported as a battleship. A
                    // contact so reported is one of group 3's list, so by
                    // decision 0063 it launches at it, in the afternoon. In
                    // the morning, on the starting contacts, it never does.
                    assert_eq!(pulse, Pulse::Afternoon);
                    assert_eq!(kind, ShipKind::Battleship);
                    group_3_launches += 1;
                }
                let _ = hex;
            }
            // The raids: strikes at one hex are one raid. Its force is the
            // one holding the best ranked ship by the list of the first
            // group, the lowest force on a tie; each group attacks the best
            // ranked ship of that force by its own list, the lowest ship
            // number on a tie.
            let attacked = attacks(&world, pulse);
            let mut by_hex: BTreeMap<Hex, Vec<u32>> = BTreeMap::new();
            for event in &found {
                let (group, _, hex, _, _) = launch_of(event);
                by_hex.entry(hex).or_default().push(group);
            }
            for (hex, members) in by_hex {
                // Ships afloat at the start of the pulse.
                let afloat = |force: u32| -> Vec<u32> {
                    setup.ships_of[&force]
                        .iter()
                        .copied()
                        .filter(|ship| {
                            !sunk
                                .iter()
                                .any(|(id, s)| s == ship && id.raw() < found[0].id.raw())
                        })
                        .collect()
                };
                let first_list = list_of(members[0]);
                let best = setup
                    .hex_of
                    .iter()
                    .filter(|(force, at)| **at == hex && **force >= 2)
                    .filter_map(|(force, _)| {
                        afloat(*force)
                            .into_iter()
                            .map(|ship| (rank(&first_list, setup.kind[&ship]), *force, ship))
                            .min()
                    })
                    .min();
                let mut seen_force = None;
                for group in &members {
                    let mine: Vec<Attack> = attacked
                        .iter()
                        .map(attack_of)
                        .filter(|a| a.group == *group)
                        .collect();
                    match best {
                        None => assert!(mine.is_empty()),
                        Some((_, force, _)) => {
                            assert_eq!(mine.len(), 1, "group {group} attacks once");
                            let attack = &mine[0];
                            // Every group of a raid attacks the same force.
                            assert_eq!(attack.force, force);
                            assert_eq!(*seen_force.get_or_insert(attack.force), attack.force);
                            let list = list_of(*group);
                            let choice = afloat(force)
                                .into_iter()
                                .map(|ship| (rank(&list, setup.kind[&ship]), ship))
                                .min()
                                .expect("a ship afloat");
                            assert_eq!(attack.ship, choice.1);
                            // A kind off the list only when no kind of the
                            // list is afloat in the force.
                            if !list.contains(&setup.kind[&attack.ship]) {
                                assert!(afloat(force)
                                    .iter()
                                    .all(|ship| !list.contains(&setup.kind[ship])));
                            }
                        }
                    }
                }
            }
        }
        // No track is above 100.
        for ship in 3..=9 {
            let ship = &world.umpire().ships[&ShipId::new(ship)];
            assert!(ship.flotation_damage <= Fixed::from_int(100));
            assert!(ship.fire_damage <= Fixed::from_int(100));
        }
        for group in GROUPS {
            assert!(world.umpire().air_groups[&AirGroupId::new(group)].ready <= 6);
        }
    }
    // The afternoon is checked as well: some runs launch in it, and in some
    // group 3 does (see the mismatch noted above).
    assert!(afternoon_launches > 0);
    assert!(group_3_launches > 0);
}

#[test]
fn a_replay_of_any_seed_reproduces_the_daily_hash() {
    let base = with_orders();
    for seed in [1, 2, 3, 77, 1000] {
        assert_eq!(
            run_seed(&base, seed).day_hashes(),
            run_seed(&base, seed).day_hashes()
        );
    }
    // The target lists are state: with none the hashes are other.
    assert_ne!(
        run_seed(&base, 1).day_hashes(),
        run_seed(&without_target_orders(), 1).day_hashes()
    );
}
