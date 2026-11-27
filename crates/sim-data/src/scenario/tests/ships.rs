use super::*;

#[test]
fn task_force_placement_is_one_of_base_or_hex() {
    let at_base = parse_with(&task_force("at_base = 2")).expect("at a base loads");
    let hex = at_base.world.umpire().task_forces[&TaskForceId::new(1)].position();
    assert_eq!(hex, Hex::new(5, 0));
    let at_sea = parse_with(&task_force("q = 3\nr = -1")).expect("at sea loads");
    let hex = at_sea.world.umpire().task_forces[&TaskForceId::new(1)].position();
    assert_eq!(hex, Hex::new(3, -1));
    for bad in [
        "at_base = 1\nq = 3\nr = 0",
        "at_base = 1\nq = 3",
        "at_base = 1\nr = 3",
        "q = 3",
        "r = 3",
        "",
    ] {
        let message = error_of(&task_force(bad));
        assert!(message.contains("task_force:1"), "{bad}: {message}");
        assert!(
            message.contains("exactly one of at_base"),
            "{bad}: {message}"
        );
    }
}

#[test]
fn a_unit_numbered_in_the_range_kept_for_units_made_in_play_is_refused() {
    // Decision 0086: 1,000,000 and above belong to the sides' series.
    let high_force = task_force("at_base = 2").replace("id = 1", "id = 1000000");
    let error = error_of(&high_force);
    assert!(error.contains("decision 0086"), "{error}");
    assert!(error.contains("task_force:1000000"), "{error}");
    let high_group = air_group(1, 1).replace("id = 1", "id = 2000001");
    let error = error_of(&high_group);
    assert!(error.contains("decision 0086"), "{error}");
    assert!(error.contains("air_group:2000001"), "{error}");
    // The number just below the floor loads.
    parse_with(&task_force("at_base = 2").replace("id = 1", "id = 999999")).expect("999999 loads");
    parse_with(&air_group(1, 1).replace("id = 1", "id = 999999")).expect("999999 loads");
}

#[test]
fn a_ship_may_start_with_flotation_and_fire_damage_within_range() {
    let damage = |extra: &str| {
        parse_with(&with_force(&ship(extra))).map(|scenario| {
            let ship = scenario.world.umpire().ships[&ShipId::new(1)].clone();
            (ship.flotation_damage, ship.fire_damage)
        })
    };
    assert_eq!(damage("").expect("loads"), (Fixed::ZERO, Fixed::ZERO));
    assert_eq!(
        damage("flotation_damage = 12\nfire_damage = \"2.5\"").expect("loads"),
        (Fixed::from_int(12), Fixed::from_ratio(5, 2))
    );
    assert_eq!(
        damage("flotation_damage = \"99.999999\"\nfire_damage = 100").expect("loads"),
        (Fixed::from_raw(99_999_999), Fixed::from_int(100))
    );
    // Flotation: not negative, below 100, no float. Fire: not negative,
    // at most 100, no float.
    assert!(damage("flotation_damage = -1").is_err());
    assert!(damage("flotation_damage = 100").is_err());
    assert!(damage("flotation_damage = 101").is_err());
    assert!(damage("flotation_damage = 12.5").is_err());
    assert!(damage("fire_damage = -1").is_err());
    assert!(damage("fire_damage = 101").is_err());
    assert!(damage("fire_damage = 2.5").is_err());
}

#[test]
fn a_task_force_takes_an_optional_fatigue_in_whole_numbers_or_decimal_strings() {
    let fatigue = |extra: &str| {
        parse_with(&task_force(&format!("at_base = 1\n{extra}")))
            .map(|scenario| scenario.world.umpire().task_forces[&TaskForceId::new(1)].fatigue)
    };
    assert_eq!(fatigue("").expect("loads"), Fixed::ZERO);
    assert_eq!(fatigue("fatigue = 35").expect("loads"), Fixed::from_int(35));
    assert_eq!(
        fatigue("fatigue = \"2.5\"").expect("loads"),
        "2.5".parse::<Fixed>().expect("fixed")
    );
    assert_eq!(
        fatigue("fatigue = 100").expect("loads"),
        Fixed::from_int(100)
    );
    assert!(fatigue("fatigue = 101").is_err());
    assert!(fatigue("fatigue = -1").is_err());
    assert!(
        fatigue("fatigue = 3.5").is_err(),
        "a TOML float is rejected"
    );
}

#[test]
fn a_flight_deck_state_needs_a_class_that_carries_aircraft() {
    // Class 4 is the placeholder carrier, class 1 a cruiser.
    let carrier = |extra: &str| ship(extra).replace("class = 1", "class = 4");
    let deck = |extra: &str| {
        parse_with(&with_force(&carrier(extra)))
            .expect("loads")
            .world
            .umpire()
            .ships[&ShipId::new(1)]
            .systems
            .flight_deck
    };
    assert_eq!(deck(""), SystemState::Working);
    assert_eq!(deck("flight_deck = \"degraded\""), SystemState::Degraded);
    assert_eq!(deck("flight_deck = \"out\""), SystemState::Out);
    parse_with(&with_force(&ship("flight_deck = \"working\""))).expect("working loads");
    let message = error_of(&with_force(&ship("flight_deck = \"out\"")));
    assert!(message.contains("ship:1"), "{message}");
    assert!(
        message.contains("flight deck") && message.contains("has no"),
        "{message}"
    );
    assert!(parse_with(&with_force(&carrier("flight_deck = \"broken\""))).is_err());
}

#[test]
fn a_task_force_is_at_normal_readiness_unless_the_scenario_says_otherwise() {
    let truth = |scenario: Scenario| scenario.world.umpire().clone();
    let default = truth(parse_with(&task_force("at_base = 1")).expect("loads"));
    let force = &default.task_forces[&TaskForceId::new(1)];
    assert_eq!(force.readiness, Readiness::Normal);
    assert_eq!(force.readiness_order, None);
    assert!(!force.alerted);
    assert_eq!(force.alerted_by, None);
    assert!(!force.is_alert());
    assert_eq!(force.rules_of_engagement, RulesOfEngagement::DEFAULT);
    assert_eq!(force.rules_order, None);
    for (text, expected) in [
        ("readiness = \"normal\"", Readiness::Normal),
        ("readiness = \"battle\"", Readiness::Battle),
    ] {
        let loaded =
            truth(parse_with(&task_force(&format!("at_base = 1\n{text}"))).expect("loads"));
        let force = &loaded.task_forces[&TaskForceId::new(1)];
        assert_eq!(force.readiness, expected);
        assert_eq!(force.is_alert(), expected == Readiness::Battle);
    }
    assert!(parse_with(&task_force("at_base = 1\nreadiness = \"ready\"")).is_err());
    assert!(parse_with(&task_force("at_base = 1\nreadiness = true")).is_err());
    // The old key is gone: unknown fields are errors.
    assert!(parse_with(&task_force("at_base = 1\nalert = false")).is_err());
    assert!(parse_with(&task_force("at_base = 1\nalert = true")).is_err());
}

#[test]
fn a_ship_starts_with_the_guns_it_carries_and_its_torpedoes() {
    let scenario = parse_with(&with_force(&ship(""))).expect("loads");
    let truth = scenario.world.umpire();
    let cruiser = &truth.ships[&ShipId::new(1)];
    // Class 1: 8 guns x 100 rounds, 8 tubes.
    assert_eq!(cruiser.gun_rounds, Fixed::from_int(800));
    assert!(cruiser.torpedoes_ready);
    assert_eq!(cruiser.flotation_damage, Fixed::ZERO);
    assert_eq!(cruiser.systems, ShipSystems::ALL_WORKING);
    assert_eq!(cruiser.fire_damage, Fixed::ZERO);

    // Class 3 is a cargo ship: no guns and no tubes.
    let cargo = ship("").replace("class = 1", "class = 3");
    let scenario = parse_with(&with_force(&cargo)).expect("loads");
    let ship = &scenario.world.umpire().ships[&ShipId::new(1)];
    assert_eq!(ship.gun_rounds, Fixed::ZERO);
    assert!(!ship.torpedoes_ready);
}

#[test]
fn a_ship_has_experience_2_unless_the_scenario_gives_0_to_4() {
    let experience = |text: &str| {
        parse_with(&with_force(&ship(text)))
            .map(|s| s.world.umpire().ships[&ShipId::new(1)].experience)
    };
    assert_eq!(experience("").expect("loads"), 2);
    for value in 0..=4u8 {
        assert_eq!(
            experience(&format!("experience = {value}")).expect("loads"),
            value
        );
    }
    // Outside 0 to 4, below 0, a float or a string: refused, naming the
    // ship and the figure.
    for bad in ["5", "-1", "100", "2.0", "\"3\"", "\"high\""] {
        let result = experience(&format!("experience = {bad}"));
        assert!(result.is_err(), "experience = {bad}");
    }
    let message = experience("experience = 5").expect_err("refused").message;
    assert!(message.contains("experience 5"), "{message}");
}

#[test]
fn gun_rounds_are_a_whole_number_or_a_decimal_string() {
    let rounds = |text: &str| {
        parse_with(&with_force(&ship(&format!("gun_rounds = {text}"))))
            .map(|s| s.world.umpire().ships[&ShipId::new(1)].gun_rounds)
    };
    assert_eq!(rounds("120").expect("loads"), Fixed::from_int(120));
    assert_eq!(
        rounds("\"120.5\"").expect("loads"),
        Fixed::from_ratio(241, 2)
    );
    assert_eq!(rounds("0").expect("loads"), Fixed::ZERO);
    assert!(rounds("120.5").is_err());
    assert!(rounds("\"many\"").is_err());
    // A negative count starts the game in a state that breaks an
    // invariant.
    let message = rounds("-1").expect_err("rejected").message;
    assert!(message.contains("negative gun rounds"), "{message}");
}

#[test]
fn a_ship_starts_with_the_system_states_the_scenario_gives() {
    // Default: everything working. Class 1 has guns and tubes, so all
    // four systems may be set.
    let truth = |extra: &str| {
        parse_with(&with_force(&ship(extra)))
            .expect("loads")
            .world
            .umpire()
            .ships[&ShipId::new(1)]
            .systems
    };
    assert_eq!(truth(""), ShipSystems::ALL_WORKING);
    let systems = truth(
            "propulsion = \"degraded\"\nfire_control = \"out\"\nmain_battery = \"working\"\ntorpedo_mounts = \"out\"",
        );
    assert_eq!(systems.propulsion, SystemState::Degraded);
    assert_eq!(systems.fire_control, SystemState::Out);
    assert_eq!(systems.main_battery, SystemState::Working);
    assert_eq!(systems.torpedo_mounts, SystemState::Out);
}

#[test]
fn a_system_state_must_be_a_known_word_on_a_class_that_has_the_system() {
    assert!(parse_with(&with_force(&ship("propulsion = \"broken\""))).is_err());
    assert!(parse_with(&with_force(&ship("propulsion = 2"))).is_err());
    assert!(parse_with(&with_force(&ship("propulsions = \"out\""))).is_err());
    // Class 3 is a cargo ship with no guns and no tubes: only propulsion.
    let cargo = |extra: &str| ship(extra).replace("class = 1", "class = 3");
    parse_with(&with_force(&cargo("propulsion = \"out\""))).expect("propulsion loads");
    // A system the class lacks may be given as working, and nothing else.
    parse_with(&with_force(&cargo("main_battery = \"working\""))).expect("working loads");
    for (key, state) in [
        ("main_battery", "degraded"),
        ("fire_control", "out"),
        ("torpedo_mounts", "degraded"),
    ] {
        let message = error_of(&with_force(&cargo(&format!("{key} = \"{state}\""))));
        assert!(message.contains("ship:1"), "{message}");
        assert!(
            message.contains(&key.replace('_', " ")) && message.contains("has no"),
            "{message}"
        );
    }
}

#[test]
fn a_task_force_may_have_a_home_port_of_its_own_side() {
    let force = |extra: &str| {
        parse_with(&task_force(&format!("at_base = 1\n{extra}")))
            .expect("loads")
            .world
            .umpire()
            .task_forces[&TaskForceId::new(1)]
            .home_port
    };
    assert_eq!(force(""), None);
    assert_eq!(force("home_port = 1"), Some(BaseId::new(1)));
    // Base 9 does not exist; base 2 is Japanese and the force is Allied.
    let message = error_of(&task_force("at_base = 1\nhome_port = 9"));
    assert!(message.contains("task_force:1") && message.contains("home port"));
    let message = error_of(&task_force("at_base = 1\nhome_port = 2"));
    assert!(
        message.contains("home port") && message.contains("Japanese"),
        "{message}"
    );
    assert!(parse_with(&task_force("at_base = 1\nhome_port = \"one\"")).is_err());
    assert!(parse_with(&task_force("at_base = 1\nhome_ports = 1")).is_err());
}

#[test]
fn a_ship_may_start_with_fuel_cargo_that_fits_beside_its_supply() {
    // Class 3 carries 3000 t.
    let cargo = |extra: &str| with_force(&ship(extra).replace("class = 1", "class = 3"));
    let aboard = |scenario: Scenario| {
        let ship = scenario.world.umpire().ships[&ShipId::new(1)].clone();
        (ship.supply_t, ship.cargo_fuel_t)
    };
    assert_eq!(
        aboard(parse_with(&cargo("")).expect("loads")),
        (Fixed::ZERO, Fixed::ZERO)
    );
    assert_eq!(
        aboard(parse_with(&cargo("cargo_fuel_t = 3000")).expect("loads")),
        (Fixed::ZERO, Fixed::from_int(3000))
    );
    assert_eq!(
        aboard(parse_with(&cargo("supply_t = 2000\ncargo_fuel_t = 1000")).expect("loads")),
        (Fixed::from_int(2000), Fixed::from_int(1000))
    );
    // Over the capacity together, though each is under it.
    let message = error_of(&cargo("supply_t = 2000\ncargo_fuel_t = 1001"));
    assert!(
        message.contains("fuel cargo but its class"),
        "unexpected message: {message}"
    );
    assert!(parse_with(&cargo("cargo_fuel_t = 3001")).is_err());
    // Not negative, not a float.
    assert!(parse_with(&cargo("cargo_fuel_t = -1")).is_err());
    assert!(parse_with(&cargo("cargo_fuel_t = 1.5")).is_err());
    // A class with no cargo space carries none.
    assert!(parse_with(&with_force(&ship("cargo_fuel_t = 1"))).is_err());
}

#[test]
fn a_ship_with_more_fuel_than_its_class_holds_is_rejected() {
    // The helper's ship has 100 t, which its class holds. One with far
    // more does not load.
    assert!(parse_with(&with_force(&ship(""))).is_ok());
    let overfull = ship("").replace("fuel_t = 100", "fuel_t = 999999");
    let message = error_of(&with_force(&overfull));
    assert!(
        message.contains("of fuel but its class"),
        "unexpected message: {message}"
    );
}

// Forces that arrive (decision 0195): the key `enters`.

/// `HEAD` for a game of six days with a ground file, then `extra`.
fn arriving(extra: &str) -> Result<Scenario, DataError> {
    arriving_on("placeholder_search", "", extra)
}

/// As `arriving`, on the named map, with `top` added to the top keys.
fn arriving_on(map: &str, top: &str, extra: &str) -> Result<Scenario, DataError> {
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../data/scenarios/test.toml");
    let head = HEAD
            .replace("placeholder_search", map)
            .replace("days = 2\n", &format!("days = 2\n{top}"))
            .replace("days = 2\n", "days = 6\n")
            .replace(
                "ship_classes = \"../reference/ship_classes.csv\"",
                "ship_classes = [\"../reference/ship_classes.csv\", \"../reference/ship_classes_placeholder_transports.csv\"]",
            )
            .replace("search = ", "ground = \"../reference/ground.toml\"\nsearch = ");
    Scenario::parse(&format!("{head}{extra}"), &path)
}

fn arriving_error(extra: &str) -> String {
    arriving(extra)
        .expect_err("the scenario is refused")
        .message
}

/// Allied task force 1 (ships 1 and 2, a carrier with air group 1
/// aboard, 400 men of land unit 1 aboard) with `force_extra`.
fn arriving_force(force_extra: &str) -> String {
    format!(
            "\n[[task_force]]\nid = 1\nname = \"Force\"\nside = \"allied\"\nq = 3\nr = 0\nmission = \"transport\"\n{force_extra}\n\
             [[ship]]\nid = 1\nname = \"S1\"\nclass = 10\nside = \"allied\"\nfuel_t = 100\ntask_force = 1\n\
             [[ship]]\nid = 2\nname = \"S2\"\nclass = 4\nside = \"allied\"\nfuel_t = 100\ntask_force = 1\n\
             [[air_group]]\nid = 1\nname = \"AG1\"\nside = \"allied\"\nship = 2\naircraft_type = 1\nready = 4\n\
             [[land_unit]]\nid = 1\nname = \"LU1\"\nside = \"allied\"\nmen = 400\ntask_force = 1\n"
        )
}

const ARRIVING_GROUP_2: &str = "\n[[air_group]]\nid = 2\nname = \"AG2\"\nside = \"allied\"\nbase = 1\naircraft_type = 1\nready = 3\n";

const ARRIVING_UNIT_2: &str =
    "\n[[land_unit]]\nid = 2\nname = \"LU2\"\nside = \"allied\"\nmen = 300\nq = 0\nr = 0\n";

#[test]
fn units_that_enter_are_held_apart_and_a_seeded_copy_holds_the_same() {
    let text = format!(
            "{}{}{}\n[[arrival]]\nday = 3\nair_group = 2\naircraft = 2\n\n[[arrival]]\nday = 3\nair_group = 1\naircraft = 1\n",
            arriving_force("enters = 3"),
            ARRIVING_GROUP_2.replace("ready = 3", "ready = 3\nenters = 3"),
            ARRIVING_UNIT_2.replace("r = 0", "r = 0\nenters = 3"),
        );
    let scenario = arriving(&text).expect("loads");
    let truth = scenario.world.umpire();
    let keys: Vec<EntityRef> = truth.entering.keys().copied().collect();
    assert_eq!(
        keys,
        vec![
            EntityRef::TaskForce(TaskForceId::new(1)),
            EntityRef::AirGroup(AirGroupId::new(2)),
            EntityRef::LandUnit(LandUnitId::new(2)),
        ]
    );
    assert!(truth.task_forces.is_empty());
    assert!(truth.ships.is_empty());
    assert!(truth.air_groups.is_empty());
    assert!(truth.land_units.is_empty());
    let force = &truth.entering[&keys[0]];
    assert_eq!(force.day, 3);
    assert_eq!(force.ships.len(), 2);
    assert_eq!(force.air_groups.len(), 1);
    assert_eq!(force.land_units.len(), 1);
    assert_eq!(force.land_units[&LandUnitId::new(1)].men_ashore, 0);
    let other = scenario.with_seed(9).expect("a seeded copy");
    assert_eq!(
        other
            .world
            .umpire()
            .entering
            .keys()
            .copied()
            .collect::<Vec<_>>(),
        keys
    );
    assert!(other.world.umpire().task_forces.is_empty());
    // A scenario with no `enters` holds nothing.
    let plain = arriving(&arriving_force("")).expect("loads");
    assert!(plain.world.umpire().entering.is_empty());
    assert_eq!(plain.world.umpire().ships.len(), 2);
}

#[test]
fn an_arrival_dated_on_its_groups_day_is_taken_and_one_before_is_refused() {
    let group = ARRIVING_GROUP_2.replace("ready = 3", "ready = 3\nenters = 4");
    let arrival = |day: u32, group: u32| {
        format!("\n[[arrival]]\nday = {day}\nair_group = {group}\naircraft = 1\n")
    };
    let ok = arriving(&format!("{group}{}", arrival(4, 2))).expect("on the day");
    assert_eq!(ok.world.umpire().arrivals.len(), 1);
    let message = arriving_error(&format!("{group}{}", arrival(3, 2)));
    assert!(
        message.contains("before air group 2 enters on day 4"),
        "{message}"
    );
    // A group aboard a ship takes its force's day.
    let force = arriving_force("enters = 5");
    let message = arriving_error(&format!("{force}{}", arrival(4, 1)));
    assert!(
        message.contains("before air group 1 enters on day 5"),
        "{message}"
    );
    arriving(&format!("{force}{}", arrival(5, 1))).expect("on the force's day");
}

#[test]
fn enters_is_refused_with_its_reason() {
    for (extra, words) in [
        (
            arriving_force("enters = 1"),
            "task force 1 has enters = 1: a unit there on day 1 needs no `enters`",
        ),
        (arriving_force("enters = 0"), "has enters = 0"),
        (
            arriving_force("enters = 7"),
            "enters = 7, after the last day of the scenario (6)",
        ),
        (
            arriving_force("").replace(
                "fuel_t = 100\ntask_force = 1\n[[ship]]",
                "fuel_t = 100\ntask_force = 1\nenters = 3\n[[ship]]",
            ),
            "ship 1 has `enters`: a ship enters with its task force",
        ),
        (
            arriving_force("").replace("ready = 4\n", "ready = 4\nenters = 3\n"),
            "air group 1 is aboard a ship and has `enters`",
        ),
    ] {
        let message = arriving_error(&extra);
        assert!(message.contains(words), "{words}: {message}");
    }
    let aboard = arriving_force("").replace("men = 400\n", "men = 400\nenters = 3\n");
    let message = arriving_error(&aboard);
    assert!(
        message.contains("land unit 1 is aboard a task force and has `enters`"),
        "{message}"
    );
    // Group and unit at a base: below 2 and above the days.
    let message = arriving_error(&ARRIVING_GROUP_2.replace("ready = 3", "ready = 3\nenters = 1"));
    assert!(message.contains("air group 2 has enters = 1"), "{message}");
    let message = arriving_error(&ARRIVING_UNIT_2.replace("r = 0", "r = 0\nenters = 9"));
    assert!(message.contains("land unit 2 has enters = 9"), "{message}");
}

#[test]
fn a_force_enters_at_sea_or_coast_on_the_map_only() {
    let off = arriving_force("enters = 3").replace("q = 3", "q = 500");
    let message = arriving_error(&off);
    assert!(
        message.contains("task force 1 enters on day 3 at 500,0, which is off the map"),
        "{message}"
    );
    // A land hex, on a map with land.
    let land = arriving_force("enters = 3").replace("q = 3", "q = 4");
    let message = arriving_on("placeholder_island", "", &land)
        .expect_err("refused")
        .message;
    assert!(
        message.contains("task force 1 enters on day 3 at 4,0, which is land"),
        "{message}"
    );
    let sea = arriving_force("enters = 3");
    arriving_on("placeholder_island", "", &sea).expect("sea is fine");
}

#[test]
fn a_land_unit_that_enters_needs_a_base_in_its_hex() {
    // (4,0) is land on the island map, with no base.
    let unit = ARRIVING_UNIT_2.replace("q = 0\nr = 0", "q = 4\nr = 0\nenters = 3");
    let message = arriving_on("placeholder_island", "", &unit)
        .expect_err("refused")
        .message;
    assert!(message.contains("land_unit:2 enters on day 3"), "{message}");
    assert!(message.contains("where there is no base"), "{message}");
}

#[test]
fn an_escort_and_its_charge_enter_on_one_day() {
    let second = |extra: &str| {
        format!(
                "\n[[task_force]]\nid = 2\nname = \"Escort\"\nside = \"allied\"\nq = 3\nr = 0\nmission = \"escort\"\n{extra}\n\
                 [[ship]]\nid = 9\nname = \"S9\"\nclass = 1\nside = \"allied\"\nfuel_t = 100\ntask_force = 2\n\
                 [[order]]\nday = 4\nside = \"allied\"\ntask_force = 2\nkind = \"escort\"\ncharge = 1\n"
            )
    };
    // Both enter on day 3.
    arriving(&format!(
        "{}{}",
        arriving_force("enters = 3"),
        second("enters = 3")
    ))
    .expect("one day");
    // Neither enters.
    arriving(&format!("{}{}", arriving_force(""), second(""))).expect("neither");
    let message = arriving_error(&format!("{}{}", arriving_force("enters = 3"), second("")));
    assert!(message.contains("task force 2, which is there from day 1, to escort task force 1, which enters on day 3"), "{message}");
    let message = arriving_error(&format!("{}{}", arriving_force(""), second("enters = 3")));
    assert!(message.contains("task force 2, which enters on day 3, to escort task force 1, which is there from day 1"), "{message}");
    let message = arriving_error(&format!(
        "{}{}",
        arriving_force("enters = 3"),
        second("enters = 4")
    ));
    assert!(
        message.contains("an escort and its charge enter on one day"),
        "{message}"
    );
}

#[test]
fn a_held_force_may_have_a_commander_and_radio_silence_but_is_no_superior() {
    let levels = |extra: &str| arriving_on("placeholder_search", "command_levels = true\n", extra);
    let held = arriving_force("enters = 3\nradio_silence = true\nsuperior = { base = 1 }");
    let commander = "\n[[commander]]\nside = \"allied\"\ntask_force = 1\nname = \"Hold\"\ninitiative = \"bold\"\nnote = \"a placeholder\"\n";
    let scenario = levels(&format!("{held}{commander}")).expect("a held force may be commanded");
    let entry = &scenario.world.umpire().entering[&EntityRef::TaskForce(TaskForceId::new(1))];
    assert!(entry.radio_silent);
    assert_eq!(
        entry.commander.as_ref().map(|c| c.name.as_str()),
        Some("Hold")
    );
    assert!(entry.superior.is_some());
    let again = scenario.with_seed(3).expect("a seeded copy");
    assert!(again.world.umpire().entering[&EntityRef::TaskForce(TaskForceId::new(1))].radio_silent);
    // A held force named as another's superior is refused, with sim-core's reason.
    let other = "\n[[task_force]]\nid = 2\nname = \"Other\"\nside = \"allied\"\nat_base = 1\nmission = \"escort\"\nsuperior = { task_force = 1 }\n\
                     [[ship]]\nid = 9\nname = \"S9\"\nclass = 1\nside = \"allied\"\nfuel_t = 100\ntask_force = 2\n";
    let message = levels(&format!("{}{other}", arriving_force("enters = 3")))
        .expect_err("refused")
        .message;
    assert!(message.contains("command"), "{message}");
}
