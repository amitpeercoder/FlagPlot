//! Integration tests of `sim-cli validate --passages` (`docs/step2-spec.md`,
//! "Measures"; `data/validation/README.md`, "Passages").
//!
//! # The architect's hand calculation
//!
//! The fixture is a map of open sea (hexes q -5 to 10, r -5 to 5, and one
//! lone sea hex at (-20,0) that nothing joins), with hex (8,-2) land; a place
//! positions file with row 1 at the centre of hex (0,0) (12 S, 160 E by
//! decision 0017) and row 2 at the centre of hex (3,0); and a passages file.
//! All run from row 1 to row 2 unless said. The way is clear along the row
//! `r = 0`, an axis, so a step is 40 miles and the three steps of (0,0) to
//! (3,0) are charged 120 miles (decision 0058).
//!
//! - P1: 30 knots. 120 x 60 / 30 = 240 minutes.
//! - P2: 30 knots, `speed_changes = [[60, 15]]`. The first 60 minutes cover
//!   30 miles; the other 90 at 15 knots take 360; 420 minutes.
//! - P3: P2 with `to_short_by_nm = 10`. 110 miles: 30 in the first 60
//!   minutes, 80 at 15 knots in 320; 380 minutes.
//! - P4: 28 knots. 120 x 60 / 28 = 257.14, rounded up: 258 minutes.
//! - P5: 30 knots, `speed_changes = [[240, 10]]`. The 120 miles are reached
//!   at minute 240 exactly, as the change comes: 240 minutes.
//! - P6: 30 knots, `speed_changes = [[30, 0], [90, 30]]` (stopped for an
//!   hour). 15 miles in 30 minutes, none until minute 90, then 105 miles at
//!   30 knots in 210: 300 minutes.
//! - P7: to a position inside hex (3,0) but not at its centre (11.9 S, 162.2
//!   E, 9 nm from the centre of the hex): the same as P1, 240 minutes, since a
//!   place is at the centre of its hex.
//! - P8 (the architect approves these numbers; they are the developer's
//!   until then): row 1 to row 3, at the centre of hex (3,1), one hex off
//!   the row. The hexes are 4 steps apart and the way is clear. The charge is
//!   the straight-line miles between the centres by decision 0058: dq = 3,
//!   dr = 1, n = dq^2 + dr^2 + dq x dr = 9 + 1 + 3 = 13; 40^2 x 13 = 20800;
//!   sqrt(20800) = 144.22, so 144 miles to the nearest mile. At 25 knots,
//!   144 x 60 / 25 = 345.6, rounded up: 346 minutes.
//! - With a record of 250 to 260, P1's calculated less record is -10; P4 with
//!   a record of 255 to 260 is 0 (258 is inside it); P1 with a record of 200
//!   is +40 (the passage `p1_late`).
//! - A route that cannot be worked is reported and the rest still are: an end
//!   on land (row 6, hex (8,-2)); a hex with no sea route (row 7, the lone
//!   hex (-20,0)); a hex off the map (a position at 40 S).
//!
//! The position table (positions to hexes, from `map-tool locate`) is a unit
//! test in `validate/passages.rs`.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

const ROOT: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../..");

fn binary(args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_sim-cli"))
        .args(args)
        .current_dir(ROOT)
        .output()
        .expect("the binary starts")
}

fn dir(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("flagplot-passages-{}-{name}", std::process::id()));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).expect("make the temp dir");
    dir
}

fn text(p: &Path) -> String {
    p.to_string_lossy().into_owned()
}

fn stderr(o: &Output) -> String {
    String::from_utf8_lossy(&o.stderr).into_owned()
}

const MAP: &str = r#"
placeholder = true

[[region]]
kind = "deep_sea"
q = [-5, 10]
r = [-5, 5]

[[region]]
kind = "deep_sea"
q = [-20, -20]
r = [0, 0]

[[hex]]
q = 8
r = -2
kind = "land"
"#;

const PLACES: &str = "\
# Place positions of the fixture.
id,place,feature,lat,lon,datum
1,alpha,the first,-12.0,160.0,WGS84
2,bravo,the second,-12.0,162.0447,WGS84
3,carol,off the row,-12.5774,162.3855,WGS84
4,dud,no position,,,WGS84
6,hill,on land,-10.8453,164.7710,WGS84
7,isle,a lone hex,-12.0,146.3693,WGS84
";

/// The passages file of the hand calculation, as a string.
fn fixture_passages() -> String {
    let mut s = String::new();
    let one = |id: &str, record: (u32, u32), extra: &str| {
        format!(
            "[[passage]]\nid = \"{id}\"\nfacts = \"x\"\ncounts = \"C {id}\"\nfrom_place = \"alpha\"\nfrom_row = 1\nto_place = \"bravo\"\nto_row = 2\nrecord_low = {}\nrecord_high = {}\njudged = false\nrows = [7, 8]\nreason = \"r\"\n{extra}\n",
            record.0, record.1
        )
    };
    s += &one("p1", (250, 260), "speed_kn = 30");
    s += &one("p2", (1, 1), "speed_kn = 30\nspeed_changes = [[60, 15]]");
    s += &one(
        "p3",
        (1, 1),
        "speed_kn = 30\nspeed_changes = [[60, 15]]\nto_short_by_nm = 10",
    );
    s += &one("p4", (255, 260), "speed_kn = 28");
    s += &one("p5", (1, 1), "speed_kn = 30\nspeed_changes = [[240, 10]]");
    s += &one(
        "p6",
        (1, 1),
        "speed_kn = 30\nspeed_changes = [[30, 0], [90, 30]]",
    );
    s += "[[passage]]\nid = \"p7\"\nfacts = \"x\"\ncounts = \"C p7\"\nfrom_place = \"alpha\"\nfrom_row = 1\nto_lat = \"-11.9\"\nto_lon = \"162.2\"\nspeed_kn = 30\nrecord_low = 1\nrecord_high = 1\njudged = false\nrows = [7]\nreason = \"r\"\n";
    s += "[[passage]]\nid = \"p8\"\nfacts = \"x\"\ncounts = \"C p8\"\nfrom_place = \"alpha\"\nfrom_row = 1\nto_place = \"carol\"\nto_row = 3\nspeed_kn = 25\nrecord_low = 300\nrecord_high = 300\njudged = false\nrows = [7]\nreason = \"r\"\n";
    s += &one("p1_late", (200, 200), "speed_kn = 30");
    // The ends that cannot be worked.
    s += "[[passage]]\nid = \"to_land\"\nfacts = \"x\"\ncounts = \"C\"\nfrom_place = \"alpha\"\nfrom_row = 1\nto_place = \"hill\"\nto_row = 6\nspeed_kn = 30\nrecord_low = 1\nrecord_high = 1\njudged = false\nrows = [7]\nreason = \"r\"\n";
    s += "[[passage]]\nid = \"no_route\"\nfacts = \"x\"\ncounts = \"C\"\nfrom_place = \"alpha\"\nfrom_row = 1\nto_place = \"isle\"\nto_row = 7\nspeed_kn = 30\nrecord_low = 1\nrecord_high = 1\njudged = false\nrows = [7]\nreason = \"r\"\n";
    s += "[[passage]]\nid = \"off_map\"\nfacts = \"x\"\ncounts = \"C\"\nfrom_place = \"alpha\"\nfrom_row = 1\nto_lat = \"-40.0\"\nto_lon = \"160.0\"\nspeed_kn = 30\nrecord_low = 1\nrecord_high = 1\njudged = false\nrows = [7]\nreason = \"r\"\n";
    s
}

struct Fixture {
    dir: PathBuf,
    passages: PathBuf,
    map: PathBuf,
    places: PathBuf,
}

fn fixture(name: &str, passages: &str, places: &str) -> Fixture {
    let dir = dir(name);
    let f = Fixture {
        passages: dir.join("passages.toml"),
        map: dir.join("map.toml"),
        places: dir.join("places.csv"),
        dir,
    };
    fs::write(&f.passages, passages).expect("write");
    fs::write(&f.map, MAP).expect("write");
    fs::write(&f.places, places).expect("write");
    f
}

fn run(f: &Fixture, label: &str, out: &str) -> Output {
    binary(&[
        "validate",
        "--passages",
        &text(&f.passages),
        "--label",
        label,
        "--map",
        &text(&f.map),
        "--places",
        &text(&f.places),
        "--out",
        &text(&f.dir.join(out)),
    ])
}

fn cells(report: &str, id: &str) -> Vec<String> {
    let line = report
        .lines()
        .find(|l| l.starts_with(&format!("| {id} |")))
        .unwrap_or_else(|| panic!("no row for {id}"));
    line.trim_matches('|')
        .split('|')
        .map(|c| c.trim().to_string())
        .collect()
}

/// Columns: 0 id, 1 from, 2 to, 3 straight line, 4 charged, 5 short by, 6
/// miles counted, 7 speed, 8 calculated, 9 record, 10 calculated less record,
/// 11 status.
fn worked(report: &str, id: &str, charged: &str, minutes: &str, delta: &str) {
    let c = cells(report, id);
    assert_eq!(c[4], charged, "{id} charged");
    assert_eq!(c[8], minutes, "{id} minutes");
    assert_eq!(c[10], delta, "{id} less the record");
    assert_eq!(c[11], "reported", "{id} status");
}

#[test]
fn the_hand_calculation_of_every_passage() {
    let f = fixture("hand", &fixture_passages(), PLACES);
    let result = run(&f, "hand", "report.md");
    assert!(result.status.success(), "{}", stderr(&result));
    let report = fs::read_to_string(f.dir.join("report.md")).expect("report");
    worked(&report, "p1", "120", "240", "-10");
    worked(&report, "p2", "120", "420", "+419");
    worked(&report, "p3", "120", "380", "+379");
    assert_eq!(cells(&report, "p3")[5], "10");
    assert_eq!(cells(&report, "p3")[6], "110");
    worked(&report, "p4", "120", "258", "+0");
    worked(&report, "p5", "120", "240", "+239");
    worked(&report, "p6", "120", "300", "+299");
    worked(&report, "p7", "120", "240", "+239");
    assert!(cells(&report, "p7")[2].contains("(3,0)"));
    worked(&report, "p8", "144", "346", "+46");
    let p8 = cells(&report, "p8");
    assert!(p8[2].contains("(3,1)"), "{}", p8[2]);
    assert_eq!(p8[3], "144");
    assert_eq!(p8[6], "144");
    worked(&report, "p1_late", "120", "240", "+40");
    // The head names the files with their hashes, and the label.
    assert!(report.contains("Passages: "));
    assert!(report.contains("Map: "));
    assert!(report.contains("Place positions: "));
    assert!(report.contains("Label: `hand`"));
    assert!(report.contains("No passage is judged"));
    // Each passage's counts text and rows are under the table.
    assert!(report.contains("### p1\n\n- Counts: C p1\n- Rows: 7, 8\n"));
    // The first row and the speeds in words.
    assert_eq!(cells(&report, "p1")[1], "row 1, alpha, (0,0)");
    assert_eq!(
        cells(&report, "p6")[7],
        "30 knots; stopped from minute 30; 30 knots from minute 90"
    );
}

#[test]
fn a_route_that_cannot_be_worked_is_reported_and_the_rest_are_still_reported() {
    let f = fixture("unworkable", &fixture_passages(), PLACES);
    let result = run(&f, "x", "report.md");
    assert!(result.status.success(), "{}", stderr(&result));
    let report = fs::read_to_string(f.dir.join("report.md")).expect("report");
    let land = cells(&report, "to_land");
    assert!(
        land[11].contains("not worked") && land[11].contains("land"),
        "{land:?}"
    );
    let lone = cells(&report, "no_route");
    assert!(lone[11].contains("no sea route"), "{lone:?}");
    let off = cells(&report, "off_map");
    assert!(off[11].contains("off the map"), "{off:?}");
    worked(&report, "p1", "120", "240", "-10");
}

#[test]
fn the_same_inputs_give_the_same_bytes_and_a_report_is_not_written_over() {
    let f = fixture("bytes", &fixture_passages(), PLACES);
    assert!(run(&f, "x", "one.md").status.success());
    assert!(run(&f, "x", "two.md").status.success());
    let one = fs::read(f.dir.join("one.md")).expect("one");
    assert_eq!(one, fs::read(f.dir.join("two.md")).expect("two"));
    let again = run(&f, "x", "one.md");
    assert!(again.status.success());
    assert!(String::from_utf8_lossy(&again.stdout).starts_with("unchanged"));
    fs::write(f.dir.join("one.md"), "something else").expect("write");
    let refused = run(&f, "x", "one.md");
    assert!(!refused.status.success());
    assert!(stderr(&refused).contains("different contents"));
    assert_eq!(
        fs::read_to_string(f.dir.join("one.md")).expect("read"),
        "something else"
    );
}

#[test]
fn what_validate_passages_refuses() {
    let refuse = |name: &str, passages: &str, places: &str, label: Option<&str>, want: &str| {
        let f = fixture(name, passages, places);
        let mut args = vec![
            "validate".to_string(),
            "--passages".to_string(),
            text(&f.passages),
            "--map".to_string(),
            text(&f.map),
            "--places".to_string(),
            text(&f.places),
            "--out".to_string(),
            text(&f.dir.join("r.md")),
        ];
        if let Some(label) = label {
            args.push("--label".to_string());
            args.push(label.to_string());
        }
        let refs: Vec<&str> = args.iter().map(String::as_str).collect();
        let result = binary(&refs);
        assert!(!result.status.success(), "{name} should be refused");
        assert!(
            stderr(&result).contains(want),
            "{name}: {}",
            stderr(&result)
        );
        assert!(!f.dir.join("r.md").exists(), "{name} wrote a report");
    };
    let good = fixture_passages();
    refuse("no-label", &good, PLACES, None, "needs --label");
    refuse(
        "bad-label",
        &good,
        PLACES,
        Some("a/b"),
        "not letters, digits",
    );
    refuse(
        "empty-label",
        &good,
        PLACES,
        Some(""),
        "not letters, digits",
    );
    // A row id that is not in the file names the passage.
    let missing = good.replacen("to_row = 2", "to_row = 99", 1);
    refuse(
        "missing-row",
        &missing,
        PLACES,
        Some("x"),
        "passage p1: to: row 99 is not in",
    );
    // A row with no position names the passage.
    let empty = good.replacen("to_row = 2", "to_row = 4", 1);
    refuse(
        "no-position",
        &empty,
        PLACES,
        Some("x"),
        "passage p1: to: row 4 has no position",
    );
    // A row of another place than the one named.
    let wrong = good.replacen("from_place = \"alpha\"", "from_place = \"bravo\"", 1);
    refuse(
        "wrong-place",
        &wrong,
        PLACES,
        Some("x"),
        "passage p1: from: row 1 is of the place alpha",
    );
    // Both a row and a position, and neither.
    let both = good.replacen(
        "to_row = 2",
        "to_row = 2\nto_lat = \"-12\"\nto_lon = \"160\"",
        1,
    );
    refuse(
        "both",
        &both,
        PLACES,
        Some("x"),
        "both a row and a position",
    );
    let neither = good
        .replacen("to_row = 2\n", "", 1)
        .replacen("to_place = \"bravo\"\n", "", 1);
    refuse(
        "neither",
        &neither,
        PLACES,
        Some("x"),
        "neither a row nor a position",
    );
    // Speed changes must rise.
    let falling = good.replacen(
        "speed_kn = 30\nspeed_changes = [[60, 15]]",
        "speed_kn = 30\nspeed_changes = [[60, 15], [60, 10]]",
        1,
    );
    refuse("falling", &falling, PLACES, Some("x"), "must rise");
}

#[test]
fn the_real_passages_are_reported_every_one() {
    let d = dir("real");
    let out = d.join("passages-real.md");
    let result = binary(&[
        "validate",
        "--passages",
        "data/validation/passages.toml",
        "--label",
        "test",
        "--out",
        &text(&out),
    ]);
    assert!(result.status.success(), "{}", stderr(&result));
    let report = fs::read_to_string(&out).expect("report");
    let file = fs::read_to_string(Path::new(ROOT).join("data/validation/passages.toml"))
        .expect("passages");
    let mut ids = 0;
    for line in file.lines().filter(|l| l.starts_with("id = ")) {
        let id = line.trim_start_matches("id = ").trim_matches('"');
        assert!(report.contains(&format!("| {id} |")), "{id} missing");
        ids += 1;
    }
    assert_eq!(ids, 6);
    assert!(report.contains("No passage is judged"));
    assert!(report.contains("Place positions: data/oob/place_positions.csv"));
}
