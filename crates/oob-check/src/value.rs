//! What a well-formed value looks like: whole numbers, decimals, dates,
//! times, keys. Also the two comparisons the standing rules need: equal
//! values and overlapping date ranges. No floats anywhere.

fn all_digits(s: &str) -> bool {
    !s.is_empty() && s.bytes().all(|b| b.is_ascii_digit())
}

/// `[0-9]+`
pub fn is_whole(s: &str) -> bool {
    all_digits(s)
}

/// `-?[0-9]+`
pub fn is_integer(s: &str) -> bool {
    all_digits(s.strip_prefix('-').unwrap_or(s))
}

/// `-?[0-9]+(\.[0-9]+)?`
pub fn is_decimal(s: &str) -> bool {
    let s = s.strip_prefix('-').unwrap_or(s);
    match s.split_once('.') {
        Some((whole, fraction)) => all_digits(whole) && all_digits(fraction),
        None => all_digits(s),
    }
}

/// A number written with an exponent, such as `1e5` or `-2.5E-3`. The files
/// hold whole numbers and decimal strings only.
pub fn looks_like_exponent_float(s: &str) -> bool {
    let Some((mantissa, exponent)) = s.split_once(['e', 'E']) else {
        return false;
    };
    let exponent = exponent.strip_prefix(['+', '-']).unwrap_or(exponent);
    is_decimal(mantissa) && all_digits(exponent)
}

fn number_in(s: &str, min: u32, max: u32) -> bool {
    all_digits(s) && s.parse::<u32>().is_ok_and(|n| (min..=max).contains(&n))
}

/// `YYYY`, `YYYY-MM` or `YYYY-MM-DD`.
pub fn is_date(s: &str) -> bool {
    let parts: Vec<&str> = s.split('-').collect();
    match parts.as_slice() {
        [y] => y.len() == 4 && all_digits(y),
        [y, m] => y.len() == 4 && all_digits(y) && m.len() == 2 && number_in(m, 1, 12),
        [y, m, d] => {
            y.len() == 4
                && all_digits(y)
                && m.len() == 2
                && number_in(m, 1, 12)
                && d.len() == 2
                && number_in(d, 1, 31)
        }
        _ => false,
    }
}

/// An engagement time: a date, or `YYYY-MM-DDTHH:MM`.
pub fn is_time(s: &str) -> bool {
    match s.split_once('T') {
        None => is_date(s),
        Some((date, clock)) => {
            date.len() == 10
                && is_date(date)
                && clock.len() == 5
                && clock
                    .split_once(':')
                    .is_some_and(|(h, m)| number_in(h, 0, 23) && number_in(m, 0, 59))
        }
    }
}

/// `^[a-z0-9][a-z0-9.\-]*$`
pub fn is_key(s: &str) -> bool {
    let ok = |b: u8| b.is_ascii_lowercase() || b.is_ascii_digit();
    s.bytes().next().is_some_and(ok) && s.bytes().all(|b| ok(b) || b == b'.' || b == b'-')
}

/// The form two values are compared in. Numbers compare as numbers, so
/// `100`, `100.0` and `0100` are one value. Anything else compares as
/// trimmed text without regard to case.
pub fn normalise(value: &str) -> String {
    let value = value.trim();
    if !is_decimal(value) {
        return value.to_lowercase();
    }
    let (negative, digits) = match value.strip_prefix('-') {
        Some(rest) => (true, rest),
        None => (false, value),
    };
    let (whole, fraction) = digits.split_once('.').unwrap_or((digits, ""));
    let whole = whole.trim_start_matches('0');
    let fraction = fraction.trim_end_matches('0');
    let mut out = String::new();
    if negative && !(whole.is_empty() && fraction.is_empty()) {
        out.push('-');
    }
    out.push_str(if whole.is_empty() { "0" } else { whole });
    if !fraction.is_empty() {
        out.push('.');
        out.push_str(fraction);
    }
    out
}

/// The span a row holds for, as a pair of text bounds: start inclusive, end
/// exclusive. Empty means unbounded. The same non-empty date in both means
/// "true on that date", a point.
///
/// Dates compare as text. A coarser date sorts before the finer dates inside
/// it (`1942-08` before `1942-08-07`), and a point's end is the date followed
/// by `~`, which sorts after every finer date inside it. So a month-long
/// point covers its days. This is the comparison the data was produced with.
pub fn span(valid_from: &str, valid_to: &str) -> (String, String) {
    let from = if valid_from.is_empty() {
        "0000"
    } else {
        valid_from
    };
    let to = if !valid_from.is_empty() && valid_from == valid_to {
        format!("{valid_from}~")
    } else if valid_to.is_empty() {
        "9999".to_string()
    } else {
        valid_to.to_string()
    };
    (from.to_string(), to)
}

pub fn spans_overlap(a: &(String, String), b: &(String, String)) -> bool {
    a.0 < b.1 && b.0 < a.1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn numbers() {
        assert!(is_integer("-12") && is_integer("0") && !is_integer("1.0") && !is_integer("-"));
        assert!(is_decimal("-9.8333") && is_decimal("040") && is_decimal("7"));
        assert!(!is_decimal("1.") && !is_decimal(".5") && !is_decimal("1,000") && !is_decimal(""));
        assert!(!is_decimal("1e5") && !is_decimal("+1"));
        assert!(looks_like_exponent_float("1e5") && looks_like_exponent_float("-2.5E-3"));
        assert!(!looks_like_exponent_float("Zeke") && !looks_like_exponent_float("e5"));
    }

    #[test]
    fn dates_and_times() {
        assert!(is_date("1942") && is_date("1942-08") && is_date("1942-08-09"));
        assert!(!is_date("1942-8-9") && !is_date("1942-13") && !is_date("1942-08-32"));
        assert!(!is_date("") && !is_date("42-08-09") && !is_date("1942-08-09-01"));
        assert!(is_time("1942-08-09T01:43") && is_time("1942-08-09"));
        assert!(!is_time("1942-08-09T24:00") && !is_time("1942-08T01:43"));
        assert!(!is_time("1942-08-09T0143") && !is_time("1942-08-09 01:43"));
    }

    #[test]
    fn keys() {
        assert!(is_key("ca-44-vincennes") && is_key("5in-38.mk12") && is_key("0a"));
        assert!(!is_key("") && !is_key("-a") && !is_key("Chokai") && !is_key("a_b"));
    }

    #[test]
    fn values_compare_numerically_or_as_text() {
        assert_eq!(normalise("100"), normalise("100.0"));
        assert_eq!(normalise("0100.50"), "100.5");
        assert_eq!(normalise("-0.0"), "0");
        assert_eq!(normalise("-3.10"), "-3.1");
        assert_eq!(normalise("  Zeke "), "zeke");
        assert_ne!(normalise("100"), normalise("101"));
    }

    #[test]
    fn spans() {
        let open = span("", "");
        let aug = span("1942-08-01", "1942-09-01");
        let sep = span("1942-09-01", "");
        let point = span("1942-08-09", "1942-08-09");
        let month_point = span("1942-08", "1942-08");
        assert!(spans_overlap(&open, &aug));
        assert!(!spans_overlap(&aug, &sep), "valid_to is exclusive");
        assert!(spans_overlap(&aug, &point));
        assert!(!spans_overlap(&sep, &point));
        assert!(spans_overlap(&month_point, &point));
        assert!(spans_overlap(&point, &point));
    }
}
