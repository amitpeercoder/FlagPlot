//! A side's name for an event's kind is the engine's (decision 0235, ruling
//! 10). The old way of making it, from the `Debug` text of the variant, is
//! kept here for the test alone: the two are equal over every event each side
//! is given in three scenarios, which between them hold the cut forms of a
//! surface fight (04), of the air (05) and of the boats (128).

use std::collections::BTreeSet;

use crate::support::{load, play};
use sim_core::side_events::SideEventKind;
use sim_core::state::Side;

fn old_kind_name(kind: &SideEventKind) -> String {
    let debug = format!("{kind:?}");
    let name: String = if let SideEventKind::Whole(inner) = kind {
        format!("{inner:?}")
    } else {
        debug
    }
    .chars()
    .take_while(|c| c.is_ascii_alphanumeric())
    .collect();
    let mut out = String::new();
    for (i, c) in name.chars().enumerate() {
        if c.is_ascii_uppercase() {
            if i > 0 {
                out.push('_');
            }
            out.push(c.to_ascii_lowercase());
        } else {
            out.push(c);
        }
    }
    out
}

fn check(name: &str, seed: u64) -> (usize, BTreeSet<String>) {
    let mut scenario = load(name, seed);
    let mut events = 0;
    let mut cut: BTreeSet<String> = BTreeSet::new();
    play(&mut scenario, |s| {
        for side in Side::ALL {
            for event in s.world.side_view(side).events() {
                let word = event.kind.tag();
                assert_eq!(word, old_kind_name(&event.kind), "{name}");
                events += 1;
                if !matches!(event.kind, SideEventKind::Whole(_)) {
                    cut.insert(word.to_string());
                }
            }
        }
    });
    (events, cut)
}

#[test]
fn the_engines_word_for_a_kind_is_the_one_the_agent_made() {
    let mut cut_forms: BTreeSet<String> = BTreeSet::new();
    // The boat's scenario is one night with one chance an hour, so several
    // seeds are played for the boat to fire and be hunted.
    let games: Vec<(&str, u64)> = std::iter::once(("04_savo_island.toml", 1))
        .chain(std::iter::once(("05_eastern_solomons.toml", 1)))
        .chain((1..=60).map(|seed| ("128_submarines_against_warships.toml", seed)))
        .collect();
    for (name, seed) in games {
        let (events, cut) = check(name, seed);
        assert!(events > 0, "{name} gives no events");
        cut_forms.extend(cut);
    }
    println!("cut forms met: {cut_forms:?}");
    for word in [
        "own_submarine_fired",
        "salvo_from_submarine",
        "submarine_hunted_by",
    ] {
        assert!(cut_forms.contains(word), "the boats' {word} was not met");
    }
    assert!(!cut_forms.is_empty(), "no cut form was met");
}
