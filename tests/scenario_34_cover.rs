//! Scenario 34: cover, men who dig in, and a place under bombardment.
//! Troops dig fieldworks for themselves and lose them when they leave them;
//! deeper defences cost more work for each point; cover saves men from
//! shells and in the fight; men under fire do not rest and tire, up to a
//! limit; and fire wears cover down.
//!
//! Proves decision 0106 against a hand calculation. The data is placeholder
//! data: see `data/scenarios/34_cover.toml`. The frozen step 1 figures make
//! the rule do nothing, so this test gives its figures the values below, as
//! the test of ships' experience sets its figure. `engagement_ratio` and
//! `failed_attack_fatigue` (decision 0105) are left at their frozen values.
//!
//! # Hand calculation
//!
//! Written by the architect before the code. Where the dice decide (the
//! hits of a bombardment and where each lands, the hits of a fight), the
//! calculation gives the rule exactly and the tests read the hits from the
//! run. Everything else is asserted exactly. If the code disagrees, the
//! code or the rule is wrong: do not change a number here to make a test
//! pass. A mismatch goes back to the architect.
//!
//! ## Figures
//!
//! `cover_from_fire` 0.9, `cover_in_fight` 0.5, `fieldworks_per_day` 10,
//! `fieldworks_limit` 30, `prepared_from` 30, `fortified_from` 60,
//! `prepared_work_factor` 3, `fortified_work_factor` 10, `strain_per_hit`
//! 1, `strain_per_day` 10, `strain_limit` 60, `cover_from_strain` 0.5,
//! `cover_wear_per_hit` 0.2.
//!
//! From the earlier rules: `work_per_defence_point` 200, `work_per_man_day`
//! 1, `defences_strength` 0.5, `men_lost_per_base_hit` 10, `men_per_shot`
//! 100, `hit_chance` 0.3, `defender_hit_chance` 0.6, `men_per_hit` 20,
//! `fatigue_per_attack_day` 20, `rest_per_day` 5.
//!
//! A unit's cover is the higher of its base's defences and its own
//! fieldworks; a unit that marches or is on attack has none. Hits land in
//! the pulses; fieldworks are dug, work is made and the day's fatigue is
//! settled in the ground combat phase at the end of the day. So a hit meets
//! the cover the day began with, less what earlier hits of the day wore
//! away.
//!
//! ## Inputs
//!
//! A placeholder map with five coast hexes far apart and no land link: A,
//! B, C, D and F. The game is 6 days.
//!
//! | Base | Side     | Hex | Supply | Defences at the start |
//! | ---- | -------- | --- | ------ | --------------------- |
//! | 1    | Allied   | A   | 5000 t | 0                     |
//! | 2    | Allied   | B   | 5000 t | 30                    |
//! | 3    | Allied   | C   | 5000 t | 59                    |
//! | 4    | Allied   | D   | 5000 t | 80                    |
//! | 5    | Japanese | D   | 5000 t | 0                     |
//! | 6    | Allied   | F   | 5000 t | 50                    |
//!
//! | Unit | Side     | Kind     | Men  | Hex | Posture | Fatigue at the start |
//! | ---- | -------- | -------- | ---- | --- | ------- | -------------------- |
//! | 1    | Allied   | fighting | 1000 | A   | defend  | 0                    |
//! | 2    | Allied   | working  | 1800 | B   | defend  | 0                    |
//! | 3    | Allied   | working  | 1500 | C   | defend  | 0                    |
//! | 4    | Allied   | fighting | 1000 | D   | defend  | 0                    |
//! | 5    | Japanese | fighting | 1000 | D   | attack  | 0                    |
//! | 6    | Allied   | fighting | 1000 | F   | defend  | 58                   |
//! | 7    | Allied   | fighting | 500  | F   | defend  | 70                   |
//!
//! Two Japanese task forces with the bombardment mission, built as the
//! bombarding force of scenario 24 is: force 1 shells base 6 in the night
//! pulse of day 2, and force 2 shells base 1 in the night pulse of day 3.
//! Neither shells again.
//!
//! Orders: on day 1, units 2 and 3 work at the defences. On day 4, unit 1
//! is set to attack. On day 5, unit 1 is set to defend.
//!
//! ## At A: fieldworks, and shells on men who have dug
//!
//! | Day | Unit 1's posture | Fieldworks through the day | Fieldworks after the day |
//! | --- | ---------------- | -------------------------- | ------------------------ |
//! | 1   | defend           | 0                          | 10                       |
//! | 2   | defend           | 10                         | 20                       |
//! | 3   | defend           | 20, less the day's wear    | see below                |
//! | 4   | attack           | 0                          | 0                        |
//! | 5   | defend           | 0                          | 10                       |
//! | 6   | defend           | 10                         | 20                       |
//!
//! The order to attack on day 4 loses the fieldworks at intake, though no
//! enemy is at A and nothing is attacked. Set back to defend on day 5, the
//! unit digs again from nothing.
//!
//! Day 3, the shelling. Let h be the hits that land on men at A, read from
//! the run, in the order they land.
//!
//! - The k-th hit meets fieldworks of 20 - 0.2 x (k - 1), and base 1 has no
//!   defences, so that is unit 1's cover. The hit would take 10 men; it
//!   takes 10 x (1 - cover/100 x 0.9), cut to a whole man. The first hit
//!   takes 10 x 0.82 = 8.2: 8 men.
//! - Each hit wears the fieldworks by 0.2. After the day's digging they are
//!   20 - 0.2 x h + 10, and no more than 30.
//! - If h is 1 or more, unit 1 does not rest on day 3. It gains h, and no
//!   more than 10, times 1 less its cover over 100 times 0.5, with the
//!   cover it has when the day is settled, before that day's digging: 20 -
//!   0.2 x h. If h is 0 it rests.
//! - On every other day unit 1 rests, and its fatigue does not go below 0.
//!
//! ## At B and C: each grade costs more
//!
//! Unit 2 makes 1800 of work a day. Base 2's defences are 30, the bottom of
//! the prepared grade, where a point costs 200 x 3 = 600: 3 points a day.
//!
//! | After day | 1  | 2  | 3  | 4  | 5  | 6  |
//! | --------- | -- | -- | -- | -- | -- | -- |
//! | Base 2    | 33 | 36 | 39 | 42 | 45 | 48 |
//!
//! Unit 3 makes 1500 of work a day. Base 3's defences are 59. On day 1 the
//! one point to 60 costs 600; the 900 left are spent in the fortified
//! grade, where a point costs 200 x 10 = 2000: 0.45. After that each day
//! adds 1500 / 2000 = 0.75.
//!
//! | After day | 1     | 2    | 3     | 4    | 5     | 6    |
//! | --------- | ----- | ---- | ----- | ---- | ----- | ---- |
//! | Base 3    | 60.45 | 61.2 | 61.95 | 62.7 | 63.45 | 64.2 |
//!
//! ## At D: cover in the fight
//!
//! Day 1. The Japanese side attacks.
//!
//! - Unit 5: 1000 firing men, 10 shots at 0.3. It is on attack and has no
//!   cover.
//! - Unit 4: 1000 firing men, 10 shots at 0.6 x (1 + 0.5 x 80/100) = 0.84.
//!   Its cover is 80, the defences of base 4.
//!
//! Let a be unit 5's hits and d unit 4's. Unit 4 would lose 20 x a men; its
//! cover saves 80/100 x 0.5 of them: it loses 20 x a x 0.6 = 12 x a. Unit 5
//! loses 20 x d, or all it has.
//!
//! Unit 4 digs as it stands: its fieldworks are 10 after day 1 and 30 from
//! day 3. Its cover stays 80, the higher of the two. On each later day of
//! the fight the tests work the same rule from the men and fatigue the run
//! has before it.
//!
//! ## At F: strain, its limit, and wear on a base's defences
//!
//! Day 2, the shelling. Let g be the hits that land on men at F, in the
//! order they land.
//!
//! - The shelling would take 10 x g men. They are shared between units 6
//!   and 7 in proportion to their men, by the rule that serves every loss,
//!   as a shelling's men always were. Each unit's part P is then taken hit
//!   by hit, spread evenly: the piece for the k-th hit is P x k / g, cut,
//!   less P x (k - 1) / g, cut. The k-th hit meets defences of 50 - 0.2 x
//!   (k - 1), which is the cover of both units (their fieldworks are 10),
//!   and each piece is cut by it: times 1 less cover/100 x 0.9, cut to a
//!   whole man.
//! - Base 6's defences are 50 - 0.2 x g after the day. Nobody works on
//!   them: they stay there to the end. The fieldworks of units 6 and 7 are
//!   10 - 0.2 x g + 10 after the day's digging.
//! - If g is 1 or more, neither unit rests on day 2. The strain is g, and
//!   no more than 10, times 1 less the cover over 100 times 0.5, with the
//!   cover 50 - 0.2 x g.
//!   - Unit 6 began the day at 58 - 5 = 53. It gains the strain, and strain
//!     raises it no higher than 60.
//!   - Unit 7 began the day at 70 - 5 = 65, above 60: strain adds nothing,
//!     and it does not rest. It stays at 65.
//! - If g is 0 both rest: 48 and 60.
//! - On every other day both rest 5.
//!
//! # Properties
//!
//! - Cover never raises a loss: a unit never loses more men to a hit or in
//!   a fight than it would with no cover.
//! - Fieldworks are never below 0 or above the limit, and a base's defences
//!   never above 100.
//! - With the frozen figures every scenario gives what it gave before,
//!   apart from the hash version; scenario 30's defences are as its
//!   calculation has them.
//! - Crossing from the improvised grade to the prepared one is worked as
//!   crossing to the fortified one is at C: 800 of work at defences of 29
//!   gives 31.
//! - The enemy's view is the same in two worlds that differ only in the
//!   other side's fieldworks and defences, where the hits and the men lost
//!   are the same.

mod common;

use std::collections::BTreeMap;
use std::path::PathBuf;

use sim_core::events::{Cause, Event, EventKind, FieldworksChange, LandFatigueChange};
use sim_core::fixed::Fixed;
use sim_core::hex::Hex;
use sim_core::ids::{BaseId, LandUnitId, TaskForceId};
use sim_core::side_events::SideEventKind;
use sim_core::state::{Side, TruthState};
use sim_data::{LoadOptions, Scenario};

const A: Hex = Hex::new(0, 0);
const D: Hex = Hex::new(30, 0);
const F: Hex = Hex::new(40, 0);

/// The frozen step 1 figures with the thirteen figures of the hand
/// calculation. `prepared_from`, `fortified_from` and `strain_limit` already
/// hold the values the calculation uses.
fn figures(tag: &str) -> LoadOptions {
    common::altered_reference(tag, |dir| {
        common::edit_file_by(
            dir,
            "ground.toml",
            &[
                ("cover_from_fire = 0", "cover_from_fire = \"0.9\""),
                ("cover_in_fight = 0", "cover_in_fight = \"0.5\""),
                ("fieldworks_per_day = 0", "fieldworks_per_day = 10"),
                ("fieldworks_limit = 0", "fieldworks_limit = 30"),
                ("prepared_work_factor = 1", "prepared_work_factor = 3"),
                ("fortified_work_factor = 1", "fortified_work_factor = 10"),
                ("strain_per_hit = 0", "strain_per_hit = 1"),
                ("strain_per_day = 0", "strain_per_day = 10"),
                ("cover_from_strain = 0", "cover_from_strain = \"0.5\""),
                ("cover_wear_per_hit = 0", "cover_wear_per_hit = \"0.2\""),
            ],
        );
    })
}

fn path() -> PathBuf {
    common::data_dir().join("scenarios/34_cover.toml")
}

fn text() -> String {
    std::fs::read_to_string(path()).expect("the scenario file reads")
}

fn from(source: &str) -> Scenario {
    common::from_text_with(source, &path(), &figures("s34")).expect("the scenario loads")
}

fn fx(text: &str) -> Fixed {
    text.parse().expect("a decimal")
}

fn n(whole: i64) -> Fixed {
    Fixed::from_int(whole)
}

fn unit(raw: u32) -> LandUnitId {
    LandUnitId::new(raw)
}

fn base(raw: u32) -> BaseId {
    BaseId::new(raw)
}

fn force(raw: u32) -> TaskForceId {
    TaskForceId::new(raw)
}

/// The men a loss of `men` leaves after `cover` that saves `share` of them at
/// full cover: the men times 1 less the cover over 100 times the share, cut.
fn cut(men: u32, cover: Fixed, share: Fixed) -> u32 {
    let kept = Fixed::ONE - share.mul_div(cover, n(100));
    u32::try_from((n(i64::from(men)) * kept).trunc().max(0)).expect("a man count")
}

/// The pieces of a part over `hits` hits, spread evenly.
fn pieces(part: u32, hits: u32) -> Vec<u32> {
    let upto = |k: u32| u64::from(part) * u64::from(k) / u64::from(hits);
    (1..=hits)
        .map(|k| u32::try_from(upto(k) - upto(k - 1)).expect("a piece"))
        .collect()
}

/// A loss shared between units in proportion to their men: each part is cut,
/// and the men left over go one to each unit in unit order.
fn shared(loss: u32, men: &[u32]) -> Vec<u32> {
    let total: u32 = men.iter().sum();
    let mut parts: Vec<u32> = men
        .iter()
        .map(|m| u32::try_from(u64::from(loss) * u64::from(*m) / u64::from(total)).expect("part"))
        .collect();
    let mut left = loss - parts.iter().sum::<u32>();
    assert!(left as usize <= parts.len());
    for part in parts.iter_mut() {
        if left == 0 {
            break;
        }
        *part += 1;
        left -= 1;
    }
    parts
}

/// What a unit with `cover` at the start of the call loses to `hits` hits on
/// men sharing the part `part`: each piece cut by the cover the hit meets,
/// which each earlier hit has worn by `wear`.
fn shelling_loss(part: u32, hits: u32, cover: Fixed, wear: Fixed) -> u32 {
    if hits == 0 {
        return 0;
    }
    pieces(part, hits)
        .iter()
        .enumerate()
        .map(|(i, piece)| {
            let cover_k = cover - wear * i64::try_from(i).expect("index");
            cut(*piece, cover_k, fx("0.9"))
        })
        .sum()
}

/// The strain of a day: the hits, no more than 10, times 1 less the cover
/// over 100 times 0.5, and no more than takes the fatigue to 60.
fn strain(hits: u32, cover: Fixed, before: Fixed) -> Fixed {
    let raw = (Fixed::ONE * i64::from(hits)).min(n(10));
    let cutted = raw * (Fixed::ONE - fx("0.5").mul_div(cover, n(100)));
    cutted.min((n(60) - before).max(Fixed::ZERO))
}

fn rest(before: Fixed) -> Fixed {
    (before - n(5)).max(Fixed::ZERO)
}

/// A run of a scenario: the truth at the start (day 0) and at the end of each
/// day, the Allied side's cover of each of its units at those times (and the
/// Japanese side's of its own), and the log.
struct Run {
    days: Vec<TruthState>,
    allied_cover: Vec<BTreeMap<u32, Option<Fixed>>>,
    japanese_cover: Vec<Option<Fixed>>,
    events: Vec<Event>,
}

impl Run {
    fn of(mut scenario: Scenario) -> Run {
        let mut days = Vec::new();
        let mut allied_cover = Vec::new();
        let mut japanese_cover = Vec::new();
        let snap = |scenario: &Scenario,
                    days: &mut Vec<TruthState>,
                    allied: &mut Vec<BTreeMap<u32, Option<Fixed>>>,
                    japanese: &mut Vec<Option<Fixed>>| {
            days.push(scenario.world.umpire().clone());
            let view = scenario.world.side_view(Side::Allied);
            allied.push(
                scenario
                    .world
                    .umpire()
                    .land_units
                    .keys()
                    .map(|id| (id.raw(), view.cover(*id)))
                    .collect(),
            );
            japanese.push(scenario.world.side_view(Side::Japanese).cover(unit(5)));
        };
        snap(&scenario, &mut days, &mut allied_cover, &mut japanese_cover);
        while scenario.next_day().expect("a day resolves").is_some() {
            snap(&scenario, &mut days, &mut allied_cover, &mut japanese_cover);
        }
        let events = scenario.world.events().iter().cloned().collect();
        Run {
            days,
            allied_cover,
            japanese_cover,
            events,
        }
    }

    fn live() -> Run {
        Run::of(from(&text()))
    }

    fn at(&self, day: u32) -> &TruthState {
        &self.days[day as usize]
    }

    fn men(&self, day: u32, raw: u32) -> u32 {
        self.at(day).land_units[&unit(raw)].men_ashore
    }

    fn fatigue(&self, day: u32, raw: u32) -> Fixed {
        self.at(day).land_units[&unit(raw)].fatigue
    }

    fn fieldworks(&self, day: u32, raw: u32) -> Fixed {
        self.at(day).land_units[&unit(raw)].fieldworks
    }

    fn defences(&self, day: u32, raw: u32) -> Fixed {
        self.at(day).bases[&base(raw)].defences
    }

    fn on(&self, day: u32, pick: impl Fn(&EventKind) -> bool) -> Vec<&Event> {
        self.events
            .iter()
            .filter(|e| e.day == day && pick(&e.kind))
            .collect()
    }

    fn all(&self, pick: impl Fn(&EventKind) -> bool) -> Vec<&Event> {
        self.events.iter().filter(|e| pick(&e.kind)).collect()
    }

    /// The hits on men of the bombardment by `force` (one call in the game).
    fn hits_on_men(&self, day: u32, raw_force: u32) -> Vec<(u32, Hex)> {
        self.on(day, |k| {
            matches!(k, EventKind::ShoreBombardment { task_force, .. } if *task_force == force(raw_force))
        })
        .iter()
        .map(|e| match e.kind {
            EventKind::ShoreBombardment {
                hits_on_men, hex, ..
            } => (hits_on_men, hex),
            _ => unreachable!(),
        })
        .collect()
    }

    /// What a unit lost to fire on a day, by event.
    fn lost_to_fire(&self, day: u32, raw: u32) -> Vec<&Event> {
        self.on(
            day,
            |k| matches!(k, EventKind::MenLostToFire { unit, .. } if unit.raw() == raw),
        )
    }

    fn lost_in_fight(&self, day: u32, raw: u32) -> u32 {
        self.on(
            day,
            |k| matches!(k, EventKind::MenLostInFight { unit, .. } if unit.raw() == raw),
        )
        .iter()
        .map(|e| match e.kind {
            EventKind::MenLostInFight { lost, .. } => lost,
            _ => unreachable!(),
        })
        .sum()
    }

    /// A unit's fire in a day's fight: `(firing men, shots, chance, hits)`.
    fn fire(&self, day: u32, raw: u32) -> Option<(u32, Fixed, Fixed, u32)> {
        let found = self.on(
            day,
            |k| matches!(k, EventKind::GroundFire { unit, .. } if unit.raw() == raw),
        );
        assert!(found.len() <= 1);
        found.first().map(|e| match e.kind {
            EventKind::GroundFire {
                firing_men,
                shots,
                chance,
                hits,
                ..
            } => (firing_men, shots, chance, hits),
            _ => unreachable!(),
        })
    }

    /// The day's fatigue events of a unit.
    fn fatigue_events(&self, day: u32, raw: u32) -> Vec<(LandFatigueChange, Fixed)> {
        self.on(
            day,
            |k| matches!(k, EventKind::LandUnitFatigue { unit, .. } if unit.raw() == raw),
        )
        .iter()
        .map(|e| match e.kind {
            EventKind::LandUnitFatigue {
                change, fatigue, ..
            } => (change, fatigue),
            _ => unreachable!(),
        })
        .collect()
    }

    /// The day's events of fieldworks of a unit: `(change, fieldworks)`.
    fn fieldworks_events(&self, day: u32, raw: u32) -> Vec<(FieldworksChange, Fixed)> {
        self.on(
            day,
            |k| matches!(k, EventKind::FieldworksChanged { unit, .. } if unit.raw() == raw),
        )
        .iter()
        .map(|e| match e.kind {
            EventKind::FieldworksChanged {
                change, fieldworks, ..
            } => (change, fieldworks),
            _ => unreachable!(),
        })
        .collect()
    }

    /// The `total` of the `UnitWorked` events of a unit on a day.
    fn worked_total(&self, day: u32, raw: u32) -> Vec<Fixed> {
        self.on(
            day,
            |k| matches!(k, EventKind::UnitWorked { unit, .. } if unit.raw() == raw),
        )
        .iter()
        .map(|e| match e.kind {
            EventKind::UnitWorked { total, .. } => total,
            _ => unreachable!(),
        })
        .collect()
    }

    /// The day's events of the fatigue of a unit must be exactly what takes
    /// `before` to `after` by `change` (none if they are the same).
    fn assert_fatigue(&self, day: u32, raw: u32, change: LandFatigueChange, after: Fixed) {
        let before = self.fatigue(day - 1, raw);
        assert_eq!(self.fatigue(day, raw), after, "unit {raw} day {day}");
        let events = self.fatigue_events(day, raw);
        if before == after {
            assert!(events.is_empty(), "unit {raw} day {day}: {events:?}");
        } else {
            assert_eq!(events, vec![(change, after)], "unit {raw} day {day}");
        }
    }
}

fn the_bombardments_are_one_at_f_on_day_2_and_one_at_a_on_day_3(run: &Run) {
    let all = run.all(|k| matches!(k, EventKind::ShoreBombardment { .. }));
    assert_eq!(all.len(), 2);
    assert_eq!(run.hits_on_men(2, 1).len(), 1);
    assert_eq!(run.hits_on_men(2, 1)[0].1, F);
    assert_eq!(run.hits_on_men(3, 2).len(), 1);
    assert_eq!(run.hits_on_men(3, 2)[0].1, A);
}

// ---------------------------------------------------------------------------
// At A: fieldworks, and shells on men who have dug.

#[test]
fn at_a_unit_1_digs_is_shelled_leaves_its_holes_and_digs_again() {
    let run = Run::live();
    the_bombardments_are_one_at_f_on_day_2_and_one_at_a_on_day_3(&run);
    let h = run.hits_on_men(3, 2)[0].0;
    eprintln!("A: h = {h}");

    // Day 3: the men lost, hit by hit.
    let before = run.men(2, 1);
    assert_eq!(before, 1000);
    let cover0 = run.fieldworks(2, 1).max(run.defences(2, 1));
    assert_eq!(cover0, n(20));
    assert_eq!(run.defences(2, 1), Fixed::ZERO, "base 1 has no defences");
    let loss = 10 * h;
    let part = shared(loss.min(before), &[before])[0];
    let expected = shelling_loss(part, h, cover0, fx("0.2"));
    if h >= 1 {
        // The first hit takes 8 of 10.
        assert_eq!(pieces(part, h)[0], 10);
        assert_eq!(cut(10, cover0, fx("0.9")), 8);
        // No hit takes more than the 10 it would with no cover.
        assert!(expected <= loss);
    }
    let events = run.lost_to_fire(3, 1);
    if expected == 0 {
        assert!(events.is_empty());
    } else {
        assert_eq!(events.len(), 1);
        match events[0].kind {
            EventKind::MenLostToFire {
                lost,
                ashore,
                base: b,
                ..
            } => {
                assert_eq!(lost, expected);
                assert_eq!(ashore, before - expected);
                assert_eq!(b, base(1));
            }
            _ => unreachable!(),
        }
    }
    assert_eq!(run.men(3, 1), before - expected);
    for day in [1, 2, 4, 5, 6] {
        assert!(run.lost_to_fire(day, 1).is_empty());
        assert_eq!(run.men(day, 1), run.men(day - 1, 1), "day {day}");
    }

    // The fieldworks through the days.
    let worn = (cover0 - fx("0.2") * i64::from(h)).max(Fixed::ZERO);
    let dug_on_3 = (worn + n(10)).min(n(30));
    let expected_fw = [n(10), n(20), dug_on_3, Fixed::ZERO, n(10), n(20)];
    for (i, want) in expected_fw.iter().enumerate() {
        let day = u32::try_from(i).expect("day") + 1;
        assert_eq!(run.fieldworks(day, 1), *want, "day {day}");
    }
    // Their events: dug each day but day 4 (on attack), worn on day 3 by
    // the shells, lost on day 4 by the order.
    for day in [1, 2, 5, 6] {
        assert_eq!(
            run.fieldworks_events(day, 1),
            vec![(FieldworksChange::Dug, run.fieldworks(day, 1))],
            "day {day}"
        );
    }
    let mut on_3 = Vec::new();
    if h >= 1 {
        on_3.push((FieldworksChange::Worn, worn));
    }
    on_3.push((FieldworksChange::Dug, dug_on_3));
    assert_eq!(run.fieldworks_events(3, 1), on_3);
    assert_eq!(
        run.fieldworks_events(4, 1),
        vec![(FieldworksChange::Lost, Fixed::ZERO)],
        "lost at the order, and no digging on a day of attack"
    );

    // The loss at day 4 is a follow-up of the attack order.
    let lost = run
        .on(4, |k| {
            matches!(
                k,
                EventKind::FieldworksChanged {
                    change: FieldworksChange::Lost,
                    ..
                }
            )
        })
        .pop()
        .expect("lost");
    let position = run
        .events
        .iter()
        .position(|e| e.id == lost.id)
        .expect("in the log");
    let accepted = &run.events[position - 1];
    match accepted.kind {
        EventKind::OrderAccepted { order, target } => {
            assert!(
                format!("{target:?}").starts_with("LandUnit("),
                "the order was to a land unit"
            );
            let _ = order;
            assert_eq!(lost.causes, vec![Cause::Order(order)]);
        }
        ref other => panic!("the event before is not the acceptance: {other:?}"),
    }

    // The fatigue of unit 1: strain on day 3 if hit, else rest every day.
    let cover_settled = worn;
    for day in 1..=6u32 {
        let before = run.fatigue(day - 1, 1);
        if day == 3 && h >= 1 {
            let s = strain(h, cover_settled, before);
            assert!(s > Fixed::ZERO);
            run.assert_fatigue(day, 1, LandFatigueChange::Strained, before + s);
        } else {
            run.assert_fatigue(day, 1, LandFatigueChange::Rested, rest(before));
        }
    }
    // The cover the side sees: day 4 on attack none, otherwise the holes.
    for day in 1..=6u32 {
        let want = if day == 4 {
            Fixed::ZERO
        } else {
            run.fieldworks(day, 1)
        };
        assert_eq!(run.allied_cover[day as usize][&1], Some(want), "day {day}");
    }
}

// ---------------------------------------------------------------------------
// At B and C: each grade costs more.

#[test]
fn at_b_and_c_each_grade_costs_more() {
    let run = Run::live();
    let b = ["33", "36", "39", "42", "45", "48"];
    let c = ["60.45", "61.2", "61.95", "62.7", "63.45", "64.2"];
    for day in 1..=6u32 {
        let i = (day - 1) as usize;
        assert_eq!(run.defences(day, 2), fx(b[i]), "base 2 day {day}");
        assert_eq!(run.worked_total(day, 2), vec![fx(b[i])], "unit 2 day {day}");
        assert_eq!(run.defences(day, 3), fx(c[i]), "base 3 day {day}");
        assert_eq!(run.worked_total(day, 3), vec![fx(c[i])], "unit 3 day {day}");
        // Nothing digs for itself among the working units.
        assert_eq!(run.fieldworks(day, 2), Fixed::ZERO);
        assert_eq!(run.fieldworks(day, 3), Fixed::ZERO);
        // The cover of a working unit is its base's defences.
        assert_eq!(run.allied_cover[day as usize][&2], Some(fx(b[i])));
        assert_eq!(run.allied_cover[day as usize][&3], Some(fx(c[i])));
    }
    // Nothing else works on defences; none is built at the other bases but
    // the wear (base 6, in its own test).
    assert_eq!(run.defences(6, 4), n(80));
    assert_eq!(run.defences(6, 5), Fixed::ZERO);
}

// ---------------------------------------------------------------------------
// At D: cover in the fight.

#[test]
fn at_d_cover_saves_unit_4_men_in_the_fight() {
    let run = Run::live();
    let mut fights = 0;
    for day in 1..=6u32 {
        let (m4, m5) = (run.men(day - 1, 4), run.men(day - 1, 5));
        let (f4, f5) = (run.fatigue(day - 1, 4), run.fatigue(day - 1, 5));
        let attacks =
            run.at(day - 1).land_units[&unit(5)].posture == sim_core::orders::Posture::Attack;
        let fire4 = run.fire(day, 4);
        let fire5 = run.fire(day, 5);
        // Both sides are fed.
        for e in run.on(
            day,
            |k| matches!(k, EventKind::MenFed { hex, .. } if *hex == D),
        ) {
            match e.kind {
                EventKind::MenFed { unfed, .. } => assert_eq!(unfed, 0),
                _ => unreachable!(),
            }
        }
        if !attacks {
            // After unit 5 goes back to defend there is no fight.
            assert!(fire4.is_none() && fire5.is_none(), "day {day}");
            assert_eq!(run.men(day, 4), m4);
            assert_eq!(run.men(day, 5), m5);
            continue;
        }
        fights += 1;
        let cover4 = run.allied_cover[(day - 1) as usize][&4].expect("own unit");
        assert_eq!(
            cover4,
            n(80),
            "the higher of the base's defences and the holes"
        );
        let firing = |men: u32, fatigue: Fixed| {
            u32::try_from(n(i64::from(men)).mul_div(n(100) - fatigue, n(100)).trunc()).expect("men")
        };
        let (fire4, fire5) = (fire4.expect("unit 4 fires"), fire5.expect("unit 5 fires"));
        let (firing4, firing5) = (firing(m4, f4), firing(m5, f5));
        assert_eq!(fire4.0, firing4, "day {day}");
        assert_eq!(fire5.0, firing5, "day {day}");
        assert_eq!(fire4.1, Fixed::from_ratio(i64::from(firing4), 100));
        assert_eq!(fire5.1, Fixed::from_ratio(i64::from(firing5), 100));
        assert_eq!(fire5.2, fx("0.3"), "an attacker has no cover");
        assert_eq!(
            fire4.2,
            fx("0.6") * (Fixed::ONE + fx("0.5").mul_div(cover4, n(100)))
        );
        assert_eq!(fire4.2, fx("0.84"));
        let (a, d) = (fire5.3, fire4.3);
        // Unit 4 loses its part cut by its cover, unit 5 its whole part.
        let lost4 = cut((20 * a).min(m4), cover4, fx("0.5"));
        let lost5 = (20 * d).min(m5);
        assert_eq!(run.lost_in_fight(day, 4), lost4, "day {day}");
        assert_eq!(run.lost_in_fight(day, 5), lost5, "day {day}");
        if m4 >= 20 * a {
            assert_eq!(lost4, 12 * a, "day {day}");
        }
        assert_eq!(run.men(day, 4), m4 - lost4);
        assert_eq!(run.men(day, 5), m5 - lost5);
        eprintln!("D day {day}: a = {a}, d = {d}, unit 4 lost {lost4}, unit 5 lost {lost5}");
        if day == 1 {
            assert_eq!((fire4.0, fire4.1), (1000, n(10)));
            assert_eq!((fire5.0, fire5.1), (1000, n(10)));
        }
        // Unit 5 tires 20 for each day it fires, and goes back to defend
        // after the day it passes 60.
        run.assert_fatigue(day, 5, LandFatigueChange::Gained, f5 + n(20));
        let still_attacks =
            run.at(day).land_units[&unit(5)].posture == sim_core::orders::Posture::Attack;
        assert_eq!(still_attacks, run.fatigue(day, 5) <= n(60), "day {day}");
    }
    assert_eq!(fights, 4, "unit 5 fights four days: 20, 40, 60, 80");
    assert_eq!(
        (1..=4).map(|d| run.fatigue(d, 5)).collect::<Vec<_>>(),
        vec![n(20), n(40), n(60), n(80)]
    );
    // Unit 4 is never tired, never shelled, and never loses its cover.
    for day in 1..=6u32 {
        assert_eq!(run.fatigue(day, 4), Fixed::ZERO);
        assert_eq!(
            run.fieldworks(day, 4),
            [n(10), n(20), n(30), n(30), n(30), n(30)][(day - 1) as usize]
        );
        assert_eq!(run.allied_cover[day as usize][&4], Some(n(80)));
        assert!(run.lost_to_fire(day, 4).is_empty());
    }
    // Unit 5 goes back to defend after day 4, rests 5 a day and digs from
    // then, on the day it goes back as on any other, in the same phase.
    assert_eq!(
        (4..=6).map(|d| run.fatigue(d, 5)).collect::<Vec<_>>(),
        vec![n(80), n(75), n(70)]
    );
    for day in 1..=6u32 {
        let want = [0, 0, 0, 10, 20, 30][(day - 1) as usize];
        assert_eq!(run.fieldworks(day, 5), n(want), "unit 5 day {day}");
    }
    // The Japanese side's own cover: none while it attacks, its holes after.
    for day in 0..=6usize {
        let want = if day <= 3 {
            Fixed::ZERO
        } else {
            run.fieldworks(day as u32, 5)
        };
        assert_eq!(run.japanese_cover[day], Some(want), "day {day}");
    }
}

// ---------------------------------------------------------------------------
// At F: strain, its limit, and wear on a base's defences.

#[test]
fn at_f_shells_wear_the_defences_and_the_strain_stops_at_its_limit() {
    let run = Run::live();
    the_bombardments_are_one_at_f_on_day_2_and_one_at_a_on_day_3(&run);
    let g = run.hits_on_men(2, 1)[0].0;
    eprintln!("F: g = {g}");

    // The call's loss, its parts and what each unit loses.
    let men = [run.men(1, 6), run.men(1, 7)];
    assert_eq!(men, [1000, 500]);
    let loss = (10 * g).min(men[0] + men[1]);
    let parts = shared(loss, &men);
    let cover0 = run.defences(1, 6).max(run.fieldworks(1, 6));
    assert_eq!(cover0, n(50));
    assert_eq!(run.fieldworks(1, 6), n(10));
    assert_eq!(run.fieldworks(1, 7), n(10));
    let lost: Vec<u32> = parts
        .iter()
        .map(|p| shelling_loss(*p, g, cover0, fx("0.2")))
        .collect();
    eprintln!("F: parts {parts:?}, lost {lost:?}");
    for (i, raw) in [6u32, 7].into_iter().enumerate() {
        let events = run.lost_to_fire(2, raw);
        if lost[i] == 0 {
            assert!(events.is_empty());
        } else {
            assert_eq!(events.len(), 1);
            match events[0].kind {
                EventKind::MenLostToFire {
                    lost: l, ashore, ..
                } => {
                    assert_eq!(l, lost[i], "unit {raw}");
                    assert_eq!(ashore, men[i] - lost[i]);
                }
                _ => unreachable!(),
            }
        }
        assert_eq!(run.men(2, raw), men[i] - lost[i]);
        for day in [1, 3, 4, 5, 6] {
            assert_eq!(run.men(day, raw), run.men(day - 1, raw).min(men[i]));
            assert!(run.lost_to_fire(day, raw).is_empty());
        }
    }

    // Base 6's defences: worn by the shells, and never worked.
    let worn_defences = (n(50) - fx("0.2") * i64::from(g)).max(Fixed::ZERO);
    assert_eq!(run.defences(1, 6), n(50));
    for day in 2..=6 {
        assert_eq!(run.defences(day, 6), worn_defences, "day {day}");
    }
    let worn_events = run.all(|k| matches!(k, EventKind::DefencesWorn { .. }));
    if g == 0 {
        assert!(worn_events.is_empty());
    } else {
        assert_eq!(worn_events.len(), 1);
        match worn_events[0].kind {
            EventKind::DefencesWorn {
                base: b,
                side,
                hex,
                hits,
                defences,
            } => {
                assert_eq!((b, side, hex, hits), (base(6), Side::Allied, F, g));
                assert_eq!(defences, worn_defences);
            }
            _ => unreachable!(),
        }
    }
    assert!(run
        .all(|k| matches!(k, EventKind::UnitWorked { base: b, .. } if *b == base(6)))
        .is_empty());

    // The fieldworks of units 6 and 7: 10, then worn, then dug, then 30.
    for raw in [6u32, 7] {
        let worn_fw = (n(10) - fx("0.2") * i64::from(g)).max(Fixed::ZERO);
        assert_eq!(run.fieldworks(1, raw), n(10));
        assert_eq!(run.fieldworks(2, raw), worn_fw + n(10));
        assert_eq!(run.fieldworks(3, raw), (worn_fw + n(20)).min(n(30)));
        for day in 4..=6 {
            assert_eq!(run.fieldworks(day, raw), n(30));
        }
        let on_2 = run.fieldworks_events(2, raw);
        let mut want = Vec::new();
        if g >= 1 {
            want.push((FieldworksChange::Worn, worn_fw));
        }
        want.push((FieldworksChange::Dug, worn_fw + n(10)));
        assert_eq!(on_2, want, "unit {raw}");
    }

    // Fatigue: 53 and 65 after day 1; day 2 as stated; then 5 a day.
    run.assert_fatigue(1, 6, LandFatigueChange::Rested, n(53));
    run.assert_fatigue(1, 7, LandFatigueChange::Rested, n(65));
    if g >= 1 {
        let cover_settled = worn_defences.max(run.fieldworks(2, 6) - n(10));
        let s6 = strain(g, cover_settled, n(53));
        assert!(s6 > Fixed::ZERO && n(53) + s6 <= n(60));
        run.assert_fatigue(2, 6, LandFatigueChange::Strained, n(53) + s6);
        // Unit 7 is above the limit: strain adds nothing and it does not
        // rest. It stays where it was, with no event.
        assert_eq!(strain(g, cover_settled, n(65)), Fixed::ZERO);
        assert_eq!(run.fatigue(2, 7), n(65));
        assert!(run.fatigue_events(2, 7).is_empty());
    } else {
        run.assert_fatigue(2, 6, LandFatigueChange::Rested, n(48));
        run.assert_fatigue(2, 7, LandFatigueChange::Rested, n(60));
    }
    for day in 3..=6 {
        for raw in [6u32, 7] {
            let before = run.fatigue(day - 1, raw);
            run.assert_fatigue(day, raw, LandFatigueChange::Rested, rest(before));
        }
    }
    // The cover the side sees at the end of each day.
    for day in 1..=6usize {
        for raw in [6u32, 7] {
            let want = run
                .defences(day as u32, 6)
                .max(run.fieldworks(day as u32, raw));
            assert_eq!(run.allied_cover[day][&raw], Some(want));
        }
    }
}

// ---------------------------------------------------------------------------
// Properties.

fn seeded(seed: u32) -> Run {
    let source = common::replace_once(&text(), "seed = 1942", &format!("seed = {seed}"));
    Run::of(from(&source))
}

/// The value of a named factor of an event's breakdown.
fn factor(event: &Event, name: &str) -> Option<Fixed> {
    event
        .breakdown
        .as_ref()?
        .factors
        .iter()
        .find(|f| f.name == name)
        .map(|f| f.value)
}

#[test]
fn property_cover_never_raises_a_loss() {
    let mut checked = 0;
    let mut saved = 0;
    for seed in 1..=20 {
        let run = seeded(seed);
        for event in &run.events {
            let lost = match event.kind {
                EventKind::MenLostToFire { lost, .. } | EventKind::MenLostInFight { lost, .. } => {
                    lost
                }
                _ => continue,
            };
            if let Some(before) = factor(event, "men_before_cover") {
                checked += 1;
                assert!(n(i64::from(lost)) <= before, "seed {seed}: {event:?}");
                assert!(event.breakdown.as_ref().expect("a breakdown").result <= before);
                if n(i64::from(lost)) < before {
                    saved += 1;
                }
            }
        }
    }
    assert!(checked > 0, "no loss named men_before_cover");
    assert!(saved > 0, "cover saved nobody in any run");
    eprintln!("cover: {checked} losses checked, {saved} cut");
}

#[test]
fn property_fieldworks_and_defences_stay_in_their_ranges() {
    for seed in 1..=20 {
        let run = seeded(seed);
        for (day, truth) in run.days.iter().enumerate() {
            for u in truth.land_units.values() {
                assert!(
                    u.fieldworks >= Fixed::ZERO && u.fieldworks <= n(30),
                    "seed {seed} day {day}: {:?}",
                    u.fieldworks
                );
            }
            for b in truth.bases.values() {
                assert!(
                    b.defences >= Fixed::ZERO && b.defences <= n(100),
                    "seed {seed} day {day}: {:?}",
                    b.defences
                );
            }
        }
    }
}

/// Whether an event is one the rule adds: fieldworks, wear or strain.
fn is_cover_event(kind: &EventKind) -> bool {
    matches!(
        kind,
        EventKind::FieldworksChanged { .. }
            | EventKind::DefencesWorn { .. }
            | EventKind::LandUnitFatigue {
                change: LandFatigueChange::Strained,
                ..
            }
    )
}

#[test]
fn property_on_the_frozen_figures_the_rule_does_nothing() {
    let mut scenario = common::load(&path()).expect("loads on the frozen figures");
    let mut days = vec![scenario.world.umpire().clone()];
    while scenario.next_day().expect("a day").is_some() {
        days.push(scenario.world.umpire().clone());
    }
    let events: Vec<Event> = scenario.world.events().iter().cloned().collect();
    assert!(events.iter().all(|e| !is_cover_event(&e.kind)));
    // The old rule: work over 200 a day, to 100.
    let (mut b2, mut b3) = (n(30), n(59));
    #[allow(clippy::needless_range_loop)]
    for day in 1..=6usize {
        b2 = (b2 + n(1800) / n(200)).min(n(100));
        b3 = (b3 + n(1500) / n(200)).min(n(100));
        assert_eq!(days[day].bases[&base(2)].defences, b2, "base 2 day {day}");
        assert_eq!(days[day].bases[&base(3)].defences, b3, "base 3 day {day}");
        for u in days[day].land_units.values() {
            assert_eq!(u.fieldworks, Fixed::ZERO);
        }
    }
    assert_eq!(days[1].bases[&base(2)].defences, n(39));
    assert_eq!(days[6].bases[&base(3)].defences, n(100));
    // Nothing is saved: the men lost at A and F are 10 a hit.
    let hits_a = events
        .iter()
        .filter_map(|e| match e.kind {
            EventKind::ShoreBombardment {
                hex, hits_on_men, ..
            } if hex == A => Some(hits_on_men),
            _ => None,
        })
        .sum::<u32>();
    let lost_a: u32 = events
        .iter()
        .filter_map(|e| match e.kind {
            EventKind::MenLostToFire { hex, lost, .. } if hex == A => Some(lost),
            _ => None,
        })
        .sum();
    assert_eq!(lost_a, (10 * hits_a).min(1000));
    // With `strain_per_hit` at 0 the rule is off: a unit under fire rests as
    // it always did (the architect's ruling, Nov 14, 2026). Unit 6 rests on
    // day 1 and again on day 2, the day it is shelled.
    assert_eq!(days[1].land_units[&unit(6)].fatigue, n(53));
    assert_eq!(days[2].land_units[&unit(6)].fatigue, n(48));
}

#[test]
fn property_every_scenario_on_the_frozen_figures_has_no_event_of_the_rule() {
    let mut count = 0;
    let mut files: Vec<PathBuf> = std::fs::read_dir(common::data_dir().join("scenarios"))
        .expect("the scenarios folder reads")
        .map(|entry| entry.expect("an entry").path())
        .filter(|p| p.extension().is_some_and(|e| e == "toml"))
        .collect();
    files.sort();
    for file in files {
        let mut scenario = common::load(&file)
            .unwrap_or_else(|e| panic!("{} loads on the frozen figures: {e:?}", file.display()));
        scenario
            .run_to_end()
            .unwrap_or_else(|e| panic!("{} runs: {e:?}", file.display()));
        let bad: Vec<_> = scenario
            .world
            .events()
            .iter()
            .filter(|e| is_cover_event(&e.kind))
            .collect();
        assert!(bad.is_empty(), "{}: {bad:?}", file.display());
        count += 1;
    }
    assert!(count >= 50, "only {count} scenarios");
}

#[test]
fn property_crossing_from_the_improvised_grade_to_the_prepared_one() {
    let source = common::replace_once(&text(), "defences = 30", "defences = 29");
    let source = common::replace_once(&source, "men = 1800", "men = 800");
    let run = Run::of(from(&source));
    // 800 of work at 29: 200 for the point to 30, then 600 at 600 a point.
    assert_eq!(run.defences(0, 2), n(29));
    assert_eq!(run.defences(1, 2), n(31));
    assert_eq!(run.worked_total(1, 2), vec![n(31)]);
    // Each later day is 800 / 600 = 1.333333 in the prepared grade.
    let mut want = n(31);
    for day in 2..=6 {
        want += n(800) / n(600);
        assert_eq!(run.defences(day, 2), want, "day {day}");
    }
}

/// The text of a side's view at the end of each day: its events, contacts,
/// claims, what it holds of the ground, and every question it can ask with the
/// answer.
fn views(source: &str, side: Side) -> Vec<String> {
    let mut scenario = from(source);
    let mut out = Vec::new();
    while scenario.next_day().expect("a day").is_some() {
        let view = scenario.world.side_view(side);
        let mut text = String::new();
        for e in view.events() {
            text += &format!(
                "{} D{} {:?} {:?} {:?} {:?} {} {:?}\n",
                e.number,
                e.day,
                e.pulse,
                e.kind,
                e.order_number,
                e.causes,
                e.hidden_causes,
                e.breakdown
            );
        }
        for c in view.contacts() {
            text += &format!("{c:?}\n");
        }
        for c in view.claims() {
            text += &format!("{}\n", c.text(side));
        }
        for g in view.ground_estimates() {
            text += &format!("estimate {:?} {} {}\n", g.hex, g.men, g.day);
        }
        for b in view.enemy_bases() {
            text += &format!("{} {:?} {:?} {:?}\n", b.id.raw(), b.name, b.hex, b.side);
        }
        for b in view.bases() {
            text += &format!("{b:?}\n");
        }
        for u in view.land_units() {
            text += &format!("{u:?}\n");
        }
        for q in view.questions() {
            text += &format!("{q:?} -> {:?}\n", view.ask(&q));
        }
        out.push(text);
    }
    out
}

#[test]
fn property_the_enemys_view_is_the_same_whatever_the_other_sides_cover() {
    // World one: as written. World two: base 2's defences 30 -> 50, base 3's
    // 59 -> 40, and unit 4 given fieldworks of 30 to start. None of it
    // changes a hit or a man lost: unit 4's cover at D is base 4's 80 in both,
    // and the shells and the fight draw as they did.
    let one = text();
    let two = common::replace_once(&one, "defences = 59", "defences = 40");
    let two = common::replace_once(&two, "defences = 30", "defences = 50");
    let two = common::replace_once(
        &two,
        "name = \"Placeholder Allied battalion at D\"\nside = \"allied\"\nmen = 1000\nq = 30\nr = 0\n",
        "name = \"Placeholder Allied battalion at D\"\nside = \"allied\"\nmen = 1000\nq = 30\nr = 0\nfieldworks = 30\n",
    );
    let (a, b) = (Run::of(from(&one)), Run::of(from(&two)));
    // The worlds differ, in what the Allied side keeps.
    assert_ne!(a.days, b.days);
    assert_ne!(a.defences(1, 2), b.defences(1, 2));
    assert_ne!(a.defences(1, 3), b.defences(1, 3));
    assert_eq!(b.fieldworks(0, 4), n(30));
    assert_eq!(a.fieldworks(0, 4), Fixed::ZERO);
    // The hits and the men lost are the same each day.
    for day in 0..=6usize {
        let men = |run: &Run| {
            run.days[day]
                .land_units
                .values()
                .map(|u| (u.id, u.men_ashore, u.fatigue))
                .collect::<Vec<_>>()
        };
        assert_eq!(men(&a), men(&b), "day {day}");
    }
    let hits = |run: &Run| {
        run.all(|k| {
            matches!(
                k,
                EventKind::GroundFire { .. }
                    | EventKind::ShoreBombardment { .. }
                    | EventKind::MenLostToFire { .. }
                    | EventKind::MenLostInFight { .. }
            )
        })
        .iter()
        .map(|e| format!("{:?}", e.kind))
        .collect::<Vec<_>>()
    };
    assert_eq!(hits(&a), hits(&b));

    // The Japanese side's view is the same, each day.
    let seen_a = views(&one, Side::Japanese);
    let seen_b = views(&two, Side::Japanese);
    assert_eq!(seen_a.len(), 6);
    assert_eq!(seen_a, seen_b);
    // The Allied side's is not.
    assert_ne!(views(&one, Side::Allied), views(&two, Side::Allied));

    // What each side is given of fieldworks, wear and fatigue.
    let mut scenario = from(&one);
    scenario.run_to_end().expect("runs");
    let truth = scenario.world.umpire().clone();
    let log: Vec<Event> = scenario.world.events().iter().cloned().collect();
    let side_of = |raw: LandUnitId| truth.land_units[&raw].side;
    for side in [Side::Allied, Side::Japanese] {
        let view = scenario.world.side_view(side);
        let given = view.events();
        // The fieldworks given are the side's own units', whole.
        let given_fieldworks: Vec<&EventKind> = given
            .iter()
            .filter_map(|e| match &e.kind {
                SideEventKind::Whole(k @ EventKind::FieldworksChanged { .. }) => Some(k),
                _ => None,
            })
            .collect();
        let want_fieldworks: Vec<&EventKind> = log
            .iter()
            .map(|e| &e.kind)
            .filter(|k| matches!(k, EventKind::FieldworksChanged { unit, .. } if side_of(*unit) == side))
            .collect();
        assert_eq!(given_fieldworks, want_fieldworks, "{side:?}");
        let given_worn: Vec<&EventKind> = given
            .iter()
            .filter_map(|e| match &e.kind {
                SideEventKind::Whole(k @ EventKind::DefencesWorn { .. }) => Some(k),
                _ => None,
            })
            .collect();
        let want_worn: Vec<&EventKind> = log
            .iter()
            .map(|e| &e.kind)
            .filter(|k| matches!(k, EventKind::DefencesWorn { side: s, .. } if *s == side))
            .collect();
        assert_eq!(given_worn, want_worn, "{side:?}");
        // No fatigue event of the other side's unit.
        for e in &given {
            if let SideEventKind::Whole(EventKind::LandUnitFatigue { unit, .. }) = &e.kind {
                assert_eq!(side_of(*unit), side, "{side:?} was told {e:?}");
            }
        }
        // It sees the cover of its own units, and none of the enemy's.
        for (id, u) in &truth.land_units {
            if u.side == side {
                assert!(view.cover(*id).is_some());
            } else {
                assert_eq!(view.cover(*id), None);
            }
        }
    }
    // The Allied side was given some of each; the Japanese none of the
    // Allied side's.
    assert!(!want_nonempty(&log, |k| matches!(k, EventKind::DefencesWorn { .. })).is_empty());
    assert!(scenario
        .world
        .side_view(Side::Japanese)
        .events()
        .iter()
        .all(|e| !matches!(
            &e.kind,
            SideEventKind::Whole(EventKind::DefencesWorn { .. })
        )));
}

fn want_nonempty(log: &[Event], pick: impl Fn(&EventKind) -> bool) -> Vec<&Event> {
    log.iter().filter(|e| pick(&e.kind)).collect()
}
