//! The `water` command: why a position's hex has the water it has, worked
//! out from the source data with the same functions as generation.

use std::collections::BTreeMap;

use crate::generate::{analyse_hex, lattice_hex, lattice_nm, lattice_point_is_land, HexAnalysis};
use crate::land::{LandIndex, Polygon};
use crate::projection::{degrees_text, hex_of, to_nm};

/// How far from the position to look for the nearest sea point, in nm.
pub const SEARCH_NM: f64 = 6.0;
/// Polygons are kept when they come within this many degrees of the point:
/// more than a hex and its neighbours (40 nm each).
const KEEP_DEGREES: f64 = 2.5;

/// Keeps the polygons whose bounding box comes near the position.
pub fn keep_near_point(polygons: Vec<Polygon>, lat: f64, lon: f64) -> Vec<Polygon> {
    polygons
        .into_iter()
        .filter(|polygon| {
            polygon.bbox().is_some_and(|b| {
                b.2 >= lon - KEEP_DEGREES
                    && b.0 <= lon + KEEP_DEGREES
                    && b.3 >= lat - KEEP_DEGREES
                    && b.1 <= lat + KEEP_DEGREES
            })
        })
        .collect()
}

/// The nearest lattice point to (east, north) within [`SEARCH_NM`] for which
/// `is_sea` holds: nearest, ties by north (larger j) then west (smaller i).
/// Any hex. Returns the point and its squared distance.
pub fn nearest_sea_point(
    east: f64,
    north: f64,
    is_sea: &mut dyn FnMut(i32, i32) -> bool,
) -> Option<((i32, i32), f64)> {
    let step = lattice_nm(1, 0).0;
    let i_low = ((east - SEARCH_NM) / step).floor() as i32 - 1;
    let i_high = ((east + SEARCH_NM) / step).floor() as i32 + 1;
    let j_low = ((north - SEARCH_NM) / step).floor() as i32 - 1;
    let j_high = ((north + SEARCH_NM) / step).floor() as i32 + 1;
    let mut best: Option<((i32, i32), f64)> = None;
    for i in i_low..=i_high {
        for j in j_low..=j_high {
            let (pe, pn) = lattice_nm(i, j);
            let distance_squared = (pe - east) * (pe - east) + (pn - north) * (pn - north);
            if distance_squared > SEARCH_NM * SEARCH_NM || !is_sea(i, j) {
                continue;
            }
            let better = match best {
                None => true,
                Some(((bi, bj), d)) => {
                    distance_squared < d
                        || (distance_squared == d && (j > bj || (j == bj && i < bi)))
                }
            };
            if better {
                best = Some(((i, j), distance_squared));
            }
        }
    }
    best
}

/// The report of the `water` command, as text. `polygons` may be all the
/// land or only what is near the point.
pub fn report(lat: f64, lon: f64, polygons: &[Polygon]) -> String {
    let index = LandIndex::new(&keep_near_point(polygons.to_vec(), lat, lon));
    let is_land = |i: i32, j: i32| lattice_point_is_land(&index, i, j);
    let mut cache: BTreeMap<(i32, i32), HexAnalysis> = BTreeMap::new();
    let mut analysis_of = |hex: (i32, i32)| -> HexAnalysis {
        cache
            .entry(hex)
            .or_insert_with(|| analyse_hex(hex.0, hex.1, &is_land))
            .clone()
    };

    let (east, north) = to_nm(lat, lon);
    let hex = hex_of(lat, lon);
    let mut out = String::new();
    out.push_str(&format!(
        "lat {}, lon {}: east {east:.3} nm, north {north:.3} nm from hex (0,0)\n",
        degrees_text(lat),
        degrees_text(lon)
    ));
    let here = analysis_of(hex);
    let points = here.points.len();
    out.push_str(&format!(
        "hex ({},{}): {points} lattice points, {} land, {} sea\n",
        hex.0,
        hex.1,
        here.land_points,
        points - here.land_points as usize
    ));
    let mut order: Vec<usize> = (0..here.bodies.len()).collect();
    order.sort_by(|a, b| {
        let (x, y) = (&here.bodies[*a], &here.bodies[*b]);
        if x.beats(y) {
            core::cmp::Ordering::Less
        } else {
            core::cmp::Ordering::Greater
        }
    });
    out.push_str(&format!("bodies of water: {}\n", here.bodies.len()));
    for (rank, &body) in order.iter().enumerate() {
        let b = &here.bodies[body];
        out.push_str(&format!(
            "  {}. {} points, northernmost point (i,j) = ({},{}){}\n",
            rank + 1,
            b.size,
            b.top.0,
            b.top.1,
            if Some(body) == here.main {
                "  <- main"
            } else {
                ""
            }
        ));
    }

    let mut is_sea = |i: i32, j: i32| {
        let owner = lattice_hex(i, j);
        analysis_of(owner).body_of.contains_key(&(i, j))
    };
    match nearest_sea_point(east, north, &mut is_sea) {
        None => out.push_str(&format!(
            "nearest sea point within {SEARCH_NM} nm: absent\n"
        )),
        Some(((i, j), distance_squared)) => {
            let (pe, pn) = lattice_nm(i, j);
            let owner = lattice_hex(i, j);
            let analysis = analysis_of(owner);
            let body = analysis.body_of[&(i, j)];
            let size = analysis.bodies[body].size;
            out.push_str(&format!(
                "nearest sea point within {SEARCH_NM} nm: lattice point ({i},{j}), offset east {:.3}, north {:.3} nm, distance {:.3} nm\n",
                pe - east,
                pn - north,
                distance_squared.sqrt()
            ));
            let in_main = analysis.main == Some(body);
            if owner == hex {
                if in_main {
                    out.push_str("  in the position's hex, in its main body\n");
                } else {
                    out.push_str(&format!(
                        "  in the position's hex, in another body of it ({size} points), not the main body\n"
                    ));
                }
            } else {
                out.push_str(&format!(
                    "  in another hex, ({},{}), in {} ({size} points)\n",
                    owner.0,
                    owner.1,
                    if in_main {
                        "that hex's main body"
                    } else {
                        "a body that is not that hex's main body"
                    }
                ));
                out.push_str(&format!(
                    "  note: the nearest sea point is in hex ({},{}), not the position's own hex ({},{}); an anchorage (decision 0017, decision 6) searches only its own hex's sea points, so it may pick another body\n",
                    owner.0, owner.1, hex.0, hex.1
                ));
            }
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::land::Ring;
    use crate::projection::from_nm;

    fn rect_nm(east: (f64, f64), north: (f64, f64)) -> Polygon {
        let corner = |e: f64, n: f64| {
            let (lat, lon) = from_nm(e, n);
            (lon, lat)
        };
        Polygon {
            rings: vec![Ring {
                points: vec![
                    corner(east.0, north.0),
                    corner(east.1, north.0),
                    corner(east.1, north.1),
                    corner(east.0, north.1),
                    corner(east.0, north.0),
                ],
            }],
        }
    }

    #[test]
    fn the_nearest_sea_point_breaks_ties_north_then_west() {
        // A position exactly between four lattice points (east 1, north 1):
        // all four are sqrt 2 away. North wins, then west: (0,1).
        let mut sea = |_: i32, _: i32| true;
        let (point, d) = nearest_sea_point(1.0, 1.0, &mut sea).unwrap();
        assert_eq!(point, (0, 1));
        assert_eq!(d, 2.0);
        // Only (1,0) and (1,1) are sea: (1,1) is further north.
        let mut some = |i: i32, j: i32| (i, j) == (1, 0) || (i, j) == (1, 1);
        assert_eq!(nearest_sea_point(1.0, 1.0, &mut some).unwrap().0, (1, 1));
        // Nothing within 6 nm is sea.
        let mut none = |_: i32, _: i32| false;
        assert_eq!(nearest_sea_point(1.0, 1.0, &mut none), None);
        // Exactly 6 nm away counts; 6.1 does not.
        let mut far = |i: i32, j: i32| (i, j) == (3, 0);
        assert!(nearest_sea_point(0.0, 0.0, &mut far).is_some());
        assert!(nearest_sea_point(-0.1, 0.0, &mut far).is_none());
    }

    #[test]
    fn the_report_of_an_island_strip() {
        // A land strip across hex (0,0), north-south along east = 0, 2 nm
        // wide (the lattice column i = 0), from far south to far north.
        let land = rect_nm((-1.0, 1.0), (-200.0, 200.0));
        let (lat, lon) = from_nm(10.0, 0.0);
        let text = report(lat, lon, std::slice::from_ref(&land));
        // 23 land points, 166 on the west and 155 on the east.
        assert!(
            text.contains("hex (0,0): 344 lattice points, 23 land, 321 sea"),
            "{text}"
        );
        assert!(text.contains("bodies of water: 2"), "{text}");
        assert!(text.contains("1. 166 points"), "{text}");
        assert!(text.contains("<- main"), "{text}");
        // The position is 10 nm east of the strip: its nearest sea point is
        // itself, (5,0), in the east body, which is not the main body.
        assert!(text.contains("lattice point (5,0)"), "{text}");
        assert!(text.contains("another body of it (155 points)"), "{text}");
        // On the strip: the nearest sea points are 2 nm either side;
        // east and west are the same distance, the western wins the tie.
        let (lat, lon) = from_nm(0.0, 0.0);
        let text = report(lat, lon, std::slice::from_ref(&land));
        assert!(text.contains("lattice point (-1,0)"), "{text}");
        assert!(text.contains("in its main body"), "{text}");
    }

    #[test]
    fn an_all_land_position_has_no_sea_point() {
        let land = rect_nm((-3000.0, 3000.0), (-3000.0, 3000.0));
        let (lat, lon) = from_nm(0.0, 0.0);
        let text = report(lat, lon, std::slice::from_ref(&land));
        assert!(text.contains("bodies of water: 0"), "{text}");
        assert!(text.contains("absent"), "{text}");
    }

    #[test]
    fn the_report_notes_when_the_nearest_sea_point_is_in_another_hex() {
        // Land over the east half of hex (0,0) and the west edge of (1,0):
        // east in (-30, 20.5). A position at east 19 is land; the nearest
        // sea points are at east 22 or more, in hex (1,0).
        let land = rect_nm((-30.0, 20.5), (-10.0, 10.0));
        let (lat, lon) = from_nm(19.0, 0.0);
        let text = report(lat, lon, std::slice::from_ref(&land));
        assert!(text.contains("in another hex, (1,0)"), "{text}");
        assert!(
            text.contains("note: the nearest sea point is in hex (1,0)"),
            "{text}"
        );
        // In its own hex there is no note.
        let (lat, lon) = from_nm(10.0, 0.0);
        let sea = rect_nm((-1.0, 1.0), (-200.0, 200.0));
        assert!(!report(lat, lon, std::slice::from_ref(&sea)).contains("note:"));
    }
}
