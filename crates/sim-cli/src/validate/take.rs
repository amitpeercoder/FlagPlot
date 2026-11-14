//! How a number is taken from one run of a batch for each `take` of
//! `data/validation/README.md`. Every value is a whole number; a run that
//! gives no value (the divisor is nothing) is left out: `None`.

use std::collections::{BTreeMap, BTreeSet};

use sim_core::fixed::Fixed;

use super::input::{AirLoss, Batch, Measure, PerRun};

/// The sides, as the batch file names them.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Side {
    Japanese,
    Allied,
}

impl Side {
    fn parse(text: &str) -> Option<Side> {
        match text {
            "japanese" => Some(Side::Japanese),
            "allied" => Some(Side::Allied),
            _ => None,
        }
    }
}

/// Weapons by the column of `hits_on`: gun, torpedo, bomb.
#[derive(Clone, Copy)]
pub enum Weapon {
    Gun,
    Torpedo,
    Bomb,
}

impl Weapon {
    fn parse(text: &str) -> Option<Weapon> {
        match text {
            "gun" => Some(Weapon::Gun),
            "torpedo" => Some(Weapon::Torpedo),
            "bomb" => Some(Weapon::Bomb),
            _ => None,
        }
    }
}

#[derive(Clone, Copy)]
pub enum Cause {
    ShotDown,
    DestroyedOnGround,
    DamagedDestroyedOnGround,
    LostWithShip,
    WornOut,
    DitchedFuel,
    DitchedNoDeck,
}

impl Cause {
    fn parse(text: &str) -> Option<Cause> {
        Some(match text {
            "shot_down" => Cause::ShotDown,
            "destroyed_on_ground" => Cause::DestroyedOnGround,
            "damaged_destroyed_on_ground" => Cause::DamagedDestroyedOnGround,
            "lost_with_ship" => Cause::LostWithShip,
            "worn_out" => Cause::WornOut,
            "ditched_fuel" => Cause::DitchedFuel,
            "ditched_no_deck" => Cause::DitchedNoDeck,
            _ => return None,
        })
    }

    fn of(self, loss: &AirLoss) -> u64 {
        match self {
            Cause::ShotDown => loss.shot_down,
            Cause::DestroyedOnGround => loss.destroyed_on_ground,
            Cause::DamagedDestroyedOnGround => loss.damaged_destroyed_on_ground,
            Cause::LostWithShip => loss.lost_with_ship,
            Cause::WornOut => loss.worn_out,
            Cause::DitchedFuel => loss.ditched_fuel,
            Cause::DitchedNoDeck => loss.ditched_no_deck,
        }
    }
}

#[derive(Clone, Copy)]
pub enum Claim {
    Sunk,
    Hits,
    ShotDown,
    OnGround,
}

impl Claim {
    fn parse(text: &str) -> Option<Claim> {
        Some(match text {
            "sunk" => Claim::Sunk,
            "hits" => Claim::Hits,
            "shot_down" => Claim::ShotDown,
            "on_ground" => Claim::OnGround,
            _ => return None,
        })
    }
}

/// Who attacks: a side, or named air groups.
pub enum Who {
    Side(Side),
    Groups(BTreeSet<u32>),
}

/// What is attacked: ships of task forces, or a base.
pub enum Target {
    TaskForces(BTreeSet<u32>),
    Base(u32),
}

/// What was really done, for a claim to be divided by.
pub enum Real {
    Ships(BTreeSet<u32>),
    Hits(BTreeSet<u32>, Vec<Weapon>),
    Aircraft(BTreeSet<u32>, Vec<Cause>),
}

/// Whose shots a `shots_in_hundreds` or `shots` counts: a side's, from `fire`, or the
/// named ships', from `shots_by`.
pub enum Fired {
    Side(Side),
    Ships(BTreeSet<u32>),
}

/// What a `shots` take counts in: whole shots, or tens of shots, so that a
/// record of a few dozen shells is not floored to nothing (decision 0084).
#[derive(Clone, Copy)]
pub enum ShotUnit {
    Shells,
    Tens,
}

impl ShotUnit {
    fn divisor(self) -> i128 {
        match self {
            ShotUnit::Shells => 1,
            ShotUnit::Tens => 10,
        }
    }
}

pub enum Take {
    ShipsSunk(BTreeSet<u32>),
    SystemOut(BTreeSet<u32>, String),
    HitsOn(BTreeSet<u32>, Vec<Weapon>),
    HitsPerThousand(Side, Weapon),
    /// A side's shots with a weapon, or those of named ships.
    ShotsInHundreds(Fired, Weapon),
    /// The same, in whole shots or in tens of shots.
    Shots(Fired, Weapon, ShotUnit),
    /// Ships sunk, or afloat with a system out, counted once each.
    SunkOrSystemOut(BTreeSet<u32>, String),
    AirLosses(BTreeSet<u32>, Vec<Cause>),
    PilotsLost(BTreeSet<u32>),
    Ready(BTreeSet<u32>, usize),
    Attacking(Who, Target, bool),
    AttackDays(Who, Target),
    /// Aircraft launched in raids that met a force of these task forces, and
    /// with `escorts` the fighters sent with them.
    TakingPart(Who, BTreeSet<u32>, bool),
    AttackAtLeast(Side, BTreeSet<u32>, u64),
    Claims(Side, Claim),
    ClaimsPerHundred(Side, Claim, Real),
    Minutes,
    FightsByDay,
    /// Night fights whose `sighted_first` is one of these.
    FightsSightedFirst(Vec<String>),
    /// The rounds these task forces were surprised in night fights.
    RoundsSurprised(BTreeSet<u32>),
    /// 1 if the side holds the base at the end of the day.
    BaseHolder {
        base: u32,
        day: u32,
        side: Side,
    },
    /// The side's men lost on the days `from` to `to`, both included.
    MenLost {
        side: Side,
        from: u32,
        to: u32,
        /// Only the men lost ashore in this hex, when given.
        hex: Option<(i32, i32)>,
    },
    /// The day the base's airfield was built.
    AirfieldBuiltDay(u32),
    /// The first day on or after `due` on which aircraft reached the group.
    ArrivalDay {
        air_group: u32,
        due: u32,
    },
    /// Whole days the supply at the side's bases in the hex feeds its men
    /// ashore there, at the end of the day.
    DaysOfFood {
        side: Side,
        q: i32,
        r: i32,
        day: u32,
    },
}

const SYSTEMS: [&str; 5] = [
    "propulsion",
    "fire_control",
    "main_battery",
    "torpedo_mounts",
    "flight_deck",
];

/// Ids a measure names, to be found in the batch's header tables.
struct Known {
    ships: BTreeSet<u32>,
    groups: BTreeSet<u32>,
    forces: BTreeSet<u32>,
    bases: BTreeSet<u32>,
}

fn need<T>(measure: &Measure, key: &str, value: Option<T>) -> Result<T, String> {
    value.ok_or_else(|| {
        format!(
            "measure {}: take {} needs the key {key}",
            measure.id,
            measure.take.as_deref().unwrap_or("")
        )
    })
}

fn ids(
    measure: &Measure,
    key: &str,
    list: &Option<Vec<u32>>,
    known: &BTreeSet<u32>,
    what: &str,
) -> Result<BTreeSet<u32>, String> {
    let list = need(measure, key, list.as_ref())?;
    for id in list {
        if !known.contains(id) {
            return Err(format!(
                "measure {}: {key} names {what} {id}, which the batch does not have",
                measure.id
            ));
        }
    }
    Ok(list.iter().copied().collect())
}

fn side(measure: &Measure) -> Result<Side, String> {
    let text = need(measure, "side", measure.side.as_ref())?;
    Side::parse(text).ok_or_else(|| {
        format!(
            "measure {}: side \"{text}\" is not japanese or allied",
            measure.id
        )
    })
}

fn weapons(measure: &Measure) -> Result<Vec<Weapon>, String> {
    let list = need(measure, "weapons", measure.weapons.as_ref())?;
    list.iter()
        .map(|w| {
            Weapon::parse(w)
                .ok_or_else(|| format!("measure {}: weapon \"{w}\" is not known", measure.id))
        })
        .collect()
}

/// The weapon key of a measure of shots fired: gun or torpedo.
fn fired_weapon(measure: &Measure) -> Result<Weapon, String> {
    let text = need(measure, "weapon", measure.weapon.as_ref())?;
    Weapon::parse(text)
        .filter(|w| matches!(w, Weapon::Gun | Weapon::Torpedo))
        .ok_or_else(|| {
            format!(
                "measure {}: weapon \"{text}\" is not gun or torpedo",
                measure.id
            )
        })
}

fn causes(measure: &Measure) -> Result<Vec<Cause>, String> {
    let list = need(measure, "causes", measure.causes.as_ref())?;
    list.iter()
        .map(|c| {
            Cause::parse(c)
                .ok_or_else(|| format!("measure {}: cause \"{c}\" is not known", measure.id))
        })
        .collect()
}

fn claim(measure: &Measure) -> Result<Claim, String> {
    let text = need(measure, "claim", measure.claim.as_ref())?;
    Claim::parse(text)
        .ok_or_else(|| format!("measure {}: claim \"{text}\" is not known", measure.id))
}

/// Refuses a measure that gives a key its take does not use (`take`, `facts`
/// and the keys every measure has are not counted).
fn no_other_keys(measure: &Measure, allowed: &[&str]) -> Result<(), String> {
    let given = [
        ("ships", measure.ships.is_some()),
        ("system", measure.system.is_some()),
        ("weapons", measure.weapons.is_some()),
        ("weapon", measure.weapon.is_some()),
        ("side", measure.side.is_some()),
        ("air_groups", measure.air_groups.is_some()),
        ("causes", measure.causes.is_some()),
        ("day", measure.day.is_some()),
        ("escorts", measure.escorts.is_some()),
        ("target_task_forces", measure.target_task_forces.is_some()),
        ("target_base", measure.target_base.is_some()),
        ("aircraft", measure.aircraft.is_some()),
        ("claim", measure.claim.is_some()),
        ("by", measure.by.is_some()),
        ("task_forces", measure.task_forces.is_some()),
        ("unit", measure.unit.is_some()),
        ("base", measure.base.is_some()),
        ("air_group", measure.air_group.is_some()),
        ("due_day", measure.due_day.is_some()),
        ("from_day", measure.from_day.is_some()),
        ("to_day", measure.to_day.is_some()),
        ("q", measure.q.is_some()),
        ("r", measure.r.is_some()),
    ];
    let take = measure.take.as_deref().unwrap_or("");
    for (key, present) in given {
        if present && !allowed.contains(&key) {
            return Err(if allowed.is_empty() {
                format!("measure {}: take {take} takes no keys", measure.id)
            } else {
                format!(
                    "measure {}: take {take} does not take the key {key}",
                    measure.id
                )
            });
        }
    }
    Ok(())
}

/// A day key, which must fall within the scenario's days.
fn day_in_scenario(
    measure: &Measure,
    key: &str,
    day: Option<u32>,
    batch: &Batch,
) -> Result<u32, String> {
    let day = need(measure, key, day)?;
    if day < 1 || day > batch.scenario.days {
        return Err(format!(
            "measure {}: {key} {day} is outside the scenario's {} days",
            measure.id, batch.scenario.days
        ));
    }
    Ok(day)
}

/// Reads a measure's `take` and keys, checking each id against the batch.
pub fn parse(measure: &Measure, batch: &Batch) -> Result<Take, String> {
    let known = Known {
        ships: batch.ships.iter().map(|s| s.id).collect(),
        groups: batch.air_groups.iter().map(|g| g.id).collect(),
        forces: batch.task_forces.iter().map(|t| t.id).collect(),
        bases: batch.bases.iter().map(|b| b.id).collect(),
    };
    let take = need(measure, "take", measure.take.as_ref())?;
    if measure.unit.is_some() && take != "shots" {
        return Err(format!(
            "measure {}: take {take} does not take the key unit",
            measure.id
        ));
    }
    let ships = |m: &Measure| ids(m, "ships", &m.ships, &known.ships, "ship");
    let groups = |m: &Measure| ids(m, "air_groups", &m.air_groups, &known.groups, "air group");
    let who_and_target = |m: &Measure| -> Result<(Who, Target), String> {
        let who = match (&m.side, &m.air_groups) {
            (Some(_), None) => Who::Side(side(m)?),
            (None, Some(_)) => Who::Groups(groups(m)?),
            _ => {
                return Err(format!(
                    "measure {}: take {} needs side or air_groups, not both",
                    m.id,
                    m.take.as_deref().unwrap_or("")
                ))
            }
        };
        let target = match (&m.target_task_forces, m.target_base) {
            (Some(_), None) => Target::TaskForces(ids(
                m,
                "target_task_forces",
                &m.target_task_forces,
                &known.forces,
                "task force",
            )?),
            (None, Some(base)) => {
                if !known.bases.contains(&base) {
                    return Err(format!(
                        "measure {}: target_base {base} is not a base of the batch",
                        m.id
                    ));
                }
                Target::Base(base)
            }
            _ => {
                return Err(format!(
                    "measure {}: take {} needs target_task_forces or target_base, not both",
                    m.id,
                    m.take.as_deref().unwrap_or("")
                ))
            }
        };
        Ok((who, target))
    };
    Ok(match take.as_str() {
        "ships_sunk" => Take::ShipsSunk(ships(measure)?),
        "ships_with_system_out" | "ships_sunk_or_system_out" => {
            let system = need(measure, "system", measure.system.as_ref())?;
            if !SYSTEMS.contains(&system.as_str()) {
                return Err(format!(
                    "measure {}: system \"{system}\" is not known",
                    measure.id
                ));
            }
            if take == "ships_sunk_or_system_out" {
                Take::SunkOrSystemOut(ships(measure)?, system.clone())
            } else {
                Take::SystemOut(ships(measure)?, system.clone())
            }
        }
        "hits_on" => Take::HitsOn(ships(measure)?, weapons(measure)?),
        "hits_per_thousand_shots" => Take::HitsPerThousand(side(measure)?, fired_weapon(measure)?),
        "shots_in_hundreds" | "shots" => {
            // `ships` or `side`, never both: with `ships` the named ships'
            // shots are summed, and without it the side's, as before.
            let whose = match (&measure.ships, &measure.side) {
                (Some(_), Some(_)) => {
                    return Err(format!(
                        "measure {}: take {take} takes ships or side, not both",
                        measure.id
                    ))
                }
                (Some(_), None) => Fired::Ships(ships(measure)?),
                (None, _) => Fired::Side(side(measure)?),
            };
            if take == "shots" {
                let unit = match measure.unit.as_deref() {
                    None | Some("shells") => ShotUnit::Shells,
                    Some("tens") => ShotUnit::Tens,
                    Some(other) => {
                        return Err(format!(
                            "measure {}: unit \"{other}\" is not shells or tens",
                            measure.id
                        ))
                    }
                };
                Take::Shots(whose, fired_weapon(measure)?, unit)
            } else {
                Take::ShotsInHundreds(whose, fired_weapon(measure)?)
            }
        }
        "air_losses" => Take::AirLosses(groups(measure)?, causes(measure)?),
        "pilots_lost" => Take::PilotsLost(groups(measure)?),
        "ready" => {
            let day = need(measure, "day", measure.day)?;
            if day < 1 || day > batch.scenario.days {
                return Err(format!(
                    "measure {}: day {day} is outside the scenario's {} days",
                    measure.id, batch.scenario.days
                ));
            }
            Take::Ready(groups(measure)?, (day - 1) as usize)
        }
        "aircraft_attacking" => {
            let (who, target) = who_and_target(measure)?;
            Take::Attacking(who, target, need(measure, "escorts", measure.escorts)?)
        }
        "aircraft_taking_part" => {
            if measure.target_base.is_some() {
                return Err(format!(
                    "measure {}: take aircraft_taking_part does not take the key target_base",
                    measure.id
                ));
            }
            let who = match (&measure.side, &measure.air_groups) {
                (Some(_), None) => Who::Side(side(measure)?),
                (None, Some(_)) => Who::Groups(groups(measure)?),
                _ => {
                    return Err(format!(
                        "measure {}: take aircraft_taking_part needs side or air_groups, not both",
                        measure.id
                    ))
                }
            };
            let forces = ids(
                measure,
                "target_task_forces",
                &measure.target_task_forces,
                &known.forces,
                "task force",
            )?;
            Take::TakingPart(who, forces, need(measure, "escorts", measure.escorts)?)
        }
        "attack_days" => {
            let (who, target) = who_and_target(measure)?;
            Take::AttackDays(who, target)
        }
        "attack_of_at_least" => Take::AttackAtLeast(
            side(measure)?,
            ids(
                measure,
                "target_task_forces",
                &measure.target_task_forces,
                &known.forces,
                "task force",
            )?,
            need(measure, "aircraft", measure.aircraft)?,
        ),
        "claims" => Take::Claims(side(measure)?, claim(measure)?),
        "claims_per_hundred" => {
            let kind = claim(measure)?;
            let real = match kind {
                Claim::Sunk => Real::Ships(ships(measure)?),
                Claim::Hits => Real::Hits(ships(measure)?, weapons(measure)?),
                Claim::ShotDown | Claim::OnGround => {
                    Real::Aircraft(groups(measure)?, causes(measure)?)
                }
            };
            Take::ClaimsPerHundred(side(measure)?, kind, real)
        }
        "minutes_of_action" => Take::Minutes,
        "fights_by_day" => {
            no_other_keys(measure, &[])?;
            Take::FightsByDay
        }
        "fights_sighted_first" => {
            no_other_keys(measure, &["by"])?;
            let by = need(measure, "by", measure.by.as_ref())?;
            let mut seen = BTreeSet::new();
            for word in by {
                if !["allied", "japanese", "both", "neither"].contains(&word.as_str()) {
                    return Err(format!(
                        "measure {}: by names \"{word}\", which is not allied, japanese, both or neither",
                        measure.id
                    ));
                }
                if !seen.insert(word.as_str()) {
                    return Err(format!("measure {}: by names \"{word}\" twice", measure.id));
                }
            }
            if by.is_empty() {
                return Err(format!("measure {}: by is empty", measure.id));
            }
            Take::FightsSightedFirst(by.clone())
        }
        "rounds_surprised" => {
            no_other_keys(measure, &["task_forces"])?;
            let forces = ids(
                measure,
                "task_forces",
                &measure.task_forces,
                &known.forces,
                "task force",
            )?;
            if forces.is_empty() {
                return Err(format!("measure {}: task_forces is empty", measure.id));
            }
            Take::RoundsSurprised(forces)
        }
        "base_holder" => {
            no_other_keys(measure, &["base", "day", "side"])?;
            let base = need(measure, "base", measure.base)?;
            if !known.bases.contains(&base) {
                return Err(format!(
                    "measure {}: base {base} is not a base of the batch",
                    measure.id
                ));
            }
            let day = day_in_scenario(measure, "day", measure.day, batch)?;
            Take::BaseHolder {
                base,
                day,
                side: side(measure)?,
            }
        }
        "men_lost" => {
            no_other_keys(measure, &["side", "from_day", "to_day", "q", "r"])?;
            let hex = match (measure.q, measure.r) {
                (Some(q), Some(r)) => Some((q, r)),
                (None, None) => None,
                (Some(_), None) => return Err(need(measure, "r", None::<i32>).unwrap_err()),
                (None, Some(_)) => return Err(need(measure, "q", None::<i32>).unwrap_err()),
            };
            let from = day_in_scenario(measure, "from_day", measure.from_day, batch)?;
            let to = day_in_scenario(measure, "to_day", measure.to_day, batch)?;
            if from > to {
                return Err(format!(
                    "measure {}: from_day {from} is after to_day {to}",
                    measure.id
                ));
            }
            Take::MenLost {
                side: side(measure)?,
                from,
                to,
                hex,
            }
        }
        "airfield_built_day" => {
            no_other_keys(measure, &["base"])?;
            let base = need(measure, "base", measure.base)?;
            if !known.bases.contains(&base) {
                return Err(format!(
                    "measure {}: base {base} is not a base of the batch",
                    measure.id
                ));
            }
            Take::AirfieldBuiltDay(base)
        }
        "arrival_day" => {
            no_other_keys(measure, &["air_group", "due_day"])?;
            let air_group = need(measure, "air_group", measure.air_group)?;
            if !known.groups.contains(&air_group) {
                return Err(format!(
                    "measure {}: air_group names air group {air_group}, which the batch does not have",
                    measure.id
                ));
            }
            let due = day_in_scenario(measure, "due_day", measure.due_day, batch)?;
            Take::ArrivalDay { air_group, due }
        }
        "days_of_food" => {
            no_other_keys(measure, &["side", "q", "r", "day"])?;
            if batch.daily_supply_t_per_man.is_none() {
                return Err(format!(
                    "measure {}: take days_of_food needs the ground figures, which the batch's scenario does not have",
                    measure.id
                ));
            }
            Take::DaysOfFood {
                side: side(measure)?,
                q: need(measure, "q", measure.q)?,
                r: need(measure, "r", measure.r)?,
                day: day_in_scenario(measure, "day", measure.day, batch)?,
            }
        }
        other => {
            return Err(format!(
                "measure {}: take \"{other}\" is not known",
                measure.id
            ))
        }
    })
}

/// What a run needs beyond itself: the header's tables.
pub struct Context {
    /// Task force of each ship at the start.
    pub force_of: BTreeMap<u32, Option<u32>>,
    /// Side of each air group of the header.
    pub side_of: BTreeMap<u32, Side>,
    /// Minutes a surface-combat round stands for.
    pub round_minutes: Option<u64>,
    /// The side that holds each base of the header as the scenario starts.
    pub base_side: BTreeMap<u32, Side>,
    /// Tons of supply a man ashore costs a day, from the ground figures.
    pub per_man: Option<Fixed>,
}

impl Context {
    pub fn new(batch: &Batch, round_minutes: Option<u64>) -> Context {
        Context {
            force_of: batch.ships.iter().map(|s| (s.id, s.task_force)).collect(),
            side_of: batch
                .air_groups
                .iter()
                .filter_map(|g| Side::parse(&g.side).map(|s| (g.id, s)))
                .collect(),
            round_minutes,
            base_side: batch
                .bases
                .iter()
                .filter_map(|b| Some((b.id, Side::parse(b.side.as_deref()?)?)))
                .collect(),
            per_man: batch
                .daily_supply_t_per_man
                .as_deref()
                .and_then(|text| text.parse().ok()),
        }
    }
}

/// Whether `group`, or any group it was formed from in this run, is in `set`.
fn in_set(run: &PerRun, set: &BTreeSet<u32>, group: u32) -> bool {
    let from: BTreeMap<u32, u32> = run.formed_groups.iter().map(|f| (f.id, f.from)).collect();
    let mut here = group;
    for _ in 0..=from.len() {
        if set.contains(&here) {
            return true;
        }
        match from.get(&here) {
            Some(&source) if source != 0 => here = source,
            _ => return false,
        }
    }
    false
}

fn side_of_group(ctx: &Context, run: &PerRun, group: u32) -> Option<Side> {
    ctx.side_of.get(&group).copied().or_else(|| {
        run.formed_groups
            .iter()
            .find(|f| f.id == group)
            .and_then(|f| Side::parse(&f.side))
    })
}

fn hits_on(run: &PerRun, ships: &BTreeSet<u32>, weapons: &[Weapon]) -> u64 {
    run.hits_on
        .iter()
        .filter(|h| ships.contains(&h.ship))
        .map(|h| {
            weapons
                .iter()
                .map(|w| match w {
                    Weapon::Gun => h.gun,
                    Weapon::Torpedo => h.torpedo,
                    Weapon::Bomb => h.bomb,
                })
                .sum::<u64>()
        })
        .sum()
}

fn air_losses(run: &PerRun, groups: &BTreeSet<u32>, causes: &[Cause]) -> u64 {
    run.air_losses
        .iter()
        .filter(|l| in_set(run, groups, l.air_group))
        .map(|l| causes.iter().map(|c| c.of(l)).sum::<u64>())
        .sum()
}

fn ships_sunk(run: &PerRun, ships: &BTreeSet<u32>) -> u64 {
    run.sunk.iter().filter(|s| ships.contains(s)).count() as u64
}

fn system_out(run: &PerRun, ships: &BTreeSet<u32>, system: &str) -> BTreeSet<u32> {
    run.systems_out
        .iter()
        .filter(|s| s.system == system && ships.contains(&s.ship))
        .map(|s| s.ship)
        .collect()
}

fn side_claims(run: &PerRun, side: Side, claim: Claim) -> u64 {
    let c = match side {
        Side::Japanese => &run.claims.japanese,
        Side::Allied => &run.claims.allied,
    };
    match claim {
        Claim::Sunk => c.sunk,
        Claim::Hits => c.hits,
        Claim::ShotDown => c.shot_down,
        Claim::OnGround => c.on_ground,
    }
}

/// The run's attacks that a measure counts.
fn attack_set<'r>(
    ctx: &Context,
    run: &'r PerRun,
    who: &Who,
    target: &Target,
) -> Vec<&'r super::input::Attack> {
    run.attacks
        .iter()
        .filter(|a| {
            let by = match who {
                Who::Side(side) => side_of_group(ctx, run, a.air_group) == Some(*side),
                Who::Groups(set) => in_set(run, set, a.air_group),
            };
            let on = match target {
                Target::TaskForces(forces) => {
                    a.target_kind == "ship"
                        && ctx
                            .force_of
                            .get(&a.target)
                            .copied()
                            .flatten()
                            .is_some_and(|f| forces.contains(&f))
                }
                Target::Base(base) => a.target_kind == "base" && a.target == *base,
            };
            by && on
        })
        .collect()
}

/// The side's fire with a weapon (gun or torpedo) in a run.
fn fired(run: &PerRun, side: Side, weapon: Weapon) -> &super::input::WeaponFire {
    let fire = match side {
        Side::Japanese => &run.fire.japanese,
        Side::Allied => &run.fire.allied,
    };
    match weapon {
        Weapon::Torpedo => &fire.torpedo,
        _ => &fire.gun,
    }
}

/// `floor(hits x 1000 / shots)`, shots a `Fixed` decimal string; nothing
/// fired is `None`.
fn per_thousand(shots: &str, hits: u64) -> Result<Option<i64>, String> {
    let shots: Fixed = shots
        .parse()
        .map_err(|_| format!("the shots \"{shots}\" are not a decimal number"))?;
    let raw = i128::from(shots.raw());
    if raw <= 0 {
        return Ok(None);
    }
    let value = i128::from(hits) * 1000 * i128::from(Fixed::SCALE) / raw;
    Ok(Some(
        i64::try_from(value).map_err(|_| "a ratio is too large".to_string())?,
    ))
}

/// `floor(shots / 100)`, shots a `Fixed` decimal string. A run that fires
/// nothing is 0, not left out.
fn in_hundreds(shots: &str) -> Result<i64, String> {
    hundreds_of(&[shots])
}

/// `floor(sum of shots / 100)`: the strings are `Fixed` decimals, added
/// exactly and floored once.
fn hundreds_of(shots: &[&str]) -> Result<i64, String> {
    floored_sum(shots, 100)
}

/// `floor(sum of shots / divisor)`, added exactly and floored once.
fn floored_sum(shots: &[&str], divisor: i128) -> Result<i64, String> {
    let mut raw = 0i128;
    for text in shots {
        let shots: Fixed = text
            .parse()
            .map_err(|_| format!("the shots \"{text}\" are not a decimal number"))?;
        raw += i128::from(shots.raw());
    }
    i64::try_from(raw.max(0) / (divisor * i128::from(Fixed::SCALE)))
        .map_err(|_| "a count is too large".to_string())
}

/// The shots with a weapon of the named ships in a run, in hundreds.
fn ships_in_hundreds(run: &PerRun, ships: &BTreeSet<u32>, weapon: Weapon) -> Result<i64, String> {
    floored_sum(&ships_shots(run, ships, weapon), 100)
}

/// The shots of the named ships' with a weapon, as the batch gives them.
fn ships_shots<'a>(run: &'a PerRun, ships: &BTreeSet<u32>, weapon: Weapon) -> Vec<&'a str> {
    run.shots_by
        .iter()
        .filter(|s| ships.contains(&s.ship))
        .map(|s| match weapon {
            Weapon::Torpedo => s.torpedo.as_str(),
            _ => s.gun.as_str(),
        })
        .collect()
}

/// `floor(supply / (men x per_man))`, in `Fixed` raw units, which cancel. No
/// men, or no ration, leaves the run out.
fn days_of_food(supply: Fixed, men: u64, per_man: Fixed) -> Result<i64, String> {
    let need = i128::from(men) * i128::from(per_man.raw());
    if need <= 0 {
        return Err("days_of_food: the men ashore need no food".to_string());
    }
    i64::try_from(i128::from(supply.raw()).max(0) / need)
        .map_err(|_| "a count is too large".to_string())
}

/// The value of a measure in one run: `None` when the run is left out.
pub fn value(take: &Take, ctx: &Context, run: &PerRun) -> Result<Option<i64>, String> {
    let n = |x: u64| i64::try_from(x).map_err(|_| "a count is too large".to_string());
    Ok(Some(match take {
        Take::ShipsSunk(ships) => n(ships_sunk(run, ships))?,
        Take::SystemOut(ships, system) => system_out(run, ships, system).len() as i64,
        Take::SunkOrSystemOut(ships, system) => {
            // A ship that is both is counted once: only afloat ships count
            // as stopped.
            let stopped = system_out(run, ships, system)
                .iter()
                .filter(|ship| !run.sunk.contains(ship))
                .count() as u64;
            n(ships_sunk(run, ships) + stopped)?
        }
        Take::HitsOn(ships, weapons) => n(hits_on(run, ships, weapons))?,
        Take::HitsPerThousand(side, weapon) => {
            let f = fired(run, *side, *weapon);
            return per_thousand(&f.shots, f.hits);
        }
        Take::ShotsInHundreds(Fired::Side(side), weapon) => {
            in_hundreds(&fired(run, *side, *weapon).shots)?
        }
        Take::ShotsInHundreds(Fired::Ships(ships), weapon) => {
            ships_in_hundreds(run, ships, *weapon)?
        }
        Take::Shots(Fired::Side(side), weapon, unit) => {
            floored_sum(&[fired(run, *side, *weapon).shots.as_str()], unit.divisor())?
        }
        Take::Shots(Fired::Ships(ships), weapon, unit) => {
            floored_sum(&ships_shots(run, ships, *weapon), unit.divisor())?
        }
        Take::AirLosses(groups, causes) => n(air_losses(run, groups, causes))?,
        Take::PilotsLost(groups) => n(run
            .air_losses
            .iter()
            .filter(|l| in_set(run, groups, l.air_group))
            .map(|l| l.pilots_lost)
            .sum())?,
        Take::Ready(groups, index) => {
            let mut sum = 0u64;
            for row in run
                .ready
                .iter()
                .filter(|r| in_set(run, groups, r.air_group))
            {
                sum += *row.by_day.get(*index).ok_or_else(|| {
                    format!(
                        "air group {} has no ready count for day {}",
                        row.air_group,
                        index + 1
                    )
                })?;
            }
            n(sum)?
        }
        Take::Attacking(who, target, escorts) => {
            let attacks = attack_set(ctx, run, who, target);
            let mut sum: u64 = attacks.iter().map(|a| a.aircraft).sum();
            if *escorts {
                let keys: BTreeSet<(u32, &str, u32)> = attacks
                    .iter()
                    .map(|a| (a.day, a.pulse.as_str(), a.air_group))
                    .collect();
                sum += run
                    .escorts
                    .iter()
                    .filter(|e| keys.contains(&(e.day, e.pulse.as_str(), e.with_air_group)))
                    .map(|e| e.aircraft)
                    .sum::<u64>();
            }
            n(sum)?
        }
        Take::TakingPart(who, forces, escorts) => {
            let counted: Vec<&super::input::Launch> = run
                .launches
                .iter()
                .filter(|l| {
                    l.target_kind == "ship"
                        && !l.found_nothing
                        && l.task_force.is_some_and(|f| forces.contains(&f))
                        && match who {
                            Who::Side(side) => side_of_group(ctx, run, l.air_group) == Some(*side),
                            Who::Groups(set) => in_set(run, set, l.air_group),
                        }
                })
                .collect();
            let mut sum: u64 = counted.iter().map(|l| l.aircraft).sum();
            if *escorts {
                // An escort entry names the raid's first strike group; each
                // entry is counted once, whatever launch it matches.
                let keys: BTreeSet<(u32, &str, u32)> = counted
                    .iter()
                    .map(|l| (l.day, l.pulse.as_str(), l.air_group))
                    .collect();
                sum += run
                    .escorts
                    .iter()
                    .filter(|e| keys.contains(&(e.day, e.pulse.as_str(), e.with_air_group)))
                    .map(|e| e.aircraft)
                    .sum::<u64>();
            }
            n(sum)?
        }
        Take::AttackDays(who, target) => {
            let days: BTreeSet<u32> = attack_set(ctx, run, who, target)
                .iter()
                .map(|a| a.day)
                .collect();
            days.len() as i64
        }
        Take::AttackAtLeast(side, forces, floor) => {
            let attacks = attack_set(
                ctx,
                run,
                &Who::Side(*side),
                &Target::TaskForces(forces.clone()),
            );
            let mut by_pulse: BTreeMap<(u32, &str), u64> = BTreeMap::new();
            for a in attacks {
                *by_pulse.entry((a.day, a.pulse.as_str())).or_default() += a.aircraft;
            }
            i64::from(by_pulse.values().any(|total| total >= floor))
        }
        Take::Claims(side, kind) => n(side_claims(run, *side, *kind))?,
        Take::ClaimsPerHundred(side, kind, real) => {
            let done = match real {
                Real::Ships(ships) => ships_sunk(run, ships),
                Real::Hits(ships, weapons) => hits_on(run, ships, weapons),
                Real::Aircraft(groups, causes) => air_losses(run, groups, causes),
            };
            if done == 0 {
                return Ok(None);
            }
            let claimed = i128::from(side_claims(run, *side, *kind));
            i64::try_from(claimed * 100 / i128::from(done))
                .map_err(|_| "a ratio is too large".to_string())?
        }
        Take::Minutes => {
            let minutes = ctx.round_minutes.ok_or_else(|| {
                "minutes_of_action needs the surface combat figures, which the scenario does not have"
                    .to_string()
            })?;
            let rounds: u64 = run.fights.iter().map(|f| f.rounds).sum();
            n(rounds * minutes)?
        }
        Take::FightsByDay => n(run.fights.iter().filter(|f| f.pulse != "night").count() as u64)?,
        Take::FightsSightedFirst(by) => n(run
            .fights
            .iter()
            .filter(|f| f.pulse == "night" && by.contains(&f.sighted_first))
            .count() as u64)?,
        Take::RoundsSurprised(forces) => n(run
            .fights
            .iter()
            .filter(|f| f.pulse == "night")
            .flat_map(|f| &f.forces)
            .filter(|force| forces.contains(&force.task_force))
            .map(|force| force.rounds_surprised)
            .sum::<u64>())?,
        Take::BaseHolder { base, day, side } => {
            let mut holder = ctx
                .base_side
                .get(base)
                .copied()
                .ok_or_else(|| format!("base {base} is not in the batch's header"))?;
            for change in run
                .base_changes
                .iter()
                .filter(|c| c.base == *base && c.day <= *day)
            {
                holder = Side::parse(&change.to)
                    .ok_or_else(|| format!("\"{}\" is not a side", change.to))?;
            }
            i64::from(holder == *side)
        }
        Take::MenLost {
            side,
            from,
            to,
            hex: None,
        } => n(run
            .men_lost_by_day
            .iter()
            .filter(|d| d.day >= *from && d.day <= *to)
            .map(|d| match side {
                Side::Japanese => d.japanese,
                Side::Allied => d.allied,
            })
            .sum())?,
        Take::MenLost {
            side,
            from,
            to,
            hex: Some((q, r)),
        } => n(run
            .men_lost_ashore_by_day
            .iter()
            .filter(|d| d.day >= *from && d.day <= *to && d.q == *q && d.r == *r)
            .map(|d| match side {
                Side::Japanese => d.japanese,
                Side::Allied => d.allied,
            })
            .sum())?,
        Take::AirfieldBuiltDay(base) => match run
            .airfields_built
            .iter()
            .filter(|a| a.base == *base)
            .map(|a| a.day)
            .min()
        {
            Some(day) => i64::from(day),
            None => return Ok(None),
        },
        Take::ArrivalDay { air_group, due } => match run
            .arrivals_delivered
            .iter()
            .filter(|a| a.air_group == *air_group && a.day >= *due)
            .map(|a| a.day)
            .min()
        {
            Some(day) => i64::from(day),
            None => return Ok(None),
        },
        Take::DaysOfFood { side, q, r, day } => {
            let per_man = ctx.per_man.ok_or_else(|| {
                "days_of_food needs the ground figures, which the batch does not have".to_string()
            })?;
            let Some(row) = run.ashore_by_day.iter().find(|a| {
                a.day == *day && a.q == *q && a.r == *r && Side::parse(&a.side) == Some(*side)
            }) else {
                return Ok(None);
            };
            let supply: Fixed = row
                .supply_t
                .parse()
                .map_err(|_| format!("the supply \"{}\" is not a decimal number", row.supply_t))?;
            days_of_food(supply, row.men, per_man)?
        }
    }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::{json, Value};

    #[test]
    fn shots_in_tens_are_added_exactly_and_floored_once() {
        let tens = ShotUnit::Tens.divisor();
        assert_eq!(floored_sum(&["33"], tens), Ok(3));
        assert_eq!(floored_sum(&["50", "1"], tens), Ok(5));
        assert_eq!(floored_sum(&["29.9", "3.2"], tens), Ok(3));
        assert_eq!(floored_sum(&["9.999999"], tens), Ok(0));
        assert_eq!(floored_sum(&["5", "5"], tens), Ok(1));
        assert_eq!(floored_sum(&["33.5"], ShotUnit::Shells.divisor()), Ok(33));
    }

    #[test]
    fn shots_in_hundreds_rounds_down() {
        for (shots, want) in [
            ("0", 0),
            ("99.999999", 0),
            ("100", 1),
            ("199.5", 1),
            ("2791", 27),
            ("1867", 18),
            ("471", 4),
        ] {
            assert_eq!(in_hundreds(shots), Ok(want), "{shots}");
        }
        assert!(in_hundreds("many").is_err());
    }

    #[test]
    fn shots_of_several_ships_are_added_exactly_and_floored_once() {
        // 99.999999 + 0.000001 is 100: floored once, 1, where flooring each
        // would give 0.
        assert_eq!(hundreds_of(&["99.999999", "0.000001"]), Ok(1));
        assert_eq!(hundreds_of(&["150", "60.5"]), Ok(2));
        assert_eq!(hundreds_of(&["99", "0.999"]), Ok(0));
        assert_eq!(hundreds_of(&[]), Ok(0));
        assert!(hundreds_of(&["1", "many"]).is_err());
    }

    // ------------------------------------------------- the land-fight takes

    /// A batch of a 6-day scenario with bases 1 (Japanese) and 2 (Allied),
    /// air group 7, and ground figures of 0.005 t a man a day.
    fn batch_json(ground: bool) -> Batch {
        let mut batch = json!({
            "scenario": {"file": "x.toml", "days": 6},
            "inputs": {"state_version": "v", "scenario_hash": "h", "data": []},
            "runs": 1, "first_seed": 1,
            "ships": [], "per_run": [],
            "task_forces": [],
            "air_groups": [{"id": 7, "side": "allied"}],
            "bases": [{"id": 1, "side": "japanese"}, {"id": 2, "side": "allied"}],
        });
        if ground {
            batch["daily_supply_t_per_man"] = json!("0.005");
        }
        serde_json::from_value(batch).expect("a batch")
    }

    fn run_json(extra: Value) -> PerRun {
        let mut run = json!({
            "seed": 1, "sunk": [], "hits_on": [],
            "fire": {
                "japanese": {"gun": {"shots": "0", "hits": 0}, "torpedo": {"shots": "0", "hits": 0}},
                "allied": {"gun": {"shots": "0", "hits": 0}, "torpedo": {"shots": "0", "hits": 0}}
            },
            "shots_by": [], "systems_out": [], "air_losses": [], "ready": [],
            "claims": {
                "japanese": {"sunk": 0, "hits": 0, "shot_down": 0, "on_ground": 0},
                "allied": {"sunk": 0, "hits": 0, "shot_down": 0, "on_ground": 0}
            },
            "attacks": [], "escorts": [], "launches": [], "fights": [], "formed_groups": [],
        });
        for (key, value) in extra.as_object().expect("an object") {
            run[key] = value.clone();
        }
        serde_json::from_value(run).expect("a run")
    }

    fn measure_json(fields: Value) -> Measure {
        let mut measure = json!({
            "id": "m", "counts": "c", "record_low": 0, "record_high": 0,
            "judged": true, "rows": [1], "reason": "r",
        });
        for (key, value) in fields.as_object().expect("an object") {
            measure[key] = value.clone();
        }
        serde_json::from_value(measure).expect("a measure")
    }

    fn taken(batch: &Batch, fields: Value, run: &PerRun) -> Result<Option<i64>, String> {
        let measure = measure_json(fields);
        let take = parse(&measure, batch)?;
        value(&take, &Context::new(batch, None), run)
    }

    #[test]
    fn base_holder_is_the_header_side_changed_by_the_changes_up_to_the_day() {
        let batch = batch_json(true);
        let run = run_json(json!({"base_changes": [
            {"day": 3, "pulse": "night", "base": 1, "from": "japanese", "to": "allied"},
            {"day": 5, "pulse": "night", "base": 1, "from": "allied", "to": "japanese"},
            {"day": 4, "pulse": "night", "base": 2, "from": "allied", "to": "japanese"},
        ]}));
        let held = |base: u32, day: u32, side: &str| {
            taken(
                &batch,
                json!({"take": "base_holder", "base": base, "day": day, "side": side}),
                &run,
            )
        };
        assert_eq!(held(1, 2, "japanese"), Ok(Some(1)));
        assert_eq!(held(1, 2, "allied"), Ok(Some(0)));
        // The change of day 3 counts at the end of day 3.
        assert_eq!(held(1, 3, "allied"), Ok(Some(1)));
        assert_eq!(held(1, 4, "allied"), Ok(Some(1)));
        assert_eq!(held(1, 5, "japanese"), Ok(Some(1)));
        assert_eq!(held(2, 3, "allied"), Ok(Some(1)));
        assert_eq!(held(2, 6, "japanese"), Ok(Some(1)));
        // A run with no change keeps the header side.
        let quiet = run_json(json!({}));
        let take = parse(
            &measure_json(json!({"take": "base_holder", "base": 2, "day": 6, "side": "allied"})),
            &batch,
        )
        .expect("parses");
        assert_eq!(
            value(&take, &Context::new(&batch, None), &quiet),
            Ok(Some(1))
        );
    }

    #[test]
    fn men_lost_sums_the_days_asked_for_both_ends_included() {
        let batch = batch_json(true);
        let run = run_json(json!({"men_lost_by_day": [
            {"day": 2, "japanese": 10, "allied": 1},
            {"day": 3, "japanese": 20, "allied": 2},
            {"day": 5, "japanese": 40, "allied": 4},
        ]}));
        let lost = |side: &str, from: u32, to: u32| {
            taken(
                &batch,
                json!({"take": "men_lost", "side": side, "from_day": from, "to_day": to}),
                &run,
            )
        };
        assert_eq!(lost("japanese", 2, 3), Ok(Some(30)));
        assert_eq!(lost("japanese", 3, 5), Ok(Some(60)));
        assert_eq!(lost("allied", 1, 6), Ok(Some(7)));
        assert_eq!(lost("allied", 4, 4), Ok(Some(0)));
        // A run with no man lost is 0, not left out.
        let quiet = run_json(json!({}));
        let take = parse(
            &measure_json(
                json!({"take": "men_lost", "side": "allied", "from_day": 1, "to_day": 6}),
            ),
            &batch,
        )
        .expect("parses");
        assert_eq!(
            value(&take, &Context::new(&batch, None), &quiet),
            Ok(Some(0))
        );
    }

    #[test]
    fn men_lost_in_a_hex_counts_only_the_men_lost_ashore_there() {
        let batch = batch_json(true);
        let run = run_json(json!({
            "men_lost_by_day": [{"day": 2, "japanese": 100, "allied": 5}],
            "men_lost_ashore_by_day": [
                {"day": 2, "q": 1, "r": 0, "japanese": 60, "allied": 5},
                {"day": 2, "q": 2, "r": 0, "japanese": 30, "allied": 0},
                {"day": 3, "q": 1, "r": 0, "japanese": 7, "allied": 0},
            ]
        }));
        let lost = |side: &str, q: i32, from: u32, to: u32| {
            taken(
                &batch,
                json!({"take": "men_lost", "side": side, "from_day": from, "to_day": to,
                       "q": q, "r": 0}),
                &run,
            )
        };
        assert_eq!(lost("japanese", 1, 1, 6), Ok(Some(67)));
        assert_eq!(lost("japanese", 1, 2, 2), Ok(Some(60)));
        assert_eq!(lost("japanese", 2, 1, 6), Ok(Some(30)));
        assert_eq!(lost("allied", 1, 1, 6), Ok(Some(5)));
        assert_eq!(lost("allied", 9, 1, 6), Ok(Some(0)));
        // Without a hex the by-day figure, men lost with a ship included.
        assert_eq!(
            taken(
                &batch,
                json!({"take": "men_lost", "side": "japanese", "from_day": 1, "to_day": 6}),
                &run
            ),
            Ok(Some(100))
        );
        // Both keys or neither.
        let err = taken(
            &batch,
            json!({"take": "men_lost", "side": "allied", "from_day": 1, "to_day": 2, "q": 1}),
            &run,
        )
        .expect_err("refused");
        assert_eq!(err, "measure m: take men_lost needs the key r");
    }

    #[test]
    fn airfield_built_day_is_the_day_of_the_build_and_a_run_without_one_is_left_out() {
        let batch = batch_json(true);
        let built = run_json(json!({"airfields_built": [{"day": 4, "base": 1}]}));
        let measure = json!({"take": "airfield_built_day", "base": 1});
        assert_eq!(taken(&batch, measure.clone(), &built), Ok(Some(4)));
        assert_eq!(taken(&batch, measure, &run_json(json!({}))), Ok(None));
        // Another base's airfield is not this one's.
        assert_eq!(
            taken(
                &batch,
                json!({"take": "airfield_built_day", "base": 2}),
                &built
            ),
            Ok(None)
        );
    }

    #[test]
    fn arrival_day_is_the_first_delivery_on_or_after_the_due_day() {
        let batch = batch_json(true);
        let run = run_json(json!({"arrivals_delivered": [
            {"day": 2, "air_group": 7, "aircraft": 9},
            {"day": 5, "air_group": 7, "aircraft": 9},
            {"day": 3, "air_group": 8, "aircraft": 9},
        ]}));
        let arrival = |due: u32| {
            taken(
                &batch,
                json!({"take": "arrival_day", "air_group": 7, "due_day": due}),
                &run,
            )
        };
        assert_eq!(arrival(1), Ok(Some(2)));
        assert_eq!(arrival(2), Ok(Some(2)));
        assert_eq!(arrival(3), Ok(Some(5)));
        assert_eq!(arrival(6), Ok(None));
    }

    #[test]
    fn days_of_food_is_the_supply_over_a_days_ration_rounded_down() {
        let batch = batch_json(true);
        // 1000 men eat 5 t a day at 0.005 t each: 12.5 t is 2 days and a half.
        let run = run_json(json!({"ashore_by_day": [
            {"day": 2, "side": "allied", "q": 3, "r": 4, "men": 1000, "supply_t": "12.5"},
            {"day": 3, "side": "allied", "q": 3, "r": 4, "men": 1000, "supply_t": "4.999999"},
            {"day": 3, "side": "japanese", "q": 3, "r": 4, "men": 200, "supply_t": "10"},
        ]}));
        let food = |side: &str, day: u32| {
            taken(
                &batch,
                json!({"take": "days_of_food", "side": side, "q": 3, "r": 4, "day": day}),
                &run,
            )
        };
        assert_eq!(food("allied", 2), Ok(Some(2)));
        assert_eq!(food("allied", 3), Ok(Some(0)));
        // 200 men eat 1 t a day: 10 t is exactly 10 days.
        assert_eq!(food("japanese", 3), Ok(Some(10)));
        // No men there that day: left out.
        assert_eq!(food("japanese", 2), Ok(None));
        assert_eq!(food("allied", 4), Ok(None));
    }

    #[test]
    fn the_new_takes_refuse_wrong_or_missing_keys_as_the_others_do() {
        let batch = batch_json(true);
        let err = |fields: Value| taken(&batch, fields, &run_json(json!({}))).expect_err("refused");
        assert_eq!(
            err(json!({"take": "base_holder", "day": 1, "side": "allied"})),
            "measure m: take base_holder needs the key base"
        );
        assert_eq!(
            err(json!({"take": "base_holder", "base": 9, "day": 1, "side": "allied"})),
            "measure m: base 9 is not a base of the batch"
        );
        assert_eq!(
            err(json!({"take": "base_holder", "base": 1, "day": 7, "side": "allied"})),
            "measure m: day 7 is outside the scenario's 6 days"
        );
        assert_eq!(
            err(json!({"take": "base_holder", "base": 1, "day": 1, "side": "navy"})),
            "measure m: side \"navy\" is not japanese or allied"
        );
        assert_eq!(
            err(json!({"take": "base_holder", "base": 1, "day": 1, "side": "allied", "q": 1})),
            "measure m: take base_holder does not take the key q"
        );
        assert_eq!(
            err(json!({"take": "men_lost", "side": "allied", "from_day": 1})),
            "measure m: take men_lost needs the key to_day"
        );
        assert_eq!(
            err(json!({"take": "men_lost", "side": "allied", "from_day": 4, "to_day": 2})),
            "measure m: from_day 4 is after to_day 2"
        );
        assert_eq!(
            err(json!({"take": "airfield_built_day"})),
            "measure m: take airfield_built_day needs the key base"
        );
        assert_eq!(
            err(json!({"take": "arrival_day", "air_group": 99, "due_day": 1})),
            "measure m: air_group names air group 99, which the batch does not have"
        );
        assert_eq!(
            err(json!({"take": "arrival_day", "air_group": 7})),
            "measure m: take arrival_day needs the key due_day"
        );
        assert_eq!(
            err(json!({"take": "days_of_food", "side": "allied", "q": 1, "day": 1})),
            "measure m: take days_of_food needs the key r"
        );
        // A batch with no ground figures cannot be asked for days of food.
        let bare = batch_json(false);
        let measure = measure_json(
            json!({"take": "days_of_food", "side": "allied", "q": 1, "r": 1, "day": 1}),
        );
        assert!(parse(&measure, &bare)
            .err()
            .is_some_and(|e| e.contains("needs the ground figures")));
    }

    #[test]
    fn days_of_food_works_in_fixed_point() {
        // 7 t over 3 men at 0.3 t a day each is 0.9 t a day: 7 days, and the
        // 0.1 t left over does not make an eighth.
        let per_man = Fixed::from_raw(300_000);
        assert_eq!(days_of_food("7".parse().unwrap(), 3, per_man), Ok(7));
        assert_eq!(days_of_food("6.999999".parse().unwrap(), 3, per_man), Ok(7));
        assert_eq!(days_of_food("6.299999".parse().unwrap(), 3, per_man), Ok(6));
        assert!(days_of_food("1".parse().unwrap(), 0, per_man).is_err());
    }
}
