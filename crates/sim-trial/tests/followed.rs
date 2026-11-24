//! A world made for a dry run keeps what an order to destroy a force keeps,
//! with the contact's number made the made world's (decision 0220, rule 14,
//! statement 49): a made world numbers its contacts anew, from 1, in the
//! order of the picture's contacts, so the contact followed is found again by
//! its place in that order.
//!
//! The game is scenario 133's game A with a second contact put first, at
//! (2,0), a destroyer of age 4, so that the cruisers' contact is contact 2 in
//! the live game, and the order follows it. By the end of day 1 the decoy has
//! aged out (age 5 drops a contact) and the cruisers' contact, age 3, is the
//! only one held: it is contact 1 of a made world. By the end of day 2 it is
//! 6 pulses old and dropped as well: the made world holds none, and the
//! order's number names no contact of it.

use std::path::Path;

use sim_core::command::Followed;
use sim_core::ids::IntentId;
use sim_core::state::Side;
use sim_data::{LoadOptions, Scenario};
use sim_trial::{pictures, Believed, Trial};

const ROOT: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../..");

fn scenario() -> Scenario {
    let path = Path::new(ROOT).join("data/scenarios/a-force-followed/133_a_followed.toml");
    let text = std::fs::read_to_string(&path).expect("game A reads");
    let text = text
        .replacen(
            "air_combat = \"../../reference/air_combat.toml\"\n",
            "air_combat = \"../../reference/air_combat.toml\"\ntrial = \"../../reference/trial_placeholder_scenario_94.toml\"\n",
            1,
        )
        .replacen(
            "# The contact the Allied side starts with: contact 1.\n[[contact]]",
            "[[contact]]\nside = \"allied\"\nq = 2\nr = 0\nship_count = 1\nkind = \"destroyer\"\nage_pulses = 4\n\n[[contact]]",
            1,
        )
        .replacen("contact = 1\ndistance = 2", "contact = 2\ndistance = 2", 1);
    Scenario::from_text_with(&text, &path, &LoadOptions::frozen_step1(Path::new(ROOT)))
        .expect("the game loads")
}

fn followed(view: &sim_core::view::SideView<'_>) -> Followed {
    *view
        .command_state()
        .followed
        .get(&IntentId::new(1))
        .expect("the order keeps a report")
}

fn made(scenario: &Scenario) -> Vec<Followed> {
    let view = scenario.world.side_view(Side::Allied);
    pictures(
        &view,
        &Believed::default(),
        scenario.trial_figures().expect("the trial's figures"),
        Trial {
            seed: 1942,
            trial: 1,
            worlds: 3,
        },
    )
    .expect("the pictures are made")
    .iter()
    .map(|picture| {
        *picture
            .picture
            .command
            .get(&Side::Allied)
            .expect("the side's command state")
            .followed
            .get(&IntentId::new(1))
            .expect("the made world's order keeps a report")
    })
    .collect()
}

#[test]
fn the_contact_followed_is_found_by_its_place_in_the_made_worlds_numbering() {
    let mut scenario = scenario();
    scenario.next_day().expect("day 1 resolves");
    let live = followed(&scenario.world.side_view(Side::Allied));
    assert_eq!(live.contact, 2, "the live order follows contact 2");
    let in_each_world = made(&scenario);
    assert_eq!(in_each_world.len(), 3);
    for kept in in_each_world {
        // The same report, the number made the world's.
        assert_eq!(kept.contact, 1, "the one contact held is contact 1 there");
        assert_eq!(
            Followed {
                contact: live.contact,
                ..kept
            },
            live,
            "what the order keeps is what the live order keeps"
        );
    }
}

#[test]
fn a_contact_the_picture_no_longer_holds_is_named_by_no_number_of_the_made_world() {
    let mut scenario = scenario();
    scenario.next_day().expect("day 1 resolves");
    scenario.next_day().expect("day 2 resolves");
    let live = followed(&scenario.world.side_view(Side::Allied));
    assert_eq!(live.contact, 2);
    for kept in made(&scenario) {
        assert_eq!(kept.contact, 0, "no contact of a made world is numbered 0");
        assert_eq!(
            Followed {
                contact: live.contact,
                ..kept
            },
            live,
            "the order keeps its report and is read from that"
        );
    }
}
