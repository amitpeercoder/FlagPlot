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

/// Whose shots a `shots_in_hundreds` counts: a side's, from `fire`, or the
/// named ships', from `shots_by`.
pub enum Fired {
    Side(Side),
    Ships(BTreeSet<u32>),
}

pub enum Take {
    ShipsSunk(BTreeSet<u32>),
    SystemOut(BTreeSet<u32>, String),
    HitsOn(BTreeSet<u32>, Vec<Weapon>),
    HitsPerThousand(Side, Weapon),
    /// A side's shots with a weapon, or those of named ships.
    ShotsInHundreds(Fired, Weapon),
    AirLosses(BTreeSet<u32>, Vec<Cause>),
    PilotsLost(BTreeSet<u32>),
    Ready(BTreeSet<u32>, usize),
    Attacking(Who, Target, bool),
    AttackDays(Who, Target),
    AttackAtLeast(Side, BTreeSet<u32>, u64),
    Claims(Side, Claim),
    ClaimsPerHundred(Side, Claim, Real),
    Minutes,
    FightsByDay,
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

/// Reads a measure's `take` and keys, checking each id against the batch.
pub fn parse(measure: &Measure, batch: &Batch) -> Result<Take, String> {
    let known = Known {
        ships: batch.ships.iter().map(|s| s.id).collect(),
        groups: batch.air_groups.iter().map(|g| g.id).collect(),
        forces: batch.task_forces.iter().map(|t| t.id).collect(),
        bases: batch.bases.iter().map(|b| b.id).collect(),
    };
    let take = need(measure, "take", measure.take.as_ref())?;
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
        "ships_with_system_out" => {
            let system = need(measure, "system", measure.system.as_ref())?;
            if !SYSTEMS.contains(&system.as_str()) {
                return Err(format!(
                    "measure {}: system \"{system}\" is not known",
                    measure.id
                ));
            }
            Take::SystemOut(ships(measure)?, system.clone())
        }
        "hits_on" => Take::HitsOn(ships(measure)?, weapons(measure)?),
        "hits_per_thousand_shots" => Take::HitsPerThousand(side(measure)?, fired_weapon(measure)?),
        "shots_in_hundreds" => {
            // `ships` or `side`, never both: with `ships` the named ships'
            // shots are summed, and without it the side's, as before.
            let whose = match (&measure.ships, &measure.side) {
                (Some(_), Some(_)) => {
                    return Err(format!(
                        "measure {}: take shots_in_hundreds takes ships or side, not both",
                        measure.id
                    ))
                }
                (Some(_), None) => Fired::Ships(ships(measure)?),
                (None, _) => Fired::Side(side(measure)?),
            };
            Take::ShotsInHundreds(whose, fired_weapon(measure)?)
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
            let given = measure.ships.is_some()
                || measure.system.is_some()
                || measure.weapons.is_some()
                || measure.weapon.is_some()
                || measure.side.is_some()
                || measure.air_groups.is_some()
                || measure.causes.is_some()
                || measure.day.is_some()
                || measure.escorts.is_some()
                || measure.target_task_forces.is_some()
                || measure.target_base.is_some()
                || measure.aircraft.is_some()
                || measure.claim.is_some();
            if given {
                return Err(format!(
                    "measure {}: take fights_by_day takes no keys",
                    measure.id
                ));
            }
            Take::FightsByDay
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
    let mut raw = 0i128;
    for text in shots {
        let shots: Fixed = text
            .parse()
            .map_err(|_| format!("the shots \"{text}\" are not a decimal number"))?;
        raw += i128::from(shots.raw());
    }
    i64::try_from(raw.max(0) / (100 * i128::from(Fixed::SCALE)))
        .map_err(|_| "a count is too large".to_string())
}

/// The shots with a weapon of the named ships in a run, in hundreds.
fn ships_in_hundreds(run: &PerRun, ships: &BTreeSet<u32>, weapon: Weapon) -> Result<i64, String> {
    let shots: Vec<&str> = run
        .shots_by
        .iter()
        .filter(|s| ships.contains(&s.ship))
        .map(|s| match weapon {
            Weapon::Torpedo => s.torpedo.as_str(),
            _ => s.gun.as_str(),
        })
        .collect();
    hundreds_of(&shots)
}

/// The value of a measure in one run: `None` when the run is left out.
pub fn value(take: &Take, ctx: &Context, run: &PerRun) -> Result<Option<i64>, String> {
    let n = |x: u64| i64::try_from(x).map_err(|_| "a count is too large".to_string());
    Ok(Some(match take {
        Take::ShipsSunk(ships) => n(ships_sunk(run, ships))?,
        Take::SystemOut(ships, system) => {
            let out: BTreeSet<u32> = run
                .systems_out
                .iter()
                .filter(|s| &s.system == system && ships.contains(&s.ship))
                .map(|s| s.ship)
                .collect();
            out.len() as i64
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
    }))
}

#[cfg(test)]
mod tests {
    use super::{hundreds_of, in_hundreds};

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
}
