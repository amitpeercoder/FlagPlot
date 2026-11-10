//! Land polygons and the point-in-polygon test.
//!
//! Points are (longitude, latitude) in degrees. A [`Polygon`] is one shape
//! record: its rings (outer rings and holes alike) are combined with the
//! even-odd rule, so holes work. A point is land when any polygon contains it.

/// One ring of points as (longitude, latitude), as read from a shape.
#[derive(Clone, Debug)]
pub struct Ring {
    pub points: Vec<(f64, f64)>,
}

/// One shape record: all its rings.
#[derive(Clone, Debug)]
pub struct Polygon {
    pub rings: Vec<Ring>,
}

/// A bounding box: (min lon, min lat, max lon, max lat).
pub type BBox = (f64, f64, f64, f64);

impl Polygon {
    pub fn bbox(&self) -> Option<BBox> {
        let mut points = self.rings.iter().flat_map(|ring| ring.points.iter());
        let first = points.next()?;
        let mut b = (first.0, first.1, first.0, first.1);
        for &(x, y) in points {
            b = (b.0.min(x), b.1.min(y), b.2.max(x), b.3.max(y));
        }
        Some(b)
    }
}

/// How many latitude bands a ring's edges are bucketed into, at most.
const MAX_BANDS: usize = 4096;

struct IndexedRing {
    bbox: BBox,
    points: Vec<(f64, f64)>,
    band_height: f64,
    /// For each latitude band, the edges (by index of their first point)
    /// whose latitude span reaches the band.
    bands: Vec<Vec<u32>>,
}

impl IndexedRing {
    fn new(ring: &Ring) -> Option<IndexedRing> {
        let points = &ring.points;
        if points.len() < 3 {
            return None;
        }
        let mut bbox = (points[0].0, points[0].1, points[0].0, points[0].1);
        for &(x, y) in points {
            bbox = (bbox.0.min(x), bbox.1.min(y), bbox.2.max(x), bbox.3.max(y));
        }
        let height = bbox.3 - bbox.1;
        let count = (points.len() / 4).clamp(1, MAX_BANDS);
        let band_height = if height > 0.0 {
            height / count as f64
        } else {
            1.0
        };
        let mut ring = IndexedRing {
            bbox,
            points: points.clone(),
            band_height,
            bands: vec![Vec::new(); count],
        };
        let n = ring.points.len();
        for i in 0..n {
            let (_, y1) = ring.points[i];
            let (_, y2) = ring.points[(i + 1) % n];
            let lo = ring.band_of(y1.min(y2));
            let hi = ring.band_of(y1.max(y2));
            for band in lo..=hi {
                ring.bands[band].push(i as u32);
            }
        }
        Some(ring)
    }

    fn band_of(&self, y: f64) -> usize {
        let band = ((y - self.bbox.1) / self.band_height).floor();
        if band <= 0.0 {
            0
        } else {
            (band as usize).min(self.bands.len() - 1)
        }
    }

    /// Whether a ray from the point towards +longitude crosses the ring an
    /// odd number of times. Only edges whose latitude span reaches the
    /// point's band can be crossed, so the result is the same as testing
    /// every edge.
    fn contains(&self, x: f64, y: f64) -> bool {
        if x < self.bbox.0 || x > self.bbox.2 || y < self.bbox.1 || y > self.bbox.3 {
            return false;
        }
        let n = self.points.len();
        let mut inside = false;
        for &i in &self.bands[self.band_of(y)] {
            let (x1, y1) = self.points[i as usize];
            let (x2, y2) = self.points[(i as usize + 1) % n];
            if (y1 > y) != (y2 > y) && x < (x2 - x1) * (y - y1) / (y2 - y1) + x1 {
                inside = !inside;
            }
        }
        inside
    }
}

struct IndexedPolygon {
    bbox: BBox,
    rings: Vec<IndexedRing>,
}

/// A set of polygons ready for many point tests.
pub struct LandIndex {
    polygons: Vec<IndexedPolygon>,
}

impl LandIndex {
    pub fn new(polygons: &[Polygon]) -> LandIndex {
        let polygons = polygons
            .iter()
            .filter_map(|polygon| {
                let bbox = polygon.bbox()?;
                let rings = polygon.rings.iter().filter_map(IndexedRing::new).collect();
                Some(IndexedPolygon { bbox, rings })
            })
            .collect();
        LandIndex { polygons }
    }

    /// Whether the point (longitude `x`, latitude `y`) is inside any polygon,
    /// by the even-odd rule over each polygon's rings.
    pub fn contains(&self, x: f64, y: f64) -> bool {
        self.polygons.iter().any(|polygon| {
            let b = polygon.bbox;
            if x < b.0 || x > b.2 || y < b.1 || y > b.3 {
                return false;
            }
            polygon
                .rings
                .iter()
                .filter(|ring| ring.contains(x, y))
                .count()
                % 2
                == 1
        })
    }
}

/// Reference implementation: every edge of every ring, no index.
#[cfg(test)]
fn brute_force_contains(polygon: &Polygon, x: f64, y: f64) -> bool {
    let mut inside = false;
    for ring in &polygon.rings {
        let n = ring.points.len();
        for i in 0..n {
            let (x1, y1) = ring.points[i];
            let (x2, y2) = ring.points[(i + 1) % n];
            if (y1 > y) != (y2 > y) && x < (x2 - x1) * (y - y1) / (y2 - y1) + x1 {
                inside = !inside;
            }
        }
    }
    inside
}

#[cfg(test)]
mod tests {
    use super::*;

    fn square(x0: f64, y0: f64, x1: f64, y1: f64) -> Ring {
        Ring {
            points: vec![(x0, y0), (x1, y0), (x1, y1), (x0, y1), (x0, y0)],
        }
    }

    #[test]
    fn a_hole_is_not_land() {
        let polygon = Polygon {
            rings: vec![square(0.0, 0.0, 10.0, 10.0), square(4.0, 4.0, 6.0, 6.0)],
        };
        let index = LandIndex::new(std::slice::from_ref(&polygon));
        assert!(index.contains(1.0, 1.0));
        assert!(index.contains(9.0, 5.0));
        assert!(index.contains(3.9, 5.0));
        assert!(!index.contains(5.0, 5.0));
        assert!(!index.contains(11.0, 5.0));
        assert!(!index.contains(-1.0, 5.0));
        assert!(!index.contains(5.0, 11.0));
    }

    #[test]
    fn overlapping_polygons_are_a_union_not_a_cancelling() {
        let a = Polygon {
            rings: vec![square(0.0, 0.0, 10.0, 10.0)],
        };
        let b = Polygon {
            rings: vec![square(5.0, 5.0, 15.0, 15.0)],
        };
        let index = LandIndex::new(&[a, b]);
        assert!(index.contains(7.0, 7.0));
        assert!(index.contains(2.0, 2.0));
        assert!(index.contains(12.0, 12.0));
        assert!(!index.contains(12.0, 2.0));
    }

    /// The band index gives the same answer as testing every edge, on a
    /// ragged star shape with many vertices.
    #[test]
    fn the_band_index_matches_brute_force() {
        let mut points = Vec::new();
        for i in 0..200 {
            // Spiky pseudo-circle using only exact arithmetic on integers.
            let angle = f64::from(i) / 200.0 * 8.0;
            let radius = if i % 2 == 0 { 9.0 } else { 4.0 };
            // A triangle-wave "cosine/sine" keeps this libm-free.
            let tri = |t: f64| {
                let t = t - (t / 8.0).floor() * 8.0;
                if t < 4.0 {
                    t - 2.0
                } else {
                    6.0 - t
                }
            };
            points.push((radius * tri(angle + 2.0) / 2.0, radius * tri(angle) / 2.0));
        }
        let polygon = Polygon {
            rings: vec![Ring { points }, square(-1.0, -1.0, 1.0, 1.0)],
        };
        let index = LandIndex::new(std::slice::from_ref(&polygon));
        let mut inside = 0;
        for i in 0..=100 {
            for j in 0..=100 {
                let (x, y) = (
                    -10.0 + f64::from(i) * 0.2,
                    -10.0 + f64::from(j) * 0.2 + 0.013,
                );
                let expected = brute_force_contains(&polygon, x, y);
                assert_eq!(index.contains(x, y), expected, "at {x},{y}");
                inside += usize::from(expected);
            }
        }
        assert!(inside > 500 && inside < 10_000, "{inside}");
    }
}
