//! The staff reads a side's view and nothing else (decision 0107, item 3):
//! a lint over the crate's own source, as `sim-agent`'s `leaks.rs` has.

/// Code lines of every source file of the crate, comments left out, may not
/// name the world, the umpire's view, the truth, beliefs or the full event
/// log, take or join a side's half of a picture of a world (decision 0171),
/// read files or the clock, use the hash collections or floating point,
/// or `unsafe`. Its only dependency is `sim-core`.
#[test]
fn the_staff_takes_a_side_view_and_nothing_else() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let manifest = std::fs::read_to_string(root.join("Cargo.toml")).expect("the manifest reads");
    let normal: &str = manifest.split("[dev-dependencies]").next().unwrap();
    for banned in ["sim-data", "sim-cli", "sim-agent"] {
        assert!(
            !normal.contains(banned),
            "{banned} is a dependency of the staff"
        );
    }
    let banned = [
        "World",
        "world",
        "umpire",
        "at_staff_step",
        "EventLog",
        "TruthState",
        "BeliefState",
        "IdAllocator",
        // A side's own half of a picture, and the joining of two (decision
        // 0171): the staff reads views, and a half is not for it.
        "own_half",
        "from_halves",
        "PictureHalf",
        "Hasher",
        "std::fs",
        "std::net",
        "SystemTime",
        "Instant",
        "HashMap",
        "HashSet",
        "f32",
        "f64",
        "unsafe",
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
            }
        }
    }
    assert!(files > 0);
}
