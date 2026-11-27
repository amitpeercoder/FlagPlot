use crate::error::DataError;
use sim_core::commander::{Commander, Initiative, NamedCommander};
use sim_core::ids::{BaseId, TaskForceId};
use sim_core::pictures::{CommandRef, NamedSuperior};
use sim_core::state::Side;
use sim_core::World;
use std::path::Path;

use super::super::file::ScenarioFile;

pub(super) fn set_doctrines(
    file: &ScenarioFile,
    path: &Path,
    world: &mut World,
) -> Result<(), DataError> {
    let fail = |message: String| DataError::in_file(path, message);
    // Each side's submarine doctrine, as the scenario states it (decision
    // 0223, stage 0).
    if let Some(sides) = &file.side {
        for (side, spec) in [
            (Side::Allied, sides.allied.as_ref()),
            (Side::Japanese, sides.japanese.as_ref()),
        ] {
            if let Some(spec) = spec {
                let doctrine = spec.doctrine(side).map_err(&fail)?;
                world
                    .set_submarine_doctrine(side, doctrine)
                    .map_err(|e| fail(e.to_string()))?;
                if spec.staff_chooses_supply_sources == Some(true) {
                    world
                        .set_staff_chooses_sources(side)
                        .map_err(|e| fail(e.to_string()))?;
                }
            }
        }
    }
    Ok(())
}

pub(super) fn build_command_levels(
    file: &ScenarioFile,
    path: &Path,
    world: &mut World,
) -> Result<Option<Vec<NamedSuperior>>, DataError> {
    let fail = |message: String| DataError::in_file(path, message);
    let command_levels = if file.command_levels == Some(true) {
        let mut named = Vec::new();
        for spec in &file.task_force {
            if let Some(superior) = &spec.superior {
                named.push(NamedSuperior {
                    side: spec.side.into(),
                    command: CommandRef::TaskForce(TaskForceId::new(spec.id)),
                    superior: superior
                        .command()
                        .map_err(|message| fail(format!("task force {}: {message}", spec.id)))?,
                });
            }
        }
        for spec in &file.base {
            if let Some(superior) = &spec.superior {
                named.push(NamedSuperior {
                    side: spec.side.into(),
                    command: CommandRef::Base(BaseId::new(spec.id)),
                    superior: superior
                        .command()
                        .map_err(|message| fail(format!("base {}: {message}", spec.id)))?,
                });
            }
        }
        world
            .turn_command_levels_on(&named)
            .map_err(|e| fail(e.to_string()))?;
        Some(named)
    } else {
        for spec in &file.task_force {
            if spec.superior.is_some() {
                return Err(fail(format!(
                    "task force {} names a superior and command levels are off",
                    spec.id
                )));
            }
        }
        for spec in &file.base {
            if spec.superior.is_some() {
                return Err(fail(format!(
                    "base {} names a superior and command levels are off",
                    spec.id
                )));
            }
        }
        None
    };
    Ok(command_levels)
}

pub(super) fn build_commanders(
    file: &ScenarioFile,
    path: &Path,
    world: &mut World,
) -> Result<Vec<NamedCommander>, DataError> {
    let fail = |message: String| DataError::in_file(path, message);
    let mut commanders = Vec::new();
    for (index, spec) in file.commander.iter().enumerate() {
        let side: Side = spec.side.into();
        let describe = format!("commander {} ({})", index + 1, spec.name);
        let command = match (spec.task_force, spec.base, spec.area) {
            (Some(force), None, None | Some(false)) => {
                if !file
                    .task_force
                    .iter()
                    .any(|t| t.id == force && Side::from(t.side) == side)
                {
                    return Err(fail(format!(
                            "{describe} is named for task force {force}, which is not a task force of the {} side",
                            side.name()
                        )));
                }
                CommandRef::TaskForce(TaskForceId::new(force))
            }
            (None, Some(base), None | Some(false)) => {
                if !file
                    .base
                    .iter()
                    .any(|b| b.id == base && Side::from(b.side) == side)
                {
                    return Err(fail(format!(
                        "{describe} is named for base {base}, which is not a base of the {} side",
                        side.name()
                    )));
                }
                CommandRef::Base(BaseId::new(base))
            }
            (None, None, Some(true)) => CommandRef::Area,
            _ => {
                return Err(fail(format!(
                    "{describe} names exactly one of task_force, base or area = true"
                )))
            }
        };
        let initiative = Initiative::parse(&spec.initiative).ok_or_else(|| {
            fail(format!(
                "{describe} has unknown initiative \"{}\": cautious, steady or bold",
                spec.initiative
            ))
        })?;
        let note = spec.note.clone().unwrap_or_default();
        if note.trim().is_empty() {
            return Err(fail(format!(
                    "{describe}: the rating {} has no note of what it rests on; every rating is a placeholder and its note is required",
                    initiative.name()
                )));
        }
        if commanders
            .iter()
            .any(|c: &NamedCommander| c.side == side && c.command == command)
        {
            return Err(fail(format!(
                "{describe}: {command} has a commander already"
            )));
        }
        commanders.push(NamedCommander {
            side,
            command,
            commander: Commander {
                name: spec.name.clone(),
                initiative,
                note,
            },
        });
    }
    world
        .name_commanders(&commanders)
        .map_err(|e| fail(e.to_string()))?;
    Ok(commanders)
}

pub(super) fn keep_radio_silence(
    file: &ScenarioFile,
    path: &Path,
    world: &mut World,
) -> Result<Vec<TaskForceId>, DataError> {
    let fail = |message: String| DataError::in_file(path, message);
    let mut radio_silent = Vec::new();
    for spec in &file.task_force {
        if spec.radio_silence == Some(true) {
            let force = TaskForceId::new(spec.id);
            world
                .keep_radio_silence_from_the_start(force)
                .map_err(|e| fail(e.to_string()))?;
            radio_silent.push(force);
        }
    }
    Ok(radio_silent)
}
