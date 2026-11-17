//! The doctrine for "neutralise a base", at the area's level (decision 0120,
//! rules 1, 6 and 7). It belongs to the area, the addressee of the intents
//! it serves: it forms raid tasks for the bases below it and gives no order
//! to a unit and sets no setting of one (rule 1). The raid doctrine, at the
//! base's level, is `crate::raid`.
//!
//! Each morning, for each neutralise intent in force whose plan is released,
//! in priority order, and for each base of the side, in base number order,
//! that it may use and that has a bomber group able to reach the target on
//! normal reach: where no raid task from that base at that target is in
//! force under it, it forms one. Where no base qualifies it forms none,
//! hands in that it cannot meet the intent, and its reason gives the
//! distance and the reach of each bomber type it has.
//!
//! What it may use (rule 7): with no forces limit on its chain, every base of
//! the side and every group at each; a forces limit that names air groups
//! narrows it to the bases where a group it names is.

use sim_core::command::{
    CommandKind, Intent, IntentKind, IntentLimits, Maker, NoticeBasis, NoticeKind, StaffCommand,
    StaffNotice,
};
use sim_core::fixed::Fixed;
use sim_core::hex::Hex;
use sim_core::ids::{AirGroupId, BaseId};
use sim_core::refdata::{AircraftRole, AttackWeapon};
use sim_core::state::AirGroupLocation;
use sim_core::view::SideView;

use crate::air::Day;
use crate::reasons::{base_held_words, base_words, serving};

/// A group of the side as a raid reads it: whether it is a bomber, and if
/// so its type's normal reach (decision 0120, 10: not a fighter, a weapon,
/// the reach on normal reach, which is the type's strike radius).
pub(crate) fn bomber_reach(
    view: &SideView<'_>,
    group: &sim_core::state::AirGroup,
) -> Option<Fixed> {
    let kind = view.aircraft_type(group.aircraft_type)?;
    if kind.role == AircraftRole::Fighter || kind.attack_weapon == AttackWeapon::None {
        return None;
    }
    Some(kind.strike_range_nm)
}

/// The air distance from `from` to `to`, in miles.
pub(crate) fn distance_nm(from: Hex, to: Hex) -> Fixed {
    Fixed::from_int(i64::from(from.air_distance_nm(to)))
}

/// One base of the side as the forming reads it.
struct Candidate {
    base: BaseId,
    distance: Fixed,
    /// Each bomber group it may use at the base: the group, its type's name
    /// and its normal reach.
    bombers: Vec<(AirGroupId, String, Fixed)>,
}

impl Candidate {
    fn reaches(&self) -> bool {
        self.bombers
            .iter()
            .any(|(_, _, reach)| *reach >= self.distance)
    }
}

impl Day<'_, '_> {
    /// The bases the neutralise intent may use, each with its distance to
    /// the target and its bomber groups, in base number order.
    fn candidates(&self, intent: &Intent, target_hex: Hex) -> Vec<Candidate> {
        let allowed = self.state.groups_on_chain(intent.id);
        let mut found: Vec<Candidate> = Vec::new();
        let mut bases: Vec<_> = self.area.bases().collect();
        bases.sort_by_key(|b| b.id);
        for base in bases {
            let mut groups: Vec<_> = self
                .view
                .air_groups()
                .filter(|g| g.location == AirGroupLocation::Base(base.id))
                .filter(|g| allowed.as_ref().is_none_or(|set| set.contains(&g.id)))
                .collect();
            groups.sort_by_key(|g| g.id);
            if allowed.is_some() && groups.is_empty() {
                // A forces limit names no group here: the base is not one
                // it may use.
                continue;
            }
            let bombers: Vec<(AirGroupId, String, Fixed)> = groups
                .iter()
                .filter_map(|g| {
                    let reach = bomber_reach(self.area, g)?;
                    let name = self
                        .view
                        .aircraft_type(g.aircraft_type)
                        .map_or_else(|| g.aircraft_type.to_string(), |t| t.name.clone());
                    Some((g.id, name, reach))
                })
                .collect();
            found.push(Candidate {
                base: base.id,
                distance: distance_nm(base.hex, target_hex),
                bombers,
            });
        }
        found
    }

    /// The neutralise doctrine for the morning (decision 0120, 6 and 7).
    pub(crate) fn neutralises(&mut self) {
        let ends = self.in_priority_order(|i| matches!(i.kind, IntentKind::Neutralise { .. }));
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
            let IntentKind::Neutralise { target } = end.kind else {
                continue;
            };
            let in_scope = self.in_scope(id);
            let today = self.today;
            let target_text = base_words(self.view, target);
            let Some(target_hex) = self
                .view
                .enemy_bases()
                .find(|b| b.id == target)
                .map(|b| b.hex)
            else {
                let reason = serving(&end, &plan)
                    .evidence(format!("{target_text} is not an enemy base on the side's map"), today)
                    .binding("a raid needs a target on the map")
                    .worked(format!(
                        "{target_text} is not a base of the enemy's on the side's map, so the staff forms no raid task under {}.",
                        end.id
                    ))
                    .would_change("the base on the map again as the enemy's")
                    .done();
                self.target(in_scope).no_orders.push(reason);
                continue;
            };
            let candidates = self.candidates(&end, target_hex);
            let qualifying: Vec<&Candidate> = candidates.iter().filter(|c| c.reaches()).collect();
            if qualifying.is_empty() {
                self.cannot_neutralise(&end, &plan, &target_text, &candidates, in_scope);
                continue;
            }
            for candidate in qualifying {
                let base = candidate.base;
                let base_text = base_words(self.view, base);
                let base_held = base_held_words(self.area, base);
                let reaching: Vec<String> = candidate
                    .bombers
                    .iter()
                    .filter(|(_, _, reach)| *reach >= candidate.distance)
                    .map(|(group, name, reach)| format!("{group} ({name}, a reach of {reach} nm)"))
                    .collect();
                let reach_text = format!(
                    "{base_held} is {} nm from {target_text}; its bombers that reach it on normal reach: {}",
                    candidate.distance,
                    reaching.join(", ")
                );
                let standing = self
                    .state
                    .children_in_force(end.id)
                    .into_iter()
                    .filter_map(|child| self.state.intents.get(&child))
                    .find(|child| child.kind == IntentKind::Raid { base, target })
                    .map(|child| child.id);
                if let Some(raid) = standing {
                    let reason = serving(&end, &plan)
                        .evidence(reach_text.clone(), today)
                        .evidence(format!("{raid}, the raid on {target_text} from {base_text}, is in force under {}", end.id), today)
                        .binding(format!("{raid} stands"))
                        .worked(format!(
                            "{raid} raids {target_text} from {base_text} under {} and stands: the staff forms no other. {reach_text}.",
                            end.id
                        ))
                        .would_change(format!("{raid} withdrawn"))
                        .done();
                    self.target(in_scope).no_orders.push(reason);
                    continue;
                }
                let id_new = self.state.next_intent_number();
                let kind = IntentKind::Raid { base, target };
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
                let reason = serving(&end, &plan)
                    .evidence(reach_text.clone(), today)
                    .binding(format!("{} is to neutralise {target_text} and its plan is released", end.id))
                    .binding("a bomber reaches the target on normal reach")
                    .worked(format!(
                        "{} is to neutralise {target_text}, its plan is released and no raid on it from {base_text} stands under it: {reach_text}, so the staff forms {id_new}, raid {target_text} from {base_text}, under it, for {base_text} to carry out.",
                        end.id
                    ))
                    .would_change("no bomber at the base that reaches the target")
                    .would_change("a raid task from the base already standing")
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
                        "{} is to neutralise {target_text}: it would form {id_new}, raid {target_text} from {base_text}, under it ({reach_text}).",
                        end.id
                    ));
                }
            }
        }
    }

    /// No base qualifies: the reason with each base's distance and the reach
    /// of each bomber type it has, and the notice that the staff cannot meet
    /// the intent (decision 0120, 6).
    fn cannot_neutralise(
        &mut self,
        end: &Intent,
        plan: &sim_core::command::Plan,
        target_text: &str,
        candidates: &[Candidate],
        in_scope: bool,
    ) {
        let today = self.today;
        let mut lines: Vec<String> = Vec::new();
        for candidate in candidates {
            let base_text = base_held_words(self.area, candidate.base);
            if candidate.bombers.is_empty() {
                lines.push(format!(
                    "{base_text} is {} nm from {target_text} and has no bomber group it may use",
                    candidate.distance
                ));
                continue;
            }
            let mut types: Vec<(String, Fixed)> = Vec::new();
            for (_, name, reach) in &candidate.bombers {
                if !types.iter().any(|(seen, _)| seen == name) {
                    types.push((name.clone(), *reach));
                }
            }
            let types_text = types
                .iter()
                .map(|(name, reach)| {
                    format!(
                        "{} nm against a reach of {reach} nm ({name})",
                        candidate.distance
                    )
                })
                .collect::<Vec<_>>()
                .join(", ");
            lines.push(format!(
                "{base_text} is {} nm from {target_text}: {types_text}",
                candidate.distance
            ));
        }
        let body = if lines.is_empty() {
            "the side has no base it may use".to_string()
        } else {
            lines.join("; ")
        };
        let reason = serving(end, plan)
            .evidence(body.clone(), today)
            .binding("a bomber must reach the target on normal reach")
            .worked(format!(
                "No base {} may use has a bomber group that reaches {target_text} on normal reach: {body}. The staff forms no raid task and cannot meet {}.",
                end.id, end.id
            ))
            .would_change("a bomber group at a base within reach of the target")
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
                "{} is to neutralise {target_text}: it would form no raid task ({body}), and cannot meet it.",
                end.id
            ));
        }
    }
}
