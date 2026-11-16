//! The doctrine for "deny an area", at the area's level (decision 0123,
//! rules 1 to 3). It belongs to the area, the addressee of the intents it
//! serves: it forms the tasks of striking ships and of searching the waters
//! for the bases below it, and gives no order to a unit and sets no setting
//! of one (decision 0120, 1). The doctrines of those tasks, at the base's
//! level, are `crate::strike` and `crate::search`.
//!
//! Each morning, for each deny intent in force whose plan is released, in
//! priority order, and for each base of the side in base number order:
//!
//! - where the base has a bomber group whose normal reach touches a hex of
//!   the waters, and no strike task from that base is in force under the
//!   intent, it forms one;
//! - where the base has a group that can search whose search range touches a
//!   hex of the waters, and no cover of that base is in force, and no search
//!   task from that base is in force under the intent, it forms one.
//!
//! Where no base has a bomber or a searcher that reaches the waters it forms
//! nothing, and hands in that it cannot meet the intent, with the distances
//! and the reaches.
//!
//! What it may use (rule 3): as "neutralise a base", with no forces limit on
//! its chain every base of the side and every group at each; a forces limit
//! that names air groups narrows it to the bases where a group it names is,
//! and only the groups named count there.

use sim_core::command::{
    CommandKind, Intent, IntentKind, IntentLimits, Maker, NoticeBasis, NoticeKind, StaffCommand,
    StaffNotice,
};
use sim_core::fixed::Fixed;
use sim_core::hex::Hex;
use sim_core::ids::{AirGroupId, BaseId};
use sim_core::refdata::AircraftRole;
use sim_core::state::{AirGroup, AirGroupLocation};
use sim_core::view::SideView;

use crate::air::Day;
use crate::neutralise::bomber_reach;
use crate::reasons::{base_words, serving, waters_words};

/// The waters of a deny intent (decision 0123, 1): the hexes on the map
/// within `distance` hexes of `hex`, in hex order.
pub(crate) fn waters(view: &SideView<'_>, hex: Hex, distance: u32) -> Vec<Hex> {
    let reach = i32::try_from(distance).unwrap_or(i32::MAX);
    let mut found: Vec<Hex> = Vec::new();
    for q in hex.q.saturating_sub(reach)..=hex.q.saturating_add(reach) {
        for r in hex.r.saturating_sub(reach)..=hex.r.saturating_add(reach) {
            let candidate = Hex::new(q, r);
            if candidate.distance(hex) <= distance && view.hex(candidate).is_some() {
                found.push(candidate);
            }
        }
    }
    found.sort();
    found
}

/// The air distance from `from` to the nearest hex of `waters`, in miles;
/// `None` where the waters have no hex.
pub(crate) fn nearest_nm(from: Hex, waters: &[Hex]) -> Option<Fixed> {
    waters
        .iter()
        .map(|hex| Fixed::from_int(i64::from(from.air_distance_nm(*hex))))
        .min()
}

/// Whether a reach of `reach_nm` from `from` touches a hex of `waters`, by
/// the engine's own test of a distance.
pub(crate) fn touches(from: Hex, waters: &[Hex], reach_nm: Fixed) -> bool {
    waters
        .iter()
        .any(|hex| from.air_distance_within(*hex, reach_nm))
}

/// The search range of a group that can search for the deny doctrine: not a
/// fighter, and its type has a search range (decision 0123, 2 and 12).
pub(crate) fn search_range(view: &SideView<'_>, group: &AirGroup) -> Option<Fixed> {
    let kind = view.aircraft_type(group.aircraft_type)?;
    if kind.role == AircraftRole::Fighter || kind.search_range_nm <= Fixed::ZERO {
        return None;
    }
    Some(kind.search_range_nm)
}

/// One base of the side as the forming reads it.
struct Candidate {
    base: BaseId,
    hex: Hex,
    /// Each bomber group it may use at the base, with its type's name and
    /// its normal reach.
    bombers: Vec<(AirGroupId, String, Fixed)>,
    /// Each group it may use at the base that can search, with its type's
    /// name and its search range.
    searchers: Vec<(AirGroupId, String, Fixed)>,
}

impl Day<'_, '_> {
    /// The bases the deny intent may use, each with its bombers and its
    /// searchers, in base number order.
    fn deny_candidates(&self, intent: &Intent) -> Vec<Candidate> {
        let allowed = self.state.groups_on_chain(intent.id);
        let mut bases: Vec<_> = self.view.bases().collect();
        bases.sort_by_key(|b| b.id);
        let mut found = Vec::new();
        for base in bases {
            let mut groups: Vec<_> = self
                .view
                .air_groups()
                .filter(|g| g.location == AirGroupLocation::Base(base.id))
                .filter(|g| allowed.as_ref().is_none_or(|set| set.contains(&g.id)))
                .collect();
            groups.sort_by_key(|g| g.id);
            if allowed.is_some() && groups.is_empty() {
                continue;
            }
            let name = |g: &AirGroup| {
                self.view
                    .aircraft_type(g.aircraft_type)
                    .map_or_else(|| g.aircraft_type.to_string(), |t| t.name.clone())
            };
            let bombers = groups
                .iter()
                .filter_map(|g| Some((g.id, name(g), bomber_reach(self.view, g)?)))
                .collect();
            let searchers = groups
                .iter()
                .filter_map(|g| Some((g.id, name(g), search_range(self.view, g)?)))
                .collect();
            found.push(Candidate {
                base: base.id,
                hex: base.hex,
                bombers,
                searchers,
            });
        }
        found
    }

    /// The deny doctrine for the morning (decision 0123, 2 and 3).
    #[allow(clippy::too_many_lines)]
    pub(crate) fn denials(&mut self) {
        let ends = self.in_priority_order(|i| matches!(i.kind, IntentKind::Deny { .. }));
        for id in ends {
            let Some(end) = self.state.intents.get(&id).cloned() else {
                continue;
            };
            if !self.released(&end) {
                continue;
            }
            let Some(plan) = self.plan_of(&end) else {
                continue;
            };
            let IntentKind::Deny { hex, distance } = end.kind else {
                continue;
            };
            let in_scope = self.in_scope(id);
            let today = self.today;
            let waters_hexes = waters(self.view, hex, distance);
            let waters_text = format!(
                "the waters within {distance} hexes of {hex} ({} hex(es) on the map)",
                waters_hexes.len()
            );
            let candidates = self.deny_candidates(&end);
            let mut served_any = false;
            for candidate in &candidates {
                let base = candidate.base;
                let base_text = base_words(self.view, base);
                let near = nearest_nm(candidate.hex, &waters_hexes);
                let near_text = near.map_or_else(
                    || "the waters have no hex on the map".to_string(),
                    |nm| format!("{base_text} is {nm} nm from the nearest hex of {waters_text}"),
                );
                // (a) Striking ships.
                let reaching: Vec<String> = candidate
                    .bombers
                    .iter()
                    .filter(|(_, _, reach)| touches(candidate.hex, &waters_hexes, *reach))
                    .map(|(g, name, reach)| format!("{g} ({name}, a reach of {reach} nm)"))
                    .collect();
                if !reaching.is_empty() {
                    served_any = true;
                    let reach_text = format!(
                        "{near_text}; its bombers whose normal reach touches them: {}",
                        reaching.join(", ")
                    );
                    let kind = IntentKind::StrikeShips { base };
                    self.form_or_stand(&end, &plan, kind, &reach_text, in_scope, today);
                }
                // (b) Searching the waters.
                let searching: Vec<String> = candidate
                    .searchers
                    .iter()
                    .filter(|(_, _, range)| touches(candidate.hex, &waters_hexes, *range))
                    .map(|(g, name, range)| format!("{g} ({name}, a search range of {range} nm)"))
                    .collect();
                if searching.is_empty() {
                    continue;
                }
                served_any = true;
                let search_text = format!(
                    "{near_text}; its groups whose search range touches them: {}",
                    searching.join(", ")
                );
                let cover = self
                    .state
                    .intents_in_force()
                    .find(|c| c.kind == IntentKind::Cover { base })
                    .map(|c| c.id);
                if let Some(cover) = cover {
                    let reason = serving(&end, &plan)
                        .evidence(search_text.clone(), today)
                        .evidence(format!("{cover}, a cover of {base_text}, is in force"), today)
                        .binding("one arc for a base's searchers: the cover's")
                        .worked(format!(
                            "{cover} covers {base_text}, so the staff forms no search task there under {}: the cover's arc takes the hexes of {waters_text} as threats. {search_text}.",
                            end.id
                        ))
                        .would_change(format!("{cover} withdrawn"))
                        .done();
                    self.target(in_scope).no_orders.push(reason);
                    if in_scope {
                        self.lines.push(format!(
                            "{} is to {}: {base_text}'s cover, {cover}, would search them.",
                            end.id,
                            waters_words(hex, distance)
                        ));
                    }
                    continue;
                }
                let kind = IntentKind::SearchWaters { base };
                self.form_or_stand(&end, &plan, kind, &search_text, in_scope, today);
            }
            if !served_any {
                self.cannot_deny(
                    &end,
                    &plan,
                    &waters_text,
                    &waters_hexes,
                    &candidates,
                    in_scope,
                );
            }
        }
    }

    /// Forms the task `kind` under `end`, or, where one stands, says so.
    fn form_or_stand(
        &mut self,
        end: &Intent,
        plan: &sim_core::command::Plan,
        kind: IntentKind,
        reach_text: &str,
        in_scope: bool,
        today: u32,
    ) {
        let Some(base) = kind.own_base() else {
            return;
        };
        let base_text = base_words(self.view, base);
        let task_words = match kind {
            IntentKind::StrikeShips { .. } => format!("strike ships from {base_text}"),
            _ => format!("search the waters from {base_text}"),
        };
        let (hex, distance) = end.kind.waters().unwrap_or((Hex::new(0, 0), 0));
        let end_words = waters_words(hex, distance);
        let standing = self
            .state
            .children_in_force(end.id)
            .into_iter()
            .filter_map(|child| self.state.intents.get(&child))
            .find(|child| child.kind == kind)
            .map(|child| child.id);
        if let Some(task) = standing {
            let reason = serving(end, plan)
                .evidence(reach_text.to_string(), today)
                .evidence(format!("{task}, {task_words}, is in force under {}", end.id), today)
                .binding(format!("{task} stands"))
                .worked(format!(
                    "{task}, {task_words}, stands under {}: the staff forms no other. {reach_text}.",
                    end.id
                ))
                .would_change(format!("{task} withdrawn"))
                .done();
            self.target(in_scope).no_orders.push(reason);
            return;
        }
        let id_new = self.state.next_intent_number();
        self.state.intents.insert(
            id_new,
            Intent {
                id: id_new,
                kind,
                parent: Some(end.id),
                maker: Maker::Staff,
                addressee: kind.addressee(),
                limits: IntentLimits::default(),
                set_day: today,
                plan: end.plan,
                withdrawn_day: None,
            },
        );
        let binding = match kind {
            IntentKind::StrikeShips { .. } => "a bomber's normal reach touches a hex of the waters",
            _ => "a search range touches a hex of the waters, and no cover of the base stands",
        };
        let reason = serving(end, plan)
            .evidence(reach_text.to_string(), today)
            .binding(format!("{} is to {end_words} and its plan is released", end.id))
            .binding(binding)
            .worked(format!(
                "{} is to {end_words}, its plan is released and no task to {task_words} stands under it: {reach_text}, so the staff forms {id_new}, {task_words}, under it, for {base_text} to carry out.",
                end.id
            ))
            .would_change("no group at the base that reaches the waters")
            .would_change("such a task already standing")
            .done();
        self.target(in_scope).commands.push(StaffCommand {
            kind: CommandKind::FormIntent {
                parent: end.id,
                kind,
            },
            reason,
        });
        if in_scope {
            self.lines.push(format!(
                "{} is to {end_words}: it would form {id_new}, {task_words}, under it ({reach_text}).",
                end.id
            ));
        }
    }

    /// No base reaches the waters: the reason with each base's distance and
    /// the reach of each type it has, and the notice that the staff cannot
    /// meet the intent (decision 0123, 2).
    fn cannot_deny(
        &mut self,
        end: &Intent,
        plan: &sim_core::command::Plan,
        waters_text: &str,
        waters_hexes: &[Hex],
        candidates: &[Candidate],
        in_scope: bool,
    ) {
        let today = self.today;
        let mut lines: Vec<String> = Vec::new();
        for candidate in candidates {
            let base_text = base_words(self.view, candidate.base);
            let near = nearest_nm(candidate.hex, waters_hexes).map_or_else(
                || "no hex of the waters is on the map".to_string(),
                |nm| format!("{nm} nm from the nearest hex of the waters"),
            );
            let mut types: Vec<String> = Vec::new();
            for (_, name, reach) in &candidate.bombers {
                let text = format!("a reach of {reach} nm ({name})");
                if !types.contains(&text) {
                    types.push(text);
                }
            }
            for (_, name, range) in &candidate.searchers {
                let text = format!("a search range of {range} nm ({name})");
                if !types.contains(&text) {
                    types.push(text);
                }
            }
            if types.is_empty() {
                lines.push(format!(
                    "{base_text} is {near} and has no bomber and no group that can search that the intent may use"
                ));
            } else {
                lines.push(format!("{base_text} is {near}: {}", types.join(", ")));
            }
        }
        let body = if lines.is_empty() {
            "the side has no base it may use".to_string()
        } else {
            lines.join("; ")
        };
        let reason = serving(end, plan)
            .evidence(body.clone(), today)
            .binding("a bomber's normal reach or a search range must touch a hex of the waters")
            .worked(format!(
                "No base {} may use has a bomber whose normal reach, or a group whose search range, touches {waters_text}: {body}. The staff forms no task and cannot meet {}.",
                end.id, end.id
            ))
            .would_change("a bomber or a search group at a base within reach of the waters")
            .done();
        let out = self.target(in_scope);
        out.no_orders.push(reason);
        let index = out.no_orders.len() - 1;
        out.notices.push(StaffNotice {
            kind: NoticeKind::CannotMeet,
            plan: plan.id,
            run: None,
            rests_on: NoticeBasis::NoOrder(index),
        });
        if in_scope {
            self.lines.push(format!(
                "{} is to deny {waters_text}: it would form no task ({body}), and cannot meet it.",
                end.id
            ));
        }
    }

    /// The waters of every deny intent in force whose plan is released that
    /// stands above a task at `base` or could be served from it: the hexes a
    /// cover of `base` takes as threats beside its own (decision 0123, 13).
    /// Only the deny intents whose searchers at `base`, as `searchers` gives
    /// their ranges, touch the waters.
    pub(crate) fn waters_for_cover(&self, base_hex: Hex, ranges: &[Fixed]) -> Vec<Hex> {
        let mut found: Vec<Hex> = Vec::new();
        for intent in self.state.intents_in_force() {
            let Some((hex, distance)) = intent.kind.waters() else {
                continue;
            };
            if !self.released(intent) {
                continue;
            }
            let hexes = waters(self.view, hex, distance);
            if ranges.iter().any(|range| touches(base_hex, &hexes, *range)) {
                for hex in hexes {
                    if !found.contains(&hex) {
                        found.push(hex);
                    }
                }
            }
        }
        found
    }
}
