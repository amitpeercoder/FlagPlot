//! `oob-check <dir>`: checks the order-of-battle facts files in a directory,
//! prints every problem and exits non-zero if there is one.

use std::path::Path;
use std::process::ExitCode;

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let [dir] = args.as_slice() else {
        eprintln!("usage: oob-check <dir>");
        return ExitCode::from(2);
    };
    let report = oob_check::check_dir(Path::new(dir));
    for problem in &report.problems {
        println!("{problem}");
    }
    if report.is_clean() {
        println!("{dir}: no problems");
        ExitCode::SUCCESS
    } else {
        println!("{dir}: {} problems", report.problems.len());
        ExitCode::FAILURE
    }
}
