//! `sim-cli validate`: judges a batch of runs against the record
//! (`docs/step2-spec.md`, "What counts as agreement" and "The harness").
//!
//! `validate BATCH.json MEASURES.toml [--out FILE]` takes each measure's
//! number from every run, judges it, and writes a report under
//! `docs/validation/`. `validate MEASURES.toml [--out FILE]` is for a
//! measures file whose scenario is not built: every measure is "not run".
//!
//! It is the umpire's tool: it reads truth (the batch file) and nothing that
//! plays the game may use it or its reports. It changes no rule, state or
//! hash, and uses no float and no wall clock: the same files give the same
//! bytes.

mod input;
mod judge;
mod passages;
mod report;
mod take;

use std::fs;
use std::path::{Path, PathBuf};

use sim_core::hash::STATE_VERSION;

use input::{Batch, BatchHeader, MeasuresFile};
use report::{Head, Row};

/// A label becomes part of a file name, so it is letters, digits, dot, dash
/// and underscore.
fn label_ok(label: &str) -> bool {
    !label.is_empty()
        && label
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || matches!(c, '.' | '-' | '_'))
}

/// Writes the report. A file of the same name with other bytes is refused;
/// one with the same bytes is left as it is.
fn write_report(path: &Path, text: &str) -> Result<bool, String> {
    if path.exists() {
        let old = fs::read(path).map_err(|e| format!("{}: {e}", path.display()))?;
        if old == text.as_bytes() {
            return Ok(false);
        }
        return Err(format!(
            "{} exists with different contents; a report is not written over (give another label or --out)",
            path.display()
        ));
    }
    if let Some(dir) = path.parent() {
        if !dir.as_os_str().is_empty() {
            fs::create_dir_all(dir).map_err(|e| format!("{}: {e}", dir.display()))?;
        }
    }
    fs::write(path, text).map_err(|e| format!("{}: {e}", path.display()))?;
    Ok(true)
}

/// `validate`: `args` are the words after it (`--reference` already taken
/// out by the caller and given as `options`).
pub fn command(
    args: &[String],
    usage: &str,
    options: &sim_data::LoadOptions,
) -> Result<(), String> {
    let mut positional: Vec<&str> = Vec::new();
    let mut out: Option<PathBuf> = None;
    let (mut passages, mut label, mut map, mut places): (
        Option<&str>,
        Option<&str>,
        Option<&str>,
        Option<&str>,
    ) = (None, None, None, None);
    let mut words = args.iter().map(String::as_str);
    while let Some(word) = words.next() {
        match word {
            "--out" => {
                out = Some(
                    words
                        .next()
                        .ok_or_else(|| format!("--out needs a file\n\n{usage}"))?
                        .into(),
                );
            }
            "--passages" | "--label" | "--map" | "--places" => {
                let value = words
                    .next()
                    .ok_or_else(|| format!("{word} needs a value\n\n{usage}"))?;
                match word {
                    "--passages" => passages = Some(value),
                    "--label" => label = Some(value),
                    "--map" => map = Some(value),
                    _ => places = Some(value),
                }
            }
            flag if flag.starts_with('-') => {
                return Err(format!("unknown option \"{flag}\"\n\n{usage}"))
            }
            path => positional.push(path),
        }
    }
    if let Some(passages) = passages {
        if !positional.is_empty() {
            return Err(format!(
                "validate --passages takes no batch or measures file\n\n{usage}"
            ));
        }
        return passages::command(&passages::Request {
            passages,
            label,
            map,
            places,
            out,
        });
    }
    if label.is_some() || map.is_some() || places.is_some() {
        return Err(format!(
            "--label, --map and --places are for validate --passages\n\n{usage}"
        ));
    }
    let (batch_path, measures_path) = match positional.as_slice() {
        [measures] => (None, *measures),
        [batch, measures] => (Some(*batch), *measures),
        _ => {
            return Err(format!(
                "validate needs BATCH.json MEASURES.toml, or MEASURES.toml alone\n\n{usage}"
            ))
        }
    };

    let measures_bytes = fs::read(measures_path).map_err(|e| format!("{measures_path}: {e}"))?;
    let measures_hash = crate::save::hash_hex(&measures_bytes);
    let measures_text =
        String::from_utf8(measures_bytes).map_err(|_| format!("{measures_path}: not text"))?;

    let Some(batch_path) = batch_path else {
        return not_built(measures_path, &measures_text, &measures_hash, out);
    };

    // 1. The batch file itself.
    let batch_bytes = fs::read(batch_path).map_err(|e| format!("{batch_path}: {e}"))?;
    let batch_hash = crate::save::hash_hex(&batch_bytes);
    let header: BatchHeader = serde_json::from_slice(&batch_bytes)
        .map_err(|e| format!("{batch_path}: not a batch file (decision 0062): {e}"))?;
    if header.format != "flag-plot-batch" || header.version != 3 {
        let made_again = if header.version < 3 {
            "; make it again with the engine's `batch` (version 3 adds the shots of each ship)"
        } else {
            ""
        };
        return Err(format!(
            "{batch_path}: the batch is {} version {}; validate reads flag-plot-batch version 3 only{made_again}",
            header.format, header.version
        ));
    }
    let batch: Batch = serde_json::from_slice(&batch_bytes)
        .map_err(|e| format!("{batch_path}: not a batch file of version 3 (decision 0062): {e}"))?;
    // 2. The label is the tuning round.
    let label = match batch.label.as_deref() {
        None => {
            return Err(format!(
                "{batch_path}: the batch has no label; make it with --label"
            ))
        }
        Some(label) if !label_ok(label) => {
            return Err(format!(
            "{batch_path}: the label \"{label}\" is not letters, digits, dot, dash and underscore"
        ))
        }
        Some(label) => label,
    };
    // 3. The engine's hash layout.
    let engine = String::from_utf8_lossy(STATE_VERSION).into_owned();
    if batch.inputs.state_version != engine {
        return Err(format!(
            "{batch_path}: the batch was made with state hash layout {}, the engine's is {engine}",
            batch.inputs.state_version
        ));
    }
    // 4. The inputs against the files on disk.
    let scenario = sim_data::Scenario::load_with(Path::new(&batch.scenario.file), options)
        .map_err(|e| e.to_string())?;
    let now = crate::save::inputs(&batch.scenario.file, &scenario)?;
    if now.scenario_hash != batch.inputs.scenario_hash {
        return Err(format!(
            "the scenario file {} has changed since the batch was made (batch hash {}, now {})",
            batch.scenario.file, batch.inputs.scenario_hash, now.scenario_hash
        ));
    }
    let now_names: Vec<&str> = now.data.iter().map(|f| f.file.as_str()).collect();
    let batch_names: Vec<&str> = batch.inputs.data.iter().map(|f| f.file.as_str()).collect();
    if now_names != batch_names {
        return Err(format!(
            "the scenario {} now names different data files than the batch was made with (batch: {}; now: {}); give the same --reference",
            batch.scenario.file,
            batch_names.join(", "),
            now_names.join(", ")
        ));
    }
    for (was, is) in batch.inputs.data.iter().zip(&now.data) {
        if was.hash != is.hash {
            return Err(format!(
                "the data file {} has changed since the batch was made (batch hash {}, now {})",
                was.file, was.hash, is.hash
            ));
        }
    }
    // 5. The measures are of this scenario.
    let measures: MeasuresFile =
        toml::from_str(&measures_text).map_err(|e| format!("{measures_path}: {e}"))?;
    let Some(measures_scenario) = &measures.scenario else {
        return Err(format!(
            "{measures_path} has no scenario (it is not built); give it to validate without a batch"
        ));
    };
    let scenario_bytes =
        fs::read(measures_scenario).map_err(|e| format!("{measures_scenario}: {e}"))?;
    let scenario_file_hash = crate::save::hash_hex(&scenario_bytes);
    if scenario_file_hash != batch.inputs.scenario_hash {
        return Err(format!(
            "{measures_path} is for {measures_scenario} (hash {scenario_file_hash}), but the batch was made from {} (hash {})",
            batch.scenario.file, batch.inputs.scenario_hash
        ));
    }
    // 6. The runs are what the batch says.
    if batch.per_run.len() as u64 != batch.runs {
        return Err(format!(
            "{batch_path}: it says {} runs and holds {}",
            batch.runs,
            batch.per_run.len()
        ));
    }
    for (i, run) in batch.per_run.iter().enumerate() {
        if run.seed != batch.first_seed + i as u64 {
            return Err(format!(
                "{batch_path}: run {} has seed {}, not {}",
                i + 1,
                run.seed,
                batch.first_seed + i as u64
            ));
        }
    }
    // 7. Every measure is one validate can take.
    let round_minutes = scenario
        .world
        .ref_data()
        .surface_combat
        .as_ref()
        .map(|figures| u64::from(figures.round_minutes));
    let ctx = take::Context::new(&batch, round_minutes);
    let mut seen = std::collections::BTreeSet::new();
    let mut takes = Vec::new();
    for measure in &measures.measure {
        if !seen.insert(measure.id.as_str()) {
            return Err(format!(
                "{measures_path}: the id {} is used twice",
                measure.id
            ));
        }
        if measure.record_low > measure.record_high {
            return Err(format!(
                "{measures_path}: measure {} has record_low above record_high",
                measure.id
            ));
        }
        takes.push(take::parse(measure, &batch)?);
    }

    let mut rows = Vec::new();
    for (measure, take) in measures.measure.iter().zip(&takes) {
        let mut values = Vec::new();
        for run in &batch.per_run {
            if let Some(v) = take::value(take, &ctx, run)
                .map_err(|e| format!("measure {}: seed {}: {e}", measure.id, run.seed))?
            {
                values.push(v);
            }
        }
        rows.push(Row {
            measure,
            judgement: Some(judge::judge(
                &values,
                batch.runs,
                measure.record_low,
                measure.record_high,
                measure.judged,
            )),
        });
    }

    let head = Head {
        battle: &measures.battle,
        scenario: Some(measures_scenario),
        measures_file: measures_path,
        measures_hash: &measures_hash,
        batch: Some((&batch, &batch_hash)),
    };
    let text = report::write(&head, &rows);
    let path = out.unwrap_or_else(|| {
        PathBuf::from(format!("docs/validation/{}-{label}.md", measures.battle))
    });
    let (agree, edge, miss, judged) = report::tally(&rows);
    let wrote = write_report(&path, &text)?;
    println!(
        "{} {}: judged {judged}, agree {agree}, at the edge {edge}, miss {miss}",
        if wrote { "wrote" } else { "unchanged" },
        path.display()
    );
    Ok(())
}

/// A measures file with no scenario: every measure is not run.
fn not_built(
    measures_path: &str,
    text: &str,
    hash: &str,
    out: Option<PathBuf>,
) -> Result<(), String> {
    let measures: MeasuresFile =
        toml::from_str(text).map_err(|e| format!("{measures_path}: {e}"))?;
    if measures.scenario.is_some() {
        return Err(format!(
            "{measures_path} names a scenario; give validate a batch of it"
        ));
    }
    for measure in &measures.measure {
        if measure.take.is_some() {
            return Err(format!(
                "{measures_path}: measure {} has a take but the file has no scenario",
                measure.id
            ));
        }
    }
    let rows: Vec<Row> = measures
        .measure
        .iter()
        .map(|measure| Row {
            measure,
            judgement: None,
        })
        .collect();
    let head = Head {
        battle: &measures.battle,
        scenario: None,
        measures_file: measures_path,
        measures_hash: hash,
        batch: None,
    };
    let text = report::write(&head, &rows);
    let path = out.unwrap_or_else(|| {
        PathBuf::from(format!("docs/validation/{}-not-built.md", measures.battle))
    });
    let wrote = write_report(&path, &text)?;
    println!(
        "{} {}: {} measures, not run",
        if wrote { "wrote" } else { "unchanged" },
        path.display(),
        rows.len()
    );
    Ok(())
}
