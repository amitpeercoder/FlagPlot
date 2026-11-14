//! The troop figure of a class row, `troop_capacity_men` (decision 0093, 8):
//! the men a ship carries, read in a fixed order from what a source states,
//! the load of one voyage, or the total of a named run.

use std::collections::{BTreeMap, BTreeSet};

use oob_check::table::Data;

use crate::date::Day;
use crate::decimal::Dec;
use crate::derive::{number, Cite, Env, Figure, Value, Work};
use crate::input::TroopsInput;
use crate::problem::{Kind, Problem};
use crate::reading::{resolve, resolve_over, Failure, Read, Row, Table};

/// The name of the figure, the last column of the class CSV.
pub const TROOP_FIGURE: &str = "troop_capacity_men";

/// The rule, as written in the sources file.
pub const TROOP_RULE: &str = "troop_capacity_men of the ship, else of its class, as a source states it; else, where the list names a voyage, troops_carried_men of the ship on that voyage (a load on one voyage, not a capacity); else, where the list names a run for the ship, troops_carried of the run over the ships in the run as the list reads them from the named ships_in_run row, rounded down (a run total, a weaker kind of support than a stated capacity); else 0 for a warship (no troop row: carries none) and an error for a transport or cargo ship (decision 0093, 8)";

const STATED: &str = "troop_capacity_men";
const VOYAGE_FACT: &str = "troops_carried_men";
const FACTS_FILE: &str = "troop_lift_facts.csv";
const ENTITIES_FILE: &str = "lift_entities.csv";
const MEN_ATTRIBUTE: &str = "troops_carried";
const SHIPS_ATTRIBUTE: &str = "ships_in_run";
const RUN_ATTRIBUTE: &str = "troop_run";

/// A run as it is read for one ship.
#[derive(Clone, Debug)]
struct ShipRun {
    run: String,
    men: i64,
    men_row: u64,
    ships_row: u64,
    ships_in_run: i64,
    reason: String,
    /// Other `troops_carried` rows of the run, not read: `troop_lift_facts.csv:N
    /// (standing; value)`.
    other_rows: Vec<String>,
    /// The ship's `troop_run` rows that name this run.
    ship_rows: Vec<u64>,
}

/// What the list says about the troop figure, checked against the facts.
#[derive(Clone, Debug, Default)]
pub struct TroopEnv {
    /// The voyage whose loads are read: as written, and as a day.
    pub voyage: Option<(String, Day)>,
    runs: BTreeMap<String, ShipRun>,
}

fn whole_number(text: &str) -> Option<i64> {
    if text.is_empty() || !text.bytes().all(|b| b.is_ascii_digit()) {
        return None;
    }
    text.parse().ok()
}

fn one_line(text: &str) -> String {
    text.split_whitespace().collect::<Vec<_>>().join(" ")
}

/// Checks the runs of the list against the facts. The problems are of the
/// list's choices; the figure itself is read for each ship by
/// [`Env::troop_capacity`].
pub fn build(input: &TroopsInput, data: &Data) -> (TroopEnv, Vec<Problem>) {
    let mut problems = Vec::new();
    let mut env = TroopEnv {
        voyage: input.voyage.clone(),
        runs: BTreeMap::new(),
    };
    let entities = data.rows(ENTITIES_FILE);
    let facts = data.rows(FACTS_FILE);
    let ship_facts = data.rows("ship_facts.csv");
    for entry in &input.runs {
        let run = entry.run.as_str();
        let mut fail = |message: String| {
            problems.push(Problem::new(run, Kind::List, STATED, message));
        };
        let mut ok = true;
        match entities.iter().find(|r| r.get("key") == run) {
            None => {
                fail(format!("`{run}` is not a key of {ENTITIES_FILE}"));
                ok = false;
            }
            Some(r) if r.get("kind") != "run" => {
                fail(format!(
                    "`{run}` is a key of {ENTITIES_FILE} of kind `{}`, not a run",
                    r.get("kind")
                ));
                ok = false;
            }
            Some(_) => {}
        }
        let row = |id: i64| facts.iter().find(|r| r.get("id") == id.to_string());
        let mut men = None;
        match row(entry.men_row) {
            None => {
                fail(format!(
                    "men_row {} is not a row of {FACTS_FILE}",
                    entry.men_row
                ));
                ok = false;
            }
            Some(r) => {
                if r.get("attribute") != MEN_ATTRIBUTE || r.get("entity") != run {
                    fail(format!(
                        "men_row {} is not a `{MEN_ATTRIBUTE}` row of `{run}` (it is `{}` of `{}`)",
                        entry.men_row,
                        r.get("attribute"),
                        r.get("entity")
                    ));
                    ok = false;
                } else {
                    match whole_number(r.get("value")).filter(|n| *n > 0) {
                        Some(n) => men = Some(n),
                        None => {
                            fail(format!(
                                "men_row {} has value `{}`, which is not a whole number above 0",
                                entry.men_row,
                                r.get("value")
                            ));
                            ok = false;
                        }
                    }
                }
            }
        }
        match row(entry.ships_row) {
            None => {
                fail(format!(
                    "ships_row {} is not a row of {FACTS_FILE}",
                    entry.ships_row
                ));
                ok = false;
            }
            Some(r) if r.get("attribute") != SHIPS_ATTRIBUTE || r.get("entity") != run => {
                fail(format!(
                    "ships_row {} is not a `{SHIPS_ATTRIBUTE}` row of `{run}` (it is `{}` of `{}`)",
                    entry.ships_row,
                    r.get("attribute"),
                    r.get("entity")
                ));
                ok = false;
            }
            Some(_) => {}
        }
        let mut other_rows = Vec::new();
        for r in &facts {
            if r.get("entity") == run
                && r.get("attribute") == MEN_ATTRIBUTE
                && r.get("id") != entry.men_row.to_string()
            {
                other_rows.push(format!(
                    "{FACTS_FILE}:{} ({}; {})",
                    r.get("id"),
                    r.get("standing"),
                    r.get("value")
                ));
            }
        }
        let mut per_ship = Vec::new();
        for ship in &entry.ships {
            let rows: Vec<u64> = ship_facts
                .iter()
                .filter(|r| {
                    r.get("entity") == ship
                        && r.get("attribute") == RUN_ATTRIBUTE
                        && r.get("value") == run
                        && r.get("standing") != "disputed"
                })
                .filter_map(|r| r.get("id").parse().ok())
                .collect();
            if rows.is_empty() {
                problems.push(Problem::new(
                    ship,
                    Kind::List,
                    STATED,
                    format!(
                        "no ship_facts.csv row says {RUN_ATTRIBUTE} {run} for the ship (a disputed row does not count)"
                    ),
                ));
                ok = false;
            }
            per_ship.push((ship, rows));
        }
        let (Some(men), true) = (men, ok) else {
            continue;
        };
        for (ship, ship_rows) in per_ship {
            env.runs.insert(
                ship.clone(),
                ShipRun {
                    run: entry.run.clone(),
                    men,
                    men_row: entry.men_row.unsigned_abs(),
                    ships_row: entry.ships_row.unsigned_abs(),
                    ships_in_run: entry.ships_in_run,
                    reason: one_line(&entry.reason),
                    other_rows: other_rows.clone(),
                    ship_rows,
                },
            );
        }
    }
    (env, problems)
}

/// A whole number the row says, or why not.
fn whole(read: &Read, fact: &str) -> Result<Dec, Failure> {
    let dec = number(read, fact)?;
    let row = &read.rows[0];
    if whole_number(&row.value).is_none() {
        return Err(Failure {
            kind: Kind::BadValue,
            message: format!(
                "{fact} of {}: row {} has value `{}`, which is not a whole number",
                row.entity,
                row.cite(),
                row.value
            ),
        });
    }
    Ok(dec)
}

/// `file:id (standing)` of the rows, in the order given.
fn not_read<'a>(rows: impl Iterator<Item = &'a Row>) -> Vec<String> {
    rows.map(|r| format!("{} ({})", r.cite(), r.standing))
        .collect()
}

fn recoverable(failure: &Failure) -> bool {
    matches!(
        failure.kind,
        Kind::Missing | Kind::NotPreferred | Kind::NotOnDate
    )
}

impl Env<'_> {
    /// The troop figure of one ship, in the order of [`TROOP_RULE`]. `None`
    /// with a problem recorded when it cannot be derived.
    pub(crate) fn troop_capacity(
        &self,
        w: &mut Work,
        ship: &str,
        class_key: &str,
        kind_type: &str,
    ) -> Option<Figure> {
        let troops = self.troops.as_ref()?;
        let mut notes: BTreeSet<String> = BTreeSet::new();
        let figure = |value: i64, cites: BTreeSet<Cite>, excepted, doubts, notes| {
            Dec::from_int(value).map(|value| Figure {
                value: Value::Num(value),
                cites,
                excepted,
                doubts,
                notes,
            })
        };

        // 1. A stated capacity of the ship, else of its class.
        let ship_rows = self.index.rows(Table::ShipFacts, ship, STATED);
        let class_rows = self.index.rows(Table::ClassFacts, class_key, STATED);
        match resolve_over(
            ship_rows,
            &format!("{STATED} of ship {ship}"),
            class_rows,
            &format!("{STATED} of class {class_key}"),
            self.date,
            &self.rules.exceptions,
        ) {
            Ok(read) => match whole(&read, STATED) {
                Ok(value) => {
                    let (cites, excepted, doubts) = w.take(&read);
                    return figure(value.whole(), cites, excepted, doubts, notes);
                }
                Err(failure) => {
                    w.failed(STATED, failure);
                    return None;
                }
            },
            Err(failure) if recoverable(&failure) => {
                let unread = not_read(ship_rows.iter().chain(class_rows));
                if !unread.is_empty() {
                    notes.insert(format!("stated rows not read: {}", unread.join(", ")));
                }
            }
            Err(failure) => {
                w.failed(STATED, failure);
                return None;
            }
        }

        // 2. The load of the ship on the list's voyage.
        if let Some((text, day)) = &troops.voyage {
            let rows = self.index.rows(Table::ShipFacts, ship, VOYAGE_FACT);
            match resolve(
                rows,
                &format!("{VOYAGE_FACT} of ship {ship}"),
                *day,
                &self.rules.exceptions,
            ) {
                Ok(read) => match whole(&read, VOYAGE_FACT) {
                    Ok(value) => {
                        let (cites, excepted, doubts) = w.take(&read);
                        notes.insert(format!(
                            "men she carried on the voyage of {text}: a load on one voyage, not a stated capacity"
                        ));
                        return figure(value.whole(), cites, excepted, doubts, notes);
                    }
                    Err(failure) => {
                        w.failed(STATED, failure);
                        return None;
                    }
                },
                Err(failure) if recoverable(&failure) => {
                    let unread = not_read(rows.iter());
                    if !unread.is_empty() {
                        notes.insert(format!(
                            "{VOYAGE_FACT} rows not read: {}",
                            unread.join(", ")
                        ));
                    }
                }
                Err(failure) => {
                    w.failed(STATED, failure);
                    return None;
                }
            }
        }

        // 3. The total of a run the list names for the ship.
        if let Some(run) = troops.runs.get(ship) {
            let (Some(men), Some(ships)) =
                (Dec::from_int(run.men), Dec::from_int(run.ships_in_run))
            else {
                w.fail(
                    Kind::BadValue,
                    STATED,
                    format!("the run {} does not fit", run.run),
                );
                return None;
            };
            let Some(exact) = men.div(ships) else {
                w.fail(
                    Kind::BadValue,
                    STATED,
                    format!("the run {} does not divide", run.run),
                );
                return None;
            };
            notes.insert(format!(
                "men carried on run {}: {} men ({FACTS_FILE}:{}) over {} ships ({FACTS_FILE}:{}), rounded down: a run total, a weaker kind of support than a stated capacity; exact {exact}; {}",
                run.run, run.men, run.men_row, run.ships_in_run, run.ships_row, run.reason
            ));
            if !run.other_rows.is_empty() {
                notes.insert(format!(
                    "other rows of the run not read: {}",
                    run.other_rows.join(", ")
                ));
            }
            let mut cites: BTreeSet<Cite> = BTreeSet::new();
            cites.insert((FACTS_FILE.to_string(), run.men_row));
            cites.insert((FACTS_FILE.to_string(), run.ships_row));
            cites.extend(
                run.ship_rows
                    .iter()
                    .map(|id| ("ship_facts.csv".to_string(), *id)),
            );
            return figure(exact.whole(), cites, false, BTreeSet::new(), notes);
        }

        // 4. No troop row: none for a warship, an error for a transport.
        if matches!(kind_type, "transport" | "cargo_ship") {
            w.fail(
                Kind::Missing,
                STATED,
                format!(
                    "{STATED} of ship {ship}: no stated capacity, no load on the list's voyage and no run, and a transport must have one"
                ),
            );
            return None;
        }
        notes.insert("no troop row: carries none".to_string());
        figure(0, BTreeSet::new(), false, BTreeSet::new(), notes)
    }
}
