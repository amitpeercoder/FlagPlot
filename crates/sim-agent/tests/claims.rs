//! What a side is told of its claims (decisions 14 and 15 of 0056).
//!
//! 1. The briefing lists the claims of the day, one line each, as "Claims
//!    yesterday", with what each says of hits, the kind attacked, aircraft
//!    shot down and aircraft destroyed on the ground; and the JSON of it is
//!    valid against the schema.
//! 2. `explain` on a claim shows the firing events of the side's own that it
//!    rests on, and nothing of the enemy's.
//! 3. A side is never given a claim of the enemy's.

mod support;

use serde_json::Value;
use sim_agent::{briefing, explain};
use sim_core::events::EventKind;
use sim_core::side_events::SideEventKind;
use sim_core::state::Side;
use support::check::valid;
use support::{load, schemas};

const SEED: u64 = 1942;

#[test]
fn the_briefing_lists_claims_yesterday_and_explain_shows_the_firing_each_rests_on() {
    let mut scenario = load("05_eastern_solomons.toml", SEED);
    let title = scenario.name.clone();
    scenario.run_to_end().expect("the run resolves");
    let schemas = schemas();
    let mut explained = 0;
    for side in [Side::Allied, Side::Japanese] {
        let view = scenario.world.side_view(side);
        let made = briefing(&view, &title);
        let json = serde_json::to_value(&made).expect("serialises");
        valid(&schemas, "briefing.schema.json", &json, "briefing");
        let text = made.to_text();
        let claims = &made.losses.claims;
        assert!(!claims.is_empty(), "{side:?} claims something");
        assert!(claims.iter().all(|c| c.from_air));
        for claim in claims {
            let words = match claim.taken_for {
                Some(kind) => format!("{} hits on a {kind}", claim.claimed_hits),
                None => format!("{} aircraft shot down", claim.claimed_shot_down),
            };
            assert!(text.contains(&words), "{side:?}: {words}\n{text}");
            assert!(text.contains(&format!("[event {}]", claim.event)));

            // explain on the claim: node 0 is the claim, and its causes are
            // the side's own firing and attacks, each of an event kind that
            // fires.
            let answer = serde_json::to_value(explain(&view, claim.event)).expect("serialises");
            valid(&schemas, "explain.schema.json", &answer, "explain");
            let nodes = answer["nodes"].as_array().expect("nodes");
            assert_eq!(nodes[0]["event_kind"], "claim_received");
            let causes: Vec<&Value> = nodes[0]["causes"]
                .as_array()
                .expect("causes")
                .iter()
                .map(|i| &nodes[i.as_u64().expect("an index") as usize])
                .collect();
            assert!(!causes.is_empty(), "a claim rests on firing");
            for cause in causes {
                let kind = cause["event_kind"].as_str().expect("a kind");
                assert!(
                    matches!(
                        kind,
                        "air_fight"
                            | "anti_aircraft_fire"
                            | "air_attack"
                            | "base_attacked"
                            | "base_anti_aircraft_fire"
                    ),
                    "{kind}"
                );
            }
            explained += 1;
        }
        // The side's claims are its own: no claim of the other side is among
        // the events it may see.
        for event in view.events() {
            if let SideEventKind::Whole(EventKind::ClaimReceived { side: owner, .. }) = &event.kind
            {
                assert_eq!(*owner, side);
            }
        }
        // And what the objectives say it knows is the sum of the claims.
        let claimed_aircraft: u32 = claims
            .iter()
            .map(|c| c.claimed_shot_down + c.claimed_on_ground)
            .sum();
        let known = &made.objectives.objectives;
        let aircraft = known
            .iter()
            .find(|o| o.kind == "aircraft_destroyed")
            .expect("the objective");
        assert_eq!(aircraft.known.basis, "claims");
        assert_eq!(
            aircraft.known.count.0.raw(),
            i64::from(claimed_aircraft) * 1_000_000
        );
        assert!(text.contains("claimed shot down or destroyed on the ground"));
    }
    assert!(explained >= 6, "{explained}");
}

#[test]
fn a_side_that_has_fired_no_claim_is_told_none() {
    // Scenario 1 has no air or sea fight at all.
    let mut scenario = load("01_transit.toml", 1);
    let title = scenario.name.clone();
    scenario.run_to_end().expect("the run resolves");
    let view = scenario.world.side_view(Side::Allied);
    let made = briefing(&view, &title);
    assert!(made.losses.claims.is_empty());
    assert!(made.to_text().contains("Claims yesterday: none"));
}
