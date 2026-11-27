use crate::error::DataError;
use crate::map::MapFrame;
use crate::source::{read_text, Disk, Noting, Pack, Source};
use sim_core::believed::{BelievedForce, BelievedIntent, BelievedMen};
use sim_core::choices::Moment;
use sim_core::command::CommandKind;
use sim_core::commander::NamedCommander;
use sim_core::ids::TaskForceId;
use sim_core::orders::OrderRequest;
use sim_core::pictures::{CommandRef, NamedSuperior};
use sim_core::refdata::TrialFigures;
use sim_core::state::{Side, WatersWaiting};
use sim_core::time::Pulse;
use sim_core::{DaySummary, Staffs, StartingContact, StartingEstimate, World};
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

mod air;
mod bases;
mod beliefs;
mod build;
mod built_on;
mod commanders;
mod file;
mod islands;
mod land;
mod objectives;
mod orders;
mod ships;
mod submarines;
mod supply;

pub(crate) use orders::SideSpec;

/// Builds the objectives from their table. Named bases are checked against
/// the game by `World::set_objectives`.
///
/// `days` is the scenario's length, which is the horizon unless the table
/// states a longer one.
/// A unit's day from the scenario (decision 0140, 1): a whole number from
/// 1, or none.
fn available_day(day: Option<u32>, unit: impl std::fmt::Display) -> Result<Option<u32>, String> {
    match day {
        Some(0) => Err(format!(
            "{unit} has available_from = 0: days are numbered from 1"
        )),
        day => Ok(day),
    }
}

/// The orders a scenario's script gives inside a day (decision 0156), by day,
/// each with the pulse it is handed in for (morning or afternoon) and its
/// side.
pub type ScriptInsideDay = BTreeMap<u32, Vec<(Pulse, Side, OrderRequest)>>;

/// A scenario ready to run: a game at its starting state plus the orders each
/// side will give on each day.
#[derive(Clone, Debug)]
pub struct Scenario {
    pub name: String,
    /// True when the scenario's units and geography are invented.
    pub placeholder: bool,
    /// How many days the scenario runs.
    pub days: u32,
    pub world: World,
    script: BTreeMap<u32, Vec<(Side, OrderRequest)>>,
    /// Whether a side may give orders inside a day (decision 0156): the
    /// scenario's `orders_inside_a_day`, off when absent.
    orders_inside_a_day: bool,
    /// The orders still to be given inside a day, by day, each with the
    /// pulse it is handed in for (morning or afternoon), in the order the
    /// file gives them (decision 0156). Empty where the key is off.
    script_inside_day: ScriptInsideDay,
    /// The commander's commands still to be given, by day, as they will be
    /// submitted (decision 0107, item 16).
    commands: BTreeMap<u32, Vec<(Side, CommandKind)>>,
    /// The contacts the sides start with, each with the command that saw
    /// it if the scenario names one (decision 0137, 6), kept so that a world
    /// built afresh for another seed holds them too.
    contacts: Vec<(StartingContact, Option<CommandRef>)>,
    /// The opening estimates, kept for the same reason (decision 0092, 16).
    estimates: Vec<StartingEstimate>,
    /// The suspected waters the scenario gives, kept for the same reason
    /// (decision 0218, 47).
    waters: Vec<WatersWaiting>,
    /// The superiors the scenario names, when command levels are on
    /// (decision 0134); `None` when they are off.
    command_levels: Option<Vec<NamedSuperior>>,
    /// The task forces that start on radio silence (decision 0136, 15), kept
    /// so that a world built afresh for another seed has them too.
    radio_silent: Vec<TaskForceId>,
    /// The commanders the scenario names (decision 0137), kept for the same
    /// reason. Held in the world only with command levels on.
    commanders: Vec<NamedCommander>,
    /// Every file the scenario names, as the paths it reads them from, in a
    /// fixed order (decision 0043, 9).
    data_files: Vec<PathBuf>,
    /// The figures of each class that rest on an estimate, from the class
    /// files' `estimated` column (decision 0207, rule 10): never hashed and
    /// read by no rule.
    estimated_figures: crate::reference::EstimatedFigures,
    /// The enemy forces each side believed in (decision 0173, 5): the
    /// scenario's, not the world's, never hashed and read by no rule.
    believed_forces: Vec<BelievedForce>,
    /// The intents each side believed the enemy held (decision 0173, 7),
    /// kept as the forces are.
    believed_intents: Vec<BelievedIntent>,
    /// The men of the enemy's each side believed may be ashore at a place
    /// beyond its estimate (decision 0176, 1), kept as the forces are.
    believed_men: Vec<BelievedMen>,
    /// A trial's figures (decision 0174), where the scenario names them:
    /// the scenario's, not the world's, never hashed and read by no rule.
    trial_figures: Option<TrialFigures>,
    /// The frame of the map the scenario is played on, from the map's header
    /// (decision 0199, 18): `None` for a hand-written map. The tools' and
    /// the export's, never the world's, never hashed and read by no rule.
    map_frame: Option<MapFrame>,
}

/// How a scenario is loaded, beyond the file itself.
///
/// The default reads every file where the scenario names it.
#[derive(Clone, Debug, Default)]
pub struct LoadOptions {
    /// A directory to read the scenario's reference (figures) files from, in
    /// place of the places it names. Each reference file the scenario names
    /// (ship classes, port sizes, aircraft types, and the search, surface
    /// combat, air combat, air upkeep, fatigue and base supply figures, the
    /// file of sides of decision 0173 and a trial's figures of decision 0174)
    /// is
    /// looked up by its file name directly under this directory. A file that
    /// is not there is an error that names it: there is no fallback to the
    /// file the scenario names, which would defeat the purpose. The map is
    /// not a reference file and is read where the scenario names it, and so
    /// are the report figures (decision 0135), which the frozen step 1
    /// figures do not have.
    ///
    /// The hand-calculation tests set this to `data/reference/step1`, the
    /// figures as they stood at the end of step 1 (`docs/step2-spec.md`,
    /// "Tests while figures move").
    pub reference_root: Option<PathBuf>,
}

/// Where the frozen figures of step 1 are, relative to the repository's root.
/// The one place the directory is named in code: the tests that read them
/// (`tests/common`, the agent's tests, the runner's tests) all go through
/// [`LoadOptions::frozen_step1`].
pub const FROZEN_REFERENCE_DIR: &str = "data/reference/step1";

impl LoadOptions {
    /// Options that read the reference files from `root`.
    pub fn with_reference_root(root: impl Into<PathBuf>) -> LoadOptions {
        LoadOptions {
            reference_root: Some(root.into()),
        }
    }

    /// Options that read the frozen figures of step 1, as the hand-calculation
    /// tests do, given the repository's root (`FROZEN_REFERENCE_DIR` under it).
    pub fn frozen_step1(repo_root: &Path) -> LoadOptions {
        LoadOptions::with_reference_root(repo_root.join(FROZEN_REFERENCE_DIR))
    }
}

impl Scenario {
    /// Loads a scenario and the map and reference files it names.
    pub fn load(path: &Path) -> Result<Scenario, DataError> {
        Scenario::load_with(path, &LoadOptions::default())
    }

    /// Loads a scenario as [`Scenario::load`] does, with `options`. The files
    /// the scenario records as its inputs ([`Scenario::data_files`]) are the
    /// ones actually read, so a saved game made with a reference root names
    /// that root's files and its hashes are theirs.
    pub fn load_with(path: &Path, options: &LoadOptions) -> Result<Scenario, DataError> {
        Scenario::load_via(&Disk, path, options)
    }

    /// As [`Scenario::load_with`], reading the scenario and every file it
    /// names through `source` (decision 0229, ruling 8): the disk, or a pack.
    pub fn load_via(
        source: &dyn Source,
        path: &Path,
        options: &LoadOptions,
    ) -> Result<Scenario, DataError> {
        let text = read_text(source, path)?;
        Scenario::parse_via(source, &text, path, options)
    }

    /// The files the load of the scenario at `path` reads, by the paths it
    /// asks for, with their text: the pack of that scenario (decision 0229,
    /// ruling 9). Made by loading it from `source` through a source that
    /// notes what it is asked for, so the pack holds exactly the files the
    /// load read and no list of them is kept by hand.
    pub fn pack_of(
        source: &dyn Source,
        path: &Path,
        options: &LoadOptions,
    ) -> Result<Pack, DataError> {
        let noting = Noting::new(source);
        Scenario::load_via(&noting, path, options)?;
        Ok(noting.pack())
    }

    /// Builds a scenario from text that is not (or no longer) the file's own
    /// bytes, for tests that load a file with its tables in another order
    /// (decision 0043, 2). `path` says where the text would sit: the files it
    /// names are read relative to it.
    pub fn from_text(text: &str, path: &Path) -> Result<Scenario, DataError> {
        Scenario::parse(text, path)
    }

    /// As [`Scenario::from_text`], with `options` as in [`Scenario::load_with`].
    pub fn from_text_with(
        text: &str,
        path: &Path,
        options: &LoadOptions,
    ) -> Result<Scenario, DataError> {
        Scenario::parse_with(text, path, options)
    }

    /// Builds a scenario from its text. `path` says where the text came from:
    /// the files it names are read relative to it.
    fn parse(text: &str, path: &Path) -> Result<Scenario, DataError> {
        Scenario::parse_with(text, path, &LoadOptions::default())
    }

    /// As `parse`, reading the reference files as `options` say.
    fn parse_with(text: &str, path: &Path, options: &LoadOptions) -> Result<Scenario, DataError> {
        Scenario::parse_via(&Disk, text, path, options)
    }

    /// The frame of the scenario's map, from its header (decision 0199):
    /// `None` for a hand-written map, which has none.
    pub fn map_frame(&self) -> Option<&MapFrame> {
        self.map_frame.as_ref()
    }

    /// A trial's figures (decision 0174), where the scenario names them.
    pub fn trial_figures(&self) -> Option<&TrialFigures> {
        self.trial_figures.as_ref()
    }

    /// The enemy forces `side` believed in and held no contact of when the
    /// game began (decision 0173, 5), in the order the file gives them.
    pub fn believed_forces(&self, side: Side) -> impl Iterator<Item = &BelievedForce> + '_ {
        self.believed_forces
            .iter()
            .filter(move |force| force.side == side)
    }

    /// The intents `side` believed the enemy held when the game began
    /// (decision 0173, 7), in the order the file gives them.
    pub fn believed_intents(&self, side: Side) -> impl Iterator<Item = &BelievedIntent> + '_ {
        self.believed_intents
            .iter()
            .filter(move |intent| intent.side == side)
    }

    /// The men of the enemy's `side` believed may be ashore at a place beyond
    /// its estimate when the game began (decision 0176, 1), in the order the
    /// file gives them.
    pub fn believed_men(&self, side: Side) -> impl Iterator<Item = &BelievedMen> + '_ {
        self.believed_men.iter().filter(move |men| men.side == side)
    }

    /// The superiors the scenario names, if command levels are on (decision
    /// 0134): `None` when they are off, `Some` (empty if it names none) when
    /// on.
    pub fn command_levels(&self) -> Option<&[NamedSuperior]> {
        self.command_levels.as_deref()
    }

    /// The files the scenario names (map, ship classes, port sizes, aircraft
    /// types, then the figures files it has), as the paths they are read from.
    /// The classes the scenario loads with figures that rest on an estimate,
    /// each with the figures (decision 0207, rule 10).
    pub fn estimated_figures(&self) -> &crate::reference::EstimatedFigures {
        &self.estimated_figures
    }

    pub fn data_files(&self) -> &[PathBuf] {
        &self.data_files
    }

    /// The orders still to be given, by day, as they will be submitted.
    pub fn scripted_orders(&self) -> &BTreeMap<u32, Vec<(Side, OrderRequest)>> {
        &self.script
    }

    /// Replaces the scripted orders, for replaying a saved game: the orders
    /// given are submitted day by day in place of the scenario's own.
    pub fn replace_script(&mut self, script: BTreeMap<u32, Vec<(Side, OrderRequest)>>) {
        self.script = script;
    }

    /// Whether a side may give orders inside a day, at 0600 and 1200
    /// (decision 0156): the scenario's key `orders_inside_a_day`, off when
    /// absent.
    pub fn orders_inside_a_day(&self) -> bool {
        self.orders_inside_a_day
    }

    /// The orders still to be given inside a day, by day, each with its
    /// pulse and side, in the order the file gives them (decision 0156).
    pub fn scripted_orders_inside_day(&self) -> &ScriptInsideDay {
        &self.script_inside_day
    }

    /// Replaces the orders given inside a day, for replaying a saved game
    /// (decision 0156), as [`Scenario::replace_script`] replaces those of
    /// dusk.
    pub fn replace_script_inside_day(&mut self, script: ScriptInsideDay) {
        self.script_inside_day = script;
    }

    /// The commander's commands still to be given, by day, as they will be
    /// submitted (decision 0107, item 16).
    pub fn scripted_commands(&self) -> &BTreeMap<u32, Vec<(Side, CommandKind)>> {
        &self.commands
    }

    /// Replaces the scripted commands, for replaying a saved game: the
    /// commands it handed in are submitted day by day in place of the
    /// scenario's own.
    pub fn replace_commands(&mut self, commands: BTreeMap<u32, Vec<(Side, CommandKind)>>) {
        self.commands = commands;
    }

    /// A copy of this scenario with a different seed, for a run of its own.
    /// The world is built afresh, so the scenario must not have started: it
    /// is an error once any day has been resolved. The script, name and
    /// length are kept. Used by the 1,000-seed test and by `--seed`.
    pub fn with_seed(&self, seed: u64) -> Result<Scenario, DataError> {
        if self.world.day() != 0 {
            return Err(DataError {
                path: None,
                message: format!(
                    "the seed can only change before day 1, and day {} has been resolved",
                    self.world.day()
                ),
            });
        }
        let mut world = World::new(
            seed,
            self.world.ref_data().clone(),
            self.world.map().clone(),
            self.world.umpire().clone(),
        )?;
        if let Some(named) = &self.command_levels {
            world.turn_command_levels_on(named)?;
        }
        world.name_commanders(&self.commanders)?;
        for force in &self.radio_silent {
            world.keep_radio_silence_from_the_start(*force)?;
        }
        world.hold_starting_contacts_seen(self.contacts.clone())?;
        world.hold_starting_estimates(self.estimates.clone())?;
        world.hold_suspected_waters(self.waters.clone())?;
        if let Some(objectives) = self.world.objectives() {
            world.set_objectives(objectives.clone())?;
        }
        if let Some(days) = self.world.game_days() {
            world.set_game_days(days);
        }
        Ok(Scenario {
            name: self.name.clone(),
            placeholder: self.placeholder,
            days: self.days,
            world,
            script: self.script.clone(),
            orders_inside_a_day: self.orders_inside_a_day,
            script_inside_day: self.script_inside_day.clone(),
            commands: self.commands.clone(),
            contacts: self.contacts.clone(),
            estimates: self.estimates.clone(),
            waters: self.waters.clone(),
            command_levels: self.command_levels.clone(),
            radio_silent: self.radio_silent.clone(),
            commanders: self.commanders.clone(),
            data_files: self.data_files.clone(),
            estimated_figures: self.estimated_figures.clone(),
            believed_forces: self.believed_forces.clone(),
            believed_intents: self.believed_intents.clone(),
            believed_men: self.believed_men.clone(),
            trial_figures: self.trial_figures.clone(),
            map_frame: self.map_frame,
        })
    }

    /// Submits the scripted orders for the next day and resolves it, with no
    /// staff for either side. Returns `None` once the scenario has run all
    /// its days.
    pub fn next_day(&mut self) -> Result<Option<DaySummary>, DataError> {
        self.next_day_watched(Staffs::none(), &mut |_, _| {})
    }

    /// [`Scenario::next_day`], with each side's staff in `staffs` asked at
    /// its step of the day (decision 0107).
    pub fn next_day_staffed(
        &mut self,
        staffs: Staffs<'_>,
    ) -> Result<Option<DaySummary>, DataError> {
        self.next_day_watched(staffs, &mut |_, _| {})
    }

    /// [`Scenario::next_day_staffed`], handing the world, read only, to
    /// `watch` at each moment of the day as [`World::resolve_day_watched`]
    /// does. The day is the same day: `next_day_staffed` is this with a
    /// watcher that does nothing.
    pub fn next_day_watched(
        &mut self,
        staffs: Staffs<'_>,
        watch: &mut dyn FnMut(&World, Moment),
    ) -> Result<Option<DaySummary>, DataError> {
        let day = self.world.day() + 1;
        if day > self.days {
            return Ok(None);
        }
        // The commander's commands are handed in before the day resolves and
        // taken in at its first step (decision 0107).
        for (side, command) in self.commands.remove(&day).unwrap_or_default() {
            self.world.submit_commands(side, vec![command]);
        }
        for (side, request) in self.script.remove(&day).unwrap_or_default() {
            self.world.submit_orders(side, vec![request]);
        }
        // A day with no order inside it resolves whole, as it always has.
        let inside = self.script_inside_day.remove(&day).unwrap_or_default();
        if inside.is_empty() {
            return Ok(Some(self.world.resolve_day_watched(staffs, watch)?));
        }
        // A day with orders inside it is resolved in its parts (decision
        // 0156), each order handed in before the pulse it is for, in the
        // order the file gives them.
        self.world.open_day(staffs, watch)?;
        for pulse in Pulse::ALL {
            for (at, side, request) in &inside {
                if *at == pulse {
                    self.world
                        .submit_orders_inside_day(*side, vec![request.clone()])?;
                }
            }
            self.world.resolve_next_pulse(watch)?;
        }
        Ok(Some(self.world.close_day(watch)?))
    }

    /// Runs every remaining day and returns their summaries.
    pub fn run_to_end(&mut self) -> Result<Vec<DaySummary>, DataError> {
        let mut summaries = Vec::new();
        while let Some(summary) = self.next_day()? {
            summaries.push(summary);
        }
        Ok(summaries)
    }
}

#[cfg(test)]
mod tests;
