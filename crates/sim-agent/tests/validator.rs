//! The validator itself is tested on a few good and bad documents, with
//! small schemas of its own and then with the real ones.

mod support;

use std::collections::BTreeMap;

use serde_json::{json, Value};
use support::validator::{is_decimal, Schemas};

fn schemas(files: &[(&str, Value)]) -> Schemas {
    Schemas::from_files(
        files
            .iter()
            .map(|(name, value)| ((*name).to_string(), value.clone()))
            .collect::<BTreeMap<_, _>>(),
    )
}

fn thing() -> Schemas {
    schemas(&[
        (
            "defs.schema.json",
            json!({"$defs": {
                "dec": {"type": "string", "format": "decimal"},
                "name": {"type": ["string", "null"]}
            }}),
        ),
        (
            "thing.schema.json",
            json!({
                "type": "object",
                "required": ["kind", "n", "weight", "tags"],
                "additionalProperties": false,
                "properties": {
                    "kind": {"const": "thing"},
                    "n": {"type": "integer"},
                    "weight": {"$ref": "defs.schema.json#/$defs/dec"},
                    "tags": {"type": "array", "items": {"enum": ["a", "b"]}},
                    "name": {"$ref": "#/$defs/local"},
                    "either": {"oneOf": [{"type": "integer"}, {"type": "string"}]}
                },
                "$defs": {"local": {"$ref": "defs.schema.json#/$defs/name"}}
            }),
        ),
    ])
}

fn errors(doc: Value) -> Vec<String> {
    thing().check("thing.schema.json", &doc)
}

#[test]
fn a_good_document_is_valid() {
    let doc = json!({"kind": "thing", "n": 3, "weight": "0.25", "tags": ["a", "b"],
        "name": null, "either": "x"});
    assert!(errors(doc).is_empty());
}

#[test]
fn a_missing_required_field_is_found() {
    let doc = json!({"kind": "thing", "n": 3, "tags": []});
    assert_eq!(errors(doc), vec!["$: missing weight".to_string()]);
}

#[test]
fn an_extra_field_is_found_where_the_schema_closes_the_object() {
    let doc = json!({"kind": "thing", "n": 3, "weight": "1", "tags": [], "extra": 1});
    let found = errors(doc);
    assert_eq!(found.len(), 1);
    assert!(found[0].contains("extra is not allowed"), "{found:?}");
}

#[test]
fn types_are_checked_and_a_float_is_not_an_integer() {
    assert!(!errors(json!({"kind": "thing", "n": 3.5, "weight": "1", "tags": []})).is_empty());
    assert!(!errors(json!({"kind": "thing", "n": "3", "weight": "1", "tags": []})).is_empty());
    // A float is also refused where a decimal string is wanted.
    assert!(!errors(json!({"kind": "thing", "n": 3, "weight": 0.25, "tags": []})).is_empty());
    // 3.0 written in JSON text is a float.
    let parsed: Value =
        serde_json::from_str(r#"{"kind":"thing","n":3.0,"weight":"1","tags":[]}"#).unwrap();
    assert!(!errors(parsed).is_empty());
}

#[test]
fn constants_and_enums_are_checked_inside_arrays() {
    assert!(!errors(json!({"kind": "other", "n": 1, "weight": "1", "tags": []})).is_empty());
    let found = errors(json!({"kind": "thing", "n": 1, "weight": "1", "tags": ["a", "z"]}));
    assert_eq!(found.len(), 1);
    assert!(found[0].starts_with("$.tags[1]"), "{found:?}");
}

#[test]
fn decimal_strings_are_checked_by_form() {
    for good in ["0", "12", "-0.5", "0.333333", "100.25"] {
        assert!(is_decimal(good), "{good}");
    }
    for bad in [
        "", "-", ".5", "5.", "1e3", "0x1", "1,5", "NaN", "--1", "1.2.3", " 1",
    ] {
        assert!(!is_decimal(bad), "{bad}");
    }
    assert!(!errors(json!({"kind": "thing", "n": 1, "weight": "1e3", "tags": []})).is_empty());
}

#[test]
fn one_of_needs_exactly_one_match_and_refs_cross_files() {
    let ok = |extra: Value| {
        let mut doc = json!({"kind": "thing", "n": 1, "weight": "1", "tags": []});
        doc.as_object_mut()
            .unwrap()
            .extend(extra.as_object().unwrap().clone());
        errors(doc)
    };
    assert!(ok(json!({"either": 5})).is_empty());
    assert!(!ok(json!({"either": true})).is_empty());
    assert!(!ok(json!({"name": 5})).is_empty());
    assert!(ok(json!({"name": "x"})).is_empty());
}

#[test]
#[should_panic(expected = "does not support the keyword")]
fn a_keyword_the_validator_ignores_is_an_error() {
    let s = schemas(&[("x.schema.json", json!({"type": "string", "pattern": "^a"}))]);
    let _ = s.check("x.schema.json", &json!("abc"));
}

#[test]
fn the_real_schemas_load_and_refuse_a_wrong_document() {
    let schemas = Schemas::load(&std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("schemas"));
    let bad = json!({"format": "flagplot-agent", "version": 2, "kind": "list_forces"});
    let found = schemas.check("list_forces.schema.json", &bad);
    assert!(
        found.iter().any(|e| e.contains("missing task_forces")),
        "{found:?}"
    );
    let not_known = json!({"format": "flagplot-agent", "version": 2,
        "kind": "not_known", "query": "explain"});
    assert!(schemas.check("explain.schema.json", &not_known).is_empty());
    assert!(schemas.check("entity.schema.json", &not_known).is_empty());
    assert!(schemas
        .check("not_known.schema.json", &not_known)
        .is_empty());
    let wrong = json!({"format": "flagplot-agent", "version": 3,
        "kind": "not_known", "query": "explain"});
    assert!(!schemas.check("not_known.schema.json", &wrong).is_empty());
}
