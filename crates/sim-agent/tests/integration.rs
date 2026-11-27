//! Every integration test of this package, as one test program (see `Cargo.toml`).
//! Each file is a module; the files are unchanged but for how they reach `support`.

mod support;

mod act;
mod advance;
mod aircraft_fire;
mod airfield_damage;
mod amendments;
mod area_seat;
mod available_from;
mod choices;
mod claims;
mod command_picture;
mod command_tools;
mod cover;
mod deny_intent;
mod destroy_force;
mod engaged;
mod experience;
mod falling_back;
mod findings;
mod forces_that_arrive;
mod forming;
mod fuelling_lack;
mod golden;
mod ground;
mod hold_intent;
mod holds_and_preview;
mod intelligence_tool;
mod interrupts;
mod land_units;
mod leaks;
mod legibility;
mod march;
mod measure;
mod men_replacements;
mod neutralise_intent;
mod numbering;
mod objectives;
mod orders_inside_proof;
mod orders_kept_for_dusk;
mod patrol_tools;
mod plan_order;
mod play;
mod preparing;
mod pursuit;
mod queued_numbers;
mod queued_take_back;
mod rebase;
mod same_day_force;
mod scenarios;
mod seat;
mod seat_mends;
mod shells;
mod skills;
mod supply_lines;
mod told_ashore;
mod told_of_arrivals;
mod validator;
mod warnings;
mod wording2;
mod works;
mod would_strike;
mod yard;

/// `autotests = false` means a file with no `mod` line here would never run, and
/// nothing would say so. This names every top-level `.rs` file of `sim-agent`'s `tests/` that
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
