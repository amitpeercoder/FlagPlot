//! The map from Papua to Midway (decision 0242, step 2): the committed map,
//! its marks and its file of located positions, checked against the
//! architect's figures.
//!
//! # The hand calculation
//!
//! Written by the architect on Nov 28, 2026, before any code. Nobody else
//! changes a number here; a mismatch goes back to the architect.
//!
//! ## The map
//!
//! `data/map/papua_to_midway.csv` is made from `data/map/frames/papua_to_midway.toml`
//! and the anchors file `data/map/papua_to_midway_anchors.csv`. Its header's
//! projection and extent lines are those of part 4 of
//! `tests/about_a_centre.rs`, and its hexes are the 10,395 of that file's
//! part 3, in rows -60 to 58, each centre where the projection puts it.
//!
//! ## The marks
//!
//! Two land marks, each a place with a base whose hex the sampling finds no
//! land in. By part 1 of `tests/about_a_centre.rs`:
//!
//! - Midway, row 5100 (28.232, -177.370), is in hex (31,-41).
//! - Wake, row 5107 (19.299, 166.624), is in hex (2,-26).
//!
//! Each of the two hexes is coast with land percentage 0, has one body of
//! water, and is named in the header's "Marked land" line. Neither is counted
//! among the hexes that have a land point and 0 percent.
//!
//! ## The counts by kind (the tool's own, not worked by hand)
//!
//! The sampling's result cannot be worked by hand. The architect's own run
//! of the tool on this frame without marks or anchors gave 344 land, 363
//! coast and 9,688 deep sea. With the two marks that is 344 land, 365 coast
//! and 9,686 deep sea; the anchors change which water a hex stands for and
//! never its kind. The test holds the map to those three numbers. If the
//! tool gives others, that is reported, not mended here.
//!
//! ## The file of located positions
//!
//! `data/map/papua_to_midway_located.csv` has one line for every row of
//! `data/oob/place_positions.csv` whose position is inside the extent, in the
//! order of the rows' ids: `row,place,lat,lon,q,r`, with `lat` and `lon` as
//! the row gives them and (q, r) the hex whose centre is nearest by part 1 of
//! `tests/about_a_centre.rs`. A row outside the extent has no line. On
//! Nov 28, 2026 the file of positions has 113 rows and every one is inside.
//!
//! Worked by the architect apart from the tool, each row's hex:
//!
//! 113 rows. The least margin between a row's nearest centre and its next nearest is 0.11 nm.
//! 1 rabaul (-39,14); 2 rabaul (-39,14); 3 rabaul (-39,13)
//! 4 tulagi (-32,22); 5 tulagi (-32,22); 6 lunga-point (-32,22)
//! 7 lunga-point (-32,22); 8 henderson-field (-32,22); 9 henderson-field (-32,22)
//! 10 savo-island (-32,22); 11 savo-island (-32,22); 12 espiritu-santo (-27,33)
//! 13 espiritu-santo (-27,33); 14 espiritu-santo (-27,33); 15 espiritu-santo (-27,32)
//! 16 noumea (-33,44); 17 noumea (-33,44); 18 kavieng (-40,11)
//! 19 kavieng (-40,11); 20 buka (-37,16); 21 buka (-37,16)
//! 22 buin (-36,18); 23 buin (-36,18); 24 shortland-harbor (-36,18)
//! 25 shortland-harbor (-36,18); 26 rekata-bay (-32,19); 27 rekata-bay (-32,19)
//! 28 gizo (-35,20); 29 gizo (-35,20); 30 munda (-35,21)
//! 31 munda (-35,21); 32 cape-esperance (-32,22); 33 cape-esperance (-32,22)
//! 34 ndeni-graciosa-bay (-25,25); 35 ndeni-graciosa-bay (-25,25); 36 efate (-27,36)
//! 37 efate (-27,36); 38 efate (-27,36); 39 port-moresby (-51,23)
//! 40 port-moresby (-51,23); 41 milne-bay (-47,24); 42 milne-bay (-47,24)
//! 43 milne-bay (-46,24); 44 lae (-49,18); 45 buna (-48,21)
//! 46 truk (-29,-7); 47 truk (-30,-6); 48 henderson-field (-32,22)
//! 49 rabaul-vunakanau (-39,14); 50 rabaul-lakunai (-39,14); 51 tassafaronga (-32,22)
//! 52 taivu-point (-31,22); 53 tasimboko (-32,23); 54 koli-point (-32,23)
//! 55 tenaru-river (-32,22); 56 ilu-river (-32,22); 57 bloody-ridge (-32,22)
//! 58 matanikau-river (-32,22); 59 point-cruz (-32,22); 60 kokumbona (-32,22)
//! 61 aola-bay (-32,23); 62 kamimbo (-32,22); 63 haleta (-32,22)
//! 64 halavo (-32,22); 65 metapona-river (-32,23); 66 kokumbona (-32,22)
//! 67 kukum (-32,22); 68 gavutu (-32,22); 69 tetere (-32,23)
//! 70 koli-point (-32,22); 71 tulagi (-32,22); 72 taivu-point (-31,22)
//! 73 kamimbo (-32,22); 74 gavutu (-32,22); 75 tanambogo (-32,22)
//! 76 kukum (-32,22); 77 halavo (-32,22); 78 haleta (-32,22)
//! 79 tetere (-32,23); 80 metapona-river (-32,23); 81 maravovo (-32,22)
//! 82 maravovo (-32,22); 83 visale (-32,22); 84 koilotumaria (-32,23)
//! 85 gurabusu (-32,23); 86 kokumbona (-32,22); 87 tassafaronga (-32,22)
//! 88 taivu-point (-31,22); 89 tasimboko (-32,23); 90 red-beach (-32,22)
//! 91 tenaru-river (-32,22); 92 lunga-point (-32,22); 93 point-cruz (-32,22)
//! 94 aola-bay (-32,23); 95 deboyne (-44,25); 96 misima (-44,25)
//! 97 rossel-island (-42,26); 98 townsville (-59,40); 99 point-zed (-41,30)
//! 5100 midway (31,-41); 5101 midway (31,-41); 5102 midway-sand-island (31,-41)
//! 5103 midway-sand-island (31,-41); 5104 midway-eastern-island (31,-41); 5105 midway-eastern-island (31,-41)
//! 5106 kure-atoll (30,-41); 5107 wake (2,-26); 5108 saipan (-30,-20)
//! 5109 guam (-33,-18); 5110 french-frigate-shoals (43,-34); 5111 laysan (37,-37)
//! 5112 lisianski (34,-37); 5113 point-luck (39,-47)
//!
//! Two of these are land hexes on the map (row 44, Lae town; row 98,
//! Townsville city). That is decision 0242's rule 9 and is not mended here:
//! the file says where a position falls, not where a port is.

use std::path::{Path, PathBuf};

use map_tool::frame::Frame;
use map_tool::generate::extent_hexes;
use map_tool::located::{located_text, LOCATED_HEADER};

const MAP: &str = "data/map/papua_to_midway.csv";
const LOCATED: &str = "data/map/papua_to_midway_located.csv";
const POSITIONS: &str = "data/oob/place_positions.csv";
const ANCHORS: &str = "data/map/papua_to_midway_anchors.csv";
const FRAME_FILE: &str = "data/map/frames/papua_to_midway.toml";

/// Each row of the located file as the hand calculation gives it: the row's id,
/// its place, and its hex (q, r).
const HAND: [(u32, &str, i32, i32); 113] = [
    (1, "rabaul", -39, 14),
    (2, "rabaul", -39, 14),
    (3, "rabaul", -39, 13),
    (4, "tulagi", -32, 22),
    (5, "tulagi", -32, 22),
    (6, "lunga-point", -32, 22),
    (7, "lunga-point", -32, 22),
    (8, "henderson-field", -32, 22),
    (9, "henderson-field", -32, 22),
    (10, "savo-island", -32, 22),
    (11, "savo-island", -32, 22),
    (12, "espiritu-santo", -27, 33),
    (13, "espiritu-santo", -27, 33),
    (14, "espiritu-santo", -27, 33),
    (15, "espiritu-santo", -27, 32),
    (16, "noumea", -33, 44),
    (17, "noumea", -33, 44),
    (18, "kavieng", -40, 11),
    (19, "kavieng", -40, 11),
    (20, "buka", -37, 16),
    (21, "buka", -37, 16),
    (22, "buin", -36, 18),
    (23, "buin", -36, 18),
    (24, "shortland-harbor", -36, 18),
    (25, "shortland-harbor", -36, 18),
    (26, "rekata-bay", -32, 19),
    (27, "rekata-bay", -32, 19),
    (28, "gizo", -35, 20),
    (29, "gizo", -35, 20),
    (30, "munda", -35, 21),
    (31, "munda", -35, 21),
    (32, "cape-esperance", -32, 22),
    (33, "cape-esperance", -32, 22),
    (34, "ndeni-graciosa-bay", -25, 25),
    (35, "ndeni-graciosa-bay", -25, 25),
    (36, "efate", -27, 36),
    (37, "efate", -27, 36),
    (38, "efate", -27, 36),
    (39, "port-moresby", -51, 23),
    (40, "port-moresby", -51, 23),
    (41, "milne-bay", -47, 24),
    (42, "milne-bay", -47, 24),
    (43, "milne-bay", -46, 24),
    (44, "lae", -49, 18),
    (45, "buna", -48, 21),
    (46, "truk", -29, -7),
    (47, "truk", -30, -6),
    (48, "henderson-field", -32, 22),
    (49, "rabaul-vunakanau", -39, 14),
    (50, "rabaul-lakunai", -39, 14),
    (51, "tassafaronga", -32, 22),
    (52, "taivu-point", -31, 22),
    (53, "tasimboko", -32, 23),
    (54, "koli-point", -32, 23),
    (55, "tenaru-river", -32, 22),
    (56, "ilu-river", -32, 22),
    (57, "bloody-ridge", -32, 22),
    (58, "matanikau-river", -32, 22),
    (59, "point-cruz", -32, 22),
    (60, "kokumbona", -32, 22),
    (61, "aola-bay", -32, 23),
    (62, "kamimbo", -32, 22),
    (63, "haleta", -32, 22),
    (64, "halavo", -32, 22),
    (65, "metapona-river", -32, 23),
    (66, "kokumbona", -32, 22),
    (67, "kukum", -32, 22),
    (68, "gavutu", -32, 22),
    (69, "tetere", -32, 23),
    (70, "koli-point", -32, 22),
    (71, "tulagi", -32, 22),
    (72, "taivu-point", -31, 22),
    (73, "kamimbo", -32, 22),
    (74, "gavutu", -32, 22),
    (75, "tanambogo", -32, 22),
    (76, "kukum", -32, 22),
    (77, "halavo", -32, 22),
    (78, "haleta", -32, 22),
    (79, "tetere", -32, 23),
    (80, "metapona-river", -32, 23),
    (81, "maravovo", -32, 22),
    (82, "maravovo", -32, 22),
    (83, "visale", -32, 22),
    (84, "koilotumaria", -32, 23),
    (85, "gurabusu", -32, 23),
    (86, "kokumbona", -32, 22),
    (87, "tassafaronga", -32, 22),
    (88, "taivu-point", -31, 22),
    (89, "tasimboko", -32, 23),
    (90, "red-beach", -32, 22),
    (91, "tenaru-river", -32, 22),
    (92, "lunga-point", -32, 22),
    (93, "point-cruz", -32, 22),
    (94, "aola-bay", -32, 23),
    (95, "deboyne", -44, 25),
    (96, "misima", -44, 25),
    (97, "rossel-island", -42, 26),
    (98, "townsville", -59, 40),
    (99, "point-zed", -41, 30),
    (5100, "midway", 31, -41),
    (5101, "midway", 31, -41),
    (5102, "midway-sand-island", 31, -41),
    (5103, "midway-sand-island", 31, -41),
    (5104, "midway-eastern-island", 31, -41),
    (5105, "midway-eastern-island", 31, -41),
    (5106, "kure-atoll", 30, -41),
    (5107, "wake", 2, -26),
    (5108, "saipan", -30, -20),
    (5109, "guam", -33, -18),
    (5110, "french-frigate-shoals", 43, -34),
    (5111, "laysan", 37, -37),
    (5112, "lisianski", 34, -37),
    (5113, "point-luck", 39, -47),
];

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../")
}

fn read(path: &str) -> String {
    std::fs::read_to_string(root().join(path)).unwrap()
}

/// The header's comment lines, without the leading "# ".
fn header_of(path: &str) -> Vec<String> {
    read(path)
        .lines()
        .take_while(|l| l.starts_with('#'))
        .map(|l| l.strip_prefix("# ").unwrap_or(l).to_string())
        .collect()
}

/// One data row of the map: q, r, kind, land percent, water bodies.
struct Hex {
    q: i32,
    r: i32,
    kind: String,
    percent: u32,
    bodies: u32,
}

fn hexes() -> Vec<Hex> {
    let text = read(MAP);
    let mut lines = text.lines().filter(|l| !l.starts_with('#'));
    assert_eq!(
        lines.next(),
        Some("q,r,kind,land_percent,lat,lon,water_bodies,closed_sides")
    );
    lines
        .map(|line| {
            let f: Vec<&str> = line.split(',').collect();
            Hex {
                q: f[0].parse().unwrap(),
                r: f[1].parse().unwrap(),
                kind: f[2].to_string(),
                percent: f[3].parse().unwrap(),
                bodies: f[6].parse().unwrap(),
            }
        })
        .collect()
}

fn frame() -> Frame {
    Frame::of_map(&root().join(MAP)).unwrap().unwrap()
}

/// The header: the title and the frame file's own lines, the projection and
/// extent lines of part 4 of `about_a_centre.rs`, the command that made the
/// map, and the anchors file it names.
#[test]
fn the_header_gives_the_frame_the_anchors_and_the_command() {
    let header = header_of(MAP);
    let file = Frame::read(&root().join(FRAME_FILE)).unwrap();
    assert_eq!(
        header[0],
        "Flag Plot map of the Pacific from Papua to Midway (decision 0242)."
    );
    assert_eq!(header[0], file.title);
    for line in [
        "Projection: about a centre. Hex (0,0) is centred at 4 N, 174.5 E.",
        "  distance_nm = angle from the centre in radians * 3437.746771 * 0.964, on the bearing from the centre",
        "  east_nm = distance_nm * sin(bearing), north_nm = distance_nm * cos(bearing); a sphere of 60 nm to the degree.",
        "Extent: every hex whose centre lies from 38 N to 30 S and from 142 E to 153 W.",
    ] {
        assert!(header.iter().any(|h| h == line), "{line}");
    }
    for line in file.projection_lines() {
        assert!(header.contains(&line), "{line}");
    }
    assert!(header.contains(&file.extent_line()));
    assert!(header
        .iter()
        .any(|h| h == "Anchors: data/map/papua_to_midway_anchors.csv. A hex that holds an anchorage stands for the body of water nearest it, if that body has at least 5 points (decision 0017, decision 6); the main bodies below are worked out after that."));
    let generated = header.last().unwrap();
    assert_eq!(
        generated,
        "Generated 2026-11-28 by `cargo run --release -p map-tool -- generate --land references/natural-earth/ne_10m_land/ne_10m_land.shp --minor-islands references/natural-earth/ne_10m_minor_islands/ne_10m_minor_islands.shp --frame data/map/frames/papua_to_midway.toml --out data/map/papua_to_midway.csv --anchors data/map/papua_to_midway_anchors.csv`"
    );
    // `sim-data` reads the same frame from the header.
    let read = sim_data::read_map_frame(&root().join(MAP))
        .unwrap()
        .unwrap();
    assert_eq!(read.east.to_string(), "207");
    assert!(read.crosses_date_line());
    assert_eq!(frame().projection(), file.projection());
}

/// The 10,395 hexes, in rows -60 to 58, are the hexes of the extent, in order,
/// one row of the file each.
#[test]
fn the_map_has_the_10395_hexes_of_the_extent() {
    let hexes = hexes();
    assert_eq!(hexes.len(), 10_395);
    let found: Vec<(i32, i32)> = hexes.iter().map(|h| (h.q, h.r)).collect();
    assert_eq!(found, extent_hexes(&frame()));
    assert_eq!(found.first().map(|h| h.1), Some(-60));
    assert_eq!(found.last().map(|h| h.1), Some(58));
    assert!(found
        .windows(2)
        .all(|w| (w[0].1, w[0].0) < (w[1].1, w[1].0)));
    assert!(header_of(MAP)
        .iter()
        .any(|h| h.starts_with("Hexes: 10395 (")));
}

/// The two land marks: Midway's hex (31,-41) and Wake's (2,-26), each coast
/// with land percentage 0 and one body of water, named in "Marked land".
#[test]
fn midway_and_wake_are_marked_coast_with_one_body_of_water() {
    let hexes = hexes();
    let header = header_of(MAP);
    let marked = header
        .iter()
        .find(|h| h.starts_with("Marked land: "))
        .expect("the Marked land line");
    for (name, row, q, r) in [("midway", 5100, 31, -41), ("wake", 5107, 2, -26)] {
        let hex = hexes.iter().find(|h| (h.q, h.r) == (q, r)).unwrap();
        assert_eq!(
            (hex.kind.as_str(), hex.percent, hex.bodies),
            ("coast", 0, 1),
            "{name}"
        );
        assert!(
            marked.contains(&format!(
                "{name} (data/oob/place_positions.csv row {row}) hex ({q},{r})"
            )),
            "{marked}"
        );
    }
    // Only those two are named.
    assert_eq!(
        marked.matches("(data/oob/place_positions.csv row ").count(),
        2
    );
    // The hexes with a land point and 0 percent are 97 and do not count the
    // marks: the map has 99 coast hexes of 0 percent.
    assert!(header
        .iter()
        .any(|h| h.starts_with("  A hex with at least one land point but 0 percent after rounding counts as coast with land_percent 0: 97 such hexes.")));
    let zero_coast = hexes
        .iter()
        .filter(|h| h.kind == "coast" && h.percent == 0)
        .count();
    assert_eq!(zero_coast, 99);
}

/// The counts by kind: 344 land, 365 coast and 9,686 deep sea, as the header
/// says.
#[test]
fn the_counts_by_kind() {
    let hexes = hexes();
    let count = |kind: &str| hexes.iter().filter(|h| h.kind == kind).count();
    assert_eq!(
        (count("land"), count("coast"), count("deep_sea")),
        (344, 365, 9_686)
    );
    assert!(header_of(MAP)
        .iter()
        .any(|h| h == "Hexes: 10395 (344 land, 365 coast, 9686 deep_sea)."));
}

/// The anchors file began as a copy of the south-west Pacific map's: its rows
/// are the same, none added; the map's header has one line for each.
#[test]
fn the_anchors_file_is_the_present_maps_rows_and_the_header_has_a_line_each() {
    let rows = |path: &str| -> Vec<String> {
        read(path)
            .lines()
            .filter(|l| !l.starts_with('#'))
            .map(str::to_string)
            .collect()
    };
    assert_eq!(rows(ANCHORS), rows("data/map/south_pacific_anchors.csv"));
    assert!(read(ANCHORS).starts_with("# Anchorages that decide"));
    let header = header_of(MAP);
    let at = header
        .iter()
        .position(|h| h.starts_with("Anchors: data/map/papua_to_midway_anchors.csv."))
        .unwrap();
    let anchors = rows(ANCHORS);
    for (k, anchor) in anchors.iter().skip(1).enumerate() {
        let place = anchor.split(',').next().unwrap();
        assert!(
            header[at + 1 + k].starts_with(&format!("  {place}: hex (")),
            "{}",
            header[at + 1 + k]
        );
    }
    assert!(header[at + anchors.len()].starts_with("closed_sides:"));
}

/// Every one of the 113 rows of the located file is at the hex the hand
/// calculation gives, with the row's own place, latitude and longitude text,
/// and nothing else is in the file.
#[test]
fn every_located_row_is_at_the_hex_of_the_hand_calculation() {
    let located = read(LOCATED);
    let mut lines = located.lines().filter(|l| !l.starts_with('#'));
    assert_eq!(lines.next(), Some(LOCATED_HEADER));
    let lines: Vec<&str> = lines.collect();
    assert_eq!(lines.len(), 113);
    assert_eq!(HAND.len(), 113);
    // The positions file, by id.
    let positions = read(POSITIONS);
    let mut reader = csv::ReaderBuilder::new()
        .comment(Some(b'#'))
        .from_reader(positions.as_bytes());
    let given: Vec<(u32, String, String, String)> = reader
        .records()
        .map(|r| {
            let r = r.unwrap();
            (
                r[0].parse().unwrap(),
                r[1].to_string(),
                r[3].to_string(),
                r[4].to_string(),
            )
        })
        .collect();
    assert_eq!(given.len(), 113);
    for ((line, hand), row) in lines.iter().zip(HAND.iter()).zip(given.iter()) {
        let (id, place, q, r) = *hand;
        assert_eq!(
            *line,
            format!("{id},{place},{},{},{q},{r}", row.2, row.3),
            "row {id}"
        );
        assert_eq!((row.0, row.1.as_str()), (id, place), "row {id}");
    }
}

/// The committed located file is byte for byte what the command writes.
#[test]
fn the_committed_located_file_is_what_the_command_writes() {
    let command = format!("locate-rows --map {MAP} --positions {POSITIONS} --out {LOCATED}");
    let made = located_text(&frame(), &read(POSITIONS), &command).unwrap();
    assert_eq!(made, read(LOCATED));
    assert!(Path::new(&root().join(LOCATED)).exists());
}

/// A row outside the extent has no line, and the order is that of the ids,
/// whatever order the positions file gives them.
#[test]
fn a_row_outside_the_extent_has_no_line_and_the_order_is_the_ids() {
    let positions = "id,place,feature,lat,lon\n\
        9,midway,\"the atoll, a quoted comma\",28.232,-177.370\n\
        3,tokyo,city,35.68,139.69\n\
        5,rabaul,harbour,-4.22224,152.16544\n\
        4,sydney,city,-33.87,151.21\n";
    let made = located_text(&frame(), positions, "x").unwrap();
    let lines: Vec<&str> = made.lines().filter(|l| !l.starts_with('#')).collect();
    assert_eq!(
        lines,
        [
            LOCATED_HEADER,
            "5,rabaul,-4.22224,152.16544,-39,14",
            "9,midway,28.232,-177.370,31,-41"
        ]
    );
}
