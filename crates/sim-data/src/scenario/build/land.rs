use crate::error::DataError;
use sim_core::fixed::Fixed;
use sim_core::hex::Hex;
use sim_core::ids::{FormationId, LandUnitId, TaskForceId};
use sim_core::map::HexKind;
use sim_core::orders::{Activity, Posture};
use sim_core::refdata::RefData;
use sim_core::state::{LandUnit, Side, Skills, TruthState};
use sim_core::world::MADE_IDS_FLOOR;
use std::collections::BTreeMap;
use std::path::Path;

use super::super::available_day;
use super::super::file::ScenarioFile;
use super::super::land::{formations, LandUnitSpec};

pub(super) fn build_land_units(
    file: &ScenarioFile,
    path: &Path,
    truth: &mut TruthState,
    ref_data: &RefData,
    map: &sim_core::map::Map,
) -> Result<(BTreeMap<LandUnitId, u32>, bool), DataError> {
    let fail = |message: String| DataError::in_file(path, message);
    // Formations come before land units, which name them (decision
    // 0140, 7).
    truth.formations = formations(&file.formation).map_err(&fail)?;

    // Land units come after ships, since a unit may be aboard some
    // (decision 0066, 3).
    if !file.land_unit.is_empty() && file.ground.is_none() {
        return Err(fail(
            "the scenario has a land unit but names no ground file (decision 0066, 3)".to_string(),
        ));
    }
    let mut unit_specs: Vec<&LandUnitSpec> = file.land_unit.iter().collect();
    unit_specs.sort_by_key(|spec| spec.id);
    // Units are put aboard in unit order, each over the force's ships in
    // ship order.
    let mut to_put_aboard: Vec<(LandUnitId, TaskForceId, u32)> = Vec::new();
    // Each unit's establishment, kept below only where the scenario has
    // a stream of men or names one (decision 0197, 1).
    let mut establishments: BTreeMap<LandUnitId, u32> = BTreeMap::new();
    let mut named_establishment = false;
    for spec in unit_specs {
        let id = LandUnitId::new(spec.id);
        if spec.id == 0 || spec.id >= MADE_IDS_FLOOR {
            return Err(fail(format!(
                    "{id} is numbered 0 or {MADE_IDS_FLOOR} or above: 0 is no unit and those numbers are kept for units made in play (decision 0066)"
                )));
        }
        if truth.land_units.contains_key(&id) {
            return Err(fail(format!("{id} appears twice")));
        }
        if spec.men == 0 {
            return Err(fail(format!("{id} has no men")));
        }
        let establishment = spec.establishment.unwrap_or(spec.men);
        if establishment < spec.men {
            return Err(fail(format!(
                "{id} has an establishment of {establishment}, below its {} men",
                spec.men
            )));
        }
        establishments.insert(id, establishment);
        named_establishment |= spec.establishment.is_some();
        let side: Side = spec.side.into();
        let mut unit = LandUnit::new(id, spec.name.clone(), side, 0, None);
        unit.available_from = available_day(spec.available_from, id).map_err(&fail)?;
        if let Some(raw) = spec.formation {
            let formation = FormationId::new(raw);
            match truth.formations.get(&formation) {
                None => {
                    return Err(fail(format!(
                        "{id} belongs to {formation}, which is not there"
                    )))
                }
                Some(found) if found.side != side => {
                    return Err(fail(format!(
                        "{id} is {} but its formation {formation} is {}",
                        side.name(),
                        found.side.name()
                    )))
                }
                Some(_) => unit.formation = Some(formation),
            }
        }
        if let Some(posture) = spec.posture {
            unit.posture = posture.into();
        }
        if spec.working.unwrap_or(false) {
            if unit.posture == Posture::Attack {
                return Err(fail(format!(
                        "{id} is a working unit and has posture attack: a working unit does not attack, it is always on defend"
                    )));
            }
            unit.working = true;
            unit.activity = Activity::Work(None);
        }
        if let Some(fatigue) = &spec.fatigue {
            let fatigue = fatigue
                .to_fixed()
                .map_err(|e| fail(format!("{id} fatigue {e}")))?;
            if fatigue.is_negative() || fatigue > Fixed::from_int(100) {
                return Err(fail(format!(
                    "{id} has fatigue {fatigue}, which is not between 0 and 100"
                )));
            }
            unit.fatigue = fatigue;
        }
        if let Some(fieldworks) = &spec.fieldworks {
            let fieldworks = fieldworks
                .to_fixed()
                .map_err(|e| fail(format!("{id} fieldworks {e}")))?;
            let limit = ref_data
                .ground
                .as_ref()
                .map_or(Fixed::ZERO, |g| g.fieldworks_limit);
            if fieldworks.is_negative() || fieldworks > limit {
                return Err(fail(format!(
                        "{id} has fieldworks {fieldworks}, which is not between 0 and the fieldworks_limit {limit}"
                    )));
            }
            if fieldworks > Fixed::ZERO
                && (unit.working || unit.posture == Posture::Attack || spec.task_force.is_some())
            {
                return Err(fail(format!(
                        "{id} has fieldworks but is a working unit, on attack or has no men ashore: only a fighting unit on defend with men ashore has fieldworks"
                    )));
            }
            unit.fieldworks = fieldworks;
        }
        if let Some(experience) = spec.experience {
            if experience > u32::from(LandUnit::MAX_EXPERIENCE) {
                return Err(fail(format!(
                    "{id} has experience {experience}, which is not a whole number from 0 to {}",
                    LandUnit::MAX_EXPERIENCE
                )));
            }
            unit.experience = experience as u8;
        }
        if let Some(days) = spec.training_days {
            let per_level = ref_data
                .ground
                .as_ref()
                .map_or(u32::MAX, |g| g.training_days_per_level);
            if days >= per_level {
                return Err(fail(format!(
                        "{id} has {days} days of training counted, which is not below the {per_level} days a step takes"
                    )));
            }
            unit.training_days = days;
        }
        if let Some(days) = spec.fight_days {
            let per_level = ref_data
                .ground
                .as_ref()
                .map_or(u32::MAX, |g| g.fight_days_per_level);
            if days >= per_level {
                return Err(fail(format!(
                        "{id} has {days} days of fighting counted, which is not below the {per_level} days a step takes"
                    )));
            }
            unit.fight_days = days;
        }
        // The count it starts with (decision 0205, 7).
        unit.hunger_nights = spec.hunger_nights.unwrap_or(0);
        if let Some(skills) = &spec.skills {
            let standard = Skills::STANDARD_ALL;
            let mut set = Skills::STANDARD_ALL;
            for (name, given, slot, default) in [
                (
                    "assault",
                    skills.assault,
                    &mut set.assault,
                    standard.assault,
                ),
                (
                    "holding",
                    skills.holding,
                    &mut set.holding,
                    standard.holding,
                ),
                ("weight", skills.weight, &mut set.weight, standard.weight),
                ("clear", skills.clear, &mut set.clear, standard.clear),
                ("jungle", skills.jungle, &mut set.jungle, standard.jungle),
                (
                    "mountain",
                    skills.mountain,
                    &mut set.mountain,
                    standard.mountain,
                ),
                ("atoll", skills.atoll, &mut set.atoll, standard.atoll),
                ("works", skills.works, &mut set.works, standard.works),
            ] {
                let value = given.unwrap_or(u32::from(default));
                if value > u32::from(Skills::MAX) {
                    return Err(fail(format!(
                            "{id} has a {name} skill of {value}, which is not a whole number from 0 to {}",
                            Skills::MAX
                        )));
                }
                *slot = value as u8;
            }
            unit.skills = set;
        }
        match (spec.q, spec.r, spec.task_force) {
            (Some(q), Some(r), None) => {
                let hex = Hex::new(q, r);
                let data = map
                    .get(hex)
                    .ok_or_else(|| fail(format!("{id} is ashore at {q},{r}, off the map")))?;
                if !matches!(data.kind, HexKind::Coast | HexKind::Land) {
                    return Err(fail(format!(
                        "{id} is ashore at {q},{r}, which is not coast or land"
                    )));
                }
                unit.men_ashore = spec.men;
                unit.hex = Some(hex);
            }
            (None, None, Some(raw)) => {
                let force_id = TaskForceId::new(raw);
                let force = truth
                    .task_forces
                    .get(&force_id)
                    .ok_or_else(|| fail(format!("{id} is aboard unknown task force {raw}")))?;
                if force.side != side {
                    return Err(fail(format!(
                        "{id} is {} but its task force {force_id} is {}",
                        side.name(),
                        force.side.name()
                    )));
                }
                to_put_aboard.push((id, force_id, spec.men));
            }
            _ => {
                return Err(fail(format!(
                    "{id} needs exactly one of a hex (both q and r) and a task_force"
                )))
            }
        }
        truth.land_units.insert(id, unit);
    }
    for (id, force_id, men) in to_put_aboard {
        truth
            .put_aboard_at_start(id, force_id, men, |class| {
                ref_data
                    .ship_classes
                    .get(&class)
                    .map_or(0, |class| class.troop_capacity_men)
            })
            .map_err(&fail)?;
    }
    Ok((establishments, named_establishment))
}
