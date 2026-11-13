//! The figures of one ship, from the facts, as of a date (decision 0020).

use std::collections::{BTreeMap, BTreeSet};

use oob_check::table::Data;

use crate::date::Day;
use crate::decimal::Dec;
use crate::input::Rules;
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
    /// The ship has tubes and no readable row says how they bear, so the
    /// tubes on one side are all of them (decision 0074, 3).
    pub tubes_not_split: bool,
    /// Exceptions used: index into the rules' list, and the row read.
    pub used: Vec<(usize, Row)>,
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
    pub(crate) date: Day,
    pub(crate) rules: &'a Rules,
    /// The achieved rates of fire, when the rules have a `[fire_rate]`
    /// block and every case in it is sound (decision 0070).
    pub(crate) fire_rate: Option<crate::fire_rate::Achieved>,
    /// Problems with that block's cases, read against the facts.
    pub fire_rate_problems: Vec<Problem>,
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
        let mut env = Env {
            index: Index::build(data),
            ships,
            classes,
            aircraft,
            weapons,
            date,
            rules,
            fire_rate: None,
            fire_rate_problems: Vec::new(),
        };
        if let Some(fire_rate) = &rules.fire_rate {
            let (achieved, problems) = crate::fire_rate::compute(&env, data, fire_rate);
            env.fire_rate = achieved;
            env.fire_rate_problems = problems;
        }
        env
    }

    /// Problems for exceptions that name no row of the facts.
    pub fn exception_problems(&self) -> Vec<Problem> {
        let search = self
            .rules
            .search_radius
            .as_ref()
            .map_or(&[][..], |s| s.exceptions.as_slice());
        self.rules
            .exceptions
            .iter()
            .chain(search)
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

    /// Derives every figure of a ship. All problems are returned, not the
    /// first. `assume_no_radar` is the list's working assumption.
    pub fn derive_ship(&self, ship: &str, opts: ShipOpts) -> Result<ShipResult, Vec<Problem>> {
        let ShipOpts {
            assume_no_radar,
            assume_ammunition_not_limiting,
            air_figures,
            surface_weapons_not_modelled,
        } = opts;
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
        let figure_count = FIGURES.len() + if air_figures { AIR_FIGURES.len() } else { 0 };
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

        // main battery, ammunition and torpedoes
        let mut designed_rate = false;
        let mut ammunition_assumed = false;
        let mut tubes_not_split = false;
        if surface_weapons_not_modelled {
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

        // toughness
        if let Some(got) = self.fact(&mut w, keys, "displacement_standard_lt", FIGURES[13]) {
            match got.value.div(self.rules.displacement_lt_per_toughness) {
                Some(toughness) if !toughness.is_positive() => w.fail(
                    Kind::ZeroToughness,
                    FIGURES[13],
                    format!(
                        "displacement_standard_lt {} / {} = {toughness}, which is not above zero",
                        got.value, self.rules.displacement_lt_per_toughness
                    ),
                ),
                Some(toughness) => put(&mut figs, 13, Value::Num(toughness), got.bare()),
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
        if air_figures {
            self.aircraft_capacity(&mut w, keys, class, &mut figs);
            self.anti_aircraft(&mut w, keys, &mut figs);
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
            ship_name: ship_info.name.clone(),
            class_id: class.row_id.parse().unwrap_or(0),
            class_name: class.name.clone(),
            figures,
            radar_assumed,
            radar_not_counted,
            ammunition_assumed,
            designed_rate,
            weapons_not_modelled: surface_weapons_not_modelled,
            tubes_not_split,
            used: w.used,
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
    fn anti_aircraft(&self, w: &mut Work, keys: (&str, &str), figs: &mut [Option<Figure>]) {
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
            return;
        };
        if !ok {
            return;
        }
        let total = Dec::from_int(sums[0])
            .and_then(|h| h.mul(heavy_weight))
            .zip(Dec::from_int(sums[1]).and_then(|l| l.mul(light_weight)))
            .and_then(|(h, l)| h.checked_add(l));
        let Some(total) = total else {
            w.failed(name, overflow("anti-aircraft shots"));
            return;
        };
        let Some(whole) = Dec::from_int(total.whole()) else {
            w.failed(name, overflow("anti-aircraft shots"));
            return;
        };
        notes.insert(format!("{} heavy, {} light", sums[0], sums[1]));
        if total.has_fraction() {
            notes.insert(format!("exact {total}, rounded down"));
        }
        put(figs, slot_index, Value::Num(whole), sources);
        if let Some(figure) = figs[slot_index].as_mut() {
            figure.notes = notes;
        }
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
