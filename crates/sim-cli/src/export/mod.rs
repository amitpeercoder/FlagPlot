//! `sim-cli export`: a finished run as one JSON replay file for the viewer
//! (decision 0018). The file is the umpire's view: it carries the truth.
//!
//! Every `Fixed` quantity is a JSON number written from its millionths
//! (see [`num::Num`]); ids, counts, hexes and days are integers; state hashes
//! are 16-digit lowercase hex strings. The structs below are serialised in
//! declaration order, so the same run gives the same bytes.

mod data;
mod events;
mod map;
#[cfg(test)]
mod measure_fixture;
mod names;
mod num;
mod orders;
mod sides;
mod state;

use std::fs;
use std::path::Path;

use serde::Serialize;

use self::map::{LabelOut, MapOut};
use self::orders::OrderOut;
use self::sides::SidesOut;
use self::state::{BaseOut, FinalOut, Snapshot};

#[derive(Serialize)]
struct Replay {
    format: &'static str,
    version: u32,
    scenario: ScenarioOut,
    /// What the run was made on (decision 0057).
    inputs: crate::save::Inputs,
    map: MapOut,
    directions: Vec<[i32; 2]>,
    labels: Vec<LabelOut>,
    bases: Vec<BaseOut>,
    /// The reach figures of each aircraft type (version 3), for the viewer's
    /// rings: reference data, not state.
    aircraft_types: Vec<state::AircraftTypeOut>,
    initial: Snapshot,
    orders: Vec<OrderOut>,
    events: Vec<events::EventOut>,
    /// What each side may see of `events`, as the engine gives it (decision 0047).
    side_views: SidesOut,
    days: Vec<DayOut>,
    #[serde(rename = "final")]
    final_state: FinalOut,
}

#[derive(Serialize)]
struct ScenarioOut {
    name: String,
    placeholder: bool,
    /// The seed the run used.
    seed: u64,
    days: u32,
    /// The path as given on the command line.
    path: String,
}

#[derive(Serialize)]
struct DayOut {
    day: u32,
    hash: String,
    /// Index into `events` of the day's first event.
    first_event: usize,
    event_count: usize,
}

/// `sim-cli export <scenario.toml | game.json> --out <file.json> [--seed N] [--labels <anchors.csv>]`.
/// `args` are the words after `export`.
pub fn command(
    args: &[String],
    usage: &str,
    options: &sim_data::LoadOptions,
) -> Result<(), String> {
    let mut scenario_path: Option<&str> = None;
    let mut out: Option<&str> = None;
    let mut labels_path: Option<&str> = None;
    let mut seed: Option<u64> = None;

    let mut words = args.iter().map(String::as_str);
    while let Some(word) = words.next() {
        match word {
            "--seed" => seed = Some(crate::parse_seed(words.next())?),
            "--out" => {
                out = Some(
                    words
                        .next()
                        .ok_or_else(|| format!("--out needs a file\n\n{usage}"))?,
                );
            }
            "--labels" => {
                labels_path = Some(
                    words
                        .next()
                        .ok_or_else(|| format!("--labels needs a file\n\n{usage}"))?,
                );
            }
            flag if flag.starts_with('-') => {
                return Err(format!("unknown option \"{flag}\"\n\n{usage}"))
            }
            path if scenario_path.is_none() => scenario_path = Some(path),
            extra => return Err(format!("unexpected argument \"{extra}\"\n\n{usage}")),
        }
    }
    let scenario_path =
        scenario_path.ok_or_else(|| format!("no scenario or saved game given\n\n{usage}"))?;
    let out = out.ok_or_else(|| format!("export needs --out <file.json>\n\n{usage}"))?;

    let labels = match labels_path {
        Some(path) => map::labels(Path::new(path))?,
        None => Vec::new(),
    };
    let (executed, saved) = crate::execute_source(scenario_path, seed, options)?;
    // A saved game is of the scenario it names; the replay says so, and says
    // which game it is.
    let (scenario_file, title) = match &saved {
        Some(saved) => (
            saved.scenario_file.as_str(),
            Some(format!(
                "{} (saved game {scenario_path})",
                executed.scenario.name
            )),
        ),
        None => (scenario_path, None),
    };
    let inputs = crate::save::inputs(scenario_file, &executed.scenario)?;
    let mut replay = build(&executed, scenario_file, inputs, labels);
    if let Some(title) = title {
        replay.scenario.name = title;
    }

    let mut text = serde_json::to_string(&replay).map_err(|e| e.to_string())?;
    text.push('\n');
    fs::write(out, text).map_err(|e| format!("{out}: {e}"))
}

fn build(
    executed: &crate::Executed,
    scenario_path: &str,
    inputs: crate::save::Inputs,
    labels: Vec<LabelOut>,
) -> Replay {
    let scenario = &executed.scenario;
    let world = &scenario.world;
    let key = world.umpire_key();
    Replay {
        format: "flagplot-replay",
        version: 3,
        scenario: ScenarioOut {
            name: scenario.name.clone(),
            placeholder: scenario.placeholder,
            seed: world.seed(),
            days: scenario.days,
            path: scenario_path.to_string(),
        },
        inputs,
        map: map::map(world.map()),
        directions: map::directions(),
        labels,
        bases: state::bases(&executed.initial),
        aircraft_types: state::aircraft_types(world.ref_data()),
        initial: state::snapshot(&executed.initial, world.ref_data()),
        orders: world
            .orders()
            .map(|order| orders::order(order, &key))
            .collect(),
        events: world
            .events()
            .iter()
            .map(|event| events::event(event, &key))
            .collect(),
        side_views: sides::sides(world),
        days: executed
            .summaries
            .iter()
            .map(|summary| DayOut {
                day: summary.day,
                hash: format!("{:016x}", summary.state_hash),
                first_event: summary.first_event,
                event_count: summary.event_count,
            })
            .collect(),
        final_state: state::final_state(world),
    }
}
