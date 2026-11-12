//! A ship's experience value, derived from fact rows by four yes or no
//! questions (decision 0064, items 1 to 5).
//!
//! A value is a whole number from 0 to 4 and starts at 2. Each question a
//! row answers "yes" moves it by 1, once, however many rows answer; a
//! question no row answers moves nothing (a missing fact is a gap, not a
//! guess). The questions, as the architect fixed them:
//!
//! 1. `night_practice_recent` (up 1): a preferred row of medium or high
//!    confidence, naming the ship, says she fired or exercised at night, and
//!    its date falls inside the three months that end on the engagement's
//!    first day. An undated "recently" answers nothing. A statement about a
//!    force or a squadron answers for a ship only if a row names her.
//! 2. `no_night_practice` (down 1): the same kind of row says she held no
//!    night practice over a period that reaches into the twelve months that
//!    end on that day.
//! 3. `new_ship` (down 1): her `commissioned` date is later than that day
//!    less twelve months. Where every commissioning row is disputed, the
//!    question is answered if every row, whatever its standing, gives the
//!    same answer.
//! 4. `navy_night_first` (up 1): the rules file names, for a nation, the
//!    `training_facts.csv` rows that say its fleet training put night action
//!    first, and the kinds of ship they cover. A nation with no such row is
//!    not answered "no": nothing moves.
//!
//! The list carries each answer (`[[experience.answer]]`). The tool checks
//! the row exists, is of that ship, is preferred and of medium or high
//! confidence, and that its period meets the question's window. The period
//! is read from the row's `valid_from` and `valid_to`; an answer may give
//! dates too, and they must agree, and where the row has none (a row that
//! states no period) the answer must give them. A statement about a force or
//! a squadron may answer question 1 for a ship (`named_by`): the statement
//! row (a night practice row, or a training row) is preferred, firm and dated
//! inside the window, and each `named_by` row is of that ship, of medium or
//! high confidence, preferred (or disputed, with every disputed row of hers
//! that bears on the period named too) and dated so that it puts her with the
//! force inside the statement's period; the attributes allowed for each are
//! in the rules file (`[experience.force]`). A row for and a row against on
//! one ship and question stop the derivation unless the list settles it
//! (`settle`). Rows that no question read never move a value; they are
//! printed by a run and by `--check`, never written to the file.
//!
//! The Japanese values rest on one statement about a navy. Decision 0064
//! requires a second source before they are fixed, so while the rules file's
//! `second_source` list for a nation is empty the output marks those ships
//! `provisional`.

use std::collections::{BTreeMap, BTreeSet};

use oob_check::table::{Data, Rec};
use serde::Deserialize;

use crate::date::{Day, Span};
use crate::problem::{Kind, Problem};

const START: i32 = 2;
const MIN: i32 = 0;
const MAX: i32 = 4;

pub const Q1: &str = "night_practice_recent";
pub const Q2: &str = "no_night_practice";

const SHIP_FACTS: &str = "ship_facts.csv";
const TRAINING_FACTS: &str = "training_facts.csv";
const NIGHT_PRACTICE: &str = "night_practice_note";

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RawExperience {
    engagement: String,
    output: String,
    ships: Vec<String>,
    #[serde(default)]
    answer: Vec<RawAnswer>,
    #[serde(default)]
    settle: Vec<RawSettle>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RawAnswer {
    ship: String,
    question: String,
    row: String,
    answer: String,
    #[serde(default)]
    named_by: Vec<String>,
    #[serde(default)]
    from: String,
    #[serde(default)]
    to: String,
    reason: String,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RawSettle {
    ship: String,
    question: String,
    row: String,
    reason: String,
}

#[derive(Clone, Debug)]
pub struct Answer {
    pub ship: String,
    pub question: &'static str,
    /// The row that answers: the ship's own, or, with `named_by`, the
    /// statement about a force.
    pub row: RowRef,
    /// Rows of this ship in `ship_facts.csv` that name her as part of the
    /// force at that time. Empty for an answer by the ship's own row.
    pub named_by: Vec<i64>,
    pub yes: bool,
    /// The period. As written in the list it is optional: the tool reads it
    /// from the row, and these are what the list gave (filled in with the
    /// row's own when the facts are read).
    pub from: String,
    pub to: String,
    pub reason: String,
}

/// A row of `ship_facts.csv` or `training_facts.csv`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RowRef {
    pub file: &'static str,
    pub id: i64,
}

impl std::fmt::Display for RowRef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.file, self.id)
    }
}

#[derive(Clone, Debug)]
pub struct Settle {
    pub ship: String,
    pub question: &'static str,
    pub row: i64,
    pub reason: String,
}

/// The `[experience]` table of a list, checked.
#[derive(Clone, Debug)]
pub struct Experience {
    pub engagement: String,
    pub output: String,
    pub ships: Vec<String>,
    pub answers: Vec<Answer>,
    pub settles: Vec<Settle>,
}

fn list_problem(figure: &str, message: impl Into<String>) -> Problem {
    Problem::new("list", Kind::List, figure, message)
}

fn question_name(text: &str) -> Option<&'static str> {
    [Q1, Q2].into_iter().find(|q| *q == text)
}

/// `ship_facts.csv:1016` to 1016 of that file, or the same for
/// `training_facts.csv`.
fn row_ref(text: &str) -> Option<RowRef> {
    [SHIP_FACTS, TRAINING_FACTS].into_iter().find_map(|file| {
        let id = text
            .strip_prefix(file)?
            .strip_prefix(':')?
            .parse()
            .ok()
            .filter(|n: &i64| *n >= 0)?;
        Some(RowRef { file, id })
    })
}

/// `ship_facts.csv:1016` to 1016 (a row that names a ship in a force).
fn ship_row_id(text: &str) -> Option<i64> {
    row_ref(text).filter(|r| r.file == SHIP_FACTS).map(|r| r.id)
}

pub fn parse_list(raw: RawExperience) -> (Option<Experience>, Vec<Problem>) {
    let mut problems = Vec::new();
    let mut bad = |figure: &str, message: String| problems.push(list_problem(figure, message));
    if raw.engagement.trim().is_empty() {
        bad("experience", "`engagement` must not be empty".to_string());
    }
    let path = std::path::Path::new(&raw.output);
    if path.file_stem().is_none() || path.extension().is_none_or(|e| e != "csv") {
        bad(
            "experience",
            format!("`output` `{}` is not the path of a .csv file", raw.output),
        );
    }
    if raw.ships.is_empty() {
        bad("experience", "`ships` must not be empty".to_string());
    }
    let mut ships: Vec<String> = Vec::new();
    for ship in &raw.ships {
        if ships.contains(ship) {
            bad(
                "experience",
                format!("ship `{ship}` is listed more than once"),
            );
        } else {
            ships.push(ship.clone());
        }
    }
    let mut answers: Vec<Answer> = Vec::new();
    for (i, a) in raw.answer.iter().enumerate() {
        let figure = format!("experience answer {}", i + 1);
        let mut ok = true;
        let mut fail = |message: String| {
            bad(&figure, message);
            ok = false;
        };
        if !ships.contains(&a.ship) {
            fail(format!("`{}` is not in the experience ships", a.ship));
        }
        let question = question_name(&a.question);
        if question.is_none() {
            fail(format!(
                "question `{}` is not `{Q1}` or `{Q2}` (the other two are read from the facts)",
                a.question
            ));
        }
        let row = row_ref(&a.row);
        if row.is_none() {
            fail(format!(
                "row `{}` is not `{SHIP_FACTS}:<id>` or `{TRAINING_FACTS}:<id>`",
                a.row
            ));
        }
        let mut named_by: Vec<i64> = Vec::new();
        for text in &a.named_by {
            match ship_row_id(text) {
                Some(id) if !named_by.contains(&id) => named_by.push(id),
                Some(_) => fail(format!("`named_by` names `{text}` twice")),
                None => fail(format!(
                    "`named_by` row `{text}` is not `{SHIP_FACTS}:<id>`"
                )),
            }
        }
        if !named_by.is_empty() && question == Some(Q2) {
            fail(format!("`named_by` is for {Q1} only"));
        }
        let yes = match a.answer.as_str() {
            "yes" => true,
            "no" => false,
            other => {
                fail(format!("answer `{other}` is not yes or no"));
                false
            }
        };
        if a.reason.trim().is_empty() {
            fail("`reason` must not be empty".to_string());
        }
        let mut span = |column: &str, text: &str| {
            if !text.is_empty() && Span::parse(text).is_none() {
                fail(format!("`{column}` `{text}` is not a date"));
            }
        };
        span("from", &a.from);
        span("to", &a.to);
        // Whether the row gives the dates or the answer must is known only
        // when the facts are read (`resolve_period`).
        if let (true, Some(question), Some(row)) = (ok, question, row) {
            if answers
                .iter()
                .any(|b| b.ship == a.ship && b.question == question && b.row == row)
            {
                bad(&figure, "the same answer is given twice".to_string());
            } else {
                answers.push(Answer {
                    ship: a.ship.clone(),
                    question,
                    row,
                    named_by,
                    yes,
                    from: a.from.clone(),
                    to: a.to.clone(),
                    reason: a.reason.clone(),
                });
            }
        }
    }
    let mut settles: Vec<Settle> = Vec::new();
    for (i, s) in raw.settle.iter().enumerate() {
        let figure = format!("experience settle {}", i + 1);
        let (question, row) = (question_name(&s.question), ship_row_id(&s.row));
        if !ships.contains(&s.ship) {
            bad(
                &figure,
                format!("`{}` is not in the experience ships", s.ship),
            );
        } else if let (Some(question), Some(row)) = (question, row) {
            if s.reason.trim().is_empty() {
                bad(&figure, "`reason` must not be empty".to_string());
            } else {
                settles.push(Settle {
                    ship: s.ship.clone(),
                    question,
                    row,
                    reason: s.reason.clone(),
                });
            }
        } else {
            bad(
                &figure,
                format!("question `{}` or row `{}` is not valid", s.question, s.row),
            );
        }
    }
    (
        problems.is_empty().then_some(Experience {
            engagement: raw.engagement,
            output: raw.output,
            ships,
            answers,
            settles,
        }),
        problems,
    )
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RawRulesExperience {
    recent_months: u32,
    lookback_months: u32,
    new_ship_months: u32,
    #[serde(default)]
    navy: Vec<RawNavy>,
    #[serde(default)]
    force: Option<RawForce>,
}

/// Which attributes may carry a statement about a force and which may name
/// a ship as part of it, each as `<file>:<attribute>`.
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RawForce {
    statement: Vec<String>,
    named_by: Vec<String>,
}

/// The attributes an answer by a force's statement may use.
#[derive(Clone, Debug)]
pub struct ForceRules {
    /// `(file, attribute)` of a row that may be the statement.
    pub statement: Vec<(&'static str, String)>,
    /// `(file, attribute)` of a row that may name a ship as part of the force.
    pub named_by: Vec<(&'static str, String)>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RawNavy {
    nation: String,
    entity: String,
    rows: Vec<i64>,
    kinds: Vec<String>,
    #[serde(default)]
    second_source: Vec<i64>,
    /// The entity a second source row may have; the navy's `entity` if not
    /// given (a second source may speak of the navy's whole surface fleet).
    #[serde(default)]
    second_source_entity: Option<String>,
}

/// A navy's statement that its fleet training put night action first.
#[derive(Clone, Debug)]
pub struct Navy {
    pub nation: String,
    pub entity: String,
    pub rows: Vec<i64>,
    /// Class types (`ship_classes.csv`) the rows cover.
    pub kinds: Vec<String>,
    /// Rows of a second source. While empty the values are provisional.
    pub second_source: Vec<i64>,
    pub second_source_entity: String,
}

#[derive(Clone, Debug)]
pub struct ExperienceRules {
    pub recent_months: u32,
    pub lookback_months: u32,
    pub new_ship_months: u32,
    pub navies: Vec<Navy>,
    /// None when the rules file has no `[experience.force]`: then an answer
    /// with `named_by` is a problem.
    pub force: Option<ForceRules>,
}

pub fn parse_rules(raw: RawRulesExperience) -> (Option<ExperienceRules>, Vec<Problem>) {
    let mut problems = Vec::new();
    let mut bad = |message: String| {
        problems.push(Problem::new("rules", Kind::Rules, "experience", message));
    };
    for (name, value) in [
        ("recent_months", raw.recent_months),
        ("lookback_months", raw.lookback_months),
        ("new_ship_months", raw.new_ship_months),
    ] {
        if value == 0 {
            bad(format!("`{name}` must be at least 1"));
        }
    }
    let mut navies: Vec<Navy> = Vec::new();
    for n in raw.navy {
        if navies.iter().any(|m| m.nation == n.nation) {
            bad(format!("nation `{}` has two entries", n.nation));
        } else if n.nation.trim().is_empty()
            || n.entity.trim().is_empty()
            || n.rows.is_empty()
            || n.kinds.is_empty()
        {
            bad(format!(
                "navy `{}`: nation, entity, rows and kinds must not be empty",
                n.nation
            ));
        } else {
            navies.push(Navy {
                nation: n.nation,
                second_source_entity: n.second_source_entity.unwrap_or_else(|| n.entity.clone()),
                entity: n.entity,
                rows: n.rows,
                kinds: n.kinds,
                second_source: n.second_source,
            });
        }
    }
    let force = raw.force.map(|f| {
        let mut read = |what: &str, list: &[String]| -> Vec<(&'static str, String)> {
            let mut out = Vec::new();
            for text in list {
                let parsed = [SHIP_FACTS, TRAINING_FACTS].into_iter().find_map(|file| {
                    let attribute = text.strip_prefix(file)?.strip_prefix(':')?;
                    (!attribute.is_empty()).then(|| (file, attribute.to_string()))
                });
                match parsed {
                    Some(pair) => out.push(pair),
                    None => problems.push(Problem::new(
                        "rules",
                        Kind::Rules,
                        "experience force",
                        format!(
                            "`{what}` entry `{text}` is not `{SHIP_FACTS}:<attribute>` or \
                             `{TRAINING_FACTS}:<attribute>`"
                        ),
                    )),
                }
            }
            if out.is_empty() && list.is_empty() {
                problems.push(Problem::new(
                    "rules",
                    Kind::Rules,
                    "experience force",
                    format!("`{what}` must not be empty"),
                ));
            }
            out
        };
        let statement = read("statement", &f.statement);
        let named_by = read("named_by", &f.named_by);
        ForceRules {
            statement,
            named_by,
        }
    });
    (
        problems.is_empty().then_some(ExperienceRules {
            recent_months: raw.recent_months,
            lookback_months: raw.lookback_months,
            new_ship_months: raw.new_ship_months,
            navies,
            force,
        }),
        problems,
    )
}

/// What a derivation of one list needs to say about itself.
pub struct Names<'a> {
    pub list_file: &'a str,
    pub list_title: &'a str,
    pub rules_file: &'a str,
}

type Cited = BTreeSet<(String, i64)>;

fn problem(ship: &str, kind: Kind, figure: &str, message: impl Into<String>) -> Problem {
    Problem::new(ship, kind, figure, message)
}

fn by_id<'a>(rows: &[Rec<'a>], id: i64) -> Option<Rec<'a>> {
    rows.iter().find(|r| r.get("id") == id.to_string()).copied()
}

fn firm(row: &Rec) -> bool {
    row.get("standing") == "preferred" && matches!(row.get("confidence"), "medium" | "high")
}

/// Whether a row puts a ship with the force inside the statement's period
/// `sf..=st`: its own period meets it. A row with a start and no end goes on
/// (she joined and stayed); a row with an end and no start has always been so;
/// a row with neither says nothing about when.
fn with_force(from: &str, to: &str, sf: Span, st: Span) -> bool {
    if from.is_empty() && to.is_empty() {
        return false;
    }
    let began = match Span::parse(from) {
        Some(f) => f.first <= st.last,
        None => from.is_empty(),
    };
    let not_ended = match Span::parse(to) {
        Some(t) => t.last >= sf.first,
        None => to.is_empty(),
    };
    began && not_ended
}

/// Whether a period may lie inside `start..=end`: it can begin on or after
/// `start` and end on or before `end`. A partial date counts as inside if any
/// day it could stand for is.
fn within(from: &str, to: &str, start: Day, end: Day) -> bool {
    let (Some(f), Some(t)) = (Span::parse(from), Span::parse(to)) else {
        return false;
    };
    f.last >= start && t.first <= end && f.first <= t.last
}

/// Whether a period reaches into `start..=end`: it ends on or after `start`
/// and begins on or before `end`. An empty `from` is an open start.
fn reaches(from: &str, to: &str, start: Day, end: Day) -> bool {
    let Some(t) = Span::parse(to) else {
        return false;
    };
    let began = match Span::parse(from) {
        Some(f) => f.first <= end && f.first <= t.last,
        None => from.is_empty(),
    };
    t.last >= start && began
}

/// A derived experience file, and what the run says beside it.
pub struct Derived {
    /// The text of the generated file.
    pub text: String,
    /// For each listed ship with any: the rows of hers that no question read,
    /// as `ship: file:id(standing), ...`. Printed by `--check` and by a run;
    /// never written to the file, so a new row for a ship leaves it as it was.
    pub unread: Vec<String>,
}

struct Verdict {
    /// The question moves the value.
    yes: bool,
    cited: Cited,
}

pub fn derive(
    listed: &Experience,
    rules: &ExperienceRules,
    data: &Data,
    names: &Names,
) -> Result<Derived, Vec<Problem>> {
    let mut problems = Vec::new();

    let engagements = data.rows("engagements.csv");
    let Some(first_day) = engagements
        .iter()
        .find(|r| r.get("key") == listed.engagement)
        .and_then(|r| Day::parse_full(r.get("date_from")))
    else {
        return Err(vec![list_problem(
            "experience",
            format!(
                "engagement `{}` is not in engagements.csv with a full date_from",
                listed.engagement
            ),
        )]);
    };
    let recent_start = first_day.minus_months(rules.recent_months);
    let lookback_start = first_day.minus_months(rules.lookback_months);
    let new_cut = first_day.minus_months(rules.new_ship_months);

    // The navy rows must exist and say what the rules file says they do.
    let training = data.rows(TRAINING_FACTS);
    for navy in &rules.navies {
        let figure = format!("navy {}", navy.nation);
        let mut first_sources: BTreeSet<&str> = BTreeSet::new();
        let mut bad = |message: String| {
            problems.push(problem("rules", Kind::Rules, &figure, message));
        };
        for id in &navy.rows {
            match by_id(&training, *id) {
                None => bad(format!("row {TRAINING_FACTS}:{id} does not exist")),
                Some(row) => {
                    if row.get("entity") != navy.entity
                        || row.get("attribute") != "night_training_note"
                        || !firm(&row)
                    {
                        bad(format!(
                            "row {TRAINING_FACTS}:{id} is not a preferred, medium or high \
                             `night_training_note` row of `{}`",
                            navy.entity
                        ));
                    } else {
                        first_sources.insert(row.get("source"));
                    }
                }
            }
        }
        // A second source: a row of the navy's entity (or the entity the rules
        // name for it), a `night_training_note` from a different source than
        // the first rows, whose standing is not `disputed`. It need not be
        // preferred: it may be one witness.
        for id in &navy.second_source {
            match by_id(&training, *id) {
                None => bad(format!("row {TRAINING_FACTS}:{id} does not exist")),
                Some(row) => {
                    if row.get("entity") != navy.second_source_entity
                        || row.get("attribute") != "night_training_note"
                        || row.get("standing") == "disputed"
                    {
                        bad(format!(
                            "second source row {TRAINING_FACTS}:{id} is not a `night_training_note` \
                             row of `{}` that is not disputed",
                            navy.second_source_entity
                        ));
                    } else if first_sources.contains(row.get("source")) {
                        bad(format!(
                            "second source row {TRAINING_FACTS}:{id} is from the source \
                             of the first rows, so it is not a second source"
                        ));
                    }
                }
            }
        }
    }

    let ship_rows = data.rows("ships.csv");
    let class_rows = data.rows("ship_classes.csv");
    let facts = data.rows(SHIP_FACTS);
    let mut facts_of: BTreeMap<&str, Vec<Rec>> = BTreeMap::new();
    for row in &facts {
        facts_of.entry(row.get("entity")).or_default().push(*row);
    }

    // Answers: each is checked against the rows it cites, and its period is
    // read from the row (or from the answer, where the row has none).
    let training_rows = data.rows(TRAINING_FACTS);
    let mut owned = listed.clone();
    for a in &mut owned.answers {
        let ship = a.ship.clone();
        let question = a.question;
        let mut bad = |kind: Kind, message: String| {
            problems.push(problem(&ship, kind, question, message));
        };
        let stmt = if a.row.file == SHIP_FACTS {
            by_id(&facts, a.row.id)
        } else {
            by_id(&training_rows, a.row.id)
        };
        let Some(stmt) = stmt else {
            bad(Kind::Experience, format!("row {} does not exist", a.row));
            continue;
        };
        let forced = !a.named_by.is_empty();
        if forced {
            match &rules.force {
                None => {
                    bad(
                        Kind::Rules,
                        "`named_by` needs the rules file's `[experience.force]` table".to_string(),
                    );
                    continue;
                }
                Some(force) => {
                    if !force
                        .statement
                        .iter()
                        .any(|(f, at)| *f == a.row.file && at == stmt.get("attribute"))
                    {
                        bad(
                            Kind::Experience,
                            format!(
                                "row {} is a `{}` row, which the rules file does not allow as the \
                                 statement about a force",
                                a.row,
                                stmt.get("attribute")
                            ),
                        );
                    } else if !firm(&stmt) {
                        bad(
                            Kind::NotPreferred,
                            format!(
                                "row {} is {} at {} confidence: only a preferred row of medium or \
                                 high confidence answers",
                                a.row,
                                stmt.get("standing"),
                                stmt.get("confidence")
                            ),
                        );
                    }
                }
            }
        } else if a.row.file != SHIP_FACTS {
            bad(
                Kind::Experience,
                format!(
                    "row {} is not a row of this ship; a statement about a force answers only \
                     with `named_by`",
                    a.row
                ),
            );
        } else if stmt.get("entity") != ship {
            bad(
                Kind::Experience,
                format!("row {} is not a row of this ship", a.row),
            );
        } else if stmt.get("attribute") != NIGHT_PRACTICE {
            bad(
                Kind::Experience,
                format!(
                    "row {} is a `{}` row, not `{NIGHT_PRACTICE}`",
                    a.row,
                    stmt.get("attribute")
                ),
            );
        } else if !firm(&stmt) {
            bad(
                Kind::NotPreferred,
                format!(
                    "row {} is {} at {} confidence: only a preferred row of medium or high \
                     confidence answers",
                    a.row,
                    stmt.get("standing"),
                    stmt.get("confidence")
                ),
            );
        }

        // The period: from the row where it has one, and then the answer's, if
        // it gives one, must agree; where the row has none, the answer gives it.
        let mut period = |column: &str, row_value: &str, listed: &str, required: bool| {
            if !row_value.is_empty() {
                if !listed.is_empty() && listed != row_value {
                    bad(
                        Kind::Experience,
                        format!(
                            "`{column}` `{listed}` disagrees with row {}, which has `{row_value}`",
                            a.row
                        ),
                    );
                    None
                } else {
                    Some(row_value.to_string())
                }
            } else if !listed.is_empty() {
                Some(listed.to_string())
            } else if required {
                bad(
                    Kind::Experience,
                    format!("row {} has no `{column}` and the answer gives none", a.row),
                );
                None
            } else {
                Some(String::new())
            }
        };
        let from = period(
            "valid_from",
            stmt.get("valid_from"),
            &a.from,
            question == Q1,
        );
        let to = period("valid_to", stmt.get("valid_to"), &a.to, true);
        let (Some(from), Some(to)) = (from, to) else {
            continue;
        };
        a.from = from;
        a.to = to;
        let (start, ok) = if question == Q1 {
            (
                recent_start,
                within(&a.from, &a.to, recent_start, first_day),
            )
        } else {
            (
                lookback_start,
                reaches(&a.from, &a.to, lookback_start, first_day),
            )
        };
        if !ok {
            bad(
                Kind::Experience,
                format!(
                    "row {} is dated {}..{}, which does not meet the window {start} to {first_day}",
                    a.row,
                    if a.from.is_empty() { "-" } else { &a.from },
                    a.to
                ),
            );
            continue;
        }
        if !forced {
            continue;
        }

        // Rows that name her as part of the force inside the statement's period.
        let (Some(force), Some(sf), Some(st)) =
            (&rules.force, Span::parse(&a.from), Span::parse(&a.to))
        else {
            continue;
        };
        let hers = facts_of.get(ship.as_str()).map_or(&[][..], Vec::as_slice);
        let allowed = |r: &Rec| {
            force
                .named_by
                .iter()
                .any(|(f, at)| *f == SHIP_FACTS && at == r.get("attribute"))
        };
        for id in &a.named_by {
            let Some(row) = by_id(hers, *id) else {
                bad(
                    Kind::Experience,
                    format!("named_by row {SHIP_FACTS}:{id} is not a row of this ship"),
                );
                continue;
            };
            if !allowed(&row) {
                bad(
                    Kind::Experience,
                    format!(
                        "named_by row {SHIP_FACTS}:{id} is a `{}` row, which the rules file does \
                         not allow to name a ship as part of a force",
                        row.get("attribute")
                    ),
                );
            } else if !matches!(row.get("confidence"), "medium" | "high")
                || !matches!(row.get("standing"), "preferred" | "disputed")
            {
                bad(
                    Kind::NotPreferred,
                    format!(
                        "named_by row {SHIP_FACTS}:{id} is {} at {} confidence: it must be \
                         preferred (or disputed, with every disputed row of hers named) and of \
                         medium or high confidence",
                        row.get("standing"),
                        row.get("confidence")
                    ),
                );
            } else if !with_force(row.get("valid_from"), row.get("valid_to"), sf, st) {
                bad(
                    Kind::NotOnDate,
                    format!(
                        "named_by row {SHIP_FACTS}:{id} is dated {}..{}, which does not put her \
                         with the force inside the statement's period {}..{}",
                        row.get("valid_from"),
                        row.get("valid_to"),
                        a.from,
                        a.to
                    ),
                );
            }
        }
        // A disputed row may name her only if every disputed row of hers that
        // bears on the period is named, so that all of them are read together.
        let named_disputed = a
            .named_by
            .iter()
            .filter_map(|id| by_id(hers, *id))
            .any(|r| r.get("standing") == "disputed");
        if named_disputed {
            let missing: Vec<String> = hers
                .iter()
                .filter(|r| allowed(r) && r.get("standing") == "disputed")
                .filter(|r| with_force(r.get("valid_from"), r.get("valid_to"), sf, st))
                .filter_map(|r| r.get("id").parse::<i64>().ok())
                .filter(|id| !a.named_by.contains(id))
                .map(|id| format!("{SHIP_FACTS}:{id}"))
                .collect();
            if !missing.is_empty() {
                bad(
                    Kind::Conflict,
                    format!(
                        "a disputed row names her, so every disputed row of hers that bears on \
                         the period must be named too; missing {}",
                        missing.join(", ")
                    ),
                );
            }
        }
    }
    let exp = &owned;
    for s in &exp.settles {
        let rows: Vec<&Answer> = exp
            .answers
            .iter()
            .filter(|a| a.ship == s.ship && a.question == s.question)
            .collect();
        let conflict = rows.iter().any(|a| a.yes) && rows.iter().any(|a| !a.yes);
        if !conflict
            || !rows
                .iter()
                .any(|a| a.row.file == SHIP_FACTS && a.row.id == s.row)
        {
            problems.push(problem(
                &s.ship,
                Kind::Experience,
                s.question,
                format!(
                    "settle names {SHIP_FACTS}:{}, but there is no conflict to settle or it is \
                     not one of the answers",
                    s.row
                ),
            ));
        }
    }

    let mut lines: Vec<Vec<String>> = Vec::new();
    let mut any_provisional = false;
    let mut navy_notes: BTreeSet<String> = BTreeSet::new();
    let mut unread: Vec<String> = Vec::new();
    for ship in &exp.ships {
        let Some(ship_row) = ship_rows.iter().find(|r| r.get("key") == ship) else {
            problems.push(problem(
                ship,
                Kind::UnknownShip,
                "experience",
                "not in ships.csv",
            ));
            continue;
        };
        let rows = facts_of.get(ship.as_str()).map_or(&[][..], Vec::as_slice);

        let q1 = answered(exp, ship, Q1, &mut problems);
        let q2 = answered(exp, ship, Q2, &mut problems);
        let q3 = new_ship(ship, rows, new_cut, &mut problems);

        // Question 4: the navy's row, if the nation has one and covers the kind.
        let class_type = class_rows
            .iter()
            .find(|c| c.get("key") == ship_row.get("class"))
            .map_or("", |c| c.get("type"));
        let navy = rules.navies.iter().find(|n| {
            n.nation == ship_row.get("nation") && n.kinds.iter().any(|k| k == class_type)
        });
        let mut q4_cited = Cited::new();
        let mut provisional = false;
        if let Some(navy) = navy {
            for id in &navy.rows {
                q4_cited.insert((TRAINING_FACTS.to_string(), *id));
            }
            provisional = navy.second_source.is_empty();
        }
        let q4 = navy.is_some();
        any_provisional |= provisional;
        if let Some(navy) = navy {
            let ids = |v: &[i64]| {
                v.iter()
                    .map(|i| format!("{TRAINING_FACTS}:{i}"))
                    .collect::<Vec<_>>()
                    .join(", ")
            };
            navy_notes.insert(format!(
                "# navy_night_first for `{}`: rows {}; second source: {}.",
                navy.nation,
                ids(&navy.rows),
                if navy.second_source.is_empty() {
                    "none".to_string()
                } else {
                    ids(&navy.second_source)
                }
            ));
        }

        let value = (START + i32::from(q1.yes) - i32::from(q2.yes) - i32::from(q3.yes)
            + i32::from(q4))
        .clamp(MIN, MAX);

        let mut cited = Cited::new();
        for v in [&q1, &q2, &q3] {
            cited.extend(v.cited.iter().cloned());
        }
        cited.extend(q4_cited);
        let bears = |r: &Rec| {
            matches!(r.get("attribute"), NIGHT_PRACTICE | "commissioned")
                || rules.force.as_ref().is_some_and(|f| {
                    f.named_by
                        .iter()
                        .any(|(file, at)| *file == SHIP_FACTS && at == r.get("attribute"))
                })
        };
        let not_read: Vec<String> = rows
            .iter()
            .filter(|r| bears(r))
            .filter_map(|r| {
                let id = r.get("id").parse::<i64>().ok()?;
                (!cited.contains(&(SHIP_FACTS.to_string(), id)))
                    .then(|| format!("{SHIP_FACTS}:{id}({})", r.get("standing")))
            })
            .collect();
        if !not_read.is_empty() {
            unread.push(format!("{ship}: {}", not_read.join(", ")));
        }
        let cite = |c: &Cited| {
            c.iter()
                .map(|(f, i)| format!("{f}:{i}"))
                .collect::<Vec<_>>()
                .join("|")
        };
        lines.push(vec![
            ship.clone(),
            value.to_string(),
            if provisional { "provisional" } else { "fixed" }.to_string(),
            move_word(q1.yes, "up"),
            move_word(q2.yes, "down"),
            move_word(q3.yes, "down"),
            move_word(q4, "up"),
            cite(&cited),
        ]);
    }
    if !problems.is_empty() {
        return Err(problems);
    }

    let mut text = String::new();
    let mut line = |s: String| {
        text.push_str(&s);
        text.push('\n');
    };
    line("# GENERATED by oob-derive from the facts in data/oob. Do not edit.".to_string());
    line(format!(
        "# List: {}: {} (engagement {}, first day {first_day})",
        names.list_file, names.list_title, exp.engagement
    ));
    line(format!("# Rules: {}", names.rules_file));
    line(format!(
        "# Experience (decision 0064): a whole number 0 to 4, starting at {START}. Each question a \
         row answers moves it by 1, once; a question no row answers moves nothing."
    ));
    line(format!(
        "# {Q1}: up 1, a preferred row of medium or high confidence naming the ship says she fired \
         or exercised at night within {recent_start}..{first_day} (three months before the first \
         day)."
    ));
    line(format!(
        "# {Q2}: down 1, such a row says she held no night practice over a period reaching into \
         {lookback_start}..{first_day}, and no such row says she held one."
    ));
    line(format!(
        "# new_ship: down 1, her commissioned date is later than {new_cut}."
    ));
    line(
        "# navy_night_first: up 1, the rules file names training_facts rows saying her navy's \
         fleet training put night action first, for her nation and kind of ship."
            .to_string(),
    );
    line(
        "# rows: file:id of the rows read to answer a question, \"no\" answers included, and of the \
         rows that name a ship as part of a force. The rows of a listed ship that no question read \
         are not kept here: `oob-derive` prints them, so a new row for a ship does not make this \
         file stale."
            .to_string(),
    );
    for note in &navy_notes {
        line(note.clone());
    }
    if any_provisional {
        line(
            "# PROVISIONAL: ships marked `provisional` rest on one statement about their navy and \
             the rules file lists no second source for it (decision 0064, item 5). `--check` \
             passes on this file. A stage that writes these values into scenarios must refuse a \
             file that has a provisional ship."
                .to_string(),
        );
    }
    for a in &exp.answers {
        line(format!(
            "# answer: {} {} {} by {}{}, dated {}..{}: {}",
            a.ship,
            a.question,
            if a.yes { "yes" } else { "no" },
            a.row,
            if a.named_by.is_empty() {
                String::new()
            } else {
                format!(
                    " (she is named in the force by {})",
                    a.named_by
                        .iter()
                        .map(|id| format!("{SHIP_FACTS}:{id}"))
                        .collect::<Vec<_>>()
                        .join(", ")
                )
            },
            if a.from.is_empty() { "-" } else { &a.from },
            a.to,
            a.reason.replace('\n', " ")
        ));
    }
    for s in &exp.settles {
        line(format!(
            "# settled: {} {} reads {SHIP_FACTS}:{}: {}",
            s.ship,
            s.question,
            s.row,
            s.reason.replace('\n', " ")
        ));
    }
    line(
        "ship,experience,fixed,night_practice_recent,no_night_practice,new_ship,\
         navy_night_first,rows"
            .to_string(),
    );
    for fields in lines {
        line(fields.join(","));
    }
    Ok(Derived { text, unread })
}

fn move_word(yes: bool, word: &str) -> String {
    if yes { word } else { "none" }.to_string()
}

/// Question 1 or 2 for one ship: the answers the list gives, and what
/// they come to.
fn answered(
    exp: &Experience,
    ship: &str,
    question: &'static str,
    problems: &mut Vec<Problem>,
) -> Verdict {
    let given: Vec<&Answer> = exp
        .answers
        .iter()
        .filter(|a| a.ship == ship && a.question == question)
        .collect();
    let (yes, no): (Vec<&Answer>, Vec<&Answer>) = given.iter().partition(|a| a.yes);
    // The rows an answer read: its own, and those that name her in the force.
    let cite = |list: &[&Answer]| -> Cited {
        list.iter()
            .flat_map(|a| {
                std::iter::once((a.row.file.to_string(), a.row.id))
                    .chain(a.named_by.iter().map(|id| (SHIP_FACTS.to_string(), *id)))
            })
            .collect()
    };
    if !yes.is_empty() && !no.is_empty() {
        let settle = exp
            .settles
            .iter()
            .find(|s| s.ship == ship && s.question == question);
        return match settle {
            Some(s) => {
                let chosen: Vec<&Answer> = given
                    .iter()
                    .filter(|a| a.row.file == SHIP_FACTS && a.row.id == s.row && a.yes)
                    .copied()
                    .collect();
                Verdict {
                    yes: !chosen.is_empty(),
                    cited: cite(&chosen),
                }
            }
            None => {
                let list = |l: &[&Answer]| {
                    l.iter()
                        .map(|a| a.row.to_string())
                        .collect::<Vec<_>>()
                        .join(", ")
                };
                problems.push(problem(
                    ship,
                    Kind::Conflict,
                    question,
                    format!(
                        "preferred rows answer both ways and the list does not settle it: yes by \
                         {}, no by {}",
                        list(&yes),
                        list(&no)
                    ),
                ));
                Verdict {
                    yes: false,
                    cited: Cited::new(),
                }
            }
        };
    }
    Verdict {
        yes: !yes.is_empty(),
        cited: if yes.is_empty() {
            cite(&no)
        } else {
            cite(&yes)
        },
    }
}

/// Question 3 for one ship.
fn new_ship(ship: &str, rows: &[Rec], cut: Day, problems: &mut Vec<Problem>) -> Verdict {
    let none = Verdict {
        yes: false,
        cited: Cited::new(),
    };
    let all: Vec<&Rec> = rows
        .iter()
        .filter(|r| r.get("attribute") == "commissioned")
        .collect();
    if all.is_empty() {
        return none;
    }
    let preferred: Vec<&Rec> = all
        .iter()
        .filter(|r| r.get("standing") == "preferred")
        .copied()
        .collect();
    let used: Vec<&Rec> = if !preferred.is_empty() {
        preferred
    } else if all.iter().all(|r| r.get("standing") == "disputed") {
        all
    } else {
        problems.push(problem(
            ship,
            Kind::NotPreferred,
            "new_ship",
            "no commissioned row is preferred, and not every row is disputed",
        ));
        return none;
    };
    let mut answers: BTreeSet<bool> = BTreeSet::new();
    for row in &used {
        let Some(span) = Span::parse(row.get("value")) else {
            problems.push(problem(
                ship,
                Kind::BadValue,
                "new_ship",
                format!(
                    "{SHIP_FACTS}:{} has `{}`, which is not a date",
                    row.get("id"),
                    row.get("value")
                ),
            ));
            return none;
        };
        if span.first > cut {
            answers.insert(true);
        } else if span.last <= cut {
            answers.insert(false);
        } else {
            problems.push(problem(
                ship,
                Kind::AmbiguousDate,
                "new_ship",
                format!(
                    "{SHIP_FACTS}:{} has `{}`, which cannot be told from the line {cut}",
                    row.get("id"),
                    row.get("value")
                ),
            ));
            return none;
        }
    }
    if answers.len() > 1 {
        problems.push(problem(
            ship,
            Kind::Conflict,
            "new_ship",
            format!("the commissioned rows lie on both sides of {cut}: settle which date is read"),
        ));
        return none;
    }
    Verdict {
        yes: answers.contains(&true),
        cited: used
            .iter()
            .filter_map(|r| r.get("id").parse::<i64>().ok())
            .map(|id| (SHIP_FACTS.to_string(), id))
            .collect(),
    }
}
