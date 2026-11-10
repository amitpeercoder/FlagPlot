//! The projection of decision 0017, 2: hex (0,0) is centred at 12 S, 160 E;
//! east in nm is (longitude - 160) x 60 x 0.978148; north in nm is
//! (latitude + 12) x 60. Hexes are pointy-topped, 40 nm between centres, and
//! north is decreasing r (decision 0012).

/// Cosine of 12 degrees, as a literal.
pub const COS_12: f64 = 0.978148;
/// Nautical miles in a degree of latitude.
pub const NM_PER_DEG_LAT: f64 = 60.0;
/// Nautical miles in a degree of longitude, at 12 degrees south.
pub const NM_PER_DEG_LON: f64 = 60.0 * COS_12;
/// Distance between centres of neighbouring hexes, in nm.
pub const HEX_NM: f64 = 40.0;
/// Distance between rows of hex centres, in nm: 40 x sqrt(3) / 2.
pub const ROW_NM: f64 = 34.641016;
/// Latitude of hex (0,0)'s centre.
pub const ORIGIN_LAT: f64 = -12.0;
/// Longitude of hex (0,0)'s centre.
pub const ORIGIN_LON: f64 = 160.0;

/// East and north of a point from hex (0,0)'s centre, in nm.
pub fn to_nm(lat: f64, lon: f64) -> (f64, f64) {
    (
        (lon - ORIGIN_LON) * NM_PER_DEG_LON,
        (lat - ORIGIN_LAT) * NM_PER_DEG_LAT,
    )
}

/// Latitude and longitude of a point given its east and north in nm.
pub fn from_nm(east: f64, north: f64) -> (f64, f64) {
    (
        ORIGIN_LAT + north / NM_PER_DEG_LAT,
        ORIGIN_LON + east / NM_PER_DEG_LON,
    )
}

/// East and north of hex (q, r)'s centre, in nm.
pub fn centre_nm(q: i32, r: i32) -> (f64, f64) {
    let (q, r) = (f64::from(q), f64::from(r));
    (HEX_NM * (q + r / 2.0), -ROW_NM * r)
}

/// Latitude and longitude of hex (q, r)'s centre.
pub fn centre_of(q: i32, r: i32) -> (f64, f64) {
    let (east, north) = centre_nm(q, r);
    from_nm(east, north)
}

/// The hex whose centre is nearest to the point, by cube rounding.
pub fn hex_of(lat: f64, lon: f64) -> (i32, i32) {
    let (east, north) = to_nm(lat, lon);
    hex_of_nm(east, north)
}

/// The hex whose centre is nearest to the point given in projected nm (east,
/// north from hex (0,0)'s centre), by cube rounding. A point exactly the same
/// distance from two centres is decided by rounding noise here; the lattice
/// of the generator fixes its own rule (`generate::lattice_hex`).
pub fn hex_of_nm(east: f64, north: f64) -> (i32, i32) {
    let r_f = -north / ROW_NM;
    let q_f = east / HEX_NM - r_f / 2.0;
    // Cube coordinates x = q, z = r, y = -x - z; round each, then put right
    // the one with the largest rounding error.
    let (x_f, z_f) = (q_f, r_f);
    let y_f = -x_f - z_f;
    let (mut x, y, mut z) = (x_f.round(), y_f.round(), z_f.round());
    let (dx, dy, dz) = ((x - x_f).abs(), (y - y_f).abs(), (z - z_f).abs());
    if dx > dy && dx > dz {
        x = -y - z;
    } else if dy > dz {
        // y is the worst: x and z stay as rounded.
    } else {
        z = -x - y;
    }
    (x as i32, z as i32)
}

/// Distance in nm between a point and the centre of hex (q, r).
pub fn distance_to_centre(lat: f64, lon: f64, q: i32, r: i32) -> f64 {
    let (east, north) = to_nm(lat, lon);
    let (ce, cn) = centre_nm(q, r);
    ((east - ce) * (east - ce) + (north - cn) * (north - cn)).sqrt()
}

/// A latitude or longitude as plain decimal text with four decimals.
pub fn degrees_text(value: f64) -> String {
    format!("{value:.4}")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn close(a: f64, b: f64, tolerance: f64) -> bool {
        (a - b).abs() <= tolerance
    }

    /// Part 2 of the stage 4b scenario's doc comment.
    #[test]
    fn the_four_points_of_part_2() {
        // (lat, lon, east, north, hex, nearest centre east, north, distance)
        let rows = [
            (-12.0, 160.0, 0.0, 0.0, (0, 0), 0.0, 0.0, 0.0),
            (-10.0, 162.0, 117.378, 120.0, (5, -4), 120.0, 138.564, 18.7),
            (
                -15.0,
                157.0,
                -176.067,
                -180.0,
                (-7, 5),
                -180.0,
                -173.205,
                7.9,
            ),
            (
                -4.0,
                152.0,
                -469.511,
                480.0,
                (-5, -14),
                -480.0,
                484.974,
                11.6,
            ),
        ];
        for (lat, lon, east, north, hex, ce, cn, distance) in rows {
            let (e, n) = to_nm(lat, lon);
            assert!(close(e, east, 0.0006), "east of {lat},{lon}: {e}");
            assert!(close(n, north, 1e-9), "north of {lat},{lon}: {n}");
            assert_eq!(hex_of(lat, lon), hex);
            let (cx, cy) = centre_nm(hex.0, hex.1);
            assert!(close(cx, ce, 1e-9) && close(cy, cn, 0.0006), "{cx} {cy}");
            let d = distance_to_centre(lat, lon, hex.0, hex.1);
            assert!(close(d, distance, 0.05), "distance {d} for {lat},{lon}");
        }
    }

    #[test]
    fn the_other_candidates_for_10_s_162_e() {
        let d43 = distance_to_centre(-10.0, 162.0, 4, -3);
        let d53 = distance_to_centre(-10.0, 162.0, 5, -3);
        assert!(close(d43, 23.7, 0.05), "{d43}");
        assert!(close(d53, 27.8, 0.05), "{d53}");
        let chosen = distance_to_centre(-10.0, 162.0, 5, -4);
        assert!(chosen < d43 && chosen < d53);
    }

    #[test]
    fn the_centre_of_5_minus_4() {
        let (lat, lon) = centre_of(5, -4);
        assert_eq!(degrees_text(lat), "-9.6906");
        assert_eq!(degrees_text(lon), "162.0447");
        let (lat, lon) = centre_of(0, 0);
        assert_eq!((lat, lon), (-12.0, 160.0));
    }

    #[test]
    fn centres_round_trip_through_hex_of() {
        for r in -30..=30 {
            for q in -40..=40 {
                let (lat, lon) = centre_of(q, r);
                assert_eq!(hex_of(lat, lon), (q, r));
            }
        }
    }

    /// Cube rounding agrees with a brute-force nearest-centre search over a
    /// grid of points every 0.05 degrees. Where the two best centres are
    /// within a millionth of a nm (a tie), either is accepted.
    #[test]
    fn hex_rounding_matches_brute_force() {
        let mut checked = 0;
        for i in 0..=120 {
            for j in 0..=120 {
                let lat = -14.0 + f64::from(i) * 0.05;
                let lon = 158.0 + f64::from(j) * 0.05;
                let got = hex_of(lat, lon);
                let mut best = (f64::MAX, (0, 0));
                let mut second = f64::MAX;
                for r in -20..=20 {
                    for q in -25..=25 {
                        let d = distance_to_centre(lat, lon, q, r);
                        if d < best.0 {
                            second = best.0;
                            best = (d, (q, r));
                        } else if d < second {
                            second = d;
                        }
                    }
                }
                if second - best.0 > 1e-6 {
                    assert_eq!(got, best.1, "at {lat},{lon}");
                } else {
                    let d = distance_to_centre(lat, lon, got.0, got.1);
                    assert!(d - best.0 < 1e-6, "tie at {lat},{lon}");
                }
                checked += 1;
            }
        }
        assert_eq!(checked, 121 * 121);
    }
}
