use super::*;

fn contact(body: &str) -> String {
    format!("[[contact]]\nside = \"allied\"\nq = 3\nr = 0\nship_count = 3\n{body}\n")
}

#[test]
fn a_starting_contact_is_held_at_the_given_age() {
    // Age 2: two pulses old.
    let scenario = parse_with(&contact("kind = \"cruiser\"\nage_pulses = 2")).expect("loads");
    let view = scenario.world.side_view(Side::Allied);
    let held: Vec<_> = view.contacts().collect();
    assert_eq!(held.len(), 1);
    assert_eq!(held[0].estimated_hex, Hex::new(3, 0));
    assert_eq!(held[0].estimated_ship_count, 3);
    assert_eq!(held[0].age_pulses, 2);
    // One report and two ageing events, all on day 0.
    assert_eq!(scenario.world.events().len(), 3);

    // The age is optional and defaults to 0.
    let scenario = parse_with(&contact("kind = \"destroyer\"")).expect("loads");
    let view = scenario.world.side_view(Side::Allied);
    assert_eq!(view.contacts().next().expect("held").age_pulses, 0);
    assert_eq!(scenario.world.events().len(), 1);
}

#[test]
fn a_bad_starting_contact_is_a_load_error() {
    let message = error_of(&contact("kind = \"dreadnought\""));
    assert!(
        message.contains("unknown kind \"dreadnought\""),
        "{message}"
    );
    let message = error_of(&contact("kind = \"cruiser\"\nspeed = 3"));
    assert!(message.contains("speed"), "{message}");
    // Age 6 is past the drop age of 5.
    let message = error_of(&contact("kind = \"cruiser\"\nage_pulses = 6"));
    assert!(message.contains("would be dropped"), "{message}");
    let message = error_of(&format!(
        "{}{}",
        contact("kind = \"cruiser\""),
        contact("kind = \"cruiser\"")
    ));
    assert!(message.contains("too close"), "{message}");
}

#[test]
fn a_scenario_for_another_seed_holds_the_same_contacts() {
    let scenario = parse_with(&contact("kind = \"cruiser\"\nage_pulses = 1")).expect("loads");
    let other = scenario.with_seed(77).expect("copies");
    assert_eq!(other.world.events().len(), scenario.world.events().len());
    assert_eq!(other.world.state_hash(), {
        let mut again = scenario.clone();
        again = again.with_seed(77).expect("copies");
        again.world.state_hash()
    });
    assert_eq!(other.world.side_view(Side::Allied).contacts().count(), 1);
}

// -----------------------------------------------------------------
// Opening estimates of the enemy's men ashore (decision 0092, 16).
// -----------------------------------------------------------------

fn estimate(body: &str) -> String {
    format!("\n[[estimate]]\n{body}\n")
}

#[test]
fn an_opening_estimate_is_held_on_day_0_with_no_error_and_no_cause() {
    let scenario =
        parse_with(&estimate("side = \"japanese\"\nq = 3\nr = 0\nmen = 300")).expect("loads");
    let view = scenario.world.side_view(Side::Japanese);
    let held = view.ground_estimate(Hex::new(3, 0)).expect("held");
    assert_eq!((held.men, held.day, held.hex), (300, 0, Hex::new(3, 0)));
    assert_eq!(view.ground_estimates().count(), 1);
    // The other side holds none, and is told nothing.
    let other = scenario.world.side_view(Side::Allied);
    assert_eq!(other.ground_estimates().count(), 0);
    assert!(other.events().is_empty());
    // One event on day 0, citing nothing.
    let events: Vec<_> = scenario.world.events().iter().collect();
    assert_eq!(events.len(), 1);
    assert_eq!(events[0].day, 0);
    assert!(events[0].causes.is_empty());
    assert!(events[0].breakdown.is_none());
    // A second for another hex, or for the other side at the same hex,
    // is allowed; the scenario keeps what it was given.
    let two = format!(
        "{}{}{}",
        estimate("side = \"japanese\"\nq = 3\nr = 0\nmen = 300"),
        estimate("side = \"japanese\"\nq = 4\nr = 0\nmen = 1"),
        estimate("side = \"allied\"\nq = 3\nr = 0\nmen = 0"),
    );
    let scenario = parse_with(&two).expect("loads");
    assert_eq!(
        scenario
            .world
            .side_view(Side::Japanese)
            .ground_estimates()
            .count(),
        2
    );
    assert_eq!(
        scenario
            .world
            .side_view(Side::Allied)
            .ground_estimate(Hex::new(3, 0))
            .expect("held")
            .men,
        0
    );
    // It is kept through a copy for another seed.
    let other = scenario.with_seed(77).expect("copies");
    assert_eq!(
        other
            .world
            .side_view(Side::Japanese)
            .ground_estimates()
            .count(),
        2
    );
}

#[test]
fn a_bad_opening_estimate_is_a_load_error() {
    // Off the map.
    let message = error_of(&estimate("side = \"japanese\"\nq = 99\nr = 0\nmen = 300"));
    assert!(message.contains("off the map"), "{message}");
    // Two for the same side and hex.
    let message = error_of(&format!(
        "{}{}",
        estimate("side = \"japanese\"\nq = 3\nr = 0\nmen = 300"),
        estimate("side = \"japanese\"\nq = 3\nr = 0\nmen = 400"),
    ));
    assert!(message.contains("second estimate"), "{message}");
    // An unknown key.
    let message = error_of(&estimate(
        "side = \"japanese\"\nq = 3\nr = 0\nmen = 300\nerror = 2",
    ));
    assert!(message.contains("error"), "{message}");
    // A float, or a negative number, for men.
    for men in ["300.5", "0.5", "-1", "\"300\""] {
        let message = error_of(&estimate(&format!(
            "side = \"japanese\"\nq = 3\nr = 0\nmen = {men}"
        )));
        assert!(message.contains("men"), "men = {men}: {message}");
    }
    // A key missing.
    for body in [
        "q = 3\nr = 0\nmen = 300",
        "side = \"japanese\"\nr = 0\nmen = 300",
        "side = \"japanese\"\nq = 3\nmen = 300",
        "side = \"japanese\"\nq = 3\nr = 0",
    ] {
        let message = error_of(&estimate(body));
        assert!(message.contains("missing field"), "{body}: {message}");
    }
    // A side that is neither.
    assert!(parse_with(&estimate("side = \"french\"\nq = 3\nr = 0\nmen = 300")).is_err());
}

fn believed_force(body: &str) -> String {
    format!(
        "\n[[believed_force]]\nside = \"allied\"\nname = \"A force\"\nkind = \"carrier\"\n{body}\n"
    )
}

/// Statement 2: a believed force.
#[test]
fn a_believed_force_is_kept_as_written_and_refused_where_it_cannot_be() {
    use sim_core::believed::{LastKnown, Standing};
    let text = format!(
            "{}{}{}",
            believed_force("ships = 6\nstanding = \"present\"\nat_base = 2\nplaceholder = true\nnote = \"Rests on no source.\""),
            believed_force("ships = 8\nstanding = \"unknown\"").replace("A force", "Carriers"),
            believed_force("ships = 1\nstanding = \"absent\"\nq = 7\nr = -1\nday = 2")
                .replace("carrier", "cruiser")
                .replace("A force", "Ruled out"),
        );
    let scenario = parse_with(&text).expect("loads");
    let forces: Vec<_> = scenario.believed_forces(Side::Allied).collect();
    assert_eq!(forces.len(), 3);
    // Present, at a base: kept with that base's hex, before the first day.
    assert_eq!(forces[0].side, Side::Allied);
    assert_eq!(forces[0].enemy(), Side::Japanese);
    assert_eq!(forces[0].name, "A force");
    assert_eq!(forces[0].kind, ShipKind::Carrier);
    assert_eq!(forces[0].ships, 6);
    assert_eq!(forces[0].standing, Standing::Present);
    assert_eq!(
        forces[0].last_known,
        LastKnown::AtBase {
            base: BaseId::new(2),
            hex: Hex::new(5, 0),
            day: 0
        }
    );
    assert!(forces[0].placeholder);
    assert_eq!(forces[0].note.as_deref(), Some("Rests on no source."));
    // Unknown, where not known.
    assert_eq!(forces[1].name, "Carriers");
    assert_eq!(forces[1].ships, 8);
    assert_eq!(forces[1].standing, Standing::Unknown);
    assert_eq!(forces[1].last_known, LastKnown::NotKnown);
    assert!(!forces[1].placeholder);
    assert_eq!(forces[1].note, None);
    // Absent, at a hex on a day.
    assert_eq!(forces[2].kind, ShipKind::Cruiser);
    assert_eq!(forces[2].standing, Standing::Absent);
    assert_eq!(
        forces[2].last_known,
        LastKnown::AtHex {
            hex: Hex::new(7, -1),
            day: 2
        }
    );
    // The other side believes none; they are kept for another seed.
    assert_eq!(scenario.believed_forces(Side::Japanese).count(), 0);
    let other = scenario.with_seed(77).expect("copies");
    assert_eq!(
        other
            .believed_forces(Side::Allied)
            .cloned()
            .collect::<Vec<_>>(),
        scenario
            .believed_forces(Side::Allied)
            .cloned()
            .collect::<Vec<_>>()
    );

    let refused = |body: &str| error_of(&believed_force(body));
    for (body, words) in [
        ("ships = 0\nstanding = \"present\"", "has no ship"),
        (
            "ships = 2\nstanding = \"likely\"",
            "unknown standing \"likely\"",
        ),
        (
            "ships = 2\nstanding = \"present\"\nat_base = 1",
            "the Allied side's own",
        ),
        (
            "ships = 2\nstanding = \"present\"\nq = 99\nr = 0",
            "at 99,0, off the map",
        ),
        (
            "ships = 2\nstanding = \"present\"\nat_base = 2\nq = 5\nr = 0",
            "both \"at_base\" and a hex",
        ),
        (
            "ships = 2\nstanding = \"present\"\nat_base = 2\nday = 3",
            "on day 3, after the scenario's last, day 2",
        ),
        (
            "ships = 2\nstanding = \"present\"\nspeed = 3",
            "unknown field `speed`",
        ),
        (
            "ships = 2\nstanding = \"present\"\nplaceholder = true",
            "no note of what it rests on",
        ),
        (
            "ships = 2\nstanding = \"present\"\nday = 1",
            "gives a day and no place",
        ),
        (
            "ships = 2\nstanding = \"present\"\nq = 3",
            "a hex takes both",
        ),
        (
            "ships = 2\nstanding = \"present\"\nat_base = 9",
            "not a base of the scenario",
        ),
    ] {
        let message = refused(body);
        assert!(message.contains(words), "{body}: {message}");
    }
    // An unknown kind.
    let message = error_of(
        &believed_force("ships = 2\nstanding = \"present\"").replace("carrier", "dreadnought"),
    );
    assert!(
        message.contains("unknown kind \"dreadnought\""),
        "{message}"
    );
    // A base off the believing side's map.
    let hidden = HEAD.replacen("q = 5\nr = 0\n", "q = 5\nr = 0\non_map = false\n", 1);
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../data/scenarios/test.toml");
    let message = Scenario::parse(
        &format!(
            "{hidden}{}",
            believed_force("ships = 2\nstanding = \"present\"\nat_base = 2")
        ),
        &path,
    )
    .expect_err("refused")
    .message;
    assert!(
        message.contains("not on the Allied side's map"),
        "{message}"
    );
    // A side it does not know.
    assert!(parse_with(
        &believed_force("ships = 2\nstanding = \"present\"").replace("\"allied\"", "\"french\"")
    )
    .is_err());
}

fn believed_intent(body: &str) -> String {
    format!("\n[[believed_intent]]\nside = \"allied\"\n{body}\n")
}

/// Statement 3: a believed intent.
#[test]
fn a_believed_intent_is_checked_as_the_enemys_command_on_the_believing_sides_map() {
    use sim_core::command::IntentKind;
    let scenario = parse_with(&believed_intent(
        "intent_kind = \"hold\"\nbase = 2\nplaceholder = true\nnote = \"Rests on no source.\"",
    ))
    .expect("loads");
    let intents: Vec<_> = scenario.believed_intents(Side::Allied).collect();
    assert_eq!(intents.len(), 1);
    assert_eq!(intents[0].side, Side::Allied);
    assert_eq!(intents[0].enemy(), Side::Japanese);
    assert_eq!(
        intents[0].kind,
        IntentKind::Hold {
            base: BaseId::new(2)
        }
    );
    assert!(intents[0].placeholder);
    assert_eq!(scenario.believed_intents(Side::Japanese).count(), 0);
    // It is no command: neither side's command state holds an intent.
    assert!(scenario.world.command_state(Side::Japanese).is_empty());
    assert!(scenario.scripted_commands().is_empty());

    // An intent naming a base not on the Allied side's map.
    let hidden = HEAD.replacen("q = 5\nr = 0\n", "q = 5\nr = 0\non_map = false\n", 1);
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../data/scenarios/test.toml");
    let message = Scenario::parse(
        &format!(
            "{hidden}{}",
            believed_intent("intent_kind = \"hold\"\nbase = 2")
        ),
        &path,
    )
    .expect_err("refused")
    .message;
    assert!(
        message.contains("names base 2, which is not on the Allied side's map"),
        "{message}"
    );
    // A kind only the staff may give.
    let message = error_of(&believed_intent(
        "intent_kind = \"search_waters\"\nbase = 2",
    ));
    assert!(
        message.contains("could not be given to the Japanese side"),
        "{message}"
    );
    // An unknown key, and a key the kind does not take.
    let message = error_of(&believed_intent(
        "intent_kind = \"hold\"\nbase = 2\nday = 1",
    ));
    assert!(message.contains("unknown field `day`"), "{message}");
    let message = error_of(&believed_intent(
        "intent_kind = \"hold\"\nbase = 2\ntarget = 1",
    ));
    assert!(message.contains("does not take \"target\""), "{message}");
    // An intent the enemy could not be given: a supply intent for a base
    // that is not his, a neutralise of his own base, no kind.
    let message = error_of(&believed_intent(
        "intent_kind = \"keep_supplied\"\nbase = 1\nfloor_days = 3",
    ));
    assert!(
        message.contains("could not be given to the Japanese side"),
        "{message}"
    );
    let message = error_of(&believed_intent("intent_kind = \"neutralise\"\ntarget = 2"));
    assert!(
        message.contains("could not be given to the Japanese side"),
        "{message}"
    );
    let message = error_of(&believed_intent("base = 2"));
    assert!(message.contains("needs \"intent_kind\""), "{message}");
    // A hold of the believing side's own base, on both maps, loads.
    parse_with(&believed_intent("intent_kind = \"hold\"\nbase = 1")).expect("loads");
}

/// The architect's ruling on reading 10 of decision 0173's "As built": a
/// believed intent may not name a force of the enemy's, and the check of
/// the kinds left reads nothing of the enemy's units.
#[test]
fn a_believed_intent_names_no_enemy_force_and_reads_none() {
    for body in [
        "intent_kind = \"stand_at\"\ntask_force = 1\nhex = [3, 0]",
        "intent_kind = \"operate_carriers\"\ntask_force = 1\nhex = [3, 0]",
        "intent_kind = \"ready_landing\"\ntask_force = 1\nhex = [0, 0]",
        "intent_kind = \"bombard\"\ntask_force = 1\ntarget = 1",
        // Refused by the kind alone, whatever the keys.
        "intent_kind = \"bombard\"",
    ] {
        let message = error_of(&believed_intent(body));
        assert!(
            message.contains("a side cannot name a force of the enemy's"),
            "{body}: {message}"
        );
    }
    // Two scenarios that differ only in the enemy's forces: the second
    // has one more Japanese task force, with a ship. Both load the same
    // believed intents, of each kind that names a base, a hex or an
    // island.
    let intents = format!(
        "{}{}{}{}{}{}{}{}{}{}",
        believed_intent("intent_kind = \"cover\"\nbase = 2"),
        believed_intent("intent_kind = \"strike_ships\"\nbase = 2"),
        believed_intent("intent_kind = \"build_up\"\nbase = 2\nwants = [\"airfield\"]"),
        believed_intent("intent_kind = \"hold\"\nbase = 2"),
        believed_intent("intent_kind = \"hold\"\nbase = 1"),
        believed_intent("intent_kind = \"keep_supplied\"\nbase = 2\nfloor_days = 3"),
        believed_intent("intent_kind = \"neutralise\"\ntarget = 1"),
        believed_intent("intent_kind = \"raid\"\nbase = 2\ntarget = 1"),
        believed_intent("intent_kind = \"deny\"\nhex = [3, 0]\ndistance = 2"),
        believed_intent("intent_kind = \"take_place\"\nhex = [0, 0]"),
    );
    let enemy_force = "\n[[task_force]]\nid = 7\nname = \"Enemy TF\"\nside = \"japanese\"\nat_base = 2\nmission = \"escort\"\n\n[[ship]]\nid = 7\nname = \"Enemy ship\"\nclass = 1\nside = \"japanese\"\nfuel_t = 100\ntask_force = 7\n";
    let without = parse_with(&intents).expect("loads without the force");
    let with = parse_with(&format!("{intents}{enemy_force}")).expect("loads with the force");
    let kept = |scenario: &Scenario| -> Vec<BelievedIntent> {
        scenario.believed_intents(Side::Allied).cloned().collect()
    };
    assert_eq!(kept(&without).len(), 10);
    assert_eq!(kept(&without), kept(&with));
}

/// Statement 5, in part: entries and a file of sides change no game.
/// The world loaded with them is the world loaded without them, hash and
/// log alike.
#[test]
fn believed_entries_and_a_file_of_sides_change_nothing_in_the_world() {
    let plain = parse_with("").expect("loads");
    let with = parse_with_sides(
        PLACEHOLDER_SIDES,
        &format!(
            "{}{}",
            believed_force("ships = 6\nstanding = \"present\"\nat_base = 2"),
            believed_intent("intent_kind = \"hold\"\nbase = 2")
        ),
    )
    .expect("loads");
    assert_eq!(plain.world.state_hash(), with.world.state_hash());
    assert_eq!(plain.world.events().len(), with.world.events().len());
}
