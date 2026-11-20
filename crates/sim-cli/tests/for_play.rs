//! A scenario for play (decision 0187, rule 22): `run` names its scale as its
//! own, `batch` says so in its header, and `validate` will not judge it.
//!
//! The scenario for play is a copy of scenario 5 with `for_play = true` under
//! `[objectives]`, written in a folder of its own under `data/scenarios` (so
//! that the loops over `data/scenarios/*.toml` never see it) with its
//! relative paths made one level deeper, and removed when the test ends.

use std::fs;
use std::path::PathBuf;
use std::process::{Command, Output};

use serde_json::Value;

const ROOT: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../..");
const OF_RECORD: &str = "data/scenarios/05_eastern_solomons.toml";

fn binary(args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_sim-cli"))
        .args(args)
        .current_dir(ROOT)
        .output()
        .expect("the binary starts")
}

/// The copy for play, and the folder that holds it; both go on drop.
struct ForPlay {
    folder: PathBuf,
    file: String,
}

impl ForPlay {
    fn new(name: &str) -> ForPlay {
        let folder = PathBuf::from(ROOT)
            .join("data/scenarios")
            .join(format!("tmp-for-play-{}-{name}", std::process::id()));
        let _ = fs::remove_dir_all(&folder);
        fs::create_dir_all(&folder).expect("make the folder");
        let text = fs::read_to_string(PathBuf::from(ROOT).join(OF_RECORD)).expect("scenario 5");
        let text = text.replace("= \"../", "= \"../../");
        assert_eq!(text.matches("\n[objectives]\n").count(), 1);
        let text = text.replace("\n[objectives]\n", "\n[objectives]\nfor_play = true\n");
        fs::write(folder.join("for_play.toml"), text).expect("write the copy");
        let file = format!(
            "data/scenarios/tmp-for-play-{}-{name}/for_play.toml",
            std::process::id()
        );
        ForPlay { folder, file }
    }
}

impl Drop for ForPlay {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.folder);
    }
}

fn stdout(o: &Output) -> String {
    String::from_utf8_lossy(&o.stdout).into_owned()
}

fn stderr(o: &Output) -> String {
    String::from_utf8_lossy(&o.stderr).into_owned()
}

fn batch_of(scenario: &str, name: &str) -> (PathBuf, Value) {
    let out = std::env::temp_dir().join(format!(
        "flagplot-for-play-{}-{name}.json",
        std::process::id()
    ));
    let result = binary(&[
        "batch",
        scenario,
        "--runs",
        "2",
        "--label",
        "for-play-test",
        "--out",
        &out.to_string_lossy(),
    ]);
    assert!(result.status.success(), "{}", stderr(&result));
    let value = serde_json::from_slice(&fs::read(&out).expect("batch file")).expect("JSON");
    (out, value)
}

#[test]
fn run_names_the_scale_of_a_scenario_for_play_as_its_own() {
    let play = ForPlay::new("run");
    let result = binary(&["run", &play.file]);
    assert!(result.status.success(), "{}", stderr(&result));
    let text = stdout(&result);
    assert!(
        text.contains("Result (the scenario's own scale, for play):"),
        "{text}"
    );
    assert!(!text.contains("placeholder scale of decision 0054"));

    let result = binary(&["run", OF_RECORD]);
    assert!(result.status.success());
    let text = stdout(&result);
    assert!(
        text.contains("Result (the placeholder scale of decision 0054):"),
        "{text}"
    );
    assert!(!text.contains("for play"));
}

#[test]
fn a_batch_header_says_whether_the_scenario_is_for_play() {
    let play = ForPlay::new("batch");
    let (path, for_play) = batch_of(&play.file, "play");
    assert_eq!(for_play["version"], 6);
    assert_eq!(for_play["for_play"], true);
    let _ = fs::remove_file(path);
    let (path, of_record) = batch_of(OF_RECORD, "record");
    assert_eq!(of_record["version"], 6);
    assert_eq!(of_record["for_play"], false);
    let _ = fs::remove_file(path);
}

#[test]
fn validate_refuses_a_scenario_for_play_by_the_file_and_by_the_scenario() {
    let play = ForPlay::new("validate");
    let (path, mut batch) = batch_of(&play.file, "refuse");
    let measures = std::env::temp_dir().join(format!(
        "flagplot-for-play-{}-measures.toml",
        std::process::id()
    ));
    fs::write(&measures, "").expect("measures");
    let report = std::env::temp_dir().join(format!(
        "flagplot-for-play-{}-report.md",
        std::process::id()
    ));
    let judge = |path: &PathBuf| {
        binary(&[
            "validate",
            &path.to_string_lossy(),
            &measures.to_string_lossy(),
            "--out",
            &report.to_string_lossy(),
        ])
    };
    let want = "is a scenario for play (decision 0187): its results are not for the record, so validate does not judge it";
    // The batch says so.
    let refused = judge(&path);
    assert!(!refused.status.success());
    assert!(stderr(&refused).contains(want), "{}", stderr(&refused));
    // The batch is of a file from before the key (or says false): the
    // scenario on disk still says so.
    batch.as_object_mut().expect("object").remove("for_play");
    let older = path.with_extension("older.json");
    fs::write(&older, serde_json::to_vec(&batch).expect("JSON")).expect("write");
    let refused = judge(&older);
    assert!(!refused.status.success());
    assert!(stderr(&refused).contains(want), "{}", stderr(&refused));
    assert!(!report.exists(), "a report was written");
    for file in [path, older, measures, report] {
        let _ = fs::remove_file(file);
    }
}
