//! Opening a game for play, and what is kept of it when play ends: the
//! function `serve`, `mcp` and `play` open a game with, here in the library so
//! that a test opens a game as they do (decision 0182, amended 25).

use std::collections::BTreeMap;
use std::path::Path;

use sim_agent::{Post, Seat, SeatKind, Session};
use sim_core::orders::OrderRequest;
use sim_core::state::Side;
use sim_data::{Disk, LoadOptions, Scenario, Source};

use crate::save;

/// The orders a script seat gives, by day.
fn script_of(
    script: &BTreeMap<u32, Vec<(Side, OrderRequest)>>,
    side: Side,
) -> BTreeMap<u32, Vec<OrderRequest>> {
    let mut by_day: BTreeMap<u32, Vec<OrderRequest>> = BTreeMap::new();
    for (day, list) in script {
        for (who, request) in list {
            if *who == side {
                by_day.entry(*day).or_default().push(request.clone());
            }
        }
    }
    by_day
}

/// What the saved game needs of a game besides its session.
pub struct Kept {
    scenario_path: String,
    seed: u64,
    remembered: save::Remembered,
    seats: BTreeMap<Side, SeatKind>,
}

/// Loads a scenario and opens a session over it with the given seats (a side
/// not named is the script's). `serve`, `mcp` and `play` all open games here.
pub fn open(
    scenario_path: &str,
    seed: Option<u64>,
    seats: BTreeMap<Side, SeatKind>,
    posts: &BTreeMap<Side, Post>,
) -> Result<(Session, Kept), String> {
    open_from(&Disk, scenario_path, seed, seats, posts)
}

/// As [`open`], reading the scenario and the files it names through `source`
/// (decision 0229, ruling 8).
pub fn open_from(
    source: &dyn Source,
    scenario_path: &str,
    seed: Option<u64>,
    seats: BTreeMap<Side, SeatKind>,
    posts: &BTreeMap<Side, Post>,
) -> Result<(Session, Kept), String> {
    let mut scenario =
        Scenario::load_via(source, Path::new(scenario_path), &LoadOptions::default())
            .map_err(|e| e.to_string())?;
    if let Some(seed) = seed {
        scenario = scenario.with_seed(seed).map_err(|e| e.to_string())?;
    }
    open_loaded_from(source, scenario_path, scenario, seats, posts)
}

/// [`open`] on a scenario already loaded from `scenario_path`.
pub fn open_loaded(
    scenario_path: &str,
    scenario: Scenario,
    seats: BTreeMap<Side, SeatKind>,
    posts: &BTreeMap<Side, Post>,
) -> Result<(Session, Kept), String> {
    open_loaded_from(&Disk, scenario_path, scenario, seats, posts)
}

/// As [`open_loaded`], hashing the scenario's files through `source`.
pub fn open_loaded_from(
    source: &dyn Source,
    scenario_path: &str,
    scenario: Scenario,
    mut seats: BTreeMap<Side, SeatKind>,
    posts: &BTreeMap<Side, Post>,
) -> Result<(Session, Kept), String> {
    for side in Side::ALL {
        seats.entry(side).or_insert(SeatKind::Script);
    }
    let remembered = save::remember_from(source, &scenario)?;
    let seat = |side: Side| match seats[&side] {
        SeatKind::Player => Seat::Player,
        SeatKind::Script => Seat::Script(script_of(remembered.script(), side)),
    };
    let (allied, japanese) = (seat(Side::Allied), seat(Side::Japanese));
    let seed = scenario.world.seed();
    let (name, days) = (scenario.name.clone(), scenario.days);
    let inside_day = scenario.orders_inside_a_day();
    // The staff each side is given comes from the doctrine; the commands are
    // the scenario's own, given as the commander's (decision 0107).
    let staffs = sim_staff::staffs();
    // What plays a seat's trials, where the scenario names a file of the
    // trial's figures (decision 0178, rule 6); with none, no trial.
    let trial = crate::seat_trial::SeatTrials::runner_of(&scenario);
    let mut session = Session::new(scenario.world, &name, days, allied, japanese)
        .with_staffs(staffs)
        .with_commands(remembered.commands().clone())
        .with_orders_inside_a_day(inside_day, remembered.script_inside_day().clone());
    if let Some(runner) = trial {
        session = session.with_trial(runner);
    }
    for (side, post) in posts {
        session = session.with_post(*side, *post)?;
    }
    Ok((
        session,
        Kept {
            scenario_path: scenario_path.to_string(),
            seed,
            remembered,
            seats,
        },
    ))
}

impl Kept {
    /// The seed the game is played on: the one given, or the scenario's.
    pub fn seed(&self) -> u64 {
        self.seed
    }

    /// Every data file the scenario names, with its hash, in the order the
    /// scenario names them, as they were when the game was opened.
    pub fn data_files(&self) -> &[(String, String)] {
        self.remembered.files()
    }

    /// Writes the saved game and the transcript, as asked.
    pub fn finish(
        &self,
        session: &Session,
        save: Option<&str>,
        transcript: Option<&str>,
    ) -> Result<(), String> {
        if let Some(file) = save {
            let text = self.saved_game_text(&Disk, session)?;
            std::fs::write(file, text).map_err(|e| format!("{file}: {e}"))?;
        }
        if let Some(file) = transcript {
            let text = transcript_text(session)?;
            std::fs::write(file, text).map_err(|e| format!("{file}: {e}"))?;
        }
        Ok(())
    }

    /// The text of the saved game as it stands, made and not written
    /// (decision 0229, ruling 12). The scenario file is hashed through
    /// `source`, which must be the one the game was opened from.
    pub fn saved_game_text(
        &self,
        source: &dyn Source,
        session: &Session,
    ) -> Result<String, String> {
        let hashes: Vec<(u32, u64)> = (1u32..).zip(session.day_hashes().iter().copied()).collect();
        save::write_text(
            source,
            &self.scenario_path,
            self.seed,
            &self.remembered,
            session.handed_in(),
            // The orders given inside a day, each with its pulse
            // (decision 0156).
            session.handed_in_inside_day(),
            session.commands_handed_in(),
            session.staff_handed_in(),
            &self.seats,
            &hashes,
            // A game that stopped inside a day says so (decision 0172,
            // 3), and so does a watched game left inside a day (decision
            // 0182, amended 19).
            session
                .stopped_inside_day()
                .or_else(|| session.left_inside_day()),
        )
    }
}

/// The text of a game's transcript, made and not written (decision 0229,
/// ruling 12).
pub fn transcript_text(session: &Session) -> Result<String, String> {
    let mut text = String::new();
    for entry in session.transcript() {
        text.push_str(&serde_json::to_string(&entry.to_json()).map_err(|e| e.to_string())?);
        text.push('\n');
    }
    Ok(text)
}
