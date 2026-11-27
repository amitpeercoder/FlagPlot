use crate::error::DataError;
use crate::source::Source;
use sim_core::state::TruthState;
use sim_core::World;
use std::path::{Path, PathBuf};

use super::built_on::{self, Chain};
use super::file::ScenarioFile;
use super::objectives::build_objectives;
use super::{LoadOptions, Scenario};

mod air;
mod bases;
mod beliefs;
mod commanders;
mod enters;
mod figures;
mod islands;
mod land;
mod orders;
mod ships;
mod submarines;
mod supply;

impl Scenario {
    /// As `parse_with`, reading the files the scenario names through `source`.
    ///
    /// Each table is built by a function of its own in `build/`, in the order
    /// the checks have always run, so a file with two faults reports the same
    /// one (decision 0234, step E).
    pub(super) fn parse_via(
        source: &dyn Source,
        text: &str,
        path: &Path,
        options: &LoadOptions,
    ) -> Result<Scenario, DataError> {
        let fail = |message: String| DataError::in_file(path, message);

        // A file that states no `built_on` is read as it always was, so its
        // errors keep their lines; an overlay is laid over the files it is
        // built on as tables and read from the result (decision 0234).
        let mut built_on_files: Vec<PathBuf> = Vec::new();
        let file: ScenarioFile = if built_on::may_state_built_on(text)
            && toml::from_str::<toml::Table>(text).is_ok_and(|t| t.contains_key("built_on"))
        {
            let chain = Chain::read(source, text, path)?;
            built_on_files = chain.paths[1..].to_vec();
            chain
                .laid()?
                .try_into()
                .map_err(|e: toml::de::Error| fail(e.to_string()))?
        } else {
            toml::from_str(text).map_err(|e| fail(e.to_string()))?
        };
        let file = &file;

        let figures::Figures {
            mut map,
            map_frame,
            ref_data,
            data_files,
            estimated_figures,
            trial_figures,
        } = figures::load_figures(source, options, path, file, built_on_files)?;

        let mut truth = TruthState::default();

        bases::build_bases(file, path, &mut truth)?;
        ships::build_task_forces(file, path, &mut truth)?;
        ships::build_ships(file, path, &mut truth, &ref_data)?;
        let (establishments, named_establishment) =
            land::build_land_units(file, path, &mut truth, &ref_data, &map)?;
        let aboard = air::build_air_groups(file, path, &mut truth, &ref_data)?;
        air::build_replacements(file, path, &mut truth, &ref_data)?;
        supply::build_supply_lines(file, path, &mut truth)?;
        supply::build_men_streams(file, path, &mut truth, establishments, named_establishment)?;
        air::build_arrivals(file, path, &mut truth, &ref_data, &aboard)?;

        let orders::OrderScript {
            orders_inside_a_day,
            script,
            script_inside_day,
        } = orders::build_orders(file, path, &truth, &ref_data, &map)?;
        let commands = orders::build_commands(file, path)?;

        // Replacements and arrivals are lists in the state and the hash, so
        // they are kept in one order whatever order the file lists them in
        // (decision 0043, 4c). Orders are put in order where they are taken
        // in, in `sim-core`.
        truth.replacements.sort_by_key(|r| {
            (
                r.side.code(),
                r.base,
                r.aircraft_type,
                r.aircraft_per_day,
                r.pilots_per_day,
            )
        });
        truth
            .arrivals
            .sort_by_key(|a| (a.day, a.air_group, a.aircraft, a.pilots));
        truth
            .supply_lines
            .sort_by_key(sim_core::state::SupplyLine::order_key);
        truth
            .men_replacement
            .streams
            .sort_by_key(sim_core::state::MenStream::order_key);

        islands::build_islands(file, path, &truth, &mut map)?;
        ships::check_class_sides(file, path, &truth, &ref_data)?;
        submarines::build_submarines(file, path, &mut truth, &ref_data)?;
        enters::hold_units_that_enter_later(file, path, &mut truth, &map, &aboard)?;

        let mut world =
            World::new(file.seed, ref_data, map, truth).map_err(|e| fail(e.to_string()))?;
        // The game's days, configuration beside the objectives (decision
        // 0194, 12): set once, here, where the world is made.
        world.set_game_days(file.days);
        commanders::set_doctrines(file, path, &mut world)?;
        // Command levels go on before anything that reaches a belief
        // (decision 0134, 12): the starting contacts and estimates then reach
        // every picture as they reach the side's.
        let command_levels = commanders::build_command_levels(file, path, &mut world)?;
        // The commanders (decision 0137, 1 to 3): checked against the
        // scenario's commands whether command levels are on or not, and held
        // only when they are.
        let commanders = commanders::build_commanders(file, path, &mut world)?;
        let radio_silent = commanders::keep_radio_silence(file, path, &mut world)?;
        if let Some(spec) = &file.objectives {
            let objectives = build_objectives(spec, file.days).map_err(&fail)?;
            world
                .set_objectives(objectives)
                .map_err(|e| fail(e.to_string()))?;
        }

        let contacts = beliefs::hold_contacts(file, path, &mut world)?;
        let estimates = beliefs::hold_estimates(file, path, &mut world)?;
        // The waters a side suspects enemy submarines work (decision 0218,
        // 47): told on their days.
        let waters = beliefs::hold_suspected_waters(file, path, &mut world)?;

        // What each side believed of the enemy (decision 0173, 5 to 9):
        // checked and kept with the scenario, never put into the world.
        let beliefs::Believed {
            forces: believed_forces,
            intents: believed_intents,
            men: believed_men,
        } = beliefs::build_believed(file, path, &world)?;

        Ok(Scenario {
            name: file.name.clone(),
            placeholder: file.placeholder,
            days: file.days,
            world,
            script,
            orders_inside_a_day,
            script_inside_day,
            commands,
            contacts,
            estimates,
            waters,
            command_levels,
            radio_silent,
            commanders,
            data_files,
            estimated_figures,
            believed_forces,
            believed_intents,
            believed_men,
            trial_figures,
            map_frame,
        })
    }
}
