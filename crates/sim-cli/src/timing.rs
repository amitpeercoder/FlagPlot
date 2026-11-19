//! `sim-cli time`: what a day costs, measured outside the game (decision 0100,
//! 11 to 13).
//!
//! It runs fixed workloads on copies of a scenario's world and reports how
//! long each took, as a spread over repeated runs on a named machine. The
//! clock is read only around work and never fed into it: the single function
//! that reads it is [`clock`], and every result of the game is the same with
//! or without the command. It changes nothing in a scenario, a save or a hash.
//!
//! The workloads, each on the world at the day before the one resolved:
//! one day resolved, early and late; the same day with every question of
//! every unit of both sides asked at every moment of the day, and the asking
//! alone once before the day; a copy of the world with and without its event
//! log; and a number of games of some days from one start, from the start of
//! the soak and from day 150. Two more measure the staff (decision 0107,
//! "What it must pass", Speed): the staff's decisions for one side on each
//! day of the slice played with the staffs, and a quiet week (seven staffed
//! days) in one call. Last, a trial (decision 0175, 9): the Allied side's
//! "control Guadalcanal" on the 39 days, from the evening before day 1 and
//! from the evening of day 20, at each of the three sizes, with both
//! staffs. No number fails them.

use std::hint::black_box;
use std::time::{Duration, Instant};

use sim_core::World;
use sim_data::{LoadOptions, Scenario};

const DEFAULT_SLICE: &str = "data/scenarios/07_full_slice.toml";
const DEFAULT_SOAK: &str = "data/scenarios/08_soak.toml";
/// The 39 days with the intents as stated: stage 3's proof's arrangement
/// (a), whose first Allied intent is "control Guadalcanal" (decision 0153).
const DEFAULT_TRIAL_SLICE: &str = "data/scenarios/stage-3-proof/07a_slice.toml";
/// The trial's figures where the slice names none (decision 0174).
const DEFAULT_TRIAL_FIGURES: &str = "data/reference/trial.toml";

/// The one place the clock is read (decision 0100, 11). The workspace lint
/// forbids `Instant::now` everywhere else. What it returns is only ever
/// subtracted from a later reading to print a duration; no result of the game
/// is made from it.
#[allow(clippy::disallowed_methods)]
fn clock() -> Instant {
    Instant::now()
}

/// How long `work` took.
fn timed<T>(work: impl FnOnce() -> T) -> Duration {
    let start = clock();
    black_box(work());
    start.elapsed()
}

struct Settings {
    machine: String,
    repeats: u32,
    games: u32,
    days: u32,
    slice: String,
    soak: String,
    early: u32,
    late: u32,
    soak_late: u32,
    trial_slice: String,
    trial_figures: String,
    trial_late: u32,
    only_trial: bool,
    trial_size: Option<String>,
}

fn parse(args: &[String], usage: &str) -> Result<Settings, String> {
    let mut settings = Settings {
        machine: String::new(),
        repeats: 20,
        games: 100,
        days: 30,
        slice: DEFAULT_SLICE.to_string(),
        soak: DEFAULT_SOAK.to_string(),
        early: 1,
        late: 12,
        soak_late: 150,
        trial_slice: DEFAULT_TRIAL_SLICE.to_string(),
        trial_figures: DEFAULT_TRIAL_FIGURES.to_string(),
        trial_late: 20,
        only_trial: false,
        trial_size: None,
    };
    let mut words = args.iter();
    while let Some(word) = words.next() {
        let mut value = |name: &str| -> Result<&String, String> {
            words
                .next()
                .ok_or_else(|| format!("{name} needs a value\n\n{usage}"))
        };
        let number = |name: &str, text: &str| -> Result<u32, String> {
            text.parse::<u32>()
                .map_err(|_| format!("{name} \"{text}\" is not a whole number"))
        };
        match word.as_str() {
            "--machine" => settings.machine = value("--machine")?.clone(),
            "--repeats" => settings.repeats = number("--repeats", value("--repeats")?)?,
            "--games" => settings.games = number("--games", value("--games")?)?,
            "--days" => settings.days = number("--days", value("--days")?)?,
            "--slice" => settings.slice = value("--slice")?.clone(),
            "--soak" => settings.soak = value("--soak")?.clone(),
            "--early-day" => settings.early = number("--early-day", value("--early-day")?)?,
            "--late-day" => settings.late = number("--late-day", value("--late-day")?)?,
            "--soak-late-day" => {
                settings.soak_late = number("--soak-late-day", value("--soak-late-day")?)?
            }
            "--trial-slice" => settings.trial_slice = value("--trial-slice")?.clone(),
            "--trial-figures" => settings.trial_figures = value("--trial-figures")?.clone(),
            "--trial-late-day" => {
                settings.trial_late = number("--trial-late-day", value("--trial-late-day")?)?
            }
            "--only-trial" => settings.only_trial = true,
            "--trial-size" => {
                let name = value("--trial-size")?.clone();
                if !["small", "standard", "full"].contains(&name.as_str()) {
                    return Err(format!(
                        "--trial-size \"{name}\" is not small, standard or full"
                    ));
                }
                settings.trial_size = Some(name);
            }
            other => return Err(format!("unknown argument \"{other}\"\n\n{usage}")),
        }
    }
    if settings.machine.trim().is_empty() {
        return Err(format!("--machine TEXT is required\n\n{usage}"));
    }
    if settings.repeats == 0 || settings.games == 0 {
        return Err("--repeats and --games must be at least 1".to_string());
    }
    Ok(settings)
}

/// A scenario run to the end of `day` with its own orders, and the orders of
/// the days after it, still to be given.
struct Start {
    world: World,
    script: std::collections::BTreeMap<
        u32,
        Vec<(sim_core::state::Side, sim_core::orders::OrderRequest)>,
    >,
    /// The commander's commands of the days after it (decision 0107).
    commands: crate::save::Commands,
    /// The staff each side is given, from the doctrine.
    staffs:
        std::collections::BTreeMap<sim_core::state::Side, Box<dyn sim_core::command::Staff + Send>>,
    days: u32,
}

fn start_at(path: &str, day: u32, options: &LoadOptions) -> Result<Start, String> {
    let mut scenario =
        Scenario::load_with(std::path::Path::new(path), options).map_err(|e| e.to_string())?;
    if day >= scenario.days {
        return Err(format!(
            "{path} has {} days, so a start at day {day} leaves none to resolve",
            scenario.days
        ));
    }
    let staffs = sim_staff::staffs();
    while scenario.world.day() < day {
        scenario
            .next_day_staffed(sim_staff::as_staffs(&staffs))
            .map_err(|e| e.to_string())?
            .ok_or_else(|| format!("{path} ended before day {day}"))?;
    }
    Ok(Start {
        script: scenario.scripted_orders().clone(),
        commands: scenario.scripted_commands().clone(),
        staffs,
        days: scenario.days,
        world: scenario.world,
    })
}

impl Start {
    /// Hands in the scripted commands and orders of `day` to `world`.
    fn submit(&self, world: &mut World, day: u32) {
        for (side, command) in self.commands.get(&day).cloned().unwrap_or_default() {
            world.submit_commands(side, vec![command]);
        }
        for (side, request) in self.script.get(&day).cloned().unwrap_or_default() {
            world.submit_orders(side, vec![request]);
        }
    }

    /// A copy of the start with the orders of its next day in.
    fn ready(&self) -> World {
        let mut world = self.world.clone();
        let next = world.day() + 1;
        self.submit(&mut world, next);
        world
    }

    /// Plays `days` days (or to the scenario's end) on `world`.
    fn play(&self, world: &mut World, days: u32) -> Result<u32, String> {
        let mut played = 0;
        for _ in 0..days {
            let day = world.day() + 1;
            if day > self.days {
                break;
            }
            self.submit(world, day);
            world
                .resolve_day_staffed(sim_staff::as_staffs(&self.staffs))
                .map_err(|e| e.to_string())?;
            played += 1;
        }
        Ok(played)
    }
}

/// Every question of both sides, asked and answered; how many there were.
fn ask_everything(world: &World) -> usize {
    let mut count = 0;
    for side in sim_core::state::Side::ALL {
        let view = world.side_view(side);
        let questions = view.questions();
        count += questions.len();
        for question in &questions {
            black_box(view.ask(question));
        }
    }
    count
}

/// The minimum, median and maximum of some durations.
struct Spread {
    min: Duration,
    median: Duration,
    max: Duration,
    runs: usize,
}

fn spread(mut times: Vec<Duration>) -> Spread {
    times.sort();
    Spread {
        min: times[0],
        median: times[times.len() / 2],
        max: times[times.len() - 1],
        runs: times.len(),
    }
}

fn show(d: Duration) -> String {
    let micros = d.as_micros();
    if micros >= 10_000 {
        format!("{}.{:01} ms", micros / 1000, (micros % 1000) / 100)
    } else if micros >= 1000 {
        format!("{}.{:02} ms", micros / 1000, (micros % 1000) / 10)
    } else {
        format!("{micros} us")
    }
}

fn line(label: &str, s: &Spread) {
    println!(
        "  {label:<46} min {:>10}  median {:>10}  max {:>10}  ({} runs)",
        show(s.min),
        show(s.median),
        show(s.max),
        s.runs
    );
}

fn repeat(n: u32, mut one: impl FnMut() -> Duration) -> Spread {
    spread((0..n).map(|_| one()).collect())
}

/// A day resolved from `start`, timing the resolve only.
fn resolve_one_day(start: &Start, repeats: u32) -> Spread {
    repeat(repeats, || {
        let mut world = start.ready();
        timed(|| {
            world
                .resolve_day_staffed(sim_staff::as_staffs(&start.staffs))
                .map(|s| s.state_hash)
        })
    })
}

/// The same day with every question asked at every moment of it. Returns the
/// spread and the number of questions asked in all.
fn resolve_asking(start: &Start, repeats: u32) -> (Spread, usize, usize) {
    let mut asked = 0;
    let mut moments = 0;
    let s = repeat(repeats, || {
        let mut world = start.ready();
        let mut count = 0;
        let mut seen = 0;
        let t = timed(|| {
            world.resolve_day_watched(sim_staff::as_staffs(&start.staffs), &mut |w, _moment| {
                seen += 1;
                count += ask_everything(w);
            })
        });
        asked = count;
        moments = seen;
        t
    });
    (s, asked, moments)
}

/// The asking alone, once, before the day.
fn ask_once(start: &Start, repeats: u32) -> (Spread, usize) {
    let mut asked = 0;
    let s = repeat(repeats, || {
        let world = start.ready();
        timed(|| {
            asked = ask_everything(&world);
        })
    });
    (s, asked)
}

/// A day's staff decisions for one side, for each day of the scenario as it
/// is played with the staffs: `Staff::decide` on the side's view at the
/// staff's step. The view (a copy of the world taken to the staff's step) is
/// made before the clock starts. Each day's time is the median of `repeats`
/// askings; the spread is over the days. Returns, for each side, that spread,
/// the days counted, and the day of the longest time with that time.
struct StaffDays {
    side: sim_core::state::Side,
    spread: Spread,
    counted: u32,
    longest_day: u32,
    longest: Duration,
}

fn staff_days(start: &Start, repeats: u32) -> Result<Vec<StaffDays>, String> {
    let mut per_side: std::collections::BTreeMap<_, Vec<(u32, Duration)>> =
        std::collections::BTreeMap::new();
    let mut world = start.world.clone();
    while world.day() < start.days {
        let day = world.day() + 1;
        let mut ready = world.clone();
        start.submit(&mut ready, day);
        let staged = ready.at_staff_step();
        for side in sim_core::state::Side::ALL {
            let Some(staff) = start.staffs.get(&side) else {
                continue;
            };
            let median = repeat(repeats, || {
                let view = staged.side_view(side);
                timed(|| staff.decide(view))
            })
            .median;
            per_side.entry(side).or_default().push((day, median));
        }
        start.play(&mut world, 1)?;
    }
    Ok(per_side
        .into_iter()
        .map(|(side, days)| {
            let longest = days
                .iter()
                .copied()
                .max_by_key(|(_, time)| *time)
                .unwrap_or((0, Duration::ZERO));
            let counted = u32::try_from(days.len()).unwrap_or(u32::MAX);
            StaffDays {
                side,
                spread: spread(days.into_iter().map(|(_, time)| time).collect()),
                counted,
                longest_day: longest.0,
                longest: longest.1,
            }
        })
        .collect())
}

/// A quiet week in one call: seven days resolved in a row with the staffs, as
/// `run {days: 7}` does, from `start`. The copy is made before the clock
/// starts. Returns the spread and the days played (the scenario's end may cut
/// the week short).
fn quiet_week(start: &Start, repeats: u32) -> Result<(Spread, u32), String> {
    let mut played = 0;
    let mut times = Vec::new();
    for _ in 0..repeats {
        let mut world = start.world.clone();
        let begin = clock();
        played = start.play(&mut world, 7)?;
        black_box(&world);
        times.push(begin.elapsed());
    }
    Ok((spread(times), played))
}

fn copies(start: &Start, repeats: u32) -> (Spread, Spread, Spread) {
    let whole = repeat(repeats, || timed(|| start.world.clone()));
    let log = repeat(repeats, || timed(|| start.world.events().clone()));
    // Without its log: the difference of the two medians' runs, run by run.
    let mut without = Vec::new();
    for _ in 0..repeats {
        let a = timed(|| start.world.clone());
        let b = timed(|| start.world.events().clone());
        without.push(a.saturating_sub(b));
    }
    (whole, log, spread(without))
}

fn games(start: &Start, games: u32, days: u32) -> Result<(Spread, Duration, u32), String> {
    let mut times = Vec::new();
    let mut played = 0;
    for _ in 0..games {
        let begin = clock();
        let mut world = start.world.clone();
        played = start.play(&mut world, days)?;
        black_box(&world);
        drop(world);
        times.push(begin.elapsed());
    }
    let total: Duration = times.iter().sum();
    Ok((spread(times), total, played))
}

/// The course of the measure from the evening before day 1 (decision 0175,
/// amended point 13): the Allied side's own commands of day 1 in the
/// slice, each on the world's first day, in the order given.
fn trial_course(scenario: &Scenario) -> Result<Vec<sim_trial::CourseCommand>, String> {
    let course: Vec<sim_trial::CourseCommand> = scenario
        .scripted_commands()
        .get(&1)
        .cloned()
        .unwrap_or_default()
        .into_iter()
        .filter(|(side, _)| *side == sim_core::state::Side::Allied)
        .map(|(_, command)| sim_trial::CourseCommand::first_day(command))
        .collect();
    if course.is_empty() {
        return Err("the trial's slice gives the Allied side no command on day 1".to_string());
    }
    Ok(course)
}

/// The slice played with its own commands and both staffs to the evening of
/// `day`: the live game a trial is asked on.
fn trial_live_game(path: &str, day: u32, options: &LoadOptions) -> Result<Scenario, String> {
    let mut scenario =
        Scenario::load_with(std::path::Path::new(path), options).map_err(|e| e.to_string())?;
    let staffs = sim_staff::staffs();
    while scenario.world.day() < day {
        scenario
            .next_day_staffed(sim_staff::as_staffs(&staffs))
            .map_err(|e| e.to_string())?
            .ok_or_else(|| format!("{path} ended before day {day}"))?;
    }
    Ok(scenario)
}

/// What one trial of the measure took and gave.
struct TrialTime {
    size: &'static str,
    worlds: u32,
    days: u32,
    from: u32,
    time: Duration,
    played_days: usize,
    failed: Vec<String>,
}

/// One trial of the Allied side on `live`, timed from the making of its
/// pictures to the dropping of its worlds, with both staffs (decision 0175,
/// 9).
fn time_trial(
    live: &Scenario,
    figures: &sim_core::refdata::TrialFigures,
    course: &[sim_trial::CourseCommand],
    size: (&'static str, sim_core::refdata::TrialSize),
) -> Result<TrialTime, String> {
    let side = sim_core::state::Side::Allied;
    let believed = sim_trial::Believed {
        forces: live.believed_forces(side).cloned().collect(),
        intents: live.believed_intents(side).cloned().collect(),
    };
    let asked = sim_trial::Asked {
        trial: 1,
        worlds: size.1.worlds,
        days: size.1.days,
        course: course.to_vec(),
    };
    let view = live.world.side_view(side);
    let begin = clock();
    let played = sim_trial::play(
        &view,
        &believed,
        figures,
        live.world.ref_data(),
        live.world.map(),
        asked,
    )
    .map_err(|e| e.to_string())?;
    let played_days = played.worlds.iter().map(|w| w.days.len()).sum();
    let failed = played
        .failed()
        .iter()
        .map(|f| format!("world {} day {}: {}", f.world, f.day, f.words))
        .collect();
    drop(played);
    let time = begin.elapsed();
    Ok(TrialTime {
        size: size.0,
        worlds: size.1.worlds,
        days: size.1.days,
        from: live.world.day(),
        time,
        played_days,
        failed,
    })
}

/// The measure of decision 0175, 9: a trial from the evening before day 1
/// and from the evening of `late`, at each size, once each.
fn trials(s: &Settings, options: &LoadOptions) -> Result<Vec<TrialTime>, String> {
    let early = trial_live_game(&s.trial_slice, 0, options)?;
    let course = trial_course(&early)?;
    let figures = match early.trial_figures() {
        Some(figures) => figures.clone(),
        None => sim_data::load_trial_figures(std::path::Path::new(&s.trial_figures))
            .map_err(|e| e.to_string())?,
    };
    let late = trial_live_game(&s.trial_slice, s.trial_late, options)?;
    println!();
    println!(
        "h. A trial of the Allied side on {} (decision 0175, 9): from the evening before day 1 with its own {} command(s) of day 1 as the course, and from the evening of day {} with an empty course; the trial's seed {}, trial 1; both staffs; once each, timed from the making of the pictures to the dropping of the worlds:",
        s.trial_slice,
        course.len(),
        s.trial_late,
        figures.seed
    );
    let mut times = Vec::new();
    for (live, course) in [(&early, course.as_slice()), (&late, &[][..])] {
        let before = live.world.state_hash();
        for size in figures.sizes() {
            if s.trial_size.as_deref().is_some_and(|name| name != size.0) {
                continue;
            }
            let t = time_trial(live, &figures, course, size)?;
            println!(
                "  {:<8} {:>2} worlds x {:>2} days ({:>4} days of play) from the evening of day {:>2}: {:>10}   {} days played{}",
                t.size,
                t.worlds,
                t.days,
                size.1.days_of_play(),
                t.from,
                show(t.time),
                t.played_days,
                if t.failed.is_empty() {
                    String::new()
                } else {
                    format!("; {} world(s) failed", t.failed.len())
                }
            );
            for failed in &t.failed {
                println!("    failed: {failed}");
            }
            times.push(t);
        }
        if live.world.state_hash() != before {
            return Err("a trial changed the live game's hash".to_string());
        }
    }
    Ok(times)
}

pub fn command(args: &[String], usage: &str, options: &LoadOptions) -> Result<(), String> {
    let s = parse(args, usage)?;
    let release = !cfg!(debug_assertions);

    println!("sim-cli time");
    println!("machine: {}", s.machine);
    println!(
        "build: {}",
        if release {
            "release"
        } else {
            "DEBUG (the times mean nothing; build with --release)"
        }
    );
    println!(
        "repeats: {} (workloads a to c, f and g); games: {} of {} days (d and e, once each)",
        s.repeats, s.games, s.days
    );
    println!("slice: {}   soak: {}", s.slice, s.soak);
    println!(
        "The times are wall-clock times of this run, outside the game; they change nothing in it."
    );
    if s.only_trial {
        let h = trials(&s, options)?;
        println!();
        println!("What it means:");
        trial_totals(&h);
        return Ok(());
    }

    let early = start_at(&s.slice, s.early, options)?;
    let late = start_at(&s.slice, s.late, options)?;
    let soak0 = start_at(&s.soak, 0, options)?;
    let soak_late = start_at(&s.soak, s.soak_late, options)?;

    println!();
    println!("Start points (events in the log):");
    for (label, start) in [
        (format!("slice at day {}", s.early), &early),
        (format!("slice at day {}", s.late), &late),
        ("soak at day 0".to_string(), &soak0),
        (format!("soak at day {}", s.soak_late), &soak_late),
    ] {
        println!("  {label:<22} {} events", start.world.events().len());
    }

    println!();
    println!("a. One day resolved (the resolve only; the copy is made before the clock starts):");
    let a_early = resolve_one_day(&early, s.repeats);
    let a_late = resolve_one_day(&late, s.repeats);
    line(
        &format!("day {} from day {}", s.early + 1, s.early),
        &a_early,
    );
    line(&format!("day {} from day {}", s.late + 1, s.late), &a_late);

    println!();
    println!("b. The same day with every question asked of every unit of both sides:");
    let mut b_total = Vec::new();
    for (name, start, from) in [("early", &early, s.early), ("late", &late, s.late)] {
        let (all, asked, moments) = resolve_asking(start, s.repeats);
        let (once, once_asked) = ask_once(start, s.repeats);
        line(
            &format!("day {} resolved, all asked at every moment", from + 1),
            &all,
        );
        println!("    {asked} questions asked in all, over {moments} moments of the day");
        line(
            &format!("the asking alone, once before day {}", from + 1),
            &once,
        );
        println!("    {once_asked} questions asked ({name})");
        b_total.push((name, all, once));
    }

    println!();
    println!("c. A copy of the world (dropped as well, as a trial drops it):");
    let mut c_all = Vec::new();
    for (label, start, day) in [
        ("slice", &early, s.early),
        ("slice", &late, s.late),
        ("soak", &soak_late, s.soak_late),
    ] {
        let (whole, log, without) = copies(start, s.repeats);
        println!(
            "  {label} at day {day}, {} events:",
            start.world.events().len()
        );
        line("  the whole world, log included", &whole);
        line("  the event log alone", &log);
        line("  the world without its log (whole minus log)", &without);
        c_all.push((label, day, whole));
    }

    println!();
    println!(
        "d. {} games of {} days from the start of the soak (one start, one seed: the games are the same game, since a copy keeps its seed; each time is a copy and its days):",
        s.games, s.days
    );
    let (d, d_total, d_days) = games(&soak0, s.games, s.days)?;
    line("one game", &d);
    println!(
        "  total for {} games: {}  ({d_days} days played in each)",
        s.games,
        show(d_total)
    );

    println!();
    println!(
        "e. {} games of {} days from the soak at day {} ({} events behind it):",
        s.games,
        s.days,
        s.soak_late,
        soak_late.world.events().len()
    );
    let (e, e_total, e_days) = games(&soak_late, s.games, s.days)?;
    line("one game", &e);
    println!(
        "  total for {} games: {}  ({e_days} days played in each; the scenario's end may cut them short)",
        s.games,
        show(e_total)
    );

    println!();
    println!(
        "f. A day's staff decisions for one side (Staff::decide on the side's view at the staff's step, each day of {} played with the staffs; each day's time is the median of {} askings, the spread is over the days):",
        s.slice, s.repeats
    );
    let slice0 = start_at(&s.slice, 0, options)?;
    let f = staff_days(&slice0, s.repeats)?;
    for d in &f {
        line(&format!("{:?}, {} days", d.side, d.counted), &d.spread);
        println!(
            "    the longest, day {}: {}",
            d.longest_day,
            show(d.longest)
        );
    }

    println!();
    println!(
        "g. A quiet week in one call (seven days resolved in a row with the staffs, as run {{days: 7}} does; {} runs each):",
        s.repeats
    );
    let mut g = Vec::new();
    for (label, start) in [
        ("from the start of the slice".to_string(), &slice0),
        (format!("from day {} of the slice", s.early), &early),
        (format!("from day {} of the slice", s.late), &late),
    ] {
        let (week, played) = quiet_week(start, s.repeats)?;
        line(&label, &week);
        println!("    {played} days played");
        g.push((label, week, played));
    }

    let h = trials(&s, options)?;

    println!();
    println!("What it means (medians; the times a designer can set \"Speed\" from):");
    println!(
        "  One day of the slice: {} early (day {}), {} late (day {}).",
        show(a_early.median),
        s.early + 1,
        show(a_late.median),
        s.late + 1
    );
    for (name, all, once) in &b_total {
        println!(
            "  A day with every question asked at every moment, {name}: {} (the asking alone, once: {}).",
            show(all.median),
            show(once.median)
        );
    }
    for (label, day, whole) in &c_all {
        println!(
            "  A copy of the {label} world at day {day}: {}.",
            show(whole.median)
        );
    }
    println!(
        "  A trial of {} games x {} days from the start of the soak: {} in all ({} a game, median).",
        s.games,
        s.days,
        show(d_total),
        show(d.median)
    );
    println!(
        "  A trial of {} games x {} days from the soak at day {}: {} in all ({} a game, median).",
        s.games,
        s.days,
        s.soak_late,
        show(e_total),
        show(e.median)
    );
    for d in &f {
        println!(
            "  A day's staff decisions for the {:?} side: {} (median over {} days of the slice), {} the longest (day {}).",
            d.side,
            show(d.spread.median),
            d.counted,
            show(d.longest),
            d.longest_day
        );
    }
    for (label, week, played) in &g {
        println!(
            "  A quiet week of {played} days {label}: {} (median).",
            show(week.median)
        );
    }
    trial_totals(&h);
    Ok(())
}

fn trial_totals(h: &[TrialTime]) {
    for t in h {
        println!(
            "  A trial, {} ({} worlds x {} days), from the evening of day {}: {}.",
            t.size,
            t.worlds,
            t.days,
            t.from,
            show(t.time)
        );
    }
}
