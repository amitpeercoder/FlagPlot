//! Bearings and arcs on the hex grid.
//!
//! Frame: x east, y north. North is the direction of decreasing `r`. Hexes
//! are pointy-topped, so the six neighbours lie on bearings 030, 090, 150,
//! 210, 270 and 330. Bearings run clockwise from north in whole multiples of
//! ten degrees.
//!
//! The offset of a hex from a base, in hex-centre spacings, is
//! `x = dq + dr / 2`, `y = -dr * 0.866025`. Directions come from a table of
//! sines and cosines at ten-degree steps, with exact values at the axes and
//! on the six neighbour bearings, so those lines test exactly. Everything is
//! integer arithmetic.

use crate::fixed::Fixed;
use crate::hex::Hex;

/// Sine of each bearing 0, 10, ..., 350 degrees, in millionths.
const SIN: [i64; 36] = [
    0, 173_648, 342_020, 500_000, 642_788, 766_044, 866_025, 939_693, 984_808, 1_000_000, 984_808,
    939_693, 866_025, 766_044, 642_788, 500_000, 342_020, 173_648, 0, -173_648, -342_020, -500_000,
    -642_788, -766_044, -866_025, -939_693, -984_808, -1_000_000, -984_808, -939_693, -866_025,
    -766_044, -642_788, -500_000, -342_020, -173_648,
];

/// Cosine of each bearing 0, 10, ..., 350 degrees, in millionths.
const COS: [i64; 36] = [
    1_000_000, 984_808, 939_693, 866_025, 766_044, 642_788, 500_000, 342_020, 173_648, 0, -173_648,
    -342_020, -500_000, -642_788, -766_044, -866_025, -939_693, -984_808, -1_000_000, -984_808,
    -939_693, -866_025, -766_044, -642_788, -500_000, -342_020, -173_648, 0, 173_648, 342_020,
    500_000, 642_788, 766_044, 866_025, 939_693, 984_808,
];

/// Vertical spacing of hex rows, `sqrt(3) / 2`, in millionths.
const ROW_HEIGHT: Fixed = Fixed::from_raw(866_025);

/// Offset of `hex` from `base` as `(x, y)` in hex-centre spacings, x east and
/// y north.
pub fn offset(base: Hex, hex: Hex) -> (Fixed, Fixed) {
    let dq = i64::from(hex.q) - i64::from(base.q);
    let dr = i64::from(hex.r) - i64::from(base.r);
    let x = Fixed::from_int(dq) + Fixed::from_ratio(dr, 2);
    let y = -(ROW_HEIGHT * dr);
    (x, y)
}

/// Width of the clockwise arc from `from_deg` to `to_deg`, in degrees:
/// `(to - from) mod 360`, where 0 means the whole circle, 360.
pub fn arc_degrees(from_deg: u32, to_deg: u32) -> u32 {
    let width = (to_deg + 360 - from_deg) % 360;
    if width == 0 {
        360
    } else {
        width
    }
}

/// Direction vector `(sin, cos)` of a bearing, in raw millionths.
fn direction(deg: u32) -> (i128, i128) {
    let index = usize::try_from(deg / 10).expect("bearing index");
    (i128::from(SIN[index]), i128::from(COS[index]))
}

/// Is `hex` inside the arc running clockwise from bearing `from_deg` to
/// bearing `to_deg`, as seen from `base`?
///
/// Both bearings must be multiples of 10 in `0..=350` (asserted). The base's
/// own hex is always inside, edges are inclusive, and `from_deg == to_deg`
/// means the whole circle. Arcs wider than 180 degrees work.
pub fn in_arc(base: Hex, hex: Hex, from_deg: u32, to_deg: u32) -> bool {
    assert!(
        from_deg < 360 && from_deg.is_multiple_of(10),
        "bearing {from_deg} is not a multiple of 10 in 0..=350"
    );
    assert!(
        to_deg < 360 && to_deg.is_multiple_of(10),
        "bearing {to_deg} is not a multiple of 10 in 0..=350"
    );
    if base == hex || from_deg == to_deg {
        return true;
    }
    let (px, py) = offset(base, hex);
    let (px, py) = (i128::from(px.raw()), i128::from(py.raw()));
    // Negative means the point is clockwise of the direction `a`.
    let cross = |a: (i128, i128)| a.0 * py - a.1 * px;
    let after_from = cross(direction(from_deg)) <= 0;
    let before_to = cross(direction(to_deg)) >= 0;
    if arc_degrees(from_deg, to_deg) <= 180 {
        after_from && before_to
    } else {
        after_from || before_to
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const BASE: Hex = Hex::new(0, 0);

    #[test]
    fn the_table_is_consistent() {
        for index in 0..36usize {
            let (s, c) = (i128::from(SIN[index]), i128::from(COS[index]));
            // sin^2 + cos^2 = 1 within 2e-6, in millionths squared.
            let error = (s * s + c * c - 1_000_000_000_000i128).abs();
            assert!(error <= 2_000_000, "entry {index}: error {error}");
            // sin(b) = cos(90 - b), and cos(b) = sin(90 - b).
            let mirror = (9 + 36 - index) % 36;
            assert_eq!(SIN[index], COS[mirror], "sin {index} vs cos mirror");
            assert_eq!(COS[index], SIN[mirror], "cos {index} vs sin mirror");
            // sin(b + 180) = -sin(b), cos(b + 180) = -cos(b).
            assert_eq!(SIN[(index + 18) % 36], -SIN[index]);
            assert_eq!(COS[(index + 18) % 36], -COS[index]);
            // sin(180 - b) = sin(b).
            assert_eq!(SIN[(18 + 36 - index) % 36], SIN[index]);
        }
        assert_eq!((SIN[3], COS[6]), (500_000, 500_000));
        assert_eq!((SIN[6], COS[3]), (866_025, 866_025));
        assert_eq!(
            (SIN[0], COS[0], SIN[9], COS[9]),
            (0, 1_000_000, 1_000_000, 0)
        );
    }

    #[test]
    fn offsets_follow_the_hex_geometry() {
        // (1,0): due east one spacing.
        assert_eq!(offset(BASE, Hex::new(1, 0)), (Fixed::ONE, Fixed::ZERO));
        // (1,-1): dr = -1 gives x = 1 - 0.5 = 0.5, y = +0.866025 (bearing 030).
        assert_eq!(
            offset(BASE, Hex::new(1, -1)),
            (Fixed::from_raw(500_000), Fixed::from_raw(866_025))
        );
        // (-1,1): x = -1 + 0.5 = -0.5, y = -0.866025 (bearing 210).
        assert_eq!(
            offset(BASE, Hex::new(-1, 1)),
            (Fixed::from_raw(-500_000), Fixed::from_raw(-866_025))
        );
    }

    #[test]
    fn arc_width_wraps_and_full_circle_is_360() {
        assert_eq!(arc_degrees(60, 120), 60);
        assert_eq!(arc_degrees(330, 30), 60);
        assert_eq!(arc_degrees(0, 270), 270);
        assert_eq!(arc_degrees(270, 0), 90);
        assert_eq!(arc_degrees(0, 0), 360);
        assert_eq!(arc_degrees(90, 90), 360);
        assert_eq!(arc_degrees(0, 350), 350);
    }

    #[test]
    fn the_six_neighbours_lie_on_their_bearings() {
        // Neighbour offsets in DIRECTIONS order, paired with their bearing.
        let neighbours = [
            ((1, 0), 90),
            ((1, -1), 30),
            ((0, -1), 330),
            ((-1, 0), 270),
            ((-1, 1), 210),
            ((0, 1), 150),
        ];
        for ((q, r), bearing) in neighbours {
            let hex = Hex::new(q, r);
            let before = (bearing + 350) % 360;
            let after = (bearing + 10) % 360;
            // A narrow arc around the bearing holds it.
            assert!(
                in_arc(BASE, hex, before, after),
                "{hex} in {before}-{after}"
            );
            // The bearing is exactly on the edge of arcs starting or ending there.
            assert!(
                in_arc(BASE, hex, bearing, after),
                "{hex} on edge from {bearing}"
            );
            assert!(
                in_arc(BASE, hex, before, bearing),
                "{hex} on edge to {bearing}"
            );
            // The arc 20 degrees further round does not.
            let far_from = (bearing + 20) % 360;
            let far_to = (bearing + 40) % 360;
            assert!(
                !in_arc(BASE, hex, far_from, far_to),
                "{hex} in {far_from}-{far_to}"
            );
        }
        // The east neighbour specifically: inside 080-100, edge of 090-100
        // and 080-090, outside 100-120.
        let east = Hex::new(1, 0);
        assert!(in_arc(BASE, east, 80, 100));
        assert!(in_arc(BASE, east, 90, 100));
        assert!(in_arc(BASE, east, 80, 90));
        assert!(!in_arc(BASE, east, 100, 120));
    }

    #[test]
    fn hexes_due_east_and_west_on_row_zero() {
        // On r = 0, y = 0 and x = q, so bearing 090 (q > 0) or 270 (q < 0).
        for q in 1..=25 {
            let east = Hex::new(q, 0);
            let west = Hex::new(-q, 0);
            assert!(in_arc(BASE, east, 60, 120), "{east}");
            assert!(!in_arc(BASE, east, 240, 300), "{east}");
            assert!(in_arc(BASE, west, 240, 300), "{west}");
            assert!(!in_arc(BASE, west, 60, 120), "{west}");
        }
    }

    #[test]
    fn hexes_exactly_on_the_edge_lines_are_inside() {
        // (2,-1): x = 2 - 0.5 = 1.5, y = 0.866025. Ratio y/x = 0.577350, the
        // tangent of 30, so bearing 060 (angle from north = atan(x/y) = 60).
        // (4,-2) is the same line, twice as far.
        // (1,1): x = 1.5, y = -0.866025, bearing 120. (3,3) is the same, 3x.
        for hex in [
            Hex::new(2, -1),
            Hex::new(4, -2),
            Hex::new(1, 1),
            Hex::new(3, 3),
        ] {
            assert!(in_arc(BASE, hex, 60, 120), "{hex} on an edge of 060-120");
        }
        // Each is inside when it is the only edge of a narrower arc too.
        assert!(in_arc(BASE, Hex::new(2, -1), 60, 90));
        assert!(in_arc(BASE, Hex::new(1, 1), 90, 120));
        // And outside the arc on the other side of the edge.
        assert!(!in_arc(BASE, Hex::new(2, -1), 90, 120));
        assert!(!in_arc(BASE, Hex::new(1, 1), 60, 90));
    }

    #[test]
    fn an_arc_across_north() {
        // (1,-2): x = 1 - 1 = 0, y = 1.732050 > 0, due north (bearing 000).
        let north = Hex::new(1, -2);
        assert_eq!(offset(BASE, north).0, Fixed::ZERO);
        assert!(in_arc(BASE, north, 330, 30));
        // (0,-1) is bearing 330 and (1,-1) is bearing 030: both on the edges.
        assert!(in_arc(BASE, Hex::new(0, -1), 330, 30));
        assert!(in_arc(BASE, Hex::new(1, -1), 330, 30));
        // Due east (3,0) and due west are outside.
        assert!(!in_arc(BASE, Hex::new(3, 0), 330, 30));
        assert!(!in_arc(BASE, Hex::new(-3, 0), 330, 30));
        // Due south (-1,2): x = -1 + 1 = 0, y = -1.732050, is outside too.
        assert!(!in_arc(BASE, Hex::new(-1, 2), 330, 30));
    }

    #[test]
    fn a_wide_arc_of_270_degrees() {
        // Clockwise from 000 to 270 covers north through east and south to west.
        let east = Hex::new(3, 0); // 090
        let south = Hex::new(-1, 2); // x = 0, y = -1.73: 180
        let west = Hex::new(-3, 0); // 270, on the edge
        assert!(in_arc(BASE, east, 0, 270));
        assert!(in_arc(BASE, south, 0, 270));
        assert!(in_arc(BASE, west, 0, 270));
        // (-1,-1): x = -1 - 0.5 = -1.5, y = 0.866025. It is 60 degrees west
        // of north, bearing 300, in the missing quadrant 270 to 360.
        assert!(!in_arc(BASE, Hex::new(-1, -1), 0, 270));
        // (0,-1) is bearing 330, also in the missing part.
        assert!(!in_arc(BASE, Hex::new(0, -1), 0, 270));
    }

    #[test]
    fn full_circle_holds_every_hex_in_radius_five() {
        for (from, to) in [(0, 0), (90, 90), (210, 210)] {
            for q in -5..=5 {
                for r in -5..=5 {
                    let hex = Hex::new(q, r);
                    if BASE.distance(hex) <= 5 {
                        assert!(in_arc(BASE, hex, from, to), "{hex} in {from}-{to}");
                    }
                }
            }
        }
    }

    #[test]
    fn the_base_hex_is_inside_every_arc() {
        for from in (0..360).step_by(10) {
            for to in (0..360).step_by(10) {
                assert!(in_arc(BASE, BASE, from, to));
            }
        }
        let elsewhere = Hex::new(7, -3);
        assert!(in_arc(elsewhere, elsewhere, 80, 90));
    }

    #[test]
    fn complementary_arcs_cover_every_hex() {
        // Every hex is in arc A or in its complement (edges belong to both).
        for q in -6..=6 {
            for r in -6..=6 {
                let hex = Hex::new(q, r);
                assert!(
                    in_arc(BASE, hex, 40, 200) || in_arc(BASE, hex, 200, 40),
                    "{hex}"
                );
            }
        }
    }

    #[test]
    fn the_arc_is_relative_to_the_base() {
        let base = Hex::new(10, -4);
        assert!(in_arc(base, Hex::new(15, -4), 80, 100));
        assert!(!in_arc(base, Hex::new(5, -4), 80, 100));
    }
}
