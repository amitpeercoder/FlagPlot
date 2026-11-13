//! Stage 9a: the events a side may see (decision 0047).
//!
//! The rule is written, kind by kind, in `crates/sim-core/src/side_events.rs`,
//! and its table is tested there against a hand-built event of each of the 60
//! kinds. This file tests it on whole runs, with nothing but the public side
//! view: what a side is given must hold no truth about the enemy.
//!
//! 1. No truth in a side's events (scenario 7, seeds 1 to 10): no hits scored
//!    by the side's own firing, no failed search or sighting roll, nothing of
//!    a surface fight before the side first sighted or was fired on, no event
//!    whose only units are the enemy's, and no cause the side could not see.
//! 2. Both sides together: every event of the log is seen by one side, or is
//!    one of the few kinds and cases the decision hides from both.
//! 3. Why: from a ship sunk, following the causes the side may see, the walk
//!    ends at its own order, an event with no causes, or an event that says
//!    it has more causes it may not see. It never leads to an event the side
//!    does not have.
//! 4. Leaks by difference: a second world that differs only in something the
//!    side cannot know gives the side the same events, field for field, and
//!    the side that can know it sees the difference.

mod common;

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

use sim_core::events::{Cause, EngagementEndReason, Event, EventKind};
use sim_core::ids::{EntityRef, EventId};
use sim_core::side_events::{SideEvent, SideEventKind};
use sim_core::state::{Side, TruthState};
use sim_core::World;

const ROOT: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/..");

/// Every run here is on the frozen figures, and frozen figures go with frozen
/// scenarios (decision 0073, `data/scenarios/step2-frozen/README.md`): scenarios
/// 5, 7 and 8 are read from their frozen copies in `data/scenarios/step2-frozen`;
/// every other scenario from `data/scenarios`.
fn scenario_path(name: &str) -> PathBuf {
    let frozen_copy = matches!(
        name,
        "05_eastern_solomons.toml" | "07_full_slice.toml" | "08_soak.toml"
    );
    let folder = if frozen_copy {
        "data/scenarios/step2-frozen"
    } else {
        "data/scenarios"
    };
    Path::new(ROOT).join(folder).join(name)
}

fn scenario_text(name: &str) -> String {
    std::fs::read_to_string(scenario_path(name)).expect("the scenario reads")
}

/// Runs a scenario from `text` (which sits where the file `name` does) with
/// `seed` to its end.
fn run_text(text: &str, name: &str, seed: u64) -> World {
    let scenario = common::from_text(text, &scenario_path(name)).expect("the scenario loads");
    let mut scenario = scenario.with_seed(seed).expect("a seed can be set");
    scenario.run_to_end().expect("the run resolves");
    scenario.world
}

fn run(name: &str, seed: u64) -> World {
    run_text(&scenario_text(name), name, seed)
}

const SIDES: [Side; 2] = [Side::Allied, Side::Japanese];

fn unit_side(truth: &TruthState, entity: EntityRef) -> Option<Side> {
    match entity {
        EntityRef::Ship(id) => truth.ships.get(&id).map(|u| u.side),
        EntityRef::TaskForce(id) => truth.task_forces.get(&id).map(|u| u.side),
        EntityRef::AirGroup(id) => truth.air_groups.get(&id).map(|u| u.side),
        EntityRef::Base(id) => truth.bases.get(&id).map(|u| u.side),
        EntityRef::Pilot(_) | EntityRef::LandUnit(_) | EntityRef::Contact(_) => None,
    }
}

/// The name of the kind of a full event or of a side's event.
fn kind_name(debug: &str) -> String {
    debug
        .chars()
        .take_while(|c| c.is_ascii_alphanumeric())
        .collect()
}

fn full_name(kind: &EventKind) -> String {
    kind_name(&format!("{kind:?}"))
}

fn side_name(kind: &SideEventKind) -> String {
    match kind {
        SideEventKind::Whole(kind) => full_name(kind),
        cut => kind_name(&format!("{cut:?}")),
    }
}

/// The words a unit is written with in text and in a `Debug` print.
type MakeUnit = fn(u32) -> EntityRef;
const UNIT_WORDS: [(&str, MakeUnit); 3] = [
    ("ship:", |n| EntityRef::Ship(sim_core::ids::ShipId::new(n))),
    ("task_force:", |n| {
        EntityRef::TaskForce(sim_core::ids::TaskForceId::new(n))
    }),
    ("air_group:", |n| {
        EntityRef::AirGroup(sim_core::ids::AirGroupId::new(n))
    }),
];
const UNIT_TYPES: [(&str, MakeUnit); 3] = [
    ("ShipId(", |n| {
        EntityRef::Ship(sim_core::ids::ShipId::new(n))
    }),
    ("TaskForceId(", |n| {
        EntityRef::TaskForce(sim_core::ids::TaskForceId::new(n))
    }),
    ("AirGroupId(", |n| {
        EntityRef::AirGroup(sim_core::ids::AirGroupId::new(n))
    }),
];

/// Every number that follows `prefix` in `text`.
fn numbers_after(text: &str, prefix: &str) -> Vec<u32> {
    text.match_indices(prefix)
        .filter_map(|(at, _)| {
            let digits: String = text[at + prefix.len()..]
                .chars()
                .take_while(char::is_ascii_digit)
                .collect();
            digits.parse().ok()
        })
        .collect()
}

/// Every ship, task force and air group a `Debug` print of an event holds,
/// whatever field it is in.
fn units_named(printed: &str) -> Vec<EntityRef> {
    UNIT_TYPES
        .iter()
        .flat_map(|(prefix, make)| numbers_after(printed, prefix).into_iter().map(make))
        .collect()
}

/// The side named in the event, for the kinds that name one.
fn named_side(kind: &EventKind) -> Option<Side> {
    match kind {
        EventKind::ReportReceived { side, .. }
        | EventKind::ContactAged { side, .. }
        | EventKind::ContactDropped { side, .. }
        | EventKind::ClaimReceived { side, .. }
        | EventKind::ContactNotThere { side, .. } => Some(*side),
        _ => None,
    }
}

/// The round a surface-fight event carries, for the kinds that carry one
/// and are fired inside a fight.
fn round_of(kind: &EventKind) -> Option<u32> {
    match kind {
        EventKind::SightingRoll { round, .. }
        | EventKind::ForceSurprised { round, .. }
        | EventKind::Salvo { round, .. }
        | EventKind::ForceBecameAlert { round, .. }
        | EventKind::ForceBrokeOff { round, .. }
        | EventKind::ForceSlippedAway { round, .. } => Some(*round),
        EventKind::ShipDamaged { round, .. }
        | EventKind::SystemHit { round, .. }
        | EventKind::ShipSunk { round, .. }
            if *round > 0 =>
        {
            Some(*round)
        }
        _ => None,
    }
}

/// Each surface fight of the log as the first and last event number of the
/// events between its beginning and its end, with its two forces.
fn fights(world: &World) -> Vec<(u32, u32, [sim_core::ids::TaskForceId; 2])> {
    let mut open: Option<(u32, [sim_core::ids::TaskForceId; 2])> = None;
    let mut found = Vec::new();
    for event in world.events().iter() {
        match &event.kind {
            EventKind::EngagementBegun { a, b, .. } => open = Some((event.id.raw(), [*a, *b])),
            EventKind::EngagementEnded { .. } => {
                let (first, forces) = open.take().expect("an end follows a beginning");
                found.push((first, event.id.raw(), forces));
            }
            _ => {}
        }
    }
    assert!(open.is_none(), "every fight ended");
    found
}

/// The round in which `side` first sighted the enemy or was fired on in the
/// fight, read from the full log.
fn first_round_known(world: &World, side: Side, first: u32, last: u32) -> Option<u32> {
    let truth = world.umpire();
    let own_ship = |id| unit_side(truth, EntityRef::Ship(id)) == Some(side);
    let own_force = |id| unit_side(truth, EntityRef::TaskForce(id)) == Some(side);
    world
        .events()
        .iter()
        .filter(|e| e.id.raw() >= first && e.id.raw() <= last)
        .filter_map(|e| match &e.kind {
            EventKind::SightingRoll {
                round,
                task_force,
                sighted: true,
                ..
            } if own_force(*task_force) => Some(*round),
            EventKind::Salvo {
                round,
                firer,
                target,
                ..
            } if own_ship(*firer) || own_ship(*target) => Some(*round),
            _ => None,
        })
        .min()
}

/// Every check of "no truth in a side's events" for one side of one run.
fn assert_no_truth(world: &World, side: Side, label: &str) {
    let truth = world.umpire();
    let events = world.side_view(side).events();
    let full: BTreeMap<EventId, &Event> = world.events().iter().map(|e| (e.id, e)).collect();
    let visible: BTreeSet<EventId> = events.iter().map(|e| e.id).collect();
    assert!(
        events.windows(2).all(|w| w[0].id < w[1].id),
        "{label}: in log order"
    );
    let fights = fights(world);

    for event in &events {
        let log = full[&event.id];
        let name = side_name(&event.kind);
        let at = format!("{label} {side:?} #{} {name}", event.id.raw());

        // The day and the pulse are the log's.
        assert_eq!((event.day, event.pulse), (log.day, log.pulse), "{at}");

        // 1. Firing: the side's own firing is always cut. Only the defender
        // of a base attack, and nobody else, sees hits. A search that sighted
        // is cut too: no distance and no chance (decision 0061).
        match &log.kind {
            EventKind::Salvo { .. }
            | EventKind::AirAttack { .. }
            | EventKind::AirFight { .. }
            | EventKind::AntiAircraftFire { .. }
            | EventKind::BaseAntiAircraftFire { .. } => assert!(
                !matches!(event.kind, SideEventKind::Whole(_)),
                "{at}: a firing event is whole"
            ),
            EventKind::SearchAttempt { .. } => {
                assert!(
                    matches!(event.kind, SideEventKind::SearchAttempt { .. }),
                    "{at}: a search that sighted is whole"
                );
                let shown = format!("{:?} {:?}", event.kind, event.breakdown);
                assert!(
                    !shown.contains("distance") && !shown.contains("chance"),
                    "{at}: {shown}"
                );
                assert!(
                    event.breakdown.as_ref().is_none_or(|b| b.result.is_none()),
                    "{at}: the chance is the breakdown's result"
                );
            }
            // The defender sees the raid with its hits and without the air
            // group that made it; the attacker sees that it attacked.
            EventKind::BaseAttacked { base, .. } => {
                let defender = unit_side(truth, EntityRef::Base(*base)) == Some(side);
                assert_eq!(
                    matches!(event.kind, SideEventKind::BaseAttackedAgainst { .. }),
                    defender,
                    "{at}: only the defender sees what a base attack hit"
                );
                assert_eq!(
                    matches!(event.kind, SideEventKind::BaseAttacked { .. }),
                    !defender,
                    "{at}: the attacker sees that it attacked"
                );
            }
            // Cut of the enemy's name and nothing else (decision 0065).
            EventKind::EngagementBegun { .. }
            | EventKind::EngagementEnded { .. }
            | EventKind::SightingRoll { .. }
            | EventKind::ForceSurprised { .. }
            | EventKind::ForceSlippedAway { .. }
            | EventKind::AlarmRoll { .. } => {
                assert!(
                    !matches!(event.kind, SideEventKind::Whole(_)),
                    "{at}: an event that names an enemy force is whole"
                );
                assert_eq!(name, full_name(&log.kind), "{at}");
                // Its breakdown is the log's, whole.
                assert_eq!(
                    event
                        .breakdown
                        .as_ref()
                        .map(|b| (b.factors.clone(), b.result)),
                    log.breakdown
                        .as_ref()
                        .map(|b| (b.factors.clone(), Some(b.result))),
                    "{at}"
                );
            }
            _ => assert!(
                matches!(event.kind, SideEventKind::Whole(_)),
                "{at}: a kind that is cut for no reason"
            ),
        }

        // No enemy ship, task force or air group is named, in a field or in
        // the text (decision 0065). The one exception is the target of an
        // order the side itself gave.
        let own_order = matches!(
            event.kind,
            SideEventKind::Whole(EventKind::OrderAccepted { .. } | EventKind::OrderRejected { .. })
        );
        if !own_order {
            for entity in units_named(&format!("{:?}", event.kind))
                .into_iter()
                .chain(event.kind.entities())
            {
                if matches!(
                    entity,
                    EntityRef::Ship(_) | EntityRef::TaskForce(_) | EntityRef::AirGroup(_)
                ) {
                    assert_eq!(
                        unit_side(truth, entity),
                        Some(side),
                        "{at}: names {entity}, which is not the side's own"
                    );
                }
            }
            let text = event.to_string();
            for (prefix, make) in UNIT_WORDS {
                for number in numbers_after(&text, prefix) {
                    let entity = make(number);
                    assert_eq!(
                        unit_side(truth, entity),
                        Some(side),
                        "{at}: the text names {entity}: {text}"
                    );
                }
            }
        }
        // Fired at: the view carries no shots (salvo) and no weapon or aircraft
        // figures but the count (air attack); never the chance or the hits.
        let printed = format!("{:?}", event.kind);
        if printed.starts_with("SalvoAgainst") {
            assert!(!printed.contains("shots:"), "{at}: {printed}");
        }
        if printed.starts_with("AirAttackAgainst") {
            assert!(!printed.contains("weapon:"), "{at}: {printed}");
        }
        // The firing kinds only: an event cut of nothing but the enemy's
        // name keeps its own figures.
        if !matches!(
            event.kind,
            SideEventKind::Whole(_)
                | SideEventKind::BaseAttackedAgainst { .. }
                | SideEventKind::SightingRoll { .. }
                | SideEventKind::AlarmRoll { .. }
        ) {
            for field in [
                "hits",
                "hits_on_aircraft",
                "hits_on_supply",
                "chance",
                "expected",
            ] {
                assert!(
                    !printed.contains(&format!("{field}:")),
                    "{at}: a cut event holds {field}"
                );
            }
        }

        // No failed alarm roll, and a search attempt only if it sighted and
        // is the side's own.
        match &log.kind {
            EventKind::AlarmRoll { alarmed, .. } => {
                assert!(*alarmed, "{at}: a failed alarm roll is visible")
            }
            EventKind::SearchAttempt {
                air_group, sighted, ..
            } => {
                assert!(*sighted, "{at}: a failed search attempt is visible");
                assert_eq!(
                    unit_side(truth, EntityRef::AirGroup(*air_group)),
                    Some(side),
                    "{at}: a search attempt of the other side is visible"
                );
            }
            _ => {}
        }

        // 2. No failed search attempt and no failed sighting roll, and no
        // search attempt at all.
        assert!(
            !matches!(
                &event.kind,
                SideEventKind::Whole(EventKind::SearchAttempt { sighted: false, .. })
                    | SideEventKind::Whole(EventKind::SightingRoll { sighted: false, .. })
                    | SideEventKind::Whole(EventKind::ForcesPassedUnseen { .. })
            ),
            "{at}"
        );

        // 4. No event whose only units are the enemy's.
        let own = match (&log.kind, named_side(&log.kind)) {
            (_, Some(named)) => named == side,
            (
                EventKind::OrderAccepted { order, .. } | EventKind::OrderRejected { order, .. },
                _,
            ) => world.order(*order).map(|o| o.side) == Some(side),
            (kind, None) => kind
                .entities()
                .into_iter()
                .any(|entity| unit_side(truth, entity) == Some(side)),
        };
        assert!(own, "{at}: an event with no unit of the side");

        // 5. Every cause is the side's own order or an event it has.
        for cause in &event.causes {
            match cause {
                Cause::Order(id) => assert_eq!(
                    world.order(*id).map(|o| o.side),
                    Some(side),
                    "{at}: a cause is an order of the other side"
                ),
                Cause::Event(id) => assert!(
                    visible.contains(id),
                    "{at}: a cause, #{}, is an event the side does not have",
                    id.raw()
                ),
            }
        }
        // The causes kept are a subset of the log's, in the log's order, and
        // the flag says whether any were dropped.
        let kept: Vec<Cause> = log
            .causes
            .iter()
            .copied()
            .filter(|cause| event.causes.contains(cause))
            .collect();
        assert_eq!(kept, event.causes, "{at}: causes are a subsequence");
        assert_eq!(
            event.hidden_causes,
            event.causes.len() < log.causes.len(),
            "{at}: the flag"
        );
    }

    // 3. Fights: nothing of a surface fight before the side first sighted or
    // was fired on, and the fight itself only if it did.
    for (first, last, _) in &fights {
        let known = first_round_known(world, side, *first, *last);
        let inside: Vec<&SideEvent> = events
            .iter()
            .filter(|e| e.id.raw() >= *first && e.id.raw() <= *last)
            .collect();
        let begun_seen = inside.iter().any(|e| e.id.raw() == *first);
        let ended_seen = inside.iter().any(|e| e.id.raw() == *last);
        assert_eq!(
            begun_seen,
            known.is_some(),
            "{label} {side:?}: begun, fight {first}"
        );
        assert_eq!(
            ended_seen,
            known.is_some(),
            "{label} {side:?}: ended, fight {first}"
        );
        for event in inside {
            let round = match &event.kind {
                SideEventKind::Whole(kind) => round_of(kind),
                SideEventKind::SightingRoll { round, .. }
                | SideEventKind::ForceSurprised { round, .. }
                | SideEventKind::ForceSlippedAway { round, .. } => Some(*round),
                _ => None,
            };
            if let Some(round) = round {
                let known = known.unwrap_or_else(|| {
                    panic!("{label} {side:?} {event} in fight {first}-{last}, which the side does not know of")
                });
                assert!(
                    round >= known,
                    "{label} {side:?} #{}: round {round} is before the side knew of the fight (round {known})",
                    event.id.raw()
                );
            }
        }
        for event in events
            .iter()
            .filter(|e| e.id.raw() > *first && e.id.raw() < *last)
        {
            let kind = match &event.kind {
                SideEventKind::Whole(kind) => full_name(kind),
                cut => side_name(cut),
            };
            if let (
                Some(known),
                SideEventKind::Salvo { round, .. } | SideEventKind::SalvoAgainst { round, .. },
            ) = (known, &event.kind)
            {
                assert!(*round >= known, "{label} {side:?} {kind}");
            }
        }
    }
}

#[test]
fn a_side_is_given_no_truth_about_the_enemy_in_scenario_7() {
    for seed in 1..=10 {
        let world = run("07_full_slice.toml", seed);
        for side in SIDES {
            assert_no_truth(&world, side, &format!("seed {seed}"));
        }
    }
}

#[test]
fn the_other_scenarios_with_fights_searches_and_strikes_give_a_side_no_truth_either() {
    for (name, seeds) in [
        ("04a_night_action.toml", 1..=20u64),
        ("04_savo_island.toml", 1..=5),
        ("05c_air_combat.toml", 1..=10),
        ("05_eastern_solomons.toml", 1..=5),
        ("06_rabaul_guadalcanal.toml", 1..=5),
        ("06a_base_raid.toml", 1..=5),
        ("04c_three_forces.toml", 1..=10),
    ] {
        for seed in seeds {
            let world = run(name, seed);
            for side in SIDES {
                assert_no_truth(&world, side, &format!("{name} seed {seed}"));
            }
        }
    }
}

// ---------------------------------------------------------------------
// 2. Both sides together.
// ---------------------------------------------------------------------

/// Why an event is in neither side's version, or `None` if it is not one the
/// decision hides from both.
fn hidden_from_both(event: &Event) -> Option<&'static str> {
    match &event.kind {
        EventKind::SearchAttempt { sighted: false, .. } => {
            Some("a search attempt that found nothing (decision 4)")
        }
        EventKind::AlarmRoll { alarmed: false, .. } => Some("a failed alarm roll"),
        EventKind::ForcesPassedUnseen { .. } => Some("forces passed unseen (decision 4)"),
        EventKind::SightingRoll { sighted: false, .. } => Some("a roll that did not sight"),
        // A fight neither force knew of leaves no trace but its own begin and
        // end, and those are hidden too.
        EventKind::EngagementBegun { .. } | EventKind::EngagementEnded { .. } => {
            Some("a fight neither side knew of (decision 5)")
        }
        // A force that broke off before it knew of the fight.
        EventKind::ForceBrokeOff { .. } => Some("a break-off before the force knew (decision 5)"),
        _ => None,
    }
}

#[test]
fn every_event_of_the_log_is_seen_by_a_side_or_is_one_the_decision_hides_from_both() {
    let mut hidden: BTreeMap<String, BTreeSet<&'static str>> = BTreeMap::new();
    let mut counted = BTreeMap::<String, usize>::new();
    for (name, seeds) in [
        ("07_full_slice.toml", 1..=10u64),
        ("04_savo_island.toml", 1..=5),
        ("05_eastern_solomons.toml", 1..=5),
        ("06_rabaul_guadalcanal.toml", 1..=3),
    ] {
        for seed in seeds {
            let world = run(name, seed);
            let seen: BTreeSet<EventId> = SIDES
                .into_iter()
                .flat_map(|side| world.side_view(side).events())
                .map(|e| e.id)
                .collect();
            for event in world.events().iter() {
                if seen.contains(&event.id) {
                    continue;
                }
                let why = hidden_from_both(event).unwrap_or_else(|| {
                    panic!("{name} seed {seed}: {event} is seen by neither side")
                });
                // A fight neither side knew of ended with nobody sighted
                // (passed unseen) or with a force that broke off at once.
                if let EventKind::EngagementEnded { reason, .. } = &event.kind {
                    assert!(
                        matches!(
                            reason,
                            EngagementEndReason::PassedUnseen | EngagementEndReason::BrokeOff
                        ),
                        "{name} seed {seed}: {event}"
                    );
                }
                hidden
                    .entry(full_name(&event.kind))
                    .or_default()
                    .insert(why);
                *counted.entry(full_name(&event.kind)).or_default() += 1;
            }
        }
    }
    let kinds: Vec<&String> = hidden.keys().collect();
    // Search attempts are certainly among them: that is what the table says.
    assert!(kinds.iter().any(|k| *k == "SearchAttempt"), "{kinds:?}");
    println!("hidden from both: {counted:?}");
}

// ---------------------------------------------------------------------
// 3. Why.
// ---------------------------------------------------------------------

/// How a walk back through the causes a side may see ended.
#[derive(Default, Debug)]
struct Ends {
    own_order: usize,
    hidden_causes: usize,
    no_causes: usize,
}

/// Follows every cause of `id` the side may see, back, using only the side's
/// view. A cause the side has no event for is a failure.
fn walk(
    world: &World,
    side: Side,
    id: EventId,
    seen: &mut BTreeSet<EventId>,
    ends: &mut Ends,
    label: &str,
) {
    if !seen.insert(id) {
        return;
    }
    let view = world.side_view(side);
    let event = view.event(id).unwrap_or_else(|| {
        panic!(
            "{label}: a cause leads to #{}, which {side:?} may not see",
            id.raw()
        )
    });
    if event.hidden_causes {
        ends.hidden_causes += 1;
    }
    if event.causes.is_empty() && !event.hidden_causes {
        // The log agrees that it has no cause.
        let log = world.events().get(id).expect("in the log");
        assert!(
            log.causes.is_empty(),
            "{label}: #{} lost its causes silently",
            id.raw()
        );
        ends.no_causes += 1;
    }
    for cause in &event.causes {
        match cause {
            Cause::Order(order) => {
                assert_eq!(world.order(*order).map(|o| o.side), Some(side), "{label}");
                ends.own_order += 1;
            }
            Cause::Event(earlier) => walk(world, side, *earlier, seen, ends, label),
        }
    }
}

#[test]
fn from_a_ship_sunk_the_causes_a_side_may_see_lead_back_to_its_own_order_or_to_what_it_cannot_see()
{
    let mut sunk_walks = 0;
    let mut totals = Ends::default();
    let mut sunk_totals = Ends::default();
    for (name, seeds) in [
        ("07_full_slice.toml", 1..=10u64),
        ("04_savo_island.toml", 1..=5),
        ("05_eastern_solomons.toml", 1..=3),
    ] {
        for seed in seeds {
            let world = run(name, seed);
            for side in SIDES {
                let label = format!("{name} seed {seed} {side:?}");
                for event in world.side_view(side).events() {
                    let sunk =
                        matches!(event.kind, SideEventKind::Whole(EventKind::ShipSunk { .. }));
                    let others = matches!(
                        event.kind,
                        SideEventKind::Whole(
                            EventKind::StrikeLaunched { .. }
                                | EventKind::ClaimReceived { .. }
                                | EventKind::TaskForceSteamed { .. }
                        )
                    );
                    if !sunk && !(others && event.id.raw() % 11 == 0) {
                        continue;
                    }
                    sunk_walks += 1;
                    let mut seen = BTreeSet::new();
                    let mut ends = Ends::default();
                    walk(&world, side, event.id, &mut seen, &mut ends, &label);
                    // The walk stopped somewhere honest.
                    assert!(
                        ends.own_order + ends.hidden_causes + ends.no_causes > 0,
                        "{label}: the walk from #{} ended nowhere",
                        event.id.raw()
                    );
                    // A ship sunk walks back through the hit it took to the
                    // salvo or air attack that was made at it, which the
                    // side sees cut, and from there to what it cannot see
                    // (the enemy's orders) or to its own order.
                    if sunk {
                        let through_cut = seen.iter().any(|id| {
                            matches!(
                                world.side_view(side).event(*id).map(|e| e.kind),
                                Some(
                                    SideEventKind::SalvoAgainst { .. }
                                        | SideEventKind::AirAttackAgainst { .. }
                                )
                            )
                        });
                        assert!(through_cut, "{label}: #{}", event.id.raw());
                        assert!(
                            ends.hidden_causes + ends.own_order > 0,
                            "{label}: #{}",
                            event.id.raw()
                        );
                        sunk_totals.own_order += ends.own_order;
                        sunk_totals.hidden_causes += ends.hidden_causes;
                    }
                    totals.own_order += ends.own_order;
                    totals.hidden_causes += ends.hidden_causes;
                    totals.no_causes += ends.no_causes;
                }
            }
        }
    }
    assert!(sunk_walks > 10, "{sunk_walks} walks");
    assert!(
        totals.own_order > 0 && totals.hidden_causes > 0,
        "{totals:?}"
    );
    println!("{sunk_walks} walks: {totals:?}; ships sunk: {sunk_totals:?}");
}

#[test]
fn the_accessors_agree_with_one_another() {
    let world = run("05_eastern_solomons.toml", 3);
    for side in SIDES {
        let view = world.side_view(side);
        let all = view.events();
        assert!(!all.is_empty());
        for day in 1..=3 {
            let of_day: Vec<&SideEvent> = all.iter().filter(|e| e.day == day).collect();
            assert_eq!(
                view.events_on_day(day),
                of_day.into_iter().cloned().collect::<Vec<_>>()
            );
        }
        for event in all.iter().step_by(37) {
            assert_eq!(view.event(event.id).as_ref(), Some(event));
        }
        // An event the side may not see is not there by number either.
        let seen: BTreeSet<EventId> = all.iter().map(|e| e.id).collect();
        let hidden = world
            .events()
            .iter()
            .map(|e| e.id)
            .find(|id| !seen.contains(id))
            .expect("some event is hidden");
        assert_eq!(view.event(hidden), None);
        assert_eq!(view.event(EventId::new(u32::MAX)), None);
    }
}

// ---------------------------------------------------------------------
// 4. Leaks by difference.
// ---------------------------------------------------------------------
//
// Each difference is made in the scenario's text and changes only something
// of the enemy that the side cannot know. The side's events must not change.
// The enemy's own must change, or nothing was compared.

/// A scenario file's text as its head and its `[[tables]]`.
struct Doc {
    head: String,
    tables: Vec<String>,
}

fn parse_doc(text: &str) -> Doc {
    let mut head = String::new();
    let mut tables: Vec<String> = Vec::new();
    for line in text.split_inclusive('\n') {
        if line.trim_end().starts_with("[[") {
            tables.push(line.to_string());
        } else if let Some(last) = tables.last_mut() {
            last.push_str(line);
        } else {
            head.push_str(line);
        }
    }
    Doc { head, tables }
}

impl Doc {
    fn text(&self) -> String {
        let mut text = self.head.clone();
        for table in &self.tables {
            text.push_str(table);
        }
        text
    }
}

fn table_kind(table: &str) -> &str {
    table
        .lines()
        .next()
        .and_then(|line| line.trim().strip_prefix("[["))
        .and_then(|rest| rest.strip_suffix("]]"))
        .unwrap_or("")
}

fn table_value(table: &str, key: &str) -> Option<String> {
    table.lines().find_map(|line| {
        line.strip_prefix(key)
            .and_then(|rest| rest.strip_prefix(" = "))
            .map(|v| v.trim().trim_matches('"').to_string())
    })
}

/// The table with the whole number under `key` changed by `change`, if it has
/// one and `change` gives a value.
fn change_number(table: &str, key: &str, change: impl Fn(i64) -> Option<i64>) -> Option<String> {
    let old = table_value(table, key)?.parse::<i64>().ok()?;
    let new = change(old)?;
    Some(table.replace(&format!("{key} = {old}\n"), &format!("{key} = {new}\n")))
}

fn side_word(side: Side) -> &'static str {
    match side {
        Side::Allied => "allied",
        Side::Japanese => "japanese",
    }
}

/// What a difference does to a scenario's text: the text of the first world
/// and of the second, or `None` if the difference does not apply to the
/// scenario.
type Make = fn(&Doc, Side, Side, &World) -> Option<(String, String)>;

/// Applies `change_number` to every table of `kind` of `side`.
fn edit_tables(
    doc: &Doc,
    side: Side,
    (kind, key): (&str, &str),
    only: impl Fn(&str) -> bool,
    change: impl Fn(i64) -> Option<i64>,
) -> Option<(String, String)> {
    let mut edited = false;
    let tables = doc
        .tables
        .iter()
        .map(|table| {
            if table_kind(table) == kind
                && table_value(table, "side").as_deref() == Some(side_word(side))
                && only(table)
            {
                if let Some(new) = change_number(table, key, &change) {
                    edited = true;
                    return new;
                }
            }
            table.clone()
        })
        .collect();
    edited.then(|| {
        (
            doc.text(),
            Doc {
                head: doc.head.clone(),
                tables,
            }
            .text(),
        )
    })
}

/// The enemy's ships carry a ton less fuel.
fn less_fuel(doc: &Doc, _side: Side, enemy: Side, _world: &World) -> Option<(String, String)> {
    edit_tables(
        doc,
        enemy,
        ("ship", "fuel_t"),
        |_| true,
        |n| (n >= 100).then_some(n - 1),
    )
}

/// The enemy's bases hold a ton more supply.
fn more_supply(doc: &Doc, _side: Side, enemy: Side, _world: &World) -> Option<(String, String)> {
    edit_tables(
        doc,
        enemy,
        ("base", "supply_t"),
        |_| true,
        |n| (n >= 10).then_some(n + 1),
    )
}

/// The enemy's air groups rest at a threshold one point higher.
fn higher_rest_threshold(
    doc: &Doc,
    _side: Side,
    enemy: Side,
    _world: &World,
) -> Option<(String, String)> {
    edit_tables(
        doc,
        enemy,
        ("order", "fatigue"),
        |table| table_value(table, "kind").as_deref() == Some("set_rest_threshold"),
        |n| (n < 99).then_some(n + 1),
    )
}

/// A control, which must be seen: the enemy gets one more aircraft a day. It
/// changes what the enemy can send, which the side meets.
fn more_replacements(
    doc: &Doc,
    _side: Side,
    enemy: Side,
    _world: &World,
) -> Option<(String, String)> {
    edit_tables(
        doc,
        enemy,
        ("replacement", "aircraft_per_day"),
        |_| true,
        |n| Some(n + 1),
    )
}

/// The enemy gives an order, on the last day, to a force of the side's. It is
/// rejected, and the rejection never reveals that the unit exists (decision
/// 0008).
fn rejected_order(doc: &Doc, side: Side, enemy: Side, _world: &World) -> Option<(String, String)> {
    let days = doc.head.lines().find_map(|l| {
        l.strip_prefix("days = ")
            .and_then(|v| v.trim().parse::<u32>().ok())
    })?;
    let target = doc
        .tables
        .iter()
        .find(|t| {
            table_kind(t) == "task_force"
                && table_value(t, "side").as_deref() == Some(side_word(side))
        })
        .and_then(|t| table_value(t, "id"))?;
    let mut text = doc.text();
    text.push_str(&format!(
        "\n[[order]]\nday = {days}\nside = \"{}\"\ntask_force = {target}\nkind = \"set_speed\"\nspeed = \"cruise\"\n",
        side_word(enemy)
    ));
    Some((doc.text(), text))
}

/// The enemy has a force far from every unit of the side, and outside every
/// arc its aircraft could search, in both worlds. In the first it steams one
/// way and in the second another. (Whether it is there at all is a different
/// question, see `a_new_force_takes_the_next_number`.)
fn far_unit_moves(doc: &Doc, side: Side, enemy: Side, world: &World) -> Option<(String, String)> {
    let truth = world.umpire();
    let mine: Vec<sim_core::hex::Hex> = truth
        .task_forces
        .values()
        .filter(|f| f.side == side)
        .map(|f| f.position())
        .chain(
            truth
                .bases
                .values()
                .filter(|b| b.side == side)
                .map(|b| b.hex),
        )
        .collect();
    let (hex, away) = world
        .map()
        .iter()
        .filter(|(hex, _)| world.map().is_navigable(**hex))
        .map(|(hex, _)| {
            let nearest = mine
                .iter()
                .map(|m| m.air_distance_nm(*hex))
                .min()
                .unwrap_or(0);
            (*hex, nearest)
        })
        .max_by_key(|(hex, away)| (*away, hex.q, hex.r))?;
    // Beyond the longest search range of the side's aircraft, with room to
    // spare.
    let longest = truth
        .air_groups
        .values()
        .filter(|g| g.side == side)
        .filter_map(|g| world.ref_data().aircraft_types.get(&g.aircraft_type))
        .map(|t| t.search_range_nm.trunc())
        .max()
        .unwrap_or(0);
    if i64::from(away) < longest + 100 {
        return None;
    }
    let ways: Vec<sim_core::hex::Hex> = (0..6)
        .filter(|d| world.map().can_step(hex, *d))
        .map(|d| hex.neighbors()[d])
        .collect();
    let (way_a, way_b) = (*ways.first()?, *ways.get(1)?);
    let class = truth.ships.values().find(|s| s.side == enemy)?.class.raw();
    let free_force = truth.task_forces.keys().map(|k| k.raw()).max()? + 1;
    let free_ship = truth.ships.keys().map(|k| k.raw()).max()? + 1000;
    let text = |way: sim_core::hex::Hex| {
        let mut text = doc.text();
        text.push_str(&format!(
            "\n[[task_force]]\nid = {free_force}\nname = \"Placeholder far unit\"\nside = \"{e}\"\nq = {q}\nr = {r}\nmission = \"escort\"\n\n[[ship]]\nid = {free_ship}\nname = \"Placeholder far ship\"\nclass = {class}\nside = \"{e}\"\nfuel_t = 100\ntask_force = {free_force}\n\n[[order]]\nday = 1\nside = \"{e}\"\ntask_force = {free_force}\nkind = \"set_route\"\nwaypoints = [[{wq}, {wr}]]\n",
            e = side_word(enemy),
            q = hex.q,
            r = hex.r,
            wq = way.q,
            wr = way.r
        ));
        text
    };
    Some((text(way_a), text(way_b)))
}

/// The enemy has one more force, far away and idle, in the second world and
/// not in the first.
fn far_unit_present(doc: &Doc, side: Side, enemy: Side, world: &World) -> Option<(String, String)> {
    let (_, with) = far_unit_moves(doc, side, enemy, world)?;
    // Drop its order: it only sits there.
    let without_order = with
        .rsplit_once("\n[[order]]")
        .map(|(kept, _)| format!("{kept}\n"))?;
    Some((doc.text(), without_order))
}

/// A side's events with the numbers of the full log taken out: each event is
/// its place among the side's events, and its causes name places too. Two
/// worlds that differ by an event the side does not see give the same list.
#[derive(PartialEq, Debug)]
struct Norm {
    place: usize,
    day: u32,
    pulse: Option<sim_core::time::Pulse>,
    kind: SideEventKind,
    causes: Vec<Result<usize, sim_core::ids::OrderId>>,
    hidden_causes: bool,
    breakdown: Option<sim_core::side_events::SideBreakdown>,
}

fn normalised(world: &World, side: Side) -> Vec<Norm> {
    let events = world.side_view(side).events();
    let place: BTreeMap<EventId, usize> =
        events.iter().enumerate().map(|(i, e)| (e.id, i)).collect();
    events
        .into_iter()
        .enumerate()
        .map(|(i, e)| Norm {
            place: i,
            day: e.day,
            pulse: e.pulse,
            kind: e.kind,
            causes: e
                .causes
                .iter()
                .map(|c| match c {
                    Cause::Event(id) => Ok(place[id]),
                    Cause::Order(id) => Err(*id),
                })
                .collect(),
            hidden_causes: e.hidden_causes,
            breakdown: e.breakdown,
        })
        .collect()
}

fn own_orders(world: &World, side: Side) -> Vec<(u32, u32)> {
    world
        .orders()
        .filter(|o| o.side == side)
        .map(|o| (o.id.raw(), o.day_issued))
        .collect()
}

/// What one difference found across the runs it applied to.
#[derive(Default, Debug)]
struct Found {
    compared: usize,
    /// Runs where the side's events are equal field for field, numbers of
    /// the full log included.
    identical: usize,
    /// Runs where they are equal once the full log's numbers are taken out:
    /// the difference made events the side cannot see, so its numbers moved
    /// (decision 0047, "gaps in event numbers").
    identical_but_for_numbers: usize,
    /// The enemy's own events changed, so the comparison compared something.
    enemy_saw_it: usize,
    /// Runs where the side's events changed, with the first difference.
    changed: Vec<String>,
    inapplicable: usize,
}

fn compare(base_name: &str, seeds: std::ops::RangeInclusive<u64>, make: Make, found: &mut Found) {
    let text = scenario_text(base_name);
    let doc = parse_doc(&text);
    let world0 = common::from_text(&text, &scenario_path(base_name))
        .expect("loads")
        .world;
    for side in SIDES {
        let enemy = if side == Side::Allied {
            Side::Japanese
        } else {
            Side::Allied
        };
        let Some((first, second)) = make(&doc, side, enemy, &world0) else {
            found.inapplicable += 1;
            continue;
        };
        for seed in seeds.clone() {
            let one = run_text(&first, base_name, seed);
            let two = run_text(&second, base_name, seed);
            let label = format!("{base_name} seed {seed}, side {side:?}");
            if own_orders(&one, side) != own_orders(&two, side) {
                // The side's own orders are numbered differently: the
                // comparison would be about the numbers.
                found.inapplicable += 1;
                continue;
            }
            found.compared += 1;
            let before = one.side_view(side).events();
            let after = two.side_view(side).events();
            if before == after {
                found.identical += 1;
            } else if normalised(&one, side) == normalised(&two, side) {
                found.identical_but_for_numbers += 1;
            } else {
                let detail = before.iter().zip(&after).find(|(a, b)| a != b).map_or_else(
                    || format!("{} events against {}", before.len(), after.len()),
                    |(a, b)| format!("\n   {a}\n   {b}"),
                );
                found.changed.push(format!("{label}: {detail}"));
            }
            if normalised(&one, enemy) != normalised(&two, enemy) {
                found.enemy_saw_it += 1;
            }
        }
    }
}

const LEAK_SCENARIOS: [&str; 6] = [
    "04a_night_action.toml",
    "04_savo_island.toml",
    "05c_air_combat.toml",
    "05_eastern_solomons.toml",
    "06_rabaul_guadalcanal.toml",
    "07_full_slice.toml",
];

fn found_by(make: Make, seeds: std::ops::RangeInclusive<u64>) -> Found {
    let mut found = Found::default();
    for base_name in LEAK_SCENARIOS {
        compare(base_name, seeds.clone(), make, &mut found);
    }
    found
}

#[test]
fn what_a_side_cannot_know_changes_nothing_it_is_given() {
    let differences: [(&str, Make); 5] = [
        ("enemy ships carry a ton less fuel", less_fuel),
        ("enemy bases hold a ton more supply", more_supply),
        (
            "enemy rest thresholds are a point higher",
            higher_rest_threshold,
        ),
        ("the enemy gives an order that is rejected", rejected_order),
        (
            "an enemy force far away steams one way, not another",
            far_unit_moves,
        ),
    ];
    for (name, make) in differences {
        let found = found_by(make, 1..=3);
        println!("{name}: {found:?}");
        assert!(
            found.changed.is_empty(),
            "{name}: the side's events changed: {:?}",
            found.changed
        );
        assert!(
            found.compared >= 8,
            "{name}: only {} runs compared",
            found.compared
        );
        assert!(
            found.enemy_saw_it * 2 >= found.compared,
            "{name}: the enemy saw the difference in only {} of {} runs",
            found.enemy_saw_it,
            found.compared
        );
    }
}

#[test]
fn a_difference_the_side_can_meet_does_change_what_it_is_given() {
    // The control. One more aircraft a day for the enemy reaches the side in
    // the raids it meets, so the comparison can tell. If this passed with no
    // change the comparisons above would prove nothing.
    let found = found_by(more_replacements, 1..=3);
    println!("one more replacement aircraft a day: {found:?}");
    assert!(found.compared >= 3);
    assert!(
        !found.changed.is_empty(),
        "more aircraft for the enemy changed nothing the side is given"
    );
}

#[test]
#[ignore = "a known leak: the number of a force made in the game counts the enemy's forces (reported to the architect)"]
fn an_enemy_force_that_is_merely_there_changes_nothing_the_side_is_given() {
    // A known leak, kept as documentation: see decision 0047, "What it
    // costs". The architect will record it with its remedy (numbering created
    // forces per side). Id allocation is not changed in this stage.
    // Task force numbers made during a game (a straggler rule that splits
    // ships off) follow the highest number in the starting state, both
    // sides' forces together. One more enemy force moves them, and so the
    // side's own `ShipsDetached` event, and then every random draw keyed on
    // the new force. Run with `--ignored` to see it.
    let found = found_by(far_unit_present, 1..=3);
    println!("{found:?}");
    assert!(found.changed.is_empty(), "{:?}", found.changed);
}
