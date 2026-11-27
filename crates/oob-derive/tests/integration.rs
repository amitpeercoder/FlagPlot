//! Every integration test of this package, as one test program (see `Cargo.toml`).
//! Each file is a module; the files are unchanged but for how they reach `support`.

mod derive;
mod experience;
mod fire_rate;
mod submarines;
mod troops;

/// `autotests = false` means a file with no `mod` line here would never run, and
/// nothing would say so. This names every top-level `.rs` file of `oob-derive`'s `tests/` that
/// this file does not declare as a module.
#[test]
fn every_test_file_is_declared_as_a_module() {
    use std::collections::BTreeSet;
    let dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("tests");
    let root = std::fs::read_to_string(dir.join("integration.rs")).expect("the root file is read");
    let mut declared: BTreeSet<String> = BTreeSet::new();
    for line in root.lines().map(str::trim) {
        if let Some(rest) = line.strip_prefix("#[path = \"") {
            if let Some(path) = rest.split('"').next() {
                declared.insert(path.rsplit('/').next().unwrap_or(path).to_string());
            }
        }
        let rest = line
            .strip_prefix("pub mod ")
            .or_else(|| line.strip_prefix("mod "));
        if let Some(name) = rest.and_then(|r| r.strip_suffix(';')) {
            declared.insert(format!("{name}.rs"));
        }
    }
    let mut missing: BTreeSet<String> = BTreeSet::new();
    for entry in std::fs::read_dir(dir).expect("the directory is read") {
        let name = entry
            .expect("an entry is read")
            .file_name()
            .to_string_lossy()
            .into_owned();
        if name.ends_with(".rs") && name != "integration.rs" && !declared.contains(&name) {
            missing.insert(name);
        }
    }
    assert!(
        missing.is_empty(),
        "not declared as a module in integration.rs, so never run: {missing:?}"
    );
}
