//! A trial reads a side's view, what the side believed and the trial's
//! figures, and nothing else of a live game (decision 0174, 1, and decision
//! 0161, 6); it hands in commands to, and resolves, only a world it built
//! (decision 0175, "What it costs"): a lint over the crate's own source, as
//! the staff's `source.rs` and the agent's `leaks.rs` have.

/// Code lines of every source file of the crate, comments left out.
///
/// - **A world** may be named only to build one from a picture and to keep
///   the world built: the word `World`, alone, is allowed on a line that
///   imports it, builds one with `World::from_picture`, names it as what
///   that call returns, or names it as the field `world` of a world played.
///   No function takes a world: every world the crate holds it built.
/// - **What it does to a world** is what a game does to it between days:
///   it hands in commands (`submit_commands`) and resolves a day with the
///   staffs (`resolve_day_staffed`), and reads the world's day, its hash
///   and what the staffs handed in. Every other way into a world is banned:
///   orders and staff decisions handed in, a day worked in its parts, a
///   day resolved with a watcher, a copy at the staff's step or as if
///   closed, the hashes of past days, the events and the orders of the
///   book.
/// - **Banned outright:** the umpire and his key; the whole picture of a
///   live world and the rebuild on a live seed; a view taken from a world
///   (`side_view`, `at_seat`, `known_to`, `at_staff_step`), since a trial is
///   handed the view and is never given the live world to take one from;
///   the truth state, the belief state and the event log of a world; reading
///   files or the clock; the hash collections and floating point; `unsafe`.
/// - **Its dependencies** are `sim-core` and `sim-staff` only; `sim-data`
///   only for its tests.
#[test]
fn a_trial_takes_a_side_view_and_nothing_else_of_a_live_game() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let manifest = std::fs::read_to_string(root.join("Cargo.toml")).expect("the manifest reads");
    let normal: &str = manifest
        .split("[dev-dependencies]")
        .next()
        .expect("the manifest has a first part");
    for banned in ["sim-data", "sim-cli", "sim-agent", "map-tool", "oob-"] {
        assert!(
            !normal.contains(banned),
            "{banned} is a dependency of the trial"
        );
    }
    let banned = [
        "umpire",
        "Umpire",
        "whole_picture",
        "rebuild_from_picture",
        "side_view",
        "at_seat",
        "known_to",
        "at_staff_step",
        "World::new",
        "TruthState",
        "BeliefState",
        "EventLog",
        "events_all",
        "IdAllocator",
        "std::fs",
        "std::net",
        "std::env",
        "SystemTime",
        "Instant",
        "HashMap",
        "HashSet",
        "f32",
        "f64",
        "unsafe",
        // Ways into a world other than handing in commands and resolving a
        // day with the staffs (decision 0175).
        "submit_orders",
        "submit_staff",
        "open_day",
        "resolve_next_pulse",
        "close_day",
        "resolve_day_watched",
        "resolve_day(",
        "as_if_day_closed",
        "day_hashes",
        "events()",
        "orders()",
        "command_state(",
    ];
    let allowed_world = [
        "use sim_core::World;",
        "World::from_picture(",
        "Result<World, SimError>",
        "pub world: World,",
    ];
    let mut files = 0;
    let mut stack = vec![root.join("src")];
    while let Some(dir) = stack.pop() {
        for entry in std::fs::read_dir(&dir).expect("src reads") {
            let path = entry.expect("an entry").path();
            if path.is_dir() {
                stack.push(path);
                continue;
            }
            files += 1;
            let text = std::fs::read_to_string(&path).expect("a source reads");
            for (i, line) in text.lines().enumerate() {
                let code = line.trim_start();
                if code.starts_with("//") {
                    continue;
                }
                for word in banned {
                    assert!(
                        !code.contains(word),
                        "{}:{} uses {word}: {line}",
                        path.display(),
                        i + 1
                    );
                }
                // `World` as a word of its own, not the start of
                // `WorldPicture` or the like.
                let names_world = code.match_indices("World").any(|(at, _)| {
                    let after = code[at + "World".len()..].chars().next();
                    let before = code[..at].chars().next_back();
                    !after.is_some_and(|c| c.is_alphanumeric() || c == '_')
                        && !before.is_some_and(|c| c.is_alphanumeric() || c == '_')
                });
                if names_world {
                    assert!(
                        allowed_world.iter().any(|allowed| code.contains(allowed)),
                        "{}:{} names a world other than to build one from a picture: {line}",
                        path.display(),
                        i + 1
                    );
                }
            }
        }
    }
    assert!(files > 0);
}
