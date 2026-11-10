//! Reading the CSV files into rows of text. Nothing is interpreted here.

use std::collections::{BTreeMap, BTreeSet};

use crate::Problem;

/// One data row: the line it started on and its fields, in header order.
#[derive(Debug)]
pub struct Row {
    pub line: usize,
    pub cells: Vec<String>,
}

/// One CSV file: its header and the rows that have the header's field count.
#[derive(Debug, Default)]
pub struct Table {
    pub fields: Vec<String>,
    pub rows: Vec<Row>,
}

impl Table {
    /// Parses a file's text. The file may open with `#` comment lines before
    /// the header. Rows with the wrong number of fields are reported and
    /// left out, so later checks never see a short row.
    pub fn parse(file: &str, text: &str, out: &mut Vec<Problem>) -> Table {
        let mut body = text;
        let mut comment_lines = 0;
        while body.starts_with('#') {
            body = body.split_once('\n').map_or("", |(_, rest)| rest);
            comment_lines += 1;
        }
        let mut reader = csv::ReaderBuilder::new()
            .has_headers(false)
            .flexible(true)
            .from_reader(body.as_bytes());
        let mut table = Table::default();
        let mut seen_header = false;
        for record in reader.records() {
            let record = match record {
                Ok(record) => record,
                Err(err) => {
                    out.push(Problem::file(file, format!("cannot parse: {err}")));
                    break;
                }
            };
            let line = comment_lines
                + record
                    .position()
                    .map_or(0, |p| usize::try_from(p.line()).unwrap_or(0));
            let cells: Vec<String> = record.iter().map(str::to_string).collect();
            if !seen_header {
                seen_header = true;
                table.fields = cells;
            } else if cells.len() != table.fields.len() {
                out.push(Problem {
                    file: file.to_string(),
                    line,
                    row: format!("line {line}"),
                    message: format!(
                        "row has {} fields, the header has {}",
                        cells.len(),
                        table.fields.len()
                    ),
                });
            } else {
                table.rows.push(Row { line, cells });
            }
        }
        if !seen_header {
            out.push(Problem::file(file, "no header row"));
        }
        table
    }

    pub fn column(&self, name: &str) -> Option<usize> {
        self.fields.iter().position(|f| f == name)
    }

    pub fn has(&self, name: &str) -> bool {
        self.column(name).is_some()
    }
}

/// One row together with the file and header it belongs to.
#[derive(Clone, Copy)]
pub struct Rec<'a> {
    pub file: &'a str,
    table: &'a Table,
    row: &'a Row,
}

impl<'a> Rec<'a> {
    /// The field under `column`, or the empty string if the file has no such
    /// column. `schema::check` makes sure the known files have their columns
    /// before any rule reads them.
    pub fn get(&self, column: &str) -> &'a str {
        self.table
            .column(column)
            .map_or("", |i| self.row.cells[i].as_str())
    }

    /// How a problem names this row: by `id`, else by `key`, else by line.
    pub fn label(&self) -> String {
        if self.table.has("id") {
            format!("id {}", self.get("id"))
        } else if self.table.has("key") {
            format!("key {}", self.get("key"))
        } else {
            format!("line {}", self.row.line)
        }
    }

    pub fn problem(&self, message: impl Into<String>) -> Problem {
        Problem {
            file: self.file.to_string(),
            line: self.row.line,
            row: self.label(),
            message: message.into(),
        }
    }
}

/// Entity type to the keys its identity file lists.
pub type EntityKeys<'a> = BTreeMap<&'static str, BTreeSet<&'a str>>;

/// Every file of the data set, by file name.
#[derive(Debug, Default)]
pub struct Data {
    pub tables: BTreeMap<String, Table>,
}

impl Data {
    /// The rows of one file. A file that is not there has no rows.
    pub fn rows<'a>(&'a self, file: &'a str) -> Vec<Rec<'a>> {
        self.tables.get(file).map_or_else(Vec::new, |table| {
            table
                .rows
                .iter()
                .map(|row| Rec { file, table, row })
                .collect()
        })
    }

    /// The `key` column of one file.
    pub fn keys<'a>(&'a self, file: &'a str) -> BTreeSet<&'a str> {
        self.rows(file).iter().map(|r| r.get("key")).collect()
    }

    pub fn entity_keys(&self) -> EntityKeys<'_> {
        crate::schema::ENTITY_FILES
            .iter()
            .map(|(entity_type, file)| (*entity_type, self.keys(file)))
            .collect()
    }
}
