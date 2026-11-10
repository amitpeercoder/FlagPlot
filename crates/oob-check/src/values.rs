//! Rule 3: attributes are in the vocabulary and every value has the form
//! its column or attribute declares.

use std::collections::BTreeMap;

use crate::schema::{ENGAGEMENT_FILES, FACT_FILES};
use crate::table::{Data, Rec};
use crate::value::{is_date, is_decimal, is_integer, is_time, is_whole, looks_like_exponent_float};
use crate::Problem;

/// Engagement columns that hold a quantity when they are not empty.
const ENGAGEMENT_DECIMALS: [&str; 7] = [
    "range_m",
    "bearing_deg",
    "lat",
    "lon",
    "distance_nm",
    "course_deg",
    "speed_kn",
];

pub fn check(data: &Data, out: &mut Vec<Problem>) {
    let value_types: BTreeMap<(&str, &str), &str> = data
        .rows("attributes.csv")
        .iter()
        .map(|a| ((a.get("key"), a.get("applies_to")), a.get("value_type")))
        .collect();
    let keys = data.entity_keys();
    let places = data.keys("places.csv");
    let known =
        |entity_type: &str, key: &str| keys.get(entity_type).is_some_and(|k| k.contains(key));

    for (file, entity_type) in FACT_FILES {
        for row in data.rows(file) {
            validity(row, out);
            let (attribute, value) = (row.get("attribute"), row.get("value"));
            let Some(value_type) = value_types.get(&(attribute, entity_type)) else {
                out.push(row.problem(format!(
                    "attribute `{attribute}` is not in attributes.csv for `{entity_type}`"
                )));
                continue;
            };
            no_exponent(row, "value", out);
            let ok = match *value_type {
                "integer" => is_integer(value),
                "decimal" => is_decimal(value),
                "date" => is_date(value),
                "text" => !value.trim().is_empty(),
                "ref:ship_class" => known("ship_class", value),
                "ref:aircraft_type" => known("aircraft_type", value),
                "ref:place_or_ship" => places.contains(value) || known("ship", value),
                // An unknown type is reported against attributes.csv.
                _ => true,
            };
            if !ok {
                out.push(row.problem(format!(
                    "value `{value}` of `{attribute}` is not a valid {value_type}"
                )));
            }
        }
    }

    for row in data.rows("fits.csv") {
        validity(row, out);
        let count = row.get("count");
        if !count.is_empty() && !is_whole(count) {
            out.push(row.problem(format!("count `{count}` is not a whole number")));
        }
    }

    for row in data.rows("engagements.csv") {
        for column in ["date_from", "date_to"] {
            date_or_empty(row, column, out);
        }
    }
    for file in ENGAGEMENT_FILES {
        for row in data.rows(file) {
            let time = row.get("time");
            if !time.is_empty() && !is_time(time) {
                out.push(row.problem(format!("time `{time}` is not a date or YYYY-MM-DDTHH:MM")));
            }
            for column in ENGAGEMENT_DECIMALS {
                let value = row.get(column);
                if !value.is_empty() && !is_decimal(value) {
                    out.push(row.problem(format!("{column} `{value}` is not a decimal")));
                }
            }
            no_exponent(row, "value", out);
        }
    }
}

fn validity(row: Rec, out: &mut Vec<Problem>) {
    for column in ["valid_from", "valid_to"] {
        date_or_empty(row, column, out);
    }
}

fn date_or_empty(row: Rec, column: &str, out: &mut Vec<Problem>) {
    let value = row.get(column);
    if !value.is_empty() && !is_date(value) {
        out.push(row.problem(format!("{column} `{value}` is not a date")));
    }
}

fn no_exponent(row: Rec, column: &str, out: &mut Vec<Problem>) {
    let value = row.get(column);
    if looks_like_exponent_float(value.trim()) {
        out.push(row.problem(format!(
            "{column} `{value}` looks like a float with an exponent"
        )));
    }
}
