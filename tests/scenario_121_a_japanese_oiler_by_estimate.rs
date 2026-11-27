//! Scenario 121: a Japanese oiler by estimate. Toho Maru is derived with
//! one attribute estimated from like ships and one read as her sister's,
//! each marked, and sails with the Striking Force at the Coral Sea.
//!
//! Proves decision 0207. The statements are the architect's, written
//! before the code. A mismatch goes back to the architect, and nobody else
//! changes a statement or a number here.
//!
//! # The figures of Toho Maru, worked from the rows
//!
//! A ton of the simulation is a long ton. A division is cut at the sixth
//! place, as the derivation cuts.
//!
//! 1. **Read from her own rows and her class's:** kind auxiliary, and the
//!    mark that she fuels others at sea; maximum speed 19 knots; cruising
//!    speed 16 knots; her own fuel 1,100 tons; endurance 15,000 nautical
//!    miles, so 0.073333 tons a mile at her cruising speed; no main
//!    battery and no torpedo tubes, by her kind.
//! 2. **Her cargo, as her sister's:** Toa Maru's 12,487 tons. Toho Maru
//!    has no cargo row of her own, and Toa Maru is the one ship of her
//!    class that has.
//! 3. **Her light displacement, by ratio.** Her deadweight is 13,431
//!    tons. The like classes, every class of the type oiler on file with
//!    a light displacement and a deadweight, are two:
//!    - the Cimarron class, 7,256 over 18,250, 0.397589;
//!    - the Patoka class, 5,375 over 12,600, 0.426587.
//!    Their mean is 0.412088. Times 13,431 it is 5,534.753928 tons.
//! 4. **Her toughness** is that over 25: 221.390157.
//! 5. **The least and the greatest** the like classes give are
//!    5,340.018 tons (the Cimarron class's ratio) and 5,729.489 tons (the
//!    Patoka class's), and the check prints them.
//! 6. **Her guns are not estimated.** No source gives them. Her
//!    anti-aircraft figure is 0 by the list's assumption that guns
//!    unknown means none, said as an assumption of the scenario; her
//!    radar is 0 by the same kind of assumption.
//!
//! # Statements: the marks
//!
//! 7. **The sources file says "estimated"** for her toughness, with the
//!    method, her deadweight row, each like class and its rows, and the
//!    least and the greatest; and for her cargo, with the method and Toa
//!    Maru's row. It prints no rule of a figure read beside either.
//! 8. **The class file games load names them:** its column of estimated
//!    figures has toughness and the cargo capacity for her class, and is
//!    empty for a class with none. No class file on file has the column.
//! 9. **The check counts them:** it prints, for her list, two attributes
//!    estimated, for Toho Maru.
//! 10. **A fact drives the estimate out.** With a row of any standing
//!     filed for Toho Maru's light displacement, or for her class's, her
//!     list's check fails and names the entry to take out.
//! 11. **An estimate does not rest on an estimate.** A second list that
//!     names Toho Maru's class as a like class for the light displacement
//!     fails its check.
//! 12. **Never evidence.** A measure of the validation that names a
//!     scenario which loads her class, and whose notes do not name the
//!     class as estimated, is refused by a test; with the note it passes.
//!     A scenario file that loads her class and does not say so in its
//!     header is refused by a test.
//! 13. **Neosho's cargo is marked too.** In decision 0206's Coral Sea
//!     list her cargo, the least of her sisters', is an entry by the
//!     second method and is marked estimated; its value, 15,408.3208
//!     tons, is what it was.
//!
//! # Statements: the Coral Sea with both oilers
//!
//! The new Coral Sea file is decision 0206's, with one force more.
//!
//! 14. **The Japanese fuelling group.** Toho Maru is a force of the
//!     Japanese side by herself, of the mission transport, with no home
//!     port, in the hex of the Striking Force, (4,-2), at the start. She
//!     has 1,100 tons of her own fuel and 12,487 tons of cargo fuel, full,
//!     as every ship in that file starts full. No order is given to her
//!     by the scenario. The file's header says her light displacement and
//!     her cargo are estimated, and that her load on 5 May is not known.
//! 15. **The staff sends a short force to her.** With a destroyer of the
//!     Striking Force below a quarter of her fuel and the Striking Force
//!     under a task of the Japanese staff's, the staff gives the Striking
//!     Force the order to fuel from Toho Maru's force on the first
//!     morning, as it does for the Allied side with Neosho.
//! 16. **A destroyer fuels.** With that destroyer 300 tons short of full,
//!     the Striking Force lying still and under the order to fuel from
//!     Toho Maru's force before the first night: in that night she takes
//!     300 tons, Toho Maru's cargo is 12,187 tons, and the order ends
//!     because every ship is full.
//!
//! # Properties
//!
//! - **No game on file moves.** Every scenario and recorded game on file
//!   gives the events and the daily hashes it gave before; no class file,
//!   ships file or sides file a scenario on file reads is changed but
//!   decision 0206's Coral Sea oilers' class file, which gains the mark
//!   for Neosho's cargo and no change of any figure.
//! - **Nothing is typed.** No figure of Toho Maru's class is written by
//!   hand: each is read or estimated by the derivation, and its check
//!   passes on her list.
//! - **The table's copies** are made from the new Coral Sea files, and its
//!   line for the Coral Sea says each side has an oiler.

// The architect's statement lists are list items in the doc comment above,
// which is left as written.
#![allow(clippy::doc_lazy_continuation)]

use crate::common;

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};
use std::process::Command;

use sim_core::events::{Event, EventKind, FuelOrderEnd};
use sim_core::fixed::Fixed;
use sim_core::hex::Hex;
use sim_core::ids::{ShipClassId, ShipId, TaskForceId};
use sim_core::orders::{Mission, OrderKind, TaskForceOrder};
use sim_core::refdata::{ShipClass, ShipKind};
use sim_core::state::{ShipLocation, Side};
use sim_core::time::Pulse;
use sim_data::Scenario;
use sim_staff::{as_staffs, staffs};

// ------------------------------------------------------------ The harness
//
// The class file and the scenario files are read as games read them, on the
// live figures (they have frozen copies, but no frozen test reads them).

fn repo() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("..")
}

fn reference(file: &str) -> PathBuf {
    common::data_dir().join("reference").join(file)
}

fn read(path: &Path) -> String {
    std::fs::read_to_string(path).unwrap_or_else(|e| panic!("{} reads: {e}", path.display()))
}

fn fx(text: &str) -> Fixed {
    text.parse().expect("a decimal")
}

fn t(n: i64) -> Fixed {
    Fixed::from_int(n)
}

const CLASSES: &str = "ship_classes_coral_sea_japanese_oilers.csv";
const SOURCES: &str = "ship_classes_coral_sea_japanese_oilers.sources.csv";
const LIST: &str = "coral_sea_japanese_oilers.toml";
const TOHO_CLASS: u32 = 4032;
/// Toho Maru, her force and the Striking Force in the new Coral Sea files.
const TOHO: u32 = 56;
const GROUP: u32 = 7;
const STRIKING_FORCE: u32 = 3;
/// Akebono, a destroyer of the Striking Force, 475 tons full.
const AKEBONO: u32 = 26;

fn file(letter: char) -> PathBuf {
    common::data_dir()
        .join("scenarios/coral-sea")
        .join(format!("03{letter}_coral_sea_both_oilers.toml"))
}

/// A scenario file resolved (decision 0234): a key, a row or a table is read
/// from the scenario it makes, whether the file holds it or the file it is
/// built on.
fn resolved(path: &Path) -> String {
    common::resolved_text(path)
}

fn load(path: &Path) -> Scenario {
    Scenario::load(path).unwrap_or_else(|e| panic!("{} loads: {e}", path.display()))
}

fn load_text(text: &str, path: &Path) -> Scenario {
    Scenario::from_text(text, path).expect("the altered scenario loads")
}

fn toho_class() -> ShipClass {
    let data = sim_data::load_ship_classes(&reference(CLASSES)).expect("the class file loads");
    data[&ShipClassId::new(TOHO_CLASS)].clone()
}

/// The rows of the sources file for her class: figure to (rule, facts, note).
fn sources() -> BTreeMap<String, (String, String, String)> {
    let mut reader = csv::ReaderBuilder::new()
        .comment(Some(b'#'))
        .from_path(reference(SOURCES))
        .expect("the sources file opens");
    reader
        .records()
        .map(|r| {
            let r = r.expect("a row");
            assert_eq!(&r[0], "4032");
            (
                r[1].to_string(),
                (r[3].to_string(), r[4].to_string(), r[5].to_string()),
            )
        })
        .collect()
}

/// A number written in a note after `label`, e.g. "least 5340.017859".
fn number_after(note: &str, label: &str) -> Fixed {
    let at = note
        .find(label)
        .unwrap_or_else(|| panic!("`{label}` in {note}"));
    let rest = &note[at + label.len()..];
    let digits: String = rest
        .trim_start()
        .chars()
        .take_while(|c| c.is_ascii_digit() || *c == '.')
        .collect();
    fx(digits.trim_end_matches('.'))
}

/// The rows of a data/oob file as maps of column to text.
fn oob_rows(name: &str) -> Vec<BTreeMap<String, String>> {
    let mut reader = csv::ReaderBuilder::new()
        .comment(Some(b'#'))
        .from_path(common::data_dir().join("oob").join(name))
        .expect("the facts file opens");
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

/// `oob-derive <list> --check`, with the facts of `facts` (by default
/// data/oob): whether it passed, and what it printed. It is asked of the
/// library the program is built on, as the program asks it.
fn derive_check(list: &Path, facts: Option<&Path>) -> (bool, String) {
    let facts = facts.map_or_else(|| repo().join("data/oob"), Path::to_path_buf);
    let said = oob_derive::said(
        &list.display().to_string(),
        oob_derive::run(list, &facts, true),
    );
    (said.passed, format!("{}{}", said.out, said.err))
}

/// A fresh folder of this test's own under the system's temporary folder.
fn scratch(tag: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!(
        "flagpilot-scenario-121-{tag}-{}",
        std::process::id()
    ));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("a scratch folder");
    dir
}

/// A copy of data/oob with `line` added to the end of `file`.
fn facts_with(tag: &str, file: &str, line: &str) -> PathBuf {
    let dir = scratch(tag);
    for entry in std::fs::read_dir(common::data_dir().join("oob")).expect("data/oob") {
        let path = entry.expect("an entry").path();
        if path.is_file() {
            std::fs::copy(&path, dir.join(path.file_name().expect("a name"))).expect("copied");
        }
    }
    let path = dir.join(file);
    let mut text = read(&path);
    text.push_str(line);
    text.push('\n');
    std::fs::write(&path, text).expect("written");
    dir
}

// ----------------------------------------- Statements 1 to 6: her figures

#[test]
fn statement_1_read_from_her_own_rows_and_her_class_s() {
    let c = toho_class();
    assert_eq!(c.kind, ShipKind::Auxiliary, "kind auxiliary");
    assert!(c.fuels_at_sea, "the mark that she fuels others at sea");
    assert_eq!(c.max_speed_kn, t(19));
    assert_eq!(c.cruise_speed_kn, t(16));
    assert_eq!(c.fuel_capacity_t, t(1100));
    assert_eq!(c.endurance_nm, t(15000));
    // What a mile costs her at her cruising speed, as the engine burns it
    // (sim-core `movement`: distance x fuel capacity / endurance).
    assert_eq!(
        Fixed::ONE.mul_div(c.fuel_capacity_t, c.endurance_nm),
        fx("0.073333")
    );
    assert_eq!(c.main_guns, 0);
    assert!(c.gun_rounds_per_min.is_zero());
    assert_eq!(c.gun_rounds_per_gun, 0);
    assert!(c.gun_damage.is_zero());
    assert_eq!(c.torpedo_tubes, 0);
    assert_eq!(c.torpedo_tubes_one_side, 0);
    assert!(c.torpedo_damage.is_zero());
    // Each is read, not estimated: its rule is a rule of a figure read.
    let s = sources();
    for figure in [
        "cruise_speed_kn",
        "max_speed_kn",
        "fuel_capacity_t",
        "endurance_nm",
    ] {
        assert!(
            !s[figure].0.starts_with("estimated"),
            "{figure}: {:?}",
            s[figure]
        );
    }
    assert_eq!(s["max_speed_kn"].1, "ship_facts.csv:14545");
    assert_eq!(s["fuel_capacity_t"].1, "ship_facts.csv:14710");
    assert_eq!(s["endurance_nm"].1, "ship_facts.csv:14711");
    assert_eq!(s["cruise_speed_kn"].1, "ship_facts.csv:14712");
    assert!(s["main_guns"].0.starts_with("0 by kind"));
}

#[test]
fn statement_2_her_cargo_as_her_sister_s() {
    assert_eq!(toho_class().cargo_capacity_t, t(12487));
    let ships = oob_rows("ships.csv");
    let class_of: BTreeMap<String, String> = ships
        .iter()
        .map(|r| (r["key"].clone(), r["class"].clone()))
        .collect();
    assert_eq!(class_of["toho-maru"], "jp-toa-maru-class");
    let facts = oob_rows("ship_facts.csv");
    let cargo: Vec<&BTreeMap<String, String>> = facts
        .iter()
        .filter(|r| {
            r["attribute"] == "cargo_capacity_lt"
                && class_of
                    .get(&r["entity"])
                    .is_some_and(|c| c == "jp-toa-maru-class")
        })
        .collect();
    // Toho Maru has no cargo row of her own; Toa Maru is the one ship of her
    // class that has, and hers is 12,487.
    assert_eq!(cargo.len(), 1, "{cargo:?}");
    assert_eq!(cargo[0]["entity"], "toa-maru");
    assert_eq!(cargo[0]["value"], "12487");
    assert_eq!(cargo[0]["id"], "14721");
    let s = sources();
    assert_eq!(s["cargo_capacity_t"].1, "ship_facts.csv:14721");
}

/// The like classes, found from the rows as the rule has them: every class
/// of the type oiler on file, her own left out, with class rows of both a
/// light displacement and a deadweight.
fn like_classes() -> BTreeSet<String> {
    let oilers: BTreeSet<String> = oob_rows("ship_classes.csv")
        .into_iter()
        .filter(|r| r["type"] == "oiler" && r["key"] != "jp-toa-maru-class")
        .map(|r| r["key"].clone())
        .collect();
    let facts = oob_rows("ship_class_facts.csv");
    let has = |class: &str, attribute: &str| {
        facts
            .iter()
            .any(|r| r["entity"] == class && r["attribute"] == attribute)
    };
    oilers
        .into_iter()
        .filter(|c| has(c, "displacement_light_lt") && has(c, "deadweight_lt"))
        .collect()
}

#[test]
fn statement_3_her_light_displacement_by_ratio() {
    assert_eq!(
        like_classes(),
        BTreeSet::from([
            "us-cimarron-class".to_string(),
            "us-patoka-class".to_string()
        ])
    );
    let (rule, facts, note) = sources()["toughness"].clone();
    eprintln!("statement 3 note: {note}");
    assert!(rule.starts_with("estimated"), "{rule}");
    assert!(
        note.contains("her deadweight_lt 13431 (ship_facts.csv:14709)"),
        "{note}"
    );
    assert!(
        note.contains("us-cimarron-class 7256 / 18250 = 0.397589"),
        "{note}"
    );
    assert!(
        note.contains("us-patoka-class 5375 / 12600 = 0.426587"),
        "{note}"
    );
    assert!(
        note.contains("x 0.412088, the mean over 2 like classes"),
        "{note}"
    );
    assert_eq!(
        number_after(&note, "displacement_light_lt "),
        fx("5534.753928")
    );
    // The working, cut at the sixth place as the derivation cuts.
    let cut = |n: i64, d: i64| t(n) / t(d);
    assert_eq!(cut(7256, 18250), fx("0.397589"));
    assert_eq!(cut(5375, 12600), fx("0.426587"));
    assert_eq!((fx("0.397589") + fx("0.426587")) / t(2), fx("0.412088"));
    assert_eq!(t(13431) * fx("0.412088"), fx("5534.753928"));
    for row in [
        "ship_facts.csv:14709",
        "ship_class_facts.csv:14300",
        "ship_class_facts.csv:14343",
        "ship_class_facts.csv:14306",
        "ship_class_facts.csv:14344",
    ] {
        assert!(facts.split(' ').any(|f| f == row), "{row} in {facts}");
    }
}

#[test]
fn statement_4_her_toughness() {
    assert_eq!(toho_class().toughness, fx("221.390157"));
    assert_eq!(fx("5534.753928") / t(25), fx("221.390157"));
}

#[test]
fn statement_5_the_least_and_the_greatest() {
    let note = sources()["toughness"].2.clone();
    let least = number_after(&note, "least ");
    let greatest = number_after(&note, "greatest ");
    eprintln!("statement 5: least {least}, greatest {greatest}");
    // The statement gives them to the third place; the derivation writes
    // them to the sixth (13,431 x 0.397589 and 13,431 x 0.426587).
    let thousandth = fx("0.001");
    let near = |a: Fixed, b: Fixed| a - b < thousandth && b - a < thousandth;
    assert!(near(least, fx("5340.018")), "least {least}");
    assert!(near(greatest, fx("5729.489")), "greatest {greatest}");
    assert!(note.contains("(us-cimarron-class), greatest"), "{note}");
    assert!(
        note.ends_with("(us-patoka-class)") || note.contains("(us-patoka-class);"),
        "{note}"
    );
    // The check prints them.
    let (ok, said) = derive_check(&repo().join("data/derivation").join(LIST), None);
    assert!(ok, "{said}");
    assert!(
        said.contains(&format!("least {least} (us-cimarron-class)")),
        "{said}"
    );
    assert!(
        said.contains(&format!("greatest {greatest} (us-patoka-class)")),
        "{said}"
    );
}

#[test]
fn statement_6_her_guns_are_not_estimated() {
    let c = toho_class();
    assert_eq!(c.anti_aircraft, 0);
    assert!(!c.radar);
    // No fit of hers is on file.
    assert!(oob_rows("fits.csv")
        .iter()
        .all(|r| r["carrier"] != "toho-maru" && r["carrier"] != "jp-toa-maru-class"));
    let s = sources();
    assert!(
        s["anti_aircraft"]
            .2
            .contains("assumption of the scenario, not a fact: anti-aircraft guns unknown"),
        "{:?}",
        s["anti_aircraft"]
    );
    assert!(s["radar"].2.contains("radar assumed none for toho-maru"));
    let header = read(&reference(CLASSES));
    assert!(header.contains("Anti-aircraft assumptions used (decision 0207):\n#   toho-maru"));
    assert!(header.contains("Radar assumptions used (decision 0019, 5):\n#   toho-maru"));
    let list = read(&repo().join("data/derivation").join(LIST));
    assert!(list.contains("anti_aircraft_unknown_means_none = [\"toho-maru\"]"));
    assert!(list.contains("radar_unknown_means_none = [\"toho-maru\"]"));
}

// --------------------------------------------- Statements 7 to 13: the marks

#[test]
fn statement_7_the_sources_file_says_estimated() {
    let s = sources();
    let (rule, _, note) = &s["toughness"];
    assert!(
        rule.starts_with("estimated, not read from a row (decision 0207)"),
        "{rule}"
    );
    assert!(
        rule.contains("by ratio") || rule.contains("x the mean"),
        "{rule}"
    );
    assert!(note.contains("estimated by ratio"), "{note}");
    assert!(
        note.contains("least ") && note.contains("greatest "),
        "{note}"
    );
    // The rule of a figure read is not printed beside it.
    assert!(!rule.contains(
        "displacement_light_lt of the ship, else of its class / displacement_lt_per_toughness"
    ));
    let (rule, facts, note) = &s["cargo_capacity_t"];
    assert!(
        rule.starts_with("estimated, not read from a row (decision 0207)"),
        "{rule}"
    );
    assert!(rule.contains("as her sisters"), "{rule}");
    assert!(note.contains("estimated as her sisters"), "{note}");
    assert!(
        note.contains("toa-maru 12487 (ship_facts.csv:14721)"),
        "{note}"
    );
    assert_eq!(facts, "ship_facts.csv:14721");
    assert!(!rule.contains("cargo_capacity_lt of the ship, else of its class"));
    // No other figure says so.
    for (figure, (rule, _, _)) in &s {
        if figure != "toughness" && figure != "cargo_capacity_t" {
            assert!(!rule.starts_with("estimated"), "{figure}: {rule}");
        }
    }
}

/// The header of a class file: its first line that is not a comment.
fn columns(path: &Path) -> Vec<String> {
    read(path)
        .lines()
        .find(|l| !l.starts_with('#'))
        .unwrap_or_default()
        .split(',')
        .map(str::to_string)
        .collect()
}

#[test]
fn statement_8_the_class_file_games_load_names_them() {
    let (_, marked) =
        sim_data::load_ship_classes_marked(&reference(CLASSES)).expect("the class file loads");
    assert_eq!(
        marked[&ShipClassId::new(TOHO_CLASS)],
        vec!["cargo_capacity_t".to_string(), "toughness".to_string()]
    );
    // Empty for a class with none: Sims's, in the oilers' class file.
    let (_, oilers) =
        sim_data::load_ship_classes_marked(&reference("ship_classes_coral_sea_oilers.csv"))
            .expect("loads");
    assert!(!oilers.contains_key(&ShipClassId::new(4031)));
    // The column is in a class file only where its list has an estimate:
    // hers, and decision 0206's Coral Sea oilers' (statement 13).
    let mut with_column = Vec::new();
    for entry in std::fs::read_dir(common::data_dir().join("reference")).expect("the folder") {
        let path = entry.expect("an entry").path();
        let name = path
            .file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .into_owned();
        if name.starts_with("ship_classes")
            && name.ends_with(".csv")
            && !name.contains(".sources")
            && !name.contains(".ships")
            && columns(&path).iter().any(|c| c == "estimated")
        {
            with_column.push(name);
        }
    }
    with_column.sort();
    assert_eq!(
        with_column,
        vec![
            "ship_classes_coral_sea_japanese_oilers.csv".to_string(),
            "ship_classes_coral_sea_oilers.csv".to_string()
        ]
    );
}

#[test]
fn statement_9_the_check_counts_them() {
    let (ok, said) = derive_check(&repo().join("data/derivation").join(LIST), None);
    assert!(ok, "{said}");
    assert!(
        said.contains("2 attributes estimated (decision 0207), for toho-maru:"),
        "{said}"
    );
    assert!(
        said.contains("toho-maru displacement_light_lt: by ratio"),
        "{said}"
    );
    assert!(
        said.contains("toho-maru cargo_capacity_lt: as her sisters"),
        "{said}"
    );
}

const ROW_TAIL: &str = ",Placeholder statement,,,oni-208-j-revised,placeholder locator,2026-11-21,low,stated,unconfirmed,a row filed to test decision 0207 rule 11";

#[test]
fn statement_10_a_fact_drives_the_estimate_out() {
    let list = repo().join("data/derivation").join(LIST);
    for (tag, file, entity) in [
        ("ship", "ship_facts.csv", "toho-maru"),
        ("class", "ship_class_facts.csv", "jp-toa-maru-class"),
    ] {
        let facts = facts_with(
            tag,
            file,
            &format!("99999,{entity},displacement_light_lt,5500{ROW_TAIL}"),
        );
        let (ok, said) = derive_check(&list, Some(&facts));
        let _ = std::fs::remove_dir_all(&facts);
        assert!(!ok, "{tag}: the check fails: {said}");
        assert!(
            said.contains("toho-maru: estimate: toughness"),
            "{tag}: {said}"
        );
        assert!(said.contains(&format!("{file}:99999")), "{tag}: {said}");
        assert!(
            said.contains(
                "take out the list's [[estimate]] entry for ship toho-maru displacement_light_lt"
            ),
            "{tag}: {said}"
        );
    }
}

#[test]
fn statement_11_an_estimate_does_not_rest_on_an_estimate() {
    // A second list beside hers, in a copy of data/derivation, whose entry
    // names her class as a like class for the light displacement.
    let dir = scratch("lists");
    for entry in std::fs::read_dir(repo().join("data/derivation")).expect("the lists") {
        let path = entry.expect("an entry").path();
        std::fs::copy(&path, dir.join(path.file_name().expect("a name"))).expect("copied");
    }
    let second = dir.join("second.toml");
    std::fs::write(
        &second,
        "name = \"A second list\"\ndate = \"1942-05-06\"\nrules = \"rules.toml\"\n\
         output = \"second.csv\"\nfirst_class_id = 9000\nair_figures = true\n\
         ships = [\"kyokuto-maru\"]\n\n[[estimate]]\nship = \"kyokuto-maru\"\n\
         attribute = \"displacement_light_lt\"\nmethod = \"ratio\"\nfrom = \"deadweight_lt\"\n\
         like_classes = [\"jp-toa-maru-class\"]\nby = \"a test\"\ndate = \"2026-11-21\"\n\
         reason = \"A test of decision 0207, rule 8.\"\n\n[class_ids]\n9000 = [\"kyokuto-maru\"]\n",
    )
    .expect("written");
    let (ok, said) = derive_check(&second, Some(&common::data_dir().join("oob")));
    let _ = std::fs::remove_dir_all(&dir);
    eprintln!("statement 11: {said}");
    assert!(!ok, "{said}");
    assert!(said.contains("the like classes are not chosen"), "{said}");
    assert!(
        said.contains(
            "class jp-toa-maru-class has its displacement_light_lt estimated in coral_sea_japanese_oilers.toml, and an estimate never rests on an estimate (decision 0207, rule 8)"
        ),
        "{said}"
    );
}

#[test]
fn statement_12_never_evidence() {
    use sim_cli::estimates::{header_problem_of, measure_problems};
    let path = file('a');
    let text = read(&path);
    // A scenario file that loads her class and does not say so in its
    // header is refused; the file on file says so (in its header or the
    // header of a file it is built on). The resolved text has no header.
    assert_eq!(header_problem_of(&text, &path).expect("reads"), None);
    let body = resolved(&path);
    let refused = header_problem_of(&body, &path)
        .expect("reads")
        .expect("refused");
    assert!(
        refused.contains("class ship_class:4032 (Toa Maru class: cargo_capacity_t toughness)"),
        "{refused}"
    );
    // A measure that names a scenario which loads her class, with notes that
    // do not name the class as estimated, is refused; with the note it
    // passes. The file also loads the oilers' class with Neosho's cargo
    // marked (statement 13), so the note names that class too.
    let measures = |reason: &str| {
        format!(
            "battle = \"coral-sea\"\nscenario = \"data/scenarios/coral-sea/03a_coral_sea_both_oilers.toml\"\nfacts = \"engagement_outcomes.csv\"\n\n[[measure]]\nid = \"a-test\"\ncounts = \"a test\"\nrecord_low = 0\nrecord_high = 0\njudged = false\nrows = []\nreason = \"{reason}\"\n"
        )
    };
    let without = measure_problems(&measures("Rows 1 and 2."), &repo()).expect("reads");
    assert_eq!(without.len(), 1, "{without:?}");
    assert!(
        without[0].contains("class ship_class:4032 (Toa Maru class)"),
        "{without:?}"
    );
    let with = measure_problems(
        &measures("Rows 1 and 2. Toho Maru's Toa Maru class has estimated figures, and Neosho's Cimarron class an estimated cargo (decision 0207)."),
        &repo(),
    )
    .expect("reads");
    assert!(with.is_empty(), "{with:?}");
    // Naming her class alone is refused for the Cimarron class only.
    let half = measure_problems(
        &measures("The Toa Maru class has estimated figures."),
        &repo(),
    )
    .expect("reads");
    assert_eq!(half.len(), 1);
    assert!(
        half[0].contains("class ship_class:4030 (Cimarron class)") && !half[0].contains("4032"),
        "{half:?}"
    );
}

#[test]
fn statement_13_neosho_s_cargo_is_marked_too() {
    let list = read(&repo().join("data/derivation/coral_sea_oilers.toml"));
    assert!(!list.contains("[least_of_sisters]"));
    assert!(list.contains("[[estimate]]\nship = \"ao-23-neosho\"\nattribute = \"cargo_oil_capacity_bbl\"\nmethod = \"sisters\""));
    let (data, marked) =
        sim_data::load_ship_classes_marked(&reference("ship_classes_coral_sea_oilers.csv"))
            .expect("loads");
    assert_eq!(
        marked[&ShipClassId::new(4030)],
        vec!["cargo_capacity_t".to_string()]
    );
    assert_eq!(
        data[&ShipClassId::new(4030)].cargo_capacity_t,
        fx("15408.3208")
    );
    let (ok, said) = derive_check(&repo().join("data/derivation/coral_sea_oilers.toml"), None);
    assert!(ok, "{said}");
    assert!(
        said.contains("1 attribute estimated (decision 0207), for ao-23-neosho:"),
        "{said}"
    );
}

// ------------------------------------- Statements 14 to 16: the Coral Sea

fn force(raw: u32) -> TaskForceId {
    TaskForceId::new(raw)
}

fn ship(raw: u32) -> ShipId {
    ShipId::new(raw)
}

fn events(scenario: &Scenario) -> Vec<Event> {
    scenario.world.events().iter().cloned().collect()
}

/// The values after the keys named, in the lines of the tables of the given
/// headers.
fn named_in_orders(text: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut in_order = false;
    for line in text.lines().map(str::trim) {
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        if line.starts_with('[') {
            in_order = line == "[[order]]" || line == "[[command]]";
            continue;
        }
        if in_order {
            out.push(line.to_string());
        }
    }
    out
}

#[test]
fn statement_14_the_japanese_fuelling_group() {
    let scenario = load(&file('d'));
    let truth = scenario.world.umpire();
    let striking = &truth.task_forces[&force(STRIKING_FORCE)];
    assert_eq!(striking.name, "Striking Force");
    assert_eq!(striking.position(), Hex::new(4, -2));
    let group = &truth.task_forces[&force(GROUP)];
    assert_eq!(group.side, Side::Japanese);
    assert_eq!(group.mission, Mission::Transport);
    assert_eq!(group.home_port, None, "no home port");
    assert_eq!(group.position(), Hex::new(4, -2));
    assert!(group.route.waypoints.is_empty(), "no route");
    assert_eq!(group.ships, BTreeSet::from([ship(TOHO)]), "by herself");
    let toho = &truth.ships[&ship(TOHO)];
    assert_eq!(toho.name, "Toho Maru");
    assert_eq!(toho.class, ShipClassId::new(TOHO_CLASS));
    assert_eq!(toho.location, ShipLocation::TaskForce(force(GROUP)));
    assert_eq!(toho.fuel_t, t(1100));
    assert_eq!(toho.cargo_fuel_t, t(12487));
    // No order of any of the four files names her or her force.
    for letter in ['a', 'b', 'c', 'd'] {
        for line in named_in_orders(&resolved(&file(letter))) {
            let (key, value) = line
                .split_once('=')
                .map(|(k, v)| (k.trim(), v.trim()))
                .unwrap_or(("", ""));
            if ["task_force", "task_forces", "giver"].contains(&key) {
                assert!(
                    !value.split(|c: char| !c.is_ascii_digit()).any(|n| n == "7"),
                    "file {letter}: {line}"
                );
            }
            if ["ship", "ships"].contains(&key) {
                assert!(
                    !value
                        .split(|c: char| !c.is_ascii_digit())
                        .any(|n| n == "56"),
                    "file {letter}: {line}"
                );
            }
        }
        // The header says her light displacement and her cargo are
        // estimated, and that her load on 5 May is not known.
        let head = common::headers(&file(letter))
            .replace('#', " ")
            .split_whitespace()
            .collect::<Vec<_>>()
            .join(" ")
            .to_lowercase();
        assert!(head.contains("light displacement"), "file {letter}");
        assert!(head.contains("cargo"), "file {letter}");
        assert!(head.contains("estimated"), "file {letter}");
        assert!(head.contains("load on 5 may is not known"), "file {letter}");
    }
}

const AKEBONO_FULL: &str =
    "class = 4017\nfuel_t = 475\nid = 26\nname = \"Akebono\"\nside = \"japanese\"";

fn akebono_at(text: &str, tons: i64) -> String {
    common::replace_once(
        text,
        AKEBONO_FULL,
        &format!("class = 4017\nfuel_t = {tons}\nid = 26\nname = \"Akebono\"\nside = \"japanese\""),
    )
}

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
fn statement_15_the_staff_sends_a_short_force_to_her() {
    // 03b gives the Japanese side its orders: the Striking Force is under a
    // task of the Japanese staff's.
    let path = file('b');
    let text = resolved(&path);
    // At 100 tons, below a quarter of her 475 (118.75).
    let mut scenario = load_text(&akebono_at(&text, 100), &path);
    let held = staffs();
    scenario
        .next_day_staffed(as_staffs(&held))
        .expect("day 1 resolves");
    let log = events(&scenario);
    let orders = staff_orders(&log, &scenario.world);
    eprintln!("statement 15 orders: {orders:?}");
    assert!(
        orders.iter().any(|(day, target, order)| *day == 1
            && *target == force(STRIKING_FORCE)
            && *order == TaskForceOrder::FuelFrom(force(GROUP))),
        "the staff gives the Striking Force the order to fuel from Toho Maru's force: {orders:?}"
    );
    // On the first morning.
    // The staff is asked each morning; the order is of day 1, the first.
    let day = log
        .iter()
        .find_map(|e| match &e.kind {
            EventKind::StaffReason {
                order: Some(id), ..
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
                Some(e.day)
            }
            _ => None,
        })
        .expect("the order's reason");
    assert_eq!(day, 1, "on the first morning");
}

#[test]
fn statement_16_a_destroyer_fuels() {
    let path = file('d');
    let text = akebono_at(&resolved(&path), 175)
        + "\n[[order]]\nday = 1\nside = \"japanese\"\ntask_force = 3\nkind = \"fuel_from\"\ngiver = 7\n";
    let mut scenario = load_text(&text, &path);
    {
        let tf = &scenario.world.umpire().task_forces[&force(STRIKING_FORCE)];
        assert!(tf.route.waypoints.is_empty(), "lying still");
    }
    assert_eq!(
        scenario.world.umpire().ships[&ship(AKEBONO)].fuel_t,
        t(175),
        "300 short of 475"
    );
    scenario.next_day().expect("day 1 resolves");
    let log = events(&scenario);
    let night: Vec<(u32, u32, Fixed, Fixed, Fixed)> = log
        .iter()
        .filter(|e| e.day == 1 && e.pulse == Some(Pulse::Night))
        .filter_map(|e| match &e.kind {
            EventKind::ShipFuelledAtSea {
                ship,
                oiler,
                tons,
                fuel_t,
                oiler_cargo_fuel_t,
                ..
            } => Some((ship.raw(), oiler.raw(), *tons, *fuel_t, *oiler_cargo_fuel_t)),
            _ => None,
        })
        .collect();
    assert_eq!(
        night,
        vec![(AKEBONO, TOHO, t(300), t(475), t(12187))],
        "in the night she takes 300 tons"
    );
    let truth = scenario.world.umpire();
    assert_eq!(truth.ships[&ship(TOHO)].cargo_fuel_t, t(12187));
    let ended: Vec<(u32, Option<Pulse>, FuelOrderEnd)> = log
        .iter()
        .filter_map(|e| match &e.kind {
            EventKind::FuelOrderEnded { reason, .. } => Some((e.day, e.pulse, *reason)),
            _ => None,
        })
        .collect();
    assert_eq!(
        ended,
        vec![(1, Some(Pulse::Night), FuelOrderEnd::Full)],
        "the order ends because every ship is full"
    );
}

// ------------------------------------------------------------- Properties

#[test]
fn property_nothing_is_typed() {
    // Her list's check passes, and the derivation writes exactly the files
    // on file (crates/oob-derive/tests/derive.rs,
    // `the_lists_of_the_oilers_derive_exactly_the_files_in_data_reference`,
    // compares them byte for byte).
    let (ok, said) = derive_check(&repo().join("data/derivation").join(LIST), None);
    assert!(ok, "{said}");
    assert!(read(&reference(CLASSES)).starts_with("# GENERATED by oob-derive"));
}

#[test]
fn property_the_table_s_copies() {
    let json = read(&repo().join("table/scenarios.json"));
    assert!(
        !json.contains("02b_coral_sea_oilers")
            && !json.contains("02c_coral_sea_oilers")
            && !json.contains("02d_coral_sea_oilers")
    );
    for letter in ['b', 'c', 'd'] {
        let copy = format!("data/scenarios/table-hunger/03{letter}_coral_sea_both_oilers.toml");
        assert!(repo().join(&copy).exists(), "{copy}");
        // The table plays the copy with submarines made from it (decision
        // 0227), which stays beside the copy it is made from.
        let played =
            format!("data/scenarios/table-boats/03{letter}_coral_sea_both_oilers_submarines.toml");
        assert!(json.contains(&played), "{played}");
        assert!(repo().join(&played).exists(), "{played}");
    }
    assert!(
        json.contains("Each side has an oiler"),
        "the Coral Sea's line"
    );
    let out = Command::new("python3")
        .arg(repo().join("scripts/table-copies/make_copies.py"))
        .arg("--check")
        .current_dir(repo())
        .output()
        .expect("python3 runs");
    let said = String::from_utf8_lossy(&out.stdout);
    assert!(out.status.success(), "{said}");
    assert_eq!(said.matches(": the copy it makes").count(), 3, "{said}");
    assert_eq!(
        said.matches(": kept, its source is built on another file")
            .count(),
        9,
        "{said}"
    );
}
