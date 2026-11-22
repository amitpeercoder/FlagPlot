//! Cover for an operation by carriers (decision 0148, step 6 of stage 3,
//! second part): "operate a carrier force" formed under "take a place".
//!
//! **Forming** (items 1 and 2). Under a "take a place" whose plan is
//! released, for each task force on the carrier mission with a carrier
//! afloat that has an air group aboard that the nearest forces limit on the
//! end's chain names, in task force order, where no carrier task for that
//! force is in force anywhere on the side: the task at the covering hex, the
//! first hex along the shortest sea route from the hex the force is in to the
//! place within the least normal strike radius of the bomber groups aboard
//! (the closing rule's reach and first hex, decision 0130, 12 and 13, with
//! the place for the contact); the hex the force is in where there is no
//! bomber group or it is within that reach already; none where there is no
//! sea route.
//!
//! **Moved to look** (decision 0159, 14, as the architect ruled it on Nov
//! 18, 2026, in scenario 83's hand calculation). Under a "take a place"
//! whose plan is preparing, on a morning when the need to know is not met,
//! no air group the forces limit names reaches the place from where it is,
//! and the limit names a group that can search aboard a ship of a carrier
//! force it names: the carrier task of that force is formed then, at the
//! covering hex where a named scout aboard reaches the place from it, else
//! at the first hex on her way from which one does
//! ([`Day::form_look_task`]). At preserve the force only with leave to
//! launch given. Before the plan is released the task gives every order but
//! the strike on the place, and no order to a group the look keeps on
//! reconnaissance of the place.
//!
//! **The force's morning** is the carrier doctrine's (`crate::carrier`), with
//! the strike on the place (items 4 to 6, in [`Day::cover_strike`]) and,
//! on the first morning the end is met, the way home (item 8,
//! [`Day::send_home`]).

use std::collections::BTreeMap;

use sim_core::command::{
    CommandKind, Intent, IntentKind, IntentLimits, Maker, Plan, Reason, Risk, StaffCommand,
    StaffOrder,
};
use sim_core::fixed::Fixed;
use sim_core::hex::Hex;
use sim_core::ids::{AirGroupId, IntentId, TaskForceId};
use sim_core::orders::{AirGroupOrder, Mission, OrderKind, SpeedSetting, TaskForceOrder};
use sim_core::state::{AirGroupLocation, ShipLocation};

use crate::air::{mission_words, Day};
use crate::carrier::{CarrierAir, CarrierGroup, CarrierRole, CLOSE_MAX_AGE};
use crate::neutralise::{bomber_reach, distance_nm};
use crate::raid::{
    decide as raid_decide, held_share, least_weight, rest_level, BomberVerdict, RaidGroup, RaidRole,
};
use crate::reasons::{base_words, serving};
use crate::stand::{afloat, expected_words, hours_to, stopped};

/// Why no covering hex is found: no sea route reaches a hex within `reach`
/// of the place, the reach set by the groups `sets`.
pub(crate) struct NoCoveringHex {
    pub(crate) reach: Fixed,
    pub(crate) sets: String,
}

/// The base a carrier task's bombers strike, with the words that say what
/// it is to the task.
pub(crate) struct StrikeAim {
    pub(crate) base: sim_core::ids::BaseId,
    pub(crate) hex: Hex,
    /// After the base's name: "the enemy's at the place (q,r)".
    pub(crate) of: String,
    /// In the binding: "the enemy's base at the place".
    pub(crate) binding: String,
}

/// What the strike on the place hands the carrier doctrine's morning.
#[derive(Default)]
pub(crate) struct CoverStrike {
    /// The orders the raid's rules give the groups that go.
    pub(crate) orders: Vec<StaffOrder>,
    /// The groups whose mission and held share the raid's rules decide this
    /// morning: the table's orders for those two settings are dropped.
    pub(crate) taken: Vec<AirGroupId>,
    /// Each such group's line, for the reason of an order of another setting.
    pub(crate) lines: BTreeMap<AirGroupId, String>,
    /// The reason of a raid that does not go.
    pub(crate) note: Option<Reason>,
}

impl Day<'_, '_> {
    /// Item 1 and the serving of every carrier task standing under a "take a
    /// place", after the landing's morning (the plan is released by then on
    /// the morning the landing is ready): the forming for each released end,
    /// then each task's morning, or its way home on the first morning the
    /// end is met.
    pub(crate) fn take_carriers(&mut self) {
        let ends = self.in_priority_order(|i| matches!(i.kind, IntentKind::TakePlace { .. }));
        for id in ends {
            let Some(end) = self.state.intents.get(&id).cloned() else {
                continue;
            };
            if self.preparing(&end) {
                // Decision 0159, 14: a carrier force moved to look.
                if let (Some(plan), IntentKind::TakePlace { hex, .. }) =
                    (self.plan_of(&end), end.kind)
                {
                    let in_scope = self.in_scope(id);
                    self.form_look_task(&end, &plan, hex, in_scope);
                }
                continue;
            }
            if !self.released(&end) {
                continue;
            }
            let Some(plan) = self.plan_of(&end) else {
                continue;
            };
            let IntentKind::TakePlace { hex, .. } = end.kind else {
                continue;
            };
            let in_scope = self.in_scope(id);
            self.form_cover_tasks(&end, &plan, hex, in_scope);
        }
        // Decision 0151, 8a: the carrier tasks under an island's denial, after
        // the landing's morning, by the carrier doctrine as it is.
        let island_tasks = self.in_priority_order(|i| {
            matches!(i.kind, IntentKind::OperateCarriers { .. }) && self.under_island(i).is_some()
        });
        for id in island_tasks {
            let Some(task) = self.state.intents.get(&id).cloned() else {
                continue;
            };
            if self.released(&task) {
                self.serve_carrier(&task);
            }
        }
        let tasks = self.in_priority_order(|i| {
            matches!(i.kind, IntentKind::OperateCarriers { .. }) && self.under_take(i).is_some()
        });
        for id in tasks {
            let Some(task) = self.state.intents.get(&id).cloned() else {
                continue;
            };
            let Some(end) = self.under_take(&task) else {
                continue;
            };
            if self.released(&task) {
                self.serve_carrier(&task);
                continue;
            }
            // Decision 0159, 14: before the plan is released the task gives
            // every order but the strike on the place.
            if self.preparing(&task) {
                self.serve_carrier_with(&task, false);
                continue;
            }
            // Met this morning: the plan was released at the staff's hour
            // and is complete in the planned state now.
            let released_at_hour = self
                .view
                .command_state()
                .plans
                .get(&task.plan)
                .is_some_and(|p| p.state == sim_core::command::PlanState::Released);
            let complete = self
                .state
                .plans
                .get(&task.plan)
                .is_some_and(|p| p.state == sim_core::command::PlanState::Complete);
            if released_at_hour && complete {
                self.send_home(&task, &end);
            }
        }
    }

    /// The "control an island" a carrier task stands under, through the
    /// denial it was formed under (decision 0151, 6 and 8a), where it does.
    pub(crate) fn under_island(&self, task: &Intent) -> Option<Intent> {
        let deny = task
            .parent
            .and_then(|p| self.state.intents.get(&p))
            .filter(|p| matches!(p.kind, IntentKind::Deny { .. }))?;
        deny.parent
            .and_then(|p| self.state.intents.get(&p))
            .filter(|p| matches!(p.kind, IntentKind::ControlIsland { .. }))
            .cloned()
    }

    /// Decision 0151, 8a: the take the carrier task `task` under an island's
    /// denial strikes the place of: the first take of the island's tree, in
    /// the island's order, whose plan is released and not met and whose
    /// place is within a bomber group's reach of `here`.
    pub(crate) fn island_strike_take(
        &self,
        task: &Intent,
        here: Hex,
        reach_of: &BTreeMap<AirGroupId, Fixed>,
    ) -> Option<Intent> {
        let island = self.under_island(task)?;
        let number = island.kind.island()?;
        let places = self.view.island(number)?;
        let reach = reach_of.values().copied().max()?;
        places.places.iter().find_map(|place| {
            if self.place_is_ours(place.hex) || distance_nm(here, place.hex) > reach {
                return None;
            }
            self.state
                .intents_in_force()
                .find(|i| {
                    i.parent == Some(island.id)
                        && matches!(i.kind, IntentKind::TakePlace { hex, .. } if hex == place.hex)
                        && self.released(i)
                })
                .cloned()
        })
    }

    /// Whether the plan of `intent` is preparing in the planned state.
    pub(crate) fn preparing(&self, intent: &Intent) -> bool {
        self.state
            .plans
            .get(&intent.plan)
            .is_some_and(|plan| plan.state == sim_core::command::PlanState::Preparing)
    }

    /// The carrier task in force under the take `end`, with its force and
    /// hex: the force moved to look while the plan is preparing (decision
    /// 0159, 14), or its cover once released.
    pub(crate) fn carrier_task_under(&self, end: &Intent) -> Option<(IntentId, TaskForceId, Hex)> {
        self.state.intents_in_force().find_map(|i| match i.kind {
            IntentKind::OperateCarriers { force, hex } if i.parent == Some(end.id) => {
                Some((i.id, force, hex))
            }
            _ => None,
        })
    }

    /// Whether the air group `group` is aboard a ship of the force `force`.
    pub(crate) fn group_aboard(&self, group: AirGroupId, force: TaskForceId) -> bool {
        let Some(found) = self.view.air_groups().find(|g| g.id == group) else {
            return false;
        };
        let AirGroupLocation::Ship(ship) = found.location else {
            return false;
        };
        self.view
            .ships()
            .any(|s| s.id == ship && s.location == ShipLocation::TaskForce(force))
    }

    /// Decision 0159, 14, as the architect ruled it on Nov 18, 2026: under
    /// `end`, whose plan is preparing and whose place is `place`, the carrier
    /// task of the first carrier force its forces limit names that has a
    /// group the limit names aboard that can search, formed on a morning the
    /// need to know is not met and no group the limit names reaches the place
    /// from where it is. Its hex is the covering hex where such a group
    /// reaches the place from it, else the first hex along the shortest sea
    /// route from which one does. At preserve the force it is formed only
    /// with leave to launch given; without it the reason says where the
    /// force could look from and that the staff waits for leave.
    #[allow(clippy::too_many_lines)]
    pub(crate) fn form_look_task(&mut self, end: &Intent, plan: &Plan, place: Hex, in_scope: bool) {
        // One force moved to look, and one carrier task a take: where one
        // stands under the take, nothing more is formed.
        if self.carrier_task_under(end).is_some() {
            return;
        }
        let risk = self.state.risk_read_up(end.id);
        let k = self.knowledge(end, place, risk);
        if k.met || k.fighting_ashore || !k.on_it.is_empty() {
            return;
        }
        let may = self.may_use(end);
        // The groups the limit names that can search and are not held, with
        // their search range and where they lie now.
        let mut scouts: Vec<(AirGroupId, Fixed)> = Vec::new();
        for id in &may {
            let Some(group) = self.view.air_groups().find(|g| g.id == *id) else {
                continue;
            };
            let Some(range) = crate::deny::search_range(self.view, group) else {
                continue;
            };
            // A group the limit names that reaches the place from where it
            // is: no force is moved (it is sent, or the look says why not).
            if let Some(origin) = self.group_hex(group) {
                if Fixed::from_int(i64::from(origin.air_distance_nm(place))) <= range {
                    return;
                }
            }
            if self.state.hold_on_group(*id).is_some() {
                continue;
            }
            scouts.push((*id, range));
        }
        if scouts.is_empty() {
            return;
        }
        let named: std::collections::BTreeSet<TaskForceId> = self
            .state
            .nearest_forces(end.id)
            .map(|f| f.task_forces.clone())
            .unwrap_or_default();
        let today = self.today;
        for id in named {
            let Some(force) = self.area.task_forces().find(|f| f.id == id).cloned() else {
                continue;
            };
            if force.mission != Mission::Carrier {
                continue;
            }
            if crate::available::force_waiting(self.area, id).is_some() {
                continue;
            }
            if self.state.hold_on_force(id, &force.ships).is_some() {
                continue;
            }
            // One carrier task a force, whoever made it, wherever it stands.
            if self.state.intents_in_force().any(|other| {
                other.kind.force() == Some(id)
                    && matches!(other.kind, IntentKind::OperateCarriers { .. })
            }) {
                continue;
            }
            let ships = afloat(self.area, id);
            let carriers: Vec<_> = ships
                .iter()
                .filter(|(_, _, _, kind)| *kind == sim_core::refdata::ShipKind::Carrier)
                .map(|(ship, _, _, _)| *ship)
                .collect();
            if carriers.is_empty() {
                continue;
            }
            let aboard: Vec<_> = self
                .area
                .air_groups()
                .filter(
                    |g| matches!(g.location, AirGroupLocation::Ship(s) if carriers.contains(&s)),
                )
                .collect();
            let scouts_aboard: Vec<(AirGroupId, Fixed)> = scouts
                .iter()
                .copied()
                .filter(|(g, _)| self.group_aboard(*g, id))
                .collect();
            let Some(longest) = scouts_aboard.iter().map(|(_, r)| *r).max() else {
                continue;
            };
            let scout_words = scouts_aboard
                .iter()
                .map(|(g, r)| format!("{g}, which searches to {r} nm"))
                .collect::<Vec<_>>()
                .join(" and ");
            let here = force.position();
            let place_nm = distance_nm(here, place);
            let reaching = |hex: Hex| {
                let nm = distance_nm(hex, place);
                scouts_aboard.iter().any(|(_, r)| nm <= *r)
            };
            // The covering hex where a scout aboard reaches from it, else the
            // first hex on her way from which one does.
            let covering = self.covering_hex(id, here, &aboard, place).ok();
            let chosen = match covering {
                Some((hex, words)) if reaching(hex) => Some((
                    hex,
                    format!("{words}; from there {place} is {} nm away, within the search of {scout_words}", distance_nm(hex, place)),
                )),
                covering => self
                    .view
                    .sea_path(here, place)
                    .and_then(|path| {
                        crate::waters::close_to_regarding(self.area, &path, place, longest)
                    })
                    .map(|regard| {
                        let hex = regard.hex;
                        let before = covering.map_or_else(
                            || "no covering hex is found".to_string(),
                            |(c, _)| format!("the covering hex {c} is beyond the search of {scout_words}"),
                        );
                        (
                            hex,
                            format!(
                                "{before}: the first hex along the shortest sea route to {place} within {longest} nm of it is {hex}, {} nm from {here} and {} nm from {place}{}",
                                distance_nm(here, hex),
                                distance_nm(hex, place),
                                regard.words()
                            ),
                        )
                    }),
            };
            let held = crate::reasons::force_held_words(self.area, id);
            let have = crate::look::estimate_words(place, k.estimate.as_ref());
            let why = format!(
                "{have}; {}; no group the forces limit of {} names reaches {place} from where it is, and {held}, on the carrier mission at {here}, {place_nm} nm from {place}, has aboard {scout_words}",
                k.asks, end.id
            );
            let Some((hex, hex_words)) = chosen else {
                let reason = serving(end, plan)
                    .evidence(why.clone(), today)
                    .binding("the force looks from a hex on the shortest sea route to the place")
                    .worked(format!(
                        "{why}; but no sea route from {here} reaches a hex within {longest} nm of {place}, so the staff does not move {id} to look."
                    ))
                    .would_change("a sea route to the place")
                    .done();
                self.target(in_scope).no_orders.push(reason);
                continue;
            };
            let (speed, speed_text) = crate::carrier::passage_speed(self.view, id, risk, hex);
            let passage = match hours_to(self.view, id, hex, speed) {
                Some(h) => format!("the passage to {hex} is {h} hours at {speed_text}"),
                None => format!("the engine's measure does not bring {id} to {hex}"),
            };
            let from_there = distance_nm(hex, place);
            // At preserve the force the staff moves a force to look only with
            // leave to launch given.
            if risk == Risk::Preserve && !self.state.leaves.launch {
                let reason = serving(end, plan)
                    .evidence(why.clone(), today)
                    .evidence(hex_words.clone(), today)
                    .binding("at preserve the force a force is moved to look only with leave to launch")
                    .worked(format!(
                        "{why}. {id} could look from {hex}: {hex_words}; {passage}, and from there it is within {from_there} nm of {place}. At preserve the force the staff moves a force to look only with leave to launch, which is withheld, so it waits for leave to launch and gives {id} no order."
                    ))
                    .would_change("leave to launch given")
                    .would_change("a group the forces limit names that reaches the place")
                    .done();
                self.target(in_scope).no_orders.push(reason);
                return;
            }
            let kind = IntentKind::OperateCarriers { force: id, hex };
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
            let reason = serving(end, plan)
                .evidence(why.clone(), today)
                .evidence(hex_words.clone(), today)
                .evidence(passage.clone(), today)
                .binding(k.asks.clone())
                .worked(format!(
                    "{why}. The need is not met while {} is preparing, so the staff moves {id} to look: {hex_words}. It forms {id_new}, to operate the carrier force {id} from {hex}, under {}, for {id} to carry out: {passage}, and from there it is within {from_there} nm of {place}. Until {} is released the task strikes nothing at the place.",
                    plan.id, end.id, plan.id
                ))
                .would_change("a count of the grade asked for")
                .would_change("a group the forces limit names that reaches the place")
                .would_change(format!("{id} not named in the forces limit"))
                .done();
            self.target(in_scope).commands.push(StaffCommand {
                kind: CommandKind::FormIntent {
                    parent: end.id,
                    kind,
                },
                reason,
            });
            if let Some(need) = self.needs.iter_mut().find(|n| n.plan == plan.id) {
                need.moved_to_look = Some(sim_core::command::MovedToLook {
                    task_force: id,
                    hex,
                });
            }
            if in_scope {
                self.lines.push(format!(
                    "{} would form {id_new}, to operate the carrier force {id} from {hex}, under it, to look at {place} ({hex_words}).",
                    end.id
                ));
            }
            return;
        }
    }

    /// The "take a place" intent `task` stands under, where it does.
    pub(crate) fn under_take(&self, task: &Intent) -> Option<Intent> {
        task.parent
            .and_then(|p| self.state.intents.get(&p))
            .filter(|p| matches!(p.kind, IntentKind::TakePlace { .. }))
            .cloned()
    }

    /// Items 1 and 2: the forming under `end`, whose place is `place`.
    #[allow(clippy::too_many_lines)]
    fn form_cover_tasks(&mut self, end: &Intent, plan: &Plan, place: Hex, in_scope: bool) {
        let named: std::collections::BTreeSet<TaskForceId> = self
            .state
            .nearest_forces(end.id)
            .map(|f| f.task_forces.clone())
            .unwrap_or_default();
        let today = self.today;
        for id in named {
            let Some(force) = self.area.task_forces().find(|f| f.id == id) else {
                continue;
            };
            if force.mission != Mission::Carrier {
                continue;
            }
            // A force before its day is no force a task may use (decision
            // 0140, 3).
            if crate::available::force_waiting(self.area, id).is_some() {
                continue;
            }
            let ships = afloat(self.area, id);
            let carriers: Vec<_> = ships
                .iter()
                .filter(|(_, _, _, kind)| *kind == sim_core::refdata::ShipKind::Carrier)
                .map(|(ship, _, _, _)| *ship)
                .collect();
            let aboard: Vec<_> = self
                .area
                .air_groups()
                .filter(
                    |g| matches!(g.location, AirGroupLocation::Ship(s) if carriers.contains(&s)),
                )
                .collect();
            if aboard.is_empty() {
                continue;
            }
            let held = crate::reasons::force_held_words(self.area, id);
            let here = force.position();
            let why = format!(
                "{held} is on the carrier mission with {} carrier(s) afloat and {} air group(s) aboard, and lies at {here}",
                carriers.len(),
                aboard.len()
            );
            // One carrier task a force, whoever made it, wherever it stands.
            let standing = self
                .state
                .intents_in_force()
                .find(|other| {
                    other.kind.force() == Some(id)
                        && matches!(other.kind, IntentKind::OperateCarriers { .. })
                })
                .map(|other| (other.id, other.parent, other.maker));
            if let Some((task, parent, maker)) = standing {
                let by = match maker {
                    Maker::Commander => "the commander's",
                    Maker::Staff => "the staff's own",
                };
                let where_ = match parent {
                    Some(p) if p == end.id => format!("under {p}"),
                    Some(p) => format!("under {p}"),
                    None => "as a root".to_string(),
                };
                let reason = serving(end, plan)
                    .evidence(why.clone(), today)
                    .evidence(
                        format!("{task}, {by} carrier task of {id}, is in force {where_}"),
                        today,
                    )
                    .binding(format!("{task} stands"))
                    .worked(format!(
                        "{task}, {by} task to operate the carrier force {id}, stands {where_}: a force has one carrier task, so the staff forms no other for {id}. {why}."
                    ))
                    .would_change(format!("{task} withdrawn"))
                    .done();
                self.target(in_scope).no_orders.push(reason);
                continue;
            }
            // Item 2: the reach, the least normal strike radius among the
            // bomber groups aboard, and the covering hex.
            let (hex, hex_words) = match self.covering_hex(id, here, &aboard, place) {
                Ok(found) => found,
                Err(NoCoveringHex { reach, sets }) => {
                    let reason = serving(end, plan)
                        .evidence(why.clone(), today)
                        .evidence(
                            format!("no sea route from {here} reaches a hex within {reach} nm of {place}"),
                            today,
                        )
                        .binding("the covering hex is on the shortest sea route to the place")
                        .worked(format!(
                            "{id} is named in the forces limit of {}, but no sea route from {here} reaches a hex within the {reach} nm normal strike radius of {place}, set by {sets}, so the staff forms no carrier task for {id}. {why}.",
                            end.id
                        ))
                        .would_change("a sea route to the place")
                        .done();
                    self.target(in_scope).no_orders.push(reason);
                    continue;
                }
            };
            let kind = IntentKind::OperateCarriers { force: id, hex };
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
            let reason = serving(end, plan)
                .evidence(why.clone(), today)
                .evidence(hex_words.clone(), today)
                .binding(format!("{id} is named in the forces limit of {}", end.id))
                .worked(format!(
                    "{id} is named in the forces limit of {}, its plan is released and no carrier task of {id} stands: {why}. For the place at {place}: {hex_words}. The staff forms {id_new}, to operate the carrier force {id} from {hex}, under it, for {id} to carry out.",
                    end.id
                ))
                .would_change(format!("{id} not named in the forces limit"))
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
                    "{} would form {id_new}, to operate the carrier force {id} from {hex}, under it ({hex_words}).",
                    end.id
                ));
            }
        }
    }

    /// Item 2: the covering hex of force `id`, lying at `here` with the air
    /// groups `aboard`, for the place at `place`, with its words; or, where
    /// no sea route reaches a hex within the reach, the reach and the groups
    /// that set it. Decision 0151, 6, reckons it to an island's first place.
    pub(crate) fn covering_hex(
        &self,
        id: TaskForceId,
        here: Hex,
        aboard: &[&sim_core::state::AirGroup],
        place: Hex,
    ) -> Result<(Hex, String), NoCoveringHex> {
        let bombers: Vec<(AirGroupId, Fixed)> = aboard
            .iter()
            .filter_map(|g| bomber_reach(self.view, g).map(|r| (g.id, r)))
            .collect();
        let reach = bombers.iter().map(|(_, r)| *r).min();
        let place_nm = distance_nm(here, place);
        let Some(reach) = reach else {
            return Ok((
                here,
                format!(
                    "no bomber group is aboard {id}, so the covering hex is the hex it lies in, {here}"
                ),
            ));
        };
        let sets = bombers
            .iter()
            .filter(|(_, r)| *r == reach)
            .map(|(g, _)| g.to_string())
            .collect::<Vec<_>>()
            .join(" and ");
        if place_nm <= reach {
            let within = format!(
                "the least normal strike radius of the bomber groups aboard is {reach} nm, set by {sets}, and {place} is {place_nm} nm from {here}, within it: the covering hex is the hex {id} lies in, {here}"
            );
            // Decision 0218, 55: the hex it lies in is the only hex.
            let regard = match crate::waters::in_submarine_waters(self.area, here) {
                None => String::new(),
                Some(_) => format!("; no place outside submarine waters serves, so it is {here}"),
            };
            return Ok((here, format!("{within}{regard}")));
        }
        match self
            .view
            .sea_path(here, place)
            .and_then(|path| crate::waters::close_to_regarding(self.area, &path, place, reach))
        {
            Some(regard) => {
                let hex = regard.hex;
                let from_force = here.air_distance_nm(hex);
                let from_place = hex.air_distance_nm(place);
                Ok((
                    hex,
                    format!(
                        "the least normal strike radius of the bomber groups aboard is {reach} nm, set by {sets}, and {place} is {place_nm} nm from {here}, beyond it: the first hex along the shortest sea route to {place} within {reach} nm of it is {hex}, {from_force} nm from {here} and {from_place} nm from {place}{}",
                        regard.words()
                    ),
                ))
            }
            None => Err(NoCoveringHex { reach, sets }),
        }
    }

    /// Items 4 to 6: on a morning with no fresh contact of ships within a
    /// group's reach of the force's hex `here`, while the enemy holds a base
    /// in the place's hex within that reach, the bomber groups that the raid's
    /// rules 1 to 4 (decision 0120) send are set to strike that base.
    #[allow(clippy::too_many_arguments, clippy::too_many_lines)]
    pub(crate) fn cover_strike(
        &mut self,
        task: &Intent,
        plan: &Plan,
        end: &Intent,
        here: Hex,
        risk: Risk,
        groups: &[CarrierGroup],
        reach_of: &BTreeMap<AirGroupId, Fixed>,
        air: &CarrierAir,
        targets: &crate::targets::StrikeTargets,
    ) -> CoverStrike {
        let IntentKind::TakePlace { hex: place, .. } = end.kind else {
            return CoverStrike::default();
        };
        let side = self.view.side();
        let Some(base) = self
            .view
            .enemy_bases()
            .filter(|b| b.hex == place && b.side != side)
            .map(|b| (b.id, b.hex))
            .min_by_key(|(id, _)| *id)
        else {
            return CoverStrike::default();
        };
        let aim = StrikeAim {
            base: base.0,
            hex: base.1,
            of: format!("the enemy's at the place {place}"),
            binding: "the enemy's base at the place".to_string(),
        };
        self.strike_base(task, plan, &aim, here, risk, groups, reach_of, air, targets)
    }

    /// The strike on a base by the bombers of a carrier task: under a take,
    /// the enemy's base at the place (decision 0148, 4 to 6); under a
    /// neutralise, its target (decision 0202, 21). Ships come first.
    #[allow(clippy::too_many_arguments, clippy::too_many_lines)]
    pub(crate) fn strike_base(
        &mut self,
        task: &Intent,
        plan: &Plan,
        aim: &StrikeAim,
        here: Hex,
        risk: Risk,
        groups: &[CarrierGroup],
        reach_of: &BTreeMap<AirGroupId, Fixed>,
        air: &CarrierAir,
        targets: &crate::targets::StrikeTargets,
    ) -> CoverStrike {
        let mut out = CoverStrike::default();
        let view = self.view;
        let today = self.today;
        let (base, base_hex) = (aim.base, aim.hex);
        let base_text = base_words(view, base);
        let distance = distance_nm(here, base_hex);
        let IntentKind::OperateCarriers { force: id, .. } = task.kind else {
            return out;
        };
        let picture = crate::picture::picture_of(view, task.kind.addressee());
        // Only contacts the bombers would launch at keep them from the
        // place: with a targets limit, a kind left off its list does not
        // (decision 0190, 17). With none, every kind, as before.
        let fresh: Vec<(u32, Fixed)> = picture
            .contacts()
            .filter(|c| c.age_pulses <= CLOSE_MAX_AGE)
            .filter(|c| {
                targets
                    .kinds
                    .as_ref()
                    .is_none_or(|(kinds, _)| kinds.contains(&c.estimated_kind))
            })
            .map(|c| {
                (
                    picture.contact_number(c.id),
                    distance_nm(here, c.estimated_hex),
                )
            })
            .collect();
        let role_of = |g: AirGroupId| {
            air.roles
                .iter()
                .find(|(x, _)| *x == g)
                .map_or(CarrierRole::Other, |(_, r)| *r)
        };
        // The bomber groups the strike on the place is for: those that the
        // table sets to strike ships, with no fresh contact within their
        // reach and the base within it.
        let mut raid_groups: Vec<RaidGroup> = Vec::new();
        let mut ship_words: Vec<String> = Vec::new();
        for g in groups
            .iter()
            .filter(|g| role_of(g.id) == CarrierRole::Striker)
        {
            let Some(reach) = reach_of.get(&g.id).copied() else {
                continue;
            };
            if let Some((number, nm)) = fresh.iter().find(|(_, nm)| *nm <= reach) {
                ship_words.push(format!(
                    "{}: {} is {nm} nm from {here}, within its {reach} nm: it is left to the table for ships",
                    g.id,
                    crate::picture::contact_words(&picture, *number)
                ));
                continue;
            }
            if distance > reach {
                continue;
            }
            let last_raid = self.last_raid_of(g.id);
            raid_groups.push(RaidGroup {
                id: g.id,
                role: RaidRole::Bomber,
                fatigue: g.fatigue,
                able: g.able,
                mission: g.mission,
                patrol_share: g.patrol_share,
                held_share: g.held_share,
                last_raid,
            });
        }
        if raid_groups.is_empty() {
            return out;
        }
        // A fighter group aboard is the force's cover: no order from this,
        // and the escorts it gives are what its patrol share leaves.
        for g in groups
            .iter()
            .filter(|g| role_of(g.id) == CarrierRole::Fighter)
        {
            let share = (!air.rested.contains(&g.id)).then_some(air.share);
            raid_groups.push(RaidGroup {
                id: g.id,
                role: RaidRole::CoverFighter(share),
                fatigue: g.fatigue,
                able: g.able,
                mission: g.mission,
                patrol_share: g.patrol_share,
                held_share: g.held_share,
                last_raid: None,
            });
        }
        let morning = raid_decide(today, risk, base, &raid_groups);
        let risk_text = format!("{} risk", risk.name());
        let level = rest_level(risk);
        let intent_words = format!("{}, operating the carrier force {id}", task.id);
        let ships_text = if fresh.is_empty() {
            format!("the side holds no contact of ships {CLOSE_MAX_AGE} pulses old or less")
        } else {
            let others = if ship_words.is_empty() {
                "none within the reach of a bomber group that can strike the base".to_string()
            } else {
                ship_words.join("; ")
            };
            format!("no fresh contact of ships is within the reach of the groups that can strike the base ({others})")
        };
        let weight_text = format!(
            "the bombers that go can send {} between them, against the {} the {risk_text} asks",
            morning.weight, morning.least
        );
        if !morning.goes {
            let reason = serving(task, plan)
                .evidence(ships_text.clone(), today)
                .evidence(
                    format!("{base_text} is {distance} nm from {here}, within the reach of its bombers"),
                    today,
                )
                .evidence(weight_text.clone(), today)
                .binding(format!("the least a raid sends, {}, at the {risk_text}", least_weight(risk)))
                .worked(format!(
                    "{intent_words}: {ships_text}; {base_text}, {}, is {distance} nm from {here}, within the reach of its bombers: the raid on it does not go: {weight_text}. No group is set to strike the base, and the table's orders for ships stand.",
                    aim.of
                ))
                .would_change("more bombers able to fly, a bolder risk level")
                .done();
            out.note = Some(reason);
            return out;
        }
        let mut lines: BTreeMap<AirGroupId, String> = BTreeMap::new();
        for (group, verdict) in &morning.bombers {
            let Some(g) = groups.iter().find(|g| g.id == *group) else {
                continue;
            };
            let reach = reach_of.get(group).copied().unwrap_or(Fixed::ZERO);
            let figures = match verdict {
                BomberVerdict::Goes { sends } => format!(
                    "fatigue {}, not above the {level} of the {risk_text}: it can send {sends}",
                    g.fatigue
                ),
                BomberVerdict::Tired => {
                    format!("fatigue {}, above the {level} of the {risk_text}: it rests", g.fatigue)
                }
                BomberVerdict::StandsDown { lost, sent } => format!(
                    "on its last raid, yesterday, it lost {lost} of {sent}: it stands down this morning"
                ),
            };
            let line = format!(
                "{group}, a bomber group within {reach} nm of {base_text} at {distance} nm, has {figures}"
            );
            if matches!(verdict, BomberVerdict::Goes { .. }) {
                out.taken.push(*group);
                lines.insert(
                    *group,
                    format!("{line}: it strikes {base_text} (the raid goes: {weight_text})"),
                );
            }
        }
        for (group, order) in &morning.orders {
            if !out.taken.contains(group) {
                continue;
            }
            let line = lines.get(group).cloned().unwrap_or_default();
            let before = groups.iter().find(|g| g.id == *group);
            let (worked, binding) = match order {
                AirGroupOrder::SetAirMission(mission) => (
                    format!(
                        "{intent_words}: {ships_text}; {line}: its mission is set from {} to {}.",
                        before.map_or_else(String::new, |b| mission_words(b.mission)),
                        mission_words(*mission)
                    ),
                    format!(
                        "no fresh contact of ships in reach, {} in reach, and the least a raid sends, {}, at the {risk_text}",
                        aim.binding,
                        morning.least
                    ),
                ),
                AirGroupOrder::SetStrikeHeldShare { share } => (
                    format!(
                        "{intent_words}: {line}: its held share is set from {} to {share}, the share of its bombers the {risk_text} holds back.",
                        before.map_or_else(String::new, |b| b.held_share.to_string())
                    ),
                    format!("the held share of the {risk_text}: {}", held_share(risk)),
                ),
                _ => continue,
            };
            let reason = serving(task, plan)
                .evidence(ships_text.clone(), today)
                .evidence(
                    format!("{base_text} is {distance} nm from {here}, within the reach of its bombers"),
                    today,
                )
                .evidence(line.clone(), today)
                .evidence(weight_text.clone(), today)
                .binding(binding)
                .worked(worked)
                .would_change("a fresh contact of ships within reach, a different fatigue or risk level, or the cost of the last raid")
                .done();
            out.orders.push(StaffOrder {
                kind: OrderKind::AirGroup {
                    target: *group,
                    order: order.clone(),
                },
                intent: task.id,
                reason,
            });
        }
        out.lines = lines;
        out
    }

    /// Item 8: on the first morning the end is met the force is routed to its
    /// home port at cruising speed, and it is given no order after; with no
    /// home port it is given none, and the reason says the commander must
    /// move it.
    fn send_home(&mut self, task: &Intent, end: &Intent) {
        let Some(plan) = self.plan_of(task) else {
            return;
        };
        let IntentKind::OperateCarriers { force: id, .. } = task.kind else {
            return;
        };
        let view = self.view;
        let today = self.today;
        let in_scope = self.in_scope(task.id);
        let intent_words = format!("{}, operating the carrier force {id}", task.id);
        let Some(force) = view.task_forces().find(|f| f.id == id).cloned() else {
            return;
        };
        if let Some(hold) = self.state.hold_on_force(id, &force.ships) {
            let reason = serving(task, &plan)
                .evidence(format!("{hold} is held by the commander"), today)
                .binding(format!("the hold on {hold}"))
                .worked(format!(
                    "{intent_words}: {} is met, but {hold} is held by the commander, so the staff gives {id} no order.",
                    end.id
                ))
                .would_change("the hold released")
                .done();
            self.target(in_scope).no_orders.push(reason);
            return;
        }
        let here = force.position();
        let home = force.home_port;
        let home_hex = home.and_then(|b| view.bases().find(|x| x.id == b).map(|x| x.hex));
        let Some((home, home_hex)) = home.zip(home_hex) else {
            let reason = serving(task, &plan)
                .evidence(format!("{} is met; {id} lies at {here} and has no home port", end.id), today)
                .binding("a force is sent home only where it has a home port")
                .worked(format!(
                    "{intent_words}: {} is met, and {id} lies at {here} with no home port, so the staff gives it no order: the commander must move it.",
                    end.id
                ))
                .would_change("a home port set")
                .done();
            self.target(in_scope).no_orders.push(reason);
            return;
        };
        let home_text = base_words(view, home);
        let at_home = stopped(&force) && here == home_hex;
        let on_way = force.route.waypoints.last().copied() == Some(home_hex)
            && force.route.next_index < force.route.waypoints.len();
        if at_home || on_way {
            let why = if at_home {
                format!("{id} is stopped at its home port {home_text}")
            } else {
                format!(
                    "{id} is on its way to its home port {home_text}; {}",
                    expected_words(view, id, home_hex)
                )
            };
            let reason = serving(task, &plan)
                .evidence(format!("{} is met; {why}", end.id), today)
                .worked(format!(
                    "{intent_words}: {} is met; {why}, so the staff gives it no order.",
                    end.id
                ))
                .would_change("the force away from its home port")
                .done();
            self.target(in_scope).no_orders.push(reason);
            return;
        }
        let reason = serving(task, &plan)
            .evidence(format!("{} is met; {id} lies at {here}", end.id), today)
            .evidence(format!("its home port is {home_text}"), today)
            .binding("on the morning the end is met the force goes home at cruise")
            .worked(format!(
                "{intent_words}: {} is met: {id}, at {here}, is routed to its home port {home_text} at cruising speed, and the staff gives its task and its groups no order after.",
                end.id
            ))
            .would_change("the end not met")
            .done();
        let mut orders: Vec<TaskForceOrder> = Vec::new();
        if force.speed != SpeedSetting::Cruise {
            orders.push(TaskForceOrder::SetSpeed(SpeedSetting::Cruise));
        }
        orders.push(TaskForceOrder::route(vec![home_hex]));
        let words = reason.worked.clone();
        let out = self.target(in_scope);
        for order in orders {
            out.orders.push(StaffOrder {
                kind: OrderKind::TaskForce { target: id, order },
                intent: task.id,
                reason: reason.clone(),
            });
        }
        if in_scope {
            self.lines.push(words);
        }
    }
}
