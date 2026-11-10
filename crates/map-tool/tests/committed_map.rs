//! Checks on the committed map, data/map/south_pacific.csv. It needs nothing
//! from references/. The map is loaded through `sim_data::load_map`, which
//! checks the symmetry of the closed sides; the raw text is parsed here to
//! check the rest.

use std::collections::BTreeSet;
use std::path::PathBuf;

use map_tool::generate::{
    classify, closed_sides_text, extent_hexes, hex_points, parse_anchors, Kind,
    MIN_ANCHOR_BODY_POINTS,
};
use map_tool::projection::{centre_of, degrees_text, hex_of};
use sim_core::hex::Hex;
use sim_core::map::HexKind;

const HEADER: &str = "q,r,kind,land_percent,lat,lon,water_bodies,closed_sides";

fn map_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../data/map/south_pacific.csv")
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
fn the_committed_map_is_well_formed_and_complete() {
    let text = std::fs::read_to_string(map_path()).unwrap();
    assert!(text.starts_with("# "), "the header comments come first");
    let rows = parse_rows(&text);

    // Sorted by r then q, which also means no duplicate hex.
    assert!(
        rows.windows(2).all(|w| (w[0].r, w[0].q) < (w[1].r, w[1].q)),
        "rows must be sorted by r then q, without duplicates"
    );

    // Exactly the hexes whose centres fall in the extent.
    let wanted = extent_hexes();
    let found: Vec<(i32, i32)> = rows.iter().map(|row| (row.q, row.r)).collect();
    assert_eq!(found, wanted);
    let extent: BTreeSet<(i32, i32)> = wanted.into_iter().collect();

    for row in &rows {
        let place = format!("({},{})", row.q, row.r);
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
        // The centre is the projection's.
        let (lat, lon) = centre_of(row.q, row.r);
        assert_eq!(row.lat, degrees_text(lat), "lat of {place}");
        assert_eq!(row.lon, degrees_text(lon), "lon of {place}");
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
    // The map has some closed sides and some hexes with several bodies, or
    // it would not be doing its job.
    assert!(rows.iter().any(|r| r.water_bodies > 1));
    assert!(rows
        .iter()
        .any(|r| r.kind == Kind::Coast && r.closed.iter().any(|c| *c)));
}

#[test]
fn sim_data_loads_the_committed_map_and_agrees_with_the_text() {
    let text = std::fs::read_to_string(map_path()).unwrap();
    let rows = parse_rows(&text);
    // The loader rejects a file whose closed sides are not symmetric.
    let map = sim_data::load_map(&map_path()).unwrap();
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
    for (k, anchor) in anchors.iter().enumerate() {
        let (q, r) = hex_of(anchor.lat, anchor.lon);
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
