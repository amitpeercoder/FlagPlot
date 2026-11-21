//! The three output files.

use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

use crate::aircraft::{AircraftResult, AIRCRAFT_FIGURES};
use crate::derive::{
    ShipResult, Value, AIR_FIGURES, AMMUNITION_NOT_LIMITING_ROUNDS, FIGURES, KIND, MARK_FIGURE,
    MARK_RULE, RADAR, SURFACE_WEAPONS, WHOLE,
};
use crate::input::{Exception, List, Rules};
use crate::problem::{Kind, Problem};
use crate::reading::Row;
use crate::troops::{TROOP_FIGURE, TROOP_RULE};

/// The rule behind each figure, in words, in the order of `FIGURES`.
const RULES: [&str; 17] = [
    "type of the class in ship_classes.csv, mapped to a simulation kind (heavy_cruiser, light_cruiser: cruiser; destroyer; battleship; carrier, light_carrier: carrier)",
    "endurance_speed_kn of the ship, else of its class",
    "max_speed_kn of the ship, else of its class",
    "fuel_capacity_lt of the ship, else of its class (a simulation ton is a long ton)",
    "endurance_nm of the ship, else of its class",
    "fixed at 0: warships carry no cargo",
    "count of the main_battery fit",
    "rate_of_fire_practical_rpm of the main_battery weapon (rate_of_fire_rpm, a designed rate, is read only where the rules file names a row of it by exception)",
    "main_rounds_per_gun of the ship, else of its class (9999 for a ship the list names as not limited by ammunition, when no row exists)",
    "shell_weight_kg of the main_battery weapon / gun_shell_kg_per_damage",
    "count of the torpedo_tubes fit",
    "tubes that bear on one side (decisions 0074 and 0083): torpedo_tubes_per_broadside of the ship, else of its class, if a read row gives it; else torpedo_mount_position (centreline: all the tubes; wing: half of them); else all the tubes, and the class is named as not split; 0 when no tubes are fitted",
    "warhead_kg of the torpedo_tubes weapon x torpedo_damage_per_warhead_kg; 0 when no tubes are fitted",
    "displacement_standard_lt of the ship, else of its class / displacement_lt_per_toughness",
    "1 if a read surface_search_radar fit has count above 0 and the set it names has a read display row (sensor_facts.csv) with a plan position indicator (PPI) in it; an air_search_radar fit does not give it; a surface-search set with no read display row gives 0 and is named as not counted (decision 0077); 0 if a read fit of a no-radar stand-in item has count 0 or both slots read count 0",
    "aircraft_capacity of the ship, else of its class; 0 where no row exists (an error for a carrier) (decision 0029, 2)",
    "heavy guns (counts of the weapons in the heavy_aa slot) x anti_aircraft_heavy_weight + light barrels (light_aa slot) x anti_aircraft_light_weight, rounded down (decision 0029, 1); the slot of the ship if it has a readable row there, else of its class",
];

/// Which columns a list's class file has beyond the 15 of decision 0020.
#[derive(Clone, Copy)]
pub(crate) struct Cols {
    /// The two air figures (decision 0029).
    pub air: bool,
    /// The troop figure (decision 0093, 8).
    pub troop: bool,
    /// The mark that a class fuels others at sea, written only where the
    /// list has an oiler (decision 0206, rule 1).
    pub mark: bool,
}

impl Cols {
    fn of(list: &List, mark: bool) -> Cols {
        Cols {
            air: list.air_figures,
            troop: list.troop_figures,
            mark,
        }
    }

    fn count(self) -> usize {
        FIGURES.len()
            + if self.air { AIR_FIGURES.len() } else { 0 }
            + usize::from(self.troop)
            + usize::from(self.mark)
    }
}

/// The name of figure `i` of a class row: the 15 of decision 0020, then
/// the two air figures when the list has them, then the troop figure when
/// the list has it, then the mark when the list has an oiler.
fn figure_name(i: usize, cols: Cols) -> &'static str {
    let after_air = FIGURES.len() + if cols.air { AIR_FIGURES.len() } else { 0 };
    if i < FIGURES.len() {
        FIGURES[i]
    } else if i < after_air {
        AIR_FIGURES[i - FIGURES.len()]
    } else if cols.troop && i == after_air {
        TROOP_FIGURE
    } else {
        MARK_FIGURE
    }
}

/// Whether figure `i` is written as a whole number: the counts and rounds
/// carried, and the troop figure, wherever it falls.
fn is_whole(i: usize, cols: Cols) -> bool {
    let name = figure_name(i, cols);
    name == TROOP_FIGURE || (name != MARK_FIGURE && WHOLE.contains(&i))
}

/// The rule behind figure `i`, in words.
fn rule_text(i: usize, cols: Cols) -> &'static str {
    match figure_name(i, cols) {
        TROOP_FIGURE => TROOP_RULE,
        MARK_FIGURE => MARK_RULE,
        _ => RULES[i],
    }
}

/// The three files, as text.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Outputs {
    pub class_csv: String,
    pub sources_csv: String,
    pub ships_csv: String,
    /// The aircraft type file and its sources file, when the list has
    /// aircraft.
    pub aircraft_csv: Option<String>,
    pub aircraft_sources_csv: Option<String>,
    /// The ships' experience values, when the list has `[experience]`.
    pub experience_csv: Option<String>,
    /// Rows of the experience ships that no question read, one line a ship
    /// (not written to the file).
    pub experience_unread: Vec<String>,
    /// Aircraft types whose search radius was not settled, one line a type
    /// (decision 0075); they are also in the aircraft file's header.
    pub search_not_settled: Vec<String>,
    /// Ships with tubes of which no readable row says how they bear, so the
    /// tubes on one side are all of them (decision 0074, 3), one line a
    /// ship; they are also in the class file's header.
    pub torpedo_not_split: Vec<String>,
    /// The attributes estimated (decision 0207, rule 13): a first line with
    /// the count and the ships, then one line an attribute with its method,
    /// working and range (rule 6). Empty for a list with no estimate.
    pub estimated: Vec<String>,
}

/// The name of the column of a class file that names the estimated figures
/// of each class, written only where the list has an estimate (decision
/// 0207, rule 10).
pub const ESTIMATED_COLUMN: &str = "estimated";

/// `<stem>.sources.csv`, the sources file beside an aircraft CSV.
pub fn aircraft_sources_name(output: &str) -> String {
    let stem = Path::new(output).file_stem().map_or_else(
        || "aircraft_types".to_string(),
        |s| s.to_string_lossy().into_owned(),
    );
    format!("{stem}.sources.csv")
}

/// `<stem>.sources.csv` and `<stem>.ships.csv`, the names of the companions
/// of a class CSV, from the output path as the list writes it.
pub fn companion_names(output: &str) -> (String, String) {
    let stem = Path::new(output).file_stem().map_or_else(
        || "ship_classes".to_string(),
        |s| s.to_string_lossy().into_owned(),
    );
    (format!("{stem}.sources.csv"), format!("{stem}.ships.csv"))
}

struct ClassRow {
    id: u32,
    name: String,
    class_id: u64,
    figures: Vec<Value>,
    members: Vec<usize>,
}

fn cell(i: usize, cols: Cols, value: &Value) -> String {
    match value {
        Value::Text(text) => text.clone(),
        Value::Flag(flag) => if *flag { "1" } else { "0" }.to_string(),
        Value::Num(dec) if is_whole(i, cols) => dec.whole().to_string(),
        Value::Num(dec) => dec.to_string(),
    }
}

fn shown(value: &Value) -> String {
    match value {
        Value::Text(text) => text.clone(),
        Value::Flag(flag) => if *flag { "1" } else { "0" }.to_string(),
        Value::Num(dec) => dec.to_string(),
    }
}

pub(crate) fn one_line(text: &str) -> String {
    text.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn csv_text(header: &[&str], rows: &[Vec<String>]) -> String {
    let mut writer = csv::Writer::from_writer(Vec::new());
    writer.write_record(header).expect("writing to memory");
    for row in rows {
        writer.write_record(row).expect("writing to memory");
    }
    let bytes = writer.into_inner().expect("writing to memory");
    String::from_utf8(bytes).expect("the text written was UTF-8")
}

/// A ship's name without the prefix of its navy (USS, HMAS), as it is
/// written in brackets in the name of a class row.
fn short_name(name: &str) -> &str {
    match name.split_once(' ') {
        Some((prefix, rest))
            if (2..=5).contains(&prefix.len())
                && prefix.chars().all(|c| c.is_ascii_uppercase()) =>
        {
            rest
        }
        _ => name,
    }
}

/// The header lines for the troop figure: the voyage and the runs read.
fn troop_header_lines(list: &List) -> Vec<String> {
    let mut lines = Vec::new();
    if let Some((text, _)) = &list.troops.voyage {
        lines.push(format!(
            "Troop figure (decision 0093, 8): the voyage whose loads are read is {text}"
        ));
    }
    if !list.troops.runs.is_empty() {
        lines.push(
            "Troop figure (decision 0093, 8): runs read (run, men row, ships row, ships_in_run):"
                .to_string(),
        );
        for run in &list.troops.runs {
            lines.push(format!(
                "  {}, troop_lift_facts.csv:{}, troop_lift_facts.csv:{}, {}",
                run.run, run.men_row, run.ships_row, run.ships_in_run
            ));
        }
    }
    lines
}

/// The problems of the list's pinned class rows against the derived ships
/// (decision 0077): ships pinned to one id must have one facts class and
/// the same derived figures.
pub fn check_pins(list: &List, results: &[ShipResult], mark: bool) -> Vec<Problem> {
    let mut problems = Vec::new();
    let air = Cols::of(list, mark);
    let Some(pins) = &list.class_ids else {
        return problems;
    };
    for (id, ships) in pins {
        let members: Vec<&ShipResult> =
            results.iter().filter(|r| ships.contains(&r.ship)).collect();
        let Some(first) = members.first() else {
            continue;
        };
        for other in &members[1..] {
            if other.class_id != first.class_id {
                problems.push(Problem::new(
                    &other.ship,
                    Kind::List,
                    "class_ids",
                    format!(
                        "class id {id}: {} is of class {} and {} of class {}; one id is one class",
                        first.ship, first.class_name, other.ship, other.class_name
                    ),
                ));
                continue;
            }
            for (f, (a, b)) in first.figures.iter().zip(&other.figures).enumerate() {
                if a.value != b.value {
                    problems.push(Problem::new(
                        &other.ship,
                        Kind::List,
                        figure_name(f, air),
                        format!(
                            "class id {id}: {} has {} {} and {} has {}; ships pinned to one id must have the same figures",
                            first.ship,
                            figure_name(f, air),
                            shown(&a.value),
                            other.ship,
                            shown(&b.value)
                        ),
                    ));
                }
            }
        }
    }
    problems
}

/// One class row for each pinned id, in id order, holding the ships pinned
/// to it (decision 0077); rows are never merged. Names them.
fn class_rows(list: &List, results: &[ShipResult], mark: bool) -> Result<Vec<ClassRow>, Problem> {
    let pins = list
        .class_ids
        .as_ref()
        .ok_or_else(|| Problem::new("list", Kind::List, "class_ids", "required (decision 0077)"))?;
    let air = Cols::of(list, mark);
    let mut rows = Vec::new();
    for (&id, ships) in pins {
        let members: Vec<usize> = results
            .iter()
            .enumerate()
            .filter(|(_, r)| ships.contains(&r.ship))
            .map(|(i, _)| i)
            .collect();
        let Some(&first) = members.first() else {
            continue;
        };
        rows.push(ClassRow {
            id,
            name: String::new(),
            class_id: results[first].class_id,
            figures: results[first]
                .figures
                .iter()
                .map(|f| f.value.clone())
                .collect(),
            members,
        });
    }
    // Names: the facts class name, and for a class with several rows what
    // differs between them.
    let mut by_class: BTreeMap<u64, Vec<usize>> = BTreeMap::new();
    for (i, row) in rows.iter().enumerate() {
        by_class.entry(row.class_id).or_default().push(i);
    }
    for indexes in by_class.values() {
        let base = results[rows[indexes[0]].members[0]].class_name.clone();
        let figure_count = rows[indexes[0]].figures.len();
        let differing: Vec<usize> = (0..figure_count)
            .filter(|&f| {
                indexes
                    .iter()
                    .any(|&i| rows[i].figures[f] != rows[indexes[0]].figures[f])
            })
            .collect();
        for &i in indexes {
            let mut name = base.clone();
            let not_modelled = results[rows[i].members[0]].weapons_not_modelled;
            if indexes.len() > 1 {
                if not_modelled && differing.iter().any(|f| SURFACE_WEAPONS.contains(f)) {
                    name.push_str(", surface weapons not modelled");
                }
                for &f in &differing {
                    // The surface weapon figures of a ship not modelled are
                    // named by the one phrase above; for a modelled ship
                    // only a difference among the modelled rows is named.
                    if SURFACE_WEAPONS.contains(&f)
                        && (not_modelled
                            || !indexes
                                .iter()
                                .filter(|&&j| !results[rows[j].members[0]].weapons_not_modelled)
                                .any(|&j| rows[j].figures[f] != rows[i].figures[f]))
                    {
                        continue;
                    }
                    match (&rows[i].figures[f], f) {
                        (Value::Flag(true), RADAR) => name.push_str(", radar"),
                        (Value::Flag(false), RADAR) => name.push_str(", no radar"),
                        (value, _) => {
                            name.push_str(&format!(", {} {}", figure_name(f, air), shown(value)));
                        }
                    }
                }
            }
            let equal: bool = indexes
                .iter()
                .any(|&j| j != i && rows[j].figures == rows[i].figures);
            if equal {
                let names: Vec<&str> = rows[i]
                    .members
                    .iter()
                    .map(|&m| short_name(&results[m].ship_name))
                    .collect();
                name.push_str(&format!(" ({})", names.join(", ")));
            }
            rows[i].name = name;
        }
    }
    debug_assert_eq!(
        rows.iter().map(|r| &r.name).collect::<BTreeSet<_>>().len(),
        rows.len(),
        "class row names must be distinct"
    );
    Ok(rows)
}

pub struct Context<'a> {
    /// The list has an oiler, so the class file has the column of the mark
    /// (decision 0206, rule 1).
    pub mark_column: bool,
    pub list: &'a List,
    pub list_name: &'a str,
    pub rules: &'a Rules,
    pub results: &'a [ShipResult],
    /// Exceptions used, deduplicated, in rules order.
    pub used: &'a BTreeMap<(usize, String, u64), Row>,
    pub aircraft: &'a [AircraftResult],
    /// Exceptions the aircraft types used.
    pub aircraft_used: &'a BTreeMap<(usize, String, u64), Row>,
    /// The achieved rates of fire (decision 0070), when the rules have them.
    pub fire_rate: Option<&'a crate::fire_rate::Achieved>,
    /// Exceptions saying `later = true` that do nothing on this list
    /// (decision 0200, 9).
    pub idle_later: &'a [String],
}

/// One exception used, for the header: the row, its standing, the reason
/// and the day agreed, and for one saying `later = true` the row's date.
fn exception_line(e: &Exception, row: &Row) -> String {
    let later = if e.later {
        format!(
            "; later = true: read from a row of a later date, valid from {} (decision 0200)",
            row.from
        )
    } else {
        String::new()
    };
    format!(
        "  {} {}: {} row {} ({}); reason: {}; agreed {}{later}",
        e.entity,
        e.fact,
        row.file,
        row.id,
        row.standing,
        one_line(&e.reason),
        e.agreed
    )
}

/// The header lines for exceptions saying `later = true` that do nothing on
/// this list; none when there are none.
fn idle_lines(idle: &[String]) -> Vec<String> {
    if idle.is_empty() {
        return Vec::new();
    }
    let mut lines = vec![
        "Exceptions saying later = true that do nothing on this list, the row they name not beginning after its date (decision 0200, 9):".to_string(),
    ];
    lines.extend(idle.iter().map(|text| format!("  {}", one_line(text))));
    lines
}

pub fn render(ctx: &Context) -> Result<Outputs, Problem> {
    let Context {
        list,
        list_name,
        rules,
        results,
        used,
        ..
    } = ctx;
    let cols = Cols::of(list, ctx.mark_column);
    let air = list.air_figures;
    if results.is_empty() {
        // A list with no ships writes no class files.
        let (aircraft_csv, aircraft_sources_csv) = aircraft_files(ctx);
        return Ok(Outputs {
            class_csv: String::new(),
            sources_csv: String::new(),
            ships_csv: String::new(),
            aircraft_csv,
            aircraft_sources_csv,
            experience_csv: None,
            experience_unread: Vec::new(),
            search_not_settled: Vec::new(),
            torpedo_not_split: Vec::new(),
            estimated: Vec::new(),
        });
    }
    let troop = list.troop_figures;
    let names: Vec<&str> = (0..cols.count()).map(|i| figure_name(i, cols)).collect();
    let not_modelled: Vec<&str> = results
        .iter()
        .filter(|r| r.weapons_not_modelled)
        .map(|r| r.ship.as_str())
        .collect();
    let rows = class_rows(list, results, ctx.mark_column)?;
    let (sources_name, ships_name) = companion_names(&list.output);
    let class_name = Path::new(&list.output)
        .file_name()
        .map_or_else(String::new, |n| n.to_string_lossy().into_owned());
    let assumed: Vec<&str> = results
        .iter()
        .filter(|r| r.radar_assumed)
        .map(|r| r.ship.as_str())
        .collect();

    let ammunition: Vec<&str> = results
        .iter()
        .filter(|r| r.ammunition_assumed)
        .map(|r| r.ship.as_str())
        .collect();

    // Class CSV.
    let mut class_text = String::new();
    let mut line = |text: String| {
        class_text.push_str("# ");
        class_text.push_str(&text);
        class_text.push('\n');
    };
    line("GENERATED by oob-derive from the facts in data/oob. Do not edit.".to_string());
    line(format!(
        "List: {list_name}: {} (as of {})",
        one_line(&list.name),
        list.date_text
    ));
    line(format!("Rules: {}", list.rules));
    line(format!("Companions: {sources_name} (the rule and fact rows behind every figure), {ships_name} (the class of each listed ship)"));
    line(
        "Constants, MODEL PARAMETERS AND PLACEHOLDERS, not history (decision 0020, 8):".to_string(),
    );
    line(format!(
        "  gun_shell_kg_per_damage = {}",
        rules.gun_shell_kg_per_damage
    ));
    line(format!(
        "  torpedo_damage_per_warhead_kg = {}",
        rules.torpedo_damage_per_warhead_kg
    ));
    line(format!(
        "  displacement_lt_per_toughness = {}",
        rules.displacement_lt_per_toughness
    ));
    if air {
        line(format!(
            "  anti_aircraft_heavy_weight = {}",
            rules
                .anti_aircraft_heavy_weight
                .map_or_else(String::new, |d| d.to_string())
        ));
        line(format!(
            "  anti_aircraft_light_weight = {}",
            rules
                .anti_aircraft_light_weight
                .map_or_else(String::new, |d| d.to_string())
        ));
    }
    line("Counts and rounds carried are whole numbers. The other figures are exact decimals of up to six places, trailing zeros trimmed; a division truncates toward zero at the sixth place.".to_string());
    if used.is_empty() {
        line("Exceptions used (decision 0020, 3): none".to_string());
    } else {
        line("Exceptions used (decision 0020, 3):".to_string());
        for ((index, _, _), row) in used.iter() {
            let e = &rules.exceptions[*index];
            line(exception_line(e, row));
        }
    }
    if !results.is_empty() {
        for text in idle_lines(ctx.idle_later) {
            line(text);
        }
    }
    if let (Some(block), Some(achieved), true) = (
        rules.fire_rate.as_ref(),
        ctx.fire_rate,
        results
            .iter()
            .any(|r| !r.weapons_not_modelled && !r.unarmed_by_kind),
    ) {
        for text in crate::fire_rate::header_lines(block, achieved) {
            line(text);
        }
    }
    if assumed.is_empty() {
        line("Radar assumptions used (decision 0019, 5): none".to_string());
    } else {
        line("Radar assumptions used (decision 0019, 5):".to_string());
        for ship in &assumed {
            line(format!(
                "  {ship}: radar unknown in the facts, treated as not fitted"
            ));
        }
    }
    let uncounted: Vec<(&str, &str)> = results
        .iter()
        .flat_map(|r| {
            r.radar_not_counted
                .iter()
                .map(move |set| (r.ship.as_str(), set.as_str()))
        })
        .collect();
    if uncounted.is_empty() {
        line(
            "Radar sets not counted, no row gives their display (decision 0077, 3): none"
                .to_string(),
        );
    } else {
        line("Radar sets not counted, no row gives their display (decision 0077, 3):".to_string());
        for (ship, set) in &uncounted {
            line(format!("  {ship}: {set}"));
        }
    }
    if ammunition.is_empty() {
        line("Ammunition assumptions used (decision 0023): none".to_string());
    } else {
        line("Ammunition assumptions used (decision 0023):".to_string());
        for ship in &ammunition {
            line(format!(
                "  {ship}: no main_rounds_per_gun row in the facts, main-gun ammunition treated as not limiting ({AMMUNITION_NOT_LIMITING_ROUNDS} is a marker, not an outfit)"
            ));
        }
    }
    let unarmed: Vec<&str> = results
        .iter()
        .filter(|r| r.unarmed_by_kind)
        .map(|r| r.ship.as_str())
        .collect();
    if !unarmed.is_empty() {
        line("No surface weapons by kind (decision 0166, rule 5; decision 0206, rule 3): an auxiliary has no main battery and no torpedo tubes, so main_guns, gun_rounds_per_min, gun_rounds_per_gun, gun_damage, torpedo_tubes, torpedo_tubes_one_side and torpedo_damage are 0 and no fit, shell or outfit is read, for:".to_string());
        for ship in &unarmed {
            line(format!("  {ship}"));
        }
    }
    let estimates: Vec<&crate::derive::EstimateUse> =
        results.iter().flat_map(|r| r.estimates.iter()).collect();
    if !list.estimates.is_empty() {
        line("Estimated, not read (decision 0207): a figure of a real ship worked by a written rule from the rows of like ships, where the record has none; the column `estimated` names the figures of each class that rest on one, and the sources file gives the method, the rows and the range. An estimate is never evidence (rule 12):".to_string());
        for used in &estimates {
            line(format!("  {} ({})", one_line(&used.line), used.figure));
        }
    }
    let aa_assumed: Vec<&str> = results
        .iter()
        .filter(|r| r.anti_aircraft_assumed)
        .map(|r| r.ship.as_str())
        .collect();
    if !list.anti_aircraft_unknown_means_none.is_empty() {
        if aa_assumed.is_empty() {
            line("Anti-aircraft assumptions used (decision 0207): none".to_string());
        } else {
            line("Anti-aircraft assumptions used (decision 0207):".to_string());
            for ship in &aa_assumed {
                line(format!(
                    "  {ship}: anti-aircraft guns unknown in the facts, treated as none, an assumption of the scenario and not a fact"
                ));
            }
        }
    }
    if !not_modelled.is_empty() {
        line("Surface weapons not modelled (decision 0029, 4): main_guns, gun_rounds_per_min, gun_rounds_per_gun, gun_damage, torpedo_tubes, torpedo_tubes_one_side and torpedo_damage are written as 0, an assumption of the scenario and not a fact, for:".to_string());
        for ship in &not_modelled {
            line(format!("  {ship}"));
        }
    }
    let torpedo_not_split: Vec<String> = results
        .iter()
        .filter(|r| r.tubes_not_split)
        .map(|r| format!("{} ({})", r.ship, r.class_name))
        .collect();
    if torpedo_not_split.is_empty() {
        line("Torpedo tubes not split (decision 0074, 3): none".to_string());
    } else {
        line("Torpedo tubes not split (decision 0074, 3): not split: no row says how its tubes bear, so torpedo_tubes_one_side is torpedo_tubes, for:".to_string());
        for ship in &torpedo_not_split {
            line(format!("  {ship}"));
        }
    }
    if troop {
        for text in troop_header_lines(list) {
            line(text);
        }
    }
    let estimate_column = !list.estimates.is_empty();
    let mut class_header: Vec<&str> = [&["id", "name"][..], &names[..]].concat();
    if estimate_column {
        class_header.push(ESTIMATED_COLUMN);
    }
    let class_body: Vec<Vec<String>> = rows
        .iter()
        .map(|row| {
            let mut cells = vec![row.id.to_string(), row.name.clone()];
            cells.extend(
                row.figures
                    .iter()
                    .enumerate()
                    .map(|(i, v)| cell(i, cols, v)),
            );
            if estimate_column {
                let marked: Vec<&str> = names
                    .iter()
                    .enumerate()
                    .filter(|(i, _)| {
                        row.members
                            .iter()
                            .any(|&m| results[m].figures[*i].estimate.is_some())
                    })
                    .map(|(_, name)| *name)
                    .collect();
                cells.push(marked.join(" "));
            }
            cells
        })
        .collect();
    let class_csv = format!("{class_text}{}", csv_text(&class_header, &class_body));

    // Sources CSV.
    let mut sources_text = String::new();
    for text in [
        "GENERATED by oob-derive. Do not edit.".to_string(),
        format!("For each class row of {class_name} and each of its figures: the value as written there, the rule, the fact rows read, and a note."),
        "facts: file:id of each row read, files in data/oob, unioned over the ships of the class row. Constants and fixed values cite no row.".to_string(),
        "The note gives the exact value where the class CSV truncated a count or rounds figure to a whole number, says where a figure rests on an exception or an assumption, and lists the ship rows that hold on the date but were not read (so the class's rows were).".to_string(),
    ] {
        sources_text.push_str("# ");
        sources_text.push_str(&text);
        sources_text.push('\n');
    }
    let mut source_rows = Vec::new();
    for row in &rows {
        for (i, figure) in names.iter().enumerate() {
            let cites: BTreeSet<_> = row
                .members
                .iter()
                .flat_map(|&m| results[m].figures[i].cites.iter().cloned())
                .collect();
            let facts = cites
                .iter()
                .map(|(file, id)| format!("{file}:{id}"))
                .collect::<Vec<_>>()
                .join(" ");
            let mut notes = Vec::new();
            if let Value::Num(exact) = &row.figures[i] {
                if is_whole(i, cols) && exact.has_fraction() {
                    notes.push(format!("exact {exact}, truncated"));
                }
            }
            let doubts: BTreeSet<&str> = row
                .members
                .iter()
                .flat_map(|&m| results[m].figures[i].doubts.iter().map(String::as_str))
                .collect();
            if !doubts.is_empty() {
                notes.push(format!(
                    "ship rows holding on the date but not read: {}",
                    doubts.into_iter().collect::<Vec<_>>().join(", ")
                ));
            }
            let extra: BTreeSet<&str> = row
                .members
                .iter()
                .flat_map(|&m| results[m].figures[i].notes.iter().map(String::as_str))
                .collect();
            notes.extend(extra.into_iter().map(str::to_string));
            if i == 7 && row.members.iter().any(|&m| results[m].designed_rate) {
                notes.push(
                    "designed rate, read by exception (rate_of_fire_rpm, not a practical rate)"
                        .to_string(),
                );
            } else if row.members.iter().any(|&m| results[m].figures[i].excepted) {
                notes.push("rests on an exception".to_string());
            }
            if i == 8 && row.members.iter().any(|&m| results[m].ammunition_assumed) {
                notes.push(format!(
                    "assumption, not a fact: main-gun ammunition is treated as not limiting (decision 0023); {AMMUNITION_NOT_LIMITING_ROUNDS} is a marker, not an outfit"
                ));
            }
            if i == RADAR {
                let ships: Vec<&str> = row
                    .members
                    .iter()
                    .filter(|&&m| results[m].radar_assumed)
                    .map(|&m| results[m].ship.as_str())
                    .collect();
                if !ships.is_empty() {
                    notes.push(format!("radar assumed none for {}", ships.join(", ")));
                }
            }
            let _ = KIND;
            let estimate = row
                .members
                .iter()
                .find_map(|&m| results[m].figures[i].estimate.as_deref());
            let rule = estimate.unwrap_or_else(|| {
                results[row.members[0]].figures[i]
                    .rule
                    .unwrap_or_else(|| rule_text(i, cols))
            });
            source_rows.push(vec![
                row.id.to_string(),
                (*figure).to_string(),
                cell(i, cols, &row.figures[i]),
                rule.to_string(),
                facts,
                notes.join("; "),
            ]);
        }
    }
    let sources_csv = format!(
        "{sources_text}{}",
        csv_text(
            &["class_id", "figure", "value", "rule", "facts", "note"],
            &source_rows
        )
    );

    // Ships CSV.
    let mut ships_text = String::new();
    for text in [
        "GENERATED by oob-derive. Do not edit.".to_string(),
        format!("The class row of {class_name} each listed ship is given, in list order."),
    ] {
        ships_text.push_str("# ");
        ships_text.push_str(&text);
        ships_text.push('\n');
    }
    let ship_rows: Vec<Vec<String>> = results
        .iter()
        .enumerate()
        .map(|(i, result)| {
            let row = rows
                .iter()
                .find(|r| r.members.contains(&i))
                .expect("every ship is in a class row");
            vec![result.ship.clone(), row.id.to_string(), row.name.clone()]
        })
        .collect();
    let ships_csv = format!(
        "{ships_text}{}",
        csv_text(&["ship", "class_id", "class_name"], &ship_rows)
    );
    let (aircraft_csv, aircraft_sources_csv) = aircraft_files(ctx);
    Ok(Outputs {
        class_csv,
        sources_csv,
        ships_csv,
        aircraft_csv,
        aircraft_sources_csv,
        experience_csv: None,
        experience_unread: Vec::new(),
        search_not_settled: Vec::new(),
        torpedo_not_split,
        estimated: estimated_lines(&estimates),
    })
}

/// The check's count of the estimates (decision 0207, rule 13).
fn estimated_lines(estimates: &[&crate::derive::EstimateUse]) -> Vec<String> {
    if estimates.is_empty() {
        return Vec::new();
    }
    let ships: BTreeSet<&str> = estimates
        .iter()
        .filter_map(|u| u.line.split_once(' ').map(|(ship, _)| ship))
        .collect();
    let n = estimates.len();
    let mut lines = vec![format!(
        "{n} {} estimated (decision 0207), for {}",
        if n == 1 { "attribute" } else { "attributes" },
        ships.into_iter().collect::<Vec<_>>().join(", ")
    )];
    lines.extend(estimates.iter().map(|u| one_line(&u.line)));
    lines
}

fn aircraft_files(ctx: &Context) -> (Option<String>, Option<String>) {
    if ctx.aircraft.is_empty() {
        return (None, None);
    }
    let (csv, sources) = render_aircraft(
        ctx.list,
        ctx.list_name,
        ctx.rules,
        ctx.aircraft,
        ctx.aircraft_used,
        ctx.idle_later,
    );
    (Some(csv), Some(sources))
}

/// The aircraft type file and its sources file.
fn render_aircraft(
    list: &List,
    list_name: &str,
    rules: &Rules,
    aircraft: &[AircraftResult],
    used: &BTreeMap<(usize, String, u64), Row>,
    idle_later: &[String],
) -> (String, String) {
    let output = list.aircraft_output.as_deref().unwrap_or_default();
    let file_name = Path::new(output)
        .file_name()
        .map_or_else(String::new, |n| n.to_string_lossy().into_owned());
    let sources_name = aircraft_sources_name(output);
    let fraction = rules
        .strike_radius_fraction_of_range
        .map_or_else(String::new, |d| d.to_string());
    let sweep = rules
        .search_sweep_width_nm
        .map_or_else(String::new, |d| d.to_string());
    let mut text = String::new();
    let mut line = |t: String| {
        text.push_str("# ");
        text.push_str(&t);
        text.push('\n');
    };
    line("GENERATED by oob-derive from the facts in data/oob. Do not edit.".to_string());
    line(format!(
        "List: {list_name}: {} (as of {})",
        one_line(&list.name),
        list.date_text
    ));
    line(format!("Rules: {}", list.rules));
    line(format!(
        "Companion: {sources_name} (the rule and fact rows behind every figure)"
    ));
    line(
        "Constants, MODEL PARAMETERS AND PLACEHOLDERS, not history (decisions 0020, 8 and 0029):"
            .to_string(),
    );
    line(format!(
        "  gun_shell_kg_per_damage = {}",
        rules.gun_shell_kg_per_damage
    ));
    line(format!(
        "  torpedo_damage_per_warhead_kg = {}",
        rules.torpedo_damage_per_warhead_kg
    ));
    line(format!("  strike_radius_fraction_of_range = {fraction}"));
    line(format!("  search_sweep_width_nm = {sweep}"));
    line("Ranges are whole nautical miles. attack_damage is an exact decimal of up to six places, trailing zeros trimmed. pilot_armour and self_sealing_tanks are 1 for yes and 0 for no.".to_string());
    let own: Vec<(&str, &Row, &str)> = aircraft
        .iter()
        .zip(&list.aircraft)
        .filter_map(|(result, entry)| {
            result
                .radius_choice
                .as_ref()
                .map(|(row, reason)| (entry.type_key.as_str(), row, reason.as_str()))
        })
        .collect();
    if used.is_empty() {
        line("Exceptions used (decision 0020, 3 and 0029, 5): none".to_string());
    } else {
        line("Exceptions used (decision 0020, 3 and 0029, 5):".to_string());
        for ((index, _, _), row) in used.iter() {
            let e = &rules.exceptions[*index];
            line(exception_line(e, row));
        }
    }
    for text in idle_lines(idle_later) {
        line(text);
    }
    if let Some(block) = &rules.search_radius {
        line(format!(
            "Search radius (decision 0075): a type's range in the scouting condition / {}, rounded down; where it cannot be read the strike radius is kept and the type is listed as not settled",
            block.divisor
        ));
        let mut own_used: BTreeMap<(usize, String, u64), &Row> = BTreeMap::new();
        for result in aircraft {
            for (index, row) in &result.search_used {
                own_used.insert((*index, row.file.clone(), row.cite_key().1), row);
            }
        }
        if own_used.is_empty() {
            line("Search radius exceptions used (decision 0075): none".to_string());
        } else {
            line("Search radius exceptions used (decision 0075):".to_string());
            for ((index, _, _), row) in &own_used {
                let e = &block.exceptions[*index];
                line(exception_line(e, row));
            }
        }
        let unsettled: Vec<(&str, &str)> = aircraft
            .iter()
            .zip(&list.aircraft)
            .filter_map(|(result, entry)| {
                result
                    .search_not_settled
                    .as_deref()
                    .map(|why| (entry.type_key.as_str(), why))
            })
            .collect();
        if unsettled.is_empty() {
            line("Search radius not settled (the strike radius is kept): none".to_string());
        } else {
            line("Search radius not settled (the strike radius is kept):".to_string());
            for (entity, why) in unsettled {
                line(format!("  {entity}: {}", one_line(why)));
            }
        }
    }
    if !own.is_empty() {
        line("Radius rows chosen by this list itself, instead of the rules file's rule or exception for the same figure:".to_string());
        for (entity, row, reason) in &own {
            line(format!(
                "  {entity} combat_radius_nm: {} row {} ({}); the list's own choice, reason: {}",
                row.file,
                row.id,
                row.standing,
                one_line(reason)
            ));
        }
    }
    let header: Vec<&str> = [&["id", "name"][..], &AIRCRAFT_FIGURES[..]].concat();
    let mut body = Vec::new();
    let mut source_rows = Vec::new();
    for (n, result) in aircraft.iter().enumerate() {
        let id = u64::from(list.first_aircraft_type_id) + n as u64;
        let mut cells = vec![id.to_string(), result.name.clone()];
        cells.extend(result.figures.iter().map(|f| f.value.clone()));
        body.push(cells);
        for (name, fig) in AIRCRAFT_FIGURES.iter().zip(&result.figures) {
            let facts = fig
                .cites
                .iter()
                .map(|(file, id)| format!("{file}:{id}"))
                .collect::<Vec<_>>()
                .join(" ");
            source_rows.push(vec![
                id.to_string(),
                (*name).to_string(),
                fig.value.clone(),
                fig.rule.clone(),
                facts,
                fig.notes.join("; "),
            ]);
        }
    }
    let csv = format!("{text}{}", csv_text(&header, &body));
    let mut sources_text = String::new();
    for t in [
        "GENERATED by oob-derive. Do not edit.".to_string(),
        format!("For each type of {file_name} and each of its figures: the value as written there, the rule, the fact rows read, and a note."),
        "facts: file:id of each row read, files in data/oob. Constants and fixed values cite no row.".to_string(),
        "The note says which radius rule was used and why, gives the exact value where a figure was rounded down, and says where a figure rests on an exception.".to_string(),
    ] {
        sources_text.push_str("# ");
        sources_text.push_str(&t);
        sources_text.push('\n');
    }
    let sources = format!(
        "{sources_text}{}",
        csv_text(
            &["type_id", "figure", "value", "rule", "facts", "note"],
            &source_rows
        )
    );
    (csv, sources)
}
