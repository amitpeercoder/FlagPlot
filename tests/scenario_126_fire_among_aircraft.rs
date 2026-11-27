//! Scenario 126: fire among a carrier's aircraft. A carrier's bombers
//! strike a carrier that has her own bombers aboard. A raid whose bombs
//! hit may set them alight; the fire is fought from the next pulse, rises
//! or falls a level at each pulse's end, and loses her if it rises above
//! the top.
//!
//! Proves decision 0212 as amended and measured (its sections of Nov 22,
//! 2026, which govern), with a figure by side from decision 0213. The data
//! is placeholder data. It stands on scenario 5b's setting and hand
//! calculation for the search, the decision to strike and the bomb hits.
//!
//! # Hand calculation
//!
//! Written by the architect before the code. Everything stated as a
//! number is asserted exactly, in the game's own arithmetic. If the code
//! disagrees, the code or the rule is wrong: do not change a number or a
//! statement here to make a test pass. A mismatch goes back to the
//! architect.
//!
//! ## The setting
//!
//! Scenario 5b's, with these differences. The game is 3 days. Carrier B,
//! ship 3, Japanese, has aboard air group 3, 20 placeholder dive bombers,
//! Japanese, on the rest mission, and air group 4, 10 placeholder
//! fighters, on the rest mission: neither flies. The figures are the live
//! figures' shape with these values of the scenario's own (a copy of the
//! air combat figures, as scenario files of its kind have):
//! `aircraft_fire_chance` Japanese 0.6, Allied 0.15;
//! `fire_spreads_chance` 0.5; `aircraft_fire_levels` 3.
//!
//! ## The share
//!
//! Carrier B's strike aircraft are the 20 dive bombers of group 3: the
//! fighters are not counted. None flies in the pulse. Aboard 20 of 20: the
//! share is 1.
//!
//! ## A raid's chance
//!
//! For a raid that puts b bomb hits on carrier B and does not sink her,
//! the chance is 1 - (1 - 0.6 x 1)^b:
//!
//! - 1 hit: 0.6
//! - 2 hits: 1 - 0.4^2 = 0.84
//! - 3 hits: 1 - 0.4^3 = 0.936
//! - 4 hits: 1 - 0.4^4 = 0.9744
//!
//! A hit counts whichever track of damage it landed on. A raid with no
//! hit makes no draw and records nothing.
//!
//! The record of the fire set (for every raid that drew, whether or not
//! it set one) gives: the bombs that hit, 20 bombers aboard, 0 flying,
//! the share 1, the chance above for its hits, the draw, and her fire
//! among aircraft after. It cites the raid's attacks on her.
//!
//! ## What the draw does
//!
//! Over the seeds the test runs, in every pulse in which a raid hit
//! carrier B: where the draw recorded is under the chance, her fire among
//! aircraft is 3 after it, and the pulse it was set is that pulse; where
//! it is not, her fire is what it was. Both happen on some seed.
//!
//! ## The fire fought
//!
//! At the end of the pulse in which a fire was set or raised to the top
//! by a raid: no draw, and no record of a rise or fall.
//!
//! At the end of each pulse after it, while she is afloat and her fire is
//! 1 or more: one draw. Under 0.5 it rises a level; otherwise it falls
//! one. The record gives the level before, the draw, and the level after.
//!
//! - From 3, a rise is above the top: she is lost. `ShipSunk` is recorded
//!   with fire as its cause, citing the fire's record, and with her go her
//!   30 aircraft and her men as a sinking takes them.
//! - From 3, a fall leaves 2; from 2, a rise 3 and a fall 1; from 1, a
//!   rise 2 and a fall nothing: the fire is out, no draw is made after it,
//!   and she flies again.
//! - The night's end is a pulse's end like the others.
//!
//! The chances this comes to, stated and not asserted on seeds: a fire set
//! at the top is lost at the end of the next pulse one time in two, and in
//! the end three times in four.
//!
//! ## A carrier with a fire among her aircraft
//!
//! While her fire is 1 or more she flies nothing off and lands nothing on:
//! her groups launch nothing whatever their mission, and aircraft of hers
//! in the air when it is set look for another deck or a base as when a
//! deck is out. Her flight deck's own state is not changed by the fire,
//! her fire damage is not changed by it, she is not counted as crippled
//! for it, and she does not "look lost" for it (decision 0211).
//!
//! ## Bombed again
//!
//! A raid that sets a fire on a carrier already burning puts it at 3
//! again, and that pulse is the one it was last set in: no draw at its
//! end.
//!
//! # Properties
//!
//! - **Half away.** With group 3 at 20 dive bombers of which 10 fly in the
//!   pulse (the test arranges a launch that takes 10): the share is 0.5,
//!   and the chances are 0.3 for 1 hit, 0.51 for 2, 0.657 for 3.
//! - **All away, and none.** With all 20 flying the share is nothing, the
//!   chance is nothing, and no draw is made. With no bomber in her groups
//!   (group 3 removed, the fighters kept) the same.
//! - **The other navy.** With the sides exchanged, so that the carrier
//!   bombed is Allied with 20 bombers aboard: 0.15 for 1 hit, 0.2775 for
//!   2, 0.385875 for 3; with 10 of 20 flying, 0.075, 0.144375 and
//!   0.208547: 1 - 0.925^3 = 0.208546875, which the game holds to six
//!   places as 0.208547. (Restated by the architect on Nov 22, 2026.)
//! - **Not a carrier.** A cruiser of task force 2 with a floatplane group
//!   aboard, bombed: no draw, no record, nothing of this rule.
//! - **A torpedo and a shell set no such fire.** With the attackers
//!   torpedo bombers that hit with torpedoes: no draw.
//! - **Fire damage is as before.** Every ship's fire damage on every seed
//!   is what the build without this rule gives, on every pulse up to the
//!   first at which a carrier is lost to a fire among aircraft or closed
//!   by one.
//! - **Sunk by the raid.** A raid whose hits sink her makes no draw.
//! - **Off where the figures are absent.** On the frozen figures, which
//!   have none of the four, every event of scenario 5b is byte for byte
//!   what it was.
//! - **By side.** With no value by side given, both sides read the plain
//!   value; a side's view gives its own value in both places and not the
//!   enemy's (decision 0213).
//! - The same seed gives the same game; the hash holds her fire among
//!   aircraft and the pulse it was set, and a save and replay agree.

use crate::common;

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::PathBuf;
use std::sync::OnceLock;

use sim_core::events::{Cause, DamageTrack, Event, EventKind, Weapon};
use sim_core::fixed::Fixed;
use sim_core::ids::{AirGroupId, EventId, ShipId};
use sim_core::refdata::BySide;
use sim_core::state::{ShipLocation, Side};
use sim_core::time::Pulse;
use sim_data::{LoadOptions, Scenario};

const CARRIER_B: ShipId = ShipId::new(3);
const SEEDS: u64 = 1000;

fn scenario_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../data/scenarios/126_fire_among_aircraft.toml")
}

fn text() -> String {
    fs::read_to_string(scenario_path()).expect("read the scenario")
}

/// A figure in millionths.
fn m(raw: i64) -> Fixed {
    Fixed::from_raw(raw)
}

// ---------------------------------------------------------------------
// The figures. The frozen air combat figures have none of the four keys.
// A test that wants them builds the frozen figures plus the four keys and,
// where the hand calculation says so, the two figures by side: a copy of
// the frozen file with the keys put at its head and the entries at its
// foot, in a folder of its own. Nothing in the data folder is touched.
// ---------------------------------------------------------------------

#[derive(Clone, Copy, PartialEq, Eq)]
enum Figures {
    /// The frozen figures as they are: none of the four.
    Frozen,
    /// The four, with `aircraft_fire_chance` 0.3 plain and 0.6 Japanese and
    /// 0.15 Allied by side.
    Fire,
    /// The four, with the plain 0.3 and nothing by side.
    FireNoBySide,
}

const FIRE_KEYS: &str =
    "aircraft_fire_chance = \"0.3\"\nfire_spreads_chance = \"0.5\"\naircraft_fire_levels = 3\n\n";

const BY_SIDE: &str = "
[[by_side]]
figure = \"aircraft_fire_chance\"
side = \"japanese\"
value = \"0.6\"
rests_on = \"Placeholder of scenario 126's hand calculation: a judgement, as the live file's.\"

[[by_side]]
figure = \"aircraft_fire_chance\"
side = \"allied\"
value = \"0.15\"
rests_on = \"Placeholder of scenario 126's hand calculation: a judgement, as the live file's.\"
";

/// A change to a figures file of the copy a variant reads.
#[derive(Clone, Copy)]
enum Alter {
    /// Add a line to the end of the file.
    Append(&'static str, &'static str),
    /// Replace a text that appears once in the file.
    Replace(&'static str, &'static str, &'static str),
}

/// A placeholder torpedo bomber of the dive bomber's reach, for the variants
/// that want a torpedo to hit with.
const TORPEDO_BOMBER: Alter = Alter::Append(
    "aircraft_types.csv",
    "5,Placeholder torpedo bomber,bomber,0,0,250,torpedo,60,0,0\n",
);

/// The placeholder cruiser (class 1) with room for four floatplanes aboard.
const CRUISER_WITH_FLOATPLANES: Alter = Alter::Replace(
    "ship_classes.csv",
    "1,Placeholder cruiser,cruiser,16,32,2000,8000,0,8,3,100,20,8,8,200,400,0,0,0",
    "1,Placeholder cruiser,cruiser,16,32,2000,8000,0,8,3,100,20,8,8,200,400,0,4,0",
);

fn options(figures: Figures, alters: &[Alter]) -> LoadOptions {
    if figures == Figures::Frozen && alters.is_empty() {
        return common::frozen();
    }
    common::altered_reference("fire", |dir| {
        if figures != Figures::Frozen {
            common::edit_file(dir, "air_combat.toml", |text| {
                let by_side = if figures == Figures::Fire {
                    BY_SIDE
                } else {
                    ""
                };
                format!("{FIRE_KEYS}{text}\n{by_side}")
            });
        }
        for alter in alters {
            match *alter {
                Alter::Append(file, line) => common::edit_file(dir, file, |mut text| {
                    if !text.ends_with('\n') {
                        text.push('\n');
                    }
                    text.push_str(line);
                    text
                }),
                Alter::Replace(file, from, to) => {
                    common::edit_file(dir, file, |text| common::replace_once(&text, from, to))
                }
            }
        }
    })
}

/// The scenario with `change` made to its text, on the figures asked for
/// and with the figures files altered as asked.
fn build(figures: Figures, alters: &[Alter], change: impl FnOnce(String) -> String) -> Scenario {
    let text = change(text());
    common::from_text_with(&text, &scenario_path(), &options(figures, alters))
        .expect("the variant loads")
}

fn base() -> Scenario {
    build(Figures::Fire, &[], |text| text)
}

/// Exchanges the two sides throughout a scenario's text.
fn exchange_sides(text: String) -> String {
    assert!(text.contains("\"allied\"") && text.contains("\"japanese\""));
    text.replace("\"allied\"", "\"@\"")
        .replace("\"japanese\"", "\"allied\"")
        .replace("\"@\"", "\"japanese\"")
}

// ---------------------------------------------------------------------
// What one run did.
// ---------------------------------------------------------------------

struct Run {
    seed: u64,
    days: u32,
    events: Vec<Event>,
    hashes: Vec<u64>,
    /// Ship 3's end state: sunk, her fire among aircraft, her fire damage.
    b_sunk: bool,
    b_fire: Option<sim_core::state::AircraftFire>,
    b_fire_damage: Fixed,
    /// The ready aircraft of each group at the end.
    ready: BTreeMap<u32, (u32, u32)>,
}

#[derive(Clone, Copy, Debug)]
struct Roll {
    id: EventId,
    day: u32,
    pulse: Pulse,
    bombs: u32,
    aboard: u32,
    flying: u32,
    share: Fixed,
    chance: Fixed,
    draw: Fixed,
    set: bool,
}

impl Run {
    fn rolls_of(&self, ship: ShipId) -> Vec<Roll> {
        self.events
            .iter()
            .filter_map(|event| match event.kind {
                EventKind::AircraftFireRoll {
                    ship: rolled,
                    bombs,
                    aboard,
                    flying,
                    share,
                    chance,
                    draw,
                    set,
                    ..
                } if rolled == ship => Some(Roll {
                    id: event.id,
                    day: event.day,
                    pulse: event.pulse.expect("a pulse"),
                    bombs,
                    aboard,
                    flying,
                    share,
                    chance,
                    draw,
                    set,
                }),
                _ => None,
            })
            .collect()
    }

    fn rolls(&self) -> Vec<Roll> {
        self.rolls_of(CARRIER_B)
    }

    fn any_fire_record(&self) -> bool {
        self.events.iter().any(|event| {
            matches!(
                event.kind,
                EventKind::AircraftFireRoll { .. } | EventKind::AircraftFireFought { .. }
            )
        })
    }

    fn event(&self, id: EventId) -> &Event {
        self.events
            .iter()
            .find(|event| event.id == id)
            .expect("the event exists")
    }

    /// The bomb hits of the attacks on `ship` in a pulse, with their ids.
    fn attacks_on(&self, ship: ShipId, at: (u32, Pulse)) -> Vec<(EventId, u32)> {
        self.events
            .iter()
            .filter(|event| self.at_opt(event) == Some(at))
            .filter_map(|event| match event.kind {
                EventKind::AirAttack {
                    ship: hit,
                    weapon: Weapon::Bomb,
                    hits,
                    ..
                } if hit == ship => Some((event.id, hits)),
                _ => None,
            })
            .collect()
    }

    fn at_opt(&self, event: &Event) -> Option<(u32, Pulse)> {
        event.pulse.map(|pulse| (event.day, pulse))
    }

    /// The ships sunk in a pulse, with the event that caused each.
    fn sunk_in(&self, at: (u32, Pulse)) -> Vec<(ShipId, Vec<Cause>)> {
        self.events
            .iter()
            .filter(|event| self.at_opt(event) == Some(at))
            .filter_map(|event| match event.kind {
                EventKind::ShipSunk { ship, .. } => Some((ship, event.causes.clone())),
                _ => None,
            })
            .collect()
    }

    /// The aircraft an air group launched (a strike) or flew (a search) in
    /// a pulse.
    fn flew(&self, group: u32, at: (u32, Pulse)) -> u32 {
        self.events
            .iter()
            .filter(|event| self.at_opt(event) == Some(at))
            .map(|event| match event.kind {
                EventKind::StrikeLaunched {
                    air_group,
                    aircraft,
                    ..
                }
                | EventKind::SearchFlown {
                    air_group,
                    aircraft,
                    ..
                } if air_group.raw() == group => aircraft,
                _ => 0,
            })
            .sum()
    }
}

fn run_of(scenario: &Scenario, seed: u64) -> Run {
    let mut run = scenario.with_seed(seed).expect("a seed can be set");
    run.run_to_end().expect("the run resolves");
    let truth = run.world.umpire();
    let ship = &truth.ships[&CARRIER_B];
    Run {
        seed,
        days: run.days,
        events: run.world.events().iter().cloned().collect(),
        hashes: run.world.day_hashes().to_vec(),
        b_sunk: ship.location == ShipLocation::Sunk,
        b_fire: ship.aircraft_fire,
        b_fire_damage: ship.fire_damage,
        ready: truth
            .air_groups
            .values()
            .map(|group| (group.id.raw(), (group.ready, group.damaged)))
            .collect(),
    }
}

fn batch_of(scenario: &Scenario, seeds: u64) -> Vec<Run> {
    (1..=seeds).map(|seed| run_of(scenario, seed)).collect()
}

fn batch() -> &'static Vec<Run> {
    static BATCH: OnceLock<Vec<Run>> = OnceLock::new();
    BATCH.get_or_init(|| batch_of(&base(), SEEDS))
}

/// The chance of a fire for `bombs` hits where one bomb sets it with
/// `one`: one less the chance every bomb fails, each product cut to
/// millionths in turn, as the game computes it.
fn chance_for(one: Fixed, bombs: u32) -> Fixed {
    let mut fails = Fixed::ONE;
    for _ in 0..bombs {
        fails = fails * (Fixed::ONE - one);
    }
    Fixed::ONE - fails
}

// ---------------------------------------------------------------------
// What a fire's history must be, walked pulse by pulse.
// ---------------------------------------------------------------------

#[derive(Default, Debug)]
struct History {
    /// Rolls that set a fire, and rolls that drew and did not.
    sets: u32,
    not_set: u32,
    /// A fire set on a carrier already burning.
    bombed_again: u32,
    /// Transitions fought: (level before, level after), with 4 meaning lost.
    steps: BTreeSet<(u8, u8)>,
    lost: u32,
    out: u32,
    /// The first pulse at which she was lost to a fire, if she was.
    lost_at: Option<(u32, Pulse)>,
}

/// Walks the fire of carrier B through a run, asserting every statement of
/// "What the draw does" and "The fire fought" as it goes.
fn walk(run: &Run) -> History {
    let mut history = History::default();
    let mut level: u8 = 0;
    let mut set_at: Option<(u32, Pulse)> = None;
    let mut last_record: Option<EventId> = None;
    let mut afloat = true;
    for day in 1..=run.days {
        for pulse in Pulse::ALL {
            let here = (day, pulse);
            let which = |event: &&Event| run.at_opt(event) == Some(here);
            // A raid's draws are in the air combat step: before the fighting.
            let rolls: Vec<&Event> = run
                .events
                .iter()
                .filter(which)
                .filter(|e| {
                    matches!(e.kind, EventKind::AircraftFireRoll { ship, .. } if ship == CARRIER_B)
                })
                .collect();
            let fought: Vec<&Event> = run
                .events
                .iter()
                .filter(which)
                .filter(|e| {
                    matches!(e.kind, EventKind::AircraftFireFought { ship, .. } if ship == CARRIER_B)
                })
                .collect();
            assert!(rolls.len() <= 1, "seed {}: one raid at a time", run.seed);
            for event in &rolls {
                assert!(afloat, "seed {}: no draw for a ship that is gone", run.seed);
                let EventKind::AircraftFireRoll {
                    set, level_after, ..
                } = event.kind
                else {
                    unreachable!()
                };
                if set {
                    assert_eq!(level_after, 3, "seed {}", run.seed);
                    history.sets += 1;
                    if level > 0 {
                        history.bombed_again += 1;
                    }
                    level = 3;
                    set_at = Some(here);
                    last_record = Some(event.id);
                } else {
                    // Her fire is what it was.
                    assert_eq!(level_after, level, "seed {}", run.seed);
                    history.not_set += 1;
                }
            }
            // The ship sunk by a raid in this pulse (not by the fire) takes
            // her fire with her.
            let sunk_by_raid = run
                .sunk_in(here)
                .iter()
                .any(|(ship, causes)| *ship == CARRIER_B && !causes_are_fire(run, causes));
            if sunk_by_raid {
                afloat = false;
                level = 0;
            }
            let expect_one = afloat && level >= 1 && set_at != Some(here);
            if !expect_one {
                assert!(
                    fought.is_empty(),
                    "seed {} {here:?}: no fire to fight (level {level}, set {set_at:?})",
                    run.seed
                );
                continue;
            }
            assert_eq!(fought.len(), 1, "seed {} {here:?}", run.seed);
            let event = fought[0];
            let EventKind::AircraftFireFought {
                level_before,
                draw,
                level_after,
                lost,
                ..
            } = event.kind
            else {
                unreachable!()
            };
            assert_eq!(level_before, level, "seed {}", run.seed);
            let rises = draw < m(500_000);
            assert_eq!(
                level_after,
                if rises { level + 1 } else { level - 1 },
                "seed {}",
                run.seed
            );
            assert_eq!(lost, rises && level == 3, "seed {}", run.seed);
            // It cites the fire's record before it.
            assert_eq!(
                event.causes,
                vec![Cause::Event(last_record.expect("a fire was set"))],
                "seed {}",
                run.seed
            );
            // It is after the raid's draw of the pulse, if any, in the log.
            if let Some(roll) = rolls.first() {
                assert!(roll.id < event.id);
            }
            history.steps.insert((level_before, level_after));
            last_record = Some(event.id);
            if lost {
                history.lost += 1;
                history.lost_at = Some(here);
                afloat = false;
                level = 0;
                // ShipSunk, with the fire's record as its cause.
                let sunk: Vec<_> = run
                    .sunk_in(here)
                    .into_iter()
                    .filter(|(ship, _)| *ship == CARRIER_B)
                    .collect();
                assert_eq!(sunk.len(), 1, "seed {}", run.seed);
                assert_eq!(sunk[0].1, vec![Cause::Event(event.id)], "seed {}", run.seed);
            } else if level_after == 0 {
                history.out += 1;
                level = 0;
            } else {
                level = level_after;
            }
        }
    }
    // The end state agrees with the history.
    assert_eq!(run.b_sunk, !afloat, "seed {}", run.seed);
    match (level, run.b_fire) {
        (0, None) => {}
        (level, Some(fire)) => {
            assert!(level >= 1);
            assert_eq!(fire.level, level, "seed {}", run.seed);
            let set = set_at.expect("it was set");
            assert_eq!((fire.set_day, fire.set_pulse), set, "seed {}", run.seed);
            assert_eq!(Some(fire.record), last_record, "seed {}", run.seed);
        }
        (level, fire) => panic!("seed {}: level {level} but state {fire:?}", run.seed),
    }
    history
}

/// Whether a `ShipSunk`'s causes are a fire among aircraft's record.
fn causes_are_fire(run: &Run, causes: &[Cause]) -> bool {
    causes.iter().any(|cause| match cause {
        Cause::Event(id) => matches!(run.event(*id).kind, EventKind::AircraftFireFought { .. }),
        Cause::Order(_) => false,
    })
}

// ---------------------------------------------------------------------
// The setting.
// ---------------------------------------------------------------------

#[test]
fn the_setting_is_as_the_hand_calculation_says() {
    let scenario = base();
    assert_eq!(scenario.days, 3);
    let truth = scenario.world.umpire();
    let carrier_b = &truth.ships[&CARRIER_B];
    assert_eq!(carrier_b.side, Side::Japanese);
    let group = |id: u32| &truth.air_groups[&AirGroupId::new(id)];
    assert_eq!(
        group(3).location,
        sim_core::state::AirGroupLocation::Ship(CARRIER_B)
    );
    assert_eq!(group(3).side, Side::Japanese);
    assert_eq!((group(3).aircraft_type.raw(), group(3).ready), (3, 20));
    assert_eq!(
        group(4).location,
        sim_core::state::AirGroupLocation::Ship(CARRIER_B)
    );
    assert_eq!((group(4).aircraft_type.raw(), group(4).ready), (4, 10));
    // The rest mission: neither flies.
    assert_eq!(group(3).mission, sim_core::orders::AirMission::Rest);
    assert_eq!(group(4).mission, sim_core::orders::AirMission::Rest);
    // The figures.
    let air = scenario.world.ref_data().air_combat.as_ref().expect("held");
    let fire = air.aircraft_fire.expect("the figures hold the four");
    assert_eq!(fire.chance, BySide::new(m(150_000), m(600_000)));
    assert_eq!(fire.chance.of(Side::Japanese), m(600_000));
    assert_eq!(fire.chance.of(Side::Allied), m(150_000));
    assert_eq!(fire.spreads_chance, m(500_000));
    assert_eq!(fire.levels, 3);
    assert!(!air.strike_is_spread);
    // No fire at the start.
    assert!(truth
        .ships
        .values()
        .all(|ship| ship.aircraft_fire.is_none()));
}

// ---------------------------------------------------------------------
// The share, and a raid's chance.
// ---------------------------------------------------------------------

#[test]
fn carrier_b_has_a_share_of_1_and_a_raid_that_hits_draws_with_the_chance_for_its_hits() {
    let mut seen_hits: BTreeSet<u32> = BTreeSet::new();
    let mut raids_that_hit = 0;
    for run in batch() {
        let rolls = run.rolls();
        // Every raid that put a bomb on her and did not sink her drew once,
        // and a raid with no hit made no draw and recorded nothing.
        let mut pulses_with_a_roll = 0;
        for day in 1..=run.days {
            for pulse in Pulse::ALL {
                let here = (day, pulse);
                let attacks = run.attacks_on(CARRIER_B, here);
                let hits: u32 = attacks.iter().map(|(_, hits)| hits).sum();
                let sunk = run.sunk_in(here).iter().any(|(ship, _)| *ship == CARRIER_B)
                    && !run
                        .sunk_in(here)
                        .iter()
                        .any(|(_, causes)| causes_are_fire(run, causes));
                let rolled: Vec<&Roll> = rolls
                    .iter()
                    .filter(|roll| (roll.day, roll.pulse) == here)
                    .collect();
                if hits > 0 && !sunk {
                    assert_eq!(rolled.len(), 1, "seed {} {here:?}", run.seed);
                    let roll = rolled[0];
                    raids_that_hit += 1;
                    pulses_with_a_roll += 1;
                    assert_eq!(roll.bombs, hits);
                    seen_hits.insert(hits);
                    // 20 bombers aboard, none flying: the share is 1.
                    assert_eq!((roll.aboard, roll.flying), (20, 0));
                    assert_eq!(roll.share, Fixed::ONE);
                    // The chance, 1 - (1 - 0.6 x 1)^b.
                    let expected = match hits {
                        1 => m(600_000),
                        2 => m(840_000),
                        3 => m(936_000),
                        4 => m(974_400),
                        more => chance_for(m(600_000), more),
                    };
                    assert_eq!(roll.chance, expected, "seed {} {hits} hits", run.seed);
                    // It cites the raid's attacks on her.
                    let cited: Vec<Cause> = attacks
                        .iter()
                        .filter(|(_, hits)| *hits > 0)
                        .map(|(id, _)| Cause::Event(*id))
                        .collect();
                    let event = run.event(roll.id);
                    assert_eq!(event.causes, cited, "seed {}", run.seed);
                } else {
                    assert!(rolled.is_empty(), "seed {} {here:?}", run.seed);
                }
            }
        }
        assert_eq!(rolls.len(), pulses_with_a_roll);
    }
    assert!(raids_that_hit > 0);
    // The four chances of the hand calculation each came up.
    for hits in 1..=4 {
        assert!(seen_hits.contains(&hits), "{hits} hits never came up");
    }
}

#[test]
fn a_hit_counts_whichever_track_it_landed_on() {
    // Every hit of the raid counts, on flotation or on fire: the roll's
    // bombs are the raid's hits, which the damage events split in two.
    let mut fire_track_seen = false;
    for run in batch() {
        for roll in run.rolls() {
            let attacks = run.attacks_on(CARRIER_B, (roll.day, roll.pulse));
            let hits: u32 = attacks.iter().map(|(_, hits)| hits).sum();
            assert_eq!(roll.bombs, hits);
            for (id, _) in attacks {
                for event in &run.events {
                    if let EventKind::ShipDamaged {
                        track: DamageTrack::Fire,
                        ship,
                        ..
                    } = event.kind
                    {
                        if ship == CARRIER_B && event.causes.contains(&Cause::Event(id)) {
                            fire_track_seen = true;
                        }
                    }
                }
            }
        }
    }
    assert!(
        fire_track_seen,
        "a raid put hits on her fire track on some seed"
    );
}

#[test]
fn the_record_of_a_fire_set_names_its_figures() {
    // The record: the bombs that hit, 20 aboard, 0 flying, the share 1, the
    // chance for its hits, the draw, and the fire after: 3 where set, and
    // what it was where not (checked in the walk).
    for run in batch() {
        for roll in run.rolls() {
            assert_eq!((roll.aboard, roll.flying, roll.share), (20, 0, Fixed::ONE));
            assert!(roll.draw >= Fixed::ZERO && roll.draw < Fixed::ONE);
            assert_eq!(roll.set, roll.draw < roll.chance);
        }
    }
}

// ---------------------------------------------------------------------
// What the draw does, and the fire fought.
// ---------------------------------------------------------------------

#[test]
fn the_draw_sets_her_fire_at_3_or_leaves_it_and_both_happen() {
    let mut sets = 0;
    let mut not_set = 0;
    for run in batch() {
        let history = walk(run);
        sets += history.sets;
        not_set += history.not_set;
    }
    assert!(sets > 0, "a fire is set on some seed");
    assert!(not_set > 0, "a draw fails to set one on some seed");
}

#[test]
fn a_fire_is_fought_from_the_next_pulse_by_one_draw_at_each_pulses_end() {
    let mut steps: BTreeSet<(u8, u8)> = BTreeSet::new();
    let (mut lost, mut out, mut bombed_again) = (0, 0, 0);
    for run in batch() {
        let history = walk(run);
        steps.extend(history.steps);
        lost += history.lost;
        out += history.out;
        bombed_again += history.bombed_again;
    }
    // From 3 a fall leaves 2, and a rise is above the top (shown as 4: lost);
    // from 2 a rise is 3 and a fall 1; from 1 a rise is 2 and a fall nothing.
    let want: BTreeSet<(u8, u8)> = [(3, 2), (3, 4), (2, 3), (2, 1), (1, 2), (1, 0)]
        .into_iter()
        .collect();
    assert_eq!(steps, want);
    assert!(lost > 0, "she is lost to a fire on some seed");
    assert!(out > 0, "a fire goes out on some seed");
    assert!(
        bombed_again > 0,
        "she is bombed again while burning on some seed"
    );
}

#[test]
fn a_loss_takes_her_30_aircraft_with_her() {
    let mut seen = 0;
    for run in batch() {
        let history = walk(run);
        let Some(at) = history.lost_at else { continue };
        seen += 1;
        let gone: Vec<(u32, u32)> = run
            .events
            .iter()
            .filter(|event| run.at_opt(event) == Some(at))
            .filter_map(|event| match event.kind {
                EventKind::AirGroupLost {
                    air_group,
                    ship,
                    aircraft,
                } if ship == CARRIER_B => Some((air_group.raw(), aircraft)),
                _ => None,
            })
            .collect();
        assert_eq!(gone, vec![(3, 20), (4, 10)], "seed {}", run.seed);
        assert_eq!(run.ready[&3], (0, 0));
        assert_eq!(run.ready[&4], (0, 0));
    }
    assert!(seen > 0);
}

#[test]
fn a_fire_that_is_out_is_not_fought_again() {
    // No draw after it: the walk asserts none is made at a pulse with no
    // fire to fight. Here, the pulses after an out fire hold none until a
    // raid sets another.
    let mut out_then_quiet = 0;
    for run in batch() {
        let history = walk(run);
        if history.out > 0 && run.b_fire.is_none() && !run.b_sunk {
            out_then_quiet += 1;
        }
    }
    assert!(out_then_quiet > 0);
}

// ---------------------------------------------------------------------
// A carrier with a fire among her aircraft.
// ---------------------------------------------------------------------

/// Carrier B with two flights of six scouts of her own (the fighters cut to
/// four, so that her 20 bombers, 4 fighters and 12 scouts make 36, her
/// capacity). Group 5 is ordered to search from the first day and group 6
/// from the third: the order of a day's search is theirs to fly if her deck
/// lets them.
fn with_scouts(text: String) -> String {
    let mut text = common::replace_once(&text, "ready = 10", "ready = 4");
    text.push_str(
        "
[[air_group]]
id = 5
name = \"Placeholder scouting squadron B1\"
side = \"japanese\"
ship = 3
aircraft_type = 2
ready = 6

[[air_group]]
id = 6
name = \"Placeholder scouting squadron B2\"
side = \"japanese\"
ship = 3
aircraft_type = 2
ready = 6
",
    );
    for (day, group) in [(1, 5), (3, 6)] {
        text.push_str(&format!(
            "
[[order]]
day = {day}
side = \"japanese\"
air_group = {group}
kind = \"set_air_mission\"
mission = \"search\"

[[order]]
day = {day}
side = \"japanese\"
air_group = {group}
kind = \"set_search_arc\"
from_deg = 220
to_deg = 320
range_nm = 300
"
        ));
    }
    text
}

#[test]
fn while_her_fire_burns_she_flies_nothing_off_and_it_ends_when_it_is_out() {
    let scenario = build(Figures::Fire, &[], with_scouts);
    let runs = batch_of(&scenario, SEEDS);
    let mut flew_free = 0;
    let mut grounded_while_burning = 0;
    let mut flew_after_out = 0;
    let mut came_down_elsewhere = 0;
    for run in &runs {
        let mut level = 0u8;
        let mut was_out = false;
        for day in 1..=run.days {
            for pulse in Pulse::ALL {
                let here = (day, pulse);
                let (one, two) = (run.flew(5, here), run.flew(6, here));
                if level >= 1 {
                    // Her groups launch nothing, whatever their mission.
                    assert_eq!(
                        (one, two),
                        (0, 0),
                        "seed {} {here:?}: she launched with a fire among her aircraft",
                        run.seed
                    );
                    if pulse != Pulse::Night && day == 3 {
                        grounded_while_burning += 1;
                    }
                } else {
                    if one + two > 0 {
                        flew_free += 1;
                    }
                    if two > 0 && was_out {
                        flew_after_out += 1;
                    }
                }
                // The pulse's events change the fire for the next pulse.
                for event in run.events.iter().filter(|e| run.at_opt(e) == Some(here)) {
                    match event.kind {
                        EventKind::AircraftFireRoll {
                            ship, set: true, ..
                        } if ship == CARRIER_B => {
                            level = 3;
                            // Aircraft of hers in the air when it was set
                            // look for another deck or a base, as when a deck
                            // is out: here, there is none, and they come down
                            // in the sea beside her force.
                            if one > 0 {
                                let found_a_place = run.events.iter().any(|e| {
                                    run.at_opt(e) == Some(here)
                                        && e.causes.contains(&Cause::Event(event.id))
                                        && matches!(
                                            e.kind,
                                            EventKind::AircraftDitchedNoDeck { air_group, .. }
                                                if air_group.raw() == 5
                                        )
                                });
                                assert!(
                                    found_a_place,
                                    "seed {} {here:?}: her scouts in the air, no landing recorded for the fire",
                                    run.seed
                                );
                                came_down_elsewhere += 1;
                            }
                        }
                        EventKind::AircraftFireFought {
                            ship,
                            level_after,
                            lost,
                            ..
                        } if ship == CARRIER_B => {
                            level = if lost { 0 } else { level_after };
                            if level_after == 0 {
                                was_out = true;
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
    }
    assert!(flew_free > 0, "her scouts fly while no fire burns");
    assert!(
        grounded_while_burning > 0,
        "a daylight pulse of the third day comes with a fire burning"
    );
    assert!(flew_after_out > 0, "she flies again once a fire is out");
    assert!(
        came_down_elsewhere > 0,
        "scouts were in the air when a fire was set"
    );
}

#[test]
fn a_fire_among_aircraft_leaves_her_deck_her_fire_damage_and_her_standing_alone() {
    // Fire damage, flotation and the systems hit are what the build without
    // the rule gives, on every pulse up to and including the first at which
    // she is lost to a fire among aircraft: the damage events of the same
    // seed on the frozen figures, which have none of the four, are the
    // same, event for event.
    let frozen = build(Figures::Frozen, &[], |text| text);
    let mut compared = 0;
    let mut burned_and_still_afloat = 0;
    for run in batch().iter().take(500) {
        let without = run_of(&frozen, run.seed);
        assert!(!without.any_fire_record());
        let history = walk(run);
        let upto = history.lost_at.unwrap_or((u32::MAX, Pulse::Afternoon));
        let damage = |run: &Run| -> Vec<(u32, Option<Pulse>, EventKind)> {
            run.events
                .iter()
                .filter(|event| {
                    matches!(
                        event.kind,
                        EventKind::ShipDamaged { .. } | EventKind::SystemHit { .. }
                    )
                })
                .filter(|event| (event.day, event.pulse.expect("pulse")) <= upto)
                .map(|event| (event.day, event.pulse, event.kind.clone()))
                .collect()
        };
        assert_eq!(damage(run), damage(&without), "seed {}", run.seed);
        if history.lost_at.is_none() {
            // The whole game: her fire damage at the end is the same.
            assert_eq!(
                run.b_fire_damage, without.b_fire_damage,
                "seed {}",
                run.seed
            );
            assert_eq!(run.b_sunk, without.b_sunk);
            if history.sets > 0 {
                burned_and_still_afloat += 1;
            }
        }
        compared += 1;
    }
    assert!(compared > 0 && burned_and_still_afloat > 0);
}

// ---------------------------------------------------------------------
// Bombed again.
// ---------------------------------------------------------------------

#[test]
fn a_raid_that_sets_a_fire_on_a_burning_carrier_puts_it_at_3_and_that_pulse_is_the_one_set() {
    // The walk asserts that after a set the level is 3, the pulse is the one
    // it was set in and no draw is made at its end; it counts the sets on a
    // carrier already burning.
    let mut again = 0;
    for run in batch() {
        again += walk(run).bombed_again;
    }
    assert!(again > 0);
}

// ---------------------------------------------------------------------
// The variants.
// ---------------------------------------------------------------------

/// The one-day game, in which the Japanese bombers of group 3 are ordered
/// to strike at the Allied force, which the Japanese side starts holding a
/// contact of (a contact held at the start is one pulse old by the morning,
/// so the group is allowed a strike on a contact one pulse old), holding
/// back the share `held` of their aircraft: the launch that takes some of
/// the bombers away in the same pulse the Allied bombers come.
fn strike_back(text: String, held: &str) -> String {
    let mut text = common::replace_once(&text, "days = 3", "days = 1");
    text.push_str(&format!(
        "
[[contact]]
side = \"japanese\"
q = 4
r = 0
ship_count = 2
kind = \"carrier\"

[[order]]
day = 1
side = \"japanese\"
air_group = 3
kind = \"set_air_mission\"
mission = \"naval_strike\"

[[order]]
day = 1
side = \"japanese\"
air_group = 3
kind = \"set_strike_contact_age\"
pulses = 1

[[order]]
day = 1
side = \"japanese\"
air_group = 3
kind = \"set_strike_held_share\"
share = \"{held}\"
"
    ));
    text
}

/// Checks every raid that hit carrier B in a run of a variant in which the
/// Japanese bombers of group 3 strike: she drew with those of the group that
/// the group launched in that pulse flying, the share of those aboard, and
/// `plain` the chance for one bomb at a share of 1. In the morning nothing
/// has flown before, so the 20 bombers are the flying and the aboard
/// exactly; later in the day aircraft away from an earlier sortie are
/// counted in neither. Returns the (flying, hits) of the morning's raids.
fn check_rolls_against_launches(
    runs: &[Run],
    plain: Fixed,
    skip: impl Fn(u32, u32) -> bool,
) -> BTreeSet<(u32, u32)> {
    let mut seen = BTreeSet::new();
    for run in runs {
        for day in 1..=run.days {
            for pulse in Pulse::ALL {
                let here = (day, pulse);
                let flying = run.flew(3, here);
                let attacks = run.attacks_on(CARRIER_B, here);
                let hits: u32 = attacks.iter().map(|(_, hits)| hits).sum();
                // Sunk by the raid, not by the fire at the pulse's end.
                let sunk = run
                    .sunk_in(here)
                    .iter()
                    .any(|(ship, causes)| *ship == CARRIER_B && !causes_are_fire(run, causes));
                let rolled: Vec<Roll> = run
                    .rolls()
                    .into_iter()
                    .filter(|roll| (roll.day, roll.pulse) == here)
                    .collect();
                if hits == 0 || sunk {
                    assert!(rolled.is_empty(), "seed {} {here:?}", run.seed);
                    continue;
                }
                let morning = pulse == Pulse::Morning;
                if morning && flying == 20 {
                    // All away: the share is nothing, and no draw is made.
                    assert!(rolled.is_empty(), "seed {} {here:?}", run.seed);
                    seen.insert((20, hits));
                    continue;
                }
                if morning {
                    assert_eq!(rolled.len(), 1, "seed {} {here:?}", run.seed);
                } else if rolled.is_empty() {
                    // None aboard: no draw.
                    continue;
                }
                let roll = rolled[0];
                assert_eq!(roll.flying, flying, "seed {} {here:?}", run.seed);
                assert_eq!(roll.bombs, hits);
                assert!(roll.aboard > 0);
                assert_eq!(
                    roll.share,
                    Fixed::from_ratio(i64::from(roll.aboard), i64::from(roll.aboard + roll.flying))
                );
                if morning {
                    assert_eq!(roll.aboard, 20 - flying);
                }
                if !skip(roll.flying, hits) {
                    assert_eq!(
                        roll.chance,
                        chance_for(plain * roll.share, hits),
                        "seed {} {here:?} {} flying, {hits} hits",
                        run.seed,
                        roll.flying
                    );
                }
                if morning {
                    seen.insert((flying, hits));
                }
            }
        }
    }
    seen
}

#[test]
fn half_away_the_share_is_0_5_and_the_chances_0_3_0_51_and_0_657() {
    let scenario = build(Figures::Fire, &[], |text| strike_back(text, "0.5"));
    let runs = batch_of(&scenario, SEEDS);
    // The launch that takes 10.
    let launched: Vec<u32> = runs
        .iter()
        .flat_map(|run| {
            run.events.iter().filter_map(|event| match event.kind {
                EventKind::StrikeLaunched {
                    air_group,
                    aircraft,
                    ..
                } if air_group.raw() == 3 && event.pulse == Some(Pulse::Morning) => Some(aircraft),
                _ => None,
            })
        })
        .collect();
    assert!(!launched.is_empty() && launched.iter().all(|aircraft| *aircraft == 10));
    let seen = check_rolls_against_launches(&runs, m(600_000), |_, _| false);
    for hits in 1..=3 {
        assert!(
            seen.contains(&(10, hits)),
            "10 flying and {hits} hits never came up"
        );
    }
    // The stated chances, in the game's arithmetic.
    for run in &runs {
        for roll in run
            .rolls()
            .into_iter()
            .filter(|roll| roll.flying == 10 && roll.pulse == Pulse::Morning)
        {
            assert_eq!(roll.share, m(500_000));
            match roll.bombs {
                1 => assert_eq!(roll.chance, m(300_000)),
                2 => assert_eq!(roll.chance, m(510_000)),
                3 => assert_eq!(roll.chance, m(657_000)),
                _ => {}
            }
        }
    }
}

#[test]
fn all_away_and_none_the_chance_is_nothing_and_no_draw_is_made() {
    // All 20 flying.
    let scenario = build(Figures::Fire, &[], |text| strike_back(text, "0"));
    let runs = batch_of(&scenario, SEEDS);
    let seen = check_rolls_against_launches(&runs, m(600_000), |_, _| false);
    assert!(
        seen.iter().any(|(flying, hits)| *flying == 20 && *hits > 0),
        "a raid hit her while all 20 were away on some seed"
    );
    // Every roll of the morning was of a pulse with none of her bombers
    // launched in it.
    for run in &runs {
        for roll in run
            .rolls()
            .into_iter()
            .filter(|roll| roll.pulse == Pulse::Morning)
        {
            assert_eq!(roll.flying, 0);
        }
    }

    // No bomber in her groups: group 3 removed, the fighters kept.
    let group_3 = "[[air_group]]
id = 3
name = \"Placeholder bombing squadron B\"
side = \"japanese\"
ship = 3
aircraft_type = 3
ready = 20

";
    let none = build(Figures::Fire, &[], |text| {
        common::replace_once(&text, group_3, "")
    });
    assert!(!none
        .world
        .umpire()
        .air_groups
        .contains_key(&AirGroupId::new(3)));
    assert_eq!(
        none.world.umpire().air_groups[&AirGroupId::new(4)].ready,
        10
    );
    let mut raids_that_hit = 0;
    for run in batch_of(&none, SEEDS) {
        if run
            .events
            .iter()
            .any(|e| matches!(e.kind, EventKind::AirAttack { ship, hits, .. } if ship == CARRIER_B && hits > 0))
        {
            raids_that_hit += 1;
        }
        assert!(!run.any_fire_record(), "seed {}", run.seed);
        assert!(run.b_fire.is_none());
    }
    assert!(raids_that_hit > 0);
}

#[test]
fn the_other_navy_reads_its_own_figure_0_15_and_the_chances_follow() {
    // Sides exchanged: the carrier bombed is Allied, 20 bombers aboard.
    let scenario = build(Figures::Fire, &[], exchange_sides);
    let truth = scenario.world.umpire();
    assert_eq!(truth.ships[&CARRIER_B].side, Side::Allied);
    assert_eq!(truth.air_groups[&AirGroupId::new(3)].side, Side::Allied);
    let runs = batch_of(&scenario, SEEDS);
    let mut seen = BTreeSet::new();
    for run in &runs {
        for roll in run.rolls() {
            assert_eq!((roll.aboard, roll.flying, roll.share), (20, 0, Fixed::ONE));
            seen.insert(roll.bombs);
            match roll.bombs {
                1 => assert_eq!(roll.chance, m(150_000), "seed {}", run.seed),
                2 => assert_eq!(roll.chance, m(277_500), "seed {}", run.seed),
                3 => assert_eq!(roll.chance, m(385_875), "seed {}", run.seed),
                bombs => assert_eq!(roll.chance, chance_for(m(150_000), bombs)),
            }
        }
        walk(run);
    }
    for hits in 1..=3 {
        assert!(seen.contains(&hits));
    }

    // With 10 of 20 flying: 0.075, 0.144375 and (see the next test) 0.208547.
    let half = build(Figures::Fire, &[], |text| {
        exchange_sides(strike_back(text, "0.5"))
    });
    let runs = batch_of(&half, SEEDS);
    let seen = check_rolls_against_launches(&runs, m(150_000), |flying, hits| {
        // The chance for 3 hits, 0.208547, is asserted by the next test.
        flying == 10 && hits == 3
    });
    for hits in [1, 2, 3] {
        assert!(seen.contains(&(10, hits)));
    }
    for run in &runs {
        for roll in run
            .rolls()
            .into_iter()
            .filter(|roll| roll.flying == 10 && roll.pulse == Pulse::Morning)
        {
            assert_eq!(roll.share, m(500_000));
            match roll.bombs {
                1 => assert_eq!(roll.chance, m(75_000)),
                2 => assert_eq!(roll.chance, m(144_375)),
                _ => {}
            }
        }
    }
}

#[test]
fn the_other_navy_half_away_3_hits_is_0_208547() {
    // 1 - 0.925^3 = 0.208546875, which the game holds to six places as
    // 0.208547 (restated by the architect): each product cut to millionths,
    // 0.925 x 0.925 = 0.855625, x 0.925 = 0.791453.
    let half = build(Figures::Fire, &[], |text| {
        exchange_sides(strike_back(text, "0.5"))
    });
    let runs = batch_of(&half, SEEDS);
    let mut seen = 0;
    for run in &runs {
        for roll in run
            .rolls()
            .into_iter()
            .filter(|r| r.flying == 10 && r.bombs == 3)
        {
            seen += 1;
            assert_eq!(roll.chance, m(208_547), "the game gives {}", roll.chance);
        }
    }
    assert!(seen > 0);
}

#[test]
fn a_cruiser_with_a_floatplane_group_aboard_that_is_bombed_draws_nothing() {
    // The Allied bombers are ordered to rank cruisers first, and attack the
    // lower-numbered cruiser, ship 4, which carries four floatplanes
    // (placeholder scouts of type 2). Carrier B is not attacked at all.
    let scenario = build(Figures::Fire, &[CRUISER_WITH_FLOATPLANES], |mut text| {
        text.push_str(
            "
[[air_group]]
id = 5
name = \"Placeholder floatplanes B1\"
side = \"japanese\"
ship = 4
aircraft_type = 2
ready = 4

[[order]]
day = 1
side = \"allied\"
air_group = 2
kind = \"set_target_priority\"
kinds = [\"cruiser\"]
",
        );
        text
    });
    let cruiser = ShipId::new(4);
    let mut cruiser_hit = 0;
    for run in batch_of(&scenario, SEEDS) {
        for event in &run.events {
            if let EventKind::AirAttack {
                ship, hits, weapon, ..
            } = event.kind
            {
                assert!(
                    ship == cruiser || ship == ShipId::new(5),
                    "seed {}: only the cruisers are bombed",
                    run.seed
                );
                assert_eq!(weapon, Weapon::Bomb);
                if hits > 0 {
                    cruiser_hit += 1;
                }
            }
        }
        // No draw, no record, nothing of this rule.
        assert!(!run.any_fire_record(), "seed {}", run.seed);
        assert!(run.b_fire.is_none());
    }
    assert!(
        cruiser_hit > 0,
        "the cruiser was bombed with hits on some seed"
    );
}

#[test]
fn a_torpedo_sets_no_such_fire() {
    let scenario = build(Figures::Fire, &[TORPEDO_BOMBER], |text| {
        common::replace_once(
            &text,
            "aircraft_type = 3\nready = 18",
            "aircraft_type = 5\nready = 18",
        )
    });
    let mut torpedo_hits = 0;
    for run in batch_of(&scenario, SEEDS) {
        for event in &run.events {
            if let EventKind::AirAttack {
                ship,
                weapon: Weapon::Torpedo,
                hits,
                ..
            } = event.kind
            {
                if ship == CARRIER_B {
                    torpedo_hits += hits;
                }
            }
            assert!(
                !matches!(
                    event.kind,
                    EventKind::AirAttack {
                        weapon: Weapon::Bomb,
                        ..
                    }
                ),
                "the attackers carry torpedoes only"
            );
        }
        assert!(!run.any_fire_record(), "seed {}", run.seed);
    }
    assert!(torpedo_hits > 0, "torpedoes hit her on some seed");
}

#[test]
fn a_raid_whose_hits_sink_her_makes_no_draw() {
    // She starts with 95 of flotation damage: a hit on flotation sinks her.
    let scenario = build(Figures::Fire, &[], |text| {
        common::replace_once(
            &text,
            "name = \"Placeholder carrier B\"\n",
            "name = \"Placeholder carrier B\"\nflotation_damage = 95\n",
        )
    });
    let (mut sunk_by_a_raid, mut hit_and_not_sunk) = (0, 0);
    for run in batch_of(&scenario, SEEDS) {
        for day in 1..=run.days {
            for pulse in Pulse::ALL {
                let here = (day, pulse);
                let hits: u32 = run
                    .attacks_on(CARRIER_B, here)
                    .iter()
                    .map(|(_, hits)| hits)
                    .sum();
                let sunk = run.sunk_in(here).iter().any(|(ship, _)| *ship == CARRIER_B);
                let rolled = run
                    .rolls()
                    .iter()
                    .any(|roll| (roll.day, roll.pulse) == here);
                if hits > 0 && sunk {
                    assert!(
                        !rolled,
                        "seed {} {here:?}: a draw for a ship the raid sank",
                        run.seed
                    );
                    sunk_by_a_raid += 1;
                } else if hits > 0 {
                    assert!(rolled);
                    hit_and_not_sunk += 1;
                }
            }
        }
        assert!(
            !run.events
                .iter()
                .any(|e| matches!(e.kind, EventKind::AircraftFireFought { .. }))
                || run.rolls().iter().any(|roll| roll.set)
        );
    }
    assert!(sunk_by_a_raid > 0, "a raid sank her on some seed");
    // Every raid that hit her but did not sink her drew (fire-track hits).
    let _ = hit_and_not_sunk;
}

// ---------------------------------------------------------------------
// The figure by side.
// ---------------------------------------------------------------------

#[test]
fn with_no_value_by_side_both_sides_read_the_plain_value() {
    let scenario = build(Figures::FireNoBySide, &[], |text| text);
    let air = scenario.world.ref_data().air_combat.as_ref().expect("held");
    assert_eq!(
        air.aircraft_fire.expect("held").chance,
        BySide::both(m(300_000))
    );
    for side in [Side::Allied, Side::Japanese] {
        let view = scenario.world.side_view(side);
        let seen = view
            .air_combat_figures()
            .expect("held")
            .aircraft_fire
            .expect("held");
        assert_eq!(seen.chance, BySide::both(m(300_000)));
    }
    // The chances of a raid follow the plain value, whichever side's
    // carrier is bombed.
    for exchanged in [false, true] {
        let scenario = build(Figures::FireNoBySide, &[], |text| {
            if exchanged {
                exchange_sides(text)
            } else {
                text
            }
        });
        let mut seen = BTreeSet::new();
        for run in batch_of(&scenario, 300) {
            for roll in run.rolls() {
                seen.insert(roll.bombs);
                assert_eq!(roll.chance, chance_for(m(300_000), roll.bombs));
                match roll.bombs {
                    1 => assert_eq!(roll.chance, m(300_000)),
                    2 => assert_eq!(roll.chance, m(510_000)),
                    3 => assert_eq!(roll.chance, m(657_000)),
                    _ => {}
                }
            }
        }
        assert!(seen.contains(&1) && seen.contains(&2) && seen.contains(&3));
    }
}

#[test]
fn a_sides_view_gives_its_own_value_in_both_places_and_not_the_enemys() {
    let scenario = base();
    let air = scenario.world.ref_data().air_combat.as_ref().expect("held");
    // The truth holds each side's own.
    assert_eq!(
        air.aircraft_fire.expect("held").chance,
        BySide::new(m(150_000), m(600_000))
    );
    let own = |side: Side| {
        scenario
            .world
            .side_view(side)
            .air_combat_figures()
            .expect("held")
            .aircraft_fire
            .expect("held")
            .chance
    };
    assert_eq!(own(Side::Allied), BySide::both(m(150_000)));
    assert_eq!(own(Side::Japanese), BySide::both(m(600_000)));
    // The enemy's value appears nowhere in a side's view.
    assert_eq!(own(Side::Allied).of(Side::Japanese), m(150_000));
    assert_eq!(own(Side::Japanese).of(Side::Allied), m(600_000));
}

// ---------------------------------------------------------------------
// Off where the figures are absent; determinism and the hash.
// ---------------------------------------------------------------------

#[test]
fn on_the_frozen_figures_none_of_this_happens() {
    let scenario = build(Figures::Frozen, &[], |text| text);
    assert!(scenario
        .world
        .ref_data()
        .air_combat
        .as_ref()
        .expect("held")
        .aircraft_fire
        .is_none());
    let mut hit = 0;
    for run in batch_of(&scenario, 300) {
        assert!(!run.any_fire_record());
        assert!(run.b_fire.is_none());
        if run
            .events
            .iter()
            .any(|e| matches!(e.kind, EventKind::AirAttack { ship, hits, .. } if ship == CARRIER_B && hits > 0))
        {
            hit += 1;
        }
    }
    assert!(hit > 0);
}

#[test]
fn the_same_seed_gives_the_same_game_and_the_hash_holds_her_fire() {
    let scenario = base();
    let frozen = build(Figures::Frozen, &[], |text| text);
    let mut quiet_days = 0;
    let mut burning_days = 0;
    for seed in 1..=300 {
        let run = run_of(&scenario, seed);
        let again = run_of(&scenario, seed);
        assert_eq!(run.events, again.events, "seed {seed}");
        assert_eq!(run.hashes, again.hashes, "seed {seed}");
        assert_eq!(run.hashes.len(), 3);

        // The build without the rule differs from this one in her fire among
        // aircraft and what follows from a loss, and in nothing else: its
        // daily hash is the same on each day until a fire is set, and
        // differs on each day on which one burns (the hash holds it).
        let without = run_of(&frozen, seed);
        let mut level = 0u8;
        let mut touched = false;
        for day in 1..=run.days {
            for pulse in Pulse::ALL {
                for event in run
                    .events
                    .iter()
                    .filter(|e| run.at_opt(e) == Some((day, pulse)))
                {
                    match event.kind {
                        EventKind::AircraftFireRoll { set, .. } => {
                            touched = true;
                            if set {
                                level = 3;
                            }
                        }
                        EventKind::AircraftFireFought {
                            level_after, lost, ..
                        } => level = if lost { 0 } else { level_after },
                        _ => {}
                    }
                }
            }
            let index = usize::try_from(day - 1).expect("small");
            if !touched {
                assert_eq!(
                    run.hashes[index], without.hashes[index],
                    "seed {seed} day {day}"
                );
                quiet_days += 1;
            } else if level >= 1 {
                assert_ne!(
                    run.hashes[index], without.hashes[index],
                    "seed {seed} day {day}"
                );
                burning_days += 1;
            }
        }
    }
    assert!(quiet_days > 0 && burning_days > 0);
}

/// A digest of every event of a scenario's runs on the frozen figures, for
/// seeds 1 to 100: FNV-1a over the debug text of each event, ids, causes
/// and all, in order.
fn digest_of_frozen_runs(name: &str) -> (u64, usize) {
    let path =
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(format!("../data/scenarios/{name}.toml"));
    let base = common::load(&path).expect("the scenario loads on the frozen figures");
    let mut hash = 0xcbf2_9ce4_8422_2325u64;
    let mut count = 0;
    for seed in 1..=100u64 {
        let mut run = base.with_seed(seed).expect("a seed can be set");
        run.run_to_end().expect("the run resolves");
        for event in run.world.events().iter() {
            for byte in format!("{event:?}").bytes() {
                hash ^= u64::from(byte);
                hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
            }
            count += 1;
        }
    }
    (hash, count)
}

#[test]
fn on_the_frozen_figures_every_event_of_scenario_5b_is_byte_for_byte_what_it_was() {
    // The digest below was taken from the build without this rule (the
    // commit before the engine change of decisions 0211 and 0212, 9fc04a5c)
    // and is the same on the build with it: 1,554 events over seeds 1 to
    // 100, with their ids, causes and breakdowns. Remade for decision 0218,
    // stage 2: every claim's record gains `from_submarine`, false here; with
    // it taken out the digest is the one before, 0x9c1b_fecc_6760_de9d.
    assert_eq!(
        digest_of_frozen_runs("05b_strike"),
        (0x8343_3cb7_77dd_5cfb, 1554)
    );
}
