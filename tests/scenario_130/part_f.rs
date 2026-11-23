//! Scenario 130, part F: the tests of its statements (see the doc comment
//! of the file above): what does not move.
//!
//! # What is compared
//!
//! Every staffed scenario file on file, played by its staffs on the live
//! figures (as scenario 106's `record::played` plays one, `common::load_live`):
//! every file directly under `data/scenarios` and one folder down, but the
//! frozen copies in `step2-frozen` and this scenario's own folder, whose
//! text has a `set_intent` command (a side with intents): 95 files. The
//! digest of a file is FNV-1a over the text of every decision each side's
//! staff handed in, day by day (`{:#?}` of `world.staff_handed_in()`), then
//! every event of the game (`{:?}`). **The daily hashes are not in it.**
//! Statement 24 says "hash": the state layout version was raised by the
//! architect's ruling A (the new field of a forces limit, `none_of`), so
//! every daily hash of every game moves, and the games are compared by the
//! orders, commands, reasons' words and events instead. (A reason's list of
//! limits is left out of the text it is written as: decision 0222, rule 5.)
//!
//! # Where the table of "before" digests comes from
//!
//! `BEFORE` below was taken again on Nov 23, 2026, by the lead, from a
//! worktree of main at 284a7614 (decision 0222, rule 1 merged: the reason of
//! a hold with no source, which moved the words of the Coral Sea, five-day,
//! Midway and table-hunger games and no order) with the same `digest`
//! module, after this branch merged main. It was first taken from the
//! commit 9bdc3062 (origin/limits-engine: the statements and the staffs
//! before the engine of decision 0222, rule 2 and what follows it), with
//! this file's own `digest` module copied into a temporary test there and the
//! files of that commit (so the second proof's and stage 2b's files are as
//! they were before they were given `none_of`). The worktree was removed
//! after. The digests are of the files as they were at that commit; the
//! comparison is of each file as it is on this branch.
//!
//! A file with no row in `BEFORE` (one added to the repository after this
//! was written) is passed over: there is nothing to compare it to. A change
//! that moves a game on purpose remakes its row, with `FLAGPILOT_130_TEXTS`
//! set to a folder to write each game's text into for a comparison (the
//! file's path, "/" as "__").
//!
//! # The statements
//!
//! - 7: the files whose games have a take or an island order (`take_place`
//!   or `control_island`) and a forces limit: the same digest.
//! - 8: the files with a forces limit on an order of another kind: listed
//!   by `statement_8`'s own run, each "moved" or "same", for the decision's
//!   "As built". Nothing is asserted of them.
//! - 24: every other staffed file: the same digest.
//!
//! What moved, and why, is in `MOVED` below.
//!
//! These tests play 95 games. Run them with `--release`.

#[allow(unused_imports)]
use super::common;

use std::collections::BTreeMap;
use std::sync::OnceLock;

// ---- the digest of a staffed game (shared by part F and the "before" run)

pub mod digest {
    use std::fmt::Write as _;
    use std::path::{Path, PathBuf};

    use sim_core::state::Side;
    use sim_staff::{as_staffs, staffs};

    use super::super::common;

    /// FNV-1a, 64 bits: a digest of a text that does not change with the
    /// compiler (as scenario 106's `record::fnv`).
    pub fn fnv(text: &str) -> u64 {
        let mut h: u64 = 0xcbf2_9ce4_8422_2325;
        for byte in text.bytes() {
            h ^= u64::from(byte);
            h = h.wrapping_mul(0x0100_0000_01b3);
        }
        h
    }

    /// The staffed scenario files on file: every `.toml` directly under
    /// `data/scenarios` and one folder down, but the frozen copies in
    /// `step2-frozen` and this scenario's own folder, whose text has a
    /// `set_intent` command (a side with intents). Paths relative to
    /// `data/scenarios`, in order.
    pub fn staffed_files() -> Vec<String> {
        let root = common::data_dir().join("scenarios");
        let mut found: Vec<String> = Vec::new();
        let mut visit = |dir: &Path, prefix: &str| {
            let mut names: Vec<_> = std::fs::read_dir(dir)
                .expect("the folder reads")
                .filter_map(Result::ok)
                .map(|e| e.path())
                .filter(|p| p.extension().is_some_and(|x| x == "toml"))
                .collect();
            names.sort();
            for path in names {
                let name = format!("{prefix}{}", path.file_name().unwrap().to_string_lossy());
                let text = std::fs::read_to_string(&path).expect("the file reads");
                if text.contains("kind = \"set_intent\"") {
                    found.push(name);
                }
            }
        };
        visit(&root, "");
        let mut folders: Vec<PathBuf> = std::fs::read_dir(&root)
            .expect("the folder reads")
            .filter_map(Result::ok)
            .map(|e| e.path())
            .filter(|p| p.is_dir())
            .collect();
        folders.sort();
        for folder in folders {
            let name = folder.file_name().unwrap().to_string_lossy().to_string();
            if name == "step2-frozen" || name == "a-limit-binds-what-it-names" {
                continue;
            }
            visit(&folder, &format!("{name}/"));
        }
        found.sort();
        found
    }

    /// A scenario file played by its staffs on the live figures for all its
    /// days: every decision each side's staff handed in, day by day, then
    /// every event of the game, as text (as scenario 106's `record::played`;
    /// the daily hashes are not in it). `Err` where the file does not load.
    pub fn played(file: &str) -> Result<String, String> {
        let path = common::data_dir().join("scenarios").join(file);
        let mut scenario = common::load_live(&path).map_err(|e| format!("{e}"))?;
        let held = staffs();
        let mut out = String::new();
        loop {
            let day = scenario.world.day() + 1;
            let summary = scenario
                .next_day_staffed(as_staffs(&held))
                .map_err(|e| format!("{e:?}"))?;
            if summary.is_none() {
                break;
            }
            for side in Side::ALL {
                let handed = scenario
                    .world
                    .staff_handed_in()
                    .get(&side)
                    .cloned()
                    .unwrap_or_default();
                writeln!(out, "day {day} {side:?} {handed:#?}").expect("a string writes");
            }
        }
        for event in scenario.world.events().iter() {
            writeln!(out, "{event:?}").expect("a string writes");
        }
        Ok(out)
    }

    /// The digest of each staffed file, played in parallel; `None` where the
    /// file did not load or play. With `FLAGPILOT_130_TEXTS` set to a folder,
    /// each text is written there (the file's path with "/" as "__").
    pub fn digests() -> Vec<(String, Option<u64>)> {
        let files = staffed_files();
        let texts = std::env::var("FLAGPILOT_130_TEXTS").ok();
        let mut out = Vec::new();
        for chunk in files.chunks(8) {
            let results: Vec<(String, Option<u64>)> = std::thread::scope(|scope| {
                let handles: Vec<_> = chunk
                    .iter()
                    .map(|file| {
                        let texts = texts.clone();
                        scope.spawn(move || {
                            let text = played(file);
                            if let (Some(dir), Ok(text)) = (&texts, &text) {
                                let name = file.replace('/', "__");
                                std::fs::write(Path::new(dir).join(name), text)
                                    .expect("the text writes");
                            }
                            (file.clone(), text.ok().map(|t| fnv(&t)))
                        })
                    })
                    .collect();
                handles
                    .into_iter()
                    .map(|h| h.join().expect("a game plays"))
                    .collect()
            });
            out.extend(results);
        }
        out
    }
}

/// The digests taken at 284a7614 (see above), in file order.
const BEFORE: &[(&str, u64)] = &[
    ("125_a_force_asks_leave_to_pursue.toml", 0xf97d80e74fc0fdba),
    (
        "129_submarines_the_staff_and_the_patrol.toml",
        0x359e9d429e8749d5,
    ),
    ("36_one_convoy.toml", 0x6e1422cadfbaf838),
    ("40a_one_pool_two_hexes.toml", 0x482d065978fe6caf),
    ("40b_one_stock_two_plans.toml", 0x72088dac6863b6f3),
    ("41a_hold_change_release.toml", 0x6fae0a6c8251b4b0),
    ("41b_an_override.toml", 0x086ee197215521f4),
    ("41c_cancelled_at_sea.toml", 0x781d6f6211cf794e),
    ("42a_a_shortage_stops_the_clock.toml", 0xc278f831a9ef8f08),
    ("42b_two_reasons_one_day.toml", 0xe5d4a75991f6cb60),
    ("42c_displaced_under_run.toml", 0x482d065978fe6caf),
    ("42d_saved_in_the_middle.toml", 0x371f7e9855638227),
    ("45_soak_ashore.toml", 0x10664b31ad2aa396),
    ("47_cover_from_the_air.toml", 0xe89b36b0b6b3b612),
    ("48_hold_a_place.toml", 0x969f970b5b6817b0),
    ("50_neutralise_a_base.toml", 0x4e9512a125f6894d),
    ("53_deny_an_area.toml", 0x1ddf42951d00fcfe),
    ("54_a_force_that_stands_by_night.toml", 0xd5aeb8b7f40d6286),
    ("55_a_carrier_force_from_an_end.toml", 0xa2a2331c2000e91d),
    ("56_an_escorted_run.toml", 0xfcc486c5eb29499f),
    ("57_going_back_to_rearm.toml", 0x4fe928ccd1403f11),
    ("58_a_carrier_force_closes.toml", 0xe20ff7647e820d43),
    ("59_a_search_kept_up.toml", 0x9779ce04e57c1461),
    ("61_commands_and_their_pictures.toml", 0xe20ff7647e820d43),
    ("62_reports_between_commands.toml", 0xeebfca8ff326978e),
    ("63_a_superiors_knowledge.toml", 0x42f5db929b48d6aa),
    ("64_commanders_and_initiative.toml", 0x7732b97507d3a784),
    (
        "65_a_day_from_which_a_force_may_be_used.toml",
        0xe3ac1ba461a3a7b8,
    ),
    ("66_units_made_ready.toml", 0xe09b972aea4c4d99),
    ("67_taking_a_place.toml", 0xfa44a1d1cc1f6272),
    ("68_ashore.toml", 0x43a73179b1f688e6),
    ("69_taken_and_held.toml", 0xc93dee735cb6b143),
    ("70_a_run_by_night.toml", 0x1da55ec6d0f529db),
    ("71_bombardment_from_the_sea.toml", 0x57563498ac3f08ea),
    ("72_reconnaissance.toml", 0x6c183057cc284d07),
    ("73_carrier_cover.toml", 0xa9d8bf1615d4103f),
    ("74_the_force_assembled.toml", 0x904bcc2adce81399),
    ("76_controlling_an_island.toml", 0x8ef21afffe9e6f7d),
    ("77_forces_with_men_aboard.toml", 0x3ea2d46764597d99),
    ("79_fed_before_it_is_taken.toml", 0x24c038e88b90eab5),
    ("80_a_take_that_waits.toml", 0x4d04d1c486494ac5),
    ("81_told_to_wait.toml", 0xc2b8d0659cf101fa),
    ("82_how_much_must_be_known.toml", 0xc531d09cd970a2c3),
    ("83_a_force_moved_to_look.toml", 0xc1463ebb32857737),
    ("85_a_nights_load_spread.toml", 0xacf1f2775d411fea),
    ("86_fed_with_the_lift.toml", 0xf5fa2a0e504c47ad),
    (
        "90_a_take_that_does_not_go_to_starve.toml",
        0xeb3b376cdc4c4c6d,
    ),
    (
        "carriers-serve-an-end/116_carriers_serve_an_end.toml",
        0x60728e019239c7f0,
    ),
    ("coral-sea/01a_coral_sea.toml", 0x661d4c5c16082bc5),
    ("coral-sea/01b_coral_sea.toml", 0x4544e7849882af2a),
    ("coral-sea/01c_coral_sea.toml", 0x783ee3d37b15be08),
    ("coral-sea/02a_coral_sea_oilers.toml", 0x661d4c5c16082bc5),
    ("coral-sea/02b_coral_sea_oilers.toml", 0x4544e7849882af2a),
    ("coral-sea/02c_coral_sea_oilers.toml", 0x783ee3d37b15be08),
    (
        "coral-sea/03a_coral_sea_both_oilers.toml",
        0x661d4c5c16082bc5,
    ),
    (
        "coral-sea/03b_coral_sea_both_oilers.toml",
        0x4544e7849882af2a,
    ),
    (
        "coral-sea/03c_coral_sea_both_oilers.toml",
        0x783ee3d37b15be08,
    ),
    ("five-days/01a_eastern_solomons.toml", 0xae1b70a84e669f81),
    ("five-days/01b_eastern_solomons.toml", 0xd154c525fa803b8d),
    ("five-days/01c_eastern_solomons.toml", 0x55b02ee573255e13),
    (
        "five-days/02a_eastern_solomons_oilers.toml",
        0xae1b70a84e669f81,
    ),
    (
        "five-days/02b_eastern_solomons_oilers.toml",
        0xd154c525fa803b8d,
    ),
    (
        "five-days/02c_eastern_solomons_oilers.toml",
        0x55b02ee573255e13,
    ),
    (
        "forces-that-arrive/111b_carrier_force_intent.toml",
        0x1ec0899d34d36025,
    ),
    (
        "lift-turned-back/107_a_lift_turned_back.toml",
        0xab04bf27d7ca1e06,
    ),
    (
        "men-ashore-no-more-coming/109_men_ashore_with_no_more_coming.toml",
        0x9a0d7acdb1a1d464,
    ),
    ("midway/01a_midway.toml", 0x5f6de67090e3801a),
    ("midway/01b_midway.toml", 0x747f2a173872fe49),
    ("midway/01c_midway.toml", 0x79f2afc2f0dc4f81),
    (
        "orders-inside-proof/36_one_convoy_on.toml",
        0x6e1422cadfbaf838,
    ),
    ("second-proof/05a_eastern_solomons.toml", 0x72dc491607fd653b),
    ("second-proof/05b_eastern_solomons.toml", 0xc17b097e78a8dc64),
    ("second-proof/05d_eastern_solomons.toml", 0xaf615a412e4f96e1),
    ("second-proof/07a_week.toml", 0xb8174bbd4b8d4333),
    ("second-proof/07b_week.toml", 0x5d4f3b6557da90f9),
    ("second-proof/07d_week.toml", 0xfbb86d23b04c6fa7),
    ("second-proof/07m_model_week.toml", 0x01028c43b742c16b),
    (
        "stage-2b-proof/05a_eastern_solomons.toml",
        0xe2396ee2ad84bb8e,
    ),
    (
        "stage-2b-proof/05b_eastern_solomons_off.toml",
        0x72dc491607fd653b,
    ),
    (
        "stage-2b-proof/05c_eastern_solomons_bold.toml",
        0xe2396ee2ad84bb8e,
    ),
    ("stage-2b-proof/07a_week.toml", 0x485071c7e65da537),
    ("stage-3-proof/07a_slice.toml", 0x3117fa6015e5aeda),
    ("stage-3-proof/07b_slice.toml", 0x65b57ae5bd373ab9),
    ("stage-3-proof/07c_slice.toml", 0xeb154cd9f0856a51),
    (
        "station-from-the-sea/108_a_station_kept_from_the_sea.toml",
        0x130a56e3fb23efd0,
    ),
    ("table-hunger/01b_midway.toml", 0x5444e0b1bfe5d55f),
    ("table-hunger/01c_midway.toml", 0x79f2afc2f0dc4f81),
    (
        "table-hunger/02b_eastern_solomons_oilers.toml",
        0xb58016163247dfca,
    ),
    (
        "table-hunger/02c_eastern_solomons_oilers.toml",
        0xd62d809290e02ca3,
    ),
    (
        "table-hunger/03b_coral_sea_both_oilers.toml",
        0x2a1045306b879843,
    ),
    (
        "table-hunger/03c_coral_sea_both_oilers.toml",
        0x783ee3d37b15be08,
    ),
    ("table-hunger/07b_slice.toml", 0x8c82b9e0a37ea641),
    ("table-hunger/07c_slice.toml", 0x5bf9105d4d7d4976),
    (
        "the-way-a-force-takes/117_the_way_a_force_takes.toml",
        0xba638023f17e38ef,
    ),
    (
        "what-to-strike-first/106_what_to_strike_first.toml",
        0x9f4022d97eb6b46e,
    ),
];

/// The files that move, and the first line of the game's text that differs
/// (taken on Nov 23, 2026, with `FLAGPILOT_130_TEXTS`):
///
/// - `55_a_carrier_force_from_an_end.toml`: day 1, the Allied staff's
///   reason for the carrier task of task force 1, which the deny's forces
///   limit names. Before, the force was given its task where it lay, (0,0),
///   with the reason "task_force:1 is on the carrier mission with 1
///   carrier(s) afloat ... and lies at (0,0)" and the binding "is named in
///   the forces limit of intent:1"; now the carrier pass stations it as a
///   free force and says so ("task_force:1 lies at (0,0) with bombers of a
///   least normal strike radius of 250 nm", "the candidates ... 117"). The
///   station chosen is (0,0) again, so no order and no later event moves but
///   the reasons' words.
/// - `second-proof/07a_week.toml`, `07d_week.toml`, `07m_model_week.toml` and
///   `stage-2b-proof/07a_week.toml`: day 15, the Japanese staff's first
///   handing in (first differing line: an intent number, 6 before and 8
///   now, in the order to air group 6's group). The Japanese deny names the
///   carrier forces 3 and 4, which before were given their task at the hex
///   they lay in ((9,-19) and (7,-18)) and are now stationed by the carrier
///   pass, both at (0,-7); the two tasks use the intent numbers 6 and 7 and
///   shift the numbers of the tasks formed after them.
/// - `second-proof/07b_week.toml` does not move: its two carrier forces each
///   have the commander's own `operate_carriers` order (intents 4 and 5), so
///   they keep their hexes.
pub const MOVED: &[&str] = &[
    "55_a_carrier_force_from_an_end.toml",
    "second-proof/07a_week.toml",
    "second-proof/07d_week.toml",
    "second-proof/07m_model_week.toml",
    "stage-2b-proof/07a_week.toml",
];

/// What a file's commands say: whether it has a take or an island order, and
/// whether it has a forces limit on an order of another kind, or on a take
/// or an island (a limit on an intent the file does not state, one the staff
/// forms, counts as of another kind).
struct Commands {
    take_or_island: bool,
    forces_limit: bool,
    forces_limit_on_other: bool,
    /// What those limits on other kinds of order name, as the keys of the
    /// command: ships, task_forces, air_groups, land_units, submarines,
    /// source.
    named_on_other: std::collections::BTreeSet<&'static str>,
}

fn commands_of(file: &str) -> Commands {
    let path = common::data_dir().join("scenarios").join(file);
    let text = std::fs::read_to_string(path).expect("the file reads");
    let doc: toml::Table = text.parse().expect("the file is TOML");
    let commands = doc
        .get("command")
        .and_then(toml::Value::as_array)
        .cloned()
        .unwrap_or_default();
    let get =
        |c: &toml::Value, key: &str| c.get(key).and_then(toml::Value::as_str).map(str::to_string);
    let mut kinds: BTreeMap<(String, i64), String> = BTreeMap::new();
    for c in &commands {
        if get(c, "kind").as_deref() == Some("set_intent") {
            kinds.insert(
                (
                    get(c, "side").unwrap_or_default(),
                    c.get("intent")
                        .and_then(toml::Value::as_integer)
                        .unwrap_or(0),
                ),
                get(c, "intent_kind").unwrap_or_else(|| "keep_supplied".to_string()),
            );
        }
    }
    let is_take = |kind: &str| kind == "take_place" || kind == "control_island";
    let mut out = Commands {
        take_or_island: kinds.values().any(|k| is_take(k)),
        forces_limit: false,
        forces_limit_on_other: false,
        named_on_other: std::collections::BTreeSet::new(),
    };
    for c in &commands {
        if get(c, "kind").as_deref() == Some("set_limit")
            && get(c, "limit").as_deref() == Some("forces")
            && c.get("cleared").is_none()
        {
            out.forces_limit = true;
            let side = get(c, "side").unwrap_or_default();
            let intent = c
                .get("intent")
                .and_then(toml::Value::as_integer)
                .unwrap_or(0);
            match kinds.get(&(side, intent)) {
                Some(kind) if is_take(kind) => {}
                _ => {
                    out.forces_limit_on_other = true;
                    for key in [
                        "ships",
                        "task_forces",
                        "air_groups",
                        "land_units",
                        "submarines",
                        "source",
                        "none_of",
                    ] {
                        if c.get(key)
                            .is_some_and(|v| v.as_array().is_none_or(|a| !a.is_empty()))
                        {
                            out.named_on_other.insert(key);
                        }
                    }
                }
            }
        }
    }
    out
}

/// Every staffed file played once, shared by the tests.
fn digests() -> &'static BTreeMap<String, Option<u64>> {
    static DIGESTS: OnceLock<BTreeMap<String, Option<u64>>> = OnceLock::new();
    DIGESTS.get_or_init(|| digest::digests().into_iter().collect())
}

/// The files with a row in `BEFORE`, with whether their game is as it was.
fn compared() -> Vec<(&'static str, bool)> {
    let after = digests();
    BEFORE
        .iter()
        .filter(|(file, _)| after.contains_key(*file))
        .map(|(file, before)| (*file, after[*file] == Some(*before)))
        .collect()
}

/// The table has a row for every staffed file on file when it was taken, and
/// every file of it still loads and plays.
#[test]
fn the_table_is_of_the_staffed_files() {
    let files = digest::staffed_files();
    for (file, _) in BEFORE {
        assert!(
            files.iter().any(|f| f == file),
            "{file} is in the table and no longer a staffed file"
        );
    }
    for (file, digest) in digests() {
        assert!(digest.is_some(), "{file} does not play");
    }
    assert_eq!(BEFORE.len(), 95);
}

/// Statement 7: under an order to take a place and an order to control an
/// island the staff forms, orders and says what it did before this rule:
/// every staffed file whose game has such an order and a forces limit has
/// the digest it had.
#[test]
fn statement_7_take_and_island_games_are_as_they_were() {
    let mut checked = 0;
    for (file, same) in compared() {
        let commands = commands_of(file);
        if commands.take_or_island && commands.forces_limit {
            assert!(same, "{file} moved");
            checked += 1;
        }
    }
    println!("statement 7: {checked} files checked");
    assert!(checked > 0);
}

/// Statement 8: the staffed files whose forces limits are on other kinds of
/// order, each with whether its game moved. Printed (run with
/// `--nocapture`); nothing is asserted of the list, but a file that moves
/// must be one `MOVED` names.
#[test]
fn statement_8_the_files_with_a_forces_limit_on_other_orders() {
    let mut listed = 0;
    for (file, same) in compared() {
        let commands = commands_of(file);
        if commands.forces_limit_on_other {
            println!(
                "statement 8: {file}: {} (the limits name {:?})",
                if same { "same" } else { "moved" },
                commands.named_on_other
            );
            listed += 1;
            assert_eq!(!same, MOVED.contains(&file), "{file}");
        }
    }
    println!("statement 8: {listed} files");
}

/// Statement 24: a game with no forces limit has the orders, commands,
/// reasons' words and events it had: every other staffed file, with no
/// forces limit on any order, has its digest. (The daily hashes are left
/// out: they all moved with the state's layout.)
#[test]
fn statement_24_every_other_staffed_file_is_as_it_was() {
    let mut checked = 0;
    for (file, same) in compared() {
        if !commands_of(file).forces_limit {
            assert!(same, "{file} moved");
            checked += 1;
        }
    }
    println!("statement 24: {checked} files checked");
    assert!(checked > 0);
}
