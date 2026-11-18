//! The force assembled by the staff (decision 0149, step 7 of stage 3).
//!
//! A "take a place" whose forces limit does not name all three of the task
//! forces, the land units and the source has the rest chosen by the staff,
//! on the first morning its plan is preparing, in the order a staff works
//! in (items 6 to 11):
//!
//! - **The enemy:** the side's estimate and the men wanted by the risk
//!   (decision 0142, 10), in [`Day::choose`]'s first line.
//! - **The source** (item 7): each base of the side the staff may take, in
//!   base order, where a transport force it may take lies stopped and a
//!   fighting unit it may take has all its men ashore. For each the staff
//!   works the lift: the transport force of item 9 and the units of item 8
//!   there, by [`Day::choose`]. The source is the base whose lift gives the
//!   men wanted with the shortest passage by the engine's forecast; where
//!   none gives them, the one that gives the most men; then the lower
//!   number.
//! - **The risk on the way** (item 10): the forces on the bombardment
//!   mission stopped at a base of the side, those on the carrier mission
//!   anywhere, and those on the escort mission stopped at the source, that
//!   the staff may take.
//!
//! What the staff may take (item 5): the side's, usable that day (decision
//! 0140), not held by the commander, and not named in the forces limit of
//! an intent in force served before the end, nor addressed by a task of one.
//!
//! What it chooses is kept as the forces limit of the "ready a landing" it
//! forms: only what the commander did not name (item 3). The rest of the
//! end is decisions 0142 to 0148, reading the limit as it binds
//! ([`sim_core::command::CommandState::forces_as_bound`]).

use std::collections::{BTreeMap, BTreeSet};

use sim_core::command::{Addressee, ForcesLimit, Intent, IntentKind, Risk};
use sim_core::fixed::Fixed;
use sim_core::hex::Hex;
use sim_core::ids::{BaseId, EntityRef, IntentId, LandUnitId, ShipId, TaskForceId};
use sim_core::orders::{Mission, SpeedSetting};
use sim_core::state::{LandUnit, ShipLocation, TaskForce};

use crate::air::Day;
use crate::reasons::{base_words, hold_words};
use crate::take::{men_per_man, unit_words, Choice};

/// What the staff chose, with the limit it sets on the task it forms.
pub(crate) struct Assembled {
    pub(crate) choice: Choice,
    pub(crate) source: BaseId,
    pub(crate) source_hex: Hex,
    /// The limit the staff sets: what the commander did not name.
    pub(crate) limit: ForcesLimit,
}

/// Why nothing can be assembled: the choice's words as far as they go, and
/// the working.
pub(crate) struct NoSource {
    pub(crate) evidence: String,
    pub(crate) worked: String,
}

/// The intents served before the end, out of its own tree, and what each
/// names: what the staff may not take for the end (item 5).
#[derive(Default)]
pub(crate) struct Others {
    forces: BTreeMap<TaskForceId, IntentId>,
    units: BTreeMap<LandUnitId, IntentId>,
    bases: BTreeMap<BaseId, IntentId>,
}

/// One base's lift (item 7).
struct Lift {
    base: BaseId,
    hex: Hex,
    berths: u32,
    hours: Fixed,
    choice: Choice,
}

/// Whether a forces limit names task forces (or ships).
pub(crate) fn names_forces(limit: &ForcesLimit) -> bool {
    !(limit.task_forces.is_empty() && limit.ships.is_empty())
}

/// Whether the commander's limit names all three of the task forces, the
/// land units and the source: then the end is decision 0142's as it was.
pub(crate) fn names_all(limit: Option<&ForcesLimit>) -> bool {
    limit.is_some_and(|f| names_forces(f) && !f.land_units.is_empty() && f.source.is_some())
}

fn list<T: std::fmt::Display>(items: impl IntoIterator<Item = T>) -> String {
    let words: Vec<String> = items.into_iter().map(|i| i.to_string()).collect();
    match words.len() {
        0 => "none".to_string(),
        _ => words.join(" and "),
    }
}

/// The limit in words: "task_force:1 and task_force:3, land_unit:1 and
/// base:1 as the source".
pub(crate) fn limit_words(limit: &ForcesLimit) -> String {
    let mut parts: Vec<String> = Vec::new();
    if !limit.task_forces.is_empty() {
        parts.push(list(&limit.task_forces));
    }
    if !limit.land_units.is_empty() {
        parts.push(list(&limit.land_units));
    }
    if let Some(source) = limit.source {
        parts.push(format!("{source} as the source"));
    }
    if parts.is_empty() {
        "nothing".to_string()
    } else {
        parts.join(", ")
    }
}

impl Day<'_, '_> {
    /// Item 5: what the intents served before `end` name.
    pub(crate) fn others_above(&self, end: &Intent) -> Others {
        let mut others = Others::default();
        let root = self.state.root_of(end.id);
        for id in self.in_priority_order(|_| true) {
            if id == end.id {
                break;
            }
            if self.state.root_of(id) == root {
                continue;
            }
            let Some(intent) = self.state.intents.get(&id) else {
                continue;
            };
            if let Addressee::TaskForce(force) = intent.addressee {
                others.forces.entry(force).or_insert(id);
            }
            if let Some(forces) = self.state.forces_as_bound(intent) {
                for force in &forces.task_forces {
                    others.forces.entry(*force).or_insert(id);
                }
                for unit in &forces.land_units {
                    others.units.entry(*unit).or_insert(id);
                }
                if let Some(base) = forces.source {
                    others.bases.entry(base).or_insert(id);
                }
            }
        }
        others
    }

    /// Item 5 for a task force: `Err(words)` where the staff may not take it.
    pub(crate) fn may_take_force(&self, force: &TaskForce, others: &Others) -> Result<(), String> {
        if let Some(day) = crate::available::force_waiting(self.view, force.id) {
            return Err(crate::available::named_with_day(force.id, day));
        }
        if let Some(hold) = self.state.hold_on_force(force.id, &force.ships) {
            let words = self
                .state
                .hold_of(hold)
                .map_or_else(|| format!("{hold} is held"), |h| hold_words(hold, *h));
            return Err(format!("{} is left out: {words}", force.id));
        }
        if let Some(other) = others.forces.get(&force.id) {
            return Err(format!(
                "{} serves {other}, an intent higher in priority",
                force.id
            ));
        }
        Ok(())
    }

    /// Item 5 for a land unit.
    fn may_take_unit(&self, unit: &LandUnit, others: &Others) -> Result<(), String> {
        if let Some(day) = crate::available::not_before(self.view, EntityRef::LandUnit(unit.id)) {
            return Err(crate::available::named_with_day(unit_words(unit), day));
        }
        let hold = sim_core::command::HoldTarget::LandUnit(unit.id);
        if let Some(found) = self.state.hold_of(hold) {
            return Err(hold_words(hold, *found));
        }
        if let Some(other) = others.units.get(&unit.id) {
            return Err(format!(
                "{} serves {other}, an intent higher in priority",
                unit_words(unit)
            ));
        }
        Ok(())
    }

    /// Item 5 for a base.
    pub(crate) fn may_take_base(&self, base: BaseId, others: &Others) -> Result<(), String> {
        if let Some(day) = crate::available::not_before(self.view, EntityRef::Base(base)) {
            return Err(crate::available::named_with_day(
                base_words(self.view, base),
                day,
            ));
        }
        if let Some(other) = others.bases.get(&base) {
            return Err(format!(
                "{} is the source of {other}, an intent higher in priority",
                base_words(self.view, base)
            ));
        }
        Ok(())
    }

    /// The berths of a force: its ships afloat that carry men.
    fn berths_of(&self, force: TaskForceId) -> u32 {
        self.view
            .ships()
            .filter(|s| s.location == ShipLocation::TaskForce(force))
            .filter_map(|s| self.view.ship_class(s.class))
            .map(|c| c.troop_capacity_men)
            .sum()
    }

    /// Whether a force has a ship afloat.
    fn has_ship_afloat(&self, force: TaskForceId) -> bool {
        self.view
            .ships()
            .any(|s| s.location == ShipLocation::TaskForce(force))
    }

    /// Items 5 to 12: the force the staff assembles for `end`, inside what
    /// the commander named (`commander`), or why it can assemble none.
    #[allow(clippy::too_many_lines)]
    pub(crate) fn assemble(
        &self,
        end: &Intent,
        commander: Option<&ForcesLimit>,
        place: Hex,
        risk: Risk,
    ) -> Result<Assembled, NoSource> {
        let view = self.view;
        let others = self.others_above(end);
        let named_forces = commander.filter(|f| names_forces(f));
        let named_units = commander.filter(|f| !f.land_units.is_empty());
        let named_source = commander.and_then(|f| f.source);
        let mut not_taken: Vec<String> = Vec::new();

        // The forces it may take, of those the commander named, or of all.
        let mut forces: Vec<TaskForce> = Vec::new();
        for force in view.task_forces() {
            let named = named_forces.is_none_or(|f| {
                f.task_forces.contains(&force.id)
                    || (!force.ships.is_empty() && force.ships.iter().all(|s| f.ships.contains(s)))
            });
            if !named || !self.has_ship_afloat(force.id) {
                continue;
            }
            match self.may_take_force(force, &others) {
                Ok(()) => forces.push(force.clone()),
                Err(words) => not_taken.push(words),
            }
        }
        forces.sort_by_key(|f| f.id);

        // The fighting units it may take.
        let mut units: Vec<LandUnit> = Vec::new();
        for unit in view.land_units() {
            if unit.working {
                continue;
            }
            if named_units.is_some_and(|f| !f.land_units.contains(&unit.id)) {
                continue;
            }
            match self.may_take_unit(unit, &others) {
                Ok(()) => units.push(unit.clone()),
                Err(words) => not_taken.push(words),
            }
        }
        units.sort_by_key(|u| u.id);

        // Item 6: the enemy and the men wanted.
        let estimate = view.ground_estimate(place).map(|e| e.men);
        let wanted = estimate.map(|men| men * men_per_man(risk));

        // Decision 0152, 2: the lifts as they stand with a fighting unit
        // aboard come before any base; the first is the lowest-numbered.
        let (stand, stand_words) = self.first_lift_as_it_stands(end, &forces, &units, place, risk);

        // Item 7: each base that can be the source, and its lift.
        let mut bases: Vec<(BaseId, Hex)> = view
            .bases()
            .filter(|b| named_source.is_none_or(|s| s == b.id))
            .filter(|_| stand.is_none())
            .map(|b| (b.id, b.hex))
            .collect();
        bases.sort_by_key(|(id, _)| *id);
        let mut lifts: Vec<Lift> = Vec::new();
        let mut missing: Vec<String> = Vec::new();
        let mut any_transport = false;
        for (base, hex) in bases {
            let words = base_words(view, base);
            if let Err(why) = self.may_take_base(base, &others) {
                missing.push(why);
                continue;
            }
            let transports: Vec<&TaskForce> = forces
                .iter()
                .filter(|f| f.mission == Mission::Transport)
                .filter(|f| self.berths_of(f.id) > 0)
                .filter(|f| crate::fleet::force_stopped_at(f, hex))
                .collect();
            let here: BTreeSet<LandUnitId> = units
                .iter()
                .filter(|u| {
                    u.men_ashore > 0
                        && u.hex == Some(hex)
                        && view.men_aboard(u.id).unwrap_or(0) == 0
                })
                .map(|u| u.id)
                .collect();
            any_transport |= !transports.is_empty();
            match (transports.is_empty(), here.is_empty()) {
                (true, true) => {
                    missing.push(format!(
                        "{words}: no transport force it may take lies stopped there, and no fighting unit it may take is ashore there"
                    ));
                    continue;
                }
                (true, false) => {
                    missing.push(format!(
                        "{words}: no transport force it may take lies stopped there"
                    ));
                    continue;
                }
                (false, true) => {
                    missing.push(format!(
                        "{words}: no fighting unit it may take is ashore beside its transport force"
                    ));
                    continue;
                }
                (false, false) => {}
            }
            // Item 9: the transport force.
            let force = match wanted {
                Some(want) => transports
                    .iter()
                    .find(|f| self.berths_of(f.id) >= want)
                    .copied(),
                None => None,
            }
            .or_else(|| {
                transports
                    .iter()
                    .max_by_key(|f| (self.berths_of(f.id), std::cmp::Reverse(f.id)))
                    .copied()
            });
            let Some(force) = force else {
                continue;
            };
            let berths = self.berths_of(force.id);
            // Item 8: the units, by decision 0142, 8 and 9.
            let limit = ForcesLimit {
                ships: BTreeSet::new(),
                task_forces: BTreeSet::from([force.id]),
                air_groups: BTreeSet::new(),
                source: Some(base),
                land_units: here,
            };
            let choice = match self.choose(end, &limit, place, hex, risk, false) {
                Ok(choice) => choice,
                Err((evidence, _)) => {
                    missing.push(format!("{words}: {evidence}"));
                    continue;
                }
            };
            let hours = view
                .forecast_passage(force.id, place, Some(SpeedSetting::Cruise))
                .filter(|f| f.refusal.is_none())
                .map(|f| f.hours);
            let Some(hours) = hours else {
                missing.push(format!("{words}: {} has no passage to {place}", force.id));
                continue;
            };
            lifts.push(Lift {
                base,
                hex,
                berths,
                hours,
                choice,
            });
        }

        let enemy = match (estimate, wanted) {
            (Some(est), Some(want)) => format!(
                "the side's estimate is {est} enemy men at {place}; at {} the staff wants {} men for each, {want} men",
                risk.name(),
                men_per_man(risk)
            ),
            _ => format!(
                "the side holds no estimate of the enemy's men at {place}: the staff takes as many as can be carried"
            ),
        };
        let mut lift_words: Vec<String> = lifts
            .iter()
            .map(|l| {
                format!(
                    "{}: {} with {} berths, its lift gives {} men, {} hours to {place}",
                    base_words(view, l.base),
                    l.choice.force,
                    l.berths,
                    l.choice.men,
                    l.hours
                )
            })
            .collect();
        lift_words.extend(missing.iter().cloned());
        let not_taken_words = if not_taken.is_empty() {
            String::new()
        } else {
            format!("; the staff may not take: {}", not_taken.join("; "))
        };

        let stand_line = if stand_words.is_empty() {
            String::new()
        } else {
            format!(
                "; the forces with men aboard, in force order: {}",
                stand_words.join("; ")
            )
        };
        // Item 13: no base can be the source.
        if lifts.is_empty() && stand.is_none() {
            let lack = if any_transport {
                "no base has a transport force the staff may take stopped there with a fighting unit it may take ashore beside it"
            } else {
                "no transport force the staff may take lies stopped at a base of the side"
            };
            let evidence = format!(
                "{enemy}{stand_line}; no base can be the source: {lack}: {}{not_taken_words}",
                if lift_words.is_empty() {
                    "the side has no base the staff may take".to_string()
                } else {
                    lift_words.join("; ")
                }
            );
            let worked = format!(
                "{evidence}. The staff forms nothing under {}: it wants a transport force stopped at a base of the side, with a fighting unit beside it.",
                end.id
            );
            return Err(NoSource { evidence, worked });
        }

        // Item 7: the source.
        let enough: Vec<&Lift> = match wanted {
            Some(want) => lifts.iter().filter(|l| l.choice.men >= want).collect(),
            None => Vec::new(),
        };
        let (chosen, why) = if let Some(found) = &stand {
            (
                found,
                "a lift as it stands with a fighting unit aboard comes before any base, and its source is its own",
            )
        } else if enough.is_empty() {
            let best = lifts
                .iter()
                .max_by_key(|l| {
                    (
                        l.choice.men,
                        std::cmp::Reverse(if wanted.is_none() {
                            l.hours
                        } else {
                            Fixed::ZERO
                        }),
                        std::cmp::Reverse(l.base),
                    )
                })
                .expect("a lift");
            let equal = lifts
                .iter()
                .filter(|l| l.choice.men == best.choice.men)
                .count()
                > 1;
            let why = match (wanted, equal) {
                (Some(_), false) => "no base's lift gives the men wanted, and its gives the most men",
                (Some(_), true) => "no base's lift gives the men wanted; of those that give the most men it is the lower numbered",
                (None, _) => "with no count of the enemy, its lift gives the most men, then the shortest passage",
            };
            (best, why)
        } else {
            let best = enough
                .iter()
                .min_by_key(|l| (l.hours, l.base))
                .copied()
                .expect("a lift");
            (
                best,
                "its lift gives the men wanted with the shortest passage",
            )
        };
        let source = chosen.base;
        let source_hex = chosen.hex;
        let transport = chosen.choice.force;
        let source_line = if stand.is_some() {
            format!(
                "the forces with men aboard, in force order: {}; the first lift is {transport}, as it stands, and the source is {}: {why}{not_taken_words}",
                stand_words.join("; "),
                base_words(view, source)
            )
        } else {
            format!(
                "the bases that could be the source, in base order: {}; the source is {}: {why}{not_taken_words}",
                lift_words.join("; "),
                base_words(view, source)
            )
        };

        // Item 10: the risk on the way.
        // Under "control an island" a take takes no carrier force and no
        // force on the bombardment mission: the carriers are the island's,
        // under its denial, and the bombardment forces under its
        // neutralising (decision 0151, 8a and 8c).
        let under_island = self.state.island_on_chain(end.id).is_some();
        let base_hexes: BTreeSet<Hex> = view.bases().map(|b| b.hex).collect();
        let bombard_stopped: Vec<TaskForceId> = forces
            .iter()
            .filter(|f| f.mission == Mission::Bombardment)
            .filter(|f| {
                base_hexes
                    .iter()
                    .any(|h| crate::fleet::force_stopped_at(f, *h))
            })
            .map(|f| f.id)
            .collect();
        let bombard: Vec<TaskForceId> = if under_island {
            Vec::new()
        } else {
            bombard_stopped.clone()
        };
        let carrier: Vec<TaskForceId> = forces
            .iter()
            .filter(|f| f.mission == Mission::Carrier && !under_island)
            .map(|f| f.id)
            .collect();
        let escort: Vec<TaskForceId> = forces
            .iter()
            .filter(|f| f.mission == Mission::Escort)
            .filter(|f| crate::fleet::force_stopped_at(f, source_hex))
            .map(|f| f.id)
            .collect();
        let risk_line = [
            if under_island && !bombard_stopped.is_empty() {
                "the bombardment forces are the island's, under its neutralising".to_string()
            } else if bombard.is_empty() {
                "no force on the bombardment mission to bombard the place".to_string()
            } else {
                format!("to bombard the place: {}", list(&bombard))
            },
            if under_island {
                "the carrier forces are the island's, under its denial".to_string()
            } else if carrier.is_empty() {
                "no force on the carrier mission to cover the landing".to_string()
            } else {
                format!("to cover the landing: {}", list(&carrier))
            },
            if escort.is_empty() {
                format!(
                    "no force on the escort mission at {} to escort it",
                    base_words(view, source)
                )
            } else {
                format!("to escort it: {}", list(&escort))
            },
        ]
        .join("; ");

        let mut choice = chosen.choice.clone();
        let mut lines = vec![enemy, source_line];
        lines.extend(choice.lines.iter().skip(1).cloned());
        // Decision 0152, 3: the working units aboard a lift as it stands
        // land with it, and the reason names them.
        if stand.is_some() {
            if let Some(force) = forces.iter().find(|f| f.id == transport) {
                let picked: Vec<LandUnitId> = choice.picks.iter().map(|p| p.id).collect();
                let extra = self.also_aboard(force, &picked);
                let extra = extra.trim().trim_end_matches('.');
                if !extra.is_empty() {
                    lines.push(extra.to_string());
                }
            }
        }
        if let (Some((want, have)), false) = (choice.short, risk == Risk::Preserve) {
            lines.push(format!(
                "short by {} men: the lift gives {have} men against the {want} wanted",
                want - have
            ));
        }
        lines.push(risk_line);
        choice.lines = lines;

        // Item 3: the limit, of what the commander did not name. Under an
        // island, where what he named is the island's limit and not the
        // take's, the staff's limit names the whole choice, so that the take
        // reads it as it binds (decision 0151, 4).
        let names_whole = end.limits.forces.is_none() && under_island;
        let named_forces = named_forces.filter(|_| !names_whole);
        let named_units = named_units.filter(|_| !names_whole);
        let named_source = named_source.filter(|_| !names_whole);
        let picked: BTreeSet<LandUnitId> = choice.picks.iter().map(|p| p.id).collect();
        let mut task_forces: BTreeSet<TaskForceId> = BTreeSet::new();
        if named_forces.is_none() {
            task_forces.insert(transport);
            task_forces.extend(bombard.iter().copied());
            task_forces.extend(carrier.iter().copied());
            task_forces.extend(escort.iter().copied());
        }
        let land_units = match named_units {
            None => picked,
            Some(f) if f.land_units == picked => BTreeSet::new(),
            Some(_) => picked,
        };
        let limit = ForcesLimit {
            ships: BTreeSet::<ShipId>::new(),
            task_forces,
            air_groups: BTreeSet::new(),
            source: named_source.is_none().then_some(source),
            land_units,
        };
        Ok(Assembled {
            choice,
            source,
            source_hex,
            limit,
        })
    }

    /// Decision 0152, 1 and 2: the first lift as it stands with a fighting
    /// unit aboard, of the forces the staff may take (`forces`) and the
    /// fighting units it may take (`units`), in force order; with the words
    /// for each force with men aboard. Its units are its fighting units
    /// aboard, whole; its source is its own (item 1).
    fn first_lift_as_it_stands(
        &self,
        end: &Intent,
        forces: &[TaskForce],
        units: &[LandUnit],
        place: Hex,
        risk: Risk,
    ) -> (Option<Lift>, Vec<String>) {
        let mut words: Vec<String> = Vec::new();
        for force in forces {
            if !self.is_lift(force) {
                continue;
            }
            let aboard = self.aboard_words(force.id);
            if let Some(take) = self.lift_of_take(force.id, Some(end.id)) {
                words.push(format!("{} is the lift of {take}", force.id));
                continue;
            }
            if let Some(other) = self.lifts_sent.get(&force.id) {
                words.push(format!("{} is sent by {other} this morning", force.id));
                continue;
            }
            if let Some(why) = Self::not_free(force) {
                words.push(why);
                continue;
            }
            let fighting: BTreeSet<LandUnitId> = self
                .aboard(force.id)
                .into_iter()
                .filter(|(u, _)| !u.working && units.iter().any(|t| t.id == u.id))
                .map(|(u, _)| u.id)
                .collect();
            if fighting.is_empty() {
                words.push(format!(
                    "{} has {aboard} aboard, and no fighting unit the staff may take",
                    force.id
                ));
                continue;
            }
            let Some((base, hex)) = self.lift_source(force) else {
                words.push(format!(
                    "{} has no base of the side to come back to",
                    force.id
                ));
                continue;
            };
            let limit = ForcesLimit {
                ships: BTreeSet::new(),
                task_forces: BTreeSet::from([force.id]),
                air_groups: BTreeSet::new(),
                source: Some(base),
                land_units: fighting,
            };
            let choice = match self.choose(end, &limit, place, hex, risk, true) {
                Ok(choice) => choice,
                Err((evidence, _)) => {
                    words.push(format!("{}: {evidence}", force.id));
                    continue;
                }
            };
            let Some(hours) = self
                .view
                .forecast_passage(force.id, place, Some(SpeedSetting::Cruise))
                .filter(|f| f.refusal.is_none())
                .map(|f| f.hours)
            else {
                words.push(format!("{} has no passage to {place}", force.id));
                continue;
            };
            words.push(format!(
                "{} has {aboard} aboard, a lift as it stands, {hours} hours to {place}",
                force.id
            ));
            let berths = self.berths_of(force.id);
            return (
                Some(Lift {
                    base,
                    hex,
                    berths,
                    hours,
                    choice,
                }),
                words,
            );
        }
        (None, words)
    }

    /// The limit the units are chosen again on (decision 0147, 16, and
    /// decision 0149, 4): the end's limit as it binds, and where the
    /// commander named no land unit, every fighting unit the staff may take
    /// ashore at the source besides those chosen. The source and the force
    /// stand: the task formed is the force's.
    pub(crate) fn rechoice_limit(&self, end: &Intent, task: &Intent) -> Option<ForcesLimit> {
        let mut bound = self.state.forces_as_bound(end)?.into_owned();
        if names_all(end.limits.forces.as_ref()) {
            return Some(bound);
        }
        // The force stands: the task formed is the force's.
        if let IntentKind::ReadyLanding { force, .. } = task.kind {
            bound.task_forces = BTreeSet::from([force]);
        }
        if end
            .limits
            .forces
            .as_ref()
            .is_some_and(|f| !f.land_units.is_empty())
        {
            return Some(bound);
        }
        let source_hex = bound
            .source
            .and_then(|s| self.view.bases().find(|b| b.id == s))
            .map(|b| b.hex)?;
        let others = self.others_above(end);
        for unit in self.view.land_units() {
            if !unit.working
                && unit.men_ashore > 0
                && unit.hex == Some(source_hex)
                && self.may_take_unit(unit, &others).is_ok()
            {
                bound.land_units.insert(unit.id);
            }
        }
        Some(bound)
    }
}
