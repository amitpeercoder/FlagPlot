//! Checks on the committed generated maps: every `.csv` file of `data/map`
//! whose header gives a frame (decision 0199), the present map
//! `south_pacific.csv`, the Midway map `midway.csv` and the map from Papua to
//! Midway `papua_to_midway.csv` (decision 0242) among them. It needs
//! nothing from references/. Each map is loaded through `sim_data::load_map`,
//! which checks the symmetry of the closed sides; the raw text is parsed here
//! to check the rest, in the frame its own header gives.

use std::collections::BTreeSet;
use std::path::PathBuf;

use map_tool::frame::Frame;
use map_tool::generate::{
    classify, closed_sides_text, extent_hexes, hex_points, parse_anchors, Kind,
    MIN_ANCHOR_BODY_POINTS,
};
use map_tool::projection::degrees_text;
use sim_core::hex::Hex;
use sim_core::map::HexKind;

const HEADER: &str = "q,r,kind,land_percent,lat,lon,water_bodies,closed_sides";

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../")
}

fn map_path() -> PathBuf {
    root().join("data/map/south_pacific.csv")
}

fn midway_path() -> PathBuf {
    root().join("data/map/midway.csv")
}

/// Every committed generated map: each `.csv` of `data/map` whose header
/// gives a frame, with that frame. The anchors and land links files have
/// none.
fn generated_maps() -> Vec<(PathBuf, Frame)> {
    let mut paths: Vec<PathBuf> = std::fs::read_dir(root().join("data/map"))
        .unwrap()
        .map(|entry| entry.unwrap().path())
        .filter(|path| path.extension().and_then(|e| e.to_str()) == Some("csv"))
        .collect();
    paths.sort();
    let maps: Vec<(PathBuf, Frame)> = paths
        .into_iter()
        .filter_map(|path| {
            let frame = Frame::of_map(&path).unwrap()?;
            Some((path, frame))
        })
        .collect();
    let names: Vec<String> = maps
        .iter()
        .map(|(p, _)| p.file_name().unwrap().to_string_lossy().to_string())
        .collect();
    assert!(
        names.contains(&"south_pacific.csv".to_string())
            && names.contains(&"midway.csv".to_string())
            && names.contains(&"papua_to_midway.csv".to_string()),
        "{names:?}"
    );
    maps
}

/// The anchors file as the map's header names it, and as `generate` is run
/// from the repository root.
const ANCHORS_FILE: &str = "data/map/south_pacific_anchors.csv";

fn anchors_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../")
        .join(ANCHORS_FILE)
}

/// One raw data row of the file.
struct Row {
    q: i32,
    r: i32,
    kind: Kind,
    land_percent: u32,
    lat: String,
    lon: String,
    water_bodies: u32,
    closed: [bool; 6],
    closed_text: String,
}

fn parse_rows(text: &str) -> Vec<Row> {
    let mut lines = text.lines().filter(|l| !l.starts_with('#'));
    assert_eq!(lines.next(), Some(HEADER), "the column header line");
    lines
        .map(|line| {
            let fields: Vec<&str> = line.split(',').collect();
            let [q, r, kind, percent, lat, lon, bodies, closed] = fields[..] else {
                panic!("not eight fields: {line}");
            };
            let mut flags = [false; 6];
            if !closed.is_empty() {
                for part in closed.split('|') {
                    let d: usize = part.parse().expect("a direction");
                    assert!(d <= 5, "{line}");
                    assert!(!flags[d], "repeated direction: {line}");
                    flags[d] = true;
                }
            }
            Row {
                q: q.parse().unwrap(),
                r: r.parse().unwrap(),
                kind: match kind {
                    "land" => Kind::Land,
                    "coast" => Kind::Coast,
                    "deep_sea" => Kind::DeepSea,
                    other => panic!("unknown kind {other}"),
                },
                land_percent: percent.parse().unwrap(),
                lat: lat.to_string(),
                lon: lon.to_string(),
                water_bodies: bodies.parse().unwrap(),
                closed: flags,
                closed_text: closed.to_string(),
            }
        })
        .collect()
}

#[test]
fn every_committed_map_is_well_formed_and_complete() {
    for (path, frame) in generated_maps() {
        is_well_formed_and_complete(&path, &frame);
    }
    // The present map has some closed sides and some hexes with several
    // bodies, or it would not be doing its job. (The Midway map is open sea
    // and has neither.)
    let rows = parse_rows(&std::fs::read_to_string(map_path()).unwrap());
    assert!(rows.iter().any(|r| r.water_bodies > 1));
    assert!(rows
        .iter()
        .any(|r| r.kind == Kind::Coast && r.closed.iter().any(|c| *c)));
}

fn is_well_formed_and_complete(path: &std::path::Path, frame: &Frame) {
    let text = std::fs::read_to_string(path).unwrap();
    assert!(text.starts_with("# "), "the header comments come first");
    let projection = frame.projection();
    let rows = parse_rows(&text);

    // Sorted by r then q, which also means no duplicate hex.
    assert!(
        rows.windows(2).all(|w| (w[0].r, w[0].q) < (w[1].r, w[1].q)),
        "rows must be sorted by r then q, without duplicates"
    );

    // Exactly the hexes whose centres fall in the header's extent.
    let wanted = extent_hexes(frame);
    let found: Vec<(i32, i32)> = rows.iter().map(|row| (row.q, row.r)).collect();
    assert_eq!(found, wanted);
    let extent: BTreeSet<(i32, i32)> = wanted.into_iter().collect();

    for row in &rows {
        let place = format!("{} ({},{})", path.display(), row.q, row.r);
        // The kind agrees with the percentage.
        let agrees = match row.kind {
            Kind::Land => (80..=100).contains(&row.land_percent),
            Kind::Coast => row.land_percent <= 79,
            Kind::DeepSea => row.land_percent == 0,
        };
        assert!(agrees, "{place} {:?} {}", row.kind, row.land_percent);
        // Some land-point count of this hex's own point count gives this kind
        // and percentage.
        let points = hex_points(row.q, row.r).len() as u32;
        assert!(
            (0..=points).any(|n| classify(n, points) == (row.land_percent, row.kind)),
            "{place} {:?} {} is not reachable from any land count of {points} points",
            row.kind,
            row.land_percent
        );
        // The centre is where the header's projection puts it, its
        // longitude written west negative.
        let (lat, lon) = projection.centre_of(row.q, row.r);
        assert_eq!(row.lat, degrees_text(lat), "lat of {place}");
        assert_eq!(
            row.lon,
            degrees_text(frame.lon_for_file(lon)),
            "lon of {place}"
        );
        // The closed sides are written ascending, joined by |.
        assert_eq!(row.closed_text, closed_sides_text(&row.closed), "{place}");
        // A hex with no land has one body of water; one that is all land has
        // none.
        if row.kind == Kind::DeepSea {
            assert_eq!(row.water_bodies, 1, "{place}");
        }
        if row.land_percent == 100 {
            assert_eq!(row.water_bodies, 0, "{place}");
        }
        // A side toward a hex outside the extent is closed.
        for (d, (dq, dr)) in Hex::DIRECTIONS.iter().enumerate() {
            if !extent.contains(&(row.q + dq, row.r + dr)) {
                assert!(row.closed[d], "{place} direction {d} leads off the extent");
            }
        }
    }
    // Closed sides are symmetric: where both hexes are in the file, a side
    // closed in one is closed in the other.
    let closed: std::collections::BTreeMap<(i32, i32), [bool; 6]> = rows
        .iter()
        .map(|row| ((row.q, row.r), row.closed))
        .collect();
    for row in &rows {
        for (d, (dq, dr)) in Hex::DIRECTIONS.iter().enumerate() {
            if let Some(other) = closed.get(&(row.q + dq, row.r + dr)) {
                assert_eq!(
                    row.closed[d],
                    other[(d + 3) % 6],
                    "({},{}) direction {d}",
                    row.q,
                    row.r
                );
            }
        }
    }
}

#[test]
fn sim_data_loads_every_committed_map_and_agrees_with_the_text() {
    for (path, _) in generated_maps() {
        loads_and_agrees(&path);
    }
}

fn loads_and_agrees(path: &std::path::Path) {
    let text = std::fs::read_to_string(path).unwrap();
    let rows = parse_rows(&text);
    // The loader rejects a file whose closed sides are not symmetric.
    let map = sim_data::load_map(path).unwrap();
    assert_eq!(map.len(), rows.len());
    for row in &rows {
        let data = map.get(Hex::new(row.q, row.r)).unwrap();
        let expected = match row.kind {
            Kind::Land => HexKind::Land,
            Kind::Coast => HexKind::Coast,
            Kind::DeepSea => HexKind::DeepSea,
        };
        assert_eq!(data.kind, expected);
        assert_eq!(u32::from(data.land_percent), row.land_percent);
        assert_eq!(data.closed_sides, row.closed);
        assert!(data.lat.is_some() && data.lon.is_some());
    }
}

/// The header of the committed map says it was generated with the committed
/// anchors file: the file is named, the recorded command passes it, and
/// there is one line per anchorage, in the file's order, naming the hex the
/// anchorage is in (from its position in the anchors file) and what it did.
#[test]
fn the_committed_map_was_generated_with_the_committed_anchors_file() {
    let text = std::fs::read_to_string(map_path()).unwrap();
    let anchors = parse_anchors(&std::fs::read_to_string(anchors_path()).unwrap()).unwrap();
    assert!(!anchors.is_empty());
    let header: Vec<&str> = text
        .lines()
        .take_while(|l| l.starts_with('#'))
        .map(|l| l.strip_prefix('#').unwrap_or(l))
        .collect();

    let at = header
        .iter()
        .position(|l| l.starts_with(&format!(" Anchors: {ANCHORS_FILE}.")))
        .expect("the header names the anchors file");
    let projection = Frame::of_map(&map_path()).unwrap().unwrap().projection();
    for (k, anchor) in anchors.iter().enumerate() {
        let (q, r) = projection.hex_of(anchor.lat, anchor.lon);
        let line = header[at + 1 + k];
        let start = format!("   {}: hex ({q},{r}): ", anchor.place);
        let rest = line.strip_prefix(&start).unwrap_or_else(|| {
            panic!(
                "line for {}: \"{line}\", wanted it to start \"{start}\"",
                anchor.place
            )
        });
        let known = rest.starts_with("changed the main body of water, from ")
            || rest.starts_with("its body of water (")
            || rest.starts_with("ignored: its body of water has ");
        assert!(known, "{line}");
        if rest.starts_with("ignored") {
            assert!(
                rest.contains(&format!("fewer than {MIN_ANCHOR_BODY_POINTS};")),
                "{line}"
            );
        }
        // The anchorage is in a hex of the map that is not land.
        let row = text
            .lines()
            .find(|l| l.starts_with(&format!("{q},{r},")))
            .unwrap_or_else(|| panic!("hex ({q},{r}) of {} is not in the map", anchor.place));
        assert!(!row.starts_with(&format!("{q},{r},land,")), "{row}");
    }
    assert!(
        header[at + 1 + anchors.len()].starts_with(" closed_sides:"),
        "exactly one line per anchorage"
    );

    // The recorded command passes the anchors file.
    let generated = header
        .iter()
        .find(|l| l.starts_with(" Generated "))
        .expect("the generated line");
    assert!(
        generated.contains(&format!("--anchors {ANCHORS_FILE}")),
        "{generated}"
    );
}

// ----- The Midway map (decision 0199) -------------------------------------

fn header_of(path: &std::path::Path) -> Vec<String> {
    std::fs::read_to_string(path)
        .unwrap()
        .lines()
        .take_while(|l| l.starts_with('#'))
        .map(|l| l.strip_prefix("# ").unwrap_or(l).to_string())
        .collect()
}

fn frame_file() -> Frame {
    Frame::read(&root().join("data/map/frames/midway.toml")).unwrap()
}

/// The frame file the Midway map was made from and the map's header agree:
/// the title, the projection and extent lines, the date and the command that
/// names the frame file; and the frame `sim-data` reads from the header is
/// the file's, longitudes in the run. The factor is the cosine of the
/// centre's latitude to six places.
#[test]
fn the_midway_frame_file_and_the_header_agree() {
    let file = frame_file();
    let header = header_of(&midway_path());
    assert_eq!(header[0], file.title);
    for line in file.projection_lines() {
        assert!(header.contains(&line), "{line}");
    }
    assert!(header.contains(&file.extent_line()));
    let generated = header.last().unwrap();
    assert!(
        generated.starts_with(&format!("Generated {} by ", file.date)),
        "{generated}"
    );
    assert!(
        generated.contains("--frame data/map/frames/midway.toml --out data/map/midway.csv"),
        "{generated}"
    );
    let read = Frame::of_map(&midway_path()).unwrap().unwrap();
    assert_eq!(read.projection(), file.projection());
    assert_eq!(
        (
            read.west_run(),
            read.east_run(),
            read.south.value,
            read.north.value
        ),
        (
            file.west_run(),
            file.east_run(),
            file.south.value,
            file.north.value
        )
    );
    assert_eq!((file.west_run(), file.east_run()), (163.0, 194.0));
    let frame = sim_data::read_map_frame(&midway_path()).unwrap().unwrap();
    assert_eq!(frame.east.to_string(), "194");
    assert_eq!(frame.centre_lon.to_string(), "178.5");
    assert!(frame.crosses_date_line());
    // cos 28.5 degrees is 0.8788171...; the literal is it to six places.
    // The check is of a map of one factor (decision 0242, rule 16).
    let map_tool::frame::Kind::OneFactor(factor) = &file.kind else {
        panic!("the Midway frame is of one factor");
    };
    let cos = (28.5_f64).to_radians().cos();
    assert!((cos - factor.value).abs() < 0.0000005, "{cos}");
}

/// Each land mark of the Midway frame cites its row of
/// `data/oob/place_positions.csv` and gives that row's position exactly; its
/// hex is coast with land percentage 0 and is listed in the header's line
/// "Marked land:". Midway and Wake are the marks (rule 11).
#[test]
fn midway_and_wake_are_marked_land() {
    let file = frame_file();
    let names: Vec<&str> = file.marks.iter().map(|m| m.name.as_str()).collect();
    assert_eq!(names, ["midway", "wake"]);
    let places = std::fs::read_to_string(root().join("data/oob/place_positions.csv")).unwrap();
    let text = std::fs::read_to_string(midway_path()).unwrap();
    let rows = parse_rows(&text);
    let header = header_of(&midway_path());
    let marked = header
        .iter()
        .find(|l| l.starts_with("Marked land: "))
        .expect("the Marked land line");
    let projection = file.projection();
    for mark in &file.marks {
        let row = places
            .lines()
            .find(|l| l.starts_with(&format!("{},", mark.row)))
            .unwrap_or_else(|| panic!("row {} is not in place_positions.csv", mark.row));
        let fields: Vec<&str> = row.split(',').collect();
        assert_eq!(
            (fields[3], fields[4]),
            (mark.lat.text.as_str(), mark.lon.text.as_str()),
            "{row}"
        );
        assert!(
            fields[1].starts_with(&mark.name),
            "row {} is of {}",
            mark.row,
            fields[1]
        );
        let (q, r) = projection.hex_of(mark.lat.value, file.lon_in_run(mark.lon.value));
        let hex = rows
            .iter()
            .find(|h| (h.q, h.r) == (q, r))
            .unwrap_or_else(|| panic!("{} hex ({q},{r}) is not on the map", mark.name));
        assert_eq!(
            (hex.kind, hex.land_percent),
            (Kind::Coast, 0),
            "{}",
            mark.name
        );
        assert!(
            marked.contains(&format!(
                "{} (data/oob/place_positions.csv row {}) hex ({q},{r})",
                mark.name, mark.row
            )),
            "{marked}"
        );
    }
}

/// The Midway map loads through `sim-data`: its hexes as the header counts
/// them, every one of them navigable sea or coast, Midway's hex coast and
/// reachable by sea from Wake's, each hex's longitude west negative.
#[test]
fn sim_data_loads_the_midway_map() {
    let map = sim_data::load_map(&midway_path()).unwrap();
    let header = header_of(&midway_path());
    let counts = header
        .iter()
        .find(|l| l.starts_with("Hexes: "))
        .expect("the Hexes line");
    assert!(
        counts.starts_with(&format!("Hexes: {} (", map.len())),
        "{counts}"
    );
    assert!(map.iter().all(|(_, d)| d.kind != HexKind::Land));
    let file = frame_file();
    let projection = file.projection();
    let at = |mark: usize| {
        let m = &file.marks[mark];
        let (q, r) = projection.hex_of(m.lat.value, file.lon_in_run(m.lon.value));
        Hex::new(q, r)
    };
    let (midway, wake) = (at(0), at(1));
    assert_eq!(map.get(midway).unwrap().kind, HexKind::Coast);
    assert_eq!(map.get(wake).unwrap().kind, HexKind::Coast);
    assert!(map.sea_distance(wake, midway).is_some());
    let one_eighty = sim_core::fixed::Fixed::from_int(180);
    assert!(map.iter().all(|(_, d)| d
        .lon
        .is_some_and(|lon| lon <= one_eighty && lon >= -one_eighty)));
}
