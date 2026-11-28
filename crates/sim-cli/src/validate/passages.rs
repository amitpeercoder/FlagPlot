//! `validate --passages`: ship passages, the second stage of `validate`
//! (`docs/step2-spec.md`, "Measures"; `data/validation/README.md`,
//! "Passages").
//!
//! A passage is one calculation, not runs: the hexes of its two ends, the
//! miles the rules charge for the sea route between them (decision 0058,
//! `Map::sea_charged_nm`), less `to_short_by_nm`, and the minutes that takes
//! at `speed_kn` with `speed_changes`. It is reported beside the record and
//! never judged.
//!
//! The umpire's tool: it reads `data/oob/place_positions.csv` for the
//! positions of the places a passage names, and nothing else under
//! `data/oob`. The simulation never reads it. No float anywhere: a position
//! becomes a hex in whole numbers (see [`hex_of`]).
//!
//! On a map about a centre (decision 0242) nothing is worked into a hex here:
//! each end is a row, and its hex is read from the map's file of located
//! positions (see [`Placer`]).

use std::collections::BTreeMap;
use std::fmt::Write;
use std::fs;
use std::path::{Path, PathBuf};

use serde::Deserialize;
use sim_core::hex::Hex;
use sim_core::map::Map;

/// The default place positions file.
pub const PLACES: &str = "data/oob/place_positions.csv";
/// The default map.
pub const MAP: &str = "data/map/south_pacific.csv";

// ------------------------------------------------------------- the input

#[derive(Deserialize)]
struct PassagesFile {
    #[serde(default)]
    passage: Vec<Passage>,
}

#[derive(Deserialize)]
struct Passage {
    id: String,
    counts: String,
    #[serde(default)]
    from_place: Option<String>,
    #[serde(default)]
    from_row: Option<u64>,
    #[serde(default)]
    from_lat: Option<String>,
    #[serde(default)]
    from_lon: Option<String>,
    #[serde(default)]
    to_place: Option<String>,
    #[serde(default)]
    to_row: Option<u64>,
    #[serde(default)]
    to_lat: Option<String>,
    #[serde(default)]
    to_lon: Option<String>,
    #[serde(default)]
    to_short_by_nm: Option<u64>,
    speed_kn: u64,
    #[serde(default)]
    speed_changes: Vec<[u64; 2]>,
    record_low: u64,
    record_high: u64,
    rows: Vec<i64>,
}

/// A row of the place positions file that a passage names.
struct PlaceRow {
    place: String,
    lat: String,
    lon: String,
}

/// Reads `place_positions.csv`: lines that start with `#` are comments; the
/// columns used are `id`, `place`, `lat` and `lon`. This is a reader of its
/// own, not `oob-check`'s: `sim-cli` stays free of that crate, and four
/// columns are all `validate` needs.
fn read_places(text: &str) -> Result<BTreeMap<u64, PlaceRow>, String> {
    let mut reader = csv::ReaderBuilder::new()
        .comment(Some(b'#'))
        .flexible(true)
        .from_reader(text.as_bytes());
    let headers = reader.headers().map_err(|e| e.to_string())?.clone();
    let column = |name: &str| {
        headers
            .iter()
            .position(|h| h == name)
            .ok_or_else(|| format!("it has no column \"{name}\""))
    };
    let (id, place, lat, lon) = (
        column("id")?,
        column("place")?,
        column("lat")?,
        column("lon")?,
    );
    let mut rows = BTreeMap::new();
    for record in reader.records() {
        let record = record.map_err(|e| e.to_string())?;
        let get = |i: usize| record.get(i).unwrap_or("").trim().to_string();
        let key: u64 = get(id)
            .parse()
            .map_err(|_| format!("row id \"{}\" is not a whole number", get(id)))?;
        let row = PlaceRow {
            place: get(place),
            lat: get(lat),
            lon: get(lon),
        };
        if rows.insert(key, row).is_some() {
            return Err(format!("row id {key} is used twice"));
        }
    }
    Ok(rows)
}

// ------------------------------------------- positions to hexes, in integers

/// Decimals kept of a latitude or longitude: positions are whole numbers of
/// 10^-8 degree.
const DECIMALS: usize = 8;
const UNIT: i128 = 100_000_000;

/// The projection of decision 0017, 2, as `map-tool`'s `projection.rs` has it,
/// in the frame of the map the passages are measured on, read from its
/// header (decision 0199, 18), with each quantity a whole number over a
/// power of ten:
///
/// - the factor for longitude (`0.978148`, the cosine of 12 degrees, for the
///   present map) is its millionths, as the header's literal;
/// - the origin, hex (0,0)'s centre (12 S, 160 E for the present map), is in
///   10^-8 degree, the longitude in the frame's run;
/// - the row spacing `34.641016` nm is 34641016 / 10^6;
/// - the hex spacing is 40 nm;
/// - nm per degree of latitude is 60, and of longitude `60 x factor`.
///
/// east = dlon x 60 x factor / (UNIT x 10^6); north = dlat x 60 / UNIT, in
/// nm, with dlat and dlon in 10^-8 degree from the origin. Then, as there,
/// `r = -north / row` and `q = east / 40 - r / 2` as exact fractions, rounded
/// by cube rounding.
#[derive(Clone, Copy, Debug)]
struct Frame {
    /// Hex (0,0)'s centre, in 10^-8 degree.
    origin_lat: i128,
    origin_lon: i128,
    /// The factor for longitude, in millionths.
    factor_millionths: i128,
    /// The western edge, in 10^-8 degree, and whether the map crosses the
    /// date line: a longitude west of the edge is then raised by 360.
    west: i128,
    crosses: bool,
}

impl Frame {
    /// From a map's frame of one factor as `sim-data` reads it: `Fixed` is
    /// millionths, and a degree here is 10^8 units, so each is a hundred
    /// times its raw value. A map about a centre (decision 0242) has no
    /// `Frame` here: see [`Placer`].
    fn of(frame: &sim_data::MapFrame) -> Option<Frame> {
        let units = |value: sim_core::fixed::Fixed| i128::from(value.raw()) * (UNIT / MILLION);
        let sim_data::MapProjection::OneFactor { factor } = frame.projection else {
            return None;
        };
        Some(Frame {
            origin_lat: units(frame.centre_lat),
            origin_lon: units(frame.centre_lon),
            factor_millionths: i128::from(factor.raw()),
            west: units(frame.west),
            crosses: frame.crosses_date_line(),
        })
    }

    /// A longitude, west negative, brought into the frame's run.
    fn in_run(&self, lon: i128) -> i128 {
        if self.crosses && lon < self.west {
            lon + 360 * UNIT
        } else {
            lon
        }
    }
}

const ROW_MILLIONTHS: i128 = 34_641_016;
const HEX_NM: i128 = 40;
const NM_PER_DEG_LAT: i128 = 60;
const MILLION: i128 = 1_000_000;

/// A latitude or longitude, a decimal string with south and west negative, as
/// whole 10^-8 degrees. `None` if it is not a plain decimal of up to eight
/// decimals within 360 degrees.
fn parse_degrees(text: &str) -> Option<i128> {
    let text = text.trim();
    let (negative, digits) = match text.strip_prefix('-') {
        Some(rest) => (true, rest),
        None => (false, text.strip_prefix('+').unwrap_or(text)),
    };
    let (whole, fraction) = digits.split_once('.').unwrap_or((digits, ""));
    let plain = |s: &str| s.bytes().all(|b| b.is_ascii_digit());
    if whole.is_empty() || !plain(whole) || !plain(fraction) || fraction.len() > DECIMALS {
        return None;
    }
    if whole.len() > 3 {
        return None;
    }
    let whole: i128 = whole.parse().ok()?;
    let mut padded = fraction.to_string();
    while padded.len() < DECIMALS {
        padded.push('0');
    }
    let fraction: i128 = padded.parse().ok()?;
    let value = whole * UNIT + fraction;
    if value > 360 * UNIT {
        return None;
    }
    Some(if negative { -value } else { value })
}

/// Rounds `n / d` (`d` positive) to the nearest whole number, halves away
/// from zero, as `f64::round` does.
fn round_div(n: i128, d: i128) -> i128 {
    if n >= 0 {
        (2 * n + d) / (2 * d)
    } else {
        -((-2 * n + d) / (2 * d))
    }
}

/// The hex whose centre is nearest a position, by the cube rounding of
/// `map-tool`'s `hex_of_nm`, worked in whole numbers.
///
/// `r_f = -north / ROW` and `q_f = east / 40 - r_f / 2` are put over one
/// denominator, `2AB`, with `A = UNIT x 10^6` and `B = UNIT x 8660254`; each
/// of `x = q`, `z = r` and `y = -x - z` is rounded to the nearest whole
/// number (a half goes away from zero), and the one with the largest
/// rounding error is put right, with the same comparisons as `map-tool`
/// (`dx > dy && dx > dz`, then `dy > dz`, else z).
///
/// A point exactly on a boundary, where two or three centres are the same
/// distance away, is decided by those rules on the exact numbers: a half
/// rounds away from zero, and equal errors fall to the last branch (z is put
/// right). `map-tool` does the same on floats, where such a point is decided
/// by rounding noise; no table of this tool's depends on a tie.
fn hex_of(frame: &Frame, lat: i128, lon: i128) -> Hex {
    let dlat = lat - frame.origin_lat;
    let dlon = frame.in_run(lon) - frame.origin_lon;
    // r_f = -(dlat x 60 / UNIT) / (ROW / 10^6)
    //     = -dlat x 15 x 10^6 / (UNIT x 8660254), as 60 / 4 and ROW / 4.
    let b = UNIT * (ROW_MILLIONTHS / 4);
    let rn = -dlat * (NM_PER_DEG_LAT / 4) * MILLION;
    // east / 40 = dlon x 60 x factor / (UNIT x 10^6 x 40)
    //           = dlon x 3 x factor / (2 x UNIT x 10^6), whole for any factor
    //           (for 0.978148 it is the same fraction as dlon x 1467222 /
    //           (UNIT x 10^6)).
    let a = 2 * UNIT * MILLION;
    let qe = dlon * (NM_PER_DEG_LAT * frame.factor_millionths / (HEX_NM / 2));
    let den = 2 * a * b;
    let x_n = 2 * qe * b - rn * a;
    let z_n = 2 * rn * a;
    let y_n = -x_n - z_n;
    let (mut x, y, mut z) = (
        round_div(x_n, den),
        round_div(y_n, den),
        round_div(z_n, den),
    );
    let dx = (x_n - x * den).abs();
    let dy = (y_n - y * den).abs();
    let dz = (z_n - z * den).abs();
    if dx > dy && dx > dz {
        x = -y - z;
    } else if dy > dz {
        // y is the worst: x and z stay as rounded.
    } else {
        z = -x - y;
    }
    Hex::new(
        i32::try_from(x).expect("a hex within 360 degrees fits"),
        i32::try_from(z).expect("a hex within 360 degrees fits"),
    )
}

// ------------------------------------------- where an end's hex comes from

/// How a passage's ends become hexes.
///
/// On a map of one factor a position is worked into a hex here, in whole
/// numbers (`hex_of`). On a map about a centre (decision 0242, rule 15) it is
/// not: only the map tool works a position into a hex there, and it wrote the
/// answer, once, to the map's file of located positions. The check reads that
/// file as text and whole numbers, by the row of the place positions file the
/// passage names. The file is under `data/map`, and is read here, as
/// `place_positions.csv` is, because this is the umpire's tool and no more
/// than three of its columns are wanted; `sim-data` has no reader of it.
enum Placer {
    Flat(Frame),
    Located {
        hexes: BTreeMap<u64, Hex>,
        /// The located file's path, for the messages.
        file: String,
    },
}

/// The located file beside a map: `<the map file's stem>_located.csv` in the
/// same folder.
fn located_path(map_path: &str) -> PathBuf {
    let path = Path::new(map_path);
    let stem = path
        .file_stem()
        .map_or(String::new(), |s| s.to_string_lossy().into_owned());
    path.with_file_name(format!("{stem}_located.csv"))
}

/// Reads a located file: lines that start with `#` are comments; the columns
/// used are `row`, `q` and `r`, all whole numbers.
fn read_located(text: &str) -> Result<BTreeMap<u64, Hex>, String> {
    let mut reader = csv::ReaderBuilder::new()
        .comment(Some(b'#'))
        .flexible(true)
        .from_reader(text.as_bytes());
    let headers = reader.headers().map_err(|e| e.to_string())?.clone();
    let column = |name: &str| {
        headers
            .iter()
            .position(|h| h == name)
            .ok_or_else(|| format!("it has no column \"{name}\""))
    };
    let (row, q, r) = (column("row")?, column("q")?, column("r")?);
    let mut hexes = BTreeMap::new();
    for record in reader.records() {
        let record = record.map_err(|e| e.to_string())?;
        let get = |i: usize| record.get(i).unwrap_or("").trim().to_string();
        let whole = |i: usize| -> Result<i32, String> {
            get(i)
                .parse()
                .map_err(|_| format!("\"{}\" is not a whole number", get(i)))
        };
        let id: u64 = get(row)
            .parse()
            .map_err(|_| format!("row \"{}\" is not a whole number", get(row)))?;
        if hexes.insert(id, Hex::new(whole(q)?, whole(r)?)).is_some() {
            return Err(format!("row {id} is used twice"));
        }
    }
    Ok(hexes)
}

impl Placer {
    /// For a map: its frame as `sim-data` reads it (`None` for a map with no
    /// frame, measured in the present map's frame as before decision 0199).
    fn of(map_path: &str, frame: &sim_data::MapFrame) -> Result<Placer, String> {
        if let Some(flat) = Frame::of(frame) {
            return Ok(Placer::Flat(flat));
        }
        let path = located_path(map_path);
        let shown = path.display().to_string();
        let bytes = fs::read(&path).map_err(|e| {
            format!(
                "{map_path}: this map is about a centre, so its ends are read from its file of located positions, {shown}, which is wanted beside it and is not readable ({e})"
            )
        })?;
        let text = String::from_utf8(bytes).map_err(|_| format!("{shown}: not text"))?;
        let hexes = read_located(&text).map_err(|e| format!("{shown}: {e}"))?;
        Ok(Placer::Located { hexes, file: shown })
    }
}

// ------------------------------------------------------ the calculation

/// Where a passage's end is, and how it is described in the report.
struct End {
    hex: Hex,
    text: String,
}

/// The minute, rounded up, in which `miles` are reached, steaming at `speed`
/// knots and changing at the minutes in `changes` (each the speed held from
/// that minute on). `None` if the miles are never reached: the last speed is
/// 0 and some are left.
///
/// Whole numbers: a knot is a mile an hour, so in units of mile-minutes / 60,
/// `speed x minutes` is what has been steamed and the target is `miles x 60`.
fn minutes_to_steam(miles: u64, speed: u64, changes: &[[u64; 2]]) -> Option<u64> {
    let target = u128::from(miles) * 60;
    let mut done: u128 = 0;
    let mut start: u64 = 0;
    let mut speed = u128::from(speed);
    let mut next = changes.iter();
    loop {
        let change = next.next();
        let until = change.map(|c| c[0]);
        let reachable = match until {
            Some(end) => done + speed * u128::from(end - start),
            None => u128::MAX,
        };
        if target <= done {
            return Some(start);
        }
        if speed > 0 && target <= reachable {
            let need = target - done;
            let more = need.div_ceil(speed);
            return Some(start + u64::try_from(more).ok()?);
        }
        let c = change?;
        done = reachable;
        start = c[0];
        speed = u128::from(c[1]);
    }
}

fn speed_words(speed: u64, changes: &[[u64; 2]]) -> String {
    let mut text = format!("{speed} knots");
    for [minute, knots] in changes {
        if *knots == 0 {
            let _ = write!(text, "; stopped from minute {minute}");
        } else {
            let _ = write!(text, "; {knots} knots from minute {minute}");
        }
    }
    text
}

fn record_text(low: u64, high: u64) -> String {
    if low == high {
        low.to_string()
    } else {
        format!("{low} to {high}")
    }
}

/// What a passage came to: a row of cells, or why it could not be worked.
struct Worked {
    straight: u32,
    charged: u32,
    counted: u32,
    minutes: u64,
}

fn work(
    map: &Map,
    from: Hex,
    to: Hex,
    short: u64,
    speed: u64,
    changes: &[[u64; 2]],
) -> Result<Worked, String> {
    for (name, hex) in [("start", from), ("end", to)] {
        match map.get(hex) {
            None => {
                return Err(format!(
                    "the {name} hex ({},{}) is off the map",
                    hex.q, hex.r
                ))
            }
            Some(_) if !map.is_navigable(hex) => {
                return Err(format!(
                    "the {name} hex ({},{}) is land: ships cannot enter it",
                    hex.q, hex.r
                ))
            }
            Some(_) => {}
        }
    }
    let charged = map
        .sea_charged_nm(from, to)
        .ok_or_else(|| "there is no sea route between the two hexes".to_string())?;
    let short = u32::try_from(short).map_err(|_| "to_short_by_nm is too large".to_string())?;
    let counted = charged.checked_sub(short).ok_or_else(|| {
        format!("to_short_by_nm {short} is more than the {charged} miles charged")
    })?;
    let minutes = minutes_to_steam(u64::from(counted), speed, changes)
        .ok_or_else(|| "the speed profile ends stopped before the miles are steamed".to_string())?;
    Ok(Worked {
        straight: from.air_distance_nm(to),
        charged,
        counted,
        minutes,
    })
}

// ------------------------------------------------------------ the command

fn label_ok(label: &str) -> bool {
    super::label_ok(label)
}

/// What the words after `validate` gave, for the passages form.
pub struct Request<'a> {
    pub passages: &'a str,
    pub label: Option<&'a str>,
    pub map: Option<&'a str>,
    pub places: Option<&'a str>,
    pub out: Option<PathBuf>,
}

fn end_of(
    placer: &Placer,
    passage: &Passage,
    which: &str,
    places: &BTreeMap<u64, PlaceRow>,
) -> Result<End, String> {
    let id = &passage.id;
    let (place, row, lat, lon) = if which == "from" {
        (
            &passage.from_place,
            passage.from_row,
            &passage.from_lat,
            &passage.from_lon,
        )
    } else {
        (
            &passage.to_place,
            passage.to_row,
            &passage.to_lat,
            &passage.to_lon,
        )
    };
    let bad = |what: String| format!("passage {id}: {which}: {what}");
    match (row, lat, lon) {
        (Some(_), Some(_), _) | (Some(_), _, Some(_)) => {
            Err(bad("it gives both a row and a position".to_string()))
        }
        (Some(row), None, None) => {
            let found = places
                .get(&row)
                .ok_or_else(|| bad(format!("row {row} is not in the place positions file")))?;
            if found.lat.is_empty() || found.lon.is_empty() {
                return Err(bad(format!("row {row} has no position")));
            }
            if let Some(named) = place {
                if *named != found.place {
                    return Err(bad(format!(
                        "row {row} is of the place {}, not {named}",
                        found.place
                    )));
                }
            }
            if let Placer::Located { hexes, file } = placer {
                let hex = *hexes.get(&row).ok_or_else(|| {
                    bad(format!(
                        "row {row} is not in the file of located positions {file}"
                    ))
                })?;
                return Ok(End {
                    hex,
                    text: format!("row {row}, {}, ({},{})", found.place, hex.q, hex.r),
                });
            }
            let Placer::Flat(frame) = placer else {
                unreachable!("a located placer returned above")
            };
            let (la, lo) = (
                parse_degrees(&found.lat).ok_or_else(|| {
                    bad(format!(
                        "row {row}: latitude \"{}\" is not a decimal",
                        found.lat
                    ))
                })?,
                parse_degrees(&found.lon).ok_or_else(|| {
                    bad(format!(
                        "row {row}: longitude \"{}\" is not a decimal",
                        found.lon
                    ))
                })?,
            );
            let hex = hex_of(frame, la, lo);
            Ok(End {
                hex,
                text: format!("row {row}, {}, ({},{})", found.place, hex.q, hex.r),
            })
        }
        (None, Some(_), Some(_)) if matches!(placer, Placer::Located { .. }) => {
            Err(bad("on a map about a centre an end is a row of the place positions file, since only the map tool works a position into a hex there".to_string()))
        }
        (None, Some(lat), Some(lon)) => {
            let Placer::Flat(frame) = placer else {
                unreachable!("a located placer returned above")
            };
            let (la, lo) = (
                parse_degrees(lat)
                    .ok_or_else(|| bad(format!("latitude \"{lat}\" is not a decimal")))?,
                parse_degrees(lon)
                    .ok_or_else(|| bad(format!("longitude \"{lon}\" is not a decimal")))?,
            );
            let hex = hex_of(frame, la, lo);
            Ok(End {
                hex,
                text: format!("position {lat}, {lon}, ({},{})", hex.q, hex.r),
            })
        }
        (None, None, None) => Err(bad("it gives neither a row nor a position".to_string())),
        _ => Err(bad(
            "a position needs both a latitude and a longitude".to_string()
        )),
    }
}

pub fn command(request: &Request) -> Result<(), String> {
    let label = request
        .label
        .ok_or_else(|| "--passages needs --label TEXT".to_string())?;
    if !label_ok(label) {
        return Err(format!(
            "the label \"{label}\" is not letters, digits, dot, dash and underscore"
        ));
    }
    let map_path = request.map.unwrap_or(MAP);
    let places_path = request.places.unwrap_or(PLACES);

    let passages_bytes =
        fs::read(request.passages).map_err(|e| format!("{}: {e}", request.passages))?;
    let passages_hash = crate::save::hash_hex(&passages_bytes);
    let passages_text =
        String::from_utf8(passages_bytes).map_err(|_| format!("{}: not text", request.passages))?;
    let file: PassagesFile =
        toml::from_str(&passages_text).map_err(|e| format!("{}: {e}", request.passages))?;
    let map_bytes = fs::read(map_path).map_err(|e| format!("{map_path}: {e}"))?;
    let map_hash = crate::save::hash_hex(&map_bytes);
    let map = sim_data::load_map(Path::new(map_path)).map_err(|e| e.to_string())?;
    // A map with no frame (a hand-written map) is measured as before
    // decision 0199: in the present map's frame, read from its header.
    let frame = match sim_data::read_map_frame(Path::new(map_path)).map_err(|e| e.to_string())? {
        Some(frame) => frame,
        None => sim_data::read_map_frame(Path::new(MAP))
            .map_err(|e| e.to_string())?
            .ok_or_else(|| format!("{MAP}: the present map has no frame in its header"))?,
    };
    let placer = Placer::of(map_path, &frame)?;
    let places_bytes = fs::read(places_path).map_err(|e| format!("{places_path}: {e}"))?;
    let places_hash = crate::save::hash_hex(&places_bytes);
    let places_text =
        String::from_utf8(places_bytes).map_err(|_| format!("{places_path}: not text"))?;
    let places = read_places(&places_text).map_err(|e| format!("{places_path}: {e}"))?;

    let mut seen = std::collections::BTreeSet::new();
    let mut out = String::new();
    let _ = writeln!(out, "# Passages: validation of `{label}`\n");
    let _ = writeln!(
        out,
        "Passages: {} (hash {passages_hash}), {} passages",
        request.passages,
        file.passage.len()
    );
    let _ = writeln!(out, "Map: {map_path} (hash {map_hash})");
    let _ = writeln!(out, "Place positions: {places_path} (hash {places_hash})");
    let _ = writeln!(out, "Label: `{label}`\n");
    let _ = writeln!(
        out,
        "No passage is judged. Each is one calculation, worked from the map and the rules' charge for a sea route (decision 0058), and reported beside the record. A place is at the centre of the hex its position falls in.\n"
    );
    let _ = writeln!(
        out,
        "| Passage | From | To | Straight line (nm) | Charged (nm) | Short by (nm) | Miles counted | Speed | Calculated (min) | Record (min) | Calculated less record (min) | |"
    );
    let _ = writeln!(
        out,
        "| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |"
    );
    for passage in &file.passage {
        if !seen.insert(passage.id.as_str()) {
            return Err(format!(
                "{}: the id {} is used twice",
                request.passages, passage.id
            ));
        }
        if passage.record_low > passage.record_high {
            return Err(format!(
                "{}: passage {} has record_low above record_high",
                request.passages, passage.id
            ));
        }
        let mut last = 0;
        for [minute, _] in &passage.speed_changes {
            if *minute <= last {
                return Err(format!(
                    "{}: passage {}: speed_changes minutes must rise and be above 0",
                    request.passages, passage.id
                ));
            }
            last = *minute;
        }
        let from = end_of(&placer, passage, "from", &places)?;
        let to = end_of(&placer, passage, "to", &places)?;
        let short = passage.to_short_by_nm.unwrap_or(0);
        let speed = speed_words(passage.speed_kn, &passage.speed_changes);
        let record = record_text(passage.record_low, passage.record_high);
        let short_text = passage
            .to_short_by_nm
            .map_or("-".to_string(), |s| s.to_string());
        match work(
            &map,
            from.hex,
            to.hex,
            short,
            passage.speed_kn,
            &passage.speed_changes,
        ) {
            Ok(w) => {
                let delta = if w.minutes < passage.record_low {
                    i128::from(w.minutes) - i128::from(passage.record_low)
                } else if w.minutes > passage.record_high {
                    i128::from(w.minutes) - i128::from(passage.record_high)
                } else {
                    0
                };
                let _ = writeln!(
                    out,
                    "| {} | {} | {} | {} | {} | {short_text} | {} | {speed} | {} | {record} | {delta:+} | reported |",
                    passage.id, from.text, to.text, w.straight, w.charged, w.counted, w.minutes
                );
            }
            Err(why) => {
                let _ = writeln!(
                    out,
                    "| {} | {} | {} | - | - | {short_text} | - | {speed} | - | {record} | - | not worked: {why} |",
                    passage.id, from.text, to.text
                );
            }
        }
    }
    out.push('\n');
    for passage in &file.passage {
        let rows: Vec<String> = passage.rows.iter().map(i64::to_string).collect();
        let _ = writeln!(out, "### {}\n", passage.id);
        let _ = writeln!(out, "- Counts: {}", passage.counts);
        let _ = writeln!(out, "- Rows: {}\n", rows.join(", "));
    }
    let _ = writeln!(out, "No passage is judged: these are reported to show the size of the difference between the record and the rules, which hold a force to its ordered speed on a straight course.");

    let path = request
        .out
        .clone()
        .unwrap_or_else(|| PathBuf::from(format!("docs/validation/passages-{label}.md")));
    let wrote = super::write_report(&path, &out)?;
    println!(
        "{} {}: {} passages, none judged",
        if wrote { "wrote" } else { "unchanged" },
        path.display(),
        file.passage.len()
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The present map's frame, read from its header as the command reads it.
    fn present() -> Frame {
        let path = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../")
            .join(MAP);
        Frame::of(
            &sim_data::read_map_frame(&path)
                .unwrap()
                .expect("the map has a frame"),
        )
        .expect("one factor")
    }

    /// 50 positions and the hexes `map-tool locate` gave them, made with
    ///
    /// `cargo run -q --release -p map-tool -- locate --lat LAT --lon LON`
    ///
    /// for each (the hex is the "is hex (q,r)" of its first line). They are:
    /// the six rows of `place_positions.csv` the passages name (ids 4, 6,
    /// 10, 12, 24, 51) and the three positions given in `passages.toml`; the
    /// origin and points in all four quadrants about hex (0,0) and the hexes
    /// beside it; points 19.5 to 23 nm from a centre in assorted directions
    /// (the hex circumradius is 23.1 nm, so those lie near edges and
    /// corners); and a run of points zig-zagging across the (0,0) / (1,0) /
    /// (-1,0) / (-1,1) / (0,1) edges. Where the two disagree, this table
    /// is wrong or the integer code is, and nobody has changed either to
    /// make them agree.
    const TABLE: [(&str, &str, i32, i32); 50] = [
        ("-9.09975", "160.15356", 3, -5),
        ("-9.40538", "160.02861", 2, -4),
        ("-9.1303", "159.81619", 2, -5),
        ("-15.55075", "167.15375", 7, 6),
        ("-7.07268", "155.88036", -2, -8),
        ("-9.36806", "159.86412", 2, -5),
        ("-7.5371", "158.2126", 1, -8),
        ("-6.2417", "165.8833", 14, -10),
        ("-1.3917", "161.7833", 12, -18),
        ("-12.0", "160.0", 0, 0),
        ("-11.5", "160.6", 1, -1),
        ("-11.5", "159.4", 0, -1),
        ("-12.5", "160.6", 0, 1),
        ("-12.5", "159.4", -1, 1),
        ("-10.0", "162.0", 5, -4),
        ("-15.0", "157.0", -7, 5),
        ("-4.0", "152.0", -5, -14),
        ("-13.3", "161.2", 1, 2),
        ("-10.7", "158.9", -1, -2),
        ("-12.00000", "160.33226", 0, 0),
        ("-11.81667", "160.32464", 0, 0),
        ("-11.66802", "160.19595", 1, -1),
        ("-11.67000", "160.00000", 0, 0),
        ("-11.81250", "159.66799", 0, 0),
        ("-12.00000", "159.66433", 0, 0),
        ("-12.18500", "159.67241", 0, 0),
        ("-12.33167", "160.00000", 0, 0),
        ("-12.19000", "160.33644", 0, 0),
        ("-12.00000", "162.38887", 4, 0),
        ("-11.66947", "162.23978", 4, -1),
        ("-12.85159", "157.77641", -4, 1),
        ("-9.81601", "161.69243", 4, -4),
        ("-15.18264", "157.10763", -7, 6),
        ("-8.27073", "159.58953", 3, -7),
        ("-10.21122", "156.60409", -3, -3),
        ("-12.45830", "160.06130", 0, 1),
        ("-12.41660", "159.87740", -1, 1),
        ("-12.37490", "160.18390", 0, 1),
        ("-12.33320", "159.75480", -1, 1),
        ("-12.29150", "160.30650", 0, 1),
        ("-12.24980", "159.63220", -1, 1),
        ("-12.20810", "160.42910", 1, 0),
        ("-12.16640", "159.50960", -1, 0),
        ("-12.12470", "160.55170", 1, 0),
        ("-12.08300", "159.38700", -1, 0),
        ("-12.04130", "160.67430", 1, 0),
        ("-11.99960", "159.26440", -1, 0),
        ("-11.95790", "160.79690", 1, 0),
        ("-11.91620", "159.14180", -1, 0),
        ("-11.87450", "160.91950", 1, 0),
    ];

    #[test]
    fn positions_become_the_hexes_map_tool_gives() {
        for (lat, lon, q, r) in TABLE {
            let hex = hex_of(
                &present(),
                parse_degrees(lat).unwrap(),
                parse_degrees(lon).unwrap(),
            );
            assert_eq!((hex.q, hex.r), (q, r), "at {lat}, {lon}");
        }
    }

    #[test]
    fn degrees_are_read_as_whole_numbers() {
        assert_eq!(parse_degrees("-12"), Some(-1_200_000_000));
        assert_eq!(parse_degrees("160.5"), Some(16_050_000_000));
        assert_eq!(parse_degrees("-0.00000001"), Some(-1));
        assert_eq!(parse_degrees("+7.25"), Some(725_000_000));
        for bad in ["", "-", ".5", "1e3", "1.234567891", "abc", "1.2.3", "999"] {
            assert_eq!(parse_degrees(bad), None, "{bad:?}");
        }
    }

    #[test]
    fn a_hex_centre_is_in_its_own_hex() {
        // The centre of (q, r) is 40 x (q + r/2) nm east and 34.641016 x r
        // south of the origin; to the nearest 10^-8 degree it falls in its own
        // hex. Worked in integers: latitude = -12 - r x 34641016 / (60 x 10^6),
        // longitude = 160 + 40 x (q + r/2) x 10^6 / (60 x 978148).
        let frame = present();
        assert_eq!(
            (frame.origin_lat, frame.origin_lon, frame.factor_millionths),
            (-12 * UNIT, 160 * UNIT, 978_148)
        );
        for r in -30..=30i128 {
            for q in -40..=40i128 {
                let lat = -12 * UNIT - round_div(r * 34_641_016 * UNIT, 60 * MILLION);
                let lon = 160 * UNIT
                    + round_div(
                        (2 * q + r) * 20 * MILLION * UNIT,
                        60 * frame.factor_millionths,
                    );
                let hex = hex_of(&frame, lat, lon);
                assert_eq!((i128::from(hex.q), i128::from(hex.r)), (q, r));
            }
        }
    }

    /// On the Midway map, whose frame crosses the date line (decision 0199),
    /// the centre each row of the map file gives, its longitude west
    /// negative and to four places, falls in that row's own hex.
    #[test]
    fn a_hex_centre_on_the_midway_map_is_in_its_own_hex() {
        let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../data/map/midway.csv");
        let frame = Frame::of(&sim_data::read_map_frame(&path).unwrap().expect("a frame"))
            .expect("one factor");
        assert!(frame.crosses);
        let text = fs::read_to_string(&path).unwrap();
        let mut west_of_the_line = 0;
        for line in text.lines().filter(|l| !l.starts_with('#')).skip(1) {
            let fields: Vec<&str> = line.split(',').collect();
            let (q, r): (i32, i32) = (fields[0].parse().unwrap(), fields[1].parse().unwrap());
            let (lat, lon) = (
                parse_degrees(fields[4]).unwrap(),
                parse_degrees(fields[5]).unwrap(),
            );
            if lon < 0 {
                west_of_the_line += 1;
            }
            let hex = hex_of(&frame, lat, lon);
            assert_eq!((hex.q, hex.r), (q, r), "{line}");
        }
        assert!(west_of_the_line > 500, "{west_of_the_line}");
        // Midway's atoll, row 5100 of place_positions.csv, is the hex the
        // map tool marked, (5,1).
        let hex = hex_of(
            &frame,
            parse_degrees("28.232").unwrap(),
            parse_degrees("-177.370").unwrap(),
        );
        assert_eq!((hex.q, hex.r), (5, 1));
    }

    #[test]
    fn the_minutes_are_rounded_up_to_the_minute_the_miles_are_reached() {
        assert_eq!(minutes_to_steam(120, 30, &[]), Some(240));
        assert_eq!(minutes_to_steam(120, 28, &[]), Some(258));
        assert_eq!(minutes_to_steam(120, 30, &[[240, 10]]), Some(240));
        assert_eq!(minutes_to_steam(0, 30, &[]), Some(0));
        assert_eq!(minutes_to_steam(10, 0, &[]), None);
        assert_eq!(minutes_to_steam(10, 30, &[[10, 0]]), None);
    }
}
