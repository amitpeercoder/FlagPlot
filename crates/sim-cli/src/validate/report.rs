//! The report: markdown, a pure function of the batch, the measures and the
//! hashes of the files. It holds no date and no float.

use std::fmt::Write;

use sim_core::fixed::Fixed;

use super::input::{Batch, Measure};
use super::judge::{self, Judgement, Spread, Verdict};

/// The runs the spec asks for before a battle can pass.
pub const RUNS_ASKED: u64 = 1000;

/// A measure and what was worked out for it; `None` where it is not run.
pub struct Row<'a> {
    pub measure: &'a Measure,
    pub judgement: Option<Judgement>,
}

pub struct Head<'a> {
    pub battle: &'a str,
    pub scenario: Option<&'a str>,
    pub measures_file: &'a str,
    pub measures_hash: &'a str,
    /// The batch, its hash, and the file's own name; none when not built.
    pub batch: Option<(&'a Batch, &'a str)>,
}

fn record(m: &Measure) -> String {
    if m.record_low == m.record_high {
        m.record_low.to_string()
    } else {
        format!("{} to {}", m.record_low, m.record_high)
    }
}

fn cell(spread: &Option<Spread>, pick: impl Fn(&Spread) -> i64) -> String {
    spread
        .as_ref()
        .map_or("-".to_string(), |s| pick(s).to_string())
}

fn verdict_text(v: &Option<Verdict>, judged: bool) -> String {
    match v {
        None if judged => "not run".to_string(),
        None => "reported".to_string(),
        Some(Verdict::Agrees) => "agrees".to_string(),
        Some(Verdict::AgreesAtEdge) => "agrees, at the edge".to_string(),
        Some(Verdict::TooFew) => "**MISS**: too few runs kept".to_string(),
        Some(Verdict::Miss { first, second }) => {
            let first_text = "the record is outside the middle 90 percent";
            let second_text = "fewer than 1 run in 20 holds the record";
            match (first, second) {
                (true, true) => format!("**MISS**: {first_text}; {second_text}"),
                (true, false) => format!("**MISS**: {first_text}"),
                _ => format!("**MISS**: {second_text}"),
            }
        }
    }
}

/// Judged measures that agree plainly, that agree at the edge, that miss,
/// and the judged in all.
pub fn tally(rows: &[Row]) -> (usize, usize, usize, usize) {
    let judged: Vec<&Row> = rows.iter().filter(|r| r.measure.judged).collect();
    let count = |f: fn(&Verdict) -> bool| {
        judged
            .iter()
            .filter(|r| verdict_of(r).as_ref().is_some_and(f))
            .count()
    };
    let agree = count(|v| matches!(v, Verdict::Agrees));
    let edge = count(|v| matches!(v, Verdict::AgreesAtEdge));
    let miss = count(|v| matches!(v, Verdict::Miss { .. } | Verdict::TooFew));
    (agree, edge, miss, judged.len())
}

pub fn write(head: &Head, rows: &[Row]) -> String {
    let mut out = String::new();
    let runs = head.batch.map(|(b, _)| b.runs);
    match head.batch {
        Some((batch, _)) => {
            let label = batch.label.as_deref().unwrap_or("");
            let _ = writeln!(out, "# {}: validation of batch `{label}`\n", head.battle);
        }
        None => {
            let _ = writeln!(out, "# {}: measures not run\n", head.battle);
        }
    }
    let _ = writeln!(out, "Battle: {}", head.battle);
    if let Some(scenario) = head.scenario {
        let _ = writeln!(out, "Scenario: {scenario}");
    }
    if let Some((batch, batch_hash)) = head.batch {
        let _ = writeln!(
            out,
            "Batch: label `{}`, {} runs, seeds {} to {}, file hash {batch_hash}",
            batch.label.as_deref().unwrap_or(""),
            batch.runs,
            batch.first_seed,
            batch.first_seed + batch.runs.saturating_sub(1)
        );
        let _ = writeln!(
            out,
            "Inputs: state hash layout {}, scenario file hash {}",
            batch.inputs.state_version, batch.inputs.scenario_hash
        );
        for file in &batch.inputs.data {
            let _ = writeln!(out, "- {} {}", file.file, file.hash);
        }
    }
    let _ = writeln!(
        out,
        "Measures: {} (hash {}), {} measures, {} judged\n",
        head.measures_file,
        head.measures_hash,
        rows.len(),
        rows.iter().filter(|r| r.measure.judged).count()
    );

    // The result line.
    let judged: Vec<&Row> = rows.iter().filter(|r| r.measure.judged).collect();
    let (agree, edge, miss, _) = tally(rows);
    match runs {
        None => {
            let _ = writeln!(
                out,
                "**Not run: the scenario is not built.** There is no batch and no verdict.\n"
            );
        }
        Some(runs) => {
            let _ = writeln!(
                out,
                "Judged measures: {agree} agree, {edge} agree at the edge, {miss} miss, of {}.\n",
                judged.len()
            );
            if runs < RUNS_ASKED {
                let _ = writeln!(
                    out,
                    "**No verdict on the battle.** This batch has {runs} runs and the spec asks for {RUNS_ASKED}. Each measure has its verdict below.\n"
                );
            } else if miss == 0 {
                let _ = writeln!(out, "**The battle passes:** every judged measure agrees.\n");
            } else {
                let _ = writeln!(
                    out,
                    "**The battle does not pass:** {miss} of {} judged measures miss.\n",
                    judged.len()
                );
            }
        }
    }

    // The table.
    let _ = writeln!(
        out,
        "| Measure | Record | 5th to 95th | Median | Runs holding the record | Left out | Verdict |"
    );
    let _ = writeln!(out, "| --- | --- | --- | --- | --- | --- | --- |");
    for row in rows {
        let m = row.measure;
        match &row.judgement {
            None => {
                let _ = writeln!(
                    out,
                    "| {} | {} | - | - | - | - | not run: scenario not built |",
                    m.id,
                    record(m)
                );
            }
            Some(j) => {
                let band = match &j.spread {
                    Some(s) => format!("{} to {}", s.p5, s.p95),
                    None => "-".to_string(),
                };
                let kept = j.spread.as_ref().map_or(0, |s| s.kept);
                let _ = writeln!(
                    out,
                    "| {} | {} | {band} | {} | {} of {kept} | {} | {} |",
                    m.id,
                    record(m),
                    cell(&j.spread, |s| s.median),
                    j.holding,
                    j.left_out,
                    verdict_text(&j.verdict, m.judged)
                );
            }
        }
    }
    out.push('\n');

    // The misses, then the agreements at the edge.
    let with = |f: fn(&Verdict) -> bool| -> Vec<&Row> {
        judged
            .iter()
            .copied()
            .filter(|r| verdict_of(r).as_ref().is_some_and(f))
            .collect()
    };
    detail(
        &mut out,
        "Misses",
        &with(|v| matches!(v, Verdict::Miss { .. } | Verdict::TooFew)),
    );
    detail(
        &mut out,
        "At the edge",
        &with(|v| matches!(v, Verdict::AgreesAtEdge)),
    );

    // The objectives scale.
    if let Some((batch, _)) = head.batch {
        objectives(&mut out, batch);
    }

    let _ = writeln!(out, "## Notes\n");
    let _ = writeln!(out, "- Percentiles are by nearest rank: the value at place ceil(p x n / 100) of the n kept runs, sorted. The record is inside the middle 90 percent if any part of it lies from the 5th percentile to the 95th, ends included. The verdict is \"agrees, at the edge\" where a measure agrees and no part of its record lies from the 25th percentile to the 75th, ends included; it still agrees. A run holds the record if its value is from the low end to the high end, ends included.");
    let _ = writeln!(out, "- Hits are counted as rolled: a salvo is rolled at the ships afloat when its round starts, so a hit on a ship another salvo sinks in the same round is counted (decision 0062).");
    let _ = writeln!(out, "- Escorts are matched to a raid's first strike group: an escort entry is added once, when its day, pulse and first group are those of an attack the measure counts (decision 0062).");
    out
}

/// A section listing measures with what each counts, its record and rows,
/// and its spread.
fn detail(out: &mut String, title: &str, rows: &[&Row]) {
    if rows.is_empty() {
        return;
    }
    let _ = writeln!(out, "## {title}\n");
    for row in rows {
        let m = row.measure;
        let j = row
            .judgement
            .as_ref()
            .expect("a listed measure has a judgement");
        let _ = writeln!(out, "### {}\n", m.id);
        let _ = writeln!(out, "- Counts: {}", m.counts);
        let rows_text: Vec<String> = m.rows.iter().map(i64::to_string).collect();
        let _ = writeln!(
            out,
            "- Record: {} (rows {})",
            record(m),
            rows_text.join(", ")
        );
        let _ = writeln!(out, "- Verdict: {}", verdict_text(&j.verdict, true));
        match &j.spread {
            Some(s) => {
                let _ = writeln!(
                    out,
                    "- Spread over {} runs kept ({} left out): least {}, 5th {}, 25th {}, median {}, 75th {}, 95th {}, greatest {}",
                    s.kept, j.left_out, s.least, s.p5, s.p25, s.median, s.p75, s.p95, s.greatest
                );
                if let Some(counts) = &s.counts {
                    let list: Vec<String> =
                        counts.iter().map(|(v, n)| format!("{v}: {n}")).collect();
                    let _ = writeln!(out, "- Each value and its runs: {}", list.join(", "));
                }
            }
            None => {
                let _ = writeln!(out, "- No run gave a value ({} left out).", j.left_out);
            }
        }
        out.push('\n');
    }
}

fn verdict_of<'a>(row: &'a Row) -> &'a Option<Verdict> {
    static NONE: Option<Verdict> = None;
    row.judgement.as_ref().map_or(&NONE, |j| &j.verdict)
}

/// The tally of outcomes and the Allied margin, reported. Margins are
/// decimals (`Fixed`); they are ordered by their exact value, the number of
/// millionths each is, never by a float or by their text.
fn objectives(out: &mut String, batch: &Batch) {
    let mut margins: Vec<i64> = Vec::new();
    for run in &batch.per_run {
        if let Some(text) = &run.allied_margin {
            if let Ok(f) = text.parse::<Fixed>() {
                margins.push(f.raw());
            }
        }
    }
    let Some(results) = &batch.results else {
        return;
    };
    let _ = writeln!(
        out,
        "## Result on the objectives scale (reported, not judged)\n"
    );
    let _ = writeln!(out, "Seen from the Allied side, runs of each result:\n");
    for row in results {
        let _ = writeln!(out, "- {}: {} of {}", row.outcome, row.runs, batch.runs);
    }
    margins.sort_unstable();
    if !margins.is_empty() {
        let at =
            |p| Fixed::from_raw(judge::percentile(&margins, p).expect("margins are not empty"));
        let _ = writeln!(
            out,
            "\nAllied margin over {} runs: 5th {}, median {}, 95th {}",
            margins.len(),
            at(5),
            at(50),
            at(95)
        );
    }
    out.push('\n');
}
