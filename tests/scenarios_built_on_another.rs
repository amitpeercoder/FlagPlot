//! A scenario built on another (decision 0234): the architect's statements,
//! written on Nov 27, 2026, before any code. No number here is a figure of
//! the game. The rule was decision 0234's "The format as planned" and is,
//! since step B, the rulebook's section "A scenario built on another"
//! (`docs/rules/01-the-day-orders-and-what-a-side-knows.md`), cited below by
//! the number of its item.
//!
//! The statements L are of the loader. L1 to L14 and L16 are tested in
//! `sim-data` (`crates/sim-data/src/scenario/tests/built_on.rs`) on invented
//! files held in a `Pack`; L15, which reads `sim-cli`'s check of decision
//! 0207, is tested here. The statements R are of the repository's files and
//! are tested here: R1 and R2 since step B, R3 from step C, R4 and R5 from
//! step D. `tests/all.rs` names this file as a module.
//!
//! The loader
//!
//! L1.  A file with no `built_on` loads as before: the same scenario, the
//!      same `data_files`, and for a faulty file the same error text, line
//!      and column included (item 1).
//! L2.  A plain key the overlay states takes the place of the value below; a
//!      plain key it does not state is the one below. `ship_classes` stated
//!      as one name over a list below gives one name. `[side.allied]` stated
//!      over `[side.allied]` and `[side.japanese]` below gives a side table
//!      with no Japanese part (item 4).
//! L3.  In a list by id, a row whose id is below changes the keys it states
//!      and no other, and keeps its place: over ships 1, 2, 3 an overlay's
//!      row for ship 2 stating `fuel_t` gives ships 1, 2, 3 in that order,
//!      ship 2 with the new `fuel_t` and every other key as below (item 5).
//! L4.  A row with a new id comes after the rows below, in the overlay's
//!      order: over ships 1, 2 an overlay's rows 9 then 4 give 1, 2, 9, 4
//!      (item 5).
//! L5.  A row of a list by id with no `id`, and two rows of one id in the
//!      overlay, are each refused with an error that names the overlay, the
//!      list and, for the second, the id (item 5).
//! L6.  In any other list the overlay's rows come after the rows below: over
//!      commands A, B an overlay's C, D give A, B, C, D (item 6).
//! L7.  An overlay in another folder reads the files named below where the
//!      file below reads them, and names them with the folder pair struck
//!      out: an overlay in `x/` built on `../y/s.toml`, whose `map` is
//!      `../m/a.toml`, reads `../m/a.toml` from `x/`, and its `data_files`
//!      are, name for name, those of a copy of `s.toml` written out in full
//!      in `x/` (item 8). A file key the overlay states itself is used as
//!      written.
//! L8.  A chain of three loads as the three laid in order, lowest first
//!      (item 2).
//! L9.  A file built on itself, a chain of two that name each other, a
//!      chain of nine files, and a `built_on` that names no file are each
//!      refused; the error names the file asked for and the file that could
//!      not be read or was read twice (item 2).
//! L10. `built_on` given as a list, or as anything but one string, is
//!      refused (item 2).
//! L11. A key no scenario has, stated in an overlay or in the file below,
//!      is refused, and so is a float in either (item 3).
//! L12. Loaded with a reference root, an overlay reads the file it is built
//!      on where it names it, and its reference files from the root, as a
//!      file written out in full does (item 9).
//! L13. `data_files` of an overlay are those of the same scenario written
//!      out in full, then the files it is built on, nearest first. The pack
//!      of an overlay holds the overlay and each of them, and the overlay
//!      loads from that pack alone (item 11).
//! L14. `resolve` of a file with no `built_on` parses back to that file's
//!      table; `resolve` of an overlay, written to a file in the overlay's
//!      folder, loads to a scenario with the same hash at day 0, the same
//!      script and the same commands (item 12).
//! L15. `estimated_classes` of an overlay that states no `ship_classes`
//!      gives the classes of the same scenario written out in full. The
//!      check of decision 0207's rule 12 passes where the mark's words are
//!      in the header of the overlay or of any file it is built on, and
//!      fails where they are in none.
//! L16. Text that states `built_on`, given to `from_text` with a path, loads
//!      as a file of that text at that path. The resolved text of an
//!      overlay states no `built_on`, and given to `from_text` with the
//!      overlay's path loads to a scenario with the same hash at day 0,
//!      the same script and the same commands as the overlay (items 12
//!      and 13).
//!
//! The repository
//!
//! R1.  Every scenario file under `data/scenarios`, in every folder, that
//!      states `built_on` names a file under `data/scenarios` that is there,
//!      and that file loads alone (items 1 and 10).
//! R2.  No file of `step2-frozen`, `orders-inside-proof`, `second-proof`,
//!      `stage-2b-proof`, `stage-3-proof`, `stage-4-proof` or `staff`, and
//!      no scenario a recorded game under
//!      `crates/sim-cli/tests/recorded_games` names, states `built_on`
//!      (item 14).
//! R3.  Each converted file states `built_on`, the file decision 0234's
//!      table "Which files convert" gives it, and beside it only the keys,
//!      lists and ids that table gives its row. From step C for the 27
//!      files of `coral-sea`, `five-days` and `midway`; from step D for the
//!      31 of `table-hunger`, `table-boats` and `table-rear-bases`. The
//!      table is copied into this test as data when the step is built; a
//!      test reads no Markdown.
//! R4.  The three folders of the table's games hold the converted files and
//!      nothing else: `table-hunger` 12, `table-boats` 16,
//!      `table-rear-bases` 3 (from step D).
//! R5.  The words "Do not edit by hand" are in eight scenario files and no
//!      other: the four of `orders-inside-proof`, the three of
//!      `stage-3-proof` and `staff/07_full_slice.toml` (from step D).

use crate::common;

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

fn scenarios_dir() -> PathBuf {
    common::data_dir().join("scenarios")
}

/// Every `.toml` file under `dir`, but in the `tmp-` folders, which are
/// someone's trials, sorted.
fn scenario_files(dir: &Path, into: &mut Vec<PathBuf>) {
    for entry in std::fs::read_dir(dir).expect("a folder") {
        let path = entry.expect("an entry").path();
        let name = path
            .file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .into_owned();
        if path.is_dir() {
            if !name.starts_with("tmp-") {
                scenario_files(&path, into);
            }
        } else if name.ends_with(".toml") {
            into.push(path);
        }
    }
    into.sort();
}

/// The file a scenario file is built on, as it names it, if it states one.
fn built_on_of(path: &Path) -> Option<String> {
    let text = std::fs::read_to_string(path).expect("reads");
    let table: toml::Table = toml::from_str(&text).ok()?;
    table
        .get("built_on")
        .map(|v| v.as_str().unwrap_or_default().to_string())
}

#[test]
fn r1_every_file_built_on_another_names_a_scenario_that_loads_alone() {
    let mut files = Vec::new();
    scenario_files(&scenarios_dir(), &mut files);
    assert!(files.len() > 200, "{} files", files.len());
    let mut overlays = 0;
    for file in &files {
        let Some(name) = built_on_of(file) else {
            continue;
        };
        overlays += 1;
        let below = file.parent().expect("a folder").join(&name);
        assert!(
            below.is_file(),
            "{} names {name}, which is not there",
            file.display()
        );
        assert!(
            below
                .canonicalize()
                .expect("it is there")
                .starts_with(scenarios_dir().canonicalize().expect("the folder")),
            "{} is built on {name}, which is not under data/scenarios",
            file.display()
        );
        common::load_live(&below).unwrap_or_else(|e| {
            panic!(
                "{} is built on {name}, which does not load alone: {e}",
                file.display()
            )
        });
    }
    eprintln!("{overlays} scenario files state built_on");
}

#[test]
fn r2_no_frozen_proof_or_recorded_file_states_built_on() {
    let mut barred: BTreeSet<PathBuf> = BTreeSet::new();
    let mut files = Vec::new();
    for folder in [
        "step2-frozen",
        "orders-inside-proof",
        "second-proof",
        "stage-2b-proof",
        "stage-3-proof",
        "stage-4-proof",
        "staff",
    ] {
        let mut inside = Vec::new();
        scenario_files(&scenarios_dir().join(folder), &mut inside);
        assert!(!inside.is_empty(), "{folder} holds no scenario");
        files.extend(inside);
    }
    let games = common::data_dir().join("../crates/sim-cli/tests/recorded_games");
    for entry in std::fs::read_dir(&games).expect("the recorded games") {
        let save = entry.expect("an entry").path().join("save.json");
        if !save.is_file() {
            continue;
        }
        let saved: serde_json::Value =
            serde_json::from_str(&std::fs::read_to_string(&save).expect("reads")).expect("json");
        let named = saved["scenario"]["file"].as_str().expect("a scenario file");
        barred.insert(common::data_dir().join("..").join(named));
    }
    assert!(barred.len() >= 5, "{barred:?}");
    files.extend(barred);
    for file in files {
        assert!(
            built_on_of(&file).is_none(),
            "{} states built_on, and may not (decision 0234, item 14)",
            file.display()
        );
    }
}

/// L15: a scenario that states no `ship_classes` has those of the file it
/// is built on, and rule 12 of decision 0207 finds its words in the header
/// of any file of the chain.
#[test]
fn l15_estimates_are_read_from_the_resolved_scenario_and_the_headers_of_the_chain() {
    use sim_cli::estimates::{estimated_classes, estimated_classes_of, header_problem};

    let dir = std::env::temp_dir().join(format!("flagplot-0234-estimates-{}", std::process::id()));
    std::fs::create_dir_all(&dir).expect("a folder");
    let real = scenarios_dir().join("coral-sea/02a_coral_sea_oilers.toml");
    // The file resolved: 02a is itself an overlay, and the file written out
    // in full is the scenario it makes.
    let real_text = common::resolved_text(&real);
    let full = estimated_classes_of(&real_text, &real).expect("classes");
    assert!(!full.is_empty(), "the file loads a class with an estimate");
    // The file written out whole but for its header and with its file names
    // made absolute, so that it can sit in another folder.
    let folder = real.parent().expect("a folder");
    let mut below = String::new();
    for line in real_text.lines() {
        if line.trim_start().starts_with('#') {
            continue;
        }
        below.push_str(&line.replace("\"../", &format!("\"{}/../", folder.display())));
        below.push('\n');
    }
    let base = dir.join("base.toml");
    std::fs::write(&base, &below).expect("writes");
    let over = dir.join("over.toml");
    let say = "# Built on base.toml. Its figures are estimated (decision 0207, rule 12).\n";
    // The overlay states no ship_classes, and neither header says the mark.
    std::fs::write(&over, "built_on = \"base.toml\"\nname = \"Over\"\n").expect("writes");
    let classes = estimated_classes(&over).expect("classes of the resolved scenario");
    let ids = |list: &[sim_cli::estimates::EstimatedClass]| -> Vec<_> {
        list.iter()
            .map(|c| (c.id, c.name.clone(), c.figures.clone()))
            .collect()
    };
    assert_eq!(ids(&classes), ids(&full));
    let problem = header_problem(&over).expect("reads").expect("a problem");
    assert!(problem.contains("decision 0207"), "{problem}");
    // The words in the header of the overlay.
    std::fs::write(
        &over,
        format!("{say}built_on = \"base.toml\"\nname = \"Over\"\n"),
    )
    .expect("writes");
    assert_eq!(header_problem(&over).expect("reads"), None);
    // The words in the header of the file below.
    std::fs::write(&base, format!("{say}{below}")).expect("writes");
    std::fs::write(&over, "built_on = \"base.toml\"\nname = \"Over\"\n").expect("writes");
    assert_eq!(header_problem(&over).expect("reads"), None);
    assert!(common::headers(&over).contains("decision 0207"));
    std::fs::remove_dir_all(&dir).expect("removes the folder");
}

/// One row of decision 0234's table "Which files convert": the file, the
/// file it is built on, the plain keys it states beside `built_on`, and the
/// lists of units it states with the ids of their rows. `commands` is true
/// where it also states `[[command]]` rows, which no row's id names.
struct Converted {
    file: &'static str,
    built_on: &'static str,
    keys: &'static [&'static str],
    rows: &'static [(&'static str, &'static [i64])],
    commands: bool,
}

const NAME: &[&str] = &["name"];
const GROUND: &[&str] = &["ground"];
/// What a file of `table-boats` states beside its boats: the three keys of
/// the submarine files and the two `[side]` tables; file a also the ground
/// figures of the table.
const BOAT_KEYS: &[&str] = &[
    "submarine_classes",
    "submarine_torpedoes",
    "submarines",
    "side",
];
const BOAT_KEYS_A: &[&str] = &[
    "ground",
    "submarine_classes",
    "submarine_torpedoes",
    "submarines",
    "side",
];
const REAR_KEYS: &[&str] = &["name", "trial"];
const RUNS: &[i64] = &[30, 32, 33, 34, 35, 36, 37, 38, 39];
const NEW_CLASSES: &[&str] = &["name", "ship_classes", "class_sides"];

/// The table, copied as data when step C was built and step D (its 31 rows
/// of `table-hunger`, `table-boats` and `table-rear-bases`). A file `a`, `b`
/// or `c` of a pairing states `name` and its commands; a file `d`, `name`
/// alone. A row of the lists `island` is named by its `number`, not an `id`.
fn converted() -> Vec<Converted> {
    fn leak(text: String) -> &'static str {
        Box::leak(text.into_boxed_str())
    }
    let mut out = vec![
        Converted {
            file: "coral-sea/base_coral_sea_oilers.toml",
            built_on: "base_coral_sea.toml",
            keys: NEW_CLASSES,
            rows: &[("task_force", &[6]), ("ship", &[54, 55])],
            commands: false,
        },
        Converted {
            file: "coral-sea/base_coral_sea_both_oilers.toml",
            built_on: "base_coral_sea_oilers.toml",
            keys: NEW_CLASSES,
            rows: &[("task_force", &[7]), ("ship", &[56])],
            commands: false,
        },
        Converted {
            file: "five-days/base_eastern_solomons_oilers.toml",
            built_on: "base_eastern_solomons.toml",
            keys: NEW_CLASSES,
            rows: &[("task_force", &[6]), ("ship", &[44, 45])],
            commands: false,
        },
    ];
    // (folder, file number, file name after the letter, the base it is built on)
    for (folder, number, rest, base) in [
        ("coral-sea", "01", "coral_sea", "base_coral_sea"),
        (
            "coral-sea",
            "02",
            "coral_sea_oilers",
            "base_coral_sea_oilers",
        ),
        (
            "coral-sea",
            "03",
            "coral_sea_both_oilers",
            "base_coral_sea_both_oilers",
        ),
        (
            "five-days",
            "01",
            "eastern_solomons",
            "base_eastern_solomons",
        ),
        (
            "five-days",
            "02",
            "eastern_solomons_oilers",
            "base_eastern_solomons_oilers",
        ),
        ("midway", "01", "midway", "base_midway"),
    ] {
        for letter in ["a", "b", "c", "d"] {
            let file: &'static str =
                Box::leak(format!("{folder}/{number}{letter}_{rest}.toml").into_boxed_str());
            let built_on: &'static str = Box::leak(format!("{base}.toml").into_boxed_str());
            out.push(Converted {
                file,
                built_on,
                keys: NAME,
                rows: &[],
                commands: letter != "d",
            });
        }
    }
    // The table's own files (step D).
    // (the table-hunger copy, the file it is built on)
    for (copy, below) in [
        ("01b_midway", "midway/01b_midway"),
        ("01c_midway", "midway/01c_midway"),
        ("01d_midway", "midway/01d_midway"),
        (
            "02b_eastern_solomons_oilers",
            "five-days/02b_eastern_solomons_oilers",
        ),
        (
            "02c_eastern_solomons_oilers",
            "five-days/02c_eastern_solomons_oilers",
        ),
        (
            "02d_eastern_solomons_oilers",
            "five-days/02d_eastern_solomons_oilers",
        ),
        (
            "03b_coral_sea_both_oilers",
            "coral-sea/03b_coral_sea_both_oilers",
        ),
        (
            "03c_coral_sea_both_oilers",
            "coral-sea/03c_coral_sea_both_oilers",
        ),
        (
            "03d_coral_sea_both_oilers",
            "coral-sea/03d_coral_sea_both_oilers",
        ),
        ("07_full_slice", "staff/07_full_slice"),
        ("07b_slice", "stage-3-proof/07b_slice"),
        ("07c_slice", "stage-3-proof/07c_slice"),
    ] {
        out.push(Converted {
            file: leak(format!("table-hunger/{copy}.toml")),
            built_on: leak(format!("../{below}.toml")),
            keys: GROUND,
            rows: &[],
            commands: false,
        });
    }
    // (the file of table-boats, the file it is built on, the boats' ids)
    let midway: &'static [i64] = &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
    let coral: &'static [i64] = &[1, 2, 3, 4];
    let five: &'static [i64] = &[1, 2, 3, 4, 5];
    let slice: &'static [i64] = &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13];
    for (file, below, a, boats) in [
        ("01a_midway", "midway/01a_midway", true, midway),
        ("01b_midway", "table-hunger/01b_midway", false, midway),
        ("01c_midway", "table-hunger/01c_midway", false, midway),
        ("01d_midway", "table-hunger/01d_midway", false, midway),
        (
            "02a_eastern_solomons_oilers",
            "five-days/02a_eastern_solomons_oilers",
            true,
            five,
        ),
        (
            "02b_eastern_solomons_oilers",
            "table-hunger/02b_eastern_solomons_oilers",
            false,
            five,
        ),
        (
            "02c_eastern_solomons_oilers",
            "table-hunger/02c_eastern_solomons_oilers",
            false,
            five,
        ),
        (
            "02d_eastern_solomons_oilers",
            "table-hunger/02d_eastern_solomons_oilers",
            false,
            five,
        ),
        (
            "03a_coral_sea_both_oilers",
            "coral-sea/03a_coral_sea_both_oilers",
            true,
            coral,
        ),
        (
            "03b_coral_sea_both_oilers",
            "table-hunger/03b_coral_sea_both_oilers",
            false,
            coral,
        ),
        (
            "03c_coral_sea_both_oilers",
            "table-hunger/03c_coral_sea_both_oilers",
            false,
            coral,
        ),
        (
            "03d_coral_sea_both_oilers",
            "table-hunger/03d_coral_sea_both_oilers",
            false,
            coral,
        ),
        ("07_full_slice", "table-hunger/07_full_slice", false, slice),
        ("07a_slice", "stage-3-proof/07a_slice", true, slice),
        ("07b_slice", "table-hunger/07b_slice", false, slice),
        ("07c_slice", "table-hunger/07c_slice", false, slice),
    ] {
        let rows: &'static [(&'static str, &'static [i64])] =
            Box::leak(vec![("submarine", boats)].into_boxed_slice());
        out.push(Converted {
            file: leak(format!("table-boats/{file}_submarines.toml")),
            built_on: leak(format!("../{below}.toml")),
            keys: if a { BOAT_KEYS_A } else { BOAT_KEYS },
            rows,
            commands: false,
        });
    }
    // (the file of table-rear-bases: built on the file of its name in
    // table-boats; the islands: 1, 2 and 3 in the full slice, 3 in the others)
    for (file, islands) in [
        ("07_full_slice", &[1, 2, 3][..]),
        ("07b_slice", &[3][..]),
        ("07c_slice", &[3][..]),
    ] {
        let rows: &'static [(&'static str, &'static [i64])] = Box::leak(
            vec![
                ("base", &[3][..]),
                ("task_force", RUNS),
                ("island", islands),
            ]
            .into_boxed_slice(),
        );
        out.push(Converted {
            file: leak(format!("table-rear-bases/{file}_submarines.toml")),
            built_on: leak(format!("../table-boats/{file}_submarines.toml")),
            keys: REAR_KEYS,
            rows,
            commands: false,
        });
    }
    out
}

#[test]
fn r3_each_converted_file_states_what_its_row_gives_and_nothing_else() {
    let files = converted();
    assert_eq!(files.len(), 58, "steps C and D convert 27 and 31 files");
    for row in &files {
        let path = scenarios_dir().join(row.file);
        let text = std::fs::read_to_string(&path).expect("reads");
        let table: toml::Table =
            toml::from_str(&text).unwrap_or_else(|e| panic!("{}: {e}", row.file));
        assert_eq!(
            table.get("built_on").and_then(toml::Value::as_str),
            Some(row.built_on),
            "{} is built on {}",
            row.file,
            row.built_on
        );
        let mut want: BTreeSet<&str> = BTreeSet::from(["built_on"]);
        want.extend(row.keys.iter().copied());
        want.extend(row.rows.iter().map(|(list, _)| *list));
        if row.commands {
            want.insert("command");
        }
        let has: BTreeSet<&str> = table.keys().map(String::as_str).collect();
        assert_eq!(
            has, want,
            "{} states the keys and lists its row gives",
            row.file
        );
        for (list, ids) in row.rows {
            let got: Vec<i64> = table[*list]
                .as_array()
                .expect("a list of tables")
                .iter()
                .map(|r| {
                    r.get("id")
                        .or_else(|| r.get("number"))
                        .and_then(toml::Value::as_integer)
                        .expect("an id, or for an island its number")
                })
                .collect();
            assert_eq!(&got, ids, "{}: the rows of {list}", row.file);
            // The files of `table-rear-bases` state, of each row, these keys
            // and no other (decision 0232, item 5a).
            if row.file.starts_with("table-rear-bases/") {
                let want: &[&str] = match *list {
                    "base" => &["id", "supply_t"],
                    "task_force" => &["at_base", "available_from", "home_port", "id"],
                    _ => &["name", "number", "places"],
                };
                for r in table[*list].as_array().expect("a list of tables") {
                    let have: Vec<&str> = r
                        .as_table()
                        .expect("a table")
                        .keys()
                        .map(String::as_str)
                        .collect();
                    let mut want = want.to_vec();
                    want.sort_unstable();
                    let mut have = have;
                    have.sort_unstable();
                    assert_eq!(have, want, "{}: the keys of a row of {list}", row.file);
                }
            }
        }
        if row.commands {
            assert!(
                !table["command"].as_array().expect("commands").is_empty(),
                "{} states commands",
                row.file
            );
        }
    }
}

#[test]
fn r4_the_three_folders_of_the_tables_games_hold_the_converted_files_and_nothing_else() {
    for (folder, count) in [
        ("table-hunger", 12),
        ("table-boats", 16),
        ("table-rear-bases", 3),
    ] {
        let mut files = Vec::new();
        scenario_files(&scenarios_dir().join(folder), &mut files);
        assert_eq!(files.len(), count, "{folder} holds {count} files");
        let others: Vec<_> = std::fs::read_dir(scenarios_dir().join(folder))
            .expect("the folder")
            .map(|e| e.expect("an entry").path())
            .filter(|p| p.extension().is_none_or(|e| e != "toml"))
            .collect();
        assert!(
            others.is_empty(),
            "{folder} holds {others:?} beside the files"
        );
    }
}

#[test]
fn r5_do_not_edit_by_hand_is_in_eight_files_and_no_other() {
    let mut files = Vec::new();
    scenario_files(&scenarios_dir(), &mut files);
    let said: Vec<String> = files
        .iter()
        .filter(|f| {
            std::fs::read_to_string(f)
                .expect("reads")
                .contains("Do not edit by hand")
        })
        .map(|f| {
            f.strip_prefix(scenarios_dir())
                .expect("under the folder")
                .to_string_lossy()
                .into_owned()
        })
        .collect();
    let folder = |name: &str| said.iter().filter(|f| f.starts_with(name)).count();
    assert_eq!(said.len(), 8, "{said:?}");
    assert_eq!(folder("orders-inside-proof/"), 4, "{said:?}");
    assert_eq!(folder("stage-3-proof/"), 3, "{said:?}");
    assert!(
        said.contains(&"staff/07_full_slice.toml".to_string()),
        "{said:?}"
    );
}
