//! Reading a fact: which rows count on a date, and what they say.
//!
//! The rules are in decision 0020 and `docs/research/oob/database-design.md`
//! ("Dates", "Fits", "Conflicting sources"):
//!
//! - only `preferred` rows are read, and any row an exception names;
//! - a row counts only if it holds on the date;
//! - rows that count must agree, and then all of them are cited;
//! - a ship's rows for a slot or attribute replace the class's rows only
//!   when one of them is readable (preferred or excepted) and holds on the
//!   date. Ship rows that hold but are not readable do not block the class
//!   rows; they are passed back as doubts, for the sources file.

use std::collections::BTreeMap;

use oob_check::table::Data;

use crate::date::{valid_on, Day, Tri};
use crate::input::Exception;
use crate::problem::Kind;

/// Which kind of row, and about what kind of entity.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum Table {
    ShipFacts,
    ClassFacts,
    WeaponFacts,
    ShipFits,
    ClassFits,
    AircraftFacts,
    AircraftFits,
    SensorFacts,
}

/// One row of a facts file or of `fits.csv`, reduced to what reading needs.
#[derive(Clone, Debug)]
pub struct Row {
    pub file: String,
    pub id: String,
    /// The entity key, or for a fit the carrier key.
    pub entity: String,
    /// The attribute, or for a fit the slot.
    pub name: String,
    /// What the row says: the value, or for a fit `item x count`.
    pub value: String,
    pub item: String,
    pub count: String,
    pub from: String,
    pub to: String,
    pub standing: String,
}

impl Row {
    pub fn cite(&self) -> String {
        format!("{}:{}", self.file, self.id)
    }

    pub fn cite_key(&self) -> (String, u64) {
        (self.file.clone(), self.id.parse().unwrap_or(0))
    }
}

fn shown(text: &str) -> &str {
    if text.is_empty() {
        "-"
    } else {
        text
    }
}

fn describe(row: &Row, tri: Tri) -> String {
    format!(
        "{} ({}; {}; valid {}..{}; {})",
        row.cite(),
        row.standing,
        row.value,
        shown(&row.from),
        shown(&row.to),
        tri.word()
    )
}

fn list(items: impl Iterator<Item = String>) -> String {
    items.collect::<Vec<_>>().join(", ")
}

/// Every row the derivation can read, by table, entity and attribute or slot.
#[derive(Default)]
pub struct Index {
    rows: BTreeMap<(Table, String, String), Vec<Row>>,
    /// Entity, attribute or slot, and id of every row of every facts file
    /// and of fits.csv, to check that an exception names a row that exists.
    all: Vec<(String, String, String)>,
}

impl Index {
    pub fn build(data: &Data) -> Index {
        let mut index = Index::default();
        for (file, table) in [
            ("ship_facts.csv", Table::ShipFacts),
            ("ship_class_facts.csv", Table::ClassFacts),
            ("weapon_facts.csv", Table::WeaponFacts),
            ("aircraft_type_facts.csv", Table::AircraftFacts),
            ("sensor_facts.csv", Table::SensorFacts),
        ] {
            for rec in data.rows(file) {
                let row = Row {
                    file: file.to_string(),
                    id: rec.get("id").to_string(),
                    entity: rec.get("entity").to_string(),
                    name: rec.get("attribute").to_string(),
                    value: rec.get("value").to_string(),
                    item: String::new(),
                    count: String::new(),
                    from: rec.get("valid_from").to_string(),
                    to: rec.get("valid_to").to_string(),
                    standing: rec.get("standing").to_string(),
                };
                index.add(table, row);
            }
        }
        for rec in data.rows("fits.csv") {
            let table = match rec.get("carrier_type") {
                "ship" => Some(Table::ShipFits),
                "ship_class" => Some(Table::ClassFits),
                "aircraft_type" => Some(Table::AircraftFits),
                _ => None,
            };
            let (item, count) = (rec.get("item"), rec.get("count"));
            let row = Row {
                file: "fits.csv".to_string(),
                id: rec.get("id").to_string(),
                entity: rec.get("carrier").to_string(),
                name: rec.get("slot").to_string(),
                value: format!("{item} x {count}"),
                item: item.to_string(),
                count: count.to_string(),
                from: rec.get("valid_from").to_string(),
                to: rec.get("valid_to").to_string(),
                standing: rec.get("standing").to_string(),
            };
            index
                .all
                .push((row.entity.clone(), row.name.clone(), row.id.clone()));
            if let Some(table) = table {
                index.add(table, row);
            }
        }
        for file in data.tables.keys().filter(|f| f.ends_with("_facts.csv")) {
            for rec in data.rows(file) {
                index.all.push((
                    rec.get("entity").to_string(),
                    rec.get("attribute").to_string(),
                    rec.get("id").to_string(),
                ));
            }
        }
        index
    }

    fn add(&mut self, table: Table, row: Row) {
        self.rows
            .entry((table, row.entity.clone(), row.name.clone()))
            .or_default()
            .push(row);
    }

    pub fn rows(&self, table: Table, entity: &str, name: &str) -> &[Row] {
        self.rows
            .get(&(table, entity.to_string(), name.to_string()))
            .map_or(&[], Vec::as_slice)
    }

    /// Every row of a table about one entity, whatever attribute.
    pub fn rows_of_entity<'a>(
        &'a self,
        table: Table,
        entity: &'a str,
    ) -> impl Iterator<Item = &'a Row> + 'a {
        self.rows
            .iter()
            .filter(move |((t, e, _), _)| *t == table && e == entity)
            .flat_map(|(_, rows)| rows)
    }

    /// The row of `aircraft_type_facts.csv` with this id, whatever it says.
    pub fn aircraft_fact(&self, id: i64) -> Option<&Row> {
        let id = id.to_string();
        self.rows
            .iter()
            .filter(|((table, _, _), _)| *table == Table::AircraftFacts)
            .flat_map(|(_, rows)| rows)
            .find(|row| row.id == id)
    }

    /// Whether some row of any facts file or fits.csv is the one named.
    pub fn has_row(&self, exception: &Exception) -> bool {
        let id = exception.row.to_string();
        self.all.iter().any(|(entity, name, row)| {
            *entity == exception.entity && *name == exception.fact && *row == id
        })
    }
}

/// Why nothing could be read.
#[derive(Clone, Debug)]
pub struct Failure {
    pub kind: Kind,
    pub message: String,
}

impl Failure {
    fn new(kind: Kind, message: String) -> Failure {
        Failure { kind, message }
    }
}

/// The rows that were read: all valid on the date, all saying the same.
#[derive(Clone, Debug)]
pub struct Read {
    pub rows: Vec<Row>,
    /// Rows read only because an exception names them: the index of the
    /// exception and the row.
    pub excepted: Vec<(usize, Row)>,
    /// Ship rows that hold on the date (or may) but are not readable, left
    /// unused because the class's rows were read instead: what they were
    /// worth noting for. `file:id (standing)`, plus `(may hold)` when the
    /// date cannot be told.
    pub doubts: Vec<String>,
}

fn bad_date(row: &Row, text: &str) -> Failure {
    Failure::new(
        Kind::BadValue,
        format!(
            "row {} has `{text}` in valid_from or valid_to, which is not a date",
            row.cite()
        ),
    )
}

struct Seen<'a> {
    row: &'a Row,
    tri: Tri,
    excepted: Option<usize>,
}

impl Seen<'_> {
    /// An exception names a row whose date is only known to the year or
    /// month: it says the architect judged that the row holds on the date.
    fn judged(&self) -> bool {
        self.excepted.is_some() && self.tri == Tri::CannotTell
    }
}

impl Seen<'_> {
    fn read(&self) -> bool {
        self.row.standing == "preferred" || self.excepted.is_some()
    }
}

/// Reads one group of rows (one entity, one attribute or slot) on a date.
/// `what` names it in messages, with the entity.
pub fn resolve(
    rows: &[Row],
    what: &str,
    date: Day,
    exceptions: &[Exception],
) -> Result<Read, Failure> {
    let mut seen = Vec::new();
    for row in rows {
        let tri = valid_on(&row.from, &row.to, date).map_err(|text| bad_date(row, &text))?;
        let excepted = exceptions.iter().position(|e| e.matches(row));
        seen.push(Seen { row, tri, excepted });
    }
    let unclear: Vec<&Seen> = seen
        .iter()
        .filter(|s| s.read() && s.tri == Tri::CannotTell && !s.judged())
        .collect();
    if !unclear.is_empty() {
        return Err(Failure::new(
            Kind::AmbiguousDate,
            format!(
                "{what}: cannot tell whether a row to be read holds on {date}: {}",
                list(unclear.iter().map(|s| describe(s.row, s.tri)))
            ),
        ));
    }
    let mut read: Vec<&Seen> = seen
        .iter()
        .filter(|s| s.read() && (s.tri == Tri::Yes || s.judged()))
        .collect();
    if read.is_empty() {
        if seen.is_empty() {
            return Err(Failure::new(
                Kind::Missing,
                format!("{what}: no rows in the facts"),
            ));
        }
        let all = list(seen.iter().map(|s| describe(s.row, s.tri)));
        if seen.iter().any(|s| s.tri == Tri::Yes) {
            return Err(Failure::new(
                Kind::NotPreferred,
                format!(
                    "{what}: rows that hold on {date} are not preferred, and no exception names one: {all}"
                ),
            ));
        }
        return Err(Failure::new(
            Kind::NotOnDate,
            format!("{what}: no row holds on {date}: {all}"),
        ));
    }
    let first = &read[0].row.value;
    let mut settled = false;
    if read.iter().any(|s| s.row.value != *first) {
        // Rows that disagree: a row an exception names wins over the
        // others, if the named rows agree with each other.
        let named: Vec<&Seen> = read
            .iter()
            .filter(|s| s.excepted.is_some())
            .copied()
            .collect();
        if named.is_empty() || named.iter().any(|s| s.row.value != named[0].row.value) {
            return Err(Failure::new(
                Kind::Conflict,
                format!(
                    "{what}: rows read on {date} disagree: {}",
                    list(read.iter().map(|s| describe(s.row, s.tri)))
                ),
            ));
        }
        read = named;
        settled = true;
    }
    Ok(Read {
        rows: read.iter().map(|s| s.row.clone()).collect(),
        excepted: read
            .iter()
            .filter(|s| s.row.standing != "preferred" || s.judged() || settled)
            .filter_map(|s| s.excepted.map(|i| (i, s.row.clone())))
            .collect(),
        doubts: Vec::new(),
    })
}

/// Which side of a ship-over-class reading applies.
struct Side {
    /// A ship row is readable and holds on the date, so the class rows do
    /// not apply.
    ship_holds: bool,
    /// Ship rows that hold (or may) but are not readable.
    doubts: Vec<String>,
}

/// Looks at a ship's rows for one attribute or slot: do any of them
/// replace the class's rows? An exception naming a row that may hold, by a
/// date known only to the year or month, makes it hold. A readable row
/// that cannot be placed relative to the date, with none that holds, is an
/// `ambiguous-date` failure: it is not known whether the class applies.
fn ship_side(
    ship_rows: &[Row],
    ship_what: &str,
    date: Day,
    exceptions: &[Exception],
) -> Result<Side, Failure> {
    let mut holds = false;
    let mut unclear = Vec::new();
    let mut doubts = Vec::new();
    for row in ship_rows {
        let tri = valid_on(&row.from, &row.to, date).map_err(|text| bad_date(row, &text))?;
        let named = exceptions.iter().any(|e| e.matches(row));
        let readable = row.standing == "preferred" || named;
        match (tri, readable) {
            (Tri::No, _) => {}
            (Tri::Yes, true) => holds = true,
            (Tri::CannotTell, true) if named => holds = true,
            (Tri::CannotTell, true) => unclear.push(row),
            (Tri::Yes, false) => doubts.push(format!("{} ({})", row.cite(), row.standing)),
            (Tri::CannotTell, false) => {
                doubts.push(format!("{} ({}, may hold)", row.cite(), row.standing))
            }
        }
    }
    if !holds && !unclear.is_empty() {
        return Err(Failure::new(
            Kind::AmbiguousDate,
            format!(
                "{ship_what}: cannot tell whether a ship row holds on {date}, so cannot tell whether the class rows apply: {}",
                list(unclear.iter().map(|r| describe(r, Tri::CannotTell)))
            ),
        ));
    }
    Ok(Side {
        ship_holds: holds,
        doubts,
    })
}

/// A ship's rows over its class's: if a ship row is readable (preferred, or
/// named by an exception) and holds on the date, the class rows do not
/// apply. A readable ship row that cannot be placed relative to the date,
/// with none that holds, leaves it unknown whether the class applies. Ship
/// rows that hold (or may) but are not readable never replace the class
/// rows; when the class rows are read they are returned as doubts, and when
/// reading the class rows fails the failure names them.
pub fn resolve_over(
    ship_rows: &[Row],
    ship_what: &str,
    class_rows: &[Row],
    class_what: &str,
    date: Day,
    exceptions: &[Exception],
) -> Result<Read, Failure> {
    let side = ship_side(ship_rows, ship_what, date, exceptions)?;
    if side.ship_holds {
        return resolve(ship_rows, ship_what, date, exceptions);
    }
    let doubts = side.doubts;
    match resolve(class_rows, class_what, date, exceptions) {
        Ok(mut read) => {
            read.doubts = doubts;
            Ok(read)
        }
        Err(mut failure) => {
            if !doubts.is_empty() {
                failure.message = format!(
                    "{}; ship rows not read: {}",
                    failure.message,
                    doubts.join(", ")
                );
            }
            Err(failure)
        }
    }
}

/// What a slot that holds several items says (decision 0029, 1 and 3).
#[derive(Debug)]
pub struct SlotRead {
    /// Each item that holds on the date, in item order, with what reading
    /// its rows gave. An item none of whose rows holds on the date is not
    /// listed: it was not fitted.
    pub items: Vec<(String, Result<Read, Failure>)>,
    /// Ship rows that hold but are not readable, when the class's rows
    /// were used.
    pub doubts: Vec<String>,
    /// Rows with no count that hold or may hold on the date, set aside.
    pub blank: Vec<String>,
}

/// Reads a slot that holds several items, ship over class by the whole
/// slot: if the ship has any readable row in the slot that holds on the
/// date, the ship's rows are the slot; otherwise the class's. Each item's
/// rows are then read as one fact. With `set_aside_blank`, a row with a
/// blank count is not a count: it is neither readable nor able to replace
/// the class's rows, and is listed in `blank`.
pub fn resolve_slot(
    ship_rows: &[Row],
    ship_what: &str,
    class_rows: &[Row],
    class_what: &str,
    date: Day,
    exceptions: &[Exception],
    set_aside_blank: bool,
) -> Result<SlotRead, Failure> {
    let usable = |row: &Row| !(set_aside_blank && row.count.trim().is_empty());
    let mut blank = Vec::new();
    for row in ship_rows.iter().chain(class_rows) {
        if !usable(row) {
            let tri = valid_on(&row.from, &row.to, date).map_err(|text| bad_date(row, &text))?;
            if tri != Tri::No {
                blank.push(format!("{} count not given", row.cite()));
            }
        }
    }
    let ship_rows: Vec<Row> = ship_rows.iter().filter(|r| usable(r)).cloned().collect();
    let class_rows: Vec<Row> = class_rows.iter().filter(|r| usable(r)).cloned().collect();
    let side = ship_side(&ship_rows, ship_what, date, exceptions)?;
    let (rows, what, doubts) = if side.ship_holds {
        (ship_rows, ship_what, Vec::new())
    } else {
        (class_rows, class_what, side.doubts)
    };
    let mut by_item: BTreeMap<String, Vec<Row>> = BTreeMap::new();
    for row in rows {
        by_item.entry(row.item.clone()).or_default().push(row);
    }
    let mut items = Vec::new();
    for (item, rows) in by_item {
        match resolve(&rows, &format!("{what}, item {item}"), date, exceptions) {
            Err(failure) if failure.kind == Kind::NotOnDate => {}
            read => items.push((item, read)),
        }
    }
    Ok(SlotRead {
        items,
        doubts,
        blank,
    })
}
