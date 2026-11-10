//! Reading polygons from a shapefile's .shp geometry. Attributes (.dbf) are
//! never opened.

use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use shapefile::{PolygonRing, Shape, ShapeReader};

use crate::land::{Polygon, Ring};

fn rings<P>(
    polygon: &shapefile::record::polygon::GenericPolygon<P>,
    xy: impl Fn(&P) -> (f64, f64),
) -> Polygon {
    Polygon {
        rings: polygon
            .rings()
            .iter()
            .map(|ring| {
                let points: &[P] = match ring {
                    PolygonRing::Outer(points) | PolygonRing::Inner(points) => points,
                };
                Ring {
                    points: points.iter().map(&xy).collect(),
                }
            })
            .collect(),
    }
}

/// Reads every shape of a .shp file as a polygon (x = longitude, y =
/// latitude). Polygon, PolygonZ and PolygonM are accepted (x and y are
/// taken); any other shape type is an error.
pub fn read_polygons(path: &Path) -> Result<Vec<Polygon>, String> {
    let file = File::open(path).map_err(|e| format!("{}: {e}", path.display()))?;
    let mut reader =
        ShapeReader::new(BufReader::new(file)).map_err(|e| format!("{}: {e}", path.display()))?;
    let mut polygons = Vec::new();
    for (index, shape) in reader.iter_shapes().enumerate() {
        let shape = shape.map_err(|e| format!("{}: shape {index}: {e}", path.display()))?;
        polygons.push(match shape {
            Shape::Polygon(p) => rings(&p, |pt| (pt.x, pt.y)),
            Shape::PolygonZ(p) => rings(&p, |pt| (pt.x, pt.y)),
            Shape::PolygonM(p) => rings(&p, |pt| (pt.x, pt.y)),
            other => {
                return Err(format!(
                    "{}: shape {index} is a {}, not a polygon",
                    path.display(),
                    other.shapetype()
                ))
            }
        });
    }
    Ok(polygons)
}
