//! Scenario 113: men who replace the lost. A base with a stream of
//! replacement men brings the units ashore there back toward their
//! establishment, and their experience down toward a recruit's.
//!
//! Proves decision 0197. The arithmetic is the architect's, written
//! before the code. A mismatch goes back to the architect, and nobody else
//! changes a statement or a number here.
//!
//! # The rule of experience
//!
//! Experience is a whole level, 0 to 4; 2 is the standard and the level
//! of replacements. A unit above 2 loses one level on the night the men
//! who have joined it since its level last changed reach half its
//! establishment, and the count begins again.
//!
//! # The setting
//!
//! A placeholder scenario of eight days made for the test. Side J has a
//! base R with ample supply and a stream of 100 men a day from day 2. At
//! R, ashore, on defend, with no enemy in the hex: unit 1, 400 men, an
//! establishment of 600 by the scenario's key, experience 4; unit 2, 500
//! men with no key, so an establishment of 500, experience 2; unit 3,
//! 250 men, an establishment of 300, experience 3. A day's night comes
//! first; the fight ashore is at the day's end.
//!
//! # Statements
//!
//! 1. **Nothing on day 1.** At the end of day 1 the units have 400, 500
//!    and 250 men.
//! 2. **The lowest number first, up to its establishment.** Night of day
//!    2: unit 1 takes 100 and has 500. Night of day 3: unit 1 takes 100
//!    and has 600. Night of day 4: unit 1 is full and unit 2 is full;
//!    unit 3 takes 50 and has 300, and the other 50 are lost. Night of
//!    day 5: no unit is below its establishment; nobody joins and no
//!    event is recorded.
//! 3. **Fewer than half: no change.** Unit 1 has had 200 join by the
//!    night of day 3, fewer than 300, half of 600: experience 4. Unit 3
//!    has had 50 join, fewer than 150: experience 3.
//! 3a. **Half its establishment: one level.** With unit 1 beginning at
//!    200 men: 100 join on each of the nights of days 2, 3 and 4. On the
//!    night of day 4 the count is 300, half of 600: its experience falls
//!    from 4 to 3 and the count begins again. On the night of day 5 it
//!    takes 100 and has 600, with a count of 100: experience 3.
//! 3b. **Never below the standard, and never raised.** A unit at
//!    experience 2 that takes 400 men against an establishment of 500
//!    stays at 2. A unit at 1 that takes the same stays at 1.
//! 4. **The event.** `MenJoined` for unit 1 on day 2 gives 100 men, 500
//!    after, experience 4 before and after, and the count, 100. On the
//!    night of statement 3a's fall it gives 4 before and 3 after. J sees
//!    it whole; the other side is given nothing.
//! 5. **Not to a unit in a fight.** With an enemy unit in R's hex and
//!    unit 1 on attack on the night of day 2: it takes none that night,
//!    and unit 3, the next below its establishment, takes 50 and has
//!    300; 50 are lost. With the enemy attacking on day 1 and unit 1 on
//!    defend: unit 1 fought on day 1 and takes none on the night of day
//!    2; with no fight on day 2 it takes 100 on the night of day 3.
//! 5a. **Bombs are not a fight.** Unit 1 bombed on day 1 and not fought:
//!    it takes 100 on the night of day 2.
//! 6. **Not to a unit elsewhere, aboard, on a march or with no men.**
//!    Unit 1 in the next hex, or wholly aboard a ship at R, or marching,
//!    or destroyed: it takes none.
//! 6a. **Men aboard count toward its establishment.** Unit 1 with 400
//!    ashore at R and 150 aboard a ship there: it is 50 short and takes
//!    50, not 200.
//! 7. **Not when the base is not its side's.** With R taken at the end of
//!    day 2: men join on the night of day 2 and none from the night of
//!    day 3.
//! 8. **A unit divided.** Unit 1 with 400 men ashore at R and 200 aboard
//!    a ship, an establishment of 600: the 200 land in another hex and
//!    become a new unit. The new unit's establishment is 200 and unit
//!    1's is 400.
//! 9. **They are fed.** On the night of day 2 unit 1 draws for 500 men:
//!    500 x 0.005 = 2.5 t.
//! 10. **Refused scenarios:** an establishment below a unit's men; a
//!     stream of no men; a stream to a base the scenario has not.
//! 11. **No stream, no change.** The 39 days' first week gives the events
//!     it gave before.
//!
//! # Properties
//!
//! - A unit's men never pass its establishment by this rule.
//! - A unit's experience is never raised by replacements, never lowered
//!   below 2 by them, and never lowered by more than one level in a
//!   night.
//! - The men who join in a night are never more than the stream gives.
//!
//! The tests are written by the build under these statements.

// The architect's doc comment above is kept as he wrote it; its numbered
// items with letters (3a, 5a, 6a) read to clippy as lazy list continuations.
#![allow(clippy::doc_lazy_continuation)]

// ---------------------------------------------------------------- The harness

use std::path::PathBuf;

use sim_core::events::{Event, EventKind, SupplyUse};
use sim_core::fixed::Fixed;
use sim_core::ids::{BaseId, LandUnitId};
use sim_core::side_events::SideEventKind;
use sim_core::state::{Side, TruthState};
use sim_data::Scenario;

mod common;

const A: Side = Side::Allied;
const J: Side = Side::Japanese;

/// R, base 2, J's.
const R: BaseId = BaseId::new(2);

const U1: LandUnitId = LandUnitId::new(1);
const U2: LandUnitId = LandUnitId::new(2);
const U3: LandUnitId = LandUnitId::new(3);

fn path() -> PathBuf {
    common::data_dir().join("scenarios/men-who-replace-the-lost/113_men_who_replace_the_lost.toml")
}

fn text() -> String {
    std::fs::read_to_string(path()).expect("the scenario file reads")
}

fn changed(text: &str, original: &str, replacement: &str) -> String {
    common::replace_once(text, original, replacement)
}

/// Adds `what` at the mark `# MARK: name`; the mark stays.
fn at(text: &str, name: &str, what: &str) -> String {
    let mark = format!("# MARK: {name}\n");
    changed(text, &mark, &format!("{what}{mark}"))
}

fn load(text: &str) -> Scenario {
    common::from_text(text, &path()).expect("the scenario loads")
}

fn load_error(text: &str) -> String {
    match common::from_text(text, &path()) {
        Ok(_) => panic!("the scenario is refused"),
        Err(error) => error.message,
    }
}

/// Unit 1's entry in the file, written out, for a variant that changes it.
const UNIT_1: &str = "[[land_unit]]\nid = 1\nname = \"Placeholder J unit 1\"\nside = \"japanese\"\nmen = 400\nestablishment = 600\nexperience = 4\nq = 14\nr = 0\n";

/// The text with unit 1's keys replaced by `keys` (its id, name and side are
/// kept).
fn unit_1_as(text: &str, keys: &str) -> String {
    changed(
        text,
        UNIT_1,
        &format!(
            "[[land_unit]]\nid = 1\nname = \"Placeholder J unit 1\"\nside = \"japanese\"\n{keys}\n"
        ),
    )
}

const UNIT_2: &str = "[[land_unit]]\nid = 2\nname = \"Placeholder J unit 2\"\nside = \"japanese\"\nmen = 500\nexperience = 2\nq = 14\nr = 0\n";
const UNIT_3: &str = "[[land_unit]]\nid = 3\nname = \"Placeholder J unit 3\"\nside = \"japanese\"\nmen = 250\nestablishment = 300\nexperience = 3\nq = 14\nr = 0\n";
const THE_STREAM: &str = "[[men_replacement]]\nbase = 2\nmen = 100\nfirst_day = 2\n";

/// A game played to its end: the events, the daily hashes, and the truth at
/// the start (index 0) and at the end of each day.
struct Played {
    events: Vec<Event>,
    hashes: Vec<u64>,
    truth: Vec<TruthState>,
    scenario: Scenario,
}

fn play(text: &str) -> Played {
    play_seeded(text, None)
}

fn play_seeded(text: &str, seed: Option<u64>) -> Played {
    let mut scenario = load(text);
    if let Some(seed) = seed {
        scenario = scenario.with_seed(seed).expect("a seed can be set");
    }
    let mut truth = vec![scenario.world.umpire().clone()];
    let mut hashes = Vec::new();
    while let Some(summary) = scenario.next_day().expect("a day resolves") {
        hashes.push(summary.state_hash);
        truth.push(scenario.world.umpire().clone());
    }
    Played {
        events: scenario.world.events().iter().cloned().collect(),
        hashes,
        truth,
        scenario,
    }
}

/// One `MenJoined`, as the tests read it.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct Joined {
    day: u32,
    unit: LandUnitId,
    side: Side,
    base: BaseId,
    men: u32,
    men_after: u32,
    before: u8,
    after: u8,
    joined: u32,
}

impl Played {
    fn joined(&self) -> Vec<Joined> {
        self.events
            .iter()
            .filter_map(|e| match &e.kind {
                EventKind::MenJoined {
                    unit,
                    side,
                    base,
                    men,
                    men_after,
                    experience_before,
                    experience_after,
                    joined,
                } => Some(Joined {
                    day: e.day,
                    unit: *unit,
                    side: *side,
                    base: *base,
                    men: *men,
                    men_after: *men_after,
                    before: *experience_before,
                    after: *experience_after,
                    joined: *joined,
                }),
                _ => None,
            })
            .collect()
    }

    /// The (unit, men) that joined on the night of `day`.
    fn joined_on(&self, day: u32) -> Vec<(LandUnitId, u32)> {
        self.joined()
            .into_iter()
            .filter(|j| j.day == day)
            .map(|j| (j.unit, j.men))
            .collect()
    }

    /// A unit's men, ashore and aboard, at the end of `day` (0 is the start).
    fn men(&self, day: usize, unit: LandUnitId) -> u32 {
        let truth = &self.truth[day];
        truth.land_units[&unit].men_ashore + truth.men_aboard(unit)
    }

    fn experience(&self, day: usize, unit: LandUnitId) -> u8 {
        self.truth[day].land_units[&unit].experience
    }

    fn establishment(&self, day: usize, unit: LandUnitId) -> u32 {
        self.truth[day].men_replacement.units[&unit].establishment
    }

    fn count(&self, day: usize, unit: LandUnitId) -> u32 {
        self.truth[day].men_replacement.units[&unit].joined
    }

    fn on(&self, day: u32) -> impl Iterator<Item = &Event> {
        self.events.iter().filter(move |e| e.day == day)
    }
}

/// An Allied unit 4 of `men` men ashore at R, with `keys` added.
fn allied_at_r(text: &str, men: u32, keys: &str) -> String {
    at(
        text,
        "units",
        &format!(
            "[[land_unit]]\nid = 4\nname = \"Placeholder Allied unit 4\"\nside = \"allied\"\nmen = {men}\nq = 14\nr = 0\n{keys}\n"
        ),
    )
}

fn order(text: &str, body: &str) -> String {
    at(text, "orders", &format!("[[order]]\n{body}\n"))
}

// ------------------------------------------------------------------ Statements

#[test]
fn the_setting_is_what_the_comment_says() {
    let s = load(&text());
    let truth = s.world.umpire();
    assert_eq!(s.days, 8);
    assert_eq!(truth.bases[&R].side, J);
    let streams = &truth.men_replacement.streams;
    assert_eq!(streams.len(), 1);
    assert_eq!(
        (
            streams[0].side,
            streams[0].base,
            streams[0].men_per_day,
            streams[0].first_day,
            streams[0].last_day
        ),
        (J, R, 100, 2, 8)
    );
    for (unit, men, establishment, experience) in
        [(U1, 400, 600, 4), (U2, 500, 500, 2), (U3, 250, 300, 3)]
    {
        let land_unit = &truth.land_units[&unit];
        assert_eq!(land_unit.men_ashore, men, "{unit}");
        assert_eq!(land_unit.hex, Some(truth.bases[&R].hex), "{unit}");
        assert_eq!(land_unit.side, J);
        assert_eq!(land_unit.experience, experience, "{unit}");
        let strength = truth.men_replacement.units[&unit];
        assert_eq!(strength.establishment, establishment, "{unit}");
        assert_eq!((strength.joined, strength.last_fought), (0, None));
    }
}

#[test]
fn statement_1_nothing_on_day_1() {
    let played = play(&text());
    assert!(played.joined_on(1).is_empty());
    assert_eq!(
        [played.men(1, U1), played.men(1, U2), played.men(1, U3)],
        [400, 500, 250]
    );
}

#[test]
fn statement_2_the_lowest_number_first_up_to_its_establishment() {
    let played = play(&text());
    assert_eq!(played.joined_on(2), vec![(U1, 100)]);
    assert_eq!(played.men(2, U1), 500);
    assert_eq!(played.joined_on(3), vec![(U1, 100)]);
    assert_eq!(played.men(3, U1), 600);
    // Night of day 4: units 1 and 2 are full; unit 3 takes 50, and the other
    // 50 are lost.
    assert_eq!(played.joined_on(4), vec![(U3, 50)]);
    assert_eq!(
        [played.men(4, U1), played.men(4, U2), played.men(4, U3)],
        [600, 500, 300]
    );
    // Night of day 5 and after: nobody joins, and no event is recorded.
    for day in 5..=8 {
        assert!(played.joined_on(day).is_empty(), "day {day}");
    }
    assert_eq!(played.joined().len(), 3);
}

#[test]
fn statement_3_fewer_than_half_no_change() {
    let played = play(&text());
    assert_eq!(played.count(3, U1), 200);
    assert_eq!(played.experience(3, U1), 4);
    assert_eq!(played.count(8, U3), 50);
    assert_eq!(played.experience(8, U3), 3);
    assert_eq!(played.experience(8, U1), 4);
}

/// Unit 1 beginning at 200 men (statement 3a).
fn unit_1_at_200() -> String {
    unit_1_as(
        &text(),
        "men = 200\nestablishment = 600\nexperience = 4\nq = 14\nr = 0",
    )
}

#[test]
fn statement_3a_half_its_establishment_one_level() {
    let played = play(&unit_1_at_200());
    for day in 2..=4 {
        assert_eq!(played.joined_on(day), vec![(U1, 100)], "day {day}");
    }
    assert_eq!(played.experience(3, U1), 4);
    assert_eq!(played.count(3, U1), 200);
    // Night of day 4: the count is 300, half of 600; experience 4 to 3 and
    // the count begins again.
    assert_eq!(played.experience(4, U1), 3);
    assert_eq!(played.count(4, U1), 0);
    assert_eq!(played.men(4, U1), 500);
    // Night of day 5: 100 more, 600 men, a count of 100, experience 3.
    assert_eq!(played.joined_on(5), vec![(U1, 100)]);
    assert_eq!(played.men(5, U1), 600);
    assert_eq!(played.count(5, U1), 100);
    assert_eq!(played.experience(5, U1), 3);
}

#[test]
fn statement_3b_never_below_the_standard_and_never_raised() {
    for level in [2u8, 1] {
        let played = play(&unit_1_as(
            &text(),
            &format!("men = 100\nestablishment = 500\nexperience = {level}\nq = 14\nr = 0"),
        ));
        let taken: u32 = played
            .joined()
            .iter()
            .filter(|j| j.unit == U1)
            .map(|j| j.men)
            .sum();
        assert_eq!(taken, 400, "experience {level}");
        assert_eq!(played.men(8, U1), 500);
        for day in 0..=8 {
            assert_eq!(played.experience(day, U1), level, "day {day}");
        }
    }
}

#[test]
fn statement_4_the_event() {
    let played = play(&text());
    let first = played.joined()[0];
    assert_eq!(
        first,
        Joined {
            day: 2,
            unit: U1,
            side: J,
            base: R,
            men: 100,
            men_after: 500,
            before: 4,
            after: 4,
            joined: 100,
        }
    );
    // It cites no cause, as a stream of aircraft cites none.
    let event = played
        .events
        .iter()
        .find(|e| matches!(e.kind, EventKind::MenJoined { .. }))
        .expect("a MenJoined");
    assert!(event.causes.is_empty());
    // On the night of statement 3a's fall: 4 before and 3 after.
    let fall = play(&unit_1_at_200());
    let night_4: Vec<Joined> = fall.joined().into_iter().filter(|j| j.day == 4).collect();
    assert_eq!(night_4.len(), 1);
    assert_eq!((night_4[0].before, night_4[0].after), (4, 3));
    // J sees each whole; A is given nothing of the kind.
    let s = &played.scenario;
    let whole = s
        .world
        .side_view(J)
        .events()
        .into_iter()
        .filter(|e| matches!(e.kind, SideEventKind::Whole(EventKind::MenJoined { .. })))
        .count();
    assert_eq!(whole, 3);
    let told_to_a = s
        .world
        .side_view(A)
        .events()
        .into_iter()
        .filter(|e| format!("{:?}", e.kind).contains("MenJoined"))
        .count();
    assert_eq!(told_to_a, 0);
}

#[test]
fn statement_5_not_to_a_unit_on_attack() {
    // An Allied unit in R's hex, on defend; unit 1 set to attack on day 2,
    // before that day's night.
    let t = allied_at_r(&text(), 200, "");
    let t = order(
        &t,
        "day = 2\nside = \"japanese\"\nland_unit = 1\nkind = \"set_posture\"\nposture = \"attack\"",
    );
    let played = play(&t);
    assert!(played.joined_on(1).is_empty());
    assert_eq!(played.joined_on(2), vec![(U3, 50)]);
    assert_eq!(played.men(2, U3), 300);
    // Unit 1 took none: it is at 400 less what the day's fight cost it.
    assert!(played.men(2, U1) <= 400);
}

#[test]
fn statement_5_not_to_a_unit_attacked_the_day_before() {
    // The enemy attacks on day 1, unit 1 on defend; on day 2 the enemy goes
    // back to defend and there is no fight.
    let t = allied_at_r(&text(), 200, "posture = \"attack\"");
    let t = order(
        &t,
        "day = 2\nside = \"allied\"\nland_unit = 4\nkind = \"set_posture\"\nposture = \"defend\"",
    );
    let played = play(&t);
    let fights: Vec<u32> = played
        .events
        .iter()
        .filter(|e| matches!(e.kind, EventKind::GroundFight { .. }))
        .map(|e| e.day)
        .collect();
    assert_eq!(fights, vec![1], "one fight, on day 1");
    assert_eq!(
        played.truth[1].men_replacement.units[&U1].last_fought,
        Some(1)
    );
    // Night of day 2: unit 1 fought on day 1 and takes none (nor do units 2
    // and 3, which fought beside it).
    assert!(played.joined_on(2).iter().all(|(unit, _)| *unit != U1));
    assert!(played.joined_on(2).is_empty());
    // Night of day 3: no fight on day 2, it takes 100.
    assert_eq!(played.joined_on(3).first(), Some(&(U1, 100)));
}

/// An Allied bombardment force at R from the start: a cruiser with shells.
fn bombarded(text: &str) -> String {
    at(
        text,
        "forces",
        "[[task_force]]\nid = 1\nname = \"Placeholder Allied bombardment force\"\nside = \"allied\"\nq = 14\nr = 0\nmission = \"bombardment\"\n\n[[ship]]\nid = 1\nname = \"Placeholder Allied cruiser\"\nclass = 1\nside = \"allied\"\nfuel_t = 2000\ngun_rounds = 800\ntask_force = 1\n\n",
    )
}

#[test]
fn statement_5a_bombs_are_not_a_fight() {
    // The first seed on which the shells of day 1 kill men of unit 1.
    let played = (1u64..=60)
        .map(|seed| play_seeded(&bombarded(&text()), Some(seed)))
        .find(|p| {
            p.on(1).any(|e| {
                matches!(e.kind, EventKind::MenLostToFire { unit, lost, .. } if unit == U1 && lost > 0)
            })
        })
        .expect("a seed on which unit 1 is bombed on day 1");
    assert!(played
        .on(1)
        .all(|e| !matches!(e.kind, EventKind::GroundFight { .. })));
    assert_eq!(played.truth[1].men_replacement.units[&U1].last_fought, None);
    assert_eq!(played.joined_on(2).first(), Some(&(U1, 100)));
}

#[test]
fn statement_6_not_to_a_unit_in_the_next_hex() {
    let played = play(&unit_1_as(
        &text(),
        "men = 400\nestablishment = 600\nexperience = 4\nq = 15\nr = 0",
    ));
    // It is in no base's hex: it takes none (and, unfed there, it loses men).
    assert!(played.joined().iter().all(|j| j.unit != U1));
    assert!(played.men(8, U1) <= 400);
}

#[test]
fn statement_6_not_to_a_unit_wholly_aboard() {
    let t = at(
        &text(),
        "forces",
        "[[task_force]]\nid = 2\nname = \"Placeholder J transport force\"\nside = \"japanese\"\nq = 14\nr = 0\nmission = \"transport\"\n\n[[ship]]\nid = 2\nname = \"Placeholder J troop transport\"\nclass = 10\nside = \"japanese\"\nfuel_t = 900\ntask_force = 2\n\n",
    );
    let played = play(&unit_1_as(
        &t,
        "men = 400\nestablishment = 600\nexperience = 4\ntask_force = 2",
    ));
    assert_eq!(played.truth[0].land_units[&U1].men_ashore, 0);
    assert_eq!(played.truth[0].men_aboard(U1), 400);
    assert!(played.joined().iter().all(|j| j.unit != U1));
    assert_eq!(played.men(8, U1), 400);
}

#[test]
fn statement_6_not_to_a_unit_on_a_march() {
    // A march to R2 ordered on day 2, before that day's night: three days a
    // hex, so it is marching on the nights of days 2 to 4.
    let t = order(
        &text(),
        "day = 2\nside = \"japanese\"\nland_unit = 1\nkind = \"march\"\nhex = [15, 0]",
    );
    let played = play(&t);
    assert!(played.truth[2].land_units[&U1].march.is_some());
    for day in 2..=4 {
        assert!(
            played.joined_on(day).iter().all(|(unit, _)| *unit != U1),
            "day {day}"
        );
    }
    // The stream passes to unit 3 on the night of day 2.
    assert_eq!(played.joined_on(2), vec![(U3, 50)]);
}

// Statement 6, a unit destroyed, and statement 6a, men aboard, are proved in
// sim-core (`replacement_men_tests.rs`), where a unit can be given no men
// and men both ashore and aboard at once, which no scenario file can say.

#[test]
fn statement_7_not_when_the_base_is_not_its_sides() {
    // A great Allied unit at R, set to attack on day 2: J is beaten at the
    // end of day 2 and R falls.
    let t = allied_at_r(&text(), 20000, "");
    let t = order(
        &t,
        "day = 2\nside = \"allied\"\nland_unit = 4\nkind = \"set_posture\"\nposture = \"attack\"",
    );
    let played = play(&t);
    let changes: Vec<(u32, Side)> = played
        .events
        .iter()
        .filter_map(|e| match &e.kind {
            EventKind::BaseChangedSide { base, to, .. } if *base == R => Some((e.day, *to)),
            _ => None,
        })
        .collect();
    assert_eq!(changes, vec![(2, A)]);
    assert_eq!(played.joined_on(2), vec![(U1, 100)]);
    for day in 3..=8 {
        assert!(played.joined_on(day).is_empty(), "day {day}");
    }
}

#[test]
fn statement_8_a_unit_divided() {
    // Unit 1 with 600 men and an establishment of 600 at R; a fast transport
    // there loads 200 of them on day 1 and lands them at R2 on day 2.
    let t = at(
        &text(),
        "forces",
        "[[task_force]]\nid = 2\nname = \"Placeholder J fast transport force\"\nside = \"japanese\"\nq = 14\nr = 0\nmission = \"transport\"\n\n[[ship]]\nid = 2\nname = \"Placeholder J fast transport\"\nclass = 11\nside = \"japanese\"\nfuel_t = 500\ntask_force = 2\n\n",
    );
    let t = unit_1_as(
        &t,
        "men = 600\nestablishment = 600\nexperience = 4\nq = 14\nr = 0",
    );
    let t = order(
        &t,
        "day = 1\nside = \"japanese\"\ntask_force = 2\nkind = \"load\"\nunit = 1\ncargo = \"none\"",
    );
    let t = order(
        &t,
        "day = 2\nside = \"japanese\"\ntask_force = 2\nkind = \"set_route\"\nwaypoints = [[15, 0]]",
    );
    let t = order(
        &t,
        "day = 2\nside = \"japanese\"\ntask_force = 2\nkind = \"set_speed\"\nspeed = \"cruise\"",
    );
    let t = order(
        &t,
        "day = 2\nside = \"japanese\"\ntask_force = 2\nkind = \"unload\"",
    );
    let played = play(&t);
    assert_eq!(played.truth[1].land_units[&U1].men_ashore, 400);
    assert_eq!(played.truth[1].men_aboard(U1), 200);
    let divided: Vec<(LandUnitId, LandUnitId, u32)> = played
        .events
        .iter()
        .filter_map(|e| match &e.kind {
            EventKind::UnitDivided {
                from, unit, men, ..
            } => Some((*from, *unit, *men)),
            _ => None,
        })
        .collect();
    assert_eq!(divided.len(), 1, "{divided:?}");
    let (from, new, men) = divided[0];
    assert_eq!((from, men), (U1, 200));
    let end = played.truth.len() - 1;
    assert_eq!(played.establishment(end, new), 200);
    assert_eq!(played.establishment(end, U1), 400);
    assert_eq!(played.count(end, new), 0);
}

#[test]
fn statement_9_they_are_fed() {
    // With all three units: on the night of day 2 the men of J at R are
    // 500 + 500 + 250.
    let played = play(&text());
    let fed: Vec<u32> = played
        .on(2)
        .filter_map(|e| match &e.kind {
            EventKind::MenFed { side, men, .. } if *side == J => Some(*men),
            _ => None,
        })
        .collect();
    assert_eq!(fed, vec![1250]);
    // With unit 1 alone at R, it draws for its 500 men: 500 x 0.005 = 2.5 t.
    let alone = changed(&changed(&text(), UNIT_2, ""), UNIT_3, "");
    let played = play(&alone);
    assert_eq!(played.joined_on(2), vec![(U1, 100)]);
    let drawn: Vec<(u32, Fixed)> = played
        .on(2)
        .filter_map(|e| match &e.kind {
            EventKind::BaseSupplySpent {
                base,
                reason: SupplyUse::Men,
                men,
                paid_t,
                ..
            } if *base == R => Some((*men, *paid_t)),
            _ => None,
        })
        .collect();
    assert_eq!(drawn, vec![(500, "2.5".parse::<Fixed>().unwrap())]);
}

#[test]
fn statement_10_refused_scenarios() {
    let m = load_error(&unit_1_as(
        &text(),
        "men = 400\nestablishment = 399\nq = 14\nr = 0",
    ));
    assert!(m.contains("establishment") && m.contains("below"), "{m}");
    let m = load_error(&changed(
        &text(),
        THE_STREAM,
        "[[men_replacement]]\nbase = 2\nmen = 0\n",
    ));
    assert!(
        m.contains("men replacement 1") && m.contains("no men"),
        "{m}"
    );
    let m = load_error(&changed(
        &text(),
        THE_STREAM,
        "[[men_replacement]]\nbase = 9\nmen = 100\n",
    ));
    assert!(
        m.contains("men replacement 1") && m.contains("unknown base"),
        "{m}"
    );
    // And the days, as a supply line's are.
    for (keys, words) in [
        ("first_day = 0", "first day below 1"),
        ("first_day = 9", "past the game's last day"),
        ("last_day = 9", "past the game's last day"),
        ("first_day = 4\nlast_day = 3", "before its first day"),
    ] {
        let m = load_error(&changed(
            &text(),
            THE_STREAM,
            &format!("[[men_replacement]]\nbase = 2\nmen = 100\n{keys}\n"),
        ));
        assert!(m.contains(words), "{keys}: {m}");
    }
    // An establishment equal to the men is allowed.
    load(&unit_1_as(
        &text(),
        "men = 400\nestablishment = 400\nq = 14\nr = 0",
    ));
}

// ----------------------------------------------------------------- Statement 11

fn repo_root() -> PathBuf {
    common::data_dir().join("..")
}

/// A recorded game's saved file replayed on the live figures as `sim-cli
/// replay` replays it, against the day-by-day hashes the file holds.
fn replayed(save: &str) -> Vec<(u32, String, String)> {
    let file = repo_root()
        .join("crates/sim-cli/tests/recorded_games")
        .join(save);
    let saved = sim_cli::save::read(file.to_str().expect("a path in words")).expect("reads");
    let scenario_path = repo_root().join(&saved.scenario_file);
    let scenario = Scenario::load(&scenario_path).expect("the scenario loads");
    // No stream and no establishment: nothing of the rule is kept.
    assert!(scenario.world.umpire().men_replacement.is_empty(), "{save}");
    let mut scenario = scenario.with_seed(saved.seed).expect("seeded");
    scenario.replace_script(saved.script.clone());
    scenario.replace_script_inside_day(saved.script_inside_day.clone());
    scenario.replace_commands(saved.commands.clone());
    let mut out = Vec::new();
    for (day, hash) in &saved.hashes {
        for (side, decisions) in saved.staff.get(day).into_iter().flatten() {
            scenario.world.submit_staff(*side, decisions.clone());
        }
        let summary = scenario
            .next_day_watched(sim_core::Staffs::none(), &mut |_, _| {})
            .expect("the day resolves")
            .expect("the day exists");
        out.push((*day, hash.clone(), format!("{:016x}", summary.state_hash)));
    }
    out
}

#[test]
fn statement_11_the_39_days_first_week_gives_the_hashes_it_gave() {
    for save in [
        "eastern_solomons_haiku/save.json",
        "full_slice_five_days_sonnet/save.json",
        "second_proof_week_sonnet/save.json",
    ] {
        let days = replayed(save);
        assert!(!days.is_empty(), "{save}");
        for (day, saved, replayed) in &days {
            assert_eq!(saved, replayed, "{save}, day {day}: the daily hash");
        }
    }
}

#[test]
fn statement_11_the_scenarios_hold_no_stream() {
    for name in ["05_eastern_solomons.toml", "07_full_slice.toml"] {
        let file = common::data_dir().join("scenarios").join(name);
        let scenario = common::load_live(&file).expect("the scenario loads");
        assert!(scenario.world.umpire().men_replacement.is_empty(), "{name}");
        let text = std::fs::read_to_string(&file).expect("the file reads");
        assert!(!text.contains("[[men_replacement]]"), "{name}");
        assert!(!text.contains("\nestablishment ="), "{name}");
    }
}

#[test]
fn statement_11_with_no_stream_and_no_establishment_nothing_is_kept_or_joins() {
    let none = changed(&text(), THE_STREAM, "");
    let none = unit_1_as(&none, "men = 400\nexperience = 4\nq = 14\nr = 0");
    let none = changed(
        &none,
        UNIT_3,
        "[[land_unit]]\nid = 3\nname = \"Placeholder J unit 3\"\nside = \"japanese\"\nmen = 250\nexperience = 3\nq = 14\nr = 0\n",
    );
    let a = play(&none);
    assert!(a.truth[0].men_replacement.is_empty());
    assert!(a.joined().is_empty());
    for day in 0..=8 {
        assert!(a.truth[day].men_replacement.is_empty());
        assert_eq!(a.men(day, U1), 400);
    }
    assert_eq!(a.hashes, play(&none).hashes);
    // The stream changes the hash.
    assert_ne!(a.hashes, play(&text()).hashes);
}

// ------------------------------------------------------------------ Properties

/// Every variant the properties are checked on.
fn variants() -> Vec<(String, String)> {
    let mut out: Vec<(String, String)> = vec![
        ("the file".into(), text()),
        ("unit 1 at 200".into(), unit_1_at_200()),
        (
            "an attack on day 1".into(),
            order(
                &allied_at_r(&text(), 200, "posture = \"attack\""),
                "day = 2\nside = \"allied\"\nland_unit = 4\nkind = \"set_posture\"\nposture = \"defend\"",
            ),
        ),
        (
            "R taken".into(),
            order(
                &allied_at_r(&text(), 20000, ""),
                "day = 2\nside = \"allied\"\nland_unit = 4\nkind = \"set_posture\"\nposture = \"attack\"",
            ),
        ),
        ("bombarded".into(), bombarded(&text())),
    ];
    for (men, establishment, experience, per_day) in [
        (100, 2000, 4, 700),
        (50, 60, 3, 1000),
        (300, 900, 4, 30),
        (100, 500, 1, 250),
    ] {
        let t = unit_1_as(
            &text(),
            &format!(
                "men = {men}\nestablishment = {establishment}\nexperience = {experience}\nq = 14\nr = 0"
            ),
        );
        let t = changed(
            &t,
            THE_STREAM,
            &format!("[[men_replacement]]\nbase = 2\nmen = {per_day}\n"),
        );
        out.push((
            format!("unit 1 {men}/{establishment} at {experience}, {per_day} a day"),
            t,
        ));
    }
    out.push((
        "two streams".into(),
        at(
            &text(),
            "streams",
            "[[men_replacement]]\nbase = 2\nmen = 75\nfirst_day = 3\nlast_day = 6\n\n",
        ),
    ));
    out
}

#[test]
fn properties_of_every_variant() {
    for (label, variant) in variants() {
        for seed in [1942u64, 7] {
            let played = play_seeded(&variant, Some(seed));
            let streams = played.truth[0].men_replacement.streams.clone();
            for day in 1..played.truth.len() {
                let d = u32::try_from(day).expect("a day");
                // A unit's men never pass its establishment by this rule.
                for (id, strength) in &played.truth[day].men_replacement.units {
                    if played.truth[day].land_units.contains_key(id) {
                        assert!(
                            played.men(day, *id) <= strength.establishment,
                            "{label}, seed {seed}, day {day}, {id}"
                        );
                    }
                }
                // The men who join in a night are never more than the
                // streams give.
                let given: u32 = streams
                    .iter()
                    .filter(|s| s.gives_on(d))
                    .map(|s| s.men_per_day)
                    .sum();
                let joined: u32 = played.joined_on(d).iter().map(|(_, men)| men).sum();
                assert!(joined <= given, "{label}, seed {seed}, day {day}");
            }
            // Experience is never raised by replacements, never lowered
            // below 2 by them, and never by more than one level in a night.
            for j in played.joined() {
                assert!(j.after <= j.before, "{label}: {j:?}");
                assert!(j.before - j.after <= 1, "{label}: {j:?}");
                if j.after < j.before {
                    assert!(j.after >= 2, "{label}: {j:?}");
                }
            }
            let mut nights: Vec<(u32, LandUnitId)> =
                played.joined().iter().map(|j| (j.day, j.unit)).collect();
            let all = nights.len();
            nights.dedup();
            assert_eq!(nights.len(), all, "{label}: one event a unit a night");
        }
    }
}

#[test]
fn property_the_same_seed_gives_the_same_daily_hashes_twice() {
    for (label, variant) in variants() {
        let first = play(&variant);
        let second = play(&variant);
        assert_eq!(first.hashes.len(), 8, "{label}");
        assert_eq!(first.hashes, second.hashes, "{label}");
        assert_eq!(first.events, second.events, "{label}");
    }
}
