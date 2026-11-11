//! A small JSON Schema validator for the schemas of `crates/sim-agent/schemas`.
//!
//! It knows exactly what those schemas use: `type` (one name or a list),
//! `const`, `enum`, `required`, `properties`, `additionalProperties` (a
//! boolean), `items`, `oneOf` (exactly one branch must match), `$ref` (to
//! `#/$defs/name` in the same file or `file#/$defs/name` in another file of the
//! folder), and `format: "decimal"` for strings. Any other keyword that
//! constrains a value is an error, so a schema cannot quietly use something
//! this validator ignores. Annotations (`$schema`, `$id`, `title`,
//! `description`, `$defs`) are skipped.

use std::collections::BTreeMap;
use std::path::Path;

use serde_json::Value;

pub struct Schemas {
    files: BTreeMap<String, Value>,
}

const ANNOTATIONS: &[&str] = &["$schema", "$id", "title", "description", "$defs"];
const KNOWN: &[&str] = &[
    "type",
    "const",
    "enum",
    "required",
    "properties",
    "additionalProperties",
    "items",
    "oneOf",
    "$ref",
    "format",
];

impl Schemas {
    /// Reads every `*.schema.json` file of a folder.
    pub fn load(dir: &Path) -> Schemas {
        let mut files = BTreeMap::new();
        for entry in std::fs::read_dir(dir).expect("the schema folder reads") {
            let path = entry.expect("an entry reads").path();
            let name = path
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or_default()
                .to_string();
            if name.ends_with(".schema.json") {
                let text = std::fs::read_to_string(&path).expect("a schema reads");
                let value: Value = serde_json::from_str(&text)
                    .unwrap_or_else(|e| panic!("{name} is not JSON: {e}"));
                files.insert(name, value);
            }
        }
        Schemas { files }
    }

    pub fn from_files(files: BTreeMap<String, Value>) -> Schemas {
        Schemas { files }
    }

    /// Every problem of `instance` against the schema file `file`; empty if
    /// it is valid.
    pub fn check(&self, file: &str, instance: &Value) -> Vec<String> {
        let root = self
            .files
            .get(file)
            .unwrap_or_else(|| panic!("no schema file {file}"));
        let mut errors = Vec::new();
        self.walk(file, root, instance, "$", &mut errors);
        errors
    }

    fn resolve<'a>(&'a self, file: &'a str, reference: &str) -> (&'a str, &'a Value) {
        let (target_file, pointer) = match reference.split_once('#') {
            Some((f, p)) => (if f.is_empty() { file } else { f }, p),
            None => (reference, ""),
        };
        let (name, root) = self
            .files
            .get_key_value(target_file)
            .unwrap_or_else(|| panic!("no schema file {target_file}"));
        let mut node = root;
        for part in pointer.split('/').filter(|p| !p.is_empty()) {
            node = node
                .get(part)
                .unwrap_or_else(|| panic!("no {part} in {reference}"));
        }
        (name.as_str(), node)
    }

    fn walk(&self, file: &str, schema: &Value, inst: &Value, path: &str, errors: &mut Vec<String>) {
        let Some(schema) = schema.as_object() else {
            // `true` and `false` schemas are not used.
            panic!("schema at {path} in {file} is not an object");
        };
        for key in schema.keys() {
            assert!(
                KNOWN.contains(&key.as_str()) || ANNOTATIONS.contains(&key.as_str()),
                "the validator does not support the keyword {key} (in {file})"
            );
        }
        if let Some(reference) = schema.get("$ref").and_then(Value::as_str) {
            let (target_file, target) = self.resolve(file, reference);
            self.walk(target_file, target, inst, path, errors);
        }
        if let Some(ty) = schema.get("type") {
            let names: Vec<&str> = match ty {
                Value::String(s) => vec![s.as_str()],
                Value::Array(a) => a.iter().filter_map(Value::as_str).collect(),
                _ => panic!("bad type in {file}"),
            };
            if !names.iter().any(|n| type_is(n, inst)) {
                errors.push(format!(
                    "{path}: expected {names:?}, found {}",
                    kind_of(inst)
                ));
                return;
            }
        }
        if let Some(expected) = schema.get("const") {
            if expected != inst {
                errors.push(format!(
                    "{path}: expected the constant {expected}, found {inst}"
                ));
            }
        }
        if let Some(Value::Array(allowed)) = schema.get("enum") {
            if !allowed.contains(inst) {
                errors.push(format!("{path}: {inst} is not one of {allowed:?}"));
            }
        }
        if schema.get("format").and_then(Value::as_str) == Some("decimal") {
            if let Some(text) = inst.as_str() {
                if !is_decimal(text) {
                    errors.push(format!("{path}: {text:?} is not a decimal string"));
                }
            }
        }
        if let Some(branches) = schema.get("oneOf").and_then(Value::as_array) {
            let matching = branches
                .iter()
                .filter(|branch| {
                    let mut sub = Vec::new();
                    self.walk(file, branch, inst, path, &mut sub);
                    sub.is_empty()
                })
                .count();
            if matching != 1 {
                errors.push(format!(
                    "{path}: {matching} of the oneOf branches match, not one"
                ));
            }
        }
        if let Some(object) = inst.as_object() {
            if let Some(Value::Array(required)) = schema.get("required") {
                for name in required.iter().filter_map(Value::as_str) {
                    if !object.contains_key(name) {
                        errors.push(format!("{path}: missing {name}"));
                    }
                }
            }
            let properties = schema.get("properties").and_then(Value::as_object);
            for (name, value) in object {
                match properties.and_then(|p| p.get(name)) {
                    Some(sub) => self.walk(file, sub, value, &format!("{path}.{name}"), errors),
                    None => {
                        if schema.get("additionalProperties") == Some(&Value::Bool(false)) {
                            errors.push(format!("{path}: {name} is not allowed"));
                        }
                    }
                }
            }
        }
        if let (Some(items), Some(array)) = (schema.get("items"), inst.as_array()) {
            for (i, item) in array.iter().enumerate() {
                self.walk(file, items, item, &format!("{path}[{i}]"), errors);
            }
        }
    }
}

fn type_is(name: &str, inst: &Value) -> bool {
    match name {
        "object" => inst.is_object(),
        "array" => inst.is_array(),
        "string" => inst.is_string(),
        "boolean" => inst.is_boolean(),
        "null" => inst.is_null(),
        // A whole number, written without a fraction or exponent.
        "integer" => inst.is_i64() || inst.is_u64(),
        other => panic!("the validator does not support the type {other}"),
    }
}

fn kind_of(inst: &Value) -> &'static str {
    match inst {
        Value::Null => "null",
        Value::Bool(_) => "boolean",
        Value::Number(n) if n.is_f64() => "a float",
        Value::Number(_) => "an integer",
        Value::String(_) => "a string",
        Value::Array(_) => "an array",
        Value::Object(_) => "an object",
    }
}

/// An optional minus, digits, and optionally a point and more digits.
pub fn is_decimal(text: &str) -> bool {
    let body = text.strip_prefix('-').unwrap_or(text);
    let (whole, fraction) = match body.split_once('.') {
        Some((w, f)) => (w, Some(f)),
        None => (body, None),
    };
    let digits = |s: &str| !s.is_empty() && s.chars().all(|c| c.is_ascii_digit());
    digits(whole) && fraction.is_none_or(digits)
}
