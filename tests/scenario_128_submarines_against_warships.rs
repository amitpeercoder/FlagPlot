//! Scenario 128: submarines against warships. A boat on her station, the
//! forces that pass through her hex, her salvo, the hunt, aircraft that
//! find her, and what each side is told.
//!
//! Proves decision 0218 as amended on its second review (the section
//! "Amended on a second review", which governs, with rules 1 to 26 where
//! it leaves them standing). The data is placeholder data: every boat,
//! ship, aircraft and figure here is made up for the proof and named
//! "Placeholder". It is several small games, one for each part, so that
//! no part's numbers rest on another's draws.
//!
//! # Hand calculation
//!
//! Written by the architect before the code. Everything stated is
//! asserted exactly. If the code disagrees, the code or the rule is
//! wrong: do not change a statement here to make a test pass. A mismatch
//! goes back to the architect.
//!
//! Every quantity is a fixed-point number of six decimal places and every
//! multiplication and division is cut toward zero at the sixth place, as
//! `Fixed` does. The numbers below are worked so.
//!
//! ## The figures
//!
//! The scenario's own, placeholders all:
//!
//! | Figure | Value |
//! | --- | --- |
//! | `submarine_attacks_per_hour` | 0.02 |
//! | `attacks_ship_dead_in_water` | 5 |
//! | `held_down_pulses` | 2 |
//! | `hunt_damages_chance` | 0.3 |
//! | `hunt_sinks_share` | 0.2 |
//! | `submarine_surfaced_share` | 0.25 |
//! | `air_attack_damages_chance` | 0.25 |
//! | `air_attack_sinks_share` | 0.2 |
//!
//! The placeholder boat class: a salvo of 4, 2 salvoes carried, a passage
//! speed of 10 knots, the placeholder torpedo. The placeholder torpedo: a
//! `submarine_hit_chance` of 0.5 and the damage of scenario 4a's torpedo,
//! so that a hit is worked exactly as that scenario works one.
//!
//! A hex is 40 nm. The night pulse is 12 hours, the morning and the
//! afternoon 6 each.
//!
//! ## Part A: the hours and the chance to attack
//!
//! Boat 1, Japanese, on her station at (5,0), sound, with both salvoes.
//! One Allied task force in each game, of one placeholder carrier and one
//! placeholder destroyer, neither dead in the water. The hourly chance is
//! p = 0.02 and its miss q = 0.98. The miss for a pulse is q multiplied by
//! itself once for each whole hour, then by 1 - p x f for the part f of an
//! hour left over; the chance is one less that.
//!
//! | Game | The force in the pulse | Hours in (5,0) | Miss | Chance |
//! | --- | --- | --- | --- | --- |
//! | A1 | lies stopped at (5,0) all the night | 12 | 0.784713 | 0.215287 |
//! | A2 | lies stopped at (5,0) all the morning | 6 | 0.885841 | 0.114159 |
//! | A3 | crosses (5,0) at 30 knots, entering and leaving in the morning | 40/30 = 1.333333 | 0.973467 | 0.026533 |
//! | A4 | crosses at 20 knots in the morning | 2 | 0.960400 | 0.039600 |
//! | A5 | crosses at 15 knots in the morning | 40/15 = 2.666666 | 0.947594 | 0.052406 |
//! | A6 | arrives at (5,0) with 3 hours of the morning left and stops | 3 | 0.941192 | 0.058808 |
//! | A7 | passes by (4,0), (5,-1), (6,-1) and (6,0) and never enters (5,0) | 0 | none | no draw |
//!
//! Worked: 0.98 squared is 0.960400; cubed 0.941192; the sixth power
//! 0.885841 and the twelfth 0.784713, each product cut at the sixth
//! place. A3: f = 0.333333, p x f = 0.006666, 1 - 0.006666 = 0.993334, and
//! 0.98 x 0.993334 = 0.973467. A5: f = 0.666666, p x f = 0.013333,
//! 0.960400 x 0.986667 = 0.947594.
//!
//! Statements:
//!
//! 1. In each of A1 to A6 exactly one attack chance is recorded for boat
//!    1 and the force in that pulse, with the hours and the chance of the
//!    table. In A7 none is recorded and no draw of the attack is made.
//! 2. The force's hours in a hex are from when it enters to when it
//!    leaves: half a leg before the hex's centre and half after. In A3 the
//!    force is in (5,0) for one hex's steaming, not two.
//! 3. A ship dead in the water multiplies the hourly chance by 5: p =
//!    0.1. Game A8, the force of A2 with its carrier dead in the water:
//!    miss 0.9 to the sixth, 0.531441; chance 0.468559. Game A9, the same
//!    all the night: miss 0.282428 (0.9 to the twelfth, cut at each
//!    product); chance 0.717572.
//! 4. A boat that is held down, on passage or damaged makes no draw. Games
//!    A10 to A12, the force of A1 with the boat in each state: no attack
//!    chance is recorded. A boat without a salvo is never on station: she
//!    leaves for home in the pulse she fires her last (statement 11), and a
//!    scenario may not give her none (scenario 129, statement 20).
//! 5. A force lying in a hex that holds a base of its own side is not
//!    attacked. Game A14, A1 with an Allied base at (5,0): none recorded.
//! 6. A force with none of a carrier, a battleship, a transport, an
//!    auxiliary or a cruiser is not attacked. Game A15, A1 with two
//!    destroyers only: none recorded.
//!
//! ## Part B: which force, which ship, the salvo
//!
//! 7. Of several forces she attacks one. Game B1: three Allied forces lie
//!    at (5,0) all the night, force 1 of two cruisers, force 2 of a
//!    transport and a destroyer, force 3 of a battleship and a cruiser.
//!    One attack chance is recorded for each, 0.215287, and each has its
//!    own draw. On a seed on which more than one draw succeeds she fires
//!    at force 3 where its draw succeeded, else at force 2, else at force
//!    1: battleship before transport before cruiser. She fires once.
//! 8. She aims at one ship: the first by the order of kinds, a ship dead
//!    in the water before another of its kind, then the lowest number.
//!    Game B2: a force of carriers 1 and 2, carrier 2 dead in the water,
//!    and a battleship: on every seed on which she fires, at carrier 2.
//! 9. A salvo is 4 torpedoes, each its own draw of the boat, the ship and
//!    its place in the salvo, each hitting at 0.5. Over the seeds on which
//!    she fires in game A1 the hits on the aimed ship run from 0 to 4 and
//!    no other ship is ever hit. Her salvoes go from 2 to 1.
//! 10. Each hit is worked as scenario 4a works a torpedo hit, with the
//!    damage of the placeholder torpedo.
//! 11. With one salvo left she fires again at a later pulse and then has
//!    none: she leaves for the base her scenario names, at 10 knots, and
//!    is taken off the map there. She attacks nothing on the way.
//!
//! ## Part C: the hunt
//!
//! 12. A force with a warship that is not the ship aimed at hunts her,
//!    hit or miss. In game A1 on every seed on which she fires she is held
//!    down for the rest of that pulse and the 2 pulses after: no attack
//!    chance is recorded for her in those pulses, and one is recorded in
//!    the third.
//! 13. The hunt is one stream of the boat and the force: a first draw
//!    under 0.3 damages her; of those, a second under 0.2 sinks her. Over
//!    the seeds the test runs each of the three ends happens: sound,
//!    damaged, sunk. The chance she is sunk is 0.3 x 0.2 = 0.06; that she
//!    is damaged and afloat 0.24; untouched 0.7.
//! 14. Damaged, she leaves for home and attacks nothing. Sunk, she is
//!    gone at once and her side is told in that pulse.
//! 15. A force of one ship, the ship aimed at, hunts nothing. Game C1: a
//!    lone placeholder cruiser lies at (5,0): on every seed on which she
//!    fires she is not held down and may fire again in the next pulse.
//!
//! ## Part D: aircraft
//!
//! Game D1. Allied air group 1 at a base at (0,0): 6 placeholder search
//! aircraft, a sweep width of 20 nm, on the search mission with an arc of
//! 60 degrees that holds (5,0), 200 nm away. No ship of either side but
//! the boat.
//!
//! The chance of sighting one force there is 57.295779 x 6 x 20 / (200 x
//! 60) = 6875.493480 / 12000 = 0.572957.
//!
//! 16. Boat 1 on her station at (5,0): in each daylight pulse one chance
//!    is recorded for the boat and the group, 0.572957 x 0.25 = 0.143239.
//!    None is recorded in the night pulse.
//! 17. Boat 1 on passage through (5,0): the chance is 0.572957 whole.
//! 18. Boat 1 on her station at (0,0), the group's own hex: the distance
//!    is taken as one hex, 40 nm, so the chance of sighting a force is
//!    6875.493480 / 2400, cut to 1, and the boat's is 0.25. It is not 1.
//! 19. Sighted, she is reported: the Allied side gains one report of a
//!    submarine, in its own list, at her hex or, where the error of a
//!    report struck, a neighbour. Its contacts of ships are unchanged and
//!    no force of its side is alerted by it.
//! 20. Sighted, she is held down for the rest of that pulse and the next,
//!    and the aircraft attack her in the same pulse: one stream of the
//!    boat and the group, a first draw under 0.25 damages her, of those a
//!    second under 0.2 sinks her. Sunk 0.05, damaged and afloat 0.2,
//!    untouched 0.75. No aircraft is lost, no bomb is used and no sortie
//!    is added to the group's count for the day.
//!
//! ## Part E: what each side is told
//!
//! 21. Of her salvo her own side is told that she fired, the kind she
//!    took her target for, and her claim. It is given no contact of the
//!    force and no report of ships.
//! 22. The side fired on is told of each salvo, hit or miss, with no boat
//!    named and no number of hers, and gains one report of a submarine at
//!    her hex through the report's error.
//! 23. Two reports of a submarine in one hex are one, the newer. A report
//!    of "nothing there" for that hex removes no report of a submarine.
//! 24. A side's view of its own boats is exact; the other side's view
//!    holds no boat, only its reports of submarines. No event of the boat's
//!    passage, station, salvoes, state or held-down count is seen by the
//!    other side.
//! 25. A report of a submarine ages and is dropped by its own figures,
//!    and is in every command's picture where command levels are on.
//!
//! ## Part F: suspected waters
//!
//! 26. Game F1: the Allied side is given suspected waters, (5,0) and 2
//!    hexes round, told on day 2. On day 1 its briefing and view hold
//!    none; from day 2 they hold them, with the words of the belief. The
//!    Japanese side, given none, holds none. The waters do not move when a
//!    boat is reported elsewhere.
//!
//! ## Part G: nothing else moves
//!
//! 27. A scenario with no boat and no suspected waters has the hash it
//!    had: scenario 4a and scenario 5b give their hashes on file.
//! 28. No rule made for a task force touches a boat: in a game where an
//!    Allied surface force with battle readiness lies in her hex all a
//!    night and she does not fire, no surface fight is recorded; where a
//!    carrier's groups on naval strike are in reach of her report, no
//!    strike is launched at it.
//! 29. Every draw of this rule has its own `Draw` value in a subsystem of
//!    its own, and two games that differ only in a boat that never meets a
//!    force give every other draw the same.
//!
//! The station order, holds on a boat, the patrol order, the staff's
//! regard for submarine waters and the table are the third stage's and
//! are proved there.

// The hand calculation above is the architect's text, kept as written.
#![allow(clippy::doc_lazy_continuation)]

use crate::common;

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::PathBuf;
use std::sync::OnceLock;

use sim_core::events::{Event, EventKind, SubmarineLeftReason};
use sim_core::fixed::Fixed;
use sim_core::ids::{EntityRef, EventId, ShipId, SubmarineId, TaskForceId};
use sim_core::pictures::CommandRef;
use sim_core::refdata::ShipKind;
use sim_core::rng::{Draw, Subsystem};
use sim_core::side_events::SideEventKind;
use sim_core::state::{Side, SubmarineCondition};
use sim_core::time::Pulse;
use sim_core::world::{SimConfig, World};
use sim_data::{LoadOptions, Scenario};

const SEEDS: u64 = 1000;

fn scenario_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../data/scenarios/128_submarines_against_warships.toml")
}

fn fx(text: &str) -> Fixed {
    text.parse().expect("a fixed number")
}

/// The scenario's map, with its strip of sea made longer to the west so that
/// a force can come to the boat's hex from far enough away to arrive in the
/// morning. Written once, to the build directory; the scenario's own map
/// file is not touched.
fn wide_map() -> &'static str {
    static MAP: OnceLock<String> = OnceLock::new();
    MAP.get_or_init(|| {
        let original =
            fs::read_to_string(common::data_dir().join("map/placeholder_submarines.toml"))
                .expect("read the map");
        let wide = common::replace_once(&original, "q = [-1, 12]", "q = [-10, 12]");
        let path = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join("scenario_128_wide_map.toml");
        fs::write(&path, wide).expect("write the wide map");
        path.to_string_lossy().into_owned()
    })
}

// ---------------------------------------------------------------------
// The games. Each is the scenario's head (the map, the figures, the base
// and boat 1) with the forces of the game after it.
// ---------------------------------------------------------------------

#[derive(Clone)]
struct Ship {
    id: u32,
    class: u32,
    out: bool,
}

const CRUISER: u32 = 1;
const DESTROYER: u32 = 2;
const TRANSPORT: u32 = 3;
const CARRIER: u32 = 4;
/// A placeholder battleship, class 9, added to a copy of the frozen classes
/// (the frozen ones hold none): the cruiser's figures, the battleship's kind.
const BATTLESHIP: u32 = 9;

fn ship(id: u32, class: u32) -> Ship {
    Ship {
        id,
        class,
        out: false,
    }
}

fn dead(id: u32, class: u32) -> Ship {
    Ship {
        id,
        class,
        out: true,
    }
}

#[derive(Clone)]
struct Force {
    id: u32,
    at: (i32, i32),
    ships: Vec<Ship>,
    route: Vec<(i32, i32)>,
    knots: u32,
}

fn force(id: u32, ships: Vec<Ship>) -> Force {
    Force {
        id,
        at: (5, 0),
        ships,
        route: Vec::new(),
        knots: 0,
    }
}

fn carrier_and_destroyer() -> Force {
    force(1, vec![ship(1, CARRIER), ship(2, DESTROYER)])
}

impl Force {
    fn steams(mut self, from: (i32, i32), route: &[(i32, i32)], knots: u32) -> Force {
        self.at = from;
        self.route = route.to_vec();
        self.knots = knots;
        self
    }
}

#[derive(Clone)]
struct Game {
    forces: Vec<Force>,
    /// Lines added to boat 1's entry.
    boat: String,
    days: u32,
    allied_base_at_the_boat: bool,
    wide_map: bool,
    /// Command levels on, with no report figures: every command holds a
    /// report at once.
    command_levels: bool,
}

fn game(forces: Vec<Force>) -> Game {
    Game {
        forces,
        boat: String::new(),
        days: 1,
        allied_base_at_the_boat: false,
        wide_map: false,
        command_levels: false,
    }
}

impl Game {
    fn boat(mut self, lines: &str) -> Game {
        self.boat = format!("{lines}\n");
        self
    }
    fn days(mut self, days: u32) -> Game {
        self.days = days;
        self
    }
    fn wide(mut self) -> Game {
        self.wide_map = true;
        self
    }
    fn base(mut self) -> Game {
        self.allied_base_at_the_boat = true;
        self
    }
    fn command_levels(mut self) -> Game {
        self.command_levels = true;
        self
    }

    fn text(&self) -> String {
        let text = fs::read_to_string(scenario_path()).expect("read the scenario");
        let marker = "# The Allied force lies stopped at (5,0) all the night.";
        let mut text = text
            .split(marker)
            .next()
            .expect("the head")
            .trim_end()
            .to_string();
        text.push('\n');
        text = common::replace_once(&text, "days = 1\n", &format!("days = {}\n", self.days));
        if self.command_levels {
            text = common::replace_once(
                &text,
                "placeholder = true\n",
                "command_levels = true\nplaceholder = true\n",
            );
        }
        text = common::replace_once(
            &text,
            "class = 6900\nq = 5\nr = 0\nhome = 1\n",
            &format!("class = 6900\nq = 5\nr = 0\nhome = 1\n{}", self.boat),
        );
        if self.wide_map {
            text = common::replace_once(
                &text,
                "map = \"../map/placeholder_submarines.toml\"",
                &format!("map = \"{}\"", wide_map()),
            );
        }
        if self.allied_base_at_the_boat {
            text.push_str(
                "\n[[base]]\nid = 2\nname = \"Placeholder Allied base\"\nside = \"allied\"\nq = 5\nr = 0\nairfield = false\n",
            );
        }
        for force in &self.forces {
            text.push_str(&format!(
                "\n[[task_force]]\nid = {}\nname = \"Placeholder Allied force {}\"\nside = \"allied\"\nq = {}\nr = {}\nmission = \"surface_combat\"\n",
                force.id, force.id, force.at.0, force.at.1
            ));
            for ship in &force.ships {
                let fuel = match ship.class {
                    DESTROYER => 500,
                    TRANSPORT => 1000,
                    CARRIER => 3000,
                    _ => 2000,
                };
                text.push_str(&format!(
                    "\n[[ship]]\nid = {}\nname = \"Placeholder ship {}\"\nclass = {}\nside = \"allied\"\nfuel_t = {fuel}\ntask_force = {}\n{}",
                    ship.id,
                    ship.id,
                    ship.class,
                    force.id,
                    if ship.out { "propulsion = \"out\"\n" } else { "" }
                ));
            }
            if !force.route.is_empty() {
                let waypoints: Vec<String> = force
                    .route
                    .iter()
                    .map(|(q, r)| format!("[{q}, {r}]"))
                    .collect();
                text.push_str(&format!(
                    "\n[[order]]\nday = 1\nside = \"allied\"\ntask_force = {}\nkind = \"set_route\"\nwaypoints = [{}]\n\n[[order]]\nday = 1\nside = \"allied\"\ntask_force = {}\nkind = \"set_speed\"\nknots = \"{}\"\n",
                    force.id,
                    waypoints.join(", "),
                    force.id,
                    force.knots
                ));
            }
        }
        text
    }

    fn scenario(&self) -> Scenario {
        self.scenario_on(&common::frozen())
    }

    fn scenario_on(&self, options: &LoadOptions) -> Scenario {
        common::from_text_with(&self.text(), &scenario_path(), options).expect("the game loads")
    }
}

/// The frozen figures with a placeholder battleship, class 9, added to the
/// ship classes.
fn with_a_battleship() -> LoadOptions {
    common::altered_reference("scenario_128_battleship", |dir| {
        common::edit_file(dir, "ship_classes.csv", |mut text| {
            if !text.ends_with('\n') {
                text.push('\n');
            }
            text.push_str(
                "9,Placeholder battleship,battleship,16,32,2000,8000,0,8,3,100,20,8,8,200,400,0,0,0\n",
            );
            text
        })
    })
}

// ---------------------------------------------------------------------
// What one run did.
// ---------------------------------------------------------------------

struct Run {
    world: World,
    events: Vec<Event>,
}

fn play(scenario: &Scenario, seed: u64) -> Run {
    let mut run = scenario.with_seed(seed).expect("a seed can be set");
    run.world.set_config(SimConfig {
        record_breakdowns: true,
    });
    run.run_to_end().expect("the run resolves");
    let events: Vec<Event> = run.world.events().iter().cloned().collect();
    Run {
        world: run.world,
        events,
    }
}

const BOAT: SubmarineId = SubmarineId::new(1);

/// One attack chance of boat 1.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct Chance {
    id: EventId,
    day: u32,
    pulse: Pulse,
    force: u32,
    hours: Fixed,
    chance: Fixed,
    attacks: Fixed,
}

/// One salvo of boat 1.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct Salvo {
    id: EventId,
    day: u32,
    pulse: Pulse,
    force: u32,
    ship: u32,
    torpedoes: u32,
    hits: u32,
}

impl Run {
    fn chances(&self) -> Vec<Chance> {
        self.events
            .iter()
            .filter_map(|event| match event.kind {
                EventKind::SubmarineAttackChance {
                    submarine,
                    task_force,
                    hours,
                    chance,
                    attacks,
                    ..
                } if submarine == BOAT => Some(Chance {
                    id: event.id,
                    day: event.day,
                    pulse: event.pulse.expect("in a pulse"),
                    force: task_force.raw(),
                    hours,
                    chance,
                    attacks,
                }),
                _ => None,
            })
            .collect()
    }

    fn chances_in(&self, day: u32, pulse: Pulse) -> Vec<Chance> {
        self.chances()
            .into_iter()
            .filter(|c| (c.day, c.pulse) == (day, pulse))
            .collect()
    }

    fn salvos(&self) -> Vec<Salvo> {
        self.events
            .iter()
            .filter_map(|event| match event.kind {
                EventKind::SubmarineSalvo {
                    submarine,
                    task_force,
                    ship,
                    torpedoes,
                    hits,
                    ..
                } if submarine == BOAT => Some(Salvo {
                    id: event.id,
                    day: event.day,
                    pulse: event.pulse.expect("in a pulse"),
                    force: task_force.raw(),
                    ship: ship.raw(),
                    torpedoes,
                    hits,
                }),
                _ => None,
            })
            .collect()
    }

    fn boat(&self) -> &sim_core::state::Submarine {
        &self.world.umpire().submarines[&BOAT]
    }

    fn event(&self, id: EventId) -> &Event {
        self.events
            .iter()
            .find(|event| event.id == id)
            .expect("the event")
    }

    /// The draw of the boat's attack on a force, replayed from the world's
    /// seed: the one number of its stream, as the rule draws it.
    fn attack_draw_succeeds(&self, day: u32, pulse: Pulse, force: u32, chance: Fixed) -> bool {
        self.world
            .rng_pair(
                day,
                Some(pulse),
                Subsystem::Submarine,
                Draw::SubmarineAttack,
                EntityRef::Submarine(BOAT),
                EntityRef::TaskForce(TaskForceId::new(force)),
            )
            .chance(chance)
    }
}

/// The pulses of a game of `days` days, in order.
fn pulses(days: u32) -> Vec<(u32, Pulse)> {
    (1..=days)
        .flat_map(|day| Pulse::ALL.into_iter().map(move |pulse| (day, pulse)))
        .collect()
}

// ---------------------------------------------------------------------
// Part A: the hours and the chance to attack.
// ---------------------------------------------------------------------

/// Exactly one attack chance of boat 1 and force 1 is recorded in the pulse,
/// with these hours and this chance.
fn assert_one_chance(run: &Run, day: u32, pulse: Pulse, hours: &str, chance: &str, label: &str) {
    let found = run.chances_in(day, pulse);
    assert_eq!(found.len(), 1, "{label}: one chance in the pulse");
    assert_eq!(found[0].force, 1, "{label}");
    assert_eq!(found[0].hours, fx(hours), "{label}: hours");
    assert_eq!(found[0].chance, fx(chance), "{label}: chance");
    assert_eq!(found[0].attacks, fx("0.02"), "{label}: hourly chance");
}

#[test]
fn statement_01_one_chance_is_recorded_for_each_of_the_games_a1_to_a6_and_none_for_a7() {
    // A1: stopped all the night.
    let a1 = game(vec![carrier_and_destroyer()]).scenario();
    for seed in [1, 2, 3] {
        let run = play(&a1, seed);
        assert_one_chance(&run, 1, Pulse::Night, "12", "0.215287", "A1");
    }
    // A2: stopped all the morning. The boat is held down through the night
    // (she starts held down for the first pulse), so nothing came before.
    let a2 = game(vec![carrier_and_destroyer()])
        .boat("held_down_pulses = 1")
        .scenario();
    let run = play(&a2, 1);
    assert!(run.chances_in(1, Pulse::Night).is_empty(), "A2: held down");
    assert_one_chance(&run, 1, Pulse::Morning, "6", "0.114159", "A2");

    // A3, A4, A5: a force that crosses (5,0) in the morning, coming from the
    // west so that the night leaves it at a centre or at half a leg short of
    // (5,0). The night has no hours in (5,0).
    let cross = |from: i32, knots: u32| {
        game(vec![force(1, vec![ship(1, CARRIER), ship(2, DESTROYER)])
            .steams((from, 0), &[(11, 0)], knots)])
        .wide()
        .scenario()
    };
    // 30 knots: 40/30 = 1.333333, not 2.000000 - 0.666666.
    let run = play(&cross(-5, 30), 1);
    assert!(run.chances_in(1, Pulse::Night).is_empty(), "A3: night");
    assert_one_chance(&run, 1, Pulse::Morning, "1.333333", "0.026533", "A3");
    // 20 knots: 2 hours.
    let run = play(&cross(-2, 20), 1);
    assert!(run.chances_in(1, Pulse::Night).is_empty(), "A4: night");
    assert_one_chance(&run, 1, Pulse::Morning, "2", "0.039600", "A4");
    // 15 knots: 40/15 = 2.666666.
    let run = play(&cross(0, 15), 1);
    assert!(run.chances_in(1, Pulse::Night).is_empty(), "A5: night");
    assert_one_chance(&run, 1, Pulse::Morning, "2.666666", "0.052406", "A5");

    // A6: arrives at (5,0) with 3 hours of the morning left, and stops.
    let a6 = game(vec![
        force(1, vec![ship(1, CARRIER), ship(2, DESTROYER)]).steams((-3, 0), &[(5, 0)], 20)
    ])
    .wide()
    .scenario();
    let run = play(&a6, 1);
    assert!(run.chances_in(1, Pulse::Night).is_empty(), "A6: night");
    assert_one_chance(&run, 1, Pulse::Morning, "3", "0.058808", "A6");

    // A7: passes through (4,0) and (6,0) and never enters (5,0): it goes
    // round by (5,-1) and (6,-1). No chance is recorded, in any pulse.
    let a7 = game(vec![force(1, vec![ship(1, CARRIER), ship(2, DESTROYER)])
        .steams((-5, 0), &[(4, 0), (5, -1), (6, -1), (6, 0)], 30)])
    .wide()
    .scenario();
    for seed in [1, 2, 3] {
        let run = play(&a7, seed);
        assert!(run.chances().is_empty(), "A7: no chance, seed {seed}");
        assert!(run.salvos().is_empty(), "A7: no salvo, seed {seed}");
        let touched: BTreeSet<(i32, i32)> = run
            .events
            .iter()
            .filter_map(|event| match &event.kind {
                EventKind::TaskForceSteamed { hexes_entered, .. } => Some(hexes_entered.clone()),
                _ => None,
            })
            .flatten()
            .map(|hex| (hex.q, hex.r))
            .collect();
        assert!(touched.contains(&(4, 0)) && touched.contains(&(6, 0)));
        assert!(!touched.contains(&(5, 0)), "A7: never enters (5,0)");
    }
}

#[test]
fn statement_02_the_hours_are_from_entering_to_leaving_one_hexs_steaming_not_two() {
    // A3: the force is in (5,0) from half a leg before its centre to half a
    // leg after: 40 nm, one hex's steaming, 40/30 hours. Two hexes' would be
    // 80/30 = 2.666666 and the time between two position changes a hex
    // apart is the same 1.333333.
    let a3 = game(vec![
        force(1, vec![ship(1, CARRIER), ship(2, DESTROYER)]).steams((-5, 0), &[(11, 0)], 30)
    ])
    .wide()
    .scenario();
    let run = play(&a3, 1);
    let found = run.chances_in(1, Pulse::Morning);
    assert_eq!(found.len(), 1);
    assert_eq!(found[0].hours, fx("1.333333"));
    assert_ne!(found[0].hours, fx("2.666666"));
    // The steaming that took the force through (5,0): from the centre of
    // (4,0) its position changes into (5,0) 20 nm along and out of it 60 nm
    // along. The force steamed one 40 nm hex in between.
    let entered: Vec<(Fixed, Vec<(i32, i32)>)> = run
        .events
        .iter()
        .filter_map(|event| match &event.kind {
            EventKind::TaskForceSteamed {
                distance_nm,
                hexes_entered,
                ..
            } if event.pulse == Some(Pulse::Morning) => Some((
                *distance_nm,
                hexes_entered.iter().map(|h| (h.q, h.r)).collect(),
            )),
            _ => None,
        })
        .collect();
    assert_eq!(entered.len(), 1);
    // Morning: 30 kn x 6 h = 180 nm; the hexes entered start at (5,0).
    assert_eq!(entered[0].0, Fixed::from_int(180));
    assert_eq!(entered[0].1.first(), Some(&(5, 0)));
}

#[test]
fn statement_03_a_ship_dead_in_the_water_multiplies_the_hourly_chance_by_5() {
    // A8: the force of A2 with its carrier dead in the water: p = 0.1,
    // 6 hours, miss 0.531441, chance 0.468559.
    let a8 = game(vec![force(1, vec![dead(1, CARRIER), ship(2, DESTROYER)])])
        .boat("held_down_pulses = 1")
        .scenario();
    let run = play(&a8, 1);
    let found = run.chances_in(1, Pulse::Morning);
    assert_eq!(found.len(), 1, "A8");
    assert_eq!((found[0].hours, found[0].chance), (fx("6"), fx("0.468559")));
    assert_eq!(found[0].attacks, fx("0.1"), "A8: 0.02 x 5");
    // A9: all the night: miss 0.282428, chance 0.717572.
    let a9 = game(vec![force(1, vec![dead(1, CARRIER), ship(2, DESTROYER)])]).scenario();
    let run = play(&a9, 1);
    let found = run.chances_in(1, Pulse::Night);
    assert_eq!(found.len(), 1, "A9");
    assert_eq!(
        (found[0].hours, found[0].chance),
        (fx("12"), fx("0.717572"))
    );
    assert_eq!(found[0].attacks, fx("0.1"), "A9");
    // The breakdown names the factors.
    let event = run.event(found[0].id);
    let factors: BTreeMap<&str, Fixed> = event
        .breakdown
        .as_ref()
        .expect("a breakdown")
        .factors
        .iter()
        .map(|f| (f.name, f.value))
        .collect();
    assert_eq!(factors["attacks_ship_dead_in_water"], fx("5"));
    assert_eq!(factors["miss"], fx("0.282428"));
}

#[test]
fn statement_04_a_boat_held_down_on_passage_or_damaged_makes_no_draw() {
    let none_at_night = |lines: &str, label: &str| {
        let scenario = game(vec![carrier_and_destroyer()]).boat(lines).scenario();
        for seed in [1, 2, 3, 4, 5] {
            let run = play(&scenario, seed);
            assert!(
                run.chances_in(1, Pulse::Night).is_empty(),
                "{label}: no chance in the night, seed {seed}"
            );
            assert!(run.salvos().is_empty(), "{label}: no salvo");
        }
    };
    none_at_night("held_down_pulses = 1", "A10 held down");
    none_at_night("station_q = 9\nstation_r = 0", "A11 on passage");
    none_at_night("condition = \"damaged\"", "A12 damaged");
    // The held boat is held for that pulse only: she has a chance in the
    // next.
    let held = game(vec![carrier_and_destroyer()])
        .boat("held_down_pulses = 1")
        .scenario();
    assert_eq!(play(&held, 1).chances_in(1, Pulse::Morning).len(), 1);
}

#[test]
fn statement_05_a_force_lying_where_a_base_of_its_own_side_is_not_attacked() {
    // A14: A1 with an Allied base at (5,0).
    let a14 = game(vec![carrier_and_destroyer()]).base().scenario();
    for seed in [1, 2, 3] {
        let run = play(&a14, seed);
        assert!(run.chances().is_empty(), "A14: none, seed {seed}");
    }
    // Without the base the same game has one.
    let a1 = game(vec![carrier_and_destroyer()]).scenario();
    assert_eq!(play(&a1, 1).chances_in(1, Pulse::Night).len(), 1);
}

#[test]
fn statement_06_a_force_with_none_of_the_kinds_is_not_attacked() {
    // A15: A1 with two destroyers only.
    let a15 = game(vec![force(1, vec![ship(1, DESTROYER), ship(2, DESTROYER)])]).scenario();
    for seed in [1, 2, 3] {
        let run = play(&a15, seed);
        assert!(run.chances().is_empty(), "A15: none, seed {seed}");
    }
}

// ---------------------------------------------------------------------
// Part B: which force, which ship, the salvo.
// ---------------------------------------------------------------------

/// The ships of the night's first salvo of a run, if boat 1 fired in the
/// night.
fn night_salvos(run: &Run) -> Vec<Salvo> {
    run.salvos()
        .into_iter()
        .filter(|s| (s.day, s.pulse) == (1, Pulse::Night))
        .collect()
}

#[test]
fn statement_07_of_several_forces_she_attacks_one_by_the_order_of_kinds() {
    // B1: force 1 two cruisers (ships 1, 2), force 2 a transport and a
    // destroyer (3, 4), force 3 a battleship and a cruiser (5, 6).
    let b1 = game(vec![
        force(1, vec![ship(1, CRUISER), ship(2, CRUISER)]),
        force(2, vec![ship(3, TRANSPORT), ship(4, DESTROYER)]),
        force(3, vec![ship(5, BATTLESHIP), ship(6, CRUISER)]),
    ])
    .scenario_on(&with_a_battleship());
    let (mut fired, mut several, mut not_the_lowest) = (0, 0, 0);
    for seed in 1..=SEEDS {
        let run = play(&b1, seed);
        let chances = run.chances_in(1, Pulse::Night);
        assert_eq!(chances.len(), 3, "one chance for each force, seed {seed}");
        let mut succeeded = Vec::new();
        for chance in &chances {
            assert_eq!(chance.hours, fx("12"), "seed {seed}");
            assert_eq!(chance.chance, fx("0.215287"), "seed {seed}");
            if run.attack_draw_succeeds(1, Pulse::Night, chance.force, chance.chance) {
                succeeded.push(chance.force);
            }
        }
        let salvos = night_salvos(&run);
        // She fires once, or not at all.
        assert_eq!(
            salvos.len(),
            usize::from(!succeeded.is_empty()),
            "seed {seed}"
        );
        if succeeded.is_empty() {
            continue;
        }
        // Battleship (force 3) before transport (force 2) before cruiser
        // (force 1).
        let wanted = if succeeded.contains(&3) {
            3
        } else if succeeded.contains(&2) {
            2
        } else {
            1
        };
        assert_eq!(salvos[0].force, wanted, "seed {seed}: draws {succeeded:?}");
        fired += 1;
        if succeeded.len() > 1 {
            several += 1;
            if succeeded.iter().min() != Some(&wanted) {
                not_the_lowest += 1;
            }
        }
    }
    assert!(
        fired > 0 && several > 0,
        "the seeds fire, and more than one draw succeeds"
    );
    assert!(not_the_lowest > 0, "the kind decides, not the number");
}

#[test]
fn statement_08_she_aims_at_a_ship_dead_in_the_water_before_another_of_its_kind() {
    // B2: carriers 1 and 2, carrier 2 dead in the water, and a battleship.
    let b2 = game(vec![force(
        1,
        vec![ship(1, CARRIER), dead(2, CARRIER), ship(3, BATTLESHIP)],
    )])
    .scenario_on(&with_a_battleship());
    let mut fired = 0;
    for seed in 1..=SEEDS {
        let run = play(&b2, seed);
        for salvo in run.salvos() {
            fired += 1;
            // A carrier before the battleship, and the dead one first. (A
            // later salvo may come after carrier 2 has sunk: then carrier 1.)
            let sunk_before: BTreeSet<u32> = run
                .events
                .iter()
                .take_while(|event| event.id != salvo.id)
                .filter_map(|event| match event.kind {
                    EventKind::ShipSunk { ship, .. } => Some(ship.raw()),
                    _ => None,
                })
                .collect();
            let wanted = if !sunk_before.contains(&2) {
                2
            } else if !sunk_before.contains(&1) {
                1
            } else {
                3
            };
            assert_eq!(salvo.ship, wanted, "seed {seed}");
        }
    }
    assert!(fired > 0);
    // The first salvo of every seed, at least, is at carrier 2.
    let first: Vec<u32> = (1..=SEEDS)
        .filter_map(|seed| play(&b2, seed).salvos().first().map(|s| s.ship))
        .collect();
    assert!(!first.is_empty() && first.iter().all(|ship| *ship == 2));
}

#[test]
fn statement_09_a_salvo_is_4_torpedoes_each_its_own_draw_hitting_at_one_half() {
    let a1 = game(vec![carrier_and_destroyer()]).scenario();
    let mut hit_counts = BTreeSet::new();
    for seed in 1..=SEEDS {
        let run = play(&a1, seed);
        let salvos = run.salvos();
        for salvo in &salvos {
            assert_eq!(salvo.torpedoes, 4, "seed {seed}");
            // The aimed ship is the carrier, never the destroyer.
            assert_eq!(salvo.ship, 1, "seed {seed}");
            // Each torpedo is the next number of the stream of the boat and
            // the ship, under 0.5.
            let mut stream = run.world.rng_pair(
                salvo.day,
                Some(salvo.pulse),
                Subsystem::Submarine,
                Draw::SubmarineTorpedo,
                EntityRef::Submarine(BOAT),
                EntityRef::Ship(ShipId::new(salvo.ship)),
            );
            let replayed = (0..4).filter(|_| stream.chance(fx("0.5"))).count();
            assert_eq!(salvo.hits as usize, replayed, "seed {seed}");
            hit_counts.insert(salvo.hits);
            let event = run.event(salvo.id);
            let results: Vec<Fixed> = event
                .breakdown
                .as_ref()
                .expect("a breakdown")
                .factors
                .iter()
                .filter(|f| f.name == "torpedo_hit")
                .map(|f| f.value)
                .collect();
            assert_eq!(results.len(), 4);
            assert_eq!(
                results.iter().filter(|v| **v == Fixed::ONE).count(),
                salvo.hits as usize
            );
        }
        // No ship but the carrier is ever hit.
        for event in &run.events {
            match &event.kind {
                EventKind::ShipDamaged { ship, .. } | EventKind::SystemHit { ship, .. } => {
                    assert_eq!(ship.raw(), 1, "seed {seed}: only the aimed ship is hit")
                }
                _ => {}
            }
        }
        // Her salvoes go from 2, one for each she fired.
        assert_eq!(run.boat().salvoes as usize, 2 - salvos.len(), "seed {seed}");
    }
    assert_eq!(
        hit_counts,
        BTreeSet::from([0, 1, 2, 3, 4]),
        "hits run from 0 to 4"
    );
    // A seed with one salvo has one left.
    assert!((1..=SEEDS).any(|seed| {
        let run = play(&a1, seed);
        run.salvos().len() == 1 && run.boat().salvoes == 1
    }));
}

#[test]
fn statement_10_each_hit_is_worked_as_scenario_4a_works_a_torpedo_hit() {
    // The placeholder torpedo is that of scenario 4a: damage 200. A hit does
    // 100 x damage / toughness points on flotation: 100 x 200 / 600 =
    // 33.333333 on the carrier. The hits are applied through the one
    // function a ship's torpedo hit is, caused by the salvo, outside a fight
    // round (round 0), flotation only, and a carrier at 100 sinks.
    let a1 = game(vec![carrier_and_destroyer()]).scenario();
    let per_hit = fx("33.333333");
    let mut checked = BTreeSet::new();
    for seed in 1..=SEEDS {
        let run = play(&a1, seed);
        let Some(first) = run.salvos().first().copied() else {
            continue;
        };
        let damaged: Vec<&Event> = run
            .events
            .iter()
            .filter(|event| {
                matches!(event.kind, EventKind::ShipDamaged { .. })
                    && event
                        .causes
                        .contains(&sim_core::events::Cause::Event(first.id))
            })
            .collect();
        if first.hits == 0 {
            assert!(damaged.is_empty(), "seed {seed}: a miss damages nothing");
            checked.insert(0);
            continue;
        }
        assert_eq!(damaged.len(), 1, "seed {seed}");
        let EventKind::ShipDamaged {
            round,
            ship,
            track,
            hits,
            points,
            value_after,
        } = damaged[0].kind
        else {
            unreachable!("damage");
        };
        assert_eq!((round, ship.raw(), hits), (0, 1, first.hits), "seed {seed}");
        assert_eq!(track, sim_core::events::DamageTrack::Flotation);
        assert_eq!(points, per_hit * i64::from(first.hits), "seed {seed}");
        assert_eq!(value_after, points.min(Fixed::from_int(100)), "seed {seed}");
        // Four hits take the carrier to 100 and sink her.
        let sunk = run.events.iter().any(|event| {
            matches!(event.kind, EventKind::ShipSunk { ship, .. } if ship.raw() == 1)
                && event.pulse == Some(first.pulse)
                && event.day == first.day
        });
        assert_eq!(sunk, first.hits == 4, "seed {seed}");
        checked.insert(first.hits);
    }
    assert_eq!(checked, BTreeSet::from([0, 1, 2, 3, 4]));
}

#[test]
fn statement_11_with_one_salvo_left_she_fires_then_leaves_for_home_and_attacks_nothing_on_the_way()
{
    // Three days, so that she has time to go. Base 1 is at (12,0), 280 nm
    // from her station: 28 hours at 10 knots.
    let a1 = game(vec![carrier_and_destroyer()]).days(3).scenario();
    let mut arrived = 0;
    let mut went_home_for_no_salvo = 0;
    for seed in 1..=SEEDS {
        let run = play(&a1, seed);
        let salvos = run.salvos();
        if salvos.len() < 2 {
            continue;
        }
        // The second salvo is fired with one salvo left, and then she has
        // none.
        assert_eq!(salvos.len(), 2, "seed {seed}");
        let second = salvos[1];
        assert_eq!(run.boat().salvoes, 0, "seed {seed}");
        // The hunt after the second salvo may have sunk her: she is gone and
        // goes nowhere (statement 14).
        if run.boat().condition == SubmarineCondition::Sunk {
            continue;
        }
        let left: Vec<&Event> = run
            .events
            .iter()
            .filter(|event| matches!(event.kind, EventKind::SubmarineLeftForHome { .. }))
            .collect();
        assert_eq!(left.len(), 1, "seed {seed}: she leaves once");
        let EventKind::SubmarineLeftForHome { reason, base, .. } = left[0].kind else {
            unreachable!("left");
        };
        assert_eq!(base.raw(), 1);
        assert_eq!(
            (left[0].day, left[0].pulse),
            (second.day, Some(second.pulse))
        );
        // By the hunt after the salvo she may be damaged, and then she
        // leaves for that; else she has no salvo.
        let hunted_damaged = run.events.iter().any(|event| {
            matches!(event.kind, EventKind::SubmarineHunted { damaged: true, .. })
                && event.day == second.day
                && event.pulse == Some(second.pulse)
        });
        if hunted_damaged {
            assert_eq!(reason, SubmarineLeftReason::Damaged, "seed {seed}");
        } else {
            assert_eq!(reason, SubmarineLeftReason::NoSalvo, "seed {seed}");
            assert_eq!(
                left[0].causes,
                vec![sim_core::events::Cause::Event(second.id)]
            );
            went_home_for_no_salvo += 1;
        }
        // She attacks nothing after she leaves.
        let after: Vec<&Event> = run
            .events
            .iter()
            .filter(|event| event.id > left[0].id)
            .filter(|event| {
                matches!(
                    event.kind,
                    EventKind::SubmarineAttackChance { .. } | EventKind::SubmarineSalvo { .. }
                )
            })
            .collect();
        assert!(after.is_empty(), "seed {seed}: nothing on the way");
        // At 10 knots from the next pulse she is 280 nm from home after 28
        // hours: she is taken off the map there, if the days allow.
        let mut hours = 0;
        let mut home_in = None;
        for (day, pulse) in pulses(3) {
            if (day, pulse as u8) <= (second.day, second.pulse as u8) {
                continue;
            }
            hours += match pulse {
                Pulse::Night => 12,
                _ => 6,
            };
            if hours >= 28 {
                home_in = Some((day, pulse));
                break;
            }
        }
        let reached: Vec<&Event> = run
            .events
            .iter()
            .filter(|event| matches!(event.kind, EventKind::SubmarineReachedHome { .. }))
            .collect();
        match home_in {
            Some((day, pulse)) => {
                assert_eq!(reached.len(), 1, "seed {seed}");
                assert_eq!((reached[0].day, reached[0].pulse), (day, Some(pulse)));
                assert_eq!(run.boat().condition, SubmarineCondition::Home);
                assert_eq!((run.boat().hex.q, run.boat().hex.r), (12, 0));
                arrived += 1;
            }
            None => assert!(reached.is_empty(), "seed {seed}"),
        }
    }
    assert!(
        went_home_for_no_salvo > 0 && arrived > 0,
        "{went_home_for_no_salvo} {arrived}"
    );

    // And a boat that starts with one salvo.
    let one = game(vec![carrier_and_destroyer()])
        .boat("salvoes = 1")
        .days(3)
        .scenario();
    let mut left_for_none = 0;
    for seed in 1..=SEEDS {
        let run = play(&one, seed);
        let Some(salvo) = run.salvos().first().copied() else {
            continue;
        };
        assert_eq!(run.salvos().len(), 1, "seed {seed}");
        assert_eq!(run.boat().salvoes, 0);
        let hunted = run.events.iter().any(|event| {
            matches!(
                event.kind,
                EventKind::SubmarineHunted { damaged: true, .. } | EventKind::SubmarineSunk { .. }
            ) && event.id > salvo.id
        });
        let left = run
            .events
            .iter()
            .find(|event| matches!(event.kind, EventKind::SubmarineLeftForHome { .. }));
        if !hunted {
            let left = left.expect("she leaves with no salvo left");
            assert!(matches!(
                left.kind,
                EventKind::SubmarineLeftForHome {
                    reason: SubmarineLeftReason::NoSalvo,
                    ..
                }
            ));
            left_for_none += 1;
        }
    }
    assert!(left_for_none > 0);
}

// ---------------------------------------------------------------------
// Part C: the hunt.
// ---------------------------------------------------------------------

/// The hunts of a run: (the event, day, pulse, damaged, sunk).
fn hunts(run: &Run) -> Vec<(EventId, u32, Pulse, bool, bool)> {
    run.events
        .iter()
        .filter_map(|event| match event.kind {
            EventKind::SubmarineHunted {
                submarine,
                damaged,
                sunk,
                ..
            } if submarine == BOAT => Some((
                event.id,
                event.day,
                event.pulse.expect("in a pulse"),
                damaged,
                sunk,
            )),
            _ => None,
        })
        .collect()
}

fn index_of(day: u32, pulse: Pulse) -> usize {
    pulses(9)
        .iter()
        .position(|p| *p == (day, pulse))
        .expect("a pulse of the game")
}

#[test]
fn statement_12_a_force_with_another_warship_hunts_her_and_she_is_held_down_two_pulses_after() {
    // A1 over two days: the force stays where it is.
    let a1 = game(vec![carrier_and_destroyer()]).days(2).scenario();
    let (mut fired, mut third_checked) = (0, 0);
    for seed in 1..=SEEDS {
        let run = play(&a1, seed);
        let Some(first) = run.salvos().first().copied() else {
            continue;
        };
        fired += 1;
        let at = index_of(first.day, first.pulse);
        let hunt = hunts(&run);
        assert!(!hunt.is_empty(), "seed {seed}: hit or miss she is hunted");
        let (event, ..) = hunt[0];
        // Held down for the rest of that pulse and the 2 after.
        let EventKind::SubmarineHunted {
            held_down_until, ..
        } = run.event(event).kind
        else {
            unreachable!("a hunt");
        };
        assert_eq!(
            index_of(held_down_until.0, held_down_until.1),
            at + 2,
            "seed {seed}"
        );
        for k in 1..=2 {
            if let Some((day, pulse)) = pulses(2).get(at + k) {
                assert!(
                    run.chances_in(*day, *pulse).is_empty(),
                    "seed {seed}: held down in pulse +{k}"
                );
            }
        }
        // One is recorded in the third pulse after, if she is still sound
        // and the force still has a carrier to fire at.
        let (_, _, _, damaged, sunk) = hunt[0];
        let carrier_sunk = run
            .events
            .iter()
            .any(|e| matches!(e.kind, EventKind::ShipSunk { ship, .. } if ship.raw() == 1));
        if let Some((day, pulse)) = pulses(2).get(at + 3) {
            if !damaged && !sunk && !carrier_sunk {
                assert_eq!(
                    run.chances_in(*day, *pulse).len(),
                    1,
                    "seed {seed}: a chance in the third pulse after"
                );
                third_checked += 1;
            }
        }
    }
    assert!(fired > 0 && third_checked > 0);
}

#[test]
fn statement_13_the_hunt_is_one_stream_a_first_draw_under_0_3_damages_and_a_second_under_0_2_sinks()
{
    let a1 = game(vec![carrier_and_destroyer()]).days(2).scenario();
    let (mut sound, mut damaged_afloat, mut sunk_count) = (0u32, 0u32, 0u32);
    for seed in 1..=SEEDS {
        let run = play(&a1, seed);
        for salvo in run.salvos() {
            // The stream of the boat and the force; replayed from the
            // seed.
            let mut stream = run.world.rng_pair(
                salvo.day,
                Some(salvo.pulse),
                Subsystem::Submarine,
                Draw::SubmarineHunt,
                EntityRef::Submarine(BOAT),
                EntityRef::TaskForce(TaskForceId::new(salvo.force)),
            );
            let damaged = stream.chance(fx("0.3"));
            let sunk = damaged && stream.chance(fx("0.2"));
            let hunt = hunts(&run)
                .into_iter()
                .find(|h| (h.1, h.2) == (salvo.day, salvo.pulse))
                .expect("a hunt of the salvo");
            assert_eq!((hunt.3, hunt.4), (damaged, sunk), "seed {seed}");
            match (damaged, sunk) {
                (false, _) => sound += 1,
                (true, false) => damaged_afloat += 1,
                (true, true) => sunk_count += 1,
            }
        }
    }
    assert!(
        sound > 0 && damaged_afloat > 0 && sunk_count > 0,
        "each of the three ends happens"
    );
    let total = f64::from(sound + damaged_afloat + sunk_count);
    let share = |n: u32| f64::from(n) / total;
    // 0.7 untouched, 0.24 damaged and afloat, 0.06 sunk.
    assert!((share(sound) - 0.7).abs() < 0.1, "{}", share(sound));
    assert!(
        (share(damaged_afloat) - 0.24).abs() < 0.1,
        "{}",
        share(damaged_afloat)
    );
    assert!(
        (share(sunk_count) - 0.06).abs() < 0.06,
        "{}",
        share(sunk_count)
    );
}

#[test]
fn statement_14_damaged_she_leaves_for_home_and_attacks_nothing_sunk_she_is_gone_and_told() {
    let a1 = game(vec![carrier_and_destroyer()]).days(2).scenario();
    let (mut damaged_seen, mut sunk_seen) = (0, 0);
    for seed in 1..=SEEDS {
        let run = play(&a1, seed);
        for (event, day, pulse, damaged, sunk) in hunts(&run) {
            let after = |kind: fn(&EventKind) -> bool| {
                run.events.iter().any(|e| e.id > event && kind(&e.kind))
            };
            if sunk {
                sunk_seen += 1;
                // Gone at once: sunk in the same pulse, off the map.
                let sinking: Vec<&Event> = run
                    .events
                    .iter()
                    .filter(|e| matches!(e.kind, EventKind::SubmarineSunk { .. }))
                    .collect();
                assert_eq!(sinking.len(), 1, "seed {seed}");
                assert_eq!((sinking[0].day, sinking[0].pulse), (day, Some(pulse)));
                assert_eq!(
                    sinking[0].causes,
                    vec![sim_core::events::Cause::Event(event)]
                );
                assert_eq!(run.boat().condition, SubmarineCondition::Sunk);
                assert!(run.boat().passage.is_none());
                assert!(!after(|k| matches!(
                    k,
                    EventKind::SubmarineAttackChance { .. }
                        | EventKind::SubmarineSalvo { .. }
                        | EventKind::SubmarineSteamed { .. }
                )));
                // Her side is told in that pulse; the other is not.
                let told = |side: Side| {
                    run.world
                        .side_view(side)
                        .events()
                        .iter()
                        .filter(|e| {
                            matches!(
                                e.kind,
                                SideEventKind::Whole(EventKind::SubmarineSunk { .. })
                            )
                        })
                        .map(|e| (e.day, e.pulse))
                        .collect::<Vec<_>>()
                };
                assert_eq!(
                    told(Side::Japanese),
                    vec![(day, Some(pulse))],
                    "seed {seed}"
                );
                assert!(told(Side::Allied).is_empty(), "seed {seed}");
            } else if damaged {
                damaged_seen += 1;
                let left = run
                    .events
                    .iter()
                    .find(|e| matches!(e.kind, EventKind::SubmarineLeftForHome { .. }))
                    .expect("she leaves");
                assert!(matches!(
                    left.kind,
                    EventKind::SubmarineLeftForHome {
                        reason: SubmarineLeftReason::Damaged,
                        ..
                    }
                ));
                assert_eq!(left.causes, vec![sim_core::events::Cause::Event(event)]);
                assert_ne!(run.boat().condition, SubmarineCondition::Sound);
                assert!(!after(|k| matches!(
                    k,
                    EventKind::SubmarineAttackChance { .. } | EventKind::SubmarineSalvo { .. }
                )));
            }
        }
    }
    assert!(damaged_seen > 0 && sunk_seen > 0);
}

#[test]
fn statement_15_a_force_of_one_ship_the_ship_aimed_at_hunts_nothing() {
    // C1: a lone placeholder cruiser at (5,0).
    let c1 = game(vec![force(1, vec![ship(1, CRUISER)])]).scenario();
    let (mut fired, mut again) = (0, 0);
    for seed in 1..=SEEDS {
        let run = play(&c1, seed);
        let Some(first) = run.salvos().first().copied() else {
            continue;
        };
        fired += 1;
        assert!(hunts(&run).is_empty(), "seed {seed}: nothing hunts her");
        assert_eq!(run.boat().held_down_until, None, "seed {seed}");
        // She is not held down: a chance in the next pulse, unless the
        // cruiser has been sunk (two hits of 50 do it) or it is the last
        // pulse of the game.
        let at = index_of(first.day, first.pulse);
        let cruiser_sunk = run
            .events
            .iter()
            .any(|e| matches!(e.kind, EventKind::ShipSunk { .. }));
        if let Some((day, pulse)) = pulses(1).get(at + 1) {
            if !cruiser_sunk {
                assert_eq!(
                    run.chances_in(*day, *pulse).len(),
                    1,
                    "seed {seed}: she may fire again in the next pulse"
                );
                again += 1;
            }
        }
    }
    assert!(fired > 0 && again > 0);
}

// ---------------------------------------------------------------------
// Part E: what each side is told.
// ---------------------------------------------------------------------

/// What the boat's stream of claims gives for a salvo at `ship` in
/// `(day, pulse)`, replayed from the world's seed as the rule draws it: the
/// kind taken for, read by the confusion row of `true_kind`, and the number
/// of false hits among `missed` torpedoes.
fn claim_replayed(
    run: &Run,
    day: u32,
    pulse: Pulse,
    ship: u32,
    true_kind: ShipKind,
    missed: u32,
) -> (ShipKind, u32) {
    let search = run
        .world
        .ref_data()
        .search
        .as_ref()
        .expect("search figures");
    let mut stream = run.world.rng_pair(
        day,
        Some(pulse),
        Subsystem::Submarine,
        Draw::SubmarineClaim,
        EntityRef::Submarine(BOAT),
        EntityRef::Ship(ShipId::new(ship)),
    );
    let draw = i64::try_from(stream.below(1_000_000)).expect("a draw");
    let row = &search.kind_confusion[&true_kind];
    let mut cumulative = 0i64;
    let mut taken_for = None;
    for kind in ShipKind::ALL {
        cumulative += row.get(&kind).map_or(0, |chance| chance.raw());
        if draw < cumulative {
            taken_for = Some(kind);
            break;
        }
    }
    let false_hits = (0..missed).filter(|_| stream.chance(fx("0.25"))).count();
    (
        taken_for.expect("a draw lands in a confusion row"),
        u32::try_from(false_hits).expect("few"),
    )
}

/// Where the report of a submarine to the side fired on says she is, for a
/// salvo at `ship` in `(day, pulse)`, replayed from the world's seed as the
/// rule draws it: her hex, or a neighbour where the error struck.
fn report_replayed(run: &Run, day: u32, pulse: Pulse, ship: u32, at: (i32, i32)) -> (i32, i32) {
    let search = run
        .world
        .ref_data()
        .search
        .as_ref()
        .expect("search figures");
    let mut stream = run.world.rng_pair(
        day,
        Some(pulse),
        Subsystem::Submarine,
        Draw::SubmarineReportError,
        EntityRef::Submarine(BOAT),
        EntityRef::Ship(ShipId::new(ship)),
    );
    if !stream.chance(search.position_error_chance) {
        return at;
    }
    let index = usize::try_from(stream.below(6)).expect("below 6");
    let (dq, dr) = sim_core::hex::Hex::DIRECTIONS[index];
    let neighbour = sim_core::hex::Hex::new(at.0 + dq, at.1 + dr);
    if run.world.map().is_navigable(neighbour) {
        (neighbour.q, neighbour.r)
    } else {
        at
    }
}

#[test]
fn statement_21_her_side_is_told_that_she_fired_the_kind_taken_and_her_claim_and_no_contact() {
    let a1 = game(vec![carrier_and_destroyer()]).scenario();
    let (mut fired, mut other_kind, mut false_seen, mut no_hits, mut hits_seen) = (0, 0, 0, 0, 0);
    for seed in 1..=SEEDS {
        let run = play(&a1, seed);
        let Some(salvo) = run.salvos().first().copied() else {
            continue;
        };
        fired += 1;
        // The aimed ship is the carrier, ship 1 (statement 8).
        assert_eq!(salvo.ship, 1, "seed {seed}");
        let (taken_for, false_hits) = claim_replayed(
            &run,
            salvo.day,
            salvo.pulse,
            salvo.ship,
            ShipKind::Carrier,
            salvo.torpedoes - salvo.hits,
        );
        let sunk = u32::try_from(
            run.events
                .iter()
                .filter(|e| matches!(e.kind, EventKind::ShipSunk { .. }))
                .count(),
        )
        .expect("few");

        let japanese = run.world.side_view(Side::Japanese);
        let events = japanese.events();
        // That she fired and the kind she took her target for.
        let told: Vec<&sim_core::side_events::SideEvent> = events
            .iter()
            .filter(|e| matches!(e.kind, SideEventKind::OwnSubmarineFired { .. }))
            .collect();
        assert_eq!(told.len(), 1, "seed {seed}: told once that she fired");
        let SideEventKind::OwnSubmarineFired {
            submarine,
            hex,
            torpedoes,
            taken_for: told_kind,
        } = told[0].kind.clone()
        else {
            unreachable!("filtered");
        };
        assert_eq!(submarine, BOAT);
        assert_eq!((hex.q, hex.r), (5, 0));
        assert_eq!(torpedoes, 4);
        assert_eq!(told_kind, taken_for, "seed {seed}: the kind taken for");
        // The umpire's salvo event carries the same kind.
        let EventKind::SubmarineSalvo {
            taken_for: logged, ..
        } = run.event(salvo.id).kind
        else {
            unreachable!("a salvo");
        };
        assert_eq!(logged, taken_for);
        if taken_for != ShipKind::Carrier {
            other_kind += 1;
        }

        // Her claim: seen whole, as a submarine's, the hits that landed and
        // a chance of a false hit for each that missed, the ships sunk.
        let claims: Vec<&sim_core::side_events::SideEvent> = events
            .iter()
            .filter(|e| {
                matches!(
                    e.kind,
                    SideEventKind::Whole(EventKind::ClaimReceived { .. })
                )
            })
            .collect();
        assert_eq!(claims.len(), 1, "seed {seed}: one claim");
        let SideEventKind::Whole(EventKind::ClaimReceived {
            side,
            hex,
            claimed_sunk,
            claimed_hits,
            claimed_kind,
            claimed_shot_down,
            claimed_on_ground,
            from_air,
            from_bombardment,
            from_submarine,
            ..
        }) = claims[0].kind.clone()
        else {
            unreachable!("filtered");
        };
        assert_eq!(side, Side::Japanese);
        assert_eq!((hex.q, hex.r), (5, 0), "her hex");
        assert!(from_submarine && !from_air && !from_bombardment);
        assert_eq!(claimed_kind, Some(taken_for), "seed {seed}");
        assert_eq!(claimed_hits, salvo.hits + false_hits, "seed {seed}: hits");
        assert_eq!(claimed_sunk, sunk, "seed {seed}: the ships her hits sank");
        assert_eq!((claimed_shot_down, claimed_on_ground), (0, 0));
        assert!(claimed_hits >= salvo.hits && claimed_hits <= salvo.torpedoes);
        if false_hits > 0 {
            false_seen += 1;
        }
        if salvo.hits == 0 {
            no_hits += 1;
        } else {
            hits_seen += 1;
        }
        // The claim cites the salvo, and the umpire's breakdown of it holds
        // the real hits, the chance and the false hits and no raw draw.
        let logged_claim = run
            .events
            .iter()
            .find(|e| matches!(e.kind, EventKind::ClaimReceived { .. }))
            .expect("the umpire's claim");
        assert_eq!(
            logged_claim.causes,
            vec![sim_core::events::Cause::Event(salvo.id)]
        );
        let breakdown = logged_claim.breakdown.as_ref().expect("a breakdown");
        let factor = |name: &str| {
            breakdown
                .factors
                .iter()
                .find(|f| f.name == name)
                .unwrap_or_else(|| panic!("factor {name}"))
                .value
        };
        assert_eq!(factor("real_hits"), Fixed::from_int(i64::from(salvo.hits)));
        assert_eq!(factor("false_hit_chance"), fx("0.25"));
        assert_eq!(factor("false_hits"), Fixed::from_int(i64::from(false_hits)));
        assert_eq!(breakdown.result, Fixed::from_int(i64::from(claimed_hits)));
        assert!(breakdown.factors.iter().all(|f| f.name != "draw"));
        // The words say it is a submarine's claim.
        assert!(
            logged_claim.kind.to_string().contains("submarine"),
            "{}",
            logged_claim.kind
        );

        // Held in her side's belief as that claim, and in no other's.
        let claims_held = japanese.claims();
        assert_eq!(claims_held.len(), 1);
        assert!(claims_held[0].from_submarine);
        assert_eq!(claims_held[0].claimed_hits, claimed_hits);
        assert_eq!(claims_held[0].claimed_kind, Some(taken_for));
        assert_eq!(
            (claims_held[0].day, claims_held[0].pulse),
            (salvo.day, salvo.pulse)
        );
        assert!(claims_held[0].text(Side::Japanese).contains("submarine"));
        assert!(run.world.side_view(Side::Allied).claims().is_empty());

        // No contact of the force and no report of ships.
        assert_eq!(japanese.contacts().count(), 0, "seed {seed}: no contact");
        assert!(
            !events.iter().any(|e| matches!(
                e.kind,
                SideEventKind::Whole(
                    EventKind::ReportReceived { .. } | EventKind::ContactNotThere { .. }
                )
            )),
            "seed {seed}: no report of ships"
        );
        // And no report of a submarine: her own side knows her exactly.
        assert_eq!(japanese.submarine_reports().count(), 0);
    }
    assert!(fired > 100, "she fired on {fired} seeds");
    // The kind taken for is sometimes not the carrier (0.3 of the time), a
    // claim is sometimes raised above the hits (a false hit), and both a
    // salvo that hit and one that missed are claimed.
    assert!(other_kind > 0 && false_seen > 0 && no_hits > 0 && hits_seen > 0);
}

#[test]
fn statement_22_the_side_fired_on_is_told_of_each_salvo_with_no_boat_named_and_one_report() {
    let a1 = game(vec![carrier_and_destroyer()]).scenario();
    let (mut fired, mut missed_all, mut struck, mut true_hex) = (0, 0, 0, 0);
    for seed in 1..=SEEDS {
        let run = play(&a1, seed);
        let Some(salvo) = run.salvos().first().copied() else {
            continue;
        };
        fired += 1;
        let allied = run.world.side_view(Side::Allied);
        let events = allied.events();
        // Told of the salvo, hit or miss, with no boat named.
        let salvos: Vec<&sim_core::side_events::SideEvent> = events
            .iter()
            .filter(|e| matches!(e.kind, SideEventKind::SalvoFromSubmarine { .. }))
            .collect();
        assert_eq!(salvos.len(), 1, "seed {seed}: told of the salvo");
        let SideEventKind::SalvoFromSubmarine {
            task_force,
            ship,
            hex,
        } = salvos[0].kind.clone()
        else {
            unreachable!("filtered");
        };
        assert_eq!((task_force.raw(), ship.raw()), (1, salvo.ship));
        assert_eq!((hex.q, hex.r), (5, 0));
        assert_eq!(
            (salvos[0].day, salvos[0].pulse),
            (salvo.day, Some(salvo.pulse))
        );
        assert!(
            salvos[0]
                .kind
                .entities()
                .iter()
                .all(|e| !matches!(e, EntityRef::Submarine(_))),
            "seed {seed}: no boat named"
        );
        let words = salvos[0].kind.to_string();
        assert!(
            !words.contains("submarine:") && !words.contains("Placeholder submarine"),
            "seed {seed}: {words}"
        );
        if salvo.hits == 0 {
            missed_all += 1;
        }
        // No event told to the side names the boat or a number of hers.
        for event in &events {
            assert!(
                event
                    .kind
                    .entities()
                    .iter()
                    .all(|e| !matches!(e, EntityRef::Submarine(_))),
                "seed {seed}: {}",
                event.kind
            );
        }

        // One report of a submarine at her hex through the report's error.
        let at = report_replayed(&run, salvo.day, salvo.pulse, salvo.ship, (5, 0));
        if at == (5, 0) {
            true_hex += 1;
        } else {
            struck += 1;
        }
        let reports: Vec<_> = allied.submarine_reports().collect();
        assert_eq!(reports.len(), 1, "seed {seed}: one report");
        assert_eq!((reports[0].hex.q, reports[0].hex.r), at, "seed {seed}");
        assert_eq!(
            (reports[0].day, reports[0].pulse),
            (salvo.day, Some(salvo.pulse))
        );
        // Made in the salvo's pulse and aged one a pulse after it.
        let after = pulses(1).len() - 1 - index_of(salvo.day, salvo.pulse);
        assert_eq!(
            reports[0].age_pulses,
            u32::try_from(after).expect("few"),
            "seed {seed}"
        );
        let told: Vec<&sim_core::side_events::SideEvent> = events
            .iter()
            .filter(|e| {
                matches!(
                    e.kind,
                    SideEventKind::Whole(EventKind::SubmarineReported { .. })
                )
            })
            .collect();
        assert_eq!(told.len(), 1, "seed {seed}: told of the report");
        assert!(matches!(
            told[0].kind,
            SideEventKind::Whole(EventKind::SubmarineReported {
                side: Side::Allied,
                replaced: false,
                ..
            })
        ));
        // The umpire's event cites the salvo.
        let logged = run
            .events
            .iter()
            .find(|e| matches!(e.kind, EventKind::SubmarineReported { .. }))
            .expect("logged");
        assert_eq!(
            logged.causes,
            vec![sim_core::events::Cause::Event(salvo.id)]
        );
        assert_eq!(reports[0].received_event, logged.id);
        // Its contacts of ships are unchanged.
        assert_eq!(allied.contacts().count(), 0, "seed {seed}");
        // And the boat's side is not told of the report.
        assert!(!run
            .world
            .side_view(Side::Japanese)
            .events()
            .iter()
            .any(|e| matches!(
                e.kind,
                SideEventKind::Whole(EventKind::SubmarineReported { .. })
            )));
    }
    assert!(fired > 100);
    // The error strikes about 0.3 of the time, and a salvo that missed
    // every torpedo is told and reported as one that hit.
    assert!(struck > 0 && true_hex > 0 && missed_all > 0);
}

#[test]
fn statement_23_two_reports_of_a_submarine_in_one_hex_are_one_the_newer() {
    // Two salvoes in two days: a second salvo is a second report. Where both
    // name one hex the side holds one, the newer; where they name two it
    // holds two.
    let a1 = game(vec![carrier_and_destroyer()]).days(2).scenario();
    let (mut same, mut apart) = (0, 0);
    for seed in 1..=SEEDS {
        let run = play(&a1, seed);
        let salvos = run.salvos();
        if salvos.len() < 2 {
            continue;
        }
        let reports: Vec<&Event> = run
            .events
            .iter()
            .filter(|e| matches!(e.kind, EventKind::SubmarineReported { .. }))
            .collect();
        assert_eq!(reports.len(), 2, "seed {seed}");
        let (hexes, replaced): (Vec<_>, Vec<_>) = reports
            .iter()
            .map(|e| match e.kind {
                EventKind::SubmarineReported { hex, replaced, .. } => ((hex.q, hex.r), replaced),
                _ => unreachable!("filtered"),
            })
            .unzip();
        for (i, salvo) in salvos.iter().enumerate() {
            assert_eq!(
                hexes[i],
                report_replayed(&run, salvo.day, salvo.pulse, salvo.ship, (5, 0)),
                "seed {seed}: report {i}"
            );
        }
        let held: Vec<_> = run
            .world
            .side_view(Side::Allied)
            .submarine_reports()
            .map(|r| ((r.hex.q, r.hex.r), r.received_event, r.age_pulses))
            .collect();
        let end = pulses(2).len() - 1;
        let age_of =
            |salvo: &Salvo| u32::try_from(end - index_of(salvo.day, salvo.pulse)).expect("few");
        if hexes[0] == hexes[1] {
            same += 1;
            // One report of the hex, the newer; the second said so.
            assert_eq!(replaced, vec![false, true], "seed {seed}");
            assert_eq!(held, vec![(hexes[0], reports[1].id, age_of(&salvos[1]))]);
        } else {
            apart += 1;
            assert_eq!(replaced, vec![false, false], "seed {seed}");
            let mut expected = vec![
                (hexes[0], reports[0].id, age_of(&salvos[0])),
                (hexes[1], reports[1].id, age_of(&salvos[1])),
            ];
            expected.sort_by_key(|(hex, _, _)| *hex);
            let mut held_sorted = held.clone();
            held_sorted.sort_by_key(|(hex, _, _)| *hex);
            assert_eq!(held_sorted, expected, "seed {seed}");
        }
    }
    assert!(same > 0 && apart > 0, "same {same}, apart {apart}");
}

#[test]
fn statement_24_a_side_sees_its_own_boats_exactly_and_the_other_only_its_reports() {
    // Three days, so that she fires, goes home and arrives on some seeds.
    let a1 = game(vec![carrier_and_destroyer()]).days(3).scenario();
    let (mut fired, mut home, mut sunk) = (0, 0, 0);
    for seed in 1..=300 {
        let run = play(&a1, seed);
        let truth = run.boat();
        // The boat's side: exact.
        let japanese = run.world.side_view(Side::Japanese);
        let own: Vec<&sim_core::state::Submarine> = japanese.own_submarines().collect();
        assert_eq!(own.len(), 1);
        assert_eq!(own[0], truth, "seed {seed}: exactly as she is");
        assert_eq!(japanese.submarine(BOAT), Some(truth));
        assert_eq!(
            (own[0].salvoes, own[0].held_down_until, own[0].condition),
            (truth.salvoes, truth.held_down_until, truth.condition)
        );
        match truth.condition {
            SubmarineCondition::Home => home += 1,
            SubmarineCondition::Sunk => sunk += 1,
            _ => {}
        }
        // The other side: no boat, only its reports.
        let allied = run.world.side_view(Side::Allied);
        assert_eq!(allied.own_submarines().count(), 0, "seed {seed}");
        assert_eq!(allied.submarine(BOAT), None);
        let salvos = run.salvos().len();
        if salvos > 0 {
            fired += 1;
            assert!(allied.submarine_reports().count() >= 1 || run_dropped(&run));
        }
        // No event of her passage, station, salvoes, state or held-down
        // count is seen by the other side; nothing it is given names her.
        for event in allied.events() {
            let boat_event = matches!(
                &event.kind,
                SideEventKind::OwnSubmarineFired { .. }
                    | SideEventKind::OwnSubmarineHunted { .. }
                    | SideEventKind::Whole(
                        EventKind::SubmarineEntered { .. }
                            | EventKind::SubmarineSteamed { .. }
                            | EventKind::SubmarineOnStation { .. }
                            | EventKind::SubmarineLeftForHome { .. }
                            | EventKind::SubmarineReachedHome { .. }
                            | EventKind::SubmarineAttackChance { .. }
                            | EventKind::SubmarineSalvo { .. }
                            | EventKind::SubmarineHunted { .. }
                            | EventKind::SubmarineSunk { .. }
                    )
            );
            assert!(!boat_event, "seed {seed}: {}", event.kind);
            assert!(
                event
                    .kind
                    .entities()
                    .iter()
                    .all(|e| !matches!(e, EntityRef::Submarine(_))),
                "seed {seed}: {}",
                event.kind
            );
        }
        // The same of the other side's picture of the world it would be
        // built into: a command's view holds no boat either.
        assert!(allied.commands().is_empty() || allied.command(CommandRef::Area).is_some());
        // The boat's side is told of her own passage, station and state.
        let told = japanese.events();
        assert!(
            told.iter()
                .any(|e| matches!(e.kind, SideEventKind::OwnSubmarineFired { .. }))
                == (salvos > 0)
        );
    }
    assert!(fired > 50 && home > 0 && sunk > 0, "{fired} {home} {sunk}");
}

/// Whether the other side's report of the boat was dropped for its age in
/// the run.
fn run_dropped(run: &Run) -> bool {
    run.events
        .iter()
        .any(|e| matches!(e.kind, EventKind::SubmarineReportDropped { .. }))
}

#[test]
fn statement_25_a_report_ages_and_is_dropped_by_its_own_figures_and_is_in_every_commands_picture() {
    // A boat with one salvo makes one report, so nothing replaces it. The
    // contact figure drops a contact at 5 pulses; the report's own figure
    // is 6.
    let held = |days: u32, command_levels: bool| {
        let mut game = game(vec![carrier_and_destroyer()])
            .boat("salvoes = 1")
            .days(days);
        if command_levels {
            game = game.command_levels();
        }
        game.scenario()
    };
    let (mut checked_held, mut checked_dropped) = (0, 0);
    for seed in 1..=SEEDS {
        // Two days: the end is 5 pulses after a night salvo of day 1.
        let two = play(&held(2, false), seed);
        let Some(salvo) = two.salvos().first().copied() else {
            continue;
        };
        if (salvo.day, salvo.pulse) != (1, Pulse::Night) {
            continue;
        }
        let allied = two.world.side_view(Side::Allied);
        let kept: Vec<_> = allied.submarine_reports().collect();
        assert_eq!(kept.len(), 1, "seed {seed}: still held at 5 pulses");
        assert_eq!(kept[0].age_pulses, 5, "seed {seed}");
        assert!(!run_dropped(&two));
        checked_held += 1;

        // Three days: it reaches 6 pulses in the night of day 3 and is
        // dropped there, by an event citing the report that made it.
        let three = play(&held(3, false), seed);
        assert_eq!(three.salvos().first().copied(), Some(salvo), "seed {seed}");
        let report = three
            .events
            .iter()
            .find(|e| matches!(e.kind, EventKind::SubmarineReported { .. }))
            .expect("the report");
        let dropped: Vec<&Event> = three
            .events
            .iter()
            .filter(|e| matches!(e.kind, EventKind::SubmarineReportDropped { .. }))
            .collect();
        assert_eq!(dropped.len(), 1, "seed {seed}");
        assert_eq!((dropped[0].day, dropped[0].pulse), (3, Some(Pulse::Night)));
        assert_eq!(
            dropped[0].causes,
            vec![sim_core::events::Cause::Event(report.id)]
        );
        assert_eq!(
            three
                .world
                .side_view(Side::Allied)
                .submarine_reports()
                .count(),
            0
        );
        // Whole for the side that held it, absent for the other.
        let told = |side: Side| {
            three
                .world
                .side_view(side)
                .events()
                .iter()
                .filter(|e| {
                    matches!(
                        e.kind,
                        SideEventKind::Whole(EventKind::SubmarineReportDropped { .. })
                    )
                })
                .count()
        };
        assert_eq!((told(Side::Allied), told(Side::Japanese)), (1, 0));

        // With command levels on, the report is in every command's picture
        // of the side, aged and dropped as the side's own is.
        let on_two = play(&held(2, true), seed);
        let on_three = play(&held(3, true), seed);
        for (run, age) in [(&on_two, Some(5)), (&on_three, None)] {
            let view = run.world.side_view(Side::Allied);
            let commands = view.commands();
            assert!(commands.contains(&CommandRef::Area) && commands.len() >= 2);
            for command in commands {
                let pictured: Vec<_> = view
                    .command(command)
                    .expect("a command's view")
                    .submarine_reports()
                    .map(|r| (r.hex, r.age_pulses))
                    .collect();
                let own: Vec<_> = view
                    .submarine_reports()
                    .map(|r| (r.hex, r.age_pulses))
                    .collect();
                assert_eq!(pictured, own, "seed {seed}: {command:?}");
                assert_eq!(pictured.len(), usize::from(age.is_some()));
                if let Some(age) = age {
                    assert_eq!(pictured[0].1, age);
                }
            }
        }
        checked_dropped += 1;
        if checked_dropped >= 40 {
            break;
        }
    }
    assert!(checked_held > 20 && checked_dropped > 20);
}

// ---------------------------------------------------------------------
// Part D: aircraft.
// ---------------------------------------------------------------------

const GROUP: sim_core::ids::AirGroupId = sim_core::ids::AirGroupId::new(1);

/// Game D1: the head of the scenario with boat 1 put where `boat_at` says
/// and given `boat_lines` (a station, a held-down count), and Allied air
/// group 1 at a base at (0,0): 6 aircraft of the frozen reference's type 2,
/// the placeholder carrier scout (a sweep width of 20 nm, a range of 300 nm;
/// no new type is added), on the search mission with an arc from 60 to 120
/// degrees and a range of 300 nm (the type's own limit). No ship of either side.
fn game_d1(boat_at: (i32, i32), boat_lines: &str) -> Scenario {
    let mut text = game(Vec::new()).boat(boat_lines).text();
    text = common::replace_once(
        &text,
        "class = 6900\nq = 5\nr = 0\nhome = 1\n",
        &format!(
            "class = 6900\nq = {}\nr = {}\nhome = 1\n",
            boat_at.0, boat_at.1
        ),
    );
    text.push_str(
        "\n[[base]]\nid = 2\nname = \"Placeholder Allied air base\"\nside = \"allied\"\nq = 0\nr = 0\n\
         \n[[air_group]]\nid = 1\nname = \"Placeholder search group\"\nside = \"allied\"\nbase = 2\naircraft_type = 2\nready = 6\n\
         \n[[order]]\nday = 1\nside = \"allied\"\nair_group = 1\nkind = \"set_air_mission\"\nmission = \"search\"\n\
         \n[[order]]\nday = 1\nside = \"allied\"\nair_group = 1\nkind = \"set_search_arc\"\nfrom_deg = 60\nto_deg = 120\nrange_nm = 300\n",
    );
    common::from_text_with(&text, &scenario_path(), &common::frozen()).expect("game D1 loads")
}

/// One search attempt for boat 1.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct Attempt {
    id: EventId,
    day: u32,
    pulse: Pulse,
    distance_nm: Fixed,
    chance: Fixed,
    sighted: bool,
}

impl Run {
    fn attempts(&self) -> Vec<Attempt> {
        self.events
            .iter()
            .filter_map(|event| match event.kind {
                EventKind::SubmarineSearchAttempt {
                    air_group,
                    submarine,
                    distance_nm,
                    chance,
                    sighted,
                } if submarine == BOAT && air_group == GROUP => Some(Attempt {
                    id: event.id,
                    day: event.day,
                    pulse: event.pulse.expect("in a pulse"),
                    distance_nm,
                    chance,
                    sighted,
                }),
                _ => None,
            })
            .collect()
    }

    fn attempts_in(&self, pulse: Pulse) -> Vec<Attempt> {
        self.attempts()
            .into_iter()
            .filter(|a| a.pulse == pulse)
            .collect()
    }

    /// The attack from the air in `(day, pulse)`: damaged and sunk.
    fn air_attacks(&self) -> Vec<(EventId, u32, Pulse, bool, bool)> {
        self.events
            .iter()
            .filter_map(|event| match event.kind {
                EventKind::SubmarineAttackedFromAir {
                    submarine,
                    damaged,
                    sunk,
                    ..
                } if submarine == BOAT => Some((
                    event.id,
                    event.day,
                    event.pulse.expect("in a pulse"),
                    damaged,
                    sunk,
                )),
                _ => None,
            })
            .collect()
    }
}

#[test]
fn statement_16_on_her_station_the_chance_is_the_surfaced_share_of_the_forces_in_each_daylight_pulse(
) {
    let d1 = game_d1((5, 0), "");
    let mut afternoons = 0;
    let mut sighted_seen = (false, false);
    for seed in 1..=SEEDS {
        let run = play(&d1, seed);
        // The night: none.
        assert!(run.attempts_in(Pulse::Night).is_empty(), "seed {seed}");
        // The morning: one, of 0.572957 x 0.25.
        let morning = run.attempts_in(Pulse::Morning);
        assert_eq!(morning.len(), 1, "seed {seed}");
        assert_eq!(morning[0].chance, fx("0.143239"), "seed {seed}");
        assert_eq!(morning[0].distance_nm, Fixed::from_int(200), "seed {seed}");
        sighted_seen.0 |= morning[0].sighted;
        sighted_seen.1 |= !morning[0].sighted;
        // The afternoon: one, while she is still on her station.
        let afternoon = run.attempts_in(Pulse::Afternoon);
        let left = run.events.iter().any(|e| {
            matches!(
                e.kind,
                EventKind::SubmarineLeftForHome { .. } | EventKind::SubmarineSunk { .. }
            )
        });
        if !left {
            assert_eq!(afternoon.len(), 1, "seed {seed}");
            assert_eq!(afternoon[0].chance, fx("0.143239"), "seed {seed}");
            afternoons += 1;
        }
        // The breakdown: the forces' chance and her share.
        let attempt = run
            .events
            .iter()
            .find(|e| e.id == morning[0].id)
            .expect("the attempt");
        let breakdown = attempt.breakdown.as_ref().expect("breakdowns are on");
        let factor = |name: &str| {
            breakdown
                .factors
                .iter()
                .find(|f| f.name == name)
                .unwrap_or_else(|| panic!("factor {name}"))
                .value
        };
        assert_eq!(factor("chance_of_a_force"), fx("0.572957"));
        assert_eq!(factor("surfaced_share"), fx("0.25"));
        assert_eq!(breakdown.result, fx("0.143239"));
    }
    assert!(sighted_seen.0 && sighted_seen.1);
    assert!(afternoons > 0);
}

#[test]
fn statement_17_on_passage_through_the_hex_the_chance_is_whole() {
    // She starts at (2,0) bound for (9,0): after the night's steaming she
    // is at (5,0) as the morning's search looks.
    let d1 = game_d1((2, 0), "station_q = 9\nstation_r = 0");
    let mut checked = 0;
    for seed in 1..=SEEDS {
        let run = play(&d1, seed);
        assert!(run.attempts_in(Pulse::Night).is_empty(), "seed {seed}");
        let morning = run.attempts_in(Pulse::Morning);
        assert_eq!(morning.len(), 1, "seed {seed}");
        assert_eq!(morning[0].distance_nm, Fixed::from_int(200), "seed {seed}");
        assert_eq!(morning[0].chance, fx("0.572957"), "seed {seed}");
        checked += 1;
    }
    assert_eq!(checked, SEEDS);
}

#[test]
fn statement_18_in_the_groups_own_hex_the_distance_is_one_hex_and_the_chance_is_not_one() {
    let d1 = game_d1((0, 0), "");
    for seed in 1..=SEEDS {
        let run = play(&d1, seed);
        let morning = run.attempts_in(Pulse::Morning);
        assert_eq!(morning.len(), 1, "seed {seed}");
        assert_eq!(morning[0].distance_nm, Fixed::from_int(40), "seed {seed}");
        assert_eq!(morning[0].chance, fx("0.25"), "seed {seed}");
        assert_ne!(morning[0].chance, Fixed::ONE);
        let attempt = run
            .events
            .iter()
            .find(|e| e.id == morning[0].id)
            .expect("the attempt");
        let breakdown = attempt.breakdown.as_ref().expect("breakdowns are on");
        let of_a_force = breakdown
            .factors
            .iter()
            .find(|f| f.name == "chance_of_a_force")
            .expect("the forces' chance")
            .value;
        // 6875.493480 / 2400 = 2.86, cut to 1.
        assert_eq!(of_a_force, Fixed::ONE, "seed {seed}");
    }
}

#[test]
fn statement_19_sighted_she_is_reported_in_its_own_list_and_nothing_else_is_told() {
    let d1 = game_d1((5, 0), "");
    let search = d1.clone().with_seed(1).expect("a seed");
    let search = search
        .world
        .ref_data()
        .search
        .clone()
        .expect("search figures");
    let (mut sighted_runs, mut struck, mut at_her_hex) = (0, 0, 0);
    for seed in 1..=SEEDS {
        let run = play(&d1, seed);
        let sightings: Vec<Attempt> = run.attempts().into_iter().filter(|a| a.sighted).collect();
        let reports: Vec<&Event> = run
            .events
            .iter()
            .filter(|e| matches!(e.kind, EventKind::SubmarineReported { .. }))
            .collect();
        assert_eq!(reports.len(), sightings.len(), "seed {seed}");
        let allied = run.world.side_view(Side::Allied);
        // No contact of a ship and no force alerted, ever.
        assert_eq!(allied.contacts().count(), 0, "seed {seed}");
        assert!(
            !run.events.iter().any(|e| matches!(
                e.kind,
                EventKind::ReportReceived { .. } | EventKind::ForceBecameAlert { .. }
            )),
            "seed {seed}"
        );
        // The Japanese side is told no report.
        assert_eq!(
            run.world
                .side_view(Side::Japanese)
                .submarine_reports()
                .count(),
            0,
            "seed {seed}"
        );
        if sightings.is_empty() {
            assert_eq!(allied.submarine_reports().count(), 0, "seed {seed}");
            continue;
        }
        sighted_runs += 1;
        // Each report cites its attempt, at her hex or a neighbour where the
        // error struck, as the rule draws it.
        for (sighting, report) in sightings.iter().zip(&reports) {
            let EventKind::SubmarineReported { side, hex, .. } = report.kind else {
                unreachable!("filtered");
            };
            assert_eq!(side, Side::Allied);
            assert_eq!(
                report.causes,
                vec![sim_core::events::Cause::Event(sighting.id)],
                "seed {seed}"
            );
            // Where she truly was: the hex of the attack that cites the same
            // attempt (a boat damaged in the morning is on her way home by
            // the afternoon).
            let truly = run
                .events
                .iter()
                .find_map(|e| match e.kind {
                    EventKind::SubmarineAttackedFromAir { hex, .. }
                        if e.causes == vec![sim_core::events::Cause::Event(sighting.id)] =>
                    {
                        Some((hex.q, hex.r))
                    }
                    _ => None,
                })
                .expect("the attack");
            let mut stream = run.world.rng_pair(
                sighting.day,
                Some(sighting.pulse),
                Subsystem::Submarine,
                Draw::SubmarineReportError,
                EntityRef::Submarine(BOAT),
                EntityRef::AirGroup(GROUP),
            );
            let expected = if stream.chance(search.position_error_chance) {
                let index = usize::try_from(stream.below(6)).expect("below 6");
                let (dq, dr) = sim_core::hex::Hex::DIRECTIONS[index];
                let neighbour = sim_core::hex::Hex::new(truly.0 + dq, truly.1 + dr);
                if run.world.map().is_navigable(neighbour) {
                    (neighbour.q, neighbour.r)
                } else {
                    truly
                }
            } else {
                truly
            };
            assert_eq!((hex.q, hex.r), expected, "seed {seed}");
            if expected == truly {
                at_her_hex += 1;
            } else {
                struck += 1;
            }
        }
        // One report in the side's own list per hex named.
        let held: BTreeSet<(i32, i32)> = allied
            .submarine_reports()
            .map(|r| (r.hex.q, r.hex.r))
            .collect();
        assert!(!held.is_empty(), "seed {seed}");
    }
    assert!(sighted_runs > 0 && struck > 0 && at_her_hex > 0);
}

#[test]
fn statement_20_sighted_she_is_held_down_and_attacked_in_the_same_pulse_and_nothing_is_spent() {
    let d1 = game_d1((5, 0), "");
    let figures = d1.clone().with_seed(1).expect("a seed");
    let sub = figures
        .world
        .ref_data()
        .submarines
        .as_ref()
        .expect("submarine figures")
        .figures
        .clone();
    assert_eq!(sub.air_attack_damages_chance, fx("0.25"));
    assert_eq!(sub.air_attack_sinks_share, fx("0.2"));
    let (mut sunk, mut damaged, mut untouched) = (0u32, 0u32, 0u32);
    let sorties_of = |run: &Run| {
        run.events
            .iter()
            .filter(|e| matches!(e.kind, EventKind::SearchFlown { .. }))
            .count()
    };
    for seed in 1..=SEEDS {
        let run = play(&d1, seed);
        let sightings: Vec<Attempt> = run.attempts().into_iter().filter(|a| a.sighted).collect();
        let attacks = run.air_attacks();
        // One attack for each sighting, in the same pulse, citing it.
        assert_eq!(attacks.len(), sightings.len(), "seed {seed}");
        for (sighting, attack) in sightings.iter().zip(&attacks) {
            assert_eq!((attack.1, attack.2), (sighting.day, sighting.pulse));
            assert_eq!(
                run.event(attack.0).causes,
                vec![sim_core::events::Cause::Event(sighting.id)],
                "seed {seed}"
            );
            // One stream of the boat and the group: a first draw under 0.25
            // damages, a second under 0.2 sinks.
            let mut stream = run.world.rng_pair(
                sighting.day,
                Some(sighting.pulse),
                Subsystem::Submarine,
                Draw::SubmarineAirAttack,
                EntityRef::Submarine(BOAT),
                EntityRef::AirGroup(GROUP),
            );
            let expected_damaged = stream.chance(fx("0.25"));
            let expected_sunk = expected_damaged && stream.chance(fx("0.2"));
            assert_eq!((attack.3, attack.4), (expected_damaged, expected_sunk));
            if attack.4 {
                sunk += 1;
            } else if attack.3 {
                damaged += 1;
            } else {
                untouched += 1;
            }
        }
        // Held down to the end of the next pulse after the last sighting.
        let boat = run.boat();
        match sightings.last() {
            Some(last) => {
                let next = index_of(last.day, last.pulse) + 1;
                let expected = pulses(2)[next];
                assert_eq!(boat.held_down_until, Some(expected), "seed {seed}");
            }
            None => assert_eq!(boat.held_down_until, None, "seed {seed}"),
        }
        // The end she came to: sunk if any attack sank her, else damaged (or
        // home again) if any damaged her, else sound.
        if attacks.iter().any(|a| a.4) {
            assert_eq!(boat.condition, SubmarineCondition::Sunk, "seed {seed}");
        } else if attacks.iter().any(|a| a.3) {
            assert!(
                matches!(
                    boat.condition,
                    SubmarineCondition::Damaged | SubmarineCondition::Home
                ),
                "seed {seed}: {:?}",
                boat.condition
            );
        } else {
            assert_eq!(boat.condition, SubmarineCondition::Sound, "seed {seed}");
        }
        // No aircraft lost, no bomb, no sortie added: the group is as it
        // was, one flight in each daylight pulse she was searched.
        assert_eq!(
            run.world.umpire().air_groups[&GROUP].ready,
            6,
            "seed {seed}"
        );
        assert!(sorties_of(&run) <= 2, "seed {seed}");
        assert!(
            !run.events.iter().any(|e| matches!(
                e.kind,
                EventKind::StrikeLaunched { .. } | EventKind::AircraftLost { .. }
            )),
            "seed {seed}"
        );
    }
    // Each end occurs, near its share of 0.05, 0.2 and 0.75.
    assert!(
        sunk > 0 && damaged > 0 && untouched > 0,
        "{sunk} {damaged} {untouched}"
    );
    let total = f64::from(sunk + damaged + untouched);
    let share = |n: u32| f64::from(n) / total;
    assert!((share(sunk) - 0.05).abs() < 0.08, "sunk {}", share(sunk));
    assert!(
        (share(damaged) - 0.2).abs() < 0.1,
        "damaged {}",
        share(damaged)
    );
    assert!(
        (share(untouched) - 0.75).abs() < 0.12,
        "untouched {}",
        share(untouched)
    );
}

#[test]
fn statement_20_a_boat_held_down_longer_is_held_the_later_of_the_two() {
    // Held for 5 pulses from the start: a sighting's next pulse is earlier,
    // so she is held to the same pulse as before.
    let held = game_d1((5, 0), "held_down_pulses = 5");
    let none = game_d1((5, 0), "held_down_pulses = 5");
    let start = none.with_seed(1).expect("a seed").world.umpire().submarines[&BOAT].held_down_until;
    assert!(start.is_some());
    for seed in 1..=200 {
        let run = play(&held, seed);
        assert_eq!(run.boat().held_down_until, start, "seed {seed}");
    }
}

// ---------------------------------------------------------------------
// Phase 5: the station order, suspected waters, nothing else moves.
// ---------------------------------------------------------------------

/// The wording of Allied waters at (5,0), 2 hexes round, told on `day`, as
/// the scenario gives them (a scenario's `[[suspected_waters]]`).
fn allied_waters_at_5_0(day: u32) -> String {
    format!(
        "\n[[suspected_waters]]\nside = \"allied\"\nq = 5\nr = 0\ndistance = 2\nday = {day}\n\
         words = \"Placeholder belief: enemy submarines are thought to work off (5,0).\"\n\
         rows = \"Placeholder rows: none, the belief is made up for the proof.\"\n\
         reading = \"Placeholder reading by the architect: (5,0) and 2 hexes.\"\n"
    )
}

const WATERS_WORDS: &str = "Placeholder belief: enemy submarines are thought to work off (5,0).";

/// Game F1: the Allied side is given suspected waters at (5,0), 2 hexes
/// round, told on day 2, in a game of two days. Boat 1 lies at (7,0), where
/// Allied air group 1 (the search group of game D1, at a base at (0,0))
/// searches an arc that holds her: she is sighted and reported on some
/// seeds, at (7,0) or a neighbour and never at (5,0).
fn game_f1() -> Scenario {
    let mut text = game(Vec::new()).days(2).text();
    text = common::replace_once(
        &text,
        "class = 6900\nq = 5\nr = 0\nhome = 1\n",
        "class = 6900\nq = 7\nr = 0\nhome = 1\n",
    );
    text.push_str(
        "\n[[base]]\nid = 2\nname = \"Placeholder Allied air base\"\nside = \"allied\"\nq = 0\nr = 0\n\
         \n[[air_group]]\nid = 1\nname = \"Placeholder search group\"\nside = \"allied\"\nbase = 2\naircraft_type = 2\nready = 6\n\
         \n[[order]]\nday = 1\nside = \"allied\"\nair_group = 1\nkind = \"set_air_mission\"\nmission = \"search\"\n\
         \n[[order]]\nday = 1\nside = \"allied\"\nair_group = 1\nkind = \"set_search_arc\"\nfrom_deg = 60\nto_deg = 120\nrange_nm = 300\n",
    );
    text.push_str(&allied_waters_at_5_0(2));
    common::from_text_with(&text, &scenario_path(), &common::frozen()).expect("game F1 loads")
}

fn waters_of(world: &World, side: Side) -> Vec<(i32, i32, u32, String, u32)> {
    world
        .side_view(side)
        .suspected_waters()
        .iter()
        .map(|w| (w.hex.q, w.hex.r, w.distance_hexes, w.words.clone(), w.day))
        .collect()
}

#[test]
fn statement_26_suspected_waters_are_told_on_their_day_to_their_side_and_never_move() {
    let f1 = game_f1();
    let told = vec![(5, 0, 2, WATERS_WORDS.to_string(), 2)];
    let (mut reported_elsewhere, mut sunk_or_home) = (0, 0);
    for seed in 1..=300 {
        let mut run = f1.with_seed(seed).expect("a seed");
        let allied_briefing =
            |world: &World| sim_agent::briefing(&world.side_view(Side::Allied), "scenario 128 F1");
        let japanese_briefing = |world: &World| {
            sim_agent::briefing(&world.side_view(Side::Japanese), "scenario 128 F1")
        };
        // Before day 1: no waters in the Allied view or briefing, and
        // nothing of submarines in the Allied briefing at all.
        assert!(
            waters_of(&run.world, Side::Allied).is_empty(),
            "seed {seed}"
        );
        assert!(allied_briefing(&run.world).submarines.is_none());
        // Day 1, at every moment of it: still none, in the view and in the
        // briefing the view gives.
        let mut moments = 0;
        run.next_day_watched(sim_core::Staffs::none(), &mut |world, moment| {
            moments += 1;
            assert_eq!(moment.day(), 1);
            assert!(waters_of(world, Side::Allied).is_empty(), "seed {seed}");
            assert!(waters_of(world, Side::Japanese).is_empty(), "seed {seed}");
        })
        .expect("day 1")
        .expect("a day");
        assert!(moments > 3);
        // From day 2: held, with the words of the belief, in the view and in
        // the briefing, and its event is the side's own.
        assert_eq!(waters_of(&run.world, Side::Allied), told, "seed {seed}");
        let briefing = allied_briefing(&run.world);
        let part = briefing.submarines.as_ref().expect("the submarines part");
        assert_eq!(part.suspected_waters.len(), 1);
        let line = &part.suspected_waters[0];
        assert_eq!((line.place.q, line.place.r), (5, 0));
        assert_eq!((line.distance_hexes, line.told_day), (2, 2));
        assert_eq!(line.words, WATERS_WORDS);
        assert!(briefing.to_text().contains(WATERS_WORDS), "seed {seed}");
        assert!(part.boats.is_empty());
        // The Japanese side, given none, holds none; its briefing has its
        // boat and no waters.
        assert!(waters_of(&run.world, Side::Japanese).is_empty());
        let japanese = japanese_briefing(&run.world);
        let part = japanese.submarines.as_ref().expect("her boat");
        assert!(part.suspected_waters.is_empty());
        assert!(!japanese.to_text().contains(WATERS_WORDS));
        let told_events = |side: Side| {
            run.world
                .side_view(side)
                .events()
                .iter()
                .filter(|e| {
                    matches!(
                        e.kind,
                        SideEventKind::Whole(EventKind::SuspectedWatersTold { .. })
                    )
                })
                .count()
        };
        assert_eq!(
            (told_events(Side::Allied), told_events(Side::Japanese)),
            (1, 0)
        );
        // Day 2 is played: the waters are as they were told, whatever is
        // reported elsewhere.
        run.next_day().expect("day 2").expect("a day");
        assert_eq!(waters_of(&run.world, Side::Allied), told, "seed {seed}");
        let allied = run.world.side_view(Side::Allied);
        if allied.submarine_reports().next().is_some() {
            reported_elsewhere += 1;
            // Where the boat lies, not at the waters' centre.
            assert!(allied
                .submarine_reports()
                .all(|r| (r.hex.q, r.hex.r) != (5, 0)));
        }
        if run.world.umpire().submarines[&BOAT].condition != SubmarineCondition::Sound {
            sunk_or_home += 1;
        }
    }
    assert!(reported_elsewhere > 20, "{reported_elsewhere}");
    assert!(sunk_or_home > 0);
}

#[test]
fn statement_26_a_side_given_no_waters_is_shown_nothing_until_a_boat_is_reported() {
    // Game D1 has no waters and no Allied boat: the Allied briefing has no
    // submarines part until a sighting is reported, and then only the
    // report.
    let d1 = game_d1((5, 0), "");
    let mut shown_empty = 0;
    let mut shown_report = 0;
    for seed in 1..=200 {
        let run = play(&d1, seed);
        let allied = run.world.side_view(Side::Allied);
        assert!(allied.suspected_waters().is_empty());
        let briefing = sim_agent::briefing(&allied, "scenario 128 D1");
        match (&briefing.submarines, allied.submarine_reports().count()) {
            (None, 0) => shown_empty += 1,
            (Some(part), n) if n > 0 => {
                assert_eq!(part.reports.len(), n);
                assert!(part.boats.is_empty() && part.suspected_waters.is_empty());
                shown_report += 1;
            }
            other => panic!("seed {seed}: {other:?}"),
        }
    }
    assert!(shown_empty > 0 && shown_report > 0);
}

/// The hash of day 1 that the repository pins for `name`'s run: the first
/// hash of its `run_` line in tests/golden/pinned_hashes.txt.
fn baseline_hash(name: &str) -> u64 {
    common::pinned_day_hashes(name)[0]
}

#[test]
fn statement_27_a_scenario_with_no_boat_and_no_waters_has_the_hash_it_had() {
    for (name, file) in [
        ("scenario_04a", "04a_night_action"),
        ("scenario_05b", "05b_strike"),
    ] {
        let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../data/scenarios")
            .join(format!("{file}.toml"));
        // On the frozen figures the hand-calculation tests use: the hash
        // pinned in the repository's file of pinned hashes.
        let mut frozen = common::load(&path).expect("the scenario loads");
        assert!(frozen.world.umpire().submarines.is_empty());
        assert!(frozen.world.umpire().waters_waiting.is_empty());
        frozen.run_to_end().expect("runs");
        let hash = frozen.world.state_hash();
        assert_eq!(
            hash,
            common::pinned_hash(&format!("scenario_128_statement_27_{name}"), hash),
            "{file} on the frozen figures"
        );
        // On the live figures, the hash the repository's run baseline gives
        // for the file's own seed, written before the rule was built.
        let mut live = common::load_live(&path).expect("the scenario loads");
        live.run_to_end().expect("runs");
        assert_eq!(
            live.world.state_hash(),
            baseline_hash(file),
            "{file} on the live figures"
        );
    }
}

/// Game of statement 28: an Allied force of a carrier and a destroyer at
/// battle readiness lies stopped in boat 1's hex all the night.
fn battle_readiness_game() -> Game {
    game(vec![carrier_and_destroyer()])
}

fn battle_readiness_text(game: &Game, extra: &str) -> String {
    let mut text = game.text();
    // A group can strike only where the scenario has the air combat figures.
    if extra.contains("[[air_group]]") {
        text = common::replace_once(
            &text,
            "surface_combat = \"../reference/surface_combat.toml\"\n",
            "surface_combat = \"../reference/surface_combat.toml\"\nair_combat = \"../reference/air_combat.toml\"\n",
        );
    }
    text = common::replace_once(
        &text,
        "mission = \"surface_combat\"\n\n[[ship]]",
        "mission = \"surface_combat\"\nreadiness = \"battle\"\n\n[[ship]]",
    );
    text.push_str(extra);
    text
}

fn is_surface_fight(kind: &EventKind) -> bool {
    matches!(
        kind,
        EventKind::EngagementBegun { .. }
            | EventKind::SightingRoll { .. }
            | EventKind::Salvo { .. }
            | EventKind::ForceBrokeOff { .. }
            | EventKind::EngagementEnded { .. }
    )
}

/// The strikes a group launched at ships or a base, and the contact ones.
fn strikes(run: &Run) -> usize {
    run.events
        .iter()
        .filter(|e| matches!(e.kind, EventKind::StrikeLaunched { .. }))
        .count()
}

#[test]
fn statement_28_no_rule_made_for_a_task_force_touches_a_boat() {
    // Part 1: a force at battle readiness lies in her hex all a night.
    let text = battle_readiness_text(&battle_readiness_game(), "");
    let part_1 =
        common::from_text_with(&text, &scenario_path(), &common::frozen()).expect("the game loads");
    let (mut fired, mut quiet) = (0, 0);
    for seed in 1..=SEEDS {
        let run = play(&part_1, seed);
        // No surface fight is recorded, whether she fires or not.
        assert!(
            !run.events.iter().any(|e| is_surface_fight(&e.kind)),
            "seed {seed}"
        );
        // The force is not given a contact of her, and she is in no force's
        // alert or contact list.
        assert_eq!(
            run.world.side_view(Side::Allied).contacts().count(),
            0,
            "seed {seed}"
        );
        if run.salvos().is_empty() {
            quiet += 1;
        } else {
            fired += 1;
        }
    }
    assert!(fired > 0 && quiet > 0, "{fired} fired, {quiet} did not");

    // Part 2: the carrier's group on naval strike, in reach of her report.
    // The group's aircraft are ready; the boat's salvo is reported to the
    // side at her hex, which the group is within 250 nm of. No strike is
    // launched at it.
    let group = "\n[[air_group]]\nid = 1\nname = \"Placeholder strike group\"\nside = \"allied\"\nship = 1\naircraft_type = 3\nready = 6\n\
                 \n[[order]]\nday = 1\nside = \"allied\"\nair_group = 1\nkind = \"set_air_mission\"\nmission = \"naval_strike\"\n";
    let text = battle_readiness_text(&battle_readiness_game().days(2), group);
    let part_2 =
        common::from_text_with(&text, &scenario_path(), &common::frozen()).expect("the game loads");
    let mut reported = 0;
    for seed in 1..=300 {
        let run = play(&part_2, seed);
        assert_eq!(strikes(&run), 0, "seed {seed}");
        if run
            .world
            .side_view(Side::Allied)
            .submarine_reports()
            .count()
            > 0
            || run
                .events
                .iter()
                .any(|e| matches!(e.kind, EventKind::SubmarineReported { .. }))
        {
            reported += 1;
        }
    }
    assert!(reported > 50, "{reported}");

    // The same group does launch at a contact of ships in reach: the
    // control that the group could have struck had there been a target.
    let contact = "\n[[contact]]\nside = \"allied\"\nq = 2\nr = 0\nship_count = 1\nkind = \"cruiser\"\nage_pulses = 0\n\
                   \n[[order]]\nday = 1\nside = \"allied\"\nair_group = 1\nkind = \"set_strike_contact_age\"\npulses = 4\n";
    let text = battle_readiness_text(
        &battle_readiness_game().days(2),
        &format!("{group}{contact}"),
    );
    let control =
        common::from_text_with(&text, &scenario_path(), &common::frozen()).expect("the game loads");
    let launched = (1..=50)
        .filter(|seed| strikes(&play(&control, *seed)) > 0)
        .count();
    assert!(launched > 0, "the control launched no strike");
}

#[test]
fn statement_29_every_draw_of_the_rule_has_its_own_value_in_a_subsystem_of_its_own() {
    let rule_draws = [
        Draw::SubmarineAttack,
        Draw::SubmarineTorpedo,
        Draw::SubmarineHunt,
        Draw::SubmarineSighted,
        Draw::SubmarineAirAttack,
        Draw::SubmarineReportError,
        Draw::SubmarineClaim,
    ];
    // Each has a code of its own, among the codes of all the draws.
    let all: Vec<u64> = Draw::ALL.iter().map(|d| d.code()).collect();
    let mut codes: Vec<u64> = rule_draws.iter().map(|d| d.code()).collect();
    for code in &codes {
        assert_eq!(all.iter().filter(|c| *c == code).count(), 1, "{code}");
    }
    codes.sort_unstable();
    codes.dedup();
    assert_eq!(codes.len(), rule_draws.len());
    assert_eq!(codes, (22..=28).collect::<Vec<u64>>());
    let mut every = all.clone();
    every.sort_unstable();
    every.dedup();
    assert_eq!(every.len(), all.len(), "no two draws share a code");
    // The subsystem is a value of its own.
    assert_eq!(Subsystem::Submarine.code(), 10);
    // And the draws are made in it: the attack's draw replayed under the
    // boat, the force and `Subsystem::Submarine` gives the result the log
    // has, for each chance of each seed of game A1.
    let a1 = game(vec![carrier_and_destroyer()]).scenario();
    let mut checked = 0;
    for seed in 1..=300 {
        let run = play(&a1, seed);
        for chance in run.chances() {
            let attacked = run
                .salvos()
                .iter()
                .any(|s| (s.day, s.pulse, s.force) == (chance.day, chance.pulse, chance.force));
            assert_eq!(
                run.attack_draw_succeeds(chance.day, chance.pulse, chance.force, chance.chance),
                attacked,
                "seed {seed}"
            );
            checked += 1;
        }
    }
    assert!(checked > 100);
}

/// What an event is, for comparing two logs whose numbers differ: its day,
/// pulse, kind and breakdown.
fn shape(event: &Event) -> String {
    format!(
        "{} {:?} {:?} {:?}",
        event.day, event.pulse, event.kind, event.breakdown
    )
}

fn names_boat_2(event: &Event) -> bool {
    event
        .kind
        .entities()
        .contains(&EntityRef::Submarine(SubmarineId::new(2)))
}

/// Game of statement 29: game A1 with a second Japanese boat that never
/// meets a force: she lies at (10,1), far from the force at (5,0), or, with
/// `sails`, is sent by a scripted station order to (11,1).
fn with_a_second_boat(sails: bool) -> Scenario {
    let mut text = game(vec![carrier_and_destroyer()]).text();
    text.push_str(
        "\n[[submarine]]\nid = 2\nname = \"Placeholder submarine 2\"\nside = \"japanese\"\nclass = 6900\nq = 10\nr = 1\nhome = 1\n",
    );
    if sails {
        text.push_str(
            "\n[[order]]\nday = 1\nside = \"japanese\"\nsubmarine = 2\nkind = \"station\"\nhex = [11, 1]\n",
        );
    }
    common::from_text_with(&text, &scenario_path(), &common::frozen()).expect("the game loads")
}

#[test]
fn statement_29_two_games_that_differ_only_in_a_boat_that_never_meets_a_force_draw_alike() {
    let one_boat = game(vec![carrier_and_destroyer()]).scenario();
    for sails in [false, true] {
        let two_boats = with_a_second_boat(sails);
        let mut fired = 0;
        for seed in 1..=SEEDS {
            let alone = play(&one_boat, seed);
            let with = play(&two_boats, seed);
            if !alone.salvos().is_empty() {
                fired += 1;
            }
            // Every event but the second boat's own is the same, in order,
            // with the same breakdown of every chance.
            let rest: Vec<String> = with
                .events
                .iter()
                .filter(|e| !names_boat_2(e))
                .map(shape)
                .collect();
            let first: Vec<String> = alone.events.iter().map(shape).collect();
            assert_eq!(first, rest, "seed {seed}, sails {sails}");
            // Where she lies quiet she has no event at all, so the logs
            // are the same event for event, numbers and causes too.
            if !sails {
                assert_eq!(alone.events, with.events, "seed {seed}");
            } else {
                assert!(with.events.iter().any(names_boat_2), "seed {seed}");
            }
            // The outcomes: the ships, the forces, the first boat and the
            // claims of both sides.
            let (a, w) = (alone.world.umpire(), with.world.umpire());
            assert_eq!(a.ships, w.ships, "seed {seed}");
            assert_eq!(a.task_forces, w.task_forces, "seed {seed}");
            assert_eq!(a.submarines[&BOAT], w.submarines[&BOAT], "seed {seed}");
            for side in Side::ALL {
                // The claims, but for the number of the event that gave
                // them, which the second boat's events move.
                let claims = |run: &Run| -> Vec<sim_core::belief::BattleClaim> {
                    run.world
                        .side_view(side)
                        .claims()
                        .iter()
                        .map(|c| sim_core::belief::BattleClaim {
                            received_event: alone.events[0].id,
                            ..c.clone()
                        })
                        .collect()
                };
                assert_eq!(claims(&alone), claims(&with), "seed {seed}");
                assert_eq!(
                    alone
                        .world
                        .side_view(side)
                        .submarine_reports()
                        .map(|r| (r.hex, r.age_pulses))
                        .collect::<Vec<_>>(),
                    with.world
                        .side_view(side)
                        .submarine_reports()
                        .map(|r| (r.hex, r.age_pulses))
                        .collect::<Vec<_>>(),
                    "seed {seed}"
                );
            }
        }
        assert!(fired > 100, "{fired}");
    }
}

#[test]
fn a_scripted_station_order_sends_a_boat_to_a_new_station() {
    // Boat 1 lies at (5,0), 4 hexes from (9,0): 120 nm in the night, 40
    // more in the morning, where she arrives and is on station from the
    // afternoon.
    let mut text = game(Vec::new()).text();
    text.push_str(
        "\n[[order]]\nday = 1\nside = \"japanese\"\nsubmarine = 1\nkind = \"station\"\nhex = [9, 0]\n",
    );
    let scenario =
        common::from_text_with(&text, &scenario_path(), &common::frozen()).expect("the game loads");
    // The order is in the script as the engine's order.
    let scripted = &scenario.scripted_orders()[&1];
    assert_eq!(scripted.len(), 1);
    assert_eq!(scripted[0].0, Side::Japanese);
    assert_eq!(
        scripted[0].1.kind,
        sim_core::orders::OrderKind::Submarine {
            target: BOAT,
            order: sim_core::orders::SubmarineOrder::Station(sim_core::hex::Hex::new(9, 0)),
        }
    );
    let run = play(&scenario, 1);
    let boat = run.boat();
    assert_eq!((boat.station.q, boat.station.r), (9, 0));
    assert_eq!((boat.hex.q, boat.hex.r), (9, 0));
    assert!(boat.on_station(1));
    let order = boat.station_order.expect("her order is kept");
    let ordered: Vec<&Event> = run
        .events
        .iter()
        .filter(|e| matches!(e.kind, EventKind::SubmarineOrderedToStation { .. }))
        .collect();
    assert_eq!(ordered.len(), 1);
    assert_eq!(
        ordered[0].causes,
        vec![sim_core::events::Cause::Order(order)]
    );
    // She steamed in the night and the morning and reached her station in
    // the morning.
    let steamed: Vec<(Pulse, (i32, i32))> = run
        .events
        .iter()
        .filter_map(|e| match e.kind {
            EventKind::SubmarineSteamed { to, .. } => {
                Some((e.pulse.expect("a pulse"), (to.q, to.r)))
            }
            _ => None,
        })
        .collect();
    assert_eq!(
        steamed,
        vec![(Pulse::Night, (8, 0)), (Pulse::Morning, (9, 0))]
    );
    assert!(run
        .events
        .iter()
        .any(|e| matches!(e.kind, EventKind::SubmarineOnStation { .. })
            && e.pulse == Some(Pulse::Morning)));
    // Her side is told of the order; the other side is not.
    let seen = |side: Side| {
        run.world
            .side_view(side)
            .events()
            .iter()
            .filter(|e| {
                matches!(
                    e.kind,
                    SideEventKind::Whole(EventKind::SubmarineOrderedToStation { .. })
                )
            })
            .count()
    };
    assert_eq!((seen(Side::Japanese), seen(Side::Allied)), (1, 0));
}

#[test]
fn a_scripted_station_order_names_exactly_one_unit_and_a_submarine_takes_only_station() {
    let bad = |extra: &str| {
        let mut text = game(Vec::new()).text();
        text.push_str(extra);
        match common::from_text_with(&text, &scenario_path(), &common::frozen()) {
            Ok(_) => panic!("the game was accepted"),
            Err(error) => error.to_string(),
        }
    };
    let two = bad(
        "\n[[order]]\nday = 1\nside = \"japanese\"\nsubmarine = 1\ntask_force = 1\nkind = \"station\"\nhex = [9, 0]\n",
    );
    assert!(two.contains("exactly one of"), "{two}");
    let other = bad(
        "\n[[order]]\nday = 1\nside = \"japanese\"\nsubmarine = 1\nkind = \"march\"\nhex = [9, 0]\n",
    );
    assert!(other.contains("must be station"), "{other}");
    let wrong = bad(
        "\n[[order]]\nday = 1\nside = \"japanese\"\ntask_force = 1\nkind = \"station\"\nhex = [9, 0]\n",
    );
    assert!(wrong.contains("submarine"), "{wrong}");
}

#[test]
fn suspected_waters_in_a_scenario_are_refused_off_the_map_on_day_0_or_after_the_last_day() {
    let bad = |extra: &str| {
        let mut text = game(Vec::new()).text();
        text.push_str(extra);
        match common::from_text_with(&text, &scenario_path(), &common::frozen()) {
            Ok(_) => panic!("the game was accepted"),
            Err(error) => error.to_string(),
        }
    };
    let waters = |q: i32, day: u32| {
        format!(
            "\n[[suspected_waters]]\nside = \"allied\"\nq = {q}\nr = 0\ndistance = 2\nday = {day}\nwords = \"W.\"\nrows = \"R.\"\nreading = \"A.\"\n"
        )
    };
    assert!(bad(&waters(500, 1)).contains("off the map"));
    assert!(bad(&waters(5, 0)).contains("not a day of the game"));
    assert!(bad(&waters(5, 2)).contains("not a day of the game"));
    // Words, rows and a reading are all required.
    let empty = "\n[[suspected_waters]]\nside = \"allied\"\nq = 5\nr = 0\ndistance = 2\nwords = \" \"\nrows = \"R.\"\nreading = \"A.\"\n";
    assert!(bad(empty).contains("no words"));
    // Day 1 is the default.
    let mut text = game(Vec::new()).text();
    text.push_str(
        "\n[[suspected_waters]]\nside = \"allied\"\nq = 5\nr = 0\ndistance = 2\nwords = \"W.\"\nrows = \"R.\"\nreading = \"A.\"\n",
    );
    let scenario =
        common::from_text_with(&text, &scenario_path(), &common::frozen()).expect("the game loads");
    assert_eq!(
        scenario
            .world
            .side_view(Side::Allied)
            .suspected_waters()
            .len(),
        1
    );
    // A world made again for another seed holds them too.
    let other = scenario.with_seed(77).expect("a seed");
    assert_eq!(
        other.world.side_view(Side::Allied).suspected_waters().len(),
        1
    );
    assert_eq!(other.world.state_hash(), {
        let again = scenario.with_seed(77).expect("a seed");
        again.world.state_hash()
    });
}
