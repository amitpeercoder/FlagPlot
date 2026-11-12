//! The rule of agreement (`docs/step2-spec.md`, "What counts as agreement"),
//! in whole numbers. A percentile is a value a run gave: the kept values are
//! sorted and the percentile p is the value at place `ceil(p x n / 100)`,
//! counting from 1, of n.

/// What a measure's kept values come to.
pub struct Spread {
    pub kept: u64,
    pub least: i64,
    pub p5: i64,
    pub p25: i64,
    pub median: i64,
    pub p75: i64,
    pub p95: i64,
    pub greatest: i64,
    /// A count of each value, in value order, when there are 20 or fewer
    /// distinct values.
    pub counts: Option<Vec<(i64, u64)>>,
}

/// The percentile `p` of sorted values, nearest rank, 1-based.
pub fn percentile(sorted: &[i64], p: u64) -> Option<i64> {
    let n = sorted.len() as u64;
    if n == 0 {
        return None;
    }
    let rank = ((p * n).div_ceil(100)).max(1);
    sorted.get((rank - 1) as usize).copied()
}

pub fn spread(values: &[i64]) -> Option<Spread> {
    let mut sorted = values.to_vec();
    sorted.sort_unstable();
    let mut counts: Vec<(i64, u64)> = Vec::new();
    for v in &sorted {
        match counts.last_mut() {
            Some((last, n)) if last == v => *n += 1,
            _ => counts.push((*v, 1)),
        }
    }
    Some(Spread {
        kept: sorted.len() as u64,
        least: *sorted.first()?,
        p5: percentile(&sorted, 5)?,
        p25: percentile(&sorted, 25)?,
        median: percentile(&sorted, 50)?,
        p75: percentile(&sorted, 75)?,
        p95: percentile(&sorted, 95)?,
        greatest: *sorted.last()?,
        counts: (counts.len() <= 20).then_some(counts),
    })
}

/// The verdict on a judged measure.
#[derive(PartialEq, Eq, Debug)]
pub enum Verdict {
    Agrees,
    /// Agrees, and no part of the record lies from the 25th percentile to
    /// the 75th, ends included. It still agrees and the battle still passes on
    /// it; the mark keeps a count of agreements from being read as health.
    AgreesAtEdge,
    /// Which tests failed: the first (the record is outside the middle 90
    /// percent), the second (fewer than 1 run in 20 holds it).
    Miss {
        first: bool,
        second: bool,
    },
    TooFew,
}

pub struct Judgement {
    pub spread: Option<Spread>,
    /// Runs among those kept whose value is in the record, ends included.
    pub holding: u64,
    pub left_out: u64,
    /// `None` for a measure that is only reported.
    pub verdict: Option<Verdict>,
}

/// Judges the kept `values` of a measure over `runs` runs.
pub fn judge(values: &[i64], runs: u64, low: i64, high: i64, judged: bool) -> Judgement {
    let kept = values.len() as u64;
    let left_out = runs - kept;
    let spread = spread(values);
    let holding = values.iter().filter(|v| **v >= low && **v <= high).count() as u64;
    let verdict = judged.then(|| match &spread {
        Some(s) if kept * 5 >= runs => {
            let first = !(low <= s.p95 && high >= s.p5);
            let second = holding * 20 < kept;
            if first || second {
                Verdict::Miss { first, second }
            } else if !(low <= s.p75 && high >= s.p25) {
                Verdict::AgreesAtEdge
            } else {
                Verdict::Agrees
            }
        }
        _ => Verdict::TooFew,
    });
    Judgement {
        spread,
        holding,
        left_out,
        verdict,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The second hand calculation of the architect: 1,000 runs, forty-nine
    /// 0, nine hundred and one 1, fifty 2. Rank 50 is 1 and rank 950 is 1, so
    /// the middle 90 percent is 1 to 1; the median, rank 500, is 1.
    fn thousand() -> Vec<i64> {
        let mut v = vec![0; 49];
        v.extend(vec![1; 901]);
        v.extend(vec![2; 50]);
        v
    }

    #[test]
    fn the_middle_ninety_percent_of_a_thousand() {
        let s = spread(&thousand()).expect("values");
        assert_eq!((s.p5, s.median, s.p95), (1, 1, 1));
        assert_eq!((s.least, s.greatest), (0, 2));
        assert_eq!(s.counts, Some(vec![(0, 49), (1, 901), (2, 50)]));
    }

    #[test]
    fn verdicts_on_a_thousand() {
        let v = thousand();
        let verdict = |low, high| judge(&v, 1000, low, high, true).verdict.expect("judged");
        // 1 is inside 1 to 1 and 901 hold it.
        assert_eq!(verdict(1, 1), Verdict::Agrees);
        // 2 is outside 1 to 1; 50 hold it and 50 x 20 = 1000 >= 1000.
        assert_eq!(
            verdict(2, 2),
            Verdict::Miss {
                first: true,
                second: false
            }
        );
        // 0 is outside; 49 x 20 = 980 < 1000.
        assert_eq!(
            verdict(0, 0),
            Verdict::Miss {
                first: true,
                second: true
            }
        );
        // 0 to 1 overlaps 1 to 1; 950 hold it. Rank 250 and rank 750 are
        // both 1, which it includes: a plain agreement.
        assert_eq!(verdict(0, 1), Verdict::Agrees);
        assert_eq!(judge(&v, 1000, 0, 1, true).holding, 950);
    }

    /// The architect's second set: one hundred 0, eight hundred 1, one
    /// hundred 2. Rank 50 is 0 and rank 950 is 2; rank 250 and rank 750 are
    /// both 1.
    #[test]
    fn an_agreement_at_the_edge_of_a_thousand() {
        let mut v = vec![0; 100];
        v.extend(vec![1; 800]);
        v.extend(vec![2; 100]);
        let s = spread(&v).expect("values");
        assert_eq!((s.p5, s.p25, s.median, s.p75, s.p95), (0, 1, 1, 1, 2));
        let verdict = |low, high| judge(&v, 1000, low, high, true).verdict.expect("judged");
        // 2 is outside 1 to 1; 100 x 20 = 2000 >= 1000.
        assert_eq!(verdict(2, 2), Verdict::AgreesAtEdge);
        assert_eq!(verdict(0, 0), Verdict::AgreesAtEdge);
        assert_eq!(verdict(1, 1), Verdict::Agrees);
        assert_eq!(verdict(0, 1), Verdict::Agrees);
        // A reported measure gets no mark.
        assert_eq!(judge(&v, 1000, 2, 2, false).verdict, None);
    }

    #[test]
    fn the_floor_of_runs_kept() {
        // 20 runs: 4 kept is judged (4 x 5 = 20), 3 kept is too few.
        assert_eq!(
            judge(&[1, 1, 1, 1], 20, 1, 1, true).verdict,
            Some(Verdict::Agrees)
        );
        assert_eq!(
            judge(&[1, 1, 1], 20, 1, 1, true).verdict,
            Some(Verdict::TooFew)
        );
        assert_eq!(judge(&[], 20, 1, 1, true).verdict, Some(Verdict::TooFew));
        assert_eq!(judge(&[1, 1, 1], 20, 1, 1, false).verdict, None);
    }

    #[test]
    fn a_percentile_is_a_value_a_run_gave() {
        let sorted = [10, 20];
        assert_eq!(percentile(&sorted, 50), Some(10));
        assert_eq!(percentile(&sorted, 95), Some(20));
        assert_eq!(percentile(&[], 50), None);
    }
}
