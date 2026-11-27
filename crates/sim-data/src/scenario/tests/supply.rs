use super::*;

fn supply_line(extra: &str) -> String {
    format!("[[supply_line]]\n{extra}")
}

#[test]
fn supply_lines_load_with_defaults_in_one_order_and_are_checked() {
    let none = parse_with("").expect("loads");
    assert!(none.world.umpire().supply_lines.is_empty());
    // Defaults: the base's side, day 1 to the last day, fuel 0.
    let one = parse_with(&supply_line("base = 2\nsupply_t = 10\n")).expect("loads");
    let lines = &one.world.umpire().supply_lines;
    assert_eq!(lines.len(), 1);
    assert_eq!(
        (
            lines[0].side,
            lines[0].base.raw(),
            lines[0].first_day,
            lines[0].last_day
        ),
        (Side::Japanese, 2, 1, 2)
    );
    assert_eq!(lines[0].supply_t, Fixed::from_int(10));
    assert_eq!(lines[0].fuel_t, Fixed::ZERO);
    // A decimal string loads exactly; a TOML float is refused.
    let dec = parse_with(&supply_line("base = 1\nfuel_t = \"12.5\"\n")).expect("loads");
    assert_eq!(
        dec.world.umpire().supply_lines[0].fuel_t,
        "12.5".parse::<Fixed>().unwrap()
    );
    assert!(parse_with(&supply_line("base = 1\nfuel_t = 12.5\n")).is_err());
    // An explicit side may differ from the base's.
    let other = parse_with(&supply_line(
        "base = 1\nside = \"japanese\"\nsupply_t = 1\n",
    ))
    .expect("loads");
    assert_eq!(other.world.umpire().supply_lines[0].side, Side::Japanese);
    // Refusals.
    let bad = |extra: &str| error_of(&supply_line(extra));
    let m = bad("base = 9\nsupply_t = 1\n");
    assert!(
        m.contains("supply line 1") && m.contains("unknown base"),
        "{m}"
    );
    let m = bad("base = 1\nsupply_t = -1\nfuel_t = 5\n");
    assert!(m.contains("supply line 1") && m.contains("below 0"), "{m}");
    let m = bad("base = 1\nsupply_t = 1\nfuel_t = -1\n");
    assert!(m.contains("supply line 1") && m.contains("below 0"), "{m}");
    let m = bad("base = 1\n");
    assert!(
        m.contains("supply line 1") && m.contains("gives nothing"),
        "{m}"
    );
    let m = bad("base = 1\nsupply_t = 0\nfuel_t = 0\n");
    assert!(m.contains("gives nothing"), "{m}");
    let m = bad("base = 1\nsupply_t = 1\nfirst_day = 0\n");
    assert!(
        m.contains("supply line 1") && m.contains("first day below 1"),
        "{m}"
    );
    let m = bad("base = 1\nsupply_t = 1\nfirst_day = 3\n");
    assert!(
        m.contains("supply line 1") && m.contains("past the game's last day"),
        "{m}"
    );
    let m = bad("base = 1\nsupply_t = 1\nlast_day = 3\n");
    assert!(m.contains("past the game's last day"), "{m}");
    let m = bad("base = 1\nsupply_t = 1\nfirst_day = 2\nlast_day = 1\n");
    assert!(
        m.contains("supply line 1") && m.contains("before its first day"),
        "{m}"
    );
    // The number is the line's place in the file.
    let second = format!(
        "{}{}",
        supply_line("base = 1\nsupply_t = 1\n"),
        supply_line("base = 9\nsupply_t = 1\n")
    );
    assert!(error_of(&second).contains("supply line 2"));
    assert!(parse_with(&supply_line("base = 1\nsupply_t = 1\nextra = 1\n")).is_err());
    // Two lines to one base are both kept, in one order.
    let a = supply_line("base = 1\nsupply_t = 5\nfirst_day = 2\n");
    let b = supply_line("base = 1\nsupply_t = 3\n");
    let ab = parse_with(&format!("{a}{b}")).expect("loads");
    let ba = parse_with(&format!("{b}{a}")).expect("loads");
    let (x, y) = (
        &ab.world.umpire().supply_lines,
        &ba.world.umpire().supply_lines,
    );
    assert_eq!(x.len(), 2);
    assert_eq!(x, y);
    assert_eq!((x[0].first_day, x[1].first_day), (1, 2));
}
