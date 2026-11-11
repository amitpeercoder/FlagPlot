//! Everything the scenario tests ask of one side's view on one day.

use std::collections::{BTreeMap, BTreeSet};

use serde::Serialize;
use serde_json::Value;
use sim_agent::{
    briefing, get_entity, list_bases, list_contacts, list_forces, Briefing, Explainer, DEPTH_LIMIT,
};
use sim_core::hex::Hex;
use sim_core::view::SideView;

use super::validator::Schemas;

/// What was produced for one view: the briefing and its text, and every answer
/// as JSON text under a label, for comparing worlds.
pub struct Checked {
    pub briefing: Briefing,
    pub text: String,
    pub answers: BTreeMap<String, String>,
    pub explained: usize,
}

fn json<T: Serialize>(value: &T) -> Value {
    serde_json::to_value(value).expect("an answer serialises")
}

fn valid(schemas: &Schemas, file: &str, value: &Value, label: &str) {
    let errors = schemas.check(file, value);
    assert!(
        errors.is_empty(),
        "{label} against {file}: {errors:?}\n{value}"
    );
}

/// Every number token of a text: runs of digits with an optional fraction.
pub fn numbers_in(text: &str) -> Vec<String> {
    let chars: Vec<char> = text.chars().collect();
    let mut found = Vec::new();
    let mut i = 0;
    while i < chars.len() {
        if chars[i].is_ascii_digit() {
            let start = i;
            while i < chars.len() && chars[i].is_ascii_digit() {
                i += 1;
            }
            if i + 1 < chars.len() && chars[i] == '.' && chars[i + 1].is_ascii_digit() {
                i += 1;
                while i < chars.len() && chars[i].is_ascii_digit() {
                    i += 1;
                }
            }
            found.push(chars[start..i].iter().collect());
        } else {
            i += 1;
        }
    }
    found
}

/// Every number token of the data: of its numbers and of its strings, leaving
/// out the format's own version.
fn data_numbers(value: &Value, out: &mut BTreeSet<String>) {
    match value {
        Value::Number(n) => out.extend(numbers_in(&n.to_string())),
        Value::String(s) => out.extend(numbers_in(s)),
        Value::Array(a) => a.iter().for_each(|v| data_numbers(v, out)),
        Value::Object(o) => {
            for (key, v) in o {
                if key != "version" {
                    data_numbers(v, out);
                }
            }
        }
        _ => {}
    }
}

/// Every number in the text occurs in the data.
pub fn assert_numbers_in_data(text: &str, data: &Value, label: &str) {
    let mut known = BTreeSet::new();
    data_numbers(data, &mut known);
    for number in numbers_in(text) {
        assert!(
            known.contains(&number),
            "{label}: the number {number} of the text is not in the data\n{text}"
        );
    }
}

/// Every unit handle of a value.
pub fn handles_in(value: &Value, out: &mut BTreeSet<String>) {
    match value {
        Value::Array(a) => a.iter().for_each(|v| handles_in(v, out)),
        Value::Object(o) => {
            for (key, v) in o {
                if key == "handle" {
                    if let Some(s) = v.as_str() {
                        out.insert(s.to_string());
                    }
                } else {
                    handles_in(v, out);
                }
            }
        }
        _ => {}
    }
}

/// The handles a side may be shown: its own units, the contacts it holds, and
/// every unit named in an event it may see (decisions 0012 and 0047).
pub fn allowed_handles(view: &SideView<'_>) -> BTreeSet<String> {
    let mut allowed = BTreeSet::new();
    allowed.extend(view.ships().map(|s| s.id.to_string()));
    allowed.extend(view.task_forces().map(|s| s.id.to_string()));
    allowed.extend(view.air_groups().map(|s| s.id.to_string()));
    allowed.extend(view.bases().map(|s| s.id.to_string()));
    allowed.extend(view.contacts().map(|c| c.id.to_string()));
    for event in view.events() {
        allowed.extend(event.kind.entities().iter().map(ToString::to_string));
    }
    allowed
}

fn assert_only_allowed(value: &Value, allowed: &BTreeSet<String>, label: &str) {
    let mut found = BTreeSet::new();
    handles_in(value, &mut found);
    for handle in found {
        assert!(
            allowed.contains(&handle),
            "{label} names {handle}, which is no own unit, no contact held and in no event the side sees"
        );
    }
}

/// The ranks of the events a side sees, to take the log's numbers out.
fn ranks(view: &SideView<'_>) -> BTreeMap<u64, u64> {
    view.events()
        .iter()
        .enumerate()
        .map(|(i, e)| (u64::from(e.id.raw()), i as u64))
        .collect()
}

/// A value with the full log's event numbers replaced by their rank among the
/// side's own events.
pub fn without_event_numbers(value: &Value, rank: &BTreeMap<u64, u64>) -> Value {
    let swap = |v: &Value| match v.as_u64().and_then(|n| rank.get(&n)) {
        Some(r) => Value::from(*r),
        None => v.clone(),
    };
    match value {
        Value::Array(a) => Value::Array(a.iter().map(|v| without_event_numbers(v, rank)).collect()),
        Value::Object(o) => Value::Object(
            o.iter()
                .map(|(k, v)| {
                    let new = match (k.as_str(), v) {
                        ("event" | "last_report_event", v) => swap(v),
                        ("events", Value::Array(a)) => Value::Array(a.iter().map(swap).collect()),
                        _ => without_event_numbers(v, rank),
                    };
                    (k.clone(), new)
                })
                .collect(),
        ),
        other => other.clone(),
    }
}

pub fn normalised_answers(
    view: &SideView<'_>,
    answers: &BTreeMap<String, String>,
) -> BTreeMap<String, Value> {
    let rank = ranks(view);
    answers
        .iter()
        .map(|(k, v)| {
            let value: Value = serde_json::from_str(v).expect("an answer is JSON");
            (k.clone(), without_event_numbers(&value, &rank))
        })
        .collect()
}

/// Builds the briefing, the lists, every entity and every event's explanation
/// for a view and checks them all. `explain_every` asks `explain` about every
/// event the side sees; otherwise about every tenth.
pub fn check_view(
    view: &SideView<'_>,
    scenario: &str,
    schemas: &Schemas,
    explain_every: bool,
) -> Checked {
    let label = format!(
        "{scenario} {:?} start of day {}",
        view.side(),
        view.day() + 1
    );
    let allowed = allowed_handles(view);
    let mut answers = BTreeMap::new();

    // The briefing.
    let b = briefing(view, scenario);
    let b_json = json(&b);
    valid(
        schemas,
        "briefing.schema.json",
        &b_json,
        &format!("{label} briefing"),
    );
    assert_only_allowed(&b_json, &allowed, &format!("{label} briefing"));
    let text = b.to_text();
    assert_numbers_in_data(&text, &b_json, &format!("{label} briefing text"));
    assert_eq!(b.heading.day, view.day() + 1);
    answers.insert("briefing".to_string(), b_json.to_string());

    // The lists, whole and in an area.
    let centre = view
        .task_forces()
        .map(|f| f.position())
        .chain(view.bases().map(|b| b.hex))
        .next()
        .unwrap_or(Hex::new(0, 0));
    for (suffix, area) in [
        ("", None),
        (":area", Some((centre, 200))),
        (":empty", Some((centre, 0))),
    ] {
        let forces = json(&list_forces(view, area));
        valid(
            schemas,
            "list_forces.schema.json",
            &forces,
            &format!("{label} list_forces"),
        );
        assert_only_allowed(&forces, &allowed, &format!("{label} list_forces"));
        answers.insert(format!("list_forces{suffix}"), forces.to_string());
        let bases = json(&list_bases(view, area));
        valid(
            schemas,
            "list_bases.schema.json",
            &bases,
            &format!("{label} list_bases"),
        );
        assert_only_allowed(&bases, &allowed, &format!("{label} list_bases"));
        answers.insert(format!("list_bases{suffix}"), bases.to_string());
        let contacts = json(&list_contacts(view, area));
        valid(
            schemas,
            "list_contacts.schema.json",
            &contacts,
            &format!("{label} list_contacts"),
        );
        assert_only_allowed(&contacts, &allowed, &format!("{label} list_contacts"));
        answers.insert(format!("list_contacts{suffix}"), contacts.to_string());
    }

    // Every own unit and every contact.
    let handles: Vec<String> = view
        .ships()
        .map(|s| s.id.to_string())
        .chain(view.task_forces().map(|s| s.id.to_string()))
        .chain(view.air_groups().map(|s| s.id.to_string()))
        .chain(view.bases().map(|s| s.id.to_string()))
        .chain(view.contacts().map(|s| s.id.to_string()))
        .collect();
    for handle in &handles {
        let answer = json(&get_entity(view, handle));
        valid(
            schemas,
            "entity.schema.json",
            &answer,
            &format!("{label} {handle}"),
        );
        assert_eq!(answer["kind"], "entity", "{label} {handle} is known");
        assert_only_allowed(&answer, &allowed, &format!("{label} {handle}"));
        answers.insert(format!("entity:{handle}"), answer.to_string());
    }

    // Every event the side sees.
    let events = view.events();
    let explainer = Explainer::new(view);
    let numbers = explainer.event_numbers();
    assert_eq!(numbers.len(), events.len());
    let mut explained = 0;
    for (i, event) in events.iter().enumerate() {
        if !explain_every && i % 10 != 0 {
            continue;
        }
        // The side's own number: its place in its list, counting from 1.
        let number = u32::try_from(i + 1).expect("few events");
        assert_eq!(numbers[i], number);
        let answer = json(&explainer.explain(number));
        valid(
            schemas,
            "explain.schema.json",
            &answer,
            &format!("{label} explain {number}"),
        );
        assert_eq!(answer["kind"], "explain");
        assert_only_allowed(&answer, &allowed, &format!("{label} explain {number}"));
        check_tree(&answer, number, event, &label);
        explained += 1;
    }
    Checked {
        briefing: b,
        text,
        answers,
        explained,
    }
}

/// The nodes of an explanation hold together: node 0 is the event, causes are
/// valid indices one step deeper at most, and the root says what the side's
/// event says about hidden causes.
fn check_tree(answer: &Value, number: u32, event: &sim_core::side_events::SideEvent, label: &str) {
    let nodes = answer["nodes"].as_array().expect("nodes");
    assert_eq!(nodes[0]["id"], u64::from(number), "{label}");
    assert_eq!(nodes[0]["hidden_causes"], event.hidden_causes, "{label}");
    assert_eq!(
        nodes[0]["causes"].as_array().unwrap().len(),
        event.causes.len(),
        "{label}"
    );
    let limit = u64::from(DEPTH_LIMIT);
    for (i, node) in nodes.iter().enumerate() {
        // The text is the event's own; the log's `#number` is not in it.
        if let Some(text) = node["text"].as_str() {
            assert!(!text.contains('#'), "{label}: {text}");
        }
        assert_eq!(node["index"], i as u64, "{label}");
        assert!(node["depth"].as_u64().unwrap() <= limit, "{label}");
        for cause in node["causes"].as_array().unwrap() {
            let at = cause.as_u64().unwrap() as usize;
            assert!(at < nodes.len(), "{label}");
            assert!(
                nodes[at]["depth"].as_u64().unwrap() <= node["depth"].as_u64().unwrap() + 1,
                "{label}"
            );
        }
        if node["causes_truncated"] == true {
            assert_eq!(node["depth"].as_u64().unwrap(), limit, "{label}");
            assert_eq!(answer["truncated"], true, "{label}");
        }
    }
}
