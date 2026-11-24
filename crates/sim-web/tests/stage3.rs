//! What stage 3 of decision 0229 holds natively: no `unsafe` written by hand
//! in `sim-web` (ruling 6), the lints of the workspace taken unchanged, and
//! the words of a refusal that must be the same where `usize` is 32 bits as
//! where it is 64 (ruling 30).
//!
//! # What the tests are to show
//!
//! Ruling 6, last paragraph, written by the architect in decision 0229: in
//! every case no `unsafe` is written by hand: a test reads `sim-web`'s source
//! with its comments and the one attribute `allow(unsafe_code)` taken out,
//! and fails on the word. (The first case of ruling 6 held in this build: the
//! workspace's `forbid` compiles with the binding macro, so `sim-web` takes
//! the workspace's lints unchanged and the attribute is nowhere; a second
//! test reads the crate's manifest to hold that.)
//!
//! Ruling 30, on a 32-bit `usize`: `take_back` reads `number` through `u32`
//! in both builds, which changes the native words for a number of 2 to the
//! 32nd or more and nothing else, so that the two builds answer alike. No
//! expected number is written here: a number that does not fit 32 bits is
//! answered as a number that is not a number of an entry is, with the same
//! words as a `number` that is missing.

use std::path::Path;

use serde_json::{json, Value};
use sim_cli::pack;
use sim_data::LoadOptions;
use sim_web::Door;

/// The code of a line: what stands before a comment.
fn code(line: &str) -> &str {
    line.split("//").next().unwrap_or("")
}

fn sources() -> Vec<(String, String)> {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
    let mut found = Vec::new();
    let mut stack = vec![root];
    while let Some(dir) = stack.pop() {
        for entry in std::fs::read_dir(&dir).expect("src reads") {
            let path = entry.expect("an entry").path();
            if path.is_dir() {
                stack.push(path);
            } else if path.extension().is_some_and(|e| e == "rs") {
                let text = std::fs::read_to_string(&path).expect("a source reads");
                found.push((path.display().to_string(), text));
            }
        }
    }
    found
}

#[test]
fn no_unsafe_is_written_by_hand_in_sim_web() {
    let files = sources();
    assert!(
        files.iter().any(|(path, _)| path.ends_with("binding.rs")),
        "the binding module is read"
    );
    for (path, text) in files {
        for (i, line) in text.lines().enumerate() {
            // The one attribute the second and third cases of ruling 6 allow.
            let line = code(line).replace("allow(unsafe_code)", "");
            assert!(
                !line.contains("unsafe"),
                "{path}:{} writes unsafe: {line}",
                i + 1
            );
        }
    }
}

#[test]
fn sim_web_takes_the_workspace_lints_unchanged() {
    let manifest =
        std::fs::read_to_string(Path::new(env!("CARGO_MANIFEST_DIR")).join("Cargo.toml"))
            .expect("the manifest reads");
    let lints: Vec<&str> = manifest
        .lines()
        .skip_while(|l| l.trim() != "[lints]")
        .skip(1)
        .take_while(|l| !l.trim_start().starts_with('['))
        .map(str::trim)
        .filter(|l| !l.is_empty())
        .collect();
    assert_eq!(
        lints,
        ["workspace = true"],
        "the crate restates no lint of the workspace"
    );
}

#[test]
fn a_number_of_two_to_the_thirty_second_or_more_is_refused_as_no_number() {
    let scenario = format!(
        "{}/../../data/scenarios/second-proof/05c_eastern_solomons.toml",
        env!("CARGO_MANIFEST_DIR")
    );
    let packed = pack::pack_of(&[scenario.as_str()], &LoadOptions::default()).expect("a pack");
    let packed = pack::pack_text(&packed).expect("its text");
    let mut door = Door::new();
    door.begin(
        packed.as_bytes(),
        &json!({"scenario": scenario, "seat": "allied"}).to_string(),
    )
    .expect("begun");
    let queued: Value =
        serde_json::from_str(&door.call("queued", "{}").expect("queued")).expect("JSON");
    let as_of = queued["ok"]["as_of"].clone();
    let mut refused = Vec::new();
    for number in [0u64, 1u64 << 32, (1u64 << 32) + 1, u64::MAX] {
        let answer = door
            .call(
                "take_back",
                &json!({"queue": "dusk", "number": number, "as_of": as_of}).to_string(),
            )
            .expect("an answer");
        let answer: Value = serde_json::from_str(&answer).expect("JSON");
        refused.push(answer["refused"].as_str().expect("a refusal").to_string());
    }
    // Number 0 is read as a number and judged by the queue; the others are
    // not numbers of an entry, and are refused in the words of a missing one.
    let missing = "\"number\" is needed, as the number of an entry you were shown";
    assert!(refused[1..].iter().all(|w| w == missing), "{refused:?}");
}
