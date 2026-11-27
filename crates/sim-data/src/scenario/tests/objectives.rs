use super::*;

// -----------------------------------------------------------------
// Objectives (decision 0054, "As built").
// -----------------------------------------------------------------

/// An `[objectives]` section with the given levels line and the given
/// body for the allied objectives. The Japanese side has a mission and
/// no objective.
fn objectives(levels: &str, allied: &str) -> String {
    format!(
            "\n[objectives]\n{levels}\nhistory = \"A note.\"\n\n[objectives.allied]\nmission = \"Hold base one.\"\n{allied}\n[objectives.japanese]\nmission = \"Take base one.\"\n"
        )
}

const LEVELS: &str = "minor = 3\nmajor = 8\ndecisive = 16";

#[test]
fn men_landed_loads_with_its_base_and_points_and_is_refused_as_the_others_are() {
    let body = "[[objectives.allied.objective]]\nkind = \"men_landed\"\nbase = 1\npoints_per_100_men = \"0.5\"\n";
    let scenario = parse_with(&objectives(LEVELS, body)).expect("loads");
    let found = scenario.world.objectives().expect("has objectives");
    assert_eq!(
        found.allied.objectives,
        vec![Objective::MenLanded {
            base: BaseId::new(1),
            points_per_100_men: "0.5".parse().expect("a decimal"),
        }]
    );
    // The world keeps the hex of the base named.
    let hex = scenario.world.umpire().bases[&BaseId::new(1)].hex;
    assert_eq!(found.base_hexes.get(&BaseId::new(1)), Some(&hex));
    let message = error_of(&objectives(
            LEVELS,
            "[[objectives.allied.objective]]\nkind = \"men_landed\"\nbase = 1\npoints_per_100_men = -1\n",
        ));
    assert!(
        message.contains("points_per_100_men is negative"),
        "{message}"
    );
    let message = error_of(&objectives(
            LEVELS,
            "[[objectives.allied.objective]]\nkind = \"men_landed\"\nbase = 77\npoints_per_100_men = 1\n",
        ));
    assert!(message.contains("unknown base:77"), "{message}");
}

#[test]
fn submarines_sunk_loads_with_its_points_and_is_refused_as_the_others_are() {
    let body = "[[objectives.allied.objective]]\nkind = \"submarines_sunk\"\npoints = \"2.5\"\n";
    let scenario = parse_with(&objectives(LEVELS, body)).expect("loads");
    let found = scenario.world.objectives().expect("has objectives");
    assert_eq!(
        found.allied.objectives,
        vec![Objective::SubmarinesSunk {
            points: "2.5".parse().expect("a decimal"),
        }]
    );
    assert_eq!(found.allied.objectives[0].name(), "submarines_sunk");
    let message = error_of(&objectives(
        LEVELS,
        "[[objectives.allied.objective]]\nkind = \"submarines_sunk\"\npoints = -1\n",
    ));
    assert!(message.contains("points is negative"), "{message}");
}

#[test]
fn a_scenario_is_of_record_unless_its_objectives_say_it_is_for_play() {
    let of = |levels: &str| {
        parse_with(&objectives(levels, ""))
            .expect("loads")
            .world
            .objectives()
            .expect("has objectives")
            .for_play
    };
    assert!(!of(LEVELS));
    assert!(!of(&format!("{LEVELS}\nfor_play = false")));
    assert!(of(&format!("{LEVELS}\nfor_play = true")));
    let message = error_of(&objectives(&format!("{LEVELS}\nfor_play = 1"), ""));
    assert!(!message.is_empty());
}

const ALL_KINDS: &str = r#"
[[objectives.allied.objective]]
kind = "ships_sunk"
points = { carrier = 10, destroyer = 1 }

[[objectives.allied.objective]]
kind = "ships_crippled"
points = { carrier = 5, destroyer = "0.5" }

[[objectives.allied.objective]]
kind = "aircraft_destroyed"
points = "0.05"

[[objectives.allied.objective]]
kind = "cargo_landed"
base = 1
points_per_t = "0.1"

[[objectives.allied.objective]]
kind = "base_open"
base = 2
closed = true
points = 4
"#;

#[test]
fn a_scenario_may_give_each_side_a_mission_and_objectives_of_the_five_kinds() {
    let scenario = parse_with(&objectives(LEVELS, ALL_KINDS)).expect("loads");
    let found = scenario.world.objectives().expect("has objectives");
    assert_eq!(found.allied.mission, "Hold base one.");
    assert_eq!(found.japanese.mission, "Take base one.");
    assert_eq!(found.japanese.objectives.len(), 0);
    assert_eq!(found.history.as_deref(), Some("A note."));
    assert_eq!(
        (
            found.levels.minor,
            found.levels.major,
            found.levels.decisive
        ),
        (Fixed::from_int(3), Fixed::from_int(8), Fixed::from_int(16))
    );
    let kinds: Vec<&str> = found
        .allied
        .objectives
        .iter()
        .map(Objective::name)
        .collect();
    assert_eq!(
        kinds,
        [
            "ships_sunk",
            "ships_crippled",
            "aircraft_destroyed",
            "cargo_landed",
            "base_open"
        ]
    );
    match &found.allied.objectives[1] {
        Objective::ShipsCrippled(points) => {
            assert_eq!(points.of(ShipKind::Carrier), Fixed::from_int(5));
            assert_eq!(points.of(ShipKind::Destroyer), Fixed::from_ratio(1, 2));
            // A kind not listed is worth nothing.
            assert_eq!(points.of(ShipKind::Cruiser), Fixed::ZERO);
        }
        other => panic!("{other:?}"),
    }
    assert_eq!(
        found.allied.objectives[4],
        Objective::BaseOpen {
            base: BaseId::new(2),
            closed: true,
            points: Fixed::from_int(4)
        }
    );
    // Whole numbers and decimal strings both read; `closed` defaults to
    // false and `history` is optional.
    let plain = parse_with(
            "\n[objectives]\nminor = \"0.5\"\nmajor = 1\ndecisive = 2\n\n[objectives.allied]\nmission = \"A.\"\n\n[[objectives.allied.objective]]\nkind = \"base_open\"\nbase = 1\npoints = 1\n\n[objectives.japanese]\nmission = \"B.\"\n",
        )
        .expect("loads");
    let found = plain.world.objectives().expect("has objectives");
    assert_eq!(found.history, None);
    assert_eq!(
        found.allied.objectives[0],
        Objective::BaseOpen {
            base: BaseId::new(1),
            closed: false,
            points: Fixed::ONE
        }
    );
}

#[test]
fn a_scenario_without_objectives_has_none() {
    let scenario = parse_with("").expect("loads");
    assert!(scenario.world.objectives().is_none());
    assert!(scenario.world.result().is_none());
    // A copy for another seed keeps them.
    let with = parse_with(&objectives(LEVELS, ALL_KINDS)).expect("loads");
    let other = with.with_seed(9).expect("copies");
    assert_eq!(other.world.objectives(), with.world.objectives());
}

// -----------------------------------------------------------------
// Decision 0088: the horizon and ships out of action.
// -----------------------------------------------------------------

const OUT_OF_ACTION: &str = r#"
[[objectives.allied.objective]]
kind = "ships_out_of_action"
points = { carrier = 5, destroyer = "0.5" }
"#;

fn horizon_of(levels: &str, allied: &str) -> Result<u32, String> {
    parse_with(&objectives(levels, allied))
        .map(|scenario| {
            scenario
                .world
                .objectives()
                .expect("objectives")
                .horizon_days
        })
        .map_err(|e| e.to_string())
}

#[test]
fn the_horizon_is_the_scenarios_days_unless_a_longer_one_is_stated() {
    // HEAD has two days.
    assert_eq!(horizon_of(LEVELS, OUT_OF_ACTION), Ok(2));
    assert_eq!(horizon_of(LEVELS, ""), Ok(2));
    let stated = |value: &str| format!("{LEVELS}\nhorizon_days = {value}");
    assert_eq!(horizon_of(&stated("2"), OUT_OF_ACTION), Ok(2));
    assert_eq!(horizon_of(&stated("60"), OUT_OF_ACTION), Ok(60));
    // A horizon may be stated where nothing uses it.
    assert_eq!(horizon_of(&stated("60"), ""), Ok(60));
    let scenario = parse_with(&objectives(&stated("60"), OUT_OF_ACTION)).expect("loads");
    match &scenario
        .world
        .objectives()
        .expect("objectives")
        .allied
        .objectives[0]
    {
        Objective::ShipsOutOfAction(points) => {
            assert_eq!(points.of(ShipKind::Carrier), Fixed::from_int(5));
            assert_eq!(points.of(ShipKind::Destroyer), Fixed::from_ratio(1, 2));
            assert_eq!(points.of(ShipKind::Cruiser), Fixed::ZERO);
        }
        other => panic!("{other:?}"),
    }
    // A copy for another seed keeps the horizon.
    let other = scenario.with_seed(9).expect("copies");
    assert_eq!(
        other.world.objectives().expect("objectives").horizon_days,
        60
    );
}

#[test]
fn a_horizon_that_is_zero_short_or_not_a_whole_number_is_refused() {
    let stated = |value: &str| format!("{LEVELS}\nhorizon_days = {value}");
    for (value, wanted) in [
        ("0", "horizon_days 0 is not a whole number of at least 1"),
        ("-4", "horizon_days -4 is not a whole number of at least 1"),
        ("1", "horizon_days 1 is shorter than the scenario's 2 days"),
    ] {
        let message = error_of(&objectives(&stated(value), OUT_OF_ACTION));
        assert!(message.contains(wanted), "{value}: {message}");
    }
    for value in ["2.5", "\"60\""] {
        let message = error_of(&objectives(&stated(value), OUT_OF_ACTION));
        assert!(message.contains("horizon_days"), "{value}: {message}");
    }
}

#[test]
fn a_scenario_counts_damaged_ships_one_way_out_of_action_or_crippled() {
    let crippled =
        "\n[[objectives.allied.objective]]\nkind = \"ships_crippled\"\npoints = { carrier = 5 }\n";
    // On the same side, in either order.
    for both in [
        format!("{OUT_OF_ACTION}{crippled}"),
        format!("{crippled}{OUT_OF_ACTION}"),
    ] {
        let message = error_of(&objectives(LEVELS, &both));
        assert!(
            message.contains("counts damaged ships one way"),
            "{message}"
        );
    }
    // On different sides, in either order.
    let split = |allied: &str, japanese_kind: &str| {
        format!(
                "\n[objectives]\n{LEVELS}\n\n[objectives.allied]\nmission = \"A.\"\n{allied}\n[objectives.japanese]\nmission = \"B.\"\n\n[[objectives.japanese.objective]]\nkind = \"{japanese_kind}\"\npoints = {{ carrier = 5 }}\n"
            )
    };
    for text in [
        split(OUT_OF_ACTION, "ships_crippled"),
        split(crippled, "ships_out_of_action"),
    ] {
        let message = error_of(&text);
        assert!(
            message.contains("counts damaged ships one way"),
            "{message}"
        );
    }
    // Crippled alone, and out of action on both sides, are fine.
    assert!(parse_with(&objectives(LEVELS, crippled)).is_ok());
    assert!(parse_with(&split(OUT_OF_ACTION, "ships_out_of_action")).is_ok());
}

#[test]
fn ships_out_of_action_needs_the_surface_combat_figures() {
    // The surface combat file is a required key, so a scenario without
    // it is refused whatever its objectives; the check in
    // `build_objectives` is the second line of that.
    let text = format!("{HEAD}{}", objectives(LEVELS, OUT_OF_ACTION)).replace(
        "surface_combat = \"../reference/surface_combat.toml\"\n",
        "",
    );
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../data/scenarios/test.toml");
    let message = Scenario::parse(&text, &path)
        .expect_err("refused")
        .to_string();
    assert!(message.contains("surface_combat"), "{message}");
    let spec: ObjectivesSpec = toml::from_str(
            "minor = 3\nmajor = 8\ndecisive = 16\n[allied]\nmission = \"A.\"\n[[allied.objective]]\nkind = \"ships_out_of_action\"\npoints = { carrier = 5 }\n[japanese]\nmission = \"B.\"\n",
        )
        .expect("parses");
    let message = build_objectives(&spec, 2, false).expect_err("refused");
    assert!(
        message.contains("needs the surface combat figures"),
        "{message}"
    );
    assert!(build_objectives(&spec, 2, true).is_ok());
}

#[test]
fn objectives_are_checked_with_clear_errors() {
    let bad_levels = [
        "minor = 0\nmajor = 8\ndecisive = 16",
        "minor = 3\nmajor = 3\ndecisive = 16",
        "minor = 3\nmajor = 8\ndecisive = 8",
        "minor = 9\nmajor = 8\ndecisive = 16",
        "minor = -3\nmajor = 8\ndecisive = 16",
    ];
    for levels in bad_levels {
        let message = error_of(&objectives(levels, ""));
        assert!(
            message.contains("levels must be above zero and rising"),
            "{levels}: {message}"
        );
    }
    // A float level or points are rejected, as everywhere.
    let message = error_of(&objectives("minor = 0.5\nmajor = 8\ndecisive = 16", ""));
    assert!(message.contains("minor"), "{message}");
    let message = error_of(&objectives(
        LEVELS,
        "[[objectives.allied.objective]]\nkind = \"aircraft_destroyed\"\npoints = 0.05\n",
    ));
    assert!(!message.is_empty());
    let message = error_of(&objectives(
        LEVELS,
        "[[objectives.allied.objective]]\nkind = \"ships_sunk\"\npoints = { carrier = 0.5 }\n",
    ));
    assert!(!message.is_empty());
    // Negative points.
    let message = error_of(&objectives(
        LEVELS,
        "[[objectives.allied.objective]]\nkind = \"aircraft_destroyed\"\npoints = -1\n",
    ));
    assert!(
        message.contains("allied objective 1 points is negative"),
        "{message}"
    );
    let message = error_of(&objectives(
        LEVELS,
        "[[objectives.allied.objective]]\nkind = \"ships_sunk\"\npoints = { carrier = -10 }\n",
    ));
    assert!(message.contains("carrier is negative"), "{message}");
    let message = error_of(&objectives(
            LEVELS,
            "[[objectives.allied.objective]]\nkind = \"cargo_landed\"\nbase = 1\npoints_per_t = \"-0.1\"\n",
        ));
    assert!(message.contains("points_per_t is negative"), "{message}");
    // A named base that does not exist.
    for kind in [
        "cargo_landed\"\npoints_per_t = 1\nbase = 77\n#",
        "base_open\"\npoints = 1\nbase = 77\n#",
    ] {
        let message = error_of(&objectives(
            LEVELS,
            &format!("[[objectives.allied.objective]]\nkind = \"{kind}\n"),
        ));
        assert!(message.contains("unknown base:77"), "{message}");
    }
    // An unknown kind of objective, of ship, or field.
    let message = error_of(&objectives(
        LEVELS,
        "[[objectives.allied.objective]]\nkind = \"islands_taken\"\npoints = 1\n",
    ));
    assert!(message.contains("islands_taken"), "{message}");
    let message = error_of(&objectives(
        LEVELS,
        "[[objectives.allied.objective]]\nkind = \"ships_sunk\"\npoints = { dreadnought = 1 }\n",
    ));
    assert!(
        message.contains("unknown ship kind \"dreadnought\""),
        "{message}"
    );
    let message = error_of(&objectives(
        LEVELS,
        "[[objectives.allied.objective]]\nkind = \"aircraft_destroyed\"\npoints = 1\nbase = 1\n",
    ));
    assert!(message.contains("base"), "{message}");
    // An empty mission, a side missing.
    let message = error_of(&objectives(LEVELS, "").replace("Take base one.", "  "));
    assert!(message.contains("japanese mission is empty"), "{message}");
    let message = error_of(
            "\n[objectives]\nminor = 3\nmajor = 8\ndecisive = 16\n\n[objectives.allied]\nmission = \"A.\"\n",
        );
    assert!(message.contains("japanese"), "{message}");
}
