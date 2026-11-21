//! Scenario 120: real oilers. The American fleet oiler of 1942 is a class
//! derived from fact rows, and the oilers the record names are in the
//! Coral Sea and in the five days of the Eastern Solomons.
//!
//! Proves decision 0206. The statements are the architect's, written
//! before the code. A mismatch goes back to the architect, and nobody else
//! changes a statement or a number here.
//!
//! # The figures of the class, worked from the rows
//!
//! The Navy's factor is 0.1516 long tons to a barrel of fuel oil
//! (ship_class_facts 14311). A ton of the simulation is a long ton.
//!
//! 1. **Neosho and Platte** (the class as her sisters have it):
//!    - kind auxiliary, and the mark that she fuels others at sea;
//!    - toughness: light displacement 7,256 tons over 25, 290.24;
//!    - maximum speed 18 knots; cruising speed 16.5 knots;
//!    - her own fuel: 15,000 barrels times 0.1516, 2,274 tons;
//!    - endurance 12,000 nautical miles, so 0.1895 tons a mile at her
//!      cruising speed;
//!    - cargo: 101,638 barrels times 0.1516, 15,408.3208 tons;
//!    - no main battery and no torpedo tubes.
//! 2. **Cimarron,** a class of her own: the same but for her own fuel,
//!    14,850 barrels, 2,251.26 tons, and her cargo, 103,233 barrels,
//!    15,650.1228 tons.
//! 3. **Their anti-aircraft figure** is what the list's rule gives each
//!    fit: Neosho, three 3-inch/50 and eight 20 mm with one 5-inch/51;
//!    Cimarron, four 5-inch/38 and eight 20 mm; Platte, the lesser of her
//!    two lines of 1942. The build reports the three figures and the rows
//!    each rests on, and the architect checks them: no figure is set here.
//! 4. **Sims** comes out with every figure her sisters Anderson, Hammann,
//!    Russell and Morris have in the Coral Sea's own class file, and with
//!    no mark.
//! 5. **Every figure names its rows.** The sources file of the new class
//!    file gives, for each figure of each class, the rows it was read
//!    from and the exception or rule that read them.
//! 6. **The lists on file are as they were.** Each derivation list on
//!    file passes its check, and the class files, ships files and sides
//!    files that scenarios on file read are byte for byte what they were.
//!
//! # Statements: the Coral Sea with her oiler
//!
//! The new Coral Sea file is the Coral Sea's own with one force more.
//!
//! 7. **The fuelling group.** Neosho and Sims are one force of the Allied
//!    side, of the mission transport, with no home port, in the hex of
//!    Task Force 17, (-3,5), at the start. Neosho has 2,274 tons of her
//!    own fuel and 15,408.3208 tons of cargo fuel. Sims is full, as her
//!    sisters are, with 453 tons. No order is given to the force by the
//!    scenario.
//! 8. **A destroyer fuels.** With Anderson set to 153 tons, 300 short of
//!    her 453, Task Force 17 lying still, and Task Force 17 given the
//!    order to fuel from Neosho's force before the first night: in that
//!    night Anderson is alongside for one hour and then takes 300 tons on
//!    one line at 250 tons an hour. She has 453 tons, Neosho's cargo is
//!    15,108.3208 tons, and the order ends because every ship is full.
//! 9. **The staff sends a short force to her.** A quarter of Anderson's
//!    453 tons is 113.25. With Anderson at 100 tons and Task Force 17
//!    under a task of the staff's, the staff gives Task Force 17 the
//!    order to fuel from Neosho's force on the first morning, and its
//!    reason names Anderson and Neosho's force. With Anderson at 120 tons
//!    it does not.
//! 10. **She is no lift.** With an order of the Allied commander's to
//!     take a place and no forces named, Neosho's force is not counted
//!     among the forces that could carry the men, and the staff's reason
//!     does not name it.
//! 11. **Sunk, she scores as an auxiliary,** by the points the Coral
//!     Sea's file already gives that kind.
//!
//! # Statements: the five days with their oilers
//!
//! 12. **The fuelling group.** Platte and Cimarron are one force of the
//!     Allied side, of the mission transport, with no home port, stopped
//!     in the hex of Efate, (7,10), at the start. Platte has 2,274 tons
//!     of her own fuel and 15,408.3208 of cargo; Cimarron 2,251.26 and
//!     15,650.1228. No order is given to the force by the scenario.
//! 13. **The meeting place is reachable.** From (7,10) to the meeting
//!     place of 25 August, hex (5,2), is a passage by sea on the map, and
//!     the force has the fuel for it many times over.
//!
//! # Properties
//!
//! - **No game on file moves.** Every scenario and recorded game on file
//!   gives the events and the daily hashes it gave before.
//! - **Nothing is invented.** No figure of an oiler's class is written by
//!   hand: each is derived, and the derivation's check passes on the new
//!   lists.
//! - **The table's new files** differ from the files they stand beside in
//!   the ships added, the class file and the sides file named, and
//!   nothing else.

// The architect's statement lists are list items in the doc comment above,
// which is left as written.
#![allow(clippy::doc_lazy_continuation)]

mod common;

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};
use std::process::Command;

use sim_core::events::{Event, EventKind, FuelOrderEnd};
use sim_core::fixed::Fixed;
use sim_core::hex::Hex;
use sim_core::ids::{ShipClassId, ShipId, TaskForceId};
use sim_core::objectives::{score, tallies};
use sim_core::orders::{OrderKind, TaskForceOrder};
use sim_core::refdata::{ShipClass, ShipKind};
use sim_core::state::{ShipLocation, Side};
use sim_core::time::Pulse;
use sim_data::Scenario;
use sim_staff::{as_staffs, counted_by_a_take, staffs};

// ------------------------------------------------------------ The harness
//
// These scenarios are for play and read the live figures in data/reference
// (the oiler class files have no frozen copy), as scenarios 103 and 105
// load theirs: through `Scenario::load` and `Scenario::from_text`.

fn repo() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("..")
}

fn reference(file: &str) -> PathBuf {
    common::data_dir().join("reference").join(file)
}

fn read(path: &Path) -> String {
    std::fs::read_to_string(path).unwrap_or_else(|e| panic!("{} reads: {e}", path.display()))
}

/// The file of one of the two scenarios: `coral` or not, and its letter.
fn file(coral: bool, letter: char) -> PathBuf {
    let name = if coral {
        format!("coral-sea/02{letter}_coral_sea_oilers.toml")
    } else {
        format!("five-days/02{letter}_eastern_solomons_oilers.toml")
    };
    common::data_dir().join("scenarios").join(name)
}

/// The file the oiler file stands beside.
fn old_file(coral: bool, letter: char) -> PathBuf {
    let name = if coral {
        format!("coral-sea/01{letter}_coral_sea.toml")
    } else {
        format!("five-days/01{letter}_eastern_solomons.toml")
    };
    common::data_dir().join("scenarios").join(name)
}

const LETTERS: [char; 4] = ['a', 'b', 'c', 'd'];

fn load(path: &Path) -> Scenario {
    Scenario::load(path).unwrap_or_else(|e| panic!("{} loads: {e}", path.display()))
}

/// Loads altered text as if it sat where the file does, on the live figures.
fn load_text(text: &str, path: &Path) -> Scenario {
    Scenario::from_text(text, path).expect("the altered scenario loads")
}

fn changed(text: &str, original: &str, replacement: &str) -> String {
    common::replace_once(text, original, replacement)
}

fn fx(text: &str) -> Fixed {
    text.parse().expect("a decimal")
}

fn t(n: i64) -> Fixed {
    Fixed::from_int(n)
}

fn force(raw: u32) -> TaskForceId {
    TaskForceId::new(raw)
}

fn ship(raw: u32) -> ShipId {
    ShipId::new(raw)
}

fn fuel(scenario: &Scenario, raw: u32) -> Fixed {
    scenario.world.umpire().ships[&ship(raw)].fuel_t
}

fn cargo_fuel(scenario: &Scenario, raw: u32) -> Fixed {
    scenario.world.umpire().ships[&ship(raw)].cargo_fuel_t
}

fn events(scenario: &Scenario) -> Vec<Event> {
    scenario.world.events().iter().cloned().collect()
}

/// The class rows of a class file as written: a map from the column name to
/// the text, one for each class id.
fn raw_rows(path: &Path) -> BTreeMap<u32, BTreeMap<String, String>> {
    let mut reader = csv::ReaderBuilder::new()
        .comment(Some(b'#'))
        .trim(csv::Trim::All)
        .from_path(path)
        .expect("the class file opens");
    let header: Vec<String> = reader
        .headers()
        .expect("a header")
        .iter()
        .map(str::to_string)
        .collect();
    reader
        .records()
        .map(|r| {
            let r = r.expect("a row");
            let row: BTreeMap<String, String> = header
                .iter()
                .cloned()
                .zip(r.iter().map(str::to_string))
                .collect();
            (row["id"].parse().expect("an id"), row)
        })
        .collect()
}

/// A class as the engine's own loader reads it from the class file.
fn engine_class(file: &str, id: u32) -> ShipClass {
    let data = sim_data::load_ship_classes(&reference(file)).expect("the class file loads");
    data.ship_classes[&ShipClassId::new(id)].clone()
}

/// The rows of a sources file: the class id, the figure, then the row.
fn source_rows(file: &str) -> Vec<BTreeMap<String, String>> {
    let mut reader = csv::ReaderBuilder::new()
        .comment(Some(b'#'))
        .trim(csv::Trim::All)
        .from_path(reference(file))
        .expect("the sources file opens");
    let header: Vec<String> = reader
        .headers()
        .expect("a header")
        .iter()
        .map(str::to_string)
        .collect();
    reader
        .records()
        .map(|r| {
            header
                .iter()
                .cloned()
                .zip(r.expect("a row").iter().map(str::to_string))
                .collect()
        })
        .collect()
}

const CORAL_CLASSES: &str = "ship_classes_coral_sea_oilers.csv";
const FIVE_CLASSES: &str = "ship_classes_five_days_oilers.csv";
const CORAL_SOURCES: &str = "ship_classes_coral_sea_oilers.sources.csv";
const FIVE_SOURCES: &str = "ship_classes_five_days_oilers.sources.csv";

// ------------------------------------------------- Statements 1 and 2: classes

/// The class of the hand calculation's statement 1 (Neosho and Platte) or,
/// with her own fuel and cargo, statement 2 (Cimarron).
fn the_oiler_class(c: &ShipClass, own_fuel: &str, own_fuel_bbl: i64, cargo: &str) {
    assert_eq!(c.kind, ShipKind::Auxiliary, "kind auxiliary");
    assert!(c.fuels_at_sea, "the mark that she fuels others at sea");
    // Light displacement 7,256 over 25.
    assert_eq!(c.toughness, fx("290.24"));
    assert_eq!(c.toughness, t(7256) / t(25), "7,256 / 25");
    assert_eq!(c.max_speed_kn, t(18));
    assert_eq!(c.cruise_speed_kn, fx("16.5"));
    // Her own fuel: barrels times the Navy's 0.1516.
    assert_eq!(c.fuel_capacity_t, fx(own_fuel));
    assert_eq!(
        t(own_fuel_bbl) * fx("0.1516"),
        fx(own_fuel),
        "{own_fuel_bbl} barrels x 0.1516"
    );
    assert_eq!(c.endurance_nm, t(12000));
    // Cargo: barrels times 0.1516.
    assert_eq!(c.cargo_capacity_t, fx(cargo));
    // No main battery and no torpedo tubes, nor any other surface weapon.
    assert_eq!(c.main_guns, 0);
    assert!(c.gun_rounds_per_min.is_zero());
    assert_eq!(c.gun_rounds_per_gun, 0);
    assert!(c.gun_damage.is_zero());
    assert_eq!(c.torpedo_tubes, 0);
    assert_eq!(c.torpedo_tubes_one_side, 0);
    assert!(c.torpedo_damage.is_zero());
}

/// What a mile costs her at her cruising speed, as the engine burns it:
/// the distance times the fuel capacity over the endurance
/// (sim-core `movement`, `distance_nm.mul_div(fuel_capacity_t, endurance_nm)`).
fn burn_a_mile(c: &ShipClass) -> Fixed {
    Fixed::ONE.mul_div(c.fuel_capacity_t, c.endurance_nm)
}

#[test]
fn statement_1_neosho_and_platte_are_the_class_as_her_sisters_have_it() {
    let neosho = engine_class(CORAL_CLASSES, 4030);
    the_oiler_class(&neosho, "2274", 15000, "15408.3208");
    let platte = engine_class(FIVE_CLASSES, 2030);
    the_oiler_class(&platte, "2274", 15000, "15408.3208");
    for c in [&neosho, &platte] {
        assert_eq!(burn_a_mile(c), fx("0.1895"), "2,274 / 12,000 tons a mile");
        assert_eq!(t(101638) * fx("0.1516"), fx("15408.3208"), "cargo");
    }
    // The class file as written says the same, column by column.
    for (file, id) in [(CORAL_CLASSES, 4030), (FIVE_CLASSES, 2030)] {
        let row = &raw_rows(&reference(file))[&id];
        assert_eq!(row["kind"], "auxiliary");
        assert_eq!(row["fuels_at_sea"], "1");
        assert_eq!(row["toughness"], "290.24");
        assert_eq!(row["max_speed_kn"], "18");
        assert_eq!(row["cruise_speed_kn"], "16.5");
        assert_eq!(row["fuel_capacity_t"], "2274");
        assert_eq!(row["endurance_nm"], "12000");
        assert_eq!(row["cargo_capacity_t"], "15408.3208");
        for column in [
            "main_guns",
            "gun_rounds_per_min",
            "gun_rounds_per_gun",
            "gun_damage",
            "torpedo_tubes",
            "torpedo_tubes_one_side",
            "torpedo_damage",
        ] {
            assert_eq!(row[column], "0", "{column}");
        }
    }
}

#[test]
fn statement_2_cimarron_is_a_class_of_her_own() {
    let cimarron = engine_class(FIVE_CLASSES, 2031);
    the_oiler_class(&cimarron, "2251.26", 14850, "15650.1228");
    assert_eq!(t(103233) * fx("0.1516"), fx("15650.1228"), "cargo");
    let platte = engine_class(FIVE_CLASSES, 2030);
    assert_ne!(cimarron.fuel_capacity_t, platte.fuel_capacity_t);
    assert_ne!(cimarron.cargo_capacity_t, platte.cargo_capacity_t);
}

// ------------------------------------------------- Statement 3: the guns

/// Statement 3 sets no figure: the build reports the three anti-aircraft
/// figures and the rows each rests on, and the architect checks them. The
/// figures asserted here are the architect's of Nov 21, 2026, given by
/// message: Neosho 5 (fits 14305, 14306), Platte 5 (14310, 14311), Cimarron
/// 6 (14315, 14320). The test asserts that the class files and their sources
/// say so, and nothing it computes.
#[test]
fn statement_3_the_anti_aircraft_figures_and_the_rows_they_rest_on() {
    let expected: [(&str, &str, u32, u32, [&str; 2]); 3] = [
        (
            CORAL_CLASSES,
            CORAL_SOURCES,
            4030,
            5,
            ["fits.csv:14305", "fits.csv:14306"],
        ),
        (
            FIVE_CLASSES,
            FIVE_SOURCES,
            2030,
            5,
            ["fits.csv:14310", "fits.csv:14311"],
        ),
        (
            FIVE_CLASSES,
            FIVE_SOURCES,
            2031,
            6,
            ["fits.csv:14315", "fits.csv:14320"],
        ),
    ];
    for (classes, sources, id, figure, rows) in expected {
        assert_eq!(
            engine_class(classes, id).anti_aircraft,
            figure,
            "class {id}"
        );
        let line = source_rows(sources)
            .into_iter()
            .find(|r| r["class_id"] == id.to_string() && r["figure"] == "anti_aircraft")
            .expect("an anti_aircraft line");
        assert_eq!(line["value"], figure.to_string());
        let cited: BTreeSet<&str> = line["facts"].split_whitespace().collect();
        assert_eq!(cited, BTreeSet::from(rows), "class {id} cites its fits");
    }
}

// ------------------------------------------------- Statement 4: Sims

#[test]
fn statement_4_sims_has_every_figure_her_sisters_have_and_no_mark() {
    let sims = &raw_rows(&reference(CORAL_CLASSES))[&4031];
    let sisters = &raw_rows(&reference("ship_classes_coral_sea.csv"))[&4008];
    let columns = [
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
        "aircraft_capacity",
        "anti_aircraft",
    ];
    for column in columns {
        assert_eq!(sims[column], sisters[column], "{column}");
    }
    assert_eq!(sims["fuels_at_sea"], "0", "no mark");
    assert!(
        !sisters.contains_key("fuels_at_sea"),
        "the old file has none"
    );
    assert!(!engine_class(CORAL_CLASSES, 4031).fuels_at_sea);
    assert!(!engine_class("ship_classes_coral_sea.csv", 4008).fuels_at_sea);
}

// ------------------------------------------------- Statement 5: the rows

/// The rows the header of a class file lists under "Exceptions used", as
/// `file:id`.
fn exception_rows(classes: &str) -> BTreeSet<String> {
    let mut inside = false;
    let mut rows = BTreeSet::new();
    for line in read(&reference(classes)).lines() {
        if line.starts_with("# Exceptions used") {
            inside = true;
        } else if inside && line.starts_with("#   ") {
            let (_, rest) = line.split_once(": ").expect("an exception line");
            let mut words = rest.split_whitespace();
            let (file, row_word, id) = (words.next(), words.next(), words.next());
            assert_eq!(row_word, Some("row"), "{line}");
            rows.insert(format!("{}:{}", file.expect("a file"), id.expect("an id")));
        } else if inside {
            break;
        }
    }
    rows
}

/// Statement 5: every figure names its rows and the rule that read them.
/// Exactly these figures cite no row, each for a reason the file gives: the
/// surface weapons, which are 0 by kind (an auxiliary) or an assumption of
/// the scenario (the destroyer's), the destroyer's cargo, fixed at 0, her
/// radar, assumed none, and the aircraft capacity, none carried.
#[test]
fn statement_5_every_figure_names_its_rows_and_its_rule() {
    for (classes, sources) in [(CORAL_CLASSES, CORAL_SOURCES), (FIVE_CLASSES, FIVE_SOURCES)] {
        let rows = source_rows(sources);
        let ids: BTreeSet<String> = raw_rows(&reference(classes))
            .keys()
            .map(u32::to_string)
            .collect();
        let class_figures = raw_rows(&reference(classes));
        let exceptions = exception_rows(classes);
        assert!(!exceptions.is_empty(), "{classes} lists its exceptions");
        // One line for each figure of each class, from kind to the mark.
        let columns: Vec<String> = {
            let first = class_figures.values().next().expect("a class");
            let order = [
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
                "aircraft_capacity",
                "anti_aircraft",
                "fuels_at_sea",
            ];
            order
                .iter()
                .filter(|c| first.contains_key(**c))
                .map(|c| (*c).to_string())
                .collect()
        };
        assert_eq!(columns.len(), 18);
        assert_eq!(rows.len(), ids.len() * columns.len(), "a line each");
        for id in &ids {
            for column in &columns {
                let line = rows
                    .iter()
                    .find(|r| &r["class_id"] == id && &r["figure"] == column)
                    .unwrap_or_else(|| panic!("{id} {column} has a line"));
                let who = format!("{classes} class {id} {column}");
                assert_eq!(
                    line["value"],
                    class_figures[&id.parse::<u32>().unwrap()][column],
                    "{who}: the value is the class file's"
                );
                assert!(!line["rule"].is_empty(), "{who}: a rule");
                let facts: Vec<&str> = line["facts"].split_whitespace().collect();
                if facts.is_empty() {
                    let by_kind = line["rule"].starts_with("0 by kind");
                    let assumed = line["note"].contains("assumption of the scenario")
                        && line["note"].contains("surface weapons not modelled");
                    let cargo_fixed = column == "cargo_capacity_t"
                        && line["rule"].starts_with("fixed at 0: warships carry no cargo");
                    let radar = column == "radar" && line["note"].starts_with("radar assumed none");
                    let none_carried =
                        column == "aircraft_capacity" && line["note"].contains("none carried");
                    let surface = [
                        "main_guns",
                        "gun_rounds_per_min",
                        "gun_rounds_per_gun",
                        "gun_damage",
                        "torpedo_tubes",
                        "torpedo_tubes_one_side",
                        "torpedo_damage",
                    ]
                    .contains(&column.as_str());
                    assert!(
                        ((by_kind || assumed) && surface) || cargo_fixed || radar || none_carried,
                        "{who} cites no row and is not one of those allowed"
                    );
                }
                // A figure that rests on a row an exception reads says so.
                if facts.iter().any(|f| exceptions.contains(*f)) {
                    assert!(
                        line["note"].contains("exception")
                            || line["note"].contains("read from a row"),
                        "{who} rests on an exception and its note says so: {}",
                        line["note"]
                    );
                }
            }
        }
    }
}

// ------------------------------------------------- Statement 6: as they were

fn cargo() -> Command {
    let mut command = Command::new(std::env::var("CARGO").unwrap_or_else(|_| "cargo".to_string()));
    command.current_dir(repo());
    command
}

fn derive_check(list: &str) {
    let out = cargo()
        .args(["run", "--quiet", "-p", "oob-derive", "--"])
        .arg(format!("data/derivation/{list}"))
        .arg("--check")
        .output()
        .expect("the derivation runs");
    let said = format!(
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );
    assert!(out.status.success(), "{list}: {said}");
}

/// Each derivation list on file still derives exactly its committed files.
#[test]
fn statement_6_each_list_on_file_derives_its_files() {
    let mut lists: Vec<String> = std::fs::read_dir(common::data_dir().join("derivation"))
        .expect("the lists")
        .map(|e| {
            e.expect("an entry")
                .file_name()
                .to_string_lossy()
                .into_owned()
        })
        .filter(|n| {
            n.ends_with(".toml")
                && n != "rules.toml"
                && n != "coral_sea_oilers.toml"
                && n != "five_days_oilers.toml"
        })
        .collect();
    lists.sort();
    assert!(lists.len() >= 6, "{lists:?}");
    for list in &lists {
        derive_check(list);
    }
}

/// Nothing is invented: the check passes on the two new lists.
#[test]
fn property_nothing_is_invented_the_new_lists_pass_their_check() {
    derive_check("coral_sea_oilers.toml");
    derive_check("five_days_oilers.toml");
}

/// The class, ships and sides files that scenarios on file read are as they
/// were. That each is byte for byte what it was before decision 0206 was
/// checked when it was built, against commit ae3972c3; a test cannot keep
/// that commit (the checks fetch two commits only), so this holds what must
/// stay so: no scenario on file names a file of the oilers, and no class,
/// ships or sides file on file has the oilers' classes or the column of the
/// mark, but scenario 115's placeholder file, which has had it since stage A.
#[test]
fn statement_6_the_class_ships_and_sides_files_are_as_they_were() {
    let new_ids = ["4030", "4031", "2030", "2031"];
    let mut names: Vec<String> = std::fs::read_dir(common::data_dir().join("reference"))
        .expect("the reference folder")
        .map(|e| {
            e.expect("an entry")
                .file_name()
                .to_string_lossy()
                .into_owned()
        })
        .filter(|n| {
            let class = n.starts_with("ship_classes") && n.ends_with(".csv");
            let sides = n.starts_with("class_sides_") && n.ends_with(".csv");
            let experience = n.starts_with("ship_experience_") && n.ends_with(".ships.csv");
            (class || sides || experience) && !n.contains("_oilers")
        })
        .collect();
    names.sort();
    assert!(names.len() >= 30, "{} files", names.len());
    for name in &names {
        let text = read(&reference(name));
        let header = text
            .lines()
            .find(|l| !l.starts_with('#'))
            .unwrap_or_default();
        if name != "ship_classes_placeholder_scenario_115.csv" {
            assert!(
                !header.split(',').any(|c| c == "fuels_at_sea"),
                "{name} has the column of the mark"
            );
        }
        for line in text.lines().filter(|l| !l.starts_with('#')) {
            let cells: Vec<&str> = line.split(',').collect();
            let id = if name.starts_with("class_sides_") {
                cells.get(1)
            } else if name.ends_with(".sources.csv") || !name.ends_with(".ships.csv") {
                cells.first()
            } else {
                cells.get(1)
            };
            assert!(
                !id.is_some_and(|id| new_ids.contains(id)),
                "{name} has a class of the oilers: {line}"
            );
        }
    }
    // No scenario on file names a file of the oilers.
    let mut stack = vec![common::data_dir().join("scenarios")];
    while let Some(dir) = stack.pop() {
        for entry in std::fs::read_dir(&dir).expect("a folder") {
            let path = entry.expect("an entry").path();
            if path.is_dir() {
                stack.push(path);
                continue;
            }
            let file_name = path.file_name().unwrap_or_default().to_string_lossy();
            if !file_name.ends_with(".toml") || file_name.contains("_oilers") {
                continue;
            }
            let text = read(&path);
            assert!(
                !text.contains("_oilers.csv"),
                "{} names a file of the oilers",
                path.display()
            );
        }
    }
}

// ------------------------------------------- The fuelling groups (7 and 12)

/// A group as a statement has it: of the Allied side, of the mission
/// transport, with no home port, stopped (no route, nowhere steaming to) in
/// `hex`, with these ships: id, class, fuel and cargo fuel.
fn the_group(scenario: &Scenario, hex: Hex, ships: &[(u32, u32, &str, &str)]) {
    let truth = scenario.world.umpire();
    let group = &truth.task_forces[&force(6)];
    assert_eq!(group.side, Side::Allied);
    assert_eq!(group.mission, sim_core::orders::Mission::Transport);
    assert_eq!(group.home_port, None, "no home port");
    assert_eq!(group.position(), hex);
    assert!(group.route.waypoints.is_empty(), "no route");
    assert_eq!(group.toward_hex, None, "steaming nowhere");
    assert_eq!(
        group.ships,
        ships.iter().map(|s| ship(s.0)).collect::<BTreeSet<_>>(),
        "its ships"
    );
    for (id, class, fuel_t, cargo) in ships {
        let record = &truth.ships[&ship(*id)];
        assert_eq!(record.class, ShipClassId::new(*class), "ship {id}");
        assert_eq!(record.side, Side::Allied);
        assert_eq!(record.location, ShipLocation::TaskForce(force(6)));
        assert_eq!(record.fuel_t, fx(fuel_t), "ship {id}'s own fuel");
        assert_eq!(record.cargo_fuel_t, fx(cargo), "ship {id}'s cargo fuel");
    }
}

/// The values after `key` in the lines of a table that are not comments,
/// as whole numbers.
fn numbers_of(block: &[String], keys: &[&str]) -> Vec<i64> {
    let mut out = Vec::new();
    for line in block {
        let Some((key, value)) = line.split_once('=') else {
            continue;
        };
        if keys.contains(&key.trim()) {
            let mut digits = String::new();
            for c in value.chars().chain(std::iter::once(' ')) {
                if c.is_ascii_digit() {
                    digits.push(c);
                } else if !digits.is_empty() {
                    out.push(digits.parse().expect("a number"));
                    digits.clear();
                }
            }
        }
    }
    out
}

/// The tables of a scenario's text, comments and blank lines taken away: the
/// first is what comes before any table. Each other begins with its header.
fn tables(text: &str) -> Vec<Vec<String>> {
    let mut out = vec![Vec::new()];
    for line in text.lines() {
        let line = line.trim_end();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        if line.starts_with('[') {
            out.push(Vec::new());
        }
        out.last_mut().expect("a table").push(line.to_string());
    }
    out
}

/// No order and no command of the file names the group or its ships.
fn nobody_orders_the_group(coral: bool, group_ships: [i64; 2]) {
    for letter in LETTERS {
        let text = read(&file(coral, letter));
        for table in tables(&text) {
            let header = table.first().map(String::as_str);
            if header != Some("[[order]]") && header != Some("[[command]]") {
                continue;
            }
            let forces = numbers_of(&table, &["task_force", "task_forces", "giver"]);
            assert!(!forces.contains(&6), "file {letter}: {table:?}");
            let ships = numbers_of(&table, &["ship", "ships"]);
            assert!(
                !ships.contains(&group_ships[0]) && !ships.contains(&group_ships[1]),
                "file {letter}: {table:?}"
            );
        }
    }
}

#[test]
fn statement_7_the_fuelling_group_of_the_coral_sea() {
    let scenario = load(&file(true, 'd'));
    // In the hex of Task Force 17 (task force 1), (-3,5).
    let task_force_17 = &scenario.world.umpire().task_forces[&force(1)];
    assert_eq!(task_force_17.name, "Task Force 17");
    assert_eq!(task_force_17.position(), Hex::new(-3, 5));
    the_group(
        &scenario,
        Hex::new(-3, 5),
        &[(54, 4030, "2274", "15408.3208"), (55, 4031, "453", "0")],
    );
    // Sims is full, as her sisters, the ships of class 4008, are.
    let sisters: Vec<u32> = scenario
        .world
        .umpire()
        .ships
        .values()
        .filter(|s| s.class == ShipClassId::new(4008))
        .map(|s| s.id.raw())
        .collect();
    assert!(sisters.len() >= 4, "Anderson, Hammann, Russell, Morris");
    for id in sisters {
        assert_eq!(fuel(&scenario, id), fuel(&scenario, 55), "ship {id}");
    }
    assert_eq!(fuel(&scenario, 14), t(453), "Anderson");
    // The scenario gives the force no order.
    nobody_orders_the_group(true, [54, 55]);
}

#[test]
fn statement_12_the_fuelling_group_of_the_five_days() {
    let scenario = load(&file(false, 'd'));
    the_group(
        &scenario,
        Hex::new(7, 10),
        &[
            (44, 2030, "2274", "15408.3208"),
            (45, 2031, "2251.26", "15650.1228"),
        ],
    );
    nobody_orders_the_group(false, [44, 45]);
}

/// Statement 13: the passage by sea from (7,10) to (5,2), at the length the
/// game charges for it, and the fuel it takes at her cruising speed.
#[test]
fn statement_13_the_meeting_place_is_reachable_with_fuel_many_times_over() {
    let scenario = load(&file(false, 'd'));
    let map = scenario.world.map();
    let (from, to) = (Hex::new(7, 10), Hex::new(5, 2));
    let path = map.sea_path(from, to).expect("a passage by sea");
    assert!(!path.is_empty());
    assert_eq!(path.last(), Some(&to));
    let field = map.sea_distances_to(to);
    let mut here = from;
    let mut nm = 0u32;
    for next in &path {
        nm += map.step_charge_nm(&field, here, *next, to);
        here = *next;
    }
    assert!(nm > 0);
    for (class, own_fuel) in [(2030, "2274"), (2031, "2251.26")] {
        let c = engine_class(FIVE_CLASSES, class);
        let needed = Fixed::from_int(i64::from(nm)) * burn_a_mile(&c);
        assert!(needed > Fixed::ZERO);
        // Many times over: at least three.
        assert!(
            needed * t(3) <= fx(own_fuel),
            "class {class}: {nm} nm takes {needed} t of {own_fuel}"
        );
    }
    // Platte, as the statement works it: distance x 0.1895.
    let platte = engine_class(FIVE_CLASSES, 2030);
    assert_eq!(
        Fixed::from_int(i64::from(nm)) * fx("0.1895"),
        Fixed::from_int(i64::from(nm)) * burn_a_mile(&platte)
    );
    eprintln!("(7,10) to (5,2): {} hexes, {nm} nm", path.len());
}

// ------------------------------------------- Statement 8: a destroyer fuels

const ANDERSON: &str = "name = \"USS Anderson\"\nclass = 4008\nside = \"allied\"\nfuel_t = 453";

fn anderson_at(text: &str, tons: i64) -> String {
    changed(
        text,
        ANDERSON,
        &format!("name = \"USS Anderson\"\nclass = 4008\nside = \"allied\"\nfuel_t = {tons}"),
    )
}

const FUEL_FROM_6: &str =
    "\n[[order]]\nday = 1\nside = \"allied\"\ntask_force = 1\nkind = \"fuel_from\"\ngiver = 6\n";

/// One ship fuelled at sea, as the log has it.
#[derive(Debug)]
struct Fuelled {
    day: u32,
    pulse: Pulse,
    ship: u32,
    oiler: u32,
    tons: Fixed,
    fuel_t: Fixed,
    oiler_left: Fixed,
    factors: Vec<(&'static str, Fixed)>,
}

fn fuelled(log: &[Event]) -> Vec<Fuelled> {
    log.iter()
        .filter_map(|e| match &e.kind {
            EventKind::ShipFuelledAtSea {
                ship,
                oiler,
                tons,
                fuel_t,
                oiler_cargo_fuel_t,
                ..
            } => Some(Fuelled {
                day: e.day,
                pulse: e.pulse.expect("in a pulse"),
                ship: ship.raw(),
                oiler: oiler.raw(),
                tons: *tons,
                fuel_t: *fuel_t,
                oiler_left: *oiler_cargo_fuel_t,
                factors: e
                    .breakdown
                    .as_ref()
                    .map(|b| b.factors.iter().map(|f| (f.name, f.value)).collect())
                    .unwrap_or_default(),
            }),
            _ => None,
        })
        .collect()
}

fn ended(log: &[Event]) -> Vec<(u32, Option<Pulse>, FuelOrderEnd)> {
    log.iter()
        .filter_map(|e| match &e.kind {
            EventKind::FuelOrderEnded { reason, .. } => Some((e.day, e.pulse, *reason)),
            _ => None,
        })
        .collect()
}

#[test]
fn statement_8_a_destroyer_fuels_in_the_first_night() {
    let path = file(true, 'd');
    let text = anderson_at(&read(&path), 153) + FUEL_FROM_6;
    let mut scenario = load_text(&text, &path);
    // Task Force 17 lies still: no route, and none given by the file.
    {
        let tf = &scenario.world.umpire().task_forces[&force(1)];
        assert!(tf.route.waypoints.is_empty(), "no route at the start");
        assert_eq!(tf.position(), Hex::new(-3, 5));
    }
    assert_eq!(fuel(&scenario, 14), t(153), "300 short of her 453");
    scenario.next_day().expect("day 1 resolves");
    let log = events(&scenario);
    // 02d has no order: nothing moved Task Force 17.
    {
        let tf = &scenario.world.umpire().task_forces[&force(1)];
        assert!(tf.route.waypoints.is_empty(), "no route after day 1");
        assert_eq!(tf.position(), Hex::new(-3, 5), "lying still");
    }
    let night: Vec<_> = fuelled(&log)
        .into_iter()
        .filter(|f| f.day == 1 && f.pulse == Pulse::Night)
        .collect();
    assert_eq!(night.len(), 1, "one ship fuelled, in the night: {night:?}");
    let one = &night[0];
    assert_eq!((one.ship, one.oiler), (14, 54), "Anderson from Neosho");
    assert_eq!(one.tons, t(300));
    assert_eq!(one.fuel_t, t(453), "Anderson has 453 tons");
    assert_eq!(one.oiler_left, fx("15108.3208"), "Neosho's cargo");
    let factors = &one.factors;
    let factor = |name: &str| factors.iter().find(|f| f.0 == name).map(|f| f.1);
    assert_eq!(factor("alongside_hours"), Some(t(1)), "alongside one hour");
    assert_eq!(factor("lines"), Some(t(1)), "on one line");
    assert_eq!(factor("rate_t_per_h"), Some(t(250)), "250 tons an hour");
    assert_eq!(factor("lacked_t"), Some(t(300)));
    assert_eq!(factor("start_hours"), Some(Fixed::ZERO), "at once");
    eprintln!("statement 8 breakdown: {factors:?}");
    assert_eq!(fuel(&scenario, 14), t(453));
    assert_eq!(cargo_fuel(&scenario, 54), fx("15108.3208"));
    assert_eq!(
        ended(&log),
        vec![(1, Some(Pulse::Night), FuelOrderEnd::Full)],
        "the order ends because every ship is full"
    );
}

// ------------------------------------------- The staff (9 and 10)

/// Each side's staff asked each morning, one day.
fn staffed_day_1(text: &str, path: &Path) -> Scenario {
    let mut scenario = load_text(text, path);
    let held = staffs();
    scenario
        .next_day_staffed(as_staffs(&held))
        .expect("day 1 resolves");
    scenario
}

fn reasons(log: &[Event], day: u32) -> Vec<String> {
    log.iter()
        .filter(|e| e.day == day)
        .filter_map(|e| match &e.kind {
            EventKind::StaffReason { reason, .. } => Some(reason.worked.clone()),
            _ => None,
        })
        .collect()
}

/// The orders the staff gave, with the day, the force and the order.
fn staff_orders(
    log: &[Event],
    world: &sim_core::world::World,
) -> Vec<(u32, TaskForceId, TaskForceOrder)> {
    log.iter()
        .filter_map(|e| match &e.kind {
            EventKind::StaffReason {
                order: Some(id), ..
            } => world.order(*id).and_then(|o| match &o.kind {
                OrderKind::TaskForce { target, order } => Some((e.day, *target, order.clone())),
                _ => None,
            }),
            _ => None,
        })
        .collect()
}

#[test]
fn statement_9_the_staff_sends_a_short_force_to_her() {
    // 02c gives the Allied side its orders, task force 1 an operate_carriers
    // intent: a task of the staff's.
    let path = file(true, 'c');
    let text = read(&path);
    // At 100 tons, below 113.25, a quarter of her 453.
    let scenario = staffed_day_1(&anderson_at(&text, 100), &path);
    let log = events(&scenario);
    let orders = staff_orders(&log, &scenario.world);
    assert!(
        orders.iter().any(|(day, target, order)| *day == 1
            && *target == force(1)
            && *order == TaskForceOrder::FuelFrom(force(6))),
        "the staff gives task force 1 the order to fuel from task force 6: {orders:?}"
    );
    let reason = log
        .iter()
        .find_map(|e| match &e.kind {
            EventKind::StaffReason {
                order: Some(id),
                reason,
                ..
            } if scenario.world.order(*id).is_some_and(|o| {
                matches!(
                    o.kind,
                    OrderKind::TaskForce {
                        order: TaskForceOrder::FuelFrom(_),
                        ..
                    }
                )
            }) =>
            {
                Some(reason.worked.clone())
            }
            _ => None,
        })
        .expect("its reason");
    eprintln!("statement 9 reason: {reason}");
    assert!(reason.contains("ship:14"), "names Anderson: {reason}");
    assert!(
        reason.contains("task_force:6"),
        "names the oiler's force: {reason}"
    );
    assert!(reason.contains("task_force:1"), "and the force: {reason}");

    // At 120 tons, above the quarter, it does not.
    let scenario = staffed_day_1(&anderson_at(&text, 120), &path);
    let log = events(&scenario);
    let orders = staff_orders(&log, &scenario.world);
    assert!(
        orders
            .iter()
            .all(|(_, _, order)| !matches!(order, TaskForceOrder::FuelFrom(_))),
        "at 120 t no order to fuel: {orders:?}"
    );
    assert!(
        reasons(&log, 1)
            .iter()
            .all(|r| !(r.contains("ship:14") && r.contains("below a quarter"))),
        "and no report of her below a quarter"
    );
}

/// The Allied commander's order to take Rabaul, with no forces named.
const TAKE_RABAUL: &str = "\n[[command]]\nday = 1\nside = \"allied\"\nkind = \"set_intent\"\nintent = 1\nintent_kind = \"take_place\"\nhex = [-5, -13]\n";

fn counted_forces(scenario: &Scenario) -> Vec<u32> {
    let view = scenario.world.side_view(Side::Allied);
    counted_by_a_take(&view, None)
        .task_forces
        .iter()
        .map(|f| f.id.raw())
        .collect()
}

#[test]
fn statement_10_she_is_no_lift() {
    let path = file(true, 'd');
    let text = read(&path) + TAKE_RABAUL;
    let scenario = load_text(&text, &path);
    let counted = counted_forces(&scenario);
    eprintln!("statement 10: a take with no forces named counts {counted:?}");
    assert!(
        !counted.contains(&6),
        "Neosho's force is not counted among the forces that could carry the men: {counted:?}"
    );
}

/// A force of Neosho alone: Sims moved to the support group, which is in the
/// same hex. The force has a ship with the mark, so it is not counted.
#[test]
fn statement_10_a_force_of_neosho_alone_is_not_counted() {
    let path = file(true, 'd');
    let text = changed(
        &(read(&path) + TAKE_RABAUL),
        "name = \"USS Sims\"\nclass = 4031\nside = \"allied\"\nfuel_t = 453\nexperience = 2\ntask_force = 6",
        "name = \"USS Sims\"\nclass = 4031\nside = \"allied\"\nfuel_t = 453\nexperience = 2\ntask_force = 2",
    );
    let scenario = load_text(&text, &path);
    let truth = scenario.world.umpire();
    assert_eq!(
        truth.task_forces[&force(6)].ships,
        BTreeSet::from([ship(54)]),
        "Neosho alone"
    );
    let counted = counted_forces(&scenario);
    assert!(!counted.contains(&6), "not counted: {counted:?}");
}

/// The same rule, the other way: a transport force with no ship with the
/// mark is counted as before. The Japanese invasion force, task force 5, on
/// the transport mission, is counted by the Japanese side's take with no
/// forces named.
#[test]
fn statement_10_a_transport_force_with_no_oiler_is_counted_as_before() {
    let scenario = load(&file(true, 'd'));
    let view = scenario.world.side_view(Side::Japanese);
    let counted: Vec<u32> = counted_by_a_take(&view, None)
        .task_forces
        .iter()
        .map(|f| f.id.raw())
        .collect();
    assert_eq!(counted, vec![5], "the invasion force alone");
}

/// A force the forces limit names is the take's to use though it has an
/// oiler: the commander who wants her to carry men names her force.
#[test]
fn statement_10_a_force_named_in_the_forces_limit_is_used_though_it_has_an_oiler() {
    let scenario = load(&file(true, 'd'));
    let view = scenario.world.side_view(Side::Allied);
    let limit = sim_core::command::ForcesLimit {
        ships: BTreeSet::new(),
        task_forces: BTreeSet::from([force(6)]),
        air_groups: BTreeSet::new(),
        source: None,
        land_units: BTreeSet::new(),
    };
    let counted: Vec<u32> = counted_by_a_take(&view, Some(&limit))
        .task_forces
        .iter()
        .map(|f| f.id.raw())
        .collect();
    assert_eq!(counted, vec![6], "named, so counted");
}

// ------------------------------------------- Statement 11: sunk

#[test]
fn statement_11_sunk_she_scores_as_an_auxiliary() {
    let scenario = load(&file(true, 'd'));
    let world = &scenario.world;
    // Her class is an auxiliary, and the file's points for a ship sunk by
    // kind give that kind 1 point.
    assert_eq!(engine_class(CORAL_CLASSES, 4030).kind, ShipKind::Auxiliary);
    let objectives = world.objectives().expect("the file has objectives");
    let mut truth = world.umpire().clone();
    let baseline = score(
        objectives,
        &tallies(&truth, world.events(), world.ref_data()),
    );
    assert_eq!(
        baseline.japanese.total,
        Fixed::ZERO,
        "nothing sunk, no points"
    );
    // Neosho sunk.
    truth.ships.get_mut(&ship(54)).expect("Neosho").location = ShipLocation::Sunk;
    let tallied = tallies(&truth, world.events(), world.ref_data());
    assert_eq!(
        tallied[&Side::Japanese].sunk,
        BTreeMap::from([(ShipKind::Auxiliary, 1)]),
        "the Japanese side has one auxiliary sunk"
    );
    assert!(tallied[&Side::Allied].sunk.is_empty());
    let result = score(objectives, &tallied);
    // The file's ships_sunk points: auxiliary = 1.
    assert_eq!(result.japanese.total, t(1), "the points the kind has");
    assert_eq!(result.allied.total, Fixed::ZERO);
    // The points are the file's: the line is in both sides' tables.
    let text = read(&file(true, 'd'));
    assert_eq!(
        text.matches("points = { carrier = 10, battleship = 8, cruiser = 4, destroyer = 1, transport = 2, auxiliary = 1 }")
            .count(),
        2,
        "ships_sunk gives an auxiliary 1 point on each side"
    );
}

// ------------------------------------------- Properties

/// The scripts that make the new files, asked to check, say they are what
/// each makes: the base and the four files of each.
#[test]
fn the_scripts_say_the_files_are_what_they_make() {
    for script in ["coral-sea", "five-days"] {
        let out = Command::new("python3")
            .arg(repo().join(format!("scripts/{script}/make_oiler_files.py")))
            .arg(repo())
            .arg("--check")
            .output()
            .expect("python3 runs the script");
        let said = String::from_utf8_lossy(&out.stdout);
        assert!(
            out.status.success(),
            "{said}{}",
            String::from_utf8_lossy(&out.stderr)
        );
        assert_eq!(said.matches(": as made").count(), 5, "{said}");
    }
}

/// The lines of the `ship_classes = [ ... ]` list of a file's first table.
fn class_list(preamble: &[String]) -> Vec<String> {
    let start = preamble
        .iter()
        .position(|l| l.starts_with("ship_classes = ["))
        .expect("the list of class files");
    let end = start
        + preamble[start..]
            .iter()
            .position(|l| l.trim() == "]")
            .expect("its end");
    preamble[start + 1..end]
        .iter()
        .map(|l| l.trim().to_string())
        .collect()
}

/// Property: the new files differ from the files they stand beside only in
/// the ships added, the class file and the sides file named, and the name.
/// Taken away the name, the class and sides files, the group's force and its
/// ships, the two tables are equal; and the new list of class files is the
/// old with the one oiler file inserted.
#[test]
fn property_the_new_files_differ_only_in_the_group_the_class_file_and_the_sides_file() {
    for (coral, group_ships, oiler_file) in [
        (
            true,
            [54, 55],
            "\"../../reference/ship_classes_coral_sea_oilers.csv\",",
        ),
        (
            false,
            [44, 45],
            "\"../../reference/ship_classes_five_days_oilers.csv\",",
        ),
    ] {
        for letter in LETTERS {
            let who = format!("coral {coral}, file {letter}");
            let old = tables(&read(&old_file(coral, letter)));
            let new = tables(&read(&file(coral, letter)));

            // The list of class files: the old with the one file inserted.
            let (old_list, new_list) = (class_list(&old[0]), class_list(&new[0]));
            assert_eq!(new_list.len(), old_list.len() + 1, "{who}");
            let without: Vec<String> = new_list
                .iter()
                .filter(|l| l.as_str() != oiler_file)
                .cloned()
                .collect();
            assert_eq!(without, old_list, "{who}: the list with one inserted");
            assert_eq!(
                new_list.iter().filter(|l| l.as_str() == oiler_file).count(),
                1,
                "{who}"
            );

            // What comes before any table, without the name, the list of
            // class files and the sides file.
            let bare = |preamble: &[String]| -> Vec<String> {
                let start = preamble
                    .iter()
                    .position(|l| l.starts_with("ship_classes = ["))
                    .expect("a list");
                let end = start
                    + preamble[start..]
                        .iter()
                        .position(|l| l.trim() == "]")
                        .expect("its end");
                preamble
                    .iter()
                    .enumerate()
                    .filter(|(i, l)| {
                        (*i < start || *i > end)
                            && !l.starts_with("name = ")
                            && !l.starts_with("class_sides = ")
                    })
                    .map(|(_, l)| l.clone())
                    .collect()
            };
            assert_eq!(bare(&old[0]), bare(&new[0]), "{who}: the header");
            assert!(
                old[0].iter().any(|l| l.starts_with("class_sides = "))
                    && new[0].iter().any(|l| l.starts_with("class_sides = ")),
                "{who}: both name a sides file"
            );
            assert_ne!(
                old[0].iter().find(|l| l.starts_with("class_sides = ")),
                new[0].iter().find(|l| l.starts_with("class_sides = ")),
                "{who}: the sides file is another"
            );

            // The other tables, the new without the group's force and ships.
            let mut removed_forces = 0;
            let mut removed_ships = Vec::new();
            let rest: Vec<Vec<String>> = new[1..]
                .iter()
                .filter(|table| {
                    let header = table[0].as_str();
                    if header == "[[task_force]]" && numbers_of(table, &["id"]) == [6] {
                        removed_forces += 1;
                        return false;
                    }
                    if header == "[[ship]]" && numbers_of(table, &["task_force"]) == [6] {
                        removed_ships.push(numbers_of(table, &["id"])[0]);
                        return false;
                    }
                    true
                })
                .cloned()
                .collect();
            assert_eq!(removed_forces, 1, "{who}: one force added");
            assert_eq!(removed_ships, group_ships, "{who}: two ships added");
            assert!(rest == old[1..], "{who}: every other table is the old's");
        }
    }
}
