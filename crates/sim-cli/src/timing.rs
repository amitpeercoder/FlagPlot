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
//! the soak and from day 150.

use std::hint::black_box;
use std::time::{Duration, Instant};

use sim_core::World;
use sim_data::{LoadOptions, Scenario};

const DEFAULT_SLICE: &str = "data/scenarios/07_full_slice.toml";
const DEFAULT_SOAK: &str = "data/scenarios/08_soak.toml";

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
    while scenario.world.day() < day {
        scenario
            .next_day()
            .map_err(|e| e.to_string())?
            .ok_or_else(|| format!("{path} ended before day {day}"))?;
    }
    Ok(Start {
        script: scenario.scripted_orders().clone(),
        days: scenario.days,
        world: scenario.world,
    })
}

impl Start {
    /// Hands in the scripted orders of `day` to `world`.
    fn submit(&self, world: &mut World, day: u32) {
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
            world.resolve_day().map_err(|e| e.to_string())?;
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
        timed(|| world.resolve_day().map(|s| s.state_hash))
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
            world.resolve_day_watched(&mut |w, _moment| {
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
        "repeats: {} (workloads a to c); games: {} of {} days (d and e, once each)",
        s.repeats, s.games, s.days
    );
    println!("slice: {}   soak: {}", s.slice, s.soak);
    println!(
        "The times are wall-clock times of this run, outside the game; they change nothing in it."
    );

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
    Ok(())
}
