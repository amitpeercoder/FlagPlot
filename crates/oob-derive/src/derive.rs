//! The figures of one ship, from the facts, as of a date (decision 0020).

use std::collections::{BTreeMap, BTreeSet};

use oob_check::table::Data;

use crate::date::Day;
use crate::decimal::Dec;
use crate::input::Rules;
use crate::problem::{Kind, Problem};
use crate::reading::{resolve, resolve_over, Failure, Index, Read, Row, Table};

/// The figures of a class row, in the column order of the class CSV after
/// `id` and `name`.
pub const FIGURES: [&str; 14] = [
    "kind",
    "cruise_speed_kn",
    "max_speed_kn",
    "fuel_capacity_t",
    "endurance_nm",
    "cargo_capacity_t",
    "main_guns",
    "gun_rounds_per_min",
    "gun_rounds_per_gun",
    "gun_damage",
    "torpedo_tubes",
    "torpedo_damage",
    "toughness",
    "radar",
];

pub const KIND: usize = 0;
pub const CARGO: usize = 5;
pub const RADAR: usize = 13;

/// Figures that are whole numbers in the class CSV: the counts and the
/// rounds carried. The rest are written as exact decimals.
pub const WHOLE: [usize; 3] = [6, 8, 10];

/// The class types the simulation has a kind for.
const KIND_FOR_TYPE: [(&str, &str); 6] = [
    ("heavy_cruiser", "cruiser"),
    ("light_cruiser", "cruiser"),
    ("destroyer", "destroyer"),
    ("battleship", "battleship"),
    ("carrier", "carrier"),
    ("light_carrier", "carrier"),
];

/// Sensor items that stand for "no radar of any kind" (`data/oob/README.md`,
/// "Known limits of the format"). A count of 0 of anything else only says
/// that one set was absent.
pub const NO_RADAR_ITEMS: [&str; 1] = ["jp-radar-any"];

/// The rate of fire the tool reads: what a source says a gun achieved or
/// could keep up in service. `rate_of_fire_rpm`, a design or maximum rate,
/// is never read.
const RATE_OF_FIRE: &str = "rate_of_fire_practical_rpm";

const RADAR_SLOTS: [&str; 2] = ["air_search_radar", "surface_search_radar"];

/// A fact row cited by a figure: file and id.
pub type Cite = (String, u64);

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum Value {
    Text(String),
    Num(Dec),
    Flag(bool),
}

#[derive(Clone, Debug)]
pub struct Figure {
    pub value: Value,
    pub cites: BTreeSet<Cite>,
    /// Read through an exception.
    pub excepted: bool,
    /// Ship rows that hold on the date but are not readable, so the class's
    /// rows were read instead: `file:id (standing)`.
    pub doubts: BTreeSet<String>,
}

#[derive(Clone, Debug)]
pub struct ShipResult {
    pub ship: String,
    pub class_id: u64,
    pub class_name: String,
    pub figures: Vec<Figure>,
    /// Radar was unknown and the list says to treat it as not fitted.
    pub radar_assumed: bool,
    /// Exceptions used: index into the rules' list, and the row read.
    pub used: Vec<(usize, Row)>,
}

struct ShipInfo {
    row_id: String,
    class: String,
}

struct ClassInfo {
    row_id: String,
    name: String,
    kind_type: String,
}

pub struct Env<'a> {
    index: Index,
    ships: BTreeMap<String, ShipInfo>,
    classes: BTreeMap<String, ClassInfo>,
    date: Day,
    rules: &'a Rules,
}

/// Where a figure's value came from.
struct Got<T> {
    value: T,
    cites: BTreeSet<Cite>,
    excepted: bool,
    doubts: BTreeSet<String>,
}

struct Work<'a> {
    subject: &'a str,
    problems: Vec<Problem>,
    used: Vec<(usize, Row)>,
}

impl Work<'_> {
    fn fail(&mut self, kind: Kind, figure: &str, message: String) {
        self.problems
            .push(Problem::new(self.subject, kind, figure, message));
    }

    fn failed(&mut self, figure: &str, failure: Failure) {
        self.fail(failure.kind, figure, failure.message);
    }

    /// Notes the exceptions a read used and returns the rows cited, whether
    /// an exception was used, and the ship rows passed over.
    fn take(&mut self, read: &Read) -> (BTreeSet<Cite>, bool, BTreeSet<String>) {
        self.used.extend(read.excepted.iter().cloned());
        (
            read.rows.iter().map(Row::cite_key).collect(),
            !read.excepted.is_empty(),
            read.doubts.iter().cloned().collect(),
        )
    }
}

fn number(read: &Read, fact: &str) -> Result<Dec, Failure> {
    let row = &read.rows[0];
    let bad = |why: &str| Failure {
        kind: Kind::BadValue,
        message: format!(
            "{fact} of {}: row {} has value `{}`: {why}",
            row.entity,
            row.cite(),
            row.value
        ),
    };
    let dec = Dec::parse(&row.value).map_err(|e| bad(&e.to_string()))?;
    if dec.is_negative() {
        return Err(bad("negative"));
    }
    Ok(dec)
}

fn count(read: &Read, slot: &str) -> Result<u32, Failure> {
    let row = &read.rows[0];
    row.count.parse::<u32>().map_err(|_| Failure {
        kind: Kind::BadValue,
        message: format!(
            "{slot} of {}: row {} has count `{}`, which is not a whole number",
            row.entity,
            row.cite(),
            row.count
        ),
    })
}

fn put(figs: &mut [Option<Figure>], i: usize, value: Value, got: Got<()>) {
    figs[i] = Some(Figure {
        value,
        cites: got.cites,
        excepted: got.excepted,
        doubts: got.doubts,
    });
}

impl<T> Got<T> {
    /// The sources of this value, for a figure derived from it.
    fn bare(&self) -> Got<()> {
        Got {
            value: (),
            cites: self.cites.clone(),
            excepted: self.excepted,
            doubts: self.doubts.clone(),
        }
    }
}

fn overflow(what: &str) -> Failure {
    Failure {
        kind: Kind::BadValue,
        message: format!("{what} does not fit"),
    }
}

/// What a fit says: how many, of what.
struct Fit {
    count: u32,
    item: String,
}

impl<'a> Env<'a> {
    pub fn build(data: &Data, date: Day, rules: &'a Rules) -> Env<'a> {
        let mut ships = BTreeMap::new();
        for rec in data.rows("ships.csv") {
            ships.insert(
                rec.get("key").to_string(),
                ShipInfo {
                    row_id: rec.get("id").to_string(),
                    class: rec.get("class").to_string(),
                },
            );
        }
        let mut classes = BTreeMap::new();
        for rec in data.rows("ship_classes.csv") {
            classes.insert(
                rec.get("key").to_string(),
                ClassInfo {
                    row_id: rec.get("id").to_string(),
                    name: rec.get("name").to_string(),
                    kind_type: rec.get("type").to_string(),
                },
            );
        }
        Env {
            index: Index::build(data),
            ships,
            classes,
            date,
            rules,
        }
    }

    /// Problems for exceptions that name no row of the facts.
    pub fn exception_problems(&self) -> Vec<Problem> {
        self.rules
            .exceptions
            .iter()
            .filter(|e| !self.index.has_row(e))
            .map(|e| {
                Problem::new(
                    "rules",
                    Kind::Exception,
                    e.fact.clone(),
                    format!(
                        "exception for `{}` names row {}, which is not a row about `{}` in any facts file or fits.csv",
                        e.fact, e.row, e.entity
                    ),
                )
            })
            .collect()
    }

    fn fact(
        &self,
        w: &mut Work,
        keys: (&str, &str),
        attribute: &str,
        figure: &str,
    ) -> Option<Got<Dec>> {
        let (ship, class) = keys;
        let read = resolve_over(
            self.index.rows(Table::ShipFacts, ship, attribute),
            &format!("{attribute} of ship {ship}"),
            self.index.rows(Table::ClassFacts, class, attribute),
            &format!("{attribute} of class {class}"),
            self.date,
            &self.rules.exceptions,
        );
        let read = match read {
            Ok(read) => read,
            Err(failure) => {
                w.failed(figure, failure);
                return None;
            }
        };
        match number(&read, attribute) {
            Ok(value) => {
                let (cites, excepted, doubts) = w.take(&read);
                Some(Got {
                    value,
                    cites,
                    excepted,
                    doubts,
                })
            }
            Err(failure) => {
                w.failed(figure, failure);
                None
            }
        }
    }

    fn fit_read(&self, keys: (&str, &str), slot: &str) -> Result<Read, Failure> {
        let (ship, class) = keys;
        resolve_over(
            self.index.rows(Table::ShipFits, ship, slot),
            &format!("{slot} fit of ship {ship}"),
            self.index.rows(Table::ClassFits, class, slot),
            &format!("{slot} fit of class {class}"),
            self.date,
            &self.rules.exceptions,
        )
    }

    fn fit(&self, w: &mut Work, keys: (&str, &str), slot: &str, figure: &str) -> Option<Got<Fit>> {
        let read = match self.fit_read(keys, slot) {
            Ok(read) => read,
            Err(failure) => {
                w.failed(figure, failure);
                return None;
            }
        };
        match count(&read, slot) {
            Ok(n) => {
                let (cites, excepted, doubts) = w.take(&read);
                Some(Got {
                    value: Fit {
                        count: n,
                        item: read.rows[0].item.clone(),
                    },
                    cites,
                    excepted,
                    doubts,
                })
            }
            Err(failure) => {
                w.failed(figure, failure);
                None
            }
        }
    }

    /// A fact about the weapon of a fit. The fit's rows are cited too, since
    /// they say which weapon.
    fn weapon_fact(
        &self,
        w: &mut Work,
        fit: &Got<Fit>,
        attribute: &str,
        figure: &str,
    ) -> Option<Got<Dec>> {
        let weapon = &fit.value.item;
        let read = resolve(
            self.index.rows(Table::WeaponFacts, weapon, attribute),
            &format!("{attribute} of weapon {weapon}"),
            self.date,
            &self.rules.exceptions,
        );
        let read = match read {
            Ok(read) => read,
            Err(failure) => {
                w.failed(figure, failure);
                return None;
            }
        };
        match number(&read, attribute) {
            Ok(value) => {
                let (mut cites, excepted, mut doubts) = w.take(&read);
                cites.extend(fit.cites.iter().cloned());
                doubts.extend(fit.doubts.iter().cloned());
                Some(Got {
                    value,
                    cites,
                    excepted: excepted || fit.excepted,
                    doubts,
                })
            }
            Err(failure) => {
                w.failed(figure, failure);
                None
            }
        }
    }

    /// The practical rate of fire of the weapon of a fit. A weapon with no
    /// `rate_of_fire_practical_rpm` row at all is `missing`, and the message
    /// says whether a `rate_of_fire_rpm` row (a theoretical or maximum rate,
    /// which is not read) exists. There is no fallback.
    fn rate_of_fire(&self, w: &mut Work, fit: &Got<Fit>) -> Option<Got<Dec>> {
        let weapon = &fit.value.item;
        if self
            .index
            .rows(Table::WeaponFacts, weapon, RATE_OF_FIRE)
            .is_empty()
        {
            let other = self
                .index
                .rows(Table::WeaponFacts, weapon, "rate_of_fire_rpm");
            let said = if other.is_empty() {
                "no rate_of_fire_rpm row exists either".to_string()
            } else {
                format!(
                    "a rate_of_fire_rpm row exists ({}), a theoretical or maximum rate, which the tool does not read",
                    other
                        .iter()
                        .map(|r| format!("{} ({}; {})", r.cite(), r.standing, r.value))
                        .collect::<Vec<_>>()
                        .join(", ")
                )
            };
            w.fail(
                Kind::Missing,
                FIGURES[7],
                format!("{RATE_OF_FIRE} of weapon {weapon}: no rows in the facts; {said}"),
            );
            return None;
        }
        self.weapon_fact(w, fit, RATE_OF_FIRE, FIGURES[7])
    }

    /// Derives every figure of a ship. All problems are returned, not the
    /// first. `assume_no_radar` is the list's working assumption.
    pub fn derive_ship(
        &self,
        ship: &str,
        assume_no_radar: bool,
    ) -> Result<ShipResult, Vec<Problem>> {
        let mut w = Work {
            subject: ship,
            problems: Vec::new(),
            used: Vec::new(),
        };
        let Some(ship_info) = self.ships.get(ship) else {
            w.fail(
                Kind::UnknownShip,
                "ship",
                "no such ship key in ships.csv".to_string(),
            );
            return Err(w.problems);
        };
        let class_key = ship_info.class.as_str();
        let Some(class) = self.classes.get(class_key) else {
            let message = if class_key.is_empty() {
                format!("ships.csv:{} has no class", ship_info.row_id)
            } else {
                format!(
                    "ships.csv:{} has class `{class_key}`, which is not in ship_classes.csv",
                    ship_info.row_id
                )
            };
            w.fail(Kind::NoClass, "class", message);
            return Err(w.problems);
        };
        let keys = (ship, class_key);
        let mut figs: Vec<Option<Figure>> = vec![None; FIGURES.len()];
        let num = |got: Got<Dec>| (Value::Num(got.value), got.bare());

        // kind
        match KIND_FOR_TYPE.iter().find(|(t, _)| *t == class.kind_type) {
            Some((_, kind)) => {
                let cites = BTreeSet::from([
                    ("ships.csv".to_string(), ship_info.row_id.parse().unwrap_or(0)),
                    (
                        "ship_classes.csv".to_string(),
                        class.row_id.parse().unwrap_or(0),
                    ),
                ]);
                let got = Got {
                    value: (),
                    cites,
                    excepted: false,
                    doubts: BTreeSet::new(),
                };
                put(&mut figs, KIND, Value::Text((*kind).to_string()), got);
            }
            None => w.fail(
                Kind::UnmappedType,
                "kind",
                format!(
                    "class {class_key} (ship_classes.csv:{}) has type `{}`, which has no simulation kind",
                    class.row_id, class.kind_type
                ),
            ),
        }

        // speeds, fuel, endurance
        for (i, attribute) in [
            (1, "endurance_speed_kn"),
            (2, "max_speed_kn"),
            (3, "fuel_capacity_lt"),
            (4, "endurance_nm"),
        ] {
            if let Some(got) = self.fact(&mut w, keys, attribute, FIGURES[i]) {
                let (v, sources) = num(got);
                put(&mut figs, i, v, sources);
            }
        }
        put(
            &mut figs,
            CARGO,
            Value::Num(Dec::ZERO),
            Got {
                value: (),
                cites: BTreeSet::new(),
                excepted: false,
                doubts: BTreeSet::new(),
            },
        );

        // main battery
        let main = self.fit(&mut w, keys, "main_battery", "main_guns");
        if let Some(main) = &main {
            put(
                &mut figs,
                6,
                Value::Num(Dec::from_int(i64::from(main.value.count)).unwrap_or(Dec::ZERO)),
                main.bare(),
            );
            if let Some(got) = self.rate_of_fire(&mut w, main) {
                let (v, sources) = num(got);
                put(&mut figs, 7, v, sources);
            }
        }
        if let Some(got) = self.fact(&mut w, keys, "main_rounds_per_gun", FIGURES[8]) {
            let (v, sources) = num(got);
            put(&mut figs, 8, v, sources);
        }
        if let Some(main) = &main {
            if let Some(got) = self.weapon_fact(&mut w, main, "shell_weight_kg", FIGURES[9]) {
                match got.value.div(self.rules.gun_shell_kg_per_damage) {
                    Some(damage) => put(&mut figs, 9, Value::Num(damage), got.bare()),
                    None => w.failed(FIGURES[9], overflow("gun damage")),
                }
            }
        }

        // torpedoes
        if let Some(tubes) = self.fit(&mut w, keys, "torpedo_tubes", "torpedo_tubes") {
            put(
                &mut figs,
                10,
                Value::Num(Dec::from_int(i64::from(tubes.value.count)).unwrap_or(Dec::ZERO)),
                tubes.bare(),
            );
            if tubes.value.count == 0 {
                put(&mut figs, 11, Value::Num(Dec::ZERO), tubes.bare());
            } else if let Some(got) = self.weapon_fact(&mut w, &tubes, "warhead_kg", FIGURES[11]) {
                match got.value.mul(self.rules.torpedo_damage_per_warhead_kg) {
                    Some(damage) => put(&mut figs, 11, Value::Num(damage), got.bare()),
                    None => w.failed(FIGURES[11], overflow("torpedo damage")),
                }
            }
        }

        // toughness
        if let Some(got) = self.fact(&mut w, keys, "displacement_standard_lt", FIGURES[12]) {
            match got.value.div(self.rules.displacement_lt_per_toughness) {
                Some(toughness) if !toughness.is_positive() => w.fail(
                    Kind::ZeroToughness,
                    FIGURES[12],
                    format!(
                        "displacement_standard_lt {} / {} = {toughness}, which is not above zero",
                        got.value, self.rules.displacement_lt_per_toughness
                    ),
                ),
                Some(toughness) => put(&mut figs, 12, Value::Num(toughness), got.bare()),
                None => w.failed(FIGURES[12], overflow("toughness")),
            }
        }

        // radar
        let mut radar_assumed = false;
        if let Some((fitted, sources)) = self.radar(&mut w, keys, assume_no_radar) {
            radar_assumed = fitted.1;
            put(&mut figs, RADAR, Value::Flag(fitted.0), sources);
        }

        if !w.problems.is_empty() {
            return Err(w.problems);
        }
        let figures = figs
            .into_iter()
            .map(|f| f.expect("every figure is set when there is no problem"))
            .collect();
        Ok(ShipResult {
            ship: ship.to_string(),
            class_id: class.row_id.parse().unwrap_or(0),
            class_name: class.name.clone(),
            figures,
            radar_assumed,
            used: w.used,
        })
    }

    /// Radar: the figure and whether it is the list's assumption.
    fn radar(
        &self,
        w: &mut Work,
        keys: (&str, &str),
        assume_no_radar: bool,
    ) -> Option<((bool, bool), Got<()>)> {
        let mut found: Vec<(&str, Read, u32)> = Vec::new();
        let mut absent: Vec<(&str, Failure)> = Vec::new();
        let mut hard = false;
        for slot in RADAR_SLOTS {
            match self.fit_read(keys, slot) {
                Ok(read) => match count(&read, slot) {
                    Ok(n) => found.push((slot, read, n)),
                    Err(failure) => {
                        w.failed(FIGURES[RADAR], failure);
                        hard = true;
                    }
                },
                Err(failure)
                    if matches!(
                        failure.kind,
                        Kind::Missing | Kind::NotPreferred | Kind::NotOnDate
                    ) =>
                {
                    absent.push((slot, failure));
                }
                Err(failure) => {
                    w.failed(FIGURES[RADAR], failure);
                    hard = true;
                }
            }
        }
        if hard {
            return None;
        }
        let fitted = found.iter().any(|(_, _, n)| *n > 0);
        let stand_in = found
            .iter()
            .any(|(_, read, n)| *n == 0 && NO_RADAR_ITEMS.contains(&read.rows[0].item.as_str()));
        let both_empty = found.len() == RADAR_SLOTS.len() && found.iter().all(|(_, _, n)| *n == 0);
        let mut sources = Got {
            value: (),
            cites: BTreeSet::new(),
            excepted: false,
            doubts: BTreeSet::new(),
        };
        for (_, read, _) in &found {
            let (c, e, d) = w.take(read);
            sources.cites.extend(c);
            sources.excepted |= e;
            sources.doubts.extend(d);
        }
        if fitted {
            return Some(((true, false), sources));
        }
        if stand_in || both_empty {
            return Some(((false, false), sources));
        }
        if assume_no_radar {
            return Some(((false, true), sources));
        }
        let mut found_text: Vec<String> = found
            .iter()
            .map(|(slot, read, n)| {
                format!(
                    "{slot}: {} says {} x {n} (a specific set absent, not all radar)",
                    read.rows
                        .iter()
                        .map(Row::cite)
                        .collect::<Vec<_>>()
                        .join(" "),
                    read.rows[0].item
                )
            })
            .collect();
        found_text.extend(absent.iter().map(|(_, f)| f.message.clone()));
        w.fail(
            Kind::RadarUnknown,
            FIGURES[RADAR],
            format!(
                "nothing in the facts says a search radar was or was not fitted on {}: {}",
                self.date,
                found_text.join("; ")
            ),
        );
        None
    }
}
