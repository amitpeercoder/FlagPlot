//! `measure`: the geometry of the map and of the side's own units, answered
//! with the functions the rules themselves use (decision 0049).
//!
//! Four questions: the air distance between two places, the sea route between
//! two places, the passage of one of the side's own task forces to a place,
//! and the air reach of one of its air groups. A place is a hex, or one of the
//! side's own units, or a contact it holds (which stands for the hex the side
//! believes). Anything else is [`NotKnown`], the same document in every case,
//! as for `get_entity` and `explain`.
//!
//! Nothing here has arithmetic of its own for a distance, a speed, a burn or a
//! time: the air distance is `Hex::air_distance_nm`, the radius test is
//! `Hex::air_distance_within`, the route is `SideView::sea_path` (the map's
//! own next-step rule) and the passage is `SideView::forecast_passage`, which
//! runs the movement step's own code on a copy. The answer states what it
//! assumes, and gives no chance, no odds and no advice.

use serde::de::{self, Deserializer};
use serde::{Deserialize, Serialize};
use sim_core::fixed::Fixed;
use sim_core::hex::Hex;
use sim_core::ids::EntityRef;
use sim_core::orders::SpeedSetting;
use sim_core::refdata::{AircraftRole, AttackWeapon};
use sim_core::time::{pulse_start, ClockTime, Pulse};
use sim_core::view::SideView;
use sim_core::{PassageEnd, PassageForecast};

use crate::answer::{NotKnown, FORMAT, VERSION};
use crate::common::{Ctx, Dec, Handle, Place};
use crate::words;

/// A place in a question: a hex as `{"q": 2, "r": -4}`, or a unit or contact
/// by its handle, `"task_force:5"`.
#[derive(Clone, PartialEq, Eq, Debug, Deserialize)]
#[serde(untagged)]
pub enum PlaceArg {
    Hex { q: i32, r: i32 },
    Unit(String),
}

impl PlaceArg {
    pub fn hex(q: i32, r: i32) -> PlaceArg {
        PlaceArg::Hex { q, r }
    }

    pub fn unit(handle: &str) -> PlaceArg {
        PlaceArg::Unit(handle.to_string())
    }
}

/// A speed in a question: `"cruise"`, `"full"`, or knots above zero as a
/// whole number or a decimal string, `17` or `"17.5"`.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum SpeedArg {
    Cruise,
    Full,
    Knots(Fixed),
}

impl SpeedArg {
    fn setting(self) -> SpeedSetting {
        match self {
            SpeedArg::Cruise => SpeedSetting::Cruise,
            SpeedArg::Full => SpeedSetting::Full,
            SpeedArg::Knots(knots) => SpeedSetting::Knots(knots),
        }
    }
}

impl<'de> Deserialize<'de> for SpeedArg {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<SpeedArg, D::Error> {
        #[derive(Deserialize)]
        #[serde(untagged)]
        enum Raw {
            Whole(u32),
            Text(String),
        }
        let knots = match Raw::deserialize(deserializer)? {
            Raw::Whole(n) => Fixed::from_int(i64::from(n)),
            Raw::Text(text) => match text.as_str() {
                "cruise" => return Ok(SpeedArg::Cruise),
                "full" => return Ok(SpeedArg::Full),
                other => other.parse::<Fixed>().map_err(de::Error::custom)?,
            },
        };
        if knots <= Fixed::ZERO {
            return Err(de::Error::custom("a speed in knots must be above zero"));
        }
        Ok(SpeedArg::Knots(knots))
    }
}

/// A question to `measure`. In JSON it is an object whose `question` names it.
#[derive(Clone, PartialEq, Eq, Debug, Deserialize)]
#[serde(tag = "question", rename_all = "snake_case", deny_unknown_fields)]
pub enum Question {
    /// The straight line an aircraft flies between two places.
    AirDistance { from: PlaceArg, to: PlaceArg },
    /// The hexes a force would steam through between two places.
    SeaRoute { from: PlaceArg, to: PlaceArg },
    /// What one of the side's task forces would do, ordered to a place.
    Passage {
        task_force: String,
        to: PlaceArg,
        #[serde(default)]
        speed: Option<SpeedArg>,
    },
    /// How far an air group of the side reaches, and whether it reaches each
    /// place asked about.
    AirReach {
        air_group: String,
        /// A list of places; a single place is taken as a list of one.
        #[serde(deserialize_with = "places_or_one")]
        to: Vec<PlaceArg>,
    },
}

/// `to` of an air reach question: a list of places, or one place, which is a
/// list of one. Anything else is an error that names the shape expected.
fn places_or_one<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Vec<PlaceArg>, D::Error> {
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum Raw {
        Many(Vec<PlaceArg>),
        One(PlaceArg),
    }
    match Raw::deserialize(deserializer) {
        Ok(Raw::Many(places)) => Ok(places),
        Ok(Raw::One(place)) => Ok(vec![place]),
        Err(_) => Err(de::Error::custom(
            "\"to\" must be a list of places, such as [{\"q\": 2, \"r\": -4}, \"base:1\"], or one place; a place is {\"q\": whole number, \"r\": whole number} or a handle such as \"base:1\"",
        )),
    }
}

impl Question {
    /// Reads a question from JSON text.
    pub fn from_json(text: &str) -> Result<Question, String> {
        serde_json::from_str(text).map_err(|e| e.to_string())
    }
}

/// A place as the answer shows it: the hex, and the unit or contact it was
/// named by, if it was named by one.
#[derive(Clone, PartialEq, Eq, Debug, Serialize)]
pub struct Resolved {
    pub unit: Option<Handle>,
    pub place: Place,
}

#[derive(Clone, PartialEq, Eq, Debug, Serialize)]
pub struct AirDistance {
    pub format: &'static str,
    pub version: u32,
    pub kind: &'static str,
    pub question: &'static str,
    pub from: Resolved,
    pub to: Resolved,
    /// Whole miles, the straight line (decision 0040).
    pub distance_nm: u32,
}

#[derive(Clone, PartialEq, Eq, Debug, Serialize)]
pub struct SeaRoute {
    pub format: &'static str,
    pub version: u32,
    pub kind: &'static str,
    pub question: &'static str,
    pub from: Resolved,
    pub to: Resolved,
    /// Whether ships can get from the one place to the other.
    pub route_found: bool,
    /// Each hex entered, in order, ending at `to`. Empty when there is no
    /// route and when the two places are one hex.
    pub hexes: Vec<Place>,
    pub steps: Option<u32>,
    /// Steps times 40 miles.
    pub distance_nm: Option<u32>,
}

/// A moment: the game day and pulse it falls in, and the clock.
#[derive(Clone, PartialEq, Eq, Debug, Serialize)]
pub struct Moment {
    pub day: u32,
    pub pulse: &'static str,
    pub elapsed_hours: Dec,
    /// `D2 10:00`: the game day and the clock time, as the events show it.
    pub clock: String,
}

fn moment(day: u32, pulse: Pulse, elapsed_hours: Fixed) -> Moment {
    Moment {
        day,
        pulse: pulse.name(),
        elapsed_hours: Dec(elapsed_hours),
        clock: ClockTime::from_elapsed(elapsed_hours).to_string(),
    }
}

/// The speed the force would make.
#[derive(Clone, PartialEq, Eq, Debug, Serialize)]
pub struct PassageSpeed {
    /// `cruise`, `full` or knots, as the force has it or as asked.
    pub setting: String,
    /// The speed it makes: the slowest ship's at that setting, limited by
    /// damaged propulsion. Zero if it cannot make way.
    pub knots: Dec,
    pub slowest_ship: Option<Handle>,
}

/// One ship's fuel on the passage.
#[derive(Clone, PartialEq, Eq, Debug, Serialize)]
pub struct ShipFuel {
    pub ship: Handle,
    pub class: String,
    pub fuel_t: Dec,
    pub burned_t: Dec,
    pub left_t: Dec,
    /// This ship's fuel is what stops the force short.
    pub runs_dry: bool,
    /// Miles steamed when it does.
    pub runs_dry_after_nm: Option<Dec>,
}

/// What a passage forecast takes as given. Fixed words.
#[derive(Clone, PartialEq, Eq, Debug, Serialize)]
pub struct Assumptions {
    pub departs: &'static str,
    pub speed: &'static str,
    pub meets: &'static str,
    pub losses: &'static str,
    pub fuel: &'static str,
}

const ASSUMPTIONS: Assumptions = Assumptions {
    departs: "it leaves at the start of the coming day",
    speed: "it keeps the speed it makes now, with its ships' propulsion as it is",
    meets: "it meets nobody and fights nothing",
    losses: "no ship is lost or damaged",
    fuel: "it has the fuel aboard now and takes none",
};

#[derive(Clone, PartialEq, Eq, Debug, Serialize)]
pub struct Passage {
    pub format: &'static str,
    pub version: u32,
    pub kind: &'static str,
    pub question: &'static str,
    pub task_force: Handle,
    /// Where the force is now.
    pub from: Place,
    pub to: Resolved,
    /// `accepted`, or `refused` if the engine would refuse the route order.
    pub order: &'static str,
    /// `no_route`, `already_there` or `no_ship_afloat` when refused.
    pub refusal: Option<&'static str>,
    /// The force is between two hex centres.
    pub between_hexes: bool,
    /// It would turn back to the hex it left (decision 0005, 5).
    pub turns_back: bool,
    /// The hex centre the new route starts from.
    pub route_starts_at: Place,
    pub speed: PassageSpeed,
    /// Every hex of the route, in order, ending at the destination.
    pub route: Vec<Place>,
    /// How many hexes the route has.
    pub route_steps: u32,
    /// How many of them the force enters before it arrives or stops: the
    /// first `entered_steps` of `route`, as the steaming events list them.
    pub entered_steps: u32,
    /// Miles from where the force is to the destination, along the route.
    pub route_nm: Dec,
    pub leaves: Moment,
    /// `arrives`, `runs_out_of_fuel`, `cannot_move` or `does_not_sail`.
    pub outcome: &'static str,
    /// The arrival, or the stop for want of fuel.
    pub ends: Option<Moment>,
    pub ends_at: Option<Place>,
    pub steaming_hours: Dec,
    pub steamed_nm: Dec,
    /// Miles still to go when it stops short; zero otherwise.
    pub short_by_nm: Dec,
    pub ships: Vec<ShipFuel>,
    pub assumptions: Assumptions,
}

#[derive(Clone, PartialEq, Eq, Debug, Serialize)]
pub struct ReachLine {
    pub to: Resolved,
    pub distance_nm: u32,
    /// The distance is within the reach in force, by the comparison the
    /// strike decision makes (a distance equal to the reach is within it).
    /// On normal reach the reach is the radius.
    pub within: bool,
    /// The distance is within the type's radius, whatever the reach in force.
    pub within_radius: bool,
    /// The chance an aircraft that came back from a strike or escort sortie
    /// to this distance runs out of fuel (decision 3 of 0059): 0 within the
    /// radius. `None` where the scenario has no air combat figures.
    pub ditch_chance: Option<Dec>,
}

#[derive(Clone, PartialEq, Eq, Debug, Serialize)]
pub struct AirReach {
    pub format: &'static str,
    pub version: u32,
    pub kind: &'static str,
    pub question: &'static str,
    pub air_group: Handle,
    pub at: Place,
    pub aircraft_type: String,
    /// `patrol`, `bomber` or `fighter`.
    pub role: &'static str,
    /// The type's radius, in miles. Zero for a type that cannot strike.
    pub radius_nm: Dec,
    /// `strike`, `escort` (a fighter's) or `none`.
    pub radius_is: &'static str,
    /// The group's reach in force: `normal` (the radius applies) or
    /// `extended` (the longer reach applies; decision 0059).
    pub reach: &'static str,
    /// The reach in force in miles: the radius on normal reach, and the
    /// radius and a share more, cut to a whole mile, on extended. A distance
    /// is `within` if it is no more than this.
    pub reach_nm: Dec,
    /// For a fighter, the same figure: the radius within which its escorts go
    /// with a raid (decision 0030).
    pub escort_radius_nm: Option<Dec>,
    pub places: Vec<ReachLine>,
}

/// The answer to `measure`: one of the four, or the one answer given for
/// everything the side does not know.
#[derive(Clone, PartialEq, Eq, Debug, Serialize)]
#[serde(untagged)]
pub enum MeasureAnswer {
    AirDistance(AirDistance),
    SeaRoute(SeaRoute),
    Passage(Box<Passage>),
    AirReach(AirReach),
    NotKnown(NotKnown),
}

fn not_known() -> MeasureAnswer {
    MeasureAnswer::NotKnown(NotKnown::of("measure"))
}

/// Answers one question for one side. Every place is looked up among what the
/// side knows; if any is not, or the unit asked about is not the side's own,
/// the answer is [`NotKnown`], whichever it was.
pub fn measure(view: &SideView<'_>, question: &Question) -> MeasureAnswer {
    let ctx = Ctx::new(*view);
    answer(&ctx, question).unwrap_or_else(not_known)
}

/// Reads a question from JSON text, answers it and writes the answer as JSON.
/// `Err` only if the text is not a question.
pub fn measure_json(view: &SideView<'_>, text: &str) -> Result<String, String> {
    let question = Question::from_json(text)?;
    serde_json::to_string(&measure(view, &question)).map_err(|e| e.to_string())
}

/// A hex the side may name, and the unit it was named by.
fn resolve(ctx: &Ctx<'_>, arg: &PlaceArg) -> Option<(Hex, Resolved)> {
    let (hex, unit) = match arg {
        PlaceArg::Hex { q, r } => (Hex::new(*q, *r), None),
        PlaceArg::Unit(handle) => {
            let (kind, number) = handle.split_once(':')?;
            let id: u32 = number.parse().ok()?;
            match kind {
                "ship" => {
                    let ship = ctx.own_ship(id)?;
                    (ctx.ship_hex(ship)?, Some(ctx.ship(ship.id)))
                }
                "task_force" => {
                    let force = ctx.own_force(id)?;
                    (force.position(), Some(ctx.force(force.id)))
                }
                "air_group" => {
                    let group = ctx.own_group(id)?;
                    (ctx.group_hex(group)?, Some(ctx.group(group.id)))
                }
                "base" => {
                    let id = sim_core::ids::BaseId::new(id);
                    match ctx.bases.get(&id) {
                        Some(base) => (base.hex, Some(ctx.base(base.id))),
                        // A base of the other side: where it is is the map's.
                        None => {
                            let base = ctx.enemy_bases.get(&id)?;
                            (base.hex, Some(ctx.base(base.id)))
                        }
                    }
                }
                "contact" => {
                    let contact = ctx.view.contact_by_number(id)?;
                    (
                        contact.estimated_hex,
                        Some(ctx.entity(EntityRef::Contact(contact.id))),
                    )
                }
                _ => return None,
            }
        }
    };
    // A hex off the map is nothing the side knows of.
    ctx.view.hex(hex)?;
    Some((
        hex,
        Resolved {
            unit,
            place: ctx.place(hex),
        },
    ))
}

fn answer(ctx: &Ctx<'_>, question: &Question) -> Option<MeasureAnswer> {
    match question {
        Question::AirDistance { from, to } => {
            let (a, from) = resolve(ctx, from)?;
            let (b, to) = resolve(ctx, to)?;
            Some(MeasureAnswer::AirDistance(AirDistance {
                format: FORMAT,
                version: VERSION,
                kind: "measure",
                question: "air_distance",
                from,
                to,
                distance_nm: a.air_distance_nm(b),
            }))
        }
        Question::SeaRoute { from, to } => {
            let (a, from) = resolve(ctx, from)?;
            let (b, to) = resolve(ctx, to)?;
            let path = ctx.view.sea_path(a, b);
            let steps = path
                .as_ref()
                .map(|p| u32::try_from(p.len()).expect("a path has few hexes"));
            Some(MeasureAnswer::SeaRoute(SeaRoute {
                format: FORMAT,
                version: VERSION,
                kind: "measure",
                question: "sea_route",
                from,
                to,
                route_found: path.is_some(),
                hexes: path.iter().flatten().map(|hex| ctx.place(*hex)).collect(),
                steps,
                // What the force is charged for the way (decision 0058).
                distance_nm: ctx.view.sea_charged_nm(a, b),
            }))
        }
        Question::Passage {
            task_force,
            to,
            speed,
        } => passage(ctx, task_force, to, *speed).map(|p| MeasureAnswer::Passage(Box::new(p))),
        Question::AirReach { air_group, to } => air_reach(ctx, air_group, to),
    }
}

/// An own task force named `task_force:N`.
fn own_force<'a>(ctx: &Ctx<'a>, handle: &str) -> Option<&'a sim_core::state::TaskForce> {
    let (kind, number) = handle.split_once(':')?;
    if kind != "task_force" {
        return None;
    }
    let id: u32 = number.parse().ok()?;
    ctx.own_force(id)
}

fn passage(ctx: &Ctx<'_>, handle: &str, to: &PlaceArg, speed: Option<SpeedArg>) -> Option<Passage> {
    let force = own_force(ctx, handle)?;
    let (destination, to) = resolve(ctx, to)?;
    let forecast: PassageForecast =
        ctx.view
            .forecast_passage(force.id, destination, speed.map(SpeedArg::setting))?;
    let first_day = ctx.view.day() + 1;
    let departs = moment(
        first_day,
        Pulse::Night,
        pulse_start(first_day, Pulse::Night),
    );

    let (outcome, ends, ends_at) = match forecast.end {
        PassageEnd::Refused => ("does_not_sail", None, None),
        PassageEnd::CannotMove => ("cannot_move", None, None),
        PassageEnd::Arrives {
            day,
            pulse,
            elapsed_hours,
        } => (
            "arrives",
            Some(moment(day, pulse, elapsed_hours)),
            Some(ctx.place(destination)),
        ),
        PassageEnd::RunsDry {
            day,
            pulse,
            elapsed_hours,
            at,
        } => (
            "runs_out_of_fuel",
            Some(moment(day, pulse, elapsed_hours)),
            Some(ctx.place(at)),
        ),
    };
    let refusal = forecast.refusal.as_ref().map(|reason| {
        use sim_core::orders::RejectReason as R;
        match reason {
            R::EmptyRoute => "already_there",
            R::NoShipAfloat => "no_ship_afloat",
            _ => "no_route",
        }
    });
    let ran_dry = matches!(forecast.end, PassageEnd::RunsDry { .. });
    let setting = words::speed(speed.map_or(force.speed, SpeedArg::setting));
    Some(Passage {
        format: FORMAT,
        version: VERSION,
        kind: "measure",
        question: "passage",
        task_force: ctx.force(force.id),
        from: ctx.place(forecast.position),
        to,
        order: if refusal.is_some() {
            "refused"
        } else {
            "accepted"
        },
        refusal,
        between_hexes: force.toward_hex.is_some(),
        turns_back: forecast.turns_back,
        route_starts_at: ctx.place(forecast.starts_at),
        speed: PassageSpeed {
            setting,
            knots: Dec(forecast.speed_kn),
            slowest_ship: forecast.slowest.map(|s| ctx.ship(s)),
        },
        route: forecast.route_hexes.iter().map(|h| ctx.place(*h)).collect(),
        route_steps: u32::try_from(forecast.route_hexes.len()).expect("a route has few hexes"),
        entered_steps: u32::try_from(forecast.hexes.len()).expect("a route has few hexes"),
        route_nm: Dec(forecast.route_nm),
        leaves: departs,
        outcome,
        ends,
        ends_at,
        steaming_hours: Dec(forecast.hours),
        steamed_nm: Dec(forecast.steamed_nm),
        short_by_nm: Dec(forecast.route_nm - forecast.steamed_nm),
        ships: forecast
            .ships
            .iter()
            .map(|line| ShipFuel {
                ship: ctx.ship(line.ship),
                class: ctx.ships.get(&line.ship).map_or_else(String::new, |s| {
                    ctx.view
                        .ship_class(s.class)
                        .map_or_else(|| s.class.to_string(), |c| c.name.clone())
                }),
                fuel_t: Dec(line.fuel_t),
                burned_t: Dec(line.burned_t),
                left_t: Dec(line.left_t),
                runs_dry: line.runs_dry,
                runs_dry_after_nm: (ran_dry && line.runs_dry).then_some(Dec(forecast.steamed_nm)),
            })
            .collect(),
        assumptions: ASSUMPTIONS,
    })
}

fn air_reach(ctx: &Ctx<'_>, handle: &str, to: &[PlaceArg]) -> Option<MeasureAnswer> {
    let (kind, number) = handle.split_once(':')?;
    if kind != "air_group" {
        return None;
    }
    let id: u32 = number.parse().ok()?;
    let group = ctx.own_group(id)?;
    let origin = ctx.group_hex(group)?;
    let aircraft = ctx.view.aircraft_type(group.aircraft_type)?;
    let places: Vec<(Hex, Resolved)> = to
        .iter()
        .map(|arg| resolve(ctx, arg))
        .collect::<Option<_>>()?;
    let radius = aircraft.strike_range_nm;
    // The reach in force, by the one function the strike decision uses.
    let reach_nm = ctx.view.reach_nm(group);
    let radius_is = if aircraft.role == AircraftRole::Fighter {
        "escort"
    } else if aircraft.attack_weapon != AttackWeapon::None {
        "strike"
    } else {
        "none"
    };
    Some(MeasureAnswer::AirReach(AirReach {
        format: FORMAT,
        version: VERSION,
        kind: "measure",
        question: "air_reach",
        air_group: ctx.group(group.id),
        at: ctx.place(origin),
        aircraft_type: aircraft.name.clone(),
        role: words::role(aircraft.role),
        radius_nm: Dec(radius),
        radius_is,
        reach: group.reach.name(),
        reach_nm: Dec(reach_nm),
        escort_radius_nm: (aircraft.role == AircraftRole::Fighter).then_some(Dec(radius)),
        places: places
            .into_iter()
            .map(|(hex, to)| ReachLine {
                to,
                distance_nm: origin.air_distance_nm(hex),
                // A type with no radius reaches nothing, not even its own hex.
                within: radius_is != "none" && origin.air_distance_within(hex, reach_nm),
                within_radius: radius_is != "none" && origin.air_distance_within(hex, radius),
                ditch_chance: if radius_is == "none" {
                    None
                } else {
                    ctx.view
                        .ditch_chance(
                            Fixed::from_int(i64::from(origin.air_distance_nm(hex))),
                            radius,
                        )
                        .map(Dec)
                },
            })
            .collect(),
    }))
}

// ---------------------------------------------------------------------
// Text.
// ---------------------------------------------------------------------

fn unit_at(resolved: &Resolved) -> String {
    match &resolved.unit {
        Some(unit) => format!("{} at {}", unit.text(), resolved.place.text()),
        None => resolved.place.text(),
    }
}

fn hex_list(places: &[Place]) -> String {
    let list: Vec<String> = places.iter().map(Place::coords).collect();
    list.join(", ")
}

fn plain(word: &str) -> String {
    word.replace('_', " ")
}

fn when(moment: &Moment) -> String {
    format!(
        "day {}, {} pulse, {}",
        moment.day, moment.pulse, moment.clock
    )
}

impl MeasureAnswer {
    /// The answer as a few plain lines, made from its data and nothing else.
    /// Fixed words contain no digits, so every number comes from the data.
    pub fn to_text(&self) -> String {
        match self {
            MeasureAnswer::AirDistance(a) => a.to_text(),
            MeasureAnswer::SeaRoute(a) => a.to_text(),
            MeasureAnswer::Passage(a) => a.to_text(),
            MeasureAnswer::AirReach(a) => a.to_text(),
            MeasureAnswer::NotKnown(_) => "FLAG PLOT MEASURE\n- Not known.\n".to_string(),
        }
    }
}

impl AirDistance {
    pub fn to_text(&self) -> String {
        format!(
            "FLAG PLOT MEASURE: AIR DISTANCE\n- From: {}\n- To: {}\n- Air distance: {} miles, the straight line\n",
            unit_at(&self.from),
            unit_at(&self.to),
            self.distance_nm
        )
    }
}

impl SeaRoute {
    pub fn to_text(&self) -> String {
        let mut text = format!(
            "FLAG PLOT MEASURE: SEA ROUTE\n- From: {}\n- To: {}\n",
            unit_at(&self.from),
            unit_at(&self.to)
        );
        match (self.steps, self.distance_nm) {
            (Some(0), _) => text.push_str("- Sea route: none needed, the same hex\n"),
            (Some(steps), Some(miles)) => {
                text.push_str(&format!(
                    "- Sea route: {steps} hexes, {miles} miles\n- Hexes entered: {}\n",
                    hex_list(&self.hexes)
                ));
            }
            _ => text.push_str("- Sea route: none, ships cannot get from the one to the other\n"),
        }
        text
    }
}

impl Passage {
    pub fn to_text(&self) -> String {
        let mut text = String::from("FLAG PLOT MEASURE: PASSAGE\n");
        let mut line = |s: String| {
            text.push_str("- ");
            text.push_str(&s);
            text.push('\n');
        };
        line(format!(
            "Task force: {}, now at {}",
            self.task_force.text(),
            self.from.text()
        ));
        line(format!("To: {}", unit_at(&self.to)));
        if let Some(reason) = self.refusal {
            line(format!(
                "Order: refused, {}. The force would not sail",
                plain(reason)
            ));
        } else {
            line("Order: accepted".to_string());
            let mut start = format!("Route starts from {}", self.route_starts_at.text());
            if self.turns_back {
                start.push_str(", the hex it left: it turns back");
            } else if self.between_hexes {
                start.push_str(", the hex ahead");
            }
            line(start);
            line(format!(
                "Route: {} hexes, {} miles: {}",
                self.route_steps,
                self.route_nm,
                hex_list(&self.route)
            ));
            let slowest = self
                .speed
                .slowest_ship
                .as_ref()
                .map_or_else(String::new, |s| format!(", set by {}", s.text()));
            line(format!(
                "Speed: {}, {} knots{}",
                self.speed.setting, self.speed.knots, slowest
            ));
            line(format!("Leaves: {}", when(&self.leaves)));
            match (&self.ends, &self.ends_at) {
                (Some(ends), Some(at)) if self.outcome == "arrives" => line(format!(
                    "Arrives: {} at {}, after {} hours of steaming",
                    when(ends),
                    at.text(),
                    self.steaming_hours
                )),
                (Some(ends), Some(at)) => line(format!(
                    "Runs out of fuel: {} at {}, after {} hours and {} miles, {} miles short",
                    when(ends),
                    at.text(),
                    self.steaming_hours,
                    self.steamed_nm,
                    self.short_by_nm
                )),
                _ => line("Cannot move: no ship can make way".to_string()),
            }
        }
        for ship in &self.ships {
            let mut s = format!(
                "Fuel: {} has {} t, burns {} t, would have {} t",
                ship.ship.text(),
                ship.fuel_t,
                ship.burned_t,
                ship.left_t
            );
            if let Some(miles) = ship.runs_dry_after_nm {
                s.push_str(&format!(", runs dry after {miles} miles"));
            }
            line(s);
        }
        let a = &self.assumptions;
        for assumption in [a.departs, a.speed, a.meets, a.losses, a.fuel] {
            line(format!("Assumes: {assumption}"));
        }
        text
    }
}

impl AirReach {
    pub fn to_text(&self) -> String {
        let mut text = format!(
            "FLAG PLOT MEASURE: AIR REACH\n- Air group: {} at {}\n- Type: {}, {}\n",
            self.air_group.text(),
            self.at.text(),
            self.aircraft_type,
            self.role
        );
        match self.radius_is {
            "none" => text.push_str("- Radius: none, the type does not strike\n"),
            "escort" => text.push_str(&format!(
                "- Radius: {} miles, also its escort radius\n",
                self.radius_nm
            )),
            _ => text.push_str(&format!("- Radius: {} miles\n", self.radius_nm)),
        }
        if self.radius_is != "none" {
            text.push_str(&format!(
                "- Reach in force: {}, {} miles\n",
                self.reach, self.reach_nm
            ));
        }
        if self.places.is_empty() {
            text.push_str("- Places asked: none\n");
        }
        for line in &self.places {
            text.push_str(&format!(
                "- {}: {} miles, {}\n",
                unit_at(&line.to),
                line.distance_nm,
                match (line.within, line.within_radius) {
                    (_, true) => "within the radius",
                    (true, false) => "beyond the radius, within the reach in force",
                    (false, false) => "beyond the reach in force",
                }
            ));
            if let Some(chance) = line.ditch_chance {
                if !line.within_radius && line.within {
                    text.push_str(&format!(
                        "  An aircraft that came back would run out of fuel with chance {chance}\n"
                    ));
                }
            }
        }
        text
    }
}
