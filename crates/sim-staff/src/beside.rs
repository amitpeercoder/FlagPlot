//! Men ashore beside a place (decision 0229, as amended on review): under
//! "control an island" the staff feeds the side's men at a base of the side
//! that stands on the island's land and in no named place's hex, as well as
//! the men at the named places.
//!
//! - **The island's land** (rule 1) is the hexes of its named places and
//!   every hex joined to one of them by land links, through any number of
//!   hexes. It is read from the map, which both sides hold: no state, no hash.
//! - **A base beside a place** (rules 2 to 4): each morning, after the takes
//!   and the holds are formed and before the denial, each base of the side,
//!   in base order, on the island's land and in no place's hex, that the
//!   staff may take and that is not the island's source, in whose hex the side
//!   has men ashore, or to whose hex a unit of the side is marching (both
//!   read from its own units), gets a "keep supplied" under the island, with
//!   the floor of the island's risk in days, no interval and a plan of its
//!   own, unless one for a base in that hex is in force anywhere on the side.
//!   Its forces limit is set by the island's pools, as a hold's is. With no
//!   source it forms none and says so.
//! - **It ends** (rule 6, as amended) on the first morning no man of the side
//!   is ashore in the hex, none is marching to it and no run of its plan is
//!   loaded or at sea.
//! - **Men at a hex with no base** of the side (rule 8) cannot be fed: the
//!   staff gives no order and records one reason for the hex each morning.
//!
//! The doctrine reads the side's own view, its own units and the map, and
//! nothing of the enemy.

use std::collections::{BTreeMap, BTreeSet};

use sim_core::command::{CommandKind, Intent, IntentKind, Maker, Plan, StaffCommand};
use sim_core::hex::Hex;
use sim_core::ids::{BaseId, LandUnitId};

use crate::air::Day;
use crate::fleet::{run_state, Fleet, RunStatus};
use crate::holding::margin_days;
use crate::island::Source;
use crate::reasons::{base_words, serving};

/// Rule 1: the hexes of `places` and every hex joined to one of them by land
/// links, through any number of hexes. `neighbours` gives the hexes joined to
/// a hex by a land link.
pub(crate) fn island_land(places: &[Hex], neighbours: impl Fn(Hex) -> Vec<Hex>) -> BTreeSet<Hex> {
    let mut land: BTreeSet<Hex> = BTreeSet::new();
    let mut next: Vec<Hex> = places.to_vec();
    while let Some(hex) = next.pop() {
        if !land.insert(hex) {
            continue;
        }
        for other in neighbours(hex) {
            if !land.contains(&other) {
                next.push(other);
            }
        }
    }
    land
}

/// One unit of the side with men ashore, as the rule reads it.
#[derive(Clone, Debug)]
struct Ashore {
    id: LandUnitId,
    name: String,
    men: u32,
    /// The hex its men are recorded in.
    at: Hex,
    /// The hex it marches to, where a march stands.
    marching_to: Option<Hex>,
}

impl Ashore {
    fn words(&self) -> String {
        format!("{} ({}), {} men", self.id, self.name, self.men)
    }
}

impl Day<'_, '_> {
    /// The side's own units with men ashore, in unit order.
    fn men_ashore(&self) -> Vec<Ashore> {
        let mut found: Vec<Ashore> = self
            .view
            .land_units()
            .filter(|u| u.men_ashore > 0)
            .filter_map(|u| {
                Some(Ashore {
                    id: u.id,
                    name: u.name.clone(),
                    men: u.men_ashore,
                    at: u.hex?,
                    marching_to: u.march.map(|m| m.to),
                })
            })
            .collect();
        found.sort_by_key(|u| u.id);
        found
    }

    /// Rule 2, as amended: the units with men ashore in `hex` or marching to
    /// it.
    fn men_beside(men: &[Ashore], hex: Hex) -> Vec<Ashore> {
        men.iter()
            .filter(|u| u.at == hex || u.marching_to == Some(hex))
            .cloned()
            .collect()
    }

    /// The hex of a base on the side's map, the side's or the enemy's.
    fn hex_of_base(&self, base: BaseId) -> Option<Hex> {
        self.view
            .bases()
            .find(|b| b.id == base)
            .map(|b| b.hex)
            .or_else(|| {
                self.view
                    .enemy_bases()
                    .find(|b| b.id == base)
                    .map(|b| b.hex)
            })
    }

    /// The base's supply and its days at today's use, in words.
    fn supply_words(&self, hex: Hex) -> String {
        match self.view.days_of_supply(hex) {
            Some(figure) => match figure.days {
                Some(days) => format!(
                    "{} t of supply, {} {} at today's use",
                    figure.stock_t,
                    crate::units::floor_whole(days),
                    crate::units::days_word(crate::units::floor_whole(days))
                ),
                None => format!("{} t of supply and no planned use", figure.stock_t),
            },
            None => "no supply figure".to_string(),
        }
    }

    /// Rule 6, as amended: each "keep supplied" the staff formed directly
    /// under the island whose base has no man ashore in its hex, none marching
    /// to it and no run of its plan loaded or at sea is withdrawn.
    pub(crate) fn withdraw_beside(&mut self, end: &Intent, plan: &Plan, in_scope: bool) {
        let today = self.today;
        let mine: Vec<Intent> = self
            .state
            .intents_in_force()
            .filter(|i| {
                i.parent == Some(end.id)
                    && i.maker == Maker::Staff
                    && matches!(i.kind, IntentKind::KeepSupplied { .. })
            })
            .cloned()
            .collect();
        if mine.is_empty() {
            return;
        }
        let men = self.men_ashore();
        let fleet = Fleet::of(self.view);
        for intent in mine {
            let IntentKind::KeepSupplied { base, .. } = intent.kind else {
                continue;
            };
            let hex = self.hex_of_base(base);
            if let Some(hex) = hex {
                if !Self::men_beside(&men, hex).is_empty() {
                    continue;
                }
            }
            // A run of its plan loaded or at sea keeps it in force, so that
            // the supply doctrine brings the run home as any is.
            let busy = self.state.plans.get(&intent.plan).is_some_and(|own| {
                own.runs.values().any(|run| {
                    self.view
                        .bases()
                        .find(|b| b.id == run.source)
                        .is_some_and(|source| {
                            run_state(&fleet, &self.state, own.id, run, source.hex, today).status
                                != RunStatus::Finished
                        })
                })
            });
            if busy {
                continue;
            }
            let base_text = base_words(self.view, base);
            let where_text = hex.map_or_else(String::new, |h| format!(" at {h}"));
            let reason = serving(end, plan)
                .evidence(
                    format!("no man of the side is ashore at {base_text}{where_text} and none is marching to it"),
                    today,
                )
                .evidence(
                    format!("no run of {} is loaded or at sea", intent.plan),
                    today,
                )
                .binding(format!("{} has done its work", intent.id))
                .worked(format!(
                    "{} formed {} to keep {base_text} supplied beside a place of the island; no man of the side is ashore{where_text}, none is marching there and no run of {} is loaded or at sea, so the staff withdraws {}.",
                    end.id, intent.id, intent.plan, intent.id
                ))
                .would_change("a man of the side ashore at the base, or marching to it")
                .done();
            if in_scope {
                self.lines.push(format!(
                    "{} would withdraw {}: no man is at {base_text}.",
                    end.id, intent.id
                ));
            }
            self.target(in_scope).commands.push(StaffCommand {
                kind: CommandKind::WithdrawIntent { intent: intent.id },
                reason,
            });
            for below in self.state.descendants_in_force(intent.id) {
                if let Some(found) = self.state.intents.get_mut(&below) {
                    found.withdrawn_day = Some(today);
                }
            }
            if let Some(found) = self.state.intents.get_mut(&intent.id) {
                found.withdrawn_day = Some(today);
            }
        }
    }

    /// Rules 2 to 4, 8 and 10: the supply of each base beside a place, and the
    /// reason for the men at a hex with no base.
    #[allow(clippy::too_many_arguments, clippy::too_many_lines)]
    pub(crate) fn island_beside(
        &mut self,
        end: &Intent,
        plan: &Plan,
        place_hexes: &[Hex],
        source: &Source,
        island_text: &str,
        in_scope: bool,
    ) {
        let today = self.today;
        let land = island_land(place_hexes, |hex| self.view.land_neighbours(hex));
        let men = self.men_ashore();
        if men.is_empty() {
            return;
        }
        let others = self.others_above(end);
        let source_base = source.chosen_base();
        let mut bases: Vec<sim_core::state::Base> = self.view.bases().cloned().collect();
        bases.sort_by_key(|b| b.id);
        for base in bases {
            if !land.contains(&base.hex)
                || place_hexes.contains(&base.hex)
                || source_base == Some(base.id)
                || self.may_take_base(base.id, &others).is_err()
            {
                continue;
            }
            let here = Self::men_beside(&men, base.hex);
            if here.is_empty() {
                continue;
            }
            let base_text = base_words(self.view, base.id);
            // Rule 3: a "keep supplied" for a base in the hex, anywhere on
            // the side, stands.
            let standing = self
                .state
                .intents_in_force()
                .find(|i| match i.kind {
                    IntentKind::KeepSupplied { base: b, .. } => {
                        self.hex_of_base(b) == Some(base.hex)
                    }
                    _ => false,
                })
                .cloned();
            if let Some(other) = standing {
                self.island_stands(
                    end,
                    plan,
                    &other,
                    &format!("keep a base in the hex of {base_text} supplied"),
                    in_scope,
                );
                continue;
            }
            // Rule 4: the island's source and its ships. With no source it
            // forms none and says so.
            if source.chosen_base().is_none() {
                let reason = serving(end, plan)
                    .evidence(source.words().to_string(), today)
                    .binding("no base can be the source of supply")
                    .worked(format!(
                        "{}: {} men of the side are ashore beside a place at {base_text}, {}; so the staff forms no supply for it.",
                        end.id,
                        here.iter().map(Ashore::words).collect::<Vec<_>>().join("; "),
                        source.words()
                    ))
                    .would_change("a base of the side off the island, that the staff may take, with supply above its reserve")
                    .done();
                self.target(in_scope).no_orders.push(reason);
                continue;
            }
            let risk = self.state.risk_read_up(end.id);
            let floor_days = margin_days(risk);
            let kind = IntentKind::KeepSupplied {
                base: base.id,
                floor_days,
                interval_days: None,
            };
            let beside = place_hexes
                .iter()
                .map(|p| (p.distance(base.hex), *p))
                .min()
                .map_or_else(String::new, |(d, p)| {
                    format!("{d} hex(es) from the place at {p}")
                });
            let men_words = here
                .iter()
                .map(|u| match u.marching_to {
                    Some(to) if to == base.hex && u.at != base.hex => {
                        format!("{}, marching here from {}", u.words(), u.at)
                    }
                    _ => u.words(),
                })
                .collect::<Vec<_>>()
                .join("; ");
            let held = self.supply_words(base.hex);
            let reason = serving(end, plan)
                .evidence(
                    format!("{base_text} stands at {} on the island's land, {beside}", base.hex),
                    today,
                )
                .evidence(format!("men of the side at it: {men_words}"), today)
                .evidence(format!("the base holds {held}"), today)
                .evidence(source.words().to_string(), today)
                .binding(format!("{} is released", plan.id))
                .worked(format!(
                    "{} controls {island_text}: {base_text} stands at {}, {beside}, in no named place's hex, and the side has men there ({men_words}); it holds {held}, and no supply for its hex is in force, so the staff forms a supply of it under the island, with a floor of {floor_days} {} (the margin at {}), no interval and a plan of its own. For its source: {}.",
                    end.id,
                    base.hex,
                    crate::units::days_word(floor_days),
                    risk.name(),
                    source.words()
                ))
                .would_change("no man of the side at the base")
                .would_change("a supply intent for the base already in force")
                .would_change("another source of supply")
                .done();
            let id_new = self.form_under(end, kind, reason, in_scope);
            if in_scope {
                self.lines.push(format!(
                    "{} would form {id_new}, keep {base_text} supplied beside a place, under it.",
                    end.id
                ));
            }
        }
        self.men_with_no_base(end, plan, &land, &men, island_text, in_scope);
    }

    /// Rule 8: for each hex of the island's land with men of the side ashore
    /// and no base of the side, one reason, each morning it is so.
    fn men_with_no_base(
        &mut self,
        end: &Intent,
        plan: &Plan,
        land: &BTreeSet<Hex>,
        men: &[Ashore],
        island_text: &str,
        in_scope: bool,
    ) {
        let today = self.today;
        let base_hexes: BTreeSet<Hex> = self.view.bases().map(|b| b.hex).collect();
        let mut bare: BTreeMap<Hex, Vec<&Ashore>> = BTreeMap::new();
        for unit in men {
            if land.contains(&unit.at) && !base_hexes.contains(&unit.at) {
                bare.entry(unit.at).or_default().push(unit);
            }
        }
        for (hex, units) in bare {
            let words = units
                .iter()
                .map(|u| match u.marching_to {
                    Some(to) => format!(
                        "{}, recorded at {hex}, carrying nothing and marching to {to}",
                        u.words()
                    ),
                    None => u.words(),
                })
                .collect::<Vec<_>>()
                .join("; ");
            let total: u32 = units.iter().map(|u| u.men).sum();
            let reason = serving(end, plan)
                .evidence(
                    format!("{total} men of the side are ashore at {hex}, on the island's land: {words}"),
                    today,
                )
                .evidence(format!("no base of the side stands at {hex}"), today)
                .binding("no base of the side stands there to feed them through")
                .worked(format!(
                    "{} controls {island_text}: {words} are ashore at {hex}, and no base of the side stands there to feed them through, so the staff gives no order for them.",
                    end.id
                ))
                .would_change(format!("a base of the side at {hex}"))
                .would_change("the men moved to a base")
                .done();
            self.target(in_scope).no_orders.push(reason);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    type Pair = (i32, i32);

    /// A row of hexes joined end to end, as a map's land links give them.
    fn row(links: &[(Pair, Pair)]) -> impl Fn(Hex) -> Vec<Hex> + '_ {
        move |hex| {
            links
                .iter()
                .filter_map(|(a, b)| {
                    let (a, b) = (Hex::new(a.0, a.1), Hex::new(b.0, b.1));
                    (a == hex).then_some(b).or_else(|| (b == hex).then_some(a))
                })
                .collect()
        }
    }

    #[test]
    fn the_land_is_the_places_and_every_hex_joined_to_one_through_any_number_of_hexes() {
        let links = [((4, 0), (5, 0)), ((5, 0), (6, 0)), ((6, 0), (7, 0))];
        let land = island_land(&[Hex::new(5, 0)], row(&links));
        let expect: BTreeSet<Hex> = [(4, 0), (5, 0), (6, 0), (7, 0)]
            .into_iter()
            .map(|(q, r)| Hex::new(q, r))
            .collect();
        assert_eq!(land, expect);
    }

    #[test]
    fn a_hex_joined_to_no_place_is_not_on_the_land() {
        let links = [((4, 0), (5, 0)), ((0, 4), (0, 5))];
        let land = island_land(&[Hex::new(5, 0)], row(&links));
        assert!(land.contains(&Hex::new(4, 0)));
        assert!(!land.contains(&Hex::new(0, 4)));
        assert!(!land.contains(&Hex::new(0, 5)));
    }

    #[test]
    fn two_places_on_one_landmass_give_one_land_and_none_give_none() {
        let links = [((1, 0), (2, 0)), ((2, 0), (3, 0))];
        let land = island_land(&[Hex::new(1, 0), Hex::new(3, 0)], row(&links));
        assert_eq!(land.len(), 3);
        assert!(island_land(&[], row(&links)).is_empty());
    }

    #[test]
    fn a_place_with_no_link_is_its_own_land() {
        let land = island_land(&[Hex::new(2, 2)], row(&[]));
        assert_eq!(land.len(), 1);
    }
}
