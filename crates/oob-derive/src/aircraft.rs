//! The figures of one aircraft type, from the facts, as of a date
//! (decision 0029, 6 to 11).

use std::collections::BTreeSet;

use crate::derive::{number, overflow, Cite, Env, Fit, Got, Work};
use crate::input::AircraftEntry;
use crate::problem::{Kind, Problem};
use crate::reading::{resolve, Failure, Read, Row, Table};

/// The figures of an aircraft type row, in the column order of the
/// aircraft CSV after `id` and `name`.
pub const AIRCRAFT_FIGURES: [&str; 8] = [
    "role",
    "search_range_nm",
    "sweep_width_nm",
    "strike_range_nm",
    "attack_weapon",
    "attack_damage",
    "pilot_armour",
    "self_sealing_tanks",
];

const ROLE: usize = 0;
const SEARCH: usize = 1;
const SWEEP: usize = 2;
const STRIKE: usize = 3;
const WEAPON: usize = 4;
const DAMAGE: usize = 5;
const ARMOUR: usize = 6;
const TANKS: usize = 7;

/// The facts role of a type, mapped to a simulation role (decision 0029, 9).
const ROLE_FOR_FACTS_ROLE: [(&str, &str); 8] = [
    ("fighter", "fighter"),
    ("dive_bomber", "bomber"),
    ("torpedo_bomber", "bomber"),
    ("heavy_bomber", "bomber"),
    ("land_attack_bomber", "bomber"),
    ("patrol_flying_boat", "patrol"),
    ("floatplane_reconnaissance", "patrol"),
    ("patrol_amphibian", "patrol"),
];

/// One figure of an aircraft type: the value as written, the rule that
/// gave it, the rows read and the notes for the sources file.
#[derive(Clone, Debug)]
pub struct AircraftFigure {
    pub value: String,
    pub rule: String,
    pub cites: BTreeSet<Cite>,
    pub notes: Vec<String>,
}

#[derive(Clone, Debug)]
pub struct AircraftResult {
    /// `name` of the type in `aircraft_types.csv`.
    pub name: String,
    pub figures: Vec<AircraftFigure>,
    /// Exceptions used: index into the rules' list, and the row read.
    pub used: Vec<(usize, Row)>,
}

fn figure(value: impl Into<String>, rule: impl Into<String>, got: Got<()>) -> AircraftFigure {
    let mut notes = Vec::new();
    if got.excepted {
        notes.push("rests on an exception".to_string());
    }
    AircraftFigure {
        value: value.into(),
        rule: rule.into(),
        cites: got.cites,
        notes,
    }
}

fn nothing() -> Got<()> {
    Got {
        value: (),
        cites: BTreeSet::new(),
        excepted: false,
        doubts: BTreeSet::new(),
    }
}

impl Env<'_> {
    fn type_fact(&self, key: &str, attribute: &str) -> Result<Read, Failure> {
        resolve(
            self.index.rows(Table::AircraftFacts, key, attribute),
            &format!("{attribute} of aircraft type {key}"),
            self.date,
            &self.rules.exceptions,
        )
    }

    /// The radius of action (decision 0029, 6): the one sourced combat
    /// radius that can be read, else the range with a normal load x the
    /// fraction of the rules file, rounded down. Returns the radius, the
    /// rule that gave it, its sources, and notes.
    fn radius(&self, w: &mut Work, key: &str) -> Option<(i64, String, Got<()>, Vec<String>)> {
        let fraction = self.rules.strike_radius_fraction_of_range?;
        let name = AIRCRAFT_FIGURES[STRIKE];
        let mut notes = Vec::new();
        let combat = self.type_fact(key, "combat_radius_nm");
        let combat_failure = match combat {
            Ok(read) => {
                let value = match number(&read, "combat_radius_nm") {
                    Ok(value) => value,
                    Err(failure) => {
                        w.failed(name, failure);
                        return None;
                    }
                };
                if value.has_fraction() {
                    w.fail(
                        Kind::BadValue,
                        name,
                        format!("combat_radius_nm of aircraft type {key}: {value} is not a whole number"),
                    );
                    return None;
                }
                let (cites, excepted, _) = w.take(&read);
                let got = Got {
                    value: (),
                    cites,
                    excepted,
                    doubts: BTreeSet::new(),
                };
                return Some((
                    value.whole(),
                    "combat_radius_nm of the type: the one value that can be read (decision 0029, 6)"
                        .to_string(),
                    got,
                    notes,
                ));
            }
            Err(failure)
                if matches!(
                    failure.kind,
                    Kind::Missing | Kind::NotPreferred | Kind::NotOnDate
                ) =>
            {
                failure
            }
            Err(failure) => {
                w.failed(name, failure);
                return None;
            }
        };
        let range = match self.type_fact(key, "range_normal_nm") {
            Ok(read) => read,
            Err(failure) => {
                w.fail(
                    failure.kind,
                    name,
                    format!(
                        "no radius can be read: {}; {}",
                        combat_failure.message, failure.message
                    ),
                );
                return None;
            }
        };
        let range_value = match number(&range, "range_normal_nm") {
            Ok(value) => value,
            Err(failure) => {
                w.failed(name, failure);
                return None;
            }
        };
        let Some(exact) = range_value.mul(fraction) else {
            w.failed(name, overflow("radius"));
            return None;
        };
        let (cites, excepted, _) = w.take(&range);
        notes.push(format!(
            "combat_radius_nm not readable ({}); range_normal_nm {range_value} x {fraction} = {exact}",
            combat_failure.message
        ));
        if exact.has_fraction() {
            notes.push(format!("exact {exact}, rounded down"));
        }
        Some((
            exact.whole(),
            "range_normal_nm of the type x strike_radius_fraction_of_range, rounded down to a whole number (decision 0029, 6); no combat radius could be read"
                .to_string(),
            Got {
                value: (),
                cites,
                excepted,
                doubts: BTreeSet::new(),
            },
            notes,
        ))
    }

    /// A yes or no fact as 1 or 0.
    fn yes_no(&self, w: &mut Work, key: &str, attribute: &str) -> Option<(i64, Got<()>)> {
        let read = match self.type_fact(key, attribute) {
            Ok(read) => read,
            Err(failure) => {
                w.failed(attribute, failure);
                return None;
            }
        };
        let row = &read.rows[0];
        let value = match row.value.trim().to_ascii_lowercase().as_str() {
            "yes" => 1,
            "no" => 0,
            _ => {
                w.fail(
                    Kind::BadValue,
                    attribute,
                    format!(
                        "{attribute} of aircraft type {key}: row {} has value `{}`, which is not yes or no",
                        row.cite(),
                        row.value
                    ),
                );
                return None;
            }
        };
        let (cites, excepted, _) = w.take(&read);
        Some((
            value,
            Got {
                value: (),
                cites,
                excepted,
                doubts: BTreeSet::new(),
            },
        ))
    }

    /// What the type attacks with (decision 0029, 7 and 8): the weapon, its
    /// damage, and the rule and sources for each.
    fn attack(
        &self,
        w: &mut Work,
        key: &str,
        role: &str,
        load: Option<&str>,
    ) -> Option<[AircraftFigure; 2]> {
        let (weapon_name, damage_name) = (AIRCRAFT_FIGURES[WEAPON], AIRCRAFT_FIGURES[DAMAGE]);
        let Some(load) = load else {
            if role == "bomber" {
                w.fail(
                    Kind::BadValue,
                    weapon_name,
                    format!("aircraft type {key} is a bomber and the list names no load for it"),
                );
                return None;
            }
            let rule = "no load named in the list: attacks with nothing (decision 0029, 7)";
            return Some([
                figure("none", rule, nothing()),
                figure("0", rule, nothing()),
            ]);
        };
        if role == "fighter" {
            w.fail(
                Kind::BadValue,
                weapon_name,
                format!(
                    "aircraft type {key} is a fighter and the list names a load for it (`{load}`)"
                ),
            );
            return None;
        }
        // The fit rows of the slot that name the load; rows for other
        // weapons are ignored, and the count is not read, so rows that
        // differ only in count agree.
        let rows_for = |slot: &str| -> Vec<Row> {
            self.index
                .rows(Table::AircraftFits, key, slot)
                .iter()
                .filter(|r| r.item == load)
                .map(|r| Row {
                    value: r.item.clone(),
                    ..r.clone()
                })
                .collect()
        };
        let (bombs, torpedoes) = (rows_for("bomb_load"), rows_for("torpedo_load"));
        let (slot, rows) = match (bombs.is_empty(), torpedoes.is_empty()) {
            (false, true) => ("bomb_load", bombs),
            (true, false) => ("torpedo_load", torpedoes),
            (true, true) => {
                w.fail(
                    Kind::Missing,
                    weapon_name,
                    format!(
                        "aircraft type {key}: no bomb_load or torpedo_load fit row names `{load}`"
                    ),
                );
                return None;
            }
            (false, false) => {
                w.fail(
                    Kind::BadValue,
                    weapon_name,
                    format!(
                        "aircraft type {key}: `{load}` is named in both bomb_load and torpedo_load rows"
                    ),
                );
                return None;
            }
        };
        let read = match resolve(
            &rows,
            &format!("{slot} fit of aircraft type {key}, item {load}"),
            self.date,
            &self.rules.exceptions,
        ) {
            Ok(read) => read,
            Err(failure) => {
                w.failed(weapon_name, failure);
                return None;
            }
        };
        let (cites, excepted, doubts) = w.take(&read);
        let fit = Got {
            value: Fit {
                count: 0,
                item: load.to_string(),
            },
            cites,
            excepted,
            doubts,
        };
        let (kind, attribute, rule) = if slot == "bomb_load" {
            (
                "bomb",
                "weight_kg",
                "weight_kg of the load / gun_shell_kg_per_damage",
            )
        } else {
            (
                "torpedo",
                "warhead_kg",
                "warhead_kg of the load x torpedo_damage_per_warhead_kg",
            )
        };
        let got = self.weapon_fact(w, &fit, attribute, damage_name)?;
        let damage = if slot == "bomb_load" {
            got.value.div(self.rules.gun_shell_kg_per_damage)
        } else {
            got.value.mul(self.rules.torpedo_damage_per_warhead_kg)
        };
        let Some(damage) = damage else {
            w.failed(damage_name, overflow("attack damage"));
            return None;
        };
        let weapon_rule = format!("the load named in the list, which the {slot} fit rows of the type hold: weapon `{kind}`");
        let mut weapon = figure(kind, weapon_rule, fit.bare());
        weapon.notes.push(format!("load {load}"));
        let mut damage_figure = figure(damage.to_string(), rule, got.bare());
        damage_figure.notes.push(format!("load {load}"));
        Some([weapon, damage_figure])
    }

    /// Derives every figure of an aircraft type. All problems are returned.
    pub fn derive_aircraft(&self, entry: &AircraftEntry) -> Result<AircraftResult, Vec<Problem>> {
        let key = entry.type_key.as_str();
        let mut w = Work {
            subject: key,
            problems: Vec::new(),
            used: Vec::new(),
        };
        let Some(info) = self.aircraft.get(key) else {
            w.fail(
                Kind::UnknownType,
                "aircraft",
                "no such aircraft type key in aircraft_types.csv".to_string(),
            );
            return Err(w.problems);
        };
        let mut load_known = true;
        if let Some(load) = &entry.load {
            if !self.weapons.contains_key(load) {
                w.fail(
                    Kind::UnknownType,
                    "load",
                    format!("`{load}` is not a weapon key in weapons.csv"),
                );
                load_known = false;
            }
        }
        let mut figs: Vec<Option<AircraftFigure>> = vec![None; AIRCRAFT_FIGURES.len()];

        // role
        let role = ROLE_FOR_FACTS_ROLE
            .iter()
            .find(|(facts, _)| *facts == info.role)
            .map(|(_, role)| *role);
        match role {
            Some(role) => {
                let got = Got {
                    value: (),
                    cites: BTreeSet::from([(
                        "aircraft_types.csv".to_string(),
                        info.row_id.parse().unwrap_or(0),
                    )]),
                    excepted: false,
                    doubts: BTreeSet::new(),
                };
                figs[ROLE] = Some(figure(
                    role,
                    "role of the type in aircraft_types.csv, mapped (fighter: fighter; dive_bomber, torpedo_bomber, heavy_bomber, land_attack_bomber: bomber; patrol_flying_boat, patrol_amphibian, floatplane_reconnaissance: patrol)",
                    got,
                ));
            }
            None => w.fail(
                Kind::UnmappedType,
                AIRCRAFT_FIGURES[ROLE],
                format!(
                    "aircraft type {key} (aircraft_types.csv:{}) has role `{}`, which has no simulation role",
                    info.row_id, info.role
                ),
            ),
        }

        // radius, search and strike
        if let Some((radius, rule, got, notes)) = self.radius(&mut w, key) {
            for i in [SEARCH, STRIKE] {
                let mut fig = figure(radius.to_string(), rule.clone(), got.clone());
                fig.notes.extend(notes.iter().cloned());
                figs[i] = Some(fig);
            }
        }

        // sweep width
        if let Some(sweep) = self.rules.search_sweep_width_nm {
            figs[SWEEP] = Some(figure(
                sweep.to_string(),
                "search_sweep_width_nm of the rules file: a placeholder, no source gives a sweep width (decision 0029, 10)",
                nothing(),
            ));
        }

        // attack
        if let (Some(role), true) = (role, load_known) {
            if let Some([weapon, damage]) = self.attack(&mut w, key, role, entry.load.as_deref()) {
                figs[WEAPON] = Some(weapon);
                figs[DAMAGE] = Some(damage);
            }
        }

        // protection
        for (i, attribute) in [(ARMOUR, "pilot_armour"), (TANKS, "self_sealing_tanks")] {
            if let Some((value, got)) = self.yes_no(&mut w, key, attribute) {
                figs[i] = Some(figure(
                    value.to_string(),
                    format!("{attribute} of the type: yes is 1, no is 0 (decision 0029, 11)"),
                    got,
                ));
            }
        }

        if !w.problems.is_empty() {
            return Err(w.problems);
        }
        Ok(AircraftResult {
            name: info.name.clone(),
            figures: figs
                .into_iter()
                .map(|f| f.expect("every figure is set when there is no problem"))
                .collect(),
            used: w.used,
        })
    }
}
