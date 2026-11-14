//! `measure` `choices`: what the rules would choose now for an own unit
//! (decision 0100).
//!
//! The answer is every question the side's view lists for the unit, with the
//! view's own answer. So: at the start of every day of several scenarios, for
//! every own unit, the lines of the answer are exactly the questions of
//! `SideView::questions` that name the unit and have an answer, in the same
//! order, and the facts each line states (a count, a contact, a distance, a
//! yes or a no) are the view's. Nothing is worked out here. An enemy unit, a
//! wrong kind and a number that names nothing are all `not_known`, the one
//! document.

mod support;

use std::collections::BTreeSet;

use serde_json::Value;
use sim_agent::{measure, MeasureAnswer, Question};
use sim_core::choices::air::{AirAnswer, AirQuestion};
use sim_core::choices::ground::{GroundAnswer, GroundQuestion};
use sim_core::choices::surface::{SurfaceAnswer, SurfaceQuestion};
use sim_core::choices::{Answer, Question as CoreQuestion};
use sim_core::state::Side;
use sim_core::view::SideView;
use support::check::check_measure;
use support::{force_handle, group_handle, land_unit_handle, load, play, schemas};

const SIDES: [Side; 2] = [Side::Allied, Side::Japanese];

fn choices_of(view: &SideView<'_>, handle: &str) -> Value {
    let question = Question::Choices {
        unit: handle.to_string(),
    };
    let (answer, data) = check_measure(view, &question, &schemas(), handle);
    assert!(
        matches!(answer, MeasureAnswer::Choices(_)),
        "{handle}: {data}"
    );
    data
}

/// The name an answer's line carries, from the question alone.
fn choice_name(question: &CoreQuestion) -> &'static str {
    use sim_core::choices::movement::MovementQuestion as M;
    match question {
        CoreQuestion::Air(q) => match q {
            AirQuestion::Strike { .. } => "strike",
            AirQuestion::AircraftAbleToFly { .. } => "aircraft_able_to_fly",
            AirQuestion::BaseStrike { .. } => "base_strike",
            AirQuestion::FighterSplit { .. } => "fighter_split",
            AirQuestion::Escort { .. } => "escort",
            AirQuestion::Landing { .. } => "landing",
            AirQuestion::Rest { .. } => "rest",
        },
        CoreQuestion::Surface(q) => match q {
            SurfaceQuestion::Alert { .. } => "alert",
            SurfaceQuestion::Ready { .. } => "ready",
            SurfaceQuestion::BreakOff { .. } => "break_off",
            SurfaceQuestion::Stragglers { .. } => "stragglers",
            SurfaceQuestion::HeadHome { .. } => "head_home",
        },
        CoreQuestion::Movement(q) => match q {
            M::Reroute { .. } => "reroute",
            M::Halt { .. } => "halt",
            M::SteamOn { .. } => "steam_on",
        },
        CoreQuestion::Ground(q) => match q {
            GroundQuestion::BackToDefend { .. } => "back_to_defend",
            GroundQuestion::UnitDay { .. } => "unit_day",
            GroundQuestion::AttackSpent { .. } => "attack_spent",
        },
    }
}

/// The handles of the own units a question names.
fn units_of(view: &SideView<'_>, question: &CoreQuestion) -> Vec<String> {
    use sim_core::choices::movement::MovementQuestion as M;
    let group = |id| group_handle(view, id);
    let force = |id| force_handle(view, id);
    match question {
        CoreQuestion::Air(q) => match q {
            AirQuestion::Strike { group: g }
            | AirQuestion::AircraftAbleToFly { group: g }
            | AirQuestion::BaseStrike { group: g, .. }
            | AirQuestion::FighterSplit { group: g }
            | AirQuestion::Escort { group: g }
            | AirQuestion::Rest { group: g } => vec![group(*g)],
            AirQuestion::Landing { groups } => groups.iter().map(|(g, _)| group(*g)).collect(),
        },
        CoreQuestion::Surface(q) => vec![force(match q {
            SurfaceQuestion::Alert { task_force }
            | SurfaceQuestion::Ready { task_force, .. }
            | SurfaceQuestion::BreakOff { task_force, .. }
            | SurfaceQuestion::Stragglers { task_force }
            | SurfaceQuestion::HeadHome { task_force } => *task_force,
        })],
        CoreQuestion::Movement(q) => vec![force(match q {
            M::Reroute { force, .. } | M::Halt { force } | M::SteamOn { force } => *force,
        })],
        CoreQuestion::Ground(q) => vec![land_unit_handle(
            view,
            match q {
                GroundQuestion::BackToDefend { unit, .. }
                | GroundQuestion::UnitDay { unit, .. }
                | GroundQuestion::AttackSpent { unit, .. } => *unit,
            },
        )],
    }
}

/// The facts of a line that the view's answer states, checked against it.
fn check_facts(view: &SideView<'_>, answer: &Answer, line: &Value, label: &str) {
    let a = &line["answer"];
    match answer {
        Answer::Air(AirAnswer::AircraftAbleToFly(n)) => assert_eq!(a["aircraft"], *n, "{label}"),
        Answer::Air(AirAnswer::Strike(None)) | Answer::Air(AirAnswer::BaseStrike(None)) => {
            assert_eq!(a["launches"], false, "{label}");
        }
        Answer::Air(AirAnswer::Strike(Some(s))) => {
            assert_eq!(a["launches"], true, "{label}");
            assert_eq!(
                a["contact"]["handle"],
                format!("contact:{}", view.contact_number(s.contact)),
                "{label}"
            );
            assert_eq!(a["distance_nm"], s.distance_nm.to_string(), "{label}");
            assert_eq!(a["taken_for"], s.kind.name(), "{label}");
            assert_eq!(a["hex"]["q"], s.hex.q, "{label}");
            assert_eq!(a["hex"]["r"], s.hex.r, "{label}");
        }
        Answer::Air(AirAnswer::BaseStrike(Some(l))) => {
            assert_eq!(a["launches"], true, "{label}");
            assert_eq!(a["base"]["handle"], l.base.to_string(), "{label}");
            assert_eq!(a["aircraft"], l.aircraft, "{label}");
            assert_eq!(a["distance_nm"], l.distance_nm.to_string(), "{label}");
        }
        Answer::Air(AirAnswer::FighterSplit(s)) => {
            assert_eq!(a["patrol"], s.patrol, "{label}");
            assert_eq!(a["escorts"], s.escorts, "{label}");
        }
        Answer::Air(AirAnswer::Landing(landings)) => {
            let written = a["landings"].as_array().expect("a list");
            assert_eq!(written.len(), landings.len(), "{label}");
            for (w, l) in written.iter().zip(landings) {
                assert_eq!(w["into_the_sea"], l.sea, "{label}");
                assert_eq!(w["places"].as_array().unwrap().len(), l.places.len());
            }
        }
        Answer::Surface(SurfaceAnswer::Ready(r)) => assert_eq!(a["ready"], *r, "{label}"),
        Answer::Surface(SurfaceAnswer::BreakOff(b)) => assert_eq!(a["breaks_off"], *b, "{label}"),
        Answer::Surface(SurfaceAnswer::Stragglers(plan)) => {
            assert_eq!(a["sound_ships"], plan.sound_ships, "{label}");
            assert_eq!(a["kept"], plan.kept, "{label}");
        }
        Answer::Ground(GroundAnswer::BackToDefend(b)) => assert_eq!(a["goes_back"], *b, "{label}"),
        Answer::Ground(GroundAnswer::AttackSpent(c)) => assert_eq!(a["does"], c.name(), "{label}"),
        _ => {}
    }
}

const SCENARIOS: [&str; 12] = [
    "07_full_slice.toml",
    "30_working_units.toml",
    "05b_strike.toml",
    "06a_base_raid.toml",
    "06c_fatigue_and_rest.toml",
    "12c_no_deck.toml",
    "13b_afternoon_base_strike.toml",
    "23_fight_ashore.toml",
    "26_act_and_steam_on.toml",
    "27_preparing_a_landing.toml",
    "04a2_stragglers.toml",
    "05d_carrier_duel.toml",
];

/// The lines of the answer for each own unit are the view's questions that
/// name it and have an answer, in order, with the view's facts; and over the
/// scenarios every kind of choice the sixth question lists is seen.
#[test]
fn the_choices_of_a_unit_are_the_views_questions_about_it_with_the_views_answers() {
    let mut seen: BTreeSet<&'static str> = BTreeSet::new();
    let mut launches = 0;
    for name in SCENARIOS {
        let mut scenario = load(name, 1942);
        play(&mut scenario, |s| {
            for side in SIDES {
                let view = s.world.side_view(side);
                let mut units: Vec<String> = view
                    .air_groups()
                    .map(|u| group_handle(&view, u.id))
                    .chain(view.task_forces().map(|u| force_handle(&view, u.id)))
                    .chain(view.land_units().map(|u| land_unit_handle(&view, u.id)))
                    .collect();
                units.sort();
                let asked: Vec<(CoreQuestion, Answer)> = view
                    .questions()
                    .into_iter()
                    .filter_map(|q| view.ask(&q).map(|a| (q, a)))
                    .collect();
                for unit in units {
                    let label = format!("{name} {side:?} day {} {unit}", view.day() + 1);
                    let data = choices_of(&view, &unit);
                    assert_eq!(data["unit"]["handle"], unit.as_str(), "{label}");
                    let mine: Vec<&(CoreQuestion, Answer)> = asked
                        .iter()
                        .filter(|(q, _)| units_of(&view, q).contains(&unit))
                        .collect();
                    let lines = data["choices"].as_array().expect("a list");
                    assert_eq!(lines.len(), mine.len(), "{label}: {data}");
                    for (line, (question, answer)) in lines.iter().zip(mine) {
                        assert_eq!(line["choice"], choice_name(question), "{label}");
                        seen.insert(choice_name(question));
                        check_facts(&view, answer, line, &label);
                        if line["choice"] == "strike" && line["answer"]["launches"] == true {
                            launches += 1;
                        }
                    }
                }
            }
        });
    }
    assert!(launches > 0, "a strike that launches was seen");
    println!("seen: {seen:?}");
    for choice in [
        "aircraft_able_to_fly",
        "strike",
        "base_strike",
        "fighter_split",
        "escort",
        "alert",
        "ready",
        "break_off",
        "stragglers",
        "head_home",
        "reroute",
        "halt",
        "back_to_defend",
        "unit_day",
        "landing",
        "rest",
        "steam_on",
    ] {
        assert!(seen.contains(choice), "{choice} was never seen: {seen:?}");
    }
}

/// An enemy unit (a side has no handle for one; the numbers above its own
/// count stand for them), a handle of the wrong kind, a number that names
/// nothing and text that is no handle all answer `not_known`, and the same
/// document each time.
#[test]
fn an_enemy_unit_a_wrong_kind_and_a_missing_number_are_not_known() {
    for name in ["07_full_slice.toml", "30_working_units.toml"] {
        let scenario = load(name, 1942);
        for side in SIDES {
            let view = scenario.world.side_view(side);
            let truth = scenario.world.umpire();
            let own_groups = view.air_groups().count();
            let own_forces = view.task_forces().count();
            let own_units = view.land_units().count();
            let mut handles: Vec<String> = Vec::new();
            handles.extend((1..=3).map(|i| format!("air_group:{}", own_groups + i)));
            handles.extend((1..=3).map(|i| format!("task_force:{}", own_forces + i)));
            handles.extend((1..=3).map(|i| format!("land_unit:{}", own_units + i)));
            let enemy_groups = truth.air_groups.values().filter(|u| u.side != side).count();
            handles.extend((1..=enemy_groups).map(|i| format!("air_group:{}", own_groups + i)));
            for kind in ["ship", "base", "contact", "pilot", "event"] {
                handles.extend([format!("{kind}:1"), format!("{kind}:99999")]);
            }
            handles.extend(
                [
                    "air_group:0",
                    "task_force:0",
                    "land_unit:0",
                    "air_group:99999",
                    "air_group:-1",
                    "air_group:x",
                    "air_group:",
                    "air_group",
                    "",
                    "banana",
                    ":3",
                    "task_force:1:2",
                ]
                .map(String::from),
            );
            let first = serde_json::to_string(&measure(
                &view,
                &Question::Choices {
                    unit: handles[0].clone(),
                },
            ))
            .unwrap();
            assert!(first.contains("not_known"), "{name}: {first}");
            for handle in &handles {
                let answer = serde_json::to_string(&measure(
                    &view,
                    &Question::Choices {
                        unit: handle.clone(),
                    },
                ))
                .unwrap();
                assert_eq!(answer, first, "{name} {side:?} {handle:?}");
            }
            assert!(!first.contains("99999"), "{first}");
            // An own unit is known.
            let first_group = view.air_groups().next().map(|g| g.id);
            if let Some(id) = first_group {
                let own = group_handle(&view, id);
                let known =
                    serde_json::to_string(&measure(&view, &Question::Choices { unit: own }))
                        .unwrap();
                assert_ne!(known, first);
            }
        }
    }
}

/// The question reads from JSON as the interface sends it, and an extra key
/// is refused.
#[test]
fn the_question_reads_from_json() {
    assert_eq!(
        Question::from_json(r#"{"question": "choices", "unit": "air_group:2"}"#).unwrap(),
        Question::Choices {
            unit: "air_group:2".to_string()
        }
    );
    assert!(Question::from_json(r#"{"question": "choices"}"#).is_err());
    assert!(
        Question::from_json(r#"{"question": "choices", "unit": "air_group:2", "x": 1}"#).is_err()
    );
}
