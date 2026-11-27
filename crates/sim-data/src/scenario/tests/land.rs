use super::*;

#[test]
fn land_units_are_built_ashore_or_put_aboard_in_unit_order() {
    // Listed out of order: unit 2 is put aboard after unit 1 all the same.
    let extra = format!(
        "{}{}{}",
        unit_aboard(2, 400, 1),
        unit_ashore(3, 500, 10, 0),
        unit_aboard(1, 600, 1)
    );
    let scenario = landing(true, &extra).expect("loads");
    let truth = scenario.world.umpire();
    // The live figures load; no value of them is asserted, so a tuning
    // round changes nothing here (the architect's ruling of Nov 15, 2026).
    assert!(
        scenario.world.ref_data().ground.is_some(),
        "the figures load"
    );
    assert_eq!(truth.land_units.len(), 3);
    let first = &truth.land_units[&LandUnitId::new(1)];
    assert_eq!((first.men_ashore, first.hex), (0, None));
    assert_eq!(first.side, Side::Allied);
    assert_eq!(first.name, "Unit 1");
    let third = &truth.land_units[&LandUnitId::new(3)];
    assert_eq!((third.men_ashore, third.hex), (500, Some(Hex::new(10, 0))));
    let troops = |ship: u32| truth.ships[&ShipId::new(ship)].troops;
    let aboard = |unit: u32, men: u32| {
        Some(sim_core::state::Troops {
            unit: LandUnitId::new(unit),
            men,
        })
    };
    assert_eq!(troops(1), aboard(1, 600));
    assert_eq!(troops(2), aboard(2, 400));
    assert_eq!(troops(3), None);
    assert_eq!(troops(4), None);
    assert!(truth.bases.values().all(|base| base.on_map));
}

#[test]
fn a_unit_larger_than_a_ship_is_spread_over_the_ships_in_ship_order() {
    let scenario = landing(true, &unit_aboard(1, 900, 1)).expect("loads");
    let truth = scenario.world.umpire();
    assert_eq!(
        truth.ships[&ShipId::new(1)].troops.map(|t| t.men),
        Some(600)
    );
    assert_eq!(
        truth.ships[&ShipId::new(2)].troops.map(|t| t.men),
        Some(300)
    );
}

#[test]
fn a_land_unit_without_a_ground_file_is_refused() {
    let message = landing_error(false, &unit_ashore(1, 100, 10, 0));
    assert!(message.contains("land unit"), "{message}");
    assert!(message.contains("ground"), "{message}");
    // A scenario with no land unit needs no ground file, and has none.
    let scenario = landing(false, "").expect("loads");
    assert!(scenario.world.ref_data().ground.is_none());
    assert!(scenario.world.umpire().land_units.is_empty());
    // The key names a file that must be there.
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../data/scenarios/test.toml");
    let text = "name = \"T\"\nground = \"../reference/nothing.toml\"\n";
    assert!(Scenario::parse(text, &path).is_err());
}

#[test]
fn a_land_unit_that_does_not_fit_aboard_is_refused() {
    // Two transports hold 1,200 men.
    let message = landing_error(true, &unit_aboard(1, 1300, 1));
    assert!(message.contains("land_unit:1"), "{message}");
    assert!(message.contains("berths for 1200"), "{message}");
    // Unit 1 takes ship 1, unit 2 takes ship 2, and unit 3 has no room.
    let extra = format!(
        "{}{}{}",
        unit_aboard(1, 600, 1),
        unit_aboard(2, 600, 1),
        unit_aboard(3, 1, 1)
    );
    let message = landing_error(true, &extra);
    assert!(message.contains("land_unit:3"), "{message}");
    // A force whose ships have no troop capacity holds no man.
    let message = landing_error(true, &unit_aboard(1, 1, 3));
    assert!(message.contains("land_unit:1"), "{message}");
}

#[test]
fn a_land_unit_needs_exactly_one_place() {
    let both = "\n[[land_unit]]\nid = 1\nname = \"U\"\nside = \"allied\"\nmen = 5\nq = 10\nr = 0\ntask_force = 1\n";
    let neither = "\n[[land_unit]]\nid = 1\nname = \"U\"\nside = \"allied\"\nmen = 5\n";
    let half = "\n[[land_unit]]\nid = 1\nname = \"U\"\nside = \"allied\"\nmen = 5\nq = 10\n";
    for extra in [both, neither, half] {
        let message = landing_error(true, extra);
        assert!(message.contains("land_unit:1"), "{message}");
        assert!(message.contains("exactly one of"), "{message}");
    }
}

#[test]
fn a_land_unit_with_no_men_a_bad_number_or_a_duplicate_is_refused() {
    let message = landing_error(true, &unit_ashore(1, 0, 10, 0));
    assert!(message.contains("land_unit:1 has no men"), "{message}");
    let message = landing_error(true, &unit_ashore(0, 5, 10, 0));
    assert!(message.contains("land_unit:0"), "{message}");
    let message = landing_error(true, &unit_ashore(MADE_IDS_FLOOR, 5, 10, 0));
    assert!(message.contains(&MADE_IDS_FLOOR.to_string()), "{message}");
    let extra = format!("{}{}", unit_ashore(1, 5, 10, 0), unit_ashore(1, 6, 8, 0));
    let message = landing_error(true, &extra);
    assert!(message.contains("land_unit:1 appears twice"), "{message}");
}

#[test]
fn a_land_unit_ashore_must_be_on_the_map_on_coast_or_land() {
    let message = landing_error(true, &unit_ashore(1, 5, 50, 50));
    assert!(message.contains("land_unit:1"), "{message}");
    assert!(message.contains("off the map"), "{message}");
    // (5,0) is deep sea.
    let message = landing_error(true, &unit_ashore(1, 5, 5, 0));
    assert!(message.contains("not coast or land"), "{message}");
    assert!(landing(true, &unit_ashore(1, 5, 8, 0)).is_ok());
}

#[test]
fn a_land_unit_aboard_needs_a_force_of_its_own_side() {
    let message = landing_error(true, &unit_aboard(1, 5, 9));
    assert!(message.contains("land_unit:1"), "{message}");
    assert!(message.contains("unknown task force 9"), "{message}");
    // Task force 3 is Japanese.
    let message = landing_error(true, &unit_aboard(1, 5, 3));
    assert!(message.contains("land_unit:1"), "{message}");
    assert!(message.contains("Allied but its task force"), "{message}");
}

#[test]
fn a_land_unit_has_no_unknown_keys() {
    let extra = format!("{}rank = 3\n", unit_ashore(1, 5, 10, 0));
    assert!(landing_error(true, &extra).contains("rank"));
}

#[test]
fn the_shipped_landing_scenario_loads_on_the_live_and_the_frozen_figures() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let path = root.join("data/scenarios/21_landing.toml");
    let live = Scenario::load(&path).expect("loads on the live figures");
    let frozen = Scenario::load_with(&path, &LoadOptions::frozen_step1(&root))
        .expect("loads on the frozen figures");
    for scenario in [&live, &frozen] {
        let truth = scenario.world.umpire();
        assert_eq!(scenario.days, 7);
        assert_eq!(truth.land_units.len(), 3);
        let aboard = |ship: u32| truth.ships[&ShipId::new(ship)].troops;
        assert_eq!(
            aboard(1),
            Some(sim_core::state::Troops {
                unit: LandUnitId::new(1),
                men: 600
            })
        );
        assert_eq!(
            aboard(2),
            Some(sim_core::state::Troops {
                unit: LandUnitId::new(2),
                men: 400
            })
        );
        assert_eq!(aboard(3), None);
        let third = &truth.land_units[&LandUnitId::new(3)];
        assert_eq!((third.men_ashore, third.hex), (500, Some(Hex::new(10, 0))));
        for unit in [1, 2] {
            let unit = &truth.land_units[&LandUnitId::new(unit)];
            assert_eq!((unit.men_ashore, unit.hex), (0, None));
        }
        let ground = scenario.world.ref_data().ground.expect("the figures load");
        assert_eq!(ground.daily_supply_t_per_man, Fixed::from_ratio(1, 200));
        let load = &scenario.scripted_orders()[&1];
        assert!(load.iter().any(|(_, request)| matches!(
            &request.kind,
            OrderKind::TaskForce {
                order: TaskForceOrder::LoadUnit { cargo: None, .. },
                ..
            }
        )));
    }
    assert!(live
        .data_files()
        .iter()
        .any(|file| file.ends_with("ground.toml")));
    assert!(frozen
        .data_files()
        .iter()
        .any(|file| file.to_string_lossy().contains("step1/ground.toml")));
}

// -----------------------------------------------------------------
// Experience, training and preparing a landing (decisions 0095, 0097).
// -----------------------------------------------------------------

fn unit_with(id: u32, keys: &str) -> String {
    format!(
            "\n[[land_unit]]\nid = {id}\nname = \"Unit {id}\"\nside = \"allied\"\nmen = 100\nq = 0\nr = 0\n{keys}\n"
        )
}

#[test]
fn a_land_unit_takes_experience_and_counts() {
    let scenario = landing(
        true,
        &format!(
            "{}{}",
            unit_ashore(1, 100, 0, 0),
            unit_with(2, "experience = 4\ntraining_days = 29\nfight_days = 9")
        ),
    );
    // Unit 1 is built the usual way; unit 2 gives all three keys.
    let scenario = scenario.expect("loads");
    let truth = scenario.world.umpire();
    let one = &truth.land_units[&LandUnitId::new(1)];
    assert_eq!(
        (one.experience, one.training_days, one.fight_days),
        (2, 0, 0)
    );
    let two = &truth.land_units[&LandUnitId::new(2)];
    assert_eq!(
        (two.experience, two.training_days, two.fight_days),
        (4, 29, 9)
    );
}

#[test]
fn a_land_unit_takes_its_nights_of_hunger() {
    // Decision 0205, 7: a scenario may give the count a unit starts
    // with; without it the count is nothing.
    let scenario = landing(
        true,
        &format!(
            "{}{}",
            unit_ashore(1, 100, 0, 0),
            unit_with(2, "hunger_nights = 14")
        ),
    )
    .expect("loads");
    let truth = scenario.world.umpire();
    assert_eq!(truth.land_units[&LandUnitId::new(1)].hunger_nights, 0);
    assert_eq!(truth.land_units[&LandUnitId::new(2)].hunger_nights, 14);
    let message = landing_error(true, &unit_with(1, "hunger_nights = -1"));
    assert!(message.contains("hunger_nights"), "{message}");
}

#[test]
fn a_land_unit_takes_fieldworks_up_to_the_limit() {
    let scenario = landing(
        true,
        &format!(
            "{}{}{}{}",
            unit_ashore(1, 100, 0, 0),
            unit_with(2, "fieldworks = 30"),
            unit_with(3, "fieldworks = \"12.5\""),
            unit_with(4, "fieldworks = 0"),
        ),
    )
    .expect("loads");
    let truth = scenario.world.umpire();
    let fieldworks = |unit: u32| truth.land_units[&LandUnitId::new(unit)].fieldworks;
    assert_eq!(fieldworks(1), Fixed::ZERO);
    assert_eq!(fieldworks(2), Fixed::from_int(30));
    assert_eq!(fieldworks(3), Fixed::from_ratio(25, 2));
    assert_eq!(fieldworks(4), Fixed::ZERO);
}

#[test]
fn fieldworks_above_the_limit_or_below_nothing_are_refused() {
    for keys in ["fieldworks = 31", "fieldworks = -1", "fieldworks = 3.5"] {
        let message = landing_error(true, &unit_with(1, keys));
        assert!(
            message.contains("land_unit:1") || message.contains("fieldworks"),
            "{keys}: {message}"
        );
    }
    let message = landing_error(true, &unit_with(1, "fieldworks = 31"));
    assert!(message.contains("fieldworks_limit"), "{message}");
    let message = landing_error(true, &unit_with(1, "fieldworks = -1"));
    assert!(message.contains("fieldworks"), "{message}");
}

#[test]
fn fieldworks_are_refused_for_a_working_unit_an_attacker_and_a_unit_aboard() {
    let message = landing_error(true, &unit_with(1, "working = true\nfieldworks = 10"));
    assert!(message.contains("land_unit:1"), "{message}");
    assert!(message.contains("fieldworks"), "{message}");
    let message = landing_error(true, &unit_with(1, "posture = \"attack\"\nfieldworks = 10"));
    assert!(message.contains("land_unit:1"), "{message}");
    assert!(message.contains("fieldworks"), "{message}");
    let aboard = format!("{}fieldworks = 10\n", unit_aboard(1, 100, 1));
    let message = landing_error(true, &aboard);
    assert!(message.contains("land_unit:1"), "{message}");
    assert!(message.contains("fieldworks"), "{message}");
    // Zero fieldworks are no fieldworks: allowed anywhere.
    landing(true, &unit_with(1, "working = true\nfieldworks = 0")).expect("loads");
    landing(true, &unit_with(1, "posture = \"attack\"\nfieldworks = 0")).expect("loads");
}

#[test]
fn a_land_unit_takes_skills_and_any_not_given_is_2() {
    let scenario = landing(
        true,
        &format!(
            "{}{}",
            unit_ashore(1, 100, 0, 0),
            unit_with(
                2,
                "skills = { assault = 4, holding = 3, weight = 0, mountain = 4, clear = 1 }"
            )
        ),
    )
    .expect("loads");
    let truth = scenario.world.umpire();
    assert_eq!(
        truth.land_units[&LandUnitId::new(1)].skills,
        Skills::STANDARD_ALL
    );
    assert_eq!(
        truth.land_units[&LandUnitId::new(2)].skills,
        Skills {
            assault: 4,
            holding: 3,
            weight: 0,
            clear: 1,
            jungle: 2,
            mountain: 4,
            atoll: 2,
            works: 2,
        }
    );
}

#[test]
fn a_skill_is_whole_from_0_to_4_and_named() {
    let message = landing_error(true, &unit_with(1, "skills = { weight = 5 }"));
    assert!(message.contains("weight skill of 5"), "{message}");
    for keys in [
        "skills = { assault = -1 }",
        "skills = { holding = 1.5 }",
        "skills = { mountain = \"3\" }",
        "skills = { swimming = 3 }",
    ] {
        assert!(landing(true, &unit_with(1, keys)).is_err(), "{keys}");
    }
    assert!(landing(true, &unit_with(1, "skills = { atoll = 0, jungle = 4 }")).is_ok());
    assert!(landing(true, &unit_with(1, "skills = {}")).is_ok());
}

#[test]
fn experience_and_the_counts_are_whole_and_in_range() {
    let message = landing_error(true, &unit_with(1, "experience = 5"));
    assert!(message.contains("experience 5"), "{message}");
    let message = landing_error(true, &unit_with(1, "experience = -1"));
    assert!(!message.is_empty());
    let message = landing_error(true, &unit_with(1, "experience = 1.5"));
    assert!(!message.is_empty());
    // The live ground figures: 30 days of training and 10 of fighting
    // for a step.
    let message = landing_error(true, &unit_with(1, "training_days = 30"));
    assert!(message.contains("training"), "{message}");
    let message = landing_error(true, &unit_with(1, "fight_days = 10"));
    assert!(message.contains("fighting"), "{message}");
    let message = landing_error(true, &unit_with(1, "fight_days = -1"));
    assert!(!message.is_empty());
    let message = landing_error(true, &unit_with(1, "training_days = 2.5"));
    assert!(!message.is_empty());
    assert!(landing(true, &unit_with(1, "training_days = 29\nfight_days = 9")).is_ok());
}

#[test]
fn a_working_unit_is_marked_works_and_may_not_attack() {
    let scenario = landing(
        true,
        &format!(
            "{}{}",
            unit_ashore(1, 100, 0, 0),
            unit_with(2, "working = true\nskills = { works = 4 }")
        ),
    )
    .expect("loads");
    let truth = scenario.world.umpire();
    let fighter = &truth.land_units[&LandUnitId::new(1)];
    assert!(!fighter.working);
    assert_eq!(fighter.activity, Activity::Ready);
    assert_eq!(fighter.skills.works, 2);
    let worker = &truth.land_units[&LandUnitId::new(2)];
    assert!(worker.working);
    assert_eq!(worker.activity, Activity::Work(None));
    assert_eq!(worker.skills.works, 4);
    assert!(landing(true, &unit_with(1, "working = false\nposture = \"attack\"")).is_ok());
    assert!(landing(true, &unit_with(1, "working = true\nposture = \"defend\"")).is_ok());
    let message = landing_error(true, &unit_with(1, "working = true\nposture = \"attack\""));
    assert!(
        message.contains("working unit") && message.contains("attack"),
        "{message}"
    );
    // The works skill is whole, 0 to 4.
    let message = landing_error(true, &unit_with(1, "skills = { works = 5 }"));
    assert!(message.contains("works skill of 5"), "{message}");
    assert!(landing(true, &unit_with(1, "skills = { works = 0 }")).is_ok());
    assert!(landing(true, &unit_with(1, "skills = { works = -1 }")).is_err());
    assert!(landing(true, &unit_with(1, "skills = { works = 1.5 }")).is_err());
    assert!(landing(true, &unit_with(1, "working = \"yes\"")).is_err());
}

#[test]
fn a_scenario_names_a_land_links_file_that_adds_links_and_is_a_data_file() {
    let dir = std::env::temp_dir().join(format!("flagplot-land-links-{}", std::process::id()));
    fs::create_dir_all(&dir).expect("make the temp dir");
    let map = dir.join("map.toml");
    fs::write(
        &map,
        "placeholder = true\n[[hex]]\nq = 0\nr = 0\nkind = \"coast\"\nport_size = 3\n\
             [[hex]]\nq = 1\nr = 0\nkind = \"coast\"\n\
             [[hex]]\nq = 5\nr = 0\nkind = \"coast\"\n",
    )
    .expect("write the map");
    let links = dir.join("links.csv");
    fs::write(
        &links,
        "# placeholder\nq1,r1,q2,r2,landmass,reason\n0,0,1,0,Placeholder island,invented\n",
    )
    .expect("write the links");
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../data/scenarios/test.toml");
    let text = HEAD.replace(
        "map = \"../map/placeholder_search.toml\"",
        &format!(
            "map = {:?}\nland_links = {:?}",
            map.to_string_lossy(),
            links.to_string_lossy()
        ),
    );
    let scenario = Scenario::parse(&text, &path).expect("loads");
    assert!(scenario
        .world
        .map()
        .has_land_link(Hex::new(0, 0), Hex::new(1, 0)));
    assert_eq!(scenario.data_files()[0], map);
    assert_eq!(scenario.data_files()[1], links);
    // A scenario without the key has no links.
    let plain = Scenario::parse(HEAD, &path).expect("loads");
    assert_eq!(plain.world.map().land_links().count(), 0);
    // A bad links file is the scenario's error, naming the file.
    fs::write(&links, "q1,r1,q2,r2,landmass,reason\n0,0,5,5,a,b\n").expect("rewrite");
    let error = Scenario::parse(&text, &path)
        .expect_err("refused")
        .to_string();
    assert!(
        error.contains("links.csv") && error.contains("not neighbours"),
        "{error}"
    );
    fs::remove_dir_all(&dir).ok();
}
