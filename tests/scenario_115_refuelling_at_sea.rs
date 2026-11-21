//! Scenario 115: refuelling at sea. A force fuels from an oiler's force
//! while both steam slowly together, by day or by night, a few ships at a
//! time; an attack breaks it off; the staff warns of a force running low
//! and sends it to an oiler nearer than home.
//!
//! Proves decision 0201, stage A. The statements are the architect's,
//! written before the code. A mismatch goes back to the architect, and
//! nobody else changes a statement or a number here.
//!
//! # The setting
//!
//! A placeholder scenario made for the test, on a placeholder map of open
//! sea, with no real ship. Hexes are 40 nm and are named (q,r). A day has a
//! night pulse of 12 hours and a morning and an afternoon pulse of 6 hours
//! each.
//!
//! Side A has a force F: a carrier CV with a fuel capacity of 4,000 t, and
//! two destroyers D1 and D2 with 500 t each, in id order CV, D1, D2. It has
//! a force O with one oiler, an auxiliary whose class has the mark, with
//! room for 3,000 t of cargo and 3,000 t of cargo fuel aboard, and her own
//! fuel full. Both cruise at 15 kn. Unless a statement says otherwise, F
//! and O lie stopped in (0,0) with no route, there is no base in the hex
//! and no enemy, and F is under the order to fuel from O.
//!
//! The figures: the fuelling speed is 8 kn; a line passes 250 t an hour; a
//! ship is 1 hour alongside before her fuel passes; an oiler has two lines.
//!
//! Ships are started in the order of rule 13, and no ship starts before one
//! that is earlier in the order has started.
//!
//! # Statements: fuelling
//!
//! CV holds 2,000 t (half), D1 100 t (a fifth) and D2 300 t (three
//! fifths). The order is D1, CV, D2.
//!
//! 1. **A morning.** In the morning pulse, 6 hours:
//!    - D1 starts at once on one line. She is alongside for 1 hour and
//!      then takes the 400 t she lacks in 1.6 hours: full at 2.6 hours.
//!    - CV needs both lines, and has them from 2.6 hours. She is alongside
//!      to 3.6, and takes 500 t an hour for the 2.4 hours left: 1,200 t.
//!    - D2 comes after CV and no line is free before the pulse ends: she
//!      takes nothing.
//!    At the pulse's end D1 holds 500 t, CV 3,200 t, D2 300 t, and the
//!    oiler 1,400 t of cargo fuel. Two events of a ship fuelled at sea are
//!    recorded, of 400 t and 1,200 t, each naming the oiler.
//! 2. **The afternoon after it.** The order is now D2 (three fifths), CV
//!    (four fifths). D2 is alongside for 1 hour and takes 200 t in 0.8
//!    hours: full at 1.8. CV has both lines from 1.8, is alongside to 2.8,
//!    and takes the 800 t she lacks in 1.6 hours: full at 4.4. Every ship
//!    is full, the order ends, and F is idle at cruise with no route. The
//!    oiler holds 400 t.
//! 3. **By night.** The same three ships in a night pulse of 12 hours: D1
//!    is full at 2.6 hours; CV is alongside from 2.6 to 3.6 and takes her
//!    2,000 t in 4 hours, full at 7.6; D2 starts at 7.6, is alongside to
//!    8.6 and takes 200 t in 0.8 hours, full at 9.4. All in one night; the
//!    order ends; the oiler holds 400 t.
//! 4. **The oiler runs out.** With 300 t of cargo fuel, in a morning: D1
//!    takes 300 t and holds 400 t; the oiler holds none; CV and D2 take
//!    nothing; the order ends, and its event says the giver holds no
//!    cargo fuel.
//! 5. **Two oilers.** O has a second oiler, of a higher id, with 3,000 t;
//!    the first holds 300 t. In a morning D1 goes to the first, takes its
//!    300 t and holds 400 t. CV goes to the second, both of whose lines
//!    are free: alongside to 1 hour, then 2,000 t in 4 hours, full at 5.
//!    D2 has no line with fuel before 5, starts on the second at 5, and is
//!    alongside when the pulse ends: she takes nothing.
//! 6. **An oiler takes nothing.** With an oiler of F's own, her own fuel
//!    at half, she is not in the order and takes nothing.
//! 7. **Tons are kept.** In every statement the fuel the ships take is the
//!    cargo fuel the oilers give, and no ship holds more than her
//!    capacity.
//!
//! # Statements: steaming
//!
//! 8. **Slow together.** O has a route east at cruise and F is in her hex
//!    under the order with ships lacking fuel. In the morning pulse both
//!    make 48 nm on O's route and are 8 nm past the centre of (1,0) at
//!    its end, in one hex still; fuel passes as in statement 1, less what
//!    the ships burn in the 48 nm, which is burned before the fuel passes.
//!    The speed F and O report in that pulse is 8 kn. In the night pulse
//!    they make 96 nm. With F's ships full O makes 90 nm in a morning, as
//!    today.
//! 9. **Not together at the start.** F begins a morning in (1,0) under
//!    the order, O stopped in (0,0). F steers for O and reaches her hex
//!    in the pulse. No fuel passes in that pulse and F is not slowed in
//!    it. In the afternoon fuel passes.
//! 10. **Following.** With O on a route and F in her hex under the order
//!     and full but for 1 t in one ship, F takes O's route, and the two
//!     are in one hex at the end of each pulse until the order ends.
//!
//! # Statements: broken off
//!
//! 11. **A raid.** In a morning in which an enemy raid attacks a ship of
//!     F, no fuel passes: every ship of F holds what it held, the oiler
//!     holds 3,000 t, and an event says the fuelling was broken off and
//!     cites the raid's attack. Both forces still made no more than 48 nm.
//!     The same where the raid attacks the oiler. In the afternoon, with
//!     no attack, statement 1's fuelling takes place.
//! 12. **A surface action.** In a pulse in which O fights a surface
//!     action, no fuel passes and the event cites the action.
//!
//! # Statements: the order
//!
//! 13. **Refused.** The order is refused, each with its own reason, where:
//!     the giver has no ship with the mark; the giver is the force
//!     itself; the giver is under the order herself; the giver is an
//!     escort; the giver has an escort; the force is the giver for
//!     another force; the force is an escort; the force has an escort.
//! 14. **It ends.** Besides statements 2 and 4, the order ends, with its
//!     event saying why, when: the oiler is sunk; the oiler is detached
//!     from O; the commander gives F a route. After each F is at cruise
//!     with no route but the one the commander gave, and no fuel passes
//!     in later pulses.
//! 15. **A force that has run dry.** F is out of fuel, adrift where it
//!     stopped, D1 holding nothing.
//!     - O steams through F's hex on a route: no fuel passes, and O is
//!       not slowed.
//!     - O lies stopped in F's hex: fuel passes as in statement 1 with D1
//!       first, and the first ship fuelled sets F idle where it is with
//!       its route dropped.
//! 16. **A port before an oiler.** F and O lie stopped in a hex with a
//!     base of A's that holds fuel and can fuel them. The ships fuel from
//!     the base by the rule in force today, take nothing from the oiler
//!     in that pulse, and the oiler's cargo fuel is unchanged. In a pulse
//!     in which the oiler loads cargo fuel at the base she gives none.
//! 17. **Who knows.** A's own view gives every event of statements 1 to
//!     16. B's view gives none of them.
//!
//! # Statements: the staff
//!
//! 18. **The warning.** A force of A's with no home port has a destroyer
//!     at 120 t of 500: below a quarter, 125 t. The staff's report names
//!     the force and the ship each day. With the destroyer at 130 t it
//!     does not. With no oiler on A's side the staff gives the force no
//!     order because of it.
//! 19. **Sent to the oiler.** A force on a stand, with a home port 10
//!     hexes away, is short of fuel by the stand doctrine's own measure
//!     and would be routed home today. With O 4 hexes away holding cargo
//!     fuel, the staff gives it the order to fuel from O, and its reason
//!     gives both passages. While the order stands the staff gives it no
//!     route. On the morning after the order ends it is routed back to
//!     its station.
//! 20. **Home is nearer, or the oiler is empty.** With O 12 hexes away the
//!     force is routed home as today. With O 4 hexes away holding no
//!     cargo fuel it is routed home as today.
//! 21. **No home port.** A carrier force with no home port and a
//!     destroyer at 120 t of 500, O holding cargo fuel: the staff gives
//!     it the order to fuel from O.
//!
//! # Properties
//!
//! - **No game on file moves.** Every scenario and recorded game on file
//!   gives the events it gave before, and the daily hashes it gave before
//!   but for the state's layout version; the golden files are remade for
//!   the new layout and for nothing else.
//! - **Nothing is drawn.** Fuelling at sea draws no random number.
//! - **A class file with no column for the mark loads,** and no ship of it
//!   has the mark.

// The hand calculation above is the architect's, word for word, and its
// statement 1 continues a list item without indentation.
#![allow(clippy::doc_lazy_continuation)]

mod common;

use std::path::PathBuf;
use std::sync::OnceLock;

use sim_core::events::{Cause, Event, EventKind, FuelOrderEnd};
use sim_core::fixed::Fixed;
use sim_core::hex::Hex;
use sim_core::ids::{EventId, ShipId, TaskForceId};
use sim_core::orders::{OrderKind, RejectReason, SpeedSetting, TaskForceOrder};
use sim_core::side_events::SideEventKind;
use sim_core::state::{Side, TaskForceStatus};
use sim_core::time::Pulse;
use sim_data::{LoadOptions, Scenario};
use sim_staff::{as_staffs, staffs};

// ------------------------------------------------------------ The harness

fn path() -> PathBuf {
    common::data_dir().join("scenarios/refuelling-at-sea/115_refuelling_at_sea.toml")
}

fn text() -> String {
    std::fs::read_to_string(path()).expect("the scenario file reads")
}

fn changed(text: &str, original: &str, replacement: &str) -> String {
    common::replace_once(text, original, replacement)
}

/// The four figures of the setting, as the hand calculation gives them.
const AT_SEA: &str =
    "\n[fuelling_at_sea]\nspeed_kn = 8\nline_rate_t_per_h = 250\nalongside_hours = 1\nlines = 2\n";

/// The frozen step 1 figures, with the scenario's own placeholder classes
/// beside them and the four figures of fuelling at sea added to the surface
/// combat figures, which the frozen figures do not have.
fn options() -> LoadOptions {
    static OPTIONS: OnceLock<LoadOptions> = OnceLock::new();
    OPTIONS
        .get_or_init(|| {
            common::altered_reference("s115", |root| {
                let classes = "ship_classes_placeholder_scenario_115.csv";
                std::fs::copy(
                    common::data_dir().join("reference").join(classes),
                    root.join(classes),
                )
                .expect("copy the scenario's classes");
                common::edit_file(root, "surface_combat.toml", |text| text + AT_SEA);
            })
        })
        .clone()
}

fn load(text: &str) -> Scenario {
    common::from_text_with(text, &path(), &options()).expect("the scenario loads")
}

fn run(text: &str) -> Scenario {
    let mut scenario = load(text);
    scenario.run_to_end().expect("the run resolves");
    scenario
}

/// Runs with each side's staff asked each morning.
fn run_staffed(text: &str) -> Scenario {
    let mut scenario = load(text);
    let held = staffs();
    while scenario
        .next_day_staffed(as_staffs(&held))
        .expect("the day resolves")
        .is_some()
    {}
    scenario
}

fn events(scenario: &Scenario) -> Vec<Event> {
    scenario.world.events().iter().cloned().collect()
}

fn fx(text: &str) -> Fixed {
    text.parse().expect("a decimal")
}

fn t(n: i64) -> Fixed {
    Fixed::from_int(n)
}

fn force(raw: u32) -> TaskForceId {
    TaskForceId::new(raw)
}

fn ship(raw: u32) -> ShipId {
    ShipId::new(raw)
}

fn fuel(scenario: &Scenario, raw: u32) -> Fixed {
    scenario.world.umpire().ships[&ship(raw)].fuel_t
}

fn cargo_fuel(scenario: &Scenario, raw: u32) -> Fixed {
    scenario.world.umpire().ships[&ship(raw)].cargo_fuel_t
}

/// One ship fuelled at sea, as the log has it.
#[derive(Clone, Debug, PartialEq)]
struct Fuelled {
    day: u32,
    pulse: Pulse,
    ship: u32,
    oiler: u32,
    tons: Fixed,
    fuel_t: Fixed,
    oiler_left: Fixed,
    restored: bool,
    start: Option<Fixed>,
    event: EventId,
}

fn fuelled(log: &[Event]) -> Vec<Fuelled> {
    log.iter()
        .filter_map(|e| match &e.kind {
            EventKind::ShipFuelledAtSea {
                ship,
                oiler,
                tons,
                fuel_t,
                oiler_cargo_fuel_t,
                restored,
                ..
            } => Some(Fuelled {
                day: e.day,
                pulse: e.pulse.expect("in a pulse"),
                ship: ship.raw(),
                oiler: oiler.raw(),
                tons: *tons,
                fuel_t: *fuel_t,
                oiler_left: *oiler_cargo_fuel_t,
                restored: *restored,
                start: e.breakdown.as_ref().and_then(|b| {
                    b.factors
                        .iter()
                        .find(|f| f.name == "start_hours")
                        .map(|f| f.value)
                }),
                event: e.id,
            }),
            _ => None,
        })
        .collect()
}

fn fuelled_in(log: &[Event], day: u32, pulse: Pulse) -> Vec<(u32, Fixed)> {
    fuelled(log)
        .into_iter()
        .filter(|f| f.day == day && f.pulse == pulse)
        .map(|f| (f.ship, f.tons))
        .collect()
}

fn ended(log: &[Event]) -> Vec<(u32, Option<Pulse>, FuelOrderEnd, Vec<Cause>)> {
    log.iter()
        .filter_map(|e| match &e.kind {
            EventKind::FuelOrderEnded { reason, .. } => {
                Some((e.day, e.pulse, *reason, e.causes.clone()))
            }
            _ => None,
        })
        .collect()
}

fn broken_off(log: &[Event]) -> Vec<&Event> {
    log.iter()
        .filter(|e| matches!(e.kind, EventKind::FuellingBrokenOff { .. }))
        .collect()
}

/// Each steaming of `id`: the day, the pulse, the miles, the speed, from and
/// to.
fn steamed(log: &[Event], id: u32) -> Vec<(u32, Pulse, Fixed, Fixed, Hex, Hex)> {
    log.iter()
        .filter_map(|e| match &e.kind {
            EventKind::TaskForceSteamed {
                task_force,
                from,
                to,
                distance_nm,
                speed_kn,
                ..
            } if *task_force == force(id) => Some((
                e.day,
                e.pulse.expect("in a pulse"),
                *distance_nm,
                *speed_kn,
                *from,
                *to,
            )),
            _ => None,
        })
        .collect()
}

fn burned(log: &[Event], raw: u32, day: u32, pulse: Pulse) -> Fixed {
    log.iter()
        .filter(|e| e.day == day && e.pulse == Some(pulse))
        .filter_map(|e| match &e.kind {
            EventKind::FuelBurned {
                ship: s, burned_t, ..
            } if *s == ship(raw) => Some(*burned_t),
            _ => None,
        })
        .fold(Fixed::ZERO, |sum, tons| sum + tons)
}

fn rejections(log: &[Event]) -> Vec<RejectReason> {
    log.iter()
        .filter_map(|e| match &e.kind {
            EventKind::OrderRejected { reason, .. } => Some(reason.clone()),
            _ => None,
        })
        .collect()
}

/// Statement 7: the fuel the ships take is the cargo fuel the oilers give,
/// and no ship holds more than her capacity.
fn tons_are_kept(scenario: &Scenario, oilers_at_start: &[(u32, i64)]) {
    let log = events(scenario);
    let taken = fuelled(&log)
        .iter()
        .fold(Fixed::ZERO, |sum, f| sum + f.tons);
    let given = oilers_at_start
        .iter()
        .fold(Fixed::ZERO, |sum, (raw, start)| {
            sum + t(*start) - cargo_fuel(scenario, *raw)
        });
    assert_eq!(taken, given, "the fuel taken is the cargo fuel given");
    for record in scenario.world.umpire().ships.values() {
        let class = &scenario.world.ref_data().ship_classes[&record.class];
        assert!(
            record.fuel_t <= class.fuel_capacity_t,
            "{} over capacity",
            record.id
        );
    }
}

/// The main file's opening: F begins in (1,0), steers for O in the night and
/// reaches her hex. These replace it with F in (0,0) from the start, the
/// fuel it would have had there.
fn f_in_o_hex(text: &str) -> String {
    let text = changed(
        text,
        "q = 1\nr = 0\nmission = \"surface_combat\"",
        "q = 0\nr = 0\nmission = \"surface_combat\"",
    );
    let text = changed(&text, "fuel_t = 2020", "fuel_t = 2000");
    let text = changed(&text, "fuel_t = 104", "fuel_t = 100");
    changed(&text, "fuel_t = 304", "fuel_t = 300")
}

fn order(day: u32, side: &str, target: &str, body: &str) -> String {
    format!("\n[[order]]\nday = {day}\nside = \"{side}\"\n{target}\n{body}\n")
}

fn task_force(id: u32, side: &str, q: i32, r: i32, mission: &str) -> String {
    format!(
        "\n[[task_force]]\nid = {id}\nname = \"Placeholder force {id}\"\nside = \"{side}\"\nq = {q}\nr = {r}\nmission = \"{mission}\"\n"
    )
}

fn a_ship(id: u32, class: u32, side: &str, fuel: i64, tf: u32, cargo_fuel: Option<i64>) -> String {
    let cargo = cargo_fuel.map_or_else(String::new, |c| format!("cargo_fuel_t = {c}\n"));
    format!(
        "\n[[ship]]\nid = {id}\nname = \"Placeholder ship {id}\"\nclass = {class}\nside = \"{side}\"\nfuel_t = {fuel}\n{cargo}task_force = {tf}\n"
    )
}

// ------------------------------------------------------ Statements: fuelling

#[test]
fn statements_1_and_2_a_morning_and_the_afternoon_after_it() {
    let scenario = run(&text());
    let log = events(&scenario);
    // Statement 1: the morning.
    assert_eq!(
        fuelled_in(&log, 1, Pulse::Morning),
        vec![(2, t(400)), (1, t(1200))],
        "D1 takes 400 t, CV 1,200 t, D2 nothing"
    );
    let morning: Vec<Fuelled> = fuelled(&log)
        .into_iter()
        .filter(|f| f.pulse == Pulse::Morning)
        .collect();
    assert_eq!(morning[0].fuel_t, t(500), "D1 holds 500 t");
    assert_eq!(morning[1].fuel_t, t(3200), "CV holds 3,200 t");
    assert_eq!(morning[1].oiler_left, t(1400), "the oiler holds 1,400 t");
    assert!(morning.iter().all(|f| f.oiler == 4), "each names the oiler");
    assert_eq!(morning[0].start, Some(Fixed::ZERO));
    assert_eq!(
        morning[1].start,
        Some(fx("2.6")),
        "CV has both lines from 2.6"
    );
    // Statement 2: the afternoon, D2 then CV, both full; the order ends.
    let afternoon: Vec<Fuelled> = fuelled(&log)
        .into_iter()
        .filter(|f| f.pulse == Pulse::Afternoon)
        .collect();
    assert_eq!(
        afternoon
            .iter()
            .map(|f| (f.ship, f.tons))
            .collect::<Vec<_>>(),
        vec![(3, t(200)), (1, t(800))]
    );
    assert_eq!(
        afternoon[1].start,
        Some(fx("1.8")),
        "CV has both lines from 1.8"
    );
    assert_eq!(
        ended(&log)
            .iter()
            .map(|(d, p, r, _)| (*d, *p, *r))
            .collect::<Vec<_>>(),
        vec![(1, Some(Pulse::Afternoon), FuelOrderEnd::Full)]
    );
    let f = &scenario.world.umpire().task_forces[&force(1)];
    assert_eq!(f.status, TaskForceStatus::Idle);
    assert_eq!(f.speed, SpeedSetting::Cruise);
    assert!(f.route.waypoints.is_empty(), "no route");
    assert!(f.fuel_from.is_none());
    assert_eq!(
        (fuel(&scenario, 1), fuel(&scenario, 2), fuel(&scenario, 3)),
        (t(4000), t(500), t(500))
    );
    assert_eq!(cargo_fuel(&scenario, 4), t(400));
    tons_are_kept(&scenario, &[(4, 3000)]);
}

#[test]
fn statement_3_by_night() {
    let scenario = run(&f_in_o_hex(&text()));
    let log = events(&scenario);
    let night: Vec<Fuelled> = fuelled(&log)
        .into_iter()
        .filter(|f| f.day == 1 && f.pulse == Pulse::Night)
        .collect();
    assert_eq!(
        night
            .iter()
            .map(|f| (f.ship, f.tons, f.start))
            .collect::<Vec<_>>(),
        vec![
            (2, t(400), Some(Fixed::ZERO)),
            (1, t(2000), Some(fx("2.6"))),
            (3, t(200), Some(fx("7.6"))),
        ],
        "D1 full at 2.6; CV alongside 2.6 to 3.6 and full at 7.6; D2 from 7.6"
    );
    assert_eq!(fuelled(&log).len(), 3, "all in one night");
    assert_eq!(ended(&log)[0].2, FuelOrderEnd::Full);
    assert_eq!(ended(&log)[0].1, Some(Pulse::Night));
    assert_eq!(cargo_fuel(&scenario, 4), t(400));
    tons_are_kept(&scenario, &[(4, 3000)]);
}

#[test]
fn statement_4_the_oiler_runs_out() {
    let scenario = run(&changed(
        &text(),
        "cargo_fuel_t = 3000",
        "cargo_fuel_t = 300",
    ));
    let log = events(&scenario);
    assert_eq!(fuelled_in(&log, 1, Pulse::Morning), vec![(2, t(300))]);
    assert_eq!(fuelled(&log).len(), 1, "CV and D2 take nothing");
    assert_eq!(fuel(&scenario, 2), t(400));
    assert_eq!(cargo_fuel(&scenario, 4), Fixed::ZERO);
    let ends = ended(&log);
    assert_eq!(ends.len(), 1);
    assert_eq!(
        (ends[0].1, ends[0].2),
        (Some(Pulse::Morning), FuelOrderEnd::GiverEmpty)
    );
    tons_are_kept(&scenario, &[(4, 300)]);
}

#[test]
fn statement_5_two_oilers() {
    let text = changed(&text(), "cargo_fuel_t = 3000", "cargo_fuel_t = 300")
        + &a_ship(5, 153, "allied", 2000, 2, Some(3000));
    let scenario = run(&text);
    let log = events(&scenario);
    let morning: Vec<Fuelled> = fuelled(&log)
        .into_iter()
        .filter(|f| f.day == 1 && f.pulse == Pulse::Morning)
        .collect();
    assert_eq!(
        morning
            .iter()
            .map(|f| (f.ship, f.oiler, f.tons, f.fuel_t, f.start))
            .collect::<Vec<_>>(),
        vec![
            (2, 4, t(300), t(400), Some(Fixed::ZERO)),
            (1, 5, t(2000), t(4000), Some(Fixed::ZERO)),
        ],
        "D1 from the first oiler, CV from the second from the start; D2 alongside at the end takes nothing"
    );
    tons_are_kept(&scenario, &[(4, 300), (5, 3000)]);
}

#[test]
fn statement_6_an_oiler_takes_nothing() {
    // An oiler of F's own, her own fuel at half.
    let text = text() + &a_ship(6, 153, "allied", 1000, 1, None);
    let scenario = run(&text);
    let log = events(&scenario);
    assert!(
        fuelled(&log).iter().all(|f| f.ship != 6),
        "she takes nothing"
    );
    assert_eq!(
        fuel(&scenario, 6),
        t(1000) - burned(&log, 6, 1, Pulse::Night)
    );
    assert_eq!(
        ended(&log)[0].2,
        FuelOrderEnd::Full,
        "the order ends with her at half"
    );
    tons_are_kept(&scenario, &[(4, 3000)]);
}

// ------------------------------------------------------ Statements: steaming

/// O is routed east at 0600 of day 1, inside the day.
fn o_routed_in_the_morning(text: &str) -> String {
    let text = changed(text, "days = 2\n", "days = 2\norders_inside_a_day = true\n");
    text + &order(
        1,
        "allied",
        "task_force = 2",
        "pulse = \"morning\"\nkind = \"set_route\"\nwaypoints = [[8, 0]]",
    )
}

#[test]
fn statement_8_slow_together_by_day() {
    let scenario = run(&o_routed_in_the_morning(&text()));
    let log = events(&scenario);
    let morning = |id| -> Vec<_> {
        steamed(&log, id)
            .into_iter()
            .filter(|s| s.0 == 1 && s.1 == Pulse::Morning)
            .collect()
    };
    for id in [1, 2] {
        let legs = morning(id);
        assert_eq!(legs.len(), 1, "one segment for {id}");
        let (_, _, nm, kn, from, to) = legs[0];
        assert_eq!(
            (nm, kn, from, to),
            (t(48), t(8), Hex::new(0, 0), Hex::new(1, 0))
        );
    }
    // The fuel passes as in statement 1, with what the 48 nm burned taken
    // first: D1 lacks 400 t and her burn, CV starts when D1 is done.
    let b_cv = burned(&log, 1, 1, Pulse::Morning);
    let b_d1 = burned(&log, 2, 1, Pulse::Morning);
    let b_d2 = burned(&log, 3, 1, Pulse::Morning);
    assert!(b_cv > Fixed::ZERO && b_d1 > Fixed::ZERO);
    let burned_at = log
        .iter()
        .rposition(|e| {
            e.day == 1
                && e.pulse == Some(Pulse::Morning)
                && matches!(e.kind, EventKind::FuelBurned { .. })
        })
        .expect("a burn");
    let first_fuel = log
        .iter()
        .position(|e| matches!(e.kind, EventKind::ShipFuelledAtSea { .. }))
        .expect("a fuelling");
    assert!(burned_at < first_fuel, "burned before the fuel passes");
    let d1_lacks = t(400) + b_d1;
    let d1_done = t(1) + d1_lacks / t(250);
    let cv_tons = t(500) * (t(6) - d1_done - t(1));
    assert_eq!(
        fuelled_in(&log, 1, Pulse::Morning),
        vec![(2, d1_lacks), (1, cv_tons)]
    );
    let morning_fuel: Vec<Fuelled> = fuelled(&log)
        .into_iter()
        .filter(|f| f.pulse == Pulse::Morning)
        .collect();
    assert_eq!(morning_fuel[1].fuel_t, t(2000) - b_cv + cv_tons);
    let _ = b_d2;
    tons_are_kept(&scenario, &[(4, 3000)]);
}

#[test]
fn statement_8_slow_together_by_night_and_not_slowed_when_full() {
    // By night: O routed from the evening, F in her hex.
    let text = f_in_o_hex(&text())
        + &order(
            1,
            "allied",
            "task_force = 2",
            "kind = \"set_route\"\nwaypoints = [[12, 0]]",
        );
    let log = events(&run(&text));
    for id in [1, 2] {
        let night: Vec<_> = steamed(&log, id)
            .into_iter()
            .filter(|s| s.0 == 1 && s.1 == Pulse::Night)
            .collect();
        assert_eq!(
            (night[0].2, night[0].3),
            (t(96), t(8)),
            "{id} makes 96 nm by night"
        );
    }
    // With F's ships full, O makes 90 nm in a morning.
    let full = changed(&self::text(), "fuel_t = 2020", "fuel_t = 4000");
    let full = changed(&full, "fuel_t = 104", "fuel_t = 500");
    let full = changed(&full, "fuel_t = 304", "fuel_t = 500");
    let full = changed(
        &full,
        "q = 1\nr = 0\nmission = \"surface_combat\"",
        "q = 0\nr = 0\nmission = \"surface_combat\"",
    );
    let log = events(&run(&o_routed_in_the_morning(&full)));
    let morning: Vec<_> = steamed(&log, 2)
        .into_iter()
        .filter(|s| s.0 == 1 && s.1 == Pulse::Morning)
        .collect();
    assert_eq!((morning[0].2, morning[0].3), (t(90), t(15)));
    assert!(fuelled(&log).is_empty());
}

#[test]
fn statement_9_not_together_at_the_start() {
    // F begins a morning in (1,0): it starts the night six hexes east and
    // steams 180 nm toward O, ending 20 nm into the leg to (1,0).
    let text = changed(
        &text(),
        "q = 1\nr = 0\nmission = \"surface_combat\"",
        "q = 6\nr = 0\nmission = \"surface_combat\"",
    );
    let scenario = run(&text);
    let log = events(&scenario);
    let night: Vec<_> = steamed(&log, 1)
        .into_iter()
        .filter(|s| s.1 == Pulse::Night)
        .collect();
    assert_eq!(night[0].5, Hex::new(1, 0), "F begins the morning in (1,0)");
    let morning: Vec<_> = steamed(&log, 1)
        .into_iter()
        .filter(|s| s.0 == 1 && s.1 == Pulse::Morning)
        .collect();
    assert_eq!(
        morning[0].5,
        Hex::new(0, 0),
        "F reaches O's hex in the morning"
    );
    assert_eq!(morning[0].3, t(15), "F is not slowed");
    assert!(
        fuelled_in(&log, 1, Pulse::Morning).is_empty(),
        "no fuel passes in it"
    );
    assert!(
        !fuelled_in(&log, 1, Pulse::Afternoon).is_empty(),
        "in the afternoon fuel passes"
    );
    // And the same a pulse earlier, in the main file: F reaches O's hex in
    // the night at cruise, and no fuel passes in it.
    let log = events(&run(&self::text()));
    let night: Vec<_> = steamed(&log, 1)
        .into_iter()
        .filter(|s| s.1 == Pulse::Night)
        .collect();
    assert_eq!((night[0].3, night[0].5), (t(15), Hex::new(0, 0)));
    assert!(fuelled_in(&log, 1, Pulse::Night).is_empty());
}

#[test]
fn statement_10_following() {
    let text = f_in_o_hex(&text());
    let text = changed(
        &text,
        "fuel_t = 2000\ntask_force = 1",
        "fuel_t = 4000\ntask_force = 1",
    );
    let text = changed(&text, "fuel_t = 100", "fuel_t = 499");
    let text = changed(
        &text,
        "fuel_t = 300\ntask_force = 1",
        "fuel_t = 500\ntask_force = 1",
    );
    let text = text
        + &order(
            1,
            "allied",
            "task_force = 2",
            "kind = \"set_route\"\nwaypoints = [[12, 0]]",
        );
    let scenario = run(&text);
    let log = events(&scenario);
    let f: Vec<_> = steamed(&log, 1)
        .into_iter()
        .filter(|s| s.1 == Pulse::Night)
        .collect();
    let o: Vec<_> = steamed(&log, 2)
        .into_iter()
        .filter(|s| s.1 == Pulse::Night && s.0 == 1)
        .collect();
    assert_eq!(f[0].5, o[0].5, "one hex at the end of the pulse");
    assert_eq!(f[0].2, o[0].2, "F takes O's route");
    // What the night's 96 nm burned is taken first, so each ship is topped
    // up: D1 her 1 t and her burn.
    let d1 = fuelled(&log)
        .into_iter()
        .find(|f| f.ship == 2)
        .expect("D1 fuelled");
    assert_eq!(d1.tons, t(1) + burned(&log, 2, 1, Pulse::Night));
    assert!(fuelled(&log)
        .iter()
        .all(|f| f.day == 1 && f.pulse == Pulse::Night));
    assert_eq!(ended(&log)[0].2, FuelOrderEnd::Full);
    assert_eq!(ended(&log)[0].1, Some(Pulse::Night));
}

// ------------------------------------------------- Statements: broken off

/// A Japanese carrier force four hexes east of O, searching west and
/// striking ships: its raid may attack F or O in the morning.
fn with_a_raid(text: &str, seed: u64) -> String {
    let text = changed(text, "seed = 1942", &format!("seed = {seed}"));
    let text = changed(
        &text,
        "surface_combat = \"../../reference/surface_combat.toml\"",
        "surface_combat = \"../../reference/surface_combat.toml\"\nair_combat = \"../../reference/air_combat.toml\"",
    );
    text + &task_force(9, "japanese", 4, 0, "carrier")
        + &a_ship(9, 4, "japanese", 3000, 9, None)
        + "\n[[air_group]]\nid = 9\nname = \"Placeholder scouts 9\"\nside = \"japanese\"\nship = 9\naircraft_type = 2\nready = 8\n"
        + "\n[[air_group]]\nid = 10\nname = \"Placeholder bombers 10\"\nside = \"japanese\"\nship = 9\naircraft_type = 3\nready = 18\n"
        + &order(1, "japanese", "air_group = 9", "kind = \"set_air_mission\"\nmission = \"search\"")
        + &order(1, "japanese", "air_group = 9", "kind = \"set_search_arc\"\nfrom_deg = 200\nto_deg = 340\nrange_nm = 300")
        + &order(1, "japanese", "air_group = 10", "kind = \"set_air_mission\"\nmission = \"naval_strike\"")
}

fn attacks_in(log: &[Event], day: u32, pulse: Pulse) -> Vec<(EventId, u32)> {
    log.iter()
        .filter(|e| e.day == day && e.pulse == Some(pulse))
        .filter_map(|e| match &e.kind {
            EventKind::AirAttack { task_force, .. } => Some((e.id, task_force.raw())),
            _ => None,
        })
        .collect()
}

#[test]
fn statement_11_a_raid() {
    let mut on_f = 0;
    let mut on_o = 0;
    for seed in 1..=30 {
        let scenario = run(&with_a_raid(&text(), seed));
        let log = events(&scenario);
        let attacks = attacks_in(&log, 1, Pulse::Morning);
        if attacks.is_empty() {
            continue;
        }
        let pair_afloat = !scenario.world.umpire().task_forces[&force(1)]
            .ships
            .is_empty()
            && !scenario.world.umpire().task_forces[&force(2)]
                .ships
                .is_empty();
        if attacks.iter().any(|(_, tf)| *tf == 1) {
            on_f += 1;
        }
        if attacks.iter().all(|(_, tf)| *tf == 2) {
            on_o += 1;
        }
        assert!(
            fuelled_in(&log, 1, Pulse::Morning).is_empty(),
            "seed {seed}: no fuel passes"
        );
        // Both forces made no more than 48 nm in the morning.
        for id in [1, 2] {
            for leg in steamed(&log, id)
                .into_iter()
                .filter(|s| s.0 == 1 && s.1 == Pulse::Morning)
            {
                assert!(leg.2 <= t(48), "seed {seed}: {id} made {}", leg.2);
            }
        }
        if pair_afloat {
            let off: Vec<&Event> = broken_off(&log)
                .into_iter()
                .filter(|e| e.day == 1 && e.pulse == Some(Pulse::Morning))
                .collect();
            assert_eq!(off.len(), 1, "seed {seed}: broken off");
            for (attack, _) in &attacks {
                assert!(
                    off[0].causes.contains(&Cause::Event(*attack)),
                    "seed {seed}: cites the raid's attack"
                );
            }
            // With no attack in the afternoon, statement 1's fuelling there.
            if attacks_in(&log, 1, Pulse::Afternoon).is_empty() {
                assert!(
                    !fuelled_in(&log, 1, Pulse::Afternoon).is_empty(),
                    "seed {seed}"
                );
            }
        }
        if attacks.iter().all(|(_, tf)| *tf == 1) && pair_afloat {
            assert_eq!(
                cargo_fuel(&scenario, 4)
                    + fuelled(&log).iter().fold(Fixed::ZERO, |s, f| s + f.tons),
                t(3000)
            );
        }
    }
    assert!(on_f > 0, "a raid attacked F on some seed");
    let _ = on_o;
}

/// Two Japanese cruisers routed into (0,0), arriving in the morning.
fn with_a_surface_action(text: &str, seed: u64) -> String {
    let text = changed(text, "seed = 1942", &format!("seed = {seed}"));
    text + &task_force(9, "japanese", -6, 0, "surface_combat")
        + &a_ship(9, 154, "japanese", 2000, 9, None)
        + &a_ship(10, 154, "japanese", 2000, 9, None)
        + &order(
            1,
            "japanese",
            "task_force = 9",
            "kind = \"set_route\"\nwaypoints = [[0, 0]]",
        )
}

#[test]
fn statements_12_and_14_a_surface_action_and_an_oiler_sunk() {
    let mut broken = 0;
    let mut sunk = 0;
    for seed in 1..=20 {
        let scenario = run(&with_a_surface_action(&text(), seed));
        let log = events(&scenario);
        let actions: Vec<EventId> = log
            .iter()
            .filter(|e| e.day == 1 && e.pulse == Some(Pulse::Morning))
            .filter_map(|e| match e.kind {
                EventKind::EngagementBegun { a, b, .. }
                    if [a, b].contains(&force(2)) || [a, b].contains(&force(1)) =>
                {
                    Some(e.id)
                }
                _ => None,
            })
            .collect();
        if actions.is_empty() {
            continue;
        }
        assert!(
            fuelled_in(&log, 1, Pulse::Morning).is_empty(),
            "seed {seed}: no fuel passes"
        );
        let off: Vec<&Event> = broken_off(&log)
            .into_iter()
            .filter(|e| e.day == 1 && e.pulse == Some(Pulse::Morning))
            .collect();
        if let Some(off) = off.first() {
            broken += 1;
            assert!(
                actions
                    .iter()
                    .any(|a| off.causes.contains(&Cause::Event(*a))),
                "seed {seed}: cites the action"
            );
        }
        // Statement 14: the oiler sunk ends the order.
        let oiler_sunk = log
            .iter()
            .find(|e| matches!(e.kind, EventKind::ShipSunk { ship: s, .. } if s == ship(4)));
        if let Some(sinking) = oiler_sunk {
            if scenario.world.umpire().task_forces[&force(1)]
                .ships
                .is_empty()
            {
                continue;
            }
            sunk += 1;
            let ends = ended(&log);
            assert_eq!(ends.len(), 1, "seed {seed}");
            assert_eq!(ends[0].2, FuelOrderEnd::NoOiler);
            assert!(
                ends[0].3.contains(&Cause::Event(sinking.id)),
                "seed {seed}: cites the sinking"
            );
            let f = &scenario.world.umpire().task_forces[&force(1)];
            assert_eq!(f.speed, SpeedSetting::Cruise);
            assert!(f.fuel_from.is_none());
            let after = fuelled(&log)
                .into_iter()
                .filter(|f| f.event > sinking.id)
                .count();
            assert_eq!(after, 0, "seed {seed}: no fuel passes later");
        }
    }
    assert!(broken > 0, "an action broke the fuelling off on some seed");
    assert!(sunk > 0, "the oiler was sunk on some seed");
}

// ---------------------------------------------------- Statements: the order

/// The main file with more forces and orders added, run for its two days.
fn refusal(added: &str) -> Vec<RejectReason> {
    rejections(&events(&run(&(text() + added))))
}

#[test]
fn statement_13_refused() {
    let dd = |id: u32, tf: u32| a_ship(id, 152, "allied", 500, tf, None);
    let oiler = |id: u32, tf: u32| a_ship(id, 153, "allied", 2000, tf, Some(1000));
    // The giver has no ship with the mark.
    let r = refusal(
        &(task_force(3, "allied", 5, 0, "surface_combat")
            + &dd(5, 3)
            + &order(
                1,
                "allied",
                "task_force = 1",
                "kind = \"fuel_from\"\ngiver = 3",
            )),
    );
    assert_eq!(r, vec![RejectReason::GiverHasNoOiler(force(3))]);
    // The giver is the force itself.
    let r = refusal(&order(
        1,
        "allied",
        "task_force = 1",
        "kind = \"fuel_from\"\ngiver = 1",
    ));
    assert_eq!(r, vec![RejectReason::CannotFuelFromItself]);
    // The giver is under the order herself.
    let r = refusal(
        &(task_force(5, "allied", 12, 0, "transport")
            + &oiler(5, 5)
            + &a_ship(8, 152, "allied", 100, 5, None)
            + &order(
                1,
                "allied",
                "task_force = 5",
                "kind = \"fuel_from\"\ngiver = 2",
            )
            + &task_force(6, "allied", 6, 0, "surface_combat")
            + &dd(6, 6)
            + &order(
                2,
                "allied",
                "task_force = 6",
                "kind = \"fuel_from\"\ngiver = 5",
            )),
    );
    assert_eq!(r, vec![RejectReason::GiverIsFuelling(force(5))]);
    // The giver is an escort.
    let r = refusal(
        &(task_force(5, "allied", 5, 0, "escort")
            + &oiler(5, 5)
            + &task_force(6, "allied", 5, 0, "transport")
            + &dd(6, 6)
            + &order(
                1,
                "allied",
                "task_force = 5",
                "kind = \"escort\"\ncharge = 6",
            )
            + &task_force(7, "allied", 6, 0, "surface_combat")
            + &dd(7, 7)
            + &order(
                2,
                "allied",
                "task_force = 7",
                "kind = \"fuel_from\"\ngiver = 5",
            )),
    );
    assert_eq!(r, vec![RejectReason::GiverIsAnEscort(force(5))]);
    // The giver has an escort.
    let r = refusal(
        &(task_force(5, "allied", 5, 0, "transport")
            + &oiler(5, 5)
            + &task_force(6, "allied", 5, 0, "escort")
            + &dd(6, 6)
            + &order(
                1,
                "allied",
                "task_force = 6",
                "kind = \"escort\"\ncharge = 5",
            )
            + &task_force(7, "allied", 6, 0, "surface_combat")
            + &dd(7, 7)
            + &order(
                2,
                "allied",
                "task_force = 7",
                "kind = \"fuel_from\"\ngiver = 5",
            )),
    );
    assert_eq!(
        r,
        vec![RejectReason::GiverHasAnEscort {
            giver: force(5),
            escort: force(6)
        }]
    );
    // The force is the giver for another: F fuels from O, and O is ordered
    // to fuel from a second oiler's force.
    let r = refusal(
        &(task_force(5, "allied", 5, 0, "transport")
            + &oiler(5, 5)
            + &order(
                1,
                "allied",
                "task_force = 2",
                "kind = \"fuel_from\"\ngiver = 5",
            )),
    );
    assert_eq!(r, vec![RejectReason::IsAGiver { taker: force(1) }]);
    // The force is an escort.
    let r = refusal(
        &(task_force(6, "allied", 5, 0, "escort")
            + &dd(6, 6)
            + &task_force(7, "allied", 5, 0, "transport")
            + &dd(7, 7)
            + &order(
                1,
                "allied",
                "task_force = 6",
                "kind = \"escort\"\ncharge = 7",
            )
            + &order(
                2,
                "allied",
                "task_force = 6",
                "kind = \"fuel_from\"\ngiver = 2",
            )),
    );
    assert_eq!(r, vec![RejectReason::EscortCannotFuel { charge: force(7) }]);
    // The force has an escort.
    let r = refusal(
        &(task_force(6, "allied", 5, 0, "escort")
            + &dd(6, 6)
            + &task_force(7, "allied", 5, 0, "surface_combat")
            + &dd(7, 7)
            + &order(
                1,
                "allied",
                "task_force = 6",
                "kind = \"escort\"\ncharge = 7",
            )
            + &order(
                2,
                "allied",
                "task_force = 7",
                "kind = \"fuel_from\"\ngiver = 2",
            )),
    );
    assert_eq!(r, vec![RejectReason::HasAnEscort { escort: force(6) }]);
}

#[test]
fn statement_14_it_ends_when_the_oiler_is_detached_or_a_route_is_given() {
    // The oiler detached from O, which keeps a destroyer.
    let text = self::text()
        + &a_ship(5, 152, "allied", 500, 2, None)
        + &order(
            1,
            "allied",
            "task_force = 2",
            "kind = \"detach\"\nships = [4]",
        );
    let scenario = run(&text);
    let log = events(&scenario);
    let ends = ended(&log);
    assert_eq!(ends.len(), 1);
    assert_eq!(ends[0].2, FuelOrderEnd::NoOiler);
    let detached = log
        .iter()
        .find(|e| matches!(e.kind, EventKind::ShipsDetached { .. }))
        .expect("the detach");
    assert!(
        ends[0].3.contains(&Cause::Event(detached.id)),
        "cites the detach"
    );
    assert!(fuelled(&log).is_empty(), "no fuel passes");
    let f = &scenario.world.umpire().task_forces[&force(1)];
    assert_eq!(
        (f.speed, f.route.waypoints.is_empty()),
        (SpeedSetting::Cruise, true)
    );
    // The commander gives F a route.
    let text = self::text()
        + &order(
            1,
            "allied",
            "task_force = 1",
            "kind = \"set_route\"\nwaypoints = [[4, 0]]",
        );
    let scenario = run(&text);
    let log = events(&scenario);
    let ends = ended(&log);
    assert_eq!(ends.len(), 1);
    assert_eq!(ends[0].2, FuelOrderEnd::NewRoute);
    assert!(fuelled(&log).is_empty(), "no fuel passes");
    let f = &scenario.world.umpire().task_forces[&force(1)];
    assert_eq!(f.speed, SpeedSetting::Cruise);
    assert_eq!(
        f.route.waypoints,
        vec![Hex::new(4, 0)],
        "the route the commander gave"
    );
    assert!(f.fuel_from.is_none());
}

/// F run dry on day 1: D1 holds nothing and F is routed east, so it is out
/// of fuel at (0,0) at the end of the night. Its order to fuel from O is
/// given on day 2.
fn run_dry(text: &str) -> String {
    let text = f_in_o_hex(text);
    let text = changed(&text, "fuel_t = 100", "fuel_t = 0");
    let text = changed(
        &text,
        "kind = \"fuel_from\"\ngiver = 2",
        "kind = \"set_route\"\nwaypoints = [[3, 0]]",
    );
    let text = changed(&text, "days = 2", "days = 3");
    text + &order(
        2,
        "allied",
        "task_force = 1",
        "kind = \"fuel_from\"\ngiver = 2",
    )
}

#[test]
fn statement_15_a_force_that_has_run_dry() {
    // O steams through F's hex on a route: no fuel, O not slowed.
    let text = run_dry(&self::text())
        + &order(
            2,
            "allied",
            "task_force = 2",
            "kind = \"set_route\"\nwaypoints = [[-8, 0]]",
        );
    let scenario = run(&text);
    let log = events(&scenario);
    assert!(log
        .iter()
        .any(|e| e.day == 1 && matches!(e.kind, EventKind::TaskForceOutOfFuel { .. })));
    assert!(fuelled(&log).is_empty(), "no fuel passes");
    let o: Vec<_> = steamed(&log, 2)
        .into_iter()
        .filter(|s| s.0 == 2 && s.1 == Pulse::Night)
        .collect();
    assert_eq!(o[0].3, t(15), "O is not slowed");
    // O lies stopped in F's hex: D1 first, and F is set idle.
    let scenario = run(&run_dry(&self::text()));
    let log = events(&scenario);
    let all = fuelled(&log);
    assert_eq!(all[0].ship, 2, "D1 first");
    assert!(all[0].restored, "the first ship fuelled sets F idle");
    assert!(all[1..].iter().all(|f| !f.restored));
    let f = &scenario.world.umpire().task_forces[&force(1)];
    assert_ne!(f.status, TaskForceStatus::OutOfFuel);
    assert!(f.route.waypoints.is_empty(), "its route dropped");
}

/// F and O lie stopped in (0,3), where A has a base.
fn at_a_base(text: &str, base: &str) -> String {
    let text = f_in_o_hex(text);
    let text = changed(
        &text,
        "q = 0\nr = 0\nmission = \"surface_combat\"",
        "q = 0\nr = 3\nmission = \"surface_combat\"",
    );
    let text = changed(
        &text,
        "q = 0\nr = 0\nmission = \"transport\"",
        "q = 0\nr = 3\nmission = \"transport\"",
    );
    text + &format!("\n[[base]]\nid = 1\nname = \"Placeholder base 1\"\nside = \"allied\"\nq = 0\nr = 3\n{base}\n")
}

#[test]
fn statement_16_a_port_before_an_oiler() {
    let scenario = run(&at_a_base(&text(), "fuel_t = 5000"));
    let log = events(&scenario);
    let port: Vec<&Event> = log
        .iter()
        .filter(|e| e.day == 1 && e.pulse == Some(Pulse::Night))
        .filter(|e| matches!(e.kind, EventKind::ShipRefuelled { .. }))
        .collect();
    assert_eq!(port.len(), 3, "the ships fuel from the base");
    assert!(fuelled(&log).is_empty(), "nothing from the oiler");
    assert_eq!(cargo_fuel(&scenario, 4), t(3000));
    // The oiler loads cargo fuel at the base in the night: she gives none
    // then, and gives in the morning, when the base's 100 t are aboard her
    // and she has no cargo work left. The base's fuel is at its reserve, so
    // F takes none from it; the commander's load to the oiler is not held
    // by the reserve.
    let text = at_a_base(&text(), "fuel_t = 100\nfuel_reserve_t = 100");
    let text = changed(&text, "cargo_fuel_t = 3000", "cargo_fuel_t = 2000");
    let text = text
        + &order(
            1,
            "allied",
            "task_force = 2",
            "kind = \"load\"\ncargo = \"fuel\"",
        );
    let log = events(&run(&text));
    assert!(log.iter().any(|e| e.day == 1
        && e.pulse == Some(Pulse::Night)
        && matches!(e.kind, EventKind::CargoLoaded { ship: s, .. } if s == ship(4))));
    assert!(
        fuelled_in(&log, 1, Pulse::Night).is_empty(),
        "she gives none in the night"
    );
    assert!(
        !fuelled_in(&log, 1, Pulse::Morning).is_empty(),
        "she gives in the morning"
    );
}

#[test]
fn statement_17_who_knows() {
    let fuelling = |kind: &EventKind| {
        matches!(
            kind,
            EventKind::ShipFuelledAtSea { .. }
                | EventKind::FuellingBrokenOff { .. }
                | EventKind::FuelOrderEnded { .. }
        )
    };
    let mut scenarios = vec![run(&text()), run(&run_dry(&text()))];
    scenarios.extend((1..=10).map(|seed| run(&with_a_surface_action(&text(), seed))));
    for scenario in &scenarios {
        let truth: Vec<EventId> = events(scenario)
            .iter()
            .filter(|e| fuelling(&e.kind))
            .map(|e| e.id)
            .collect();
        let allied: Vec<EventId> = scenario
            .world
            .side_view(Side::Allied)
            .events()
            .iter()
            .filter(|e| matches!(&e.kind, SideEventKind::Whole(kind) if fuelling(kind)))
            .map(|e| e.id)
            .collect();
        assert_eq!(allied, truth, "A sees every one, whole");
        let japanese = scenario.world.side_view(Side::Japanese).events();
        assert!(
            japanese
                .iter()
                .all(|e| !matches!(&e.kind, SideEventKind::Whole(kind) if fuelling(kind))),
            "B sees none"
        );
        assert!(
            japanese.iter().all(|e| !truth.contains(&e.id)),
            "nor any of them cut"
        );
    }
}

// ------------------------------------------------------ Statements: the staff

fn reasons(log: &[Event], day: u32) -> Vec<String> {
    log.iter()
        .filter(|e| e.day == day)
        .filter_map(|e| match &e.kind {
            EventKind::StaffReason { reason, .. } => Some(reason.worked.clone()),
            _ => None,
        })
        .collect()
}

fn staff_orders(log: &[Event], world: &sim_core::world::World) -> Vec<(u32, TaskForceOrder)> {
    log.iter()
        .filter_map(|e| match &e.kind {
            EventKind::StaffReason {
                order: Some(id), ..
            } => world.order(*id).and_then(|o| match &o.kind {
                OrderKind::TaskForce { order, .. } => Some((e.day, order.clone())),
                _ => None,
            }),
            _ => None,
        })
        .collect()
}

/// A carrier force (task force 3) with no home port at (-6,0): a carrier
/// with fighters aboard and a destroyer holding `dd` t of 500, under a
/// carrier task at its own hex. O is removed when `with_oiler` is false.
fn carrier_force(dd: i64, with_oiler: bool) -> String {
    let text = changed(
        &text(),
        "surface_combat = \"../../reference/surface_combat.toml\"",
        "surface_combat = \"../../reference/surface_combat.toml\"\nair_combat = \"../../reference/air_combat.toml\"",
    );
    let text = changed(
        &text,
        "kind = \"fuel_from\"\ngiver = 2",
        "kind = \"set_speed\"\nspeed = \"cruise\"",
    );
    let text = if with_oiler {
        text
    } else {
        changed(&text, "cargo_fuel_t = 3000", "cargo_fuel_t = 0")
    };
    text + &task_force(3, "allied", -6, 0, "carrier")
        + &a_ship(5, 151, "allied", 4000, 3, None)
        + &a_ship(6, 152, "allied", dd, 3, None)
        + "\n[[air_group]]\nid = 1\nname = \"Placeholder fighters 1\"\nside = \"allied\"\nship = 5\naircraft_type = 4\nready = 12\n"
        + "\n[[command]]\nday = 1\nside = \"allied\"\nkind = \"set_intent\"\nintent = 1\nintent_kind = \"deny\"\nhex = [-6, 0]\ndistance = 0\n"
        + "\n[[command]]\nday = 1\nside = \"allied\"\nkind = \"set_limit\"\nlimit = \"forces\"\nintent = 1\ntask_forces = [3]\n"
        + "\n[[command]]\nday = 1\nside = \"allied\"\nkind = \"set_leave\"\nleave = \"prepare\"\ngiven = true\n"
        + "\n[[command]]\nday = 1\nside = \"allied\"\nkind = \"set_leave\"\nleave = \"launch\"\ngiven = true\n"
}

#[test]
fn statement_18_the_warning() {
    let scenario = run_staffed(&carrier_force(120, false));
    let log = events(&scenario);
    for day in [1, 2] {
        assert!(
            reasons(&log, day).iter().any(|r| r.contains("task_force:3")
                && r.contains("ship:6")
                && r.contains("below a quarter")),
            "day {day}: the report names the force and the ship"
        );
    }
    assert!(
        staff_orders(&log, &scenario.world)
            .iter()
            .all(|(_, o)| !matches!(o, TaskForceOrder::FuelFrom(_))),
        "with no oiler holding fuel the staff gives no order because of it"
    );
    let scenario = run_staffed(&carrier_force(130, false));
    let log = events(&scenario);
    assert!(
        (1..=2).all(|day| reasons(&log, day)
            .iter()
            .all(|r| !(r.contains("ship:6") && r.contains("below a quarter")))),
        "at 130 t it does not"
    );
}

#[test]
fn statement_21_no_home_port() {
    let scenario = run_staffed(&carrier_force(120, true));
    let log = events(&scenario);
    let orders = staff_orders(&log, &scenario.world);
    assert!(
        orders
            .iter()
            .any(|(day, o)| *day == 1 && *o == TaskForceOrder::FuelFrom(force(2))),
        "the staff gives it the order to fuel from O: {orders:?}"
    );
}

/// A force on a stand (task force 3, two destroyers) stopped at its station
/// (0,0) on no route, with its home port, base 1, ten hexes east at (10,0).
/// One destroyer holds 40 t: short of the passage home at cruise, 40 t, by
/// the stand doctrine's margin. O lies at `oiler_q` holding `cargo` t.
fn stand_force(oiler_q: i32, cargo: i64) -> String {
    let text = changed(
        &text(),
        "surface_combat = \"../../reference/surface_combat.toml\"",
        "surface_combat = \"../../reference/surface_combat.toml\"\nair_combat = \"../../reference/air_combat.toml\"",
    );
    let text = changed(
        &text,
        "kind = \"fuel_from\"\ngiver = 2",
        "kind = \"set_speed\"\nspeed = \"cruise\"",
    );
    let text = changed(
        &text,
        "cargo_fuel_t = 3000",
        &format!("cargo_fuel_t = {cargo}"),
    );
    let text = changed(
        &text,
        "q = 0\nr = 0\nmission = \"transport\"",
        &format!("q = {oiler_q}\nr = 0\nmission = \"transport\""),
    );
    let text = changed(&text, "days = 2", "days = 4");
    text + "\n[[base]]\nid = 1\nname = \"Placeholder home port\"\nside = \"allied\"\nq = 10\nr = 0\nfuel_t = 5000\n"
        + "\n[[task_force]]\nid = 3\nname = \"Placeholder stand force 3\"\nside = \"allied\"\nq = 0\nr = 0\nmission = \"surface_combat\"\nhome_port = 1\n"
        + &a_ship(5, 152, "allied", 40, 3, None)
        + &a_ship(6, 152, "allied", 500, 3, None)
        + "\n[[command]]\nday = 1\nside = \"allied\"\nkind = \"set_intent\"\nintent = 1\nintent_kind = \"deny\"\nhex = [0, 0]\ndistance = 0\n"
        + "\n[[command]]\nday = 1\nside = \"allied\"\nkind = \"set_limit\"\nlimit = \"forces\"\nintent = 1\ntask_forces = [3]\n"
        + "\n[[command]]\nday = 1\nside = \"allied\"\nkind = \"set_leave\"\nleave = \"prepare\"\ngiven = true\n"
        + "\n[[command]]\nday = 1\nside = \"allied\"\nkind = \"set_leave\"\nleave = \"launch\"\ngiven = true\n"
}

fn routed_home(orders: &[(u32, TaskForceOrder)], day: u32) -> bool {
    orders.iter().any(|(d, o)| {
        *d == day
            && matches!(o, TaskForceOrder::SetRoute { waypoints, .. } if waypoints.last() == Some(&Hex::new(10, 0)))
    })
}

#[test]
fn statement_19_sent_to_the_oiler() {
    let scenario = run_staffed(&stand_force(-4, 3000));
    let log = events(&scenario);
    let orders = staff_orders(&log, &scenario.world);
    assert!(
        orders
            .iter()
            .any(|(d, o)| *d == 1 && *o == TaskForceOrder::FuelFrom(force(2))),
        "the order to fuel from O: {orders:?}"
    );
    let given = log
        .iter()
        .find_map(|e| match &e.kind {
            EventKind::StaffReason {
                order: Some(id),
                reason,
                ..
            } if scenario.world.order(*id).is_some_and(|o| {
                matches!(
                    o.kind,
                    OrderKind::TaskForce {
                        order: TaskForceOrder::FuelFrom(_),
                        ..
                    }
                )
            }) =>
            {
                Some(reason.worked.clone())
            }
            _ => None,
        })
        .expect("its reason");
    assert!(
        given.contains("160 nm") && given.contains("400 nm"),
        "both passages: {given}"
    );
    // While the order stands the staff gives it no route.
    let ends = ended(&log);
    let end_day = ends.first().map_or(u32::MAX, |e| e.0);
    for (day, o) in &orders {
        if *day > 1 && *day <= end_day {
            assert!(
                !matches!(o, TaskForceOrder::SetRoute { .. }),
                "day {day}: no route while it stands"
            );
        }
    }
}

#[test]
fn statement_19_on_the_morning_after_it_is_routed_back_to_its_station() {
    let scenario = run_staffed(&stand_force(-4, 3000));
    let log = events(&scenario);
    let end_day = ended(&log).first().expect("the order ends").0;
    let orders = staff_orders(&log, &scenario.world);
    assert!(orders.iter().any(|(d, o)| *d == end_day + 1
        && matches!(o, TaskForceOrder::SetRoute { waypoints, .. } if waypoints.first() == Some(&Hex::new(0, 0)))));
}

#[test]
fn statement_19_a_force_still_short_when_the_giver_runs_empty_goes_home() {
    // O holds 20 t: the destroyer takes it and is still short of the
    // passage to the station and home; the order ends for a giver that holds
    // no cargo fuel, and on the morning after it the force is routed home,
    // as today (decision 0192, 16).
    let scenario = run_staffed(&stand_force(-4, 20));
    let log = events(&scenario);
    let ends = ended(&log);
    let (end_day, _, reason, _) = ends.first().expect("the order ends");
    assert_eq!(*reason, FuelOrderEnd::GiverEmpty);
    let orders = staff_orders(&log, &scenario.world);
    assert!(routed_home(&orders, end_day + 1), "routed home: {orders:?}");
    assert!(!orders.iter().any(|(d, o)| *d == end_day + 1
        && matches!(o, TaskForceOrder::SetRoute { waypoints, .. } if waypoints.first() == Some(&Hex::new(0, 0)))));
}

#[test]
fn statement_20_home_is_nearer_or_the_oiler_is_empty() {
    for (q, cargo) in [(-12, 3000), (-4, 0)] {
        let scenario = run_staffed(&stand_force(q, cargo));
        let log = events(&scenario);
        let orders = staff_orders(&log, &scenario.world);
        assert!(
            routed_home(&orders, 1),
            "O at {q} with {cargo} t: routed home: {orders:?}"
        );
        assert!(orders
            .iter()
            .all(|(_, o)| !matches!(o, TaskForceOrder::FuelFrom(_))));
    }
}

// ---------------------------------------------------------------- Properties

#[test]
fn nothing_is_drawn() {
    // With no enemy and no search, two seeds give the same game.
    let one = run(&text());
    let two = run(&changed(&text(), "seed = 1942", "seed = 7"));
    assert_eq!(events(&one), events(&two));
}

#[test]
fn a_class_file_with_no_column_for_the_mark_loads_and_gives_no_mark() {
    let live = sim_data::load_ship_classes(&common::data_dir().join("reference/ship_classes.csv"))
        .expect("the live classes load");
    assert!(live.ship_classes.values().all(|c| !c.fuels_at_sea));
    let frozen = sim_data::load_ship_classes(&common::frozen_root().join("ship_classes.csv"))
        .expect("the frozen classes load");
    assert!(frozen.ship_classes.values().all(|c| !c.fuels_at_sea));
    let ours = sim_data::load_ship_classes(
        &common::data_dir().join("reference/ship_classes_placeholder_scenario_115.csv"),
    )
    .expect("the scenario's classes load");
    let marked: Vec<u32> = ours
        .ship_classes
        .values()
        .filter(|c| c.fuels_at_sea)
        .map(|c| c.id.raw())
        .collect();
    assert_eq!(marked, vec![153]);
}
