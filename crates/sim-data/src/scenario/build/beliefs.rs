use crate::error::DataError;
use crate::reference::parse_kind;
use sim_core::hex::Hex;
use sim_core::pictures::CommandRef;
use sim_core::state::{Side, WatersWaiting};
use sim_core::{StartingContact, StartingEstimate, World};
use std::path::Path;

use super::super::beliefs::{believed_force, believed_intent, believed_men};
use super::super::file::ScenarioFile;

/// What each side believed of the enemy at the start.
pub(super) struct Believed {
    pub(super) forces: Vec<sim_core::believed::BelievedForce>,
    pub(super) intents: Vec<sim_core::believed::BelievedIntent>,
    pub(super) men: Vec<sim_core::believed::BelievedMen>,
}

pub(super) fn hold_contacts(
    file: &ScenarioFile,
    path: &Path,
    world: &mut World,
) -> Result<Vec<(StartingContact, Option<CommandRef>)>, DataError> {
    let fail = |message: String| DataError::in_file(path, message);
    let mut contacts = Vec::new();
    for (index, spec) in file.contact.iter().enumerate() {
        let kind = parse_kind(&spec.kind).ok_or_else(|| {
            fail(format!(
                "contact {} has unknown kind \"{}\"",
                index + 1,
                spec.kind
            ))
        })?;
        let seen_by = match &spec.seen_by {
            None => None,
            Some(seen_by) => {
                let command = seen_by.command().map_err(|_| {
                    fail(format!(
                        "contact {}: seen_by names exactly one of task_force or base",
                        index + 1
                    ))
                })?;
                let side: Side = spec.side.into();
                let known = match command {
                    CommandRef::TaskForce(force) => file
                        .task_force
                        .iter()
                        .any(|t| t.id == force.raw() && Side::from(t.side) == side),
                    CommandRef::Base(base) => file
                        .base
                        .iter()
                        .any(|b| b.id == base.raw() && Side::from(b.side) == side),
                    CommandRef::Area => true,
                };
                if !known {
                    return Err(fail(format!(
                        "contact {} is seen by {command}, which is not a command of the {} side",
                        index + 1,
                        side.name()
                    )));
                }
                Some(command)
            }
        };
        contacts.push((
            StartingContact {
                side: spec.side.into(),
                hex: Hex::new(spec.q, spec.r),
                ship_count: spec.ship_count,
                kind,
                age_pulses: spec.age_pulses.unwrap_or(0),
            },
            seen_by,
        ));
    }
    world
        .hold_starting_contacts_seen(contacts.clone())
        .map_err(|e| fail(e.to_string()))?;
    Ok(contacts)
}

pub(super) fn hold_estimates(
    file: &ScenarioFile,
    path: &Path,
    world: &mut World,
) -> Result<Vec<StartingEstimate>, DataError> {
    let fail = |message: String| DataError::in_file(path, message);
    let estimates: Vec<StartingEstimate> = file
        .estimate
        .iter()
        .map(|spec| StartingEstimate {
            side: spec.side.into(),
            hex: Hex::new(spec.q, spec.r),
            men: spec.men,
        })
        .collect();
    world
        .hold_starting_estimates(estimates.clone())
        .map_err(|e| fail(e.to_string()))?;
    Ok(estimates)
}

pub(super) fn hold_suspected_waters(
    file: &ScenarioFile,
    path: &Path,
    world: &mut World,
) -> Result<Vec<WatersWaiting>, DataError> {
    let fail = |message: String| DataError::in_file(path, message);
    let mut waters = Vec::new();
    for (index, spec) in file.suspected_waters.iter().enumerate() {
        let what = format!("suspected waters {}", index + 1);
        let day = spec.day.unwrap_or(1);
        if day == 0 || day > file.days {
            return Err(fail(format!(
                "{what} are told on day {day}, which is not a day of the game (1 to {})",
                file.days
            )));
        }
        for (name, text) in [
            ("words", &spec.words),
            ("rows", &spec.rows),
            ("reading", &spec.reading),
        ] {
            if text.trim().is_empty() {
                return Err(fail(format!("{what} have no {name}")));
            }
        }
        waters.push(WatersWaiting {
            side: spec.side.into(),
            hex: Hex::new(spec.q, spec.r),
            distance_hexes: spec.distance,
            words: spec.words.clone(),
            rows: spec.rows.clone(),
            reading: spec.reading.clone(),
            day,
        });
    }
    world
        .hold_suspected_waters(waters.clone())
        .map_err(|e| fail(e.to_string()))?;
    Ok(waters)
}

pub(super) fn build_believed(
    file: &ScenarioFile,
    path: &Path,
    world: &World,
) -> Result<Believed, DataError> {
    let fail = |message: String| DataError::in_file(path, message);
    let mut believed_forces = Vec::new();
    for (index, spec) in file.believed_force.iter().enumerate() {
        believed_forces.push(believed_force(spec, index, file.days, world).map_err(&fail)?);
    }
    let mut believed_intents = Vec::new();
    for (index, spec) in file.believed_intent.iter().enumerate() {
        believed_intents.push(believed_intent(spec, index, world).map_err(&fail)?);
    }
    let mut believed_men_list = Vec::new();
    for (index, spec) in file.believed_men.iter().enumerate() {
        believed_men_list.push(believed_men(spec, index, world).map_err(&fail)?);
    }
    Ok(Believed {
        forces: believed_forces,
        intents: believed_intents,
        men: believed_men_list,
    })
}
