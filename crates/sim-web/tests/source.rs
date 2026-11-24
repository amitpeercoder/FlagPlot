//! What `sim-web` may name, and what its door may take (decision 0229,
//! rulings 22 and 17; stage 2, statement 5b's second half).
//!
//! # What the tests are to show
//!
//! Written by the architect in decision 0229. Statement 5b, whose first half
//! (a second `begin` is refused) is held in `sim-cli`'s `tests/door.rs`,
//! where `sim-cli play` is built:
//!
//! 5b. A second `begin` on a door that has a game is refused; no function
//!    of the door takes a side but `begin` and `resume`.
//!
//! And ruling 22: `sim-web` stands where `sim-cli mcp` stands, the umpire's
//! side of a seat's door. Its source opens a game, calls the seat's tools
//! for the one side chosen at `begin`, and asks for the game file and, when
//! the game is over, the saved game. A test reads its source, as
//! `sim-agent`'s and `sim-staff`'s do, and fails on `umpire`, `into_world`,
//! `at_staff_step`, `transcript`, `handed_in`, `watch`, `standing`,
//! `seat_calls`, `state_hash` and `day_hashes` by name. A state hash changes
//! with what a seat cannot see (decision 0085), so the door never gives one
//! while a game is in play; the hashes of the days reach the outside only
//! inside the saved game, when the game is over, as the rule already
//! allows.

use std::path::{Path, PathBuf};

fn sources() -> Vec<(PathBuf, String)> {
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
                found.push((path, text));
            }
        }
    }
    found
}

/// The code of a line: what stands before a comment.
fn code(line: &str) -> &str {
    line.split("//").next().unwrap_or("")
}

#[test]
fn the_door_names_nothing_of_the_umpire() {
    let banned = [
        "umpire",
        "into_world",
        "at_staff_step",
        "transcript",
        "handed_in",
        "watch",
        "standing",
        "seat_calls",
        "state_hash",
        "day_hashes",
    ];
    let files = sources();
    assert!(files.len() >= 2, "the library and the program are read");
    for (path, text) in files {
        for (i, line) in text.lines().enumerate() {
            for word in banned {
                assert!(
                    !code(line).contains(word),
                    "{}:{} names {word}: {line}",
                    path.display(),
                    i + 1
                );
            }
        }
    }
}

#[test]
fn no_function_of_the_door_takes_a_side_but_begin_and_resume() {
    let (_, text) = sources()
        .into_iter()
        .find(|(path, _)| path.ends_with("lib.rs"))
        .expect("the library is read");
    let mut seen = Vec::new();
    // Each `pub fn` of the library, with its parameters up to the `{`.
    let mut rest = text.as_str();
    while let Some(at) = rest.find("pub fn ") {
        rest = &rest[at + "pub fn ".len()..];
        let name: String = rest
            .chars()
            .take_while(|c| c.is_alphanumeric() || *c == '_')
            .collect();
        let end = rest.find('{').expect("a body");
        let signature = &rest[..end];
        seen.push(name.clone());
        if name != "begin" && name != "resume" {
            for word in ["side", "Side", "seat", "Seat", "Post"] {
                assert!(
                    !signature.contains(word),
                    "{name} takes a side: {signature}"
                );
            }
        }
    }
    // The door's whole set, and the constructor.
    for expected in [
        "new",
        "versions",
        "begin",
        "tools",
        "call",
        "game_file",
        "resume",
        "saved_game",
    ] {
        assert!(seen.iter().any(|n| n == expected), "{expected} is read");
    }
    assert_eq!(seen.len(), 8, "no other public function: {seen:?}");
}
