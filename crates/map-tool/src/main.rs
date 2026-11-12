//! The map tool's command line. See `--help`.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

use map_tool::generate::{csv_text, generate_with_anchors, parse_anchors, Kind};
use map_tool::projection::{centre_of, degrees_text, hex_of};
use map_tool::{render, shp, water};
use sim_core::hex::{Hex, HEX_NM};
use sim_core::map::{HexKind, Map};

const DEFAULT_MAP: &str = "data/map/south_pacific.csv";

const USAGE: &str = "usage:
  map-tool generate --land <shp> --minor-islands <shp> --out <csv> [--anchors <csv>]
  map-tool render [--map <csv>] [--mark q,r=X ...]
  map-tool locate --lat <deg> --lon <deg> [--map <csv>]
  map-tool path --from q,r --to q,r [--map <csv>]
  map-tool sides --hex q,r [--map <csv>]
  map-tool water --lat <deg> --lon <deg> [--land <shp>] [--minor-islands <shp>]
                 (defaults: references/natural-earth/ne_10m_land/ne_10m_land.shp and
                  references/natural-earth/ne_10m_minor_islands/ne_10m_minor_islands.shp)

render marks coast hexes with a closed side toward water as % (+ otherwise).
Each row is labelled with its r and the q of its first hex.";

/// Flags and their values, in order. Every flag takes one value, and the
/// value is whatever follows, so `--lat -4.2` works.
fn parse_flags(args: &[String]) -> Result<Vec<(String, String)>, String> {
    let mut flags = Vec::new();
    let mut iter = args.iter();
    while let Some(flag) = iter.next() {
        if !flag.starts_with("--") {
            return Err(format!("unexpected argument {flag}"));
        }
        let value = iter.next().ok_or_else(|| format!("{flag} needs a value"))?;
        flags.push((flag.clone(), value.clone()));
    }
    Ok(flags)
}

struct Flags(Vec<(String, String)>);

impl Flags {
    fn all(&self, name: &str) -> Vec<&str> {
        self.0
            .iter()
            .filter(|(flag, _)| flag == name)
            .map(|(_, v)| v.as_str())
            .collect()
    }

    fn get(&self, name: &str) -> Option<&str> {
        self.all(name).last().copied()
    }

    fn require(&self, name: &str) -> Result<&str, String> {
        self.get(name).ok_or_else(|| format!("{name} is required"))
    }

    fn check_known(&self, known: &[&str]) -> Result<(), String> {
        match self
            .0
            .iter()
            .find(|(flag, _)| !known.contains(&flag.as_str()))
        {
            Some((flag, _)) => Err(format!("unknown flag {flag}")),
            None => Ok(()),
        }
    }

    fn map_path(&self) -> PathBuf {
        PathBuf::from(self.get("--map").unwrap_or(DEFAULT_MAP))
    }
}

fn parse_hex(text: &str) -> Result<(i32, i32), String> {
    let (q, r) = text
        .split_once(',')
        .ok_or_else(|| format!("\"{text}\" is not q,r"))?;
    let number = |part: &str| {
        part.trim()
            .parse::<i32>()
            .map_err(|_| format!("\"{part}\" in \"{text}\" is not a whole number"))
    };
    Ok((number(q)?, number(r)?))
}

fn parse_mark(text: &str) -> Result<((i32, i32), char), String> {
    let (hex, mark) = text
        .split_once('=')
        .ok_or_else(|| format!("\"{text}\" is not q,r=X"))?;
    let mut chars = mark.chars();
    match (chars.next(), chars.next()) {
        (Some(c), None) => Ok((parse_hex(hex)?, c)),
        _ => Err(format!("the mark in \"{text}\" must be one character")),
    }
}

fn parse_degrees(flags: &Flags, name: &str) -> Result<f64, String> {
    let text = flags.require(name)?;
    text.parse::<f64>()
        .map_err(|_| format!("{name} \"{text}\" is not a number"))
}

fn run_generate(flags: &Flags, command_line: &str) -> Result<(), String> {
    flags.check_known(&["--land", "--minor-islands", "--out", "--anchors"])?;
    let anchors_file = flags.get("--anchors");
    let anchors = match anchors_file {
        Some(file) => {
            let text = std::fs::read_to_string(file).map_err(|e| format!("{file}: {e}"))?;
            parse_anchors(&text).map_err(|e| format!("{file}: {e}"))?
        }
        None => Vec::new(),
    };
    let mut polygons = shp::read_polygons(Path::new(flags.require("--land")?))?;
    polygons.extend(shp::read_polygons(Path::new(
        flags.require("--minor-islands")?,
    ))?);
    let out = flags.require("--out")?;
    println!("read {} polygon records", polygons.len());
    let map = generate_with_anchors(polygons, &anchors)?;
    for note in &map.anchors {
        println!("anchorage {}", note.text().trim_start());
    }
    let text = csv_text(&map, command_line, anchors_file);
    std::fs::write(out, text).map_err(|e| format!("{out}: {e}"))?;
    let count = |kind: Kind| map.hexes.iter().filter(|h| h.kind == kind).count();
    println!(
        "wrote {out}: {} hexes: {} land, {} coast, {} deep_sea",
        map.hexes.len(),
        count(Kind::Land),
        count(Kind::Coast),
        count(Kind::DeepSea)
    );
    println!(
        "coast hexes with land_percent 0: {}",
        map.zero_percent_coast
    );
    Ok(())
}

fn run_render(flags: &Flags) -> Result<(), String> {
    flags.check_known(&["--map", "--mark"])?;
    let map = load(flags)?;
    let marks = flags
        .all("--mark")
        .into_iter()
        .map(parse_mark)
        .collect::<Result<Vec<_>, _>>()?;
    print!("{}", render::render(&map, &marks));
    Ok(())
}

fn run_locate(flags: &Flags) -> Result<(), String> {
    flags.check_known(&["--lat", "--lon", "--map"])?;
    let lat = parse_degrees(flags, "--lat")?;
    let lon = parse_degrees(flags, "--lon")?;
    let (q, r) = hex_of(lat, lon);
    let (clat, clon) = centre_of(q, r);
    println!("lat {lat}, lon {lon} is hex ({q},{r})");
    println!(
        "centre: lat {}, lon {}",
        degrees_text(clat),
        degrees_text(clon)
    );
    match load(flags) {
        Ok(map) => match map.get(Hex::new(q, r)) {
            Some(data) => {
                println!(
                    "map: {}, land_percent {}",
                    kind_name(&map, Hex::new(q, r)),
                    data.land_percent
                );
                println!("closed sides: {}", closed_sides_text(&map, Hex::new(q, r)));
            }
            None => println!("map: the hex is not on the map"),
        },
        Err(e) => println!("map: not read ({e})"),
    }
    Ok(())
}

fn load(flags: &Flags) -> Result<Map, String> {
    sim_data::load_map(&flags.map_path()).map_err(|e| e.to_string())
}

/// The closed sides of a hex as text: direction (vector) and neighbour.
fn closed_sides_text(map: &Map, hex: Hex) -> String {
    let Some(data) = map.get(hex) else {
        return "none".to_string();
    };
    let list: Vec<String> = Hex::DIRECTIONS
        .iter()
        .enumerate()
        .filter(|(d, _)| data.closed_sides[*d])
        .map(|(d, (dq, dr))| format!("{d} ({dq:+},{dr:+}) to ({},{})", hex.q + dq, hex.r + dr))
        .collect();
    if list.is_empty() {
        "none".to_string()
    } else {
        list.join(", ")
    }
}

fn kind_name(map: &Map, hex: Hex) -> &'static str {
    match map.get(hex).map(|d| d.kind) {
        Some(HexKind::Land) => "land",
        Some(HexKind::Coast) => "coast",
        Some(HexKind::DeepSea) => "deep_sea",
        Some(HexKind::ShallowSea) => "shallow_sea",
        None => "off the map",
    }
}

fn run_sides(flags: &Flags) -> Result<(), String> {
    flags.check_known(&["--hex", "--map"])?;
    let (q, r) = parse_hex(flags.require("--hex")?)?;
    let hex = Hex::new(q, r);
    let map = load(flags)?;
    let Some(data) = map.get(hex) else {
        return Err(format!("({q},{r}) is not on the map"));
    };
    println!(
        "hex ({q},{r}): {}, land_percent {}",
        kind_name(&map, hex),
        data.land_percent
    );
    for (d, (dq, dr)) in Hex::DIRECTIONS.iter().enumerate() {
        let neighbor = Hex::new(q + dq, r + dr);
        let flag = |closed: bool| if closed { "closed" } else { "open" };
        let state = match map.get(neighbor) {
            None => "off the map".to_string(),
            Some(other) => format!(
                "{}; file flags: this hex {}, neighbour {}",
                if map.can_step(hex, d) {
                    "a ship can step"
                } else {
                    "a ship cannot step"
                },
                flag(data.closed_sides[d]),
                flag(other.closed_sides[(d + 3) % 6])
            ),
        };
        println!(
            "  {d}  ({dq:+},{dr:+})  ({},{})  {}  {state}",
            neighbor.q,
            neighbor.r,
            kind_name(&map, neighbor)
        );
    }
    Ok(())
}

fn run_water(flags: &Flags) -> Result<(), String> {
    flags.check_known(&["--lat", "--lon", "--land", "--minor-islands"])?;
    let lat = parse_degrees(flags, "--lat")?;
    let lon = parse_degrees(flags, "--lon")?;
    let land = flags
        .get("--land")
        .unwrap_or("references/natural-earth/ne_10m_land/ne_10m_land.shp");
    let minor = flags
        .get("--minor-islands")
        .unwrap_or("references/natural-earth/ne_10m_minor_islands/ne_10m_minor_islands.shp");
    let mut polygons = shp::read_polygons(Path::new(land))?;
    polygons.extend(shp::read_polygons(Path::new(minor))?);
    // Only the polygons near the point are indexed.
    let polygons = water::keep_near_point(polygons, lat, lon);
    print!("{}", water::report(lat, lon, &polygons));
    Ok(())
}

fn run_path(flags: &Flags) -> Result<(), String> {
    flags.check_known(&["--from", "--to", "--map"])?;
    let (fq, fr) = parse_hex(flags.require("--from")?)?;
    let (tq, tr) = parse_hex(flags.require("--to")?)?;
    let (from, to) = (Hex::new(fq, fr), Hex::new(tq, tr));
    let map = load(flags)?;
    let straight = from.distance(to);
    let air = from.air_distance_nm(to);
    println!(
        "from ({},{}) [{}] to ({},{}) [{}]",
        from.q,
        from.r,
        kind_name(&map, from),
        to.q,
        to.r,
        kind_name(&map, to)
    );
    let distances: BTreeMap<Hex, u32> = map.sea_distances_to(to);
    let Some(&steps) = distances.get(&from) else {
        println!("UNREACHABLE by sea: {}", why_unreachable(&map, from, to));
        println!("hex count with no land in the way: {straight} hexes");
        println!("air distance: {air} nm (the game's measure for aircraft, decision 0040)");
        return Ok(());
    };
    let mut hex = from;
    let mut path = vec![hex];
    while let Some(next) = map.next_step_in(&distances, hex) {
        path.push(next);
        hex = next;
    }
    println!("path ({} hexes including both ends):", path.len());
    for (i, h) in path.iter().enumerate() {
        println!("  {i:>3}  ({},{})  {}", h.q, h.r, kind_name(&map, *h));
    }
    println!(
        "path length: {steps} hexes = {} nm charged to a ship (decision 0058)",
        map.sea_charged_nm_in(&distances, from, to)
            .expect("the path was found")
    );
    println!(
        "hex count with no land in the way: {straight} hexes = {} nm ({} hexes of detour)",
        i64::from(straight) * HEX_NM,
        steps - straight
    );
    println!("air distance: {air} nm (the game's measure for aircraft, decision 0040)");
    Ok(())
}

fn why_unreachable(map: &Map, from: Hex, to: Hex) -> &'static str {
    if !map.is_navigable(to) {
        "the target is land or off the map"
    } else if !map.is_navigable(from) {
        "the start is land or off the map"
    } else {
        "there is no way round the land and closed sides"
    }
}

fn run(args: &[String]) -> Result<(), String> {
    let Some((command, rest)) = args.split_first() else {
        return Err(USAGE.to_string());
    };
    let flags = Flags(parse_flags(rest)?);
    match command.as_str() {
        "generate" => run_generate(&flags, &args.join(" ")),
        "render" => run_render(&flags),
        "locate" => run_locate(&flags),
        "path" => run_path(&flags),
        "sides" => run_sides(&flags),
        "water" => run_water(&flags),
        other => Err(format!("unknown command {other}\n{USAGE}")),
    }
}

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    match run(&args) {
        Ok(()) => ExitCode::SUCCESS,
        Err(message) => {
            eprintln!("{message}");
            ExitCode::FAILURE
        }
    }
}
