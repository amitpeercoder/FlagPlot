//! The fixture `viewer/measure_fixture.json`: cases for the viewer's
//! measuring tool, worked out by the engine's own code. `node viewer/check.js`
//! runs the page's measuring functions on the same cases and fails on any
//! difference, so the page and the engine cannot disagree about a distance.
//!
//! The test below writes the fixture again and asserts that the committed file
//! is the same. To change the cases, run with `FLAGPILOT_UPDATE_GOLDEN=1`.
//!
//! * Air distance is `Hex::air_distance_nm` (decision 0040).
//! * A sea route is `Map::sea_path` (decision 0017); its miles are what the
//!   steps are charged (`Map::step_charge_nm`, decision 0058), as `measure`
//!   states them (decision 0049). The case lists each step's charge.

use std::collections::BTreeSet;
use std::path::Path;

use serde_json::{json, Value};
use sim_core::hex::{Hex, HEX_NM};
use sim_core::map::Map;

use super::names;

/// A small deterministic generator, so the cases never depend on a crate.
struct Lcg(u64);

impl Lcg {
    fn next(&mut self) -> u64 {
        self.0 = self
            .0
            .wrapping_mul(6_364_136_223_846_793_005)
            .wrapping_add(1_442_695_040_888_963_407);
        self.0 >> 33
    }

    fn below(&mut self, n: usize) -> usize {
        usize::try_from(self.next() % n as u64).expect("fits")
    }

    fn range(&mut self, low: i32, high: i32) -> i32 {
        low + i32::try_from(self.next() % u64::try_from(high - low + 1).expect("range"))
            .expect("fits")
    }
}

fn pair(h: Hex) -> Value {
    json!([h.q, h.r])
}

fn air_cases() -> Vec<Value> {
    let mut pairs: Vec<(Hex, Hex)> = Vec::new();
    // Every offset from the origin within 8 hexes, which covers each bearing
    // and the rounding at short range.
    for dq in -8..=8 {
        for dr in -8..=8 {
            pairs.push((Hex::new(0, 0), Hex::new(dq, dr)));
        }
    }
    // The known cases of decision 0040.
    pairs.push((Hex::new(-5, -13), Hex::new(2, -4)));
    pairs.push((Hex::new(1, -9), Hex::new(0, 0)));
    // Random pairs over a larger box than the slice, with both signs.
    let mut rng = Lcg(40);
    for _ in 0..200 {
        let a = Hex::new(rng.range(-40, 40), rng.range(-40, 40));
        let b = Hex::new(rng.range(-40, 40), rng.range(-40, 40));
        pairs.push((a, b));
    }
    pairs
        .into_iter()
        .map(|(a, b)| json!({"from": pair(a), "to": pair(b), "nm": a.air_distance_nm(b)}))
        .collect()
}

fn sea_case(map: &Map, from: Hex, to: Hex) -> Value {
    let path = map.sea_path(from, to);
    let steps = path.as_ref().map(Vec::len);
    let field = map.sea_distances_to(to);
    let charges: Option<Vec<u32>> = path.as_ref().map(|p| {
        let mut here = from;
        p.iter()
            .map(|next| {
                let charge = map.step_charge_nm(&field, here, *next, to);
                here = *next;
                charge
            })
            .collect()
    });
    json!({
        "from": pair(from),
        "to": pair(to),
        "path": path.as_ref().map(|p| p.iter().map(|h| pair(*h)).collect::<Vec<_>>()),
        "steps": steps,
        "charges": charges,
        "nm": charges.as_ref().map(|c| c.iter().map(|n| i64::from(*n)).sum::<i64>()),
    })
}

fn sea_cases(map: &Map) -> Vec<Value> {
    let navigable: Vec<Hex> = map
        .iter()
        .filter(|(h, _)| map.is_navigable(**h))
        .map(|(h, _)| *h)
        .collect();
    let land: Vec<Hex> = map
        .iter()
        .filter(|(h, _)| !map.is_navigable(**h))
        .map(|(h, _)| *h)
        .collect();
    let mut cases = Vec::new();
    let mut seen: BTreeSet<(Hex, Hex)> = BTreeSet::new();
    // Adds a case unless it is there already; true when it was added.
    let mut push = |cases: &mut Vec<Value>, from: Hex, to: Hex| -> bool {
        if !seen.insert((from, to)) {
            return false;
        }
        cases.push(sea_case(map, from, to));
        true
    };

    // Across a closed side: a hex and its neighbour with the side between them
    // closed, both hexes sea. The route must go around, or does not exist.
    let mut around = 0;
    for hex in &navigable {
        for (direction, neighbor) in hex.neighbors().into_iter().enumerate() {
            if around >= 24 {
                break;
            }
            if map.is_navigable(neighbor) && !map.can_step(*hex, direction) && hex < &neighbor {
                push(&mut cases, *hex, neighbor);
                around += 1;
            }
        }
    }
    // The places of the scenarios: Rabaul, Buka, Tulagi, Lunga Point, Milne
    // Bay, Espiritu Santo and Noumea, in every pairing.
    let places = [
        Hex::new(-5, -13),
        Hex::new(-2, -11),
        Hex::new(3, -5),
        Hex::new(2, -4),
        Hex::new(-13, -3),
        Hex::new(7, 6),
        Hex::new(0, 18),
    ];
    for a in places {
        for b in places {
            if a != b {
                push(&mut cases, a, b);
            }
        }
    }
    // The same hex, and land at either end.
    push(&mut cases, places[0], places[0]);
    for i in 0..3 {
        push(&mut cases, land[i * 7], places[i]);
        push(&mut cases, places[i + 3], land[i * 11 + 2]);
    }
    push(&mut cases, land[0], land[0]);
    // Off the map at either end.
    push(&mut cases, places[0], Hex::new(500, 500));
    push(&mut cases, Hex::new(-500, 3), places[5]);
    // Sea hexes that cannot reach a random target, read off its distance
    // field. On the map as it is, every sea hex reaches every other (the
    // closed sides only lengthen routes), so this adds nothing today; it is
    // here so that a map with a cut-off lagoon gets its cases.
    let mut rng = Lcg(17);
    let mut unreachable = 0;
    for _ in 0..200 {
        if unreachable >= 16 {
            break;
        }
        let target = navigable[rng.below(navigable.len())];
        let field = map.sea_distances_to(target);
        if let Some(cut_off) = navigable.iter().find(|h| !field.contains_key(h)) {
            if push(&mut cases, *cut_off, target) {
                unreachable += 1;
            }
        }
    }
    // Random sea pairs.
    for _ in 0..60 {
        let a = navigable[rng.below(navigable.len())];
        let b = navigable[rng.below(navigable.len())];
        push(&mut cases, a, b);
    }
    cases
}

fn map_json(map: &Map) -> Value {
    let mut hexes: Vec<(&Hex, Value)> = map
        .iter()
        .map(|(h, d)| {
            let closed: Vec<usize> = (0..6).filter(|i| d.closed_sides[*i]).collect();
            (
                h,
                json!({"q": h.q, "r": h.r, "kind": names::hex_kind(d.kind), "closed": closed}),
            )
        })
        .collect();
    hexes.sort_by_key(|(h, _)| (h.r, h.q));
    Value::Array(hexes.into_iter().map(|(_, v)| v).collect())
}

/// The fixture as text: one case or hex to a line, so a change reads in a diff.
fn fixture_text(map: &Map, air: &[Value], sea: &[Value]) -> String {
    fn lines(items: &[Value]) -> String {
        items
            .iter()
            .map(|v| format!("    {v}"))
            .collect::<Vec<_>>()
            .join(",\n")
    }
    let hexes = match map_json(map) {
        Value::Array(a) => a,
        _ => unreachable!("map_json is an array"),
    };
    let directions: Vec<Value> = super::map::directions().iter().map(|d| json!(d)).collect();
    format!(
        "{{\n  \"format\": \"flagplot-measure-fixture\",\n  \"version\": 2,\n  \"note\": \"Worked out by the engine (Hex::air_distance_nm, Map::sea_path, Map::step_charge_nm) in crates/sim-cli/src/export/measure_fixture.rs. node viewer/check.js runs the page's functions on it. Rewrite with FLAGPILOT_UPDATE_GOLDEN=1 cargo test -p sim-cli measure_fixture.\",\n  \"map\": \"data/map/south_pacific.csv\",\n  \"hex_nm\": {HEX_NM},\n  \"directions\": {},\n  \"hexes\": [\n{}\n  ],\n  \"air\": [\n{}\n  ],\n  \"sea\": [\n{}\n  ]\n}}\n",
        Value::Array(directions),
        lines(&hexes),
        lines(air),
        lines(sea),
    )
}

#[test]
fn the_viewer_fixture_is_what_the_engine_gives() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let map = sim_data::load_map(&root.join("data/map/south_pacific.csv")).expect("the map loads");
    let air = air_cases();
    let sea = sea_cases(&map);
    let text = fixture_text(&map, &air, &sea);

    // The fixture must exercise what it is for.
    assert!(air.len() >= 200, "{} air cases", air.len());
    assert!(sea.len() >= 60, "{} sea cases", sea.len());
    let none = sea.iter().filter(|c| c["path"].is_null()).count();
    assert!(none >= 9, "{none} sea cases with no route");
    let found = sea.iter().filter(|c| !c["path"].is_null()).count();
    assert!(found >= 50, "{found} sea cases with a route");
    // Some route has to go around a closed side: its length beats the hex count.
    let hex_of = |v: &Value| Hex::new(v[0].as_i64().unwrap() as i32, v[1].as_i64().unwrap() as i32);
    let detours = sea
        .iter()
        .filter(|c| {
            c["steps"].as_u64().is_some_and(|steps| {
                steps > u64::from(hex_of(&c["from"]).distance(hex_of(&c["to"])))
            })
        })
        .count();
    assert!(detours >= 10, "{detours} routes longer than the hex count");

    let path = root.join("viewer/measure_fixture.json");
    if std::env::var("FLAGPILOT_UPDATE_GOLDEN").is_ok() {
        std::fs::write(&path, &text).expect("the fixture writes");
        return;
    }
    let expected = std::fs::read_to_string(&path)
        .expect("viewer/measure_fixture.json is missing; run with FLAGPILOT_UPDATE_GOLDEN=1");
    assert!(
        expected == text,
        "viewer/measure_fixture.json differs from what the engine gives; run with FLAGPILOT_UPDATE_GOLDEN=1 and read the difference"
    );
}
