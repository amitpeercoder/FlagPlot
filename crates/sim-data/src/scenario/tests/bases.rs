use super::*;

#[test]
fn a_base_takes_an_optional_whole_number_of_anti_aircraft_shots() {
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../data/scenarios/test.toml");
    let guns = |extra: &str| {
        let head = HEAD.replace("q = 5\nr = 0\n", &format!("q = 5\nr = 0\n{extra}"));
        Scenario::parse(&head, &path)
    };
    let shots = |scenario: Scenario| scenario.world.umpire().bases[&BaseId::new(2)].anti_aircraft;
    assert_eq!(shots(guns("").expect("loads")), 0);
    assert_eq!(shots(guns("anti_aircraft = 12\n").expect("loads")), 12);
    assert!(guns("anti_aircraft = 1.5\n").is_err());
    assert!(guns("anti_aircraft = -1\n").is_err());
}

#[test]
fn a_base_takes_an_optional_fuel_stock_in_tons() {
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../data/scenarios/test.toml");
    let fuel = |extra: &str| {
        let head = HEAD.replace("q = 5\nr = 0\n", &format!("q = 5\nr = 0\n{extra}"));
        Scenario::parse(&head, &path)
    };
    let stock = |scenario: Scenario| scenario.world.umpire().bases[&BaseId::new(2)].fuel_t;
    assert_eq!(stock(fuel("").expect("loads")), Fixed::ZERO);
    assert_eq!(
        stock(fuel("fuel_t = 150\n").expect("loads")),
        Fixed::from_int(150)
    );
    assert_eq!(
        stock(fuel("fuel_t = \"0.25\"\n").expect("loads")),
        Fixed::from_ratio(1, 4)
    );
    // A TOML float and a negative stock are both refused.
    assert!(fuel("fuel_t = 1.5\n").is_err());
    assert!(fuel("fuel_t = -1\n").is_err());
    assert!(fuel("fuel_t = \"-0.5\"\n").is_err());
}

#[test]
fn a_base_takes_an_optional_whole_number_of_repairs_a_day() {
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../data/scenarios/test.toml");
    let repairs = |extra: &str| {
        let head = HEAD.replace("q = 5\nr = 0\n", &format!("q = 5\nr = 0\n{extra}"));
        Scenario::parse(&head, &path)
    };
    let figure = |scenario: Scenario| {
        scenario.world.umpire().bases[&BaseId::new(2)].aircraft_repairs_per_day
    };
    assert_eq!(figure(repairs("").expect("loads")), 0);
    assert_eq!(
        figure(repairs("aircraft_repairs_per_day = 3\n").expect("loads")),
        3
    );
    assert!(repairs("aircraft_repairs_per_day = 1.5\n").is_err());
    assert!(repairs("aircraft_repairs_per_day = -1\n").is_err());
}

#[test]
fn a_base_takes_repair_points_a_day_and_a_yard_passage_of_at_least_a_day() {
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../data/scenarios/test.toml");
    let base = |extra: &str| {
        let head = HEAD.replace("q = 5\nr = 0\n", &format!("q = 5\nr = 0\n{extra}"));
        Scenario::parse(&head, &path)
    };
    let figures = |scenario: Scenario| {
        let base = scenario.world.umpire().bases[&BaseId::new(2)].clone();
        (base.ship_repair_points_per_day, base.yard_passage_days)
    };
    assert_eq!(figures(base("").expect("loads")), (Fixed::ZERO, None));
    assert_eq!(
        figures(base("ship_repair_points_per_day = 6\nyard_passage_days = 4\n").expect("loads")),
        (Fixed::from_int(6), Some(4))
    );
    assert_eq!(
        figures(base("ship_repair_points_per_day = \"2.5\"\n").expect("loads")).0,
        Fixed::from_ratio(5, 2)
    );
    assert_eq!(
        figures(base("ship_repair_points_per_day = 0\n").expect("loads")).0,
        Fixed::ZERO
    );
    // Negative points, a float, and a passage of 0, below 0 or not whole.
    assert!(base("ship_repair_points_per_day = -1\n").is_err());
    assert!(base("ship_repair_points_per_day = \"-0.5\"\n").is_err());
    assert!(base("ship_repair_points_per_day = 1.5\n").is_err());
    assert!(base("yard_passage_days = 0\n").is_err());
    assert!(base("yard_passage_days = -3\n").is_err());
    assert!(base("yard_passage_days = 2.5\n").is_err());
    assert!(base("yard_passage_days = \"4\"\n").is_err());
}

#[test]
fn a_base_takes_optional_reserves_that_are_not_negative() {
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../data/scenarios/test.toml");
    let base = |extra: &str| {
        let head = HEAD.replace("q = 5\nr = 0\n", &format!("q = 5\nr = 0\n{extra}"));
        Scenario::parse(&head, &path)
    };
    let reserves = |scenario: Scenario| {
        let base = scenario.world.umpire().bases[&BaseId::new(2)].clone();
        (base.supply_reserve_t, base.fuel_reserve_t)
    };
    assert_eq!(
        reserves(base("").expect("loads")),
        (Fixed::ZERO, Fixed::ZERO)
    );
    assert_eq!(
        reserves(base("supply_reserve_t = 400\nfuel_reserve_t = 300\n").expect("loads")),
        (Fixed::from_int(400), Fixed::from_int(300))
    );
    assert_eq!(
        reserves(base("fuel_reserve_t = \"0.25\"\n").expect("loads")),
        (Fixed::ZERO, Fixed::from_ratio(1, 4))
    );
    assert!(base("supply_reserve_t = -1\n").is_err());
    assert!(base("fuel_reserve_t = \"-0.5\"\n").is_err());
    assert!(base("supply_reserve_t = 1.5\n").is_err());
    assert!(base("fuel_reserve_t = 1.5\n").is_err());
}

#[test]
fn a_base_takes_an_airfield_flag_work_done_on_it_and_defences() {
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../data/scenarios/test.toml");
    let base = |extra: &str| {
        let head = HEAD.replace("q = 5\nr = 0\n", &format!("q = 5\nr = 0\n{extra}"));
        Scenario::parse(&head, &path)
    };
    let figures = |scenario: Scenario| {
        let base = scenario.world.umpire().bases[&BaseId::new(2)].clone();
        (
            base.airfield,
            base.airfield_work,
            base.defences,
            base.day_work,
        )
    };
    assert_eq!(
        figures(base("").expect("loads")),
        (true, Fixed::ZERO, Fixed::ZERO, None)
    );
    assert_eq!(
        figures(
            base("airfield = false\nairfield_work = 5000\ndefences = \"12.5\"\n").expect("loads")
        ),
        (false, Fixed::from_int(5000), Fixed::from_ratio(25, 2), None)
    );
    assert_eq!(
        figures(base("airfield = false\nairfield_work = \"0.5\"\n").expect("loads")).1,
        Fixed::from_ratio(1, 2)
    );
    assert_eq!(
        figures(base("defences = 100\n").expect("loads")).2,
        Fixed::from_int(100)
    );
    assert!(figures(base("airfield = false\n").expect("loads")).1 == Fixed::ZERO);
    // Work on an airfield that exists, negative work, defences out of
    // range and floats are refused.
    let message = base("airfield_work = 100\n")
        .map(|_| ())
        .expect_err("refused")
        .to_string();
    assert!(
        message.contains("airfield_work") && message.contains("has an airfield"),
        "{message}"
    );
    assert!(base("airfield = true\nairfield_work = 0\n").is_err());
    assert!(base("airfield = false\nairfield_work = -1\n").is_err());
    assert!(base("airfield = false\nairfield_work = 1.5\n").is_err());
    let message = base("defences = 101\n")
        .map(|_| ())
        .expect_err("refused")
        .to_string();
    assert!(
        message.contains("defences") && message.contains("0 and 100"),
        "{message}"
    );
    assert!(base("defences = -1\n").is_err());
    assert!(base("defences = 1.5\n").is_err());
    assert!(base("airfield = \"no\"\n").is_err());
}

#[test]
fn a_base_takes_airfield_damage_only_with_an_airfield() {
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../data/scenarios/test.toml");
    let base = |extra: &str| {
        let head = HEAD.replace("q = 5\nr = 0\n", &format!("q = 5\nr = 0\n{extra}"));
        Scenario::parse(&head, &path)
    };
    let damage =
        |scenario: Scenario| scenario.world.umpire().bases[&BaseId::new(2)].airfield_damage;
    assert_eq!(damage(base("").expect("loads")), Fixed::ZERO);
    assert_eq!(
        damage(base("airfield_damage = 30\n").expect("loads")),
        Fixed::from_int(30)
    );
    assert_eq!(
        damage(base("airfield_damage = \"12.5\"\n").expect("loads")),
        Fixed::from_ratio(25, 2)
    );
    assert_eq!(
        damage(base("airfield_damage = 100\n").expect("loads")),
        Fixed::from_int(100)
    );
    let message = base("airfield = false\nairfield_damage = 10\n")
        .map(|_| ())
        .expect_err("refused")
        .to_string();
    assert!(
        message.contains("airfield_damage") && message.contains("no airfield"),
        "{message}"
    );
    let message = base("airfield_damage = 101\n")
        .map(|_| ())
        .expect_err("refused")
        .to_string();
    assert!(
        message.contains("airfield_damage") && message.contains("0 and 100"),
        "{message}"
    );
    assert!(base("airfield_damage = -1\n").is_err());
    assert!(base("airfield_damage = 1.5\n").is_err());
}

#[test]
fn an_air_group_may_not_start_at_a_base_with_no_airfield() {
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../data/scenarios/test.toml");
    let with_group = |extra: &str| {
        let head = HEAD.replace("q = 0\nr = 0\n", &format!("q = 0\nr = 0\n{extra}"));
        Scenario::parse(&format!("{head}\n{}", air_group(1, 1)), &path)
    };
    assert!(with_group("").is_ok());
    let message = with_group("airfield = false\n")
        .map(|_| ())
        .expect_err("refused")
        .to_string();
    assert!(message.contains("no airfield"), "{message}");
}

/// Decision 0101: a group with no aircraft and no pilots may start at a
/// base that is not its side's or has no airfield; with an aircraft or a
/// pilot it is refused as before.
#[test]
fn an_empty_air_group_may_start_at_an_enemy_base_or_one_with_no_airfield() {
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../data/scenarios/test.toml");
    let group = |base: u32, extra: &str| {
        format!(
                "[[air_group]]\nid = 1\nname = \"AG\"\nside = \"allied\"\nbase = {base}\naircraft_type = 1\n{extra}"
            )
    };
    let load = |head: &str, group: String| Scenario::parse(&format!("{head}\n{group}"), &path);
    let no_field = HEAD.replace("q = 0\nr = 0\n", "q = 0\nr = 0\nairfield = false\n");
    // The other side's base.
    assert!(load(HEAD, group(2, "ready = 0\n")).is_ok());
    assert!(load(HEAD, group(2, "ready = 1\n"))
        .expect_err("refused")
        .to_string()
        .contains("its base"));
    assert!(load(HEAD, group(2, "ready = 0\ndamaged = 1\n")).is_err());
    assert!(load(HEAD, group(2, "ready = 0\npilots = 1\n")).is_err());
    assert!(load(HEAD, group(2, "ready = 0\npilots = 0\n")).is_ok());
    // Its own side's base with no airfield.
    assert!(load(&no_field, group(1, "ready = 0\n")).is_ok());
    assert!(load(&no_field, group(1, "ready = 2\n"))
        .expect_err("refused")
        .to_string()
        .contains("no airfield"));
    // Both: the enemy's base with no airfield.
    let both = HEAD.replace("q = 5\nr = 0\n", "q = 5\nr = 0\nairfield = false\n");
    assert!(load(&both, group(2, "ready = 0\n")).is_ok());
    assert!(load(&both, group(2, "ready = 1\n")).is_err());
}
