use super::*;

#[test]
fn air_groups_are_checked() {
    let scenario = parse_with(&air_group(1, 1)).expect("a good air group loads");
    let group = &scenario.world.umpire().air_groups[&AirGroupId::new(1)];
    assert_eq!(group.ready, 3);
    assert_eq!(group.mission, AirMission::Rest);
    assert_eq!(group.search, None);
    assert!(error_of(&air_group(9, 1)).contains("unknown base"));
    assert!(error_of(&air_group(2, 1)).contains("its base"));
    assert!(error_of(&air_group(1, 9)).contains("unknown aircraft type"));
    let twice = format!("{}{}", air_group(1, 1), air_group(1, 1));
    assert!(error_of(&twice).contains("appears twice"));
}

#[test]
fn pilots_and_strength_load_and_a_group_without_them_is_untracked() {
    let plain = parse_with(&air_group(1, 1)).expect("loads");
    let group = &plain.world.umpire().air_groups[&AirGroupId::new(1)];
    assert_eq!((group.pilots, group.strength), (None, 3));
    let with = |extra: &str| parse_with(&format!("{}{extra}", air_group(1, 1))).expect("loads");
    let tracked = with("pilots = 2\n");
    let group = &tracked.world.umpire().air_groups[&AirGroupId::new(1)];
    assert_eq!((group.pilots, group.strength), (Some(2), 3));
    let given = with("pilots = 5\nstrength = 9\n");
    let group = &given.world.umpire().air_groups[&AirGroupId::new(1)];
    assert_eq!((group.pilots, group.strength), (Some(5), 9));
    // The default strength counts damaged aircraft too.
    let damaged = with("damaged = 4\n");
    let group = &damaged.world.umpire().air_groups[&AirGroupId::new(1)];
    assert_eq!(group.strength, 7);
    assert!(parse_with(&format!("{}pilots = 1.5\n", air_group(1, 1))).is_err());
}

fn replacement(side: &str, base: u32, aircraft_type: u32) -> String {
    format!(
            "[[replacement]]\nside = \"{side}\"\nbase = {base}\naircraft_type = {aircraft_type}\naircraft_per_day = 2\npilots_per_day = 1\n"
        )
}

#[test]
fn replacement_streams_are_kept_in_one_order_whatever_the_file_gives() {
    let a = format!(
        "{}{}{}",
        replacement("japanese", 2, 2),
        replacement("allied", 1, 2),
        replacement("allied", 1, 1)
    );
    let b = format!(
        "{}{}{}",
        replacement("allied", 1, 1),
        replacement("japanese", 2, 2),
        replacement("allied", 1, 2)
    );
    let (a, b) = (
        parse_with(&a).expect("loads"),
        parse_with(&b).expect("loads"),
    );
    let key = |s: &Scenario| -> Vec<(u8, u32, u32)> {
        s.world
            .umpire()
            .replacements
            .iter()
            .map(|r| (r.side.code(), r.base.raw(), r.aircraft_type.raw()))
            .collect()
    };
    // Side, then base, then type (decision 0043, 4c).
    assert_eq!(key(&a), vec![(1, 1, 1), (1, 1, 2), (2, 2, 2)]);
    assert_eq!(key(&a), key(&b));
    assert_eq!(a.world.state_hash(), b.world.state_hash());
}

#[test]
fn replacement_tables_load_in_order_and_are_checked() {
    let none = parse_with("").expect("loads");
    assert!(none.world.umpire().replacements.is_empty());
    let both = format!(
        "{}{}",
        replacement("allied", 1, 1),
        replacement("japanese", 2, 2)
    );
    let scenario = parse_with(&both).expect("loads");
    let streams = &scenario.world.umpire().replacements;
    assert_eq!(streams.len(), 2);
    assert_eq!(
        (
            streams[0].side,
            streams[0].base.raw(),
            streams[0].aircraft_type.raw(),
            streams[0].aircraft_per_day,
            streams[0].pilots_per_day
        ),
        (Side::Allied, 1, 1, 2, 1)
    );
    assert_eq!(
        (streams[1].side, streams[1].base.raw()),
        (Side::Japanese, 2)
    );
    assert!(error_of(&replacement("allied", 9, 1)).contains("unknown base"));
    assert!(error_of(&replacement("allied", 2, 1)).contains("its base"));
    assert!(error_of(&replacement("allied", 1, 9)).contains("unknown aircraft type"));
    assert!(parse_with(&replacement("allied", 1, 1).replace("= 2\n", "= 2.5\n")).is_err());
    assert!(parse_with(&format!("{}extra = 1\n", replacement("allied", 1, 1))).is_err());
}

#[test]
fn arrival_tables_load_in_one_order_whatever_the_file_gives_and_are_checked() {
    let none = parse_with("").expect("loads");
    assert!(none.world.umpire().arrivals.is_empty());
    let tracked = air_group(1, 1).replace("ready = 3", "ready = 0\npilots = 0");
    let untracked = air_group(1, 1).replace("ready = 3", "ready = 0");
    let text = format!(
        "{tracked}{}{}",
        arrival(2, 1, 6, "pilots = 5\n"),
        arrival(1, 1, 2, "")
    );
    let scenario = parse_with(&text).expect("loads");
    let truth = scenario.world.umpire();
    let group = &truth.air_groups[&AirGroupId::new(1)];
    assert_eq!((group.ready, group.pilots, group.strength), (0, Some(0), 0));
    let listed: Vec<_> = truth
        .arrivals
        .iter()
        .map(|a| (a.day, a.air_group.raw(), a.aircraft, a.pilots))
        .collect();
    // Listed as day 2 then day 1; kept by day (decision 0043, 4c).
    assert_eq!(listed, vec![(1, 1, 2, 0), (2, 1, 6, 5)]);
    // A group with no aircraft and no pilots key is loaded, untracked.
    let loaded = parse_with(&untracked).expect("loads");
    let group = &loaded.world.umpire().air_groups[&AirGroupId::new(1)];
    assert_eq!((group.ready, group.pilots), (0, None));
    // Aircraft alone, to an untracked group.
    assert!(parse_with(&format!("{untracked}{}", arrival(1, 1, 4, ""))).is_ok());
    let with = |arrival: String| error_of(&format!("{tracked}{arrival}"));
    assert!(with(arrival(1, 9, 1, "")).contains("unknown air group"));
    assert!(with(arrival(0, 1, 1, "")).contains("outside days"));
    assert!(with(arrival(3, 1, 1, "")).contains("outside days"));
    assert!(with(arrival(1, 1, 0, "")).contains("brings nothing"));
    assert!(with(arrival(1, 1, 0, "pilots = 0\n")).contains("brings nothing"));
    assert!(
        error_of(&format!("{untracked}{}", arrival(1, 1, 2, "pilots = 1\n")))
            .contains("not tracked")
    );
    assert!(parse_with(&format!("{tracked}{}extra = 1\n", arrival(1, 1, 1, ""))).is_err());
    assert!(parse_with(
        &format!("{tracked}{}", arrival(1, 1, 1, "")).replace("= 1\naircraft", "= 1.5\naircraft")
    )
    .is_err());
}

#[test]
fn an_arrival_that_would_overfill_a_ship_is_refused() {
    // Class 4 carries 36: 30 aboard, then 6 on day 1 fits, 7 does not.
    let base = format!(
        "{}{}",
        force_with_ship(4),
        carried_group(1, "ship = 1", 30, "allied")
    );
    parse_with(&format!("{base}{}", arrival(1, 1, 6, ""))).expect("30 and 6 fit in 36");
    let message = error_of(&format!("{base}{}", arrival(1, 1, 7, "")));
    assert!(
        message.contains("arrival 1 would put 37 aircraft aboard ship:1")
            && message.contains("class carries 36"),
        "{message}"
    );
    // Arrivals add up by day, across the ship's groups: the second, on
    // day 2, is the one that overfills it (30 + 3 + 4 = 37).
    let two = format!(
        "{base}{}{}{}",
        carried_group(2, "ship = 1", 0, "allied"),
        arrival(1, 1, 3, ""),
        arrival(2, 2, 4, "")
    );
    let message = error_of(&two);
    assert!(message.contains("arrival 2 would put 37"), "{message}");
    // An arrival to a group at a base is not a ship's concern.
    parse_with(&format!("{}{}", air_group(1, 1), arrival(1, 1, 500, "")))
        .expect("a base takes any number");
}

/// An Allied group aboard a ship, `extra` replacing the placement.
fn carried_group(id: u32, placement: &str, ready: u32, side: &str) -> String {
    format!(
            "[[air_group]]\nid = {id}\nname = \"AG\"\nside = \"{side}\"\n{placement}\naircraft_type = 1\nready = {ready}\n"
        )
}

/// Task force 1 holding Allied ship 1 of the given class.
fn force_with_ship(class: u32) -> String {
    with_force(&ship("").replace("class = 1", &format!("class = {class}")))
}

#[test]
fn an_air_group_may_be_aboard_a_ship_with_the_capacity() {
    // Class 4 is the placeholder carrier, which carries 36.
    let text = format!(
        "{}{}",
        force_with_ship(4),
        carried_group(1, "ship = 1", 36, "allied")
    );
    let scenario = parse_with(&text).expect("a carried group loads");
    let group = &scenario.world.umpire().air_groups[&AirGroupId::new(1)];
    assert_eq!(group.location, AirGroupLocation::Ship(ShipId::new(1)));
    assert_eq!(group.ready, 36);
    // The group is where the ship's task force is.
    assert_eq!(
        scenario.world.umpire().air_group_hex(group),
        Some(Hex::new(0, 0))
    );
    // Two groups may share a ship while they fit.
    let two = format!(
        "{}{}{}",
        force_with_ship(4),
        carried_group(1, "ship = 1", 30, "allied"),
        carried_group(2, "ship = 1", 6, "allied")
    );
    parse_with(&two).expect("30 and 6 fit in 36");
}

#[test]
fn an_air_group_needs_exactly_one_of_base_and_ship() {
    for placement in ["base = 1\nship = 1", ""] {
        let text = format!(
            "{}{}",
            force_with_ship(4),
            carried_group(1, placement, 3, "allied")
        );
        let message = error_of(&text);
        assert!(message.contains("air_group:1"), "{placement}: {message}");
        assert!(
            message.contains("exactly one of base and ship"),
            "{placement}: {message}"
        );
    }
}

#[test]
fn a_carried_air_group_is_checked_against_its_ship() {
    let carried = |class: u32, placement: &str, ready: u32, side: &str| {
        error_of(&format!(
            "{}{}",
            force_with_ship(class),
            carried_group(1, placement, ready, side)
        ))
    };
    // A ship that is not in the scenario.
    let message = carried(4, "ship = 9", 3, "allied");
    assert!(
        message.contains("air_group:1") && message.contains("unknown ship"),
        "{message}"
    );
    // A ship of the other side.
    let message = carried(4, "ship = 1", 3, "japanese");
    assert!(
        message.contains("air_group:1") && message.contains("its ship"),
        "{message}"
    );
    // A ship whose class carries nothing (class 1 is a cruiser).
    let message = carried(1, "ship = 1", 1, "allied");
    assert!(
        message.contains("air_group:1") && message.contains("carries 0"),
        "{message}"
    );
    // One aircraft over the capacity of 36.
    let message = carried(4, "ship = 1", 37, "allied");
    assert!(
        message.contains("air_group:1") && message.contains("carries 36"),
        "{message}"
    );
    // Two groups that together are over it.
    let text = format!(
        "{}{}{}",
        force_with_ship(4),
        carried_group(1, "ship = 1", 30, "allied"),
        carried_group(2, "ship = 1", 7, "allied")
    );
    let message = error_of(&text);
    assert!(
        message.contains("air_group:2") && message.contains("37 aircraft"),
        "{message}"
    );
}

#[test]
fn an_air_group_takes_an_optional_whole_number_of_damaged_aircraft() {
    let damaged = |extra: &str| {
        let text = air_group(1, 3).replace("ready = ", &format!("{extra}ready = "));
        parse_with(&text)
            .map(|scenario| scenario.world.umpire().air_groups[&AirGroupId::new(1)].damaged)
    };
    assert_eq!(damaged("").expect("loads"), 0);
    assert_eq!(damaged("damaged = 4\n").expect("loads"), 4);
    assert!(damaged("damaged = 1.5\n").is_err());
    assert!(damaged("damaged = -1\n").is_err());
}

#[test]
fn an_air_group_takes_an_optional_fatigue_in_whole_numbers_or_decimal_strings() {
    let fatigue = |extra: &str| {
        let text = air_group(1, 3).replace("ready = ", &format!("{extra}ready = "));
        parse_with(&text)
            .map(|scenario| scenario.world.umpire().air_groups[&AirGroupId::new(1)].fatigue)
    };
    assert_eq!(fatigue("").expect("loads"), Fixed::ZERO);
    assert_eq!(
        fatigue("fatigue = 40\n").expect("loads"),
        Fixed::from_int(40)
    );
    assert_eq!(
        fatigue("fatigue = \"6.2\"\n").expect("loads"),
        "6.2".parse::<Fixed>().expect("fixed")
    );
    assert!(fatigue("fatigue = 6.2\n").is_err());
    assert!(fatigue("fatigue = -1\n").is_err());
    assert!(fatigue("fatigue = 101\n").is_err());
    assert_eq!(
        fatigue("fatigue = 100\n").expect("loads"),
        Fixed::from_int(100)
    );
}

#[test]
fn an_air_group_takes_optional_strike_settings_checked_at_load() {
    let group = |extra: &str| {
        let text = air_group(1, 3).replace("ready = ", &format!("{extra}ready = "));
        parse_with(&text).map(|scenario| {
            let group = scenario.world.umpire().air_groups[&AirGroupId::new(1)].clone();
            (
                group.strike_contact_age,
                group.strike_held_share,
                group.strike_age_order,
                group.strike_share_order,
            )
        })
    };
    assert_eq!(group("").expect("loads"), (None, Fixed::ZERO, None, None));
    assert_eq!(
        group("strike_contact_age_pulses = 0\nstrike_held_share = \"0.25\"\n").expect("loads"),
        (Some(0), Fixed::from_ratio(1, 4), None, None)
    );
    // The search figures drop a contact at 5 pulses, so 4 is the top.
    assert_eq!(
        group("strike_contact_age_pulses = 4\n").expect("loads").0,
        Some(4)
    );
    assert!(error_of_group(&group("strike_contact_age_pulses = 5\n")).contains("between 0 and 4"));
    assert!(group("strike_contact_age_pulses = -1\n").is_err());
    assert!(group("strike_contact_age_pulses = 1.5\n").is_err());
    assert_eq!(
        group("strike_held_share = 1\n").expect("loads").1,
        Fixed::ONE
    );
    assert!(group("strike_held_share = \"1.5\"\n").is_err());
    assert!(group("strike_held_share = \"-0.1\"\n").is_err());
    assert!(group("strike_held_share = 0.5\n").is_err());
}
