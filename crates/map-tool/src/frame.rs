//! A map's frame (decision 0199): where it lies on the earth. The latitude
//! and longitude of the centre of hex (0,0), how it is flattened (decision
//! 0242, rule 10: of one factor, the cosine of the centre's latitude as a
//! literal to six places, or about a centre, with a scale), the extent, and
//! for a map being made its title, its date and its land marks.
//!
//! The map tool alone reads a frame file (`data/map/frames/<name>.toml`),
//! and makes a map from it. With none it makes the present map, from
//! [`Frame::south_pacific`], the one place a map's frame is in code. Every
//! other reader takes a map's frame from that map's header, through
//! `sim_data::read_map_frame` ([`Frame::of_map`] here).
//!
//! Longitude runs on without a break across the date line inside the tool
//! (rule 23 of decision 0199): a frame whose eastern edge is west of its
//! western edge has its eastern edge, and any position west of its western
//! edge, raised by 360. The map file's longitude column stays west negative,
//! between -180 and 180 (rule 22).

use std::path::Path;

use serde::Deserialize;

use crate::projection::Projection;

/// A number of degrees as the frame file gives it: the text, kept for the
/// header, and its value.
#[derive(Clone, Debug, PartialEq)]
pub struct Degrees {
    pub text: String,
    pub value: f64,
}

impl Degrees {
    /// Reads plain decimal text: an optional `-`, digits, an optional
    /// fraction. No exponent, no `+`, nothing else.
    pub fn parse(name: &str, text: &str) -> Result<Degrees, String> {
        let digits = text.strip_prefix('-').unwrap_or(text);
        let (whole, fraction) = digits.split_once('.').unwrap_or((digits, ""));
        let plain = |s: &str| s.bytes().all(|b| b.is_ascii_digit());
        let fraction_ok = !digits.contains('.') || !fraction.is_empty();
        if whole.is_empty() || !plain(whole) || !plain(fraction) || !fraction_ok {
            return Err(format!("{name} \"{text}\" is not a plain decimal"));
        }
        let value: f64 = text
            .parse()
            .map_err(|_| format!("{name} \"{text}\" is not a number"))?;
        Ok(Degrees {
            text: text.to_string(),
            value,
        })
    }

    /// The text without its sign.
    fn magnitude(&self) -> &str {
        self.text.strip_prefix('-').unwrap_or(&self.text)
    }

    /// As a latitude for a person: "12 S", "28.5 N".
    pub fn latitude_words(&self) -> String {
        let hemisphere = if self.value < 0.0 { "S" } else { "N" };
        format!("{} {hemisphere}", self.magnitude())
    }

    /// As a longitude for a person: "160 E", "166 W".
    pub fn longitude_words(&self) -> String {
        let hemisphere = if self.value < 0.0 { "W" } else { "E" };
        format!("{} {hemisphere}", self.magnitude())
    }
}

/// A place made land enough (decision 0199, 27 and 28): its hex, if no
/// lattice point of it is land, is coast with land percentage 0.
#[derive(Clone, Debug, PartialEq)]
pub struct LandMark {
    pub name: String,
    pub lat: Degrees,
    pub lon: Degrees,
    /// The id of the row of `data/oob/place_positions.csv` the position is
    /// from.
    pub row: String,
}

/// How a frame flattens the earth (decision 0242, rule 10).
#[derive(Clone, Debug, PartialEq)]
pub enum Kind {
    /// East is the difference of longitude times 60 times this factor, the
    /// cosine of the centre's latitude (decisions 0017 and 0199).
    OneFactor(Degrees),
    /// About the centre (decision 0242, rule 3); this is the scale.
    AboutCentre(Degrees),
}

/// The words a frame file gives for each kind.
const KIND_ONE_FACTOR: &str = "one factor";
const KIND_ABOUT_A_CENTRE: &str = "about a centre";
/// The miles in a radian: the literal of a frame about a centre.
const MILES_IN_A_RADIAN_TEXT: &str = "3437.746771";

/// A map's frame.
#[derive(Clone, Debug, PartialEq)]
pub struct Frame {
    /// The first line of the map file's header.
    pub title: String,
    /// The date in the header's last line. A literal: no wall clock.
    pub date: String,
    pub centre_lat: Degrees,
    /// As the frame gives it, west negative.
    pub centre_lon: Degrees,
    /// The kind of frame, with its factor or its scale.
    pub kind: Kind,
    pub north: Degrees,
    pub south: Degrees,
    /// As the frame gives them, west negative.
    pub west: Degrees,
    pub east: Degrees,
    pub marks: Vec<LandMark>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct FrameFile {
    title: String,
    date: String,
    centre_lat: String,
    centre_lon: String,
    /// "one factor" or "about a centre"; none is the first.
    #[serde(default)]
    kind: Option<String>,
    #[serde(default)]
    factor: Option<String>,
    #[serde(default)]
    scale: Option<String>,
    north: String,
    south: String,
    west: String,
    east: String,
    #[serde(default)]
    land_mark: Vec<LandMarkFile>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct LandMarkFile {
    name: String,
    lat: String,
    lon: String,
    row: String,
}

impl Frame {
    /// The present map's frame (decision 0017, 2), the map tool's default:
    /// the frame `data/map/south_pacific.csv` was made with on 2026-11-10.
    pub fn south_pacific() -> Frame {
        let degrees = |text: &str| Degrees::parse("default", text).expect("a plain decimal");
        Frame {
            title: "Flag Plot map of the south-west Pacific slice (decision 0017).".to_string(),
            date: "2026-11-10".to_string(),
            centre_lat: degrees("-12"),
            centre_lon: degrees("160"),
            kind: Kind::OneFactor(degrees("0.978148")),
            north: degrees("-1"),
            south: degrees("-24"),
            west: degrees("145"),
            east: degrees("172"),
            marks: Vec::new(),
        }
    }

    /// Reads a frame file: TOML, every quantity a string.
    pub fn parse(text: &str) -> Result<Frame, String> {
        let file: FrameFile = toml::from_str(text).map_err(|e| e.to_string())?;
        let marks = file
            .land_mark
            .iter()
            .map(|mark| {
                if mark.name.is_empty() {
                    return Err("a land mark has no name".to_string());
                }
                if mark.row.is_empty() || !mark.row.bytes().all(|b| b.is_ascii_digit()) {
                    return Err(format!(
                        "land mark {}: row \"{}\" is not a row id",
                        mark.name, mark.row
                    ));
                }
                Ok(LandMark {
                    name: mark.name.clone(),
                    lat: Degrees::parse(&format!("land mark {} lat", mark.name), &mark.lat)?,
                    lon: Degrees::parse(&format!("land mark {} lon", mark.name), &mark.lon)?,
                    row: mark.row.clone(),
                })
            })
            .collect::<Result<Vec<_>, String>>()?;
        let kind = match (file.kind.as_deref(), &file.factor, &file.scale) {
            (None | Some(KIND_ONE_FACTOR), Some(factor), None) => {
                Kind::OneFactor(Degrees::parse("factor", factor)?)
            }
            (None | Some(KIND_ONE_FACTOR), None, None) => {
                return Err("a frame of one factor needs a factor".to_string())
            }
            (None | Some(KIND_ONE_FACTOR), _, Some(_)) => {
                return Err("a frame of one factor takes a factor, and no scale".to_string())
            }
            (Some(KIND_ABOUT_A_CENTRE), None, Some(scale)) => {
                Kind::AboutCentre(Degrees::parse("scale", scale)?)
            }
            (Some(KIND_ABOUT_A_CENTRE), None, None) => {
                return Err("a frame about a centre needs a scale".to_string())
            }
            (Some(KIND_ABOUT_A_CENTRE), Some(_), _) => {
                return Err("a frame about a centre takes a scale, and no factor".to_string())
            }
            (Some(other), _, _) => {
                return Err(format!(
                    "kind \"{other}\" is not \"{KIND_ONE_FACTOR}\" or \"{KIND_ABOUT_A_CENTRE}\""
                ))
            }
        };
        let frame = Frame {
            title: file.title,
            date: file.date,
            centre_lat: Degrees::parse("centre_lat", &file.centre_lat)?,
            centre_lon: Degrees::parse("centre_lon", &file.centre_lon)?,
            kind,
            north: Degrees::parse("north", &file.north)?,
            south: Degrees::parse("south", &file.south)?,
            west: Degrees::parse("west", &file.west)?,
            east: Degrees::parse("east", &file.east)?,
            marks,
        };
        frame.check()?;
        Ok(frame)
    }

    /// Reads a frame file from disk.
    pub fn read(path: &Path) -> Result<Frame, String> {
        let text = std::fs::read_to_string(path).map_err(|e| format!("{}: {e}", path.display()))?;
        Frame::parse(&text).map_err(|e| format!("{}: {e}", path.display()))
    }

    /// The frame of a map, read from its header by `sim-data`; `None` for a
    /// map with no frame (a hand-written map). It has no title, date or
    /// marks: only the projection and the extent.
    pub fn of_map(path: &Path) -> Result<Option<Frame>, String> {
        let Some(frame) = sim_data::read_map_frame(path).map_err(|e| e.to_string())? else {
            return Ok(None);
        };
        let degrees =
            |name: &str, value: sim_core::fixed::Fixed| Degrees::parse(name, &value.to_string());
        // The header's longitudes are in the run; the frame keeps the
        // centre and the eastern edge west negative, as a frame file does.
        let back = |name: &str, value: sim_core::fixed::Fixed| {
            let text = value.to_string();
            let run = Degrees::parse(name, &text)?;
            if run.value > 180.0 {
                let shifted = value - sim_core::fixed::Fixed::from_int(360);
                degrees(name, shifted)
            } else {
                Ok(run)
            }
        };
        let frame = Frame {
            title: String::new(),
            date: String::new(),
            centre_lat: degrees("centre_lat", frame.centre_lat)?,
            centre_lon: back("centre_lon", frame.centre_lon)?,
            kind: match frame.projection {
                sim_data::MapProjection::OneFactor { factor } => {
                    Kind::OneFactor(degrees("factor", factor)?)
                }
                sim_data::MapProjection::AboutCentre { scale } => {
                    Kind::AboutCentre(degrees("scale", scale)?)
                }
            },
            north: degrees("north", frame.north)?,
            south: degrees("south", frame.south)?,
            west: degrees("west", frame.west)?,
            east: back("east", frame.east)?,
            marks: Vec::new(),
        };
        frame.check()?;
        Ok(Some(frame))
    }

    /// The frame of a map, or the default frame for a map with none, as
    /// every map was read before decision 0199.
    pub fn of_map_or_default(path: &Path) -> Result<Frame, String> {
        Ok(Frame::of_map(path)?.unwrap_or_else(Frame::south_pacific))
    }

    fn check(&self) -> Result<(), String> {
        let (name, number) = match &self.kind {
            Kind::OneFactor(factor) => ("factor", factor),
            Kind::AboutCentre(scale) => ("scale", scale),
        };
        if !(number.value > 0.0 && number.value <= 1.0) {
            return Err(format!("{name} {} is not between 0 and 1", number.text));
        }
        for (name, lat) in [
            ("centre_lat", &self.centre_lat),
            ("north", &self.north),
            ("south", &self.south),
        ] {
            if lat.value.abs() >= 90.0 {
                return Err(format!("{name} {} is not a latitude", lat.text));
            }
        }
        for (name, lon) in [
            ("centre_lon", &self.centre_lon),
            ("west", &self.west),
            ("east", &self.east),
        ] {
            if lon.value.abs() > 180.0 {
                return Err(format!(
                    "{name} {} is not a longitude from -180 to 180",
                    lon.text
                ));
            }
        }
        if self.north.value <= self.south.value {
            return Err(format!(
                "north {} is not north of south {}",
                self.north.text, self.south.text
            ));
        }
        let (west, east) = (self.west_run(), self.east_run());
        if east - west >= 360.0 || east <= west {
            return Err("the extent's longitudes do not make a span".to_string());
        }
        let centre = self.lon_in_run(self.centre_lon.value);
        if !(self.south.value..=self.north.value).contains(&self.centre_lat.value)
            || !(west..=east).contains(&centre)
        {
            return Err("the centre is not inside the extent".to_string());
        }
        for (k, mark) in self.marks.iter().enumerate() {
            if self.marks[..k].iter().any(|m| m.name == mark.name) {
                return Err(format!("land mark {} is listed twice", mark.name));
            }
            if mark.lat.value.abs() >= 90.0 || mark.lon.value.abs() > 180.0 {
                return Err(format!("land mark {} is not a position", mark.name));
            }
        }
        Ok(())
    }

    /// Whether the frame crosses the date line: its eastern edge is given
    /// west of its western edge.
    pub fn crosses_date_line(&self) -> bool {
        self.east.value < self.west.value
    }

    /// The western edge, in the run.
    pub fn west_run(&self) -> f64 {
        self.west.value
    }

    /// The eastern edge, in the run: raised by 360 when the frame crosses
    /// the date line.
    pub fn east_run(&self) -> f64 {
        if self.crosses_date_line() {
            self.east.value + 360.0
        } else {
            self.east.value
        }
    }

    /// A longitude brought into the frame's run: raised by 360 when it is
    /// west of the frame's western edge and the frame crosses the date
    /// line. A frame that does not cross leaves every longitude as it is.
    pub fn lon_in_run(&self, lon: f64) -> f64 {
        if self.crosses_date_line() && lon < self.west.value {
            lon + 360.0
        } else {
            lon
        }
    }

    /// A longitude of the run as the map file writes it: west negative,
    /// from -180 to 180.
    pub fn lon_for_file(&self, lon: f64) -> f64 {
        if lon > 180.0 {
            lon - 360.0
        } else {
            lon
        }
    }

    /// Whether the frame is about a centre (decision 0242).
    pub fn is_about_a_centre(&self) -> bool {
        matches!(self.kind, Kind::AboutCentre(_))
    }

    /// The projection of the frame: hex (0,0)'s centre and the factor or the
    /// scale.
    pub fn projection(&self) -> Projection {
        let (factor, about_centre) = match &self.kind {
            Kind::OneFactor(factor) => (factor.value, false),
            Kind::AboutCentre(scale) => (scale.value, true),
        };
        Projection {
            origin_lat: self.centre_lat.value,
            origin_lon: self.lon_in_run(self.centre_lon.value),
            factor,
            about_centre,
        }
    }

    /// The centre's longitude in the run, as text for the header's formula.
    fn centre_lon_run_text(&self) -> String {
        let run = self.lon_in_run(self.centre_lon.value);
        if run == self.centre_lon.value {
            self.centre_lon.text.clone()
        } else {
            format!("{run}")
        }
    }

    /// The header's lines of the projection and the extent, without the
    /// `# `: exactly the present map's lines, with this frame's numbers
    /// (decision 0199, 17). The two lines between them, the hex centres and
    /// the nearest centre, are the same for every map and are not here.
    pub fn projection_lines(&self) -> [String; 3] {
        let factor = match &self.kind {
            Kind::OneFactor(factor) => factor,
            Kind::AboutCentre(scale) => {
                // Decision 0242, rule 12: exactly these lines, with the
                // frame's own numbers.
                return [
                    format!(
                        "Projection: about a centre. Hex (0,0) is centred at {}, {}.",
                        self.centre_lat.latitude_words(),
                        self.centre_lon.longitude_words()
                    ),
                    format!(
                        "  distance_nm = angle from the centre in radians * {MILES_IN_A_RADIAN_TEXT} * {}, on the bearing from the centre",
                        scale.text
                    ),
                    "  east_nm = distance_nm * sin(bearing), north_nm = distance_nm * cos(bearing); a sphere of 60 nm to the degree.".to_string(),
                ];
            }
        };
        let lat = &self.centre_lat;
        let north_formula = if lat.value < 0.0 {
            format!("(lat + {})", lat.magnitude())
        } else {
            format!("(lat - {})", lat.magnitude())
        };
        [
            format!(
                "Projection: hex (0,0) is centred at {}, {}.",
                lat.latitude_words(),
                self.centre_lon.longitude_words()
            ),
            format!(
                "  east_nm  = (lon - {}) * 60 * {}   ({} is cos {} degrees)",
                self.centre_lon_run_text(),
                factor.text,
                factor.text,
                lat.magnitude()
            ),
            format!("  north_nm = {north_formula} * 60"),
        ]
    }

    /// The header's extent line, without the `# `.
    pub fn extent_line(&self) -> String {
        format!(
            "Extent: every hex whose centre lies from {} to {} and from {} to {}.",
            self.north.latitude_words(),
            self.south.latitude_words(),
            self.west.longitude_words(),
            self.east.longitude_words()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_default_frame_writes_the_present_header_lines() {
        let frame = Frame::south_pacific();
        assert_eq!(
            frame.projection_lines(),
            [
                "Projection: hex (0,0) is centred at 12 S, 160 E.".to_string(),
                "  east_nm  = (lon - 160) * 60 * 0.978148   (0.978148 is cos 12 degrees)"
                    .to_string(),
                "  north_nm = (lat + 12) * 60".to_string(),
            ]
        );
        assert_eq!(
            frame.extent_line(),
            "Extent: every hex whose centre lies from 1 S to 24 S and from 145 E to 172 E."
        );
        let projection = frame.projection();
        assert_eq!(
            (
                projection.origin_lat,
                projection.origin_lon,
                projection.factor
            ),
            (-12.0, 160.0, 0.978148)
        );
        assert!(!frame.crosses_date_line());
        assert_eq!(frame.lon_in_run(-170.0), -170.0);
    }

    const MIDWAY: &str = "title = \"A test frame.\"\n\
        date = \"2026-11-20\"\n\
        centre_lat = \"28.5\"\n\
        centre_lon = \"178.5\"\n\
        factor = \"0.878817\"\n\
        north = \"38\"\n\
        south = \"18\"\n\
        west = \"163\"\n\
        east = \"-166\"\n\
        [[land_mark]]\n\
        name = \"midway\"\n\
        lat = \"28.232\"\n\
        lon = \"-177.370\"\n\
        row = \"5100\"\n";

    #[test]
    fn a_frame_across_the_date_line_runs_longitude_on() {
        let frame = Frame::parse(MIDWAY).unwrap();
        assert!(frame.crosses_date_line());
        assert_eq!((frame.west_run(), frame.east_run()), (163.0, 194.0));
        assert_eq!(frame.lon_in_run(-177.37), 360.0 - 177.37);
        assert_eq!(frame.lon_in_run(170.0), 170.0);
        assert_eq!(frame.lon_for_file(182.6), 182.6 - 360.0);
        assert_eq!(
            frame.projection_lines(),
            [
                "Projection: hex (0,0) is centred at 28.5 N, 178.5 E.".to_string(),
                "  east_nm  = (lon - 178.5) * 60 * 0.878817   (0.878817 is cos 28.5 degrees)"
                    .to_string(),
                "  north_nm = (lat - 28.5) * 60".to_string(),
            ]
        );
        assert_eq!(
            frame.extent_line(),
            "Extent: every hex whose centre lies from 38 N to 18 N and from 163 E to 166 W."
        );
        assert_eq!(frame.marks.len(), 1);
        assert_eq!(frame.marks[0].row, "5100");
    }

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

    /// Decision 0242, rule 12 and part 4 of the hand calculation in
    /// `tests/about_a_centre.rs`.
    #[test]
    fn a_frame_about_a_centre_writes_its_own_lines() {
        let frame = Frame::parse(WIDE).unwrap();
        assert!(frame.is_about_a_centre());
        assert!(frame.crosses_date_line());
        assert_eq!((frame.west_run(), frame.east_run()), (142.0, 207.0));
        assert_eq!(
            frame.projection_lines(),
            [
                "Projection: about a centre. Hex (0,0) is centred at 4 N, 174.5 E.".to_string(),
                "  distance_nm = angle from the centre in radians * 3437.746771 * 0.964, on the bearing from the centre".to_string(),
                "  east_nm = distance_nm * sin(bearing), north_nm = distance_nm * cos(bearing); a sphere of 60 nm to the degree.".to_string(),
            ]
        );
        assert_eq!(
            frame.extent_line(),
            "Extent: every hex whose centre lies from 38 N to 30 S and from 142 E to 153 W."
        );
        // "one factor" may be said, and a frame with no kind is of one factor.
        let said = MIDWAY.replace("centre_lat", "kind = \"one factor\"\ncentre_lat");
        assert_eq!(Frame::parse(&said).unwrap(), Frame::parse(MIDWAY).unwrap());
        assert!(!Frame::parse(MIDWAY).unwrap().is_about_a_centre());
    }

    #[test]
    fn the_two_kinds_keys_do_not_mix() {
        let bad = [
            // a scale in a frame of one factor, with or without a factor
            MIDWAY.replace("factor = ", "scale = \"0.9\"\nfactor = "),
            MIDWAY.replace("factor = ", "scale = "),
            // a factor in a frame about a centre, with or without a scale
            WIDE.replace("scale = ", "factor = \"0.9\"\nscale = "),
            WIDE.replace("scale = ", "factor = "),
            // neither, an unknown kind, an unknown key
            WIDE.replace("scale = \"0.964\"\n", ""),
            MIDWAY.replace("factor = \"0.878817\"\n", ""),
            WIDE.replace("about a centre", "about a pole"),
            format!("{WIDE}extra = \"1\"\n"),
            WIDE.replace("\"0.964\"", "\"1.5\""),
        ];
        for text in bad {
            assert!(Frame::parse(&text).is_err(), "{text}");
        }
    }

    #[test]
    fn a_bad_frame_is_refused() {
        let bad = [
            MIDWAY.replace("\"0.878817\"", "0.878817"),
            MIDWAY.replace("\"0.878817\"", "\"8.8e-1\""),
            MIDWAY.replace("north = \"38\"", "north = \"10\""),
            MIDWAY.replace("centre_lat = \"28.5\"", "centre_lat = \"40\""),
            MIDWAY.replace("row = \"5100\"", "row = \"row 5100\""),
            MIDWAY.replace("title", "name"),
            format!(
                "{MIDWAY}[[land_mark]]\nname = \"midway\"\nlat = \"1\"\nlon = \"1\"\nrow = \"1\"\n"
            ),
        ];
        for text in bad {
            assert!(Frame::parse(&text).is_err(), "{text}");
        }
    }
}
