//! The figures of one ship, from the facts, as of a date (decision 0020).

use std::collections::{BTreeMap, BTreeSet};

use oob_check::table::Data;

use crate::date::{begins_after, valid_on, Day, Tri};
use crate::decimal::Dec;
use crate::input::{Estimate, Exception, Method, Rules, Subject};
use crate::problem::{Kind, Problem};
use crate::reading::{
    resolve, resolve_over, resolve_slot, Failure, Index, Read, Row, SlotRead, Table,
};

/// The figures of a class row, in the column order of the class CSV after
/// `id` and `name`.
pub const FIGURES: [&str; 15] = [
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
    "torpedo_tubes_one_side",
    "torpedo_damage",
    "toughness",
    "radar",
];

/// The two figures added when the list asks for `air_figures`, after
/// `radar` (decision 0029).
pub const AIR_FIGURES: [&str; 2] = ["aircraft_capacity", "anti_aircraft"];

/// Indexes of the surface weapon figures a ship the list names in
/// `surface_weapons_not_modelled` has written as 0: main guns, rate of
/// fire, rounds per gun, gun damage, torpedo tubes, torpedo tubes on one
/// side, torpedo damage.
pub const SURFACE_WEAPONS: std::ops::RangeInclusive<usize> = 6..=12;

pub const NOT_MODELLED_NOTE: &str =
    "assumption of the scenario, not a fact: surface weapons not modelled (decision 0029)";

pub const KIND: usize = 0;
pub const CARGO: usize = 5;
pub const RADAR: usize = 14;

/// Figures that are whole numbers in the class CSV: the counts and the
/// rounds carried. The rest are written as exact decimals.
pub const WHOLE: [usize; 6] = [6, 8, 10, 11, 15, 16];

/// The class types the simulation has a kind for. An oiler is the one type
/// that is not a warship (decision 0206, rule 1).
const KIND_FOR_TYPE: [(&str, &str); 7] = [
    ("heavy_cruiser", "cruiser"),
    ("light_cruiser", "cruiser"),
    ("destroyer", "destroyer"),
    ("battleship", "battleship"),
    ("carrier", "carrier"),
    ("light_carrier", "carrier"),
    ("oiler", "auxiliary"),
];

/// The class type whose ships fuel others at sea (decision 0206, rule 1).
pub const OILER_TYPE: &str = "oiler";
/// The simulation kinds that are not warships: a ship of one carries no
/// main battery and no torpedo tubes, by her kind (decision 0166, rule 5;
/// decision 0206, rule 3).
const UNARMED_KINDS: [&str; 1] = ["auxiliary"];
/// The figure written after the others when the list has an oiler: 1 for a
/// class whose ships fuel others at sea (decision 0201, 1; decision 0206,
/// rule 1).
pub const MARK_FIGURE: &str = "fuels_at_sea";
pub const MARK_RULE: &str = "1 for a class of the type oiler in ship_classes.csv, whose ships fuel others at sea; else 0 (decision 0201, 1; decision 0206, rule 1). The column is written only where the list has an oiler";
/// The note on the surface weapon figures of a ship of a kind that is not a
/// warship.
pub const UNARMED_NOTE: &str = "by her kind: an auxiliary has no main battery and no torpedo tubes, and no fit, shell or outfit is read (decision 0166, rule 5; decision 0206, rule 3)";
/// Her own fuel and her cargo of oil in barrels, and the factor that makes
/// them tons (decision 0206, rules 6 to 8).
const FUEL_BBL: &str = "fuel_capacity_bbl";
const CARGO_BBL: &str = "cargo_oil_capacity_bbl";
const BARREL_FACTOR: &str = "fuel_oil_barrel_factor_lt";
/// The displacement an auxiliary's toughness is read from (decision 0166,
/// rule 2; decision 0206, rule 2).
const LIGHT_DISPLACEMENT: &str = "displacement_light_lt";
const OILER_KIND_RULE: &str = "type of the class in ship_classes.csv, mapped to a simulation kind (oiler: auxiliary; decision 0206, rule 1)";
const OILER_FUEL_RULE: &str = "fuel_capacity_bbl of the ship, else of its class, x fuel_oil_barrel_factor_lt of the ship, else of its class: her own bunkers in barrels made long tons by the factor read from a row (decision 0206, rules 6 and 8; a simulation ton is a long ton)";
const OILER_CARGO_RULE: &str = "cargo_oil_capacity_bbl of the ship, else of its class, x fuel_oil_barrel_factor_lt of the ship, else of its class: the oil she can carry for others, a capacity, made long tons by the factor read from a row (decision 0206, rules 7 and 8)";
/// An oiler's cargo by weight, where no row gives it in barrels (decision
/// 0207, as decision 0166, rule 3 has it for a transport).
const CARGO_LT: &str = "cargo_capacity_lt";
const OILER_CARGO_WEIGHT_RULE: &str = "cargo_capacity_lt of the ship, else of its class: the oil she can carry for others by weight, a capacity in long tons, read where no row gives it in barrels (decision 0166, rule 3; decision 0207)";
/// The note on an anti-aircraft figure the list's assumption gives (decision
/// 0207, first use).
pub const AA_ASSUMED_NOTE: &str = "assumption of the scenario, not a fact: anti-aircraft guns unknown in the facts, treated as none";
const UNARMED_RULE: &str = "0 by kind: a ship of a kind that is not a warship (auxiliary) has no main battery and no torpedo tubes (decision 0166, rule 5; decision 0206, rule 3)";
const AUXILIARY_TOUGHNESS_RULE: &str = "displacement_light_lt of the ship, else of its class / displacement_lt_per_toughness, for a kind that is not a warship (decision 0166, rule 2; decision 0206, rule 2)";

/// Sensor items that stand for "no radar of any kind" (`data/oob/README.md`,
/// "Known limits of the format"). A count of 0 of anything else only says
/// that one set was absent.
pub const NO_RADAR_ITEMS: [&str; 1] = ["jp-radar-any"];

/// The rate of fire the tool reads: what a source says a gun achieved or
/// could keep up in service. `rate_of_fire_rpm`, a design or maximum rate,
/// is never read.
const RATE_OF_FIRE: &str = "rate_of_fire_practical_rpm";
/// A gun's designed rate, read only where the rules file names a row of it.
const DESIGNED_RATE: &str = "rate_of_fire_rpm";
const ROUNDS: &str = "main_rounds_per_gun";
/// The attributes that say how a class's torpedo tubes bear (decision 0074).
const PER_BROADSIDE: &str = "torpedo_tubes_per_broadside";
const MOUNT_POSITION: &str = "torpedo_mount_position";
/// The note for a class no row of which says how its tubes bear.
pub const NOT_SPLIT_NOTE: &str = "not split: no row says how its tubes bear";

/// The rounds per gun written for a ship the list names in
/// `main_gun_ammunition_not_limiting` when no `main_rounds_per_gun` row
/// exists (decision 0023). It is a marker for "ammunition does not limit
/// fire", far above any real outfit, not a sourced or invented outfit; the
/// sources file says so beside the figure.
pub const AMMUNITION_NOT_LIMITING_ROUNDS: i64 = 9999;

const RADAR_SLOTS: [&str; 2] = ["air_search_radar", "surface_search_radar"];
/// The slot whose sets give the `radar` figure (decision 0077, 2). The
/// other slot is read only to know that a radar fit is on record.
const SURFACE_SLOT: &str = "surface_search_radar";
/// The attribute of a sensor that says what its display shows, and the
/// text of its value that marks a plan position indicator (decision 0077, 1).
const DISPLAY: &str = "display";
const PLAN_POSITION: &str = "PPI";
const AA_SLOTS: [&str; 2] = ["heavy_aa", "light_aa"];

/// What a surface-search set's display rows say (decision 0077).
enum Display {
    /// A readable row whose value has a plan position indicator.
    Plan(Read),
    /// A readable row whose value has none.
    NoPlan(Read),
    /// No readable row: the set and the fit row naming it.
    NotGiven(String),
}

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
    /// Other things to say beside the figure in the sources file.
    pub notes: BTreeSet<String>,
    /// The rule behind the figure in words, where it is not the figure's
    /// usual one (an oiler's, decision 0206).
    pub rule: Option<&'static str>,
    /// The figure rests on an estimated attribute (decision 0207, rule 9):
    /// the rule printed in place of the rule of a figure read.
    pub estimate: Option<String>,
}

/// What the list says about one ship.
#[derive(Clone, Copy, Debug, Default)]
pub struct ShipOpts {
    /// The working assumption that unknown radar means none.
    pub assume_no_radar: bool,
    pub assume_ammunition_not_limiting: bool,
    /// Derive `aircraft_capacity` and `anti_aircraft` too.
    pub air_figures: bool,
    /// Write the surface weapon figures as 0 without reading any fact.
    pub surface_weapons_not_modelled: bool,
    /// Write the `fuels_at_sea` figure: the list has an oiler (decision
    /// 0206, rule 1).
    pub mark_column: bool,
    /// The list's working assumption that unknown anti-aircraft guns mean
    /// none (decision 0207, first use).
    pub assume_no_anti_aircraft: bool,
}

#[derive(Clone, Debug)]
pub struct ShipResult {
    pub ship: String,
    /// The ship's name in ships.csv.
    pub ship_name: String,
    pub class_id: u64,
    pub class_name: String,
    pub figures: Vec<Figure>,
    /// Radar was unknown and the list says to treat it as not fitted.
    pub radar_assumed: bool,
    /// Surface-search sets of the ship that did not count for the radar
    /// figure because no readable row gives their display (decision 0077,
    /// 3): the set and the fit row naming it.
    pub radar_not_counted: Vec<String>,
    /// No rounds-per-gun row exists and the list says to treat main-gun
    /// ammunition as not limiting.
    pub ammunition_assumed: bool,
    /// The rate of fire is a designed rate (`rate_of_fire_rpm`) read
    /// because the rules file names a row of it.
    pub designed_rate: bool,
    /// The surface weapon figures are the scenario's assumption of 0.
    pub weapons_not_modelled: bool,
    /// The surface weapon figures are 0 by the ship's kind (decision 0206,
    /// rule 3).
    pub unarmed_by_kind: bool,
    /// The ship has tubes and no readable row says how they bear, so the
    /// tubes on one side are all of them (decision 0074, 3).
    pub tubes_not_split: bool,
    /// Exceptions used: index into the rules' list, and the row read.
    pub used: Vec<(usize, Row)>,
    /// Anti-aircraft guns were unknown and the list says to treat them as
    /// none (decision 0207, first use).
    pub anti_aircraft_assumed: bool,
    /// The attributes estimated for this ship (decision 0207).
    pub estimates: Vec<EstimateUse>,
}

/// One estimate a ship's figures rest on (decision 0207).
#[derive(Clone, Debug)]
pub struct EstimateUse {
    /// The figure of the class file it gives.
    pub figure: &'static str,
    pub subject: Subject,
    pub attribute: String,
    /// The words of the method, for the rule column of the sources file.
    pub method_words: String,
    /// The working, for the note column of the sources file.
    pub note: String,
    /// One line for the check's count (rule 13), with the range (rule 6).
    pub line: String,
}

pub(crate) struct ShipInfo {
    pub(crate) row_id: String,
    pub(crate) name: String,
    pub(crate) class: String,
}

pub(crate) struct ClassInfo {
    pub(crate) row_id: String,
    pub(crate) name: String,
    pub(crate) kind_type: String,
}

pub struct Env<'a> {
    pub(crate) index: Index,
    pub(crate) ships: BTreeMap<String, ShipInfo>,
    pub(crate) classes: BTreeMap<String, ClassInfo>,
    pub(crate) aircraft: BTreeMap<String, AircraftInfo>,
    /// Weapon key to its `kind` in weapons.csv.
    pub(crate) weapons: BTreeMap<String, String>,
    /// Weapon key to its name in weapons.csv.
    pub(crate) weapon_names: BTreeMap<String, String>,
    /// The rows of engagement_events.csv by id, for the rows a list's
    /// `torpedo_from` entry cites (decision 0218, rule 29).
    pub(crate) events: BTreeMap<String, crate::submarine::EventRow>,
    pub(crate) date: Day,
    pub(crate) rules: &'a Rules,
    /// The achieved rates of fire, when the rules have a `[fire_rate]`
    /// block and every case in it is sound (decision 0070).
    pub(crate) fire_rate: Option<crate::fire_rate::Achieved>,
    /// Problems with that block's cases, read against the facts.
    pub fire_rate_problems: Vec<Problem>,
    /// The runs and voyage of the troop figure, when the list asks for it
    /// (decision 0093, 8).
    pub(crate) troops: Option<crate::troops::TroopEnv>,
    /// The rows read as of a later date on this list's date, with their
    /// `valid_from` (decision 0200).
    pub(crate) later: BTreeMap<Cite, String>,
    /// The list's estimates (decision 0207, rule 4).
    pub(crate) estimates: Vec<Estimate>,
    /// Every estimate of every list, this one's among them, by the list's
    /// file name: subject and attribute (decision 0207, rule 8).
    pub(crate) all_estimates: Vec<(String, Subject, String)>,
    /// The rows of gaps.csv: entity type and the entities it names.
    pub(crate) gaps: Vec<(String, Vec<String>)>,
    /// The list's file name, as `all_estimates` names it.
    pub(crate) list_name: String,
    /// Classes the list leaves out without `like_class = true`: in no pool
    /// of like classes (decision 0225, the ruling of Nov 23, 2026).
    pub(crate) not_like: BTreeSet<String>,
}

/// A row of `aircraft_types.csv`.
pub(crate) struct AircraftInfo {
    pub row_id: String,
    pub name: String,
    pub role: String,
}

/// Where a figure's value came from.
#[derive(Clone)]
pub(crate) struct Got<T> {
    pub value: T,
    pub cites: BTreeSet<Cite>,
    pub excepted: bool,
    pub doubts: BTreeSet<String>,
}

pub(crate) struct Work<'a> {
    pub subject: &'a str,
    pub problems: Vec<Problem>,
    pub used: Vec<(usize, Row)>,
    /// The estimates read for this ship (decision 0207).
    pub estimated: Vec<EstimateUse>,
}

impl Work<'_> {
    pub(crate) fn fail(&mut self, kind: Kind, figure: &str, message: String) {
        self.problems
            .push(Problem::new(self.subject, kind, figure, message));
    }

    pub(crate) fn failed(&mut self, figure: &str, failure: Failure) {
        self.fail(failure.kind, figure, failure.message);
    }

    /// Notes the exceptions a read used and returns the rows cited, whether
    /// an exception was used, and the ship rows passed over.
    pub(crate) fn take(&mut self, read: &Read) -> (BTreeSet<Cite>, bool, BTreeSet<String>) {
        self.used.extend(read.excepted.iter().cloned());
        (
            read.rows.iter().map(Row::cite_key).collect(),
            !read.excepted.is_empty(),
            read.doubts.iter().cloned().collect(),
        )
    }
}

pub(crate) fn number(read: &Read, fact: &str) -> Result<Dec, Failure> {
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

pub(crate) fn count(read: &Read, slot: &str) -> Result<u32, Failure> {
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
        notes: BTreeSet::new(),
        rule: None,
        estimate: None,
    });
}

impl<T> Got<T> {
    /// The same sources with another value.
    pub(crate) fn map_value<U>(&self, value: U) -> Got<U> {
        Got {
            value,
            cites: self.cites.clone(),
            excepted: self.excepted,
            doubts: self.doubts.clone(),
        }
    }

    /// The sources of this value, for a figure derived from it.
    pub(crate) fn bare(&self) -> Got<()> {
        Got {
            value: (),
            cites: self.cites.clone(),
            excepted: self.excepted,
            doubts: self.doubts.clone(),
        }
    }
}

/// The step from an estimated attribute to the figure of the class file,
/// for the rule column of the sources file (decision 0207, rule 9).
fn estimate_step(figure: &str, attribute: &str) -> &'static str {
    match (figure, attribute) {
        ("toughness", _) => "toughness is it / displacement_lt_per_toughness",
        (_, "fuel_capacity_bbl" | "cargo_oil_capacity_bbl") => {
            "the figure is it x fuel_oil_barrel_factor_lt of the ship, else of its class (decision 0206, rule 8)"
        }
        _ => "the figure is it (a simulation ton is a long ton)",
    }
}

pub(crate) fn overflow(what: &str) -> Failure {
    Failure {
        kind: Kind::BadValue,
        message: format!("{what} does not fit"),
    }
}

/// What a fit says: how many, of what.
pub(crate) struct Fit {
    pub count: u32,
    pub item: String,
}

impl<'a> Env<'a> {
    pub fn build(data: &Data, date: Day, rules: &'a Rules) -> Env<'a> {
        let mut ships = BTreeMap::new();
        for rec in data.rows("ships.csv") {
            ships.insert(
                rec.get("key").to_string(),
                ShipInfo {
                    row_id: rec.get("id").to_string(),
                    name: rec.get("name").to_string(),
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
        let mut aircraft = BTreeMap::new();
        for rec in data.rows("aircraft_types.csv") {
            aircraft.insert(
                rec.get("key").to_string(),
                AircraftInfo {
                    row_id: rec.get("id").to_string(),
                    name: rec.get("name").to_string(),
                    role: rec.get("role").to_string(),
                },
            );
        }
        let weapons = data
            .rows("weapons.csv")
            .into_iter()
            .map(|rec| (rec.get("key").to_string(), rec.get("kind").to_string()))
            .collect();
        let weapon_names = data
            .rows("weapons.csv")
            .into_iter()
            .map(|rec| (rec.get("key").to_string(), rec.get("name").to_string()))
            .collect();
        let events = data
            .rows("engagement_events.csv")
            .into_iter()
            .map(|rec| {
                (
                    rec.get("id").to_string(),
                    crate::submarine::EventRow {
                        time: rec.get("time").to_string(),
                        actor: rec.get("actor").to_string(),
                        means: rec.get("means").to_string(),
                        standing: rec.get("standing").to_string(),
                    },
                )
            })
            .collect();
        let mut env = Env {
            index: Index::build(data),
            ships,
            classes,
            aircraft,
            weapons,
            weapon_names,
            events,
            date,
            rules,
            fire_rate: None,
            fire_rate_problems: Vec::new(),
            troops: None,
            later: BTreeMap::new(),
            estimates: Vec::new(),
            all_estimates: Vec::new(),
            list_name: String::new(),
            not_like: BTreeSet::new(),
            gaps: data
                .rows("gaps.csv")
                .into_iter()
                .map(|rec| {
                    (
                        rec.get("entity_type").to_string(),
                        rec.get("entity")
                            .split(',')
                            .map(|e| e.trim().to_string())
                            .collect(),
                    )
                })
                .collect(),
        };
        env.later = env.later_rows();
        if let Some(fire_rate) = &rules.fire_rate {
            let (achieved, problems) = crate::fire_rate::compute(&env, data, fire_rate);
            env.fire_rate = achieved;
            env.fire_rate_problems = problems;
        }
        env
    }

    /// Every exception of the rules file, the top-level list and then the
    /// search radius block's.
    fn all_exceptions(&self) -> impl Iterator<Item = &Exception> {
        let search = self
            .rules
            .search_radius
            .as_ref()
            .map_or(&[][..], |s| s.exceptions.as_slice());
        self.rules.exceptions.iter().chain(search)
    }

    /// The rows an exception saying `later = true` reads on this list's
    /// date, each with its `valid_from` (decision 0200, 5 and 9): a figure
    /// citing one is marked as read from a row of a later date.
    fn later_rows(&self) -> BTreeMap<Cite, String> {
        let mut rows = BTreeMap::new();
        for exception in self.all_exceptions().filter(|e| e.later) {
            for row in self.index.named_by(exception) {
                if begins_after(&row.from, self.date).unwrap_or(false) {
                    rows.insert(row.cite_key(), row.from.clone());
                }
            }
        }
        rows
    }

    /// The exceptions saying `later = true` that do nothing on this list,
    /// because the row they name does not begin after its date (decision
    /// 0200, 9): one line each, for the derived files' header.
    pub fn idle_later(&self) -> Vec<String> {
        let mut lines = BTreeSet::new();
        for exception in self.all_exceptions().filter(|e| e.later) {
            for row in self.index.named_by(exception) {
                if !begins_after(&row.from, self.date).unwrap_or(false) {
                    lines.insert(format!(
                        "{} {}: {} row {} (valid from {}) does not begin after {}",
                        exception.entity,
                        exception.fact,
                        row.file,
                        row.id,
                        if row.from.is_empty() { "-" } else { &row.from },
                        self.date
                    ));
                }
            }
        }
        lines.into_iter().collect()
    }

    /// Marks each figure that cites a row read as of a later date (decision
    /// 0200, 5): the words of the rule and the row's date.
    pub(crate) fn later_notes(&self, cites: &BTreeSet<Cite>) -> Vec<String> {
        cites
            .iter()
            .filter_map(|cite| {
                self.later.get(cite).map(|from| {
                    format!(
                        "read from a row of a later date: {}:{}, valid from {from} (decision 0200)",
                        cite.0, cite.1
                    )
                })
            })
            .collect()
    }

    /// Problems for exceptions that name no row of the facts.
    pub fn exception_problems(&self) -> Vec<Problem> {
        self.all_exceptions()
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

    pub(crate) fn fact(
        &self,
        w: &mut Work,
        keys: (&str, &str),
        attribute: &str,
        figure: &str,
    ) -> Option<Got<Dec>> {
        let (ship, class) = keys;
        if let Some(entry) = self.estimate_for(ship, class, attribute) {
            return self.estimated(w, keys, entry, figure);
        }
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
        self.fit_read_on(keys, slot, self.date)
    }

    /// A fit as it stood on a date (a case of decision 0070 reads the guns
    /// on the date of its engagement).
    pub(crate) fn fit_read_on(
        &self,
        keys: (&str, &str),
        slot: &str,
        date: Day,
    ) -> Result<Read, Failure> {
        let (ship, class) = keys;
        resolve_over(
            self.index.rows(Table::ShipFits, ship, slot),
            &format!("{slot} fit of ship {ship}"),
            self.index.rows(Table::ClassFits, class, slot),
            &format!("{slot} fit of class {class}"),
            date,
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
    pub(crate) fn weapon_fact(
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
    ///
    /// The one way in for a designed rate: an exception of the rules file
    /// naming a `rate_of_fire_rpm` row of the weapon. Then that row is read
    /// instead of the practical rate, whatever its standing. The result says
    /// whether the rate is a designed one.
    fn rate_of_fire(&self, w: &mut Work, fit: &Got<Fit>) -> Option<(Got<Dec>, bool)> {
        let weapon = &fit.value.item;
        if let Some((index, exception)) = self
            .rules
            .exceptions
            .iter()
            .enumerate()
            .find(|(_, e)| e.entity == *weapon && e.fact == DESIGNED_RATE)
        {
            let got = self.weapon_fact(w, fit, DESIGNED_RATE, FIGURES[7])?;
            let Some(named) = self
                .index
                .rows(Table::WeaponFacts, weapon, DESIGNED_RATE)
                .iter()
                .find(|r| exception.matches(r))
            else {
                w.fail(
                    Kind::Missing,
                    FIGURES[7],
                    format!(
                        "{DESIGNED_RATE} of weapon {weapon}: the exception names row {}, which is not a row of the weapon",
                        exception.row
                    ),
                );
                return None;
            };
            let cite = named.cite_key();
            if !got.cites.contains(&cite) {
                w.fail(
                    Kind::Missing,
                    FIGURES[7],
                    format!(
                        "{DESIGNED_RATE} of weapon {weapon}: the exception names row {}, which does not hold on {}",
                        named.cite(),
                        self.date
                    ),
                );
                return None;
            }
            if !w.used.iter().any(|(i, r)| *i == index && r.id == named.id) {
                w.used.push((index, named.clone()));
            }
            return Some((got, true));
        }
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
            .map(|got| (got, false))
    }

    /// The rate of fire in action (decision 0070): the lesser of the sourced
    /// rate and the achieved rate of the class's kind of battery. Without a
    /// `[fire_rate]` block, or for a class whose kind has no achieved rate,
    /// the sourced rate alone. Returns the figure, whether the sourced rate
    /// was a designed one, and the note for the sources file.
    fn rate_in_action(
        &self,
        w: &mut Work,
        main: &Got<Fit>,
        class_key: &str,
        kind_type: &str,
    ) -> Option<(Got<Dec>, bool, Option<String>)> {
        let weapon = &main.value.item;
        let block = self.rules.fire_rate.as_ref();
        let no_sourced = block.and_then(|b| b.no_sourced_rate(class_key));
        if let Some(n) = no_sourced {
            let id = n.row.to_string();
            let named = self
                .index
                .rows_of_entity(Table::WeaponFacts, weapon)
                .any(|r| r.id == id);
            if !named {
                w.fail(
                    Kind::Exception,
                    FIGURES[7],
                    format!(
                        "fire_rate no_sourced_rate for {class_key} names weapon_facts.csv row {}, which is not a row of weapon {weapon}",
                        n.row
                    ),
                );
                return None;
            }
        }
        let simulation_kind = KIND_FOR_TYPE
            .iter()
            .find(|(t, _)| *t == kind_type)
            .map(|(_, k)| *k);
        let achieved = match (block, &self.fire_rate, simulation_kind) {
            (Some(b), Some(a), Some(sim)) => b.kind_of(class_key, sim).and_then(|kind| {
                a.medians
                    .get(&kind)
                    .map(|(median, n)| (b, a, kind, *median, *n))
            }),
            _ => None,
        };
        let designed_named = self
            .rules
            .exceptions
            .iter()
            .any(|e| e.entity == *weapon && e.fact == DESIGNED_RATE);
        let practical_absent = self
            .index
            .rows(Table::WeaponFacts, weapon, RATE_OF_FIRE)
            .is_empty();
        let sourced_skipped =
            no_sourced.is_some() || (achieved.is_some() && !designed_named && practical_absent);
        let sourced = if sourced_skipped {
            None
        } else {
            Some(self.rate_of_fire(w, main)?)
        };
        let Some((block, achieved_set, kind, median, n)) = achieved else {
            return match sourced {
                Some((got, designed)) => Some((got, designed, None)),
                None => {
                    w.fail(
                        Kind::Missing,
                        FIGURES[7],
                        format!(
                            "no sourced rate (weapon_facts.csv row {} is not read for class {class_key}) and no achieved rate of its kind",
                            no_sourced.map_or(0, |n| n.row)
                        ),
                    );
                    None
                }
            };
        };
        let mut cites: BTreeSet<Cite> = BTreeSet::new();
        for case in achieved_set.cases_of(block, kind) {
            cites.extend(case.cites.iter().cloned());
            for (index, row) in &case.used {
                if !w
                    .used
                    .iter()
                    .any(|(i, r)| i == index && r.file == row.file && r.id == row.id)
                {
                    w.used.push((*index, row.clone()));
                }
            }
        }
        if let Some(c) = block.continuous_class(class_key) {
            cites.extend(c.rows.iter().map(|r| {
                (
                    "engagement_outcomes.csv".to_string(),
                    u64::try_from(*r).unwrap_or(0),
                )
            }));
        }
        let cases = format!(
            "the median of {n} {}",
            if n == 1 { "case" } else { "cases" }
        );
        let (value, base, designed, note) = match sourced {
            Some((got, designed)) => {
                let value = got.value.min(median);
                let note = format!(
                    "rate in action (decision 0070): the lesser of the sourced rate {} and the achieved rate {median} of kind {}, {cases}",
                    got.value,
                    kind.name()
                );
                (value, got, designed, note)
            }
            None => {
                let why = match no_sourced {
                    Some(n) => format!("weapon_facts.csv row {} not read for this class", n.row),
                    None => format!("no {RATE_OF_FIRE} row of weapon {weapon}"),
                };
                let note = format!(
                    "no sourced rate ({why}); the achieved rate {median} of kind {}, {cases}",
                    kind.name()
                );
                (median, main.map_value(Dec::ZERO), false, note)
            }
        };
        let mut got = base;
        got.value = value;
        got.cites.extend(cites);
        Some((got, designed, Some(note)))
    }

    /// Rounds per gun. A ship the list names, with no `main_rounds_per_gun`
    /// row of its own or its class at all, gets
    /// [`AMMUNITION_NOT_LIMITING_ROUNDS`]; rows that exist but cannot be
    /// read are still a problem. The flag says the figure is the assumption.
    fn rounds_per_gun(
        &self,
        w: &mut Work,
        keys: (&str, &str),
        assume_not_limiting: bool,
    ) -> Option<(Got<Dec>, bool)> {
        let (ship, class) = keys;
        if assume_not_limiting
            && self.index.rows(Table::ShipFacts, ship, ROUNDS).is_empty()
            && self.index.rows(Table::ClassFacts, class, ROUNDS).is_empty()
        {
            let value = Dec::from_int(AMMUNITION_NOT_LIMITING_ROUNDS)?;
            return Some((
                Got {
                    value,
                    cites: BTreeSet::new(),
                    excepted: false,
                    doubts: BTreeSet::new(),
                },
                true,
            ));
        }
        self.fact(w, keys, ROUNDS, FIGURES[8])
            .map(|got| (got, false))
    }

    /// Whether the ship's class is of the type oiler (decision 0206, rule 1).
    /// A ship or class not on file is not.
    pub fn is_oiler(&self, ship: &str) -> bool {
        self.ships
            .get(ship)
            .and_then(|s| self.classes.get(&s.class))
            .is_some_and(|c| c.kind_type == OILER_TYPE)
    }

    /// An oiler's figure in barrels made long tons by the factor (decision
    /// 0206, rules 6 to 8): the barrels and the factor are each read as any
    /// fact is, and the figure cites both.
    fn barrels_in_tons(
        &self,
        w: &mut Work,
        keys: (&str, &str),
        attribute: &str,
        figure: &str,
        factor: &Option<Got<Dec>>,
    ) -> Option<(Value, Got<()>, BTreeSet<String>)> {
        let barrels = self.fact(w, keys, attribute, figure);
        let (Some(barrels), Some(factor)) = (barrels, factor) else {
            return None;
        };
        let Some(tons) = barrels.value.mul(factor.value) else {
            w.failed(figure, overflow(figure));
            return None;
        };
        let mut sources = barrels.bare();
        sources.cites.extend(factor.cites.iter().cloned());
        sources.excepted |= factor.excepted;
        sources.doubts.extend(factor.doubts.iter().cloned());
        let notes = BTreeSet::from([format!(
            "{} barrels x {} long tons a barrel",
            barrels.value, factor.value
        )]);
        Some((Value::Num(tons), sources, notes))
    }

    /// The list's estimate of `attribute` for the ship or her class, if it
    /// has one (decision 0207, rule 4).
    pub(crate) fn estimate_for(
        &self,
        ship: &str,
        class: &str,
        attribute: &str,
    ) -> Option<&Estimate> {
        self.estimates.iter().find(|e| {
            e.attribute == attribute
                && match &e.subject {
                    Subject::Ship(key) => key == ship,
                    Subject::Class(key) => key == class,
                }
        })
    }

    /// The list, of all the lists, that estimates one of `attributes` for the
    /// class `class` or a ship of it (decision 0207, rule 8).
    fn estimated_in(&self, class: &str, attributes: &[&str]) -> Option<&str> {
        self.estimated_in_but(class, attributes, None)
    }

    /// As [`Self::estimated_in`], passing over one entry of this list's own.
    fn estimated_in_but(
        &self,
        class: &str,
        attributes: &[&str],
        but: Option<&Estimate>,
    ) -> Option<&str> {
        self.all_estimates
            .iter()
            .find(|(list, subject, attribute)| {
                let itself = but.is_some_and(|e| {
                    *list == self.list_name && e.subject == *subject && e.attribute == *attribute
                });
                !itself
                    && attributes.contains(&attribute.as_str())
                    && match subject {
                        Subject::Class(key) => key == class,
                        Subject::Ship(key) => self.ships.get(key).is_some_and(|s| s.class == class),
                    }
            })
            .map(|(list, _, _)| list.as_str())
    }

    /// Whether a row of gaps.csv says the attribute was looked for: one of
    /// the subject's kind naming the subject (decision 0207, rule 2).
    fn has_gap(&self, subject: &Subject) -> bool {
        let kind = match subject {
            Subject::Ship(_) => "ship",
            Subject::Class(_) => "ship_class",
        };
        self.gaps
            .iter()
            .any(|(t, entities)| t == kind && entities.iter().any(|e| e == subject.key()))
    }

    /// Problems of the entries themselves, before any ship is derived: one
    /// that names its like classes (rule 6), saying which of them rest on an
    /// estimate (rule 8).
    pub fn estimate_entry_problems(&self) -> Vec<Problem> {
        let mut problems = Vec::new();
        for entry in &self.estimates {
            if entry.named_like.is_empty() {
                continue;
            }
            let mut message = format!(
                "{} {}: the entry names its like classes ({}); the like classes are not chosen: every class of the type on file with both rows is read (decision 0207, rule 6)",
                entry.subject.words(),
                entry.attribute,
                entry.named_like.join(", ")
            );
            for class in &entry.named_like {
                if let Some(list) =
                    self.estimated_in_but(class, &[entry.attribute.as_str()], Some(entry))
                {
                    message.push_str(&format!(
                        "; class {class} has its {} estimated in {list}, and an estimate never rests on an estimate (decision 0207, rule 8)",
                        entry.attribute
                    ));
                }
            }
            problems.push(Problem::new(
                entry.subject.key(),
                Kind::Estimate,
                entry.attribute.clone(),
                message,
            ));
        }
        problems
    }

    /// An attribute the list estimates (decision 0207): refused where any
    /// row of it is filed for the ship or her class (rule 11) or no gap row
    /// says it was looked for (rule 2); otherwise worked by the entry's
    /// method (rule 5) and noted for the figure.
    fn estimated(
        &self,
        w: &mut Work,
        keys: (&str, &str),
        entry: &Estimate,
        figure: &str,
    ) -> Option<Got<Dec>> {
        let (ship, class) = keys;
        let attribute = entry.attribute.as_str();
        let filed: Vec<String> = self
            .index
            .rows(Table::ShipFacts, ship, attribute)
            .iter()
            .chain(self.index.rows(Table::ClassFacts, class, attribute))
            .map(Row::cite)
            .collect();
        if !filed.is_empty() {
            w.fail(
                Kind::Estimate,
                figure,
                format!(
                    "{attribute} of {}: a row is filed ({}), so a fact drives the estimate out: take out the list's [[estimate]] entry for {} {attribute} (decision 0207, rule 11)",
                    if ship.is_empty() {
                        format!("class {class}")
                    } else {
                        format!("ship {ship} or class {class}")
                    },
                    filed.join(", "),
                    entry.subject.words()
                ),
            );
            return None;
        }
        if !self.has_gap(&entry.subject) {
            w.fail(
                Kind::Estimate,
                figure,
                format!(
                    "{attribute} of {}: no row of gaps.csv names her as looked for, and an estimate is allowed only over a gap (decision 0207, rule 2)",
                    entry.subject.words()
                ),
            );
            return None;
        }
        let worked = match &entry.method {
            Method::Ratio { from } => self.by_ratio(w, keys, entry, from, figure),
            Method::Sisters => self.as_sisters(w, keys, entry, figure),
        }?;
        let (got, method_words, working, line) = worked;
        let by = format!(
            "entry by {}, {}; reason: {}",
            entry.by,
            entry.date,
            crate::output::one_line(&entry.reason)
        );
        w.estimated.push(EstimateUse {
            figure: FIGURES
                .iter()
                .chain(AIR_FIGURES.iter())
                .chain(std::iter::once(&crate::submarine::PASSAGE_SPEED))
                .copied()
                .find(|f| *f == figure)
                .unwrap_or("?"),
            subject: entry.subject.clone(),
            attribute: attribute.to_string(),
            method_words,
            note: format!("{working}; {by}"),
            line: format!(
                "{} {attribute}: {line}",
                if ship.is_empty() { class } else { ship }
            ),
        });
        Some(got)
    }

    /// By ratio (decision 0207, rules 5 and 6): her own row of `from`, times
    /// the mean over every like class of its `attribute` over its `from`,
    /// each division cut at the sixth place. The like classes are every
    /// class of her class's type on file, her own left out, with class rows
    /// of both, but for a class the list leaves out without `like_class =
    /// true` (decision 0225, the ruling of Nov 23, 2026); each is read as
    /// any row is, and one that cannot be read, or
    /// that has either attribute estimated in any list (rule 8), fails the
    /// estimate rather than being passed over.
    #[allow(clippy::type_complexity)]
    fn by_ratio(
        &self,
        w: &mut Work,
        keys: (&str, &str),
        entry: &Estimate,
        from: &str,
        figure: &str,
    ) -> Option<(Got<Dec>, String, String, String)> {
        let (ship, class) = keys;
        let attribute = entry.attribute.as_str();
        let who = if ship.is_empty() {
            format!("class {class}")
        } else {
            format!("ship {ship}")
        };
        if self.estimate_for(ship, class, from).is_some() {
            w.fail(
                Kind::Estimate,
                figure,
                format!(
                    "{attribute} of {who}: it is worked from {from}, which the list also estimates, and an estimate never rests on an estimate (decision 0207, rule 8)"
                ),
            );
            return None;
        }
        let own = resolve_over(
            self.index.rows(Table::ShipFacts, ship, from),
            &format!("{from} of {who} (her own row, for an estimate by ratio)"),
            self.index.rows(Table::ClassFacts, class, from),
            &format!("{from} of class {class} (for an estimate by ratio)"),
            self.date,
            &self.rules.exceptions,
        )
        .and_then(|read| number(&read, from).map(|v| (read, v)));
        let (own_read, own) = match own {
            Ok(pair) => pair,
            Err(failure) => {
                w.failed(figure, failure);
                return None;
            }
        };
        let kind_type = self
            .classes
            .get(class)
            .map(|c| c.kind_type.clone())
            .unwrap_or_default();
        let mut likes: Vec<(String, Dec, Dec, Dec, Read, Read)> = Vec::new();
        let mut ok = true;
        for (key, info) in &self.classes {
            if info.kind_type != kind_type || key == class || self.not_like.contains(key) {
                continue;
            }
            let a_rows = self.index.rows(Table::ClassFacts, key, attribute);
            let f_rows = self.index.rows(Table::ClassFacts, key, from);
            if a_rows.is_empty() || f_rows.is_empty() {
                continue;
            }
            if let Some(list) = self.estimated_in(key, &[attribute, from]) {
                w.fail(
                    Kind::Estimate,
                    figure,
                    format!(
                        "{attribute} of {who}: class {key} is of the type {kind_type} with rows of both {attribute} and {from}, and has one of them estimated in {list}: an estimate never rests on an estimate (decision 0207, rule 8)"
                    ),
                );
                ok = false;
                continue;
            }
            let read = |rows: &[Row], what: &str| {
                resolve(
                    rows,
                    &format!("{what} of class {key} (a like class of {who})"),
                    self.date,
                    &self.rules.exceptions,
                )
                .and_then(|read| number(&read, what).map(|v| (read, v)))
            };
            match (read(a_rows, attribute), read(f_rows, from)) {
                (Ok((a_read, a)), Ok((f_read, f))) => {
                    match a.div(f) {
                        Some(ratio) if f.is_positive() => {
                            likes.push((key.clone(), a, f, ratio, a_read, f_read));
                        }
                        _ => {
                            w.fail(
                            Kind::BadValue,
                            figure,
                            format!("like class {key}: {attribute} {a} / {from} {f} cannot be worked"),
                        );
                            ok = false;
                        }
                    }
                }
                (a, f) => {
                    for failure in [a.err(), f.err()].into_iter().flatten() {
                        w.fail(
                            failure.kind,
                            figure,
                            format!(
                                "{}; the like classes are not chosen, every one with both rows is read (decision 0207, rule 6)",
                                failure.message
                            ),
                        );
                    }
                    ok = false;
                }
            }
        }
        if !ok {
            return None;
        }
        if likes.is_empty() {
            w.fail(
                Kind::Missing,
                figure,
                format!(
                    "{attribute} of {who}: no like class: no other class of the type {kind_type} on file has rows of both {attribute} and {from} (decision 0207, rule 2)"
                ),
            );
            return None;
        }
        let sum = likes
            .iter()
            .try_fold(Dec::ZERO, |sum, like| sum.checked_add(like.3));
        let n = Dec::from_int(likes.len() as i64);
        let Some(mean) = sum.zip(n).and_then(|(sum, n)| sum.div(n)) else {
            w.failed(figure, overflow("the mean of the like classes' ratios"));
            return None;
        };
        let least = likes.iter().min_by_key(|l| l.3).expect("one like class");
        let greatest = likes.iter().max_by_key(|l| l.3).expect("one like class");
        let (Some(value), Some(low), Some(high)) =
            (own.mul(mean), own.mul(least.3), own.mul(greatest.3))
        else {
            w.failed(figure, overflow("the estimate"));
            return None;
        };
        let mut got = Got {
            value,
            cites: BTreeSet::new(),
            excepted: false,
            doubts: BTreeSet::new(),
        };
        let cite_list = |read: &Read| {
            read.rows
                .iter()
                .map(Row::cite)
                .collect::<Vec<_>>()
                .join(" ")
        };
        let own_cites = cite_list(&own_read);
        let mut reads: Vec<&Read> = vec![&own_read];
        let mut said = Vec::new();
        for (key, a, f, ratio, a_read, f_read) in &likes {
            said.push(format!(
                "{key} {a} / {f} = {ratio} ({} {})",
                cite_list(a_read),
                cite_list(f_read)
            ));
            reads.push(a_read);
            reads.push(f_read);
        }
        for read in reads {
            let (cites, excepted, doubts) = w.take(read);
            got.cites.extend(cites);
            got.excepted |= excepted;
            got.doubts.extend(doubts);
        }
        let method_words = format!(
            "estimated, not read from a row (decision 0207): {attribute} is {from} of the ship, else of its class, x the mean of {attribute} / {from} over every like class, each class of the type {kind_type} on file with class rows of both, her own left out (rules 5 and 6)"
        );
        let working = format!(
            "estimated by ratio: her {from} {own} ({own_cites}) x {mean}, the mean over {} like classes of {}; {attribute} {value}; least {low} ({}), greatest {high} ({})",
            likes.len(),
            said.join(", "),
            least.0,
            greatest.0
        );
        let line = format!(
            "by ratio, from her {from} {own}, x the mean {mean} over {} like classes ({}): {value}; least {low} ({}), greatest {high} ({})",
            likes.len(),
            likes
                .iter()
                .map(|l| format!("{} {}", l.0, l.3))
                .collect::<Vec<_>>()
                .join(", "),
            least.0,
            greatest.0
        );
        Some((got, method_words, working, line))
    }

    /// As her sisters (decision 0207, rule 5): the least of the values the
    /// other ships of her class in ships.csv give in rows of their own, each
    /// read as any ship's row is on the list's date; a sister whose row
    /// cannot be read on it is passed over.
    #[allow(clippy::type_complexity)]
    fn as_sisters(
        &self,
        w: &mut Work,
        keys: (&str, &str),
        entry: &Estimate,
        figure: &str,
    ) -> Option<(Got<Dec>, String, String, String)> {
        let (ship, class) = keys;
        let attribute = entry.attribute.as_str();
        let mut read_sisters: Vec<(String, Dec, Read)> = Vec::new();
        for (sister, info) in &self.ships {
            if sister == ship || info.class != class {
                continue;
            }
            let rows = self.index.rows(Table::ShipFacts, sister, attribute);
            if rows.is_empty() {
                continue;
            }
            match resolve(
                rows,
                &format!("{attribute} of ship {sister} (a sister of {ship})"),
                self.date,
                &self.rules.exceptions,
            ) {
                Ok(read) => match number(&read, attribute) {
                    Ok(value) => read_sisters.push((sister.clone(), value, read)),
                    Err(failure) => {
                        w.failed(figure, failure);
                        return None;
                    }
                },
                Err(failure)
                    if matches!(
                        failure.kind,
                        Kind::Missing | Kind::NotPreferred | Kind::NotOnDate
                    ) => {}
                Err(failure) => {
                    w.failed(figure, failure);
                    return None;
                }
            }
        }
        let Some(least) = read_sisters.iter().map(|(_, v, _)| *v).min() else {
            w.fail(
                Kind::Missing,
                figure,
                format!(
                    "{attribute} of ship {ship}: no row of her own or of class {class}, and no sister of her class has a readable row (an estimate as her sisters, decision 0207, rule 5)"
                ),
            );
            return None;
        };
        let mut got = Got {
            value: least,
            cites: BTreeSet::new(),
            excepted: false,
            doubts: BTreeSet::new(),
        };
        let mut said = Vec::new();
        for (sister, value, read) in &read_sisters {
            let (cites, excepted, doubts) = w.take(read);
            said.push(format!(
                "{sister} {value} ({})",
                read.rows
                    .iter()
                    .map(Row::cite)
                    .collect::<Vec<_>>()
                    .join(" ")
            ));
            got.cites.extend(cites);
            got.excepted |= excepted;
            got.doubts.extend(doubts);
        }
        let method_words = format!(
            "estimated, not read from a row (decision 0207): {attribute} is the least of the rows of the other ships of her class in ships.csv, as her sisters, where she and her class have none (rule 5)"
        );
        let working = format!(
            "estimated as her sisters: no row of her own or of her class; the least of her sisters' rows, {least}, of {}",
            said.join(", ")
        );
        let line = format!("as her sisters, the least of {}: {least}", said.join(", "));
        Some((got, method_words, working, line))
    }

    /// Derives every figure of a ship. All problems are returned, not the
    /// first. `assume_no_radar` is the list's working assumption.
    pub fn derive_ship(&self, ship: &str, opts: ShipOpts) -> Result<ShipResult, Vec<Problem>> {
        let ShipOpts {
            assume_no_radar,
            assume_ammunition_not_limiting,
            air_figures,
            surface_weapons_not_modelled,
            mark_column,
            assume_no_anti_aircraft,
        } = opts;
        let mut w = Work {
            subject: ship,
            problems: Vec::new(),
            used: Vec::new(),
            estimated: Vec::new(),
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
        let troop_index = FIGURES.len() + if air_figures { AIR_FIGURES.len() } else { 0 };
        let mark_index = troop_index + usize::from(self.troops.is_some());
        let figure_count = mark_index + usize::from(mark_column);
        let oiler = class.kind_type == OILER_TYPE;
        let unarmed = KIND_FOR_TYPE
            .iter()
            .any(|(t, kind)| *t == class.kind_type && UNARMED_KINDS.contains(kind));
        // A list may not also call an oiler's weapons an assumption of the
        // scenario: she has none by her kind.
        let surface_weapons_not_modelled = surface_weapons_not_modelled && !unarmed;
        let mut figs: Vec<Option<Figure>> = vec![None; figure_count];
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
                if let (true, Some(figure)) = (oiler, figs[KIND].as_mut()) {
                    figure.rule = Some(OILER_KIND_RULE);
                }
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
            if oiler && i == 3 {
                // Her own fuel is read below, in barrels or by weight.
                continue;
            }
            if let Some(got) = self.fact(&mut w, keys, attribute, FIGURES[i]) {
                let (v, sources) = num(got);
                put(&mut figs, i, v, sources);
            }
        }
        if oiler {
            // Her own fuel and her cargo of oil, in barrels made tons by the
            // Navy's factor where a row (or the list's estimate) gives them
            // in barrels (decision 0206, rules 6 to 8), else by weight in
            // long tons (decision 0207).
            let in_barrels = |attribute: &str| {
                !self
                    .index
                    .rows(Table::ShipFacts, ship, attribute)
                    .is_empty()
                    || !self
                        .index
                        .rows(Table::ClassFacts, class_key, attribute)
                        .is_empty()
                    || self.estimate_for(ship, class_key, attribute).is_some()
            };
            let mut factor: Option<Option<Got<Dec>>> = None;
            for (i, barrels, tons, barrels_rule, tons_rule) in [
                (3, FUEL_BBL, "fuel_capacity_lt", OILER_FUEL_RULE, None),
                (
                    CARGO,
                    CARGO_BBL,
                    CARGO_LT,
                    OILER_CARGO_RULE,
                    Some(OILER_CARGO_WEIGHT_RULE),
                ),
            ] {
                if in_barrels(barrels) {
                    let factor = factor
                        .get_or_insert_with(|| self.fact(&mut w, keys, BARREL_FACTOR, FIGURES[3]));
                    if let Some((value, sources, notes)) =
                        self.barrels_in_tons(&mut w, keys, barrels, FIGURES[i], factor)
                    {
                        put(&mut figs, i, value, sources);
                        if let Some(figure) = figs[i].as_mut() {
                            figure.notes = notes;
                            figure.rule = Some(barrels_rule);
                        }
                    }
                } else if let Some(got) = self.fact(&mut w, keys, tons, FIGURES[i]) {
                    let (v, sources) = num(got);
                    put(&mut figs, i, v, sources);
                    if let Some(figure) = figs[i].as_mut() {
                        figure.rule = tons_rule;
                    }
                }
            }
        } else {
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
        }

        // main battery, ammunition and torpedoes
        let mut designed_rate = false;
        let mut ammunition_assumed = false;
        let mut tubes_not_split = false;
        if unarmed {
            for i in SURFACE_WEAPONS {
                put(
                    &mut figs,
                    i,
                    Value::Num(Dec::ZERO),
                    Got {
                        value: (),
                        cites: BTreeSet::new(),
                        excepted: false,
                        doubts: BTreeSet::new(),
                    },
                );
                if let Some(figure) = figs[i].as_mut() {
                    figure.notes.insert(UNARMED_NOTE.to_string());
                    figure.rule = Some(UNARMED_RULE);
                }
            }
        } else if surface_weapons_not_modelled {
            for i in SURFACE_WEAPONS {
                put(
                    &mut figs,
                    i,
                    Value::Num(Dec::ZERO),
                    Got {
                        value: (),
                        cites: BTreeSet::new(),
                        excepted: false,
                        doubts: BTreeSet::new(),
                    },
                );
                if let Some(figure) = figs[i].as_mut() {
                    figure.notes.insert(NOT_MODELLED_NOTE.to_string());
                }
            }
        } else {
            let main = self.fit(&mut w, keys, "main_battery", "main_guns");
            if let Some(main) = &main {
                put(
                    &mut figs,
                    6,
                    Value::Num(Dec::from_int(i64::from(main.value.count)).unwrap_or(Dec::ZERO)),
                    main.bare(),
                );
                if let Some((got, designed, note)) =
                    self.rate_in_action(&mut w, main, class_key, &class.kind_type)
                {
                    designed_rate = designed;
                    let (v, sources) = num(got);
                    put(&mut figs, 7, v, sources);
                    if let (Some(note), Some(figure)) = (note, figs[7].as_mut()) {
                        figure.notes.insert(note);
                    }
                }
            }
            if let Some((got, assumed)) =
                self.rounds_per_gun(&mut w, keys, assume_ammunition_not_limiting)
            {
                ammunition_assumed = assumed;
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
                if let Some((one_side, not_split)) = self.tubes_one_side(&mut w, keys, &tubes) {
                    tubes_not_split = not_split;
                    let value =
                        Value::Num(Dec::from_int(i64::from(one_side.value)).unwrap_or(Dec::ZERO));
                    put(&mut figs, 11, value, one_side.bare());
                    if let (true, Some(figure)) = (not_split, figs[11].as_mut()) {
                        figure.notes.insert(NOT_SPLIT_NOTE.to_string());
                    }
                }
                if tubes.value.count == 0 {
                    put(&mut figs, 12, Value::Num(Dec::ZERO), tubes.bare());
                } else if let Some(got) =
                    self.weapon_fact(&mut w, &tubes, "warhead_kg", FIGURES[12])
                {
                    match got.value.mul(self.rules.torpedo_damage_per_warhead_kg) {
                        Some(damage) => put(&mut figs, 12, Value::Num(damage), got.bare()),
                        None => w.failed(FIGURES[12], overflow("torpedo damage")),
                    }
                }
            }
        }

        // toughness: from the standard displacement of a warship, the light
        // displacement of a ship of another kind (decision 0206, rule 2)
        let displacement = if unarmed {
            LIGHT_DISPLACEMENT
        } else {
            "displacement_standard_lt"
        };
        if let Some(got) = self.fact(&mut w, keys, displacement, FIGURES[13]) {
            match got.value.div(self.rules.displacement_lt_per_toughness) {
                Some(toughness) if !toughness.is_positive() => w.fail(
                    Kind::ZeroToughness,
                    FIGURES[13],
                    format!(
                        "{displacement} {} / {} = {toughness}, which is not above zero",
                        got.value, self.rules.displacement_lt_per_toughness
                    ),
                ),
                Some(toughness) => {
                    put(&mut figs, 13, Value::Num(toughness), got.bare());
                    if let (true, Some(figure)) = (unarmed, figs[13].as_mut()) {
                        figure.rule = Some(AUXILIARY_TOUGHNESS_RULE);
                    }
                }
                None => w.failed(FIGURES[13], overflow("toughness")),
            }
        }

        // radar
        let mut radar_assumed = false;
        let mut radar_not_counted = Vec::new();
        if let Some((fitted, sources, not_counted)) = self.radar(&mut w, keys, assume_no_radar) {
            radar_assumed = fitted.1;
            put(&mut figs, RADAR, Value::Flag(fitted.0), sources);
            if let Some(figure) = figs[RADAR].as_mut() {
                for set in &not_counted {
                    figure.notes.insert(format!(
                        "not counted: no row gives the display of {set} (decision 0077)"
                    ));
                }
            }
            radar_not_counted = not_counted;
        }

        // aircraft capacity and anti-aircraft shots (decision 0029)
        let mut anti_aircraft_assumed = false;
        if air_figures {
            self.aircraft_capacity(&mut w, keys, class, &mut figs);
            anti_aircraft_assumed =
                self.anti_aircraft(&mut w, keys, &mut figs, assume_no_anti_aircraft);
        }

        // troop capacity (decision 0093, 8)
        if self.troops.is_some() {
            figs[troop_index] = self.troop_capacity(&mut w, ship, class_key, &class.kind_type);
        }

        // the mark that she fuels others at sea (decision 0206, rule 1)
        if mark_column {
            let cites = BTreeSet::from([
                (
                    "ships.csv".to_string(),
                    ship_info.row_id.parse().unwrap_or(0),
                ),
                (
                    "ship_classes.csv".to_string(),
                    class.row_id.parse().unwrap_or(0),
                ),
            ]);
            put(
                &mut figs,
                mark_index,
                Value::Flag(oiler),
                Got {
                    value: (),
                    cites,
                    excepted: false,
                    doubts: BTreeSet::new(),
                },
            );
            if let Some(figure) = figs[mark_index].as_mut() {
                figure.rule = Some(MARK_RULE);
            }
        }

        if !w.problems.is_empty() {
            return Err(w.problems);
        }
        let names: Vec<&str> = FIGURES.to_vec();
        let figures = figs
            .into_iter()
            .enumerate()
            .map(|(i, f)| {
                let mut f = f.expect("every figure is set when there is no problem");
                let later = self.later_notes(&f.cites);
                f.notes.extend(later);
                // A figure that rests on an estimate says so in place of the
                // rule of a figure read (decision 0207, rule 9).
                if let Some(name) = names.get(i) {
                    for used in w.estimated.iter().filter(|u| u.figure == *name) {
                        f.estimate = Some(format!(
                            "{}; {}",
                            used.method_words,
                            estimate_step(name, &used.attribute)
                        ));
                        f.notes.insert(used.note.clone());
                    }
                }
                f
            })
            .collect();
        Ok(ShipResult {
            ship: ship.to_string(),
            ship_name: ship_info.name.clone(),
            class_id: class.row_id.parse().unwrap_or(0),
            class_name: class.name.clone(),
            figures,
            radar_assumed,
            radar_not_counted,
            ammunition_assumed,
            designed_rate,
            weapons_not_modelled: surface_weapons_not_modelled,
            unarmed_by_kind: unarmed,
            tubes_not_split,
            used: w.used,
            anti_aircraft_assumed,
            estimates: w.estimated,
        })
    }

    /// One fact about how a class's tubes bear, as text: `Ok(None)` where no
    /// row is readable (none at all, none preferred or excepted, none that
    /// holds on the date), a failure where rows that are readable disagree
    /// or cannot be placed on the date, as for every other fact.
    fn split_row(
        &self,
        w: &mut Work,
        keys: (&str, &str),
        attribute: &str,
    ) -> Result<Option<Got<String>>, ()> {
        let (ship, class) = keys;
        let read = resolve_over(
            self.index.rows(Table::ShipFacts, ship, attribute),
            &format!("{attribute} of ship {ship}"),
            self.index.rows(Table::ClassFacts, class, attribute),
            &format!("{attribute} of class {class}"),
            self.date,
            &self.rules.exceptions,
        );
        match read {
            Ok(read) => {
                let value = read.rows[0].value.clone();
                let (cites, excepted, doubts) = w.take(&read);
                Ok(Some(Got {
                    value,
                    cites,
                    excepted,
                    doubts,
                }))
            }
            Err(failure)
                if matches!(
                    failure.kind,
                    Kind::Missing | Kind::NotPreferred | Kind::NotOnDate
                ) =>
            {
                Ok(None)
            }
            Err(failure) => {
                w.failed(FIGURES[11], failure);
                Err(())
            }
        }
    }

    /// The tubes that bear on one side (decisions 0074 and 0083), and
    /// whether the class is not split. With T tubes: a readable
    /// `torpedo_tubes_per_broadside` row gives the number; else a readable
    /// `torpedo_mount_position` row gives T for a centreline fit and T / 2
    /// for a wing fit; else T, and the class is named as not split. A class
    /// with no tubes has 0 and reads nothing more.
    fn tubes_one_side(
        &self,
        w: &mut Work,
        keys: (&str, &str),
        tubes: &Got<Fit>,
    ) -> Option<(Got<u32>, bool)> {
        let figure = FIGURES[11];
        let total = tubes.value.count;
        if total == 0 {
            return Some((tubes.map_value(0), false));
        }
        let (ship, class) = keys;
        let who = format!("{ship} (class {class})");
        let number = self.split_row(w, keys, PER_BROADSIDE).ok()?;
        let position = self.split_row(w, keys, MOUNT_POSITION).ok()?;
        let mut cites = tubes.cites.clone();
        let mut excepted = tubes.excepted;
        let mut doubts = tubes.doubts.clone();
        let mut rows = Vec::new();
        for got in [&number, &position].into_iter().flatten() {
            cites.extend(got.cites.iter().cloned());
            excepted |= got.excepted;
            doubts.extend(got.doubts.iter().cloned());
            rows.extend(got.cites.iter().map(|(f, id)| format!("{f}:{id}")));
        }
        let rows = rows.join(", ");
        let mut refuse = |kind: Kind, message: String| -> Option<(Got<u32>, bool)> {
            w.fail(kind, figure, message);
            None
        };
        let stated = match &number {
            Some(got) => match got.value.trim().parse::<u32>() {
                Ok(n) => Some(n),
                Err(_) => {
                    return refuse(
                        Kind::BadValue,
                        format!(
                            "{PER_BROADSIDE} of {who}: row {rows} has value `{}`, which is not a whole number of tubes",
                            got.value
                        ),
                    )
                }
            },
            None => None,
        };
        if let Some(n) = stated {
            if n > total {
                return refuse(
                    Kind::BadValue,
                    format!(
                        "{PER_BROADSIDE} of {who}: {n} tubes on one side is above the {total} tubes of the fit (rows {rows})"
                    ),
                );
            }
            if n == 0 {
                return refuse(
                    Kind::BadValue,
                    format!(
                        "{PER_BROADSIDE} of {who}: 0 tubes on one side where the fit has {total} (rows {rows})"
                    ),
                );
            }
        }
        let place = position.as_ref().map(|got| got.value.trim().to_string());
        if let Some(place) = &place {
            match place.as_str() {
                "centreline" | "wing" | "mixed" | "fixed" => {}
                other => {
                    return refuse(
                        Kind::BadValue,
                        format!(
                            "{MOUNT_POSITION} of {who}: row {rows} has value `{other}`, which is not centreline, wing, mixed or fixed"
                        ),
                    )
                }
            }
            if place == "wing" && !total.is_multiple_of(2) {
                return refuse(
                    Kind::BadValue,
                    format!(
                        "{MOUNT_POSITION} of {who}: wing mounts with {total} tubes, an odd number, cannot be halved (rows {rows})"
                    ),
                );
            }
        }
        let value = match (stated, place.as_deref()) {
            (Some(n), Some("centreline")) if n < total => {
                return refuse(
                    Kind::Conflict,
                    format!(
                        "{who}: rows {rows} disagree: centreline mounts bear all {total} tubes on either side, and the number of tubes on one side is {n}"
                    ),
                )
            }
            (Some(n), Some("wing")) if n != total / 2 => {
                return refuse(
                    Kind::Conflict,
                    format!(
                        "{who}: rows {rows} disagree: wing mounts give {} tubes on one side, and the number of tubes on one side is {n}",
                        total / 2
                    ),
                )
            }
            (Some(n), _) => n,
            (None, Some("centreline")) => total,
            (None, Some("wing")) => total / 2,
            (None, Some(other)) => {
                return refuse(
                    Kind::BadValue,
                    format!(
                        "{MOUNT_POSITION} of {who}: a `{other}` fit with no readable {PER_BROADSIDE} row: the position alone does not give the tubes on one side (rows {rows})"
                    ),
                )
            }
            (None, None) => {
                return Some((
                    Got {
                        value: total,
                        cites,
                        excepted,
                        doubts,
                    },
                    true,
                ))
            }
        };
        Some((
            Got {
                value,
                cites,
                excepted,
                doubts,
            },
            false,
        ))
    }

    /// Whether the display of a surface-search set has a plan position
    /// indicator (decision 0077, 1 and 4): its `display` row is read as
    /// every fact is, with exceptions. No readable row is not a failure:
    /// the set is not counted and is named (decision 0077, 3). Rows that
    /// disagree are.
    fn plan_position(&self, item: &str, fit: &str) -> Result<Display, Failure> {
        let rows = self.index.rows(Table::SensorFacts, item, DISPLAY);
        match resolve(
            rows,
            &format!("{DISPLAY} of sensor {item}"),
            self.date,
            &self.rules.exceptions,
        ) {
            Ok(read) => Ok(if read.rows[0].value.contains(PLAN_POSITION) {
                Display::Plan(read)
            } else {
                Display::NoPlan(read)
            }),
            Err(failure)
                if matches!(
                    failure.kind,
                    Kind::Missing | Kind::NotPreferred | Kind::NotOnDate
                ) =>
            {
                Ok(Display::NotGiven(format!("{item} ({fit})")))
            }
            Err(failure) => Err(failure),
        }
    }

    fn slot_read(
        &self,
        keys: (&str, &str),
        slot: &str,
        set_aside_blank: bool,
    ) -> Result<SlotRead, Failure> {
        let (ship, class) = keys;
        resolve_slot(
            self.index.rows(Table::ShipFits, ship, slot),
            &format!("{slot} fit of ship {ship}"),
            self.index.rows(Table::ClassFits, class, slot),
            &format!("{slot} fit of class {class}"),
            self.date,
            &self.rules.exceptions,
            set_aside_blank,
        )
    }

    /// Aircraft capacity (decision 0029, 2): the sourced fact; no row at
    /// all means none carried, except for a carrier, where it is missing.
    fn aircraft_capacity(
        &self,
        w: &mut Work,
        keys: (&str, &str),
        class: &ClassInfo,
        figs: &mut [Option<Figure>],
    ) {
        let (ship, class_key) = keys;
        let name = AIR_FIGURES[0];
        let slot = FIGURES.len();
        let no_rows = self.index.rows(Table::ShipFacts, ship, name).is_empty()
            && self
                .index
                .rows(Table::ClassFacts, class_key, name)
                .is_empty();
        if no_rows {
            if KIND_FOR_TYPE
                .iter()
                .any(|(t, kind)| *t == class.kind_type && *kind == "carrier")
            {
                w.fail(
                    Kind::Missing,
                    name,
                    format!(
                        "{name} of ship {ship} and of class {class_key}: no rows in the facts, and a carrier must have one"
                    ),
                );
                return;
            }
            put(
                figs,
                slot,
                Value::Num(Dec::ZERO),
                Got {
                    value: (),
                    cites: BTreeSet::new(),
                    excepted: false,
                    doubts: BTreeSet::new(),
                },
            );
            if let Some(figure) = figs[slot].as_mut() {
                figure
                    .notes
                    .insert("no aircraft_capacity row in the facts: none carried".to_string());
            }
            return;
        }
        let Some(got) = self.fact(w, keys, name, name) else {
            return;
        };
        if got.value.has_fraction() {
            w.fail(
                Kind::BadValue,
                name,
                format!("{name} of {ship}: {} is not a whole number", got.value),
            );
            return;
        }
        put(figs, slot, Value::Num(got.value), got.bare());
    }

    /// Anti-aircraft shots (decision 0029, 1): heavy guns x heavy weight +
    /// light barrels x light weight, rounded down. Each slot holds several
    /// weapons, added; a slot with no rows is 0.
    /// Returns whether the list's assumption that unknown anti-aircraft guns
    /// mean none was used: it is, for a ship it names, where no row of either
    /// slot exists for her or her class.
    fn anti_aircraft(
        &self,
        w: &mut Work,
        keys: (&str, &str),
        figs: &mut [Option<Figure>],
        assume_none: bool,
    ) -> bool {
        let unknown = AA_SLOTS.iter().all(|slot| {
            self.index.rows(Table::ShipFits, keys.0, slot).is_empty()
                && self.index.rows(Table::ClassFits, keys.1, slot).is_empty()
        });
        let assumed = assume_none && unknown;
        let name = AIR_FIGURES[1];
        let slot_index = FIGURES.len() + 1;
        let mut sums = [0i64; 2];
        let mut sources = Got {
            value: (),
            cites: BTreeSet::new(),
            excepted: false,
            doubts: BTreeSet::new(),
        };
        let mut notes = BTreeSet::new();
        let mut ok = true;
        for (n, slot) in AA_SLOTS.iter().enumerate() {
            // Machine guns are not counted and their rows are not read;
            // any other kind than aa_gun is a problem.
            let (ship, class) = keys;
            let mut passed = BTreeSet::new();
            let mut slot_rows: [Vec<Row>; 2] = [Vec::new(), Vec::new()];
            for (k, (table, entity)) in [(Table::ShipFits, ship), (Table::ClassFits, class)]
                .into_iter()
                .enumerate()
            {
                for row in self.index.rows(table, entity, slot) {
                    match self.weapons.get(&row.item).map(String::as_str) {
                        Some("aa_gun") => slot_rows[k].push(row.clone()),
                        Some("machine_gun") => {
                            passed.insert(row.cite_key());
                        }
                        // A weapon of another kind in a row that does not
                        // hold on the list's date is not read and is no
                        // problem (decision 0206: Neosho's 3-inch/23 of
                        // 1941).
                        _ if matches!(valid_on(&row.from, &row.to, self.date), Ok(Tri::No)) => {}
                        other => {
                            w.fail(
                                Kind::UnknownType,
                                name,
                                match other {
                                    None => format!(
                                        "{slot} fit row {}: weapon `{}` is not in weapons.csv",
                                        row.cite(),
                                        row.item
                                    ),
                                    Some(kind) => format!(
                                        "{slot} fit row {}: weapon `{}` has kind `{kind}`, not aa_gun or machine_gun",
                                        row.cite(),
                                        row.item
                                    ),
                                },
                            );
                            ok = false;
                        }
                    }
                }
            }
            if !passed.is_empty() {
                notes.insert(format!(
                    "machine guns not counted: {}",
                    passed
                        .iter()
                        .map(|(f, i)| format!("{f}:{i}"))
                        .collect::<Vec<_>>()
                        .join(" ")
                ));
            }
            let slot_read = match resolve_slot(
                &slot_rows[0],
                &format!("{slot} fit of ship {ship}"),
                &slot_rows[1],
                &format!("{slot} fit of class {class}"),
                self.date,
                &self.rules.exceptions,
                true,
            ) {
                Ok(slot_read) => slot_read,
                Err(failure) => {
                    w.failed(name, failure);
                    ok = false;
                    continue;
                }
            };
            notes.extend(slot_read.blank);
            if !slot_read.items.is_empty() {
                sources.doubts.extend(slot_read.doubts);
            }
            for (_, read) in slot_read.items {
                let counted = read.and_then(|read| count(&read, slot).map(|c| (read, c)));
                match counted {
                    Ok((read, c)) => {
                        sums[n] += i64::from(c);
                        let (cites, excepted, doubts) = w.take(&read);
                        sources.cites.extend(cites);
                        sources.excepted |= excepted;
                        sources.doubts.extend(doubts);
                    }
                    Err(failure) => {
                        w.failed(name, failure);
                        ok = false;
                    }
                }
            }
        }
        let (Some(heavy_weight), Some(light_weight)) = (
            self.rules.anti_aircraft_heavy_weight,
            self.rules.anti_aircraft_light_weight,
        ) else {
            return assumed;
        };
        if !ok {
            return assumed;
        }
        let total = Dec::from_int(sums[0])
            .and_then(|h| h.mul(heavy_weight))
            .zip(Dec::from_int(sums[1]).and_then(|l| l.mul(light_weight)))
            .and_then(|(h, l)| h.checked_add(l));
        let Some(total) = total else {
            w.failed(name, overflow("anti-aircraft shots"));
            return assumed;
        };
        let Some(whole) = Dec::from_int(total.whole()) else {
            w.failed(name, overflow("anti-aircraft shots"));
            return assumed;
        };
        notes.insert(format!("{} heavy, {} light", sums[0], sums[1]));
        if total.has_fraction() {
            notes.insert(format!("exact {total}, rounded down"));
        }
        put(figs, slot_index, Value::Num(whole), sources);
        if assumed {
            notes.insert(AA_ASSUMED_NOTE.to_string());
        }
        if let Some(figure) = figs[slot_index].as_mut() {
            figure.notes = notes;
        }
        assumed
    }

    /// Radar (decision 0020 as changed by decision 0029, 3 and decision
    /// 0077): the figure and whether it is the list's assumption, and the
    /// sets not counted for want of a display row. The figure is yes only if
    /// a readable surface-search fit has count above 0 and the set it names
    /// has a readable `display` row containing "PPI". An air-search fit does
    /// not give the figure, but is read to know that a radar fit is on
    /// record. A sourced "none" (a no-radar stand-in item at count 0) beside
    /// a fitted set is a conflict.
    #[allow(clippy::type_complexity)]
    fn radar(
        &self,
        w: &mut Work,
        keys: (&str, &str),
        assume_no_radar: bool,
    ) -> Option<((bool, bool), Got<()>, Vec<String>)> {
        let mut reads: Vec<Read> = Vec::new();
        let mut fitted = false;
        let mut counted = false;
        let mut not_counted: Vec<String> = Vec::new();
        let mut display_reads: Vec<Read> = Vec::new();
        let mut stand_in = false;
        let mut slots_found = 0;
        let mut absent: Vec<String> = Vec::new();
        let mut doubts: Vec<String> = Vec::new();
        let mut hard = false;
        let mut fitted_items: Vec<String> = Vec::new();
        let mut none_items: Vec<String> = Vec::new();
        for slot in RADAR_SLOTS {
            let slot_read = match self.slot_read(keys, slot, false) {
                Ok(slot_read) => slot_read,
                Err(failure) => {
                    if slot == SURFACE_SLOT {
                        w.failed(FIGURES[RADAR], failure);
                        hard = true;
                    } else {
                        absent.push(failure.message);
                    }
                    continue;
                }
            };
            let slot_doubts = slot_read.doubts;
            let mut found = false;
            let had_items = !slot_read.items.is_empty();
            for (item, read) in slot_read.items {
                match read {
                    Ok(read) => {
                        let text = read.rows[0].count.trim().to_string();
                        let n = if text.is_empty() {
                            None
                        } else {
                            match count(&read, slot) {
                                Ok(n) => Some(n),
                                Err(failure) if slot != SURFACE_SLOT => {
                                    absent.push(failure.message);
                                    continue;
                                }
                                Err(failure) => {
                                    w.failed(FIGURES[RADAR], failure);
                                    hard = true;
                                    continue;
                                }
                            }
                        };
                        let none_item = NO_RADAR_ITEMS.contains(&item.as_str());
                        match n {
                            Some(0) if none_item => {
                                stand_in = true;
                                none_items.push(read.rows[0].cite());
                            }
                            None if none_item => continue,
                            Some(0) => {}
                            _ => {
                                fitted = true;
                                fitted_items.push(format!("{item} ({})", read.rows[0].cite()));
                                if slot == SURFACE_SLOT {
                                    match self.plan_position(&item, &read.rows[0].cite()) {
                                        Ok(Display::Plan(display)) => {
                                            counted = true;
                                            display_reads.push(display);
                                        }
                                        Ok(Display::NoPlan(display)) => display_reads.push(display),
                                        Ok(Display::NotGiven(set)) => not_counted.push(set),
                                        Err(failure) => {
                                            w.failed(FIGURES[RADAR], failure);
                                            hard = true;
                                        }
                                    }
                                }
                            }
                        }
                        found = true;
                        reads.push(read);
                    }
                    Err(failure)
                        if matches!(
                            failure.kind,
                            Kind::Missing | Kind::NotPreferred | Kind::NotOnDate
                        ) =>
                    {
                        absent.push(failure.message);
                    }
                    Err(failure) if slot != SURFACE_SLOT => absent.push(failure.message),
                    Err(failure) => {
                        w.failed(FIGURES[RADAR], failure);
                        hard = true;
                    }
                }
            }
            if found {
                slots_found += 1;
                doubts.extend(slot_doubts);
            } else if had_items {
                // The failures of the items were noted above.
            } else if self.index.rows(Table::ShipFits, keys.0, slot).is_empty()
                && self.index.rows(Table::ClassFits, keys.1, slot).is_empty()
            {
                absent.push(format!(
                    "{slot} fit of class {}: no rows in the facts",
                    keys.1
                ));
            } else {
                absent.push(format!("{slot}: no row holds on {}", self.date));
            }
        }
        if hard {
            return None;
        }
        let both_empty = slots_found == RADAR_SLOTS.len() && !fitted;
        let mut sources = Got {
            value: (),
            cites: BTreeSet::new(),
            excepted: false,
            doubts: doubts.into_iter().collect(),
        };
        for read in reads.iter().chain(&display_reads) {
            let (c, e, d) = w.take(read);
            sources.cites.extend(c);
            sources.excepted |= e;
            sources.doubts.extend(d);
        }
        if fitted && stand_in {
            w.fail(
                Kind::Conflict,
                FIGURES[RADAR],
                format!(
                    "radar on {}: a fitted set ({}) and a sourced none ({}) are both read",
                    self.date,
                    fitted_items.join(", "),
                    none_items.join(", ")
                ),
            );
            return None;
        }
        if fitted {
            return Some(((counted, false), sources, not_counted));
        }
        if stand_in || both_empty {
            return Some(((false, false), sources, not_counted));
        }
        if assume_no_radar {
            return Some(((false, true), sources, not_counted));
        }
        let mut found_text: Vec<String> = reads
            .iter()
            .map(|read| {
                format!(
                    "{}: {} says {} x {} (a specific set absent, not all radar)",
                    read.rows[0].name,
                    read.rows
                        .iter()
                        .map(Row::cite)
                        .collect::<Vec<_>>()
                        .join(" "),
                    read.rows[0].item,
                    read.rows[0].count
                )
            })
            .collect();
        found_text.extend(absent);
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
