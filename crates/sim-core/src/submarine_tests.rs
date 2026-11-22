//! The submarine as a unit (decision 0218, stage 2, phase 1): her state, her
//! hash, the checks that refuse a bad one, her entering on her day and who
//! sees her. No rule of the attack is built yet, so none is tested here;
//! scenario 128 (`tests/scenario_128_submarines_against_warships.rs`) holds
//! the hand calculation of the rest.
//!
//! The world is the open sea of `pictures_tests` with a Japanese base 2 at
//! (5,0), the submarine class 9 (a salvo of 4, 2 salvoes, 10 knots, torpedo
//! 3) and boat 1 of it on station at (3,0), the Japanese side's, base 2 as
//! her home.

use crate::events::{Cause, EventKind, SubmarineLeftReason};
use crate::fixed::Fixed;
use crate::hash::hash_world;
use crate::hex::Hex;
use crate::ids::{BaseId, EntityRef, SubmarineClassId, SubmarineId, SubmarineTorpedoId};
use crate::pictures_tests::world as test_world;
use crate::refdata::{SubmarineClass, SubmarineData, SubmarineFigures, SubmarineTorpedo};
use crate::side_events::SideEventKind;
use crate::state::{Base, Side, Submarine, SubmarineCondition, SubmarinePassage, TruthState};
use crate::time::Pulse;
use crate::world::{SimError, World};

const CLASS: SubmarineClassId = SubmarineClassId::new(9);
const TORPEDO: SubmarineTorpedoId = SubmarineTorpedoId::new(3);
const HOME: BaseId = BaseId::new(2);

fn hx(q: i32, r: i32) -> Hex {
    Hex::new(q, r)
}

fn fx(text: &str) -> Fixed {
    text.parse().expect("a fixed number")
}

fn data() -> SubmarineData {
    SubmarineData {
        classes: [(
            CLASS,
            SubmarineClass {
                id: CLASS,
                name: "Test submarine class".to_string(),
                salvo: 4,
                salvoes_carried: 2,
                passage_speed_kn: Fixed::from_int(10),
                torpedo: TORPEDO,
            },
        )]
        .into(),
        torpedoes: [(
            TORPEDO,
            SubmarineTorpedo {
                id: TORPEDO,
                name: "Test torpedo".to_string(),
                damage: Fixed::from_int(200),
            },
        )]
        .into(),
        figures: SubmarineFigures {
            attacks_per_hour: fx("0.02"),
            attacks_ship_dead_in_water: Fixed::from_int(5),
            held_down_pulses: 2,
            hunt_damages_chance: fx("0.3"),
            hunt_sinks_share: fx("0.2"),
            surfaced_share: fx("0.25"),
            air_attack_damages_chance: fx("0.25"),
            air_attack_sinks_share: fx("0.2"),
            false_hit_chance: fx("0.25"),
            report_drop_age_pulses: 6,
            hit_chance: [(TORPEDO, fx("0.5"))].into(),
        },
    }
}

fn boat(raw: u32) -> Submarine {
    Submarine {
        id: SubmarineId::new(raw),
        name: format!("Test submarine {raw}"),
        side: Side::Japanese,
        class: CLASS,
        hex: hx(3, 0),
        station: hx(3, 0),
        passage: None,
        salvoes: 2,
        held_down_until: None,
        condition: SubmarineCondition::Sound,
        home: HOME,
        leave_day: None,
        enters_day: None,
        available_from: None,
        station_order: None,
    }
}

/// The parts of a world with submarine data, the Japanese base 2 and the
/// boats given.
fn parts(boats: Vec<Submarine>) -> (crate::refdata::RefData, crate::map::Map, TruthState) {
    let base = test_world();
    let mut ref_data = base.ref_data.clone();
    ref_data.submarines = Some(data());
    let mut truth = base.truth.clone();
    truth.bases.insert(
        HOME,
        Base::new(HOME, "Base 2".to_string(), Side::Japanese, hx(5, 0)),
    );
    for boat in boats {
        truth.submarines.insert(boat.id, boat);
    }
    (ref_data, base.map.clone(), truth)
}

fn world_with(boats: Vec<Submarine>) -> World {
    let (ref_data, map, truth) = parts(boats);
    World::new(7, ref_data, map, truth).expect("a valid setup")
}

fn refused(boats: Vec<Submarine>, edit: impl FnOnce(&mut crate::refdata::RefData)) -> String {
    let (mut ref_data, map, truth) = parts(boats);
    edit(&mut ref_data);
    match World::new(7, ref_data, map, truth) {
        Ok(_) => panic!("the setup was accepted"),
        Err(SimError::InvalidSetup(words)) => words,
        Err(other) => other.to_string(),
    }
}

#[test]
fn a_boat_is_a_unit_of_her_own_kind_with_her_own_code() {
    let id = SubmarineId::new(5);
    assert_eq!(id.to_string(), "submarine:5");
    let entity = EntityRef::from(id);
    assert_eq!(entity.kind_code(), 10);
    assert_eq!(entity.raw_id(), 5);
    assert_eq!(entity.to_string(), "submarine:5");
    // The codes of the condition are stable.
    let codes: Vec<u8> = [
        SubmarineCondition::Sound,
        SubmarineCondition::Damaged,
        SubmarineCondition::Sunk,
        SubmarineCondition::Home,
    ]
    .iter()
    .map(|condition| condition.code())
    .collect();
    assert_eq!(codes, vec![1, 2, 3, 4]);
}

#[test]
fn a_boat_is_on_the_map_unless_off_it_or_held_apart_and_on_station_when_she_has_no_passage() {
    let mut one = boat(1);
    assert!(one.on_map(1) && one.on_station(1));
    one.enters_day = Some(3);
    assert!(!one.on_map(1) && !one.on_map(2) && !one.on_station(2));
    assert!(one.on_map(3) && one.on_station(3));
    one.passage = Some(SubmarinePassage {
        to: hx(6, 0),
        home: false,
        toward_hex: None,
        progress_nm: Fixed::ZERO,
        leg_nm: Fixed::ZERO,
        arrived: false,
        order: None,
    });
    assert!(one.on_map(3) && !one.on_station(3));
    one.passage = None;
    one.hex = hx(4, 0);
    assert!(!one.on_station(3));
    one.hex = hx(3, 0);
    for off in [SubmarineCondition::Sunk, SubmarineCondition::Home] {
        one.condition = off;
        assert!(!one.on_map(9) && !one.on_station(9));
    }
}

#[test]
fn a_game_with_no_boat_hashes_as_it_did_and_a_boat_changes_the_hash() {
    let plain = world_with(Vec::new());
    let with = world_with(vec![boat(1)]);
    assert_ne!(hash_world(&plain), hash_world(&with));
    // Two worlds alike in every boat hash alike.
    assert_eq!(hash_world(&with), hash_world(&world_with(vec![boat(1)])));
    // A boat's name is not hashed; each other field is.
    let mut renamed = boat(1);
    renamed.name = "Another name".to_string();
    assert_eq!(hash_world(&with), hash_world(&world_with(vec![renamed])));
    type Edit = Box<dyn Fn(&mut Submarine)>;
    let edits: Vec<Edit> = vec![
        Box::new(|b| b.salvoes = 1),
        Box::new(|b| b.hex = hx(4, 0)),
        Box::new(|b| b.station = hx(4, 0)),
        Box::new(|b| b.condition = SubmarineCondition::Damaged),
        Box::new(|b| b.held_down_until = Some((1, Pulse::Morning))),
        Box::new(|b| b.leave_day = Some(2)),
        Box::new(|b| b.enters_day = Some(2)),
        Box::new(|b| b.available_from = Some(2)),
        Box::new(|b| b.side = Side::Allied),
        Box::new(|b| {
            b.passage = Some(SubmarinePassage {
                to: hx(3, 0),
                home: true,
                toward_hex: None,
                progress_nm: Fixed::ZERO,
                leg_nm: Fixed::ZERO,
                arrived: false,
                order: None,
            })
        }),
    ];
    let mut seen = vec![hash_world(&with)];
    for edit in edits {
        let mut edited = boat(1);
        edit(&mut edited);
        // Only a boat whose hex is her station (or who is on passage) is a
        // sound boat, so the hash is taken on the state, not on a world.
        let mut truth = with.truth.clone();
        truth.submarines.insert(edited.id, edited);
        let mut world = world_with(vec![boat(1)]);
        world.truth = truth;
        let hash = hash_world(&world);
        assert!(!seen.contains(&hash), "an edit left the hash as it was");
        seen.push(hash);
    }
}

#[test]
fn a_setup_with_a_bad_boat_is_refused_with_the_reason() {
    let mut unknown = boat(1);
    unknown.class = SubmarineClassId::new(77);
    assert!(refused(vec![unknown], |_| {}).contains("unknown submarine class"));

    let mut off = boat(1);
    off.station = hx(50, 0);
    assert!(refused(vec![off], |_| {}).contains("station"));
    let mut off = boat(1);
    off.hex = hx(50, 0);
    assert!(refused(vec![off], |_| {}).contains("hex"));

    let mut homeless = boat(1);
    homeless.home = BaseId::new(99);
    assert!(refused(vec![homeless], |_| {}).contains("home base"));

    let high = boat(1_000_000);
    assert!(refused(vec![high], |_| {}).contains("1,000,000"));

    assert!(refused(vec![boat(1)], |data| data.submarines = None).contains("no submarine data"));

    // A boat with more salvoes than her class carries breaks an invariant.
    let mut heavy = boat(1);
    heavy.salvoes = 3;
    let words = refused(vec![heavy], |_| {});
    assert!(
        words.contains("more salvoes than her class carries"),
        "{words}"
    );

    // On no passage she is at her station.
    let mut adrift = boat(1);
    adrift.hex = hx(4, 0);
    let words = refused(vec![adrift], |_| {});
    assert!(words.contains("not at her station"), "{words}");

    // The data itself: a class whose torpedo has no hit chance.
    let words = refused(vec![boat(1)], |data| {
        data.submarines
            .as_mut()
            .expect("data")
            .figures
            .hit_chance
            .clear();
    });
    assert!(words.contains("no hit chance"), "{words}");
}

#[test]
fn the_figures_are_checked_for_chances_and_ages() {
    let mut figures = data().figures;
    assert!(figures.validate().is_ok());
    figures.hunt_sinks_share = fx("1.5");
    assert!(figures.validate().is_err());
    let mut figures = data().figures;
    figures.attacks_ship_dead_in_water = fx("-1");
    assert!(figures.validate().is_err());
    let mut figures = data().figures;
    figures.report_drop_age_pulses = 0;
    assert!(figures.validate().is_err());
}

#[test]
fn a_boat_held_apart_enters_first_thing_on_her_day_and_her_side_alone_is_told() {
    let mut late = boat(1);
    late.enters_day = Some(2);
    late.available_from = Some(2);
    let mut world = world_with(vec![late]);
    let entered = |world: &World| -> Vec<(u32, Option<Pulse>)> {
        world
            .events()
            .from_index(0)
            .iter()
            .filter(|event| matches!(event.kind, EventKind::SubmarineEntered { .. }))
            .map(|event| (event.day, event.pulse))
            .collect()
    };
    world.resolve_day().expect("day 1");
    assert!(entered(&world).is_empty());
    // On day 1 she is in no hex: her day is 2.
    assert!(!world.truth.submarines[&SubmarineId::new(1)].on_map(1));
    world.resolve_day().expect("day 2");
    assert_eq!(entered(&world), vec![(2, None)]);
    let event = world
        .events()
        .from_index(0)
        .iter()
        .find(|event| matches!(event.kind, EventKind::SubmarineEntered { .. }))
        .cloned()
        .expect("her entering");
    assert!(event.causes.is_empty());
    assert_eq!(
        event.kind,
        EventKind::SubmarineEntered {
            submarine: SubmarineId::new(1),
            hex: hx(3, 0),
        }
    );
    assert_eq!(
        event.kind.entities(),
        vec![EntityRef::Submarine(SubmarineId::new(1))]
    );
    let names = |side: Side| -> Vec<String> {
        world
            .side_view(side)
            .events()
            .iter()
            .filter_map(|event| match &event.kind {
                SideEventKind::Whole(EventKind::SubmarineEntered { submarine, hex }) => {
                    Some(format!("{submarine} at {hex}"))
                }
                _ => None,
            })
            .collect()
    };
    assert_eq!(
        names(Side::Japanese),
        vec!["submarine:1 at (3,0)".to_string()]
    );
    assert!(names(Side::Allied).is_empty());
}

#[test]
fn a_side_sees_its_own_boats_exactly_and_no_enemy_boat() {
    let mut late = boat(2);
    late.enters_day = Some(2);
    let allied = Submarine {
        side: Side::Allied,
        home: BaseId::new(1),
        ..boat(3)
    };
    let (ref_data, map, mut truth) = parts(vec![boat(1), late, allied]);
    truth.bases.insert(
        BaseId::new(1),
        Base::new(BaseId::new(1), "Base 1".to_string(), Side::Allied, hx(0, 0)),
    );
    let world = World::new(7, ref_data, map, truth).expect("a valid setup");
    let ids = |side: Side| -> Vec<u32> {
        world
            .side_view(side)
            .own_submarines()
            .map(|boat| boat.id.raw())
            .collect()
    };
    // The Japanese side's include the one held apart, as its land units
    // held apart are shown.
    assert_eq!(ids(Side::Japanese), vec![1, 2]);
    assert_eq!(ids(Side::Allied), vec![3]);
    let japanese = world.side_view(Side::Japanese);
    assert_eq!(
        japanese.submarine(SubmarineId::new(1)).map(|b| b.salvoes),
        Some(2)
    );
    assert!(japanese.submarine(SubmarineId::new(3)).is_none());
    assert!(world
        .side_view(Side::Allied)
        .submarine(SubmarineId::new(1))
        .is_none());
    // Her day, as the side's view gives it: the day she enters.
    assert_eq!(
        japanese.available_from(EntityRef::Submarine(SubmarineId::new(2))),
        Some(2)
    );
    assert_eq!(
        world
            .side_view(Side::Allied)
            .available_from(EntityRef::Submarine(SubmarineId::new(2))),
        None
    );
}

#[test]
fn a_half_of_a_picture_holds_the_sides_own_boats_and_the_joined_picture_both() {
    let allied = Submarine {
        side: Side::Allied,
        home: BaseId::new(1),
        ..boat(3)
    };
    let (ref_data, map, mut truth) = parts(vec![boat(1), allied]);
    truth.bases.insert(
        BaseId::new(1),
        Base::new(BaseId::new(1), "Base 1".to_string(), Side::Allied, hx(0, 0)),
    );
    let world = World::new(7, ref_data, map, truth).expect("a valid setup");
    let japanese = world
        .side_view(Side::Japanese)
        .own_half()
        .expect("its half");
    let allied = world.side_view(Side::Allied).own_half().expect("its half");
    assert_eq!(
        japanese
            .submarines
            .keys()
            .map(|id| id.raw())
            .collect::<Vec<_>>(),
        vec![1]
    );
    assert_eq!(
        allied
            .submarines
            .keys()
            .map(|id| id.raw())
            .collect::<Vec<_>>(),
        vec![3]
    );
    let picture =
        crate::world_picture::WorldPicture::from_halves(japanese, allied).expect("the halves join");
    assert_eq!(picture.truth.submarines.len(), 2);
}

// ---- the boat's passage (decision 0218, 3, 21 and 43) ----
//
// The class steams 10 knots on a passage: 120 nm in a night, 60 in a morning
// or an afternoon, a hex 40 nm. She enters a hex 20 nm before its centre.

fn passage_to(to: Hex, home: bool) -> SubmarinePassage {
    SubmarinePassage {
        to,
        home,
        toward_hex: None,
        progress_nm: Fixed::ZERO,
        leg_nm: Fixed::ZERO,
        arrived: false,
        order: None,
    }
}

/// The movement step of a pulse, with the boat's passage in it.
fn step(world: &mut World, day: u32, pulse: Pulse) {
    world.move_task_forces(day, pulse);
}

fn boat_of(world: &World) -> &Submarine {
    &world.truth.submarines[&SubmarineId::new(1)]
}

/// The passage events of boat 1 from event `from` on: (kind, pulse).
fn passage_events(world: &World, from: usize) -> Vec<(String, Option<Pulse>)> {
    world
        .events()
        .from_index(from)
        .iter()
        .filter_map(|event| {
            let word = match &event.kind {
                EventKind::SubmarineSteamed { from, to, .. } => format!("steamed {from} {to}"),
                EventKind::SubmarineOnStation { hex, .. } => format!("on station {hex}"),
                EventKind::SubmarineLeftForHome { reason, base, .. } => {
                    format!("left for {base} {}", reason.code())
                }
                EventKind::SubmarineReachedHome { base, .. } => format!("reached {base}"),
                _ => return None,
            };
            Some((word, event.pulse))
        })
        .collect()
}

#[test]
fn a_boat_on_passage_reaches_her_station_at_10_knots_and_is_on_station_from_the_next_pulse() {
    // From (3,0) to (9,0): 240 nm. The night's 120 take her to the centre of
    // (6,0); the morning's 60 to the moment she enters (8,0), half way along
    // the leg to (8,0)'s centre; the afternoon's 60 to the centre of (9,0).
    let mut one = boat(1);
    one.station = hx(9, 0);
    one.passage = Some(passage_to(hx(9, 0), false));
    let mut world = world_with(vec![one]);
    let at = |world: &World| {
        let boat = boat_of(world);
        (
            boat.hex,
            boat.passage.and_then(|p| p.toward_hex),
            boat.on_station(1),
        )
    };

    step(&mut world, 1, Pulse::Night);
    assert_eq!(at(&world), (hx(6, 0), None, false));
    // A leg is entered at half its charge: 20 nm along, she is in (7,0).
    step(&mut world, 1, Pulse::Morning);
    assert_eq!(at(&world), (hx(8, 0), Some(hx(8, 0)), false));
    assert_eq!(
        boat_of(&world).passage.map(|p| (p.progress_nm, p.leg_nm)),
        Some((Fixed::from_int(20), Fixed::from_int(40)))
    );
    assert!(!boat_of(&world).passage.expect("passage").arrived);
    step(&mut world, 1, Pulse::Afternoon);
    // She has arrived: the passage stands, marked, for the rest of the pulse.
    let passage = boat_of(&world).passage.expect("passage");
    assert!(passage.arrived);
    assert_eq!(boat_of(&world).hex, hx(9, 0));
    assert!(!boat_of(&world).on_station(1));
    assert_eq!(
        passage_events(&world, 0),
        vec![
            ("steamed (3,0) (6,0)".to_string(), Some(Pulse::Night)),
            ("steamed (6,0) (8,0)".to_string(), Some(Pulse::Morning)),
            ("steamed (8,0) (9,0)".to_string(), Some(Pulse::Afternoon)),
            ("on station (9,0)".to_string(), Some(Pulse::Afternoon)),
        ]
    );
    // From the next pulse she is on station and steams no more.
    let before = world.events().len();
    step(&mut world, 2, Pulse::Night);
    assert!(boat_of(&world).passage.is_none());
    assert!(boat_of(&world).on_station(2));
    assert!(passage_events(&world, before).is_empty());
    assert!(crate::invariants::check(&world.truth, &world.map, &world.ref_data).is_empty());
}

#[test]
fn a_boat_going_home_on_her_leave_day_is_taken_off_the_map_at_her_base() {
    // Base 2 is at (5,0), 80 nm from (3,0): she gets there in the night of
    // her leave day, the second.
    let mut one = boat(1);
    one.leave_day = Some(2);
    let mut world = world_with(vec![one]);
    step(&mut world, 1, Pulse::Night);
    step(&mut world, 1, Pulse::Morning);
    step(&mut world, 1, Pulse::Afternoon);
    assert!(boat_of(&world).passage.is_none());
    assert_eq!(boat_of(&world).hex, hx(3, 0));
    assert!(passage_events(&world, 0).is_empty());

    step(&mut world, 2, Pulse::Night);
    let events: Vec<_> = world.events().iter().cloned().collect();
    let kinds: Vec<_> = events
        .iter()
        .filter(|event| {
            matches!(
                &event.kind,
                EventKind::SubmarineLeftForHome { .. }
                    | EventKind::SubmarineSteamed { .. }
                    | EventKind::SubmarineReachedHome { .. }
            )
        })
        .collect();
    assert_eq!(kinds.len(), 3);
    assert_eq!(
        kinds[0].kind,
        EventKind::SubmarineLeftForHome {
            submarine: SubmarineId::new(1),
            base: HOME,
            reason: SubmarineLeftReason::LeaveDay,
        }
    );
    assert!(kinds[0].causes.is_empty());
    assert_eq!(
        kinds[1].kind,
        EventKind::SubmarineSteamed {
            submarine: SubmarineId::new(1),
            from: hx(3, 0),
            to: hx(5, 0),
        }
    );
    // Her steaming cites what sent her; her arrival cites her steaming.
    assert_eq!(kinds[1].causes, vec![Cause::Event(kinds[0].id)]);
    assert_eq!(
        kinds[2].kind,
        EventKind::SubmarineReachedHome {
            submarine: SubmarineId::new(1),
            base: HOME,
        }
    );
    assert_eq!(kinds[2].causes, vec![Cause::Event(kinds[1].id)]);
    let gone = boat_of(&world);
    assert_eq!(gone.condition, SubmarineCondition::Home);
    assert!(gone.passage.is_none());
    assert!(!gone.on_map(2));
    // Off the map she does nothing more.
    let before = world.events().len();
    step(&mut world, 2, Pulse::Morning);
    step(&mut world, 3, Pulse::Night);
    assert!(passage_events(&world, before).is_empty());
    assert!(crate::invariants::check(&world.truth, &world.map, &world.ref_data).is_empty());
    // Her side is told of it, whole; the enemy is not.
    let told = |side: Side| {
        world
            .side_view(side)
            .events()
            .iter()
            .filter(|event| {
                matches!(
                    event.kind,
                    SideEventKind::Whole(EventKind::SubmarineLeftForHome { .. })
                        | SideEventKind::Whole(EventKind::SubmarineSteamed { .. })
                        | SideEventKind::Whole(EventKind::SubmarineReachedHome { .. })
                )
            })
            .count()
    };
    assert_eq!(told(Side::Japanese), 3);
    assert_eq!(told(Side::Allied), 0);
}

#[test]
fn a_boat_that_enters_after_her_leave_day_leaves_in_her_first_pulse_and_a_damaged_one_has_no_event()
{
    let mut late = boat(1);
    late.leave_day = Some(1);
    late.enters_day = Some(3);
    let mut world = world_with(vec![late]);
    step(&mut world, 1, Pulse::Night);
    step(&mut world, 2, Pulse::Night);
    assert!(passage_events(&world, 0).is_empty());
    step(&mut world, 3, Pulse::Night);
    assert_eq!(
        passage_events(&world, 0)
            .into_iter()
            .map(|(word, _)| word)
            .collect::<Vec<_>>(),
        vec![
            "left for base:2 1".to_string(),
            "steamed (3,0) (5,0)".to_string(),
            "reached base:2".to_string(),
        ]
    );

    // A boat loaded damaged already goes home: no event of leaving, her
    // steaming cites nothing.
    let mut hurt = boat(1);
    hurt.condition = SubmarineCondition::Damaged;
    hurt.passage = Some(passage_to(hx(5, 0), true));
    let mut world = world_with(vec![hurt]);
    step(&mut world, 1, Pulse::Night);
    assert_eq!(
        passage_events(&world, 0)
            .into_iter()
            .map(|(word, _)| word)
            .collect::<Vec<_>>(),
        vec![
            "steamed (3,0) (5,0)".to_string(),
            "reached base:2".to_string()
        ]
    );
    assert_eq!(boat_of(&world).condition, SubmarineCondition::Home);
}

#[test]
fn send_home_finishes_the_leg_she_is_on_and_does_nothing_for_a_boat_already_going_home() {
    let mut one = boat(1);
    one.station = hx(9, 0);
    one.passage = Some(passage_to(hx(9, 0), false));
    let mut world = world_with(vec![one]);
    // The night takes her to the centre of (6,0); the morning's 60 nm to the
    // moment she enters (8,0), half way along its leg.
    step(&mut world, 1, Pulse::Night);
    step(&mut world, 1, Pulse::Morning);
    let id = SubmarineId::new(1);
    let toward = boat_of(&world).passage.expect("passage").toward_hex;
    assert_eq!(toward, Some(hx(8, 0)));
    let sent = world.send_home(
        1,
        Pulse::Morning,
        id,
        SubmarineLeftReason::Damaged,
        Vec::new(),
    );
    assert!(sent.is_some());
    let passage = boat_of(&world).passage.expect("passage");
    assert!(passage.home);
    assert_eq!(passage.to, hx(5, 0));
    // The leg to (8,0) is finished first, then the way back to (5,0).
    assert_eq!(passage.toward_hex, Some(hx(8, 0)));
    assert!(world
        .send_home(
            1,
            Pulse::Morning,
            id,
            SubmarineLeftReason::NoSalvo,
            Vec::new()
        )
        .is_none());
    step(&mut world, 1, Pulse::Afternoon);
    assert_eq!(boat_of(&world).hex, hx(7, 0));
    step(&mut world, 2, Pulse::Night);
    assert_eq!(boat_of(&world).condition, SubmarineCondition::Home);
    assert_eq!(boat_of(&world).hex, hx(5, 0));
    assert_eq!(
        [
            SubmarineLeftReason::LeaveDay.code(),
            SubmarineLeftReason::Damaged.code(),
            SubmarineLeftReason::NoSalvo.code()
        ],
        [1, 2, 3]
    );
}
