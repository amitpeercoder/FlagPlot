use super::*;

/// Decision 0156: an order may carry the pulse it is given for inside a
/// day only where the scenario turns orders inside a day on, and only
/// the morning (0600) or the afternoon (1200). The key is off when
/// absent, and an order with no pulse is an order of dusk.
#[test]
fn an_order_inside_a_day_needs_the_key_and_the_morning_or_the_afternoon() {
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../data/scenarios/test.toml");
    let keyed = |on: &str, extra: &str| {
        let head = HEAD.replacen("days = 2\n", &format!("days = 2\n{on}"), 1);
        Scenario::parse(&format!("{head}{extra}"), &path)
    };
    let order = |pulse: &str| {
        format!(
                "\n[[order]]\nday = 1\n{pulse}side = \"allied\"\ntask_force = 1\nkind = \"set_speed\"\nspeed = \"full\"\n"
            )
    };
    // Off when absent, and an order of dusk has no pulse.
    let plain = parse_with(&order("")).expect("loads");
    assert!(!plain.orders_inside_a_day());
    assert_eq!(plain.scripted_orders()[&1].len(), 1);
    assert!(plain.scripted_orders_inside_day().is_empty());
    // On, with an order at 0600 and one at 1200.
    let on = keyed(
        "orders_inside_a_day = true\n",
        &format!(
            "{}{}{}",
            order(""),
            order("pulse = \"afternoon\"\n"),
            order("pulse = \"morning\"\n")
        ),
    )
    .expect("loads");
    assert!(on.orders_inside_a_day());
    assert_eq!(on.scripted_orders()[&1].len(), 1);
    let pulses: Vec<Pulse> = on.scripted_orders_inside_day()[&1]
        .iter()
        .map(|(pulse, side, _)| {
            assert_eq!(*side, Side::Allied);
            *pulse
        })
        .collect();
    assert_eq!(pulses, vec![Pulse::Afternoon, Pulse::Morning]);
    // A pulse without the key, or with it off.
    for off in ["", "orders_inside_a_day = false\n"] {
        let message = keyed(off, &order("pulse = \"morning\"\n"))
            .expect_err("refused")
            .message;
        assert!(
            message.contains("orders inside a day need orders_inside_a_day = true"),
            "{message}"
        );
    }
    // Any pulse but the morning or the afternoon.
    for bad in ["night", "dusk", "Morning"] {
        let message = keyed(
            "orders_inside_a_day = true\n",
            &order(&format!("pulse = \"{bad}\"\n")),
        )
        .expect_err("refused")
        .message;
        assert!(
            message.contains(&format!("has pulse \"{bad}\""))
                && message.contains("\"morning\" (at 0600) or the \"afternoon\" (at 1200)"),
            "{message}"
        );
    }
}

fn order(target: &str, body: &str) -> String {
    format!(
        "{}{}[[order]]\nday = 1\nside = \"allied\"\n{target}\n{body}\n",
        air_group(1, 1),
        task_force("at_base = 1")
    )
}

#[test]
fn orders_are_addressed_to_one_kind_of_unit() {
    let arc = "kind = \"set_search_arc\"\nfrom_deg = 60\nto_deg = 120\nrange_nm = 600";
    let mission = "kind = \"set_air_mission\"\nmission = \"search\"";
    parse_with(&order("air_group = 1", arc)).expect("arc loads");
    parse_with(&order("air_group = 1", mission)).expect("mission loads");
    parse_with(&order("task_force = 1", "kind = \"load\"")).expect("load loads");
    assert!(error_of(&order("task_force = 1", arc)).contains("must be addressed to a"));
    assert!(error_of(&order("task_force = 1", mission)).contains("air_group"));
    assert!(error_of(&order("air_group = 1", "kind = \"load\"")).contains("air_group"));
    assert!(
        error_of(&order("task_force = 1\nair_group = 1", "kind = \"load\""))
            .contains("exactly one of task_force, air_group, land_unit and base")
    );
    assert!(error_of(&order("", "kind = \"load\"")).contains("exactly one"));
    let float = arc.replace("range_nm = 600", "range_nm = 600.5");
    assert!(parse_with(&order("air_group = 1", &float)).is_err());
}

#[test]
fn the_naval_strike_mission_loads_for_an_air_group() {
    let mission = "kind = \"set_air_mission\"\nmission = \"naval_strike\"";
    parse_with(&order("air_group = 1", mission)).expect("naval_strike loads");
    let typo = mission.replace("naval_strike", "naval strike");
    assert!(parse_with(&order("air_group = 1", &typo)).is_err());
}

#[test]
fn the_combat_air_patrol_mission_and_the_patrol_share_load_for_an_air_group() {
    let mission = "kind = \"set_air_mission\"\nmission = \"combat_air_patrol\"";
    parse_with(&order("air_group = 1", mission)).expect("combat_air_patrol loads");
    let typo = mission.replace("combat_air_patrol", "combat air patrol");
    assert!(parse_with(&order("air_group = 1", &typo)).is_err());

    let share = "kind = \"set_patrol_share\"\nshare = \"0.5\"";
    parse_with(&order("air_group = 1", share)).expect("share loads");
    // A share is a decimal string, never a float, and has a field.
    let float = share.replace("\"0.5\"", "0.5");
    assert!(parse_with(&order("air_group = 1", &float)).is_err());
    assert!(parse_with(&order("air_group = 1", "kind = \"set_patrol_share\"")).is_err());
    // It is for an air group, not a task force.
    assert!(error_of(&order("task_force = 1", share)).contains("air_group"));
    // The value is checked when the order reaches intake, not here: a
    // share above 1 loads.
    parse_with(&order(
        "air_group = 1",
        "kind = \"set_patrol_share\"\nshare = \"1.5\"",
    ))
    .expect("a share out of range is rejected by the world, not the loader");
}

#[test]
fn the_base_strike_mission_needs_its_target_base_and_no_other_mission_has_one() {
    let strike =
        |target: &str| format!("kind = \"set_air_mission\"\nmission = \"base_strike\"\n{target}");
    // A base of the other side loads, and the order is addressed to a
    // group.
    parse_with(&order("air_group = 1", &strike("target_base = 2"))).expect("loads");
    let message = error_of(&order("air_group = 1", &strike("")));
    assert!(message.contains("needs a target_base"), "{message}");
    let message = error_of(&order("air_group = 1", &strike("target_base = 9")));
    assert!(message.contains("unknown base 9"), "{message}");
    let message = error_of(&order("air_group = 1", &strike("target_base = 1")));
    assert!(message.contains("own side"), "{message}");
    // A later day's order on the same base loads: the base may have
    // changed side by then, and intake checks it (decision 0101, as built).
    parse_with(
        &order("air_group = 1", &strike("target_base = 1")).replace("day = 1\n", "day = 2\n"),
    )
    .expect("a later day's strike on a base of the side at the start loads");
    let message = error_of(&order(
        "air_group = 1",
        "kind = \"set_air_mission\"\nmission = \"naval_strike\"\ntarget_base = 2",
    ));
    assert!(message.contains("only for the base_strike"), "{message}");
    // A target is a base number, not a float.
    assert!(parse_with(&order("air_group = 1", &strike("target_base = 2.5"))).is_err());
    // The order is for an air group.
    assert!(error_of(&order("task_force = 1", &strike("target_base = 2"))).contains("air_group"));
}

#[test]
fn a_base_strike_order_sets_the_mission_with_its_target() {
    let mut scenario = with_air_combat(&format!(
            "{}[[order]]\nday = 1\nside = \"allied\"\nair_group = 1\nkind = \"set_air_mission\"\nmission = \"base_strike\"\ntarget_base = 2\n",
            air_group(1, 3)
        ))
        .expect("loads");
    scenario.next_day().expect("day runs");
    assert_eq!(
        scenario.world.umpire().air_groups[&AirGroupId::new(1)].mission,
        AirMission::BaseStrike(BaseId::new(2))
    );
}

#[test]
fn a_send_to_yard_order_takes_no_other_key() {
    let orders = loaded_orders("kind = \"send_to_yard\"").expect("loads");
    assert_eq!(orders, vec![TaskForceOrder::SendToYard]);
    assert!(loaded_orders("kind = \"send_to_yard\"\nbase = 1").is_err());
    assert!(error_of(&order("air_group = 1", "kind = \"send_to_yard\"")).contains("air_group"));
}

#[test]
fn the_rest_threshold_order_loads_for_an_air_group_and_is_checked_at_intake() {
    let rest = "kind = \"set_rest_threshold\"\nfatigue = 30";
    parse_with(&order("air_group = 1", rest)).expect("a whole number loads");
    parse_with(&order(
        "air_group = 1",
        "kind = \"set_rest_threshold\"\nfatigue = \"12.5\"",
    ))
    .expect("a decimal string loads");
    // Never a float, always a value, and for an air group.
    let float = rest.replace("= 30", "= 30.5");
    assert!(parse_with(&order("air_group = 1", &float)).is_err());
    assert!(parse_with(&order("air_group = 1", "kind = \"set_rest_threshold\"")).is_err());
    assert!(error_of(&order("task_force = 1", rest)).contains("air_group"));
    // The range is checked when the order reaches intake, not here.
    parse_with(&order(
        "air_group = 1",
        "kind = \"set_rest_threshold\"\nfatigue = 150",
    ))
    .expect("a threshold out of range is rejected by the world, not the loader");
    // An unknown key is rejected.
    assert!(parse_with(&order("air_group = 1", &format!("{rest}\nextra = 1"))).is_err());
}

#[test]
fn the_reach_order_loads_for_an_air_group_with_a_name_and_nothing_else() {
    let reach = |name: &str| format!("kind = \"set_reach\"\nreach = \"{name}\"");
    parse_with(&order("air_group = 1", &reach("normal"))).expect("normal loads");
    parse_with(&order("air_group = 1", &reach("extended"))).expect("extended loads");
    assert!(parse_with(&order("air_group = 1", &reach("long"))).is_err());
    assert!(parse_with(&order("air_group = 1", "kind = \"set_reach\"")).is_err());
    assert!(error_of(&order("task_force = 1", &reach("extended"))).contains("air_group"));
    // An unknown key is rejected.
    assert!(parse_with(&order(
        "air_group = 1",
        &format!("{}\nextra = 1", reach("extended"))
    ))
    .is_err());
}

#[test]
fn the_target_priority_order_loads_for_an_air_group_with_kind_names_and_nothing_else() {
    let list = |kinds: &str| format!("kind = \"set_target_priority\"\nkinds = {kinds}");
    parse_with(&order(
        "air_group = 1",
        &list("[\"transport\", \"cruiser\"]"),
    ))
    .expect("a list of kinds loads");
    // An empty list or a repeated kind is the world's to refuse, at
    // intake.
    parse_with(&order("air_group = 1", &list("[]"))).expect("an empty list loads");
    parse_with(&order("air_group = 1", &list("[\"cruiser\", \"cruiser\"]")))
        .expect("a repeated kind loads");
    assert!(error_of(&order("air_group = 1", &list("[\"frigate\"]"))).contains("frigate"));
    assert!(parse_with(&order("air_group = 1", "kind = \"set_target_priority\"")).is_err());
    assert!(error_of(&order("task_force = 1", &list("[\"cruiser\"]"))).contains("air_group"));
    assert!(parse_with(&order(
        "air_group = 1",
        &format!("{}\nextra = 1", list("[\"cruiser\"]"))
    ))
    .is_err());
}

#[test]
fn the_strike_setting_orders_load_for_an_air_group_and_are_checked_at_intake() {
    let age = |pulses: &str| format!("kind = \"set_strike_contact_age\"\npulses = {pulses}");
    let share = |share: &str| format!("kind = \"set_strike_held_share\"\nshare = {share}");
    parse_with(&order("air_group = 1", &age("1"))).expect("an age loads");
    // The range is the world's to refuse, at intake.
    parse_with(&order("air_group = 1", &age("99"))).expect("too old loads");
    parse_with(&order("air_group = 1", &age("-1"))).expect("negative loads");
    assert!(parse_with(&order("air_group = 1", &age("1.5"))).is_err());
    assert!(parse_with(&order("air_group = 1", "kind = \"set_strike_contact_age\"")).is_err());
    assert!(error_of(&order("task_force = 1", &age("1"))).contains("air_group"));
    assert!(parse_with(&order("air_group = 1", &format!("{}\nextra = 1", age("1")))).is_err());
    parse_with(&order("air_group = 1", &share("\"0.25\""))).expect("a share loads");
    parse_with(&order("air_group = 1", &share("\"1.5\""))).expect("a share out of range loads");
    assert!(parse_with(&order("air_group = 1", &share("0.25"))).is_err());
    assert!(parse_with(&order("air_group = 1", "kind = \"set_strike_held_share\"")).is_err());
    assert!(error_of(&order("task_force = 1", &share("\"0.25\""))).contains("air_group"));
    assert!(parse_with(&order(
        "air_group = 1",
        &format!("{}\nextra = 1", share("\"0.25\""))
    ))
    .is_err());
}

#[test]
fn the_strike_sizing_order_loads_for_an_air_group() {
    let sizing = |on: &str| format!("kind = \"set_strike_sizing\"\non = {on}");
    parse_with(&order("air_group = 1", &sizing("true"))).expect("on loads");
    parse_with(&order("air_group = 1", &sizing("false"))).expect("off loads");
    assert!(parse_with(&order("air_group = 1", "kind = \"set_strike_sizing\"")).is_err());
    assert!(parse_with(&order("air_group = 1", &sizing("1"))).is_err());
    assert!(error_of(&order("task_force = 1", &sizing("true"))).contains("air_group"));
}

#[test]
fn the_search_once_order_loads_for_an_air_group() {
    let once = |on: &str| format!("kind = \"set_search_once\"\non = {on}");
    parse_with(&order("air_group = 1", &once("true"))).expect("on loads");
    parse_with(&order("air_group = 1", &once("false"))).expect("off loads");
    assert!(parse_with(&order("air_group = 1", "kind = \"set_search_once\"")).is_err());
    assert!(parse_with(&order("air_group = 1", &once("1"))).is_err());
    assert!(error_of(&order("task_force = 1", &once("true"))).contains("air_group"));
}

#[test]
fn the_strike_first_waters_order_loads_for_an_air_group() {
    let waters = |hexes: &str| format!("kind = \"set_strike_first_waters\"\nhexes = {hexes}");
    parse_with(&order("air_group = 1", &waters("[[0, 0], [1, -1]]"))).expect("hexes load");
    parse_with(&order("air_group = 1", &waters("[]"))).expect("an empty list loads");
    assert!(parse_with(&order(
        "air_group = 1",
        "kind = \"set_strike_first_waters\""
    ))
    .is_err());
    assert!(error_of(&order("task_force = 1", &waters("[[0, 0]]"))).contains("air_group"));
}

#[test]
fn a_misspelled_order_field_is_an_error() {
    let typo =
        "kind = \"set_search_arc\"\nfrom_deg = 60\nto_deg = 120\nrange_nm = 600\nrange_nmm = 5";
    assert!(parse_with(&order("air_group = 1", typo)).is_err());
    let typo = "kind = \"set_speed\"\nspeed = \"full\"\nspeeed = 5";
    assert!(parse_with(&order("task_force = 1", typo)).is_err());
    assert!(parse_with(&order("task_force = 1", "kind = \"load\"\nspeeed = 5")).is_err());
    let typo = "kind = \"load\"\nintnet = 1";
    assert!(parse_with(&order("task_force = 1", typo)).is_err());
}

#[test]
fn a_set_speed_order_gives_a_named_speed_or_knots_but_not_both_or_neither() {
    let speed_of = |body: &str| {
        let scenario = parse_with(&order("task_force = 1", body)).expect("loads");
        let (_, request) = &scenario.script[&1][0];
        match &request.kind {
            OrderKind::TaskForce {
                order: TaskForceOrder::SetSpeed(speed),
                ..
            } => *speed,
            other => panic!("not a speed order: {other:?}"),
        }
    };
    assert_eq!(
        speed_of("kind = \"set_speed\"\nspeed = \"full\""),
        SpeedSetting::Full
    );
    assert_eq!(
        speed_of("kind = \"set_speed\"\nknots = \"24\""),
        SpeedSetting::Knots(Fixed::from_int(24))
    );
    assert_eq!(
        speed_of("kind = \"set_speed\"\nknots = \"22.5\""),
        SpeedSetting::Knots(Fixed::from_ratio(45, 2))
    );
    let both = "kind = \"set_speed\"\nspeed = \"full\"\nknots = \"24\"";
    let message = error_of(&order("task_force = 1", both));
    assert!(
        message.contains("exactly one of speed and knots"),
        "{message}"
    );
    let message = error_of(&order("task_force = 1", "kind = \"set_speed\""));
    assert!(
        message.contains("exactly one of speed and knots"),
        "{message}"
    );
    let message = error_of(&order(
        "task_force = 1",
        "kind = \"set_speed\"\nknots = \"fast\"",
    ));
    assert!(message.contains("knots on day 1"), "{message}");
}

#[test]
fn a_readiness_order_loads_for_a_task_force_only() {
    let scenario = parse_with(&order(
        "task_force = 1",
        "kind = \"set_readiness\"\nreadiness = \"battle\"",
    ))
    .expect("loads");
    let (_, request) = &scenario.script[&1][0];
    assert_eq!(
        request.kind,
        OrderKind::TaskForce {
            target: TaskForceId::new(1),
            order: TaskForceOrder::SetReadiness(Readiness::Battle),
        }
    );
    let scenario = parse_with(&order(
        "task_force = 1",
        "kind = \"set_readiness\"\nreadiness = \"normal\"",
    ))
    .expect("loads");
    assert!(matches!(
        &scenario.script[&1][0].1.kind,
        OrderKind::TaskForce {
            order: TaskForceOrder::SetReadiness(Readiness::Normal),
            ..
        }
    ));
    assert!(error_of(&order(
        "air_group = 1",
        "kind = \"set_readiness\"\nreadiness = \"battle\""
    ))
    .contains("air_group"));
    assert!(parse_with(&order("task_force = 1", "kind = \"set_readiness\"")).is_err());
    assert!(parse_with(&order(
        "task_force = 1",
        "kind = \"set_readiness\"\nreadiness = \"ready\""
    ))
    .is_err());
    assert!(parse_with(&order(
        "task_force = 1",
        "kind = \"set_readiness\"\nreadiness = \"battle\"\nextra = 1"
    ))
    .is_err());
}

fn roe(body: &str) -> String {
    order(
        "task_force = 1",
        &format!("kind = \"set_rules_of_engagement\"\n{body}"),
    )
}

#[test]
fn a_rules_of_engagement_order_loads() {
    let scenario = parse_with(&roe("stance = \"avoid\"\nbreak_off = \"0.25\"")).expect("loads");
    let (_, request) = &scenario.script[&1][0];
    assert_eq!(
        request.kind,
        OrderKind::TaskForce {
            target: TaskForceId::new(1),
            order: TaskForceOrder::SetRulesOfEngagement {
                stance: Stance::Avoid,
                break_off: Fixed::from_ratio(1, 4),
                stragglers: StragglerInstruction::Keep,
                keep_share: Fixed::ZERO,
            },
        }
    );
    let scenario = parse_with(&roe("stance = \"engage\"\nbreak_off = \"1\"")).expect("loads");
    let (_, request) = &scenario.script[&1][0];
    assert_eq!(
        request.kind,
        OrderKind::TaskForce {
            target: TaskForceId::new(1),
            order: TaskForceOrder::SetRulesOfEngagement {
                stance: Stance::Engage,
                break_off: Fixed::ONE,
                stragglers: StragglerInstruction::Keep,
                keep_share: Fixed::ZERO,
            },
        }
    );
    // A whole number works as for any quantity.
    parse_with(&roe("stance = \"engage\"\nbreak_off = 1")).expect("loads");
}

#[test]
fn a_rules_of_engagement_order_loads_stragglers_and_keep_share() {
    for (text, wanted) in [
        ("keep", StragglerInstruction::Keep),
        ("detach", StragglerInstruction::Detach),
        ("leave", StragglerInstruction::Leave),
        ("retire", StragglerInstruction::Retire),
    ] {
        let scenario = parse_with(&roe(&format!(
                "stance = \"engage\"\nbreak_off = \"0.5\"\nstragglers = \"{text}\"\nkeep_share = \"0.5\""
            )))
            .expect("loads");
        let (_, request) = &scenario.script[&1][0];
        assert_eq!(
            request.kind,
            OrderKind::TaskForce {
                target: TaskForceId::new(1),
                order: TaskForceOrder::SetRulesOfEngagement {
                    stance: Stance::Engage,
                    break_off: Fixed::from_ratio(1, 2),
                    stragglers: wanted,
                    keep_share: Fixed::from_ratio(1, 2),
                },
            }
        );
    }
    // A bad name, a float and a misspelled field are errors.
    assert!(parse_with(&roe(
        "stance = \"engage\"\nbreak_off = \"0.5\"\nstragglers = \"abandon\""
    ))
    .is_err());
    assert!(parse_with(&roe(
        "stance = \"engage\"\nbreak_off = \"0.5\"\nkeep_share = 0.5"
    ))
    .is_err());
    let message = error_of(&roe(
        "stance = \"engage\"\nbreak_off = \"0.5\"\nkeep_share = \"most\"",
    ));
    assert!(message.contains("keep_share on day 1"), "{message}");
    assert!(parse_with(&roe(
        "stance = \"engage\"\nbreak_off = \"0.5\"\nstraggler = \"detach\""
    ))
    .is_err());
    assert!(parse_with(&roe(
        "stance = \"engage\"\nbreak_off = \"0.5\"\nkeepshare = \"0.5\""
    ))
    .is_err());
}

#[test]
fn a_home_port_order_loads_for_a_task_force_only() {
    let scenario = parse_with(&order(
        "task_force = 1",
        "kind = \"set_home_port\"\nbase = 2",
    ))
    .expect("loads");
    let (_, request) = &scenario.script[&1][0];
    assert_eq!(
        request.kind,
        OrderKind::TaskForce {
            target: TaskForceId::new(1),
            order: TaskForceOrder::SetHomePort(BaseId::new(2)),
        }
    );
    assert!(error_of(&order(
        "air_group = 1",
        "kind = \"set_home_port\"\nbase = 1"
    ))
    .contains("air_group"));
    assert!(parse_with(&order(
        "task_force = 1",
        "kind = \"set_home_port\"\nbase = 1.5"
    ))
    .is_err());
    assert!(parse_with(&order("task_force = 1", "kind = \"set_home_port\"")).is_err());
    assert!(parse_with(&order(
        "task_force = 1",
        "kind = \"set_home_port\"\nbse = 1"
    ))
    .is_err());
    assert!(parse_with(&order(
        "task_force = 1",
        "kind = \"set_home_port\"\nbase = 1\nextra = 1"
    ))
    .is_err());
}

#[test]
fn a_detach_order_loads_for_a_task_force_only() {
    let scenario = parse_with(&order(
        "task_force = 1",
        "kind = \"detach\"\nships = [3, 1, 3]",
    ))
    .expect("loads");
    let (_, request) = &scenario.script[&1][0];
    assert_eq!(
        request.kind,
        OrderKind::TaskForce {
            target: TaskForceId::new(1),
            order: TaskForceOrder::DetachShips(vec![
                ShipId::new(3),
                ShipId::new(1),
                ShipId::new(3)
            ]),
        }
    );
    // An empty list loads: intake rejects it, as a rule of the game.
    parse_with(&order("task_force = 1", "kind = \"detach\"\nships = []")).expect("loads");
    assert!(
        error_of(&order("air_group = 1", "kind = \"detach\"\nships = [1]")).contains("air_group")
    );
    assert!(parse_with(&order("task_force = 1", "kind = \"detach\"")).is_err());
    assert!(parse_with(&order("task_force = 1", "kind = \"detach\"\nship = [1]")).is_err());
    assert!(parse_with(&order("task_force = 1", "kind = \"detach\"\nships = [1.5]")).is_err());
}

#[test]
fn rules_of_engagement_order_errors_are_rejected() {
    // A float, text that is no number, an unknown stance, a missing or
    // misspelled field.
    assert!(parse_with(&roe("stance = \"engage\"\nbreak_off = 0.5")).is_err());
    assert!(parse_with(&roe("stance = \"engage\"\nbreak_off = \"half\"")).is_err());
    let message = error_of(&roe("stance = \"engage\"\nbreak_off = \"half\""));
    assert!(message.contains("break_off on day 1"), "{message}");
    assert!(parse_with(&roe("stance = \"charge\"\nbreak_off = \"0.5\"")).is_err());
    assert!(parse_with(&roe("stance = \"engage\"")).is_err());
    assert!(parse_with(&roe("break_off = \"0.5\"")).is_err());
    assert!(parse_with(&roe(
        "stance = \"engage\"\nbreak_off = \"0.5\"\nbreak_offf = 1"
    ))
    .is_err());
    // It is addressed to a task force, not an air group.
    let to_group = order(
        "air_group = 1",
        "kind = \"set_rules_of_engagement\"\nstance = \"engage\"\nbreak_off = \"0.5\"",
    );
    assert!(error_of(&to_group).contains("air_group"));
}

/// The task force orders scripted for day 1 of a scenario with one load
/// order whose body is `body`.
fn loaded_orders(body: &str) -> Result<Vec<TaskForceOrder>, DataError> {
    let scenario = parse_with(&order("task_force = 1", body))?;
    Ok(scenario.scripted_orders()[&1]
        .iter()
        .filter_map(|(_, request)| match &request.kind {
            OrderKind::TaskForce { order, .. } => Some(order.clone()),
            _ => None,
        })
        .collect())
}

#[test]
fn a_load_order_names_its_cargo_and_defaults_to_supply() {
    assert_eq!(
        loaded_orders("kind = \"load\"").expect("loads"),
        vec![TaskForceOrder::Load]
    );
    assert_eq!(
        loaded_orders("kind = \"load\"\ncargo = \"supply\"").expect("loads"),
        vec![TaskForceOrder::Load]
    );
    assert_eq!(
        loaded_orders("kind = \"load\"\ncargo = \"fuel\"").expect("loads"),
        vec![TaskForceOrder::LoadFuel]
    );
    for bad in ["\"oil\"", "\"Fuel\"", "1", "\"\""] {
        assert!(
            loaded_orders(&format!("kind = \"load\"\ncargo = {bad}")).is_err(),
            "cargo = {bad} should not load"
        );
    }
    // An unload names no cargo.
    assert!(loaded_orders("kind = \"unload\"\ncargo = \"fuel\"").is_err());
}

fn load_orders(extra: &str) -> Result<Vec<TaskForceOrder>, DataError> {
    let orders: String = extra
            .split('|')
            .map(|body| {
                format!("\n[[order]]\nday = 1\nside = \"allied\"\ntask_force = 2\nkind = \"load\"\n{body}\n")
            })
            .collect();
    let scenario = landing(true, &orders)?;
    Ok(scenario.scripted_orders()[&1]
        .iter()
        .map(|(_, request)| match &request.kind {
            OrderKind::TaskForce { order, .. } => order.clone(),
            other => panic!("not a task force order: {other:?}"),
        })
        .collect())
}

#[test]
fn a_load_order_may_name_a_unit_and_a_cargo_or_none() {
    let orders = load_orders(
            "|cargo = \"supply\"|cargo = \"fuel\"|unit = 3|unit = 3\ncargo = \"supply\"|unit = 3\ncargo = \"fuel\"|unit = 3\ncargo = \"none\"",
        )
        .expect("loads");
    let unit = LandUnitId::new(3);
    assert_eq!(
        orders,
        vec![
            TaskForceOrder::Load,
            TaskForceOrder::Load,
            TaskForceOrder::LoadFuel,
            TaskForceOrder::LoadUnit {
                unit,
                cargo: Some(Cargo::Supply)
            },
            TaskForceOrder::LoadUnit {
                unit,
                cargo: Some(Cargo::Supply)
            },
            TaskForceOrder::LoadUnit {
                unit,
                cargo: Some(Cargo::Fuel)
            },
            TaskForceOrder::LoadUnit { unit, cargo: None },
        ]
    );
}

#[test]
fn a_load_order_with_no_unit_cannot_name_no_cargo() {
    let error = load_orders("cargo = \"none\"").expect_err("refused");
    let message = error.to_string();
    assert!(
        message.contains("a load with no unit must name supply or fuel"),
        "{message}"
    );
    assert!(load_orders("cargo = \"gold\"").is_err());
}

#[test]
fn a_load_order_may_name_a_weight_for_supply_or_fuel() {
    let orders = loaded_orders("kind = \"load\"\nweight_t = 250").expect("loads");
    assert_eq!(
        orders,
        vec![TaskForceOrder::LoadTo {
            cargo: Cargo::Supply,
            weight_t: Fixed::from_int(250),
            each_t: None,
        }]
    );
    assert_eq!(
        loaded_orders("kind = \"load\"\ncargo = \"supply\"\nweight_t = \"2.5\"").expect("loads"),
        vec![TaskForceOrder::LoadTo {
            cargo: Cargo::Supply,
            weight_t: Fixed::from_ratio(5, 2),
            each_t: None,
        }]
    );
    assert_eq!(
        loaded_orders("kind = \"load\"\ncargo = \"fuel\"\nweight_t = 80").expect("loads"),
        vec![TaskForceOrder::LoadTo {
            cargo: Cargo::Fuel,
            weight_t: Fixed::from_int(80),
            each_t: None,
        }]
    );
    // The sign is for intake to refuse, as it is for knots.
    assert_eq!(
        loaded_orders("kind = \"load\"\nweight_t = 0").expect("loads"),
        vec![TaskForceOrder::LoadTo {
            cargo: Cargo::Supply,
            weight_t: Fixed::ZERO,
            each_t: None,
        }]
    );
    assert!(loaded_orders("kind = \"load\"\nweight_t = -5").is_ok());
    // A float is never a quantity; an unload takes no weight.
    assert!(loaded_orders("kind = \"load\"\nweight_t = 2.5").is_err());
    assert!(loaded_orders("kind = \"unload\"\nweight_t = 5").is_err());
}

#[test]
fn a_load_to_a_weight_may_name_a_level_above_0() {
    assert_eq!(
        loaded_orders("kind = \"load\"\nweight_t = 100\neach_t = 40").expect("loads"),
        vec![TaskForceOrder::LoadTo {
            cargo: Cargo::Supply,
            weight_t: Fixed::from_int(100),
            each_t: Some(Fixed::from_int(40)),
        }]
    );
    assert_eq!(
        loaded_orders("kind = \"load\"\ncargo = \"fuel\"\nweight_t = 100\neach_t = \"2.5\"")
            .expect("loads"),
        vec![TaskForceOrder::LoadTo {
            cargo: Cargo::Fuel,
            weight_t: Fixed::from_int(100),
            each_t: Some(Fixed::from_ratio(5, 2)),
        }]
    );
    // Refused with no weight, with a unit, at 0 or below, and as a float
    // (decision 0162, 3).
    for text in [
        "kind = \"load\"\neach_t = 40",
        "kind = \"load\"\nunit = 3\neach_t = 40",
        "kind = \"load\"\nunit = 3\nweight_t = 100\neach_t = 40",
        "kind = \"load\"\nweight_t = 100\neach_t = 0",
        "kind = \"load\"\nweight_t = 100\neach_t = -5",
        "kind = \"load\"\nweight_t = 100\neach_t = 2.5",
        "kind = \"unload\"\neach_t = 5",
    ] {
        assert!(loaded_orders(text).is_err(), "{text}");
    }
}

#[test]
fn a_weight_does_not_apply_to_men_or_to_no_cargo() {
    let message = loaded_orders("kind = \"load\"\nunit = 3\nweight_t = 100")
        .expect_err("refused")
        .to_string();
    assert!(
        message.contains("a weight does not apply to men"),
        "{message}"
    );
    let message = loaded_orders("kind = \"load\"\nunit = 3\ncargo = \"fuel\"\nweight_t = 100")
        .expect_err("refused")
        .to_string();
    assert!(
        message.contains("a weight does not apply to men"),
        "{message}"
    );
    let message = loaded_orders("kind = \"load\"\ncargo = \"none\"\nweight_t = 100")
        .expect_err("refused")
        .to_string();
    assert!(
        message.contains("a weight needs supply or fuel"),
        "{message}"
    );
}

#[test]
fn a_no_cargo_work_order_loads_for_a_task_force_with_no_other_key() {
    assert_eq!(
        loaded_orders("kind = \"no_cargo_work\"").expect("loads"),
        vec![TaskForceOrder::NoCargoWork]
    );
    assert!(loaded_orders("kind = \"no_cargo_work\"\nweight_t = 1").is_err());
    assert!(loaded_orders("kind = \"no_cargo_work\"\ninto = 7").is_err());
    let body = "kind = \"no_cargo_work\"";
    assert!(parse_with(&order("air_group = 1", body)).is_err());
    assert!(parse_with(&order("land_unit = 1", body)).is_err());
    assert!(error_of(&order("base = 1", body)).contains("for a base must be form_task_force"));
}

#[test]
fn join_and_stand_down_orders_load_for_a_task_force() {
    assert_eq!(
        loaded_orders("kind = \"join\"\ninto = 7").expect("loads"),
        vec![TaskForceOrder::JoinTo(TaskForceId::new(7))]
    );
    assert_eq!(
        loaded_orders("kind = \"stand_down\"").expect("loads"),
        vec![TaskForceOrder::StandDown]
    );
    // A join names the force it joins; neither takes another key.
    assert!(loaded_orders("kind = \"join\"").is_err());
    assert!(loaded_orders("kind = \"join\"\ninto = \"7\"").is_err());
    assert!(loaded_orders("kind = \"join\"\ninto = 7\nother = 1").is_err());
    assert!(loaded_orders("kind = \"stand_down\"\ninto = 7").is_err());
    assert!(loaded_orders("kind = \"stand_down\"\nweight_t = 1").is_err());
    // Both are addressed to a task force.
    for body in ["kind = \"join\"\ninto = 7", "kind = \"stand_down\""] {
        assert!(parse_with(&order("air_group = 1", body)).is_err());
        assert!(parse_with(&order("land_unit = 1", body)).is_err());
    }
}

#[test]
fn a_base_order_forms_a_task_force_and_takes_no_other_kind() {
    let form = "kind = \"form_task_force\"\nships = [5, 6]";
    let scenario = parse_with(&order("base = 1", form)).expect("loads");
    let found: Vec<OrderKind> = scenario.scripted_orders()[&1]
        .iter()
        .map(|(_, request)| request.kind.clone())
        .filter(|kind| matches!(kind, OrderKind::Base { .. }))
        .collect();
    assert_eq!(
        found,
        vec![OrderKind::Base {
            target: BaseId::new(1),
            order: BaseOrder::FormTaskForce {
                ships: vec![ShipId::new(5), ShipId::new(6)]
            }
        }]
    );
    // An empty list loads; intake refuses it (NoShipsToForm).
    parse_with(&order("base = 1", "kind = \"form_task_force\"\nships = []")).expect("loads");
    // The ships are a list; the kind is for a base only.
    assert!(parse_with(&order("base = 1", "kind = \"form_task_force\"")).is_err());
    assert!(parse_with(&order("base = 1", "kind = \"form_task_force\"\nships = 5")).is_err());
    for target in ["task_force = 1", "air_group = 1", "land_unit = 1"] {
        assert!(parse_with(&order(target, form)).is_err());
    }
    assert!(error_of(&order("task_force = 1", form)).contains("must be addressed to a base"));
    // A base takes that kind and no other, and is the only target given.
    for body in [
        "kind = \"load\"",
        "kind = \"unload\"",
        "kind = \"join\"\ninto = 2",
        "kind = \"stand_down\"",
        "kind = \"set_posture\"\nposture = \"defend\"",
    ] {
        assert!(error_of(&order("base = 1", body)).contains("for a base must be form_task_force"));
    }
    assert!(error_of(&order("base = 1\ntask_force = 1", form))
        .contains("exactly one of task_force, air_group, land_unit and base"));
}

// -----------------------------------------------------------------
// Marching overland (decision 0094).
// -----------------------------------------------------------------

fn unit_at_base(id: u32) -> String {
    format!(
            "\n[[land_unit]]\nid = {id}\nname = \"Unit {id}\"\nside = \"allied\"\nmen = 100\nq = 0\nr = 0\n"
        )
}

#[test]
fn a_march_order_names_a_land_unit_and_a_hex() {
    let extra = format!(
            "{}\n[[order]]\nday = 2\nside = \"allied\"\nland_unit = 1\nkind = \"march\"\nhex = [3, -2]\n",
            unit_at_base(1)
        );
    let scenario = landing(true, &extra).expect("loads");
    let (side, request) = &scenario.scripted_orders()[&2][0];
    assert_eq!(*side, Side::Allied);
    match &request.kind {
        OrderKind::LandUnit { target, order } => {
            assert_eq!(*target, LandUnitId::new(1));
            assert_eq!(*order, LandUnitOrder::March(Hex::new(3, -2)));
        }
        other => panic!("not a land unit order: {other:?}"),
    }
}

#[test]
fn a_march_order_must_be_for_a_land_unit_and_a_land_unit_takes_only_two_kinds() {
    let order = |target: &str, kind: &str, key: &str| {
        format!(
            "{}\n[[order]]\nday = 1\nside = \"allied\"\n{target}\nkind = \"{kind}\"\n{key}\n",
            unit_at_base(1)
        )
    };
    let message = landing_error(true, &order("task_force = 1", "march", "hex = [1, 0]"));
    assert!(
        message.contains("march") && message.contains("land_unit"),
        "{message}"
    );
    let message = landing_error(
        true,
        &order("land_unit = 1", "set_route", "waypoints = [[1, 0]]"),
    );
    assert!(
        message.contains("set_posture, march or set_activity"),
        "{message}"
    );
    // The hex is two whole numbers.
    let message = landing_error(true, &order("land_unit = 1", "march", "hex = [1]"));
    assert!(!message.is_empty());
}

#[test]
fn a_prepare_landing_order_names_a_force_a_hex_and_a_roster() {
    let order = |roster: &str| {
        format!(
                "{}\n[[order]]\nday = 1\nside = \"allied\"\ntask_force = 1\nkind = \"prepare_landing\"\nhex = [4, 0]\nroster = {roster}\n",
                unit_aboard(1, 100, 1)
            )
    };
    let scenario = landing(true, &order("[1, 3]")).expect("loads");
    let (side, request) = &scenario.scripted_orders()[&1][0];
    assert_eq!(*side, Side::Allied);
    match &request.kind {
        OrderKind::TaskForce { target, order } => {
            assert_eq!(*target, TaskForceId::new(1));
            assert_eq!(
                *order,
                TaskForceOrder::PrepareLanding {
                    hex: Hex::new(4, 0),
                    roster: [LandUnitId::new(1), LandUnitId::new(3)]
                        .into_iter()
                        .collect(),
                }
            );
        }
        other => panic!("not a task force order: {other:?}"),
    }
    // An empty roster is accepted by the loader.
    let scenario = landing(true, &order("[]")).expect("loads");
    match &scenario.scripted_orders()[&1][0].1.kind {
        OrderKind::TaskForce {
            order: TaskForceOrder::PrepareLanding { roster, .. },
            ..
        } => assert!(roster.is_empty()),
        other => panic!("not a prepare order: {other:?}"),
    }
}

#[test]
fn a_prepare_landing_order_is_refused_for_a_land_unit_and_when_malformed() {
    let order = |target: &str, keys: &str| {
        format!(
                "\n[[order]]\nday = 1\nside = \"allied\"\n{target}\nkind = \"prepare_landing\"\n{keys}\n"
            )
    };
    let message = landing_error(
        true,
        &(unit_ashore(1, 100, 0, 0) + &order("land_unit = 1", "hex = [4, 0]\nroster = []")),
    );
    assert!(
        message.contains("set_posture, march or set_activity"),
        "{message}"
    );
    // The hex and the roster are both needed, the hex is two whole
    // numbers and the roster is whole numbers.
    for keys in [
        "roster = []",
        "hex = [4, 0]",
        "hex = [4]\nroster = []",
        "hex = [4, 0]\nroster = [1.5]",
        "hex = [4, 0]\nroster = [-1]",
        "hex = [4, 0]\nroster = []\nmarch = 1",
    ] {
        assert!(
            landing(true, &order("task_force = 1", keys)).is_err(),
            "{keys}"
        );
    }
}

#[test]
fn a_set_activity_order_names_a_land_unit_and_ready_or_train() {
    let order = |activity: &str| {
        format!(
                "{}\n[[order]]\nday = 3\nside = \"allied\"\nland_unit = 1\nkind = \"set_activity\"\nactivity = \"{activity}\"\n",
                unit_at_base(1)
            )
    };
    for (name, wanted) in [("train", Activity::Train), ("ready", Activity::Ready)] {
        let scenario = landing(true, &order(name)).expect("loads");
        match &scenario.scripted_orders()[&3][0].1.kind {
            OrderKind::LandUnit { target, order } => {
                assert_eq!(*target, LandUnitId::new(1));
                assert_eq!(*order, LandUnitOrder::SetActivity(wanted));
            }
            other => panic!("not a land unit order: {other:?}"),
        }
    }
    // An unknown activity, none, and the order given to a task force
    // are refused.
    assert!(landing(true, &order("idle")).is_err());
    let none = unit_at_base(1)
        + "\n[[order]]\nday = 3\nside = \"allied\"\nland_unit = 1\nkind = \"set_activity\"\n";
    assert!(landing(true, &none).is_err());
    let message = landing_error(
            true,
            &(unit_at_base(1)
                + "\n[[order]]\nday = 3\nside = \"allied\"\ntask_force = 1\nkind = \"set_activity\"\nactivity = \"train\"\n"),
        );
    assert!(
        message.contains("set_activity") && message.contains("land_unit"),
        "{message}"
    );
}

#[test]
fn a_set_activity_order_may_be_work_with_a_job() {
    let order = |extra: &str| {
        format!(
                "{}\n[[order]]\nday = 3\nside = \"allied\"\nland_unit = 1\nkind = \"set_activity\"\n{extra}",
                unit_at_base(1)
            )
    };
    let wanted_activity = |extra: &str| {
        let scenario = landing(true, &order(extra)).expect("loads");
        match &scenario.scripted_orders()[&3][0].1.kind {
            OrderKind::LandUnit { order, .. } => order.clone(),
            other => panic!("not a land unit order: {other:?}"),
        }
    };
    assert_eq!(
        wanted_activity("activity = \"work\"\n"),
        LandUnitOrder::SetActivity(Activity::Work(None))
    );
    for (name, job) in [
        ("airfield", Job::Airfield),
        ("port", Job::Port),
        ("aircraft", Job::Aircraft),
        ("defences", Job::Defences),
    ] {
        assert_eq!(
            wanted_activity(&format!("activity = \"work\"\njob = \"{name}\"\n")),
            LandUnitOrder::SetActivity(Activity::Work(Some(job)))
        );
    }
    // An unknown job, and a job with another activity, are refused.
    let message = landing_error(true, &order("activity = \"work\"\njob = \"farming\"\n"));
    assert!(
        message.contains("farming") && message.contains("job"),
        "{message}"
    );
    for activity in ["ready", "train"] {
        let message = landing_error(
            true,
            &order(&format!("activity = \"{activity}\"\njob = \"port\"\n")),
        );
        assert!(message.contains("only the activity work"), "{message}");
    }
}
