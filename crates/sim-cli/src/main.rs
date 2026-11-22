//! Flag Plot command-line runner.
//!
//! Commands: `run`, `export`, `batch`, `replay`, `hash`, `events` and `serve`
//! (the spec's "query events" is `events`).

mod batch;
mod batch_record;
mod events;
mod export;
mod mcp;
mod play;
use sim_cli::save;
mod serve;
mod staff_tools;
mod timing;
mod validate;

use std::collections::BTreeMap;
use std::env;
use std::path::Path;
use std::process::ExitCode;

use sim_core::events::EventKind;
use sim_core::fixed::Fixed;
use sim_core::orders::{AirMission, Posture, Stance};
use sim_core::state::{AirGroupLocation, ShipLocation, ShipSystem, Side, SystemState, TruthState};
use sim_core::DaySummary;
use sim_data::{LoadOptions, Scenario};

const USAGE: &str = "\
Usage: sim-cli run <scenario.toml> [--events] [--seed N] [--save <game.json>] [--reference DIR]
       sim-cli replay <game.json> [--check-staff] [--reference DIR]
       sim-cli hash <scenario.toml> [--seed N] [--reference DIR]
       sim-cli events <scenario.toml | game.json> --unit HANDLE [--day N] [--seed N] [--reference DIR]
       sim-cli events <scenario.toml | game.json> --why EVENT [--seed N] [--reference DIR]
       sim-cli events <scenario.toml | game.json> --report EVENT [--command COMMAND] [--seed N] [--json] [--reference DIR]
       sim-cli events <scenario.toml | game.json> --strike air_group:N --day D --pulse night|morning|afternoon
                      [--report EVENT] [--seed N] [--json] [--reference DIR]
       sim-cli export <scenario.toml | game.json> --out <file.json> [--seed N] [--labels <anchors.csv>] [--reference DIR]
       sim-cli batch <scenario.toml> --out <file.json> [--runs N] [--first-seed S] [--label TEXT] [--reference DIR]
       sim-cli validate <batch.json> <measures.toml> [--out <report.md>] [--reference DIR]
       sim-cli validate <measures.toml> [--out <report.md>]
       sim-cli validate --passages <passages.toml> --label TEXT [--map <map.csv>] [--places <positions.csv>] [--out <report.md>]
       sim-cli serve <scenario.toml> --seat allied=player|script --seat japanese=player|script
                     [--at allied=SEAT] [--at japanese=SEAT] [--seed N] [--save <game.json>] [--transcript <file.jsonl>]
       sim-cli mcp <scenario.toml> --seat allied|japanese
                   [--at SEAT] [--seed N] [--save <game.json>] [--transcript <file.jsonl>]
       sim-cli time --machine TEXT [--repeats N] [--games N] [--days N] [--slice FILE] [--soak FILE]
                    [--early-day N] [--late-day N] [--soak-late-day N] [--reference DIR]
                    [--trial-slice FILE] [--trial-figures FILE] [--trial-late-day N] [--only-trial]
                    [--trial-size small|standard|full] [--only-window] [--window-runs N]
                    [--only-seat-trial] [--seat-trial-runs N]
       sim-cli play --game FILE ... (run `sim-cli play` for its usage)

  run       Runs a scenario to its last day and prints the state hash of
            each day, then where every task force and ship ended up, and for a
            scenario with objectives (decision 0054) the result.
  replay    Runs a saved game: refuses it, before it runs, if it was saved
            under another state layout or rules label, or before saved games
            recorded them (decision 0100); loads the scenario the file names,
            refuses it if the scenario file or any data file it names has
            changed, submits
            the saved orders day by day in place of the scenario's own, prints
            each day's hash and compares it with the saved one. Exits with an
            error on the first day that differs. Saved game version 3 holds
            the commander's commands and what each side's staff handed in;
            the replay hands them in again and asks no staff. With
            --check-staff it asks the doctrine's staff as well, on each day
            and for each side that has one, and stops at the first day and
            side where the staff gives something other than what was saved,
            saying what; if every day's hashes match and the staff gave
            what was saved on every day, it says so.
  hash      Runs a scenario and prints the state hash of each day, one a line
            as `day N <hash>`, and nothing else.
  events    The umpire's questions to a finished run (a scenario run to its
            end, or a saved game replayed). --unit HANDLE (ship:N,
            task_force:N, air_group:N, base:N, land_unit:N, submarine:N, contact:N or allied:contact:N)
            prints every event that names the unit, as `run --events` prints
            it; for a ship it also prints the events of the task force it was
            in, marked `via task_force:N`. --day N keeps one day. --why EVENT
            prints that event and its causes down to orders, as a tree, with
            each event's breakdown.
  time      Times what a day costs, outside the game (decision 0100, 11 to 13):
            one day of the slice (default 07_full_slice: day 2 from day 1 and
            day 13 from day 12) resolved, and resolved with every question of
            both sides asked at every moment of it; a copy of the world with
            and without its event log; and --games (100) games of --days (30)
            from the start of the soak and from its day 150. Each is a spread
            (min, median, max) over --repeats (20) runs, except the games, one
            run each. Last, a trial of the Allied side (decision 0175, 9) on
            --trial-slice (default stage 3's proof's 07a_slice): from the
            evening before day 1 with the side's own commands of day 1 as the
            course, and from the evening of --trial-late-day (20), the slice
            played to it with both staffs, with an empty course; at each of
            the three sizes of --trial-figures (the slice's own, else
            data/reference/trial.toml), once each, with both staffs.
            --only-trial times the trial alone; --trial-size one size only.
            Then a window (decision 0177) of the same course from the evening
            before day 1, at each size, at the dates it chooses and, where it
            chooses fewer than three, at the first and 3 and 5 days after it;
            --window-runs (1) times each; --only-window times the window alone.
            Then a seat's trial (decision 0178) of that intent, at each size,
            without and with the second trial; --seat-trial-runs (1) times
            each; --only-seat-trial times it alone.
            --machine names the machine in the report. It ends with
            the totals that matter. Use a release build; it changes nothing in
            any scenario, save or hash.
  validate  The umpire's judge of a batch (decision 0062's file, version 5)
            against a measures file of data/validation (docs/step2-spec.md):
            takes each measure's number from every run and writes a markdown
            report, by default docs/validation/<battle>-<label>.md, with the
            verdict on each measure. Refuses a batch with no label, of another
            version or state hash layout, or whose inputs no longer match the
            files on disk (give the same --reference), and a batch of a
            scenario for play (decision 0187: its results are not for the
            record). With a measures file
            alone, for a scenario not built, every measure is reported as not
            run. With --passages it works each ship passage of a passages file as
            one calculation (the miles the rules charge between two hexes, at
            the stated speeds) and reports it beside the record, judged by
            nothing; it reads data/oob/place_positions.csv (or --places FILE)
            for the places a passage names. It exits with success whenever it
            wrote its report, misses or not.
  export    Runs a scenario the same way, or replays a saved game as replay
            does (with --reference as it was made; --seed does not go with
            it), and writes the finished run as one JSON replay file: the map,
            the starting forces, every event with its causes, and the final
            state. A saved game's replay is titled with the game's file and
            its scenario.
  batch     Runs a scenario N times with seeds S, S+1, ... one after another
            and writes one JSON summary file: ships sunk per side (a
            histogram and totals), per ship, and the final hash and sunk
            ships of every run. For a scenario with objectives it adds each
            run's Allied margin and result and a tally of the results. Prints
            a short summary.
  serve     Holds one game open for play on standard input and output, one JSON
            request a line and one JSON answer a line, and opens no port. A
            request is {id, seat, tool, arguments}, for example
            {\"id\": 1, \"seat\": \"allied\", \"tool\": \"get_briefing\"}; the
            answer is {id, result} or {id, error}. The tools are get_briefing,
            list_forces, list_contacts, list_bases, get_entity, measure,
            explain, issue_orders and end_day. A seat held by the script cannot
            be called; the day resolves when every player seat has ended it. At
            least one seat must be a player's (the other defaults to script).
            At the end of input it writes --save (a saved game with the seats
            and the orders handed in by both sides, which replay runs with no
            player) and --transcript (one JSON line for each call).
  --at      (serve, mcp, play) Where a player's seat sits: area (the default),
            area:easier (the side's own picture, every report at once, and its
            forces exactly), task_force:N or base:N (N is the number the
            scenario file gives it; command levels on): a seat given only what
            that command holds, which may order its own units only. serve takes
            it as side=SEAT, mcp and play bare, beside the seat they name.
  mcp       The game as a Model Context Protocol server on standard input and
            output (tools only: the nine tools of serve, answered as text),
            with the named seat the player's and the other the script's. At
            the end of input it writes --save and --transcript as serve does.
  play      One call to that server from a command line, for a player with
            nothing else: the game is kept in a file of the calls so far.
  --events  (run) Also prints the full event log, with what caused each event.
  --seed N  Runs with seed N instead of the scenario's own seed.
  --save F  (run) Also writes a saved game: the scenario's file and its hash
            and those of its data files, the seed, the orders of each day with
            their origin, and the daily hashes. It holds no state.
  --out F   (export, batch) The file to write.
  --runs N  (batch) Number of runs, at least 1. Default 1000.
  --first-seed S  (batch) Seed of the first run. Default 1.
  --labels F  (export) An anchors CSV (place, feature, lat, lon) to put place
            names on the replay's map.
  --reference DIR  (run, replay, hash, events, export, batch) Reads the
            scenario's reference files (ship classes, port sizes, aircraft
            types, and the search, surface combat, air combat, air upkeep,
            fatigue and base supply figures) from DIR, each by its file name,
            in place of where the scenario names it. A file the scenario names
            that is not in DIR is an error; the map is read where the scenario
            names it. The hand-calculation tests use data/reference/step1. A
            saved game made with it names DIR's files and replays only with
            the same --reference and the same files.";

/// A scenario that has been loaded, given its seed and run to its last day.
/// Both commands build one with [`execute`].
struct Executed {
    scenario: Scenario,
    summaries: Vec<DaySummary>,
    /// What each side's staff handed in, by day, for the saved game: the
    /// staff the run asked, or the hand-ins a replay gave back.
    staff_handed_in: save::StaffHandIns,
    /// The truth as it stood after loading and seeding, before day 1.
    initial: TruthState,
}

/// Takes `--reference DIR` out of the words after a command, wherever it is,
/// and says how the scenario is then loaded. The six commands that load a
/// scenario for a run (`run`, `replay`, `hash`, `events`, `export`, `batch`)
/// share this, so each takes the option the same way.
fn take_reference(args: &[String]) -> Result<(Vec<String>, LoadOptions), String> {
    let mut rest = Vec::new();
    let mut options = LoadOptions::default();
    let mut words = args.iter();
    while let Some(word) = words.next() {
        if word == "--reference" {
            let dir = words
                .next()
                .ok_or_else(|| format!("--reference needs a directory\n\n{USAGE}"))?;
            if options.reference_root.is_some() {
                return Err(format!("--reference is given twice\n\n{USAGE}"));
            }
            options.reference_root = Some(dir.into());
        } else {
            rest.push(word.clone());
        }
    }
    Ok((rest, options))
}

/// The one way a scenario is run: load it, apply the seed if one is given,
/// call `before_run` (the runner prints its header there, so a run that fails
/// partway still shows which scenario and seed it was), note the starting
/// truth, run every day.
fn execute(
    path: &str,
    seed: Option<u64>,
    options: &LoadOptions,
    before_run: impl FnOnce(&mut Scenario),
) -> Result<Executed, String> {
    execute_observing(path, seed, options, before_run, |_| {})
}

/// [`execute`], calling `after_day` with the scenario after each day is
/// resolved. The days are the same days in the same order, so the run, its
/// events and its hashes are those of `execute`. `batch` reads the state at
/// the end of each day this way.
fn execute_observing(
    path: &str,
    seed: Option<u64>,
    options: &LoadOptions,
    before_run: impl FnOnce(&mut Scenario),
    after_day: impl FnMut(&Scenario),
) -> Result<Executed, String> {
    execute_watching(path, seed, options, before_run, after_day, &mut |_, _| {})
}

/// [`execute_observing`], handing the world, read only, to `watch` at each
/// moment of each day (`Scenario::next_day_watched`). The days are the same
/// days, so the run is that of `execute`. `export` records the state at each
/// step this way.
fn execute_watching(
    path: &str,
    seed: Option<u64>,
    options: &LoadOptions,
    before_run: impl FnOnce(&mut Scenario),
    mut after_day: impl FnMut(&Scenario),
    watch: &mut dyn FnMut(&sim_core::World, sim_core::choices::Moment),
) -> Result<Executed, String> {
    let mut scenario = Scenario::load_with(Path::new(path), options).map_err(|e| e.to_string())?;
    if let Some(seed) = seed {
        scenario = scenario.with_seed(seed).map_err(|e| e.to_string())?;
    }
    before_run(&mut scenario);
    let initial = scenario.world.umpire().clone();
    let held = sim_staff::staffs();
    let mut summaries = Vec::new();
    let mut staff_handed_in = save::StaffHandIns::new();
    while let Some(summary) = scenario
        .next_day_watched(sim_staff::as_staffs(&held), watch)
        .map_err(|e| e.to_string())?
    {
        note_staff(&scenario.world, summary.day, &mut staff_handed_in);
        summaries.push(summary);
        after_day(&scenario);
    }
    Ok(Executed {
        scenario,
        summaries,
        staff_handed_in,
        initial,
    })
}

/// What each side's staff handed in on the day just resolved, noted for the
/// saved game.
fn note_staff(world: &sim_core::World, day: u32, into: &mut save::StaffHandIns) {
    for (side, decisions) in world.staff_handed_in() {
        if !decisions.is_empty() {
            into.entry(day)
                .or_default()
                .insert(*side, decisions.clone());
        }
    }
}

/// A saved game run again, the one way `replay`, `events` and `export` do it:
/// load the scenario file the game names, refuse it if it or a data file has
/// changed, give it the game's seed, orders and commands and, before each
/// day, what each side's staff handed in (no staff is asked), then run it day
/// by day and compare each day's hash with the saved one, failing on the
/// first that differs. `before_run` sees the scenario and the game before day
/// 1; `on_day` sees each summary before it is compared. With `to_end` the run
/// goes on to the scenario's last day after the saved days (`replay` stops
/// with them). `watch` is handed the world, read only, at each moment of each
/// day, as [`execute_watching`] does. [`execute_saved_checking`] is the same
/// run with the doctrine's staff asked as well.
fn execute_saved_watching(
    file: &str,
    options: &LoadOptions,
    to_end: bool,
    before_run: impl FnOnce(&Scenario, &save::Saved),
    on_day: impl FnMut(&DaySummary),
    watch: &mut dyn FnMut(&sim_core::World, sim_core::choices::Moment),
) -> Result<(Executed, save::Saved), String> {
    execute_saved_checking(file, options, to_end, false, before_run, on_day, watch)
        .map(|(executed, saved, _)| (executed, saved))
}

/// The first place the doctrine's staff gave something other than what a
/// saved game holds, for `replay --check-staff`.
struct StaffFinding {
    day: u32,
    side: Side,
    what: String,
}

/// [`execute_saved_watching`], and with `check_staff` also asking the
/// doctrine's staff (decision 0107, 10): on each saved day each side with a
/// staff in [`sim_staff::staffs`] is wrapped in a
/// [`sim_core::command::CheckedStaff`] holding the saved hand-in (nothing if
/// none was saved), which gives the saved hand-in back whatever the staff
/// gave and keeps the first difference. A saved hand-in for a side the
/// doctrine gives no staff is a difference too. The run stops at the first
/// difference found on a day, and gives it as the third part; a difference
/// in the staff is reported before a hash that differs on the same day.
fn execute_saved_checking(
    file: &str,
    options: &LoadOptions,
    to_end: bool,
    check_staff: bool,
    before_run: impl FnOnce(&Scenario, &save::Saved),
    mut on_day: impl FnMut(&DaySummary),
    watch: &mut dyn FnMut(&sim_core::World, sim_core::choices::Moment),
) -> Result<(Executed, save::Saved, Option<StaffFinding>), String> {
    let saved = save::read(file)?;
    let scenario =
        Scenario::load_with(Path::new(&saved.scenario_file), options).map_err(|e| e.to_string())?;
    saved.check_files(&scenario)?;
    let held = sim_staff::staffs();
    let mut scenario = scenario.with_seed(saved.seed).map_err(|e| e.to_string())?;
    scenario.replace_script(saved.script.clone());
    scenario.replace_script_inside_day(saved.script_inside_day.clone());
    scenario.replace_commands(saved.commands.clone());
    before_run(&scenario, &saved);
    let initial = scenario.world.umpire().clone();
    let mut summaries = Vec::new();
    let mut staff_handed_in = save::StaffHandIns::new();
    let mut finding: Option<StaffFinding> = None;
    for (day, hash) in &saved.hashes {
        let handed = saved.staff.get(day);
        let summary = if check_staff {
            // Each side with a staff gives the saved hand-in back through its
            // checked wrapper; the other sides' saved hand-ins are handed in
            // as a replay does, and are a difference where there are any.
            let wrapped: BTreeMap<Side, sim_core::command::CheckedStaff<'_>> = held
                .iter()
                .map(|(side, real)| {
                    let saved_for_side = handed
                        .and_then(|by_side| by_side.get(side))
                        .cloned()
                        .unwrap_or_default();
                    (
                        *side,
                        sim_core::command::CheckedStaff::new(
                            real.as_ref(),
                            *side,
                            *day,
                            saved_for_side,
                        ),
                    )
                })
                .collect();
            for side in Side::ALL {
                if wrapped.contains_key(&side) {
                    continue;
                }
                if let Some(decisions) = handed.and_then(|by_side| by_side.get(&side)) {
                    scenario.world.submit_staff(side, decisions.clone());
                    if finding.is_none() {
                        finding = Some(StaffFinding {
                            day: *day,
                            side,
                            what: format!(
                                "the game holds what the {} staff handed in, and the doctrine ({}) gives that side no staff",
                                side.name().to_lowercase(),
                                sim_staff::DOCTRINE_VERSION
                            ),
                        });
                    }
                }
            }
            let mut staffs = sim_core::Staffs::none();
            for (side, checked) in &wrapped {
                staffs = staffs.with(*side, checked);
            }
            let summary = scenario
                .next_day_watched(staffs, watch)
                .map_err(|e| e.to_string())?
                .ok_or_else(|| format!("the scenario has no day {day}"))?;
            if finding.is_none() {
                finding = wrapped.values().find_map(|checked| {
                    checked.difference().map(|d| StaffFinding {
                        day: d.day,
                        side: d.side,
                        what: d.what,
                    })
                });
            }
            summary
        } else {
            for (side, decisions) in handed.into_iter().flatten() {
                scenario.world.submit_staff(*side, decisions.clone());
            }
            scenario
                .next_day_watched(sim_core::Staffs::none(), watch)
                .map_err(|e| e.to_string())?
                .ok_or_else(|| format!("the scenario has no day {day}"))?
        };
        note_staff(&scenario.world, summary.day, &mut staff_handed_in);
        on_day(&summary);
        let now = format!("{:016x}", summary.state_hash);
        let differs = summary.day != *day || &now != hash;
        summaries.push(summary);
        // A difference in what the staff gives comes first: it is the first
        // difference, and a hash that differs on the same day is most likely
        // what it caused.
        if finding.is_some() {
            break;
        }
        if differs {
            return Err(format!(
                "replay differs on day {day}: saved hash {hash}, replayed hash {now}"
            ));
        }
    }
    if to_end && finding.is_none() {
        while let Some(summary) = scenario
            .next_day_watched(sim_core::Staffs::none(), watch)
            .map_err(|e| e.to_string())?
        {
            note_staff(&scenario.world, summary.day, &mut staff_handed_in);
            on_day(&summary);
            summaries.push(summary);
        }
    }
    let executed = Executed {
        scenario,
        summaries,
        staff_handed_in,
        initial,
    };
    Ok((executed, saved, finding))
}

/// What `events` and `export` take: a scenario file, run with its own
/// orders (and `--seed` if given), or a saved game, replayed to its end
/// ([`execute_saved`]); a saved game holds its own seed, so `--seed` does not
/// go with it. The second part is the saved game, when it was one.
fn execute_source(
    source: &str,
    seed: Option<u64>,
    options: &LoadOptions,
) -> Result<(Executed, Option<save::Saved>), String> {
    execute_source_watching(source, seed, options, &mut |_, _| {})
}

/// [`execute_source`], handing the world, read only, to `watch` at each
/// moment of each day, for a scenario and for a saved game alike.
fn execute_source_watching(
    source: &str,
    seed: Option<u64>,
    options: &LoadOptions,
    watch: &mut dyn FnMut(&sim_core::World, sim_core::choices::Moment),
) -> Result<(Executed, Option<save::Saved>), String> {
    let text = std::fs::read_to_string(source).map_err(|e| format!("{source}: {e}"))?;
    if !text.trim_start().starts_with('{') {
        return Ok((
            execute_watching(source, seed, options, |_| {}, |_| {}, watch)?,
            None,
        ));
    }
    if seed.is_some() {
        return Err("--seed does not go with a saved game: it holds its own seed".to_string());
    }
    let (executed, saved) =
        execute_saved_watching(source, options, true, |_, _| {}, |_| {}, watch)?;
    Ok((executed, Some(saved)))
}

/// Reads the number after `--seed`.
fn parse_seed(value: Option<&str>) -> Result<u64, String> {
    let value = value.ok_or_else(|| format!("--seed needs a number\n\n{USAGE}"))?;
    value
        .parse::<u64>()
        .map_err(|_| format!("--seed \"{value}\" is not a whole number"))
}

fn main() -> ExitCode {
    let args: Vec<String> = env::args().skip(1).collect();
    let result = match args.first().map(String::as_str) {
        Some("events") => with_reference(&args[1..], events::command),
        Some("export") => with_reference(&args[1..], |a, o| export::command(a, USAGE, o)),
        Some("batch") => with_reference(&args[1..], |a, o| batch::command(a, USAGE, o)),
        Some("validate") => with_reference(&args[1..], |a, o| validate::command(a, USAGE, o)),
        Some("time") => with_reference(&args[1..], |a, o| timing::command(a, USAGE, o)),
        Some("replay") => with_reference(&args[1..], replay),
        Some("serve") => serve::command(&args[1..], USAGE),
        Some("mcp") => mcp::command(&args[1..], USAGE),
        Some("play") => play::command(&args[1..]),
        Some("hash") => with_reference(&args[1..], hash),
        Some("run") => with_reference(&args, run),
        _ => run(&args, &LoadOptions::default()),
    };
    match result {
        Ok(()) => ExitCode::SUCCESS,
        Err(message) => {
            eprintln!("error: {message}");
            ExitCode::FAILURE
        }
    }
}

/// Takes `--reference` out of `args` and calls `command` with the rest.
fn with_reference(
    args: &[String],
    command: impl FnOnce(&[String], &LoadOptions) -> Result<(), String>,
) -> Result<(), String> {
    let (rest, options) = take_reference(args)?;
    command(&rest, &options)
}

/// `run`: `args` are all the words, the first being `run` (or, with no
/// command, a request for help or an error).
fn run(args: &[String], options: &LoadOptions) -> Result<(), String> {
    let mut scenario_path: Option<&str> = None;
    let mut show_events = false;
    let mut seed: Option<u64> = None;
    let mut save_path: Option<&str> = None;

    let mut words = args.iter().map(String::as_str);
    match words.next() {
        Some("run") => {}
        Some("--help" | "-h") | None => {
            println!("{USAGE}");
            return Ok(());
        }
        Some(other) => return Err(format!("unknown command \"{other}\"\n\n{USAGE}")),
    }
    while let Some(word) = words.next() {
        match word {
            "--events" => show_events = true,
            "--seed" => seed = Some(parse_seed(words.next())?),
            "--save" => {
                save_path = Some(
                    words
                        .next()
                        .ok_or_else(|| format!("--save needs a file\n\n{USAGE}"))?,
                )
            }
            flag if flag.starts_with('-') => {
                return Err(format!("unknown option \"{flag}\"\n\n{USAGE}"))
            }
            path if scenario_path.is_none() => scenario_path = Some(path),
            extra => return Err(format!("unexpected argument \"{extra}\"\n\n{USAGE}")),
        }
    }
    let scenario_path = scenario_path.ok_or_else(|| format!("no scenario given\n\n{USAGE}"))?;

    let mut remembered: Option<Result<save::Remembered, String>> = None;
    let Executed {
        scenario,
        summaries,
        staff_handed_in,
        initial: _,
    } = execute(scenario_path, seed, options, |scenario| {
        if save_path.is_some() {
            remembered = Some(save::remember(scenario));
        }
        println!("{}", scenario.name);
        if scenario.placeholder {
            println!("PLACEHOLDER DATA: some or all of the units, figures and geography are invented, not historical.");
        }
        println!("seed {}", scenario.world.seed());
        for line in held_apart_lines(scenario.world.umpire()) {
            println!("{line}");
        }
        println!();
    })?;

    for summary in &summaries {
        println!(
            "day {:>3}  hash {:016x}  {} events",
            summary.day, summary.state_hash, summary.event_count
        );
    }

    if let (Some(file), Some(remembered)) = (save_path, remembered) {
        let remembered = remembered?;
        let hashes: Vec<(u32, u64)> = summaries.iter().map(|s| (s.day, s.state_hash)).collect();
        save::write(
            file,
            scenario_path,
            scenario.world.seed(),
            &remembered,
            remembered.script(),
            remembered.script_inside_day(),
            remembered.commands(),
            &staff_handed_in,
            &save::script_seats(),
            &hashes,
            None,
        )?;
    }

    let world = &scenario.world;
    if show_events {
        println!();
        for event in world.events().iter() {
            println!("{}", event_line(world, event));
        }
    }

    // The runner is the umpire: it is allowed to print the truth.
    let truth = world.umpire();
    println!();
    for task_force in truth.task_forces.values() {
        let position = match task_force.toward_hex {
            Some(toward) => format!(
                "{}, {} nm from {} toward {}",
                task_force.position(),
                task_force.progress_nm,
                task_force.from_hex,
                toward
            ),
            None => task_force.position().to_string(),
        };
        // "Alert" is what the rules read. A force is alert by its readiness
        // or because it is alerted (decision 0021). Saying which avoids a
        // line that reads "battle readiness, not alerted".
        let readiness = format!(
            "readiness {}, {}",
            task_force.readiness.name(),
            match (task_force.is_alert(), task_force.alerted) {
                (true, true) if task_force.readiness.name() == "normal" => "alert (alerted)",
                (true, _) => "alert",
                (false, _) => "not alert",
            }
        );
        // Fatigue is shown only when there is some (decision 0037).
        let readiness = if task_force.fatigue != Fixed::ZERO {
            format!("{readiness}, fatigue {}", task_force.fatigue)
        } else {
            readiness
        };
        let stance = match task_force.rules_of_engagement.stance {
            Stance::Engage => "engage",
            Stance::Avoid => "avoid",
        };
        println!(
            "{} \"{}\" ({}): {:?} at {}; {}, rules of engagement: {}, break off at {}, stragglers: {}, keep {}; {}",
            task_force.id,
            task_force.name,
            task_force.side.name(),
            task_force.status,
            position,
            readiness,
            stance,
            task_force.rules_of_engagement.break_off,
            task_force.rules_of_engagement.stragglers.name(),
            task_force.rules_of_engagement.keep_share,
            match task_force.home_port {
                Some(port) => format!("home port {port}"),
                None => "no home port".to_string(),
            }
        );
    }
    for group in truth.air_groups.values() {
        let arc = match group.search {
            Some(arc) => format!(
                "arc {:03}-{:03} to {} nm",
                arc.from_deg, arc.to_deg, arc.range_nm
            ),
            None => "no arc".to_string(),
        };
        let share = if group.mission == AirMission::CombatAirPatrol {
            format!(", patrol share {}", group.patrol_share)
        } else {
            String::new()
        };
        let mission = match group.mission {
            AirMission::BaseStrike(base) => format!("BaseStrike on {base}"),
            AirMission::Reconnaissance(hex) => format!("Reconnaissance of {hex}"),
            other => format!("{other:?}"),
        };
        // Damaged aircraft are shown only when there are some (decision 0035).
        let ready = if group.damaged > 0 {
            format!("{} ready, {} damaged", group.ready, group.damaged)
        } else {
            format!("{} ready", group.ready)
        };
        // Fatigue, the rest threshold and a stand-down are shown only when
        // there is something to show (decision 0036).
        let mut tired = String::new();
        if group.fatigue != Fixed::ZERO {
            tired.push_str(&format!(", fatigue {}", group.fatigue));
        }
        if let Some(level) = group.rest_above {
            tired.push_str(&format!(", rests above {level}"));
        }
        if group.stood_down {
            tired.push_str(", stood down");
        }
        // Reach is shown only when extended (decision 0059).
        if group.reach == sim_core::orders::Reach::Extended {
            tired.push_str(", extended reach");
        }
        // Pilots are shown only when the scenario tracks them (decision 0038).
        let ready = match group.pilots {
            Some(pilots) => format!("{ready}, {pilots} pilots"),
            None => ready,
        };
        println!(
            "{} \"{}\" ({}) {}: {}, mission {}{}{}, {}",
            group.id,
            group.name,
            group.side.name(),
            match group.location {
                AirGroupLocation::Base(base) => format!("at {base}"),
                AirGroupLocation::Ship(ship) => format!("aboard {ship}"),
                AirGroupLocation::Lost => "lost with its base".to_string(),
            },
            ready,
            mission,
            share,
            tired,
            arc
        );
    }
    for ship in truth.ships.values() {
        let location = match ship.location {
            ShipLocation::TaskForce(id) => id.to_string(),
            ShipLocation::Port(id) => id.to_string(),
            ShipLocation::Sunk => "no task force or port".to_string(),
            ShipLocation::Yard(id) => format!("{id}, away at the yard"),
        };
        let state = if ship.location == ShipLocation::Sunk {
            "SUNK".to_string()
        } else {
            format!("afloat in {location}")
        };
        let class = &world.ref_data().ship_classes[&ship.class];
        let damaged: Vec<String> = ShipSystem::ALL
            .into_iter()
            .filter(|system| class.has_system(*system))
            .filter(|system| ship.systems.get(*system) != SystemState::Working)
            .map(|system| format!("{} {}", system.name(), ship.systems.get(system).name()))
            .collect();
        let systems = if damaged.is_empty() {
            "all systems working".to_string()
        } else {
            damaged.join(", ")
        };
        let cargo = {
            let fuel = if ship.cargo_fuel_t > Fixed::ZERO {
                format!(", {} t of fuel cargo", ship.cargo_fuel_t)
            } else {
                String::new()
            };
            // The men aboard, only when there are some (decision 0066).
            let men = ship.troops.map_or(String::new(), |troops| {
                format!(", {} men of {} aboard", troops.men, troops.unit)
            });
            format!("{fuel}{men}")
        };
        println!(
            "{} \"{}\": {state}; damage flotation {}, fire {}; {systems}; {} gun rounds, torpedoes {}; {} t of fuel, {} t of supply{}",
            ship.id,
            ship.name,
            ship.flotation_damage,
            ship.fire_damage,
            ship.gun_rounds,
            if ship.torpedoes_ready { "ready" } else { "gone" },
            ship.fuel_t,
            ship.supply_t,
            cargo
        );
    }
    for base in truth.bases.values() {
        let guns = if base.anti_aircraft > 0 {
            format!(", {} anti-aircraft shots", base.anti_aircraft)
        } else {
            String::new()
        };
        let repairs = if base.aircraft_repairs_per_day > 0 {
            format!(", repairs {} aircraft a day", base.aircraft_repairs_per_day)
        } else {
            String::new()
        };
        // The airfield, the work done on it and the defences are said only
        // where they differ from a base as it always was (decision 0099).
        let works = {
            let mut parts = Vec::new();
            if !base.airfield {
                parts.push("no airfield".to_string());
            }
            if base.airfield_work > Fixed::ZERO {
                parts.push(format!(
                    "{} of work done on the airfield",
                    base.airfield_work
                ));
            }
            if base.defences > Fixed::ZERO {
                parts.push(format!("defences {}", base.defences));
            }
            if parts.is_empty() {
                String::new()
            } else {
                format!(", {}", parts.join(", "))
            }
        };
        // Fuel is shown only for a base that has ever been given some. A
        // stock only falls, so that is a stock above nothing now or one that
        // refuelled a ship (decision 0045).
        let had_fuel = base.fuel_t > Fixed::ZERO
            || world.events().iter().any(|event| {
                matches!(&event.kind, EventKind::ShipRefuelled { base: b, .. } if *b == base.id)
            });
        let fuel = if had_fuel {
            format!(", {} t of fuel in stock", base.fuel_t)
        } else {
            String::new()
        };
        // What the base keeps in reserve, only where it keeps some
        // (decision 0111, 6).
        let reserve = {
            let mut parts = Vec::new();
            if base.supply_reserve_t > Fixed::ZERO {
                parts.push(format!("{} t of supply", base.supply_reserve_t));
            }
            if base.fuel_reserve_t > Fixed::ZERO {
                parts.push(format!("{} t of fuel", base.fuel_reserve_t));
            }
            if parts.is_empty() {
                String::new()
            } else {
                format!(", reserve {}", parts.join(" and "))
            }
        };
        // A base a landing formed is marked (decision 0066, 12).
        let formed = if base.on_map {
            ""
        } else {
            ", formed in play, on its own side's map only"
        };
        println!(
            "{} \"{}\" ({}) at {}: {} t of supply in stock{}{}{}{}{}{}",
            base.id,
            base.name,
            base.side.name(),
            base.hex,
            base.supply_t,
            fuel,
            reserve,
            guns,
            repairs,
            works,
            formed
        );
    }
    // Land units are listed only where a scenario has any (decision 0066).
    for unit in truth.land_units.values() {
        let ashore = match unit.hex {
            Some(hex) => format!("{} men ashore at {hex}", unit.men_ashore),
            None => "no men ashore".to_string(),
        };
        // Posture and fatigue only where they are not the start's
        // (decision 0090): a defending, rested unit's line is as it was.
        let mut state = String::new();
        if unit.posture != Posture::Defend {
            state += &format!(", on {}", unit.posture.name());
        }
        if unit.fatigue > Fixed::ZERO {
            state += &format!(", fatigue {}", unit.fatigue);
        }
        // Fieldworks, where the unit has dug any and has men ashore in them
        // (decision 0106).
        if unit.fieldworks > Fixed::ZERO && unit.men_ashore > 0 {
            state += &format!(", fieldworks {}", unit.fieldworks);
        }
        println!(
            "{} \"{}\" ({}): {ashore}{state}",
            unit.id,
            unit.name,
            unit.side.name()
        );
    }
    // Submarines are listed only where a scenario has any (decision 0218):
    // the umpire's truth, as the land units' lines are.
    for boat in truth.submarines.values() {
        let place = match (&boat.passage, boat.enters_day) {
            (_, Some(day)) if day > truth.day => format!("held apart until day {day}"),
            (Some(passage), _) if passage.home => format!("on passage home to {}", passage.to),
            (Some(passage), _) => format!("on passage to {}", passage.to),
            (None, _) => format!("on station at {}", boat.station),
        };
        println!(
            "{} \"{}\" ({}): {}, at {}, {place}, {} salvoes",
            boat.id,
            boat.name,
            boat.side.name(),
            boat.condition.name(),
            boat.hex,
            boat.salvoes
        );
    }
    for side in [Side::Allied, Side::Japanese] {
        let view = world.side_view(side);
        let mut any = false;
        for contact in view.contacts() {
            any = true;
            println!(
                "{} {} at {}: {} ships, {}, age {} pulses",
                side.name(),
                format_args!("contact:{}", contact.id.umpire_raw(&world.umpire_key())),
                contact.estimated_hex,
                contact.estimated_ship_count,
                contact.estimated_kind.name(),
                contact.age_pulses
            );
        }
        if !any {
            println!("{}: no contacts", side.name());
        }
        for claim in view.claims() {
            println!(
                "{}, day {} {}",
                claim.text(side),
                claim.day,
                claim.pulse.name()
            );
        }
        if view.claims().is_empty() {
            println!("{}: no battle claims", side.name());
        }
        // What the side believes of the enemy's men ashore (decision 0092):
        // printed only when it holds an estimate.
        for estimate in view.ground_estimates() {
            println!(
                "{} estimate at {}: {} enemy men ashore, reported day {} ({})",
                side.name(),
                estimate.hex,
                estimate.men,
                estimate.day,
                estimate.grade.name()
            );
        }
    }
    print_result(world);
    Ok(())
}

/// One event as `run --events` prints it. An order the simulation issued
/// itself is marked, so the log shows who gave it (decision 0016, 14).
fn event_line(world: &sim_core::World, event: &sim_core::events::Event) -> String {
    let issuer = match &event.kind {
        EventKind::OrderAccepted { order, .. } | EventKind::OrderRejected { order, .. } => {
            world.order(*order).map(|o| o.origin.issuer)
        }
        _ => None,
    };
    match issuer {
        Some(sim_core::orders::Issuer::Staff) => format!("{event}  [issued by the staff]"),
        // A rule acting inside the day (decision 0055).
        Some(sim_core::orders::Issuer::LocalCommander) => {
            format!("{event}  [issued by the local commander]")
        }
        Some(sim_core::orders::Issuer::Player) | None => format!("{event}"),
    }
}

/// The umpire's result (decision 0054), for a scenario that has objectives:
/// each side's points by objective, its margin and level, and the note of
/// how the day went in history. Nothing for a scenario without them, so its
/// output is what it was.
fn print_result(world: &sim_core::World) {
    let Some(result) = world.result() else {
        return;
    };
    println!();
    // A scenario for play has a scale of its own (decision 0187, 22).
    if world.objectives().is_some_and(|o| o.for_play) {
        println!("Result (the scenario's own scale, for play):");
    } else {
        println!("Result (the placeholder scale of decision 0054):");
    }
    for side in [Side::Allied, Side::Japanese] {
        let r = result.side(side);
        let parts: Vec<String> = r
            .objectives
            .iter()
            .map(|o| {
                let target = o.base.map_or(String::new(), |b| format!(" at {b}"));
                // A base's count is days out of the days counted (decision
                // 0067).
                let of_days = o.days.map(|d| format!(" of {d} days")).unwrap_or_default();
                // Ships out of action are over the horizon (decision 0088).
                let of_days = o
                    .horizon_days
                    .map(|d| format!(" over {d} days"))
                    .unwrap_or(of_days);
                format!("{}{target} {}{of_days} = {}", o.kind, o.count, o.points)
            })
            .collect();
        println!(
            "{}: {} points ({}), margin {}, {}",
            side.name(),
            r.total,
            parts.join("; "),
            r.margin,
            r.outcome
        );
        // One line a ship, so that the sum can be checked by eye.
        for o in &r.objectives {
            for out in &o.ships_out {
                println!(
                    "  ship {} {}: {} days at the end + {} left with = {} days out, share {}, points {}",
                    out.ship.raw(),
                    out.kind.name(),
                    out.at_end,
                    out.left_with,
                    out.days_out,
                    out.share,
                    out.points
                );
            }
        }
    }
    if let Some(history) = &result.history {
        println!("History: {history}");
    }
}

/// `sim-cli hash <scenario> [--seed N]`: the daily hashes and nothing else.
fn hash(args: &[String], options: &LoadOptions) -> Result<(), String> {
    let mut scenario_path: Option<&str> = None;
    let mut seed: Option<u64> = None;
    let mut words = args.iter().map(String::as_str);
    while let Some(word) = words.next() {
        match word {
            "--seed" => seed = Some(parse_seed(words.next())?),
            flag if flag.starts_with('-') => {
                return Err(format!("unknown option \"{flag}\"\n\n{USAGE}"))
            }
            path if scenario_path.is_none() => scenario_path = Some(path),
            extra => return Err(format!("unexpected argument \"{extra}\"\n\n{USAGE}")),
        }
    }
    let scenario_path = scenario_path.ok_or_else(|| format!("no scenario given\n\n{USAGE}"))?;
    let executed = execute(scenario_path, seed, options, |_| {})?;
    for summary in &executed.summaries {
        println!("day {} {:016x}", summary.day, summary.state_hash);
    }
    Ok(())
}

/// `sim-cli replay <game.json> [--check-staff]`: runs a saved game and
/// compares every day's hash with the saved one. With `--check-staff` it also
/// asks the doctrine's staff on every day, and stops at the first day and side
/// where it gives something other than what the game holds.
fn replay(args: &[String], options: &LoadOptions) -> Result<(), String> {
    let mut file: Option<&str> = None;
    let mut check_staff = false;
    for word in args.iter().map(String::as_str) {
        match word {
            "--check-staff" => check_staff = true,
            flag if flag.starts_with('-') => {
                return Err(format!("unknown option \"{flag}\"\n\n{USAGE}"))
            }
            path if file.is_none() => file = Some(path),
            _ => return Err(format!("replay needs one saved game\n\n{USAGE}")),
        }
    }
    let file = file.ok_or_else(|| format!("replay needs one saved game\n\n{USAGE}"))?;
    let (_, saved, finding) = execute_saved_checking(
        file,
        options,
        false,
        check_staff,
        |scenario, saved| {
            println!("{}", scenario.name);
            println!("seed {}", saved.seed);
            // A game that had a player says so; the orders it handed in are
            // replayed and nobody is asked for any.
            if saved
                .seats
                .values()
                .any(|k| *k == sim_agent::SeatKind::Player)
            {
                println!(
                    "seats allied {}, japanese {} (replayed with no player)",
                    saved.seats[&Side::Allied].name(),
                    saved.seats[&Side::Japanese].name()
                );
            }
        },
        |summary| println!("day {} {:016x}", summary.day, summary.state_hash),
        &mut |_, _| {},
    )?;
    if let Some(finding) = finding {
        return Err(format!(
            "the staff differs on day {}, {} side: {}",
            finding.day,
            finding.side.name().to_lowercase(),
            finding.what
        ));
    }
    if check_staff {
        println!(
            "the staff gave what was saved on every day (doctrine {}, the game was saved under {})",
            sim_staff::DOCTRINE_VERSION,
            saved.doctrine_version
        );
    }
    Ok(())
}

/// One line for each unit the scenario holds apart until its day (decision
/// 0195), in the state's order: the unit, its side, its day, its place and
/// what comes with it. Empty for a scenario that holds nothing apart, so its
/// output is what it was. The runner is the umpire: it prints both sides'.
fn held_apart_lines(truth: &sim_core::state::TruthState) -> Vec<String> {
    use sim_core::entering::EntryPlace;
    let plural =
        |n: usize, one: &str, many: &str| format!("{n} {}", if n == 1 { one } else { many });
    truth
        .entering
        .iter()
        .map(|(unit, entry)| {
            let name = if let Some(force) = &entry.task_force {
                force.name.clone()
            } else if let Some(land) = entry.land_units.values().next() {
                land.name.clone()
            } else {
                entry
                    .air_groups
                    .values()
                    .next()
                    .map_or_else(String::new, |group| group.name.clone())
            };
            let place = match entry.place {
                EntryPlace::Hex(hex) => format!("at {hex}"),
                EntryPlace::Base(base) => format!("at {base}"),
            };
            let mut with = Vec::new();
            if entry.task_force.is_some() {
                with.push(plural(entry.ships.len(), "ship", "ships"));
            }
            if entry.task_force.is_some() {
                with.push(plural(entry.air_groups.len(), "air group", "air groups"));
            }
            if entry.task_force.is_some() {
                with.push(plural(entry.land_units.len(), "land unit", "land units"));
            }
            let with = if with.is_empty() {
                String::new()
            } else {
                format!(": {}", with.join(", "))
            };
            format!(
                "held apart: {unit} \"{name}\", {}, enters on day {} {place}{with}",
                entry.side.name().to_lowercase(),
                entry.day,
            )
        })
        .collect()
}
