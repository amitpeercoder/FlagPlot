//! A pack made under other versions, and a game file whose pin is not the
//! pack's, are refused by the door, which then has no game (decision 0229,
//! "The architect's rulings on pull request 732", the test owed as A; stage
//! 3's statement 13, held natively where the code is).
//!
//! # What the tests are to show
//!
//! Written by the architect in decision 0229, copied word for word:
//!
//! A. The versions and the pin, natively. A test in `crates/sim-web/tests`
//! (it needs no program of `sim-cli`): a pack whose `versions` has one of
//! the three changed is refused by `begin` and by `resume`, each in one
//! sentence that names both sets of versions, and the door then has no game
//! (`tools` gives the words of a door with none). And a game file whose pin
//! is not the pack's, made by changing one byte of one data file's text in
//! the pack after the file was begun, is refused by `resume` and the door
//! has no game. This is statement 13, held where the code is.
//!
//! No expected value is written here: the versions the pack is changed from
//! are the program's own, read from the pack the program made.

use serde_json::{json, Value};
use sim_cli::pack;
use sim_data::LoadOptions;
use sim_web::Door;

const ROOT: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../..");

fn scenario() -> String {
    format!("{ROOT}/data/scenarios/second-proof/05c_eastern_solomons.toml")
}

fn pack_text() -> String {
    let scenario = scenario();
    let pack = pack::pack_of(&[scenario.as_str()], &LoadOptions::default()).expect("a pack");
    pack::pack_text(&pack).expect("its text")
}

fn request() -> String {
    json!({"scenario": scenario(), "seat": "allied"}).to_string()
}

fn has_no_game(door: &Door) {
    let words = door.tools().expect_err("no game");
    assert!(words.contains("no game has begun"), "{words}");
}

/// A sentence: no full stop but the last character, if any.
fn one_sentence(words: &str) -> bool {
    !words.trim_end_matches('.').contains(". ") && !words.contains('\n')
}

#[test]
fn a_pack_of_other_versions_is_refused_by_begin_and_by_resume() {
    let text = pack_text();
    let good = text.as_bytes();
    let mut door = Door::new();
    door.begin(good, &request()).expect("begun on the pack");
    let file = door.game_file().expect("a game file");
    let own: Value = serde_json::from_str(&door.versions()).expect("versions");
    for key in ["rules", "state", "doctrine"] {
        let mut value: Value = serde_json::from_str(&text).expect("the pack is JSON");
        value["versions"][key] = match &own[key] {
            Value::Number(n) => json!(n.as_u64().expect("a number") + 1),
            other => json!(format!("{}-changed", other.as_str().expect("text"))),
        };
        let changed = value.to_string();
        let mut begins = Door::new();
        let refused = begins
            .begin(changed.as_bytes(), &request())
            .expect_err("begin refuses");
        let mut resumes = Door::new();
        let refused_resume = resumes
            .resume(changed.as_bytes(), &file)
            .expect_err("resume refuses");
        for (words, door) in [(refused, &begins), (refused_resume, &resumes)] {
            assert!(one_sentence(&words), "{key}: {words}");
            // Both sets of versions are named: the pack's changed one and the
            // build's own.
            for set in [&value["versions"], &own] {
                for k in ["rules", "state", "doctrine"] {
                    let shown = match &set[k] {
                        Value::String(s) => s.clone(),
                        n => n.to_string(),
                    };
                    assert!(words.contains(&shown), "{key}: {words} lacks {shown}");
                }
            }
            has_no_game(door);
        }
    }
}

#[test]
fn a_game_file_whose_pin_is_not_the_packs_is_refused_by_resume() {
    let text = pack_text();
    let mut door = Door::new();
    door.begin(text.as_bytes(), &request()).expect("begun");
    let file = door.game_file().expect("a game file");
    // One byte of one data file's text, changed after the file was begun.
    let mut value: Value = serde_json::from_str(&text).expect("the pack is JSON");
    let files = value["files"].as_object_mut().expect("files");
    let path = files
        .keys()
        .find(|k| k.ends_with("search.toml"))
        .expect("a data file")
        .clone();
    let mut bytes = files[&path].as_str().expect("text").as_bytes().to_vec();
    assert!(bytes[0].is_ascii());
    bytes[0] = if bytes[0] == b'x' { b'y' } else { b'x' };
    files.insert(
        path.clone(),
        json!(String::from_utf8(bytes).expect("still text")),
    );
    let mut resumes = Door::new();
    let words = resumes
        .resume(value.to_string().as_bytes(), &file)
        .expect_err("a changed data file is refused");
    assert!(words.contains("has changed since"), "{words}");
    assert!(words.contains("search.toml"), "{words}");
    has_no_game(&resumes);
}
