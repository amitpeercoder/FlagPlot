//! The check of passages on a map about a centre (decision 0242, rule 15).
//!
//! # The hand calculation
//!
//! Written by the architect on Nov 28, 2026, before any code. Nobody else
//! changes a number here; a mismatch goes back to the architect.
//!
//! On the map from Papua to Midway (`data/map/papua_to_midway.csv`) the check
//! works no position into a hex. It reads each end's hex from the map's file
//! of located positions, `data/map/papua_to_midway_located.csv`, by the row of
//! `data/oob/place_positions.csv` the passage names.
//!
//! ## One passage, from Midway to Wake
//!
//! - From row 5100, Midway: the located file gives hex (31,-41).
//! - To row 5107, Wake: the located file gives hex (2,-26).
//! - A hex (q, r)'s centre is 40 x (q + r/2) miles east and 34.641016 x r
//!   miles south of hex (0,0)'s. So (31,-41) is at east 420.000, north
//!   1420.282, and (2,-26) at east -440.000, north 900.666.
//! - The two are 29 steps apart: q falls by 29 and r rises by 15, which is 29
//!   steps on a hex grid. Open sea lies between them, so the way is clear.
//! - Where the way is clear a ship is charged the straight line between the
//!   centres (decision 0058): the root of 860.000 squared and 519.616 squared,
//!   1004.79, charged as 1,005 miles.
//!
//! The check reports the passage with its ends as "row 5100, midway, (31,-41)"
//! and "row 5107, wake, (2,-26)" and its distance as 1,005 miles. What it says
//! of the time is what it says for 1,005 miles on any map.
//!
//! ## What stops it
//!
//! - An end given as a bare latitude and longitude, with no row: the check
//!   stops with an error that names the passage and the end and says that on
//!   a map about a centre an end is a row of the place positions file, since
//!   only the map tool works a position into a hex there.
//! - An end whose row is not in the located file: an error that names the
//!   row and the located file.
//! - A map about a centre with no located file beside it: an error that names
//!   the file wanted.
//!
//! ## What does not change
//!
//! On a map of one factor the check works in whole numbers as it did, reads
//! no located file, and reports what it reported: the existing tests of the
//! check hold without a change.

use std::fs;
use std::path::PathBuf;
use std::process::{Command, Output};

const ROOT: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../..");
const WIDE: &str = "data/map/papua_to_midway.csv";

fn binary(args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_sim-cli"))
        .args(args)
        .current_dir(ROOT)
        .output()
        .expect("the binary starts")
}

fn dir(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!(
        "flagplot-about-a-centre-{}-{name}",
        std::process::id()
    ));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).expect("make the temp dir");
    dir
}

fn passage(ends: &str) -> String {
    format!(
        "[[passage]]\nid = \"mw\"\nfacts = \"x\"\ncounts = \"C mw\"\n{ends}\nspeed_kn = 15\nrecord_low = 1\nrecord_high = 1\njudged = false\nrows = [1]\nreason = \"r\"\n"
    )
}

/// Runs the check on `map` with the passages `text`; the report goes in `d`.
fn run(d: &std::path::Path, map: &str, text: &str) -> (Output, String) {
    let file = d.join("passages.toml");
    fs::write(&file, text).unwrap();
    let out = d.join("report.md");
    let o = binary(&[
        "validate",
        "--passages",
        file.to_str().unwrap(),
        "--label",
        "t",
        "--map",
        map,
        "--out",
        out.to_str().unwrap(),
    ]);
    let report = fs::read_to_string(&out).unwrap_or_default();
    (o, report)
}

fn stderr(o: &Output) -> String {
    String::from_utf8_lossy(&o.stderr).into_owned()
}

#[test]
fn midway_to_wake_is_1005_miles_between_the_located_hexes() {
    let d = dir("midway-wake");
    let (o, report) = run(
        &d,
        WIDE,
        &passage("from_place = \"midway\"\nfrom_row = 5100\nto_place = \"wake\"\nto_row = 5107"),
    );
    assert!(o.status.success(), "{}", stderr(&o));
    let line = report
        .lines()
        .find(|l| l.starts_with("| mw |"))
        .expect("the passage's row");
    let cells: Vec<&str> = line.split('|').map(str::trim).collect();
    assert_eq!(cells[2], "row 5100, midway, (31,-41)");
    assert_eq!(cells[3], "row 5107, wake, (2,-26)");
    // Straight line, charged and counted: 1,005 miles each (decision 0058).
    assert_eq!(cells[4], "1005");
    assert_eq!(cells[5], "1005");
    assert_eq!(cells[7], "1005");
    assert!(line.ends_with("| reported |"), "{line}");
}

#[test]
fn a_bare_position_is_refused_on_a_map_about_a_centre() {
    let d = dir("bare");
    let (o, _) = run(
        &d,
        WIDE,
        &passage(
            "from_lat = \"28.232\"\nfrom_lon = \"-177.370\"\nto_place = \"wake\"\nto_row = 5107",
        ),
    );
    assert!(!o.status.success());
    let e = stderr(&o);
    assert!(e.contains("passage mw: from:"), "{e}");
    assert!(e.contains("is a row of the place positions file"), "{e}");
    assert!(
        e.contains("only the map tool works a position into a hex"),
        "{e}"
    );
}

#[test]
fn a_row_not_in_the_located_file_is_refused_naming_the_row_and_the_file() {
    let d = dir("missing-row");
    // Row 1 is Rabaul, inside the map; row 9999 is in a places file of our own
    // but not in the located file.
    let places = d.join("places.csv");
    fs::write(
        &places,
        "id,place,feature,lat,lon,datum\n9999,nowhere,x,1.0,170.0,WGS84\n5107,wake,x,19.299,166.624,WGS84\n",
    )
    .unwrap();
    let file = d.join("passages.toml");
    fs::write(
        &file,
        passage("from_place = \"nowhere\"\nfrom_row = 9999\nto_place = \"wake\"\nto_row = 5107"),
    )
    .unwrap();
    let o = binary(&[
        "validate",
        "--passages",
        file.to_str().unwrap(),
        "--label",
        "t",
        "--map",
        WIDE,
        "--places",
        places.to_str().unwrap(),
        "--out",
        d.join("r.md").to_str().unwrap(),
    ]);
    assert!(!o.status.success());
    let e = stderr(&o);
    assert!(e.contains("row 9999"), "{e}");
    assert!(e.contains("papua_to_midway_located.csv"), "{e}");
}

#[test]
fn a_map_about_a_centre_with_no_located_file_is_refused_naming_the_file() {
    let d = dir("no-located");
    // A copy of the map with no located file beside it.
    let map = d.join("lonely.csv");
    fs::copy(format!("{ROOT}/{WIDE}"), &map).unwrap();
    let (o, _) = run(
        &d,
        map.to_str().unwrap(),
        &passage("from_place = \"midway\"\nfrom_row = 5100\nto_place = \"wake\"\nto_row = 5107"),
    );
    assert!(!o.status.success());
    let e = stderr(&o);
    assert!(e.contains("lonely_located.csv"), "{e}");
    assert!(e.contains("about a centre"), "{e}");
}

#[test]
fn a_map_of_one_factor_reads_no_located_file() {
    let d = dir("one-factor");
    // The present map has a frame of one factor and no located file; a bare
    // position works there as it always has, to the same hexes.
    let (o, report) = run(
        &d,
        "data/map/south_pacific.csv",
        &passage(
            "from_lat = \"-12.0\"\nfrom_lon = \"160.0\"\nto_lat = \"-12.0\"\nto_lon = \"162.0447\"",
        ),
    );
    assert!(o.status.success(), "{}", stderr(&o));
    assert!(report.contains("position -12.0, 160.0, (0,0)"), "{report}");
    assert!(report.contains("(3,0)"), "{report}");
    assert!(!d.join("south_pacific_located.csv").exists());
}
