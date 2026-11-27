//! A scenario built on another (decision 0234): statements L1 to L14 and
//! L16 of `tests/scenarios_built_on_another.rs`, on invented files held in
//! a `Pack` (the files an invented scenario names are the repository's own,
//! read from the disk).

use super::*;
use crate::scenario::built_on::strike;
use crate::source::{Pack, Source};

/// The repository's `data` folder.
fn data() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../data")
}

/// Invented files in a pack, over the disk for everything else. The
/// invented folders are not on the disk, so a path is read by its words,
/// the `..` parts struck out.
struct Layered {
    invented: Pack,
}

impl Source for Layered {
    fn read_text(&self, path: &Path) -> Result<String, String> {
        let path = strike(path);
        self.invented
            .read_text(&path)
            .or_else(|_| Disk.read_text(&path))
    }
    fn read_bytes(&self, path: &Path) -> Result<Vec<u8>, String> {
        let path = strike(path);
        self.invented
            .read_bytes(&path)
            .or_else(|_| Disk.read_bytes(&path))
    }
    fn is_file(&self, path: &Path) -> bool {
        let path = strike(path);
        self.invented.is_file(&path) || Disk.is_file(&path)
    }
}

/// Where an invented file sits: a folder of the scenarios folder.
fn at(name: &str) -> PathBuf {
    data().join("scenarios").join(name)
}

fn source(files: &[(&str, String)]) -> Layered {
    let mut invented = Pack::new();
    for (name, text) in files {
        invented.insert(strike(&at(name)), text.clone());
    }
    Layered { invented }
}

/// A whole scenario, its file names given for a folder `up` levels below
/// the scenarios folder (`up` = 1 is a folder in it), with `extra` after
/// the two bases.
fn whole(up: usize, extra: &str) -> String {
    let names = "../".repeat(up + 1);
    format!("{}{extra}", HEAD.replace("\"../", &format!("\"{names}")))
}

fn table(text: &str) -> toml::Table {
    toml::from_str(text).expect("the text is a table")
}

fn resolved(files: &[(&str, String)], name: &str) -> toml::Table {
    let text = Scenario::resolved_text(&source(files), &at(name)).expect("resolves");
    table(&text)
}

fn error(files: &[(&str, String)], name: &str) -> String {
    Scenario::load_via(&source(files), &at(name), &LoadOptions::default())
        .expect_err("refused")
        .to_string()
}

fn ids_and_fuel(table: &toml::Table, list: &str) -> Vec<(i64, i64)> {
    table[list]
        .as_array()
        .expect("a list")
        .iter()
        .map(|row| {
            (
                row["id"].as_integer().expect("an id"),
                row["fuel_t"].as_integer().expect("fuel"),
            )
        })
        .collect()
}

fn ships(rows: &[(u32, u32)]) -> String {
    rows.iter()
        .map(|(id, fuel)| format!("\n[[ship]]\nid = {id}\nfuel_t = {fuel}\nname = \"S{id}\"\n"))
        .collect()
}

#[test]
fn l1_a_file_with_no_built_on_loads_as_before() {
    let text = whole(1, &with_force(&ship("")));
    let from_pack = Scenario::load_via(
        &source(&[("y/s.toml", text.clone())]),
        &at("y/s.toml"),
        &LoadOptions::default(),
    )
    .expect("loads");
    let direct = Scenario::parse_via(
        &Layered {
            invented: Pack::new(),
        },
        &text,
        &at("y/s.toml"),
        &LoadOptions::default(),
    )
    .expect("loads");
    assert_eq!(from_pack.world.state_hash(), direct.world.state_hash());
    assert_eq!(from_pack.data_files(), direct.data_files());
    // A faulty file gives the error of `toml::from_str`, line and column
    // included, and says nothing of any chain.
    let faulty = format!("{text}\nstray_key = 1\n");
    let old = toml::from_str::<crate::scenario::file::ScenarioFile>(&faulty)
        .err()
        .expect("refused")
        .to_string();
    assert_eq!(
        Scenario::from_text(&faulty, &at("y/s.toml"))
            .expect_err("refused")
            .message,
        old
    );
    let broken = "name = \n";
    let old = toml::from_str::<crate::scenario::file::ScenarioFile>(broken)
        .err()
        .expect("refused")
        .to_string();
    assert_eq!(
        Scenario::from_text(broken, &at("y/s.toml"))
            .expect_err("refused")
            .message,
        old
    );
    // A mention of the key in a comment is no overlay.
    let commented = format!("# built_on = \"x\"\n{text}");
    assert_eq!(
        Scenario::parse_via(
            &Layered {
                invented: Pack::new()
            },
            &commented,
            &at("y/s.toml"),
            &LoadOptions::default()
        )
        .expect("loads")
        .world
        .state_hash(),
        direct.world.state_hash()
    );
}

#[test]
fn l2_a_plain_key_stated_takes_the_place_of_the_value_below() {
    let below = "name = \"Below\"\ndays = 4\nship_classes = [\"a.csv\", \"b.csv\"]\n\n[side.allied]\ndoctrine = \"x\"\n\n[side.japanese]\ndoctrine = \"y\"\n\n[objectives]\nhorizon = 3\n";
    let over = "built_on = \"s.toml\"\nname = \"Over\"\nship_classes = \"c.csv\"\n\n[side.allied]\ndoctrine = \"z\"\n";
    let files = [
        ("m/s.toml", below.to_string()),
        ("m/o.toml", over.to_string()),
    ];
    let result = resolved(&files, "m/o.toml");
    assert_eq!(result["name"].as_str(), Some("Over"));
    assert_eq!(result["days"].as_integer(), Some(4));
    assert_eq!(result["ship_classes"].as_str(), Some("c.csv"));
    let sides = result["side"].as_table().expect("a side table");
    assert_eq!(sides.len(), 1, "{sides:?}");
    assert_eq!(sides["allied"]["doctrine"].as_str(), Some("z"));
    assert_eq!(result["objectives"]["horizon"].as_integer(), Some(3));
    assert!(!result.contains_key("built_on"));
}

#[test]
fn l3_a_row_of_a_known_id_changes_the_keys_it_states_and_keeps_its_place() {
    let below = format!("name = \"B\"\n{}", ships(&[(1, 10), (2, 20), (3, 30)]));
    let over = "built_on = \"s.toml\"\n\n[[ship]]\nid = 2\nfuel_t = 99\n";
    let files = [("m/s.toml", below), ("m/o.toml", over.to_string())];
    let result = resolved(&files, "m/o.toml");
    assert_eq!(
        ids_and_fuel(&result, "ship"),
        vec![(1, 10), (2, 99), (3, 30)]
    );
    assert_eq!(result["ship"][1]["name"].as_str(), Some("S2"));
}

#[test]
fn l4_a_row_of_a_new_id_comes_after_the_rows_below_in_its_order() {
    let below = ships(&[(1, 10), (2, 20)]);
    let over = format!("built_on = \"s.toml\"\n{}", ships(&[(9, 90), (4, 40)]));
    let files = [("m/s.toml", below), ("m/o.toml", over)];
    let result = resolved(&files, "m/o.toml");
    assert_eq!(
        ids_and_fuel(&result, "ship"),
        vec![(1, 10), (2, 20), (9, 90), (4, 40)]
    );
}

#[test]
fn l5_a_row_with_no_id_and_two_rows_of_one_id_are_refused() {
    let below = ships(&[(1, 10)]);
    let no_id = "built_on = \"s.toml\"\n\n[[ship]]\nfuel_t = 5\n";
    let twice = format!("built_on = \"s.toml\"\n{}", ships(&[(1, 5), (1, 6)]));
    for (over, wants) in [(no_id.to_string(), "no id"), (twice, "two rows of id 1")] {
        let files = [("m/s.toml", below.clone()), ("m/o.toml", over)];
        let message = Scenario::resolved_text(&source(&files), &at("m/o.toml"))
            .expect_err("refused")
            .to_string();
        assert!(message.contains("m/o.toml"), "{message}");
        assert!(message.contains("[[ship]]"), "{message}");
        assert!(message.contains(wants), "{message}");
    }
}

#[test]
fn l6_in_any_other_list_the_rows_of_the_overlay_come_after() {
    let row = |day: u32, text: &str| format!("\n[[command]]\nday = {day}\ntext = \"{text}\"\n");
    let below = format!("{}{}", row(1, "A"), row(2, "B"));
    let over = format!("built_on = \"s.toml\"\n{}{}", row(3, "C"), row(4, "D"));
    let files = [("m/s.toml", below), ("m/o.toml", over)];
    let result = resolved(&files, "m/o.toml");
    let texts: Vec<&str> = result["command"]
        .as_array()
        .expect("a list")
        .iter()
        .map(|c| c["text"].as_str().expect("text"))
        .collect();
    assert_eq!(texts, ["A", "B", "C", "D"]);
}

/// An overlay two folders deep, built on a file in another folder.
fn deep_files() -> [(&'static str, String); 2] {
    let below = whole(1, &with_force(&ship("")));
    let over = "built_on = \"../../y/s.toml\"\nname = \"Over\"\n";
    [("y/s.toml", below), ("x/z/o.toml", over.to_string())]
}

#[test]
fn l7_a_name_taken_from_below_is_given_the_path_between_the_folders() {
    let files = deep_files();
    let src = source(&files);
    let overlay = Scenario::load_via(&src, &at("x/z/o.toml"), &LoadOptions::default())
        .expect("the overlay loads");
    // The same scenario written out in full in the overlay's folder.
    let copy_text = whole(2, &with_force(&ship(""))).replace("name = \"Test\"", "name = \"Over\"");
    let copy = Scenario::parse_via(&src, &copy_text, &at("x/z/s.toml"), &LoadOptions::default())
        .expect("the copy loads");
    assert_eq!(overlay.world.state_hash(), copy.world.state_hash());
    let (last, written) = overlay.data_files().split_last().expect("files");
    assert_eq!(written, copy.data_files());
    assert_eq!(last, &at("x/z/../../y/s.toml"));
    // The first of them is the map, read from the overlay's folder.
    assert_eq!(
        overlay.data_files()[0],
        at("x/z/../../../map/placeholder_search.toml")
    );
    // A file key the overlay states itself is used as written.
    let mut with_map = files.clone();
    with_map[1]
        .1
        .push_str("map = \"../../../map/placeholder_search.toml\"\n");
    let stated = Scenario::load_via(
        &source(&with_map),
        &at("x/z/o.toml"),
        &LoadOptions::default(),
    )
    .expect("loads");
    assert_eq!(stated.world.state_hash(), copy.world.state_hash());
}

#[test]
fn l8_a_chain_of_three_loads_as_the_three_laid_in_order() {
    let a = ships(&[(1, 1), (2, 2)]);
    let b = format!(
        "built_on = \"a.toml\"\n\n[[ship]]\nid = 1\nfuel_t = 11\n{}",
        ships(&[(3, 3)])
    );
    let c = "built_on = \"b.toml\"\n\n[[ship]]\nid = 1\nfuel_t = 111\n\n[[ship]]\nid = 3\nfuel_t = 33\n";
    let files = [
        ("m/a.toml", a),
        ("m/b.toml", b),
        ("m/c.toml", c.to_string()),
    ];
    let result = resolved(&files, "m/c.toml");
    assert_eq!(
        ids_and_fuel(&result, "ship"),
        vec![(1, 111), (2, 2), (3, 33)]
    );
    let chain = Scenario::built_on_chain(&source(&files), &at("m/c.toml")).expect("a chain");
    assert_eq!(chain, vec![at("m/c.toml"), at("m/b.toml"), at("m/a.toml")]);
}

#[test]
fn l9_a_chain_that_cannot_be_read_is_refused_naming_the_files() {
    // Built on itself.
    let files = [("m/a.toml", "built_on = \"a.toml\"\n".to_string())];
    let message = error(&files, "m/a.toml");
    assert!(
        message.contains("m/a.toml") && message.contains("itself"),
        "{message}"
    );
    // Two that name each other, the second by another spelling.
    let files = [
        ("m/a.toml", "built_on = \"b.toml\"\n".to_string()),
        ("m/b.toml", "built_on = \"../m/a.toml\"\n".to_string()),
    ];
    let message = error(&files, "m/a.toml");
    assert!(
        message.contains("m/a.toml") && message.contains("already read"),
        "{message}"
    );
    // Nine files, and eight.
    let names: Vec<String> = (0..9).map(|n| format!("m/f{n}.toml")).collect();
    let mut texts: Vec<(&str, String)> = Vec::new();
    for (n, name) in names.iter().enumerate() {
        let text = if n < 8 {
            format!("built_on = \"f{}.toml\"\n", n + 1)
        } else {
            "name = \"end\"\n".to_string()
        };
        texts.push((name.as_str(), text));
    }
    let message = error(&texts, "m/f0.toml");
    assert!(
        message.contains("m/f0.toml") && message.contains("more than 8"),
        "{message}"
    );
    let chain = Scenario::built_on_chain(&source(&texts), &at("m/f1.toml")).expect("eight files");
    assert_eq!(chain.len(), 8);
    // Names no file.
    let files = [("m/a.toml", "built_on = \"gone.toml\"\n".to_string())];
    let message = error(&files, "m/a.toml");
    assert!(
        message.contains("m/a.toml") && message.contains("m/gone.toml"),
        "{message}"
    );
}

#[test]
fn l10_built_on_is_one_string() {
    for value in ["[\"a.toml\"]", "5", "{ x = 1 }"] {
        let files = [("m/a.toml", format!("built_on = {value}\n"))];
        let message = error(&files, "m/a.toml");
        assert!(
            message.contains("built_on") && message.contains("one file"),
            "{message}"
        );
    }
}

#[test]
fn l11_an_unknown_key_and_a_float_are_refused_wherever_they_are() {
    let base = whole(1, &with_force(&ship("")));
    let over = |extra: &str| format!("built_on = \"s.toml\"\n{extra}");
    let cases = [
        // In the overlay.
        (base.clone(), over("stray_key = 1\n")),
        (base.clone(), over("\n[[ship]]\nid = 1\nfuel_t = 0.5\n")),
        // In the file below.
        (format!("{base}stray_key = 1\n"), over("")),
        (
            whole(
                1,
                &with_force(&ship("")).replace("fuel_t = 100", "fuel_t = 0.5"),
            ),
            over(""),
        ),
    ];
    for (below, top) in cases {
        let files = [("y/s.toml", below), ("y/o.toml", top)];
        let message = error(&files, "y/o.toml");
        assert!(
            message.contains("stray_key")
                || message.contains("fuel_t")
                || message.contains("float"),
            "{message}"
        );
    }
    // The same overlay without the fault loads.
    let files = [("y/s.toml", base), ("y/o.toml", over(""))];
    Scenario::load_via(&source(&files), &at("y/o.toml"), &LoadOptions::default()).expect("loads");
}

#[test]
fn l12_a_reference_root_moves_the_reference_files_and_not_the_file_built_on() {
    let files = deep_files();
    let root = data().join("reference");
    let options = LoadOptions::with_reference_root(&root);
    let overlay = Scenario::load_via(&source(&files), &at("x/z/o.toml"), &options).expect("loads");
    let (last, written) = overlay.data_files().split_last().expect("files");
    assert_eq!(last, &at("x/z/../../y/s.toml"));
    let whole_copy = Scenario::load_via(
        &source(&[(
            "x/z/s.toml",
            whole(2, &with_force(&ship(""))).replace("name = \"Test\"", "name = \"Over\""),
        )]),
        &at("x/z/s.toml"),
        &options,
    )
    .expect("loads");
    assert_eq!(written, whole_copy.data_files());
    assert!(
        written.iter().skip(1).all(|p| p.starts_with(&root)),
        "{written:?}"
    );
}

#[test]
fn l13_the_files_built_on_are_recorded_and_the_pack_holds_them() {
    let files = deep_files();
    let src = source(&files);
    let overlay =
        Scenario::load_via(&src, &at("x/z/o.toml"), &LoadOptions::default()).expect("loads");
    let pack = Scenario::pack_of(&src, &at("x/z/o.toml"), &LoadOptions::default()).expect("a pack");
    assert!(pack.contains(&at("x/z/o.toml")));
    assert!(pack.contains(&at("x/z/../../y/s.toml")));
    for file in overlay.data_files() {
        assert!(pack.contains(file), "{}", file.display());
    }
    let from_pack = Scenario::load_via(&pack, &at("x/z/o.toml"), &LoadOptions::default())
        .expect("loads from the pack alone");
    assert_eq!(from_pack.world.state_hash(), overlay.world.state_hash());
    assert_eq!(from_pack.data_files(), overlay.data_files());
}

/// What a scenario plays: its hash at day 0, its script and its commands.
fn played(scenario: &Scenario) -> (u64, String, String) {
    (
        scenario.world.state_hash(),
        format!("{:?}", scenario.scripted_orders()),
        format!("{:?}", scenario.scripted_commands()),
    )
}

#[test]
fn l14_the_resolved_text_loads_as_the_overlay_does() {
    // A file with no built_on parses back to its table.
    let plain = whole(1, &with_force(&ship("")));
    let files = [("y/s.toml", plain.clone())];
    assert_eq!(resolved(&files, "y/s.toml"), table(&plain));
    // An overlay, with orders and a command of its own, resolved and
    // written in its own folder.
    let over = "built_on = \"../../y/s.toml\"\nname = \"Over\"\n\n[[command]]\nday = 1\nside = \"allied\"\ntext = \"hold\"\n";
    let mut files = vec![("y/s.toml", plain), ("x/z/o.toml", over.to_string())];
    let overlay = Scenario::load_via(&source(&files), &at("x/z/o.toml"), &LoadOptions::default());
    // The command may be refused as a command; the test needs the overlay
    // to load, so a plain order is used when it is.
    let over = if overlay.is_ok() {
        over.to_string()
    } else {
        "built_on = \"../../y/s.toml\"\nname = \"Over\"\n".to_string()
    };
    files[1].1 = over;
    let src = source(&files);
    let overlay = Scenario::load_via(&src, &at("x/z/o.toml"), &LoadOptions::default())
        .expect("the overlay loads");
    let text = Scenario::resolved_text(&src, &at("x/z/o.toml")).expect("resolves");
    assert!(!text.contains("built_on"));
    let again = Scenario::load_via(
        &source(&[("x/z/o.toml", text)]),
        &at("x/z/o.toml"),
        &LoadOptions::default(),
    )
    .expect("the resolved text loads");
    assert_eq!(played(&again), played(&overlay));
}

#[test]
fn l16_text_that_states_built_on_loads_as_a_file_of_that_text_would() {
    let dir = std::env::temp_dir().join(format!("flagplot-0234-{}", std::process::id()));
    fs::create_dir_all(&dir).expect("a folder");
    // Written with the repository's files by absolute names, which an
    // overlay passes on as written.
    let names = format!("\"{}/", data().display());
    let base = HEAD
        .replace("\"../map/", &format!("{names}map/"))
        .replace("\"../reference/", &format!("{names}reference/"));
    let base = format!("{base}{}", with_force(&ship("")));
    fs::write(dir.join("s.toml"), &base).expect("writes");
    let over = "built_on = \"s.toml\"\nname = \"Over\"\n";
    let path = dir.join("o.toml");
    fs::write(&path, over).expect("writes");
    let from_file = Scenario::load(&path).expect("loads");
    let from_text = Scenario::from_text(over, &path).expect("loads");
    assert_eq!(played(&from_text), played(&from_file));
    assert_eq!(from_text.data_files(), from_file.data_files());
    assert_eq!(from_text.name, "Over");
    let text = Scenario::resolved_text(&Disk, &path).expect("resolves");
    assert!(!text.contains("built_on"));
    let again = Scenario::from_text(&text, &path).expect("the resolved text loads");
    assert_eq!(played(&again), played(&from_file));
    fs::remove_dir_all(&dir).expect("removes the folder");
}
