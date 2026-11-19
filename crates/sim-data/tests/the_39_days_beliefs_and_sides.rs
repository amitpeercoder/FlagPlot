//! Decision 0173, statement 4 of "What the tests are to show": the full slice
//! and each arrangement of stage 3's proof load with their file of sides,
//! every class and type they give has a row, and no ship or group is of a
//! class or type given to the other side alone; each holds the five entries
//! of item 11 as written, each marked placeholder. On the live figures and on
//! the frozen step 1 figures, with which the agent's tests load the slice.

use std::path::Path;

use sim_core::believed::{LastKnown, Standing};
use sim_core::command::IntentKind;
use sim_core::hex::Hex;
use sim_core::ids::BaseId;
use sim_core::refdata::ShipKind;
use sim_core::state::Side;
use sim_data::{LoadOptions, Scenario};

const FILES: [&str; 4] = [
    "data/scenarios/07_full_slice.toml",
    "data/scenarios/stage-3-proof/07a_slice.toml",
    "data/scenarios/stage-3-proof/07b_slice.toml",
    "data/scenarios/stage-3-proof/07c_slice.toml",
];

fn check(scenario: &Scenario, name: &str) {
    let ref_data = scenario.world.ref_data();
    let sides = ref_data
        .class_sides
        .as_ref()
        .unwrap_or_else(|| panic!("{name} names a file of sides"));
    // Whole: every class and type has a row, and no row is extra (the
    // loader refuses either; this says it was read).
    assert!(ref_data.ship_classes.keys().eq(sides.ship_classes.keys()));
    assert!(ref_data
        .aircraft_types
        .keys()
        .eq(sides.aircraft_types.keys()));
    // True: no ship and no air group of a class or type of the other side
    // alone.
    let truth = scenario.world.umpire();
    for ship in truth.ships.values() {
        assert!(
            sides.ship_classes[&ship.class].used_by(ship.side),
            "{name}: ship {} ({})",
            ship.id.raw(),
            ship.name
        );
    }
    for group in truth.air_groups.values() {
        assert!(
            sides.aircraft_types[&group.aircraft_type].used_by(group.side),
            "{name}: air group {} ({})",
            group.id.raw(),
            group.name
        );
    }
    // The data file is named by the scenario, last.
    assert!(scenario
        .data_files()
        .last()
        .expect("files")
        .ends_with("class_sides_full_slice.csv"));

    // The five entries of item 11, as written.
    let allied: Vec<_> = scenario.believed_forces(Side::Allied).collect();
    assert_eq!(allied.len(), 2, "{name}");
    assert_eq!(allied[0].kind, ShipKind::Cruiser);
    assert_eq!(allied[0].ships, 6);
    assert_eq!(allied[0].standing, Standing::Present);
    assert_eq!(
        allied[0].last_known,
        LastKnown::AtBase {
            base: BaseId::new(2),
            hex: truth.bases[&BaseId::new(2)].hex,
            day: 0
        }
    );
    assert_eq!(truth.bases[&BaseId::new(2)].name, "Rabaul");
    assert_eq!(allied[1].kind, ShipKind::Carrier);
    assert_eq!(allied[1].ships, 8);
    assert_eq!(allied[1].standing, Standing::Unknown);
    assert_eq!(allied[1].last_known, LastKnown::NotKnown);
    let intents: Vec<_> = scenario.believed_intents(Side::Allied).collect();
    assert_eq!(
        intents.iter().map(|i| i.kind).collect::<Vec<_>>(),
        vec![
            IntentKind::Hold {
                base: BaseId::new(1)
            },
            IntentKind::Hold {
                base: BaseId::new(4)
            },
        ],
        "{name}"
    );
    assert_eq!(truth.bases[&BaseId::new(1)].name, "Lunga");
    assert_eq!(truth.bases[&BaseId::new(4)].name, "Tulagi");
    assert!(intents.iter().all(|i| i.enemy() == Side::Japanese));
    let japanese: Vec<_> = scenario.believed_forces(Side::Japanese).collect();
    assert_eq!(japanese.len(), 1, "{name}");
    assert_eq!(japanese[0].kind, ShipKind::Carrier);
    assert_eq!(japanese[0].ships, 8);
    assert_eq!(japanese[0].standing, Standing::Unknown);
    assert_eq!(japanese[0].last_known, LastKnown::NotKnown);
    assert_eq!(japanese[0].last_known.hex(), None::<Hex>);
    // The Japanese side is given no belief of what the Allied side intends.
    assert_eq!(scenario.believed_intents(Side::Japanese).count(), 0);
    // Every entry is a placeholder, with a note that the research is owed.
    for note in allied
        .iter()
        .chain(japanese.iter())
        .map(|f| (f.placeholder, f.note.clone()))
        .chain(intents.iter().map(|i| (i.placeholder, i.note.clone())))
    {
        assert!(note.0, "{name}: an entry is not marked placeholder");
        let text = note.1.expect("a placeholder has a note");
        assert!(
            text.contains("rests on no source yet") || text.contains("Rests on no source yet"),
            "{name}: {text}"
        );
        assert!(text.contains("is owed"), "{name}: {text}");
    }
}

#[test]
fn the_39_days_load_with_their_file_of_sides_and_their_five_beliefs() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    for file in FILES {
        let path = root.join(file);
        let live = Scenario::load(&path).unwrap_or_else(|e| panic!("{file}: {e}"));
        check(&live, file);
        let frozen = Scenario::load_with(&path, &LoadOptions::frozen_step1(&root))
            .unwrap_or_else(|e| panic!("{file} on the frozen figures: {e}"));
        check(&frozen, file);
    }
}
