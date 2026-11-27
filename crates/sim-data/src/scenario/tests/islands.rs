use super::*;

// Islands (decision 0151, 1).

#[test]
fn an_island_loads_with_its_places_as_hexes_in_order() {
    let scenario = landing(
            false,
            "\n[[island]]\nnumber = 3\nname = \"Placeholder island\"\nplaces = [{ hex = [8, 0] }, { base = 1 }]\n",
        )
        .expect("loads");
    let map = scenario.world.map();
    let island = map.island(3).expect("the island is on the map");
    assert_eq!(island.name, "Placeholder island");
    assert_eq!(island.places.len(), 2);
    assert_eq!(island.places[0].base, None);
    assert_eq!(island.places[0].hex, Hex::new(8, 0));
    assert_eq!(island.places[1].base, Some(BaseId::new(1)));
    assert_eq!(island.places[1].hex, Hex::new(0, 0));
    assert!(map.island(4).is_none());
    // It is kept through a copy for another seed.
    let other = scenario.with_seed(77).expect("copies");
    assert!(other.world.map().island(3).is_some());
}

#[test]
fn a_bad_island_is_a_load_error() {
    let island = |body: &str| format!("\n[[island]]\nnumber = 1\nname = \"Island\"\n{body}\n");
    // No place.
    let message = landing_error(false, &island("places = []"));
    assert!(message.contains("has no place"), "{message}");
    // A base the scenario does not have.
    let message = landing_error(false, &island("places = [{ base = 9 }]"));
    assert!(message.contains("not a base of the scenario"), "{message}");
    // A hex off the map.
    let message = landing_error(false, &island("places = [{ hex = [99, 0] }]"));
    assert!(message.contains("off the map"), "{message}");
    // A hex that is not coast.
    let message = landing_error(false, &island("places = [{ hex = [5, 0] }]"));
    assert!(message.contains("not coast"), "{message}");
    // Both keys, and neither.
    let message = landing_error(false, &island("places = [{ base = 1, hex = [8, 0] }]"));
    assert!(message.contains("both"), "{message}");
    let message = landing_error(false, &island("places = [{}]"));
    assert!(message.contains("neither"), "{message}");
    // A number used twice.
    let message = landing_error(
        false,
        &format!(
            "{}{}",
            island("places = [{ base = 1 }]"),
            island("places = [{ base = 1 }]")
        ),
    );
    assert!(message.contains("appears twice"), "{message}");
    // An unknown key.
    let message = landing_error(false, &island("places = [{ base = 1 }]\ncolour = 1"));
    assert!(message.contains("colour"), "{message}");
}
