//! Scenario 45: the soak ashore.
//!
//! `data/scenarios/45_soak_ashore.toml` is the full slice of scenario 7 (7
//! August 1942, the landings and the land fight in it) carried on for 180
//! days, as scenario 8 carried the old slice on, with the rules built since
//! the first soak at work in it: land units, the fight ashore, marches,
//! landings, works, cover, preparation, training, airfield damage, a beaten
//! side that falls back or surrenders, the staff and its commands, and
//! refuelling at a port's rate. It is a machine test and not history. There
//! is no hand calculation of outcomes: nothing in it is compared with the
//! record, and no number of a result is asserted. What is claimed is below,
//! and nothing else.
//!
//! **Every test here runs on the live figures** (`common::load_live`), as the
//! slice does. Scenario 8's window and running-out tests run on frozen
//! figures; this scenario has no frozen copy and could not run on them, since
//! the frozen step 1 ground file switches off preparation, experience,
//! skills, cover and airfield damage, and most of what it keeps busy could not
//! happen there. Frozen copies are made only by the architect.
//!
//! 1. **It survives.** The scenario runs its 180 days with the staffs of both
//!    sides passed in (`sim_staff::staffs()`, as `sim-cli` does) without a
//!    panic and with every invariant the engine checks at the close of a day
//!    holding (`Scenario::next_day_watched` fails a day that breaks one, the
//!    state of both sides' commands now among them), for the scenario's own
//!    seed, 1942, and for seeds 1 to `OTHER_SEEDS` in the ordinary test run.
//!    A second test, `#[ignore]`d and run by hand, does the same for seeds 1
//!    to 100.
//! 2. **Every subsystem stays in use.** The 180 days are split into six
//!    windows of 30, and in the seed-1942 run each window has at least one
//!    event of each of these kinds, named by the engine's own event kinds:
//!
//!    | Activity | Event kinds |
//!    | -------- | ----------- |
//!    | a raid on a base | `BaseStrikeLaunched` |
//!    | an air search flown | `SearchFlown` |
//!    | a strike on ships launched, or found nothing | `StrikeLaunched`, `StrikeFoundNothing` |
//!    | a surface engagement begun | `EngagementBegun` |
//!    | a convoy unloading completed | `UnloadingComplete` |
//!    | an aircraft repair | `AircraftRepaired` |
//!    | an arrival or replacement received | `AircraftArrived`, `ReplacementsReceived` |
//!    | a group stood down | `AirGroupStoodDown` |
//!    | a group returned | `AirGroupReturned` |
//!    | a fight ashore | `GroundFight` |
//!    | a landing of men | `MenLanded` |
//!    | a march | `MarchDay` |
//!    | men fed | `MenFed` with fed above 0 |
//!    | men unfed | `MenFed` with unfed above 0, `MenLostUnfed` |
//!    | a unit's work at a base | `UnitWorked`, `AirfieldBuilt`, `AirfieldMended` |
//!    | bombs or shells falling on men | `MenLostToFire` |
//!    | bombs or shells on an airfield | `AirfieldDamaged` |
//!    | fieldworks dug | `FieldworksChanged` (dug) |
//!    | an estimate of the enemy ashore | `GroundReportReceived` |
//!    | a ship fuelling at a port's rate | `ShipRefuelled` |
//!    | a staff order given (each side) | the staff's hand-in of the day |
//!    | a staff day with no order (Allied) | the same |
//!    | a load to a weight (Allied) | a staff order `LoadTo` |
//!
//!    Both sides also still have groups flying in the last window.
//! 3. **Each of these happens at least once in the seed-1942 run, and the run
//!    then carries on to day 180** with the invariants holding: a side beaten
//!    that falls back along a land link; a unit that surrenders where no land
//!    link leaves its hex, and one that surrenders where one does and no hex
//!    is free; an attack that fails; a spent attacker that holds and one that
//!    falls back; a base that changes side ashore; a unit at the limit of
//!    strain; fieldworks at their limit and worn by fire; an airfield that
//!    refuses landings for damage and is mended; a base's defences raised past
//!    the prepared grade; a prepared landing (loading, rehearsal, the landing)
//!    and an unprepared one; a unit trained up a level; men lost unfed; a force
//!    that sails short of fuel; a ship held at a fuel reserve; a hex found
//!    empty; a staff that cannot meet an intent with a standing interrupt
//!    firing; a commander's order that overrides the staff; a run ended by a
//!    hold; a task force out of fuel; an air group that had aircraft and has
//!    none, and one with fewer pilots than aircraft; a task force that lost
//!    every ship. Scenario 8's "Rabaul runs out of supply" is not among them
//!    (the Japanese staff draws on Rabaul's stock: see the scenario file).
//! 4. **Nothing leaves its range, every day of every run** (the checks of
//!    `check_day`): no supply, fuel or gun-round count is negative; fatigue,
//!    damage, rehearsal, loading, fed share, experience, fieldworks, defences
//!    and airfield damage are inside their ranges; a ship carries no more men
//!    than its troop capacity, no more cargo than its room, and no more fuel
//!    than its tanks; a land unit has a hex exactly when it has men ashore;
//!    groups have no more aircraft or pilots than their strength; gun rounds
//!    never rise and the sunk stay sunk; no ship has two commitments on a
//!    day; every interrupt's status agrees with its firings.
//! 5. **Conservation, every day,** from the day's events and the state:
//!    - men: for each side, the men ashore and aboard equal the men at the
//!      start less each loss counted once (lost with a ship, unfed, in a fight,
//!      to fire, surrendered);
//!    - supply at each base: start + unloaded + carried in by a march = held +
//!      spent (aircraft, repair, sorties, men, a march) + destroyed + loaded
//!      out + lost to a capture; fuel at each base: start + unloaded =
//!      held + taken by ships + loaded out + lost to a capture;
//!    - aboard a ship: supply and cargo fuel are what it started with plus
//!      what it loaded less what it unloaded; its own fuel is what it started
//!      with less what it burned plus what it took in port;
//!    - aircraft and pilots, from the events and the final state, as scenario 8
//!      states it.
//! 6. **The staffs.** Run on every seed, no order of a staff is ever refused
//!    and none is given to a held ship, a held force or a force with a held
//!    ship in it; no staff load takes a stock below its reserve and no
//!    refuelling takes fuel below it; and every event names causes that exist
//!    and come before it. `sim-cli replay --check-staff` passes on the saved
//!    game of the seed-1942 run.
//! 7. **Determinism.** Seed 1942 run twice gives the same 180 daily hashes
//!    and the same events.
//! 8. **Measurements** are printed by `print_the_summary`: events in the log
//!    in total and by 30-day window, and the first day each thing happens.
//!
//! # Departures from scenario 7, and why
//!
//! - **Every test here runs on the live figures; there is no frozen copy**
//!   (the architect's ruling 2). Frozen copies are made only by the architect,
//!   and the frozen step 1 ground file switches off preparation, experience,
//!   skills, cover and airfield damage, so most of what this scenario keeps
//!   busy could not happen on it.
//! - **The Allied intents are set on day 2, not day 1.** Lunga and Tulagi are
//!   Japanese on day 1 (the Marines take them in that day's ground phase), and
//!   the engine refuses an intent on a base that is not the side's. The
//!   Japanese intent, on a placeholder base of their own, is set on day 1.
//! - **Scenario 8's scripted convoys 21 and 22 are dropped**: with them the
//!   scripted supply left Lunga so full that the Allied staff had nothing to
//!   do for sixty days. The staff's runs, and the slice's convoy 5, supply it.
//! - **Scenario 8's two Allied replacement streams at base 1 are dropped**: the
//!   loader refuses an Allied stream at a base that is Japanese at the start.
//!   The Allied groups of scenario 8's part 2 start with nothing and arrive on
//!   day 17 for the same reason, and its arrivals for group 12 (retired in
//!   scenario 7) and the two that scenario 7 now has are left out.
//! - **Scenario 8's "Rabaul runs out of supply" does not happen**: the
//!   Japanese staff draws on Rabaul's 10,000 t, and some is left at day 180. The other
//!   running-out checks of scenario 8 (a ship with no fuel, a group with no
//!   aircraft, fewer pilots than aircraft, a force with no ship) are kept.
//!
//! # What it found
//!
//! On seed 79, day 120, the staff's commitment of a ship was refused
//! (`AlreadyCommitted`): the morning's cancelled plan still held the ship,
//! which the doctrine read as free but freed only after serving the intents.
//! The doctrine now frees an ended plan's ships first (`supply-7`; decision
//! 0093, "As built: the soak ashore").
//!
//! The scenario file's comments say what each part of it is for. A fault this
//! test finds in a rule is a finding and not something to work around.

mod common;

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::Instant;

use sim_core::choices::Moment;
use sim_core::command::{
    CommandKind, HoldTarget, InterruptCondition, InterruptRef, StaffDecisions, StandingKind,
};
use sim_core::events::{
    Cause, Event, EventKind, ExperienceSource, FieldworksChange, LandFatigueChange,
};
use sim_core::fixed::Fixed;
use sim_core::hex::Hex;
use sim_core::ids::{LandUnitId, ShipId, TaskForceId};
use sim_core::orders::{BaseOrder, Cargo, Issuer, OrderKind, TaskForceOrder};
use sim_core::state::{ShipLocation, Side, TaskForceStatus, TruthState};
use sim_core::view::SideView;
use sim_core::world::World;
use sim_data::Scenario;
use sim_staff::{as_staffs, staffs};

const ROOT: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/..");
const SOAK: &str = "45_soak_ashore.toml";
const DAYS: usize = 180;
const WINDOW: usize = 30;
const WINDOWS: usize = DAYS / WINDOW;
/// Seeds 1 to this are run in the ordinary test run, besides 1942.
const OTHER_SEEDS: u64 = 48;

/// The air groups of the two bases (the slice's and the soak's own).
const RABAUL_GROUPS: [u32; 4] = [1, 2, 3, 205];
const HENDERSON_GROUPS: [u32; 10] = [206, 207, 10, 11, 13, 14, 15, 16, 17, 204];

fn scenario_path(name: &str) -> PathBuf {
    Path::new(ROOT).join("data/scenarios").join(name)
}

/// Every test of this file runs on the live figures.
fn load(seed: u64) -> Scenario {
    common::load_live(&scenario_path(SOAK))
        .expect("the scenario loads")
        .with_seed(seed)
        .expect("a seed can be set")
}

fn hundred() -> Fixed {
    Fixed::from_int(100)
}

/// A wall-clock stopwatch for the measurements. The workspace forbids
/// `Instant::now` so that nothing in the simulation can read the clock
/// (clippy.toml). This is the one place a test reads it, to time a run from
/// outside: no rule, order or assertion depends on what it says.
struct Stopwatch(Instant);

impl Stopwatch {
    #[allow(clippy::disallowed_methods)]
    fn start() -> Stopwatch {
        Stopwatch(Instant::now())
    }

    fn seconds(&self) -> f64 {
        self.0.elapsed().as_secs_f64()
    }
}

/// An event's number in the umpire's log, for a message.
fn ev(world: &World, event: &Event) -> u32 {
    event.id.umpire_raw(&world.umpire_key())
}

fn day_of(event: &Event) -> usize {
    usize::try_from(event.day).expect("a day fits")
}

// ---------------------------------------------------------------------
// What one run leaves.
// ---------------------------------------------------------------------

/// An air group at the close of a day.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct GroupState {
    ready: u32,
    damaged: u32,
    pilots: Option<u32>,
    strength: u32,
}

impl GroupState {
    fn aircraft(&self) -> u32 {
        self.ready + self.damaged
    }
}

/// The part of the truth that the running-out checks read, at one moment.
#[derive(Clone, PartialEq, Eq, Debug, Default)]
struct Picture {
    groups: BTreeMap<u32, GroupState>,
    base_supply: BTreeMap<u32, Fixed>,
    base_fuel: BTreeMap<u32, Fixed>,
    ship_fuel: BTreeMap<u32, Fixed>,
    ship_supply: BTreeMap<u32, Fixed>,
    ship_cargo_fuel: BTreeMap<u32, Fixed>,
    base_defences: BTreeMap<u32, Fixed>,
    /// Task forces with no ship.
    empty_forces: BTreeSet<u32>,
    /// Ships afloat with no fuel.
    dry_ships: BTreeSet<u32>,
}

fn picture(truth: &TruthState) -> Picture {
    Picture {
        groups: truth
            .air_groups
            .values()
            .map(|g| {
                (
                    g.id.raw(),
                    GroupState {
                        ready: g.ready,
                        damaged: g.damaged,
                        pilots: g.pilots,
                        strength: g.strength,
                    },
                )
            })
            .collect(),
        base_supply: truth
            .bases
            .values()
            .map(|b| (b.id.raw(), b.supply_t))
            .collect(),
        base_fuel: truth
            .bases
            .values()
            .map(|b| (b.id.raw(), b.fuel_t))
            .collect(),
        ship_fuel: truth
            .ships
            .values()
            .map(|s| (s.id.raw(), s.fuel_t))
            .collect(),
        ship_supply: truth
            .ships
            .values()
            .map(|s| (s.id.raw(), s.supply_t))
            .collect(),
        ship_cargo_fuel: truth
            .ships
            .values()
            .map(|s| (s.id.raw(), s.cargo_fuel_t))
            .collect(),
        base_defences: truth
            .bases
            .values()
            .map(|b| (b.id.raw(), b.defences))
            .collect(),
        empty_forces: truth
            .task_forces
            .values()
            .filter(|t| t.ships.is_empty())
            .map(|t| t.id.raw())
            .collect(),
        dry_ships: truth
            .ships
            .values()
            .filter(|s| s.location != ShipLocation::Sunk && s.fuel_t.is_zero())
            .map(|s| s.id.raw())
            .collect(),
    }
}

/// What a side's staff handed in on one day, and the standing of its command
/// state after it.
#[derive(Clone, Default, Debug)]
struct StaffDay {
    orders: usize,
    no_orders: usize,
    /// A staff order that loads to a weight.
    load_to: bool,
    /// The names of the notices handed in.
    cannot_meet: bool,
}

/// What one run left: the day's hash and picture for every day, the whole
/// event log, the staff's days and the time the days took.
struct Run {
    seed: u64,
    start: Picture,
    hashes: Vec<u64>,
    days: Vec<Picture>,
    events: Vec<Event>,
    /// Each day's hand-in of the Allied and the Japanese staff.
    staff: Vec<[StaffDay; 2]>,
    /// The first day a unit's surrender was where no land link leaves.
    unlinked_hexes: BTreeSet<Hex>,
    /// The days on which a force sailed from a base of its side with a ship
    /// whose tanks were not full.
    sailed_short: Vec<usize>,
    /// The first day a staff order was refused, with its description, per
    /// seed: empty when the staffs are sound.
    problems: Vec<String>,
    seconds: f64,
    /// The time each day took to resolve, in seconds.
    day_seconds: Vec<f64>,
}

impl Run {
    fn end(&self) -> &Picture {
        self.days.last().expect("a run has days")
    }
}

// ---------------------------------------------------------------------
// What the staff may not reach: the holds of the morning, as scenario 41
// reads them.
// ---------------------------------------------------------------------

#[derive(Clone, Debug, Default)]
struct Held {
    ships: BTreeSet<ShipId>,
    forces: BTreeSet<TaskForceId>,
    settings: usize,
    /// The ships of each force of the side.
    members: BTreeMap<TaskForceId, BTreeSet<ShipId>>,
}

impl Held {
    fn read(view: SideView<'_>) -> Held {
        let mut held = Held::default();
        for target in view.command_state().holds.keys() {
            match target {
                HoldTarget::Ship(s) => {
                    held.ships.insert(*s);
                }
                HoldTarget::TaskForce(f) => {
                    held.forces.insert(*f);
                }
                HoldTarget::Setting { .. }
                | HoldTarget::AirGroup(_)
                | HoldTarget::LandUnit(_)
                | HoldTarget::Submarine(_) => held.settings += 1,
            }
        }
        for s in view.ships() {
            if let ShipLocation::TaskForce(f) = s.location {
                held.members.entry(f).or_default().insert(s.id);
            }
        }
        held
    }

    /// Whether an order of the staff reaches what is held.
    fn reaches(&self, kind: &OrderKind) -> bool {
        match kind {
            OrderKind::TaskForce { target, .. } => {
                self.forces.contains(target)
                    || self
                        .members
                        .get(target)
                        .is_some_and(|m| m.iter().any(|s| self.ships.contains(s)))
            }
            OrderKind::Base {
                order: BaseOrder::FormTaskForce { ships },
                ..
            } => ships
                .iter()
                .any(|s| self.ships.contains(s) || self.in_held_force(*s)),
            _ => false,
        }
    }

    fn in_held_force(&self, ship: ShipId) -> bool {
        self.members.iter().any(|(f, m)| {
            m.contains(&ship)
                && (self.forces.contains(f) || m.iter().any(|s| self.ships.contains(s)))
        })
    }
}

// ---------------------------------------------------------------------
// The books of a run: what each stock should be, from the events.
// ---------------------------------------------------------------------

/// What the stocks of the run should be at the close of each day, worked from
/// the events alone. A base that did not exist at the start (formed by a
/// landing or a march) starts at 0.
#[derive(Default)]
struct Books {
    base_supply: BTreeMap<u32, Fixed>,
    base_fuel: BTreeMap<u32, Fixed>,
    ship_fuel: BTreeMap<u32, Fixed>,
    ship_supply: BTreeMap<u32, Fixed>,
    ship_cargo_fuel: BTreeMap<u32, Fixed>,
    /// Men at the start, and what has been lost since, for each side.
    men_start: [u64; 2],
    men_lost: [u64; 2],
}

fn side_index(side: Side) -> usize {
    match side {
        Side::Allied => 0,
        Side::Japanese => 1,
    }
}

fn men_in_state(truth: &TruthState) -> [u64; 2] {
    let mut total = [0u64; 2];
    for unit in truth.land_units.values() {
        total[side_index(unit.side)] += u64::from(unit.men_ashore);
    }
    for ship in truth.ships.values() {
        if let Some(troops) = ship.troops {
            let unit = truth.land_units.get(&troops.unit);
            let side = unit.map_or(ship.side, |u| u.side);
            total[side_index(side)] += u64::from(troops.men);
        }
    }
    total
}

impl Books {
    fn new(truth: &TruthState) -> Books {
        Books {
            base_supply: truth
                .bases
                .values()
                .map(|b| (b.id.raw(), b.supply_t))
                .collect(),
            base_fuel: truth
                .bases
                .values()
                .map(|b| (b.id.raw(), b.fuel_t))
                .collect(),
            ship_fuel: truth
                .ships
                .values()
                .map(|s| (s.id.raw(), s.fuel_t))
                .collect(),
            ship_supply: truth
                .ships
                .values()
                .map(|s| (s.id.raw(), s.supply_t))
                .collect(),
            ship_cargo_fuel: truth
                .ships
                .values()
                .map(|s| (s.id.raw(), s.cargo_fuel_t))
                .collect(),
            men_start: men_in_state(truth),
            men_lost: [0, 0],
        }
    }

    fn add(map: &mut BTreeMap<u32, Fixed>, id: u32, amount: Fixed) {
        let entry = map.entry(id).or_insert(Fixed::ZERO);
        *entry += amount;
    }

    fn sub(map: &mut BTreeMap<u32, Fixed>, id: u32, amount: Fixed) {
        let entry = map.entry(id).or_insert(Fixed::ZERO);
        *entry -= amount;
    }

    /// Takes the day's events into the books. `truth` is the state at the
    /// close of the day (it names each unit's side).
    fn take(&mut self, events: &[Event], truth: &TruthState) {
        let side_of_unit = |unit: LandUnitId| -> usize {
            truth
                .land_units
                .get(&unit)
                .map_or(0, |u| side_index(u.side))
        };
        for event in events {
            match &event.kind {
                EventKind::CargoUnloaded {
                    ship,
                    base,
                    cargo,
                    unloaded_t,
                    ..
                } => match cargo {
                    Cargo::Supply => {
                        Self::add(&mut self.base_supply, base.raw(), *unloaded_t);
                        Self::sub(&mut self.ship_supply, ship.raw(), *unloaded_t);
                    }
                    Cargo::Fuel => {
                        Self::add(&mut self.base_fuel, base.raw(), *unloaded_t);
                        Self::sub(&mut self.ship_cargo_fuel, ship.raw(), *unloaded_t);
                    }
                },
                EventKind::CargoLoaded {
                    ship,
                    base,
                    cargo,
                    loaded_t,
                    ..
                } => match cargo {
                    Cargo::Supply => {
                        Self::sub(&mut self.base_supply, base.raw(), *loaded_t);
                        Self::add(&mut self.ship_supply, ship.raw(), *loaded_t);
                    }
                    Cargo::Fuel => {
                        Self::sub(&mut self.base_fuel, base.raw(), *loaded_t);
                        Self::add(&mut self.ship_cargo_fuel, ship.raw(), *loaded_t);
                    }
                },
                EventKind::BaseSupplySpent { base, paid_t, .. } => {
                    Self::sub(&mut self.base_supply, base.raw(), *paid_t);
                }
                EventKind::BaseSupplyDestroyed {
                    base, destroyed_t, ..
                } => Self::sub(&mut self.base_supply, base.raw(), *destroyed_t),
                EventKind::MarchArrived {
                    base: Some(base),
                    supply_t,
                    ..
                } => Self::add(&mut self.base_supply, base.raw(), *supply_t),
                EventKind::BaseChangedSide {
                    base,
                    supply_lost_t,
                    fuel_lost_t,
                    ..
                } => {
                    Self::sub(&mut self.base_supply, base.raw(), *supply_lost_t);
                    Self::sub(&mut self.base_fuel, base.raw(), *fuel_lost_t);
                }
                EventKind::ShipRefuelled {
                    ship,
                    base,
                    taken_t,
                    ..
                } => {
                    Self::sub(&mut self.base_fuel, base.raw(), *taken_t);
                    Self::add(&mut self.ship_fuel, ship.raw(), *taken_t);
                }
                EventKind::FuelBurned { ship, burned_t, .. } => {
                    Self::sub(&mut self.ship_fuel, ship.raw(), *burned_t)
                }
                EventKind::BaseFormed { base, .. } | EventKind::MarchBaseFormed { base, .. } => {
                    self.base_supply.entry(base.raw()).or_insert(Fixed::ZERO);
                    self.base_fuel.entry(base.raw()).or_insert(Fixed::ZERO);
                }
                EventKind::MenLostWithShip { unit, men, .. } => {
                    self.men_lost[side_of_unit(*unit)] += u64::from(*men);
                }
                EventKind::MenLostUnfed { unit, lost, .. }
                | EventKind::MenLostInFight { unit, lost, .. }
                | EventKind::MenLostToFire { unit, lost, .. } => {
                    self.men_lost[side_of_unit(*unit)] += u64::from(*lost);
                }
                EventKind::UnitSurrendered { side, men, .. } => {
                    self.men_lost[side_index(*side)] += u64::from(*men);
                }
                _ => {}
            }
        }
    }
}

// ---------------------------------------------------------------------
// The checks of one day.
// ---------------------------------------------------------------------

/// Every count and track of the truth is inside its range, and what only ever
/// falls has not risen since `before`. Returns the problems found.
fn out_of_range(world: &World, truth: &TruthState, before: Option<&TruthState>) -> Vec<String> {
    let mut problems = Vec::new();
    let refdata = world.ref_data();
    let ground = refdata
        .ground
        .as_ref()
        .expect("the scenario has ground figures");
    for ship in truth.ships.values() {
        let id = ship.id;
        let class = refdata.ship_classes.get(&ship.class);
        if ship.fuel_t.is_negative() {
            problems.push(format!("{id} has negative fuel {}", ship.fuel_t));
        }
        if ship.supply_t.is_negative() || ship.cargo_fuel_t.is_negative() {
            problems.push(format!(
                "{id} has negative cargo {} and {}",
                ship.supply_t, ship.cargo_fuel_t
            ));
        }
        if ship.gun_rounds.is_negative() {
            problems.push(format!("{id} has negative gun rounds {}", ship.gun_rounds));
        }
        for (name, track) in [
            ("flotation", ship.flotation_damage),
            ("fire", ship.fire_damage),
        ] {
            if track < Fixed::ZERO || track > hundred() {
                problems.push(format!("{id} has {name} damage {track}"));
            }
        }
        if let Some(class) = class {
            if ship.fuel_t > class.fuel_capacity_t {
                problems.push(format!(
                    "{id} has {} fuel over its tanks {}",
                    ship.fuel_t, class.fuel_capacity_t
                ));
            }
            if ship.supply_t + ship.cargo_fuel_t > class.cargo_capacity_t {
                problems.push(format!(
                    "{id} carries {} and {} over its room {}",
                    ship.supply_t, ship.cargo_fuel_t, class.cargo_capacity_t
                ));
            }
            if let Some(troops) = ship.troops {
                if troops.men > class.troop_capacity_men {
                    problems.push(format!(
                        "{id} carries {} men over its capacity {}",
                        troops.men, class.troop_capacity_men
                    ));
                }
                if troops.men == 0 {
                    problems.push(format!("{id} carries 0 men and says it has troops"));
                }
                if ship.location == ShipLocation::Sunk {
                    problems.push(format!("{id} is sunk and carries {} men", troops.men));
                }
            }
        }
        if let Some(old) = before.and_then(|b| b.ships.get(&ship.id)) {
            if ship.gun_rounds > old.gun_rounds {
                problems.push(format!(
                    "{id} gained gun rounds: {} to {}",
                    old.gun_rounds, ship.gun_rounds
                ));
            }
            if old.location == ShipLocation::Sunk && ship.location != ShipLocation::Sunk {
                problems.push(format!("{id} came back from the dead"));
            }
        }
    }
    for task_force in truth.task_forces.values() {
        if task_force.fatigue < Fixed::ZERO || task_force.fatigue > hundred() {
            problems.push(format!(
                "{} has fatigue {}",
                task_force.id, task_force.fatigue
            ));
        }
        if let Some(preparation) = &task_force.preparation {
            if preparation.loading < Fixed::ZERO || preparation.loading > hundred() {
                problems.push(format!(
                    "{} has loading {}",
                    task_force.id, preparation.loading
                ));
            }
        }
    }
    for base in truth.bases.values() {
        let id = base.id;
        if base.supply_t.is_negative() {
            problems.push(format!("{id} has negative supply {}", base.supply_t));
        }
        if base.fuel_t.is_negative() {
            problems.push(format!("{id} has negative fuel {}", base.fuel_t));
        }
        if base.defences < Fixed::ZERO || base.defences > hundred() {
            problems.push(format!("{id} has defences {}", base.defences));
        }
        if base.airfield_damage < Fixed::ZERO || base.airfield_damage > hundred() {
            problems.push(format!("{id} has airfield damage {}", base.airfield_damage));
        }
        if !base.airfield && !base.airfield_damage.is_zero() {
            problems.push(format!("{id} has damage to an airfield it has not"));
        }
    }
    for group in truth.air_groups.values() {
        let id = group.id;
        if group.fatigue < Fixed::ZERO || group.fatigue > hundred() {
            problems.push(format!("{id} has fatigue {}", group.fatigue));
        }
        if let Some(rest) = group.rest_above {
            if rest < Fixed::ZERO || rest > hundred() {
                problems.push(format!("{id} has rest threshold {rest}"));
            }
        }
        if group.ready + group.damaged > group.strength {
            problems.push(format!(
                "{id} has {} aircraft over its strength {}",
                group.ready + group.damaged,
                group.strength
            ));
        }
        if group.pilots.is_some_and(|p| p > group.strength) {
            problems.push(format!("{id} has more pilots than its strength"));
        }
    }
    for unit in truth.land_units.values() {
        let id = unit.id;
        if unit.fatigue < Fixed::ZERO || unit.fatigue > hundred() {
            problems.push(format!("{id} has fatigue {}", unit.fatigue));
        }
        if unit.fed_share < Fixed::ZERO || unit.fed_share > Fixed::ONE {
            problems.push(format!("{id} has fed share {}", unit.fed_share));
        }
        if unit.rehearsal < Fixed::ZERO || unit.rehearsal > hundred() {
            problems.push(format!("{id} has rehearsal {}", unit.rehearsal));
        }
        if unit.experience > 4 {
            problems.push(format!("{id} has experience {}", unit.experience));
        }
        if unit.fieldworks < Fixed::ZERO || unit.fieldworks > ground.fieldworks_limit {
            problems.push(format!("{id} has fieldworks {}", unit.fieldworks));
        }
        // A land unit is in one place: a hex when it has men ashore, and none
        // when it has not.
        if unit.hex.is_some() != (unit.men_ashore > 0) {
            problems.push(format!(
                "{id} has {} men ashore and hex {:?}",
                unit.men_ashore, unit.hex
            ));
        }
    }
    // A ship has at most one commitment on a day, in each side's command
    // state.
    for side in Side::ALL {
        let state = world.command_state(side);
        for (ship, commitments) in &state.commitments {
            for (i, a) in commitments.iter().enumerate() {
                for b in &commitments[i + 1..] {
                    if a.from_day <= b.to_day && b.from_day <= a.to_day {
                        problems.push(format!(
                            "{ship} has two commitments on one day: {a:?} and {b:?}"
                        ));
                    }
                }
            }
        }
    }
    problems
}

/// The checks of one day that read the day's events, the staff's hand-in and
/// the holds of the morning. Returns the problems found.
#[allow(clippy::too_many_lines)]
fn check_events(
    world: &World,
    truth: &TruthState,
    day: u32,
    events: &[Event],
    handed: &BTreeMap<Side, StaffDecisions>,
    held: &BTreeMap<Side, Held>,
) -> Vec<String> {
    let mut problems = Vec::new();
    for event in events {
        // Every event names causes that exist and come before it.
        for cause in &event.causes {
            match cause {
                Cause::Event(id) => {
                    if *id >= event.id || world.events().get(*id).is_none() {
                        problems.push(format!(
                            "event {} ({}) cites a cause event that does not come before it",
                            ev(world, event),
                            event.kind
                        ));
                    }
                }
                Cause::Order(id) => {
                    if world.order(*id).is_none() {
                        problems.push(format!(
                            "event {} cites an order that does not exist",
                            ev(world, event)
                        ));
                    }
                }
            }
        }
        match &event.kind {
            // No order of a staff is ever refused, and none of its commands.
            EventKind::OrderRejected { order, reason, .. } => {
                if world
                    .order(*order)
                    .is_some_and(|o| o.origin.issuer == Issuer::Staff)
                {
                    problems.push(format!(
                        "day {day}, event {}: a staff order is refused: {reason:?}",
                        ev(world, event)
                    ));
                }
            }
            EventKind::CommandRefused {
                issuer: Issuer::Staff,
                command,
                reason,
                ..
            } => problems.push(format!(
                "day {day}, event {}: a staff command is refused: {command:?}: {reason:?}",
                ev(world, event)
            )),
            // A staff's load never takes a stock below its reserve.
            EventKind::CargoLoaded {
                base,
                cargo: Cargo::Supply,
                base_stock_t,
                ..
            } => {
                let by_staff = event.causes.iter().any(|c| match c {
                    Cause::Order(id) => world
                        .order(*id)
                        .is_some_and(|o| o.origin.issuer == Issuer::Staff),
                    Cause::Event(_) => false,
                });
                if let Some(b) = truth.bases.get(base) {
                    if by_staff && *base_stock_t < b.supply_reserve_t {
                        problems.push(format!(
                            "day {day}, event {}: a staff load leaves {base} with {base_stock_t} \
                             under its reserve {}",
                            ev(world, event),
                            b.supply_reserve_t
                        ));
                    }
                }
            }
            // Refuelling never takes fuel below the reserve.
            EventKind::ShipRefuelled {
                base, base_fuel_t, ..
            } => {
                if let Some(b) = truth.bases.get(base) {
                    if *base_fuel_t < b.fuel_reserve_t {
                        problems.push(format!(
                            "day {day}, event {}: refuelling leaves {base} with {base_fuel_t} \
                             fuel under its reserve {}",
                            ev(world, event),
                            b.fuel_reserve_t
                        ));
                    }
                }
            }
            // Every interrupt that fired agrees with its status.
            EventKind::InterruptFired {
                side,
                interrupt,
                condition,
                figure,
            } => {
                let state = world.command_state(*side);
                let status = match interrupt {
                    InterruptRef::Set(id) => state.interrupts.get(id).map(|s| s.status),
                    InterruptRef::Standing { kind, subject } => {
                        state.standing.get(&(*kind, *subject)).copied()
                    }
                };
                match status {
                    None => problems.push(format!(
                        "day {day}, event {}: {interrupt} fired and has no status",
                        ev(world, event)
                    )),
                    Some(s) => {
                        if s.last_fired.is_some_and(|d| d > day) {
                            problems.push(format!("{interrupt} last fired after today"));
                        }
                    }
                }
                if let (Some(InterruptCondition::SupplyBelow { days, .. }), Some(figure)) =
                    (condition, figure)
                {
                    if *figure >= Fixed::from_int(i64::from(*days)) {
                        problems.push(format!(
                            "day {day}, event {}: {interrupt} fired at {figure} days, not below {days}",
                            ev(world, event)
                        ));
                    }
                }
            }
            _ => {}
        }
    }
    // Interrupt statuses agree with themselves: an interrupt not armed has
    // fired.
    for side in Side::ALL {
        let state = world.command_state(side);
        for set in state.interrupts.values() {
            let s = set.status;
            if !s.armed && s.last_fired.is_none() {
                problems.push(format!("{} is not armed and never fired", set.id));
            }
            if s.last_fired.is_some_and(|d| d > day) {
                problems.push(format!("{} last fired after today", set.id));
            }
        }
        for ((kind, subject), s) in &state.standing {
            if !s.armed && s.last_fired.is_none() {
                problems.push(format!(
                    "standing {} for {subject} is not armed and never fired",
                    kind.name()
                ));
            }
        }
    }
    // The staff gives no order to what is held.
    for (side, decisions) in handed {
        let held = held.get(side).cloned().unwrap_or_default();
        for order in &decisions.orders {
            if held.reaches(&order.kind) {
                problems.push(format!(
                    "day {day}: the {} staff gave an order to something held: {:?}",
                    side.name(),
                    order.kind
                ));
            }
        }
    }
    problems
}

/// What the books say against what the state says, at the close of a day.
fn check_books(books: &Books, truth: &TruthState) -> Vec<String> {
    let mut problems = Vec::new();
    for base in truth.bases.values() {
        let id = base.id.raw();
        let supply = books.base_supply.get(&id).copied().unwrap_or(Fixed::ZERO);
        if supply != base.supply_t {
            problems.push(format!(
                "base {id}: supply is {} and the events make it {supply}",
                base.supply_t
            ));
        }
        let fuel = books.base_fuel.get(&id).copied().unwrap_or(Fixed::ZERO);
        if fuel != base.fuel_t {
            problems.push(format!(
                "base {id}: fuel is {} and the events make it {fuel}",
                base.fuel_t
            ));
        }
    }
    for ship in truth.ships.values() {
        let id = ship.id.raw();
        for (name, books, now) in [
            ("fuel", &books.ship_fuel, ship.fuel_t),
            ("supply aboard", &books.ship_supply, ship.supply_t),
            (
                "cargo fuel aboard",
                &books.ship_cargo_fuel,
                ship.cargo_fuel_t,
            ),
        ] {
            let expected = books.get(&id).copied().unwrap_or(Fixed::ZERO);
            if expected != now {
                problems.push(format!(
                    "ship {id}: {name} is {now} and the events make it {expected}"
                ));
            }
        }
    }
    let now = men_in_state(truth);
    for side in Side::ALL {
        let i = side_index(side);
        if books.men_start[i] != now[i] + books.men_lost[i] {
            problems.push(format!(
                "{} men: started with {}, lost {}, have {}",
                side.name(),
                books.men_start[i],
                books.men_lost[i],
                now[i]
            ));
        }
    }
    problems
}

// ---------------------------------------------------------------------
// One run.
// ---------------------------------------------------------------------

fn staff_day(handed: &BTreeMap<Side, StaffDecisions>, side: Side) -> StaffDay {
    let Some(d) = handed.get(&side) else {
        return StaffDay::default();
    };
    StaffDay {
        orders: d.orders.len(),
        no_orders: d.no_orders.len(),
        load_to: d.orders.iter().any(|o| {
            matches!(
                o.kind,
                OrderKind::TaskForce {
                    order: TaskForceOrder::LoadTo { .. },
                    ..
                }
            )
        }),
        cannot_meet: d
            .notices
            .iter()
            .any(|n| matches!(n.kind, sim_core::command::NoticeKind::CannotMeet)),
    }
}

/// Runs the scenario for its days with the staffs of both sides passed in,
/// checking the day at its close.
fn run(seed: u64) -> Run {
    let mut scenario = load(seed);
    let start_truth = scenario.world.umpire().clone();
    let start = picture(&start_truth);
    let held_staffs = staffs();
    let mut books = Books::new(&start_truth);
    let mut hashes = Vec::new();
    let mut days = Vec::new();
    let mut staff = Vec::new();
    let mut before = start_truth;
    let mut problems: Vec<String> = Vec::new();
    let mut unlinked_hexes = BTreeSet::new();
    let mut sailed_short = Vec::new();
    let started = Stopwatch::start();
    let mut day_seconds = Vec::new();
    let mut day_started = Stopwatch::start();
    loop {
        let day = scenario.world.day() + 1;
        let first_event = scenario.world.events().len();
        let mut held: BTreeMap<Side, Held> = BTreeMap::new();
        let summary = scenario
            .next_day_watched(as_staffs(&held_staffs), &mut |world, moment| {
                if moment == (Moment::Orders { day }) {
                    for side in Side::ALL {
                        held.insert(side, Held::read(world.side_view(side)));
                    }
                }
            })
            .unwrap_or_else(|e| panic!("seed {seed}: day {day} does not resolve: {e}"));
        let Some(summary) = summary else { break };
        day_seconds.push(day_started.seconds());
        hashes.push(summary.state_hash);
        let world = &scenario.world;
        let truth = world.umpire();
        let events: Vec<Event> = world.events().from_index(first_event).to_vec();
        books.take(&events, truth);
        let mut today = out_of_range(world, truth, Some(&before));
        today.extend(check_books(&books, truth));
        today.extend(check_events(
            world,
            truth,
            summary.day,
            &events,
            world.staff_handed_in(),
            &held,
        ));
        // Where the surrenders were, and the forces that sailed short.
        for event in &events {
            match &event.kind {
                EventKind::UnitSurrendered { hex, .. } => {
                    if world.map().land_neighbours(*hex).is_empty() {
                        unlinked_hexes.insert(*hex);
                    }
                }
                EventKind::TaskForceSteamed {
                    task_force, from, ..
                } => {
                    if let Some(force) = before.task_forces.get(task_force) {
                        let at_own_base = before
                            .bases
                            .values()
                            .any(|b| b.hex == *from && b.side == force.side);
                        let tanks_not_full = force.ships.iter().any(|s| {
                            before.ships.get(s).is_some_and(|ship| {
                                world
                                    .ref_data()
                                    .ship_classes
                                    .get(&ship.class)
                                    .is_some_and(|c| ship.fuel_t < c.fuel_capacity_t)
                            })
                        });
                        let was_stopped = matches!(
                            force.status,
                            TaskForceStatus::Idle
                                | TaskForceStatus::Arrived
                                | TaskForceStatus::Halted
                        );
                        if at_own_base && tanks_not_full && was_stopped {
                            sailed_short.push(usize::try_from(summary.day).expect("a day"));
                        }
                    }
                }
                _ => {}
            }
        }
        for p in today {
            problems.push(format!("seed {seed}, day {}: {p}", summary.day));
        }
        let handed = world.staff_handed_in();
        staff.push([
            staff_day(handed, Side::Allied),
            staff_day(handed, Side::Japanese),
        ]);
        days.push(picture(truth));
        before = truth.clone();
        day_started = Stopwatch::start();
        if problems.len() > 40 {
            break;
        }
    }
    assert!(
        problems.is_empty(),
        "seed {seed}: {} problems, the first ones:\n{:#?}",
        problems.len(),
        &problems[..problems.len().min(12)]
    );
    let seconds = started.seconds();
    Run {
        seed,
        start,
        hashes,
        days,
        events: scenario.world.events().iter().cloned().collect(),
        staff,
        unlinked_hexes,
        sailed_short,
        problems,
        seconds,
        day_seconds,
    }
}

// ---------------------------------------------------------------------
// Activities, window by window.
// ---------------------------------------------------------------------

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
enum Activity {
    RaidOnABase,
    AirSearchFlown,
    StrikeOnShips,
    SurfaceEngagementBegun,
    ConvoyUnloaded,
    AircraftRepaired,
    ArrivalOrReplacement,
    GroupStoodDown,
    GroupReturned,
    GroundFight,
    MenLanded,
    March,
    MenFed,
    MenUnfed,
    UnitWork,
    FireOnMen,
    AirfieldDamaged,
    FieldworksDug,
    GroundReport,
    ShipRefuelled,
}

const ACTIVITIES: [(Activity, &str); 20] = [
    (Activity::RaidOnABase, "raid on a base"),
    (Activity::AirSearchFlown, "air search flown"),
    (Activity::StrikeOnShips, "strike on ships"),
    (Activity::SurfaceEngagementBegun, "surface engagement"),
    (Activity::ConvoyUnloaded, "convoy unloaded"),
    (Activity::AircraftRepaired, "aircraft repair"),
    (Activity::ArrivalOrReplacement, "arrival or replacement"),
    (Activity::GroupStoodDown, "group stood down"),
    (Activity::GroupReturned, "group returned"),
    (Activity::GroundFight, "fight ashore"),
    (Activity::MenLanded, "men landed"),
    (Activity::March, "march"),
    (Activity::MenFed, "men fed"),
    (Activity::MenUnfed, "men unfed"),
    (Activity::UnitWork, "unit's work"),
    (Activity::FireOnMen, "fire on men"),
    (Activity::AirfieldDamaged, "airfield damaged"),
    (Activity::FieldworksDug, "fieldworks dug"),
    (Activity::GroundReport, "ground report"),
    (Activity::ShipRefuelled, "ship refuelled"),
];

fn activity(kind: &EventKind) -> Option<Activity> {
    Some(match kind {
        EventKind::BaseStrikeLaunched { .. } => Activity::RaidOnABase,
        EventKind::SearchFlown { .. } => Activity::AirSearchFlown,
        EventKind::StrikeLaunched { .. } | EventKind::StrikeFoundNothing { .. } => {
            Activity::StrikeOnShips
        }
        EventKind::EngagementBegun { .. } => Activity::SurfaceEngagementBegun,
        EventKind::UnloadingComplete { .. } => Activity::ConvoyUnloaded,
        EventKind::AircraftRepaired { .. } => Activity::AircraftRepaired,
        EventKind::AircraftArrived { .. } | EventKind::ReplacementsReceived { .. } => {
            Activity::ArrivalOrReplacement
        }
        EventKind::AirGroupStoodDown { .. } => Activity::GroupStoodDown,
        EventKind::AirGroupReturned { .. } => Activity::GroupReturned,
        EventKind::GroundFight { .. } => Activity::GroundFight,
        EventKind::MenLanded { .. } => Activity::MenLanded,
        EventKind::MarchDay { .. } => Activity::March,
        EventKind::MenFed { fed, .. } if *fed > 0 => Activity::MenFed,
        EventKind::MenFed { unfed, .. } if *unfed > 0 => Activity::MenUnfed,
        EventKind::MenLostUnfed { .. } => Activity::MenUnfed,
        EventKind::UnitWorked { .. }
        | EventKind::AirfieldBuilt { .. }
        | EventKind::AirfieldMended { .. } => Activity::UnitWork,
        EventKind::MenLostToFire { .. } => Activity::FireOnMen,
        EventKind::AirfieldDamaged { .. } => Activity::AirfieldDamaged,
        EventKind::FieldworksChanged {
            change: FieldworksChange::Dug,
            ..
        } => Activity::FieldworksDug,
        EventKind::GroundReportReceived { .. } => Activity::GroundReport,
        EventKind::ShipRefuelled { .. } => Activity::ShipRefuelled,
        _ => return None,
    })
}

/// The window, from 0, of a day from 1 (events before day 1 are in none).
fn window_of(day: usize) -> Option<usize> {
    (1..=DAYS).contains(&day).then(|| (day - 1) / WINDOW)
}

fn activity_counts(run: &Run) -> BTreeMap<Activity, [u32; WINDOWS]> {
    let mut counts: BTreeMap<Activity, [u32; WINDOWS]> =
        ACTIVITIES.iter().map(|(a, _)| (*a, [0; WINDOWS])).collect();
    for event in &run.events {
        if let (Some(a), Some(w)) = (activity(&event.kind), window_of(day_of(event))) {
            counts.get_mut(&a).expect("every activity is there")[w] += 1;
        }
    }
    counts
}

/// Days of each window on which a staff did something: `(side, kind)`.
fn staff_counts(run: &Run) -> BTreeMap<&'static str, [u32; WINDOWS]> {
    let mut counts: BTreeMap<&'static str, [u32; WINDOWS]> = BTreeMap::new();
    for key in [
        "Allied staff order given",
        "Japanese staff order given",
        "Allied staff day with no order",
        "Japanese staff day with no order",
        "Allied staff load to a weight",
        "Japanese staff load to a weight",
        "Allied staff cannot meet",
        "Japanese staff cannot meet",
    ] {
        counts.insert(key, [0; WINDOWS]);
    }
    for (i, days) in run.staff.iter().enumerate() {
        let Some(w) = window_of(i + 1) else { continue };
        for (s, name) in [(0, "Allied"), (1, "Japanese")] {
            let d = &days[s];
            let mut bump = |what: &str| {
                let key: &'static str = match (name, what) {
                    ("Allied", "order") => "Allied staff order given",
                    ("Japanese", "order") => "Japanese staff order given",
                    ("Allied", "none") => "Allied staff day with no order",
                    ("Japanese", "none") => "Japanese staff day with no order",
                    ("Allied", "load") => "Allied staff load to a weight",
                    ("Japanese", "load") => "Japanese staff load to a weight",
                    ("Allied", "cannot") => "Allied staff cannot meet",
                    _ => "Japanese staff cannot meet",
                };
                counts.get_mut(key).expect("a key")[w] += 1;
            };
            if d.orders > 0 {
                bump("order");
            }
            if d.orders == 0 && d.no_orders > 0 {
                bump("none");
            }
            if d.load_to {
                bump("load");
            }
            if d.cannot_meet {
                bump("cannot");
            }
        }
    }
    counts
}

fn events_by_window(run: &Run) -> [usize; WINDOWS] {
    let mut counts = [0; WINDOWS];
    for event in &run.events {
        if let Some(w) = window_of(day_of(event)) {
            counts[w] += 1;
        }
    }
    counts
}

// ---------------------------------------------------------------------
// What happens at least once, and the day it first does.
// ---------------------------------------------------------------------

/// Every thing the scenario makes happen at least once, with the first day it
/// does in the run (`None` if it never does).
#[allow(clippy::too_many_lines)]
fn firsts(run: &Run) -> Vec<(&'static str, Option<usize>)> {
    let first = |holds: &dyn Fn(&Event) -> bool| run.events.iter().find(|e| holds(e)).map(day_of);
    let ground_strain = Fixed::from_int(60);
    let mut firsts: Vec<(&'static str, Option<usize>)> = vec![(
        "a beaten side falls back along a land link",
        first(&|e| matches!(e.kind, EventKind::UnitFellBack { .. })),
    )];
    firsts.push((
        "a unit surrenders where no land link leaves its hex",
        first(&|e| match &e.kind {
            EventKind::UnitSurrendered { hex, .. } => run.unlinked_hexes.contains(hex),
            _ => false,
        }),
    ));
    firsts.push((
        "a unit surrenders in a linked hex with no hex free",
        first(&|e| match &e.kind {
            EventKind::UnitSurrendered { hex, .. } => !run.unlinked_hexes.contains(hex),
            _ => false,
        }),
    ));
    firsts.push((
        "an attack fails",
        first(&|e| matches!(e.kind, EventKind::GroundAttackFailed { .. })),
    ));
    let spent_fell: BTreeSet<sim_core::ids::EventId> = run
        .events
        .iter()
        .filter_map(|e| match e.kind {
            EventKind::SpentUnitFellBack { .. } => e.causes.iter().find_map(|c| match c {
                Cause::Event(id) => Some(*id),
                Cause::Order(_) => None,
            }),
            _ => None,
        })
        .collect();
    firsts.push((
        "a spent attacker holds",
        first(&|e| {
            // A back-to-defend with no SpentUnitFellBack citing it: a hold (or a
            // unit with no enemy to face; the scenario's spent attackers face
            // one).
            matches!(e.kind, EventKind::LandUnitBackToDefend { .. }) && !spent_fell.contains(&e.id)
        }),
    ));
    firsts.push((
        "a spent attacker falls back",
        first(&|e| matches!(e.kind, EventKind::SpentUnitFellBack { .. })),
    ));
    firsts.push((
        "a base changes side ashore",
        first(&|e| matches!(e.kind, EventKind::BaseChangedSide { .. })),
    ));
    firsts.push((
        "a unit at the limit of strain",
        first(&|e| match e.kind {
            EventKind::LandUnitFatigue {
                change: LandFatigueChange::Strained,
                fatigue,
                ..
            } => fatigue >= ground_strain,
            _ => false,
        }),
    ));
    firsts.push((
        "fieldworks at their limit",
        first(&|e| match e.kind {
            EventKind::FieldworksChanged {
                change: FieldworksChange::Dug,
                fieldworks,
                ..
            } => fieldworks >= Fixed::from_int(30),
            _ => false,
        }),
    ));
    firsts.push((
        "fieldworks worn by fire",
        first(&|e| {
            matches!(
                e.kind,
                EventKind::FieldworksChanged {
                    change: FieldworksChange::Worn,
                    ..
                }
            )
        }),
    ));
    firsts.push((
        "an airfield refuses landings for damage",
        first(&|e| match e.kind {
            EventKind::AirfieldDamaged { damage, .. } => damage >= Fixed::from_int(50),
            _ => false,
        }),
    ));
    let refused = first(&|e| match e.kind {
        EventKind::AirfieldDamaged { damage, .. } => damage >= Fixed::from_int(50),
        _ => false,
    });
    firsts.push((
        "an airfield refused for damage is mended below the level",
        run.events
            .iter()
            .find(|e| {
                refused.is_some_and(|d| day_of(e) >= d)
                    && matches!(&e.kind, EventKind::AirfieldMended { damage, .. }
                        if *damage < Fixed::from_int(50))
            })
            .map(day_of),
    ));
    firsts.push((
        "a base's defences raised past the prepared grade",
        run.days
            .iter()
            .position(|p| {
                p.base_defences.iter().any(|(b, d)| {
                    *d >= Fixed::from_int(30)
                        && run
                            .start
                            .base_defences
                            .get(b)
                            .is_some_and(|s| *s < Fixed::from_int(30))
                })
            })
            .map(|i| i + 1),
    ));
    // A prepared landing: a force that gained loading, and then landed men; an
    // unprepared one: a force that landed men having gained none.
    let mut loaded: BTreeSet<TaskForceId> = BTreeSet::new();
    let mut prepared = None;
    let mut unprepared = None;
    for e in &run.events {
        match &e.kind {
            EventKind::LoadingGained { task_force, .. } => {
                loaded.insert(*task_force);
            }
            EventKind::MenLanded { task_force, .. } => {
                if loaded.contains(task_force) {
                    prepared.get_or_insert(day_of(e));
                } else {
                    unprepared.get_or_insert(day_of(e));
                }
            }
            _ => {}
        }
    }
    firsts.push((
        "loading gained",
        first(&|e| matches!(e.kind, EventKind::LoadingGained { .. })),
    ));
    firsts.push((
        "rehearsal gained",
        first(&|e| matches!(e.kind, EventKind::RehearsalGained { .. })),
    ));
    firsts.push(("a prepared landing", prepared));
    firsts.push(("an unprepared landing", unprepared));
    firsts.push((
        "a unit trained up a level",
        first(&|e| {
            matches!(
                e.kind,
                EventKind::ExperienceRaised {
                    by: ExperienceSource::Training,
                    ..
                }
            )
        }),
    ));
    firsts.push((
        "men lost unfed",
        first(&|e| matches!(e.kind, EventKind::MenLostUnfed { .. })),
    ));
    firsts.push((
        "a force sails short of fuel",
        run.sailed_short.first().copied(),
    ));
    firsts.push((
        "a ship held at a fuel reserve",
        first(&|e| matches!(e.kind, EventKind::RefuellingHeldAtReserve { .. })),
    ));
    firsts.push((
        "a hex found empty",
        first(&|e| matches!(e.kind, EventKind::GroundFoundEmpty { .. })),
    ));
    firsts.push((
        "a task force out of fuel",
        first(&|e| matches!(e.kind, EventKind::TaskForceOutOfFuel { .. })),
    ));
    firsts.push((
        "a commander's order overrides the staff (HeldByOrder)",
        first(&|e| matches!(e.kind, EventKind::HeldByOrder { .. })),
    ));
    firsts.push((
        "a staff cannot meet an intent (a notice)",
        run.staff
            .iter()
            .position(|d| d[0].cannot_meet || d[1].cannot_meet)
            .map(|i| i + 1),
    ));
    firsts.push((
        "a standing interrupt fires (cannot_meet)",
        first(&|e| {
            matches!(
                e.kind,
                EventKind::InterruptFired {
                    interrupt: InterruptRef::Standing {
                        kind: StandingKind::CannotMeet,
                        ..
                    },
                    ..
                }
            )
        }),
    ));
    firsts.push((
        "a set interrupt fires (days of supply below a number)",
        first(&|e| {
            matches!(
                e.kind,
                EventKind::InterruptFired {
                    interrupt: InterruptRef::Set(_),
                    ..
                }
            )
        }),
    ));
    firsts.push((
        "a run ended by a hold (a run-ended notice)",
        first(&|e| {
            matches!(
                e.kind,
                EventKind::StaffNotice {
                    kind: sim_core::command::NoticeKind::RunEnded,
                    ..
                }
            )
        }),
    ));
    firsts.push((
        "a plan is cancelled",
        first(&|e| match &e.kind {
            EventKind::CommandTaken { command, .. } => {
                matches!(command, CommandKind::CancelPlan { .. })
            }
            _ => false,
        }),
    ));
    firsts.push((
        "a scripted command is amended into a plan",
        first(&|e| matches!(e.kind, EventKind::PlanAmended { .. })),
    ));
    firsts.push((
        "a task force out of every ship",
        run.days
            .iter()
            .position(|p| {
                p.empty_forces
                    .difference(&run.start.empty_forces)
                    .next()
                    .is_some()
            })
            .map(|i| i + 1),
    ));
    firsts.push((
        "a ship has no fuel",
        run.days
            .iter()
            .position(|p| !p.dry_ships.is_empty())
            .map(|i| i + 1),
    ));
    firsts.push((
        "an air group that had aircraft has none",
        run.days
            .iter()
            .position(|p| {
                p.groups.iter().any(|(g, now)| {
                    run.start.groups.get(g).is_some_and(|s| s.aircraft() > 0) && now.aircraft() == 0
                })
            })
            .map(|i| i + 1),
    ));
    let comes_about = |holds: &dyn Fn(&GroupState) -> bool| {
        (1..run.days.len())
            .find(|&i| {
                run.days[i]
                    .groups
                    .iter()
                    .any(|(g, now)| holds(now) && !run.days[i - 1].groups.get(g).is_some_and(holds))
            })
            .map(|i| i + 1)
    };
    firsts.push((
        "an air group has fewer pilots than aircraft",
        comes_about(&|g| g.pilots.is_some_and(|n| n > 0 && n < g.aircraft())),
    ));
    firsts.push((
        "an air group has aircraft and no pilots",
        comes_about(&|g| g.pilots == Some(0) && g.aircraft() > 0),
    ));
    firsts
}

// ---------------------------------------------------------------------
// Conservation of aircraft and pilots, from the events and the final state.
// ---------------------------------------------------------------------

fn assert_aircraft_conserved(run: &Run) {
    let seed = run.seed;
    let end = run.end();
    for (g, now) in &end.groups {
        let now = *now;
        let start = run.start.groups.get(g).copied().unwrap_or(GroupState {
            ready: 0,
            damaged: 0,
            pilots: now.pilots.map(|_| 0),
            strength: 0,
        });
        let start = &start;
        let (mut arrived, mut received, mut shot, mut ground, mut written_off, mut with_ship) =
            (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
        let (mut pilots_arrived, mut pilots_received, mut pilots_lost) = (0u32, 0u32, 0u32);
        let mut fuel = 0u32;
        let (mut no_deck, mut landed_out, mut landed_in) = (0u32, 0u32, 0u32);
        let (mut pilots_out, mut pilots_in) = (0u32, 0u32);
        for event in &run.events {
            match event.kind {
                EventKind::AircraftArrived {
                    air_group,
                    aircraft,
                    pilots,
                    ..
                } if air_group.raw() == *g => {
                    arrived += aircraft;
                    pilots_arrived += pilots;
                }
                EventKind::ReplacementsReceived {
                    air_group,
                    aircraft,
                    pilots,
                    ..
                } if air_group.raw() == *g => {
                    received += aircraft;
                    pilots_received += pilots;
                }
                EventKind::AircraftLost {
                    air_group,
                    aircraft,
                    ..
                } if air_group.raw() == *g => shot += aircraft,
                EventKind::AircraftDestroyedOnGround {
                    air_group,
                    aircraft,
                    ..
                }
                | EventKind::DamagedAircraftDestroyedOnGround {
                    air_group,
                    aircraft,
                    ..
                } if air_group.raw() == *g => ground += aircraft,
                EventKind::AircraftWorn {
                    air_group,
                    written_off: w,
                    ..
                } if air_group.raw() == *g => written_off += w,
                EventKind::AirGroupLost {
                    air_group,
                    aircraft,
                    ..
                }
                | EventKind::AirGroupLostWithBase {
                    air_group,
                    aircraft,
                    ..
                } if air_group.raw() == *g => with_ship += aircraft,
                EventKind::PilotsLost {
                    air_group, lost, ..
                } if air_group.raw() == *g => pilots_lost += lost,
                EventKind::AircraftDitched {
                    air_group, ditched, ..
                } if air_group.raw() == *g => fuel += ditched,
                EventKind::AircraftDitchedNoDeck {
                    air_group,
                    aircraft,
                    ..
                } if air_group.raw() == *g => no_deck += aircraft,
                EventKind::AircraftLandedElsewhere {
                    from,
                    to,
                    aircraft,
                    pilots,
                    ..
                } => {
                    if from.raw() == *g {
                        landed_out += aircraft;
                        pilots_out += pilots;
                    }
                    if to.raw() == *g {
                        landed_in += aircraft;
                        pilots_in += pilots;
                    }
                }
                _ => {}
            }
        }
        assert_eq!(
            start.aircraft() + arrived + received + landed_in,
            now.aircraft() + shot + ground + written_off + with_ship + fuel + no_deck + landed_out,
            "seed {seed}, air group {g}: aircraft started {} + arrived {arrived} + received \
             {received} + landed to it {landed_in} = has {} + shot {shot} + on the ground \
             {ground} + written off {written_off} + lost with a ship or a base {with_ship} + out \
             of fuel {fuel} + no deck {no_deck} + landed from it {landed_out}",
            start.aircraft(),
            now.aircraft()
        );
        match (start.pilots, now.pilots) {
            (Some(p), Some(q)) => assert_eq!(
                p + pilots_arrived + pilots_received + pilots_in,
                q + pilots_lost + pilots_out,
                "seed {seed}, air group {g}: pilots"
            ),
            (None, None) => {}
            other => panic!("seed {seed}, air group {g}: pilots tracked changed: {other:?}"),
        }
    }
}

/// Every check that holds of every run of any seed (the day's checks have
/// been made as the run went).
fn check_every_run(run: &Run) {
    assert_eq!(run.hashes.len(), DAYS, "seed {}", run.seed);
    assert_eq!(run.days.len(), DAYS, "seed {}", run.seed);
    assert_eq!(run.staff.len(), DAYS, "seed {}", run.seed);
    assert!(run.problems.is_empty());
    assert_aircraft_conserved(run);
}

/// Runs seeds `first..=last` on as many threads as the machine has, each seed
/// being its own run (so the result does not depend on the threads).
fn run_seeds(first: u64, last: u64) -> Vec<Run> {
    let seeds: Vec<u64> = (first..=last).collect();
    let threads = std::thread::available_parallelism()
        .map_or(1, std::num::NonZeroUsize::get)
        .min(seeds.len().max(1));
    let mut runs: Vec<Run> = std::thread::scope(|scope| {
        let handles: Vec<_> = (0..threads)
            .map(|t| {
                let mine: Vec<u64> = seeds.iter().copied().skip(t).step_by(threads).collect();
                scope.spawn(move || {
                    mine.into_iter()
                        .map(|seed| {
                            let one = run(seed);
                            check_every_run(&one);
                            one
                        })
                        .collect::<Vec<Run>>()
                })
            })
            .collect();
        handles
            .into_iter()
            .flat_map(|h| h.join().expect("a seed's run does not panic"))
            .collect()
    });
    runs.sort_by_key(|r| r.seed);
    runs
}

// ---------------------------------------------------------------------
// The tests.
// ---------------------------------------------------------------------

#[test]
fn the_scenarios_own_seed_runs_180_days_with_everything_holding() {
    let one = run(1942);
    check_every_run(&one);
    let distinct: BTreeSet<u64> = one.hashes.iter().copied().collect();
    assert_eq!(distinct.len(), DAYS, "every day changes the state");
}

#[test]
fn other_seeds_run_180_days_with_everything_holding() {
    let runs = run_seeds(1, OTHER_SEEDS);
    assert_eq!(runs.len(), usize::try_from(OTHER_SEEDS).expect("small"));
    let ends: BTreeSet<u64> = runs
        .iter()
        .map(|r| *r.hashes.last().expect("days"))
        .collect();
    assert!(ends.len() > 1, "the seed matters");
}

/// Seeds 1 to 100, in release mode by hand:
/// `cargo test --release -p scenario-tests --test scenario_45_soak_ashore -- --ignored`
#[test]
#[ignore = "a hundred 180-day runs; run by hand"]
fn a_hundred_seeds_run_180_days_with_everything_holding() {
    let started = Stopwatch::start();
    let runs = run_seeds(1, 100);
    let events: usize = runs.iter().map(|r| r.events.len()).sum();
    eprintln!("100 seeds, {events} events, {:.1} s", started.seconds());
    assert_eq!(runs.len(), 100);
}

#[test]
fn seed_1942_run_twice_gives_the_same_180_daily_hashes_and_events() {
    let a = run(1942);
    let b = run(1942);
    assert_eq!(a.hashes.len(), DAYS);
    assert_eq!(a.hashes, b.hashes, "the daily hashes differ");
    assert_eq!(a.events.len(), b.events.len(), "the logs differ in length");
    for (x, y) in a.events.iter().zip(&b.events) {
        assert_eq!(x, y, "events differ");
    }
    let other = run(1);
    assert_ne!(a.hashes, other.hashes, "the seed matters");
}

#[test]
fn every_subsystem_is_in_use_in_every_30_days() {
    let one = run(1942);
    let counts = activity_counts(&one);
    let mut missing = Vec::new();
    for (activity, name) in ACTIVITIES {
        for (w, n) in counts[&activity].iter().enumerate() {
            if *n == 0 {
                missing.push(format!(
                    "no {name} in days {} to {}",
                    w * WINDOW + 1,
                    (w + 1) * WINDOW
                ));
            }
        }
    }
    let staff = staff_counts(&one);
    for (name, windows) in &staff {
        // The Japanese staff has one intent and so many days with no order;
        // what it must do in every window is give orders. The commander's
        // other doings are asked of the Allied staff, which has two intents
        // and a pool too small for both at times.
        let wanted = matches!(
            *name,
            "Allied staff order given"
                | "Japanese staff order given"
                | "Allied staff day with no order"
                | "Allied staff load to a weight"
        );
        if !wanted {
            continue;
        }
        for (w, n) in windows.iter().enumerate() {
            if *n == 0 {
                missing.push(format!(
                    "no {name} in days {} to {}",
                    w * WINDOW + 1,
                    (w + 1) * WINDOW
                ));
            }
        }
    }
    assert!(missing.is_empty(), "{missing:#?}");
}

#[test]
fn each_thing_that_must_happen_happens_and_leaves_the_run_going() {
    let one = run(1942);
    let firsts = firsts(&one);
    let mut missing = Vec::new();
    let mut last = 0;
    for (name, day) in &firsts {
        match day {
            Some(d) => last = last.max(*d),
            None => missing.push(*name),
        }
    }
    for (name, day) in &firsts {
        eprintln!("first day: {name}: {day:?}");
    }
    assert!(missing.is_empty(), "never happens: {missing:#?}");
    // The run goes on: after the last of these days there are days left, and
    // they have events.
    assert!(last < DAYS, "the last of them is on day {last}");
    let after = one.events.iter().filter(|e| day_of(e) > last).count();
    assert!(after > 0, "nothing happens after day {last}");
}

#[test]
fn both_sides_still_have_aircraft_flying_in_the_last_30_days() {
    let one = run(1942);
    let in_last_window = |e: &&Event| window_of(day_of(e)) == Some(WINDOWS - 1);
    let flown_by = |groups: &[u32]| {
        one.events
            .iter()
            .filter(in_last_window)
            .filter(|e| match &e.kind {
                EventKind::BaseStrikeLaunched { air_group, .. }
                | EventKind::SearchFlown { air_group, .. }
                | EventKind::StrikeLaunched { air_group, .. } => groups.contains(&air_group.raw()),
                _ => false,
            })
            .count()
    };
    assert!(
        flown_by(&RABAUL_GROUPS) > 0,
        "Japanese groups at Rabaul fly in the last 30 days"
    );
    assert!(
        flown_by(&HENDERSON_GROUPS) > 0,
        "Allied groups at Henderson Field fly in the last 30 days"
    );
}

/// The scenario's scripted commands are all taken (none is refused) in the
/// seed-1942 run, and every kind of staff command the scenario's script is
/// meant to give is given at least once.
#[test]
fn every_scripted_command_is_taken_in_the_seed_1942_run() {
    let one = run(1942);
    let refused: Vec<String> = one
        .events
        .iter()
        .filter_map(|e| match &e.kind {
            EventKind::CommandRefused {
                command, reason, ..
            } => Some(format!("day {}: {command:?}: {reason:?}", e.day)),
            _ => None,
        })
        .collect();
    assert!(refused.is_empty(), "{refused:#?}");
}

/// The seed-1942 game saved, replayed and its staffs asked again on every
/// day, as `sim-cli replay --check-staff` does it.
#[test]
fn the_saved_game_replays_and_the_staffs_give_what_was_saved() {
    let cargo = || {
        let mut command =
            Command::new(std::env::var("CARGO").unwrap_or_else(|_| "cargo".to_string()));
        command.current_dir(common::data_dir().join(".."));
        command
    };
    let sim_cli = |arguments: &[&str]| {
        let mut command = cargo();
        command.args(["run", "--quiet", "-p", "sim-cli"]);
        if !cfg!(debug_assertions) {
            command.arg("--release");
        }
        command.arg("--").args(arguments);
        command.output().expect("the command runs")
    };
    let dir = std::env::temp_dir().join(format!("flag_plot_s45_{}", std::process::id()));
    std::fs::create_dir_all(&dir).expect("a directory");
    let save = dir.join("game.json");
    let scenario = scenario_path(SOAK);
    let ran = sim_cli(&[
        "run",
        scenario.to_str().expect("a path"),
        "--save",
        save.to_str().expect("a path"),
    ]);
    assert!(
        ran.status.success(),
        "run: {}",
        String::from_utf8_lossy(&ran.stderr)
    );
    let replayed = sim_cli(&["replay", save.to_str().expect("a path"), "--check-staff"]);
    let said = String::from_utf8_lossy(&replayed.stdout).to_string();
    assert!(
        replayed.status.success(),
        "replay: {said} {}",
        String::from_utf8_lossy(&replayed.stderr)
    );
    assert!(
        said.contains("the staff gave what was saved on every day"),
        "{said}"
    );
    std::fs::remove_dir_all(&dir).ok();
}

// ---------------------------------------------------------------------
// Measurements.
// ---------------------------------------------------------------------

/// Prints what the soak does and costs. Run with `--nocapture`; in release
/// mode for the time a player would see.
#[test]
fn print_the_summary() {
    let soak = run(1942);
    let total = soak.events.len();
    eprintln!("== scenario 45, seed 1942 ==");
    eprintln!("events in the log: {total}");
    eprintln!("events by window of 30 days: {:?}", events_by_window(&soak));
    let seconds_by_window: Vec<String> = soak
        .day_seconds
        .chunks(WINDOW)
        .map(|c| format!("{:.3}", c.iter().sum::<f64>()))
        .collect();
    eprintln!("seconds by window of 30 days: {seconds_by_window:?}");
    eprintln!(
        "time for the 180 days, load excluded: {:.3} s",
        soak.seconds
    );
    let counts = activity_counts(&soak);
    eprintln!("window table (days 1-30, 31-60, ..., 151-180):");
    for (activity, name) in ACTIVITIES {
        eprintln!("  {name:<28} {:?}", counts[&activity]);
    }
    for (name, windows) in staff_counts(&soak) {
        eprintln!("  {name:<28} {windows:?}");
    }
    for (name, day) in firsts(&soak) {
        eprintln!("  first day: {name}: {day:?}");
    }
    eprintln!(
        "  unlinked hexes surrendered in: {:?}; sailed short on days {:?}",
        soak.unlinked_hexes, soak.sailed_short
    );
    assert!(total > 0);
}
