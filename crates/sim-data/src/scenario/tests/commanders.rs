use super::*;

/// `HEAD` with `command_levels` as given (nothing when `None`), two
/// allied task forces each with a ship, task force 2 with `superior_2`
/// as an extra key of its table, and `base_extra` added to base 1.
fn command_text(levels: Option<&str>, superior_2: &str, base_extra: &str) -> String {
    let head = HEAD.replace(
        "days = 2\n",
        &format!(
            "days = 2\n{}",
            levels.map_or(String::new(), |v| format!("command_levels = {v}\n"))
        ),
    );
    let head = head.replace("q = 0\nr = 0\n", &format!("q = 0\nr = 0\n{base_extra}\n"));
    let force = |id: u32, extra: &str| {
        format!(
                "[[task_force]]\nid = {id}\nname = \"TF\"\nside = \"allied\"\nat_base = 1\nmission = \"escort\"\n{extra}\n[[ship]]\nid = {id}\nname = \"S\"\nclass = 1\nside = \"allied\"\nfuel_t = 100\ntask_force = {id}\n"
            )
    };
    format!("{head}{}{}", force(1, ""), force(2, superior_2))
}

fn load_command_text(text: &str) -> Result<Scenario, DataError> {
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../data/scenarios/test.toml");
    Scenario::parse(text, &path)
}

fn command_ids(scenario: &Scenario, side: Side) -> Vec<CommandRef> {
    scenario
        .world
        .commands(side)
        .map(|posts| posts.keys().copied().collect())
        .unwrap_or_default()
}

#[test]
fn command_levels_are_off_when_the_key_is_absent_or_false() {
    for levels in [None, Some("false")] {
        let scenario = load_command_text(&command_text(levels, "", "")).expect("loads");
        assert!(!scenario.world.command_levels());
        assert!(scenario.world.commands(Side::Allied).is_none());
        assert!(scenario.command_levels().is_none());
    }
}

#[test]
fn command_levels_on_gives_each_side_its_commands() {
    let scenario = load_command_text(&command_text(Some("true"), "", "")).expect("loads");
    assert!(scenario.world.command_levels());
    assert_eq!(scenario.command_levels(), Some(&[][..]));
    let force = |raw| CommandRef::TaskForce(TaskForceId::new(raw));
    assert_eq!(
        command_ids(&scenario, Side::Allied),
        vec![
            CommandRef::Area,
            force(1),
            force(2),
            CommandRef::Base(BaseId::new(1))
        ]
    );
    assert_eq!(
        command_ids(&scenario, Side::Japanese),
        vec![CommandRef::Area, CommandRef::Base(BaseId::new(2))]
    );
    let posts = scenario.world.commands(Side::Allied).expect("on");
    assert_eq!(posts[&CommandRef::Area].superior(), None);
    assert_eq!(posts[&force(1)].superior(), Some(CommandRef::Area));
    // The seed copy turns them on likewise.
    let other = scenario.with_seed(9).expect("copies");
    assert_eq!(
        command_ids(&other, Side::Allied),
        command_ids(&scenario, Side::Allied)
    );
    assert_eq!(other.command_levels(), scenario.command_levels());
}

#[test]
fn a_named_superior_is_kept_and_with_seed_names_it_again() {
    let scenario = load_command_text(&command_text(
        Some("true"),
        "superior = { task_force = 1 }",
        "superior = { task_force = 2 }",
    ))
    .expect("loads");
    let force = |raw| CommandRef::TaskForce(TaskForceId::new(raw));
    let posts = scenario.world.commands(Side::Allied).expect("on");
    assert_eq!(posts[&force(2)].superior(), Some(force(1)));
    assert_eq!(
        posts[&CommandRef::Base(BaseId::new(1))].superior(),
        Some(force(2))
    );
    assert_eq!(scenario.command_levels().expect("on").len(), 2);
    let other = scenario.with_seed(3).expect("copies");
    let posts = other.world.commands(Side::Allied).expect("on");
    assert_eq!(posts[&force(2)].superior(), Some(force(1)));
    assert_eq!(
        posts[&CommandRef::Base(BaseId::new(1))].superior(),
        Some(force(2))
    );
}

#[test]
fn a_superior_with_command_levels_off_is_refused_naming_the_entry() {
    for levels in [None, Some("false")] {
        let message = load_command_text(&command_text(levels, "superior = { task_force = 1 }", ""))
            .expect_err("refused")
            .message;
        assert!(message.contains("task force 2"), "{message}");
        assert!(message.contains("command levels are off"), "{message}");
        let message = load_command_text(&command_text(levels, "", "superior = { task_force = 1 }"))
            .expect_err("refused")
            .message;
        assert!(message.contains("base 1"), "{message}");
    }
}

#[test]
fn a_superior_that_is_no_command_names_itself_or_makes_a_loop_is_refused() {
    // An unknown command, and a base that is not the side's.
    for superior in [
        "superior = { task_force = 9 }",
        "superior = { base = 2 }",
        "superior = { base = 7 }",
    ] {
        let message = load_command_text(&command_text(Some("true"), superior, ""))
            .expect_err("refused")
            .message;
        assert!(
            message.contains("is not a command"),
            "{superior}: {message}"
        );
    }
    // Itself.
    let message = load_command_text(&command_text(
        Some("true"),
        "superior = { task_force = 2 }",
        "",
    ))
    .expect_err("refused")
    .message;
    assert!(message.contains("its own superior"), "{message}");
    // A loop: base 1 under task force 2, task force 2 under base 1.
    let message = load_command_text(&command_text(
        Some("true"),
        "superior = { base = 1 }",
        "superior = { task_force = 2 }",
    ))
    .expect_err("refused")
    .message;
    assert!(message.contains("loop"), "{message}");
    // Neither or both of task_force and base.
    for superior in [
        "superior = {}",
        "superior = { task_force = 1, base = 1 }",
        "superior = { fleet = 1 }",
    ] {
        assert!(
            load_command_text(&command_text(Some("true"), superior, "")).is_err(),
            "{superior}"
        );
    }
}

// -----------------------------------------------------------------
// A scenario's commands (decision 0107, item 16).
// -----------------------------------------------------------------

fn command(body: &str) -> String {
    format!("\n[[command]]\nday = 1\nside = \"allied\"\n{body}\n")
}

fn only(extra: &str) -> CommandKind {
    let scenario = parse_with(extra).expect("loads");
    let list = &scenario.scripted_commands()[&1];
    assert_eq!(list.len(), 1);
    assert_eq!(list[0].0, Side::Allied);
    list[0].1.clone()
}

#[test]
fn the_pursuit_commands_load_and_refuse_what_they_do_not_take() {
    use sim_core::command::{
        InterruptRef, InterruptSetting, PursueLeave, StandingKind, StandingSubject,
    };
    use sim_core::ids::TaskForceId;
    for (name, leave) in [
        ("ask", PursueLeave::Ask),
        ("go", PursueLeave::Go),
        ("stay", PursueLeave::Stay),
    ] {
        assert_eq!(
            only(&command(&format!(
                "kind = \"set_pursuit\"\nintent = 4\nleave = \"{name}\""
            ))),
            CommandKind::SetPursuit {
                intent: IntentId::new(4),
                leave: Some(leave),
            }
        );
    }
    // Left out, the setting is cleared.
    assert_eq!(
        only(&command("kind = \"set_pursuit\"\nintent = 4")),
        CommandKind::SetPursuit {
            intent: IntentId::new(4),
            leave: None,
        }
    );
    assert!(error_of(&command(
        "kind = \"set_pursuit\"\nintent = 4\nleave = \"maybe\""
    ))
    .contains("not ask, go or stay"));
    assert!(
        error_of(&command("kind = \"set_pursuit\"\nleave = \"go\"")).contains("needs \"intent\"")
    );
    assert!(
        error_of(&command("kind = \"set_pursuit\"\nintent = 4\nbase = 1"))
            .contains("does not take \"base\"")
    );
    for allow in [true, false] {
        assert_eq!(
            only(&command(&format!(
                "kind = \"answer_pursuit\"\ntask_force = 2\ncontact = 7\nallow = {allow}"
            ))),
            CommandKind::AnswerPursuit {
                force: TaskForceId::new(2),
                contact: 7,
                allow,
            }
        );
    }
    for missing in [
        "kind = \"answer_pursuit\"\ncontact = 7\nallow = true",
        "kind = \"answer_pursuit\"\ntask_force = 2\nallow = true",
        "kind = \"answer_pursuit\"\ntask_force = 2\ncontact = 7",
    ] {
        assert!(error_of(&command(missing)).contains("needs"), "{missing}");
    }
    assert!(error_of(&command(
        "kind = \"answer_pursuit\"\ntask_force = 2\ncontact = 7\nallow = true\nintent = 1"
    ))
    .contains("does not take \"intent\""));
    // The interrupt's standing kind and its subject.
    assert_eq!(
        only(&command(
            "kind = \"acknowledge\"\nstanding = \"asks_leave_to_pursue\"\ntask_force = 2"
        )),
        CommandKind::Acknowledge {
            interrupt: InterruptRef::Standing {
                kind: StandingKind::AsksLeaveToPursue,
                subject: StandingSubject::TaskForce(TaskForceId::new(2)),
            }
        }
    );
    assert_eq!(
        only(&command(
            "kind = \"set_interrupt\"\nstanding = \"asks_leave_to_pursue\"\non = false"
        )),
        CommandKind::SetInterrupt(InterruptSetting::Standing {
            kind: StandingKind::AsksLeaveToPursue,
            on: false
        })
    );
}

#[test]
fn the_targets_limit_loads_with_kind_names_and_clears_with_none() {
    use sim_core::command::{Limit, TargetsLimit};
    use sim_core::refdata::ShipKind;
    let limit = |keys: &str| {
        command(&format!(
            "kind = \"set_limit\"\nlimit = \"targets\"\nintent = 4{keys}"
        ))
    };
    assert_eq!(
        only(&limit(
            "\nkinds = [\"transport\", \"carrier\"]\nwaters_first = true"
        )),
        CommandKind::SetLimit(Limit::Targets {
            intent: IntentId::new(4),
            targets: Some(TargetsLimit {
                kinds: vec![ShipKind::Transport, ShipKind::Carrier],
                waters_first: true,
            }),
        })
    );
    assert_eq!(
        only(&limit("\nkinds = [\"cruiser\"]")),
        CommandKind::SetLimit(Limit::Targets {
            intent: IntentId::new(4),
            targets: Some(TargetsLimit {
                kinds: vec![ShipKind::Cruiser],
                waters_first: false,
            }),
        })
    );
    for cleared in ["", "\nkinds = []"] {
        assert_eq!(
            only(&limit(cleared)),
            CommandKind::SetLimit(Limit::Targets {
                intent: IntentId::new(4),
                targets: None,
            })
        );
    }
    assert!(error_of(&limit("\nkinds = [\"submarine\"]")).contains("submarine"));
    assert!(parse_with(&limit("\nkinds = [\"carrier\"]\nrank = 1")).is_err());
}

#[test]
fn every_commander_command_loads_as_the_command_it_names() {
    use sim_core::command::{ForcesLimit, IntentKind, Leave, Limit, PlanId, Risk, Want, Wants};
    use sim_core::ids::LandUnitId;
    let intent = IntentId::new(1);
    assert_eq!(
        only(&command(
            "kind = \"set_intent\"\nintent = 1\nbase = 2\nfloor_days = 10\ninterval_days = 15"
        )),
        CommandKind::SetIntent {
            parent: None,
            intent,
            kind: IntentKind::KeepSupplied {
                base: BaseId::new(2),
                floor_days: 10,
                interval_days: Some(15),
            },
        }
    );
    // The interval is optional.
    assert_eq!(
        only(&command(
            "kind = \"set_intent\"\nintent = 1\nbase = 2\nfloor_days = 10"
        )),
        CommandKind::SetIntent {
            parent: None,
            intent,
            kind: IntentKind::KeepSupplied {
                base: BaseId::new(2),
                floor_days: 10,
                interval_days: None,
            },
        }
    );
    assert_eq!(
        only(&command("kind = \"withdraw_intent\"\nintent = 1")),
        CommandKind::WithdrawIntent { intent }
    );
    assert_eq!(
            only(&command(
                "kind = \"set_limit\"\nlimit = \"forces\"\nintent = 1\nships = [4, 3]\ntask_forces = [2]\nsource = 1"
            )),
            CommandKind::SetLimit(Limit::Forces {
                intent,
                limit: ForcesLimit {
                    ships: [ShipId::new(3), ShipId::new(4)].into_iter().collect(),
                    task_forces: [TaskForceId::new(2)].into_iter().collect(),
                    air_groups: BTreeSet::new(),
                    land_units: BTreeSet::new(),
                    source: Some(BaseId::new(1)),
                    submarines: Default::default(),
                    none_of: Default::default(),
                    in_place: false,
                },
            })
        );
    // Decision 0141: land units in a forces limit.
    assert_eq!(
        only(&command(
            "kind = \"set_limit\"\nlimit = \"forces\"\nintent = 2\nland_units = [5, 3]"
        )),
        CommandKind::SetLimit(Limit::Forces {
            intent: IntentId::new(2),
            limit: ForcesLimit {
                ships: BTreeSet::new(),
                task_forces: BTreeSet::new(),
                air_groups: BTreeSet::new(),
                land_units: [LandUnitId::new(3), LandUnitId::new(5)]
                    .into_iter()
                    .collect(),
                source: None,
                submarines: Default::default(),
                none_of: Default::default(),
                in_place: false,
            },
        })
    );
    // Decision 0141: the two new intent kinds.
    assert_eq!(
            only(&command(
                "kind = \"set_intent\"\nintent = 3\nintent_kind = \"ready_landing\"\ntask_force = 2\nhex = [6, 0]\nby_day = 6"
            )),
            CommandKind::SetIntent { parent: None,
                intent: IntentId::new(3),
                kind: IntentKind::ReadyLanding {
                    force: TaskForceId::new(2),
                    hex: Hex::new(6, 0),
                    by_day: Some(6),
                },
            }
        );
    assert_eq!(
            only(&command(
                "kind = \"set_intent\"\nintent = 3\nintent_kind = \"ready_landing\"\ntask_force = 2\nhex = [6, 0]"
            )),
            CommandKind::SetIntent { parent: None,
                intent: IntentId::new(3),
                kind: IntentKind::ReadyLanding {
                    force: TaskForceId::new(2),
                    hex: Hex::new(6, 0),
                    by_day: None,
                },
            }
        );
    assert_eq!(
            only(&command(
                "kind = \"set_intent\"\nintent = 4\nintent_kind = \"build_up\"\nbase = 1\nwants = [\"airfield\", \"defences\"]\ndefences = 10"
            )),
            CommandKind::SetIntent { parent: None,
                intent: IntentId::new(4),
                kind: IntentKind::BuildUp {
                    base: BaseId::new(1),
                    wants: Wants::new(&[Want::Airfield, Want::Defences(10)]).expect("good wants"),
                },
            }
        );
    // Decision 0142: a place to take, with and without a date.
    assert_eq!(
            only(&command(
                "kind = \"set_intent\"\nintent = 5\nintent_kind = \"take_place\"\nhex = [6, 0]\nby_day = 9"
            )),
            CommandKind::SetIntent { parent: None,
                intent: IntentId::new(5),
                kind: IntentKind::TakePlace {
                    hex: Hex::new(6, 0),
                    by_day: Some(9),
                },
            }
        );
    assert_eq!(
        only(&command(
            "kind = \"set_intent\"\nintent = 5\nintent_kind = \"take_place\"\nhex = [6, 0]"
        )),
        CommandKind::SetIntent {
            parent: None,
            intent: IntentId::new(5),
            kind: IntentKind::TakePlace {
                hex: Hex::new(6, 0),
                by_day: None,
            },
        }
    );
    // Decision 0151: an island to control, with and without a date.
    assert_eq!(
            only(&command(
                "kind = \"set_intent\"\nintent = 6\nintent_kind = \"control_island\"\nisland = 2\nby_day = 9"
            )),
            CommandKind::SetIntent { parent: None,
                intent: IntentId::new(6),
                kind: IntentKind::ControlIsland {
                    island: 2,
                    by_day: Some(9),
                },
            }
        );
    assert_eq!(
        only(&command(
            "kind = \"set_intent\"\nintent = 6\nintent_kind = \"control_island\"\nisland = 2"
        )),
        CommandKind::SetIntent {
            parent: None,
            intent: IntentId::new(6),
            kind: IntentKind::ControlIsland {
                island: 2,
                by_day: None,
            },
        }
    );
    // Decision 0146: a base to bombard with a task force.
    assert_eq!(
            only(&command(
                "kind = \"set_intent\"\nintent = 5\nintent_kind = \"bombard\"\ntask_force = 3\ntarget = 12"
            )),
            CommandKind::SetIntent { parent: None,
                intent: IntentId::new(5),
                kind: IntentKind::Bombard {
                    force: TaskForceId::new(3),
                    target: BaseId::new(12),
                },
            }
        );
    // The step 2 shapes: air groups, no source, no ships, no task forces.
    assert_eq!(
        only(&command(
            "kind = \"set_limit\"\nlimit = \"forces\"\nintent = 2\nair_groups = [9, 7]"
        )),
        CommandKind::SetLimit(Limit::Forces {
            intent: IntentId::new(2),
            limit: ForcesLimit {
                ships: BTreeSet::new(),
                task_forces: BTreeSet::new(),
                air_groups: [AirGroupId::new(7), AirGroupId::new(9)]
                    .into_iter()
                    .collect(),
                land_units: BTreeSet::new(),
                source: None,
                submarines: Default::default(),
                none_of: Default::default(),
                in_place: false,
            },
        })
    );
    // The intent kinds: cover and hold name a base and nothing else.
    for (name, kind) in [
        (
            "cover",
            IntentKind::Cover {
                base: BaseId::new(2),
            },
        ),
        (
            "hold",
            IntentKind::Hold {
                base: BaseId::new(2),
            },
        ),
    ] {
        assert_eq!(
            only(&command(&format!(
                "kind = \"set_intent\"\nintent = 3\nintent_kind = \"{name}\"\nbase = 2"
            ))),
            CommandKind::SetIntent {
                parent: None,
                intent: IntentId::new(3),
                kind
            }
        );
    }
    assert_eq!(
            only(&command(
                "kind = \"set_intent\"\nintent = 1\nintent_kind = \"keep_supplied\"\nbase = 2\nfloor_days = 10"
            )),
            CommandKind::SetIntent { parent: None,
                intent,
                kind: IntentKind::KeepSupplied {
                    base: BaseId::new(2),
                    floor_days: 10,
                    interval_days: None,
                },
            }
        );
    assert_eq!(
        only(&command(
            "kind = \"set_limit\"\nlimit = \"area\"\nintent = 1\nhexes = [[0, 0], [1, -1]]"
        )),
        CommandKind::SetLimit(Limit::Area {
            intent,
            hexes: [Hex::new(0, 0), Hex::new(1, -1)].into_iter().collect(),
        })
    );
    assert_eq!(
        only(&command(
            "kind = \"set_limit\"\nlimit = \"priority\"\nintent = 1\nrank = 2"
        )),
        CommandKind::SetLimit(Limit::Priority {
            intent,
            rank: Some(2)
        })
    );
    assert_eq!(
        only(&command(
            "kind = \"set_limit\"\nlimit = \"protected\"\nintent = 1\nships = [3]"
        )),
        CommandKind::SetLimit(Limit::Protected {
            intent,
            ships: [ShipId::new(3)].into_iter().collect(),
        })
    );
    // A weight is a whole number or a decimal string.
    assert_eq!(
            only(&command(
                "kind = \"set_limit\"\nlimit = \"reserve\"\nbase = 1\nsupply_t = 400\nfuel_t = \"12.5\""
            )),
            CommandKind::SetLimit(Limit::Reserve {
                base: BaseId::new(1),
                supply_t: Fixed::from_int(400),
                fuel_t: "12.5".parse().expect("a weight"),
            })
        );
    assert_eq!(
        only(&command(
            "kind = \"set_limit\"\nlimit = \"risk\"\nintent = 1\nrisk = \"press\""
        )),
        CommandKind::SetLimit(Limit::Risk {
            intent,
            risk: Risk::Press
        })
    );
    assert_eq!(
        only(&command(
            "kind = \"set_leave\"\nleave = \"launch\"\ngiven = true"
        )),
        CommandKind::SetLeave {
            leave: Leave::Launch,
            given: true
        }
    );
    assert_eq!(
        only(&command("kind = \"approve\"\nplan = 1")),
        CommandKind::Approve {
            plan: PlanId::new(1)
        }
    );
    assert_eq!(
        only(&command("kind = \"cancel_plan\"\nplan = 3")),
        CommandKind::CancelPlan {
            plan: PlanId::new(3)
        }
    );
}

/// Decision 0232, rule 1: a `set_intent` takes `parent`, an intent
/// number, of any kind (the engine refuses a pair that is not open); no
/// other command takes it, and a float is not a number.
#[test]
fn a_set_intent_takes_a_parent_and_no_other_command_does() {
    use sim_core::command::IntentKind;
    use sim_core::hex::Hex;
    use sim_core::ids::BaseId;
    assert_eq!(
            only(&command(
                "kind = \"set_intent\"\nintent = 2\nparent = 1\nintent_kind = \"take_place\"\nhex = [6, 0]"
            )),
            CommandKind::SetIntent {
                parent: Some(IntentId::new(1)),
                intent: IntentId::new(2),
                kind: IntentKind::TakePlace {
                    hex: Hex::new(6, 0),
                    by_day: None,
                },
            }
        );
    // Left out, it is none: a root, as before.
    assert_eq!(
        only(&command(
            "kind = \"set_intent\"\nintent = 2\nintent_kind = \"take_place\"\nhex = [6, 0]"
        )),
        CommandKind::SetIntent {
            parent: None,
            intent: IntentId::new(2),
            kind: IntentKind::TakePlace {
                hex: Hex::new(6, 0),
                by_day: None,
            },
        }
    );
    // Any kind of intent takes the key; the engine's pairs refuse.
    assert_eq!(
        only(&command(
            "kind = \"set_intent\"\nintent = 2\nparent = 1\nintent_kind = \"hold\"\nbase = 2"
        )),
        CommandKind::SetIntent {
            parent: Some(IntentId::new(1)),
            intent: IntentId::new(2),
            kind: IntentKind::Hold {
                base: BaseId::new(2)
            },
        }
    );
    // No other command takes it.
    assert!(error_of(&command(
        "kind = \"withdraw_intent\"\nintent = 2\nparent = 1"
    ))
    .contains("does not take \"parent\""));
    assert!(error_of(&command(
        "kind = \"set_limit\"\nlimit = \"risk\"\nintent = 2\nrisk = \"press\"\nparent = 1"
    ))
    .contains("does not take \"parent\""));
    // A float is not an intent number.
    assert!(parse_with(&command(
            "kind = \"set_intent\"\nintent = 2\nparent = 1.0\nintent_kind = \"take_place\"\nhex = [6, 0]"
        ))
        .is_err());
}

#[test]
fn a_landing_or_build_up_intent_with_wrong_keys_is_refused() {
    let intent = |body: &str| {
        error_of(&command(&format!(
            "kind = \"set_intent\"\nintent = 1\n{body}"
        )))
    };
    let landing = "intent_kind = \"ready_landing\"\ntask_force = 2\nhex = [6, 0]";
    assert!(intent(&format!("{landing}\nbase = 1")).contains("does not take \"base\""));
    assert!(intent(&format!("{landing}\nby_day = 0")).contains("by_day 0"));
    assert!(
        intent("intent_kind = \"ready_landing\"\nhex = [6, 0]").contains("needs \"task_force\"")
    );
    let build = "intent_kind = \"build_up\"\nbase = 1";
    assert!(intent(build).contains("needs \"wants\""));
    assert!(intent(&format!("{build}\nwants = [\"defences\"]")).contains("needs \"defences\""));
    assert!(
        intent(&format!("{build}\nwants = [\"airfield\"]\ndefences = 5")).contains("does not want")
    );
    assert!(
        intent(&format!("{build}\nwants = [\"airfield\", \"airfield\"]"))
            .contains("name a kind twice")
    );
    assert!(intent(&format!("{build}\nwants = []")).contains("empty"));
    assert!(
        intent(&format!("{build}\nwants = [\"defences\"]\ndefences = 101"))
            .contains("outside 1 to 100")
    );
    assert!(intent(&format!("{build}\nwants = [\"dock\"]")).contains("not airfield"));
}

#[test]
fn a_take_place_intent_with_wrong_keys_is_refused() {
    let intent = |body: &str| {
        error_of(&command(&format!(
            "kind = \"set_intent\"\nintent = 1\n{body}"
        )))
    };
    let place = "intent_kind = \"take_place\"\nhex = [6, 0]";
    assert!(intent(&format!("{place}\nbase = 1")).contains("does not take \"base\""));
    assert!(intent(&format!("{place}\ntask_force = 1")).contains("does not take \"task_force\""));
    assert!(intent(&format!("{place}\ntarget = 1")).contains("does not take \"target\""));
    assert!(intent(&format!("{place}\ndistance = 1")).contains("does not take \"distance\""));
    assert!(intent(&format!("{place}\nby_day = 0")).contains("by_day 0"));
    assert!(intent("intent_kind = \"take_place\"").contains("needs \"hex\""));
}

#[test]
fn a_control_island_intent_with_wrong_keys_is_refused() {
    let intent = |body: &str| {
        error_of(&command(&format!(
            "kind = \"set_intent\"\nintent = 1\n{body}"
        )))
    };
    let island = "intent_kind = \"control_island\"\nisland = 1";
    assert!(intent(&format!("{island}\nbase = 1")).contains("does not take \"base\""));
    assert!(intent(&format!("{island}\nhex = [6, 0]")).contains("does not take \"hex\""));
    assert!(intent(&format!("{island}\ntask_force = 1")).contains("does not take \"task_force\""));
    assert!(intent(&format!("{island}\nby_day = 0")).contains("by_day 0"));
    assert!(intent("intent_kind = \"control_island\"").contains("needs \"island\""));
    // Another kind refuses the island key.
    assert!(
        intent("intent_kind = \"take_place\"\nhex = [6, 0]\nisland = 1")
            .contains("does not take \"island\"")
    );
}

#[test]
fn a_bombard_intent_with_wrong_keys_is_refused() {
    let intent = |body: &str| {
        error_of(&command(&format!(
            "kind = \"set_intent\"\nintent = 1\n{body}"
        )))
    };
    let bombard = "intent_kind = \"bombard\"\ntask_force = 3\ntarget = 12";
    for key in ["base = 1", "hex = [6, 0]", "by_day = 2", "distance = 1"] {
        let name = key.split(' ').next().expect("a key");
        assert!(intent(&format!("{bombard}\n{key}")).contains(&format!("does not take \"{name}\"")));
    }
    assert!(intent("intent_kind = \"bombard\"\ntarget = 12").contains("needs \"task_force\""));
    assert!(intent("intent_kind = \"bombard\"\ntask_force = 3").contains("needs \"target\""));
}

#[test]
fn a_hold_or_release_loads_with_its_one_target() {
    use sim_core::command::{HeldSetting, HoldTarget};
    use sim_core::ids::{AirGroupId, LandUnitId};
    for (name, make) in [
        (
            "hold",
            (|t| CommandKind::Hold { target: t }) as fn(HoldTarget) -> CommandKind,
        ),
        ("release", |t| CommandKind::Release { target: t }),
    ] {
        assert_eq!(
            only(&command(&format!("kind = \"{name}\"\nship = 5"))),
            make(HoldTarget::Ship(ShipId::new(5)))
        );
        assert_eq!(
            only(&command(&format!(
                "kind = \"{name}\"\ntask_force = 1000001"
            ))),
            make(HoldTarget::TaskForce(TaskForceId::new(1000001)))
        );
        assert_eq!(
            only(&command(&format!(
                "kind = \"{name}\"\nair_group = 7\nsetting = \"search_arc\""
            ))),
            make(HoldTarget::Setting {
                group: AirGroupId::new(7),
                setting: HeldSetting::SearchArc
            })
        );
        // With no setting the whole group is held.
        assert_eq!(
            only(&command(&format!("kind = \"{name}\"\nair_group = 7"))),
            make(HoldTarget::AirGroup(AirGroupId::new(7)))
        );
        // Decision 0141: a land unit.
        assert_eq!(
            only(&command(&format!("kind = \"{name}\"\nland_unit = 4"))),
            make(HoldTarget::LandUnit(LandUnitId::new(4)))
        );
    }
}

#[test]
fn a_hold_or_release_with_a_wrong_target_or_key_is_refused_and_end_run_cannot_be_written() {
    for name in ["hold", "release"] {
        let with = |body: &str| error_of(&command(&format!("kind = \"{name}\"\n{body}")));
        assert!(with("").contains("exactly one of"));
        assert!(with("ship = 1\ntask_force = 2").contains("exactly one of"));
        assert!(
            with("ship = 1\nair_group = 2\nsetting = \"search_arc\"").contains("exactly one of")
        );
        assert!(with("air_group = 2\nsetting = \"mission\"").contains("not search_arc"));
        assert!(with("ship = 1\nsetting = \"search_arc\"").contains("only with \"air_group\""));
        assert!(
            with("task_force = 1\nsetting = \"search_arc\"").contains("only with \"air_group\"")
        );
        assert!(with("ship = 1\nland_unit = 2").contains("exactly one of"));
        assert!(with("land_unit = 2\nsetting = \"search_arc\"").contains("only with \"air_group\""));
        assert!(with("ship = 1\nplan = 1").contains("does not take \"plan\""));
        assert!(parse_with(&command(&format!("kind = \"{name}\"\nship = 1.5"))).is_err());
    }
    assert!(error_of(&command("kind = \"end_run\"\nplan = 1"))
        .contains("the staff's commands are the staff's to give"));
}

#[test]
fn an_interrupt_command_loads_in_each_of_its_forms() {
    use sim_core::command::{
        InterruptCondition, InterruptRef, InterruptSetting, StandingKind, StandingSubject,
    };
    use sim_core::ids::InterruptId;
    assert_eq!(
        only(&command(
            "kind = \"set_interrupt\"\ninterrupt = 1\nbase = 2\nbelow_days = 35"
        )),
        CommandKind::SetInterrupt(InterruptSetting::Set {
            interrupt: InterruptId::new(1),
            condition: InterruptCondition::SupplyBelow {
                base: BaseId::new(2),
                days: 35
            }
        })
    );
    for (name, kind) in [
        ("cannot_meet", StandingKind::CannotMeet),
        ("cannot_bring_enough", StandingKind::CannotBringEnough),
        ("lift_turned_back", StandingKind::LiftTurnedBack),
        ("waits_for_leave", StandingKind::WaitsForLeave),
        ("ship_lost", StandingKind::ShipLost),
    ] {
        assert_eq!(
            only(&command(&format!(
                "kind = \"set_interrupt\"\nstanding = \"{name}\"\non = false"
            ))),
            CommandKind::SetInterrupt(InterruptSetting::Standing { kind, on: false })
        );
    }
    assert_eq!(
        only(&command("kind = \"acknowledge\"\ninterrupt = 3")),
        CommandKind::Acknowledge {
            interrupt: InterruptRef::Set(InterruptId::new(3))
        }
    );
    assert_eq!(
        only(&command(
            "kind = \"acknowledge\"\nstanding = \"waits_for_leave\"\nplan = 1"
        )),
        CommandKind::Acknowledge {
            interrupt: InterruptRef::Standing {
                kind: StandingKind::WaitsForLeave,
                subject: StandingSubject::Plan(sim_core::command::PlanId::new(1))
            }
        }
    );
    assert_eq!(
        only(&command(
            "kind = \"acknowledge\"\nstanding = \"ship_lost\"\nship = 4"
        )),
        CommandKind::Acknowledge {
            interrupt: InterruptRef::Standing {
                kind: StandingKind::ShipLost,
                subject: StandingSubject::Ship(ShipId::new(4))
            }
        }
    );
}

#[test]
fn an_interrupt_command_with_a_wrong_key_or_value_is_refused() {
    let set = |body: &str| error_of(&command(&format!("kind = \"set_interrupt\"\n{body}")));
    let ack = |body: &str| error_of(&command(&format!("kind = \"acknowledge\"\n{body}")));
    // A missing key.
    assert!(set("interrupt = 1\nbase = 2").contains("needs \"below_days\""));
    assert!(set("interrupt = 1\nbelow_days = 5").contains("needs \"base\""));
    assert!(set("base = 2\nbelow_days = 5").contains("needs \"interrupt\""));
    assert!(set("standing = \"cannot_meet\"").contains("needs \"on\""));
    assert!(ack("").contains("needs \"interrupt\""));
    assert!(ack("standing = \"cannot_meet\"").contains("exactly one of"));
    assert!(ack("standing = \"cannot_meet\"\nplan = 1\nship = 2").contains("exactly one of"));
    // A key the kind does not take.
    assert!(
        set("interrupt = 1\nbase = 2\nbelow_days = 5\nplan = 1").contains("does not take \"plan\"")
    );
    assert!(ack("interrupt = 1\nbase = 2").contains("does not take \"base\""));
    // A mixture of the two forms.
    assert!(
        set("interrupt = 1\nbase = 2\nbelow_days = 5\nstanding = \"ship_lost\"\non = true")
            .contains("mixes the two forms")
    );
    assert!(
        set("interrupt = 1\nstanding = \"ship_lost\"\non = true").contains("mixes the two forms")
    );
    assert!(
        ack("interrupt = 1\nstanding = \"cannot_meet\"\nplan = 1").contains("mixes the two forms")
    );
    // An unknown standing kind.
    assert!(set("standing = \"cannot_swim\"\non = true").contains("not cannot_meet"));
    assert!(ack("standing = \"cannot_swim\"\nplan = 1").contains("not cannot_meet"));
    // A TOML float.
    assert!(parse_with(&command(
        "kind = \"set_interrupt\"\ninterrupt = 1\nbase = 2\nbelow_days = 5.5"
    ))
    .is_err());
    assert!(parse_with(&command("kind = \"acknowledge\"\ninterrupt = 1.5")).is_err());
}

#[test]
fn commands_are_kept_by_day_in_the_order_given_and_survive_a_new_seed() {
    let text = format!(
            "{}{}",
            command("kind = \"approve\"\nplan = 1"),
            "\n[[command]]\nday = 2\nside = \"japanese\"\nkind = \"withdraw_intent\"\nintent = 4\n\n[[command]]\nday = 1\nside = \"japanese\"\nkind = \"cancel_plan\"\nplan = 2\n"
        );
    let scenario = parse_with(&text).expect("loads");
    assert_eq!(scenario.scripted_commands().len(), 2);
    let first: Vec<Side> = scenario.scripted_commands()[&1]
        .iter()
        .map(|(side, _)| *side)
        .collect();
    assert_eq!(first, [Side::Allied, Side::Japanese]);
    let again = scenario.with_seed(77).expect("a new seed");
    assert_eq!(again.scripted_commands(), scenario.scripted_commands());
    // A scenario with none has none.
    assert!(parse_with("")
        .expect("loads")
        .scripted_commands()
        .is_empty());
}

/// Decision 0220, R12: the order to destroy a force takes a contact, a
/// distance and search only, false where left out.
#[test]
fn the_order_to_destroy_a_force_loads_with_its_contact_and_its_distance() {
    use sim_core::command::IntentKind;
    assert_eq!(
            only(&command(
                "kind = \"set_intent\"\nintent = 1\nintent_kind = \"destroy_force\"\ncontact = 2\ndistance = 3"
            )),
            CommandKind::SetIntent { parent: None,
                intent: IntentId::new(1),
                kind: IntentKind::DestroyForce {
                    contact: 2,
                    distance: 3,
                    search_only: false
                }
            }
        );
    assert_eq!(
            only(&command(
                "kind = \"set_intent\"\nintent = 1\nintent_kind = \"destroy_force\"\ncontact = 2\ndistance = 0\nsearch_only = true"
            )),
            CommandKind::SetIntent { parent: None,
                intent: IntentId::new(1),
                kind: IntentKind::DestroyForce {
                    contact: 2,
                    distance: 0,
                    search_only: true
                }
            }
        );
    // It takes a contact and a distance and no hex or base.
    for body in [
        "intent_kind = \"destroy_force\"\ndistance = 3",
        "intent_kind = \"destroy_force\"\ncontact = 2",
        "intent_kind = \"destroy_force\"\ncontact = 2\ndistance = 3\nhex = [1, 1]",
        "intent_kind = \"destroy_force\"\ncontact = 2\ndistance = 3\nbase = 1",
    ] {
        assert!(
            parse_with(&command(&format!(
                "kind = \"set_intent\"\nintent = 1\n{body}"
            )))
            .is_err(),
            "{body}"
        );
    }
}

/// Decision 0218, stage 3: the patrol order, a hold on a boat and a
/// forces limit naming boats are given by a scenario's commands.
#[test]
fn the_patrol_order_a_hold_on_a_boat_and_a_limit_naming_boats_load() {
    use sim_core::command::{ForcesLimit, HoldTarget, IntentKind, Limit};
    use sim_core::ids::SubmarineId;
    assert_eq!(
            only(&command(
                "kind = \"set_intent\"\nintent = 3\nintent_kind = \"patrol\"\nhex = [5, -2]\ndistance = 4"
            )),
            CommandKind::SetIntent { parent: None,
                intent: IntentId::new(3),
                kind: IntentKind::Patrol {
                    hex: Hex::new(5, -2),
                    distance: 4
                }
            }
        );
    // A patrol takes a hex and a distance and no base.
    for body in [
        "intent_kind = \"patrol\"\nhex = [5, -2]",
        "intent_kind = \"patrol\"\ndistance = 4",
        "intent_kind = \"patrol\"\nhex = [5, -2]\ndistance = 4\nbase = 1",
    ] {
        assert!(
            parse_with(&command(&format!(
                "kind = \"set_intent\"\nintent = 3\n{body}"
            )))
            .is_err(),
            "{body}"
        );
    }
    for (kind, make) in [("hold", true), ("release", false)] {
        let target = HoldTarget::Submarine(SubmarineId::new(6));
        assert_eq!(
            only(&command(&format!("kind = \"{kind}\"\nsubmarine = 6"))),
            if make {
                CommandKind::Hold { target }
            } else {
                CommandKind::Release { target }
            }
        );
    }
    // A hold names exactly one target.
    assert!(parse_with(&command("kind = \"hold\"\nsubmarine = 6\nship = 1")).is_err());
    assert!(parse_with(&command(
        "kind = \"hold\"\nsubmarine = 6\nsetting = \"search_arc\""
    ))
    .is_err());
    assert_eq!(
        only(&command(
            "kind = \"set_limit\"\nlimit = \"forces\"\nintent = 3\nsubmarines = [2, 6]"
        )),
        CommandKind::SetLimit(Limit::Forces {
            intent: IntentId::new(3),
            limit: ForcesLimit {
                ships: Default::default(),
                task_forces: Default::default(),
                air_groups: Default::default(),
                source: None,
                land_units: Default::default(),
                submarines: [SubmarineId::new(2), SubmarineId::new(6)]
                    .into_iter()
                    .collect(),
                none_of: Default::default(),
                in_place: false,
            }
        })
    );
}

#[test]
fn a_command_with_a_wrong_key_or_value_is_refused() {
    let ok = "kind = \"set_intent\"\nintent = 1\nbase = 2\nfloor_days = 10";
    parse_with(&command(ok)).expect("loads");
    // A key no command has, and a key the kind does not take.
    assert!(parse_with(&command(&format!("{ok}\nspeed = 3"))).is_err());
    assert!(error_of(&command(&format!("{ok}\nplan = 3"))).contains("does not take \"plan\""));
    // A TOML float is refused for a number of days and for a weight.
    assert!(parse_with(&command(
        &ok.replace("floor_days = 10", "floor_days = 10.5")
    ))
    .is_err());
    assert!(parse_with(&command(
        "kind = \"set_limit\"\nlimit = \"reserve\"\nbase = 1\nsupply_t = 4.5\nfuel_t = 3"
    ))
    .is_err());
    // A missing key.
    assert!(
        error_of(&command("kind = \"set_intent\"\nintent = 1\nbase = 2"))
            .contains("needs \"floor_days\"")
    );
    assert!(error_of(&command("kind = \"approve\"")).contains("needs \"plan\""));
    // A cover or hold intent takes no days; a kind that is none is refused.
    for name in ["cover", "hold"] {
        let with = |extra: &str| {
            error_of(&command(&format!(
                "kind = \"set_intent\"\nintent = 1\nintent_kind = \"{name}\"\nbase = 2\n{extra}"
            )))
        };
        assert!(with("floor_days = 10").contains("does not take \"floor_days\""));
        assert!(with("interval_days = 5").contains("does not take \"interval_days\""));
    }
    assert!(error_of(&command(
            "kind = \"set_intent\"\nintent = 1\nintent_kind = \"win\"\nbase = 2"
        ))
        .contains(
            "not keep_supplied, cover, hold, neutralise, raid, deny, strike_ships, search_waters, stand_at, operate_carriers, ready_landing, build_up, take_place, control_island or bombard"
        ));
    // Neutralise and raid name a target; a neutralise takes no base.
    let aimed = |kind: &str, extra: &str| {
        command(&format!(
            "kind = \"set_intent\"\nintent = 4\nintent_kind = \"{kind}\"\n{extra}"
        ))
    };
    assert_eq!(
        only(&aimed("neutralise", "target = 9")),
        CommandKind::SetIntent {
            parent: None,
            intent: IntentId::new(4),
            kind: sim_core::command::IntentKind::Neutralise {
                target: BaseId::new(9)
            },
        }
    );
    assert_eq!(
        only(&aimed("raid", "base = 2\ntarget = 9")),
        CommandKind::SetIntent {
            parent: None,
            intent: IntentId::new(4),
            kind: sim_core::command::IntentKind::Raid {
                base: BaseId::new(2),
                target: BaseId::new(9)
            },
        }
    );
    assert_eq!(
        only(&aimed("deny", "hex = [3, -1]\ndistance = 4")),
        CommandKind::SetIntent {
            parent: None,
            intent: IntentId::new(4),
            kind: sim_core::command::IntentKind::Deny {
                hex: sim_core::hex::Hex::new(3, -1),
                distance: 4
            },
        }
    );
    assert_eq!(
        only(&aimed("strike_ships", "base = 2")),
        CommandKind::SetIntent {
            parent: None,
            intent: IntentId::new(4),
            kind: sim_core::command::IntentKind::StrikeShips {
                base: BaseId::new(2)
            },
        }
    );
    assert_eq!(
        only(&aimed("search_waters", "base = 2")),
        CommandKind::SetIntent {
            parent: None,
            intent: IntentId::new(4),
            kind: sim_core::command::IntentKind::SearchWaters {
                base: BaseId::new(2)
            },
        }
    );
    assert!(error_of(&aimed("deny", "distance = 4")).contains("needs \"hex\""));
    assert!(error_of(&aimed("deny", "hex = [3, -1]")).contains("needs \"distance\""));
    assert!(
        error_of(&aimed("deny", "hex = [3, -1]\ndistance = 4\nbase = 2"))
            .contains("does not take \"base\"")
    );
    assert!(
        error_of(&aimed("neutralise", "base = 2\ntarget = 9")).contains("does not take \"base\"")
    );
    assert!(error_of(&aimed("neutralise", "")).contains("needs \"target\""));
    assert!(error_of(&aimed("raid", "base = 2")).contains("needs \"target\""));
    assert!(error_of(&aimed("raid", "target = 9")).contains("needs \"base\""));
    assert!(error_of(&aimed("cover", "base = 2\ntarget = 9")).contains("does not take \"target\""));
    // A kind that is the staff's, or none, and a limit that is none.
    assert!(error_of(&command("kind = \"commit_ships\"\nplan = 1"))
        .contains("not set_intent, withdraw_intent"));
    assert!(error_of(&command("kind = \"set_limit\"\nintent = 1")).contains("needs a limit"));
    assert!(error_of(&command(
        "kind = \"set_limit\"\nlimit = \"speed\"\nintent = 1"
    ))
    .contains("not forces, area"));
    assert!(error_of(&command(
        "kind = \"set_limit\"\nlimit = \"risk\"\nintent = 1\nrisk = \"rash\""
    ))
    .contains("not preserve, balanced or press"));
    assert!(error_of(&command(
        "kind = \"set_leave\"\nleave = \"sail\"\ngiven = true"
    ))
    .contains("not prepare or launch"));
    // A day outside the scenario's days.
    assert!(
        error_of("\n[[command]]\nday = 3\nside = \"allied\"\nkind = \"approve\"\nplan = 1\n")
            .contains("outside days 1 to 2")
    );
    assert!(
        error_of("\n[[command]]\nday = 0\nside = \"allied\"\nkind = \"approve\"\nplan = 1\n")
            .contains("outside days 1 to 2")
    );
}

#[test]
fn a_scenarios_commands_are_taken_in_on_their_day() {
    // An intent set on day 1 is in force after it, and one withdrawn on
    // day 2 is withdrawn.
    let text = format!(
        "{}{}",
        command("kind = \"set_intent\"\nintent = 1\nbase = 1\nfloor_days = 10"),
        "\n[[command]]\nday = 2\nside = \"allied\"\nkind = \"withdraw_intent\"\nintent = 1\n"
    );
    let mut scenario = parse_with(&text).expect("loads");
    scenario.next_day().expect("day 1");
    let state = scenario.world.command_state(Side::Allied);
    assert!(state.intents[&IntentId::new(1)].in_force());
    assert!(scenario.scripted_commands().get(&1).is_none());
    scenario.next_day().expect("day 2");
    let state = scenario.world.command_state(Side::Allied);
    assert_eq!(state.intents[&IntentId::new(1)].withdrawn_day, Some(2));
    assert!(scenario.world.command_state(Side::Japanese).is_empty());
}

/// Statement 1: a file of sides, by hand.
#[test]
fn a_file_of_sides_says_whose_a_class_is_and_is_refused_where_it_is_not_whole_or_true() {
    use sim_core::refdata::ClassSide;
    let scenario = parse_with_sides(PLACEHOLDER_SIDES, "").expect("loads");
    let ref_data = scenario.world.ref_data();
    assert_eq!(
        ref_data.ship_class_side(ShipClassId::new(1)),
        Some(ClassSide::Allied)
    );
    assert_eq!(
        ref_data.ship_class_side(ShipClassId::new(2)),
        Some(ClassSide::Both)
    );
    assert_eq!(
        ref_data.aircraft_type_side(AircraftTypeId::new(3)),
        Some(ClassSide::Both)
    );
    // A side's view reads it as it reads a class.
    let view = scenario.world.side_view(Side::Japanese);
    assert_eq!(
        view.ship_class_side(ShipClassId::new(1)),
        Some(ClassSide::Allied)
    );
    assert_eq!(
        view.ship_class_side(ShipClassId::new(2)),
        Some(ClassSide::Both)
    );
    assert_eq!(
        view.aircraft_type_side(AircraftTypeId::new(4)),
        Some(ClassSide::Both)
    );
    // The file is one of the scenario's data files, last.
    assert!(scenario
        .data_files()
        .last()
        .expect("files")
        .to_string_lossy()
        .contains("flagplot_class_sides_"));
    // A scenario that names no file has none.
    let plain = parse_with("").expect("loads");
    assert_eq!(plain.world.ref_data().class_sides, None);
    assert_eq!(
        plain
            .world
            .side_view(Side::Allied)
            .ship_class_side(ShipClassId::new(1)),
        None
    );

    let refused = |rows: &str, extra: &str| -> String {
        parse_with_sides(rows, extra)
            .expect_err("the scenario is refused")
            .message
    };
    // A class the files give with no row.
    let message = refused(
        &PLACEHOLDER_SIDES.replace("ship_class,4,both,test\n", ""),
        "",
    );
    assert!(
        message.contains("ship class 4 has no row in the file of sides"),
        "{message}"
    );
    // A type the files give with no row.
    let message = refused(
        &PLACEHOLDER_SIDES.replace("aircraft_type,2,both,test\n", ""),
        "",
    );
    assert!(
        message.contains("aircraft type 2 has no row in the file of sides"),
        "{message}"
    );
    // A row for a class they do not give.
    let message = refused(&format!("{PLACEHOLDER_SIDES}ship_class,99,both,test\n"), "");
    assert!(
        message.contains("gives ship class 99, which the scenario's class files do not give"),
        "{message}"
    );
    // A class given twice.
    let message = refused(
        &format!("{PLACEHOLDER_SIDES}ship_class,2,japanese,test\n"),
        "",
    );
    assert!(message.contains("ship_class 2 is given twice"), "{message}");
    // A Japanese ship of class 1.
    let japanese_force = "[[task_force]]\nid = 1\nname = \"TF\"\nside = \"japanese\"\nat_base = 2\nmission = \"escort\"\n";
    let japanese_ship = "[[ship]]\nid = 1\nname = \"S\"\nclass = 1\nside = \"japanese\"\nfuel_t = 100\ntask_force = 1\n";
    let message = refused(
        PLACEHOLDER_SIDES,
        &format!("\n{japanese_force}\n{japanese_ship}"),
    );
    assert!(
            message.contains("ship 1 (S) is Japanese and of ship class 1, which the file of sides gives to the allied side alone"),
            "{message}"
        );
    // The same ship of class 2, both sides', loads.
    parse_with_sides(
        PLACEHOLDER_SIDES,
        &format!(
            "\n{japanese_force}\n{}",
            japanese_ship.replace("class = 1", "class = 2")
        ),
    )
    .expect("a class of both sides loads");
    // A side and a table the file does not know, and an empty note.
    let message = refused(
        &PLACEHOLDER_SIDES.replace("1,allied,test", "1,french,test"),
        "",
    );
    assert!(message.contains("unknown side \"french\""), "{message}");
    let message = refused(&format!("{PLACEHOLDER_SIDES}land_unit,1,both,test\n"), "");
    assert!(message.contains("unknown table \"land_unit\""), "{message}");
    let message = refused(&PLACEHOLDER_SIDES.replace("1,allied,test", "1,allied,"), "");
    assert!(message.contains("the note is empty"), "{message}");
}
