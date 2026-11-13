//! Rates of fire as achieved (decision 0070).
//!
//! A class's `gun_rounds_per_min` is the lesser of its sourced rate and the
//! achieved rate of its kind of battery. The achieved rate of a kind is the
//! median of named cases, each a ship for which rows of
//! `engagement_outcomes.csv` (and `engagement_events.csv`) state the shells
//! one battery fired and the span it fired over. The cases, the classes that
//! fire in continuous rapid fire and the classes that must not read their
//! sourced rate are all written in the `[fire_rate]` block of the rules file.

use std::collections::{BTreeMap, BTreeSet};

use oob_check::table::{Data, Rec};
use serde::Deserialize;

use crate::date::Day;
use crate::decimal::Dec;
use crate::derive::{Cite, Env};
use crate::problem::{Kind, Problem};
use crate::reading::Row;

const OUTCOMES: &str = "engagement_outcomes.csv";
const EVENTS: &str = "engagement_events.csv";

const SHELLS_MEASURES: [&str; 2] = ["main_rounds_fired", "rounds_fired_in_window"];
const SPAN_MEASURES: [&str; 2] = ["firing_duration_min", "duration_min"];
const OPENING_KINDS: [&str; 2] = ["open_fire", "resume_fire"];
const CLOSING_KINDS: [&str; 3] = ["cease_fire", "check_fire", "salvo"];

/// The kinds of battery with an achieved rate.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum FireKind {
    Salvo,
    Destroyer,
    Continuous,
}

impl FireKind {
    pub const ALL: [FireKind; 3] = [FireKind::Salvo, FireKind::Destroyer, FireKind::Continuous];

    pub fn name(self) -> &'static str {
        match self {
            FireKind::Salvo => "salvo",
            FireKind::Destroyer => "destroyer",
            FireKind::Continuous => "continuous",
        }
    }

    fn parse(text: &str) -> Option<FireKind> {
        FireKind::ALL.into_iter().find(|k| k.name() == text)
    }
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RawFireRate {
    min_span_minutes: i64,
    #[serde(default)]
    case: Vec<RawCase>,
    #[serde(default)]
    continuous: Vec<RawContinuous>,
    #[serde(default)]
    no_sourced_rate: Vec<RawNoSourced>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RawCase {
    kind: String,
    ship: String,
    class: String,
    engagement: String,
    shells_row: i64,
    #[serde(default)]
    span_row: Option<i64>,
    #[serde(default)]
    span_from_event: Option<i64>,
    #[serde(default)]
    span_to_event: Option<i64>,
    reason: String,
    agreed: String,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RawContinuous {
    class: String,
    rows: Vec<i64>,
    reason: String,
    agreed: String,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RawNoSourced {
    class: String,
    row: i64,
    reason: String,
    agreed: String,
}

/// Where a case's span comes from.
#[derive(Clone, Debug)]
pub enum SpanSource {
    /// A duration row of `engagement_outcomes.csv`.
    Row(i64),
    /// An opening and a closing row of `engagement_events.csv`.
    Events(i64, i64),
}

#[derive(Clone, Debug)]
pub struct Case {
    pub kind: FireKind,
    pub ship: String,
    pub class: String,
    pub engagement: String,
    pub shells_row: i64,
    pub span: SpanSource,
    pub reason: String,
    pub agreed: String,
}

impl Case {
    fn label(&self) -> String {
        format!("{} ({})", self.ship, self.engagement)
    }
}

#[derive(Clone, Debug)]
pub struct ContinuousClass {
    pub class: String,
    pub rows: Vec<i64>,
    pub reason: String,
    pub agreed: String,
}

#[derive(Clone, Debug)]
pub struct NoSourcedRate {
    pub class: String,
    pub row: i64,
    pub reason: String,
    pub agreed: String,
}

/// The `[fire_rate]` block of the rules file, checked for form.
#[derive(Clone, Debug)]
pub struct FireRateRules {
    pub min_span_minutes: i64,
    pub cases: Vec<Case>,
    pub continuous: Vec<ContinuousClass>,
    pub no_sourced_rate: Vec<NoSourcedRate>,
}

impl FireRateRules {
    pub fn continuous_class(&self, class: &str) -> Option<&ContinuousClass> {
        self.continuous.iter().find(|c| c.class == class)
    }

    pub fn no_sourced_rate(&self, class: &str) -> Option<&NoSourcedRate> {
        self.no_sourced_rate.iter().find(|c| c.class == class)
    }

    /// The kind of battery of a class: continuous if the block names it,
    /// else by the simulation kind of its type.
    pub fn kind_of(&self, class: &str, simulation_kind: &str) -> Option<FireKind> {
        if self.continuous_class(class).is_some() {
            return Some(FireKind::Continuous);
        }
        match simulation_kind {
            "cruiser" | "battleship" => Some(FireKind::Salvo),
            "destroyer" => Some(FireKind::Destroyer),
            _ => None,
        }
    }
}

fn rules_problem(figure: String, message: impl Into<String>) -> Problem {
    Problem::new("rules", Kind::Rules, figure, message)
}

/// Checks the form of the block: no data is read.
pub fn parse(raw: RawFireRate) -> (Option<FireRateRules>, Vec<Problem>) {
    let mut problems = Vec::new();
    if raw.min_span_minutes <= 0 {
        problems.push(rules_problem(
            "fire_rate".to_string(),
            "min_span_minutes must be above zero",
        ));
    }
    let common = |figure: &str, reason: &str, agreed: &str, problems: &mut Vec<Problem>| {
        let mut ok = true;
        for (field, value) in [("reason", reason), ("agreed", agreed)] {
            if value.trim().is_empty() {
                problems.push(rules_problem(
                    figure.to_string(),
                    format!("`{field}` is empty"),
                ));
                ok = false;
            }
        }
        if !agreed.trim().is_empty() && Day::parse_full(agreed).is_none() {
            problems.push(rules_problem(
                figure.to_string(),
                format!("`agreed` is `{agreed}`, not a full date YYYY-MM-DD"),
            ));
            ok = false;
        }
        ok
    };
    let mut cases = Vec::new();
    for raw in raw.case {
        let figure = format!("fire_rate case {} ({})", raw.ship, raw.engagement);
        let mut ok = common(&figure, &raw.reason, &raw.agreed, &mut problems);
        for (field, value) in [
            ("ship", &raw.ship),
            ("class", &raw.class),
            ("engagement", &raw.engagement),
        ] {
            if value.trim().is_empty() {
                problems.push(rules_problem(figure.clone(), format!("`{field}` is empty")));
                ok = false;
            }
        }
        let kind = FireKind::parse(&raw.kind);
        if kind.is_none() {
            problems.push(rules_problem(
                figure.clone(),
                format!(
                    "kind `{}` is not one of salvo, destroyer, continuous",
                    raw.kind
                ),
            ));
            ok = false;
        }
        let span = match (raw.span_row, raw.span_from_event, raw.span_to_event) {
            (Some(row), None, None) => Some(SpanSource::Row(row)),
            (None, Some(from), Some(to)) => Some(SpanSource::Events(from, to)),
            _ => {
                problems.push(rules_problem(
                    figure.clone(),
                    "give exactly one span: `span_row`, or both `span_from_event` and `span_to_event`",
                ));
                None
            }
        };
        if let (true, Some(kind), Some(span)) = (ok, kind, span) {
            cases.push(Case {
                kind,
                ship: raw.ship,
                class: raw.class,
                engagement: raw.engagement,
                shells_row: raw.shells_row,
                span,
                reason: raw.reason,
                agreed: raw.agreed,
            });
        }
    }
    let mut continuous: Vec<ContinuousClass> = Vec::new();
    for raw in raw.continuous {
        let figure = format!("fire_rate continuous {}", raw.class);
        let mut ok = common(&figure, &raw.reason, &raw.agreed, &mut problems);
        if raw.class.trim().is_empty() || raw.rows.is_empty() {
            problems.push(rules_problem(
                figure.clone(),
                "`class` and `rows` must be given",
            ));
            ok = false;
        }
        if continuous.iter().any(|c| c.class == raw.class) {
            problems.push(rules_problem(
                figure.clone(),
                "the class is listed more than once",
            ));
            ok = false;
        }
        if ok {
            continuous.push(ContinuousClass {
                class: raw.class,
                rows: raw.rows,
                reason: raw.reason,
                agreed: raw.agreed,
            });
        }
    }
    let mut no_sourced_rate: Vec<NoSourcedRate> = Vec::new();
    for raw in raw.no_sourced_rate {
        let figure = format!("fire_rate no_sourced_rate {}", raw.class);
        let mut ok = common(&figure, &raw.reason, &raw.agreed, &mut problems);
        if raw.class.trim().is_empty() {
            problems.push(rules_problem(figure.clone(), "`class` is empty"));
            ok = false;
        }
        if no_sourced_rate.iter().any(|c| c.class == raw.class) {
            problems.push(rules_problem(
                figure.clone(),
                "the class is listed more than once",
            ));
            ok = false;
        }
        if ok {
            no_sourced_rate.push(NoSourcedRate {
                class: raw.class,
                row: raw.row,
                reason: raw.reason,
                agreed: raw.agreed,
            });
        }
    }
    let rules = problems.is_empty().then_some(FireRateRules {
        min_span_minutes: raw.min_span_minutes,
        cases,
        continuous,
        no_sourced_rate,
    });
    (rules, problems)
}

/// One case, worked.
#[derive(Clone, Debug)]
pub struct CaseResult {
    /// Index into the rules' cases.
    pub case: usize,
    pub shells: Dec,
    pub guns: u32,
    pub minutes: Dec,
    pub rate: Dec,
    /// The rows it rests on: the outcome and event rows and the fit rows.
    pub cites: BTreeSet<Cite>,
    /// Exceptions the fit reading of the case used.
    pub used: Vec<(usize, Row)>,
}

/// The achieved rates, from the cases.
#[derive(Clone, Debug)]
pub struct Achieved {
    pub cases: Vec<CaseResult>,
    /// Kind to its median and the number of cases.
    pub medians: BTreeMap<FireKind, (Dec, usize)>,
}

impl Achieved {
    pub fn cases_of(&self, rules: &FireRateRules, kind: FireKind) -> Vec<&CaseResult> {
        self.cases
            .iter()
            .filter(|c| rules.cases[c.case].kind == kind)
            .collect()
    }
}

fn median(mut rates: Vec<Dec>) -> Option<Dec> {
    rates.sort();
    let n = rates.len();
    match n {
        0 => None,
        _ if n % 2 == 1 => Some(rates[n / 2]),
        _ => rates[n / 2 - 1]
            .checked_add(rates[n / 2])?
            .div(Dec::from_int(2)?),
    }
}

/// `YYYY-MM-DDTHH:MM` as minutes since the epoch day.
fn minutes_of(time: &str) -> Option<i64> {
    let (date, clock) = time.split_once('T')?;
    let day = Day::parse_full(date)?;
    let (h, m) = clock.split_once(':')?;
    let digits = |s: &str| s.len() == 2 && s.bytes().all(|b| b.is_ascii_digit());
    if !digits(h) || !digits(m) {
        return None;
    }
    let (h, m): (i64, i64) = (h.parse().ok()?, m.parse().ok()?);
    if h > 23 || m > 59 {
        return None;
    }
    Some(day.day_number() * 1440 + h * 60 + m)
}

struct Tables<'a> {
    outcomes: BTreeMap<&'a str, Rec<'a>>,
    events: BTreeMap<&'a str, Rec<'a>>,
    engagements: BTreeMap<&'a str, Rec<'a>>,
}

fn cite(file: &str, id: &str) -> Cite {
    (file.to_string(), id.parse().unwrap_or(0))
}

/// Reads and checks every case, and works the medians. Every problem is
/// returned; the achieved rates only when there is none.
pub fn compute(env: &Env, data: &Data, rules: &FireRateRules) -> (Option<Achieved>, Vec<Problem>) {
    let tables = Tables {
        outcomes: data
            .rows(OUTCOMES)
            .into_iter()
            .map(|r| (r.get("id"), r))
            .collect(),
        events: data
            .rows(EVENTS)
            .into_iter()
            .map(|r| (r.get("id"), r))
            .collect(),
        engagements: data
            .rows("engagements.csv")
            .into_iter()
            .map(|r| (r.get("key"), r))
            .collect(),
    };
    let mut problems = Vec::new();
    let mut results = Vec::new();
    for (index, case) in rules.cases.iter().enumerate() {
        let figure = format!("fire_rate case {}", case.label());
        let mut bad = Vec::new();
        let worked = work_case(env, rules, &tables, index, case, &mut bad);
        problems.extend(bad.into_iter().map(|m| rules_problem(figure.clone(), m)));
        results.extend(worked);
    }
    let class_exists = |class: &str| env.classes.contains_key(class);
    for c in &rules.continuous {
        let figure = format!("fire_rate continuous {}", c.class);
        if !class_exists(&c.class) {
            problems.push(rules_problem(
                figure.clone(),
                format!("class `{}` is not in ship_classes.csv", c.class),
            ));
        }
        for row in &c.rows {
            if !tables.outcomes.contains_key(row.to_string().as_str()) {
                problems.push(rules_problem(
                    figure.clone(),
                    format!("row {row} is not a row of {OUTCOMES}"),
                ));
            }
        }
    }
    for n in &rules.no_sourced_rate {
        let figure = format!("fire_rate no_sourced_rate {}", n.class);
        if !class_exists(&n.class) {
            problems.push(rules_problem(
                figure.clone(),
                format!("class `{}` is not in ship_classes.csv", n.class),
            ));
        }
        let id = n.row.to_string();
        if !data
            .rows("weapon_facts.csv")
            .iter()
            .any(|r| r.get("id") == id)
        {
            problems.push(rules_problem(
                figure,
                format!("row {} is not a row of weapon_facts.csv", n.row),
            ));
        }
    }
    if !problems.is_empty() {
        return (None, problems);
    }
    let mut medians = BTreeMap::new();
    for kind in FireKind::ALL {
        let rates: Vec<Dec> = results
            .iter()
            .filter(|r: &&CaseResult| rules.cases[r.case].kind == kind)
            .map(|r| r.rate)
            .collect();
        let n = rates.len();
        if let Some(m) = median(rates) {
            medians.insert(kind, (m, n));
        }
    }
    (
        Some(Achieved {
            cases: results,
            medians,
        }),
        problems,
    )
}

fn work_case(
    env: &Env,
    rules: &FireRateRules,
    t: &Tables,
    index: usize,
    case: &Case,
    bad: &mut Vec<String>,
) -> Option<CaseResult> {
    let before = bad.len();
    let mut cites = BTreeSet::new();
    // The class and the ship.
    if !env.classes.contains_key(&case.class) {
        bad.push(format!("class `{}` is not in ship_classes.csv", case.class));
    }
    if let Some(ship) = env.ships.get(&case.ship) {
        if ship.class != case.class {
            bad.push(format!(
                "ship `{}` is of class `{}` in ships.csv, not `{}`",
                case.ship, ship.class, case.class
            ));
        }
    }
    // The date of the engagement, for the guns.
    let date = match t.engagements.get(case.engagement.as_str()) {
        None => {
            bad.push(format!(
                "engagement `{}` is not in engagements.csv",
                case.engagement
            ));
            None
        }
        Some(rec) => {
            let day = Day::parse_full(rec.get("date_from"));
            if day.is_none() {
                bad.push(format!(
                    "engagement `{}` has date_from `{}`, not a full date",
                    case.engagement,
                    rec.get("date_from")
                ));
            }
            day
        }
    };
    // The shells.
    let shells_id = case.shells_row.to_string();
    let mut shells = None;
    match t.outcomes.get(shells_id.as_str()) {
        None => bad.push(format!(
            "shells row {} is not a row of {OUTCOMES}",
            case.shells_row
        )),
        Some(rec) => {
            let mut fine = true;
            if rec.get("engagement") != case.engagement || rec.get("ship") != case.ship {
                bad.push(format!(
                    "shells row {} is about `{}` in `{}`, not `{}` in `{}`",
                    case.shells_row,
                    rec.get("ship"),
                    rec.get("engagement"),
                    case.ship,
                    case.engagement
                ));
                fine = false;
            }
            if !SHELLS_MEASURES.contains(&rec.get("measure")) {
                bad.push(format!(
                    "shells row {} has measure `{}`, not main_rounds_fired or rounds_fired_in_window",
                    case.shells_row,
                    rec.get("measure")
                ));
                fine = false;
            }
            match Dec::parse(rec.get("value")) {
                Ok(v) if v.is_positive() && fine => {
                    shells = Some(v);
                    cites.insert(cite(OUTCOMES, rec.get("id")));
                }
                Ok(v) if v.is_positive() => {}
                _ => bad.push(format!(
                    "shells row {} has value `{}`, which is not a number above zero",
                    case.shells_row,
                    rec.get("value")
                )),
            }
        }
    }
    // The span.
    let minutes = match &case.span {
        SpanSource::Row(row) => span_from_row(t, case, *row, &mut cites, bad),
        SpanSource::Events(from, to) => span_from_events(t, case, *from, *to, &mut cites, bad),
    };
    if let Some(m) = minutes {
        let enough = Dec::from_int(rules.min_span_minutes).is_some_and(|min| m >= min);
        if !enough {
            bad.push(format!(
                "the span is {m} minutes, under min_span_minutes {}",
                rules.min_span_minutes
            ));
        }
    }
    // The guns.
    let mut guns = None;
    let mut used = Vec::new();
    if let Some(date) = date {
        match env.fit_read_on((&case.ship, &case.class), "main_battery", date) {
            Err(failure) => bad.push(format!("guns: {} ({})", failure.message, failure.kind)),
            Ok(read) => match crate::derive::count(&read, "main_battery") {
                Ok(n) if n > 0 => {
                    guns = Some(n);
                    cites.extend(read.rows.iter().map(Row::cite_key));
                    used = read.excepted.clone();
                }
                Ok(_) => bad.push("the main battery fit has no guns".to_string()),
                Err(failure) => bad.push(format!("guns: {}", failure.message)),
            },
        }
    }
    if bad.len() > before {
        return None;
    }
    let (shells, minutes, guns) = (shells?, minutes?, guns?);
    let rate = Dec::from_int(i64::from(guns))
        .and_then(|g| g.mul(minutes))
        .and_then(|d| shells.div(d));
    let Some(rate) = rate else {
        bad.push("the rate does not fit".to_string());
        return None;
    };
    Some(CaseResult {
        case: index,
        shells,
        guns,
        minutes,
        rate,
        cites,
        used,
    })
}

fn span_from_row(
    t: &Tables,
    case: &Case,
    row: i64,
    cites: &mut BTreeSet<Cite>,
    bad: &mut Vec<String>,
) -> Option<Dec> {
    let id = row.to_string();
    let Some(rec) = t.outcomes.get(id.as_str()) else {
        bad.push(format!("span row {row} is not a row of {OUTCOMES}"));
        return None;
    };
    let mut fine = true;
    if rec.get("engagement") != case.engagement || rec.get("ship") != case.ship {
        bad.push(format!(
            "span row {row} is about `{}` in `{}`, not `{}` in `{}`",
            rec.get("ship"),
            rec.get("engagement"),
            case.ship,
            case.engagement
        ));
        fine = false;
    }
    if !SPAN_MEASURES.contains(&rec.get("measure")) {
        bad.push(format!(
            "span row {row} has measure `{}`, not firing_duration_min or duration_min",
            rec.get("measure")
        ));
        fine = false;
    }
    let minutes = match Dec::parse(rec.get("value")) {
        Ok(v) if v.is_positive() => Some(v),
        _ => {
            bad.push(format!(
                "span row {row} has value `{}`, which is not a number above zero",
                rec.get("value")
            ));
            None
        }
    };
    if fine {
        cites.insert(cite(OUTCOMES, rec.get("id")));
        minutes
    } else {
        None
    }
}

fn span_from_events(
    t: &Tables,
    case: &Case,
    from: i64,
    to: i64,
    cites: &mut BTreeSet<Cite>,
    bad: &mut Vec<String>,
) -> Option<Dec> {
    let mut times = Vec::new();
    for (what, row, kinds) in [
        ("from", from, &OPENING_KINDS[..]),
        ("to", to, &CLOSING_KINDS[..]),
    ] {
        let id = row.to_string();
        let Some(rec) = t.events.get(id.as_str()) else {
            bad.push(format!("span_{what}_event {row} is not a row of {EVENTS}"));
            continue;
        };
        let mut fine = true;
        if rec.get("engagement") != case.engagement {
            bad.push(format!(
                "span_{what}_event {row} is of `{}`, not `{}`",
                rec.get("engagement"),
                case.engagement
            ));
            fine = false;
        }
        if rec.get("actor") != case.ship {
            bad.push(format!(
                "span_{what}_event {row} has actor `{}`, not `{}`",
                rec.get("actor"),
                case.ship
            ));
            fine = false;
        }
        if !kinds.contains(&rec.get("kind")) {
            bad.push(format!(
                "span_{what}_event {row} has kind `{}`, not one of {}",
                rec.get("kind"),
                kinds.join(", ")
            ));
            fine = false;
        }
        let Some(time) = minutes_of(rec.get("time")) else {
            bad.push(format!(
                "span_{what}_event {row} has time `{}`, not YYYY-MM-DDTHH:MM",
                rec.get("time")
            ));
            continue;
        };
        if fine {
            cites.insert(cite(EVENTS, rec.get("id")));
            times.push(time);
        }
    }
    let [start, end] = times[..] else {
        return None;
    };
    if end <= start {
        bad.push(format!(
            "event {to} is not after event {from}: the span is {} minutes",
            end - start
        ));
        return None;
    }
    Dec::from_int(end - start)
}

fn plain(text: &str) -> String {
    text.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn cite_list(cites: &BTreeSet<Cite>) -> String {
    cites
        .iter()
        .map(|(file, id)| format!("{file}:{id}"))
        .collect::<Vec<_>>()
        .join(" ")
}

/// The section of the class CSV's header that states the achieved rates,
/// one comment line each, without the leading `# `.
pub fn header_lines(rules: &FireRateRules, achieved: &Achieved) -> Vec<String> {
    let mut lines = vec!["Achieved rates of fire (decision 0070):".to_string()];
    for kind in FireKind::ALL {
        lines.push(match achieved.medians.get(&kind) {
            Some((median, n)) => format!(
                "  {}: median {median} of {n} {}",
                kind.name(),
                if *n == 1 { "case" } else { "cases" }
            ),
            None => format!("  {}: no case: the sourced rate is kept", kind.name()),
        });
    }
    for result in &achieved.cases {
        let case = &rules.cases[result.case];
        lines.push(format!(
            "  {} ({}), {}: {} / ({} x {}) = {}; rows {}; reason: {}; agreed {}",
            case.ship,
            case.engagement,
            case.kind.name(),
            result.shells,
            result.guns,
            result.minutes,
            result.rate,
            cite_list(&result.cites),
            plain(&case.reason),
            case.agreed
        ));
    }
    for c in &rules.continuous {
        lines.push(format!(
            "  continuous class {}: rows {}; reason: {}; agreed {}",
            c.class,
            c.rows
                .iter()
                .map(|r| format!("{OUTCOMES}:{r}"))
                .collect::<Vec<_>>()
                .join(" "),
            plain(&c.reason),
            c.agreed
        ));
    }
    for n in &rules.no_sourced_rate {
        lines.push(format!(
            "  no sourced rate for {}: weapon_facts.csv:{} is not read; reason: {}; agreed {}",
            n.class,
            n.row,
            plain(&n.reason),
            n.agreed
        ));
    }
    lines
}
