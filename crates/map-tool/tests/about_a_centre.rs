//! The projection about a centre (decision 0242, rules 3, 12 and 16), checked
//! against a hand calculation.
//!
//! # The hand calculation
//!
//! Written by the architect on Nov 28, 2026, before any code. Nobody else
//! changes a number here; a mismatch goes back to the architect.
//!
//! ## The frame
//!
//! The frame of decision 0242, rule 1: kind "about a centre", centre 4 N,
//! 174.5 E, scale 0.964, extent 38 N to 30 S and 142 E to 153 W (207 in the
//! run). The miles in a radian are 3437.746771. Hexes are 40 nm between
//! centres and rows 34.641016 nm apart, as on every map: hex (q, r)'s centre
//! is 40 x (q + r/2) nm east and 34.641016 x r nm south of hex (0,0)'s.
//!
//! ## Part 1: a position to the plane, and to its hex
//!
//! For a point at latitude `lat` and longitude `lon` (in the run), with the
//! centre at `lat0`, `lon0`, all in radians:
//!
//! - `cos c = sin lat0 sin lat + cos lat0 cos lat cos(lon - lon0)`; `c` is the
//!   angle from the centre.
//! - the bearing is `atan2(sin(lon - lon0) cos lat,
//!   cos lat0 sin lat - sin lat0 cos lat cos(lon - lon0))`.
//! - the distance on the map is `c x 3437.746771 x 0.964`; east is the
//!   distance times the sine of the bearing, north the distance times the
//!   cosine.
//!
//! Each position is a row of `data/oob/place_positions.csv`, as the row gives
//! it. A longitude west of 142 E has 360 added.
//!
//! | Row | lat | lon | angle, rad | bearing | east, nm | north, nm | hex | to its centre, nm | next nearest |
//! | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
//! | the centre | 4.0 | 174.5 | 0 | none | 0.000 | 0.000 | (0,0) | 0.00 | |
//! | 5100 Midway | 28.232 | -177.370 | 0.44395982 | 16.8629 | 426.793 | 1408.015 | (31,-41) | 14.02 | (31,-40) 25.98 |
//! | 5107 Wake | 19.299 | 166.624 | 0.29883292 | 333.9413 | -435.042 | 889.657 | (2,-26) | 12.07 | (2,-25) 28.01 |
//! | 5113 Point Luck | 32.0 | -173.0 | 0.52982716 | 21.2963 | 637.706 | 1635.942 | (39,-47) | 19.35 | (40,-47) 23.62 |
//! | 46 Truk | 7.454 | 151.808 | 0.39857090 | 279.7315 | -1301.853 | 223.266 | (-29,-7) | 19.31 | (-30,-6) 23.81 |
//! | 1 Rabaul | -4.22224 | 152.16544 | 0.41507194 | 250.0193 | -1292.746 | -470.029 | (-39,14) | 19.64 | (-39,13) 20.99 |
//! | 6 Lunga Point | -9.40538 | 160.02861 | 0.34365516 | 227.0303 | -833.327 | -776.266 | (-32,22) | 15.66 | (-32,23) 24.43 |
//! | 16 Noumea | -22.25675 | 166.42019 | 0.47856679 | 196.4078 | -447.991 | -1521.377 | (-33,44) | 8.48 | (-34,44) 32.13 |
//! | 5108 Saipan | 15.194 | 145.748 | 0.53103177 | 293.5611 | -1613.123 | 703.453 | (-30,-20) | 16.89 | (-30,-21) 24.97 |
//!
//! East and north hold to 0.001 nm, the angle to 0.00000001, the bearing to
//! 0.0001 degree, a distance to a centre to 0.01 nm. The hex is exact.
//!
//! ## Part 2: a hex to its position
//!
//! The other way: for a point `east`, `north` in the plane, `rho` is its
//! distance from the origin, `c = rho / (3437.746771 x 0.964)`, the bearing is
//! `atan2(east, north)`, and
//!
//! - `lat = asin(sin lat0 cos c + cos lat0 sin c cos bearing)`;
//! - `lon = lon0 + atan2(sin bearing sin c cos lat0,
//!   cos c - sin lat0 sin lat)`, which is in the run.
//!
//! The origin itself is the centre. The file's longitude is the run's less
//! 360 where the run's is above 180.
//!
//! | Hex | east, nm | north, nm | lat | lon in the run | lon in the file |
//! | --- | --- | --- | --- | --- | --- |
//! | (0,0) | 0.000 | 0.000 | 4.0000 | 174.5000 | 174.5000 |
//! | (10,0) | 400.000 | 0.000 | 3.9709 | 181.4324 | -178.5676 |
//! | (0,-10) | -200.000 | 346.410 | 9.9781 | 170.9954 | 170.9954 |
//! | (18,-13) | 460.000 | 450.333 | 11.7213 | 182.5983 | -177.4017 |
//! | (-40,30) | -1000.000 | -1039.230 | -13.8741 | 156.9763 | 156.9763 |
//! | (45,-20) | 1400.000 | 692.820 | 15.2519 | 199.4508 | -160.5492 |
//! | (-46,3) | -1780.000 | -103.923 | 1.7252 | 143.7157 | 143.7157 |
//!
//! Latitude and longitude hold to four decimals as printed. Each of these
//! centres, put back through part 1, falls in its own hex. So does every hex
//! of part 3.
//!
//! ## Part 3: the hexes of the extent
//!
//! A hex is of the map when its centre's latitude is from -30 to 38 and its
//! longitude in the run from 142 to 207, edges included.
//!
//! - There are 10,395 such hexes, in rows -60 to 58.
//! - Row by row, the first and last q and the count:
//!
//! | r | first q | last q | hexes |
//! | --- | --- | --- | --- |
//! | -28 | -30 | 58 | 89 |
//! | -10 | -41 | 51 | 93 |
//! | 0 | -46 | 46 | 93 |
//! | 3 | -48 | 45 | 94 |
//! | 10 | -52 | 42 | 95 |
//! | 30 | -61 | 31 | 93 |
//! | 57 | -71 | 14 | 56 |
//!
//! - The edges bow. Along the west edge (142 E) the plane's east is least at
//!   2.1 S, -1882.04 nm, and at the north-west corner it is only -1559.20.
//!   Along the north edge (38 N) the plane's north is greatest at the two
//!   corners, 2090.98 nm, and along the south edge least at the two corners,
//!   -2034.03. A box taken from the north-west and south-east corners alone
//!   (west -1559.2, east 1721.7) with a hex to spare would miss 587 of the
//!   10,395. Hex (-46,3) of part 2 is one of them.
//! - No hex centre lies within 0.0005 degree of an edge, so the count does
//!   not turn on the last bits of the arithmetic.
//!
//! ## Part 4: the header's lines
//!
//! For this frame the tool writes exactly:
//!
//! ```text
//! # Projection: about a centre. Hex (0,0) is centred at 4 N, 174.5 E.
//! #   distance_nm = angle from the centre in radians * 3437.746771 * 0.964, on the bearing from the centre
//! #   east_nm = distance_nm * sin(bearing), north_nm = distance_nm * cos(bearing); a sphere of 60 nm to the degree.
//! # Extent: every hex whose centre lies from 38 N to 30 S and from 142 E to 153 W.
//! ```
//!
//! `sim-data` reads them back as: kind about a centre, centre 4 and 174.5,
//! scale 0.964, north 38, south -30, west 142, east 207 in the run. A header
//! with the first line and without the second or third is refused; so is one
//! with a line of each kind.
//!
//! ## Part 5: the two present maps
//!
//! For a frame of one factor nothing changes: the lines the tool writes are
//! those it writes today, and the hexes of the extent are the same hexes in
//! the same order. The test of the committed maps checks this for both.

use map_tool::frame::Frame;
use map_tool::generate::extent_hexes;
use map_tool::projection::{centre_nm, hex_of_nm, HEX_NM, ROW_NM};

const WIDE: &str = "title = \"A test frame.\"\n\
    date = \"2026-11-28\"\n\
    kind = \"about a centre\"\n\
    centre_lat = \"4\"\n\
    centre_lon = \"174.5\"\n\
    scale = \"0.964\"\n\
    north = \"38\"\n\
    south = \"-30\"\n\
    west = \"142\"\n\
    east = \"-153\"\n";

fn wide() -> Frame {
    Frame::parse(WIDE).unwrap()
}

fn close(a: f64, b: f64, tolerance: f64) -> bool {
    (a - b).abs() <= tolerance
}

/// Part 1: each row of the table, from the position to the plane and its
/// hex, with the distance to its centre and to the next nearest.
#[test]
fn part_1_a_position_to_the_plane_and_its_hex() {
    // (name, [lat, lon in the file, angle, bearing, east, north, distance to
    // its centre, distance to the next nearest], hex, next nearest)
    #[rustfmt::skip]
    let rows = [
        ("centre", [4.0, 174.5, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0], (0, 0), (0, 0)),
        ("Midway", [28.232, -177.370, 0.44395982, 16.8629, 426.793, 1408.015, 14.02, 25.98], (31, -41), (31, -40)),
        ("Wake", [19.299, 166.624, 0.29883292, 333.9413, -435.042, 889.657, 12.07, 28.01], (2, -26), (2, -25)),
        ("Point Luck", [32.0, -173.0, 0.52982716, 21.2963, 637.706, 1635.942, 19.35, 23.62], (39, -47), (40, -47)),
        ("Truk", [7.454, 151.808, 0.39857090, 279.7315, -1301.853, 223.266, 19.31, 23.81], (-29, -7), (-30, -6)),
        ("Rabaul", [-4.22224, 152.16544, 0.41507194, 250.0193, -1292.746, -470.029, 19.64, 20.99], (-39, 14), (-39, 13)),
        ("Lunga Point", [-9.40538, 160.02861, 0.34365516, 227.0303, -833.327, -776.266, 15.66, 24.43], (-32, 22), (-32, 23)),
        ("Noumea", [-22.25675, 166.42019, 0.47856679, 196.4078, -447.991, -1521.377, 8.48, 32.13], (-33, 44), (-34, 44)),
        ("Saipan", [15.194, 145.748, 0.53103177, 293.5611, -1613.123, 703.453, 16.89, 24.97], (-30, -20), (-30, -21)),
    ];
    let frame = wide();
    let projection = frame.projection();
    for (name, v, hex, next) in rows {
        let [lat, lon, angle, bearing, east, north, d_hex, d_next] = v;
        let lon = frame.lon_in_run(lon);
        let (e, n) = projection.to_nm(lat, lon);
        assert!(close(e, east, 0.001), "{name}: east {e}");
        assert!(close(n, north, 0.001), "{name}: north {n}");
        let distance = e.hypot(n);
        let a = distance / (3437.746771 * 0.964);
        assert!(close(a, angle, 0.00000001), "{name}: angle {a}");
        if distance > 0.0 {
            let b = e.atan2(n).to_degrees().rem_euclid(360.0);
            assert!(close(b, bearing, 0.0001), "{name}: bearing {b}");
        }
        assert_eq!(projection.hex_of(lat, lon), hex, "{name}");
        let d = projection.distance_to_centre(lat, lon, hex.0, hex.1);
        assert!(close(d, d_hex, 0.01), "{name}: distance {d}");
        if next != hex {
            let d = projection.distance_to_centre(lat, lon, next.0, next.1);
            assert!(close(d, d_next, 0.01), "{name}: next {d}");
        }
    }
}

/// Part 2: each hex's centre in latitude and longitude, the longitude in the
/// run and in the file, and each put back through part 1 in its own hex.
#[test]
fn part_2_a_hex_to_its_position() {
    // (q, r, east, north, lat, lon in the run, lon in the file)
    let rows = [
        (0, 0, 0.0, 0.0, 4.0, 174.5, 174.5),
        (10, 0, 400.0, 0.0, 3.9709, 181.4324, -178.5676),
        (0, -10, -200.0, 346.410, 9.9781, 170.9954, 170.9954),
        (18, -13, 460.0, 450.333, 11.7213, 182.5983, -177.4017),
        (-40, 30, -1000.0, -1039.230, -13.8741, 156.9763, 156.9763),
        (45, -20, 1400.0, 692.820, 15.2519, 199.4508, -160.5492),
        (-46, 3, -1780.0, -103.923, 1.7252, 143.7157, 143.7157),
    ];
    let frame = wide();
    let projection = frame.projection();
    for (q, r, east, north, lat, lon_run, lon_file) in rows {
        let (e, n) = centre_nm(q, r);
        assert!(
            close(e, east, 0.001) && close(n, north, 0.001),
            "({q},{r}): {e} {n}"
        );
        let (la, lo) = projection.centre_of(q, r);
        assert!(close(la, lat, 0.00005), "({q},{r}): lat {la}");
        assert!(close(lo, lon_run, 0.00005), "({q},{r}): lon {lo}");
        assert!(
            close(frame.lon_for_file(lo), lon_file, 0.00005),
            "({q},{r})"
        );
        assert_eq!(projection.hex_of(la, lo), (q, r));
    }
}

/// Part 3: the hexes of the extent.
#[test]
fn part_3_the_hexes_of_the_extent() {
    let frame = wide();
    let hexes = extent_hexes(&frame);
    assert_eq!(hexes.len(), 10_395);
    let rows: Vec<i32> = hexes.iter().map(|h| h.1).collect();
    assert_eq!((rows[0], *rows.last().unwrap()), (-60, 58));
    // Sorted by r, then q, and no hex twice.
    assert!(hexes
        .windows(2)
        .all(|w| (w[0].1, w[0].0) < (w[1].1, w[1].0)));
    // Row by row: first q, last q, count.
    for (r, first, last, count) in [
        (-28, -30, 58, 89),
        (-10, -41, 51, 93),
        (0, -46, 46, 93),
        (3, -48, 45, 94),
        (10, -52, 42, 95),
        (30, -61, 31, 93),
        (57, -71, 14, 56),
    ] {
        let row: Vec<i32> = hexes.iter().filter(|h| h.1 == r).map(|h| h.0).collect();
        assert_eq!(
            (row[0], *row.last().unwrap(), row.len()),
            (first, last, count),
            "row {r}"
        );
    }
    // Every hex is of the extent, and none is within 0.0005 degree of an
    // edge.
    let projection = frame.projection();
    for &(q, r) in &hexes {
        let (lat, lon) = projection.centre_of(q, r);
        assert!(
            (-30.0..=38.0).contains(&lat) && (142.0..=207.0).contains(&lon),
            "({q},{r}) {lat} {lon}"
        );
    }
    let mut nearest_edge = f64::MAX;
    for q in -90..=90 {
        for r in -70..=70 {
            let (lat, lon) = projection.centre_of(q, r);
            for d in [lat - -30.0, lat - 38.0, lon - 142.0, lon - 207.0] {
                nearest_edge = nearest_edge.min(d.abs());
            }
        }
    }
    // The nearest to an edge among all the hexes searched, in or out.
    assert!(nearest_edge > 0.0005, "{nearest_edge}");

    // The edges bow.
    let along = |from: f64, to: f64| -> Vec<f64> {
        let steps = ((to - from) * 10.0).round() as i32;
        (0..=steps).map(|k| from + f64::from(k) / 10.0).collect()
    };
    let west_edge: Vec<(f64, f64)> = along(-30.0, 38.0)
        .into_iter()
        .map(|lat| (lat, projection.to_nm(lat, 142.0).0))
        .collect();
    let least = west_edge
        .iter()
        .copied()
        .fold((0.0, f64::MAX), |a, b| if b.1 < a.1 { b } else { a });
    assert!(
        close(least.1, -1882.04, 0.01) && close(least.0, -2.1, 0.0501),
        "{least:?}"
    );
    assert!(close(projection.to_nm(38.0, 142.0).0, -1559.20, 0.01));
    let north_edge_max = along(142.0, 207.0)
        .into_iter()
        .map(|lon| projection.to_nm(38.0, lon).1)
        .fold(f64::MIN, f64::max);
    assert!(close(north_edge_max, 2090.98, 0.01), "{north_edge_max}");
    assert!(close(projection.to_nm(38.0, 142.0).1, 2090.98, 0.01));
    assert!(close(projection.to_nm(38.0, 207.0).1, 2090.98, 0.01));
    let south_edge_min = along(142.0, 207.0)
        .into_iter()
        .map(|lon| projection.to_nm(-30.0, lon).1)
        .fold(f64::MAX, f64::min);
    assert!(close(south_edge_min, -2034.03, 0.01), "{south_edge_min}");

    // A box taken from the north-west and south-east corners alone, with a
    // hex to spare, would miss 587 of the hexes (and hex (-46,3) is one).
    let (west, north) = projection.to_nm(38.0, 142.0);
    let (east, south) = projection.to_nm(-30.0, 207.0);
    assert!(close(west, -1559.2, 0.05) && close(east, 1721.7, 0.05));
    let r_low = (-north / ROW_NM).floor() as i32 - 1;
    let r_high = (-south / ROW_NM).ceil() as i32 + 1;
    let mut seen_by_corners = std::collections::BTreeSet::new();
    for r in r_low..=r_high {
        let half = f64::from(r) / 2.0;
        let q_low = (west / HEX_NM - half).floor() as i32 - 1;
        let q_high = (east / HEX_NM - half).ceil() as i32 + 1;
        for q in q_low..=q_high {
            seen_by_corners.insert((q, r));
        }
    }
    let missed = hexes
        .iter()
        .filter(|h| !seen_by_corners.contains(h))
        .count();
    assert_eq!(missed, 587);
    assert!(!seen_by_corners.contains(&(-46, 3)));
    assert!(hexes.contains(&(-46, 3)));
    // Every centre of the extent put back through part 1 is in its own hex.
    for &(q, r) in &hexes {
        let (lat, lon) = projection.centre_of(q, r);
        assert_eq!(projection.hex_of(lat, lon), (q, r));
        let (e, n) = projection.to_nm(lat, lon);
        assert_eq!(hex_of_nm(e, n), (q, r));
    }
}

/// Part 4: the header's lines, as the tool writes them, and as `sim-data`
/// reads them back.
#[test]
fn part_4_the_headers_lines() {
    let frame = wide();
    let mut lines: Vec<String> = frame.projection_lines().to_vec();
    lines.push(frame.extent_line());
    let written: String = lines.iter().map(|l| format!("# {l}\n")).collect();
    assert_eq!(
        written,
        "# Projection: about a centre. Hex (0,0) is centred at 4 N, 174.5 E.\n\
         #   distance_nm = angle from the centre in radians * 3437.746771 * 0.964, on the bearing from the centre\n\
         #   east_nm = distance_nm * sin(bearing), north_nm = distance_nm * cos(bearing); a sphere of 60 nm to the degree.\n\
         # Extent: every hex whose centre lies from 38 N to 30 S and from 142 E to 153 W.\n"
    );
    let fixed = |text: &str| text.parse::<sim_core::fixed::Fixed>().unwrap();
    let read = sim_data::parse_map_frame(&written).unwrap().unwrap();
    assert_eq!(
        read,
        sim_data::MapFrame {
            centre_lat: fixed("4"),
            centre_lon: fixed("174.5"),
            projection: sim_data::MapProjection::AboutCentre {
                scale: fixed("0.964")
            },
            north: fixed("38"),
            south: fixed("-30"),
            west: fixed("142"),
            east: fixed("207"),
        }
    );
    // The first line with the second or third missing is refused, and so is
    // a line of each kind.
    let kept = |skip: usize| -> String {
        written
            .lines()
            .enumerate()
            .filter(|(k, _)| *k != skip)
            .map(|(_, l)| format!("{l}\n"))
            .collect()
    };
    assert!(sim_data::parse_map_frame(&kept(1)).is_err());
    assert!(sim_data::parse_map_frame(&kept(2)).is_err());
    let one_factor = "# Projection: hex (0,0) is centred at 12 S, 160 E.\n";
    assert!(sim_data::parse_map_frame(&format!("{written}{one_factor}")).is_err());
    // The map tool's own reading of the frame it wrote agrees.
    let dir = std::env::temp_dir().join(format!("about_a_centre_{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    let path = dir.join("wide.csv");
    std::fs::write(
        &path,
        format!("{written}q,r,kind,land_percent,lat,lon,water_bodies,closed_sides\n"),
    )
    .unwrap();
    let back = Frame::of_map(&path).unwrap().unwrap();
    std::fs::remove_dir_all(&dir).unwrap();
    assert!(back.is_about_a_centre());
    assert_eq!(back.projection(), frame.projection());
    assert_eq!(back.extent_line(), frame.extent_line());
}

/// Part 5: the two present maps. The lines the tool writes for each are the
/// lines in its file, and the hexes of the extent are its rows, in order.
#[test]
fn part_5_the_two_present_maps_are_unchanged() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../");
    for name in ["south_pacific", "midway"] {
        let path = root.join(format!("data/map/{name}.csv"));
        let text = std::fs::read_to_string(&path).unwrap();
        let frame = Frame::of_map(&path).unwrap().unwrap();
        assert!(!frame.is_about_a_centre(), "{name}");
        let header: Vec<&str> = text.lines().filter(|l| l.starts_with('#')).collect();
        let written: Vec<String> = frame
            .projection_lines()
            .into_iter()
            .chain([frame.extent_line()])
            .map(|l| format!("# {l}"))
            .collect();
        for line in &written {
            assert!(header.contains(&line.as_str()), "{name}: {line}");
        }
        let rows: Vec<(i32, i32)> = text
            .lines()
            .filter(|l| !l.starts_with('#'))
            .skip(1)
            .map(|l| {
                let mut f = l.split(',');
                (
                    f.next().unwrap().parse().unwrap(),
                    f.next().unwrap().parse().unwrap(),
                )
            })
            .collect();
        assert_eq!(extent_hexes(&frame), rows, "{name}");
    }
}
